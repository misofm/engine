define internal fastcc void @_RNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB5_11LimiterCoreNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E13process_blockB5_(ptr noalias noundef nonnull align 16 dereferenceable(1136) %self, ptr noalias noundef nonnull align 4 captures(address) %left_io.0, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef nonnull align 4 captures(address) %right_io.0, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef %frames) unnamed_addr #1 !dbg !4760 {
start:
  %peaks_right.i756 = alloca [1024 x i8], align 4
  %peaks_left.i757 = alloca [1024 x i8], align 4
  %scratch.i758 = alloca [32 x i8], align 4
  %hot_right.i760 = alloca [368 x i8], align 16
  %hot_left.i761 = alloca [368 x i8], align 16
  %peaks_right.i468 = alloca [1024 x i8], align 4
  %peaks_left.i469 = alloca [1024 x i8], align 4
  %scratch.i = alloca [32 x i8], align 4
  %hot_right.i471 = alloca [368 x i8], align 16
  %hot_left.i472 = alloca [368 x i8], align 16
  %uniform_right.i38 = alloca [64 x i8], align 16
  %uniform_left.i39 = alloca [64 x i8], align 16
  %peaks_right.i40 = alloca [1024 x i8], align 4
  %peaks_left.i41 = alloca [1024 x i8], align 4
  %hot_right.i43 = alloca [368 x i8], align 16
  %hot_left.i44 = alloca [368 x i8], align 16
  %uniform_right.i = alloca [64 x i8], align 16
  %uniform_left.i = alloca [64 x i8], align 16
  %peaks_right.i = alloca [1024 x i8], align 4
  %peaks_left.i = alloca [1024 x i8], align 4
  %hot_right.i = alloca [368 x i8], align 16
  %hot_left.i = alloca [368 x i8], align 16
  %shape = alloca [12 x i8], align 4
  %words = shl i32 %frames, 2, !dbg !4762
  %0 = getelementptr inbounds nuw i8, ptr %self, i32 1125, !dbg !4763
  %1 = load i8, ptr %0, align 1, !dbg !4763, !range !4765, !noundef !10
  %2 = getelementptr inbounds nuw i8, ptr %self, i32 904, !dbg !4766
  %3 = load i8, ptr %2, align 8, !dbg !4766, !range !4765, !noundef !10
  %_7 = icmp eq i8 %1, %3, !dbg !4763
  %4 = getelementptr inbounds nuw i8, ptr %self, i32 988
  %_95.0 = load ptr, ptr %4, align 4, !dbg !4767
  %5 = getelementptr inbounds nuw i8, ptr %self, i32 992
  %_95.1 = load i32, ptr %5, align 4, !dbg !4767
  br i1 %_7, label %bb1, label %bb20.thread, !dbg !4763

bb1:                                              ; preds = %start
  %_8.i6281 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_95.0, i32 %_95.1, !dbg !4768
  br label %bb1.i.i6282, !dbg !4773

bb1.i.i6282:                                      ; preds = %bb11.i.i, %bb1
  %_221.i.i = phi ptr [ %_22.i.i6283, %bb11.i.i ], [ %_95.0, %bb1 ]
  %_12.i.i = icmp eq ptr %_221.i.i, %_8.i6281, !dbg !4775
  br i1 %_12.i.i, label %bb3, label %bb11.i.i, !dbg !4778

bb11.i.i:                                         ; preds = %bb1.i.i6282
  %_22.i.i6283 = getelementptr inbounds nuw i8, ptr %_221.i.i, i32 16, !dbg !4779
  %6 = getelementptr inbounds nuw i8, ptr %_221.i.i, i32 12, !dbg !4781
  %_3.i.i.i = load i32, ptr %6, align 4, !dbg !4781, !alias.scope !4783, !noalias !4788, !noundef !10
  %7 = icmp eq i32 %_3.i.i.i, 0, !dbg !4781
  %_51.i.i.i = load i32, ptr %_221.i.i, align 4, !dbg !4781, !alias.scope !4783, !noalias !4788
  %8 = getelementptr inbounds nuw i8, ptr %_221.i.i, i32 4, !dbg !4781
  %_72.i.i.i = load i32, ptr %8, align 4, !dbg !4781, !alias.scope !4783, !noalias !4788
  %9 = icmp eq i32 %_51.i.i.i, %_72.i.i.i, !dbg !4781
  %_0.sroa.0.0.off0.i.i.i = select i1 %7, i1 %9, i1 false, !dbg !4781
  br i1 %_0.sroa.0.0.off0.i.i.i, label %bb1.i.i6282, label %bb20.thread, !dbg !4791

bb3:                                              ; preds = %bb1.i.i6282
  %10 = getelementptr inbounds nuw i8, ptr %self, i32 996, !dbg !4792
  %_96.0 = load ptr, ptr %10, align 4, !dbg !4792, !nonnull !10, !noundef !10
  %11 = getelementptr inbounds nuw i8, ptr %self, i32 1000, !dbg !4792
  %_96.1 = load i32, ptr %11, align 4, !dbg !4792, !noundef !10
  %_8.i6284 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_96.0, i32 %_96.1, !dbg !4793
  br label %bb1.i.i6285, !dbg !4798

bb1.i.i6285:                                      ; preds = %bb11.i.i6288, %bb3
  %_221.i.i6286 = phi ptr [ %_22.i.i6289, %bb11.i.i6288 ], [ %_96.0, %bb3 ]
  %_12.i.i6287 = icmp eq ptr %_221.i.i6286, %_8.i6284, !dbg !4800
  br i1 %_12.i.i6287, label %bb5, label %bb11.i.i6288, !dbg !4803

bb11.i.i6288:                                     ; preds = %bb1.i.i6285
  %_22.i.i6289 = getelementptr inbounds nuw i8, ptr %_221.i.i6286, i32 16, !dbg !4804
  %12 = getelementptr inbounds nuw i8, ptr %_221.i.i6286, i32 12, !dbg !4806
  %_3.i.i.i6290 = load i32, ptr %12, align 4, !dbg !4806, !alias.scope !4808, !noalias !4813, !noundef !10
  %13 = icmp eq i32 %_3.i.i.i6290, 0, !dbg !4806
  %_51.i.i.i6291 = load i32, ptr %_221.i.i6286, align 4, !dbg !4806, !alias.scope !4808, !noalias !4813
  %14 = getelementptr inbounds nuw i8, ptr %_221.i.i6286, i32 4, !dbg !4806
  %_72.i.i.i6292 = load i32, ptr %14, align 4, !dbg !4806, !alias.scope !4808, !noalias !4813
  %15 = icmp eq i32 %_51.i.i.i6291, %_72.i.i.i6292, !dbg !4806
  %_0.sroa.0.0.off0.i.i.i6293 = select i1 %13, i1 %15, i1 false, !dbg !4806
  br i1 %_0.sroa.0.0.off0.i.i.i6293, label %bb1.i.i6285, label %bb20.thread, !dbg !4816

bb5:                                              ; preds = %bb1.i.i6285
  %16 = getelementptr inbounds nuw i8, ptr %self, i32 1088, !dbg !4817
  %_97.0 = load ptr, ptr %16, align 16, !dbg !4817, !nonnull !10, !noundef !10
  %17 = getelementptr inbounds nuw i8, ptr %self, i32 1092, !dbg !4817
  %_97.1 = load i32, ptr %17, align 4, !dbg !4817, !noundef !10
  %_8.i6295 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_97.0, i32 %_97.1, !dbg !4818
  br label %bb1.i.i6296, !dbg !4823

bb1.i.i6296:                                      ; preds = %bb11.i.i6299, %bb5
  %_221.i.i6297 = phi ptr [ %_22.i.i6300, %bb11.i.i6299 ], [ %_97.0, %bb5 ]
  %_12.i.i6298 = icmp eq ptr %_221.i.i6297, %_8.i6295, !dbg !4825
  br i1 %_12.i.i6298, label %bb7, label %bb11.i.i6299, !dbg !4828

bb11.i.i6299:                                     ; preds = %bb1.i.i6296
  %_22.i.i6300 = getelementptr inbounds nuw i8, ptr %_221.i.i6297, i32 16, !dbg !4829
  %18 = getelementptr inbounds nuw i8, ptr %_221.i.i6297, i32 12, !dbg !4831
  %_3.i.i.i6301 = load i32, ptr %18, align 4, !dbg !4831, !alias.scope !4833, !noalias !4838, !noundef !10
  %19 = icmp eq i32 %_3.i.i.i6301, 0, !dbg !4831
  %_51.i.i.i6302 = load i32, ptr %_221.i.i6297, align 4, !dbg !4831, !alias.scope !4833, !noalias !4838
  %20 = getelementptr inbounds nuw i8, ptr %_221.i.i6297, i32 4, !dbg !4831
  %_72.i.i.i6303 = load i32, ptr %20, align 4, !dbg !4831, !alias.scope !4833, !noalias !4838
  %21 = icmp eq i32 %_51.i.i.i6302, %_72.i.i.i6303, !dbg !4831
  %_0.sroa.0.0.off0.i.i.i6304 = select i1 %19, i1 %21, i1 false, !dbg !4831
  br i1 %_0.sroa.0.0.off0.i.i.i6304, label %bb1.i.i6296, label %bb20.thread, !dbg !4841

bb7:                                              ; preds = %bb1.i.i6296
  %22 = getelementptr inbounds nuw i8, ptr %self, i32 1096, !dbg !4842
  %_98.0 = load ptr, ptr %22, align 8, !dbg !4842, !nonnull !10, !noundef !10
  %23 = getelementptr inbounds nuw i8, ptr %self, i32 1100, !dbg !4842
  %_98.1 = load i32, ptr %23, align 4, !dbg !4842, !noundef !10
  %_8.i6306 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_98.0, i32 %_98.1, !dbg !4843
  br label %bb1.i.i6307, !dbg !4848

bb1.i.i6307:                                      ; preds = %bb11.i.i6310, %bb7
  %_221.i.i6308 = phi ptr [ %_22.i.i6311, %bb11.i.i6310 ], [ %_98.0, %bb7 ]
  %_12.i.i6309 = icmp eq ptr %_221.i.i6308, %_8.i6306, !dbg !4850
  br i1 %_12.i.i6309, label %bb9, label %bb11.i.i6310, !dbg !4853

bb11.i.i6310:                                     ; preds = %bb1.i.i6307
  %_22.i.i6311 = getelementptr inbounds nuw i8, ptr %_221.i.i6308, i32 16, !dbg !4854
  %24 = getelementptr inbounds nuw i8, ptr %_221.i.i6308, i32 12, !dbg !4856
  %_3.i.i.i6312 = load i32, ptr %24, align 4, !dbg !4856, !alias.scope !4858, !noalias !4863, !noundef !10
  %25 = icmp eq i32 %_3.i.i.i6312, 0, !dbg !4856
  %_51.i.i.i6313 = load i32, ptr %_221.i.i6308, align 4, !dbg !4856, !alias.scope !4858, !noalias !4863
  %26 = getelementptr inbounds nuw i8, ptr %_221.i.i6308, i32 4, !dbg !4856
  %_72.i.i.i6314 = load i32, ptr %26, align 4, !dbg !4856, !alias.scope !4858, !noalias !4863
  %27 = icmp eq i32 %_51.i.i.i6313, %_72.i.i.i6314, !dbg !4856
  %_0.sroa.0.0.off0.i.i.i6315 = select i1 %25, i1 %27, i1 false, !dbg !4856
  br i1 %_0.sroa.0.0.off0.i.i.i6315, label %bb1.i.i6307, label %bb20.thread, !dbg !4866

bb9:                                              ; preds = %bb1.i.i6307
  %_65.not = icmp ugt i32 %words, %left_io.1
  br i1 %_65.not, label %bb45, label %bb1.i6317, !dbg !4867, !prof !4694

bb45:                                             ; preds = %bb9
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef %words, i32 noundef %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ea4a5c70c3b6737d27e6f8140269468e) #33, !dbg !4876
  unreachable, !dbg !4876

bb1.i6317:                                        ; preds = %bb9, %bb12.i6323
  %io.sroa.5.0.i = phi i32 [ %len.i.i.i, %bb12.i6323 ], [ %words, %bb9 ]
  %io.sroa.0.0.i = phi ptr [ %data.i.i.i, %bb12.i6323 ], [ %left_io.0, %bb9 ]
  %28 = icmp eq i32 %io.sroa.5.0.i, 0, !dbg !4877
  br i1 %28, label %bb11, label %bb13.preheader.i, !dbg !4877

bb13.preheader.i:                                 ; preds = %bb1.i6317
  %spec.store.select.i6318 = tail call i32 @llvm.umin.i32(i32 %io.sroa.5.0.i, i32 32), !dbg !4880
  %data.i.i.idx.i = shl nuw nsw i32 %spec.store.select.i6318, 2, !dbg !4883
  %data.i.i.i = getelementptr inbounds nuw i8, ptr %io.sroa.0.0.i, i32 %data.i.i.idx.i, !dbg !4883
  br label %bb13.i6319, !dbg !4888

bb13.i6319:                                       ; preds = %bb13.i6319, %bb13.preheader.i
  %iter.sroa.0.08.i = phi ptr [ %_35.i6320, %bb13.i6319 ], [ %io.sroa.0.0.i, %bb13.preheader.i ]
  %bits.sroa.0.07.i = phi i32 [ %29, %bb13.i6319 ], [ 0, %bb13.preheader.i ]
  %_35.i6320 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i, i32 4, !dbg !4890
  %_95.i6321 = load i32, ptr %iter.sroa.0.08.i, align 4, !dbg !4892, !alias.scope !4893, !noundef !10
  %29 = or i32 %_95.i6321, %bits.sroa.0.07.i, !dbg !4896
  %_29.i6322 = icmp eq ptr %_35.i6320, %data.i.i.i, !dbg !4897
  br i1 %_29.i6322, label %bb12.i6323, label %bb13.i6319, !dbg !4888

bb12.i6323:                                       ; preds = %bb13.i6319
  %len.i.i.i = sub nuw nsw i32 %io.sroa.5.0.i, %spec.store.select.i6318, !dbg !4899
  %30 = icmp eq i32 %29, 0, !dbg !4900
  br i1 %30, label %bb1.i6317, label %bb20.thread, !dbg !4900

bb11:                                             ; preds = %bb1.i6317
  %_73.not = icmp ugt i32 %words, %right_io.1, !dbg !4901
  br i1 %_73.not, label %bb48, label %bb1.i6325, !dbg !4901, !prof !902

bb20.thread:                                      ; preds = %bb11.i.i, %bb11.i.i6288, %bb11.i.i6299, %bb11.i.i6310, %bb12.i6323, %start
  %31 = getelementptr inbounds nuw i8, ptr %self, i32 1124
  br label %bb26, !dbg !4907

bb20:                                             ; preds = %bb1.i6325
  %32 = getelementptr inbounds nuw i8, ptr %self, i32 1124
  %33 = load i8, ptr %32, align 4, !range !4765
  %_22 = trunc nuw i8 %33 to i1
  br i1 %_22, label %bb22, label %bb26, !dbg !4907

bb48:                                             ; preds = %bb11
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef %words, i32 noundef %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_786d3728118d574b1dded47beaf5441a) #33, !dbg !4909
  unreachable, !dbg !4909

bb1.i6325:                                        ; preds = %bb11, %bb12.i6339
  %io.sroa.5.0.i6326 = phi i32 [ %len.i.i.i6332, %bb12.i6339 ], [ %words, %bb11 ]
  %io.sroa.0.0.i6327 = phi ptr [ %data.i.i.i6331, %bb12.i6339 ], [ %right_io.0, %bb11 ]
  %34 = icmp eq i32 %io.sroa.5.0.i6326, 0, !dbg !4910
  br i1 %34, label %bb20, label %bb13.preheader.i6328, !dbg !4910

bb13.preheader.i6328:                             ; preds = %bb1.i6325
  %spec.store.select.i6329 = tail call i32 @llvm.umin.i32(i32 %io.sroa.5.0.i6326, i32 32), !dbg !4913
  %data.i.i.idx.i6330 = shl nuw nsw i32 %spec.store.select.i6329, 2, !dbg !4916
  %data.i.i.i6331 = getelementptr inbounds nuw i8, ptr %io.sroa.0.0.i6327, i32 %data.i.i.idx.i6330, !dbg !4916
  br label %bb13.i6333, !dbg !4921

bb13.i6333:                                       ; preds = %bb13.i6333, %bb13.preheader.i6328
  %iter.sroa.0.08.i6334 = phi ptr [ %_35.i6336, %bb13.i6333 ], [ %io.sroa.0.0.i6327, %bb13.preheader.i6328 ]
  %bits.sroa.0.07.i6335 = phi i32 [ %35, %bb13.i6333 ], [ 0, %bb13.preheader.i6328 ]
  %_35.i6336 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i6334, i32 4, !dbg !4923
  %_95.i6337 = load i32, ptr %iter.sroa.0.08.i6334, align 4, !dbg !4925, !alias.scope !4926, !noundef !10
  %35 = or i32 %_95.i6337, %bits.sroa.0.07.i6335, !dbg !4929
  %_29.i6338 = icmp eq ptr %_35.i6336, %data.i.i.i6331, !dbg !4930
  br i1 %_29.i6338, label %bb12.i6339, label %bb13.i6333, !dbg !4921

bb12.i6339:                                       ; preds = %bb13.i6333
  %len.i.i.i6332 = sub nuw nsw i32 %io.sroa.5.0.i6326, %spec.store.select.i6329, !dbg !4932
  %36 = icmp eq i32 %35, 0, !dbg !4933
  br i1 %36, label %bb1.i6325, label %bb20.thread15026, !dbg !4933

bb20.thread15026:                                 ; preds = %bb12.i6339
  %37 = getelementptr inbounds nuw i8, ptr %self, i32 1124
  br label %bb26, !dbg !4907

bb26:                                             ; preds = %bb20.thread15026, %bb20.thread, %bb20
  %38 = phi ptr [ %31, %bb20.thread ], [ %32, %bb20 ], [ %37, %bb20.thread15026 ]
  %quiet.sroa.0.0.off015025 = phi i1 [ false, %bb20.thread ], [ true, %bb20 ], [ false, %bb20.thread15026 ]
  %_31 = getelementptr inbounds nuw i8, ptr %self, i32 16, !dbg !4934
  %_32 = getelementptr inbounds nuw i8, ptr %self, i32 912, !dbg !4935
  %_33 = getelementptr inbounds nuw i8, ptr %self, i32 924, !dbg !4936
  %_34 = getelementptr inbounds nuw i8, ptr %self, i32 1024, !dbg !4937
  %_35 = getelementptr inbounds nuw i8, ptr %self, i32 800, !dbg !4938
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4939), !dbg !4942
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4945), !dbg !4942
  %_8.i6342 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_95.0, i32 %_95.1, !dbg !4947
  br label %bb1.i.i6343, !dbg !4954

bb1.i.i6343:                                      ; preds = %bb11.i.i6346, %bb26
  %_221.i.i6344 = phi ptr [ %_22.i.i6347, %bb11.i.i6346 ], [ %_95.0, %bb26 ]
  %_12.i.i6345 = icmp eq ptr %_221.i.i6344, %_8.i6342, !dbg !4956
  br i1 %_12.i.i6345, label %bb2.i1560, label %bb11.i.i6346, !dbg !4959

bb11.i.i6346:                                     ; preds = %bb1.i.i6343
  %_22.i.i6347 = getelementptr inbounds nuw i8, ptr %_221.i.i6344, i32 16, !dbg !4960
  %39 = getelementptr inbounds nuw i8, ptr %_221.i.i6344, i32 12, !dbg !4962
  %_3.i.i.i6348 = load i32, ptr %39, align 4, !dbg !4962, !alias.scope !4964, !noalias !4969, !noundef !10
  %40 = icmp eq i32 %_3.i.i.i6348, 0, !dbg !4962
  %_51.i.i.i6349 = load i32, ptr %_221.i.i6344, align 4, !dbg !4962, !alias.scope !4964, !noalias !4969
  %41 = getelementptr inbounds nuw i8, ptr %_221.i.i6344, i32 4, !dbg !4962
  %_72.i.i.i6350 = load i32, ptr %41, align 4, !dbg !4962, !alias.scope !4964, !noalias !4969
  %42 = icmp eq i32 %_51.i.i.i6349, %_72.i.i.i6350, !dbg !4962
  %_0.sroa.0.0.off0.i.i.i6351 = select i1 %40, i1 %42, i1 false, !dbg !4962
  br i1 %_0.sroa.0.0.off0.i.i.i6351, label %bb1.i.i6343, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, !dbg !4978

bb2.i1560:                                        ; preds = %bb1.i.i6343
  %43 = getelementptr inbounds nuw i8, ptr %self, i32 996, !dbg !4979
  %_15.0.i = load ptr, ptr %43, align 4, !dbg !4979, !alias.scope !4939, !noalias !4980, !nonnull !10, !noundef !10
  %44 = getelementptr inbounds nuw i8, ptr %self, i32 1000, !dbg !4979
  %_15.1.i = load i32, ptr %44, align 4, !dbg !4979, !alias.scope !4939, !noalias !4980, !noundef !10
  %_8.i6353 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_15.0.i, i32 %_15.1.i, !dbg !4981
  br label %bb1.i.i6354, !dbg !4986

bb1.i.i6354:                                      ; preds = %bb11.i.i6357, %bb2.i1560
  %_221.i.i6355 = phi ptr [ %_22.i.i6358, %bb11.i.i6357 ], [ %_15.0.i, %bb2.i1560 ]
  %_12.i.i6356 = icmp eq ptr %_221.i.i6355, %_8.i6353, !dbg !4988
  br i1 %_12.i.i6356, label %bb4.i1562, label %bb11.i.i6357, !dbg !4991

bb11.i.i6357:                                     ; preds = %bb1.i.i6354
  %_22.i.i6358 = getelementptr inbounds nuw i8, ptr %_221.i.i6355, i32 16, !dbg !4992
  %45 = getelementptr inbounds nuw i8, ptr %_221.i.i6355, i32 12, !dbg !4994
  %_3.i.i.i6359 = load i32, ptr %45, align 4, !dbg !4994, !alias.scope !4996, !noalias !5001, !noundef !10
  %46 = icmp eq i32 %_3.i.i.i6359, 0, !dbg !4994
  %_51.i.i.i6360 = load i32, ptr %_221.i.i6355, align 4, !dbg !4994, !alias.scope !4996, !noalias !5001
  %47 = getelementptr inbounds nuw i8, ptr %_221.i.i6355, i32 4, !dbg !4994
  %_72.i.i.i6361 = load i32, ptr %47, align 4, !dbg !4994, !alias.scope !4996, !noalias !5001
  %48 = icmp eq i32 %_51.i.i.i6360, %_72.i.i.i6361, !dbg !4994
  %_0.sroa.0.0.off0.i.i.i6362 = select i1 %46, i1 %48, i1 false, !dbg !4994
  br i1 %_0.sroa.0.0.off0.i.i.i6362, label %bb1.i.i6354, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, !dbg !5004

bb4.i1562:                                        ; preds = %bb1.i.i6354
  %49 = getelementptr inbounds nuw i8, ptr %self, i32 1088, !dbg !5005
  %_16.0.i = load ptr, ptr %49, align 4, !dbg !5005, !alias.scope !4945, !noalias !5006, !nonnull !10, !noundef !10
  %50 = getelementptr inbounds nuw i8, ptr %self, i32 1092, !dbg !5005
  %_16.1.i = load i32, ptr %50, align 4, !dbg !5005, !alias.scope !4945, !noalias !5006, !noundef !10
  %_8.i6364 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_16.0.i, i32 %_16.1.i, !dbg !5007
  br label %bb1.i.i6365, !dbg !5012

bb1.i.i6365:                                      ; preds = %bb11.i.i6368, %bb4.i1562
  %_221.i.i6366 = phi ptr [ %_22.i.i6369, %bb11.i.i6368 ], [ %_16.0.i, %bb4.i1562 ]
  %_12.i.i6367 = icmp eq ptr %_221.i.i6366, %_8.i6364, !dbg !5014
  br i1 %_12.i.i6367, label %bb6.i1563, label %bb11.i.i6368, !dbg !5017

bb11.i.i6368:                                     ; preds = %bb1.i.i6365
  %_22.i.i6369 = getelementptr inbounds nuw i8, ptr %_221.i.i6366, i32 16, !dbg !5018
  %51 = getelementptr inbounds nuw i8, ptr %_221.i.i6366, i32 12, !dbg !5020
  %_3.i.i.i6370 = load i32, ptr %51, align 4, !dbg !5020, !alias.scope !5022, !noalias !5027, !noundef !10
  %52 = icmp eq i32 %_3.i.i.i6370, 0, !dbg !5020
  %_51.i.i.i6371 = load i32, ptr %_221.i.i6366, align 4, !dbg !5020, !alias.scope !5022, !noalias !5027
  %53 = getelementptr inbounds nuw i8, ptr %_221.i.i6366, i32 4, !dbg !5020
  %_72.i.i.i6372 = load i32, ptr %53, align 4, !dbg !5020, !alias.scope !5022, !noalias !5027
  %54 = icmp eq i32 %_51.i.i.i6371, %_72.i.i.i6372, !dbg !5020
  %_0.sroa.0.0.off0.i.i.i6373 = select i1 %52, i1 %54, i1 false, !dbg !5020
  br i1 %_0.sroa.0.0.off0.i.i.i6373, label %bb1.i.i6365, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, !dbg !5030

bb6.i1563:                                        ; preds = %bb1.i.i6365
  %55 = getelementptr inbounds nuw i8, ptr %self, i32 1096, !dbg !5031
  %_17.0.i = load ptr, ptr %55, align 4, !dbg !5031, !alias.scope !4945, !noalias !5006, !nonnull !10, !noundef !10
  %56 = getelementptr inbounds nuw i8, ptr %self, i32 1100, !dbg !5031
  %_17.1.i = load i32, ptr %56, align 4, !dbg !5031, !alias.scope !4945, !noalias !5006, !noundef !10
  %_8.i6375 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_17.0.i, i32 %_17.1.i, !dbg !5032
  br label %bb1.i.i6376, !dbg !5037

bb1.i.i6376:                                      ; preds = %bb11.i.i6379, %bb6.i1563
  %_221.i.i6377 = phi ptr [ %_22.i.i6380, %bb11.i.i6379 ], [ %_17.0.i, %bb6.i1563 ]
  %_12.i.i6378 = icmp eq ptr %_221.i.i6377, %_8.i6375, !dbg !5039
  br i1 %_12.i.i6378, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, label %bb11.i.i6379, !dbg !5042

bb11.i.i6379:                                     ; preds = %bb1.i.i6376
  %_22.i.i6380 = getelementptr inbounds nuw i8, ptr %_221.i.i6377, i32 16, !dbg !5043
  %57 = getelementptr inbounds nuw i8, ptr %_221.i.i6377, i32 12, !dbg !5045
  %_3.i.i.i6381 = load i32, ptr %57, align 4, !dbg !5045, !alias.scope !5047, !noalias !5052, !noundef !10
  %58 = icmp eq i32 %_3.i.i.i6381, 0, !dbg !5045
  %_51.i.i.i6382 = load i32, ptr %_221.i.i6377, align 4, !dbg !5045, !alias.scope !5047, !noalias !5052
  %59 = getelementptr inbounds nuw i8, ptr %_221.i.i6377, i32 4, !dbg !5045
  %_72.i.i.i6383 = load i32, ptr %59, align 4, !dbg !5045, !alias.scope !5047, !noalias !5052
  %60 = icmp eq i32 %_51.i.i.i6382, %_72.i.i.i6383, !dbg !5045
  %_0.sroa.0.0.off0.i.i.i6384 = select i1 %58, i1 %60, i1 false, !dbg !5045
  br i1 %_0.sroa.0.0.off0.i.i.i6384, label %bb1.i.i6376, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, !dbg !5055

_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit: ; preds = %bb11.i.i6346, %bb11.i.i6357, %bb11.i.i6368, %bb11.i.i6379, %bb1.i.i6376
  %_0.sroa.0.0.off0.i = phi i1 [ false, %bb11.i.i6368 ], [ false, %bb11.i.i6357 ], [ false, %bb11.i.i6379 ], [ true, %bb1.i.i6376 ], [ false, %bb11.i.i6346 ]
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5056), !dbg !5059
  %61 = getelementptr inbounds nuw i8, ptr %self, i32 1012, !dbg !5061
  %_31.0.i = load ptr, ptr %61, align 4, !dbg !5061, !alias.scope !5056, !noalias !5063, !nonnull !10, !noundef !10
  %62 = getelementptr inbounds nuw i8, ptr %self, i32 1016, !dbg !5061
  %_31.1.i = load i32, ptr %62, align 4, !dbg !5061, !alias.scope !5056, !noalias !5063, !noundef !10
  %_17.idx.i = mul nuw nsw i32 %_31.1.i, 12, !dbg !5064
  %_17.i = getelementptr inbounds nuw i8, ptr %_31.0.i, i32 %_17.idx.i, !dbg !5064
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5068), !dbg !5071, !noalias !5063
  %_5.not.i.i.i = icmp eq i32 %_31.1.i, 0
  %63 = getelementptr inbounds nuw i8, ptr %_31.0.i, i32 4
  %64 = getelementptr inbounds nuw i8, ptr %_31.0.i, i32 8
  br i1 %_5.not.i.i.i, label %bb2.i6397, label %bb1.i.i6386

bb1.i.i6386:                                      ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i
  %_224.i.i = phi ptr [ %_22.i.i6389, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i ], [ %_31.0.i, %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit ]
  %_12.i.i6387 = icmp eq ptr %_224.i.i, %_17.i, !dbg !5072
  br i1 %_12.i.i6387, label %bb2.i6397, label %bb11.i.i6388, !dbg !5076

bb11.i.i6388:                                     ; preds = %bb1.i.i6386
  %_22.i.i6389 = getelementptr inbounds nuw i8, ptr %_224.i.i, i32 12, !dbg !5077
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5079), !dbg !5082, !noalias !5063
  %_9.i.i.i6390 = load i32, ptr %_224.i.i, align 4, !dbg !5083, !alias.scope !5079, !noalias !5086, !noundef !10
  %_10.i.i.i = load i32, ptr %_31.0.i, align 4, !dbg !5083, !alias.scope !5068, !noalias !5088, !noundef !10
  %_8.i.i.i = icmp eq i32 %_9.i.i.i6390, %_10.i.i.i, !dbg !5083
  br i1 %_8.i.i.i, label %bb2.i.i.i, label %bb10.i, !dbg !5083

bb2.i.i.i:                                        ; preds = %bb11.i.i6388
  %65 = getelementptr inbounds nuw i8, ptr %_224.i.i, i32 4, !dbg !5083
  %_12.i.i.i6393 = load i32, ptr %65, align 4, !dbg !5083, !alias.scope !5079, !noalias !5086, !noundef !10
  %_13.i.i.i6394 = load i32, ptr %63, align 4, !dbg !5083, !alias.scope !5068, !noalias !5088, !noundef !10
  %_11.i.i.i = icmp eq i32 %_12.i.i.i6393, %_13.i.i.i6394, !dbg !5083
  br i1 %_11.i.i.i, label %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, label %bb10.i, !dbg !5083

_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i: ; preds = %bb2.i.i.i
  %66 = getelementptr inbounds nuw i8, ptr %_224.i.i, i32 8, !dbg !5083
  %_14.i.i.i6395 = load i32, ptr %66, align 4, !dbg !5083, !alias.scope !5079, !noalias !5086, !noundef !10
  %_15.i.i.i6396 = load i32, ptr %64, align 4, !dbg !5083, !alias.scope !5068, !noalias !5088, !noundef !10
  %67 = icmp eq i32 %_14.i.i.i6395, %_15.i.i.i6396, !dbg !5083
  br i1 %67, label %bb1.i.i6386, label %bb10.i, !dbg !5082

bb2.i6397:                                        ; preds = %bb1.i.i6386, %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit
  %68 = getelementptr inbounds nuw i8, ptr %self, i32 980, !dbg !5089
  %_32.0.i = load ptr, ptr %68, align 4, !dbg !5089, !alias.scope !5056, !noalias !5063, !nonnull !10, !noundef !10
  %69 = getelementptr inbounds nuw i8, ptr %self, i32 984, !dbg !5089
  %_32.1.i = load i32, ptr %69, align 4, !dbg !5089, !alias.scope !5056, !noalias !5063, !noundef !10
  %_26.idx.i = shl nuw nsw i32 %_32.1.i, 2, !dbg !5090
  %_26.i = getelementptr inbounds nuw i8, ptr %_32.0.i, i32 %_26.idx.i, !dbg !5090
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5094), !dbg !5097, !noalias !5063
  %_6.not.i.i.i = icmp eq i32 %_32.1.i, 0
  br i1 %_6.not.i.i.i, label %bb3.i, label %bb1.i3.i

bb1.i3.i:                                         ; preds = %bb2.i6397, %bb11.i5.i
  %_223.i.i = phi ptr [ %_22.i6.i, %bb11.i5.i ], [ %_32.0.i, %bb2.i6397 ]
  %_12.i4.i = icmp eq ptr %_223.i.i, %_26.i, !dbg !5098
  br i1 %_12.i4.i, label %bb3.i, label %bb11.i5.i, !dbg !5102

bb11.i5.i:                                        ; preds = %bb1.i3.i
  %_22.i6.i = getelementptr inbounds nuw i8, ptr %_223.i.i, i32 4, !dbg !5103
  %ptr.val.i.i = load i32, ptr %_223.i.i, align 4, !dbg !5105, !noalias !5106
  %_4.i.i.i6398 = load i32, ptr %_32.0.i, align 4, !dbg !5108, !alias.scope !5094, !noalias !5110, !noundef !10
  %_0.i.i.i = icmp eq i32 %ptr.val.i.i, %_4.i.i.i6398, !dbg !5111
  br i1 %_0.i.i.i, label %bb1.i3.i, label %bb10.i, !dbg !5105

bb3.i:                                            ; preds = %bb1.i3.i, %bb2.i6397
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5112), !dbg !5115
  %70 = getelementptr inbounds nuw i8, ptr %self, i32 1112, !dbg !5116
  %_31.0.i6399 = load ptr, ptr %70, align 4, !dbg !5116, !alias.scope !5112, !noalias !5063, !nonnull !10, !noundef !10
  %71 = getelementptr inbounds nuw i8, ptr %self, i32 1116, !dbg !5116
  %_31.1.i6400 = load i32, ptr %71, align 4, !dbg !5116, !alias.scope !5112, !noalias !5063, !noundef !10
  %_17.idx.i6401 = mul nuw nsw i32 %_31.1.i6400, 12, !dbg !5118
  %_17.i6402 = getelementptr inbounds nuw i8, ptr %_31.0.i6399, i32 %_17.idx.i6401, !dbg !5118
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5122), !dbg !5125, !noalias !5063
  %_5.not.i.i.i6403 = icmp eq i32 %_31.1.i6400, 0
  %72 = getelementptr inbounds nuw i8, ptr %_31.0.i6399, i32 4
  %73 = getelementptr inbounds nuw i8, ptr %_31.0.i6399, i32 8
  br i1 %_5.not.i.i.i6403, label %bb2.i6421, label %bb1.i.i6404

bb1.i.i6404:                                      ; preds = %bb3.i, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i6418
  %_224.i.i6405 = phi ptr [ %_22.i.i6408, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i6418 ], [ %_31.0.i6399, %bb3.i ]
  %_12.i.i6406 = icmp eq ptr %_224.i.i6405, %_17.i6402, !dbg !5126
  br i1 %_12.i.i6406, label %bb2.i6421, label %bb11.i.i6407, !dbg !5130

bb11.i.i6407:                                     ; preds = %bb1.i.i6404
  %_22.i.i6408 = getelementptr inbounds nuw i8, ptr %_224.i.i6405, i32 12, !dbg !5131
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5133), !dbg !5136, !noalias !5063
  %_9.i.i.i6409 = load i32, ptr %_224.i.i6405, align 4, !dbg !5137, !alias.scope !5133, !noalias !5140, !noundef !10
  %_10.i.i.i6410 = load i32, ptr %_31.0.i6399, align 4, !dbg !5137, !alias.scope !5122, !noalias !5142, !noundef !10
  %_8.i.i.i6411 = icmp eq i32 %_9.i.i.i6409, %_10.i.i.i6410, !dbg !5137
  br i1 %_8.i.i.i6411, label %bb2.i.i.i6414, label %bb10.i, !dbg !5137

bb2.i.i.i6414:                                    ; preds = %bb11.i.i6407
  %74 = getelementptr inbounds nuw i8, ptr %_224.i.i6405, i32 4, !dbg !5137
  %_12.i.i.i6415 = load i32, ptr %74, align 4, !dbg !5137, !alias.scope !5133, !noalias !5140, !noundef !10
  %_13.i.i.i6416 = load i32, ptr %72, align 4, !dbg !5137, !alias.scope !5122, !noalias !5142, !noundef !10
  %_11.i.i.i6417 = icmp eq i32 %_12.i.i.i6415, %_13.i.i.i6416, !dbg !5137
  br i1 %_11.i.i.i6417, label %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i6418, label %bb10.i, !dbg !5137

_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i6418: ; preds = %bb2.i.i.i6414
  %75 = getelementptr inbounds nuw i8, ptr %_224.i.i6405, i32 8, !dbg !5137
  %_14.i.i.i6419 = load i32, ptr %75, align 4, !dbg !5137, !alias.scope !5133, !noalias !5140, !noundef !10
  %_15.i.i.i6420 = load i32, ptr %73, align 4, !dbg !5137, !alias.scope !5122, !noalias !5142, !noundef !10
  %76 = icmp eq i32 %_14.i.i.i6419, %_15.i.i.i6420, !dbg !5137
  br i1 %76, label %bb1.i.i6404, label %bb10.i, !dbg !5136

bb2.i6421:                                        ; preds = %bb1.i.i6404, %bb3.i
  %77 = getelementptr inbounds nuw i8, ptr %self, i32 1080, !dbg !5143
  %_32.0.i6422 = load ptr, ptr %77, align 4, !dbg !5143, !alias.scope !5112, !noalias !5063, !nonnull !10, !noundef !10
  %78 = getelementptr inbounds nuw i8, ptr %self, i32 1084, !dbg !5143
  %_32.1.i6423 = load i32, ptr %78, align 4, !dbg !5143, !alias.scope !5112, !noalias !5063, !noundef !10
  %_26.idx.i6424 = shl nuw nsw i32 %_32.1.i6423, 2, !dbg !5144
  %_26.i6425 = getelementptr inbounds nuw i8, ptr %_32.0.i6422, i32 %_26.idx.i6424, !dbg !5144
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5148), !dbg !5151, !noalias !5063
  %_6.not.i.i.i6426 = icmp eq i32 %_32.1.i6423, 0
  br i1 %_6.not.i.i.i6426, label %bb5.i, label %bb1.i3.i6427

bb1.i3.i6427:                                     ; preds = %bb2.i6421, %bb11.i5.i6430
  %_223.i.i6428 = phi ptr [ %_22.i6.i6431, %bb11.i5.i6430 ], [ %_32.0.i6422, %bb2.i6421 ]
  %_12.i4.i6429 = icmp eq ptr %_223.i.i6428, %_26.i6425, !dbg !5152
  br i1 %_12.i4.i6429, label %bb5.i, label %bb11.i5.i6430, !dbg !5156

bb11.i5.i6430:                                    ; preds = %bb1.i3.i6427
  %_22.i6.i6431 = getelementptr inbounds nuw i8, ptr %_223.i.i6428, i32 4, !dbg !5157
  %ptr.val.i.i6432 = load i32, ptr %_223.i.i6428, align 4, !dbg !5159, !noalias !5160
  %_4.i.i.i6433 = load i32, ptr %_32.0.i6422, align 4, !dbg !5162, !alias.scope !5148, !noalias !5164, !noundef !10
  %_0.i.i.i6434 = icmp eq i32 %ptr.val.i.i6432, %_4.i.i.i6433, !dbg !5165
  br i1 %_0.i.i.i6434, label %bb1.i3.i6427, label %bb10.i, !dbg !5159

bb10.i:                                           ; preds = %bb11.i.i6388, %bb2.i.i.i, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, %bb11.i5.i, %bb11.i.i6407, %bb2.i.i.i6414, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i6418, %bb11.i5.i6430
  %79 = getelementptr inbounds nuw i8, ptr %self, i32 784, !dbg !5166
  %80 = getelementptr inbounds nuw i8, ptr %self, i32 785, !dbg !5166
  %81 = getelementptr inbounds nuw i8, ptr %self, i32 804, !dbg !5166
  %_111.not.i18055 = icmp eq i32 %frames, 0, !dbg !5166
  br i1 %_0.sroa.0.0.off0.i, label %bb11.i, label %bb12.i, !dbg !5167

bb5.i:                                            ; preds = %bb1.i3.i6427, %bb2.i6421
  %82 = getelementptr inbounds nuw i8, ptr %self, i32 784, !dbg !5166
  %83 = getelementptr inbounds nuw i8, ptr %self, i32 785, !dbg !5166
  %84 = getelementptr inbounds nuw i8, ptr %self, i32 916, !dbg !5166
  %85 = getelementptr inbounds nuw i8, ptr %self, i32 920, !dbg !5166
  %86 = getelementptr inbounds nuw i8, ptr %self, i32 804, !dbg !5166
  br i1 %_0.sroa.0.0.off0.i, label %bb6.i, label %bb7.i, !dbg !5168

bb12.i:                                           ; preds = %bb10.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5169), !dbg !5172
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5173), !dbg !5172
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5175), !dbg !5172
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5177), !dbg !5172
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5179), !dbg !5172
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_(ptr noalias noundef align 16 captures(none) dereferenceable(368) %hot_left.i761, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #32, !dbg !5181
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_(ptr noalias noundef align 16 captures(none) dereferenceable(368) %hot_right.i760, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #32, !dbg !5185
  %87 = load i8, ptr %79, align 16, !dbg !5187, !range !4765, !alias.scope !5169, !noalias !5191, !noundef !10
  %88 = load i8, ptr %80, align 1, !dbg !5194, !range !4765, !alias.scope !5169, !noalias !5191, !noundef !10
  %_35.i769 = load i32, ptr %_35, align 4, !dbg !5196, !alias.scope !5179, !noalias !5198, !noundef !10
  %_37.i770 = load i32, ptr %81, align 4, !dbg !5199, !alias.scope !5179, !noalias !5198, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i758), !dbg !5201, !noalias !5203
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i758, i8 0, i32 32, i1 false), !noalias !5203
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i757), !dbg !5204, !noalias !5203
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i757, i8 0, i32 1024, i1 false), !noalias !5203
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i756), !dbg !5206, !noalias !5203
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i756, i8 0, i32 1024, i1 false), !noalias !5203
  br i1 %_111.not.i18055, label %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb37.i784.lr.ph, !dbg !5208

bb37.i784.lr.ph:                                  ; preds = %bb12.i
  %_33.i767 = trunc nuw i8 %88 to i1, !dbg !5194
  %spec.store.select28.i768 = select i1 %_33.i767, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !5194
  %_32.i765 = trunc nuw i8 %87 to i1, !dbg !5187
  %link.sroa.0.0.i766 = select i1 %_32.i765, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !5187
  %d9.i = lshr i32 %frames, 5, !dbg !5222
  %r2.i = and i32 %frames, 31, !dbg !5234
  %_19.not.i = icmp ne i32 %r2.i, 0, !dbg !5236
  %89 = zext i1 %_19.not.i to i32, !dbg !5236
  %yield_count.sroa.0.0.i = add nuw nsw i32 %d9.i, %89, !dbg !5236
  %history.i141.i.sroa.7.0.hot_left.i761.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i761, i32 16
  %history.i141.i.sroa.10.0.hot_left.i761.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i761, i32 32
  %history.i141.i.sroa.13.0.hot_left.i761.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i761, i32 48
  %history.i141.i.sroa.16.0.hot_left.i761.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i761, i32 64
  %history.i141.i.sroa.19.0.hot_left.i761.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i761, i32 80
  %history.i141.i.sroa.22.0.hot_left.i761.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i761, i32 96
  %history.i141.i.sroa.26.0.hot_left.i761.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i761, i32 112
  %history.i141.i.sroa.29.0.hot_left.i761.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i761, i32 128
  %history.i141.i.sroa.32.0.hot_left.i761.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i761, i32 144
  %history.i141.i.sroa.35.0.hot_left.i761.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i761, i32 160
  %history.i141.i.sroa.38.0.hot_left.i761.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i761, i32 176
  %90 = getelementptr inbounds nuw i8, ptr %self, i32 32
  %91 = getelementptr inbounds nuw i8, ptr %self, i32 48
  %92 = getelementptr inbounds nuw i8, ptr %self, i32 64
  %row1.i.i.i178.i = getelementptr inbounds nuw i8, ptr %self, i32 80
  %93 = getelementptr inbounds nuw i8, ptr %self, i32 96
  %94 = getelementptr inbounds nuw i8, ptr %self, i32 112
  %95 = getelementptr inbounds nuw i8, ptr %self, i32 128
  %row3.i.i.i192.i = getelementptr inbounds nuw i8, ptr %self, i32 144
  %96 = getelementptr inbounds nuw i8, ptr %self, i32 160
  %97 = getelementptr inbounds nuw i8, ptr %self, i32 176
  %98 = getelementptr inbounds nuw i8, ptr %self, i32 192
  %row5.i.i.i206.i = getelementptr inbounds nuw i8, ptr %self, i32 208
  %99 = getelementptr inbounds nuw i8, ptr %self, i32 224
  %100 = getelementptr inbounds nuw i8, ptr %self, i32 240
  %101 = getelementptr inbounds nuw i8, ptr %self, i32 256
  %row7.i.i.i220.i = getelementptr inbounds nuw i8, ptr %self, i32 272
  %102 = getelementptr inbounds nuw i8, ptr %self, i32 288
  %103 = getelementptr inbounds nuw i8, ptr %self, i32 304
  %104 = getelementptr inbounds nuw i8, ptr %self, i32 320
  %row9.i.i.i234.i = getelementptr inbounds nuw i8, ptr %self, i32 336
  %105 = getelementptr inbounds nuw i8, ptr %self, i32 352
  %106 = getelementptr inbounds nuw i8, ptr %self, i32 368
  %107 = getelementptr inbounds nuw i8, ptr %self, i32 384
  %row11.i.i.i248.i = getelementptr inbounds nuw i8, ptr %self, i32 400
  %108 = getelementptr inbounds nuw i8, ptr %self, i32 416
  %109 = getelementptr inbounds nuw i8, ptr %self, i32 432
  %110 = getelementptr inbounds nuw i8, ptr %self, i32 448
  %row13.i.i.i262.i = getelementptr inbounds nuw i8, ptr %self, i32 464
  %111 = getelementptr inbounds nuw i8, ptr %self, i32 480
  %112 = getelementptr inbounds nuw i8, ptr %self, i32 496
  %113 = getelementptr inbounds nuw i8, ptr %self, i32 512
  %row15.i.i.i276.i = getelementptr inbounds nuw i8, ptr %self, i32 528
  %114 = getelementptr inbounds nuw i8, ptr %self, i32 544
  %115 = getelementptr inbounds nuw i8, ptr %self, i32 560
  %116 = getelementptr inbounds nuw i8, ptr %self, i32 576
  %row17.i.i.i290.i = getelementptr inbounds nuw i8, ptr %self, i32 592
  %117 = getelementptr inbounds nuw i8, ptr %self, i32 608
  %118 = getelementptr inbounds nuw i8, ptr %self, i32 624
  %119 = getelementptr inbounds nuw i8, ptr %self, i32 640
  %row19.i.i.i304.i = getelementptr inbounds nuw i8, ptr %self, i32 656
  %120 = getelementptr inbounds nuw i8, ptr %self, i32 672
  %121 = getelementptr inbounds nuw i8, ptr %self, i32 688
  %122 = getelementptr inbounds nuw i8, ptr %self, i32 704
  %row21.i.i.i318.i = getelementptr inbounds nuw i8, ptr %self, i32 720
  %123 = getelementptr inbounds nuw i8, ptr %self, i32 736
  %124 = getelementptr inbounds nuw i8, ptr %self, i32 752
  %125 = getelementptr inbounds nuw i8, ptr %self, i32 768
  %history.i.i751.sroa.7.0.hot_right.i760.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i760, i32 16
  %history.i.i751.sroa.10.0.hot_right.i760.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i760, i32 32
  %history.i.i751.sroa.13.0.hot_right.i760.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i760, i32 48
  %history.i.i751.sroa.16.0.hot_right.i760.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i760, i32 64
  %history.i.i751.sroa.19.0.hot_right.i760.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i760, i32 80
  %history.i.i751.sroa.22.0.hot_right.i760.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i760, i32 96
  %history.i.i751.sroa.26.0.hot_right.i760.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i760, i32 112
  %history.i.i751.sroa.29.0.hot_right.i760.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i760, i32 128
  %history.i.i751.sroa.32.0.hot_right.i760.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i760, i32 144
  %history.i.i751.sroa.35.0.hot_right.i760.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i760, i32 160
  %history.i.i751.sroa.38.0.hot_right.i760.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i760, i32 176
  %_64.i798 = getelementptr inbounds nuw i8, ptr %hot_left.i761, i32 192
  %_65.i799 = getelementptr inbounds nuw i8, ptr %hot_left.i761, i32 256
  %126 = getelementptr inbounds nuw i8, ptr %hot_left.i761, i32 240
  %127 = getelementptr inbounds nuw i8, ptr %hot_left.i761, i32 224
  %128 = getelementptr inbounds nuw i8, ptr %hot_left.i761, i32 208
  %129 = getelementptr inbounds nuw i8, ptr %hot_left.i761, i32 304
  %130 = getelementptr inbounds nuw i8, ptr %hot_left.i761, i32 288
  %131 = getelementptr inbounds nuw i8, ptr %hot_left.i761, i32 272
  %_69.i802 = getelementptr inbounds nuw i8, ptr %hot_right.i760, i32 192
  %_70.i803 = getelementptr inbounds nuw i8, ptr %hot_right.i760, i32 256
  %132 = getelementptr inbounds nuw i8, ptr %hot_right.i760, i32 240
  %133 = getelementptr inbounds nuw i8, ptr %hot_right.i760, i32 224
  %134 = getelementptr inbounds nuw i8, ptr %hot_right.i760, i32 208
  %135 = getelementptr inbounds nuw i8, ptr %hot_right.i760, i32 304
  %136 = getelementptr inbounds nuw i8, ptr %hot_right.i760, i32 288
  %137 = getelementptr inbounds nuw i8, ptr %hot_right.i760, i32 272
  %138 = bitcast <4 x i32> %link.sroa.0.0.i766 to <16 x i8>
  %139 = getelementptr inbounds nuw i8, ptr %self, i32 916
  %140 = getelementptr inbounds nuw i8, ptr %self, i32 1020
  %141 = getelementptr inbounds nuw i8, ptr %self, i32 944
  %142 = getelementptr inbounds nuw i8, ptr %self, i32 940
  %143 = getelementptr inbounds nuw i8, ptr %self, i32 984
  %144 = getelementptr inbounds nuw i8, ptr %self, i32 980
  %145 = getelementptr inbounds nuw i8, ptr %self, i32 968
  %146 = getelementptr inbounds nuw i8, ptr %self, i32 964
  %147 = getelementptr inbounds nuw i8, ptr %self, i32 952
  %148 = getelementptr inbounds nuw i8, ptr %self, i32 948
  %149 = getelementptr inbounds nuw i8, ptr %hot_left.i761, i32 336
  %150 = getelementptr inbounds nuw i8, ptr %hot_left.i761, i32 352
  %151 = getelementptr inbounds nuw i8, ptr %hot_left.i761, i32 320
  %152 = getelementptr inbounds nuw i8, ptr %self, i32 936
  %153 = getelementptr inbounds nuw i8, ptr %self, i32 932
  %154 = bitcast <4 x i32> %spec.store.select28.i768 to <16 x i8>
  %155 = getelementptr inbounds nuw i8, ptr %self, i32 1120
  %156 = getelementptr inbounds nuw i8, ptr %self, i32 1044
  %157 = getelementptr inbounds nuw i8, ptr %self, i32 1040
  %158 = getelementptr inbounds nuw i8, ptr %self, i32 1116
  %159 = getelementptr inbounds nuw i8, ptr %self, i32 1112
  %160 = getelementptr inbounds nuw i8, ptr %self, i32 1084
  %161 = getelementptr inbounds nuw i8, ptr %self, i32 1080
  %162 = getelementptr inbounds nuw i8, ptr %self, i32 1068
  %163 = getelementptr inbounds nuw i8, ptr %self, i32 1064
  %164 = getelementptr inbounds nuw i8, ptr %self, i32 1052
  %165 = getelementptr inbounds nuw i8, ptr %self, i32 1048
  %166 = getelementptr inbounds nuw i8, ptr %hot_right.i760, i32 336
  %167 = getelementptr inbounds nuw i8, ptr %hot_right.i760, i32 352
  %168 = getelementptr inbounds nuw i8, ptr %hot_right.i760, i32 320
  %169 = getelementptr inbounds nuw i8, ptr %self, i32 1036
  %170 = getelementptr inbounds nuw i8, ptr %self, i32 1032
  %171 = getelementptr inbounds nuw i8, ptr %self, i32 920
  %iter.sroa.0.0.ptr.i53.i84817798.1 = getelementptr inbounds nuw i8, ptr %scratch.i758, i32 4
  %iter.sroa.0.0.ptr.i53.i84817798.2 = getelementptr inbounds nuw i8, ptr %scratch.i758, i32 8
  %iter.sroa.0.0.ptr.i53.i84817798.3 = getelementptr inbounds nuw i8, ptr %scratch.i758, i32 12
  %iter.sroa.0.0.ptr.i53.i84817798.4 = getelementptr inbounds nuw i8, ptr %scratch.i758, i32 16
  %iter.sroa.0.0.ptr.i53.i84817798.5 = getelementptr inbounds nuw i8, ptr %scratch.i758, i32 20
  %iter.sroa.0.0.ptr.i53.i84817798.6 = getelementptr inbounds nuw i8, ptr %scratch.i758, i32 24
  %iter.sroa.0.0.ptr.i53.i84817798.7 = getelementptr inbounds nuw i8, ptr %scratch.i758, i32 28
  %iter.sroa.0.0.ptr.i.i94317810.1 = getelementptr inbounds nuw i8, ptr %scratch.i758, i32 4
  %iter.sroa.0.0.ptr.i.i94317810.2 = getelementptr inbounds nuw i8, ptr %scratch.i758, i32 8
  %iter.sroa.0.0.ptr.i.i94317810.3 = getelementptr inbounds nuw i8, ptr %scratch.i758, i32 12
  %iter.sroa.0.0.ptr.i.i94317810.4 = getelementptr inbounds nuw i8, ptr %scratch.i758, i32 16
  %iter.sroa.0.0.ptr.i.i94317810.5 = getelementptr inbounds nuw i8, ptr %scratch.i758, i32 20
  %iter.sroa.0.0.ptr.i.i94317810.6 = getelementptr inbounds nuw i8, ptr %scratch.i758, i32 24
  %iter.sroa.0.0.ptr.i.i94317810.7 = getelementptr inbounds nuw i8, ptr %scratch.i758, i32 28
  br label %bb37.i784, !dbg !5208

bb13.i778.loopexit.loopexit:                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5892
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %502, ptr %149, align 16, !dbg !5264
  store <4 x float> %592, ptr %166, align 16, !dbg !5279
  br label %bb13.i778.loopexit, !dbg !5208

bb13.i778.loopexit:                               ; preds = %bb13.i778.loopexit.loopexit, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i789
  %ring_cursor.sroa.0.1.i792.lcssa = phi i32 [ %ring_cursor.sroa.0.0.i78117820, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i789 ], [ %spec.store.select12.i1014, %bb13.i778.loopexit.loopexit ], !dbg !5281
  %main_cursor.sroa.0.1.i793.lcssa = phi i32 [ %main_cursor.sroa.0.0.i78217821, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i789 ], [ %spec.store.select11.i1012, %bb13.i778.loopexit.loopexit ], !dbg !5282
  %_111.not.i783 = icmp eq i32 %174, 0, !dbg !5208
  %indvars.iv.next = add i32 %indvars.iv, -32, !dbg !5208
  br i1 %_111.not.i783, label %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb37.i784, !dbg !5208

bb37.i784:                                        ; preds = %bb37.i784.lr.ph, %bb13.i778.loopexit
  %indvars.iv = phi i32 [ %frames, %bb37.i784.lr.ph ], [ %indvars.iv.next, %bb13.i778.loopexit ]
  %main_cursor.sroa.0.0.i78217821 = phi i32 [ %_35.i769, %bb37.i784.lr.ph ], [ %main_cursor.sroa.0.1.i793.lcssa, %bb13.i778.loopexit ]
  %ring_cursor.sroa.0.0.i78117820 = phi i32 [ %_37.i770, %bb37.i784.lr.ph ], [ %ring_cursor.sroa.0.1.i792.lcssa, %bb13.i778.loopexit ]
  %iter2.sroa.0.0.i78017819 = phi i32 [ %yield_count.sroa.0.0.i, %bb37.i784.lr.ph ], [ %174, %bb13.i778.loopexit ]
  %iter.sroa.0.0.i77917818 = phi i32 [ 0, %bb37.i784.lr.ph ], [ %173, %bb13.i778.loopexit ]
  %172 = call i32 @llvm.umax.i32(i32 %indvars.iv, i32 1), !dbg !5283
  %umax20489 = call i32 @llvm.umin.i32(i32 %172, i32 32), !dbg !5283
  %173 = add i32 %iter.sroa.0.0.i77917818, 32, !dbg !5283
  %174 = add nsw i32 %iter2.sroa.0.0.i78017819, -1, !dbg !5287
  %history.i141.i.sroa.0.0.copyload = load <4 x i32>, ptr %hot_left.i761, align 16, !dbg !5288
  %history.i141.i.sroa.7.0.copyload = load <4 x i32>, ptr %history.i141.i.sroa.7.0.hot_left.i761.sroa_idx, align 16, !dbg !5288
  %history.i141.i.sroa.10.0.copyload = load <4 x i32>, ptr %history.i141.i.sroa.10.0.hot_left.i761.sroa_idx, align 16, !dbg !5288
  %history.i141.i.sroa.13.0.copyload = load <4 x i32>, ptr %history.i141.i.sroa.13.0.hot_left.i761.sroa_idx, align 16, !dbg !5288
  %history.i141.i.sroa.16.0.copyload = load <4 x i32>, ptr %history.i141.i.sroa.16.0.hot_left.i761.sroa_idx, align 16, !dbg !5288
  %history.i141.i.sroa.19.0.copyload = load <4 x i32>, ptr %history.i141.i.sroa.19.0.hot_left.i761.sroa_idx, align 16, !dbg !5288
  %history.i141.i.sroa.22.0.copyload = load <4 x i32>, ptr %history.i141.i.sroa.22.0.hot_left.i761.sroa_idx, align 16, !dbg !5288
  %history.i141.i.sroa.26.0.copyload = load <4 x i32>, ptr %history.i141.i.sroa.26.0.hot_left.i761.sroa_idx, align 16, !dbg !5288
  %history.i141.i.sroa.29.0.copyload = load <4 x i32>, ptr %history.i141.i.sroa.29.0.hot_left.i761.sroa_idx, align 16, !dbg !5288
  %history.i141.i.sroa.32.0.copyload = load <4 x i32>, ptr %history.i141.i.sroa.32.0.hot_left.i761.sroa_idx, align 16, !dbg !5288
  %history.i141.i.sroa.35.0.copyload = load <4 x i32>, ptr %history.i141.i.sroa.35.0.hot_left.i761.sroa_idx, align 16, !dbg !5288
  %history.i141.i.sroa.38.0.copyload = load <4 x i32>, ptr %history.i141.i.sroa.38.0.hot_left.i761.sroa_idx, align 16, !dbg !5288
  %_20.i144.i17736.not = icmp eq i32 %frames, %iter.sroa.0.0.i77917818, !dbg !5292
  br i1 %_20.i144.i17736.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit349.i, label %bb5.i145.i.lr.ph, !dbg !5302

bb5.i145.i.lr.ph:                                 ; preds = %bb37.i784
  %_11.i.i.i166.i15174 = load <4 x float>, ptr %_31, align 16
  %_14.i.i.i169.i15175 = load <4 x float>, ptr %90, align 16
  %_17.i.i.i172.i15176 = load <4 x float>, ptr %91, align 16
  %_20.i.i.i175.i15177 = load <4 x float>, ptr %92, align 16
  %_25.i.i.i180.i15178 = load <4 x float>, ptr %row1.i.i.i178.i, align 16
  %_28.i.i.i183.i15179 = load <4 x float>, ptr %93, align 16
  %_31.i.i.i186.i15180 = load <4 x float>, ptr %94, align 16
  %_34.i.i.i189.i15181 = load <4 x float>, ptr %95, align 16
  %_39.i.i.i194.i15182 = load <4 x float>, ptr %row3.i.i.i192.i, align 16
  %_42.i.i.i197.i15183 = load <4 x float>, ptr %96, align 16
  %_45.i.i.i200.i15184 = load <4 x float>, ptr %97, align 16
  %_48.i.i.i203.i15185 = load <4 x float>, ptr %98, align 16
  %_53.i.i.i208.i15186 = load <4 x float>, ptr %row5.i.i.i206.i, align 16
  %_56.i.i.i211.i15187 = load <4 x float>, ptr %99, align 16
  %_59.i.i.i214.i15188 = load <4 x float>, ptr %100, align 16
  %_62.i.i.i217.i15189 = load <4 x float>, ptr %101, align 16
  %_67.i.i.i222.i15190 = load <4 x float>, ptr %row7.i.i.i220.i, align 16
  %_70.i.i.i225.i15191 = load <4 x float>, ptr %102, align 16
  %_73.i.i.i228.i15192 = load <4 x float>, ptr %103, align 16
  %_76.i.i.i231.i15193 = load <4 x float>, ptr %104, align 16
  %_81.i.i.i236.i15194 = load <4 x float>, ptr %row9.i.i.i234.i, align 16
  %_84.i.i.i239.i15195 = load <4 x float>, ptr %105, align 16
  %_87.i.i.i242.i15196 = load <4 x float>, ptr %106, align 16
  %_90.i.i.i245.i15197 = load <4 x float>, ptr %107, align 16
  %_95.i.i.i250.i15198 = load <4 x float>, ptr %row11.i.i.i248.i, align 16
  %_98.i.i.i253.i15199 = load <4 x float>, ptr %108, align 16
  %_101.i.i.i256.i15200 = load <4 x float>, ptr %109, align 16
  %_104.i.i.i259.i15201 = load <4 x float>, ptr %110, align 16
  %_109.i.i.i264.i15202 = load <4 x float>, ptr %row13.i.i.i262.i, align 16
  %_112.i.i.i267.i15203 = load <4 x float>, ptr %111, align 16
  %_115.i.i.i270.i15204 = load <4 x float>, ptr %112, align 16
  %_118.i.i.i273.i15205 = load <4 x float>, ptr %113, align 16
  %_123.i.i.i278.i15206 = load <4 x float>, ptr %row15.i.i.i276.i, align 16
  %_126.i.i.i281.i15207 = load <4 x float>, ptr %114, align 16
  %_129.i.i.i284.i15208 = load <4 x float>, ptr %115, align 16
  %_132.i.i.i287.i15209 = load <4 x float>, ptr %116, align 16
  %_137.i.i.i292.i15210 = load <4 x float>, ptr %row17.i.i.i290.i, align 16
  %_140.i.i.i295.i15211 = load <4 x float>, ptr %117, align 16
  %_143.i.i.i298.i15212 = load <4 x float>, ptr %118, align 16
  %_146.i.i.i301.i15213 = load <4 x float>, ptr %119, align 16
  %_151.i.i.i306.i15214 = load <4 x float>, ptr %row19.i.i.i304.i, align 16
  %_154.i.i.i309.i15215 = load <4 x float>, ptr %120, align 16
  %_157.i.i.i312.i15216 = load <4 x float>, ptr %121, align 16
  %_160.i.i.i315.i15217 = load <4 x float>, ptr %122, align 16
  %_165.i.i.i320.i15218 = load <4 x float>, ptr %row21.i.i.i318.i, align 16
  %_168.i.i.i323.i15219 = load <4 x float>, ptr %123, align 16
  %_171.i.i.i326.i15220 = load <4 x float>, ptr %124, align 16
  %_174.i.i.i329.i15221 = load <4 x float>, ptr %125, align 16
  br label %bb5.i145.i, !dbg !5302

bb5.i145.i:                                       ; preds = %bb5.i145.i.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit
  %iter.sroa.0.0.i143.i17748 = phi i32 [ 0, %bb5.i145.i.lr.ph ], [ %175, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %history.i141.i.sroa.35.017747 = phi <4 x i32> [ %history.i141.i.sroa.35.0.copyload, %bb5.i145.i.lr.ph ], [ %history.i141.i.sroa.32.017746, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %history.i141.i.sroa.32.017746 = phi <4 x i32> [ %history.i141.i.sroa.32.0.copyload, %bb5.i145.i.lr.ph ], [ %history.i141.i.sroa.29.017745, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %history.i141.i.sroa.29.017745 = phi <4 x i32> [ %history.i141.i.sroa.29.0.copyload, %bb5.i145.i.lr.ph ], [ %history.i141.i.sroa.26.017744, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %history.i141.i.sroa.26.017744 = phi <4 x i32> [ %history.i141.i.sroa.26.0.copyload, %bb5.i145.i.lr.ph ], [ %history.i141.i.sroa.22.017743, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %history.i141.i.sroa.22.017743 = phi <4 x i32> [ %history.i141.i.sroa.22.0.copyload, %bb5.i145.i.lr.ph ], [ %history.i141.i.sroa.19.017742, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %history.i141.i.sroa.19.017742 = phi <4 x i32> [ %history.i141.i.sroa.19.0.copyload, %bb5.i145.i.lr.ph ], [ %history.i141.i.sroa.16.017741, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %history.i141.i.sroa.16.017741 = phi <4 x i32> [ %history.i141.i.sroa.16.0.copyload, %bb5.i145.i.lr.ph ], [ %history.i141.i.sroa.13.017740, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %history.i141.i.sroa.13.017740 = phi <4 x i32> [ %history.i141.i.sroa.13.0.copyload, %bb5.i145.i.lr.ph ], [ %history.i141.i.sroa.10.017739, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %history.i141.i.sroa.10.017739 = phi <4 x i32> [ %history.i141.i.sroa.10.0.copyload, %bb5.i145.i.lr.ph ], [ %history.i141.i.sroa.7.017738, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %history.i141.i.sroa.7.017738 = phi <4 x i32> [ %history.i141.i.sroa.7.0.copyload, %bb5.i145.i.lr.ph ], [ %history.i141.i.sroa.0.017737, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %history.i141.i.sroa.0.017737 = phi <4 x i32> [ %history.i141.i.sroa.0.0.copyload, %bb5.i145.i.lr.ph ], [ %lanes.i5111.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %175 = add nuw nsw i32 %iter.sroa.0.0.i143.i17748, 1, !dbg !5303
  %_11.i146.i = add nuw nsw i32 %iter.sroa.0.0.i143.i17748, %iter.sroa.0.0.i77917818, !dbg !5309
  %base.i147.i = shl i32 %_11.i146.i, 2, !dbg !5309
  %_24.i148.i = icmp ugt i32 %base.i147.i, %left_io.1, !dbg !5311
  br i1 %_24.i148.i, label %bb7.i348.i, label %bb8.i149.i, !dbg !5311, !prof !902

bb8.i149.i:                                       ; preds = %bb5.i145.i
  %_27.i150.i = sub nuw nsw i32 %left_io.1, %base.i147.i, !dbg !5317
  %_8.i5114 = icmp samesign ugt i32 %_27.i150.i, 3, !dbg !5318
  br i1 %_8.i5114, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit, label %bb2.i5115, !dbg !5318, !prof !1153

bb2.i5115:                                        ; preds = %bb8.i149.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_27.i150.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !5323, !noalias !5324
  unreachable, !dbg !5323

bb7.i348.i:                                       ; preds = %bb5.i145.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i147.i, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #33, !dbg !5331, !noalias !5332
  unreachable, !dbg !5331

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit: ; preds = %bb8.i149.i
  %_31.i151.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %base.i147.i, !dbg !5333
  %lanes.i5111.sroa.0.0.copyload = load <4 x i32>, ptr %_31.i151.i, align 4, !dbg !5338, !alias.scope !5342, !noalias !5346
  %176 = bitcast <4 x i32> %history.i141.i.sroa.19.017742 to <4 x float>, !dbg !5348
  %177 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %176), !dbg !5355
  %178 = bitcast <4 x i32> %lanes.i5111.sroa.0.0.copyload to <4 x float>, !dbg !5356
  %179 = fmul <4 x float> %_11.i.i.i166.i15174, %178, !dbg !5373
  %180 = fadd <4 x float> %179, zeroinitializer, !dbg !5374
  %181 = fmul <4 x float> %_14.i.i.i169.i15175, %178, !dbg !5382
  %182 = fadd <4 x float> %181, zeroinitializer, !dbg !5386
  %183 = fmul <4 x float> %_17.i.i.i172.i15176, %178, !dbg !5390
  %184 = fadd <4 x float> %183, zeroinitializer, !dbg !5394
  %185 = fmul <4 x float> %_20.i.i.i175.i15177, %178, !dbg !5398
  %186 = fadd <4 x float> %185, zeroinitializer, !dbg !5402
  %187 = bitcast <4 x i32> %history.i141.i.sroa.0.017737 to <4 x float>, !dbg !5406
  %188 = fmul <4 x float> %_25.i.i.i180.i15178, %187, !dbg !5412
  %189 = fadd <4 x float> %180, %188, !dbg !5413
  %190 = fmul <4 x float> %_28.i.i.i183.i15179, %187, !dbg !5417
  %191 = fadd <4 x float> %182, %190, !dbg !5421
  %192 = fmul <4 x float> %_31.i.i.i186.i15180, %187, !dbg !5425
  %193 = fadd <4 x float> %184, %192, !dbg !5429
  %194 = fmul <4 x float> %_34.i.i.i189.i15181, %187, !dbg !5433
  %195 = fadd <4 x float> %186, %194, !dbg !5437
  %196 = bitcast <4 x i32> %history.i141.i.sroa.7.017738 to <4 x float>, !dbg !5441
  %197 = fmul <4 x float> %_39.i.i.i194.i15182, %196, !dbg !5447
  %198 = fadd <4 x float> %189, %197, !dbg !5448
  %199 = fmul <4 x float> %_42.i.i.i197.i15183, %196, !dbg !5452
  %200 = fadd <4 x float> %191, %199, !dbg !5456
  %201 = fmul <4 x float> %_45.i.i.i200.i15184, %196, !dbg !5460
  %202 = fadd <4 x float> %193, %201, !dbg !5464
  %203 = fmul <4 x float> %_48.i.i.i203.i15185, %196, !dbg !5468
  %204 = fadd <4 x float> %195, %203, !dbg !5472
  %205 = bitcast <4 x i32> %history.i141.i.sroa.10.017739 to <4 x float>, !dbg !5476
  %206 = fmul <4 x float> %_53.i.i.i208.i15186, %205, !dbg !5482
  %207 = fadd <4 x float> %198, %206, !dbg !5483
  %208 = fmul <4 x float> %_56.i.i.i211.i15187, %205, !dbg !5487
  %209 = fadd <4 x float> %200, %208, !dbg !5491
  %210 = fmul <4 x float> %_59.i.i.i214.i15188, %205, !dbg !5495
  %211 = fadd <4 x float> %202, %210, !dbg !5499
  %212 = fmul <4 x float> %_62.i.i.i217.i15189, %205, !dbg !5503
  %213 = fadd <4 x float> %204, %212, !dbg !5507
  %214 = bitcast <4 x i32> %history.i141.i.sroa.13.017740 to <4 x float>, !dbg !5511
  %215 = fmul <4 x float> %_67.i.i.i222.i15190, %214, !dbg !5517
  %216 = fadd <4 x float> %207, %215, !dbg !5518
  %217 = fmul <4 x float> %_70.i.i.i225.i15191, %214, !dbg !5522
  %218 = fadd <4 x float> %209, %217, !dbg !5526
  %219 = fmul <4 x float> %_73.i.i.i228.i15192, %214, !dbg !5530
  %220 = fadd <4 x float> %211, %219, !dbg !5534
  %221 = fmul <4 x float> %_76.i.i.i231.i15193, %214, !dbg !5538
  %222 = fadd <4 x float> %213, %221, !dbg !5542
  %223 = bitcast <4 x i32> %history.i141.i.sroa.16.017741 to <4 x float>, !dbg !5546
  %224 = fmul <4 x float> %_81.i.i.i236.i15194, %223, !dbg !5552
  %225 = fadd <4 x float> %216, %224, !dbg !5553
  %226 = fmul <4 x float> %_84.i.i.i239.i15195, %223, !dbg !5557
  %227 = fadd <4 x float> %218, %226, !dbg !5561
  %228 = fmul <4 x float> %_87.i.i.i242.i15196, %223, !dbg !5565
  %229 = fadd <4 x float> %220, %228, !dbg !5569
  %230 = fmul <4 x float> %_90.i.i.i245.i15197, %223, !dbg !5573
  %231 = fadd <4 x float> %222, %230, !dbg !5577
  %232 = fmul <4 x float> %_95.i.i.i250.i15198, %176, !dbg !5581
  %233 = fadd <4 x float> %225, %232, !dbg !5587
  %234 = fmul <4 x float> %_98.i.i.i253.i15199, %176, !dbg !5591
  %235 = fadd <4 x float> %227, %234, !dbg !5595
  %236 = fmul <4 x float> %_101.i.i.i256.i15200, %176, !dbg !5599
  %237 = fadd <4 x float> %229, %236, !dbg !5603
  %238 = fmul <4 x float> %_104.i.i.i259.i15201, %176, !dbg !5607
  %239 = fadd <4 x float> %231, %238, !dbg !5611
  %240 = bitcast <4 x i32> %history.i141.i.sroa.22.017743 to <4 x float>, !dbg !5615
  %241 = fmul <4 x float> %_109.i.i.i264.i15202, %240, !dbg !5621
  %242 = fadd <4 x float> %233, %241, !dbg !5622
  %243 = fmul <4 x float> %_112.i.i.i267.i15203, %240, !dbg !5626
  %244 = fadd <4 x float> %235, %243, !dbg !5630
  %245 = fmul <4 x float> %_115.i.i.i270.i15204, %240, !dbg !5634
  %246 = fadd <4 x float> %237, %245, !dbg !5638
  %247 = fmul <4 x float> %_118.i.i.i273.i15205, %240, !dbg !5642
  %248 = fadd <4 x float> %239, %247, !dbg !5646
  %249 = bitcast <4 x i32> %history.i141.i.sroa.26.017744 to <4 x float>, !dbg !5650
  %250 = fmul <4 x float> %_123.i.i.i278.i15206, %249, !dbg !5656
  %251 = fadd <4 x float> %242, %250, !dbg !5657
  %252 = fmul <4 x float> %_126.i.i.i281.i15207, %249, !dbg !5661
  %253 = fadd <4 x float> %244, %252, !dbg !5665
  %254 = fmul <4 x float> %_129.i.i.i284.i15208, %249, !dbg !5669
  %255 = fadd <4 x float> %246, %254, !dbg !5673
  %256 = fmul <4 x float> %_132.i.i.i287.i15209, %249, !dbg !5677
  %257 = fadd <4 x float> %248, %256, !dbg !5681
  %258 = bitcast <4 x i32> %history.i141.i.sroa.29.017745 to <4 x float>, !dbg !5685
  %259 = fmul <4 x float> %_137.i.i.i292.i15210, %258, !dbg !5691
  %260 = fadd <4 x float> %251, %259, !dbg !5692
  %261 = fmul <4 x float> %_140.i.i.i295.i15211, %258, !dbg !5696
  %262 = fadd <4 x float> %253, %261, !dbg !5700
  %263 = fmul <4 x float> %_143.i.i.i298.i15212, %258, !dbg !5704
  %264 = fadd <4 x float> %255, %263, !dbg !5708
  %265 = fmul <4 x float> %_146.i.i.i301.i15213, %258, !dbg !5712
  %266 = fadd <4 x float> %257, %265, !dbg !5716
  %267 = bitcast <4 x i32> %history.i141.i.sroa.32.017746 to <4 x float>, !dbg !5720
  %268 = fmul <4 x float> %_151.i.i.i306.i15214, %267, !dbg !5726
  %269 = fadd <4 x float> %260, %268, !dbg !5727
  %270 = fmul <4 x float> %_154.i.i.i309.i15215, %267, !dbg !5731
  %271 = fadd <4 x float> %262, %270, !dbg !5735
  %272 = fmul <4 x float> %_157.i.i.i312.i15216, %267, !dbg !5739
  %273 = fadd <4 x float> %264, %272, !dbg !5743
  %274 = fmul <4 x float> %_160.i.i.i315.i15217, %267, !dbg !5747
  %275 = fadd <4 x float> %266, %274, !dbg !5751
  %276 = bitcast <4 x i32> %history.i141.i.sroa.35.017747 to <4 x float>, !dbg !5755
  %277 = fmul <4 x float> %_165.i.i.i320.i15218, %276, !dbg !5761
  %278 = fadd <4 x float> %269, %277, !dbg !5762
  %279 = fmul <4 x float> %_168.i.i.i323.i15219, %276, !dbg !5766
  %280 = fadd <4 x float> %271, %279, !dbg !5770
  %281 = fmul <4 x float> %_171.i.i.i326.i15220, %276, !dbg !5774
  %282 = fadd <4 x float> %273, %281, !dbg !5778
  %283 = fmul <4 x float> %_174.i.i.i329.i15221, %276, !dbg !5782
  %284 = fadd <4 x float> %275, %283, !dbg !5786
  %285 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %278), !dbg !5790
  %286 = fcmp olt <4 x float> %285, %177, !dbg !5796
  %287 = select <4 x i1> %286, <4 x float> %177, <4 x float> %285, !dbg !5803
  %288 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %280), !dbg !5790
  %289 = fcmp olt <4 x float> %288, %287, !dbg !5796
  %290 = select <4 x i1> %289, <4 x float> %287, <4 x float> %288, !dbg !5803
  %291 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %282), !dbg !5790
  %292 = fcmp olt <4 x float> %291, %290, !dbg !5796
  %293 = select <4 x i1> %292, <4 x float> %290, <4 x float> %291, !dbg !5803
  %294 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %284), !dbg !5790
  %295 = fcmp olt <4 x float> %294, %293, !dbg !5796
  %296 = select <4 x i1> %295, <4 x float> %293, <4 x float> %294, !dbg !5803
  %_39.i342.i.idx = shl i32 %iter.sroa.0.0.i143.i17748, 4, !dbg !5804
  %_39.i342.i = getelementptr inbounds nuw i8, ptr %peaks_left.i757, i32 %_39.i342.i.idx, !dbg !5804
  store <4 x float> %296, ptr %_39.i342.i, align 4, !dbg !5818, !alias.scope !5823, !noalias !5827
  %exitcond.not = icmp eq i32 %175, %umax20489, !dbg !5292
  br i1 %exitcond.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit349.i, label %bb5.i145.i, !dbg !5302

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit349.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit, %bb37.i784
  %history.i141.i.sroa.0.0.lcssa = phi <4 x i32> [ %history.i141.i.sroa.0.0.copyload, %bb37.i784 ], [ %lanes.i5111.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !5831
  %history.i141.i.sroa.7.0.lcssa = phi <4 x i32> [ %history.i141.i.sroa.7.0.copyload, %bb37.i784 ], [ %history.i141.i.sroa.0.017737, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !5831
  %history.i141.i.sroa.10.0.lcssa = phi <4 x i32> [ %history.i141.i.sroa.10.0.copyload, %bb37.i784 ], [ %history.i141.i.sroa.7.017738, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !5831
  %history.i141.i.sroa.13.0.lcssa = phi <4 x i32> [ %history.i141.i.sroa.13.0.copyload, %bb37.i784 ], [ %history.i141.i.sroa.10.017739, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !5831
  %history.i141.i.sroa.16.0.lcssa = phi <4 x i32> [ %history.i141.i.sroa.16.0.copyload, %bb37.i784 ], [ %history.i141.i.sroa.13.017740, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !5831
  %history.i141.i.sroa.19.0.lcssa = phi <4 x i32> [ %history.i141.i.sroa.19.0.copyload, %bb37.i784 ], [ %history.i141.i.sroa.16.017741, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !5831
  %history.i141.i.sroa.22.0.lcssa = phi <4 x i32> [ %history.i141.i.sroa.22.0.copyload, %bb37.i784 ], [ %history.i141.i.sroa.19.017742, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !5831
  %history.i141.i.sroa.26.0.lcssa = phi <4 x i32> [ %history.i141.i.sroa.26.0.copyload, %bb37.i784 ], [ %history.i141.i.sroa.22.017743, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !5831
  %history.i141.i.sroa.29.0.lcssa = phi <4 x i32> [ %history.i141.i.sroa.29.0.copyload, %bb37.i784 ], [ %history.i141.i.sroa.26.017744, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !5831
  %history.i141.i.sroa.32.0.lcssa = phi <4 x i32> [ %history.i141.i.sroa.32.0.copyload, %bb37.i784 ], [ %history.i141.i.sroa.29.017745, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !5831
  %history.i141.i.sroa.35.0.lcssa = phi <4 x i32> [ %history.i141.i.sroa.35.0.copyload, %bb37.i784 ], [ %history.i141.i.sroa.32.017746, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !5831
  %history.i141.i.sroa.38.0.lcssa = phi <4 x i32> [ %history.i141.i.sroa.38.0.copyload, %bb37.i784 ], [ %history.i141.i.sroa.35.017747, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !5831
  store <4 x i32> %history.i141.i.sroa.0.0.lcssa, ptr %hot_left.i761, align 16, !dbg !5832
  store <4 x i32> %history.i141.i.sroa.7.0.lcssa, ptr %history.i141.i.sroa.7.0.hot_left.i761.sroa_idx, align 16, !dbg !5832
  store <4 x i32> %history.i141.i.sroa.10.0.lcssa, ptr %history.i141.i.sroa.10.0.hot_left.i761.sroa_idx, align 16, !dbg !5832
  store <4 x i32> %history.i141.i.sroa.13.0.lcssa, ptr %history.i141.i.sroa.13.0.hot_left.i761.sroa_idx, align 16, !dbg !5832
  store <4 x i32> %history.i141.i.sroa.16.0.lcssa, ptr %history.i141.i.sroa.16.0.hot_left.i761.sroa_idx, align 16, !dbg !5832
  store <4 x i32> %history.i141.i.sroa.19.0.lcssa, ptr %history.i141.i.sroa.19.0.hot_left.i761.sroa_idx, align 16, !dbg !5832
  store <4 x i32> %history.i141.i.sroa.22.0.lcssa, ptr %history.i141.i.sroa.22.0.hot_left.i761.sroa_idx, align 16, !dbg !5832
  store <4 x i32> %history.i141.i.sroa.26.0.lcssa, ptr %history.i141.i.sroa.26.0.hot_left.i761.sroa_idx, align 16, !dbg !5832
  store <4 x i32> %history.i141.i.sroa.29.0.lcssa, ptr %history.i141.i.sroa.29.0.hot_left.i761.sroa_idx, align 16, !dbg !5832
  store <4 x i32> %history.i141.i.sroa.32.0.lcssa, ptr %history.i141.i.sroa.32.0.hot_left.i761.sroa_idx, align 16, !dbg !5832
  store <4 x i32> %history.i141.i.sroa.35.0.lcssa, ptr %history.i141.i.sroa.35.0.hot_left.i761.sroa_idx, align 16, !dbg !5832
  store <4 x i32> %history.i141.i.sroa.38.0.lcssa, ptr %history.i141.i.sroa.38.0.hot_left.i761.sroa_idx, align 16, !dbg !5832
  %history.i.i751.sroa.0.0.copyload = load <4 x i32>, ptr %hot_right.i760, align 16, !dbg !5833
  %history.i.i751.sroa.7.0.copyload = load <4 x i32>, ptr %history.i.i751.sroa.7.0.hot_right.i760.sroa_idx, align 16, !dbg !5833
  %history.i.i751.sroa.10.0.copyload = load <4 x i32>, ptr %history.i.i751.sroa.10.0.hot_right.i760.sroa_idx, align 16, !dbg !5833
  %history.i.i751.sroa.13.0.copyload = load <4 x i32>, ptr %history.i.i751.sroa.13.0.hot_right.i760.sroa_idx, align 16, !dbg !5833
  %history.i.i751.sroa.16.0.copyload = load <4 x i32>, ptr %history.i.i751.sroa.16.0.hot_right.i760.sroa_idx, align 16, !dbg !5833
  %history.i.i751.sroa.19.0.copyload = load <4 x i32>, ptr %history.i.i751.sroa.19.0.hot_right.i760.sroa_idx, align 16, !dbg !5833
  %history.i.i751.sroa.22.0.copyload = load <4 x i32>, ptr %history.i.i751.sroa.22.0.hot_right.i760.sroa_idx, align 16, !dbg !5833
  %history.i.i751.sroa.26.0.copyload = load <4 x i32>, ptr %history.i.i751.sroa.26.0.hot_right.i760.sroa_idx, align 16, !dbg !5833
  %history.i.i751.sroa.29.0.copyload = load <4 x i32>, ptr %history.i.i751.sroa.29.0.hot_right.i760.sroa_idx, align 16, !dbg !5833
  %history.i.i751.sroa.32.0.copyload = load <4 x i32>, ptr %history.i.i751.sroa.32.0.hot_right.i760.sroa_idx, align 16, !dbg !5833
  %history.i.i751.sroa.35.0.copyload = load <4 x i32>, ptr %history.i.i751.sroa.35.0.hot_right.i760.sroa_idx, align 16, !dbg !5833
  %history.i.i751.sroa.38.0.copyload = load <4 x i32>, ptr %history.i.i751.sroa.38.0.hot_right.i760.sroa_idx, align 16, !dbg !5833
  br i1 %_20.i144.i17736.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i789, label %bb5.i.i1028.lr.ph, !dbg !5835

bb5.i.i1028.lr.ph:                                ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit349.i
  %_11.i.i.i.i104715121 = load <4 x float>, ptr %_31, align 16
  %_14.i.i.i.i105015122 = load <4 x float>, ptr %90, align 16
  %_17.i.i.i.i105315123 = load <4 x float>, ptr %91, align 16
  %_20.i.i.i.i105615124 = load <4 x float>, ptr %92, align 16
  %_25.i.i.i.i106115125 = load <4 x float>, ptr %row1.i.i.i178.i, align 16
  %_28.i.i.i.i106415126 = load <4 x float>, ptr %93, align 16
  %_31.i.i.i.i106715127 = load <4 x float>, ptr %94, align 16
  %_34.i.i.i.i107015128 = load <4 x float>, ptr %95, align 16
  %_39.i.i.i.i107515129 = load <4 x float>, ptr %row3.i.i.i192.i, align 16
  %_42.i.i.i.i107815130 = load <4 x float>, ptr %96, align 16
  %_45.i.i.i.i108115131 = load <4 x float>, ptr %97, align 16
  %_48.i.i.i.i108415132 = load <4 x float>, ptr %98, align 16
  %_53.i.i.i.i108915133 = load <4 x float>, ptr %row5.i.i.i206.i, align 16
  %_56.i.i.i.i109215134 = load <4 x float>, ptr %99, align 16
  %_59.i.i.i.i109515135 = load <4 x float>, ptr %100, align 16
  %_62.i.i.i.i109815136 = load <4 x float>, ptr %101, align 16
  %_67.i.i.i.i110315137 = load <4 x float>, ptr %row7.i.i.i220.i, align 16
  %_70.i.i.i.i110615138 = load <4 x float>, ptr %102, align 16
  %_73.i.i.i.i110915139 = load <4 x float>, ptr %103, align 16
  %_76.i.i.i.i111215140 = load <4 x float>, ptr %104, align 16
  %_81.i.i.i.i111715141 = load <4 x float>, ptr %row9.i.i.i234.i, align 16
  %_84.i.i.i.i112015142 = load <4 x float>, ptr %105, align 16
  %_87.i.i.i.i112315143 = load <4 x float>, ptr %106, align 16
  %_90.i.i.i.i112615144 = load <4 x float>, ptr %107, align 16
  %_95.i.i.i.i113115145 = load <4 x float>, ptr %row11.i.i.i248.i, align 16
  %_98.i.i.i.i113415146 = load <4 x float>, ptr %108, align 16
  %_101.i.i.i.i113715147 = load <4 x float>, ptr %109, align 16
  %_104.i.i.i.i114015148 = load <4 x float>, ptr %110, align 16
  %_109.i.i.i.i114515149 = load <4 x float>, ptr %row13.i.i.i262.i, align 16
  %_112.i.i.i.i114815150 = load <4 x float>, ptr %111, align 16
  %_115.i.i.i.i115115151 = load <4 x float>, ptr %112, align 16
  %_118.i.i.i.i115415152 = load <4 x float>, ptr %113, align 16
  %_123.i.i.i.i115915153 = load <4 x float>, ptr %row15.i.i.i276.i, align 16
  %_126.i.i.i.i116215154 = load <4 x float>, ptr %114, align 16
  %_129.i.i.i.i116515155 = load <4 x float>, ptr %115, align 16
  %_132.i.i.i.i116815156 = load <4 x float>, ptr %116, align 16
  %_137.i.i.i.i117315157 = load <4 x float>, ptr %row17.i.i.i290.i, align 16
  %_140.i.i.i.i117615158 = load <4 x float>, ptr %117, align 16
  %_143.i.i.i.i117915159 = load <4 x float>, ptr %118, align 16
  %_146.i.i.i.i118215160 = load <4 x float>, ptr %119, align 16
  %_151.i.i.i.i118715161 = load <4 x float>, ptr %row19.i.i.i304.i, align 16
  %_154.i.i.i.i119015162 = load <4 x float>, ptr %120, align 16
  %_157.i.i.i.i119315163 = load <4 x float>, ptr %121, align 16
  %_160.i.i.i.i119615164 = load <4 x float>, ptr %122, align 16
  %_165.i.i.i.i120115165 = load <4 x float>, ptr %row21.i.i.i318.i, align 16
  %_168.i.i.i.i120415166 = load <4 x float>, ptr %123, align 16
  %_171.i.i.i.i120715167 = load <4 x float>, ptr %124, align 16
  %_174.i.i.i.i121015168 = load <4 x float>, ptr %125, align 16
  br label %bb5.i.i1028, !dbg !5835

bb5.i.i1028:                                      ; preds = %bb5.i.i1028.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887
  %iter.sroa.0.0.i.i78717774 = phi i32 [ 0, %bb5.i.i1028.lr.ph ], [ %297, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887 ]
  %history.i.i751.sroa.35.017773 = phi <4 x i32> [ %history.i.i751.sroa.35.0.copyload, %bb5.i.i1028.lr.ph ], [ %history.i.i751.sroa.32.017772, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887 ]
  %history.i.i751.sroa.32.017772 = phi <4 x i32> [ %history.i.i751.sroa.32.0.copyload, %bb5.i.i1028.lr.ph ], [ %history.i.i751.sroa.29.017771, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887 ]
  %history.i.i751.sroa.29.017771 = phi <4 x i32> [ %history.i.i751.sroa.29.0.copyload, %bb5.i.i1028.lr.ph ], [ %history.i.i751.sroa.26.017770, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887 ]
  %history.i.i751.sroa.26.017770 = phi <4 x i32> [ %history.i.i751.sroa.26.0.copyload, %bb5.i.i1028.lr.ph ], [ %history.i.i751.sroa.22.017769, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887 ]
  %history.i.i751.sroa.22.017769 = phi <4 x i32> [ %history.i.i751.sroa.22.0.copyload, %bb5.i.i1028.lr.ph ], [ %history.i.i751.sroa.19.017768, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887 ]
  %history.i.i751.sroa.19.017768 = phi <4 x i32> [ %history.i.i751.sroa.19.0.copyload, %bb5.i.i1028.lr.ph ], [ %history.i.i751.sroa.16.017767, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887 ]
  %history.i.i751.sroa.16.017767 = phi <4 x i32> [ %history.i.i751.sroa.16.0.copyload, %bb5.i.i1028.lr.ph ], [ %history.i.i751.sroa.13.017766, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887 ]
  %history.i.i751.sroa.13.017766 = phi <4 x i32> [ %history.i.i751.sroa.13.0.copyload, %bb5.i.i1028.lr.ph ], [ %history.i.i751.sroa.10.017765, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887 ]
  %history.i.i751.sroa.10.017765 = phi <4 x i32> [ %history.i.i751.sroa.10.0.copyload, %bb5.i.i1028.lr.ph ], [ %history.i.i751.sroa.7.017764, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887 ]
  %history.i.i751.sroa.7.017764 = phi <4 x i32> [ %history.i.i751.sroa.7.0.copyload, %bb5.i.i1028.lr.ph ], [ %history.i.i751.sroa.0.017763, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887 ]
  %history.i.i751.sroa.0.017763 = phi <4 x i32> [ %history.i.i751.sroa.0.0.copyload, %bb5.i.i1028.lr.ph ], [ %lanes.i5120.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887 ]
  %297 = add nuw nsw i32 %iter.sroa.0.0.i.i78717774, 1, !dbg !5838
  %_11.i125.i = add nuw nsw i32 %iter.sroa.0.0.i.i78717774, %iter.sroa.0.0.i77917818, !dbg !5841
  %base.i.i1029 = shl i32 %_11.i125.i, 2, !dbg !5841
  %_24.i.i1030 = icmp ugt i32 %base.i.i1029, %right_io.1, !dbg !5842
  br i1 %_24.i.i1030, label %bb7.i.i1228, label %bb8.i.i1031, !dbg !5842, !prof !902

bb8.i.i1031:                                      ; preds = %bb5.i.i1028
  %_27.i.i1032 = sub nuw nsw i32 %right_io.1, %base.i.i1029, !dbg !5845
  %_8.i5123 = icmp samesign ugt i32 %_27.i.i1032, 3, !dbg !5846
  br i1 %_8.i5123, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887, label %bb2.i5124, !dbg !5846, !prof !1153

bb2.i5124:                                        ; preds = %bb8.i.i1031
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_27.i.i1032, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !5851, !noalias !5852
  unreachable, !dbg !5851

bb7.i.i1228:                                      ; preds = %bb5.i.i1028
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i.i1029, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #33, !dbg !5859, !noalias !5860
  unreachable, !dbg !5859

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887: ; preds = %bb8.i.i1031
  %_31.i126.i = getelementptr inbounds nuw float, ptr %right_io.0, i32 %base.i.i1029, !dbg !5861
  %lanes.i5120.sroa.0.0.copyload = load <4 x i32>, ptr %_31.i126.i, align 4, !dbg !5863, !alias.scope !5867, !noalias !5871
  %298 = bitcast <4 x i32> %history.i.i751.sroa.19.017768 to <4 x float>, !dbg !5873
  %299 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %298), !dbg !5878
  %300 = bitcast <4 x i32> %lanes.i5120.sroa.0.0.copyload to <4 x float>, !dbg !5879
  %301 = fmul <4 x float> %_11.i.i.i.i104715121, %300, !dbg !5884
  %302 = fadd <4 x float> %301, zeroinitializer, !dbg !5885
  %303 = fmul <4 x float> %_14.i.i.i.i105015122, %300, !dbg !5889
  %304 = fadd <4 x float> %303, zeroinitializer, !dbg !5893
  %305 = fmul <4 x float> %_17.i.i.i.i105315123, %300, !dbg !5897
  %306 = fadd <4 x float> %305, zeroinitializer, !dbg !5901
  %307 = fmul <4 x float> %_20.i.i.i.i105615124, %300, !dbg !5905
  %308 = fadd <4 x float> %307, zeroinitializer, !dbg !5909
  %309 = bitcast <4 x i32> %history.i.i751.sroa.0.017763 to <4 x float>, !dbg !5913
  %310 = fmul <4 x float> %_25.i.i.i.i106115125, %309, !dbg !5917
  %311 = fadd <4 x float> %302, %310, !dbg !5918
  %312 = fmul <4 x float> %_28.i.i.i.i106415126, %309, !dbg !5922
  %313 = fadd <4 x float> %304, %312, !dbg !5926
  %314 = fmul <4 x float> %_31.i.i.i.i106715127, %309, !dbg !5930
  %315 = fadd <4 x float> %306, %314, !dbg !5934
  %316 = fmul <4 x float> %_34.i.i.i.i107015128, %309, !dbg !5938
  %317 = fadd <4 x float> %308, %316, !dbg !5942
  %318 = bitcast <4 x i32> %history.i.i751.sroa.7.017764 to <4 x float>, !dbg !5946
  %319 = fmul <4 x float> %_39.i.i.i.i107515129, %318, !dbg !5950
  %320 = fadd <4 x float> %311, %319, !dbg !5951
  %321 = fmul <4 x float> %_42.i.i.i.i107815130, %318, !dbg !5955
  %322 = fadd <4 x float> %313, %321, !dbg !5959
  %323 = fmul <4 x float> %_45.i.i.i.i108115131, %318, !dbg !5963
  %324 = fadd <4 x float> %315, %323, !dbg !5967
  %325 = fmul <4 x float> %_48.i.i.i.i108415132, %318, !dbg !5971
  %326 = fadd <4 x float> %317, %325, !dbg !5975
  %327 = bitcast <4 x i32> %history.i.i751.sroa.10.017765 to <4 x float>, !dbg !5979
  %328 = fmul <4 x float> %_53.i.i.i.i108915133, %327, !dbg !5983
  %329 = fadd <4 x float> %320, %328, !dbg !5984
  %330 = fmul <4 x float> %_56.i.i.i.i109215134, %327, !dbg !5988
  %331 = fadd <4 x float> %322, %330, !dbg !5992
  %332 = fmul <4 x float> %_59.i.i.i.i109515135, %327, !dbg !5996
  %333 = fadd <4 x float> %324, %332, !dbg !6000
  %334 = fmul <4 x float> %_62.i.i.i.i109815136, %327, !dbg !6004
  %335 = fadd <4 x float> %326, %334, !dbg !6008
  %336 = bitcast <4 x i32> %history.i.i751.sroa.13.017766 to <4 x float>, !dbg !6012
  %337 = fmul <4 x float> %_67.i.i.i.i110315137, %336, !dbg !6016
  %338 = fadd <4 x float> %329, %337, !dbg !6017
  %339 = fmul <4 x float> %_70.i.i.i.i110615138, %336, !dbg !6021
  %340 = fadd <4 x float> %331, %339, !dbg !6025
  %341 = fmul <4 x float> %_73.i.i.i.i110915139, %336, !dbg !6029
  %342 = fadd <4 x float> %333, %341, !dbg !6033
  %343 = fmul <4 x float> %_76.i.i.i.i111215140, %336, !dbg !6037
  %344 = fadd <4 x float> %335, %343, !dbg !6041
  %345 = bitcast <4 x i32> %history.i.i751.sroa.16.017767 to <4 x float>, !dbg !6045
  %346 = fmul <4 x float> %_81.i.i.i.i111715141, %345, !dbg !6049
  %347 = fadd <4 x float> %338, %346, !dbg !6050
  %348 = fmul <4 x float> %_84.i.i.i.i112015142, %345, !dbg !6054
  %349 = fadd <4 x float> %340, %348, !dbg !6058
  %350 = fmul <4 x float> %_87.i.i.i.i112315143, %345, !dbg !6062
  %351 = fadd <4 x float> %342, %350, !dbg !6066
  %352 = fmul <4 x float> %_90.i.i.i.i112615144, %345, !dbg !6070
  %353 = fadd <4 x float> %344, %352, !dbg !6074
  %354 = fmul <4 x float> %_95.i.i.i.i113115145, %298, !dbg !6078
  %355 = fadd <4 x float> %347, %354, !dbg !6082
  %356 = fmul <4 x float> %_98.i.i.i.i113415146, %298, !dbg !6086
  %357 = fadd <4 x float> %349, %356, !dbg !6090
  %358 = fmul <4 x float> %_101.i.i.i.i113715147, %298, !dbg !6094
  %359 = fadd <4 x float> %351, %358, !dbg !6098
  %360 = fmul <4 x float> %_104.i.i.i.i114015148, %298, !dbg !6102
  %361 = fadd <4 x float> %353, %360, !dbg !6106
  %362 = bitcast <4 x i32> %history.i.i751.sroa.22.017769 to <4 x float>, !dbg !6110
  %363 = fmul <4 x float> %_109.i.i.i.i114515149, %362, !dbg !6114
  %364 = fadd <4 x float> %355, %363, !dbg !6115
  %365 = fmul <4 x float> %_112.i.i.i.i114815150, %362, !dbg !6119
  %366 = fadd <4 x float> %357, %365, !dbg !6123
  %367 = fmul <4 x float> %_115.i.i.i.i115115151, %362, !dbg !6127
  %368 = fadd <4 x float> %359, %367, !dbg !6131
  %369 = fmul <4 x float> %_118.i.i.i.i115415152, %362, !dbg !6135
  %370 = fadd <4 x float> %361, %369, !dbg !6139
  %371 = bitcast <4 x i32> %history.i.i751.sroa.26.017770 to <4 x float>, !dbg !6143
  %372 = fmul <4 x float> %_123.i.i.i.i115915153, %371, !dbg !6147
  %373 = fadd <4 x float> %364, %372, !dbg !6148
  %374 = fmul <4 x float> %_126.i.i.i.i116215154, %371, !dbg !6152
  %375 = fadd <4 x float> %366, %374, !dbg !6156
  %376 = fmul <4 x float> %_129.i.i.i.i116515155, %371, !dbg !6160
  %377 = fadd <4 x float> %368, %376, !dbg !6164
  %378 = fmul <4 x float> %_132.i.i.i.i116815156, %371, !dbg !6168
  %379 = fadd <4 x float> %370, %378, !dbg !6172
  %380 = bitcast <4 x i32> %history.i.i751.sroa.29.017771 to <4 x float>, !dbg !6176
  %381 = fmul <4 x float> %_137.i.i.i.i117315157, %380, !dbg !6180
  %382 = fadd <4 x float> %373, %381, !dbg !6181
  %383 = fmul <4 x float> %_140.i.i.i.i117615158, %380, !dbg !6185
  %384 = fadd <4 x float> %375, %383, !dbg !6189
  %385 = fmul <4 x float> %_143.i.i.i.i117915159, %380, !dbg !6193
  %386 = fadd <4 x float> %377, %385, !dbg !6197
  %387 = fmul <4 x float> %_146.i.i.i.i118215160, %380, !dbg !6201
  %388 = fadd <4 x float> %379, %387, !dbg !6205
  %389 = bitcast <4 x i32> %history.i.i751.sroa.32.017772 to <4 x float>, !dbg !6209
  %390 = fmul <4 x float> %_151.i.i.i.i118715161, %389, !dbg !6213
  %391 = fadd <4 x float> %382, %390, !dbg !6214
  %392 = fmul <4 x float> %_154.i.i.i.i119015162, %389, !dbg !6218
  %393 = fadd <4 x float> %384, %392, !dbg !6222
  %394 = fmul <4 x float> %_157.i.i.i.i119315163, %389, !dbg !6226
  %395 = fadd <4 x float> %386, %394, !dbg !6230
  %396 = fmul <4 x float> %_160.i.i.i.i119615164, %389, !dbg !6234
  %397 = fadd <4 x float> %388, %396, !dbg !6238
  %398 = bitcast <4 x i32> %history.i.i751.sroa.35.017773 to <4 x float>, !dbg !6242
  %399 = fmul <4 x float> %_165.i.i.i.i120115165, %398, !dbg !6246
  %400 = fadd <4 x float> %391, %399, !dbg !6247
  %401 = fmul <4 x float> %_168.i.i.i.i120415166, %398, !dbg !6251
  %402 = fadd <4 x float> %393, %401, !dbg !6255
  %403 = fmul <4 x float> %_171.i.i.i.i120715167, %398, !dbg !6259
  %404 = fadd <4 x float> %395, %403, !dbg !6263
  %405 = fmul <4 x float> %_174.i.i.i.i121015168, %398, !dbg !6267
  %406 = fadd <4 x float> %397, %405, !dbg !6271
  %407 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %400), !dbg !6275
  %408 = fcmp olt <4 x float> %407, %299, !dbg !6279
  %409 = select <4 x i1> %408, <4 x float> %299, <4 x float> %407, !dbg !6283
  %410 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %402), !dbg !6275
  %411 = fcmp olt <4 x float> %410, %409, !dbg !6279
  %412 = select <4 x i1> %411, <4 x float> %409, <4 x float> %410, !dbg !6283
  %413 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %404), !dbg !6275
  %414 = fcmp olt <4 x float> %413, %412, !dbg !6279
  %415 = select <4 x i1> %414, <4 x float> %412, <4 x float> %413, !dbg !6283
  %416 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %406), !dbg !6275
  %417 = fcmp olt <4 x float> %416, %415, !dbg !6279
  %418 = select <4 x i1> %417, <4 x float> %415, <4 x float> %416, !dbg !6283
  %_39.i.i1222.idx = shl i32 %iter.sroa.0.0.i.i78717774, 4, !dbg !6284
  %_39.i.i1222 = getelementptr inbounds nuw i8, ptr %peaks_right.i756, i32 %_39.i.i1222.idx, !dbg !6284
  store <4 x float> %418, ptr %_39.i.i1222, align 4, !dbg !6289, !alias.scope !6294, !noalias !6298
  %exitcond20465.not = icmp eq i32 %297, %umax20489, !dbg !6302
  br i1 %exitcond20465.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i789, label %bb5.i.i1028, !dbg !5835

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i789: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit349.i
  %history.i.i751.sroa.0.0.lcssa = phi <4 x i32> [ %history.i.i751.sroa.0.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit349.i ], [ %lanes.i5120.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887 ], !dbg !6304
  %history.i.i751.sroa.7.0.lcssa = phi <4 x i32> [ %history.i.i751.sroa.7.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit349.i ], [ %history.i.i751.sroa.0.017763, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887 ], !dbg !6304
  %history.i.i751.sroa.10.0.lcssa = phi <4 x i32> [ %history.i.i751.sroa.10.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit349.i ], [ %history.i.i751.sroa.7.017764, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887 ], !dbg !6304
  %history.i.i751.sroa.13.0.lcssa = phi <4 x i32> [ %history.i.i751.sroa.13.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit349.i ], [ %history.i.i751.sroa.10.017765, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887 ], !dbg !6304
  %history.i.i751.sroa.16.0.lcssa = phi <4 x i32> [ %history.i.i751.sroa.16.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit349.i ], [ %history.i.i751.sroa.13.017766, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887 ], !dbg !6304
  %history.i.i751.sroa.19.0.lcssa = phi <4 x i32> [ %history.i.i751.sroa.19.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit349.i ], [ %history.i.i751.sroa.16.017767, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887 ], !dbg !6304
  %history.i.i751.sroa.22.0.lcssa = phi <4 x i32> [ %history.i.i751.sroa.22.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit349.i ], [ %history.i.i751.sroa.19.017768, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887 ], !dbg !6304
  %history.i.i751.sroa.26.0.lcssa = phi <4 x i32> [ %history.i.i751.sroa.26.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit349.i ], [ %history.i.i751.sroa.22.017769, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887 ], !dbg !6304
  %history.i.i751.sroa.29.0.lcssa = phi <4 x i32> [ %history.i.i751.sroa.29.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit349.i ], [ %history.i.i751.sroa.26.017770, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887 ], !dbg !6304
  %history.i.i751.sroa.32.0.lcssa = phi <4 x i32> [ %history.i.i751.sroa.32.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit349.i ], [ %history.i.i751.sroa.29.017771, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887 ], !dbg !6304
  %history.i.i751.sroa.35.0.lcssa = phi <4 x i32> [ %history.i.i751.sroa.35.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit349.i ], [ %history.i.i751.sroa.32.017772, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887 ], !dbg !6304
  %history.i.i751.sroa.38.0.lcssa = phi <4 x i32> [ %history.i.i751.sroa.38.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit349.i ], [ %history.i.i751.sroa.35.017773, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5887 ], !dbg !6304
  store <4 x i32> %history.i.i751.sroa.0.0.lcssa, ptr %hot_right.i760, align 16, !dbg !6305
  store <4 x i32> %history.i.i751.sroa.7.0.lcssa, ptr %history.i.i751.sroa.7.0.hot_right.i760.sroa_idx, align 16, !dbg !6305
  store <4 x i32> %history.i.i751.sroa.10.0.lcssa, ptr %history.i.i751.sroa.10.0.hot_right.i760.sroa_idx, align 16, !dbg !6305
  store <4 x i32> %history.i.i751.sroa.13.0.lcssa, ptr %history.i.i751.sroa.13.0.hot_right.i760.sroa_idx, align 16, !dbg !6305
  store <4 x i32> %history.i.i751.sroa.16.0.lcssa, ptr %history.i.i751.sroa.16.0.hot_right.i760.sroa_idx, align 16, !dbg !6305
  store <4 x i32> %history.i.i751.sroa.19.0.lcssa, ptr %history.i.i751.sroa.19.0.hot_right.i760.sroa_idx, align 16, !dbg !6305
  store <4 x i32> %history.i.i751.sroa.22.0.lcssa, ptr %history.i.i751.sroa.22.0.hot_right.i760.sroa_idx, align 16, !dbg !6305
  store <4 x i32> %history.i.i751.sroa.26.0.lcssa, ptr %history.i.i751.sroa.26.0.hot_right.i760.sroa_idx, align 16, !dbg !6305
  store <4 x i32> %history.i.i751.sroa.29.0.lcssa, ptr %history.i.i751.sroa.29.0.hot_right.i760.sroa_idx, align 16, !dbg !6305
  store <4 x i32> %history.i.i751.sroa.32.0.lcssa, ptr %history.i.i751.sroa.32.0.hot_right.i760.sroa_idx, align 16, !dbg !6305
  store <4 x i32> %history.i.i751.sroa.35.0.lcssa, ptr %history.i.i751.sroa.35.0.hot_right.i760.sroa_idx, align 16, !dbg !6305
  store <4 x i32> %history.i.i751.sroa.38.0.lcssa, ptr %history.i.i751.sroa.38.0.hot_right.i760.sroa_idx, align 16, !dbg !6305
  br i1 %_20.i144.i17736.not, label %bb13.i778.loopexit, label %bb42.i795.preheader, !dbg !6306

bb42.i795.preheader:                              ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i789
  %_5.i199815084.pre = load <4 x float>, ptr %126, align 16, !dbg !6312
  %_11.i200315086.pre = load <4 x float>, ptr %_64.i798, align 16, !dbg !6313
  %_12.i2004.pre = load <4 x i32>, ptr %127, align 16, !dbg !6314
  %_13.i200615087.pre = load <16 x i8>, ptr %128, align 16, !dbg !6315
  %_5.i198515088.pre = load <4 x float>, ptr %129, align 16, !dbg !6316
  %_11.i199015090.pre = load <4 x float>, ptr %_65.i799, align 16, !dbg !6317
  %_12.i1991.pre = load <4 x i32>, ptr %130, align 16, !dbg !6318
  %_13.i199315091.pre = load <16 x i8>, ptr %131, align 16, !dbg !6319
  %_5.i197215092.pre = load <4 x float>, ptr %132, align 16, !dbg !6320
  %_11.i197715094.pre = load <4 x float>, ptr %_69.i802, align 16, !dbg !6321
  %_12.i1978.pre = load <4 x i32>, ptr %133, align 16, !dbg !6322
  %_13.i198015095.pre = load <16 x i8>, ptr %134, align 16, !dbg !6323
  %_5.i196315096.pre = load <4 x float>, ptr %135, align 16, !dbg !6324
  %.promoted22419 = load <4 x float>, ptr %149, align 16
  %.promoted22455 = load <4 x float>, ptr %166, align 16
  br label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5209

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5209: ; preds = %bb42.i795.preheader, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5892
  %419 = phi <4 x float> [ %.promoted22455, %bb42.i795.preheader ], [ %592, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5892 ]
  %420 = phi <4 x float> [ %.promoted22419, %bb42.i795.preheader ], [ %502, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5892 ]
  %_5.i196315096 = phi <4 x float> [ %_5.i196315096.pre, %bb42.i795.preheader ], [ %451, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5892 ], !dbg !6324
  %_12.i1978 = phi <4 x i32> [ %_12.i1978.pre, %bb42.i795.preheader ], [ %463, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5892 ], !dbg !6322
  %_11.i197715094 = phi <4 x float> [ %_11.i197715094.pre, %bb42.i795.preheader ], [ %462, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5892 ], !dbg !6321
  %_5.i197215092 = phi <4 x float> [ %_5.i197215092.pre, %bb42.i795.preheader ], [ %441, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5892 ], !dbg !6320
  %_12.i1991 = phi <4 x i32> [ %_12.i1991.pre, %bb42.i795.preheader ], [ %461, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5892 ], !dbg !6318
  %_11.i199015090 = phi <4 x float> [ %_11.i199015090.pre, %bb42.i795.preheader ], [ %460, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5892 ], !dbg !6317
  %_5.i198515088 = phi <4 x float> [ %_5.i198515088.pre, %bb42.i795.preheader ], [ %432, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5892 ], !dbg !6316
  %_12.i2004 = phi <4 x i32> [ %_12.i2004.pre, %bb42.i795.preheader ], [ %459, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5892 ], !dbg !6314
  %_11.i200315086 = phi <4 x float> [ %_11.i200315086.pre, %bb42.i795.preheader ], [ %458, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5892 ], !dbg !6313
  %_5.i199815084 = phi <4 x float> [ %_5.i199815084.pre, %bb42.i795.preheader ], [ %423, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5892 ], !dbg !6312
  %main_cursor.sroa.0.1.i79317814 = phi i32 [ %main_cursor.sroa.0.0.i78217821, %bb42.i795.preheader ], [ %spec.store.select11.i1012, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5892 ]
  %ring_cursor.sroa.0.1.i79217813 = phi i32 [ %ring_cursor.sroa.0.0.i78117820, %bb42.i795.preheader ], [ %spec.store.select12.i1014, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5892 ]
  %iter1.sroa.0.0.i79117812 = phi i32 [ 0, %bb42.i795.preheader ], [ %448, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5892 ]
  %421 = fadd <4 x float> %_5.i199815084, splat (float -1.000000e+00), !dbg !6325
  %422 = fcmp ogt <4 x float> %421, zeroinitializer, !dbg !6333
  %423 = select <4 x i1> %422, <4 x float> %421, <4 x float> zeroinitializer, !dbg !6337
  %424 = sext <4 x i1> %422 to <4 x i32>, !dbg !6338
  %425 = bitcast <4 x i32> %_12.i2004 to <4 x float>, !dbg !6348
  %426 = fadd <4 x float> %_11.i200315086, %425, !dbg !6352
  %427 = bitcast <4 x float> %426 to <16 x i8>, !dbg !6353
  %428 = bitcast <4 x i32> %424 to <16 x i8>, !dbg !6357
  %_4.i6458 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %427, <16 x i8> %_13.i200615087.pre, <16 x i8> %428), !dbg !6358
  %429 = bitcast <4 x i32> %_12.i2004 to <16 x i8>, !dbg !6359
  %_4.i6459 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %429, <16 x i8> zeroinitializer, <16 x i8> %428), !dbg !6363
  %430 = fadd <4 x float> %_5.i198515088, splat (float -1.000000e+00), !dbg !6364
  %431 = fcmp ogt <4 x float> %430, zeroinitializer, !dbg !6368
  %432 = select <4 x i1> %431, <4 x float> %430, <4 x float> zeroinitializer, !dbg !6372
  %433 = sext <4 x i1> %431 to <4 x i32>, !dbg !6373
  %434 = bitcast <4 x i32> %_12.i1991 to <4 x float>, !dbg !6378
  %435 = fadd <4 x float> %_11.i199015090, %434, !dbg !6382
  %436 = bitcast <4 x float> %435 to <16 x i8>, !dbg !6383
  %437 = bitcast <4 x i32> %433 to <16 x i8>, !dbg !6387
  %_4.i6460 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %436, <16 x i8> %_13.i199315091.pre, <16 x i8> %437), !dbg !6388
  %438 = bitcast <4 x i32> %_12.i1991 to <16 x i8>, !dbg !6389
  %_4.i6461 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %438, <16 x i8> zeroinitializer, <16 x i8> %437), !dbg !6393
  %439 = fadd <4 x float> %_5.i197215092, splat (float -1.000000e+00), !dbg !6394
  %440 = fcmp ogt <4 x float> %439, zeroinitializer, !dbg !6398
  %441 = select <4 x i1> %440, <4 x float> %439, <4 x float> zeroinitializer, !dbg !6402
  %442 = sext <4 x i1> %440 to <4 x i32>, !dbg !6403
  %443 = bitcast <4 x i32> %_12.i1978 to <4 x float>, !dbg !6408
  %444 = fadd <4 x float> %_11.i197715094, %443, !dbg !6412
  %445 = bitcast <4 x float> %444 to <16 x i8>, !dbg !6413
  %446 = bitcast <4 x i32> %442 to <16 x i8>, !dbg !6417
  %_4.i6462 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %445, <16 x i8> %_13.i198015095.pre, <16 x i8> %446), !dbg !6418
  %447 = bitcast <4 x i32> %_12.i1978 to <16 x i8>, !dbg !6419
  %_4.i6463 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %447, <16 x i8> zeroinitializer, <16 x i8> %446), !dbg !6423
  %448 = add nuw nsw i32 %iter1.sroa.0.0.i79117812, 1, !dbg !6424
  %_60.i796 = add nuw nsw i32 %iter1.sroa.0.0.i79117812, %iter.sroa.0.0.i77917818, !dbg !6430
  %base.i797 = shl i32 %_60.i796, 2, !dbg !6430
  %449 = fadd <4 x float> %_5.i196315096, splat (float -1.000000e+00), !dbg !6431
  %450 = fcmp ogt <4 x float> %449, zeroinitializer, !dbg !6435
  %451 = select <4 x i1> %450, <4 x float> %449, <4 x float> zeroinitializer, !dbg !6439
  %452 = sext <4 x i1> %450 to <4 x i32>, !dbg !6440
  %_11.i196515098 = load <4 x float>, ptr %_70.i803, align 16, !dbg !6445
  %_12.i1966 = load <4 x i32>, ptr %136, align 16, !dbg !6446
  %453 = bitcast <4 x i32> %_12.i1966 to <4 x float>, !dbg !6447
  %454 = fadd <4 x float> %_11.i196515098, %453, !dbg !6451
  %_13.i196815099 = load <16 x i8>, ptr %137, align 16, !dbg !6452
  %455 = bitcast <4 x float> %454 to <16 x i8>, !dbg !6453
  %456 = bitcast <4 x i32> %452 to <16 x i8>, !dbg !6457
  %_4.i6464 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %455, <16 x i8> %_13.i196815099, <16 x i8> %456), !dbg !6458
  store <16 x i8> %_4.i6464, ptr %_70.i803, align 16, !dbg !6459
  %457 = bitcast <4 x i32> %_12.i1966 to <16 x i8>, !dbg !6460
  %_4.i6465 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %457, <16 x i8> zeroinitializer, <16 x i8> %456), !dbg !6464
  store <16 x i8> %_4.i6465, ptr %136, align 16, !dbg !6465
  %_74.i808 = shl i32 %iter1.sroa.0.0.i79117812, 2, !dbg !6466
  %458 = bitcast <16 x i8> %_4.i6458 to <4 x float>, !dbg !6467
  %459 = bitcast <16 x i8> %_4.i6459 to <4 x i32>, !dbg !6467
  %460 = bitcast <16 x i8> %_4.i6460 to <4 x float>, !dbg !6467
  %461 = bitcast <16 x i8> %_4.i6461 to <4 x i32>, !dbg !6467
  %462 = bitcast <16 x i8> %_4.i6462 to <4 x float>, !dbg !6467
  %463 = bitcast <16 x i8> %_4.i6463 to <4 x i32>, !dbg !6467
  %_126.i812 = getelementptr inbounds nuw float, ptr %peaks_left.i757, i32 %_74.i808, !dbg !6475
  %lanes.i5202.sroa.0.0.copyload = load <4 x i32>, ptr %_126.i812, align 4, !dbg !6480, !alias.scope !6485, !noalias !6489
  %_131.i814 = getelementptr inbounds nuw float, ptr %peaks_right.i756, i32 %_74.i808, !dbg !6493
  %lanes.i5193.sroa.0.0.copyload = load <4 x i32>, ptr %_131.i814, align 4, !dbg !6503, !alias.scope !6508, !noalias !6512
  %464 = bitcast <4 x i32> %lanes.i5202.sroa.0.0.copyload to <4 x float>, !dbg !6516
  %465 = bitcast <4 x i32> %lanes.i5193.sroa.0.0.copyload to <4 x float>, !dbg !6520
  %466 = fcmp olt <4 x float> %464, %465, !dbg !6521
  %.v = select <4 x i1> %466, <4 x i32> %lanes.i5193.sroa.0.0.copyload, <4 x i32> %lanes.i5202.sroa.0.0.copyload, !dbg !6522
  %467 = bitcast <4 x i32> %.v to <16 x i8>, !dbg !6523
  %468 = bitcast <4 x i32> %lanes.i5202.sroa.0.0.copyload to <16 x i8>, !dbg !6527
  %_4.i6468 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %467, <16 x i8> %468, <16 x i8> %138), !dbg !6528
  %469 = bitcast <4 x i32> %lanes.i5193.sroa.0.0.copyload to <16 x i8>, !dbg !6529
  %_4.i6469 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %467, <16 x i8> %469, <16 x i8> %138), !dbg !6533
  %_132.i819 = icmp ugt i32 %base.i797, %left_io.1, !dbg !6534
  br i1 %_132.i819, label %bb46.i1026, label %bb47.i820, !dbg !6534, !prof !902

bb47.i820:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5209
  %_135.i821 = sub nuw nsw i32 %left_io.1, %base.i797, !dbg !6538
  %_139.i822 = getelementptr inbounds nuw float, ptr %left_io.0, i32 %base.i797, !dbg !6539
  %_8.i5187 = icmp samesign ugt i32 %_135.i821, 3, !dbg !6544
  br i1 %_8.i5187, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5191, label %bb2.i5188, !dbg !6544, !prof !1153

bb2.i5188:                                        ; preds = %bb47.i820
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %420, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_135.i821, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !6549, !noalias !6550
  unreachable, !dbg !6549

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5191: ; preds = %bb47.i820
  %lanes.i5184.sroa.0.0.copyload = load <4 x i32>, ptr %_139.i822, align 4, !dbg !6554, !alias.scope !6558, !noalias !6562
  %_87.i824 = load i32, ptr %139, align 4, !dbg !6564, !alias.scope !5173, !noalias !6565, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6566), !dbg !6569
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6570), !dbg !6569
  %width.i31.i826 = load i32, ptr %140, align 4, !dbg !6572, !alias.scope !6573, !noalias !6574, !noundef !10
  %470 = bitcast <16 x i8> %_4.i6468 to <4 x float>, !dbg !6582
  %471 = bitcast <16 x i8> %_4.i6458 to <4 x float>, !dbg !6587
  %472 = fcmp ogt <4 x float> %470, %471, !dbg !6588
  %473 = sext <4 x i1> %472 to <4 x i32>, !dbg !6588
  %474 = fdiv <4 x float> %471, %470, !dbg !6589
  %475 = bitcast <4 x float> %474 to <16 x i8>, !dbg !6597
  %476 = bitcast <4 x i32> %473 to <16 x i8>, !dbg !6601
  %_4.i6471 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %475, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %476), !dbg !6602
  %_158.1.i36.i831 = load i32, ptr %141, align 4, !dbg !6603, !alias.scope !6573, !noalias !6574, !noundef !10
  %_22.i37.i832 = mul i32 %width.i31.i826, %ring_cursor.sroa.0.1.i79217813, !dbg !6604
  %_90.i38.i833 = icmp ugt i32 %_22.i37.i832, %_158.1.i36.i831, !dbg !6605
  br i1 %_90.i38.i833, label %bb34.i123.i1025, label %bb35.i39.i834, !dbg !6605, !prof !902

bb35.i39.i834:                                    ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5191
  %_93.i41.i836 = sub nuw i32 %_158.1.i36.i831, %_22.i37.i832, !dbg !6610
  %_8.i5924 = icmp samesign ugt i32 %_93.i41.i836, 3, !dbg !6611
  br i1 %_8.i5924, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5927, label %bb2.i5925, !dbg !6611, !prof !1153

bb2.i5925:                                        ; preds = %bb35.i39.i834
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %420, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_93.i41.i836, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !6616, !noalias !6617
  unreachable, !dbg !6616

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5927: ; preds = %bb35.i39.i834
  %_158.0.i40.i835 = load ptr, ptr %142, align 4, !dbg !6603, !alias.scope !6573, !noalias !6574, !nonnull !10, !noundef !10
  %_97.i42.i837 = getelementptr inbounds nuw float, ptr %_158.0.i40.i835, i32 %_22.i37.i832, !dbg !6621
  store <16 x i8> %_4.i6471, ptr %_97.i42.i837, align 4, !dbg !6626, !alias.scope !6630, !noalias !6634
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6636), !dbg !6639
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6640), !dbg !6639
  %width.i1587 = load i32, ptr %140, align 4, !dbg !6642, !alias.scope !6636, !noalias !6645, !noundef !10
  %477 = icmp eq i32 %width.i1587, 0, !dbg !6646
  br i1 %477, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1693, label %bb29.i1593.lr.ph, !dbg !6646

bb29.i1593.lr.ph:                                 ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5927
  %_126.1.i1598 = load i32, ptr %62, align 4, !alias.scope !6636, !noalias !6645, !noundef !10
  %_126.0.i1602 = load ptr, ptr %61, align 4, !nonnull !10
  %478 = add i32 %ring_cursor.sroa.0.1.i79217813, 1
  %_21.not.i1607 = icmp ult i32 %478, %_87.i824
  %479 = select i1 %_21.not.i1607, i32 0, i32 %_87.i824
  %start1.sroa.0.0.i1608 = sub nuw i32 %478, %479
  %_128.1.i1611 = load i32, ptr %141, align 4
  %_128.0.i1615 = load ptr, ptr %142, align 4, !nonnull !10
  %_130.1.i1616 = load i32, ptr %143, align 4
  %_130.0.i1620 = load ptr, ptr %144, align 4, !nonnull !10
  %_132.1.i1623 = load i32, ptr %145, align 4
  %_132.0.i1627 = load ptr, ptr %146, align 4, !nonnull !10
  %_43.i1640 = mul i32 %width.i1587, %start1.sroa.0.0.i1608
  br label %bb29.i1593, !dbg !6646

bb29.i1593:                                       ; preds = %bb29.i1593.lr.ph, %bb28.i1655
  %iter.sroa.0.0.idx.i159117793 = phi i32 [ 0, %bb29.i1593.lr.ph ], [ %iter.sroa.0.0.add.i1596, %bb28.i1655 ]
  %iter.sroa.4.0.i159017792 = phi i32 [ 0, %bb29.i1593.lr.ph ], [ %_102.0.i1597, %bb28.i1655 ]
  %iter.sroa.7.0.i158917791 = phi i32 [ %width.i1587, %bb29.i1593.lr.ph ], [ %480, %bb28.i1655 ]
  %iter.sroa.0.0.ptr.i159217794 = getelementptr inbounds nuw i8, ptr %scratch.i758, i32 %iter.sroa.0.0.idx.i159117793, !dbg !6655
  %480 = add i32 %iter.sroa.7.0.i158917791, -1, !dbg !6655
  %_109.i1594 = icmp eq i32 %iter.sroa.0.0.idx.i159117793, 32, !dbg !6656
  br i1 %_109.i1594, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1693, label %bb33.i1595, !dbg !6665

bb33.i1595:                                       ; preds = %bb29.i1593
  %iter.sroa.0.0.add.i1596 = add nuw nsw i32 %iter.sroa.0.0.idx.i159117793, 4, !dbg !6666
  %_102.0.i1597 = add nuw nsw i32 %iter.sroa.4.0.i159017792, 1, !dbg !6669
  %exitcond20472.not = icmp eq i32 %iter.sroa.4.0.i159017792, %_126.1.i1598, !dbg !6672
  br i1 %exitcond20472.not, label %panic.i1600, label %bb2.i1601, !dbg !6672

bb2.i1601:                                        ; preds = %bb33.i1595
  %481 = getelementptr inbounds nuw %LaneShape, ptr %_126.0.i1602, i32 %iter.sroa.4.0.i159017792, !dbg !6672
  %shape.i1603 = load i32, ptr %481, align 4, !dbg !6672, !noalias !6674, !noundef !10
  %482 = getelementptr inbounds nuw i8, ptr %481, i32 4, !dbg !6672
  %shape3.i1604 = load i32, ptr %482, align 4, !dbg !6672, !noalias !6674, !noundef !10
  %483 = add i32 %shape3.i1604, %ring_cursor.sroa.0.1.i79217813, !dbg !6675
  %_18.not.i1605 = icmp ult i32 %483, %_87.i824, !dbg !6678
  %484 = select i1 %_18.not.i1605, i32 0, i32 %_87.i824, !dbg !6678
  %spec.select.i1606 = sub nuw i32 %483, %484, !dbg !6678
  %_25.i1609 = mul i32 %spec.select.i1606, %width.i1587, !dbg !6680
  %_24.i1610 = add i32 %_25.i1609, %iter.sroa.4.0.i159017792, !dbg !6680
  %_28.i1612 = icmp ult i32 %_24.i1610, %_128.1.i1611, !dbg !6682
  br i1 %_28.i1612, label %bb9.i1614, label %panic5.i1613, !dbg !6682

panic.i1600:                                      ; preds = %bb33.i1595
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %420, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_126.1.i1598, i32 noundef %_126.1.i1598, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_12856b5f9033764dbf23e04eb77c5c46) #33, !dbg !6672, !noalias !6674
  unreachable, !dbg !6672

bb9.i1614:                                        ; preds = %bb2.i1601
  %485 = getelementptr inbounds nuw float, ptr %_128.0.i1615, i32 %_24.i1610, !dbg !6682
  %486 = load float, ptr %485, align 4, !dbg !6682, !noalias !6674, !noundef !10
  %exitcond20473.not = icmp eq i32 %iter.sroa.4.0.i159017792, %_130.1.i1616, !dbg !6683
  br i1 %exitcond20473.not, label %panic6.i1618, label %bb10.i1619, !dbg !6683

panic5.i1613:                                     ; preds = %bb2.i1601
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %420, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_24.i1610, i32 noundef %_128.1.i1611, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70ebf0cf1c90c6161926611ccf249a32) #33, !dbg !6682, !noalias !6674
  unreachable, !dbg !6682

bb10.i1619:                                       ; preds = %bb9.i1614
  %487 = getelementptr inbounds nuw i32, ptr %_130.0.i1620, i32 %iter.sroa.4.0.i159017792, !dbg !6683
  %_30.i1621 = load i32, ptr %487, align 4, !dbg !6683, !noalias !6674, !noundef !10
  %488 = icmp eq i32 %_30.i1621, 0, !dbg !6685
  br i1 %488, label %bb14.i1630, label %bb12.i1622, !dbg !6685

panic6.i1618:                                     ; preds = %bb9.i1614
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %420, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_130.1.i1616, i32 noundef %_130.1.i1616, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_023d591aaad7f5cf482ac80a9401eca1) #33, !dbg !6683, !noalias !6674
  unreachable, !dbg !6683

bb12.i1622:                                       ; preds = %bb10.i1619
  %_35.i1624 = icmp ult i32 %iter.sroa.4.0.i159017792, %_132.1.i1623, !dbg !6687
  br i1 %_35.i1624, label %bb13.i1626, label %panic7.i1625, !dbg !6687

bb14.i1630:                                       ; preds = %bb34.i1691, %bb13.i1626, %bb10.i1619
  %newest.sroa.0.0.i1631 = phi float [ %486, %bb10.i1619 ], [ %_33.i1628, %bb34.i1691 ], [ %486, %bb13.i1626 ], !dbg !6688
  %exitcond20474.not = icmp eq i32 %iter.sroa.4.0.i159017792, %_132.1.i1623, !dbg !6689
  br i1 %exitcond20474.not, label %panic8.i1634, label %bb15.i1635, !dbg !6689

bb13.i1626:                                       ; preds = %bb12.i1622
  %489 = getelementptr inbounds nuw float, ptr %_132.0.i1627, i32 %iter.sroa.4.0.i159017792, !dbg !6687
  %_33.i1628 = load float, ptr %489, align 4, !dbg !6687, !noalias !6674, !noundef !10
  %_116.i1629 = fcmp olt float %_33.i1628, %486, !dbg !6691
  br i1 %_116.i1629, label %bb34.i1691, label %bb14.i1630, !dbg !6691

panic7.i1625:                                     ; preds = %bb12.i1622
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %420, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %iter.sroa.4.0.i159017792, i32 noundef %_132.1.i1623, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a228cac3279aae3a34886f59f51fbe9e) #33, !dbg !6687, !noalias !6674
  unreachable, !dbg !6687

bb34.i1691:                                       ; preds = %bb13.i1626
  br label %bb14.i1630, !dbg !6694

bb15.i1635:                                       ; preds = %bb14.i1630
  %490 = getelementptr inbounds nuw float, ptr %_132.0.i1627, i32 %iter.sroa.4.0.i159017792, !dbg !6689
  store float %newest.sroa.0.0.i1631, ptr %490, align 4, !dbg !6689, !noalias !6674
  %_40.i1637 = add i32 %_30.i1621, 1, !dbg !6695
  %complete.i1638 = icmp eq i32 %_40.i1637, %shape.i1603, !dbg !6695
  br i1 %complete.i1638, label %bb19.i1660, label %bb17.i1639, !dbg !6696

panic8.i1634:                                     ; preds = %bb14.i1630
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %420, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_132.1.i1623, i32 noundef %_132.1.i1623, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1fa5d88c1a2cc292a2767c48606a38fe) #33, !dbg !6689, !noalias !6674
  unreachable, !dbg !6689

bb17.i1639:                                       ; preds = %bb15.i1635
  %_42.i1641 = add i32 %iter.sroa.4.0.i159017792, %_43.i1640, !dbg !6698
  %_45.i1643 = icmp ult i32 %_42.i1641, %_128.1.i1611, !dbg !6699
  br i1 %_45.i1643, label %bb27.i1653, label %panic9.i1644, !dbg !6699

panic9.i1644:                                     ; preds = %bb17.i1639
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %420, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_42.i1641, i32 noundef %_128.1.i1611, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70d40ae2302f3ea19fa0c4a65fe82786) #33, !dbg !6699, !noalias !6674
  unreachable, !dbg !6699

bb27.i1653:                                       ; preds = %bb17.i1639
  %491 = getelementptr inbounds nuw float, ptr %_128.0.i1615, i32 %_42.i1641, !dbg !6699
  %_41.i1647 = load float, ptr %491, align 4, !dbg !6699, !noalias !6674, !noundef !10
  %_117.i1648 = fcmp olt float %_41.i1647, %newest.sroa.0.0.i1631, !dbg !6700
  %newest.sroa.0.1.i1649 = select i1 %_117.i1648, float %_41.i1647, float %newest.sroa.0.0.i1631, !dbg !6700
  store float %newest.sroa.0.1.i1649, ptr %iter.sroa.0.0.ptr.i159217794, align 4, !dbg !6702, !alias.scope !6640, !noalias !6703
  br label %bb28.i1655, !dbg !6704

bb28.i1655:                                       ; preds = %bb22.i1688, %bb19.i1660, %bb27.i1653
  %storemerge = phi i32 [ %_40.i1637, %bb27.i1653 ], [ 0, %bb19.i1660 ], [ 0, %bb22.i1688 ], !dbg !6705
  store i32 %storemerge, ptr %487, align 4, !dbg !6705, !noalias !6674
  %492 = icmp eq i32 %480, 0, !dbg !6646
  br i1 %492, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1693, label %bb29.i1593, !dbg !6646

bb19.i1660:                                       ; preds = %bb15.i1635
  store float %newest.sroa.0.0.i1631, ptr %iter.sroa.0.0.ptr.i159217794, align 4, !dbg !6702, !alias.scope !6640, !noalias !6703
  %_118.i166617787.not = icmp eq i32 %shape.i1603, 0, !dbg !6706
  br i1 %_118.i166617787.not, label %bb28.i1655, label %bb40.i1673.preheader, !dbg !6717

bb40.i1673.preheader:                             ; preds = %bb19.i1660
  %493 = load float, ptr %485, align 4, !dbg !6718, !noalias !6674, !noundef !10
  br label %bb40.i1673, !dbg !6719

bb40.i1673:                                       ; preds = %bb40.i1673.preheader, %bb22.i1688
  %iter2.sroa.0.0.i166517790 = phi i32 [ %_119.i1674, %bb22.i1688 ], [ 0, %bb40.i1673.preheader ]
  %suffix.sroa.0.0.i166417789 = phi float [ %suffix.sroa.0.1.i1684, %bb22.i1688 ], [ %493, %bb40.i1673.preheader ]
  %end.sroa.0.1.i166317788 = phi i32 [ %496, %bb22.i1688 ], [ %spec.select.i1606, %bb40.i1673.preheader ]
  %_54.i1675 = mul i32 %end.sroa.0.1.i166317788, %width.i1587, !dbg !6720
  %_53.i1676 = add i32 %_54.i1675, %iter.sroa.4.0.i159017792, !dbg !6720
  %_57.i1678 = icmp ult i32 %_53.i1676, %_128.1.i1611, !dbg !6719
  br i1 %_57.i1678, label %bb22.i1688, label %panic13.i1679, !dbg !6719

panic13.i1679:                                    ; preds = %bb40.i1673
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %420, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_53.i1676, i32 noundef %_128.1.i1611, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a271bbb7fd83c3605ea58a06b7065fa4) #33, !dbg !6719, !noalias !6674
  unreachable, !dbg !6719

bb22.i1688:                                       ; preds = %bb40.i1673
  %_119.i1674 = add nuw i32 %iter2.sroa.0.0.i166517790, 1, !dbg !6721
  %494 = getelementptr inbounds nuw float, ptr %_128.0.i1615, i32 %_53.i1676, !dbg !6719
  %_52.i1682 = load float, ptr %494, align 4, !dbg !6719, !noalias !6674, !noundef !10
  %_121.i1683 = fcmp olt float %suffix.sroa.0.0.i166417789, %_52.i1682, !dbg !6727
  %suffix.sroa.0.1.i1684 = select i1 %_121.i1683, float %suffix.sroa.0.0.i166417789, float %_52.i1682, !dbg !6727
  store float %suffix.sroa.0.1.i1684, ptr %494, align 4, !dbg !6729, !noalias !6674
  %495 = icmp eq i32 %end.sroa.0.1.i166317788, 0, !dbg !6730
  %spec.store.select.i1690 = select i1 %495, i32 %_87.i824, i32 %end.sroa.0.1.i166317788, !dbg !6730
  %496 = add i32 %spec.store.select.i1690, -1, !dbg !6731
  %exitcond20471.not = icmp eq i32 %_119.i1674, %shape.i1603, !dbg !6706
  br i1 %exitcond20471.not, label %bb28.i1655, label %bb40.i1673, !dbg !6717

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1693: ; preds = %bb29.i1593, %bb28.i1655, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5927
  %lanes.i5177.sroa.0.0.copyload = load <4 x float>, ptr %scratch.i758, align 4, !dbg !6732, !alias.scope !6737, !noalias !6741
  %497 = fmul <4 x float> %lanes.i5177.sroa.0.0.copyload, splat (float 1.638400e+04), !dbg !6745
  %498 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %497), !dbg !6749
  %499 = fmul <4 x float> %498, splat (float 0x3F10000000000000), !dbg !6756
  %500 = icmp eq i32 %width.i31.i826, 0, !dbg !6760
  %_163.1.i80.i875.pre = load i32, ptr %147, align 4, !dbg !6765, !alias.scope !6573, !noalias !6574
  br i1 %500, label %bb53.i75.i870, label %bb36.i54.i849.lr.ph, !dbg !6760

bb36.i54.i849.lr.ph:                              ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1693
  %_159.1.i59.i854 = load i32, ptr %62, align 4, !alias.scope !6573, !noalias !6574, !noundef !10
  %_159.0.i63.i858 = load ptr, ptr %61, align 4, !nonnull !10
  %_161.0.i73.i868 = load ptr, ptr %148, align 4, !nonnull !10
  %exitcond20477.not = icmp eq i32 %_159.1.i59.i854, 0, !dbg !6766
  br i1 %exitcond20477.not, label %panic.i61.i856, label %bb14.i62.i857, !dbg !6766

bb34.i123.i1025:                                  ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5191
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %420, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i37.i832, i32 noundef %_158.1.i36.i831, i32 noundef %_158.1.i36.i831, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_db7c42041d7aaaba4839906f37294547) #33, !dbg !6768, !noalias !6769
  unreachable, !dbg !6768

bb53.i75.i870.loopexit:                           ; preds = %bb18.i72.i867.7, %bb18.i72.i867.6, %bb18.i72.i867.5, %bb18.i72.i867.4, %bb18.i72.i867.3, %bb18.i72.i867.2, %bb18.i72.i867.1, %bb18.i72.i867
  %lanes.i5170.sroa.0.0.copyload.pre = load <4 x float>, ptr %scratch.i758, align 4, !dbg !6770, !alias.scope !6775, !noalias !6779
  br label %bb53.i75.i870, !dbg !6783

bb53.i75.i870:                                    ; preds = %bb53.i75.i870.loopexit, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1693
  %lanes.i5170.sroa.0.0.copyload = phi <4 x float> [ %lanes.i5170.sroa.0.0.copyload.pre, %bb53.i75.i870.loopexit ], [ %lanes.i5177.sroa.0.0.copyload, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1693 ], !dbg !6770
  %501 = fadd <4 x float> %499, %420, !dbg !6784
  %502 = fsub <4 x float> %501, %lanes.i5170.sroa.0.0.copyload, !dbg !6788
  %_123.i81.i876 = icmp ugt i32 %_22.i37.i832, %_163.1.i80.i875.pre, !dbg !6792
  br i1 %_123.i81.i876, label %bb41.i122.i1024, label %bb42.i82.i877, !dbg !6792, !prof !902

bb14.i62.i857:                                    ; preds = %bb36.i54.i849.lr.ph
  %503 = getelementptr inbounds nuw i8, ptr %_159.0.i63.i858, i32 8, !dbg !6766
  %_42.i64.i859 = load i32, ptr %503, align 4, !dbg !6766, !noalias !6796, !noundef !10
  %504 = add i32 %_42.i64.i859, %ring_cursor.sroa.0.1.i79217813, !dbg !6797
  %_45.not.i65.i860 = icmp ult i32 %504, %_87.i824, !dbg !6798
  %505 = select i1 %_45.not.i65.i860, i32 0, i32 %_87.i824, !dbg !6798
  %spec.select.i66.i861 = sub nuw i32 %504, %505, !dbg !6798
  %_49.i67.i862 = mul i32 %spec.select.i66.i861, %width.i31.i826, !dbg !6800
  %_51.i70.i865 = icmp ult i32 %_49.i67.i862, %_163.1.i80.i875.pre, !dbg !6801
  br i1 %_51.i70.i865, label %bb18.i72.i867, label %panic1.i71.i866, !dbg !6801

panic.i61.i856:                                   ; preds = %bb36.i54.i849.7, %bb36.i54.i849.6, %bb36.i54.i849.5, %bb36.i54.i849.4, %bb36.i54.i849.3, %bb36.i54.i849.2, %bb36.i54.i849.1, %bb36.i54.i849.lr.ph
  %_159.1.i59.i854.lcssa.ph = phi i32 [ 7, %bb36.i54.i849.7 ], [ 6, %bb36.i54.i849.6 ], [ 5, %bb36.i54.i849.5 ], [ 4, %bb36.i54.i849.4 ], [ 3, %bb36.i54.i849.3 ], [ 2, %bb36.i54.i849.2 ], [ 1, %bb36.i54.i849.1 ], [ 0, %bb36.i54.i849.lr.ph ]
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %420, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_159.1.i59.i854.lcssa.ph, i32 noundef %_159.1.i59.i854.lcssa.ph, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_33746731a929bad63709ddedbca82f5f) #33, !dbg !6766, !noalias !6796
  unreachable, !dbg !6766

bb18.i72.i867:                                    ; preds = %bb14.i62.i857
  %506 = getelementptr inbounds nuw float, ptr %_161.0.i73.i868, i32 %_49.i67.i862, !dbg !6801
  %_47.i74.i869 = load float, ptr %506, align 4, !dbg !6801, !noalias !6796, !noundef !10
  store float %_47.i74.i869, ptr %scratch.i758, align 4, !dbg !6802, !alias.scope !6570, !noalias !6803
  %507 = icmp eq i32 %width.i31.i826, 1, !dbg !6760
  br i1 %507, label %bb53.i75.i870.loopexit, label %bb36.i54.i849.1, !dbg !6760

bb36.i54.i849.1:                                  ; preds = %bb18.i72.i867
  %exitcond20477.1.not = icmp eq i32 %_159.1.i59.i854, 1, !dbg !6766
  br i1 %exitcond20477.1.not, label %panic.i61.i856, label %bb14.i62.i857.1, !dbg !6766

bb14.i62.i857.1:                                  ; preds = %bb36.i54.i849.1
  %508 = getelementptr inbounds nuw i8, ptr %_159.0.i63.i858, i32 20, !dbg !6766
  %_42.i64.i859.1 = load i32, ptr %508, align 4, !dbg !6766, !noalias !6796, !noundef !10
  %509 = add i32 %_42.i64.i859.1, %ring_cursor.sroa.0.1.i79217813, !dbg !6797
  %_45.not.i65.i860.1 = icmp ult i32 %509, %_87.i824, !dbg !6798
  %510 = select i1 %_45.not.i65.i860.1, i32 0, i32 %_87.i824, !dbg !6798
  %spec.select.i66.i861.1 = sub nuw i32 %509, %510, !dbg !6798
  %_49.i67.i862.1 = mul i32 %spec.select.i66.i861.1, %width.i31.i826, !dbg !6800
  %_48.i68.i863.1 = add i32 %_49.i67.i862.1, 1, !dbg !6800
  %_51.i70.i865.1 = icmp ult i32 %_48.i68.i863.1, %_163.1.i80.i875.pre, !dbg !6801
  br i1 %_51.i70.i865.1, label %bb18.i72.i867.1, label %panic1.i71.i866, !dbg !6801

bb18.i72.i867.1:                                  ; preds = %bb14.i62.i857.1
  %511 = getelementptr inbounds nuw float, ptr %_161.0.i73.i868, i32 %_48.i68.i863.1, !dbg !6801
  %_47.i74.i869.1 = load float, ptr %511, align 4, !dbg !6801, !noalias !6796, !noundef !10
  store float %_47.i74.i869.1, ptr %iter.sroa.0.0.ptr.i53.i84817798.1, align 4, !dbg !6802, !alias.scope !6570, !noalias !6803
  %512 = icmp eq i32 %width.i31.i826, 2, !dbg !6760
  br i1 %512, label %bb53.i75.i870.loopexit, label %bb36.i54.i849.2, !dbg !6760

bb36.i54.i849.2:                                  ; preds = %bb18.i72.i867.1
  %exitcond20477.2.not = icmp eq i32 %_159.1.i59.i854, 2, !dbg !6766
  br i1 %exitcond20477.2.not, label %panic.i61.i856, label %bb14.i62.i857.2, !dbg !6766

bb14.i62.i857.2:                                  ; preds = %bb36.i54.i849.2
  %513 = getelementptr inbounds nuw i8, ptr %_159.0.i63.i858, i32 32, !dbg !6766
  %_42.i64.i859.2 = load i32, ptr %513, align 4, !dbg !6766, !noalias !6796, !noundef !10
  %514 = add i32 %_42.i64.i859.2, %ring_cursor.sroa.0.1.i79217813, !dbg !6797
  %_45.not.i65.i860.2 = icmp ult i32 %514, %_87.i824, !dbg !6798
  %515 = select i1 %_45.not.i65.i860.2, i32 0, i32 %_87.i824, !dbg !6798
  %spec.select.i66.i861.2 = sub nuw i32 %514, %515, !dbg !6798
  %_49.i67.i862.2 = mul i32 %spec.select.i66.i861.2, %width.i31.i826, !dbg !6800
  %_48.i68.i863.2 = add i32 %_49.i67.i862.2, 2, !dbg !6800
  %_51.i70.i865.2 = icmp ult i32 %_48.i68.i863.2, %_163.1.i80.i875.pre, !dbg !6801
  br i1 %_51.i70.i865.2, label %bb18.i72.i867.2, label %panic1.i71.i866, !dbg !6801

bb18.i72.i867.2:                                  ; preds = %bb14.i62.i857.2
  %516 = getelementptr inbounds nuw float, ptr %_161.0.i73.i868, i32 %_48.i68.i863.2, !dbg !6801
  %_47.i74.i869.2 = load float, ptr %516, align 4, !dbg !6801, !noalias !6796, !noundef !10
  store float %_47.i74.i869.2, ptr %iter.sroa.0.0.ptr.i53.i84817798.2, align 4, !dbg !6802, !alias.scope !6570, !noalias !6803
  %517 = icmp eq i32 %width.i31.i826, 3, !dbg !6760
  br i1 %517, label %bb53.i75.i870.loopexit, label %bb36.i54.i849.3, !dbg !6760

bb36.i54.i849.3:                                  ; preds = %bb18.i72.i867.2
  %exitcond20477.3.not = icmp eq i32 %_159.1.i59.i854, 3, !dbg !6766
  br i1 %exitcond20477.3.not, label %panic.i61.i856, label %bb14.i62.i857.3, !dbg !6766

bb14.i62.i857.3:                                  ; preds = %bb36.i54.i849.3
  %518 = getelementptr inbounds nuw i8, ptr %_159.0.i63.i858, i32 44, !dbg !6766
  %_42.i64.i859.3 = load i32, ptr %518, align 4, !dbg !6766, !noalias !6796, !noundef !10
  %519 = add i32 %_42.i64.i859.3, %ring_cursor.sroa.0.1.i79217813, !dbg !6797
  %_45.not.i65.i860.3 = icmp ult i32 %519, %_87.i824, !dbg !6798
  %520 = select i1 %_45.not.i65.i860.3, i32 0, i32 %_87.i824, !dbg !6798
  %spec.select.i66.i861.3 = sub nuw i32 %519, %520, !dbg !6798
  %_49.i67.i862.3 = mul i32 %spec.select.i66.i861.3, %width.i31.i826, !dbg !6800
  %_48.i68.i863.3 = add i32 %_49.i67.i862.3, 3, !dbg !6800
  %_51.i70.i865.3 = icmp ult i32 %_48.i68.i863.3, %_163.1.i80.i875.pre, !dbg !6801
  br i1 %_51.i70.i865.3, label %bb18.i72.i867.3, label %panic1.i71.i866, !dbg !6801

bb18.i72.i867.3:                                  ; preds = %bb14.i62.i857.3
  %521 = getelementptr inbounds nuw float, ptr %_161.0.i73.i868, i32 %_48.i68.i863.3, !dbg !6801
  %_47.i74.i869.3 = load float, ptr %521, align 4, !dbg !6801, !noalias !6796, !noundef !10
  store float %_47.i74.i869.3, ptr %iter.sroa.0.0.ptr.i53.i84817798.3, align 4, !dbg !6802, !alias.scope !6570, !noalias !6803
  %522 = icmp eq i32 %width.i31.i826, 4, !dbg !6760
  br i1 %522, label %bb53.i75.i870.loopexit, label %bb36.i54.i849.4, !dbg !6760

bb36.i54.i849.4:                                  ; preds = %bb18.i72.i867.3
  %exitcond20477.4.not = icmp eq i32 %_159.1.i59.i854, 4, !dbg !6766
  br i1 %exitcond20477.4.not, label %panic.i61.i856, label %bb14.i62.i857.4, !dbg !6766

bb14.i62.i857.4:                                  ; preds = %bb36.i54.i849.4
  %523 = getelementptr inbounds nuw i8, ptr %_159.0.i63.i858, i32 56, !dbg !6766
  %_42.i64.i859.4 = load i32, ptr %523, align 4, !dbg !6766, !noalias !6796, !noundef !10
  %524 = add i32 %_42.i64.i859.4, %ring_cursor.sroa.0.1.i79217813, !dbg !6797
  %_45.not.i65.i860.4 = icmp ult i32 %524, %_87.i824, !dbg !6798
  %525 = select i1 %_45.not.i65.i860.4, i32 0, i32 %_87.i824, !dbg !6798
  %spec.select.i66.i861.4 = sub nuw i32 %524, %525, !dbg !6798
  %_49.i67.i862.4 = mul i32 %spec.select.i66.i861.4, %width.i31.i826, !dbg !6800
  %_48.i68.i863.4 = add i32 %_49.i67.i862.4, 4, !dbg !6800
  %_51.i70.i865.4 = icmp ult i32 %_48.i68.i863.4, %_163.1.i80.i875.pre, !dbg !6801
  br i1 %_51.i70.i865.4, label %bb18.i72.i867.4, label %panic1.i71.i866, !dbg !6801

bb18.i72.i867.4:                                  ; preds = %bb14.i62.i857.4
  %526 = getelementptr inbounds nuw float, ptr %_161.0.i73.i868, i32 %_48.i68.i863.4, !dbg !6801
  %_47.i74.i869.4 = load float, ptr %526, align 4, !dbg !6801, !noalias !6796, !noundef !10
  store float %_47.i74.i869.4, ptr %iter.sroa.0.0.ptr.i53.i84817798.4, align 4, !dbg !6802, !alias.scope !6570, !noalias !6803
  %527 = icmp eq i32 %width.i31.i826, 5, !dbg !6760
  br i1 %527, label %bb53.i75.i870.loopexit, label %bb36.i54.i849.5, !dbg !6760

bb36.i54.i849.5:                                  ; preds = %bb18.i72.i867.4
  %exitcond20477.5.not = icmp eq i32 %_159.1.i59.i854, 5, !dbg !6766
  br i1 %exitcond20477.5.not, label %panic.i61.i856, label %bb14.i62.i857.5, !dbg !6766

bb14.i62.i857.5:                                  ; preds = %bb36.i54.i849.5
  %528 = getelementptr inbounds nuw i8, ptr %_159.0.i63.i858, i32 68, !dbg !6766
  %_42.i64.i859.5 = load i32, ptr %528, align 4, !dbg !6766, !noalias !6796, !noundef !10
  %529 = add i32 %_42.i64.i859.5, %ring_cursor.sroa.0.1.i79217813, !dbg !6797
  %_45.not.i65.i860.5 = icmp ult i32 %529, %_87.i824, !dbg !6798
  %530 = select i1 %_45.not.i65.i860.5, i32 0, i32 %_87.i824, !dbg !6798
  %spec.select.i66.i861.5 = sub nuw i32 %529, %530, !dbg !6798
  %_49.i67.i862.5 = mul i32 %spec.select.i66.i861.5, %width.i31.i826, !dbg !6800
  %_48.i68.i863.5 = add i32 %_49.i67.i862.5, 5, !dbg !6800
  %_51.i70.i865.5 = icmp ult i32 %_48.i68.i863.5, %_163.1.i80.i875.pre, !dbg !6801
  br i1 %_51.i70.i865.5, label %bb18.i72.i867.5, label %panic1.i71.i866, !dbg !6801

bb18.i72.i867.5:                                  ; preds = %bb14.i62.i857.5
  %531 = getelementptr inbounds nuw float, ptr %_161.0.i73.i868, i32 %_48.i68.i863.5, !dbg !6801
  %_47.i74.i869.5 = load float, ptr %531, align 4, !dbg !6801, !noalias !6796, !noundef !10
  store float %_47.i74.i869.5, ptr %iter.sroa.0.0.ptr.i53.i84817798.5, align 4, !dbg !6802, !alias.scope !6570, !noalias !6803
  %532 = icmp eq i32 %width.i31.i826, 6, !dbg !6760
  br i1 %532, label %bb53.i75.i870.loopexit, label %bb36.i54.i849.6, !dbg !6760

bb36.i54.i849.6:                                  ; preds = %bb18.i72.i867.5
  %exitcond20477.6.not = icmp eq i32 %_159.1.i59.i854, 6, !dbg !6766
  br i1 %exitcond20477.6.not, label %panic.i61.i856, label %bb14.i62.i857.6, !dbg !6766

bb14.i62.i857.6:                                  ; preds = %bb36.i54.i849.6
  %533 = getelementptr inbounds nuw i8, ptr %_159.0.i63.i858, i32 80, !dbg !6766
  %_42.i64.i859.6 = load i32, ptr %533, align 4, !dbg !6766, !noalias !6796, !noundef !10
  %534 = add i32 %_42.i64.i859.6, %ring_cursor.sroa.0.1.i79217813, !dbg !6797
  %_45.not.i65.i860.6 = icmp ult i32 %534, %_87.i824, !dbg !6798
  %535 = select i1 %_45.not.i65.i860.6, i32 0, i32 %_87.i824, !dbg !6798
  %spec.select.i66.i861.6 = sub nuw i32 %534, %535, !dbg !6798
  %_49.i67.i862.6 = mul i32 %spec.select.i66.i861.6, %width.i31.i826, !dbg !6800
  %_48.i68.i863.6 = add i32 %_49.i67.i862.6, 6, !dbg !6800
  %_51.i70.i865.6 = icmp ult i32 %_48.i68.i863.6, %_163.1.i80.i875.pre, !dbg !6801
  br i1 %_51.i70.i865.6, label %bb18.i72.i867.6, label %panic1.i71.i866, !dbg !6801

bb18.i72.i867.6:                                  ; preds = %bb14.i62.i857.6
  %536 = getelementptr inbounds nuw float, ptr %_161.0.i73.i868, i32 %_48.i68.i863.6, !dbg !6801
  %_47.i74.i869.6 = load float, ptr %536, align 4, !dbg !6801, !noalias !6796, !noundef !10
  store float %_47.i74.i869.6, ptr %iter.sroa.0.0.ptr.i53.i84817798.6, align 4, !dbg !6802, !alias.scope !6570, !noalias !6803
  %537 = icmp eq i32 %width.i31.i826, 7, !dbg !6760
  br i1 %537, label %bb53.i75.i870.loopexit, label %bb36.i54.i849.7, !dbg !6760

bb36.i54.i849.7:                                  ; preds = %bb18.i72.i867.6
  %exitcond20477.7.not = icmp eq i32 %_159.1.i59.i854, 7, !dbg !6766
  br i1 %exitcond20477.7.not, label %panic.i61.i856, label %bb14.i62.i857.7, !dbg !6766

bb14.i62.i857.7:                                  ; preds = %bb36.i54.i849.7
  %538 = getelementptr inbounds nuw i8, ptr %_159.0.i63.i858, i32 92, !dbg !6766
  %_42.i64.i859.7 = load i32, ptr %538, align 4, !dbg !6766, !noalias !6796, !noundef !10
  %539 = add i32 %_42.i64.i859.7, %ring_cursor.sroa.0.1.i79217813, !dbg !6797
  %_45.not.i65.i860.7 = icmp ult i32 %539, %_87.i824, !dbg !6798
  %540 = select i1 %_45.not.i65.i860.7, i32 0, i32 %_87.i824, !dbg !6798
  %spec.select.i66.i861.7 = sub nuw i32 %539, %540, !dbg !6798
  %_49.i67.i862.7 = mul i32 %spec.select.i66.i861.7, %width.i31.i826, !dbg !6800
  %_48.i68.i863.7 = add i32 %_49.i67.i862.7, 7, !dbg !6800
  %_51.i70.i865.7 = icmp ult i32 %_48.i68.i863.7, %_163.1.i80.i875.pre, !dbg !6801
  br i1 %_51.i70.i865.7, label %bb18.i72.i867.7, label %panic1.i71.i866, !dbg !6801

bb18.i72.i867.7:                                  ; preds = %bb14.i62.i857.7
  %541 = getelementptr inbounds nuw float, ptr %_161.0.i73.i868, i32 %_48.i68.i863.7, !dbg !6801
  %_47.i74.i869.7 = load float, ptr %541, align 4, !dbg !6801, !noalias !6796, !noundef !10
  store float %_47.i74.i869.7, ptr %iter.sroa.0.0.ptr.i53.i84817798.7, align 4, !dbg !6802, !alias.scope !6570, !noalias !6803
  br label %bb53.i75.i870.loopexit, !dbg !6760

panic1.i71.i866:                                  ; preds = %bb14.i62.i857.7, %bb14.i62.i857.6, %bb14.i62.i857.5, %bb14.i62.i857.4, %bb14.i62.i857.3, %bb14.i62.i857.2, %bb14.i62.i857.1, %bb14.i62.i857
  %_48.i68.i863.lcssa.ph = phi i32 [ %_48.i68.i863.7, %bb14.i62.i857.7 ], [ %_48.i68.i863.6, %bb14.i62.i857.6 ], [ %_48.i68.i863.5, %bb14.i62.i857.5 ], [ %_48.i68.i863.4, %bb14.i62.i857.4 ], [ %_48.i68.i863.3, %bb14.i62.i857.3 ], [ %_48.i68.i863.2, %bb14.i62.i857.2 ], [ %_48.i68.i863.1, %bb14.i62.i857.1 ], [ %_49.i67.i862, %bb14.i62.i857 ]
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %420, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_48.i68.i863.lcssa.ph, i32 noundef %_163.1.i80.i875.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_0e76a26fbb751dfb8cf8a069b5b0d39a) #33, !dbg !6801, !noalias !6796
  unreachable, !dbg !6801

bb42.i82.i877:                                    ; preds = %bb53.i75.i870
  %_126.i84.i879 = sub nuw i32 %_163.1.i80.i875.pre, %_22.i37.i832, !dbg !6804
  %_8.i5919 = icmp samesign ugt i32 %_126.i84.i879, 3, !dbg !6805
  br i1 %_8.i5919, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5922, label %bb2.i5920, !dbg !6805, !prof !1153

bb2.i5920:                                        ; preds = %bb42.i82.i877
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %502, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_126.i84.i879, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !6810, !noalias !6811
  unreachable, !dbg !6810

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5922: ; preds = %bb42.i82.i877
  %_163.0.i83.i878 = load ptr, ptr %148, align 4, !dbg !6765, !alias.scope !6573, !noalias !6574, !nonnull !10, !noundef !10
  %_130.i85.i880 = getelementptr inbounds nuw float, ptr %_163.0.i83.i878, i32 %_22.i37.i832, !dbg !6815
  store <4 x float> %499, ptr %_130.i85.i880, align 4, !dbg !6820, !alias.scope !6824, !noalias !6828
  %_62.i87.i88215105 = load <4 x float>, ptr %150, align 16, !dbg !6830
  %_66.i90.i88515106 = load <4 x float>, ptr %151, align 16, !dbg !6831
  %542 = fdiv <4 x float> %502, %_62.i87.i88215105, !dbg !6834
  %543 = fsub <4 x float> splat (float 1.000000e+00), %542, !dbg !6838
  %544 = fsub <4 x float> %543, %_66.i90.i88515106, !dbg !6842
  %545 = bitcast <16 x i8> %_4.i6460 to <4 x float>, !dbg !6846
  %546 = fmul <4 x float> %544, %545, !dbg !6852
  %547 = fadd <4 x float> %_66.i90.i88515106, %546, !dbg !6853
  %548 = fcmp olt <4 x float> %547, %543, !dbg !6858
  %549 = select <4 x i1> %548, <4 x float> %543, <4 x float> %547, !dbg !6863
  %550 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %549), !dbg !6864
  %551 = fcmp uge <4 x float> %550, splat (float 0x3BC79CA100000000), !dbg !6871
  %552 = bitcast <4 x float> %549 to <4 x i32>, !dbg !6876
  %553 = select <4 x i1> %551, <4 x i32> %552, <4 x i32> zeroinitializer, !dbg !6876
  store <4 x i32> %553, ptr %151, align 16, !dbg !6881
  %554 = bitcast <4 x i32> %553 to <4 x float>, !dbg !6882
  %555 = fsub <4 x float> splat (float 1.000000e+00), %554, !dbg !6886
  %_164.1.i100.i895 = load i32, ptr %152, align 4, !dbg !6887, !alias.scope !6573, !noalias !6574, !noundef !10
  %_74.i101.i896 = mul i32 %width.i31.i826, %main_cursor.sroa.0.1.i79317814, !dbg !6889
  %_134.i102.i897 = icmp ugt i32 %_74.i101.i896, %_164.1.i100.i895, !dbg !6890
  br i1 %_134.i102.i897, label %bb47.i121.i1023, label %bb48.i103.i898, !dbg !6890, !prof !902

bb41.i122.i1024:                                  ; preds = %bb53.i75.i870
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %502, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i37.i832, i32 noundef %_163.1.i80.i875.pre, i32 noundef %_163.1.i80.i875.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c2286ff41a33b8c11aa270fe215441de) #33, !dbg !6895, !noalias !6796
  unreachable, !dbg !6895

bb48.i103.i898:                                   ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5922
  %_137.i105.i900 = sub nuw i32 %_164.1.i100.i895, %_74.i101.i896, !dbg !6896
  %_8.i5164 = icmp samesign ugt i32 %_137.i105.i900, 3, !dbg !6897
  br i1 %_8.i5164, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5912, label %bb2.i5165, !dbg !6897, !prof !1153

bb2.i5165:                                        ; preds = %bb48.i103.i898
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %502, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_137.i105.i900, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !6902, !noalias !6903
  unreachable, !dbg !6902

bb47.i121.i1023:                                  ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5922
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %502, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_74.i101.i896, i32 noundef %_164.1.i100.i895, i32 noundef %_164.1.i100.i895, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a3a21efd18b403ec25db545d522c479) #33, !dbg !6907, !noalias !6796
  unreachable, !dbg !6907

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5912: ; preds = %bb48.i103.i898
  %_164.0.i104.i899 = load ptr, ptr %153, align 4, !dbg !6887, !alias.scope !6573, !noalias !6574, !nonnull !10, !noundef !10
  %_141.i106.i901 = getelementptr inbounds nuw float, ptr %_164.0.i104.i899, i32 %_74.i101.i896, !dbg !6908
  %lanes.i5161.sroa.0.0.copyload = load <4 x i32>, ptr %_141.i106.i901, align 4, !dbg !6913, !alias.scope !6917, !noalias !6921
  store <4 x i32> %lanes.i5184.sroa.0.0.copyload, ptr %_141.i106.i901, align 4, !dbg !6923, !alias.scope !6929, !noalias !6933
  %556 = bitcast <4 x i32> %lanes.i5161.sroa.0.0.copyload to <4 x float>, !dbg !6937
  %557 = fmul <4 x float> %555, %556, !dbg !6941
  %558 = bitcast <4 x i32> %lanes.i5161.sroa.0.0.copyload to <16 x i8>, !dbg !6942
  %559 = bitcast <4 x float> %557 to <16 x i8>, !dbg !6946
  %_4.i6478 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %558, <16 x i8> %559, <16 x i8> %154), !dbg !6947
  store <16 x i8> %_4.i6478, ptr %_139.i822, align 4, !dbg !6948, !alias.scope !6953, !noalias !6957
  %_140.i915 = icmp ugt i32 %base.i797, %right_io.1, !dbg !6961
  br i1 %_140.i915, label %bb48.i1020, label %bb49.i916, !dbg !6961, !prof !902

bb46.i1026:                                       ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5209
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %420, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i797, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_f462f4e49ffb40af04504eaf795aa606) #33, !dbg !6965, !noalias !6966
  unreachable, !dbg !6965

bb49.i916:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5912
  %_143.i917 = sub nuw nsw i32 %right_io.1, %base.i797, !dbg !6967
  %_147.i918 = getelementptr inbounds nuw float, ptr %right_io.0, i32 %base.i797, !dbg !6968
  %_8.i5155 = icmp samesign ugt i32 %_143.i917, 3, !dbg !6973
  br i1 %_8.i5155, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5159, label %bb2.i5156, !dbg !6973, !prof !1153

bb2.i5156:                                        ; preds = %bb49.i916
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %502, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_143.i917, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !6978, !noalias !6979
  unreachable, !dbg !6978

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5159: ; preds = %bb49.i916
  %lanes.i5152.sroa.0.0.copyload = load <4 x i32>, ptr %_147.i918, align 4, !dbg !6983, !alias.scope !6987, !noalias !6991
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6993), !dbg !6996
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6997), !dbg !6996
  %width.i.i921 = load i32, ptr %155, align 4, !dbg !6999, !alias.scope !7000, !noalias !7001, !noundef !10
  %560 = bitcast <16 x i8> %_4.i6469 to <4 x float>, !dbg !7009
  %561 = bitcast <16 x i8> %_4.i6462 to <4 x float>, !dbg !7014
  %562 = fcmp ogt <4 x float> %560, %561, !dbg !7015
  %563 = sext <4 x i1> %562 to <4 x i32>, !dbg !7015
  %564 = fdiv <4 x float> %561, %560, !dbg !7016
  %565 = bitcast <4 x float> %564 to <16 x i8>, !dbg !7020
  %566 = bitcast <4 x i32> %563 to <16 x i8>, !dbg !7024
  %_4.i6481 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %565, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %566), !dbg !7025
  %_158.1.i.i926 = load i32, ptr %156, align 4, !dbg !7026, !alias.scope !7000, !noalias !7001, !noundef !10
  %_22.i.i927 = mul i32 %width.i.i921, %ring_cursor.sroa.0.1.i79217813, !dbg !7027
  %_90.i.i928 = icmp ugt i32 %_22.i.i927, %_158.1.i.i926, !dbg !7028
  br i1 %_90.i.i928, label %bb34.i.i1019, label %bb35.i.i929, !dbg !7028, !prof !902

bb35.i.i929:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5159
  %_93.i.i931 = sub nuw i32 %_158.1.i.i926, %_22.i.i927, !dbg !7031
  %_8.i5904 = icmp samesign ugt i32 %_93.i.i931, 3, !dbg !7032
  br i1 %_8.i5904, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5907, label %bb2.i5905, !dbg !7032, !prof !1153

bb2.i5905:                                        ; preds = %bb35.i.i929
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %502, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_93.i.i931, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !7037, !noalias !7038
  unreachable, !dbg !7037

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5907: ; preds = %bb35.i.i929
  %_158.0.i.i930 = load ptr, ptr %157, align 4, !dbg !7026, !alias.scope !7000, !noalias !7001, !nonnull !10, !noundef !10
  %_97.i.i932 = getelementptr inbounds nuw float, ptr %_158.0.i.i930, i32 %_22.i.i927, !dbg !7042
  store <16 x i8> %_4.i6481, ptr %_97.i.i932, align 4, !dbg !7044, !alias.scope !7048, !noalias !7052
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7054), !dbg !7057
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7058), !dbg !7057
  %width.i = load i32, ptr %155, align 4, !dbg !7060, !alias.scope !7054, !noalias !7062, !noundef !10
  %567 = icmp eq i32 %width.i, 0, !dbg !7063
  br i1 %567, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit, label %bb29.i.lr.ph, !dbg !7063

bb29.i.lr.ph:                                     ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5907
  %_126.1.i = load i32, ptr %158, align 4, !alias.scope !7054, !noalias !7062, !noundef !10
  %_126.0.i = load ptr, ptr %159, align 4, !nonnull !10
  %568 = add i32 %ring_cursor.sroa.0.1.i79217813, 1
  %_21.not.i = icmp ult i32 %568, %_87.i824
  %569 = select i1 %_21.not.i, i32 0, i32 %_87.i824
  %start1.sroa.0.0.i = sub nuw i32 %568, %569
  %_128.1.i = load i32, ptr %156, align 4
  %_128.0.i = load ptr, ptr %157, align 4, !nonnull !10
  %_130.1.i = load i32, ptr %160, align 4
  %_130.0.i = load ptr, ptr %161, align 4, !nonnull !10
  %_132.1.i = load i32, ptr %162, align 4
  %_132.0.i = load ptr, ptr %163, align 4, !nonnull !10
  %_43.i = mul i32 %width.i, %start1.sroa.0.0.i
  br label %bb29.i, !dbg !7063

bb29.i:                                           ; preds = %bb29.i.lr.ph, %bb28.i
  %iter.sroa.0.0.idx.i17805 = phi i32 [ 0, %bb29.i.lr.ph ], [ %iter.sroa.0.0.add.i, %bb28.i ]
  %iter.sroa.4.0.i17804 = phi i32 [ 0, %bb29.i.lr.ph ], [ %_102.0.i, %bb28.i ]
  %iter.sroa.7.0.i17803 = phi i32 [ %width.i, %bb29.i.lr.ph ], [ %570, %bb28.i ]
  %iter.sroa.0.0.ptr.i17806 = getelementptr inbounds nuw i8, ptr %scratch.i758, i32 %iter.sroa.0.0.idx.i17805, !dbg !7065
  %570 = add i32 %iter.sroa.7.0.i17803, -1, !dbg !7065
  %_109.i1564 = icmp eq i32 %iter.sroa.0.0.idx.i17805, 32, !dbg !7066
  br i1 %_109.i1564, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit, label %bb33.i, !dbg !7070

bb33.i:                                           ; preds = %bb29.i
  %iter.sroa.0.0.add.i = add nuw nsw i32 %iter.sroa.0.0.idx.i17805, 4, !dbg !7071
  %_102.0.i = add nuw nsw i32 %iter.sroa.4.0.i17804, 1, !dbg !7073
  %exitcond20481.not = icmp eq i32 %iter.sroa.4.0.i17804, %_126.1.i, !dbg !7074
  br i1 %exitcond20481.not, label %panic.i, label %bb2.i1566, !dbg !7074

bb2.i1566:                                        ; preds = %bb33.i
  %571 = getelementptr inbounds nuw %LaneShape, ptr %_126.0.i, i32 %iter.sroa.4.0.i17804, !dbg !7074
  %shape.i = load i32, ptr %571, align 4, !dbg !7074, !noalias !7075, !noundef !10
  %572 = getelementptr inbounds nuw i8, ptr %571, i32 4, !dbg !7074
  %shape3.i = load i32, ptr %572, align 4, !dbg !7074, !noalias !7075, !noundef !10
  %573 = add i32 %shape3.i, %ring_cursor.sroa.0.1.i79217813, !dbg !7076
  %_18.not.i = icmp ult i32 %573, %_87.i824, !dbg !7077
  %574 = select i1 %_18.not.i, i32 0, i32 %_87.i824, !dbg !7077
  %spec.select.i = sub nuw i32 %573, %574, !dbg !7077
  %_25.i = mul i32 %spec.select.i, %width.i, !dbg !7078
  %_24.i = add i32 %_25.i, %iter.sroa.4.0.i17804, !dbg !7078
  %_28.i1567 = icmp ult i32 %_24.i, %_128.1.i, !dbg !7079
  br i1 %_28.i1567, label %bb9.i, label %panic5.i, !dbg !7079

panic.i:                                          ; preds = %bb33.i
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %502, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_126.1.i, i32 noundef %_126.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_12856b5f9033764dbf23e04eb77c5c46) #33, !dbg !7074, !noalias !7075
  unreachable, !dbg !7074

bb9.i:                                            ; preds = %bb2.i1566
  %575 = getelementptr inbounds nuw float, ptr %_128.0.i, i32 %_24.i, !dbg !7079
  %576 = load float, ptr %575, align 4, !dbg !7079, !noalias !7075, !noundef !10
  %exitcond20482.not = icmp eq i32 %iter.sroa.4.0.i17804, %_130.1.i, !dbg !7080
  br i1 %exitcond20482.not, label %panic6.i, label %bb10.i1569, !dbg !7080

panic5.i:                                         ; preds = %bb2.i1566
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %502, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_24.i, i32 noundef %_128.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70ebf0cf1c90c6161926611ccf249a32) #33, !dbg !7079, !noalias !7075
  unreachable, !dbg !7079

bb10.i1569:                                       ; preds = %bb9.i
  %577 = getelementptr inbounds nuw i32, ptr %_130.0.i, i32 %iter.sroa.4.0.i17804, !dbg !7080
  %_30.i1570 = load i32, ptr %577, align 4, !dbg !7080, !noalias !7075, !noundef !10
  %578 = icmp eq i32 %_30.i1570, 0, !dbg !7081
  br i1 %578, label %bb14.i, label %bb12.i1571, !dbg !7081

panic6.i:                                         ; preds = %bb9.i
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %502, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_130.1.i, i32 noundef %_130.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_023d591aaad7f5cf482ac80a9401eca1) #33, !dbg !7080, !noalias !7075
  unreachable, !dbg !7080

bb12.i1571:                                       ; preds = %bb10.i1569
  %_35.i1572 = icmp ult i32 %iter.sroa.4.0.i17804, %_132.1.i, !dbg !7082
  br i1 %_35.i1572, label %bb13.i1573, label %panic7.i, !dbg !7082

bb14.i:                                           ; preds = %bb34.i, %bb13.i1573, %bb10.i1569
  %newest.sroa.0.0.i = phi float [ %576, %bb10.i1569 ], [ %_33.i1574, %bb34.i ], [ %576, %bb13.i1573 ], !dbg !7083
  %exitcond20483.not = icmp eq i32 %iter.sroa.4.0.i17804, %_132.1.i, !dbg !7084
  br i1 %exitcond20483.not, label %panic8.i, label %bb15.i1576, !dbg !7084

bb13.i1573:                                       ; preds = %bb12.i1571
  %579 = getelementptr inbounds nuw float, ptr %_132.0.i, i32 %iter.sroa.4.0.i17804, !dbg !7082
  %_33.i1574 = load float, ptr %579, align 4, !dbg !7082, !noalias !7075, !noundef !10
  %_116.i1575 = fcmp olt float %_33.i1574, %576, !dbg !7085
  br i1 %_116.i1575, label %bb34.i, label %bb14.i, !dbg !7085

panic7.i:                                         ; preds = %bb12.i1571
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %502, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %iter.sroa.4.0.i17804, i32 noundef %_132.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a228cac3279aae3a34886f59f51fbe9e) #33, !dbg !7082, !noalias !7075
  unreachable, !dbg !7082

bb34.i:                                           ; preds = %bb13.i1573
  br label %bb14.i, !dbg !7087

bb15.i1576:                                       ; preds = %bb14.i
  %580 = getelementptr inbounds nuw float, ptr %_132.0.i, i32 %iter.sroa.4.0.i17804, !dbg !7084
  store float %newest.sroa.0.0.i, ptr %580, align 4, !dbg !7084, !noalias !7075
  %_40.i = add i32 %_30.i1570, 1, !dbg !7088
  %complete.i1577 = icmp eq i32 %_40.i, %shape.i, !dbg !7088
  br i1 %complete.i1577, label %bb19.i1580, label %bb17.i, !dbg !7089

panic8.i:                                         ; preds = %bb14.i
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %502, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_132.1.i, i32 noundef %_132.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1fa5d88c1a2cc292a2767c48606a38fe) #33, !dbg !7084, !noalias !7075
  unreachable, !dbg !7084

bb17.i:                                           ; preds = %bb15.i1576
  %_42.i = add i32 %iter.sroa.4.0.i17804, %_43.i, !dbg !7090
  %_45.i = icmp ult i32 %_42.i, %_128.1.i, !dbg !7091
  br i1 %_45.i, label %bb27.i, label %panic9.i, !dbg !7091

panic9.i:                                         ; preds = %bb17.i
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %502, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_42.i, i32 noundef %_128.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70d40ae2302f3ea19fa0c4a65fe82786) #33, !dbg !7091, !noalias !7075
  unreachable, !dbg !7091

bb27.i:                                           ; preds = %bb17.i
  %581 = getelementptr inbounds nuw float, ptr %_128.0.i, i32 %_42.i, !dbg !7091
  %_41.i = load float, ptr %581, align 4, !dbg !7091, !noalias !7075, !noundef !10
  %_117.i = fcmp olt float %_41.i, %newest.sroa.0.0.i, !dbg !7092
  %newest.sroa.0.1.i = select i1 %_117.i, float %_41.i, float %newest.sroa.0.0.i, !dbg !7092
  store float %newest.sroa.0.1.i, ptr %iter.sroa.0.0.ptr.i17806, align 4, !dbg !7094, !alias.scope !7058, !noalias !7095
  br label %bb28.i, !dbg !7096

bb28.i:                                           ; preds = %bb22.i, %bb19.i1580, %bb27.i
  %storemerge15108 = phi i32 [ %_40.i, %bb27.i ], [ 0, %bb19.i1580 ], [ 0, %bb22.i ], !dbg !7097
  store i32 %storemerge15108, ptr %577, align 4, !dbg !7097, !noalias !7075
  %582 = icmp eq i32 %570, 0, !dbg !7063
  br i1 %582, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit, label %bb29.i, !dbg !7063

bb19.i1580:                                       ; preds = %bb15.i1576
  store float %newest.sroa.0.0.i, ptr %iter.sroa.0.0.ptr.i17806, align 4, !dbg !7094, !alias.scope !7058, !noalias !7095
  %_118.i17799.not = icmp eq i32 %shape.i, 0, !dbg !7098
  br i1 %_118.i17799.not, label %bb28.i, label %bb40.i.preheader, !dbg !7102

bb40.i.preheader:                                 ; preds = %bb19.i1580
  %583 = load float, ptr %575, align 4, !dbg !7103, !noalias !7075, !noundef !10
  br label %bb40.i, !dbg !7104

bb40.i:                                           ; preds = %bb40.i.preheader, %bb22.i
  %iter2.sroa.0.0.i158217802 = phi i32 [ %_119.i, %bb22.i ], [ 0, %bb40.i.preheader ]
  %suffix.sroa.0.0.i17801 = phi float [ %suffix.sroa.0.1.i, %bb22.i ], [ %583, %bb40.i.preheader ]
  %end.sroa.0.1.i17800 = phi i32 [ %586, %bb22.i ], [ %spec.select.i, %bb40.i.preheader ]
  %_54.i = mul i32 %end.sroa.0.1.i17800, %width.i, !dbg !7105
  %_53.i = add i32 %_54.i, %iter.sroa.4.0.i17804, !dbg !7105
  %_57.i = icmp ult i32 %_53.i, %_128.1.i, !dbg !7104
  br i1 %_57.i, label %bb22.i, label %panic13.i, !dbg !7104

panic13.i:                                        ; preds = %bb40.i
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %502, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_53.i, i32 noundef %_128.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a271bbb7fd83c3605ea58a06b7065fa4) #33, !dbg !7104, !noalias !7075
  unreachable, !dbg !7104

bb22.i:                                           ; preds = %bb40.i
  %_119.i = add nuw i32 %iter2.sroa.0.0.i158217802, 1, !dbg !7106
  %584 = getelementptr inbounds nuw float, ptr %_128.0.i, i32 %_53.i, !dbg !7104
  %_52.i = load float, ptr %584, align 4, !dbg !7104, !noalias !7075, !noundef !10
  %_121.i = fcmp olt float %suffix.sroa.0.0.i17801, %_52.i, !dbg !7109
  %suffix.sroa.0.1.i = select i1 %_121.i, float %suffix.sroa.0.0.i17801, float %_52.i, !dbg !7109
  store float %suffix.sroa.0.1.i, ptr %584, align 4, !dbg !7111, !noalias !7075
  %585 = icmp eq i32 %end.sroa.0.1.i17800, 0, !dbg !7112
  %spec.store.select.i1585 = select i1 %585, i32 %_87.i824, i32 %end.sroa.0.1.i17800, !dbg !7112
  %586 = add i32 %spec.store.select.i1585, -1, !dbg !7113
  %exitcond20480.not = icmp eq i32 %_119.i, %shape.i, !dbg !7098
  br i1 %exitcond20480.not, label %bb28.i, label %bb40.i, !dbg !7102

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit: ; preds = %bb28.i, %bb29.i
  %lanes.i5145.sroa.0.0.copyload.pre = load <4 x float>, ptr %scratch.i758, align 4, !dbg !7114, !alias.scope !7119, !noalias !7123
  br label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit, !dbg !7127

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit: ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5907
  %lanes.i5145.sroa.0.0.copyload = phi <4 x float> [ %lanes.i5145.sroa.0.0.copyload.pre, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit ], [ %lanes.i5170.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5907 ], !dbg !7114
  %587 = fmul <4 x float> %lanes.i5145.sroa.0.0.copyload, splat (float 1.638400e+04), !dbg !7128
  %588 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %587), !dbg !7132
  %589 = fmul <4 x float> %588, splat (float 0x3F10000000000000), !dbg !7136
  %590 = icmp eq i32 %width.i.i921, 0, !dbg !7140
  %_163.1.i.i970.pre = load i32, ptr %164, align 4, !dbg !7142, !alias.scope !7000, !noalias !7001
  br i1 %590, label %bb53.i.i965, label %bb36.i.i944.lr.ph, !dbg !7140

bb36.i.i944.lr.ph:                                ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit
  %_159.1.i.i949 = load i32, ptr %158, align 4, !alias.scope !7000, !noalias !7001, !noundef !10
  %_159.0.i.i953 = load ptr, ptr %159, align 4, !nonnull !10
  %_161.0.i.i963 = load ptr, ptr %165, align 4, !nonnull !10
  %exitcond20486.not = icmp eq i32 %_159.1.i.i949, 0, !dbg !7143
  br i1 %exitcond20486.not, label %panic.i.i951, label %bb14.i.i952, !dbg !7143

bb34.i.i1019:                                     ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5159
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %502, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i927, i32 noundef %_158.1.i.i926, i32 noundef %_158.1.i.i926, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_db7c42041d7aaaba4839906f37294547) #33, !dbg !7144, !noalias !7145
  unreachable, !dbg !7144

bb53.i.i965.loopexit:                             ; preds = %bb18.i.i962.7, %bb18.i.i962.6, %bb18.i.i962.5, %bb18.i.i962.4, %bb18.i.i962.3, %bb18.i.i962.2, %bb18.i.i962.1, %bb18.i.i962
  %lanes.i5138.sroa.0.0.copyload.pre = load <4 x float>, ptr %scratch.i758, align 4, !dbg !7146, !alias.scope !7151, !noalias !7155
  br label %bb53.i.i965, !dbg !7159

bb53.i.i965:                                      ; preds = %bb53.i.i965.loopexit, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit
  %lanes.i5138.sroa.0.0.copyload = phi <4 x float> [ %lanes.i5138.sroa.0.0.copyload.pre, %bb53.i.i965.loopexit ], [ %lanes.i5145.sroa.0.0.copyload, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit ], !dbg !7146
  %591 = fadd <4 x float> %589, %419, !dbg !7160
  %592 = fsub <4 x float> %591, %lanes.i5138.sroa.0.0.copyload, !dbg !7164
  %_123.i.i971 = icmp ugt i32 %_22.i.i927, %_163.1.i.i970.pre, !dbg !7168
  br i1 %_123.i.i971, label %bb41.i.i1018, label %bb42.i.i972, !dbg !7168, !prof !902

bb14.i.i952:                                      ; preds = %bb36.i.i944.lr.ph
  %593 = getelementptr inbounds nuw i8, ptr %_159.0.i.i953, i32 8, !dbg !7143
  %_42.i.i954 = load i32, ptr %593, align 4, !dbg !7143, !noalias !7171, !noundef !10
  %594 = add i32 %_42.i.i954, %ring_cursor.sroa.0.1.i79217813, !dbg !7172
  %_45.not.i.i955 = icmp ult i32 %594, %_87.i824, !dbg !7173
  %595 = select i1 %_45.not.i.i955, i32 0, i32 %_87.i824, !dbg !7173
  %spec.select.i.i956 = sub nuw i32 %594, %595, !dbg !7173
  %_49.i.i957 = mul i32 %spec.select.i.i956, %width.i.i921, !dbg !7174
  %_51.i.i960 = icmp ult i32 %_49.i.i957, %_163.1.i.i970.pre, !dbg !7175
  br i1 %_51.i.i960, label %bb18.i.i962, label %panic1.i.i961, !dbg !7175

panic.i.i951:                                     ; preds = %bb36.i.i944.7, %bb36.i.i944.6, %bb36.i.i944.5, %bb36.i.i944.4, %bb36.i.i944.3, %bb36.i.i944.2, %bb36.i.i944.1, %bb36.i.i944.lr.ph
  %_159.1.i.i949.lcssa.ph = phi i32 [ 7, %bb36.i.i944.7 ], [ 6, %bb36.i.i944.6 ], [ 5, %bb36.i.i944.5 ], [ 4, %bb36.i.i944.4 ], [ 3, %bb36.i.i944.3 ], [ 2, %bb36.i.i944.2 ], [ 1, %bb36.i.i944.1 ], [ 0, %bb36.i.i944.lr.ph ]
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %502, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_159.1.i.i949.lcssa.ph, i32 noundef %_159.1.i.i949.lcssa.ph, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_33746731a929bad63709ddedbca82f5f) #33, !dbg !7143, !noalias !7171
  unreachable, !dbg !7143

bb18.i.i962:                                      ; preds = %bb14.i.i952
  %596 = getelementptr inbounds nuw float, ptr %_161.0.i.i963, i32 %_49.i.i957, !dbg !7175
  %_47.i.i964 = load float, ptr %596, align 4, !dbg !7175, !noalias !7171, !noundef !10
  store float %_47.i.i964, ptr %scratch.i758, align 4, !dbg !7176, !alias.scope !6997, !noalias !7177
  %597 = icmp eq i32 %width.i.i921, 1, !dbg !7140
  br i1 %597, label %bb53.i.i965.loopexit, label %bb36.i.i944.1, !dbg !7140

bb36.i.i944.1:                                    ; preds = %bb18.i.i962
  %exitcond20486.1.not = icmp eq i32 %_159.1.i.i949, 1, !dbg !7143
  br i1 %exitcond20486.1.not, label %panic.i.i951, label %bb14.i.i952.1, !dbg !7143

bb14.i.i952.1:                                    ; preds = %bb36.i.i944.1
  %598 = getelementptr inbounds nuw i8, ptr %_159.0.i.i953, i32 20, !dbg !7143
  %_42.i.i954.1 = load i32, ptr %598, align 4, !dbg !7143, !noalias !7171, !noundef !10
  %599 = add i32 %_42.i.i954.1, %ring_cursor.sroa.0.1.i79217813, !dbg !7172
  %_45.not.i.i955.1 = icmp ult i32 %599, %_87.i824, !dbg !7173
  %600 = select i1 %_45.not.i.i955.1, i32 0, i32 %_87.i824, !dbg !7173
  %spec.select.i.i956.1 = sub nuw i32 %599, %600, !dbg !7173
  %_49.i.i957.1 = mul i32 %spec.select.i.i956.1, %width.i.i921, !dbg !7174
  %_48.i.i958.1 = add i32 %_49.i.i957.1, 1, !dbg !7174
  %_51.i.i960.1 = icmp ult i32 %_48.i.i958.1, %_163.1.i.i970.pre, !dbg !7175
  br i1 %_51.i.i960.1, label %bb18.i.i962.1, label %panic1.i.i961, !dbg !7175

bb18.i.i962.1:                                    ; preds = %bb14.i.i952.1
  %601 = getelementptr inbounds nuw float, ptr %_161.0.i.i963, i32 %_48.i.i958.1, !dbg !7175
  %_47.i.i964.1 = load float, ptr %601, align 4, !dbg !7175, !noalias !7171, !noundef !10
  store float %_47.i.i964.1, ptr %iter.sroa.0.0.ptr.i.i94317810.1, align 4, !dbg !7176, !alias.scope !6997, !noalias !7177
  %602 = icmp eq i32 %width.i.i921, 2, !dbg !7140
  br i1 %602, label %bb53.i.i965.loopexit, label %bb36.i.i944.2, !dbg !7140

bb36.i.i944.2:                                    ; preds = %bb18.i.i962.1
  %exitcond20486.2.not = icmp eq i32 %_159.1.i.i949, 2, !dbg !7143
  br i1 %exitcond20486.2.not, label %panic.i.i951, label %bb14.i.i952.2, !dbg !7143

bb14.i.i952.2:                                    ; preds = %bb36.i.i944.2
  %603 = getelementptr inbounds nuw i8, ptr %_159.0.i.i953, i32 32, !dbg !7143
  %_42.i.i954.2 = load i32, ptr %603, align 4, !dbg !7143, !noalias !7171, !noundef !10
  %604 = add i32 %_42.i.i954.2, %ring_cursor.sroa.0.1.i79217813, !dbg !7172
  %_45.not.i.i955.2 = icmp ult i32 %604, %_87.i824, !dbg !7173
  %605 = select i1 %_45.not.i.i955.2, i32 0, i32 %_87.i824, !dbg !7173
  %spec.select.i.i956.2 = sub nuw i32 %604, %605, !dbg !7173
  %_49.i.i957.2 = mul i32 %spec.select.i.i956.2, %width.i.i921, !dbg !7174
  %_48.i.i958.2 = add i32 %_49.i.i957.2, 2, !dbg !7174
  %_51.i.i960.2 = icmp ult i32 %_48.i.i958.2, %_163.1.i.i970.pre, !dbg !7175
  br i1 %_51.i.i960.2, label %bb18.i.i962.2, label %panic1.i.i961, !dbg !7175

bb18.i.i962.2:                                    ; preds = %bb14.i.i952.2
  %606 = getelementptr inbounds nuw float, ptr %_161.0.i.i963, i32 %_48.i.i958.2, !dbg !7175
  %_47.i.i964.2 = load float, ptr %606, align 4, !dbg !7175, !noalias !7171, !noundef !10
  store float %_47.i.i964.2, ptr %iter.sroa.0.0.ptr.i.i94317810.2, align 4, !dbg !7176, !alias.scope !6997, !noalias !7177
  %607 = icmp eq i32 %width.i.i921, 3, !dbg !7140
  br i1 %607, label %bb53.i.i965.loopexit, label %bb36.i.i944.3, !dbg !7140

bb36.i.i944.3:                                    ; preds = %bb18.i.i962.2
  %exitcond20486.3.not = icmp eq i32 %_159.1.i.i949, 3, !dbg !7143
  br i1 %exitcond20486.3.not, label %panic.i.i951, label %bb14.i.i952.3, !dbg !7143

bb14.i.i952.3:                                    ; preds = %bb36.i.i944.3
  %608 = getelementptr inbounds nuw i8, ptr %_159.0.i.i953, i32 44, !dbg !7143
  %_42.i.i954.3 = load i32, ptr %608, align 4, !dbg !7143, !noalias !7171, !noundef !10
  %609 = add i32 %_42.i.i954.3, %ring_cursor.sroa.0.1.i79217813, !dbg !7172
  %_45.not.i.i955.3 = icmp ult i32 %609, %_87.i824, !dbg !7173
  %610 = select i1 %_45.not.i.i955.3, i32 0, i32 %_87.i824, !dbg !7173
  %spec.select.i.i956.3 = sub nuw i32 %609, %610, !dbg !7173
  %_49.i.i957.3 = mul i32 %spec.select.i.i956.3, %width.i.i921, !dbg !7174
  %_48.i.i958.3 = add i32 %_49.i.i957.3, 3, !dbg !7174
  %_51.i.i960.3 = icmp ult i32 %_48.i.i958.3, %_163.1.i.i970.pre, !dbg !7175
  br i1 %_51.i.i960.3, label %bb18.i.i962.3, label %panic1.i.i961, !dbg !7175

bb18.i.i962.3:                                    ; preds = %bb14.i.i952.3
  %611 = getelementptr inbounds nuw float, ptr %_161.0.i.i963, i32 %_48.i.i958.3, !dbg !7175
  %_47.i.i964.3 = load float, ptr %611, align 4, !dbg !7175, !noalias !7171, !noundef !10
  store float %_47.i.i964.3, ptr %iter.sroa.0.0.ptr.i.i94317810.3, align 4, !dbg !7176, !alias.scope !6997, !noalias !7177
  %612 = icmp eq i32 %width.i.i921, 4, !dbg !7140
  br i1 %612, label %bb53.i.i965.loopexit, label %bb36.i.i944.4, !dbg !7140

bb36.i.i944.4:                                    ; preds = %bb18.i.i962.3
  %exitcond20486.4.not = icmp eq i32 %_159.1.i.i949, 4, !dbg !7143
  br i1 %exitcond20486.4.not, label %panic.i.i951, label %bb14.i.i952.4, !dbg !7143

bb14.i.i952.4:                                    ; preds = %bb36.i.i944.4
  %613 = getelementptr inbounds nuw i8, ptr %_159.0.i.i953, i32 56, !dbg !7143
  %_42.i.i954.4 = load i32, ptr %613, align 4, !dbg !7143, !noalias !7171, !noundef !10
  %614 = add i32 %_42.i.i954.4, %ring_cursor.sroa.0.1.i79217813, !dbg !7172
  %_45.not.i.i955.4 = icmp ult i32 %614, %_87.i824, !dbg !7173
  %615 = select i1 %_45.not.i.i955.4, i32 0, i32 %_87.i824, !dbg !7173
  %spec.select.i.i956.4 = sub nuw i32 %614, %615, !dbg !7173
  %_49.i.i957.4 = mul i32 %spec.select.i.i956.4, %width.i.i921, !dbg !7174
  %_48.i.i958.4 = add i32 %_49.i.i957.4, 4, !dbg !7174
  %_51.i.i960.4 = icmp ult i32 %_48.i.i958.4, %_163.1.i.i970.pre, !dbg !7175
  br i1 %_51.i.i960.4, label %bb18.i.i962.4, label %panic1.i.i961, !dbg !7175

bb18.i.i962.4:                                    ; preds = %bb14.i.i952.4
  %616 = getelementptr inbounds nuw float, ptr %_161.0.i.i963, i32 %_48.i.i958.4, !dbg !7175
  %_47.i.i964.4 = load float, ptr %616, align 4, !dbg !7175, !noalias !7171, !noundef !10
  store float %_47.i.i964.4, ptr %iter.sroa.0.0.ptr.i.i94317810.4, align 4, !dbg !7176, !alias.scope !6997, !noalias !7177
  %617 = icmp eq i32 %width.i.i921, 5, !dbg !7140
  br i1 %617, label %bb53.i.i965.loopexit, label %bb36.i.i944.5, !dbg !7140

bb36.i.i944.5:                                    ; preds = %bb18.i.i962.4
  %exitcond20486.5.not = icmp eq i32 %_159.1.i.i949, 5, !dbg !7143
  br i1 %exitcond20486.5.not, label %panic.i.i951, label %bb14.i.i952.5, !dbg !7143

bb14.i.i952.5:                                    ; preds = %bb36.i.i944.5
  %618 = getelementptr inbounds nuw i8, ptr %_159.0.i.i953, i32 68, !dbg !7143
  %_42.i.i954.5 = load i32, ptr %618, align 4, !dbg !7143, !noalias !7171, !noundef !10
  %619 = add i32 %_42.i.i954.5, %ring_cursor.sroa.0.1.i79217813, !dbg !7172
  %_45.not.i.i955.5 = icmp ult i32 %619, %_87.i824, !dbg !7173
  %620 = select i1 %_45.not.i.i955.5, i32 0, i32 %_87.i824, !dbg !7173
  %spec.select.i.i956.5 = sub nuw i32 %619, %620, !dbg !7173
  %_49.i.i957.5 = mul i32 %spec.select.i.i956.5, %width.i.i921, !dbg !7174
  %_48.i.i958.5 = add i32 %_49.i.i957.5, 5, !dbg !7174
  %_51.i.i960.5 = icmp ult i32 %_48.i.i958.5, %_163.1.i.i970.pre, !dbg !7175
  br i1 %_51.i.i960.5, label %bb18.i.i962.5, label %panic1.i.i961, !dbg !7175

bb18.i.i962.5:                                    ; preds = %bb14.i.i952.5
  %621 = getelementptr inbounds nuw float, ptr %_161.0.i.i963, i32 %_48.i.i958.5, !dbg !7175
  %_47.i.i964.5 = load float, ptr %621, align 4, !dbg !7175, !noalias !7171, !noundef !10
  store float %_47.i.i964.5, ptr %iter.sroa.0.0.ptr.i.i94317810.5, align 4, !dbg !7176, !alias.scope !6997, !noalias !7177
  %622 = icmp eq i32 %width.i.i921, 6, !dbg !7140
  br i1 %622, label %bb53.i.i965.loopexit, label %bb36.i.i944.6, !dbg !7140

bb36.i.i944.6:                                    ; preds = %bb18.i.i962.5
  %exitcond20486.6.not = icmp eq i32 %_159.1.i.i949, 6, !dbg !7143
  br i1 %exitcond20486.6.not, label %panic.i.i951, label %bb14.i.i952.6, !dbg !7143

bb14.i.i952.6:                                    ; preds = %bb36.i.i944.6
  %623 = getelementptr inbounds nuw i8, ptr %_159.0.i.i953, i32 80, !dbg !7143
  %_42.i.i954.6 = load i32, ptr %623, align 4, !dbg !7143, !noalias !7171, !noundef !10
  %624 = add i32 %_42.i.i954.6, %ring_cursor.sroa.0.1.i79217813, !dbg !7172
  %_45.not.i.i955.6 = icmp ult i32 %624, %_87.i824, !dbg !7173
  %625 = select i1 %_45.not.i.i955.6, i32 0, i32 %_87.i824, !dbg !7173
  %spec.select.i.i956.6 = sub nuw i32 %624, %625, !dbg !7173
  %_49.i.i957.6 = mul i32 %spec.select.i.i956.6, %width.i.i921, !dbg !7174
  %_48.i.i958.6 = add i32 %_49.i.i957.6, 6, !dbg !7174
  %_51.i.i960.6 = icmp ult i32 %_48.i.i958.6, %_163.1.i.i970.pre, !dbg !7175
  br i1 %_51.i.i960.6, label %bb18.i.i962.6, label %panic1.i.i961, !dbg !7175

bb18.i.i962.6:                                    ; preds = %bb14.i.i952.6
  %626 = getelementptr inbounds nuw float, ptr %_161.0.i.i963, i32 %_48.i.i958.6, !dbg !7175
  %_47.i.i964.6 = load float, ptr %626, align 4, !dbg !7175, !noalias !7171, !noundef !10
  store float %_47.i.i964.6, ptr %iter.sroa.0.0.ptr.i.i94317810.6, align 4, !dbg !7176, !alias.scope !6997, !noalias !7177
  %627 = icmp eq i32 %width.i.i921, 7, !dbg !7140
  br i1 %627, label %bb53.i.i965.loopexit, label %bb36.i.i944.7, !dbg !7140

bb36.i.i944.7:                                    ; preds = %bb18.i.i962.6
  %exitcond20486.7.not = icmp eq i32 %_159.1.i.i949, 7, !dbg !7143
  br i1 %exitcond20486.7.not, label %panic.i.i951, label %bb14.i.i952.7, !dbg !7143

bb14.i.i952.7:                                    ; preds = %bb36.i.i944.7
  %628 = getelementptr inbounds nuw i8, ptr %_159.0.i.i953, i32 92, !dbg !7143
  %_42.i.i954.7 = load i32, ptr %628, align 4, !dbg !7143, !noalias !7171, !noundef !10
  %629 = add i32 %_42.i.i954.7, %ring_cursor.sroa.0.1.i79217813, !dbg !7172
  %_45.not.i.i955.7 = icmp ult i32 %629, %_87.i824, !dbg !7173
  %630 = select i1 %_45.not.i.i955.7, i32 0, i32 %_87.i824, !dbg !7173
  %spec.select.i.i956.7 = sub nuw i32 %629, %630, !dbg !7173
  %_49.i.i957.7 = mul i32 %spec.select.i.i956.7, %width.i.i921, !dbg !7174
  %_48.i.i958.7 = add i32 %_49.i.i957.7, 7, !dbg !7174
  %_51.i.i960.7 = icmp ult i32 %_48.i.i958.7, %_163.1.i.i970.pre, !dbg !7175
  br i1 %_51.i.i960.7, label %bb18.i.i962.7, label %panic1.i.i961, !dbg !7175

bb18.i.i962.7:                                    ; preds = %bb14.i.i952.7
  %631 = getelementptr inbounds nuw float, ptr %_161.0.i.i963, i32 %_48.i.i958.7, !dbg !7175
  %_47.i.i964.7 = load float, ptr %631, align 4, !dbg !7175, !noalias !7171, !noundef !10
  store float %_47.i.i964.7, ptr %iter.sroa.0.0.ptr.i.i94317810.7, align 4, !dbg !7176, !alias.scope !6997, !noalias !7177
  br label %bb53.i.i965.loopexit, !dbg !7140

panic1.i.i961:                                    ; preds = %bb14.i.i952.7, %bb14.i.i952.6, %bb14.i.i952.5, %bb14.i.i952.4, %bb14.i.i952.3, %bb14.i.i952.2, %bb14.i.i952.1, %bb14.i.i952
  %_48.i.i958.lcssa.ph = phi i32 [ %_48.i.i958.7, %bb14.i.i952.7 ], [ %_48.i.i958.6, %bb14.i.i952.6 ], [ %_48.i.i958.5, %bb14.i.i952.5 ], [ %_48.i.i958.4, %bb14.i.i952.4 ], [ %_48.i.i958.3, %bb14.i.i952.3 ], [ %_48.i.i958.2, %bb14.i.i952.2 ], [ %_48.i.i958.1, %bb14.i.i952.1 ], [ %_49.i.i957, %bb14.i.i952 ]
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %502, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_48.i.i958.lcssa.ph, i32 noundef %_163.1.i.i970.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_0e76a26fbb751dfb8cf8a069b5b0d39a) #33, !dbg !7175, !noalias !7171
  unreachable, !dbg !7175

bb42.i.i972:                                      ; preds = %bb53.i.i965
  %_126.i.i974 = sub nuw i32 %_163.1.i.i970.pre, %_22.i.i927, !dbg !7178
  %_8.i5899 = icmp samesign ugt i32 %_126.i.i974, 3, !dbg !7179
  br i1 %_8.i5899, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5902, label %bb2.i5900, !dbg !7179, !prof !1153

bb2.i5900:                                        ; preds = %bb42.i.i972
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %502, ptr %149, align 16, !dbg !5264
  store <4 x float> %592, ptr %166, align 16, !dbg !5279
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_126.i.i974, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !7184, !noalias !7185
  unreachable, !dbg !7184

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5902: ; preds = %bb42.i.i972
  %_163.0.i.i973 = load ptr, ptr %165, align 4, !dbg !7142, !alias.scope !7000, !noalias !7001, !nonnull !10, !noundef !10
  %_130.i.i975 = getelementptr inbounds nuw float, ptr %_163.0.i.i973, i32 %_22.i.i927, !dbg !7189
  store <4 x float> %589, ptr %_130.i.i975, align 4, !dbg !7191, !alias.scope !7195, !noalias !7199
  %_62.i.i97715114 = load <4 x float>, ptr %167, align 16, !dbg !7201
  %_66.i.i98015115 = load <4 x float>, ptr %168, align 16, !dbg !7202
  %632 = fdiv <4 x float> %592, %_62.i.i97715114, !dbg !7203
  %633 = fsub <4 x float> splat (float 1.000000e+00), %632, !dbg !7207
  %634 = fsub <4 x float> %633, %_66.i.i98015115, !dbg !7211
  %635 = bitcast <16 x i8> %_4.i6464 to <4 x float>, !dbg !7215
  %636 = fmul <4 x float> %634, %635, !dbg !7219
  %637 = fadd <4 x float> %_66.i.i98015115, %636, !dbg !7220
  %638 = fcmp olt <4 x float> %637, %633, !dbg !7223
  %639 = select <4 x i1> %638, <4 x float> %633, <4 x float> %637, !dbg !7227
  %640 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %639), !dbg !7228
  %641 = fcmp uge <4 x float> %640, splat (float 0x3BC79CA100000000), !dbg !7233
  %642 = bitcast <4 x float> %639 to <4 x i32>, !dbg !7238
  %643 = select <4 x i1> %641, <4 x i32> %642, <4 x i32> zeroinitializer, !dbg !7238
  store <4 x i32> %643, ptr %168, align 16, !dbg !7241
  %644 = bitcast <4 x i32> %643 to <4 x float>, !dbg !7242
  %645 = fsub <4 x float> splat (float 1.000000e+00), %644, !dbg !7246
  %_164.1.i.i990 = load i32, ptr %169, align 4, !dbg !7247, !alias.scope !7000, !noalias !7001, !noundef !10
  %_74.i.i991 = mul i32 %width.i.i921, %main_cursor.sroa.0.1.i79317814, !dbg !7248
  %_134.i.i992 = icmp ugt i32 %_74.i.i991, %_164.1.i.i990, !dbg !7249
  br i1 %_134.i.i992, label %bb47.i.i1017, label %bb48.i.i993, !dbg !7249, !prof !902

bb41.i.i1018:                                     ; preds = %bb53.i.i965
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %502, ptr %149, align 16, !dbg !5264
  store <4 x float> %592, ptr %166, align 16, !dbg !5279
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i927, i32 noundef %_163.1.i.i970.pre, i32 noundef %_163.1.i.i970.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c2286ff41a33b8c11aa270fe215441de) #33, !dbg !7252, !noalias !7171
  unreachable, !dbg !7252

bb48.i.i993:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5902
  %_137.i.i995 = sub nuw i32 %_164.1.i.i990, %_74.i.i991, !dbg !7253
  %_8.i5132 = icmp samesign ugt i32 %_137.i.i995, 3, !dbg !7254
  br i1 %_8.i5132, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5892, label %bb2.i5133, !dbg !7254, !prof !1153

bb2.i5133:                                        ; preds = %bb48.i.i993
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %502, ptr %149, align 16, !dbg !5264
  store <4 x float> %592, ptr %166, align 16, !dbg !5279
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_137.i.i995, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !7259, !noalias !7260
  unreachable, !dbg !7259

bb47.i.i1017:                                     ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5902
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %502, ptr %149, align 16, !dbg !5264
  store <4 x float> %592, ptr %166, align 16, !dbg !5279
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_74.i.i991, i32 noundef %_164.1.i.i990, i32 noundef %_164.1.i.i990, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a3a21efd18b403ec25db545d522c479) #33, !dbg !7264, !noalias !7171
  unreachable, !dbg !7264

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5892: ; preds = %bb48.i.i993
  %_164.0.i.i994 = load ptr, ptr %170, align 4, !dbg !7247, !alias.scope !7000, !noalias !7001, !nonnull !10, !noundef !10
  %_141.i.i996 = getelementptr inbounds nuw float, ptr %_164.0.i.i994, i32 %_74.i.i991, !dbg !7265
  %lanes.i5129.sroa.0.0.copyload = load <4 x i32>, ptr %_141.i.i996, align 4, !dbg !7267, !alias.scope !7271, !noalias !7275
  store <4 x i32> %lanes.i5152.sroa.0.0.copyload, ptr %_141.i.i996, align 4, !dbg !7277, !alias.scope !7282, !noalias !7286
  %646 = bitcast <4 x i32> %lanes.i5129.sroa.0.0.copyload to <4 x float>, !dbg !7290
  %647 = fmul <4 x float> %645, %646, !dbg !7294
  %648 = bitcast <4 x i32> %lanes.i5129.sroa.0.0.copyload to <16 x i8>, !dbg !7295
  %649 = bitcast <4 x float> %647 to <16 x i8>, !dbg !7299
  %_4.i6488 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %648, <16 x i8> %649, <16 x i8> %154), !dbg !7300
  store <16 x i8> %_4.i6488, ptr %_147.i918, align 4, !dbg !7301, !alias.scope !7306, !noalias !7310
  %650 = add i32 %main_cursor.sroa.0.1.i79317814, 1, !dbg !7314
  %_102.i1010 = load i32, ptr %171, align 4, !dbg !7315, !alias.scope !5173, !noalias !6565, !noundef !10
  %_100.i1011 = icmp eq i32 %650, %_102.i1010, !dbg !7316
  %spec.store.select11.i1012 = select i1 %_100.i1011, i32 0, i32 %650, !dbg !7316
  %651 = add i32 %ring_cursor.sroa.0.1.i79217813, 1, !dbg !7317
  %_103.i1013 = icmp eq i32 %651, %_87.i824, !dbg !7318
  %spec.store.select12.i1014 = select i1 %_103.i1013, i32 0, i32 %651, !dbg !7318
  %exitcond20490.not = icmp eq i32 %448, %umax20489, !dbg !7319
  br i1 %exitcond20490.not, label %bb13.i778.loopexit.loopexit, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5209, !dbg !6306

bb48.i1020:                                       ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5912
  store <4 x float> %423, ptr %126, align 16, !dbg !5238
  store <16 x i8> %_4.i6458, ptr %_64.i798, align 16, !dbg !5249
  store <16 x i8> %_4.i6459, ptr %127, align 16, !dbg !5251
  store <4 x float> %432, ptr %129, align 16, !dbg !5252
  store <16 x i8> %_4.i6460, ptr %_65.i799, align 16, !dbg !5254
  store <16 x i8> %_4.i6461, ptr %130, align 16, !dbg !5255
  store <4 x float> %441, ptr %132, align 16, !dbg !5256
  store <16 x i8> %_4.i6462, ptr %_69.i802, align 16, !dbg !5260
  store <16 x i8> %_4.i6463, ptr %133, align 16, !dbg !5261
  store <4 x float> %451, ptr %135, align 16, !dbg !5262
  store <4 x float> %502, ptr %149, align 16, !dbg !5264
  store <4 x float> %419, ptr %166, align 16, !dbg !5279
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i797, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_3ec0ee57975400dd5dbd052e150c5392) #33, !dbg !7322, !noalias !6966
  unreachable, !dbg !7322

_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %bb13.i778.loopexit, %bb12.i
  %ring_cursor.sroa.0.0.i781.lcssa = phi i32 [ %_37.i770, %bb12.i ], [ %ring_cursor.sroa.0.1.i792.lcssa, %bb13.i778.loopexit ], !dbg !5199
  %main_cursor.sroa.0.0.i782.lcssa = phi i32 [ %_35.i769, %bb12.i ], [ %main_cursor.sroa.0.1.i793.lcssa, %bb13.i778.loopexit ], !dbg !5196
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_(ptr noalias noundef readonly align 16 captures(none) dereferenceable(368) %hot_left.i761, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33) #32, !dbg !7323
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_(ptr noalias noundef readonly align 16 captures(none) dereferenceable(368) %hot_right.i760, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34) #32, !dbg !7324
  store i32 %main_cursor.sroa.0.0.i782.lcssa, ptr %_35, align 4, !dbg !7325, !alias.scope !5179, !noalias !5198
  store i32 %ring_cursor.sroa.0.0.i781.lcssa, ptr %81, align 4, !dbg !7326, !alias.scope !5179, !noalias !5198
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i756), !dbg !7327, !noalias !5203
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i757), !dbg !7328, !noalias !5203
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i758), !dbg !7329, !noalias !5203
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !5172

bb11.i:                                           ; preds = %bb10.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7330), !dbg !7333
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7334), !dbg !7333
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7336), !dbg !7333
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7338), !dbg !7333
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_(ptr noalias noundef align 16 captures(none) dereferenceable(368) %hot_left.i472, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #32, !dbg !7340
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_(ptr noalias noundef align 16 captures(none) dereferenceable(368) %hot_right.i471, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #32, !dbg !7344
  %652 = load i8, ptr %79, align 16, !dbg !7346, !range !4765, !alias.scope !7330, !noalias !7350, !noundef !10
  %653 = load i8, ptr %80, align 1, !dbg !7354, !range !4765, !alias.scope !7330, !noalias !7350, !noundef !10
  %_35.i = load i32, ptr %_35, align 4, !dbg !7356, !alias.scope !7338, !noalias !7358, !noundef !10
  %_37.i479 = load i32, ptr %81, align 4, !dbg !7359, !alias.scope !7338, !noalias !7358, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i), !dbg !7361, !noalias !7363
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i, i8 0, i32 32, i1 false), !noalias !7363
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i469), !dbg !7364, !noalias !7363
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i469, i8 0, i32 1024, i1 false), !noalias !7363
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i468), !dbg !7366, !noalias !7363
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i468, i8 0, i32 1024, i1 false), !noalias !7363
  br i1 %_111.not.i18055, label %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb37.i.lr.ph, !dbg !7368

bb37.i.lr.ph:                                     ; preds = %bb11.i
  %_33.i478 = trunc nuw i8 %653 to i1, !dbg !7354
  %spec.store.select28.i = select i1 %_33.i478, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !7354
  %_32.i476 = trunc nuw i8 %652 to i1, !dbg !7346
  %link.sroa.0.0.i477 = select i1 %_32.i476, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !7346
  %d9.i6490 = lshr i32 %frames, 5, !dbg !7378
  %r2.i6491 = and i32 %frames, 31, !dbg !7385
  %_19.not.i6492 = icmp ne i32 %r2.i6491, 0, !dbg !7386
  %654 = zext i1 %_19.not.i6492 to i32, !dbg !7386
  %yield_count.sroa.0.0.i6493 = add nuw nsw i32 %d9.i6490, %654, !dbg !7386
  %history.i140.i.sroa.7.0.hot_left.i472.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i472, i32 16
  %history.i140.i.sroa.10.0.hot_left.i472.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i472, i32 32
  %history.i140.i.sroa.13.0.hot_left.i472.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i472, i32 48
  %history.i140.i.sroa.16.0.hot_left.i472.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i472, i32 64
  %history.i140.i.sroa.19.0.hot_left.i472.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i472, i32 80
  %history.i140.i.sroa.22.0.hot_left.i472.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i472, i32 96
  %history.i140.i.sroa.26.0.hot_left.i472.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i472, i32 112
  %history.i140.i.sroa.29.0.hot_left.i472.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i472, i32 128
  %history.i140.i.sroa.32.0.hot_left.i472.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i472, i32 144
  %history.i140.i.sroa.35.0.hot_left.i472.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i472, i32 160
  %history.i140.i.sroa.38.0.hot_left.i472.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i472, i32 176
  %655 = getelementptr inbounds nuw i8, ptr %self, i32 32
  %656 = getelementptr inbounds nuw i8, ptr %self, i32 48
  %657 = getelementptr inbounds nuw i8, ptr %self, i32 64
  %row1.i.i.i177.i = getelementptr inbounds nuw i8, ptr %self, i32 80
  %658 = getelementptr inbounds nuw i8, ptr %self, i32 96
  %659 = getelementptr inbounds nuw i8, ptr %self, i32 112
  %660 = getelementptr inbounds nuw i8, ptr %self, i32 128
  %row3.i.i.i191.i = getelementptr inbounds nuw i8, ptr %self, i32 144
  %661 = getelementptr inbounds nuw i8, ptr %self, i32 160
  %662 = getelementptr inbounds nuw i8, ptr %self, i32 176
  %663 = getelementptr inbounds nuw i8, ptr %self, i32 192
  %row5.i.i.i205.i = getelementptr inbounds nuw i8, ptr %self, i32 208
  %664 = getelementptr inbounds nuw i8, ptr %self, i32 224
  %665 = getelementptr inbounds nuw i8, ptr %self, i32 240
  %666 = getelementptr inbounds nuw i8, ptr %self, i32 256
  %row7.i.i.i219.i = getelementptr inbounds nuw i8, ptr %self, i32 272
  %667 = getelementptr inbounds nuw i8, ptr %self, i32 288
  %668 = getelementptr inbounds nuw i8, ptr %self, i32 304
  %669 = getelementptr inbounds nuw i8, ptr %self, i32 320
  %row9.i.i.i233.i = getelementptr inbounds nuw i8, ptr %self, i32 336
  %670 = getelementptr inbounds nuw i8, ptr %self, i32 352
  %671 = getelementptr inbounds nuw i8, ptr %self, i32 368
  %672 = getelementptr inbounds nuw i8, ptr %self, i32 384
  %row11.i.i.i247.i = getelementptr inbounds nuw i8, ptr %self, i32 400
  %673 = getelementptr inbounds nuw i8, ptr %self, i32 416
  %674 = getelementptr inbounds nuw i8, ptr %self, i32 432
  %675 = getelementptr inbounds nuw i8, ptr %self, i32 448
  %row13.i.i.i261.i = getelementptr inbounds nuw i8, ptr %self, i32 464
  %676 = getelementptr inbounds nuw i8, ptr %self, i32 480
  %677 = getelementptr inbounds nuw i8, ptr %self, i32 496
  %678 = getelementptr inbounds nuw i8, ptr %self, i32 512
  %row15.i.i.i275.i = getelementptr inbounds nuw i8, ptr %self, i32 528
  %679 = getelementptr inbounds nuw i8, ptr %self, i32 544
  %680 = getelementptr inbounds nuw i8, ptr %self, i32 560
  %681 = getelementptr inbounds nuw i8, ptr %self, i32 576
  %row17.i.i.i289.i = getelementptr inbounds nuw i8, ptr %self, i32 592
  %682 = getelementptr inbounds nuw i8, ptr %self, i32 608
  %683 = getelementptr inbounds nuw i8, ptr %self, i32 624
  %684 = getelementptr inbounds nuw i8, ptr %self, i32 640
  %row19.i.i.i303.i = getelementptr inbounds nuw i8, ptr %self, i32 656
  %685 = getelementptr inbounds nuw i8, ptr %self, i32 672
  %686 = getelementptr inbounds nuw i8, ptr %self, i32 688
  %687 = getelementptr inbounds nuw i8, ptr %self, i32 704
  %row21.i.i.i317.i = getelementptr inbounds nuw i8, ptr %self, i32 720
  %688 = getelementptr inbounds nuw i8, ptr %self, i32 736
  %689 = getelementptr inbounds nuw i8, ptr %self, i32 752
  %690 = getelementptr inbounds nuw i8, ptr %self, i32 768
  %history.i.i466.sroa.7.0.hot_right.i471.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i471, i32 16
  %history.i.i466.sroa.10.0.hot_right.i471.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i471, i32 32
  %history.i.i466.sroa.13.0.hot_right.i471.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i471, i32 48
  %history.i.i466.sroa.16.0.hot_right.i471.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i471, i32 64
  %history.i.i466.sroa.19.0.hot_right.i471.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i471, i32 80
  %history.i.i466.sroa.22.0.hot_right.i471.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i471, i32 96
  %history.i.i466.sroa.26.0.hot_right.i471.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i471, i32 112
  %history.i.i466.sroa.29.0.hot_right.i471.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i471, i32 128
  %history.i.i466.sroa.32.0.hot_right.i471.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i471, i32 144
  %history.i.i466.sroa.35.0.hot_right.i471.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i471, i32 160
  %history.i.i466.sroa.38.0.hot_right.i471.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i471, i32 176
  %_64.i = getelementptr inbounds nuw i8, ptr %hot_left.i472, i32 192
  %_65.i497 = getelementptr inbounds nuw i8, ptr %hot_left.i472, i32 256
  %_69.i500 = getelementptr inbounds nuw i8, ptr %hot_right.i471, i32 192
  %_70.i = getelementptr inbounds nuw i8, ptr %hot_right.i471, i32 256
  %691 = bitcast <4 x i32> %link.sroa.0.0.i477 to <16 x i8>
  %692 = getelementptr inbounds nuw i8, ptr %self, i32 916
  %693 = getelementptr inbounds nuw i8, ptr %self, i32 1020
  %694 = getelementptr inbounds nuw i8, ptr %self, i32 944
  %695 = getelementptr inbounds nuw i8, ptr %self, i32 940
  %696 = getelementptr inbounds nuw i8, ptr %self, i32 984
  %697 = getelementptr inbounds nuw i8, ptr %self, i32 980
  %698 = getelementptr inbounds nuw i8, ptr %self, i32 968
  %699 = getelementptr inbounds nuw i8, ptr %self, i32 964
  %700 = getelementptr inbounds nuw i8, ptr %self, i32 952
  %701 = getelementptr inbounds nuw i8, ptr %self, i32 948
  %702 = getelementptr inbounds nuw i8, ptr %hot_left.i472, i32 336
  %703 = getelementptr inbounds nuw i8, ptr %hot_left.i472, i32 352
  %704 = getelementptr inbounds nuw i8, ptr %hot_left.i472, i32 320
  %705 = getelementptr inbounds nuw i8, ptr %self, i32 936
  %706 = getelementptr inbounds nuw i8, ptr %self, i32 932
  %707 = bitcast <4 x i32> %spec.store.select28.i to <16 x i8>
  %708 = getelementptr inbounds nuw i8, ptr %self, i32 1120
  %709 = getelementptr inbounds nuw i8, ptr %self, i32 1044
  %710 = getelementptr inbounds nuw i8, ptr %self, i32 1040
  %711 = getelementptr inbounds nuw i8, ptr %self, i32 1116
  %712 = getelementptr inbounds nuw i8, ptr %self, i32 1112
  %713 = getelementptr inbounds nuw i8, ptr %self, i32 1084
  %714 = getelementptr inbounds nuw i8, ptr %self, i32 1080
  %715 = getelementptr inbounds nuw i8, ptr %self, i32 1068
  %716 = getelementptr inbounds nuw i8, ptr %self, i32 1064
  %717 = getelementptr inbounds nuw i8, ptr %self, i32 1052
  %718 = getelementptr inbounds nuw i8, ptr %self, i32 1048
  %719 = getelementptr inbounds nuw i8, ptr %hot_right.i471, i32 336
  %720 = getelementptr inbounds nuw i8, ptr %hot_right.i471, i32 352
  %721 = getelementptr inbounds nuw i8, ptr %hot_right.i471, i32 320
  %722 = getelementptr inbounds nuw i8, ptr %self, i32 1036
  %723 = getelementptr inbounds nuw i8, ptr %self, i32 1032
  %724 = getelementptr inbounds nuw i8, ptr %self, i32 920
  %iter.sroa.0.0.ptr.i53.i17889.1 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 4
  %iter.sroa.0.0.ptr.i53.i17889.2 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 8
  %iter.sroa.0.0.ptr.i53.i17889.3 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 12
  %iter.sroa.0.0.ptr.i53.i17889.4 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 16
  %iter.sroa.0.0.ptr.i53.i17889.5 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 20
  %iter.sroa.0.0.ptr.i53.i17889.6 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 24
  %iter.sroa.0.0.ptr.i53.i17889.7 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 28
  %iter.sroa.0.0.ptr.i.i17901.1 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 4
  %iter.sroa.0.0.ptr.i.i17901.2 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 8
  %iter.sroa.0.0.ptr.i.i17901.3 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 12
  %iter.sroa.0.0.ptr.i.i17901.4 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 16
  %iter.sroa.0.0.ptr.i.i17901.5 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 20
  %iter.sroa.0.0.ptr.i.i17901.6 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 24
  %iter.sroa.0.0.ptr.i.i17901.7 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 28
  br label %bb37.i, !dbg !7368

bb16.i.bb13.i.loopexit_crit_edge:                 ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5942
  store <4 x float> %1012, ptr %702, align 16, !dbg !7387
  store <4 x float> %1100, ptr %719, align 16, !dbg !7401
  br label %bb13.i.loopexit, !dbg !7403

bb13.i.loopexit:                                  ; preds = %bb16.i.bb13.i.loopexit_crit_edge, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i491
  %ring_cursor.sroa.0.1.i493.lcssa = phi i32 [ %spec.store.select12.i, %bb16.i.bb13.i.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i48518058, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i491 ], !dbg !7409
  %main_cursor.sroa.0.1.i494.lcssa = phi i32 [ %spec.store.select11.i, %bb16.i.bb13.i.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i48618059, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i491 ], !dbg !7410
  %_111.not.i = icmp eq i32 %727, 0, !dbg !7368
  %indvars.iv.next20492 = add i32 %indvars.iv20491, -32, !dbg !7368
  br i1 %_111.not.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb37.i, !dbg !7368

bb37.i:                                           ; preds = %bb37.i.lr.ph, %bb13.i.loopexit
  %indvars.iv20491 = phi i32 [ %frames, %bb37.i.lr.ph ], [ %indvars.iv.next20492, %bb13.i.loopexit ]
  %main_cursor.sroa.0.0.i48618059 = phi i32 [ %_35.i, %bb37.i.lr.ph ], [ %main_cursor.sroa.0.1.i494.lcssa, %bb13.i.loopexit ]
  %ring_cursor.sroa.0.0.i48518058 = phi i32 [ %_37.i479, %bb37.i.lr.ph ], [ %ring_cursor.sroa.0.1.i493.lcssa, %bb13.i.loopexit ]
  %iter2.sroa.0.0.i48418057 = phi i32 [ %yield_count.sroa.0.0.i6493, %bb37.i.lr.ph ], [ %727, %bb13.i.loopexit ]
  %iter.sroa.0.0.i18056 = phi i32 [ 0, %bb37.i.lr.ph ], [ %726, %bb13.i.loopexit ]
  %725 = call i32 @llvm.umax.i32(i32 %indvars.iv20491, i32 1), !dbg !7411
  %umax20518 = call i32 @llvm.umin.i32(i32 %725, i32 32), !dbg !7411
  %726 = add i32 %iter.sroa.0.0.i18056, 32, !dbg !7411
  %727 = add nsw i32 %iter2.sroa.0.0.i48418057, -1, !dbg !7415
  %history.i140.i.sroa.0.0.copyload = load <4 x i32>, ptr %hot_left.i472, align 16, !dbg !7416
  %history.i140.i.sroa.7.0.copyload = load <4 x i32>, ptr %history.i140.i.sroa.7.0.hot_left.i472.sroa_idx, align 16, !dbg !7416
  %history.i140.i.sroa.10.0.copyload = load <4 x i32>, ptr %history.i140.i.sroa.10.0.hot_left.i472.sroa_idx, align 16, !dbg !7416
  %history.i140.i.sroa.13.0.copyload = load <4 x i32>, ptr %history.i140.i.sroa.13.0.hot_left.i472.sroa_idx, align 16, !dbg !7416
  %history.i140.i.sroa.16.0.copyload = load <4 x i32>, ptr %history.i140.i.sroa.16.0.hot_left.i472.sroa_idx, align 16, !dbg !7416
  %history.i140.i.sroa.19.0.copyload = load <4 x i32>, ptr %history.i140.i.sroa.19.0.hot_left.i472.sroa_idx, align 16, !dbg !7416
  %history.i140.i.sroa.22.0.copyload = load <4 x i32>, ptr %history.i140.i.sroa.22.0.hot_left.i472.sroa_idx, align 16, !dbg !7416
  %history.i140.i.sroa.26.0.copyload = load <4 x i32>, ptr %history.i140.i.sroa.26.0.hot_left.i472.sroa_idx, align 16, !dbg !7416
  %history.i140.i.sroa.29.0.copyload = load <4 x i32>, ptr %history.i140.i.sroa.29.0.hot_left.i472.sroa_idx, align 16, !dbg !7416
  %history.i140.i.sroa.32.0.copyload = load <4 x i32>, ptr %history.i140.i.sroa.32.0.hot_left.i472.sroa_idx, align 16, !dbg !7416
  %history.i140.i.sroa.35.0.copyload = load <4 x i32>, ptr %history.i140.i.sroa.35.0.hot_left.i472.sroa_idx, align 16, !dbg !7416
  %history.i140.i.sroa.38.0.copyload = load <4 x i32>, ptr %history.i140.i.sroa.38.0.hot_left.i472.sroa_idx, align 16, !dbg !7416
  %_20.i143.i17826.not = icmp eq i32 %frames, %iter.sroa.0.0.i18056, !dbg !7418
  br i1 %_20.i143.i17826.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit348.i, label %bb5.i144.i.lr.ph, !dbg !7422

bb5.i144.i.lr.ph:                                 ; preds = %bb37.i
  %_11.i.i.i165.i15305 = load <4 x float>, ptr %_31, align 16
  %_14.i.i.i168.i15306 = load <4 x float>, ptr %655, align 16
  %_17.i.i.i171.i15307 = load <4 x float>, ptr %656, align 16
  %_20.i.i.i174.i15308 = load <4 x float>, ptr %657, align 16
  %_25.i.i.i179.i15309 = load <4 x float>, ptr %row1.i.i.i177.i, align 16
  %_28.i.i.i182.i15310 = load <4 x float>, ptr %658, align 16
  %_31.i.i.i185.i15311 = load <4 x float>, ptr %659, align 16
  %_34.i.i.i188.i15312 = load <4 x float>, ptr %660, align 16
  %_39.i.i.i193.i15313 = load <4 x float>, ptr %row3.i.i.i191.i, align 16
  %_42.i.i.i196.i15314 = load <4 x float>, ptr %661, align 16
  %_45.i.i.i199.i15315 = load <4 x float>, ptr %662, align 16
  %_48.i.i.i202.i15316 = load <4 x float>, ptr %663, align 16
  %_53.i.i.i207.i15317 = load <4 x float>, ptr %row5.i.i.i205.i, align 16
  %_56.i.i.i210.i15318 = load <4 x float>, ptr %664, align 16
  %_59.i.i.i213.i15319 = load <4 x float>, ptr %665, align 16
  %_62.i.i.i216.i15320 = load <4 x float>, ptr %666, align 16
  %_67.i.i.i221.i15321 = load <4 x float>, ptr %row7.i.i.i219.i, align 16
  %_70.i.i.i224.i15322 = load <4 x float>, ptr %667, align 16
  %_73.i.i.i227.i15323 = load <4 x float>, ptr %668, align 16
  %_76.i.i.i230.i15324 = load <4 x float>, ptr %669, align 16
  %_81.i.i.i235.i15325 = load <4 x float>, ptr %row9.i.i.i233.i, align 16
  %_84.i.i.i238.i15326 = load <4 x float>, ptr %670, align 16
  %_87.i.i.i241.i15327 = load <4 x float>, ptr %671, align 16
  %_90.i.i.i244.i15328 = load <4 x float>, ptr %672, align 16
  %_95.i.i.i249.i15329 = load <4 x float>, ptr %row11.i.i.i247.i, align 16
  %_98.i.i.i252.i15330 = load <4 x float>, ptr %673, align 16
  %_101.i.i.i255.i15331 = load <4 x float>, ptr %674, align 16
  %_104.i.i.i258.i15332 = load <4 x float>, ptr %675, align 16
  %_109.i.i.i263.i15333 = load <4 x float>, ptr %row13.i.i.i261.i, align 16
  %_112.i.i.i266.i15334 = load <4 x float>, ptr %676, align 16
  %_115.i.i.i269.i15335 = load <4 x float>, ptr %677, align 16
  %_118.i.i.i272.i15336 = load <4 x float>, ptr %678, align 16
  %_123.i.i.i277.i15337 = load <4 x float>, ptr %row15.i.i.i275.i, align 16
  %_126.i.i.i280.i15338 = load <4 x float>, ptr %679, align 16
  %_129.i.i.i283.i15339 = load <4 x float>, ptr %680, align 16
  %_132.i.i.i286.i15340 = load <4 x float>, ptr %681, align 16
  %_137.i.i.i291.i15341 = load <4 x float>, ptr %row17.i.i.i289.i, align 16
  %_140.i.i.i294.i15342 = load <4 x float>, ptr %682, align 16
  %_143.i.i.i297.i15343 = load <4 x float>, ptr %683, align 16
  %_146.i.i.i300.i15344 = load <4 x float>, ptr %684, align 16
  %_151.i.i.i305.i15345 = load <4 x float>, ptr %row19.i.i.i303.i, align 16
  %_154.i.i.i308.i15346 = load <4 x float>, ptr %685, align 16
  %_157.i.i.i311.i15347 = load <4 x float>, ptr %686, align 16
  %_160.i.i.i314.i15348 = load <4 x float>, ptr %687, align 16
  %_165.i.i.i319.i15349 = load <4 x float>, ptr %row21.i.i.i317.i, align 16
  %_168.i.i.i322.i15350 = load <4 x float>, ptr %688, align 16
  %_171.i.i.i325.i15351 = load <4 x float>, ptr %689, align 16
  %_174.i.i.i328.i15352 = load <4 x float>, ptr %690, align 16
  br label %bb5.i144.i, !dbg !7422

bb5.i144.i:                                       ; preds = %bb5.i144.i.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932
  %iter.sroa.0.0.i142.i17838 = phi i32 [ 0, %bb5.i144.i.lr.ph ], [ %728, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932 ]
  %history.i140.i.sroa.35.017837 = phi <4 x i32> [ %history.i140.i.sroa.35.0.copyload, %bb5.i144.i.lr.ph ], [ %history.i140.i.sroa.32.017836, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932 ]
  %history.i140.i.sroa.32.017836 = phi <4 x i32> [ %history.i140.i.sroa.32.0.copyload, %bb5.i144.i.lr.ph ], [ %history.i140.i.sroa.29.017835, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932 ]
  %history.i140.i.sroa.29.017835 = phi <4 x i32> [ %history.i140.i.sroa.29.0.copyload, %bb5.i144.i.lr.ph ], [ %history.i140.i.sroa.26.017834, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932 ]
  %history.i140.i.sroa.26.017834 = phi <4 x i32> [ %history.i140.i.sroa.26.0.copyload, %bb5.i144.i.lr.ph ], [ %history.i140.i.sroa.22.017833, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932 ]
  %history.i140.i.sroa.22.017833 = phi <4 x i32> [ %history.i140.i.sroa.22.0.copyload, %bb5.i144.i.lr.ph ], [ %history.i140.i.sroa.19.017832, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932 ]
  %history.i140.i.sroa.19.017832 = phi <4 x i32> [ %history.i140.i.sroa.19.0.copyload, %bb5.i144.i.lr.ph ], [ %history.i140.i.sroa.16.017831, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932 ]
  %history.i140.i.sroa.16.017831 = phi <4 x i32> [ %history.i140.i.sroa.16.0.copyload, %bb5.i144.i.lr.ph ], [ %history.i140.i.sroa.13.017830, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932 ]
  %history.i140.i.sroa.13.017830 = phi <4 x i32> [ %history.i140.i.sroa.13.0.copyload, %bb5.i144.i.lr.ph ], [ %history.i140.i.sroa.10.017829, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932 ]
  %history.i140.i.sroa.10.017829 = phi <4 x i32> [ %history.i140.i.sroa.10.0.copyload, %bb5.i144.i.lr.ph ], [ %history.i140.i.sroa.7.017828, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932 ]
  %history.i140.i.sroa.7.017828 = phi <4 x i32> [ %history.i140.i.sroa.7.0.copyload, %bb5.i144.i.lr.ph ], [ %history.i140.i.sroa.0.017827, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932 ]
  %history.i140.i.sroa.0.017827 = phi <4 x i32> [ %history.i140.i.sroa.0.0.copyload, %bb5.i144.i.lr.ph ], [ %lanes.i5211.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932 ]
  %728 = add nuw nsw i32 %iter.sroa.0.0.i142.i17838, 1, !dbg !7423
  %_11.i145.i = add nuw nsw i32 %iter.sroa.0.0.i142.i17838, %iter.sroa.0.0.i18056, !dbg !7426
  %base.i146.i = shl i32 %_11.i145.i, 2, !dbg !7426
  %_24.i147.i = icmp ugt i32 %base.i146.i, %left_io.1, !dbg !7427
  br i1 %_24.i147.i, label %bb7.i347.i, label %bb8.i148.i, !dbg !7427, !prof !902

bb8.i148.i:                                       ; preds = %bb5.i144.i
  %_27.i149.i = sub nuw nsw i32 %left_io.1, %base.i146.i, !dbg !7430
  %_8.i5214 = icmp samesign ugt i32 %_27.i149.i, 3, !dbg !7431
  br i1 %_8.i5214, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932, label %bb2.i5215, !dbg !7431, !prof !1153

bb2.i5215:                                        ; preds = %bb8.i148.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_27.i149.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !7436, !noalias !7437
  unreachable, !dbg !7436

bb7.i347.i:                                       ; preds = %bb5.i144.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i146.i, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #33, !dbg !7444, !noalias !7445
  unreachable, !dbg !7444

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932: ; preds = %bb8.i148.i
  %_31.i150.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %base.i146.i, !dbg !7446
  %lanes.i5211.sroa.0.0.copyload = load <4 x i32>, ptr %_31.i150.i, align 4, !dbg !7448, !alias.scope !7452, !noalias !7456
  %729 = bitcast <4 x i32> %history.i140.i.sroa.19.017832 to <4 x float>, !dbg !7458
  %730 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %729), !dbg !7463
  %731 = bitcast <4 x i32> %lanes.i5211.sroa.0.0.copyload to <4 x float>, !dbg !7464
  %732 = fmul <4 x float> %_11.i.i.i165.i15305, %731, !dbg !7469
  %733 = fadd <4 x float> %732, zeroinitializer, !dbg !7470
  %734 = fmul <4 x float> %_14.i.i.i168.i15306, %731, !dbg !7474
  %735 = fadd <4 x float> %734, zeroinitializer, !dbg !7478
  %736 = fmul <4 x float> %_17.i.i.i171.i15307, %731, !dbg !7482
  %737 = fadd <4 x float> %736, zeroinitializer, !dbg !7486
  %738 = fmul <4 x float> %_20.i.i.i174.i15308, %731, !dbg !7490
  %739 = fadd <4 x float> %738, zeroinitializer, !dbg !7494
  %740 = bitcast <4 x i32> %history.i140.i.sroa.0.017827 to <4 x float>, !dbg !7498
  %741 = fmul <4 x float> %_25.i.i.i179.i15309, %740, !dbg !7502
  %742 = fadd <4 x float> %733, %741, !dbg !7503
  %743 = fmul <4 x float> %_28.i.i.i182.i15310, %740, !dbg !7507
  %744 = fadd <4 x float> %735, %743, !dbg !7511
  %745 = fmul <4 x float> %_31.i.i.i185.i15311, %740, !dbg !7515
  %746 = fadd <4 x float> %737, %745, !dbg !7519
  %747 = fmul <4 x float> %_34.i.i.i188.i15312, %740, !dbg !7523
  %748 = fadd <4 x float> %739, %747, !dbg !7527
  %749 = bitcast <4 x i32> %history.i140.i.sroa.7.017828 to <4 x float>, !dbg !7531
  %750 = fmul <4 x float> %_39.i.i.i193.i15313, %749, !dbg !7535
  %751 = fadd <4 x float> %742, %750, !dbg !7536
  %752 = fmul <4 x float> %_42.i.i.i196.i15314, %749, !dbg !7540
  %753 = fadd <4 x float> %744, %752, !dbg !7544
  %754 = fmul <4 x float> %_45.i.i.i199.i15315, %749, !dbg !7548
  %755 = fadd <4 x float> %746, %754, !dbg !7552
  %756 = fmul <4 x float> %_48.i.i.i202.i15316, %749, !dbg !7556
  %757 = fadd <4 x float> %748, %756, !dbg !7560
  %758 = bitcast <4 x i32> %history.i140.i.sroa.10.017829 to <4 x float>, !dbg !7564
  %759 = fmul <4 x float> %_53.i.i.i207.i15317, %758, !dbg !7568
  %760 = fadd <4 x float> %751, %759, !dbg !7569
  %761 = fmul <4 x float> %_56.i.i.i210.i15318, %758, !dbg !7573
  %762 = fadd <4 x float> %753, %761, !dbg !7577
  %763 = fmul <4 x float> %_59.i.i.i213.i15319, %758, !dbg !7581
  %764 = fadd <4 x float> %755, %763, !dbg !7585
  %765 = fmul <4 x float> %_62.i.i.i216.i15320, %758, !dbg !7589
  %766 = fadd <4 x float> %757, %765, !dbg !7593
  %767 = bitcast <4 x i32> %history.i140.i.sroa.13.017830 to <4 x float>, !dbg !7597
  %768 = fmul <4 x float> %_67.i.i.i221.i15321, %767, !dbg !7601
  %769 = fadd <4 x float> %760, %768, !dbg !7602
  %770 = fmul <4 x float> %_70.i.i.i224.i15322, %767, !dbg !7606
  %771 = fadd <4 x float> %762, %770, !dbg !7610
  %772 = fmul <4 x float> %_73.i.i.i227.i15323, %767, !dbg !7614
  %773 = fadd <4 x float> %764, %772, !dbg !7618
  %774 = fmul <4 x float> %_76.i.i.i230.i15324, %767, !dbg !7622
  %775 = fadd <4 x float> %766, %774, !dbg !7626
  %776 = bitcast <4 x i32> %history.i140.i.sroa.16.017831 to <4 x float>, !dbg !7630
  %777 = fmul <4 x float> %_81.i.i.i235.i15325, %776, !dbg !7634
  %778 = fadd <4 x float> %769, %777, !dbg !7635
  %779 = fmul <4 x float> %_84.i.i.i238.i15326, %776, !dbg !7639
  %780 = fadd <4 x float> %771, %779, !dbg !7643
  %781 = fmul <4 x float> %_87.i.i.i241.i15327, %776, !dbg !7647
  %782 = fadd <4 x float> %773, %781, !dbg !7651
  %783 = fmul <4 x float> %_90.i.i.i244.i15328, %776, !dbg !7655
  %784 = fadd <4 x float> %775, %783, !dbg !7659
  %785 = fmul <4 x float> %_95.i.i.i249.i15329, %729, !dbg !7663
  %786 = fadd <4 x float> %778, %785, !dbg !7667
  %787 = fmul <4 x float> %_98.i.i.i252.i15330, %729, !dbg !7671
  %788 = fadd <4 x float> %780, %787, !dbg !7675
  %789 = fmul <4 x float> %_101.i.i.i255.i15331, %729, !dbg !7679
  %790 = fadd <4 x float> %782, %789, !dbg !7683
  %791 = fmul <4 x float> %_104.i.i.i258.i15332, %729, !dbg !7687
  %792 = fadd <4 x float> %784, %791, !dbg !7691
  %793 = bitcast <4 x i32> %history.i140.i.sroa.22.017833 to <4 x float>, !dbg !7695
  %794 = fmul <4 x float> %_109.i.i.i263.i15333, %793, !dbg !7699
  %795 = fadd <4 x float> %786, %794, !dbg !7700
  %796 = fmul <4 x float> %_112.i.i.i266.i15334, %793, !dbg !7704
  %797 = fadd <4 x float> %788, %796, !dbg !7708
  %798 = fmul <4 x float> %_115.i.i.i269.i15335, %793, !dbg !7712
  %799 = fadd <4 x float> %790, %798, !dbg !7716
  %800 = fmul <4 x float> %_118.i.i.i272.i15336, %793, !dbg !7720
  %801 = fadd <4 x float> %792, %800, !dbg !7724
  %802 = bitcast <4 x i32> %history.i140.i.sroa.26.017834 to <4 x float>, !dbg !7728
  %803 = fmul <4 x float> %_123.i.i.i277.i15337, %802, !dbg !7732
  %804 = fadd <4 x float> %795, %803, !dbg !7733
  %805 = fmul <4 x float> %_126.i.i.i280.i15338, %802, !dbg !7737
  %806 = fadd <4 x float> %797, %805, !dbg !7741
  %807 = fmul <4 x float> %_129.i.i.i283.i15339, %802, !dbg !7745
  %808 = fadd <4 x float> %799, %807, !dbg !7749
  %809 = fmul <4 x float> %_132.i.i.i286.i15340, %802, !dbg !7753
  %810 = fadd <4 x float> %801, %809, !dbg !7757
  %811 = bitcast <4 x i32> %history.i140.i.sroa.29.017835 to <4 x float>, !dbg !7761
  %812 = fmul <4 x float> %_137.i.i.i291.i15341, %811, !dbg !7765
  %813 = fadd <4 x float> %804, %812, !dbg !7766
  %814 = fmul <4 x float> %_140.i.i.i294.i15342, %811, !dbg !7770
  %815 = fadd <4 x float> %806, %814, !dbg !7774
  %816 = fmul <4 x float> %_143.i.i.i297.i15343, %811, !dbg !7778
  %817 = fadd <4 x float> %808, %816, !dbg !7782
  %818 = fmul <4 x float> %_146.i.i.i300.i15344, %811, !dbg !7786
  %819 = fadd <4 x float> %810, %818, !dbg !7790
  %820 = bitcast <4 x i32> %history.i140.i.sroa.32.017836 to <4 x float>, !dbg !7794
  %821 = fmul <4 x float> %_151.i.i.i305.i15345, %820, !dbg !7798
  %822 = fadd <4 x float> %813, %821, !dbg !7799
  %823 = fmul <4 x float> %_154.i.i.i308.i15346, %820, !dbg !7803
  %824 = fadd <4 x float> %815, %823, !dbg !7807
  %825 = fmul <4 x float> %_157.i.i.i311.i15347, %820, !dbg !7811
  %826 = fadd <4 x float> %817, %825, !dbg !7815
  %827 = fmul <4 x float> %_160.i.i.i314.i15348, %820, !dbg !7819
  %828 = fadd <4 x float> %819, %827, !dbg !7823
  %829 = bitcast <4 x i32> %history.i140.i.sroa.35.017837 to <4 x float>, !dbg !7827
  %830 = fmul <4 x float> %_165.i.i.i319.i15349, %829, !dbg !7831
  %831 = fadd <4 x float> %822, %830, !dbg !7832
  %832 = fmul <4 x float> %_168.i.i.i322.i15350, %829, !dbg !7836
  %833 = fadd <4 x float> %824, %832, !dbg !7840
  %834 = fmul <4 x float> %_171.i.i.i325.i15351, %829, !dbg !7844
  %835 = fadd <4 x float> %826, %834, !dbg !7848
  %836 = fmul <4 x float> %_174.i.i.i328.i15352, %829, !dbg !7852
  %837 = fadd <4 x float> %828, %836, !dbg !7856
  %838 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %831), !dbg !7860
  %839 = fcmp olt <4 x float> %838, %730, !dbg !7864
  %840 = select <4 x i1> %839, <4 x float> %730, <4 x float> %838, !dbg !7868
  %841 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %833), !dbg !7860
  %842 = fcmp olt <4 x float> %841, %840, !dbg !7864
  %843 = select <4 x i1> %842, <4 x float> %840, <4 x float> %841, !dbg !7868
  %844 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %835), !dbg !7860
  %845 = fcmp olt <4 x float> %844, %843, !dbg !7864
  %846 = select <4 x i1> %845, <4 x float> %843, <4 x float> %844, !dbg !7868
  %847 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %837), !dbg !7860
  %848 = fcmp olt <4 x float> %847, %846, !dbg !7864
  %849 = select <4 x i1> %848, <4 x float> %846, <4 x float> %847, !dbg !7868
  %_39.i341.i.idx = shl i32 %iter.sroa.0.0.i142.i17838, 4, !dbg !7869
  %_39.i341.i = getelementptr inbounds nuw i8, ptr %peaks_left.i469, i32 %_39.i341.i.idx, !dbg !7869
  store <4 x float> %849, ptr %_39.i341.i, align 4, !dbg !7874, !alias.scope !7879, !noalias !7883
  %exitcond20495.not = icmp eq i32 %728, %umax20518, !dbg !7418
  br i1 %exitcond20495.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit348.i, label %bb5.i144.i, !dbg !7422

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit348.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932, %bb37.i
  %history.i140.i.sroa.0.0.lcssa = phi <4 x i32> [ %history.i140.i.sroa.0.0.copyload, %bb37.i ], [ %lanes.i5211.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932 ], !dbg !7887
  %history.i140.i.sroa.7.0.lcssa = phi <4 x i32> [ %history.i140.i.sroa.7.0.copyload, %bb37.i ], [ %history.i140.i.sroa.0.017827, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932 ], !dbg !7887
  %history.i140.i.sroa.10.0.lcssa = phi <4 x i32> [ %history.i140.i.sroa.10.0.copyload, %bb37.i ], [ %history.i140.i.sroa.7.017828, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932 ], !dbg !7887
  %history.i140.i.sroa.13.0.lcssa = phi <4 x i32> [ %history.i140.i.sroa.13.0.copyload, %bb37.i ], [ %history.i140.i.sroa.10.017829, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932 ], !dbg !7887
  %history.i140.i.sroa.16.0.lcssa = phi <4 x i32> [ %history.i140.i.sroa.16.0.copyload, %bb37.i ], [ %history.i140.i.sroa.13.017830, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932 ], !dbg !7887
  %history.i140.i.sroa.19.0.lcssa = phi <4 x i32> [ %history.i140.i.sroa.19.0.copyload, %bb37.i ], [ %history.i140.i.sroa.16.017831, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932 ], !dbg !7887
  %history.i140.i.sroa.22.0.lcssa = phi <4 x i32> [ %history.i140.i.sroa.22.0.copyload, %bb37.i ], [ %history.i140.i.sroa.19.017832, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932 ], !dbg !7887
  %history.i140.i.sroa.26.0.lcssa = phi <4 x i32> [ %history.i140.i.sroa.26.0.copyload, %bb37.i ], [ %history.i140.i.sroa.22.017833, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932 ], !dbg !7887
  %history.i140.i.sroa.29.0.lcssa = phi <4 x i32> [ %history.i140.i.sroa.29.0.copyload, %bb37.i ], [ %history.i140.i.sroa.26.017834, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932 ], !dbg !7887
  %history.i140.i.sroa.32.0.lcssa = phi <4 x i32> [ %history.i140.i.sroa.32.0.copyload, %bb37.i ], [ %history.i140.i.sroa.29.017835, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932 ], !dbg !7887
  %history.i140.i.sroa.35.0.lcssa = phi <4 x i32> [ %history.i140.i.sroa.35.0.copyload, %bb37.i ], [ %history.i140.i.sroa.32.017836, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932 ], !dbg !7887
  %history.i140.i.sroa.38.0.lcssa = phi <4 x i32> [ %history.i140.i.sroa.38.0.copyload, %bb37.i ], [ %history.i140.i.sroa.35.017837, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5932 ], !dbg !7887
  store <4 x i32> %history.i140.i.sroa.0.0.lcssa, ptr %hot_left.i472, align 16, !dbg !7888
  store <4 x i32> %history.i140.i.sroa.7.0.lcssa, ptr %history.i140.i.sroa.7.0.hot_left.i472.sroa_idx, align 16, !dbg !7888
  store <4 x i32> %history.i140.i.sroa.10.0.lcssa, ptr %history.i140.i.sroa.10.0.hot_left.i472.sroa_idx, align 16, !dbg !7888
  store <4 x i32> %history.i140.i.sroa.13.0.lcssa, ptr %history.i140.i.sroa.13.0.hot_left.i472.sroa_idx, align 16, !dbg !7888
  store <4 x i32> %history.i140.i.sroa.16.0.lcssa, ptr %history.i140.i.sroa.16.0.hot_left.i472.sroa_idx, align 16, !dbg !7888
  store <4 x i32> %history.i140.i.sroa.19.0.lcssa, ptr %history.i140.i.sroa.19.0.hot_left.i472.sroa_idx, align 16, !dbg !7888
  store <4 x i32> %history.i140.i.sroa.22.0.lcssa, ptr %history.i140.i.sroa.22.0.hot_left.i472.sroa_idx, align 16, !dbg !7888
  store <4 x i32> %history.i140.i.sroa.26.0.lcssa, ptr %history.i140.i.sroa.26.0.hot_left.i472.sroa_idx, align 16, !dbg !7888
  store <4 x i32> %history.i140.i.sroa.29.0.lcssa, ptr %history.i140.i.sroa.29.0.hot_left.i472.sroa_idx, align 16, !dbg !7888
  store <4 x i32> %history.i140.i.sroa.32.0.lcssa, ptr %history.i140.i.sroa.32.0.hot_left.i472.sroa_idx, align 16, !dbg !7888
  store <4 x i32> %history.i140.i.sroa.35.0.lcssa, ptr %history.i140.i.sroa.35.0.hot_left.i472.sroa_idx, align 16, !dbg !7888
  store <4 x i32> %history.i140.i.sroa.38.0.lcssa, ptr %history.i140.i.sroa.38.0.hot_left.i472.sroa_idx, align 16, !dbg !7888
  %history.i.i466.sroa.0.0.copyload = load <4 x i32>, ptr %hot_right.i471, align 16, !dbg !7889
  %history.i.i466.sroa.7.0.copyload = load <4 x i32>, ptr %history.i.i466.sroa.7.0.hot_right.i471.sroa_idx, align 16, !dbg !7889
  %history.i.i466.sroa.10.0.copyload = load <4 x i32>, ptr %history.i.i466.sroa.10.0.hot_right.i471.sroa_idx, align 16, !dbg !7889
  %history.i.i466.sroa.13.0.copyload = load <4 x i32>, ptr %history.i.i466.sroa.13.0.hot_right.i471.sroa_idx, align 16, !dbg !7889
  %history.i.i466.sroa.16.0.copyload = load <4 x i32>, ptr %history.i.i466.sroa.16.0.hot_right.i471.sroa_idx, align 16, !dbg !7889
  %history.i.i466.sroa.19.0.copyload = load <4 x i32>, ptr %history.i.i466.sroa.19.0.hot_right.i471.sroa_idx, align 16, !dbg !7889
  %history.i.i466.sroa.22.0.copyload = load <4 x i32>, ptr %history.i.i466.sroa.22.0.hot_right.i471.sroa_idx, align 16, !dbg !7889
  %history.i.i466.sroa.26.0.copyload = load <4 x i32>, ptr %history.i.i466.sroa.26.0.hot_right.i471.sroa_idx, align 16, !dbg !7889
  %history.i.i466.sroa.29.0.copyload = load <4 x i32>, ptr %history.i.i466.sroa.29.0.hot_right.i471.sroa_idx, align 16, !dbg !7889
  %history.i.i466.sroa.32.0.copyload = load <4 x i32>, ptr %history.i.i466.sroa.32.0.hot_right.i471.sroa_idx, align 16, !dbg !7889
  %history.i.i466.sroa.35.0.copyload = load <4 x i32>, ptr %history.i.i466.sroa.35.0.hot_right.i471.sroa_idx, align 16, !dbg !7889
  %history.i.i466.sroa.38.0.copyload = load <4 x i32>, ptr %history.i.i466.sroa.38.0.hot_right.i471.sroa_idx, align 16, !dbg !7889
  br i1 %_20.i143.i17826.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i491, label %bb5.i.i534.lr.ph, !dbg !7891

bb5.i.i534.lr.ph:                                 ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit348.i
  %_11.i.i.i.i55415252 = load <4 x float>, ptr %_31, align 16
  %_14.i.i.i.i55715253 = load <4 x float>, ptr %655, align 16
  %_17.i.i.i.i56015254 = load <4 x float>, ptr %656, align 16
  %_20.i.i.i.i56315255 = load <4 x float>, ptr %657, align 16
  %_25.i.i.i.i56815256 = load <4 x float>, ptr %row1.i.i.i177.i, align 16
  %_28.i.i.i.i57115257 = load <4 x float>, ptr %658, align 16
  %_31.i.i.i.i57415258 = load <4 x float>, ptr %659, align 16
  %_34.i.i.i.i57715259 = load <4 x float>, ptr %660, align 16
  %_39.i.i.i.i58215260 = load <4 x float>, ptr %row3.i.i.i191.i, align 16
  %_42.i.i.i.i58515261 = load <4 x float>, ptr %661, align 16
  %_45.i.i.i.i58815262 = load <4 x float>, ptr %662, align 16
  %_48.i.i.i.i59115263 = load <4 x float>, ptr %663, align 16
  %_53.i.i.i.i59615264 = load <4 x float>, ptr %row5.i.i.i205.i, align 16
  %_56.i.i.i.i59915265 = load <4 x float>, ptr %664, align 16
  %_59.i.i.i.i60215266 = load <4 x float>, ptr %665, align 16
  %_62.i.i.i.i60515267 = load <4 x float>, ptr %666, align 16
  %_67.i.i.i.i61015268 = load <4 x float>, ptr %row7.i.i.i219.i, align 16
  %_70.i.i.i.i61315269 = load <4 x float>, ptr %667, align 16
  %_73.i.i.i.i61615270 = load <4 x float>, ptr %668, align 16
  %_76.i.i.i.i61915271 = load <4 x float>, ptr %669, align 16
  %_81.i.i.i.i62415272 = load <4 x float>, ptr %row9.i.i.i233.i, align 16
  %_84.i.i.i.i62715273 = load <4 x float>, ptr %670, align 16
  %_87.i.i.i.i63015274 = load <4 x float>, ptr %671, align 16
  %_90.i.i.i.i63315275 = load <4 x float>, ptr %672, align 16
  %_95.i.i.i.i63815276 = load <4 x float>, ptr %row11.i.i.i247.i, align 16
  %_98.i.i.i.i64115277 = load <4 x float>, ptr %673, align 16
  %_101.i.i.i.i64415278 = load <4 x float>, ptr %674, align 16
  %_104.i.i.i.i64715279 = load <4 x float>, ptr %675, align 16
  %_109.i.i.i.i65215280 = load <4 x float>, ptr %row13.i.i.i261.i, align 16
  %_112.i.i.i.i65515281 = load <4 x float>, ptr %676, align 16
  %_115.i.i.i.i65815282 = load <4 x float>, ptr %677, align 16
  %_118.i.i.i.i66115283 = load <4 x float>, ptr %678, align 16
  %_123.i.i.i.i66615284 = load <4 x float>, ptr %row15.i.i.i275.i, align 16
  %_126.i.i.i.i66915285 = load <4 x float>, ptr %679, align 16
  %_129.i.i.i.i67215286 = load <4 x float>, ptr %680, align 16
  %_132.i.i.i.i67515287 = load <4 x float>, ptr %681, align 16
  %_137.i.i.i.i68015288 = load <4 x float>, ptr %row17.i.i.i289.i, align 16
  %_140.i.i.i.i68315289 = load <4 x float>, ptr %682, align 16
  %_143.i.i.i.i68615290 = load <4 x float>, ptr %683, align 16
  %_146.i.i.i.i68915291 = load <4 x float>, ptr %684, align 16
  %_151.i.i.i.i69415292 = load <4 x float>, ptr %row19.i.i.i303.i, align 16
  %_154.i.i.i.i69715293 = load <4 x float>, ptr %685, align 16
  %_157.i.i.i.i70015294 = load <4 x float>, ptr %686, align 16
  %_160.i.i.i.i70315295 = load <4 x float>, ptr %687, align 16
  %_165.i.i.i.i70815296 = load <4 x float>, ptr %row21.i.i.i317.i, align 16
  %_168.i.i.i.i71115297 = load <4 x float>, ptr %688, align 16
  %_171.i.i.i.i71415298 = load <4 x float>, ptr %689, align 16
  %_174.i.i.i.i71715299 = load <4 x float>, ptr %690, align 16
  br label %bb5.i.i534, !dbg !7891

bb5.i.i534:                                       ; preds = %bb5.i.i534.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937
  %iter.sroa.0.0.i.i48917865 = phi i32 [ 0, %bb5.i.i534.lr.ph ], [ %850, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937 ]
  %history.i.i466.sroa.35.017864 = phi <4 x i32> [ %history.i.i466.sroa.35.0.copyload, %bb5.i.i534.lr.ph ], [ %history.i.i466.sroa.32.017863, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937 ]
  %history.i.i466.sroa.32.017863 = phi <4 x i32> [ %history.i.i466.sroa.32.0.copyload, %bb5.i.i534.lr.ph ], [ %history.i.i466.sroa.29.017862, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937 ]
  %history.i.i466.sroa.29.017862 = phi <4 x i32> [ %history.i.i466.sroa.29.0.copyload, %bb5.i.i534.lr.ph ], [ %history.i.i466.sroa.26.017861, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937 ]
  %history.i.i466.sroa.26.017861 = phi <4 x i32> [ %history.i.i466.sroa.26.0.copyload, %bb5.i.i534.lr.ph ], [ %history.i.i466.sroa.22.017860, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937 ]
  %history.i.i466.sroa.22.017860 = phi <4 x i32> [ %history.i.i466.sroa.22.0.copyload, %bb5.i.i534.lr.ph ], [ %history.i.i466.sroa.19.017859, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937 ]
  %history.i.i466.sroa.19.017859 = phi <4 x i32> [ %history.i.i466.sroa.19.0.copyload, %bb5.i.i534.lr.ph ], [ %history.i.i466.sroa.16.017858, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937 ]
  %history.i.i466.sroa.16.017858 = phi <4 x i32> [ %history.i.i466.sroa.16.0.copyload, %bb5.i.i534.lr.ph ], [ %history.i.i466.sroa.13.017857, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937 ]
  %history.i.i466.sroa.13.017857 = phi <4 x i32> [ %history.i.i466.sroa.13.0.copyload, %bb5.i.i534.lr.ph ], [ %history.i.i466.sroa.10.017856, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937 ]
  %history.i.i466.sroa.10.017856 = phi <4 x i32> [ %history.i.i466.sroa.10.0.copyload, %bb5.i.i534.lr.ph ], [ %history.i.i466.sroa.7.017855, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937 ]
  %history.i.i466.sroa.7.017855 = phi <4 x i32> [ %history.i.i466.sroa.7.0.copyload, %bb5.i.i534.lr.ph ], [ %history.i.i466.sroa.0.017854, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937 ]
  %history.i.i466.sroa.0.017854 = phi <4 x i32> [ %history.i.i466.sroa.0.0.copyload, %bb5.i.i534.lr.ph ], [ %lanes.i5220.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937 ]
  %850 = add nuw nsw i32 %iter.sroa.0.0.i.i48917865, 1, !dbg !7894
  %_11.i.i535 = add nuw nsw i32 %iter.sroa.0.0.i.i48917865, %iter.sroa.0.0.i18056, !dbg !7897
  %base.i.i536 = shl i32 %_11.i.i535, 2, !dbg !7897
  %_24.i.i537 = icmp ugt i32 %base.i.i536, %right_io.1, !dbg !7898
  br i1 %_24.i.i537, label %bb7.i.i735, label %bb8.i.i538, !dbg !7898, !prof !902

bb8.i.i538:                                       ; preds = %bb5.i.i534
  %_27.i.i539 = sub nuw nsw i32 %right_io.1, %base.i.i536, !dbg !7901
  %_8.i5223 = icmp samesign ugt i32 %_27.i.i539, 3, !dbg !7902
  br i1 %_8.i5223, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937, label %bb2.i5224, !dbg !7902, !prof !1153

bb2.i5224:                                        ; preds = %bb8.i.i538
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_27.i.i539, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !7907, !noalias !7908
  unreachable, !dbg !7907

bb7.i.i735:                                       ; preds = %bb5.i.i534
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i.i536, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #33, !dbg !7915, !noalias !7916
  unreachable, !dbg !7915

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937: ; preds = %bb8.i.i538
  %_31.i125.i = getelementptr inbounds nuw float, ptr %right_io.0, i32 %base.i.i536, !dbg !7917
  %lanes.i5220.sroa.0.0.copyload = load <4 x i32>, ptr %_31.i125.i, align 4, !dbg !7919, !alias.scope !7923, !noalias !7927
  %851 = bitcast <4 x i32> %history.i.i466.sroa.19.017859 to <4 x float>, !dbg !7929
  %852 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %851), !dbg !7934
  %853 = bitcast <4 x i32> %lanes.i5220.sroa.0.0.copyload to <4 x float>, !dbg !7935
  %854 = fmul <4 x float> %_11.i.i.i.i55415252, %853, !dbg !7940
  %855 = fadd <4 x float> %854, zeroinitializer, !dbg !7941
  %856 = fmul <4 x float> %_14.i.i.i.i55715253, %853, !dbg !7945
  %857 = fadd <4 x float> %856, zeroinitializer, !dbg !7949
  %858 = fmul <4 x float> %_17.i.i.i.i56015254, %853, !dbg !7953
  %859 = fadd <4 x float> %858, zeroinitializer, !dbg !7957
  %860 = fmul <4 x float> %_20.i.i.i.i56315255, %853, !dbg !7961
  %861 = fadd <4 x float> %860, zeroinitializer, !dbg !7965
  %862 = bitcast <4 x i32> %history.i.i466.sroa.0.017854 to <4 x float>, !dbg !7969
  %863 = fmul <4 x float> %_25.i.i.i.i56815256, %862, !dbg !7973
  %864 = fadd <4 x float> %855, %863, !dbg !7974
  %865 = fmul <4 x float> %_28.i.i.i.i57115257, %862, !dbg !7978
  %866 = fadd <4 x float> %857, %865, !dbg !7982
  %867 = fmul <4 x float> %_31.i.i.i.i57415258, %862, !dbg !7986
  %868 = fadd <4 x float> %859, %867, !dbg !7990
  %869 = fmul <4 x float> %_34.i.i.i.i57715259, %862, !dbg !7994
  %870 = fadd <4 x float> %861, %869, !dbg !7998
  %871 = bitcast <4 x i32> %history.i.i466.sroa.7.017855 to <4 x float>, !dbg !8002
  %872 = fmul <4 x float> %_39.i.i.i.i58215260, %871, !dbg !8006
  %873 = fadd <4 x float> %864, %872, !dbg !8007
  %874 = fmul <4 x float> %_42.i.i.i.i58515261, %871, !dbg !8011
  %875 = fadd <4 x float> %866, %874, !dbg !8015
  %876 = fmul <4 x float> %_45.i.i.i.i58815262, %871, !dbg !8019
  %877 = fadd <4 x float> %868, %876, !dbg !8023
  %878 = fmul <4 x float> %_48.i.i.i.i59115263, %871, !dbg !8027
  %879 = fadd <4 x float> %870, %878, !dbg !8031
  %880 = bitcast <4 x i32> %history.i.i466.sroa.10.017856 to <4 x float>, !dbg !8035
  %881 = fmul <4 x float> %_53.i.i.i.i59615264, %880, !dbg !8039
  %882 = fadd <4 x float> %873, %881, !dbg !8040
  %883 = fmul <4 x float> %_56.i.i.i.i59915265, %880, !dbg !8044
  %884 = fadd <4 x float> %875, %883, !dbg !8048
  %885 = fmul <4 x float> %_59.i.i.i.i60215266, %880, !dbg !8052
  %886 = fadd <4 x float> %877, %885, !dbg !8056
  %887 = fmul <4 x float> %_62.i.i.i.i60515267, %880, !dbg !8060
  %888 = fadd <4 x float> %879, %887, !dbg !8064
  %889 = bitcast <4 x i32> %history.i.i466.sroa.13.017857 to <4 x float>, !dbg !8068
  %890 = fmul <4 x float> %_67.i.i.i.i61015268, %889, !dbg !8072
  %891 = fadd <4 x float> %882, %890, !dbg !8073
  %892 = fmul <4 x float> %_70.i.i.i.i61315269, %889, !dbg !8077
  %893 = fadd <4 x float> %884, %892, !dbg !8081
  %894 = fmul <4 x float> %_73.i.i.i.i61615270, %889, !dbg !8085
  %895 = fadd <4 x float> %886, %894, !dbg !8089
  %896 = fmul <4 x float> %_76.i.i.i.i61915271, %889, !dbg !8093
  %897 = fadd <4 x float> %888, %896, !dbg !8097
  %898 = bitcast <4 x i32> %history.i.i466.sroa.16.017858 to <4 x float>, !dbg !8101
  %899 = fmul <4 x float> %_81.i.i.i.i62415272, %898, !dbg !8105
  %900 = fadd <4 x float> %891, %899, !dbg !8106
  %901 = fmul <4 x float> %_84.i.i.i.i62715273, %898, !dbg !8110
  %902 = fadd <4 x float> %893, %901, !dbg !8114
  %903 = fmul <4 x float> %_87.i.i.i.i63015274, %898, !dbg !8118
  %904 = fadd <4 x float> %895, %903, !dbg !8122
  %905 = fmul <4 x float> %_90.i.i.i.i63315275, %898, !dbg !8126
  %906 = fadd <4 x float> %897, %905, !dbg !8130
  %907 = fmul <4 x float> %_95.i.i.i.i63815276, %851, !dbg !8134
  %908 = fadd <4 x float> %900, %907, !dbg !8138
  %909 = fmul <4 x float> %_98.i.i.i.i64115277, %851, !dbg !8142
  %910 = fadd <4 x float> %902, %909, !dbg !8146
  %911 = fmul <4 x float> %_101.i.i.i.i64415278, %851, !dbg !8150
  %912 = fadd <4 x float> %904, %911, !dbg !8154
  %913 = fmul <4 x float> %_104.i.i.i.i64715279, %851, !dbg !8158
  %914 = fadd <4 x float> %906, %913, !dbg !8162
  %915 = bitcast <4 x i32> %history.i.i466.sroa.22.017860 to <4 x float>, !dbg !8166
  %916 = fmul <4 x float> %_109.i.i.i.i65215280, %915, !dbg !8170
  %917 = fadd <4 x float> %908, %916, !dbg !8171
  %918 = fmul <4 x float> %_112.i.i.i.i65515281, %915, !dbg !8175
  %919 = fadd <4 x float> %910, %918, !dbg !8179
  %920 = fmul <4 x float> %_115.i.i.i.i65815282, %915, !dbg !8183
  %921 = fadd <4 x float> %912, %920, !dbg !8187
  %922 = fmul <4 x float> %_118.i.i.i.i66115283, %915, !dbg !8191
  %923 = fadd <4 x float> %914, %922, !dbg !8195
  %924 = bitcast <4 x i32> %history.i.i466.sroa.26.017861 to <4 x float>, !dbg !8199
  %925 = fmul <4 x float> %_123.i.i.i.i66615284, %924, !dbg !8203
  %926 = fadd <4 x float> %917, %925, !dbg !8204
  %927 = fmul <4 x float> %_126.i.i.i.i66915285, %924, !dbg !8208
  %928 = fadd <4 x float> %919, %927, !dbg !8212
  %929 = fmul <4 x float> %_129.i.i.i.i67215286, %924, !dbg !8216
  %930 = fadd <4 x float> %921, %929, !dbg !8220
  %931 = fmul <4 x float> %_132.i.i.i.i67515287, %924, !dbg !8224
  %932 = fadd <4 x float> %923, %931, !dbg !8228
  %933 = bitcast <4 x i32> %history.i.i466.sroa.29.017862 to <4 x float>, !dbg !8232
  %934 = fmul <4 x float> %_137.i.i.i.i68015288, %933, !dbg !8236
  %935 = fadd <4 x float> %926, %934, !dbg !8237
  %936 = fmul <4 x float> %_140.i.i.i.i68315289, %933, !dbg !8241
  %937 = fadd <4 x float> %928, %936, !dbg !8245
  %938 = fmul <4 x float> %_143.i.i.i.i68615290, %933, !dbg !8249
  %939 = fadd <4 x float> %930, %938, !dbg !8253
  %940 = fmul <4 x float> %_146.i.i.i.i68915291, %933, !dbg !8257
  %941 = fadd <4 x float> %932, %940, !dbg !8261
  %942 = bitcast <4 x i32> %history.i.i466.sroa.32.017863 to <4 x float>, !dbg !8265
  %943 = fmul <4 x float> %_151.i.i.i.i69415292, %942, !dbg !8269
  %944 = fadd <4 x float> %935, %943, !dbg !8270
  %945 = fmul <4 x float> %_154.i.i.i.i69715293, %942, !dbg !8274
  %946 = fadd <4 x float> %937, %945, !dbg !8278
  %947 = fmul <4 x float> %_157.i.i.i.i70015294, %942, !dbg !8282
  %948 = fadd <4 x float> %939, %947, !dbg !8286
  %949 = fmul <4 x float> %_160.i.i.i.i70315295, %942, !dbg !8290
  %950 = fadd <4 x float> %941, %949, !dbg !8294
  %951 = bitcast <4 x i32> %history.i.i466.sroa.35.017864 to <4 x float>, !dbg !8298
  %952 = fmul <4 x float> %_165.i.i.i.i70815296, %951, !dbg !8302
  %953 = fadd <4 x float> %944, %952, !dbg !8303
  %954 = fmul <4 x float> %_168.i.i.i.i71115297, %951, !dbg !8307
  %955 = fadd <4 x float> %946, %954, !dbg !8311
  %956 = fmul <4 x float> %_171.i.i.i.i71415298, %951, !dbg !8315
  %957 = fadd <4 x float> %948, %956, !dbg !8319
  %958 = fmul <4 x float> %_174.i.i.i.i71715299, %951, !dbg !8323
  %959 = fadd <4 x float> %950, %958, !dbg !8327
  %960 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %953), !dbg !8331
  %961 = fcmp olt <4 x float> %960, %852, !dbg !8335
  %962 = select <4 x i1> %961, <4 x float> %852, <4 x float> %960, !dbg !8339
  %963 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %955), !dbg !8331
  %964 = fcmp olt <4 x float> %963, %962, !dbg !8335
  %965 = select <4 x i1> %964, <4 x float> %962, <4 x float> %963, !dbg !8339
  %966 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %957), !dbg !8331
  %967 = fcmp olt <4 x float> %966, %965, !dbg !8335
  %968 = select <4 x i1> %967, <4 x float> %965, <4 x float> %966, !dbg !8339
  %969 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %959), !dbg !8331
  %970 = fcmp olt <4 x float> %969, %968, !dbg !8335
  %971 = select <4 x i1> %970, <4 x float> %968, <4 x float> %969, !dbg !8339
  %_39.i.i729.idx = shl i32 %iter.sroa.0.0.i.i48917865, 4, !dbg !8340
  %_39.i.i729 = getelementptr inbounds nuw i8, ptr %peaks_right.i468, i32 %_39.i.i729.idx, !dbg !8340
  store <4 x float> %971, ptr %_39.i.i729, align 4, !dbg !8345, !alias.scope !8350, !noalias !8354
  %exitcond20498.not = icmp eq i32 %850, %umax20518, !dbg !8358
  br i1 %exitcond20498.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i491, label %bb5.i.i534, !dbg !7891

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i491: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit348.i
  %history.i.i466.sroa.0.0.lcssa = phi <4 x i32> [ %history.i.i466.sroa.0.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit348.i ], [ %lanes.i5220.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937 ], !dbg !8360
  %history.i.i466.sroa.7.0.lcssa = phi <4 x i32> [ %history.i.i466.sroa.7.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit348.i ], [ %history.i.i466.sroa.0.017854, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937 ], !dbg !8360
  %history.i.i466.sroa.10.0.lcssa = phi <4 x i32> [ %history.i.i466.sroa.10.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit348.i ], [ %history.i.i466.sroa.7.017855, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937 ], !dbg !8360
  %history.i.i466.sroa.13.0.lcssa = phi <4 x i32> [ %history.i.i466.sroa.13.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit348.i ], [ %history.i.i466.sroa.10.017856, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937 ], !dbg !8360
  %history.i.i466.sroa.16.0.lcssa = phi <4 x i32> [ %history.i.i466.sroa.16.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit348.i ], [ %history.i.i466.sroa.13.017857, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937 ], !dbg !8360
  %history.i.i466.sroa.19.0.lcssa = phi <4 x i32> [ %history.i.i466.sroa.19.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit348.i ], [ %history.i.i466.sroa.16.017858, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937 ], !dbg !8360
  %history.i.i466.sroa.22.0.lcssa = phi <4 x i32> [ %history.i.i466.sroa.22.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit348.i ], [ %history.i.i466.sroa.19.017859, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937 ], !dbg !8360
  %history.i.i466.sroa.26.0.lcssa = phi <4 x i32> [ %history.i.i466.sroa.26.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit348.i ], [ %history.i.i466.sroa.22.017860, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937 ], !dbg !8360
  %history.i.i466.sroa.29.0.lcssa = phi <4 x i32> [ %history.i.i466.sroa.29.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit348.i ], [ %history.i.i466.sroa.26.017861, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937 ], !dbg !8360
  %history.i.i466.sroa.32.0.lcssa = phi <4 x i32> [ %history.i.i466.sroa.32.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit348.i ], [ %history.i.i466.sroa.29.017862, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937 ], !dbg !8360
  %history.i.i466.sroa.35.0.lcssa = phi <4 x i32> [ %history.i.i466.sroa.35.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit348.i ], [ %history.i.i466.sroa.32.017863, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937 ], !dbg !8360
  %history.i.i466.sroa.38.0.lcssa = phi <4 x i32> [ %history.i.i466.sroa.38.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit348.i ], [ %history.i.i466.sroa.35.017864, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5937 ], !dbg !8360
  store <4 x i32> %history.i.i466.sroa.0.0.lcssa, ptr %hot_right.i471, align 16, !dbg !8361
  store <4 x i32> %history.i.i466.sroa.7.0.lcssa, ptr %history.i.i466.sroa.7.0.hot_right.i471.sroa_idx, align 16, !dbg !8361
  store <4 x i32> %history.i.i466.sroa.10.0.lcssa, ptr %history.i.i466.sroa.10.0.hot_right.i471.sroa_idx, align 16, !dbg !8361
  store <4 x i32> %history.i.i466.sroa.13.0.lcssa, ptr %history.i.i466.sroa.13.0.hot_right.i471.sroa_idx, align 16, !dbg !8361
  store <4 x i32> %history.i.i466.sroa.16.0.lcssa, ptr %history.i.i466.sroa.16.0.hot_right.i471.sroa_idx, align 16, !dbg !8361
  store <4 x i32> %history.i.i466.sroa.19.0.lcssa, ptr %history.i.i466.sroa.19.0.hot_right.i471.sroa_idx, align 16, !dbg !8361
  store <4 x i32> %history.i.i466.sroa.22.0.lcssa, ptr %history.i.i466.sroa.22.0.hot_right.i471.sroa_idx, align 16, !dbg !8361
  store <4 x i32> %history.i.i466.sroa.26.0.lcssa, ptr %history.i.i466.sroa.26.0.hot_right.i471.sroa_idx, align 16, !dbg !8361
  store <4 x i32> %history.i.i466.sroa.29.0.lcssa, ptr %history.i.i466.sroa.29.0.hot_right.i471.sroa_idx, align 16, !dbg !8361
  store <4 x i32> %history.i.i466.sroa.32.0.lcssa, ptr %history.i.i466.sroa.32.0.hot_right.i471.sroa_idx, align 16, !dbg !8361
  store <4 x i32> %history.i.i466.sroa.35.0.lcssa, ptr %history.i.i466.sroa.35.0.hot_right.i471.sroa_idx, align 16, !dbg !8361
  store <4 x i32> %history.i.i466.sroa.38.0.lcssa, ptr %history.i.i466.sroa.38.0.hot_right.i471.sroa_idx, align 16, !dbg !8361
  br i1 %_20.i143.i17826.not, label %bb13.i.loopexit, label %bb42.i.lr.ph, !dbg !7403

bb42.i.lr.ph:                                     ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i491
  %.promoted = load <4 x float>, ptr %702, align 16
  %.promoted17981 = load <4 x float>, ptr %719, align 16
  %_8.i29.i15225.pre = load <4 x float>, ptr %_64.i, align 16, !dbg !8362
  %_9.i30.i15226.pre = load <4 x float>, ptr %_65.i497, align 16, !dbg !8367
  %_8.i.i50115227.pre = load <4 x float>, ptr %_69.i500, align 16, !dbg !8369
  %_9.i.i50215228.pre = load <4 x float>, ptr %_70.i, align 16, !dbg !8372
  %_87.i = load i32, ptr %692, align 4
  %_62.i87.i15236 = load <4 x float>, ptr %703, align 16
  %_62.i.i52715245 = load <4 x float>, ptr %720, align 16
  %_102.i = load i32, ptr %724, align 4
  br label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5309, !dbg !7403

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5309: ; preds = %bb42.i.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5942
  %972 = phi <4 x float> [ %.promoted17981, %bb42.i.lr.ph ], [ %1100, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5942 ]
  %973 = phi <4 x float> [ %.promoted, %bb42.i.lr.ph ], [ %1012, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5942 ]
  %main_cursor.sroa.0.1.i49417905 = phi i32 [ %main_cursor.sroa.0.0.i48618059, %bb42.i.lr.ph ], [ %spec.store.select11.i, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5942 ]
  %ring_cursor.sroa.0.1.i49317904 = phi i32 [ %ring_cursor.sroa.0.0.i48518058, %bb42.i.lr.ph ], [ %spec.store.select12.i, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5942 ]
  %iter1.sroa.0.0.i49217903 = phi i32 [ 0, %bb42.i.lr.ph ], [ %974, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5942 ]
  %974 = add nuw nsw i32 %iter1.sroa.0.0.i49217903, 1, !dbg !8374
  %_60.i = add nuw nsw i32 %iter1.sroa.0.0.i49217903, %iter.sroa.0.0.i18056, !dbg !8380
  %base.i496 = shl i32 %_60.i, 2, !dbg !8380
  %_74.i = shl i32 %iter1.sroa.0.0.i49217903, 2, !dbg !8381
  %_126.i = getelementptr inbounds nuw float, ptr %peaks_left.i469, i32 %_74.i, !dbg !8382
  %lanes.i5302.sroa.0.0.copyload = load <4 x i32>, ptr %_126.i, align 4, !dbg !8393, !alias.scope !8398, !noalias !8402
  %_131.i506 = getelementptr inbounds nuw float, ptr %peaks_right.i468, i32 %_74.i, !dbg !8406
  %lanes.i5293.sroa.0.0.copyload = load <4 x i32>, ptr %_131.i506, align 4, !dbg !8416, !alias.scope !8421, !noalias !8425
  %975 = bitcast <4 x i32> %lanes.i5302.sroa.0.0.copyload to <4 x float>, !dbg !8429
  %976 = bitcast <4 x i32> %lanes.i5293.sroa.0.0.copyload to <4 x float>, !dbg !8433
  %977 = fcmp olt <4 x float> %975, %976, !dbg !8434
  %.v15229 = select <4 x i1> %977, <4 x i32> %lanes.i5293.sroa.0.0.copyload, <4 x i32> %lanes.i5302.sroa.0.0.copyload, !dbg !8435
  %978 = bitcast <4 x i32> %.v15229 to <16 x i8>, !dbg !8436
  %979 = bitcast <4 x i32> %lanes.i5302.sroa.0.0.copyload to <16 x i8>, !dbg !8440
  %_4.i6522 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %978, <16 x i8> %979, <16 x i8> %691), !dbg !8441
  %980 = bitcast <4 x i32> %lanes.i5293.sroa.0.0.copyload to <16 x i8>, !dbg !8442
  %_4.i6523 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %978, <16 x i8> %980, <16 x i8> %691), !dbg !8446
  %_132.i509 = icmp ugt i32 %base.i496, %left_io.1, !dbg !8447
  br i1 %_132.i509, label %bb46.i, label %bb47.i, !dbg !8447, !prof !902

bb47.i:                                           ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5309
  %_135.i = sub nuw nsw i32 %left_io.1, %base.i496, !dbg !8451
  %_139.i510 = getelementptr inbounds nuw float, ptr %left_io.0, i32 %base.i496, !dbg !8452
  %_8.i5287 = icmp samesign ugt i32 %_135.i, 3, !dbg !8457
  br i1 %_8.i5287, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5291, label %bb2.i5288, !dbg !8457, !prof !1153

bb2.i5288:                                        ; preds = %bb47.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_135.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !8462, !noalias !8463
  unreachable, !dbg !8462

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5291: ; preds = %bb47.i
  %lanes.i5284.sroa.0.0.copyload = load <4 x i32>, ptr %_139.i510, align 4, !dbg !8467, !alias.scope !8471, !noalias !8475
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8477), !dbg !8480
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8481), !dbg !8480
  %width.i31.i = load i32, ptr %693, align 4, !dbg !8483, !alias.scope !8484, !noalias !8485, !noundef !10
  %981 = bitcast <16 x i8> %_4.i6522 to <4 x float>, !dbg !8493
  %982 = fcmp olt <4 x float> %_8.i29.i15225.pre, %981, !dbg !8498
  %983 = sext <4 x i1> %982 to <4 x i32>, !dbg !8498
  %984 = fdiv <4 x float> %_8.i29.i15225.pre, %981, !dbg !8499
  %985 = bitcast <4 x float> %984 to <16 x i8>, !dbg !8503
  %986 = bitcast <4 x i32> %983 to <16 x i8>, !dbg !8507
  %_4.i6525 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %985, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %986), !dbg !8508
  %_158.1.i36.i = load i32, ptr %694, align 4, !dbg !8509, !alias.scope !8484, !noalias !8485, !noundef !10
  %_22.i37.i = mul i32 %width.i31.i, %ring_cursor.sroa.0.1.i49317904, !dbg !8510
  %_90.i38.i = icmp ugt i32 %_22.i37.i, %_158.1.i36.i, !dbg !8511
  br i1 %_90.i38.i, label %bb34.i123.i, label %bb35.i39.i, !dbg !8511, !prof !902

bb35.i39.i:                                       ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5291
  %_93.i41.i = sub nuw i32 %_158.1.i36.i, %_22.i37.i, !dbg !8514
  %_8.i5974 = icmp samesign ugt i32 %_93.i41.i, 3, !dbg !8515
  br i1 %_8.i5974, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5977, label %bb2.i5975, !dbg !8515, !prof !1153

bb2.i5975:                                        ; preds = %bb35.i39.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_93.i41.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !8520, !noalias !8521
  unreachable, !dbg !8520

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5977: ; preds = %bb35.i39.i
  %_158.0.i40.i = load ptr, ptr %695, align 4, !dbg !8509, !alias.scope !8484, !noalias !8485, !nonnull !10, !noundef !10
  %_97.i42.i = getelementptr inbounds nuw float, ptr %_158.0.i40.i, i32 %_22.i37.i, !dbg !8525
  store <16 x i8> %_4.i6525, ptr %_97.i42.i, align 4, !dbg !8527, !alias.scope !8531, !noalias !8535
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8537), !dbg !8540
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8541), !dbg !8540
  %width.i1801 = load i32, ptr %693, align 4, !dbg !8543, !alias.scope !8537, !noalias !8545, !noundef !10
  %987 = icmp eq i32 %width.i1801, 0, !dbg !8546
  br i1 %987, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1907, label %bb29.i1807.lr.ph, !dbg !8546

bb29.i1807.lr.ph:                                 ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5977
  %_126.1.i1812 = load i32, ptr %62, align 4, !alias.scope !8537, !noalias !8545, !noundef !10
  %_126.0.i1816 = load ptr, ptr %61, align 4, !nonnull !10
  %988 = add i32 %ring_cursor.sroa.0.1.i49317904, 1
  %_21.not.i1821 = icmp ult i32 %988, %_87.i
  %989 = select i1 %_21.not.i1821, i32 0, i32 %_87.i
  %start1.sroa.0.0.i1822 = sub nuw i32 %988, %989
  %_128.1.i1825 = load i32, ptr %694, align 4
  %_128.0.i1829 = load ptr, ptr %695, align 4, !nonnull !10
  %_130.1.i1830 = load i32, ptr %696, align 4
  %_130.0.i1834 = load ptr, ptr %697, align 4, !nonnull !10
  %_132.1.i1837 = load i32, ptr %698, align 4
  %_132.0.i1841 = load ptr, ptr %699, align 4, !nonnull !10
  %_43.i1854 = mul i32 %width.i1801, %start1.sroa.0.0.i1822
  br label %bb29.i1807, !dbg !8546

bb29.i1807:                                       ; preds = %bb29.i1807.lr.ph, %bb28.i1869
  %iter.sroa.0.0.idx.i180517884 = phi i32 [ 0, %bb29.i1807.lr.ph ], [ %iter.sroa.0.0.add.i1810, %bb28.i1869 ]
  %iter.sroa.4.0.i180417883 = phi i32 [ 0, %bb29.i1807.lr.ph ], [ %_102.0.i1811, %bb28.i1869 ]
  %iter.sroa.7.0.i180317882 = phi i32 [ %width.i1801, %bb29.i1807.lr.ph ], [ %990, %bb28.i1869 ]
  %iter.sroa.0.0.ptr.i180617885 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 %iter.sroa.0.0.idx.i180517884, !dbg !8548
  %990 = add i32 %iter.sroa.7.0.i180317882, -1, !dbg !8548
  %_109.i1808 = icmp eq i32 %iter.sroa.0.0.idx.i180517884, 32, !dbg !8549
  br i1 %_109.i1808, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1907, label %bb33.i1809, !dbg !8553

bb33.i1809:                                       ; preds = %bb29.i1807
  %iter.sroa.0.0.add.i1810 = add nuw nsw i32 %iter.sroa.0.0.idx.i180517884, 4, !dbg !8554
  %_102.0.i1811 = add nuw nsw i32 %iter.sroa.4.0.i180417883, 1, !dbg !8556
  %exitcond20501.not = icmp eq i32 %iter.sroa.4.0.i180417883, %_126.1.i1812, !dbg !8557
  br i1 %exitcond20501.not, label %panic.i1814, label %bb2.i1815, !dbg !8557

bb2.i1815:                                        ; preds = %bb33.i1809
  %991 = getelementptr inbounds nuw %LaneShape, ptr %_126.0.i1816, i32 %iter.sroa.4.0.i180417883, !dbg !8557
  %shape.i1817 = load i32, ptr %991, align 4, !dbg !8557, !noalias !8558, !noundef !10
  %992 = getelementptr inbounds nuw i8, ptr %991, i32 4, !dbg !8557
  %shape3.i1818 = load i32, ptr %992, align 4, !dbg !8557, !noalias !8558, !noundef !10
  %993 = add i32 %shape3.i1818, %ring_cursor.sroa.0.1.i49317904, !dbg !8559
  %_18.not.i1819 = icmp ult i32 %993, %_87.i, !dbg !8560
  %994 = select i1 %_18.not.i1819, i32 0, i32 %_87.i, !dbg !8560
  %spec.select.i1820 = sub nuw i32 %993, %994, !dbg !8560
  %_25.i1823 = mul i32 %spec.select.i1820, %width.i1801, !dbg !8561
  %_24.i1824 = add i32 %_25.i1823, %iter.sroa.4.0.i180417883, !dbg !8561
  %_28.i1826 = icmp ult i32 %_24.i1824, %_128.1.i1825, !dbg !8562
  br i1 %_28.i1826, label %bb9.i1828, label %panic5.i1827, !dbg !8562

panic.i1814:                                      ; preds = %bb33.i1809
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_126.1.i1812, i32 noundef %_126.1.i1812, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_12856b5f9033764dbf23e04eb77c5c46) #33, !dbg !8557, !noalias !8558
  unreachable, !dbg !8557

bb9.i1828:                                        ; preds = %bb2.i1815
  %995 = getelementptr inbounds nuw float, ptr %_128.0.i1829, i32 %_24.i1824, !dbg !8562
  %996 = load float, ptr %995, align 4, !dbg !8562, !noalias !8558, !noundef !10
  %exitcond20502.not = icmp eq i32 %iter.sroa.4.0.i180417883, %_130.1.i1830, !dbg !8563
  br i1 %exitcond20502.not, label %panic6.i1832, label %bb10.i1833, !dbg !8563

panic5.i1827:                                     ; preds = %bb2.i1815
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_24.i1824, i32 noundef %_128.1.i1825, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70ebf0cf1c90c6161926611ccf249a32) #33, !dbg !8562, !noalias !8558
  unreachable, !dbg !8562

bb10.i1833:                                       ; preds = %bb9.i1828
  %997 = getelementptr inbounds nuw i32, ptr %_130.0.i1834, i32 %iter.sroa.4.0.i180417883, !dbg !8563
  %_30.i1835 = load i32, ptr %997, align 4, !dbg !8563, !noalias !8558, !noundef !10
  %998 = icmp eq i32 %_30.i1835, 0, !dbg !8564
  br i1 %998, label %bb14.i1844, label %bb12.i1836, !dbg !8564

panic6.i1832:                                     ; preds = %bb9.i1828
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_130.1.i1830, i32 noundef %_130.1.i1830, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_023d591aaad7f5cf482ac80a9401eca1) #33, !dbg !8563, !noalias !8558
  unreachable, !dbg !8563

bb12.i1836:                                       ; preds = %bb10.i1833
  %_35.i1838 = icmp ult i32 %iter.sroa.4.0.i180417883, %_132.1.i1837, !dbg !8565
  br i1 %_35.i1838, label %bb13.i1840, label %panic7.i1839, !dbg !8565

bb14.i1844:                                       ; preds = %bb34.i1905, %bb13.i1840, %bb10.i1833
  %newest.sroa.0.0.i1845 = phi float [ %996, %bb10.i1833 ], [ %_33.i1842, %bb34.i1905 ], [ %996, %bb13.i1840 ], !dbg !8566
  %exitcond20503.not = icmp eq i32 %iter.sroa.4.0.i180417883, %_132.1.i1837, !dbg !8567
  br i1 %exitcond20503.not, label %panic8.i1848, label %bb15.i1849, !dbg !8567

bb13.i1840:                                       ; preds = %bb12.i1836
  %999 = getelementptr inbounds nuw float, ptr %_132.0.i1841, i32 %iter.sroa.4.0.i180417883, !dbg !8565
  %_33.i1842 = load float, ptr %999, align 4, !dbg !8565, !noalias !8558, !noundef !10
  %_116.i1843 = fcmp olt float %_33.i1842, %996, !dbg !8568
  br i1 %_116.i1843, label %bb34.i1905, label %bb14.i1844, !dbg !8568

panic7.i1839:                                     ; preds = %bb12.i1836
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %iter.sroa.4.0.i180417883, i32 noundef %_132.1.i1837, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a228cac3279aae3a34886f59f51fbe9e) #33, !dbg !8565, !noalias !8558
  unreachable, !dbg !8565

bb34.i1905:                                       ; preds = %bb13.i1840
  br label %bb14.i1844, !dbg !8570

bb15.i1849:                                       ; preds = %bb14.i1844
  %1000 = getelementptr inbounds nuw float, ptr %_132.0.i1841, i32 %iter.sroa.4.0.i180417883, !dbg !8567
  store float %newest.sroa.0.0.i1845, ptr %1000, align 4, !dbg !8567, !noalias !8558
  %_40.i1851 = add i32 %_30.i1835, 1, !dbg !8571
  %complete.i1852 = icmp eq i32 %_40.i1851, %shape.i1817, !dbg !8571
  br i1 %complete.i1852, label %bb19.i1874, label %bb17.i1853, !dbg !8572

panic8.i1848:                                     ; preds = %bb14.i1844
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_132.1.i1837, i32 noundef %_132.1.i1837, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1fa5d88c1a2cc292a2767c48606a38fe) #33, !dbg !8567, !noalias !8558
  unreachable, !dbg !8567

bb17.i1853:                                       ; preds = %bb15.i1849
  %_42.i1855 = add i32 %iter.sroa.4.0.i180417883, %_43.i1854, !dbg !8573
  %_45.i1857 = icmp ult i32 %_42.i1855, %_128.1.i1825, !dbg !8574
  br i1 %_45.i1857, label %bb27.i1867, label %panic9.i1858, !dbg !8574

panic9.i1858:                                     ; preds = %bb17.i1853
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_42.i1855, i32 noundef %_128.1.i1825, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70d40ae2302f3ea19fa0c4a65fe82786) #33, !dbg !8574, !noalias !8558
  unreachable, !dbg !8574

bb27.i1867:                                       ; preds = %bb17.i1853
  %1001 = getelementptr inbounds nuw float, ptr %_128.0.i1829, i32 %_42.i1855, !dbg !8574
  %_41.i1861 = load float, ptr %1001, align 4, !dbg !8574, !noalias !8558, !noundef !10
  %_117.i1862 = fcmp olt float %_41.i1861, %newest.sroa.0.0.i1845, !dbg !8575
  %newest.sroa.0.1.i1863 = select i1 %_117.i1862, float %_41.i1861, float %newest.sroa.0.0.i1845, !dbg !8575
  store float %newest.sroa.0.1.i1863, ptr %iter.sroa.0.0.ptr.i180617885, align 4, !dbg !8577, !alias.scope !8541, !noalias !8578
  br label %bb28.i1869, !dbg !8579

bb28.i1869:                                       ; preds = %bb22.i1902, %bb19.i1874, %bb27.i1867
  %storemerge15230 = phi i32 [ %_40.i1851, %bb27.i1867 ], [ 0, %bb19.i1874 ], [ 0, %bb22.i1902 ], !dbg !8580
  store i32 %storemerge15230, ptr %997, align 4, !dbg !8580, !noalias !8558
  %1002 = icmp eq i32 %990, 0, !dbg !8546
  br i1 %1002, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1907, label %bb29.i1807, !dbg !8546

bb19.i1874:                                       ; preds = %bb15.i1849
  store float %newest.sroa.0.0.i1845, ptr %iter.sroa.0.0.ptr.i180617885, align 4, !dbg !8577, !alias.scope !8541, !noalias !8578
  %_118.i188017878.not = icmp eq i32 %shape.i1817, 0, !dbg !8581
  br i1 %_118.i188017878.not, label %bb28.i1869, label %bb40.i1887.preheader, !dbg !8585

bb40.i1887.preheader:                             ; preds = %bb19.i1874
  %1003 = load float, ptr %995, align 4, !dbg !8586, !noalias !8558, !noundef !10
  br label %bb40.i1887, !dbg !8587

bb40.i1887:                                       ; preds = %bb40.i1887.preheader, %bb22.i1902
  %iter2.sroa.0.0.i187917881 = phi i32 [ %_119.i1888, %bb22.i1902 ], [ 0, %bb40.i1887.preheader ]
  %suffix.sroa.0.0.i187817880 = phi float [ %suffix.sroa.0.1.i1898, %bb22.i1902 ], [ %1003, %bb40.i1887.preheader ]
  %end.sroa.0.1.i187717879 = phi i32 [ %1006, %bb22.i1902 ], [ %spec.select.i1820, %bb40.i1887.preheader ]
  %_54.i1889 = mul i32 %end.sroa.0.1.i187717879, %width.i1801, !dbg !8588
  %_53.i1890 = add i32 %_54.i1889, %iter.sroa.4.0.i180417883, !dbg !8588
  %_57.i1892 = icmp ult i32 %_53.i1890, %_128.1.i1825, !dbg !8587
  br i1 %_57.i1892, label %bb22.i1902, label %panic13.i1893, !dbg !8587

panic13.i1893:                                    ; preds = %bb40.i1887
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_53.i1890, i32 noundef %_128.1.i1825, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a271bbb7fd83c3605ea58a06b7065fa4) #33, !dbg !8587, !noalias !8558
  unreachable, !dbg !8587

bb22.i1902:                                       ; preds = %bb40.i1887
  %_119.i1888 = add nuw i32 %iter2.sroa.0.0.i187917881, 1, !dbg !8589
  %1004 = getelementptr inbounds nuw float, ptr %_128.0.i1829, i32 %_53.i1890, !dbg !8587
  %_52.i1896 = load float, ptr %1004, align 4, !dbg !8587, !noalias !8558, !noundef !10
  %_121.i1897 = fcmp olt float %suffix.sroa.0.0.i187817880, %_52.i1896, !dbg !8592
  %suffix.sroa.0.1.i1898 = select i1 %_121.i1897, float %suffix.sroa.0.0.i187817880, float %_52.i1896, !dbg !8592
  store float %suffix.sroa.0.1.i1898, ptr %1004, align 4, !dbg !8594, !noalias !8558
  %1005 = icmp eq i32 %end.sroa.0.1.i187717879, 0, !dbg !8595
  %spec.store.select.i1904 = select i1 %1005, i32 %_87.i, i32 %end.sroa.0.1.i187717879, !dbg !8595
  %1006 = add i32 %spec.store.select.i1904, -1, !dbg !8596
  %exitcond20500.not = icmp eq i32 %_119.i1888, %shape.i1817, !dbg !8581
  br i1 %exitcond20500.not, label %bb28.i1869, label %bb40.i1887, !dbg !8585

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1907: ; preds = %bb29.i1807, %bb28.i1869, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5977
  %lanes.i5277.sroa.0.0.copyload = load <4 x float>, ptr %scratch.i, align 4, !dbg !8597, !alias.scope !8602, !noalias !8606
  %1007 = fmul <4 x float> %lanes.i5277.sroa.0.0.copyload, splat (float 1.638400e+04), !dbg !8610
  %1008 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %1007), !dbg !8614
  %1009 = fmul <4 x float> %1008, splat (float 0x3F10000000000000), !dbg !8618
  %1010 = icmp eq i32 %width.i31.i, 0, !dbg !8622
  %_163.1.i80.i.pre = load i32, ptr %700, align 4, !dbg !8624, !alias.scope !8484, !noalias !8485
  br i1 %1010, label %bb53.i75.i, label %bb36.i54.i.lr.ph, !dbg !8622

bb36.i54.i.lr.ph:                                 ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1907
  %_159.1.i59.i = load i32, ptr %62, align 4, !alias.scope !8484, !noalias !8485, !noundef !10
  %_159.0.i63.i = load ptr, ptr %61, align 4, !nonnull !10
  %_161.0.i73.i = load ptr, ptr %701, align 4, !nonnull !10
  %exitcond20506.not = icmp eq i32 %_159.1.i59.i, 0, !dbg !8625
  br i1 %exitcond20506.not, label %panic.i61.i, label %bb14.i62.i, !dbg !8625

bb34.i123.i:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5291
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i37.i, i32 noundef %_158.1.i36.i, i32 noundef %_158.1.i36.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_db7c42041d7aaaba4839906f37294547) #33, !dbg !8626, !noalias !8627
  unreachable, !dbg !8626

bb53.i75.i.loopexit:                              ; preds = %bb18.i72.i.7, %bb18.i72.i.6, %bb18.i72.i.5, %bb18.i72.i.4, %bb18.i72.i.3, %bb18.i72.i.2, %bb18.i72.i.1, %bb18.i72.i
  %lanes.i5270.sroa.0.0.copyload.pre = load <4 x float>, ptr %scratch.i, align 4, !dbg !8628, !alias.scope !8633, !noalias !8637
  br label %bb53.i75.i, !dbg !8641

bb53.i75.i:                                       ; preds = %bb53.i75.i.loopexit, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1907
  %lanes.i5270.sroa.0.0.copyload = phi <4 x float> [ %lanes.i5270.sroa.0.0.copyload.pre, %bb53.i75.i.loopexit ], [ %lanes.i5277.sroa.0.0.copyload, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1907 ], !dbg !8628
  %1011 = fadd <4 x float> %1009, %973, !dbg !8642
  %1012 = fsub <4 x float> %1011, %lanes.i5270.sroa.0.0.copyload, !dbg !8646
  %_123.i81.i = icmp ugt i32 %_22.i37.i, %_163.1.i80.i.pre, !dbg !8650
  br i1 %_123.i81.i, label %bb41.i122.i, label %bb42.i82.i, !dbg !8650, !prof !902

bb14.i62.i:                                       ; preds = %bb36.i54.i.lr.ph
  %1013 = getelementptr inbounds nuw i8, ptr %_159.0.i63.i, i32 8, !dbg !8625
  %_42.i64.i = load i32, ptr %1013, align 4, !dbg !8625, !noalias !8653, !noundef !10
  %1014 = add i32 %_42.i64.i, %ring_cursor.sroa.0.1.i49317904, !dbg !8654
  %_45.not.i65.i = icmp ult i32 %1014, %_87.i, !dbg !8655
  %1015 = select i1 %_45.not.i65.i, i32 0, i32 %_87.i, !dbg !8655
  %spec.select.i66.i = sub nuw i32 %1014, %1015, !dbg !8655
  %_49.i67.i = mul i32 %spec.select.i66.i, %width.i31.i, !dbg !8656
  %_51.i70.i = icmp ult i32 %_49.i67.i, %_163.1.i80.i.pre, !dbg !8657
  br i1 %_51.i70.i, label %bb18.i72.i, label %panic1.i71.i, !dbg !8657

panic.i61.i:                                      ; preds = %bb36.i54.i.7, %bb36.i54.i.6, %bb36.i54.i.5, %bb36.i54.i.4, %bb36.i54.i.3, %bb36.i54.i.2, %bb36.i54.i.1, %bb36.i54.i.lr.ph
  %_159.1.i59.i.lcssa.ph = phi i32 [ 7, %bb36.i54.i.7 ], [ 6, %bb36.i54.i.6 ], [ 5, %bb36.i54.i.5 ], [ 4, %bb36.i54.i.4 ], [ 3, %bb36.i54.i.3 ], [ 2, %bb36.i54.i.2 ], [ 1, %bb36.i54.i.1 ], [ 0, %bb36.i54.i.lr.ph ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_159.1.i59.i.lcssa.ph, i32 noundef %_159.1.i59.i.lcssa.ph, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_33746731a929bad63709ddedbca82f5f) #33, !dbg !8625, !noalias !8653
  unreachable, !dbg !8625

bb18.i72.i:                                       ; preds = %bb14.i62.i
  %1016 = getelementptr inbounds nuw float, ptr %_161.0.i73.i, i32 %_49.i67.i, !dbg !8657
  %_47.i74.i = load float, ptr %1016, align 4, !dbg !8657, !noalias !8653, !noundef !10
  store float %_47.i74.i, ptr %scratch.i, align 4, !dbg !8658, !alias.scope !8481, !noalias !8659
  %1017 = icmp eq i32 %width.i31.i, 1, !dbg !8622
  br i1 %1017, label %bb53.i75.i.loopexit, label %bb36.i54.i.1, !dbg !8622

bb36.i54.i.1:                                     ; preds = %bb18.i72.i
  %exitcond20506.1.not = icmp eq i32 %_159.1.i59.i, 1, !dbg !8625
  br i1 %exitcond20506.1.not, label %panic.i61.i, label %bb14.i62.i.1, !dbg !8625

bb14.i62.i.1:                                     ; preds = %bb36.i54.i.1
  %1018 = getelementptr inbounds nuw i8, ptr %_159.0.i63.i, i32 20, !dbg !8625
  %_42.i64.i.1 = load i32, ptr %1018, align 4, !dbg !8625, !noalias !8653, !noundef !10
  %1019 = add i32 %_42.i64.i.1, %ring_cursor.sroa.0.1.i49317904, !dbg !8654
  %_45.not.i65.i.1 = icmp ult i32 %1019, %_87.i, !dbg !8655
  %1020 = select i1 %_45.not.i65.i.1, i32 0, i32 %_87.i, !dbg !8655
  %spec.select.i66.i.1 = sub nuw i32 %1019, %1020, !dbg !8655
  %_49.i67.i.1 = mul i32 %spec.select.i66.i.1, %width.i31.i, !dbg !8656
  %_48.i68.i.1 = add i32 %_49.i67.i.1, 1, !dbg !8656
  %_51.i70.i.1 = icmp ult i32 %_48.i68.i.1, %_163.1.i80.i.pre, !dbg !8657
  br i1 %_51.i70.i.1, label %bb18.i72.i.1, label %panic1.i71.i, !dbg !8657

bb18.i72.i.1:                                     ; preds = %bb14.i62.i.1
  %1021 = getelementptr inbounds nuw float, ptr %_161.0.i73.i, i32 %_48.i68.i.1, !dbg !8657
  %_47.i74.i.1 = load float, ptr %1021, align 4, !dbg !8657, !noalias !8653, !noundef !10
  store float %_47.i74.i.1, ptr %iter.sroa.0.0.ptr.i53.i17889.1, align 4, !dbg !8658, !alias.scope !8481, !noalias !8659
  %1022 = icmp eq i32 %width.i31.i, 2, !dbg !8622
  br i1 %1022, label %bb53.i75.i.loopexit, label %bb36.i54.i.2, !dbg !8622

bb36.i54.i.2:                                     ; preds = %bb18.i72.i.1
  %exitcond20506.2.not = icmp eq i32 %_159.1.i59.i, 2, !dbg !8625
  br i1 %exitcond20506.2.not, label %panic.i61.i, label %bb14.i62.i.2, !dbg !8625

bb14.i62.i.2:                                     ; preds = %bb36.i54.i.2
  %1023 = getelementptr inbounds nuw i8, ptr %_159.0.i63.i, i32 32, !dbg !8625
  %_42.i64.i.2 = load i32, ptr %1023, align 4, !dbg !8625, !noalias !8653, !noundef !10
  %1024 = add i32 %_42.i64.i.2, %ring_cursor.sroa.0.1.i49317904, !dbg !8654
  %_45.not.i65.i.2 = icmp ult i32 %1024, %_87.i, !dbg !8655
  %1025 = select i1 %_45.not.i65.i.2, i32 0, i32 %_87.i, !dbg !8655
  %spec.select.i66.i.2 = sub nuw i32 %1024, %1025, !dbg !8655
  %_49.i67.i.2 = mul i32 %spec.select.i66.i.2, %width.i31.i, !dbg !8656
  %_48.i68.i.2 = add i32 %_49.i67.i.2, 2, !dbg !8656
  %_51.i70.i.2 = icmp ult i32 %_48.i68.i.2, %_163.1.i80.i.pre, !dbg !8657
  br i1 %_51.i70.i.2, label %bb18.i72.i.2, label %panic1.i71.i, !dbg !8657

bb18.i72.i.2:                                     ; preds = %bb14.i62.i.2
  %1026 = getelementptr inbounds nuw float, ptr %_161.0.i73.i, i32 %_48.i68.i.2, !dbg !8657
  %_47.i74.i.2 = load float, ptr %1026, align 4, !dbg !8657, !noalias !8653, !noundef !10
  store float %_47.i74.i.2, ptr %iter.sroa.0.0.ptr.i53.i17889.2, align 4, !dbg !8658, !alias.scope !8481, !noalias !8659
  %1027 = icmp eq i32 %width.i31.i, 3, !dbg !8622
  br i1 %1027, label %bb53.i75.i.loopexit, label %bb36.i54.i.3, !dbg !8622

bb36.i54.i.3:                                     ; preds = %bb18.i72.i.2
  %exitcond20506.3.not = icmp eq i32 %_159.1.i59.i, 3, !dbg !8625
  br i1 %exitcond20506.3.not, label %panic.i61.i, label %bb14.i62.i.3, !dbg !8625

bb14.i62.i.3:                                     ; preds = %bb36.i54.i.3
  %1028 = getelementptr inbounds nuw i8, ptr %_159.0.i63.i, i32 44, !dbg !8625
  %_42.i64.i.3 = load i32, ptr %1028, align 4, !dbg !8625, !noalias !8653, !noundef !10
  %1029 = add i32 %_42.i64.i.3, %ring_cursor.sroa.0.1.i49317904, !dbg !8654
  %_45.not.i65.i.3 = icmp ult i32 %1029, %_87.i, !dbg !8655
  %1030 = select i1 %_45.not.i65.i.3, i32 0, i32 %_87.i, !dbg !8655
  %spec.select.i66.i.3 = sub nuw i32 %1029, %1030, !dbg !8655
  %_49.i67.i.3 = mul i32 %spec.select.i66.i.3, %width.i31.i, !dbg !8656
  %_48.i68.i.3 = add i32 %_49.i67.i.3, 3, !dbg !8656
  %_51.i70.i.3 = icmp ult i32 %_48.i68.i.3, %_163.1.i80.i.pre, !dbg !8657
  br i1 %_51.i70.i.3, label %bb18.i72.i.3, label %panic1.i71.i, !dbg !8657

bb18.i72.i.3:                                     ; preds = %bb14.i62.i.3
  %1031 = getelementptr inbounds nuw float, ptr %_161.0.i73.i, i32 %_48.i68.i.3, !dbg !8657
  %_47.i74.i.3 = load float, ptr %1031, align 4, !dbg !8657, !noalias !8653, !noundef !10
  store float %_47.i74.i.3, ptr %iter.sroa.0.0.ptr.i53.i17889.3, align 4, !dbg !8658, !alias.scope !8481, !noalias !8659
  %1032 = icmp eq i32 %width.i31.i, 4, !dbg !8622
  br i1 %1032, label %bb53.i75.i.loopexit, label %bb36.i54.i.4, !dbg !8622

bb36.i54.i.4:                                     ; preds = %bb18.i72.i.3
  %exitcond20506.4.not = icmp eq i32 %_159.1.i59.i, 4, !dbg !8625
  br i1 %exitcond20506.4.not, label %panic.i61.i, label %bb14.i62.i.4, !dbg !8625

bb14.i62.i.4:                                     ; preds = %bb36.i54.i.4
  %1033 = getelementptr inbounds nuw i8, ptr %_159.0.i63.i, i32 56, !dbg !8625
  %_42.i64.i.4 = load i32, ptr %1033, align 4, !dbg !8625, !noalias !8653, !noundef !10
  %1034 = add i32 %_42.i64.i.4, %ring_cursor.sroa.0.1.i49317904, !dbg !8654
  %_45.not.i65.i.4 = icmp ult i32 %1034, %_87.i, !dbg !8655
  %1035 = select i1 %_45.not.i65.i.4, i32 0, i32 %_87.i, !dbg !8655
  %spec.select.i66.i.4 = sub nuw i32 %1034, %1035, !dbg !8655
  %_49.i67.i.4 = mul i32 %spec.select.i66.i.4, %width.i31.i, !dbg !8656
  %_48.i68.i.4 = add i32 %_49.i67.i.4, 4, !dbg !8656
  %_51.i70.i.4 = icmp ult i32 %_48.i68.i.4, %_163.1.i80.i.pre, !dbg !8657
  br i1 %_51.i70.i.4, label %bb18.i72.i.4, label %panic1.i71.i, !dbg !8657

bb18.i72.i.4:                                     ; preds = %bb14.i62.i.4
  %1036 = getelementptr inbounds nuw float, ptr %_161.0.i73.i, i32 %_48.i68.i.4, !dbg !8657
  %_47.i74.i.4 = load float, ptr %1036, align 4, !dbg !8657, !noalias !8653, !noundef !10
  store float %_47.i74.i.4, ptr %iter.sroa.0.0.ptr.i53.i17889.4, align 4, !dbg !8658, !alias.scope !8481, !noalias !8659
  %1037 = icmp eq i32 %width.i31.i, 5, !dbg !8622
  br i1 %1037, label %bb53.i75.i.loopexit, label %bb36.i54.i.5, !dbg !8622

bb36.i54.i.5:                                     ; preds = %bb18.i72.i.4
  %exitcond20506.5.not = icmp eq i32 %_159.1.i59.i, 5, !dbg !8625
  br i1 %exitcond20506.5.not, label %panic.i61.i, label %bb14.i62.i.5, !dbg !8625

bb14.i62.i.5:                                     ; preds = %bb36.i54.i.5
  %1038 = getelementptr inbounds nuw i8, ptr %_159.0.i63.i, i32 68, !dbg !8625
  %_42.i64.i.5 = load i32, ptr %1038, align 4, !dbg !8625, !noalias !8653, !noundef !10
  %1039 = add i32 %_42.i64.i.5, %ring_cursor.sroa.0.1.i49317904, !dbg !8654
  %_45.not.i65.i.5 = icmp ult i32 %1039, %_87.i, !dbg !8655
  %1040 = select i1 %_45.not.i65.i.5, i32 0, i32 %_87.i, !dbg !8655
  %spec.select.i66.i.5 = sub nuw i32 %1039, %1040, !dbg !8655
  %_49.i67.i.5 = mul i32 %spec.select.i66.i.5, %width.i31.i, !dbg !8656
  %_48.i68.i.5 = add i32 %_49.i67.i.5, 5, !dbg !8656
  %_51.i70.i.5 = icmp ult i32 %_48.i68.i.5, %_163.1.i80.i.pre, !dbg !8657
  br i1 %_51.i70.i.5, label %bb18.i72.i.5, label %panic1.i71.i, !dbg !8657

bb18.i72.i.5:                                     ; preds = %bb14.i62.i.5
  %1041 = getelementptr inbounds nuw float, ptr %_161.0.i73.i, i32 %_48.i68.i.5, !dbg !8657
  %_47.i74.i.5 = load float, ptr %1041, align 4, !dbg !8657, !noalias !8653, !noundef !10
  store float %_47.i74.i.5, ptr %iter.sroa.0.0.ptr.i53.i17889.5, align 4, !dbg !8658, !alias.scope !8481, !noalias !8659
  %1042 = icmp eq i32 %width.i31.i, 6, !dbg !8622
  br i1 %1042, label %bb53.i75.i.loopexit, label %bb36.i54.i.6, !dbg !8622

bb36.i54.i.6:                                     ; preds = %bb18.i72.i.5
  %exitcond20506.6.not = icmp eq i32 %_159.1.i59.i, 6, !dbg !8625
  br i1 %exitcond20506.6.not, label %panic.i61.i, label %bb14.i62.i.6, !dbg !8625

bb14.i62.i.6:                                     ; preds = %bb36.i54.i.6
  %1043 = getelementptr inbounds nuw i8, ptr %_159.0.i63.i, i32 80, !dbg !8625
  %_42.i64.i.6 = load i32, ptr %1043, align 4, !dbg !8625, !noalias !8653, !noundef !10
  %1044 = add i32 %_42.i64.i.6, %ring_cursor.sroa.0.1.i49317904, !dbg !8654
  %_45.not.i65.i.6 = icmp ult i32 %1044, %_87.i, !dbg !8655
  %1045 = select i1 %_45.not.i65.i.6, i32 0, i32 %_87.i, !dbg !8655
  %spec.select.i66.i.6 = sub nuw i32 %1044, %1045, !dbg !8655
  %_49.i67.i.6 = mul i32 %spec.select.i66.i.6, %width.i31.i, !dbg !8656
  %_48.i68.i.6 = add i32 %_49.i67.i.6, 6, !dbg !8656
  %_51.i70.i.6 = icmp ult i32 %_48.i68.i.6, %_163.1.i80.i.pre, !dbg !8657
  br i1 %_51.i70.i.6, label %bb18.i72.i.6, label %panic1.i71.i, !dbg !8657

bb18.i72.i.6:                                     ; preds = %bb14.i62.i.6
  %1046 = getelementptr inbounds nuw float, ptr %_161.0.i73.i, i32 %_48.i68.i.6, !dbg !8657
  %_47.i74.i.6 = load float, ptr %1046, align 4, !dbg !8657, !noalias !8653, !noundef !10
  store float %_47.i74.i.6, ptr %iter.sroa.0.0.ptr.i53.i17889.6, align 4, !dbg !8658, !alias.scope !8481, !noalias !8659
  %1047 = icmp eq i32 %width.i31.i, 7, !dbg !8622
  br i1 %1047, label %bb53.i75.i.loopexit, label %bb36.i54.i.7, !dbg !8622

bb36.i54.i.7:                                     ; preds = %bb18.i72.i.6
  %exitcond20506.7.not = icmp eq i32 %_159.1.i59.i, 7, !dbg !8625
  br i1 %exitcond20506.7.not, label %panic.i61.i, label %bb14.i62.i.7, !dbg !8625

bb14.i62.i.7:                                     ; preds = %bb36.i54.i.7
  %1048 = getelementptr inbounds nuw i8, ptr %_159.0.i63.i, i32 92, !dbg !8625
  %_42.i64.i.7 = load i32, ptr %1048, align 4, !dbg !8625, !noalias !8653, !noundef !10
  %1049 = add i32 %_42.i64.i.7, %ring_cursor.sroa.0.1.i49317904, !dbg !8654
  %_45.not.i65.i.7 = icmp ult i32 %1049, %_87.i, !dbg !8655
  %1050 = select i1 %_45.not.i65.i.7, i32 0, i32 %_87.i, !dbg !8655
  %spec.select.i66.i.7 = sub nuw i32 %1049, %1050, !dbg !8655
  %_49.i67.i.7 = mul i32 %spec.select.i66.i.7, %width.i31.i, !dbg !8656
  %_48.i68.i.7 = add i32 %_49.i67.i.7, 7, !dbg !8656
  %_51.i70.i.7 = icmp ult i32 %_48.i68.i.7, %_163.1.i80.i.pre, !dbg !8657
  br i1 %_51.i70.i.7, label %bb18.i72.i.7, label %panic1.i71.i, !dbg !8657

bb18.i72.i.7:                                     ; preds = %bb14.i62.i.7
  %1051 = getelementptr inbounds nuw float, ptr %_161.0.i73.i, i32 %_48.i68.i.7, !dbg !8657
  %_47.i74.i.7 = load float, ptr %1051, align 4, !dbg !8657, !noalias !8653, !noundef !10
  store float %_47.i74.i.7, ptr %iter.sroa.0.0.ptr.i53.i17889.7, align 4, !dbg !8658, !alias.scope !8481, !noalias !8659
  br label %bb53.i75.i.loopexit, !dbg !8622

panic1.i71.i:                                     ; preds = %bb14.i62.i.7, %bb14.i62.i.6, %bb14.i62.i.5, %bb14.i62.i.4, %bb14.i62.i.3, %bb14.i62.i.2, %bb14.i62.i.1, %bb14.i62.i
  %_48.i68.i.lcssa.ph = phi i32 [ %_48.i68.i.7, %bb14.i62.i.7 ], [ %_48.i68.i.6, %bb14.i62.i.6 ], [ %_48.i68.i.5, %bb14.i62.i.5 ], [ %_48.i68.i.4, %bb14.i62.i.4 ], [ %_48.i68.i.3, %bb14.i62.i.3 ], [ %_48.i68.i.2, %bb14.i62.i.2 ], [ %_48.i68.i.1, %bb14.i62.i.1 ], [ %_49.i67.i, %bb14.i62.i ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_48.i68.i.lcssa.ph, i32 noundef %_163.1.i80.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_0e76a26fbb751dfb8cf8a069b5b0d39a) #33, !dbg !8657, !noalias !8653
  unreachable, !dbg !8657

bb42.i82.i:                                       ; preds = %bb53.i75.i
  %_126.i84.i = sub nuw i32 %_163.1.i80.i.pre, %_22.i37.i, !dbg !8660
  %_8.i5969 = icmp samesign ugt i32 %_126.i84.i, 3, !dbg !8661
  br i1 %_8.i5969, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5972, label %bb2.i5970, !dbg !8661, !prof !1153

bb2.i5970:                                        ; preds = %bb42.i82.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_126.i84.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !8666, !noalias !8667
  unreachable, !dbg !8666

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5972: ; preds = %bb42.i82.i
  %_163.0.i83.i = load ptr, ptr %701, align 4, !dbg !8624, !alias.scope !8484, !noalias !8485, !nonnull !10, !noundef !10
  %_130.i85.i = getelementptr inbounds nuw float, ptr %_163.0.i83.i, i32 %_22.i37.i, !dbg !8671
  store <4 x float> %1009, ptr %_130.i85.i, align 4, !dbg !8673, !alias.scope !8677, !noalias !8681
  %_66.i90.i15237 = load <4 x float>, ptr %704, align 16, !dbg !8683
  %1052 = fdiv <4 x float> %1012, %_62.i87.i15236, !dbg !8684
  %1053 = fsub <4 x float> splat (float 1.000000e+00), %1052, !dbg !8688
  %1054 = fsub <4 x float> %1053, %_66.i90.i15237, !dbg !8692
  %1055 = fmul <4 x float> %_9.i30.i15226.pre, %1054, !dbg !8696
  %1056 = fadd <4 x float> %_66.i90.i15237, %1055, !dbg !8700
  %1057 = fcmp olt <4 x float> %1056, %1053, !dbg !8703
  %1058 = select <4 x i1> %1057, <4 x float> %1053, <4 x float> %1056, !dbg !8707
  %1059 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1058), !dbg !8708
  %1060 = fcmp uge <4 x float> %1059, splat (float 0x3BC79CA100000000), !dbg !8713
  %1061 = bitcast <4 x float> %1058 to <4 x i32>, !dbg !8718
  %1062 = select <4 x i1> %1060, <4 x i32> %1061, <4 x i32> zeroinitializer, !dbg !8718
  store <4 x i32> %1062, ptr %704, align 16, !dbg !8721
  %1063 = bitcast <4 x i32> %1062 to <4 x float>, !dbg !8722
  %1064 = fsub <4 x float> splat (float 1.000000e+00), %1063, !dbg !8726
  %_164.1.i100.i = load i32, ptr %705, align 4, !dbg !8727, !alias.scope !8484, !noalias !8485, !noundef !10
  %_74.i101.i = mul i32 %width.i31.i, %main_cursor.sroa.0.1.i49417905, !dbg !8728
  %_134.i102.i = icmp ugt i32 %_74.i101.i, %_164.1.i100.i, !dbg !8729
  br i1 %_134.i102.i, label %bb47.i121.i, label %bb48.i103.i, !dbg !8729, !prof !902

bb41.i122.i:                                      ; preds = %bb53.i75.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i37.i, i32 noundef %_163.1.i80.i.pre, i32 noundef %_163.1.i80.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c2286ff41a33b8c11aa270fe215441de) #33, !dbg !8732, !noalias !8653
  unreachable, !dbg !8732

bb48.i103.i:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5972
  %_137.i105.i = sub nuw i32 %_164.1.i100.i, %_74.i101.i, !dbg !8733
  %_8.i5264 = icmp samesign ugt i32 %_137.i105.i, 3, !dbg !8734
  br i1 %_8.i5264, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5962, label %bb2.i5265, !dbg !8734, !prof !1153

bb2.i5265:                                        ; preds = %bb48.i103.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_137.i105.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !8739, !noalias !8740
  unreachable, !dbg !8739

bb47.i121.i:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5972
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_74.i101.i, i32 noundef %_164.1.i100.i, i32 noundef %_164.1.i100.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a3a21efd18b403ec25db545d522c479) #33, !dbg !8744, !noalias !8653
  unreachable, !dbg !8744

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5962: ; preds = %bb48.i103.i
  %_164.0.i104.i = load ptr, ptr %706, align 4, !dbg !8727, !alias.scope !8484, !noalias !8485, !nonnull !10, !noundef !10
  %_141.i106.i = getelementptr inbounds nuw float, ptr %_164.0.i104.i, i32 %_74.i101.i, !dbg !8745
  %lanes.i5261.sroa.0.0.copyload = load <4 x i32>, ptr %_141.i106.i, align 4, !dbg !8747, !alias.scope !8751, !noalias !8755
  store <4 x i32> %lanes.i5284.sroa.0.0.copyload, ptr %_141.i106.i, align 4, !dbg !8757, !alias.scope !8762, !noalias !8766
  %1065 = bitcast <4 x i32> %lanes.i5261.sroa.0.0.copyload to <4 x float>, !dbg !8770
  %1066 = fmul <4 x float> %1064, %1065, !dbg !8774
  %1067 = bitcast <4 x i32> %lanes.i5261.sroa.0.0.copyload to <16 x i8>, !dbg !8775
  %1068 = bitcast <4 x float> %1066 to <16 x i8>, !dbg !8779
  %_4.i6532 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1067, <16 x i8> %1068, <16 x i8> %707), !dbg !8780
  store <16 x i8> %_4.i6532, ptr %_139.i510, align 4, !dbg !8781, !alias.scope !8786, !noalias !8790
  %_140.i = icmp ugt i32 %base.i496, %right_io.1, !dbg !8794
  br i1 %_140.i, label %bb48.i, label %bb49.i511, !dbg !8794, !prof !902

bb46.i:                                           ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5309
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i496, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_f462f4e49ffb40af04504eaf795aa606) #33, !dbg !8798, !noalias !8799
  unreachable, !dbg !8798

bb49.i511:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5962
  %_143.i = sub nuw nsw i32 %right_io.1, %base.i496, !dbg !8800
  %_147.i = getelementptr inbounds nuw float, ptr %right_io.0, i32 %base.i496, !dbg !8801
  %_8.i5255 = icmp samesign ugt i32 %_143.i, 3, !dbg !8806
  br i1 %_8.i5255, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5259, label %bb2.i5256, !dbg !8806, !prof !1153

bb2.i5256:                                        ; preds = %bb49.i511
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_143.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !8811, !noalias !8812
  unreachable, !dbg !8811

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5259: ; preds = %bb49.i511
  %lanes.i5252.sroa.0.0.copyload = load <4 x i32>, ptr %_147.i, align 4, !dbg !8816, !alias.scope !8820, !noalias !8824
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8826), !dbg !8829
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8830), !dbg !8829
  %width.i.i = load i32, ptr %708, align 4, !dbg !8832, !alias.scope !8833, !noalias !8834, !noundef !10
  %1069 = bitcast <16 x i8> %_4.i6523 to <4 x float>, !dbg !8842
  %1070 = fcmp olt <4 x float> %_8.i.i50115227.pre, %1069, !dbg !8847
  %1071 = sext <4 x i1> %1070 to <4 x i32>, !dbg !8847
  %1072 = fdiv <4 x float> %_8.i.i50115227.pre, %1069, !dbg !8848
  %1073 = bitcast <4 x float> %1072 to <16 x i8>, !dbg !8852
  %1074 = bitcast <4 x i32> %1071 to <16 x i8>, !dbg !8856
  %_4.i6535 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1073, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %1074), !dbg !8857
  %_158.1.i.i = load i32, ptr %709, align 4, !dbg !8858, !alias.scope !8833, !noalias !8834, !noundef !10
  %_22.i.i517 = mul i32 %width.i.i, %ring_cursor.sroa.0.1.i49317904, !dbg !8859
  %_90.i.i = icmp ugt i32 %_22.i.i517, %_158.1.i.i, !dbg !8860
  br i1 %_90.i.i, label %bb34.i.i, label %bb35.i.i, !dbg !8860, !prof !902

bb35.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5259
  %_93.i.i = sub nuw i32 %_158.1.i.i, %_22.i.i517, !dbg !8863
  %_8.i5954 = icmp samesign ugt i32 %_93.i.i, 3, !dbg !8864
  br i1 %_8.i5954, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5957, label %bb2.i5955, !dbg !8864, !prof !1153

bb2.i5955:                                        ; preds = %bb35.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_93.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !8869, !noalias !8870
  unreachable, !dbg !8869

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5957: ; preds = %bb35.i.i
  %_158.0.i.i = load ptr, ptr %710, align 4, !dbg !8858, !alias.scope !8833, !noalias !8834, !nonnull !10, !noundef !10
  %_97.i.i = getelementptr inbounds nuw float, ptr %_158.0.i.i, i32 %_22.i.i517, !dbg !8874
  store <16 x i8> %_4.i6535, ptr %_97.i.i, align 4, !dbg !8876, !alias.scope !8880, !noalias !8884
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8886), !dbg !8889
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8890), !dbg !8889
  %width.i1694 = load i32, ptr %708, align 4, !dbg !8892, !alias.scope !8886, !noalias !8894, !noundef !10
  %1075 = icmp eq i32 %width.i1694, 0, !dbg !8895
  br i1 %1075, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1800, label %bb29.i1700.lr.ph, !dbg !8895

bb29.i1700.lr.ph:                                 ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5957
  %_126.1.i1705 = load i32, ptr %711, align 4, !alias.scope !8886, !noalias !8894, !noundef !10
  %_126.0.i1709 = load ptr, ptr %712, align 4, !nonnull !10
  %1076 = add i32 %ring_cursor.sroa.0.1.i49317904, 1
  %_21.not.i1714 = icmp ult i32 %1076, %_87.i
  %1077 = select i1 %_21.not.i1714, i32 0, i32 %_87.i
  %start1.sroa.0.0.i1715 = sub nuw i32 %1076, %1077
  %_128.1.i1718 = load i32, ptr %709, align 4
  %_128.0.i1722 = load ptr, ptr %710, align 4, !nonnull !10
  %_130.1.i1723 = load i32, ptr %713, align 4
  %_130.0.i1727 = load ptr, ptr %714, align 4, !nonnull !10
  %_132.1.i1730 = load i32, ptr %715, align 4
  %_132.0.i1734 = load ptr, ptr %716, align 4, !nonnull !10
  %_43.i1747 = mul i32 %width.i1694, %start1.sroa.0.0.i1715
  br label %bb29.i1700, !dbg !8895

bb29.i1700:                                       ; preds = %bb29.i1700.lr.ph, %bb28.i1762
  %iter.sroa.0.0.idx.i169817896 = phi i32 [ 0, %bb29.i1700.lr.ph ], [ %iter.sroa.0.0.add.i1703, %bb28.i1762 ]
  %iter.sroa.4.0.i169717895 = phi i32 [ 0, %bb29.i1700.lr.ph ], [ %_102.0.i1704, %bb28.i1762 ]
  %iter.sroa.7.0.i169617894 = phi i32 [ %width.i1694, %bb29.i1700.lr.ph ], [ %1078, %bb28.i1762 ]
  %iter.sroa.0.0.ptr.i169917897 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 %iter.sroa.0.0.idx.i169817896, !dbg !8897
  %1078 = add i32 %iter.sroa.7.0.i169617894, -1, !dbg !8897
  %_109.i1701 = icmp eq i32 %iter.sroa.0.0.idx.i169817896, 32, !dbg !8898
  br i1 %_109.i1701, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1800.loopexit, label %bb33.i1702, !dbg !8902

bb33.i1702:                                       ; preds = %bb29.i1700
  %iter.sroa.0.0.add.i1703 = add nuw nsw i32 %iter.sroa.0.0.idx.i169817896, 4, !dbg !8903
  %_102.0.i1704 = add nuw nsw i32 %iter.sroa.4.0.i169717895, 1, !dbg !8905
  %exitcond20510.not = icmp eq i32 %iter.sroa.4.0.i169717895, %_126.1.i1705, !dbg !8906
  br i1 %exitcond20510.not, label %panic.i1707, label %bb2.i1708, !dbg !8906

bb2.i1708:                                        ; preds = %bb33.i1702
  %1079 = getelementptr inbounds nuw %LaneShape, ptr %_126.0.i1709, i32 %iter.sroa.4.0.i169717895, !dbg !8906
  %shape.i1710 = load i32, ptr %1079, align 4, !dbg !8906, !noalias !8907, !noundef !10
  %1080 = getelementptr inbounds nuw i8, ptr %1079, i32 4, !dbg !8906
  %shape3.i1711 = load i32, ptr %1080, align 4, !dbg !8906, !noalias !8907, !noundef !10
  %1081 = add i32 %shape3.i1711, %ring_cursor.sroa.0.1.i49317904, !dbg !8908
  %_18.not.i1712 = icmp ult i32 %1081, %_87.i, !dbg !8909
  %1082 = select i1 %_18.not.i1712, i32 0, i32 %_87.i, !dbg !8909
  %spec.select.i1713 = sub nuw i32 %1081, %1082, !dbg !8909
  %_25.i1716 = mul i32 %spec.select.i1713, %width.i1694, !dbg !8910
  %_24.i1717 = add i32 %_25.i1716, %iter.sroa.4.0.i169717895, !dbg !8910
  %_28.i1719 = icmp ult i32 %_24.i1717, %_128.1.i1718, !dbg !8911
  br i1 %_28.i1719, label %bb9.i1721, label %panic5.i1720, !dbg !8911

panic.i1707:                                      ; preds = %bb33.i1702
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_126.1.i1705, i32 noundef %_126.1.i1705, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_12856b5f9033764dbf23e04eb77c5c46) #33, !dbg !8906, !noalias !8907
  unreachable, !dbg !8906

bb9.i1721:                                        ; preds = %bb2.i1708
  %1083 = getelementptr inbounds nuw float, ptr %_128.0.i1722, i32 %_24.i1717, !dbg !8911
  %1084 = load float, ptr %1083, align 4, !dbg !8911, !noalias !8907, !noundef !10
  %exitcond20511.not = icmp eq i32 %iter.sroa.4.0.i169717895, %_130.1.i1723, !dbg !8912
  br i1 %exitcond20511.not, label %panic6.i1725, label %bb10.i1726, !dbg !8912

panic5.i1720:                                     ; preds = %bb2.i1708
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_24.i1717, i32 noundef %_128.1.i1718, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70ebf0cf1c90c6161926611ccf249a32) #33, !dbg !8911, !noalias !8907
  unreachable, !dbg !8911

bb10.i1726:                                       ; preds = %bb9.i1721
  %1085 = getelementptr inbounds nuw i32, ptr %_130.0.i1727, i32 %iter.sroa.4.0.i169717895, !dbg !8912
  %_30.i1728 = load i32, ptr %1085, align 4, !dbg !8912, !noalias !8907, !noundef !10
  %1086 = icmp eq i32 %_30.i1728, 0, !dbg !8913
  br i1 %1086, label %bb14.i1737, label %bb12.i1729, !dbg !8913

panic6.i1725:                                     ; preds = %bb9.i1721
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_130.1.i1723, i32 noundef %_130.1.i1723, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_023d591aaad7f5cf482ac80a9401eca1) #33, !dbg !8912, !noalias !8907
  unreachable, !dbg !8912

bb12.i1729:                                       ; preds = %bb10.i1726
  %_35.i1731 = icmp ult i32 %iter.sroa.4.0.i169717895, %_132.1.i1730, !dbg !8914
  br i1 %_35.i1731, label %bb13.i1733, label %panic7.i1732, !dbg !8914

bb14.i1737:                                       ; preds = %bb34.i1798, %bb13.i1733, %bb10.i1726
  %newest.sroa.0.0.i1738 = phi float [ %1084, %bb10.i1726 ], [ %_33.i1735, %bb34.i1798 ], [ %1084, %bb13.i1733 ], !dbg !8915
  %exitcond20512.not = icmp eq i32 %iter.sroa.4.0.i169717895, %_132.1.i1730, !dbg !8916
  br i1 %exitcond20512.not, label %panic8.i1741, label %bb15.i1742, !dbg !8916

bb13.i1733:                                       ; preds = %bb12.i1729
  %1087 = getelementptr inbounds nuw float, ptr %_132.0.i1734, i32 %iter.sroa.4.0.i169717895, !dbg !8914
  %_33.i1735 = load float, ptr %1087, align 4, !dbg !8914, !noalias !8907, !noundef !10
  %_116.i1736 = fcmp olt float %_33.i1735, %1084, !dbg !8917
  br i1 %_116.i1736, label %bb34.i1798, label %bb14.i1737, !dbg !8917

panic7.i1732:                                     ; preds = %bb12.i1729
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %iter.sroa.4.0.i169717895, i32 noundef %_132.1.i1730, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a228cac3279aae3a34886f59f51fbe9e) #33, !dbg !8914, !noalias !8907
  unreachable, !dbg !8914

bb34.i1798:                                       ; preds = %bb13.i1733
  br label %bb14.i1737, !dbg !8919

bb15.i1742:                                       ; preds = %bb14.i1737
  %1088 = getelementptr inbounds nuw float, ptr %_132.0.i1734, i32 %iter.sroa.4.0.i169717895, !dbg !8916
  store float %newest.sroa.0.0.i1738, ptr %1088, align 4, !dbg !8916, !noalias !8907
  %_40.i1744 = add i32 %_30.i1728, 1, !dbg !8920
  %complete.i1745 = icmp eq i32 %_40.i1744, %shape.i1710, !dbg !8920
  br i1 %complete.i1745, label %bb19.i1767, label %bb17.i1746, !dbg !8921

panic8.i1741:                                     ; preds = %bb14.i1737
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_132.1.i1730, i32 noundef %_132.1.i1730, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1fa5d88c1a2cc292a2767c48606a38fe) #33, !dbg !8916, !noalias !8907
  unreachable, !dbg !8916

bb17.i1746:                                       ; preds = %bb15.i1742
  %_42.i1748 = add i32 %iter.sroa.4.0.i169717895, %_43.i1747, !dbg !8922
  %_45.i1750 = icmp ult i32 %_42.i1748, %_128.1.i1718, !dbg !8923
  br i1 %_45.i1750, label %bb27.i1760, label %panic9.i1751, !dbg !8923

panic9.i1751:                                     ; preds = %bb17.i1746
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_42.i1748, i32 noundef %_128.1.i1718, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70d40ae2302f3ea19fa0c4a65fe82786) #33, !dbg !8923, !noalias !8907
  unreachable, !dbg !8923

bb27.i1760:                                       ; preds = %bb17.i1746
  %1089 = getelementptr inbounds nuw float, ptr %_128.0.i1722, i32 %_42.i1748, !dbg !8923
  %_41.i1754 = load float, ptr %1089, align 4, !dbg !8923, !noalias !8907, !noundef !10
  %_117.i1755 = fcmp olt float %_41.i1754, %newest.sroa.0.0.i1738, !dbg !8924
  %newest.sroa.0.1.i1756 = select i1 %_117.i1755, float %_41.i1754, float %newest.sroa.0.0.i1738, !dbg !8924
  store float %newest.sroa.0.1.i1756, ptr %iter.sroa.0.0.ptr.i169917897, align 4, !dbg !8926, !alias.scope !8890, !noalias !8927
  br label %bb28.i1762, !dbg !8928

bb28.i1762:                                       ; preds = %bb22.i1795, %bb19.i1767, %bb27.i1760
  %storemerge15239 = phi i32 [ %_40.i1744, %bb27.i1760 ], [ 0, %bb19.i1767 ], [ 0, %bb22.i1795 ], !dbg !8929
  store i32 %storemerge15239, ptr %1085, align 4, !dbg !8929, !noalias !8907
  %1090 = icmp eq i32 %1078, 0, !dbg !8895
  br i1 %1090, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1800.loopexit, label %bb29.i1700, !dbg !8895

bb19.i1767:                                       ; preds = %bb15.i1742
  store float %newest.sroa.0.0.i1738, ptr %iter.sroa.0.0.ptr.i169917897, align 4, !dbg !8926, !alias.scope !8890, !noalias !8927
  %_118.i177317890.not = icmp eq i32 %shape.i1710, 0, !dbg !8930
  br i1 %_118.i177317890.not, label %bb28.i1762, label %bb40.i1780.preheader, !dbg !8934

bb40.i1780.preheader:                             ; preds = %bb19.i1767
  %1091 = load float, ptr %1083, align 4, !dbg !8935, !noalias !8907, !noundef !10
  br label %bb40.i1780, !dbg !8936

bb40.i1780:                                       ; preds = %bb40.i1780.preheader, %bb22.i1795
  %iter2.sroa.0.0.i177217893 = phi i32 [ %_119.i1781, %bb22.i1795 ], [ 0, %bb40.i1780.preheader ]
  %suffix.sroa.0.0.i177117892 = phi float [ %suffix.sroa.0.1.i1791, %bb22.i1795 ], [ %1091, %bb40.i1780.preheader ]
  %end.sroa.0.1.i177017891 = phi i32 [ %1094, %bb22.i1795 ], [ %spec.select.i1713, %bb40.i1780.preheader ]
  %_54.i1782 = mul i32 %end.sroa.0.1.i177017891, %width.i1694, !dbg !8937
  %_53.i1783 = add i32 %_54.i1782, %iter.sroa.4.0.i169717895, !dbg !8937
  %_57.i1785 = icmp ult i32 %_53.i1783, %_128.1.i1718, !dbg !8936
  br i1 %_57.i1785, label %bb22.i1795, label %panic13.i1786, !dbg !8936

panic13.i1786:                                    ; preds = %bb40.i1780
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_53.i1783, i32 noundef %_128.1.i1718, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a271bbb7fd83c3605ea58a06b7065fa4) #33, !dbg !8936, !noalias !8907
  unreachable, !dbg !8936

bb22.i1795:                                       ; preds = %bb40.i1780
  %_119.i1781 = add nuw i32 %iter2.sroa.0.0.i177217893, 1, !dbg !8938
  %1092 = getelementptr inbounds nuw float, ptr %_128.0.i1722, i32 %_53.i1783, !dbg !8936
  %_52.i1789 = load float, ptr %1092, align 4, !dbg !8936, !noalias !8907, !noundef !10
  %_121.i1790 = fcmp olt float %suffix.sroa.0.0.i177117892, %_52.i1789, !dbg !8941
  %suffix.sroa.0.1.i1791 = select i1 %_121.i1790, float %suffix.sroa.0.0.i177117892, float %_52.i1789, !dbg !8941
  store float %suffix.sroa.0.1.i1791, ptr %1092, align 4, !dbg !8943, !noalias !8907
  %1093 = icmp eq i32 %end.sroa.0.1.i177017891, 0, !dbg !8944
  %spec.store.select.i1797 = select i1 %1093, i32 %_87.i, i32 %end.sroa.0.1.i177017891, !dbg !8944
  %1094 = add i32 %spec.store.select.i1797, -1, !dbg !8945
  %exitcond20509.not = icmp eq i32 %_119.i1781, %shape.i1710, !dbg !8930
  br i1 %exitcond20509.not, label %bb28.i1762, label %bb40.i1780, !dbg !8934

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1800.loopexit: ; preds = %bb28.i1762, %bb29.i1700
  %lanes.i5245.sroa.0.0.copyload.pre = load <4 x float>, ptr %scratch.i, align 4, !dbg !8946, !alias.scope !8951, !noalias !8955
  br label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1800, !dbg !8959

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1800: ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1800.loopexit, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5957
  %lanes.i5245.sroa.0.0.copyload = phi <4 x float> [ %lanes.i5245.sroa.0.0.copyload.pre, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1800.loopexit ], [ %lanes.i5270.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5957 ], !dbg !8946
  %1095 = fmul <4 x float> %lanes.i5245.sroa.0.0.copyload, splat (float 1.638400e+04), !dbg !8960
  %1096 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %1095), !dbg !8964
  %1097 = fmul <4 x float> %1096, splat (float 0x3F10000000000000), !dbg !8968
  %1098 = icmp eq i32 %width.i.i, 0, !dbg !8972
  %_163.1.i.i.pre = load i32, ptr %717, align 4, !dbg !8974, !alias.scope !8833, !noalias !8834
  br i1 %1098, label %bb53.i.i, label %bb36.i.i.lr.ph, !dbg !8972

bb36.i.i.lr.ph:                                   ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1800
  %_159.1.i.i = load i32, ptr %711, align 4, !alias.scope !8833, !noalias !8834, !noundef !10
  %_159.0.i.i = load ptr, ptr %712, align 4, !nonnull !10
  %_161.0.i.i = load ptr, ptr %718, align 4, !nonnull !10
  %exitcond20515.not = icmp eq i32 %_159.1.i.i, 0, !dbg !8975
  br i1 %exitcond20515.not, label %panic.i.i, label %bb14.i.i, !dbg !8975

bb34.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5259
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i517, i32 noundef %_158.1.i.i, i32 noundef %_158.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_db7c42041d7aaaba4839906f37294547) #33, !dbg !8976, !noalias !8977
  unreachable, !dbg !8976

bb53.i.i.loopexit:                                ; preds = %bb18.i.i.7, %bb18.i.i.6, %bb18.i.i.5, %bb18.i.i.4, %bb18.i.i.3, %bb18.i.i.2, %bb18.i.i.1, %bb18.i.i
  %lanes.i5238.sroa.0.0.copyload.pre = load <4 x float>, ptr %scratch.i, align 4, !dbg !8978, !alias.scope !8983, !noalias !8987
  br label %bb53.i.i, !dbg !8991

bb53.i.i:                                         ; preds = %bb53.i.i.loopexit, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1800
  %lanes.i5238.sroa.0.0.copyload = phi <4 x float> [ %lanes.i5238.sroa.0.0.copyload.pre, %bb53.i.i.loopexit ], [ %lanes.i5245.sroa.0.0.copyload, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1800 ], !dbg !8978
  %1099 = fadd <4 x float> %1097, %972, !dbg !8992
  %1100 = fsub <4 x float> %1099, %lanes.i5238.sroa.0.0.copyload, !dbg !8996
  %_123.i.i = icmp ugt i32 %_22.i.i517, %_163.1.i.i.pre, !dbg !9000
  br i1 %_123.i.i, label %bb41.i.i, label %bb42.i.i, !dbg !9000, !prof !902

bb14.i.i:                                         ; preds = %bb36.i.i.lr.ph
  %1101 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 8, !dbg !8975
  %_42.i.i523 = load i32, ptr %1101, align 4, !dbg !8975, !noalias !9003, !noundef !10
  %1102 = add i32 %_42.i.i523, %ring_cursor.sroa.0.1.i49317904, !dbg !9004
  %_45.not.i.i = icmp ult i32 %1102, %_87.i, !dbg !9005
  %1103 = select i1 %_45.not.i.i, i32 0, i32 %_87.i, !dbg !9005
  %spec.select.i.i = sub nuw i32 %1102, %1103, !dbg !9005
  %_49.i.i524 = mul i32 %spec.select.i.i, %width.i.i, !dbg !9006
  %_51.i.i = icmp ult i32 %_49.i.i524, %_163.1.i.i.pre, !dbg !9007
  br i1 %_51.i.i, label %bb18.i.i, label %panic1.i.i, !dbg !9007

panic.i.i:                                        ; preds = %bb36.i.i.7, %bb36.i.i.6, %bb36.i.i.5, %bb36.i.i.4, %bb36.i.i.3, %bb36.i.i.2, %bb36.i.i.1, %bb36.i.i.lr.ph
  %_159.1.i.i.lcssa.ph = phi i32 [ 7, %bb36.i.i.7 ], [ 6, %bb36.i.i.6 ], [ 5, %bb36.i.i.5 ], [ 4, %bb36.i.i.4 ], [ 3, %bb36.i.i.3 ], [ 2, %bb36.i.i.2 ], [ 1, %bb36.i.i.1 ], [ 0, %bb36.i.i.lr.ph ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_159.1.i.i.lcssa.ph, i32 noundef %_159.1.i.i.lcssa.ph, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_33746731a929bad63709ddedbca82f5f) #33, !dbg !8975, !noalias !9003
  unreachable, !dbg !8975

bb18.i.i:                                         ; preds = %bb14.i.i
  %1104 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_49.i.i524, !dbg !9007
  %_47.i.i = load float, ptr %1104, align 4, !dbg !9007, !noalias !9003, !noundef !10
  store float %_47.i.i, ptr %scratch.i, align 4, !dbg !9008, !alias.scope !8830, !noalias !9009
  %1105 = icmp eq i32 %width.i.i, 1, !dbg !8972
  br i1 %1105, label %bb53.i.i.loopexit, label %bb36.i.i.1, !dbg !8972

bb36.i.i.1:                                       ; preds = %bb18.i.i
  %exitcond20515.1.not = icmp eq i32 %_159.1.i.i, 1, !dbg !8975
  br i1 %exitcond20515.1.not, label %panic.i.i, label %bb14.i.i.1, !dbg !8975

bb14.i.i.1:                                       ; preds = %bb36.i.i.1
  %1106 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 20, !dbg !8975
  %_42.i.i523.1 = load i32, ptr %1106, align 4, !dbg !8975, !noalias !9003, !noundef !10
  %1107 = add i32 %_42.i.i523.1, %ring_cursor.sroa.0.1.i49317904, !dbg !9004
  %_45.not.i.i.1 = icmp ult i32 %1107, %_87.i, !dbg !9005
  %1108 = select i1 %_45.not.i.i.1, i32 0, i32 %_87.i, !dbg !9005
  %spec.select.i.i.1 = sub nuw i32 %1107, %1108, !dbg !9005
  %_49.i.i524.1 = mul i32 %spec.select.i.i.1, %width.i.i, !dbg !9006
  %_48.i.i.1 = add i32 %_49.i.i524.1, 1, !dbg !9006
  %_51.i.i.1 = icmp ult i32 %_48.i.i.1, %_163.1.i.i.pre, !dbg !9007
  br i1 %_51.i.i.1, label %bb18.i.i.1, label %panic1.i.i, !dbg !9007

bb18.i.i.1:                                       ; preds = %bb14.i.i.1
  %1109 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.1, !dbg !9007
  %_47.i.i.1 = load float, ptr %1109, align 4, !dbg !9007, !noalias !9003, !noundef !10
  store float %_47.i.i.1, ptr %iter.sroa.0.0.ptr.i.i17901.1, align 4, !dbg !9008, !alias.scope !8830, !noalias !9009
  %1110 = icmp eq i32 %width.i.i, 2, !dbg !8972
  br i1 %1110, label %bb53.i.i.loopexit, label %bb36.i.i.2, !dbg !8972

bb36.i.i.2:                                       ; preds = %bb18.i.i.1
  %exitcond20515.2.not = icmp eq i32 %_159.1.i.i, 2, !dbg !8975
  br i1 %exitcond20515.2.not, label %panic.i.i, label %bb14.i.i.2, !dbg !8975

bb14.i.i.2:                                       ; preds = %bb36.i.i.2
  %1111 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 32, !dbg !8975
  %_42.i.i523.2 = load i32, ptr %1111, align 4, !dbg !8975, !noalias !9003, !noundef !10
  %1112 = add i32 %_42.i.i523.2, %ring_cursor.sroa.0.1.i49317904, !dbg !9004
  %_45.not.i.i.2 = icmp ult i32 %1112, %_87.i, !dbg !9005
  %1113 = select i1 %_45.not.i.i.2, i32 0, i32 %_87.i, !dbg !9005
  %spec.select.i.i.2 = sub nuw i32 %1112, %1113, !dbg !9005
  %_49.i.i524.2 = mul i32 %spec.select.i.i.2, %width.i.i, !dbg !9006
  %_48.i.i.2 = add i32 %_49.i.i524.2, 2, !dbg !9006
  %_51.i.i.2 = icmp ult i32 %_48.i.i.2, %_163.1.i.i.pre, !dbg !9007
  br i1 %_51.i.i.2, label %bb18.i.i.2, label %panic1.i.i, !dbg !9007

bb18.i.i.2:                                       ; preds = %bb14.i.i.2
  %1114 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.2, !dbg !9007
  %_47.i.i.2 = load float, ptr %1114, align 4, !dbg !9007, !noalias !9003, !noundef !10
  store float %_47.i.i.2, ptr %iter.sroa.0.0.ptr.i.i17901.2, align 4, !dbg !9008, !alias.scope !8830, !noalias !9009
  %1115 = icmp eq i32 %width.i.i, 3, !dbg !8972
  br i1 %1115, label %bb53.i.i.loopexit, label %bb36.i.i.3, !dbg !8972

bb36.i.i.3:                                       ; preds = %bb18.i.i.2
  %exitcond20515.3.not = icmp eq i32 %_159.1.i.i, 3, !dbg !8975
  br i1 %exitcond20515.3.not, label %panic.i.i, label %bb14.i.i.3, !dbg !8975

bb14.i.i.3:                                       ; preds = %bb36.i.i.3
  %1116 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 44, !dbg !8975
  %_42.i.i523.3 = load i32, ptr %1116, align 4, !dbg !8975, !noalias !9003, !noundef !10
  %1117 = add i32 %_42.i.i523.3, %ring_cursor.sroa.0.1.i49317904, !dbg !9004
  %_45.not.i.i.3 = icmp ult i32 %1117, %_87.i, !dbg !9005
  %1118 = select i1 %_45.not.i.i.3, i32 0, i32 %_87.i, !dbg !9005
  %spec.select.i.i.3 = sub nuw i32 %1117, %1118, !dbg !9005
  %_49.i.i524.3 = mul i32 %spec.select.i.i.3, %width.i.i, !dbg !9006
  %_48.i.i.3 = add i32 %_49.i.i524.3, 3, !dbg !9006
  %_51.i.i.3 = icmp ult i32 %_48.i.i.3, %_163.1.i.i.pre, !dbg !9007
  br i1 %_51.i.i.3, label %bb18.i.i.3, label %panic1.i.i, !dbg !9007

bb18.i.i.3:                                       ; preds = %bb14.i.i.3
  %1119 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.3, !dbg !9007
  %_47.i.i.3 = load float, ptr %1119, align 4, !dbg !9007, !noalias !9003, !noundef !10
  store float %_47.i.i.3, ptr %iter.sroa.0.0.ptr.i.i17901.3, align 4, !dbg !9008, !alias.scope !8830, !noalias !9009
  %1120 = icmp eq i32 %width.i.i, 4, !dbg !8972
  br i1 %1120, label %bb53.i.i.loopexit, label %bb36.i.i.4, !dbg !8972

bb36.i.i.4:                                       ; preds = %bb18.i.i.3
  %exitcond20515.4.not = icmp eq i32 %_159.1.i.i, 4, !dbg !8975
  br i1 %exitcond20515.4.not, label %panic.i.i, label %bb14.i.i.4, !dbg !8975

bb14.i.i.4:                                       ; preds = %bb36.i.i.4
  %1121 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 56, !dbg !8975
  %_42.i.i523.4 = load i32, ptr %1121, align 4, !dbg !8975, !noalias !9003, !noundef !10
  %1122 = add i32 %_42.i.i523.4, %ring_cursor.sroa.0.1.i49317904, !dbg !9004
  %_45.not.i.i.4 = icmp ult i32 %1122, %_87.i, !dbg !9005
  %1123 = select i1 %_45.not.i.i.4, i32 0, i32 %_87.i, !dbg !9005
  %spec.select.i.i.4 = sub nuw i32 %1122, %1123, !dbg !9005
  %_49.i.i524.4 = mul i32 %spec.select.i.i.4, %width.i.i, !dbg !9006
  %_48.i.i.4 = add i32 %_49.i.i524.4, 4, !dbg !9006
  %_51.i.i.4 = icmp ult i32 %_48.i.i.4, %_163.1.i.i.pre, !dbg !9007
  br i1 %_51.i.i.4, label %bb18.i.i.4, label %panic1.i.i, !dbg !9007

bb18.i.i.4:                                       ; preds = %bb14.i.i.4
  %1124 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.4, !dbg !9007
  %_47.i.i.4 = load float, ptr %1124, align 4, !dbg !9007, !noalias !9003, !noundef !10
  store float %_47.i.i.4, ptr %iter.sroa.0.0.ptr.i.i17901.4, align 4, !dbg !9008, !alias.scope !8830, !noalias !9009
  %1125 = icmp eq i32 %width.i.i, 5, !dbg !8972
  br i1 %1125, label %bb53.i.i.loopexit, label %bb36.i.i.5, !dbg !8972

bb36.i.i.5:                                       ; preds = %bb18.i.i.4
  %exitcond20515.5.not = icmp eq i32 %_159.1.i.i, 5, !dbg !8975
  br i1 %exitcond20515.5.not, label %panic.i.i, label %bb14.i.i.5, !dbg !8975

bb14.i.i.5:                                       ; preds = %bb36.i.i.5
  %1126 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 68, !dbg !8975
  %_42.i.i523.5 = load i32, ptr %1126, align 4, !dbg !8975, !noalias !9003, !noundef !10
  %1127 = add i32 %_42.i.i523.5, %ring_cursor.sroa.0.1.i49317904, !dbg !9004
  %_45.not.i.i.5 = icmp ult i32 %1127, %_87.i, !dbg !9005
  %1128 = select i1 %_45.not.i.i.5, i32 0, i32 %_87.i, !dbg !9005
  %spec.select.i.i.5 = sub nuw i32 %1127, %1128, !dbg !9005
  %_49.i.i524.5 = mul i32 %spec.select.i.i.5, %width.i.i, !dbg !9006
  %_48.i.i.5 = add i32 %_49.i.i524.5, 5, !dbg !9006
  %_51.i.i.5 = icmp ult i32 %_48.i.i.5, %_163.1.i.i.pre, !dbg !9007
  br i1 %_51.i.i.5, label %bb18.i.i.5, label %panic1.i.i, !dbg !9007

bb18.i.i.5:                                       ; preds = %bb14.i.i.5
  %1129 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.5, !dbg !9007
  %_47.i.i.5 = load float, ptr %1129, align 4, !dbg !9007, !noalias !9003, !noundef !10
  store float %_47.i.i.5, ptr %iter.sroa.0.0.ptr.i.i17901.5, align 4, !dbg !9008, !alias.scope !8830, !noalias !9009
  %1130 = icmp eq i32 %width.i.i, 6, !dbg !8972
  br i1 %1130, label %bb53.i.i.loopexit, label %bb36.i.i.6, !dbg !8972

bb36.i.i.6:                                       ; preds = %bb18.i.i.5
  %exitcond20515.6.not = icmp eq i32 %_159.1.i.i, 6, !dbg !8975
  br i1 %exitcond20515.6.not, label %panic.i.i, label %bb14.i.i.6, !dbg !8975

bb14.i.i.6:                                       ; preds = %bb36.i.i.6
  %1131 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 80, !dbg !8975
  %_42.i.i523.6 = load i32, ptr %1131, align 4, !dbg !8975, !noalias !9003, !noundef !10
  %1132 = add i32 %_42.i.i523.6, %ring_cursor.sroa.0.1.i49317904, !dbg !9004
  %_45.not.i.i.6 = icmp ult i32 %1132, %_87.i, !dbg !9005
  %1133 = select i1 %_45.not.i.i.6, i32 0, i32 %_87.i, !dbg !9005
  %spec.select.i.i.6 = sub nuw i32 %1132, %1133, !dbg !9005
  %_49.i.i524.6 = mul i32 %spec.select.i.i.6, %width.i.i, !dbg !9006
  %_48.i.i.6 = add i32 %_49.i.i524.6, 6, !dbg !9006
  %_51.i.i.6 = icmp ult i32 %_48.i.i.6, %_163.1.i.i.pre, !dbg !9007
  br i1 %_51.i.i.6, label %bb18.i.i.6, label %panic1.i.i, !dbg !9007

bb18.i.i.6:                                       ; preds = %bb14.i.i.6
  %1134 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.6, !dbg !9007
  %_47.i.i.6 = load float, ptr %1134, align 4, !dbg !9007, !noalias !9003, !noundef !10
  store float %_47.i.i.6, ptr %iter.sroa.0.0.ptr.i.i17901.6, align 4, !dbg !9008, !alias.scope !8830, !noalias !9009
  %1135 = icmp eq i32 %width.i.i, 7, !dbg !8972
  br i1 %1135, label %bb53.i.i.loopexit, label %bb36.i.i.7, !dbg !8972

bb36.i.i.7:                                       ; preds = %bb18.i.i.6
  %exitcond20515.7.not = icmp eq i32 %_159.1.i.i, 7, !dbg !8975
  br i1 %exitcond20515.7.not, label %panic.i.i, label %bb14.i.i.7, !dbg !8975

bb14.i.i.7:                                       ; preds = %bb36.i.i.7
  %1136 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 92, !dbg !8975
  %_42.i.i523.7 = load i32, ptr %1136, align 4, !dbg !8975, !noalias !9003, !noundef !10
  %1137 = add i32 %_42.i.i523.7, %ring_cursor.sroa.0.1.i49317904, !dbg !9004
  %_45.not.i.i.7 = icmp ult i32 %1137, %_87.i, !dbg !9005
  %1138 = select i1 %_45.not.i.i.7, i32 0, i32 %_87.i, !dbg !9005
  %spec.select.i.i.7 = sub nuw i32 %1137, %1138, !dbg !9005
  %_49.i.i524.7 = mul i32 %spec.select.i.i.7, %width.i.i, !dbg !9006
  %_48.i.i.7 = add i32 %_49.i.i524.7, 7, !dbg !9006
  %_51.i.i.7 = icmp ult i32 %_48.i.i.7, %_163.1.i.i.pre, !dbg !9007
  br i1 %_51.i.i.7, label %bb18.i.i.7, label %panic1.i.i, !dbg !9007

bb18.i.i.7:                                       ; preds = %bb14.i.i.7
  %1139 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.7, !dbg !9007
  %_47.i.i.7 = load float, ptr %1139, align 4, !dbg !9007, !noalias !9003, !noundef !10
  store float %_47.i.i.7, ptr %iter.sroa.0.0.ptr.i.i17901.7, align 4, !dbg !9008, !alias.scope !8830, !noalias !9009
  br label %bb53.i.i.loopexit, !dbg !8972

panic1.i.i:                                       ; preds = %bb14.i.i.7, %bb14.i.i.6, %bb14.i.i.5, %bb14.i.i.4, %bb14.i.i.3, %bb14.i.i.2, %bb14.i.i.1, %bb14.i.i
  %_48.i.i.lcssa.ph = phi i32 [ %_48.i.i.7, %bb14.i.i.7 ], [ %_48.i.i.6, %bb14.i.i.6 ], [ %_48.i.i.5, %bb14.i.i.5 ], [ %_48.i.i.4, %bb14.i.i.4 ], [ %_48.i.i.3, %bb14.i.i.3 ], [ %_48.i.i.2, %bb14.i.i.2 ], [ %_48.i.i.1, %bb14.i.i.1 ], [ %_49.i.i524, %bb14.i.i ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_48.i.i.lcssa.ph, i32 noundef %_163.1.i.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_0e76a26fbb751dfb8cf8a069b5b0d39a) #33, !dbg !9007, !noalias !9003
  unreachable, !dbg !9007

bb42.i.i:                                         ; preds = %bb53.i.i
  %_126.i.i = sub nuw i32 %_163.1.i.i.pre, %_22.i.i517, !dbg !9010
  %_8.i5949 = icmp samesign ugt i32 %_126.i.i, 3, !dbg !9011
  br i1 %_8.i5949, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5952, label %bb2.i5950, !dbg !9011, !prof !1153

bb2.i5950:                                        ; preds = %bb42.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_126.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !9016, !noalias !9017
  unreachable, !dbg !9016

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5952: ; preds = %bb42.i.i
  %_163.0.i.i = load ptr, ptr %718, align 4, !dbg !8974, !alias.scope !8833, !noalias !8834, !nonnull !10, !noundef !10
  %_130.i.i = getelementptr inbounds nuw float, ptr %_163.0.i.i, i32 %_22.i.i517, !dbg !9021
  store <4 x float> %1097, ptr %_130.i.i, align 4, !dbg !9023, !alias.scope !9027, !noalias !9031
  %_66.i.i15246 = load <4 x float>, ptr %721, align 16, !dbg !9033
  %1140 = fdiv <4 x float> %1100, %_62.i.i52715245, !dbg !9034
  %1141 = fsub <4 x float> splat (float 1.000000e+00), %1140, !dbg !9038
  %1142 = fsub <4 x float> %1141, %_66.i.i15246, !dbg !9042
  %1143 = fmul <4 x float> %_9.i.i50215228.pre, %1142, !dbg !9046
  %1144 = fadd <4 x float> %_66.i.i15246, %1143, !dbg !9050
  %1145 = fcmp olt <4 x float> %1144, %1141, !dbg !9053
  %1146 = select <4 x i1> %1145, <4 x float> %1141, <4 x float> %1144, !dbg !9057
  %1147 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1146), !dbg !9058
  %1148 = fcmp uge <4 x float> %1147, splat (float 0x3BC79CA100000000), !dbg !9063
  %1149 = bitcast <4 x float> %1146 to <4 x i32>, !dbg !9068
  %1150 = select <4 x i1> %1148, <4 x i32> %1149, <4 x i32> zeroinitializer, !dbg !9068
  store <4 x i32> %1150, ptr %721, align 16, !dbg !9071
  %1151 = bitcast <4 x i32> %1150 to <4 x float>, !dbg !9072
  %1152 = fsub <4 x float> splat (float 1.000000e+00), %1151, !dbg !9076
  %_164.1.i.i = load i32, ptr %722, align 4, !dbg !9077, !alias.scope !8833, !noalias !8834, !noundef !10
  %_74.i.i = mul i32 %width.i.i, %main_cursor.sroa.0.1.i49417905, !dbg !9078
  %_134.i.i = icmp ugt i32 %_74.i.i, %_164.1.i.i, !dbg !9079
  br i1 %_134.i.i, label %bb47.i.i, label %bb48.i.i, !dbg !9079, !prof !902

bb41.i.i:                                         ; preds = %bb53.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i517, i32 noundef %_163.1.i.i.pre, i32 noundef %_163.1.i.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c2286ff41a33b8c11aa270fe215441de) #33, !dbg !9082, !noalias !9003
  unreachable, !dbg !9082

bb48.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5952
  %_137.i.i = sub nuw i32 %_164.1.i.i, %_74.i.i, !dbg !9083
  %_8.i5232 = icmp samesign ugt i32 %_137.i.i, 3, !dbg !9084
  br i1 %_8.i5232, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5942, label %bb2.i5233, !dbg !9084, !prof !1153

bb2.i5233:                                        ; preds = %bb48.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_137.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !9089, !noalias !9090
  unreachable, !dbg !9089

bb47.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5952
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_74.i.i, i32 noundef %_164.1.i.i, i32 noundef %_164.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a3a21efd18b403ec25db545d522c479) #33, !dbg !9094, !noalias !9003
  unreachable, !dbg !9094

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5942: ; preds = %bb48.i.i
  %_164.0.i.i = load ptr, ptr %723, align 4, !dbg !9077, !alias.scope !8833, !noalias !8834, !nonnull !10, !noundef !10
  %_141.i.i = getelementptr inbounds nuw float, ptr %_164.0.i.i, i32 %_74.i.i, !dbg !9095
  %lanes.i5229.sroa.0.0.copyload = load <4 x i32>, ptr %_141.i.i, align 4, !dbg !9097, !alias.scope !9101, !noalias !9105
  store <4 x i32> %lanes.i5252.sroa.0.0.copyload, ptr %_141.i.i, align 4, !dbg !9107, !alias.scope !9112, !noalias !9116
  %1153 = bitcast <4 x i32> %lanes.i5229.sroa.0.0.copyload to <4 x float>, !dbg !9120
  %1154 = fmul <4 x float> %1152, %1153, !dbg !9124
  %1155 = bitcast <4 x i32> %lanes.i5229.sroa.0.0.copyload to <16 x i8>, !dbg !9125
  %1156 = bitcast <4 x float> %1154 to <16 x i8>, !dbg !9129
  %_4.i6542 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1155, <16 x i8> %1156, <16 x i8> %707), !dbg !9130
  store <16 x i8> %_4.i6542, ptr %_147.i, align 4, !dbg !9131, !alias.scope !9136, !noalias !9140
  %1157 = add i32 %main_cursor.sroa.0.1.i49417905, 1, !dbg !9144
  %_100.i = icmp eq i32 %1157, %_102.i, !dbg !9145
  %spec.store.select11.i = select i1 %_100.i, i32 0, i32 %1157, !dbg !9145
  %1158 = add i32 %ring_cursor.sroa.0.1.i49317904, 1, !dbg !9146
  %_103.i = icmp eq i32 %1158, %_87.i, !dbg !9147
  %spec.store.select12.i = select i1 %_103.i, i32 0, i32 %1158, !dbg !9147
  %exitcond20519.not = icmp eq i32 %974, %umax20518, !dbg !9148
  br i1 %exitcond20519.not, label %bb16.i.bb13.i.loopexit_crit_edge, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5309, !dbg !7403

bb48.i:                                           ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5962
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i496, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_3ec0ee57975400dd5dbd052e150c5392) #33, !dbg !9151, !noalias !8799
  unreachable, !dbg !9151

_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %bb13.i.loopexit, %bb11.i
  %ring_cursor.sroa.0.0.i485.lcssa = phi i32 [ %_37.i479, %bb11.i ], [ %ring_cursor.sroa.0.1.i493.lcssa, %bb13.i.loopexit ], !dbg !7359
  %main_cursor.sroa.0.0.i486.lcssa = phi i32 [ %_35.i, %bb11.i ], [ %main_cursor.sroa.0.1.i494.lcssa, %bb13.i.loopexit ], !dbg !7356
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_(ptr noalias noundef readonly align 16 captures(none) dereferenceable(368) %hot_left.i472, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33) #32, !dbg !9152
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_(ptr noalias noundef readonly align 16 captures(none) dereferenceable(368) %hot_right.i471, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34) #32, !dbg !9153
  store i32 %main_cursor.sroa.0.0.i486.lcssa, ptr %_35, align 4, !dbg !9154, !alias.scope !7338, !noalias !7358
  store i32 %ring_cursor.sroa.0.0.i485.lcssa, ptr %81, align 4, !dbg !9155, !alias.scope !7338, !noalias !7358
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i468), !dbg !9156, !noalias !7363
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i469), !dbg !9157, !noalias !7363
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i), !dbg !9158, !noalias !7363
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !7333

bb7.i:                                            ; preds = %bb5.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9159), !dbg !9162
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9163), !dbg !9162
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9165), !dbg !9162
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9167), !dbg !9162
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9169), !dbg !9162
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_(ptr noalias noundef align 16 captures(none) dereferenceable(368) %hot_left.i44, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #32, !dbg !9171
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_(ptr noalias noundef align 16 captures(none) dereferenceable(368) %hot_right.i43, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #32, !dbg !9175
  %1159 = load i8, ptr %82, align 16, !dbg !9177, !range !4765, !alias.scope !9159, !noalias !9181, !noundef !10
  %1160 = load i8, ptr %83, align 1, !dbg !9184, !range !4765, !alias.scope !9159, !noalias !9181, !noundef !10
  %ring.i52 = load i32, ptr %84, align 4, !dbg !9186, !alias.scope !9163, !noalias !9188, !noundef !10
  %main.i53 = load i32, ptr %85, align 4, !dbg !9189, !alias.scope !9163, !noalias !9188, !noundef !10
  %_36.i54 = load i32, ptr %_35, align 4, !dbg !9191, !alias.scope !9169, !noalias !9193, !noundef !10
  %_37.i55 = load i32, ptr %86, align 4, !dbg !9194, !alias.scope !9169, !noalias !9193, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i41), !dbg !9196, !noalias !9198
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i41, i8 0, i32 1024, i1 false), !noalias !9198
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i40), !dbg !9199, !noalias !9198
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i40, i8 0, i32 1024, i1 false), !noalias !9198
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i39), !dbg !9201, !noalias !9198
; call <true_peak_limiter::UniformHot<wide::f32x4_::f32x4>>::new
  call fastcc void @_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E3newB5_(ptr noalias noundef align 16 captures(none) dereferenceable(64) %uniform_left.i39, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33, i32 %ring.i52, i32 %main.i53) #32, !dbg !9203
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_right.i38), !dbg !9204, !noalias !9198
  %_32.val = load i32, ptr %84, align 4, !dbg !9206, !noundef !10
  %_32.val6273 = load i32, ptr %85, align 4, !dbg !9206, !noundef !10
; call <true_peak_limiter::UniformHot<wide::f32x4_::f32x4>>::new
  call fastcc void @_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E3newB5_(ptr noalias noundef align 16 captures(none) dereferenceable(64) %uniform_right.i38, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34, i32 %_32.val, i32 %_32.val6273) #32, !dbg !9206
  %_162.not.i6618131 = icmp eq i32 %frames, 0, !dbg !9207
  br i1 %_162.not.i6618131, label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb44.i67.lr.ph, !dbg !9207

bb44.i67.lr.ph:                                   ; preds = %bb7.i
  %_33.i50 = trunc nuw i8 %1160 to i1, !dbg !9184
  %spec.store.select25.i51 = select i1 %_33.i50, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !9184
  %_32.i48 = trunc nuw i8 %1159 to i1, !dbg !9177
  %link.sroa.0.0.i49 = select i1 %_32.i48, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !9177
  %d9.i6544 = lshr i32 %frames, 5, !dbg !9217
  %r2.i6545 = and i32 %frames, 31, !dbg !9224
  %_19.not.i6546 = icmp ne i32 %r2.i6545, 0, !dbg !9225
  %1161 = zext i1 %_19.not.i6546 to i32, !dbg !9225
  %yield_count.sroa.0.0.i6547 = add nuw nsw i32 %d9.i6544, %1161, !dbg !9225
  %history.i42.i.sroa.7.0.hot_left.i44.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i44, i32 16
  %history.i42.i.sroa.10.0.hot_left.i44.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i44, i32 32
  %history.i42.i.sroa.13.0.hot_left.i44.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i44, i32 48
  %history.i42.i.sroa.16.0.hot_left.i44.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i44, i32 64
  %history.i42.i.sroa.19.0.hot_left.i44.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i44, i32 80
  %history.i42.i.sroa.22.0.hot_left.i44.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i44, i32 96
  %history.i42.i.sroa.26.0.hot_left.i44.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i44, i32 112
  %history.i42.i.sroa.29.0.hot_left.i44.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i44, i32 128
  %history.i42.i.sroa.32.0.hot_left.i44.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i44, i32 144
  %history.i42.i.sroa.35.0.hot_left.i44.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i44, i32 160
  %history.i42.i.sroa.38.0.hot_left.i44.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i44, i32 176
  %1162 = getelementptr inbounds nuw i8, ptr %self, i32 32
  %1163 = getelementptr inbounds nuw i8, ptr %self, i32 48
  %1164 = getelementptr inbounds nuw i8, ptr %self, i32 64
  %row1.i.i.i79.i = getelementptr inbounds nuw i8, ptr %self, i32 80
  %1165 = getelementptr inbounds nuw i8, ptr %self, i32 96
  %1166 = getelementptr inbounds nuw i8, ptr %self, i32 112
  %1167 = getelementptr inbounds nuw i8, ptr %self, i32 128
  %row3.i.i.i93.i = getelementptr inbounds nuw i8, ptr %self, i32 144
  %1168 = getelementptr inbounds nuw i8, ptr %self, i32 160
  %1169 = getelementptr inbounds nuw i8, ptr %self, i32 176
  %1170 = getelementptr inbounds nuw i8, ptr %self, i32 192
  %row5.i.i.i107.i = getelementptr inbounds nuw i8, ptr %self, i32 208
  %1171 = getelementptr inbounds nuw i8, ptr %self, i32 224
  %1172 = getelementptr inbounds nuw i8, ptr %self, i32 240
  %1173 = getelementptr inbounds nuw i8, ptr %self, i32 256
  %row7.i.i.i121.i = getelementptr inbounds nuw i8, ptr %self, i32 272
  %1174 = getelementptr inbounds nuw i8, ptr %self, i32 288
  %1175 = getelementptr inbounds nuw i8, ptr %self, i32 304
  %1176 = getelementptr inbounds nuw i8, ptr %self, i32 320
  %row9.i.i.i135.i = getelementptr inbounds nuw i8, ptr %self, i32 336
  %1177 = getelementptr inbounds nuw i8, ptr %self, i32 352
  %1178 = getelementptr inbounds nuw i8, ptr %self, i32 368
  %1179 = getelementptr inbounds nuw i8, ptr %self, i32 384
  %row11.i.i.i149.i = getelementptr inbounds nuw i8, ptr %self, i32 400
  %1180 = getelementptr inbounds nuw i8, ptr %self, i32 416
  %1181 = getelementptr inbounds nuw i8, ptr %self, i32 432
  %1182 = getelementptr inbounds nuw i8, ptr %self, i32 448
  %row13.i.i.i163.i = getelementptr inbounds nuw i8, ptr %self, i32 464
  %1183 = getelementptr inbounds nuw i8, ptr %self, i32 480
  %1184 = getelementptr inbounds nuw i8, ptr %self, i32 496
  %1185 = getelementptr inbounds nuw i8, ptr %self, i32 512
  %row15.i.i.i177.i = getelementptr inbounds nuw i8, ptr %self, i32 528
  %1186 = getelementptr inbounds nuw i8, ptr %self, i32 544
  %1187 = getelementptr inbounds nuw i8, ptr %self, i32 560
  %1188 = getelementptr inbounds nuw i8, ptr %self, i32 576
  %row17.i.i.i191.i = getelementptr inbounds nuw i8, ptr %self, i32 592
  %1189 = getelementptr inbounds nuw i8, ptr %self, i32 608
  %1190 = getelementptr inbounds nuw i8, ptr %self, i32 624
  %1191 = getelementptr inbounds nuw i8, ptr %self, i32 640
  %row19.i.i.i205.i = getelementptr inbounds nuw i8, ptr %self, i32 656
  %1192 = getelementptr inbounds nuw i8, ptr %self, i32 672
  %1193 = getelementptr inbounds nuw i8, ptr %self, i32 688
  %1194 = getelementptr inbounds nuw i8, ptr %self, i32 704
  %row21.i.i.i219.i = getelementptr inbounds nuw i8, ptr %self, i32 720
  %1195 = getelementptr inbounds nuw i8, ptr %self, i32 736
  %1196 = getelementptr inbounds nuw i8, ptr %self, i32 752
  %1197 = getelementptr inbounds nuw i8, ptr %self, i32 768
  %history.i.i18.sroa.7.0.hot_right.i43.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i43, i32 16
  %history.i.i18.sroa.10.0.hot_right.i43.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i43, i32 32
  %history.i.i18.sroa.13.0.hot_right.i43.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i43, i32 48
  %history.i.i18.sroa.16.0.hot_right.i43.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i43, i32 64
  %history.i.i18.sroa.19.0.hot_right.i43.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i43, i32 80
  %history.i.i18.sroa.22.0.hot_right.i43.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i43, i32 96
  %history.i.i18.sroa.26.0.hot_right.i43.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i43, i32 112
  %history.i.i18.sroa.29.0.hot_right.i43.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i43, i32 128
  %history.i.i18.sroa.32.0.hot_right.i43.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i43, i32 144
  %history.i.i18.sroa.35.0.hot_right.i43.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i43, i32 160
  %history.i.i18.sroa.38.0.hot_right.i43.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i43, i32 176
  %1198 = getelementptr inbounds nuw i8, ptr %uniform_left.i39, i32 20
  %_68.i36.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i39, i32 24
  %_68.i36.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i39, i32 28
  %1199 = getelementptr inbounds nuw i8, ptr %uniform_right.i38, i32 20
  %_69.i35.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i38, i32 24
  %_69.i35.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i38, i32 28
  %_110.i124 = getelementptr inbounds nuw i8, ptr %hot_left.i44, i32 192
  %_111.i125 = getelementptr inbounds nuw i8, ptr %hot_left.i44, i32 256
  %1200 = getelementptr inbounds nuw i8, ptr %hot_left.i44, i32 240
  %1201 = getelementptr inbounds nuw i8, ptr %hot_left.i44, i32 224
  %1202 = getelementptr inbounds nuw i8, ptr %hot_left.i44, i32 208
  %1203 = getelementptr inbounds nuw i8, ptr %hot_left.i44, i32 304
  %1204 = getelementptr inbounds nuw i8, ptr %hot_left.i44, i32 288
  %1205 = getelementptr inbounds nuw i8, ptr %hot_left.i44, i32 272
  %_115.i128 = getelementptr inbounds nuw i8, ptr %hot_right.i43, i32 192
  %_116.i129 = getelementptr inbounds nuw i8, ptr %hot_right.i43, i32 256
  %1206 = getelementptr inbounds nuw i8, ptr %hot_right.i43, i32 240
  %1207 = getelementptr inbounds nuw i8, ptr %hot_right.i43, i32 224
  %1208 = getelementptr inbounds nuw i8, ptr %hot_right.i43, i32 208
  %1209 = getelementptr inbounds nuw i8, ptr %hot_right.i43, i32 304
  %1210 = getelementptr inbounds nuw i8, ptr %hot_right.i43, i32 288
  %1211 = getelementptr inbounds nuw i8, ptr %hot_right.i43, i32 272
  %1212 = bitcast <4 x i32> %link.sroa.0.0.i49 to <16 x i8>
  %1213 = getelementptr inbounds nuw i8, ptr %uniform_left.i39, i32 32
  %1214 = getelementptr inbounds nuw i8, ptr %uniform_left.i39, i32 36
  %_22.i279.i = getelementptr inbounds nuw i8, ptr %uniform_left.i39, i32 16
  %1215 = getelementptr inbounds nuw i8, ptr %uniform_left.i39, i32 40
  %1216 = getelementptr inbounds nuw i8, ptr %uniform_left.i39, i32 44
  %1217 = getelementptr inbounds nuw i8, ptr %hot_left.i44, i32 336
  %1218 = getelementptr inbounds nuw i8, ptr %hot_left.i44, i32 352
  %1219 = getelementptr inbounds nuw i8, ptr %hot_left.i44, i32 320
  %1220 = getelementptr inbounds nuw i8, ptr %uniform_left.i39, i32 52
  %1221 = getelementptr inbounds nuw i8, ptr %uniform_left.i39, i32 48
  %1222 = bitcast <4 x i32> %spec.store.select25.i51 to <16 x i8>
  %1223 = getelementptr inbounds nuw i8, ptr %uniform_right.i38, i32 32
  %1224 = getelementptr inbounds nuw i8, ptr %uniform_right.i38, i32 36
  %_22.i.i178 = getelementptr inbounds nuw i8, ptr %uniform_right.i38, i32 16
  %1225 = getelementptr inbounds nuw i8, ptr %uniform_right.i38, i32 40
  %1226 = getelementptr inbounds nuw i8, ptr %uniform_right.i38, i32 44
  %1227 = getelementptr inbounds nuw i8, ptr %hot_right.i43, i32 336
  %1228 = getelementptr inbounds nuw i8, ptr %hot_right.i43, i32 352
  %1229 = getelementptr inbounds nuw i8, ptr %hot_right.i43, i32 320
  %1230 = getelementptr inbounds nuw i8, ptr %uniform_right.i38, i32 52
  %1231 = getelementptr inbounds nuw i8, ptr %uniform_right.i38, i32 48
  br label %bb44.i67, !dbg !9207

bb15.i61.loopexit.loopexit:                       ; preds = %bb67.i227
  store i32 %storemerge.i1257.lcssa2262222670, ptr %_22.i279.i, align 4
  store i32 %storemerge.i.lcssa2265222688, ptr %_22.i.i178, align 4
  br label %bb15.i61.loopexit, !dbg !9207

bb15.i61.loopexit:                                ; preds = %bb15.i61.loopexit.loopexit, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i72
  %ring_cursor.sroa.0.1.i74.lcssa = phi i32 [ %ring_cursor.sroa.0.0.i6218132, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i72 ], [ %ring_cursor.sroa.0.2.i230, %bb15.i61.loopexit.loopexit ], !dbg !9226
  %main_cursor.sroa.0.1.i75.lcssa = phi i32 [ %main_cursor.sroa.0.0.i6318133, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i72 ], [ %main_cursor.sroa.0.2.i233, %bb15.i61.loopexit.loopexit ], !dbg !9227
  %_162.not.i66 = icmp eq i32 %1233, 0, !dbg !9207
  %indvars.iv.next20521 = add i32 %indvars.iv20520, -32, !dbg !9207
  br i1 %_162.not.i66, label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb44.i67, !dbg !9207

bb44.i67:                                         ; preds = %bb44.i67.lr.ph, %bb15.i61.loopexit
  %indvars.iv20520 = phi i32 [ %frames, %bb44.i67.lr.ph ], [ %indvars.iv.next20521, %bb15.i61.loopexit ]
  %iter2.sroa.0.0.i6518135 = phi i32 [ %yield_count.sroa.0.0.i6547, %bb44.i67.lr.ph ], [ %1233, %bb15.i61.loopexit ]
  %iter1.sroa.0.0.i6418134 = phi i32 [ 0, %bb44.i67.lr.ph ], [ %1232, %bb15.i61.loopexit ]
  %main_cursor.sroa.0.0.i6318133 = phi i32 [ %_36.i54, %bb44.i67.lr.ph ], [ %main_cursor.sroa.0.1.i75.lcssa, %bb15.i61.loopexit ]
  %ring_cursor.sroa.0.0.i6218132 = phi i32 [ %_37.i55, %bb44.i67.lr.ph ], [ %ring_cursor.sroa.0.1.i74.lcssa, %bb15.i61.loopexit ]
  %umin20548 = call i32 @llvm.umin.i32(i32 %indvars.iv20520, i32 32), !dbg !9228
  %umax20526 = call i32 @llvm.umax.i32(i32 %umin20548, i32 1), !dbg !9228
  %1232 = add i32 %iter1.sroa.0.0.i6418134, 32, !dbg !9228
  %1233 = add nsw i32 %iter2.sroa.0.0.i6518135, -1, !dbg !9232
  %1234 = sub i32 %frames, %iter1.sroa.0.0.i6418134, !dbg !9233
  %spec.store.select.i68 = tail call i32 @llvm.umin.i32(i32 %1234, i32 32), !dbg !9235
  %history.i42.i.sroa.0.0.copyload = load <4 x i32>, ptr %hot_left.i44, align 16, !dbg !9240
  %history.i42.i.sroa.7.0.copyload = load <4 x i32>, ptr %history.i42.i.sroa.7.0.hot_left.i44.sroa_idx, align 16, !dbg !9240
  %history.i42.i.sroa.10.0.copyload = load <4 x i32>, ptr %history.i42.i.sroa.10.0.hot_left.i44.sroa_idx, align 16, !dbg !9240
  %history.i42.i.sroa.13.0.copyload = load <4 x i32>, ptr %history.i42.i.sroa.13.0.hot_left.i44.sroa_idx, align 16, !dbg !9240
  %history.i42.i.sroa.16.0.copyload = load <4 x i32>, ptr %history.i42.i.sroa.16.0.hot_left.i44.sroa_idx, align 16, !dbg !9240
  %history.i42.i.sroa.19.0.copyload = load <4 x i32>, ptr %history.i42.i.sroa.19.0.hot_left.i44.sroa_idx, align 16, !dbg !9240
  %history.i42.i.sroa.22.0.copyload = load <4 x i32>, ptr %history.i42.i.sroa.22.0.hot_left.i44.sroa_idx, align 16, !dbg !9240
  %history.i42.i.sroa.26.0.copyload = load <4 x i32>, ptr %history.i42.i.sroa.26.0.hot_left.i44.sroa_idx, align 16, !dbg !9240
  %history.i42.i.sroa.29.0.copyload = load <4 x i32>, ptr %history.i42.i.sroa.29.0.hot_left.i44.sroa_idx, align 16, !dbg !9240
  %history.i42.i.sroa.32.0.copyload = load <4 x i32>, ptr %history.i42.i.sroa.32.0.hot_left.i44.sroa_idx, align 16, !dbg !9240
  %history.i42.i.sroa.35.0.copyload = load <4 x i32>, ptr %history.i42.i.sroa.35.0.hot_left.i44.sroa_idx, align 16, !dbg !9240
  %history.i42.i.sroa.38.0.copyload = load <4 x i32>, ptr %history.i42.i.sroa.38.0.hot_left.i44.sroa_idx, align 16, !dbg !9240
  %_20.i45.i18064.not = icmp eq i32 %frames, %iter1.sroa.0.0.i6418134, !dbg !9243
  br i1 %_20.i45.i18064.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit250.i, label %bb5.i46.i.lr.ph, !dbg !9247

bb5.i46.i.lr.ph:                                  ; preds = %bb44.i67
  %_11.i.i.i67.i15454 = load <4 x float>, ptr %_31, align 16
  %_14.i.i.i70.i15455 = load <4 x float>, ptr %1162, align 16
  %_17.i.i.i73.i15456 = load <4 x float>, ptr %1163, align 16
  %_20.i.i.i76.i15457 = load <4 x float>, ptr %1164, align 16
  %_25.i.i.i81.i15458 = load <4 x float>, ptr %row1.i.i.i79.i, align 16
  %_28.i.i.i84.i15459 = load <4 x float>, ptr %1165, align 16
  %_31.i.i.i87.i15460 = load <4 x float>, ptr %1166, align 16
  %_34.i.i.i90.i15461 = load <4 x float>, ptr %1167, align 16
  %_39.i.i.i95.i15462 = load <4 x float>, ptr %row3.i.i.i93.i, align 16
  %_42.i.i.i98.i15463 = load <4 x float>, ptr %1168, align 16
  %_45.i.i.i101.i15464 = load <4 x float>, ptr %1169, align 16
  %_48.i.i.i104.i15465 = load <4 x float>, ptr %1170, align 16
  %_53.i.i.i109.i15466 = load <4 x float>, ptr %row5.i.i.i107.i, align 16
  %_56.i.i.i112.i15467 = load <4 x float>, ptr %1171, align 16
  %_59.i.i.i115.i15468 = load <4 x float>, ptr %1172, align 16
  %_62.i.i.i118.i15469 = load <4 x float>, ptr %1173, align 16
  %_67.i.i.i123.i15470 = load <4 x float>, ptr %row7.i.i.i121.i, align 16
  %_70.i.i.i126.i15471 = load <4 x float>, ptr %1174, align 16
  %_73.i.i.i129.i15472 = load <4 x float>, ptr %1175, align 16
  %_76.i.i.i132.i15473 = load <4 x float>, ptr %1176, align 16
  %_81.i.i.i137.i15474 = load <4 x float>, ptr %row9.i.i.i135.i, align 16
  %_84.i.i.i140.i15475 = load <4 x float>, ptr %1177, align 16
  %_87.i.i.i143.i15476 = load <4 x float>, ptr %1178, align 16
  %_90.i.i.i146.i15477 = load <4 x float>, ptr %1179, align 16
  %_95.i.i.i151.i15478 = load <4 x float>, ptr %row11.i.i.i149.i, align 16
  %_98.i.i.i154.i15479 = load <4 x float>, ptr %1180, align 16
  %_101.i.i.i157.i15480 = load <4 x float>, ptr %1181, align 16
  %_104.i.i.i160.i15481 = load <4 x float>, ptr %1182, align 16
  %_109.i.i.i165.i15482 = load <4 x float>, ptr %row13.i.i.i163.i, align 16
  %_112.i.i.i168.i15483 = load <4 x float>, ptr %1183, align 16
  %_115.i.i.i171.i15484 = load <4 x float>, ptr %1184, align 16
  %_118.i.i.i174.i15485 = load <4 x float>, ptr %1185, align 16
  %_123.i.i.i179.i15486 = load <4 x float>, ptr %row15.i.i.i177.i, align 16
  %_126.i.i.i182.i15487 = load <4 x float>, ptr %1186, align 16
  %_129.i.i.i185.i15488 = load <4 x float>, ptr %1187, align 16
  %_132.i.i.i188.i15489 = load <4 x float>, ptr %1188, align 16
  %_137.i.i.i193.i15490 = load <4 x float>, ptr %row17.i.i.i191.i, align 16
  %_140.i.i.i196.i15491 = load <4 x float>, ptr %1189, align 16
  %_143.i.i.i199.i15492 = load <4 x float>, ptr %1190, align 16
  %_146.i.i.i202.i15493 = load <4 x float>, ptr %1191, align 16
  %_151.i.i.i207.i15494 = load <4 x float>, ptr %row19.i.i.i205.i, align 16
  %_154.i.i.i210.i15495 = load <4 x float>, ptr %1192, align 16
  %_157.i.i.i213.i15496 = load <4 x float>, ptr %1193, align 16
  %_160.i.i.i216.i15497 = load <4 x float>, ptr %1194, align 16
  %_165.i.i.i221.i15498 = load <4 x float>, ptr %row21.i.i.i219.i, align 16
  %_168.i.i.i224.i15499 = load <4 x float>, ptr %1195, align 16
  %_171.i.i.i227.i15500 = load <4 x float>, ptr %1196, align 16
  %_174.i.i.i230.i15501 = load <4 x float>, ptr %1197, align 16
  br label %bb5.i46.i, !dbg !9247

bb5.i46.i:                                        ; preds = %bb5.i46.i.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992
  %iter.sroa.0.0.i44.i18076 = phi i32 [ 0, %bb5.i46.i.lr.ph ], [ %1235, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992 ]
  %history.i42.i.sroa.35.018075 = phi <4 x i32> [ %history.i42.i.sroa.35.0.copyload, %bb5.i46.i.lr.ph ], [ %history.i42.i.sroa.32.018074, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992 ]
  %history.i42.i.sroa.32.018074 = phi <4 x i32> [ %history.i42.i.sroa.32.0.copyload, %bb5.i46.i.lr.ph ], [ %history.i42.i.sroa.29.018073, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992 ]
  %history.i42.i.sroa.29.018073 = phi <4 x i32> [ %history.i42.i.sroa.29.0.copyload, %bb5.i46.i.lr.ph ], [ %history.i42.i.sroa.26.018072, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992 ]
  %history.i42.i.sroa.26.018072 = phi <4 x i32> [ %history.i42.i.sroa.26.0.copyload, %bb5.i46.i.lr.ph ], [ %history.i42.i.sroa.22.018071, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992 ]
  %history.i42.i.sroa.22.018071 = phi <4 x i32> [ %history.i42.i.sroa.22.0.copyload, %bb5.i46.i.lr.ph ], [ %history.i42.i.sroa.19.018070, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992 ]
  %history.i42.i.sroa.19.018070 = phi <4 x i32> [ %history.i42.i.sroa.19.0.copyload, %bb5.i46.i.lr.ph ], [ %history.i42.i.sroa.16.018069, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992 ]
  %history.i42.i.sroa.16.018069 = phi <4 x i32> [ %history.i42.i.sroa.16.0.copyload, %bb5.i46.i.lr.ph ], [ %history.i42.i.sroa.13.018068, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992 ]
  %history.i42.i.sroa.13.018068 = phi <4 x i32> [ %history.i42.i.sroa.13.0.copyload, %bb5.i46.i.lr.ph ], [ %history.i42.i.sroa.10.018067, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992 ]
  %history.i42.i.sroa.10.018067 = phi <4 x i32> [ %history.i42.i.sroa.10.0.copyload, %bb5.i46.i.lr.ph ], [ %history.i42.i.sroa.7.018066, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992 ]
  %history.i42.i.sroa.7.018066 = phi <4 x i32> [ %history.i42.i.sroa.7.0.copyload, %bb5.i46.i.lr.ph ], [ %history.i42.i.sroa.0.018065, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992 ]
  %history.i42.i.sroa.0.018065 = phi <4 x i32> [ %history.i42.i.sroa.0.0.copyload, %bb5.i46.i.lr.ph ], [ %lanes.i5311.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992 ]
  %1235 = add nuw nsw i32 %iter.sroa.0.0.i44.i18076, 1, !dbg !9248
  %_11.i47.i = add nuw nsw i32 %iter.sroa.0.0.i44.i18076, %iter1.sroa.0.0.i6418134, !dbg !9251
  %base.i48.i = shl i32 %_11.i47.i, 2, !dbg !9251
  %_24.i49.i = icmp ugt i32 %base.i48.i, %left_io.1, !dbg !9252
  br i1 %_24.i49.i, label %bb7.i249.i, label %bb8.i50.i, !dbg !9252, !prof !902

bb8.i50.i:                                        ; preds = %bb5.i46.i
  %_27.i51.i = sub nuw nsw i32 %left_io.1, %base.i48.i, !dbg !9255
  %_8.i5314 = icmp samesign ugt i32 %_27.i51.i, 3, !dbg !9256
  br i1 %_8.i5314, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992, label %bb2.i5315, !dbg !9256, !prof !1153

bb2.i5315:                                        ; preds = %bb8.i50.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_27.i51.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !9261, !noalias !9262
  unreachable, !dbg !9261

bb7.i249.i:                                       ; preds = %bb5.i46.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i48.i, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #33, !dbg !9269, !noalias !9270
  unreachable, !dbg !9269

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992: ; preds = %bb8.i50.i
  %_31.i52.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %base.i48.i, !dbg !9271
  %lanes.i5311.sroa.0.0.copyload = load <4 x i32>, ptr %_31.i52.i, align 4, !dbg !9273, !alias.scope !9277, !noalias !9281
  %1236 = bitcast <4 x i32> %history.i42.i.sroa.19.018070 to <4 x float>, !dbg !9283
  %1237 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1236), !dbg !9288
  %1238 = bitcast <4 x i32> %lanes.i5311.sroa.0.0.copyload to <4 x float>, !dbg !9289
  %1239 = fmul <4 x float> %_11.i.i.i67.i15454, %1238, !dbg !9294
  %1240 = fadd <4 x float> %1239, zeroinitializer, !dbg !9295
  %1241 = fmul <4 x float> %_14.i.i.i70.i15455, %1238, !dbg !9299
  %1242 = fadd <4 x float> %1241, zeroinitializer, !dbg !9303
  %1243 = fmul <4 x float> %_17.i.i.i73.i15456, %1238, !dbg !9307
  %1244 = fadd <4 x float> %1243, zeroinitializer, !dbg !9311
  %1245 = fmul <4 x float> %_20.i.i.i76.i15457, %1238, !dbg !9315
  %1246 = fadd <4 x float> %1245, zeroinitializer, !dbg !9319
  %1247 = bitcast <4 x i32> %history.i42.i.sroa.0.018065 to <4 x float>, !dbg !9323
  %1248 = fmul <4 x float> %_25.i.i.i81.i15458, %1247, !dbg !9327
  %1249 = fadd <4 x float> %1240, %1248, !dbg !9328
  %1250 = fmul <4 x float> %_28.i.i.i84.i15459, %1247, !dbg !9332
  %1251 = fadd <4 x float> %1242, %1250, !dbg !9336
  %1252 = fmul <4 x float> %_31.i.i.i87.i15460, %1247, !dbg !9340
  %1253 = fadd <4 x float> %1244, %1252, !dbg !9344
  %1254 = fmul <4 x float> %_34.i.i.i90.i15461, %1247, !dbg !9348
  %1255 = fadd <4 x float> %1246, %1254, !dbg !9352
  %1256 = bitcast <4 x i32> %history.i42.i.sroa.7.018066 to <4 x float>, !dbg !9356
  %1257 = fmul <4 x float> %_39.i.i.i95.i15462, %1256, !dbg !9360
  %1258 = fadd <4 x float> %1249, %1257, !dbg !9361
  %1259 = fmul <4 x float> %_42.i.i.i98.i15463, %1256, !dbg !9365
  %1260 = fadd <4 x float> %1251, %1259, !dbg !9369
  %1261 = fmul <4 x float> %_45.i.i.i101.i15464, %1256, !dbg !9373
  %1262 = fadd <4 x float> %1253, %1261, !dbg !9377
  %1263 = fmul <4 x float> %_48.i.i.i104.i15465, %1256, !dbg !9381
  %1264 = fadd <4 x float> %1255, %1263, !dbg !9385
  %1265 = bitcast <4 x i32> %history.i42.i.sroa.10.018067 to <4 x float>, !dbg !9389
  %1266 = fmul <4 x float> %_53.i.i.i109.i15466, %1265, !dbg !9393
  %1267 = fadd <4 x float> %1258, %1266, !dbg !9394
  %1268 = fmul <4 x float> %_56.i.i.i112.i15467, %1265, !dbg !9398
  %1269 = fadd <4 x float> %1260, %1268, !dbg !9402
  %1270 = fmul <4 x float> %_59.i.i.i115.i15468, %1265, !dbg !9406
  %1271 = fadd <4 x float> %1262, %1270, !dbg !9410
  %1272 = fmul <4 x float> %_62.i.i.i118.i15469, %1265, !dbg !9414
  %1273 = fadd <4 x float> %1264, %1272, !dbg !9418
  %1274 = bitcast <4 x i32> %history.i42.i.sroa.13.018068 to <4 x float>, !dbg !9422
  %1275 = fmul <4 x float> %_67.i.i.i123.i15470, %1274, !dbg !9426
  %1276 = fadd <4 x float> %1267, %1275, !dbg !9427
  %1277 = fmul <4 x float> %_70.i.i.i126.i15471, %1274, !dbg !9431
  %1278 = fadd <4 x float> %1269, %1277, !dbg !9435
  %1279 = fmul <4 x float> %_73.i.i.i129.i15472, %1274, !dbg !9439
  %1280 = fadd <4 x float> %1271, %1279, !dbg !9443
  %1281 = fmul <4 x float> %_76.i.i.i132.i15473, %1274, !dbg !9447
  %1282 = fadd <4 x float> %1273, %1281, !dbg !9451
  %1283 = bitcast <4 x i32> %history.i42.i.sroa.16.018069 to <4 x float>, !dbg !9455
  %1284 = fmul <4 x float> %_81.i.i.i137.i15474, %1283, !dbg !9459
  %1285 = fadd <4 x float> %1276, %1284, !dbg !9460
  %1286 = fmul <4 x float> %_84.i.i.i140.i15475, %1283, !dbg !9464
  %1287 = fadd <4 x float> %1278, %1286, !dbg !9468
  %1288 = fmul <4 x float> %_87.i.i.i143.i15476, %1283, !dbg !9472
  %1289 = fadd <4 x float> %1280, %1288, !dbg !9476
  %1290 = fmul <4 x float> %_90.i.i.i146.i15477, %1283, !dbg !9480
  %1291 = fadd <4 x float> %1282, %1290, !dbg !9484
  %1292 = fmul <4 x float> %_95.i.i.i151.i15478, %1236, !dbg !9488
  %1293 = fadd <4 x float> %1285, %1292, !dbg !9492
  %1294 = fmul <4 x float> %_98.i.i.i154.i15479, %1236, !dbg !9496
  %1295 = fadd <4 x float> %1287, %1294, !dbg !9500
  %1296 = fmul <4 x float> %_101.i.i.i157.i15480, %1236, !dbg !9504
  %1297 = fadd <4 x float> %1289, %1296, !dbg !9508
  %1298 = fmul <4 x float> %_104.i.i.i160.i15481, %1236, !dbg !9512
  %1299 = fadd <4 x float> %1291, %1298, !dbg !9516
  %1300 = bitcast <4 x i32> %history.i42.i.sroa.22.018071 to <4 x float>, !dbg !9520
  %1301 = fmul <4 x float> %_109.i.i.i165.i15482, %1300, !dbg !9524
  %1302 = fadd <4 x float> %1293, %1301, !dbg !9525
  %1303 = fmul <4 x float> %_112.i.i.i168.i15483, %1300, !dbg !9529
  %1304 = fadd <4 x float> %1295, %1303, !dbg !9533
  %1305 = fmul <4 x float> %_115.i.i.i171.i15484, %1300, !dbg !9537
  %1306 = fadd <4 x float> %1297, %1305, !dbg !9541
  %1307 = fmul <4 x float> %_118.i.i.i174.i15485, %1300, !dbg !9545
  %1308 = fadd <4 x float> %1299, %1307, !dbg !9549
  %1309 = bitcast <4 x i32> %history.i42.i.sroa.26.018072 to <4 x float>, !dbg !9553
  %1310 = fmul <4 x float> %_123.i.i.i179.i15486, %1309, !dbg !9557
  %1311 = fadd <4 x float> %1302, %1310, !dbg !9558
  %1312 = fmul <4 x float> %_126.i.i.i182.i15487, %1309, !dbg !9562
  %1313 = fadd <4 x float> %1304, %1312, !dbg !9566
  %1314 = fmul <4 x float> %_129.i.i.i185.i15488, %1309, !dbg !9570
  %1315 = fadd <4 x float> %1306, %1314, !dbg !9574
  %1316 = fmul <4 x float> %_132.i.i.i188.i15489, %1309, !dbg !9578
  %1317 = fadd <4 x float> %1308, %1316, !dbg !9582
  %1318 = bitcast <4 x i32> %history.i42.i.sroa.29.018073 to <4 x float>, !dbg !9586
  %1319 = fmul <4 x float> %_137.i.i.i193.i15490, %1318, !dbg !9590
  %1320 = fadd <4 x float> %1311, %1319, !dbg !9591
  %1321 = fmul <4 x float> %_140.i.i.i196.i15491, %1318, !dbg !9595
  %1322 = fadd <4 x float> %1313, %1321, !dbg !9599
  %1323 = fmul <4 x float> %_143.i.i.i199.i15492, %1318, !dbg !9603
  %1324 = fadd <4 x float> %1315, %1323, !dbg !9607
  %1325 = fmul <4 x float> %_146.i.i.i202.i15493, %1318, !dbg !9611
  %1326 = fadd <4 x float> %1317, %1325, !dbg !9615
  %1327 = bitcast <4 x i32> %history.i42.i.sroa.32.018074 to <4 x float>, !dbg !9619
  %1328 = fmul <4 x float> %_151.i.i.i207.i15494, %1327, !dbg !9623
  %1329 = fadd <4 x float> %1320, %1328, !dbg !9624
  %1330 = fmul <4 x float> %_154.i.i.i210.i15495, %1327, !dbg !9628
  %1331 = fadd <4 x float> %1322, %1330, !dbg !9632
  %1332 = fmul <4 x float> %_157.i.i.i213.i15496, %1327, !dbg !9636
  %1333 = fadd <4 x float> %1324, %1332, !dbg !9640
  %1334 = fmul <4 x float> %_160.i.i.i216.i15497, %1327, !dbg !9644
  %1335 = fadd <4 x float> %1326, %1334, !dbg !9648
  %1336 = bitcast <4 x i32> %history.i42.i.sroa.35.018075 to <4 x float>, !dbg !9652
  %1337 = fmul <4 x float> %_165.i.i.i221.i15498, %1336, !dbg !9656
  %1338 = fadd <4 x float> %1329, %1337, !dbg !9657
  %1339 = fmul <4 x float> %_168.i.i.i224.i15499, %1336, !dbg !9661
  %1340 = fadd <4 x float> %1331, %1339, !dbg !9665
  %1341 = fmul <4 x float> %_171.i.i.i227.i15500, %1336, !dbg !9669
  %1342 = fadd <4 x float> %1333, %1341, !dbg !9673
  %1343 = fmul <4 x float> %_174.i.i.i230.i15501, %1336, !dbg !9677
  %1344 = fadd <4 x float> %1335, %1343, !dbg !9681
  %1345 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1338), !dbg !9685
  %1346 = fcmp olt <4 x float> %1345, %1237, !dbg !9689
  %1347 = select <4 x i1> %1346, <4 x float> %1237, <4 x float> %1345, !dbg !9693
  %1348 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1340), !dbg !9685
  %1349 = fcmp olt <4 x float> %1348, %1347, !dbg !9689
  %1350 = select <4 x i1> %1349, <4 x float> %1347, <4 x float> %1348, !dbg !9693
  %1351 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1342), !dbg !9685
  %1352 = fcmp olt <4 x float> %1351, %1350, !dbg !9689
  %1353 = select <4 x i1> %1352, <4 x float> %1350, <4 x float> %1351, !dbg !9693
  %1354 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1344), !dbg !9685
  %1355 = fcmp olt <4 x float> %1354, %1353, !dbg !9689
  %1356 = select <4 x i1> %1355, <4 x float> %1353, <4 x float> %1354, !dbg !9693
  %_39.i243.i.idx = shl i32 %iter.sroa.0.0.i44.i18076, 4, !dbg !9694
  %_39.i243.i = getelementptr inbounds nuw i8, ptr %peaks_left.i41, i32 %_39.i243.i.idx, !dbg !9694
  store <4 x float> %1356, ptr %_39.i243.i, align 4, !dbg !9699, !alias.scope !9704, !noalias !9708
  %exitcond20524.not = icmp eq i32 %1235, %umax20526, !dbg !9243
  br i1 %exitcond20524.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit250.i, label %bb5.i46.i, !dbg !9247

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit250.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992, %bb44.i67
  %history.i42.i.sroa.0.0.lcssa = phi <4 x i32> [ %history.i42.i.sroa.0.0.copyload, %bb44.i67 ], [ %lanes.i5311.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992 ], !dbg !9712
  %history.i42.i.sroa.7.0.lcssa = phi <4 x i32> [ %history.i42.i.sroa.7.0.copyload, %bb44.i67 ], [ %history.i42.i.sroa.0.018065, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992 ], !dbg !9712
  %history.i42.i.sroa.10.0.lcssa = phi <4 x i32> [ %history.i42.i.sroa.10.0.copyload, %bb44.i67 ], [ %history.i42.i.sroa.7.018066, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992 ], !dbg !9712
  %history.i42.i.sroa.13.0.lcssa = phi <4 x i32> [ %history.i42.i.sroa.13.0.copyload, %bb44.i67 ], [ %history.i42.i.sroa.10.018067, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992 ], !dbg !9712
  %history.i42.i.sroa.16.0.lcssa = phi <4 x i32> [ %history.i42.i.sroa.16.0.copyload, %bb44.i67 ], [ %history.i42.i.sroa.13.018068, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992 ], !dbg !9712
  %history.i42.i.sroa.19.0.lcssa = phi <4 x i32> [ %history.i42.i.sroa.19.0.copyload, %bb44.i67 ], [ %history.i42.i.sroa.16.018069, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992 ], !dbg !9712
  %history.i42.i.sroa.22.0.lcssa = phi <4 x i32> [ %history.i42.i.sroa.22.0.copyload, %bb44.i67 ], [ %history.i42.i.sroa.19.018070, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992 ], !dbg !9712
  %history.i42.i.sroa.26.0.lcssa = phi <4 x i32> [ %history.i42.i.sroa.26.0.copyload, %bb44.i67 ], [ %history.i42.i.sroa.22.018071, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992 ], !dbg !9712
  %history.i42.i.sroa.29.0.lcssa = phi <4 x i32> [ %history.i42.i.sroa.29.0.copyload, %bb44.i67 ], [ %history.i42.i.sroa.26.018072, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992 ], !dbg !9712
  %history.i42.i.sroa.32.0.lcssa = phi <4 x i32> [ %history.i42.i.sroa.32.0.copyload, %bb44.i67 ], [ %history.i42.i.sroa.29.018073, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992 ], !dbg !9712
  %history.i42.i.sroa.35.0.lcssa = phi <4 x i32> [ %history.i42.i.sroa.35.0.copyload, %bb44.i67 ], [ %history.i42.i.sroa.32.018074, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992 ], !dbg !9712
  %history.i42.i.sroa.38.0.lcssa = phi <4 x i32> [ %history.i42.i.sroa.38.0.copyload, %bb44.i67 ], [ %history.i42.i.sroa.35.018075, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5992 ], !dbg !9712
  store <4 x i32> %history.i42.i.sroa.0.0.lcssa, ptr %hot_left.i44, align 16, !dbg !9713
  store <4 x i32> %history.i42.i.sroa.7.0.lcssa, ptr %history.i42.i.sroa.7.0.hot_left.i44.sroa_idx, align 16, !dbg !9713
  store <4 x i32> %history.i42.i.sroa.10.0.lcssa, ptr %history.i42.i.sroa.10.0.hot_left.i44.sroa_idx, align 16, !dbg !9713
  store <4 x i32> %history.i42.i.sroa.13.0.lcssa, ptr %history.i42.i.sroa.13.0.hot_left.i44.sroa_idx, align 16, !dbg !9713
  store <4 x i32> %history.i42.i.sroa.16.0.lcssa, ptr %history.i42.i.sroa.16.0.hot_left.i44.sroa_idx, align 16, !dbg !9713
  store <4 x i32> %history.i42.i.sroa.19.0.lcssa, ptr %history.i42.i.sroa.19.0.hot_left.i44.sroa_idx, align 16, !dbg !9713
  store <4 x i32> %history.i42.i.sroa.22.0.lcssa, ptr %history.i42.i.sroa.22.0.hot_left.i44.sroa_idx, align 16, !dbg !9713
  store <4 x i32> %history.i42.i.sroa.26.0.lcssa, ptr %history.i42.i.sroa.26.0.hot_left.i44.sroa_idx, align 16, !dbg !9713
  store <4 x i32> %history.i42.i.sroa.29.0.lcssa, ptr %history.i42.i.sroa.29.0.hot_left.i44.sroa_idx, align 16, !dbg !9713
  store <4 x i32> %history.i42.i.sroa.32.0.lcssa, ptr %history.i42.i.sroa.32.0.hot_left.i44.sroa_idx, align 16, !dbg !9713
  store <4 x i32> %history.i42.i.sroa.35.0.lcssa, ptr %history.i42.i.sroa.35.0.hot_left.i44.sroa_idx, align 16, !dbg !9713
  store <4 x i32> %history.i42.i.sroa.38.0.lcssa, ptr %history.i42.i.sroa.38.0.hot_left.i44.sroa_idx, align 16, !dbg !9713
  %history.i.i18.sroa.0.0.copyload = load <4 x i32>, ptr %hot_right.i43, align 16, !dbg !9714
  %history.i.i18.sroa.7.0.copyload = load <4 x i32>, ptr %history.i.i18.sroa.7.0.hot_right.i43.sroa_idx, align 16, !dbg !9714
  %history.i.i18.sroa.10.0.copyload = load <4 x i32>, ptr %history.i.i18.sroa.10.0.hot_right.i43.sroa_idx, align 16, !dbg !9714
  %history.i.i18.sroa.13.0.copyload = load <4 x i32>, ptr %history.i.i18.sroa.13.0.hot_right.i43.sroa_idx, align 16, !dbg !9714
  %history.i.i18.sroa.16.0.copyload = load <4 x i32>, ptr %history.i.i18.sroa.16.0.hot_right.i43.sroa_idx, align 16, !dbg !9714
  %history.i.i18.sroa.19.0.copyload = load <4 x i32>, ptr %history.i.i18.sroa.19.0.hot_right.i43.sroa_idx, align 16, !dbg !9714
  %history.i.i18.sroa.22.0.copyload = load <4 x i32>, ptr %history.i.i18.sroa.22.0.hot_right.i43.sroa_idx, align 16, !dbg !9714
  %history.i.i18.sroa.26.0.copyload = load <4 x i32>, ptr %history.i.i18.sroa.26.0.hot_right.i43.sroa_idx, align 16, !dbg !9714
  %history.i.i18.sroa.29.0.copyload = load <4 x i32>, ptr %history.i.i18.sroa.29.0.hot_right.i43.sroa_idx, align 16, !dbg !9714
  %history.i.i18.sroa.32.0.copyload = load <4 x i32>, ptr %history.i.i18.sroa.32.0.hot_right.i43.sroa_idx, align 16, !dbg !9714
  %history.i.i18.sroa.35.0.copyload = load <4 x i32>, ptr %history.i.i18.sroa.35.0.hot_right.i43.sroa_idx, align 16, !dbg !9714
  %history.i.i18.sroa.38.0.copyload = load <4 x i32>, ptr %history.i.i18.sroa.38.0.hot_right.i43.sroa_idx, align 16, !dbg !9714
  br i1 %_20.i45.i18064.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i72, label %bb5.i.i236.lr.ph, !dbg !9716

bb5.i.i236.lr.ph:                                 ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit250.i
  %_11.i.i.i.i25615401 = load <4 x float>, ptr %_31, align 16
  %_14.i.i.i.i25915402 = load <4 x float>, ptr %1162, align 16
  %_17.i.i.i.i26215403 = load <4 x float>, ptr %1163, align 16
  %_20.i.i.i.i26515404 = load <4 x float>, ptr %1164, align 16
  %_25.i.i.i.i27015405 = load <4 x float>, ptr %row1.i.i.i79.i, align 16
  %_28.i.i.i.i27315406 = load <4 x float>, ptr %1165, align 16
  %_31.i.i.i.i27615407 = load <4 x float>, ptr %1166, align 16
  %_34.i.i.i.i27915408 = load <4 x float>, ptr %1167, align 16
  %_39.i.i.i.i28415409 = load <4 x float>, ptr %row3.i.i.i93.i, align 16
  %_42.i.i.i.i28715410 = load <4 x float>, ptr %1168, align 16
  %_45.i.i.i.i29015411 = load <4 x float>, ptr %1169, align 16
  %_48.i.i.i.i29315412 = load <4 x float>, ptr %1170, align 16
  %_53.i.i.i.i29815413 = load <4 x float>, ptr %row5.i.i.i107.i, align 16
  %_56.i.i.i.i30115414 = load <4 x float>, ptr %1171, align 16
  %_59.i.i.i.i30415415 = load <4 x float>, ptr %1172, align 16
  %_62.i.i.i.i30715416 = load <4 x float>, ptr %1173, align 16
  %_67.i.i.i.i31215417 = load <4 x float>, ptr %row7.i.i.i121.i, align 16
  %_70.i.i.i.i31515418 = load <4 x float>, ptr %1174, align 16
  %_73.i.i.i.i31815419 = load <4 x float>, ptr %1175, align 16
  %_76.i.i.i.i32115420 = load <4 x float>, ptr %1176, align 16
  %_81.i.i.i.i32615421 = load <4 x float>, ptr %row9.i.i.i135.i, align 16
  %_84.i.i.i.i32915422 = load <4 x float>, ptr %1177, align 16
  %_87.i.i.i.i33215423 = load <4 x float>, ptr %1178, align 16
  %_90.i.i.i.i33515424 = load <4 x float>, ptr %1179, align 16
  %_95.i.i.i.i34015425 = load <4 x float>, ptr %row11.i.i.i149.i, align 16
  %_98.i.i.i.i34315426 = load <4 x float>, ptr %1180, align 16
  %_101.i.i.i.i34615427 = load <4 x float>, ptr %1181, align 16
  %_104.i.i.i.i34915428 = load <4 x float>, ptr %1182, align 16
  %_109.i.i.i.i35415429 = load <4 x float>, ptr %row13.i.i.i163.i, align 16
  %_112.i.i.i.i35715430 = load <4 x float>, ptr %1183, align 16
  %_115.i.i.i.i36015431 = load <4 x float>, ptr %1184, align 16
  %_118.i.i.i.i36315432 = load <4 x float>, ptr %1185, align 16
  %_123.i.i.i.i36815433 = load <4 x float>, ptr %row15.i.i.i177.i, align 16
  %_126.i.i.i.i37115434 = load <4 x float>, ptr %1186, align 16
  %_129.i.i.i.i37415435 = load <4 x float>, ptr %1187, align 16
  %_132.i.i.i.i37715436 = load <4 x float>, ptr %1188, align 16
  %_137.i.i.i.i38215437 = load <4 x float>, ptr %row17.i.i.i191.i, align 16
  %_140.i.i.i.i38515438 = load <4 x float>, ptr %1189, align 16
  %_143.i.i.i.i38815439 = load <4 x float>, ptr %1190, align 16
  %_146.i.i.i.i39115440 = load <4 x float>, ptr %1191, align 16
  %_151.i.i.i.i39615441 = load <4 x float>, ptr %row19.i.i.i205.i, align 16
  %_154.i.i.i.i39915442 = load <4 x float>, ptr %1192, align 16
  %_157.i.i.i.i40215443 = load <4 x float>, ptr %1193, align 16
  %_160.i.i.i.i40515444 = load <4 x float>, ptr %1194, align 16
  %_165.i.i.i.i41015445 = load <4 x float>, ptr %row21.i.i.i219.i, align 16
  %_168.i.i.i.i41315446 = load <4 x float>, ptr %1195, align 16
  %_171.i.i.i.i41615447 = load <4 x float>, ptr %1196, align 16
  %_174.i.i.i.i41915448 = load <4 x float>, ptr %1197, align 16
  br label %bb5.i.i236, !dbg !9716

bb5.i.i236:                                       ; preds = %bb5.i.i236.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997
  %iter.sroa.0.0.i.i7018103 = phi i32 [ 0, %bb5.i.i236.lr.ph ], [ %1357, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997 ]
  %history.i.i18.sroa.35.018102 = phi <4 x i32> [ %history.i.i18.sroa.35.0.copyload, %bb5.i.i236.lr.ph ], [ %history.i.i18.sroa.32.018101, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997 ]
  %history.i.i18.sroa.32.018101 = phi <4 x i32> [ %history.i.i18.sroa.32.0.copyload, %bb5.i.i236.lr.ph ], [ %history.i.i18.sroa.29.018100, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997 ]
  %history.i.i18.sroa.29.018100 = phi <4 x i32> [ %history.i.i18.sroa.29.0.copyload, %bb5.i.i236.lr.ph ], [ %history.i.i18.sroa.26.018099, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997 ]
  %history.i.i18.sroa.26.018099 = phi <4 x i32> [ %history.i.i18.sroa.26.0.copyload, %bb5.i.i236.lr.ph ], [ %history.i.i18.sroa.22.018098, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997 ]
  %history.i.i18.sroa.22.018098 = phi <4 x i32> [ %history.i.i18.sroa.22.0.copyload, %bb5.i.i236.lr.ph ], [ %history.i.i18.sroa.19.018097, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997 ]
  %history.i.i18.sroa.19.018097 = phi <4 x i32> [ %history.i.i18.sroa.19.0.copyload, %bb5.i.i236.lr.ph ], [ %history.i.i18.sroa.16.018096, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997 ]
  %history.i.i18.sroa.16.018096 = phi <4 x i32> [ %history.i.i18.sroa.16.0.copyload, %bb5.i.i236.lr.ph ], [ %history.i.i18.sroa.13.018095, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997 ]
  %history.i.i18.sroa.13.018095 = phi <4 x i32> [ %history.i.i18.sroa.13.0.copyload, %bb5.i.i236.lr.ph ], [ %history.i.i18.sroa.10.018094, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997 ]
  %history.i.i18.sroa.10.018094 = phi <4 x i32> [ %history.i.i18.sroa.10.0.copyload, %bb5.i.i236.lr.ph ], [ %history.i.i18.sroa.7.018093, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997 ]
  %history.i.i18.sroa.7.018093 = phi <4 x i32> [ %history.i.i18.sroa.7.0.copyload, %bb5.i.i236.lr.ph ], [ %history.i.i18.sroa.0.018092, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997 ]
  %history.i.i18.sroa.0.018092 = phi <4 x i32> [ %history.i.i18.sroa.0.0.copyload, %bb5.i.i236.lr.ph ], [ %lanes.i5320.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997 ]
  %1357 = add nuw nsw i32 %iter.sroa.0.0.i.i7018103, 1, !dbg !9719
  %_11.i28.i = add nuw nsw i32 %iter.sroa.0.0.i.i7018103, %iter1.sroa.0.0.i6418134, !dbg !9722
  %base.i.i237 = shl i32 %_11.i28.i, 2, !dbg !9722
  %_24.i.i238 = icmp ugt i32 %base.i.i237, %right_io.1, !dbg !9723
  br i1 %_24.i.i238, label %bb7.i.i438, label %bb8.i.i239, !dbg !9723, !prof !902

bb8.i.i239:                                       ; preds = %bb5.i.i236
  %_27.i.i240 = sub nuw nsw i32 %right_io.1, %base.i.i237, !dbg !9726
  %_8.i5323 = icmp samesign ugt i32 %_27.i.i240, 3, !dbg !9727
  br i1 %_8.i5323, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997, label %bb2.i5324, !dbg !9727, !prof !1153

bb2.i5324:                                        ; preds = %bb8.i.i239
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_27.i.i240, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !9732, !noalias !9733
  unreachable, !dbg !9732

bb7.i.i438:                                       ; preds = %bb5.i.i236
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i.i237, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #33, !dbg !9740, !noalias !9741
  unreachable, !dbg !9740

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997: ; preds = %bb8.i.i239
  %_31.i.i241 = getelementptr inbounds nuw float, ptr %right_io.0, i32 %base.i.i237, !dbg !9742
  %lanes.i5320.sroa.0.0.copyload = load <4 x i32>, ptr %_31.i.i241, align 4, !dbg !9744, !alias.scope !9748, !noalias !9752
  %1358 = bitcast <4 x i32> %history.i.i18.sroa.19.018097 to <4 x float>, !dbg !9754
  %1359 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1358), !dbg !9759
  %1360 = bitcast <4 x i32> %lanes.i5320.sroa.0.0.copyload to <4 x float>, !dbg !9760
  %1361 = fmul <4 x float> %_11.i.i.i.i25615401, %1360, !dbg !9765
  %1362 = fadd <4 x float> %1361, zeroinitializer, !dbg !9766
  %1363 = fmul <4 x float> %_14.i.i.i.i25915402, %1360, !dbg !9770
  %1364 = fadd <4 x float> %1363, zeroinitializer, !dbg !9774
  %1365 = fmul <4 x float> %_17.i.i.i.i26215403, %1360, !dbg !9778
  %1366 = fadd <4 x float> %1365, zeroinitializer, !dbg !9782
  %1367 = fmul <4 x float> %_20.i.i.i.i26515404, %1360, !dbg !9786
  %1368 = fadd <4 x float> %1367, zeroinitializer, !dbg !9790
  %1369 = bitcast <4 x i32> %history.i.i18.sroa.0.018092 to <4 x float>, !dbg !9794
  %1370 = fmul <4 x float> %_25.i.i.i.i27015405, %1369, !dbg !9798
  %1371 = fadd <4 x float> %1362, %1370, !dbg !9799
  %1372 = fmul <4 x float> %_28.i.i.i.i27315406, %1369, !dbg !9803
  %1373 = fadd <4 x float> %1364, %1372, !dbg !9807
  %1374 = fmul <4 x float> %_31.i.i.i.i27615407, %1369, !dbg !9811
  %1375 = fadd <4 x float> %1366, %1374, !dbg !9815
  %1376 = fmul <4 x float> %_34.i.i.i.i27915408, %1369, !dbg !9819
  %1377 = fadd <4 x float> %1368, %1376, !dbg !9823
  %1378 = bitcast <4 x i32> %history.i.i18.sroa.7.018093 to <4 x float>, !dbg !9827
  %1379 = fmul <4 x float> %_39.i.i.i.i28415409, %1378, !dbg !9831
  %1380 = fadd <4 x float> %1371, %1379, !dbg !9832
  %1381 = fmul <4 x float> %_42.i.i.i.i28715410, %1378, !dbg !9836
  %1382 = fadd <4 x float> %1373, %1381, !dbg !9840
  %1383 = fmul <4 x float> %_45.i.i.i.i29015411, %1378, !dbg !9844
  %1384 = fadd <4 x float> %1375, %1383, !dbg !9848
  %1385 = fmul <4 x float> %_48.i.i.i.i29315412, %1378, !dbg !9852
  %1386 = fadd <4 x float> %1377, %1385, !dbg !9856
  %1387 = bitcast <4 x i32> %history.i.i18.sroa.10.018094 to <4 x float>, !dbg !9860
  %1388 = fmul <4 x float> %_53.i.i.i.i29815413, %1387, !dbg !9864
  %1389 = fadd <4 x float> %1380, %1388, !dbg !9865
  %1390 = fmul <4 x float> %_56.i.i.i.i30115414, %1387, !dbg !9869
  %1391 = fadd <4 x float> %1382, %1390, !dbg !9873
  %1392 = fmul <4 x float> %_59.i.i.i.i30415415, %1387, !dbg !9877
  %1393 = fadd <4 x float> %1384, %1392, !dbg !9881
  %1394 = fmul <4 x float> %_62.i.i.i.i30715416, %1387, !dbg !9885
  %1395 = fadd <4 x float> %1386, %1394, !dbg !9889
  %1396 = bitcast <4 x i32> %history.i.i18.sroa.13.018095 to <4 x float>, !dbg !9893
  %1397 = fmul <4 x float> %_67.i.i.i.i31215417, %1396, !dbg !9897
  %1398 = fadd <4 x float> %1389, %1397, !dbg !9898
  %1399 = fmul <4 x float> %_70.i.i.i.i31515418, %1396, !dbg !9902
  %1400 = fadd <4 x float> %1391, %1399, !dbg !9906
  %1401 = fmul <4 x float> %_73.i.i.i.i31815419, %1396, !dbg !9910
  %1402 = fadd <4 x float> %1393, %1401, !dbg !9914
  %1403 = fmul <4 x float> %_76.i.i.i.i32115420, %1396, !dbg !9918
  %1404 = fadd <4 x float> %1395, %1403, !dbg !9922
  %1405 = bitcast <4 x i32> %history.i.i18.sroa.16.018096 to <4 x float>, !dbg !9926
  %1406 = fmul <4 x float> %_81.i.i.i.i32615421, %1405, !dbg !9930
  %1407 = fadd <4 x float> %1398, %1406, !dbg !9931
  %1408 = fmul <4 x float> %_84.i.i.i.i32915422, %1405, !dbg !9935
  %1409 = fadd <4 x float> %1400, %1408, !dbg !9939
  %1410 = fmul <4 x float> %_87.i.i.i.i33215423, %1405, !dbg !9943
  %1411 = fadd <4 x float> %1402, %1410, !dbg !9947
  %1412 = fmul <4 x float> %_90.i.i.i.i33515424, %1405, !dbg !9951
  %1413 = fadd <4 x float> %1404, %1412, !dbg !9955
  %1414 = fmul <4 x float> %_95.i.i.i.i34015425, %1358, !dbg !9959
  %1415 = fadd <4 x float> %1407, %1414, !dbg !9963
  %1416 = fmul <4 x float> %_98.i.i.i.i34315426, %1358, !dbg !9967
  %1417 = fadd <4 x float> %1409, %1416, !dbg !9971
  %1418 = fmul <4 x float> %_101.i.i.i.i34615427, %1358, !dbg !9975
  %1419 = fadd <4 x float> %1411, %1418, !dbg !9979
  %1420 = fmul <4 x float> %_104.i.i.i.i34915428, %1358, !dbg !9983
  %1421 = fadd <4 x float> %1413, %1420, !dbg !9987
  %1422 = bitcast <4 x i32> %history.i.i18.sroa.22.018098 to <4 x float>, !dbg !9991
  %1423 = fmul <4 x float> %_109.i.i.i.i35415429, %1422, !dbg !9995
  %1424 = fadd <4 x float> %1415, %1423, !dbg !9996
  %1425 = fmul <4 x float> %_112.i.i.i.i35715430, %1422, !dbg !10000
  %1426 = fadd <4 x float> %1417, %1425, !dbg !10004
  %1427 = fmul <4 x float> %_115.i.i.i.i36015431, %1422, !dbg !10008
  %1428 = fadd <4 x float> %1419, %1427, !dbg !10012
  %1429 = fmul <4 x float> %_118.i.i.i.i36315432, %1422, !dbg !10016
  %1430 = fadd <4 x float> %1421, %1429, !dbg !10020
  %1431 = bitcast <4 x i32> %history.i.i18.sroa.26.018099 to <4 x float>, !dbg !10024
  %1432 = fmul <4 x float> %_123.i.i.i.i36815433, %1431, !dbg !10028
  %1433 = fadd <4 x float> %1424, %1432, !dbg !10029
  %1434 = fmul <4 x float> %_126.i.i.i.i37115434, %1431, !dbg !10033
  %1435 = fadd <4 x float> %1426, %1434, !dbg !10037
  %1436 = fmul <4 x float> %_129.i.i.i.i37415435, %1431, !dbg !10041
  %1437 = fadd <4 x float> %1428, %1436, !dbg !10045
  %1438 = fmul <4 x float> %_132.i.i.i.i37715436, %1431, !dbg !10049
  %1439 = fadd <4 x float> %1430, %1438, !dbg !10053
  %1440 = bitcast <4 x i32> %history.i.i18.sroa.29.018100 to <4 x float>, !dbg !10057
  %1441 = fmul <4 x float> %_137.i.i.i.i38215437, %1440, !dbg !10061
  %1442 = fadd <4 x float> %1433, %1441, !dbg !10062
  %1443 = fmul <4 x float> %_140.i.i.i.i38515438, %1440, !dbg !10066
  %1444 = fadd <4 x float> %1435, %1443, !dbg !10070
  %1445 = fmul <4 x float> %_143.i.i.i.i38815439, %1440, !dbg !10074
  %1446 = fadd <4 x float> %1437, %1445, !dbg !10078
  %1447 = fmul <4 x float> %_146.i.i.i.i39115440, %1440, !dbg !10082
  %1448 = fadd <4 x float> %1439, %1447, !dbg !10086
  %1449 = bitcast <4 x i32> %history.i.i18.sroa.32.018101 to <4 x float>, !dbg !10090
  %1450 = fmul <4 x float> %_151.i.i.i.i39615441, %1449, !dbg !10094
  %1451 = fadd <4 x float> %1442, %1450, !dbg !10095
  %1452 = fmul <4 x float> %_154.i.i.i.i39915442, %1449, !dbg !10099
  %1453 = fadd <4 x float> %1444, %1452, !dbg !10103
  %1454 = fmul <4 x float> %_157.i.i.i.i40215443, %1449, !dbg !10107
  %1455 = fadd <4 x float> %1446, %1454, !dbg !10111
  %1456 = fmul <4 x float> %_160.i.i.i.i40515444, %1449, !dbg !10115
  %1457 = fadd <4 x float> %1448, %1456, !dbg !10119
  %1458 = bitcast <4 x i32> %history.i.i18.sroa.35.018102 to <4 x float>, !dbg !10123
  %1459 = fmul <4 x float> %_165.i.i.i.i41015445, %1458, !dbg !10127
  %1460 = fadd <4 x float> %1451, %1459, !dbg !10128
  %1461 = fmul <4 x float> %_168.i.i.i.i41315446, %1458, !dbg !10132
  %1462 = fadd <4 x float> %1453, %1461, !dbg !10136
  %1463 = fmul <4 x float> %_171.i.i.i.i41615447, %1458, !dbg !10140
  %1464 = fadd <4 x float> %1455, %1463, !dbg !10144
  %1465 = fmul <4 x float> %_174.i.i.i.i41915448, %1458, !dbg !10148
  %1466 = fadd <4 x float> %1457, %1465, !dbg !10152
  %1467 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1460), !dbg !10156
  %1468 = fcmp olt <4 x float> %1467, %1359, !dbg !10160
  %1469 = select <4 x i1> %1468, <4 x float> %1359, <4 x float> %1467, !dbg !10164
  %1470 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1462), !dbg !10156
  %1471 = fcmp olt <4 x float> %1470, %1469, !dbg !10160
  %1472 = select <4 x i1> %1471, <4 x float> %1469, <4 x float> %1470, !dbg !10164
  %1473 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1464), !dbg !10156
  %1474 = fcmp olt <4 x float> %1473, %1472, !dbg !10160
  %1475 = select <4 x i1> %1474, <4 x float> %1472, <4 x float> %1473, !dbg !10164
  %1476 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1466), !dbg !10156
  %1477 = fcmp olt <4 x float> %1476, %1475, !dbg !10160
  %1478 = select <4 x i1> %1477, <4 x float> %1475, <4 x float> %1476, !dbg !10164
  %_39.i.i432.idx = shl i32 %iter.sroa.0.0.i.i7018103, 4, !dbg !10165
  %_39.i.i432 = getelementptr inbounds nuw i8, ptr %peaks_right.i40, i32 %_39.i.i432.idx, !dbg !10165
  store <4 x float> %1478, ptr %_39.i.i432, align 4, !dbg !10170, !alias.scope !10175, !noalias !10179
  %exitcond20527.not = icmp eq i32 %1357, %umax20526, !dbg !10183
  br i1 %exitcond20527.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i72, label %bb5.i.i236, !dbg !9716

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i72: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit250.i
  %history.i.i18.sroa.0.0.lcssa = phi <4 x i32> [ %history.i.i18.sroa.0.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit250.i ], [ %lanes.i5320.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997 ], !dbg !10185
  %history.i.i18.sroa.7.0.lcssa = phi <4 x i32> [ %history.i.i18.sroa.7.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit250.i ], [ %history.i.i18.sroa.0.018092, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997 ], !dbg !10185
  %history.i.i18.sroa.10.0.lcssa = phi <4 x i32> [ %history.i.i18.sroa.10.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit250.i ], [ %history.i.i18.sroa.7.018093, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997 ], !dbg !10185
  %history.i.i18.sroa.13.0.lcssa = phi <4 x i32> [ %history.i.i18.sroa.13.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit250.i ], [ %history.i.i18.sroa.10.018094, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997 ], !dbg !10185
  %history.i.i18.sroa.16.0.lcssa = phi <4 x i32> [ %history.i.i18.sroa.16.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit250.i ], [ %history.i.i18.sroa.13.018095, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997 ], !dbg !10185
  %history.i.i18.sroa.19.0.lcssa = phi <4 x i32> [ %history.i.i18.sroa.19.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit250.i ], [ %history.i.i18.sroa.16.018096, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997 ], !dbg !10185
  %history.i.i18.sroa.22.0.lcssa = phi <4 x i32> [ %history.i.i18.sroa.22.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit250.i ], [ %history.i.i18.sroa.19.018097, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997 ], !dbg !10185
  %history.i.i18.sroa.26.0.lcssa = phi <4 x i32> [ %history.i.i18.sroa.26.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit250.i ], [ %history.i.i18.sroa.22.018098, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997 ], !dbg !10185
  %history.i.i18.sroa.29.0.lcssa = phi <4 x i32> [ %history.i.i18.sroa.29.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit250.i ], [ %history.i.i18.sroa.26.018099, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997 ], !dbg !10185
  %history.i.i18.sroa.32.0.lcssa = phi <4 x i32> [ %history.i.i18.sroa.32.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit250.i ], [ %history.i.i18.sroa.29.018100, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997 ], !dbg !10185
  %history.i.i18.sroa.35.0.lcssa = phi <4 x i32> [ %history.i.i18.sroa.35.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit250.i ], [ %history.i.i18.sroa.32.018101, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997 ], !dbg !10185
  %history.i.i18.sroa.38.0.lcssa = phi <4 x i32> [ %history.i.i18.sroa.38.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit250.i ], [ %history.i.i18.sroa.35.018102, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5997 ], !dbg !10185
  store <4 x i32> %history.i.i18.sroa.0.0.lcssa, ptr %hot_right.i43, align 16, !dbg !10186
  store <4 x i32> %history.i.i18.sroa.7.0.lcssa, ptr %history.i.i18.sroa.7.0.hot_right.i43.sroa_idx, align 16, !dbg !10186
  store <4 x i32> %history.i.i18.sroa.10.0.lcssa, ptr %history.i.i18.sroa.10.0.hot_right.i43.sroa_idx, align 16, !dbg !10186
  store <4 x i32> %history.i.i18.sroa.13.0.lcssa, ptr %history.i.i18.sroa.13.0.hot_right.i43.sroa_idx, align 16, !dbg !10186
  store <4 x i32> %history.i.i18.sroa.16.0.lcssa, ptr %history.i.i18.sroa.16.0.hot_right.i43.sroa_idx, align 16, !dbg !10186
  store <4 x i32> %history.i.i18.sroa.19.0.lcssa, ptr %history.i.i18.sroa.19.0.hot_right.i43.sroa_idx, align 16, !dbg !10186
  store <4 x i32> %history.i.i18.sroa.22.0.lcssa, ptr %history.i.i18.sroa.22.0.hot_right.i43.sroa_idx, align 16, !dbg !10186
  store <4 x i32> %history.i.i18.sroa.26.0.lcssa, ptr %history.i.i18.sroa.26.0.hot_right.i43.sroa_idx, align 16, !dbg !10186
  store <4 x i32> %history.i.i18.sroa.29.0.lcssa, ptr %history.i.i18.sroa.29.0.hot_right.i43.sroa_idx, align 16, !dbg !10186
  store <4 x i32> %history.i.i18.sroa.32.0.lcssa, ptr %history.i.i18.sroa.32.0.hot_right.i43.sroa_idx, align 16, !dbg !10186
  store <4 x i32> %history.i.i18.sroa.35.0.lcssa, ptr %history.i.i18.sroa.35.0.hot_right.i43.sroa_idx, align 16, !dbg !10186
  store <4 x i32> %history.i.i18.sroa.38.0.lcssa, ptr %history.i.i18.sroa.38.0.hot_right.i43.sroa_idx, align 16, !dbg !10186
  br i1 %_20.i45.i18064.not, label %bb15.i61.loopexit, label %bb20.i78.preheader, !dbg !10187

bb20.i78.preheader:                               ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i72
  %_68.i36.sroa.3.0.copyload = load i32, ptr %_68.i36.sroa.3.0..sroa_idx, align 4, !noalias !9198
  %_68.i36.sroa.4.0.copyload = load i32, ptr %_68.i36.sroa.4.0..sroa_idx, align 4, !noalias !9198
  %_69.i35.sroa.3.0.copyload = load i32, ptr %_69.i35.sroa.3.0..sroa_idx, align 4, !noalias !9198
  %_69.i35.sroa.4.0.copyload = load i32, ptr %_69.i35.sroa.4.0..sroa_idx, align 4, !noalias !9198
  %_54.0.i264.i = load ptr, ptr %1213, align 16, !nonnull !10, !align !10189
  %_54.1.i265.i = load i32, ptr %1214, align 4
  %_18.i276.i = load i32, ptr %1198, align 4
  %_29.i126218116.not = icmp eq i32 %_18.i276.i, 0
  %_56.0.i286.i = load ptr, ptr %1215, align 8, !nonnull !10, !align !10189
  %_56.1.i287.i = load i32, ptr %1216, align 4
  %_58.1.i317.i = load i32, ptr %1220, align 4
  %_58.0.i316.i = load ptr, ptr %1221, align 16, !nonnull !10, !align !10189
  %_54.0.i.i166 = load ptr, ptr %1223, align 16, !nonnull !10, !align !10189
  %_54.1.i.i167 = load i32, ptr %1224, align 4
  %_18.i251.i = load i32, ptr %1199, align 4
  %_29.i123918119.not = icmp eq i32 %_18.i251.i, 0
  %_56.0.i.i183 = load ptr, ptr %1225, align 8, !nonnull !10, !align !10189
  %_56.1.i.i184 = load i32, ptr %1226, align 4
  %_58.1.i.i212 = load i32, ptr %1230, align 4
  %_58.0.i.i211 = load ptr, ptr %1231, align 16, !nonnull !10, !align !10189
  %_22.i279.i.promoted22669 = load i32, ptr %_22.i279.i, align 4
  %_22.i.i178.promoted22687 = load i32, ptr %_22.i.i178, align 4
  br label %bb20.i78, !dbg !10190

bb20.i78:                                         ; preds = %bb20.i78.preheader, %bb67.i227
  %storemerge.i.lcssa2265222689 = phi i32 [ %_22.i.i178.promoted22687, %bb20.i78.preheader ], [ %storemerge.i.lcssa2265222688, %bb67.i227 ]
  %storemerge.i1257.lcssa2262222671 = phi i32 [ %_22.i279.i.promoted22669, %bb20.i78.preheader ], [ %storemerge.i1257.lcssa2262222670, %bb67.i227 ]
  %frame.sroa.0.0.i7618128 = phi i32 [ 0, %bb20.i78.preheader ], [ %_83.i94, %bb67.i227 ]
  %main_cursor.sroa.0.1.i7518127 = phi i32 [ %main_cursor.sroa.0.0.i6318133, %bb20.i78.preheader ], [ %main_cursor.sroa.0.2.i233, %bb67.i227 ]
  %ring_cursor.sroa.0.1.i7418126 = phi i32 [ %ring_cursor.sroa.0.0.i6218132, %bb20.i78.preheader ], [ %ring_cursor.sroa.0.2.i230, %bb67.i227 ]
  %_65.i79 = sub nuw nsw i32 %spec.store.select.i68, %frame.sroa.0.0.i7618128, !dbg !10201
  %ring.i1908 = load i32, ptr %84, align 4, !dbg !10202, !alias.scope !10205, !noalias !10208, !noundef !10
  %main.i1909 = load i32, ptr %85, align 4, !dbg !10212, !alias.scope !10205, !noalias !10208, !noundef !10
  %_10.i = add i32 %ring_cursor.sroa.0.1.i7418126, 1, !dbg !10214
  %_38.not.i = icmp ult i32 %_10.i, %ring.i1908, !dbg !10216
  %1479 = select i1 %_38.not.i, i32 0, i32 %ring.i1908, !dbg !10216
  %start1.sroa.0.0.i1910 = sub nuw i32 %_10.i, %1479, !dbg !10216
  %_12.i1912 = add i32 %_68.i36.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i7418126, !dbg !10219
  %_39.not.i = icmp ult i32 %_12.i1912, %ring.i1908, !dbg !10221
  %1480 = select i1 %_39.not.i, i32 0, i32 %ring.i1908, !dbg !10221
  %left_end.sroa.0.0.i = sub nuw i32 %_12.i1912, %1480, !dbg !10221
  %_15.i1914 = add i32 %_69.i35.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i7418126, !dbg !10223
  %_40.not.i = icmp ult i32 %_15.i1914, %ring.i1908, !dbg !10225
  %1481 = select i1 %_40.not.i, i32 0, i32 %ring.i1908, !dbg !10225
  %right_end.sroa.0.0.i = sub nuw i32 %_15.i1914, %1481, !dbg !10225
  %_18.i1915 = add i32 %_68.i36.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i7418126, !dbg !10227
  %_41.not.i = icmp ult i32 %_18.i1915, %ring.i1908, !dbg !10229
  %1482 = select i1 %_41.not.i, i32 0, i32 %ring.i1908, !dbg !10229
  %left_expiring.sroa.0.0.i = sub nuw i32 %_18.i1915, %1482, !dbg !10229
  %_21.i1917 = add i32 %_69.i35.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i7418126, !dbg !10231
  %_42.not.i = icmp ult i32 %_21.i1917, %ring.i1908, !dbg !10233
  %1483 = select i1 %_42.not.i, i32 0, i32 %ring.i1908, !dbg !10233
  %right_expiring.sroa.0.0.i = sub nuw i32 %_21.i1917, %1483, !dbg !10233
  %1484 = sub i32 %ring.i1908, %ring_cursor.sroa.0.1.i7418126, !dbg !10235
  %spec.store.select.i1918 = tail call i32 @llvm.umin.i32(i32 %1484, i32 %_65.i79), !dbg !10237
  %1485 = sub i32 %main.i1909, %main_cursor.sroa.0.1.i7518127, !dbg !10240
  %_24.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %1485, i32 %spec.store.select.i1918), !dbg !10241
  %1486 = sub i32 %ring.i1908, %start1.sroa.0.0.i1910, !dbg !10243
  %_25.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %1486, i32 %_24.sroa.0.0.i), !dbg !10244
  %1487 = sub i32 %ring.i1908, %left_end.sroa.0.0.i, !dbg !10246
  %_27.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %1487, i32 %_25.sroa.0.0.i), !dbg !10247
  %1488 = sub i32 %ring.i1908, %right_end.sroa.0.0.i, !dbg !10249
  %_29.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %1488, i32 %_27.sroa.0.0.i), !dbg !10250
  %1489 = sub i32 %ring.i1908, %left_expiring.sroa.0.0.i, !dbg !10252
  %_31.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %1489, i32 %_29.sroa.0.0.i), !dbg !10253
  %1490 = sub i32 %ring.i1908, %right_expiring.sroa.0.0.i, !dbg !10255
  %run.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %1490, i32 %_31.sroa.0.0.i), !dbg !10256
  %_72.i81 = add i32 %frame.sroa.0.0.i7618128, %iter1.sroa.0.0.i6418134, !dbg !10258
  %base.i82 = shl i32 %_72.i81, 2, !dbg !10258
  %base.i8215357 = add i32 %run.sroa.0.0.i, %_72.i81, !dbg !10259
  %_76.i84 = shl i32 %base.i8215357, 2, !dbg !10259
  %_172.i85 = icmp ult i32 %_76.i84, %base.i82, !dbg !10190
  %_166.not.i86 = icmp ugt i32 %_76.i84, %left_io.1
  %or.cond.i87 = or i1 %_172.i85, %_166.not.i86, !dbg !10190
  br i1 %or.cond.i87, label %bb51.i235, label %bb49.i88, !dbg !10190, !prof !4694

bb51.i235:                                        ; preds = %bb20.i78
  store i32 %storemerge.i1257.lcssa2262222671, ptr %_22.i279.i, align 4
  store i32 %storemerge.i.lcssa2265222689, ptr %_22.i.i178, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i82, i32 noundef %_76.i84, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_bd3c2aadcc2cdd6816f02be0ceab2cc1) #33, !dbg !10260, !noalias !9169
  unreachable, !dbg !10260

bb49.i88:                                         ; preds = %bb20.i78
  %_175.i89 = getelementptr inbounds nuw float, ptr %left_io.0, i32 %base.i82, !dbg !10261
  %_176.not.i90 = icmp ugt i32 %_76.i84, %right_io.1, !dbg !10265
  br i1 %_176.not.i90, label %bb54.i234, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit, !dbg !10265, !prof !902

bb54.i234:                                        ; preds = %bb49.i88
  store i32 %storemerge.i1257.lcssa2262222671, ptr %_22.i279.i, align 4
  store i32 %storemerge.i.lcssa2265222689, ptr %_22.i.i178, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i82, i32 noundef %_76.i84, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c1156ffa98f252967759477badafe965) #33, !dbg !10270, !noalias !9169
  unreachable, !dbg !10270

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit: ; preds = %bb49.i88
  %_183.i92 = getelementptr inbounds nuw float, ptr %right_io.0, i32 %base.i82, !dbg !10271
  %_83.i94 = add nuw nsw i32 %run.sroa.0.0.i, %frame.sroa.0.0.i7618128, !dbg !10275
  %_80.i93 = shl nuw nsw i32 %frame.sroa.0.0.i7618128, 2, !dbg !10277
  %_192.i102 = getelementptr inbounds nuw float, ptr %peaks_left.i41, i32 %_80.i93, !dbg !10278
  %_201.i103 = getelementptr inbounds nuw float, ptr %peaks_right.i40, i32 %_80.i93, !dbg !10288
  %_2.i660218122.not = icmp eq i32 %run.sroa.0.0.i, 0, !dbg !10298
  br i1 %_2.i660218122.not, label %bb67.i227, label %bb68.i107.preheader, !dbg !10298

bb68.i107.preheader:                              ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit
  %umin20542 = call i32 @llvm.umin.i32(i32 %1487, i32 %1488)
  %umin20543 = call i32 @llvm.umin.i32(i32 %umin20542, i32 %1489)
  %umin20544 = call i32 @llvm.umin.i32(i32 %umin20543, i32 %1490)
  %umin20545 = call i32 @llvm.umin.i32(i32 %umin20544, i32 %1486)
  %umin20546 = call i32 @llvm.umin.i32(i32 %umin20545, i32 %1484)
  %umin20547 = call i32 @llvm.umin.i32(i32 %umin20546, i32 %1485)
  %1491 = sub nsw i32 %umin20548, %frame.sroa.0.0.i7618128
  %umin20549 = call i32 @llvm.umin.i32(i32 %umin20547, i32 %1491)
  %1492 = and i32 %umin20549, 1073741823
  %_11.i204215364.pre = load <4 x float>, ptr %_111.i125, align 16, !dbg !10308
  %_12.i2043.pre = load <4 x i32>, ptr %1204, align 16, !dbg !10312
  %_5.i202415366.pre = load <4 x float>, ptr %1206, align 16, !dbg !10313
  %_11.i202915368.pre = load <4 x float>, ptr %_115.i128, align 16, !dbg !10317
  %_12.i2030.pre = load <4 x i32>, ptr %1207, align 16, !dbg !10318
  %_13.i203215369.pre = load <16 x i8>, ptr %1208, align 16, !dbg !10319
  %_5.i201115370.pre = load <4 x float>, ptr %1209, align 16, !dbg !10320
  %_13.i205815361 = load <16 x i8>, ptr %1202, align 16
  %_13.i204515365 = load <16 x i8>, ptr %1205, align 16
  %_13.i201915373 = load <16 x i8>, ptr %1211, align 16
  %_37.i303.i15381 = load <4 x float>, ptr %1218, align 16
  %_37.i.i19815390 = load <4 x float>, ptr %1228, align 16
  %.promoted22491 = load <4 x float>, ptr %1200, align 16
  %.promoted22507 = load <4 x float>, ptr %1203, align 16
  %.promoted22623 = load <4 x float>, ptr %1217, align 16
  %.promoted22653 = load <4 x float>, ptr %1227, align 16
  br label %bb68.i107

bb68.i107:                                        ; preds = %bb68.i107.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1495
  %1493 = phi <4 x float> [ %.promoted22653, %bb68.i107.preheader ], [ %1626, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1495 ]
  %storemerge.i22639 = phi i32 [ %storemerge.i.lcssa2265222689, %bb68.i107.preheader ], [ %storemerge.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1495 ]
  %1494 = phi <4 x float> [ %.promoted22623, %bb68.i107.preheader ], [ %1575, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1495 ]
  %storemerge.i125722609 = phi i32 [ %storemerge.i1257.lcssa2262222671, %bb68.i107.preheader ], [ %storemerge.i1257, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1495 ]
  %1495 = phi <4 x float> [ %.promoted22507, %bb68.i107.preheader ], [ %1508, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1495 ], !dbg !10322
  %1496 = phi <4 x float> [ %.promoted22491, %bb68.i107.preheader ], [ %1499, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1495 ], !dbg !10327
  %_5.i201115370 = phi <4 x float> [ %_5.i201115370.pre, %bb68.i107.preheader ], [ %1526, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1495 ], !dbg !10320
  %_12.i2030 = phi <4 x i32> [ %_12.i2030.pre, %bb68.i107.preheader ], [ %1542, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1495 ], !dbg !10318
  %_11.i202915368 = phi <4 x float> [ %_11.i202915368.pre, %bb68.i107.preheader ], [ %1541, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1495 ], !dbg !10317
  %_5.i202415366 = phi <4 x float> [ %_5.i202415366.pre, %bb68.i107.preheader ], [ %1517, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1495 ], !dbg !10313
  %_12.i2043 = phi <4 x i32> [ %_12.i2043.pre, %bb68.i107.preheader ], [ %1540, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1495 ], !dbg !10312
  %_11.i204215364 = phi <4 x float> [ %_11.i204215364.pre, %bb68.i107.preheader ], [ %1539, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1495 ], !dbg !10308
  %iter.i26.sroa.41.018124 = phi i32 [ 0, %bb68.i107.preheader ], [ %_206.0.i123, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1495 ]
  %1497 = fadd <4 x float> %1496, splat (float -1.000000e+00), !dbg !10322
  %1498 = fcmp ogt <4 x float> %1497, zeroinitializer, !dbg !10328
  %1499 = select <4 x i1> %1498, <4 x float> %1497, <4 x float> zeroinitializer, !dbg !10332
  %1500 = sext <4 x i1> %1498 to <4 x i32>, !dbg !10333
  %_11.i205515360 = load <4 x float>, ptr %_110.i124, align 16, !dbg !10338
  %_12.i2056 = load <4 x i32>, ptr %1201, align 16, !dbg !10339
  %1501 = bitcast <4 x i32> %_12.i2056 to <4 x float>, !dbg !10340
  %1502 = fadd <4 x float> %_11.i205515360, %1501, !dbg !10344
  %1503 = bitcast <4 x float> %1502 to <16 x i8>, !dbg !10345
  %1504 = bitcast <4 x i32> %1500 to <16 x i8>, !dbg !10349
  %_4.i6609 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1503, <16 x i8> %_13.i205815361, <16 x i8> %1504), !dbg !10350
  store <16 x i8> %_4.i6609, ptr %_110.i124, align 16, !dbg !10351
  %1505 = bitcast <4 x i32> %_12.i2056 to <16 x i8>, !dbg !10352
  %_4.i6610 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1505, <16 x i8> zeroinitializer, <16 x i8> %1504), !dbg !10356
  store <16 x i8> %_4.i6610, ptr %1201, align 16, !dbg !10357
  %1506 = fadd <4 x float> %1495, splat (float -1.000000e+00), !dbg !10358
  %1507 = fcmp ogt <4 x float> %1506, zeroinitializer, !dbg !10362
  %1508 = select <4 x i1> %1507, <4 x float> %1506, <4 x float> zeroinitializer, !dbg !10366
  %1509 = sext <4 x i1> %1507 to <4 x i32>, !dbg !10367
  %1510 = bitcast <4 x i32> %_12.i2043 to <4 x float>, !dbg !10372
  %1511 = fadd <4 x float> %_11.i204215364, %1510, !dbg !10376
  %1512 = bitcast <4 x float> %1511 to <16 x i8>, !dbg !10377
  %1513 = bitcast <4 x i32> %1509 to <16 x i8>, !dbg !10381
  %_4.i6611 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1512, <16 x i8> %_13.i204515365, <16 x i8> %1513), !dbg !10382
  %1514 = bitcast <4 x i32> %_12.i2043 to <16 x i8>, !dbg !10383
  %_4.i6612 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1514, <16 x i8> zeroinitializer, <16 x i8> %1513), !dbg !10387
  %1515 = fadd <4 x float> %_5.i202415366, splat (float -1.000000e+00), !dbg !10388
  %1516 = fcmp ogt <4 x float> %1515, zeroinitializer, !dbg !10392
  %1517 = select <4 x i1> %1516, <4 x float> %1515, <4 x float> zeroinitializer, !dbg !10396
  %1518 = sext <4 x i1> %1516 to <4 x i32>, !dbg !10397
  %1519 = bitcast <4 x i32> %_12.i2030 to <4 x float>, !dbg !10402
  %1520 = fadd <4 x float> %_11.i202915368, %1519, !dbg !10406
  %1521 = bitcast <4 x float> %1520 to <16 x i8>, !dbg !10407
  %1522 = bitcast <4 x i32> %1518 to <16 x i8>, !dbg !10411
  %_4.i6613 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1521, <16 x i8> %_13.i203215369.pre, <16 x i8> %1522), !dbg !10412
  %1523 = bitcast <4 x i32> %_12.i2030 to <16 x i8>, !dbg !10413
  %_4.i6614 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1523, <16 x i8> zeroinitializer, <16 x i8> %1522), !dbg !10417
  %1524 = fadd <4 x float> %_5.i201115370, splat (float -1.000000e+00), !dbg !10418
  %1525 = fcmp ogt <4 x float> %1524, zeroinitializer, !dbg !10422
  %1526 = select <4 x i1> %1525, <4 x float> %1524, <4 x float> zeroinitializer, !dbg !10426
  %1527 = sext <4 x i1> %1525 to <4 x i32>, !dbg !10427
  %_11.i201615372 = load <4 x float>, ptr %_116.i129, align 16, !dbg !10432
  %_12.i2017 = load <4 x i32>, ptr %1210, align 16, !dbg !10433
  %1528 = bitcast <4 x i32> %_12.i2017 to <4 x float>, !dbg !10434
  %1529 = fadd <4 x float> %_11.i201615372, %1528, !dbg !10438
  %1530 = bitcast <4 x float> %1529 to <16 x i8>, !dbg !10439
  %1531 = bitcast <4 x i32> %1527 to <16 x i8>, !dbg !10443
  %_4.i6615 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1530, <16 x i8> %_13.i201915373, <16 x i8> %1531), !dbg !10444
  store <16 x i8> %_4.i6615, ptr %_116.i129, align 16, !dbg !10445
  %1532 = bitcast <4 x i32> %_12.i2017 to <16 x i8>, !dbg !10446
  %_4.i6616 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1532, <16 x i8> zeroinitializer, <16 x i8> %1531), !dbg !10450
  store <16 x i8> %_4.i6616, ptr %1210, align 16, !dbg !10451
  %start1.i.i.i.i.i.i = shl i32 %iter.i26.sroa.41.018124, 2, !dbg !10452
  %data.i.i.i.i = getelementptr inbounds nuw float, ptr %_192.i102, i32 %start1.i.i.i.i.i.i, !dbg !10468
  %lanes.i5356.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i.i.i, align 4, !dbg !10474, !alias.scope !10480, !noalias !10484
  %data.i.i6608 = getelementptr inbounds nuw float, ptr %_201.i103, i32 %start1.i.i.i.i.i.i, !dbg !10488
  %lanes.i5347.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i6608, align 4, !dbg !10491, !alias.scope !10497, !noalias !10501
  %data.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_175.i89, i32 %start1.i.i.i.i.i.i, !dbg !10505
  %lanes.i5338.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i.i.i.i.i, align 4, !dbg !10509, !alias.scope !10518, !noalias !10522
  %data.i5.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_183.i92, i32 %start1.i.i.i.i.i.i, !dbg !10526
  %lanes.i5329.sroa.0.0.copyload = load <4 x i32>, ptr %data.i5.i.i.i.i.i, align 4, !dbg !10529, !alias.scope !10535, !noalias !10539
  %_206.0.i123 = add nuw nsw i32 %iter.i26.sroa.41.018124, 1, !dbg !10543
  %1533 = bitcast <4 x i32> %lanes.i5356.sroa.0.0.copyload to <4 x float>, !dbg !10546
  %1534 = bitcast <4 x i32> %lanes.i5347.sroa.0.0.copyload to <4 x float>, !dbg !10550
  %1535 = fcmp olt <4 x float> %1533, %1534, !dbg !10551
  %.v15374 = select <4 x i1> %1535, <4 x i32> %lanes.i5347.sroa.0.0.copyload, <4 x i32> %lanes.i5356.sroa.0.0.copyload, !dbg !10552
  %1536 = bitcast <4 x i32> %.v15374 to <16 x i8>, !dbg !10553
  %1537 = bitcast <4 x i32> %lanes.i5347.sroa.0.0.copyload to <16 x i8>, !dbg !10557
  %_4.i6620 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1536, <16 x i8> %1537, <16 x i8> %1212), !dbg !10561
  %_213.i145 = add i32 %iter.i26.sroa.41.018124, %ring_cursor.sroa.0.1.i7418126, !dbg !10562
  %_214.i146 = add i32 %iter.i26.sroa.41.018124, %main_cursor.sroa.0.1.i7518127, !dbg !10567
  %_215.i147 = add i32 %iter.i26.sroa.41.018124, %left_end.sroa.0.0.i, !dbg !10568
  %_216.i148 = add i32 %iter.i26.sroa.41.018124, %start1.sroa.0.0.i1910, !dbg !10569
  %_217.i149 = add i32 %iter.i26.sroa.41.018124, %left_expiring.sroa.0.0.i, !dbg !10570
  %base.i9.i267.i = shl i32 %_213.i145, 2, !dbg !10571
  %_7.i10.i268.i = add i32 %base.i9.i267.i, 4, !dbg !10578
  %1538 = or disjoint i32 %base.i9.i267.i, 3, !dbg !10580
  %or.cond.i13.i271.i.not = icmp ult i32 %1538, %_54.1.i265.i, !dbg !10580
  %1539 = bitcast <16 x i8> %_4.i6611 to <4 x float>, !dbg !10580
  %1540 = bitcast <16 x i8> %_4.i6612 to <4 x i32>, !dbg !10580
  %1541 = bitcast <16 x i8> %_4.i6613 to <4 x float>, !dbg !10580
  %1542 = bitcast <16 x i8> %_4.i6614 to <4 x i32>, !dbg !10580
  br i1 %or.cond.i13.i271.i.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i272.i, label %bb4.i15.i332.i, !dbg !10580, !prof !10587

bb4.i15.i332.i:                                   ; preds = %bb68.i107
  store i32 %storemerge.i1257.lcssa2262222671, ptr %_22.i279.i, align 4
  store i32 %storemerge.i.lcssa2265222689, ptr %_22.i.i178, align 4
  store <4 x float> %1499, ptr %1200, align 16, !dbg !10588
  store <4 x float> %1508, ptr %1203, align 16, !dbg !10589
  store <16 x i8> %_4.i6611, ptr %_111.i125, align 16, !dbg !10590
  store <16 x i8> %_4.i6612, ptr %1204, align 16, !dbg !10591
  store <4 x float> %1517, ptr %1206, align 16, !dbg !10592
  store <16 x i8> %_4.i6613, ptr %_115.i128, align 16, !dbg !10593
  store <16 x i8> %_4.i6614, ptr %1207, align 16, !dbg !10594
  store <4 x float> %1526, ptr %1209, align 16, !dbg !10595
  store i32 %storemerge.i125722609, ptr %_22.i279.i, align 4, !dbg !10596
  store <4 x float> %1494, ptr %1217, align 16, !dbg !10600
  store i32 %storemerge.i22639, ptr %_22.i.i178, align 4, !dbg !10604
  store <4 x float> %1493, ptr %1227, align 16, !dbg !10607
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i9.i267.i, i32 noundef %_7.i10.i268.i, i32 noundef range(i32 0, 536870912) %_54.1.i265.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #33, !dbg !10608, !noalias !10609
  unreachable, !dbg !10608

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i272.i: ; preds = %bb68.i107
  %1543 = bitcast <4 x i32> %lanes.i5356.sroa.0.0.copyload to <16 x i8>, !dbg !10623
  %_4.i6619 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1536, <16 x i8> %1543, <16 x i8> %1212), !dbg !10624
  %1544 = bitcast <16 x i8> %_4.i6609 to <4 x float>, !dbg !10625
  %1545 = bitcast <16 x i8> %_4.i6619 to <4 x float>, !dbg !10630
  %1546 = fdiv <4 x float> %1544, %1545, !dbg !10631
  %1547 = bitcast <4 x float> %1546 to <16 x i8>, !dbg !10635
  %1548 = fcmp ogt <4 x float> %1545, %1544, !dbg !10639
  %1549 = sext <4 x i1> %1548 to <4 x i32>, !dbg !10639
  %1550 = bitcast <4 x i32> %1549 to <16 x i8>, !dbg !10640
  %_4.i6623 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1547, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %1550), !dbg !10641
  %_17.i14.i273.i = getelementptr inbounds nuw float, ptr %_54.0.i264.i, i32 %base.i9.i267.i, !dbg !10642
  store <16 x i8> %_4.i6623, ptr %_17.i14.i273.i, align 4, !dbg !10646, !alias.scope !10651, !noalias !10655
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10659), !dbg !10662
  %base.i1442 = shl i32 %_215.i147, 2, !dbg !10663
  %1551 = or disjoint i32 %base.i1442, 3, !dbg !10666
  %or.cond.i1446.not = icmp ult i32 %1551, %_54.1.i265.i, !dbg !10666
  br i1 %or.cond.i1446.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1450, label %bb4.i1449, !dbg !10666, !prof !10587

bb4.i1449:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i272.i
  store i32 %storemerge.i1257.lcssa2262222671, ptr %_22.i279.i, align 4
  store i32 %storemerge.i.lcssa2265222689, ptr %_22.i.i178, align 4
  store <4 x float> %1499, ptr %1200, align 16, !dbg !10588
  store <4 x float> %1508, ptr %1203, align 16, !dbg !10589
  store <16 x i8> %_4.i6611, ptr %_111.i125, align 16, !dbg !10590
  store <16 x i8> %_4.i6612, ptr %1204, align 16, !dbg !10591
  store <4 x float> %1517, ptr %1206, align 16, !dbg !10592
  store <16 x i8> %_4.i6613, ptr %_115.i128, align 16, !dbg !10593
  store <16 x i8> %_4.i6614, ptr %1207, align 16, !dbg !10594
  store <4 x float> %1526, ptr %1209, align 16, !dbg !10595
  store i32 %storemerge.i125722609, ptr %_22.i279.i, align 4, !dbg !10596
  store <4 x float> %1494, ptr %1217, align 16, !dbg !10600
  store i32 %storemerge.i22639, ptr %_22.i.i178, align 4, !dbg !10604
  store <4 x float> %1493, ptr %1227, align 16, !dbg !10607
  %_5.i1443 = add i32 %base.i1442, 4, !dbg !10674
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1442, i32 noundef %_5.i1443, i32 noundef range(i32 0, 536870912) %_54.1.i265.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !10675, !noalias !10676
  unreachable, !dbg !10675

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1450: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i272.i
  %_15.i1448 = getelementptr inbounds nuw float, ptr %_54.0.i264.i, i32 %base.i1442, !dbg !10682
  %lanes.i5027.sroa.0.0.copyload = load <4 x i32>, ptr %_15.i1448, align 4, !dbg !10686
  %1552 = icmp eq i32 %storemerge.i125722609, 0, !dbg !10691
  %_12.i124915375 = load <4 x float>, ptr %uniform_left.i39, align 16, !dbg !10691
  %1553 = bitcast <4 x i32> %lanes.i5027.sroa.0.0.copyload to <4 x float>, !dbg !10691
  %1554 = fcmp olt <4 x float> %_12.i124915375, %1553, !dbg !10691
  %1555 = select <4 x i1> %1554, <4 x float> %_12.i124915375, <4 x float> %1553, !dbg !10691
  %1556 = bitcast <4 x float> %1555 to <4 x i32>, !dbg !10691
  %.sroa.08736.0 = select i1 %1552, <4 x i32> %lanes.i5027.sroa.0.0.copyload, <4 x i32> %1556, !dbg !10691
  store <4 x i32> %.sroa.08736.0, ptr %uniform_left.i39, align 16, !dbg !10693, !alias.scope !10659, !noalias !10695
  %_15.i1252 = add i32 %storemerge.i125722609, 1, !dbg !10697
  %complete.i1253 = icmp eq i32 %_15.i1252, %_18.i276.i, !dbg !10697
  br i1 %complete.i1253, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1432, label %bb7.i1254, !dbg !10698

bb7.i1254:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1450
  %base.i1433 = shl i32 %_216.i148, 2, !dbg !10700
  %1557 = or disjoint i32 %base.i1433, 3, !dbg !10702
  %or.cond.i1437.not = icmp ult i32 %1557, %_54.1.i265.i, !dbg !10702
  br i1 %or.cond.i1437.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1441, label %bb4.i1440, !dbg !10702, !prof !10587

bb4.i1440:                                        ; preds = %bb7.i1254
  store i32 %storemerge.i1257.lcssa2262222671, ptr %_22.i279.i, align 4
  store i32 %storemerge.i.lcssa2265222689, ptr %_22.i.i178, align 4
  store <4 x float> %1499, ptr %1200, align 16, !dbg !10588
  store <4 x float> %1508, ptr %1203, align 16, !dbg !10589
  store <16 x i8> %_4.i6611, ptr %_111.i125, align 16, !dbg !10590
  store <16 x i8> %_4.i6612, ptr %1204, align 16, !dbg !10591
  store <4 x float> %1517, ptr %1206, align 16, !dbg !10592
  store <16 x i8> %_4.i6613, ptr %_115.i128, align 16, !dbg !10593
  store <16 x i8> %_4.i6614, ptr %1207, align 16, !dbg !10594
  store <4 x float> %1526, ptr %1209, align 16, !dbg !10595
  store i32 %storemerge.i125722609, ptr %_22.i279.i, align 4, !dbg !10596
  store <4 x float> %1494, ptr %1217, align 16, !dbg !10600
  store i32 %storemerge.i22639, ptr %_22.i.i178, align 4, !dbg !10604
  store <4 x float> %1493, ptr %1227, align 16, !dbg !10607
  %_5.i1434 = add i32 %base.i1433, 4, !dbg !10706
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1433, i32 noundef %_5.i1434, i32 noundef range(i32 0, 536870912) %_54.1.i265.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !10707, !noalias !10708
  unreachable, !dbg !10707

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1441: ; preds = %bb7.i1254
  %_15.i1439 = getelementptr inbounds nuw float, ptr %_54.0.i264.i, i32 %base.i1433, !dbg !10712
  %lanes.i5034.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1439, align 4, !dbg !10714, !alias.scope !10719, !noalias !10723
  %1558 = bitcast <4 x i32> %.sroa.08736.0 to <4 x float>, !dbg !10727
  %1559 = fcmp olt <4 x float> %lanes.i5034.sroa.0.0.copyload, %1558, !dbg !10734
  %1560 = select <4 x i1> %1559, <4 x float> %lanes.i5034.sroa.0.0.copyload, <4 x float> %1558, !dbg !10735
  %1561 = bitcast <4 x float> %1560 to <4 x i32>, !dbg !10736
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1278, !dbg !10741

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1432: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1450
  %1562 = bitcast <4 x i32> %lanes.i5027.sroa.0.0.copyload to <4 x float>, !dbg !10698
  br i1 %_29.i126218116.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1278, label %bb19.i1263, !dbg !10743

bb19.i1263:                                       ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1432, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1423
  %end.sroa.0.0.i126118118 = phi i32 [ %1568, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1423 ], [ %_215.i147, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1432 ]
  %iter.sroa.0.0.i126018117 = phi i32 [ %_30.i1264, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1423 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1432 ]
  %1563 = phi <4 x float> [ %1566, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1423 ], [ %1562, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1432 ]
  %base.i1415 = shl i32 %end.sroa.0.0.i126118118, 2, !dbg !10752
  %1564 = or disjoint i32 %base.i1415, 3, !dbg !10754
  %or.cond.i1419.not = icmp ult i32 %1564, %_54.1.i265.i, !dbg !10754
  br i1 %or.cond.i1419.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1423, label %bb4.i1422, !dbg !10754, !prof !10587

bb4.i1422:                                        ; preds = %bb19.i1263
  store i32 %storemerge.i1257.lcssa2262222671, ptr %_22.i279.i, align 4
  store i32 %storemerge.i.lcssa2265222689, ptr %_22.i.i178, align 4
  store <4 x float> %1499, ptr %1200, align 16, !dbg !10588
  store <4 x float> %1508, ptr %1203, align 16, !dbg !10589
  store <16 x i8> %_4.i6611, ptr %_111.i125, align 16, !dbg !10590
  store <16 x i8> %_4.i6612, ptr %1204, align 16, !dbg !10591
  store <4 x float> %1517, ptr %1206, align 16, !dbg !10592
  store <16 x i8> %_4.i6613, ptr %_115.i128, align 16, !dbg !10593
  store <16 x i8> %_4.i6614, ptr %1207, align 16, !dbg !10594
  store <4 x float> %1526, ptr %1209, align 16, !dbg !10595
  store i32 %storemerge.i125722609, ptr %_22.i279.i, align 4, !dbg !10596
  store <4 x float> %1494, ptr %1217, align 16, !dbg !10600
  store i32 %storemerge.i22639, ptr %_22.i.i178, align 4, !dbg !10604
  store <4 x float> %1493, ptr %1227, align 16, !dbg !10607
  %_5.i1416 = add i32 %base.i1415, 4, !dbg !10758
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1415, i32 noundef %_5.i1416, i32 noundef range(i32 0, 536870912) %_54.1.i265.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !10759, !noalias !10760
  unreachable, !dbg !10759

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1423: ; preds = %bb19.i1263
  %_30.i1264 = add nuw i32 %iter.sroa.0.0.i126018117, 1, !dbg !10764
  %_15.i1421 = getelementptr inbounds nuw float, ptr %_54.0.i264.i, i32 %base.i1415, !dbg !10770
  %lanes.i5048.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1421, align 4, !dbg !10772, !alias.scope !10777, !noalias !10781
  %1565 = fcmp olt <4 x float> %1563, %lanes.i5048.sroa.0.0.copyload, !dbg !10785
  %1566 = select <4 x i1> %1565, <4 x float> %1563, <4 x float> %lanes.i5048.sroa.0.0.copyload, !dbg !10789
  store <4 x float> %1566, ptr %_15.i1421, align 4, !dbg !10790, !alias.scope !10796, !noalias !10800
  %1567 = icmp eq i32 %end.sroa.0.0.i126118118, 0, !dbg !10806
  %spec.store.select.i1275 = select i1 %1567, i32 %ring.i52, i32 %end.sroa.0.0.i126118118, !dbg !10806
  %1568 = add i32 %spec.store.select.i1275, -1, !dbg !10807
  %exitcond20533.not = icmp eq i32 %_30.i1264, %_18.i276.i, !dbg !10808
  br i1 %exitcond20533.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1278, label %bb19.i1263, !dbg !10743

_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1278: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1423, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1432, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1441
  %.sroa.08736.1 = phi <4 x i32> [ %1561, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1441 ], [ %.sroa.08736.0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1432 ], [ %.sroa.08736.0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1423 ], !dbg !10811
  %storemerge.i1257 = phi i32 [ %_15.i1252, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1441 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1432 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1423 ], !dbg !10812
  %1569 = bitcast <4 x i32> %.sroa.08736.1 to <4 x float>, !dbg !10813
  %1570 = fmul <4 x float> %1569, splat (float 1.638400e+04), !dbg !10817
  %1571 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %1570), !dbg !10818
  %1572 = fmul <4 x float> %1571, splat (float 0x3F10000000000000), !dbg !10822
  %base.i1514 = shl i32 %_217.i149, 2, !dbg !10826
  %1573 = or disjoint i32 %base.i1514, 3, !dbg !10828
  %or.cond.i1518.not = icmp ult i32 %1573, %_56.1.i287.i, !dbg !10828
  br i1 %or.cond.i1518.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1522, label %bb4.i1521, !dbg !10828, !prof !10587

bb4.i1521:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1278
  store i32 %storemerge.i1257.lcssa2262222671, ptr %_22.i279.i, align 4
  store i32 %storemerge.i.lcssa2265222689, ptr %_22.i.i178, align 4
  store <4 x float> %1499, ptr %1200, align 16, !dbg !10588
  store <4 x float> %1508, ptr %1203, align 16, !dbg !10589
  store <16 x i8> %_4.i6611, ptr %_111.i125, align 16, !dbg !10590
  store <16 x i8> %_4.i6612, ptr %1204, align 16, !dbg !10591
  store <4 x float> %1517, ptr %1206, align 16, !dbg !10592
  store <16 x i8> %_4.i6613, ptr %_115.i128, align 16, !dbg !10593
  store <16 x i8> %_4.i6614, ptr %1207, align 16, !dbg !10594
  store <4 x float> %1526, ptr %1209, align 16, !dbg !10595
  store i32 %storemerge.i1257, ptr %_22.i279.i, align 4, !dbg !10596
  store <4 x float> %1494, ptr %1217, align 16, !dbg !10600
  store i32 %storemerge.i22639, ptr %_22.i.i178, align 4, !dbg !10604
  store <4 x float> %1493, ptr %1227, align 16, !dbg !10607
  %_5.i1515 = add i32 %base.i1514, 4, !dbg !10832
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1514, i32 noundef %_5.i1515, i32 noundef range(i32 0, 536870912) %_56.1.i287.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !10833, !noalias !10834
  unreachable, !dbg !10833

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1522: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1278
  %_15.i1520 = getelementptr inbounds nuw float, ptr %_56.0.i286.i, i32 %base.i1514, !dbg !10838
  %lanes.i4971.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1520, align 4, !dbg !10840, !alias.scope !10845, !noalias !10849
  %1574 = fadd <4 x float> %1572, %1494, !dbg !10853
  %1575 = fsub <4 x float> %1574, %lanes.i4971.sroa.0.0.copyload, !dbg !10857
  %_8.not.i4.i298.i = icmp ugt i32 %_7.i10.i268.i, %_56.1.i287.i
  br i1 %_8.not.i4.i298.i, label %bb4.i7.i331.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i300.i, !dbg !10861, !prof !4694

bb4.i7.i331.i:                                    ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1522
  store i32 %storemerge.i1257.lcssa2262222671, ptr %_22.i279.i, align 4
  store i32 %storemerge.i.lcssa2265222689, ptr %_22.i.i178, align 4
  store <4 x float> %1499, ptr %1200, align 16, !dbg !10588
  store <4 x float> %1508, ptr %1203, align 16, !dbg !10589
  store <16 x i8> %_4.i6611, ptr %_111.i125, align 16, !dbg !10590
  store <16 x i8> %_4.i6612, ptr %1204, align 16, !dbg !10591
  store <4 x float> %1517, ptr %1206, align 16, !dbg !10592
  store <16 x i8> %_4.i6613, ptr %_115.i128, align 16, !dbg !10593
  store <16 x i8> %_4.i6614, ptr %1207, align 16, !dbg !10594
  store <4 x float> %1526, ptr %1209, align 16, !dbg !10595
  store i32 %storemerge.i1257, ptr %_22.i279.i, align 4, !dbg !10596
  store <4 x float> %1575, ptr %1217, align 16, !dbg !10600
  store i32 %storemerge.i22639, ptr %_22.i.i178, align 4, !dbg !10604
  store <4 x float> %1493, ptr %1227, align 16, !dbg !10607
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i9.i267.i, i32 noundef %_7.i10.i268.i, i32 noundef range(i32 0, 536870912) %_56.1.i287.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #33, !dbg !10866, !noalias !10867
  unreachable, !dbg !10866

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i300.i: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1522
  %_17.i6.i301.i = getelementptr inbounds nuw float, ptr %_56.0.i286.i, i32 %base.i9.i267.i, !dbg !10871
  store <4 x float> %1572, ptr %_17.i6.i301.i, align 4, !dbg !10873, !alias.scope !10878, !noalias !10882
  %_41.i306.i15382 = load <4 x float>, ptr %1219, align 16, !dbg !10886
  %1576 = fdiv <4 x float> %1575, %_37.i303.i15381, !dbg !10889
  %1577 = fsub <4 x float> splat (float 1.000000e+00), %1576, !dbg !10893
  %1578 = fsub <4 x float> %1577, %_41.i306.i15382, !dbg !10897
  %1579 = bitcast <16 x i8> %_4.i6611 to <4 x float>, !dbg !10901
  %1580 = fmul <4 x float> %1578, %1579, !dbg !10905
  %1581 = fadd <4 x float> %_41.i306.i15382, %1580, !dbg !10906
  %1582 = fcmp olt <4 x float> %1581, %1577, !dbg !10909
  %1583 = select <4 x i1> %1582, <4 x float> %1577, <4 x float> %1581, !dbg !10914
  %1584 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1583), !dbg !10915
  %1585 = fcmp uge <4 x float> %1584, splat (float 0x3BC79CA100000000), !dbg !10921
  %1586 = bitcast <4 x float> %1583 to <4 x i32>, !dbg !10926
  %1587 = select <4 x i1> %1585, <4 x i32> %1586, <4 x i32> zeroinitializer, !dbg !10926
  store <4 x i32> %1587, ptr %1219, align 16, !dbg !10929
  %base.i1505 = shl i32 %_214.i146, 2, !dbg !10930
  %_5.i1506 = add i32 %base.i1505, 4, !dbg !10933
  %1588 = or disjoint i32 %base.i1505, 3, !dbg !10934
  %or.cond.i1509.not = icmp ult i32 %1588, %_58.1.i317.i, !dbg !10934
  br i1 %or.cond.i1509.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1513, label %bb4.i1512, !dbg !10934, !prof !10587

bb4.i1512:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i300.i
  store i32 %storemerge.i1257.lcssa2262222671, ptr %_22.i279.i, align 4
  store i32 %storemerge.i.lcssa2265222689, ptr %_22.i.i178, align 4
  store <4 x float> %1499, ptr %1200, align 16, !dbg !10588
  store <4 x float> %1508, ptr %1203, align 16, !dbg !10589
  store <16 x i8> %_4.i6611, ptr %_111.i125, align 16, !dbg !10590
  store <16 x i8> %_4.i6612, ptr %1204, align 16, !dbg !10591
  store <4 x float> %1517, ptr %1206, align 16, !dbg !10592
  store <16 x i8> %_4.i6613, ptr %_115.i128, align 16, !dbg !10593
  store <16 x i8> %_4.i6614, ptr %1207, align 16, !dbg !10594
  store <4 x float> %1526, ptr %1209, align 16, !dbg !10595
  store i32 %storemerge.i1257, ptr %_22.i279.i, align 4, !dbg !10596
  store <4 x float> %1575, ptr %1217, align 16, !dbg !10600
  store i32 %storemerge.i22639, ptr %_22.i.i178, align 4, !dbg !10604
  store <4 x float> %1493, ptr %1227, align 16, !dbg !10607
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1505, i32 noundef %_5.i1506, i32 noundef range(i32 0, 536870912) %_58.1.i317.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !10938, !noalias !10939
  unreachable, !dbg !10938

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1513: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i300.i
  %1589 = bitcast <4 x i32> %1587 to <4 x float>, !dbg !10943
  %1590 = fsub <4 x float> splat (float 1.000000e+00), %1589, !dbg !10947
  %_15.i1511 = getelementptr inbounds nuw float, ptr %_58.0.i316.i, i32 %base.i1505, !dbg !10948
  %lanes.i4978.sroa.0.0.copyload = load <4 x i32>, ptr %_15.i1511, align 4, !dbg !10950, !alias.scope !10955, !noalias !10959
  store <4 x i32> %lanes.i5338.sroa.0.0.copyload, ptr %_15.i1511, align 4, !dbg !10963, !alias.scope !10970, !noalias !10974
  %1591 = bitcast <4 x i32> %lanes.i4978.sroa.0.0.copyload to <4 x float>, !dbg !10980
  %1592 = fmul <4 x float> %1590, %1591, !dbg !10984
  %1593 = bitcast <4 x i32> %lanes.i4978.sroa.0.0.copyload to <16 x i8>, !dbg !10985
  %1594 = bitcast <4 x float> %1592 to <16 x i8>, !dbg !10989
  %_4.i6634 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1593, <16 x i8> %1594, <16 x i8> %1222), !dbg !10990
  store <16 x i8> %_4.i6634, ptr %data.i.i.i.i.i.i, align 4, !dbg !10991, !alias.scope !10996, !noalias !11000
  %_220.i158 = add i32 %iter.i26.sroa.41.018124, %right_end.sroa.0.0.i, !dbg !11004
  %_222.i160 = add i32 %iter.i26.sroa.41.018124, %right_expiring.sroa.0.0.i, !dbg !11006
  %_8.not.i12.i.i172 = icmp ugt i32 %_7.i10.i268.i, %_54.1.i.i167
  br i1 %_8.not.i12.i.i172, label %bb4.i15.i.i226, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i174, !dbg !11007, !prof !4694

bb4.i15.i.i226:                                   ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1513
  store i32 %storemerge.i1257.lcssa2262222671, ptr %_22.i279.i, align 4
  store i32 %storemerge.i.lcssa2265222689, ptr %_22.i.i178, align 4
  store <4 x float> %1499, ptr %1200, align 16, !dbg !10588
  store <4 x float> %1508, ptr %1203, align 16, !dbg !10589
  store <16 x i8> %_4.i6611, ptr %_111.i125, align 16, !dbg !10590
  store <16 x i8> %_4.i6612, ptr %1204, align 16, !dbg !10591
  store <4 x float> %1517, ptr %1206, align 16, !dbg !10592
  store <16 x i8> %_4.i6613, ptr %_115.i128, align 16, !dbg !10593
  store <16 x i8> %_4.i6614, ptr %1207, align 16, !dbg !10594
  store <4 x float> %1526, ptr %1209, align 16, !dbg !10595
  store i32 %storemerge.i1257, ptr %_22.i279.i, align 4, !dbg !10596
  store <4 x float> %1575, ptr %1217, align 16, !dbg !10600
  store i32 %storemerge.i22639, ptr %_22.i.i178, align 4, !dbg !10604
  store <4 x float> %1493, ptr %1227, align 16, !dbg !10607
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i9.i267.i, i32 noundef %_7.i10.i268.i, i32 noundef range(i32 0, 536870912) %_54.1.i.i167, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #33, !dbg !11012, !noalias !11013
  unreachable, !dbg !11012

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i174: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1513
  %1595 = bitcast <16 x i8> %_4.i6613 to <4 x float>, !dbg !11027
  %1596 = bitcast <16 x i8> %_4.i6620 to <4 x float>, !dbg !11032
  %1597 = fdiv <4 x float> %1595, %1596, !dbg !11033
  %1598 = bitcast <4 x float> %1597 to <16 x i8>, !dbg !11037
  %1599 = fcmp ogt <4 x float> %1596, %1595, !dbg !11041
  %1600 = sext <4 x i1> %1599 to <4 x i32>, !dbg !11041
  %1601 = bitcast <4 x i32> %1600 to <16 x i8>, !dbg !11042
  %_4.i6636 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1598, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %1601), !dbg !11043
  %_17.i14.i.i175 = getelementptr inbounds nuw float, ptr %_54.0.i.i166, i32 %base.i9.i267.i, !dbg !11044
  store <16 x i8> %_4.i6636, ptr %_17.i14.i.i175, align 4, !dbg !11046, !alias.scope !11051, !noalias !11055
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11059), !dbg !11062
  %base.i1478 = shl i32 %_220.i158, 2, !dbg !11063
  %1602 = or disjoint i32 %base.i1478, 3, !dbg !11065
  %or.cond.i1482.not = icmp ult i32 %1602, %_54.1.i.i167, !dbg !11065
  br i1 %or.cond.i1482.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1486, label %bb4.i1485, !dbg !11065, !prof !10587

bb4.i1485:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i174
  store i32 %storemerge.i1257.lcssa2262222671, ptr %_22.i279.i, align 4
  store i32 %storemerge.i.lcssa2265222689, ptr %_22.i.i178, align 4
  store <4 x float> %1499, ptr %1200, align 16, !dbg !10588
  store <4 x float> %1508, ptr %1203, align 16, !dbg !10589
  store <16 x i8> %_4.i6611, ptr %_111.i125, align 16, !dbg !10590
  store <16 x i8> %_4.i6612, ptr %1204, align 16, !dbg !10591
  store <4 x float> %1517, ptr %1206, align 16, !dbg !10592
  store <16 x i8> %_4.i6613, ptr %_115.i128, align 16, !dbg !10593
  store <16 x i8> %_4.i6614, ptr %1207, align 16, !dbg !10594
  store <4 x float> %1526, ptr %1209, align 16, !dbg !10595
  store i32 %storemerge.i1257, ptr %_22.i279.i, align 4, !dbg !10596
  store <4 x float> %1575, ptr %1217, align 16, !dbg !10600
  store i32 %storemerge.i22639, ptr %_22.i.i178, align 4, !dbg !10604
  store <4 x float> %1493, ptr %1227, align 16, !dbg !10607
  %_5.i1479 = add i32 %base.i1478, 4, !dbg !11069
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1478, i32 noundef %_5.i1479, i32 noundef range(i32 0, 536870912) %_54.1.i.i167, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !11070, !noalias !11071
  unreachable, !dbg !11070

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1486: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i174
  %_15.i1484 = getelementptr inbounds nuw float, ptr %_54.0.i.i166, i32 %base.i1478, !dbg !11077
  %lanes.i4999.sroa.0.0.copyload = load <4 x i32>, ptr %_15.i1484, align 4, !dbg !11079
  %1603 = icmp eq i32 %storemerge.i22639, 0, !dbg !11084
  %_12.i123315384 = load <4 x float>, ptr %uniform_right.i38, align 16, !dbg !11084
  %1604 = bitcast <4 x i32> %lanes.i4999.sroa.0.0.copyload to <4 x float>, !dbg !11084
  %1605 = fcmp olt <4 x float> %_12.i123315384, %1604, !dbg !11084
  %1606 = select <4 x i1> %1605, <4 x float> %_12.i123315384, <4 x float> %1604, !dbg !11084
  %1607 = bitcast <4 x float> %1606 to <4 x i32>, !dbg !11084
  %.sroa.08662.0 = select i1 %1603, <4 x i32> %lanes.i4999.sroa.0.0.copyload, <4 x i32> %1607, !dbg !11084
  store <4 x i32> %.sroa.08662.0, ptr %uniform_right.i38, align 16, !dbg !11085, !alias.scope !11059, !noalias !11086
  %_15.i = add i32 %storemerge.i22639, 1, !dbg !11088
  %complete.i = icmp eq i32 %_15.i, %_18.i251.i, !dbg !11088
  br i1 %complete.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1468, label %bb7.i1235, !dbg !11089

bb7.i1235:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1486
  %base.i1469 = shl i32 %_216.i148, 2, !dbg !11090
  %1608 = or disjoint i32 %base.i1469, 3, !dbg !11092
  %or.cond.i1473.not = icmp ult i32 %1608, %_54.1.i.i167, !dbg !11092
  br i1 %or.cond.i1473.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1477, label %bb4.i1476, !dbg !11092, !prof !10587

bb4.i1476:                                        ; preds = %bb7.i1235
  store i32 %storemerge.i1257.lcssa2262222671, ptr %_22.i279.i, align 4
  store i32 %storemerge.i.lcssa2265222689, ptr %_22.i.i178, align 4
  store <4 x float> %1499, ptr %1200, align 16, !dbg !10588
  store <4 x float> %1508, ptr %1203, align 16, !dbg !10589
  store <16 x i8> %_4.i6611, ptr %_111.i125, align 16, !dbg !10590
  store <16 x i8> %_4.i6612, ptr %1204, align 16, !dbg !10591
  store <4 x float> %1517, ptr %1206, align 16, !dbg !10592
  store <16 x i8> %_4.i6613, ptr %_115.i128, align 16, !dbg !10593
  store <16 x i8> %_4.i6614, ptr %1207, align 16, !dbg !10594
  store <4 x float> %1526, ptr %1209, align 16, !dbg !10595
  store i32 %storemerge.i1257, ptr %_22.i279.i, align 4, !dbg !10596
  store <4 x float> %1575, ptr %1217, align 16, !dbg !10600
  store i32 %storemerge.i22639, ptr %_22.i.i178, align 4, !dbg !10604
  store <4 x float> %1493, ptr %1227, align 16, !dbg !10607
  %_5.i1470 = add i32 %base.i1469, 4, !dbg !11096
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1469, i32 noundef %_5.i1470, i32 noundef range(i32 0, 536870912) %_54.1.i.i167, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !11097, !noalias !11098
  unreachable, !dbg !11097

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1477: ; preds = %bb7.i1235
  %_15.i1475 = getelementptr inbounds nuw float, ptr %_54.0.i.i166, i32 %base.i1469, !dbg !11102
  %lanes.i5006.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1475, align 4, !dbg !11104, !alias.scope !11109, !noalias !11113
  %1609 = bitcast <4 x i32> %.sroa.08662.0 to <4 x float>, !dbg !11117
  %1610 = fcmp olt <4 x float> %lanes.i5006.sroa.0.0.copyload, %1609, !dbg !11121
  %1611 = select <4 x i1> %1610, <4 x float> %lanes.i5006.sroa.0.0.copyload, <4 x float> %1609, !dbg !11122
  %1612 = bitcast <4 x float> %1611 to <4 x i32>, !dbg !11123
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !11125

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1468: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1486
  %1613 = bitcast <4 x i32> %lanes.i4999.sroa.0.0.copyload to <4 x float>, !dbg !11089
  br i1 %_29.i123918119.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb19.i1240, !dbg !11126

bb19.i1240:                                       ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1468, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1459
  %end.sroa.0.0.i18121 = phi i32 [ %1619, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1459 ], [ %_220.i158, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1468 ]
  %iter.sroa.0.0.i123818120 = phi i32 [ %_30.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1459 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1468 ]
  %1614 = phi <4 x float> [ %1617, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1459 ], [ %1613, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1468 ]
  %base.i1451 = shl i32 %end.sroa.0.0.i18121, 2, !dbg !11129
  %1615 = or disjoint i32 %base.i1451, 3, !dbg !11131
  %or.cond.i1455.not = icmp ult i32 %1615, %_54.1.i.i167, !dbg !11131
  br i1 %or.cond.i1455.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1459, label %bb4.i1458, !dbg !11131, !prof !10587

bb4.i1458:                                        ; preds = %bb19.i1240
  store i32 %storemerge.i1257.lcssa2262222671, ptr %_22.i279.i, align 4
  store i32 %storemerge.i.lcssa2265222689, ptr %_22.i.i178, align 4
  store <4 x float> %1499, ptr %1200, align 16, !dbg !10588
  store <4 x float> %1508, ptr %1203, align 16, !dbg !10589
  store <16 x i8> %_4.i6611, ptr %_111.i125, align 16, !dbg !10590
  store <16 x i8> %_4.i6612, ptr %1204, align 16, !dbg !10591
  store <4 x float> %1517, ptr %1206, align 16, !dbg !10592
  store <16 x i8> %_4.i6613, ptr %_115.i128, align 16, !dbg !10593
  store <16 x i8> %_4.i6614, ptr %1207, align 16, !dbg !10594
  store <4 x float> %1526, ptr %1209, align 16, !dbg !10595
  store i32 %storemerge.i1257, ptr %_22.i279.i, align 4, !dbg !10596
  store <4 x float> %1575, ptr %1217, align 16, !dbg !10600
  store i32 %storemerge.i22639, ptr %_22.i.i178, align 4, !dbg !10604
  store <4 x float> %1493, ptr %1227, align 16, !dbg !10607
  %_5.i1452 = add i32 %base.i1451, 4, !dbg !11135
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1451, i32 noundef %_5.i1452, i32 noundef range(i32 0, 536870912) %_54.1.i.i167, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !11136, !noalias !11137
  unreachable, !dbg !11136

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1459: ; preds = %bb19.i1240
  %_30.i = add nuw i32 %iter.sroa.0.0.i123818120, 1, !dbg !11141
  %_15.i1457 = getelementptr inbounds nuw float, ptr %_54.0.i.i166, i32 %base.i1451, !dbg !11144
  %lanes.i5020.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1457, align 4, !dbg !11146, !alias.scope !11151, !noalias !11155
  %1616 = fcmp olt <4 x float> %1614, %lanes.i5020.sroa.0.0.copyload, !dbg !11159
  %1617 = select <4 x i1> %1616, <4 x float> %1614, <4 x float> %lanes.i5020.sroa.0.0.copyload, !dbg !11163
  store <4 x float> %1617, ptr %_15.i1457, align 4, !dbg !11164, !alias.scope !11170, !noalias !11174
  %1618 = icmp eq i32 %end.sroa.0.0.i18121, 0, !dbg !11180
  %spec.store.select.i1244 = select i1 %1618, i32 %ring.i52, i32 %end.sroa.0.0.i18121, !dbg !11180
  %1619 = add i32 %spec.store.select.i1244, -1, !dbg !11181
  %exitcond20538.not = icmp eq i32 %_30.i, %_18.i251.i, !dbg !11182
  br i1 %exitcond20538.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb19.i1240, !dbg !11126

_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1459, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1468, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1477
  %.sroa.08662.1 = phi <4 x i32> [ %1612, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1477 ], [ %.sroa.08662.0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1468 ], [ %.sroa.08662.0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1459 ], !dbg !11184
  %storemerge.i = phi i32 [ %_15.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1477 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1468 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1459 ], !dbg !11185
  %1620 = bitcast <4 x i32> %.sroa.08662.1 to <4 x float>, !dbg !11186
  %1621 = fmul <4 x float> %1620, splat (float 1.638400e+04), !dbg !11190
  %1622 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %1621), !dbg !11191
  %1623 = fmul <4 x float> %1622, splat (float 0x3F10000000000000), !dbg !11195
  %base.i1496 = shl i32 %_222.i160, 2, !dbg !11199
  %1624 = or disjoint i32 %base.i1496, 3, !dbg !11201
  %or.cond.i1500.not = icmp ult i32 %1624, %_56.1.i.i184, !dbg !11201
  br i1 %or.cond.i1500.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1504, label %bb4.i1503, !dbg !11201, !prof !10587

bb4.i1503:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
  store i32 %storemerge.i1257.lcssa2262222671, ptr %_22.i279.i, align 4
  store i32 %storemerge.i.lcssa2265222689, ptr %_22.i.i178, align 4
  store <4 x float> %1499, ptr %1200, align 16, !dbg !10588
  store <4 x float> %1508, ptr %1203, align 16, !dbg !10589
  store <16 x i8> %_4.i6611, ptr %_111.i125, align 16, !dbg !10590
  store <16 x i8> %_4.i6612, ptr %1204, align 16, !dbg !10591
  store <4 x float> %1517, ptr %1206, align 16, !dbg !10592
  store <16 x i8> %_4.i6613, ptr %_115.i128, align 16, !dbg !10593
  store <16 x i8> %_4.i6614, ptr %1207, align 16, !dbg !10594
  store <4 x float> %1526, ptr %1209, align 16, !dbg !10595
  store i32 %storemerge.i1257, ptr %_22.i279.i, align 4, !dbg !10596
  store <4 x float> %1575, ptr %1217, align 16, !dbg !10600
  store i32 %storemerge.i, ptr %_22.i.i178, align 4, !dbg !10604
  store <4 x float> %1493, ptr %1227, align 16, !dbg !10607
  %_5.i1497 = add i32 %base.i1496, 4, !dbg !11205
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1496, i32 noundef %_5.i1497, i32 noundef range(i32 0, 536870912) %_56.1.i.i184, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !11206, !noalias !11207
  unreachable, !dbg !11206

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1504: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
  %_15.i1502 = getelementptr inbounds nuw float, ptr %_56.0.i.i183, i32 %base.i1496, !dbg !11211
  %lanes.i4985.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1502, align 4, !dbg !11213, !alias.scope !11218, !noalias !11222
  %1625 = fadd <4 x float> %1623, %1493, !dbg !11226
  %1626 = fsub <4 x float> %1625, %lanes.i4985.sroa.0.0.copyload, !dbg !11230
  %_8.not.i4.i.i193 = icmp ugt i32 %_7.i10.i268.i, %_56.1.i.i184
  br i1 %_8.not.i4.i.i193, label %bb4.i7.i.i225, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i195, !dbg !11234, !prof !4694

bb4.i7.i.i225:                                    ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1504
  store i32 %storemerge.i1257.lcssa2262222671, ptr %_22.i279.i, align 4
  store i32 %storemerge.i.lcssa2265222689, ptr %_22.i.i178, align 4
  store <4 x float> %1499, ptr %1200, align 16, !dbg !10588
  store <4 x float> %1508, ptr %1203, align 16, !dbg !10589
  store <16 x i8> %_4.i6611, ptr %_111.i125, align 16, !dbg !10590
  store <16 x i8> %_4.i6612, ptr %1204, align 16, !dbg !10591
  store <4 x float> %1517, ptr %1206, align 16, !dbg !10592
  store <16 x i8> %_4.i6613, ptr %_115.i128, align 16, !dbg !10593
  store <16 x i8> %_4.i6614, ptr %1207, align 16, !dbg !10594
  store <4 x float> %1526, ptr %1209, align 16, !dbg !10595
  store i32 %storemerge.i1257, ptr %_22.i279.i, align 4, !dbg !10596
  store <4 x float> %1575, ptr %1217, align 16, !dbg !10600
  store i32 %storemerge.i, ptr %_22.i.i178, align 4, !dbg !10604
  store <4 x float> %1626, ptr %1227, align 16, !dbg !10607
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i9.i267.i, i32 noundef %_7.i10.i268.i, i32 noundef range(i32 0, 536870912) %_56.1.i.i184, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #33, !dbg !11239, !noalias !11240
  unreachable, !dbg !11239

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i195: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1504
  %_17.i6.i.i196 = getelementptr inbounds nuw float, ptr %_56.0.i.i183, i32 %base.i9.i267.i, !dbg !11244
  store <4 x float> %1623, ptr %_17.i6.i.i196, align 4, !dbg !11246, !alias.scope !11251, !noalias !11255
  %_41.i.i20115391 = load <4 x float>, ptr %1229, align 16, !dbg !11259
  %1627 = fdiv <4 x float> %1626, %_37.i.i19815390, !dbg !11260
  %1628 = fsub <4 x float> splat (float 1.000000e+00), %1627, !dbg !11264
  %1629 = fsub <4 x float> %1628, %_41.i.i20115391, !dbg !11268
  %1630 = bitcast <16 x i8> %_4.i6615 to <4 x float>, !dbg !11272
  %1631 = fmul <4 x float> %1629, %1630, !dbg !11276
  %1632 = fadd <4 x float> %_41.i.i20115391, %1631, !dbg !11277
  %1633 = fcmp olt <4 x float> %1632, %1628, !dbg !11280
  %1634 = select <4 x i1> %1633, <4 x float> %1628, <4 x float> %1632, !dbg !11284
  %1635 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1634), !dbg !11285
  %1636 = fcmp uge <4 x float> %1635, splat (float 0x3BC79CA100000000), !dbg !11290
  %1637 = bitcast <4 x float> %1634 to <4 x i32>, !dbg !11295
  %1638 = select <4 x i1> %1636, <4 x i32> %1637, <4 x i32> zeroinitializer, !dbg !11295
  store <4 x i32> %1638, ptr %1229, align 16, !dbg !11298
  %_6.not.i1490 = icmp ugt i32 %_5.i1506, %_58.1.i.i212
  br i1 %_6.not.i1490, label %bb4.i1494, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1495, !dbg !11299, !prof !4694

bb4.i1494:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i195
  store i32 %storemerge.i1257.lcssa2262222671, ptr %_22.i279.i, align 4
  store i32 %storemerge.i.lcssa2265222689, ptr %_22.i.i178, align 4
  store <4 x float> %1499, ptr %1200, align 16, !dbg !10588
  store <4 x float> %1508, ptr %1203, align 16, !dbg !10589
  store <16 x i8> %_4.i6611, ptr %_111.i125, align 16, !dbg !10590
  store <16 x i8> %_4.i6612, ptr %1204, align 16, !dbg !10591
  store <4 x float> %1517, ptr %1206, align 16, !dbg !10592
  store <16 x i8> %_4.i6613, ptr %_115.i128, align 16, !dbg !10593
  store <16 x i8> %_4.i6614, ptr %1207, align 16, !dbg !10594
  store <4 x float> %1526, ptr %1209, align 16, !dbg !10595
  store i32 %storemerge.i1257, ptr %_22.i279.i, align 4, !dbg !10596
  store <4 x float> %1575, ptr %1217, align 16, !dbg !10600
  store i32 %storemerge.i, ptr %_22.i.i178, align 4, !dbg !10604
  store <4 x float> %1626, ptr %1227, align 16, !dbg !10607
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1505, i32 noundef %_5.i1506, i32 noundef range(i32 0, 536870912) %_58.1.i.i212, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !11304, !noalias !11305
  unreachable, !dbg !11304

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1495: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i195
  %1639 = bitcast <4 x i32> %1638 to <4 x float>, !dbg !11309
  %1640 = fsub <4 x float> splat (float 1.000000e+00), %1639, !dbg !11313
  %_15.i1493 = getelementptr inbounds nuw float, ptr %_58.0.i.i211, i32 %base.i1505, !dbg !11314
  %lanes.i4992.sroa.0.0.copyload = load <4 x i32>, ptr %_15.i1493, align 4, !dbg !11316, !alias.scope !11321, !noalias !11325
  store <4 x i32> %lanes.i5329.sroa.0.0.copyload, ptr %_15.i1493, align 4, !dbg !11329, !alias.scope !11335, !noalias !11339
  %1641 = bitcast <4 x i32> %lanes.i4992.sroa.0.0.copyload to <4 x float>, !dbg !11345
  %1642 = fmul <4 x float> %1640, %1641, !dbg !11349
  %1643 = bitcast <4 x i32> %lanes.i4992.sroa.0.0.copyload to <16 x i8>, !dbg !11350
  %1644 = bitcast <4 x float> %1642 to <16 x i8>, !dbg !11354
  %_4.i6647 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1643, <16 x i8> %1644, <16 x i8> %1222), !dbg !11355
  store <16 x i8> %_4.i6647, ptr %data.i5.i.i.i.i.i, align 4, !dbg !11356, !alias.scope !11361, !noalias !11365
  %exitcond20550.not = icmp eq i32 %_206.0.i123, %1492, !dbg !10298
  br i1 %exitcond20550.not, label %bb67.i227.loopexit, label %bb68.i107, !dbg !10298

bb67.i227.loopexit:                               ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1495
  store <4 x float> %1499, ptr %1200, align 16, !dbg !10588
  store <4 x float> %1508, ptr %1203, align 16, !dbg !10589
  store <16 x i8> %_4.i6611, ptr %_111.i125, align 16, !dbg !10590
  store <16 x i8> %_4.i6612, ptr %1204, align 16, !dbg !10591
  store <4 x float> %1517, ptr %1206, align 16, !dbg !10592
  store <16 x i8> %_4.i6613, ptr %_115.i128, align 16, !dbg !10593
  store <16 x i8> %_4.i6614, ptr %1207, align 16, !dbg !10594
  store <4 x float> %1526, ptr %1209, align 16, !dbg !10595
  store <4 x float> %1575, ptr %1217, align 16, !dbg !10600
  store <4 x float> %1626, ptr %1227, align 16, !dbg !10607
  br label %bb67.i227, !dbg !11369

bb67.i227:                                        ; preds = %bb67.i227.loopexit, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit
  %storemerge.i.lcssa2265222688 = phi i32 [ %storemerge.i, %bb67.i227.loopexit ], [ %storemerge.i.lcssa2265222689, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ]
  %storemerge.i1257.lcssa2262222670 = phi i32 [ %storemerge.i1257, %bb67.i227.loopexit ], [ %storemerge.i1257.lcssa2262222671, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ]
  %_139.i228 = add i32 %run.sroa.0.0.i, %ring_cursor.sroa.0.1.i7418126, !dbg !11369
  %_212.not.i229 = icmp ult i32 %_139.i228, %ring.i52, !dbg !11370
  %1645 = select i1 %_212.not.i229, i32 0, i32 %ring.i52, !dbg !11370
  %ring_cursor.sroa.0.2.i230 = sub nuw i32 %_139.i228, %1645, !dbg !11370
  %_141.i231 = add i32 %run.sroa.0.0.i, %main_cursor.sroa.0.1.i7518127, !dbg !11373
  %_223.not.i232 = icmp ult i32 %_141.i231, %main.i53, !dbg !11374
  %1646 = select i1 %_223.not.i232, i32 0, i32 %main.i53, !dbg !11374
  %main_cursor.sroa.0.2.i233 = sub nuw i32 %_141.i231, %1646, !dbg !11374
  %_59.i77 = icmp ult i32 %_83.i94, %spec.store.select.i68, !dbg !10187
  br i1 %_59.i77, label %bb20.i78, label %bb15.i61.loopexit.loopexit, !dbg !10187

_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %bb15.i61.loopexit, %bb7.i
  %ring_cursor.sroa.0.0.i62.lcssa = phi i32 [ %_37.i55, %bb7.i ], [ %ring_cursor.sroa.0.1.i74.lcssa, %bb15.i61.loopexit ], !dbg !9194
  %main_cursor.sroa.0.0.i63.lcssa = phi i32 [ %_36.i54, %bb7.i ], [ %main_cursor.sroa.0.1.i75.lcssa, %bb15.i61.loopexit ], !dbg !9191
  %left_prefix.i439 = load <4 x i32>, ptr %uniform_left.i39, align 16, !dbg !11376, !noalias !9198
  %1647 = getelementptr inbounds nuw i8, ptr %uniform_left.i39, i32 16, !dbg !11377
  %left_phase.i440 = load i32, ptr %1647, align 16, !dbg !11377, !noalias !9198, !noundef !10
  %right_prefix.i441 = load <4 x i32>, ptr %uniform_right.i38, align 16, !dbg !11378, !noalias !9198
  %1648 = getelementptr inbounds nuw i8, ptr %uniform_right.i38, i32 16, !dbg !11379
  %right_phase.i442 = load i32, ptr %1648, align 16, !dbg !11379, !noalias !9198, !noundef !10
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_right.i38), !dbg !11380, !noalias !9198
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i39), !dbg !11381, !noalias !9198
  %1649 = getelementptr inbounds nuw i8, ptr %self, i32 968, !dbg !11382
  %_236.1.i444 = load i32, ptr %1649, align 4, !dbg !11382, !alias.scope !9165, !noalias !11384, !noundef !10
  %_8.i5984 = icmp samesign ugt i32 %_236.1.i444, 3, !dbg !11385
  br i1 %_8.i5984, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5987, label %bb2.i5985, !dbg !11385, !prof !1153

bb2.i5985:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_236.1.i444, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !11390, !noalias !11391
  unreachable, !dbg !11390

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5987: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
  %1650 = getelementptr inbounds nuw i8, ptr %self, i32 964, !dbg !11382
  %_236.0.i443 = load ptr, ptr %1650, align 4, !dbg !11382, !alias.scope !9165, !noalias !11384, !nonnull !10, !noundef !10
  store <4 x i32> %left_prefix.i439, ptr %_236.0.i443, align 4, !dbg !11395, !alias.scope !11399, !noalias !11403
  %_237.0.i445 = load ptr, ptr %68, align 4, !dbg !11405, !alias.scope !9165, !noalias !11384, !nonnull !10, !noundef !10
  %_237.1.i446 = load i32, ptr %69, align 4, !dbg !11405, !alias.scope !9165, !noalias !11384, !noundef !10
  %1651 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i440), !dbg !11406
  br i1 %1651, label %bb2.i6654, label %bb6.i6650, !dbg !11406

bb6.i6650:                                        ; preds = %bb2.i6654, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5987
  %end_or_len.idx.i = shl nuw nsw i32 %_237.1.i446, 2, !dbg !11410
  %end_or_len.i = getelementptr inbounds nuw i8, ptr %_237.0.i445, i32 %end_or_len.idx.i, !dbg !11410
  %_293.i = icmp eq i32 %_237.1.i446, 0, !dbg !11419
  br i1 %_293.i, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i6651, !dbg !11428

bb2.i6654:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5987
  %bytes1.sroa.0.0.zext.i = and i32 %left_phase.i440, 255, !dbg !11429
  %bytes1.sroa.0.0.isplat.i = mul nuw i32 %bytes1.sroa.0.0.zext.i, 16843009, !dbg !11429
  %_5.i6655 = icmp eq i32 %left_phase.i440, %bytes1.sroa.0.0.isplat.i, !dbg !11430
  br i1 %_5.i6655, label %bb3.i6656, label %bb6.i6650, !dbg !11430

bb3.i6656:                                        ; preds = %bb2.i6654
  %bytes.sroa.0.0.extract.trunc.i = trunc i32 %left_phase.i440 to i8, !dbg !11431
  %1652 = shl nuw nsw i32 %_237.1.i446, 2, !dbg !11434
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %_237.0.i445, i8 %bytes.sroa.0.0.extract.trunc.i, i32 %1652, i1 false), !dbg !11434, !alias.scope !11435, !noalias !9169
  br label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, !dbg !11438

bb10.i6651:                                       ; preds = %bb6.i6650, %bb10.i6651
  %iter.sroa.0.04.i = phi ptr [ %_38.i6652, %bb10.i6651 ], [ %_237.0.i445, %bb6.i6650 ]
  %_38.i6652 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i, i32 4, !dbg !11439
  store i32 %left_phase.i440, ptr %iter.sroa.0.04.i, align 4, !dbg !11442, !alias.scope !11435, !noalias !9169
  %_29.i6653 = icmp eq ptr %_38.i6652, %end_or_len.i, !dbg !11419
  br i1 %_29.i6653, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i6651, !dbg !11428

_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit: ; preds = %bb10.i6651, %bb6.i6650, %bb3.i6656
  %1653 = getelementptr inbounds nuw i8, ptr %self, i32 1068, !dbg !11444
  %_238.1.i448 = load i32, ptr %1653, align 4, !dbg !11444, !alias.scope !9167, !noalias !11445, !noundef !10
  %_8.i5979 = icmp samesign ugt i32 %_238.1.i448, 3, !dbg !11446
  br i1 %_8.i5979, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5982, label %bb2.i5980, !dbg !11446, !prof !1153

bb2.i5980:                                        ; preds = %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_238.1.i448, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !11451, !noalias !11452
  unreachable, !dbg !11451

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5982: ; preds = %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit
  %1654 = getelementptr inbounds nuw i8, ptr %self, i32 1064, !dbg !11444
  %_238.0.i447 = load ptr, ptr %1654, align 4, !dbg !11444, !alias.scope !9167, !noalias !11445, !nonnull !10, !noundef !10
  store <4 x i32> %right_prefix.i441, ptr %_238.0.i447, align 4, !dbg !11456, !alias.scope !11460, !noalias !11464
  %_239.0.i449 = load ptr, ptr %77, align 4, !dbg !11466, !alias.scope !9167, !noalias !11445, !nonnull !10, !noundef !10
  %_239.1.i450 = load i32, ptr %78, align 4, !dbg !11466, !alias.scope !9167, !noalias !11445, !noundef !10
  %1655 = tail call i1 @llvm.is.constant.i32(i32 %right_phase.i442), !dbg !11467
  br i1 %1655, label %bb2.i6666, label %bb6.i6658, !dbg !11467

bb6.i6658:                                        ; preds = %bb2.i6666, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5982
  %end_or_len.idx.i6659 = shl nuw nsw i32 %_239.1.i450, 2, !dbg !11470
  %end_or_len.i6660 = getelementptr inbounds nuw i8, ptr %_239.0.i449, i32 %end_or_len.idx.i6659, !dbg !11470
  %_293.i6661 = icmp eq i32 %_239.1.i450, 0, !dbg !11474
  br i1 %_293.i6661, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit6672, label %bb10.i6662, !dbg !11477

bb2.i6666:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit5982
  %bytes1.sroa.0.0.zext.i6667 = and i32 %right_phase.i442, 255, !dbg !11478
  %bytes1.sroa.0.0.isplat.i6668 = mul nuw i32 %bytes1.sroa.0.0.zext.i6667, 16843009, !dbg !11478
  %_5.i6669 = icmp eq i32 %right_phase.i442, %bytes1.sroa.0.0.isplat.i6668, !dbg !11479
  br i1 %_5.i6669, label %bb3.i6670, label %bb6.i6658, !dbg !11479

bb3.i6670:                                        ; preds = %bb2.i6666
  %bytes.sroa.0.0.extract.trunc.i6671 = trunc i32 %right_phase.i442 to i8, !dbg !11480
  %1656 = shl nuw nsw i32 %_239.1.i450, 2, !dbg !11482
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %_239.0.i449, i8 %bytes.sroa.0.0.extract.trunc.i6671, i32 %1656, i1 false), !dbg !11482, !alias.scope !11483, !noalias !9169
  br label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit6672, !dbg !11486

bb10.i6662:                                       ; preds = %bb6.i6658, %bb10.i6662
  %iter.sroa.0.04.i6663 = phi ptr [ %_38.i6664, %bb10.i6662 ], [ %_239.0.i449, %bb6.i6658 ]
  %_38.i6664 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i6663, i32 4, !dbg !11487
  store i32 %right_phase.i442, ptr %iter.sroa.0.04.i6663, align 4, !dbg !11489, !alias.scope !11483, !noalias !9169
  %_29.i6665 = icmp eq ptr %_38.i6664, %end_or_len.i6660, !dbg !11474
  br i1 %_29.i6665, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit6672, label %bb10.i6662, !dbg !11477

_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit6672: ; preds = %bb10.i6662, %bb6.i6658, %bb3.i6670
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_(ptr noalias noundef readonly align 16 captures(none) dereferenceable(368) %hot_left.i44, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33) #32, !dbg !11490
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_(ptr noalias noundef readonly align 16 captures(none) dereferenceable(368) %hot_right.i43, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34) #32, !dbg !11491
  store i32 %main_cursor.sroa.0.0.i63.lcssa, ptr %_35, align 4, !dbg !11492, !alias.scope !9169, !noalias !9193
  store i32 %ring_cursor.sroa.0.0.i62.lcssa, ptr %86, align 4, !dbg !11493, !alias.scope !9169, !noalias !9193
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i40), !dbg !11494, !noalias !9198
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i41), !dbg !11495, !noalias !9198
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !9162

bb6.i:                                            ; preds = %bb5.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11496), !dbg !11499
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11500), !dbg !11499
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11502), !dbg !11499
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11504), !dbg !11499
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11506), !dbg !11499
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_(ptr noalias noundef align 16 captures(none) dereferenceable(368) %hot_left.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #32, !dbg !11508
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_(ptr noalias noundef align 16 captures(none) dereferenceable(368) %hot_right.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #32, !dbg !11512
  %1657 = load i8, ptr %82, align 16, !dbg !11514, !range !4765, !alias.scope !11496, !noalias !11518, !noundef !10
  %1658 = load i8, ptr %83, align 1, !dbg !11521, !range !4765, !alias.scope !11496, !noalias !11518, !noundef !10
  %ring.i = load i32, ptr %84, align 4, !dbg !11523, !alias.scope !11500, !noalias !11525, !noundef !10
  %main.i = load i32, ptr %85, align 4, !dbg !11526, !alias.scope !11500, !noalias !11525, !noundef !10
  %_36.i = load i32, ptr %_35, align 4, !dbg !11528, !alias.scope !11506, !noalias !11530, !noundef !10
  %_37.i = load i32, ptr %86, align 4, !dbg !11531, !alias.scope !11506, !noalias !11530, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i), !dbg !11533, !noalias !11535
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i, i8 0, i32 1024, i1 false), !noalias !11535
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i), !dbg !11536, !noalias !11535
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i, i8 0, i32 1024, i1 false), !noalias !11535
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i), !dbg !11538, !noalias !11535
; call <true_peak_limiter::UniformHot<wide::f32x4_::f32x4>>::new
  call fastcc void @_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E3newB5_(ptr noalias noundef align 16 captures(none) dereferenceable(64) %uniform_left.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33, i32 %ring.i, i32 %main.i) #32, !dbg !11540
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_right.i), !dbg !11541, !noalias !11535
  %_32.val6276 = load i32, ptr %84, align 4, !dbg !11543, !noundef !10
  %_32.val6277 = load i32, ptr %85, align 4, !dbg !11543, !noundef !10
; call <true_peak_limiter::UniformHot<wide::f32x4_::f32x4>>::new
  call fastcc void @_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E3newB5_(ptr noalias noundef align 16 captures(none) dereferenceable(64) %uniform_right.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34, i32 %_32.val6276, i32 %_32.val6277) #32, !dbg !11543
  %_162.not.i18465 = icmp eq i32 %frames, 0, !dbg !11544
  br i1 %_162.not.i18465, label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb44.i.lr.ph, !dbg !11544

bb44.i.lr.ph:                                     ; preds = %bb6.i
  %_33.i = trunc nuw i8 %1658 to i1, !dbg !11521
  %spec.store.select25.i = select i1 %_33.i, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !11521
  %_32.i = trunc nuw i8 %1657 to i1, !dbg !11514
  %link.sroa.0.0.i = select i1 %_32.i, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !11514
  %d9.i6673 = lshr i32 %frames, 5, !dbg !11554
  %r2.i6674 = and i32 %frames, 31, !dbg !11561
  %_19.not.i6675 = icmp ne i32 %r2.i6674, 0, !dbg !11562
  %1659 = zext i1 %_19.not.i6675 to i32, !dbg !11562
  %yield_count.sroa.0.0.i6676 = add nuw nsw i32 %d9.i6673, %1659, !dbg !11562
  %history.i41.i.sroa.7.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 16
  %history.i41.i.sroa.10.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 32
  %history.i41.i.sroa.13.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 48
  %history.i41.i.sroa.16.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 64
  %history.i41.i.sroa.19.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 80
  %history.i41.i.sroa.22.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 96
  %history.i41.i.sroa.26.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 112
  %history.i41.i.sroa.29.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 128
  %history.i41.i.sroa.32.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 144
  %history.i41.i.sroa.35.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 160
  %history.i41.i.sroa.38.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 176
  %1660 = getelementptr inbounds nuw i8, ptr %self, i32 32
  %1661 = getelementptr inbounds nuw i8, ptr %self, i32 48
  %1662 = getelementptr inbounds nuw i8, ptr %self, i32 64
  %row1.i.i.i78.i = getelementptr inbounds nuw i8, ptr %self, i32 80
  %1663 = getelementptr inbounds nuw i8, ptr %self, i32 96
  %1664 = getelementptr inbounds nuw i8, ptr %self, i32 112
  %1665 = getelementptr inbounds nuw i8, ptr %self, i32 128
  %row3.i.i.i92.i = getelementptr inbounds nuw i8, ptr %self, i32 144
  %1666 = getelementptr inbounds nuw i8, ptr %self, i32 160
  %1667 = getelementptr inbounds nuw i8, ptr %self, i32 176
  %1668 = getelementptr inbounds nuw i8, ptr %self, i32 192
  %row5.i.i.i106.i = getelementptr inbounds nuw i8, ptr %self, i32 208
  %1669 = getelementptr inbounds nuw i8, ptr %self, i32 224
  %1670 = getelementptr inbounds nuw i8, ptr %self, i32 240
  %1671 = getelementptr inbounds nuw i8, ptr %self, i32 256
  %row7.i.i.i120.i = getelementptr inbounds nuw i8, ptr %self, i32 272
  %1672 = getelementptr inbounds nuw i8, ptr %self, i32 288
  %1673 = getelementptr inbounds nuw i8, ptr %self, i32 304
  %1674 = getelementptr inbounds nuw i8, ptr %self, i32 320
  %row9.i.i.i134.i = getelementptr inbounds nuw i8, ptr %self, i32 336
  %1675 = getelementptr inbounds nuw i8, ptr %self, i32 352
  %1676 = getelementptr inbounds nuw i8, ptr %self, i32 368
  %1677 = getelementptr inbounds nuw i8, ptr %self, i32 384
  %row11.i.i.i148.i = getelementptr inbounds nuw i8, ptr %self, i32 400
  %1678 = getelementptr inbounds nuw i8, ptr %self, i32 416
  %1679 = getelementptr inbounds nuw i8, ptr %self, i32 432
  %1680 = getelementptr inbounds nuw i8, ptr %self, i32 448
  %row13.i.i.i162.i = getelementptr inbounds nuw i8, ptr %self, i32 464
  %1681 = getelementptr inbounds nuw i8, ptr %self, i32 480
  %1682 = getelementptr inbounds nuw i8, ptr %self, i32 496
  %1683 = getelementptr inbounds nuw i8, ptr %self, i32 512
  %row15.i.i.i176.i = getelementptr inbounds nuw i8, ptr %self, i32 528
  %1684 = getelementptr inbounds nuw i8, ptr %self, i32 544
  %1685 = getelementptr inbounds nuw i8, ptr %self, i32 560
  %1686 = getelementptr inbounds nuw i8, ptr %self, i32 576
  %row17.i.i.i190.i = getelementptr inbounds nuw i8, ptr %self, i32 592
  %1687 = getelementptr inbounds nuw i8, ptr %self, i32 608
  %1688 = getelementptr inbounds nuw i8, ptr %self, i32 624
  %1689 = getelementptr inbounds nuw i8, ptr %self, i32 640
  %row19.i.i.i204.i = getelementptr inbounds nuw i8, ptr %self, i32 656
  %1690 = getelementptr inbounds nuw i8, ptr %self, i32 672
  %1691 = getelementptr inbounds nuw i8, ptr %self, i32 688
  %1692 = getelementptr inbounds nuw i8, ptr %self, i32 704
  %row21.i.i.i218.i = getelementptr inbounds nuw i8, ptr %self, i32 720
  %1693 = getelementptr inbounds nuw i8, ptr %self, i32 736
  %1694 = getelementptr inbounds nuw i8, ptr %self, i32 752
  %1695 = getelementptr inbounds nuw i8, ptr %self, i32 768
  %history.i.i.sroa.7.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 16
  %history.i.i.sroa.10.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 32
  %history.i.i.sroa.13.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 48
  %history.i.i.sroa.16.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 64
  %history.i.i.sroa.19.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 80
  %history.i.i.sroa.22.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 96
  %history.i.i.sroa.26.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 112
  %history.i.i.sroa.29.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 128
  %history.i.i.sroa.32.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 144
  %history.i.i.sroa.35.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 160
  %history.i.i.sroa.38.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 176
  %1696 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 20
  %_68.i.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 24
  %_68.i.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 28
  %1697 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 20
  %_69.i.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 24
  %_69.i.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 28
  %_110.i = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 192
  %_111.i = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 256
  %_115.i = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 192
  %_116.i = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 256
  %1698 = bitcast <4 x i32> %link.sroa.0.0.i to <16 x i8>
  %1699 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 32
  %1700 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 36
  %_22.i278.i = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 16
  %1701 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 40
  %1702 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 44
  %1703 = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 336
  %1704 = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 352
  %1705 = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 320
  %1706 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 52
  %1707 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 48
  %1708 = bitcast <4 x i32> %spec.store.select25.i to <16 x i8>
  %1709 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 32
  %1710 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 36
  %_22.i.i = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 16
  %1711 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 40
  %1712 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 44
  %1713 = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 336
  %1714 = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 352
  %1715 = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 320
  %1716 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 52
  %1717 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 48
  %hot_left.i.promoted = load <4 x i32>, ptr %hot_left.i, align 16
  %history.i41.i.sroa.7.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i41.i.sroa.7.0.hot_left.i.sroa_idx, align 16
  %history.i41.i.sroa.10.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i41.i.sroa.10.0.hot_left.i.sroa_idx, align 16
  %history.i41.i.sroa.13.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i41.i.sroa.13.0.hot_left.i.sroa_idx, align 16
  %history.i41.i.sroa.16.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i41.i.sroa.16.0.hot_left.i.sroa_idx, align 16
  %history.i41.i.sroa.19.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i41.i.sroa.19.0.hot_left.i.sroa_idx, align 16
  %history.i41.i.sroa.22.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i41.i.sroa.22.0.hot_left.i.sroa_idx, align 16
  %history.i41.i.sroa.26.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i41.i.sroa.26.0.hot_left.i.sroa_idx, align 16
  %history.i41.i.sroa.29.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i41.i.sroa.29.0.hot_left.i.sroa_idx, align 16
  %history.i41.i.sroa.32.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i41.i.sroa.32.0.hot_left.i.sroa_idx, align 16
  %history.i41.i.sroa.35.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i41.i.sroa.35.0.hot_left.i.sroa_idx, align 16
  %history.i41.i.sroa.38.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i41.i.sroa.38.0.hot_left.i.sroa_idx, align 16
  %hot_right.i.promoted = load <4 x i32>, ptr %hot_right.i, align 16
  %history.i.i.sroa.7.0.hot_right.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16
  %history.i.i.sroa.10.0.hot_right.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16
  %history.i.i.sroa.13.0.hot_right.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16
  %history.i.i.sroa.16.0.hot_right.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16
  %history.i.i.sroa.19.0.hot_right.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16
  %history.i.i.sroa.22.0.hot_right.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16
  %history.i.i.sroa.26.0.hot_right.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16
  %history.i.i.sroa.29.0.hot_right.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16
  %history.i.i.sroa.32.0.hot_right.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16
  %history.i.i.sroa.35.0.hot_right.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16
  %history.i.i.sroa.38.0.hot_right.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16
  %_22.i278.i.promoted = load i32, ptr %_22.i278.i, align 4
  %.promoted23206 = load <4 x float>, ptr %1703, align 16
  %_22.i.i.promoted = load i32, ptr %_22.i.i, align 4
  %.promoted23249 = load <4 x float>, ptr %1713, align 16
  br label %bb44.i, !dbg !11544

bb15.i.loopexit:                                  ; preds = %bb67.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i
  %.lcssa1831018430.lcssa23250 = phi <4 x float> [ %.lcssa1831018430.lcssa23251, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i ], [ %.lcssa1831018430, %bb67.i ]
  %storemerge.i1291.lcssa1828018394.lcssa23228 = phi i32 [ %storemerge.i1291.lcssa1828018394.lcssa23229, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i ], [ %storemerge.i1291.lcssa1828018394, %bb67.i ]
  %.lcssa1825318358.lcssa23207 = phi <4 x float> [ %.lcssa1825318358.lcssa23208, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i ], [ %.lcssa1825318358, %bb67.i ]
  %storemerge.i1325.lcssa1822318322.lcssa23185 = phi i32 [ %storemerge.i1325.lcssa1822318322.lcssa23186, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i ], [ %storemerge.i1325.lcssa1822318322, %bb67.i ]
  %ring_cursor.sroa.0.1.i.lcssa = phi i32 [ %ring_cursor.sroa.0.0.i18466, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i ], [ %ring_cursor.sroa.0.2.i, %bb67.i ], !dbg !11563
  %main_cursor.sroa.0.1.i.lcssa = phi i32 [ %main_cursor.sroa.0.0.i18467, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i ], [ %main_cursor.sroa.0.2.i, %bb67.i ], !dbg !11564
  %_162.not.i = icmp eq i32 %1719, 0, !dbg !11544
  %indvars.iv.next20552 = add i32 %indvars.iv20551, -32, !dbg !11544
  br i1 %_162.not.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit, label %bb44.i, !dbg !11544

bb44.i:                                           ; preds = %bb44.i.lr.ph, %bb15.i.loopexit
  %.lcssa1831018430.lcssa23251 = phi <4 x float> [ %.promoted23249, %bb44.i.lr.ph ], [ %.lcssa1831018430.lcssa23250, %bb15.i.loopexit ]
  %storemerge.i1291.lcssa1828018394.lcssa23229 = phi i32 [ %_22.i.i.promoted, %bb44.i.lr.ph ], [ %storemerge.i1291.lcssa1828018394.lcssa23228, %bb15.i.loopexit ]
  %.lcssa1825318358.lcssa23208 = phi <4 x float> [ %.promoted23206, %bb44.i.lr.ph ], [ %.lcssa1825318358.lcssa23207, %bb15.i.loopexit ]
  %storemerge.i1325.lcssa1822318322.lcssa23186 = phi i32 [ %_22.i278.i.promoted, %bb44.i.lr.ph ], [ %storemerge.i1325.lcssa1822318322.lcssa23185, %bb15.i.loopexit ]
  %history.i.i.sroa.38.0.lcssa23165 = phi <4 x i32> [ %history.i.i.sroa.38.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.38.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.35.0.lcssa23145 = phi <4 x i32> [ %history.i.i.sroa.35.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.35.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.32.0.lcssa23125 = phi <4 x i32> [ %history.i.i.sroa.32.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.32.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.29.0.lcssa23105 = phi <4 x i32> [ %history.i.i.sroa.29.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.29.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.26.0.lcssa23085 = phi <4 x i32> [ %history.i.i.sroa.26.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.26.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.22.0.lcssa23065 = phi <4 x i32> [ %history.i.i.sroa.22.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.22.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.19.0.lcssa23045 = phi <4 x i32> [ %history.i.i.sroa.19.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.19.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.16.0.lcssa23025 = phi <4 x i32> [ %history.i.i.sroa.16.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.16.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.13.0.lcssa23005 = phi <4 x i32> [ %history.i.i.sroa.13.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.13.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.10.0.lcssa22985 = phi <4 x i32> [ %history.i.i.sroa.10.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.10.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.7.0.lcssa22965 = phi <4 x i32> [ %history.i.i.sroa.7.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.7.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.0.0.lcssa22945 = phi <4 x i32> [ %hot_right.i.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i41.i.sroa.38.0.lcssa22925 = phi <4 x i32> [ %history.i41.i.sroa.38.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i41.i.sroa.38.0.lcssa, %bb15.i.loopexit ]
  %history.i41.i.sroa.35.0.lcssa22905 = phi <4 x i32> [ %history.i41.i.sroa.35.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i41.i.sroa.35.0.lcssa, %bb15.i.loopexit ]
  %history.i41.i.sroa.32.0.lcssa22885 = phi <4 x i32> [ %history.i41.i.sroa.32.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i41.i.sroa.32.0.lcssa, %bb15.i.loopexit ]
  %history.i41.i.sroa.29.0.lcssa22865 = phi <4 x i32> [ %history.i41.i.sroa.29.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i41.i.sroa.29.0.lcssa, %bb15.i.loopexit ]
  %history.i41.i.sroa.26.0.lcssa22845 = phi <4 x i32> [ %history.i41.i.sroa.26.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i41.i.sroa.26.0.lcssa, %bb15.i.loopexit ]
  %history.i41.i.sroa.22.0.lcssa22825 = phi <4 x i32> [ %history.i41.i.sroa.22.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i41.i.sroa.22.0.lcssa, %bb15.i.loopexit ]
  %history.i41.i.sroa.19.0.lcssa22805 = phi <4 x i32> [ %history.i41.i.sroa.19.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i41.i.sroa.19.0.lcssa, %bb15.i.loopexit ]
  %history.i41.i.sroa.16.0.lcssa22785 = phi <4 x i32> [ %history.i41.i.sroa.16.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i41.i.sroa.16.0.lcssa, %bb15.i.loopexit ]
  %history.i41.i.sroa.13.0.lcssa22765 = phi <4 x i32> [ %history.i41.i.sroa.13.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i41.i.sroa.13.0.lcssa, %bb15.i.loopexit ]
  %history.i41.i.sroa.10.0.lcssa22745 = phi <4 x i32> [ %history.i41.i.sroa.10.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i41.i.sroa.10.0.lcssa, %bb15.i.loopexit ]
  %history.i41.i.sroa.7.0.lcssa22725 = phi <4 x i32> [ %history.i41.i.sroa.7.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i41.i.sroa.7.0.lcssa, %bb15.i.loopexit ]
  %history.i41.i.sroa.0.0.lcssa22705 = phi <4 x i32> [ %hot_left.i.promoted, %bb44.i.lr.ph ], [ %history.i41.i.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %indvars.iv20551 = phi i32 [ %frames, %bb44.i.lr.ph ], [ %indvars.iv.next20552, %bb15.i.loopexit ]
  %iter2.sroa.0.0.i18469 = phi i32 [ %yield_count.sroa.0.0.i6676, %bb44.i.lr.ph ], [ %1719, %bb15.i.loopexit ]
  %iter1.sroa.0.0.i18468 = phi i32 [ 0, %bb44.i.lr.ph ], [ %1718, %bb15.i.loopexit ]
  %main_cursor.sroa.0.0.i18467 = phi i32 [ %_36.i, %bb44.i.lr.ph ], [ %main_cursor.sroa.0.1.i.lcssa, %bb15.i.loopexit ]
  %ring_cursor.sroa.0.0.i18466 = phi i32 [ %_37.i, %bb44.i.lr.ph ], [ %ring_cursor.sroa.0.1.i.lcssa, %bb15.i.loopexit ]
  %umin20575 = call i32 @llvm.umin.i32(i32 %indvars.iv20551, i32 32), !dbg !11565
  %umax20557 = call i32 @llvm.umax.i32(i32 %umin20575, i32 1), !dbg !11565
  %1718 = add i32 %iter1.sroa.0.0.i18468, 32, !dbg !11565
  %1719 = add nsw i32 %iter2.sroa.0.0.i18469, -1, !dbg !11569
  %1720 = sub i32 %frames, %iter1.sroa.0.0.i18468, !dbg !11570
  %spec.store.select.i = tail call i32 @llvm.umin.i32(i32 %1720, i32 32), !dbg !11572
  %_20.i44.i18140.not = icmp eq i32 %frames, %iter1.sroa.0.0.i18468, !dbg !11577
  br i1 %_20.i44.i18140.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit249.i, label %bb5.i45.i.lr.ph, !dbg !11583

bb5.i45.i.lr.ph:                                  ; preds = %bb44.i
  %_11.i.i.i66.i15591 = load <4 x float>, ptr %_31, align 16
  %_14.i.i.i69.i15592 = load <4 x float>, ptr %1660, align 16
  %_17.i.i.i72.i15593 = load <4 x float>, ptr %1661, align 16
  %_20.i.i.i75.i15594 = load <4 x float>, ptr %1662, align 16
  %_25.i.i.i80.i15595 = load <4 x float>, ptr %row1.i.i.i78.i, align 16
  %_28.i.i.i83.i15596 = load <4 x float>, ptr %1663, align 16
  %_31.i.i.i86.i15597 = load <4 x float>, ptr %1664, align 16
  %_34.i.i.i89.i15598 = load <4 x float>, ptr %1665, align 16
  %_39.i.i.i94.i15599 = load <4 x float>, ptr %row3.i.i.i92.i, align 16
  %_42.i.i.i97.i15600 = load <4 x float>, ptr %1666, align 16
  %_45.i.i.i100.i15601 = load <4 x float>, ptr %1667, align 16
  %_48.i.i.i103.i15602 = load <4 x float>, ptr %1668, align 16
  %_53.i.i.i108.i15603 = load <4 x float>, ptr %row5.i.i.i106.i, align 16
  %_56.i.i.i111.i15604 = load <4 x float>, ptr %1669, align 16
  %_59.i.i.i114.i15605 = load <4 x float>, ptr %1670, align 16
  %_62.i.i.i117.i15606 = load <4 x float>, ptr %1671, align 16
  %_67.i.i.i122.i15607 = load <4 x float>, ptr %row7.i.i.i120.i, align 16
  %_70.i.i.i125.i15608 = load <4 x float>, ptr %1672, align 16
  %_73.i.i.i128.i15609 = load <4 x float>, ptr %1673, align 16
  %_76.i.i.i131.i15610 = load <4 x float>, ptr %1674, align 16
  %_81.i.i.i136.i15611 = load <4 x float>, ptr %row9.i.i.i134.i, align 16
  %_84.i.i.i139.i15612 = load <4 x float>, ptr %1675, align 16
  %_87.i.i.i142.i15613 = load <4 x float>, ptr %1676, align 16
  %_90.i.i.i145.i15614 = load <4 x float>, ptr %1677, align 16
  %_95.i.i.i150.i15615 = load <4 x float>, ptr %row11.i.i.i148.i, align 16
  %_98.i.i.i153.i15616 = load <4 x float>, ptr %1678, align 16
  %_101.i.i.i156.i15617 = load <4 x float>, ptr %1679, align 16
  %_104.i.i.i159.i15618 = load <4 x float>, ptr %1680, align 16
  %_109.i.i.i164.i15619 = load <4 x float>, ptr %row13.i.i.i162.i, align 16
  %_112.i.i.i167.i15620 = load <4 x float>, ptr %1681, align 16
  %_115.i.i.i170.i15621 = load <4 x float>, ptr %1682, align 16
  %_118.i.i.i173.i15622 = load <4 x float>, ptr %1683, align 16
  %_123.i.i.i178.i15623 = load <4 x float>, ptr %row15.i.i.i176.i, align 16
  %_126.i.i.i181.i15624 = load <4 x float>, ptr %1684, align 16
  %_129.i.i.i184.i15625 = load <4 x float>, ptr %1685, align 16
  %_132.i.i.i187.i15626 = load <4 x float>, ptr %1686, align 16
  %_137.i.i.i192.i15627 = load <4 x float>, ptr %row17.i.i.i190.i, align 16
  %_140.i.i.i195.i15628 = load <4 x float>, ptr %1687, align 16
  %_143.i.i.i198.i15629 = load <4 x float>, ptr %1688, align 16
  %_146.i.i.i201.i15630 = load <4 x float>, ptr %1689, align 16
  %_151.i.i.i206.i15631 = load <4 x float>, ptr %row19.i.i.i204.i, align 16
  %_154.i.i.i209.i15632 = load <4 x float>, ptr %1690, align 16
  %_157.i.i.i212.i15633 = load <4 x float>, ptr %1691, align 16
  %_160.i.i.i215.i15634 = load <4 x float>, ptr %1692, align 16
  %_165.i.i.i220.i15635 = load <4 x float>, ptr %row21.i.i.i218.i, align 16
  %_168.i.i.i223.i15636 = load <4 x float>, ptr %1693, align 16
  %_171.i.i.i226.i15637 = load <4 x float>, ptr %1694, align 16
  %_174.i.i.i229.i15638 = load <4 x float>, ptr %1695, align 16
  br label %bb5.i45.i, !dbg !11583

bb5.i45.i:                                        ; preds = %bb5.i45.i.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034
  %iter.sroa.0.0.i43.i18152 = phi i32 [ 0, %bb5.i45.i.lr.ph ], [ %1721, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034 ]
  %history.i41.i.sroa.0.018151 = phi <4 x i32> [ %history.i41.i.sroa.0.0.lcssa22705, %bb5.i45.i.lr.ph ], [ %lanes.i5365.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034 ]
  %history.i41.i.sroa.7.018150 = phi <4 x i32> [ %history.i41.i.sroa.7.0.lcssa22725, %bb5.i45.i.lr.ph ], [ %history.i41.i.sroa.0.018151, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034 ]
  %history.i41.i.sroa.10.018149 = phi <4 x i32> [ %history.i41.i.sroa.10.0.lcssa22745, %bb5.i45.i.lr.ph ], [ %history.i41.i.sroa.7.018150, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034 ]
  %history.i41.i.sroa.13.018148 = phi <4 x i32> [ %history.i41.i.sroa.13.0.lcssa22765, %bb5.i45.i.lr.ph ], [ %history.i41.i.sroa.10.018149, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034 ]
  %history.i41.i.sroa.16.018147 = phi <4 x i32> [ %history.i41.i.sroa.16.0.lcssa22785, %bb5.i45.i.lr.ph ], [ %history.i41.i.sroa.13.018148, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034 ]
  %history.i41.i.sroa.19.018146 = phi <4 x i32> [ %history.i41.i.sroa.19.0.lcssa22805, %bb5.i45.i.lr.ph ], [ %history.i41.i.sroa.16.018147, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034 ]
  %history.i41.i.sroa.22.018145 = phi <4 x i32> [ %history.i41.i.sroa.22.0.lcssa22825, %bb5.i45.i.lr.ph ], [ %history.i41.i.sroa.19.018146, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034 ]
  %history.i41.i.sroa.26.018144 = phi <4 x i32> [ %history.i41.i.sroa.26.0.lcssa22845, %bb5.i45.i.lr.ph ], [ %history.i41.i.sroa.22.018145, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034 ]
  %history.i41.i.sroa.29.018143 = phi <4 x i32> [ %history.i41.i.sroa.29.0.lcssa22865, %bb5.i45.i.lr.ph ], [ %history.i41.i.sroa.26.018144, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034 ]
  %history.i41.i.sroa.32.018142 = phi <4 x i32> [ %history.i41.i.sroa.32.0.lcssa22885, %bb5.i45.i.lr.ph ], [ %history.i41.i.sroa.29.018143, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034 ]
  %history.i41.i.sroa.35.018141 = phi <4 x i32> [ %history.i41.i.sroa.35.0.lcssa22905, %bb5.i45.i.lr.ph ], [ %history.i41.i.sroa.32.018142, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034 ]
  %1721 = add nuw nsw i32 %iter.sroa.0.0.i43.i18152, 1, !dbg !11584
  %_11.i46.i = add nuw nsw i32 %iter.sroa.0.0.i43.i18152, %iter1.sroa.0.0.i18468, !dbg !11587
  %base.i47.i = shl i32 %_11.i46.i, 2, !dbg !11587
  %_24.i48.i = icmp ugt i32 %base.i47.i, %left_io.1, !dbg !11588
  br i1 %_24.i48.i, label %bb7.i248.i, label %bb8.i49.i, !dbg !11588, !prof !902

bb8.i49.i:                                        ; preds = %bb5.i45.i
  %_27.i50.i = sub nuw nsw i32 %left_io.1, %base.i47.i, !dbg !11591
  %_8.i5368 = icmp samesign ugt i32 %_27.i50.i, 3, !dbg !11592
  br i1 %_8.i5368, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034, label %bb2.i5369, !dbg !11592, !prof !1153

bb2.i5369:                                        ; preds = %bb8.i49.i
  store <4 x i32> %history.i41.i.sroa.0.0.lcssa22705, ptr %hot_left.i, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.7.0.lcssa22725, ptr %history.i41.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.10.0.lcssa22745, ptr %history.i41.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.13.0.lcssa22765, ptr %history.i41.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.16.0.lcssa22785, ptr %history.i41.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.19.0.lcssa22805, ptr %history.i41.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.22.0.lcssa22825, ptr %history.i41.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.26.0.lcssa22845, ptr %history.i41.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.29.0.lcssa22865, ptr %history.i41.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.32.0.lcssa22885, ptr %history.i41.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.35.0.lcssa22905, ptr %history.i41.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.38.0.lcssa22925, ptr %history.i41.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i.i.sroa.0.0.lcssa22945, ptr %hot_right.i, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.7.0.lcssa22965, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.10.0.lcssa22985, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.13.0.lcssa23005, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.16.0.lcssa23025, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.19.0.lcssa23045, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.22.0.lcssa23065, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.26.0.lcssa23085, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.29.0.lcssa23105, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.32.0.lcssa23125, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.35.0.lcssa23145, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.38.0.lcssa23165, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store i32 %storemerge.i1325.lcssa1822318322.lcssa23186, ptr %_22.i278.i, align 4
  store <4 x float> %.lcssa1825318358.lcssa23208, ptr %1703, align 16
  store i32 %storemerge.i1291.lcssa1828018394.lcssa23229, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1831018430.lcssa23251, ptr %1713, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_27.i50.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !11600, !noalias !11601
  unreachable, !dbg !11600

bb7.i248.i:                                       ; preds = %bb5.i45.i
  store <4 x i32> %history.i41.i.sroa.0.0.lcssa22705, ptr %hot_left.i, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.7.0.lcssa22725, ptr %history.i41.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.10.0.lcssa22745, ptr %history.i41.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.13.0.lcssa22765, ptr %history.i41.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.16.0.lcssa22785, ptr %history.i41.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.19.0.lcssa22805, ptr %history.i41.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.22.0.lcssa22825, ptr %history.i41.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.26.0.lcssa22845, ptr %history.i41.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.29.0.lcssa22865, ptr %history.i41.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.32.0.lcssa22885, ptr %history.i41.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.35.0.lcssa22905, ptr %history.i41.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.38.0.lcssa22925, ptr %history.i41.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i.i.sroa.0.0.lcssa22945, ptr %hot_right.i, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.7.0.lcssa22965, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.10.0.lcssa22985, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.13.0.lcssa23005, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.16.0.lcssa23025, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.19.0.lcssa23045, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.22.0.lcssa23065, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.26.0.lcssa23085, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.29.0.lcssa23105, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.32.0.lcssa23125, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.35.0.lcssa23145, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.38.0.lcssa23165, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store i32 %storemerge.i1325.lcssa1822318322.lcssa23186, ptr %_22.i278.i, align 4
  store <4 x float> %.lcssa1825318358.lcssa23208, ptr %1703, align 16
  store i32 %storemerge.i1291.lcssa1828018394.lcssa23229, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1831018430.lcssa23251, ptr %1713, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i47.i, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #33, !dbg !11608, !noalias !11609
  unreachable, !dbg !11608

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034: ; preds = %bb8.i49.i
  %_31.i51.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %base.i47.i, !dbg !11610
  %lanes.i5365.sroa.0.0.copyload = load <4 x i32>, ptr %_31.i51.i, align 4, !dbg !11612, !alias.scope !11616, !noalias !11620
  %1722 = bitcast <4 x i32> %history.i41.i.sroa.19.018146 to <4 x float>, !dbg !11622
  %1723 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1722), !dbg !11627
  %1724 = bitcast <4 x i32> %lanes.i5365.sroa.0.0.copyload to <4 x float>, !dbg !11628
  %1725 = fmul <4 x float> %_11.i.i.i66.i15591, %1724, !dbg !11633
  %1726 = fadd <4 x float> %1725, zeroinitializer, !dbg !11634
  %1727 = fmul <4 x float> %_14.i.i.i69.i15592, %1724, !dbg !11638
  %1728 = fadd <4 x float> %1727, zeroinitializer, !dbg !11642
  %1729 = fmul <4 x float> %_17.i.i.i72.i15593, %1724, !dbg !11646
  %1730 = fadd <4 x float> %1729, zeroinitializer, !dbg !11650
  %1731 = fmul <4 x float> %_20.i.i.i75.i15594, %1724, !dbg !11654
  %1732 = fadd <4 x float> %1731, zeroinitializer, !dbg !11658
  %1733 = bitcast <4 x i32> %history.i41.i.sroa.0.018151 to <4 x float>, !dbg !11662
  %1734 = fmul <4 x float> %_25.i.i.i80.i15595, %1733, !dbg !11666
  %1735 = fadd <4 x float> %1726, %1734, !dbg !11667
  %1736 = fmul <4 x float> %_28.i.i.i83.i15596, %1733, !dbg !11671
  %1737 = fadd <4 x float> %1728, %1736, !dbg !11675
  %1738 = fmul <4 x float> %_31.i.i.i86.i15597, %1733, !dbg !11679
  %1739 = fadd <4 x float> %1730, %1738, !dbg !11683
  %1740 = fmul <4 x float> %_34.i.i.i89.i15598, %1733, !dbg !11687
  %1741 = fadd <4 x float> %1732, %1740, !dbg !11691
  %1742 = bitcast <4 x i32> %history.i41.i.sroa.7.018150 to <4 x float>, !dbg !11695
  %1743 = fmul <4 x float> %_39.i.i.i94.i15599, %1742, !dbg !11699
  %1744 = fadd <4 x float> %1735, %1743, !dbg !11700
  %1745 = fmul <4 x float> %_42.i.i.i97.i15600, %1742, !dbg !11704
  %1746 = fadd <4 x float> %1737, %1745, !dbg !11708
  %1747 = fmul <4 x float> %_45.i.i.i100.i15601, %1742, !dbg !11712
  %1748 = fadd <4 x float> %1739, %1747, !dbg !11716
  %1749 = fmul <4 x float> %_48.i.i.i103.i15602, %1742, !dbg !11720
  %1750 = fadd <4 x float> %1741, %1749, !dbg !11724
  %1751 = bitcast <4 x i32> %history.i41.i.sroa.10.018149 to <4 x float>, !dbg !11728
  %1752 = fmul <4 x float> %_53.i.i.i108.i15603, %1751, !dbg !11732
  %1753 = fadd <4 x float> %1744, %1752, !dbg !11733
  %1754 = fmul <4 x float> %_56.i.i.i111.i15604, %1751, !dbg !11737
  %1755 = fadd <4 x float> %1746, %1754, !dbg !11741
  %1756 = fmul <4 x float> %_59.i.i.i114.i15605, %1751, !dbg !11745
  %1757 = fadd <4 x float> %1748, %1756, !dbg !11749
  %1758 = fmul <4 x float> %_62.i.i.i117.i15606, %1751, !dbg !11753
  %1759 = fadd <4 x float> %1750, %1758, !dbg !11757
  %1760 = bitcast <4 x i32> %history.i41.i.sroa.13.018148 to <4 x float>, !dbg !11761
  %1761 = fmul <4 x float> %_67.i.i.i122.i15607, %1760, !dbg !11765
  %1762 = fadd <4 x float> %1753, %1761, !dbg !11766
  %1763 = fmul <4 x float> %_70.i.i.i125.i15608, %1760, !dbg !11770
  %1764 = fadd <4 x float> %1755, %1763, !dbg !11774
  %1765 = fmul <4 x float> %_73.i.i.i128.i15609, %1760, !dbg !11778
  %1766 = fadd <4 x float> %1757, %1765, !dbg !11782
  %1767 = fmul <4 x float> %_76.i.i.i131.i15610, %1760, !dbg !11786
  %1768 = fadd <4 x float> %1759, %1767, !dbg !11790
  %1769 = bitcast <4 x i32> %history.i41.i.sroa.16.018147 to <4 x float>, !dbg !11794
  %1770 = fmul <4 x float> %_81.i.i.i136.i15611, %1769, !dbg !11798
  %1771 = fadd <4 x float> %1762, %1770, !dbg !11799
  %1772 = fmul <4 x float> %_84.i.i.i139.i15612, %1769, !dbg !11803
  %1773 = fadd <4 x float> %1764, %1772, !dbg !11807
  %1774 = fmul <4 x float> %_87.i.i.i142.i15613, %1769, !dbg !11811
  %1775 = fadd <4 x float> %1766, %1774, !dbg !11815
  %1776 = fmul <4 x float> %_90.i.i.i145.i15614, %1769, !dbg !11819
  %1777 = fadd <4 x float> %1768, %1776, !dbg !11823
  %1778 = fmul <4 x float> %_95.i.i.i150.i15615, %1722, !dbg !11827
  %1779 = fadd <4 x float> %1771, %1778, !dbg !11831
  %1780 = fmul <4 x float> %_98.i.i.i153.i15616, %1722, !dbg !11835
  %1781 = fadd <4 x float> %1773, %1780, !dbg !11839
  %1782 = fmul <4 x float> %_101.i.i.i156.i15617, %1722, !dbg !11843
  %1783 = fadd <4 x float> %1775, %1782, !dbg !11847
  %1784 = fmul <4 x float> %_104.i.i.i159.i15618, %1722, !dbg !11851
  %1785 = fadd <4 x float> %1777, %1784, !dbg !11855
  %1786 = bitcast <4 x i32> %history.i41.i.sroa.22.018145 to <4 x float>, !dbg !11859
  %1787 = fmul <4 x float> %_109.i.i.i164.i15619, %1786, !dbg !11863
  %1788 = fadd <4 x float> %1779, %1787, !dbg !11864
  %1789 = fmul <4 x float> %_112.i.i.i167.i15620, %1786, !dbg !11868
  %1790 = fadd <4 x float> %1781, %1789, !dbg !11872
  %1791 = fmul <4 x float> %_115.i.i.i170.i15621, %1786, !dbg !11876
  %1792 = fadd <4 x float> %1783, %1791, !dbg !11880
  %1793 = fmul <4 x float> %_118.i.i.i173.i15622, %1786, !dbg !11884
  %1794 = fadd <4 x float> %1785, %1793, !dbg !11888
  %1795 = bitcast <4 x i32> %history.i41.i.sroa.26.018144 to <4 x float>, !dbg !11892
  %1796 = fmul <4 x float> %_123.i.i.i178.i15623, %1795, !dbg !11896
  %1797 = fadd <4 x float> %1788, %1796, !dbg !11897
  %1798 = fmul <4 x float> %_126.i.i.i181.i15624, %1795, !dbg !11901
  %1799 = fadd <4 x float> %1790, %1798, !dbg !11905
  %1800 = fmul <4 x float> %_129.i.i.i184.i15625, %1795, !dbg !11909
  %1801 = fadd <4 x float> %1792, %1800, !dbg !11913
  %1802 = fmul <4 x float> %_132.i.i.i187.i15626, %1795, !dbg !11917
  %1803 = fadd <4 x float> %1794, %1802, !dbg !11921
  %1804 = bitcast <4 x i32> %history.i41.i.sroa.29.018143 to <4 x float>, !dbg !11925
  %1805 = fmul <4 x float> %_137.i.i.i192.i15627, %1804, !dbg !11929
  %1806 = fadd <4 x float> %1797, %1805, !dbg !11930
  %1807 = fmul <4 x float> %_140.i.i.i195.i15628, %1804, !dbg !11934
  %1808 = fadd <4 x float> %1799, %1807, !dbg !11938
  %1809 = fmul <4 x float> %_143.i.i.i198.i15629, %1804, !dbg !11942
  %1810 = fadd <4 x float> %1801, %1809, !dbg !11946
  %1811 = fmul <4 x float> %_146.i.i.i201.i15630, %1804, !dbg !11950
  %1812 = fadd <4 x float> %1803, %1811, !dbg !11954
  %1813 = bitcast <4 x i32> %history.i41.i.sroa.32.018142 to <4 x float>, !dbg !11958
  %1814 = fmul <4 x float> %_151.i.i.i206.i15631, %1813, !dbg !11962
  %1815 = fadd <4 x float> %1806, %1814, !dbg !11963
  %1816 = fmul <4 x float> %_154.i.i.i209.i15632, %1813, !dbg !11967
  %1817 = fadd <4 x float> %1808, %1816, !dbg !11971
  %1818 = fmul <4 x float> %_157.i.i.i212.i15633, %1813, !dbg !11975
  %1819 = fadd <4 x float> %1810, %1818, !dbg !11979
  %1820 = fmul <4 x float> %_160.i.i.i215.i15634, %1813, !dbg !11983
  %1821 = fadd <4 x float> %1812, %1820, !dbg !11987
  %1822 = bitcast <4 x i32> %history.i41.i.sroa.35.018141 to <4 x float>, !dbg !11991
  %1823 = fmul <4 x float> %_165.i.i.i220.i15635, %1822, !dbg !11995
  %1824 = fadd <4 x float> %1815, %1823, !dbg !11996
  %1825 = fmul <4 x float> %_168.i.i.i223.i15636, %1822, !dbg !12000
  %1826 = fadd <4 x float> %1817, %1825, !dbg !12004
  %1827 = fmul <4 x float> %_171.i.i.i226.i15637, %1822, !dbg !12008
  %1828 = fadd <4 x float> %1819, %1827, !dbg !12012
  %1829 = fmul <4 x float> %_174.i.i.i229.i15638, %1822, !dbg !12016
  %1830 = fadd <4 x float> %1821, %1829, !dbg !12020
  %1831 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1824), !dbg !12024
  %1832 = fcmp olt <4 x float> %1831, %1723, !dbg !12028
  %1833 = select <4 x i1> %1832, <4 x float> %1723, <4 x float> %1831, !dbg !12032
  %1834 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1826), !dbg !12024
  %1835 = fcmp olt <4 x float> %1834, %1833, !dbg !12028
  %1836 = select <4 x i1> %1835, <4 x float> %1833, <4 x float> %1834, !dbg !12032
  %1837 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1828), !dbg !12024
  %1838 = fcmp olt <4 x float> %1837, %1836, !dbg !12028
  %1839 = select <4 x i1> %1838, <4 x float> %1836, <4 x float> %1837, !dbg !12032
  %1840 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1830), !dbg !12024
  %1841 = fcmp olt <4 x float> %1840, %1839, !dbg !12028
  %1842 = select <4 x i1> %1841, <4 x float> %1839, <4 x float> %1840, !dbg !12032
  %_39.i242.i.idx = shl i32 %iter.sroa.0.0.i43.i18152, 4, !dbg !12033
  %_39.i242.i = getelementptr inbounds nuw i8, ptr %peaks_left.i, i32 %_39.i242.i.idx, !dbg !12033
  store <4 x float> %1842, ptr %_39.i242.i, align 4, !dbg !12038, !alias.scope !12043, !noalias !12047
  %exitcond20555.not = icmp eq i32 %1721, %umax20557, !dbg !11577
  br i1 %exitcond20555.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit249.i, label %bb5.i45.i, !dbg !11583

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit249.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034, %bb44.i
  %history.i41.i.sroa.38.0.lcssa = phi <4 x i32> [ %history.i41.i.sroa.38.0.lcssa22925, %bb44.i ], [ %history.i41.i.sroa.35.018141, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034 ], !dbg !11597
  %history.i41.i.sroa.35.0.lcssa = phi <4 x i32> [ %history.i41.i.sroa.35.0.lcssa22905, %bb44.i ], [ %history.i41.i.sroa.32.018142, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034 ], !dbg !11597
  %history.i41.i.sroa.32.0.lcssa = phi <4 x i32> [ %history.i41.i.sroa.32.0.lcssa22885, %bb44.i ], [ %history.i41.i.sroa.29.018143, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034 ], !dbg !11597
  %history.i41.i.sroa.29.0.lcssa = phi <4 x i32> [ %history.i41.i.sroa.29.0.lcssa22865, %bb44.i ], [ %history.i41.i.sroa.26.018144, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034 ], !dbg !11597
  %history.i41.i.sroa.26.0.lcssa = phi <4 x i32> [ %history.i41.i.sroa.26.0.lcssa22845, %bb44.i ], [ %history.i41.i.sroa.22.018145, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034 ], !dbg !11597
  %history.i41.i.sroa.22.0.lcssa = phi <4 x i32> [ %history.i41.i.sroa.22.0.lcssa22825, %bb44.i ], [ %history.i41.i.sroa.19.018146, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034 ], !dbg !11597
  %history.i41.i.sroa.19.0.lcssa = phi <4 x i32> [ %history.i41.i.sroa.19.0.lcssa22805, %bb44.i ], [ %history.i41.i.sroa.16.018147, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034 ], !dbg !11597
  %history.i41.i.sroa.16.0.lcssa = phi <4 x i32> [ %history.i41.i.sroa.16.0.lcssa22785, %bb44.i ], [ %history.i41.i.sroa.13.018148, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034 ], !dbg !11597
  %history.i41.i.sroa.13.0.lcssa = phi <4 x i32> [ %history.i41.i.sroa.13.0.lcssa22765, %bb44.i ], [ %history.i41.i.sroa.10.018149, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034 ], !dbg !11597
  %history.i41.i.sroa.10.0.lcssa = phi <4 x i32> [ %history.i41.i.sroa.10.0.lcssa22745, %bb44.i ], [ %history.i41.i.sroa.7.018150, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034 ], !dbg !11597
  %history.i41.i.sroa.7.0.lcssa = phi <4 x i32> [ %history.i41.i.sroa.7.0.lcssa22725, %bb44.i ], [ %history.i41.i.sroa.0.018151, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034 ], !dbg !11597
  %history.i41.i.sroa.0.0.lcssa = phi <4 x i32> [ %history.i41.i.sroa.0.0.lcssa22705, %bb44.i ], [ %lanes.i5365.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6034 ], !dbg !11597
  br i1 %_20.i44.i18140.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i, label %bb5.i.i.lr.ph, !dbg !12051

bb5.i.i.lr.ph:                                    ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit249.i
  %_11.i.i.i.i15538 = load <4 x float>, ptr %_31, align 16
  %_14.i.i.i.i15539 = load <4 x float>, ptr %1660, align 16
  %_17.i.i.i.i15540 = load <4 x float>, ptr %1661, align 16
  %_20.i.i.i.i15541 = load <4 x float>, ptr %1662, align 16
  %_25.i.i.i.i15542 = load <4 x float>, ptr %row1.i.i.i78.i, align 16
  %_28.i.i.i.i15543 = load <4 x float>, ptr %1663, align 16
  %_31.i.i.i.i15544 = load <4 x float>, ptr %1664, align 16
  %_34.i.i.i.i15545 = load <4 x float>, ptr %1665, align 16
  %_39.i.i.i.i15546 = load <4 x float>, ptr %row3.i.i.i92.i, align 16
  %_42.i.i.i.i15547 = load <4 x float>, ptr %1666, align 16
  %_45.i.i.i.i15548 = load <4 x float>, ptr %1667, align 16
  %_48.i.i.i.i15549 = load <4 x float>, ptr %1668, align 16
  %_53.i.i.i.i15550 = load <4 x float>, ptr %row5.i.i.i106.i, align 16
  %_56.i.i.i.i15551 = load <4 x float>, ptr %1669, align 16
  %_59.i.i.i.i15552 = load <4 x float>, ptr %1670, align 16
  %_62.i.i.i.i15553 = load <4 x float>, ptr %1671, align 16
  %_67.i.i.i.i15554 = load <4 x float>, ptr %row7.i.i.i120.i, align 16
  %_70.i.i.i.i15555 = load <4 x float>, ptr %1672, align 16
  %_73.i.i.i.i15556 = load <4 x float>, ptr %1673, align 16
  %_76.i.i.i.i15557 = load <4 x float>, ptr %1674, align 16
  %_81.i.i.i.i15558 = load <4 x float>, ptr %row9.i.i.i134.i, align 16
  %_84.i.i.i.i15559 = load <4 x float>, ptr %1675, align 16
  %_87.i.i.i.i15560 = load <4 x float>, ptr %1676, align 16
  %_90.i.i.i.i15561 = load <4 x float>, ptr %1677, align 16
  %_95.i.i.i.i15562 = load <4 x float>, ptr %row11.i.i.i148.i, align 16
  %_98.i.i.i.i15563 = load <4 x float>, ptr %1678, align 16
  %_101.i.i.i.i15564 = load <4 x float>, ptr %1679, align 16
  %_104.i.i.i.i15565 = load <4 x float>, ptr %1680, align 16
  %_109.i.i.i.i15566 = load <4 x float>, ptr %row13.i.i.i162.i, align 16
  %_112.i.i.i.i15567 = load <4 x float>, ptr %1681, align 16
  %_115.i.i.i.i15568 = load <4 x float>, ptr %1682, align 16
  %_118.i.i.i.i15569 = load <4 x float>, ptr %1683, align 16
  %_123.i.i.i.i15570 = load <4 x float>, ptr %row15.i.i.i176.i, align 16
  %_126.i.i.i.i15571 = load <4 x float>, ptr %1684, align 16
  %_129.i.i.i.i15572 = load <4 x float>, ptr %1685, align 16
  %_132.i.i.i.i15573 = load <4 x float>, ptr %1686, align 16
  %_137.i.i.i.i15574 = load <4 x float>, ptr %row17.i.i.i190.i, align 16
  %_140.i.i.i.i15575 = load <4 x float>, ptr %1687, align 16
  %_143.i.i.i.i15576 = load <4 x float>, ptr %1688, align 16
  %_146.i.i.i.i15577 = load <4 x float>, ptr %1689, align 16
  %_151.i.i.i.i15578 = load <4 x float>, ptr %row19.i.i.i204.i, align 16
  %_154.i.i.i.i15579 = load <4 x float>, ptr %1690, align 16
  %_157.i.i.i.i15580 = load <4 x float>, ptr %1691, align 16
  %_160.i.i.i.i15581 = load <4 x float>, ptr %1692, align 16
  %_165.i.i.i.i15582 = load <4 x float>, ptr %row21.i.i.i218.i, align 16
  %_168.i.i.i.i15583 = load <4 x float>, ptr %1693, align 16
  %_171.i.i.i.i15584 = load <4 x float>, ptr %1694, align 16
  %_174.i.i.i.i15585 = load <4 x float>, ptr %1695, align 16
  br label %bb5.i.i, !dbg !12051

bb5.i.i:                                          ; preds = %bb5.i.i.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039
  %iter.sroa.0.0.i.i18179 = phi i32 [ 0, %bb5.i.i.lr.ph ], [ %1843, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039 ]
  %history.i.i.sroa.0.018178 = phi <4 x i32> [ %history.i.i.sroa.0.0.lcssa22945, %bb5.i.i.lr.ph ], [ %lanes.i5374.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039 ]
  %history.i.i.sroa.7.018177 = phi <4 x i32> [ %history.i.i.sroa.7.0.lcssa22965, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.0.018178, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039 ]
  %history.i.i.sroa.10.018176 = phi <4 x i32> [ %history.i.i.sroa.10.0.lcssa22985, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.7.018177, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039 ]
  %history.i.i.sroa.13.018175 = phi <4 x i32> [ %history.i.i.sroa.13.0.lcssa23005, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.10.018176, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039 ]
  %history.i.i.sroa.16.018174 = phi <4 x i32> [ %history.i.i.sroa.16.0.lcssa23025, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.13.018175, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039 ]
  %history.i.i.sroa.19.018173 = phi <4 x i32> [ %history.i.i.sroa.19.0.lcssa23045, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.16.018174, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039 ]
  %history.i.i.sroa.22.018172 = phi <4 x i32> [ %history.i.i.sroa.22.0.lcssa23065, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.19.018173, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039 ]
  %history.i.i.sroa.26.018171 = phi <4 x i32> [ %history.i.i.sroa.26.0.lcssa23085, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.22.018172, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039 ]
  %history.i.i.sroa.29.018170 = phi <4 x i32> [ %history.i.i.sroa.29.0.lcssa23105, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.26.018171, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039 ]
  %history.i.i.sroa.32.018169 = phi <4 x i32> [ %history.i.i.sroa.32.0.lcssa23125, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.29.018170, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039 ]
  %history.i.i.sroa.35.018168 = phi <4 x i32> [ %history.i.i.sroa.35.0.lcssa23145, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.32.018169, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039 ]
  %1843 = add nuw nsw i32 %iter.sroa.0.0.i.i18179, 1, !dbg !12054
  %_11.i.i = add nuw nsw i32 %iter.sroa.0.0.i.i18179, %iter1.sroa.0.0.i18468, !dbg !12057
  %base.i.i = shl i32 %_11.i.i, 2, !dbg !12057
  %_24.i.i = icmp ugt i32 %base.i.i, %right_io.1, !dbg !12058
  br i1 %_24.i.i, label %bb7.i.i, label %bb8.i.i, !dbg !12058, !prof !902

bb8.i.i:                                          ; preds = %bb5.i.i
  %_27.i.i = sub nuw nsw i32 %right_io.1, %base.i.i, !dbg !12061
  %_8.i5377 = icmp samesign ugt i32 %_27.i.i, 3, !dbg !12062
  br i1 %_8.i5377, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039, label %bb2.i5378, !dbg !12062, !prof !1153

bb2.i5378:                                        ; preds = %bb8.i.i
  store <4 x i32> %history.i41.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.7.0.lcssa, ptr %history.i41.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.10.0.lcssa, ptr %history.i41.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.13.0.lcssa, ptr %history.i41.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.16.0.lcssa, ptr %history.i41.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.19.0.lcssa, ptr %history.i41.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.22.0.lcssa, ptr %history.i41.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.26.0.lcssa, ptr %history.i41.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.29.0.lcssa, ptr %history.i41.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.32.0.lcssa, ptr %history.i41.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.35.0.lcssa, ptr %history.i41.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.38.0.lcssa, ptr %history.i41.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i.i.sroa.0.0.lcssa22945, ptr %hot_right.i, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.7.0.lcssa22965, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.10.0.lcssa22985, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.13.0.lcssa23005, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.16.0.lcssa23025, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.19.0.lcssa23045, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.22.0.lcssa23065, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.26.0.lcssa23085, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.29.0.lcssa23105, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.32.0.lcssa23125, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.35.0.lcssa23145, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.38.0.lcssa23165, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store i32 %storemerge.i1325.lcssa1822318322.lcssa23186, ptr %_22.i278.i, align 4
  store <4 x float> %.lcssa1825318358.lcssa23208, ptr %1703, align 16
  store i32 %storemerge.i1291.lcssa1828018394.lcssa23229, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1831018430.lcssa23251, ptr %1713, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_27.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !12067, !noalias !12068
  unreachable, !dbg !12067

bb7.i.i:                                          ; preds = %bb5.i.i
  store <4 x i32> %history.i41.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.7.0.lcssa, ptr %history.i41.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.10.0.lcssa, ptr %history.i41.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.13.0.lcssa, ptr %history.i41.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.16.0.lcssa, ptr %history.i41.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.19.0.lcssa, ptr %history.i41.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.22.0.lcssa, ptr %history.i41.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.26.0.lcssa, ptr %history.i41.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.29.0.lcssa, ptr %history.i41.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.32.0.lcssa, ptr %history.i41.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.35.0.lcssa, ptr %history.i41.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.38.0.lcssa, ptr %history.i41.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i.i.sroa.0.0.lcssa22945, ptr %hot_right.i, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.7.0.lcssa22965, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.10.0.lcssa22985, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.13.0.lcssa23005, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.16.0.lcssa23025, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.19.0.lcssa23045, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.22.0.lcssa23065, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.26.0.lcssa23085, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.29.0.lcssa23105, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.32.0.lcssa23125, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.35.0.lcssa23145, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.38.0.lcssa23165, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store i32 %storemerge.i1325.lcssa1822318322.lcssa23186, ptr %_22.i278.i, align 4
  store <4 x float> %.lcssa1825318358.lcssa23208, ptr %1703, align 16
  store i32 %storemerge.i1291.lcssa1828018394.lcssa23229, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1831018430.lcssa23251, ptr %1713, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i.i, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #33, !dbg !12075, !noalias !12076
  unreachable, !dbg !12075

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039: ; preds = %bb8.i.i
  %_31.i.i = getelementptr inbounds nuw float, ptr %right_io.0, i32 %base.i.i, !dbg !12077
  %lanes.i5374.sroa.0.0.copyload = load <4 x i32>, ptr %_31.i.i, align 4, !dbg !12079, !alias.scope !12083, !noalias !12087
  %1844 = bitcast <4 x i32> %history.i.i.sroa.19.018173 to <4 x float>, !dbg !12089
  %1845 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1844), !dbg !12094
  %1846 = bitcast <4 x i32> %lanes.i5374.sroa.0.0.copyload to <4 x float>, !dbg !12095
  %1847 = fmul <4 x float> %_11.i.i.i.i15538, %1846, !dbg !12100
  %1848 = fadd <4 x float> %1847, zeroinitializer, !dbg !12101
  %1849 = fmul <4 x float> %_14.i.i.i.i15539, %1846, !dbg !12105
  %1850 = fadd <4 x float> %1849, zeroinitializer, !dbg !12109
  %1851 = fmul <4 x float> %_17.i.i.i.i15540, %1846, !dbg !12113
  %1852 = fadd <4 x float> %1851, zeroinitializer, !dbg !12117
  %1853 = fmul <4 x float> %_20.i.i.i.i15541, %1846, !dbg !12121
  %1854 = fadd <4 x float> %1853, zeroinitializer, !dbg !12125
  %1855 = bitcast <4 x i32> %history.i.i.sroa.0.018178 to <4 x float>, !dbg !12129
  %1856 = fmul <4 x float> %_25.i.i.i.i15542, %1855, !dbg !12133
  %1857 = fadd <4 x float> %1848, %1856, !dbg !12134
  %1858 = fmul <4 x float> %_28.i.i.i.i15543, %1855, !dbg !12138
  %1859 = fadd <4 x float> %1850, %1858, !dbg !12142
  %1860 = fmul <4 x float> %_31.i.i.i.i15544, %1855, !dbg !12146
  %1861 = fadd <4 x float> %1852, %1860, !dbg !12150
  %1862 = fmul <4 x float> %_34.i.i.i.i15545, %1855, !dbg !12154
  %1863 = fadd <4 x float> %1854, %1862, !dbg !12158
  %1864 = bitcast <4 x i32> %history.i.i.sroa.7.018177 to <4 x float>, !dbg !12162
  %1865 = fmul <4 x float> %_39.i.i.i.i15546, %1864, !dbg !12166
  %1866 = fadd <4 x float> %1857, %1865, !dbg !12167
  %1867 = fmul <4 x float> %_42.i.i.i.i15547, %1864, !dbg !12171
  %1868 = fadd <4 x float> %1859, %1867, !dbg !12175
  %1869 = fmul <4 x float> %_45.i.i.i.i15548, %1864, !dbg !12179
  %1870 = fadd <4 x float> %1861, %1869, !dbg !12183
  %1871 = fmul <4 x float> %_48.i.i.i.i15549, %1864, !dbg !12187
  %1872 = fadd <4 x float> %1863, %1871, !dbg !12191
  %1873 = bitcast <4 x i32> %history.i.i.sroa.10.018176 to <4 x float>, !dbg !12195
  %1874 = fmul <4 x float> %_53.i.i.i.i15550, %1873, !dbg !12199
  %1875 = fadd <4 x float> %1866, %1874, !dbg !12200
  %1876 = fmul <4 x float> %_56.i.i.i.i15551, %1873, !dbg !12204
  %1877 = fadd <4 x float> %1868, %1876, !dbg !12208
  %1878 = fmul <4 x float> %_59.i.i.i.i15552, %1873, !dbg !12212
  %1879 = fadd <4 x float> %1870, %1878, !dbg !12216
  %1880 = fmul <4 x float> %_62.i.i.i.i15553, %1873, !dbg !12220
  %1881 = fadd <4 x float> %1872, %1880, !dbg !12224
  %1882 = bitcast <4 x i32> %history.i.i.sroa.13.018175 to <4 x float>, !dbg !12228
  %1883 = fmul <4 x float> %_67.i.i.i.i15554, %1882, !dbg !12232
  %1884 = fadd <4 x float> %1875, %1883, !dbg !12233
  %1885 = fmul <4 x float> %_70.i.i.i.i15555, %1882, !dbg !12237
  %1886 = fadd <4 x float> %1877, %1885, !dbg !12241
  %1887 = fmul <4 x float> %_73.i.i.i.i15556, %1882, !dbg !12245
  %1888 = fadd <4 x float> %1879, %1887, !dbg !12249
  %1889 = fmul <4 x float> %_76.i.i.i.i15557, %1882, !dbg !12253
  %1890 = fadd <4 x float> %1881, %1889, !dbg !12257
  %1891 = bitcast <4 x i32> %history.i.i.sroa.16.018174 to <4 x float>, !dbg !12261
  %1892 = fmul <4 x float> %_81.i.i.i.i15558, %1891, !dbg !12265
  %1893 = fadd <4 x float> %1884, %1892, !dbg !12266
  %1894 = fmul <4 x float> %_84.i.i.i.i15559, %1891, !dbg !12270
  %1895 = fadd <4 x float> %1886, %1894, !dbg !12274
  %1896 = fmul <4 x float> %_87.i.i.i.i15560, %1891, !dbg !12278
  %1897 = fadd <4 x float> %1888, %1896, !dbg !12282
  %1898 = fmul <4 x float> %_90.i.i.i.i15561, %1891, !dbg !12286
  %1899 = fadd <4 x float> %1890, %1898, !dbg !12290
  %1900 = fmul <4 x float> %_95.i.i.i.i15562, %1844, !dbg !12294
  %1901 = fadd <4 x float> %1893, %1900, !dbg !12298
  %1902 = fmul <4 x float> %_98.i.i.i.i15563, %1844, !dbg !12302
  %1903 = fadd <4 x float> %1895, %1902, !dbg !12306
  %1904 = fmul <4 x float> %_101.i.i.i.i15564, %1844, !dbg !12310
  %1905 = fadd <4 x float> %1897, %1904, !dbg !12314
  %1906 = fmul <4 x float> %_104.i.i.i.i15565, %1844, !dbg !12318
  %1907 = fadd <4 x float> %1899, %1906, !dbg !12322
  %1908 = bitcast <4 x i32> %history.i.i.sroa.22.018172 to <4 x float>, !dbg !12326
  %1909 = fmul <4 x float> %_109.i.i.i.i15566, %1908, !dbg !12330
  %1910 = fadd <4 x float> %1901, %1909, !dbg !12331
  %1911 = fmul <4 x float> %_112.i.i.i.i15567, %1908, !dbg !12335
  %1912 = fadd <4 x float> %1903, %1911, !dbg !12339
  %1913 = fmul <4 x float> %_115.i.i.i.i15568, %1908, !dbg !12343
  %1914 = fadd <4 x float> %1905, %1913, !dbg !12347
  %1915 = fmul <4 x float> %_118.i.i.i.i15569, %1908, !dbg !12351
  %1916 = fadd <4 x float> %1907, %1915, !dbg !12355
  %1917 = bitcast <4 x i32> %history.i.i.sroa.26.018171 to <4 x float>, !dbg !12359
  %1918 = fmul <4 x float> %_123.i.i.i.i15570, %1917, !dbg !12363
  %1919 = fadd <4 x float> %1910, %1918, !dbg !12364
  %1920 = fmul <4 x float> %_126.i.i.i.i15571, %1917, !dbg !12368
  %1921 = fadd <4 x float> %1912, %1920, !dbg !12372
  %1922 = fmul <4 x float> %_129.i.i.i.i15572, %1917, !dbg !12376
  %1923 = fadd <4 x float> %1914, %1922, !dbg !12380
  %1924 = fmul <4 x float> %_132.i.i.i.i15573, %1917, !dbg !12384
  %1925 = fadd <4 x float> %1916, %1924, !dbg !12388
  %1926 = bitcast <4 x i32> %history.i.i.sroa.29.018170 to <4 x float>, !dbg !12392
  %1927 = fmul <4 x float> %_137.i.i.i.i15574, %1926, !dbg !12396
  %1928 = fadd <4 x float> %1919, %1927, !dbg !12397
  %1929 = fmul <4 x float> %_140.i.i.i.i15575, %1926, !dbg !12401
  %1930 = fadd <4 x float> %1921, %1929, !dbg !12405
  %1931 = fmul <4 x float> %_143.i.i.i.i15576, %1926, !dbg !12409
  %1932 = fadd <4 x float> %1923, %1931, !dbg !12413
  %1933 = fmul <4 x float> %_146.i.i.i.i15577, %1926, !dbg !12417
  %1934 = fadd <4 x float> %1925, %1933, !dbg !12421
  %1935 = bitcast <4 x i32> %history.i.i.sroa.32.018169 to <4 x float>, !dbg !12425
  %1936 = fmul <4 x float> %_151.i.i.i.i15578, %1935, !dbg !12429
  %1937 = fadd <4 x float> %1928, %1936, !dbg !12430
  %1938 = fmul <4 x float> %_154.i.i.i.i15579, %1935, !dbg !12434
  %1939 = fadd <4 x float> %1930, %1938, !dbg !12438
  %1940 = fmul <4 x float> %_157.i.i.i.i15580, %1935, !dbg !12442
  %1941 = fadd <4 x float> %1932, %1940, !dbg !12446
  %1942 = fmul <4 x float> %_160.i.i.i.i15581, %1935, !dbg !12450
  %1943 = fadd <4 x float> %1934, %1942, !dbg !12454
  %1944 = bitcast <4 x i32> %history.i.i.sroa.35.018168 to <4 x float>, !dbg !12458
  %1945 = fmul <4 x float> %_165.i.i.i.i15582, %1944, !dbg !12462
  %1946 = fadd <4 x float> %1937, %1945, !dbg !12463
  %1947 = fmul <4 x float> %_168.i.i.i.i15583, %1944, !dbg !12467
  %1948 = fadd <4 x float> %1939, %1947, !dbg !12471
  %1949 = fmul <4 x float> %_171.i.i.i.i15584, %1944, !dbg !12475
  %1950 = fadd <4 x float> %1941, %1949, !dbg !12479
  %1951 = fmul <4 x float> %_174.i.i.i.i15585, %1944, !dbg !12483
  %1952 = fadd <4 x float> %1943, %1951, !dbg !12487
  %1953 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1946), !dbg !12491
  %1954 = fcmp olt <4 x float> %1953, %1845, !dbg !12495
  %1955 = select <4 x i1> %1954, <4 x float> %1845, <4 x float> %1953, !dbg !12499
  %1956 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1948), !dbg !12491
  %1957 = fcmp olt <4 x float> %1956, %1955, !dbg !12495
  %1958 = select <4 x i1> %1957, <4 x float> %1955, <4 x float> %1956, !dbg !12499
  %1959 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1950), !dbg !12491
  %1960 = fcmp olt <4 x float> %1959, %1958, !dbg !12495
  %1961 = select <4 x i1> %1960, <4 x float> %1958, <4 x float> %1959, !dbg !12499
  %1962 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1952), !dbg !12491
  %1963 = fcmp olt <4 x float> %1962, %1961, !dbg !12495
  %1964 = select <4 x i1> %1963, <4 x float> %1961, <4 x float> %1962, !dbg !12499
  %_39.i.i.idx = shl i32 %iter.sroa.0.0.i.i18179, 4, !dbg !12500
  %_39.i.i = getelementptr inbounds nuw i8, ptr %peaks_right.i, i32 %_39.i.i.idx, !dbg !12500
  store <4 x float> %1964, ptr %_39.i.i, align 4, !dbg !12505, !alias.scope !12510, !noalias !12514
  %exitcond20558.not = icmp eq i32 %1843, %umax20557, !dbg !12518
  br i1 %exitcond20558.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i, label %bb5.i.i, !dbg !12051

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit249.i
  %history.i.i.sroa.38.0.lcssa = phi <4 x i32> [ %history.i.i.sroa.38.0.lcssa23165, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit249.i ], [ %history.i.i.sroa.35.018168, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039 ], !dbg !11598
  %history.i.i.sroa.35.0.lcssa = phi <4 x i32> [ %history.i.i.sroa.35.0.lcssa23145, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit249.i ], [ %history.i.i.sroa.32.018169, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039 ], !dbg !11598
  %history.i.i.sroa.32.0.lcssa = phi <4 x i32> [ %history.i.i.sroa.32.0.lcssa23125, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit249.i ], [ %history.i.i.sroa.29.018170, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039 ], !dbg !11598
  %history.i.i.sroa.29.0.lcssa = phi <4 x i32> [ %history.i.i.sroa.29.0.lcssa23105, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit249.i ], [ %history.i.i.sroa.26.018171, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039 ], !dbg !11598
  %history.i.i.sroa.26.0.lcssa = phi <4 x i32> [ %history.i.i.sroa.26.0.lcssa23085, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit249.i ], [ %history.i.i.sroa.22.018172, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039 ], !dbg !11598
  %history.i.i.sroa.22.0.lcssa = phi <4 x i32> [ %history.i.i.sroa.22.0.lcssa23065, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit249.i ], [ %history.i.i.sroa.19.018173, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039 ], !dbg !11598
  %history.i.i.sroa.19.0.lcssa = phi <4 x i32> [ %history.i.i.sroa.19.0.lcssa23045, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit249.i ], [ %history.i.i.sroa.16.018174, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039 ], !dbg !11598
  %history.i.i.sroa.16.0.lcssa = phi <4 x i32> [ %history.i.i.sroa.16.0.lcssa23025, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit249.i ], [ %history.i.i.sroa.13.018175, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039 ], !dbg !11598
  %history.i.i.sroa.13.0.lcssa = phi <4 x i32> [ %history.i.i.sroa.13.0.lcssa23005, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit249.i ], [ %history.i.i.sroa.10.018176, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039 ], !dbg !11598
  %history.i.i.sroa.10.0.lcssa = phi <4 x i32> [ %history.i.i.sroa.10.0.lcssa22985, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit249.i ], [ %history.i.i.sroa.7.018177, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039 ], !dbg !11598
  %history.i.i.sroa.7.0.lcssa = phi <4 x i32> [ %history.i.i.sroa.7.0.lcssa22965, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit249.i ], [ %history.i.i.sroa.0.018178, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039 ], !dbg !11598
  %history.i.i.sroa.0.0.lcssa = phi <4 x i32> [ %history.i.i.sroa.0.0.lcssa22945, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit249.i ], [ %lanes.i5374.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6039 ], !dbg !11598
  br i1 %_20.i44.i18140.not, label %bb15.i.loopexit, label %bb20.i.lr.ph, !dbg !12520

bb20.i.lr.ph:                                     ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i
  %_68.i.sroa.3.0.copyload.pre = load i32, ptr %_68.i.sroa.3.0..sroa_idx, align 4, !dbg !12522, !noalias !11535
  %_68.i.sroa.4.0.copyload.pre = load i32, ptr %_68.i.sroa.4.0..sroa_idx, align 4, !dbg !12522, !noalias !11535
  %_69.i.sroa.3.0.copyload.pre = load i32, ptr %_69.i.sroa.3.0..sroa_idx, align 4, !dbg !12523, !noalias !11535
  %_69.i.sroa.4.0.copyload.pre = load i32, ptr %_69.i.sroa.4.0..sroa_idx, align 4, !dbg !12523, !noalias !11535
  %_8.i26.i15507.pre = load <4 x float>, ptr %_110.i, align 16
  %_9.i27.i15508.pre = load <4 x float>, ptr %_111.i, align 16
  %_8.i.i15509.pre = load <4 x float>, ptr %_115.i, align 16
  %_9.i.i15510.pre = load <4 x float>, ptr %_116.i, align 16
  %_54.0.i263.i.pre = load ptr, ptr %1699, align 16
  %_54.1.i264.i.pre = load i32, ptr %1700, align 4
  %_18.i275.i = load i32, ptr %1696, align 4
  %_29.i133018192.not = icmp eq i32 %_18.i275.i, 0
  %_56.0.i285.i = load ptr, ptr %1701, align 8, !nonnull !10, !align !10189
  %_56.1.i286.i = load i32, ptr %1702, align 4
  %_37.i302.i15518 = load <4 x float>, ptr %1704, align 16
  %_58.1.i316.i = load i32, ptr %1706, align 4
  %_58.0.i315.i = load ptr, ptr %1707, align 16, !nonnull !10, !align !10189
  %_54.0.i.i = load ptr, ptr %1709, align 16, !nonnull !10, !align !10189
  %_54.1.i.i = load i32, ptr %1710, align 4
  %_18.i250.i = load i32, ptr %1697, align 4
  %_29.i129618195.not = icmp eq i32 %_18.i250.i, 0
  %_56.0.i.i = load ptr, ptr %1711, align 8, !nonnull !10, !align !10189
  %_56.1.i.i = load i32, ptr %1712, align 4
  %_37.i.i15527 = load <4 x float>, ptr %1714, align 16
  %_58.1.i.i = load i32, ptr %1716, align 4
  %_58.0.i.i = load ptr, ptr %1717, align 16, !nonnull !10, !align !10189
  br label %bb20.i, !dbg !12520

bb20.i:                                           ; preds = %bb20.i.lr.ph, %bb67.i
  %.lcssa1831018431 = phi <4 x float> [ %.lcssa1831018430.lcssa23251, %bb20.i.lr.ph ], [ %.lcssa1831018430, %bb67.i ]
  %storemerge.i1291.lcssa1828018395 = phi i32 [ %storemerge.i1291.lcssa1828018394.lcssa23229, %bb20.i.lr.ph ], [ %storemerge.i1291.lcssa1828018394, %bb67.i ]
  %.lcssa1825318359 = phi <4 x float> [ %.lcssa1825318358.lcssa23208, %bb20.i.lr.ph ], [ %.lcssa1825318358, %bb67.i ]
  %storemerge.i1325.lcssa1822318323 = phi i32 [ %storemerge.i1325.lcssa1822318322.lcssa23186, %bb20.i.lr.ph ], [ %storemerge.i1325.lcssa1822318322, %bb67.i ]
  %frame.sroa.0.0.i18318 = phi i32 [ 0, %bb20.i.lr.ph ], [ %_83.i, %bb67.i ]
  %main_cursor.sroa.0.1.i18317 = phi i32 [ %main_cursor.sroa.0.0.i18467, %bb20.i.lr.ph ], [ %main_cursor.sroa.0.2.i, %bb67.i ]
  %ring_cursor.sroa.0.1.i18316 = phi i32 [ %ring_cursor.sroa.0.0.i18466, %bb20.i.lr.ph ], [ %ring_cursor.sroa.0.2.i, %bb67.i ]
  %_65.i = sub nuw nsw i32 %spec.store.select.i, %frame.sroa.0.0.i18318, !dbg !12524
  %ring.i1921 = load i32, ptr %84, align 4, !dbg !12525, !alias.scope !12527, !noalias !12530, !noundef !10
  %main.i1922 = load i32, ptr %85, align 4, !dbg !12534, !alias.scope !12527, !noalias !12530, !noundef !10
  %_10.i1923 = add i32 %ring_cursor.sroa.0.1.i18316, 1, !dbg !12535
  %_38.not.i1924 = icmp ult i32 %_10.i1923, %ring.i1921, !dbg !12536
  %1965 = select i1 %_38.not.i1924, i32 0, i32 %ring.i1921, !dbg !12536
  %start1.sroa.0.0.i1925 = sub nuw i32 %_10.i1923, %1965, !dbg !12536
  %_12.i1927 = add i32 %_68.i.sroa.3.0.copyload.pre, %ring_cursor.sroa.0.1.i18316, !dbg !12538
  %_39.not.i1928 = icmp ult i32 %_12.i1927, %ring.i1921, !dbg !12539
  %1966 = select i1 %_39.not.i1928, i32 0, i32 %ring.i1921, !dbg !12539
  %left_end.sroa.0.0.i1929 = sub nuw i32 %_12.i1927, %1966, !dbg !12539
  %_15.i1931 = add i32 %_69.i.sroa.3.0.copyload.pre, %ring_cursor.sroa.0.1.i18316, !dbg !12541
  %_40.not.i1932 = icmp ult i32 %_15.i1931, %ring.i1921, !dbg !12542
  %1967 = select i1 %_40.not.i1932, i32 0, i32 %ring.i1921, !dbg !12542
  %right_end.sroa.0.0.i1933 = sub nuw i32 %_15.i1931, %1967, !dbg !12542
  %_18.i1935 = add i32 %_68.i.sroa.4.0.copyload.pre, %ring_cursor.sroa.0.1.i18316, !dbg !12544
  %_41.not.i1936 = icmp ult i32 %_18.i1935, %ring.i1921, !dbg !12545
  %1968 = select i1 %_41.not.i1936, i32 0, i32 %ring.i1921, !dbg !12545
  %left_expiring.sroa.0.0.i1937 = sub nuw i32 %_18.i1935, %1968, !dbg !12545
  %_21.i1939 = add i32 %_69.i.sroa.4.0.copyload.pre, %ring_cursor.sroa.0.1.i18316, !dbg !12547
  %_42.not.i1940 = icmp ult i32 %_21.i1939, %ring.i1921, !dbg !12548
  %1969 = select i1 %_42.not.i1940, i32 0, i32 %ring.i1921, !dbg !12548
  %right_expiring.sroa.0.0.i1941 = sub nuw i32 %_21.i1939, %1969, !dbg !12548
  %1970 = sub i32 %ring.i1921, %ring_cursor.sroa.0.1.i18316, !dbg !12550
  %spec.store.select.i1942 = tail call i32 @llvm.umin.i32(i32 %1970, i32 %_65.i), !dbg !12551
  %1971 = sub i32 %main.i1922, %main_cursor.sroa.0.1.i18317, !dbg !12553
  %_24.sroa.0.0.i1944 = tail call i32 @llvm.umin.i32(i32 %1971, i32 %spec.store.select.i1942), !dbg !12554
  %1972 = sub i32 %ring.i1921, %start1.sroa.0.0.i1925, !dbg !12556
  %_25.sroa.0.0.i1946 = tail call i32 @llvm.umin.i32(i32 %1972, i32 %_24.sroa.0.0.i1944), !dbg !12557
  %1973 = sub i32 %ring.i1921, %left_end.sroa.0.0.i1929, !dbg !12559
  %_27.sroa.0.0.i1948 = tail call i32 @llvm.umin.i32(i32 %1973, i32 %_25.sroa.0.0.i1946), !dbg !12560
  %1974 = sub i32 %ring.i1921, %right_end.sroa.0.0.i1933, !dbg !12562
  %_29.sroa.0.0.i1950 = tail call i32 @llvm.umin.i32(i32 %1974, i32 %_27.sroa.0.0.i1948), !dbg !12563
  %1975 = sub i32 %ring.i1921, %left_expiring.sroa.0.0.i1937, !dbg !12565
  %_31.sroa.0.0.i1952 = tail call i32 @llvm.umin.i32(i32 %1975, i32 %_29.sroa.0.0.i1950), !dbg !12566
  %1976 = sub i32 %ring.i1921, %right_expiring.sroa.0.0.i1941, !dbg !12568
  %run.sroa.0.0.i1954 = tail call i32 @llvm.umin.i32(i32 %1976, i32 %_31.sroa.0.0.i1952), !dbg !12569
  %_72.i = add i32 %frame.sroa.0.0.i18318, %iter1.sroa.0.0.i18468, !dbg !12571
  %base.i = shl i32 %_72.i, 2, !dbg !12571
  %base.i15506 = add i32 %run.sroa.0.0.i1954, %_72.i, !dbg !12574
  %_76.i = shl i32 %base.i15506, 2, !dbg !12574
  %_172.i = icmp ult i32 %_76.i, %base.i, !dbg !12577
  %_166.not.i = icmp ugt i32 %_76.i, %left_io.1
  %or.cond.i = or i1 %_172.i, %_166.not.i, !dbg !12577
  br i1 %or.cond.i, label %bb51.i, label %bb49.i, !dbg !12577, !prof !4694

bb51.i:                                           ; preds = %bb20.i
  store <4 x i32> %history.i41.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.7.0.lcssa, ptr %history.i41.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.10.0.lcssa, ptr %history.i41.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.13.0.lcssa, ptr %history.i41.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.16.0.lcssa, ptr %history.i41.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.19.0.lcssa, ptr %history.i41.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.22.0.lcssa, ptr %history.i41.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.26.0.lcssa, ptr %history.i41.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.29.0.lcssa, ptr %history.i41.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.32.0.lcssa, ptr %history.i41.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.35.0.lcssa, ptr %history.i41.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.38.0.lcssa, ptr %history.i41.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store i32 %storemerge.i1325.lcssa1822318322.lcssa23186, ptr %_22.i278.i, align 4
  store <4 x float> %.lcssa1825318358.lcssa23208, ptr %1703, align 16
  store i32 %storemerge.i1291.lcssa1828018394.lcssa23229, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1831018430.lcssa23251, ptr %1713, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i, i32 noundef %_76.i, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_bd3c2aadcc2cdd6816f02be0ceab2cc1) #33, !dbg !12584, !noalias !11506
  unreachable, !dbg !12584

bb49.i:                                           ; preds = %bb20.i
  %_175.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %base.i, !dbg !12585
  %_176.not.i = icmp ugt i32 %_76.i, %right_io.1, !dbg !12589
  br i1 %_176.not.i, label %bb54.i, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit6747, !dbg !12589, !prof !902

bb54.i:                                           ; preds = %bb49.i
  store <4 x i32> %history.i41.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.7.0.lcssa, ptr %history.i41.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.10.0.lcssa, ptr %history.i41.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.13.0.lcssa, ptr %history.i41.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.16.0.lcssa, ptr %history.i41.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.19.0.lcssa, ptr %history.i41.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.22.0.lcssa, ptr %history.i41.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.26.0.lcssa, ptr %history.i41.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.29.0.lcssa, ptr %history.i41.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.32.0.lcssa, ptr %history.i41.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.35.0.lcssa, ptr %history.i41.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.38.0.lcssa, ptr %history.i41.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store i32 %storemerge.i1325.lcssa1822318322.lcssa23186, ptr %_22.i278.i, align 4
  store <4 x float> %.lcssa1825318358.lcssa23208, ptr %1703, align 16
  store i32 %storemerge.i1291.lcssa1828018394.lcssa23229, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1831018430.lcssa23251, ptr %1713, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i, i32 noundef %_76.i, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c1156ffa98f252967759477badafe965) #33, !dbg !12594, !noalias !11506
  unreachable, !dbg !12594

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit6747: ; preds = %bb49.i
  %_183.i = getelementptr inbounds nuw float, ptr %right_io.0, i32 %base.i, !dbg !12595
  %_83.i = add nuw nsw i32 %run.sroa.0.0.i1954, %frame.sroa.0.0.i18318, !dbg !12599
  %_80.i = shl nuw nsw i32 %frame.sroa.0.0.i18318, 2, !dbg !12601
  %_192.i = getelementptr inbounds nuw float, ptr %peaks_left.i, i32 %_80.i, !dbg !12602
  %_201.i = getelementptr inbounds nuw float, ptr %peaks_right.i, i32 %_80.i, !dbg !12612
  %_2.i675018198.not = icmp eq i32 %run.sroa.0.0.i1954, 0, !dbg !12622
  br i1 %_2.i675018198.not, label %bb67.i, label %bb68.i.preheader, !dbg !12622

bb68.i.preheader:                                 ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit6747
  %umin20569 = call i32 @llvm.umin.i32(i32 %1973, i32 %1974)
  %umin20570 = call i32 @llvm.umin.i32(i32 %umin20569, i32 %1975)
  %umin20571 = call i32 @llvm.umin.i32(i32 %umin20570, i32 %1976)
  %umin20572 = call i32 @llvm.umin.i32(i32 %umin20571, i32 %1972)
  %umin20573 = call i32 @llvm.umin.i32(i32 %umin20572, i32 %1970)
  %umin20574 = call i32 @llvm.umin.i32(i32 %umin20573, i32 %1971)
  %1977 = sub nsw i32 %umin20575, %frame.sroa.0.0.i18318
  %umin20576 = call i32 @llvm.umin.i32(i32 %umin20574, i32 %1977)
  %1978 = and i32 %umin20576, 1073741823
  br label %bb68.i

bb68.i:                                           ; preds = %bb68.i.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1531
  %1979 = phi <4 x float> [ %.lcssa1831018431, %bb68.i.preheader ], [ %2068, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1531 ]
  %storemerge.i129118258 = phi i32 [ %storemerge.i1291.lcssa1828018395, %bb68.i.preheader ], [ %storemerge.i1291, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1531 ]
  %1980 = phi <4 x float> [ %.lcssa1825318359, %bb68.i.preheader ], [ %2019, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1531 ]
  %storemerge.i132518201 = phi i32 [ %storemerge.i1325.lcssa1822318323, %bb68.i.preheader ], [ %storemerge.i1325, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1531 ]
  %iter.i.sroa.36.018200 = phi i32 [ 0, %bb68.i.preheader ], [ %1981, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1531 ]
  %start1.i.i.i.i.i.i6760 = shl i32 %iter.i.sroa.36.018200, 2, !dbg !12631
  %data.i.i.i.i6770 = getelementptr inbounds nuw float, ptr %_192.i, i32 %start1.i.i.i.i.i.i6760, !dbg !12637
  %lanes.i5410.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i.i.i6770, align 4, !dbg !12640, !alias.scope !12648, !noalias !12652
  %data.i.i6775 = getelementptr inbounds nuw float, ptr %_201.i, i32 %start1.i.i.i.i.i.i6760, !dbg !12656
  %lanes.i5401.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i6775, align 4, !dbg !12659, !alias.scope !12665, !noalias !12669
  %data.i.i.i.i.i.i6761 = getelementptr inbounds nuw float, ptr %_175.i, i32 %start1.i.i.i.i.i.i6760, !dbg !12673
  %lanes.i5392.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i.i.i.i.i6761, align 4, !dbg !12675, !alias.scope !12684, !noalias !12688
  %data.i5.i.i.i.i.i6765 = getelementptr inbounds nuw float, ptr %_183.i, i32 %start1.i.i.i.i.i.i6760, !dbg !12692
  %lanes.i5383.sroa.0.0.copyload = load <4 x i32>, ptr %data.i5.i.i.i.i.i6765, align 4, !dbg !12695, !alias.scope !12701, !noalias !12705
  %1981 = add nuw nsw i32 %iter.i.sroa.36.018200, 1, !dbg !12709
  %1982 = bitcast <4 x i32> %lanes.i5410.sroa.0.0.copyload to <4 x float>, !dbg !12710
  %1983 = bitcast <4 x i32> %lanes.i5401.sroa.0.0.copyload to <4 x float>, !dbg !12714
  %1984 = fcmp olt <4 x float> %1982, %1983, !dbg !12715
  %.v15511 = select <4 x i1> %1984, <4 x i32> %lanes.i5401.sroa.0.0.copyload, <4 x i32> %lanes.i5410.sroa.0.0.copyload, !dbg !12716
  %1985 = bitcast <4 x i32> %.v15511 to <16 x i8>, !dbg !12717
  %1986 = bitcast <4 x i32> %lanes.i5401.sroa.0.0.copyload to <16 x i8>, !dbg !12721
  %_4.i6787 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1985, <16 x i8> %1986, <16 x i8> %1698), !dbg !12725
  %_213.i = add i32 %iter.i.sroa.36.018200, %ring_cursor.sroa.0.1.i18316, !dbg !12726
  %_214.i = add i32 %iter.i.sroa.36.018200, %main_cursor.sroa.0.1.i18317, !dbg !12730
  %_215.i = add i32 %iter.i.sroa.36.018200, %left_end.sroa.0.0.i1929, !dbg !12731
  %_216.i = add i32 %iter.i.sroa.36.018200, %start1.sroa.0.0.i1925, !dbg !12732
  %_217.i = add i32 %iter.i.sroa.36.018200, %left_expiring.sroa.0.0.i1937, !dbg !12733
  %base.i9.i266.i = shl i32 %_213.i, 2, !dbg !12734
  %_7.i10.i267.i = add i32 %base.i9.i266.i, 4, !dbg !12737
  %1987 = or disjoint i32 %base.i9.i266.i, 3, !dbg !12738
  %or.cond.i13.i270.i.not = icmp ult i32 %1987, %_54.1.i264.i.pre, !dbg !12738
  br i1 %or.cond.i13.i270.i.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i271.i, label %bb4.i15.i331.i, !dbg !12738, !prof !10587

bb4.i15.i331.i:                                   ; preds = %bb68.i
  store <4 x i32> %history.i41.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.7.0.lcssa, ptr %history.i41.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.10.0.lcssa, ptr %history.i41.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.13.0.lcssa, ptr %history.i41.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.16.0.lcssa, ptr %history.i41.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.19.0.lcssa, ptr %history.i41.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.22.0.lcssa, ptr %history.i41.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.26.0.lcssa, ptr %history.i41.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.29.0.lcssa, ptr %history.i41.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.32.0.lcssa, ptr %history.i41.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.35.0.lcssa, ptr %history.i41.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.38.0.lcssa, ptr %history.i41.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store i32 %storemerge.i1325.lcssa1822318322.lcssa23186, ptr %_22.i278.i, align 4
  store <4 x float> %.lcssa1825318358.lcssa23208, ptr %1703, align 16
  store i32 %storemerge.i1291.lcssa1828018394.lcssa23229, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1831018430.lcssa23251, ptr %1713, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i9.i266.i, i32 noundef %_7.i10.i267.i, i32 noundef range(i32 0, 536870912) %_54.1.i264.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #33, !dbg !12742, !noalias !12743
  unreachable, !dbg !12742

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i271.i: ; preds = %bb68.i
  %1988 = bitcast <4 x i32> %lanes.i5410.sroa.0.0.copyload to <16 x i8>, !dbg !12757
  %_4.i6786 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1985, <16 x i8> %1988, <16 x i8> %1698), !dbg !12758
  %1989 = bitcast <16 x i8> %_4.i6786 to <4 x float>, !dbg !12759
  %1990 = fdiv <4 x float> %_8.i26.i15507.pre, %1989, !dbg !12764
  %1991 = bitcast <4 x float> %1990 to <16 x i8>, !dbg !12768
  %1992 = fcmp olt <4 x float> %_8.i26.i15507.pre, %1989, !dbg !12772
  %1993 = sext <4 x i1> %1992 to <4 x i32>, !dbg !12772
  %1994 = bitcast <4 x i32> %1993 to <16 x i8>, !dbg !12773
  %_4.i6790 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1991, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %1994), !dbg !12774
  %_17.i14.i272.i = getelementptr inbounds nuw float, ptr %_54.0.i263.i.pre, i32 %base.i9.i266.i, !dbg !12775
  store <16 x i8> %_4.i6790, ptr %_17.i14.i272.i, align 4, !dbg !12777, !alias.scope !12782, !noalias !12786
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12790), !dbg !12793
  %base.i1370 = shl i32 %_215.i, 2, !dbg !12794
  %1995 = or disjoint i32 %base.i1370, 3, !dbg !12797
  %or.cond.i1374.not = icmp ult i32 %1995, %_54.1.i264.i.pre, !dbg !12797
  br i1 %or.cond.i1374.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1378, label %bb4.i1377, !dbg !12797, !prof !10587

bb4.i1377:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i271.i
  store <4 x i32> %history.i41.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.7.0.lcssa, ptr %history.i41.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.10.0.lcssa, ptr %history.i41.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.13.0.lcssa, ptr %history.i41.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.16.0.lcssa, ptr %history.i41.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.19.0.lcssa, ptr %history.i41.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.22.0.lcssa, ptr %history.i41.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.26.0.lcssa, ptr %history.i41.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.29.0.lcssa, ptr %history.i41.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.32.0.lcssa, ptr %history.i41.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.35.0.lcssa, ptr %history.i41.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.38.0.lcssa, ptr %history.i41.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store i32 %storemerge.i1325.lcssa1822318322.lcssa23186, ptr %_22.i278.i, align 4
  store <4 x float> %.lcssa1825318358.lcssa23208, ptr %1703, align 16
  store i32 %storemerge.i1291.lcssa1828018394.lcssa23229, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1831018430.lcssa23251, ptr %1713, align 16
  %_5.i1371 = add i32 %base.i1370, 4, !dbg !12801
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1370, i32 noundef %_5.i1371, i32 noundef range(i32 0, 536870912) %_54.1.i264.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !12802, !noalias !12803
  unreachable, !dbg !12802

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1378: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i271.i
  %_15.i1376 = getelementptr inbounds nuw float, ptr %_54.0.i263.i.pre, i32 %base.i1370, !dbg !12809
  %lanes.i5083.sroa.0.0.copyload = load <4 x i32>, ptr %_15.i1376, align 4, !dbg !12811
  %1996 = icmp eq i32 %storemerge.i132518201, 0, !dbg !12816
  %_12.i131715512 = load <4 x float>, ptr %uniform_left.i, align 16, !dbg !12816
  %1997 = bitcast <4 x i32> %lanes.i5083.sroa.0.0.copyload to <4 x float>, !dbg !12816
  %1998 = fcmp olt <4 x float> %_12.i131715512, %1997, !dbg !12816
  %1999 = select <4 x i1> %1998, <4 x float> %_12.i131715512, <4 x float> %1997, !dbg !12816
  %2000 = bitcast <4 x float> %1999 to <4 x i32>, !dbg !12816
  %.sroa.07802.0 = select i1 %1996, <4 x i32> %lanes.i5083.sroa.0.0.copyload, <4 x i32> %2000, !dbg !12816
  store <4 x i32> %.sroa.07802.0, ptr %uniform_left.i, align 16, !dbg !12817, !alias.scope !12790, !noalias !12818
  %_15.i1320 = add i32 %storemerge.i132518201, 1, !dbg !12820
  %complete.i1321 = icmp eq i32 %_15.i1320, %_18.i275.i, !dbg !12820
  br i1 %complete.i1321, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1360, label %bb7.i1322, !dbg !12821

bb7.i1322:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1378
  %base.i1361 = shl i32 %_216.i, 2, !dbg !12822
  %2001 = or disjoint i32 %base.i1361, 3, !dbg !12824
  %or.cond.i1365.not = icmp ult i32 %2001, %_54.1.i264.i.pre, !dbg !12824
  br i1 %or.cond.i1365.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1369, label %bb4.i1368, !dbg !12824, !prof !10587

bb4.i1368:                                        ; preds = %bb7.i1322
  store <4 x i32> %history.i41.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.7.0.lcssa, ptr %history.i41.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.10.0.lcssa, ptr %history.i41.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.13.0.lcssa, ptr %history.i41.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.16.0.lcssa, ptr %history.i41.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.19.0.lcssa, ptr %history.i41.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.22.0.lcssa, ptr %history.i41.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.26.0.lcssa, ptr %history.i41.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.29.0.lcssa, ptr %history.i41.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.32.0.lcssa, ptr %history.i41.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.35.0.lcssa, ptr %history.i41.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.38.0.lcssa, ptr %history.i41.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store i32 %storemerge.i1325.lcssa1822318322.lcssa23186, ptr %_22.i278.i, align 4
  store <4 x float> %.lcssa1825318358.lcssa23208, ptr %1703, align 16
  store i32 %storemerge.i1291.lcssa1828018394.lcssa23229, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1831018430.lcssa23251, ptr %1713, align 16
  %_5.i1362 = add i32 %base.i1361, 4, !dbg !12828
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1361, i32 noundef %_5.i1362, i32 noundef range(i32 0, 536870912) %_54.1.i264.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !12829, !noalias !12830
  unreachable, !dbg !12829

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1369: ; preds = %bb7.i1322
  %_15.i1367 = getelementptr inbounds nuw float, ptr %_54.0.i263.i.pre, i32 %base.i1361, !dbg !12834
  %lanes.i5090.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1367, align 4, !dbg !12836, !alias.scope !12841, !noalias !12845
  %2002 = bitcast <4 x i32> %.sroa.07802.0 to <4 x float>, !dbg !12849
  %2003 = fcmp olt <4 x float> %lanes.i5090.sroa.0.0.copyload, %2002, !dbg !12853
  %2004 = select <4 x i1> %2003, <4 x float> %lanes.i5090.sroa.0.0.copyload, <4 x float> %2002, !dbg !12854
  %2005 = bitcast <4 x float> %2004 to <4 x i32>, !dbg !12855
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1346, !dbg !12857

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1360: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1378
  %2006 = bitcast <4 x i32> %lanes.i5083.sroa.0.0.copyload to <4 x float>, !dbg !12821
  br i1 %_29.i133018192.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1346, label %bb19.i1331, !dbg !12858

bb19.i1331:                                       ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1360, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
  %end.sroa.0.0.i132918194 = phi i32 [ %2012, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit ], [ %_215.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1360 ]
  %iter.sroa.0.0.i132818193 = phi i32 [ %_30.i1332, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1360 ]
  %2007 = phi <4 x float> [ %2010, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit ], [ %2006, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1360 ]
  %base.i1347 = shl i32 %end.sroa.0.0.i132918194, 2, !dbg !12861
  %2008 = or disjoint i32 %base.i1347, 3, !dbg !12863
  %or.cond.i1349.not = icmp ult i32 %2008, %_54.1.i264.i.pre, !dbg !12863
  br i1 %or.cond.i1349.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb4.i, !dbg !12863, !prof !10587

bb4.i:                                            ; preds = %bb19.i1331
  store <4 x i32> %history.i41.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.7.0.lcssa, ptr %history.i41.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.10.0.lcssa, ptr %history.i41.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.13.0.lcssa, ptr %history.i41.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.16.0.lcssa, ptr %history.i41.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.19.0.lcssa, ptr %history.i41.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.22.0.lcssa, ptr %history.i41.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.26.0.lcssa, ptr %history.i41.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.29.0.lcssa, ptr %history.i41.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.32.0.lcssa, ptr %history.i41.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.35.0.lcssa, ptr %history.i41.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.38.0.lcssa, ptr %history.i41.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store i32 %storemerge.i1325.lcssa1822318322.lcssa23186, ptr %_22.i278.i, align 4
  store <4 x float> %.lcssa1825318358.lcssa23208, ptr %1703, align 16
  store i32 %storemerge.i1291.lcssa1828018394.lcssa23229, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1831018430.lcssa23251, ptr %1713, align 16
  %_5.i = add i32 %base.i1347, 4, !dbg !12867
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1347, i32 noundef %_5.i, i32 noundef range(i32 0, 536870912) %_54.1.i264.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !12868, !noalias !12869
  unreachable, !dbg !12868

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %bb19.i1331
  %_30.i1332 = add nuw i32 %iter.sroa.0.0.i132818193, 1, !dbg !12873
  %_15.i1351 = getelementptr inbounds nuw float, ptr %_54.0.i263.i.pre, i32 %base.i1347, !dbg !12876
  %lanes.i5104.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1351, align 4, !dbg !12878, !alias.scope !12883, !noalias !12887
  %2009 = fcmp olt <4 x float> %2007, %lanes.i5104.sroa.0.0.copyload, !dbg !12891
  %2010 = select <4 x i1> %2009, <4 x float> %2007, <4 x float> %lanes.i5104.sroa.0.0.copyload, !dbg !12895
  store <4 x float> %2010, ptr %_15.i1351, align 4, !dbg !12896, !alias.scope !12902, !noalias !12906
  %2011 = icmp eq i32 %end.sroa.0.0.i132918194, 0, !dbg !12912
  %spec.store.select.i1343 = select i1 %2011, i32 %ring.i, i32 %end.sroa.0.0.i132918194, !dbg !12912
  %2012 = add i32 %spec.store.select.i1343, -1, !dbg !12913
  %exitcond20560.not = icmp eq i32 %_30.i1332, %_18.i275.i, !dbg !12914
  br i1 %exitcond20560.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1346, label %bb19.i1331, !dbg !12858

_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1346: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1360, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1369
  %.sroa.07802.1 = phi <4 x i32> [ %2005, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1369 ], [ %.sroa.07802.0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1360 ], [ %.sroa.07802.0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit ], !dbg !12916
  %storemerge.i1325 = phi i32 [ %_15.i1320, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1369 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1360 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit ], !dbg !12917
  %2013 = bitcast <4 x i32> %.sroa.07802.1 to <4 x float>, !dbg !12918
  %2014 = fmul <4 x float> %2013, splat (float 1.638400e+04), !dbg !12922
  %2015 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %2014), !dbg !12923
  %2016 = fmul <4 x float> %2015, splat (float 0x3F10000000000000), !dbg !12927
  %base.i1550 = shl i32 %_217.i, 2, !dbg !12931
  %2017 = or disjoint i32 %base.i1550, 3, !dbg !12933
  %or.cond.i1554.not = icmp ult i32 %2017, %_56.1.i286.i, !dbg !12933
  br i1 %or.cond.i1554.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1558, label %bb4.i1557, !dbg !12933, !prof !10587

bb4.i1557:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1346
  store <4 x i32> %history.i41.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.7.0.lcssa, ptr %history.i41.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.10.0.lcssa, ptr %history.i41.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.13.0.lcssa, ptr %history.i41.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.16.0.lcssa, ptr %history.i41.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.19.0.lcssa, ptr %history.i41.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.22.0.lcssa, ptr %history.i41.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.26.0.lcssa, ptr %history.i41.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.29.0.lcssa, ptr %history.i41.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.32.0.lcssa, ptr %history.i41.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.35.0.lcssa, ptr %history.i41.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.38.0.lcssa, ptr %history.i41.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store i32 %storemerge.i1325.lcssa1822318322.lcssa23186, ptr %_22.i278.i, align 4
  store <4 x float> %.lcssa1825318358.lcssa23208, ptr %1703, align 16
  store i32 %storemerge.i1291.lcssa1828018394.lcssa23229, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1831018430.lcssa23251, ptr %1713, align 16
  %_5.i1551 = add i32 %base.i1550, 4, !dbg !12937
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1550, i32 noundef %_5.i1551, i32 noundef range(i32 0, 536870912) %_56.1.i286.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !12938, !noalias !12939
  unreachable, !dbg !12938

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1558: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1346
  %_15.i1556 = getelementptr inbounds nuw float, ptr %_56.0.i285.i, i32 %base.i1550, !dbg !12943
  %lanes.i.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1556, align 4, !dbg !12945, !alias.scope !12950, !noalias !12954
  %2018 = fadd <4 x float> %2016, %1980, !dbg !12958
  %2019 = fsub <4 x float> %2018, %lanes.i.sroa.0.0.copyload, !dbg !12962
  %_8.not.i4.i297.i = icmp ugt i32 %_7.i10.i267.i, %_56.1.i286.i
  br i1 %_8.not.i4.i297.i, label %bb4.i7.i330.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i299.i, !dbg !12966, !prof !4694

bb4.i7.i330.i:                                    ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1558
  store <4 x i32> %history.i41.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.7.0.lcssa, ptr %history.i41.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.10.0.lcssa, ptr %history.i41.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.13.0.lcssa, ptr %history.i41.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.16.0.lcssa, ptr %history.i41.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.19.0.lcssa, ptr %history.i41.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.22.0.lcssa, ptr %history.i41.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.26.0.lcssa, ptr %history.i41.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.29.0.lcssa, ptr %history.i41.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.32.0.lcssa, ptr %history.i41.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.35.0.lcssa, ptr %history.i41.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.38.0.lcssa, ptr %history.i41.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store i32 %storemerge.i1325.lcssa1822318322.lcssa23186, ptr %_22.i278.i, align 4
  store <4 x float> %.lcssa1825318358.lcssa23208, ptr %1703, align 16
  store i32 %storemerge.i1291.lcssa1828018394.lcssa23229, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1831018430.lcssa23251, ptr %1713, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i9.i266.i, i32 noundef %_7.i10.i267.i, i32 noundef range(i32 0, 536870912) %_56.1.i286.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #33, !dbg !12971, !noalias !12972
  unreachable, !dbg !12971

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i299.i: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1558
  %_17.i6.i300.i = getelementptr inbounds nuw float, ptr %_56.0.i285.i, i32 %base.i9.i266.i, !dbg !12976
  store <4 x float> %2016, ptr %_17.i6.i300.i, align 4, !dbg !12978, !alias.scope !12983, !noalias !12987
  %_41.i305.i15519 = load <4 x float>, ptr %1705, align 16, !dbg !12991
  %2020 = fdiv <4 x float> %2019, %_37.i302.i15518, !dbg !12992
  %2021 = fsub <4 x float> splat (float 1.000000e+00), %2020, !dbg !12996
  %2022 = fsub <4 x float> %2021, %_41.i305.i15519, !dbg !13000
  %2023 = fmul <4 x float> %_9.i27.i15508.pre, %2022, !dbg !13004
  %2024 = fadd <4 x float> %_41.i305.i15519, %2023, !dbg !13008
  %2025 = fcmp olt <4 x float> %2024, %2021, !dbg !13011
  %2026 = select <4 x i1> %2025, <4 x float> %2021, <4 x float> %2024, !dbg !13015
  %2027 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %2026), !dbg !13016
  %2028 = fcmp uge <4 x float> %2027, splat (float 0x3BC79CA100000000), !dbg !13021
  %2029 = bitcast <4 x float> %2026 to <4 x i32>, !dbg !13026
  %2030 = select <4 x i1> %2028, <4 x i32> %2029, <4 x i32> zeroinitializer, !dbg !13026
  store <4 x i32> %2030, ptr %1705, align 16, !dbg !13029
  %base.i1541 = shl i32 %_214.i, 2, !dbg !13030
  %_5.i1542 = add i32 %base.i1541, 4, !dbg !13032
  %2031 = or disjoint i32 %base.i1541, 3, !dbg !13033
  %or.cond.i1545.not = icmp ult i32 %2031, %_58.1.i316.i, !dbg !13033
  br i1 %or.cond.i1545.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1549, label %bb4.i1548, !dbg !13033, !prof !10587

bb4.i1548:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i299.i
  store <4 x i32> %history.i41.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.7.0.lcssa, ptr %history.i41.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.10.0.lcssa, ptr %history.i41.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.13.0.lcssa, ptr %history.i41.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.16.0.lcssa, ptr %history.i41.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.19.0.lcssa, ptr %history.i41.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.22.0.lcssa, ptr %history.i41.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.26.0.lcssa, ptr %history.i41.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.29.0.lcssa, ptr %history.i41.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.32.0.lcssa, ptr %history.i41.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.35.0.lcssa, ptr %history.i41.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.38.0.lcssa, ptr %history.i41.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store i32 %storemerge.i1325.lcssa1822318322.lcssa23186, ptr %_22.i278.i, align 4
  store <4 x float> %.lcssa1825318358.lcssa23208, ptr %1703, align 16
  store i32 %storemerge.i1291.lcssa1828018394.lcssa23229, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1831018430.lcssa23251, ptr %1713, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1541, i32 noundef %_5.i1542, i32 noundef range(i32 0, 536870912) %_58.1.i316.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !13037, !noalias !13038
  unreachable, !dbg !13037

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1549: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i299.i
  %2032 = bitcast <4 x i32> %2030 to <4 x float>, !dbg !13042
  %2033 = fsub <4 x float> splat (float 1.000000e+00), %2032, !dbg !13046
  %_15.i1547 = getelementptr inbounds nuw float, ptr %_58.0.i315.i, i32 %base.i1541, !dbg !13047
  %lanes.i4950.sroa.0.0.copyload = load <4 x i32>, ptr %_15.i1547, align 4, !dbg !13049, !alias.scope !13054, !noalias !13058
  store <4 x i32> %lanes.i5392.sroa.0.0.copyload, ptr %_15.i1547, align 4, !dbg !13062, !alias.scope !13068, !noalias !13072
  %2034 = bitcast <4 x i32> %lanes.i4950.sroa.0.0.copyload to <4 x float>, !dbg !13078
  %2035 = fmul <4 x float> %2033, %2034, !dbg !13082
  %2036 = bitcast <4 x i32> %lanes.i4950.sroa.0.0.copyload to <16 x i8>, !dbg !13083
  %2037 = bitcast <4 x float> %2035 to <16 x i8>, !dbg !13087
  %_4.i6801 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %2036, <16 x i8> %2037, <16 x i8> %1708), !dbg !13088
  store <16 x i8> %_4.i6801, ptr %data.i.i.i.i.i.i6761, align 4, !dbg !13089, !alias.scope !13094, !noalias !13098
  %_220.i = add i32 %iter.i.sroa.36.018200, %right_end.sroa.0.0.i1933, !dbg !13102
  %_222.i = add i32 %iter.i.sroa.36.018200, %right_expiring.sroa.0.0.i1941, !dbg !13104
  %_8.not.i12.i.i = icmp ugt i32 %_7.i10.i267.i, %_54.1.i.i
  br i1 %_8.not.i12.i.i, label %bb4.i15.i.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i, !dbg !13105, !prof !4694

bb4.i15.i.i:                                      ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1549
  store <4 x i32> %history.i41.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.7.0.lcssa, ptr %history.i41.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.10.0.lcssa, ptr %history.i41.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.13.0.lcssa, ptr %history.i41.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.16.0.lcssa, ptr %history.i41.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.19.0.lcssa, ptr %history.i41.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.22.0.lcssa, ptr %history.i41.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.26.0.lcssa, ptr %history.i41.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.29.0.lcssa, ptr %history.i41.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.32.0.lcssa, ptr %history.i41.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.35.0.lcssa, ptr %history.i41.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.38.0.lcssa, ptr %history.i41.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store i32 %storemerge.i1325.lcssa1822318322.lcssa23186, ptr %_22.i278.i, align 4
  store <4 x float> %.lcssa1825318358.lcssa23208, ptr %1703, align 16
  store i32 %storemerge.i1291.lcssa1828018394.lcssa23229, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1831018430.lcssa23251, ptr %1713, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i9.i266.i, i32 noundef %_7.i10.i267.i, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #33, !dbg !13111, !noalias !13112
  unreachable, !dbg !13111

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1549
  %2038 = bitcast <16 x i8> %_4.i6787 to <4 x float>, !dbg !13126
  %2039 = fdiv <4 x float> %_8.i.i15509.pre, %2038, !dbg !13131
  %2040 = bitcast <4 x float> %2039 to <16 x i8>, !dbg !13135
  %2041 = fcmp olt <4 x float> %_8.i.i15509.pre, %2038, !dbg !13139
  %2042 = sext <4 x i1> %2041 to <4 x i32>, !dbg !13139
  %2043 = bitcast <4 x i32> %2042 to <16 x i8>, !dbg !13140
  %_4.i6803 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %2040, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %2043), !dbg !13141
  %_17.i14.i.i = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %base.i9.i266.i, !dbg !13142
  store <16 x i8> %_4.i6803, ptr %_17.i14.i.i, align 4, !dbg !13144, !alias.scope !13149, !noalias !13153
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13157), !dbg !13160
  %base.i1406 = shl i32 %_220.i, 2, !dbg !13161
  %2044 = or disjoint i32 %base.i1406, 3, !dbg !13164
  %or.cond.i1410.not = icmp ult i32 %2044, %_54.1.i.i, !dbg !13164
  br i1 %or.cond.i1410.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1414, label %bb4.i1413, !dbg !13164, !prof !10587

bb4.i1413:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i
  store <4 x i32> %history.i41.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.7.0.lcssa, ptr %history.i41.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.10.0.lcssa, ptr %history.i41.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.13.0.lcssa, ptr %history.i41.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.16.0.lcssa, ptr %history.i41.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.19.0.lcssa, ptr %history.i41.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.22.0.lcssa, ptr %history.i41.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.26.0.lcssa, ptr %history.i41.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.29.0.lcssa, ptr %history.i41.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.32.0.lcssa, ptr %history.i41.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.35.0.lcssa, ptr %history.i41.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.38.0.lcssa, ptr %history.i41.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store i32 %storemerge.i1325.lcssa1822318322.lcssa23186, ptr %_22.i278.i, align 4
  store <4 x float> %.lcssa1825318358.lcssa23208, ptr %1703, align 16
  store i32 %storemerge.i1291.lcssa1828018394.lcssa23229, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1831018430.lcssa23251, ptr %1713, align 16
  %_5.i1407 = add i32 %base.i1406, 4, !dbg !13168
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1406, i32 noundef %_5.i1407, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !13169, !noalias !13170
  unreachable, !dbg !13169

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1414: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i
  %_15.i1412 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %base.i1406, !dbg !13176
  %lanes.i5055.sroa.0.0.copyload = load <4 x i32>, ptr %_15.i1412, align 4, !dbg !13178
  %2045 = icmp eq i32 %storemerge.i129118258, 0, !dbg !13183
  %_12.i128315521 = load <4 x float>, ptr %uniform_right.i, align 16, !dbg !13183
  %2046 = bitcast <4 x i32> %lanes.i5055.sroa.0.0.copyload to <4 x float>, !dbg !13183
  %2047 = fcmp olt <4 x float> %_12.i128315521, %2046, !dbg !13183
  %2048 = select <4 x i1> %2047, <4 x float> %_12.i128315521, <4 x float> %2046, !dbg !13183
  %2049 = bitcast <4 x float> %2048 to <4 x i32>, !dbg !13183
  %.sroa.07728.0 = select i1 %2045, <4 x i32> %lanes.i5055.sroa.0.0.copyload, <4 x i32> %2049, !dbg !13183
  store <4 x i32> %.sroa.07728.0, ptr %uniform_right.i, align 16, !dbg !13184, !alias.scope !13157, !noalias !13185
  %_15.i1286 = add i32 %storemerge.i129118258, 1, !dbg !13187
  %complete.i1287 = icmp eq i32 %_15.i1286, %_18.i250.i, !dbg !13187
  br i1 %complete.i1287, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1396, label %bb7.i1288, !dbg !13188

bb7.i1288:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1414
  %base.i1397 = shl i32 %_216.i, 2, !dbg !13189
  %2050 = or disjoint i32 %base.i1397, 3, !dbg !13191
  %or.cond.i1401.not = icmp ult i32 %2050, %_54.1.i.i, !dbg !13191
  br i1 %or.cond.i1401.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1405, label %bb4.i1404, !dbg !13191, !prof !10587

bb4.i1404:                                        ; preds = %bb7.i1288
  store <4 x i32> %history.i41.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.7.0.lcssa, ptr %history.i41.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.10.0.lcssa, ptr %history.i41.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.13.0.lcssa, ptr %history.i41.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.16.0.lcssa, ptr %history.i41.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.19.0.lcssa, ptr %history.i41.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.22.0.lcssa, ptr %history.i41.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.26.0.lcssa, ptr %history.i41.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.29.0.lcssa, ptr %history.i41.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.32.0.lcssa, ptr %history.i41.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.35.0.lcssa, ptr %history.i41.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.38.0.lcssa, ptr %history.i41.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store i32 %storemerge.i1325.lcssa1822318322.lcssa23186, ptr %_22.i278.i, align 4
  store <4 x float> %.lcssa1825318358.lcssa23208, ptr %1703, align 16
  store i32 %storemerge.i1291.lcssa1828018394.lcssa23229, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1831018430.lcssa23251, ptr %1713, align 16
  %_5.i1398 = add i32 %base.i1397, 4, !dbg !13195
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1397, i32 noundef %_5.i1398, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !13196, !noalias !13197
  unreachable, !dbg !13196

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1405: ; preds = %bb7.i1288
  %_15.i1403 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %base.i1397, !dbg !13201
  %lanes.i5062.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1403, align 4, !dbg !13203, !alias.scope !13208, !noalias !13212
  %2051 = bitcast <4 x i32> %.sroa.07728.0 to <4 x float>, !dbg !13216
  %2052 = fcmp olt <4 x float> %lanes.i5062.sroa.0.0.copyload, %2051, !dbg !13220
  %2053 = select <4 x i1> %2052, <4 x float> %lanes.i5062.sroa.0.0.copyload, <4 x float> %2051, !dbg !13221
  %2054 = bitcast <4 x float> %2053 to <4 x i32>, !dbg !13222
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1312, !dbg !13224

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1396: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1414
  %2055 = bitcast <4 x i32> %lanes.i5055.sroa.0.0.copyload to <4 x float>, !dbg !13188
  br i1 %_29.i129618195.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1312, label %bb19.i1297, !dbg !13225

bb19.i1297:                                       ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1396, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1387
  %end.sroa.0.0.i129518197 = phi i32 [ %2061, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1387 ], [ %_220.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1396 ]
  %iter.sroa.0.0.i129418196 = phi i32 [ %_30.i1298, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1387 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1396 ]
  %2056 = phi <4 x float> [ %2059, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1387 ], [ %2055, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1396 ]
  %base.i1379 = shl i32 %end.sroa.0.0.i129518197, 2, !dbg !13228
  %2057 = or disjoint i32 %base.i1379, 3, !dbg !13230
  %or.cond.i1383.not = icmp ult i32 %2057, %_54.1.i.i, !dbg !13230
  br i1 %or.cond.i1383.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1387, label %bb4.i1386, !dbg !13230, !prof !10587

bb4.i1386:                                        ; preds = %bb19.i1297
  store <4 x i32> %history.i41.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.7.0.lcssa, ptr %history.i41.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.10.0.lcssa, ptr %history.i41.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.13.0.lcssa, ptr %history.i41.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.16.0.lcssa, ptr %history.i41.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.19.0.lcssa, ptr %history.i41.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.22.0.lcssa, ptr %history.i41.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.26.0.lcssa, ptr %history.i41.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.29.0.lcssa, ptr %history.i41.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.32.0.lcssa, ptr %history.i41.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.35.0.lcssa, ptr %history.i41.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.38.0.lcssa, ptr %history.i41.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store i32 %storemerge.i1325.lcssa1822318322.lcssa23186, ptr %_22.i278.i, align 4
  store <4 x float> %.lcssa1825318358.lcssa23208, ptr %1703, align 16
  store i32 %storemerge.i1291.lcssa1828018394.lcssa23229, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1831018430.lcssa23251, ptr %1713, align 16
  %_5.i1380 = add i32 %base.i1379, 4, !dbg !13234
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1379, i32 noundef %_5.i1380, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !13235, !noalias !13236
  unreachable, !dbg !13235

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1387: ; preds = %bb19.i1297
  %_30.i1298 = add nuw i32 %iter.sroa.0.0.i129418196, 1, !dbg !13240
  %_15.i1385 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %base.i1379, !dbg !13243
  %lanes.i5076.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1385, align 4, !dbg !13245, !alias.scope !13250, !noalias !13254
  %2058 = fcmp olt <4 x float> %2056, %lanes.i5076.sroa.0.0.copyload, !dbg !13258
  %2059 = select <4 x i1> %2058, <4 x float> %2056, <4 x float> %lanes.i5076.sroa.0.0.copyload, !dbg !13262
  store <4 x float> %2059, ptr %_15.i1385, align 4, !dbg !13263, !alias.scope !13269, !noalias !13273
  %2060 = icmp eq i32 %end.sroa.0.0.i129518197, 0, !dbg !13279
  %spec.store.select.i1309 = select i1 %2060, i32 %ring.i, i32 %end.sroa.0.0.i129518197, !dbg !13279
  %2061 = add i32 %spec.store.select.i1309, -1, !dbg !13280
  %exitcond20565.not = icmp eq i32 %_30.i1298, %_18.i250.i, !dbg !13281
  br i1 %exitcond20565.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1312, label %bb19.i1297, !dbg !13225

_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1312: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1387, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1396, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1405
  %.sroa.07728.1 = phi <4 x i32> [ %2054, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1405 ], [ %.sroa.07728.0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1396 ], [ %.sroa.07728.0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1387 ], !dbg !13283
  %storemerge.i1291 = phi i32 [ %_15.i1286, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1405 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1396 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1387 ], !dbg !13284
  %2062 = bitcast <4 x i32> %.sroa.07728.1 to <4 x float>, !dbg !13285
  %2063 = fmul <4 x float> %2062, splat (float 1.638400e+04), !dbg !13289
  %2064 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %2063), !dbg !13290
  %2065 = fmul <4 x float> %2064, splat (float 0x3F10000000000000), !dbg !13294
  %base.i1532 = shl i32 %_222.i, 2, !dbg !13298
  %2066 = or disjoint i32 %base.i1532, 3, !dbg !13300
  %or.cond.i1536.not = icmp ult i32 %2066, %_56.1.i.i, !dbg !13300
  br i1 %or.cond.i1536.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1540, label %bb4.i1539, !dbg !13300, !prof !10587

bb4.i1539:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1312
  store <4 x i32> %history.i41.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.7.0.lcssa, ptr %history.i41.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.10.0.lcssa, ptr %history.i41.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.13.0.lcssa, ptr %history.i41.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.16.0.lcssa, ptr %history.i41.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.19.0.lcssa, ptr %history.i41.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.22.0.lcssa, ptr %history.i41.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.26.0.lcssa, ptr %history.i41.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.29.0.lcssa, ptr %history.i41.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.32.0.lcssa, ptr %history.i41.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.35.0.lcssa, ptr %history.i41.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.38.0.lcssa, ptr %history.i41.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store i32 %storemerge.i1325.lcssa1822318322.lcssa23186, ptr %_22.i278.i, align 4
  store <4 x float> %.lcssa1825318358.lcssa23208, ptr %1703, align 16
  store i32 %storemerge.i1291.lcssa1828018394.lcssa23229, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1831018430.lcssa23251, ptr %1713, align 16
  %_5.i1533 = add i32 %base.i1532, 4, !dbg !13304
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1532, i32 noundef %_5.i1533, i32 noundef range(i32 0, 536870912) %_56.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !13305, !noalias !13306
  unreachable, !dbg !13305

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1540: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1312
  %_15.i1538 = getelementptr inbounds nuw float, ptr %_56.0.i.i, i32 %base.i1532, !dbg !13310
  %lanes.i4957.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1538, align 4, !dbg !13312, !alias.scope !13317, !noalias !13321
  %2067 = fadd <4 x float> %2065, %1979, !dbg !13325
  %2068 = fsub <4 x float> %2067, %lanes.i4957.sroa.0.0.copyload, !dbg !13329
  %_8.not.i4.i.i = icmp ugt i32 %_7.i10.i267.i, %_56.1.i.i
  br i1 %_8.not.i4.i.i, label %bb4.i7.i.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i, !dbg !13333, !prof !4694

bb4.i7.i.i:                                       ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1540
  store <4 x i32> %history.i41.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.7.0.lcssa, ptr %history.i41.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.10.0.lcssa, ptr %history.i41.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.13.0.lcssa, ptr %history.i41.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.16.0.lcssa, ptr %history.i41.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.19.0.lcssa, ptr %history.i41.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.22.0.lcssa, ptr %history.i41.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.26.0.lcssa, ptr %history.i41.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.29.0.lcssa, ptr %history.i41.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.32.0.lcssa, ptr %history.i41.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.35.0.lcssa, ptr %history.i41.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.38.0.lcssa, ptr %history.i41.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store i32 %storemerge.i1325.lcssa1822318322.lcssa23186, ptr %_22.i278.i, align 4
  store <4 x float> %.lcssa1825318358.lcssa23208, ptr %1703, align 16
  store i32 %storemerge.i1291.lcssa1828018394.lcssa23229, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1831018430.lcssa23251, ptr %1713, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i9.i266.i, i32 noundef %_7.i10.i267.i, i32 noundef range(i32 0, 536870912) %_56.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #33, !dbg !13338, !noalias !13339
  unreachable, !dbg !13338

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1540
  %_17.i6.i.i = getelementptr inbounds nuw float, ptr %_56.0.i.i, i32 %base.i9.i266.i, !dbg !13343
  store <4 x float> %2065, ptr %_17.i6.i.i, align 4, !dbg !13345, !alias.scope !13350, !noalias !13354
  %_41.i.i15528 = load <4 x float>, ptr %1715, align 16, !dbg !13358
  %2069 = fdiv <4 x float> %2068, %_37.i.i15527, !dbg !13359
  %2070 = fsub <4 x float> splat (float 1.000000e+00), %2069, !dbg !13363
  %2071 = fsub <4 x float> %2070, %_41.i.i15528, !dbg !13367
  %2072 = fmul <4 x float> %_9.i.i15510.pre, %2071, !dbg !13371
  %2073 = fadd <4 x float> %_41.i.i15528, %2072, !dbg !13375
  %2074 = fcmp olt <4 x float> %2073, %2070, !dbg !13378
  %2075 = select <4 x i1> %2074, <4 x float> %2070, <4 x float> %2073, !dbg !13382
  %2076 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %2075), !dbg !13383
  %2077 = fcmp uge <4 x float> %2076, splat (float 0x3BC79CA100000000), !dbg !13388
  %2078 = bitcast <4 x float> %2075 to <4 x i32>, !dbg !13393
  %2079 = select <4 x i1> %2077, <4 x i32> %2078, <4 x i32> zeroinitializer, !dbg !13393
  store <4 x i32> %2079, ptr %1715, align 16, !dbg !13396
  %_6.not.i1526 = icmp ugt i32 %_5.i1542, %_58.1.i.i
  br i1 %_6.not.i1526, label %bb4.i1530, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1531, !dbg !13397, !prof !4694

bb4.i1530:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i
  store <4 x i32> %history.i41.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.7.0.lcssa, ptr %history.i41.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.10.0.lcssa, ptr %history.i41.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.13.0.lcssa, ptr %history.i41.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.16.0.lcssa, ptr %history.i41.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.19.0.lcssa, ptr %history.i41.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.22.0.lcssa, ptr %history.i41.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.26.0.lcssa, ptr %history.i41.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.29.0.lcssa, ptr %history.i41.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.32.0.lcssa, ptr %history.i41.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.35.0.lcssa, ptr %history.i41.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.38.0.lcssa, ptr %history.i41.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store i32 %storemerge.i1325.lcssa1822318322.lcssa23186, ptr %_22.i278.i, align 4
  store <4 x float> %.lcssa1825318358.lcssa23208, ptr %1703, align 16
  store i32 %storemerge.i1291.lcssa1828018394.lcssa23229, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1831018430.lcssa23251, ptr %1713, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1541, i32 noundef %_5.i1542, i32 noundef range(i32 0, 536870912) %_58.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !13402, !noalias !13403
  unreachable, !dbg !13402

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1531: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i
  %2080 = bitcast <4 x i32> %2079 to <4 x float>, !dbg !13407
  %2081 = fsub <4 x float> splat (float 1.000000e+00), %2080, !dbg !13411
  %_15.i1529 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i32 %base.i1541, !dbg !13412
  %lanes.i4964.sroa.0.0.copyload = load <4 x i32>, ptr %_15.i1529, align 4, !dbg !13414, !alias.scope !13419, !noalias !13423
  store <4 x i32> %lanes.i5383.sroa.0.0.copyload, ptr %_15.i1529, align 4, !dbg !13427, !alias.scope !13433, !noalias !13437
  %2082 = bitcast <4 x i32> %lanes.i4964.sroa.0.0.copyload to <4 x float>, !dbg !13443
  %2083 = fmul <4 x float> %2081, %2082, !dbg !13447
  %2084 = bitcast <4 x i32> %lanes.i4964.sroa.0.0.copyload to <16 x i8>, !dbg !13448
  %2085 = bitcast <4 x float> %2083 to <16 x i8>, !dbg !13452
  %_4.i6814 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %2084, <16 x i8> %2085, <16 x i8> %1708), !dbg !13453
  store <16 x i8> %_4.i6814, ptr %data.i5.i.i.i.i.i6765, align 4, !dbg !13454, !alias.scope !13459, !noalias !13463
  %exitcond20577.not = icmp eq i32 %1981, %1978, !dbg !12622
  br i1 %exitcond20577.not, label %bb67.i, label %bb68.i, !dbg !12622

bb67.i:                                           ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1531, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit6747
  %.lcssa1831018430 = phi <4 x float> [ %.lcssa1831018431, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit6747 ], [ %2068, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1531 ]
  %storemerge.i1291.lcssa1828018394 = phi i32 [ %storemerge.i1291.lcssa1828018395, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit6747 ], [ %storemerge.i1291, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1531 ]
  %.lcssa1825318358 = phi <4 x float> [ %.lcssa1825318359, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit6747 ], [ %2019, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1531 ]
  %storemerge.i1325.lcssa1822318322 = phi i32 [ %storemerge.i1325.lcssa1822318323, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit6747 ], [ %storemerge.i1325, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1531 ]
  %_139.i = add i32 %run.sroa.0.0.i1954, %ring_cursor.sroa.0.1.i18316, !dbg !13467
  %_212.not.i = icmp ult i32 %_139.i, %ring.i, !dbg !13468
  %2086 = select i1 %_212.not.i, i32 0, i32 %ring.i, !dbg !13468
  %ring_cursor.sroa.0.2.i = sub nuw i32 %_139.i, %2086, !dbg !13468
  %_141.i = add i32 %run.sroa.0.0.i1954, %main_cursor.sroa.0.1.i18317, !dbg !13471
  %_223.not.i = icmp ult i32 %_141.i, %main.i, !dbg !13472
  %2087 = select i1 %_223.not.i, i32 0, i32 %main.i, !dbg !13472
  %main_cursor.sroa.0.2.i = sub nuw i32 %_141.i, %2087, !dbg !13472
  %_59.i = icmp ult i32 %_83.i, %spec.store.select.i, !dbg !12520
  br i1 %_59.i, label %bb20.i, label %bb15.i.loopexit, !dbg !12520

_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit: ; preds = %bb15.i.loopexit
  store <4 x i32> %history.i41.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.7.0.lcssa, ptr %history.i41.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.10.0.lcssa, ptr %history.i41.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.13.0.lcssa, ptr %history.i41.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.16.0.lcssa, ptr %history.i41.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.19.0.lcssa, ptr %history.i41.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.22.0.lcssa, ptr %history.i41.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.26.0.lcssa, ptr %history.i41.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.29.0.lcssa, ptr %history.i41.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.32.0.lcssa, ptr %history.i41.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.35.0.lcssa, ptr %history.i41.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i41.i.sroa.38.0.lcssa, ptr %history.i41.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11597
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11598
  store i32 %storemerge.i1325.lcssa1822318322.lcssa23185, ptr %_22.i278.i, align 4
  store <4 x float> %.lcssa1825318358.lcssa23207, ptr %1703, align 16
  store i32 %storemerge.i1291.lcssa1828018394.lcssa23228, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1831018430.lcssa23250, ptr %1713, align 16
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !13474

_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit, %bb6.i
  %ring_cursor.sroa.0.0.i.lcssa = phi i32 [ %_37.i, %bb6.i ], [ %ring_cursor.sroa.0.1.i.lcssa, %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit ], !dbg !11531
  %main_cursor.sroa.0.0.i.lcssa = phi i32 [ %_36.i, %bb6.i ], [ %main_cursor.sroa.0.1.i.lcssa, %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit ], !dbg !11528
  %left_prefix.i = load <4 x i32>, ptr %uniform_left.i, align 16, !dbg !13474, !noalias !11535
  %2088 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 16, !dbg !13475
  %left_phase.i = load i32, ptr %2088, align 16, !dbg !13475, !noalias !11535, !noundef !10
  %right_prefix.i = load <4 x i32>, ptr %uniform_right.i, align 16, !dbg !13476, !noalias !11535
  %2089 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 16, !dbg !13477
  %right_phase.i = load i32, ptr %2089, align 16, !dbg !13477, !noalias !11535, !noundef !10
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_right.i), !dbg !13478, !noalias !11535
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i), !dbg !13479, !noalias !11535
  %2090 = getelementptr inbounds nuw i8, ptr %self, i32 968, !dbg !13480
  %_236.1.i = load i32, ptr %2090, align 4, !dbg !13480, !alias.scope !11502, !noalias !13482, !noundef !10
  %_8.i6026 = icmp samesign ugt i32 %_236.1.i, 3, !dbg !13483
  br i1 %_8.i6026, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6029, label %bb2.i6027, !dbg !13483, !prof !1153

bb2.i6027:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_236.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !13488, !noalias !13489
  unreachable, !dbg !13488

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6029: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
  %2091 = getelementptr inbounds nuw i8, ptr %self, i32 964, !dbg !13480
  %_236.0.i = load ptr, ptr %2091, align 4, !dbg !13480, !alias.scope !11502, !noalias !13482, !nonnull !10, !noundef !10
  store <4 x i32> %left_prefix.i, ptr %_236.0.i, align 4, !dbg !13493, !alias.scope !13497, !noalias !13501
  %_237.0.i = load ptr, ptr %68, align 4, !dbg !13503, !alias.scope !11502, !noalias !13482, !nonnull !10, !noundef !10
  %_237.1.i = load i32, ptr %69, align 4, !dbg !13503, !alias.scope !11502, !noalias !13482, !noundef !10
  %2092 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i), !dbg !13504
  br i1 %2092, label %bb2.i6825, label %bb6.i6817, !dbg !13504

bb6.i6817:                                        ; preds = %bb2.i6825, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6029
  %end_or_len.idx.i6818 = shl nuw nsw i32 %_237.1.i, 2, !dbg !13508
  %end_or_len.i6819 = getelementptr inbounds nuw i8, ptr %_237.0.i, i32 %end_or_len.idx.i6818, !dbg !13508
  %_293.i6820 = icmp eq i32 %_237.1.i, 0, !dbg !13512
  br i1 %_293.i6820, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit6831, label %bb10.i6821, !dbg !13515

bb2.i6825:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6029
  %bytes1.sroa.0.0.zext.i6826 = and i32 %left_phase.i, 255, !dbg !13516
  %bytes1.sroa.0.0.isplat.i6827 = mul nuw i32 %bytes1.sroa.0.0.zext.i6826, 16843009, !dbg !13516
  %_5.i6828 = icmp eq i32 %left_phase.i, %bytes1.sroa.0.0.isplat.i6827, !dbg !13517
  br i1 %_5.i6828, label %bb3.i6829, label %bb6.i6817, !dbg !13517

bb3.i6829:                                        ; preds = %bb2.i6825
  %bytes.sroa.0.0.extract.trunc.i6830 = trunc i32 %left_phase.i to i8, !dbg !13518
  %2093 = shl nuw nsw i32 %_237.1.i, 2, !dbg !13520
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %_237.0.i, i8 %bytes.sroa.0.0.extract.trunc.i6830, i32 %2093, i1 false), !dbg !13520, !alias.scope !13521, !noalias !11506
  br label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit6831, !dbg !13524

bb10.i6821:                                       ; preds = %bb6.i6817, %bb10.i6821
  %iter.sroa.0.04.i6822 = phi ptr [ %_38.i6823, %bb10.i6821 ], [ %_237.0.i, %bb6.i6817 ]
  %_38.i6823 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i6822, i32 4, !dbg !13525
  store i32 %left_phase.i, ptr %iter.sroa.0.04.i6822, align 4, !dbg !13527, !alias.scope !13521, !noalias !11506
  %_29.i6824 = icmp eq ptr %_38.i6823, %end_or_len.i6819, !dbg !13512
  br i1 %_29.i6824, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit6831, label %bb10.i6821, !dbg !13515

_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit6831: ; preds = %bb10.i6821, %bb6.i6817, %bb3.i6829
  %2094 = getelementptr inbounds nuw i8, ptr %self, i32 1068, !dbg !13528
  %_238.1.i = load i32, ptr %2094, align 4, !dbg !13528, !alias.scope !11504, !noalias !13529, !noundef !10
  %_8.i6021 = icmp samesign ugt i32 %_238.1.i, 3, !dbg !13530
  br i1 %_8.i6021, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6024, label %bb2.i6022, !dbg !13530, !prof !1153

bb2.i6022:                                        ; preds = %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit6831
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_238.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !13535, !noalias !13536
  unreachable, !dbg !13535

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6024: ; preds = %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit6831
  %2095 = getelementptr inbounds nuw i8, ptr %self, i32 1064, !dbg !13528
  %_238.0.i = load ptr, ptr %2095, align 4, !dbg !13528, !alias.scope !11504, !noalias !13529, !nonnull !10, !noundef !10
  store <4 x i32> %right_prefix.i, ptr %_238.0.i, align 4, !dbg !13540, !alias.scope !13544, !noalias !13548
  %_239.0.i = load ptr, ptr %77, align 4, !dbg !13550, !alias.scope !11504, !noalias !13529, !nonnull !10, !noundef !10
  %_239.1.i = load i32, ptr %78, align 4, !dbg !13550, !alias.scope !11504, !noalias !13529, !noundef !10
  %2096 = tail call i1 @llvm.is.constant.i32(i32 %right_phase.i), !dbg !13551
  br i1 %2096, label %bb2.i6841, label %bb6.i6833, !dbg !13551

bb6.i6833:                                        ; preds = %bb2.i6841, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6024
  %end_or_len.idx.i6834 = shl nuw nsw i32 %_239.1.i, 2, !dbg !13554
  %end_or_len.i6835 = getelementptr inbounds nuw i8, ptr %_239.0.i, i32 %end_or_len.idx.i6834, !dbg !13554
  %_293.i6836 = icmp eq i32 %_239.1.i, 0, !dbg !13558
  br i1 %_293.i6836, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit6847, label %bb10.i6837, !dbg !13561

bb2.i6841:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6024
  %bytes1.sroa.0.0.zext.i6842 = and i32 %right_phase.i, 255, !dbg !13562
  %bytes1.sroa.0.0.isplat.i6843 = mul nuw i32 %bytes1.sroa.0.0.zext.i6842, 16843009, !dbg !13562
  %_5.i6844 = icmp eq i32 %right_phase.i, %bytes1.sroa.0.0.isplat.i6843, !dbg !13563
  br i1 %_5.i6844, label %bb3.i6845, label %bb6.i6833, !dbg !13563

bb3.i6845:                                        ; preds = %bb2.i6841
  %bytes.sroa.0.0.extract.trunc.i6846 = trunc i32 %right_phase.i to i8, !dbg !13564
  %2097 = shl nuw nsw i32 %_239.1.i, 2, !dbg !13566
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %_239.0.i, i8 %bytes.sroa.0.0.extract.trunc.i6846, i32 %2097, i1 false), !dbg !13566, !alias.scope !13567, !noalias !11506
  br label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit6847, !dbg !13570

bb10.i6837:                                       ; preds = %bb6.i6833, %bb10.i6837
  %iter.sroa.0.04.i6838 = phi ptr [ %_38.i6839, %bb10.i6837 ], [ %_239.0.i, %bb6.i6833 ]
  %_38.i6839 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i6838, i32 4, !dbg !13571
  store i32 %right_phase.i, ptr %iter.sroa.0.04.i6838, align 4, !dbg !13573, !alias.scope !13567, !noalias !11506
  %_29.i6840 = icmp eq ptr %_38.i6839, %end_or_len.i6835, !dbg !13558
  br i1 %_29.i6840, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit6847, label %bb10.i6837, !dbg !13561

_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit6847: ; preds = %bb10.i6837, %bb6.i6833, %bb3.i6845
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_(ptr noalias noundef readonly align 16 captures(none) dereferenceable(368) %hot_left.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33) #32, !dbg !13574
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_(ptr noalias noundef readonly align 16 captures(none) dereferenceable(368) %hot_right.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34) #32, !dbg !13575
  store i32 %main_cursor.sroa.0.0.i.lcssa, ptr %_35, align 4, !dbg !13576, !alias.scope !11506, !noalias !11530
  store i32 %ring_cursor.sroa.0.0.i.lcssa, ptr %86, align 4, !dbg !13577, !alias.scope !11506, !noalias !11530
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i), !dbg !13578, !noalias !11535
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i), !dbg !13579, !noalias !11535
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !11499

_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit6672, %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit6847
  br i1 %quiet.sroa.0.0.off015025, label %bb28, label %bb40, !dbg !13580

bb22:                                             ; preds = %bb20
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13581), !dbg !13584
  %2098 = getelementptr inbounds nuw i8, ptr %self, i32 980, !dbg !13585
  %_40.0.i = load ptr, ptr %2098, align 4, !dbg !13585, !alias.scope !13581, !nonnull !10, !noundef !10
  %2099 = getelementptr inbounds nuw i8, ptr %self, i32 984, !dbg !13585
  %_40.1.i = load i32, ptr %2099, align 4, !dbg !13585, !alias.scope !13581, !noundef !10
  %2100 = getelementptr inbounds nuw i8, ptr %self, i32 1012, !dbg !13587
  %_41.0.i = load ptr, ptr %2100, align 4, !dbg !13587, !alias.scope !13581, !nonnull !10, !noundef !10
  %2101 = getelementptr inbounds nuw i8, ptr %self, i32 1016, !dbg !13587
  %_41.1.i = load i32, ptr %2101, align 4, !dbg !13587, !alias.scope !13581, !noundef !10
  %spec.store.select.i.i = tail call i32 @llvm.umin.i32(i32 %_41.1.i, i32 %_40.1.i), !dbg !13588
  %_2.i6.not.i = icmp eq i32 %spec.store.select.i.i, 0, !dbg !13594
  br i1 %_2.i6.not.i, label %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb3.i6848, !dbg !13594

bb3.i6848:                                        ; preds = %bb22, %bb5.i6850
  %iter.sroa.8.07.i = phi i32 [ %2102, %bb5.i6850 ], [ 0, %bb22 ]
  %_3.i1.i.i = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i, i32 %iter.sroa.8.07.i, !dbg !13597
  %_14.i6849 = load i32, ptr %_3.i1.i.i, align 4, !dbg !13600, !noalias !13581, !noundef !10
  %_20.i = icmp eq i32 %_14.i6849, 0, !dbg !13601
  br i1 %_20.i, label %panic.i6857, label %bb5.i6850, !dbg !13601

bb5.i6850:                                        ; preds = %bb3.i6848
  %_3.i.i.i6851 = getelementptr inbounds nuw i32, ptr %_40.0.i, i32 %iter.sroa.8.07.i, !dbg !13602
  %2102 = add nuw i32 %iter.sroa.8.07.i, 1, !dbg !13605
  %_18.i6852 = load i32, ptr %_3.i.i.i6851, align 4, !dbg !13606, !noalias !13581, !noundef !10
  %_19.i6853 = urem i32 %frames, %_14.i6849, !dbg !13601
  %_16.i6854 = add i32 %_19.i6853, %_18.i6852, !dbg !13607
  %_15.i6855 = urem i32 %_16.i6854, %_14.i6849, !dbg !13608
  store i32 %_15.i6855, ptr %_3.i.i.i6851, align 4, !dbg !13609, !noalias !13581
  %exitcond.not.i = icmp eq i32 %2102, %spec.store.select.i.i, !dbg !13594
  br i1 %exitcond.not.i, label %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb3.i6848, !dbg !13594

panic.i6857:                                      ; preds = %bb3.i6848
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_f5b0427df9b659e554a697ca46ce8b5a) #33, !dbg !13601, !noalias !13581
  unreachable, !dbg !13601

_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit: ; preds = %bb5.i6850, %bb22
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13610), !dbg !13613
  %2103 = getelementptr inbounds nuw i8, ptr %self, i32 1080, !dbg !13614
  %_40.0.i6858 = load ptr, ptr %2103, align 4, !dbg !13614, !alias.scope !13610, !nonnull !10, !noundef !10
  %2104 = getelementptr inbounds nuw i8, ptr %self, i32 1084, !dbg !13614
  %_40.1.i6859 = load i32, ptr %2104, align 4, !dbg !13614, !alias.scope !13610, !noundef !10
  %2105 = getelementptr inbounds nuw i8, ptr %self, i32 1112, !dbg !13616
  %_41.0.i6860 = load ptr, ptr %2105, align 4, !dbg !13616, !alias.scope !13610, !nonnull !10, !noundef !10
  %2106 = getelementptr inbounds nuw i8, ptr %self, i32 1116, !dbg !13616
  %_41.1.i6861 = load i32, ptr %2106, align 4, !dbg !13616, !alias.scope !13610, !noundef !10
  %spec.store.select.i.i6862 = tail call i32 @llvm.umin.i32(i32 %_41.1.i6861, i32 %_40.1.i6859), !dbg !13617
  %_2.i6.not.i6863 = icmp eq i32 %spec.store.select.i.i6862, 0, !dbg !13623
  br i1 %_2.i6.not.i6863, label %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit6878, label %bb3.i6864, !dbg !13623

bb3.i6864:                                        ; preds = %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, %bb5.i6869
  %iter.sroa.8.07.i6865 = phi i32 [ %2107, %bb5.i6869 ], [ 0, %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit ]
  %_3.i1.i.i6866 = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i6860, i32 %iter.sroa.8.07.i6865, !dbg !13626
  %_14.i6867 = load i32, ptr %_3.i1.i.i6866, align 4, !dbg !13629, !noalias !13610, !noundef !10
  %_20.i6868 = icmp eq i32 %_14.i6867, 0, !dbg !13630
  br i1 %_20.i6868, label %panic.i6877, label %bb5.i6869, !dbg !13630

bb5.i6869:                                        ; preds = %bb3.i6864
  %_3.i.i.i6870 = getelementptr inbounds nuw i32, ptr %_40.0.i6858, i32 %iter.sroa.8.07.i6865, !dbg !13631
  %2107 = add nuw i32 %iter.sroa.8.07.i6865, 1, !dbg !13634
  %_18.i6871 = load i32, ptr %_3.i.i.i6870, align 4, !dbg !13635, !noalias !13610, !noundef !10
  %_19.i6872 = urem i32 %frames, %_14.i6867, !dbg !13630
  %_16.i6873 = add i32 %_19.i6872, %_18.i6871, !dbg !13636
  %_15.i6874 = urem i32 %_16.i6873, %_14.i6867, !dbg !13637
  store i32 %_15.i6874, ptr %_3.i.i.i6870, align 4, !dbg !13638, !noalias !13610
  %exitcond.not.i6875 = icmp eq i32 %2107, %spec.store.select.i.i6862, !dbg !13623
  br i1 %exitcond.not.i6875, label %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit6878, label %bb3.i6864, !dbg !13623

panic.i6877:                                      ; preds = %bb3.i6864
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_f5b0427df9b659e554a697ca46ce8b5a) #33, !dbg !13630, !noalias !13610
  unreachable, !dbg !13630

_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit6878: ; preds = %bb5.i6869, %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit
  %2108 = getelementptr inbounds nuw i8, ptr %self, i32 916, !dbg !13639
  %_29.val = load i32, ptr %2108, align 4, !dbg !13639
  %2109 = getelementptr inbounds nuw i8, ptr %self, i32 920, !dbg !13639
  %_29.val6280 = load i32, ptr %2109, align 4, !dbg !13639, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13640), !dbg !13639
  %_10.i6879 = icmp eq i32 %_29.val6280, 0, !dbg !13643
  br i1 %_10.i6879, label %panic.i6889, label %bb1.i6880, !dbg !13643

bb1.i6880:                                        ; preds = %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit6878
  %_28 = getelementptr inbounds nuw i8, ptr %self, i32 800, !dbg !13647
  %_7.i = load i32, ptr %_28, align 4, !dbg !13648, !alias.scope !13640, !noundef !10
  %_8.i6881 = urem i32 %frames, %_29.val6280, !dbg !13643
  %_5.i6882 = add i32 %_8.i6881, %_7.i, !dbg !13649
  %_4.i6883 = urem i32 %_5.i6882, %_29.val6280, !dbg !13650
  store i32 %_4.i6883, ptr %_28, align 4, !dbg !13651, !alias.scope !13640
  %_17.i6884 = icmp eq i32 %_29.val, 0, !dbg !13652
  br i1 %_17.i6884, label %panic2.i, label %_RNvMs1_CsjLJhryqjeDL_17true_peak_limiterNtB5_7Cursors7advance.exit, !dbg !13652

panic.i6889:                                      ; preds = %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit6878
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c7e64e9e8489bc8e648659d0098d136c) #33, !dbg !13643, !noalias !13640
  unreachable, !dbg !13643

panic2.i:                                         ; preds = %bb1.i6880
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_34b23597fad9d85cb3bee6d64ebf77d5) #33, !dbg !13652, !noalias !13640
  unreachable, !dbg !13652

_RNvMs1_CsjLJhryqjeDL_17true_peak_limiterNtB5_7Cursors7advance.exit: ; preds = %bb1.i6880
  %2110 = getelementptr inbounds nuw i8, ptr %self, i32 804, !dbg !13653
  %_14.i6886 = load i32, ptr %2110, align 4, !dbg !13653, !alias.scope !13640, !noundef !10
  %_15.i6887 = urem i32 %frames, %_29.val, !dbg !13652
  %_12.i = add i32 %_15.i6887, %_14.i6886, !dbg !13654
  %_11.i6888 = urem i32 %_12.i, %_29.val, !dbg !13655
  store i32 %_11.i6888, ptr %2110, align 4, !dbg !13656, !alias.scope !13640
  br label %bb42, !dbg !13657

bb28:                                             ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_37 = tail call noundef zeroext i1 @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #32, !dbg !13658
  br i1 %_37, label %bb30, label %bb40, !dbg !13659

bb30:                                             ; preds = %bb28
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_39 = tail call noundef zeroext i1 @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #32, !dbg !13660
  br i1 %_39, label %bb32, label %bb40, !dbg !13661

bb32:                                             ; preds = %bb30
  %_80.not = icmp ugt i32 %words, %left_io.1
  br i1 %_80.not, label %bb51, label %bb1.i6890, !dbg !13662, !prof !4694

bb51:                                             ; preds = %bb32
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef %words, i32 noundef %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_8457596f73b1d71f2ddde42261d136f3) #33, !dbg !13670
  unreachable, !dbg !13670

bb1.i6890:                                        ; preds = %bb32, %bb12.i6904
  %io.sroa.5.0.i6891 = phi i32 [ %len.i.i.i6897, %bb12.i6904 ], [ %words, %bb32 ]
  %io.sroa.0.0.i6892 = phi ptr [ %data.i.i.i6896, %bb12.i6904 ], [ %left_io.0, %bb32 ]
  %2111 = icmp eq i32 %io.sroa.5.0.i6891, 0, !dbg !13671
  br i1 %2111, label %bb34, label %bb13.preheader.i6893, !dbg !13671

bb13.preheader.i6893:                             ; preds = %bb1.i6890
  %spec.store.select.i6894 = tail call i32 @llvm.umin.i32(i32 %io.sroa.5.0.i6891, i32 32), !dbg !13674
  %data.i.i.idx.i6895 = shl nuw nsw i32 %spec.store.select.i6894, 2, !dbg !13677
  %data.i.i.i6896 = getelementptr inbounds nuw i8, ptr %io.sroa.0.0.i6892, i32 %data.i.i.idx.i6895, !dbg !13677
  br label %bb13.i6898, !dbg !13682

bb13.i6898:                                       ; preds = %bb13.i6898, %bb13.preheader.i6893
  %iter.sroa.0.08.i6899 = phi ptr [ %_35.i6901, %bb13.i6898 ], [ %io.sroa.0.0.i6892, %bb13.preheader.i6893 ]
  %bits.sroa.0.07.i6900 = phi i32 [ %2112, %bb13.i6898 ], [ 0, %bb13.preheader.i6893 ]
  %_35.i6901 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i6899, i32 4, !dbg !13684
  %_95.i6902 = load i32, ptr %iter.sroa.0.08.i6899, align 4, !dbg !13686, !alias.scope !13687, !noundef !10
  %2112 = or i32 %_95.i6902, %bits.sroa.0.07.i6900, !dbg !13690
  %_29.i6903 = icmp eq ptr %_35.i6901, %data.i.i.i6896, !dbg !13691
  br i1 %_29.i6903, label %bb12.i6904, label %bb13.i6898, !dbg !13682

bb12.i6904:                                       ; preds = %bb13.i6898
  %len.i.i.i6897 = sub nuw nsw i32 %io.sroa.5.0.i6891, %spec.store.select.i6894, !dbg !13693
  %2113 = icmp eq i32 %2112, 0, !dbg !13694
  br i1 %2113, label %bb1.i6890, label %bb40, !dbg !13694

bb34:                                             ; preds = %bb1.i6890
  %_88.not = icmp ugt i32 %words, %right_io.1, !dbg !13695
  br i1 %_88.not, label %bb54, label %bb1.i6918, !dbg !13695, !prof !902

bb40:                                             ; preds = %bb12.i6904, %bb12.i6932, %bb1.i6918, %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, %bb28, %bb30
  %_36.sroa.0.0.off0 = phi i1 [ false, %bb12.i6932 ], [ false, %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit ], [ false, %bb30 ], [ false, %bb28 ], [ true, %bb1.i6918 ], [ false, %bb12.i6904 ]
  %2114 = zext i1 %_36.sroa.0.0.off0 to i8, !dbg !13701
  store i8 %2114, ptr %38, align 4, !dbg !13701
  %2115 = load i8, ptr %2, align 8, !dbg !13702, !range !4765, !noundef !10
  store i8 %2115, ptr %0, align 1, !dbg !13703
  call void @llvm.lifetime.start.p0(ptr nonnull %shape), !dbg !13704
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(12) %shape, ptr noundef nonnull align 16 dereferenceable(12) %_32, i32 12, i1 false), !dbg !13705
  %2116 = getelementptr inbounds nuw i8, ptr %self, i32 880, !dbg !13706
  %2117 = load i32, ptr %2116, align 8, !dbg !13706, !noundef !10
  %2118 = getelementptr inbounds nuw i8, ptr %self, i32 808, !dbg !13708
  %_99.0 = load ptr, ptr %2118, align 8, !dbg !13708, !nonnull !10, !noundef !10
  %2119 = getelementptr inbounds nuw i8, ptr %self, i32 812, !dbg !13708
  %_99.1 = load i32, ptr %2119, align 4, !dbg !13708, !noundef !10
  %2120 = getelementptr inbounds nuw i8, ptr %self, i32 816, !dbg !13708
  %_100.0 = load ptr, ptr %2120, align 16, !dbg !13708, !nonnull !10, !noundef !10
  %2121 = getelementptr inbounds nuw i8, ptr %self, i32 820, !dbg !13708
  %_100.1 = load i32, ptr %2121, align 4, !dbg !13708, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13715), !dbg !13718
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13719), !dbg !13718
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13721), !dbg !13718
  %fst_len.i.i = and i32 %left_io.1, 536870908, !dbg !13723
  %_22.not.i14201.i = icmp eq i32 %fst_len.i.i, 0, !dbg !13732
  br i1 %_22.not.i14201.i, label %bb2.i6913, label %bb13.i15.i, !dbg !13732

bb13.i15.i:                                       ; preds = %bb40, %bb13.i15.i
  %iter.sroa.0.0.i13204.i = phi ptr [ %_27.i16.i, %bb13.i15.i ], [ %left_io.0, %bb40 ]
  %iter.sroa.5.0.i12203.i = phi i32 [ %_28.i17.i, %bb13.i15.i ], [ %fst_len.i.i, %bb40 ]
  %ok.i2.sroa.0.0202.i = phi <4 x i32> [ %2124, %bb13.i15.i ], [ splat (i32 -1), %bb40 ]
  %_27.i16.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i13204.i, i32 16, !dbg !13739
  %_28.i17.i = add i32 %iter.sroa.5.0.i12203.i, -4, !dbg !13746
  %lanes.i.sroa.0.0.copyload.i = load <4 x float>, ptr %iter.sroa.0.0.i13204.i, align 4, !dbg !13747, !alias.scope !13753, !noalias !13757
  %2122 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %lanes.i.sroa.0.0.copyload.i), !dbg !13762
  %2123 = fcmp olt <4 x float> %2122, splat (float 0x46293E5940000000), !dbg !13767
  %2124 = select <4 x i1> %2123, <4 x i32> %ok.i2.sroa.0.0202.i, <4 x i32> zeroinitializer, !dbg !13772
  %_22.not.i14.i = icmp eq i32 %_28.i17.i, 0, !dbg !13732
  br i1 %_22.not.i14.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit25.i, label %bb13.i15.i, !dbg !13732

_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit25.i: ; preds = %bb13.i15.i
  %2125 = bitcast <4 x i32> %2124 to <16 x i8>, !dbg !13776
  %2126 = xor <16 x i8> %2125, splat (i8 -1), !dbg !13776
  %2127 = tail call i32 @llvm.wasm.anytrue.v16i8(<16 x i8> %2126), !dbg !13783
  %2128 = icmp eq i32 %2127, 0, !dbg !13783
  br i1 %2128, label %bb2.i6913, label %bb19.i.preheader.i, !dbg !13784

bb2.i6913:                                        ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit25.i, %bb40
  %fst_len.i89.i = and i32 %right_io.1, 536870908, !dbg !13785
  %_22.not.i205.i = icmp eq i32 %fst_len.i89.i, 0, !dbg !13789
  br i1 %_22.not.i205.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB1z_11LimiterCoreBR_E13process_block0EB1z_.exit, label %bb13.i.i6914, !dbg !13789

bb13.i.i6914:                                     ; preds = %bb2.i6913, %bb13.i.i6914
  %iter.sroa.0.0.i208.i = phi ptr [ %_27.i.i6915, %bb13.i.i6914 ], [ %right_io.0, %bb2.i6913 ]
  %iter.sroa.5.0.i207.i = phi i32 [ %_28.i.i6916, %bb13.i.i6914 ], [ %fst_len.i89.i, %bb2.i6913 ]
  %ok.i.sroa.0.0206.i = phi <4 x i32> [ %2131, %bb13.i.i6914 ], [ splat (i32 -1), %bb2.i6913 ]
  %_27.i.i6915 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i208.i, i32 16, !dbg !13792
  %_28.i.i6916 = add i32 %iter.sroa.5.0.i207.i, -4, !dbg !13795
  %lanes.i41.sroa.0.0.copyload.i = load <4 x float>, ptr %iter.sroa.0.0.i208.i, align 4, !dbg !13796, !alias.scope !13801, !noalias !13805
  %2129 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %lanes.i41.sroa.0.0.copyload.i), !dbg !13809
  %2130 = fcmp olt <4 x float> %2129, splat (float 0x46293E5940000000), !dbg !13813
  %2131 = select <4 x i1> %2130, <4 x i32> %ok.i.sroa.0.0206.i, <4 x i32> zeroinitializer, !dbg !13818
  %_22.not.i.i = icmp eq i32 %_28.i.i6916, 0, !dbg !13789
  br i1 %_22.not.i.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i, label %bb13.i.i6914, !dbg !13789

_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i: ; preds = %bb13.i.i6914
  %2132 = bitcast <4 x i32> %2131 to <16 x i8>, !dbg !13822
  %2133 = xor <16 x i8> %2132, splat (i8 -1), !dbg !13822
  %2134 = tail call i32 @llvm.wasm.anytrue.v16i8(<16 x i8> %2133), !dbg !13826
  %2135 = icmp eq i32 %2134, 0, !dbg !13826
  br i1 %2135, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB1z_11LimiterCoreBR_E13process_block0EB1z_.exit, label %bb7.i6917, !dbg !13827

bb7.i6917:                                        ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i
  br i1 %_22.not.i14201.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i, label %bb19.i.preheader.i, !dbg !13828

bb19.i.preheader.i:                               ; preds = %bb7.i6917, %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit25.i
  br label %bb19.i.i, !dbg !13828

bb20.loopexit.i.i:                                ; preds = %bb19.i.i
  %2136 = bitcast <4 x i32> %2139 to <16 x i8>, !dbg !13832
  %.pre = and i32 %right_io.1, 536870908, !dbg !13836
  br label %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i, !dbg !13840

bb19.i.i:                                         ; preds = %bb19.i.i, %bb19.i.preheader.i
  %iter.sroa.0.089.i.i = phi ptr [ %_42.i.i6907, %bb19.i.i ], [ %left_io.0, %bb19.i.preheader.i ]
  %iter.sroa.5.088.i.i = phi i32 [ %_43.i.i6908, %bb19.i.i ], [ %fst_len.i.i, %bb19.i.preheader.i ]
  %ok.sroa.0.087.i.i = phi <4 x i32> [ %2139, %bb19.i.i ], [ splat (i32 -1), %bb19.i.preheader.i ]
  %_42.i.i6907 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.089.i.i, i32 16, !dbg !13841
  %_43.i.i6908 = add i32 %iter.sroa.5.088.i.i, -4, !dbg !13844
  %lanes.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %iter.sroa.0.089.i.i, align 4, !dbg !13845, !alias.scope !13850, !noalias !13856
  %2137 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %lanes.i.sroa.0.0.copyload.i.i), !dbg !13860
  %2138 = fcmp olt <4 x float> %2137, splat (float 0x46293E5940000000), !dbg !13864
  %2139 = select <4 x i1> %2138, <4 x i32> %ok.sroa.0.087.i.i, <4 x i32> zeroinitializer, !dbg !13869
  %_37.not.i.i = icmp eq i32 %_43.i.i6908, 0, !dbg !13828
  br i1 %_37.not.i.i, label %bb20.loopexit.i.i, label %bb19.i.i, !dbg !13828

_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i: ; preds = %bb20.loopexit.i.i, %bb7.i6917
  %fst_len.i.i93.i.pre-phi = phi i32 [ %.pre, %bb20.loopexit.i.i ], [ %fst_len.i89.i, %bb7.i6917 ], !dbg !13836
  %ok.sroa.0.0.lcssa.i.i = phi <16 x i8> [ %2136, %bb20.loopexit.i.i ], [ splat (i8 -1), %bb7.i6917 ], !dbg !13873
  %_4.i39.i.i = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> zeroinitializer, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %ok.sroa.0.0.lcssa.i.i), !dbg !13874
  %2140 = bitcast <16 x i8> %_4.i39.i.i to <4 x i32>, !dbg !13875
  %words.sroa.0.0.vec.extract.i.i = extractelement <4 x i32> %2140, i64 0, !dbg !13880
  %2141 = icmp ne i32 %words.sroa.0.0.vec.extract.i.i, 0, !dbg !13880
  %2142 = zext i1 %2141 to i32, !dbg !13880
  %words.sroa.0.4.vec.extract.i.i = extractelement <4 x i32> %2140, i64 1, !dbg !13880
  %2143 = icmp eq i32 %words.sroa.0.4.vec.extract.i.i, 0, !dbg !13880
  %2144 = select i1 %2143, i32 0, i32 2, !dbg !13880
  %words.sroa.0.8.vec.extract.i.i = extractelement <4 x i32> %2140, i64 2, !dbg !13880
  %2145 = icmp eq i32 %words.sroa.0.8.vec.extract.i.i, 0, !dbg !13880
  %2146 = select i1 %2145, i32 0, i32 4, !dbg !13880
  %words.sroa.0.12.vec.extract.i.i = extractelement <4 x i32> %2140, i64 3, !dbg !13880
  %2147 = icmp eq i32 %words.sroa.0.12.vec.extract.i.i, 0, !dbg !13880
  %2148 = select i1 %2147, i32 0, i32 8, !dbg !13880
  %_37.not86.i94.i = icmp eq i32 %fst_len.i.i93.i.pre-phi, 0, !dbg !13881
  br i1 %_37.not86.i94.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit113.i, label %bb19.i95.i, !dbg !13881

bb20.loopexit.i103.i:                             ; preds = %bb19.i95.i
  %2149 = bitcast <4 x i32> %2152 to <16 x i8>, !dbg !13884
  br label %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit113.i, !dbg !13888

bb19.i95.i:                                       ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i, %bb19.i95.i
  %iter.sroa.0.089.i96.i = phi ptr [ %_42.i99.i, %bb19.i95.i ], [ %right_io.0, %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i ]
  %iter.sroa.5.088.i97.i = phi i32 [ %_43.i100.i, %bb19.i95.i ], [ %fst_len.i.i93.i.pre-phi, %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i ]
  %ok.sroa.0.087.i98.i = phi <4 x i32> [ %2152, %bb19.i95.i ], [ splat (i32 -1), %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i ]
  %_42.i99.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.089.i96.i, i32 16, !dbg !13889
  %_43.i100.i = add i32 %iter.sroa.5.088.i97.i, -4, !dbg !13892
  %lanes.i.sroa.0.0.copyload.i101.i = load <4 x float>, ptr %iter.sroa.0.089.i96.i, align 4, !dbg !13893, !alias.scope !13898, !noalias !13904
  %2150 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %lanes.i.sroa.0.0.copyload.i101.i), !dbg !13908
  %2151 = fcmp olt <4 x float> %2150, splat (float 0x46293E5940000000), !dbg !13912
  %2152 = select <4 x i1> %2151, <4 x i32> %ok.sroa.0.087.i98.i, <4 x i32> zeroinitializer, !dbg !13917
  %_37.not.i102.i = icmp eq i32 %_43.i100.i, 0, !dbg !13881
  br i1 %_37.not.i102.i, label %bb20.loopexit.i103.i, label %bb19.i95.i, !dbg !13881

_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit113.i: ; preds = %bb20.loopexit.i103.i, %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i
  %ok.sroa.0.0.lcssa.i104.i = phi <16 x i8> [ splat (i8 -1), %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i ], [ %2149, %bb20.loopexit.i103.i ], !dbg !13921
  %_4.i39.i105.i = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> zeroinitializer, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %ok.sroa.0.0.lcssa.i104.i), !dbg !13922
  %2153 = bitcast <16 x i8> %_4.i39.i105.i to <4 x i32>, !dbg !13923
  %words.sroa.0.0.vec.extract.i106.i = extractelement <4 x i32> %2153, i64 0, !dbg !13928
  %2154 = icmp ne i32 %words.sroa.0.0.vec.extract.i106.i, 0, !dbg !13928
  %2155 = zext i1 %2154 to i32, !dbg !13928
  %words.sroa.0.4.vec.extract.i107.i = extractelement <4 x i32> %2153, i64 1, !dbg !13928
  %2156 = icmp eq i32 %words.sroa.0.4.vec.extract.i107.i, 0, !dbg !13928
  %2157 = select i1 %2156, i32 0, i32 2, !dbg !13928
  %words.sroa.0.8.vec.extract.i109.i = extractelement <4 x i32> %2153, i64 2, !dbg !13928
  %2158 = icmp eq i32 %words.sroa.0.8.vec.extract.i109.i, 0, !dbg !13928
  %2159 = select i1 %2158, i32 0, i32 4, !dbg !13928
  %words.sroa.0.12.vec.extract.i111.i = extractelement <4 x i32> %2153, i64 3, !dbg !13928
  %2160 = icmp eq i32 %words.sroa.0.12.vec.extract.i111.i, 0, !dbg !13928
  %2161 = select i1 %2160, i32 0, i32 8, !dbg !13928
  %2162 = getelementptr inbounds nuw i8, ptr %self, i32 8, !dbg !13929
  %mask.sroa.0.1.1.i108.i = or disjoint i32 %2144, %2142, !dbg !13928
  %mask.sroa.0.1.2.i110.i = or disjoint i32 %mask.sroa.0.1.1.i108.i, %2146, !dbg !13928
  %mask.sroa.0.1.3.i112.i = or disjoint i32 %mask.sroa.0.1.2.i110.i, %2148, !dbg !13928
  %mask.sroa.0.1.1.i.i = or i32 %mask.sroa.0.1.3.i112.i, %2155, !dbg !13880
  %mask.sroa.0.1.2.i.i = or i32 %mask.sroa.0.1.1.i.i, %2157, !dbg !13880
  %mask.sroa.0.1.3.i.i = or i32 %mask.sroa.0.1.2.i.i, %2159, !dbg !13880
  %2163 = or i32 %mask.sroa.0.1.3.i.i, %2161, !dbg !13929
  store i32 %2163, ptr %2162, align 8, !dbg !13929, !alias.scope !13721, !noalias !13930
  %_14.i6909 = load i64, ptr %self, align 8, !dbg !13931, !alias.scope !13721, !noalias !13930, !noundef !10
  %2164 = tail call i64 @llvm.uadd.sat.i64(i64 %_14.i6909, i64 1), !dbg !13932
  store i64 %2164, ptr %self, align 8, !dbg !13935, !alias.scope !13721, !noalias !13930
  %_222.i.i = icmp eq i32 %left_io.1, 0, !dbg !13936
  br i1 %_222.i.i, label %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit.i, label %bb12.i.preheader.i, !dbg !13942

bb12.i.preheader.i:                               ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit113.i
  %.idx.i.i = shl nuw nsw i32 %left_io.1, 2, !dbg !13943
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %left_io.0, i8 0, i32 %.idx.i.i, i1 false), !dbg !13947, !alias.scope !13948, !noalias !13951
  br label %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit.i, !dbg !13952

_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit.i: ; preds = %bb12.i.preheader.i, %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit113.i
  %_222.i115.i = icmp eq i32 %right_io.1, 0, !dbg !13958
  br i1 %_222.i115.i, label %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit120.i, label %bb12.i116.preheader.i, !dbg !13961

bb12.i116.preheader.i:                            ; preds = %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit.i
  %.idx.i114.i = shl nuw nsw i32 %right_io.1, 2, !dbg !13952
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %right_io.0, i8 0, i32 %.idx.i114.i, i1 false), !dbg !13962, !alias.scope !13963, !noalias !13966
  br label %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit120.i, !dbg !13967

_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit120.i: ; preds = %bb12.i116.preheader.i, %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit.i
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 4 dereferenceable(100) %_33, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(12) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_99.0, i32 noundef %_99.1, i32 noundef %2117) #32, !dbg !13968, !noalias !13973
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 4 dereferenceable(100) %_34, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(12) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_100.0, i32 noundef %_100.1, i32 noundef %2117) #32, !dbg !13976, !noalias !13973
  store i32 0, ptr %_35, align 4, !dbg !13977, !noalias !13973
  %2165 = getelementptr inbounds nuw i8, ptr %self, i32 804, !dbg !13977
  store i32 0, ptr %2165, align 4, !dbg !13977, !noalias !13973
  br label %_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB1z_11LimiterCoreBR_E13process_block0EB1z_.exit, !dbg !13978

_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB1z_11LimiterCoreBR_E13process_block0EB1z_.exit: ; preds = %bb2.i6913, %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i, %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit120.i
  call void @llvm.lifetime.end.p0(ptr nonnull %shape), !dbg !13979
  br label %bb42, !dbg !13657

bb54:                                             ; preds = %bb34
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef %words, i32 noundef %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_8b6b4fff29b57e2077f91d7df0eea7e7) #33, !dbg !13980
  unreachable, !dbg !13980

bb1.i6918:                                        ; preds = %bb34, %bb12.i6932
  %io.sroa.5.0.i6919 = phi i32 [ %len.i.i.i6925, %bb12.i6932 ], [ %words, %bb34 ]
  %io.sroa.0.0.i6920 = phi ptr [ %data.i.i.i6924, %bb12.i6932 ], [ %right_io.0, %bb34 ]
  %2166 = icmp eq i32 %io.sroa.5.0.i6919, 0, !dbg !13981
  br i1 %2166, label %bb40, label %bb13.preheader.i6921, !dbg !13981

bb13.preheader.i6921:                             ; preds = %bb1.i6918
  %spec.store.select.i6922 = tail call i32 @llvm.umin.i32(i32 %io.sroa.5.0.i6919, i32 32), !dbg !13984
  %data.i.i.idx.i6923 = shl nuw nsw i32 %spec.store.select.i6922, 2, !dbg !13987
  %data.i.i.i6924 = getelementptr inbounds nuw i8, ptr %io.sroa.0.0.i6920, i32 %data.i.i.idx.i6923, !dbg !13987
  br label %bb13.i6926, !dbg !13992

bb13.i6926:                                       ; preds = %bb13.i6926, %bb13.preheader.i6921
  %iter.sroa.0.08.i6927 = phi ptr [ %_35.i6929, %bb13.i6926 ], [ %io.sroa.0.0.i6920, %bb13.preheader.i6921 ]
  %bits.sroa.0.07.i6928 = phi i32 [ %2167, %bb13.i6926 ], [ 0, %bb13.preheader.i6921 ]
  %_35.i6929 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i6927, i32 4, !dbg !13994
  %_95.i6930 = load i32, ptr %iter.sroa.0.08.i6927, align 4, !dbg !13996, !alias.scope !13997, !noundef !10
  %2167 = or i32 %_95.i6930, %bits.sroa.0.07.i6928, !dbg !14000
  %_29.i6931 = icmp eq ptr %_35.i6929, %data.i.i.i6924, !dbg !14001
  br i1 %_29.i6931, label %bb12.i6932, label %bb13.i6926, !dbg !13992

bb12.i6932:                                       ; preds = %bb13.i6926
  %len.i.i.i6925 = sub nuw nsw i32 %io.sroa.5.0.i6919, %spec.store.select.i6922, !dbg !14003
  %2168 = icmp eq i32 %2167, 0, !dbg !14004
  br i1 %2168, label %bb1.i6918, label %bb40, !dbg !14004

bb42:                                             ; preds = %_RNvMs1_CsjLJhryqjeDL_17true_peak_limiterNtB5_7Cursors7advance.exit, %_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB1z_11LimiterCoreBR_E13process_block0EB1z_.exit
  ret void, !dbg !13657
}
