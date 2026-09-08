define internal void @_RNvXse_CsdvPQf9CMsz3_17true_peak_limiterINtB5_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank17process_bank_monoB5_(ptr dead_on_unwind noalias noundef writable writeonly sret([328 x i8]) align 8 captures(none) dereferenceable(328) %_0, ptr noalias noundef align 32 dereferenceable(2304) %self, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(112) %block) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !25909 {
start:
  %peaks_left.i210.i.i = alloca [1024 x i8], align 4
  %scratch.i.i.i = alloca [32 x i8], align 4
  %hot_left.i216.i.i = alloca [736 x i8], align 32
  %left_prefix.i.i.i = alloca [32 x i8], align 32
  %uniform_left.i.i.i = alloca [128 x i8], align 32
  %peaks_left.i.i.i = alloca [1024 x i8], align 4
  %hot_left.i.i.i = alloca [736 x i8], align 32
  %shape.i.i = alloca [24 x i8], align 8
  %report.i = alloca [328 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !25910), !dbg !25913
  tail call void @llvm.experimental.noalias.scope.decl(metadata !25914), !dbg !25913
  %0 = getelementptr inbounds nuw i8, ptr %block, i64 32, !dbg !25916
  %_37.0.i = load ptr, ptr %0, align 8, !dbg !25916, !alias.scope !25914, !noalias !25919, !nonnull !12, !align !21228, !noundef !12
  %1 = getelementptr inbounds nuw i8, ptr %block, i64 40, !dbg !25916
  %_37.1.i = load i64, ptr %1, align 8, !dbg !25916, !alias.scope !25914, !noalias !25919, !noundef !12
  %2 = icmp eq i64 %_37.1.i, 0, !dbg !25916
  br i1 %2, label %bb2.i, label %bb1.i, !dbg !25916

bb2.i:                                            ; preds = %bb1.i, %start
  call void @llvm.lifetime.start.p0(ptr nonnull %report.i), !dbg !25921, !noalias !25922
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 2288, !dbg !25923
  %4 = load i8, ptr %3, align 16, !dbg !25923, !range !5399, !alias.scope !25910, !noalias !25924, !noundef !12
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(320) %report.i, i8 0, i64 320, i1 false), !noalias !25922
  %5 = getelementptr inbounds nuw i8, ptr %report.i, i64 320, !dbg !25925
  store i8 %4, ptr %5, align 8, !dbg !25925, !noalias !25922
  %6 = getelementptr inbounds nuw i8, ptr %block, i64 48
  %_38.0.i = load ptr, ptr %6, align 8, !alias.scope !25914, !noalias !25919, !nonnull !12, !align !9542, !noundef !12
  %7 = getelementptr inbounds nuw i8, ptr %block, i64 56
  %_38.1.i = load i64, ptr %7, align 8, !alias.scope !25914, !noalias !25919, !noundef !12
  %_26.i = getelementptr inbounds nuw i8, ptr %self, i64 1848
  %_25.i = getelementptr inbounds nuw i8, ptr %self, i64 1648
  %8 = getelementptr inbounds nuw i8, ptr %block, i64 96
  %_24.i = load i64, ptr %8, align 8, !alias.scope !25914, !noalias !25919
  %_23.i = getelementptr inbounds nuw i8, ptr %self, i64 2048
  %9 = tail call i64 @llvm.usub.sat.i64(i64 %_38.1.i, i64 1), !dbg !25928
  %exitcond.not.i = icmp eq i64 %_38.1.i, 0, !dbg !25936
  br i1 %exitcond.not.i, label %panic.i, label %bb4.i, !dbg !25936

bb1.i:                                            ; preds = %start
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 2152, !dbg !25938
  store i8 0, ptr %10, align 8, !dbg !25938, !alias.scope !25910, !noalias !25924
  br label %bb2.i, !dbg !25939

bb1.i.i:                                          ; preds = %bb6.7.i
  %11 = getelementptr inbounds nuw i8, ptr %self, i64 1776, !dbg !25940
  %_71.0.i.i = load ptr, ptr %11, align 16, !dbg !25940, !alias.scope !25944, !noalias !25947, !nonnull !12, !noundef !12
  %12 = getelementptr inbounds nuw i8, ptr %self, i64 1784, !dbg !25940
  %_71.1.i.i = load i64, ptr %12, align 8, !dbg !25940, !alias.scope !25944, !noalias !25947, !noundef !12
  %_8.i1659.i.i = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_71.0.i.i, i64 %_71.1.i.i, !dbg !25949
  br label %bb1.i.i1660.i.i, !dbg !25954

bb1.i.i1660.i.i:                                  ; preds = %bb13.i.i1662.i.i, %bb1.i.i
  %_221.i.i.i.i = phi ptr [ %_22.i.i1663.i.i, %bb13.i.i1662.i.i ], [ %_71.0.i.i, %bb1.i.i ]
  %_12.i.i1661.i.i = icmp eq ptr %_221.i.i.i.i, %_8.i1659.i.i, !dbg !25956
  br i1 %_12.i.i1661.i.i, label %bb3.i.i, label %bb13.i.i1662.i.i, !dbg !25959

bb13.i.i1662.i.i:                                 ; preds = %bb1.i.i1660.i.i
  %_22.i.i1663.i.i = getelementptr inbounds nuw i8, ptr %_221.i.i.i.i, i64 16, !dbg !25960
  %13 = getelementptr inbounds nuw i8, ptr %_221.i.i.i.i, i64 12, !dbg !25962
  %_3.i.i.i.i.i = load i32, ptr %13, align 4, !dbg !25962, !alias.scope !25964, !noalias !25969, !noundef !12
  %14 = icmp eq i32 %_3.i.i.i.i.i, 0, !dbg !25962
  %_51.i.i.i.i.i = load i32, ptr %_221.i.i.i.i, align 4, !dbg !25962, !alias.scope !25964, !noalias !25969
  %15 = getelementptr inbounds nuw i8, ptr %_221.i.i.i.i, i64 4, !dbg !25962
  %_72.i.i.i.i.i = load i32, ptr %15, align 4, !dbg !25962, !alias.scope !25964, !noalias !25969
  %16 = icmp eq i32 %_51.i.i.i.i.i, %_72.i.i.i.i.i, !dbg !25962
  %_0.sroa.0.0.i.i.i.i.i = select i1 %14, i1 %16, i1 false, !dbg !25962
  br i1 %_0.sroa.0.0.i.i.i.i.i, label %bb1.i.i1660.i.i, label %bb11.thread.i.i, !dbg !25972

bb3.i.i:                                          ; preds = %bb1.i.i1660.i.i
  %17 = getelementptr inbounds nuw i8, ptr %self, i64 1792, !dbg !25973
  %_72.0.i.i = load ptr, ptr %17, align 16, !dbg !25973, !alias.scope !25944, !noalias !25947, !nonnull !12, !noundef !12
  %18 = getelementptr inbounds nuw i8, ptr %self, i64 1800, !dbg !25973
  %_72.1.i.i = load i64, ptr %18, align 8, !dbg !25973, !alias.scope !25944, !noalias !25947, !noundef !12
  %_8.i1664.i.i = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_72.0.i.i, i64 %_72.1.i.i, !dbg !25974
  br label %bb1.i.i1665.i.i, !dbg !25979

bb1.i.i1665.i.i:                                  ; preds = %bb13.i.i1668.i.i, %bb3.i.i
  %_221.i.i1666.i.i = phi ptr [ %_22.i.i1669.i.i, %bb13.i.i1668.i.i ], [ %_72.0.i.i, %bb3.i.i ]
  %_12.i.i1667.i.i = icmp eq ptr %_221.i.i1666.i.i, %_8.i1664.i.i, !dbg !25981
  br i1 %_12.i.i1667.i.i, label %bb5.i.i, label %bb13.i.i1668.i.i, !dbg !25984

bb13.i.i1668.i.i:                                 ; preds = %bb1.i.i1665.i.i
  %_22.i.i1669.i.i = getelementptr inbounds nuw i8, ptr %_221.i.i1666.i.i, i64 16, !dbg !25985
  %19 = getelementptr inbounds nuw i8, ptr %_221.i.i1666.i.i, i64 12, !dbg !25987
  %_3.i.i.i1670.i.i = load i32, ptr %19, align 4, !dbg !25987, !alias.scope !25989, !noalias !25994, !noundef !12
  %20 = icmp eq i32 %_3.i.i.i1670.i.i, 0, !dbg !25987
  %_51.i.i.i1671.i.i = load i32, ptr %_221.i.i1666.i.i, align 4, !dbg !25987, !alias.scope !25989, !noalias !25994
  %21 = getelementptr inbounds nuw i8, ptr %_221.i.i1666.i.i, i64 4, !dbg !25987
  %_72.i.i.i1672.i.i = load i32, ptr %21, align 4, !dbg !25987, !alias.scope !25989, !noalias !25994
  %22 = icmp eq i32 %_51.i.i.i1671.i.i, %_72.i.i.i1672.i.i, !dbg !25987
  %_0.sroa.0.0.i.i.i1673.i.i = select i1 %20, i1 %22, i1 false, !dbg !25987
  br i1 %_0.sroa.0.0.i.i.i1673.i.i, label %bb1.i.i1665.i.i, label %bb11.thread.i.i, !dbg !25997

bb5.i.i:                                          ; preds = %bb1.i.i1665.i.i
  %_50.not.i.i = icmp samesign ugt i64 %words.i.i, %_39.1.i
  br i1 %_50.not.i.i, label %bb34.i.i, label %bb1.i1675.i.i, !dbg !25998, !prof !5262

bb11.thread.i.i:                                  ; preds = %bb13.i.i1662.i.i, %bb13.i.i1668.i.i, %bb6.7.i
  %23 = getelementptr inbounds nuw i8, ptr %self, i64 2152
  br label %bb16.i.i, !dbg !26007

bb11.i.i:                                         ; preds = %bb1.i1675.i.i
  %24 = getelementptr inbounds nuw i8, ptr %self, i64 2152
  %25 = load i8, ptr %24, align 8, !range !5399, !alias.scope !25944, !noalias !25947
  %_15.i.i = trunc nuw i8 %25 to i1
  br i1 %_15.i.i, label %bb13.i.i, label %bb16.i.i, !dbg !26007

bb1.i1675.i.i:                                    ; preds = %bb5.i.i, %bb10.i.i.i
  %iter.sroa.6.0.i.i.i = phi i64 [ %len.i.i.i.i.i.i, %bb10.i.i.i ], [ %words.i.i, %bb5.i.i ], !dbg !26009
  %iter.sroa.0.0.i1676.i.i = phi ptr [ %data.i.i.i.i.i.i, %bb10.i.i.i ], [ %_39.0.i, %bb5.i.i ], !dbg !26009
  %26 = icmp eq i64 %iter.sroa.6.0.i.i.i, 0, !dbg !26011
  br i1 %26, label %bb11.i.i, label %bb11.preheader.i.i.i, !dbg !26011

bb11.preheader.i.i.i:                             ; preds = %bb1.i1675.i.i
  %..i.i.i.i.i = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i.i.i, i64 32), !dbg !26013
  %_18.idx.i.i.i = shl nuw nsw i64 %..i.i.i.i.i, 2, !dbg !26016
  %_18.i1677.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i1676.i.i, i64 %_18.idx.i.i.i, !dbg !26016
  br label %bb11.i1678.i.i, !dbg !26021

bb11.i1678.i.i:                                   ; preds = %bb11.i1678.i.i, %bb11.preheader.i.i.i
  %iter1.sroa.0.014.i.i.i = phi ptr [ %_31.i1679.i.i, %bb11.i1678.i.i ], [ %iter.sroa.0.0.i1676.i.i, %bb11.preheader.i.i.i ]
  %bits.sroa.0.013.i.i.i = phi i32 [ %27, %bb11.i1678.i.i ], [ 0, %bb11.preheader.i.i.i ]
  %_31.i1679.i.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i.i.i, i64 4, !dbg !26023
  %_134.i1680.i.i = load i32, ptr %iter1.sroa.0.014.i.i.i, align 4, !dbg !26025, !alias.scope !26026, !noalias !26029, !noundef !12
  %27 = or i32 %_134.i1680.i.i, %bits.sroa.0.013.i.i.i, !dbg !26030
  %_25.i1681.i.i = icmp eq ptr %_31.i1679.i.i, %_18.i1677.i.i, !dbg !26031
  br i1 %_25.i1681.i.i, label %bb10.i.i.i, label %bb11.i1678.i.i, !dbg !26021

bb10.i.i.i:                                       ; preds = %bb11.i1678.i.i
  %data.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i1676.i.i, i64 %..i.i.i.i.i, !dbg !26033
  %len.i.i.i.i.i.i = sub nuw nsw i64 %iter.sroa.6.0.i.i.i, %..i.i.i.i.i, !dbg !26038
  %28 = icmp eq i32 %27, 0, !dbg !26039
  br i1 %28, label %bb1.i1675.i.i, label %bb11.thread3257.i.i, !dbg !26039

bb11.thread3257.i.i:                              ; preds = %bb10.i.i.i
  %29 = getelementptr inbounds nuw i8, ptr %self, i64 2152
  br label %bb16.i.i, !dbg !26007

bb34.i.i:                                         ; preds = %bb5.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %words.i.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5a4c0a22e74ec240d954f0d98bfc1654) #31, !dbg !26040, !noalias !25947
  unreachable, !dbg !26040

bb16.i.i:                                         ; preds = %bb11.thread3257.i.i, %bb11.i.i, %bb11.thread.i.i
  %30 = phi ptr [ %23, %bb11.thread.i.i ], [ %24, %bb11.i.i ], [ %29, %bb11.thread3257.i.i ]
  %quiet.sroa.0.03256.i.i = phi i1 [ false, %bb11.thread.i.i ], [ true, %bb11.i.i ], [ false, %bb11.thread3257.i.i ]
  %_22.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1616, !dbg !26041
  %_24.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1640, !dbg !26042
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26043), !dbg !26046
  %31 = getelementptr inbounds nuw i8, ptr %self, i64 1824, !dbg !26049
  %_31.0.i.i.i = load ptr, ptr %31, align 8, !dbg !26049, !alias.scope !26051, !noalias !25947, !nonnull !12, !noundef !12
  %32 = getelementptr inbounds nuw i8, ptr %self, i64 1832, !dbg !26049
  %_31.1.i.i.i = load i64, ptr %32, align 8, !dbg !26049, !alias.scope !26051, !noalias !25947, !noundef !12
  %_17.idx.i.i.i = mul nuw nsw i64 %_31.1.i.i.i, 12, !dbg !26052
  %_17.i.i.i = getelementptr inbounds nuw i8, ptr %_31.0.i.i.i, i64 %_17.idx.i.i.i, !dbg !26052
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26056), !dbg !26059
  %_5.not.i.i.i.i.i = icmp eq i64 %_31.1.i.i.i, 0
  %33 = getelementptr inbounds nuw i8, ptr %_31.0.i.i.i, i64 4
  %34 = getelementptr inbounds nuw i8, ptr %_31.0.i.i.i, i64 8
  br i1 %_5.not.i.i.i.i.i, label %bb2.i1692.i.i, label %bb1.i.i1682.i.i

bb1.i.i1682.i.i:                                  ; preds = %bb16.i.i, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i.i.i
  %_224.i.i.i.i = phi ptr [ %_22.i.i1685.i.i, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i.i.i ], [ %_31.0.i.i.i, %bb16.i.i ]
  %_12.i.i1683.i.i = icmp eq ptr %_224.i.i.i.i, %_17.i.i.i, !dbg !26060
  br i1 %_12.i.i1683.i.i, label %bb2.i1692.i.i, label %bb13.i.i1684.i.i, !dbg !26064

bb13.i.i1684.i.i:                                 ; preds = %bb1.i.i1682.i.i
  %_22.i.i1685.i.i = getelementptr inbounds nuw i8, ptr %_224.i.i.i.i, i64 12, !dbg !26065
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26067), !dbg !26070
  %_9.i.i.i1686.i.i = load i32, ptr %_224.i.i.i.i, align 4, !dbg !26071, !alias.scope !26067, !noalias !26074, !noundef !12
  %_10.i.i.i.i.i = load i32, ptr %_31.0.i.i.i, align 4, !dbg !26071, !alias.scope !26056, !noalias !26076, !noundef !12
  %_8.i.i.i.i.i = icmp eq i32 %_9.i.i.i1686.i.i, %_10.i.i.i.i.i, !dbg !26071
  br i1 %_8.i.i.i.i.i, label %bb2.i.i.i.i.i, label %bb39.i.i, !dbg !26071

bb2.i.i.i.i.i:                                    ; preds = %bb13.i.i1684.i.i
  %35 = getelementptr inbounds nuw i8, ptr %_224.i.i.i.i, i64 4, !dbg !26071
  %_12.i.i.i1688.i.i = load i32, ptr %35, align 4, !dbg !26071, !alias.scope !26067, !noalias !26074, !noundef !12
  %_13.i.i.i1689.i.i = load i32, ptr %33, align 4, !dbg !26071, !alias.scope !26056, !noalias !26076, !noundef !12
  %_11.i.i.i.i.i = icmp eq i32 %_12.i.i.i1688.i.i, %_13.i.i.i1689.i.i, !dbg !26071
  br i1 %_11.i.i.i.i.i, label %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i.i.i, label %bb39.i.i, !dbg !26071

_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i.i.i: ; preds = %bb2.i.i.i.i.i
  %36 = getelementptr inbounds nuw i8, ptr %_224.i.i.i.i, i64 8, !dbg !26071
  %_14.i.i.i1690.i.i = load i32, ptr %36, align 4, !dbg !26071, !alias.scope !26067, !noalias !26074, !noundef !12
  %_15.i.i.i1691.i.i = load i32, ptr %34, align 4, !dbg !26071, !alias.scope !26056, !noalias !26076, !noundef !12
  %37 = icmp eq i32 %_14.i.i.i1690.i.i, %_15.i.i.i1691.i.i, !dbg !26071
  br i1 %37, label %bb1.i.i1682.i.i, label %bb39.i.i, !dbg !26070

bb2.i1692.i.i:                                    ; preds = %bb1.i.i1682.i.i, %bb16.i.i
  %38 = getelementptr inbounds nuw i8, ptr %self, i64 1760, !dbg !26077
  %_32.0.i.i.i = load ptr, ptr %38, align 8, !dbg !26077, !alias.scope !26051, !noalias !25947, !nonnull !12, !noundef !12
  %39 = getelementptr inbounds nuw i8, ptr %self, i64 1768, !dbg !26077
  %_32.1.i.i.i = load i64, ptr %39, align 8, !dbg !26077, !alias.scope !26051, !noalias !25947, !noundef !12
  %_26.idx.i.i.i = shl nuw nsw i64 %_32.1.i.i.i, 2, !dbg !26078
  %_26.i1693.i.i = getelementptr inbounds nuw i8, ptr %_32.0.i.i.i, i64 %_26.idx.i.i.i, !dbg !26078
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26082), !dbg !26085
  %_6.not.i.i.i.i.i = icmp eq i64 %_32.1.i.i.i, 0
  br i1 %_6.not.i.i.i.i.i, label %bb38.i.i, label %bb1.i3.i.i.i

bb1.i3.i.i.i:                                     ; preds = %bb2.i1692.i.i, %bb13.i5.i.i.i
  %_223.i.i.i.i = phi ptr [ %_22.i6.i.i.i, %bb13.i5.i.i.i ], [ %_32.0.i.i.i, %bb2.i1692.i.i ]
  %_12.i4.i.i.i = icmp eq ptr %_223.i.i.i.i, %_26.i1693.i.i, !dbg !26086
  br i1 %_12.i4.i.i.i, label %bb38.i.i, label %bb13.i5.i.i.i, !dbg !26090

bb13.i5.i.i.i:                                    ; preds = %bb1.i3.i.i.i
  %_22.i6.i.i.i = getelementptr inbounds nuw i8, ptr %_223.i.i.i.i, i64 4, !dbg !26091
  %ptr.val.i.i.i.i = load i32, ptr %_223.i.i.i.i, align 4, !dbg !26093, !noalias !26094
  %_4.i.i.i1694.i.i = load i32, ptr %_32.0.i.i.i, align 4, !dbg !26096, !alias.scope !26082, !noalias !26098, !noundef !12
  %_0.i.i.i.i.i = icmp eq i32 %ptr.val.i.i.i.i, %_4.i.i.i1694.i.i, !dbg !26099
  br i1 %_0.i.i.i.i.i, label %bb1.i3.i.i.i, label %bb39.i.i, !dbg !26093

bb13.i.i:                                         ; preds = %bb11.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26100), !dbg !26103
  %40 = getelementptr inbounds nuw i8, ptr %self, i64 1760, !dbg !26104
  %_40.0.i.i.i = load ptr, ptr %40, align 8, !dbg !26104, !alias.scope !26106, !noalias !25947, !nonnull !12, !noundef !12
  %41 = getelementptr inbounds nuw i8, ptr %self, i64 1768, !dbg !26104
  %_40.1.i.i.i = load i64, ptr %41, align 8, !dbg !26104, !alias.scope !26106, !noalias !25947, !noundef !12
  %42 = getelementptr inbounds nuw i8, ptr %self, i64 1824, !dbg !26107
  %_41.0.i.i.i = load ptr, ptr %42, align 8, !dbg !26107, !alias.scope !26106, !noalias !25947, !nonnull !12, !noundef !12
  %43 = getelementptr inbounds nuw i8, ptr %self, i64 1832, !dbg !26107
  %_41.1.i.i.i = load i64, ptr %43, align 8, !dbg !26107, !alias.scope !26106, !noalias !25947, !noundef !12
  %..i.i.i.i.i.i = tail call noundef i64 @llvm.umin.i64(i64 %_41.1.i.i.i, i64 %_40.1.i.i.i), !dbg !26108
  %_2.i6.not.i.i.i = icmp eq i64 %..i.i.i.i.i.i, 0, !dbg !26114
  br i1 %_2.i6.not.i.i.i, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit.i.i, label %bb4.i1695.i.i, !dbg !26114

bb4.i1695.i.i:                                    ; preds = %bb13.i.i, %bb6.i1698.i.i
  %iter.sroa.8.07.i.i.i = phi i64 [ %44, %bb6.i1698.i.i ], [ 0, %bb13.i.i ]
  %_3.i1.i.i.i.i = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i.i.i, i64 %iter.sroa.8.07.i.i.i, !dbg !26117
  %_14.i1696.i.i = load i32, ptr %_3.i1.i.i.i.i, align 4, !dbg !26120, !noalias !26121, !noundef !12
  %_20.i1697.i.i = icmp eq i32 %_14.i1696.i.i, 0, !dbg !26122
  br i1 %_20.i1697.i.i, label %panic.i1707.i.i, label %bb6.i1698.i.i, !dbg !26122

bb6.i1698.i.i:                                    ; preds = %bb4.i1695.i.i
  %_3.i.i.i1699.i.i = getelementptr inbounds nuw i32, ptr %_40.0.i.i.i, i64 %iter.sroa.8.07.i.i.i, !dbg !26123
  %44 = add nuw i64 %iter.sroa.8.07.i.i.i, 1, !dbg !26126
  %window.i1700.i.i = zext i32 %_14.i1696.i.i to i64, !dbg !26120
  %_18.i1701.i.i = load i32, ptr %_3.i.i.i1699.i.i, align 4, !dbg !26127, !noalias !26121, !noundef !12
  %_17.i1702.i.i = zext i32 %_18.i1701.i.i to i64, !dbg !26127
  %_19.i1703.i.i = urem i64 %_31.i, %window.i1700.i.i, !dbg !26122
  %_16.i1704.i.i = add nuw nsw i64 %_19.i1703.i.i, %_17.i1702.i.i, !dbg !26128
  %_15.i1705.i.i = urem i64 %_16.i1704.i.i, %window.i1700.i.i, !dbg !26129
  %45 = trunc nuw i64 %_15.i1705.i.i to i32, !dbg !26130
  store i32 %45, ptr %_3.i.i.i1699.i.i, align 4, !dbg !26130, !noalias !26121
  %exitcond.not.i.i.i = icmp eq i64 %44, %..i.i.i.i.i.i, !dbg !26114
  br i1 %exitcond.not.i.i.i, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit.i.i, label %bb4.i1695.i.i, !dbg !26114

panic.i1707.i.i:                                  ; preds = %bb4.i1695.i.i
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1781ea1b97b96e9885c590e9b4440a45) #31, !dbg !26122, !noalias !26121
  unreachable, !dbg !26122

_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit.i.i: ; preds = %bb6.i1698.i.i, %bb13.i.i
  %46 = getelementptr inbounds nuw i8, ptr %self, i64 1624, !dbg !26131
  %_20.val.i.i = load i64, ptr %46, align 8, !dbg !26131, !alias.scope !25944, !noalias !25947
  %47 = getelementptr inbounds nuw i8, ptr %self, i64 1632, !dbg !26131
  %_20.val1657.i.i = load i64, ptr %47, align 8, !dbg !26131, !alias.scope !25944, !noalias !25947, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26132), !dbg !26131
  %_10.i1708.i.i = icmp eq i64 %_20.val1657.i.i, 0, !dbg !26135
  br i1 %_10.i1708.i.i, label %panic.i1722.i.i, label %bb1.i1709.i.i, !dbg !26135

bb1.i1709.i.i:                                    ; preds = %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit.i.i
  %_19.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1640, !dbg !26137
  %_7.i1710.i.i = load i32, ptr %_19.i.i, align 4, !dbg !26138, !alias.scope !26139, !noalias !25947, !noundef !12
  %_6.i1711.i.i = zext i32 %_7.i1710.i.i to i64, !dbg !26138
  %_8.i1712.i.i = urem i64 %_31.i, %_20.val1657.i.i, !dbg !26135
  %_5.i1713.i.i = add nuw nsw i64 %_8.i1712.i.i, %_6.i1711.i.i, !dbg !26140
  %_4.i1714.i.i = urem i64 %_5.i1713.i.i, %_20.val1657.i.i, !dbg !26141
  %48 = trunc i64 %_4.i1714.i.i to i32, !dbg !26142
  store i32 %48, ptr %_19.i.i, align 4, !dbg !26142, !alias.scope !26139, !noalias !25947
  %_17.i1715.i.i = icmp eq i64 %_20.val.i.i, 0, !dbg !26143
  br i1 %_17.i1715.i.i, label %panic2.i.i.i, label %_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit.i.i, !dbg !26143

panic.i1722.i.i:                                  ; preds = %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit.i.i
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6b076e9a9e313bc2481504f4da644a5c) #31, !dbg !26135, !noalias !26144
  unreachable, !dbg !26135

panic2.i.i.i:                                     ; preds = %bb1.i1709.i.i
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f370b9a38141751c9788be81259bacff) #31, !dbg !26143, !noalias !26144
  unreachable, !dbg !26143

_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit.i.i: ; preds = %bb1.i1709.i.i
  %49 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !26145
  %_14.i1717.i.i = load i32, ptr %49, align 4, !dbg !26145, !alias.scope !26139, !noalias !25947, !noundef !12
  %_13.i1718.i.i = zext i32 %_14.i1717.i.i to i64, !dbg !26145
  %_15.i1719.i.i = urem i64 %_31.i, %_20.val.i.i, !dbg !26143
  %_12.i1720.i.i = add nuw nsw i64 %_15.i1719.i.i, %_13.i1718.i.i, !dbg !26146
  %_11.i1721.i.i = urem i64 %_12.i1720.i.i, %_20.val.i.i, !dbg !26147
  %50 = trunc i64 %_11.i1721.i.i to i32, !dbg !26148
  store i32 %50, ptr %49, align 4, !dbg !26148, !alias.scope !26139, !noalias !25947
  br label %_RINvMsf_CsdvPQf9CMsz3_17true_peak_limiterINtB6_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb1_EB6_.exit, !dbg !26149

bb39.i.i:                                         ; preds = %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i.i.i, %bb2.i.i.i.i.i, %bb13.i.i1684.i.i, %bb13.i5.i.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26150), !dbg !26153
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26154), !dbg !26153
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26156), !dbg !26153
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_left.i216.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_25.i) #30, !dbg !26158, !noalias !25947
  %51 = getelementptr inbounds nuw i8, ptr %self, i64 1776, !dbg !26162
  %_108.0.i.i.i = load ptr, ptr %51, align 8, !dbg !26162, !alias.scope !26164, !noalias !26165, !nonnull !12, !noundef !12
  %52 = getelementptr inbounds nuw i8, ptr %self, i64 1784, !dbg !26162
  %_108.1.i.i.i = load i64, ptr %52, align 8, !dbg !26162, !alias.scope !26164, !noalias !26165, !noundef !12
  %_8.i1723.i.i = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_108.0.i.i.i, i64 %_108.1.i.i.i, !dbg !26168
  br label %bb1.i.i1724.i.i, !dbg !26173

bb1.i.i1724.i.i:                                  ; preds = %bb13.i.i1727.i.i, %bb39.i.i
  %_221.i.i1725.i.i = phi ptr [ %_22.i.i1728.i.i, %bb13.i.i1727.i.i ], [ %_108.0.i.i.i, %bb39.i.i ]
  %_12.i.i1726.i.i = icmp eq ptr %_221.i.i1725.i.i, %_8.i1723.i.i, !dbg !26175
  br i1 %_12.i.i1726.i.i, label %bb3.i289.i.i, label %bb13.i.i1727.i.i, !dbg !26178

bb13.i.i1727.i.i:                                 ; preds = %bb1.i.i1724.i.i
  %_22.i.i1728.i.i = getelementptr inbounds nuw i8, ptr %_221.i.i1725.i.i, i64 16, !dbg !26179
  %53 = getelementptr inbounds nuw i8, ptr %_221.i.i1725.i.i, i64 12, !dbg !26181
  %_3.i.i.i1729.i.i = load i32, ptr %53, align 4, !dbg !26181, !alias.scope !26183, !noalias !26188, !noundef !12
  %54 = icmp eq i32 %_3.i.i.i1729.i.i, 0, !dbg !26181
  %_51.i.i.i1730.i.i = load i32, ptr %_221.i.i1725.i.i, align 4, !dbg !26181, !alias.scope !26183, !noalias !26188
  %55 = getelementptr inbounds nuw i8, ptr %_221.i.i1725.i.i, i64 4, !dbg !26181
  %_72.i.i.i1731.i.i = load i32, ptr %55, align 4, !dbg !26181, !alias.scope !26183, !noalias !26188
  %56 = icmp eq i32 %_51.i.i.i1730.i.i, %_72.i.i.i1731.i.i, !dbg !26181
  %_0.sroa.0.0.i.i.i1732.i.i = select i1 %54, i1 %56, i1 false, !dbg !26181
  br i1 %_0.sroa.0.0.i.i.i1732.i.i, label %bb1.i.i1724.i.i, label %bb6.i218.i.i, !dbg !26191

bb3.i289.i.i:                                     ; preds = %bb1.i.i1724.i.i
  %57 = getelementptr inbounds nuw i8, ptr %self, i64 1792, !dbg !26192
  %_109.0.i.i.i = load ptr, ptr %57, align 8, !dbg !26192, !alias.scope !26164, !noalias !26165, !nonnull !12, !noundef !12
  %58 = getelementptr inbounds nuw i8, ptr %self, i64 1800, !dbg !26192
  %_109.1.i.i.i = load i64, ptr %58, align 8, !dbg !26192, !alias.scope !26164, !noalias !26165, !noundef !12
  %_8.i1734.i.i = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_109.0.i.i.i, i64 %_109.1.i.i.i, !dbg !26193
  br label %bb1.i.i1735.i.i, !dbg !26198

bb1.i.i1735.i.i:                                  ; preds = %bb13.i.i1738.i.i, %bb3.i289.i.i
  %_221.i.i1736.i.i = phi ptr [ %_22.i.i1739.i.i, %bb13.i.i1738.i.i ], [ %_109.0.i.i.i, %bb3.i289.i.i ]
  %_12.i.i1737.i.i = icmp eq ptr %_221.i.i1736.i.i, %_8.i1734.i.i, !dbg !26200
  br i1 %_12.i.i1737.i.i, label %bb6.i218.i.i, label %bb13.i.i1738.i.i, !dbg !26203

bb13.i.i1738.i.i:                                 ; preds = %bb1.i.i1735.i.i
  %_22.i.i1739.i.i = getelementptr inbounds nuw i8, ptr %_221.i.i1736.i.i, i64 16, !dbg !26204
  %59 = getelementptr inbounds nuw i8, ptr %_221.i.i1736.i.i, i64 12, !dbg !26206
  %_3.i.i.i1740.i.i = load i32, ptr %59, align 4, !dbg !26206, !alias.scope !26208, !noalias !26213, !noundef !12
  %60 = icmp eq i32 %_3.i.i.i1740.i.i, 0, !dbg !26206
  %_51.i.i.i1741.i.i = load i32, ptr %_221.i.i1736.i.i, align 4, !dbg !26206, !alias.scope !26208, !noalias !26213
  %61 = getelementptr inbounds nuw i8, ptr %_221.i.i1736.i.i, i64 4, !dbg !26206
  %_72.i.i.i1742.i.i = load i32, ptr %61, align 4, !dbg !26206, !alias.scope !26208, !noalias !26213
  %62 = icmp eq i32 %_51.i.i.i1741.i.i, %_72.i.i.i1742.i.i, !dbg !26206
  %_0.sroa.0.0.i.i.i1743.i.i = select i1 %60, i1 %62, i1 false, !dbg !26206
  br i1 %_0.sroa.0.0.i.i.i1743.i.i, label %bb1.i.i1735.i.i, label %bb6.i218.i.i, !dbg !26216

bb6.i218.i.i:                                     ; preds = %bb13.i.i1727.i.i, %bb13.i.i1738.i.i, %bb1.i.i1735.i.i
  %stationary.sroa.0.0.i219.i.i = phi i1 [ false, %bb13.i.i1738.i.i ], [ true, %bb1.i.i1735.i.i ], [ false, %bb13.i.i1727.i.i ], !dbg !26217
  %63 = getelementptr inbounds nuw i8, ptr %self, i64 1536, !dbg !26218
  %64 = load i8, ptr %63, align 32, !dbg !26218, !range !5399, !alias.scope !26222, !noalias !26223, !noundef !12
  %65 = getelementptr inbounds nuw i8, ptr %self, i64 1537, !dbg !26224
  %66 = load i8, ptr %65, align 1, !dbg !26224, !range !5399, !alias.scope !26222, !noalias !26223, !noundef !12
  %_27.i.i.i = load i32, ptr %_24.i.i, align 4, !dbg !26226, !alias.scope !26228, !noalias !26229, !noundef !12
  %67 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !26230
  %_29.i226.i.i = load i32, ptr %67, align 4, !dbg !26230, !alias.scope !26228, !noalias !26229, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i.i.i), !dbg !26232, !noalias !26234
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i.i.i, i8 0, i64 32, i1 false), !noalias !26234
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i210.i.i), !dbg !26235, !noalias !26234
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i210.i.i, i8 0, i64 1024, i1 false), !noalias !26234
  %68 = add nuw nsw i64 %_31.i, 31, !dbg !26237
  %yield_count.sroa.0.0.i.i.i.i = lshr i64 %68, 5, !dbg !26237
  %_80.not.i3823.i.i = icmp eq i64 %yield_count.sroa.0.0.i.i.i.i, 0, !dbg !26244
  br i1 %_80.not.i3823.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i, label %bb38.i.lr.ph.i.i, !dbg !26244

bb38.i.lr.ph.i.i:                                 ; preds = %bb6.i218.i.i
  %69 = zext i32 %_29.i226.i.i to i64, !dbg !26230
  %70 = zext i32 %_27.i.i.i to i64, !dbg !26226
  %_25.i223.i.i = trunc nuw i8 %66 to i1, !dbg !26224
  %_24.i220.i.i = trunc nuw i8 %64 to i1, !dbg !26218
  %71 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !26253
  %72 = bitcast <8 x float> %71 to <8 x i32>, !dbg !26259
  %73 = xor <8 x i32> %72, splat (i32 -1), !dbg !26265
  %history.i.i189.sroa.10.0.hot_left.i216.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i216.i.i, i64 32
  %history.i.i189.sroa.13.0.hot_left.i216.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i216.i.i, i64 64
  %history.i.i189.sroa.16.0.hot_left.i216.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i216.i.i, i64 96
  %history.i.i189.sroa.19.0.hot_left.i216.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i216.i.i, i64 128
  %history.i.i189.sroa.22.0.hot_left.i216.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i216.i.i, i64 160
  %history.i.i189.sroa.25.0.hot_left.i216.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i216.i.i, i64 192
  %history.i.i189.sroa.29.0.hot_left.i216.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i216.i.i, i64 224
  %history.i.i189.sroa.32.0.hot_left.i216.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i216.i.i, i64 256
  %history.i.i189.sroa.35.0.hot_left.i216.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i216.i.i, i64 288
  %history.i.i189.sroa.38.0.hot_left.i216.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i216.i.i, i64 320
  %history.i.i189.sroa.41.0.hot_left.i216.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i216.i.i, i64 352
  %74 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %75 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %76 = getelementptr inbounds nuw i8, ptr %self, i64 96
  %row12.i.i.i.i264.i.i = getelementptr inbounds nuw i8, ptr %self, i64 128
  %77 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %78 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %79 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %row13.i.i.i.i265.i.i = getelementptr inbounds nuw i8, ptr %self, i64 256
  %80 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %81 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %82 = getelementptr inbounds nuw i8, ptr %self, i64 352
  %row14.i.i.i.i266.i.i = getelementptr inbounds nuw i8, ptr %self, i64 384
  %83 = getelementptr inbounds nuw i8, ptr %self, i64 416
  %84 = getelementptr inbounds nuw i8, ptr %self, i64 448
  %85 = getelementptr inbounds nuw i8, ptr %self, i64 480
  %row15.i.i.i.i267.i.i = getelementptr inbounds nuw i8, ptr %self, i64 512
  %86 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %87 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %88 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %row16.i.i.i.i268.i.i = getelementptr inbounds nuw i8, ptr %self, i64 640
  %89 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %90 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %91 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %row17.i.i.i.i269.i.i = getelementptr inbounds nuw i8, ptr %self, i64 768
  %92 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %93 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %94 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %row18.i.i.i.i270.i.i = getelementptr inbounds nuw i8, ptr %self, i64 896
  %95 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %96 = getelementptr inbounds nuw i8, ptr %self, i64 960
  %97 = getelementptr inbounds nuw i8, ptr %self, i64 992
  %row19.i.i.i.i271.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %98 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %99 = getelementptr inbounds nuw i8, ptr %self, i64 1088
  %100 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %row20.i.i.i.i272.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %101 = getelementptr inbounds nuw i8, ptr %self, i64 1184
  %102 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %103 = getelementptr inbounds nuw i8, ptr %self, i64 1248
  %row21.i.i.i.i273.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1280
  %104 = getelementptr inbounds nuw i8, ptr %self, i64 1312
  %105 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %106 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %row22.i.i.i.i274.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1408
  %107 = getelementptr inbounds nuw i8, ptr %self, i64 1440
  %108 = getelementptr inbounds nuw i8, ptr %self, i64 1472
  %109 = getelementptr inbounds nuw i8, ptr %self, i64 1504
  %_49.i244.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i216.i.i, i64 384
  %110 = getelementptr inbounds nuw i8, ptr %hot_left.i216.i.i, i64 480
  %111 = getelementptr inbounds nuw i8, ptr %hot_left.i216.i.i, i64 448
  %112 = getelementptr inbounds nuw i8, ptr %hot_left.i216.i.i, i64 416
  %_51.i245.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i216.i.i, i64 512
  %113 = getelementptr inbounds nuw i8, ptr %hot_left.i216.i.i, i64 608
  %114 = getelementptr inbounds nuw i8, ptr %hot_left.i216.i.i, i64 576
  %115 = getelementptr inbounds nuw i8, ptr %hot_left.i216.i.i, i64 544
  %116 = select i1 %_24.i220.i.i, <8 x i32> %72, <8 x i32> %73
  %117 = icmp slt <8 x i32> %116, zeroinitializer
  %118 = getelementptr inbounds nuw i8, ptr %self, i64 1624
  %119 = getelementptr inbounds nuw i8, ptr %self, i64 1840
  %120 = getelementptr inbounds nuw i8, ptr %self, i64 1688
  %121 = getelementptr inbounds nuw i8, ptr %self, i64 1680
  %122 = getelementptr inbounds nuw i8, ptr %self, i64 1768
  %123 = getelementptr inbounds nuw i8, ptr %self, i64 1760
  %124 = getelementptr inbounds nuw i8, ptr %self, i64 1736
  %125 = getelementptr inbounds nuw i8, ptr %self, i64 1728
  %126 = getelementptr inbounds nuw i8, ptr %self, i64 1704
  %127 = getelementptr inbounds nuw i8, ptr %self, i64 1696
  %128 = getelementptr inbounds nuw i8, ptr %hot_left.i216.i.i, i64 672
  %129 = getelementptr inbounds nuw i8, ptr %hot_left.i216.i.i, i64 704
  %130 = getelementptr inbounds nuw i8, ptr %hot_left.i216.i.i, i64 640
  %131 = getelementptr inbounds nuw i8, ptr %self, i64 1672
  %132 = getelementptr inbounds nuw i8, ptr %self, i64 1664
  %133 = select i1 %_25.i223.i.i, <8 x i32> %72, <8 x i32> %73
  %134 = icmp slt <8 x i32> %133, zeroinitializer
  %135 = getelementptr inbounds nuw i8, ptr %self, i64 1632
  %history.i.i189.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %hot_left.i216.i.i, align 32, !dbg !26267, !noalias !26271
  %history.i.i189.sroa.10.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %history.i.i189.sroa.10.0.hot_left.i216.sroa_idx.i.i, align 32, !dbg !26267, !noalias !26271
  %history.i.i189.sroa.13.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %history.i.i189.sroa.13.0.hot_left.i216.sroa_idx.i.i, align 32, !dbg !26267, !noalias !26271
  %history.i.i189.sroa.16.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %history.i.i189.sroa.16.0.hot_left.i216.sroa_idx.i.i, align 32, !dbg !26267, !noalias !26271
  %history.i.i189.sroa.19.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %history.i.i189.sroa.19.0.hot_left.i216.sroa_idx.i.i, align 32, !dbg !26267, !noalias !26271
  %history.i.i189.sroa.22.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %history.i.i189.sroa.22.0.hot_left.i216.sroa_idx.i.i, align 32, !dbg !26267, !noalias !26271
  %history.i.i189.sroa.25.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %history.i.i189.sroa.25.0.hot_left.i216.sroa_idx.i.i, align 32, !dbg !26267, !noalias !26271
  %history.i.i189.sroa.29.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %history.i.i189.sroa.29.0.hot_left.i216.sroa_idx.i.i, align 32, !dbg !26267, !noalias !26271
  %history.i.i189.sroa.32.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %history.i.i189.sroa.32.0.hot_left.i216.sroa_idx.i.i, align 32, !dbg !26267, !noalias !26271
  %history.i.i189.sroa.35.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %history.i.i189.sroa.35.0.hot_left.i216.sroa_idx.i.i, align 32, !dbg !26267, !noalias !26271
  %history.i.i189.sroa.38.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %history.i.i189.sroa.38.0.hot_left.i216.sroa_idx.i.i, align 32, !dbg !26267, !noalias !26271
  %history.i.i189.sroa.41.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %history.i.i189.sroa.41.0.hot_left.i216.sroa_idx.i.i, align 32, !dbg !26267, !noalias !26271
  %iter.i.i.sroa.0.0.ptr3764.1.i.i = getelementptr inbounds nuw i8, ptr %scratch.i.i.i, i64 4
  %iter.i.i.sroa.0.0.ptr3764.2.i.i = getelementptr inbounds nuw i8, ptr %scratch.i.i.i, i64 8
  %iter.i.i.sroa.0.0.ptr3764.3.i.i = getelementptr inbounds nuw i8, ptr %scratch.i.i.i, i64 12
  %iter.i.i.sroa.0.0.ptr3764.4.i.i = getelementptr inbounds nuw i8, ptr %scratch.i.i.i, i64 16
  %iter.i.i.sroa.0.0.ptr3764.5.i.i = getelementptr inbounds nuw i8, ptr %scratch.i.i.i, i64 20
  %iter.i.i.sroa.0.0.ptr3764.6.i.i = getelementptr inbounds nuw i8, ptr %scratch.i.i.i, i64 24
  %iter.i.i.sroa.0.0.ptr3764.7.i.i = getelementptr inbounds nuw i8, ptr %scratch.i.i.i, i64 28
  br label %bb38.i.i.i, !dbg !26244

bb19.i.bb17.i.loopexit_crit_edge.i.i:             ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1532.i.i
  store <8 x float> %269, ptr %114, align 1, !dbg !26272
  store <8 x float> %309, ptr %128, align 1, !dbg !26277
  store <8 x float> %271, ptr %113, align 1, !dbg !26283, !noalias !25922
  store <8 x float> %270, ptr %_51.i245.i.i, align 1, !dbg !26284, !noalias !25922
  store <8 x float> %273, ptr %_49.i244.i.i, align 1, !dbg !26285, !noalias !26271
  store <8 x float> %272, ptr %111, align 1, !dbg !26287, !noalias !26271
  store <8 x float> %275, ptr %110, align 32, !dbg !26288, !noalias !26271
  br label %bb17.i.loopexit.i.i, !dbg !26289

bb17.i.loopexit.i.i:                              ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i237.i.i, %bb19.i.bb17.i.loopexit_crit_edge.i.i
  %ring_cursor.sroa.0.1.i239.lcssa.i.i = phi i64 [ %spec.store.select9.i.i.i, %bb19.i.bb17.i.loopexit_crit_edge.i.i ], [ %ring_cursor.sroa.0.0.i2303826.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i237.i.i ], !dbg !26295
  %main_cursor.sroa.0.1.i240.lcssa.i.i = phi i64 [ %spec.store.select.i.i.i, %bb19.i.bb17.i.loopexit_crit_edge.i.i ], [ %main_cursor.sroa.0.0.i2313827.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i237.i.i ], !dbg !26296
  %_80.not.i.i.i = icmp eq i64 %138, 0, !dbg !26244
  %indvars.iv.next.i.i = add nsw i64 %indvars.iv.i.i, -32, !dbg !26244
  br i1 %_80.not.i.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i.i, label %bb38.i.i.i, !dbg !26244

bb38.i.i.i:                                       ; preds = %bb17.i.loopexit.i.i, %bb38.i.lr.ph.i.i
  %history.i.i189.sroa.41.sroa.0.0.copyload.i.i = phi <8 x float> [ %history.i.i189.sroa.41.sroa.0.0.copyload.pre.i.i, %bb38.i.lr.ph.i.i ], [ %history.i.i189.sroa.41.sroa.0.0.lcssa.i.i, %bb17.i.loopexit.i.i ], !dbg !26267
  %history.i.i189.sroa.38.sroa.0.0.copyload.i.i = phi <8 x float> [ %history.i.i189.sroa.38.sroa.0.0.copyload.pre.i.i, %bb38.i.lr.ph.i.i ], [ %history.i.i189.sroa.38.sroa.0.0.lcssa.i.i, %bb17.i.loopexit.i.i ], !dbg !26267
  %history.i.i189.sroa.35.sroa.0.0.copyload.i.i = phi <8 x float> [ %history.i.i189.sroa.35.sroa.0.0.copyload.pre.i.i, %bb38.i.lr.ph.i.i ], [ %history.i.i189.sroa.35.sroa.0.0.lcssa.i.i, %bb17.i.loopexit.i.i ], !dbg !26267
  %history.i.i189.sroa.32.sroa.0.0.copyload.i.i = phi <8 x float> [ %history.i.i189.sroa.32.sroa.0.0.copyload.pre.i.i, %bb38.i.lr.ph.i.i ], [ %history.i.i189.sroa.32.sroa.0.0.lcssa.i.i, %bb17.i.loopexit.i.i ], !dbg !26267
  %history.i.i189.sroa.29.sroa.0.0.copyload.i.i = phi <8 x float> [ %history.i.i189.sroa.29.sroa.0.0.copyload.pre.i.i, %bb38.i.lr.ph.i.i ], [ %history.i.i189.sroa.29.sroa.0.0.lcssa.i.i, %bb17.i.loopexit.i.i ], !dbg !26267
  %history.i.i189.sroa.25.sroa.0.0.copyload.i.i = phi <8 x float> [ %history.i.i189.sroa.25.sroa.0.0.copyload.pre.i.i, %bb38.i.lr.ph.i.i ], [ %history.i.i189.sroa.25.sroa.0.0.lcssa.i.i, %bb17.i.loopexit.i.i ], !dbg !26267
  %history.i.i189.sroa.22.sroa.0.0.copyload.i.i = phi <8 x float> [ %history.i.i189.sroa.22.sroa.0.0.copyload.pre.i.i, %bb38.i.lr.ph.i.i ], [ %history.i.i189.sroa.22.sroa.0.0.lcssa.i.i, %bb17.i.loopexit.i.i ], !dbg !26267
  %history.i.i189.sroa.19.sroa.0.0.copyload.i.i = phi <8 x float> [ %history.i.i189.sroa.19.sroa.0.0.copyload.pre.i.i, %bb38.i.lr.ph.i.i ], [ %history.i.i189.sroa.19.sroa.0.0.lcssa.i.i, %bb17.i.loopexit.i.i ], !dbg !26267
  %history.i.i189.sroa.16.sroa.0.0.copyload.i.i = phi <8 x float> [ %history.i.i189.sroa.16.sroa.0.0.copyload.pre.i.i, %bb38.i.lr.ph.i.i ], [ %history.i.i189.sroa.16.sroa.0.0.lcssa.i.i, %bb17.i.loopexit.i.i ], !dbg !26267
  %history.i.i189.sroa.13.sroa.0.0.copyload.i.i = phi <8 x float> [ %history.i.i189.sroa.13.sroa.0.0.copyload.pre.i.i, %bb38.i.lr.ph.i.i ], [ %history.i.i189.sroa.13.sroa.0.0.lcssa.i.i, %bb17.i.loopexit.i.i ], !dbg !26267
  %history.i.i189.sroa.10.sroa.0.0.copyload.i.i = phi <8 x float> [ %history.i.i189.sroa.10.sroa.0.0.copyload.pre.i.i, %bb38.i.lr.ph.i.i ], [ %history.i.i189.sroa.10.sroa.0.0.lcssa.i.i, %bb17.i.loopexit.i.i ], !dbg !26267
  %history.i.i189.sroa.0.0.copyload.i.i = phi <8 x float> [ %history.i.i189.sroa.0.0.copyload.pre.i.i, %bb38.i.lr.ph.i.i ], [ %history.i.i189.sroa.0.0.lcssa.i.i, %bb17.i.loopexit.i.i ], !dbg !26267
  %indvars.iv.i.i = phi i64 [ %_31.i, %bb38.i.lr.ph.i.i ], [ %indvars.iv.next.i.i, %bb17.i.loopexit.i.i ]
  %main_cursor.sroa.0.0.i2313827.i.i = phi i64 [ %70, %bb38.i.lr.ph.i.i ], [ %main_cursor.sroa.0.1.i240.lcssa.i.i, %bb17.i.loopexit.i.i ]
  %ring_cursor.sroa.0.0.i2303826.i.i = phi i64 [ %69, %bb38.i.lr.ph.i.i ], [ %ring_cursor.sroa.0.1.i239.lcssa.i.i, %bb17.i.loopexit.i.i ]
  %iter3.sroa.0.0.i2293825.i.i = phi i64 [ %yield_count.sroa.0.0.i.i.i.i, %bb38.i.lr.ph.i.i ], [ %138, %bb17.i.loopexit.i.i ]
  %iter.sroa.0.0.i3824.i.i = phi i64 [ 0, %bb38.i.lr.ph.i.i ], [ %137, %bb17.i.loopexit.i.i ]
  %136 = tail call i64 @llvm.umax.i64(i64 %indvars.iv.i.i, i64 1), !dbg !26297
  %umax4429.i.i = tail call i64 @llvm.umin.i64(i64 %136, i64 32), !dbg !26297
  %137 = add nuw nsw i64 %iter.sroa.0.0.i3824.i.i, 32, !dbg !26297
  %138 = add nsw i64 %iter3.sroa.0.0.i2293825.i.i, -1, !dbg !26301
  %_20.i.i2363730.not.i.i = icmp eq i64 %iter.sroa.0.0.i3824.i.i, %_31.i, !dbg !26302
  br i1 %_20.i.i2363730.not.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i237.i.i, label %bb5.i.i258.lr.ph.i.i, !dbg !26306

bb5.i.i258.lr.ph.i.i:                             ; preds = %bb38.i.i.i
  %_5.i1043.i.i = load <8 x float>, ptr %self, align 32, !alias.scope !25944, !noalias !25947
  %_14.i.i.i.i154.sroa.0.0.copyload.i.i = load <8 x float>, ptr %74, align 32, !alias.scope !25944, !noalias !25947
  %_17.i.i.i.i151.sroa.0.0.copyload.i.i = load <8 x float>, ptr %75, align 32, !alias.scope !25944, !noalias !25947
  %_20.i.i.i.i148.sroa.0.0.copyload.i.i = load <8 x float>, ptr %76, align 32, !alias.scope !25944, !noalias !25947
  %_25.i.i.i.i144.sroa.0.0.copyload.i.i = load <8 x float>, ptr %row12.i.i.i.i264.i.i, align 32, !alias.scope !25944, !noalias !25947
  %_28.i.i.i.i141.sroa.0.0.copyload.i.i = load <8 x float>, ptr %77, align 32, !alias.scope !25944, !noalias !25947
  %_31.i.i.i.i138.sroa.0.0.copyload.i.i = load <8 x float>, ptr %78, align 32, !alias.scope !25944, !noalias !25947
  %_34.i.i.i.i135.sroa.0.0.copyload.i.i = load <8 x float>, ptr %79, align 32, !alias.scope !25944, !noalias !25947
  %_39.i.i.i.i131.sroa.0.0.copyload.i.i = load <8 x float>, ptr %row13.i.i.i.i265.i.i, align 32, !alias.scope !25944, !noalias !25947
  %_42.i.i.i.i128.sroa.0.0.copyload.i.i = load <8 x float>, ptr %80, align 32, !alias.scope !25944, !noalias !25947
  %_45.i.i.i.i125.sroa.0.0.copyload.i.i = load <8 x float>, ptr %81, align 32, !alias.scope !25944, !noalias !25947
  %_48.i.i.i.i122.sroa.0.0.copyload.i.i = load <8 x float>, ptr %82, align 32, !alias.scope !25944, !noalias !25947
  %_53.i.i.i.i118.sroa.0.0.copyload.i.i = load <8 x float>, ptr %row14.i.i.i.i266.i.i, align 32, !alias.scope !25944, !noalias !25947
  %_56.i.i.i.i115.sroa.0.0.copyload.i.i = load <8 x float>, ptr %83, align 32, !alias.scope !25944, !noalias !25947
  %_59.i.i.i.i112.sroa.0.0.copyload.i.i = load <8 x float>, ptr %84, align 32, !alias.scope !25944, !noalias !25947
  %_62.i.i.i.i109.sroa.0.0.copyload.i.i = load <8 x float>, ptr %85, align 32, !alias.scope !25944, !noalias !25947
  %_67.i.i.i.i105.sroa.0.0.copyload.i.i = load <8 x float>, ptr %row15.i.i.i.i267.i.i, align 32, !alias.scope !25944, !noalias !25947
  %_70.i.i.i.i102.sroa.0.0.copyload.i.i = load <8 x float>, ptr %86, align 32, !alias.scope !25944, !noalias !25947
  %_73.i.i.i.i99.sroa.0.0.copyload.i.i = load <8 x float>, ptr %87, align 32, !alias.scope !25944, !noalias !25947
  %_76.i.i.i.i96.sroa.0.0.copyload.i.i = load <8 x float>, ptr %88, align 32, !alias.scope !25944, !noalias !25947
  %_81.i.i.i.i92.sroa.0.0.copyload.i.i = load <8 x float>, ptr %row16.i.i.i.i268.i.i, align 32, !alias.scope !25944, !noalias !25947
  %_84.i.i.i.i89.sroa.0.0.copyload.i.i = load <8 x float>, ptr %89, align 32, !alias.scope !25944, !noalias !25947
  %_87.i.i.i.i86.sroa.0.0.copyload.i.i = load <8 x float>, ptr %90, align 32, !alias.scope !25944, !noalias !25947
  %_90.i.i.i.i83.sroa.0.0.copyload.i.i = load <8 x float>, ptr %91, align 32, !alias.scope !25944, !noalias !25947
  %_95.i.i.i.i79.sroa.0.0.copyload.i.i = load <8 x float>, ptr %row17.i.i.i.i269.i.i, align 32, !alias.scope !25944, !noalias !25947
  %_98.i.i.i.i76.sroa.0.0.copyload.i.i = load <8 x float>, ptr %92, align 32, !alias.scope !25944, !noalias !25947
  %_101.i.i.i.i73.sroa.0.0.copyload.i.i = load <8 x float>, ptr %93, align 32, !alias.scope !25944, !noalias !25947
  %_104.i.i.i.i70.sroa.0.0.copyload.i.i = load <8 x float>, ptr %94, align 32, !alias.scope !25944, !noalias !25947
  %_109.i.i.i.i66.sroa.0.0.copyload.i.i = load <8 x float>, ptr %row18.i.i.i.i270.i.i, align 32, !alias.scope !25944, !noalias !25947
  %_112.i.i.i.i63.sroa.0.0.copyload.i.i = load <8 x float>, ptr %95, align 32, !alias.scope !25944, !noalias !25947
  %_115.i.i.i.i60.sroa.0.0.copyload.i.i = load <8 x float>, ptr %96, align 32, !alias.scope !25944, !noalias !25947
  %_118.i.i.i.i57.sroa.0.0.copyload.i.i = load <8 x float>, ptr %97, align 32, !alias.scope !25944, !noalias !25947
  %_123.i.i.i.i53.sroa.0.0.copyload.i.i = load <8 x float>, ptr %row19.i.i.i.i271.i.i, align 32, !alias.scope !25944, !noalias !25947
  %_126.i.i.i.i50.sroa.0.0.copyload.i.i = load <8 x float>, ptr %98, align 32, !alias.scope !25944, !noalias !25947
  %_129.i.i.i.i47.sroa.0.0.copyload.i.i = load <8 x float>, ptr %99, align 32, !alias.scope !25944, !noalias !25947
  %_132.i.i.i.i44.sroa.0.0.copyload.i.i = load <8 x float>, ptr %100, align 32, !alias.scope !25944, !noalias !25947
  %_137.i.i.i.i40.sroa.0.0.copyload.i.i = load <8 x float>, ptr %row20.i.i.i.i272.i.i, align 32, !alias.scope !25944, !noalias !25947
  %_140.i.i.i.i37.sroa.0.0.copyload.i.i = load <8 x float>, ptr %101, align 32, !alias.scope !25944, !noalias !25947
  %_143.i.i.i.i34.sroa.0.0.copyload.i.i = load <8 x float>, ptr %102, align 32, !alias.scope !25944, !noalias !25947
  %_146.i.i.i.i31.sroa.0.0.copyload.i.i = load <8 x float>, ptr %103, align 32, !alias.scope !25944, !noalias !25947
  %_151.i.i.i.i27.sroa.0.0.copyload.i.i = load <8 x float>, ptr %row21.i.i.i.i273.i.i, align 32, !alias.scope !25944, !noalias !25947
  %_154.i.i.i.i24.sroa.0.0.copyload.i.i = load <8 x float>, ptr %104, align 32, !alias.scope !25944, !noalias !25947
  %_157.i.i.i.i21.sroa.0.0.copyload.i.i = load <8 x float>, ptr %105, align 32, !alias.scope !25944, !noalias !25947
  %_160.i.i.i.i18.sroa.0.0.copyload.i.i = load <8 x float>, ptr %106, align 32, !alias.scope !25944, !noalias !25947
  %_165.i.i.i.i14.sroa.0.0.copyload.i.i = load <8 x float>, ptr %row22.i.i.i.i274.i.i, align 32, !alias.scope !25944, !noalias !25947
  %_168.i.i.i.i11.sroa.0.0.copyload.i.i = load <8 x float>, ptr %107, align 32, !alias.scope !25944, !noalias !25947
  %_171.i.i.i.i8.sroa.0.0.copyload.i.i = load <8 x float>, ptr %108, align 32, !alias.scope !25944, !noalias !25947
  %_174.i.i.i.i5.sroa.0.0.copyload.i.i = load <8 x float>, ptr %109, align 32, !alias.scope !25944, !noalias !25947
  br label %bb5.i.i258.i.i, !dbg !26306

bb5.i.i258.i.i:                                   ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i, %bb5.i.i258.lr.ph.i.i
  %iter.sroa.0.0.i.i2353742.i.i = phi i64 [ 0, %bb5.i.i258.lr.ph.i.i ], [ %139, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i ]
  %history.i.i189.sroa.10.sroa.0.03741.i.i = phi <8 x float> [ %history.i.i189.sroa.10.sroa.0.0.copyload.i.i, %bb5.i.i258.lr.ph.i.i ], [ %history.i.i189.sroa.0.03731.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i ]
  %history.i.i189.sroa.13.sroa.0.03740.i.i = phi <8 x float> [ %history.i.i189.sroa.13.sroa.0.0.copyload.i.i, %bb5.i.i258.lr.ph.i.i ], [ %history.i.i189.sroa.10.sroa.0.03741.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i ]
  %history.i.i189.sroa.16.sroa.0.03739.i.i = phi <8 x float> [ %history.i.i189.sroa.16.sroa.0.0.copyload.i.i, %bb5.i.i258.lr.ph.i.i ], [ %history.i.i189.sroa.13.sroa.0.03740.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i ]
  %history.i.i189.sroa.19.sroa.0.03738.i.i = phi <8 x float> [ %history.i.i189.sroa.19.sroa.0.0.copyload.i.i, %bb5.i.i258.lr.ph.i.i ], [ %history.i.i189.sroa.16.sroa.0.03739.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i ]
  %history.i.i189.sroa.22.sroa.0.03737.i.i = phi <8 x float> [ %history.i.i189.sroa.22.sroa.0.0.copyload.i.i, %bb5.i.i258.lr.ph.i.i ], [ %history.i.i189.sroa.19.sroa.0.03738.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i ]
  %history.i.i189.sroa.38.sroa.0.03736.i.i = phi <8 x float> [ %history.i.i189.sroa.38.sroa.0.0.copyload.i.i, %bb5.i.i258.lr.ph.i.i ], [ %history.i.i189.sroa.35.sroa.0.03735.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i ]
  %history.i.i189.sroa.35.sroa.0.03735.i.i = phi <8 x float> [ %history.i.i189.sroa.35.sroa.0.0.copyload.i.i, %bb5.i.i258.lr.ph.i.i ], [ %history.i.i189.sroa.32.sroa.0.03734.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i ]
  %history.i.i189.sroa.32.sroa.0.03734.i.i = phi <8 x float> [ %history.i.i189.sroa.32.sroa.0.0.copyload.i.i, %bb5.i.i258.lr.ph.i.i ], [ %history.i.i189.sroa.29.sroa.0.03733.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i ]
  %history.i.i189.sroa.29.sroa.0.03733.i.i = phi <8 x float> [ %history.i.i189.sroa.29.sroa.0.0.copyload.i.i, %bb5.i.i258.lr.ph.i.i ], [ %history.i.i189.sroa.25.sroa.0.03732.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i ]
  %history.i.i189.sroa.25.sroa.0.03732.i.i = phi <8 x float> [ %history.i.i189.sroa.25.sroa.0.0.copyload.i.i, %bb5.i.i258.lr.ph.i.i ], [ %history.i.i189.sroa.22.sroa.0.03737.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i ]
  %history.i.i189.sroa.0.03731.i.i = phi <8 x float> [ %history.i.i189.sroa.0.0.copyload.i.i, %bb5.i.i258.lr.ph.i.i ], [ %lanes.i1293.sroa.0.0.copyload.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i ]
  %139 = add nuw nsw i64 %iter.sroa.0.0.i.i2353742.i.i, 1, !dbg !26307
  %_11.i.i259.i.i = add nuw nsw i64 %iter.sroa.0.0.i.i2353742.i.i, %iter.sroa.0.0.i3824.i.i, !dbg !26310
  %base.i.i260.i.i = shl i64 %_11.i.i259.i.i, 3, !dbg !26310
  %_24.i.i261.i.i = icmp samesign ugt i64 %base.i.i260.i.i, %_39.1.i, !dbg !26311
  br i1 %_24.i.i261.i.i, label %bb7.i.i286.i.i, label %bb8.i.i262.i.i, !dbg !26311, !prof !905

bb8.i.i262.i.i:                                   ; preds = %bb5.i.i258.i.i
  %_27.i.i263.i.i = sub nuw nsw i64 %_39.1.i, %base.i.i260.i.i, !dbg !26314
  %_8.i1296.i.i = icmp samesign ugt i64 %_27.i.i263.i.i, 7, !dbg !26315
  br i1 %_8.i1296.i.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i, label %bb2.i1297.i.i, !dbg !26315, !prof !1076

bb2.i1297.i.i:                                    ; preds = %bb8.i.i262.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_27.i.i263.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #31, !dbg !26320, !noalias !26321
  unreachable, !dbg !26320

bb7.i.i286.i.i:                                   ; preds = %bb5.i.i258.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i.i260.i.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e0592aef22128a0ac53753b9632a8183) #31, !dbg !26328, !noalias !26329
  unreachable, !dbg !26328

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i: ; preds = %bb8.i.i262.i.i
  %_31.i19.i.i.i = getelementptr inbounds nuw float, ptr %_39.0.i, i64 %base.i.i260.i.i, !dbg !26330
  %lanes.i1293.sroa.0.0.copyload.i.i = load <8 x float>, ptr %_31.i19.i.i.i, align 4, !dbg !26332, !alias.scope !26336, !noalias !26340
  %140 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i.i189.sroa.22.sroa.0.03737.i.i), !dbg !26342
  %141 = fmul <8 x float> %_5.i1043.i.i, %lanes.i1293.sroa.0.0.copyload.i.i, !dbg !26349
  %142 = fadd <8 x float> %141, zeroinitializer, !dbg !26355
  %143 = fmul <8 x float> %_14.i.i.i.i154.sroa.0.0.copyload.i.i, %lanes.i1293.sroa.0.0.copyload.i.i, !dbg !26360
  %144 = fadd <8 x float> %143, zeroinitializer, !dbg !26365
  %145 = fmul <8 x float> %_17.i.i.i.i151.sroa.0.0.copyload.i.i, %lanes.i1293.sroa.0.0.copyload.i.i, !dbg !26370
  %146 = fadd <8 x float> %145, zeroinitializer, !dbg !26375
  %147 = fmul <8 x float> %_20.i.i.i.i148.sroa.0.0.copyload.i.i, %lanes.i1293.sroa.0.0.copyload.i.i, !dbg !26380
  %148 = fadd <8 x float> %147, zeroinitializer, !dbg !26385
  %149 = fmul <8 x float> %_25.i.i.i.i144.sroa.0.0.copyload.i.i, %history.i.i189.sroa.0.03731.i.i, !dbg !26390
  %150 = fadd <8 x float> %149, %142, !dbg !26395
  %151 = fmul <8 x float> %_28.i.i.i.i141.sroa.0.0.copyload.i.i, %history.i.i189.sroa.0.03731.i.i, !dbg !26400
  %152 = fadd <8 x float> %151, %144, !dbg !26405
  %153 = fmul <8 x float> %_31.i.i.i.i138.sroa.0.0.copyload.i.i, %history.i.i189.sroa.0.03731.i.i, !dbg !26410
  %154 = fadd <8 x float> %153, %146, !dbg !26415
  %155 = fmul <8 x float> %_34.i.i.i.i135.sroa.0.0.copyload.i.i, %history.i.i189.sroa.0.03731.i.i, !dbg !26420
  %156 = fadd <8 x float> %155, %148, !dbg !26425
  %157 = fmul <8 x float> %_39.i.i.i.i131.sroa.0.0.copyload.i.i, %history.i.i189.sroa.10.sroa.0.03741.i.i, !dbg !26430
  %158 = fadd <8 x float> %157, %150, !dbg !26435
  %159 = fmul <8 x float> %_42.i.i.i.i128.sroa.0.0.copyload.i.i, %history.i.i189.sroa.10.sroa.0.03741.i.i, !dbg !26440
  %160 = fadd <8 x float> %159, %152, !dbg !26445
  %161 = fmul <8 x float> %_45.i.i.i.i125.sroa.0.0.copyload.i.i, %history.i.i189.sroa.10.sroa.0.03741.i.i, !dbg !26450
  %162 = fadd <8 x float> %161, %154, !dbg !26455
  %163 = fmul <8 x float> %_48.i.i.i.i122.sroa.0.0.copyload.i.i, %history.i.i189.sroa.10.sroa.0.03741.i.i, !dbg !26460
  %164 = fadd <8 x float> %163, %156, !dbg !26465
  %165 = fmul <8 x float> %_53.i.i.i.i118.sroa.0.0.copyload.i.i, %history.i.i189.sroa.13.sroa.0.03740.i.i, !dbg !26470
  %166 = fadd <8 x float> %165, %158, !dbg !26475
  %167 = fmul <8 x float> %_56.i.i.i.i115.sroa.0.0.copyload.i.i, %history.i.i189.sroa.13.sroa.0.03740.i.i, !dbg !26480
  %168 = fadd <8 x float> %167, %160, !dbg !26485
  %169 = fmul <8 x float> %_59.i.i.i.i112.sroa.0.0.copyload.i.i, %history.i.i189.sroa.13.sroa.0.03740.i.i, !dbg !26490
  %170 = fadd <8 x float> %169, %162, !dbg !26495
  %171 = fmul <8 x float> %_62.i.i.i.i109.sroa.0.0.copyload.i.i, %history.i.i189.sroa.13.sroa.0.03740.i.i, !dbg !26500
  %172 = fadd <8 x float> %171, %164, !dbg !26505
  %173 = fmul <8 x float> %_67.i.i.i.i105.sroa.0.0.copyload.i.i, %history.i.i189.sroa.16.sroa.0.03739.i.i, !dbg !26510
  %174 = fadd <8 x float> %173, %166, !dbg !26515
  %175 = fmul <8 x float> %_70.i.i.i.i102.sroa.0.0.copyload.i.i, %history.i.i189.sroa.16.sroa.0.03739.i.i, !dbg !26520
  %176 = fadd <8 x float> %175, %168, !dbg !26525
  %177 = fmul <8 x float> %_73.i.i.i.i99.sroa.0.0.copyload.i.i, %history.i.i189.sroa.16.sroa.0.03739.i.i, !dbg !26530
  %178 = fadd <8 x float> %177, %170, !dbg !26535
  %179 = fmul <8 x float> %_76.i.i.i.i96.sroa.0.0.copyload.i.i, %history.i.i189.sroa.16.sroa.0.03739.i.i, !dbg !26540
  %180 = fadd <8 x float> %179, %172, !dbg !26545
  %181 = fmul <8 x float> %_81.i.i.i.i92.sroa.0.0.copyload.i.i, %history.i.i189.sroa.19.sroa.0.03738.i.i, !dbg !26550
  %182 = fadd <8 x float> %181, %174, !dbg !26555
  %183 = fmul <8 x float> %_84.i.i.i.i89.sroa.0.0.copyload.i.i, %history.i.i189.sroa.19.sroa.0.03738.i.i, !dbg !26560
  %184 = fadd <8 x float> %183, %176, !dbg !26565
  %185 = fmul <8 x float> %_87.i.i.i.i86.sroa.0.0.copyload.i.i, %history.i.i189.sroa.19.sroa.0.03738.i.i, !dbg !26570
  %186 = fadd <8 x float> %185, %178, !dbg !26575
  %187 = fmul <8 x float> %_90.i.i.i.i83.sroa.0.0.copyload.i.i, %history.i.i189.sroa.19.sroa.0.03738.i.i, !dbg !26580
  %188 = fadd <8 x float> %187, %180, !dbg !26585
  %189 = fmul <8 x float> %_95.i.i.i.i79.sroa.0.0.copyload.i.i, %history.i.i189.sroa.22.sroa.0.03737.i.i, !dbg !26590
  %190 = fadd <8 x float> %189, %182, !dbg !26595
  %191 = fmul <8 x float> %_98.i.i.i.i76.sroa.0.0.copyload.i.i, %history.i.i189.sroa.22.sroa.0.03737.i.i, !dbg !26600
  %192 = fadd <8 x float> %191, %184, !dbg !26605
  %193 = fmul <8 x float> %_101.i.i.i.i73.sroa.0.0.copyload.i.i, %history.i.i189.sroa.22.sroa.0.03737.i.i, !dbg !26610
  %194 = fadd <8 x float> %193, %186, !dbg !26615
  %195 = fmul <8 x float> %_104.i.i.i.i70.sroa.0.0.copyload.i.i, %history.i.i189.sroa.22.sroa.0.03737.i.i, !dbg !26620
  %196 = fadd <8 x float> %195, %188, !dbg !26625
  %197 = fmul <8 x float> %_109.i.i.i.i66.sroa.0.0.copyload.i.i, %history.i.i189.sroa.25.sroa.0.03732.i.i, !dbg !26630
  %198 = fadd <8 x float> %197, %190, !dbg !26635
  %199 = fmul <8 x float> %_112.i.i.i.i63.sroa.0.0.copyload.i.i, %history.i.i189.sroa.25.sroa.0.03732.i.i, !dbg !26640
  %200 = fadd <8 x float> %199, %192, !dbg !26645
  %201 = fmul <8 x float> %_115.i.i.i.i60.sroa.0.0.copyload.i.i, %history.i.i189.sroa.25.sroa.0.03732.i.i, !dbg !26650
  %202 = fadd <8 x float> %201, %194, !dbg !26655
  %203 = fmul <8 x float> %_118.i.i.i.i57.sroa.0.0.copyload.i.i, %history.i.i189.sroa.25.sroa.0.03732.i.i, !dbg !26660
  %204 = fadd <8 x float> %203, %196, !dbg !26665
  %205 = fmul <8 x float> %_123.i.i.i.i53.sroa.0.0.copyload.i.i, %history.i.i189.sroa.29.sroa.0.03733.i.i, !dbg !26670
  %206 = fadd <8 x float> %205, %198, !dbg !26675
  %207 = fmul <8 x float> %_126.i.i.i.i50.sroa.0.0.copyload.i.i, %history.i.i189.sroa.29.sroa.0.03733.i.i, !dbg !26680
  %208 = fadd <8 x float> %207, %200, !dbg !26685
  %209 = fmul <8 x float> %_129.i.i.i.i47.sroa.0.0.copyload.i.i, %history.i.i189.sroa.29.sroa.0.03733.i.i, !dbg !26690
  %210 = fadd <8 x float> %209, %202, !dbg !26695
  %211 = fmul <8 x float> %_132.i.i.i.i44.sroa.0.0.copyload.i.i, %history.i.i189.sroa.29.sroa.0.03733.i.i, !dbg !26700
  %212 = fadd <8 x float> %211, %204, !dbg !26705
  %213 = fmul <8 x float> %_137.i.i.i.i40.sroa.0.0.copyload.i.i, %history.i.i189.sroa.32.sroa.0.03734.i.i, !dbg !26710
  %214 = fadd <8 x float> %213, %206, !dbg !26715
  %215 = fmul <8 x float> %_140.i.i.i.i37.sroa.0.0.copyload.i.i, %history.i.i189.sroa.32.sroa.0.03734.i.i, !dbg !26720
  %216 = fadd <8 x float> %215, %208, !dbg !26725
  %217 = fmul <8 x float> %_143.i.i.i.i34.sroa.0.0.copyload.i.i, %history.i.i189.sroa.32.sroa.0.03734.i.i, !dbg !26730
  %218 = fadd <8 x float> %217, %210, !dbg !26735
  %219 = fmul <8 x float> %_146.i.i.i.i31.sroa.0.0.copyload.i.i, %history.i.i189.sroa.32.sroa.0.03734.i.i, !dbg !26740
  %220 = fadd <8 x float> %219, %212, !dbg !26745
  %221 = fmul <8 x float> %_151.i.i.i.i27.sroa.0.0.copyload.i.i, %history.i.i189.sroa.35.sroa.0.03735.i.i, !dbg !26750
  %222 = fadd <8 x float> %221, %214, !dbg !26755
  %223 = fmul <8 x float> %_154.i.i.i.i24.sroa.0.0.copyload.i.i, %history.i.i189.sroa.35.sroa.0.03735.i.i, !dbg !26760
  %224 = fadd <8 x float> %223, %216, !dbg !26765
  %225 = fmul <8 x float> %_157.i.i.i.i21.sroa.0.0.copyload.i.i, %history.i.i189.sroa.35.sroa.0.03735.i.i, !dbg !26770
  %226 = fadd <8 x float> %225, %218, !dbg !26775
  %227 = fmul <8 x float> %_160.i.i.i.i18.sroa.0.0.copyload.i.i, %history.i.i189.sroa.35.sroa.0.03735.i.i, !dbg !26780
  %228 = fadd <8 x float> %227, %220, !dbg !26785
  %229 = fmul <8 x float> %_165.i.i.i.i14.sroa.0.0.copyload.i.i, %history.i.i189.sroa.38.sroa.0.03736.i.i, !dbg !26790
  %230 = fadd <8 x float> %229, %222, !dbg !26795
  %231 = fmul <8 x float> %_168.i.i.i.i11.sroa.0.0.copyload.i.i, %history.i.i189.sroa.38.sroa.0.03736.i.i, !dbg !26800
  %232 = fadd <8 x float> %231, %224, !dbg !26805
  %233 = fmul <8 x float> %_171.i.i.i.i8.sroa.0.0.copyload.i.i, %history.i.i189.sroa.38.sroa.0.03736.i.i, !dbg !26810
  %234 = fadd <8 x float> %233, %226, !dbg !26815
  %235 = fmul <8 x float> %_174.i.i.i.i5.sroa.0.0.copyload.i.i, %history.i.i189.sroa.38.sroa.0.03736.i.i, !dbg !26820
  %236 = fadd <8 x float> %235, %228, !dbg !26825
  %237 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %230), !dbg !26830
  %238 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %140, <8 x float> %237), !dbg !26836
  %239 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %232), !dbg !26830
  %240 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %238, <8 x float> %239), !dbg !26836
  %241 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %234), !dbg !26830
  %242 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %240, <8 x float> %241), !dbg !26836
  %243 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %236), !dbg !26830
  %244 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %242, <8 x float> %243), !dbg !26836
  %_39.i.i283.idx.i.i = shl i64 %iter.sroa.0.0.i.i2353742.i.i, 5, !dbg !26841
  %_39.i.i283.i.i = getelementptr inbounds nuw i8, ptr %peaks_left.i210.i.i, i64 %_39.i.i283.idx.i.i, !dbg !26841
  store <8 x float> %244, ptr %_39.i.i283.i.i, align 4, !dbg !26846, !alias.scope !26851, !noalias !26855
  %exitcond.not.i.i = icmp eq i64 %139, %umax4429.i.i, !dbg !26302
  br i1 %exitcond.not.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i237.i.i, label %bb5.i.i258.i.i, !dbg !26306

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i237.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i, %bb38.i.i.i
  %history.i.i189.sroa.0.0.lcssa.i.i = phi <8 x float> [ %history.i.i189.sroa.0.0.copyload.i.i, %bb38.i.i.i ], [ %lanes.i1293.sroa.0.0.copyload.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i ], !dbg !26859
  %history.i.i189.sroa.25.sroa.0.0.lcssa.i.i = phi <8 x float> [ %history.i.i189.sroa.25.sroa.0.0.copyload.i.i, %bb38.i.i.i ], [ %history.i.i189.sroa.22.sroa.0.03737.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i ], !dbg !26859
  %history.i.i189.sroa.29.sroa.0.0.lcssa.i.i = phi <8 x float> [ %history.i.i189.sroa.29.sroa.0.0.copyload.i.i, %bb38.i.i.i ], [ %history.i.i189.sroa.25.sroa.0.03732.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i ], !dbg !26859
  %history.i.i189.sroa.32.sroa.0.0.lcssa.i.i = phi <8 x float> [ %history.i.i189.sroa.32.sroa.0.0.copyload.i.i, %bb38.i.i.i ], [ %history.i.i189.sroa.29.sroa.0.03733.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i ], !dbg !26859
  %history.i.i189.sroa.35.sroa.0.0.lcssa.i.i = phi <8 x float> [ %history.i.i189.sroa.35.sroa.0.0.copyload.i.i, %bb38.i.i.i ], [ %history.i.i189.sroa.32.sroa.0.03734.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i ], !dbg !26859
  %history.i.i189.sroa.38.sroa.0.0.lcssa.i.i = phi <8 x float> [ %history.i.i189.sroa.38.sroa.0.0.copyload.i.i, %bb38.i.i.i ], [ %history.i.i189.sroa.35.sroa.0.03735.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i ], !dbg !26859
  %history.i.i189.sroa.41.sroa.0.0.lcssa.i.i = phi <8 x float> [ %history.i.i189.sroa.41.sroa.0.0.copyload.i.i, %bb38.i.i.i ], [ %history.i.i189.sroa.38.sroa.0.03736.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i ], !dbg !26859
  %history.i.i189.sroa.22.sroa.0.0.lcssa.i.i = phi <8 x float> [ %history.i.i189.sroa.22.sroa.0.0.copyload.i.i, %bb38.i.i.i ], [ %history.i.i189.sroa.19.sroa.0.03738.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i ], !dbg !26859
  %history.i.i189.sroa.19.sroa.0.0.lcssa.i.i = phi <8 x float> [ %history.i.i189.sroa.19.sroa.0.0.copyload.i.i, %bb38.i.i.i ], [ %history.i.i189.sroa.16.sroa.0.03739.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i ], !dbg !26859
  %history.i.i189.sroa.16.sroa.0.0.lcssa.i.i = phi <8 x float> [ %history.i.i189.sroa.16.sroa.0.0.copyload.i.i, %bb38.i.i.i ], [ %history.i.i189.sroa.13.sroa.0.03740.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i ], !dbg !26859
  %history.i.i189.sroa.13.sroa.0.0.lcssa.i.i = phi <8 x float> [ %history.i.i189.sroa.13.sroa.0.0.copyload.i.i, %bb38.i.i.i ], [ %history.i.i189.sroa.10.sroa.0.03741.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i ], !dbg !26859
  %history.i.i189.sroa.10.sroa.0.0.lcssa.i.i = phi <8 x float> [ %history.i.i189.sroa.10.sroa.0.0.copyload.i.i, %bb38.i.i.i ], [ %history.i.i189.sroa.0.03731.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i.i ], !dbg !26859
  store <8 x float> %history.i.i189.sroa.0.0.lcssa.i.i, ptr %hot_left.i216.i.i, align 32, !dbg !26860, !noalias !26271
  store <8 x float> %history.i.i189.sroa.10.sroa.0.0.lcssa.i.i, ptr %history.i.i189.sroa.10.0.hot_left.i216.sroa_idx.i.i, align 32, !dbg !26860, !noalias !26271
  store <8 x float> %history.i.i189.sroa.13.sroa.0.0.lcssa.i.i, ptr %history.i.i189.sroa.13.0.hot_left.i216.sroa_idx.i.i, align 32, !dbg !26860, !noalias !26271
  store <8 x float> %history.i.i189.sroa.16.sroa.0.0.lcssa.i.i, ptr %history.i.i189.sroa.16.0.hot_left.i216.sroa_idx.i.i, align 32, !dbg !26860, !noalias !26271
  store <8 x float> %history.i.i189.sroa.19.sroa.0.0.lcssa.i.i, ptr %history.i.i189.sroa.19.0.hot_left.i216.sroa_idx.i.i, align 32, !dbg !26860, !noalias !26271
  store <8 x float> %history.i.i189.sroa.22.sroa.0.0.lcssa.i.i, ptr %history.i.i189.sroa.22.0.hot_left.i216.sroa_idx.i.i, align 32, !dbg !26860, !noalias !26271
  store <8 x float> %history.i.i189.sroa.25.sroa.0.0.lcssa.i.i, ptr %history.i.i189.sroa.25.0.hot_left.i216.sroa_idx.i.i, align 32, !dbg !26860, !noalias !26271
  store <8 x float> %history.i.i189.sroa.29.sroa.0.0.lcssa.i.i, ptr %history.i.i189.sroa.29.0.hot_left.i216.sroa_idx.i.i, align 32, !dbg !26860, !noalias !26271
  store <8 x float> %history.i.i189.sroa.32.sroa.0.0.lcssa.i.i, ptr %history.i.i189.sroa.32.0.hot_left.i216.sroa_idx.i.i, align 32, !dbg !26860, !noalias !26271
  store <8 x float> %history.i.i189.sroa.35.sroa.0.0.lcssa.i.i, ptr %history.i.i189.sroa.35.0.hot_left.i216.sroa_idx.i.i, align 32, !dbg !26860, !noalias !26271
  store <8 x float> %history.i.i189.sroa.38.sroa.0.0.lcssa.i.i, ptr %history.i.i189.sroa.38.0.hot_left.i216.sroa_idx.i.i, align 32, !dbg !26860, !noalias !26271
  store <8 x float> %history.i.i189.sroa.41.sroa.0.0.lcssa.i.i, ptr %history.i.i189.sroa.41.0.hot_left.i216.sroa_idx.i.i, align 32, !dbg !26860, !noalias !26271
  br i1 %_20.i.i2363730.not.i.i, label %bb17.i.loopexit.i.i, label %bb41.i.lr.ph.i.i, !dbg !26289

bb41.i.lr.ph.i.i:                                 ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i237.i.i
  %_13.i405.sroa.0.0.copyload.i.i = load <8 x float>, ptr %112, align 32, !noalias !26271
  %_13.i397.sroa.0.0.copyload.i.i = load <8 x float>, ptr %115, align 32, !noalias !26271
  %_63.i.i.i = load i64, ptr %118, align 8, !alias.scope !25944, !noalias !25947
  %_64.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %129, align 32, !noalias !26271
  %_70.i255.i.i = load i64, ptr %135, align 8, !alias.scope !25944, !noalias !25947
  %.promoted.i.i = load <8 x float>, ptr %110, align 32, !noalias !26271
  %_49.i244.promoted.i.i = load <8 x float>, ptr %_49.i244.i.i, align 32, !noalias !26271
  %.promoted3812.i.i = load <8 x float>, ptr %111, align 32, !noalias !26271
  %.promoted3815.i.i = load <8 x float>, ptr %113, align 32, !noalias !26271
  %_51.i245.promoted.i.i = load <8 x float>, ptr %_51.i245.i.i, align 32, !noalias !26271
  %.promoted3818.i.i = load <8 x float>, ptr %114, align 32, !noalias !26271
  %.promoted3821.i.i = load <8 x float>, ptr %128, align 32, !noalias !26271
  br label %bb41.i.i.i, !dbg !26289

bb41.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1532.i.i, %bb41.i.lr.ph.i.i
  %245 = phi <8 x float> [ %.promoted3818.i.i, %bb41.i.lr.ph.i.i ], [ %269, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1532.i.i ]
  %246 = phi <8 x float> [ %_51.i245.promoted.i.i, %bb41.i.lr.ph.i.i ], [ %270, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1532.i.i ]
  %247 = phi <8 x float> [ %.promoted3815.i.i, %bb41.i.lr.ph.i.i ], [ %271, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1532.i.i ]
  %248 = phi <8 x float> [ %.promoted3812.i.i, %bb41.i.lr.ph.i.i ], [ %272, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1532.i.i ]
  %249 = phi <8 x float> [ %_49.i244.promoted.i.i, %bb41.i.lr.ph.i.i ], [ %273, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1532.i.i ]
  %_58.i.i.sroa.0.0.copyload3822.i.i = phi <8 x float> [ %.promoted3821.i.i, %bb41.i.lr.ph.i.i ], [ %309, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1532.i.i ]
  %_12.i398.sroa.0.0.copyload3820.i.i = phi <8 x float> [ %.promoted3818.i.i, %bb41.i.lr.ph.i.i ], [ %_12.i398.sroa.0.0.copyload3819.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1532.i.i ]
  %_11.i.sroa.0.0.copyload3817.i.i = phi <8 x float> [ %_51.i245.promoted.i.i, %bb41.i.lr.ph.i.i ], [ %_11.i.sroa.0.0.copyload3816.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1532.i.i ]
  %250 = phi <8 x float> [ %.promoted3815.i.i, %bb41.i.lr.ph.i.i ], [ %274, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1532.i.i ]
  %_12.i406.sroa.0.0.copyload3814.i.i = phi <8 x float> [ %.promoted3812.i.i, %bb41.i.lr.ph.i.i ], [ %_12.i406.sroa.0.0.copyload3813.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1532.i.i ]
  %_11.i407.sroa.0.0.copyload3811.i.i = phi <8 x float> [ %_49.i244.promoted.i.i, %bb41.i.lr.ph.i.i ], [ %_11.i407.sroa.0.0.copyload3810.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1532.i.i ]
  %251 = phi <8 x float> [ %.promoted.i.i, %bb41.i.lr.ph.i.i ], [ %275, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1532.i.i ]
  %main_cursor.sroa.0.1.i2403768.i.i = phi i64 [ %main_cursor.sroa.0.0.i2313827.i.i, %bb41.i.lr.ph.i.i ], [ %spec.store.select.i.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1532.i.i ]
  %ring_cursor.sroa.0.1.i2393767.i.i = phi i64 [ %ring_cursor.sroa.0.0.i2303826.i.i, %bb41.i.lr.ph.i.i ], [ %spec.store.select9.i.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1532.i.i ]
  %iter2.sroa.0.0.i2383766.i.i = phi i64 [ 0, %bb41.i.lr.ph.i.i ], [ %252, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1532.i.i ]
  %252 = add nuw nsw i64 %iter2.sroa.0.0.i2383766.i.i, 1, !dbg !26861
  %_46.i.i.i = add nuw nsw i64 %iter2.sroa.0.0.i2383766.i.i, %iter.sroa.0.0.i3824.i.i, !dbg !26867
  %base.i242.i.i = shl i64 %_46.i.i.i, 3, !dbg !26867
  br i1 %stationary.sroa.0.0.i219.i.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1341.i.i, label %bb21.i243.i.i, !dbg !26868

bb21.i243.i.i:                                    ; preds = %bb41.i.i.i
  %253 = fadd <8 x float> %251, splat (float -1.000000e+00), !dbg !26869
  %254 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %253, <8 x float> zeroinitializer), !dbg !26874
  %255 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %254, <8 x float> zeroinitializer, i8 30), !dbg !26879
  %256 = fadd <8 x float> %_12.i406.sroa.0.0.copyload3814.i.i, %_11.i407.sroa.0.0.copyload3811.i.i, !dbg !26885
  %257 = bitcast <8 x float> %255 to <8 x i32>, !dbg !26890
  %258 = icmp slt <8 x i32> %257, zeroinitializer, !dbg !26894
  %259 = select <8 x i1> %258, <8 x float> %256, <8 x float> %_13.i405.sroa.0.0.copyload.i.i, !dbg !26894
  %260 = select <8 x i1> %258, <8 x float> %_12.i406.sroa.0.0.copyload3814.i.i, <8 x float> zeroinitializer, !dbg !26896
  %261 = fadd <8 x float> %250, splat (float -1.000000e+00), !dbg !26901
  %262 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %261, <8 x float> zeroinitializer), !dbg !26906
  %263 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %262, <8 x float> zeroinitializer, i8 30), !dbg !26911
  %264 = fadd <8 x float> %_12.i398.sroa.0.0.copyload3820.i.i, %_11.i.sroa.0.0.copyload3817.i.i, !dbg !26917
  %265 = bitcast <8 x float> %263 to <8 x i32>, !dbg !26922
  %266 = icmp slt <8 x i32> %265, zeroinitializer, !dbg !26926
  %267 = select <8 x i1> %266, <8 x float> %264, <8 x float> %_13.i397.sroa.0.0.copyload.i.i, !dbg !26926
  %268 = select <8 x i1> %266, <8 x float> %_12.i398.sroa.0.0.copyload3820.i.i, <8 x float> zeroinitializer, !dbg !26928
  br label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1341.i.i, !dbg !26933

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1341.i.i: ; preds = %bb21.i243.i.i, %bb41.i.i.i
  %269 = phi <8 x float> [ %268, %bb21.i243.i.i ], [ %245, %bb41.i.i.i ]
  %270 = phi <8 x float> [ %267, %bb21.i243.i.i ], [ %246, %bb41.i.i.i ]
  %271 = phi <8 x float> [ %262, %bb21.i243.i.i ], [ %247, %bb41.i.i.i ]
  %272 = phi <8 x float> [ %260, %bb21.i243.i.i ], [ %248, %bb41.i.i.i ]
  %273 = phi <8 x float> [ %259, %bb21.i243.i.i ], [ %249, %bb41.i.i.i ]
  %_12.i398.sroa.0.0.copyload3819.i.i = phi <8 x float> [ %268, %bb21.i243.i.i ], [ %_12.i398.sroa.0.0.copyload3820.i.i, %bb41.i.i.i ]
  %_11.i.sroa.0.0.copyload3816.i.i = phi <8 x float> [ %267, %bb21.i243.i.i ], [ %_11.i.sroa.0.0.copyload3817.i.i, %bb41.i.i.i ]
  %274 = phi <8 x float> [ %262, %bb21.i243.i.i ], [ %250, %bb41.i.i.i ]
  %_12.i406.sroa.0.0.copyload3813.i.i = phi <8 x float> [ %260, %bb21.i243.i.i ], [ %_12.i406.sroa.0.0.copyload3814.i.i, %bb41.i.i.i ]
  %_11.i407.sroa.0.0.copyload3810.i.i = phi <8 x float> [ %259, %bb21.i243.i.i ], [ %_11.i407.sroa.0.0.copyload3811.i.i, %bb41.i.i.i ]
  %275 = phi <8 x float> [ %254, %bb21.i243.i.i ], [ %251, %bb41.i.i.i ]
  %_97.i248.idx.i.i = shl i64 %iter2.sroa.0.0.i2383766.i.i, 5, !dbg !26934
  %_97.i248.i.i = getelementptr inbounds nuw i8, ptr %peaks_left.i210.i.i, i64 %_97.i248.idx.i.i, !dbg !26934
  %lanes.i1334.sroa.0.0.copyload.i.i = load <8 x float>, ptr %_97.i248.i.i, align 4, !dbg !26945, !alias.scope !26950, !noalias !26954
  %276 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i1334.sroa.0.0.copyload.i.i, <8 x float> %lanes.i1334.sroa.0.0.copyload.i.i), !dbg !26958
  %277 = select <8 x i1> %117, <8 x float> %276, <8 x float> %lanes.i1334.sroa.0.0.copyload.i.i, !dbg !26963
  %_98.i.i.i = icmp samesign ugt i64 %base.i242.i.i, %_39.1.i, !dbg !26968
  br i1 %_98.i.i.i, label %bb45.i257.i.i, label %bb46.i.i.i, !dbg !26968, !prof !905

bb46.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1341.i.i
  %_101.i.i.i = sub nuw nsw i64 %_39.1.i, %base.i242.i.i, !dbg !26972
  %_105.i.i.i = getelementptr inbounds nuw float, ptr %_39.0.i, i64 %base.i242.i.i, !dbg !26973
  %_8.i1328.i.i = icmp samesign ugt i64 %_101.i.i.i, 7, !dbg !26978
  br i1 %_8.i1328.i.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1332.i.i, label %bb2.i1329.i.i, !dbg !26978, !prof !1076

bb2.i1329.i.i:                                    ; preds = %bb46.i.i.i
  store <8 x float> %269, ptr %114, align 1, !dbg !26272
  store <8 x float> %_58.i.i.sroa.0.0.copyload3822.i.i, ptr %128, align 1, !dbg !26277
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_101.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #31, !dbg !26983, !noalias !26984
  unreachable, !dbg !26983

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1332.i.i: ; preds = %bb46.i.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26988), !dbg !26991
  %width.i.i.i.i = load i64, ptr %119, align 8, !dbg !26992, !alias.scope !26993, !noalias !26994, !noundef !12
  %278 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %277, <8 x float> %_11.i407.sroa.0.0.copyload3810.i.i, i8 30), !dbg !27003
  %279 = fdiv <8 x float> %_11.i407.sroa.0.0.copyload3810.i.i, %277, !dbg !27009
  %280 = bitcast <8 x float> %278 to <8 x i32>, !dbg !27014
  %281 = icmp slt <8 x i32> %280, zeroinitializer, !dbg !27018
  %282 = select <8 x i1> %281, <8 x float> %279, <8 x float> splat (float 1.000000e+00), !dbg !27018
  %_144.1.i.i.i.i = load i64, ptr %120, align 8, !dbg !27020, !alias.scope !26993, !noalias !26994, !noundef !12
  %_22.i.i249.i.i = mul i64 %width.i.i.i.i, %ring_cursor.sroa.0.1.i2393767.i.i, !dbg !27021
  %_92.i.i.i.i = icmp ugt i64 %_22.i.i249.i.i, %_144.1.i.i.i.i, !dbg !27022
  br i1 %_92.i.i.i.i, label %bb37.i.i.i.i, label %bb38.i.i.i.i, !dbg !27022, !prof !905

bb38.i.i.i.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1332.i.i
  %_95.i.i.i.i = sub nuw i64 %_144.1.i.i.i.i, %_22.i.i249.i.i, !dbg !27025
  %_8.i1544.i.i = icmp samesign ugt i64 %_95.i.i.i.i, 7, !dbg !27026
  br i1 %_8.i1544.i.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1547.i.i, label %bb2.i1545.i.i, !dbg !27026, !prof !1076

bb2.i1545.i.i:                                    ; preds = %bb38.i.i.i.i
  store <8 x float> %269, ptr %114, align 1, !dbg !26272
  store <8 x float> %_58.i.i.sroa.0.0.copyload3822.i.i, ptr %128, align 1, !dbg !26277
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_95.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #31, !dbg !27031, !noalias !27032
  unreachable, !dbg !27031

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1547.i.i: ; preds = %bb38.i.i.i.i
  %_144.0.i.i.i.i = load ptr, ptr %121, align 8, !dbg !27020, !alias.scope !26993, !noalias !26994, !nonnull !12, !noundef !12
  %_99.i.i.i.i = getelementptr inbounds nuw float, ptr %_144.0.i.i.i.i, i64 %_22.i.i249.i.i, !dbg !27036
  store <8 x float> %282, ptr %_99.i.i.i.i, align 4, !dbg !27038, !alias.scope !27042, !noalias !27046
  tail call void @llvm.experimental.noalias.scope.decl(metadata !27048), !dbg !27051
  %width.i.i.i = load i64, ptr %119, align 8, !dbg !27052, !alias.scope !27054, !noalias !27055, !noundef !12
  %283 = icmp eq i64 %width.i.i.i, 0, !dbg !27057
  br i1 %283, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.i.i, label %bb32.i341.lr.ph.i.i, !dbg !27057

bb32.i341.lr.ph.i.i:                              ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1547.i.i
  %_112.1.i.i.i = load i64, ptr %32, align 8, !alias.scope !27054, !noalias !27055, !noundef !12
  %_112.0.i.i.i = load ptr, ptr %31, align 8, !alias.scope !25944, !noalias !25947, !nonnull !12
  %284 = add i64 %ring_cursor.sroa.0.1.i2393767.i.i, 1
  %_23.not.i.i.i = icmp ult i64 %284, %_63.i.i.i
  %285 = select i1 %_23.not.i.i.i, i64 0, i64 %_63.i.i.i
  %start1.sroa.0.0.i.i.i = sub nuw i64 %284, %285
  %_114.1.i.i.i = load i64, ptr %120, align 8, !alias.scope !25944, !noalias !25947
  %_114.0.i.i.i = load ptr, ptr %121, align 8, !alias.scope !25944, !noalias !25947, !nonnull !12
  %_116.1.i.i.i = load i64, ptr %122, align 8, !alias.scope !25944, !noalias !25947
  %_116.0.i.i.i = load ptr, ptr %123, align 8, !alias.scope !25944, !noalias !25947, !nonnull !12
  %_118.1.i.i.i = load i64, ptr %124, align 8, !alias.scope !25944, !noalias !25947
  %_118.0.i.i.i = load ptr, ptr %125, align 8, !alias.scope !25944, !noalias !25947, !nonnull !12
  %_45.i.i.i = mul i64 %width.i.i.i, %start1.sroa.0.0.i.i.i
  br label %bb32.i341.i.i, !dbg !27057

bb32.i341.i.i:                                    ; preds = %bb31.i359.i.i, %bb32.i341.lr.ph.i.i
  %iter.i339.sroa.10.03759.i.i = phi i64 [ %width.i.i.i, %bb32.i341.lr.ph.i.i ], [ %286, %bb31.i359.i.i ]
  %iter.i339.sroa.7.03758.i.i = phi i64 [ 0, %bb32.i341.lr.ph.i.i ], [ %_9.0.i.i.i, %bb31.i359.i.i ]
  %iter.i339.sroa.0.0.idx3757.i.i = phi i64 [ 0, %bb32.i341.lr.ph.i.i ], [ %iter.i339.sroa.0.0.add.i.i, %bb31.i359.i.i ]
  %iter.i339.sroa.0.0.ptr3760.i.i = getelementptr inbounds nuw i8, ptr %scratch.i.i.i, i64 %iter.i339.sroa.0.0.idx3757.i.i, !dbg !27059
  %286 = add i64 %iter.i339.sroa.10.03759.i.i, -1, !dbg !27059
  %_7.i.i.i.i = icmp eq i64 %iter.i339.sroa.0.0.idx3757.i.i, 32, !dbg !27060
  br i1 %_7.i.i.i.i, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.i.i, label %bb3.i343.i.i, !dbg !27064

bb3.i343.i.i:                                     ; preds = %bb32.i341.i.i
  %iter.i339.sroa.0.0.add.i.i = add nuw nsw i64 %iter.i339.sroa.0.0.idx3757.i.i, 4, !dbg !27065
  %_9.0.i.i.i = add nuw nsw i64 %iter.i339.sroa.7.03758.i.i, 1, !dbg !27067
  %exitcond4420.not.i.i = icmp eq i64 %iter.i339.sroa.7.03758.i.i, %_112.1.i.i.i, !dbg !27068
  br i1 %exitcond4420.not.i.i, label %panic.i.i.i, label %bb5.i.i.i, !dbg !27068

bb5.i.i.i:                                        ; preds = %bb3.i343.i.i
  %287 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i.i.i, i64 %iter.i339.sroa.7.03758.i.i, !dbg !27068
  %shape.i.i.i = load i32, ptr %287, align 4, !dbg !27068, !noalias !27069, !noundef !12
  %288 = getelementptr inbounds nuw i8, ptr %287, i64 4, !dbg !27068
  %shape3.i.i.i = load i32, ptr %288, align 4, !dbg !27068, !noalias !27069, !noundef !12
  %window.i.i.i = zext i32 %shape.i.i.i to i64, !dbg !27070
  %_19.i.i.i = zext i32 %shape3.i.i.i to i64, !dbg !27071
  %289 = add i64 %ring_cursor.sroa.0.1.i2393767.i.i, %_19.i.i.i, !dbg !27072
  %_20.not.i.i.i = icmp ult i64 %289, %_63.i.i.i, !dbg !27073
  %290 = select i1 %_20.not.i.i.i, i64 0, i64 %_63.i.i.i, !dbg !27073
  %spec.select.i.i.i = sub nuw i64 %289, %290, !dbg !27073
  %_27.i345.i.i = mul i64 %spec.select.i.i.i, %width.i.i.i, !dbg !27074
  %_26.i.i.i = add i64 %_27.i345.i.i, %iter.i339.sroa.7.03758.i.i, !dbg !27074
  %_30.i.i.i = icmp ult i64 %_26.i.i.i, %_114.1.i.i.i, !dbg !27075
  br i1 %_30.i.i.i, label %bb12.i346.i.i, label %panic5.i.i.i, !dbg !27075

panic.i.i.i:                                      ; preds = %bb3.i343.i.i
  store <8 x float> %269, ptr %114, align 1, !dbg !26272
  store <8 x float> %_58.i.i.sroa.0.0.copyload3822.i.i, ptr %128, align 1, !dbg !26277
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i.i.i, i64 noundef %_112.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_077370d5cece7380867993336836eb69) #31, !dbg !27068, !noalias !27069
  unreachable, !dbg !27068

bb12.i346.i.i:                                    ; preds = %bb5.i.i.i
  %291 = getelementptr inbounds nuw float, ptr %_114.0.i.i.i, i64 %_26.i.i.i, !dbg !27075
  %292 = load float, ptr %291, align 4, !dbg !27075, !noalias !27069, !noundef !12
  %exitcond4421.not.i.i = icmp eq i64 %iter.i339.sroa.7.03758.i.i, %_116.1.i.i.i, !dbg !27076
  br i1 %exitcond4421.not.i.i, label %panic6.i.i.i, label %bb13.i347.i.i, !dbg !27076

panic5.i.i.i:                                     ; preds = %bb5.i.i.i
  store <8 x float> %269, ptr %114, align 1, !dbg !26272
  store <8 x float> %_58.i.i.sroa.0.0.copyload3822.i.i, ptr %128, align 1, !dbg !26277
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i.i.i, i64 noundef %_114.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5cae9ea88ed6362f62dbad10291edbd4) #31, !dbg !27075, !noalias !27069
  unreachable, !dbg !27075

bb13.i347.i.i:                                    ; preds = %bb12.i346.i.i
  %293 = getelementptr inbounds nuw i32, ptr %_116.0.i.i.i, i64 %iter.i339.sroa.7.03758.i.i, !dbg !27076
  %_32.i348.i.i = load i32, ptr %293, align 4, !dbg !27076, !noalias !27069, !noundef !12
  %position.i.i.i = zext i32 %_32.i348.i.i to i64, !dbg !27076
  %294 = icmp eq i32 %_32.i348.i.i, 0, !dbg !27077
  br i1 %294, label %bb17.i352.i.i, label %bb15.i349.i.i, !dbg !27077

panic6.i.i.i:                                     ; preds = %bb12.i346.i.i
  store <8 x float> %269, ptr %114, align 1, !dbg !26272
  store <8 x float> %_58.i.i.sroa.0.0.copyload3822.i.i, ptr %128, align 1, !dbg !26277
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i.i.i, i64 noundef %_116.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9fdd00ed7abe4ccfd73f0d145a7b741b) #31, !dbg !27076, !noalias !27069
  unreachable, !dbg !27076

bb15.i349.i.i:                                    ; preds = %bb13.i347.i.i
  %_37.i350.i.i = icmp ult i64 %iter.i339.sroa.7.03758.i.i, %_118.1.i.i.i, !dbg !27078
  br i1 %_37.i350.i.i, label %bb16.i351.i.i, label %panic7.i.i.i, !dbg !27078

bb17.i352.i.i:                                    ; preds = %bb35.i373.i.i, %bb16.i351.i.i, %bb13.i347.i.i
  %newest.sroa.0.0.i.i.i = phi float [ %292, %bb13.i347.i.i ], [ %_35.i.i.i, %bb35.i373.i.i ], [ %292, %bb16.i351.i.i ], !dbg !27079
  %exitcond4422.not.i.i = icmp eq i64 %iter.i339.sroa.7.03758.i.i, %_118.1.i.i.i, !dbg !27080
  br i1 %exitcond4422.not.i.i, label %panic8.i.i.i, label %bb18.i353.i.i, !dbg !27080

bb16.i351.i.i:                                    ; preds = %bb15.i349.i.i
  %295 = getelementptr inbounds nuw float, ptr %_118.0.i.i.i, i64 %iter.i339.sroa.7.03758.i.i, !dbg !27078
  %_35.i.i.i = load float, ptr %295, align 4, !dbg !27078, !noalias !27069, !noundef !12
  %_102.i.i.i = fcmp olt float %_35.i.i.i, %292, !dbg !27081
  br i1 %_102.i.i.i, label %bb35.i373.i.i, label %bb17.i352.i.i, !dbg !27081

panic7.i.i.i:                                     ; preds = %bb15.i349.i.i
  store <8 x float> %269, ptr %114, align 1, !dbg !26272
  store <8 x float> %_58.i.i.sroa.0.0.copyload3822.i.i, ptr %128, align 1, !dbg !26277
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i339.sroa.7.03758.i.i, i64 noundef %_118.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_697dc8945f5e5b040d7e9a0b92cb249f) #31, !dbg !27078, !noalias !27069
  unreachable, !dbg !27078

bb35.i373.i.i:                                    ; preds = %bb16.i351.i.i
  br label %bb17.i352.i.i, !dbg !27083

bb18.i353.i.i:                                    ; preds = %bb17.i352.i.i
  %296 = getelementptr inbounds nuw float, ptr %_118.0.i.i.i, i64 %iter.i339.sroa.7.03758.i.i, !dbg !27080
  store float %newest.sroa.0.0.i.i.i, ptr %296, align 4, !dbg !27080, !noalias !27069
  %_42.i.i.i = add nuw nsw i64 %position.i.i.i, 1, !dbg !27084
  %complete.i.i.i = icmp eq i64 %_42.i.i.i, %window.i.i.i, !dbg !27084
  br i1 %complete.i.i.i, label %bb22.i361.i.i, label %bb20.i354.i.i, !dbg !27085

panic8.i.i.i:                                     ; preds = %bb17.i352.i.i
  store <8 x float> %269, ptr %114, align 1, !dbg !26272
  store <8 x float> %_58.i.i.sroa.0.0.copyload3822.i.i, ptr %128, align 1, !dbg !26277
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i.i.i, i64 noundef %_118.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_067dce244605df4a33956fbd4d726027) #31, !dbg !27080, !noalias !27069
  unreachable, !dbg !27080

bb20.i354.i.i:                                    ; preds = %bb18.i353.i.i
  %_44.i355.i.i = add i64 %iter.i339.sroa.7.03758.i.i, %_45.i.i.i, !dbg !27086
  %_47.i.i.i = icmp ult i64 %_44.i355.i.i, %_114.1.i.i.i, !dbg !27087
  br i1 %_47.i.i.i, label %bb30.i358.i.i, label %panic9.i.i.i, !dbg !27087

panic9.i.i.i:                                     ; preds = %bb20.i354.i.i
  store <8 x float> %269, ptr %114, align 1, !dbg !26272
  store <8 x float> %_58.i.i.sroa.0.0.copyload3822.i.i, ptr %128, align 1, !dbg !26277
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i355.i.i, i64 noundef %_114.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d5ea38731a1cb6311efef2f919234d06) #31, !dbg !27087, !noalias !27069
  unreachable, !dbg !27087

bb30.i358.i.i:                                    ; preds = %bb20.i354.i.i
  %297 = getelementptr inbounds nuw float, ptr %_114.0.i.i.i, i64 %_44.i355.i.i, !dbg !27087
  %_43.i.i.i = load float, ptr %297, align 4, !dbg !27087, !noalias !27069, !noundef !12
  %_103.i.i.i = fcmp olt float %_43.i.i.i, %newest.sroa.0.0.i.i.i, !dbg !27088
  %newest.sroa.0.1.i.i.i = select i1 %_103.i.i.i, float %_43.i.i.i, float %newest.sroa.0.0.i.i.i, !dbg !27088
  store float %newest.sroa.0.1.i.i.i, ptr %iter.i339.sroa.0.0.ptr3760.i.i, align 4, !dbg !27090, !noalias !27091
  %298 = trunc i64 %_42.i.i.i to i32, !dbg !27092
  br label %bb31.i359.i.i, !dbg !27093

bb31.i359.i.i:                                    ; preds = %bb25.i.i.i, %bb30.i358.i.i
  %storemerge.i.i = phi i32 [ %298, %bb30.i358.i.i ], [ 0, %bb25.i.i.i ], !dbg !27094
  store i32 %storemerge.i.i, ptr %293, align 4, !dbg !27094, !noalias !27069
  %299 = icmp eq i64 %286, 0, !dbg !27057
  br i1 %299, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.i.i, label %bb32.i341.i.i, !dbg !27057

bb22.i361.i.i:                                    ; preds = %bb18.i353.i.i
  store float %newest.sroa.0.0.i.i.i, ptr %iter.i339.sroa.0.0.ptr3760.i.i, align 4, !dbg !27090, !noalias !27091
  %300 = load float, ptr %291, align 4, !dbg !27095, !noalias !27069, !noundef !12
  br label %bb41.i366.i.i, !dbg !27096

bb41.i366.i.i:                                    ; preds = %bb25.i.i.i, %bb22.i361.i.i
  %iter2.sroa.0.0.i3623756.i.i = phi i64 [ 0, %bb22.i361.i.i ], [ %_105.i367.i.i, %bb25.i.i.i ]
  %suffix.sroa.0.0.i3755.i.i = phi float [ %300, %bb22.i361.i.i ], [ %suffix.sroa.0.1.i.i.i, %bb25.i.i.i ]
  %end.sroa.0.1.i3754.i.i = phi i64 [ %spec.select.i.i.i, %bb22.i361.i.i ], [ %303, %bb25.i.i.i ]
  %_56.i.i.i = mul i64 %end.sroa.0.1.i3754.i.i, %width.i.i.i, !dbg !27099
  %_55.i368.i.i = add i64 %_56.i.i.i, %iter.i339.sroa.7.03758.i.i, !dbg !27099
  %_59.i369.i.i = icmp ult i64 %_55.i368.i.i, %_114.1.i.i.i, !dbg !27100
  br i1 %_59.i369.i.i, label %bb25.i.i.i, label %panic13.i.i.i, !dbg !27100

panic13.i.i.i:                                    ; preds = %bb41.i366.i.i
  store <8 x float> %269, ptr %114, align 1, !dbg !26272
  store <8 x float> %_58.i.i.sroa.0.0.copyload3822.i.i, ptr %128, align 1, !dbg !26277
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i368.i.i, i64 noundef %_114.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a19fa7d9077791c9eadbe74fafea8de7) #31, !dbg !27100, !noalias !27069
  unreachable, !dbg !27100

bb25.i.i.i:                                       ; preds = %bb41.i366.i.i
  %_105.i367.i.i = add nuw nsw i64 %iter2.sroa.0.0.i3623756.i.i, 1, !dbg !27101
  %301 = getelementptr inbounds nuw float, ptr %_114.0.i.i.i, i64 %_55.i368.i.i, !dbg !27100
  %_54.i.i.i = load float, ptr %301, align 4, !dbg !27100, !noalias !27069, !noundef !12
  %_107.i.i.i = fcmp olt float %suffix.sroa.0.0.i3755.i.i, %_54.i.i.i, !dbg !27104
  %suffix.sroa.0.1.i.i.i = select i1 %_107.i.i.i, float %suffix.sroa.0.0.i3755.i.i, float %_54.i.i.i, !dbg !27104
  store float %suffix.sroa.0.1.i.i.i, ptr %301, align 4, !dbg !27106, !noalias !27069
  %302 = icmp eq i64 %end.sroa.0.1.i3754.i.i, 0, !dbg !27107
  %spec.store.select.i372.i.i = select i1 %302, i64 %_63.i.i.i, i64 %end.sroa.0.1.i3754.i.i, !dbg !27107
  %303 = add i64 %spec.store.select.i372.i.i, -1, !dbg !27108
  %exitcond4419.not.i.i = icmp eq i64 %_105.i367.i.i, %window.i.i.i, !dbg !27109
  br i1 %exitcond4419.not.i.i, label %bb31.i359.i.i, label %bb41.i366.i.i, !dbg !27096

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.i.i: ; preds = %bb31.i359.i.i, %bb32.i341.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1547.i.i
  %lanes.i1318.sroa.0.0.copyload.i.i = load <8 x float>, ptr %scratch.i.i.i, align 4, !dbg !27111, !alias.scope !27116, !noalias !27120
  %304 = fmul <8 x float> %lanes.i1318.sroa.0.0.copyload.i.i, splat (float 1.638400e+04), !dbg !27124
  %305 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %304), !dbg !27129
  %306 = fmul <8 x float> %305, splat (float 0x3F10000000000000), !dbg !27134
  %307 = icmp eq i64 %width.i.i.i.i, 0, !dbg !27139
  %_149.1.i.i.pre.i.i = load i64, ptr %126, align 8, !dbg !27141, !alias.scope !26993, !noalias !26994
  br i1 %307, label %bb16.i.i.i.i, label %bb39.i.i.lr.ph.i.i, !dbg !27139

bb39.i.i.lr.ph.i.i:                               ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.i.i
  %_145.1.i.i.i.i = load i64, ptr %32, align 8, !alias.scope !26993, !noalias !26994, !noundef !12
  %_145.0.i.i.i.i = load ptr, ptr %31, align 8, !alias.scope !25944, !noalias !25947, !nonnull !12
  %_147.0.i.i.i.i = load ptr, ptr %127, align 8, !alias.scope !25944, !noalias !25947, !nonnull !12
  %exitcond4425.not.i.i = icmp eq i64 %_145.1.i.i.i.i, 0, !dbg !27142
  br i1 %exitcond4425.not.i.i, label %panic.i.i.i.i, label %bb17.i.i.i.i, !dbg !27142

bb37.i.i.i.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1332.i.i
  store <8 x float> %269, ptr %114, align 1, !dbg !26272
  store <8 x float> %_58.i.i.sroa.0.0.copyload3822.i.i, ptr %128, align 1, !dbg !26277
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i249.i.i, i64 noundef %_144.1.i.i.i.i, i64 noundef %_144.1.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9d2713d1692431af37d60082290049ed) #31, !dbg !27143, !noalias !27144
  unreachable, !dbg !27143

bb16.i.i.loopexit.i.i:                            ; preds = %bb21.i.i.7.i.i, %bb21.i.i.6.i.i, %bb21.i.i.5.i.i, %bb21.i.i.4.i.i, %bb21.i.i.3.i.i, %bb21.i.i.2.i.i, %bb21.i.i.1.i.i, %bb21.i.i.i.i
  %lanes.i1311.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %scratch.i.i.i, align 4, !dbg !27145, !alias.scope !27150, !noalias !27154
  br label %bb16.i.i.i.i, !dbg !27158

bb16.i.i.i.i:                                     ; preds = %bb16.i.i.loopexit.i.i, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.i.i
  %lanes.i1311.sroa.0.0.copyload.i.i = phi <8 x float> [ %lanes.i1311.sroa.0.0.copyload.pre.i.i, %bb16.i.i.loopexit.i.i ], [ %lanes.i1318.sroa.0.0.copyload.i.i, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.i.i ], !dbg !27145
  %308 = fadd <8 x float> %_58.i.i.sroa.0.0.copyload3822.i.i, %306, !dbg !27159
  %309 = fsub <8 x float> %308, %lanes.i1311.sroa.0.0.copyload.i.i, !dbg !27164
  %_109.i.i.i.i = icmp ugt i64 %_22.i.i249.i.i, %_149.1.i.i.pre.i.i, !dbg !27169
  br i1 %_109.i.i.i.i, label %bb42.i.i.i.i, label %bb43.i.i.i.i, !dbg !27169, !prof !905

bb43.i.i.i.i:                                     ; preds = %bb16.i.i.i.i
  %_112.i.i.i.i = sub nuw i64 %_149.1.i.i.pre.i.i, %_22.i.i249.i.i, !dbg !27172
  %_8.i1539.i.i = icmp samesign ugt i64 %_112.i.i.i.i, 7, !dbg !27173
  br i1 %_8.i1539.i.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1542.i.i, label %bb2.i1540.i.i, !dbg !27173, !prof !1076

bb2.i1540.i.i:                                    ; preds = %bb43.i.i.i.i
  store <8 x float> %269, ptr %114, align 1, !dbg !26272
  store <8 x float> %309, ptr %128, align 1, !dbg !26277
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_112.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #31, !dbg !27178, !noalias !27179
  unreachable, !dbg !27178

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1542.i.i: ; preds = %bb43.i.i.i.i
  %_149.0.i.i.i.i = load ptr, ptr %127, align 8, !dbg !27141, !alias.scope !26993, !noalias !26994, !nonnull !12, !noundef !12
  %_116.i.i.i.i = getelementptr inbounds nuw float, ptr %_149.0.i.i.i.i, i64 %_22.i.i249.i.i, !dbg !27183
  store <8 x float> %306, ptr %_116.i.i.i.i, align 4, !dbg !27185, !alias.scope !27189, !noalias !27193
  %_68.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %130, align 32, !dbg !27195, !noalias !26271
  %310 = fdiv <8 x float> %309, %_64.i.i.sroa.0.0.copyload.i.i, !dbg !27196
  %311 = fsub <8 x float> splat (float 1.000000e+00), %310, !dbg !27201
  %312 = fsub <8 x float> %311, %_68.i.i.sroa.0.0.copyload.i.i, !dbg !27206
  %313 = fmul <8 x float> %_11.i.sroa.0.0.copyload3816.i.i, %312, !dbg !27211
  %314 = fadd <8 x float> %_68.i.i.sroa.0.0.copyload.i.i, %313, !dbg !27216
  %315 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %311, <8 x float> %314), !dbg !27220
  %316 = bitcast <8 x float> %315 to <8 x i32>, !dbg !27225
  %317 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %315), !dbg !27231
  %318 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %317, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !27233
  %319 = bitcast <8 x float> %318 to <8 x i32>, !dbg !27239
  %320 = xor <8 x i32> %319, splat (i32 -1), !dbg !27245
  %321 = and <8 x i32> %320, %316, !dbg !27247
  %322 = bitcast <8 x i32> %321 to <8 x float>, !dbg !27251
  store <8 x i32> %321, ptr %130, align 32, !dbg !27252, !noalias !26271
  %323 = fsub <8 x float> splat (float 1.000000e+00), %322, !dbg !27253
  %_150.1.i.i.i.i = load i64, ptr %131, align 8, !dbg !27258, !alias.scope !26993, !noalias !26994, !noundef !12
  %_76.i.i.i.i = mul i64 %width.i.i.i.i, %main_cursor.sroa.0.1.i2403768.i.i, !dbg !27259
  %_120.i.i.i.i = icmp ugt i64 %_76.i.i.i.i, %_150.1.i.i.i.i, !dbg !27260
  br i1 %_120.i.i.i.i, label %bb48.i.i.i.i, label %bb49.i.i.i.i, !dbg !27260, !prof !905

bb42.i.i.i.i:                                     ; preds = %bb16.i.i.i.i
  store <8 x float> %269, ptr %114, align 1, !dbg !26272
  store <8 x float> %309, ptr %128, align 1, !dbg !26277
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i249.i.i, i64 noundef %_149.1.i.i.pre.i.i, i64 noundef %_149.1.i.i.pre.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8002ed69501742f3ea2ea25eb68cd581) #31, !dbg !27263, !noalias !27264
  unreachable, !dbg !27263

bb49.i.i.i.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1542.i.i
  %_123.i.i.i.i = sub nuw i64 %_150.1.i.i.i.i, %_76.i.i.i.i, !dbg !27265
  %_8.i1305.i.i = icmp samesign ugt i64 %_123.i.i.i.i, 7, !dbg !27266
  br i1 %_8.i1305.i.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1532.i.i, label %bb2.i1306.i.i, !dbg !27266, !prof !1076

bb2.i1306.i.i:                                    ; preds = %bb49.i.i.i.i
  store <8 x float> %269, ptr %114, align 1, !dbg !26272
  store <8 x float> %309, ptr %128, align 1, !dbg !26277
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_123.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #31, !dbg !27271, !noalias !27272
  unreachable, !dbg !27271

bb48.i.i.i.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1542.i.i
  store <8 x float> %269, ptr %114, align 1, !dbg !26272
  store <8 x float> %309, ptr %128, align 1, !dbg !26277
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i.i.i.i, i64 noundef %_150.1.i.i.i.i, i64 noundef %_150.1.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a07d19b424a92543209d516ece5bdf2a) #31, !dbg !27276, !noalias !27264
  unreachable, !dbg !27276

bb17.i.i.i.i:                                     ; preds = %bb39.i.i.lr.ph.i.i
  %324 = getelementptr inbounds nuw i8, ptr %_145.0.i.i.i.i, i64 8, !dbg !27142
  %_44.i.i251.i.i = load i32, ptr %324, align 4, !dbg !27142, !noalias !27264, !noundef !12
  %_43.i.i252.i.i = zext i32 %_44.i.i251.i.i to i64, !dbg !27142
  %325 = add i64 %ring_cursor.sroa.0.1.i2393767.i.i, %_43.i.i252.i.i, !dbg !27277
  %_47.not.i.i.i.i = icmp ult i64 %325, %_63.i.i.i, !dbg !27278
  %326 = select i1 %_47.not.i.i.i.i, i64 0, i64 %_63.i.i.i, !dbg !27278
  %spec.select.i.i.i.i = sub nuw i64 %325, %326, !dbg !27278
  %_51.i.i.i.i = mul i64 %spec.select.i.i.i.i, %width.i.i.i.i, !dbg !27279
  %_53.i.i253.i.i = icmp ult i64 %_51.i.i.i.i, %_149.1.i.i.pre.i.i, !dbg !27280
  br i1 %_53.i.i253.i.i, label %bb21.i.i.i.i, label %panic1.i.i.i.i, !dbg !27280

panic.i.i.i.i:                                    ; preds = %bb39.i.i.7.i.i, %bb39.i.i.6.i.i, %bb39.i.i.5.i.i, %bb39.i.i.4.i.i, %bb39.i.i.3.i.i, %bb39.i.i.2.i.i, %bb39.i.i.1.i.i, %bb39.i.i.lr.ph.i.i
  %_145.1.i.i.lcssa.ph.i.i = phi i64 [ 7, %bb39.i.i.7.i.i ], [ 6, %bb39.i.i.6.i.i ], [ 5, %bb39.i.i.5.i.i ], [ 4, %bb39.i.i.4.i.i ], [ 3, %bb39.i.i.3.i.i ], [ 2, %bb39.i.i.2.i.i ], [ 1, %bb39.i.i.1.i.i ], [ 0, %bb39.i.i.lr.ph.i.i ]
  store <8 x float> %269, ptr %114, align 1, !dbg !26272
  store <8 x float> %_58.i.i.sroa.0.0.copyload3822.i.i, ptr %128, align 1, !dbg !26277
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i.i.lcssa.ph.i.i, i64 noundef %_145.1.i.i.lcssa.ph.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_891dd683d7367e0a1ec5a22f9c2a2aa8) #31, !dbg !27142, !noalias !27264
  unreachable, !dbg !27142

bb21.i.i.i.i:                                     ; preds = %bb17.i.i.i.i
  %327 = getelementptr inbounds nuw float, ptr %_147.0.i.i.i.i, i64 %_51.i.i.i.i, !dbg !27280
  %_49.i.i254.i.i = load float, ptr %327, align 4, !dbg !27280, !noalias !27264, !noundef !12
  store float %_49.i.i254.i.i, ptr %scratch.i.i.i, align 4, !dbg !27281, !noalias !27282
  %328 = icmp eq i64 %width.i.i.i.i, 1, !dbg !27139
  br i1 %328, label %bb16.i.i.loopexit.i.i, label %bb39.i.i.1.i.i, !dbg !27139

bb39.i.i.1.i.i:                                   ; preds = %bb21.i.i.i.i
  %exitcond4425.1.not.i.i = icmp eq i64 %_145.1.i.i.i.i, 1, !dbg !27142
  br i1 %exitcond4425.1.not.i.i, label %panic.i.i.i.i, label %bb17.i.i.1.i.i, !dbg !27142

bb17.i.i.1.i.i:                                   ; preds = %bb39.i.i.1.i.i
  %329 = getelementptr inbounds nuw i8, ptr %_145.0.i.i.i.i, i64 20, !dbg !27142
  %_44.i.i251.1.i.i = load i32, ptr %329, align 4, !dbg !27142, !noalias !27264, !noundef !12
  %_43.i.i252.1.i.i = zext i32 %_44.i.i251.1.i.i to i64, !dbg !27142
  %330 = add i64 %ring_cursor.sroa.0.1.i2393767.i.i, %_43.i.i252.1.i.i, !dbg !27277
  %_47.not.i.i.1.i.i = icmp ult i64 %330, %_63.i.i.i, !dbg !27278
  %331 = select i1 %_47.not.i.i.1.i.i, i64 0, i64 %_63.i.i.i, !dbg !27278
  %spec.select.i.i.1.i.i = sub nuw i64 %330, %331, !dbg !27278
  %_51.i.i.1.i.i = mul i64 %spec.select.i.i.1.i.i, %width.i.i.i.i, !dbg !27279
  %_50.i.i.1.i.i = add i64 %_51.i.i.1.i.i, 1, !dbg !27279
  %_53.i.i253.1.i.i = icmp ult i64 %_50.i.i.1.i.i, %_149.1.i.i.pre.i.i, !dbg !27280
  br i1 %_53.i.i253.1.i.i, label %bb21.i.i.1.i.i, label %panic1.i.i.i.i, !dbg !27280

bb21.i.i.1.i.i:                                   ; preds = %bb17.i.i.1.i.i
  %332 = getelementptr inbounds nuw float, ptr %_147.0.i.i.i.i, i64 %_50.i.i.1.i.i, !dbg !27280
  %_49.i.i254.1.i.i = load float, ptr %332, align 4, !dbg !27280, !noalias !27264, !noundef !12
  store float %_49.i.i254.1.i.i, ptr %iter.i.i.sroa.0.0.ptr3764.1.i.i, align 4, !dbg !27281, !noalias !27282
  %333 = icmp eq i64 %width.i.i.i.i, 2, !dbg !27139
  br i1 %333, label %bb16.i.i.loopexit.i.i, label %bb39.i.i.2.i.i, !dbg !27139

bb39.i.i.2.i.i:                                   ; preds = %bb21.i.i.1.i.i
  %exitcond4425.2.not.i.i = icmp eq i64 %_145.1.i.i.i.i, 2, !dbg !27142
  br i1 %exitcond4425.2.not.i.i, label %panic.i.i.i.i, label %bb17.i.i.2.i.i, !dbg !27142

bb17.i.i.2.i.i:                                   ; preds = %bb39.i.i.2.i.i
  %334 = getelementptr inbounds nuw i8, ptr %_145.0.i.i.i.i, i64 32, !dbg !27142
  %_44.i.i251.2.i.i = load i32, ptr %334, align 4, !dbg !27142, !noalias !27264, !noundef !12
  %_43.i.i252.2.i.i = zext i32 %_44.i.i251.2.i.i to i64, !dbg !27142
  %335 = add i64 %ring_cursor.sroa.0.1.i2393767.i.i, %_43.i.i252.2.i.i, !dbg !27277
  %_47.not.i.i.2.i.i = icmp ult i64 %335, %_63.i.i.i, !dbg !27278
  %336 = select i1 %_47.not.i.i.2.i.i, i64 0, i64 %_63.i.i.i, !dbg !27278
  %spec.select.i.i.2.i.i = sub nuw i64 %335, %336, !dbg !27278
  %_51.i.i.2.i.i = mul i64 %spec.select.i.i.2.i.i, %width.i.i.i.i, !dbg !27279
  %_50.i.i.2.i.i = add i64 %_51.i.i.2.i.i, 2, !dbg !27279
  %_53.i.i253.2.i.i = icmp ult i64 %_50.i.i.2.i.i, %_149.1.i.i.pre.i.i, !dbg !27280
  br i1 %_53.i.i253.2.i.i, label %bb21.i.i.2.i.i, label %panic1.i.i.i.i, !dbg !27280

bb21.i.i.2.i.i:                                   ; preds = %bb17.i.i.2.i.i
  %337 = getelementptr inbounds nuw float, ptr %_147.0.i.i.i.i, i64 %_50.i.i.2.i.i, !dbg !27280
  %_49.i.i254.2.i.i = load float, ptr %337, align 4, !dbg !27280, !noalias !27264, !noundef !12
  store float %_49.i.i254.2.i.i, ptr %iter.i.i.sroa.0.0.ptr3764.2.i.i, align 4, !dbg !27281, !noalias !27282
  %338 = icmp eq i64 %width.i.i.i.i, 3, !dbg !27139
  br i1 %338, label %bb16.i.i.loopexit.i.i, label %bb39.i.i.3.i.i, !dbg !27139

bb39.i.i.3.i.i:                                   ; preds = %bb21.i.i.2.i.i
  %exitcond4425.3.not.i.i = icmp eq i64 %_145.1.i.i.i.i, 3, !dbg !27142
  br i1 %exitcond4425.3.not.i.i, label %panic.i.i.i.i, label %bb17.i.i.3.i.i, !dbg !27142

bb17.i.i.3.i.i:                                   ; preds = %bb39.i.i.3.i.i
  %339 = getelementptr inbounds nuw i8, ptr %_145.0.i.i.i.i, i64 44, !dbg !27142
  %_44.i.i251.3.i.i = load i32, ptr %339, align 4, !dbg !27142, !noalias !27264, !noundef !12
  %_43.i.i252.3.i.i = zext i32 %_44.i.i251.3.i.i to i64, !dbg !27142
  %340 = add i64 %ring_cursor.sroa.0.1.i2393767.i.i, %_43.i.i252.3.i.i, !dbg !27277
  %_47.not.i.i.3.i.i = icmp ult i64 %340, %_63.i.i.i, !dbg !27278
  %341 = select i1 %_47.not.i.i.3.i.i, i64 0, i64 %_63.i.i.i, !dbg !27278
  %spec.select.i.i.3.i.i = sub nuw i64 %340, %341, !dbg !27278
  %_51.i.i.3.i.i = mul i64 %spec.select.i.i.3.i.i, %width.i.i.i.i, !dbg !27279
  %_50.i.i.3.i.i = add i64 %_51.i.i.3.i.i, 3, !dbg !27279
  %_53.i.i253.3.i.i = icmp ult i64 %_50.i.i.3.i.i, %_149.1.i.i.pre.i.i, !dbg !27280
  br i1 %_53.i.i253.3.i.i, label %bb21.i.i.3.i.i, label %panic1.i.i.i.i, !dbg !27280

bb21.i.i.3.i.i:                                   ; preds = %bb17.i.i.3.i.i
  %342 = getelementptr inbounds nuw float, ptr %_147.0.i.i.i.i, i64 %_50.i.i.3.i.i, !dbg !27280
  %_49.i.i254.3.i.i = load float, ptr %342, align 4, !dbg !27280, !noalias !27264, !noundef !12
  store float %_49.i.i254.3.i.i, ptr %iter.i.i.sroa.0.0.ptr3764.3.i.i, align 4, !dbg !27281, !noalias !27282
  %343 = icmp eq i64 %width.i.i.i.i, 4, !dbg !27139
  br i1 %343, label %bb16.i.i.loopexit.i.i, label %bb39.i.i.4.i.i, !dbg !27139

bb39.i.i.4.i.i:                                   ; preds = %bb21.i.i.3.i.i
  %exitcond4425.4.not.i.i = icmp eq i64 %_145.1.i.i.i.i, 4, !dbg !27142
  br i1 %exitcond4425.4.not.i.i, label %panic.i.i.i.i, label %bb17.i.i.4.i.i, !dbg !27142

bb17.i.i.4.i.i:                                   ; preds = %bb39.i.i.4.i.i
  %344 = getelementptr inbounds nuw i8, ptr %_145.0.i.i.i.i, i64 56, !dbg !27142
  %_44.i.i251.4.i.i = load i32, ptr %344, align 4, !dbg !27142, !noalias !27264, !noundef !12
  %_43.i.i252.4.i.i = zext i32 %_44.i.i251.4.i.i to i64, !dbg !27142
  %345 = add i64 %ring_cursor.sroa.0.1.i2393767.i.i, %_43.i.i252.4.i.i, !dbg !27277
  %_47.not.i.i.4.i.i = icmp ult i64 %345, %_63.i.i.i, !dbg !27278
  %346 = select i1 %_47.not.i.i.4.i.i, i64 0, i64 %_63.i.i.i, !dbg !27278
  %spec.select.i.i.4.i.i = sub nuw i64 %345, %346, !dbg !27278
  %_51.i.i.4.i.i = mul i64 %spec.select.i.i.4.i.i, %width.i.i.i.i, !dbg !27279
  %_50.i.i.4.i.i = add i64 %_51.i.i.4.i.i, 4, !dbg !27279
  %_53.i.i253.4.i.i = icmp ult i64 %_50.i.i.4.i.i, %_149.1.i.i.pre.i.i, !dbg !27280
  br i1 %_53.i.i253.4.i.i, label %bb21.i.i.4.i.i, label %panic1.i.i.i.i, !dbg !27280

bb21.i.i.4.i.i:                                   ; preds = %bb17.i.i.4.i.i
  %347 = getelementptr inbounds nuw float, ptr %_147.0.i.i.i.i, i64 %_50.i.i.4.i.i, !dbg !27280
  %_49.i.i254.4.i.i = load float, ptr %347, align 4, !dbg !27280, !noalias !27264, !noundef !12
  store float %_49.i.i254.4.i.i, ptr %iter.i.i.sroa.0.0.ptr3764.4.i.i, align 4, !dbg !27281, !noalias !27282
  %348 = icmp eq i64 %width.i.i.i.i, 5, !dbg !27139
  br i1 %348, label %bb16.i.i.loopexit.i.i, label %bb39.i.i.5.i.i, !dbg !27139

bb39.i.i.5.i.i:                                   ; preds = %bb21.i.i.4.i.i
  %exitcond4425.5.not.i.i = icmp eq i64 %_145.1.i.i.i.i, 5, !dbg !27142
  br i1 %exitcond4425.5.not.i.i, label %panic.i.i.i.i, label %bb17.i.i.5.i.i, !dbg !27142

bb17.i.i.5.i.i:                                   ; preds = %bb39.i.i.5.i.i
  %349 = getelementptr inbounds nuw i8, ptr %_145.0.i.i.i.i, i64 68, !dbg !27142
  %_44.i.i251.5.i.i = load i32, ptr %349, align 4, !dbg !27142, !noalias !27264, !noundef !12
  %_43.i.i252.5.i.i = zext i32 %_44.i.i251.5.i.i to i64, !dbg !27142
  %350 = add i64 %ring_cursor.sroa.0.1.i2393767.i.i, %_43.i.i252.5.i.i, !dbg !27277
  %_47.not.i.i.5.i.i = icmp ult i64 %350, %_63.i.i.i, !dbg !27278
  %351 = select i1 %_47.not.i.i.5.i.i, i64 0, i64 %_63.i.i.i, !dbg !27278
  %spec.select.i.i.5.i.i = sub nuw i64 %350, %351, !dbg !27278
  %_51.i.i.5.i.i = mul i64 %spec.select.i.i.5.i.i, %width.i.i.i.i, !dbg !27279
  %_50.i.i.5.i.i = add i64 %_51.i.i.5.i.i, 5, !dbg !27279
  %_53.i.i253.5.i.i = icmp ult i64 %_50.i.i.5.i.i, %_149.1.i.i.pre.i.i, !dbg !27280
  br i1 %_53.i.i253.5.i.i, label %bb21.i.i.5.i.i, label %panic1.i.i.i.i, !dbg !27280

bb21.i.i.5.i.i:                                   ; preds = %bb17.i.i.5.i.i
  %352 = getelementptr inbounds nuw float, ptr %_147.0.i.i.i.i, i64 %_50.i.i.5.i.i, !dbg !27280
  %_49.i.i254.5.i.i = load float, ptr %352, align 4, !dbg !27280, !noalias !27264, !noundef !12
  store float %_49.i.i254.5.i.i, ptr %iter.i.i.sroa.0.0.ptr3764.5.i.i, align 4, !dbg !27281, !noalias !27282
  %353 = icmp eq i64 %width.i.i.i.i, 6, !dbg !27139
  br i1 %353, label %bb16.i.i.loopexit.i.i, label %bb39.i.i.6.i.i, !dbg !27139

bb39.i.i.6.i.i:                                   ; preds = %bb21.i.i.5.i.i
  %exitcond4425.6.not.i.i = icmp eq i64 %_145.1.i.i.i.i, 6, !dbg !27142
  br i1 %exitcond4425.6.not.i.i, label %panic.i.i.i.i, label %bb17.i.i.6.i.i, !dbg !27142

bb17.i.i.6.i.i:                                   ; preds = %bb39.i.i.6.i.i
  %354 = getelementptr inbounds nuw i8, ptr %_145.0.i.i.i.i, i64 80, !dbg !27142
  %_44.i.i251.6.i.i = load i32, ptr %354, align 4, !dbg !27142, !noalias !27264, !noundef !12
  %_43.i.i252.6.i.i = zext i32 %_44.i.i251.6.i.i to i64, !dbg !27142
  %355 = add i64 %ring_cursor.sroa.0.1.i2393767.i.i, %_43.i.i252.6.i.i, !dbg !27277
  %_47.not.i.i.6.i.i = icmp ult i64 %355, %_63.i.i.i, !dbg !27278
  %356 = select i1 %_47.not.i.i.6.i.i, i64 0, i64 %_63.i.i.i, !dbg !27278
  %spec.select.i.i.6.i.i = sub nuw i64 %355, %356, !dbg !27278
  %_51.i.i.6.i.i = mul i64 %spec.select.i.i.6.i.i, %width.i.i.i.i, !dbg !27279
  %_50.i.i.6.i.i = add i64 %_51.i.i.6.i.i, 6, !dbg !27279
  %_53.i.i253.6.i.i = icmp ult i64 %_50.i.i.6.i.i, %_149.1.i.i.pre.i.i, !dbg !27280
  br i1 %_53.i.i253.6.i.i, label %bb21.i.i.6.i.i, label %panic1.i.i.i.i, !dbg !27280

bb21.i.i.6.i.i:                                   ; preds = %bb17.i.i.6.i.i
  %357 = getelementptr inbounds nuw float, ptr %_147.0.i.i.i.i, i64 %_50.i.i.6.i.i, !dbg !27280
  %_49.i.i254.6.i.i = load float, ptr %357, align 4, !dbg !27280, !noalias !27264, !noundef !12
  store float %_49.i.i254.6.i.i, ptr %iter.i.i.sroa.0.0.ptr3764.6.i.i, align 4, !dbg !27281, !noalias !27282
  %358 = icmp eq i64 %width.i.i.i.i, 7, !dbg !27139
  br i1 %358, label %bb16.i.i.loopexit.i.i, label %bb39.i.i.7.i.i, !dbg !27139

bb39.i.i.7.i.i:                                   ; preds = %bb21.i.i.6.i.i
  %exitcond4425.7.not.i.i = icmp eq i64 %_145.1.i.i.i.i, 7, !dbg !27142
  br i1 %exitcond4425.7.not.i.i, label %panic.i.i.i.i, label %bb17.i.i.7.i.i, !dbg !27142

bb17.i.i.7.i.i:                                   ; preds = %bb39.i.i.7.i.i
  %359 = getelementptr inbounds nuw i8, ptr %_145.0.i.i.i.i, i64 92, !dbg !27142
  %_44.i.i251.7.i.i = load i32, ptr %359, align 4, !dbg !27142, !noalias !27264, !noundef !12
  %_43.i.i252.7.i.i = zext i32 %_44.i.i251.7.i.i to i64, !dbg !27142
  %360 = add i64 %ring_cursor.sroa.0.1.i2393767.i.i, %_43.i.i252.7.i.i, !dbg !27277
  %_47.not.i.i.7.i.i = icmp ult i64 %360, %_63.i.i.i, !dbg !27278
  %361 = select i1 %_47.not.i.i.7.i.i, i64 0, i64 %_63.i.i.i, !dbg !27278
  %spec.select.i.i.7.i.i = sub nuw i64 %360, %361, !dbg !27278
  %_51.i.i.7.i.i = mul i64 %spec.select.i.i.7.i.i, %width.i.i.i.i, !dbg !27279
  %_50.i.i.7.i.i = add i64 %_51.i.i.7.i.i, 7, !dbg !27279
  %_53.i.i253.7.i.i = icmp ult i64 %_50.i.i.7.i.i, %_149.1.i.i.pre.i.i, !dbg !27280
  br i1 %_53.i.i253.7.i.i, label %bb21.i.i.7.i.i, label %panic1.i.i.i.i, !dbg !27280

bb21.i.i.7.i.i:                                   ; preds = %bb17.i.i.7.i.i
  %362 = getelementptr inbounds nuw float, ptr %_147.0.i.i.i.i, i64 %_50.i.i.7.i.i, !dbg !27280
  %_49.i.i254.7.i.i = load float, ptr %362, align 4, !dbg !27280, !noalias !27264, !noundef !12
  store float %_49.i.i254.7.i.i, ptr %iter.i.i.sroa.0.0.ptr3764.7.i.i, align 4, !dbg !27281, !noalias !27282
  br label %bb16.i.i.loopexit.i.i, !dbg !27139

panic1.i.i.i.i:                                   ; preds = %bb17.i.i.7.i.i, %bb17.i.i.6.i.i, %bb17.i.i.5.i.i, %bb17.i.i.4.i.i, %bb17.i.i.3.i.i, %bb17.i.i.2.i.i, %bb17.i.i.1.i.i, %bb17.i.i.i.i
  %_50.i.i.lcssa.ph.i.i = phi i64 [ %_50.i.i.7.i.i, %bb17.i.i.7.i.i ], [ %_50.i.i.6.i.i, %bb17.i.i.6.i.i ], [ %_50.i.i.5.i.i, %bb17.i.i.5.i.i ], [ %_50.i.i.4.i.i, %bb17.i.i.4.i.i ], [ %_50.i.i.3.i.i, %bb17.i.i.3.i.i ], [ %_50.i.i.2.i.i, %bb17.i.i.2.i.i ], [ %_50.i.i.1.i.i, %bb17.i.i.1.i.i ], [ %_51.i.i.i.i, %bb17.i.i.i.i ]
  store <8 x float> %269, ptr %114, align 1, !dbg !26272
  store <8 x float> %_58.i.i.sroa.0.0.copyload3822.i.i, ptr %128, align 1, !dbg !26277
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i.i.lcssa.ph.i.i, i64 noundef %_149.1.i.i.pre.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aca25255cb99dc5ba482e692667f5878) #31, !dbg !27280, !noalias !27264
  unreachable, !dbg !27280

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1532.i.i: ; preds = %bb49.i.i.i.i
  %_150.0.i.i.i.i = load ptr, ptr %132, align 8, !dbg !27258, !alias.scope !26993, !noalias !26994, !nonnull !12, !noundef !12
  %_127.i.i.i.i = getelementptr inbounds nuw float, ptr %_150.0.i.i.i.i, i64 %_76.i.i.i.i, !dbg !27283
  %lanes.i1302.sroa.0.0.copyload.i.i = load <8 x float>, ptr %_127.i.i.i.i, align 4, !dbg !27285, !alias.scope !27289, !noalias !27293
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_127.i.i.i.i, ptr noundef nonnull align 4 dereferenceable(32) %_105.i.i.i, i64 32, i1 false), !dbg !27295, !noalias !25924
  %363 = fmul <8 x float> %323, %lanes.i1302.sroa.0.0.copyload.i.i, !dbg !27300
  %364 = select <8 x i1> %134, <8 x float> %lanes.i1302.sroa.0.0.copyload.i.i, <8 x float> %363, !dbg !27305
  store <8 x float> %364, ptr %_105.i.i.i, align 4, !dbg !27310, !alias.scope !27315, !noalias !27319
  %365 = add i64 %main_cursor.sroa.0.1.i2403768.i.i, 1, !dbg !27323
  %_68.i256.i.i = icmp eq i64 %365, %_70.i255.i.i, !dbg !27324
  %spec.store.select.i.i.i = select i1 %_68.i256.i.i, i64 0, i64 %365, !dbg !27324
  %366 = add i64 %ring_cursor.sroa.0.1.i2393767.i.i, 1, !dbg !27325
  %_71.i.i.i = icmp eq i64 %366, %_63.i.i.i, !dbg !27326
  %spec.store.select9.i.i.i = select i1 %_71.i.i.i, i64 0, i64 %366, !dbg !27326
  %exitcond4430.not.i.i = icmp eq i64 %252, %umax4429.i.i, !dbg !27327
  br i1 %exitcond4430.not.i.i, label %bb19.i.bb17.i.loopexit_crit_edge.i.i, label %bb41.i.i.i, !dbg !26289

bb45.i257.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1341.i.i
  store <8 x float> %269, ptr %114, align 1, !dbg !26272
  store <8 x float> %_58.i.i.sroa.0.0.copyload3822.i.i, ptr %128, align 1, !dbg !26277
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i242.i.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_903a4f70ab87a036c36d8512c06ab29a) #31, !dbg !27330, !noalias !27331
  unreachable, !dbg !27330

_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i.i: ; preds = %bb17.i.loopexit.i.i
  %367 = trunc i64 %main_cursor.sroa.0.1.i240.lcssa.i.i to i32, !dbg !27332
  %368 = trunc i64 %ring_cursor.sroa.0.1.i239.lcssa.i.i to i32, !dbg !27333
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i, !dbg !27334

_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i.i, %bb6.i218.i.i
  %ring_cursor.sroa.0.0.i230.lcssa.i.i = phi i32 [ %_29.i226.i.i, %bb6.i218.i.i ], [ %368, %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i.i ], !dbg !26230
  %main_cursor.sroa.0.0.i231.lcssa.i.i = phi i32 [ %_27.i.i.i, %bb6.i218.i.i ], [ %367, %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i.i ], !dbg !26226
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_left.i216.i.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25.i) #30, !dbg !27335, !noalias !25947
  store i32 %main_cursor.sroa.0.0.i231.lcssa.i.i, ptr %_24.i.i, align 4, !dbg !27332, !alias.scope !26228, !noalias !26229
  store i32 %ring_cursor.sroa.0.0.i230.lcssa.i.i, ptr %67, align 4, !dbg !27333, !alias.scope !26228, !noalias !26229
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i210.i.i), !dbg !27336, !noalias !26234
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i.i.i), !dbg !27337, !noalias !26234
  br label %bb40.i.i, !dbg !26153

bb38.i.i:                                         ; preds = %bb1.i3.i.i.i, %bb2.i1692.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !27338), !dbg !27341
  tail call void @llvm.experimental.noalias.scope.decl(metadata !27342), !dbg !27341
  tail call void @llvm.experimental.noalias.scope.decl(metadata !27344), !dbg !27341
  tail call void @llvm.experimental.noalias.scope.decl(metadata !27346), !dbg !27341
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_left.i.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_25.i) #30, !dbg !27348, !noalias !25947
  %369 = getelementptr inbounds nuw i8, ptr %self, i64 1776, !dbg !27352
  %_152.0.i.i.i = load ptr, ptr %369, align 8, !dbg !27352, !alias.scope !27354, !noalias !27355, !nonnull !12, !noundef !12
  %370 = getelementptr inbounds nuw i8, ptr %self, i64 1784, !dbg !27352
  %_152.1.i.i.i = load i64, ptr %370, align 8, !dbg !27352, !alias.scope !27354, !noalias !27355, !noundef !12
  %_8.i1776.i.i = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_152.0.i.i.i, i64 %_152.1.i.i.i, !dbg !27357
  br label %bb1.i.i1777.i.i, !dbg !27362

bb1.i.i1777.i.i:                                  ; preds = %bb13.i.i1780.i.i, %bb38.i.i
  %_221.i.i1778.i.i = phi ptr [ %_22.i.i1781.i.i, %bb13.i.i1780.i.i ], [ %_152.0.i.i.i, %bb38.i.i ]
  %_12.i.i1779.i.i = icmp eq ptr %_221.i.i1778.i.i, %_8.i1776.i.i, !dbg !27364
  br i1 %_12.i.i1779.i.i, label %bb3.i.i.i, label %bb13.i.i1780.i.i, !dbg !27367

bb13.i.i1780.i.i:                                 ; preds = %bb1.i.i1777.i.i
  %_22.i.i1781.i.i = getelementptr inbounds nuw i8, ptr %_221.i.i1778.i.i, i64 16, !dbg !27368
  %371 = getelementptr inbounds nuw i8, ptr %_221.i.i1778.i.i, i64 12, !dbg !27370
  %_3.i.i.i1782.i.i = load i32, ptr %371, align 4, !dbg !27370, !alias.scope !27372, !noalias !27377, !noundef !12
  %372 = icmp eq i32 %_3.i.i.i1782.i.i, 0, !dbg !27370
  %_51.i.i.i1783.i.i = load i32, ptr %_221.i.i1778.i.i, align 4, !dbg !27370, !alias.scope !27372, !noalias !27377
  %373 = getelementptr inbounds nuw i8, ptr %_221.i.i1778.i.i, i64 4, !dbg !27370
  %_72.i.i.i1784.i.i = load i32, ptr %373, align 4, !dbg !27370, !alias.scope !27372, !noalias !27377
  %374 = icmp eq i32 %_51.i.i.i1783.i.i, %_72.i.i.i1784.i.i, !dbg !27370
  %_0.sroa.0.0.i.i.i1785.i.i = select i1 %372, i1 %374, i1 false, !dbg !27370
  br i1 %_0.sroa.0.0.i.i.i1785.i.i, label %bb1.i.i1777.i.i, label %bb6.i.i.i, !dbg !27380

bb3.i.i.i:                                        ; preds = %bb1.i.i1777.i.i
  %375 = getelementptr inbounds nuw i8, ptr %self, i64 1792, !dbg !27381
  %_153.0.i.i.i = load ptr, ptr %375, align 8, !dbg !27381, !alias.scope !27354, !noalias !27355, !nonnull !12, !noundef !12
  %376 = getelementptr inbounds nuw i8, ptr %self, i64 1800, !dbg !27381
  %_153.1.i.i.i = load i64, ptr %376, align 8, !dbg !27381, !alias.scope !27354, !noalias !27355, !noundef !12
  %_8.i1787.i.i = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_153.0.i.i.i, i64 %_153.1.i.i.i, !dbg !27382
  br label %bb1.i.i1788.i.i, !dbg !27387

bb1.i.i1788.i.i:                                  ; preds = %bb13.i.i1791.i.i, %bb3.i.i.i
  %_221.i.i1789.i.i = phi ptr [ %_22.i.i1792.i.i, %bb13.i.i1791.i.i ], [ %_153.0.i.i.i, %bb3.i.i.i ]
  %_12.i.i1790.i.i = icmp eq ptr %_221.i.i1789.i.i, %_8.i1787.i.i, !dbg !27389
  br i1 %_12.i.i1790.i.i, label %bb6.i.i.i, label %bb13.i.i1791.i.i, !dbg !27392

bb13.i.i1791.i.i:                                 ; preds = %bb1.i.i1788.i.i
  %_22.i.i1792.i.i = getelementptr inbounds nuw i8, ptr %_221.i.i1789.i.i, i64 16, !dbg !27393
  %377 = getelementptr inbounds nuw i8, ptr %_221.i.i1789.i.i, i64 12, !dbg !27395
  %_3.i.i.i1793.i.i = load i32, ptr %377, align 4, !dbg !27395, !alias.scope !27397, !noalias !27402, !noundef !12
  %378 = icmp eq i32 %_3.i.i.i1793.i.i, 0, !dbg !27395
  %_51.i.i.i1794.i.i = load i32, ptr %_221.i.i1789.i.i, align 4, !dbg !27395, !alias.scope !27397, !noalias !27402
  %379 = getelementptr inbounds nuw i8, ptr %_221.i.i1789.i.i, i64 4, !dbg !27395
  %_72.i.i.i1795.i.i = load i32, ptr %379, align 4, !dbg !27395, !alias.scope !27397, !noalias !27402
  %380 = icmp eq i32 %_51.i.i.i1794.i.i, %_72.i.i.i1795.i.i, !dbg !27395
  %_0.sroa.0.0.i.i.i1796.i.i = select i1 %378, i1 %380, i1 false, !dbg !27395
  br i1 %_0.sroa.0.0.i.i.i1796.i.i, label %bb1.i.i1788.i.i, label %bb6.i.i.i, !dbg !27405

bb6.i.i.i:                                        ; preds = %bb13.i.i1780.i.i, %bb13.i.i1791.i.i, %bb1.i.i1788.i.i
  %stationary.sroa.0.0.i.i.i = phi i1 [ false, %bb13.i.i1791.i.i ], [ true, %bb1.i.i1788.i.i ], [ false, %bb13.i.i1780.i.i ], !dbg !27406
  %381 = getelementptr inbounds nuw i8, ptr %self, i64 1536, !dbg !27407
  %382 = load i8, ptr %381, align 32, !dbg !27407, !range !5399, !alias.scope !27411, !noalias !27412, !noundef !12
  %383 = getelementptr inbounds nuw i8, ptr %self, i64 1537, !dbg !27413
  %384 = load i8, ptr %383, align 1, !dbg !27413, !range !5399, !alias.scope !27411, !noalias !27412, !noundef !12
  %385 = getelementptr inbounds nuw i8, ptr %self, i64 1624, !dbg !27415
  %ring.i.i.i = load i64, ptr %385, align 8, !dbg !27415, !alias.scope !27417, !noalias !27418, !noundef !12
  %386 = getelementptr inbounds nuw i8, ptr %self, i64 1632, !dbg !27419
  %main.i.i.i = load i64, ptr %386, align 8, !dbg !27419, !alias.scope !27417, !noalias !27418, !noundef !12
  %_28.i.i.i = load i32, ptr %_24.i.i, align 4, !dbg !27421, !alias.scope !27423, !noalias !27424, !noundef !12
  %387 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !27425
  %_29.i.i.i = load i32, ptr %387, align 4, !dbg !27425, !alias.scope !27423, !noalias !27424, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i.i.i), !dbg !27427, !noalias !27429
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i.i.i, i8 0, i64 1024, i1 false), !noalias !27429
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i.i.i), !dbg !27430, !noalias !27429
; call <true_peak_limiter::UniformHot<wide::f32x8_::f32x8>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_(ptr noalias noundef align 32 captures(none) dereferenceable(128) %uniform_left.i.i.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25.i, i64 %ring.i.i.i, i64 %main.i.i.i) #30, !dbg !27432, !noalias !25947
  %388 = add nuw nsw i64 %_31.i, 31, !dbg !27433
  %yield_count.sroa.0.0.i.i1801.i.i = lshr i64 %388, 5, !dbg !27433
  %_112.not.i3981.i.i = icmp eq i64 %yield_count.sroa.0.0.i.i1801.i.i, 0, !dbg !27440
  br i1 %_112.not.i3981.i.i, label %bb6.i.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i_crit_edge.i, label %bb45.i.lr.ph.i.i, !dbg !27440

bb6.i.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i_crit_edge.i: ; preds = %bb6.i.i.i
  %.phi.trans.insert.i = getelementptr inbounds nuw i8, ptr %uniform_left.i.i.i, i64 104
  %left_phase.i.i.pre.i = load i32, ptr %.phi.trans.insert.i, align 8, !dbg !27449, !noalias !27429
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i, !dbg !27440

bb45.i.lr.ph.i.i:                                 ; preds = %bb6.i.i.i
  %389 = zext i32 %_29.i.i.i to i64, !dbg !27425
  %390 = zext i32 %_28.i.i.i to i64, !dbg !27421
  %_25.i.i.i = trunc nuw i8 %384 to i1, !dbg !27413
  %_24.i.i.i = trunc nuw i8 %382 to i1, !dbg !27407
  %391 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !27450
  %392 = bitcast <8 x float> %391 to <8 x i32>, !dbg !27456
  %393 = xor <8 x i32> %392, splat (i32 -1), !dbg !27462
  %history.i.i.sroa.10.0.hot_left.i.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i.i, i64 32
  %history.i.i.sroa.13.0.hot_left.i.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i.i, i64 64
  %history.i.i.sroa.16.0.hot_left.i.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i.i, i64 96
  %history.i.i.sroa.19.0.hot_left.i.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i.i, i64 128
  %history.i.i.sroa.22.0.hot_left.i.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i.i, i64 160
  %history.i.i.sroa.25.0.hot_left.i.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i.i, i64 192
  %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i.i, i64 224
  %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i.i, i64 256
  %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i.i, i64 288
  %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i.i, i64 320
  %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i.i, i64 352
  %394 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %395 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %396 = getelementptr inbounds nuw i8, ptr %self, i64 96
  %row12.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 128
  %397 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %398 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %399 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %row13.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 256
  %400 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %401 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %402 = getelementptr inbounds nuw i8, ptr %self, i64 352
  %row14.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 384
  %403 = getelementptr inbounds nuw i8, ptr %self, i64 416
  %404 = getelementptr inbounds nuw i8, ptr %self, i64 448
  %405 = getelementptr inbounds nuw i8, ptr %self, i64 480
  %row15.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 512
  %406 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %407 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %408 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %row16.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 640
  %409 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %410 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %411 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %row17.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 768
  %412 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %413 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %414 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %row18.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 896
  %415 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %416 = getelementptr inbounds nuw i8, ptr %self, i64 960
  %417 = getelementptr inbounds nuw i8, ptr %self, i64 992
  %row19.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %418 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %419 = getelementptr inbounds nuw i8, ptr %self, i64 1088
  %420 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %row20.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %421 = getelementptr inbounds nuw i8, ptr %self, i64 1184
  %422 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %423 = getelementptr inbounds nuw i8, ptr %self, i64 1248
  %row21.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1280
  %424 = getelementptr inbounds nuw i8, ptr %self, i64 1312
  %425 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %426 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %row22.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1408
  %427 = getelementptr inbounds nuw i8, ptr %self, i64 1440
  %428 = getelementptr inbounds nuw i8, ptr %self, i64 1472
  %429 = getelementptr inbounds nuw i8, ptr %self, i64 1504
  %430 = getelementptr inbounds nuw i8, ptr %uniform_left.i.i.i, i64 80
  %_51.i.sroa.3.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %uniform_left.i.i.i, i64 88
  %_51.i.sroa.4.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %uniform_left.i.i.i, i64 96
  %_80.i.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i.i, i64 384
  %431 = getelementptr inbounds nuw i8, ptr %hot_left.i.i.i, i64 480
  %432 = getelementptr inbounds nuw i8, ptr %hot_left.i.i.i, i64 448
  %433 = getelementptr inbounds nuw i8, ptr %hot_left.i.i.i, i64 416
  %_82.i.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i.i, i64 512
  %434 = getelementptr inbounds nuw i8, ptr %hot_left.i.i.i, i64 608
  %435 = getelementptr inbounds nuw i8, ptr %hot_left.i.i.i, i64 576
  %436 = getelementptr inbounds nuw i8, ptr %hot_left.i.i.i, i64 544
  %437 = select i1 %_24.i.i.i, <8 x i32> %392, <8 x i32> %393
  %438 = icmp slt <8 x i32> %437, zeroinitializer
  %439 = getelementptr inbounds nuw i8, ptr %uniform_left.i.i.i, i64 32
  %440 = getelementptr inbounds nuw i8, ptr %uniform_left.i.i.i, i64 40
  %_22.i.i.i.i = getelementptr inbounds nuw i8, ptr %uniform_left.i.i.i, i64 104
  %441 = getelementptr inbounds nuw i8, ptr %uniform_left.i.i.i, i64 48
  %442 = getelementptr inbounds nuw i8, ptr %uniform_left.i.i.i, i64 56
  %443 = getelementptr inbounds nuw i8, ptr %hot_left.i.i.i, i64 672
  %444 = getelementptr inbounds nuw i8, ptr %hot_left.i.i.i, i64 704
  %445 = getelementptr inbounds nuw i8, ptr %hot_left.i.i.i, i64 640
  %446 = getelementptr inbounds nuw i8, ptr %uniform_left.i.i.i, i64 72
  %447 = getelementptr inbounds nuw i8, ptr %uniform_left.i.i.i, i64 64
  %448 = select i1 %_25.i.i.i, <8 x i32> %392, <8 x i32> %393
  %449 = icmp slt <8 x i32> %448, zeroinitializer
  %history.i.i.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %hot_left.i.i.i, align 32, !dbg !27464, !noalias !26271
  %history.i.i.sroa.10.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %history.i.i.sroa.10.0.hot_left.i.sroa_idx.i.i, align 32, !dbg !27464, !noalias !26271
  %history.i.i.sroa.13.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %history.i.i.sroa.13.0.hot_left.i.sroa_idx.i.i, align 32, !dbg !27464, !noalias !26271
  %history.i.i.sroa.16.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %history.i.i.sroa.16.0.hot_left.i.sroa_idx.i.i, align 32, !dbg !27464, !noalias !26271
  %history.i.i.sroa.19.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %history.i.i.sroa.19.0.hot_left.i.sroa_idx.i.i, align 32, !dbg !27464, !noalias !26271
  %history.i.i.sroa.22.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %history.i.i.sroa.22.0.hot_left.i.sroa_idx.i.i, align 32, !dbg !27464, !noalias !26271
  %history.i.i.sroa.25.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %history.i.i.sroa.25.0.hot_left.i.sroa_idx.i.i, align 32, !dbg !27464, !noalias !26271
  %history.i.i.sroa.29.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i.i, align 32, !dbg !27464, !noalias !26271
  %history.i.i.sroa.32.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i.i, align 32, !dbg !27464, !noalias !26271
  %history.i.i.sroa.35.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i.i, align 32, !dbg !27464, !noalias !26271
  %history.i.i.sroa.38.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i.i, align 32, !dbg !27464, !noalias !26271
  %history.i.i.sroa.41.sroa.0.0.copyload.pre.i.i = load <8 x float>, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i.i, align 32, !dbg !27464, !noalias !26271
  %_51.i.sroa.3.0.copyload.pre.i.i = load i64, ptr %_51.i.sroa.3.0..sroa_idx.i.i, align 8, !noalias !26271
  %_51.i.sroa.4.0.copyload.pre.i.i = load i64, ptr %_51.i.sroa.4.0..sroa_idx.i.i, align 16, !noalias !26271
  %_54.0.i.i.i.i = load ptr, ptr %439, align 32, !noalias !26271, !nonnull !12, !align !9542
  %_54.1.i.i.i.i = load i64, ptr %440, align 8, !noalias !26271
  %_18.i21.i.i.i = load i64, ptr %430, align 16, !noalias !26271
  %_56.0.i.i.i.i = load ptr, ptr %441, align 16, !noalias !26271, !nonnull !12, !align !9542
  %_56.1.i.i.i.i = load i64, ptr %442, align 8, !noalias !26271
  %_58.1.i.i.i.i = load i64, ptr %446, align 8, !noalias !26271
  %_58.0.i.i.i.i = load ptr, ptr %447, align 32, !noalias !26271, !nonnull !12, !align !9542
  %uniform_left.i.promoted.i.i = load <8 x float>, ptr %uniform_left.i.i.i, align 1, !noalias !26271
  %_22.i.i.promoted.i.i = load i32, ptr %_22.i.i.i.i, align 4, !noalias !26271
  %_13.i433.sroa.0.0.copyload.i.i = load <8 x float>, ptr %433, align 32
  %_13.i419.sroa.0.0.copyload.i.i = load <8 x float>, ptr %436, align 32
  %_37.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %444, align 32
  %hot_left.i.i.i.promoted = load <8 x float>, ptr %hot_left.i.i.i, align 1
  %history.i.i.sroa.10.0.hot_left.i.sroa_idx.i.i.promoted = load <8 x float>, ptr %history.i.i.sroa.10.0.hot_left.i.sroa_idx.i.i, align 1
  %.promoted1272 = load <8 x float>, ptr %431, align 32
  %_80.i.i.i.promoted1275 = load <8 x float>, ptr %_80.i.i.i, align 32
  %.promoted1278 = load <8 x float>, ptr %432, align 32
  %.promoted1281 = load <8 x float>, ptr %434, align 32
  %_82.i.i.i.promoted = load <8 x float>, ptr %_82.i.i.i, align 32
  %.promoted1286 = load <8 x float>, ptr %435, align 32
  %.promoted1289 = load <8 x float>, ptr %443, align 32
  br label %bb45.i.i.i, !dbg !27440

bb21.i.bb18.i.loopexit_crit_edge.i.i:             ; preds = %bb30.i.i.i
  store <8 x float> %_11.i435.sroa.0.0.copyload3889.i.i.lcssa8831233, ptr %_80.i.i.i, align 1, !dbg !27468
  store <8 x float> %_12.i434.sroa.0.0.copyload3905.i.i.lcssa8901243, ptr %432, align 1, !dbg !27478
  store <8 x float> %.lcssa8871186.i, ptr %434, align 1, !dbg !27480, !noalias !25922
  store <8 x float> %.lcssa8951195.i, ptr %_82.i.i.i, align 1, !dbg !27482, !noalias !25922
  store <8 x float> %.lcssa9031205.i, ptr %435, align 1, !dbg !27483, !noalias !25922
  store <8 x float> %.lcssa9231215.i, ptr %443, align 1, !dbg !27484, !noalias !25922
  store <8 x float> %.lcssa46814999.i.i, ptr %431, align 32, !noalias !26271
  br label %bb18.i.loopexit.i.i, !dbg !27491

bb18.i.loopexit.i.i:                              ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i, %bb21.i.bb18.i.loopexit_crit_edge.i.i
  %.lcssa9231215.i.lcssa1290 = phi <8 x float> [ %.lcssa9231215.i, %bb21.i.bb18.i.loopexit_crit_edge.i.i ], [ %.lcssa9231215.i.lcssa1291, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %.lcssa9031205.i.lcssa1287 = phi <8 x float> [ %.lcssa9031205.i, %bb21.i.bb18.i.loopexit_crit_edge.i.i ], [ %.lcssa9031205.i.lcssa1288, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %.lcssa8951195.i.lcssa1284 = phi <8 x float> [ %.lcssa8951195.i, %bb21.i.bb18.i.loopexit_crit_edge.i.i ], [ %.lcssa8951195.i.lcssa1285, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %.lcssa8871186.i.lcssa1282 = phi <8 x float> [ %.lcssa8871186.i, %bb21.i.bb18.i.loopexit_crit_edge.i.i ], [ %.lcssa8871186.i.lcssa1283, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %_12.i434.sroa.0.0.copyload3905.i.i.lcssa8901243.lcssa1279 = phi <8 x float> [ %_12.i434.sroa.0.0.copyload3905.i.i.lcssa8901243, %bb21.i.bb18.i.loopexit_crit_edge.i.i ], [ %_12.i434.sroa.0.0.copyload3905.i.i.lcssa8901243.lcssa1280, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %_11.i435.sroa.0.0.copyload3889.i.i.lcssa8831233.lcssa1276 = phi <8 x float> [ %_11.i435.sroa.0.0.copyload3889.i.i.lcssa8831233, %bb21.i.bb18.i.loopexit_crit_edge.i.i ], [ %_11.i435.sroa.0.0.copyload3889.i.i.lcssa8831233.lcssa1277, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %.lcssa46814999.i.i.lcssa1273 = phi <8 x float> [ %.lcssa46814999.i.i, %bb21.i.bb18.i.loopexit_crit_edge.i.i ], [ %.lcssa46814999.i.i.lcssa1274, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %storemerge.i.i.lcssa39373962.lcssa5038.i.i = phi i32 [ %storemerge.i.i.lcssa39373962.i.i, %bb21.i.bb18.i.loopexit_crit_edge.i.i ], [ %storemerge.i.i.lcssa39373962.lcssa5039.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %minimum.i.i.sroa.0.03861.lcssa3943.lcssa.i.i = phi <8 x float> [ %minimum.i.i.sroa.0.03861.lcssa.i.i, %bb21.i.bb18.i.loopexit_crit_edge.i.i ], [ %minimum.i.i.sroa.0.03861.lcssa3943.lcssa5027.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %ring_cursor.sroa.0.1.i.lcssa.i.i = phi i64 [ %ring_cursor.sroa.0.2.i.i.i, %bb21.i.bb18.i.loopexit_crit_edge.i.i ], [ %ring_cursor.sroa.0.0.i3982.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ], !dbg !27492
  %main_cursor.sroa.0.1.i.lcssa.i.i = phi i64 [ %main_cursor.sroa.0.2.i.i.i, %bb21.i.bb18.i.loopexit_crit_edge.i.i ], [ %main_cursor.sroa.0.0.i3983.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ], !dbg !27493
  %_112.not.i.i.i = icmp eq i64 %451, 0, !dbg !27440
  %indvars.iv.next4434.i.i = add nsw i64 %indvars.iv4433.i.i, -32, !dbg !27440
  br i1 %_112.not.i.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i.i, label %bb45.i.i.i, !dbg !27440

bb45.i.i.i:                                       ; preds = %bb18.i.loopexit.i.i, %bb45.i.lr.ph.i.i
  %.lcssa9231215.i.lcssa1291 = phi <8 x float> [ %.promoted1289, %bb45.i.lr.ph.i.i ], [ %.lcssa9231215.i.lcssa1290, %bb18.i.loopexit.i.i ]
  %.lcssa9031205.i.lcssa1288 = phi <8 x float> [ %.promoted1286, %bb45.i.lr.ph.i.i ], [ %.lcssa9031205.i.lcssa1287, %bb18.i.loopexit.i.i ]
  %.lcssa8951195.i.lcssa1285 = phi <8 x float> [ %_82.i.i.i.promoted, %bb45.i.lr.ph.i.i ], [ %.lcssa8951195.i.lcssa1284, %bb18.i.loopexit.i.i ]
  %.lcssa8871186.i.lcssa1283 = phi <8 x float> [ %.promoted1281, %bb45.i.lr.ph.i.i ], [ %.lcssa8871186.i.lcssa1282, %bb18.i.loopexit.i.i ]
  %_12.i434.sroa.0.0.copyload3905.i.i.lcssa8901243.lcssa1280 = phi <8 x float> [ %.promoted1278, %bb45.i.lr.ph.i.i ], [ %_12.i434.sroa.0.0.copyload3905.i.i.lcssa8901243.lcssa1279, %bb18.i.loopexit.i.i ]
  %_11.i435.sroa.0.0.copyload3889.i.i.lcssa8831233.lcssa1277 = phi <8 x float> [ %_80.i.i.i.promoted1275, %bb45.i.lr.ph.i.i ], [ %_11.i435.sroa.0.0.copyload3889.i.i.lcssa8831233.lcssa1276, %bb18.i.loopexit.i.i ]
  %.lcssa46814999.i.i.lcssa1274 = phi <8 x float> [ %.promoted1272, %bb45.i.lr.ph.i.i ], [ %.lcssa46814999.i.i.lcssa1273, %bb18.i.loopexit.i.i ]
  %history.i.i.sroa.10.sroa.0.0.lcssa.i.i1262 = phi <8 x float> [ %history.i.i.sroa.10.0.hot_left.i.sroa_idx.i.i.promoted, %bb45.i.lr.ph.i.i ], [ %history.i.i.sroa.10.sroa.0.0.lcssa.i.i, %bb18.i.loopexit.i.i ]
  %history.i.i.sroa.0.0.lcssa.i.i1252 = phi <8 x float> [ %hot_left.i.i.i.promoted, %bb45.i.lr.ph.i.i ], [ %history.i.i.sroa.0.0.lcssa.i.i, %bb18.i.loopexit.i.i ]
  %storemerge.i.i.lcssa39373962.lcssa5039.i.i = phi i32 [ %_22.i.i.promoted.i.i, %bb45.i.lr.ph.i.i ], [ %storemerge.i.i.lcssa39373962.lcssa5038.i.i, %bb18.i.loopexit.i.i ]
  %minimum.i.i.sroa.0.03861.lcssa3943.lcssa5027.i.i = phi <8 x float> [ %uniform_left.i.promoted.i.i, %bb45.i.lr.ph.i.i ], [ %minimum.i.i.sroa.0.03861.lcssa3943.lcssa.i.i, %bb18.i.loopexit.i.i ]
  %history.i.i.sroa.41.sroa.0.0.copyload.i.i = phi <8 x float> [ %history.i.i.sroa.41.sroa.0.0.copyload.pre.i.i, %bb45.i.lr.ph.i.i ], [ %history.i.i.sroa.41.sroa.0.0.lcssa.i.i, %bb18.i.loopexit.i.i ], !dbg !27464
  %history.i.i.sroa.38.sroa.0.0.copyload.i.i = phi <8 x float> [ %history.i.i.sroa.38.sroa.0.0.copyload.pre.i.i, %bb45.i.lr.ph.i.i ], [ %history.i.i.sroa.38.sroa.0.0.lcssa.i.i, %bb18.i.loopexit.i.i ], !dbg !27464
  %history.i.i.sroa.35.sroa.0.0.copyload.i.i = phi <8 x float> [ %history.i.i.sroa.35.sroa.0.0.copyload.pre.i.i, %bb45.i.lr.ph.i.i ], [ %history.i.i.sroa.35.sroa.0.0.lcssa.i.i, %bb18.i.loopexit.i.i ], !dbg !27464
  %history.i.i.sroa.32.sroa.0.0.copyload.i.i = phi <8 x float> [ %history.i.i.sroa.32.sroa.0.0.copyload.pre.i.i, %bb45.i.lr.ph.i.i ], [ %history.i.i.sroa.32.sroa.0.0.lcssa.i.i, %bb18.i.loopexit.i.i ], !dbg !27464
  %history.i.i.sroa.29.sroa.0.0.copyload.i.i = phi <8 x float> [ %history.i.i.sroa.29.sroa.0.0.copyload.pre.i.i, %bb45.i.lr.ph.i.i ], [ %history.i.i.sroa.29.sroa.0.0.lcssa.i.i, %bb18.i.loopexit.i.i ], !dbg !27464
  %history.i.i.sroa.25.sroa.0.0.copyload.i.i = phi <8 x float> [ %history.i.i.sroa.25.sroa.0.0.copyload.pre.i.i, %bb45.i.lr.ph.i.i ], [ %history.i.i.sroa.25.sroa.0.0.lcssa.i.i, %bb18.i.loopexit.i.i ], !dbg !27464
  %history.i.i.sroa.22.sroa.0.0.copyload.i.i = phi <8 x float> [ %history.i.i.sroa.22.sroa.0.0.copyload.pre.i.i, %bb45.i.lr.ph.i.i ], [ %history.i.i.sroa.22.sroa.0.0.lcssa.i.i, %bb18.i.loopexit.i.i ], !dbg !27464
  %history.i.i.sroa.19.sroa.0.0.copyload.i.i = phi <8 x float> [ %history.i.i.sroa.19.sroa.0.0.copyload.pre.i.i, %bb45.i.lr.ph.i.i ], [ %history.i.i.sroa.19.sroa.0.0.lcssa.i.i, %bb18.i.loopexit.i.i ], !dbg !27464
  %history.i.i.sroa.16.sroa.0.0.copyload.i.i = phi <8 x float> [ %history.i.i.sroa.16.sroa.0.0.copyload.pre.i.i, %bb45.i.lr.ph.i.i ], [ %history.i.i.sroa.16.sroa.0.0.lcssa.i.i, %bb18.i.loopexit.i.i ], !dbg !27464
  %history.i.i.sroa.13.sroa.0.0.copyload.i.i = phi <8 x float> [ %history.i.i.sroa.13.sroa.0.0.copyload.pre.i.i, %bb45.i.lr.ph.i.i ], [ %history.i.i.sroa.13.sroa.0.0.lcssa.i.i, %bb18.i.loopexit.i.i ], !dbg !27464
  %history.i.i.sroa.10.sroa.0.0.copyload.i.i = phi <8 x float> [ %history.i.i.sroa.10.sroa.0.0.copyload.pre.i.i, %bb45.i.lr.ph.i.i ], [ %history.i.i.sroa.10.sroa.0.0.lcssa.i.i, %bb18.i.loopexit.i.i ], !dbg !27464
  %history.i.i.sroa.0.0.copyload.i.i = phi <8 x float> [ %history.i.i.sroa.0.0.copyload.pre.i.i, %bb45.i.lr.ph.i.i ], [ %history.i.i.sroa.0.0.lcssa.i.i, %bb18.i.loopexit.i.i ], !dbg !27464
  %indvars.iv4433.i.i = phi i64 [ %_31.i, %bb45.i.lr.ph.i.i ], [ %indvars.iv.next4434.i.i, %bb18.i.loopexit.i.i ]
  %iter3.sroa.0.0.i3985.i.i = phi i64 [ %yield_count.sroa.0.0.i.i1801.i.i, %bb45.i.lr.ph.i.i ], [ %451, %bb18.i.loopexit.i.i ]
  %iter2.sroa.0.0.i3984.i.i = phi i64 [ 0, %bb45.i.lr.ph.i.i ], [ %450, %bb18.i.loopexit.i.i ]
  %main_cursor.sroa.0.0.i3983.i.i = phi i64 [ %390, %bb45.i.lr.ph.i.i ], [ %main_cursor.sroa.0.1.i.lcssa.i.i, %bb18.i.loopexit.i.i ]
  %ring_cursor.sroa.0.0.i3982.i.i = phi i64 [ %389, %bb45.i.lr.ph.i.i ], [ %ring_cursor.sroa.0.1.i.lcssa.i.i, %bb18.i.loopexit.i.i ]
  %umin4450.i.i = tail call i64 @llvm.umin.i64(i64 %indvars.iv4433.i.i, i64 32), !dbg !27494
  %umax4436.i.i = tail call i64 @llvm.umax.i64(i64 %umin4450.i.i, i64 1), !dbg !27494
  %450 = add nuw nsw i64 %iter2.sroa.0.0.i3984.i.i, 32, !dbg !27494
  %451 = add nsw i64 %iter3.sroa.0.0.i3985.i.i, -1, !dbg !27498
  %_37.i.i.i = sub nsw i64 %_31.i, %iter2.sroa.0.0.i3984.i.i, !dbg !27499
  %..i1802.i.i = tail call noundef i64 @llvm.umin.i64(i64 %_37.i.i.i, i64 32), !dbg !27500
  %_20.i.i3832.not.i.i = icmp eq i64 %iter2.sroa.0.0.i3984.i.i, %_31.i, !dbg !27504
  br i1 %_20.i.i3832.not.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i, label %bb5.i.i.lr.ph.i.i, !dbg !27508

bb5.i.i.lr.ph.i.i:                                ; preds = %bb45.i.i.i
  %_5.i1196.i.i = load <8 x float>, ptr %self, align 32, !alias.scope !25944, !noalias !25947
  %_14.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %394, align 32, !alias.scope !25944, !noalias !25947
  %_17.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %395, align 32, !alias.scope !25944, !noalias !25947
  %_20.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %396, align 32, !alias.scope !25944, !noalias !25947
  %_25.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %row12.i.i.i.i.i.i, align 32, !alias.scope !25944, !noalias !25947
  %_28.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %397, align 32, !alias.scope !25944, !noalias !25947
  %_31.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %398, align 32, !alias.scope !25944, !noalias !25947
  %_34.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %399, align 32, !alias.scope !25944, !noalias !25947
  %_39.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %row13.i.i.i.i.i.i, align 32, !alias.scope !25944, !noalias !25947
  %_42.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %400, align 32, !alias.scope !25944, !noalias !25947
  %_45.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %401, align 32, !alias.scope !25944, !noalias !25947
  %_48.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %402, align 32, !alias.scope !25944, !noalias !25947
  %_53.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %row14.i.i.i.i.i.i, align 32, !alias.scope !25944, !noalias !25947
  %_56.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %403, align 32, !alias.scope !25944, !noalias !25947
  %_59.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %404, align 32, !alias.scope !25944, !noalias !25947
  %_62.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %405, align 32, !alias.scope !25944, !noalias !25947
  %_67.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %row15.i.i.i.i.i.i, align 32, !alias.scope !25944, !noalias !25947
  %_70.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %406, align 32, !alias.scope !25944, !noalias !25947
  %_73.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %407, align 32, !alias.scope !25944, !noalias !25947
  %_76.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %408, align 32, !alias.scope !25944, !noalias !25947
  %_81.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %row16.i.i.i.i.i.i, align 32, !alias.scope !25944, !noalias !25947
  %_84.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %409, align 32, !alias.scope !25944, !noalias !25947
  %_87.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %410, align 32, !alias.scope !25944, !noalias !25947
  %_90.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %411, align 32, !alias.scope !25944, !noalias !25947
  %_95.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %row17.i.i.i.i.i.i, align 32, !alias.scope !25944, !noalias !25947
  %_98.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %412, align 32, !alias.scope !25944, !noalias !25947
  %_101.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %413, align 32, !alias.scope !25944, !noalias !25947
  %_104.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %414, align 32, !alias.scope !25944, !noalias !25947
  %_109.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %row18.i.i.i.i.i.i, align 32, !alias.scope !25944, !noalias !25947
  %_112.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %415, align 32, !alias.scope !25944, !noalias !25947
  %_115.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %416, align 32, !alias.scope !25944, !noalias !25947
  %_118.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %417, align 32, !alias.scope !25944, !noalias !25947
  %_123.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %row19.i.i.i.i.i.i, align 32, !alias.scope !25944, !noalias !25947
  %_126.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %418, align 32, !alias.scope !25944, !noalias !25947
  %_129.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %419, align 32, !alias.scope !25944, !noalias !25947
  %_132.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %420, align 32, !alias.scope !25944, !noalias !25947
  %_137.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %row20.i.i.i.i.i.i, align 32, !alias.scope !25944, !noalias !25947
  %_140.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %421, align 32, !alias.scope !25944, !noalias !25947
  %_143.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %422, align 32, !alias.scope !25944, !noalias !25947
  %_146.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %423, align 32, !alias.scope !25944, !noalias !25947
  %_151.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %row21.i.i.i.i.i.i, align 32, !alias.scope !25944, !noalias !25947
  %_154.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %424, align 32, !alias.scope !25944, !noalias !25947
  %_157.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %425, align 32, !alias.scope !25944, !noalias !25947
  %_160.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %426, align 32, !alias.scope !25944, !noalias !25947
  %_165.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %row22.i.i.i.i.i.i, align 32, !alias.scope !25944, !noalias !25947
  %_168.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %427, align 32, !alias.scope !25944, !noalias !25947
  %_171.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %428, align 32, !alias.scope !25944, !noalias !25947
  %_174.i.i.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %429, align 32, !alias.scope !25944, !noalias !25947
  br label %bb5.i.i.i.i, !dbg !27508

bb5.i.i.i.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i, %bb5.i.i.lr.ph.i.i
  %iter.sroa.0.0.i.i3844.i.i = phi i64 [ 0, %bb5.i.i.lr.ph.i.i ], [ %452, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i ]
  %history.i.i.sroa.10.sroa.0.03843.i.i = phi <8 x float> [ %history.i.i.sroa.10.sroa.0.0.copyload.i.i, %bb5.i.i.lr.ph.i.i ], [ %history.i.i.sroa.0.03837.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i ]
  %history.i.i.sroa.13.sroa.0.03842.i.i = phi <8 x float> [ %history.i.i.sroa.13.sroa.0.0.copyload.i.i, %bb5.i.i.lr.ph.i.i ], [ %history.i.i.sroa.10.sroa.0.03843.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i ]
  %history.i.i.sroa.16.sroa.0.03841.i.i = phi <8 x float> [ %history.i.i.sroa.16.sroa.0.0.copyload.i.i, %bb5.i.i.lr.ph.i.i ], [ %history.i.i.sroa.13.sroa.0.03842.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i ]
  %history.i.i.sroa.19.sroa.0.03840.i.i = phi <8 x float> [ %history.i.i.sroa.19.sroa.0.0.copyload.i.i, %bb5.i.i.lr.ph.i.i ], [ %history.i.i.sroa.16.sroa.0.03841.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i ]
  %history.i.i.sroa.22.sroa.0.03839.i.i = phi <8 x float> [ %history.i.i.sroa.22.sroa.0.0.copyload.i.i, %bb5.i.i.lr.ph.i.i ], [ %history.i.i.sroa.19.sroa.0.03840.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i ]
  %history.i.i.sroa.38.sroa.0.03838.i.i = phi <8 x float> [ %history.i.i.sroa.38.sroa.0.0.copyload.i.i, %bb5.i.i.lr.ph.i.i ], [ %history.i.i.sroa.35.sroa.0.03836.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i ]
  %history.i.i.sroa.0.03837.i.i = phi <8 x float> [ %history.i.i.sroa.0.0.copyload.i.i, %bb5.i.i.lr.ph.i.i ], [ %lanes.i1343.sroa.0.0.copyload.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i ]
  %history.i.i.sroa.35.sroa.0.03836.i.i = phi <8 x float> [ %history.i.i.sroa.35.sroa.0.0.copyload.i.i, %bb5.i.i.lr.ph.i.i ], [ %history.i.i.sroa.32.sroa.0.03835.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i ]
  %history.i.i.sroa.32.sroa.0.03835.i.i = phi <8 x float> [ %history.i.i.sroa.32.sroa.0.0.copyload.i.i, %bb5.i.i.lr.ph.i.i ], [ %history.i.i.sroa.29.sroa.0.03834.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i ]
  %history.i.i.sroa.29.sroa.0.03834.i.i = phi <8 x float> [ %history.i.i.sroa.29.sroa.0.0.copyload.i.i, %bb5.i.i.lr.ph.i.i ], [ %history.i.i.sroa.25.sroa.0.03833.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i ]
  %history.i.i.sroa.25.sroa.0.03833.i.i = phi <8 x float> [ %history.i.i.sroa.25.sroa.0.0.copyload.i.i, %bb5.i.i.lr.ph.i.i ], [ %history.i.i.sroa.22.sroa.0.03839.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i ]
  %452 = add nuw nsw i64 %iter.sroa.0.0.i.i3844.i.i, 1, !dbg !27509
  %_11.i.i.i.i = add nuw nsw i64 %iter.sroa.0.0.i.i3844.i.i, %iter2.sroa.0.0.i3984.i.i, !dbg !27512
  %base.i.i.i.i = shl i64 %_11.i.i.i.i, 3, !dbg !27512
  %_24.i.i.i.i = icmp samesign ugt i64 %base.i.i.i.i, %_39.1.i, !dbg !27513
  br i1 %_24.i.i.i.i, label %bb7.i.i.i.i, label %bb8.i.i.i.i, !dbg !27513, !prof !905

bb8.i.i.i.i:                                      ; preds = %bb5.i.i.i.i
  %_27.i.i.i.i = sub nuw nsw i64 %_39.1.i, %base.i.i.i.i, !dbg !27516
  %_8.i1346.i.i = icmp samesign ugt i64 %_27.i.i.i.i, 7, !dbg !27517
  br i1 %_8.i1346.i.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i, label %bb2.i1347.i.i, !dbg !27517, !prof !1076

bb2.i1347.i.i:                                    ; preds = %bb8.i.i.i.i
  store <8 x float> %history.i.i.sroa.0.0.lcssa.i.i1252, ptr %hot_left.i.i.i, align 1, !dbg !27522
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa.i.i1262, ptr %history.i.i.sroa.10.0.hot_left.i.sroa_idx.i.i, align 1, !dbg !27522
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_27.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #31, !dbg !27523, !noalias !27524
  unreachable, !dbg !27523

bb7.i.i.i.i:                                      ; preds = %bb5.i.i.i.i
  store <8 x float> %history.i.i.sroa.0.0.lcssa.i.i1252, ptr %hot_left.i.i.i, align 1, !dbg !27522
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa.i.i1262, ptr %history.i.i.sroa.10.0.hot_left.i.sroa_idx.i.i, align 1, !dbg !27522
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i.i.i.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e0592aef22128a0ac53753b9632a8183) #31, !dbg !27531, !noalias !27532
  unreachable, !dbg !27531

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i: ; preds = %bb8.i.i.i.i
  %_31.i.i.i.i = getelementptr inbounds nuw float, ptr %_39.0.i, i64 %base.i.i.i.i, !dbg !27533
  %lanes.i1343.sroa.0.0.copyload.i.i = load <8 x float>, ptr %_31.i.i.i.i, align 4, !dbg !27535, !alias.scope !27539, !noalias !27543
  %453 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i.i.sroa.22.sroa.0.03839.i.i), !dbg !27545
  %454 = fmul <8 x float> %_5.i1196.i.i, %lanes.i1343.sroa.0.0.copyload.i.i, !dbg !27552
  %455 = fadd <8 x float> %454, zeroinitializer, !dbg !27558
  %456 = fmul <8 x float> %_14.i.i.i.i.sroa.0.0.copyload.i.i, %lanes.i1343.sroa.0.0.copyload.i.i, !dbg !27563
  %457 = fadd <8 x float> %456, zeroinitializer, !dbg !27568
  %458 = fmul <8 x float> %_17.i.i.i.i.sroa.0.0.copyload.i.i, %lanes.i1343.sroa.0.0.copyload.i.i, !dbg !27573
  %459 = fadd <8 x float> %458, zeroinitializer, !dbg !27578
  %460 = fmul <8 x float> %_20.i.i.i.i.sroa.0.0.copyload.i.i, %lanes.i1343.sroa.0.0.copyload.i.i, !dbg !27583
  %461 = fadd <8 x float> %460, zeroinitializer, !dbg !27588
  %462 = fmul <8 x float> %_25.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.0.03837.i.i, !dbg !27593
  %463 = fadd <8 x float> %462, %455, !dbg !27598
  %464 = fmul <8 x float> %_28.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.0.03837.i.i, !dbg !27603
  %465 = fadd <8 x float> %464, %457, !dbg !27608
  %466 = fmul <8 x float> %_31.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.0.03837.i.i, !dbg !27613
  %467 = fadd <8 x float> %466, %459, !dbg !27618
  %468 = fmul <8 x float> %_34.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.0.03837.i.i, !dbg !27623
  %469 = fadd <8 x float> %468, %461, !dbg !27628
  %470 = fmul <8 x float> %_39.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.10.sroa.0.03843.i.i, !dbg !27633
  %471 = fadd <8 x float> %470, %463, !dbg !27638
  %472 = fmul <8 x float> %_42.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.10.sroa.0.03843.i.i, !dbg !27643
  %473 = fadd <8 x float> %472, %465, !dbg !27648
  %474 = fmul <8 x float> %_45.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.10.sroa.0.03843.i.i, !dbg !27653
  %475 = fadd <8 x float> %474, %467, !dbg !27658
  %476 = fmul <8 x float> %_48.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.10.sroa.0.03843.i.i, !dbg !27663
  %477 = fadd <8 x float> %476, %469, !dbg !27668
  %478 = fmul <8 x float> %_53.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.13.sroa.0.03842.i.i, !dbg !27673
  %479 = fadd <8 x float> %478, %471, !dbg !27678
  %480 = fmul <8 x float> %_56.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.13.sroa.0.03842.i.i, !dbg !27683
  %481 = fadd <8 x float> %480, %473, !dbg !27688
  %482 = fmul <8 x float> %_59.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.13.sroa.0.03842.i.i, !dbg !27693
  %483 = fadd <8 x float> %482, %475, !dbg !27698
  %484 = fmul <8 x float> %_62.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.13.sroa.0.03842.i.i, !dbg !27703
  %485 = fadd <8 x float> %484, %477, !dbg !27708
  %486 = fmul <8 x float> %_67.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.16.sroa.0.03841.i.i, !dbg !27713
  %487 = fadd <8 x float> %486, %479, !dbg !27718
  %488 = fmul <8 x float> %_70.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.16.sroa.0.03841.i.i, !dbg !27723
  %489 = fadd <8 x float> %488, %481, !dbg !27728
  %490 = fmul <8 x float> %_73.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.16.sroa.0.03841.i.i, !dbg !27733
  %491 = fadd <8 x float> %490, %483, !dbg !27738
  %492 = fmul <8 x float> %_76.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.16.sroa.0.03841.i.i, !dbg !27743
  %493 = fadd <8 x float> %492, %485, !dbg !27748
  %494 = fmul <8 x float> %_81.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.19.sroa.0.03840.i.i, !dbg !27753
  %495 = fadd <8 x float> %494, %487, !dbg !27758
  %496 = fmul <8 x float> %_84.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.19.sroa.0.03840.i.i, !dbg !27763
  %497 = fadd <8 x float> %496, %489, !dbg !27768
  %498 = fmul <8 x float> %_87.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.19.sroa.0.03840.i.i, !dbg !27773
  %499 = fadd <8 x float> %498, %491, !dbg !27778
  %500 = fmul <8 x float> %_90.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.19.sroa.0.03840.i.i, !dbg !27783
  %501 = fadd <8 x float> %500, %493, !dbg !27788
  %502 = fmul <8 x float> %_95.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.22.sroa.0.03839.i.i, !dbg !27793
  %503 = fadd <8 x float> %502, %495, !dbg !27798
  %504 = fmul <8 x float> %_98.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.22.sroa.0.03839.i.i, !dbg !27803
  %505 = fadd <8 x float> %504, %497, !dbg !27808
  %506 = fmul <8 x float> %_101.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.22.sroa.0.03839.i.i, !dbg !27813
  %507 = fadd <8 x float> %506, %499, !dbg !27818
  %508 = fmul <8 x float> %_104.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.22.sroa.0.03839.i.i, !dbg !27823
  %509 = fadd <8 x float> %508, %501, !dbg !27828
  %510 = fmul <8 x float> %_109.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.25.sroa.0.03833.i.i, !dbg !27833
  %511 = fadd <8 x float> %510, %503, !dbg !27838
  %512 = fmul <8 x float> %_112.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.25.sroa.0.03833.i.i, !dbg !27843
  %513 = fadd <8 x float> %512, %505, !dbg !27848
  %514 = fmul <8 x float> %_115.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.25.sroa.0.03833.i.i, !dbg !27853
  %515 = fadd <8 x float> %514, %507, !dbg !27858
  %516 = fmul <8 x float> %_118.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.25.sroa.0.03833.i.i, !dbg !27863
  %517 = fadd <8 x float> %516, %509, !dbg !27868
  %518 = fmul <8 x float> %_123.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.29.sroa.0.03834.i.i, !dbg !27873
  %519 = fadd <8 x float> %518, %511, !dbg !27878
  %520 = fmul <8 x float> %_126.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.29.sroa.0.03834.i.i, !dbg !27883
  %521 = fadd <8 x float> %520, %513, !dbg !27888
  %522 = fmul <8 x float> %_129.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.29.sroa.0.03834.i.i, !dbg !27893
  %523 = fadd <8 x float> %522, %515, !dbg !27898
  %524 = fmul <8 x float> %_132.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.29.sroa.0.03834.i.i, !dbg !27903
  %525 = fadd <8 x float> %524, %517, !dbg !27908
  %526 = fmul <8 x float> %_137.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.32.sroa.0.03835.i.i, !dbg !27913
  %527 = fadd <8 x float> %526, %519, !dbg !27918
  %528 = fmul <8 x float> %_140.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.32.sroa.0.03835.i.i, !dbg !27923
  %529 = fadd <8 x float> %528, %521, !dbg !27928
  %530 = fmul <8 x float> %_143.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.32.sroa.0.03835.i.i, !dbg !27933
  %531 = fadd <8 x float> %530, %523, !dbg !27938
  %532 = fmul <8 x float> %_146.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.32.sroa.0.03835.i.i, !dbg !27943
  %533 = fadd <8 x float> %532, %525, !dbg !27948
  %534 = fmul <8 x float> %_151.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.35.sroa.0.03836.i.i, !dbg !27953
  %535 = fadd <8 x float> %534, %527, !dbg !27958
  %536 = fmul <8 x float> %_154.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.35.sroa.0.03836.i.i, !dbg !27963
  %537 = fadd <8 x float> %536, %529, !dbg !27968
  %538 = fmul <8 x float> %_157.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.35.sroa.0.03836.i.i, !dbg !27973
  %539 = fadd <8 x float> %538, %531, !dbg !27978
  %540 = fmul <8 x float> %_160.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.35.sroa.0.03836.i.i, !dbg !27983
  %541 = fadd <8 x float> %540, %533, !dbg !27988
  %542 = fmul <8 x float> %_165.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.38.sroa.0.03838.i.i, !dbg !27993
  %543 = fadd <8 x float> %542, %535, !dbg !27998
  %544 = fmul <8 x float> %_168.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.38.sroa.0.03838.i.i, !dbg !28003
  %545 = fadd <8 x float> %544, %537, !dbg !28008
  %546 = fmul <8 x float> %_171.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.38.sroa.0.03838.i.i, !dbg !28013
  %547 = fadd <8 x float> %546, %539, !dbg !28018
  %548 = fmul <8 x float> %_174.i.i.i.i.sroa.0.0.copyload.i.i, %history.i.i.sroa.38.sroa.0.03838.i.i, !dbg !28023
  %549 = fadd <8 x float> %548, %541, !dbg !28028
  %550 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %543), !dbg !28033
  %551 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %453, <8 x float> %550), !dbg !28039
  %552 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %545), !dbg !28033
  %553 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %551, <8 x float> %552), !dbg !28039
  %554 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %547), !dbg !28033
  %555 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %553, <8 x float> %554), !dbg !28039
  %556 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %549), !dbg !28033
  %557 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %555, <8 x float> %556), !dbg !28039
  %_39.i.i.idx.i.i = shl i64 %iter.sroa.0.0.i.i3844.i.i, 5, !dbg !28044
  %_39.i.i.i.i = getelementptr inbounds nuw i8, ptr %peaks_left.i.i.i, i64 %_39.i.i.idx.i.i, !dbg !28044
  store <8 x float> %557, ptr %_39.i.i.i.i, align 4, !dbg !28049, !alias.scope !28054, !noalias !28058
  %exitcond4437.not.i.i = icmp eq i64 %452, %umax4436.i.i, !dbg !27504
  br i1 %exitcond4437.not.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i, label %bb5.i.i.i.i, !dbg !27508

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i, %bb45.i.i.i
  %history.i.i.sroa.25.sroa.0.0.lcssa.i.i = phi <8 x float> [ %history.i.i.sroa.25.sroa.0.0.copyload.i.i, %bb45.i.i.i ], [ %history.i.i.sroa.22.sroa.0.03839.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i ], !dbg !28062
  %history.i.i.sroa.29.sroa.0.0.lcssa.i.i = phi <8 x float> [ %history.i.i.sroa.29.sroa.0.0.copyload.i.i, %bb45.i.i.i ], [ %history.i.i.sroa.25.sroa.0.03833.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i ], !dbg !28062
  %history.i.i.sroa.32.sroa.0.0.lcssa.i.i = phi <8 x float> [ %history.i.i.sroa.32.sroa.0.0.copyload.i.i, %bb45.i.i.i ], [ %history.i.i.sroa.29.sroa.0.03834.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i ], !dbg !28062
  %history.i.i.sroa.35.sroa.0.0.lcssa.i.i = phi <8 x float> [ %history.i.i.sroa.35.sroa.0.0.copyload.i.i, %bb45.i.i.i ], [ %history.i.i.sroa.32.sroa.0.03835.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i ], !dbg !28062
  %history.i.i.sroa.0.0.lcssa.i.i = phi <8 x float> [ %history.i.i.sroa.0.0.copyload.i.i, %bb45.i.i.i ], [ %lanes.i1343.sroa.0.0.copyload.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i ], !dbg !28062
  %history.i.i.sroa.38.sroa.0.0.lcssa.i.i = phi <8 x float> [ %history.i.i.sroa.38.sroa.0.0.copyload.i.i, %bb45.i.i.i ], [ %history.i.i.sroa.35.sroa.0.03836.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i ], !dbg !28062
  %history.i.i.sroa.41.sroa.0.0.lcssa.i.i = phi <8 x float> [ %history.i.i.sroa.41.sroa.0.0.copyload.i.i, %bb45.i.i.i ], [ %history.i.i.sroa.38.sroa.0.03838.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i ], !dbg !28062
  %history.i.i.sroa.22.sroa.0.0.lcssa.i.i = phi <8 x float> [ %history.i.i.sroa.22.sroa.0.0.copyload.i.i, %bb45.i.i.i ], [ %history.i.i.sroa.19.sroa.0.03840.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i ], !dbg !28062
  %history.i.i.sroa.19.sroa.0.0.lcssa.i.i = phi <8 x float> [ %history.i.i.sroa.19.sroa.0.0.copyload.i.i, %bb45.i.i.i ], [ %history.i.i.sroa.16.sroa.0.03841.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i ], !dbg !28062
  %history.i.i.sroa.16.sroa.0.0.lcssa.i.i = phi <8 x float> [ %history.i.i.sroa.16.sroa.0.0.copyload.i.i, %bb45.i.i.i ], [ %history.i.i.sroa.13.sroa.0.03842.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i ], !dbg !28062
  %history.i.i.sroa.13.sroa.0.0.lcssa.i.i = phi <8 x float> [ %history.i.i.sroa.13.sroa.0.0.copyload.i.i, %bb45.i.i.i ], [ %history.i.i.sroa.10.sroa.0.03843.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i ], !dbg !28062
  %history.i.i.sroa.10.sroa.0.0.lcssa.i.i = phi <8 x float> [ %history.i.i.sroa.10.sroa.0.0.copyload.i.i, %bb45.i.i.i ], [ %history.i.i.sroa.0.03837.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1557.i.i ], !dbg !28062
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa.i.i, ptr %history.i.i.sroa.13.0.hot_left.i.sroa_idx.i.i, align 32, !dbg !27522, !noalias !26271
  store <8 x float> %history.i.i.sroa.16.sroa.0.0.lcssa.i.i, ptr %history.i.i.sroa.16.0.hot_left.i.sroa_idx.i.i, align 32, !dbg !27522, !noalias !26271
  store <8 x float> %history.i.i.sroa.19.sroa.0.0.lcssa.i.i, ptr %history.i.i.sroa.19.0.hot_left.i.sroa_idx.i.i, align 32, !dbg !27522, !noalias !26271
  store <8 x float> %history.i.i.sroa.22.sroa.0.0.lcssa.i.i, ptr %history.i.i.sroa.22.0.hot_left.i.sroa_idx.i.i, align 32, !dbg !27522, !noalias !26271
  store <8 x float> %history.i.i.sroa.25.sroa.0.0.lcssa.i.i, ptr %history.i.i.sroa.25.0.hot_left.i.sroa_idx.i.i, align 32, !dbg !27522, !noalias !26271
  store <8 x float> %history.i.i.sroa.29.sroa.0.0.lcssa.i.i, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i.i, align 32, !dbg !27522, !noalias !26271
  store <8 x float> %history.i.i.sroa.32.sroa.0.0.lcssa.i.i, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i.i, align 32, !dbg !27522, !noalias !26271
  store <8 x float> %history.i.i.sroa.35.sroa.0.0.lcssa.i.i, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i.i, align 32, !dbg !27522, !noalias !26271
  store <8 x float> %history.i.i.sroa.38.sroa.0.0.lcssa.i.i, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i.i, align 32, !dbg !27522, !noalias !26271
  store <8 x float> %history.i.i.sroa.41.sroa.0.0.lcssa.i.i, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i.i, align 32, !dbg !27522, !noalias !26271
  br i1 %_20.i.i3832.not.i.i, label %bb18.i.loopexit.i.i, label %bb22.i.i.i, !dbg !27491

bb22.i.i.i:                                       ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i, %bb30.i.i.i
  %_12.i434.sroa.0.0.copyload3905.i.i.lcssa8901244 = phi <8 x float> [ %_12.i434.sroa.0.0.copyload3905.i.i.lcssa8901243, %bb30.i.i.i ], [ %_12.i434.sroa.0.0.copyload3905.i.i.lcssa8901243.lcssa1280, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %_11.i435.sroa.0.0.copyload3889.i.i.lcssa8831234 = phi <8 x float> [ %_11.i435.sroa.0.0.copyload3889.i.i.lcssa8831233, %bb30.i.i.i ], [ %_11.i435.sroa.0.0.copyload3889.i.i.lcssa8831233.lcssa1277, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %.lcssa9231216.i = phi <8 x float> [ %.lcssa9231215.i, %bb30.i.i.i ], [ %.lcssa9231215.i.lcssa1291, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %.lcssa9031206.i = phi <8 x float> [ %.lcssa9031205.i, %bb30.i.i.i ], [ %.lcssa9031205.i.lcssa1288, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %.lcssa8951196.i = phi <8 x float> [ %.lcssa8951195.i, %bb30.i.i.i ], [ %.lcssa8951195.i.lcssa1285, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %.lcssa8871187.i = phi <8 x float> [ %.lcssa8871186.i, %bb30.i.i.i ], [ %.lcssa8871186.i.lcssa1283, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %.lcssa49975026.i.i = phi <8 x float> [ %.lcssa49975025.i.i, %bb30.i.i.i ], [ %.lcssa9231215.i.lcssa1291, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %.lcssa49885023.i.i = phi <8 x float> [ %.lcssa49885022.i.i, %bb30.i.i.i ], [ %.lcssa9031205.i.lcssa1288, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %.lcssa49795020.i.i = phi <8 x float> [ %.lcssa49795019.i.i, %bb30.i.i.i ], [ %.lcssa8951195.i.lcssa1285, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %.lcssa49705017.i.i = phi <8 x float> [ %.lcssa49705016.i.i, %bb30.i.i.i ], [ %.lcssa8871186.i.lcssa1283, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %_12.i434.sroa.0.0.copyload3905.lcssa46955014.i.i = phi <8 x float> [ %_12.i434.sroa.0.0.copyload3905.lcssa46955013.i.i, %bb30.i.i.i ], [ %_12.i434.sroa.0.0.copyload3905.i.i.lcssa8901243.lcssa1280, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %_11.i435.sroa.0.0.copyload3889.lcssa46885011.i.i = phi <8 x float> [ %_11.i435.sroa.0.0.copyload3889.lcssa46885010.i.i, %bb30.i.i.i ], [ %_11.i435.sroa.0.0.copyload3889.i.i.lcssa8831233.lcssa1277, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %.lcssa46815000.i.i = phi <8 x float> [ %.lcssa46814999.i.i, %bb30.i.i.i ], [ %.lcssa46814999.i.i.lcssa1274, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %storemerge.i.i.lcssa39373963.i.i = phi i32 [ %storemerge.i.i.lcssa39373962.i.i, %bb30.i.i.i ], [ %storemerge.i.i.lcssa39373962.lcssa5039.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %frame.sroa.0.0.i3957.i.i = phi i64 [ %_65.i.i.i, %bb30.i.i.i ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %main_cursor.sroa.0.1.i3956.i.i = phi i64 [ %main_cursor.sroa.0.2.i.i.i, %bb30.i.i.i ], [ %main_cursor.sroa.0.0.i3983.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %ring_cursor.sroa.0.1.i3955.i.i = phi i64 [ %ring_cursor.sroa.0.2.i.i.i, %bb30.i.i.i ], [ %ring_cursor.sroa.0.0.i3982.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %minimum.i.i.sroa.0.03861.lcssa39433954.i.i = phi <8 x float> [ %minimum.i.i.sroa.0.03861.lcssa.i.i, %bb30.i.i.i ], [ %minimum.i.i.sroa.0.03861.lcssa3943.lcssa5027.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i ]
  %_49.i.i.i = sub nuw nsw i64 %..i1802.i.i, %frame.sroa.0.0.i3957.i.i, !dbg !28063
  %ring.i375.i.i = load i64, ptr %385, align 8, !dbg !28064, !alias.scope !28066, !noalias !28069, !noundef !12
  %main.i376.i.i = load i64, ptr %386, align 8, !dbg !28073, !alias.scope !28066, !noalias !28069, !noundef !12
  %_10.i.i.i = add i64 %ring_cursor.sroa.0.1.i3955.i.i, 1, !dbg !28074
  %_45.not.i.i.i = icmp ult i64 %_10.i.i.i, %ring.i375.i.i, !dbg !28075
  %558 = select i1 %_45.not.i.i.i, i64 0, i64 %ring.i375.i.i, !dbg !28075
  %start1.sroa.0.0.i377.i.i = sub nuw i64 %_10.i.i.i, %558, !dbg !28075
  %_12.i378.i.i = add i64 %ring_cursor.sroa.0.1.i3955.i.i, %_51.i.sroa.3.0.copyload.pre.i.i, !dbg !28077
  %_46.not.i.i.i = icmp ult i64 %_12.i378.i.i, %ring.i375.i.i, !dbg !28078
  %559 = select i1 %_46.not.i.i.i, i64 0, i64 %ring.i375.i.i, !dbg !28078
  %left_end.sroa.0.0.i.i.i = sub nuw i64 %_12.i378.i.i, %559, !dbg !28078
  %_18.i382.i.i = add i64 %ring_cursor.sroa.0.1.i3955.i.i, %_51.i.sroa.4.0.copyload.pre.i.i, !dbg !28080
  %_48.not.i.i.i = icmp ult i64 %_18.i382.i.i, %ring.i375.i.i, !dbg !28081
  %560 = select i1 %_48.not.i.i.i, i64 0, i64 %ring.i375.i.i, !dbg !28081
  %left_expiring.sroa.0.0.i.i.i = sub nuw i64 %_18.i382.i.i, %560, !dbg !28081
  %_30.i384.i.i = sub i64 %ring.i375.i.i, %ring_cursor.sroa.0.1.i3955.i.i, !dbg !28083
  %..i1815.i.i = tail call noundef i64 @llvm.umin.i64(i64 %_30.i384.i.i, i64 %_49.i.i.i), !dbg !28084
  %_31.i.i.i = sub i64 %main.i376.i.i, %main_cursor.sroa.0.1.i3956.i.i, !dbg !28086
  %..i1816.i.i = tail call noundef i64 @llvm.umin.i64(i64 %_31.i.i.i, i64 %..i1815.i.i), !dbg !28087
  %_32.i387.i.i = sub i64 %ring.i375.i.i, %start1.sroa.0.0.i377.i.i, !dbg !28089
  %..i1817.i.i = tail call noundef i64 @llvm.umin.i64(i64 %_32.i387.i.i, i64 %..i1816.i.i), !dbg !28090
  %_34.i389.i.i = sub i64 %ring.i375.i.i, %left_end.sroa.0.0.i.i.i, !dbg !28092
  %..i1818.i.i = tail call noundef i64 @llvm.umin.i64(i64 %_34.i389.i.i, i64 %..i1817.i.i), !dbg !28093
  %_38.i.i.i = sub i64 %ring.i375.i.i, %left_expiring.sroa.0.0.i.i.i, !dbg !28095
  %..i1820.i.i = tail call noundef i64 @llvm.umin.i64(i64 %_38.i.i.i, i64 %..i1818.i.i), !dbg !28096
  %_55.i.i.i = add i64 %frame.sroa.0.0.i3957.i.i, %iter2.sroa.0.0.i3984.i.i, !dbg !28098
  %base.i.i.i = shl i64 %_55.i.i.i, 3, !dbg !28098
  %base.i3291.i.i = add i64 %..i1820.i.i, %_55.i.i.i, !dbg !28099
  %_59.i.i.i = shl i64 %base.i3291.i.i, 3, !dbg !28099
  %_124.i.i.i = icmp ult i64 %_59.i.i.i, %base.i.i.i, !dbg !28100
  %_118.not.i.i.i = icmp ugt i64 %_59.i.i.i, %_39.1.i
  %or.cond.i.i.i = or i1 %_124.i.i.i, %_118.not.i.i.i, !dbg !28100
  br i1 %or.cond.i.i.i, label %bb50.i.i.i, label %bb48.i.i.i, !dbg !28100, !prof !5262

bb48.i.i.i:                                       ; preds = %bb22.i.i.i
  %_127.i.i.i = getelementptr inbounds nuw float, ptr %_39.0.i, i64 %base.i.i.i, !dbg !28107
  %_65.i.i.i = add nuw nsw i64 %..i1820.i.i, %frame.sroa.0.0.i3957.i.i, !dbg !28111
  %_136.i.i.idx.i = shl nuw nsw i64 %frame.sroa.0.0.i3957.i.i, 5, !dbg !28112
  %_136.i.i.i = getelementptr inbounds nuw i8, ptr %peaks_left.i.i.i, i64 %_136.i.i.idx.i, !dbg !28112
  %_2.i.i.i3868.not.i.i = icmp eq i64 %..i1820.i.i, 0, !dbg !28122
  br i1 %_2.i.i.i3868.not.i.i, label %bb30.i.i.i, label %bb29.i.lr.ph.i.i, !dbg !28122

bb50.i.i.i:                                       ; preds = %bb22.i.i.i
  store <8 x float> %history.i.i.sroa.0.0.lcssa.i.i, ptr %hot_left.i.i.i, align 1, !dbg !27522
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa.i.i, ptr %history.i.i.sroa.10.0.hot_left.i.sroa_idx.i.i, align 1, !dbg !27522
  store <8 x float> %_11.i435.sroa.0.0.copyload3889.i.i.lcssa8831234, ptr %_80.i.i.i, align 1, !dbg !27468
  store <8 x float> %_12.i434.sroa.0.0.copyload3905.i.i.lcssa8901244, ptr %432, align 1, !dbg !27478
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i.i.i, i64 noundef %_59.i.i.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b0f5241d7ea6b85fff91278ccec58019) #31, !dbg !28126, !noalias !28127
  unreachable, !dbg !28126

bb29.i.lr.ph.i.i:                                 ; preds = %bb48.i.i.i
  %umin4446.i.i = tail call i64 @llvm.umin.i64(i64 %_34.i389.i.i, i64 %_38.i.i.i), !dbg !28122
  %umin4447.i.i = tail call i64 @llvm.umin.i64(i64 %umin4446.i.i, i64 %_32.i387.i.i), !dbg !28122
  %umin4448.i.i = tail call i64 @llvm.umin.i64(i64 %umin4447.i.i, i64 %_30.i384.i.i), !dbg !28122
  %umin4449.i.i = tail call i64 @llvm.umin.i64(i64 %umin4448.i.i, i64 %_31.i.i.i), !dbg !28122
  %561 = sub nsw i64 %umin4450.i.i, %frame.sroa.0.0.i3957.i.i, !dbg !28122
  %umin4451.i.i = tail call i64 @llvm.umin.i64(i64 %umin4449.i.i, i64 %561), !dbg !28122
  %562 = and i64 %umin4451.i.i, 2305843009213693951, !dbg !28122
  br label %bb29.i.i.i, !dbg !28122

bb29.i.i.i:                                       ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i, %bb29.i.lr.ph.i.i
  %563 = phi <8 x float> [ %.lcssa49975026.i.i, %bb29.i.lr.ph.i.i ], [ %616, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %564 = phi <8 x float> [ %.lcssa49885023.i.i, %bb29.i.lr.ph.i.i ], [ %588, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %565 = phi <8 x float> [ %.lcssa49795020.i.i, %bb29.i.lr.ph.i.i ], [ %589, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %566 = phi <8 x float> [ %.lcssa49705017.i.i, %bb29.i.lr.ph.i.i ], [ %590, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %storemerge.i.i3927.i.i = phi i32 [ %storemerge.i.i.lcssa39373963.i.i, %bb29.i.lr.ph.i.i ], [ %storemerge.i.i.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %_12.i420.sroa.0.0.copyload3926.i.i = phi <8 x float> [ %.lcssa49885023.i.i, %bb29.i.lr.ph.i.i ], [ %_12.i420.sroa.0.0.copyload3925.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %_11.i421.sroa.0.0.copyload3923.i.i = phi <8 x float> [ %.lcssa49795020.i.i, %bb29.i.lr.ph.i.i ], [ %_11.i421.sroa.0.0.copyload3922.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %567 = phi <8 x float> [ %.lcssa49705017.i.i, %bb29.i.lr.ph.i.i ], [ %591, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %_12.i434.sroa.0.0.copyload3906.i.i = phi <8 x float> [ %_12.i434.sroa.0.0.copyload3905.lcssa46955014.i.i, %bb29.i.lr.ph.i.i ], [ %_12.i434.sroa.0.0.copyload3905.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %_11.i435.sroa.0.0.copyload3890.i.i = phi <8 x float> [ %_11.i435.sroa.0.0.copyload3889.lcssa46885011.i.i, %bb29.i.lr.ph.i.i ], [ %_11.i435.sroa.0.0.copyload3889.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %568 = phi <8 x float> [ %.lcssa46815000.i.i, %bb29.i.lr.ph.i.i ], [ %592, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %iter.i.sroa.16.03871.i.i = phi i64 [ 0, %bb29.i.lr.ph.i.i ], [ %569, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %minimum.i.i.sroa.0.038613869.i.i = phi <8 x float> [ %minimum.i.i.sroa.0.03861.lcssa39433954.i.i, %bb29.i.lr.ph.i.i ], [ %minimum.i.i.sroa.0.0.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %569 = add nuw nsw i64 %iter.i.sroa.16.03871.i.i, 1, !dbg !28128
  %start1.i.i.i.i.i.i = shl i64 %iter.i.sroa.16.03871.i.i, 3, !dbg !28129
  %data.i.i.i.i1831.i.i = getelementptr inbounds nuw float, ptr %_127.i.i.i, i64 %start1.i.i.i.i.i.i, !dbg !28131
  %data.i4.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_136.i.i.i, i64 %start1.i.i.i.i.i.i, !dbg !28133
  br i1 %stationary.sroa.0.0.i.i.i, label %bb35.i.i.i, label %bb32.i.i.i, !dbg !28136

bb30.i.i.i:                                       ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i, %bb48.i.i.i
  %_12.i434.sroa.0.0.copyload3905.i.i.lcssa8901243 = phi <8 x float> [ %_12.i434.sroa.0.0.copyload3905.i.i.lcssa8901244, %bb48.i.i.i ], [ %_12.i434.sroa.0.0.copyload3905.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %_11.i435.sroa.0.0.copyload3889.i.i.lcssa8831233 = phi <8 x float> [ %_11.i435.sroa.0.0.copyload3889.i.i.lcssa8831234, %bb48.i.i.i ], [ %_11.i435.sroa.0.0.copyload3889.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %.lcssa9231215.i = phi <8 x float> [ %.lcssa9231216.i, %bb48.i.i.i ], [ %616, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %.lcssa9031205.i = phi <8 x float> [ %.lcssa9031206.i, %bb48.i.i.i ], [ %588, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %.lcssa8951195.i = phi <8 x float> [ %.lcssa8951196.i, %bb48.i.i.i ], [ %589, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %.lcssa8871186.i = phi <8 x float> [ %.lcssa8871187.i, %bb48.i.i.i ], [ %590, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %.lcssa49975025.i.i = phi <8 x float> [ %.lcssa49975026.i.i, %bb48.i.i.i ], [ %616, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %.lcssa49885022.i.i = phi <8 x float> [ %.lcssa49885023.i.i, %bb48.i.i.i ], [ %588, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %.lcssa49795019.i.i = phi <8 x float> [ %.lcssa49795020.i.i, %bb48.i.i.i ], [ %589, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %.lcssa49705016.i.i = phi <8 x float> [ %.lcssa49705017.i.i, %bb48.i.i.i ], [ %590, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %_12.i434.sroa.0.0.copyload3905.lcssa46955013.i.i = phi <8 x float> [ %_12.i434.sroa.0.0.copyload3905.lcssa46955014.i.i, %bb48.i.i.i ], [ %_12.i434.sroa.0.0.copyload3905.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %_11.i435.sroa.0.0.copyload3889.lcssa46885010.i.i = phi <8 x float> [ %_11.i435.sroa.0.0.copyload3889.lcssa46885011.i.i, %bb48.i.i.i ], [ %_11.i435.sroa.0.0.copyload3889.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %.lcssa46814999.i.i = phi <8 x float> [ %.lcssa46815000.i.i, %bb48.i.i.i ], [ %592, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %storemerge.i.i.lcssa39373962.i.i = phi i32 [ %storemerge.i.i.lcssa39373963.i.i, %bb48.i.i.i ], [ %storemerge.i.i.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %minimum.i.i.sroa.0.03861.lcssa.i.i = phi <8 x float> [ %minimum.i.i.sroa.0.03861.lcssa39433954.i.i, %bb48.i.i.i ], [ %minimum.i.i.sroa.0.0.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i ]
  %_95.i.i.i = add i64 %..i1820.i.i, %ring_cursor.sroa.0.1.i3955.i.i, !dbg !28137
  %_137.not.i.i.i = icmp ult i64 %_95.i.i.i, %ring.i.i.i, !dbg !28138
  %570 = select i1 %_137.not.i.i.i, i64 0, i64 %ring.i.i.i, !dbg !28138
  %ring_cursor.sroa.0.2.i.i.i = sub nuw i64 %_95.i.i.i, %570, !dbg !28138
  %_97.i.i.i = add i64 %..i1820.i.i, %main_cursor.sroa.0.1.i3956.i.i, !dbg !28141
  %_143.not.i.i.i = icmp ult i64 %_97.i.i.i, %main.i.i.i, !dbg !28142
  %571 = select i1 %_143.not.i.i.i, i64 0, i64 %main.i.i.i, !dbg !28142
  %main_cursor.sroa.0.2.i.i.i = sub nuw i64 %_97.i.i.i, %571, !dbg !28142
  %_44.i.i.i = icmp ult i64 %_65.i.i.i, %..i1802.i.i, !dbg !27491
  br i1 %_44.i.i.i, label %bb22.i.i.i, label %bb21.i.bb18.i.loopexit_crit_edge.i.i, !dbg !27491

bb32.i.i.i:                                       ; preds = %bb29.i.i.i
  %572 = fadd <8 x float> %568, splat (float -1.000000e+00), !dbg !28144
  %573 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %572, <8 x float> zeroinitializer), !dbg !28149
  %574 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %573, <8 x float> zeroinitializer, i8 30), !dbg !28154
  %575 = fadd <8 x float> %_12.i434.sroa.0.0.copyload3906.i.i, %_11.i435.sroa.0.0.copyload3890.i.i, !dbg !28160
  %576 = bitcast <8 x float> %574 to <8 x i32>, !dbg !28165
  %577 = icmp slt <8 x i32> %576, zeroinitializer, !dbg !28169
  %578 = select <8 x i1> %577, <8 x float> %575, <8 x float> %_13.i433.sroa.0.0.copyload.i.i, !dbg !28169
  %579 = select <8 x i1> %577, <8 x float> %_12.i434.sroa.0.0.copyload3906.i.i, <8 x float> zeroinitializer, !dbg !28171
  %580 = fadd <8 x float> %567, splat (float -1.000000e+00), !dbg !28176
  %581 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %580, <8 x float> zeroinitializer), !dbg !28181
  %582 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %581, <8 x float> zeroinitializer, i8 30), !dbg !28186
  %583 = fadd <8 x float> %_12.i420.sroa.0.0.copyload3926.i.i, %_11.i421.sroa.0.0.copyload3923.i.i, !dbg !28192
  %584 = bitcast <8 x float> %582 to <8 x i32>, !dbg !28197
  %585 = icmp slt <8 x i32> %584, zeroinitializer, !dbg !28201
  %586 = select <8 x i1> %585, <8 x float> %583, <8 x float> %_13.i419.sroa.0.0.copyload.i.i, !dbg !28201
  %587 = select <8 x i1> %585, <8 x float> %_12.i420.sroa.0.0.copyload3926.i.i, <8 x float> zeroinitializer, !dbg !28203
  br label %bb35.i.i.i, !dbg !28208

bb35.i.i.i:                                       ; preds = %bb32.i.i.i, %bb29.i.i.i
  %588 = phi <8 x float> [ %587, %bb32.i.i.i ], [ %564, %bb29.i.i.i ]
  %589 = phi <8 x float> [ %586, %bb32.i.i.i ], [ %565, %bb29.i.i.i ]
  %590 = phi <8 x float> [ %581, %bb32.i.i.i ], [ %566, %bb29.i.i.i ]
  %_12.i420.sroa.0.0.copyload3925.i.i = phi <8 x float> [ %587, %bb32.i.i.i ], [ %_12.i420.sroa.0.0.copyload3926.i.i, %bb29.i.i.i ]
  %_11.i421.sroa.0.0.copyload3922.i.i = phi <8 x float> [ %586, %bb32.i.i.i ], [ %_11.i421.sroa.0.0.copyload3923.i.i, %bb29.i.i.i ]
  %591 = phi <8 x float> [ %581, %bb32.i.i.i ], [ %567, %bb29.i.i.i ]
  %_12.i434.sroa.0.0.copyload3905.i.i = phi <8 x float> [ %579, %bb32.i.i.i ], [ %_12.i434.sroa.0.0.copyload3906.i.i, %bb29.i.i.i ]
  %_11.i435.sroa.0.0.copyload3889.i.i = phi <8 x float> [ %578, %bb32.i.i.i ], [ %_11.i435.sroa.0.0.copyload3890.i.i, %bb29.i.i.i ]
  %592 = phi <8 x float> [ %573, %bb32.i.i.i ], [ %568, %bb29.i.i.i ]
  %_138.i.i.i = add i64 %iter.i.sroa.16.03871.i.i, %ring_cursor.sroa.0.1.i3955.i.i, !dbg !28209
  %_139.i.i.i = add i64 %iter.i.sroa.16.03871.i.i, %main_cursor.sroa.0.1.i3956.i.i, !dbg !28212
  %_140.i.i.i = add i64 %iter.i.sroa.16.03871.i.i, %left_end.sroa.0.0.i.i.i, !dbg !28213
  %_141.i.i.i = add i64 %iter.i.sroa.16.03871.i.i, %start1.sroa.0.0.i377.i.i, !dbg !28214
  %_142.i.i.i = add i64 %iter.i.sroa.16.03871.i.i, %left_expiring.sroa.0.0.i.i.i, !dbg !28215
  %base.i9.i.i.i.i = shl i64 %_138.i.i.i, 3, !dbg !28216
  %_7.i10.i.i.i.i = add i64 %base.i9.i.i.i.i, 8, !dbg !28218
  %593 = or disjoint i64 %base.i9.i.i.i.i, 7, !dbg !28219
  %or.cond.i13.i.i.not.i.i = icmp ult i64 %593, %_54.1.i.i.i.i, !dbg !28219
  br i1 %or.cond.i13.i.i.not.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i.i.i, label %bb4.i15.i.i.i.i, !dbg !28219, !prof !9914

bb4.i15.i.i.i.i:                                  ; preds = %bb35.i.i.i
  store <8 x float> %history.i.i.sroa.0.0.lcssa.i.i, ptr %hot_left.i.i.i, align 1, !dbg !27522
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa.i.i, ptr %history.i.i.sroa.10.0.hot_left.i.sroa_idx.i.i, align 1, !dbg !27522
  store <8 x float> %_11.i435.sroa.0.0.copyload3889.i.i.lcssa8831234, ptr %_80.i.i.i, align 1, !dbg !27468
  store <8 x float> %_12.i434.sroa.0.0.copyload3905.i.i.lcssa8901244, ptr %432, align 1, !dbg !27478
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i.i.i.i, i64 noundef %_7.i10.i.i.i.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cd0e502fea74c9eb9984d521d7f3533e) #31, !dbg !28223, !noalias !28224
  unreachable, !dbg !28223

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i.i.i: ; preds = %bb35.i.i.i
  %lanes.i1361.sroa.0.0.copyload.i.i = load <8 x float>, ptr %data.i4.i.i.i.i.i, align 4, !dbg !28238, !alias.scope !28243, !noalias !28247
  %594 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i1361.sroa.0.0.copyload.i.i, <8 x float> %lanes.i1361.sroa.0.0.copyload.i.i), !dbg !28251
  %595 = select <8 x i1> %438, <8 x float> %594, <8 x float> %lanes.i1361.sroa.0.0.copyload.i.i, !dbg !28256
  %596 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %595, <8 x float> %_11.i435.sroa.0.0.copyload3889.i.i, i8 30), !dbg !28261
  %597 = bitcast <8 x float> %596 to <8 x i32>, !dbg !28267
  %598 = icmp slt <8 x i32> %597, zeroinitializer, !dbg !28271
  %599 = fdiv <8 x float> %_11.i435.sroa.0.0.copyload3889.i.i, %595, !dbg !28273
  %600 = select <8 x i1> %598, <8 x float> %599, <8 x float> splat (float 1.000000e+00), !dbg !28271
  %_17.i14.i.i.i.i = getelementptr inbounds nuw float, ptr %_54.0.i.i.i.i, i64 %base.i9.i.i.i.i, !dbg !28278
  store <8 x float> %600, ptr %_17.i14.i.i.i.i, align 4, !dbg !28280, !alias.scope !28285, !noalias !28289
  %base.i324.i.i = shl i64 %_140.i.i.i, 3, !dbg !28293
  %601 = or disjoint i64 %base.i324.i.i, 7, !dbg !28296
  %or.cond.i328.not.i.i = icmp ult i64 %601, %_54.1.i.i.i.i, !dbg !28296
  br i1 %or.cond.i328.not.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit331.i.i, label %bb4.i330.i.i, !dbg !28296, !prof !9914

bb4.i330.i.i:                                     ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i.i.i
  store <8 x float> %history.i.i.sroa.0.0.lcssa.i.i, ptr %hot_left.i.i.i, align 1, !dbg !27522
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa.i.i, ptr %history.i.i.sroa.10.0.hot_left.i.sroa_idx.i.i, align 1, !dbg !27522
  store <8 x float> %_11.i435.sroa.0.0.copyload3889.i.i.lcssa8831234, ptr %_80.i.i.i, align 1, !dbg !27468
  store <8 x float> %_12.i434.sroa.0.0.copyload3905.i.i.lcssa8901244, ptr %432, align 1, !dbg !27478
  %_5.i325.i.i = add i64 %base.i324.i.i, 8, !dbg !28300
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i324.i.i, i64 noundef %_5.i325.i.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !28301, !noalias !28302
  unreachable, !dbg !28301

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit331.i.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i.i.i
  %_15.i329.i.i = getelementptr inbounds nuw float, ptr %_54.0.i.i.i.i, i64 %base.i324.i.i, !dbg !28310
  %lanes.i1251.sroa.0.0.copyload.i.i = load <8 x float>, ptr %_15.i329.i.i, align 4, !dbg !28312, !alias.scope !28317, !noalias !28321
  %position.i.i.i.i = zext i32 %storemerge.i.i3927.i.i to i64, !dbg !28325
  %602 = icmp eq i32 %storemerge.i.i3927.i.i, 0, !dbg !28326
  br i1 %602, label %bb5.i32.i.i.i, label %bb3.i.i.i.i, !dbg !28326

bb3.i.i.i.i:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit331.i.i
  %603 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %minimum.i.i.sroa.0.038613869.i.i, <8 x float> %lanes.i1251.sroa.0.0.copyload.i.i), !dbg !28327
  br label %bb5.i32.i.i.i, !dbg !28332

bb5.i32.i.i.i:                                    ; preds = %bb3.i.i.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit331.i.i
  %minimum.i.i.sroa.0.0.i.i = phi <8 x float> [ %603, %bb3.i.i.i.i ], [ %lanes.i1251.sroa.0.0.copyload.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit331.i.i ], !dbg !28333
  %_15.i33.i.i.i = add nuw nsw i64 %position.i.i.i.i, 1, !dbg !28334
  %complete.i.i.i.i = icmp eq i64 %_15.i33.i.i.i, %_18.i21.i.i.i, !dbg !28334
  br i1 %complete.i.i.i.i, label %bb19.i.i.i.i, label %bb7.i34.i.i.i, !dbg !28335

bb7.i34.i.i.i:                                    ; preds = %bb5.i32.i.i.i
  %base.i316.i.i = shl i64 %_141.i.i.i, 3, !dbg !28336
  %604 = or disjoint i64 %base.i316.i.i, 7, !dbg !28338
  %or.cond.i320.not.i.i = icmp ult i64 %604, %_54.1.i.i.i.i, !dbg !28338
  br i1 %or.cond.i320.not.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit323.i.i, label %bb4.i322.i.i, !dbg !28338, !prof !9914

bb4.i322.i.i:                                     ; preds = %bb7.i34.i.i.i
  store <8 x float> %history.i.i.sroa.0.0.lcssa.i.i, ptr %hot_left.i.i.i, align 1, !dbg !27522
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa.i.i, ptr %history.i.i.sroa.10.0.hot_left.i.sroa_idx.i.i, align 1, !dbg !27522
  store <8 x float> %_11.i435.sroa.0.0.copyload3889.i.i.lcssa8831234, ptr %_80.i.i.i, align 1, !dbg !27468
  store <8 x float> %_12.i434.sroa.0.0.copyload3905.i.i.lcssa8901244, ptr %432, align 1, !dbg !27478
  %_5.i317.i.i = add i64 %base.i316.i.i, 8, !dbg !28342
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i316.i.i, i64 noundef %_5.i317.i.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !28343, !noalias !28344
  unreachable, !dbg !28343

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit323.i.i: ; preds = %bb7.i34.i.i.i
  %_15.i321.i.i = getelementptr inbounds nuw float, ptr %_54.0.i.i.i.i, i64 %base.i316.i.i, !dbg !28348
  %lanes.i1258.sroa.0.0.copyload.i.i = load <8 x float>, ptr %_15.i321.i.i, align 4, !dbg !28350, !alias.scope !28355, !noalias !28359
  %605 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %lanes.i1258.sroa.0.0.copyload.i.i, <8 x float> %minimum.i.i.sroa.0.0.i.i), !dbg !28363
  %606 = trunc i64 %_15.i33.i.i.i to i32, !dbg !28368
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i, !dbg !28369

bb19.i.i.i.i:                                     ; preds = %bb5.i32.i.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i
  %end.sroa.0.0.i.i3860.i.i = phi i64 [ %610, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i ], [ %_140.i.i.i, %bb5.i32.i.i.i ]
  %iter.sroa.0.0.i35.i3859.i.i = phi i64 [ %_30.i36.i.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i ], [ 0, %bb5.i32.i.i.i ]
  %suffix.i.i.sroa.0.03858.i.i = phi <8 x float> [ %608, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i ], [ %lanes.i1251.sroa.0.0.copyload.i.i, %bb5.i32.i.i.i ]
  %base.i290.i.i = shl i64 %end.sroa.0.0.i.i3860.i.i, 3, !dbg !28370
  %607 = or disjoint i64 %base.i290.i.i, 7, !dbg !28372
  %or.cond.i291.not.i.i = icmp ult i64 %607, %_54.1.i.i.i.i, !dbg !28372
  br i1 %or.cond.i291.not.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i, label %bb4.i.i.i, !dbg !28372, !prof !9914

bb4.i.i.i:                                        ; preds = %bb19.i.i.i.i
  store <8 x float> %history.i.i.sroa.0.0.lcssa.i.i, ptr %hot_left.i.i.i, align 1, !dbg !27522
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa.i.i, ptr %history.i.i.sroa.10.0.hot_left.i.sroa_idx.i.i, align 1, !dbg !27522
  store <8 x float> %_11.i435.sroa.0.0.copyload3889.i.i.lcssa8831234, ptr %_80.i.i.i, align 1, !dbg !27468
  store <8 x float> %_12.i434.sroa.0.0.copyload3905.i.i.lcssa8901244, ptr %432, align 1, !dbg !27478
  %_5.i.i.i = add i64 %base.i290.i.i, 8, !dbg !28376
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i290.i.i, i64 noundef %_5.i.i.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !28377, !noalias !28378
  unreachable, !dbg !28377

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i: ; preds = %bb19.i.i.i.i
  %_30.i36.i.i.i = add nuw i64 %iter.sroa.0.0.i35.i3859.i.i, 1, !dbg !28382
  %_15.i.i.i = getelementptr inbounds nuw float, ptr %_54.0.i.i.i.i, i64 %base.i290.i.i, !dbg !28387
  %lanes.i1286.sroa.0.0.copyload.i.i = load <8 x float>, ptr %_15.i.i.i, align 4, !dbg !28389, !alias.scope !28394, !noalias !28398
  %608 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %suffix.i.i.sroa.0.03858.i.i, <8 x float> %lanes.i1286.sroa.0.0.copyload.i.i), !dbg !28402
  store <8 x float> %608, ptr %_15.i.i.i, align 4, !dbg !28407, !alias.scope !28413, !noalias !28417
  %609 = icmp eq i64 %end.sroa.0.0.i.i3860.i.i, 0, !dbg !28421
  %spec.store.select.i.i.i.i = select i1 %609, i64 %ring.i.i.i, i64 %end.sroa.0.0.i.i3860.i.i, !dbg !28421
  %610 = add i64 %spec.store.select.i.i.i.i, -1, !dbg !28422
  %exitcond4441.not.i.i = icmp eq i64 %_30.i36.i.i.i, %_18.i21.i.i.i, !dbg !28423
  br i1 %exitcond4441.not.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i, label %bb19.i.i.i.i, !dbg !28425

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit323.i.i
  %minimum.i.i.sroa.0.1.i.i = phi <8 x float> [ %605, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit323.i.i ], [ %minimum.i.i.sroa.0.0.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i ], !dbg !28333
  %storemerge.i.i.i.i = phi i32 [ %606, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit323.i.i ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i ], !dbg !28426
  %611 = fmul <8 x float> %minimum.i.i.sroa.0.1.i.i, splat (float 1.638400e+04), !dbg !28427
  %612 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %611), !dbg !28432
  %613 = fmul <8 x float> %612, splat (float 0x3F10000000000000), !dbg !28437
  %base.i308.i.i = shl i64 %_142.i.i.i, 3, !dbg !28442
  %614 = or disjoint i64 %base.i308.i.i, 7, !dbg !28444
  %or.cond.i312.not.i.i = icmp ult i64 %614, %_56.1.i.i.i.i, !dbg !28444
  br i1 %or.cond.i312.not.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit315.i.i, label %bb4.i314.i.i, !dbg !28444, !prof !9914

bb4.i314.i.i:                                     ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i
  store <8 x float> %history.i.i.sroa.0.0.lcssa.i.i, ptr %hot_left.i.i.i, align 1, !dbg !27522
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa.i.i, ptr %history.i.i.sroa.10.0.hot_left.i.sroa_idx.i.i, align 1, !dbg !27522
  store <8 x float> %_11.i435.sroa.0.0.copyload3889.i.i.lcssa8831234, ptr %_80.i.i.i, align 1, !dbg !27468
  store <8 x float> %_12.i434.sroa.0.0.copyload3905.i.i.lcssa8901244, ptr %432, align 1, !dbg !27478
  %_5.i309.i.i = add i64 %base.i308.i.i, 8, !dbg !28448
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i308.i.i, i64 noundef %_5.i309.i.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !28449, !noalias !28450
  unreachable, !dbg !28449

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit315.i.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i.i
  %_15.i313.i.i = getelementptr inbounds nuw float, ptr %_56.0.i.i.i.i, i64 %base.i308.i.i, !dbg !28454
  %lanes.i1265.sroa.0.0.copyload.i.i = load <8 x float>, ptr %_15.i313.i.i, align 4, !dbg !28456, !alias.scope !28461, !noalias !28465
  %615 = fadd <8 x float> %563, %613, !dbg !28469
  %616 = fsub <8 x float> %615, %lanes.i1265.sroa.0.0.copyload.i.i, !dbg !28474
  %_8.not.i4.i.i.i.i = icmp ugt i64 %_7.i10.i.i.i.i, %_56.1.i.i.i.i
  br i1 %_8.not.i4.i.i.i.i, label %bb4.i7.i.i.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i.i.i, !dbg !28479, !prof !5262

bb4.i7.i.i.i.i:                                   ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit315.i.i
  store <8 x float> %history.i.i.sroa.0.0.lcssa.i.i, ptr %hot_left.i.i.i, align 1, !dbg !27522
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa.i.i, ptr %history.i.i.sroa.10.0.hot_left.i.sroa_idx.i.i, align 1, !dbg !27522
  store <8 x float> %_11.i435.sroa.0.0.copyload3889.i.i.lcssa8831234, ptr %_80.i.i.i, align 1, !dbg !27468
  store <8 x float> %_12.i434.sroa.0.0.copyload3905.i.i.lcssa8901244, ptr %432, align 1, !dbg !27478
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i.i.i.i, i64 noundef %_7.i10.i.i.i.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cd0e502fea74c9eb9984d521d7f3533e) #31, !dbg !28484, !noalias !28485
  unreachable, !dbg !28484

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i.i.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit315.i.i
  %_17.i6.i.i.i.i = getelementptr inbounds nuw float, ptr %_56.0.i.i.i.i, i64 %base.i9.i.i.i.i, !dbg !28489
  store <8 x float> %613, ptr %_17.i6.i.i.i.i, align 4, !dbg !28491, !alias.scope !28496, !noalias !28500
  %_41.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %445, align 32, !dbg !28504, !noalias !26271
  %617 = fdiv <8 x float> %616, %_37.i.i.sroa.0.0.copyload.i.i, !dbg !28505
  %618 = fsub <8 x float> splat (float 1.000000e+00), %617, !dbg !28510
  %619 = fsub <8 x float> %618, %_41.i.i.sroa.0.0.copyload.i.i, !dbg !28515
  %620 = fmul <8 x float> %_11.i421.sroa.0.0.copyload3922.i.i, %619, !dbg !28520
  %621 = fadd <8 x float> %_41.i.i.sroa.0.0.copyload.i.i, %620, !dbg !28525
  %622 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %618, <8 x float> %621), !dbg !28529
  %623 = bitcast <8 x float> %622 to <8 x i32>, !dbg !28534
  %624 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %622), !dbg !28540
  %625 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %624, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !28542
  %626 = bitcast <8 x float> %625 to <8 x i32>, !dbg !28548
  %627 = xor <8 x i32> %626, splat (i32 -1), !dbg !28554
  %628 = and <8 x i32> %627, %623, !dbg !28556
  store <8 x i32> %628, ptr %445, align 32, !dbg !28560, !noalias !26271
  %base.i300.i.i = shl i64 %_139.i.i.i, 3, !dbg !28561
  %629 = or disjoint i64 %base.i300.i.i, 7, !dbg !28563
  %or.cond.i304.not.i.i = icmp ult i64 %629, %_58.1.i.i.i.i, !dbg !28563
  br i1 %or.cond.i304.not.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i, label %bb4.i306.i.i, !dbg !28563, !prof !9914

bb4.i306.i.i:                                     ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i.i.i
  store <8 x float> %history.i.i.sroa.0.0.lcssa.i.i, ptr %hot_left.i.i.i, align 1, !dbg !27522
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa.i.i, ptr %history.i.i.sroa.10.0.hot_left.i.sroa_idx.i.i, align 1, !dbg !27522
  store <8 x float> %_11.i435.sroa.0.0.copyload3889.i.i.lcssa8831234, ptr %_80.i.i.i, align 1, !dbg !27468
  store <8 x float> %_12.i434.sroa.0.0.copyload3905.i.i.lcssa8901244, ptr %432, align 1, !dbg !27478
  %_5.i301.i.i = add i64 %base.i300.i.i, 8, !dbg !28567
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i300.i.i, i64 noundef %_5.i301.i.i, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !28568, !noalias !28569
  unreachable, !dbg !28568

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit307.i.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i.i.i
  %630 = bitcast <8 x i32> %628 to <8 x float>, !dbg !28573
  %631 = fsub <8 x float> splat (float 1.000000e+00), %630, !dbg !28574
  %_15.i305.i.i = getelementptr inbounds nuw float, ptr %_58.0.i.i.i.i, i64 %base.i300.i.i, !dbg !28579
  %lanes.i1272.sroa.0.0.copyload.i.i = load <8 x float>, ptr %_15.i305.i.i, align 4, !dbg !28581, !alias.scope !28586, !noalias !28590
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_15.i305.i.i, ptr noundef nonnull align 4 dereferenceable(32) %data.i.i.i.i1831.i.i, i64 32, i1 false), !dbg !28594, !noalias !25924
  %632 = fmul <8 x float> %631, %lanes.i1272.sroa.0.0.copyload.i.i, !dbg !28600
  %633 = select <8 x i1> %449, <8 x float> %lanes.i1272.sroa.0.0.copyload.i.i, <8 x float> %632, !dbg !28605
  store <8 x float> %633, ptr %data.i.i.i.i1831.i.i, align 4, !dbg !28610, !alias.scope !28615, !noalias !28619
  %exitcond4452.not.i.i = icmp eq i64 %569, %562, !dbg !28122
  br i1 %exitcond4452.not.i.i, label %bb30.i.i.i, label %bb29.i.i.i, !dbg !28122

_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i.i: ; preds = %bb18.i.loopexit.i.i
  store <8 x float> %history.i.i.sroa.0.0.lcssa.i.i, ptr %hot_left.i.i.i, align 1, !dbg !27522
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa.i.i, ptr %history.i.i.sroa.10.0.hot_left.i.sroa_idx.i.i, align 1, !dbg !27522
  store <8 x float> %minimum.i.i.sroa.0.03861.lcssa3943.lcssa.i.i, ptr %uniform_left.i.i.i, align 1, !noalias !26271
  %634 = trunc i64 %main_cursor.sroa.0.1.i.lcssa.i.i to i32, !dbg !28623
  %635 = trunc i64 %ring_cursor.sroa.0.1.i.lcssa.i.i to i32, !dbg !28625
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i, !dbg !28626

_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i.i, %bb6.i.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i_crit_edge.i
  %left_phase.i.i.i = phi i32 [ %left_phase.i.i.pre.i, %bb6.i.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i_crit_edge.i ], [ %storemerge.i.i.lcssa39373962.lcssa5038.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i.i ], !dbg !27449
  %ring_cursor.sroa.0.0.i.lcssa.i.i = phi i32 [ %_29.i.i.i, %bb6.i.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i_crit_edge.i ], [ %635, %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i.i ], !dbg !27425
  %main_cursor.sroa.0.0.i.lcssa.i.i = phi i32 [ %_28.i.i.i, %bb6.i.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i_crit_edge.i ], [ %634, %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i.i ], !dbg !27421
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %left_prefix.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %uniform_left.i.i.i, i64 32, i1 false), !dbg !28626, !noalias !26271
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i.i.i), !dbg !28627, !noalias !27429
  %636 = getelementptr inbounds nuw i8, ptr %self, i64 1736, !dbg !28628
  %_154.1.i.i.i = load i64, ptr %636, align 8, !dbg !28628, !alias.scope !27354, !noalias !27355, !noundef !12
  %_8.i1549.i.i = icmp samesign ugt i64 %_154.1.i.i.i, 7, !dbg !28629
  br i1 %_8.i1549.i.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1552.i.i, label %bb2.i1550.i.i, !dbg !28629, !prof !1076

bb2.i1550.i.i:                                    ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_154.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #31, !dbg !28634, !noalias !28635
  unreachable, !dbg !28634

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1552.i.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i
  %637 = getelementptr inbounds nuw i8, ptr %self, i64 1728, !dbg !28628
  %_154.0.i.i.i = load ptr, ptr %637, align 8, !dbg !28628, !alias.scope !27354, !noalias !27355, !nonnull !12, !noundef !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_154.0.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %left_prefix.i.i.i, i64 32, i1 false), !dbg !28639, !noalias !25947
  %_155.0.i.i.i = load ptr, ptr %38, align 8, !dbg !28643, !alias.scope !27354, !noalias !27355, !nonnull !12, !noundef !12
  %_155.1.i.i.i = load i64, ptr %39, align 8, !dbg !28643, !alias.scope !27354, !noalias !27355, !noundef !12
  %638 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i.i.i), !dbg !28644
  br i1 %638, label %bb2.i1854.i.i, label %bb6.i1849.i.i, !dbg !28644

bb6.i1849.i.i:                                    ; preds = %bb2.i1854.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1552.i.i
  %end_or_len.idx.i.i.i = shl nuw nsw i64 %_155.1.i.i.i, 2, !dbg !28648
  %end_or_len.i.i.i = getelementptr inbounds nuw i8, ptr %_155.0.i.i.i, i64 %end_or_len.idx.i.i.i, !dbg !28648
  %_293.i.i.i = icmp eq i64 %_155.1.i.i.i, 0, !dbg !28652
  br i1 %_293.i.i.i, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit.i.i, label %bb10.i1850.i.i, !dbg !28655

bb2.i1854.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1552.i.i
  %bytes1.sroa.0.0.zext.i.i.i = and i32 %left_phase.i.i.i, 255, !dbg !28656
  %bytes1.sroa.0.0.isplat.i.i.i = mul nuw i32 %bytes1.sroa.0.0.zext.i.i.i, 16843009, !dbg !28656
  %_5.i1855.i.i = icmp eq i32 %left_phase.i.i.i, %bytes1.sroa.0.0.isplat.i.i.i, !dbg !28657
  br i1 %_5.i1855.i.i, label %bb3.i1856.i.i, label %bb6.i1849.i.i, !dbg !28657

bb3.i1856.i.i:                                    ; preds = %bb2.i1854.i.i
  %bytes.sroa.0.0.extract.trunc.i.i.i = trunc i32 %left_phase.i.i.i to i8, !dbg !28658
  %639 = shl nuw nsw i64 %_155.1.i.i.i, 2, !dbg !28660
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_155.0.i.i.i, i8 %bytes.sroa.0.0.extract.trunc.i.i.i, i64 %639, i1 false), !dbg !28660, !alias.scope !28661, !noalias !28127
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit.i.i, !dbg !28664

bb10.i1850.i.i:                                   ; preds = %bb6.i1849.i.i, %bb10.i1850.i.i
  %iter.sroa.0.04.i.i.i = phi ptr [ %_38.i1851.i.i, %bb10.i1850.i.i ], [ %_155.0.i.i.i, %bb6.i1849.i.i ]
  %_38.i1851.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i.i.i, i64 4, !dbg !28665
  store i32 %left_phase.i.i.i, ptr %iter.sroa.0.04.i.i.i, align 4, !dbg !28667, !alias.scope !28661, !noalias !28127
  %_29.i1852.i.i = icmp eq ptr %_38.i1851.i.i, %end_or_len.i.i.i, !dbg !28652
  br i1 %_29.i1852.i.i, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit.i.i, label %bb10.i1850.i.i, !dbg !28655

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit.i.i: ; preds = %bb10.i1850.i.i, %bb3.i1856.i.i, %bb6.i1849.i.i
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_left.i.i.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25.i) #30, !dbg !28668, !noalias !25947
  store i32 %main_cursor.sroa.0.0.i.lcssa.i.i, ptr %_24.i.i, align 4, !dbg !28623, !alias.scope !27423, !noalias !27424
  store i32 %ring_cursor.sroa.0.0.i.lcssa.i.i, ptr %387, align 4, !dbg !28625, !alias.scope !27423, !noalias !27424
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i.i.i), !dbg !28669, !noalias !27429
  br label %bb40.i.i, !dbg !27341

bb40.i.i:                                         ; preds = %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i
  br i1 %quiet.sroa.0.03256.i.i, label %bb17.i.i, label %bb23.i.i, !dbg !28670

bb17.i.i:                                         ; preds = %bb40.i.i
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_26.i.i = tail call noundef zeroext i1 @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_25.i) #30, !dbg !28671, !noalias !25947
  br i1 %_26.i.i, label %bb19.i.i, label %bb23.i.i, !dbg !28672

bb19.i.i:                                         ; preds = %bb17.i.i
  %_62.not.i.i = icmp samesign ugt i64 %words.i.i, %_39.1.i
  br i1 %_62.not.i.i, label %bb43.i.i, label %bb1.i1862.i.i, !dbg !28673, !prof !5262

bb23.i.i:                                         ; preds = %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit1878.i.i, %bb17.i.i, %bb40.i.i
  %_25.sroa.0.0.i.i = phi i8 [ %655, %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit1878.i.i ], [ 0, %bb17.i.i ], [ 0, %bb40.i.i ], !dbg !28681
  store i8 %_25.sroa.0.0.i.i, ptr %30, align 8, !dbg !28682, !alias.scope !25944, !noalias !25947
  %640 = load i8, ptr %707, align 32, !dbg !28683, !range !5399, !alias.scope !25944, !noalias !25947, !noundef !12
  store i8 %640, ptr %705, align 1, !dbg !28684, !alias.scope !25944, !noalias !25947
  %641 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !28685
  %fst_len.i1858.i.i = and i64 %_39.1.i, 2305843009213693944, !dbg !28692
  %642 = bitcast <8 x float> %641 to <8 x i32>, !dbg !28695
  %_22.not.i3988.i.i = icmp eq i64 %fst_len.i1858.i.i, 0, !dbg !28699
  br i1 %_22.not.i3988.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i.i, label %bb13.i336.i.i, !dbg !28699

bb13.i336.i.i:                                    ; preds = %bb23.i.i, %bb13.i336.i.i
  %iter.sroa.0.0.i3353991.i.i = phi ptr [ %_27.i337.i.i, %bb13.i336.i.i ], [ %_39.0.i, %bb23.i.i ]
  %iter.sroa.5.0.i3990.i.i = phi i64 [ %_28.i338.i.i, %bb13.i336.i.i ], [ %fst_len.i1858.i.i, %bb23.i.i ]
  %ok.i.sroa.0.03989.i.i = phi <8 x i32> [ %647, %bb13.i336.i.i ], [ %642, %bb23.i.i ]
  %lanes.i.sroa.0.0.copyload.i.i = load <8 x i32>, ptr %iter.sroa.0.0.i3353991.i.i, align 4, !dbg !28702, !alias.scope !28707, !noalias !28711
  %_27.i337.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i3353991.i.i, i64 32, !dbg !28715
  %_28.i338.i.i = add i64 %iter.sroa.5.0.i3990.i.i, -8, !dbg !28718
  %643 = and <8 x i32> %lanes.i.sroa.0.0.copyload.i.i, splat (i32 2147483647), !dbg !28719
  %644 = bitcast <8 x i32> %643 to <8 x float>, !dbg !28725
  %645 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %644, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !28726
  %646 = bitcast <8 x float> %645 to <8 x i32>, !dbg !28695
  %647 = and <8 x i32> %ok.i.sroa.0.03989.i.i, %646, !dbg !28732
  %_22.not.i.i.i = icmp eq i64 %_28.i338.i.i, 0, !dbg !28699
  br i1 %_22.not.i.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i.thread.i, label %bb13.i336.i.i, !dbg !28699

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i.i: ; preds = %bb23.i.i
  %648 = icmp sgt <8 x i32> %642, splat (i32 -1), !dbg !28734
  %649 = bitcast <8 x i1> %648 to i8, !dbg !28734
  %_0.i1642.not.i.i = icmp eq i8 %649, 0, !dbg !28739
  br i1 %_0.i1642.not.i.i, label %_RINvMsf_CsdvPQf9CMsz3_17true_peak_limiterINtB6_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb1_EB6_.exit, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, !dbg !28740

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i.thread.i: ; preds = %bb13.i336.i.i
  %650 = icmp sgt <8 x i32> %647, splat (i32 -1), !dbg !28734
  %651 = bitcast <8 x i1> %650 to i8, !dbg !28734
  %_0.i1642.not.i810.i = icmp eq i8 %651, 0, !dbg !28739
  br i1 %_0.i1642.not.i810.i, label %_RINvMsf_CsdvPQf9CMsz3_17true_peak_limiterINtB6_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb1_EB6_.exit, label %bb23.i6.i, !dbg !28740

bb1.i1862.i.i:                                    ; preds = %bb19.i.i, %bb10.i1877.i.i
  %iter.sroa.6.0.i1863.i.i = phi i64 [ %len.i.i.i.i1868.i.i, %bb10.i1877.i.i ], [ %words.i.i, %bb19.i.i ], !dbg !28741
  %iter.sroa.0.0.i1864.i.i = phi ptr [ %data.i.i.i.i1867.i.i, %bb10.i1877.i.i ], [ %_39.0.i, %bb19.i.i ], !dbg !28741
  %652 = icmp eq i64 %iter.sroa.6.0.i1863.i.i, 0, !dbg !28743
  br i1 %652, label %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit1878.i.i, label %bb11.preheader.i1865.i.i, !dbg !28743

bb11.preheader.i1865.i.i:                         ; preds = %bb1.i1862.i.i
  %..i.i.i1866.i.i = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i1863.i.i, i64 32), !dbg !28745
  %_18.idx.i1869.i.i = shl nuw nsw i64 %..i.i.i1866.i.i, 2, !dbg !28748
  %_18.i1870.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i1864.i.i, i64 %_18.idx.i1869.i.i, !dbg !28748
  br label %bb11.i1871.i.i, !dbg !28753

bb11.i1871.i.i:                                   ; preds = %bb11.i1871.i.i, %bb11.preheader.i1865.i.i
  %iter1.sroa.0.014.i1872.i.i = phi ptr [ %_31.i1874.i.i, %bb11.i1871.i.i ], [ %iter.sroa.0.0.i1864.i.i, %bb11.preheader.i1865.i.i ]
  %bits.sroa.0.013.i1873.i.i = phi i32 [ %653, %bb11.i1871.i.i ], [ 0, %bb11.preheader.i1865.i.i ]
  %_31.i1874.i.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i1872.i.i, i64 4, !dbg !28755
  %_134.i1875.i.i = load i32, ptr %iter1.sroa.0.014.i1872.i.i, align 4, !dbg !28757, !alias.scope !28758, !noalias !26029, !noundef !12
  %653 = or i32 %_134.i1875.i.i, %bits.sroa.0.013.i1873.i.i, !dbg !28761
  %_25.i1876.i.i = icmp eq ptr %_31.i1874.i.i, %_18.i1870.i.i, !dbg !28762
  br i1 %_25.i1876.i.i, label %bb10.i1877.i.i, label %bb11.i1871.i.i, !dbg !28753

bb10.i1877.i.i:                                   ; preds = %bb11.i1871.i.i
  %data.i.i.i.i1867.i.i = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i1864.i.i, i64 %..i.i.i1866.i.i, !dbg !28764
  %len.i.i.i.i1868.i.i = sub nuw nsw i64 %iter.sroa.6.0.i1863.i.i, %..i.i.i1866.i.i, !dbg !28769
  %654 = icmp eq i32 %653, 0, !dbg !28770
  br i1 %654, label %bb1.i1862.i.i, label %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit1878.i.i, !dbg !28770

_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit1878.i.i: ; preds = %bb10.i1877.i.i, %bb1.i1862.i.i
  %655 = zext i1 %652 to i8, !dbg !28682
  br label %bb23.i.i, !dbg !28670

bb43.i.i:                                         ; preds = %bb19.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %words.i.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6cbd2a842cd3fc5676a9a4fde31f14f8) #31, !dbg !28771, !noalias !25947
  unreachable, !dbg !28771

bb23.i6.i:                                        ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i.thread.i, %bb23.i6.i
  %iter.sroa.0.074.i.i = phi ptr [ %_45.i.i, %bb23.i6.i ], [ %_39.0.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i.thread.i ]
  %iter.sroa.5.073.i.i = phi i64 [ %_46.i.i, %bb23.i6.i ], [ %fst_len.i1858.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i.thread.i ]
  %ok.sroa.0.072.i.i = phi <8 x i32> [ %660, %bb23.i6.i ], [ %642, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i.thread.i ]
  %lanes.i.sroa.0.0.copyload.i7.i = load <8 x i32>, ptr %iter.sroa.0.074.i.i, align 4, !dbg !28772, !alias.scope !28778, !noalias !28784
  %_45.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.074.i.i, i64 32, !dbg !28788
  %_46.i.i = add i64 %iter.sroa.5.073.i.i, -8, !dbg !28793
  %656 = and <8 x i32> %lanes.i.sroa.0.0.copyload.i7.i, splat (i32 2147483647), !dbg !28794
  %657 = bitcast <8 x i32> %656 to <8 x float>, !dbg !28800
  %658 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %657, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !28801
  %659 = bitcast <8 x float> %658 to <8 x i32>, !dbg !28807
  %660 = and <8 x i32> %ok.sroa.0.072.i.i, %659, !dbg !28811
  %_40.not.i.i = icmp eq i64 %_46.i.i, 0, !dbg !28813
  br i1 %_40.not.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %bb23.i6.i, !dbg !28813

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i: ; preds = %bb23.i6.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i.i
  %ok.sroa.0.0.lcssa.i.i = phi <8 x i32> [ %642, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i.i ], [ %660, %bb23.i6.i ], !dbg !28814
  %661 = icmp slt <8 x i32> %ok.sroa.0.0.lcssa.i.i, zeroinitializer, !dbg !28815
  %bc.i.i = select <8 x i1> %661, <8 x i32> zeroinitializer, <8 x i32> splat (i32 1065353216), !dbg !28815
  %662 = extractelement <8 x i32> %bc.i.i, i64 0, !dbg !28820
  %663 = icmp ne i32 %662, 0, !dbg !28820
  %664 = zext i1 %663 to i32, !dbg !28820
  %665 = extractelement <8 x i32> %bc.i.i, i64 1, !dbg !28820
  %666 = icmp eq i32 %665, 0, !dbg !28820
  %667 = select i1 %666, i32 0, i32 2, !dbg !28820
  %mask.sroa.0.1.1.i.i = or disjoint i32 %667, %664, !dbg !28820
  %668 = extractelement <8 x i32> %bc.i.i, i64 2, !dbg !28820
  %669 = icmp eq i32 %668, 0, !dbg !28820
  %670 = select i1 %669, i32 0, i32 4, !dbg !28820
  %mask.sroa.0.1.2.i.i = or disjoint i32 %mask.sroa.0.1.1.i.i, %670, !dbg !28820
  %671 = extractelement <8 x i32> %bc.i.i, i64 3, !dbg !28820
  %672 = icmp eq i32 %671, 0, !dbg !28820
  %673 = select i1 %672, i32 0, i32 8, !dbg !28820
  %mask.sroa.0.1.3.i.i = or disjoint i32 %mask.sroa.0.1.2.i.i, %673, !dbg !28820
  %674 = extractelement <8 x i32> %bc.i.i, i64 4, !dbg !28820
  %675 = icmp eq i32 %674, 0, !dbg !28820
  %676 = select i1 %675, i32 0, i32 16, !dbg !28820
  %mask.sroa.0.1.4.i.i = or disjoint i32 %mask.sroa.0.1.3.i.i, %676, !dbg !28820
  %677 = extractelement <8 x i32> %bc.i.i, i64 5, !dbg !28820
  %678 = icmp eq i32 %677, 0, !dbg !28820
  %679 = select i1 %678, i32 0, i32 32, !dbg !28820
  %mask.sroa.0.1.5.i.i = or disjoint i32 %mask.sroa.0.1.4.i.i, %679, !dbg !28820
  %680 = extractelement <8 x i32> %bc.i.i, i64 6, !dbg !28820
  %681 = icmp eq i32 %680, 0, !dbg !28820
  %682 = select i1 %681, i32 0, i32 64, !dbg !28820
  %mask.sroa.0.1.6.i.i = or i32 %mask.sroa.0.1.5.i.i, %682, !dbg !28820
  %683 = extractelement <8 x i32> %bc.i.i, i64 7, !dbg !28820
  %684 = icmp eq i32 %683, 0, !dbg !28820
  %685 = select i1 %684, i32 0, i32 128, !dbg !28820
  %mask.sroa.0.1.7.i.i = or i32 %mask.sroa.0.1.6.i.i, %685, !dbg !28820
  %686 = getelementptr inbounds nuw i8, ptr %self, i64 1568, !dbg !28821
  %687 = getelementptr inbounds nuw i8, ptr %self, i64 1576, !dbg !28821
  store i32 %mask.sroa.0.1.7.i.i, ptr %687, align 8, !dbg !28821, !alias.scope !25944, !noalias !25947
  %_35.i.i = load i64, ptr %686, align 32, !dbg !28822, !alias.scope !25944, !noalias !25947, !noundef !12
  %688 = tail call i64 @llvm.uadd.sat.i64(i64 %_35.i.i, i64 1), !dbg !28823
  store i64 %688, ptr %686, align 32, !dbg !28826, !alias.scope !25944, !noalias !25947
  %_222.i.i.i = icmp eq i64 %_39.1.i, 0, !dbg !28827
  br i1 %_222.i.i.i, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i.i, label %bb14.i.preheader.i.i, !dbg !28833

bb14.i.preheader.i.i:                             ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i
  %.idx.i.i.i = shl nuw nsw i64 %_39.1.i, 2, !dbg !28834
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_39.0.i, i8 0, i64 %.idx.i.i.i, i1 false), !dbg !28838, !alias.scope !28839, !noalias !26029
  br label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i.i, !dbg !28842

_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i.i: ; preds = %bb14.i.preheader.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i
  call void @llvm.lifetime.start.p0(ptr nonnull %shape.i.i), !dbg !28842, !noalias !26271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %shape.i.i, ptr noundef nonnull align 16 dereferenceable(24) %_22.i.i, i64 24, i1 false), !dbg !28843, !noalias !25947
  %689 = getelementptr inbounds nuw i8, ptr %self, i64 2120, !dbg !28844
  %rate.i.i = load i32, ptr %689, align 8, !dbg !28844, !alias.scope !25944, !noalias !25947, !noundef !12
  %690 = getelementptr inbounds nuw i8, ptr %self, i64 1584, !dbg !28846
  %_73.0.i.i = load ptr, ptr %690, align 16, !dbg !28846, !alias.scope !25944, !noalias !25947, !nonnull !12, !noundef !12
  %691 = getelementptr inbounds nuw i8, ptr %self, i64 1592, !dbg !28846
  %_73.1.i.i = load i64, ptr %691, align 8, !dbg !28846, !alias.scope !25944, !noalias !25947, !noundef !12
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 8 dereferenceable(200) %_25.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %shape.i.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_73.0.i.i, i64 noundef %_73.1.i.i, i32 noundef %rate.i.i) #30, !dbg !28848, !noalias !25947
  %692 = getelementptr inbounds nuw i8, ptr %self, i64 1600, !dbg !28849
  %_74.0.i.i = load ptr, ptr %692, align 32, !dbg !28849, !alias.scope !25944, !noalias !25947, !nonnull !12, !noundef !12
  %693 = getelementptr inbounds nuw i8, ptr %self, i64 1608, !dbg !28849
  %_74.1.i.i = load i64, ptr %693, align 8, !dbg !28849, !alias.scope !25944, !noalias !25947, !noundef !12
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 8 dereferenceable(200) %_26.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %shape.i.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_74.0.i.i, i64 noundef %_74.1.i.i, i32 noundef %rate.i.i) #30, !dbg !28850, !noalias !25947
  store i32 0, ptr %_24.i.i, align 8, !dbg !28851, !alias.scope !25944, !noalias !25947
  %694 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !28851
  store i32 0, ptr %694, align 4, !dbg !28851, !alias.scope !25944, !noalias !25947
  call void @llvm.lifetime.end.p0(ptr nonnull %shape.i.i), !dbg !28852, !noalias !26271
  br label %_RINvMsf_CsdvPQf9CMsz3_17true_peak_limiterINtB6_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb1_EB6_.exit, !dbg !28853

bb4.i:                                            ; preds = %bb2.i
  %_14.i = load i32, ptr %_38.0.i, align 4, !dbg !25936, !noalias !25924, !noundef !12
  %start1.i = zext i32 %_14.i to i64, !dbg !25936
  %exitcond797.not.i = icmp eq i64 %_38.1.i, 1, !dbg !28854
  br i1 %exitcond797.not.i, label %panic2.i, label %bb5.i, !dbg !28854

panic.i:                                          ; preds = %bb6.6.i, %bb6.5.i, %bb6.4.i, %bb6.3.i, %bb6.2.i, %bb6.1.i, %bb2.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_38.1.i, i64 noundef %_38.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2864b855a71e07cd82bfe5e37ca0325a) #31, !dbg !25936, !noalias !25924
  unreachable, !dbg !25936

bb5.i:                                            ; preds = %bb4.i
  %695 = getelementptr inbounds nuw i8, ptr %_38.0.i, i64 4, !dbg !28854
  %_18.i = load i32, ptr %695, align 4, !dbg !28854, !noalias !25924, !noundef !12
  %end.i = zext i32 %_18.i to i64, !dbg !28854
  %_53.i = icmp ult i32 %_18.i, %_14.i, !dbg !28856
  %_49.not.i = icmp ult i64 %_37.1.i, %end.i
  %or.cond.i = or i1 %_53.i, %_49.not.i, !dbg !28856
  br i1 %or.cond.i, label %bb16.i, label %bb4.1.i, !dbg !28856, !prof !5262

panic2.i:                                         ; preds = %bb4.7.i, %bb4.6.i, %bb4.5.i, %bb4.4.i, %bb4.3.i, %bb4.2.i, %bb4.1.i, %bb4.i
  %.lcssa788.i = phi i64 [ 1, %bb4.i ], [ 2, %bb4.1.i ], [ 3, %bb4.2.i ], [ 4, %bb4.3.i ], [ 5, %bb4.4.i ], [ 6, %bb4.5.i ], [ 7, %bb4.6.i ], [ 8, %bb4.7.i ], !dbg !28864
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.lcssa788.i, i64 noundef %_38.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8f78f3c6b9c9252b16ab7d7ccc563a9d) #31, !dbg !28854, !noalias !25924
  unreachable, !dbg !28854

bb16.i:                                           ; preds = %bb5.7.i, %bb5.6.i, %bb5.5.i, %bb5.4.i, %bb5.3.i, %bb5.2.i, %bb5.1.i, %bb5.i
  %end.lcssa.i = phi i64 [ %end.i, %bb5.i ], [ %end.1.i, %bb5.1.i ], [ %end.2.i, %bb5.2.i ], [ %end.3.i, %bb5.3.i ], [ %end.4.i, %bb5.4.i ], [ %end.5.i, %bb5.5.i ], [ %end.6.i, %bb5.6.i ], [ %end.7.i, %bb5.7.i ], !dbg !28854
  %start1.lcssa794.i = phi i64 [ %start1.i, %bb5.i ], [ %start1.1.i, %bb5.1.i ], [ %start1.2.i, %bb5.2.i ], [ %start1.3.i, %bb5.3.i ], [ %start1.4.i, %bb5.4.i ], [ %start1.5.i, %bb5.5.i ], [ %start1.6.i, %bb5.6.i ], [ %start1.7.i, %bb5.7.i ], !dbg !25936
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %start1.lcssa794.i, i64 noundef %end.lcssa.i, i64 noundef %_37.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4ac05e629a5e1e4066705fcc17f0e09d) #31, !dbg !28870, !noalias !25924
  unreachable, !dbg !28870

bb4.1.i:                                          ; preds = %bb5.i
  %_54.i = sub nuw nsw i64 %end.i, %start1.i, !dbg !28871
  %_56.i = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0.i, i64 %start1.i, !dbg !28872
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.i, i64 noundef %_54.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23.i, i64 noundef %_24.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26.i, i64 noundef 0, ptr noalias noundef nonnull align 8 dereferenceable(40) %report.i) #30, !dbg !28876, !noalias !25924
  %_14.1.i = load i32, ptr %695, align 4, !dbg !25936, !noalias !25924, !noundef !12
  %start1.1.i = zext i32 %_14.1.i to i64, !dbg !25936
  %exitcond797.1.not.i = icmp eq i64 %9, 1, !dbg !28854
  br i1 %exitcond797.1.not.i, label %panic2.i, label %bb5.1.i, !dbg !28854

bb5.1.i:                                          ; preds = %bb4.1.i
  %696 = getelementptr inbounds nuw i8, ptr %_38.0.i, i64 8, !dbg !28854
  %_18.1.i = load i32, ptr %696, align 4, !dbg !28854, !noalias !25924, !noundef !12
  %end.1.i = zext i32 %_18.1.i to i64, !dbg !28854
  %_53.1.i = icmp ult i32 %_18.1.i, %_14.1.i, !dbg !28856
  %_49.not.1.i = icmp ult i64 %_37.1.i, %end.1.i
  %or.cond.1.i = or i1 %_53.1.i, %_49.not.1.i, !dbg !28856
  br i1 %or.cond.1.i, label %bb16.i, label %bb6.1.i, !dbg !28856, !prof !5262

bb6.1.i:                                          ; preds = %bb5.1.i
  %_54.1.i = sub nuw nsw i64 %end.1.i, %start1.1.i, !dbg !28871
  %_56.1.i = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0.i, i64 %start1.1.i, !dbg !28872
  %_27.1.i = getelementptr inbounds nuw i8, ptr %report.i, i64 40, !dbg !28877
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.1.i, i64 noundef %_54.1.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23.i, i64 noundef %_24.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26.i, i64 noundef 1, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.1.i) #30, !dbg !28876, !noalias !25924
  %exitcond.2.not.i = icmp eq i64 %_38.1.i, 2, !dbg !25936
  br i1 %exitcond.2.not.i, label %panic.i, label %bb4.2.i, !dbg !25936

bb4.2.i:                                          ; preds = %bb6.1.i
  %_14.2.i = load i32, ptr %696, align 4, !dbg !25936, !noalias !25924, !noundef !12
  %start1.2.i = zext i32 %_14.2.i to i64, !dbg !25936
  %exitcond797.2.not.i = icmp eq i64 %9, 2, !dbg !28854
  br i1 %exitcond797.2.not.i, label %panic2.i, label %bb5.2.i, !dbg !28854

bb5.2.i:                                          ; preds = %bb4.2.i
  %697 = getelementptr inbounds nuw i8, ptr %_38.0.i, i64 12, !dbg !28854
  %_18.2.i = load i32, ptr %697, align 4, !dbg !28854, !noalias !25924, !noundef !12
  %end.2.i = zext i32 %_18.2.i to i64, !dbg !28854
  %_53.2.i = icmp ult i32 %_18.2.i, %_14.2.i, !dbg !28856
  %_49.not.2.i = icmp ult i64 %_37.1.i, %end.2.i
  %or.cond.2.i = or i1 %_53.2.i, %_49.not.2.i, !dbg !28856
  br i1 %or.cond.2.i, label %bb16.i, label %bb6.2.i, !dbg !28856, !prof !5262

bb6.2.i:                                          ; preds = %bb5.2.i
  %_54.2.i = sub nuw nsw i64 %end.2.i, %start1.2.i, !dbg !28871
  %_56.2.i = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0.i, i64 %start1.2.i, !dbg !28872
  %_27.2.i = getelementptr inbounds nuw i8, ptr %report.i, i64 80, !dbg !28877
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.2.i, i64 noundef %_54.2.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23.i, i64 noundef %_24.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26.i, i64 noundef 2, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.2.i) #30, !dbg !28876, !noalias !25924
  %exitcond.3.not.i = icmp eq i64 %_38.1.i, 3, !dbg !25936
  br i1 %exitcond.3.not.i, label %panic.i, label %bb4.3.i, !dbg !25936

bb4.3.i:                                          ; preds = %bb6.2.i
  %_14.3.i = load i32, ptr %697, align 4, !dbg !25936, !noalias !25924, !noundef !12
  %start1.3.i = zext i32 %_14.3.i to i64, !dbg !25936
  %exitcond797.3.not.i = icmp eq i64 %9, 3, !dbg !28854
  br i1 %exitcond797.3.not.i, label %panic2.i, label %bb5.3.i, !dbg !28854

bb5.3.i:                                          ; preds = %bb4.3.i
  %698 = getelementptr inbounds nuw i8, ptr %_38.0.i, i64 16, !dbg !28854
  %_18.3.i = load i32, ptr %698, align 4, !dbg !28854, !noalias !25924, !noundef !12
  %end.3.i = zext i32 %_18.3.i to i64, !dbg !28854
  %_53.3.i = icmp ult i32 %_18.3.i, %_14.3.i, !dbg !28856
  %_49.not.3.i = icmp ult i64 %_37.1.i, %end.3.i
  %or.cond.3.i = or i1 %_53.3.i, %_49.not.3.i, !dbg !28856
  br i1 %or.cond.3.i, label %bb16.i, label %bb6.3.i, !dbg !28856, !prof !5262

bb6.3.i:                                          ; preds = %bb5.3.i
  %_54.3.i = sub nuw nsw i64 %end.3.i, %start1.3.i, !dbg !28871
  %_56.3.i = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0.i, i64 %start1.3.i, !dbg !28872
  %_27.3.i = getelementptr inbounds nuw i8, ptr %report.i, i64 120, !dbg !28877
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.3.i, i64 noundef %_54.3.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23.i, i64 noundef %_24.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26.i, i64 noundef 3, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.3.i) #30, !dbg !28876, !noalias !25924
  %exitcond.4.not.i = icmp eq i64 %_38.1.i, 4, !dbg !25936
  br i1 %exitcond.4.not.i, label %panic.i, label %bb4.4.i, !dbg !25936

bb4.4.i:                                          ; preds = %bb6.3.i
  %_14.4.i = load i32, ptr %698, align 4, !dbg !25936, !noalias !25924, !noundef !12
  %start1.4.i = zext i32 %_14.4.i to i64, !dbg !25936
  %exitcond797.4.not.i = icmp eq i64 %9, 4, !dbg !28854
  br i1 %exitcond797.4.not.i, label %panic2.i, label %bb5.4.i, !dbg !28854

bb5.4.i:                                          ; preds = %bb4.4.i
  %699 = getelementptr inbounds nuw i8, ptr %_38.0.i, i64 20, !dbg !28854
  %_18.4.i = load i32, ptr %699, align 4, !dbg !28854, !noalias !25924, !noundef !12
  %end.4.i = zext i32 %_18.4.i to i64, !dbg !28854
  %_53.4.i = icmp ult i32 %_18.4.i, %_14.4.i, !dbg !28856
  %_49.not.4.i = icmp ult i64 %_37.1.i, %end.4.i
  %or.cond.4.i = or i1 %_53.4.i, %_49.not.4.i, !dbg !28856
  br i1 %or.cond.4.i, label %bb16.i, label %bb6.4.i, !dbg !28856, !prof !5262

bb6.4.i:                                          ; preds = %bb5.4.i
  %_54.4.i = sub nuw nsw i64 %end.4.i, %start1.4.i, !dbg !28871
  %_56.4.i = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0.i, i64 %start1.4.i, !dbg !28872
  %_27.4.i = getelementptr inbounds nuw i8, ptr %report.i, i64 160, !dbg !28877
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.4.i, i64 noundef %_54.4.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23.i, i64 noundef %_24.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26.i, i64 noundef 4, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.4.i) #30, !dbg !28876, !noalias !25924
  %exitcond.5.not.i = icmp eq i64 %_38.1.i, 5, !dbg !25936
  br i1 %exitcond.5.not.i, label %panic.i, label %bb4.5.i, !dbg !25936

bb4.5.i:                                          ; preds = %bb6.4.i
  %_14.5.i = load i32, ptr %699, align 4, !dbg !25936, !noalias !25924, !noundef !12
  %start1.5.i = zext i32 %_14.5.i to i64, !dbg !25936
  %exitcond797.5.not.i = icmp eq i64 %9, 5, !dbg !28854
  br i1 %exitcond797.5.not.i, label %panic2.i, label %bb5.5.i, !dbg !28854

bb5.5.i:                                          ; preds = %bb4.5.i
  %700 = getelementptr inbounds nuw i8, ptr %_38.0.i, i64 24, !dbg !28854
  %_18.5.i = load i32, ptr %700, align 4, !dbg !28854, !noalias !25924, !noundef !12
  %end.5.i = zext i32 %_18.5.i to i64, !dbg !28854
  %_53.5.i = icmp ult i32 %_18.5.i, %_14.5.i, !dbg !28856
  %_49.not.5.i = icmp ult i64 %_37.1.i, %end.5.i
  %or.cond.5.i = or i1 %_53.5.i, %_49.not.5.i, !dbg !28856
  br i1 %or.cond.5.i, label %bb16.i, label %bb6.5.i, !dbg !28856, !prof !5262

bb6.5.i:                                          ; preds = %bb5.5.i
  %_54.5.i = sub nuw nsw i64 %end.5.i, %start1.5.i, !dbg !28871
  %_56.5.i = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0.i, i64 %start1.5.i, !dbg !28872
  %_27.5.i = getelementptr inbounds nuw i8, ptr %report.i, i64 200, !dbg !28877
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.5.i, i64 noundef %_54.5.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23.i, i64 noundef %_24.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26.i, i64 noundef 5, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.5.i) #30, !dbg !28876, !noalias !25924
  %exitcond.6.not.i = icmp eq i64 %_38.1.i, 6, !dbg !25936
  br i1 %exitcond.6.not.i, label %panic.i, label %bb4.6.i, !dbg !25936

bb4.6.i:                                          ; preds = %bb6.5.i
  %_14.6.i = load i32, ptr %700, align 4, !dbg !25936, !noalias !25924, !noundef !12
  %start1.6.i = zext i32 %_14.6.i to i64, !dbg !25936
  %exitcond797.6.not.i = icmp eq i64 %9, 6, !dbg !28854
  br i1 %exitcond797.6.not.i, label %panic2.i, label %bb5.6.i, !dbg !28854

bb5.6.i:                                          ; preds = %bb4.6.i
  %701 = getelementptr inbounds nuw i8, ptr %_38.0.i, i64 28, !dbg !28854
  %_18.6.i = load i32, ptr %701, align 4, !dbg !28854, !noalias !25924, !noundef !12
  %end.6.i = zext i32 %_18.6.i to i64, !dbg !28854
  %_53.6.i = icmp ult i32 %_18.6.i, %_14.6.i, !dbg !28856
  %_49.not.6.i = icmp ult i64 %_37.1.i, %end.6.i
  %or.cond.6.i = or i1 %_53.6.i, %_49.not.6.i, !dbg !28856
  br i1 %or.cond.6.i, label %bb16.i, label %bb6.6.i, !dbg !28856, !prof !5262

bb6.6.i:                                          ; preds = %bb5.6.i
  %_54.6.i = sub nuw nsw i64 %end.6.i, %start1.6.i, !dbg !28871
  %_56.6.i = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0.i, i64 %start1.6.i, !dbg !28872
  %_27.6.i = getelementptr inbounds nuw i8, ptr %report.i, i64 240, !dbg !28877
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.6.i, i64 noundef %_54.6.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23.i, i64 noundef %_24.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26.i, i64 noundef 6, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.6.i) #30, !dbg !28876, !noalias !25924
  %exitcond.7.not.i = icmp eq i64 %_38.1.i, 7, !dbg !25936
  br i1 %exitcond.7.not.i, label %panic.i, label %bb4.7.i, !dbg !25936

bb4.7.i:                                          ; preds = %bb6.6.i
  %_14.7.i = load i32, ptr %701, align 4, !dbg !25936, !noalias !25924, !noundef !12
  %start1.7.i = zext i32 %_14.7.i to i64, !dbg !25936
  %exitcond797.7.not.i = icmp eq i64 %9, 7, !dbg !28854
  br i1 %exitcond797.7.not.i, label %panic2.i, label %bb5.7.i, !dbg !28854

bb5.7.i:                                          ; preds = %bb4.7.i
  %702 = getelementptr inbounds nuw i8, ptr %_38.0.i, i64 32, !dbg !28854
  %_18.7.i = load i32, ptr %702, align 4, !dbg !28854, !noalias !25924, !noundef !12
  %end.7.i = zext i32 %_18.7.i to i64, !dbg !28854
  %_53.7.i = icmp ult i32 %_18.7.i, %_14.7.i, !dbg !28856
  %_49.not.7.i = icmp ult i64 %_37.1.i, %end.7.i
  %or.cond.7.i = or i1 %_53.7.i, %_49.not.7.i, !dbg !28856
  br i1 %or.cond.7.i, label %bb16.i, label %bb6.7.i, !dbg !28856, !prof !5262

bb6.7.i:                                          ; preds = %bb5.7.i
  %_54.7.i = sub nuw nsw i64 %end.7.i, %start1.7.i, !dbg !28871
  %_56.7.i = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0.i, i64 %start1.7.i, !dbg !28872
  %_27.7.i = getelementptr inbounds nuw i8, ptr %report.i, i64 280, !dbg !28877
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.7.i, i64 noundef %_54.7.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23.i, i64 noundef %_24.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26.i, i64 noundef 7, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.7.i) #30, !dbg !28876, !noalias !25924
  %_39.0.i = load ptr, ptr %block, align 8, !dbg !28878, !alias.scope !25914, !noalias !25919, !nonnull !12, !align !9542, !noundef !12
  %703 = getelementptr inbounds nuw i8, ptr %block, i64 8, !dbg !28878
  %_39.1.i = load i64, ptr %703, align 8, !dbg !28878, !alias.scope !25914, !noalias !25919, !noundef !12
  %704 = getelementptr inbounds nuw i8, ptr %block, i64 104, !dbg !28879
  %_32.i = load i32, ptr %704, align 8, !dbg !28879, !alias.scope !25914, !noalias !25919, !noundef !12
  %_31.i = zext i32 %_32.i to i64, !dbg !28879
  tail call void @llvm.experimental.noalias.scope.decl(metadata !28880), !dbg !28881
  tail call void @llvm.experimental.noalias.scope.decl(metadata !28882), !dbg !28881
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i216.i.i), !dbg !28883, !noalias !25922
  call void @llvm.lifetime.start.p0(ptr nonnull %left_prefix.i.i.i), !dbg !28883
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i.i.i), !dbg !28883, !noalias !25922
  %words.i.i = shl nuw nsw i64 %_31.i, 3, !dbg !28883
  %705 = getelementptr inbounds nuw i8, ptr %self, i64 2153, !dbg !28884
  %706 = load i8, ptr %705, align 1, !dbg !28884, !range !5399, !alias.scope !25944, !noalias !25947, !noundef !12
  %707 = getelementptr inbounds nuw i8, ptr %self, i64 2144, !dbg !28885
  %708 = load i8, ptr %707, align 32, !dbg !28885, !range !5399, !alias.scope !25944, !noalias !25947, !noundef !12
  %_6.i.i = icmp eq i8 %706, %708, !dbg !28884
  br i1 %_6.i.i, label %bb1.i.i, label %bb11.thread.i.i, !dbg !28884

_RINvMsf_CsdvPQf9CMsz3_17true_peak_limiterINtB6_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb1_EB6_.exit: ; preds = %_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i.thread.i, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i.i
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i216.i.i), !dbg !28853, !noalias !25922
  call void @llvm.lifetime.end.p0(ptr nonnull %left_prefix.i.i.i), !dbg !28853
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i.i.i), !dbg !28853, !noalias !25922
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(328) %_0, ptr noundef nonnull align 8 dereferenceable(328) %report.i, i64 328, i1 false), !dbg !28886, !noalias !28887
  call void @llvm.lifetime.end.p0(ptr nonnull %report.i), !dbg !28888, !noalias !25922
  ret void, !dbg !28889
}
