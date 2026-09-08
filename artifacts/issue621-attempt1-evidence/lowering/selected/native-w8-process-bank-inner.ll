define internal fastcc void @_RINvMsf_CsdvPQf9CMsz3_17true_peak_limiterINtB6_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb1_EB6_(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(328) %_0, ptr noalias noundef nonnull align 32 dereferenceable(2304) %self, ptr dead_on_return noalias noundef nonnull readonly align 8 captures(none) dereferenceable(112) %block) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !5905 {
start:
  %_75.i963.i = alloca [736 x i8], align 32
  %peaks_left.i974.i = alloca [1024 x i8], align 4
  %scratch.i975.i = alloca [32 x i8], align 4
  %hot_left.i981.i = alloca [736 x i8], align 32
  %peaks_left.i664.i = alloca [1024 x i8], align 4
  %scratch.i.i = alloca [32 x i8], align 4
  %hot_left.i670.i = alloca [736 x i8], align 32
  %left_prefix.i242.i = alloca [32 x i8], align 32
  %uniform_left.i263.i = alloca [128 x i8], align 32
  %peaks_left.i264.i = alloca [1024 x i8], align 4
  %hot_left.i270.i = alloca [736 x i8], align 32
  %left_prefix.i.i = alloca [32 x i8], align 32
  %uniform_left.i.i = alloca [128 x i8], align 32
  %peaks_left.i.i = alloca [1024 x i8], align 4
  %hot_left.i.i = alloca [736 x i8], align 32
  %shape.i = alloca [24 x i8], align 8
  %report = alloca [328 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %block, i64 32, !dbg !5906
  %_37.0 = load ptr, ptr %0, align 8, !dbg !5906, !nonnull !12, !align !14, !noundef !12
  %1 = getelementptr inbounds nuw i8, ptr %block, i64 40, !dbg !5906
  %_37.1 = load i64, ptr %1, align 8, !dbg !5906, !noundef !12
  %2 = icmp eq i64 %_37.1, 0, !dbg !5906
  br i1 %2, label %bb2, label %bb1, !dbg !5906

bb2:                                              ; preds = %bb1, %start
  call void @llvm.lifetime.start.p0(ptr nonnull %report), !dbg !5907
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 2288, !dbg !5908
  %4 = load i8, ptr %3, align 16, !dbg !5908, !range !17, !noundef !12
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(320) %report, i8 0, i64 320, i1 false)
  %5 = getelementptr inbounds nuw i8, ptr %report, i64 320, !dbg !5909
  store i8 %4, ptr %5, align 8, !dbg !5909
  %6 = getelementptr inbounds nuw i8, ptr %block, i64 48
  %_38.0 = load ptr, ptr %6, align 8, !nonnull !12, !align !24, !noundef !12
  %7 = getelementptr inbounds nuw i8, ptr %block, i64 56
  %_38.1 = load i64, ptr %7, align 8, !noundef !12
  %_26 = getelementptr inbounds nuw i8, ptr %self, i64 1848
  %_25 = getelementptr inbounds nuw i8, ptr %self, i64 1648
  %8 = getelementptr inbounds nuw i8, ptr %block, i64 96
  %_24 = load i64, ptr %8, align 8
  %_23 = getelementptr inbounds nuw i8, ptr %self, i64 2048
  %9 = call i64 @llvm.usub.sat.i64(i64 %_38.1, i64 1), !dbg !5912
  %exitcond.not = icmp eq i64 %_38.1, 0, !dbg !5920
  br i1 %exitcond.not, label %panic, label %bb4, !dbg !5920

bb1:                                              ; preds = %start
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 2152, !dbg !5922
  store i8 0, ptr %10, align 8, !dbg !5922
  br label %bb2, !dbg !5923

start.bb11.thread_crit_edge.i:                    ; preds = %bb6.7
  %_19.0.i.pre.pre.i = load ptr, ptr %1182, align 8, !dbg !5924, !alias.scope !5933, !noalias !5938
  br label %bb11.thread.i, !dbg !5944

bb1.i:                                            ; preds = %bb6.7
  %_68.0.i = load ptr, ptr %1182, align 16, !dbg !5945, !alias.scope !5946, !noalias !5947, !nonnull !12, !noundef !12
  %_8.i3771.i = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_68.0.i, i64 %_68.1.i, !dbg !5948
  br label %bb1.i.i.i, !dbg !5953

bb1.i.i.i:                                        ; preds = %bb13.i.i3773.i, %bb1.i
  %_221.i.i.i = phi ptr [ %_22.i.i3774.i, %bb13.i.i3773.i ], [ %_68.0.i, %bb1.i ]
  %_12.i.i3772.i = icmp eq ptr %_221.i.i.i, %_8.i3771.i, !dbg !5955
  br i1 %_12.i.i3772.i, label %bb3.i, label %bb13.i.i3773.i, !dbg !5958

bb13.i.i3773.i:                                   ; preds = %bb1.i.i.i
  %_22.i.i3774.i = getelementptr inbounds nuw i8, ptr %_221.i.i.i, i64 16, !dbg !5959
  %11 = getelementptr inbounds nuw i8, ptr %_221.i.i.i, i64 12, !dbg !5961
  %_3.i.i.i.i = load i32, ptr %11, align 4, !dbg !5961, !alias.scope !5963, !noalias !5968, !noundef !12
  %12 = icmp eq i32 %_3.i.i.i.i, 0, !dbg !5961
  %_51.i.i.i.i = load i32, ptr %_221.i.i.i, align 4, !dbg !5961, !alias.scope !5963, !noalias !5968
  %13 = getelementptr inbounds nuw i8, ptr %_221.i.i.i, i64 4, !dbg !5961
  %_72.i.i.i.i = load i32, ptr %13, align 4, !dbg !5961, !alias.scope !5963, !noalias !5968
  %14 = icmp eq i32 %_51.i.i.i.i, %_72.i.i.i.i, !dbg !5961
  %_0.sroa.0.0.i.i.i.i = select i1 %12, i1 %14, i1 false, !dbg !5961
  br i1 %_0.sroa.0.0.i.i.i.i, label %bb1.i.i.i, label %bb11.thread.i, !dbg !5971

bb3.i:                                            ; preds = %bb1.i.i.i
  %15 = getelementptr inbounds nuw i8, ptr %self, i64 1792, !dbg !5972
  %_69.0.i = load ptr, ptr %15, align 16, !dbg !5972, !alias.scope !5946, !noalias !5947, !nonnull !12, !noundef !12
  %16 = getelementptr inbounds nuw i8, ptr %self, i64 1800, !dbg !5972
  %_69.1.i = load i64, ptr %16, align 8, !dbg !5972, !alias.scope !5946, !noalias !5947, !noundef !12
  %_8.i3775.i = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_69.0.i, i64 %_69.1.i, !dbg !5973
  br label %bb1.i.i3776.i, !dbg !5978

bb1.i.i3776.i:                                    ; preds = %bb13.i.i3779.i, %bb3.i
  %_221.i.i3777.i = phi ptr [ %_22.i.i3780.i, %bb13.i.i3779.i ], [ %_69.0.i, %bb3.i ]
  %_12.i.i3778.i = icmp eq ptr %_221.i.i3777.i, %_8.i3775.i, !dbg !5980
  br i1 %_12.i.i3778.i, label %bb5.i, label %bb13.i.i3779.i, !dbg !5983

bb13.i.i3779.i:                                   ; preds = %bb1.i.i3776.i
  %_22.i.i3780.i = getelementptr inbounds nuw i8, ptr %_221.i.i3777.i, i64 16, !dbg !5984
  %17 = getelementptr inbounds nuw i8, ptr %_221.i.i3777.i, i64 12, !dbg !5986
  %_3.i.i.i3781.i = load i32, ptr %17, align 4, !dbg !5986, !alias.scope !5988, !noalias !5993, !noundef !12
  %18 = icmp eq i32 %_3.i.i.i3781.i, 0, !dbg !5986
  %_51.i.i.i3782.i = load i32, ptr %_221.i.i3777.i, align 4, !dbg !5986, !alias.scope !5988, !noalias !5993
  %19 = getelementptr inbounds nuw i8, ptr %_221.i.i3777.i, i64 4, !dbg !5986
  %_72.i.i.i3783.i = load i32, ptr %19, align 4, !dbg !5986, !alias.scope !5988, !noalias !5993
  %20 = icmp eq i32 %_51.i.i.i3782.i, %_72.i.i.i3783.i, !dbg !5986
  %_0.sroa.0.0.i.i.i3784.i = select i1 %18, i1 %20, i1 false, !dbg !5986
  br i1 %_0.sroa.0.0.i.i.i3784.i, label %bb1.i.i3776.i, label %bb11.thread.i, !dbg !5996

bb5.i:                                            ; preds = %bb1.i.i3776.i
  %_51.not.i = icmp samesign ugt i64 %words.i, %_39.1
  br i1 %_51.not.i, label %bb35.i, label %bb1.i3786.i, !dbg !5997, !prof !165

bb11.thread.i:                                    ; preds = %bb13.i.i3773.i, %bb13.i.i3779.i, %start.bb11.thread_crit_edge.i
  %_19.0.i.pre.i = phi ptr [ %_19.0.i.pre.pre.i, %start.bb11.thread_crit_edge.i ], [ %_68.0.i, %bb13.i.i3779.i ], [ %_68.0.i, %bb13.i.i3773.i ], !dbg !5924
  %21 = getelementptr inbounds nuw i8, ptr %self, i64 2152
  br label %bb16.i, !dbg !6006

bb11.i:                                           ; preds = %bb1.i3786.i
  %22 = getelementptr inbounds nuw i8, ptr %self, i64 2152
  %23 = load i8, ptr %22, align 8, !range !17, !alias.scope !5946, !noalias !5947
  %_15.i = trunc nuw i8 %23 to i1
  br i1 %_15.i, label %bb13.i, label %bb16.i, !dbg !6006

bb1.i3786.i:                                      ; preds = %bb5.i, %bb10.i3792.i
  %iter.sroa.6.0.i.i = phi i64 [ %len.i.i.i.i.i, %bb10.i3792.i ], [ %words.i, %bb5.i ], !dbg !6007
  %iter.sroa.0.0.i3787.i = phi ptr [ %data.i.i.i.i.i, %bb10.i3792.i ], [ %_39.0, %bb5.i ], !dbg !6007
  %24 = icmp eq i64 %iter.sroa.6.0.i.i, 0, !dbg !6009
  br i1 %24, label %bb11.i, label %bb11.preheader.i.i, !dbg !6009

bb11.preheader.i.i:                               ; preds = %bb1.i3786.i
  %..i.i.i.i = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i.i, i64 32), !dbg !6011
  %_18.idx.i.i = shl nuw nsw i64 %..i.i.i.i, 2, !dbg !6014
  %_18.i3788.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i3787.i, i64 %_18.idx.i.i, !dbg !6014
  br label %bb11.i3789.i, !dbg !6019

bb11.i3789.i:                                     ; preds = %bb11.i3789.i, %bb11.preheader.i.i
  %iter1.sroa.0.014.i.i = phi ptr [ %_31.i3790.i, %bb11.i3789.i ], [ %iter.sroa.0.0.i3787.i, %bb11.preheader.i.i ]
  %bits.sroa.0.013.i.i = phi i32 [ %25, %bb11.i3789.i ], [ 0, %bb11.preheader.i.i ]
  %_31.i3790.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i.i, i64 4, !dbg !6021
  %_134.i.i = load i32, ptr %iter1.sroa.0.014.i.i, align 4, !dbg !6023, !alias.scope !6024, !noalias !5946, !noundef !12
  %25 = or i32 %_134.i.i, %bits.sroa.0.013.i.i, !dbg !6027
  %_25.i3791.i = icmp eq ptr %_31.i3790.i, %_18.i3788.i, !dbg !6028
  br i1 %_25.i3791.i, label %bb10.i3792.i, label %bb11.i3789.i, !dbg !6019

bb10.i3792.i:                                     ; preds = %bb11.i3789.i
  %data.i.i.i.i.i = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i3787.i, i64 %..i.i.i.i, !dbg !6030
  %len.i.i.i.i.i = sub nuw nsw i64 %iter.sroa.6.0.i.i, %..i.i.i.i, !dbg !6035
  %26 = icmp eq i32 %25, 0, !dbg !6036
  br i1 %26, label %bb1.i3786.i, label %bb11.thread6946.i, !dbg !6036

bb11.thread6946.i:                                ; preds = %bb10.i3792.i
  %27 = getelementptr inbounds nuw i8, ptr %self, i64 2152
  br label %bb16.i, !dbg !6006

bb35.i:                                           ; preds = %bb5.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %words.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_618bfb3c8de0a6cf4e1facf86d98b625) #30, !dbg !6037, !noalias !5947
  unreachable, !dbg !6037

bb16.i:                                           ; preds = %bb11.thread6946.i, %bb11.i, %bb11.thread.i
  %_19.0.i.i = phi ptr [ %_19.0.i.pre.i, %bb11.thread.i ], [ %_68.0.i, %bb11.i ], [ %_68.0.i, %bb11.thread6946.i ], !dbg !5924
  %28 = phi ptr [ %21, %bb11.thread.i ], [ %22, %bb11.i ], [ %27, %bb11.thread6946.i ]
  %quiet.sroa.0.06945.i = phi i1 [ false, %bb11.thread.i ], [ true, %bb11.i ], [ false, %bb11.thread6946.i ]
  %_23.i = getelementptr inbounds nuw i8, ptr %self, i64 1616, !dbg !6038
  %_25.i = getelementptr inbounds nuw i8, ptr %self, i64 1640, !dbg !6039
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6040), !dbg !6041
  %_8.i3793.i = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_19.0.i.i, i64 %_68.1.i, !dbg !6042
  br label %bb1.i.i3794.i, !dbg !6047

bb1.i.i3794.i:                                    ; preds = %bb13.i.i3797.i, %bb16.i
  %_221.i.i3795.i = phi ptr [ %_22.i.i3798.i, %bb13.i.i3797.i ], [ %_19.0.i.i, %bb16.i ]
  %_12.i.i3796.i = icmp eq ptr %_221.i.i3795.i, %_8.i3793.i, !dbg !6049
  br i1 %_12.i.i3796.i, label %bb12.i.i, label %bb13.i.i3797.i, !dbg !6052

bb13.i.i3797.i:                                   ; preds = %bb1.i.i3794.i
  %_22.i.i3798.i = getelementptr inbounds nuw i8, ptr %_221.i.i3795.i, i64 16, !dbg !6053
  %29 = getelementptr inbounds nuw i8, ptr %_221.i.i3795.i, i64 12, !dbg !6055
  %_3.i.i.i3799.i = load i32, ptr %29, align 4, !dbg !6055, !alias.scope !6057, !noalias !6062, !noundef !12
  %30 = icmp eq i32 %_3.i.i.i3799.i, 0, !dbg !6055
  %_51.i.i.i3800.i = load i32, ptr %_221.i.i3795.i, align 4, !dbg !6055, !alias.scope !6057, !noalias !6062
  %31 = getelementptr inbounds nuw i8, ptr %_221.i.i3795.i, i64 4, !dbg !6055
  %_72.i.i.i3801.i = load i32, ptr %31, align 4, !dbg !6055, !alias.scope !6057, !noalias !6062
  %32 = icmp eq i32 %_51.i.i.i3800.i, %_72.i.i.i3801.i, !dbg !6055
  %_0.sroa.0.0.i.i.i3802.i = select i1 %30, i1 %32, i1 false, !dbg !6055
  br i1 %_0.sroa.0.0.i.i.i3802.i, label %bb1.i.i3794.i, label %bb13.i.i, !dbg !6065

bb13.i.i:                                         ; preds = %bb13.i.i3797.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6066), !dbg !6069
  %33 = getelementptr inbounds nuw i8, ptr %self, i64 1824, !dbg !6071
  %_31.0.i.i = load ptr, ptr %33, align 8, !dbg !6071, !alias.scope !6073, !noalias !5938, !nonnull !12, !noundef !12
  %34 = getelementptr inbounds nuw i8, ptr %self, i64 1832, !dbg !6071
  %_31.1.i.i = load i64, ptr %34, align 8, !dbg !6071, !alias.scope !6073, !noalias !5938, !noundef !12
  %_17.idx.i.i = mul nuw nsw i64 %_31.1.i.i, 12, !dbg !6074
  %_17.i3804.i = getelementptr inbounds nuw i8, ptr %_31.0.i.i, i64 %_17.idx.i.i, !dbg !6074
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6078), !dbg !6081, !noalias !6082
  %_5.not.i.i.i.i = icmp eq i64 %_31.1.i.i, 0
  %35 = getelementptr inbounds nuw i8, ptr %_31.0.i.i, i64 4
  %36 = getelementptr inbounds nuw i8, ptr %_31.0.i.i, i64 8
  br i1 %_5.not.i.i.i.i, label %bb2.i3815.i, label %bb1.i.i3805.i

bb1.i.i3805.i:                                    ; preds = %bb13.i.i, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i.i
  %_224.i.i.i = phi ptr [ %_22.i.i3808.i, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i.i ], [ %_31.0.i.i, %bb13.i.i ]
  %_12.i.i3806.i = icmp eq ptr %_224.i.i.i, %_17.i3804.i, !dbg !6083
  br i1 %_12.i.i3806.i, label %bb2.i3815.i, label %bb13.i.i3807.i, !dbg !6087

bb13.i.i3807.i:                                   ; preds = %bb1.i.i3805.i
  %_22.i.i3808.i = getelementptr inbounds nuw i8, ptr %_224.i.i.i, i64 12, !dbg !6088
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6090), !dbg !6093, !noalias !6082
  %_9.i.i.i3809.i = load i32, ptr %_224.i.i.i, align 4, !dbg !6094, !alias.scope !6090, !noalias !6097, !noundef !12
  %_10.i.i.i.i = load i32, ptr %_31.0.i.i, align 4, !dbg !6094, !alias.scope !6078, !noalias !6099, !noundef !12
  %_8.i.i.i.i = icmp eq i32 %_9.i.i.i3809.i, %_10.i.i.i.i, !dbg !6094
  br i1 %_8.i.i.i.i, label %bb2.i.i.i.i, label %bb8.i.i, !dbg !6094

bb2.i.i.i.i:                                      ; preds = %bb13.i.i3807.i
  %37 = getelementptr inbounds nuw i8, ptr %_224.i.i.i, i64 4, !dbg !6094
  %_12.i.i.i3811.i = load i32, ptr %37, align 4, !dbg !6094, !alias.scope !6090, !noalias !6097, !noundef !12
  %_13.i.i.i3812.i = load i32, ptr %35, align 4, !dbg !6094, !alias.scope !6078, !noalias !6099, !noundef !12
  %_11.i.i.i.i = icmp eq i32 %_12.i.i.i3811.i, %_13.i.i.i3812.i, !dbg !6094
  br i1 %_11.i.i.i.i, label %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i.i, label %bb8.i.i, !dbg !6094

_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i.i: ; preds = %bb2.i.i.i.i
  %38 = getelementptr inbounds nuw i8, ptr %_224.i.i.i, i64 8, !dbg !6094
  %_14.i.i.i3813.i = load i32, ptr %38, align 4, !dbg !6094, !alias.scope !6090, !noalias !6097, !noundef !12
  %_15.i.i.i3814.i = load i32, ptr %36, align 4, !dbg !6094, !alias.scope !6078, !noalias !6099, !noundef !12
  %39 = icmp eq i32 %_14.i.i.i3813.i, %_15.i.i.i3814.i, !dbg !6094
  br i1 %39, label %bb1.i.i3805.i, label %bb8.i.i, !dbg !6093

bb2.i3815.i:                                      ; preds = %bb1.i.i3805.i, %bb13.i.i
  %40 = getelementptr inbounds nuw i8, ptr %self, i64 1760, !dbg !6100
  %_32.0.i.i = load ptr, ptr %40, align 8, !dbg !6100, !alias.scope !6073, !noalias !5938, !nonnull !12, !noundef !12
  %41 = getelementptr inbounds nuw i8, ptr %self, i64 1768, !dbg !6100
  %_32.1.i.i = load i64, ptr %41, align 8, !dbg !6100, !alias.scope !6073, !noalias !5938, !noundef !12
  %_26.idx.i.i = shl nuw nsw i64 %_32.1.i.i, 2, !dbg !6101
  %_26.i3816.i = getelementptr inbounds nuw i8, ptr %_32.0.i.i, i64 %_26.idx.i.i, !dbg !6101
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6105), !dbg !6108, !noalias !6082
  %_6.not.i.i.i.i = icmp eq i64 %_32.1.i.i, 0
  br i1 %_6.not.i.i.i.i, label %bb4.i.i, label %bb1.i3.i.i

bb1.i3.i.i:                                       ; preds = %bb2.i3815.i, %bb13.i5.i.i
  %_223.i.i.i = phi ptr [ %_22.i6.i.i, %bb13.i5.i.i ], [ %_32.0.i.i, %bb2.i3815.i ]
  %_12.i4.i.i = icmp eq ptr %_223.i.i.i, %_26.i3816.i, !dbg !6109
  br i1 %_12.i4.i.i, label %bb4.i.i, label %bb13.i5.i.i, !dbg !6113

bb13.i5.i.i:                                      ; preds = %bb1.i3.i.i
  %_22.i6.i.i = getelementptr inbounds nuw i8, ptr %_223.i.i.i, i64 4, !dbg !6114
  %ptr.val.i.i.i = load i32, ptr %_223.i.i.i, align 4, !dbg !6116, !noalias !6117
  %_4.i.i.i3817.i = load i32, ptr %_32.0.i.i, align 4, !dbg !6119, !alias.scope !6105, !noalias !6121, !noundef !12
  %_0.i.i.i.i = icmp eq i32 %ptr.val.i.i.i, %_4.i.i.i3817.i, !dbg !6122
  br i1 %_0.i.i.i.i, label %bb1.i3.i.i, label %bb8.i.i, !dbg !6116

bb12.i.i:                                         ; preds = %bb1.i.i3794.i
  %42 = getelementptr inbounds nuw i8, ptr %self, i64 1792, !dbg !6123
  %_20.0.i.i = load ptr, ptr %42, align 8, !dbg !6123, !alias.scope !5933, !noalias !5938, !nonnull !12, !noundef !12
  %43 = getelementptr inbounds nuw i8, ptr %self, i64 1800, !dbg !6123
  %_20.1.i.i = load i64, ptr %43, align 8, !dbg !6123, !alias.scope !5933, !noalias !5938, !noundef !12
  %_8.i3818.i = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_20.0.i.i, i64 %_20.1.i.i, !dbg !6124
  br label %bb1.i.i3819.i, !dbg !6129

bb1.i.i3819.i:                                    ; preds = %bb13.i.i3822.i, %bb12.i.i
  %_221.i.i3820.i = phi ptr [ %_22.i.i3823.i, %bb13.i.i3822.i ], [ %_20.0.i.i, %bb12.i.i ]
  %_12.i.i3821.i = icmp eq ptr %_221.i.i3820.i, %_8.i3818.i, !dbg !6131
  br i1 %_12.i.i3821.i, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter20ramps_are_stationary.exit3828.i, label %bb13.i.i3822.i, !dbg !6134

bb13.i.i3822.i:                                   ; preds = %bb1.i.i3819.i
  %_22.i.i3823.i = getelementptr inbounds nuw i8, ptr %_221.i.i3820.i, i64 16, !dbg !6135
  %44 = getelementptr inbounds nuw i8, ptr %_221.i.i3820.i, i64 12, !dbg !6137
  %_3.i.i.i3824.i = load i32, ptr %44, align 4, !dbg !6137, !alias.scope !6139, !noalias !6144, !noundef !12
  %45 = icmp eq i32 %_3.i.i.i3824.i, 0, !dbg !6137
  %_51.i.i.i3825.i = load i32, ptr %_221.i.i3820.i, align 4, !dbg !6137, !alias.scope !6139, !noalias !6144
  %46 = getelementptr inbounds nuw i8, ptr %_221.i.i3820.i, i64 4, !dbg !6137
  %_72.i.i.i3826.i = load i32, ptr %46, align 4, !dbg !6137, !alias.scope !6139, !noalias !6144
  %47 = icmp eq i32 %_51.i.i.i3825.i, %_72.i.i.i3826.i, !dbg !6137
  %_0.sroa.0.0.i.i.i3827.i = select i1 %45, i1 %47, i1 false, !dbg !6137
  br i1 %_0.sroa.0.0.i.i.i3827.i, label %bb1.i.i3819.i, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter20ramps_are_stationary.exit3828.i, !dbg !6147

_RNvCsdvPQf9CMsz3_17true_peak_limiter20ramps_are_stationary.exit3828.i: ; preds = %bb13.i.i3822.i, %bb1.i.i3819.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6148), !dbg !6069
  %48 = getelementptr inbounds nuw i8, ptr %self, i64 1824, !dbg !6151
  %_31.0.i3829.i = load ptr, ptr %48, align 8, !dbg !6151, !alias.scope !6153, !noalias !5938, !nonnull !12, !noundef !12
  %49 = getelementptr inbounds nuw i8, ptr %self, i64 1832, !dbg !6151
  %_31.1.i3830.i = load i64, ptr %49, align 8, !dbg !6151, !alias.scope !6153, !noalias !5938, !noundef !12
  %_17.idx.i3831.i = mul nuw nsw i64 %_31.1.i3830.i, 12, !dbg !6154
  %_17.i3832.i = getelementptr inbounds nuw i8, ptr %_31.0.i3829.i, i64 %_17.idx.i3831.i, !dbg !6154
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6158), !dbg !6161, !noalias !6082
  %_5.not.i.i.i3833.i = icmp eq i64 %_31.1.i3830.i, 0
  %50 = getelementptr inbounds nuw i8, ptr %_31.0.i3829.i, i64 4
  %51 = getelementptr inbounds nuw i8, ptr %_31.0.i3829.i, i64 8
  br i1 %_5.not.i.i.i3833.i, label %bb2.i3851.i, label %bb1.i.i3834.i

bb1.i.i3834.i:                                    ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter20ramps_are_stationary.exit3828.i, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3848.i
  %_224.i.i3835.i = phi ptr [ %_22.i.i3838.i, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3848.i ], [ %_31.0.i3829.i, %_RNvCsdvPQf9CMsz3_17true_peak_limiter20ramps_are_stationary.exit3828.i ]
  %_12.i.i3836.i = icmp eq ptr %_224.i.i3835.i, %_17.i3832.i, !dbg !6162
  br i1 %_12.i.i3836.i, label %bb2.i3851.i, label %bb13.i.i3837.i, !dbg !6166

bb13.i.i3837.i:                                   ; preds = %bb1.i.i3834.i
  %_22.i.i3838.i = getelementptr inbounds nuw i8, ptr %_224.i.i3835.i, i64 12, !dbg !6167
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6169), !dbg !6172, !noalias !6082
  %_9.i.i.i3839.i = load i32, ptr %_224.i.i3835.i, align 4, !dbg !6173, !alias.scope !6169, !noalias !6176, !noundef !12
  %_10.i.i.i3840.i = load i32, ptr %_31.0.i3829.i, align 4, !dbg !6173, !alias.scope !6158, !noalias !6178, !noundef !12
  %_8.i.i.i3841.i = icmp eq i32 %_9.i.i.i3839.i, %_10.i.i.i3840.i, !dbg !6173
  br i1 %_8.i.i.i3841.i, label %bb2.i.i.i3844.i, label %bb6.i.i, !dbg !6173

bb2.i.i.i3844.i:                                  ; preds = %bb13.i.i3837.i
  %52 = getelementptr inbounds nuw i8, ptr %_224.i.i3835.i, i64 4, !dbg !6173
  %_12.i.i.i3845.i = load i32, ptr %52, align 4, !dbg !6173, !alias.scope !6169, !noalias !6176, !noundef !12
  %_13.i.i.i3846.i = load i32, ptr %50, align 4, !dbg !6173, !alias.scope !6158, !noalias !6178, !noundef !12
  %_11.i.i.i3847.i = icmp eq i32 %_12.i.i.i3845.i, %_13.i.i.i3846.i, !dbg !6173
  br i1 %_11.i.i.i3847.i, label %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3848.i, label %bb6.i.i, !dbg !6173

_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3848.i: ; preds = %bb2.i.i.i3844.i
  %53 = getelementptr inbounds nuw i8, ptr %_224.i.i3835.i, i64 8, !dbg !6173
  %_14.i.i.i3849.i = load i32, ptr %53, align 4, !dbg !6173, !alias.scope !6169, !noalias !6176, !noundef !12
  %_15.i.i.i3850.i = load i32, ptr %51, align 4, !dbg !6173, !alias.scope !6158, !noalias !6178, !noundef !12
  %54 = icmp eq i32 %_14.i.i.i3849.i, %_15.i.i.i3850.i, !dbg !6173
  br i1 %54, label %bb1.i.i3834.i, label %bb6.i.i, !dbg !6172

bb2.i3851.i:                                      ; preds = %bb1.i.i3834.i, %_RNvCsdvPQf9CMsz3_17true_peak_limiter20ramps_are_stationary.exit3828.i
  %55 = getelementptr inbounds nuw i8, ptr %self, i64 1760, !dbg !6179
  %_32.0.i3852.i = load ptr, ptr %55, align 8, !dbg !6179, !alias.scope !6153, !noalias !5938, !nonnull !12, !noundef !12
  %56 = getelementptr inbounds nuw i8, ptr %self, i64 1768, !dbg !6179
  %_32.1.i3853.i = load i64, ptr %56, align 8, !dbg !6179, !alias.scope !6153, !noalias !5938, !noundef !12
  %_26.idx.i3854.i = shl nuw nsw i64 %_32.1.i3853.i, 2, !dbg !6180
  %_26.i3855.i = getelementptr inbounds nuw i8, ptr %_32.0.i3852.i, i64 %_26.idx.i3854.i, !dbg !6180
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6184), !dbg !6187, !noalias !6082
  %_6.not.i.i.i3856.i = icmp eq i64 %_32.1.i3853.i, 0
  br i1 %_6.not.i.i.i3856.i, label %bb2.i.i, label %bb1.i3.i3857.i

bb1.i3.i3857.i:                                   ; preds = %bb2.i3851.i, %bb13.i5.i3860.i
  %_223.i.i3858.i = phi ptr [ %_22.i6.i3861.i, %bb13.i5.i3860.i ], [ %_32.0.i3852.i, %bb2.i3851.i ]
  %_12.i4.i3859.i = icmp eq ptr %_223.i.i3858.i, %_26.i3855.i, !dbg !6188
  br i1 %_12.i4.i3859.i, label %bb2.i.i, label %bb13.i5.i3860.i, !dbg !6192

bb13.i5.i3860.i:                                  ; preds = %bb1.i3.i3857.i
  %_22.i6.i3861.i = getelementptr inbounds nuw i8, ptr %_223.i.i3858.i, i64 4, !dbg !6193
  %ptr.val.i.i3862.i = load i32, ptr %_223.i.i3858.i, align 4, !dbg !6195, !noalias !6196
  %_4.i.i.i3863.i = load i32, ptr %_32.0.i3852.i, align 4, !dbg !6198, !alias.scope !6184, !noalias !6200, !noundef !12
  %_0.i.i.i3864.i = icmp eq i32 %ptr.val.i.i3862.i, %_4.i.i.i3863.i, !dbg !6201
  br i1 %_0.i.i.i3864.i, label %bb1.i3.i3857.i, label %bb6.i.i, !dbg !6195

bb8.i.i:                                          ; preds = %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i.i, %bb2.i.i.i.i, %bb13.i.i3807.i, %bb13.i5.i.i, %bb6.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6202), !dbg !6205
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6206), !dbg !6205
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6208), !dbg !6205
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i981.i), !dbg !6210, !noalias !6214
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_left.i981.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_25) #31, !dbg !6217, !noalias !6218
  %57 = getelementptr inbounds nuw i8, ptr %self, i64 1536, !dbg !6219
  %58 = load i8, ptr %57, align 32, !dbg !6219, !range !17, !alias.scope !6223, !noalias !6224, !noundef !12
  %59 = getelementptr inbounds nuw i8, ptr %self, i64 1537, !dbg !6225
  %60 = load i8, ptr %59, align 1, !dbg !6225, !range !17, !alias.scope !6223, !noalias !6224, !noundef !12
  %_24.i988.i = load i32, ptr %_25.i, align 4, !dbg !6227, !alias.scope !6229, !noalias !6230, !noundef !12
  %61 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !6231
  %_26.i989.i = load i32, ptr %61, align 4, !dbg !6231, !alias.scope !6229, !noalias !6230, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i975.i), !dbg !6233, !noalias !6214
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i975.i, i8 0, i64 32, i1 false), !noalias !6214
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i974.i), !dbg !6235, !noalias !6214
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i974.i, i8 0, i64 1024, i1 false), !noalias !6214
  %62 = add nuw nsw i64 %_31, 31, !dbg !6237
  %yield_count.sroa.0.0.i.i.i = lshr i64 %62, 5, !dbg !6237
  %_79.not.i10008087.i = icmp eq i64 %yield_count.sroa.0.0.i.i.i, 0, !dbg !6244
  br i1 %_79.not.i10008087.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, label %bb29.i1001.lr.ph.i, !dbg !6244

bb29.i1001.lr.ph.i:                               ; preds = %bb8.i.i
  %63 = zext i32 %_26.i989.i to i64, !dbg !6231
  %64 = zext i32 %_24.i988.i to i64, !dbg !6227
  %_22.i985.i = trunc nuw i8 %60 to i1, !dbg !6225
  %_21.i982.i = trunc nuw i8 %58 to i1, !dbg !6219
  %65 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !6253
  %66 = bitcast <8 x float> %65 to <8 x i32>, !dbg !6274
  %67 = xor <8 x i32> %66, splat (i32 -1), !dbg !6289
  %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i981.i, i64 32
  %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i981.i, i64 64
  %history.i.i927.sroa.16.0.hot_left.i981.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i981.i, i64 96
  %history.i.i927.sroa.19.0.hot_left.i981.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i981.i, i64 128
  %history.i.i927.sroa.22.0.hot_left.i981.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i981.i, i64 160
  %history.i.i927.sroa.25.0.hot_left.i981.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i981.i, i64 192
  %history.i.i927.sroa.29.0.hot_left.i981.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i981.i, i64 224
  %history.i.i927.sroa.32.0.hot_left.i981.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i981.i, i64 256
  %history.i.i927.sroa.35.0.hot_left.i981.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i981.i, i64 288
  %history.i.i927.sroa.38.0.hot_left.i981.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i981.i, i64 320
  %history.i.i927.sroa.41.0.hot_left.i981.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i981.i, i64 352
  %68 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %69 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %70 = getelementptr inbounds nuw i8, ptr %self, i64 96
  %row12.i.i.i.i1022.i = getelementptr inbounds nuw i8, ptr %self, i64 128
  %71 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %72 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %73 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %row13.i.i.i.i1023.i = getelementptr inbounds nuw i8, ptr %self, i64 256
  %74 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %75 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %76 = getelementptr inbounds nuw i8, ptr %self, i64 352
  %row14.i.i.i.i1024.i = getelementptr inbounds nuw i8, ptr %self, i64 384
  %77 = getelementptr inbounds nuw i8, ptr %self, i64 416
  %78 = getelementptr inbounds nuw i8, ptr %self, i64 448
  %79 = getelementptr inbounds nuw i8, ptr %self, i64 480
  %row15.i.i.i.i1025.i = getelementptr inbounds nuw i8, ptr %self, i64 512
  %80 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %81 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %82 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %row16.i.i.i.i1026.i = getelementptr inbounds nuw i8, ptr %self, i64 640
  %83 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %84 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %85 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %row17.i.i.i.i1027.i = getelementptr inbounds nuw i8, ptr %self, i64 768
  %86 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %87 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %88 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %row18.i.i.i.i1028.i = getelementptr inbounds nuw i8, ptr %self, i64 896
  %89 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %90 = getelementptr inbounds nuw i8, ptr %self, i64 960
  %91 = getelementptr inbounds nuw i8, ptr %self, i64 992
  %row19.i.i.i.i1029.i = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %92 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %93 = getelementptr inbounds nuw i8, ptr %self, i64 1088
  %94 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %row20.i.i.i.i1030.i = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %95 = getelementptr inbounds nuw i8, ptr %self, i64 1184
  %96 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %97 = getelementptr inbounds nuw i8, ptr %self, i64 1248
  %row21.i.i.i.i1031.i = getelementptr inbounds nuw i8, ptr %self, i64 1280
  %98 = getelementptr inbounds nuw i8, ptr %self, i64 1312
  %99 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %100 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %row22.i.i.i.i1032.i = getelementptr inbounds nuw i8, ptr %self, i64 1408
  %101 = getelementptr inbounds nuw i8, ptr %self, i64 1440
  %102 = getelementptr inbounds nuw i8, ptr %self, i64 1472
  %103 = getelementptr inbounds nuw i8, ptr %self, i64 1504
  %_51.i1048.i = getelementptr inbounds nuw i8, ptr %hot_left.i981.i, i64 384
  %_52.i1049.i = getelementptr inbounds nuw i8, ptr %hot_left.i981.i, i64 512
  %104 = getelementptr inbounds nuw i8, ptr %hot_left.i981.i, i64 480
  %105 = getelementptr inbounds nuw i8, ptr %hot_left.i981.i, i64 448
  %106 = getelementptr inbounds nuw i8, ptr %hot_left.i981.i, i64 416
  %107 = getelementptr inbounds nuw i8, ptr %hot_left.i981.i, i64 608
  %108 = getelementptr inbounds nuw i8, ptr %hot_left.i981.i, i64 576
  %109 = getelementptr inbounds nuw i8, ptr %hot_left.i981.i, i64 544
  %110 = select i1 %_21.i982.i, <8 x i32> %66, <8 x i32> %67
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
  %124 = getelementptr inbounds nuw i8, ptr %hot_left.i981.i, i64 672
  %125 = getelementptr inbounds nuw i8, ptr %hot_left.i981.i, i64 704
  %126 = getelementptr inbounds nuw i8, ptr %hot_left.i981.i, i64 640
  %127 = getelementptr inbounds nuw i8, ptr %self, i64 1672
  %128 = getelementptr inbounds nuw i8, ptr %self, i64 1664
  %129 = select i1 %_22.i985.i, <8 x i32> %66, <8 x i32> %67
  %130 = icmp slt <8 x i32> %129, zeroinitializer
  %131 = getelementptr inbounds nuw i8, ptr %self, i64 1632
  %_13.i1460.sroa.0.0.copyload.i = load <8 x float>, ptr %106, align 32, !noalias !6294
  %_13.i1452.sroa.0.0.copyload.i = load <8 x float>, ptr %109, align 32, !noalias !6294
  %_64.i.i943.sroa.0.0.copyload.i = load <8 x float>, ptr %125, align 32, !noalias !6294
  %iter.i.i949.sroa.0.0.ptr7839.1.i = getelementptr inbounds nuw i8, ptr %scratch.i975.i, i64 4
  %iter.i.i949.sroa.0.0.ptr7839.2.i = getelementptr inbounds nuw i8, ptr %scratch.i975.i, i64 8
  %iter.i.i949.sroa.0.0.ptr7839.3.i = getelementptr inbounds nuw i8, ptr %scratch.i975.i, i64 12
  %iter.i.i949.sroa.0.0.ptr7839.4.i = getelementptr inbounds nuw i8, ptr %scratch.i975.i, i64 16
  %iter.i.i949.sroa.0.0.ptr7839.5.i = getelementptr inbounds nuw i8, ptr %scratch.i975.i, i64 20
  %iter.i.i949.sroa.0.0.ptr7839.6.i = getelementptr inbounds nuw i8, ptr %scratch.i975.i, i64 24
  %iter.i.i949.sroa.0.0.ptr7839.7.i = getelementptr inbounds nuw i8, ptr %scratch.i975.i, i64 28
  %hot_left.i981.promoted.i = load <8 x float>, ptr %hot_left.i981.i, align 32, !noalias !6294
  %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.i, align 32, !noalias !6294
  %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.i, align 32, !noalias !6294
  %history.i.i927.sroa.16.0.hot_left.i981.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i927.sroa.16.0.hot_left.i981.sroa_idx.i, align 32, !noalias !6294
  %history.i.i927.sroa.19.0.hot_left.i981.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i927.sroa.19.0.hot_left.i981.sroa_idx.i, align 32, !noalias !6294
  %history.i.i927.sroa.22.0.hot_left.i981.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i927.sroa.22.0.hot_left.i981.sroa_idx.i, align 32, !noalias !6294
  %history.i.i927.sroa.25.0.hot_left.i981.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i927.sroa.25.0.hot_left.i981.sroa_idx.i, align 32, !noalias !6294
  %history.i.i927.sroa.29.0.hot_left.i981.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i927.sroa.29.0.hot_left.i981.sroa_idx.i, align 32, !noalias !6294
  %history.i.i927.sroa.32.0.hot_left.i981.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i927.sroa.32.0.hot_left.i981.sroa_idx.i, align 32, !noalias !6294
  %history.i.i927.sroa.35.0.hot_left.i981.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i927.sroa.35.0.hot_left.i981.sroa_idx.i, align 32, !noalias !6294
  %history.i.i927.sroa.38.0.hot_left.i981.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i927.sroa.38.0.hot_left.i981.sroa_idx.i, align 32, !noalias !6294
  %history.i.i927.sroa.41.0.hot_left.i981.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i927.sroa.41.0.hot_left.i981.sroa_idx.i, align 32, !noalias !6294
  %.promoted10736.i = load <8 x float>, ptr %104, align 32, !noalias !6294
  %_51.i1048.promoted10739.i = load <8 x float>, ptr %_51.i1048.i, align 32, !noalias !6294
  %.promoted.i = load <8 x float>, ptr %105, align 32, !noalias !6294
  %.promoted10744.i = load <8 x float>, ptr %107, align 32, !noalias !6294
  %_52.i1049.promoted10747.i = load <8 x float>, ptr %_52.i1049.i, align 32, !noalias !6294
  %.promoted10750.i = load <8 x float>, ptr %108, align 32, !noalias !6294
  %.promoted10753.i = load <8 x float>, ptr %124, align 32, !noalias !6294
  %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.i.promoted = load <8 x float>, ptr %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.i, align 1
  %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.i.promoted = load <8 x float>, ptr %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.i, align 1
  br label %bb29.i1001.i, !dbg !6244

bb14.i1040.bb12.i995.loopexit_crit_edge.i:        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i
  store <8 x float> %294, ptr %124, align 1, !dbg !6295, !noalias !6294
  store <8 x float> %246, ptr %104, align 32, !dbg !6315, !noalias !6321
  store <8 x float> %251, ptr %_51.i1048.i, align 32, !dbg !6327, !noalias !6321
  store <8 x float> %252, ptr %105, align 32, !dbg !6329, !noalias !6321
  store <8 x float> %254, ptr %107, align 32, !dbg !6330, !noalias !6332
  store <8 x float> %259, ptr %_52.i1049.i, align 32, !dbg !6335, !noalias !6332
  store <8 x float> %260, ptr %108, align 32, !dbg !6336, !noalias !6332
  br label %bb12.i995.loopexit.i, !dbg !6337

bb12.i995.loopexit.i:                             ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1039.i, %bb14.i1040.bb12.i995.loopexit_crit_edge.i
  %.lcssa1070610754.i = phi <8 x float> [ %294, %bb14.i1040.bb12.i995.loopexit_crit_edge.i ], [ %.lcssa1070610755.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1039.i ]
  %.lcssa1046910751.i = phi <8 x float> [ %260, %bb14.i1040.bb12.i995.loopexit_crit_edge.i ], [ %.lcssa1046910752.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1039.i ]
  %.lcssa1048710748.i = phi <8 x float> [ %259, %bb14.i1040.bb12.i995.loopexit_crit_edge.i ], [ %.lcssa1048710749.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1039.i ]
  %.lcssa1050510745.i = phi <8 x float> [ %254, %bb14.i1040.bb12.i995.loopexit_crit_edge.i ], [ %.lcssa1050510746.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1039.i ]
  %.lcssa1052310742.i = phi <8 x float> [ %252, %bb14.i1040.bb12.i995.loopexit_crit_edge.i ], [ %.lcssa1052310743.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1039.i ]
  %.lcssa1054110740.i = phi <8 x float> [ %251, %bb14.i1040.bb12.i995.loopexit_crit_edge.i ], [ %.lcssa1054110741.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1039.i ]
  %.lcssa1055910737.i = phi <8 x float> [ %246, %bb14.i1040.bb12.i995.loopexit_crit_edge.i ], [ %.lcssa1055910738.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1039.i ]
  %ring_cursor.sroa.0.1.i1042.lcssa.i = phi i64 [ %spec.store.select9.i1122.i, %bb14.i1040.bb12.i995.loopexit_crit_edge.i ], [ %ring_cursor.sroa.0.0.i9988090.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1039.i ], !dbg !6343
  %main_cursor.sroa.0.1.i1043.lcssa.i = phi i64 [ %spec.store.select.i1120.i, %bb14.i1040.bb12.i995.loopexit_crit_edge.i ], [ %main_cursor.sroa.0.0.i9998091.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1039.i ], !dbg !6344
  %_79.not.i1000.i = icmp eq i64 %134, 0, !dbg !6244
  %indvars.iv.next.i = add nsw i64 %indvars.iv.i, -32, !dbg !6244
  br i1 %_79.not.i1000.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i, label %bb29.i1001.i, !dbg !6244

bb29.i1001.i:                                     ; preds = %bb12.i995.loopexit.i, %bb29.i1001.lr.ph.i
  %history.i.i927.sroa.13.sroa.0.0.lcssa.i2756 = phi <8 x float> [ %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.i.promoted, %bb29.i1001.lr.ph.i ], [ %history.i.i927.sroa.13.sroa.0.0.lcssa.i, %bb12.i995.loopexit.i ]
  %history.i.i927.sroa.10.sroa.0.0.lcssa.i2738 = phi <8 x float> [ %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.i.promoted, %bb29.i1001.lr.ph.i ], [ %history.i.i927.sroa.10.sroa.0.0.lcssa.i, %bb12.i995.loopexit.i ]
  %.lcssa1070610755.i = phi <8 x float> [ %.promoted10753.i, %bb29.i1001.lr.ph.i ], [ %.lcssa1070610754.i, %bb12.i995.loopexit.i ]
  %.lcssa1046910752.i = phi <8 x float> [ %.promoted10750.i, %bb29.i1001.lr.ph.i ], [ %.lcssa1046910751.i, %bb12.i995.loopexit.i ]
  %.lcssa1048710749.i = phi <8 x float> [ %_52.i1049.promoted10747.i, %bb29.i1001.lr.ph.i ], [ %.lcssa1048710748.i, %bb12.i995.loopexit.i ]
  %.lcssa1050510746.i = phi <8 x float> [ %.promoted10744.i, %bb29.i1001.lr.ph.i ], [ %.lcssa1050510745.i, %bb12.i995.loopexit.i ]
  %.lcssa1052310743.i = phi <8 x float> [ %.promoted.i, %bb29.i1001.lr.ph.i ], [ %.lcssa1052310742.i, %bb12.i995.loopexit.i ]
  %.lcssa1054110741.i = phi <8 x float> [ %_51.i1048.promoted10739.i, %bb29.i1001.lr.ph.i ], [ %.lcssa1054110740.i, %bb12.i995.loopexit.i ]
  %.lcssa1055910738.i = phi <8 x float> [ %.promoted10736.i, %bb29.i1001.lr.ph.i ], [ %.lcssa1055910737.i, %bb12.i995.loopexit.i ]
  %history.i.i927.sroa.41.sroa.0.0.lcssa10735.i = phi <8 x float> [ %history.i.i927.sroa.41.0.hot_left.i981.sroa_idx.promoted.i, %bb29.i1001.lr.ph.i ], [ %history.i.i927.sroa.41.sroa.0.0.lcssa.i, %bb12.i995.loopexit.i ]
  %history.i.i927.sroa.38.sroa.0.0.lcssa10734.i = phi <8 x float> [ %history.i.i927.sroa.38.0.hot_left.i981.sroa_idx.promoted.i, %bb29.i1001.lr.ph.i ], [ %history.i.i927.sroa.38.sroa.0.0.lcssa.i, %bb12.i995.loopexit.i ]
  %history.i.i927.sroa.35.sroa.0.0.lcssa10733.i = phi <8 x float> [ %history.i.i927.sroa.35.0.hot_left.i981.sroa_idx.promoted.i, %bb29.i1001.lr.ph.i ], [ %history.i.i927.sroa.35.sroa.0.0.lcssa.i, %bb12.i995.loopexit.i ]
  %history.i.i927.sroa.32.sroa.0.0.lcssa10732.i = phi <8 x float> [ %history.i.i927.sroa.32.0.hot_left.i981.sroa_idx.promoted.i, %bb29.i1001.lr.ph.i ], [ %history.i.i927.sroa.32.sroa.0.0.lcssa.i, %bb12.i995.loopexit.i ]
  %history.i.i927.sroa.29.sroa.0.0.lcssa10731.i = phi <8 x float> [ %history.i.i927.sroa.29.0.hot_left.i981.sroa_idx.promoted.i, %bb29.i1001.lr.ph.i ], [ %history.i.i927.sroa.29.sroa.0.0.lcssa.i, %bb12.i995.loopexit.i ]
  %history.i.i927.sroa.25.sroa.0.0.lcssa10730.i = phi <8 x float> [ %history.i.i927.sroa.25.0.hot_left.i981.sroa_idx.promoted.i, %bb29.i1001.lr.ph.i ], [ %history.i.i927.sroa.25.sroa.0.0.lcssa.i, %bb12.i995.loopexit.i ]
  %history.i.i927.sroa.22.sroa.0.0.lcssa10729.i = phi <8 x float> [ %history.i.i927.sroa.22.0.hot_left.i981.sroa_idx.promoted.i, %bb29.i1001.lr.ph.i ], [ %history.i.i927.sroa.22.sroa.0.0.lcssa.i, %bb12.i995.loopexit.i ]
  %history.i.i927.sroa.19.sroa.0.0.lcssa10728.i = phi <8 x float> [ %history.i.i927.sroa.19.0.hot_left.i981.sroa_idx.promoted.i, %bb29.i1001.lr.ph.i ], [ %history.i.i927.sroa.19.sroa.0.0.lcssa.i, %bb12.i995.loopexit.i ]
  %history.i.i927.sroa.16.sroa.0.0.lcssa10727.i = phi <8 x float> [ %history.i.i927.sroa.16.0.hot_left.i981.sroa_idx.promoted.i, %bb29.i1001.lr.ph.i ], [ %history.i.i927.sroa.16.sroa.0.0.lcssa.i, %bb12.i995.loopexit.i ]
  %history.i.i927.sroa.13.sroa.0.0.lcssa10726.i = phi <8 x float> [ %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.promoted.i, %bb29.i1001.lr.ph.i ], [ %history.i.i927.sroa.13.sroa.0.0.lcssa.i, %bb12.i995.loopexit.i ]
  %history.i.i927.sroa.10.sroa.0.0.lcssa10725.i = phi <8 x float> [ %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.promoted.i, %bb29.i1001.lr.ph.i ], [ %history.i.i927.sroa.10.sroa.0.0.lcssa.i, %bb12.i995.loopexit.i ]
  %history.i.i927.sroa.0.0.lcssa10707.i = phi <8 x float> [ %hot_left.i981.promoted.i, %bb29.i1001.lr.ph.i ], [ %history.i.i927.sroa.0.0.lcssa.i, %bb12.i995.loopexit.i ]
  %indvars.iv.i = phi i64 [ %_31, %bb29.i1001.lr.ph.i ], [ %indvars.iv.next.i, %bb12.i995.loopexit.i ]
  %main_cursor.sroa.0.0.i9998091.i = phi i64 [ %64, %bb29.i1001.lr.ph.i ], [ %main_cursor.sroa.0.1.i1043.lcssa.i, %bb12.i995.loopexit.i ]
  %ring_cursor.sroa.0.0.i9988090.i = phi i64 [ %63, %bb29.i1001.lr.ph.i ], [ %ring_cursor.sroa.0.1.i1042.lcssa.i, %bb12.i995.loopexit.i ]
  %iter3.sroa.0.0.i9978089.i = phi i64 [ %yield_count.sroa.0.0.i.i.i, %bb29.i1001.lr.ph.i ], [ %134, %bb12.i995.loopexit.i ]
  %iter.sroa.0.0.i9968088.i = phi i64 [ 0, %bb29.i1001.lr.ph.i ], [ %133, %bb12.i995.loopexit.i ]
  %132 = tail call i64 @llvm.umax.i64(i64 %indvars.iv.i, i64 1), !dbg !6345
  %umax9597.i = tail call i64 @llvm.umin.i64(i64 %132, i64 32), !dbg !6345
  %133 = add nuw nsw i64 %iter.sroa.0.0.i9968088.i, 32, !dbg !6345
  %134 = add nsw i64 %iter3.sroa.0.0.i9978089.i, -1, !dbg !6349
  %_34.i1003.i = sub nsw i64 %_31, %iter.sroa.0.0.i9968088.i, !dbg !6350
  %..i.i = tail call noundef i64 @llvm.umin.i64(i64 %_34.i1003.i, i64 32), !dbg !6351
  %active_base.i1005.i = shl i64 %iter.sroa.0.0.i9968088.i, 3, !dbg !6355
  %active_base.i10057047.i = add nuw nsw i64 %..i.i, %iter.sroa.0.0.i9968088.i, !dbg !6356
  %_40.i1007.i = shl i64 %active_base.i10057047.i, 3, !dbg !6356
  %_91.i1008.i = icmp samesign ult i64 %_40.i1007.i, %active_base.i1005.i, !dbg !6357
  %_85.not.i1009.i = icmp ugt i64 %_40.i1007.i, %_39.1
  %or.cond.i1010.i = or i1 %_91.i1008.i, %_85.not.i1009.i, !dbg !6357
  br i1 %or.cond.i1010.i, label %bb33.i1130.i, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit.i, !dbg !6357, !prof !165

bb33.i1130.i:                                     ; preds = %bb29.i1001.i
  store <8 x float> %history.i.i927.sroa.10.sroa.0.0.lcssa.i2738, ptr %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
  store <8 x float> %history.i.i927.sroa.13.sroa.0.0.lcssa.i2756, ptr %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %active_base.i1005.i, i64 noundef %_40.i1007.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1c9f1dd5a676136ffd1db4032fbd14da) #30, !dbg !6369, !noalias !6370
  unreachable, !dbg !6369

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit.i: ; preds = %bb29.i1001.i
  %_94.i1015.i = getelementptr inbounds nuw float, ptr %_39.0, i64 %active_base.i1005.i, !dbg !6371
  %_2.i38727805.not.i = icmp eq i64 %iter.sroa.0.0.i9968088.i, %_31, !dbg !6375
  br i1 %_2.i38727805.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1039.i, label %bb6.i.i1018.lr.ph.i, !dbg !6375

bb6.i.i1018.lr.ph.i:                              ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit.i
  %_5.i2516.i = load <8 x float>, ptr %self, align 32, !alias.scope !6381, !noalias !6384
  %_14.i.i.i.i887.sroa.0.0.copyload.i = load <8 x float>, ptr %68, align 32, !alias.scope !5946, !noalias !6396
  %_17.i.i.i.i884.sroa.0.0.copyload.i = load <8 x float>, ptr %69, align 32, !alias.scope !5946, !noalias !6396
  %_20.i.i.i.i881.sroa.0.0.copyload.i = load <8 x float>, ptr %70, align 32, !alias.scope !5946, !noalias !6396
  %_25.i.i.i.i877.sroa.0.0.copyload.i = load <8 x float>, ptr %row12.i.i.i.i1022.i, align 32, !alias.scope !5946, !noalias !6396
  %_28.i.i.i.i874.sroa.0.0.copyload.i = load <8 x float>, ptr %71, align 32, !alias.scope !5946, !noalias !6396
  %_31.i.i.i.i871.sroa.0.0.copyload.i = load <8 x float>, ptr %72, align 32, !alias.scope !5946, !noalias !6396
  %_34.i.i.i.i868.sroa.0.0.copyload.i = load <8 x float>, ptr %73, align 32, !alias.scope !5946, !noalias !6396
  %_39.i.i.i.i864.sroa.0.0.copyload.i = load <8 x float>, ptr %row13.i.i.i.i1023.i, align 32, !alias.scope !5946, !noalias !6396
  %_42.i.i.i.i861.sroa.0.0.copyload.i = load <8 x float>, ptr %74, align 32, !alias.scope !5946, !noalias !6396
  %_45.i.i.i.i858.sroa.0.0.copyload.i = load <8 x float>, ptr %75, align 32, !alias.scope !5946, !noalias !6396
  %_48.i.i.i.i855.sroa.0.0.copyload.i = load <8 x float>, ptr %76, align 32, !alias.scope !5946, !noalias !6396
  %_53.i.i.i.i851.sroa.0.0.copyload.i = load <8 x float>, ptr %row14.i.i.i.i1024.i, align 32, !alias.scope !5946, !noalias !6396
  %_56.i.i.i.i848.sroa.0.0.copyload.i = load <8 x float>, ptr %77, align 32, !alias.scope !5946, !noalias !6396
  %_59.i.i.i.i845.sroa.0.0.copyload.i = load <8 x float>, ptr %78, align 32, !alias.scope !5946, !noalias !6396
  %_62.i.i.i.i842.sroa.0.0.copyload.i = load <8 x float>, ptr %79, align 32, !alias.scope !5946, !noalias !6396
  %_67.i.i.i.i838.sroa.0.0.copyload.i = load <8 x float>, ptr %row15.i.i.i.i1025.i, align 32, !alias.scope !5946, !noalias !6396
  %_70.i.i.i.i835.sroa.0.0.copyload.i = load <8 x float>, ptr %80, align 32, !alias.scope !5946, !noalias !6396
  %_73.i.i.i.i832.sroa.0.0.copyload.i = load <8 x float>, ptr %81, align 32, !alias.scope !5946, !noalias !6396
  %_76.i.i.i.i829.sroa.0.0.copyload.i = load <8 x float>, ptr %82, align 32, !alias.scope !5946, !noalias !6396
  %_81.i.i.i.i825.sroa.0.0.copyload.i = load <8 x float>, ptr %row16.i.i.i.i1026.i, align 32, !alias.scope !5946, !noalias !6396
  %_84.i.i.i.i822.sroa.0.0.copyload.i = load <8 x float>, ptr %83, align 32, !alias.scope !5946, !noalias !6396
  %_87.i.i.i.i819.sroa.0.0.copyload.i = load <8 x float>, ptr %84, align 32, !alias.scope !5946, !noalias !6396
  %_90.i.i.i.i816.sroa.0.0.copyload.i = load <8 x float>, ptr %85, align 32, !alias.scope !5946, !noalias !6396
  %_95.i.i.i.i812.sroa.0.0.copyload.i = load <8 x float>, ptr %row17.i.i.i.i1027.i, align 32, !alias.scope !5946, !noalias !6396
  %_98.i.i.i.i809.sroa.0.0.copyload.i = load <8 x float>, ptr %86, align 32, !alias.scope !5946, !noalias !6396
  %_101.i.i.i.i806.sroa.0.0.copyload.i = load <8 x float>, ptr %87, align 32, !alias.scope !5946, !noalias !6396
  %_104.i.i.i.i803.sroa.0.0.copyload.i = load <8 x float>, ptr %88, align 32, !alias.scope !5946, !noalias !6396
  %_109.i.i.i.i799.sroa.0.0.copyload.i = load <8 x float>, ptr %row18.i.i.i.i1028.i, align 32, !alias.scope !5946, !noalias !6396
  %_112.i.i.i.i796.sroa.0.0.copyload.i = load <8 x float>, ptr %89, align 32, !alias.scope !5946, !noalias !6396
  %_115.i.i.i.i793.sroa.0.0.copyload.i = load <8 x float>, ptr %90, align 32, !alias.scope !5946, !noalias !6396
  %_118.i.i.i.i790.sroa.0.0.copyload.i = load <8 x float>, ptr %91, align 32, !alias.scope !5946, !noalias !6396
  %_123.i.i.i.i786.sroa.0.0.copyload.i = load <8 x float>, ptr %row19.i.i.i.i1029.i, align 32, !alias.scope !5946, !noalias !6396
  %_126.i.i.i.i783.sroa.0.0.copyload.i = load <8 x float>, ptr %92, align 32, !alias.scope !5946, !noalias !6396
  %_129.i.i.i.i780.sroa.0.0.copyload.i = load <8 x float>, ptr %93, align 32, !alias.scope !5946, !noalias !6396
  %_132.i.i.i.i777.sroa.0.0.copyload.i = load <8 x float>, ptr %94, align 32, !alias.scope !5946, !noalias !6396
  %_137.i.i.i.i773.sroa.0.0.copyload.i = load <8 x float>, ptr %row20.i.i.i.i1030.i, align 32, !alias.scope !5946, !noalias !6396
  %_140.i.i.i.i770.sroa.0.0.copyload.i = load <8 x float>, ptr %95, align 32, !alias.scope !5946, !noalias !6396
  %_143.i.i.i.i767.sroa.0.0.copyload.i = load <8 x float>, ptr %96, align 32, !alias.scope !5946, !noalias !6396
  %_146.i.i.i.i764.sroa.0.0.copyload.i = load <8 x float>, ptr %97, align 32, !alias.scope !5946, !noalias !6396
  %_151.i.i.i.i760.sroa.0.0.copyload.i = load <8 x float>, ptr %row21.i.i.i.i1031.i, align 32, !alias.scope !5946, !noalias !6396
  %_154.i.i.i.i757.sroa.0.0.copyload.i = load <8 x float>, ptr %98, align 32, !alias.scope !5946, !noalias !6396
  %_157.i.i.i.i754.sroa.0.0.copyload.i = load <8 x float>, ptr %99, align 32, !alias.scope !5946, !noalias !6396
  %_160.i.i.i.i751.sroa.0.0.copyload.i = load <8 x float>, ptr %100, align 32, !alias.scope !5946, !noalias !6396
  %_165.i.i.i.i747.sroa.0.0.copyload.i = load <8 x float>, ptr %row22.i.i.i.i1032.i, align 32, !alias.scope !5946, !noalias !6396
  %_168.i.i.i.i744.sroa.0.0.copyload.i = load <8 x float>, ptr %101, align 32, !alias.scope !5946, !noalias !6396
  %_171.i.i.i.i741.sroa.0.0.copyload.i = load <8 x float>, ptr %102, align 32, !alias.scope !5946, !noalias !6396
  %_174.i.i.i.i738.sroa.0.0.copyload.i = load <8 x float>, ptr %103, align 32, !alias.scope !5946, !noalias !6396
  br label %bb6.i.i1018.i, !dbg !6375

bb6.i.i1018.i:                                    ; preds = %bb6.i.i1018.i, %bb6.i.i1018.lr.ph.i
  %history.i.i927.sroa.10.sroa.0.07817.i = phi <8 x float> [ %history.i.i927.sroa.10.sroa.0.0.lcssa10725.i, %bb6.i.i1018.lr.ph.i ], [ %history.i.i927.sroa.0.07806.i, %bb6.i.i1018.i ]
  %history.i.i927.sroa.13.sroa.0.07816.i = phi <8 x float> [ %history.i.i927.sroa.13.sroa.0.0.lcssa10726.i, %bb6.i.i1018.lr.ph.i ], [ %history.i.i927.sroa.10.sroa.0.07817.i, %bb6.i.i1018.i ]
  %history.i.i927.sroa.16.sroa.0.07815.i = phi <8 x float> [ %history.i.i927.sroa.16.sroa.0.0.lcssa10727.i, %bb6.i.i1018.lr.ph.i ], [ %history.i.i927.sroa.13.sroa.0.07816.i, %bb6.i.i1018.i ]
  %history.i.i927.sroa.19.sroa.0.07814.i = phi <8 x float> [ %history.i.i927.sroa.19.sroa.0.0.lcssa10728.i, %bb6.i.i1018.lr.ph.i ], [ %history.i.i927.sroa.16.sroa.0.07815.i, %bb6.i.i1018.i ]
  %history.i.i927.sroa.22.sroa.0.07813.i = phi <8 x float> [ %history.i.i927.sroa.22.sroa.0.0.lcssa10729.i, %bb6.i.i1018.lr.ph.i ], [ %history.i.i927.sroa.19.sroa.0.07814.i, %bb6.i.i1018.i ]
  %history.i.i927.sroa.38.sroa.0.07812.i = phi <8 x float> [ %history.i.i927.sroa.38.sroa.0.0.lcssa10734.i, %bb6.i.i1018.lr.ph.i ], [ %history.i.i927.sroa.35.sroa.0.07811.i, %bb6.i.i1018.i ]
  %history.i.i927.sroa.35.sroa.0.07811.i = phi <8 x float> [ %history.i.i927.sroa.35.sroa.0.0.lcssa10733.i, %bb6.i.i1018.lr.ph.i ], [ %history.i.i927.sroa.32.sroa.0.07810.i, %bb6.i.i1018.i ]
  %history.i.i927.sroa.32.sroa.0.07810.i = phi <8 x float> [ %history.i.i927.sroa.32.sroa.0.0.lcssa10732.i, %bb6.i.i1018.lr.ph.i ], [ %history.i.i927.sroa.29.sroa.0.07809.i, %bb6.i.i1018.i ]
  %history.i.i927.sroa.29.sroa.0.07809.i = phi <8 x float> [ %history.i.i927.sroa.29.sroa.0.0.lcssa10731.i, %bb6.i.i1018.lr.ph.i ], [ %history.i.i927.sroa.25.sroa.0.07808.i, %bb6.i.i1018.i ]
  %history.i.i927.sroa.25.sroa.0.07808.i = phi <8 x float> [ %history.i.i927.sroa.25.sroa.0.0.lcssa10730.i, %bb6.i.i1018.lr.ph.i ], [ %history.i.i927.sroa.22.sroa.0.07813.i, %bb6.i.i1018.i ]
  %iter.i19.i923.sroa.16.07807.i = phi i64 [ 0, %bb6.i.i1018.lr.ph.i ], [ %240, %bb6.i.i1018.i ]
  %history.i.i927.sroa.0.07806.i = phi <8 x float> [ %history.i.i927.sroa.0.0.lcssa10707.i, %bb6.i.i1018.lr.ph.i ], [ %lanes.i3171.sroa.0.0.copyload.i, %bb6.i.i1018.i ]
  %start1.i.i.i = shl i64 %iter.i19.i923.sroa.16.07807.i, 3, !dbg !6399
  %data.i.i3875.i = getelementptr inbounds nuw float, ptr %_94.i1015.i, i64 %start1.i.i.i, !dbg !6401
  %lanes.i3171.sroa.0.0.copyload.i = load <8 x float>, ptr %data.i.i3875.i, align 4, !dbg !6403, !alias.scope !6412, !noalias !6416
  %135 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i.i927.sroa.22.sroa.0.07813.i), !dbg !6421
  %136 = fmul <8 x float> %_5.i2516.i, %lanes.i3171.sroa.0.0.copyload.i, !dbg !6439
  %137 = fadd <8 x float> %136, zeroinitializer, !dbg !6458
  %138 = fmul <8 x float> %_14.i.i.i.i887.sroa.0.0.copyload.i, %lanes.i3171.sroa.0.0.copyload.i, !dbg !6468
  %139 = fadd <8 x float> %138, zeroinitializer, !dbg !6473
  %140 = fmul <8 x float> %_17.i.i.i.i884.sroa.0.0.copyload.i, %lanes.i3171.sroa.0.0.copyload.i, !dbg !6478
  %141 = fadd <8 x float> %140, zeroinitializer, !dbg !6483
  %142 = fmul <8 x float> %_20.i.i.i.i881.sroa.0.0.copyload.i, %lanes.i3171.sroa.0.0.copyload.i, !dbg !6488
  %143 = fadd <8 x float> %142, zeroinitializer, !dbg !6493
  %144 = fmul <8 x float> %_25.i.i.i.i877.sroa.0.0.copyload.i, %history.i.i927.sroa.0.07806.i, !dbg !6498
  %145 = fadd <8 x float> %144, %137, !dbg !6505
  %146 = fmul <8 x float> %_28.i.i.i.i874.sroa.0.0.copyload.i, %history.i.i927.sroa.0.07806.i, !dbg !6510
  %147 = fadd <8 x float> %146, %139, !dbg !6515
  %148 = fmul <8 x float> %_31.i.i.i.i871.sroa.0.0.copyload.i, %history.i.i927.sroa.0.07806.i, !dbg !6520
  %149 = fadd <8 x float> %148, %141, !dbg !6525
  %150 = fmul <8 x float> %_34.i.i.i.i868.sroa.0.0.copyload.i, %history.i.i927.sroa.0.07806.i, !dbg !6530
  %151 = fadd <8 x float> %150, %143, !dbg !6535
  %152 = fmul <8 x float> %_39.i.i.i.i864.sroa.0.0.copyload.i, %history.i.i927.sroa.10.sroa.0.07817.i, !dbg !6540
  %153 = fadd <8 x float> %152, %145, !dbg !6547
  %154 = fmul <8 x float> %_42.i.i.i.i861.sroa.0.0.copyload.i, %history.i.i927.sroa.10.sroa.0.07817.i, !dbg !6552
  %155 = fadd <8 x float> %154, %147, !dbg !6557
  %156 = fmul <8 x float> %_45.i.i.i.i858.sroa.0.0.copyload.i, %history.i.i927.sroa.10.sroa.0.07817.i, !dbg !6562
  %157 = fadd <8 x float> %156, %149, !dbg !6567
  %158 = fmul <8 x float> %_48.i.i.i.i855.sroa.0.0.copyload.i, %history.i.i927.sroa.10.sroa.0.07817.i, !dbg !6572
  %159 = fadd <8 x float> %158, %151, !dbg !6577
  %160 = fmul <8 x float> %_53.i.i.i.i851.sroa.0.0.copyload.i, %history.i.i927.sroa.13.sroa.0.07816.i, !dbg !6582
  %161 = fadd <8 x float> %160, %153, !dbg !6589
  %162 = fmul <8 x float> %_56.i.i.i.i848.sroa.0.0.copyload.i, %history.i.i927.sroa.13.sroa.0.07816.i, !dbg !6594
  %163 = fadd <8 x float> %162, %155, !dbg !6599
  %164 = fmul <8 x float> %_59.i.i.i.i845.sroa.0.0.copyload.i, %history.i.i927.sroa.13.sroa.0.07816.i, !dbg !6604
  %165 = fadd <8 x float> %164, %157, !dbg !6609
  %166 = fmul <8 x float> %_62.i.i.i.i842.sroa.0.0.copyload.i, %history.i.i927.sroa.13.sroa.0.07816.i, !dbg !6614
  %167 = fadd <8 x float> %166, %159, !dbg !6619
  %168 = fmul <8 x float> %_67.i.i.i.i838.sroa.0.0.copyload.i, %history.i.i927.sroa.16.sroa.0.07815.i, !dbg !6624
  %169 = fadd <8 x float> %168, %161, !dbg !6631
  %170 = fmul <8 x float> %_70.i.i.i.i835.sroa.0.0.copyload.i, %history.i.i927.sroa.16.sroa.0.07815.i, !dbg !6636
  %171 = fadd <8 x float> %170, %163, !dbg !6641
  %172 = fmul <8 x float> %_73.i.i.i.i832.sroa.0.0.copyload.i, %history.i.i927.sroa.16.sroa.0.07815.i, !dbg !6646
  %173 = fadd <8 x float> %172, %165, !dbg !6651
  %174 = fmul <8 x float> %_76.i.i.i.i829.sroa.0.0.copyload.i, %history.i.i927.sroa.16.sroa.0.07815.i, !dbg !6656
  %175 = fadd <8 x float> %174, %167, !dbg !6661
  %176 = fmul <8 x float> %_81.i.i.i.i825.sroa.0.0.copyload.i, %history.i.i927.sroa.19.sroa.0.07814.i, !dbg !6666
  %177 = fadd <8 x float> %176, %169, !dbg !6673
  %178 = fmul <8 x float> %_84.i.i.i.i822.sroa.0.0.copyload.i, %history.i.i927.sroa.19.sroa.0.07814.i, !dbg !6678
  %179 = fadd <8 x float> %178, %171, !dbg !6683
  %180 = fmul <8 x float> %_87.i.i.i.i819.sroa.0.0.copyload.i, %history.i.i927.sroa.19.sroa.0.07814.i, !dbg !6688
  %181 = fadd <8 x float> %180, %173, !dbg !6693
  %182 = fmul <8 x float> %_90.i.i.i.i816.sroa.0.0.copyload.i, %history.i.i927.sroa.19.sroa.0.07814.i, !dbg !6698
  %183 = fadd <8 x float> %182, %175, !dbg !6703
  %184 = fmul <8 x float> %_95.i.i.i.i812.sroa.0.0.copyload.i, %history.i.i927.sroa.22.sroa.0.07813.i, !dbg !6708
  %185 = fadd <8 x float> %184, %177, !dbg !6715
  %186 = fmul <8 x float> %_98.i.i.i.i809.sroa.0.0.copyload.i, %history.i.i927.sroa.22.sroa.0.07813.i, !dbg !6720
  %187 = fadd <8 x float> %186, %179, !dbg !6725
  %188 = fmul <8 x float> %_101.i.i.i.i806.sroa.0.0.copyload.i, %history.i.i927.sroa.22.sroa.0.07813.i, !dbg !6730
  %189 = fadd <8 x float> %188, %181, !dbg !6735
  %190 = fmul <8 x float> %_104.i.i.i.i803.sroa.0.0.copyload.i, %history.i.i927.sroa.22.sroa.0.07813.i, !dbg !6740
  %191 = fadd <8 x float> %190, %183, !dbg !6745
  %192 = fmul <8 x float> %_109.i.i.i.i799.sroa.0.0.copyload.i, %history.i.i927.sroa.25.sroa.0.07808.i, !dbg !6750
  %193 = fadd <8 x float> %192, %185, !dbg !6757
  %194 = fmul <8 x float> %_112.i.i.i.i796.sroa.0.0.copyload.i, %history.i.i927.sroa.25.sroa.0.07808.i, !dbg !6762
  %195 = fadd <8 x float> %194, %187, !dbg !6767
  %196 = fmul <8 x float> %_115.i.i.i.i793.sroa.0.0.copyload.i, %history.i.i927.sroa.25.sroa.0.07808.i, !dbg !6772
  %197 = fadd <8 x float> %196, %189, !dbg !6777
  %198 = fmul <8 x float> %_118.i.i.i.i790.sroa.0.0.copyload.i, %history.i.i927.sroa.25.sroa.0.07808.i, !dbg !6782
  %199 = fadd <8 x float> %198, %191, !dbg !6787
  %200 = fmul <8 x float> %_123.i.i.i.i786.sroa.0.0.copyload.i, %history.i.i927.sroa.29.sroa.0.07809.i, !dbg !6792
  %201 = fadd <8 x float> %200, %193, !dbg !6799
  %202 = fmul <8 x float> %_126.i.i.i.i783.sroa.0.0.copyload.i, %history.i.i927.sroa.29.sroa.0.07809.i, !dbg !6804
  %203 = fadd <8 x float> %202, %195, !dbg !6809
  %204 = fmul <8 x float> %_129.i.i.i.i780.sroa.0.0.copyload.i, %history.i.i927.sroa.29.sroa.0.07809.i, !dbg !6814
  %205 = fadd <8 x float> %204, %197, !dbg !6819
  %206 = fmul <8 x float> %_132.i.i.i.i777.sroa.0.0.copyload.i, %history.i.i927.sroa.29.sroa.0.07809.i, !dbg !6824
  %207 = fadd <8 x float> %206, %199, !dbg !6829
  %208 = fmul <8 x float> %_137.i.i.i.i773.sroa.0.0.copyload.i, %history.i.i927.sroa.32.sroa.0.07810.i, !dbg !6834
  %209 = fadd <8 x float> %208, %201, !dbg !6841
  %210 = fmul <8 x float> %_140.i.i.i.i770.sroa.0.0.copyload.i, %history.i.i927.sroa.32.sroa.0.07810.i, !dbg !6846
  %211 = fadd <8 x float> %210, %203, !dbg !6851
  %212 = fmul <8 x float> %_143.i.i.i.i767.sroa.0.0.copyload.i, %history.i.i927.sroa.32.sroa.0.07810.i, !dbg !6856
  %213 = fadd <8 x float> %212, %205, !dbg !6861
  %214 = fmul <8 x float> %_146.i.i.i.i764.sroa.0.0.copyload.i, %history.i.i927.sroa.32.sroa.0.07810.i, !dbg !6866
  %215 = fadd <8 x float> %214, %207, !dbg !6871
  %216 = fmul <8 x float> %_151.i.i.i.i760.sroa.0.0.copyload.i, %history.i.i927.sroa.35.sroa.0.07811.i, !dbg !6876
  %217 = fadd <8 x float> %216, %209, !dbg !6883
  %218 = fmul <8 x float> %_154.i.i.i.i757.sroa.0.0.copyload.i, %history.i.i927.sroa.35.sroa.0.07811.i, !dbg !6888
  %219 = fadd <8 x float> %218, %211, !dbg !6893
  %220 = fmul <8 x float> %_157.i.i.i.i754.sroa.0.0.copyload.i, %history.i.i927.sroa.35.sroa.0.07811.i, !dbg !6898
  %221 = fadd <8 x float> %220, %213, !dbg !6903
  %222 = fmul <8 x float> %_160.i.i.i.i751.sroa.0.0.copyload.i, %history.i.i927.sroa.35.sroa.0.07811.i, !dbg !6908
  %223 = fadd <8 x float> %222, %215, !dbg !6913
  %224 = fmul <8 x float> %_165.i.i.i.i747.sroa.0.0.copyload.i, %history.i.i927.sroa.38.sroa.0.07812.i, !dbg !6918
  %225 = fadd <8 x float> %224, %217, !dbg !6925
  %226 = fmul <8 x float> %_168.i.i.i.i744.sroa.0.0.copyload.i, %history.i.i927.sroa.38.sroa.0.07812.i, !dbg !6930
  %227 = fadd <8 x float> %226, %219, !dbg !6935
  %228 = fmul <8 x float> %_171.i.i.i.i741.sroa.0.0.copyload.i, %history.i.i927.sroa.38.sroa.0.07812.i, !dbg !6940
  %229 = fadd <8 x float> %228, %221, !dbg !6945
  %230 = fmul <8 x float> %_174.i.i.i.i738.sroa.0.0.copyload.i, %history.i.i927.sroa.38.sroa.0.07812.i, !dbg !6950
  %231 = fadd <8 x float> %230, %223, !dbg !6955
  %232 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %225), !dbg !6960
  %233 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %135, <8 x float> %232), !dbg !6968
  %234 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %227), !dbg !6960
  %235 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %233, <8 x float> %234), !dbg !6968
  %236 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %229), !dbg !6960
  %237 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %235, <8 x float> %236), !dbg !6968
  %238 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %231), !dbg !6960
  %239 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %237, <8 x float> %238), !dbg !6968
  %240 = add nuw nsw i64 %iter.i19.i923.sroa.16.07807.i, 1, !dbg !6977
  %data.i4.i.i = getelementptr inbounds nuw float, ptr %peaks_left.i974.i, i64 %start1.i.i.i, !dbg !6978
  store <8 x float> %239, ptr %data.i4.i.i, align 4, !dbg !6981, !alias.scope !6988, !noalias !6992
  %exitcond.not.i = icmp eq i64 %240, %umax9597.i, !dbg !6375
  br i1 %exitcond.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1039.i, label %bb6.i.i1018.i, !dbg !6375

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1039.i: ; preds = %bb6.i.i1018.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit.i
  %history.i.i927.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i927.sroa.0.0.lcssa10707.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit.i ], [ %lanes.i3171.sroa.0.0.copyload.i, %bb6.i.i1018.i ], !dbg !6996
  %history.i.i927.sroa.25.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i927.sroa.25.sroa.0.0.lcssa10730.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit.i ], [ %history.i.i927.sroa.22.sroa.0.07813.i, %bb6.i.i1018.i ], !dbg !6996
  %history.i.i927.sroa.29.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i927.sroa.29.sroa.0.0.lcssa10731.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit.i ], [ %history.i.i927.sroa.25.sroa.0.07808.i, %bb6.i.i1018.i ], !dbg !6996
  %history.i.i927.sroa.32.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i927.sroa.32.sroa.0.0.lcssa10732.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit.i ], [ %history.i.i927.sroa.29.sroa.0.07809.i, %bb6.i.i1018.i ], !dbg !6996
  %history.i.i927.sroa.35.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i927.sroa.35.sroa.0.0.lcssa10733.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit.i ], [ %history.i.i927.sroa.32.sroa.0.07810.i, %bb6.i.i1018.i ], !dbg !6996
  %history.i.i927.sroa.38.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i927.sroa.38.sroa.0.0.lcssa10734.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit.i ], [ %history.i.i927.sroa.35.sroa.0.07811.i, %bb6.i.i1018.i ], !dbg !6996
  %history.i.i927.sroa.41.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i927.sroa.41.sroa.0.0.lcssa10735.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit.i ], [ %history.i.i927.sroa.38.sroa.0.07812.i, %bb6.i.i1018.i ], !dbg !6996
  %history.i.i927.sroa.22.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i927.sroa.22.sroa.0.0.lcssa10729.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit.i ], [ %history.i.i927.sroa.19.sroa.0.07814.i, %bb6.i.i1018.i ], !dbg !6996
  %history.i.i927.sroa.19.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i927.sroa.19.sroa.0.0.lcssa10728.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit.i ], [ %history.i.i927.sroa.16.sroa.0.07815.i, %bb6.i.i1018.i ], !dbg !6996
  %history.i.i927.sroa.16.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i927.sroa.16.sroa.0.0.lcssa10727.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit.i ], [ %history.i.i927.sroa.13.sroa.0.07816.i, %bb6.i.i1018.i ], !dbg !6996
  %history.i.i927.sroa.13.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i927.sroa.13.sroa.0.0.lcssa10726.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit.i ], [ %history.i.i927.sroa.10.sroa.0.07817.i, %bb6.i.i1018.i ], !dbg !6996
  %history.i.i927.sroa.10.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i927.sroa.10.sroa.0.0.lcssa10725.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit.i ], [ %history.i.i927.sroa.0.07806.i, %bb6.i.i1018.i ], !dbg !6996
  store <8 x float> %history.i.i927.sroa.16.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.16.0.hot_left.i981.sroa_idx.i, align 32, !dbg !6364, !noalias !6997
  store <8 x float> %history.i.i927.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.19.0.hot_left.i981.sroa_idx.i, align 32, !dbg !6364, !noalias !6997
  store <8 x float> %history.i.i927.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.22.0.hot_left.i981.sroa_idx.i, align 32, !dbg !6364, !noalias !6997
  store <8 x float> %history.i.i927.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.25.0.hot_left.i981.sroa_idx.i, align 32, !dbg !6364, !noalias !6997
  store <8 x float> %history.i.i927.sroa.29.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.29.0.hot_left.i981.sroa_idx.i, align 32, !dbg !6364, !noalias !6997
  store <8 x float> %history.i.i927.sroa.32.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.32.0.hot_left.i981.sroa_idx.i, align 32, !dbg !6364, !noalias !6997
  store <8 x float> %history.i.i927.sroa.35.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.35.0.hot_left.i981.sroa_idx.i, align 32, !dbg !6364, !noalias !6997
  store <8 x float> %history.i.i927.sroa.38.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.38.0.hot_left.i981.sroa_idx.i, align 32, !dbg !6364, !noalias !6997
  store <8 x float> %history.i.i927.sroa.41.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.41.0.hot_left.i981.sroa_idx.i, align 32, !dbg !6364, !noalias !6997
  br i1 %_2.i38727805.not.i, label %bb12.i995.loopexit.i, label %bb40.i1045.lr.ph.i, !dbg !6337

bb40.i1045.lr.ph.i:                               ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1039.i
  %_64.i1059.i = load i64, ptr %112, align 8, !alias.scope !5946, !noalias !5947
  %_71.i1118.i = load i64, ptr %131, align 8, !alias.scope !5946, !noalias !5947
  br label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3169.i, !dbg !6337

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3169.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i, %bb40.i1045.lr.ph.i
  %241 = phi <8 x float> [ %.lcssa1070610755.i, %bb40.i1045.lr.ph.i ], [ %294, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ]
  %_12.i1453.sroa.0.0.copyload8045.i = phi <8 x float> [ %.lcssa1046910752.i, %bb40.i1045.lr.ph.i ], [ %260, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ]
  %_11.i.sroa.0.0.copyload8005.i = phi <8 x float> [ %.lcssa1048710749.i, %bb40.i1045.lr.ph.i ], [ %259, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ]
  %242 = phi <8 x float> [ %.lcssa1050510746.i, %bb40.i1045.lr.ph.i ], [ %254, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ]
  %_12.i1461.sroa.0.0.copyload7925.i = phi <8 x float> [ %.lcssa1052310743.i, %bb40.i1045.lr.ph.i ], [ %252, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ]
  %_11.i1462.sroa.0.0.copyload7885.i = phi <8 x float> [ %.lcssa1054110741.i, %bb40.i1045.lr.ph.i ], [ %251, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ]
  %243 = phi <8 x float> [ %.lcssa1055910738.i, %bb40.i1045.lr.ph.i ], [ %246, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ]
  %main_cursor.sroa.0.1.i10437843.i = phi i64 [ %main_cursor.sroa.0.0.i9998091.i, %bb40.i1045.lr.ph.i ], [ %spec.store.select.i1120.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ]
  %ring_cursor.sroa.0.1.i10427842.i = phi i64 [ %ring_cursor.sroa.0.0.i9988090.i, %bb40.i1045.lr.ph.i ], [ %spec.store.select9.i1122.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ]
  %iter2.sroa.0.0.i10417841.i = phi i64 [ 0, %bb40.i1045.lr.ph.i ], [ %244, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ]
  %244 = add nuw nsw i64 %iter2.sroa.0.0.i10417841.i, 1, !dbg !6998
  %_47.i1046.i = add nuw nsw i64 %iter2.sroa.0.0.i10417841.i, %iter.sroa.0.0.i9968088.i, !dbg !7004
  %base.i1047.i = shl i64 %_47.i1046.i, 3, !dbg !7004
  %245 = fadd <8 x float> %243, splat (float -1.000000e+00), !dbg !7005
  %246 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %245, <8 x float> zeroinitializer), !dbg !7015
  %247 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %246, <8 x float> zeroinitializer, i8 30), !dbg !7020
  %248 = fadd <8 x float> %_12.i1461.sroa.0.0.copyload7925.i, %_11.i1462.sroa.0.0.copyload7885.i, !dbg !7032
  %249 = bitcast <8 x float> %247 to <8 x i32>, !dbg !7037
  %250 = icmp slt <8 x i32> %249, zeroinitializer, !dbg !7044
  %251 = select <8 x i1> %250, <8 x float> %248, <8 x float> %_13.i1460.sroa.0.0.copyload.i, !dbg !7044
  %252 = select <8 x i1> %250, <8 x float> %_12.i1461.sroa.0.0.copyload7925.i, <8 x float> zeroinitializer, !dbg !7048
  %253 = fadd <8 x float> %242, splat (float -1.000000e+00), !dbg !7053
  %254 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %253, <8 x float> zeroinitializer), !dbg !7058
  %255 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %254, <8 x float> zeroinitializer, i8 30), !dbg !7063
  %256 = fadd <8 x float> %_12.i1453.sroa.0.0.copyload8045.i, %_11.i.sroa.0.0.copyload8005.i, !dbg !7069
  %257 = bitcast <8 x float> %255 to <8 x i32>, !dbg !7074
  %258 = icmp slt <8 x i32> %257, zeroinitializer, !dbg !7078
  %259 = select <8 x i1> %258, <8 x float> %256, <8 x float> %_13.i1452.sroa.0.0.copyload.i, !dbg !7078
  %260 = select <8 x i1> %258, <8 x float> %_12.i1453.sroa.0.0.copyload8045.i, <8 x float> zeroinitializer, !dbg !7080
  %_113.i1054.idx.i = shl i64 %iter2.sroa.0.0.i10417841.i, 5, !dbg !7085
  %_113.i1054.i = getelementptr inbounds nuw i8, ptr %peaks_left.i974.i, i64 %_113.i1054.idx.i, !dbg !7085
  %lanes.i3162.sroa.0.0.copyload.i = load <8 x float>, ptr %_113.i1054.i, align 4, !dbg !7096, !alias.scope !7101, !noalias !7105
  %261 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i3162.sroa.0.0.copyload.i, <8 x float> %lanes.i3162.sroa.0.0.copyload.i), !dbg !7109
  %262 = select <8 x i1> %111, <8 x float> %261, <8 x float> %lanes.i3162.sroa.0.0.copyload.i, !dbg !7114
  %_114.i1055.i = icmp samesign ugt i64 %base.i1047.i, %_39.1, !dbg !7119
  br i1 %_114.i1055.i, label %bb44.i1128.i, label %bb45.i1056.i, !dbg !7119, !prof !1406

bb45.i1056.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3169.i
  %_116.i1057.i = sub nuw nsw i64 %_39.1, %base.i1047.i, !dbg !7123
  %_120.i1058.i = getelementptr inbounds nuw float, ptr %_39.0, i64 %base.i1047.i, !dbg !7124
  %_8.i3156.i = icmp samesign ugt i64 %_116.i1057.i, 7, !dbg !7129
  br i1 %_8.i3156.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3160.i, label %bb2.i3157.i, !dbg !7129, !prof !1421

bb2.i3157.i:                                      ; preds = %bb45.i1056.i
  store <8 x float> %history.i.i927.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
  store <8 x float> %history.i.i927.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_116.i1057.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !7137, !noalias !7138
  unreachable, !dbg !7137

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3160.i: ; preds = %bb45.i1056.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7142), !dbg !7145
  %width.i.i1060.i = load i64, ptr %113, align 8, !dbg !7146, !alias.scope !7147, !noalias !7148, !noundef !12
  %263 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %262, <8 x float> %251, i8 30), !dbg !7157
  %264 = fdiv <8 x float> %251, %262, !dbg !7163
  %265 = bitcast <8 x float> %263 to <8 x i32>, !dbg !7173
  %266 = icmp slt <8 x i32> %265, zeroinitializer, !dbg !7177
  %267 = select <8 x i1> %266, <8 x float> %264, <8 x float> splat (float 1.000000e+00), !dbg !7177
  %_144.1.i.i1061.i = load i64, ptr %114, align 8, !dbg !7179, !alias.scope !7147, !noalias !7148, !noundef !12
  %_22.i.i1062.i = mul i64 %width.i.i1060.i, %ring_cursor.sroa.0.1.i10427842.i, !dbg !7180
  %_92.i.i1063.i = icmp ugt i64 %_22.i.i1062.i, %_144.1.i.i1061.i, !dbg !7181
  br i1 %_92.i.i1063.i, label %bb37.i.i1127.i, label %bb38.i.i1064.i, !dbg !7181, !prof !1406

bb38.i.i1064.i:                                   ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3160.i
  %_95.i.i1066.i = sub nuw i64 %_144.1.i.i1061.i, %_22.i.i1062.i, !dbg !7186
  %_8.i3557.i = icmp samesign ugt i64 %_95.i.i1066.i, 7, !dbg !7187
  br i1 %_8.i3557.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3560.i, label %bb2.i3558.i, !dbg !7187, !prof !1421

bb2.i3558.i:                                      ; preds = %bb38.i.i1064.i
  store <8 x float> %history.i.i927.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
  store <8 x float> %history.i.i927.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_95.i.i1066.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !7195, !noalias !7196
  unreachable, !dbg !7195

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3560.i: ; preds = %bb38.i.i1064.i
  %_144.0.i.i1065.i = load ptr, ptr %115, align 8, !dbg !7179, !alias.scope !7147, !noalias !7148, !nonnull !12, !noundef !12
  %_99.i.i1067.i = getelementptr inbounds nuw float, ptr %_144.0.i.i1065.i, i64 %_22.i.i1062.i, !dbg !7200
  store <8 x float> %267, ptr %_99.i.i1067.i, align 4, !dbg !7205, !alias.scope !7209, !noalias !7213
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7215), !dbg !7218
  %width.i.i = load i64, ptr %113, align 8, !dbg !7219, !alias.scope !7221, !noalias !7222, !noundef !12
  %268 = icmp eq i64 %width.i.i, 0, !dbg !7224
  br i1 %268, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.i, label %bb32.i1246.lr.ph.i, !dbg !7224

bb32.i1246.lr.ph.i:                               ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3560.i
  %_112.1.i.i = load i64, ptr %116, align 8, !alias.scope !7221, !noalias !7222, !noundef !12
  %_112.0.i.i = load ptr, ptr %117, align 8, !alias.scope !5946, !noalias !5947, !nonnull !12
  %269 = add i64 %ring_cursor.sroa.0.1.i10427842.i, 1
  %_23.not.i.i = icmp ult i64 %269, %_64.i1059.i
  %270 = select i1 %_23.not.i.i, i64 0, i64 %_64.i1059.i
  %start1.sroa.0.0.i.i = sub nuw i64 %269, %270
  %_114.1.i.i = load i64, ptr %114, align 8, !alias.scope !5946, !noalias !5947
  %_114.0.i.i = load ptr, ptr %115, align 8, !alias.scope !5946, !noalias !5947, !nonnull !12
  %_116.1.i.i = load i64, ptr %118, align 8, !alias.scope !5946, !noalias !5947
  %_116.0.i.i = load ptr, ptr %119, align 8, !alias.scope !5946, !noalias !5947, !nonnull !12
  %_118.1.i.i = load i64, ptr %120, align 8, !alias.scope !5946, !noalias !5947
  %_118.0.i.i = load ptr, ptr %121, align 8, !alias.scope !5946, !noalias !5947, !nonnull !12
  %_45.i1259.i = mul i64 %width.i.i, %start1.sroa.0.0.i.i
  br label %bb32.i1246.i, !dbg !7224

bb32.i1246.i:                                     ; preds = %bb31.i.i, %bb32.i1246.lr.ph.i
  %iter.i1245.sroa.10.07834.i = phi i64 [ %width.i.i, %bb32.i1246.lr.ph.i ], [ %271, %bb31.i.i ]
  %iter.i1245.sroa.7.07833.i = phi i64 [ 0, %bb32.i1246.lr.ph.i ], [ %_9.0.i.i, %bb31.i.i ]
  %iter.i1245.sroa.0.0.idx7832.i = phi i64 [ 0, %bb32.i1246.lr.ph.i ], [ %iter.i1245.sroa.0.0.add.i, %bb31.i.i ]
  %iter.i1245.sroa.0.0.ptr7835.i = getelementptr inbounds nuw i8, ptr %scratch.i975.i, i64 %iter.i1245.sroa.0.0.idx7832.i, !dbg !7226
  %271 = add i64 %iter.i1245.sroa.10.07834.i, -1, !dbg !7226
  %_7.i.i.i = icmp eq i64 %iter.i1245.sroa.0.0.idx7832.i, 32, !dbg !7227
  br i1 %_7.i.i.i, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.i, label %bb3.i1248.i, !dbg !7231

bb3.i1248.i:                                      ; preds = %bb32.i1246.i
  %iter.i1245.sroa.0.0.add.i = add nuw nsw i64 %iter.i1245.sroa.0.0.idx7832.i, 4, !dbg !7232
  %_9.0.i.i = add nuw nsw i64 %iter.i1245.sroa.7.07833.i, 1, !dbg !7234
  %exitcond9588.not.i = icmp eq i64 %iter.i1245.sroa.7.07833.i, %_112.1.i.i, !dbg !7235
  br i1 %exitcond9588.not.i, label %panic.i.i, label %bb5.i.i, !dbg !7235

bb5.i.i:                                          ; preds = %bb3.i1248.i
  %272 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i.i, i64 %iter.i1245.sroa.7.07833.i, !dbg !7235
  %shape.i.i = load i32, ptr %272, align 4, !dbg !7235, !noalias !7236, !noundef !12
  %273 = getelementptr inbounds nuw i8, ptr %272, i64 4, !dbg !7235
  %shape3.i.i = load i32, ptr %273, align 4, !dbg !7235, !noalias !7236, !noundef !12
  %window.i.i = zext i32 %shape.i.i to i64, !dbg !7237
  %_19.i.i = zext i32 %shape3.i.i to i64, !dbg !7238
  %274 = add i64 %ring_cursor.sroa.0.1.i10427842.i, %_19.i.i, !dbg !7239
  %_20.not.i.i = icmp ult i64 %274, %_64.i1059.i, !dbg !7240
  %275 = select i1 %_20.not.i.i, i64 0, i64 %_64.i1059.i, !dbg !7240
  %spec.select.i.i = sub nuw i64 %274, %275, !dbg !7240
  %_27.i1250.i = mul i64 %spec.select.i.i, %width.i.i, !dbg !7241
  %_26.i1251.i = add i64 %_27.i1250.i, %iter.i1245.sroa.7.07833.i, !dbg !7241
  %_30.i.i = icmp ult i64 %_26.i1251.i, %_114.1.i.i, !dbg !7242
  br i1 %_30.i.i, label %bb12.i1252.i, label %panic5.i.i, !dbg !7242

panic.i.i:                                        ; preds = %bb3.i1248.i
  store <8 x float> %history.i.i927.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
  store <8 x float> %history.i.i927.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i.i, i64 noundef %_112.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4a8785a681d008a9bfd0cd82628ea9cb) #30, !dbg !7235, !noalias !7236
  unreachable, !dbg !7235

bb12.i1252.i:                                     ; preds = %bb5.i.i
  %276 = getelementptr inbounds nuw float, ptr %_114.0.i.i, i64 %_26.i1251.i, !dbg !7242
  %277 = load float, ptr %276, align 4, !dbg !7242, !noalias !7236, !noundef !12
  %exitcond9589.not.i = icmp eq i64 %iter.i1245.sroa.7.07833.i, %_116.1.i.i, !dbg !7243
  br i1 %exitcond9589.not.i, label %panic6.i.i, label %bb13.i1254.i, !dbg !7243

panic5.i.i:                                       ; preds = %bb5.i.i
  store <8 x float> %history.i.i927.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
  store <8 x float> %history.i.i927.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i1251.i, i64 noundef %_114.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cbce7773ac40979e4ba2385da3aec116) #30, !dbg !7242, !noalias !7236
  unreachable, !dbg !7242

bb13.i1254.i:                                     ; preds = %bb12.i1252.i
  %278 = getelementptr inbounds nuw i32, ptr %_116.0.i.i, i64 %iter.i1245.sroa.7.07833.i, !dbg !7243
  %_32.i.i = load i32, ptr %278, align 4, !dbg !7243, !noalias !7236, !noundef !12
  %position.i.i = zext i32 %_32.i.i to i64, !dbg !7243
  %279 = icmp eq i32 %_32.i.i, 0, !dbg !7244
  br i1 %279, label %bb17.i1257.i, label %bb15.i.i, !dbg !7244

panic6.i.i:                                       ; preds = %bb12.i1252.i
  store <8 x float> %history.i.i927.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
  store <8 x float> %history.i.i927.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i.i, i64 noundef %_116.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ec0d48f73ebfc2755df5cedaa60b5c0a) #30, !dbg !7243, !noalias !7236
  unreachable, !dbg !7243

bb15.i.i:                                         ; preds = %bb13.i1254.i
  %_37.i.i = icmp ult i64 %iter.i1245.sroa.7.07833.i, %_118.1.i.i, !dbg !7245
  br i1 %_37.i.i, label %bb16.i1255.i, label %panic7.i.i, !dbg !7245

bb17.i1257.i:                                     ; preds = %bb35.i.i, %bb16.i1255.i, %bb13.i1254.i
  %newest.sroa.0.0.i.i = phi float [ %277, %bb13.i1254.i ], [ %_35.i.i, %bb35.i.i ], [ %277, %bb16.i1255.i ], !dbg !7246
  %exitcond9590.not.i = icmp eq i64 %iter.i1245.sroa.7.07833.i, %_118.1.i.i, !dbg !7247
  br i1 %exitcond9590.not.i, label %panic8.i.i, label %bb18.i.i, !dbg !7247

bb16.i1255.i:                                     ; preds = %bb15.i.i
  %280 = getelementptr inbounds nuw float, ptr %_118.0.i.i, i64 %iter.i1245.sroa.7.07833.i, !dbg !7245
  %_35.i.i = load float, ptr %280, align 4, !dbg !7245, !noalias !7236, !noundef !12
  %_102.i1256.i = fcmp olt float %_35.i.i, %277, !dbg !7248
  br i1 %_102.i1256.i, label %bb35.i.i, label %bb17.i1257.i, !dbg !7248

panic7.i.i:                                       ; preds = %bb15.i.i
  store <8 x float> %history.i.i927.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
  store <8 x float> %history.i.i927.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i1245.sroa.7.07833.i, i64 noundef %_118.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2c461872bb652d4796cdcf89c28c82c8) #30, !dbg !7245, !noalias !7236
  unreachable, !dbg !7245

bb35.i.i:                                         ; preds = %bb16.i1255.i
  br label %bb17.i1257.i, !dbg !7250

bb18.i.i:                                         ; preds = %bb17.i1257.i
  %281 = getelementptr inbounds nuw float, ptr %_118.0.i.i, i64 %iter.i1245.sroa.7.07833.i, !dbg !7247
  store float %newest.sroa.0.0.i.i, ptr %281, align 4, !dbg !7247, !noalias !7236
  %_42.i.i = add nuw nsw i64 %position.i.i, 1, !dbg !7251
  %complete.i.i = icmp eq i64 %_42.i.i, %window.i.i, !dbg !7251
  br i1 %complete.i.i, label %bb22.i1263.i, label %bb20.i.i, !dbg !7252

panic8.i.i:                                       ; preds = %bb17.i1257.i
  store <8 x float> %history.i.i927.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
  store <8 x float> %history.i.i927.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i.i, i64 noundef %_118.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2b690e2c7763f11809942906fc2ca813) #30, !dbg !7247, !noalias !7236
  unreachable, !dbg !7247

bb20.i.i:                                         ; preds = %bb18.i.i
  %_44.i.i = add i64 %iter.i1245.sroa.7.07833.i, %_45.i1259.i, !dbg !7253
  %_47.i1260.i = icmp ult i64 %_44.i.i, %_114.1.i.i, !dbg !7254
  br i1 %_47.i1260.i, label %bb30.i.i, label %panic9.i.i, !dbg !7254

panic9.i.i:                                       ; preds = %bb20.i.i
  store <8 x float> %history.i.i927.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
  store <8 x float> %history.i.i927.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i.i, i64 noundef %_114.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9fa421ae81817f58fcfc4a3243223891) #30, !dbg !7254, !noalias !7236
  unreachable, !dbg !7254

bb30.i.i:                                         ; preds = %bb20.i.i
  %282 = getelementptr inbounds nuw float, ptr %_114.0.i.i, i64 %_44.i.i, !dbg !7254
  %_43.i.i = load float, ptr %282, align 4, !dbg !7254, !noalias !7236, !noundef !12
  %_103.i.i = fcmp olt float %_43.i.i, %newest.sroa.0.0.i.i, !dbg !7255
  %newest.sroa.0.1.i.i = select i1 %_103.i.i, float %_43.i.i, float %newest.sroa.0.0.i.i, !dbg !7255
  store float %newest.sroa.0.1.i.i, ptr %iter.i1245.sroa.0.0.ptr7835.i, align 4, !dbg !7257, !noalias !7258
  %283 = trunc i64 %_42.i.i to i32, !dbg !7259
  br label %bb31.i.i, !dbg !7260

bb31.i.i:                                         ; preds = %bb25.i1273.i, %bb30.i.i
  %storemerge.i = phi i32 [ %283, %bb30.i.i ], [ 0, %bb25.i1273.i ], !dbg !7261
  store i32 %storemerge.i, ptr %278, align 4, !dbg !7261, !noalias !7236
  %284 = icmp eq i64 %271, 0, !dbg !7224
  br i1 %284, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.i, label %bb32.i1246.i, !dbg !7224

bb22.i1263.i:                                     ; preds = %bb18.i.i
  store float %newest.sroa.0.0.i.i, ptr %iter.i1245.sroa.0.0.ptr7835.i, align 4, !dbg !7257, !noalias !7258
  %285 = load float, ptr %276, align 4, !dbg !7262, !noalias !7236, !noundef !12
  br label %bb41.i.i, !dbg !7263

bb41.i.i:                                         ; preds = %bb25.i1273.i, %bb22.i1263.i
  %iter2.sroa.0.0.i12647831.i = phi i64 [ 0, %bb22.i1263.i ], [ %_105.i1267.i, %bb25.i1273.i ]
  %suffix.sroa.0.0.i7830.i = phi float [ %285, %bb22.i1263.i ], [ %suffix.sroa.0.1.i.i, %bb25.i1273.i ]
  %end.sroa.0.1.i7829.i = phi i64 [ %spec.select.i.i, %bb22.i1263.i ], [ %288, %bb25.i1273.i ]
  %_56.i1268.i = mul i64 %end.sroa.0.1.i7829.i, %width.i.i, !dbg !7266
  %_55.i.i = add i64 %_56.i1268.i, %iter.i1245.sroa.7.07833.i, !dbg !7266
  %_59.i1269.i = icmp ult i64 %_55.i.i, %_114.1.i.i, !dbg !7267
  br i1 %_59.i1269.i, label %bb25.i1273.i, label %panic13.i.i, !dbg !7267

panic13.i.i:                                      ; preds = %bb41.i.i
  store <8 x float> %history.i.i927.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
  store <8 x float> %history.i.i927.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i.i, i64 noundef %_114.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_91a4c6b9b17ebf4d863f9a70b6dc929a) #30, !dbg !7267, !noalias !7236
  unreachable, !dbg !7267

bb25.i1273.i:                                     ; preds = %bb41.i.i
  %_105.i1267.i = add nuw nsw i64 %iter2.sroa.0.0.i12647831.i, 1, !dbg !7268
  %286 = getelementptr inbounds nuw float, ptr %_114.0.i.i, i64 %_55.i.i, !dbg !7267
  %_54.i.i = load float, ptr %286, align 4, !dbg !7267, !noalias !7236, !noundef !12
  %_107.i1271.i = fcmp olt float %suffix.sroa.0.0.i7830.i, %_54.i.i, !dbg !7271
  %suffix.sroa.0.1.i.i = select i1 %_107.i1271.i, float %suffix.sroa.0.0.i7830.i, float %_54.i.i, !dbg !7271
  store float %suffix.sroa.0.1.i.i, ptr %286, align 4, !dbg !7273, !noalias !7236
  %287 = icmp eq i64 %end.sroa.0.1.i7829.i, 0, !dbg !7274
  %spec.store.select.i1274.i = select i1 %287, i64 %_64.i1059.i, i64 %end.sroa.0.1.i7829.i, !dbg !7274
  %288 = add i64 %spec.store.select.i1274.i, -1, !dbg !7275
  %exitcond9587.not.i = icmp eq i64 %_105.i1267.i, %window.i.i, !dbg !7276
  br i1 %exitcond9587.not.i, label %bb31.i.i, label %bb41.i.i, !dbg !7263

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.i: ; preds = %bb31.i.i, %bb32.i1246.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3560.i
  %lanes.i3146.sroa.0.0.copyload.i = load <8 x float>, ptr %scratch.i975.i, align 4, !dbg !7278, !alias.scope !7283, !noalias !7287
  %289 = fmul <8 x float> %lanes.i3146.sroa.0.0.copyload.i, splat (float 1.638400e+04), !dbg !7291
  %290 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %289), !dbg !7296
  %291 = fmul <8 x float> %290, splat (float 0x3F10000000000000), !dbg !7305
  %292 = icmp eq i64 %width.i.i1060.i, 0, !dbg !7310
  %_149.1.i.i1095.pre.i = load i64, ptr %122, align 8, !dbg !7315, !alias.scope !7147, !noalias !7148
  br i1 %292, label %bb16.i.i1094.i, label %bb39.i.i1074.lr.ph.i, !dbg !7310

bb39.i.i1074.lr.ph.i:                             ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.i
  %_145.1.i.i1077.i = load i64, ptr %116, align 8, !alias.scope !7147, !noalias !7148, !noundef !12
  %_145.0.i.i1081.i = load ptr, ptr %117, align 8, !alias.scope !5946, !noalias !5947, !nonnull !12
  %_147.0.i.i1092.i = load ptr, ptr %123, align 8, !alias.scope !5946, !noalias !5947, !nonnull !12
  %exitcond9593.not.i = icmp eq i64 %_145.1.i.i1077.i, 0, !dbg !7316
  br i1 %exitcond9593.not.i, label %panic.i.i1079.i, label %bb17.i.i1080.i, !dbg !7316

bb37.i.i1127.i:                                   ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3160.i
  store <8 x float> %history.i.i927.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
  store <8 x float> %history.i.i927.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i1062.i, i64 noundef %_144.1.i.i1061.i, i64 noundef %_144.1.i.i1061.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_56df7c041d29359441bca272bf4e38e3) #30, !dbg !7318, !noalias !7319
  unreachable, !dbg !7318

bb16.i.i1094.loopexit.i:                          ; preds = %bb21.i.i1091.7.i, %bb21.i.i1091.6.i, %bb21.i.i1091.5.i, %bb21.i.i1091.4.i, %bb21.i.i1091.3.i, %bb21.i.i1091.2.i, %bb21.i.i1091.1.i, %bb21.i.i1091.i
  %lanes.i3139.sroa.0.0.copyload.pre.i = load <8 x float>, ptr %scratch.i975.i, align 4, !dbg !7320, !alias.scope !7325, !noalias !7329
  br label %bb16.i.i1094.i, !dbg !7333

bb16.i.i1094.i:                                   ; preds = %bb16.i.i1094.loopexit.i, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.i
  %lanes.i3139.sroa.0.0.copyload.i = phi <8 x float> [ %lanes.i3139.sroa.0.0.copyload.pre.i, %bb16.i.i1094.loopexit.i ], [ %lanes.i3146.sroa.0.0.copyload.i, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.i ], !dbg !7320
  %293 = fadd <8 x float> %241, %291, !dbg !7334
  %294 = fsub <8 x float> %293, %lanes.i3139.sroa.0.0.copyload.i, !dbg !7339
  %_109.i.i1096.i = icmp ugt i64 %_22.i.i1062.i, %_149.1.i.i1095.pre.i, !dbg !7344
  br i1 %_109.i.i1096.i, label %bb42.i.i1126.i, label %bb43.i.i1097.i, !dbg !7344, !prof !1406

bb43.i.i1097.i:                                   ; preds = %bb16.i.i1094.i
  %_112.i.i1099.i = sub nuw i64 %_149.1.i.i1095.pre.i, %_22.i.i1062.i, !dbg !7348
  %_8.i3552.i = icmp samesign ugt i64 %_112.i.i1099.i, 7, !dbg !7349
  br i1 %_8.i3552.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3555.i, label %bb2.i3553.i, !dbg !7349, !prof !1421

bb2.i3553.i:                                      ; preds = %bb43.i.i1097.i
  store <8 x float> %history.i.i927.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
  store <8 x float> %history.i.i927.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_112.i.i1099.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !7354, !noalias !7355
  unreachable, !dbg !7354

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3555.i: ; preds = %bb43.i.i1097.i
  %_149.0.i.i1098.i = load ptr, ptr %123, align 8, !dbg !7315, !alias.scope !7147, !noalias !7148, !nonnull !12, !noundef !12
  %_116.i.i1100.i = getelementptr inbounds nuw float, ptr %_149.0.i.i1098.i, i64 %_22.i.i1062.i, !dbg !7359
  store <8 x float> %291, ptr %_116.i.i1100.i, align 4, !dbg !7364, !alias.scope !7368, !noalias !7372
  %_68.i.i939.sroa.0.0.copyload.i = load <8 x float>, ptr %126, align 32, !dbg !7374, !noalias !7377
  %295 = fdiv <8 x float> %294, %_64.i.i943.sroa.0.0.copyload.i, !dbg !7378
  %296 = fsub <8 x float> splat (float 1.000000e+00), %295, !dbg !7383
  %297 = fsub <8 x float> %296, %_68.i.i939.sroa.0.0.copyload.i, !dbg !7388
  %298 = fmul <8 x float> %259, %297, !dbg !7393
  %299 = fadd <8 x float> %_68.i.i939.sroa.0.0.copyload.i, %298, !dbg !7401
  %300 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %296, <8 x float> %299), !dbg !7408
  %301 = bitcast <8 x float> %300 to <8 x i32>, !dbg !7414
  %302 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %300), !dbg !7421
  %303 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %302, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !7423
  %304 = bitcast <8 x float> %303 to <8 x i32>, !dbg !7435
  %305 = xor <8 x i32> %304, splat (i32 -1), !dbg !7447
  %306 = and <8 x i32> %305, %301, !dbg !7449
  %307 = bitcast <8 x i32> %306 to <8 x float>, !dbg !7455
  store <8 x i32> %306, ptr %126, align 32, !dbg !7456, !noalias !7377
  %308 = fsub <8 x float> splat (float 1.000000e+00), %307, !dbg !7457
  %_150.1.i.i1101.i = load i64, ptr %127, align 8, !dbg !7462, !alias.scope !7147, !noalias !7148, !noundef !12
  %_76.i.i1102.i = mul i64 %width.i.i1060.i, %main_cursor.sroa.0.1.i10437843.i, !dbg !7464
  %_120.i.i1103.i = icmp ugt i64 %_76.i.i1102.i, %_150.1.i.i1101.i, !dbg !7465
  br i1 %_120.i.i1103.i, label %bb48.i.i1125.i, label %bb49.i.i1104.i, !dbg !7465, !prof !1406

bb42.i.i1126.i:                                   ; preds = %bb16.i.i1094.i
  store <8 x float> %history.i.i927.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
  store <8 x float> %history.i.i927.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i1062.i, i64 noundef %_149.1.i.i1095.pre.i, i64 noundef %_149.1.i.i1095.pre.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_90498045d73339daaf9e4f537508f58b) #30, !dbg !7470, !noalias !7471
  unreachable, !dbg !7470

bb49.i.i1104.i:                                   ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3555.i
  %_123.i.i1106.i = sub nuw i64 %_150.1.i.i1101.i, %_76.i.i1102.i, !dbg !7472
  %_8.i3133.i = icmp samesign ugt i64 %_123.i.i1106.i, 7, !dbg !7473
  br i1 %_8.i3133.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i, label %bb2.i3134.i, !dbg !7473, !prof !1421

bb2.i3134.i:                                      ; preds = %bb49.i.i1104.i
  store <8 x float> %history.i.i927.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
  store <8 x float> %history.i.i927.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_123.i.i1106.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !7478, !noalias !7479
  unreachable, !dbg !7478

bb48.i.i1125.i:                                   ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3555.i
  store <8 x float> %history.i.i927.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
  store <8 x float> %history.i.i927.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i.i1102.i, i64 noundef %_150.1.i.i1101.i, i64 noundef %_150.1.i.i1101.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_da8af254b6d507a8e2ca31e544bfd21d) #30, !dbg !7483, !noalias !7471
  unreachable, !dbg !7483

bb17.i.i1080.i:                                   ; preds = %bb39.i.i1074.lr.ph.i
  %309 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1081.i, i64 8, !dbg !7316
  %_44.i.i1082.i = load i32, ptr %309, align 4, !dbg !7316, !noalias !7471, !noundef !12
  %_43.i.i1083.i = zext i32 %_44.i.i1082.i to i64, !dbg !7316
  %310 = add i64 %ring_cursor.sroa.0.1.i10427842.i, %_43.i.i1083.i, !dbg !7484
  %_47.not.i.i1084.i = icmp ult i64 %310, %_64.i1059.i, !dbg !7485
  %311 = select i1 %_47.not.i.i1084.i, i64 0, i64 %_64.i1059.i, !dbg !7485
  %spec.select.i.i1085.i = sub nuw i64 %310, %311, !dbg !7485
  %_51.i.i1086.i = mul i64 %spec.select.i.i1085.i, %width.i.i1060.i, !dbg !7487
  %_53.i.i1089.i = icmp ult i64 %_51.i.i1086.i, %_149.1.i.i1095.pre.i, !dbg !7488
  br i1 %_53.i.i1089.i, label %bb21.i.i1091.i, label %panic1.i.i1090.i, !dbg !7488

panic.i.i1079.i:                                  ; preds = %bb39.i.i1074.7.i, %bb39.i.i1074.6.i, %bb39.i.i1074.5.i, %bb39.i.i1074.4.i, %bb39.i.i1074.3.i, %bb39.i.i1074.2.i, %bb39.i.i1074.1.i, %bb39.i.i1074.lr.ph.i
  %_145.1.i.i1077.lcssa.ph.i = phi i64 [ 7, %bb39.i.i1074.7.i ], [ 6, %bb39.i.i1074.6.i ], [ 5, %bb39.i.i1074.5.i ], [ 4, %bb39.i.i1074.4.i ], [ 3, %bb39.i.i1074.3.i ], [ 2, %bb39.i.i1074.2.i ], [ 1, %bb39.i.i1074.1.i ], [ 0, %bb39.i.i1074.lr.ph.i ]
  store <8 x float> %history.i.i927.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
  store <8 x float> %history.i.i927.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i.i1077.lcssa.ph.i, i64 noundef %_145.1.i.i1077.lcssa.ph.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a6feb40b34112df5f214f84424dd2c7c) #30, !dbg !7316, !noalias !7471
  unreachable, !dbg !7316

bb21.i.i1091.i:                                   ; preds = %bb17.i.i1080.i
  %312 = getelementptr inbounds nuw float, ptr %_147.0.i.i1092.i, i64 %_51.i.i1086.i, !dbg !7488
  %_49.i.i1093.i = load float, ptr %312, align 4, !dbg !7488, !noalias !7471, !noundef !12
  store float %_49.i.i1093.i, ptr %scratch.i975.i, align 4, !dbg !7489, !noalias !7490
  %313 = icmp eq i64 %width.i.i1060.i, 1, !dbg !7310
  br i1 %313, label %bb16.i.i1094.loopexit.i, label %bb39.i.i1074.1.i, !dbg !7310

bb39.i.i1074.1.i:                                 ; preds = %bb21.i.i1091.i
  %exitcond9593.1.not.i = icmp eq i64 %_145.1.i.i1077.i, 1, !dbg !7316
  br i1 %exitcond9593.1.not.i, label %panic.i.i1079.i, label %bb17.i.i1080.1.i, !dbg !7316

bb17.i.i1080.1.i:                                 ; preds = %bb39.i.i1074.1.i
  %314 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1081.i, i64 20, !dbg !7316
  %_44.i.i1082.1.i = load i32, ptr %314, align 4, !dbg !7316, !noalias !7471, !noundef !12
  %_43.i.i1083.1.i = zext i32 %_44.i.i1082.1.i to i64, !dbg !7316
  %315 = add i64 %ring_cursor.sroa.0.1.i10427842.i, %_43.i.i1083.1.i, !dbg !7484
  %_47.not.i.i1084.1.i = icmp ult i64 %315, %_64.i1059.i, !dbg !7485
  %316 = select i1 %_47.not.i.i1084.1.i, i64 0, i64 %_64.i1059.i, !dbg !7485
  %spec.select.i.i1085.1.i = sub nuw i64 %315, %316, !dbg !7485
  %_51.i.i1086.1.i = mul i64 %spec.select.i.i1085.1.i, %width.i.i1060.i, !dbg !7487
  %_50.i.i1087.1.i = add i64 %_51.i.i1086.1.i, 1, !dbg !7487
  %_53.i.i1089.1.i = icmp ult i64 %_50.i.i1087.1.i, %_149.1.i.i1095.pre.i, !dbg !7488
  br i1 %_53.i.i1089.1.i, label %bb21.i.i1091.1.i, label %panic1.i.i1090.i, !dbg !7488

bb21.i.i1091.1.i:                                 ; preds = %bb17.i.i1080.1.i
  %317 = getelementptr inbounds nuw float, ptr %_147.0.i.i1092.i, i64 %_50.i.i1087.1.i, !dbg !7488
  %_49.i.i1093.1.i = load float, ptr %317, align 4, !dbg !7488, !noalias !7471, !noundef !12
  store float %_49.i.i1093.1.i, ptr %iter.i.i949.sroa.0.0.ptr7839.1.i, align 4, !dbg !7489, !noalias !7490
  %318 = icmp eq i64 %width.i.i1060.i, 2, !dbg !7310
  br i1 %318, label %bb16.i.i1094.loopexit.i, label %bb39.i.i1074.2.i, !dbg !7310

bb39.i.i1074.2.i:                                 ; preds = %bb21.i.i1091.1.i
  %exitcond9593.2.not.i = icmp eq i64 %_145.1.i.i1077.i, 2, !dbg !7316
  br i1 %exitcond9593.2.not.i, label %panic.i.i1079.i, label %bb17.i.i1080.2.i, !dbg !7316

bb17.i.i1080.2.i:                                 ; preds = %bb39.i.i1074.2.i
  %319 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1081.i, i64 32, !dbg !7316
  %_44.i.i1082.2.i = load i32, ptr %319, align 4, !dbg !7316, !noalias !7471, !noundef !12
  %_43.i.i1083.2.i = zext i32 %_44.i.i1082.2.i to i64, !dbg !7316
  %320 = add i64 %ring_cursor.sroa.0.1.i10427842.i, %_43.i.i1083.2.i, !dbg !7484
  %_47.not.i.i1084.2.i = icmp ult i64 %320, %_64.i1059.i, !dbg !7485
  %321 = select i1 %_47.not.i.i1084.2.i, i64 0, i64 %_64.i1059.i, !dbg !7485
  %spec.select.i.i1085.2.i = sub nuw i64 %320, %321, !dbg !7485
  %_51.i.i1086.2.i = mul i64 %spec.select.i.i1085.2.i, %width.i.i1060.i, !dbg !7487
  %_50.i.i1087.2.i = add i64 %_51.i.i1086.2.i, 2, !dbg !7487
  %_53.i.i1089.2.i = icmp ult i64 %_50.i.i1087.2.i, %_149.1.i.i1095.pre.i, !dbg !7488
  br i1 %_53.i.i1089.2.i, label %bb21.i.i1091.2.i, label %panic1.i.i1090.i, !dbg !7488

bb21.i.i1091.2.i:                                 ; preds = %bb17.i.i1080.2.i
  %322 = getelementptr inbounds nuw float, ptr %_147.0.i.i1092.i, i64 %_50.i.i1087.2.i, !dbg !7488
  %_49.i.i1093.2.i = load float, ptr %322, align 4, !dbg !7488, !noalias !7471, !noundef !12
  store float %_49.i.i1093.2.i, ptr %iter.i.i949.sroa.0.0.ptr7839.2.i, align 4, !dbg !7489, !noalias !7490
  %323 = icmp eq i64 %width.i.i1060.i, 3, !dbg !7310
  br i1 %323, label %bb16.i.i1094.loopexit.i, label %bb39.i.i1074.3.i, !dbg !7310

bb39.i.i1074.3.i:                                 ; preds = %bb21.i.i1091.2.i
  %exitcond9593.3.not.i = icmp eq i64 %_145.1.i.i1077.i, 3, !dbg !7316
  br i1 %exitcond9593.3.not.i, label %panic.i.i1079.i, label %bb17.i.i1080.3.i, !dbg !7316

bb17.i.i1080.3.i:                                 ; preds = %bb39.i.i1074.3.i
  %324 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1081.i, i64 44, !dbg !7316
  %_44.i.i1082.3.i = load i32, ptr %324, align 4, !dbg !7316, !noalias !7471, !noundef !12
  %_43.i.i1083.3.i = zext i32 %_44.i.i1082.3.i to i64, !dbg !7316
  %325 = add i64 %ring_cursor.sroa.0.1.i10427842.i, %_43.i.i1083.3.i, !dbg !7484
  %_47.not.i.i1084.3.i = icmp ult i64 %325, %_64.i1059.i, !dbg !7485
  %326 = select i1 %_47.not.i.i1084.3.i, i64 0, i64 %_64.i1059.i, !dbg !7485
  %spec.select.i.i1085.3.i = sub nuw i64 %325, %326, !dbg !7485
  %_51.i.i1086.3.i = mul i64 %spec.select.i.i1085.3.i, %width.i.i1060.i, !dbg !7487
  %_50.i.i1087.3.i = add i64 %_51.i.i1086.3.i, 3, !dbg !7487
  %_53.i.i1089.3.i = icmp ult i64 %_50.i.i1087.3.i, %_149.1.i.i1095.pre.i, !dbg !7488
  br i1 %_53.i.i1089.3.i, label %bb21.i.i1091.3.i, label %panic1.i.i1090.i, !dbg !7488

bb21.i.i1091.3.i:                                 ; preds = %bb17.i.i1080.3.i
  %327 = getelementptr inbounds nuw float, ptr %_147.0.i.i1092.i, i64 %_50.i.i1087.3.i, !dbg !7488
  %_49.i.i1093.3.i = load float, ptr %327, align 4, !dbg !7488, !noalias !7471, !noundef !12
  store float %_49.i.i1093.3.i, ptr %iter.i.i949.sroa.0.0.ptr7839.3.i, align 4, !dbg !7489, !noalias !7490
  %328 = icmp eq i64 %width.i.i1060.i, 4, !dbg !7310
  br i1 %328, label %bb16.i.i1094.loopexit.i, label %bb39.i.i1074.4.i, !dbg !7310

bb39.i.i1074.4.i:                                 ; preds = %bb21.i.i1091.3.i
  %exitcond9593.4.not.i = icmp eq i64 %_145.1.i.i1077.i, 4, !dbg !7316
  br i1 %exitcond9593.4.not.i, label %panic.i.i1079.i, label %bb17.i.i1080.4.i, !dbg !7316

bb17.i.i1080.4.i:                                 ; preds = %bb39.i.i1074.4.i
  %329 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1081.i, i64 56, !dbg !7316
  %_44.i.i1082.4.i = load i32, ptr %329, align 4, !dbg !7316, !noalias !7471, !noundef !12
  %_43.i.i1083.4.i = zext i32 %_44.i.i1082.4.i to i64, !dbg !7316
  %330 = add i64 %ring_cursor.sroa.0.1.i10427842.i, %_43.i.i1083.4.i, !dbg !7484
  %_47.not.i.i1084.4.i = icmp ult i64 %330, %_64.i1059.i, !dbg !7485
  %331 = select i1 %_47.not.i.i1084.4.i, i64 0, i64 %_64.i1059.i, !dbg !7485
  %spec.select.i.i1085.4.i = sub nuw i64 %330, %331, !dbg !7485
  %_51.i.i1086.4.i = mul i64 %spec.select.i.i1085.4.i, %width.i.i1060.i, !dbg !7487
  %_50.i.i1087.4.i = add i64 %_51.i.i1086.4.i, 4, !dbg !7487
  %_53.i.i1089.4.i = icmp ult i64 %_50.i.i1087.4.i, %_149.1.i.i1095.pre.i, !dbg !7488
  br i1 %_53.i.i1089.4.i, label %bb21.i.i1091.4.i, label %panic1.i.i1090.i, !dbg !7488

bb21.i.i1091.4.i:                                 ; preds = %bb17.i.i1080.4.i
  %332 = getelementptr inbounds nuw float, ptr %_147.0.i.i1092.i, i64 %_50.i.i1087.4.i, !dbg !7488
  %_49.i.i1093.4.i = load float, ptr %332, align 4, !dbg !7488, !noalias !7471, !noundef !12
  store float %_49.i.i1093.4.i, ptr %iter.i.i949.sroa.0.0.ptr7839.4.i, align 4, !dbg !7489, !noalias !7490
  %333 = icmp eq i64 %width.i.i1060.i, 5, !dbg !7310
  br i1 %333, label %bb16.i.i1094.loopexit.i, label %bb39.i.i1074.5.i, !dbg !7310

bb39.i.i1074.5.i:                                 ; preds = %bb21.i.i1091.4.i
  %exitcond9593.5.not.i = icmp eq i64 %_145.1.i.i1077.i, 5, !dbg !7316
  br i1 %exitcond9593.5.not.i, label %panic.i.i1079.i, label %bb17.i.i1080.5.i, !dbg !7316

bb17.i.i1080.5.i:                                 ; preds = %bb39.i.i1074.5.i
  %334 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1081.i, i64 68, !dbg !7316
  %_44.i.i1082.5.i = load i32, ptr %334, align 4, !dbg !7316, !noalias !7471, !noundef !12
  %_43.i.i1083.5.i = zext i32 %_44.i.i1082.5.i to i64, !dbg !7316
  %335 = add i64 %ring_cursor.sroa.0.1.i10427842.i, %_43.i.i1083.5.i, !dbg !7484
  %_47.not.i.i1084.5.i = icmp ult i64 %335, %_64.i1059.i, !dbg !7485
  %336 = select i1 %_47.not.i.i1084.5.i, i64 0, i64 %_64.i1059.i, !dbg !7485
  %spec.select.i.i1085.5.i = sub nuw i64 %335, %336, !dbg !7485
  %_51.i.i1086.5.i = mul i64 %spec.select.i.i1085.5.i, %width.i.i1060.i, !dbg !7487
  %_50.i.i1087.5.i = add i64 %_51.i.i1086.5.i, 5, !dbg !7487
  %_53.i.i1089.5.i = icmp ult i64 %_50.i.i1087.5.i, %_149.1.i.i1095.pre.i, !dbg !7488
  br i1 %_53.i.i1089.5.i, label %bb21.i.i1091.5.i, label %panic1.i.i1090.i, !dbg !7488

bb21.i.i1091.5.i:                                 ; preds = %bb17.i.i1080.5.i
  %337 = getelementptr inbounds nuw float, ptr %_147.0.i.i1092.i, i64 %_50.i.i1087.5.i, !dbg !7488
  %_49.i.i1093.5.i = load float, ptr %337, align 4, !dbg !7488, !noalias !7471, !noundef !12
  store float %_49.i.i1093.5.i, ptr %iter.i.i949.sroa.0.0.ptr7839.5.i, align 4, !dbg !7489, !noalias !7490
  %338 = icmp eq i64 %width.i.i1060.i, 6, !dbg !7310
  br i1 %338, label %bb16.i.i1094.loopexit.i, label %bb39.i.i1074.6.i, !dbg !7310

bb39.i.i1074.6.i:                                 ; preds = %bb21.i.i1091.5.i
  %exitcond9593.6.not.i = icmp eq i64 %_145.1.i.i1077.i, 6, !dbg !7316
  br i1 %exitcond9593.6.not.i, label %panic.i.i1079.i, label %bb17.i.i1080.6.i, !dbg !7316

bb17.i.i1080.6.i:                                 ; preds = %bb39.i.i1074.6.i
  %339 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1081.i, i64 80, !dbg !7316
  %_44.i.i1082.6.i = load i32, ptr %339, align 4, !dbg !7316, !noalias !7471, !noundef !12
  %_43.i.i1083.6.i = zext i32 %_44.i.i1082.6.i to i64, !dbg !7316
  %340 = add i64 %ring_cursor.sroa.0.1.i10427842.i, %_43.i.i1083.6.i, !dbg !7484
  %_47.not.i.i1084.6.i = icmp ult i64 %340, %_64.i1059.i, !dbg !7485
  %341 = select i1 %_47.not.i.i1084.6.i, i64 0, i64 %_64.i1059.i, !dbg !7485
  %spec.select.i.i1085.6.i = sub nuw i64 %340, %341, !dbg !7485
  %_51.i.i1086.6.i = mul i64 %spec.select.i.i1085.6.i, %width.i.i1060.i, !dbg !7487
  %_50.i.i1087.6.i = add i64 %_51.i.i1086.6.i, 6, !dbg !7487
  %_53.i.i1089.6.i = icmp ult i64 %_50.i.i1087.6.i, %_149.1.i.i1095.pre.i, !dbg !7488
  br i1 %_53.i.i1089.6.i, label %bb21.i.i1091.6.i, label %panic1.i.i1090.i, !dbg !7488

bb21.i.i1091.6.i:                                 ; preds = %bb17.i.i1080.6.i
  %342 = getelementptr inbounds nuw float, ptr %_147.0.i.i1092.i, i64 %_50.i.i1087.6.i, !dbg !7488
  %_49.i.i1093.6.i = load float, ptr %342, align 4, !dbg !7488, !noalias !7471, !noundef !12
  store float %_49.i.i1093.6.i, ptr %iter.i.i949.sroa.0.0.ptr7839.6.i, align 4, !dbg !7489, !noalias !7490
  %343 = icmp eq i64 %width.i.i1060.i, 7, !dbg !7310
  br i1 %343, label %bb16.i.i1094.loopexit.i, label %bb39.i.i1074.7.i, !dbg !7310

bb39.i.i1074.7.i:                                 ; preds = %bb21.i.i1091.6.i
  %exitcond9593.7.not.i = icmp eq i64 %_145.1.i.i1077.i, 7, !dbg !7316
  br i1 %exitcond9593.7.not.i, label %panic.i.i1079.i, label %bb17.i.i1080.7.i, !dbg !7316

bb17.i.i1080.7.i:                                 ; preds = %bb39.i.i1074.7.i
  %344 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1081.i, i64 92, !dbg !7316
  %_44.i.i1082.7.i = load i32, ptr %344, align 4, !dbg !7316, !noalias !7471, !noundef !12
  %_43.i.i1083.7.i = zext i32 %_44.i.i1082.7.i to i64, !dbg !7316
  %345 = add i64 %ring_cursor.sroa.0.1.i10427842.i, %_43.i.i1083.7.i, !dbg !7484
  %_47.not.i.i1084.7.i = icmp ult i64 %345, %_64.i1059.i, !dbg !7485
  %346 = select i1 %_47.not.i.i1084.7.i, i64 0, i64 %_64.i1059.i, !dbg !7485
  %spec.select.i.i1085.7.i = sub nuw i64 %345, %346, !dbg !7485
  %_51.i.i1086.7.i = mul i64 %spec.select.i.i1085.7.i, %width.i.i1060.i, !dbg !7487
  %_50.i.i1087.7.i = add i64 %_51.i.i1086.7.i, 7, !dbg !7487
  %_53.i.i1089.7.i = icmp ult i64 %_50.i.i1087.7.i, %_149.1.i.i1095.pre.i, !dbg !7488
  br i1 %_53.i.i1089.7.i, label %bb21.i.i1091.7.i, label %panic1.i.i1090.i, !dbg !7488

bb21.i.i1091.7.i:                                 ; preds = %bb17.i.i1080.7.i
  %347 = getelementptr inbounds nuw float, ptr %_147.0.i.i1092.i, i64 %_50.i.i1087.7.i, !dbg !7488
  %_49.i.i1093.7.i = load float, ptr %347, align 4, !dbg !7488, !noalias !7471, !noundef !12
  store float %_49.i.i1093.7.i, ptr %iter.i.i949.sroa.0.0.ptr7839.7.i, align 4, !dbg !7489, !noalias !7490
  br label %bb16.i.i1094.loopexit.i, !dbg !7310

panic1.i.i1090.i:                                 ; preds = %bb17.i.i1080.7.i, %bb17.i.i1080.6.i, %bb17.i.i1080.5.i, %bb17.i.i1080.4.i, %bb17.i.i1080.3.i, %bb17.i.i1080.2.i, %bb17.i.i1080.1.i, %bb17.i.i1080.i
  %_50.i.i1087.lcssa.ph.i = phi i64 [ %_50.i.i1087.7.i, %bb17.i.i1080.7.i ], [ %_50.i.i1087.6.i, %bb17.i.i1080.6.i ], [ %_50.i.i1087.5.i, %bb17.i.i1080.5.i ], [ %_50.i.i1087.4.i, %bb17.i.i1080.4.i ], [ %_50.i.i1087.3.i, %bb17.i.i1080.3.i ], [ %_50.i.i1087.2.i, %bb17.i.i1080.2.i ], [ %_50.i.i1087.1.i, %bb17.i.i1080.1.i ], [ %_51.i.i1086.i, %bb17.i.i1080.i ]
  store <8 x float> %history.i.i927.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
  store <8 x float> %history.i.i927.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i.i1087.lcssa.ph.i, i64 noundef %_149.1.i.i1095.pre.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b305c1483509cfb31fdec21ff8752674) #30, !dbg !7488, !noalias !7471
  unreachable, !dbg !7488

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i: ; preds = %bb49.i.i1104.i
  %_150.0.i.i1105.i = load ptr, ptr %128, align 8, !dbg !7462, !alias.scope !7147, !noalias !7148, !nonnull !12, !noundef !12
  %_127.i.i1107.i = getelementptr inbounds nuw float, ptr %_150.0.i.i1105.i, i64 %_76.i.i1102.i, !dbg !7491
  %lanes.i3130.sroa.0.0.copyload.i = load <8 x float>, ptr %_127.i.i1107.i, align 4, !dbg !7496, !alias.scope !7500, !noalias !7504
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_127.i.i1107.i, ptr noundef nonnull align 4 dereferenceable(32) %_120.i1058.i, i64 32, i1 false), !dbg !7506
  %348 = fmul <8 x float> %308, %lanes.i3130.sroa.0.0.copyload.i, !dbg !7512
  %349 = select <8 x i1> %130, <8 x float> %lanes.i3130.sroa.0.0.copyload.i, <8 x float> %348, !dbg !7517
  store <8 x float> %349, ptr %_120.i1058.i, align 4, !dbg !7522, !alias.scope !7527, !noalias !7531
  %350 = add i64 %main_cursor.sroa.0.1.i10437843.i, 1, !dbg !7535
  %_69.i1119.i = icmp eq i64 %350, %_71.i1118.i, !dbg !7536
  %spec.store.select.i1120.i = select i1 %_69.i1119.i, i64 0, i64 %350, !dbg !7536
  %351 = add i64 %ring_cursor.sroa.0.1.i10427842.i, 1, !dbg !7537
  %_72.i1121.i = icmp eq i64 %351, %_64.i1059.i, !dbg !7538
  %spec.store.select9.i1122.i = select i1 %_72.i1121.i, i64 0, i64 %351, !dbg !7538
  %exitcond9598.not.i = icmp eq i64 %244, %umax9597.i, !dbg !7539
  br i1 %exitcond9598.not.i, label %bb14.i1040.bb12.i995.loopexit_crit_edge.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3169.i, !dbg !6337

bb44.i1128.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3169.i
  store <8 x float> %history.i.i927.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
  store <8 x float> %history.i.i927.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1047.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1, i64 noundef range(i64 0, 2305843009213693952) %_39.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_754b353a9c273ca837480913d0661c80) #30, !dbg !7542, !noalias !6370
  unreachable, !dbg !7542

_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i: ; preds = %bb12.i995.loopexit.i
  store <8 x float> %history.i.i927.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.10.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
  store <8 x float> %history.i.i927.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i927.sroa.13.0.hot_left.i981.sroa_idx.i, align 1, !dbg !6364
  store <8 x float> %history.i.i927.sroa.0.0.lcssa.i, ptr %hot_left.i981.i, align 32, !dbg !6996, !noalias !6294
  %352 = trunc i64 %main_cursor.sroa.0.1.i1043.lcssa.i to i32, !dbg !7543
  %353 = trunc i64 %ring_cursor.sroa.0.1.i1042.lcssa.i to i32, !dbg !7544
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, !dbg !7545

_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i, %bb8.i.i
  %ring_cursor.sroa.0.0.i998.lcssa.i = phi i32 [ %_26.i989.i, %bb8.i.i ], [ %353, %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i ], !dbg !6231
  %main_cursor.sroa.0.0.i999.lcssa.i = phi i32 [ %_24.i988.i, %bb8.i.i ], [ %352, %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i ], !dbg !6227
  call void @llvm.lifetime.start.p0(ptr nonnull %_75.i963.i), !dbg !7545, !noalias !6214
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(736) %_75.i963.i, ptr noundef nonnull align 32 dereferenceable(736) %hot_left.i981.i, i64 736, i1 false), !dbg !7545, !noalias !6214
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %_75.i963.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25) #31, !dbg !7546, !noalias !6370
  call void @llvm.lifetime.end.p0(ptr nonnull %_75.i963.i), !dbg !7547, !noalias !6214
  store i32 %main_cursor.sroa.0.0.i999.lcssa.i, ptr %_25.i, align 4, !dbg !7543, !alias.scope !6229, !noalias !6230
  store i32 %ring_cursor.sroa.0.0.i998.lcssa.i, ptr %61, align 4, !dbg !7544, !alias.scope !6229, !noalias !6230
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i974.i), !dbg !7548, !noalias !6214
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i975.i), !dbg !7549, !noalias !6214
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i981.i), !dbg !7550, !noalias !6214
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter18limiter_block_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, !dbg !6205

bb4.i.i:                                          ; preds = %bb1.i3.i.i, %bb2.i.i, %bb2.i3815.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7551), !dbg !7554
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7555), !dbg !7554
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7557), !dbg !7554
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7559), !dbg !7554
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i270.i), !dbg !7561, !noalias !7565
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_left.i270.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_25) #31, !dbg !7567, !noalias !7568
  %354 = getelementptr inbounds nuw i8, ptr %self, i64 1536, !dbg !7569
  %355 = load i8, ptr %354, align 32, !dbg !7569, !range !17, !alias.scope !7573, !noalias !7574, !noundef !12
  %356 = getelementptr inbounds nuw i8, ptr %self, i64 1537, !dbg !7575
  %357 = load i8, ptr %356, align 1, !dbg !7575, !range !17, !alias.scope !7573, !noalias !7574, !noundef !12
  %358 = getelementptr inbounds nuw i8, ptr %self, i64 1624, !dbg !7577
  %ring.i277.i = load i64, ptr %358, align 8, !dbg !7577, !alias.scope !7579, !noalias !7580, !noundef !12
  %359 = getelementptr inbounds nuw i8, ptr %self, i64 1632, !dbg !7581
  %main.i278.i = load i64, ptr %359, align 8, !dbg !7581, !alias.scope !7579, !noalias !7580, !noundef !12
  %_25.i279.i = load i32, ptr %_25.i, align 4, !dbg !7583, !alias.scope !7585, !noalias !7586, !noundef !12
  %360 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !7587
  %_26.i280.i = load i32, ptr %360, align 4, !dbg !7587, !alias.scope !7585, !noalias !7586, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i264.i), !dbg !7589, !noalias !7565
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i264.i, i8 0, i64 1024, i1 false), !noalias !7565
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i263.i), !dbg !7591, !noalias !7565
; call <true_peak_limiter::UniformHot<wide::f32x8_::f32x8>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_(ptr noalias noundef align 32 captures(none) dereferenceable(128) %uniform_left.i263.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25, i64 %ring.i277.i, i64 %main.i278.i) #31, !dbg !7593, !noalias !5947
  %361 = add nuw nsw i64 %_31, 31, !dbg !7594
  %yield_count.sroa.0.0.i.i3912.i = lshr i64 %361, 5, !dbg !7594
  %_111.not.i2898400.i = icmp eq i64 %yield_count.sroa.0.0.i.i3912.i, 0, !dbg !7601
  br i1 %_111.not.i2898400.i, label %bb4.i.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i_crit_edge, label %bb36.i290.lr.ph.i, !dbg !7601

bb4.i.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i_crit_edge: ; preds = %bb4.i.i
  %.phi.trans.insert = getelementptr inbounds nuw i8, ptr %uniform_left.i263.i, i64 104
  %left_phase.i444.i.pre = load i32, ptr %.phi.trans.insert, align 8, !dbg !7610, !noalias !7565
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, !dbg !7601

bb36.i290.lr.ph.i:                                ; preds = %bb4.i.i
  %362 = zext i32 %_26.i280.i to i64, !dbg !7587
  %363 = zext i32 %_25.i279.i to i64, !dbg !7583
  %_22.i274.i = trunc nuw i8 %357 to i1, !dbg !7575
  %_21.i271.i = trunc nuw i8 %355 to i1, !dbg !7569
  %364 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !7611
  %365 = bitcast <8 x float> %364 to <8 x i32>, !dbg !7617
  %366 = xor <8 x i32> %365, splat (i32 -1), !dbg !7623
  %history.i.i239.sroa.10.0.hot_left.i270.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i270.i, i64 32
  %history.i.i239.sroa.13.0.hot_left.i270.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i270.i, i64 64
  %history.i.i239.sroa.16.0.hot_left.i270.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i270.i, i64 96
  %history.i.i239.sroa.19.0.hot_left.i270.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i270.i, i64 128
  %history.i.i239.sroa.22.0.hot_left.i270.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i270.i, i64 160
  %history.i.i239.sroa.25.0.hot_left.i270.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i270.i, i64 192
  %history.i.i239.sroa.29.0.hot_left.i270.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i270.i, i64 224
  %history.i.i239.sroa.32.0.hot_left.i270.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i270.i, i64 256
  %history.i.i239.sroa.35.0.hot_left.i270.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i270.i, i64 288
  %history.i.i239.sroa.38.0.hot_left.i270.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i270.i, i64 320
  %history.i.i239.sroa.41.0.hot_left.i270.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i270.i, i64 352
  %367 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %368 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %369 = getelementptr inbounds nuw i8, ptr %self, i64 96
  %row12.i.i.i.i311.i = getelementptr inbounds nuw i8, ptr %self, i64 128
  %370 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %371 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %372 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %row13.i.i.i.i312.i = getelementptr inbounds nuw i8, ptr %self, i64 256
  %373 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %374 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %375 = getelementptr inbounds nuw i8, ptr %self, i64 352
  %row14.i.i.i.i313.i = getelementptr inbounds nuw i8, ptr %self, i64 384
  %376 = getelementptr inbounds nuw i8, ptr %self, i64 416
  %377 = getelementptr inbounds nuw i8, ptr %self, i64 448
  %378 = getelementptr inbounds nuw i8, ptr %self, i64 480
  %row15.i.i.i.i314.i = getelementptr inbounds nuw i8, ptr %self, i64 512
  %379 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %380 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %381 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %row16.i.i.i.i315.i = getelementptr inbounds nuw i8, ptr %self, i64 640
  %382 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %383 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %384 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %row17.i.i.i.i316.i = getelementptr inbounds nuw i8, ptr %self, i64 768
  %385 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %386 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %387 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %row18.i.i.i.i317.i = getelementptr inbounds nuw i8, ptr %self, i64 896
  %388 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %389 = getelementptr inbounds nuw i8, ptr %self, i64 960
  %390 = getelementptr inbounds nuw i8, ptr %self, i64 992
  %row19.i.i.i.i318.i = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %391 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %392 = getelementptr inbounds nuw i8, ptr %self, i64 1088
  %393 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %row20.i.i.i.i319.i = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %394 = getelementptr inbounds nuw i8, ptr %self, i64 1184
  %395 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %396 = getelementptr inbounds nuw i8, ptr %self, i64 1248
  %row21.i.i.i.i320.i = getelementptr inbounds nuw i8, ptr %self, i64 1280
  %397 = getelementptr inbounds nuw i8, ptr %self, i64 1312
  %398 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %399 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %row22.i.i.i.i321.i = getelementptr inbounds nuw i8, ptr %self, i64 1408
  %400 = getelementptr inbounds nuw i8, ptr %self, i64 1440
  %401 = getelementptr inbounds nuw i8, ptr %self, i64 1472
  %402 = getelementptr inbounds nuw i8, ptr %self, i64 1504
  %403 = getelementptr inbounds nuw i8, ptr %uniform_left.i263.i, i64 80
  %_52.i260.sroa.3.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %uniform_left.i263.i, i64 88
  %_52.i260.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %uniform_left.i263.i, i64 96
  %_82.i364.i = getelementptr inbounds nuw i8, ptr %hot_left.i270.i, i64 384
  %_83.i365.i = getelementptr inbounds nuw i8, ptr %hot_left.i270.i, i64 512
  %404 = getelementptr inbounds nuw i8, ptr %hot_left.i270.i, i64 480
  %405 = getelementptr inbounds nuw i8, ptr %hot_left.i270.i, i64 448
  %406 = getelementptr inbounds nuw i8, ptr %hot_left.i270.i, i64 416
  %407 = getelementptr inbounds nuw i8, ptr %hot_left.i270.i, i64 608
  %408 = getelementptr inbounds nuw i8, ptr %hot_left.i270.i, i64 576
  %409 = getelementptr inbounds nuw i8, ptr %hot_left.i270.i, i64 544
  %410 = select i1 %_21.i271.i, <8 x i32> %365, <8 x i32> %366
  %411 = icmp slt <8 x i32> %410, zeroinitializer
  %412 = getelementptr inbounds nuw i8, ptr %uniform_left.i263.i, i64 32
  %413 = getelementptr inbounds nuw i8, ptr %uniform_left.i263.i, i64 40
  %_22.i.i391.i = getelementptr inbounds nuw i8, ptr %uniform_left.i263.i, i64 104
  %414 = getelementptr inbounds nuw i8, ptr %uniform_left.i263.i, i64 48
  %415 = getelementptr inbounds nuw i8, ptr %uniform_left.i263.i, i64 56
  %416 = getelementptr inbounds nuw i8, ptr %hot_left.i270.i, i64 672
  %417 = getelementptr inbounds nuw i8, ptr %hot_left.i270.i, i64 704
  %418 = getelementptr inbounds nuw i8, ptr %hot_left.i270.i, i64 640
  %419 = getelementptr inbounds nuw i8, ptr %uniform_left.i263.i, i64 72
  %420 = getelementptr inbounds nuw i8, ptr %uniform_left.i263.i, i64 64
  %421 = select i1 %_22.i274.i, <8 x i32> %365, <8 x i32> %366
  %422 = icmp slt <8 x i32> %421, zeroinitializer
  %_52.i260.sroa.3.0.copyload.pre.i = load i64, ptr %_52.i260.sroa.3.0..sroa_idx.i, align 8, !noalias !6294
  %_52.i260.sroa.4.0.copyload.pre.i = load i64, ptr %_52.i260.sroa.4.0..sroa_idx.i, align 16, !noalias !6294
  %_54.0.i.i376.pre.i = load ptr, ptr %412, align 32, !noalias !6294
  %_54.1.i.i377.pre.i = load i64, ptr %413, align 8, !noalias !6294
  %_18.i.i388.i = load i64, ptr %403, align 16, !noalias !6294
  %_56.0.i.i400.i = load ptr, ptr %414, align 16, !noalias !6294, !nonnull !12, !align !24
  %_56.1.i.i401.i = load i64, ptr %415, align 8, !noalias !6294
  %_58.1.i.i413.i = load i64, ptr %419, align 8, !noalias !6294
  %_58.0.i.i412.i = load ptr, ptr %420, align 32, !noalias !6294, !nonnull !12, !align !24
  %uniform_left.i263.promoted.i = load <8 x float>, ptr %uniform_left.i263.i, align 1, !noalias !6294
  %_22.i.i391.promoted.i = load i32, ptr %_22.i.i391.i, align 4, !noalias !6294
  br label %bb36.i290.i, !dbg !7601

bb16.i329.bb13.i284.loopexit_crit_edge.i:         ; preds = %bb25.i435.i
  store <8 x float> %.lcssa21702828, ptr %416, align 1, !dbg !7625
  store <8 x float> %.lcssa21262838, ptr %407, align 1, !dbg !7651
  store <8 x float> %.lcssa21182847, ptr %_83.i365.i, align 1, !dbg !7654
  store <8 x float> %.lcssa21102857, ptr %408, align 1, !dbg !7655
  store <8 x float> %.lcssa1014610831.i, ptr %404, align 32, !noalias !6294
  store <8 x float> %.lcssa1013810842.i, ptr %_82.i364.i, align 32, !noalias !6294
  store <8 x float> %.lcssa1013010853.i, ptr %405, align 32, !noalias !6294
  br label %bb13.i284.loopexit.i, !dbg !7656

bb13.i284.loopexit.i:                             ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i328.i, %bb16.i329.bb13.i284.loopexit_crit_edge.i
  %storemerge.i.i399.lcssa83568381.lcssa10885.i = phi i32 [ %storemerge.i.i399.lcssa83568381.i, %bb16.i329.bb13.i284.loopexit_crit_edge.i ], [ %storemerge.i.i399.lcssa83568381.lcssa10886.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i328.i ]
  %minimum.i.i45.sroa.0.08238.lcssa8362.lcssa.i = phi <8 x float> [ %minimum.i.i45.sroa.0.08238.lcssa.i, %bb16.i329.bb13.i284.loopexit_crit_edge.i ], [ %minimum.i.i45.sroa.0.08238.lcssa8362.lcssa10875.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i328.i ]
  %ring_cursor.sroa.0.1.i330.lcssa.i = phi i64 [ %ring_cursor.sroa.0.2.i438.i, %bb16.i329.bb13.i284.loopexit_crit_edge.i ], [ %ring_cursor.sroa.0.0.i2858401.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i328.i ], !dbg !7657
  %main_cursor.sroa.0.1.i331.lcssa.i = phi i64 [ %main_cursor.sroa.0.2.i441.i, %bb16.i329.bb13.i284.loopexit_crit_edge.i ], [ %main_cursor.sroa.0.0.i2868402.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i328.i ], !dbg !7658
  %_111.not.i289.i = icmp eq i64 %424, 0, !dbg !7601
  %indvars.iv.next9623.i = add nsw i64 %indvars.iv9622.i, -32, !dbg !7601
  br i1 %_111.not.i289.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i, label %bb36.i290.i, !dbg !7601

bb36.i290.i:                                      ; preds = %bb13.i284.loopexit.i, %bb36.i290.lr.ph.i
  %storemerge.i.i399.lcssa83568381.lcssa10886.i = phi i32 [ %_22.i.i391.promoted.i, %bb36.i290.lr.ph.i ], [ %storemerge.i.i399.lcssa83568381.lcssa10885.i, %bb13.i284.loopexit.i ]
  %minimum.i.i45.sroa.0.08238.lcssa8362.lcssa10875.i = phi <8 x float> [ %uniform_left.i263.promoted.i, %bb36.i290.lr.ph.i ], [ %minimum.i.i45.sroa.0.08238.lcssa8362.lcssa.i, %bb13.i284.loopexit.i ]
  %indvars.iv9622.i = phi i64 [ %_31, %bb36.i290.lr.ph.i ], [ %indvars.iv.next9623.i, %bb13.i284.loopexit.i ]
  %iter3.sroa.0.0.i2888404.i = phi i64 [ %yield_count.sroa.0.0.i.i3912.i, %bb36.i290.lr.ph.i ], [ %424, %bb13.i284.loopexit.i ]
  %iter2.sroa.0.0.i2878403.i = phi i64 [ 0, %bb36.i290.lr.ph.i ], [ %423, %bb13.i284.loopexit.i ]
  %main_cursor.sroa.0.0.i2868402.i = phi i64 [ %363, %bb36.i290.lr.ph.i ], [ %main_cursor.sroa.0.1.i331.lcssa.i, %bb13.i284.loopexit.i ]
  %ring_cursor.sroa.0.0.i2858401.i = phi i64 [ %362, %bb36.i290.lr.ph.i ], [ %ring_cursor.sroa.0.1.i330.lcssa.i, %bb13.i284.loopexit.i ]
  %umin9639.i = tail call i64 @llvm.umin.i64(i64 %indvars.iv9622.i, i64 32), !dbg !7659
  %umax9625.i = tail call i64 @llvm.umax.i64(i64 %umin9639.i, i64 1), !dbg !7659
  %423 = add nuw nsw i64 %iter2.sroa.0.0.i2878403.i, 32, !dbg !7659
  %424 = add nsw i64 %iter3.sroa.0.0.i2888404.i, -1, !dbg !7663
  %_34.i292.i = sub nsw i64 %_31, %iter2.sroa.0.0.i2878403.i, !dbg !7664
  %..i3913.i = tail call noundef i64 @llvm.umin.i64(i64 %_34.i292.i, i64 32), !dbg !7665
  %active_base.i294.i = shl i64 %iter2.sroa.0.0.i2878403.i, 3, !dbg !7669
  %active_base.i2947060.i = add nuw nsw i64 %..i3913.i, %iter2.sroa.0.0.i2878403.i, !dbg !7670
  %_40.i296.i = shl i64 %active_base.i2947060.i, 3, !dbg !7670
  %_123.i297.i = icmp samesign ult i64 %_40.i296.i, %active_base.i294.i, !dbg !7671
  %_117.not.i298.i = icmp ugt i64 %_40.i296.i, %_39.1
  %or.cond.i299.i = or i1 %_123.i297.i, %_117.not.i298.i, !dbg !7671
  br i1 %or.cond.i299.i, label %bb40.i443.i, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit3943.i, !dbg !7671, !prof !165

bb40.i443.i:                                      ; preds = %bb36.i290.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %active_base.i294.i, i64 noundef %_40.i296.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cf577da48352b38fc6c5372e02febe14) #30, !dbg !7678, !noalias !7679
  unreachable, !dbg !7678

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit3943.i: ; preds = %bb36.i290.i
  %_126.i304.i = getelementptr inbounds nuw float, ptr %_39.0, i64 %active_base.i294.i, !dbg !7680
  %history.i.i239.sroa.0.0.copyload.i = load <8 x float>, ptr %hot_left.i270.i, align 32, !dbg !7684, !noalias !7686
  %history.i.i239.sroa.10.sroa.0.0.copyload.i = load <8 x float>, ptr %history.i.i239.sroa.10.0.hot_left.i270.sroa_idx.i, align 32, !dbg !7684, !noalias !7686
  %history.i.i239.sroa.13.sroa.0.0.copyload.i = load <8 x float>, ptr %history.i.i239.sroa.13.0.hot_left.i270.sroa_idx.i, align 32, !dbg !7684, !noalias !7686
  %history.i.i239.sroa.16.sroa.0.0.copyload.i = load <8 x float>, ptr %history.i.i239.sroa.16.0.hot_left.i270.sroa_idx.i, align 32, !dbg !7684, !noalias !7686
  %history.i.i239.sroa.19.sroa.0.0.copyload.i = load <8 x float>, ptr %history.i.i239.sroa.19.0.hot_left.i270.sroa_idx.i, align 32, !dbg !7684, !noalias !7686
  %history.i.i239.sroa.22.sroa.0.0.copyload.i = load <8 x float>, ptr %history.i.i239.sroa.22.0.hot_left.i270.sroa_idx.i, align 32, !dbg !7684, !noalias !7686
  %history.i.i239.sroa.25.sroa.0.0.copyload.i = load <8 x float>, ptr %history.i.i239.sroa.25.0.hot_left.i270.sroa_idx.i, align 32, !dbg !7684, !noalias !7686
  %history.i.i239.sroa.29.sroa.0.0.copyload.i = load <8 x float>, ptr %history.i.i239.sroa.29.0.hot_left.i270.sroa_idx.i, align 32, !dbg !7684, !noalias !7686
  %history.i.i239.sroa.32.sroa.0.0.copyload.i = load <8 x float>, ptr %history.i.i239.sroa.32.0.hot_left.i270.sroa_idx.i, align 32, !dbg !7684, !noalias !7686
  %history.i.i239.sroa.35.sroa.0.0.copyload.i = load <8 x float>, ptr %history.i.i239.sroa.35.0.hot_left.i270.sroa_idx.i, align 32, !dbg !7684, !noalias !7686
  %history.i.i239.sroa.38.sroa.0.0.copyload.i = load <8 x float>, ptr %history.i.i239.sroa.38.0.hot_left.i270.sroa_idx.i, align 32, !dbg !7684, !noalias !7686
  %history.i.i239.sroa.41.sroa.0.0.copyload.i = load <8 x float>, ptr %history.i.i239.sroa.41.0.hot_left.i270.sroa_idx.i, align 32, !dbg !7684, !noalias !7686
  %_2.i39468209.not.i = icmp eq i64 %iter2.sroa.0.0.i2878403.i, %_31, !dbg !7691
  br i1 %_2.i39468209.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i328.i, label %bb6.i.i307.lr.ph.i, !dbg !7691

bb6.i.i307.lr.ph.i:                               ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit3943.i
  %_5.i2822.i = load <8 x float>, ptr %self, align 32, !alias.scope !7694, !noalias !7697
  %_14.i.i.i.i199.sroa.0.0.copyload.i = load <8 x float>, ptr %367, align 32, !alias.scope !5946, !noalias !7708
  %_17.i.i.i.i196.sroa.0.0.copyload.i = load <8 x float>, ptr %368, align 32, !alias.scope !5946, !noalias !7708
  %_20.i.i.i.i193.sroa.0.0.copyload.i = load <8 x float>, ptr %369, align 32, !alias.scope !5946, !noalias !7708
  %_25.i.i.i.i189.sroa.0.0.copyload.i = load <8 x float>, ptr %row12.i.i.i.i311.i, align 32, !alias.scope !5946, !noalias !7708
  %_28.i.i.i.i186.sroa.0.0.copyload.i = load <8 x float>, ptr %370, align 32, !alias.scope !5946, !noalias !7708
  %_31.i.i.i.i183.sroa.0.0.copyload.i = load <8 x float>, ptr %371, align 32, !alias.scope !5946, !noalias !7708
  %_34.i.i.i.i180.sroa.0.0.copyload.i = load <8 x float>, ptr %372, align 32, !alias.scope !5946, !noalias !7708
  %_39.i.i.i.i176.sroa.0.0.copyload.i = load <8 x float>, ptr %row13.i.i.i.i312.i, align 32, !alias.scope !5946, !noalias !7708
  %_42.i.i.i.i173.sroa.0.0.copyload.i = load <8 x float>, ptr %373, align 32, !alias.scope !5946, !noalias !7708
  %_45.i.i.i.i170.sroa.0.0.copyload.i = load <8 x float>, ptr %374, align 32, !alias.scope !5946, !noalias !7708
  %_48.i.i.i.i167.sroa.0.0.copyload.i = load <8 x float>, ptr %375, align 32, !alias.scope !5946, !noalias !7708
  %_53.i.i.i.i163.sroa.0.0.copyload.i = load <8 x float>, ptr %row14.i.i.i.i313.i, align 32, !alias.scope !5946, !noalias !7708
  %_56.i.i.i.i160.sroa.0.0.copyload.i = load <8 x float>, ptr %376, align 32, !alias.scope !5946, !noalias !7708
  %_59.i.i.i.i157.sroa.0.0.copyload.i = load <8 x float>, ptr %377, align 32, !alias.scope !5946, !noalias !7708
  %_62.i.i.i.i154.sroa.0.0.copyload.i = load <8 x float>, ptr %378, align 32, !alias.scope !5946, !noalias !7708
  %_67.i.i.i.i150.sroa.0.0.copyload.i = load <8 x float>, ptr %row15.i.i.i.i314.i, align 32, !alias.scope !5946, !noalias !7708
  %_70.i.i.i.i147.sroa.0.0.copyload.i = load <8 x float>, ptr %379, align 32, !alias.scope !5946, !noalias !7708
  %_73.i.i.i.i144.sroa.0.0.copyload.i = load <8 x float>, ptr %380, align 32, !alias.scope !5946, !noalias !7708
  %_76.i.i.i.i141.sroa.0.0.copyload.i = load <8 x float>, ptr %381, align 32, !alias.scope !5946, !noalias !7708
  %_81.i.i.i.i137.sroa.0.0.copyload.i = load <8 x float>, ptr %row16.i.i.i.i315.i, align 32, !alias.scope !5946, !noalias !7708
  %_84.i.i.i.i134.sroa.0.0.copyload.i = load <8 x float>, ptr %382, align 32, !alias.scope !5946, !noalias !7708
  %_87.i.i.i.i131.sroa.0.0.copyload.i = load <8 x float>, ptr %383, align 32, !alias.scope !5946, !noalias !7708
  %_90.i.i.i.i128.sroa.0.0.copyload.i = load <8 x float>, ptr %384, align 32, !alias.scope !5946, !noalias !7708
  %_95.i.i.i.i124.sroa.0.0.copyload.i = load <8 x float>, ptr %row17.i.i.i.i316.i, align 32, !alias.scope !5946, !noalias !7708
  %_98.i.i.i.i121.sroa.0.0.copyload.i = load <8 x float>, ptr %385, align 32, !alias.scope !5946, !noalias !7708
  %_101.i.i.i.i118.sroa.0.0.copyload.i = load <8 x float>, ptr %386, align 32, !alias.scope !5946, !noalias !7708
  %_104.i.i.i.i115.sroa.0.0.copyload.i = load <8 x float>, ptr %387, align 32, !alias.scope !5946, !noalias !7708
  %_109.i.i.i.i111.sroa.0.0.copyload.i = load <8 x float>, ptr %row18.i.i.i.i317.i, align 32, !alias.scope !5946, !noalias !7708
  %_112.i.i.i.i108.sroa.0.0.copyload.i = load <8 x float>, ptr %388, align 32, !alias.scope !5946, !noalias !7708
  %_115.i.i.i.i105.sroa.0.0.copyload.i = load <8 x float>, ptr %389, align 32, !alias.scope !5946, !noalias !7708
  %_118.i.i.i.i102.sroa.0.0.copyload.i = load <8 x float>, ptr %390, align 32, !alias.scope !5946, !noalias !7708
  %_123.i.i.i.i98.sroa.0.0.copyload.i = load <8 x float>, ptr %row19.i.i.i.i318.i, align 32, !alias.scope !5946, !noalias !7708
  %_126.i.i.i.i95.sroa.0.0.copyload.i = load <8 x float>, ptr %391, align 32, !alias.scope !5946, !noalias !7708
  %_129.i.i.i.i92.sroa.0.0.copyload.i = load <8 x float>, ptr %392, align 32, !alias.scope !5946, !noalias !7708
  %_132.i.i.i.i89.sroa.0.0.copyload.i = load <8 x float>, ptr %393, align 32, !alias.scope !5946, !noalias !7708
  %_137.i.i.i.i85.sroa.0.0.copyload.i = load <8 x float>, ptr %row20.i.i.i.i319.i, align 32, !alias.scope !5946, !noalias !7708
  %_140.i.i.i.i82.sroa.0.0.copyload.i = load <8 x float>, ptr %394, align 32, !alias.scope !5946, !noalias !7708
  %_143.i.i.i.i79.sroa.0.0.copyload.i = load <8 x float>, ptr %395, align 32, !alias.scope !5946, !noalias !7708
  %_146.i.i.i.i76.sroa.0.0.copyload.i = load <8 x float>, ptr %396, align 32, !alias.scope !5946, !noalias !7708
  %_151.i.i.i.i72.sroa.0.0.copyload.i = load <8 x float>, ptr %row21.i.i.i.i320.i, align 32, !alias.scope !5946, !noalias !7708
  %_154.i.i.i.i69.sroa.0.0.copyload.i = load <8 x float>, ptr %397, align 32, !alias.scope !5946, !noalias !7708
  %_157.i.i.i.i66.sroa.0.0.copyload.i = load <8 x float>, ptr %398, align 32, !alias.scope !5946, !noalias !7708
  %_160.i.i.i.i63.sroa.0.0.copyload.i = load <8 x float>, ptr %399, align 32, !alias.scope !5946, !noalias !7708
  %_165.i.i.i.i59.sroa.0.0.copyload.i = load <8 x float>, ptr %row22.i.i.i.i321.i, align 32, !alias.scope !5946, !noalias !7708
  %_168.i.i.i.i56.sroa.0.0.copyload.i = load <8 x float>, ptr %400, align 32, !alias.scope !5946, !noalias !7708
  %_171.i.i.i.i53.sroa.0.0.copyload.i = load <8 x float>, ptr %401, align 32, !alias.scope !5946, !noalias !7708
  %_174.i.i.i.i50.sroa.0.0.copyload.i = load <8 x float>, ptr %402, align 32, !alias.scope !5946, !noalias !7708
  br label %bb6.i.i307.i, !dbg !7691

bb6.i.i307.i:                                     ; preds = %bb6.i.i307.i, %bb6.i.i307.lr.ph.i
  %history.i.i239.sroa.10.sroa.0.08221.i = phi <8 x float> [ %history.i.i239.sroa.10.sroa.0.0.copyload.i, %bb6.i.i307.lr.ph.i ], [ %history.i.i239.sroa.0.08210.i, %bb6.i.i307.i ]
  %history.i.i239.sroa.13.sroa.0.08220.i = phi <8 x float> [ %history.i.i239.sroa.13.sroa.0.0.copyload.i, %bb6.i.i307.lr.ph.i ], [ %history.i.i239.sroa.10.sroa.0.08221.i, %bb6.i.i307.i ]
  %history.i.i239.sroa.16.sroa.0.08219.i = phi <8 x float> [ %history.i.i239.sroa.16.sroa.0.0.copyload.i, %bb6.i.i307.lr.ph.i ], [ %history.i.i239.sroa.13.sroa.0.08220.i, %bb6.i.i307.i ]
  %history.i.i239.sroa.19.sroa.0.08218.i = phi <8 x float> [ %history.i.i239.sroa.19.sroa.0.0.copyload.i, %bb6.i.i307.lr.ph.i ], [ %history.i.i239.sroa.16.sroa.0.08219.i, %bb6.i.i307.i ]
  %history.i.i239.sroa.22.sroa.0.08217.i = phi <8 x float> [ %history.i.i239.sroa.22.sroa.0.0.copyload.i, %bb6.i.i307.lr.ph.i ], [ %history.i.i239.sroa.19.sroa.0.08218.i, %bb6.i.i307.i ]
  %history.i.i239.sroa.38.sroa.0.08216.i = phi <8 x float> [ %history.i.i239.sroa.38.sroa.0.0.copyload.i, %bb6.i.i307.lr.ph.i ], [ %history.i.i239.sroa.35.sroa.0.08215.i, %bb6.i.i307.i ]
  %history.i.i239.sroa.35.sroa.0.08215.i = phi <8 x float> [ %history.i.i239.sroa.35.sroa.0.0.copyload.i, %bb6.i.i307.lr.ph.i ], [ %history.i.i239.sroa.32.sroa.0.08214.i, %bb6.i.i307.i ]
  %history.i.i239.sroa.32.sroa.0.08214.i = phi <8 x float> [ %history.i.i239.sroa.32.sroa.0.0.copyload.i, %bb6.i.i307.lr.ph.i ], [ %history.i.i239.sroa.29.sroa.0.08213.i, %bb6.i.i307.i ]
  %history.i.i239.sroa.29.sroa.0.08213.i = phi <8 x float> [ %history.i.i239.sroa.29.sroa.0.0.copyload.i, %bb6.i.i307.lr.ph.i ], [ %history.i.i239.sroa.25.sroa.0.08212.i, %bb6.i.i307.i ]
  %history.i.i239.sroa.25.sroa.0.08212.i = phi <8 x float> [ %history.i.i239.sroa.25.sroa.0.0.copyload.i, %bb6.i.i307.lr.ph.i ], [ %history.i.i239.sroa.22.sroa.0.08217.i, %bb6.i.i307.i ]
  %iter.i.i235.sroa.16.08211.i = phi i64 [ 0, %bb6.i.i307.lr.ph.i ], [ %530, %bb6.i.i307.i ]
  %history.i.i239.sroa.0.08210.i = phi <8 x float> [ %history.i.i239.sroa.0.0.copyload.i, %bb6.i.i307.lr.ph.i ], [ %lanes.i3248.sroa.0.0.copyload.i, %bb6.i.i307.i ]
  %start1.i.i3952.i = shl i64 %iter.i.i235.sroa.16.08211.i, 3, !dbg !7709
  %data.i.i3953.i = getelementptr inbounds nuw float, ptr %_126.i304.i, i64 %start1.i.i3952.i, !dbg !7711
  %lanes.i3248.sroa.0.0.copyload.i = load <8 x float>, ptr %data.i.i3953.i, align 4, !dbg !7713, !alias.scope !7718, !noalias !7722
  %425 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i.i239.sroa.22.sroa.0.08217.i), !dbg !7726
  %426 = fmul <8 x float> %_5.i2822.i, %lanes.i3248.sroa.0.0.copyload.i, !dbg !7733
  %427 = fadd <8 x float> %426, zeroinitializer, !dbg !7739
  %428 = fmul <8 x float> %_14.i.i.i.i199.sroa.0.0.copyload.i, %lanes.i3248.sroa.0.0.copyload.i, !dbg !7744
  %429 = fadd <8 x float> %428, zeroinitializer, !dbg !7749
  %430 = fmul <8 x float> %_17.i.i.i.i196.sroa.0.0.copyload.i, %lanes.i3248.sroa.0.0.copyload.i, !dbg !7754
  %431 = fadd <8 x float> %430, zeroinitializer, !dbg !7759
  %432 = fmul <8 x float> %_20.i.i.i.i193.sroa.0.0.copyload.i, %lanes.i3248.sroa.0.0.copyload.i, !dbg !7764
  %433 = fadd <8 x float> %432, zeroinitializer, !dbg !7769
  %434 = fmul <8 x float> %_25.i.i.i.i189.sroa.0.0.copyload.i, %history.i.i239.sroa.0.08210.i, !dbg !7774
  %435 = fadd <8 x float> %434, %427, !dbg !7779
  %436 = fmul <8 x float> %_28.i.i.i.i186.sroa.0.0.copyload.i, %history.i.i239.sroa.0.08210.i, !dbg !7784
  %437 = fadd <8 x float> %436, %429, !dbg !7789
  %438 = fmul <8 x float> %_31.i.i.i.i183.sroa.0.0.copyload.i, %history.i.i239.sroa.0.08210.i, !dbg !7794
  %439 = fadd <8 x float> %438, %431, !dbg !7799
  %440 = fmul <8 x float> %_34.i.i.i.i180.sroa.0.0.copyload.i, %history.i.i239.sroa.0.08210.i, !dbg !7804
  %441 = fadd <8 x float> %440, %433, !dbg !7809
  %442 = fmul <8 x float> %_39.i.i.i.i176.sroa.0.0.copyload.i, %history.i.i239.sroa.10.sroa.0.08221.i, !dbg !7814
  %443 = fadd <8 x float> %442, %435, !dbg !7819
  %444 = fmul <8 x float> %_42.i.i.i.i173.sroa.0.0.copyload.i, %history.i.i239.sroa.10.sroa.0.08221.i, !dbg !7824
  %445 = fadd <8 x float> %444, %437, !dbg !7829
  %446 = fmul <8 x float> %_45.i.i.i.i170.sroa.0.0.copyload.i, %history.i.i239.sroa.10.sroa.0.08221.i, !dbg !7834
  %447 = fadd <8 x float> %446, %439, !dbg !7839
  %448 = fmul <8 x float> %_48.i.i.i.i167.sroa.0.0.copyload.i, %history.i.i239.sroa.10.sroa.0.08221.i, !dbg !7844
  %449 = fadd <8 x float> %448, %441, !dbg !7849
  %450 = fmul <8 x float> %_53.i.i.i.i163.sroa.0.0.copyload.i, %history.i.i239.sroa.13.sroa.0.08220.i, !dbg !7854
  %451 = fadd <8 x float> %450, %443, !dbg !7859
  %452 = fmul <8 x float> %_56.i.i.i.i160.sroa.0.0.copyload.i, %history.i.i239.sroa.13.sroa.0.08220.i, !dbg !7864
  %453 = fadd <8 x float> %452, %445, !dbg !7869
  %454 = fmul <8 x float> %_59.i.i.i.i157.sroa.0.0.copyload.i, %history.i.i239.sroa.13.sroa.0.08220.i, !dbg !7874
  %455 = fadd <8 x float> %454, %447, !dbg !7879
  %456 = fmul <8 x float> %_62.i.i.i.i154.sroa.0.0.copyload.i, %history.i.i239.sroa.13.sroa.0.08220.i, !dbg !7884
  %457 = fadd <8 x float> %456, %449, !dbg !7889
  %458 = fmul <8 x float> %_67.i.i.i.i150.sroa.0.0.copyload.i, %history.i.i239.sroa.16.sroa.0.08219.i, !dbg !7894
  %459 = fadd <8 x float> %458, %451, !dbg !7899
  %460 = fmul <8 x float> %_70.i.i.i.i147.sroa.0.0.copyload.i, %history.i.i239.sroa.16.sroa.0.08219.i, !dbg !7904
  %461 = fadd <8 x float> %460, %453, !dbg !7909
  %462 = fmul <8 x float> %_73.i.i.i.i144.sroa.0.0.copyload.i, %history.i.i239.sroa.16.sroa.0.08219.i, !dbg !7914
  %463 = fadd <8 x float> %462, %455, !dbg !7919
  %464 = fmul <8 x float> %_76.i.i.i.i141.sroa.0.0.copyload.i, %history.i.i239.sroa.16.sroa.0.08219.i, !dbg !7924
  %465 = fadd <8 x float> %464, %457, !dbg !7929
  %466 = fmul <8 x float> %_81.i.i.i.i137.sroa.0.0.copyload.i, %history.i.i239.sroa.19.sroa.0.08218.i, !dbg !7934
  %467 = fadd <8 x float> %466, %459, !dbg !7939
  %468 = fmul <8 x float> %_84.i.i.i.i134.sroa.0.0.copyload.i, %history.i.i239.sroa.19.sroa.0.08218.i, !dbg !7944
  %469 = fadd <8 x float> %468, %461, !dbg !7949
  %470 = fmul <8 x float> %_87.i.i.i.i131.sroa.0.0.copyload.i, %history.i.i239.sroa.19.sroa.0.08218.i, !dbg !7954
  %471 = fadd <8 x float> %470, %463, !dbg !7959
  %472 = fmul <8 x float> %_90.i.i.i.i128.sroa.0.0.copyload.i, %history.i.i239.sroa.19.sroa.0.08218.i, !dbg !7964
  %473 = fadd <8 x float> %472, %465, !dbg !7969
  %474 = fmul <8 x float> %_95.i.i.i.i124.sroa.0.0.copyload.i, %history.i.i239.sroa.22.sroa.0.08217.i, !dbg !7974
  %475 = fadd <8 x float> %474, %467, !dbg !7979
  %476 = fmul <8 x float> %_98.i.i.i.i121.sroa.0.0.copyload.i, %history.i.i239.sroa.22.sroa.0.08217.i, !dbg !7984
  %477 = fadd <8 x float> %476, %469, !dbg !7989
  %478 = fmul <8 x float> %_101.i.i.i.i118.sroa.0.0.copyload.i, %history.i.i239.sroa.22.sroa.0.08217.i, !dbg !7994
  %479 = fadd <8 x float> %478, %471, !dbg !7999
  %480 = fmul <8 x float> %_104.i.i.i.i115.sroa.0.0.copyload.i, %history.i.i239.sroa.22.sroa.0.08217.i, !dbg !8004
  %481 = fadd <8 x float> %480, %473, !dbg !8009
  %482 = fmul <8 x float> %_109.i.i.i.i111.sroa.0.0.copyload.i, %history.i.i239.sroa.25.sroa.0.08212.i, !dbg !8014
  %483 = fadd <8 x float> %482, %475, !dbg !8019
  %484 = fmul <8 x float> %_112.i.i.i.i108.sroa.0.0.copyload.i, %history.i.i239.sroa.25.sroa.0.08212.i, !dbg !8024
  %485 = fadd <8 x float> %484, %477, !dbg !8029
  %486 = fmul <8 x float> %_115.i.i.i.i105.sroa.0.0.copyload.i, %history.i.i239.sroa.25.sroa.0.08212.i, !dbg !8034
  %487 = fadd <8 x float> %486, %479, !dbg !8039
  %488 = fmul <8 x float> %_118.i.i.i.i102.sroa.0.0.copyload.i, %history.i.i239.sroa.25.sroa.0.08212.i, !dbg !8044
  %489 = fadd <8 x float> %488, %481, !dbg !8049
  %490 = fmul <8 x float> %_123.i.i.i.i98.sroa.0.0.copyload.i, %history.i.i239.sroa.29.sroa.0.08213.i, !dbg !8054
  %491 = fadd <8 x float> %490, %483, !dbg !8059
  %492 = fmul <8 x float> %_126.i.i.i.i95.sroa.0.0.copyload.i, %history.i.i239.sroa.29.sroa.0.08213.i, !dbg !8064
  %493 = fadd <8 x float> %492, %485, !dbg !8069
  %494 = fmul <8 x float> %_129.i.i.i.i92.sroa.0.0.copyload.i, %history.i.i239.sroa.29.sroa.0.08213.i, !dbg !8074
  %495 = fadd <8 x float> %494, %487, !dbg !8079
  %496 = fmul <8 x float> %_132.i.i.i.i89.sroa.0.0.copyload.i, %history.i.i239.sroa.29.sroa.0.08213.i, !dbg !8084
  %497 = fadd <8 x float> %496, %489, !dbg !8089
  %498 = fmul <8 x float> %_137.i.i.i.i85.sroa.0.0.copyload.i, %history.i.i239.sroa.32.sroa.0.08214.i, !dbg !8094
  %499 = fadd <8 x float> %498, %491, !dbg !8099
  %500 = fmul <8 x float> %_140.i.i.i.i82.sroa.0.0.copyload.i, %history.i.i239.sroa.32.sroa.0.08214.i, !dbg !8104
  %501 = fadd <8 x float> %500, %493, !dbg !8109
  %502 = fmul <8 x float> %_143.i.i.i.i79.sroa.0.0.copyload.i, %history.i.i239.sroa.32.sroa.0.08214.i, !dbg !8114
  %503 = fadd <8 x float> %502, %495, !dbg !8119
  %504 = fmul <8 x float> %_146.i.i.i.i76.sroa.0.0.copyload.i, %history.i.i239.sroa.32.sroa.0.08214.i, !dbg !8124
  %505 = fadd <8 x float> %504, %497, !dbg !8129
  %506 = fmul <8 x float> %_151.i.i.i.i72.sroa.0.0.copyload.i, %history.i.i239.sroa.35.sroa.0.08215.i, !dbg !8134
  %507 = fadd <8 x float> %506, %499, !dbg !8139
  %508 = fmul <8 x float> %_154.i.i.i.i69.sroa.0.0.copyload.i, %history.i.i239.sroa.35.sroa.0.08215.i, !dbg !8144
  %509 = fadd <8 x float> %508, %501, !dbg !8149
  %510 = fmul <8 x float> %_157.i.i.i.i66.sroa.0.0.copyload.i, %history.i.i239.sroa.35.sroa.0.08215.i, !dbg !8154
  %511 = fadd <8 x float> %510, %503, !dbg !8159
  %512 = fmul <8 x float> %_160.i.i.i.i63.sroa.0.0.copyload.i, %history.i.i239.sroa.35.sroa.0.08215.i, !dbg !8164
  %513 = fadd <8 x float> %512, %505, !dbg !8169
  %514 = fmul <8 x float> %_165.i.i.i.i59.sroa.0.0.copyload.i, %history.i.i239.sroa.38.sroa.0.08216.i, !dbg !8174
  %515 = fadd <8 x float> %514, %507, !dbg !8179
  %516 = fmul <8 x float> %_168.i.i.i.i56.sroa.0.0.copyload.i, %history.i.i239.sroa.38.sroa.0.08216.i, !dbg !8184
  %517 = fadd <8 x float> %516, %509, !dbg !8189
  %518 = fmul <8 x float> %_171.i.i.i.i53.sroa.0.0.copyload.i, %history.i.i239.sroa.38.sroa.0.08216.i, !dbg !8194
  %519 = fadd <8 x float> %518, %511, !dbg !8199
  %520 = fmul <8 x float> %_174.i.i.i.i50.sroa.0.0.copyload.i, %history.i.i239.sroa.38.sroa.0.08216.i, !dbg !8204
  %521 = fadd <8 x float> %520, %513, !dbg !8209
  %522 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %515), !dbg !8214
  %523 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %425, <8 x float> %522), !dbg !8220
  %524 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %517), !dbg !8214
  %525 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %523, <8 x float> %524), !dbg !8220
  %526 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %519), !dbg !8214
  %527 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %525, <8 x float> %526), !dbg !8220
  %528 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %521), !dbg !8214
  %529 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %527, <8 x float> %528), !dbg !8220
  %530 = add nuw nsw i64 %iter.i.i235.sroa.16.08211.i, 1, !dbg !8225
  %data.i4.i3957.i = getelementptr inbounds nuw float, ptr %peaks_left.i264.i, i64 %start1.i.i3952.i, !dbg !8226
  store <8 x float> %529, ptr %data.i4.i3957.i, align 4, !dbg !8229, !alias.scope !8234, !noalias !8238
  %exitcond9626.not.i = icmp eq i64 %530, %umax9625.i, !dbg !7691
  br i1 %exitcond9626.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i328.i, label %bb6.i.i307.i, !dbg !7691

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i328.i: ; preds = %bb6.i.i307.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit3943.i
  %history.i.i239.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i239.sroa.0.0.copyload.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit3943.i ], [ %lanes.i3248.sroa.0.0.copyload.i, %bb6.i.i307.i ], !dbg !8242
  %history.i.i239.sroa.25.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i239.sroa.25.sroa.0.0.copyload.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit3943.i ], [ %history.i.i239.sroa.22.sroa.0.08217.i, %bb6.i.i307.i ], !dbg !8242
  %history.i.i239.sroa.29.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i239.sroa.29.sroa.0.0.copyload.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit3943.i ], [ %history.i.i239.sroa.25.sroa.0.08212.i, %bb6.i.i307.i ], !dbg !8242
  %history.i.i239.sroa.32.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i239.sroa.32.sroa.0.0.copyload.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit3943.i ], [ %history.i.i239.sroa.29.sroa.0.08213.i, %bb6.i.i307.i ], !dbg !8242
  %history.i.i239.sroa.35.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i239.sroa.35.sroa.0.0.copyload.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit3943.i ], [ %history.i.i239.sroa.32.sroa.0.08214.i, %bb6.i.i307.i ], !dbg !8242
  %history.i.i239.sroa.38.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i239.sroa.38.sroa.0.0.copyload.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit3943.i ], [ %history.i.i239.sroa.35.sroa.0.08215.i, %bb6.i.i307.i ], !dbg !8242
  %history.i.i239.sroa.41.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i239.sroa.41.sroa.0.0.copyload.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit3943.i ], [ %history.i.i239.sroa.38.sroa.0.08216.i, %bb6.i.i307.i ], !dbg !8242
  %history.i.i239.sroa.22.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i239.sroa.22.sroa.0.0.copyload.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit3943.i ], [ %history.i.i239.sroa.19.sroa.0.08218.i, %bb6.i.i307.i ], !dbg !8242
  %history.i.i239.sroa.19.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i239.sroa.19.sroa.0.0.copyload.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit3943.i ], [ %history.i.i239.sroa.16.sroa.0.08219.i, %bb6.i.i307.i ], !dbg !8242
  %history.i.i239.sroa.16.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i239.sroa.16.sroa.0.0.copyload.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit3943.i ], [ %history.i.i239.sroa.13.sroa.0.08220.i, %bb6.i.i307.i ], !dbg !8242
  %history.i.i239.sroa.13.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i239.sroa.13.sroa.0.0.copyload.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit3943.i ], [ %history.i.i239.sroa.10.sroa.0.08221.i, %bb6.i.i307.i ], !dbg !8242
  %history.i.i239.sroa.10.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i239.sroa.10.sroa.0.0.copyload.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit3943.i ], [ %history.i.i239.sroa.0.08210.i, %bb6.i.i307.i ], !dbg !8242
  store <8 x float> %history.i.i239.sroa.0.0.lcssa.i, ptr %hot_left.i270.i, align 32, !dbg !8243, !noalias !7686
  store <8 x float> %history.i.i239.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i239.sroa.10.0.hot_left.i270.sroa_idx.i, align 32, !dbg !8243, !noalias !7686
  store <8 x float> %history.i.i239.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i239.sroa.13.0.hot_left.i270.sroa_idx.i, align 32, !dbg !8243, !noalias !7686
  store <8 x float> %history.i.i239.sroa.16.sroa.0.0.lcssa.i, ptr %history.i.i239.sroa.16.0.hot_left.i270.sroa_idx.i, align 32, !dbg !8243, !noalias !7686
  store <8 x float> %history.i.i239.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i239.sroa.19.0.hot_left.i270.sroa_idx.i, align 32, !dbg !8243, !noalias !7686
  store <8 x float> %history.i.i239.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i239.sroa.22.0.hot_left.i270.sroa_idx.i, align 32, !dbg !8243, !noalias !7686
  store <8 x float> %history.i.i239.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i239.sroa.25.0.hot_left.i270.sroa_idx.i, align 32, !dbg !8243, !noalias !7686
  store <8 x float> %history.i.i239.sroa.29.sroa.0.0.lcssa.i, ptr %history.i.i239.sroa.29.0.hot_left.i270.sroa_idx.i, align 32, !dbg !8243, !noalias !7686
  store <8 x float> %history.i.i239.sroa.32.sroa.0.0.lcssa.i, ptr %history.i.i239.sroa.32.0.hot_left.i270.sroa_idx.i, align 32, !dbg !8243, !noalias !7686
  store <8 x float> %history.i.i239.sroa.35.sroa.0.0.lcssa.i, ptr %history.i.i239.sroa.35.0.hot_left.i270.sroa_idx.i, align 32, !dbg !8243, !noalias !7686
  store <8 x float> %history.i.i239.sroa.38.sroa.0.0.lcssa.i, ptr %history.i.i239.sroa.38.0.hot_left.i270.sroa_idx.i, align 32, !dbg !8243, !noalias !7686
  store <8 x float> %history.i.i239.sroa.41.sroa.0.0.lcssa.i, ptr %history.i.i239.sroa.41.0.hot_left.i270.sroa_idx.i, align 32, !dbg !8243, !noalias !7686
  br i1 %_2.i39468209.not.i, label %bb13.i284.loopexit.i, label %bb17.i334.lr.ph.i, !dbg !7656

bb17.i334.lr.ph.i:                                ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i328.i
  %_13.i1488.sroa.0.0.copyload.i = load <8 x float>, ptr %406, align 32, !noalias !6294
  %_13.i1474.sroa.0.0.copyload.i = load <8 x float>, ptr %409, align 32, !noalias !6294
  %_37.i.i33.sroa.0.0.copyload.i = load <8 x float>, ptr %417, align 32, !noalias !6294
  %.promoted10830.i = load <8 x float>, ptr %404, align 32, !noalias !6294
  %_82.i364.promoted10841.i = load <8 x float>, ptr %_82.i364.i, align 32, !noalias !6294
  %.promoted10852.i = load <8 x float>, ptr %405, align 32, !noalias !6294
  %.promoted10863.i = load <8 x float>, ptr %407, align 32, !noalias !6294
  %_83.i365.promoted10866.i = load <8 x float>, ptr %_83.i365.i, align 32, !noalias !6294
  %.promoted10869.i = load <8 x float>, ptr %408, align 32, !noalias !6294
  %.promoted10872.i = load <8 x float>, ptr %416, align 32, !noalias !6294
  br label %bb17.i334.i, !dbg !7656

bb17.i334.i:                                      ; preds = %bb25.i435.i, %bb17.i334.lr.ph.i
  %.lcssa21102858 = phi <8 x float> [ %.promoted10869.i, %bb17.i334.lr.ph.i ], [ %.lcssa21102857, %bb25.i435.i ]
  %.lcssa21182848 = phi <8 x float> [ %_83.i365.promoted10866.i, %bb17.i334.lr.ph.i ], [ %.lcssa21182847, %bb25.i435.i ]
  %.lcssa21262839 = phi <8 x float> [ %.promoted10863.i, %bb17.i334.lr.ph.i ], [ %.lcssa21262838, %bb25.i435.i ]
  %.lcssa21702829 = phi <8 x float> [ %.promoted10872.i, %bb17.i334.lr.ph.i ], [ %.lcssa21702828, %bb25.i435.i ]
  %.lcssa1082910874.i = phi <8 x float> [ %.promoted10872.i, %bb17.i334.lr.ph.i ], [ %.lcssa1082910873.i, %bb25.i435.i ]
  %.lcssa1010610871.i = phi <8 x float> [ %.promoted10869.i, %bb17.i334.lr.ph.i ], [ %.lcssa1010610870.i, %bb25.i435.i ]
  %.lcssa1011410868.i = phi <8 x float> [ %_83.i365.promoted10866.i, %bb17.i334.lr.ph.i ], [ %.lcssa1011410867.i, %bb25.i435.i ]
  %.lcssa1012210865.i = phi <8 x float> [ %.promoted10863.i, %bb17.i334.lr.ph.i ], [ %.lcssa1012210864.i, %bb25.i435.i ]
  %.lcssa1013010854.i = phi <8 x float> [ %.promoted10852.i, %bb17.i334.lr.ph.i ], [ %.lcssa1013010853.i, %bb25.i435.i ]
  %.lcssa1013810843.i = phi <8 x float> [ %_82.i364.promoted10841.i, %bb17.i334.lr.ph.i ], [ %.lcssa1013810842.i, %bb25.i435.i ]
  %.lcssa1014610832.i = phi <8 x float> [ %.promoted10830.i, %bb17.i334.lr.ph.i ], [ %.lcssa1014610831.i, %bb25.i435.i ]
  %storemerge.i.i399.lcssa83568382.i = phi i32 [ %storemerge.i.i399.lcssa83568381.lcssa10886.i, %bb17.i334.lr.ph.i ], [ %storemerge.i.i399.lcssa83568381.i, %bb25.i435.i ]
  %frame.sroa.0.0.i3328376.i = phi i64 [ 0, %bb17.i334.lr.ph.i ], [ %_66.i347.i, %bb25.i435.i ]
  %main_cursor.sroa.0.1.i3318375.i = phi i64 [ %main_cursor.sroa.0.0.i2868402.i, %bb17.i334.lr.ph.i ], [ %main_cursor.sroa.0.2.i441.i, %bb25.i435.i ]
  %ring_cursor.sroa.0.1.i3308374.i = phi i64 [ %ring_cursor.sroa.0.0.i2858401.i, %bb17.i334.lr.ph.i ], [ %ring_cursor.sroa.0.2.i438.i, %bb25.i435.i ]
  %minimum.i.i45.sroa.0.08238.lcssa83628373.i = phi <8 x float> [ %minimum.i.i45.sroa.0.08238.lcssa8362.lcssa10875.i, %bb17.i334.lr.ph.i ], [ %minimum.i.i45.sroa.0.08238.lcssa.i, %bb25.i435.i ]
  %_50.i335.i = sub nuw nsw i64 %..i3913.i, %frame.sroa.0.0.i3328376.i, !dbg !8244
  %ring.i1386.i = load i64, ptr %358, align 8, !dbg !8245, !alias.scope !8247, !noalias !8250, !noundef !12
  %main.i1387.i = load i64, ptr %359, align 8, !dbg !8254, !alias.scope !8247, !noalias !8250, !noundef !12
  %_10.i.i = add i64 %ring_cursor.sroa.0.1.i3308374.i, 1, !dbg !8255
  %_45.not.i.i = icmp ult i64 %_10.i.i, %ring.i1386.i, !dbg !8256
  %531 = select i1 %_45.not.i.i, i64 0, i64 %ring.i1386.i, !dbg !8256
  %start1.sroa.0.0.i1388.i = sub nuw i64 %_10.i.i, %531, !dbg !8256
  %_12.i1389.i = add i64 %ring_cursor.sroa.0.1.i3308374.i, %_52.i260.sroa.3.0.copyload.pre.i, !dbg !8258
  %_46.not.i.i = icmp ult i64 %_12.i1389.i, %ring.i1386.i, !dbg !8259
  %532 = select i1 %_46.not.i.i, i64 0, i64 %ring.i1386.i, !dbg !8259
  %left_end.sroa.0.0.i.i = sub nuw i64 %_12.i1389.i, %532, !dbg !8259
  %_18.i1393.i = add i64 %ring_cursor.sroa.0.1.i3308374.i, %_52.i260.sroa.4.0.copyload.pre.i, !dbg !8261
  %_48.not.i.i = icmp ult i64 %_18.i1393.i, %ring.i1386.i, !dbg !8262
  %533 = select i1 %_48.not.i.i, i64 0, i64 %ring.i1386.i, !dbg !8262
  %left_expiring.sroa.0.0.i.i = sub nuw i64 %_18.i1393.i, %533, !dbg !8262
  %_30.i1396.i = sub i64 %ring.i1386.i, %ring_cursor.sroa.0.1.i3308374.i, !dbg !8264
  %..i3974.i = tail call noundef i64 @llvm.umin.i64(i64 %_30.i1396.i, i64 %_50.i335.i), !dbg !8265
  %_31.i.i = sub i64 %main.i1387.i, %main_cursor.sroa.0.1.i3318375.i, !dbg !8267
  %..i3975.i = tail call noundef i64 @llvm.umin.i64(i64 %_31.i.i, i64 %..i3974.i), !dbg !8268
  %_32.i1399.i = sub i64 %ring.i1386.i, %start1.sroa.0.0.i1388.i, !dbg !8270
  %..i3976.i = tail call noundef i64 @llvm.umin.i64(i64 %_32.i1399.i, i64 %..i3975.i), !dbg !8271
  %_34.i1401.i = sub i64 %ring.i1386.i, %left_end.sroa.0.0.i.i, !dbg !8273
  %..i3977.i = tail call noundef i64 @llvm.umin.i64(i64 %_34.i1401.i, i64 %..i3976.i), !dbg !8274
  %_38.i.i = sub i64 %ring.i1386.i, %left_expiring.sroa.0.0.i.i, !dbg !8276
  %..i3979.i = tail call noundef i64 @llvm.umin.i64(i64 %_38.i.i, i64 %..i3977.i), !dbg !8277
  %_56.i337.i = add i64 %frame.sroa.0.0.i3328376.i, %iter2.sroa.0.0.i2878403.i, !dbg !8279
  %base.i338.i = shl i64 %_56.i337.i, 3, !dbg !8279
  %base.i3387062.i = add i64 %..i3979.i, %_56.i337.i, !dbg !8280
  %_60.i340.i = shl i64 %base.i3387062.i, 3, !dbg !8280
  %_138.i341.i = icmp ult i64 %_60.i340.i, %base.i338.i, !dbg !8281
  %_134.not.i342.i = icmp ugt i64 %_60.i340.i, %_39.1
  %or.cond16.i343.i = or i1 %_138.i341.i, %_134.not.i342.i, !dbg !8281
  br i1 %or.cond16.i343.i, label %bb48.i442.i, label %bb47.i344.i, !dbg !8281, !prof !165

bb48.i442.i:                                      ; preds = %bb17.i334.i
  store <8 x float> %.lcssa21702829, ptr %416, align 1, !dbg !7625
  store <8 x float> %.lcssa21262839, ptr %407, align 1, !dbg !7651
  store <8 x float> %.lcssa21182848, ptr %_83.i365.i, align 1, !dbg !7654
  store <8 x float> %.lcssa21102858, ptr %408, align 1, !dbg !7655
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i338.i, i64 noundef %_60.i340.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e76d4292d58481de1418049881299651) #30, !dbg !8289, !noalias !7679
  unreachable, !dbg !8289

bb47.i344.i:                                      ; preds = %bb17.i334.i
  %_141.i345.i = getelementptr inbounds nuw float, ptr %_39.0, i64 %base.i338.i, !dbg !8290
  %_66.i347.i = add nuw nsw i64 %..i3979.i, %frame.sroa.0.0.i3328376.i, !dbg !8294
  %_150.i355.i.idx = shl nuw nsw i64 %frame.sroa.0.0.i3328376.i, 5, !dbg !8295
  %_150.i355.i = getelementptr inbounds nuw i8, ptr %peaks_left.i264.i, i64 %_150.i355.i.idx, !dbg !8295
  %_2.i.i.i8245.not.i = icmp eq i64 %..i3979.i, 0, !dbg !8304
  br i1 %_2.i.i.i8245.not.i, label %bb25.i435.i, label %bb24.i359.lr.ph.i, !dbg !8304

bb24.i359.lr.ph.i:                                ; preds = %bb47.i344.i
  %umin9635.i = tail call i64 @llvm.umin.i64(i64 %_34.i1401.i, i64 %_38.i.i), !dbg !8304
  %umin9636.i = tail call i64 @llvm.umin.i64(i64 %umin9635.i, i64 %_32.i1399.i), !dbg !8304
  %umin9637.i = tail call i64 @llvm.umin.i64(i64 %umin9636.i, i64 %_30.i1396.i), !dbg !8304
  %umin9638.i = tail call i64 @llvm.umin.i64(i64 %umin9637.i, i64 %_31.i.i), !dbg !8304
  %534 = sub nsw i64 %umin9639.i, %frame.sroa.0.0.i3328376.i, !dbg !8304
  %umin9640.i = tail call i64 @llvm.umin.i64(i64 %umin9638.i, i64 %534), !dbg !8304
  %535 = and i64 %umin9640.i, 2305843009213693951, !dbg !8304
  br label %bb24.i359.i, !dbg !8304

bb24.i359.i:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i, %bb24.i359.lr.ph.i
  %536 = phi <8 x float> [ %.lcssa1082910874.i, %bb24.i359.lr.ph.i ], [ %578, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ]
  %storemerge.i.i3998346.i = phi i32 [ %storemerge.i.i399.lcssa83568382.i, %bb24.i359.lr.ph.i ], [ %storemerge.i.i399.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ]
  %_12.i1475.sroa.0.0.copyload8330.i = phi <8 x float> [ %.lcssa1010610871.i, %bb24.i359.lr.ph.i ], [ %554, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ]
  %_11.i1476.sroa.0.0.copyload8314.i = phi <8 x float> [ %.lcssa1011410868.i, %bb24.i359.lr.ph.i ], [ %553, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ]
  %537 = phi <8 x float> [ %.lcssa1012210865.i, %bb24.i359.lr.ph.i ], [ %548, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ]
  %_12.i1489.sroa.0.0.copyload8282.i = phi <8 x float> [ %.lcssa1013010854.i, %bb24.i359.lr.ph.i ], [ %546, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ]
  %_11.i1490.sroa.0.0.copyload8266.i = phi <8 x float> [ %.lcssa1013810843.i, %bb24.i359.lr.ph.i ], [ %545, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ]
  %538 = phi <8 x float> [ %.lcssa1014610832.i, %bb24.i359.lr.ph.i ], [ %540, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ]
  %iter.i254.sroa.21.08248.i = phi i64 [ 0, %bb24.i359.lr.ph.i ], [ %_9.0.i4013.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ]
  %minimum.i.i45.sroa.0.082388246.i = phi <8 x float> [ %minimum.i.i45.sroa.0.08238.lcssa83628373.i, %bb24.i359.lr.ph.i ], [ %minimum.i.i45.sroa.0.0.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ]
  %start1.i.i.i.i.i = shl i64 %iter.i254.sroa.21.08248.i, 3, !dbg !8308
  %data.i.i.i.i4010.i = getelementptr inbounds nuw float, ptr %_141.i345.i, i64 %start1.i.i.i.i.i, !dbg !8310
  %_9.0.i4013.i = add nuw nsw i64 %iter.i254.sroa.21.08248.i, 1, !dbg !8312
  %539 = fadd <8 x float> %538, splat (float -1.000000e+00), !dbg !8313
  %540 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %539, <8 x float> zeroinitializer), !dbg !8319
  %541 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %540, <8 x float> zeroinitializer, i8 30), !dbg !8324
  %542 = fadd <8 x float> %_12.i1489.sroa.0.0.copyload8282.i, %_11.i1490.sroa.0.0.copyload8266.i, !dbg !8330
  %543 = bitcast <8 x float> %541 to <8 x i32>, !dbg !8335
  %544 = icmp slt <8 x i32> %543, zeroinitializer, !dbg !8339
  %545 = select <8 x i1> %544, <8 x float> %542, <8 x float> %_13.i1488.sroa.0.0.copyload.i, !dbg !8339
  %546 = select <8 x i1> %544, <8 x float> %_12.i1489.sroa.0.0.copyload8282.i, <8 x float> zeroinitializer, !dbg !8341
  %547 = fadd <8 x float> %537, splat (float -1.000000e+00), !dbg !8346
  %548 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %547, <8 x float> zeroinitializer), !dbg !8351
  %549 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %548, <8 x float> zeroinitializer, i8 30), !dbg !8356
  %550 = fadd <8 x float> %_12.i1475.sroa.0.0.copyload8330.i, %_11.i1476.sroa.0.0.copyload8314.i, !dbg !8362
  %551 = bitcast <8 x float> %549 to <8 x i32>, !dbg !8367
  %552 = icmp slt <8 x i32> %551, zeroinitializer, !dbg !8371
  %553 = select <8 x i1> %552, <8 x float> %550, <8 x float> %_13.i1474.sroa.0.0.copyload.i, !dbg !8371
  %554 = select <8 x i1> %552, <8 x float> %_12.i1475.sroa.0.0.copyload8330.i, <8 x float> zeroinitializer, !dbg !8373
  %_152.i371.i = add i64 %iter.i254.sroa.21.08248.i, %ring_cursor.sroa.0.1.i3308374.i, !dbg !8378
  %_153.i372.i = add i64 %iter.i254.sroa.21.08248.i, %main_cursor.sroa.0.1.i3318375.i, !dbg !8381
  %_154.i373.i = add i64 %iter.i254.sroa.21.08248.i, %left_end.sroa.0.0.i.i, !dbg !8382
  %_155.i374.i = add i64 %iter.i254.sroa.21.08248.i, %start1.sroa.0.0.i1388.i, !dbg !8383
  %_156.i375.i = add i64 %iter.i254.sroa.21.08248.i, %left_expiring.sroa.0.0.i.i, !dbg !8384
  %base.i9.i.i379.i = shl i64 %_152.i371.i, 3, !dbg !8385
  %_7.i10.i.i380.i = add i64 %base.i9.i.i379.i, 8, !dbg !8388
  %555 = or disjoint i64 %base.i9.i.i379.i, 7, !dbg !8390
  %or.cond.i13.i.i383.not.i = icmp ult i64 %555, %_54.1.i.i377.pre.i, !dbg !8390
  br i1 %or.cond.i13.i.i383.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i384.i, label %bb4.i15.i.i434.i, !dbg !8390, !prof !2740

bb4.i15.i.i434.i:                                 ; preds = %bb24.i359.i
  store <8 x float> %.lcssa21702829, ptr %416, align 1, !dbg !7625
  store <8 x float> %.lcssa21262839, ptr %407, align 1, !dbg !7651
  store <8 x float> %.lcssa21182848, ptr %_83.i365.i, align 1, !dbg !7654
  store <8 x float> %.lcssa21102858, ptr %408, align 1, !dbg !7655
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i.i379.i, i64 noundef %_7.i10.i.i380.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i377.pre.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !8397, !noalias !8398
  unreachable, !dbg !8397

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i384.i: ; preds = %bb24.i359.i
  %data.i4.i.i.i.i = getelementptr inbounds nuw float, ptr %_150.i355.i, i64 %start1.i.i.i.i.i, !dbg !8412
  %lanes.i3239.sroa.0.0.copyload.i = load <8 x float>, ptr %data.i4.i.i.i.i, align 4, !dbg !8415, !alias.scope !8420, !noalias !8424
  %556 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i3239.sroa.0.0.copyload.i, <8 x float> %lanes.i3239.sroa.0.0.copyload.i), !dbg !8428
  %557 = select <8 x i1> %411, <8 x float> %556, <8 x float> %lanes.i3239.sroa.0.0.copyload.i, !dbg !8433
  %558 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %557, <8 x float> %545, i8 30), !dbg !8438
  %559 = bitcast <8 x float> %558 to <8 x i32>, !dbg !8444
  %560 = icmp slt <8 x i32> %559, zeroinitializer, !dbg !8448
  %561 = fdiv <8 x float> %545, %557, !dbg !8450
  %562 = select <8 x i1> %560, <8 x float> %561, <8 x float> splat (float 1.000000e+00), !dbg !8448
  %_17.i14.i.i385.i = getelementptr inbounds nuw float, ptr %_54.0.i.i376.pre.i, i64 %base.i9.i.i379.i, !dbg !8455
  store <8 x float> %562, ptr %_17.i14.i.i385.i, align 4, !dbg !8459, !alias.scope !8464, !noalias !8468
  %base.i1174.i = shl i64 %_154.i373.i, 3, !dbg !8472
  %563 = or disjoint i64 %base.i1174.i, 7, !dbg !8477
  %or.cond.i1178.not.i = icmp ult i64 %563, %_54.1.i.i377.pre.i, !dbg !8477
  br i1 %or.cond.i1178.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1182.i, label %bb4.i1181.i, !dbg !8477, !prof !2740

bb4.i1181.i:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i384.i
  store <8 x float> %.lcssa21702829, ptr %416, align 1, !dbg !7625
  store <8 x float> %.lcssa21262839, ptr %407, align 1, !dbg !7651
  store <8 x float> %.lcssa21182848, ptr %_83.i365.i, align 1, !dbg !7654
  store <8 x float> %.lcssa21102858, ptr %408, align 1, !dbg !7655
  %_5.i1175.i = add i64 %base.i1174.i, 8, !dbg !8485
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1174.i, i64 noundef %_5.i1175.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i377.pre.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !8486, !noalias !8487
  unreachable, !dbg !8486

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1182.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i384.i
  %_15.i1180.i = getelementptr inbounds nuw float, ptr %_54.0.i.i376.pre.i, i64 %base.i1174.i, !dbg !8495
  %lanes.i3088.sroa.0.0.copyload.i = load <8 x float>, ptr %_15.i1180.i, align 4, !dbg !8499, !alias.scope !8504, !noalias !8508
  %position.i.i392.i = zext i32 %storemerge.i.i3998346.i to i64, !dbg !8512
  %564 = icmp eq i32 %storemerge.i.i3998346.i, 0, !dbg !8514
  br i1 %564, label %bb5.i.i394.i, label %bb3.i.i393.i, !dbg !8514

bb3.i.i393.i:                                     ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1182.i
  %565 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %minimum.i.i45.sroa.0.082388246.i, <8 x float> %lanes.i3088.sroa.0.0.copyload.i), !dbg !8516
  br label %bb5.i.i394.i, !dbg !8525

bb5.i.i394.i:                                     ; preds = %bb3.i.i393.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1182.i
  %minimum.i.i45.sroa.0.0.i = phi <8 x float> [ %565, %bb3.i.i393.i ], [ %lanes.i3088.sroa.0.0.copyload.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1182.i ], !dbg !8526
  %_15.i.i395.i = add nuw nsw i64 %position.i.i392.i, 1, !dbg !8527
  %complete.i.i396.i = icmp eq i64 %_15.i.i395.i, %_18.i.i388.i, !dbg !8527
  br i1 %complete.i.i396.i, label %bb19.i.i430.i, label %bb7.i.i397.i, !dbg !8529

bb7.i.i397.i:                                     ; preds = %bb5.i.i394.i
  %base.i1165.i = shl i64 %_155.i374.i, 3, !dbg !8531
  %566 = or disjoint i64 %base.i1165.i, 7, !dbg !8533
  %or.cond.i1169.not.i = icmp ult i64 %566, %_54.1.i.i377.pre.i, !dbg !8533
  br i1 %or.cond.i1169.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1173.i, label %bb4.i1172.i, !dbg !8533, !prof !2740

bb4.i1172.i:                                      ; preds = %bb7.i.i397.i
  store <8 x float> %.lcssa21702829, ptr %416, align 1, !dbg !7625
  store <8 x float> %.lcssa21262839, ptr %407, align 1, !dbg !7651
  store <8 x float> %.lcssa21182848, ptr %_83.i365.i, align 1, !dbg !7654
  store <8 x float> %.lcssa21102858, ptr %408, align 1, !dbg !7655
  %_5.i1166.i = add i64 %base.i1165.i, 8, !dbg !8537
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1165.i, i64 noundef %_5.i1166.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i377.pre.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !8538, !noalias !8539
  unreachable, !dbg !8538

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1173.i: ; preds = %bb7.i.i397.i
  %_15.i1171.i = getelementptr inbounds nuw float, ptr %_54.0.i.i376.pre.i, i64 %base.i1165.i, !dbg !8543
  %lanes.i3095.sroa.0.0.copyload.i = load <8 x float>, ptr %_15.i1171.i, align 4, !dbg !8545, !alias.scope !8550, !noalias !8554
  %567 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %lanes.i3095.sroa.0.0.copyload.i, <8 x float> %minimum.i.i45.sroa.0.0.i), !dbg !8558
  %568 = trunc i64 %_15.i.i395.i to i32, !dbg !8563
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i398.i, !dbg !8565

bb19.i.i430.i:                                    ; preds = %bb5.i.i394.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i
  %end.sroa.0.0.i.i4288237.i = phi i64 [ %572, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ], [ %_154.i373.i, %bb5.i.i394.i ]
  %iter.sroa.0.0.i.i4278236.i = phi i64 [ %_30.i32.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ], [ 0, %bb5.i.i394.i ]
  %suffix.i.i12.sroa.0.08235.i = phi <8 x float> [ %570, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ], [ %lanes.i3088.sroa.0.0.copyload.i, %bb5.i.i394.i ]
  %base.i1134.i = shl i64 %end.sroa.0.0.i.i4288237.i, 3, !dbg !8566
  %569 = or disjoint i64 %base.i1134.i, 7, !dbg !8571
  %or.cond.i1135.not.i = icmp ult i64 %569, %_54.1.i.i377.pre.i, !dbg !8571
  br i1 %or.cond.i1135.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, label %bb4.i1137.i, !dbg !8571, !prof !2740

bb4.i1137.i:                                      ; preds = %bb19.i.i430.i
  store <8 x float> %.lcssa21702829, ptr %416, align 1, !dbg !7625
  store <8 x float> %.lcssa21262839, ptr %407, align 1, !dbg !7651
  store <8 x float> %.lcssa21182848, ptr %_83.i365.i, align 1, !dbg !7654
  store <8 x float> %.lcssa21102858, ptr %408, align 1, !dbg !7655
  %_5.i.i = add i64 %base.i1134.i, 8, !dbg !8575
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1134.i, i64 noundef %_5.i.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i377.pre.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !8576, !noalias !8577
  unreachable, !dbg !8576

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i: ; preds = %bb19.i.i430.i
  %_30.i32.i.i = add nuw i64 %iter.sroa.0.0.i.i4278236.i, 1, !dbg !8581
  %_15.i.i = getelementptr inbounds nuw float, ptr %_54.0.i.i376.pre.i, i64 %base.i1134.i, !dbg !8592
  %lanes.i3123.sroa.0.0.copyload.i = load <8 x float>, ptr %_15.i.i, align 4, !dbg !8594, !alias.scope !8599, !noalias !8603
  %570 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %suffix.i.i12.sroa.0.08235.i, <8 x float> %lanes.i3123.sroa.0.0.copyload.i), !dbg !8607
  store <8 x float> %570, ptr %_15.i.i, align 4, !dbg !8612, !alias.scope !8618, !noalias !8622
  %571 = icmp eq i64 %end.sroa.0.0.i.i4288237.i, 0, !dbg !8626
  %spec.store.select.i.i432.i = select i1 %571, i64 %ring.i277.i, i64 %end.sroa.0.0.i.i4288237.i, !dbg !8626
  %572 = add i64 %spec.store.select.i.i432.i, -1, !dbg !8627
  %exitcond9630.not.i = icmp eq i64 %_30.i32.i.i, %_18.i.i388.i, !dbg !8628
  br i1 %exitcond9630.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i398.i, label %bb19.i.i430.i, !dbg !8631

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i398.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1173.i
  %minimum.i.i45.sroa.0.1.i = phi <8 x float> [ %567, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1173.i ], [ %minimum.i.i45.sroa.0.0.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ], !dbg !8526
  %storemerge.i.i399.i = phi i32 [ %568, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1173.i ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ], !dbg !8632
  %573 = fmul <8 x float> %minimum.i.i45.sroa.0.1.i, splat (float 1.638400e+04), !dbg !8633
  %574 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %573), !dbg !8638
  %575 = fmul <8 x float> %574, splat (float 0x3F10000000000000), !dbg !8643
  %base.i1156.i = shl i64 %_156.i375.i, 3, !dbg !8648
  %576 = or disjoint i64 %base.i1156.i, 7, !dbg !8650
  %or.cond.i1160.not.i = icmp ult i64 %576, %_56.1.i.i401.i, !dbg !8650
  br i1 %or.cond.i1160.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1164.i, label %bb4.i1163.i, !dbg !8650, !prof !2740

bb4.i1163.i:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i398.i
  store <8 x float> %.lcssa21702829, ptr %416, align 1, !dbg !7625
  store <8 x float> %.lcssa21262839, ptr %407, align 1, !dbg !7651
  store <8 x float> %.lcssa21182848, ptr %_83.i365.i, align 1, !dbg !7654
  store <8 x float> %.lcssa21102858, ptr %408, align 1, !dbg !7655
  %_5.i1157.i = add i64 %base.i1156.i, 8, !dbg !8654
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1156.i, i64 noundef %_5.i1157.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i401.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !8655, !noalias !8656
  unreachable, !dbg !8655

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1164.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i398.i
  %_15.i1162.i = getelementptr inbounds nuw float, ptr %_56.0.i.i400.i, i64 %base.i1156.i, !dbg !8660
  %lanes.i3102.sroa.0.0.copyload.i = load <8 x float>, ptr %_15.i1162.i, align 4, !dbg !8662, !alias.scope !8667, !noalias !8671
  %577 = fadd <8 x float> %536, %575, !dbg !8675
  %578 = fsub <8 x float> %577, %lanes.i3102.sroa.0.0.copyload.i, !dbg !8680
  %_8.not.i4.i.i408.i = icmp ugt i64 %_7.i10.i.i380.i, %_56.1.i.i401.i
  br i1 %_8.not.i4.i.i408.i, label %bb4.i7.i.i425.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i410.i, !dbg !8685, !prof !165

bb4.i7.i.i425.i:                                  ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1164.i
  store <8 x float> %.lcssa21702829, ptr %416, align 1, !dbg !7625
  store <8 x float> %.lcssa21262839, ptr %407, align 1, !dbg !7651
  store <8 x float> %.lcssa21182848, ptr %_83.i365.i, align 1, !dbg !7654
  store <8 x float> %.lcssa21102858, ptr %408, align 1, !dbg !7655
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i.i379.i, i64 noundef %_7.i10.i.i380.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i401.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !8690, !noalias !8691
  unreachable, !dbg !8690

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i410.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1164.i
  %_17.i6.i.i411.i = getelementptr inbounds nuw float, ptr %_56.0.i.i400.i, i64 %base.i9.i.i379.i, !dbg !8695
  store <8 x float> %575, ptr %_17.i6.i.i411.i, align 4, !dbg !8697, !alias.scope !8702, !noalias !8706
  %_41.i.i29.sroa.0.0.copyload.i = load <8 x float>, ptr %418, align 32, !dbg !8710, !noalias !8713
  %579 = fdiv <8 x float> %578, %_37.i.i33.sroa.0.0.copyload.i, !dbg !8714
  %580 = fsub <8 x float> splat (float 1.000000e+00), %579, !dbg !8719
  %581 = fsub <8 x float> %580, %_41.i.i29.sroa.0.0.copyload.i, !dbg !8724
  %582 = fmul <8 x float> %553, %581, !dbg !8729
  %583 = fadd <8 x float> %_41.i.i29.sroa.0.0.copyload.i, %582, !dbg !8734
  %584 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %580, <8 x float> %583), !dbg !8738
  %585 = bitcast <8 x float> %584 to <8 x i32>, !dbg !8744
  %586 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %584), !dbg !8751
  %587 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %586, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !8753
  %588 = bitcast <8 x float> %587 to <8 x i32>, !dbg !8759
  %589 = xor <8 x i32> %588, splat (i32 -1), !dbg !8765
  %590 = and <8 x i32> %589, %585, !dbg !8767
  store <8 x i32> %590, ptr %418, align 32, !dbg !8771, !noalias !8713
  %base.i1147.i = shl i64 %_153.i372.i, 3, !dbg !8772
  %591 = or disjoint i64 %base.i1147.i, 7, !dbg !8775
  %or.cond.i1151.not.i = icmp ult i64 %591, %_58.1.i.i413.i, !dbg !8775
  br i1 %or.cond.i1151.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i, label %bb4.i1154.i, !dbg !8775, !prof !2740

bb4.i1154.i:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i410.i
  store <8 x float> %.lcssa21702829, ptr %416, align 1, !dbg !7625
  store <8 x float> %.lcssa21262839, ptr %407, align 1, !dbg !7651
  store <8 x float> %.lcssa21182848, ptr %_83.i365.i, align 1, !dbg !7654
  store <8 x float> %.lcssa21102858, ptr %408, align 1, !dbg !7655
  %_5.i1148.i = add i64 %base.i1147.i, 8, !dbg !8779
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1147.i, i64 noundef %_5.i1148.i, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i.i413.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !8780, !noalias !8781
  unreachable, !dbg !8780

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i410.i
  %592 = bitcast <8 x i32> %590 to <8 x float>, !dbg !8785
  %593 = fsub <8 x float> splat (float 1.000000e+00), %592, !dbg !8786
  %_15.i1153.i = getelementptr inbounds nuw float, ptr %_58.0.i.i412.i, i64 %base.i1147.i, !dbg !8791
  %lanes.i3109.sroa.0.0.copyload.i = load <8 x float>, ptr %_15.i1153.i, align 4, !dbg !8793, !alias.scope !8798, !noalias !8802
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_15.i1153.i, ptr noundef nonnull align 4 dereferenceable(32) %data.i.i.i.i4010.i, i64 32, i1 false), !dbg !8806
  %594 = fmul <8 x float> %593, %lanes.i3109.sroa.0.0.copyload.i, !dbg !8813
  %595 = select <8 x i1> %422, <8 x float> %lanes.i3109.sroa.0.0.copyload.i, <8 x float> %594, !dbg !8818
  store <8 x float> %595, ptr %data.i.i.i.i4010.i, align 4, !dbg !8823, !alias.scope !8828, !noalias !8832
  %exitcond9641.not.i = icmp eq i64 %_9.0.i4013.i, %535, !dbg !8304
  br i1 %exitcond9641.not.i, label %bb25.i435.i, label %bb24.i359.i, !dbg !8304

bb25.i435.i:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i, %bb47.i344.i
  %.lcssa21102857 = phi <8 x float> [ %.lcssa21102858, %bb47.i344.i ], [ %554, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ]
  %.lcssa21182847 = phi <8 x float> [ %.lcssa21182848, %bb47.i344.i ], [ %553, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ]
  %.lcssa21262838 = phi <8 x float> [ %.lcssa21262839, %bb47.i344.i ], [ %548, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ]
  %.lcssa21702828 = phi <8 x float> [ %.lcssa21702829, %bb47.i344.i ], [ %578, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ]
  %.lcssa1082910873.i = phi <8 x float> [ %.lcssa1082910874.i, %bb47.i344.i ], [ %578, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ]
  %.lcssa1010610870.i = phi <8 x float> [ %.lcssa1010610871.i, %bb47.i344.i ], [ %554, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ]
  %.lcssa1011410867.i = phi <8 x float> [ %.lcssa1011410868.i, %bb47.i344.i ], [ %553, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ]
  %.lcssa1012210864.i = phi <8 x float> [ %.lcssa1012210865.i, %bb47.i344.i ], [ %548, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ]
  %.lcssa1013010853.i = phi <8 x float> [ %.lcssa1013010854.i, %bb47.i344.i ], [ %546, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ]
  %.lcssa1013810842.i = phi <8 x float> [ %.lcssa1013810843.i, %bb47.i344.i ], [ %545, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ]
  %.lcssa1014610831.i = phi <8 x float> [ %.lcssa1014610832.i, %bb47.i344.i ], [ %540, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ]
  %storemerge.i.i399.lcssa83568381.i = phi i32 [ %storemerge.i.i399.lcssa83568382.i, %bb47.i344.i ], [ %storemerge.i.i399.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ]
  %minimum.i.i45.sroa.0.08238.lcssa.i = phi <8 x float> [ %minimum.i.i45.sroa.0.08238.lcssa83628373.i, %bb47.i344.i ], [ %minimum.i.i45.sroa.0.0.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ]
  %_96.i436.i = add i64 %..i3979.i, %ring_cursor.sroa.0.1.i3308374.i, !dbg !8836
  %_151.not.i437.i = icmp ult i64 %_96.i436.i, %ring.i277.i, !dbg !8837
  %596 = select i1 %_151.not.i437.i, i64 0, i64 %ring.i277.i, !dbg !8837
  %ring_cursor.sroa.0.2.i438.i = sub nuw i64 %_96.i436.i, %596, !dbg !8837
  %_98.i439.i = add i64 %..i3979.i, %main_cursor.sroa.0.1.i3318375.i, !dbg !8840
  %_157.not.i440.i = icmp ult i64 %_98.i439.i, %main.i278.i, !dbg !8841
  %597 = select i1 %_157.not.i440.i, i64 0, i64 %main.i278.i, !dbg !8841
  %main_cursor.sroa.0.2.i441.i = sub nuw i64 %_98.i439.i, %597, !dbg !8841
  %_45.i333.i = icmp ult i64 %_66.i347.i, %..i3913.i, !dbg !7656
  br i1 %_45.i333.i, label %bb17.i334.i, label %bb16.i329.bb13.i284.loopexit_crit_edge.i, !dbg !7656

_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i: ; preds = %bb13.i284.loopexit.i
  store <8 x float> %minimum.i.i45.sroa.0.08238.lcssa8362.lcssa.i, ptr %uniform_left.i263.i, align 1, !noalias !6294
  %598 = trunc i64 %main_cursor.sroa.0.1.i331.lcssa.i to i32, !dbg !8843
  %599 = trunc i64 %ring_cursor.sroa.0.1.i330.lcssa.i to i32, !dbg !8845
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, !dbg !8846

_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i: ; preds = %bb4.i.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i_crit_edge, %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i
  %left_phase.i444.i = phi i32 [ %left_phase.i444.i.pre, %bb4.i.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i_crit_edge ], [ %storemerge.i.i399.lcssa83568381.lcssa10885.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i ], !dbg !7610
  %ring_cursor.sroa.0.0.i285.lcssa.i = phi i32 [ %_26.i280.i, %bb4.i.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i_crit_edge ], [ %599, %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i ], !dbg !7587
  %main_cursor.sroa.0.0.i286.lcssa.i = phi i32 [ %_25.i279.i, %bb4.i.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i_crit_edge ], [ %598, %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i ], !dbg !7583
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %left_prefix.i242.i, ptr noundef nonnull align 32 dereferenceable(32) %uniform_left.i263.i, i64 32, i1 false), !dbg !8846, !noalias !6294
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i263.i), !dbg !8847, !noalias !7565
  %600 = getelementptr inbounds nuw i8, ptr %self, i64 1736, !dbg !8848
  %_166.1.i446.i = load i64, ptr %600, align 8, !dbg !8848, !alias.scope !8849, !noalias !7568, !noundef !12
  %_8.i3592.i = icmp samesign ugt i64 %_166.1.i446.i, 7, !dbg !8850
  br i1 %_8.i3592.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3595.i, label %bb2.i3593.i, !dbg !8850, !prof !1421

bb2.i3593.i:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_166.1.i446.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !8855, !noalias !8856
  unreachable, !dbg !8855

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3595.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i
  %601 = getelementptr inbounds nuw i8, ptr %self, i64 1728, !dbg !8848
  %_166.0.i445.i = load ptr, ptr %601, align 8, !dbg !8848, !alias.scope !8849, !noalias !7568, !nonnull !12, !noundef !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_166.0.i445.i, ptr noundef nonnull align 32 dereferenceable(32) %left_prefix.i242.i, i64 32, i1 false), !dbg !8860, !noalias !5947
  %602 = getelementptr inbounds nuw i8, ptr %self, i64 1760, !dbg !8864
  %_167.0.i447.i = load ptr, ptr %602, align 8, !dbg !8864, !alias.scope !8849, !noalias !7568, !nonnull !12, !noundef !12
  %603 = getelementptr inbounds nuw i8, ptr %self, i64 1768, !dbg !8864
  %_167.1.i448.i = load i64, ptr %603, align 8, !dbg !8864, !alias.scope !8849, !noalias !7568, !noundef !12
  %604 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i444.i), !dbg !8865
  br i1 %604, label %bb2.i4033.i, label %bb6.i4028.i, !dbg !8865

bb6.i4028.i:                                      ; preds = %bb2.i4033.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3595.i
  %end_or_len.idx.i.i = shl nuw nsw i64 %_167.1.i448.i, 2, !dbg !8869
  %end_or_len.i.i = getelementptr inbounds nuw i8, ptr %_167.0.i447.i, i64 %end_or_len.idx.i.i, !dbg !8869
  %_293.i.i = icmp eq i64 %_167.1.i448.i, 0, !dbg !8873
  br i1 %_293.i.i, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit.i, label %bb10.i4029.i, !dbg !8876

bb2.i4033.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3595.i
  %bytes1.sroa.0.0.zext.i.i = and i32 %left_phase.i444.i, 255, !dbg !8877
  %bytes1.sroa.0.0.isplat.i.i = mul nuw i32 %bytes1.sroa.0.0.zext.i.i, 16843009, !dbg !8877
  %_5.i4034.i = icmp eq i32 %left_phase.i444.i, %bytes1.sroa.0.0.isplat.i.i, !dbg !8878
  br i1 %_5.i4034.i, label %bb3.i4035.i, label %bb6.i4028.i, !dbg !8878

bb3.i4035.i:                                      ; preds = %bb2.i4033.i
  %bytes.sroa.0.0.extract.trunc.i.i = trunc i32 %left_phase.i444.i to i8, !dbg !8879
  %605 = shl nuw nsw i64 %_167.1.i448.i, 2, !dbg !8881
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_167.0.i447.i, i8 %bytes.sroa.0.0.extract.trunc.i.i, i64 %605, i1 false), !dbg !8881, !alias.scope !8882, !noalias !7679
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit.i, !dbg !8885

bb10.i4029.i:                                     ; preds = %bb6.i4028.i, %bb10.i4029.i
  %iter.sroa.0.04.i.i = phi ptr [ %_38.i4030.i, %bb10.i4029.i ], [ %_167.0.i447.i, %bb6.i4028.i ]
  %_38.i4030.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i.i, i64 4, !dbg !8886
  store i32 %left_phase.i444.i, ptr %iter.sroa.0.04.i.i, align 4, !dbg !8888, !alias.scope !8882, !noalias !7679
  %_29.i4031.i = icmp eq ptr %_38.i4030.i, %end_or_len.i.i, !dbg !8873
  br i1 %_29.i4031.i, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit.i, label %bb10.i4029.i, !dbg !8876

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit.i: ; preds = %bb10.i4029.i, %bb3.i4035.i, %bb6.i4028.i
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_left.i270.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25) #31, !dbg !8889, !noalias !7679
  store i32 %main_cursor.sroa.0.0.i286.lcssa.i, ptr %_25.i, align 4, !dbg !8843, !alias.scope !7585, !noalias !7586
  store i32 %ring_cursor.sroa.0.0.i285.lcssa.i, ptr %360, align 4, !dbg !8845, !alias.scope !7585, !noalias !7586
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i264.i), !dbg !8890, !noalias !7565
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i270.i), !dbg !8891, !noalias !7565
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter18limiter_block_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, !dbg !7554

bb6.i.i:                                          ; preds = %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3848.i, %bb2.i.i.i3844.i, %bb13.i.i3837.i, %bb13.i5.i3860.i
  br i1 %_12.i.i3821.i, label %bb7.i.i, label %bb8.i.i, !dbg !8892

bb2.i.i:                                          ; preds = %bb1.i3.i3857.i, %bb2.i3851.i
  br i1 %_12.i.i3821.i, label %bb3.i.i, label %bb4.i.i, !dbg !8893

bb7.i.i:                                          ; preds = %bb6.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8894), !dbg !8897
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8898), !dbg !8897
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8900), !dbg !8897
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_left.i670.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_25) #31, !dbg !8902, !noalias !5947
  %606 = getelementptr inbounds nuw i8, ptr %self, i64 1536, !dbg !8906
  %607 = load i8, ptr %606, align 32, !dbg !8906, !range !17, !alias.scope !8910, !noalias !8911, !noundef !12
  %608 = getelementptr inbounds nuw i8, ptr %self, i64 1537, !dbg !8914
  %609 = load i8, ptr %608, align 1, !dbg !8914, !range !17, !alias.scope !8910, !noalias !8911, !noundef !12
  %_24.i.i = load i32, ptr %_25.i, align 4, !dbg !8916, !alias.scope !8918, !noalias !8919, !noundef !12
  %610 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !8920
  %_26.i677.i = load i32, ptr %610, align 4, !dbg !8920, !alias.scope !8918, !noalias !8919, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i.i), !dbg !8922, !noalias !8924
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i.i, i8 0, i64 32, i1 false), !noalias !8924
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i664.i), !dbg !8925, !noalias !8924
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i664.i, i8 0, i64 1024, i1 false), !noalias !8924
  %611 = add nuw nsw i64 %_31, 31, !dbg !8927
  %yield_count.sroa.0.0.i.i4039.i = lshr i64 %611, 5, !dbg !8927
  %hot_left.i670.promoted.i = load <8 x float>, ptr %hot_left.i670.i, align 1, !noalias !6294
  %_79.not.i8198.i = icmp eq i64 %yield_count.sroa.0.0.i.i4039.i, 0, !dbg !8934
  br i1 %_79.not.i8198.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, label %bb29.i.lr.ph.i, !dbg !8934

bb29.i.lr.ph.i:                                   ; preds = %bb7.i.i
  %612 = zext i32 %_26.i677.i to i64, !dbg !8920
  %613 = zext i32 %_24.i.i to i64, !dbg !8916
  %_22.i674.i = trunc nuw i8 %609 to i1, !dbg !8914
  %_21.i671.i = trunc nuw i8 %607 to i1, !dbg !8906
  %614 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !8943
  %615 = bitcast <8 x float> %614 to <8 x i32>, !dbg !8949
  %616 = xor <8 x i32> %615, splat (i32 -1), !dbg !8955
  %history.i.i639.sroa.10.0.hot_left.i670.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i670.i, i64 32
  %history.i.i639.sroa.13.0.hot_left.i670.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i670.i, i64 64
  %history.i.i639.sroa.16.0.hot_left.i670.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i670.i, i64 96
  %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i670.i, i64 128
  %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i670.i, i64 160
  %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i670.i, i64 192
  %history.i.i639.sroa.29.0.hot_left.i670.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i670.i, i64 224
  %history.i.i639.sroa.32.0.hot_left.i670.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i670.i, i64 256
  %history.i.i639.sroa.35.0.hot_left.i670.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i670.i, i64 288
  %history.i.i639.sroa.38.0.hot_left.i670.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i670.i, i64 320
  %history.i.i639.sroa.41.0.hot_left.i670.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i670.i, i64 352
  %617 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %618 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %619 = getelementptr inbounds nuw i8, ptr %self, i64 96
  %row12.i.i.i.i699.i = getelementptr inbounds nuw i8, ptr %self, i64 128
  %620 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %621 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %622 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %row13.i.i.i.i700.i = getelementptr inbounds nuw i8, ptr %self, i64 256
  %623 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %624 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %625 = getelementptr inbounds nuw i8, ptr %self, i64 352
  %row14.i.i.i.i701.i = getelementptr inbounds nuw i8, ptr %self, i64 384
  %626 = getelementptr inbounds nuw i8, ptr %self, i64 416
  %627 = getelementptr inbounds nuw i8, ptr %self, i64 448
  %628 = getelementptr inbounds nuw i8, ptr %self, i64 480
  %row15.i.i.i.i702.i = getelementptr inbounds nuw i8, ptr %self, i64 512
  %629 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %630 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %631 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %row16.i.i.i.i703.i = getelementptr inbounds nuw i8, ptr %self, i64 640
  %632 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %633 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %634 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %row17.i.i.i.i704.i = getelementptr inbounds nuw i8, ptr %self, i64 768
  %635 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %636 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %637 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %row18.i.i.i.i705.i = getelementptr inbounds nuw i8, ptr %self, i64 896
  %638 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %639 = getelementptr inbounds nuw i8, ptr %self, i64 960
  %640 = getelementptr inbounds nuw i8, ptr %self, i64 992
  %row19.i.i.i.i706.i = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %641 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %642 = getelementptr inbounds nuw i8, ptr %self, i64 1088
  %643 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %row20.i.i.i.i707.i = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %644 = getelementptr inbounds nuw i8, ptr %self, i64 1184
  %645 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %646 = getelementptr inbounds nuw i8, ptr %self, i64 1248
  %row21.i.i.i.i708.i = getelementptr inbounds nuw i8, ptr %self, i64 1280
  %647 = getelementptr inbounds nuw i8, ptr %self, i64 1312
  %648 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %649 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %row22.i.i.i.i709.i = getelementptr inbounds nuw i8, ptr %self, i64 1408
  %650 = getelementptr inbounds nuw i8, ptr %self, i64 1440
  %651 = getelementptr inbounds nuw i8, ptr %self, i64 1472
  %652 = getelementptr inbounds nuw i8, ptr %self, i64 1504
  %_51.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i670.i, i64 384
  %_52.i722.i = getelementptr inbounds nuw i8, ptr %hot_left.i670.i, i64 512
  %653 = select i1 %_21.i671.i, <8 x i32> %615, <8 x i32> %616
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
  %665 = getelementptr inbounds nuw i8, ptr %hot_left.i670.i, i64 672
  %666 = getelementptr inbounds nuw i8, ptr %hot_left.i670.i, i64 704
  %667 = getelementptr inbounds nuw i8, ptr %hot_left.i670.i, i64 640
  %668 = getelementptr inbounds nuw i8, ptr %self, i64 1672
  %669 = getelementptr inbounds nuw i8, ptr %self, i64 1664
  %670 = select i1 %_22.i674.i, <8 x i32> %615, <8 x i32> %616
  %671 = icmp slt <8 x i32> %670, zeroinitializer
  %672 = getelementptr inbounds nuw i8, ptr %self, i64 1632
  %_8.i.i656.sroa.0.0.copyload.i = load <8 x float>, ptr %_51.i.i, align 32, !noalias !6294
  %_9.i.i655.sroa.0.0.copyload.i = load <8 x float>, ptr %_52.i722.i, align 32, !noalias !6294
  %_64.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %666, align 32, !noalias !6294
  %iter.i.i645.sroa.0.0.ptr8131.1.i = getelementptr inbounds nuw i8, ptr %scratch.i.i, i64 4
  %iter.i.i645.sroa.0.0.ptr8131.2.i = getelementptr inbounds nuw i8, ptr %scratch.i.i, i64 8
  %iter.i.i645.sroa.0.0.ptr8131.3.i = getelementptr inbounds nuw i8, ptr %scratch.i.i, i64 12
  %iter.i.i645.sroa.0.0.ptr8131.4.i = getelementptr inbounds nuw i8, ptr %scratch.i.i, i64 16
  %iter.i.i645.sroa.0.0.ptr8131.5.i = getelementptr inbounds nuw i8, ptr %scratch.i.i, i64 20
  %iter.i.i645.sroa.0.0.ptr8131.6.i = getelementptr inbounds nuw i8, ptr %scratch.i.i, i64 24
  %iter.i.i645.sroa.0.0.ptr8131.7.i = getelementptr inbounds nuw i8, ptr %scratch.i.i, i64 28
  %history.i.i639.sroa.10.0.hot_left.i670.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i639.sroa.10.0.hot_left.i670.sroa_idx.i, align 32, !noalias !6294
  %history.i.i639.sroa.13.0.hot_left.i670.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i639.sroa.13.0.hot_left.i670.sroa_idx.i, align 32, !noalias !6294
  %history.i.i639.sroa.16.0.hot_left.i670.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i639.sroa.16.0.hot_left.i670.sroa_idx.i, align 32, !noalias !6294
  %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.i, align 32, !noalias !6294
  %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.i, align 32, !noalias !6294
  %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.i, align 32, !noalias !6294
  %history.i.i639.sroa.29.0.hot_left.i670.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i639.sroa.29.0.hot_left.i670.sroa_idx.i, align 32, !noalias !6294
  %history.i.i639.sroa.32.0.hot_left.i670.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i639.sroa.32.0.hot_left.i670.sroa_idx.i, align 32, !noalias !6294
  %history.i.i639.sroa.35.0.hot_left.i670.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i639.sroa.35.0.hot_left.i670.sroa_idx.i, align 32, !noalias !6294
  %history.i.i639.sroa.38.0.hot_left.i670.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i639.sroa.38.0.hot_left.i670.sroa_idx.i, align 32, !noalias !6294
  %history.i.i639.sroa.41.0.hot_left.i670.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i639.sroa.41.0.hot_left.i670.sroa_idx.i, align 32, !noalias !6294
  %.promoted10818.i = load <8 x float>, ptr %665, align 32, !noalias !6294
  %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.i.promoted = load <8 x float>, ptr %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.i, align 1
  %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.i.promoted = load <8 x float>, ptr %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.i, align 1
  %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.i.promoted = load <8 x float>, ptr %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.i, align 1
  br label %bb29.i.i, !dbg !8934

bb14.i.bb12.i680.loopexit_crit_edge.i:            ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3570.i
  store <8 x float> %816, ptr %665, align 32, !dbg !8957, !noalias !6294
  br label %bb12.i680.loopexit.i, !dbg !8970

bb12.i680.loopexit.i:                             ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i716.i, %bb14.i.bb12.i680.loopexit_crit_edge.i
  %.lcssa1035310819.i = phi <8 x float> [ %816, %bb14.i.bb12.i680.loopexit_crit_edge.i ], [ %.lcssa1035310820.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i716.i ]
  %ring_cursor.sroa.0.1.i718.lcssa.i = phi i64 [ %spec.store.select9.i.i, %bb14.i.bb12.i680.loopexit_crit_edge.i ], [ %ring_cursor.sroa.0.0.i6828202.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i716.i ], !dbg !8976
  %main_cursor.sroa.0.1.i719.lcssa.i = phi i64 [ %spec.store.select.i.i, %bb14.i.bb12.i680.loopexit_crit_edge.i ], [ %main_cursor.sroa.0.0.i6838203.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i716.i ], !dbg !8977
  %_79.not.i.i = icmp eq i64 %675, 0, !dbg !8934
  %indvars.iv.next9602.i = add nsw i64 %indvars.iv9601.i, -32, !dbg !8934
  br i1 %_79.not.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i, label %bb29.i.i, !dbg !8934

bb29.i.i:                                         ; preds = %bb12.i680.loopexit.i, %bb29.i.lr.ph.i
  %history.i.i639.sroa.25.sroa.0.0.lcssa.i2810 = phi <8 x float> [ %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.i.promoted, %bb29.i.lr.ph.i ], [ %history.i.i639.sroa.25.sroa.0.0.lcssa.i, %bb12.i680.loopexit.i ]
  %history.i.i639.sroa.22.sroa.0.0.lcssa.i2792 = phi <8 x float> [ %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.i.promoted, %bb29.i.lr.ph.i ], [ %history.i.i639.sroa.22.sroa.0.0.lcssa.i, %bb12.i680.loopexit.i ]
  %history.i.i639.sroa.19.sroa.0.0.lcssa.i2774 = phi <8 x float> [ %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.i.promoted, %bb29.i.lr.ph.i ], [ %history.i.i639.sroa.19.sroa.0.0.lcssa.i, %bb12.i680.loopexit.i ]
  %.lcssa1035310820.i = phi <8 x float> [ %.promoted10818.i, %bb29.i.lr.ph.i ], [ %.lcssa1035310819.i, %bb12.i680.loopexit.i ]
  %history.i.i639.sroa.41.sroa.0.0.lcssa10817.i = phi <8 x float> [ %history.i.i639.sroa.41.0.hot_left.i670.sroa_idx.promoted.i, %bb29.i.lr.ph.i ], [ %history.i.i639.sroa.41.sroa.0.0.lcssa.i, %bb12.i680.loopexit.i ]
  %history.i.i639.sroa.38.sroa.0.0.lcssa10816.i = phi <8 x float> [ %history.i.i639.sroa.38.0.hot_left.i670.sroa_idx.promoted.i, %bb29.i.lr.ph.i ], [ %history.i.i639.sroa.38.sroa.0.0.lcssa.i, %bb12.i680.loopexit.i ]
  %history.i.i639.sroa.35.sroa.0.0.lcssa10815.i = phi <8 x float> [ %history.i.i639.sroa.35.0.hot_left.i670.sroa_idx.promoted.i, %bb29.i.lr.ph.i ], [ %history.i.i639.sroa.35.sroa.0.0.lcssa.i, %bb12.i680.loopexit.i ]
  %history.i.i639.sroa.32.sroa.0.0.lcssa10814.i = phi <8 x float> [ %history.i.i639.sroa.32.0.hot_left.i670.sroa_idx.promoted.i, %bb29.i.lr.ph.i ], [ %history.i.i639.sroa.32.sroa.0.0.lcssa.i, %bb12.i680.loopexit.i ]
  %history.i.i639.sroa.29.sroa.0.0.lcssa10813.i = phi <8 x float> [ %history.i.i639.sroa.29.0.hot_left.i670.sroa_idx.promoted.i, %bb29.i.lr.ph.i ], [ %history.i.i639.sroa.29.sroa.0.0.lcssa.i, %bb12.i680.loopexit.i ]
  %history.i.i639.sroa.25.sroa.0.0.lcssa10812.i = phi <8 x float> [ %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.promoted.i, %bb29.i.lr.ph.i ], [ %history.i.i639.sroa.25.sroa.0.0.lcssa.i, %bb12.i680.loopexit.i ]
  %history.i.i639.sroa.22.sroa.0.0.lcssa10811.i = phi <8 x float> [ %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.promoted.i, %bb29.i.lr.ph.i ], [ %history.i.i639.sroa.22.sroa.0.0.lcssa.i, %bb12.i680.loopexit.i ]
  %history.i.i639.sroa.19.sroa.0.0.lcssa10810.i = phi <8 x float> [ %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.promoted.i, %bb29.i.lr.ph.i ], [ %history.i.i639.sroa.19.sroa.0.0.lcssa.i, %bb12.i680.loopexit.i ]
  %history.i.i639.sroa.16.sroa.0.0.lcssa10792.i = phi <8 x float> [ %history.i.i639.sroa.16.0.hot_left.i670.sroa_idx.promoted.i, %bb29.i.lr.ph.i ], [ %history.i.i639.sroa.16.sroa.0.0.lcssa.i, %bb12.i680.loopexit.i ]
  %history.i.i639.sroa.13.sroa.0.0.lcssa10774.i = phi <8 x float> [ %history.i.i639.sroa.13.0.hot_left.i670.sroa_idx.promoted.i, %bb29.i.lr.ph.i ], [ %history.i.i639.sroa.13.sroa.0.0.lcssa.i, %bb12.i680.loopexit.i ]
  %history.i.i639.sroa.10.sroa.0.0.lcssa10756.i = phi <8 x float> [ %history.i.i639.sroa.10.0.hot_left.i670.sroa_idx.promoted.i, %bb29.i.lr.ph.i ], [ %history.i.i639.sroa.10.sroa.0.0.lcssa.i, %bb12.i680.loopexit.i ]
  %indvars.iv9601.i = phi i64 [ %_31, %bb29.i.lr.ph.i ], [ %indvars.iv.next9602.i, %bb12.i680.loopexit.i ]
  %main_cursor.sroa.0.0.i6838203.i = phi i64 [ %613, %bb29.i.lr.ph.i ], [ %main_cursor.sroa.0.1.i719.lcssa.i, %bb12.i680.loopexit.i ]
  %ring_cursor.sroa.0.0.i6828202.i = phi i64 [ %612, %bb29.i.lr.ph.i ], [ %ring_cursor.sroa.0.1.i718.lcssa.i, %bb12.i680.loopexit.i ]
  %iter3.sroa.0.0.i6818201.i = phi i64 [ %yield_count.sroa.0.0.i.i4039.i, %bb29.i.lr.ph.i ], [ %675, %bb12.i680.loopexit.i ]
  %iter.sroa.0.0.i8200.i = phi i64 [ 0, %bb29.i.lr.ph.i ], [ %674, %bb12.i680.loopexit.i ]
  %history.i.i639.sroa.0.0.lcssa81788199.i = phi <8 x float> [ %hot_left.i670.promoted.i, %bb29.i.lr.ph.i ], [ %history.i.i639.sroa.0.0.lcssa.i, %bb12.i680.loopexit.i ]
  %673 = tail call i64 @llvm.umax.i64(i64 %indvars.iv9601.i, i64 1), !dbg !8978
  %umax9618.i = tail call i64 @llvm.umin.i64(i64 %673, i64 32), !dbg !8978
  %674 = add nuw nsw i64 %iter.sroa.0.0.i8200.i, 32, !dbg !8978
  %675 = add nsw i64 %iter3.sroa.0.0.i6818201.i, -1, !dbg !8982
  %_34.i685.i = sub nsw i64 %_31, %iter.sroa.0.0.i8200.i, !dbg !8983
  %..i4040.i = tail call noundef i64 @llvm.umin.i64(i64 %_34.i685.i, i64 32), !dbg !8984
  %active_base.i687.i = shl i64 %iter.sroa.0.0.i8200.i, 3, !dbg !8988
  %active_base.i6877053.i = add nuw nsw i64 %..i4040.i, %iter.sroa.0.0.i8200.i, !dbg !8989
  %_40.i689.i = shl i64 %active_base.i6877053.i, 3, !dbg !8989
  %_91.i.i = icmp samesign ult i64 %_40.i689.i, %active_base.i687.i, !dbg !8990
  %_85.not.i.i = icmp ugt i64 %_40.i689.i, %_39.1
  %or.cond.i690.i = or i1 %_91.i.i, %_85.not.i.i, !dbg !8990
  br i1 %or.cond.i690.i, label %bb33.i.i, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4070.i, !dbg !8990, !prof !165

bb33.i.i:                                         ; preds = %bb29.i.i
  store <8 x float> %history.i.i639.sroa.19.sroa.0.0.lcssa.i2774, ptr %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.22.sroa.0.0.lcssa.i2792, ptr %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.25.sroa.0.0.lcssa.i2810, ptr %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %active_base.i687.i, i64 noundef %_40.i689.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1c9f1dd5a676136ffd1db4032fbd14da) #30, !dbg !8999, !noalias !9000
  unreachable, !dbg !8999

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4070.i: ; preds = %bb29.i.i
  %_94.i693.i = getelementptr inbounds nuw float, ptr %_39.0, i64 %active_base.i687.i, !dbg !9001
  %_2.i40738096.not.i = icmp eq i64 %iter.sroa.0.0.i8200.i, %_31, !dbg !9005
  br i1 %_2.i40738096.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i716.i, label %bb6.i.i695.lr.ph.i, !dbg !9005

bb6.i.i695.lr.ph.i:                               ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4070.i
  %_5.i2669.i = load <8 x float>, ptr %self, align 32, !alias.scope !9008, !noalias !9011
  %_14.i.i.i.i600.sroa.0.0.copyload.i = load <8 x float>, ptr %617, align 32, !alias.scope !5946, !noalias !9023
  %_17.i.i.i.i597.sroa.0.0.copyload.i = load <8 x float>, ptr %618, align 32, !alias.scope !5946, !noalias !9023
  %_20.i.i.i.i594.sroa.0.0.copyload.i = load <8 x float>, ptr %619, align 32, !alias.scope !5946, !noalias !9023
  %_25.i.i.i.i590.sroa.0.0.copyload.i = load <8 x float>, ptr %row12.i.i.i.i699.i, align 32, !alias.scope !5946, !noalias !9023
  %_28.i.i.i.i587.sroa.0.0.copyload.i = load <8 x float>, ptr %620, align 32, !alias.scope !5946, !noalias !9023
  %_31.i.i.i.i584.sroa.0.0.copyload.i = load <8 x float>, ptr %621, align 32, !alias.scope !5946, !noalias !9023
  %_34.i.i.i.i581.sroa.0.0.copyload.i = load <8 x float>, ptr %622, align 32, !alias.scope !5946, !noalias !9023
  %_39.i.i.i.i577.sroa.0.0.copyload.i = load <8 x float>, ptr %row13.i.i.i.i700.i, align 32, !alias.scope !5946, !noalias !9023
  %_42.i.i.i.i574.sroa.0.0.copyload.i = load <8 x float>, ptr %623, align 32, !alias.scope !5946, !noalias !9023
  %_45.i.i.i.i571.sroa.0.0.copyload.i = load <8 x float>, ptr %624, align 32, !alias.scope !5946, !noalias !9023
  %_48.i.i.i.i568.sroa.0.0.copyload.i = load <8 x float>, ptr %625, align 32, !alias.scope !5946, !noalias !9023
  %_53.i.i.i.i564.sroa.0.0.copyload.i = load <8 x float>, ptr %row14.i.i.i.i701.i, align 32, !alias.scope !5946, !noalias !9023
  %_56.i.i.i.i561.sroa.0.0.copyload.i = load <8 x float>, ptr %626, align 32, !alias.scope !5946, !noalias !9023
  %_59.i.i.i.i558.sroa.0.0.copyload.i = load <8 x float>, ptr %627, align 32, !alias.scope !5946, !noalias !9023
  %_62.i.i.i.i555.sroa.0.0.copyload.i = load <8 x float>, ptr %628, align 32, !alias.scope !5946, !noalias !9023
  %_67.i.i.i.i551.sroa.0.0.copyload.i = load <8 x float>, ptr %row15.i.i.i.i702.i, align 32, !alias.scope !5946, !noalias !9023
  %_70.i.i.i.i548.sroa.0.0.copyload.i = load <8 x float>, ptr %629, align 32, !alias.scope !5946, !noalias !9023
  %_73.i.i.i.i545.sroa.0.0.copyload.i = load <8 x float>, ptr %630, align 32, !alias.scope !5946, !noalias !9023
  %_76.i.i.i.i542.sroa.0.0.copyload.i = load <8 x float>, ptr %631, align 32, !alias.scope !5946, !noalias !9023
  %_81.i.i.i.i538.sroa.0.0.copyload.i = load <8 x float>, ptr %row16.i.i.i.i703.i, align 32, !alias.scope !5946, !noalias !9023
  %_84.i.i.i.i535.sroa.0.0.copyload.i = load <8 x float>, ptr %632, align 32, !alias.scope !5946, !noalias !9023
  %_87.i.i.i.i532.sroa.0.0.copyload.i = load <8 x float>, ptr %633, align 32, !alias.scope !5946, !noalias !9023
  %_90.i.i.i.i529.sroa.0.0.copyload.i = load <8 x float>, ptr %634, align 32, !alias.scope !5946, !noalias !9023
  %_95.i.i.i.i525.sroa.0.0.copyload.i = load <8 x float>, ptr %row17.i.i.i.i704.i, align 32, !alias.scope !5946, !noalias !9023
  %_98.i.i.i.i522.sroa.0.0.copyload.i = load <8 x float>, ptr %635, align 32, !alias.scope !5946, !noalias !9023
  %_101.i.i.i.i519.sroa.0.0.copyload.i = load <8 x float>, ptr %636, align 32, !alias.scope !5946, !noalias !9023
  %_104.i.i.i.i516.sroa.0.0.copyload.i = load <8 x float>, ptr %637, align 32, !alias.scope !5946, !noalias !9023
  %_109.i.i.i.i512.sroa.0.0.copyload.i = load <8 x float>, ptr %row18.i.i.i.i705.i, align 32, !alias.scope !5946, !noalias !9023
  %_112.i.i.i.i509.sroa.0.0.copyload.i = load <8 x float>, ptr %638, align 32, !alias.scope !5946, !noalias !9023
  %_115.i.i.i.i506.sroa.0.0.copyload.i = load <8 x float>, ptr %639, align 32, !alias.scope !5946, !noalias !9023
  %_118.i.i.i.i503.sroa.0.0.copyload.i = load <8 x float>, ptr %640, align 32, !alias.scope !5946, !noalias !9023
  %_123.i.i.i.i499.sroa.0.0.copyload.i = load <8 x float>, ptr %row19.i.i.i.i706.i, align 32, !alias.scope !5946, !noalias !9023
  %_126.i.i.i.i496.sroa.0.0.copyload.i = load <8 x float>, ptr %641, align 32, !alias.scope !5946, !noalias !9023
  %_129.i.i.i.i493.sroa.0.0.copyload.i = load <8 x float>, ptr %642, align 32, !alias.scope !5946, !noalias !9023
  %_132.i.i.i.i490.sroa.0.0.copyload.i = load <8 x float>, ptr %643, align 32, !alias.scope !5946, !noalias !9023
  %_137.i.i.i.i486.sroa.0.0.copyload.i = load <8 x float>, ptr %row20.i.i.i.i707.i, align 32, !alias.scope !5946, !noalias !9023
  %_140.i.i.i.i483.sroa.0.0.copyload.i = load <8 x float>, ptr %644, align 32, !alias.scope !5946, !noalias !9023
  %_143.i.i.i.i480.sroa.0.0.copyload.i = load <8 x float>, ptr %645, align 32, !alias.scope !5946, !noalias !9023
  %_146.i.i.i.i477.sroa.0.0.copyload.i = load <8 x float>, ptr %646, align 32, !alias.scope !5946, !noalias !9023
  %_151.i.i.i.i473.sroa.0.0.copyload.i = load <8 x float>, ptr %row21.i.i.i.i708.i, align 32, !alias.scope !5946, !noalias !9023
  %_154.i.i.i.i470.sroa.0.0.copyload.i = load <8 x float>, ptr %647, align 32, !alias.scope !5946, !noalias !9023
  %_157.i.i.i.i467.sroa.0.0.copyload.i = load <8 x float>, ptr %648, align 32, !alias.scope !5946, !noalias !9023
  %_160.i.i.i.i464.sroa.0.0.copyload.i = load <8 x float>, ptr %649, align 32, !alias.scope !5946, !noalias !9023
  %_165.i.i.i.i460.sroa.0.0.copyload.i = load <8 x float>, ptr %row22.i.i.i.i709.i, align 32, !alias.scope !5946, !noalias !9023
  %_168.i.i.i.i457.sroa.0.0.copyload.i = load <8 x float>, ptr %650, align 32, !alias.scope !5946, !noalias !9023
  %_171.i.i.i.i454.sroa.0.0.copyload.i = load <8 x float>, ptr %651, align 32, !alias.scope !5946, !noalias !9023
  %_174.i.i.i.i451.sroa.0.0.copyload.i = load <8 x float>, ptr %652, align 32, !alias.scope !5946, !noalias !9023
  br label %bb6.i.i695.i, !dbg !9005

bb6.i.i695.i:                                     ; preds = %bb6.i.i695.i, %bb6.i.i695.lr.ph.i
  %history.i.i639.sroa.10.sroa.0.08108.i = phi <8 x float> [ %history.i.i639.sroa.10.sroa.0.0.lcssa10756.i, %bb6.i.i695.lr.ph.i ], [ %history.i.i639.sroa.0.08097.i, %bb6.i.i695.i ]
  %history.i.i639.sroa.13.sroa.0.08107.i = phi <8 x float> [ %history.i.i639.sroa.13.sroa.0.0.lcssa10774.i, %bb6.i.i695.lr.ph.i ], [ %history.i.i639.sroa.10.sroa.0.08108.i, %bb6.i.i695.i ]
  %history.i.i639.sroa.16.sroa.0.08106.i = phi <8 x float> [ %history.i.i639.sroa.16.sroa.0.0.lcssa10792.i, %bb6.i.i695.lr.ph.i ], [ %history.i.i639.sroa.13.sroa.0.08107.i, %bb6.i.i695.i ]
  %history.i.i639.sroa.19.sroa.0.08105.i = phi <8 x float> [ %history.i.i639.sroa.19.sroa.0.0.lcssa10810.i, %bb6.i.i695.lr.ph.i ], [ %history.i.i639.sroa.16.sroa.0.08106.i, %bb6.i.i695.i ]
  %history.i.i639.sroa.22.sroa.0.08104.i = phi <8 x float> [ %history.i.i639.sroa.22.sroa.0.0.lcssa10811.i, %bb6.i.i695.lr.ph.i ], [ %history.i.i639.sroa.19.sroa.0.08105.i, %bb6.i.i695.i ]
  %history.i.i639.sroa.38.sroa.0.08103.i = phi <8 x float> [ %history.i.i639.sroa.38.sroa.0.0.lcssa10816.i, %bb6.i.i695.lr.ph.i ], [ %history.i.i639.sroa.35.sroa.0.08102.i, %bb6.i.i695.i ]
  %history.i.i639.sroa.35.sroa.0.08102.i = phi <8 x float> [ %history.i.i639.sroa.35.sroa.0.0.lcssa10815.i, %bb6.i.i695.lr.ph.i ], [ %history.i.i639.sroa.32.sroa.0.08101.i, %bb6.i.i695.i ]
  %history.i.i639.sroa.32.sroa.0.08101.i = phi <8 x float> [ %history.i.i639.sroa.32.sroa.0.0.lcssa10814.i, %bb6.i.i695.lr.ph.i ], [ %history.i.i639.sroa.29.sroa.0.08100.i, %bb6.i.i695.i ]
  %history.i.i639.sroa.29.sroa.0.08100.i = phi <8 x float> [ %history.i.i639.sroa.29.sroa.0.0.lcssa10813.i, %bb6.i.i695.lr.ph.i ], [ %history.i.i639.sroa.25.sroa.0.08099.i, %bb6.i.i695.i ]
  %history.i.i639.sroa.25.sroa.0.08099.i = phi <8 x float> [ %history.i.i639.sroa.25.sroa.0.0.lcssa10812.i, %bb6.i.i695.lr.ph.i ], [ %history.i.i639.sroa.22.sroa.0.08104.i, %bb6.i.i695.i ]
  %iter.i19.i.sroa.16.08098.i = phi i64 [ 0, %bb6.i.i695.lr.ph.i ], [ %781, %bb6.i.i695.i ]
  %history.i.i639.sroa.0.08097.i = phi <8 x float> [ %history.i.i639.sroa.0.0.lcssa81788199.i, %bb6.i.i695.lr.ph.i ], [ %lanes.i3221.sroa.0.0.copyload.i, %bb6.i.i695.i ]
  %start1.i.i4079.i = shl i64 %iter.i19.i.sroa.16.08098.i, 3, !dbg !9026
  %data.i.i4080.i = getelementptr inbounds nuw float, ptr %_94.i693.i, i64 %start1.i.i4079.i, !dbg !9028
  %lanes.i3221.sroa.0.0.copyload.i = load <8 x float>, ptr %data.i.i4080.i, align 4, !dbg !9030, !alias.scope !9035, !noalias !9039
  %676 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i.i639.sroa.22.sroa.0.08104.i), !dbg !9044
  %677 = fmul <8 x float> %_5.i2669.i, %lanes.i3221.sroa.0.0.copyload.i, !dbg !9051
  %678 = fadd <8 x float> %677, zeroinitializer, !dbg !9057
  %679 = fmul <8 x float> %_14.i.i.i.i600.sroa.0.0.copyload.i, %lanes.i3221.sroa.0.0.copyload.i, !dbg !9062
  %680 = fadd <8 x float> %679, zeroinitializer, !dbg !9067
  %681 = fmul <8 x float> %_17.i.i.i.i597.sroa.0.0.copyload.i, %lanes.i3221.sroa.0.0.copyload.i, !dbg !9072
  %682 = fadd <8 x float> %681, zeroinitializer, !dbg !9077
  %683 = fmul <8 x float> %_20.i.i.i.i594.sroa.0.0.copyload.i, %lanes.i3221.sroa.0.0.copyload.i, !dbg !9082
  %684 = fadd <8 x float> %683, zeroinitializer, !dbg !9087
  %685 = fmul <8 x float> %_25.i.i.i.i590.sroa.0.0.copyload.i, %history.i.i639.sroa.0.08097.i, !dbg !9092
  %686 = fadd <8 x float> %685, %678, !dbg !9097
  %687 = fmul <8 x float> %_28.i.i.i.i587.sroa.0.0.copyload.i, %history.i.i639.sroa.0.08097.i, !dbg !9102
  %688 = fadd <8 x float> %687, %680, !dbg !9107
  %689 = fmul <8 x float> %_31.i.i.i.i584.sroa.0.0.copyload.i, %history.i.i639.sroa.0.08097.i, !dbg !9112
  %690 = fadd <8 x float> %689, %682, !dbg !9117
  %691 = fmul <8 x float> %_34.i.i.i.i581.sroa.0.0.copyload.i, %history.i.i639.sroa.0.08097.i, !dbg !9122
  %692 = fadd <8 x float> %691, %684, !dbg !9127
  %693 = fmul <8 x float> %_39.i.i.i.i577.sroa.0.0.copyload.i, %history.i.i639.sroa.10.sroa.0.08108.i, !dbg !9132
  %694 = fadd <8 x float> %693, %686, !dbg !9137
  %695 = fmul <8 x float> %_42.i.i.i.i574.sroa.0.0.copyload.i, %history.i.i639.sroa.10.sroa.0.08108.i, !dbg !9142
  %696 = fadd <8 x float> %695, %688, !dbg !9147
  %697 = fmul <8 x float> %_45.i.i.i.i571.sroa.0.0.copyload.i, %history.i.i639.sroa.10.sroa.0.08108.i, !dbg !9152
  %698 = fadd <8 x float> %697, %690, !dbg !9157
  %699 = fmul <8 x float> %_48.i.i.i.i568.sroa.0.0.copyload.i, %history.i.i639.sroa.10.sroa.0.08108.i, !dbg !9162
  %700 = fadd <8 x float> %699, %692, !dbg !9167
  %701 = fmul <8 x float> %_53.i.i.i.i564.sroa.0.0.copyload.i, %history.i.i639.sroa.13.sroa.0.08107.i, !dbg !9172
  %702 = fadd <8 x float> %701, %694, !dbg !9177
  %703 = fmul <8 x float> %_56.i.i.i.i561.sroa.0.0.copyload.i, %history.i.i639.sroa.13.sroa.0.08107.i, !dbg !9182
  %704 = fadd <8 x float> %703, %696, !dbg !9187
  %705 = fmul <8 x float> %_59.i.i.i.i558.sroa.0.0.copyload.i, %history.i.i639.sroa.13.sroa.0.08107.i, !dbg !9192
  %706 = fadd <8 x float> %705, %698, !dbg !9197
  %707 = fmul <8 x float> %_62.i.i.i.i555.sroa.0.0.copyload.i, %history.i.i639.sroa.13.sroa.0.08107.i, !dbg !9202
  %708 = fadd <8 x float> %707, %700, !dbg !9207
  %709 = fmul <8 x float> %_67.i.i.i.i551.sroa.0.0.copyload.i, %history.i.i639.sroa.16.sroa.0.08106.i, !dbg !9212
  %710 = fadd <8 x float> %709, %702, !dbg !9217
  %711 = fmul <8 x float> %_70.i.i.i.i548.sroa.0.0.copyload.i, %history.i.i639.sroa.16.sroa.0.08106.i, !dbg !9222
  %712 = fadd <8 x float> %711, %704, !dbg !9227
  %713 = fmul <8 x float> %_73.i.i.i.i545.sroa.0.0.copyload.i, %history.i.i639.sroa.16.sroa.0.08106.i, !dbg !9232
  %714 = fadd <8 x float> %713, %706, !dbg !9237
  %715 = fmul <8 x float> %_76.i.i.i.i542.sroa.0.0.copyload.i, %history.i.i639.sroa.16.sroa.0.08106.i, !dbg !9242
  %716 = fadd <8 x float> %715, %708, !dbg !9247
  %717 = fmul <8 x float> %_81.i.i.i.i538.sroa.0.0.copyload.i, %history.i.i639.sroa.19.sroa.0.08105.i, !dbg !9252
  %718 = fadd <8 x float> %717, %710, !dbg !9257
  %719 = fmul <8 x float> %_84.i.i.i.i535.sroa.0.0.copyload.i, %history.i.i639.sroa.19.sroa.0.08105.i, !dbg !9262
  %720 = fadd <8 x float> %719, %712, !dbg !9267
  %721 = fmul <8 x float> %_87.i.i.i.i532.sroa.0.0.copyload.i, %history.i.i639.sroa.19.sroa.0.08105.i, !dbg !9272
  %722 = fadd <8 x float> %721, %714, !dbg !9277
  %723 = fmul <8 x float> %_90.i.i.i.i529.sroa.0.0.copyload.i, %history.i.i639.sroa.19.sroa.0.08105.i, !dbg !9282
  %724 = fadd <8 x float> %723, %716, !dbg !9287
  %725 = fmul <8 x float> %_95.i.i.i.i525.sroa.0.0.copyload.i, %history.i.i639.sroa.22.sroa.0.08104.i, !dbg !9292
  %726 = fadd <8 x float> %725, %718, !dbg !9297
  %727 = fmul <8 x float> %_98.i.i.i.i522.sroa.0.0.copyload.i, %history.i.i639.sroa.22.sroa.0.08104.i, !dbg !9302
  %728 = fadd <8 x float> %727, %720, !dbg !9307
  %729 = fmul <8 x float> %_101.i.i.i.i519.sroa.0.0.copyload.i, %history.i.i639.sroa.22.sroa.0.08104.i, !dbg !9312
  %730 = fadd <8 x float> %729, %722, !dbg !9317
  %731 = fmul <8 x float> %_104.i.i.i.i516.sroa.0.0.copyload.i, %history.i.i639.sroa.22.sroa.0.08104.i, !dbg !9322
  %732 = fadd <8 x float> %731, %724, !dbg !9327
  %733 = fmul <8 x float> %_109.i.i.i.i512.sroa.0.0.copyload.i, %history.i.i639.sroa.25.sroa.0.08099.i, !dbg !9332
  %734 = fadd <8 x float> %733, %726, !dbg !9337
  %735 = fmul <8 x float> %_112.i.i.i.i509.sroa.0.0.copyload.i, %history.i.i639.sroa.25.sroa.0.08099.i, !dbg !9342
  %736 = fadd <8 x float> %735, %728, !dbg !9347
  %737 = fmul <8 x float> %_115.i.i.i.i506.sroa.0.0.copyload.i, %history.i.i639.sroa.25.sroa.0.08099.i, !dbg !9352
  %738 = fadd <8 x float> %737, %730, !dbg !9357
  %739 = fmul <8 x float> %_118.i.i.i.i503.sroa.0.0.copyload.i, %history.i.i639.sroa.25.sroa.0.08099.i, !dbg !9362
  %740 = fadd <8 x float> %739, %732, !dbg !9367
  %741 = fmul <8 x float> %_123.i.i.i.i499.sroa.0.0.copyload.i, %history.i.i639.sroa.29.sroa.0.08100.i, !dbg !9372
  %742 = fadd <8 x float> %741, %734, !dbg !9377
  %743 = fmul <8 x float> %_126.i.i.i.i496.sroa.0.0.copyload.i, %history.i.i639.sroa.29.sroa.0.08100.i, !dbg !9382
  %744 = fadd <8 x float> %743, %736, !dbg !9387
  %745 = fmul <8 x float> %_129.i.i.i.i493.sroa.0.0.copyload.i, %history.i.i639.sroa.29.sroa.0.08100.i, !dbg !9392
  %746 = fadd <8 x float> %745, %738, !dbg !9397
  %747 = fmul <8 x float> %_132.i.i.i.i490.sroa.0.0.copyload.i, %history.i.i639.sroa.29.sroa.0.08100.i, !dbg !9402
  %748 = fadd <8 x float> %747, %740, !dbg !9407
  %749 = fmul <8 x float> %_137.i.i.i.i486.sroa.0.0.copyload.i, %history.i.i639.sroa.32.sroa.0.08101.i, !dbg !9412
  %750 = fadd <8 x float> %749, %742, !dbg !9417
  %751 = fmul <8 x float> %_140.i.i.i.i483.sroa.0.0.copyload.i, %history.i.i639.sroa.32.sroa.0.08101.i, !dbg !9422
  %752 = fadd <8 x float> %751, %744, !dbg !9427
  %753 = fmul <8 x float> %_143.i.i.i.i480.sroa.0.0.copyload.i, %history.i.i639.sroa.32.sroa.0.08101.i, !dbg !9432
  %754 = fadd <8 x float> %753, %746, !dbg !9437
  %755 = fmul <8 x float> %_146.i.i.i.i477.sroa.0.0.copyload.i, %history.i.i639.sroa.32.sroa.0.08101.i, !dbg !9442
  %756 = fadd <8 x float> %755, %748, !dbg !9447
  %757 = fmul <8 x float> %_151.i.i.i.i473.sroa.0.0.copyload.i, %history.i.i639.sroa.35.sroa.0.08102.i, !dbg !9452
  %758 = fadd <8 x float> %757, %750, !dbg !9457
  %759 = fmul <8 x float> %_154.i.i.i.i470.sroa.0.0.copyload.i, %history.i.i639.sroa.35.sroa.0.08102.i, !dbg !9462
  %760 = fadd <8 x float> %759, %752, !dbg !9467
  %761 = fmul <8 x float> %_157.i.i.i.i467.sroa.0.0.copyload.i, %history.i.i639.sroa.35.sroa.0.08102.i, !dbg !9472
  %762 = fadd <8 x float> %761, %754, !dbg !9477
  %763 = fmul <8 x float> %_160.i.i.i.i464.sroa.0.0.copyload.i, %history.i.i639.sroa.35.sroa.0.08102.i, !dbg !9482
  %764 = fadd <8 x float> %763, %756, !dbg !9487
  %765 = fmul <8 x float> %_165.i.i.i.i460.sroa.0.0.copyload.i, %history.i.i639.sroa.38.sroa.0.08103.i, !dbg !9492
  %766 = fadd <8 x float> %765, %758, !dbg !9497
  %767 = fmul <8 x float> %_168.i.i.i.i457.sroa.0.0.copyload.i, %history.i.i639.sroa.38.sroa.0.08103.i, !dbg !9502
  %768 = fadd <8 x float> %767, %760, !dbg !9507
  %769 = fmul <8 x float> %_171.i.i.i.i454.sroa.0.0.copyload.i, %history.i.i639.sroa.38.sroa.0.08103.i, !dbg !9512
  %770 = fadd <8 x float> %769, %762, !dbg !9517
  %771 = fmul <8 x float> %_174.i.i.i.i451.sroa.0.0.copyload.i, %history.i.i639.sroa.38.sroa.0.08103.i, !dbg !9522
  %772 = fadd <8 x float> %771, %764, !dbg !9527
  %773 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %766), !dbg !9532
  %774 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %676, <8 x float> %773), !dbg !9538
  %775 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %768), !dbg !9532
  %776 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %774, <8 x float> %775), !dbg !9538
  %777 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %770), !dbg !9532
  %778 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %776, <8 x float> %777), !dbg !9538
  %779 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %772), !dbg !9532
  %780 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %778, <8 x float> %779), !dbg !9538
  %781 = add nuw nsw i64 %iter.i19.i.sroa.16.08098.i, 1, !dbg !9543
  %data.i4.i4084.i = getelementptr inbounds nuw float, ptr %peaks_left.i664.i, i64 %start1.i.i4079.i, !dbg !9544
  store <8 x float> %780, ptr %data.i4.i4084.i, align 4, !dbg !9547, !alias.scope !9552, !noalias !9556
  %exitcond9605.not.i = icmp eq i64 %781, %umax9618.i, !dbg !9005
  br i1 %exitcond9605.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i716.i, label %bb6.i.i695.i, !dbg !9005

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i716.i: ; preds = %bb6.i.i695.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4070.i
  %history.i.i639.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i639.sroa.0.0.lcssa81788199.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4070.i ], [ %lanes.i3221.sroa.0.0.copyload.i, %bb6.i.i695.i ], !dbg !9560
  %history.i.i639.sroa.25.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i639.sroa.25.sroa.0.0.lcssa10812.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4070.i ], [ %history.i.i639.sroa.22.sroa.0.08104.i, %bb6.i.i695.i ], !dbg !9560
  %history.i.i639.sroa.29.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i639.sroa.29.sroa.0.0.lcssa10813.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4070.i ], [ %history.i.i639.sroa.25.sroa.0.08099.i, %bb6.i.i695.i ], !dbg !9560
  %history.i.i639.sroa.32.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i639.sroa.32.sroa.0.0.lcssa10814.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4070.i ], [ %history.i.i639.sroa.29.sroa.0.08100.i, %bb6.i.i695.i ], !dbg !9560
  %history.i.i639.sroa.35.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i639.sroa.35.sroa.0.0.lcssa10815.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4070.i ], [ %history.i.i639.sroa.32.sroa.0.08101.i, %bb6.i.i695.i ], !dbg !9560
  %history.i.i639.sroa.38.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i639.sroa.38.sroa.0.0.lcssa10816.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4070.i ], [ %history.i.i639.sroa.35.sroa.0.08102.i, %bb6.i.i695.i ], !dbg !9560
  %history.i.i639.sroa.41.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i639.sroa.41.sroa.0.0.lcssa10817.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4070.i ], [ %history.i.i639.sroa.38.sroa.0.08103.i, %bb6.i.i695.i ], !dbg !9560
  %history.i.i639.sroa.22.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i639.sroa.22.sroa.0.0.lcssa10811.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4070.i ], [ %history.i.i639.sroa.19.sroa.0.08105.i, %bb6.i.i695.i ], !dbg !9560
  %history.i.i639.sroa.19.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i639.sroa.19.sroa.0.0.lcssa10810.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4070.i ], [ %history.i.i639.sroa.16.sroa.0.08106.i, %bb6.i.i695.i ], !dbg !9560
  %history.i.i639.sroa.16.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i639.sroa.16.sroa.0.0.lcssa10792.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4070.i ], [ %history.i.i639.sroa.13.sroa.0.08107.i, %bb6.i.i695.i ], !dbg !9560
  %history.i.i639.sroa.13.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i639.sroa.13.sroa.0.0.lcssa10774.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4070.i ], [ %history.i.i639.sroa.10.sroa.0.08108.i, %bb6.i.i695.i ], !dbg !9560
  %history.i.i639.sroa.10.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i639.sroa.10.sroa.0.0.lcssa10756.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4070.i ], [ %history.i.i639.sroa.0.08097.i, %bb6.i.i695.i ], !dbg !9560
  store <8 x float> %history.i.i639.sroa.29.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.29.0.hot_left.i670.sroa_idx.i, align 32, !dbg !8997, !noalias !6294
  store <8 x float> %history.i.i639.sroa.32.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.32.0.hot_left.i670.sroa_idx.i, align 32, !dbg !8997, !noalias !6294
  store <8 x float> %history.i.i639.sroa.35.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.35.0.hot_left.i670.sroa_idx.i, align 32, !dbg !8997, !noalias !6294
  store <8 x float> %history.i.i639.sroa.38.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.38.0.hot_left.i670.sroa_idx.i, align 32, !dbg !8997, !noalias !6294
  store <8 x float> %history.i.i639.sroa.41.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.41.0.hot_left.i670.sroa_idx.i, align 32, !dbg !8997, !noalias !6294
  br i1 %_2.i40738096.not.i, label %bb12.i680.loopexit.i, label %bb40.i720.lr.ph.i, !dbg !8970

bb40.i720.lr.ph.i:                                ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i716.i
  %_64.i.i = load i64, ptr %655, align 8, !alias.scope !5946, !noalias !5947
  %_71.i733.i = load i64, ptr %672, align 8, !alias.scope !5946, !noalias !5947
  br label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3219.i, !dbg !8970

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3219.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3570.i, %bb40.i720.lr.ph.i
  %_58.i.i.sroa.0.0.copyload8139.i = phi <8 x float> [ %.lcssa1035310820.i, %bb40.i720.lr.ph.i ], [ %816, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3570.i ]
  %main_cursor.sroa.0.1.i7198135.i = phi i64 [ %main_cursor.sroa.0.0.i6838203.i, %bb40.i720.lr.ph.i ], [ %spec.store.select.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3570.i ]
  %ring_cursor.sroa.0.1.i7188134.i = phi i64 [ %ring_cursor.sroa.0.0.i6828202.i, %bb40.i720.lr.ph.i ], [ %spec.store.select9.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3570.i ]
  %iter2.sroa.0.0.i7178133.i = phi i64 [ 0, %bb40.i720.lr.ph.i ], [ %782, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3570.i ]
  %782 = add nuw nsw i64 %iter2.sroa.0.0.i7178133.i, 1, !dbg !9561
  %_47.i.i = add nuw nsw i64 %iter2.sroa.0.0.i7178133.i, %iter.sroa.0.0.i8200.i, !dbg !9567
  %base.i721.i = shl i64 %_47.i.i, 3, !dbg !9567
  %_113.i.idx.i = shl i64 %iter2.sroa.0.0.i7178133.i, 5, !dbg !9568
  %_113.i.i = getelementptr inbounds nuw i8, ptr %peaks_left.i664.i, i64 %_113.i.idx.i, !dbg !9568
  %lanes.i3212.sroa.0.0.copyload.i = load <8 x float>, ptr %_113.i.i, align 4, !dbg !9579, !alias.scope !9584, !noalias !9588
  %783 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i3212.sroa.0.0.copyload.i, <8 x float> %lanes.i3212.sroa.0.0.copyload.i), !dbg !9592
  %784 = select <8 x i1> %654, <8 x float> %783, <8 x float> %lanes.i3212.sroa.0.0.copyload.i, !dbg !9597
  %_114.i725.i = icmp samesign ugt i64 %base.i721.i, %_39.1, !dbg !9602
  br i1 %_114.i725.i, label %bb44.i735.i, label %bb45.i.i, !dbg !9602, !prof !1406

bb45.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3219.i
  %_116.i.i = sub nuw nsw i64 %_39.1, %base.i721.i, !dbg !9606
  %_120.i.i = getelementptr inbounds nuw float, ptr %_39.0, i64 %base.i721.i, !dbg !9607
  %_8.i3206.i = icmp samesign ugt i64 %_116.i.i, 7, !dbg !9612
  br i1 %_8.i3206.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3210.i, label %bb2.i3207.i, !dbg !9612, !prof !1421

bb2.i3207.i:                                      ; preds = %bb45.i.i
  store <8 x float> %history.i.i639.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_116.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !9617, !noalias !9618
  unreachable, !dbg !9617

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3210.i: ; preds = %bb45.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9622), !dbg !9625
  %width.i.i.i = load i64, ptr %656, align 8, !dbg !9626, !alias.scope !9627, !noalias !9628, !noundef !12
  %785 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %784, <8 x float> %_8.i.i656.sroa.0.0.copyload.i, i8 30), !dbg !9637
  %786 = fdiv <8 x float> %_8.i.i656.sroa.0.0.copyload.i, %784, !dbg !9643
  %787 = bitcast <8 x float> %785 to <8 x i32>, !dbg !9648
  %788 = icmp slt <8 x i32> %787, zeroinitializer, !dbg !9652
  %789 = select <8 x i1> %788, <8 x float> %786, <8 x float> splat (float 1.000000e+00), !dbg !9652
  %_144.1.i.i.i = load i64, ptr %657, align 8, !dbg !9654, !alias.scope !9627, !noalias !9628, !noundef !12
  %_22.i.i726.i = mul i64 %width.i.i.i, %ring_cursor.sroa.0.1.i7188134.i, !dbg !9655
  %_92.i.i.i = icmp ugt i64 %_22.i.i726.i, %_144.1.i.i.i, !dbg !9656
  br i1 %_92.i.i.i, label %bb37.i.i.i, label %bb38.i.i.i, !dbg !9656, !prof !1406

bb38.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3210.i
  %_95.i.i.i = sub nuw i64 %_144.1.i.i.i, %_22.i.i726.i, !dbg !9659
  %_8.i3582.i = icmp samesign ugt i64 %_95.i.i.i, 7, !dbg !9660
  br i1 %_8.i3582.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3585.i, label %bb2.i3583.i, !dbg !9660, !prof !1421

bb2.i3583.i:                                      ; preds = %bb38.i.i.i
  store <8 x float> %history.i.i639.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_95.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !9665, !noalias !9666
  unreachable, !dbg !9665

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3585.i: ; preds = %bb38.i.i.i
  %_144.0.i.i.i = load ptr, ptr %658, align 8, !dbg !9654, !alias.scope !9627, !noalias !9628, !nonnull !12, !noundef !12
  %_99.i.i.i = getelementptr inbounds nuw float, ptr %_144.0.i.i.i, i64 %_22.i.i726.i, !dbg !9670
  store <8 x float> %789, ptr %_99.i.i.i, align 4, !dbg !9672, !alias.scope !9676, !noalias !9680
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9682), !dbg !9685
  %width.i1277.i = load i64, ptr %656, align 8, !dbg !9686, !alias.scope !9688, !noalias !9689, !noundef !12
  %790 = icmp eq i64 %width.i1277.i, 0, !dbg !9691
  br i1 %790, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1385.i, label %bb32.i1284.lr.ph.i, !dbg !9691

bb32.i1284.lr.ph.i:                               ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3585.i
  %_112.1.i1287.i = load i64, ptr %49, align 8, !alias.scope !9688, !noalias !9689, !noundef !12
  %_112.0.i1291.i = load ptr, ptr %48, align 8, !alias.scope !5946, !noalias !5947, !nonnull !12
  %791 = add i64 %ring_cursor.sroa.0.1.i7188134.i, 1
  %_23.not.i1298.i = icmp ult i64 %791, %_64.i.i
  %792 = select i1 %_23.not.i1298.i, i64 0, i64 %_64.i.i
  %start1.sroa.0.0.i1299.i = sub nuw i64 %791, %792
  %_114.1.i1302.i = load i64, ptr %657, align 8, !alias.scope !5946, !noalias !5947
  %_114.0.i1306.i = load ptr, ptr %658, align 8, !alias.scope !5946, !noalias !5947, !nonnull !12
  %_116.1.i1307.i = load i64, ptr %659, align 8, !alias.scope !5946, !noalias !5947
  %_116.0.i1311.i = load ptr, ptr %660, align 8, !alias.scope !5946, !noalias !5947, !nonnull !12
  %_118.1.i1315.i = load i64, ptr %661, align 8, !alias.scope !5946, !noalias !5947
  %_118.0.i1319.i = load ptr, ptr %662, align 8, !alias.scope !5946, !noalias !5947, !nonnull !12
  %_45.i1332.i = mul i64 %width.i1277.i, %start1.sroa.0.0.i1299.i
  br label %bb32.i1284.i, !dbg !9691

bb32.i1284.i:                                     ; preds = %bb31.i1347.i, %bb32.i1284.lr.ph.i
  %iter.i1276.sroa.10.08126.i = phi i64 [ %width.i1277.i, %bb32.i1284.lr.ph.i ], [ %793, %bb31.i1347.i ]
  %iter.i1276.sroa.7.08125.i = phi i64 [ 0, %bb32.i1284.lr.ph.i ], [ %_9.0.i4110.i, %bb31.i1347.i ]
  %iter.i1276.sroa.0.0.idx8124.i = phi i64 [ 0, %bb32.i1284.lr.ph.i ], [ %iter.i1276.sroa.0.0.add.i, %bb31.i1347.i ]
  %iter.i1276.sroa.0.0.ptr8127.i = getelementptr inbounds nuw i8, ptr %scratch.i.i, i64 %iter.i1276.sroa.0.0.idx8124.i, !dbg !9693
  %793 = add i64 %iter.i1276.sroa.10.08126.i, -1, !dbg !9693
  %_7.i.i4106.i = icmp eq i64 %iter.i1276.sroa.0.0.idx8124.i, 32, !dbg !9694
  br i1 %_7.i.i4106.i, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1385.i, label %bb3.i1286.i, !dbg !9698

bb3.i1286.i:                                      ; preds = %bb32.i1284.i
  %iter.i1276.sroa.0.0.add.i = add nuw nsw i64 %iter.i1276.sroa.0.0.idx8124.i, 4, !dbg !9699
  %_9.0.i4110.i = add nuw nsw i64 %iter.i1276.sroa.7.08125.i, 1, !dbg !9701
  %exitcond9609.not.i = icmp eq i64 %iter.i1276.sroa.7.08125.i, %_112.1.i1287.i, !dbg !9702
  br i1 %exitcond9609.not.i, label %panic.i1289.i, label %bb5.i1290.i, !dbg !9702

bb5.i1290.i:                                      ; preds = %bb3.i1286.i
  %794 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i1291.i, i64 %iter.i1276.sroa.7.08125.i, !dbg !9702
  %shape.i1292.i = load i32, ptr %794, align 4, !dbg !9702, !noalias !9703, !noundef !12
  %795 = getelementptr inbounds nuw i8, ptr %794, i64 4, !dbg !9702
  %shape3.i1293.i = load i32, ptr %795, align 4, !dbg !9702, !noalias !9703, !noundef !12
  %window.i1294.i = zext i32 %shape.i1292.i to i64, !dbg !9704
  %_19.i1295.i = zext i32 %shape3.i1293.i to i64, !dbg !9705
  %796 = add i64 %ring_cursor.sroa.0.1.i7188134.i, %_19.i1295.i, !dbg !9706
  %_20.not.i1296.i = icmp ult i64 %796, %_64.i.i, !dbg !9707
  %797 = select i1 %_20.not.i1296.i, i64 0, i64 %_64.i.i, !dbg !9707
  %spec.select.i1297.i = sub nuw i64 %796, %797, !dbg !9707
  %_27.i1300.i = mul i64 %spec.select.i1297.i, %width.i1277.i, !dbg !9708
  %_26.i1301.i = add i64 %_27.i1300.i, %iter.i1276.sroa.7.08125.i, !dbg !9708
  %_30.i1303.i = icmp ult i64 %_26.i1301.i, %_114.1.i1302.i, !dbg !9709
  br i1 %_30.i1303.i, label %bb12.i1305.i, label %panic5.i1304.i, !dbg !9709

panic.i1289.i:                                    ; preds = %bb3.i1286.i
  store <8 x float> %history.i.i639.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i1287.i, i64 noundef %_112.1.i1287.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4a8785a681d008a9bfd0cd82628ea9cb) #30, !dbg !9702, !noalias !9703
  unreachable, !dbg !9702

bb12.i1305.i:                                     ; preds = %bb5.i1290.i
  %798 = getelementptr inbounds nuw float, ptr %_114.0.i1306.i, i64 %_26.i1301.i, !dbg !9709
  %799 = load float, ptr %798, align 4, !dbg !9709, !noalias !9703, !noundef !12
  %exitcond9610.not.i = icmp eq i64 %iter.i1276.sroa.7.08125.i, %_116.1.i1307.i, !dbg !9710
  br i1 %exitcond9610.not.i, label %panic6.i1309.i, label %bb13.i1310.i, !dbg !9710

panic5.i1304.i:                                   ; preds = %bb5.i1290.i
  store <8 x float> %history.i.i639.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i1301.i, i64 noundef %_114.1.i1302.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cbce7773ac40979e4ba2385da3aec116) #30, !dbg !9709, !noalias !9703
  unreachable, !dbg !9709

bb13.i1310.i:                                     ; preds = %bb12.i1305.i
  %800 = getelementptr inbounds nuw i32, ptr %_116.0.i1311.i, i64 %iter.i1276.sroa.7.08125.i, !dbg !9710
  %_32.i1312.i = load i32, ptr %800, align 4, !dbg !9710, !noalias !9703, !noundef !12
  %position.i1313.i = zext i32 %_32.i1312.i to i64, !dbg !9710
  %801 = icmp eq i32 %_32.i1312.i, 0, !dbg !9711
  br i1 %801, label %bb17.i1322.i, label %bb15.i1314.i, !dbg !9711

panic6.i1309.i:                                   ; preds = %bb12.i1305.i
  store <8 x float> %history.i.i639.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i1307.i, i64 noundef %_116.1.i1307.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ec0d48f73ebfc2755df5cedaa60b5c0a) #30, !dbg !9710, !noalias !9703
  unreachable, !dbg !9710

bb15.i1314.i:                                     ; preds = %bb13.i1310.i
  %_37.i1316.i = icmp ult i64 %iter.i1276.sroa.7.08125.i, %_118.1.i1315.i, !dbg !9712
  br i1 %_37.i1316.i, label %bb16.i1318.i, label %panic7.i1317.i, !dbg !9712

bb17.i1322.i:                                     ; preds = %bb35.i1383.i, %bb16.i1318.i, %bb13.i1310.i
  %newest.sroa.0.0.i1323.i = phi float [ %799, %bb13.i1310.i ], [ %_35.i1320.i, %bb35.i1383.i ], [ %799, %bb16.i1318.i ], !dbg !9713
  %exitcond9611.not.i = icmp eq i64 %iter.i1276.sroa.7.08125.i, %_118.1.i1315.i, !dbg !9714
  br i1 %exitcond9611.not.i, label %panic8.i1326.i, label %bb18.i1327.i, !dbg !9714

bb16.i1318.i:                                     ; preds = %bb15.i1314.i
  %802 = getelementptr inbounds nuw float, ptr %_118.0.i1319.i, i64 %iter.i1276.sroa.7.08125.i, !dbg !9712
  %_35.i1320.i = load float, ptr %802, align 4, !dbg !9712, !noalias !9703, !noundef !12
  %_102.i1321.i = fcmp olt float %_35.i1320.i, %799, !dbg !9715
  br i1 %_102.i1321.i, label %bb35.i1383.i, label %bb17.i1322.i, !dbg !9715

panic7.i1317.i:                                   ; preds = %bb15.i1314.i
  store <8 x float> %history.i.i639.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i1276.sroa.7.08125.i, i64 noundef %_118.1.i1315.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2c461872bb652d4796cdcf89c28c82c8) #30, !dbg !9712, !noalias !9703
  unreachable, !dbg !9712

bb35.i1383.i:                                     ; preds = %bb16.i1318.i
  br label %bb17.i1322.i, !dbg !9717

bb18.i1327.i:                                     ; preds = %bb17.i1322.i
  %803 = getelementptr inbounds nuw float, ptr %_118.0.i1319.i, i64 %iter.i1276.sroa.7.08125.i, !dbg !9714
  store float %newest.sroa.0.0.i1323.i, ptr %803, align 4, !dbg !9714, !noalias !9703
  %_42.i1329.i = add nuw nsw i64 %position.i1313.i, 1, !dbg !9718
  %complete.i1330.i = icmp eq i64 %_42.i1329.i, %window.i1294.i, !dbg !9718
  br i1 %complete.i1330.i, label %bb22.i1352.i, label %bb20.i1331.i, !dbg !9719

panic8.i1326.i:                                   ; preds = %bb17.i1322.i
  store <8 x float> %history.i.i639.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i1315.i, i64 noundef %_118.1.i1315.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2b690e2c7763f11809942906fc2ca813) #30, !dbg !9714, !noalias !9703
  unreachable, !dbg !9714

bb20.i1331.i:                                     ; preds = %bb18.i1327.i
  %_44.i1333.i = add i64 %iter.i1276.sroa.7.08125.i, %_45.i1332.i, !dbg !9720
  %_47.i1335.i = icmp ult i64 %_44.i1333.i, %_114.1.i1302.i, !dbg !9721
  br i1 %_47.i1335.i, label %bb30.i1345.i, label %panic9.i1336.i, !dbg !9721

panic9.i1336.i:                                   ; preds = %bb20.i1331.i
  store <8 x float> %history.i.i639.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i1333.i, i64 noundef %_114.1.i1302.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9fa421ae81817f58fcfc4a3243223891) #30, !dbg !9721, !noalias !9703
  unreachable, !dbg !9721

bb30.i1345.i:                                     ; preds = %bb20.i1331.i
  %804 = getelementptr inbounds nuw float, ptr %_114.0.i1306.i, i64 %_44.i1333.i, !dbg !9721
  %_43.i1339.i = load float, ptr %804, align 4, !dbg !9721, !noalias !9703, !noundef !12
  %_103.i1340.i = fcmp olt float %_43.i1339.i, %newest.sroa.0.0.i1323.i, !dbg !9722
  %newest.sroa.0.1.i1341.i = select i1 %_103.i1340.i, float %_43.i1339.i, float %newest.sroa.0.0.i1323.i, !dbg !9722
  store float %newest.sroa.0.1.i1341.i, ptr %iter.i1276.sroa.0.0.ptr8127.i, align 4, !dbg !9724, !noalias !9725
  %805 = trunc i64 %_42.i1329.i to i32, !dbg !9726
  br label %bb31.i1347.i, !dbg !9727

bb31.i1347.i:                                     ; preds = %bb25.i1380.i, %bb30.i1345.i
  %storemerge7054.i = phi i32 [ %805, %bb30.i1345.i ], [ 0, %bb25.i1380.i ], !dbg !9728
  store i32 %storemerge7054.i, ptr %800, align 4, !dbg !9728, !noalias !9703
  %806 = icmp eq i64 %793, 0, !dbg !9691
  br i1 %806, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1385.i, label %bb32.i1284.i, !dbg !9691

bb22.i1352.i:                                     ; preds = %bb18.i1327.i
  store float %newest.sroa.0.0.i1323.i, ptr %iter.i1276.sroa.0.0.ptr8127.i, align 4, !dbg !9724, !noalias !9725
  %807 = load float, ptr %798, align 4, !dbg !9729, !noalias !9703, !noundef !12
  br label %bb41.i1365.i, !dbg !9730

bb41.i1365.i:                                     ; preds = %bb25.i1380.i, %bb22.i1352.i
  %iter2.sroa.0.0.i13578123.i = phi i64 [ 0, %bb22.i1352.i ], [ %_105.i1366.i, %bb25.i1380.i ]
  %suffix.sroa.0.0.i13568122.i = phi float [ %807, %bb22.i1352.i ], [ %suffix.sroa.0.1.i1376.i, %bb25.i1380.i ]
  %end.sroa.0.1.i13558121.i = phi i64 [ %spec.select.i1297.i, %bb22.i1352.i ], [ %810, %bb25.i1380.i ]
  %_56.i1367.i = mul i64 %end.sroa.0.1.i13558121.i, %width.i1277.i, !dbg !9733
  %_55.i1368.i = add i64 %_56.i1367.i, %iter.i1276.sroa.7.08125.i, !dbg !9733
  %_59.i1370.i = icmp ult i64 %_55.i1368.i, %_114.1.i1302.i, !dbg !9734
  br i1 %_59.i1370.i, label %bb25.i1380.i, label %panic13.i1371.i, !dbg !9734

panic13.i1371.i:                                  ; preds = %bb41.i1365.i
  store <8 x float> %history.i.i639.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i1368.i, i64 noundef %_114.1.i1302.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_91a4c6b9b17ebf4d863f9a70b6dc929a) #30, !dbg !9734, !noalias !9703
  unreachable, !dbg !9734

bb25.i1380.i:                                     ; preds = %bb41.i1365.i
  %_105.i1366.i = add nuw nsw i64 %iter2.sroa.0.0.i13578123.i, 1, !dbg !9735
  %808 = getelementptr inbounds nuw float, ptr %_114.0.i1306.i, i64 %_55.i1368.i, !dbg !9734
  %_54.i1374.i = load float, ptr %808, align 4, !dbg !9734, !noalias !9703, !noundef !12
  %_107.i1375.i = fcmp olt float %suffix.sroa.0.0.i13568122.i, %_54.i1374.i, !dbg !9738
  %suffix.sroa.0.1.i1376.i = select i1 %_107.i1375.i, float %suffix.sroa.0.0.i13568122.i, float %_54.i1374.i, !dbg !9738
  store float %suffix.sroa.0.1.i1376.i, ptr %808, align 4, !dbg !9740, !noalias !9703
  %809 = icmp eq i64 %end.sroa.0.1.i13558121.i, 0, !dbg !9741
  %spec.store.select.i1382.i = select i1 %809, i64 %_64.i.i, i64 %end.sroa.0.1.i13558121.i, !dbg !9741
  %810 = add i64 %spec.store.select.i1382.i, -1, !dbg !9742
  %exitcond9608.not.i = icmp eq i64 %_105.i1366.i, %window.i1294.i, !dbg !9743
  br i1 %exitcond9608.not.i, label %bb31.i1347.i, label %bb41.i1365.i, !dbg !9730

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1385.i: ; preds = %bb31.i1347.i, %bb32.i1284.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3585.i
  %lanes.i3196.sroa.0.0.copyload.i = load <8 x float>, ptr %scratch.i.i, align 4, !dbg !9745, !alias.scope !9750, !noalias !9754
  %811 = fmul <8 x float> %lanes.i3196.sroa.0.0.copyload.i, splat (float 1.638400e+04), !dbg !9758
  %812 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %811), !dbg !9763
  %813 = fmul <8 x float> %812, splat (float 0x3F10000000000000), !dbg !9768
  %814 = icmp eq i64 %width.i.i.i, 0, !dbg !9773
  %_149.1.i.i.pre.i = load i64, ptr %663, align 8, !dbg !9775, !alias.scope !9627, !noalias !9628
  br i1 %814, label %bb16.i.i.i, label %bb39.i.i.lr.ph.i, !dbg !9773

bb39.i.i.lr.ph.i:                                 ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1385.i
  %_145.1.i.i.i = load i64, ptr %49, align 8, !alias.scope !9627, !noalias !9628, !noundef !12
  %_145.0.i.i.i = load ptr, ptr %48, align 8, !alias.scope !5946, !noalias !5947, !nonnull !12
  %_147.0.i.i.i = load ptr, ptr %664, align 8, !alias.scope !5946, !noalias !5947, !nonnull !12
  %exitcond9614.not.i = icmp eq i64 %_145.1.i.i.i, 0, !dbg !9776
  br i1 %exitcond9614.not.i, label %panic.i.i.i, label %bb17.i.i.i, !dbg !9776

bb37.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3210.i
  store <8 x float> %history.i.i639.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i726.i, i64 noundef %_144.1.i.i.i, i64 noundef %_144.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_56df7c041d29359441bca272bf4e38e3) #30, !dbg !9777, !noalias !9778
  unreachable, !dbg !9777

bb16.i.i.loopexit.i:                              ; preds = %bb21.i.i.7.i, %bb21.i.i.6.i, %bb21.i.i.5.i, %bb21.i.i.4.i, %bb21.i.i.3.i, %bb21.i.i.2.i, %bb21.i.i.1.i, %bb21.i.i.i
  %lanes.i3189.sroa.0.0.copyload.pre.i = load <8 x float>, ptr %scratch.i.i, align 4, !dbg !9779, !alias.scope !9784, !noalias !9788
  br label %bb16.i.i.i, !dbg !9792

bb16.i.i.i:                                       ; preds = %bb16.i.i.loopexit.i, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1385.i
  %lanes.i3189.sroa.0.0.copyload.i = phi <8 x float> [ %lanes.i3189.sroa.0.0.copyload.pre.i, %bb16.i.i.loopexit.i ], [ %lanes.i3196.sroa.0.0.copyload.i, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1385.i ], !dbg !9779
  %815 = fadd <8 x float> %_58.i.i.sroa.0.0.copyload8139.i, %813, !dbg !9793
  %816 = fsub <8 x float> %815, %lanes.i3189.sroa.0.0.copyload.i, !dbg !9798
  %_109.i.i.i = icmp ugt i64 %_22.i.i726.i, %_149.1.i.i.pre.i, !dbg !9803
  br i1 %_109.i.i.i, label %bb42.i.i.i, label %bb43.i.i.i, !dbg !9803, !prof !1406

bb43.i.i.i:                                       ; preds = %bb16.i.i.i
  %_112.i.i.i = sub nuw i64 %_149.1.i.i.pre.i, %_22.i.i726.i, !dbg !9806
  %_8.i3577.i = icmp samesign ugt i64 %_112.i.i.i, 7, !dbg !9807
  br i1 %_8.i3577.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3580.i, label %bb2.i3578.i, !dbg !9807, !prof !1421

bb2.i3578.i:                                      ; preds = %bb43.i.i.i
  store <8 x float> %history.i.i639.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_112.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !9812, !noalias !9813
  unreachable, !dbg !9812

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3580.i: ; preds = %bb43.i.i.i
  %_149.0.i.i.i = load ptr, ptr %664, align 8, !dbg !9775, !alias.scope !9627, !noalias !9628, !nonnull !12, !noundef !12
  %_116.i.i.i = getelementptr inbounds nuw float, ptr %_149.0.i.i.i, i64 %_22.i.i726.i, !dbg !9817
  store <8 x float> %813, ptr %_116.i.i.i, align 4, !dbg !9819, !alias.scope !9823, !noalias !9827
  %_68.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %667, align 32, !dbg !9829, !noalias !6294
  %817 = fdiv <8 x float> %816, %_64.i.i.sroa.0.0.copyload.i, !dbg !9830
  %818 = fsub <8 x float> splat (float 1.000000e+00), %817, !dbg !9835
  %819 = fsub <8 x float> %818, %_68.i.i.sroa.0.0.copyload.i, !dbg !9840
  %820 = fmul <8 x float> %_9.i.i655.sroa.0.0.copyload.i, %819, !dbg !9845
  %821 = fadd <8 x float> %_68.i.i.sroa.0.0.copyload.i, %820, !dbg !9850
  %822 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %818, <8 x float> %821), !dbg !9854
  %823 = bitcast <8 x float> %822 to <8 x i32>, !dbg !9859
  %824 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %822), !dbg !9865
  %825 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %824, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !9867
  %826 = bitcast <8 x float> %825 to <8 x i32>, !dbg !9873
  %827 = xor <8 x i32> %826, splat (i32 -1), !dbg !9879
  %828 = and <8 x i32> %827, %823, !dbg !9881
  %829 = bitcast <8 x i32> %828 to <8 x float>, !dbg !9885
  store <8 x i32> %828, ptr %667, align 32, !dbg !9886, !noalias !6294
  %830 = fsub <8 x float> splat (float 1.000000e+00), %829, !dbg !9887
  %_150.1.i.i.i = load i64, ptr %668, align 8, !dbg !9892, !alias.scope !9627, !noalias !9628, !noundef !12
  %_76.i.i.i = mul i64 %width.i.i.i, %main_cursor.sroa.0.1.i7198135.i, !dbg !9893
  %_120.i.i.i = icmp ugt i64 %_76.i.i.i, %_150.1.i.i.i, !dbg !9894
  br i1 %_120.i.i.i, label %bb48.i.i.i, label %bb49.i.i.i, !dbg !9894, !prof !1406

bb42.i.i.i:                                       ; preds = %bb16.i.i.i
  store <8 x float> %history.i.i639.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i726.i, i64 noundef %_149.1.i.i.pre.i, i64 noundef %_149.1.i.i.pre.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_90498045d73339daaf9e4f537508f58b) #30, !dbg !9897, !noalias !9898
  unreachable, !dbg !9897

bb49.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3580.i
  %_123.i.i.i = sub nuw i64 %_150.1.i.i.i, %_76.i.i.i, !dbg !9899
  %_8.i3183.i = icmp samesign ugt i64 %_123.i.i.i, 7, !dbg !9900
  br i1 %_8.i3183.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3570.i, label %bb2.i3184.i, !dbg !9900, !prof !1421

bb2.i3184.i:                                      ; preds = %bb49.i.i.i
  store <8 x float> %history.i.i639.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_123.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !9905, !noalias !9906
  unreachable, !dbg !9905

bb48.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3580.i
  store <8 x float> %history.i.i639.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i.i.i, i64 noundef %_150.1.i.i.i, i64 noundef %_150.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_da8af254b6d507a8e2ca31e544bfd21d) #30, !dbg !9910, !noalias !9898
  unreachable, !dbg !9910

bb17.i.i.i:                                       ; preds = %bb39.i.i.lr.ph.i
  %831 = getelementptr inbounds nuw i8, ptr %_145.0.i.i.i, i64 8, !dbg !9776
  %_44.i.i729.i = load i32, ptr %831, align 4, !dbg !9776, !noalias !9898, !noundef !12
  %_43.i.i730.i = zext i32 %_44.i.i729.i to i64, !dbg !9776
  %832 = add i64 %ring_cursor.sroa.0.1.i7188134.i, %_43.i.i730.i, !dbg !9911
  %_47.not.i.i.i = icmp ult i64 %832, %_64.i.i, !dbg !9912
  %833 = select i1 %_47.not.i.i.i, i64 0, i64 %_64.i.i, !dbg !9912
  %spec.select.i.i.i = sub nuw i64 %832, %833, !dbg !9912
  %_51.i.i.i = mul i64 %spec.select.i.i.i, %width.i.i.i, !dbg !9913
  %_53.i.i731.i = icmp ult i64 %_51.i.i.i, %_149.1.i.i.pre.i, !dbg !9914
  br i1 %_53.i.i731.i, label %bb21.i.i.i, label %panic1.i.i.i, !dbg !9914

panic.i.i.i:                                      ; preds = %bb39.i.i.7.i, %bb39.i.i.6.i, %bb39.i.i.5.i, %bb39.i.i.4.i, %bb39.i.i.3.i, %bb39.i.i.2.i, %bb39.i.i.1.i, %bb39.i.i.lr.ph.i
  %_145.1.i.i.lcssa.ph.i = phi i64 [ 7, %bb39.i.i.7.i ], [ 6, %bb39.i.i.6.i ], [ 5, %bb39.i.i.5.i ], [ 4, %bb39.i.i.4.i ], [ 3, %bb39.i.i.3.i ], [ 2, %bb39.i.i.2.i ], [ 1, %bb39.i.i.1.i ], [ 0, %bb39.i.i.lr.ph.i ]
  store <8 x float> %history.i.i639.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i.i.lcssa.ph.i, i64 noundef %_145.1.i.i.lcssa.ph.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a6feb40b34112df5f214f84424dd2c7c) #30, !dbg !9776, !noalias !9898
  unreachable, !dbg !9776

bb21.i.i.i:                                       ; preds = %bb17.i.i.i
  %834 = getelementptr inbounds nuw float, ptr %_147.0.i.i.i, i64 %_51.i.i.i, !dbg !9914
  %_49.i.i732.i = load float, ptr %834, align 4, !dbg !9914, !noalias !9898, !noundef !12
  store float %_49.i.i732.i, ptr %scratch.i.i, align 4, !dbg !9915, !noalias !9916
  %835 = icmp eq i64 %width.i.i.i, 1, !dbg !9773
  br i1 %835, label %bb16.i.i.loopexit.i, label %bb39.i.i.1.i, !dbg !9773

bb39.i.i.1.i:                                     ; preds = %bb21.i.i.i
  %exitcond9614.1.not.i = icmp eq i64 %_145.1.i.i.i, 1, !dbg !9776
  br i1 %exitcond9614.1.not.i, label %panic.i.i.i, label %bb17.i.i.1.i, !dbg !9776

bb17.i.i.1.i:                                     ; preds = %bb39.i.i.1.i
  %836 = getelementptr inbounds nuw i8, ptr %_145.0.i.i.i, i64 20, !dbg !9776
  %_44.i.i729.1.i = load i32, ptr %836, align 4, !dbg !9776, !noalias !9898, !noundef !12
  %_43.i.i730.1.i = zext i32 %_44.i.i729.1.i to i64, !dbg !9776
  %837 = add i64 %ring_cursor.sroa.0.1.i7188134.i, %_43.i.i730.1.i, !dbg !9911
  %_47.not.i.i.1.i = icmp ult i64 %837, %_64.i.i, !dbg !9912
  %838 = select i1 %_47.not.i.i.1.i, i64 0, i64 %_64.i.i, !dbg !9912
  %spec.select.i.i.1.i = sub nuw i64 %837, %838, !dbg !9912
  %_51.i.i.1.i = mul i64 %spec.select.i.i.1.i, %width.i.i.i, !dbg !9913
  %_50.i.i.1.i = add i64 %_51.i.i.1.i, 1, !dbg !9913
  %_53.i.i731.1.i = icmp ult i64 %_50.i.i.1.i, %_149.1.i.i.pre.i, !dbg !9914
  br i1 %_53.i.i731.1.i, label %bb21.i.i.1.i, label %panic1.i.i.i, !dbg !9914

bb21.i.i.1.i:                                     ; preds = %bb17.i.i.1.i
  %839 = getelementptr inbounds nuw float, ptr %_147.0.i.i.i, i64 %_50.i.i.1.i, !dbg !9914
  %_49.i.i732.1.i = load float, ptr %839, align 4, !dbg !9914, !noalias !9898, !noundef !12
  store float %_49.i.i732.1.i, ptr %iter.i.i645.sroa.0.0.ptr8131.1.i, align 4, !dbg !9915, !noalias !9916
  %840 = icmp eq i64 %width.i.i.i, 2, !dbg !9773
  br i1 %840, label %bb16.i.i.loopexit.i, label %bb39.i.i.2.i, !dbg !9773

bb39.i.i.2.i:                                     ; preds = %bb21.i.i.1.i
  %exitcond9614.2.not.i = icmp eq i64 %_145.1.i.i.i, 2, !dbg !9776
  br i1 %exitcond9614.2.not.i, label %panic.i.i.i, label %bb17.i.i.2.i, !dbg !9776

bb17.i.i.2.i:                                     ; preds = %bb39.i.i.2.i
  %841 = getelementptr inbounds nuw i8, ptr %_145.0.i.i.i, i64 32, !dbg !9776
  %_44.i.i729.2.i = load i32, ptr %841, align 4, !dbg !9776, !noalias !9898, !noundef !12
  %_43.i.i730.2.i = zext i32 %_44.i.i729.2.i to i64, !dbg !9776
  %842 = add i64 %ring_cursor.sroa.0.1.i7188134.i, %_43.i.i730.2.i, !dbg !9911
  %_47.not.i.i.2.i = icmp ult i64 %842, %_64.i.i, !dbg !9912
  %843 = select i1 %_47.not.i.i.2.i, i64 0, i64 %_64.i.i, !dbg !9912
  %spec.select.i.i.2.i = sub nuw i64 %842, %843, !dbg !9912
  %_51.i.i.2.i = mul i64 %spec.select.i.i.2.i, %width.i.i.i, !dbg !9913
  %_50.i.i.2.i = add i64 %_51.i.i.2.i, 2, !dbg !9913
  %_53.i.i731.2.i = icmp ult i64 %_50.i.i.2.i, %_149.1.i.i.pre.i, !dbg !9914
  br i1 %_53.i.i731.2.i, label %bb21.i.i.2.i, label %panic1.i.i.i, !dbg !9914

bb21.i.i.2.i:                                     ; preds = %bb17.i.i.2.i
  %844 = getelementptr inbounds nuw float, ptr %_147.0.i.i.i, i64 %_50.i.i.2.i, !dbg !9914
  %_49.i.i732.2.i = load float, ptr %844, align 4, !dbg !9914, !noalias !9898, !noundef !12
  store float %_49.i.i732.2.i, ptr %iter.i.i645.sroa.0.0.ptr8131.2.i, align 4, !dbg !9915, !noalias !9916
  %845 = icmp eq i64 %width.i.i.i, 3, !dbg !9773
  br i1 %845, label %bb16.i.i.loopexit.i, label %bb39.i.i.3.i, !dbg !9773

bb39.i.i.3.i:                                     ; preds = %bb21.i.i.2.i
  %exitcond9614.3.not.i = icmp eq i64 %_145.1.i.i.i, 3, !dbg !9776
  br i1 %exitcond9614.3.not.i, label %panic.i.i.i, label %bb17.i.i.3.i, !dbg !9776

bb17.i.i.3.i:                                     ; preds = %bb39.i.i.3.i
  %846 = getelementptr inbounds nuw i8, ptr %_145.0.i.i.i, i64 44, !dbg !9776
  %_44.i.i729.3.i = load i32, ptr %846, align 4, !dbg !9776, !noalias !9898, !noundef !12
  %_43.i.i730.3.i = zext i32 %_44.i.i729.3.i to i64, !dbg !9776
  %847 = add i64 %ring_cursor.sroa.0.1.i7188134.i, %_43.i.i730.3.i, !dbg !9911
  %_47.not.i.i.3.i = icmp ult i64 %847, %_64.i.i, !dbg !9912
  %848 = select i1 %_47.not.i.i.3.i, i64 0, i64 %_64.i.i, !dbg !9912
  %spec.select.i.i.3.i = sub nuw i64 %847, %848, !dbg !9912
  %_51.i.i.3.i = mul i64 %spec.select.i.i.3.i, %width.i.i.i, !dbg !9913
  %_50.i.i.3.i = add i64 %_51.i.i.3.i, 3, !dbg !9913
  %_53.i.i731.3.i = icmp ult i64 %_50.i.i.3.i, %_149.1.i.i.pre.i, !dbg !9914
  br i1 %_53.i.i731.3.i, label %bb21.i.i.3.i, label %panic1.i.i.i, !dbg !9914

bb21.i.i.3.i:                                     ; preds = %bb17.i.i.3.i
  %849 = getelementptr inbounds nuw float, ptr %_147.0.i.i.i, i64 %_50.i.i.3.i, !dbg !9914
  %_49.i.i732.3.i = load float, ptr %849, align 4, !dbg !9914, !noalias !9898, !noundef !12
  store float %_49.i.i732.3.i, ptr %iter.i.i645.sroa.0.0.ptr8131.3.i, align 4, !dbg !9915, !noalias !9916
  %850 = icmp eq i64 %width.i.i.i, 4, !dbg !9773
  br i1 %850, label %bb16.i.i.loopexit.i, label %bb39.i.i.4.i, !dbg !9773

bb39.i.i.4.i:                                     ; preds = %bb21.i.i.3.i
  %exitcond9614.4.not.i = icmp eq i64 %_145.1.i.i.i, 4, !dbg !9776
  br i1 %exitcond9614.4.not.i, label %panic.i.i.i, label %bb17.i.i.4.i, !dbg !9776

bb17.i.i.4.i:                                     ; preds = %bb39.i.i.4.i
  %851 = getelementptr inbounds nuw i8, ptr %_145.0.i.i.i, i64 56, !dbg !9776
  %_44.i.i729.4.i = load i32, ptr %851, align 4, !dbg !9776, !noalias !9898, !noundef !12
  %_43.i.i730.4.i = zext i32 %_44.i.i729.4.i to i64, !dbg !9776
  %852 = add i64 %ring_cursor.sroa.0.1.i7188134.i, %_43.i.i730.4.i, !dbg !9911
  %_47.not.i.i.4.i = icmp ult i64 %852, %_64.i.i, !dbg !9912
  %853 = select i1 %_47.not.i.i.4.i, i64 0, i64 %_64.i.i, !dbg !9912
  %spec.select.i.i.4.i = sub nuw i64 %852, %853, !dbg !9912
  %_51.i.i.4.i = mul i64 %spec.select.i.i.4.i, %width.i.i.i, !dbg !9913
  %_50.i.i.4.i = add i64 %_51.i.i.4.i, 4, !dbg !9913
  %_53.i.i731.4.i = icmp ult i64 %_50.i.i.4.i, %_149.1.i.i.pre.i, !dbg !9914
  br i1 %_53.i.i731.4.i, label %bb21.i.i.4.i, label %panic1.i.i.i, !dbg !9914

bb21.i.i.4.i:                                     ; preds = %bb17.i.i.4.i
  %854 = getelementptr inbounds nuw float, ptr %_147.0.i.i.i, i64 %_50.i.i.4.i, !dbg !9914
  %_49.i.i732.4.i = load float, ptr %854, align 4, !dbg !9914, !noalias !9898, !noundef !12
  store float %_49.i.i732.4.i, ptr %iter.i.i645.sroa.0.0.ptr8131.4.i, align 4, !dbg !9915, !noalias !9916
  %855 = icmp eq i64 %width.i.i.i, 5, !dbg !9773
  br i1 %855, label %bb16.i.i.loopexit.i, label %bb39.i.i.5.i, !dbg !9773

bb39.i.i.5.i:                                     ; preds = %bb21.i.i.4.i
  %exitcond9614.5.not.i = icmp eq i64 %_145.1.i.i.i, 5, !dbg !9776
  br i1 %exitcond9614.5.not.i, label %panic.i.i.i, label %bb17.i.i.5.i, !dbg !9776

bb17.i.i.5.i:                                     ; preds = %bb39.i.i.5.i
  %856 = getelementptr inbounds nuw i8, ptr %_145.0.i.i.i, i64 68, !dbg !9776
  %_44.i.i729.5.i = load i32, ptr %856, align 4, !dbg !9776, !noalias !9898, !noundef !12
  %_43.i.i730.5.i = zext i32 %_44.i.i729.5.i to i64, !dbg !9776
  %857 = add i64 %ring_cursor.sroa.0.1.i7188134.i, %_43.i.i730.5.i, !dbg !9911
  %_47.not.i.i.5.i = icmp ult i64 %857, %_64.i.i, !dbg !9912
  %858 = select i1 %_47.not.i.i.5.i, i64 0, i64 %_64.i.i, !dbg !9912
  %spec.select.i.i.5.i = sub nuw i64 %857, %858, !dbg !9912
  %_51.i.i.5.i = mul i64 %spec.select.i.i.5.i, %width.i.i.i, !dbg !9913
  %_50.i.i.5.i = add i64 %_51.i.i.5.i, 5, !dbg !9913
  %_53.i.i731.5.i = icmp ult i64 %_50.i.i.5.i, %_149.1.i.i.pre.i, !dbg !9914
  br i1 %_53.i.i731.5.i, label %bb21.i.i.5.i, label %panic1.i.i.i, !dbg !9914

bb21.i.i.5.i:                                     ; preds = %bb17.i.i.5.i
  %859 = getelementptr inbounds nuw float, ptr %_147.0.i.i.i, i64 %_50.i.i.5.i, !dbg !9914
  %_49.i.i732.5.i = load float, ptr %859, align 4, !dbg !9914, !noalias !9898, !noundef !12
  store float %_49.i.i732.5.i, ptr %iter.i.i645.sroa.0.0.ptr8131.5.i, align 4, !dbg !9915, !noalias !9916
  %860 = icmp eq i64 %width.i.i.i, 6, !dbg !9773
  br i1 %860, label %bb16.i.i.loopexit.i, label %bb39.i.i.6.i, !dbg !9773

bb39.i.i.6.i:                                     ; preds = %bb21.i.i.5.i
  %exitcond9614.6.not.i = icmp eq i64 %_145.1.i.i.i, 6, !dbg !9776
  br i1 %exitcond9614.6.not.i, label %panic.i.i.i, label %bb17.i.i.6.i, !dbg !9776

bb17.i.i.6.i:                                     ; preds = %bb39.i.i.6.i
  %861 = getelementptr inbounds nuw i8, ptr %_145.0.i.i.i, i64 80, !dbg !9776
  %_44.i.i729.6.i = load i32, ptr %861, align 4, !dbg !9776, !noalias !9898, !noundef !12
  %_43.i.i730.6.i = zext i32 %_44.i.i729.6.i to i64, !dbg !9776
  %862 = add i64 %ring_cursor.sroa.0.1.i7188134.i, %_43.i.i730.6.i, !dbg !9911
  %_47.not.i.i.6.i = icmp ult i64 %862, %_64.i.i, !dbg !9912
  %863 = select i1 %_47.not.i.i.6.i, i64 0, i64 %_64.i.i, !dbg !9912
  %spec.select.i.i.6.i = sub nuw i64 %862, %863, !dbg !9912
  %_51.i.i.6.i = mul i64 %spec.select.i.i.6.i, %width.i.i.i, !dbg !9913
  %_50.i.i.6.i = add i64 %_51.i.i.6.i, 6, !dbg !9913
  %_53.i.i731.6.i = icmp ult i64 %_50.i.i.6.i, %_149.1.i.i.pre.i, !dbg !9914
  br i1 %_53.i.i731.6.i, label %bb21.i.i.6.i, label %panic1.i.i.i, !dbg !9914

bb21.i.i.6.i:                                     ; preds = %bb17.i.i.6.i
  %864 = getelementptr inbounds nuw float, ptr %_147.0.i.i.i, i64 %_50.i.i.6.i, !dbg !9914
  %_49.i.i732.6.i = load float, ptr %864, align 4, !dbg !9914, !noalias !9898, !noundef !12
  store float %_49.i.i732.6.i, ptr %iter.i.i645.sroa.0.0.ptr8131.6.i, align 4, !dbg !9915, !noalias !9916
  %865 = icmp eq i64 %width.i.i.i, 7, !dbg !9773
  br i1 %865, label %bb16.i.i.loopexit.i, label %bb39.i.i.7.i, !dbg !9773

bb39.i.i.7.i:                                     ; preds = %bb21.i.i.6.i
  %exitcond9614.7.not.i = icmp eq i64 %_145.1.i.i.i, 7, !dbg !9776
  br i1 %exitcond9614.7.not.i, label %panic.i.i.i, label %bb17.i.i.7.i, !dbg !9776

bb17.i.i.7.i:                                     ; preds = %bb39.i.i.7.i
  %866 = getelementptr inbounds nuw i8, ptr %_145.0.i.i.i, i64 92, !dbg !9776
  %_44.i.i729.7.i = load i32, ptr %866, align 4, !dbg !9776, !noalias !9898, !noundef !12
  %_43.i.i730.7.i = zext i32 %_44.i.i729.7.i to i64, !dbg !9776
  %867 = add i64 %ring_cursor.sroa.0.1.i7188134.i, %_43.i.i730.7.i, !dbg !9911
  %_47.not.i.i.7.i = icmp ult i64 %867, %_64.i.i, !dbg !9912
  %868 = select i1 %_47.not.i.i.7.i, i64 0, i64 %_64.i.i, !dbg !9912
  %spec.select.i.i.7.i = sub nuw i64 %867, %868, !dbg !9912
  %_51.i.i.7.i = mul i64 %spec.select.i.i.7.i, %width.i.i.i, !dbg !9913
  %_50.i.i.7.i = add i64 %_51.i.i.7.i, 7, !dbg !9913
  %_53.i.i731.7.i = icmp ult i64 %_50.i.i.7.i, %_149.1.i.i.pre.i, !dbg !9914
  br i1 %_53.i.i731.7.i, label %bb21.i.i.7.i, label %panic1.i.i.i, !dbg !9914

bb21.i.i.7.i:                                     ; preds = %bb17.i.i.7.i
  %869 = getelementptr inbounds nuw float, ptr %_147.0.i.i.i, i64 %_50.i.i.7.i, !dbg !9914
  %_49.i.i732.7.i = load float, ptr %869, align 4, !dbg !9914, !noalias !9898, !noundef !12
  store float %_49.i.i732.7.i, ptr %iter.i.i645.sroa.0.0.ptr8131.7.i, align 4, !dbg !9915, !noalias !9916
  br label %bb16.i.i.loopexit.i, !dbg !9773

panic1.i.i.i:                                     ; preds = %bb17.i.i.7.i, %bb17.i.i.6.i, %bb17.i.i.5.i, %bb17.i.i.4.i, %bb17.i.i.3.i, %bb17.i.i.2.i, %bb17.i.i.1.i, %bb17.i.i.i
  %_50.i.i.lcssa.ph.i = phi i64 [ %_50.i.i.7.i, %bb17.i.i.7.i ], [ %_50.i.i.6.i, %bb17.i.i.6.i ], [ %_50.i.i.5.i, %bb17.i.i.5.i ], [ %_50.i.i.4.i, %bb17.i.i.4.i ], [ %_50.i.i.3.i, %bb17.i.i.3.i ], [ %_50.i.i.2.i, %bb17.i.i.2.i ], [ %_50.i.i.1.i, %bb17.i.i.1.i ], [ %_51.i.i.i, %bb17.i.i.i ]
  store <8 x float> %history.i.i639.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i.i.lcssa.ph.i, i64 noundef %_149.1.i.i.pre.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b305c1483509cfb31fdec21ff8752674) #30, !dbg !9914, !noalias !9898
  unreachable, !dbg !9914

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3570.i: ; preds = %bb49.i.i.i
  %_150.0.i.i.i = load ptr, ptr %669, align 8, !dbg !9892, !alias.scope !9627, !noalias !9628, !nonnull !12, !noundef !12
  %_127.i.i.i = getelementptr inbounds nuw float, ptr %_150.0.i.i.i, i64 %_76.i.i.i, !dbg !9917
  %lanes.i3180.sroa.0.0.copyload.i = load <8 x float>, ptr %_127.i.i.i, align 4, !dbg !9919, !alias.scope !9923, !noalias !9927
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_127.i.i.i, ptr noundef nonnull align 4 dereferenceable(32) %_120.i.i, i64 32, i1 false), !dbg !9929
  %870 = fmul <8 x float> %830, %lanes.i3180.sroa.0.0.copyload.i, !dbg !9934
  %871 = select <8 x i1> %671, <8 x float> %lanes.i3180.sroa.0.0.copyload.i, <8 x float> %870, !dbg !9939
  store <8 x float> %871, ptr %_120.i.i, align 4, !dbg !9944, !alias.scope !9949, !noalias !9953
  %872 = add i64 %main_cursor.sroa.0.1.i7198135.i, 1, !dbg !9957
  %_69.i734.i = icmp eq i64 %872, %_71.i733.i, !dbg !9958
  %spec.store.select.i.i = select i1 %_69.i734.i, i64 0, i64 %872, !dbg !9958
  %873 = add i64 %ring_cursor.sroa.0.1.i7188134.i, 1, !dbg !9959
  %_72.i.i = icmp eq i64 %873, %_64.i.i, !dbg !9960
  %spec.store.select9.i.i = select i1 %_72.i.i, i64 0, i64 %873, !dbg !9960
  %exitcond9619.not.i = icmp eq i64 %782, %umax9618.i, !dbg !9961
  br i1 %exitcond9619.not.i, label %bb14.i.bb12.i680.loopexit_crit_edge.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3219.i, !dbg !8970

bb44.i735.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3219.i
  store <8 x float> %history.i.i639.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i721.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1, i64 noundef range(i64 0, 2305843009213693952) %_39.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_754b353a9c273ca837480913d0661c80) #30, !dbg !9964, !noalias !9000
  unreachable, !dbg !9964

_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i: ; preds = %bb12.i680.loopexit.i
  store <8 x float> %history.i.i639.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.19.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.22.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.25.0.hot_left.i670.sroa_idx.i, align 1, !dbg !8997
  store <8 x float> %history.i.i639.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.10.0.hot_left.i670.sroa_idx.i, align 32, !dbg !9560, !noalias !6294
  store <8 x float> %history.i.i639.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.13.0.hot_left.i670.sroa_idx.i, align 32, !dbg !9560, !noalias !6294
  store <8 x float> %history.i.i639.sroa.16.sroa.0.0.lcssa.i, ptr %history.i.i639.sroa.16.0.hot_left.i670.sroa_idx.i, align 32, !dbg !9560, !noalias !6294
  %874 = trunc i64 %main_cursor.sroa.0.1.i719.lcssa.i to i32, !dbg !9965
  %875 = trunc i64 %ring_cursor.sroa.0.1.i718.lcssa.i to i32, !dbg !9966
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, !dbg !9560

_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i, %bb7.i.i
  %history.i.i639.sroa.0.0.lcssa8178.lcssa.i = phi <8 x float> [ %hot_left.i670.promoted.i, %bb7.i.i ], [ %history.i.i639.sroa.0.0.lcssa.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i ]
  %ring_cursor.sroa.0.0.i682.lcssa.i = phi i32 [ %_26.i677.i, %bb7.i.i ], [ %875, %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i ], !dbg !8920
  %main_cursor.sroa.0.0.i683.lcssa.i = phi i32 [ %_24.i.i, %bb7.i.i ], [ %874, %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i ], !dbg !8916
  store <8 x float> %history.i.i639.sroa.0.0.lcssa8178.lcssa.i, ptr %hot_left.i670.i, align 1, !dbg !9560, !noalias !6294
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_left.i670.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25) #31, !dbg !9967, !noalias !5947
  store i32 %main_cursor.sroa.0.0.i683.lcssa.i, ptr %_25.i, align 4, !dbg !9965, !alias.scope !8918, !noalias !8919
  store i32 %ring_cursor.sroa.0.0.i682.lcssa.i, ptr %610, align 4, !dbg !9966, !alias.scope !8918, !noalias !8919
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i664.i), !dbg !9968, !noalias !8924
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i.i), !dbg !9969, !noalias !8924
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter18limiter_block_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, !dbg !8897

bb3.i.i:                                          ; preds = %bb2.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9970), !dbg !9973
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9974), !dbg !9973
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9976), !dbg !9973
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9978), !dbg !9973
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_left.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_25) #31, !dbg !9980, !noalias !5947
  %876 = getelementptr inbounds nuw i8, ptr %self, i64 1536, !dbg !9984
  %877 = load i8, ptr %876, align 32, !dbg !9984, !range !17, !alias.scope !9988, !noalias !9989, !noundef !12
  %878 = getelementptr inbounds nuw i8, ptr %self, i64 1537, !dbg !9991
  %879 = load i8, ptr %878, align 1, !dbg !9991, !range !17, !alias.scope !9988, !noalias !9989, !noundef !12
  %880 = getelementptr inbounds nuw i8, ptr %self, i64 1624, !dbg !9993
  %ring.i.i = load i64, ptr %880, align 8, !dbg !9993, !alias.scope !9995, !noalias !9996, !noundef !12
  %881 = getelementptr inbounds nuw i8, ptr %self, i64 1632, !dbg !9997
  %main.i.i = load i64, ptr %881, align 8, !dbg !9997, !alias.scope !9995, !noalias !9996, !noundef !12
  %_25.i.i = load i32, ptr %_25.i, align 4, !dbg !9999, !alias.scope !10001, !noalias !10002, !noundef !12
  %882 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !10003
  %_26.i.i = load i32, ptr %882, align 4, !dbg !10003, !alias.scope !10001, !noalias !10002, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i.i), !dbg !10005, !noalias !10007
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i.i, i8 0, i64 1024, i1 false), !noalias !10007
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i.i), !dbg !10008, !noalias !10007
; call <true_peak_limiter::UniformHot<wide::f32x8_::f32x8>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_(ptr noalias noundef align 32 captures(none) dereferenceable(128) %uniform_left.i.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25, i64 %ring.i.i, i64 %main.i.i) #31, !dbg !10010, !noalias !5947
  %883 = add nuw nsw i64 %_31, 31, !dbg !10011
  %yield_count.sroa.0.0.i.i4135.i = lshr i64 %883, 5, !dbg !10011
  %hot_left.i.promoted.i = load <8 x float>, ptr %hot_left.i.i, align 1, !noalias !6294
  %uniform_left.i.promoted.i = load <8 x float>, ptr %uniform_left.i.i, align 1, !noalias !6294
  %_111.not.i8559.i = icmp eq i64 %yield_count.sroa.0.0.i.i4135.i, 0, !dbg !10018
  br i1 %_111.not.i8559.i, label %bb3.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i, label %bb36.i.lr.ph.i, !dbg !10018

bb3.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i: ; preds = %bb3.i.i
  %.phi.trans.insert9927.i = getelementptr inbounds nuw i8, ptr %uniform_left.i.i, i64 104
  %left_phase.i.pre.i = load i32, ptr %.phi.trans.insert9927.i, align 8, !dbg !10027, !noalias !10007
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, !dbg !10018

bb36.i.lr.ph.i:                                   ; preds = %bb3.i.i
  %884 = zext i32 %_26.i.i to i64, !dbg !10003
  %885 = zext i32 %_25.i.i to i64, !dbg !9999
  %_22.i.i = trunc nuw i8 %879 to i1, !dbg !9991
  %_21.i.i = trunc nuw i8 %877 to i1, !dbg !9984
  %886 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !10028
  %887 = bitcast <8 x float> %886 to <8 x i32>, !dbg !10034
  %888 = xor <8 x i32> %887, splat (i32 -1), !dbg !10040
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
  %_52.i.sroa.3.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %uniform_left.i.i, i64 88
  %_52.i.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %uniform_left.i.i, i64 96
  %_82.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i, i64 384
  %_83.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i, i64 512
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
  %history.i.i.sroa.10.0.hot_left.i.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i.sroa.10.0.hot_left.i.sroa_idx.i, align 32, !noalias !6294
  %history.i.i.sroa.13.0.hot_left.i.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i.sroa.13.0.hot_left.i.sroa_idx.i, align 32, !noalias !6294
  %history.i.i.sroa.16.0.hot_left.i.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i.sroa.16.0.hot_left.i.sroa_idx.i, align 32, !noalias !6294
  %history.i.i.sroa.19.0.hot_left.i.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i.sroa.19.0.hot_left.i.sroa_idx.i, align 32, !noalias !6294
  %history.i.i.sroa.22.0.hot_left.i.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i.sroa.22.0.hot_left.i.sroa_idx.i, align 32, !noalias !6294
  %history.i.i.sroa.25.0.hot_left.i.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i.sroa.25.0.hot_left.i.sroa_idx.i, align 32, !noalias !6294
  %history.i.i.sroa.29.0.hot_left.i.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 32, !noalias !6294
  %history.i.i.sroa.32.0.hot_left.i.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 32, !noalias !6294
  %history.i.i.sroa.35.0.hot_left.i.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 32, !noalias !6294
  %history.i.i.sroa.38.0.hot_left.i.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 32, !noalias !6294
  %history.i.i.sroa.41.0.hot_left.i.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 32, !noalias !6294
  %_22.i.i.promoted.i = load i32, ptr %_22.i.i.i, align 4, !noalias !6294
  %.promoted8601.i = load <8 x float>, ptr %932, align 32, !noalias !6294
  %_52.i.sroa.3.0.copyload.i = load i64, ptr %_52.i.sroa.3.0..sroa_idx.i, align 8, !noalias !6294
  %_52.i.sroa.4.0.copyload.i = load i64, ptr %_52.i.sroa.4.0..sroa_idx.i, align 16, !noalias !6294
  %_8.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %_82.i.i, align 32, !noalias !6294
  %_9.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %_83.i.i, align 32, !noalias !6294
  %_54.0.i.i.i = load ptr, ptr %928, align 32, !noalias !6294, !nonnull !12, !align !24
  %_54.1.i.i.i = load i64, ptr %929, align 8, !noalias !6294
  %_18.i.i.i = load i64, ptr %925, align 16, !noalias !6294
  %_56.0.i.i.i = load ptr, ptr %930, align 16, !noalias !6294, !nonnull !12, !align !24
  %_56.1.i.i.i = load i64, ptr %931, align 8, !noalias !6294
  %_37.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %933, align 32, !noalias !6294
  %_58.1.i.i.i = load i64, ptr %935, align 8, !noalias !6294
  %_58.0.i.i.i = load ptr, ptr %936, align 32, !noalias !6294, !nonnull !12, !align !24
  %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i.promoted = load <8 x float>, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 1
  %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i.promoted = load <8 x float>, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 1
  %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i.promoted = load <8 x float>, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 1
  %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i.promoted = load <8 x float>, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 1
  %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i.promoted = load <8 x float>, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 1
  br label %bb36.i.i, !dbg !10018

bb16.i.bb13.i7.loopexit_crit_edge.i:              ; preds = %bb25.i.i
  store <8 x float> %.lcssa84768519.i, ptr %932, align 32, !noalias !6294
  br label %bb13.i7.loopexit.i, !dbg !10042

bb13.i7.loopexit.i:                               ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4166.i, %bb16.i.bb13.i7.loopexit_crit_edge.i
  %history.i.i.sroa.0.0.lcssa.i29 = phi <8 x float> [ %lanes.i3275.sroa.0.0.copyload.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %history.i.i.sroa.0.0.lcssa85388561.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4166.i ]
  %history.i.i.sroa.25.sroa.0.0.lcssa.i28 = phi <8 x float> [ %history.i.i.sroa.22.sroa.0.08414.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %history.i.i.sroa.25.sroa.0.0.lcssa10941.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4166.i ]
  %history.i.i.sroa.29.sroa.0.0.lcssa.i27 = phi <8 x float> [ %history.i.i.sroa.25.sroa.0.08419.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %history.i.i.sroa.29.sroa.0.0.lcssa8585.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4166.i ]
  %history.i.i.sroa.32.sroa.0.0.lcssa.i26 = phi <8 x float> [ %history.i.i.sroa.29.sroa.0.08418.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %history.i.i.sroa.32.sroa.0.0.lcssa8586.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4166.i ]
  %history.i.i.sroa.35.sroa.0.0.lcssa.i25 = phi <8 x float> [ %history.i.i.sroa.32.sroa.0.08417.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %history.i.i.sroa.35.sroa.0.0.lcssa8587.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4166.i ]
  %history.i.i.sroa.38.sroa.0.0.lcssa.i24 = phi <8 x float> [ %history.i.i.sroa.35.sroa.0.08416.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %history.i.i.sroa.38.sroa.0.0.lcssa8588.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4166.i ]
  %history.i.i.sroa.41.sroa.0.0.lcssa.i23 = phi <8 x float> [ %history.i.i.sroa.38.sroa.0.08415.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %history.i.i.sroa.41.sroa.0.0.lcssa8589.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4166.i ]
  %history.i.i.sroa.22.sroa.0.0.lcssa.i22 = phi <8 x float> [ %history.i.i.sroa.19.sroa.0.08413.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %history.i.i.sroa.22.sroa.0.0.lcssa10930.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4166.i ]
  %history.i.i.sroa.19.sroa.0.0.lcssa.i21 = phi <8 x float> [ %history.i.i.sroa.16.sroa.0.08412.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %history.i.i.sroa.19.sroa.0.0.lcssa10919.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4166.i ]
  %history.i.i.sroa.16.sroa.0.0.lcssa.i20 = phi <8 x float> [ %history.i.i.sroa.13.sroa.0.08411.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %history.i.i.sroa.16.sroa.0.0.lcssa10908.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4166.i ]
  %history.i.i.sroa.13.sroa.0.0.lcssa.i19 = phi <8 x float> [ %history.i.i.sroa.10.sroa.0.08410.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %history.i.i.sroa.13.sroa.0.0.lcssa10897.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4166.i ]
  %history.i.i.sroa.10.sroa.0.0.lcssa.i18 = phi <8 x float> [ %history.i.i.sroa.0.08421.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %history.i.i.sroa.10.sroa.0.0.lcssa8570.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4166.i ]
  %.lcssa84768519.lcssa8602.i = phi <8 x float> [ %.lcssa84768519.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %.lcssa84768519.lcssa8603.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4166.i ]
  %storemerge.i.i.lcssa84608499.lcssa8590.i = phi i32 [ %storemerge.i.i.lcssa84608499.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %storemerge.i.i.lcssa84608499.lcssa8591.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4166.i ]
  %minimum.i.i.sroa.0.08438.lcssa8480.lcssa.i = phi <8 x float> [ %minimum.i.i.sroa.0.08438.lcssa.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %minimum.i.i.sroa.0.08438.lcssa8480.lcssa85488560.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4166.i ]
  %ring_cursor.sroa.0.1.i.lcssa.i = phi i64 [ %ring_cursor.sroa.0.2.i.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %ring_cursor.sroa.0.0.i8562.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4166.i ], !dbg !10048
  %main_cursor.sroa.0.1.i.lcssa.i = phi i64 [ %main_cursor.sroa.0.2.i.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %main_cursor.sroa.0.0.i8563.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4166.i ], !dbg !10049
  %_111.not.i.i = icmp eq i64 %940, 0, !dbg !10018
  %indvars.iv.next9645.i = add nsw i64 %indvars.iv9644.i, -32, !dbg !10018
  br i1 %_111.not.i.i, label %bb13.i7._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i, label %bb36.i.i, !dbg !10018

bb36.i.i:                                         ; preds = %bb13.i7.loopexit.i, %bb36.i.lr.ph.i
  %history.i.i.sroa.41.sroa.0.0.lcssa8589.i2898 = phi <8 x float> [ %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i.promoted, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.41.sroa.0.0.lcssa.i23, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.38.sroa.0.0.lcssa8588.i2890 = phi <8 x float> [ %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i.promoted, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.38.sroa.0.0.lcssa.i24, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.35.sroa.0.0.lcssa8587.i2882 = phi <8 x float> [ %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i.promoted, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.35.sroa.0.0.lcssa.i25, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.32.sroa.0.0.lcssa8586.i2874 = phi <8 x float> [ %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i.promoted, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.32.sroa.0.0.lcssa.i26, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.29.sroa.0.0.lcssa8585.i2866 = phi <8 x float> [ %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i.promoted, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.29.sroa.0.0.lcssa.i27, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.25.sroa.0.0.lcssa10941.i = phi <8 x float> [ %history.i.i.sroa.25.0.hot_left.i.sroa_idx.promoted.i, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.25.sroa.0.0.lcssa.i28, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.22.sroa.0.0.lcssa10930.i = phi <8 x float> [ %history.i.i.sroa.22.0.hot_left.i.sroa_idx.promoted.i, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.22.sroa.0.0.lcssa.i22, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.19.sroa.0.0.lcssa10919.i = phi <8 x float> [ %history.i.i.sroa.19.0.hot_left.i.sroa_idx.promoted.i, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.19.sroa.0.0.lcssa.i21, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.16.sroa.0.0.lcssa10908.i = phi <8 x float> [ %history.i.i.sroa.16.0.hot_left.i.sroa_idx.promoted.i, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.16.sroa.0.0.lcssa.i20, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.13.sroa.0.0.lcssa10897.i = phi <8 x float> [ %history.i.i.sroa.13.0.hot_left.i.sroa_idx.promoted.i, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.13.sroa.0.0.lcssa.i19, %bb13.i7.loopexit.i ]
  %indvars.iv9644.i = phi i64 [ %_31, %bb36.i.lr.ph.i ], [ %indvars.iv.next9645.i, %bb13.i7.loopexit.i ]
  %.lcssa84768519.lcssa8603.i = phi <8 x float> [ %.promoted8601.i, %bb36.i.lr.ph.i ], [ %.lcssa84768519.lcssa8602.i, %bb13.i7.loopexit.i ]
  %storemerge.i.i.lcssa84608499.lcssa8591.i = phi i32 [ %_22.i.i.promoted.i, %bb36.i.lr.ph.i ], [ %storemerge.i.i.lcssa84608499.lcssa8590.i, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.41.sroa.0.0.lcssa8589.i = phi <8 x float> [ %history.i.i.sroa.41.0.hot_left.i.sroa_idx.promoted.i, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.41.sroa.0.0.lcssa.i23, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.38.sroa.0.0.lcssa8588.i = phi <8 x float> [ %history.i.i.sroa.38.0.hot_left.i.sroa_idx.promoted.i, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.38.sroa.0.0.lcssa.i24, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.35.sroa.0.0.lcssa8587.i = phi <8 x float> [ %history.i.i.sroa.35.0.hot_left.i.sroa_idx.promoted.i, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.35.sroa.0.0.lcssa.i25, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.32.sroa.0.0.lcssa8586.i = phi <8 x float> [ %history.i.i.sroa.32.0.hot_left.i.sroa_idx.promoted.i, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.32.sroa.0.0.lcssa.i26, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.29.sroa.0.0.lcssa8585.i = phi <8 x float> [ %history.i.i.sroa.29.0.hot_left.i.sroa_idx.promoted.i, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.29.sroa.0.0.lcssa.i27, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.10.sroa.0.0.lcssa8570.i = phi <8 x float> [ %history.i.i.sroa.10.0.hot_left.i.sroa_idx.promoted.i, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.10.sroa.0.0.lcssa.i18, %bb13.i7.loopexit.i ]
  %iter3.sroa.0.0.i8565.i = phi i64 [ %yield_count.sroa.0.0.i.i4135.i, %bb36.i.lr.ph.i ], [ %940, %bb13.i7.loopexit.i ]
  %iter2.sroa.0.0.i8564.i = phi i64 [ 0, %bb36.i.lr.ph.i ], [ %939, %bb13.i7.loopexit.i ]
  %main_cursor.sroa.0.0.i8563.i = phi i64 [ %885, %bb36.i.lr.ph.i ], [ %main_cursor.sroa.0.1.i.lcssa.i, %bb13.i7.loopexit.i ]
  %ring_cursor.sroa.0.0.i8562.i = phi i64 [ %884, %bb36.i.lr.ph.i ], [ %ring_cursor.sroa.0.1.i.lcssa.i, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.0.0.lcssa85388561.i = phi <8 x float> [ %hot_left.i.promoted.i, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.0.0.lcssa.i29, %bb13.i7.loopexit.i ]
  %minimum.i.i.sroa.0.08438.lcssa8480.lcssa85488560.i = phi <8 x float> [ %uniform_left.i.promoted.i, %bb36.i.lr.ph.i ], [ %minimum.i.i.sroa.0.08438.lcssa8480.lcssa.i, %bb13.i7.loopexit.i ]
  %umin9659.i = tail call i64 @llvm.umin.i64(i64 %indvars.iv9644.i, i64 32), !dbg !10050
  %umax9647.i = tail call i64 @llvm.umax.i64(i64 %umin9659.i, i64 1), !dbg !10050
  %939 = add nuw nsw i64 %iter2.sroa.0.0.i8564.i, 32, !dbg !10050
  %940 = add nsw i64 %iter3.sroa.0.0.i8565.i, -1, !dbg !10054
  %_34.i.i = sub nsw i64 %_31, %iter2.sroa.0.0.i8564.i, !dbg !10055
  %..i4136.i = tail call noundef i64 @llvm.umin.i64(i64 %_34.i.i, i64 32), !dbg !10056
  %active_base.i.i = shl i64 %iter2.sroa.0.0.i8564.i, 3, !dbg !10060
  %active_base.i7068.i = add nuw nsw i64 %..i4136.i, %iter2.sroa.0.0.i8564.i, !dbg !10061
  %_40.i.i = shl i64 %active_base.i7068.i, 3, !dbg !10061
  %_123.i.i = icmp samesign ult i64 %_40.i.i, %active_base.i.i, !dbg !10062
  %_117.not.i.i = icmp ugt i64 %_40.i.i, %_39.1
  %or.cond.i.i = or i1 %_123.i.i, %_117.not.i.i, !dbg !10062
  br i1 %or.cond.i.i, label %bb40.i.i, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4166.i, !dbg !10062, !prof !165

bb40.i.i:                                         ; preds = %bb36.i.i
  store <8 x float> %history.i.i.sroa.29.sroa.0.0.lcssa8585.i2866, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.32.sroa.0.0.lcssa8586.i2874, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.35.sroa.0.0.lcssa8587.i2882, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.38.sroa.0.0.lcssa8588.i2890, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.41.sroa.0.0.lcssa8589.i2898, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %active_base.i.i, i64 noundef %_40.i.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cf577da48352b38fc6c5372e02febe14) #30, !dbg !10071, !noalias !10072
  unreachable, !dbg !10071

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4166.i: ; preds = %bb36.i.i
  %_126.i.i = getelementptr inbounds nuw float, ptr %_39.0, i64 %active_base.i.i, !dbg !10073
  %_2.i41698409.not.i = icmp eq i64 %iter2.sroa.0.0.i8564.i, %_31, !dbg !10077
  br i1 %_2.i41698409.not.i, label %bb13.i7.loopexit.i, label %bb6.i.i.lr.ph.i, !dbg !10077

bb6.i.i.lr.ph.i:                                  ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4166.i
  %_5.i2975.i = load <8 x float>, ptr %self, align 32, !alias.scope !10080, !noalias !10083
  %_14.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %889, align 32, !alias.scope !5946, !noalias !10095
  %_17.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %890, align 32, !alias.scope !5946, !noalias !10095
  %_20.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %891, align 32, !alias.scope !5946, !noalias !10095
  %_25.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %row12.i.i.i.i.i, align 32, !alias.scope !5946, !noalias !10095
  %_28.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %892, align 32, !alias.scope !5946, !noalias !10095
  %_31.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %893, align 32, !alias.scope !5946, !noalias !10095
  %_34.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %894, align 32, !alias.scope !5946, !noalias !10095
  %_39.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %row13.i.i.i.i.i, align 32, !alias.scope !5946, !noalias !10095
  %_42.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %895, align 32, !alias.scope !5946, !noalias !10095
  %_45.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %896, align 32, !alias.scope !5946, !noalias !10095
  %_48.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %897, align 32, !alias.scope !5946, !noalias !10095
  %_53.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %row14.i.i.i.i.i, align 32, !alias.scope !5946, !noalias !10095
  %_56.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %898, align 32, !alias.scope !5946, !noalias !10095
  %_59.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %899, align 32, !alias.scope !5946, !noalias !10095
  %_62.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %900, align 32, !alias.scope !5946, !noalias !10095
  %_67.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %row15.i.i.i.i.i, align 32, !alias.scope !5946, !noalias !10095
  %_70.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %901, align 32, !alias.scope !5946, !noalias !10095
  %_73.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %902, align 32, !alias.scope !5946, !noalias !10095
  %_76.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %903, align 32, !alias.scope !5946, !noalias !10095
  %_81.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %row16.i.i.i.i.i, align 32, !alias.scope !5946, !noalias !10095
  %_84.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %904, align 32, !alias.scope !5946, !noalias !10095
  %_87.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %905, align 32, !alias.scope !5946, !noalias !10095
  %_90.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %906, align 32, !alias.scope !5946, !noalias !10095
  %_95.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %row17.i.i.i.i.i, align 32, !alias.scope !5946, !noalias !10095
  %_98.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %907, align 32, !alias.scope !5946, !noalias !10095
  %_101.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %908, align 32, !alias.scope !5946, !noalias !10095
  %_104.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %909, align 32, !alias.scope !5946, !noalias !10095
  %_109.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %row18.i.i.i.i.i, align 32, !alias.scope !5946, !noalias !10095
  %_112.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %910, align 32, !alias.scope !5946, !noalias !10095
  %_115.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %911, align 32, !alias.scope !5946, !noalias !10095
  %_118.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %912, align 32, !alias.scope !5946, !noalias !10095
  %_123.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %row19.i.i.i.i.i, align 32, !alias.scope !5946, !noalias !10095
  %_126.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %913, align 32, !alias.scope !5946, !noalias !10095
  %_129.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %914, align 32, !alias.scope !5946, !noalias !10095
  %_132.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %915, align 32, !alias.scope !5946, !noalias !10095
  %_137.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %row20.i.i.i.i.i, align 32, !alias.scope !5946, !noalias !10095
  %_140.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %916, align 32, !alias.scope !5946, !noalias !10095
  %_143.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %917, align 32, !alias.scope !5946, !noalias !10095
  %_146.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %918, align 32, !alias.scope !5946, !noalias !10095
  %_151.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %row21.i.i.i.i.i, align 32, !alias.scope !5946, !noalias !10095
  %_154.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %919, align 32, !alias.scope !5946, !noalias !10095
  %_157.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %920, align 32, !alias.scope !5946, !noalias !10095
  %_160.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %921, align 32, !alias.scope !5946, !noalias !10095
  %_165.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %row22.i.i.i.i.i, align 32, !alias.scope !5946, !noalias !10095
  %_168.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %922, align 32, !alias.scope !5946, !noalias !10095
  %_171.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %923, align 32, !alias.scope !5946, !noalias !10095
  %_174.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %924, align 32, !alias.scope !5946, !noalias !10095
  br label %bb6.i.i.i, !dbg !10077

bb6.i.i.i:                                        ; preds = %bb6.i.i.i, %bb6.i.i.lr.ph.i
  %history.i.i.sroa.0.08421.i = phi <8 x float> [ %history.i.i.sroa.0.0.lcssa85388561.i, %bb6.i.i.lr.ph.i ], [ %lanes.i3275.sroa.0.0.copyload.i, %bb6.i.i.i ]
  %iter.i.i.sroa.16.08420.i = phi i64 [ 0, %bb6.i.i.lr.ph.i ], [ %1046, %bb6.i.i.i ]
  %history.i.i.sroa.25.sroa.0.08419.i = phi <8 x float> [ %history.i.i.sroa.25.sroa.0.0.lcssa10941.i, %bb6.i.i.lr.ph.i ], [ %history.i.i.sroa.22.sroa.0.08414.i, %bb6.i.i.i ]
  %history.i.i.sroa.29.sroa.0.08418.i = phi <8 x float> [ %history.i.i.sroa.29.sroa.0.0.lcssa8585.i, %bb6.i.i.lr.ph.i ], [ %history.i.i.sroa.25.sroa.0.08419.i, %bb6.i.i.i ]
  %history.i.i.sroa.32.sroa.0.08417.i = phi <8 x float> [ %history.i.i.sroa.32.sroa.0.0.lcssa8586.i, %bb6.i.i.lr.ph.i ], [ %history.i.i.sroa.29.sroa.0.08418.i, %bb6.i.i.i ]
  %history.i.i.sroa.35.sroa.0.08416.i = phi <8 x float> [ %history.i.i.sroa.35.sroa.0.0.lcssa8587.i, %bb6.i.i.lr.ph.i ], [ %history.i.i.sroa.32.sroa.0.08417.i, %bb6.i.i.i ]
  %history.i.i.sroa.38.sroa.0.08415.i = phi <8 x float> [ %history.i.i.sroa.38.sroa.0.0.lcssa8588.i, %bb6.i.i.lr.ph.i ], [ %history.i.i.sroa.35.sroa.0.08416.i, %bb6.i.i.i ]
  %history.i.i.sroa.22.sroa.0.08414.i = phi <8 x float> [ %history.i.i.sroa.22.sroa.0.0.lcssa10930.i, %bb6.i.i.lr.ph.i ], [ %history.i.i.sroa.19.sroa.0.08413.i, %bb6.i.i.i ]
  %history.i.i.sroa.19.sroa.0.08413.i = phi <8 x float> [ %history.i.i.sroa.19.sroa.0.0.lcssa10919.i, %bb6.i.i.lr.ph.i ], [ %history.i.i.sroa.16.sroa.0.08412.i, %bb6.i.i.i ]
  %history.i.i.sroa.16.sroa.0.08412.i = phi <8 x float> [ %history.i.i.sroa.16.sroa.0.0.lcssa10908.i, %bb6.i.i.lr.ph.i ], [ %history.i.i.sroa.13.sroa.0.08411.i, %bb6.i.i.i ]
  %history.i.i.sroa.13.sroa.0.08411.i = phi <8 x float> [ %history.i.i.sroa.13.sroa.0.0.lcssa10897.i, %bb6.i.i.lr.ph.i ], [ %history.i.i.sroa.10.sroa.0.08410.i, %bb6.i.i.i ]
  %history.i.i.sroa.10.sroa.0.08410.i = phi <8 x float> [ %history.i.i.sroa.10.sroa.0.0.lcssa8570.i, %bb6.i.i.lr.ph.i ], [ %history.i.i.sroa.0.08421.i, %bb6.i.i.i ]
  %start1.i.i4175.i = shl i64 %iter.i.i.sroa.16.08420.i, 3, !dbg !10098
  %data.i.i4176.i = getelementptr inbounds nuw float, ptr %_126.i.i, i64 %start1.i.i4175.i, !dbg !10100
  %lanes.i3275.sroa.0.0.copyload.i = load <8 x float>, ptr %data.i.i4176.i, align 4, !dbg !10102, !alias.scope !10107, !noalias !10111
  %941 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i.i.sroa.22.sroa.0.08414.i), !dbg !10116
  %942 = fmul <8 x float> %_5.i2975.i, %lanes.i3275.sroa.0.0.copyload.i, !dbg !10123
  %943 = fadd <8 x float> %942, zeroinitializer, !dbg !10129
  %944 = fmul <8 x float> %_14.i.i.i.i.sroa.0.0.copyload.i, %lanes.i3275.sroa.0.0.copyload.i, !dbg !10134
  %945 = fadd <8 x float> %944, zeroinitializer, !dbg !10139
  %946 = fmul <8 x float> %_17.i.i.i.i.sroa.0.0.copyload.i, %lanes.i3275.sroa.0.0.copyload.i, !dbg !10144
  %947 = fadd <8 x float> %946, zeroinitializer, !dbg !10149
  %948 = fmul <8 x float> %_20.i.i.i.i.sroa.0.0.copyload.i, %lanes.i3275.sroa.0.0.copyload.i, !dbg !10154
  %949 = fadd <8 x float> %948, zeroinitializer, !dbg !10159
  %950 = fmul <8 x float> %_25.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.0.08421.i, !dbg !10164
  %951 = fadd <8 x float> %950, %943, !dbg !10169
  %952 = fmul <8 x float> %_28.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.0.08421.i, !dbg !10174
  %953 = fadd <8 x float> %952, %945, !dbg !10179
  %954 = fmul <8 x float> %_31.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.0.08421.i, !dbg !10184
  %955 = fadd <8 x float> %954, %947, !dbg !10189
  %956 = fmul <8 x float> %_34.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.0.08421.i, !dbg !10194
  %957 = fadd <8 x float> %956, %949, !dbg !10199
  %958 = fmul <8 x float> %_39.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.10.sroa.0.08410.i, !dbg !10204
  %959 = fadd <8 x float> %958, %951, !dbg !10209
  %960 = fmul <8 x float> %_42.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.10.sroa.0.08410.i, !dbg !10214
  %961 = fadd <8 x float> %960, %953, !dbg !10219
  %962 = fmul <8 x float> %_45.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.10.sroa.0.08410.i, !dbg !10224
  %963 = fadd <8 x float> %962, %955, !dbg !10229
  %964 = fmul <8 x float> %_48.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.10.sroa.0.08410.i, !dbg !10234
  %965 = fadd <8 x float> %964, %957, !dbg !10239
  %966 = fmul <8 x float> %_53.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.13.sroa.0.08411.i, !dbg !10244
  %967 = fadd <8 x float> %966, %959, !dbg !10249
  %968 = fmul <8 x float> %_56.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.13.sroa.0.08411.i, !dbg !10254
  %969 = fadd <8 x float> %968, %961, !dbg !10259
  %970 = fmul <8 x float> %_59.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.13.sroa.0.08411.i, !dbg !10264
  %971 = fadd <8 x float> %970, %963, !dbg !10269
  %972 = fmul <8 x float> %_62.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.13.sroa.0.08411.i, !dbg !10274
  %973 = fadd <8 x float> %972, %965, !dbg !10279
  %974 = fmul <8 x float> %_67.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.16.sroa.0.08412.i, !dbg !10284
  %975 = fadd <8 x float> %974, %967, !dbg !10289
  %976 = fmul <8 x float> %_70.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.16.sroa.0.08412.i, !dbg !10294
  %977 = fadd <8 x float> %976, %969, !dbg !10299
  %978 = fmul <8 x float> %_73.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.16.sroa.0.08412.i, !dbg !10304
  %979 = fadd <8 x float> %978, %971, !dbg !10309
  %980 = fmul <8 x float> %_76.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.16.sroa.0.08412.i, !dbg !10314
  %981 = fadd <8 x float> %980, %973, !dbg !10319
  %982 = fmul <8 x float> %_81.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.19.sroa.0.08413.i, !dbg !10324
  %983 = fadd <8 x float> %982, %975, !dbg !10329
  %984 = fmul <8 x float> %_84.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.19.sroa.0.08413.i, !dbg !10334
  %985 = fadd <8 x float> %984, %977, !dbg !10339
  %986 = fmul <8 x float> %_87.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.19.sroa.0.08413.i, !dbg !10344
  %987 = fadd <8 x float> %986, %979, !dbg !10349
  %988 = fmul <8 x float> %_90.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.19.sroa.0.08413.i, !dbg !10354
  %989 = fadd <8 x float> %988, %981, !dbg !10359
  %990 = fmul <8 x float> %_95.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.22.sroa.0.08414.i, !dbg !10364
  %991 = fadd <8 x float> %990, %983, !dbg !10369
  %992 = fmul <8 x float> %_98.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.22.sroa.0.08414.i, !dbg !10374
  %993 = fadd <8 x float> %992, %985, !dbg !10379
  %994 = fmul <8 x float> %_101.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.22.sroa.0.08414.i, !dbg !10384
  %995 = fadd <8 x float> %994, %987, !dbg !10389
  %996 = fmul <8 x float> %_104.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.22.sroa.0.08414.i, !dbg !10394
  %997 = fadd <8 x float> %996, %989, !dbg !10399
  %998 = fmul <8 x float> %_109.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.25.sroa.0.08419.i, !dbg !10404
  %999 = fadd <8 x float> %998, %991, !dbg !10409
  %1000 = fmul <8 x float> %_112.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.25.sroa.0.08419.i, !dbg !10414
  %1001 = fadd <8 x float> %1000, %993, !dbg !10419
  %1002 = fmul <8 x float> %_115.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.25.sroa.0.08419.i, !dbg !10424
  %1003 = fadd <8 x float> %1002, %995, !dbg !10429
  %1004 = fmul <8 x float> %_118.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.25.sroa.0.08419.i, !dbg !10434
  %1005 = fadd <8 x float> %1004, %997, !dbg !10439
  %1006 = fmul <8 x float> %_123.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.29.sroa.0.08418.i, !dbg !10444
  %1007 = fadd <8 x float> %1006, %999, !dbg !10449
  %1008 = fmul <8 x float> %_126.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.29.sroa.0.08418.i, !dbg !10454
  %1009 = fadd <8 x float> %1008, %1001, !dbg !10459
  %1010 = fmul <8 x float> %_129.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.29.sroa.0.08418.i, !dbg !10464
  %1011 = fadd <8 x float> %1010, %1003, !dbg !10469
  %1012 = fmul <8 x float> %_132.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.29.sroa.0.08418.i, !dbg !10474
  %1013 = fadd <8 x float> %1012, %1005, !dbg !10479
  %1014 = fmul <8 x float> %_137.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.32.sroa.0.08417.i, !dbg !10484
  %1015 = fadd <8 x float> %1014, %1007, !dbg !10489
  %1016 = fmul <8 x float> %_140.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.32.sroa.0.08417.i, !dbg !10494
  %1017 = fadd <8 x float> %1016, %1009, !dbg !10499
  %1018 = fmul <8 x float> %_143.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.32.sroa.0.08417.i, !dbg !10504
  %1019 = fadd <8 x float> %1018, %1011, !dbg !10509
  %1020 = fmul <8 x float> %_146.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.32.sroa.0.08417.i, !dbg !10514
  %1021 = fadd <8 x float> %1020, %1013, !dbg !10519
  %1022 = fmul <8 x float> %_151.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.35.sroa.0.08416.i, !dbg !10524
  %1023 = fadd <8 x float> %1022, %1015, !dbg !10529
  %1024 = fmul <8 x float> %_154.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.35.sroa.0.08416.i, !dbg !10534
  %1025 = fadd <8 x float> %1024, %1017, !dbg !10539
  %1026 = fmul <8 x float> %_157.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.35.sroa.0.08416.i, !dbg !10544
  %1027 = fadd <8 x float> %1026, %1019, !dbg !10549
  %1028 = fmul <8 x float> %_160.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.35.sroa.0.08416.i, !dbg !10554
  %1029 = fadd <8 x float> %1028, %1021, !dbg !10559
  %1030 = fmul <8 x float> %_165.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.38.sroa.0.08415.i, !dbg !10564
  %1031 = fadd <8 x float> %1030, %1023, !dbg !10569
  %1032 = fmul <8 x float> %_168.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.38.sroa.0.08415.i, !dbg !10574
  %1033 = fadd <8 x float> %1032, %1025, !dbg !10579
  %1034 = fmul <8 x float> %_171.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.38.sroa.0.08415.i, !dbg !10584
  %1035 = fadd <8 x float> %1034, %1027, !dbg !10589
  %1036 = fmul <8 x float> %_174.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.38.sroa.0.08415.i, !dbg !10594
  %1037 = fadd <8 x float> %1036, %1029, !dbg !10599
  %1038 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %1031), !dbg !10604
  %1039 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %941, <8 x float> %1038), !dbg !10610
  %1040 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %1033), !dbg !10604
  %1041 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1039, <8 x float> %1040), !dbg !10610
  %1042 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %1035), !dbg !10604
  %1043 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1041, <8 x float> %1042), !dbg !10610
  %1044 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %1037), !dbg !10604
  %1045 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1043, <8 x float> %1044), !dbg !10610
  %1046 = add nuw nsw i64 %iter.i.i.sroa.16.08420.i, 1, !dbg !10615
  %data.i4.i4180.i = getelementptr inbounds nuw float, ptr %peaks_left.i.i, i64 %start1.i.i4175.i, !dbg !10616
  store <8 x float> %1045, ptr %data.i4.i4180.i, align 4, !dbg !10619, !alias.scope !10624, !noalias !10628
  %exitcond9648.not.i = icmp eq i64 %1046, %umax9647.i, !dbg !10077
  br i1 %exitcond9648.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i, label %bb6.i.i.i, !dbg !10077

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i: ; preds = %bb6.i.i.i
  br label %bb17.i.i, !dbg !10042

bb17.i.i:                                         ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i, %bb25.i.i
  %.lcssa84768520.i = phi <8 x float> [ %.lcssa84768519.i, %bb25.i.i ], [ %.lcssa84768519.lcssa8603.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i ]
  %storemerge.i.i.lcssa84608500.i = phi i32 [ %storemerge.i.i.lcssa84608499.i, %bb25.i.i ], [ %storemerge.i.i.lcssa84608499.lcssa8591.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i ]
  %frame.sroa.0.0.i8494.i = phi i64 [ %_66.i.i, %bb25.i.i ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i ]
  %main_cursor.sroa.0.1.i8493.i = phi i64 [ %main_cursor.sroa.0.2.i.i, %bb25.i.i ], [ %main_cursor.sroa.0.0.i8563.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i ]
  %ring_cursor.sroa.0.1.i8492.i = phi i64 [ %ring_cursor.sroa.0.2.i.i, %bb25.i.i ], [ %ring_cursor.sroa.0.0.i8562.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i ]
  %minimum.i.i.sroa.0.08438.lcssa84808491.i = phi <8 x float> [ %minimum.i.i.sroa.0.08438.lcssa.i, %bb25.i.i ], [ %minimum.i.i.sroa.0.08438.lcssa8480.lcssa85488560.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i ]
  %_50.i.i = sub nuw nsw i64 %..i4136.i, %frame.sroa.0.0.i8494.i, !dbg !10632
  %ring.i1407.i = load i64, ptr %880, align 8, !dbg !10633, !alias.scope !10635, !noalias !10638, !noundef !12
  %main.i1408.i = load i64, ptr %881, align 8, !dbg !10642, !alias.scope !10635, !noalias !10638, !noundef !12
  %_10.i1409.i = add i64 %ring_cursor.sroa.0.1.i8492.i, 1, !dbg !10643
  %_45.not.i1410.i = icmp ult i64 %_10.i1409.i, %ring.i1407.i, !dbg !10644
  %1047 = select i1 %_45.not.i1410.i, i64 0, i64 %ring.i1407.i, !dbg !10644
  %start1.sroa.0.0.i1411.i = sub nuw i64 %_10.i1409.i, %1047, !dbg !10644
  %_12.i1413.i = add i64 %ring_cursor.sroa.0.1.i8492.i, %_52.i.sroa.3.0.copyload.i, !dbg !10646
  %_46.not.i1414.i = icmp ult i64 %_12.i1413.i, %ring.i1407.i, !dbg !10647
  %1048 = select i1 %_46.not.i1414.i, i64 0, i64 %ring.i1407.i, !dbg !10647
  %left_end.sroa.0.0.i1415.i = sub nuw i64 %_12.i1413.i, %1048, !dbg !10647
  %_18.i1421.i = add i64 %ring_cursor.sroa.0.1.i8492.i, %_52.i.sroa.4.0.copyload.i, !dbg !10649
  %_48.not.i1422.i = icmp ult i64 %_18.i1421.i, %ring.i1407.i, !dbg !10650
  %1049 = select i1 %_48.not.i1422.i, i64 0, i64 %ring.i1407.i, !dbg !10650
  %left_expiring.sroa.0.0.i1423.i = sub nuw i64 %_18.i1421.i, %1049, !dbg !10650
  %_30.i1428.i = sub i64 %ring.i1407.i, %ring_cursor.sroa.0.1.i8492.i, !dbg !10652
  %..i4197.i = tail call noundef i64 @llvm.umin.i64(i64 %_30.i1428.i, i64 %_50.i.i), !dbg !10653
  %_31.i1430.i = sub i64 %main.i1408.i, %main_cursor.sroa.0.1.i8493.i, !dbg !10655
  %..i4198.i = tail call noundef i64 @llvm.umin.i64(i64 %_31.i1430.i, i64 %..i4197.i), !dbg !10656
  %_32.i1432.i = sub i64 %ring.i1407.i, %start1.sroa.0.0.i1411.i, !dbg !10658
  %..i4199.i = tail call noundef i64 @llvm.umin.i64(i64 %_32.i1432.i, i64 %..i4198.i), !dbg !10659
  %_34.i1434.i = sub i64 %ring.i1407.i, %left_end.sroa.0.0.i1415.i, !dbg !10661
  %..i4200.i = tail call noundef i64 @llvm.umin.i64(i64 %_34.i1434.i, i64 %..i4199.i), !dbg !10662
  %_38.i1438.i = sub i64 %ring.i1407.i, %left_expiring.sroa.0.0.i1423.i, !dbg !10664
  %..i4202.i = tail call noundef i64 @llvm.umin.i64(i64 %_38.i1438.i, i64 %..i4200.i), !dbg !10665
  %_56.i.i = add i64 %frame.sroa.0.0.i8494.i, %iter2.sroa.0.0.i8564.i, !dbg !10667
  %base.i.i = shl i64 %_56.i.i, 3, !dbg !10667
  %base.i7070.i = add i64 %..i4202.i, %_56.i.i, !dbg !10670
  %_60.i.i = shl i64 %base.i7070.i, 3, !dbg !10670
  %_138.i.i = icmp ult i64 %_60.i.i, %base.i.i, !dbg !10673
  %_134.not.i.i = icmp ugt i64 %_60.i.i, %_39.1
  %or.cond16.i.i = or i1 %_138.i.i, %_134.not.i.i, !dbg !10673
  br i1 %or.cond16.i.i, label %bb48.i.i, label %bb47.i.i, !dbg !10673, !prof !165

bb48.i.i:                                         ; preds = %bb17.i.i
  store <8 x float> %history.i.i.sroa.25.sroa.0.08419.i, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.29.sroa.0.08418.i, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.32.sroa.0.08417.i, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.35.sroa.0.08416.i, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.38.sroa.0.08415.i, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i.i, i64 noundef %_60.i.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e76d4292d58481de1418049881299651) #30, !dbg !10681, !noalias !10072
  unreachable, !dbg !10681

bb47.i.i:                                         ; preds = %bb17.i.i
  %_141.i.i = getelementptr inbounds nuw float, ptr %_39.0, i64 %base.i.i, !dbg !10682
  %_66.i.i = add nuw nsw i64 %..i4202.i, %frame.sroa.0.0.i8494.i, !dbg !10686
  %_150.i.i.idx = shl nuw nsw i64 %frame.sroa.0.0.i8494.i, 5, !dbg !10688
  %_150.i.i = getelementptr inbounds nuw i8, ptr %peaks_left.i.i, i64 %_150.i.i.idx, !dbg !10688
  %_2.i.i.i42368445.not.i = icmp eq i64 %..i4202.i, 0, !dbg !10697
  br i1 %_2.i.i.i42368445.not.i, label %bb25.i.i, label %bb24.i.preheader.i, !dbg !10697

bb24.i.preheader.i:                               ; preds = %bb47.i.i
  %umin9655.i = tail call i64 @llvm.umin.i64(i64 %_34.i1434.i, i64 %_38.i1438.i)
  %umin9656.i = tail call i64 @llvm.umin.i64(i64 %umin9655.i, i64 %_32.i1432.i)
  %umin9657.i = tail call i64 @llvm.umin.i64(i64 %umin9656.i, i64 %_30.i1428.i)
  %umin9658.i = tail call i64 @llvm.umin.i64(i64 %umin9657.i, i64 %_31.i1430.i)
  %1050 = sub nsw i64 %umin9659.i, %frame.sroa.0.0.i8494.i
  %umin9660.i = tail call i64 @llvm.umin.i64(i64 %umin9658.i, i64 %1050)
  %1051 = and i64 %umin9660.i, 2305843009213693951
  br label %bb24.i.i

bb24.i.i:                                         ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1209.i, %bb24.i.preheader.i
  %_33.i.i.sroa.0.0.copyload8464.i = phi <8 x float> [ %.lcssa84768520.i, %bb24.i.preheader.i ], [ %1076, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1209.i ]
  %storemerge.i.i8450.i = phi i32 [ %storemerge.i.i.lcssa84608500.i, %bb24.i.preheader.i ], [ %storemerge.i.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1209.i ]
  %iter.i.sroa.16.08448.i = phi i64 [ 0, %bb24.i.preheader.i ], [ %1052, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1209.i ]
  %minimum.i.i.sroa.0.084388446.i = phi <8 x float> [ %minimum.i.i.sroa.0.08438.lcssa84808491.i, %bb24.i.preheader.i ], [ %minimum.i.i.sroa.0.0.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1209.i ]
  %start1.i.i.i.i4244.i = shl i64 %iter.i.sroa.16.08448.i, 3, !dbg !10703
  %data.i.i.i.i4245.i = getelementptr inbounds nuw float, ptr %_141.i.i, i64 %start1.i.i.i.i4244.i, !dbg !10705
  %1052 = add nuw nsw i64 %iter.i.sroa.16.08448.i, 1, !dbg !10707
  %_152.i.i = add i64 %iter.i.sroa.16.08448.i, %ring_cursor.sroa.0.1.i8492.i, !dbg !10708
  %_153.i.i = add i64 %iter.i.sroa.16.08448.i, %main_cursor.sroa.0.1.i8493.i, !dbg !10717
  %_154.i.i = add i64 %iter.i.sroa.16.08448.i, %left_end.sroa.0.0.i1415.i, !dbg !10718
  %_155.i.i = add i64 %iter.i.sroa.16.08448.i, %start1.sroa.0.0.i1411.i, !dbg !10719
  %_156.i.i = add i64 %iter.i.sroa.16.08448.i, %left_expiring.sroa.0.0.i1423.i, !dbg !10720
  %base.i9.i.i.i = shl i64 %_152.i.i, 3, !dbg !10721
  %_7.i10.i.i.i = add i64 %base.i9.i.i.i, 8, !dbg !10724
  %1053 = or disjoint i64 %base.i9.i.i.i, 7, !dbg !10725
  %or.cond.i13.i.i.not.i = icmp ult i64 %1053, %_54.1.i.i.i, !dbg !10725
  br i1 %or.cond.i13.i.i.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i.i, label %bb4.i15.i.i.i, !dbg !10725, !prof !2740

bb4.i15.i.i.i:                                    ; preds = %bb24.i.i
  store <8 x float> %history.i.i.sroa.25.sroa.0.08419.i, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.29.sroa.0.08418.i, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.32.sroa.0.08417.i, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.35.sroa.0.08416.i, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.38.sroa.0.08415.i, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i.i.i, i64 noundef %_7.i10.i.i.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !10729, !noalias !10730
  unreachable, !dbg !10729

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i.i: ; preds = %bb24.i.i
  %data.i4.i.i.i4250.i = getelementptr inbounds nuw float, ptr %_150.i.i, i64 %start1.i.i.i.i4244.i, !dbg !10744
  %lanes.i3266.sroa.0.0.copyload.i = load <8 x float>, ptr %data.i4.i.i.i4250.i, align 4, !dbg !10747, !alias.scope !10752, !noalias !10756
  %1054 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i3266.sroa.0.0.copyload.i, <8 x float> %lanes.i3266.sroa.0.0.copyload.i), !dbg !10760
  %1055 = select <8 x i1> %927, <8 x float> %1054, <8 x float> %lanes.i3266.sroa.0.0.copyload.i, !dbg !10765
  %1056 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1055, <8 x float> %_8.i.i.sroa.0.0.copyload.i, i8 30), !dbg !10770
  %1057 = bitcast <8 x float> %1056 to <8 x i32>, !dbg !10776
  %1058 = icmp slt <8 x i32> %1057, zeroinitializer, !dbg !10780
  %1059 = fdiv <8 x float> %_8.i.i.sroa.0.0.copyload.i, %1055, !dbg !10782
  %1060 = select <8 x i1> %1058, <8 x float> %1059, <8 x float> splat (float 1.000000e+00), !dbg !10780
  %_17.i14.i.i.i = getelementptr inbounds nuw float, ptr %_54.0.i.i.i, i64 %base.i9.i.i.i, !dbg !10787
  store <8 x float> %1060, ptr %_17.i14.i.i.i, align 4, !dbg !10789, !alias.scope !10794, !noalias !10798
  %base.i1228.i = shl i64 %_154.i.i, 3, !dbg !10802
  %1061 = or disjoint i64 %base.i1228.i, 7, !dbg !10805
  %or.cond.i1232.not.i = icmp ult i64 %1061, %_54.1.i.i.i, !dbg !10805
  br i1 %or.cond.i1232.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1236.i, label %bb4.i1235.i, !dbg !10805, !prof !2740

bb4.i1235.i:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i.i
  store <8 x float> %history.i.i.sroa.25.sroa.0.08419.i, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.29.sroa.0.08418.i, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.32.sroa.0.08417.i, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.35.sroa.0.08416.i, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.38.sroa.0.08415.i, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  %_5.i1229.i = add i64 %base.i1228.i, 8, !dbg !10809
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1228.i, i64 noundef %_5.i1229.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !10810, !noalias !10811
  unreachable, !dbg !10810

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1236.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i.i
  %_15.i1234.i = getelementptr inbounds nuw float, ptr %_54.0.i.i.i, i64 %base.i1228.i, !dbg !10819
  %lanes.i3046.sroa.0.0.copyload.i = load <8 x float>, ptr %_15.i1234.i, align 4, !dbg !10821, !alias.scope !10826, !noalias !10830
  %position.i.i.i = zext i32 %storemerge.i.i8450.i to i64, !dbg !10834
  %1062 = icmp eq i32 %storemerge.i.i8450.i, 0, !dbg !10835
  br i1 %1062, label %bb5.i.i.i, label %bb3.i.i.i, !dbg !10835

bb3.i.i.i:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1236.i
  %1063 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %minimum.i.i.sroa.0.084388446.i, <8 x float> %lanes.i3046.sroa.0.0.copyload.i), !dbg !10836
  br label %bb5.i.i.i, !dbg !10841

bb5.i.i.i:                                        ; preds = %bb3.i.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1236.i
  %minimum.i.i.sroa.0.0.i = phi <8 x float> [ %1063, %bb3.i.i.i ], [ %lanes.i3046.sroa.0.0.copyload.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1236.i ], !dbg !10842
  %_15.i.i.i = add nuw nsw i64 %position.i.i.i, 1, !dbg !10843
  %complete.i.i.i = icmp eq i64 %_15.i.i.i, %_18.i.i.i, !dbg !10843
  br i1 %complete.i.i.i, label %bb19.i.i.i, label %bb7.i.i.i, !dbg !10844

bb7.i.i.i:                                        ; preds = %bb5.i.i.i
  %base.i1219.i = shl i64 %_155.i.i, 3, !dbg !10845
  %1064 = or disjoint i64 %base.i1219.i, 7, !dbg !10847
  %or.cond.i1223.not.i = icmp ult i64 %1064, %_54.1.i.i.i, !dbg !10847
  br i1 %or.cond.i1223.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1227.i, label %bb4.i1226.i, !dbg !10847, !prof !2740

bb4.i1226.i:                                      ; preds = %bb7.i.i.i
  store <8 x float> %history.i.i.sroa.25.sroa.0.08419.i, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.29.sroa.0.08418.i, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.32.sroa.0.08417.i, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.35.sroa.0.08416.i, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.38.sroa.0.08415.i, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  %_5.i1220.i = add i64 %base.i1219.i, 8, !dbg !10851
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1219.i, i64 noundef %_5.i1220.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !10852, !noalias !10853
  unreachable, !dbg !10852

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1227.i: ; preds = %bb7.i.i.i
  %_15.i1225.i = getelementptr inbounds nuw float, ptr %_54.0.i.i.i, i64 %base.i1219.i, !dbg !10857
  %lanes.i3053.sroa.0.0.copyload.i = load <8 x float>, ptr %_15.i1225.i, align 4, !dbg !10859, !alias.scope !10864, !noalias !10868
  %1065 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %lanes.i3053.sroa.0.0.copyload.i, <8 x float> %minimum.i.i.sroa.0.0.i), !dbg !10872
  %1066 = trunc i64 %_15.i.i.i to i32, !dbg !10877
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i, !dbg !10878

bb19.i.i.i:                                       ; preds = %bb5.i.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1191.i
  %end.sroa.0.0.i.i8437.i = phi i64 [ %1070, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1191.i ], [ %_154.i.i, %bb5.i.i.i ]
  %iter.sroa.0.0.i.i8436.i = phi i64 [ %_30.i31.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1191.i ], [ 0, %bb5.i.i.i ]
  %suffix.i.i.sroa.0.08435.i = phi <8 x float> [ %1068, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1191.i ], [ %lanes.i3046.sroa.0.0.copyload.i, %bb5.i.i.i ]
  %base.i1183.i = shl i64 %end.sroa.0.0.i.i8437.i, 3, !dbg !10879
  %1067 = or disjoint i64 %base.i1183.i, 7, !dbg !10881
  %or.cond.i1187.not.i = icmp ult i64 %1067, %_54.1.i.i.i, !dbg !10881
  br i1 %or.cond.i1187.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1191.i, label %bb4.i1190.i, !dbg !10881, !prof !2740

bb4.i1190.i:                                      ; preds = %bb19.i.i.i
  store <8 x float> %history.i.i.sroa.25.sroa.0.08419.i, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.29.sroa.0.08418.i, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.32.sroa.0.08417.i, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.35.sroa.0.08416.i, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.38.sroa.0.08415.i, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  %_5.i1184.i = add i64 %base.i1183.i, 8, !dbg !10885
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1183.i, i64 noundef %_5.i1184.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !10886, !noalias !10887
  unreachable, !dbg !10886

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1191.i: ; preds = %bb19.i.i.i
  %_30.i31.i.i = add nuw i64 %iter.sroa.0.0.i.i8436.i, 1, !dbg !10891
  %_15.i1189.i = getelementptr inbounds nuw float, ptr %_54.0.i.i.i, i64 %base.i1183.i, !dbg !10896
  %lanes.i3081.sroa.0.0.copyload.i = load <8 x float>, ptr %_15.i1189.i, align 4, !dbg !10898, !alias.scope !10903, !noalias !10907
  %1068 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %suffix.i.i.sroa.0.08435.i, <8 x float> %lanes.i3081.sroa.0.0.copyload.i), !dbg !10911
  store <8 x float> %1068, ptr %_15.i1189.i, align 4, !dbg !10916, !alias.scope !10922, !noalias !10926
  %1069 = icmp eq i64 %end.sroa.0.0.i.i8437.i, 0, !dbg !10930
  %spec.store.select.i.i.i = select i1 %1069, i64 %ring.i.i, i64 %end.sroa.0.0.i.i8437.i, !dbg !10930
  %1070 = add i64 %spec.store.select.i.i.i, -1, !dbg !10931
  %exitcond9650.not.i = icmp eq i64 %_30.i31.i.i, %_18.i.i.i, !dbg !10932
  br i1 %exitcond9650.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i, label %bb19.i.i.i, !dbg !10934

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1191.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1227.i
  %minimum.i.i.sroa.0.1.i = phi <8 x float> [ %1065, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1227.i ], [ %minimum.i.i.sroa.0.0.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1191.i ], !dbg !10842
  %storemerge.i.i.i = phi i32 [ %1066, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1227.i ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1191.i ], !dbg !10935
  %1071 = fmul <8 x float> %minimum.i.i.sroa.0.1.i, splat (float 1.638400e+04), !dbg !10936
  %1072 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %1071), !dbg !10941
  %1073 = fmul <8 x float> %1072, splat (float 0x3F10000000000000), !dbg !10946
  %base.i1210.i = shl i64 %_156.i.i, 3, !dbg !10951
  %1074 = or disjoint i64 %base.i1210.i, 7, !dbg !10953
  %or.cond.i1214.not.i = icmp ult i64 %1074, %_56.1.i.i.i, !dbg !10953
  br i1 %or.cond.i1214.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1218.i, label %bb4.i1217.i, !dbg !10953, !prof !2740

bb4.i1217.i:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i
  store <8 x float> %history.i.i.sroa.25.sroa.0.08419.i, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.29.sroa.0.08418.i, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.32.sroa.0.08417.i, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.35.sroa.0.08416.i, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.38.sroa.0.08415.i, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  %_5.i1211.i = add i64 %base.i1210.i, 8, !dbg !10957
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1210.i, i64 noundef %_5.i1211.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !10958, !noalias !10959
  unreachable, !dbg !10958

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1218.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i
  %_15.i1216.i = getelementptr inbounds nuw float, ptr %_56.0.i.i.i, i64 %base.i1210.i, !dbg !10963
  %lanes.i3060.sroa.0.0.copyload.i = load <8 x float>, ptr %_15.i1216.i, align 4, !dbg !10965, !alias.scope !10970, !noalias !10974
  %1075 = fadd <8 x float> %_33.i.i.sroa.0.0.copyload8464.i, %1073, !dbg !10978
  %1076 = fsub <8 x float> %1075, %lanes.i3060.sroa.0.0.copyload.i, !dbg !10983
  %_8.not.i4.i.i.i = icmp ugt i64 %_7.i10.i.i.i, %_56.1.i.i.i
  br i1 %_8.not.i4.i.i.i, label %bb4.i7.i.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i.i, !dbg !10988, !prof !165

bb4.i7.i.i.i:                                     ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1218.i
  store <8 x float> %history.i.i.sroa.25.sroa.0.08419.i, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.29.sroa.0.08418.i, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.32.sroa.0.08417.i, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.35.sroa.0.08416.i, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.38.sroa.0.08415.i, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i.i.i, i64 noundef %_7.i10.i.i.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !10993, !noalias !10994
  unreachable, !dbg !10993

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1218.i
  %_17.i6.i.i.i = getelementptr inbounds nuw float, ptr %_56.0.i.i.i, i64 %base.i9.i.i.i, !dbg !10998
  store <8 x float> %1073, ptr %_17.i6.i.i.i, align 4, !dbg !11000, !alias.scope !11005, !noalias !11009
  %_41.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %934, align 32, !dbg !11013, !noalias !6294
  %1077 = fdiv <8 x float> %1076, %_37.i.i.sroa.0.0.copyload.i, !dbg !11014
  %1078 = fsub <8 x float> splat (float 1.000000e+00), %1077, !dbg !11019
  %1079 = fsub <8 x float> %1078, %_41.i.i.sroa.0.0.copyload.i, !dbg !11024
  %1080 = fmul <8 x float> %_9.i.i.sroa.0.0.copyload.i, %1079, !dbg !11029
  %1081 = fadd <8 x float> %_41.i.i.sroa.0.0.copyload.i, %1080, !dbg !11034
  %1082 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1078, <8 x float> %1081), !dbg !11038
  %1083 = bitcast <8 x float> %1082 to <8 x i32>, !dbg !11043
  %1084 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %1082), !dbg !11049
  %1085 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1084, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !11051
  %1086 = bitcast <8 x float> %1085 to <8 x i32>, !dbg !11057
  %1087 = xor <8 x i32> %1086, splat (i32 -1), !dbg !11063
  %1088 = and <8 x i32> %1087, %1083, !dbg !11065
  store <8 x i32> %1088, ptr %934, align 32, !dbg !11069, !noalias !6294
  %base.i1201.i = shl i64 %_153.i.i, 3, !dbg !11070
  %1089 = or disjoint i64 %base.i1201.i, 7, !dbg !11072
  %or.cond.i1205.not.i = icmp ult i64 %1089, %_58.1.i.i.i, !dbg !11072
  br i1 %or.cond.i1205.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1209.i, label %bb4.i1208.i, !dbg !11072, !prof !2740

bb4.i1208.i:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i.i
  store <8 x float> %history.i.i.sroa.25.sroa.0.08419.i, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.29.sroa.0.08418.i, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.32.sroa.0.08417.i, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.35.sroa.0.08416.i, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.38.sroa.0.08415.i, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  %_5.i1202.i = add i64 %base.i1201.i, 8, !dbg !11076
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1201.i, i64 noundef %_5.i1202.i, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !11077, !noalias !11078
  unreachable, !dbg !11077

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1209.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i.i
  %1090 = bitcast <8 x i32> %1088 to <8 x float>, !dbg !11082
  %1091 = fsub <8 x float> splat (float 1.000000e+00), %1090, !dbg !11083
  %_15.i1207.i = getelementptr inbounds nuw float, ptr %_58.0.i.i.i, i64 %base.i1201.i, !dbg !11088
  %lanes.i3067.sroa.0.0.copyload.i = load <8 x float>, ptr %_15.i1207.i, align 4, !dbg !11090, !alias.scope !11095, !noalias !11099
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_15.i1207.i, ptr noundef nonnull align 4 dereferenceable(32) %data.i.i.i.i4245.i, i64 32, i1 false), !dbg !11103
  %1092 = fmul <8 x float> %1091, %lanes.i3067.sroa.0.0.copyload.i, !dbg !11109
  %1093 = select <8 x i1> %938, <8 x float> %lanes.i3067.sroa.0.0.copyload.i, <8 x float> %1092, !dbg !11114
  store <8 x float> %1093, ptr %data.i.i.i.i4245.i, align 4, !dbg !11119, !alias.scope !11124, !noalias !11128
  %exitcond9661.not.i = icmp eq i64 %1052, %1051, !dbg !10697
  br i1 %exitcond9661.not.i, label %bb25.i.i, label %bb24.i.i, !dbg !10697

bb25.i.i:                                         ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1209.i, %bb47.i.i
  %.lcssa84768519.i = phi <8 x float> [ %.lcssa84768520.i, %bb47.i.i ], [ %1076, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1209.i ]
  %storemerge.i.i.lcssa84608499.i = phi i32 [ %storemerge.i.i.lcssa84608500.i, %bb47.i.i ], [ %storemerge.i.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1209.i ]
  %minimum.i.i.sroa.0.08438.lcssa.i = phi <8 x float> [ %minimum.i.i.sroa.0.08438.lcssa84808491.i, %bb47.i.i ], [ %minimum.i.i.sroa.0.0.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1209.i ]
  %_96.i.i = add i64 %..i4202.i, %ring_cursor.sroa.0.1.i8492.i, !dbg !11132
  %_151.not.i.i = icmp ult i64 %_96.i.i, %ring.i.i, !dbg !11133
  %1094 = select i1 %_151.not.i.i, i64 0, i64 %ring.i.i, !dbg !11133
  %ring_cursor.sroa.0.2.i.i = sub nuw i64 %_96.i.i, %1094, !dbg !11133
  %_98.i.i = add i64 %..i4202.i, %main_cursor.sroa.0.1.i8493.i, !dbg !11136
  %_157.not.i.i = icmp ult i64 %_98.i.i, %main.i.i, !dbg !11137
  %1095 = select i1 %_157.not.i.i, i64 0, i64 %main.i.i, !dbg !11137
  %main_cursor.sroa.0.2.i.i = sub nuw i64 %_98.i.i, %1095, !dbg !11137
  %_45.i.i = icmp ult i64 %_66.i.i, %..i4136.i, !dbg !10042
  br i1 %_45.i.i, label %bb17.i.i, label %bb16.i.bb13.i7.loopexit_crit_edge.i, !dbg !10042

bb13.i7._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i: ; preds = %bb13.i7.loopexit.i
  store <8 x float> %history.i.i.sroa.29.sroa.0.0.lcssa.i27, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.32.sroa.0.0.lcssa.i26, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.35.sroa.0.0.lcssa.i25, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.38.sroa.0.0.lcssa.i24, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.41.sroa.0.0.lcssa.i23, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa.i19, ptr %history.i.i.sroa.13.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069, !noalias !6294
  store <8 x float> %history.i.i.sroa.16.sroa.0.0.lcssa.i20, ptr %history.i.i.sroa.16.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069, !noalias !6294
  store <8 x float> %history.i.i.sroa.19.sroa.0.0.lcssa.i21, ptr %history.i.i.sroa.19.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069, !noalias !6294
  store <8 x float> %history.i.i.sroa.22.sroa.0.0.lcssa.i22, ptr %history.i.i.sroa.22.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069, !noalias !6294
  store <8 x float> %history.i.i.sroa.25.sroa.0.0.lcssa.i28, ptr %history.i.i.sroa.25.0.hot_left.i.sroa_idx.i, align 1, !dbg !10069, !noalias !6294
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa.i18, ptr %history.i.i.sroa.10.0.hot_left.i.sroa_idx.i, align 32, !dbg !11139, !noalias !6294
  %1096 = trunc i64 %main_cursor.sroa.0.1.i.lcssa.i to i32, !dbg !11140
  %1097 = trunc i64 %ring_cursor.sroa.0.1.i.lcssa.i to i32, !dbg !11142
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, !dbg !10018

_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i: ; preds = %bb13.i7._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i, %bb3.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i
  %left_phase.i.i = phi i32 [ %storemerge.i.i.lcssa84608499.lcssa8590.i, %bb13.i7._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i ], [ %left_phase.i.pre.i, %bb3.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i ], !dbg !10027
  %minimum.i.i.sroa.0.08438.lcssa8480.lcssa8548.lcssa.i = phi <8 x float> [ %minimum.i.i.sroa.0.08438.lcssa8480.lcssa.i, %bb13.i7._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i ], [ %uniform_left.i.promoted.i, %bb3.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i ]
  %history.i.i.sroa.0.0.lcssa8538.lcssa.i = phi <8 x float> [ %history.i.i.sroa.0.0.lcssa.i29, %bb13.i7._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i ], [ %hot_left.i.promoted.i, %bb3.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i ]
  %ring_cursor.sroa.0.0.i.lcssa.i = phi i32 [ %1097, %bb13.i7._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i ], [ %_26.i.i, %bb3.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i ], !dbg !10003
  %main_cursor.sroa.0.0.i.lcssa.i = phi i32 [ %1096, %bb13.i7._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i ], [ %_25.i.i, %bb3.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i ], !dbg !9999
  store <8 x float> %history.i.i.sroa.0.0.lcssa8538.lcssa.i, ptr %hot_left.i.i, align 1, !dbg !11139, !noalias !6294
  store <8 x float> %minimum.i.i.sroa.0.08438.lcssa8480.lcssa8548.lcssa.i, ptr %uniform_left.i.i, align 1, !noalias !6294
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %left_prefix.i.i, ptr noundef nonnull align 32 dereferenceable(32) %uniform_left.i.i, i64 32, i1 false), !dbg !11143, !noalias !6294
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i.i), !dbg !11144, !noalias !10007
  %1098 = getelementptr inbounds nuw i8, ptr %self, i64 1736, !dbg !11145
  %_166.1.i.i = load i64, ptr %1098, align 8, !dbg !11145, !alias.scope !11146, !noalias !11147, !noundef !12
  %_8.i3615.i = icmp samesign ugt i64 %_166.1.i.i, 7, !dbg !11148
  br i1 %_8.i3615.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3618.i, label %bb2.i3616.i, !dbg !11148, !prof !1421

bb2.i3616.i:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_166.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !11153, !noalias !11154
  unreachable, !dbg !11153

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3618.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i
  %1099 = getelementptr inbounds nuw i8, ptr %self, i64 1728, !dbg !11145
  %_166.0.i.i = load ptr, ptr %1099, align 8, !dbg !11145, !alias.scope !11146, !noalias !11147, !nonnull !12, !noundef !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_166.0.i.i, ptr noundef nonnull align 32 dereferenceable(32) %left_prefix.i.i, i64 32, i1 false), !dbg !11158, !noalias !5947
  %_167.0.i.i = load ptr, ptr %55, align 8, !dbg !11162, !alias.scope !11146, !noalias !11147, !nonnull !12, !noundef !12
  %_167.1.i.i = load i64, ptr %56, align 8, !dbg !11162, !alias.scope !11146, !noalias !11147, !noundef !12
  %1100 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i.i), !dbg !11163
  br i1 %1100, label %bb2.i4281.i, label %bb6.i4272.i, !dbg !11163

bb6.i4272.i:                                      ; preds = %bb2.i4281.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3618.i
  %end_or_len.idx.i4273.i = shl nuw nsw i64 %_167.1.i.i, 2, !dbg !11167
  %end_or_len.i4274.i = getelementptr inbounds nuw i8, ptr %_167.0.i.i, i64 %end_or_len.idx.i4273.i, !dbg !11167
  %_293.i4275.i = icmp eq i64 %_167.1.i.i, 0, !dbg !11171
  br i1 %_293.i4275.i, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4287.i, label %bb10.i4276.i, !dbg !11174

bb2.i4281.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3618.i
  %bytes1.sroa.0.0.zext.i4282.i = and i32 %left_phase.i.i, 255, !dbg !11175
  %bytes1.sroa.0.0.isplat.i4283.i = mul nuw i32 %bytes1.sroa.0.0.zext.i4282.i, 16843009, !dbg !11175
  %_5.i4284.i = icmp eq i32 %left_phase.i.i, %bytes1.sroa.0.0.isplat.i4283.i, !dbg !11176
  br i1 %_5.i4284.i, label %bb3.i4285.i, label %bb6.i4272.i, !dbg !11176

bb3.i4285.i:                                      ; preds = %bb2.i4281.i
  %bytes.sroa.0.0.extract.trunc.i4286.i = trunc i32 %left_phase.i.i to i8, !dbg !11177
  %1101 = shl nuw nsw i64 %_167.1.i.i, 2, !dbg !11179
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_167.0.i.i, i8 %bytes.sroa.0.0.extract.trunc.i4286.i, i64 %1101, i1 false), !dbg !11179, !alias.scope !11180, !noalias !10072
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4287.i, !dbg !11183

bb10.i4276.i:                                     ; preds = %bb6.i4272.i, %bb10.i4276.i
  %iter.sroa.0.04.i4277.i = phi ptr [ %_38.i4278.i, %bb10.i4276.i ], [ %_167.0.i.i, %bb6.i4272.i ]
  %_38.i4278.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i4277.i, i64 4, !dbg !11184
  store i32 %left_phase.i.i, ptr %iter.sroa.0.04.i4277.i, align 4, !dbg !11186, !alias.scope !11180, !noalias !10072
  %_29.i4279.i = icmp eq ptr %_38.i4278.i, %end_or_len.i4274.i, !dbg !11171
  br i1 %_29.i4279.i, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4287.i, label %bb10.i4276.i, !dbg !11174

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4287.i: ; preds = %bb10.i4276.i, %bb3.i4285.i, %bb6.i4272.i
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_left.i.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25) #31, !dbg !11187, !noalias !5947
  store i32 %main_cursor.sroa.0.0.i.lcssa.i, ptr %_25.i, align 4, !dbg !11140, !alias.scope !10001, !noalias !10002
  store i32 %ring_cursor.sroa.0.0.i.lcssa.i, ptr %882, align 4, !dbg !11142, !alias.scope !10001, !noalias !10002
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i.i), !dbg !11188, !noalias !10007
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter18limiter_block_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, !dbg !9973

_RINvCsdvPQf9CMsz3_17true_peak_limiter18limiter_block_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i: ; preds = %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4287.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i
  br i1 %quiet.sroa.0.06945.i, label %bb18.i, label %bb24.i, !dbg !11189

bb13.i:                                           ; preds = %bb11.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11190), !dbg !11193
  %1102 = getelementptr inbounds nuw i8, ptr %self, i64 1760, !dbg !11194
  %_40.0.i.i = load ptr, ptr %1102, align 8, !dbg !11194, !alias.scope !11196, !noalias !5947, !nonnull !12, !noundef !12
  %1103 = getelementptr inbounds nuw i8, ptr %self, i64 1768, !dbg !11194
  %_40.1.i.i = load i64, ptr %1103, align 8, !dbg !11194, !alias.scope !11196, !noalias !5947, !noundef !12
  %1104 = getelementptr inbounds nuw i8, ptr %self, i64 1824, !dbg !11197
  %_41.0.i.i = load ptr, ptr %1104, align 8, !dbg !11197, !alias.scope !11196, !noalias !5947, !nonnull !12, !noundef !12
  %1105 = getelementptr inbounds nuw i8, ptr %self, i64 1832, !dbg !11197
  %_41.1.i.i = load i64, ptr %1105, align 8, !dbg !11197, !alias.scope !11196, !noalias !5947, !noundef !12
  %..i.i.i.i.i = tail call noundef i64 @llvm.umin.i64(i64 %_41.1.i.i, i64 %_40.1.i.i), !dbg !11198
  %_2.i6.not.i.i = icmp eq i64 %..i.i.i.i.i, 0, !dbg !11204
  br i1 %_2.i6.not.i.i, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit.i, label %bb4.i4288.i, !dbg !11204

bb4.i4288.i:                                      ; preds = %bb13.i, %bb6.i4289.i
  %iter.sroa.8.07.i.i = phi i64 [ %1106, %bb6.i4289.i ], [ 0, %bb13.i ]
  %_3.i1.i.i.i = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i.i, i64 %iter.sroa.8.07.i.i, !dbg !11207
  %_14.i.i = load i32, ptr %_3.i1.i.i.i, align 4, !dbg !11210, !noalias !11211, !noundef !12
  %_20.i.i = icmp eq i32 %_14.i.i, 0, !dbg !11212
  br i1 %_20.i.i, label %panic.i4298.i, label %bb6.i4289.i, !dbg !11212

bb6.i4289.i:                                      ; preds = %bb4.i4288.i
  %_3.i.i.i4290.i = getelementptr inbounds nuw i32, ptr %_40.0.i.i, i64 %iter.sroa.8.07.i.i, !dbg !11213
  %1106 = add nuw i64 %iter.sroa.8.07.i.i, 1, !dbg !11216
  %window.i4291.i = zext i32 %_14.i.i to i64, !dbg !11210
  %_18.i4292.i = load i32, ptr %_3.i.i.i4290.i, align 4, !dbg !11217, !noalias !11211, !noundef !12
  %_17.i4293.i = zext i32 %_18.i4292.i to i64, !dbg !11217
  %_19.i4294.i = urem i64 %_31, %window.i4291.i, !dbg !11212
  %_16.i4295.i = add nuw nsw i64 %_19.i4294.i, %_17.i4293.i, !dbg !11218
  %_15.i4296.i = urem i64 %_16.i4295.i, %window.i4291.i, !dbg !11219
  %1107 = trunc nuw i64 %_15.i4296.i to i32, !dbg !11220
  store i32 %1107, ptr %_3.i.i.i4290.i, align 4, !dbg !11220, !noalias !11211
  %exitcond.not.i.i = icmp eq i64 %1106, %..i.i.i.i.i, !dbg !11204
  br i1 %exitcond.not.i.i, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit.i, label %bb4.i4288.i, !dbg !11204

panic.i4298.i:                                    ; preds = %bb4.i4288.i
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bac57976a2bdbfad4a3a85d5d1c7648c) #30, !dbg !11212, !noalias !11211
  unreachable, !dbg !11212

_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit.i: ; preds = %bb6.i4289.i, %bb13.i
  %1108 = getelementptr inbounds nuw i8, ptr %self, i64 1624, !dbg !11221
  %_20.val.i = load i64, ptr %1108, align 8, !dbg !11221, !alias.scope !5946, !noalias !5947
  %1109 = getelementptr inbounds nuw i8, ptr %self, i64 1632, !dbg !11221
  %_20.val3767.i = load i64, ptr %1109, align 8, !dbg !11221, !alias.scope !5946, !noalias !5947, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11222), !dbg !11221
  %_10.i4299.i = icmp eq i64 %_20.val3767.i, 0, !dbg !11225
  br i1 %_10.i4299.i, label %panic.i4313.i, label %bb1.i4300.i, !dbg !11225

bb1.i4300.i:                                      ; preds = %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit.i
  %_19.i = getelementptr inbounds nuw i8, ptr %self, i64 1640, !dbg !11227
  %_7.i4301.i = load i32, ptr %_19.i, align 4, !dbg !11228, !alias.scope !11229, !noalias !5947, !noundef !12
  %_6.i4302.i = zext i32 %_7.i4301.i to i64, !dbg !11228
  %_8.i4303.i = urem i64 %_31, %_20.val3767.i, !dbg !11225
  %_5.i4304.i = add nuw nsw i64 %_8.i4303.i, %_6.i4302.i, !dbg !11230
  %_4.i4305.i = urem i64 %_5.i4304.i, %_20.val3767.i, !dbg !11231
  %1110 = trunc i64 %_4.i4305.i to i32, !dbg !11232
  store i32 %1110, ptr %_19.i, align 4, !dbg !11232, !alias.scope !11229, !noalias !5947
  %_17.i4306.i = icmp eq i64 %_20.val.i, 0, !dbg !11233
  br i1 %_17.i4306.i, label %panic2.i.i, label %_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit.i, !dbg !11233

panic.i4313.i:                                    ; preds = %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit.i
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f0ee36f67d9a332211aa5518dd2ebfd5) #30, !dbg !11225, !noalias !11234
  unreachable, !dbg !11225

panic2.i.i:                                       ; preds = %bb1.i4300.i
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_33d4d33e0a850133578789055882dcf9) #30, !dbg !11233, !noalias !11234
  unreachable, !dbg !11233

_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit.i: ; preds = %bb1.i4300.i
  %1111 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !11235
  %_14.i4308.i = load i32, ptr %1111, align 4, !dbg !11235, !alias.scope !11229, !noalias !5947, !noundef !12
  %_13.i4309.i = zext i32 %_14.i4308.i to i64, !dbg !11235
  %_15.i4310.i = urem i64 %_31, %_20.val.i, !dbg !11233
  %_12.i4311.i = add nuw nsw i64 %_15.i4310.i, %_13.i4309.i, !dbg !11236
  %_11.i4312.i = urem i64 %_12.i4311.i, %_20.val.i, !dbg !11237
  %1112 = trunc i64 %_11.i4312.i to i32, !dbg !11238
  store i32 %1112, ptr %1111, align 4, !dbg !11238, !alias.scope !11229, !noalias !5947
  br label %_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_block_monoB5_.exit, !dbg !11239

bb18.i:                                           ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter18limiter_block_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_27.i = tail call noundef zeroext i1 @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_25) #31, !dbg !11240, !noalias !5947
  br i1 %_27.i, label %bb20.i, label %bb24.i, !dbg !11241

bb20.i:                                           ; preds = %bb18.i
  %_59.not.i = icmp samesign ugt i64 %words.i, %_39.1
  br i1 %_59.not.i, label %bb40.i, label %bb1.i4319.i, !dbg !11242, !prof !165

bb24.i:                                           ; preds = %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit4335.i, %bb18.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter18limiter_block_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i
  %_26.sroa.0.0.i = phi i8 [ %1128, %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit4335.i ], [ 0, %bb18.i ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter18limiter_block_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ], !dbg !11250
  store i8 %_26.sroa.0.0.i, ptr %28, align 8, !dbg !11251, !alias.scope !5946, !noalias !5947
  %1113 = load i8, ptr %1180, align 32, !dbg !11252, !range !17, !alias.scope !5946, !noalias !5947, !noundef !12
  store i8 %1113, ptr %1178, align 1, !dbg !11253, !alias.scope !5946, !noalias !5947
  %1114 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !11254
  %fst_len.i4315.i = and i64 %_39.1, 2305843009213693944, !dbg !11263
  %1115 = bitcast <8 x float> %1114 to <8 x i32>, !dbg !11267
  %_22.not.i8604.i = icmp eq i64 %fst_len.i4315.i, 0, !dbg !11277
  br i1 %_22.not.i8604.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %bb13.i1242.i, !dbg !11277

bb13.i1242.i:                                     ; preds = %bb24.i, %bb13.i1242.i
  %iter.sroa.0.0.i12418607.i = phi ptr [ %_27.i.i, %bb13.i1242.i ], [ %_39.0, %bb24.i ]
  %iter.sroa.5.0.i8606.i = phi i64 [ %_28.i.i, %bb13.i1242.i ], [ %fst_len.i4315.i, %bb24.i ]
  %ok.i.sroa.0.08605.i = phi <8 x i32> [ %1120, %bb13.i1242.i ], [ %1115, %bb24.i ]
  %lanes.i.sroa.0.0.copyload.i = load <8 x i32>, ptr %iter.sroa.0.0.i12418607.i, align 4, !dbg !11283, !alias.scope !11288, !noalias !11292
  %_27.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i12418607.i, i64 32, !dbg !11296
  %_28.i.i = add i64 %iter.sroa.5.0.i8606.i, -8, !dbg !11303
  %1116 = and <8 x i32> %lanes.i.sroa.0.0.copyload.i, splat (i32 2147483647), !dbg !11304
  %1117 = bitcast <8 x i32> %1116 to <8 x float>, !dbg !11310
  %1118 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1117, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !11311
  %1119 = bitcast <8 x float> %1118 to <8 x i32>, !dbg !11267
  %1120 = and <8 x i32> %ok.i.sroa.0.08605.i, %1119, !dbg !11317
  %_22.not.i.i = icmp eq i64 %_28.i.i, 0, !dbg !11277
  br i1 %_22.not.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.thread.i, label %bb13.i1242.i, !dbg !11277

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i: ; preds = %bb24.i
  %1121 = icmp sgt <8 x i32> %1115, splat (i32 -1), !dbg !11319
  %1122 = bitcast <8 x i1> %1121 to i8, !dbg !11319
  %_0.i3742.not.i = icmp eq i8 %1122, 0, !dbg !11329
  br i1 %_0.i3742.not.i, label %_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_block_monoB5_.exit, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, !dbg !11330

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.thread.i: ; preds = %bb13.i1242.i
  %1123 = icmp sgt <8 x i32> %1120, splat (i32 -1), !dbg !11319
  %1124 = bitcast <8 x i1> %1123 to i8, !dbg !11319
  %_0.i3742.not9945.i = icmp eq i8 %1124, 0, !dbg !11329
  br i1 %_0.i3742.not9945.i, label %_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_block_monoB5_.exit, label %bb23.i4336.i, !dbg !11330

bb1.i4319.i:                                      ; preds = %bb20.i, %bb10.i4334.i
  %iter.sroa.6.0.i4320.i = phi i64 [ %len.i.i.i.i4325.i, %bb10.i4334.i ], [ %words.i, %bb20.i ], !dbg !11331
  %iter.sroa.0.0.i4321.i = phi ptr [ %data.i.i.i.i4324.i, %bb10.i4334.i ], [ %_39.0, %bb20.i ], !dbg !11331
  %1125 = icmp eq i64 %iter.sroa.6.0.i4320.i, 0, !dbg !11333
  br i1 %1125, label %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit4335.i, label %bb11.preheader.i4322.i, !dbg !11333

bb11.preheader.i4322.i:                           ; preds = %bb1.i4319.i
  %..i.i.i4323.i = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i4320.i, i64 32), !dbg !11335
  %_18.idx.i4326.i = shl nuw nsw i64 %..i.i.i4323.i, 2, !dbg !11338
  %_18.i4327.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i4321.i, i64 %_18.idx.i4326.i, !dbg !11338
  br label %bb11.i4328.i, !dbg !11343

bb11.i4328.i:                                     ; preds = %bb11.i4328.i, %bb11.preheader.i4322.i
  %iter1.sroa.0.014.i4329.i = phi ptr [ %_31.i4331.i, %bb11.i4328.i ], [ %iter.sroa.0.0.i4321.i, %bb11.preheader.i4322.i ]
  %bits.sroa.0.013.i4330.i = phi i32 [ %1126, %bb11.i4328.i ], [ 0, %bb11.preheader.i4322.i ]
  %_31.i4331.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i4329.i, i64 4, !dbg !11345
  %_134.i4332.i = load i32, ptr %iter1.sroa.0.014.i4329.i, align 4, !dbg !11347, !alias.scope !11348, !noalias !5946, !noundef !12
  %1126 = or i32 %_134.i4332.i, %bits.sroa.0.013.i4330.i, !dbg !11351
  %_25.i4333.i = icmp eq ptr %_31.i4331.i, %_18.i4327.i, !dbg !11352
  br i1 %_25.i4333.i, label %bb10.i4334.i, label %bb11.i4328.i, !dbg !11343

bb10.i4334.i:                                     ; preds = %bb11.i4328.i
  %data.i.i.i.i4324.i = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i4321.i, i64 %..i.i.i4323.i, !dbg !11354
  %len.i.i.i.i4325.i = sub nuw nsw i64 %iter.sroa.6.0.i4320.i, %..i.i.i4323.i, !dbg !11359
  %1127 = icmp eq i32 %1126, 0, !dbg !11360
  br i1 %1127, label %bb1.i4319.i, label %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit4335.i, !dbg !11360

_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit4335.i: ; preds = %bb10.i4334.i, %bb1.i4319.i
  %1128 = zext i1 %1125 to i8, !dbg !11251
  br label %bb24.i, !dbg !11189

bb40.i:                                           ; preds = %bb20.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %words.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_54e75a1a7c7cc9b6c609a5a2db89b4ce) #30, !dbg !11361, !noalias !5947
  unreachable, !dbg !11361

bb23.i4336.i:                                     ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.thread.i, %bb23.i4336.i
  %iter.sroa.0.074.i.i = phi ptr [ %_45.i4337.i, %bb23.i4336.i ], [ %_39.0, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.thread.i ]
  %iter.sroa.5.073.i.i = phi i64 [ %_46.i.i, %bb23.i4336.i ], [ %fst_len.i4315.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.thread.i ]
  %ok.sroa.0.072.i.i = phi <8 x i32> [ %1133, %bb23.i4336.i ], [ %1115, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.thread.i ]
  %lanes.i.sroa.0.0.copyload.i.i = load <8 x i32>, ptr %iter.sroa.0.074.i.i, align 4, !dbg !11362, !alias.scope !11373, !noalias !11379
  %_45.i4337.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.074.i.i, i64 32, !dbg !11383
  %_46.i.i = add i64 %iter.sroa.5.073.i.i, -8, !dbg !11395
  %1129 = and <8 x i32> %lanes.i.sroa.0.0.copyload.i.i, splat (i32 2147483647), !dbg !11396
  %1130 = bitcast <8 x i32> %1129 to <8 x float>, !dbg !11403
  %1131 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1130, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !11404
  %1132 = bitcast <8 x float> %1131 to <8 x i32>, !dbg !11410
  %1133 = and <8 x i32> %ok.sroa.0.072.i.i, %1132, !dbg !11414
  %_40.not.i.i = icmp eq i64 %_46.i.i, 0, !dbg !11416
  br i1 %_40.not.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %bb23.i4336.i, !dbg !11416

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i: ; preds = %bb23.i4336.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i
  %ok.sroa.0.0.lcssa.i.i = phi <8 x i32> [ %1115, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i ], [ %1133, %bb23.i4336.i ], !dbg !11417
  %1134 = icmp slt <8 x i32> %ok.sroa.0.0.lcssa.i.i, zeroinitializer, !dbg !11418
  %bc.i.i = select <8 x i1> %1134, <8 x i32> zeroinitializer, <8 x i32> splat (i32 1065353216), !dbg !11418
  %1135 = extractelement <8 x i32> %bc.i.i, i64 0, !dbg !11424
  %1136 = icmp ne i32 %1135, 0, !dbg !11424
  %1137 = zext i1 %1136 to i32, !dbg !11424
  %1138 = extractelement <8 x i32> %bc.i.i, i64 1, !dbg !11424
  %1139 = icmp eq i32 %1138, 0, !dbg !11424
  %1140 = select i1 %1139, i32 0, i32 2, !dbg !11424
  %mask.sroa.0.1.1.i.i = or disjoint i32 %1140, %1137, !dbg !11424
  %1141 = extractelement <8 x i32> %bc.i.i, i64 2, !dbg !11424
  %1142 = icmp eq i32 %1141, 0, !dbg !11424
  %1143 = select i1 %1142, i32 0, i32 4, !dbg !11424
  %mask.sroa.0.1.2.i.i = or disjoint i32 %mask.sroa.0.1.1.i.i, %1143, !dbg !11424
  %1144 = extractelement <8 x i32> %bc.i.i, i64 3, !dbg !11424
  %1145 = icmp eq i32 %1144, 0, !dbg !11424
  %1146 = select i1 %1145, i32 0, i32 8, !dbg !11424
  %mask.sroa.0.1.3.i.i = or disjoint i32 %mask.sroa.0.1.2.i.i, %1146, !dbg !11424
  %1147 = extractelement <8 x i32> %bc.i.i, i64 4, !dbg !11424
  %1148 = icmp eq i32 %1147, 0, !dbg !11424
  %1149 = select i1 %1148, i32 0, i32 16, !dbg !11424
  %mask.sroa.0.1.4.i.i = or disjoint i32 %mask.sroa.0.1.3.i.i, %1149, !dbg !11424
  %1150 = extractelement <8 x i32> %bc.i.i, i64 5, !dbg !11424
  %1151 = icmp eq i32 %1150, 0, !dbg !11424
  %1152 = select i1 %1151, i32 0, i32 32, !dbg !11424
  %mask.sroa.0.1.5.i.i = or disjoint i32 %mask.sroa.0.1.4.i.i, %1152, !dbg !11424
  %1153 = extractelement <8 x i32> %bc.i.i, i64 6, !dbg !11424
  %1154 = icmp eq i32 %1153, 0, !dbg !11424
  %1155 = select i1 %1154, i32 0, i32 64, !dbg !11424
  %mask.sroa.0.1.6.i.i = or i32 %mask.sroa.0.1.5.i.i, %1155, !dbg !11424
  %1156 = extractelement <8 x i32> %bc.i.i, i64 7, !dbg !11424
  %1157 = icmp eq i32 %1156, 0, !dbg !11424
  %1158 = select i1 %1157, i32 0, i32 128, !dbg !11424
  %mask.sroa.0.1.7.i.i = or i32 %mask.sroa.0.1.6.i.i, %1158, !dbg !11424
  %1159 = getelementptr inbounds nuw i8, ptr %self, i64 1568, !dbg !11428
  %1160 = getelementptr inbounds nuw i8, ptr %self, i64 1576, !dbg !11428
  store i32 %mask.sroa.0.1.7.i.i, ptr %1160, align 8, !dbg !11428, !alias.scope !5946, !noalias !5947
  %_36.i = load i64, ptr %1159, align 32, !dbg !11429, !alias.scope !5946, !noalias !5947, !noundef !12
  %1161 = tail call i64 @llvm.uadd.sat.i64(i64 %_36.i, i64 1), !dbg !11430
  store i64 %1161, ptr %1159, align 32, !dbg !11433, !alias.scope !5946, !noalias !5947
  %_222.i.i = icmp eq i64 %_39.1, 0, !dbg !11434
  br i1 %_222.i.i, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %bb14.i4339.preheader.i, !dbg !11440

bb14.i4339.preheader.i:                           ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i
  %.idx.i.i = shl nuw nsw i64 %_39.1, 2, !dbg !11441
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_39.0, i8 0, i64 %.idx.i.i, i1 false), !dbg !11445, !alias.scope !11446, !noalias !5946
  br label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i, !dbg !11449

_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i: ; preds = %bb14.i4339.preheader.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i
  call void @llvm.lifetime.start.p0(ptr nonnull %shape.i), !dbg !11449, !noalias !6294
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %shape.i, ptr noundef nonnull align 16 dereferenceable(24) %_23.i, i64 24, i1 false), !dbg !11450, !noalias !5947
  %1162 = getelementptr inbounds nuw i8, ptr %self, i64 2120, !dbg !11451
  %rate.i = load i32, ptr %1162, align 8, !dbg !11451, !alias.scope !5946, !noalias !5947, !noundef !12
  %1163 = getelementptr inbounds nuw i8, ptr %self, i64 1584, !dbg !11453
  %_70.0.i = load ptr, ptr %1163, align 16, !dbg !11453, !alias.scope !5946, !noalias !5947, !nonnull !12, !noundef !12
  %1164 = getelementptr inbounds nuw i8, ptr %self, i64 1592, !dbg !11453
  %_70.1.i = load i64, ptr %1164, align 8, !dbg !11453, !alias.scope !5946, !noalias !5947, !noundef !12
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 8 dereferenceable(200) %_25, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %shape.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_70.0.i, i64 noundef %_70.1.i, i32 noundef %rate.i) #31, !dbg !11455, !noalias !5947
  %1165 = getelementptr inbounds nuw i8, ptr %self, i64 1600, !dbg !11456
  %_71.0.i = load ptr, ptr %1165, align 32, !dbg !11456, !alias.scope !5946, !noalias !5947, !nonnull !12, !noundef !12
  %1166 = getelementptr inbounds nuw i8, ptr %self, i64 1608, !dbg !11456
  %_71.1.i = load i64, ptr %1166, align 8, !dbg !11456, !alias.scope !5946, !noalias !5947, !noundef !12
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 8 dereferenceable(200) %_26, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %shape.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_71.0.i, i64 noundef %_71.1.i, i32 noundef %rate.i) #31, !dbg !11457, !noalias !5947
  store i32 0, ptr %_25.i, align 8, !dbg !11458, !alias.scope !5946, !noalias !5947
  %1167 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !11458
  store i32 0, ptr %1167, align 4, !dbg !11458, !alias.scope !5946, !noalias !5947
  call void @llvm.lifetime.end.p0(ptr nonnull %shape.i), !dbg !11459, !noalias !6294
  br label %_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_block_monoB5_.exit, !dbg !11460

_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_block_monoB5_.exit: ; preds = %_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.thread.i, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i670.i), !dbg !11460
  call void @llvm.lifetime.end.p0(ptr nonnull %left_prefix.i242.i), !dbg !11460
  call void @llvm.lifetime.end.p0(ptr nonnull %left_prefix.i.i), !dbg !11460
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i.i), !dbg !11460
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(328) %_0, ptr noundef nonnull align 8 dereferenceable(328) %report, i64 328, i1 false), !dbg !11461
  call void @llvm.lifetime.end.p0(ptr nonnull %report), !dbg !11462
  ret void, !dbg !11463

bb4:                                              ; preds = %bb2
  %_14 = load i32, ptr %_38.0, align 4, !dbg !5920, !noundef !12
  %start1 = zext i32 %_14 to i64, !dbg !5920
  %exitcond1997.not = icmp eq i64 %_38.1, 1, !dbg !11464
  br i1 %exitcond1997.not, label %panic2, label %bb5, !dbg !11464

panic:                                            ; preds = %bb6.6, %bb6.5, %bb6.4, %bb6.3, %bb6.2, %bb6.1, %bb2
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_38.1, i64 noundef %_38.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d412f54e6343f84b390d7eb959721e64) #30, !dbg !5920
  unreachable, !dbg !5920

bb5:                                              ; preds = %bb4
  %1168 = getelementptr inbounds nuw i8, ptr %_38.0, i64 4, !dbg !11464
  %_18 = load i32, ptr %1168, align 4, !dbg !11464, !noundef !12
  %end = zext i32 %_18 to i64, !dbg !11464
  %_53 = icmp ult i32 %_18, %_14, !dbg !11466
  %_49.not = icmp ult i64 %_37.1, %end
  %or.cond = or i1 %_53, %_49.not, !dbg !11466
  br i1 %or.cond, label %bb16, label %bb4.1, !dbg !11466, !prof !165

panic2:                                           ; preds = %bb4.7, %bb4.6, %bb4.5, %bb4.4, %bb4.3, %bb4.2, %bb4.1, %bb4
  %.lcssa1988 = phi i64 [ 1, %bb4 ], [ 2, %bb4.1 ], [ 3, %bb4.2 ], [ 4, %bb4.3 ], [ 5, %bb4.4 ], [ 6, %bb4.5 ], [ 7, %bb4.6 ], [ 8, %bb4.7 ], !dbg !11474
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.lcssa1988, i64 noundef %_38.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bc6430efe4e05da3532b83a8ba1c0c16) #30, !dbg !11464
  unreachable, !dbg !11464

bb16:                                             ; preds = %bb5.7, %bb5.6, %bb5.5, %bb5.4, %bb5.3, %bb5.2, %bb5.1, %bb5
  %end.lcssa = phi i64 [ %end, %bb5 ], [ %end.1, %bb5.1 ], [ %end.2, %bb5.2 ], [ %end.3, %bb5.3 ], [ %end.4, %bb5.4 ], [ %end.5, %bb5.5 ], [ %end.6, %bb5.6 ], [ %end.7, %bb5.7 ], !dbg !11464
  %start1.lcssa1994 = phi i64 [ %start1, %bb5 ], [ %start1.1, %bb5.1 ], [ %start1.2, %bb5.2 ], [ %start1.3, %bb5.3 ], [ %start1.4, %bb5.4 ], [ %start1.5, %bb5.5 ], [ %start1.6, %bb5.6 ], [ %start1.7, %bb5.7 ], !dbg !5920
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %start1.lcssa1994, i64 noundef %end.lcssa, i64 noundef %_37.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2db6ebe421ea47d9786daaa42d69aa61) #30, !dbg !11480
  unreachable, !dbg !11480

bb4.1:                                            ; preds = %bb5
  %_54 = sub nuw nsw i64 %end, %start1, !dbg !11481
  %_56 = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0, i64 %start1, !dbg !11482
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56, i64 noundef %_54, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23, i64 noundef %_24, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26, i64 noundef 0, ptr noalias noundef nonnull align 8 dereferenceable(40) %report) #31, !dbg !11486
  %_14.1 = load i32, ptr %1168, align 4, !dbg !5920, !noundef !12
  %start1.1 = zext i32 %_14.1 to i64, !dbg !5920
  %exitcond1997.1.not = icmp eq i64 %9, 1, !dbg !11464
  br i1 %exitcond1997.1.not, label %panic2, label %bb5.1, !dbg !11464

bb5.1:                                            ; preds = %bb4.1
  %1169 = getelementptr inbounds nuw i8, ptr %_38.0, i64 8, !dbg !11464
  %_18.1 = load i32, ptr %1169, align 4, !dbg !11464, !noundef !12
  %end.1 = zext i32 %_18.1 to i64, !dbg !11464
  %_53.1 = icmp ult i32 %_18.1, %_14.1, !dbg !11466
  %_49.not.1 = icmp ult i64 %_37.1, %end.1
  %or.cond.1 = or i1 %_53.1, %_49.not.1, !dbg !11466
  br i1 %or.cond.1, label %bb16, label %bb6.1, !dbg !11466, !prof !165

bb6.1:                                            ; preds = %bb5.1
  %_54.1 = sub nuw nsw i64 %end.1, %start1.1, !dbg !11481
  %_56.1 = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0, i64 %start1.1, !dbg !11482
  %_27.1 = getelementptr inbounds nuw i8, ptr %report, i64 40, !dbg !11487
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.1, i64 noundef %_54.1, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23, i64 noundef %_24, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26, i64 noundef 1, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.1) #31, !dbg !11486
  %exitcond.2.not = icmp eq i64 %_38.1, 2, !dbg !5920
  br i1 %exitcond.2.not, label %panic, label %bb4.2, !dbg !5920

bb4.2:                                            ; preds = %bb6.1
  %_14.2 = load i32, ptr %1169, align 4, !dbg !5920, !noundef !12
  %start1.2 = zext i32 %_14.2 to i64, !dbg !5920
  %exitcond1997.2.not = icmp eq i64 %9, 2, !dbg !11464
  br i1 %exitcond1997.2.not, label %panic2, label %bb5.2, !dbg !11464

bb5.2:                                            ; preds = %bb4.2
  %1170 = getelementptr inbounds nuw i8, ptr %_38.0, i64 12, !dbg !11464
  %_18.2 = load i32, ptr %1170, align 4, !dbg !11464, !noundef !12
  %end.2 = zext i32 %_18.2 to i64, !dbg !11464
  %_53.2 = icmp ult i32 %_18.2, %_14.2, !dbg !11466
  %_49.not.2 = icmp ult i64 %_37.1, %end.2
  %or.cond.2 = or i1 %_53.2, %_49.not.2, !dbg !11466
  br i1 %or.cond.2, label %bb16, label %bb6.2, !dbg !11466, !prof !165

bb6.2:                                            ; preds = %bb5.2
  %_54.2 = sub nuw nsw i64 %end.2, %start1.2, !dbg !11481
  %_56.2 = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0, i64 %start1.2, !dbg !11482
  %_27.2 = getelementptr inbounds nuw i8, ptr %report, i64 80, !dbg !11487
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.2, i64 noundef %_54.2, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23, i64 noundef %_24, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26, i64 noundef 2, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.2) #31, !dbg !11486
  %exitcond.3.not = icmp eq i64 %_38.1, 3, !dbg !5920
  br i1 %exitcond.3.not, label %panic, label %bb4.3, !dbg !5920

bb4.3:                                            ; preds = %bb6.2
  %_14.3 = load i32, ptr %1170, align 4, !dbg !5920, !noundef !12
  %start1.3 = zext i32 %_14.3 to i64, !dbg !5920
  %exitcond1997.3.not = icmp eq i64 %9, 3, !dbg !11464
  br i1 %exitcond1997.3.not, label %panic2, label %bb5.3, !dbg !11464

bb5.3:                                            ; preds = %bb4.3
  %1171 = getelementptr inbounds nuw i8, ptr %_38.0, i64 16, !dbg !11464
  %_18.3 = load i32, ptr %1171, align 4, !dbg !11464, !noundef !12
  %end.3 = zext i32 %_18.3 to i64, !dbg !11464
  %_53.3 = icmp ult i32 %_18.3, %_14.3, !dbg !11466
  %_49.not.3 = icmp ult i64 %_37.1, %end.3
  %or.cond.3 = or i1 %_53.3, %_49.not.3, !dbg !11466
  br i1 %or.cond.3, label %bb16, label %bb6.3, !dbg !11466, !prof !165

bb6.3:                                            ; preds = %bb5.3
  %_54.3 = sub nuw nsw i64 %end.3, %start1.3, !dbg !11481
  %_56.3 = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0, i64 %start1.3, !dbg !11482
  %_27.3 = getelementptr inbounds nuw i8, ptr %report, i64 120, !dbg !11487
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.3, i64 noundef %_54.3, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23, i64 noundef %_24, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26, i64 noundef 3, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.3) #31, !dbg !11486
  %exitcond.4.not = icmp eq i64 %_38.1, 4, !dbg !5920
  br i1 %exitcond.4.not, label %panic, label %bb4.4, !dbg !5920

bb4.4:                                            ; preds = %bb6.3
  %_14.4 = load i32, ptr %1171, align 4, !dbg !5920, !noundef !12
  %start1.4 = zext i32 %_14.4 to i64, !dbg !5920
  %exitcond1997.4.not = icmp eq i64 %9, 4, !dbg !11464
  br i1 %exitcond1997.4.not, label %panic2, label %bb5.4, !dbg !11464

bb5.4:                                            ; preds = %bb4.4
  %1172 = getelementptr inbounds nuw i8, ptr %_38.0, i64 20, !dbg !11464
  %_18.4 = load i32, ptr %1172, align 4, !dbg !11464, !noundef !12
  %end.4 = zext i32 %_18.4 to i64, !dbg !11464
  %_53.4 = icmp ult i32 %_18.4, %_14.4, !dbg !11466
  %_49.not.4 = icmp ult i64 %_37.1, %end.4
  %or.cond.4 = or i1 %_53.4, %_49.not.4, !dbg !11466
  br i1 %or.cond.4, label %bb16, label %bb6.4, !dbg !11466, !prof !165

bb6.4:                                            ; preds = %bb5.4
  %_54.4 = sub nuw nsw i64 %end.4, %start1.4, !dbg !11481
  %_56.4 = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0, i64 %start1.4, !dbg !11482
  %_27.4 = getelementptr inbounds nuw i8, ptr %report, i64 160, !dbg !11487
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.4, i64 noundef %_54.4, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23, i64 noundef %_24, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26, i64 noundef 4, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.4) #31, !dbg !11486
  %exitcond.5.not = icmp eq i64 %_38.1, 5, !dbg !5920
  br i1 %exitcond.5.not, label %panic, label %bb4.5, !dbg !5920

bb4.5:                                            ; preds = %bb6.4
  %_14.5 = load i32, ptr %1172, align 4, !dbg !5920, !noundef !12
  %start1.5 = zext i32 %_14.5 to i64, !dbg !5920
  %exitcond1997.5.not = icmp eq i64 %9, 5, !dbg !11464
  br i1 %exitcond1997.5.not, label %panic2, label %bb5.5, !dbg !11464

bb5.5:                                            ; preds = %bb4.5
  %1173 = getelementptr inbounds nuw i8, ptr %_38.0, i64 24, !dbg !11464
  %_18.5 = load i32, ptr %1173, align 4, !dbg !11464, !noundef !12
  %end.5 = zext i32 %_18.5 to i64, !dbg !11464
  %_53.5 = icmp ult i32 %_18.5, %_14.5, !dbg !11466
  %_49.not.5 = icmp ult i64 %_37.1, %end.5
  %or.cond.5 = or i1 %_53.5, %_49.not.5, !dbg !11466
  br i1 %or.cond.5, label %bb16, label %bb6.5, !dbg !11466, !prof !165

bb6.5:                                            ; preds = %bb5.5
  %_54.5 = sub nuw nsw i64 %end.5, %start1.5, !dbg !11481
  %_56.5 = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0, i64 %start1.5, !dbg !11482
  %_27.5 = getelementptr inbounds nuw i8, ptr %report, i64 200, !dbg !11487
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.5, i64 noundef %_54.5, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23, i64 noundef %_24, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26, i64 noundef 5, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.5) #31, !dbg !11486
  %exitcond.6.not = icmp eq i64 %_38.1, 6, !dbg !5920
  br i1 %exitcond.6.not, label %panic, label %bb4.6, !dbg !5920

bb4.6:                                            ; preds = %bb6.5
  %_14.6 = load i32, ptr %1173, align 4, !dbg !5920, !noundef !12
  %start1.6 = zext i32 %_14.6 to i64, !dbg !5920
  %exitcond1997.6.not = icmp eq i64 %9, 6, !dbg !11464
  br i1 %exitcond1997.6.not, label %panic2, label %bb5.6, !dbg !11464

bb5.6:                                            ; preds = %bb4.6
  %1174 = getelementptr inbounds nuw i8, ptr %_38.0, i64 28, !dbg !11464
  %_18.6 = load i32, ptr %1174, align 4, !dbg !11464, !noundef !12
  %end.6 = zext i32 %_18.6 to i64, !dbg !11464
  %_53.6 = icmp ult i32 %_18.6, %_14.6, !dbg !11466
  %_49.not.6 = icmp ult i64 %_37.1, %end.6
  %or.cond.6 = or i1 %_53.6, %_49.not.6, !dbg !11466
  br i1 %or.cond.6, label %bb16, label %bb6.6, !dbg !11466, !prof !165

bb6.6:                                            ; preds = %bb5.6
  %_54.6 = sub nuw nsw i64 %end.6, %start1.6, !dbg !11481
  %_56.6 = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0, i64 %start1.6, !dbg !11482
  %_27.6 = getelementptr inbounds nuw i8, ptr %report, i64 240, !dbg !11487
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.6, i64 noundef %_54.6, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23, i64 noundef %_24, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26, i64 noundef 6, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.6) #31, !dbg !11486
  %exitcond.7.not = icmp eq i64 %_38.1, 7, !dbg !5920
  br i1 %exitcond.7.not, label %panic, label %bb4.7, !dbg !5920

bb4.7:                                            ; preds = %bb6.6
  %_14.7 = load i32, ptr %1174, align 4, !dbg !5920, !noundef !12
  %start1.7 = zext i32 %_14.7 to i64, !dbg !5920
  %exitcond1997.7.not = icmp eq i64 %9, 7, !dbg !11464
  br i1 %exitcond1997.7.not, label %panic2, label %bb5.7, !dbg !11464

bb5.7:                                            ; preds = %bb4.7
  %1175 = getelementptr inbounds nuw i8, ptr %_38.0, i64 32, !dbg !11464
  %_18.7 = load i32, ptr %1175, align 4, !dbg !11464, !noundef !12
  %end.7 = zext i32 %_18.7 to i64, !dbg !11464
  %_53.7 = icmp ult i32 %_18.7, %_14.7, !dbg !11466
  %_49.not.7 = icmp ult i64 %_37.1, %end.7
  %or.cond.7 = or i1 %_53.7, %_49.not.7, !dbg !11466
  br i1 %or.cond.7, label %bb16, label %bb6.7, !dbg !11466, !prof !165

bb6.7:                                            ; preds = %bb5.7
  %_54.7 = sub nuw nsw i64 %end.7, %start1.7, !dbg !11481
  %_56.7 = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0, i64 %start1.7, !dbg !11482
  %_27.7 = getelementptr inbounds nuw i8, ptr %report, i64 280, !dbg !11487
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.7, i64 noundef %_54.7, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23, i64 noundef %_24, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26, i64 noundef 7, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.7) #31, !dbg !11486
  %_39.0 = load ptr, ptr %block, align 8, !dbg !11488, !nonnull !12, !align !24, !noundef !12
  %1176 = getelementptr inbounds nuw i8, ptr %block, i64 8, !dbg !11488
  %_39.1 = load i64, ptr %1176, align 8, !dbg !11488, !noundef !12
  %1177 = getelementptr inbounds nuw i8, ptr %block, i64 104, !dbg !11489
  %_32 = load i32, ptr %1177, align 8, !dbg !11489, !noundef !12
  %_31 = zext i32 %_32 to i64, !dbg !11489
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5946), !dbg !11490
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5947), !dbg !11490
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i670.i), !dbg !11491
  call void @llvm.lifetime.start.p0(ptr nonnull %left_prefix.i242.i), !dbg !11491
  call void @llvm.lifetime.start.p0(ptr nonnull %left_prefix.i.i), !dbg !11491
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i.i), !dbg !11491
  %words.i = shl nuw nsw i64 %_31, 3, !dbg !11491
  %1178 = getelementptr inbounds nuw i8, ptr %self, i64 2153, !dbg !5944
  %1179 = load i8, ptr %1178, align 1, !dbg !5944, !range !17, !alias.scope !5946, !noalias !5947, !noundef !12
  %1180 = getelementptr inbounds nuw i8, ptr %self, i64 2144, !dbg !11492
  %1181 = load i8, ptr %1180, align 32, !dbg !11492, !range !17, !alias.scope !5946, !noalias !5947, !noundef !12
  %_6.i = icmp eq i8 %1179, %1181, !dbg !5944
  %1182 = getelementptr inbounds nuw i8, ptr %self, i64 1776
  %1183 = getelementptr inbounds nuw i8, ptr %self, i64 1784
  %_68.1.i = load i64, ptr %1183, align 8, !dbg !11493, !alias.scope !5946, !noalias !5947
  br i1 %_6.i, label %bb1.i, label %start.bb11.thread_crit_edge.i, !dbg !5944
}
