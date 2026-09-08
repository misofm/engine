define internal fastcc void @_RNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB5_11LimiterCorefE13process_blockB5_(ptr noalias noundef nonnull align 8 dereferenceable(544) %self, ptr noalias noundef nonnull align 4 captures(address) %left_io.0, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef nonnull align 4 captures(address) %right_io.0, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef %frames) unnamed_addr #0 !dbg !18760 {
start:
  %_112.i1012 = alloca [92 x i8], align 4
  %_110.i1013 = alloca [92 x i8], align 4
  %peaks_right.i1014 = alloca [1024 x i8], align 4
  %peaks_left.i1015 = alloca [1024 x i8], align 4
  %scratch.i1016 = alloca [32 x i8], align 4
  %hot_right.i1017 = alloca [92 x i8], align 4
  %hot_left.i1018 = alloca [92 x i8], align 4
  %_112.i = alloca [92 x i8], align 4
  %_110.i = alloca [92 x i8], align 4
  %peaks_right.i731 = alloca [1024 x i8], align 4
  %peaks_left.i732 = alloca [1024 x i8], align 4
  %scratch.i = alloca [32 x i8], align 4
  %hot_right.i733 = alloca [92 x i8], align 4
  %hot_left.i734 = alloca [92 x i8], align 4
  %_159.i34 = alloca [92 x i8], align 4
  %_157.i35 = alloca [92 x i8], align 4
  %uniform_right.i50 = alloca [44 x i8], align 4
  %uniform_left.i51 = alloca [44 x i8], align 4
  %peaks_right.i52 = alloca [1024 x i8], align 4
  %peaks_left.i53 = alloca [1024 x i8], align 4
  %hot_right.i54 = alloca [92 x i8], align 4
  %hot_left.i55 = alloca [92 x i8], align 4
  %uniform_right.i = alloca [44 x i8], align 4
  %uniform_left.i = alloca [44 x i8], align 4
  %peaks_right.i = alloca [1024 x i8], align 4
  %peaks_left.i = alloca [1024 x i8], align 4
  %hot_right.i = alloca [92 x i8], align 4
  %hot_left.i = alloca [92 x i8], align 4
  %shape = alloca [12 x i8], align 4
  %0 = getelementptr inbounds nuw i8, ptr %self, i32 537, !dbg !18761
  %1 = load i8, ptr %0, align 1, !dbg !18761, !range !4667, !noundef !10
  %2 = getelementptr inbounds nuw i8, ptr %self, i32 80, !dbg !18763
  %3 = load i8, ptr %2, align 8, !dbg !18763, !range !4667, !noundef !10
  %_7 = icmp eq i8 %1, %3, !dbg !18761
  %4 = getelementptr inbounds nuw i8, ptr %self, i32 388
  %_95.0 = load ptr, ptr %4, align 4, !dbg !18764
  %5 = getelementptr inbounds nuw i8, ptr %self, i32 392
  %_95.1 = load i32, ptr %5, align 4, !dbg !18764
  br i1 %_7, label %bb1, label %bb20.thread, !dbg !18761

bb1:                                              ; preds = %start
  %_8.i4360 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_95.0, i32 %_95.1, !dbg !18765
  br label %bb1.i.i, !dbg !18770

bb1.i.i:                                          ; preds = %bb11.i.i, %bb1
  %_221.i.i = phi ptr [ %_22.i.i4362, %bb11.i.i ], [ %_95.0, %bb1 ]
  %_12.i.i4361 = icmp eq ptr %_221.i.i, %_8.i4360, !dbg !18772
  br i1 %_12.i.i4361, label %bb3, label %bb11.i.i, !dbg !18775

bb11.i.i:                                         ; preds = %bb1.i.i
  %_22.i.i4362 = getelementptr inbounds nuw i8, ptr %_221.i.i, i32 16, !dbg !18776
  %6 = getelementptr inbounds nuw i8, ptr %_221.i.i, i32 12, !dbg !18778
  %_3.i.i.i = load i32, ptr %6, align 4, !dbg !18778, !alias.scope !18780, !noalias !18785, !noundef !10
  %7 = icmp eq i32 %_3.i.i.i, 0, !dbg !18778
  %_51.i.i.i = load i32, ptr %_221.i.i, align 4, !dbg !18778, !alias.scope !18780, !noalias !18785
  %8 = getelementptr inbounds nuw i8, ptr %_221.i.i, i32 4, !dbg !18778
  %_72.i.i.i = load i32, ptr %8, align 4, !dbg !18778, !alias.scope !18780, !noalias !18785
  %9 = icmp eq i32 %_51.i.i.i, %_72.i.i.i, !dbg !18778
  %_0.sroa.0.0.off0.i.i.i = select i1 %7, i1 %9, i1 false, !dbg !18778
  br i1 %_0.sroa.0.0.off0.i.i.i, label %bb1.i.i, label %bb20.thread, !dbg !18788

bb3:                                              ; preds = %bb1.i.i
  %10 = getelementptr inbounds nuw i8, ptr %self, i32 396, !dbg !18789
  %_96.0 = load ptr, ptr %10, align 4, !dbg !18789, !nonnull !10, !noundef !10
  %11 = getelementptr inbounds nuw i8, ptr %self, i32 400, !dbg !18789
  %_96.1 = load i32, ptr %11, align 4, !dbg !18789, !noundef !10
  %_8.i4363 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_96.0, i32 %_96.1, !dbg !18790
  br label %bb1.i.i4364, !dbg !18795

bb1.i.i4364:                                      ; preds = %bb11.i.i4367, %bb3
  %_221.i.i4365 = phi ptr [ %_22.i.i4368, %bb11.i.i4367 ], [ %_96.0, %bb3 ]
  %_12.i.i4366 = icmp eq ptr %_221.i.i4365, %_8.i4363, !dbg !18797
  br i1 %_12.i.i4366, label %bb5, label %bb11.i.i4367, !dbg !18800

bb11.i.i4367:                                     ; preds = %bb1.i.i4364
  %_22.i.i4368 = getelementptr inbounds nuw i8, ptr %_221.i.i4365, i32 16, !dbg !18801
  %12 = getelementptr inbounds nuw i8, ptr %_221.i.i4365, i32 12, !dbg !18803
  %_3.i.i.i4369 = load i32, ptr %12, align 4, !dbg !18803, !alias.scope !18805, !noalias !18810, !noundef !10
  %13 = icmp eq i32 %_3.i.i.i4369, 0, !dbg !18803
  %_51.i.i.i4370 = load i32, ptr %_221.i.i4365, align 4, !dbg !18803, !alias.scope !18805, !noalias !18810
  %14 = getelementptr inbounds nuw i8, ptr %_221.i.i4365, i32 4, !dbg !18803
  %_72.i.i.i4371 = load i32, ptr %14, align 4, !dbg !18803, !alias.scope !18805, !noalias !18810
  %15 = icmp eq i32 %_51.i.i.i4370, %_72.i.i.i4371, !dbg !18803
  %_0.sroa.0.0.off0.i.i.i4372 = select i1 %13, i1 %15, i1 false, !dbg !18803
  br i1 %_0.sroa.0.0.off0.i.i.i4372, label %bb1.i.i4364, label %bb20.thread, !dbg !18813

bb5:                                              ; preds = %bb1.i.i4364
  %16 = getelementptr inbounds nuw i8, ptr %self, i32 488, !dbg !18814
  %_97.0 = load ptr, ptr %16, align 8, !dbg !18814, !nonnull !10, !noundef !10
  %17 = getelementptr inbounds nuw i8, ptr %self, i32 492, !dbg !18814
  %_97.1 = load i32, ptr %17, align 4, !dbg !18814, !noundef !10
  %_8.i4374 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_97.0, i32 %_97.1, !dbg !18815
  br label %bb1.i.i4375, !dbg !18820

bb1.i.i4375:                                      ; preds = %bb11.i.i4378, %bb5
  %_221.i.i4376 = phi ptr [ %_22.i.i4379, %bb11.i.i4378 ], [ %_97.0, %bb5 ]
  %_12.i.i4377 = icmp eq ptr %_221.i.i4376, %_8.i4374, !dbg !18822
  br i1 %_12.i.i4377, label %bb7, label %bb11.i.i4378, !dbg !18825

bb11.i.i4378:                                     ; preds = %bb1.i.i4375
  %_22.i.i4379 = getelementptr inbounds nuw i8, ptr %_221.i.i4376, i32 16, !dbg !18826
  %18 = getelementptr inbounds nuw i8, ptr %_221.i.i4376, i32 12, !dbg !18828
  %_3.i.i.i4380 = load i32, ptr %18, align 4, !dbg !18828, !alias.scope !18830, !noalias !18835, !noundef !10
  %19 = icmp eq i32 %_3.i.i.i4380, 0, !dbg !18828
  %_51.i.i.i4381 = load i32, ptr %_221.i.i4376, align 4, !dbg !18828, !alias.scope !18830, !noalias !18835
  %20 = getelementptr inbounds nuw i8, ptr %_221.i.i4376, i32 4, !dbg !18828
  %_72.i.i.i4382 = load i32, ptr %20, align 4, !dbg !18828, !alias.scope !18830, !noalias !18835
  %21 = icmp eq i32 %_51.i.i.i4381, %_72.i.i.i4382, !dbg !18828
  %_0.sroa.0.0.off0.i.i.i4383 = select i1 %19, i1 %21, i1 false, !dbg !18828
  br i1 %_0.sroa.0.0.off0.i.i.i4383, label %bb1.i.i4375, label %bb20.thread, !dbg !18838

bb7:                                              ; preds = %bb1.i.i4375
  %22 = getelementptr inbounds nuw i8, ptr %self, i32 496, !dbg !18839
  %_98.0 = load ptr, ptr %22, align 8, !dbg !18839, !nonnull !10, !noundef !10
  %23 = getelementptr inbounds nuw i8, ptr %self, i32 500, !dbg !18839
  %_98.1 = load i32, ptr %23, align 4, !dbg !18839, !noundef !10
  %_8.i4385 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_98.0, i32 %_98.1, !dbg !18840
  br label %bb1.i.i4386, !dbg !18845

bb1.i.i4386:                                      ; preds = %bb11.i.i4389, %bb7
  %_221.i.i4387 = phi ptr [ %_22.i.i4390, %bb11.i.i4389 ], [ %_98.0, %bb7 ]
  %_12.i.i4388 = icmp eq ptr %_221.i.i4387, %_8.i4385, !dbg !18847
  br i1 %_12.i.i4388, label %bb9, label %bb11.i.i4389, !dbg !18850

bb11.i.i4389:                                     ; preds = %bb1.i.i4386
  %_22.i.i4390 = getelementptr inbounds nuw i8, ptr %_221.i.i4387, i32 16, !dbg !18851
  %24 = getelementptr inbounds nuw i8, ptr %_221.i.i4387, i32 12, !dbg !18853
  %_3.i.i.i4391 = load i32, ptr %24, align 4, !dbg !18853, !alias.scope !18855, !noalias !18860, !noundef !10
  %25 = icmp eq i32 %_3.i.i.i4391, 0, !dbg !18853
  %_51.i.i.i4392 = load i32, ptr %_221.i.i4387, align 4, !dbg !18853, !alias.scope !18855, !noalias !18860
  %26 = getelementptr inbounds nuw i8, ptr %_221.i.i4387, i32 4, !dbg !18853
  %_72.i.i.i4393 = load i32, ptr %26, align 4, !dbg !18853, !alias.scope !18855, !noalias !18860
  %27 = icmp eq i32 %_51.i.i.i4392, %_72.i.i.i4393, !dbg !18853
  %_0.sroa.0.0.off0.i.i.i4394 = select i1 %25, i1 %27, i1 false, !dbg !18853
  br i1 %_0.sroa.0.0.off0.i.i.i4394, label %bb1.i.i4386, label %bb20.thread, !dbg !18863

bb9:                                              ; preds = %bb1.i.i4386
  %_65.not = icmp ugt i32 %frames, %left_io.1
  br i1 %_65.not, label %bb45, label %bb1.i4396, !dbg !18864, !prof !4596

bb45:                                             ; preds = %bb9
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef %frames, i32 noundef %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_9d0ae4ec703392629abc127e52af1d06) #32, !dbg !18873
  unreachable, !dbg !18873

bb1.i4396:                                        ; preds = %bb9, %bb12.i4400
  %io.sroa.5.0.i = phi i32 [ %len.i.i.i, %bb12.i4400 ], [ %frames, %bb9 ]
  %io.sroa.0.0.i = phi ptr [ %data.i.i.i, %bb12.i4400 ], [ %left_io.0, %bb9 ]
  %28 = icmp eq i32 %io.sroa.5.0.i, 0, !dbg !18874
  br i1 %28, label %bb11, label %bb13.preheader.i, !dbg !18874

bb13.preheader.i:                                 ; preds = %bb1.i4396
  %spec.store.select.i4397 = tail call i32 @llvm.umin.i32(i32 %io.sroa.5.0.i, i32 32), !dbg !18877
  %data.i.i.idx.i = shl nuw nsw i32 %spec.store.select.i4397, 2, !dbg !18880
  %data.i.i.i = getelementptr inbounds nuw i8, ptr %io.sroa.0.0.i, i32 %data.i.i.idx.i, !dbg !18880
  br label %bb13.i4398, !dbg !18885

bb13.i4398:                                       ; preds = %bb13.i4398, %bb13.preheader.i
  %iter.sroa.0.08.i = phi ptr [ %_35.i4399, %bb13.i4398 ], [ %io.sroa.0.0.i, %bb13.preheader.i ]
  %bits.sroa.0.07.i = phi i32 [ %29, %bb13.i4398 ], [ 0, %bb13.preheader.i ]
  %_35.i4399 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i, i32 4, !dbg !18887
  %_95.i = load i32, ptr %iter.sroa.0.08.i, align 4, !dbg !18889, !alias.scope !18890, !noundef !10
  %29 = or i32 %_95.i, %bits.sroa.0.07.i, !dbg !18893
  %_29.i = icmp eq ptr %_35.i4399, %data.i.i.i, !dbg !18894
  br i1 %_29.i, label %bb12.i4400, label %bb13.i4398, !dbg !18885

bb12.i4400:                                       ; preds = %bb13.i4398
  %len.i.i.i = sub nuw nsw i32 %io.sroa.5.0.i, %spec.store.select.i4397, !dbg !18896
  %30 = icmp eq i32 %29, 0, !dbg !18897
  br i1 %30, label %bb1.i4396, label %bb20.thread, !dbg !18897

bb11:                                             ; preds = %bb1.i4396
  %_73.not = icmp ugt i32 %frames, %right_io.1, !dbg !18898
  br i1 %_73.not, label %bb48, label %bb1.i4402, !dbg !18898, !prof !787

bb20.thread:                                      ; preds = %bb11.i.i, %bb11.i.i4367, %bb11.i.i4378, %bb11.i.i4389, %bb12.i4400, %start
  %31 = getelementptr inbounds nuw i8, ptr %self, i32 536
  br label %bb26, !dbg !18904

bb20:                                             ; preds = %bb1.i4402
  %32 = getelementptr inbounds nuw i8, ptr %self, i32 536
  %33 = load i8, ptr %32, align 8, !range !4667
  %_22 = trunc nuw i8 %33 to i1
  br i1 %_22, label %bb22, label %bb26, !dbg !18904

bb48:                                             ; preds = %bb11
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef %frames, i32 noundef %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_51ffbab136fc4cbad11477705c6179e2) #32, !dbg !18906
  unreachable, !dbg !18906

bb1.i4402:                                        ; preds = %bb11, %bb12.i4416
  %io.sroa.5.0.i4403 = phi i32 [ %len.i.i.i4409, %bb12.i4416 ], [ %frames, %bb11 ]
  %io.sroa.0.0.i4404 = phi ptr [ %data.i.i.i4408, %bb12.i4416 ], [ %right_io.0, %bb11 ]
  %34 = icmp eq i32 %io.sroa.5.0.i4403, 0, !dbg !18907
  br i1 %34, label %bb20, label %bb13.preheader.i4405, !dbg !18907

bb13.preheader.i4405:                             ; preds = %bb1.i4402
  %spec.store.select.i4406 = tail call i32 @llvm.umin.i32(i32 %io.sroa.5.0.i4403, i32 32), !dbg !18910
  %data.i.i.idx.i4407 = shl nuw nsw i32 %spec.store.select.i4406, 2, !dbg !18913
  %data.i.i.i4408 = getelementptr inbounds nuw i8, ptr %io.sroa.0.0.i4404, i32 %data.i.i.idx.i4407, !dbg !18913
  br label %bb13.i4410, !dbg !18918

bb13.i4410:                                       ; preds = %bb13.i4410, %bb13.preheader.i4405
  %iter.sroa.0.08.i4411 = phi ptr [ %_35.i4413, %bb13.i4410 ], [ %io.sroa.0.0.i4404, %bb13.preheader.i4405 ]
  %bits.sroa.0.07.i4412 = phi i32 [ %35, %bb13.i4410 ], [ 0, %bb13.preheader.i4405 ]
  %_35.i4413 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i4411, i32 4, !dbg !18920
  %_95.i4414 = load i32, ptr %iter.sroa.0.08.i4411, align 4, !dbg !18922, !alias.scope !18923, !noundef !10
  %35 = or i32 %_95.i4414, %bits.sroa.0.07.i4412, !dbg !18926
  %_29.i4415 = icmp eq ptr %_35.i4413, %data.i.i.i4408, !dbg !18927
  br i1 %_29.i4415, label %bb12.i4416, label %bb13.i4410, !dbg !18918

bb12.i4416:                                       ; preds = %bb13.i4410
  %len.i.i.i4409 = sub nuw nsw i32 %io.sroa.5.0.i4403, %spec.store.select.i4406, !dbg !18929
  %36 = icmp eq i32 %35, 0, !dbg !18930
  br i1 %36, label %bb1.i4402, label %bb20.thread5209, !dbg !18930

bb20.thread5209:                                  ; preds = %bb12.i4416
  %37 = getelementptr inbounds nuw i8, ptr %self, i32 536
  br label %bb26, !dbg !18904

bb26:                                             ; preds = %bb20.thread5209, %bb20.thread, %bb20
  %38 = phi ptr [ %31, %bb20.thread ], [ %32, %bb20 ], [ %37, %bb20.thread5209 ]
  %quiet.sroa.0.0.off05208 = phi i1 [ false, %bb20.thread ], [ true, %bb20 ], [ false, %bb20.thread5209 ]
  %_31 = getelementptr inbounds nuw i8, ptr %self, i32 128, !dbg !18931
  %_32 = getelementptr inbounds nuw i8, ptr %self, i32 524, !dbg !18932
  %_33 = getelementptr inbounds nuw i8, ptr %self, i32 324, !dbg !18933
  %_34 = getelementptr inbounds nuw i8, ptr %self, i32 424, !dbg !18934
  %_35 = getelementptr inbounds nuw i8, ptr %self, i32 104, !dbg !18935
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18936), !dbg !18939
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18942), !dbg !18939
  %_8.i4419 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_95.0, i32 %_95.1, !dbg !18944
  br label %bb1.i.i4420, !dbg !18950

bb1.i.i4420:                                      ; preds = %bb11.i.i4423, %bb26
  %_221.i.i4421 = phi ptr [ %_22.i.i4424, %bb11.i.i4423 ], [ %_95.0, %bb26 ]
  %_12.i.i4422 = icmp eq ptr %_221.i.i4421, %_8.i4419, !dbg !18952
  br i1 %_12.i.i4422, label %bb2.i, label %bb11.i.i4423, !dbg !18955

bb11.i.i4423:                                     ; preds = %bb1.i.i4420
  %_22.i.i4424 = getelementptr inbounds nuw i8, ptr %_221.i.i4421, i32 16, !dbg !18956
  %39 = getelementptr inbounds nuw i8, ptr %_221.i.i4421, i32 12, !dbg !18958
  %_3.i.i.i4425 = load i32, ptr %39, align 4, !dbg !18958, !alias.scope !18960, !noalias !18965, !noundef !10
  %40 = icmp eq i32 %_3.i.i.i4425, 0, !dbg !18958
  %_51.i.i.i4426 = load i32, ptr %_221.i.i4421, align 4, !dbg !18958, !alias.scope !18960, !noalias !18965
  %41 = getelementptr inbounds nuw i8, ptr %_221.i.i4421, i32 4, !dbg !18958
  %_72.i.i.i4427 = load i32, ptr %41, align 4, !dbg !18958, !alias.scope !18960, !noalias !18965
  %42 = icmp eq i32 %_51.i.i.i4426, %_72.i.i.i4427, !dbg !18958
  %_0.sroa.0.0.off0.i.i.i4428 = select i1 %40, i1 %42, i1 false, !dbg !18958
  br i1 %_0.sroa.0.0.off0.i.i.i4428, label %bb1.i.i4420, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, !dbg !18974

bb2.i:                                            ; preds = %bb1.i.i4420
  %43 = getelementptr inbounds nuw i8, ptr %self, i32 396, !dbg !18975
  %_15.0.i = load ptr, ptr %43, align 4, !dbg !18975, !alias.scope !18936, !noalias !18976, !nonnull !10, !noundef !10
  %44 = getelementptr inbounds nuw i8, ptr %self, i32 400, !dbg !18975
  %_15.1.i = load i32, ptr %44, align 4, !dbg !18975, !alias.scope !18936, !noalias !18976, !noundef !10
  %_8.i4430 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_15.0.i, i32 %_15.1.i, !dbg !18977
  br label %bb1.i.i4431, !dbg !18982

bb1.i.i4431:                                      ; preds = %bb11.i.i4434, %bb2.i
  %_221.i.i4432 = phi ptr [ %_22.i.i4435, %bb11.i.i4434 ], [ %_15.0.i, %bb2.i ]
  %_12.i.i4433 = icmp eq ptr %_221.i.i4432, %_8.i4430, !dbg !18984
  br i1 %_12.i.i4433, label %bb4.i1963, label %bb11.i.i4434, !dbg !18987

bb11.i.i4434:                                     ; preds = %bb1.i.i4431
  %_22.i.i4435 = getelementptr inbounds nuw i8, ptr %_221.i.i4432, i32 16, !dbg !18988
  %45 = getelementptr inbounds nuw i8, ptr %_221.i.i4432, i32 12, !dbg !18990
  %_3.i.i.i4436 = load i32, ptr %45, align 4, !dbg !18990, !alias.scope !18992, !noalias !18997, !noundef !10
  %46 = icmp eq i32 %_3.i.i.i4436, 0, !dbg !18990
  %_51.i.i.i4437 = load i32, ptr %_221.i.i4432, align 4, !dbg !18990, !alias.scope !18992, !noalias !18997
  %47 = getelementptr inbounds nuw i8, ptr %_221.i.i4432, i32 4, !dbg !18990
  %_72.i.i.i4438 = load i32, ptr %47, align 4, !dbg !18990, !alias.scope !18992, !noalias !18997
  %48 = icmp eq i32 %_51.i.i.i4437, %_72.i.i.i4438, !dbg !18990
  %_0.sroa.0.0.off0.i.i.i4439 = select i1 %46, i1 %48, i1 false, !dbg !18990
  br i1 %_0.sroa.0.0.off0.i.i.i4439, label %bb1.i.i4431, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, !dbg !19000

bb4.i1963:                                        ; preds = %bb1.i.i4431
  %49 = getelementptr inbounds nuw i8, ptr %self, i32 488, !dbg !19001
  %_16.0.i = load ptr, ptr %49, align 4, !dbg !19001, !alias.scope !18942, !noalias !19002, !nonnull !10, !noundef !10
  %50 = getelementptr inbounds nuw i8, ptr %self, i32 492, !dbg !19001
  %_16.1.i = load i32, ptr %50, align 4, !dbg !19001, !alias.scope !18942, !noalias !19002, !noundef !10
  %_8.i4441 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_16.0.i, i32 %_16.1.i, !dbg !19003
  br label %bb1.i.i4442, !dbg !19008

bb1.i.i4442:                                      ; preds = %bb11.i.i4445, %bb4.i1963
  %_221.i.i4443 = phi ptr [ %_22.i.i4446, %bb11.i.i4445 ], [ %_16.0.i, %bb4.i1963 ]
  %_12.i.i4444 = icmp eq ptr %_221.i.i4443, %_8.i4441, !dbg !19010
  br i1 %_12.i.i4444, label %bb6.i1964, label %bb11.i.i4445, !dbg !19013

bb11.i.i4445:                                     ; preds = %bb1.i.i4442
  %_22.i.i4446 = getelementptr inbounds nuw i8, ptr %_221.i.i4443, i32 16, !dbg !19014
  %51 = getelementptr inbounds nuw i8, ptr %_221.i.i4443, i32 12, !dbg !19016
  %_3.i.i.i4447 = load i32, ptr %51, align 4, !dbg !19016, !alias.scope !19018, !noalias !19023, !noundef !10
  %52 = icmp eq i32 %_3.i.i.i4447, 0, !dbg !19016
  %_51.i.i.i4448 = load i32, ptr %_221.i.i4443, align 4, !dbg !19016, !alias.scope !19018, !noalias !19023
  %53 = getelementptr inbounds nuw i8, ptr %_221.i.i4443, i32 4, !dbg !19016
  %_72.i.i.i4449 = load i32, ptr %53, align 4, !dbg !19016, !alias.scope !19018, !noalias !19023
  %54 = icmp eq i32 %_51.i.i.i4448, %_72.i.i.i4449, !dbg !19016
  %_0.sroa.0.0.off0.i.i.i4450 = select i1 %52, i1 %54, i1 false, !dbg !19016
  br i1 %_0.sroa.0.0.off0.i.i.i4450, label %bb1.i.i4442, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, !dbg !19026

bb6.i1964:                                        ; preds = %bb1.i.i4442
  %55 = getelementptr inbounds nuw i8, ptr %self, i32 496, !dbg !19027
  %_17.0.i = load ptr, ptr %55, align 4, !dbg !19027, !alias.scope !18942, !noalias !19002, !nonnull !10, !noundef !10
  %56 = getelementptr inbounds nuw i8, ptr %self, i32 500, !dbg !19027
  %_17.1.i = load i32, ptr %56, align 4, !dbg !19027, !alias.scope !18942, !noalias !19002, !noundef !10
  %_8.i4452 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_17.0.i, i32 %_17.1.i, !dbg !19028
  br label %bb1.i.i4453, !dbg !19033

bb1.i.i4453:                                      ; preds = %bb11.i.i4456, %bb6.i1964
  %_221.i.i4454 = phi ptr [ %_22.i.i4457, %bb11.i.i4456 ], [ %_17.0.i, %bb6.i1964 ]
  %_12.i.i4455 = icmp eq ptr %_221.i.i4454, %_8.i4452, !dbg !19035
  br i1 %_12.i.i4455, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, label %bb11.i.i4456, !dbg !19038

bb11.i.i4456:                                     ; preds = %bb1.i.i4453
  %_22.i.i4457 = getelementptr inbounds nuw i8, ptr %_221.i.i4454, i32 16, !dbg !19039
  %57 = getelementptr inbounds nuw i8, ptr %_221.i.i4454, i32 12, !dbg !19041
  %_3.i.i.i4458 = load i32, ptr %57, align 4, !dbg !19041, !alias.scope !19043, !noalias !19048, !noundef !10
  %58 = icmp eq i32 %_3.i.i.i4458, 0, !dbg !19041
  %_51.i.i.i4459 = load i32, ptr %_221.i.i4454, align 4, !dbg !19041, !alias.scope !19043, !noalias !19048
  %59 = getelementptr inbounds nuw i8, ptr %_221.i.i4454, i32 4, !dbg !19041
  %_72.i.i.i4460 = load i32, ptr %59, align 4, !dbg !19041, !alias.scope !19043, !noalias !19048
  %60 = icmp eq i32 %_51.i.i.i4459, %_72.i.i.i4460, !dbg !19041
  %_0.sroa.0.0.off0.i.i.i4461 = select i1 %58, i1 %60, i1 false, !dbg !19041
  br i1 %_0.sroa.0.0.off0.i.i.i4461, label %bb1.i.i4453, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, !dbg !19051

_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit: ; preds = %bb11.i.i4423, %bb11.i.i4434, %bb11.i.i4445, %bb11.i.i4456, %bb1.i.i4453
  %_0.sroa.0.0.off0.i = phi i1 [ false, %bb11.i.i4445 ], [ false, %bb11.i.i4434 ], [ false, %bb11.i.i4456 ], [ true, %bb1.i.i4453 ], [ false, %bb11.i.i4423 ]
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19052), !dbg !19055
  %61 = getelementptr inbounds nuw i8, ptr %self, i32 412, !dbg !19057
  %_31.0.i = load ptr, ptr %61, align 4, !dbg !19057, !alias.scope !19052, !noalias !19059, !nonnull !10, !noundef !10
  %62 = getelementptr inbounds nuw i8, ptr %self, i32 416, !dbg !19057
  %_31.1.i = load i32, ptr %62, align 4, !dbg !19057, !alias.scope !19052, !noalias !19059, !noundef !10
  %_17.idx.i = mul nuw nsw i32 %_31.1.i, 12, !dbg !19060
  %_17.i = getelementptr inbounds nuw i8, ptr %_31.0.i, i32 %_17.idx.i, !dbg !19060
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19064), !dbg !19067, !noalias !19059
  %_5.not.i.i.i = icmp eq i32 %_31.1.i, 0
  %63 = getelementptr inbounds nuw i8, ptr %_31.0.i, i32 4
  %64 = getelementptr inbounds nuw i8, ptr %_31.0.i, i32 8
  br i1 %_5.not.i.i.i, label %bb2.i4471, label %bb1.i.i4463

bb1.i.i4463:                                      ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i
  %_224.i.i = phi ptr [ %_22.i.i4466, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i ], [ %_31.0.i, %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit ]
  %_12.i.i4464 = icmp eq ptr %_224.i.i, %_17.i, !dbg !19068
  br i1 %_12.i.i4464, label %bb2.i4471, label %bb11.i.i4465, !dbg !19072

bb11.i.i4465:                                     ; preds = %bb1.i.i4463
  %_22.i.i4466 = getelementptr inbounds nuw i8, ptr %_224.i.i, i32 12, !dbg !19073
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19075), !dbg !19078, !noalias !19059
  %_9.i.i.i = load i32, ptr %_224.i.i, align 4, !dbg !19079, !alias.scope !19075, !noalias !19082, !noundef !10
  %_10.i.i.i = load i32, ptr %_31.0.i, align 4, !dbg !19079, !alias.scope !19064, !noalias !19084, !noundef !10
  %_8.i.i.i = icmp eq i32 %_9.i.i.i, %_10.i.i.i, !dbg !19079
  br i1 %_8.i.i.i, label %bb2.i.i.i, label %bb10.i, !dbg !19079

bb2.i.i.i:                                        ; preds = %bb11.i.i4465
  %65 = getelementptr inbounds nuw i8, ptr %_224.i.i, i32 4, !dbg !19079
  %_12.i.i.i = load i32, ptr %65, align 4, !dbg !19079, !alias.scope !19075, !noalias !19082, !noundef !10
  %_13.i.i.i = load i32, ptr %63, align 4, !dbg !19079, !alias.scope !19064, !noalias !19084, !noundef !10
  %_11.i.i.i = icmp eq i32 %_12.i.i.i, %_13.i.i.i, !dbg !19079
  br i1 %_11.i.i.i, label %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, label %bb10.i, !dbg !19079

_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i: ; preds = %bb2.i.i.i
  %66 = getelementptr inbounds nuw i8, ptr %_224.i.i, i32 8, !dbg !19079
  %_14.i.i.i4469 = load i32, ptr %66, align 4, !dbg !19079, !alias.scope !19075, !noalias !19082, !noundef !10
  %_15.i.i.i4470 = load i32, ptr %64, align 4, !dbg !19079, !alias.scope !19064, !noalias !19084, !noundef !10
  %67 = icmp eq i32 %_14.i.i.i4469, %_15.i.i.i4470, !dbg !19079
  br i1 %67, label %bb1.i.i4463, label %bb10.i, !dbg !19078

bb2.i4471:                                        ; preds = %bb1.i.i4463, %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit
  %68 = getelementptr inbounds nuw i8, ptr %self, i32 380, !dbg !19085
  %_32.0.i = load ptr, ptr %68, align 4, !dbg !19085, !alias.scope !19052, !noalias !19059, !nonnull !10, !noundef !10
  %69 = getelementptr inbounds nuw i8, ptr %self, i32 384, !dbg !19085
  %_32.1.i = load i32, ptr %69, align 4, !dbg !19085, !alias.scope !19052, !noalias !19059, !noundef !10
  %_26.idx.i = shl nuw nsw i32 %_32.1.i, 2, !dbg !19086
  %_26.i = getelementptr inbounds nuw i8, ptr %_32.0.i, i32 %_26.idx.i, !dbg !19086
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19090), !dbg !19093, !noalias !19059
  %_6.not.i.i.i = icmp eq i32 %_32.1.i, 0
  br i1 %_6.not.i.i.i, label %bb3.i, label %bb1.i3.i

bb1.i3.i:                                         ; preds = %bb2.i4471, %bb11.i5.i
  %_223.i.i = phi ptr [ %_22.i6.i, %bb11.i5.i ], [ %_32.0.i, %bb2.i4471 ]
  %_12.i4.i = icmp eq ptr %_223.i.i, %_26.i, !dbg !19094
  br i1 %_12.i4.i, label %bb3.i, label %bb11.i5.i, !dbg !19098

bb11.i5.i:                                        ; preds = %bb1.i3.i
  %_22.i6.i = getelementptr inbounds nuw i8, ptr %_223.i.i, i32 4, !dbg !19099
  %ptr.val.i.i = load i32, ptr %_223.i.i, align 4, !dbg !19101, !noalias !19102
  %_4.i.i.i4472 = load i32, ptr %_32.0.i, align 4, !dbg !19104, !alias.scope !19090, !noalias !19106, !noundef !10
  %_0.i.i.i = icmp eq i32 %ptr.val.i.i, %_4.i.i.i4472, !dbg !19107
  br i1 %_0.i.i.i, label %bb1.i3.i, label %bb10.i, !dbg !19101

bb3.i:                                            ; preds = %bb1.i3.i, %bb2.i4471
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19108), !dbg !19111
  %70 = getelementptr inbounds nuw i8, ptr %self, i32 512, !dbg !19112
  %_31.0.i4473 = load ptr, ptr %70, align 4, !dbg !19112, !alias.scope !19108, !noalias !19059, !nonnull !10, !noundef !10
  %71 = getelementptr inbounds nuw i8, ptr %self, i32 516, !dbg !19112
  %_31.1.i4474 = load i32, ptr %71, align 4, !dbg !19112, !alias.scope !19108, !noalias !19059, !noundef !10
  %_17.idx.i4475 = mul nuw nsw i32 %_31.1.i4474, 12, !dbg !19114
  %_17.i4476 = getelementptr inbounds nuw i8, ptr %_31.0.i4473, i32 %_17.idx.i4475, !dbg !19114
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19118), !dbg !19121, !noalias !19059
  %_5.not.i.i.i4477 = icmp eq i32 %_31.1.i4474, 0
  %72 = getelementptr inbounds nuw i8, ptr %_31.0.i4473, i32 4
  %73 = getelementptr inbounds nuw i8, ptr %_31.0.i4473, i32 8
  br i1 %_5.not.i.i.i4477, label %bb2.i4495, label %bb1.i.i4478

bb1.i.i4478:                                      ; preds = %bb3.i, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i4492
  %_224.i.i4479 = phi ptr [ %_22.i.i4482, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i4492 ], [ %_31.0.i4473, %bb3.i ]
  %_12.i.i4480 = icmp eq ptr %_224.i.i4479, %_17.i4476, !dbg !19122
  br i1 %_12.i.i4480, label %bb2.i4495, label %bb11.i.i4481, !dbg !19126

bb11.i.i4481:                                     ; preds = %bb1.i.i4478
  %_22.i.i4482 = getelementptr inbounds nuw i8, ptr %_224.i.i4479, i32 12, !dbg !19127
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19129), !dbg !19132, !noalias !19059
  %_9.i.i.i4483 = load i32, ptr %_224.i.i4479, align 4, !dbg !19133, !alias.scope !19129, !noalias !19136, !noundef !10
  %_10.i.i.i4484 = load i32, ptr %_31.0.i4473, align 4, !dbg !19133, !alias.scope !19118, !noalias !19138, !noundef !10
  %_8.i.i.i4485 = icmp eq i32 %_9.i.i.i4483, %_10.i.i.i4484, !dbg !19133
  br i1 %_8.i.i.i4485, label %bb2.i.i.i4488, label %bb10.i, !dbg !19133

bb2.i.i.i4488:                                    ; preds = %bb11.i.i4481
  %74 = getelementptr inbounds nuw i8, ptr %_224.i.i4479, i32 4, !dbg !19133
  %_12.i.i.i4489 = load i32, ptr %74, align 4, !dbg !19133, !alias.scope !19129, !noalias !19136, !noundef !10
  %_13.i.i.i4490 = load i32, ptr %72, align 4, !dbg !19133, !alias.scope !19118, !noalias !19138, !noundef !10
  %_11.i.i.i4491 = icmp eq i32 %_12.i.i.i4489, %_13.i.i.i4490, !dbg !19133
  br i1 %_11.i.i.i4491, label %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i4492, label %bb10.i, !dbg !19133

_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i4492: ; preds = %bb2.i.i.i4488
  %75 = getelementptr inbounds nuw i8, ptr %_224.i.i4479, i32 8, !dbg !19133
  %_14.i.i.i4493 = load i32, ptr %75, align 4, !dbg !19133, !alias.scope !19129, !noalias !19136, !noundef !10
  %_15.i.i.i4494 = load i32, ptr %73, align 4, !dbg !19133, !alias.scope !19118, !noalias !19138, !noundef !10
  %76 = icmp eq i32 %_14.i.i.i4493, %_15.i.i.i4494, !dbg !19133
  br i1 %76, label %bb1.i.i4478, label %bb10.i, !dbg !19132

bb2.i4495:                                        ; preds = %bb1.i.i4478, %bb3.i
  %77 = getelementptr inbounds nuw i8, ptr %self, i32 480, !dbg !19139
  %_32.0.i4496 = load ptr, ptr %77, align 4, !dbg !19139, !alias.scope !19108, !noalias !19059, !nonnull !10, !noundef !10
  %78 = getelementptr inbounds nuw i8, ptr %self, i32 484, !dbg !19139
  %_32.1.i4497 = load i32, ptr %78, align 4, !dbg !19139, !alias.scope !19108, !noalias !19059, !noundef !10
  %_26.idx.i4498 = shl nuw nsw i32 %_32.1.i4497, 2, !dbg !19140
  %_26.i4499 = getelementptr inbounds nuw i8, ptr %_32.0.i4496, i32 %_26.idx.i4498, !dbg !19140
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19144), !dbg !19147, !noalias !19059
  %_6.not.i.i.i4500 = icmp eq i32 %_32.1.i4497, 0
  br i1 %_6.not.i.i.i4500, label %bb5.i, label %bb1.i3.i4501

bb1.i3.i4501:                                     ; preds = %bb2.i4495, %bb11.i5.i4504
  %_223.i.i4502 = phi ptr [ %_22.i6.i4505, %bb11.i5.i4504 ], [ %_32.0.i4496, %bb2.i4495 ]
  %_12.i4.i4503 = icmp eq ptr %_223.i.i4502, %_26.i4499, !dbg !19148
  br i1 %_12.i4.i4503, label %bb5.i, label %bb11.i5.i4504, !dbg !19152

bb11.i5.i4504:                                    ; preds = %bb1.i3.i4501
  %_22.i6.i4505 = getelementptr inbounds nuw i8, ptr %_223.i.i4502, i32 4, !dbg !19153
  %ptr.val.i.i4506 = load i32, ptr %_223.i.i4502, align 4, !dbg !19155, !noalias !19156
  %_4.i.i.i4507 = load i32, ptr %_32.0.i4496, align 4, !dbg !19158, !alias.scope !19144, !noalias !19160, !noundef !10
  %_0.i.i.i4508 = icmp eq i32 %ptr.val.i.i4506, %_4.i.i.i4507, !dbg !19161
  br i1 %_0.i.i.i4508, label %bb1.i3.i4501, label %bb10.i, !dbg !19155

bb10.i:                                           ; preds = %bb11.i.i4465, %bb2.i.i.i, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, %bb11.i5.i, %bb11.i.i4481, %bb2.i.i.i4488, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i4492, %bb11.i5.i4504
  %79 = getelementptr inbounds nuw i8, ptr %self, i32 320, !dbg !19162
  %80 = getelementptr inbounds nuw i8, ptr %self, i32 321, !dbg !19162
  %81 = getelementptr inbounds nuw i8, ptr %self, i32 108, !dbg !19162
  br i1 %_0.sroa.0.0.off0.i, label %bb11.i, label %bb12.i, !dbg !19163

bb5.i:                                            ; preds = %bb1.i3.i4501, %bb2.i4495
  br i1 %_0.sroa.0.0.off0.i, label %bb6.i, label %bb7.i, !dbg !19164

bb12.i:                                           ; preds = %bb10.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19165), !dbg !19168
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19169), !dbg !19168
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19171), !dbg !19168
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19173), !dbg !19168
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i1018), !dbg !19175, !noalias !19179
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_left.i1018, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #31, !dbg !19183, !noalias !19184
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_right.i1017), !dbg !19185, !noalias !19179
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_right.i1017, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #31, !dbg !19187, !noalias !19188
  %82 = load i8, ptr %79, align 4, !dbg !19189, !range !4667, !alias.scope !19165, !noalias !19193, !noundef !10
  %83 = load i8, ptr %80, align 1, !dbg !19194, !range !4667, !alias.scope !19165, !noalias !19193, !noundef !10
  %_35.i1026 = load i32, ptr %_35, align 4, !dbg !19196, !alias.scope !19173, !noalias !19198, !noundef !10
  %_37.i1027 = load i32, ptr %81, align 4, !dbg !19199, !alias.scope !19173, !noalias !19198, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i1016), !dbg !19201, !noalias !19179
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i1016, i8 0, i32 32, i1 false), !noalias !19179
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i1015), !dbg !19203, !noalias !19179
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i1015, i8 0, i32 1024, i1 false), !noalias !19179
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i1014), !dbg !19205, !noalias !19179
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i1014, i8 0, i32 1024, i1 false), !noalias !19179
  %_32.i1022 = zext nneg i8 %82 to i32, !dbg !19189
  %.none.i1023 = sub nsw i32 0, %_32.i1022, !dbg !19207
  %_33.i1024 = zext nneg i8 %83 to i32, !dbg !19194
  %all.sroa.0.0.i1025 = sub nsw i32 0, %_33.i1024, !dbg !19194
  %_115.not.i10408145 = icmp eq i32 %frames, 0, !dbg !19208
  br i1 %_115.not.i10408145, label %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit, label %bb37.i1041.lr.ph, !dbg !19208

bb37.i1041.lr.ph:                                 ; preds = %bb12.i
  %d9.i = lshr i32 %frames, 5, !dbg !19218
  %r2.i = and i32 %frames, 31, !dbg !19225
  %_19.not.i = icmp ne i32 %r2.i, 0, !dbg !19226
  %84 = zext i1 %_19.not.i to i32, !dbg !19226
  %yield_count.sroa.0.0.i = add nuw nsw i32 %d9.i, %84, !dbg !19226
  %history.i138.i997.sroa.7.0.hot_left.i1018.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1018, i32 4
  %history.i138.i997.sroa.10.0.hot_left.i1018.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1018, i32 8
  %history.i138.i997.sroa.13.0.hot_left.i1018.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1018, i32 12
  %history.i138.i997.sroa.16.0.hot_left.i1018.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1018, i32 16
  %history.i138.i997.sroa.19.0.hot_left.i1018.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1018, i32 20
  %history.i138.i997.sroa.22.0.hot_left.i1018.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1018, i32 24
  %history.i138.i997.sroa.26.0.hot_left.i1018.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1018, i32 28
  %history.i138.i997.sroa.29.0.hot_left.i1018.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1018, i32 32
  %history.i138.i997.sroa.32.0.hot_left.i1018.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1018, i32 36
  %history.i138.i997.sroa.35.0.hot_left.i1018.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1018, i32 40
  %history.i138.i997.sroa.38.0.hot_left.i1018.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1018, i32 44
  %85 = getelementptr inbounds nuw i8, ptr %self, i32 132
  %86 = getelementptr inbounds nuw i8, ptr %self, i32 136
  %87 = getelementptr inbounds nuw i8, ptr %self, i32 140
  %row1.i.i.i171.i1081 = getelementptr inbounds nuw i8, ptr %self, i32 144
  %88 = getelementptr inbounds nuw i8, ptr %self, i32 148
  %89 = getelementptr inbounds nuw i8, ptr %self, i32 152
  %90 = getelementptr inbounds nuw i8, ptr %self, i32 156
  %row3.i.i.i185.i1095 = getelementptr inbounds nuw i8, ptr %self, i32 160
  %91 = getelementptr inbounds nuw i8, ptr %self, i32 164
  %92 = getelementptr inbounds nuw i8, ptr %self, i32 168
  %93 = getelementptr inbounds nuw i8, ptr %self, i32 172
  %row5.i.i.i199.i1109 = getelementptr inbounds nuw i8, ptr %self, i32 176
  %94 = getelementptr inbounds nuw i8, ptr %self, i32 180
  %95 = getelementptr inbounds nuw i8, ptr %self, i32 184
  %96 = getelementptr inbounds nuw i8, ptr %self, i32 188
  %row7.i.i.i213.i1123 = getelementptr inbounds nuw i8, ptr %self, i32 192
  %97 = getelementptr inbounds nuw i8, ptr %self, i32 196
  %98 = getelementptr inbounds nuw i8, ptr %self, i32 200
  %99 = getelementptr inbounds nuw i8, ptr %self, i32 204
  %row9.i.i.i227.i1137 = getelementptr inbounds nuw i8, ptr %self, i32 208
  %100 = getelementptr inbounds nuw i8, ptr %self, i32 212
  %101 = getelementptr inbounds nuw i8, ptr %self, i32 216
  %102 = getelementptr inbounds nuw i8, ptr %self, i32 220
  %row11.i.i.i241.i1151 = getelementptr inbounds nuw i8, ptr %self, i32 224
  %103 = getelementptr inbounds nuw i8, ptr %self, i32 228
  %104 = getelementptr inbounds nuw i8, ptr %self, i32 232
  %105 = getelementptr inbounds nuw i8, ptr %self, i32 236
  %row13.i.i.i255.i1165 = getelementptr inbounds nuw i8, ptr %self, i32 240
  %106 = getelementptr inbounds nuw i8, ptr %self, i32 244
  %107 = getelementptr inbounds nuw i8, ptr %self, i32 248
  %108 = getelementptr inbounds nuw i8, ptr %self, i32 252
  %row15.i.i.i269.i1179 = getelementptr inbounds nuw i8, ptr %self, i32 256
  %109 = getelementptr inbounds nuw i8, ptr %self, i32 260
  %110 = getelementptr inbounds nuw i8, ptr %self, i32 264
  %111 = getelementptr inbounds nuw i8, ptr %self, i32 268
  %row17.i.i.i283.i1193 = getelementptr inbounds nuw i8, ptr %self, i32 272
  %112 = getelementptr inbounds nuw i8, ptr %self, i32 276
  %113 = getelementptr inbounds nuw i8, ptr %self, i32 280
  %114 = getelementptr inbounds nuw i8, ptr %self, i32 284
  %row19.i.i.i297.i1207 = getelementptr inbounds nuw i8, ptr %self, i32 288
  %115 = getelementptr inbounds nuw i8, ptr %self, i32 292
  %116 = getelementptr inbounds nuw i8, ptr %self, i32 296
  %117 = getelementptr inbounds nuw i8, ptr %self, i32 300
  %row21.i.i.i311.i1221 = getelementptr inbounds nuw i8, ptr %self, i32 304
  %118 = getelementptr inbounds nuw i8, ptr %self, i32 308
  %119 = getelementptr inbounds nuw i8, ptr %self, i32 312
  %120 = getelementptr inbounds nuw i8, ptr %self, i32 316
  %history.i.i1011.sroa.7.0.hot_right.i1017.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1017, i32 4
  %history.i.i1011.sroa.10.0.hot_right.i1017.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1017, i32 8
  %history.i.i1011.sroa.13.0.hot_right.i1017.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1017, i32 12
  %history.i.i1011.sroa.16.0.hot_right.i1017.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1017, i32 16
  %history.i.i1011.sroa.19.0.hot_right.i1017.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1017, i32 20
  %history.i.i1011.sroa.22.0.hot_right.i1017.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1017, i32 24
  %history.i.i1011.sroa.26.0.hot_right.i1017.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1017, i32 28
  %history.i.i1011.sroa.29.0.hot_right.i1017.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1017, i32 32
  %history.i.i1011.sroa.32.0.hot_right.i1017.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1017, i32 36
  %history.i.i1011.sroa.35.0.hot_right.i1017.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1017, i32 40
  %history.i.i1011.sroa.38.0.hot_right.i1017.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1017, i32 44
  %_68.i1449 = getelementptr inbounds nuw i8, ptr %hot_left.i1018, i32 48
  %_69.i1450 = getelementptr inbounds nuw i8, ptr %hot_left.i1018, i32 64
  %121 = getelementptr inbounds nuw i8, ptr %hot_left.i1018, i32 60
  %122 = getelementptr inbounds nuw i8, ptr %hot_left.i1018, i32 56
  %123 = getelementptr inbounds nuw i8, ptr %hot_left.i1018, i32 52
  %124 = getelementptr inbounds nuw i8, ptr %hot_left.i1018, i32 76
  %125 = getelementptr inbounds nuw i8, ptr %hot_left.i1018, i32 72
  %126 = getelementptr inbounds nuw i8, ptr %hot_left.i1018, i32 68
  %_73.i1451 = getelementptr inbounds nuw i8, ptr %hot_right.i1017, i32 48
  %_74.i1452 = getelementptr inbounds nuw i8, ptr %hot_right.i1017, i32 64
  %127 = getelementptr inbounds nuw i8, ptr %hot_right.i1017, i32 60
  %128 = getelementptr inbounds nuw i8, ptr %hot_right.i1017, i32 56
  %129 = getelementptr inbounds nuw i8, ptr %hot_right.i1017, i32 52
  %130 = getelementptr inbounds nuw i8, ptr %hot_right.i1017, i32 76
  %131 = getelementptr inbounds nuw i8, ptr %hot_right.i1017, i32 72
  %132 = getelementptr inbounds nuw i8, ptr %hot_right.i1017, i32 68
  %_9.i3890 = add nsw i32 %_32.i1022, -1
  %133 = getelementptr inbounds nuw i8, ptr %self, i32 528
  %134 = getelementptr inbounds nuw i8, ptr %self, i32 420
  %135 = getelementptr inbounds nuw i8, ptr %self, i32 344
  %136 = getelementptr inbounds nuw i8, ptr %self, i32 340
  %137 = getelementptr inbounds nuw i8, ptr %self, i32 384
  %138 = getelementptr inbounds nuw i8, ptr %self, i32 380
  %139 = getelementptr inbounds nuw i8, ptr %self, i32 368
  %140 = getelementptr inbounds nuw i8, ptr %self, i32 364
  %141 = getelementptr inbounds nuw i8, ptr %self, i32 352
  %142 = getelementptr inbounds nuw i8, ptr %self, i32 348
  %143 = getelementptr inbounds nuw i8, ptr %hot_left.i1018, i32 84
  %144 = getelementptr inbounds nuw i8, ptr %hot_left.i1018, i32 88
  %145 = getelementptr inbounds nuw i8, ptr %hot_left.i1018, i32 80
  %146 = getelementptr inbounds nuw i8, ptr %self, i32 336
  %147 = getelementptr inbounds nuw i8, ptr %self, i32 332
  %_9.i3870 = add nsw i32 %_33.i1024, -1
  %148 = getelementptr inbounds nuw i8, ptr %self, i32 520
  %149 = getelementptr inbounds nuw i8, ptr %self, i32 444
  %150 = getelementptr inbounds nuw i8, ptr %self, i32 440
  %151 = getelementptr inbounds nuw i8, ptr %self, i32 516
  %152 = getelementptr inbounds nuw i8, ptr %self, i32 512
  %153 = getelementptr inbounds nuw i8, ptr %self, i32 484
  %154 = getelementptr inbounds nuw i8, ptr %self, i32 480
  %155 = getelementptr inbounds nuw i8, ptr %self, i32 468
  %156 = getelementptr inbounds nuw i8, ptr %self, i32 464
  %157 = getelementptr inbounds nuw i8, ptr %self, i32 452
  %158 = getelementptr inbounds nuw i8, ptr %self, i32 448
  %159 = getelementptr inbounds nuw i8, ptr %hot_right.i1017, i32 84
  %160 = getelementptr inbounds nuw i8, ptr %hot_right.i1017, i32 88
  %161 = getelementptr inbounds nuw i8, ptr %hot_right.i1017, i32 80
  %162 = getelementptr inbounds nuw i8, ptr %self, i32 436
  %163 = getelementptr inbounds nuw i8, ptr %self, i32 432
  %164 = getelementptr inbounds nuw i8, ptr %self, i32 532
  %iter.sroa.0.0.ptr.i54.i14917019.1 = getelementptr inbounds nuw i8, ptr %scratch.i1016, i32 4
  %iter.sroa.0.0.ptr.i54.i14917019.2 = getelementptr inbounds nuw i8, ptr %scratch.i1016, i32 8
  %iter.sroa.0.0.ptr.i54.i14917019.3 = getelementptr inbounds nuw i8, ptr %scratch.i1016, i32 12
  %iter.sroa.0.0.ptr.i54.i14917019.4 = getelementptr inbounds nuw i8, ptr %scratch.i1016, i32 16
  %iter.sroa.0.0.ptr.i54.i14917019.5 = getelementptr inbounds nuw i8, ptr %scratch.i1016, i32 20
  %iter.sroa.0.0.ptr.i54.i14917019.6 = getelementptr inbounds nuw i8, ptr %scratch.i1016, i32 24
  %iter.sroa.0.0.ptr.i54.i14917019.7 = getelementptr inbounds nuw i8, ptr %scratch.i1016, i32 28
  %iter.sroa.0.0.ptr.i.i15847031.1 = getelementptr inbounds nuw i8, ptr %scratch.i1016, i32 4
  %iter.sroa.0.0.ptr.i.i15847031.2 = getelementptr inbounds nuw i8, ptr %scratch.i1016, i32 8
  %iter.sroa.0.0.ptr.i.i15847031.3 = getelementptr inbounds nuw i8, ptr %scratch.i1016, i32 12
  %iter.sroa.0.0.ptr.i.i15847031.4 = getelementptr inbounds nuw i8, ptr %scratch.i1016, i32 16
  %iter.sroa.0.0.ptr.i.i15847031.5 = getelementptr inbounds nuw i8, ptr %scratch.i1016, i32 20
  %iter.sroa.0.0.ptr.i.i15847031.6 = getelementptr inbounds nuw i8, ptr %scratch.i1016, i32 24
  %iter.sroa.0.0.ptr.i.i15847031.7 = getelementptr inbounds nuw i8, ptr %scratch.i1016, i32 28
  br label %bb37.i1041, !dbg !19208

bb16.i1442.bb13.i1035.loopexit_crit_edge:         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit
  store float %_0.i.i4043, ptr %121, align 4, !dbg !19227, !alias.scope !19240, !noalias !19243
  store float %_0.i3815, ptr %_68.i1449, align 4, !dbg !19246, !alias.scope !19240, !noalias !19243
  store float %_0.i3808, ptr %122, align 4, !dbg !19248, !alias.scope !19240, !noalias !19243
  store float %_0.i.i4050, ptr %124, align 4, !dbg !19249, !alias.scope !19251, !noalias !19254
  store float %_0.i3828, ptr %_69.i1450, align 4, !dbg !19255, !alias.scope !19251, !noalias !19254
  store float %_0.i3821, ptr %125, align 4, !dbg !19256, !alias.scope !19251, !noalias !19254
  store float %_0.i.i4057, ptr %127, align 4, !dbg !19257, !alias.scope !19261, !noalias !19264
  store float %_0.i3841, ptr %_73.i1451, align 4, !dbg !19267, !alias.scope !19261, !noalias !19264
  store float %_0.i3834, ptr %128, align 4, !dbg !19268, !alias.scope !19261, !noalias !19264
  store float %_0.i.i4064, ptr %130, align 4, !dbg !19269, !alias.scope !19271, !noalias !19254
  store float %_0.i3854, ptr %_74.i1452, align 4, !dbg !19274, !alias.scope !19271, !noalias !19254
  store float %_0.i3847, ptr %131, align 4, !dbg !19275, !alias.scope !19271, !noalias !19254
  store float %_0.i3354, ptr %143, align 4, !dbg !19276
  store float %_0.i3728, ptr %145, align 4, !dbg !19291
  store float %_0.i3350, ptr %159, align 4, !dbg !19294
  store float %_0.i3724, ptr %161, align 4, !dbg !19296
  br label %bb13.i1035.loopexit, !dbg !19297

bb13.i1035.loopexit:                              ; preds = %bb16.i1442.bb13.i1035.loopexit_crit_edge, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i1441
  %ring_cursor.sroa.0.1.i1444.lcssa = phi i32 [ %spec.store.select12.i1654, %bb16.i1442.bb13.i1035.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i10388148, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i1441 ], !dbg !19303
  %main_cursor.sroa.0.1.i1445.lcssa = phi i32 [ %spec.store.select11.i1652, %bb16.i1442.bb13.i1035.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i10398149, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i1441 ], !dbg !19304
  %_115.not.i1040 = icmp eq i32 %167, 0, !dbg !19208
  %indvars.iv.next = add i32 %indvars.iv, -32, !dbg !19208
  br i1 %_115.not.i1040, label %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit, label %bb37.i1041, !dbg !19208

bb37.i1041:                                       ; preds = %bb37.i1041.lr.ph, %bb13.i1035.loopexit
  %indvars.iv = phi i32 [ %frames, %bb37.i1041.lr.ph ], [ %indvars.iv.next, %bb13.i1035.loopexit ]
  %main_cursor.sroa.0.0.i10398149 = phi i32 [ %_35.i1026, %bb37.i1041.lr.ph ], [ %main_cursor.sroa.0.1.i1445.lcssa, %bb13.i1035.loopexit ]
  %ring_cursor.sroa.0.0.i10388148 = phi i32 [ %_37.i1027, %bb37.i1041.lr.ph ], [ %ring_cursor.sroa.0.1.i1444.lcssa, %bb13.i1035.loopexit ]
  %iter2.sroa.0.0.i10378147 = phi i32 [ %yield_count.sroa.0.0.i, %bb37.i1041.lr.ph ], [ %167, %bb13.i1035.loopexit ]
  %iter.sroa.0.0.i10368146 = phi i32 [ 0, %bb37.i1041.lr.ph ], [ %166, %bb13.i1035.loopexit ]
  %165 = call i32 @llvm.umax.i32(i32 %indvars.iv, i32 1), !dbg !19305
  %umax12505 = call i32 @llvm.umin.i32(i32 %165, i32 32), !dbg !19305
  %166 = add i32 %iter.sroa.0.0.i10368146, 32, !dbg !19305
  %167 = add nsw i32 %iter2.sroa.0.0.i10378147, -1, !dbg !19309
  %168 = sub i32 %frames, %iter.sroa.0.0.i10368146, !dbg !19310
  %spec.store.select.i1042 = tail call i32 @llvm.umin.i32(i32 %168, i32 32), !dbg !19311
  %_51.i1043 = add i32 %spec.store.select.i1042, %iter.sroa.0.0.i10368146, !dbg !19316
  %_125.i1044 = icmp ult i32 %_51.i1043, %iter.sroa.0.0.i10368146, !dbg !19317
  %_119.not.i1045 = icmp ugt i32 %_51.i1043, %left_io.1
  %or.cond.i1046 = or i1 %_125.i1044, %_119.not.i1045, !dbg !19317
  br i1 %or.cond.i1046, label %bb43.i1668, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit, !dbg !19317, !prof !4596

bb43.i1668:                                       ; preds = %bb37.i1041
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %iter.sroa.0.0.i10368146, i32 noundef %_51.i1043, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_56e6f2edba81ce75fd22cd243776a278) #32, !dbg !19324, !noalias !19254
  unreachable, !dbg !19324

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit: ; preds = %bb37.i1041
  %_128.i1048 = getelementptr inbounds nuw float, ptr %left_io.0, i32 %iter.sroa.0.0.i10368146, !dbg !19325
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19329), !dbg !19332
  %history.i138.i997.sroa.0.0.copyload = load float, ptr %hot_left.i1018, align 4, !dbg !19333, !noalias !19337
  %history.i138.i997.sroa.7.0.copyload = load float, ptr %history.i138.i997.sroa.7.0.hot_left.i1018.sroa_idx, align 4, !dbg !19333, !noalias !19337
  %history.i138.i997.sroa.10.0.copyload = load float, ptr %history.i138.i997.sroa.10.0.hot_left.i1018.sroa_idx, align 4, !dbg !19333, !noalias !19337
  %history.i138.i997.sroa.13.0.copyload = load float, ptr %history.i138.i997.sroa.13.0.hot_left.i1018.sroa_idx, align 4, !dbg !19333, !noalias !19337
  %history.i138.i997.sroa.16.0.copyload = load float, ptr %history.i138.i997.sroa.16.0.hot_left.i1018.sroa_idx, align 4, !dbg !19333, !noalias !19337
  %history.i138.i997.sroa.19.0.copyload = load float, ptr %history.i138.i997.sroa.19.0.hot_left.i1018.sroa_idx, align 4, !dbg !19333, !noalias !19337
  %history.i138.i997.sroa.22.0.copyload = load float, ptr %history.i138.i997.sroa.22.0.hot_left.i1018.sroa_idx, align 4, !dbg !19333, !noalias !19337
  %history.i138.i997.sroa.26.0.copyload = load float, ptr %history.i138.i997.sroa.26.0.hot_left.i1018.sroa_idx, align 4, !dbg !19333, !noalias !19337
  %history.i138.i997.sroa.29.0.copyload = load float, ptr %history.i138.i997.sroa.29.0.hot_left.i1018.sroa_idx, align 4, !dbg !19333, !noalias !19337
  %history.i138.i997.sroa.32.0.copyload = load float, ptr %history.i138.i997.sroa.32.0.hot_left.i1018.sroa_idx, align 4, !dbg !19333, !noalias !19337
  %history.i138.i997.sroa.35.0.copyload = load float, ptr %history.i138.i997.sroa.35.0.hot_left.i1018.sroa_idx, align 4, !dbg !19333, !noalias !19337
  %history.i138.i997.sroa.38.0.copyload = load float, ptr %history.i138.i997.sroa.38.0.hot_left.i1018.sroa_idx, align 4, !dbg !19333, !noalias !19337
  %_2.i6957.not = icmp eq i32 %frames, %iter.sroa.0.0.i10368146, !dbg !19340
  br i1 %_2.i6957.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit333.i1243, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471.lr.ph, !dbg !19340

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471.lr.ph: ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit
  %_11.i.i.i159.i1069 = load float, ptr %_31, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_14.i.i.i162.i1072 = load float, ptr %85, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_17.i.i.i165.i1075 = load float, ptr %86, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_20.i.i.i168.i1078 = load float, ptr %87, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_25.i.i.i173.i1083 = load float, ptr %row1.i.i.i171.i1081, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_28.i.i.i176.i1086 = load float, ptr %88, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_31.i.i.i179.i1089 = load float, ptr %89, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_34.i.i.i182.i1092 = load float, ptr %90, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_39.i.i.i187.i1097 = load float, ptr %row3.i.i.i185.i1095, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_42.i.i.i190.i1100 = load float, ptr %91, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_45.i.i.i193.i1103 = load float, ptr %92, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_48.i.i.i196.i1106 = load float, ptr %93, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_53.i.i.i201.i1111 = load float, ptr %row5.i.i.i199.i1109, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_56.i.i.i204.i1114 = load float, ptr %94, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_59.i.i.i207.i1117 = load float, ptr %95, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_62.i.i.i210.i1120 = load float, ptr %96, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_67.i.i.i215.i1125 = load float, ptr %row7.i.i.i213.i1123, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_70.i.i.i218.i1128 = load float, ptr %97, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_73.i.i.i221.i1131 = load float, ptr %98, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_76.i.i.i224.i1134 = load float, ptr %99, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_81.i.i.i229.i1139 = load float, ptr %row9.i.i.i227.i1137, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_84.i.i.i232.i1142 = load float, ptr %100, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_87.i.i.i235.i1145 = load float, ptr %101, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_90.i.i.i238.i1148 = load float, ptr %102, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_95.i.i.i243.i1153 = load float, ptr %row11.i.i.i241.i1151, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_98.i.i.i246.i1156 = load float, ptr %103, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_101.i.i.i249.i1159 = load float, ptr %104, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_104.i.i.i252.i1162 = load float, ptr %105, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_109.i.i.i257.i1167 = load float, ptr %row13.i.i.i255.i1165, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_112.i.i.i260.i1170 = load float, ptr %106, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_115.i.i.i263.i1173 = load float, ptr %107, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_118.i.i.i266.i1176 = load float, ptr %108, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_123.i.i.i271.i1181 = load float, ptr %row15.i.i.i269.i1179, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_126.i.i.i274.i1184 = load float, ptr %109, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_129.i.i.i277.i1187 = load float, ptr %110, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_132.i.i.i280.i1190 = load float, ptr %111, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_137.i.i.i285.i1195 = load float, ptr %row17.i.i.i283.i1193, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_140.i.i.i288.i1198 = load float, ptr %112, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_143.i.i.i291.i1201 = load float, ptr %113, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_146.i.i.i294.i1204 = load float, ptr %114, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_151.i.i.i299.i1209 = load float, ptr %row19.i.i.i297.i1207, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_154.i.i.i302.i1212 = load float, ptr %115, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_157.i.i.i305.i1215 = load float, ptr %116, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_160.i.i.i308.i1218 = load float, ptr %117, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_165.i.i.i313.i1223 = load float, ptr %row21.i.i.i311.i1221, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_168.i.i.i316.i1226 = load float, ptr %118, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_171.i.i.i319.i1229 = load float, ptr %119, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  %_174.i.i.i322.i1232 = load float, ptr %120, align 4, !alias.scope !19347, !noalias !19352, !noundef !10
  br label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471, !dbg !19340

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471
  %iter.i134.i993.sroa.16.06969 = phi i32 [ 0, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471.lr.ph ], [ %174, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471 ]
  %history.i138.i997.sroa.35.06968 = phi float [ %history.i138.i997.sroa.35.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471.lr.ph ], [ %history.i138.i997.sroa.32.06967, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471 ]
  %history.i138.i997.sroa.32.06967 = phi float [ %history.i138.i997.sroa.32.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471.lr.ph ], [ %history.i138.i997.sroa.29.06966, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471 ]
  %history.i138.i997.sroa.29.06966 = phi float [ %history.i138.i997.sroa.29.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471.lr.ph ], [ %history.i138.i997.sroa.26.06965, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471 ]
  %history.i138.i997.sroa.26.06965 = phi float [ %history.i138.i997.sroa.26.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471.lr.ph ], [ %history.i138.i997.sroa.22.06964, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471 ]
  %history.i138.i997.sroa.22.06964 = phi float [ %history.i138.i997.sroa.22.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471.lr.ph ], [ %history.i138.i997.sroa.19.06963, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471 ]
  %history.i138.i997.sroa.19.06963 = phi float [ %history.i138.i997.sroa.19.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471.lr.ph ], [ %history.i138.i997.sroa.16.06962, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471 ]
  %history.i138.i997.sroa.16.06962 = phi float [ %history.i138.i997.sroa.16.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471.lr.ph ], [ %history.i138.i997.sroa.13.06961, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471 ]
  %history.i138.i997.sroa.13.06961 = phi float [ %history.i138.i997.sroa.13.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471.lr.ph ], [ %history.i138.i997.sroa.10.06960, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471 ]
  %history.i138.i997.sroa.10.06960 = phi float [ %history.i138.i997.sroa.10.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471.lr.ph ], [ %history.i138.i997.sroa.7.06959, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471 ]
  %history.i138.i997.sroa.7.06959 = phi float [ %history.i138.i997.sroa.7.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471.lr.ph ], [ %history.i138.i997.sroa.0.06958, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471 ]
  %history.i138.i997.sroa.0.06958 = phi float [ %history.i138.i997.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471.lr.ph ], [ %_0.i3469, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471 ]
  %data.i.i4516 = getelementptr inbounds nuw float, ptr %_128.i1048, i32 %iter.i134.i993.sroa.16.06969, !dbg !19357
  %_0.i3469 = load float, ptr %data.i.i4516, align 4, !dbg !19360, !alias.scope !19363, !noalias !19366, !noundef !10
  %169 = tail call noundef float @llvm.fabs.f32(float %history.i138.i997.sroa.19.06963), !dbg !19367
  %_0.i3032 = fmul float %_0.i3469, %_11.i.i.i159.i1069, !dbg !19373
  %_0.i2604 = fadd float %_0.i3032, 0.000000e+00, !dbg !19385
  %_0.i3031 = fmul float %_0.i3469, %_14.i.i.i162.i1072, !dbg !19388
  %_0.i2603 = fadd float %_0.i3031, 0.000000e+00, !dbg !19390
  %_0.i3030 = fmul float %_0.i3469, %_17.i.i.i165.i1075, !dbg !19392
  %_0.i2602 = fadd float %_0.i3030, 0.000000e+00, !dbg !19394
  %_0.i3029 = fmul float %_0.i3469, %_20.i.i.i168.i1078, !dbg !19396
  %_0.i2601 = fadd float %_0.i3029, 0.000000e+00, !dbg !19398
  %_0.i3028 = fmul float %history.i138.i997.sroa.0.06958, %_25.i.i.i173.i1083, !dbg !19400
  %_0.i2600 = fadd float %_0.i2604, %_0.i3028, !dbg !19404
  %_0.i3027 = fmul float %history.i138.i997.sroa.0.06958, %_28.i.i.i176.i1086, !dbg !19406
  %_0.i2599 = fadd float %_0.i2603, %_0.i3027, !dbg !19408
  %_0.i3026 = fmul float %history.i138.i997.sroa.0.06958, %_31.i.i.i179.i1089, !dbg !19410
  %_0.i2598 = fadd float %_0.i2602, %_0.i3026, !dbg !19412
  %_0.i3025 = fmul float %history.i138.i997.sroa.0.06958, %_34.i.i.i182.i1092, !dbg !19414
  %_0.i2597 = fadd float %_0.i2601, %_0.i3025, !dbg !19416
  %_0.i3024 = fmul float %history.i138.i997.sroa.7.06959, %_39.i.i.i187.i1097, !dbg !19418
  %_0.i2596 = fadd float %_0.i2600, %_0.i3024, !dbg !19422
  %_0.i3023 = fmul float %history.i138.i997.sroa.7.06959, %_42.i.i.i190.i1100, !dbg !19424
  %_0.i2595 = fadd float %_0.i2599, %_0.i3023, !dbg !19426
  %_0.i3022 = fmul float %history.i138.i997.sroa.7.06959, %_45.i.i.i193.i1103, !dbg !19428
  %_0.i2594 = fadd float %_0.i2598, %_0.i3022, !dbg !19430
  %_0.i3021 = fmul float %history.i138.i997.sroa.7.06959, %_48.i.i.i196.i1106, !dbg !19432
  %_0.i2593 = fadd float %_0.i2597, %_0.i3021, !dbg !19434
  %_0.i3020 = fmul float %history.i138.i997.sroa.10.06960, %_53.i.i.i201.i1111, !dbg !19436
  %_0.i2592 = fadd float %_0.i2596, %_0.i3020, !dbg !19440
  %_0.i3019 = fmul float %history.i138.i997.sroa.10.06960, %_56.i.i.i204.i1114, !dbg !19442
  %_0.i2591 = fadd float %_0.i2595, %_0.i3019, !dbg !19444
  %_0.i3018 = fmul float %history.i138.i997.sroa.10.06960, %_59.i.i.i207.i1117, !dbg !19446
  %_0.i2590 = fadd float %_0.i2594, %_0.i3018, !dbg !19448
  %_0.i3017 = fmul float %history.i138.i997.sroa.10.06960, %_62.i.i.i210.i1120, !dbg !19450
  %_0.i2589 = fadd float %_0.i2593, %_0.i3017, !dbg !19452
  %_0.i3016 = fmul float %history.i138.i997.sroa.13.06961, %_67.i.i.i215.i1125, !dbg !19454
  %_0.i2588 = fadd float %_0.i2592, %_0.i3016, !dbg !19458
  %_0.i3015 = fmul float %history.i138.i997.sroa.13.06961, %_70.i.i.i218.i1128, !dbg !19460
  %_0.i2587 = fadd float %_0.i2591, %_0.i3015, !dbg !19462
  %_0.i3014 = fmul float %history.i138.i997.sroa.13.06961, %_73.i.i.i221.i1131, !dbg !19464
  %_0.i2586 = fadd float %_0.i2590, %_0.i3014, !dbg !19466
  %_0.i3013 = fmul float %history.i138.i997.sroa.13.06961, %_76.i.i.i224.i1134, !dbg !19468
  %_0.i2585 = fadd float %_0.i2589, %_0.i3013, !dbg !19470
  %_0.i3012 = fmul float %history.i138.i997.sroa.16.06962, %_81.i.i.i229.i1139, !dbg !19472
  %_0.i2584 = fadd float %_0.i2588, %_0.i3012, !dbg !19476
  %_0.i3011 = fmul float %history.i138.i997.sroa.16.06962, %_84.i.i.i232.i1142, !dbg !19478
  %_0.i2583 = fadd float %_0.i2587, %_0.i3011, !dbg !19480
  %_0.i3010 = fmul float %history.i138.i997.sroa.16.06962, %_87.i.i.i235.i1145, !dbg !19482
  %_0.i2582 = fadd float %_0.i2586, %_0.i3010, !dbg !19484
  %_0.i3009 = fmul float %history.i138.i997.sroa.16.06962, %_90.i.i.i238.i1148, !dbg !19486
  %_0.i2581 = fadd float %_0.i2585, %_0.i3009, !dbg !19488
  %_0.i3008 = fmul float %history.i138.i997.sroa.19.06963, %_95.i.i.i243.i1153, !dbg !19490
  %_0.i2580 = fadd float %_0.i2584, %_0.i3008, !dbg !19494
  %_0.i3007 = fmul float %history.i138.i997.sroa.19.06963, %_98.i.i.i246.i1156, !dbg !19496
  %_0.i2579 = fadd float %_0.i2583, %_0.i3007, !dbg !19498
  %_0.i3006 = fmul float %history.i138.i997.sroa.19.06963, %_101.i.i.i249.i1159, !dbg !19500
  %_0.i2578 = fadd float %_0.i2582, %_0.i3006, !dbg !19502
  %_0.i3005 = fmul float %history.i138.i997.sroa.19.06963, %_104.i.i.i252.i1162, !dbg !19504
  %_0.i2577 = fadd float %_0.i2581, %_0.i3005, !dbg !19506
  %_0.i3004 = fmul float %history.i138.i997.sroa.22.06964, %_109.i.i.i257.i1167, !dbg !19508
  %_0.i2576 = fadd float %_0.i2580, %_0.i3004, !dbg !19512
  %_0.i3003 = fmul float %history.i138.i997.sroa.22.06964, %_112.i.i.i260.i1170, !dbg !19514
  %_0.i2575 = fadd float %_0.i2579, %_0.i3003, !dbg !19516
  %_0.i3002 = fmul float %history.i138.i997.sroa.22.06964, %_115.i.i.i263.i1173, !dbg !19518
  %_0.i2574 = fadd float %_0.i2578, %_0.i3002, !dbg !19520
  %_0.i3001 = fmul float %history.i138.i997.sroa.22.06964, %_118.i.i.i266.i1176, !dbg !19522
  %_0.i2573 = fadd float %_0.i2577, %_0.i3001, !dbg !19524
  %_0.i3000 = fmul float %history.i138.i997.sroa.26.06965, %_123.i.i.i271.i1181, !dbg !19526
  %_0.i2572 = fadd float %_0.i2576, %_0.i3000, !dbg !19530
  %_0.i2999 = fmul float %history.i138.i997.sroa.26.06965, %_126.i.i.i274.i1184, !dbg !19532
  %_0.i2571 = fadd float %_0.i2575, %_0.i2999, !dbg !19534
  %_0.i2998 = fmul float %history.i138.i997.sroa.26.06965, %_129.i.i.i277.i1187, !dbg !19536
  %_0.i2570 = fadd float %_0.i2574, %_0.i2998, !dbg !19538
  %_0.i2997 = fmul float %history.i138.i997.sroa.26.06965, %_132.i.i.i280.i1190, !dbg !19540
  %_0.i2569 = fadd float %_0.i2573, %_0.i2997, !dbg !19542
  %_0.i2996 = fmul float %history.i138.i997.sroa.29.06966, %_137.i.i.i285.i1195, !dbg !19544
  %_0.i2568 = fadd float %_0.i2572, %_0.i2996, !dbg !19548
  %_0.i2995 = fmul float %history.i138.i997.sroa.29.06966, %_140.i.i.i288.i1198, !dbg !19550
  %_0.i2567 = fadd float %_0.i2571, %_0.i2995, !dbg !19552
  %_0.i2994 = fmul float %history.i138.i997.sroa.29.06966, %_143.i.i.i291.i1201, !dbg !19554
  %_0.i2566 = fadd float %_0.i2570, %_0.i2994, !dbg !19556
  %_0.i2993 = fmul float %history.i138.i997.sroa.29.06966, %_146.i.i.i294.i1204, !dbg !19558
  %_0.i2565 = fadd float %_0.i2569, %_0.i2993, !dbg !19560
  %_0.i2992 = fmul float %history.i138.i997.sroa.32.06967, %_151.i.i.i299.i1209, !dbg !19562
  %_0.i2564 = fadd float %_0.i2568, %_0.i2992, !dbg !19566
  %_0.i2991 = fmul float %history.i138.i997.sroa.32.06967, %_154.i.i.i302.i1212, !dbg !19568
  %_0.i2563 = fadd float %_0.i2567, %_0.i2991, !dbg !19570
  %_0.i2990 = fmul float %history.i138.i997.sroa.32.06967, %_157.i.i.i305.i1215, !dbg !19572
  %_0.i2562 = fadd float %_0.i2566, %_0.i2990, !dbg !19574
  %_0.i2989 = fmul float %history.i138.i997.sroa.32.06967, %_160.i.i.i308.i1218, !dbg !19576
  %_0.i2561 = fadd float %_0.i2565, %_0.i2989, !dbg !19578
  %_0.i2988 = fmul float %history.i138.i997.sroa.35.06968, %_165.i.i.i313.i1223, !dbg !19580
  %_0.i2560 = fadd float %_0.i2564, %_0.i2988, !dbg !19584
  %_0.i2987 = fmul float %history.i138.i997.sroa.35.06968, %_168.i.i.i316.i1226, !dbg !19586
  %_0.i2559 = fadd float %_0.i2563, %_0.i2987, !dbg !19588
  %_0.i2986 = fmul float %history.i138.i997.sroa.35.06968, %_171.i.i.i319.i1229, !dbg !19590
  %_0.i2558 = fadd float %_0.i2562, %_0.i2986, !dbg !19592
  %_0.i2985 = fmul float %history.i138.i997.sroa.35.06968, %_174.i.i.i322.i1232, !dbg !19594
  %_0.i2557 = fadd float %_0.i2561, %_0.i2985, !dbg !19596
  %170 = tail call noundef float @llvm.fabs.f32(float %_0.i2560), !dbg !19598
  %_3.i.i4100.inv = fcmp ogt float %169, %170, !dbg !19602
  %_4.i.i4107.v = select i1 %_3.i.i4100.inv, float %169, float %170, !dbg !19602
  %171 = tail call noundef float @llvm.fabs.f32(float %_0.i2559), !dbg !19598
  %_3.i.i4100.inv.1 = fcmp ogt float %_4.i.i4107.v, %171, !dbg !19602
  %_4.i.i4107.v.1 = select i1 %_3.i.i4100.inv.1, float %_4.i.i4107.v, float %171, !dbg !19602
  %172 = tail call noundef float @llvm.fabs.f32(float %_0.i2558), !dbg !19598
  %_3.i.i4100.inv.2 = fcmp ogt float %_4.i.i4107.v.1, %172, !dbg !19602
  %_4.i.i4107.v.2 = select i1 %_3.i.i4100.inv.2, float %_4.i.i4107.v.1, float %172, !dbg !19602
  %173 = tail call noundef float @llvm.fabs.f32(float %_0.i2557), !dbg !19598
  %_3.i.i4100.inv.3 = fcmp ogt float %_4.i.i4107.v.2, %173, !dbg !19602
  %_4.i.i4107.v.3 = select i1 %_3.i.i4100.inv.3, float %_4.i.i4107.v.2, float %173, !dbg !19602
  %174 = add nuw nsw i32 %iter.i134.i993.sroa.16.06969, 1, !dbg !19608
  %data.i4.i = getelementptr inbounds nuw float, ptr %peaks_left.i1015, i32 %iter.i134.i993.sroa.16.06969, !dbg !19609
  store float %_4.i.i4107.v.3, ptr %data.i4.i, align 4, !dbg !19612, !alias.scope !19614, !noalias !19366
  %exitcond.not = icmp eq i32 %174, %umax12505, !dbg !19340
  br i1 %exitcond.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit333.i1243, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471, !dbg !19340

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit333.i1243: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit
  %history.i138.i997.sroa.0.0.lcssa = phi float [ %history.i138.i997.sroa.0.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3469, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471 ], !dbg !19617
  %history.i138.i997.sroa.7.0.lcssa = phi float [ %history.i138.i997.sroa.7.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i138.i997.sroa.0.06958, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471 ], !dbg !19617
  %history.i138.i997.sroa.10.0.lcssa = phi float [ %history.i138.i997.sroa.10.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i138.i997.sroa.7.06959, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471 ], !dbg !19617
  %history.i138.i997.sroa.13.0.lcssa = phi float [ %history.i138.i997.sroa.13.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i138.i997.sroa.10.06960, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471 ], !dbg !19617
  %history.i138.i997.sroa.16.0.lcssa = phi float [ %history.i138.i997.sroa.16.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i138.i997.sroa.13.06961, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471 ], !dbg !19617
  %history.i138.i997.sroa.19.0.lcssa = phi float [ %history.i138.i997.sroa.19.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i138.i997.sroa.16.06962, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471 ], !dbg !19617
  %history.i138.i997.sroa.22.0.lcssa = phi float [ %history.i138.i997.sroa.22.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i138.i997.sroa.19.06963, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471 ], !dbg !19617
  %history.i138.i997.sroa.26.0.lcssa = phi float [ %history.i138.i997.sroa.26.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i138.i997.sroa.22.06964, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471 ], !dbg !19617
  %history.i138.i997.sroa.29.0.lcssa = phi float [ %history.i138.i997.sroa.29.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i138.i997.sroa.26.06965, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471 ], !dbg !19617
  %history.i138.i997.sroa.32.0.lcssa = phi float [ %history.i138.i997.sroa.32.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i138.i997.sroa.29.06966, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471 ], !dbg !19617
  %history.i138.i997.sroa.35.0.lcssa = phi float [ %history.i138.i997.sroa.35.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i138.i997.sroa.32.06967, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471 ], !dbg !19617
  %history.i138.i997.sroa.38.0.lcssa = phi float [ %history.i138.i997.sroa.38.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i138.i997.sroa.35.06968, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3471 ], !dbg !19617
  store float %history.i138.i997.sroa.0.0.lcssa, ptr %hot_left.i1018, align 4, !dbg !19618, !noalias !19337
  store float %history.i138.i997.sroa.7.0.lcssa, ptr %history.i138.i997.sroa.7.0.hot_left.i1018.sroa_idx, align 4, !dbg !19618, !noalias !19337
  store float %history.i138.i997.sroa.10.0.lcssa, ptr %history.i138.i997.sroa.10.0.hot_left.i1018.sroa_idx, align 4, !dbg !19618, !noalias !19337
  store float %history.i138.i997.sroa.13.0.lcssa, ptr %history.i138.i997.sroa.13.0.hot_left.i1018.sroa_idx, align 4, !dbg !19618, !noalias !19337
  store float %history.i138.i997.sroa.16.0.lcssa, ptr %history.i138.i997.sroa.16.0.hot_left.i1018.sroa_idx, align 4, !dbg !19618, !noalias !19337
  store float %history.i138.i997.sroa.19.0.lcssa, ptr %history.i138.i997.sroa.19.0.hot_left.i1018.sroa_idx, align 4, !dbg !19618, !noalias !19337
  store float %history.i138.i997.sroa.22.0.lcssa, ptr %history.i138.i997.sroa.22.0.hot_left.i1018.sroa_idx, align 4, !dbg !19618, !noalias !19337
  store float %history.i138.i997.sroa.26.0.lcssa, ptr %history.i138.i997.sroa.26.0.hot_left.i1018.sroa_idx, align 4, !dbg !19618, !noalias !19337
  store float %history.i138.i997.sroa.29.0.lcssa, ptr %history.i138.i997.sroa.29.0.hot_left.i1018.sroa_idx, align 4, !dbg !19618, !noalias !19337
  store float %history.i138.i997.sroa.32.0.lcssa, ptr %history.i138.i997.sroa.32.0.hot_left.i1018.sroa_idx, align 4, !dbg !19618, !noalias !19337
  store float %history.i138.i997.sroa.35.0.lcssa, ptr %history.i138.i997.sroa.35.0.hot_left.i1018.sroa_idx, align 4, !dbg !19618, !noalias !19337
  store float %history.i138.i997.sroa.38.0.lcssa, ptr %history.i138.i997.sroa.38.0.hot_left.i1018.sroa_idx, align 4, !dbg !19618, !noalias !19337
  %_136.not.i1244 = icmp ugt i32 %_51.i1043, %right_io.1, !dbg !19619
  br i1 %_136.not.i1244, label %bb49.i1667, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4537, !dbg !19619, !prof !787

bb49.i1667:                                       ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit333.i1243
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %iter.sroa.0.0.i10368146, i32 noundef %_51.i1043, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_685aef6cd6b0f866813eafbee04e700c) #32, !dbg !19623, !noalias !19254
  unreachable, !dbg !19623

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4537: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit333.i1243
  %_143.i1246 = getelementptr inbounds nuw float, ptr %right_io.0, i32 %iter.sroa.0.0.i10368146, !dbg !19624
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19628), !dbg !19631
  %history.i.i1011.sroa.0.0.copyload = load float, ptr %hot_right.i1017, align 4, !dbg !19632, !noalias !19634
  %history.i.i1011.sroa.7.0.copyload = load float, ptr %history.i.i1011.sroa.7.0.hot_right.i1017.sroa_idx, align 4, !dbg !19632, !noalias !19634
  %history.i.i1011.sroa.10.0.copyload = load float, ptr %history.i.i1011.sroa.10.0.hot_right.i1017.sroa_idx, align 4, !dbg !19632, !noalias !19634
  %history.i.i1011.sroa.13.0.copyload = load float, ptr %history.i.i1011.sroa.13.0.hot_right.i1017.sroa_idx, align 4, !dbg !19632, !noalias !19634
  %history.i.i1011.sroa.16.0.copyload = load float, ptr %history.i.i1011.sroa.16.0.hot_right.i1017.sroa_idx, align 4, !dbg !19632, !noalias !19634
  %history.i.i1011.sroa.19.0.copyload = load float, ptr %history.i.i1011.sroa.19.0.hot_right.i1017.sroa_idx, align 4, !dbg !19632, !noalias !19634
  %history.i.i1011.sroa.22.0.copyload = load float, ptr %history.i.i1011.sroa.22.0.hot_right.i1017.sroa_idx, align 4, !dbg !19632, !noalias !19634
  %history.i.i1011.sroa.26.0.copyload = load float, ptr %history.i.i1011.sroa.26.0.hot_right.i1017.sroa_idx, align 4, !dbg !19632, !noalias !19634
  %history.i.i1011.sroa.29.0.copyload = load float, ptr %history.i.i1011.sroa.29.0.hot_right.i1017.sroa_idx, align 4, !dbg !19632, !noalias !19634
  %history.i.i1011.sroa.32.0.copyload = load float, ptr %history.i.i1011.sroa.32.0.hot_right.i1017.sroa_idx, align 4, !dbg !19632, !noalias !19634
  %history.i.i1011.sroa.35.0.copyload = load float, ptr %history.i.i1011.sroa.35.0.hot_right.i1017.sroa_idx, align 4, !dbg !19632, !noalias !19634
  %history.i.i1011.sroa.38.0.copyload = load float, ptr %history.i.i1011.sroa.38.0.hot_right.i1017.sroa_idx, align 4, !dbg !19632, !noalias !19634
  br i1 %_2.i6957.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i1441, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466.lr.ph, !dbg !19637

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466.lr.ph: ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4537
  %_11.i.i.i.i1267 = load float, ptr %_31, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_14.i.i.i.i1270 = load float, ptr %85, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_17.i.i.i.i1273 = load float, ptr %86, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_20.i.i.i.i1276 = load float, ptr %87, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_25.i.i.i.i1281 = load float, ptr %row1.i.i.i171.i1081, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_28.i.i.i.i1284 = load float, ptr %88, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_31.i.i.i.i1287 = load float, ptr %89, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_34.i.i.i.i1290 = load float, ptr %90, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_39.i.i.i.i1295 = load float, ptr %row3.i.i.i185.i1095, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_42.i.i.i.i1298 = load float, ptr %91, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_45.i.i.i.i1301 = load float, ptr %92, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_48.i.i.i.i1304 = load float, ptr %93, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_53.i.i.i.i1309 = load float, ptr %row5.i.i.i199.i1109, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_56.i.i.i.i1312 = load float, ptr %94, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_59.i.i.i.i1315 = load float, ptr %95, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_62.i.i.i.i1318 = load float, ptr %96, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_67.i.i.i.i1323 = load float, ptr %row7.i.i.i213.i1123, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_70.i.i.i.i1326 = load float, ptr %97, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_73.i.i.i.i1329 = load float, ptr %98, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_76.i.i.i.i1332 = load float, ptr %99, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_81.i.i.i.i1337 = load float, ptr %row9.i.i.i227.i1137, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_84.i.i.i.i1340 = load float, ptr %100, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_87.i.i.i.i1343 = load float, ptr %101, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_90.i.i.i.i1346 = load float, ptr %102, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_95.i.i.i.i1351 = load float, ptr %row11.i.i.i241.i1151, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_98.i.i.i.i1354 = load float, ptr %103, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_101.i.i.i.i1357 = load float, ptr %104, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_104.i.i.i.i1360 = load float, ptr %105, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_109.i.i.i.i1365 = load float, ptr %row13.i.i.i255.i1165, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_112.i.i.i.i1368 = load float, ptr %106, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_115.i.i.i.i1371 = load float, ptr %107, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_118.i.i.i.i1374 = load float, ptr %108, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_123.i.i.i.i1379 = load float, ptr %row15.i.i.i269.i1179, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_126.i.i.i.i1382 = load float, ptr %109, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_129.i.i.i.i1385 = load float, ptr %110, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_132.i.i.i.i1388 = load float, ptr %111, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_137.i.i.i.i1393 = load float, ptr %row17.i.i.i283.i1193, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_140.i.i.i.i1396 = load float, ptr %112, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_143.i.i.i.i1399 = load float, ptr %113, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_146.i.i.i.i1402 = load float, ptr %114, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_151.i.i.i.i1407 = load float, ptr %row19.i.i.i297.i1207, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_154.i.i.i.i1410 = load float, ptr %115, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_157.i.i.i.i1413 = load float, ptr %116, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_160.i.i.i.i1416 = load float, ptr %117, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_165.i.i.i.i1421 = load float, ptr %row21.i.i.i311.i1221, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_168.i.i.i.i1424 = load float, ptr %118, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_171.i.i.i.i1427 = load float, ptr %119, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  %_174.i.i.i.i1430 = load float, ptr %120, align 4, !alias.scope !19640, !noalias !19645, !noundef !10
  br label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466, !dbg !19637

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466
  %iter.i.i1007.sroa.16.06995 = phi i32 [ 0, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466.lr.ph ], [ %180, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466 ]
  %history.i.i1011.sroa.35.06994 = phi float [ %history.i.i1011.sroa.35.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466.lr.ph ], [ %history.i.i1011.sroa.32.06993, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466 ]
  %history.i.i1011.sroa.32.06993 = phi float [ %history.i.i1011.sroa.32.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466.lr.ph ], [ %history.i.i1011.sroa.29.06992, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466 ]
  %history.i.i1011.sroa.29.06992 = phi float [ %history.i.i1011.sroa.29.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466.lr.ph ], [ %history.i.i1011.sroa.26.06991, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466 ]
  %history.i.i1011.sroa.26.06991 = phi float [ %history.i.i1011.sroa.26.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466.lr.ph ], [ %history.i.i1011.sroa.22.06990, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466 ]
  %history.i.i1011.sroa.22.06990 = phi float [ %history.i.i1011.sroa.22.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466.lr.ph ], [ %history.i.i1011.sroa.19.06989, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466 ]
  %history.i.i1011.sroa.19.06989 = phi float [ %history.i.i1011.sroa.19.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466.lr.ph ], [ %history.i.i1011.sroa.16.06988, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466 ]
  %history.i.i1011.sroa.16.06988 = phi float [ %history.i.i1011.sroa.16.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466.lr.ph ], [ %history.i.i1011.sroa.13.06987, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466 ]
  %history.i.i1011.sroa.13.06987 = phi float [ %history.i.i1011.sroa.13.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466.lr.ph ], [ %history.i.i1011.sroa.10.06986, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466 ]
  %history.i.i1011.sroa.10.06986 = phi float [ %history.i.i1011.sroa.10.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466.lr.ph ], [ %history.i.i1011.sroa.7.06985, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466 ]
  %history.i.i1011.sroa.7.06985 = phi float [ %history.i.i1011.sroa.7.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466.lr.ph ], [ %history.i.i1011.sroa.0.06984, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466 ]
  %history.i.i1011.sroa.0.06984 = phi float [ %history.i.i1011.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466.lr.ph ], [ %_0.i3464, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466 ]
  %data.i.i4547 = getelementptr inbounds nuw float, ptr %_143.i1246, i32 %iter.i.i1007.sroa.16.06995, !dbg !19650
  %_0.i3464 = load float, ptr %data.i.i4547, align 4, !dbg !19653, !alias.scope !19655, !noalias !19658, !noundef !10
  %175 = tail call noundef float @llvm.fabs.f32(float %history.i.i1011.sroa.19.06989), !dbg !19659
  %_0.i2984 = fmul float %_0.i3464, %_11.i.i.i.i1267, !dbg !19662
  %_0.i2556 = fadd float %_0.i2984, 0.000000e+00, !dbg !19665
  %_0.i2983 = fmul float %_0.i3464, %_14.i.i.i.i1270, !dbg !19667
  %_0.i2555 = fadd float %_0.i2983, 0.000000e+00, !dbg !19669
  %_0.i2982 = fmul float %_0.i3464, %_17.i.i.i.i1273, !dbg !19671
  %_0.i2554 = fadd float %_0.i2982, 0.000000e+00, !dbg !19673
  %_0.i2981 = fmul float %_0.i3464, %_20.i.i.i.i1276, !dbg !19675
  %_0.i2553 = fadd float %_0.i2981, 0.000000e+00, !dbg !19677
  %_0.i2980 = fmul float %history.i.i1011.sroa.0.06984, %_25.i.i.i.i1281, !dbg !19679
  %_0.i2552 = fadd float %_0.i2556, %_0.i2980, !dbg !19681
  %_0.i2979 = fmul float %history.i.i1011.sroa.0.06984, %_28.i.i.i.i1284, !dbg !19683
  %_0.i2551 = fadd float %_0.i2555, %_0.i2979, !dbg !19685
  %_0.i2978 = fmul float %history.i.i1011.sroa.0.06984, %_31.i.i.i.i1287, !dbg !19687
  %_0.i2550 = fadd float %_0.i2554, %_0.i2978, !dbg !19689
  %_0.i2977 = fmul float %history.i.i1011.sroa.0.06984, %_34.i.i.i.i1290, !dbg !19691
  %_0.i2549 = fadd float %_0.i2553, %_0.i2977, !dbg !19693
  %_0.i2976 = fmul float %history.i.i1011.sroa.7.06985, %_39.i.i.i.i1295, !dbg !19695
  %_0.i2548 = fadd float %_0.i2552, %_0.i2976, !dbg !19697
  %_0.i2975 = fmul float %history.i.i1011.sroa.7.06985, %_42.i.i.i.i1298, !dbg !19699
  %_0.i2547 = fadd float %_0.i2551, %_0.i2975, !dbg !19701
  %_0.i2974 = fmul float %history.i.i1011.sroa.7.06985, %_45.i.i.i.i1301, !dbg !19703
  %_0.i2546 = fadd float %_0.i2550, %_0.i2974, !dbg !19705
  %_0.i2973 = fmul float %history.i.i1011.sroa.7.06985, %_48.i.i.i.i1304, !dbg !19707
  %_0.i2545 = fadd float %_0.i2549, %_0.i2973, !dbg !19709
  %_0.i2972 = fmul float %history.i.i1011.sroa.10.06986, %_53.i.i.i.i1309, !dbg !19711
  %_0.i2544 = fadd float %_0.i2548, %_0.i2972, !dbg !19713
  %_0.i2971 = fmul float %history.i.i1011.sroa.10.06986, %_56.i.i.i.i1312, !dbg !19715
  %_0.i2543 = fadd float %_0.i2547, %_0.i2971, !dbg !19717
  %_0.i2970 = fmul float %history.i.i1011.sroa.10.06986, %_59.i.i.i.i1315, !dbg !19719
  %_0.i2542 = fadd float %_0.i2546, %_0.i2970, !dbg !19721
  %_0.i2969 = fmul float %history.i.i1011.sroa.10.06986, %_62.i.i.i.i1318, !dbg !19723
  %_0.i2541 = fadd float %_0.i2545, %_0.i2969, !dbg !19725
  %_0.i2968 = fmul float %history.i.i1011.sroa.13.06987, %_67.i.i.i.i1323, !dbg !19727
  %_0.i2540 = fadd float %_0.i2544, %_0.i2968, !dbg !19729
  %_0.i2967 = fmul float %history.i.i1011.sroa.13.06987, %_70.i.i.i.i1326, !dbg !19731
  %_0.i2539 = fadd float %_0.i2543, %_0.i2967, !dbg !19733
  %_0.i2966 = fmul float %history.i.i1011.sroa.13.06987, %_73.i.i.i.i1329, !dbg !19735
  %_0.i2538 = fadd float %_0.i2542, %_0.i2966, !dbg !19737
  %_0.i2965 = fmul float %history.i.i1011.sroa.13.06987, %_76.i.i.i.i1332, !dbg !19739
  %_0.i2537 = fadd float %_0.i2541, %_0.i2965, !dbg !19741
  %_0.i2964 = fmul float %history.i.i1011.sroa.16.06988, %_81.i.i.i.i1337, !dbg !19743
  %_0.i2536 = fadd float %_0.i2540, %_0.i2964, !dbg !19745
  %_0.i2963 = fmul float %history.i.i1011.sroa.16.06988, %_84.i.i.i.i1340, !dbg !19747
  %_0.i2535 = fadd float %_0.i2539, %_0.i2963, !dbg !19749
  %_0.i2962 = fmul float %history.i.i1011.sroa.16.06988, %_87.i.i.i.i1343, !dbg !19751
  %_0.i2534 = fadd float %_0.i2538, %_0.i2962, !dbg !19753
  %_0.i2961 = fmul float %history.i.i1011.sroa.16.06988, %_90.i.i.i.i1346, !dbg !19755
  %_0.i2533 = fadd float %_0.i2537, %_0.i2961, !dbg !19757
  %_0.i2960 = fmul float %history.i.i1011.sroa.19.06989, %_95.i.i.i.i1351, !dbg !19759
  %_0.i2532 = fadd float %_0.i2536, %_0.i2960, !dbg !19761
  %_0.i2959 = fmul float %history.i.i1011.sroa.19.06989, %_98.i.i.i.i1354, !dbg !19763
  %_0.i2531 = fadd float %_0.i2535, %_0.i2959, !dbg !19765
  %_0.i2958 = fmul float %history.i.i1011.sroa.19.06989, %_101.i.i.i.i1357, !dbg !19767
  %_0.i2530 = fadd float %_0.i2534, %_0.i2958, !dbg !19769
  %_0.i2957 = fmul float %history.i.i1011.sroa.19.06989, %_104.i.i.i.i1360, !dbg !19771
  %_0.i2529 = fadd float %_0.i2533, %_0.i2957, !dbg !19773
  %_0.i2956 = fmul float %history.i.i1011.sroa.22.06990, %_109.i.i.i.i1365, !dbg !19775
  %_0.i2528 = fadd float %_0.i2532, %_0.i2956, !dbg !19777
  %_0.i2955 = fmul float %history.i.i1011.sroa.22.06990, %_112.i.i.i.i1368, !dbg !19779
  %_0.i2527 = fadd float %_0.i2531, %_0.i2955, !dbg !19781
  %_0.i2954 = fmul float %history.i.i1011.sroa.22.06990, %_115.i.i.i.i1371, !dbg !19783
  %_0.i2526 = fadd float %_0.i2530, %_0.i2954, !dbg !19785
  %_0.i2953 = fmul float %history.i.i1011.sroa.22.06990, %_118.i.i.i.i1374, !dbg !19787
  %_0.i2525 = fadd float %_0.i2529, %_0.i2953, !dbg !19789
  %_0.i2952 = fmul float %history.i.i1011.sroa.26.06991, %_123.i.i.i.i1379, !dbg !19791
  %_0.i2524 = fadd float %_0.i2528, %_0.i2952, !dbg !19793
  %_0.i2951 = fmul float %history.i.i1011.sroa.26.06991, %_126.i.i.i.i1382, !dbg !19795
  %_0.i2523 = fadd float %_0.i2527, %_0.i2951, !dbg !19797
  %_0.i2950 = fmul float %history.i.i1011.sroa.26.06991, %_129.i.i.i.i1385, !dbg !19799
  %_0.i2522 = fadd float %_0.i2526, %_0.i2950, !dbg !19801
  %_0.i2949 = fmul float %history.i.i1011.sroa.26.06991, %_132.i.i.i.i1388, !dbg !19803
  %_0.i2521 = fadd float %_0.i2525, %_0.i2949, !dbg !19805
  %_0.i2948 = fmul float %history.i.i1011.sroa.29.06992, %_137.i.i.i.i1393, !dbg !19807
  %_0.i2520 = fadd float %_0.i2524, %_0.i2948, !dbg !19809
  %_0.i2947 = fmul float %history.i.i1011.sroa.29.06992, %_140.i.i.i.i1396, !dbg !19811
  %_0.i2519 = fadd float %_0.i2523, %_0.i2947, !dbg !19813
  %_0.i2946 = fmul float %history.i.i1011.sroa.29.06992, %_143.i.i.i.i1399, !dbg !19815
  %_0.i2518 = fadd float %_0.i2522, %_0.i2946, !dbg !19817
  %_0.i2945 = fmul float %history.i.i1011.sroa.29.06992, %_146.i.i.i.i1402, !dbg !19819
  %_0.i2517 = fadd float %_0.i2521, %_0.i2945, !dbg !19821
  %_0.i2944 = fmul float %history.i.i1011.sroa.32.06993, %_151.i.i.i.i1407, !dbg !19823
  %_0.i2516 = fadd float %_0.i2520, %_0.i2944, !dbg !19825
  %_0.i2943 = fmul float %history.i.i1011.sroa.32.06993, %_154.i.i.i.i1410, !dbg !19827
  %_0.i2515 = fadd float %_0.i2519, %_0.i2943, !dbg !19829
  %_0.i2942 = fmul float %history.i.i1011.sroa.32.06993, %_157.i.i.i.i1413, !dbg !19831
  %_0.i2514 = fadd float %_0.i2518, %_0.i2942, !dbg !19833
  %_0.i2941 = fmul float %history.i.i1011.sroa.32.06993, %_160.i.i.i.i1416, !dbg !19835
  %_0.i2513 = fadd float %_0.i2517, %_0.i2941, !dbg !19837
  %_0.i2940 = fmul float %history.i.i1011.sroa.35.06994, %_165.i.i.i.i1421, !dbg !19839
  %_0.i2512 = fadd float %_0.i2516, %_0.i2940, !dbg !19841
  %_0.i2939 = fmul float %history.i.i1011.sroa.35.06994, %_168.i.i.i.i1424, !dbg !19843
  %_0.i2511 = fadd float %_0.i2515, %_0.i2939, !dbg !19845
  %_0.i2938 = fmul float %history.i.i1011.sroa.35.06994, %_171.i.i.i.i1427, !dbg !19847
  %_0.i2510 = fadd float %_0.i2514, %_0.i2938, !dbg !19849
  %_0.i2937 = fmul float %history.i.i1011.sroa.35.06994, %_174.i.i.i.i1430, !dbg !19851
  %_0.i2509 = fadd float %_0.i2513, %_0.i2937, !dbg !19853
  %176 = tail call noundef float @llvm.fabs.f32(float %_0.i2512), !dbg !19855
  %_3.i.i4091.inv = fcmp ogt float %175, %176, !dbg !19857
  %_4.i.i4098.v = select i1 %_3.i.i4091.inv, float %175, float %176, !dbg !19857
  %177 = tail call noundef float @llvm.fabs.f32(float %_0.i2511), !dbg !19855
  %_3.i.i4091.inv.1 = fcmp ogt float %_4.i.i4098.v, %177, !dbg !19857
  %_4.i.i4098.v.1 = select i1 %_3.i.i4091.inv.1, float %_4.i.i4098.v, float %177, !dbg !19857
  %178 = tail call noundef float @llvm.fabs.f32(float %_0.i2510), !dbg !19855
  %_3.i.i4091.inv.2 = fcmp ogt float %_4.i.i4098.v.1, %178, !dbg !19857
  %_4.i.i4098.v.2 = select i1 %_3.i.i4091.inv.2, float %_4.i.i4098.v.1, float %178, !dbg !19857
  %179 = tail call noundef float @llvm.fabs.f32(float %_0.i2509), !dbg !19855
  %_3.i.i4091.inv.3 = fcmp ogt float %_4.i.i4098.v.2, %179, !dbg !19857
  %_4.i.i4098.v.3 = select i1 %_3.i.i4091.inv.3, float %_4.i.i4098.v.2, float %179, !dbg !19857
  %180 = add nuw nsw i32 %iter.i.i1007.sroa.16.06995, 1, !dbg !19860
  %data.i4.i4551 = getelementptr inbounds nuw float, ptr %peaks_right.i1014, i32 %iter.i.i1007.sroa.16.06995, !dbg !19861
  store float %_4.i.i4098.v.3, ptr %data.i4.i4551, align 4, !dbg !19864, !alias.scope !19866, !noalias !19658
  %exitcond12493.not = icmp eq i32 %180, %umax12505, !dbg !19637
  br i1 %exitcond12493.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i1441, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466, !dbg !19637

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i1441: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4537
  %history.i.i1011.sroa.0.0.lcssa = phi float [ %history.i.i1011.sroa.0.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4537 ], [ %_0.i3464, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466 ], !dbg !19869
  %history.i.i1011.sroa.7.0.lcssa = phi float [ %history.i.i1011.sroa.7.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4537 ], [ %history.i.i1011.sroa.0.06984, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466 ], !dbg !19869
  %history.i.i1011.sroa.10.0.lcssa = phi float [ %history.i.i1011.sroa.10.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4537 ], [ %history.i.i1011.sroa.7.06985, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466 ], !dbg !19869
  %history.i.i1011.sroa.13.0.lcssa = phi float [ %history.i.i1011.sroa.13.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4537 ], [ %history.i.i1011.sroa.10.06986, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466 ], !dbg !19869
  %history.i.i1011.sroa.16.0.lcssa = phi float [ %history.i.i1011.sroa.16.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4537 ], [ %history.i.i1011.sroa.13.06987, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466 ], !dbg !19869
  %history.i.i1011.sroa.19.0.lcssa = phi float [ %history.i.i1011.sroa.19.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4537 ], [ %history.i.i1011.sroa.16.06988, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466 ], !dbg !19869
  %history.i.i1011.sroa.22.0.lcssa = phi float [ %history.i.i1011.sroa.22.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4537 ], [ %history.i.i1011.sroa.19.06989, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466 ], !dbg !19869
  %history.i.i1011.sroa.26.0.lcssa = phi float [ %history.i.i1011.sroa.26.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4537 ], [ %history.i.i1011.sroa.22.06990, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466 ], !dbg !19869
  %history.i.i1011.sroa.29.0.lcssa = phi float [ %history.i.i1011.sroa.29.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4537 ], [ %history.i.i1011.sroa.26.06991, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466 ], !dbg !19869
  %history.i.i1011.sroa.32.0.lcssa = phi float [ %history.i.i1011.sroa.32.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4537 ], [ %history.i.i1011.sroa.29.06992, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466 ], !dbg !19869
  %history.i.i1011.sroa.35.0.lcssa = phi float [ %history.i.i1011.sroa.35.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4537 ], [ %history.i.i1011.sroa.32.06993, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466 ], !dbg !19869
  %history.i.i1011.sroa.38.0.lcssa = phi float [ %history.i.i1011.sroa.38.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4537 ], [ %history.i.i1011.sroa.35.06994, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3466 ], !dbg !19869
  store float %history.i.i1011.sroa.0.0.lcssa, ptr %hot_right.i1017, align 4, !dbg !19870, !noalias !19634
  store float %history.i.i1011.sroa.7.0.lcssa, ptr %history.i.i1011.sroa.7.0.hot_right.i1017.sroa_idx, align 4, !dbg !19870, !noalias !19634
  store float %history.i.i1011.sroa.10.0.lcssa, ptr %history.i.i1011.sroa.10.0.hot_right.i1017.sroa_idx, align 4, !dbg !19870, !noalias !19634
  store float %history.i.i1011.sroa.13.0.lcssa, ptr %history.i.i1011.sroa.13.0.hot_right.i1017.sroa_idx, align 4, !dbg !19870, !noalias !19634
  store float %history.i.i1011.sroa.16.0.lcssa, ptr %history.i.i1011.sroa.16.0.hot_right.i1017.sroa_idx, align 4, !dbg !19870, !noalias !19634
  store float %history.i.i1011.sroa.19.0.lcssa, ptr %history.i.i1011.sroa.19.0.hot_right.i1017.sroa_idx, align 4, !dbg !19870, !noalias !19634
  store float %history.i.i1011.sroa.22.0.lcssa, ptr %history.i.i1011.sroa.22.0.hot_right.i1017.sroa_idx, align 4, !dbg !19870, !noalias !19634
  store float %history.i.i1011.sroa.26.0.lcssa, ptr %history.i.i1011.sroa.26.0.hot_right.i1017.sroa_idx, align 4, !dbg !19870, !noalias !19634
  store float %history.i.i1011.sroa.29.0.lcssa, ptr %history.i.i1011.sroa.29.0.hot_right.i1017.sroa_idx, align 4, !dbg !19870, !noalias !19634
  store float %history.i.i1011.sroa.32.0.lcssa, ptr %history.i.i1011.sroa.32.0.hot_right.i1017.sroa_idx, align 4, !dbg !19870, !noalias !19634
  store float %history.i.i1011.sroa.35.0.lcssa, ptr %history.i.i1011.sroa.35.0.hot_right.i1017.sroa_idx, align 4, !dbg !19870, !noalias !19634
  store float %history.i.i1011.sroa.38.0.lcssa, ptr %history.i.i1011.sroa.38.0.hot_right.i1017.sroa_idx, align 4, !dbg !19870, !noalias !19634
  br i1 %_2.i6957.not, label %bb13.i1035.loopexit, label %bb50.i1447.lr.ph, !dbg !19297

bb50.i1447.lr.ph:                                 ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i1441
  %_13.i240153405342 = load float, ptr %123, align 4, !alias.scope !19240, !noalias !19243, !noundef !10
  %_13.i238953435345 = load float, ptr %126, align 4, !alias.scope !19251, !noalias !19254, !noundef !10
  %_13.i237753465348 = load float, ptr %129, align 4, !alias.scope !19261, !noalias !19264, !noundef !10
  %_13.i236653495351 = load float, ptr %132, align 4, !alias.scope !19271, !noalias !19254, !noundef !10
  %_91.i1468 = load i32, ptr %133, align 4
  %_62.i88.i1525 = load float, ptr %144, align 4
  %_62.i.i1618 = load float, ptr %160, align 4
  %_106.i1650 = load i32, ptr %164, align 4
  %.promoted = load float, ptr %121, align 4, !alias.scope !19240, !noalias !19243
  %_68.i1449.promoted = load float, ptr %_68.i1449, align 4, !alias.scope !19240, !noalias !19243
  %.promoted7175 = load float, ptr %122, align 4, !alias.scope !19240, !noalias !19243
  %.promoted7245 = load float, ptr %124, align 4, !alias.scope !19251, !noalias !19254
  %_69.i1450.promoted = load float, ptr %_69.i1450, align 4, !alias.scope !19251, !noalias !19254
  %.promoted7383 = load float, ptr %125, align 4, !alias.scope !19251, !noalias !19254
  %.promoted7453 = load float, ptr %127, align 4, !alias.scope !19261, !noalias !19264
  %_73.i1451.promoted = load float, ptr %_73.i1451, align 4, !alias.scope !19261, !noalias !19264
  %.promoted7591 = load float, ptr %128, align 4, !alias.scope !19261, !noalias !19264
  %.promoted7661 = load float, ptr %130, align 4, !alias.scope !19271, !noalias !19254
  %_74.i1452.promoted = load float, ptr %_74.i1452, align 4, !alias.scope !19271, !noalias !19254
  %.promoted7799 = load float, ptr %131, align 4, !alias.scope !19271, !noalias !19254
  %.promoted7869 = load float, ptr %143, align 4
  %.promoted7938 = load float, ptr %145, align 4
  %.promoted8007 = load float, ptr %159, align 4
  %.promoted8076 = load float, ptr %161, align 4
  br label %bb50.i1447, !dbg !19297

bb50.i1447:                                       ; preds = %bb50.i1447.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit
  %_0.i37248077 = phi float [ %.promoted8076, %bb50.i1447.lr.ph ], [ %_0.i3724, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit ]
  %_0.i33508008 = phi float [ %.promoted8007, %bb50.i1447.lr.ph ], [ %_0.i3350, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit ]
  %_0.i37287939 = phi float [ %.promoted7938, %bb50.i1447.lr.ph ], [ %_0.i3728, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit ]
  %_0.i33547870 = phi float [ %.promoted7869, %bb50.i1447.lr.ph ], [ %_0.i3354, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit ]
  %_12.i23647800 = phi float [ %.promoted7799, %bb50.i1447.lr.ph ], [ %_0.i3847, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !19871
  %_0.i38547731 = phi float [ %_74.i1452.promoted, %bb50.i1447.lr.ph ], [ %_0.i3854, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !19871
  %_5.i23617662 = phi float [ %.promoted7661, %bb50.i1447.lr.ph ], [ %_0.i.i4064, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !19871
  %_12.i23757592 = phi float [ %.promoted7591, %bb50.i1447.lr.ph ], [ %_0.i3834, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !19871
  %_0.i38417523 = phi float [ %_73.i1451.promoted, %bb50.i1447.lr.ph ], [ %_0.i3841, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !19871
  %_5.i23697454 = phi float [ %.promoted7453, %bb50.i1447.lr.ph ], [ %_0.i.i4057, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !19871
  %_12.i23877384 = phi float [ %.promoted7383, %bb50.i1447.lr.ph ], [ %_0.i3821, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !19871
  %_0.i38287315 = phi float [ %_69.i1450.promoted, %bb50.i1447.lr.ph ], [ %_0.i3828, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !19871
  %_5.i23817246 = phi float [ %.promoted7245, %bb50.i1447.lr.ph ], [ %_0.i.i4050, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !19871
  %_12.i23997176 = phi float [ %.promoted7175, %bb50.i1447.lr.ph ], [ %_0.i3808, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !19871
  %_0.i38157107 = phi float [ %_68.i1449.promoted, %bb50.i1447.lr.ph ], [ %_0.i3815, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !19871
  %_5.i23937038 = phi float [ %.promoted, %bb50.i1447.lr.ph ], [ %_0.i.i4043, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !19871
  %main_cursor.sroa.0.1.i14457035 = phi i32 [ %main_cursor.sroa.0.0.i10398149, %bb50.i1447.lr.ph ], [ %spec.store.select11.i1652, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit ]
  %ring_cursor.sroa.0.1.i14447034 = phi i32 [ %ring_cursor.sroa.0.0.i10388148, %bb50.i1447.lr.ph ], [ %spec.store.select12.i1654, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit ]
  %iter1.sroa.0.0.i14437033 = phi i32 [ 0, %bb50.i1447.lr.ph ], [ %181, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit ]
  %181 = add nuw nsw i32 %iter1.sroa.0.0.i14437033, 1, !dbg !19871
  %_64.i1448 = add nuw nsw i32 %iter1.sroa.0.0.i14437033, %iter.sroa.0.0.i10368146, !dbg !19877
  %_0.i3343 = fadd float %_5.i23937038, -1.000000e+00, !dbg !19878
  %_3.i.i4037 = fcmp ogt float %_0.i3343, 0.000000e+00, !dbg !19881
  %_0.i.i4043 = select i1 %_3.i.i4037, float %_0.i3343, float 0.000000e+00, !dbg !19885
  %_0.i2503 = fadd float %_0.i38157107, %_12.i23997176, !dbg !19887
  %_0.i3815 = select i1 %_3.i.i4037, float %_0.i2503, float %_13.i240153405342, !dbg !19889
  %_0.i3808 = select i1 %_3.i.i4037, float %_12.i23997176, float 0.000000e+00, !dbg !19891
  %_0.i3344 = fadd float %_5.i23817246, -1.000000e+00, !dbg !19893
  %_3.i.i4044 = fcmp ogt float %_0.i3344, 0.000000e+00, !dbg !19895
  %_0.i.i4050 = select i1 %_3.i.i4044, float %_0.i3344, float 0.000000e+00, !dbg !19898
  %_0.i2504 = fadd float %_0.i38287315, %_12.i23877384, !dbg !19900
  %_0.i3828 = select i1 %_3.i.i4044, float %_0.i2504, float %_13.i238953435345, !dbg !19902
  %_0.i3821 = select i1 %_3.i.i4044, float %_12.i23877384, float 0.000000e+00, !dbg !19904
  %_0.i3345 = fadd float %_5.i23697454, -1.000000e+00, !dbg !19906
  %_3.i.i4051 = fcmp ogt float %_0.i3345, 0.000000e+00, !dbg !19908
  %_0.i.i4057 = select i1 %_3.i.i4051, float %_0.i3345, float 0.000000e+00, !dbg !19911
  %_0.i2505 = fadd float %_0.i38417523, %_12.i23757592, !dbg !19913
  %_0.i3841 = select i1 %_3.i.i4051, float %_0.i2505, float %_13.i237753465348, !dbg !19915
  %_0.i3834 = select i1 %_3.i.i4051, float %_12.i23757592, float 0.000000e+00, !dbg !19917
  %_0.i3346 = fadd float %_5.i23617662, -1.000000e+00, !dbg !19919
  %_3.i.i4058 = fcmp ogt float %_0.i3346, 0.000000e+00, !dbg !19921
  %_0.i.i4064 = select i1 %_3.i.i4058, float %_0.i3346, float 0.000000e+00, !dbg !19924
  %_0.i2506 = fadd float %_0.i38547731, %_12.i23647800, !dbg !19926
  %_0.i3854 = select i1 %_3.i.i4058, float %_0.i2506, float %_13.i236653495351, !dbg !19928
  %_0.i3847 = select i1 %_3.i.i4058, float %_12.i23647800, float 0.000000e+00, !dbg !19930
  %_159.i1456 = getelementptr inbounds nuw float, ptr %peaks_left.i1015, i32 %iter1.sroa.0.0.i14437033, !dbg !19932
  %_0.i3459 = load float, ptr %_159.i1456, align 4, !dbg !19943, !alias.scope !19945, !noalias !19254, !noundef !10
  %_164.i1458 = getelementptr inbounds nuw float, ptr %peaks_right.i1014, i32 %iter1.sroa.0.0.i14437033, !dbg !19948
  %_0.i3454 = load float, ptr %_164.i1458, align 4, !dbg !19958, !alias.scope !19960, !noalias !19254, !noundef !10
  %_3.i.i4082 = fcmp ule float %_0.i3454, %_0.i3459, !dbg !19963
  %_6.i.i4084 = bitcast float %_0.i3454 to i32, !dbg !19966
  %_8.i.i4086 = bitcast float %_0.i3459 to i32, !dbg !19970
  %_4.i.i4089 = select i1 %_3.i.i4082, i32 %_8.i.i4086, i32 %_6.i.i4084, !dbg !19972
  %_5.i3888 = and i32 %_4.i.i4089, %.none.i1023, !dbg !19973
  %_7.i3891 = and i32 %_9.i3890, %_8.i.i4086, !dbg !19975
  %_4.i3892 = or disjoint i32 %_5.i3888, %_7.i3891, !dbg !19973
  %_0.i3893 = bitcast i32 %_4.i3892 to float, !dbg !19976
  %_7.i3884 = and i32 %_9.i3890, %_6.i.i4084, !dbg !19979
  %_4.i3885 = or disjoint i32 %_5.i3888, %_7.i3884, !dbg !19981
  %_0.i3886 = bitcast i32 %_4.i3885 to float, !dbg !19982
  %_165.i1463 = icmp ugt i32 %_64.i1448, %left_io.1, !dbg !19984
  br i1 %_165.i1463, label %bb54.i1666, label %bb55.i1464, !dbg !19984, !prof !787

bb55.i1464:                                       ; preds = %bb50.i1447
  %_171.i1466 = getelementptr inbounds nuw float, ptr %left_io.0, i32 %_64.i1448, !dbg !19988
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19993), !dbg !19996
  %_3.not.i3447 = icmp eq i32 %left_io.1, %_64.i1448, !dbg !19997
  br i1 %_3.not.i3447, label %panic.i3450, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3451, !dbg !19997

panic.i3450:                                      ; preds = %bb55.i1464
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #32, !dbg !19997, !noalias !19999
  unreachable, !dbg !19997

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3451: ; preds = %bb55.i1464
  %_0.i3449 = load float, ptr %_171.i1466, align 4, !dbg !19997, !alias.scope !19993, !noalias !19254, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20000), !dbg !20003
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20004), !dbg !20003
  %width.i32.i1469 = load i32, ptr %134, align 4, !dbg !20006, !alias.scope !20007, !noalias !20008, !noundef !10
  %_3.i2469 = fcmp uge float %_0.i3815, %_0.i3893, !dbg !20011
  %_0.i2902 = fdiv float %_0.i3815, %_0.i3893, !dbg !20013
  %_0.i3879 = select i1 %_3.i2469, float 1.000000e+00, float %_0.i2902, !dbg !20016
  %_158.1.i37.i1474 = load i32, ptr %135, align 4, !dbg !20018, !alias.scope !20007, !noalias !20008, !noundef !10
  %_22.i38.i1475 = mul i32 %width.i32.i1469, %ring_cursor.sroa.0.1.i14447034, !dbg !20019
  %_90.i39.i1476 = icmp ugt i32 %_22.i38.i1475, %_158.1.i37.i1474, !dbg !20020
  br i1 %_90.i39.i1476, label %bb34.i123.i1665, label %bb35.i40.i1477, !dbg !20020, !prof !787

bb35.i40.i1477:                                   ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3451
  %_158.0.i41.i1478 = load ptr, ptr %136, align 4, !dbg !20018, !alias.scope !20007, !noalias !20008, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20025), !dbg !20028
  %_4.not.i3610 = icmp eq i32 %_158.1.i37.i1474, %_22.i38.i1475, !dbg !20029
  br i1 %_4.not.i3610, label %panic.i3612, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3613, !dbg !20029

panic.i3612:                                      ; preds = %bb35.i40.i1477
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #32, !dbg !20029, !noalias !20031
  unreachable, !dbg !20029

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3613: ; preds = %bb35.i40.i1477
  %_97.i43.i1480 = getelementptr inbounds nuw float, ptr %_158.0.i41.i1478, i32 %_22.i38.i1475, !dbg !20032
  store float %_0.i3879, ptr %_97.i43.i1480, align 4, !dbg !20029, !alias.scope !20025, !noalias !20037
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20038), !dbg !20041
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20042), !dbg !20041
  %width.i1990 = load i32, ptr %134, align 4, !dbg !20044, !alias.scope !20038, !noalias !20046, !noundef !10
  %182 = icmp eq i32 %width.i1990, 0, !dbg !20047
  br i1 %182, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2096, label %bb29.i1996.lr.ph, !dbg !20047

bb29.i1996.lr.ph:                                 ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3613
  %_126.1.i2001 = load i32, ptr %62, align 4, !alias.scope !20038, !noalias !20046, !noundef !10
  %_126.0.i2005 = load ptr, ptr %61, align 4, !nonnull !10
  %183 = add i32 %ring_cursor.sroa.0.1.i14447034, 1
  %_21.not.i2010 = icmp ult i32 %183, %_91.i1468
  %184 = select i1 %_21.not.i2010, i32 0, i32 %_91.i1468
  %start1.sroa.0.0.i2011 = sub nuw i32 %183, %184
  %_128.1.i2014 = load i32, ptr %135, align 4
  %_128.0.i2018 = load ptr, ptr %136, align 4, !nonnull !10
  %_130.1.i2019 = load i32, ptr %137, align 4
  %_130.0.i2023 = load ptr, ptr %138, align 4, !nonnull !10
  %_132.1.i2026 = load i32, ptr %139, align 4
  %_132.0.i2030 = load ptr, ptr %140, align 4, !nonnull !10
  %_43.i2043 = mul i32 %width.i1990, %start1.sroa.0.0.i2011
  br label %bb29.i1996, !dbg !20047

bb29.i1996:                                       ; preds = %bb29.i1996.lr.ph, %bb28.i2058
  %iter.sroa.0.0.idx.i19947014 = phi i32 [ 0, %bb29.i1996.lr.ph ], [ %iter.sroa.0.0.add.i1999, %bb28.i2058 ]
  %iter.sroa.4.0.i19937013 = phi i32 [ 0, %bb29.i1996.lr.ph ], [ %_102.0.i2000, %bb28.i2058 ]
  %iter.sroa.7.0.i19927012 = phi i32 [ %width.i1990, %bb29.i1996.lr.ph ], [ %185, %bb28.i2058 ]
  %iter.sroa.0.0.ptr.i19957015 = getelementptr inbounds nuw i8, ptr %scratch.i1016, i32 %iter.sroa.0.0.idx.i19947014, !dbg !20049
  %185 = add i32 %iter.sroa.7.0.i19927012, -1, !dbg !20049
  %_109.i1997 = icmp eq i32 %iter.sroa.0.0.idx.i19947014, 32, !dbg !20050
  br i1 %_109.i1997, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2096, label %bb33.i1998, !dbg !20054

bb33.i1998:                                       ; preds = %bb29.i1996
  %iter.sroa.0.0.add.i1999 = add nuw nsw i32 %iter.sroa.0.0.idx.i19947014, 4, !dbg !20055
  %_102.0.i2000 = add nuw nsw i32 %iter.sroa.4.0.i19937013, 1, !dbg !20057
  %exitcond12495.not = icmp eq i32 %iter.sroa.4.0.i19937013, %_126.1.i2001, !dbg !20058
  br i1 %exitcond12495.not, label %panic.i2003, label %bb2.i2004, !dbg !20058

bb2.i2004:                                        ; preds = %bb33.i1998
  %186 = getelementptr inbounds nuw %LaneShape, ptr %_126.0.i2005, i32 %iter.sroa.4.0.i19937013, !dbg !20058
  %shape.i2006 = load i32, ptr %186, align 4, !dbg !20058, !noalias !20059, !noundef !10
  %187 = getelementptr inbounds nuw i8, ptr %186, i32 4, !dbg !20058
  %shape3.i2007 = load i32, ptr %187, align 4, !dbg !20058, !noalias !20059, !noundef !10
  %188 = add i32 %shape3.i2007, %ring_cursor.sroa.0.1.i14447034, !dbg !20060
  %_18.not.i2008 = icmp ult i32 %188, %_91.i1468, !dbg !20061
  %189 = select i1 %_18.not.i2008, i32 0, i32 %_91.i1468, !dbg !20061
  %spec.select.i2009 = sub nuw i32 %188, %189, !dbg !20061
  %_25.i2012 = mul i32 %spec.select.i2009, %width.i1990, !dbg !20062
  %_24.i2013 = add i32 %_25.i2012, %iter.sroa.4.0.i19937013, !dbg !20062
  %_28.i2015 = icmp ult i32 %_24.i2013, %_128.1.i2014, !dbg !20063
  br i1 %_28.i2015, label %bb9.i2017, label %panic5.i2016, !dbg !20063

panic.i2003:                                      ; preds = %bb33.i1998
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_126.1.i2001, i32 noundef %_126.1.i2001, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_12856b5f9033764dbf23e04eb77c5c46) #32, !dbg !20058, !noalias !20059
  unreachable, !dbg !20058

bb9.i2017:                                        ; preds = %bb2.i2004
  %190 = getelementptr inbounds nuw float, ptr %_128.0.i2018, i32 %_24.i2013, !dbg !20063
  %191 = load float, ptr %190, align 4, !dbg !20063, !noalias !20059, !noundef !10
  %exitcond12496.not = icmp eq i32 %iter.sroa.4.0.i19937013, %_130.1.i2019, !dbg !20064
  br i1 %exitcond12496.not, label %panic6.i2021, label %bb10.i2022, !dbg !20064

panic5.i2016:                                     ; preds = %bb2.i2004
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_24.i2013, i32 noundef %_128.1.i2014, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70ebf0cf1c90c6161926611ccf249a32) #32, !dbg !20063, !noalias !20059
  unreachable, !dbg !20063

bb10.i2022:                                       ; preds = %bb9.i2017
  %192 = getelementptr inbounds nuw i32, ptr %_130.0.i2023, i32 %iter.sroa.4.0.i19937013, !dbg !20064
  %_30.i2024 = load i32, ptr %192, align 4, !dbg !20064, !noalias !20059, !noundef !10
  %193 = icmp eq i32 %_30.i2024, 0, !dbg !20065
  br i1 %193, label %bb14.i2033, label %bb12.i2025, !dbg !20065

panic6.i2021:                                     ; preds = %bb9.i2017
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_130.1.i2019, i32 noundef %_130.1.i2019, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_023d591aaad7f5cf482ac80a9401eca1) #32, !dbg !20064, !noalias !20059
  unreachable, !dbg !20064

bb12.i2025:                                       ; preds = %bb10.i2022
  %_35.i2027 = icmp ult i32 %iter.sroa.4.0.i19937013, %_132.1.i2026, !dbg !20066
  br i1 %_35.i2027, label %bb13.i2029, label %panic7.i2028, !dbg !20066

bb14.i2033:                                       ; preds = %bb34.i2094, %bb13.i2029, %bb10.i2022
  %newest.sroa.0.0.i2034 = phi float [ %191, %bb10.i2022 ], [ %_33.i2031, %bb34.i2094 ], [ %191, %bb13.i2029 ], !dbg !20067
  %exitcond12497.not = icmp eq i32 %iter.sroa.4.0.i19937013, %_132.1.i2026, !dbg !20068
  br i1 %exitcond12497.not, label %panic8.i2037, label %bb15.i2038, !dbg !20068

bb13.i2029:                                       ; preds = %bb12.i2025
  %194 = getelementptr inbounds nuw float, ptr %_132.0.i2030, i32 %iter.sroa.4.0.i19937013, !dbg !20066
  %_33.i2031 = load float, ptr %194, align 4, !dbg !20066, !noalias !20059, !noundef !10
  %_116.i2032 = fcmp olt float %_33.i2031, %191, !dbg !20069
  br i1 %_116.i2032, label %bb34.i2094, label %bb14.i2033, !dbg !20069

panic7.i2028:                                     ; preds = %bb12.i2025
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %iter.sroa.4.0.i19937013, i32 noundef %_132.1.i2026, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a228cac3279aae3a34886f59f51fbe9e) #32, !dbg !20066, !noalias !20059
  unreachable, !dbg !20066

bb34.i2094:                                       ; preds = %bb13.i2029
  br label %bb14.i2033, !dbg !20071

bb15.i2038:                                       ; preds = %bb14.i2033
  %195 = getelementptr inbounds nuw float, ptr %_132.0.i2030, i32 %iter.sroa.4.0.i19937013, !dbg !20068
  store float %newest.sroa.0.0.i2034, ptr %195, align 4, !dbg !20068, !noalias !20059
  %_40.i2040 = add i32 %_30.i2024, 1, !dbg !20072
  %complete.i2041 = icmp eq i32 %_40.i2040, %shape.i2006, !dbg !20072
  br i1 %complete.i2041, label %bb19.i2063, label %bb17.i2042, !dbg !20073

panic8.i2037:                                     ; preds = %bb14.i2033
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_132.1.i2026, i32 noundef %_132.1.i2026, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1fa5d88c1a2cc292a2767c48606a38fe) #32, !dbg !20068, !noalias !20059
  unreachable, !dbg !20068

bb17.i2042:                                       ; preds = %bb15.i2038
  %_42.i2044 = add i32 %iter.sroa.4.0.i19937013, %_43.i2043, !dbg !20074
  %_45.i2046 = icmp ult i32 %_42.i2044, %_128.1.i2014, !dbg !20075
  br i1 %_45.i2046, label %bb27.i2056, label %panic9.i2047, !dbg !20075

panic9.i2047:                                     ; preds = %bb17.i2042
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_42.i2044, i32 noundef %_128.1.i2014, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70d40ae2302f3ea19fa0c4a65fe82786) #32, !dbg !20075, !noalias !20059
  unreachable, !dbg !20075

bb27.i2056:                                       ; preds = %bb17.i2042
  %196 = getelementptr inbounds nuw float, ptr %_128.0.i2018, i32 %_42.i2044, !dbg !20075
  %_41.i2050 = load float, ptr %196, align 4, !dbg !20075, !noalias !20059, !noundef !10
  %_117.i2051 = fcmp olt float %_41.i2050, %newest.sroa.0.0.i2034, !dbg !20076
  %newest.sroa.0.1.i2052 = select i1 %_117.i2051, float %_41.i2050, float %newest.sroa.0.0.i2034, !dbg !20076
  store float %newest.sroa.0.1.i2052, ptr %iter.sroa.0.0.ptr.i19957015, align 4, !dbg !20078, !alias.scope !20042, !noalias !20079
  br label %bb28.i2058, !dbg !20080

bb28.i2058:                                       ; preds = %bb22.i2091, %bb19.i2063, %bb27.i2056
  %storemerge = phi i32 [ %_40.i2040, %bb27.i2056 ], [ 0, %bb19.i2063 ], [ 0, %bb22.i2091 ], !dbg !20081
  store i32 %storemerge, ptr %192, align 4, !dbg !20081, !noalias !20059
  %197 = icmp eq i32 %185, 0, !dbg !20047
  br i1 %197, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2096, label %bb29.i1996, !dbg !20047

bb19.i2063:                                       ; preds = %bb15.i2038
  store float %newest.sroa.0.0.i2034, ptr %iter.sroa.0.0.ptr.i19957015, align 4, !dbg !20078, !alias.scope !20042, !noalias !20079
  %_118.i20697008.not = icmp eq i32 %shape.i2006, 0, !dbg !20082
  br i1 %_118.i20697008.not, label %bb28.i2058, label %bb40.i2076.preheader, !dbg !20086

bb40.i2076.preheader:                             ; preds = %bb19.i2063
  %198 = load float, ptr %190, align 4, !dbg !20087, !noalias !20059, !noundef !10
  br label %bb40.i2076, !dbg !20088

bb40.i2076:                                       ; preds = %bb40.i2076.preheader, %bb22.i2091
  %iter2.sroa.0.0.i20687011 = phi i32 [ %_119.i2077, %bb22.i2091 ], [ 0, %bb40.i2076.preheader ]
  %suffix.sroa.0.0.i20677010 = phi float [ %suffix.sroa.0.1.i2087, %bb22.i2091 ], [ %198, %bb40.i2076.preheader ]
  %end.sroa.0.1.i20667009 = phi i32 [ %201, %bb22.i2091 ], [ %spec.select.i2009, %bb40.i2076.preheader ]
  %_54.i2078 = mul i32 %end.sroa.0.1.i20667009, %width.i1990, !dbg !20089
  %_53.i2079 = add i32 %_54.i2078, %iter.sroa.4.0.i19937013, !dbg !20089
  %_57.i2081 = icmp ult i32 %_53.i2079, %_128.1.i2014, !dbg !20088
  br i1 %_57.i2081, label %bb22.i2091, label %panic13.i2082, !dbg !20088

panic13.i2082:                                    ; preds = %bb40.i2076
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_53.i2079, i32 noundef %_128.1.i2014, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a271bbb7fd83c3605ea58a06b7065fa4) #32, !dbg !20088, !noalias !20059
  unreachable, !dbg !20088

bb22.i2091:                                       ; preds = %bb40.i2076
  %_119.i2077 = add nuw i32 %iter2.sroa.0.0.i20687011, 1, !dbg !20090
  %199 = getelementptr inbounds nuw float, ptr %_128.0.i2018, i32 %_53.i2079, !dbg !20088
  %_52.i2085 = load float, ptr %199, align 4, !dbg !20088, !noalias !20059, !noundef !10
  %_121.i2086 = fcmp olt float %suffix.sroa.0.0.i20677010, %_52.i2085, !dbg !20093
  %suffix.sroa.0.1.i2087 = select i1 %_121.i2086, float %suffix.sroa.0.0.i20677010, float %_52.i2085, !dbg !20093
  store float %suffix.sroa.0.1.i2087, ptr %199, align 4, !dbg !20095, !noalias !20059
  %200 = icmp eq i32 %end.sroa.0.1.i20667009, 0, !dbg !20096
  %spec.store.select.i2093 = select i1 %200, i32 %_91.i1468, i32 %end.sroa.0.1.i20667009, !dbg !20096
  %201 = add i32 %spec.store.select.i2093, -1, !dbg !20097
  %exitcond12494.not = icmp eq i32 %_119.i2077, %shape.i2006, !dbg !20082
  br i1 %exitcond12494.not, label %bb28.i2058, label %bb40.i2076, !dbg !20086

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2096: ; preds = %bb29.i1996, %bb28.i2058, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3613
  %_0.i3446 = load float, ptr %scratch.i1016, align 4, !dbg !20098, !alias.scope !20100, !noalias !20103, !noundef !10
  %_0.i2936 = fmul float %_0.i3446, 1.638400e+04, !dbg !20104
  %202 = tail call noundef float @llvm.floor.f32(float %_0.i2936), !dbg !20106
  %_0.i2935 = fmul float %202, 0x3F10000000000000, !dbg !20113
  %203 = icmp eq i32 %width.i32.i1469, 0, !dbg !20115
  %_163.1.i81.i1518.pre = load i32, ptr %141, align 4, !dbg !20120, !alias.scope !20007, !noalias !20008
  br i1 %203, label %bb53.i76.i1513, label %bb36.i55.i1492.lr.ph, !dbg !20115

bb36.i55.i1492.lr.ph:                             ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2096
  %_159.1.i60.i1497 = load i32, ptr %62, align 4, !alias.scope !20007, !noalias !20008, !noundef !10
  %_159.0.i64.i1501 = load ptr, ptr %61, align 4, !nonnull !10
  %_161.0.i74.i1511 = load ptr, ptr %142, align 4, !nonnull !10
  %exitcond12498.not = icmp eq i32 %_159.1.i60.i1497, 0, !dbg !20121
  br i1 %exitcond12498.not, label %panic.i62.i1499, label %bb14.i63.i1500, !dbg !20121

bb34.i123.i1665:                                  ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3451
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i38.i1475, i32 noundef %_158.1.i37.i1474, i32 noundef %_158.1.i37.i1474, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_db7c42041d7aaaba4839906f37294547) #32, !dbg !20123, !noalias !20037
  unreachable, !dbg !20123

bb53.i76.i1513:                                   ; preds = %bb18.i73.i1510.7, %bb18.i73.i1510, %bb18.i73.i1510.1, %bb18.i73.i1510.2, %bb18.i73.i1510.3, %bb18.i73.i1510.4, %bb18.i73.i1510.5, %bb18.i73.i1510.6, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2096
  %_0.i3444 = phi float [ %_0.i3446, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2096 ], [ %_47.i75.i1512, %bb18.i73.i1510 ], [ %_47.i75.i1512, %bb18.i73.i1510.7 ], [ %_47.i75.i1512, %bb18.i73.i1510.6 ], [ %_47.i75.i1512, %bb18.i73.i1510.5 ], [ %_47.i75.i1512, %bb18.i73.i1510.4 ], [ %_47.i75.i1512, %bb18.i73.i1510.3 ], [ %_47.i75.i1512, %bb18.i73.i1510.2 ], [ %_47.i75.i1512, %bb18.i73.i1510.1 ], !dbg !20124
  %_0.i2508 = fadd float %_0.i2935, %_0.i33547870, !dbg !20126
  %_0.i3354 = fsub float %_0.i2508, %_0.i3444, !dbg !20128
  %_123.i82.i1519 = icmp ugt i32 %_22.i38.i1475, %_163.1.i81.i1518.pre, !dbg !20130
  br i1 %_123.i82.i1519, label %bb41.i122.i1664, label %bb42.i83.i1520, !dbg !20130, !prof !787

bb14.i63.i1500:                                   ; preds = %bb36.i55.i1492.lr.ph
  %204 = getelementptr inbounds nuw i8, ptr %_159.0.i64.i1501, i32 8, !dbg !20121
  %_42.i65.i1502 = load i32, ptr %204, align 4, !dbg !20121, !noalias !20103, !noundef !10
  %205 = add i32 %_42.i65.i1502, %ring_cursor.sroa.0.1.i14447034, !dbg !20134
  %_45.not.i66.i1503 = icmp ult i32 %205, %_91.i1468, !dbg !20135
  %206 = select i1 %_45.not.i66.i1503, i32 0, i32 %_91.i1468, !dbg !20135
  %spec.select.i67.i1504 = sub nuw i32 %205, %206, !dbg !20135
  %_49.i68.i1505 = mul i32 %spec.select.i67.i1504, %width.i32.i1469, !dbg !20137
  %_51.i71.i1508 = icmp ult i32 %_49.i68.i1505, %_163.1.i81.i1518.pre, !dbg !20138
  br i1 %_51.i71.i1508, label %bb18.i73.i1510, label %panic1.i72.i1509, !dbg !20138

panic.i62.i1499:                                  ; preds = %bb36.i55.i1492.7, %bb36.i55.i1492.6, %bb36.i55.i1492.5, %bb36.i55.i1492.4, %bb36.i55.i1492.3, %bb36.i55.i1492.2, %bb36.i55.i1492.1, %bb36.i55.i1492.lr.ph
  %_159.1.i60.i1497.lcssa.ph = phi i32 [ 7, %bb36.i55.i1492.7 ], [ 6, %bb36.i55.i1492.6 ], [ 5, %bb36.i55.i1492.5 ], [ 4, %bb36.i55.i1492.4 ], [ 3, %bb36.i55.i1492.3 ], [ 2, %bb36.i55.i1492.2 ], [ 1, %bb36.i55.i1492.1 ], [ 0, %bb36.i55.i1492.lr.ph ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_159.1.i60.i1497.lcssa.ph, i32 noundef %_159.1.i60.i1497.lcssa.ph, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_33746731a929bad63709ddedbca82f5f) #32, !dbg !20121, !noalias !20103
  unreachable, !dbg !20121

bb18.i73.i1510:                                   ; preds = %bb14.i63.i1500
  %207 = getelementptr inbounds nuw float, ptr %_161.0.i74.i1511, i32 %_49.i68.i1505, !dbg !20138
  %_47.i75.i1512 = load float, ptr %207, align 4, !dbg !20138, !noalias !20103, !noundef !10
  store float %_47.i75.i1512, ptr %scratch.i1016, align 4, !dbg !20139, !alias.scope !20004, !noalias !20140
  %208 = icmp eq i32 %width.i32.i1469, 1, !dbg !20115
  br i1 %208, label %bb53.i76.i1513, label %bb36.i55.i1492.1, !dbg !20115

bb36.i55.i1492.1:                                 ; preds = %bb18.i73.i1510
  %exitcond12498.1.not = icmp eq i32 %_159.1.i60.i1497, 1, !dbg !20121
  br i1 %exitcond12498.1.not, label %panic.i62.i1499, label %bb14.i63.i1500.1, !dbg !20121

bb14.i63.i1500.1:                                 ; preds = %bb36.i55.i1492.1
  %209 = getelementptr inbounds nuw i8, ptr %_159.0.i64.i1501, i32 20, !dbg !20121
  %_42.i65.i1502.1 = load i32, ptr %209, align 4, !dbg !20121, !noalias !20103, !noundef !10
  %210 = add i32 %_42.i65.i1502.1, %ring_cursor.sroa.0.1.i14447034, !dbg !20134
  %_45.not.i66.i1503.1 = icmp ult i32 %210, %_91.i1468, !dbg !20135
  %211 = select i1 %_45.not.i66.i1503.1, i32 0, i32 %_91.i1468, !dbg !20135
  %spec.select.i67.i1504.1 = sub nuw i32 %210, %211, !dbg !20135
  %_49.i68.i1505.1 = mul i32 %spec.select.i67.i1504.1, %width.i32.i1469, !dbg !20137
  %_48.i69.i1506.1 = add i32 %_49.i68.i1505.1, 1, !dbg !20137
  %_51.i71.i1508.1 = icmp ult i32 %_48.i69.i1506.1, %_163.1.i81.i1518.pre, !dbg !20138
  br i1 %_51.i71.i1508.1, label %bb18.i73.i1510.1, label %panic1.i72.i1509, !dbg !20138

bb18.i73.i1510.1:                                 ; preds = %bb14.i63.i1500.1
  %212 = getelementptr inbounds nuw float, ptr %_161.0.i74.i1511, i32 %_48.i69.i1506.1, !dbg !20138
  %_47.i75.i1512.1 = load float, ptr %212, align 4, !dbg !20138, !noalias !20103, !noundef !10
  store float %_47.i75.i1512.1, ptr %iter.sroa.0.0.ptr.i54.i14917019.1, align 4, !dbg !20139, !alias.scope !20004, !noalias !20140
  %213 = icmp eq i32 %width.i32.i1469, 2, !dbg !20115
  br i1 %213, label %bb53.i76.i1513, label %bb36.i55.i1492.2, !dbg !20115

bb36.i55.i1492.2:                                 ; preds = %bb18.i73.i1510.1
  %exitcond12498.2.not = icmp eq i32 %_159.1.i60.i1497, 2, !dbg !20121
  br i1 %exitcond12498.2.not, label %panic.i62.i1499, label %bb14.i63.i1500.2, !dbg !20121

bb14.i63.i1500.2:                                 ; preds = %bb36.i55.i1492.2
  %214 = getelementptr inbounds nuw i8, ptr %_159.0.i64.i1501, i32 32, !dbg !20121
  %_42.i65.i1502.2 = load i32, ptr %214, align 4, !dbg !20121, !noalias !20103, !noundef !10
  %215 = add i32 %_42.i65.i1502.2, %ring_cursor.sroa.0.1.i14447034, !dbg !20134
  %_45.not.i66.i1503.2 = icmp ult i32 %215, %_91.i1468, !dbg !20135
  %216 = select i1 %_45.not.i66.i1503.2, i32 0, i32 %_91.i1468, !dbg !20135
  %spec.select.i67.i1504.2 = sub nuw i32 %215, %216, !dbg !20135
  %_49.i68.i1505.2 = mul i32 %spec.select.i67.i1504.2, %width.i32.i1469, !dbg !20137
  %_48.i69.i1506.2 = add i32 %_49.i68.i1505.2, 2, !dbg !20137
  %_51.i71.i1508.2 = icmp ult i32 %_48.i69.i1506.2, %_163.1.i81.i1518.pre, !dbg !20138
  br i1 %_51.i71.i1508.2, label %bb18.i73.i1510.2, label %panic1.i72.i1509, !dbg !20138

bb18.i73.i1510.2:                                 ; preds = %bb14.i63.i1500.2
  %217 = getelementptr inbounds nuw float, ptr %_161.0.i74.i1511, i32 %_48.i69.i1506.2, !dbg !20138
  %_47.i75.i1512.2 = load float, ptr %217, align 4, !dbg !20138, !noalias !20103, !noundef !10
  store float %_47.i75.i1512.2, ptr %iter.sroa.0.0.ptr.i54.i14917019.2, align 4, !dbg !20139, !alias.scope !20004, !noalias !20140
  %218 = icmp eq i32 %width.i32.i1469, 3, !dbg !20115
  br i1 %218, label %bb53.i76.i1513, label %bb36.i55.i1492.3, !dbg !20115

bb36.i55.i1492.3:                                 ; preds = %bb18.i73.i1510.2
  %exitcond12498.3.not = icmp eq i32 %_159.1.i60.i1497, 3, !dbg !20121
  br i1 %exitcond12498.3.not, label %panic.i62.i1499, label %bb14.i63.i1500.3, !dbg !20121

bb14.i63.i1500.3:                                 ; preds = %bb36.i55.i1492.3
  %219 = getelementptr inbounds nuw i8, ptr %_159.0.i64.i1501, i32 44, !dbg !20121
  %_42.i65.i1502.3 = load i32, ptr %219, align 4, !dbg !20121, !noalias !20103, !noundef !10
  %220 = add i32 %_42.i65.i1502.3, %ring_cursor.sroa.0.1.i14447034, !dbg !20134
  %_45.not.i66.i1503.3 = icmp ult i32 %220, %_91.i1468, !dbg !20135
  %221 = select i1 %_45.not.i66.i1503.3, i32 0, i32 %_91.i1468, !dbg !20135
  %spec.select.i67.i1504.3 = sub nuw i32 %220, %221, !dbg !20135
  %_49.i68.i1505.3 = mul i32 %spec.select.i67.i1504.3, %width.i32.i1469, !dbg !20137
  %_48.i69.i1506.3 = add i32 %_49.i68.i1505.3, 3, !dbg !20137
  %_51.i71.i1508.3 = icmp ult i32 %_48.i69.i1506.3, %_163.1.i81.i1518.pre, !dbg !20138
  br i1 %_51.i71.i1508.3, label %bb18.i73.i1510.3, label %panic1.i72.i1509, !dbg !20138

bb18.i73.i1510.3:                                 ; preds = %bb14.i63.i1500.3
  %222 = getelementptr inbounds nuw float, ptr %_161.0.i74.i1511, i32 %_48.i69.i1506.3, !dbg !20138
  %_47.i75.i1512.3 = load float, ptr %222, align 4, !dbg !20138, !noalias !20103, !noundef !10
  store float %_47.i75.i1512.3, ptr %iter.sroa.0.0.ptr.i54.i14917019.3, align 4, !dbg !20139, !alias.scope !20004, !noalias !20140
  %223 = icmp eq i32 %width.i32.i1469, 4, !dbg !20115
  br i1 %223, label %bb53.i76.i1513, label %bb36.i55.i1492.4, !dbg !20115

bb36.i55.i1492.4:                                 ; preds = %bb18.i73.i1510.3
  %exitcond12498.4.not = icmp eq i32 %_159.1.i60.i1497, 4, !dbg !20121
  br i1 %exitcond12498.4.not, label %panic.i62.i1499, label %bb14.i63.i1500.4, !dbg !20121

bb14.i63.i1500.4:                                 ; preds = %bb36.i55.i1492.4
  %224 = getelementptr inbounds nuw i8, ptr %_159.0.i64.i1501, i32 56, !dbg !20121
  %_42.i65.i1502.4 = load i32, ptr %224, align 4, !dbg !20121, !noalias !20103, !noundef !10
  %225 = add i32 %_42.i65.i1502.4, %ring_cursor.sroa.0.1.i14447034, !dbg !20134
  %_45.not.i66.i1503.4 = icmp ult i32 %225, %_91.i1468, !dbg !20135
  %226 = select i1 %_45.not.i66.i1503.4, i32 0, i32 %_91.i1468, !dbg !20135
  %spec.select.i67.i1504.4 = sub nuw i32 %225, %226, !dbg !20135
  %_49.i68.i1505.4 = mul i32 %spec.select.i67.i1504.4, %width.i32.i1469, !dbg !20137
  %_48.i69.i1506.4 = add i32 %_49.i68.i1505.4, 4, !dbg !20137
  %_51.i71.i1508.4 = icmp ult i32 %_48.i69.i1506.4, %_163.1.i81.i1518.pre, !dbg !20138
  br i1 %_51.i71.i1508.4, label %bb18.i73.i1510.4, label %panic1.i72.i1509, !dbg !20138

bb18.i73.i1510.4:                                 ; preds = %bb14.i63.i1500.4
  %227 = getelementptr inbounds nuw float, ptr %_161.0.i74.i1511, i32 %_48.i69.i1506.4, !dbg !20138
  %_47.i75.i1512.4 = load float, ptr %227, align 4, !dbg !20138, !noalias !20103, !noundef !10
  store float %_47.i75.i1512.4, ptr %iter.sroa.0.0.ptr.i54.i14917019.4, align 4, !dbg !20139, !alias.scope !20004, !noalias !20140
  %228 = icmp eq i32 %width.i32.i1469, 5, !dbg !20115
  br i1 %228, label %bb53.i76.i1513, label %bb36.i55.i1492.5, !dbg !20115

bb36.i55.i1492.5:                                 ; preds = %bb18.i73.i1510.4
  %exitcond12498.5.not = icmp eq i32 %_159.1.i60.i1497, 5, !dbg !20121
  br i1 %exitcond12498.5.not, label %panic.i62.i1499, label %bb14.i63.i1500.5, !dbg !20121

bb14.i63.i1500.5:                                 ; preds = %bb36.i55.i1492.5
  %229 = getelementptr inbounds nuw i8, ptr %_159.0.i64.i1501, i32 68, !dbg !20121
  %_42.i65.i1502.5 = load i32, ptr %229, align 4, !dbg !20121, !noalias !20103, !noundef !10
  %230 = add i32 %_42.i65.i1502.5, %ring_cursor.sroa.0.1.i14447034, !dbg !20134
  %_45.not.i66.i1503.5 = icmp ult i32 %230, %_91.i1468, !dbg !20135
  %231 = select i1 %_45.not.i66.i1503.5, i32 0, i32 %_91.i1468, !dbg !20135
  %spec.select.i67.i1504.5 = sub nuw i32 %230, %231, !dbg !20135
  %_49.i68.i1505.5 = mul i32 %spec.select.i67.i1504.5, %width.i32.i1469, !dbg !20137
  %_48.i69.i1506.5 = add i32 %_49.i68.i1505.5, 5, !dbg !20137
  %_51.i71.i1508.5 = icmp ult i32 %_48.i69.i1506.5, %_163.1.i81.i1518.pre, !dbg !20138
  br i1 %_51.i71.i1508.5, label %bb18.i73.i1510.5, label %panic1.i72.i1509, !dbg !20138

bb18.i73.i1510.5:                                 ; preds = %bb14.i63.i1500.5
  %232 = getelementptr inbounds nuw float, ptr %_161.0.i74.i1511, i32 %_48.i69.i1506.5, !dbg !20138
  %_47.i75.i1512.5 = load float, ptr %232, align 4, !dbg !20138, !noalias !20103, !noundef !10
  store float %_47.i75.i1512.5, ptr %iter.sroa.0.0.ptr.i54.i14917019.5, align 4, !dbg !20139, !alias.scope !20004, !noalias !20140
  %233 = icmp eq i32 %width.i32.i1469, 6, !dbg !20115
  br i1 %233, label %bb53.i76.i1513, label %bb36.i55.i1492.6, !dbg !20115

bb36.i55.i1492.6:                                 ; preds = %bb18.i73.i1510.5
  %exitcond12498.6.not = icmp eq i32 %_159.1.i60.i1497, 6, !dbg !20121
  br i1 %exitcond12498.6.not, label %panic.i62.i1499, label %bb14.i63.i1500.6, !dbg !20121

bb14.i63.i1500.6:                                 ; preds = %bb36.i55.i1492.6
  %234 = getelementptr inbounds nuw i8, ptr %_159.0.i64.i1501, i32 80, !dbg !20121
  %_42.i65.i1502.6 = load i32, ptr %234, align 4, !dbg !20121, !noalias !20103, !noundef !10
  %235 = add i32 %_42.i65.i1502.6, %ring_cursor.sroa.0.1.i14447034, !dbg !20134
  %_45.not.i66.i1503.6 = icmp ult i32 %235, %_91.i1468, !dbg !20135
  %236 = select i1 %_45.not.i66.i1503.6, i32 0, i32 %_91.i1468, !dbg !20135
  %spec.select.i67.i1504.6 = sub nuw i32 %235, %236, !dbg !20135
  %_49.i68.i1505.6 = mul i32 %spec.select.i67.i1504.6, %width.i32.i1469, !dbg !20137
  %_48.i69.i1506.6 = add i32 %_49.i68.i1505.6, 6, !dbg !20137
  %_51.i71.i1508.6 = icmp ult i32 %_48.i69.i1506.6, %_163.1.i81.i1518.pre, !dbg !20138
  br i1 %_51.i71.i1508.6, label %bb18.i73.i1510.6, label %panic1.i72.i1509, !dbg !20138

bb18.i73.i1510.6:                                 ; preds = %bb14.i63.i1500.6
  %237 = getelementptr inbounds nuw float, ptr %_161.0.i74.i1511, i32 %_48.i69.i1506.6, !dbg !20138
  %_47.i75.i1512.6 = load float, ptr %237, align 4, !dbg !20138, !noalias !20103, !noundef !10
  store float %_47.i75.i1512.6, ptr %iter.sroa.0.0.ptr.i54.i14917019.6, align 4, !dbg !20139, !alias.scope !20004, !noalias !20140
  %238 = icmp eq i32 %width.i32.i1469, 7, !dbg !20115
  br i1 %238, label %bb53.i76.i1513, label %bb36.i55.i1492.7, !dbg !20115

bb36.i55.i1492.7:                                 ; preds = %bb18.i73.i1510.6
  %exitcond12498.7.not = icmp eq i32 %_159.1.i60.i1497, 7, !dbg !20121
  br i1 %exitcond12498.7.not, label %panic.i62.i1499, label %bb14.i63.i1500.7, !dbg !20121

bb14.i63.i1500.7:                                 ; preds = %bb36.i55.i1492.7
  %239 = getelementptr inbounds nuw i8, ptr %_159.0.i64.i1501, i32 92, !dbg !20121
  %_42.i65.i1502.7 = load i32, ptr %239, align 4, !dbg !20121, !noalias !20103, !noundef !10
  %240 = add i32 %_42.i65.i1502.7, %ring_cursor.sroa.0.1.i14447034, !dbg !20134
  %_45.not.i66.i1503.7 = icmp ult i32 %240, %_91.i1468, !dbg !20135
  %241 = select i1 %_45.not.i66.i1503.7, i32 0, i32 %_91.i1468, !dbg !20135
  %spec.select.i67.i1504.7 = sub nuw i32 %240, %241, !dbg !20135
  %_49.i68.i1505.7 = mul i32 %spec.select.i67.i1504.7, %width.i32.i1469, !dbg !20137
  %_48.i69.i1506.7 = add i32 %_49.i68.i1505.7, 7, !dbg !20137
  %_51.i71.i1508.7 = icmp ult i32 %_48.i69.i1506.7, %_163.1.i81.i1518.pre, !dbg !20138
  br i1 %_51.i71.i1508.7, label %bb18.i73.i1510.7, label %panic1.i72.i1509, !dbg !20138

bb18.i73.i1510.7:                                 ; preds = %bb14.i63.i1500.7
  %242 = getelementptr inbounds nuw float, ptr %_161.0.i74.i1511, i32 %_48.i69.i1506.7, !dbg !20138
  %_47.i75.i1512.7 = load float, ptr %242, align 4, !dbg !20138, !noalias !20103, !noundef !10
  store float %_47.i75.i1512.7, ptr %iter.sroa.0.0.ptr.i54.i14917019.7, align 4, !dbg !20139, !alias.scope !20004, !noalias !20140
  br label %bb53.i76.i1513, !dbg !20115

panic1.i72.i1509:                                 ; preds = %bb14.i63.i1500.7, %bb14.i63.i1500.6, %bb14.i63.i1500.5, %bb14.i63.i1500.4, %bb14.i63.i1500.3, %bb14.i63.i1500.2, %bb14.i63.i1500.1, %bb14.i63.i1500
  %_48.i69.i1506.lcssa.ph = phi i32 [ %_48.i69.i1506.7, %bb14.i63.i1500.7 ], [ %_48.i69.i1506.6, %bb14.i63.i1500.6 ], [ %_48.i69.i1506.5, %bb14.i63.i1500.5 ], [ %_48.i69.i1506.4, %bb14.i63.i1500.4 ], [ %_48.i69.i1506.3, %bb14.i63.i1500.3 ], [ %_48.i69.i1506.2, %bb14.i63.i1500.2 ], [ %_48.i69.i1506.1, %bb14.i63.i1500.1 ], [ %_49.i68.i1505, %bb14.i63.i1500 ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_48.i69.i1506.lcssa.ph, i32 noundef %_163.1.i81.i1518.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_0e76a26fbb751dfb8cf8a069b5b0d39a) #32, !dbg !20138, !noalias !20103
  unreachable, !dbg !20138

bb42.i83.i1520:                                   ; preds = %bb53.i76.i1513
  %_163.0.i84.i1521 = load ptr, ptr %142, align 4, !dbg !20120, !alias.scope !20007, !noalias !20008, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20141), !dbg !20144
  %_4.not.i3606 = icmp eq i32 %_163.1.i81.i1518.pre, %_22.i38.i1475, !dbg !20145
  br i1 %_4.not.i3606, label %panic.i3608, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3609, !dbg !20145

panic.i3608:                                      ; preds = %bb42.i83.i1520
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #32, !dbg !20145, !noalias !20147
  unreachable, !dbg !20145

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3609: ; preds = %bb42.i83.i1520
  %_130.i86.i1523 = getelementptr inbounds nuw float, ptr %_163.0.i84.i1521, i32 %_22.i38.i1475, !dbg !20148
  store float %_0.i2935, ptr %_130.i86.i1523, align 4, !dbg !20145, !alias.scope !20141, !noalias !20103
  %_0.i2901 = fdiv float %_0.i3354, %_62.i88.i1525, !dbg !20153
  %_0.i3353 = fsub float 1.000000e+00, %_0.i2901, !dbg !20155
  %_0.i3352 = fsub float %_0.i3353, %_0.i37287939, !dbg !20157
  %_4.i2917 = fmul float %_0.i3828, %_0.i3352, !dbg !20159
  %_0.i2918 = fadd float %_0.i37287939, %_4.i2917, !dbg !20159
  %_3.i.i4073.inv = fcmp ogt float %_0.i3353, %_0.i2918, !dbg !20162
  %_4.i.i4080.v = select i1 %_3.i.i4073.inv, float %_0.i3353, float %_0.i2918, !dbg !20162
  %243 = tail call noundef float @llvm.fabs.f32(float %_4.i.i4080.v), !dbg !20166
  %244 = fcmp uge float %243, 0x3BC79CA100000000, !dbg !20170
  %_0.i3728 = select i1 %244, float %_4.i.i4080.v, float 0.000000e+00, !dbg !20173
  %_0.i3351 = fsub float 1.000000e+00, %_0.i3728, !dbg !20174
  %_164.1.i100.i1537 = load i32, ptr %146, align 4, !dbg !20176, !alias.scope !20007, !noalias !20008, !noundef !10
  %_74.i101.i1538 = mul i32 %width.i32.i1469, %main_cursor.sroa.0.1.i14457035, !dbg !20178
  %_134.i102.i1539 = icmp ugt i32 %_74.i101.i1538, %_164.1.i100.i1537, !dbg !20179
  br i1 %_134.i102.i1539, label %bb47.i121.i1663, label %bb48.i103.i1540, !dbg !20179, !prof !787

bb41.i122.i1664:                                  ; preds = %bb53.i76.i1513
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i38.i1475, i32 noundef %_163.1.i81.i1518.pre, i32 noundef %_163.1.i81.i1518.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c2286ff41a33b8c11aa270fe215441de) #32, !dbg !20184, !noalias !20103
  unreachable, !dbg !20184

bb48.i103.i1540:                                  ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3609
  %_164.0.i104.i1541 = load ptr, ptr %147, align 4, !dbg !20176, !alias.scope !20007, !noalias !20008, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20185), !dbg !20188
  %_3.not.i3438 = icmp eq i32 %_164.1.i100.i1537, %_74.i101.i1538, !dbg !20189
  br i1 %_3.not.i3438, label %panic.i3441, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3601, !dbg !20189

panic.i3441:                                      ; preds = %bb48.i103.i1540
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #32, !dbg !20189, !noalias !20191
  unreachable, !dbg !20189

bb47.i121.i1663:                                  ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3609
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_74.i101.i1538, i32 noundef %_164.1.i100.i1537, i32 noundef %_164.1.i100.i1537, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a3a21efd18b403ec25db545d522c479) #32, !dbg !20192, !noalias !20103
  unreachable, !dbg !20192

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3601: ; preds = %bb48.i103.i1540
  %_141.i106.i1543 = getelementptr inbounds nuw float, ptr %_164.0.i104.i1541, i32 %_74.i101.i1538, !dbg !20193
  %_0.i3440 = load float, ptr %_141.i106.i1543, align 4, !dbg !20189, !alias.scope !20185, !noalias !20103, !noundef !10
  store float %_0.i3449, ptr %_141.i106.i1543, align 4, !dbg !20198, !alias.scope !20201, !noalias !20103
  %_0.i2934 = fmul float %_0.i3351, %_0.i3440, !dbg !20204
  %_6.i3867 = bitcast float %_0.i3440 to i32, !dbg !20206
  %_5.i3868 = and i32 %_6.i3867, %all.sroa.0.0.i1025, !dbg !20209
  %_8.i3869 = bitcast float %_0.i2934 to i32, !dbg !20210
  %_7.i3871 = and i32 %_9.i3870, %_8.i3869, !dbg !20212
  %_4.i3872 = or disjoint i32 %_7.i3871, %_5.i3868, !dbg !20209
  store i32 %_4.i3872, ptr %_171.i1466, align 4, !dbg !20213, !alias.scope !20215, !noalias !20218
  %_172.i1557 = icmp ugt i32 %_64.i1448, %right_io.1, !dbg !20219
  br i1 %_172.i1557, label %bb56.i1660, label %bb57.i1558, !dbg !20219, !prof !787

bb54.i1666:                                       ; preds = %bb50.i1447
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_64.i1448, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_adfae95437a74b76017017a3a282b9f2) #32, !dbg !20223, !noalias !19254
  unreachable, !dbg !20223

bb57.i1558:                                       ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3601
  %_178.i1560 = getelementptr inbounds nuw float, ptr %right_io.0, i32 %_64.i1448, !dbg !20224
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20229), !dbg !20232
  %_3.not.i3433 = icmp eq i32 %right_io.1, %_64.i1448, !dbg !20233
  br i1 %_3.not.i3433, label %panic.i3436, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3437, !dbg !20233

panic.i3436:                                      ; preds = %bb57.i1558
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #32, !dbg !20233, !noalias !20235
  unreachable, !dbg !20233

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3437: ; preds = %bb57.i1558
  %_0.i3435 = load float, ptr %_178.i1560, align 4, !dbg !20233, !alias.scope !20229, !noalias !19254, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20236), !dbg !20239
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20240), !dbg !20239
  %width.i.i1562 = load i32, ptr %148, align 4, !dbg !20242, !alias.scope !20243, !noalias !20244, !noundef !10
  %_3.i2467 = fcmp uge float %_0.i3841, %_0.i3886, !dbg !20247
  %_0.i2900 = fdiv float %_0.i3841, %_0.i3886, !dbg !20249
  %_0.i3866 = select i1 %_3.i2467, float 1.000000e+00, float %_0.i2900, !dbg !20251
  %_158.1.i.i1567 = load i32, ptr %149, align 4, !dbg !20253, !alias.scope !20243, !noalias !20244, !noundef !10
  %_22.i.i1568 = mul i32 %width.i.i1562, %ring_cursor.sroa.0.1.i14447034, !dbg !20254
  %_90.i.i1569 = icmp ugt i32 %_22.i.i1568, %_158.1.i.i1567, !dbg !20255
  br i1 %_90.i.i1569, label %bb34.i.i1659, label %bb35.i.i1570, !dbg !20255, !prof !787

bb35.i.i1570:                                     ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3437
  %_158.0.i.i1571 = load ptr, ptr %150, align 4, !dbg !20253, !alias.scope !20243, !noalias !20244, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20258), !dbg !20261
  %_4.not.i3594 = icmp eq i32 %_158.1.i.i1567, %_22.i.i1568, !dbg !20262
  br i1 %_4.not.i3594, label %panic.i3596, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3597, !dbg !20262

panic.i3596:                                      ; preds = %bb35.i.i1570
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #32, !dbg !20262, !noalias !20264
  unreachable, !dbg !20262

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3597: ; preds = %bb35.i.i1570
  %_97.i.i1573 = getelementptr inbounds nuw float, ptr %_158.0.i.i1571, i32 %_22.i.i1568, !dbg !20265
  store float %_0.i3866, ptr %_97.i.i1573, align 4, !dbg !20262, !alias.scope !20258, !noalias !20267
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20268), !dbg !20271
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20272), !dbg !20271
  %width.i = load i32, ptr %148, align 4, !dbg !20274, !alias.scope !20268, !noalias !20276, !noundef !10
  %245 = icmp eq i32 %width.i, 0, !dbg !20277
  br i1 %245, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit, label %bb29.i.lr.ph, !dbg !20277

bb29.i.lr.ph:                                     ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3597
  %_126.1.i = load i32, ptr %151, align 4, !alias.scope !20268, !noalias !20276, !noundef !10
  %_126.0.i = load ptr, ptr %152, align 4, !nonnull !10
  %246 = add i32 %ring_cursor.sroa.0.1.i14447034, 1
  %_21.not.i = icmp ult i32 %246, %_91.i1468
  %247 = select i1 %_21.not.i, i32 0, i32 %_91.i1468
  %start1.sroa.0.0.i = sub nuw i32 %246, %247
  %_128.1.i = load i32, ptr %149, align 4
  %_128.0.i = load ptr, ptr %150, align 4, !nonnull !10
  %_130.1.i = load i32, ptr %153, align 4
  %_130.0.i = load ptr, ptr %154, align 4, !nonnull !10
  %_132.1.i = load i32, ptr %155, align 4
  %_132.0.i = load ptr, ptr %156, align 4, !nonnull !10
  %_43.i = mul i32 %width.i, %start1.sroa.0.0.i
  br label %bb29.i, !dbg !20277

bb29.i:                                           ; preds = %bb29.i.lr.ph, %bb28.i
  %iter.sroa.0.0.idx.i7026 = phi i32 [ 0, %bb29.i.lr.ph ], [ %iter.sroa.0.0.add.i, %bb28.i ]
  %iter.sroa.4.0.i7025 = phi i32 [ 0, %bb29.i.lr.ph ], [ %_102.0.i, %bb28.i ]
  %iter.sroa.7.0.i7024 = phi i32 [ %width.i, %bb29.i.lr.ph ], [ %248, %bb28.i ]
  %iter.sroa.0.0.ptr.i7027 = getelementptr inbounds nuw i8, ptr %scratch.i1016, i32 %iter.sroa.0.0.idx.i7026, !dbg !20279
  %248 = add i32 %iter.sroa.7.0.i7024, -1, !dbg !20279
  %_109.i = icmp eq i32 %iter.sroa.0.0.idx.i7026, 32, !dbg !20280
  br i1 %_109.i, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit, label %bb33.i, !dbg !20284

bb33.i:                                           ; preds = %bb29.i
  %iter.sroa.0.0.add.i = add nuw nsw i32 %iter.sroa.0.0.idx.i7026, 4, !dbg !20285
  %_102.0.i = add nuw nsw i32 %iter.sroa.4.0.i7025, 1, !dbg !20287
  %exitcond12500.not = icmp eq i32 %iter.sroa.4.0.i7025, %_126.1.i, !dbg !20288
  br i1 %exitcond12500.not, label %panic.i, label %bb2.i1966, !dbg !20288

bb2.i1966:                                        ; preds = %bb33.i
  %249 = getelementptr inbounds nuw %LaneShape, ptr %_126.0.i, i32 %iter.sroa.4.0.i7025, !dbg !20288
  %shape.i = load i32, ptr %249, align 4, !dbg !20288, !noalias !20289, !noundef !10
  %250 = getelementptr inbounds nuw i8, ptr %249, i32 4, !dbg !20288
  %shape3.i = load i32, ptr %250, align 4, !dbg !20288, !noalias !20289, !noundef !10
  %251 = add i32 %shape3.i, %ring_cursor.sroa.0.1.i14447034, !dbg !20290
  %_18.not.i = icmp ult i32 %251, %_91.i1468, !dbg !20291
  %252 = select i1 %_18.not.i, i32 0, i32 %_91.i1468, !dbg !20291
  %spec.select.i = sub nuw i32 %251, %252, !dbg !20291
  %_25.i = mul i32 %spec.select.i, %width.i, !dbg !20292
  %_24.i = add i32 %_25.i, %iter.sroa.4.0.i7025, !dbg !20292
  %_28.i1967 = icmp ult i32 %_24.i, %_128.1.i, !dbg !20293
  br i1 %_28.i1967, label %bb9.i, label %panic5.i, !dbg !20293

panic.i:                                          ; preds = %bb33.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_126.1.i, i32 noundef %_126.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_12856b5f9033764dbf23e04eb77c5c46) #32, !dbg !20288, !noalias !20289
  unreachable, !dbg !20288

bb9.i:                                            ; preds = %bb2.i1966
  %253 = getelementptr inbounds nuw float, ptr %_128.0.i, i32 %_24.i, !dbg !20293
  %254 = load float, ptr %253, align 4, !dbg !20293, !noalias !20289, !noundef !10
  %exitcond12501.not = icmp eq i32 %iter.sroa.4.0.i7025, %_130.1.i, !dbg !20294
  br i1 %exitcond12501.not, label %panic6.i, label %bb10.i1969, !dbg !20294

panic5.i:                                         ; preds = %bb2.i1966
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_24.i, i32 noundef %_128.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70ebf0cf1c90c6161926611ccf249a32) #32, !dbg !20293, !noalias !20289
  unreachable, !dbg !20293

bb10.i1969:                                       ; preds = %bb9.i
  %255 = getelementptr inbounds nuw i32, ptr %_130.0.i, i32 %iter.sroa.4.0.i7025, !dbg !20294
  %_30.i1970 = load i32, ptr %255, align 4, !dbg !20294, !noalias !20289, !noundef !10
  %256 = icmp eq i32 %_30.i1970, 0, !dbg !20295
  br i1 %256, label %bb14.i, label %bb12.i1971, !dbg !20295

panic6.i:                                         ; preds = %bb9.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_130.1.i, i32 noundef %_130.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_023d591aaad7f5cf482ac80a9401eca1) #32, !dbg !20294, !noalias !20289
  unreachable, !dbg !20294

bb12.i1971:                                       ; preds = %bb10.i1969
  %_35.i1972 = icmp ult i32 %iter.sroa.4.0.i7025, %_132.1.i, !dbg !20296
  br i1 %_35.i1972, label %bb13.i1973, label %panic7.i, !dbg !20296

bb14.i:                                           ; preds = %bb34.i, %bb13.i1973, %bb10.i1969
  %newest.sroa.0.0.i = phi float [ %254, %bb10.i1969 ], [ %_33.i1974, %bb34.i ], [ %254, %bb13.i1973 ], !dbg !20297
  %exitcond12502.not = icmp eq i32 %iter.sroa.4.0.i7025, %_132.1.i, !dbg !20298
  br i1 %exitcond12502.not, label %panic8.i, label %bb15.i1975, !dbg !20298

bb13.i1973:                                       ; preds = %bb12.i1971
  %257 = getelementptr inbounds nuw float, ptr %_132.0.i, i32 %iter.sroa.4.0.i7025, !dbg !20296
  %_33.i1974 = load float, ptr %257, align 4, !dbg !20296, !noalias !20289, !noundef !10
  %_116.i = fcmp olt float %_33.i1974, %254, !dbg !20299
  br i1 %_116.i, label %bb34.i, label %bb14.i, !dbg !20299

panic7.i:                                         ; preds = %bb12.i1971
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %iter.sroa.4.0.i7025, i32 noundef %_132.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a228cac3279aae3a34886f59f51fbe9e) #32, !dbg !20296, !noalias !20289
  unreachable, !dbg !20296

bb34.i:                                           ; preds = %bb13.i1973
  br label %bb14.i, !dbg !20301

bb15.i1975:                                       ; preds = %bb14.i
  %258 = getelementptr inbounds nuw float, ptr %_132.0.i, i32 %iter.sroa.4.0.i7025, !dbg !20298
  store float %newest.sroa.0.0.i, ptr %258, align 4, !dbg !20298, !noalias !20289
  %_40.i = add i32 %_30.i1970, 1, !dbg !20302
  %complete.i1976 = icmp eq i32 %_40.i, %shape.i, !dbg !20302
  br i1 %complete.i1976, label %bb19.i1980, label %bb17.i, !dbg !20303

panic8.i:                                         ; preds = %bb14.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_132.1.i, i32 noundef %_132.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1fa5d88c1a2cc292a2767c48606a38fe) #32, !dbg !20298, !noalias !20289
  unreachable, !dbg !20298

bb17.i:                                           ; preds = %bb15.i1975
  %_42.i = add i32 %iter.sroa.4.0.i7025, %_43.i, !dbg !20304
  %_45.i = icmp ult i32 %_42.i, %_128.1.i, !dbg !20305
  br i1 %_45.i, label %bb27.i, label %panic9.i, !dbg !20305

panic9.i:                                         ; preds = %bb17.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_42.i, i32 noundef %_128.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70d40ae2302f3ea19fa0c4a65fe82786) #32, !dbg !20305, !noalias !20289
  unreachable, !dbg !20305

bb27.i:                                           ; preds = %bb17.i
  %259 = getelementptr inbounds nuw float, ptr %_128.0.i, i32 %_42.i, !dbg !20305
  %_41.i = load float, ptr %259, align 4, !dbg !20305, !noalias !20289, !noundef !10
  %_117.i = fcmp olt float %_41.i, %newest.sroa.0.0.i, !dbg !20306
  %newest.sroa.0.1.i = select i1 %_117.i, float %_41.i, float %newest.sroa.0.0.i, !dbg !20306
  store float %newest.sroa.0.1.i, ptr %iter.sroa.0.0.ptr.i7027, align 4, !dbg !20308, !alias.scope !20272, !noalias !20309
  br label %bb28.i, !dbg !20310

bb28.i:                                           ; preds = %bb22.i, %bb19.i1980, %bb27.i
  %storemerge5356 = phi i32 [ %_40.i, %bb27.i ], [ 0, %bb19.i1980 ], [ 0, %bb22.i ], !dbg !20311
  store i32 %storemerge5356, ptr %255, align 4, !dbg !20311, !noalias !20289
  %260 = icmp eq i32 %248, 0, !dbg !20277
  br i1 %260, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit, label %bb29.i, !dbg !20277

bb19.i1980:                                       ; preds = %bb15.i1975
  store float %newest.sroa.0.0.i, ptr %iter.sroa.0.0.ptr.i7027, align 4, !dbg !20308, !alias.scope !20272, !noalias !20309
  %_118.i7020.not = icmp eq i32 %shape.i, 0, !dbg !20312
  br i1 %_118.i7020.not, label %bb28.i, label %bb40.i.preheader, !dbg !20316

bb40.i.preheader:                                 ; preds = %bb19.i1980
  %261 = load float, ptr %253, align 4, !dbg !20317, !noalias !20289, !noundef !10
  br label %bb40.i, !dbg !20318

bb40.i:                                           ; preds = %bb40.i.preheader, %bb22.i
  %iter2.sroa.0.0.i19837023 = phi i32 [ %_119.i1985, %bb22.i ], [ 0, %bb40.i.preheader ]
  %suffix.sroa.0.0.i19827022 = phi float [ %suffix.sroa.0.1.i, %bb22.i ], [ %261, %bb40.i.preheader ]
  %end.sroa.0.1.i7021 = phi i32 [ %264, %bb22.i ], [ %spec.select.i, %bb40.i.preheader ]
  %_54.i = mul i32 %end.sroa.0.1.i7021, %width.i, !dbg !20319
  %_53.i = add i32 %_54.i, %iter.sroa.4.0.i7025, !dbg !20319
  %_57.i = icmp ult i32 %_53.i, %_128.1.i, !dbg !20318
  br i1 %_57.i, label %bb22.i, label %panic13.i, !dbg !20318

panic13.i:                                        ; preds = %bb40.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_53.i, i32 noundef %_128.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a271bbb7fd83c3605ea58a06b7065fa4) #32, !dbg !20318, !noalias !20289
  unreachable, !dbg !20318

bb22.i:                                           ; preds = %bb40.i
  %_119.i1985 = add nuw i32 %iter2.sroa.0.0.i19837023, 1, !dbg !20320
  %262 = getelementptr inbounds nuw float, ptr %_128.0.i, i32 %_53.i, !dbg !20318
  %_52.i1986 = load float, ptr %262, align 4, !dbg !20318, !noalias !20289, !noundef !10
  %_121.i = fcmp olt float %suffix.sroa.0.0.i19827022, %_52.i1986, !dbg !20323
  %suffix.sroa.0.1.i = select i1 %_121.i, float %suffix.sroa.0.0.i19827022, float %_52.i1986, !dbg !20323
  store float %suffix.sroa.0.1.i, ptr %262, align 4, !dbg !20325, !noalias !20289
  %263 = icmp eq i32 %end.sroa.0.1.i7021, 0, !dbg !20326
  %spec.store.select.i1988 = select i1 %263, i32 %_91.i1468, i32 %end.sroa.0.1.i7021, !dbg !20326
  %264 = add i32 %spec.store.select.i1988, -1, !dbg !20327
  %exitcond12499.not = icmp eq i32 %_119.i1985, %shape.i, !dbg !20312
  br i1 %exitcond12499.not, label %bb28.i, label %bb40.i, !dbg !20316

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit: ; preds = %bb28.i, %bb29.i
  %_0.i3432.pre = load float, ptr %scratch.i1016, align 4, !dbg !20328, !alias.scope !20330, !noalias !20333
  br label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit, !dbg !20328

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit: ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3597
  %_0.i3432 = phi float [ %_0.i3432.pre, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit ], [ %_0.i3444, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3597 ], !dbg !20328
  %_0.i2933 = fmul float %_0.i3432, 1.638400e+04, !dbg !20334
  %265 = tail call noundef float @llvm.floor.f32(float %_0.i2933), !dbg !20336
  %_0.i2932 = fmul float %265, 0x3F10000000000000, !dbg !20340
  %266 = icmp eq i32 %width.i.i1562, 0, !dbg !20342
  %_163.1.i.i1611.pre = load i32, ptr %157, align 4, !dbg !20344, !alias.scope !20243, !noalias !20244
  br i1 %266, label %bb53.i.i1606, label %bb36.i.i1585.lr.ph, !dbg !20342

bb36.i.i1585.lr.ph:                               ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit
  %_159.1.i.i1590 = load i32, ptr %151, align 4, !alias.scope !20243, !noalias !20244, !noundef !10
  %_159.0.i.i1594 = load ptr, ptr %152, align 4, !nonnull !10
  %_161.0.i.i1604 = load ptr, ptr %158, align 4, !nonnull !10
  %exitcond12503.not = icmp eq i32 %_159.1.i.i1590, 0, !dbg !20345
  br i1 %exitcond12503.not, label %panic.i.i1592, label %bb14.i.i1593, !dbg !20345

bb34.i.i1659:                                     ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3437
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i1568, i32 noundef %_158.1.i.i1567, i32 noundef %_158.1.i.i1567, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_db7c42041d7aaaba4839906f37294547) #32, !dbg !20346, !noalias !20267
  unreachable, !dbg !20346

bb53.i.i1606:                                     ; preds = %bb18.i.i1603.7, %bb18.i.i1603, %bb18.i.i1603.1, %bb18.i.i1603.2, %bb18.i.i1603.3, %bb18.i.i1603.4, %bb18.i.i1603.5, %bb18.i.i1603.6, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit
  %_0.i3430 = phi float [ %_0.i3432, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit ], [ %_47.i.i1605, %bb18.i.i1603 ], [ %_47.i.i1605, %bb18.i.i1603.7 ], [ %_47.i.i1605, %bb18.i.i1603.6 ], [ %_47.i.i1605, %bb18.i.i1603.5 ], [ %_47.i.i1605, %bb18.i.i1603.4 ], [ %_47.i.i1605, %bb18.i.i1603.3 ], [ %_47.i.i1605, %bb18.i.i1603.2 ], [ %_47.i.i1605, %bb18.i.i1603.1 ], !dbg !20347
  %_0.i2507 = fadd float %_0.i2932, %_0.i33508008, !dbg !20349
  %_0.i3350 = fsub float %_0.i2507, %_0.i3430, !dbg !20351
  %_123.i.i1612 = icmp ugt i32 %_22.i.i1568, %_163.1.i.i1611.pre, !dbg !20353
  br i1 %_123.i.i1612, label %bb41.i.i1658, label %bb42.i.i1613, !dbg !20353, !prof !787

bb14.i.i1593:                                     ; preds = %bb36.i.i1585.lr.ph
  %267 = getelementptr inbounds nuw i8, ptr %_159.0.i.i1594, i32 8, !dbg !20345
  %_42.i.i1595 = load i32, ptr %267, align 4, !dbg !20345, !noalias !20333, !noundef !10
  %268 = add i32 %_42.i.i1595, %ring_cursor.sroa.0.1.i14447034, !dbg !20356
  %_45.not.i.i1596 = icmp ult i32 %268, %_91.i1468, !dbg !20357
  %269 = select i1 %_45.not.i.i1596, i32 0, i32 %_91.i1468, !dbg !20357
  %spec.select.i.i1597 = sub nuw i32 %268, %269, !dbg !20357
  %_49.i.i1598 = mul i32 %spec.select.i.i1597, %width.i.i1562, !dbg !20358
  %_51.i.i1601 = icmp ult i32 %_49.i.i1598, %_163.1.i.i1611.pre, !dbg !20359
  br i1 %_51.i.i1601, label %bb18.i.i1603, label %panic1.i.i1602, !dbg !20359

panic.i.i1592:                                    ; preds = %bb36.i.i1585.7, %bb36.i.i1585.6, %bb36.i.i1585.5, %bb36.i.i1585.4, %bb36.i.i1585.3, %bb36.i.i1585.2, %bb36.i.i1585.1, %bb36.i.i1585.lr.ph
  %_159.1.i.i1590.lcssa.ph = phi i32 [ 7, %bb36.i.i1585.7 ], [ 6, %bb36.i.i1585.6 ], [ 5, %bb36.i.i1585.5 ], [ 4, %bb36.i.i1585.4 ], [ 3, %bb36.i.i1585.3 ], [ 2, %bb36.i.i1585.2 ], [ 1, %bb36.i.i1585.1 ], [ 0, %bb36.i.i1585.lr.ph ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_159.1.i.i1590.lcssa.ph, i32 noundef %_159.1.i.i1590.lcssa.ph, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_33746731a929bad63709ddedbca82f5f) #32, !dbg !20345, !noalias !20333
  unreachable, !dbg !20345

bb18.i.i1603:                                     ; preds = %bb14.i.i1593
  %270 = getelementptr inbounds nuw float, ptr %_161.0.i.i1604, i32 %_49.i.i1598, !dbg !20359
  %_47.i.i1605 = load float, ptr %270, align 4, !dbg !20359, !noalias !20333, !noundef !10
  store float %_47.i.i1605, ptr %scratch.i1016, align 4, !dbg !20360, !alias.scope !20240, !noalias !20361
  %271 = icmp eq i32 %width.i.i1562, 1, !dbg !20342
  br i1 %271, label %bb53.i.i1606, label %bb36.i.i1585.1, !dbg !20342

bb36.i.i1585.1:                                   ; preds = %bb18.i.i1603
  %exitcond12503.1.not = icmp eq i32 %_159.1.i.i1590, 1, !dbg !20345
  br i1 %exitcond12503.1.not, label %panic.i.i1592, label %bb14.i.i1593.1, !dbg !20345

bb14.i.i1593.1:                                   ; preds = %bb36.i.i1585.1
  %272 = getelementptr inbounds nuw i8, ptr %_159.0.i.i1594, i32 20, !dbg !20345
  %_42.i.i1595.1 = load i32, ptr %272, align 4, !dbg !20345, !noalias !20333, !noundef !10
  %273 = add i32 %_42.i.i1595.1, %ring_cursor.sroa.0.1.i14447034, !dbg !20356
  %_45.not.i.i1596.1 = icmp ult i32 %273, %_91.i1468, !dbg !20357
  %274 = select i1 %_45.not.i.i1596.1, i32 0, i32 %_91.i1468, !dbg !20357
  %spec.select.i.i1597.1 = sub nuw i32 %273, %274, !dbg !20357
  %_49.i.i1598.1 = mul i32 %spec.select.i.i1597.1, %width.i.i1562, !dbg !20358
  %_48.i.i1599.1 = add i32 %_49.i.i1598.1, 1, !dbg !20358
  %_51.i.i1601.1 = icmp ult i32 %_48.i.i1599.1, %_163.1.i.i1611.pre, !dbg !20359
  br i1 %_51.i.i1601.1, label %bb18.i.i1603.1, label %panic1.i.i1602, !dbg !20359

bb18.i.i1603.1:                                   ; preds = %bb14.i.i1593.1
  %275 = getelementptr inbounds nuw float, ptr %_161.0.i.i1604, i32 %_48.i.i1599.1, !dbg !20359
  %_47.i.i1605.1 = load float, ptr %275, align 4, !dbg !20359, !noalias !20333, !noundef !10
  store float %_47.i.i1605.1, ptr %iter.sroa.0.0.ptr.i.i15847031.1, align 4, !dbg !20360, !alias.scope !20240, !noalias !20361
  %276 = icmp eq i32 %width.i.i1562, 2, !dbg !20342
  br i1 %276, label %bb53.i.i1606, label %bb36.i.i1585.2, !dbg !20342

bb36.i.i1585.2:                                   ; preds = %bb18.i.i1603.1
  %exitcond12503.2.not = icmp eq i32 %_159.1.i.i1590, 2, !dbg !20345
  br i1 %exitcond12503.2.not, label %panic.i.i1592, label %bb14.i.i1593.2, !dbg !20345

bb14.i.i1593.2:                                   ; preds = %bb36.i.i1585.2
  %277 = getelementptr inbounds nuw i8, ptr %_159.0.i.i1594, i32 32, !dbg !20345
  %_42.i.i1595.2 = load i32, ptr %277, align 4, !dbg !20345, !noalias !20333, !noundef !10
  %278 = add i32 %_42.i.i1595.2, %ring_cursor.sroa.0.1.i14447034, !dbg !20356
  %_45.not.i.i1596.2 = icmp ult i32 %278, %_91.i1468, !dbg !20357
  %279 = select i1 %_45.not.i.i1596.2, i32 0, i32 %_91.i1468, !dbg !20357
  %spec.select.i.i1597.2 = sub nuw i32 %278, %279, !dbg !20357
  %_49.i.i1598.2 = mul i32 %spec.select.i.i1597.2, %width.i.i1562, !dbg !20358
  %_48.i.i1599.2 = add i32 %_49.i.i1598.2, 2, !dbg !20358
  %_51.i.i1601.2 = icmp ult i32 %_48.i.i1599.2, %_163.1.i.i1611.pre, !dbg !20359
  br i1 %_51.i.i1601.2, label %bb18.i.i1603.2, label %panic1.i.i1602, !dbg !20359

bb18.i.i1603.2:                                   ; preds = %bb14.i.i1593.2
  %280 = getelementptr inbounds nuw float, ptr %_161.0.i.i1604, i32 %_48.i.i1599.2, !dbg !20359
  %_47.i.i1605.2 = load float, ptr %280, align 4, !dbg !20359, !noalias !20333, !noundef !10
  store float %_47.i.i1605.2, ptr %iter.sroa.0.0.ptr.i.i15847031.2, align 4, !dbg !20360, !alias.scope !20240, !noalias !20361
  %281 = icmp eq i32 %width.i.i1562, 3, !dbg !20342
  br i1 %281, label %bb53.i.i1606, label %bb36.i.i1585.3, !dbg !20342

bb36.i.i1585.3:                                   ; preds = %bb18.i.i1603.2
  %exitcond12503.3.not = icmp eq i32 %_159.1.i.i1590, 3, !dbg !20345
  br i1 %exitcond12503.3.not, label %panic.i.i1592, label %bb14.i.i1593.3, !dbg !20345

bb14.i.i1593.3:                                   ; preds = %bb36.i.i1585.3
  %282 = getelementptr inbounds nuw i8, ptr %_159.0.i.i1594, i32 44, !dbg !20345
  %_42.i.i1595.3 = load i32, ptr %282, align 4, !dbg !20345, !noalias !20333, !noundef !10
  %283 = add i32 %_42.i.i1595.3, %ring_cursor.sroa.0.1.i14447034, !dbg !20356
  %_45.not.i.i1596.3 = icmp ult i32 %283, %_91.i1468, !dbg !20357
  %284 = select i1 %_45.not.i.i1596.3, i32 0, i32 %_91.i1468, !dbg !20357
  %spec.select.i.i1597.3 = sub nuw i32 %283, %284, !dbg !20357
  %_49.i.i1598.3 = mul i32 %spec.select.i.i1597.3, %width.i.i1562, !dbg !20358
  %_48.i.i1599.3 = add i32 %_49.i.i1598.3, 3, !dbg !20358
  %_51.i.i1601.3 = icmp ult i32 %_48.i.i1599.3, %_163.1.i.i1611.pre, !dbg !20359
  br i1 %_51.i.i1601.3, label %bb18.i.i1603.3, label %panic1.i.i1602, !dbg !20359

bb18.i.i1603.3:                                   ; preds = %bb14.i.i1593.3
  %285 = getelementptr inbounds nuw float, ptr %_161.0.i.i1604, i32 %_48.i.i1599.3, !dbg !20359
  %_47.i.i1605.3 = load float, ptr %285, align 4, !dbg !20359, !noalias !20333, !noundef !10
  store float %_47.i.i1605.3, ptr %iter.sroa.0.0.ptr.i.i15847031.3, align 4, !dbg !20360, !alias.scope !20240, !noalias !20361
  %286 = icmp eq i32 %width.i.i1562, 4, !dbg !20342
  br i1 %286, label %bb53.i.i1606, label %bb36.i.i1585.4, !dbg !20342

bb36.i.i1585.4:                                   ; preds = %bb18.i.i1603.3
  %exitcond12503.4.not = icmp eq i32 %_159.1.i.i1590, 4, !dbg !20345
  br i1 %exitcond12503.4.not, label %panic.i.i1592, label %bb14.i.i1593.4, !dbg !20345

bb14.i.i1593.4:                                   ; preds = %bb36.i.i1585.4
  %287 = getelementptr inbounds nuw i8, ptr %_159.0.i.i1594, i32 56, !dbg !20345
  %_42.i.i1595.4 = load i32, ptr %287, align 4, !dbg !20345, !noalias !20333, !noundef !10
  %288 = add i32 %_42.i.i1595.4, %ring_cursor.sroa.0.1.i14447034, !dbg !20356
  %_45.not.i.i1596.4 = icmp ult i32 %288, %_91.i1468, !dbg !20357
  %289 = select i1 %_45.not.i.i1596.4, i32 0, i32 %_91.i1468, !dbg !20357
  %spec.select.i.i1597.4 = sub nuw i32 %288, %289, !dbg !20357
  %_49.i.i1598.4 = mul i32 %spec.select.i.i1597.4, %width.i.i1562, !dbg !20358
  %_48.i.i1599.4 = add i32 %_49.i.i1598.4, 4, !dbg !20358
  %_51.i.i1601.4 = icmp ult i32 %_48.i.i1599.4, %_163.1.i.i1611.pre, !dbg !20359
  br i1 %_51.i.i1601.4, label %bb18.i.i1603.4, label %panic1.i.i1602, !dbg !20359

bb18.i.i1603.4:                                   ; preds = %bb14.i.i1593.4
  %290 = getelementptr inbounds nuw float, ptr %_161.0.i.i1604, i32 %_48.i.i1599.4, !dbg !20359
  %_47.i.i1605.4 = load float, ptr %290, align 4, !dbg !20359, !noalias !20333, !noundef !10
  store float %_47.i.i1605.4, ptr %iter.sroa.0.0.ptr.i.i15847031.4, align 4, !dbg !20360, !alias.scope !20240, !noalias !20361
  %291 = icmp eq i32 %width.i.i1562, 5, !dbg !20342
  br i1 %291, label %bb53.i.i1606, label %bb36.i.i1585.5, !dbg !20342

bb36.i.i1585.5:                                   ; preds = %bb18.i.i1603.4
  %exitcond12503.5.not = icmp eq i32 %_159.1.i.i1590, 5, !dbg !20345
  br i1 %exitcond12503.5.not, label %panic.i.i1592, label %bb14.i.i1593.5, !dbg !20345

bb14.i.i1593.5:                                   ; preds = %bb36.i.i1585.5
  %292 = getelementptr inbounds nuw i8, ptr %_159.0.i.i1594, i32 68, !dbg !20345
  %_42.i.i1595.5 = load i32, ptr %292, align 4, !dbg !20345, !noalias !20333, !noundef !10
  %293 = add i32 %_42.i.i1595.5, %ring_cursor.sroa.0.1.i14447034, !dbg !20356
  %_45.not.i.i1596.5 = icmp ult i32 %293, %_91.i1468, !dbg !20357
  %294 = select i1 %_45.not.i.i1596.5, i32 0, i32 %_91.i1468, !dbg !20357
  %spec.select.i.i1597.5 = sub nuw i32 %293, %294, !dbg !20357
  %_49.i.i1598.5 = mul i32 %spec.select.i.i1597.5, %width.i.i1562, !dbg !20358
  %_48.i.i1599.5 = add i32 %_49.i.i1598.5, 5, !dbg !20358
  %_51.i.i1601.5 = icmp ult i32 %_48.i.i1599.5, %_163.1.i.i1611.pre, !dbg !20359
  br i1 %_51.i.i1601.5, label %bb18.i.i1603.5, label %panic1.i.i1602, !dbg !20359

bb18.i.i1603.5:                                   ; preds = %bb14.i.i1593.5
  %295 = getelementptr inbounds nuw float, ptr %_161.0.i.i1604, i32 %_48.i.i1599.5, !dbg !20359
  %_47.i.i1605.5 = load float, ptr %295, align 4, !dbg !20359, !noalias !20333, !noundef !10
  store float %_47.i.i1605.5, ptr %iter.sroa.0.0.ptr.i.i15847031.5, align 4, !dbg !20360, !alias.scope !20240, !noalias !20361
  %296 = icmp eq i32 %width.i.i1562, 6, !dbg !20342
  br i1 %296, label %bb53.i.i1606, label %bb36.i.i1585.6, !dbg !20342

bb36.i.i1585.6:                                   ; preds = %bb18.i.i1603.5
  %exitcond12503.6.not = icmp eq i32 %_159.1.i.i1590, 6, !dbg !20345
  br i1 %exitcond12503.6.not, label %panic.i.i1592, label %bb14.i.i1593.6, !dbg !20345

bb14.i.i1593.6:                                   ; preds = %bb36.i.i1585.6
  %297 = getelementptr inbounds nuw i8, ptr %_159.0.i.i1594, i32 80, !dbg !20345
  %_42.i.i1595.6 = load i32, ptr %297, align 4, !dbg !20345, !noalias !20333, !noundef !10
  %298 = add i32 %_42.i.i1595.6, %ring_cursor.sroa.0.1.i14447034, !dbg !20356
  %_45.not.i.i1596.6 = icmp ult i32 %298, %_91.i1468, !dbg !20357
  %299 = select i1 %_45.not.i.i1596.6, i32 0, i32 %_91.i1468, !dbg !20357
  %spec.select.i.i1597.6 = sub nuw i32 %298, %299, !dbg !20357
  %_49.i.i1598.6 = mul i32 %spec.select.i.i1597.6, %width.i.i1562, !dbg !20358
  %_48.i.i1599.6 = add i32 %_49.i.i1598.6, 6, !dbg !20358
  %_51.i.i1601.6 = icmp ult i32 %_48.i.i1599.6, %_163.1.i.i1611.pre, !dbg !20359
  br i1 %_51.i.i1601.6, label %bb18.i.i1603.6, label %panic1.i.i1602, !dbg !20359

bb18.i.i1603.6:                                   ; preds = %bb14.i.i1593.6
  %300 = getelementptr inbounds nuw float, ptr %_161.0.i.i1604, i32 %_48.i.i1599.6, !dbg !20359
  %_47.i.i1605.6 = load float, ptr %300, align 4, !dbg !20359, !noalias !20333, !noundef !10
  store float %_47.i.i1605.6, ptr %iter.sroa.0.0.ptr.i.i15847031.6, align 4, !dbg !20360, !alias.scope !20240, !noalias !20361
  %301 = icmp eq i32 %width.i.i1562, 7, !dbg !20342
  br i1 %301, label %bb53.i.i1606, label %bb36.i.i1585.7, !dbg !20342

bb36.i.i1585.7:                                   ; preds = %bb18.i.i1603.6
  %exitcond12503.7.not = icmp eq i32 %_159.1.i.i1590, 7, !dbg !20345
  br i1 %exitcond12503.7.not, label %panic.i.i1592, label %bb14.i.i1593.7, !dbg !20345

bb14.i.i1593.7:                                   ; preds = %bb36.i.i1585.7
  %302 = getelementptr inbounds nuw i8, ptr %_159.0.i.i1594, i32 92, !dbg !20345
  %_42.i.i1595.7 = load i32, ptr %302, align 4, !dbg !20345, !noalias !20333, !noundef !10
  %303 = add i32 %_42.i.i1595.7, %ring_cursor.sroa.0.1.i14447034, !dbg !20356
  %_45.not.i.i1596.7 = icmp ult i32 %303, %_91.i1468, !dbg !20357
  %304 = select i1 %_45.not.i.i1596.7, i32 0, i32 %_91.i1468, !dbg !20357
  %spec.select.i.i1597.7 = sub nuw i32 %303, %304, !dbg !20357
  %_49.i.i1598.7 = mul i32 %spec.select.i.i1597.7, %width.i.i1562, !dbg !20358
  %_48.i.i1599.7 = add i32 %_49.i.i1598.7, 7, !dbg !20358
  %_51.i.i1601.7 = icmp ult i32 %_48.i.i1599.7, %_163.1.i.i1611.pre, !dbg !20359
  br i1 %_51.i.i1601.7, label %bb18.i.i1603.7, label %panic1.i.i1602, !dbg !20359

bb18.i.i1603.7:                                   ; preds = %bb14.i.i1593.7
  %305 = getelementptr inbounds nuw float, ptr %_161.0.i.i1604, i32 %_48.i.i1599.7, !dbg !20359
  %_47.i.i1605.7 = load float, ptr %305, align 4, !dbg !20359, !noalias !20333, !noundef !10
  store float %_47.i.i1605.7, ptr %iter.sroa.0.0.ptr.i.i15847031.7, align 4, !dbg !20360, !alias.scope !20240, !noalias !20361
  br label %bb53.i.i1606, !dbg !20342

panic1.i.i1602:                                   ; preds = %bb14.i.i1593.7, %bb14.i.i1593.6, %bb14.i.i1593.5, %bb14.i.i1593.4, %bb14.i.i1593.3, %bb14.i.i1593.2, %bb14.i.i1593.1, %bb14.i.i1593
  %_48.i.i1599.lcssa.ph = phi i32 [ %_48.i.i1599.7, %bb14.i.i1593.7 ], [ %_48.i.i1599.6, %bb14.i.i1593.6 ], [ %_48.i.i1599.5, %bb14.i.i1593.5 ], [ %_48.i.i1599.4, %bb14.i.i1593.4 ], [ %_48.i.i1599.3, %bb14.i.i1593.3 ], [ %_48.i.i1599.2, %bb14.i.i1593.2 ], [ %_48.i.i1599.1, %bb14.i.i1593.1 ], [ %_49.i.i1598, %bb14.i.i1593 ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_48.i.i1599.lcssa.ph, i32 noundef %_163.1.i.i1611.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_0e76a26fbb751dfb8cf8a069b5b0d39a) #32, !dbg !20359, !noalias !20333
  unreachable, !dbg !20359

bb42.i.i1613:                                     ; preds = %bb53.i.i1606
  %_163.0.i.i1614 = load ptr, ptr %158, align 4, !dbg !20344, !alias.scope !20243, !noalias !20244, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20362), !dbg !20365
  %_4.not.i3590 = icmp eq i32 %_163.1.i.i1611.pre, %_22.i.i1568, !dbg !20366
  br i1 %_4.not.i3590, label %panic.i3592, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3593, !dbg !20366

panic.i3592:                                      ; preds = %bb42.i.i1613
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #32, !dbg !20366, !noalias !20368
  unreachable, !dbg !20366

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3593: ; preds = %bb42.i.i1613
  %_130.i.i1616 = getelementptr inbounds nuw float, ptr %_163.0.i.i1614, i32 %_22.i.i1568, !dbg !20369
  store float %_0.i2932, ptr %_130.i.i1616, align 4, !dbg !20366, !alias.scope !20362, !noalias !20333
  %_0.i2899 = fdiv float %_0.i3350, %_62.i.i1618, !dbg !20371
  %_0.i3349 = fsub float 1.000000e+00, %_0.i2899, !dbg !20373
  %_0.i3348 = fsub float %_0.i3349, %_0.i37248077, !dbg !20375
  %_4.i2915 = fmul float %_0.i3854, %_0.i3348, !dbg !20377
  %_0.i2916 = fadd float %_0.i37248077, %_4.i2915, !dbg !20377
  %_3.i.i4065.inv = fcmp ogt float %_0.i3349, %_0.i2916, !dbg !20379
  %_4.i.i.v = select i1 %_3.i.i4065.inv, float %_0.i3349, float %_0.i2916, !dbg !20379
  %306 = tail call noundef float @llvm.fabs.f32(float %_4.i.i.v), !dbg !20382
  %307 = fcmp uge float %306, 0x3BC79CA100000000, !dbg !20385
  %_0.i3724 = select i1 %307, float %_4.i.i.v, float 0.000000e+00, !dbg !20387
  %_0.i3347 = fsub float 1.000000e+00, %_0.i3724, !dbg !20388
  %_164.1.i.i1630 = load i32, ptr %162, align 4, !dbg !20390, !alias.scope !20243, !noalias !20244, !noundef !10
  %_74.i.i1631 = mul i32 %width.i.i1562, %main_cursor.sroa.0.1.i14457035, !dbg !20391
  %_134.i.i1632 = icmp ugt i32 %_74.i.i1631, %_164.1.i.i1630, !dbg !20392
  br i1 %_134.i.i1632, label %bb47.i.i1657, label %bb48.i.i1633, !dbg !20392, !prof !787

bb41.i.i1658:                                     ; preds = %bb53.i.i1606
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i1568, i32 noundef %_163.1.i.i1611.pre, i32 noundef %_163.1.i.i1611.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c2286ff41a33b8c11aa270fe215441de) #32, !dbg !20395, !noalias !20333
  unreachable, !dbg !20395

bb48.i.i1633:                                     ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3593
  %_164.0.i.i1634 = load ptr, ptr %163, align 4, !dbg !20390, !alias.scope !20243, !noalias !20244, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20396), !dbg !20399
  %_3.not.i = icmp eq i32 %_164.1.i.i1630, %_74.i.i1631, !dbg !20400
  br i1 %_3.not.i, label %panic.i3428, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit, !dbg !20400

panic.i3428:                                      ; preds = %bb48.i.i1633
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #32, !dbg !20400, !noalias !20402
  unreachable, !dbg !20400

bb47.i.i1657:                                     ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3593
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_74.i.i1631, i32 noundef %_164.1.i.i1630, i32 noundef %_164.1.i.i1630, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a3a21efd18b403ec25db545d522c479) #32, !dbg !20403, !noalias !20333
  unreachable, !dbg !20403

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit: ; preds = %bb48.i.i1633
  %_141.i.i1636 = getelementptr inbounds nuw float, ptr %_164.0.i.i1634, i32 %_74.i.i1631, !dbg !20404
  %_0.i = load float, ptr %_141.i.i1636, align 4, !dbg !20400, !alias.scope !20396, !noalias !20333, !noundef !10
  store float %_0.i3435, ptr %_141.i.i1636, align 4, !dbg !20406, !alias.scope !20408, !noalias !20333
  %_0.i2931 = fmul float %_0.i3347, %_0.i, !dbg !20411
  %_6.i3855 = bitcast float %_0.i to i32, !dbg !20413
  %_5.i3856 = and i32 %_6.i3855, %all.sroa.0.0.i1025, !dbg !20416
  %_8.i3857 = bitcast float %_0.i2931 to i32, !dbg !20417
  %_7.i3858 = and i32 %_9.i3870, %_8.i3857, !dbg !20419
  %_4.i3859 = or disjoint i32 %_7.i3858, %_5.i3856, !dbg !20416
  store i32 %_4.i3859, ptr %_178.i1560, align 4, !dbg !20420, !alias.scope !20422, !noalias !20425
  %308 = add i32 %main_cursor.sroa.0.1.i14457035, 1, !dbg !20426
  %_104.i1651 = icmp eq i32 %308, %_106.i1650, !dbg !20427
  %spec.store.select11.i1652 = select i1 %_104.i1651, i32 0, i32 %308, !dbg !20427
  %309 = add i32 %ring_cursor.sroa.0.1.i14447034, 1, !dbg !20428
  %_107.i1653 = icmp eq i32 %309, %_91.i1468, !dbg !20429
  %spec.store.select12.i1654 = select i1 %_107.i1653, i32 0, i32 %309, !dbg !20429
  %exitcond12506.not = icmp eq i32 %181, %umax12505, !dbg !20430
  br i1 %exitcond12506.not, label %bb16.i1442.bb13.i1035.loopexit_crit_edge, label %bb50.i1447, !dbg !19297

bb56.i1660:                                       ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3601
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_64.i1448, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_7e50e0c93e87b60da469fc303486c501) #32, !dbg !20433, !noalias !19254
  unreachable, !dbg !20433

_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit: ; preds = %bb13.i1035.loopexit, %bb12.i
  %ring_cursor.sroa.0.0.i1038.lcssa = phi i32 [ %_37.i1027, %bb12.i ], [ %ring_cursor.sroa.0.1.i1444.lcssa, %bb13.i1035.loopexit ], !dbg !19199
  %main_cursor.sroa.0.0.i1039.lcssa = phi i32 [ %_35.i1026, %bb12.i ], [ %main_cursor.sroa.0.1.i1445.lcssa, %bb13.i1035.loopexit ], !dbg !19196
  call void @llvm.lifetime.start.p0(ptr nonnull %_110.i1013), !dbg !20434, !noalias !19179
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(92) %_110.i1013, ptr noundef nonnull align 4 dereferenceable(92) %hot_left.i1018, i32 92, i1 false), !dbg !20434, !noalias !19179
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_110.i1013, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33) #31, !dbg !20435, !noalias !19254
  call void @llvm.lifetime.end.p0(ptr nonnull %_110.i1013), !dbg !20436, !noalias !19179
  call void @llvm.lifetime.start.p0(ptr nonnull %_112.i1012), !dbg !20437, !noalias !19179
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(92) %_112.i1012, ptr noundef nonnull align 4 dereferenceable(92) %hot_right.i1017, i32 92, i1 false), !dbg !20437, !noalias !19179
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_112.i1012, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34) #31, !dbg !20438, !noalias !19254
  call void @llvm.lifetime.end.p0(ptr nonnull %_112.i1012), !dbg !20439, !noalias !19179
  store i32 %main_cursor.sroa.0.0.i1039.lcssa, ptr %_35, align 4, !dbg !20440, !alias.scope !19173, !noalias !19198
  store i32 %ring_cursor.sroa.0.0.i1038.lcssa, ptr %81, align 4, !dbg !20441, !alias.scope !19173, !noalias !19198
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i1014), !dbg !20442, !noalias !19179
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i1015), !dbg !20443, !noalias !19179
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i1016), !dbg !20444, !noalias !19179
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_right.i1017), !dbg !20445, !noalias !19179
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i1018), !dbg !20446, !noalias !19179
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockfEB2_.exit, !dbg !19168

bb11.i:                                           ; preds = %bb10.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20447), !dbg !20450
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20451), !dbg !20450
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20453), !dbg !20450
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20455), !dbg !20450
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i734), !dbg !20457, !noalias !20461
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_left.i734, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #31, !dbg !20465, !noalias !20466
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_right.i733), !dbg !20467, !noalias !20461
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_right.i733, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #31, !dbg !20469, !noalias !20470
  %310 = load i8, ptr %79, align 4, !dbg !20471, !range !4667, !alias.scope !20447, !noalias !20475, !noundef !10
  %311 = load i8, ptr %80, align 1, !dbg !20476, !range !4667, !alias.scope !20447, !noalias !20475, !noundef !10
  %_35.i = load i32, ptr %_35, align 4, !dbg !20478, !alias.scope !20455, !noalias !20480, !noundef !10
  %_37.i742 = load i32, ptr %81, align 4, !dbg !20481, !alias.scope !20455, !noalias !20480, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i), !dbg !20483, !noalias !20461
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i, i8 0, i32 32, i1 false), !noalias !20461
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i732), !dbg !20485, !noalias !20461
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i732, i8 0, i32 1024, i1 false), !noalias !20461
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i731), !dbg !20487, !noalias !20461
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i731, i8 0, i32 1024, i1 false), !noalias !20461
  %_32.i738 = zext nneg i8 %310 to i32, !dbg !20471
  %.none.i739 = sub nsw i32 0, %_32.i738, !dbg !20489
  %_33.i740 = zext nneg i8 %311 to i32, !dbg !20476
  %all.sroa.0.0.i741 = sub nsw i32 0, %_33.i740, !dbg !20476
  %_115.not.i8512 = icmp eq i32 %frames, 0, !dbg !20490
  br i1 %_115.not.i8512, label %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit, label %bb37.i.lr.ph, !dbg !20490

bb37.i.lr.ph:                                     ; preds = %bb11.i
  %d9.i4568 = lshr i32 %frames, 5, !dbg !20500
  %r2.i4569 = and i32 %frames, 31, !dbg !20507
  %_19.not.i4570 = icmp ne i32 %r2.i4569, 0, !dbg !20508
  %312 = zext i1 %_19.not.i4570 to i32, !dbg !20508
  %yield_count.sroa.0.0.i4571 = add nuw nsw i32 %d9.i4568, %312, !dbg !20508
  %history.i138.i.sroa.7.0.hot_left.i734.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i734, i32 4
  %history.i138.i.sroa.10.0.hot_left.i734.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i734, i32 8
  %history.i138.i.sroa.13.0.hot_left.i734.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i734, i32 12
  %history.i138.i.sroa.16.0.hot_left.i734.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i734, i32 16
  %history.i138.i.sroa.19.0.hot_left.i734.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i734, i32 20
  %history.i138.i.sroa.22.0.hot_left.i734.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i734, i32 24
  %history.i138.i.sroa.26.0.hot_left.i734.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i734, i32 28
  %history.i138.i.sroa.29.0.hot_left.i734.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i734, i32 32
  %history.i138.i.sroa.32.0.hot_left.i734.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i734, i32 36
  %history.i138.i.sroa.35.0.hot_left.i734.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i734, i32 40
  %history.i138.i.sroa.38.0.hot_left.i734.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i734, i32 44
  %313 = getelementptr inbounds nuw i8, ptr %self, i32 132
  %314 = getelementptr inbounds nuw i8, ptr %self, i32 136
  %315 = getelementptr inbounds nuw i8, ptr %self, i32 140
  %row1.i.i.i171.i = getelementptr inbounds nuw i8, ptr %self, i32 144
  %316 = getelementptr inbounds nuw i8, ptr %self, i32 148
  %317 = getelementptr inbounds nuw i8, ptr %self, i32 152
  %318 = getelementptr inbounds nuw i8, ptr %self, i32 156
  %row3.i.i.i185.i = getelementptr inbounds nuw i8, ptr %self, i32 160
  %319 = getelementptr inbounds nuw i8, ptr %self, i32 164
  %320 = getelementptr inbounds nuw i8, ptr %self, i32 168
  %321 = getelementptr inbounds nuw i8, ptr %self, i32 172
  %row5.i.i.i199.i = getelementptr inbounds nuw i8, ptr %self, i32 176
  %322 = getelementptr inbounds nuw i8, ptr %self, i32 180
  %323 = getelementptr inbounds nuw i8, ptr %self, i32 184
  %324 = getelementptr inbounds nuw i8, ptr %self, i32 188
  %row7.i.i.i213.i = getelementptr inbounds nuw i8, ptr %self, i32 192
  %325 = getelementptr inbounds nuw i8, ptr %self, i32 196
  %326 = getelementptr inbounds nuw i8, ptr %self, i32 200
  %327 = getelementptr inbounds nuw i8, ptr %self, i32 204
  %row9.i.i.i227.i = getelementptr inbounds nuw i8, ptr %self, i32 208
  %328 = getelementptr inbounds nuw i8, ptr %self, i32 212
  %329 = getelementptr inbounds nuw i8, ptr %self, i32 216
  %330 = getelementptr inbounds nuw i8, ptr %self, i32 220
  %row11.i.i.i241.i = getelementptr inbounds nuw i8, ptr %self, i32 224
  %331 = getelementptr inbounds nuw i8, ptr %self, i32 228
  %332 = getelementptr inbounds nuw i8, ptr %self, i32 232
  %333 = getelementptr inbounds nuw i8, ptr %self, i32 236
  %row13.i.i.i255.i = getelementptr inbounds nuw i8, ptr %self, i32 240
  %334 = getelementptr inbounds nuw i8, ptr %self, i32 244
  %335 = getelementptr inbounds nuw i8, ptr %self, i32 248
  %336 = getelementptr inbounds nuw i8, ptr %self, i32 252
  %row15.i.i.i269.i = getelementptr inbounds nuw i8, ptr %self, i32 256
  %337 = getelementptr inbounds nuw i8, ptr %self, i32 260
  %338 = getelementptr inbounds nuw i8, ptr %self, i32 264
  %339 = getelementptr inbounds nuw i8, ptr %self, i32 268
  %row17.i.i.i283.i = getelementptr inbounds nuw i8, ptr %self, i32 272
  %340 = getelementptr inbounds nuw i8, ptr %self, i32 276
  %341 = getelementptr inbounds nuw i8, ptr %self, i32 280
  %342 = getelementptr inbounds nuw i8, ptr %self, i32 284
  %row19.i.i.i297.i = getelementptr inbounds nuw i8, ptr %self, i32 288
  %343 = getelementptr inbounds nuw i8, ptr %self, i32 292
  %344 = getelementptr inbounds nuw i8, ptr %self, i32 296
  %345 = getelementptr inbounds nuw i8, ptr %self, i32 300
  %row21.i.i.i311.i = getelementptr inbounds nuw i8, ptr %self, i32 304
  %346 = getelementptr inbounds nuw i8, ptr %self, i32 308
  %347 = getelementptr inbounds nuw i8, ptr %self, i32 312
  %348 = getelementptr inbounds nuw i8, ptr %self, i32 316
  %history.i.i730.sroa.7.0.hot_right.i733.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i733, i32 4
  %history.i.i730.sroa.10.0.hot_right.i733.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i733, i32 8
  %history.i.i730.sroa.13.0.hot_right.i733.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i733, i32 12
  %history.i.i730.sroa.16.0.hot_right.i733.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i733, i32 16
  %history.i.i730.sroa.19.0.hot_right.i733.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i733, i32 20
  %history.i.i730.sroa.22.0.hot_right.i733.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i733, i32 24
  %history.i.i730.sroa.26.0.hot_right.i733.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i733, i32 28
  %history.i.i730.sroa.29.0.hot_right.i733.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i733, i32 32
  %history.i.i730.sroa.32.0.hot_right.i733.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i733, i32 36
  %history.i.i730.sroa.35.0.hot_right.i733.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i733, i32 40
  %history.i.i730.sroa.38.0.hot_right.i733.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i733, i32 44
  %_68.i = getelementptr inbounds nuw i8, ptr %hot_left.i734, i32 48
  %_69.i952 = getelementptr inbounds nuw i8, ptr %hot_left.i734, i32 64
  %_73.i953 = getelementptr inbounds nuw i8, ptr %hot_right.i733, i32 48
  %_74.i = getelementptr inbounds nuw i8, ptr %hot_right.i733, i32 64
  %_9.i3930 = add nsw i32 %_32.i738, -1
  %349 = getelementptr inbounds nuw i8, ptr %self, i32 528
  %350 = getelementptr inbounds nuw i8, ptr %self, i32 420
  %351 = getelementptr inbounds nuw i8, ptr %self, i32 344
  %352 = getelementptr inbounds nuw i8, ptr %self, i32 340
  %353 = getelementptr inbounds nuw i8, ptr %self, i32 384
  %354 = getelementptr inbounds nuw i8, ptr %self, i32 380
  %355 = getelementptr inbounds nuw i8, ptr %self, i32 368
  %356 = getelementptr inbounds nuw i8, ptr %self, i32 364
  %357 = getelementptr inbounds nuw i8, ptr %self, i32 352
  %358 = getelementptr inbounds nuw i8, ptr %self, i32 348
  %359 = getelementptr inbounds nuw i8, ptr %hot_left.i734, i32 84
  %360 = getelementptr inbounds nuw i8, ptr %hot_left.i734, i32 88
  %361 = getelementptr inbounds nuw i8, ptr %hot_left.i734, i32 80
  %362 = getelementptr inbounds nuw i8, ptr %self, i32 336
  %363 = getelementptr inbounds nuw i8, ptr %self, i32 332
  %_9.i3910 = add nsw i32 %_33.i740, -1
  %364 = getelementptr inbounds nuw i8, ptr %self, i32 520
  %365 = getelementptr inbounds nuw i8, ptr %self, i32 444
  %366 = getelementptr inbounds nuw i8, ptr %self, i32 440
  %367 = getelementptr inbounds nuw i8, ptr %self, i32 516
  %368 = getelementptr inbounds nuw i8, ptr %self, i32 512
  %369 = getelementptr inbounds nuw i8, ptr %self, i32 484
  %370 = getelementptr inbounds nuw i8, ptr %self, i32 480
  %371 = getelementptr inbounds nuw i8, ptr %self, i32 468
  %372 = getelementptr inbounds nuw i8, ptr %self, i32 464
  %373 = getelementptr inbounds nuw i8, ptr %self, i32 452
  %374 = getelementptr inbounds nuw i8, ptr %self, i32 448
  %375 = getelementptr inbounds nuw i8, ptr %hot_right.i733, i32 84
  %376 = getelementptr inbounds nuw i8, ptr %hot_right.i733, i32 88
  %377 = getelementptr inbounds nuw i8, ptr %hot_right.i733, i32 80
  %378 = getelementptr inbounds nuw i8, ptr %self, i32 436
  %379 = getelementptr inbounds nuw i8, ptr %self, i32 432
  %380 = getelementptr inbounds nuw i8, ptr %self, i32 532
  %iter.sroa.0.0.ptr.i54.i8217.1 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 4
  %iter.sroa.0.0.ptr.i54.i8217.2 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 8
  %iter.sroa.0.0.ptr.i54.i8217.3 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 12
  %iter.sroa.0.0.ptr.i54.i8217.4 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 16
  %iter.sroa.0.0.ptr.i54.i8217.5 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 20
  %iter.sroa.0.0.ptr.i54.i8217.6 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 24
  %iter.sroa.0.0.ptr.i54.i8217.7 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 28
  %iter.sroa.0.0.ptr.i.i8229.1 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 4
  %iter.sroa.0.0.ptr.i.i8229.2 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 8
  %iter.sroa.0.0.ptr.i.i8229.3 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 12
  %iter.sroa.0.0.ptr.i.i8229.4 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 16
  %iter.sroa.0.0.ptr.i.i8229.5 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 20
  %iter.sroa.0.0.ptr.i.i8229.6 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 24
  %iter.sroa.0.0.ptr.i.i8229.7 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 28
  br label %bb37.i, !dbg !20490

bb16.i.bb13.i.loopexit_crit_edge:                 ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3625
  store float %_0.i3362, ptr %359, align 4, !dbg !20509
  store float %_0.i3736, ptr %361, align 4, !dbg !20525
  store float %_0.i3358, ptr %375, align 4, !dbg !20526
  store float %_0.i3732, ptr %377, align 4, !dbg !20528
  br label %bb13.i.loopexit, !dbg !20529

bb13.i.loopexit:                                  ; preds = %bb16.i.bb13.i.loopexit_crit_edge, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i947
  %ring_cursor.sroa.0.1.i949.lcssa = phi i32 [ %spec.store.select12.i, %bb16.i.bb13.i.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i7488515, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i947 ], !dbg !20535
  %main_cursor.sroa.0.1.i950.lcssa = phi i32 [ %spec.store.select11.i, %bb16.i.bb13.i.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i7498516, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i947 ], !dbg !20536
  %_115.not.i = icmp eq i32 %383, 0, !dbg !20490
  %indvars.iv.next12508 = add i32 %indvars.iv12507, -32, !dbg !20490
  br i1 %_115.not.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit, label %bb37.i, !dbg !20490

bb37.i:                                           ; preds = %bb37.i.lr.ph, %bb13.i.loopexit
  %indvars.iv12507 = phi i32 [ %frames, %bb37.i.lr.ph ], [ %indvars.iv.next12508, %bb13.i.loopexit ]
  %main_cursor.sroa.0.0.i7498516 = phi i32 [ %_35.i, %bb37.i.lr.ph ], [ %main_cursor.sroa.0.1.i950.lcssa, %bb13.i.loopexit ]
  %ring_cursor.sroa.0.0.i7488515 = phi i32 [ %_37.i742, %bb37.i.lr.ph ], [ %ring_cursor.sroa.0.1.i949.lcssa, %bb13.i.loopexit ]
  %iter2.sroa.0.0.i7478514 = phi i32 [ %yield_count.sroa.0.0.i4571, %bb37.i.lr.ph ], [ %383, %bb13.i.loopexit ]
  %iter.sroa.0.0.i8513 = phi i32 [ 0, %bb37.i.lr.ph ], [ %382, %bb13.i.loopexit ]
  %381 = call i32 @llvm.umax.i32(i32 %indvars.iv12507, i32 1), !dbg !20537
  %umax12526 = call i32 @llvm.umin.i32(i32 %381, i32 32), !dbg !20537
  %382 = add i32 %iter.sroa.0.0.i8513, 32, !dbg !20537
  %383 = add nsw i32 %iter2.sroa.0.0.i7478514, -1, !dbg !20541
  %384 = sub i32 %frames, %iter.sroa.0.0.i8513, !dbg !20542
  %spec.store.select.i750 = tail call i32 @llvm.umin.i32(i32 %384, i32 32), !dbg !20543
  %_51.i = add i32 %spec.store.select.i750, %iter.sroa.0.0.i8513, !dbg !20548
  %_125.i = icmp ult i32 %_51.i, %iter.sroa.0.0.i8513, !dbg !20549
  %_119.not.i = icmp ugt i32 %_51.i, %left_io.1
  %or.cond.i751 = or i1 %_125.i, %_119.not.i, !dbg !20549
  br i1 %or.cond.i751, label %bb43.i, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4586, !dbg !20549, !prof !4596

bb43.i:                                           ; preds = %bb37.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %iter.sroa.0.0.i8513, i32 noundef %_51.i, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_56e6f2edba81ce75fd22cd243776a278) #32, !dbg !20556, !noalias !20557
  unreachable, !dbg !20556

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4586: ; preds = %bb37.i
  %_128.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %iter.sroa.0.0.i8513, !dbg !20558
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20562), !dbg !20565
  %history.i138.i.sroa.0.0.copyload = load float, ptr %hot_left.i734, align 4, !dbg !20566, !noalias !20568
  %history.i138.i.sroa.7.0.copyload = load float, ptr %history.i138.i.sroa.7.0.hot_left.i734.sroa_idx, align 4, !dbg !20566, !noalias !20568
  %history.i138.i.sroa.10.0.copyload = load float, ptr %history.i138.i.sroa.10.0.hot_left.i734.sroa_idx, align 4, !dbg !20566, !noalias !20568
  %history.i138.i.sroa.13.0.copyload = load float, ptr %history.i138.i.sroa.13.0.hot_left.i734.sroa_idx, align 4, !dbg !20566, !noalias !20568
  %history.i138.i.sroa.16.0.copyload = load float, ptr %history.i138.i.sroa.16.0.hot_left.i734.sroa_idx, align 4, !dbg !20566, !noalias !20568
  %history.i138.i.sroa.19.0.copyload = load float, ptr %history.i138.i.sroa.19.0.hot_left.i734.sroa_idx, align 4, !dbg !20566, !noalias !20568
  %history.i138.i.sroa.22.0.copyload = load float, ptr %history.i138.i.sroa.22.0.hot_left.i734.sroa_idx, align 4, !dbg !20566, !noalias !20568
  %history.i138.i.sroa.26.0.copyload = load float, ptr %history.i138.i.sroa.26.0.hot_left.i734.sroa_idx, align 4, !dbg !20566, !noalias !20568
  %history.i138.i.sroa.29.0.copyload = load float, ptr %history.i138.i.sroa.29.0.hot_left.i734.sroa_idx, align 4, !dbg !20566, !noalias !20568
  %history.i138.i.sroa.32.0.copyload = load float, ptr %history.i138.i.sroa.32.0.hot_left.i734.sroa_idx, align 4, !dbg !20566, !noalias !20568
  %history.i138.i.sroa.35.0.copyload = load float, ptr %history.i138.i.sroa.35.0.hot_left.i734.sroa_idx, align 4, !dbg !20566, !noalias !20568
  %history.i138.i.sroa.38.0.copyload = load float, ptr %history.i138.i.sroa.38.0.hot_left.i734.sroa_idx, align 4, !dbg !20566, !noalias !20568
  %_2.i45898154.not = icmp eq i32 %frames, %iter.sroa.0.0.i8513, !dbg !20571
  br i1 %_2.i45898154.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit333.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519.lr.ph, !dbg !20571

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519.lr.ph: ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4586
  %_11.i.i.i159.i = load float, ptr %_31, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_14.i.i.i162.i = load float, ptr %313, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_17.i.i.i165.i = load float, ptr %314, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_20.i.i.i168.i = load float, ptr %315, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_25.i.i.i173.i = load float, ptr %row1.i.i.i171.i, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_28.i.i.i176.i = load float, ptr %316, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_31.i.i.i179.i = load float, ptr %317, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_34.i.i.i182.i = load float, ptr %318, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_39.i.i.i187.i = load float, ptr %row3.i.i.i185.i, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_42.i.i.i190.i = load float, ptr %319, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_45.i.i.i193.i = load float, ptr %320, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_48.i.i.i196.i = load float, ptr %321, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_53.i.i.i201.i = load float, ptr %row5.i.i.i199.i, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_56.i.i.i204.i = load float, ptr %322, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_59.i.i.i207.i = load float, ptr %323, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_62.i.i.i210.i = load float, ptr %324, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_67.i.i.i215.i = load float, ptr %row7.i.i.i213.i, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_70.i.i.i218.i = load float, ptr %325, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_73.i.i.i221.i = load float, ptr %326, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_76.i.i.i224.i = load float, ptr %327, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_81.i.i.i229.i = load float, ptr %row9.i.i.i227.i, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_84.i.i.i232.i = load float, ptr %328, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_87.i.i.i235.i = load float, ptr %329, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_90.i.i.i238.i = load float, ptr %330, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_95.i.i.i243.i = load float, ptr %row11.i.i.i241.i, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_98.i.i.i246.i = load float, ptr %331, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_101.i.i.i249.i = load float, ptr %332, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_104.i.i.i252.i = load float, ptr %333, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_109.i.i.i257.i = load float, ptr %row13.i.i.i255.i, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_112.i.i.i260.i = load float, ptr %334, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_115.i.i.i263.i = load float, ptr %335, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_118.i.i.i266.i = load float, ptr %336, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_123.i.i.i271.i = load float, ptr %row15.i.i.i269.i, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_126.i.i.i274.i = load float, ptr %337, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_129.i.i.i277.i = load float, ptr %338, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_132.i.i.i280.i = load float, ptr %339, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_137.i.i.i285.i = load float, ptr %row17.i.i.i283.i, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_140.i.i.i288.i = load float, ptr %340, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_143.i.i.i291.i = load float, ptr %341, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_146.i.i.i294.i = load float, ptr %342, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_151.i.i.i299.i = load float, ptr %row19.i.i.i297.i, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_154.i.i.i302.i = load float, ptr %343, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_157.i.i.i305.i = load float, ptr %344, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_160.i.i.i308.i = load float, ptr %345, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_165.i.i.i313.i = load float, ptr %row21.i.i.i311.i, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_168.i.i.i316.i = load float, ptr %346, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_171.i.i.i319.i = load float, ptr %347, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  %_174.i.i.i322.i = load float, ptr %348, align 4, !alias.scope !20574, !noalias !20579, !noundef !10
  br label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519, !dbg !20571

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519
  %iter.i134.i.sroa.16.08166 = phi i32 [ 0, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519.lr.ph ], [ %390, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519 ]
  %history.i138.i.sroa.35.08165 = phi float [ %history.i138.i.sroa.35.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519.lr.ph ], [ %history.i138.i.sroa.32.08164, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519 ]
  %history.i138.i.sroa.32.08164 = phi float [ %history.i138.i.sroa.32.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519.lr.ph ], [ %history.i138.i.sroa.29.08163, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519 ]
  %history.i138.i.sroa.29.08163 = phi float [ %history.i138.i.sroa.29.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519.lr.ph ], [ %history.i138.i.sroa.26.08162, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519 ]
  %history.i138.i.sroa.26.08162 = phi float [ %history.i138.i.sroa.26.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519.lr.ph ], [ %history.i138.i.sroa.22.08161, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519 ]
  %history.i138.i.sroa.22.08161 = phi float [ %history.i138.i.sroa.22.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519.lr.ph ], [ %history.i138.i.sroa.19.08160, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519 ]
  %history.i138.i.sroa.19.08160 = phi float [ %history.i138.i.sroa.19.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519.lr.ph ], [ %history.i138.i.sroa.16.08159, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519 ]
  %history.i138.i.sroa.16.08159 = phi float [ %history.i138.i.sroa.16.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519.lr.ph ], [ %history.i138.i.sroa.13.08158, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519 ]
  %history.i138.i.sroa.13.08158 = phi float [ %history.i138.i.sroa.13.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519.lr.ph ], [ %history.i138.i.sroa.10.08157, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519 ]
  %history.i138.i.sroa.10.08157 = phi float [ %history.i138.i.sroa.10.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519.lr.ph ], [ %history.i138.i.sroa.7.08156, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519 ]
  %history.i138.i.sroa.7.08156 = phi float [ %history.i138.i.sroa.7.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519.lr.ph ], [ %history.i138.i.sroa.0.08155, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519 ]
  %history.i138.i.sroa.0.08155 = phi float [ %history.i138.i.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519.lr.ph ], [ %_0.i3517, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519 ]
  %data.i.i4596 = getelementptr inbounds nuw float, ptr %_128.i, i32 %iter.i134.i.sroa.16.08166, !dbg !20584
  %_0.i3517 = load float, ptr %data.i.i4596, align 4, !dbg !20587, !alias.scope !20589, !noalias !20592, !noundef !10
  %385 = tail call noundef float @llvm.fabs.f32(float %history.i138.i.sroa.19.08160), !dbg !20593
  %_0.i3134 = fmul float %_0.i3517, %_11.i.i.i159.i, !dbg !20596
  %_0.i2702 = fadd float %_0.i3134, 0.000000e+00, !dbg !20599
  %_0.i3133 = fmul float %_0.i3517, %_14.i.i.i162.i, !dbg !20601
  %_0.i2701 = fadd float %_0.i3133, 0.000000e+00, !dbg !20603
  %_0.i3132 = fmul float %_0.i3517, %_17.i.i.i165.i, !dbg !20605
  %_0.i2700 = fadd float %_0.i3132, 0.000000e+00, !dbg !20607
  %_0.i3131 = fmul float %_0.i3517, %_20.i.i.i168.i, !dbg !20609
  %_0.i2699 = fadd float %_0.i3131, 0.000000e+00, !dbg !20611
  %_0.i3130 = fmul float %history.i138.i.sroa.0.08155, %_25.i.i.i173.i, !dbg !20613
  %_0.i2698 = fadd float %_0.i2702, %_0.i3130, !dbg !20615
  %_0.i3129 = fmul float %history.i138.i.sroa.0.08155, %_28.i.i.i176.i, !dbg !20617
  %_0.i2697 = fadd float %_0.i2701, %_0.i3129, !dbg !20619
  %_0.i3128 = fmul float %history.i138.i.sroa.0.08155, %_31.i.i.i179.i, !dbg !20621
  %_0.i2696 = fadd float %_0.i2700, %_0.i3128, !dbg !20623
  %_0.i3127 = fmul float %history.i138.i.sroa.0.08155, %_34.i.i.i182.i, !dbg !20625
  %_0.i2695 = fadd float %_0.i2699, %_0.i3127, !dbg !20627
  %_0.i3126 = fmul float %history.i138.i.sroa.7.08156, %_39.i.i.i187.i, !dbg !20629
  %_0.i2694 = fadd float %_0.i2698, %_0.i3126, !dbg !20631
  %_0.i3125 = fmul float %history.i138.i.sroa.7.08156, %_42.i.i.i190.i, !dbg !20633
  %_0.i2693 = fadd float %_0.i2697, %_0.i3125, !dbg !20635
  %_0.i3124 = fmul float %history.i138.i.sroa.7.08156, %_45.i.i.i193.i, !dbg !20637
  %_0.i2692 = fadd float %_0.i2696, %_0.i3124, !dbg !20639
  %_0.i3123 = fmul float %history.i138.i.sroa.7.08156, %_48.i.i.i196.i, !dbg !20641
  %_0.i2691 = fadd float %_0.i2695, %_0.i3123, !dbg !20643
  %_0.i3122 = fmul float %history.i138.i.sroa.10.08157, %_53.i.i.i201.i, !dbg !20645
  %_0.i2690 = fadd float %_0.i2694, %_0.i3122, !dbg !20647
  %_0.i3121 = fmul float %history.i138.i.sroa.10.08157, %_56.i.i.i204.i, !dbg !20649
  %_0.i2689 = fadd float %_0.i2693, %_0.i3121, !dbg !20651
  %_0.i3120 = fmul float %history.i138.i.sroa.10.08157, %_59.i.i.i207.i, !dbg !20653
  %_0.i2688 = fadd float %_0.i2692, %_0.i3120, !dbg !20655
  %_0.i3119 = fmul float %history.i138.i.sroa.10.08157, %_62.i.i.i210.i, !dbg !20657
  %_0.i2687 = fadd float %_0.i2691, %_0.i3119, !dbg !20659
  %_0.i3118 = fmul float %history.i138.i.sroa.13.08158, %_67.i.i.i215.i, !dbg !20661
  %_0.i2686 = fadd float %_0.i2690, %_0.i3118, !dbg !20663
  %_0.i3117 = fmul float %history.i138.i.sroa.13.08158, %_70.i.i.i218.i, !dbg !20665
  %_0.i2685 = fadd float %_0.i2689, %_0.i3117, !dbg !20667
  %_0.i3116 = fmul float %history.i138.i.sroa.13.08158, %_73.i.i.i221.i, !dbg !20669
  %_0.i2684 = fadd float %_0.i2688, %_0.i3116, !dbg !20671
  %_0.i3115 = fmul float %history.i138.i.sroa.13.08158, %_76.i.i.i224.i, !dbg !20673
  %_0.i2683 = fadd float %_0.i2687, %_0.i3115, !dbg !20675
  %_0.i3114 = fmul float %history.i138.i.sroa.16.08159, %_81.i.i.i229.i, !dbg !20677
  %_0.i2682 = fadd float %_0.i2686, %_0.i3114, !dbg !20679
  %_0.i3113 = fmul float %history.i138.i.sroa.16.08159, %_84.i.i.i232.i, !dbg !20681
  %_0.i2681 = fadd float %_0.i2685, %_0.i3113, !dbg !20683
  %_0.i3112 = fmul float %history.i138.i.sroa.16.08159, %_87.i.i.i235.i, !dbg !20685
  %_0.i2680 = fadd float %_0.i2684, %_0.i3112, !dbg !20687
  %_0.i3111 = fmul float %history.i138.i.sroa.16.08159, %_90.i.i.i238.i, !dbg !20689
  %_0.i2679 = fadd float %_0.i2683, %_0.i3111, !dbg !20691
  %_0.i3110 = fmul float %history.i138.i.sroa.19.08160, %_95.i.i.i243.i, !dbg !20693
  %_0.i2678 = fadd float %_0.i2682, %_0.i3110, !dbg !20695
  %_0.i3109 = fmul float %history.i138.i.sroa.19.08160, %_98.i.i.i246.i, !dbg !20697
  %_0.i2677 = fadd float %_0.i2681, %_0.i3109, !dbg !20699
  %_0.i3108 = fmul float %history.i138.i.sroa.19.08160, %_101.i.i.i249.i, !dbg !20701
  %_0.i2676 = fadd float %_0.i2680, %_0.i3108, !dbg !20703
  %_0.i3107 = fmul float %history.i138.i.sroa.19.08160, %_104.i.i.i252.i, !dbg !20705
  %_0.i2675 = fadd float %_0.i2679, %_0.i3107, !dbg !20707
  %_0.i3106 = fmul float %history.i138.i.sroa.22.08161, %_109.i.i.i257.i, !dbg !20709
  %_0.i2674 = fadd float %_0.i2678, %_0.i3106, !dbg !20711
  %_0.i3105 = fmul float %history.i138.i.sroa.22.08161, %_112.i.i.i260.i, !dbg !20713
  %_0.i2673 = fadd float %_0.i2677, %_0.i3105, !dbg !20715
  %_0.i3104 = fmul float %history.i138.i.sroa.22.08161, %_115.i.i.i263.i, !dbg !20717
  %_0.i2672 = fadd float %_0.i2676, %_0.i3104, !dbg !20719
  %_0.i3103 = fmul float %history.i138.i.sroa.22.08161, %_118.i.i.i266.i, !dbg !20721
  %_0.i2671 = fadd float %_0.i2675, %_0.i3103, !dbg !20723
  %_0.i3102 = fmul float %history.i138.i.sroa.26.08162, %_123.i.i.i271.i, !dbg !20725
  %_0.i2670 = fadd float %_0.i2674, %_0.i3102, !dbg !20727
  %_0.i3101 = fmul float %history.i138.i.sroa.26.08162, %_126.i.i.i274.i, !dbg !20729
  %_0.i2669 = fadd float %_0.i2673, %_0.i3101, !dbg !20731
  %_0.i3100 = fmul float %history.i138.i.sroa.26.08162, %_129.i.i.i277.i, !dbg !20733
  %_0.i2668 = fadd float %_0.i2672, %_0.i3100, !dbg !20735
  %_0.i3099 = fmul float %history.i138.i.sroa.26.08162, %_132.i.i.i280.i, !dbg !20737
  %_0.i2667 = fadd float %_0.i2671, %_0.i3099, !dbg !20739
  %_0.i3098 = fmul float %history.i138.i.sroa.29.08163, %_137.i.i.i285.i, !dbg !20741
  %_0.i2666 = fadd float %_0.i2670, %_0.i3098, !dbg !20743
  %_0.i3097 = fmul float %history.i138.i.sroa.29.08163, %_140.i.i.i288.i, !dbg !20745
  %_0.i2665 = fadd float %_0.i2669, %_0.i3097, !dbg !20747
  %_0.i3096 = fmul float %history.i138.i.sroa.29.08163, %_143.i.i.i291.i, !dbg !20749
  %_0.i2664 = fadd float %_0.i2668, %_0.i3096, !dbg !20751
  %_0.i3095 = fmul float %history.i138.i.sroa.29.08163, %_146.i.i.i294.i, !dbg !20753
  %_0.i2663 = fadd float %_0.i2667, %_0.i3095, !dbg !20755
  %_0.i3094 = fmul float %history.i138.i.sroa.32.08164, %_151.i.i.i299.i, !dbg !20757
  %_0.i2662 = fadd float %_0.i2666, %_0.i3094, !dbg !20759
  %_0.i3093 = fmul float %history.i138.i.sroa.32.08164, %_154.i.i.i302.i, !dbg !20761
  %_0.i2661 = fadd float %_0.i2665, %_0.i3093, !dbg !20763
  %_0.i3092 = fmul float %history.i138.i.sroa.32.08164, %_157.i.i.i305.i, !dbg !20765
  %_0.i2660 = fadd float %_0.i2664, %_0.i3092, !dbg !20767
  %_0.i3091 = fmul float %history.i138.i.sroa.32.08164, %_160.i.i.i308.i, !dbg !20769
  %_0.i2659 = fadd float %_0.i2663, %_0.i3091, !dbg !20771
  %_0.i3090 = fmul float %history.i138.i.sroa.35.08165, %_165.i.i.i313.i, !dbg !20773
  %_0.i2658 = fadd float %_0.i2662, %_0.i3090, !dbg !20775
  %_0.i3089 = fmul float %history.i138.i.sroa.35.08165, %_168.i.i.i316.i, !dbg !20777
  %_0.i2657 = fadd float %_0.i2661, %_0.i3089, !dbg !20779
  %_0.i3088 = fmul float %history.i138.i.sroa.35.08165, %_171.i.i.i319.i, !dbg !20781
  %_0.i2656 = fadd float %_0.i2660, %_0.i3088, !dbg !20783
  %_0.i3087 = fmul float %history.i138.i.sroa.35.08165, %_174.i.i.i322.i, !dbg !20785
  %_0.i2655 = fadd float %_0.i2659, %_0.i3087, !dbg !20787
  %386 = tail call noundef float @llvm.fabs.f32(float %_0.i2658), !dbg !20789
  %_3.i.i4145.inv = fcmp ogt float %385, %386, !dbg !20791
  %_4.i.i4152.v = select i1 %_3.i.i4145.inv, float %385, float %386, !dbg !20791
  %387 = tail call noundef float @llvm.fabs.f32(float %_0.i2657), !dbg !20789
  %_3.i.i4145.inv.1 = fcmp ogt float %_4.i.i4152.v, %387, !dbg !20791
  %_4.i.i4152.v.1 = select i1 %_3.i.i4145.inv.1, float %_4.i.i4152.v, float %387, !dbg !20791
  %388 = tail call noundef float @llvm.fabs.f32(float %_0.i2656), !dbg !20789
  %_3.i.i4145.inv.2 = fcmp ogt float %_4.i.i4152.v.1, %388, !dbg !20791
  %_4.i.i4152.v.2 = select i1 %_3.i.i4145.inv.2, float %_4.i.i4152.v.1, float %388, !dbg !20791
  %389 = tail call noundef float @llvm.fabs.f32(float %_0.i2655), !dbg !20789
  %_3.i.i4145.inv.3 = fcmp ogt float %_4.i.i4152.v.2, %389, !dbg !20791
  %_4.i.i4152.v.3 = select i1 %_3.i.i4145.inv.3, float %_4.i.i4152.v.2, float %389, !dbg !20791
  %390 = add nuw nsw i32 %iter.i134.i.sroa.16.08166, 1, !dbg !20794
  %data.i4.i4600 = getelementptr inbounds nuw float, ptr %peaks_left.i732, i32 %iter.i134.i.sroa.16.08166, !dbg !20795
  store float %_4.i.i4152.v.3, ptr %data.i4.i4600, align 4, !dbg !20798, !alias.scope !20800, !noalias !20592
  %exitcond12511.not = icmp eq i32 %390, %umax12526, !dbg !20571
  br i1 %exitcond12511.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit333.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519, !dbg !20571

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit333.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4586
  %history.i138.i.sroa.0.0.lcssa = phi float [ %history.i138.i.sroa.0.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4586 ], [ %_0.i3517, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519 ], !dbg !20803
  %history.i138.i.sroa.7.0.lcssa = phi float [ %history.i138.i.sroa.7.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4586 ], [ %history.i138.i.sroa.0.08155, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519 ], !dbg !20803
  %history.i138.i.sroa.10.0.lcssa = phi float [ %history.i138.i.sroa.10.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4586 ], [ %history.i138.i.sroa.7.08156, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519 ], !dbg !20803
  %history.i138.i.sroa.13.0.lcssa = phi float [ %history.i138.i.sroa.13.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4586 ], [ %history.i138.i.sroa.10.08157, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519 ], !dbg !20803
  %history.i138.i.sroa.16.0.lcssa = phi float [ %history.i138.i.sroa.16.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4586 ], [ %history.i138.i.sroa.13.08158, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519 ], !dbg !20803
  %history.i138.i.sroa.19.0.lcssa = phi float [ %history.i138.i.sroa.19.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4586 ], [ %history.i138.i.sroa.16.08159, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519 ], !dbg !20803
  %history.i138.i.sroa.22.0.lcssa = phi float [ %history.i138.i.sroa.22.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4586 ], [ %history.i138.i.sroa.19.08160, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519 ], !dbg !20803
  %history.i138.i.sroa.26.0.lcssa = phi float [ %history.i138.i.sroa.26.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4586 ], [ %history.i138.i.sroa.22.08161, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519 ], !dbg !20803
  %history.i138.i.sroa.29.0.lcssa = phi float [ %history.i138.i.sroa.29.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4586 ], [ %history.i138.i.sroa.26.08162, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519 ], !dbg !20803
  %history.i138.i.sroa.32.0.lcssa = phi float [ %history.i138.i.sroa.32.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4586 ], [ %history.i138.i.sroa.29.08163, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519 ], !dbg !20803
  %history.i138.i.sroa.35.0.lcssa = phi float [ %history.i138.i.sroa.35.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4586 ], [ %history.i138.i.sroa.32.08164, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519 ], !dbg !20803
  %history.i138.i.sroa.38.0.lcssa = phi float [ %history.i138.i.sroa.38.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4586 ], [ %history.i138.i.sroa.35.08165, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3519 ], !dbg !20803
  store float %history.i138.i.sroa.0.0.lcssa, ptr %hot_left.i734, align 4, !dbg !20804, !noalias !20568
  store float %history.i138.i.sroa.7.0.lcssa, ptr %history.i138.i.sroa.7.0.hot_left.i734.sroa_idx, align 4, !dbg !20804, !noalias !20568
  store float %history.i138.i.sroa.10.0.lcssa, ptr %history.i138.i.sroa.10.0.hot_left.i734.sroa_idx, align 4, !dbg !20804, !noalias !20568
  store float %history.i138.i.sroa.13.0.lcssa, ptr %history.i138.i.sroa.13.0.hot_left.i734.sroa_idx, align 4, !dbg !20804, !noalias !20568
  store float %history.i138.i.sroa.16.0.lcssa, ptr %history.i138.i.sroa.16.0.hot_left.i734.sroa_idx, align 4, !dbg !20804, !noalias !20568
  store float %history.i138.i.sroa.19.0.lcssa, ptr %history.i138.i.sroa.19.0.hot_left.i734.sroa_idx, align 4, !dbg !20804, !noalias !20568
  store float %history.i138.i.sroa.22.0.lcssa, ptr %history.i138.i.sroa.22.0.hot_left.i734.sroa_idx, align 4, !dbg !20804, !noalias !20568
  store float %history.i138.i.sroa.26.0.lcssa, ptr %history.i138.i.sroa.26.0.hot_left.i734.sroa_idx, align 4, !dbg !20804, !noalias !20568
  store float %history.i138.i.sroa.29.0.lcssa, ptr %history.i138.i.sroa.29.0.hot_left.i734.sroa_idx, align 4, !dbg !20804, !noalias !20568
  store float %history.i138.i.sroa.32.0.lcssa, ptr %history.i138.i.sroa.32.0.hot_left.i734.sroa_idx, align 4, !dbg !20804, !noalias !20568
  store float %history.i138.i.sroa.35.0.lcssa, ptr %history.i138.i.sroa.35.0.hot_left.i734.sroa_idx, align 4, !dbg !20804, !noalias !20568
  store float %history.i138.i.sroa.38.0.lcssa, ptr %history.i138.i.sroa.38.0.hot_left.i734.sroa_idx, align 4, !dbg !20804, !noalias !20568
  %_136.not.i = icmp ugt i32 %_51.i, %right_io.1, !dbg !20805
  br i1 %_136.not.i, label %bb49.i, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4631, !dbg !20805, !prof !787

bb49.i:                                           ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit333.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %iter.sroa.0.0.i8513, i32 noundef %_51.i, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_685aef6cd6b0f866813eafbee04e700c) #32, !dbg !20809, !noalias !20557
  unreachable, !dbg !20809

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4631: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit333.i
  %_143.i752 = getelementptr inbounds nuw float, ptr %right_io.0, i32 %iter.sroa.0.0.i8513, !dbg !20810
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20814), !dbg !20817
  %history.i.i730.sroa.0.0.copyload = load float, ptr %hot_right.i733, align 4, !dbg !20818, !noalias !20820
  %history.i.i730.sroa.7.0.copyload = load float, ptr %history.i.i730.sroa.7.0.hot_right.i733.sroa_idx, align 4, !dbg !20818, !noalias !20820
  %history.i.i730.sroa.10.0.copyload = load float, ptr %history.i.i730.sroa.10.0.hot_right.i733.sroa_idx, align 4, !dbg !20818, !noalias !20820
  %history.i.i730.sroa.13.0.copyload = load float, ptr %history.i.i730.sroa.13.0.hot_right.i733.sroa_idx, align 4, !dbg !20818, !noalias !20820
  %history.i.i730.sroa.16.0.copyload = load float, ptr %history.i.i730.sroa.16.0.hot_right.i733.sroa_idx, align 4, !dbg !20818, !noalias !20820
  %history.i.i730.sroa.19.0.copyload = load float, ptr %history.i.i730.sroa.19.0.hot_right.i733.sroa_idx, align 4, !dbg !20818, !noalias !20820
  %history.i.i730.sroa.22.0.copyload = load float, ptr %history.i.i730.sroa.22.0.hot_right.i733.sroa_idx, align 4, !dbg !20818, !noalias !20820
  %history.i.i730.sroa.26.0.copyload = load float, ptr %history.i.i730.sroa.26.0.hot_right.i733.sroa_idx, align 4, !dbg !20818, !noalias !20820
  %history.i.i730.sroa.29.0.copyload = load float, ptr %history.i.i730.sroa.29.0.hot_right.i733.sroa_idx, align 4, !dbg !20818, !noalias !20820
  %history.i.i730.sroa.32.0.copyload = load float, ptr %history.i.i730.sroa.32.0.hot_right.i733.sroa_idx, align 4, !dbg !20818, !noalias !20820
  %history.i.i730.sroa.35.0.copyload = load float, ptr %history.i.i730.sroa.35.0.hot_right.i733.sroa_idx, align 4, !dbg !20818, !noalias !20820
  %history.i.i730.sroa.38.0.copyload = load float, ptr %history.i.i730.sroa.38.0.hot_right.i733.sroa_idx, align 4, !dbg !20818, !noalias !20820
  br i1 %_2.i45898154.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i947, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514.lr.ph, !dbg !20823

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514.lr.ph: ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4631
  %_11.i.i.i.i773 = load float, ptr %_31, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_14.i.i.i.i776 = load float, ptr %313, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_17.i.i.i.i779 = load float, ptr %314, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_20.i.i.i.i782 = load float, ptr %315, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_25.i.i.i.i787 = load float, ptr %row1.i.i.i171.i, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_28.i.i.i.i790 = load float, ptr %316, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_31.i.i.i.i793 = load float, ptr %317, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_34.i.i.i.i796 = load float, ptr %318, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_39.i.i.i.i801 = load float, ptr %row3.i.i.i185.i, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_42.i.i.i.i804 = load float, ptr %319, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_45.i.i.i.i807 = load float, ptr %320, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_48.i.i.i.i810 = load float, ptr %321, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_53.i.i.i.i815 = load float, ptr %row5.i.i.i199.i, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_56.i.i.i.i818 = load float, ptr %322, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_59.i.i.i.i821 = load float, ptr %323, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_62.i.i.i.i824 = load float, ptr %324, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_67.i.i.i.i829 = load float, ptr %row7.i.i.i213.i, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_70.i.i.i.i832 = load float, ptr %325, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_73.i.i.i.i835 = load float, ptr %326, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_76.i.i.i.i838 = load float, ptr %327, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_81.i.i.i.i843 = load float, ptr %row9.i.i.i227.i, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_84.i.i.i.i846 = load float, ptr %328, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_87.i.i.i.i849 = load float, ptr %329, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_90.i.i.i.i852 = load float, ptr %330, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_95.i.i.i.i857 = load float, ptr %row11.i.i.i241.i, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_98.i.i.i.i860 = load float, ptr %331, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_101.i.i.i.i863 = load float, ptr %332, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_104.i.i.i.i866 = load float, ptr %333, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_109.i.i.i.i871 = load float, ptr %row13.i.i.i255.i, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_112.i.i.i.i874 = load float, ptr %334, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_115.i.i.i.i877 = load float, ptr %335, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_118.i.i.i.i880 = load float, ptr %336, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_123.i.i.i.i885 = load float, ptr %row15.i.i.i269.i, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_126.i.i.i.i888 = load float, ptr %337, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_129.i.i.i.i891 = load float, ptr %338, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_132.i.i.i.i894 = load float, ptr %339, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_137.i.i.i.i899 = load float, ptr %row17.i.i.i283.i, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_140.i.i.i.i902 = load float, ptr %340, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_143.i.i.i.i905 = load float, ptr %341, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_146.i.i.i.i908 = load float, ptr %342, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_151.i.i.i.i913 = load float, ptr %row19.i.i.i297.i, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_154.i.i.i.i916 = load float, ptr %343, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_157.i.i.i.i919 = load float, ptr %344, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_160.i.i.i.i922 = load float, ptr %345, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_165.i.i.i.i927 = load float, ptr %row21.i.i.i311.i, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_168.i.i.i.i930 = load float, ptr %346, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_171.i.i.i.i933 = load float, ptr %347, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  %_174.i.i.i.i936 = load float, ptr %348, align 4, !alias.scope !20826, !noalias !20831, !noundef !10
  br label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514, !dbg !20823

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514
  %iter.i.i726.sroa.16.08193 = phi i32 [ 0, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514.lr.ph ], [ %396, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514 ]
  %history.i.i730.sroa.35.08192 = phi float [ %history.i.i730.sroa.35.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514.lr.ph ], [ %history.i.i730.sroa.32.08191, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514 ]
  %history.i.i730.sroa.32.08191 = phi float [ %history.i.i730.sroa.32.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514.lr.ph ], [ %history.i.i730.sroa.29.08190, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514 ]
  %history.i.i730.sroa.29.08190 = phi float [ %history.i.i730.sroa.29.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514.lr.ph ], [ %history.i.i730.sroa.26.08189, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514 ]
  %history.i.i730.sroa.26.08189 = phi float [ %history.i.i730.sroa.26.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514.lr.ph ], [ %history.i.i730.sroa.22.08188, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514 ]
  %history.i.i730.sroa.22.08188 = phi float [ %history.i.i730.sroa.22.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514.lr.ph ], [ %history.i.i730.sroa.19.08187, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514 ]
  %history.i.i730.sroa.19.08187 = phi float [ %history.i.i730.sroa.19.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514.lr.ph ], [ %history.i.i730.sroa.16.08186, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514 ]
  %history.i.i730.sroa.16.08186 = phi float [ %history.i.i730.sroa.16.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514.lr.ph ], [ %history.i.i730.sroa.13.08185, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514 ]
  %history.i.i730.sroa.13.08185 = phi float [ %history.i.i730.sroa.13.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514.lr.ph ], [ %history.i.i730.sroa.10.08184, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514 ]
  %history.i.i730.sroa.10.08184 = phi float [ %history.i.i730.sroa.10.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514.lr.ph ], [ %history.i.i730.sroa.7.08183, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514 ]
  %history.i.i730.sroa.7.08183 = phi float [ %history.i.i730.sroa.7.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514.lr.ph ], [ %history.i.i730.sroa.0.08182, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514 ]
  %history.i.i730.sroa.0.08182 = phi float [ %history.i.i730.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514.lr.ph ], [ %_0.i3512, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514 ]
  %data.i.i4641 = getelementptr inbounds nuw float, ptr %_143.i752, i32 %iter.i.i726.sroa.16.08193, !dbg !20836
  %_0.i3512 = load float, ptr %data.i.i4641, align 4, !dbg !20839, !alias.scope !20841, !noalias !20844, !noundef !10
  %391 = tail call noundef float @llvm.fabs.f32(float %history.i.i730.sroa.19.08187), !dbg !20845
  %_0.i3086 = fmul float %_0.i3512, %_11.i.i.i.i773, !dbg !20848
  %_0.i2654 = fadd float %_0.i3086, 0.000000e+00, !dbg !20851
  %_0.i3085 = fmul float %_0.i3512, %_14.i.i.i.i776, !dbg !20853
  %_0.i2653 = fadd float %_0.i3085, 0.000000e+00, !dbg !20855
  %_0.i3084 = fmul float %_0.i3512, %_17.i.i.i.i779, !dbg !20857
  %_0.i2652 = fadd float %_0.i3084, 0.000000e+00, !dbg !20859
  %_0.i3083 = fmul float %_0.i3512, %_20.i.i.i.i782, !dbg !20861
  %_0.i2651 = fadd float %_0.i3083, 0.000000e+00, !dbg !20863
  %_0.i3082 = fmul float %history.i.i730.sroa.0.08182, %_25.i.i.i.i787, !dbg !20865
  %_0.i2650 = fadd float %_0.i2654, %_0.i3082, !dbg !20867
  %_0.i3081 = fmul float %history.i.i730.sroa.0.08182, %_28.i.i.i.i790, !dbg !20869
  %_0.i2649 = fadd float %_0.i2653, %_0.i3081, !dbg !20871
  %_0.i3080 = fmul float %history.i.i730.sroa.0.08182, %_31.i.i.i.i793, !dbg !20873
  %_0.i2648 = fadd float %_0.i2652, %_0.i3080, !dbg !20875
  %_0.i3079 = fmul float %history.i.i730.sroa.0.08182, %_34.i.i.i.i796, !dbg !20877
  %_0.i2647 = fadd float %_0.i2651, %_0.i3079, !dbg !20879
  %_0.i3078 = fmul float %history.i.i730.sroa.7.08183, %_39.i.i.i.i801, !dbg !20881
  %_0.i2646 = fadd float %_0.i2650, %_0.i3078, !dbg !20883
  %_0.i3077 = fmul float %history.i.i730.sroa.7.08183, %_42.i.i.i.i804, !dbg !20885
  %_0.i2645 = fadd float %_0.i2649, %_0.i3077, !dbg !20887
  %_0.i3076 = fmul float %history.i.i730.sroa.7.08183, %_45.i.i.i.i807, !dbg !20889
  %_0.i2644 = fadd float %_0.i2648, %_0.i3076, !dbg !20891
  %_0.i3075 = fmul float %history.i.i730.sroa.7.08183, %_48.i.i.i.i810, !dbg !20893
  %_0.i2643 = fadd float %_0.i2647, %_0.i3075, !dbg !20895
  %_0.i3074 = fmul float %history.i.i730.sroa.10.08184, %_53.i.i.i.i815, !dbg !20897
  %_0.i2642 = fadd float %_0.i2646, %_0.i3074, !dbg !20899
  %_0.i3073 = fmul float %history.i.i730.sroa.10.08184, %_56.i.i.i.i818, !dbg !20901
  %_0.i2641 = fadd float %_0.i2645, %_0.i3073, !dbg !20903
  %_0.i3072 = fmul float %history.i.i730.sroa.10.08184, %_59.i.i.i.i821, !dbg !20905
  %_0.i2640 = fadd float %_0.i2644, %_0.i3072, !dbg !20907
  %_0.i3071 = fmul float %history.i.i730.sroa.10.08184, %_62.i.i.i.i824, !dbg !20909
  %_0.i2639 = fadd float %_0.i2643, %_0.i3071, !dbg !20911
  %_0.i3070 = fmul float %history.i.i730.sroa.13.08185, %_67.i.i.i.i829, !dbg !20913
  %_0.i2638 = fadd float %_0.i2642, %_0.i3070, !dbg !20915
  %_0.i3069 = fmul float %history.i.i730.sroa.13.08185, %_70.i.i.i.i832, !dbg !20917
  %_0.i2637 = fadd float %_0.i2641, %_0.i3069, !dbg !20919
  %_0.i3068 = fmul float %history.i.i730.sroa.13.08185, %_73.i.i.i.i835, !dbg !20921
  %_0.i2636 = fadd float %_0.i2640, %_0.i3068, !dbg !20923
  %_0.i3067 = fmul float %history.i.i730.sroa.13.08185, %_76.i.i.i.i838, !dbg !20925
  %_0.i2635 = fadd float %_0.i2639, %_0.i3067, !dbg !20927
  %_0.i3066 = fmul float %history.i.i730.sroa.16.08186, %_81.i.i.i.i843, !dbg !20929
  %_0.i2634 = fadd float %_0.i2638, %_0.i3066, !dbg !20931
  %_0.i3065 = fmul float %history.i.i730.sroa.16.08186, %_84.i.i.i.i846, !dbg !20933
  %_0.i2633 = fadd float %_0.i2637, %_0.i3065, !dbg !20935
  %_0.i3064 = fmul float %history.i.i730.sroa.16.08186, %_87.i.i.i.i849, !dbg !20937
  %_0.i2632 = fadd float %_0.i2636, %_0.i3064, !dbg !20939
  %_0.i3063 = fmul float %history.i.i730.sroa.16.08186, %_90.i.i.i.i852, !dbg !20941
  %_0.i2631 = fadd float %_0.i2635, %_0.i3063, !dbg !20943
  %_0.i3062 = fmul float %history.i.i730.sroa.19.08187, %_95.i.i.i.i857, !dbg !20945
  %_0.i2630 = fadd float %_0.i2634, %_0.i3062, !dbg !20947
  %_0.i3061 = fmul float %history.i.i730.sroa.19.08187, %_98.i.i.i.i860, !dbg !20949
  %_0.i2629 = fadd float %_0.i2633, %_0.i3061, !dbg !20951
  %_0.i3060 = fmul float %history.i.i730.sroa.19.08187, %_101.i.i.i.i863, !dbg !20953
  %_0.i2628 = fadd float %_0.i2632, %_0.i3060, !dbg !20955
  %_0.i3059 = fmul float %history.i.i730.sroa.19.08187, %_104.i.i.i.i866, !dbg !20957
  %_0.i2627 = fadd float %_0.i2631, %_0.i3059, !dbg !20959
  %_0.i3058 = fmul float %history.i.i730.sroa.22.08188, %_109.i.i.i.i871, !dbg !20961
  %_0.i2626 = fadd float %_0.i2630, %_0.i3058, !dbg !20963
  %_0.i3057 = fmul float %history.i.i730.sroa.22.08188, %_112.i.i.i.i874, !dbg !20965
  %_0.i2625 = fadd float %_0.i2629, %_0.i3057, !dbg !20967
  %_0.i3056 = fmul float %history.i.i730.sroa.22.08188, %_115.i.i.i.i877, !dbg !20969
  %_0.i2624 = fadd float %_0.i2628, %_0.i3056, !dbg !20971
  %_0.i3055 = fmul float %history.i.i730.sroa.22.08188, %_118.i.i.i.i880, !dbg !20973
  %_0.i2623 = fadd float %_0.i2627, %_0.i3055, !dbg !20975
  %_0.i3054 = fmul float %history.i.i730.sroa.26.08189, %_123.i.i.i.i885, !dbg !20977
  %_0.i2622 = fadd float %_0.i2626, %_0.i3054, !dbg !20979
  %_0.i3053 = fmul float %history.i.i730.sroa.26.08189, %_126.i.i.i.i888, !dbg !20981
  %_0.i2621 = fadd float %_0.i2625, %_0.i3053, !dbg !20983
  %_0.i3052 = fmul float %history.i.i730.sroa.26.08189, %_129.i.i.i.i891, !dbg !20985
  %_0.i2620 = fadd float %_0.i2624, %_0.i3052, !dbg !20987
  %_0.i3051 = fmul float %history.i.i730.sroa.26.08189, %_132.i.i.i.i894, !dbg !20989
  %_0.i2619 = fadd float %_0.i2623, %_0.i3051, !dbg !20991
  %_0.i3050 = fmul float %history.i.i730.sroa.29.08190, %_137.i.i.i.i899, !dbg !20993
  %_0.i2618 = fadd float %_0.i2622, %_0.i3050, !dbg !20995
  %_0.i3049 = fmul float %history.i.i730.sroa.29.08190, %_140.i.i.i.i902, !dbg !20997
  %_0.i2617 = fadd float %_0.i2621, %_0.i3049, !dbg !20999
  %_0.i3048 = fmul float %history.i.i730.sroa.29.08190, %_143.i.i.i.i905, !dbg !21001
  %_0.i2616 = fadd float %_0.i2620, %_0.i3048, !dbg !21003
  %_0.i3047 = fmul float %history.i.i730.sroa.29.08190, %_146.i.i.i.i908, !dbg !21005
  %_0.i2615 = fadd float %_0.i2619, %_0.i3047, !dbg !21007
  %_0.i3046 = fmul float %history.i.i730.sroa.32.08191, %_151.i.i.i.i913, !dbg !21009
  %_0.i2614 = fadd float %_0.i2618, %_0.i3046, !dbg !21011
  %_0.i3045 = fmul float %history.i.i730.sroa.32.08191, %_154.i.i.i.i916, !dbg !21013
  %_0.i2613 = fadd float %_0.i2617, %_0.i3045, !dbg !21015
  %_0.i3044 = fmul float %history.i.i730.sroa.32.08191, %_157.i.i.i.i919, !dbg !21017
  %_0.i2612 = fadd float %_0.i2616, %_0.i3044, !dbg !21019
  %_0.i3043 = fmul float %history.i.i730.sroa.32.08191, %_160.i.i.i.i922, !dbg !21021
  %_0.i2611 = fadd float %_0.i2615, %_0.i3043, !dbg !21023
  %_0.i3042 = fmul float %history.i.i730.sroa.35.08192, %_165.i.i.i.i927, !dbg !21025
  %_0.i2610 = fadd float %_0.i2614, %_0.i3042, !dbg !21027
  %_0.i3041 = fmul float %history.i.i730.sroa.35.08192, %_168.i.i.i.i930, !dbg !21029
  %_0.i2609 = fadd float %_0.i2613, %_0.i3041, !dbg !21031
  %_0.i3040 = fmul float %history.i.i730.sroa.35.08192, %_171.i.i.i.i933, !dbg !21033
  %_0.i2608 = fadd float %_0.i2612, %_0.i3040, !dbg !21035
  %_0.i3039 = fmul float %history.i.i730.sroa.35.08192, %_174.i.i.i.i936, !dbg !21037
  %_0.i2607 = fadd float %_0.i2611, %_0.i3039, !dbg !21039
  %392 = tail call noundef float @llvm.fabs.f32(float %_0.i2610), !dbg !21041
  %_3.i.i4136.inv = fcmp ogt float %391, %392, !dbg !21043
  %_4.i.i4143.v = select i1 %_3.i.i4136.inv, float %391, float %392, !dbg !21043
  %393 = tail call noundef float @llvm.fabs.f32(float %_0.i2609), !dbg !21041
  %_3.i.i4136.inv.1 = fcmp ogt float %_4.i.i4143.v, %393, !dbg !21043
  %_4.i.i4143.v.1 = select i1 %_3.i.i4136.inv.1, float %_4.i.i4143.v, float %393, !dbg !21043
  %394 = tail call noundef float @llvm.fabs.f32(float %_0.i2608), !dbg !21041
  %_3.i.i4136.inv.2 = fcmp ogt float %_4.i.i4143.v.1, %394, !dbg !21043
  %_4.i.i4143.v.2 = select i1 %_3.i.i4136.inv.2, float %_4.i.i4143.v.1, float %394, !dbg !21043
  %395 = tail call noundef float @llvm.fabs.f32(float %_0.i2607), !dbg !21041
  %_3.i.i4136.inv.3 = fcmp ogt float %_4.i.i4143.v.2, %395, !dbg !21043
  %_4.i.i4143.v.3 = select i1 %_3.i.i4136.inv.3, float %_4.i.i4143.v.2, float %395, !dbg !21043
  %396 = add nuw nsw i32 %iter.i.i726.sroa.16.08193, 1, !dbg !21046
  %data.i4.i4645 = getelementptr inbounds nuw float, ptr %peaks_right.i731, i32 %iter.i.i726.sroa.16.08193, !dbg !21047
  store float %_4.i.i4143.v.3, ptr %data.i4.i4645, align 4, !dbg !21050, !alias.scope !21052, !noalias !20844
  %exitcond12514.not = icmp eq i32 %396, %umax12526, !dbg !20823
  br i1 %exitcond12514.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i947, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514, !dbg !20823

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i947: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4631
  %history.i.i730.sroa.0.0.lcssa = phi float [ %history.i.i730.sroa.0.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4631 ], [ %_0.i3512, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514 ], !dbg !21055
  %history.i.i730.sroa.7.0.lcssa = phi float [ %history.i.i730.sroa.7.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4631 ], [ %history.i.i730.sroa.0.08182, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514 ], !dbg !21055
  %history.i.i730.sroa.10.0.lcssa = phi float [ %history.i.i730.sroa.10.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4631 ], [ %history.i.i730.sroa.7.08183, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514 ], !dbg !21055
  %history.i.i730.sroa.13.0.lcssa = phi float [ %history.i.i730.sroa.13.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4631 ], [ %history.i.i730.sroa.10.08184, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514 ], !dbg !21055
  %history.i.i730.sroa.16.0.lcssa = phi float [ %history.i.i730.sroa.16.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4631 ], [ %history.i.i730.sroa.13.08185, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514 ], !dbg !21055
  %history.i.i730.sroa.19.0.lcssa = phi float [ %history.i.i730.sroa.19.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4631 ], [ %history.i.i730.sroa.16.08186, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514 ], !dbg !21055
  %history.i.i730.sroa.22.0.lcssa = phi float [ %history.i.i730.sroa.22.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4631 ], [ %history.i.i730.sroa.19.08187, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514 ], !dbg !21055
  %history.i.i730.sroa.26.0.lcssa = phi float [ %history.i.i730.sroa.26.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4631 ], [ %history.i.i730.sroa.22.08188, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514 ], !dbg !21055
  %history.i.i730.sroa.29.0.lcssa = phi float [ %history.i.i730.sroa.29.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4631 ], [ %history.i.i730.sroa.26.08189, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514 ], !dbg !21055
  %history.i.i730.sroa.32.0.lcssa = phi float [ %history.i.i730.sroa.32.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4631 ], [ %history.i.i730.sroa.29.08190, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514 ], !dbg !21055
  %history.i.i730.sroa.35.0.lcssa = phi float [ %history.i.i730.sroa.35.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4631 ], [ %history.i.i730.sroa.32.08191, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514 ], !dbg !21055
  %history.i.i730.sroa.38.0.lcssa = phi float [ %history.i.i730.sroa.38.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4631 ], [ %history.i.i730.sroa.35.08192, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3514 ], !dbg !21055
  store float %history.i.i730.sroa.0.0.lcssa, ptr %hot_right.i733, align 4, !dbg !21056, !noalias !20820
  store float %history.i.i730.sroa.7.0.lcssa, ptr %history.i.i730.sroa.7.0.hot_right.i733.sroa_idx, align 4, !dbg !21056, !noalias !20820
  store float %history.i.i730.sroa.10.0.lcssa, ptr %history.i.i730.sroa.10.0.hot_right.i733.sroa_idx, align 4, !dbg !21056, !noalias !20820
  store float %history.i.i730.sroa.13.0.lcssa, ptr %history.i.i730.sroa.13.0.hot_right.i733.sroa_idx, align 4, !dbg !21056, !noalias !20820
  store float %history.i.i730.sroa.16.0.lcssa, ptr %history.i.i730.sroa.16.0.hot_right.i733.sroa_idx, align 4, !dbg !21056, !noalias !20820
  store float %history.i.i730.sroa.19.0.lcssa, ptr %history.i.i730.sroa.19.0.hot_right.i733.sroa_idx, align 4, !dbg !21056, !noalias !20820
  store float %history.i.i730.sroa.22.0.lcssa, ptr %history.i.i730.sroa.22.0.hot_right.i733.sroa_idx, align 4, !dbg !21056, !noalias !20820
  store float %history.i.i730.sroa.26.0.lcssa, ptr %history.i.i730.sroa.26.0.hot_right.i733.sroa_idx, align 4, !dbg !21056, !noalias !20820
  store float %history.i.i730.sroa.29.0.lcssa, ptr %history.i.i730.sroa.29.0.hot_right.i733.sroa_idx, align 4, !dbg !21056, !noalias !20820
  store float %history.i.i730.sroa.32.0.lcssa, ptr %history.i.i730.sroa.32.0.hot_right.i733.sroa_idx, align 4, !dbg !21056, !noalias !20820
  store float %history.i.i730.sroa.35.0.lcssa, ptr %history.i.i730.sroa.35.0.hot_right.i733.sroa_idx, align 4, !dbg !21056, !noalias !20820
  store float %history.i.i730.sroa.38.0.lcssa, ptr %history.i.i730.sroa.38.0.hot_right.i733.sroa_idx, align 4, !dbg !21056, !noalias !20820
  br i1 %_2.i45898154.not, label %bb13.i.loopexit, label %bb50.i951.lr.ph, !dbg !20529

bb50.i951.lr.ph:                                  ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i947
  %_8.i30.i = load float, ptr %_68.i, align 4, !alias.scope !21057, !noalias !21060, !noundef !10
  %_9.i31.i = load float, ptr %_69.i952, align 4, !alias.scope !21062, !noalias !21063, !noundef !10
  %_8.i.i954 = load float, ptr %_73.i953, align 4, !alias.scope !21064, !noalias !21067, !noundef !10
  %_9.i.i955 = load float, ptr %_74.i, align 4, !alias.scope !21069, !noalias !21070, !noundef !10
  %_91.i = load i32, ptr %349, align 4
  %_62.i88.i = load float, ptr %360, align 4
  %_62.i.i975 = load float, ptr %376, align 4
  %_106.i = load i32, ptr %380, align 4
  %.promoted8236 = load float, ptr %359, align 4
  %.promoted8305 = load float, ptr %361, align 4
  %.promoted8374 = load float, ptr %375, align 4
  %.promoted8443 = load float, ptr %377, align 4
  br label %bb50.i951, !dbg !20529

bb50.i951:                                        ; preds = %bb50.i951.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3625
  %_0.i37328444 = phi float [ %.promoted8443, %bb50.i951.lr.ph ], [ %_0.i3732, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3625 ]
  %_0.i33588375 = phi float [ %.promoted8374, %bb50.i951.lr.ph ], [ %_0.i3358, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3625 ]
  %_0.i37368306 = phi float [ %.promoted8305, %bb50.i951.lr.ph ], [ %_0.i3736, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3625 ]
  %_0.i33628237 = phi float [ %.promoted8236, %bb50.i951.lr.ph ], [ %_0.i3362, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3625 ]
  %main_cursor.sroa.0.1.i9508233 = phi i32 [ %main_cursor.sroa.0.0.i7498516, %bb50.i951.lr.ph ], [ %spec.store.select11.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3625 ]
  %ring_cursor.sroa.0.1.i9498232 = phi i32 [ %ring_cursor.sroa.0.0.i7488515, %bb50.i951.lr.ph ], [ %spec.store.select12.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3625 ]
  %iter1.sroa.0.0.i9488231 = phi i32 [ 0, %bb50.i951.lr.ph ], [ %397, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3625 ]
  %397 = add nuw nsw i32 %iter1.sroa.0.0.i9488231, 1, !dbg !21071
  %_64.i = add nuw nsw i32 %iter1.sroa.0.0.i9488231, %iter.sroa.0.0.i8513, !dbg !21077
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21057), !dbg !21078
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21062), !dbg !21078
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21064), !dbg !21079
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21069), !dbg !21079
  %_159.i956 = getelementptr inbounds nuw float, ptr %peaks_left.i732, i32 %iter1.sroa.0.0.i9488231, !dbg !21080
  %_0.i3507 = load float, ptr %_159.i956, align 4, !dbg !21091, !alias.scope !21093, !noalias !20557, !noundef !10
  %_164.i = getelementptr inbounds nuw float, ptr %peaks_right.i731, i32 %iter1.sroa.0.0.i9488231, !dbg !21096
  %_0.i3502 = load float, ptr %_164.i, align 4, !dbg !21106, !alias.scope !21108, !noalias !20557, !noundef !10
  %_3.i.i4127 = fcmp ule float %_0.i3502, %_0.i3507, !dbg !21111
  %_6.i.i4129 = bitcast float %_0.i3502 to i32, !dbg !21114
  %_8.i.i4131 = bitcast float %_0.i3507 to i32, !dbg !21117
  %_4.i.i4134 = select i1 %_3.i.i4127, i32 %_8.i.i4131, i32 %_6.i.i4129, !dbg !21119
  %_5.i3928 = and i32 %_4.i.i4134, %.none.i739, !dbg !21120
  %_7.i3931 = and i32 %_9.i3930, %_8.i.i4131, !dbg !21122
  %_4.i3932 = or disjoint i32 %_5.i3928, %_7.i3931, !dbg !21120
  %_0.i3933 = bitcast i32 %_4.i3932 to float, !dbg !21123
  %_7.i3924 = and i32 %_9.i3930, %_6.i.i4129, !dbg !21125
  %_4.i3925 = or disjoint i32 %_5.i3928, %_7.i3924, !dbg !21127
  %_0.i3926 = bitcast i32 %_4.i3925 to float, !dbg !21128
  %_165.i = icmp ugt i32 %_64.i, %left_io.1, !dbg !21130
  br i1 %_165.i, label %bb54.i, label %bb55.i960, !dbg !21130, !prof !787

bb55.i960:                                        ; preds = %bb50.i951
  %_171.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %_64.i, !dbg !21134
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21139), !dbg !21142
  %_3.not.i3495 = icmp eq i32 %left_io.1, %_64.i, !dbg !21143
  br i1 %_3.not.i3495, label %panic.i3498, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3499, !dbg !21143

panic.i3498:                                      ; preds = %bb55.i960
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #32, !dbg !21143, !noalias !21145
  unreachable, !dbg !21143

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3499: ; preds = %bb55.i960
  %_0.i3497 = load float, ptr %_171.i, align 4, !dbg !21143, !alias.scope !21139, !noalias !20557, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21146), !dbg !21149
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21150), !dbg !21149
  %width.i32.i = load i32, ptr %350, align 4, !dbg !21152, !alias.scope !21153, !noalias !21154, !noundef !10
  %_3.i2473 = fcmp uge float %_8.i30.i, %_0.i3933, !dbg !21157
  %_0.i2906 = fdiv float %_8.i30.i, %_0.i3933, !dbg !21159
  %_0.i3919 = select i1 %_3.i2473, float 1.000000e+00, float %_0.i2906, !dbg !21161
  %_158.1.i37.i = load i32, ptr %351, align 4, !dbg !21163, !alias.scope !21153, !noalias !21154, !noundef !10
  %_22.i38.i = mul i32 %width.i32.i, %ring_cursor.sroa.0.1.i9498232, !dbg !21164
  %_90.i39.i = icmp ugt i32 %_22.i38.i, %_158.1.i37.i, !dbg !21165
  br i1 %_90.i39.i, label %bb34.i123.i, label %bb35.i40.i, !dbg !21165, !prof !787

bb35.i40.i:                                       ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3499
  %_158.0.i41.i = load ptr, ptr %352, align 4, !dbg !21163, !alias.scope !21153, !noalias !21154, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21168), !dbg !21171
  %_4.not.i3650 = icmp eq i32 %_158.1.i37.i, %_22.i38.i, !dbg !21172
  br i1 %_4.not.i3650, label %panic.i3652, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3653, !dbg !21172

panic.i3652:                                      ; preds = %bb35.i40.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #32, !dbg !21172, !noalias !21174
  unreachable, !dbg !21172

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3653: ; preds = %bb35.i40.i
  %_97.i43.i = getelementptr inbounds nuw float, ptr %_158.0.i41.i, i32 %_22.i38.i, !dbg !21175
  store float %_0.i3919, ptr %_97.i43.i, align 4, !dbg !21172, !alias.scope !21168, !noalias !21177
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21178), !dbg !21181
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21182), !dbg !21181
  %width.i2204 = load i32, ptr %350, align 4, !dbg !21184, !alias.scope !21178, !noalias !21186, !noundef !10
  %398 = icmp eq i32 %width.i2204, 0, !dbg !21187
  br i1 %398, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2310, label %bb29.i2210.lr.ph, !dbg !21187

bb29.i2210.lr.ph:                                 ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3653
  %_126.1.i2215 = load i32, ptr %62, align 4, !alias.scope !21178, !noalias !21186, !noundef !10
  %_126.0.i2219 = load ptr, ptr %61, align 4, !nonnull !10
  %399 = add i32 %ring_cursor.sroa.0.1.i9498232, 1
  %_21.not.i2224 = icmp ult i32 %399, %_91.i
  %400 = select i1 %_21.not.i2224, i32 0, i32 %_91.i
  %start1.sroa.0.0.i2225 = sub nuw i32 %399, %400
  %_128.1.i2228 = load i32, ptr %351, align 4
  %_128.0.i2232 = load ptr, ptr %352, align 4, !nonnull !10
  %_130.1.i2233 = load i32, ptr %353, align 4
  %_130.0.i2237 = load ptr, ptr %354, align 4, !nonnull !10
  %_132.1.i2240 = load i32, ptr %355, align 4
  %_132.0.i2244 = load ptr, ptr %356, align 4, !nonnull !10
  %_43.i2257 = mul i32 %width.i2204, %start1.sroa.0.0.i2225
  br label %bb29.i2210, !dbg !21187

bb29.i2210:                                       ; preds = %bb29.i2210.lr.ph, %bb28.i2272
  %iter.sroa.0.0.idx.i22088212 = phi i32 [ 0, %bb29.i2210.lr.ph ], [ %iter.sroa.0.0.add.i2213, %bb28.i2272 ]
  %iter.sroa.4.0.i22078211 = phi i32 [ 0, %bb29.i2210.lr.ph ], [ %_102.0.i2214, %bb28.i2272 ]
  %iter.sroa.7.0.i22068210 = phi i32 [ %width.i2204, %bb29.i2210.lr.ph ], [ %401, %bb28.i2272 ]
  %iter.sroa.0.0.ptr.i22098213 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 %iter.sroa.0.0.idx.i22088212, !dbg !21189
  %401 = add i32 %iter.sroa.7.0.i22068210, -1, !dbg !21189
  %_109.i2211 = icmp eq i32 %iter.sroa.0.0.idx.i22088212, 32, !dbg !21190
  br i1 %_109.i2211, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2310, label %bb33.i2212, !dbg !21194

bb33.i2212:                                       ; preds = %bb29.i2210
  %iter.sroa.0.0.add.i2213 = add nuw nsw i32 %iter.sroa.0.0.idx.i22088212, 4, !dbg !21195
  %_102.0.i2214 = add nuw nsw i32 %iter.sroa.4.0.i22078211, 1, !dbg !21197
  %exitcond12516.not = icmp eq i32 %iter.sroa.4.0.i22078211, %_126.1.i2215, !dbg !21198
  br i1 %exitcond12516.not, label %panic.i2217, label %bb2.i2218, !dbg !21198

bb2.i2218:                                        ; preds = %bb33.i2212
  %402 = getelementptr inbounds nuw %LaneShape, ptr %_126.0.i2219, i32 %iter.sroa.4.0.i22078211, !dbg !21198
  %shape.i2220 = load i32, ptr %402, align 4, !dbg !21198, !noalias !21199, !noundef !10
  %403 = getelementptr inbounds nuw i8, ptr %402, i32 4, !dbg !21198
  %shape3.i2221 = load i32, ptr %403, align 4, !dbg !21198, !noalias !21199, !noundef !10
  %404 = add i32 %shape3.i2221, %ring_cursor.sroa.0.1.i9498232, !dbg !21200
  %_18.not.i2222 = icmp ult i32 %404, %_91.i, !dbg !21201
  %405 = select i1 %_18.not.i2222, i32 0, i32 %_91.i, !dbg !21201
  %spec.select.i2223 = sub nuw i32 %404, %405, !dbg !21201
  %_25.i2226 = mul i32 %spec.select.i2223, %width.i2204, !dbg !21202
  %_24.i2227 = add i32 %_25.i2226, %iter.sroa.4.0.i22078211, !dbg !21202
  %_28.i2229 = icmp ult i32 %_24.i2227, %_128.1.i2228, !dbg !21203
  br i1 %_28.i2229, label %bb9.i2231, label %panic5.i2230, !dbg !21203

panic.i2217:                                      ; preds = %bb33.i2212
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_126.1.i2215, i32 noundef %_126.1.i2215, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_12856b5f9033764dbf23e04eb77c5c46) #32, !dbg !21198, !noalias !21199
  unreachable, !dbg !21198

bb9.i2231:                                        ; preds = %bb2.i2218
  %406 = getelementptr inbounds nuw float, ptr %_128.0.i2232, i32 %_24.i2227, !dbg !21203
  %407 = load float, ptr %406, align 4, !dbg !21203, !noalias !21199, !noundef !10
  %exitcond12517.not = icmp eq i32 %iter.sroa.4.0.i22078211, %_130.1.i2233, !dbg !21204
  br i1 %exitcond12517.not, label %panic6.i2235, label %bb10.i2236, !dbg !21204

panic5.i2230:                                     ; preds = %bb2.i2218
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_24.i2227, i32 noundef %_128.1.i2228, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70ebf0cf1c90c6161926611ccf249a32) #32, !dbg !21203, !noalias !21199
  unreachable, !dbg !21203

bb10.i2236:                                       ; preds = %bb9.i2231
  %408 = getelementptr inbounds nuw i32, ptr %_130.0.i2237, i32 %iter.sroa.4.0.i22078211, !dbg !21204
  %_30.i2238 = load i32, ptr %408, align 4, !dbg !21204, !noalias !21199, !noundef !10
  %409 = icmp eq i32 %_30.i2238, 0, !dbg !21205
  br i1 %409, label %bb14.i2247, label %bb12.i2239, !dbg !21205

panic6.i2235:                                     ; preds = %bb9.i2231
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_130.1.i2233, i32 noundef %_130.1.i2233, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_023d591aaad7f5cf482ac80a9401eca1) #32, !dbg !21204, !noalias !21199
  unreachable, !dbg !21204

bb12.i2239:                                       ; preds = %bb10.i2236
  %_35.i2241 = icmp ult i32 %iter.sroa.4.0.i22078211, %_132.1.i2240, !dbg !21206
  br i1 %_35.i2241, label %bb13.i2243, label %panic7.i2242, !dbg !21206

bb14.i2247:                                       ; preds = %bb34.i2308, %bb13.i2243, %bb10.i2236
  %newest.sroa.0.0.i2248 = phi float [ %407, %bb10.i2236 ], [ %_33.i2245, %bb34.i2308 ], [ %407, %bb13.i2243 ], !dbg !21207
  %exitcond12518.not = icmp eq i32 %iter.sroa.4.0.i22078211, %_132.1.i2240, !dbg !21208
  br i1 %exitcond12518.not, label %panic8.i2251, label %bb15.i2252, !dbg !21208

bb13.i2243:                                       ; preds = %bb12.i2239
  %410 = getelementptr inbounds nuw float, ptr %_132.0.i2244, i32 %iter.sroa.4.0.i22078211, !dbg !21206
  %_33.i2245 = load float, ptr %410, align 4, !dbg !21206, !noalias !21199, !noundef !10
  %_116.i2246 = fcmp olt float %_33.i2245, %407, !dbg !21209
  br i1 %_116.i2246, label %bb34.i2308, label %bb14.i2247, !dbg !21209

panic7.i2242:                                     ; preds = %bb12.i2239
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %iter.sroa.4.0.i22078211, i32 noundef %_132.1.i2240, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a228cac3279aae3a34886f59f51fbe9e) #32, !dbg !21206, !noalias !21199
  unreachable, !dbg !21206

bb34.i2308:                                       ; preds = %bb13.i2243
  br label %bb14.i2247, !dbg !21211

bb15.i2252:                                       ; preds = %bb14.i2247
  %411 = getelementptr inbounds nuw float, ptr %_132.0.i2244, i32 %iter.sroa.4.0.i22078211, !dbg !21208
  store float %newest.sroa.0.0.i2248, ptr %411, align 4, !dbg !21208, !noalias !21199
  %_40.i2254 = add i32 %_30.i2238, 1, !dbg !21212
  %complete.i2255 = icmp eq i32 %_40.i2254, %shape.i2220, !dbg !21212
  br i1 %complete.i2255, label %bb19.i2277, label %bb17.i2256, !dbg !21213

panic8.i2251:                                     ; preds = %bb14.i2247
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_132.1.i2240, i32 noundef %_132.1.i2240, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1fa5d88c1a2cc292a2767c48606a38fe) #32, !dbg !21208, !noalias !21199
  unreachable, !dbg !21208

bb17.i2256:                                       ; preds = %bb15.i2252
  %_42.i2258 = add i32 %iter.sroa.4.0.i22078211, %_43.i2257, !dbg !21214
  %_45.i2260 = icmp ult i32 %_42.i2258, %_128.1.i2228, !dbg !21215
  br i1 %_45.i2260, label %bb27.i2270, label %panic9.i2261, !dbg !21215

panic9.i2261:                                     ; preds = %bb17.i2256
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_42.i2258, i32 noundef %_128.1.i2228, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70d40ae2302f3ea19fa0c4a65fe82786) #32, !dbg !21215, !noalias !21199
  unreachable, !dbg !21215

bb27.i2270:                                       ; preds = %bb17.i2256
  %412 = getelementptr inbounds nuw float, ptr %_128.0.i2232, i32 %_42.i2258, !dbg !21215
  %_41.i2264 = load float, ptr %412, align 4, !dbg !21215, !noalias !21199, !noundef !10
  %_117.i2265 = fcmp olt float %_41.i2264, %newest.sroa.0.0.i2248, !dbg !21216
  %newest.sroa.0.1.i2266 = select i1 %_117.i2265, float %_41.i2264, float %newest.sroa.0.0.i2248, !dbg !21216
  store float %newest.sroa.0.1.i2266, ptr %iter.sroa.0.0.ptr.i22098213, align 4, !dbg !21218, !alias.scope !21182, !noalias !21219
  br label %bb28.i2272, !dbg !21220

bb28.i2272:                                       ; preds = %bb22.i2305, %bb19.i2277, %bb27.i2270
  %storemerge5362 = phi i32 [ %_40.i2254, %bb27.i2270 ], [ 0, %bb19.i2277 ], [ 0, %bb22.i2305 ], !dbg !21221
  store i32 %storemerge5362, ptr %408, align 4, !dbg !21221, !noalias !21199
  %413 = icmp eq i32 %401, 0, !dbg !21187
  br i1 %413, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2310, label %bb29.i2210, !dbg !21187

bb19.i2277:                                       ; preds = %bb15.i2252
  store float %newest.sroa.0.0.i2248, ptr %iter.sroa.0.0.ptr.i22098213, align 4, !dbg !21218, !alias.scope !21182, !noalias !21219
  %_118.i22838206.not = icmp eq i32 %shape.i2220, 0, !dbg !21222
  br i1 %_118.i22838206.not, label %bb28.i2272, label %bb40.i2290.preheader, !dbg !21226

bb40.i2290.preheader:                             ; preds = %bb19.i2277
  %414 = load float, ptr %406, align 4, !dbg !21227, !noalias !21199, !noundef !10
  br label %bb40.i2290, !dbg !21228

bb40.i2290:                                       ; preds = %bb40.i2290.preheader, %bb22.i2305
  %iter2.sroa.0.0.i22828209 = phi i32 [ %_119.i2291, %bb22.i2305 ], [ 0, %bb40.i2290.preheader ]
  %suffix.sroa.0.0.i22818208 = phi float [ %suffix.sroa.0.1.i2301, %bb22.i2305 ], [ %414, %bb40.i2290.preheader ]
  %end.sroa.0.1.i22808207 = phi i32 [ %417, %bb22.i2305 ], [ %spec.select.i2223, %bb40.i2290.preheader ]
  %_54.i2292 = mul i32 %end.sroa.0.1.i22808207, %width.i2204, !dbg !21229
  %_53.i2293 = add i32 %_54.i2292, %iter.sroa.4.0.i22078211, !dbg !21229
  %_57.i2295 = icmp ult i32 %_53.i2293, %_128.1.i2228, !dbg !21228
  br i1 %_57.i2295, label %bb22.i2305, label %panic13.i2296, !dbg !21228

panic13.i2296:                                    ; preds = %bb40.i2290
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_53.i2293, i32 noundef %_128.1.i2228, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a271bbb7fd83c3605ea58a06b7065fa4) #32, !dbg !21228, !noalias !21199
  unreachable, !dbg !21228

bb22.i2305:                                       ; preds = %bb40.i2290
  %_119.i2291 = add nuw i32 %iter2.sroa.0.0.i22828209, 1, !dbg !21230
  %415 = getelementptr inbounds nuw float, ptr %_128.0.i2232, i32 %_53.i2293, !dbg !21228
  %_52.i2299 = load float, ptr %415, align 4, !dbg !21228, !noalias !21199, !noundef !10
  %_121.i2300 = fcmp olt float %suffix.sroa.0.0.i22818208, %_52.i2299, !dbg !21233
  %suffix.sroa.0.1.i2301 = select i1 %_121.i2300, float %suffix.sroa.0.0.i22818208, float %_52.i2299, !dbg !21233
  store float %suffix.sroa.0.1.i2301, ptr %415, align 4, !dbg !21235, !noalias !21199
  %416 = icmp eq i32 %end.sroa.0.1.i22808207, 0, !dbg !21236
  %spec.store.select.i2307 = select i1 %416, i32 %_91.i, i32 %end.sroa.0.1.i22808207, !dbg !21236
  %417 = add i32 %spec.store.select.i2307, -1, !dbg !21237
  %exitcond12515.not = icmp eq i32 %_119.i2291, %shape.i2220, !dbg !21222
  br i1 %exitcond12515.not, label %bb28.i2272, label %bb40.i2290, !dbg !21226

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2310: ; preds = %bb29.i2210, %bb28.i2272, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3653
  %_0.i3494 = load float, ptr %scratch.i, align 4, !dbg !21238, !alias.scope !21240, !noalias !21243, !noundef !10
  %_0.i3038 = fmul float %_0.i3494, 1.638400e+04, !dbg !21244
  %418 = tail call noundef float @llvm.floor.f32(float %_0.i3038), !dbg !21246
  %_0.i3037 = fmul float %418, 0x3F10000000000000, !dbg !21250
  %419 = icmp eq i32 %width.i32.i, 0, !dbg !21252
  %_163.1.i81.i.pre = load i32, ptr %357, align 4, !dbg !21254, !alias.scope !21153, !noalias !21154
  br i1 %419, label %bb53.i76.i, label %bb36.i55.i.lr.ph, !dbg !21252

bb36.i55.i.lr.ph:                                 ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2310
  %_159.1.i60.i = load i32, ptr %62, align 4, !alias.scope !21153, !noalias !21154, !noundef !10
  %_159.0.i64.i = load ptr, ptr %61, align 4, !nonnull !10
  %_161.0.i74.i = load ptr, ptr %358, align 4, !nonnull !10
  %exitcond12519.not = icmp eq i32 %_159.1.i60.i, 0, !dbg !21255
  br i1 %exitcond12519.not, label %panic.i62.i, label %bb14.i63.i, !dbg !21255

bb34.i123.i:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3499
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i38.i, i32 noundef %_158.1.i37.i, i32 noundef %_158.1.i37.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_db7c42041d7aaaba4839906f37294547) #32, !dbg !21256, !noalias !21177
  unreachable, !dbg !21256

bb53.i76.i:                                       ; preds = %bb18.i73.i.7, %bb18.i73.i, %bb18.i73.i.1, %bb18.i73.i.2, %bb18.i73.i.3, %bb18.i73.i.4, %bb18.i73.i.5, %bb18.i73.i.6, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2310
  %_0.i3492 = phi float [ %_0.i3494, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2310 ], [ %_47.i75.i, %bb18.i73.i ], [ %_47.i75.i, %bb18.i73.i.7 ], [ %_47.i75.i, %bb18.i73.i.6 ], [ %_47.i75.i, %bb18.i73.i.5 ], [ %_47.i75.i, %bb18.i73.i.4 ], [ %_47.i75.i, %bb18.i73.i.3 ], [ %_47.i75.i, %bb18.i73.i.2 ], [ %_47.i75.i, %bb18.i73.i.1 ], !dbg !21257
  %_0.i2606 = fadd float %_0.i3037, %_0.i33628237, !dbg !21259
  %_0.i3362 = fsub float %_0.i2606, %_0.i3492, !dbg !21261
  %_123.i82.i = icmp ugt i32 %_22.i38.i, %_163.1.i81.i.pre, !dbg !21263
  br i1 %_123.i82.i, label %bb41.i122.i, label %bb42.i83.i, !dbg !21263, !prof !787

bb14.i63.i:                                       ; preds = %bb36.i55.i.lr.ph
  %420 = getelementptr inbounds nuw i8, ptr %_159.0.i64.i, i32 8, !dbg !21255
  %_42.i65.i = load i32, ptr %420, align 4, !dbg !21255, !noalias !21243, !noundef !10
  %421 = add i32 %_42.i65.i, %ring_cursor.sroa.0.1.i9498232, !dbg !21266
  %_45.not.i66.i = icmp ult i32 %421, %_91.i, !dbg !21267
  %422 = select i1 %_45.not.i66.i, i32 0, i32 %_91.i, !dbg !21267
  %spec.select.i67.i = sub nuw i32 %421, %422, !dbg !21267
  %_49.i68.i = mul i32 %spec.select.i67.i, %width.i32.i, !dbg !21268
  %_51.i71.i = icmp ult i32 %_49.i68.i, %_163.1.i81.i.pre, !dbg !21269
  br i1 %_51.i71.i, label %bb18.i73.i, label %panic1.i72.i, !dbg !21269

panic.i62.i:                                      ; preds = %bb36.i55.i.7, %bb36.i55.i.6, %bb36.i55.i.5, %bb36.i55.i.4, %bb36.i55.i.3, %bb36.i55.i.2, %bb36.i55.i.1, %bb36.i55.i.lr.ph
  %_159.1.i60.i.lcssa.ph = phi i32 [ 7, %bb36.i55.i.7 ], [ 6, %bb36.i55.i.6 ], [ 5, %bb36.i55.i.5 ], [ 4, %bb36.i55.i.4 ], [ 3, %bb36.i55.i.3 ], [ 2, %bb36.i55.i.2 ], [ 1, %bb36.i55.i.1 ], [ 0, %bb36.i55.i.lr.ph ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_159.1.i60.i.lcssa.ph, i32 noundef %_159.1.i60.i.lcssa.ph, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_33746731a929bad63709ddedbca82f5f) #32, !dbg !21255, !noalias !21243
  unreachable, !dbg !21255

bb18.i73.i:                                       ; preds = %bb14.i63.i
  %423 = getelementptr inbounds nuw float, ptr %_161.0.i74.i, i32 %_49.i68.i, !dbg !21269
  %_47.i75.i = load float, ptr %423, align 4, !dbg !21269, !noalias !21243, !noundef !10
  store float %_47.i75.i, ptr %scratch.i, align 4, !dbg !21270, !alias.scope !21150, !noalias !21271
  %424 = icmp eq i32 %width.i32.i, 1, !dbg !21252
  br i1 %424, label %bb53.i76.i, label %bb36.i55.i.1, !dbg !21252

bb36.i55.i.1:                                     ; preds = %bb18.i73.i
  %exitcond12519.1.not = icmp eq i32 %_159.1.i60.i, 1, !dbg !21255
  br i1 %exitcond12519.1.not, label %panic.i62.i, label %bb14.i63.i.1, !dbg !21255

bb14.i63.i.1:                                     ; preds = %bb36.i55.i.1
  %425 = getelementptr inbounds nuw i8, ptr %_159.0.i64.i, i32 20, !dbg !21255
  %_42.i65.i.1 = load i32, ptr %425, align 4, !dbg !21255, !noalias !21243, !noundef !10
  %426 = add i32 %_42.i65.i.1, %ring_cursor.sroa.0.1.i9498232, !dbg !21266
  %_45.not.i66.i.1 = icmp ult i32 %426, %_91.i, !dbg !21267
  %427 = select i1 %_45.not.i66.i.1, i32 0, i32 %_91.i, !dbg !21267
  %spec.select.i67.i.1 = sub nuw i32 %426, %427, !dbg !21267
  %_49.i68.i.1 = mul i32 %spec.select.i67.i.1, %width.i32.i, !dbg !21268
  %_48.i69.i.1 = add i32 %_49.i68.i.1, 1, !dbg !21268
  %_51.i71.i.1 = icmp ult i32 %_48.i69.i.1, %_163.1.i81.i.pre, !dbg !21269
  br i1 %_51.i71.i.1, label %bb18.i73.i.1, label %panic1.i72.i, !dbg !21269

bb18.i73.i.1:                                     ; preds = %bb14.i63.i.1
  %428 = getelementptr inbounds nuw float, ptr %_161.0.i74.i, i32 %_48.i69.i.1, !dbg !21269
  %_47.i75.i.1 = load float, ptr %428, align 4, !dbg !21269, !noalias !21243, !noundef !10
  store float %_47.i75.i.1, ptr %iter.sroa.0.0.ptr.i54.i8217.1, align 4, !dbg !21270, !alias.scope !21150, !noalias !21271
  %429 = icmp eq i32 %width.i32.i, 2, !dbg !21252
  br i1 %429, label %bb53.i76.i, label %bb36.i55.i.2, !dbg !21252

bb36.i55.i.2:                                     ; preds = %bb18.i73.i.1
  %exitcond12519.2.not = icmp eq i32 %_159.1.i60.i, 2, !dbg !21255
  br i1 %exitcond12519.2.not, label %panic.i62.i, label %bb14.i63.i.2, !dbg !21255

bb14.i63.i.2:                                     ; preds = %bb36.i55.i.2
  %430 = getelementptr inbounds nuw i8, ptr %_159.0.i64.i, i32 32, !dbg !21255
  %_42.i65.i.2 = load i32, ptr %430, align 4, !dbg !21255, !noalias !21243, !noundef !10
  %431 = add i32 %_42.i65.i.2, %ring_cursor.sroa.0.1.i9498232, !dbg !21266
  %_45.not.i66.i.2 = icmp ult i32 %431, %_91.i, !dbg !21267
  %432 = select i1 %_45.not.i66.i.2, i32 0, i32 %_91.i, !dbg !21267
  %spec.select.i67.i.2 = sub nuw i32 %431, %432, !dbg !21267
  %_49.i68.i.2 = mul i32 %spec.select.i67.i.2, %width.i32.i, !dbg !21268
  %_48.i69.i.2 = add i32 %_49.i68.i.2, 2, !dbg !21268
  %_51.i71.i.2 = icmp ult i32 %_48.i69.i.2, %_163.1.i81.i.pre, !dbg !21269
  br i1 %_51.i71.i.2, label %bb18.i73.i.2, label %panic1.i72.i, !dbg !21269

bb18.i73.i.2:                                     ; preds = %bb14.i63.i.2
  %433 = getelementptr inbounds nuw float, ptr %_161.0.i74.i, i32 %_48.i69.i.2, !dbg !21269
  %_47.i75.i.2 = load float, ptr %433, align 4, !dbg !21269, !noalias !21243, !noundef !10
  store float %_47.i75.i.2, ptr %iter.sroa.0.0.ptr.i54.i8217.2, align 4, !dbg !21270, !alias.scope !21150, !noalias !21271
  %434 = icmp eq i32 %width.i32.i, 3, !dbg !21252
  br i1 %434, label %bb53.i76.i, label %bb36.i55.i.3, !dbg !21252

bb36.i55.i.3:                                     ; preds = %bb18.i73.i.2
  %exitcond12519.3.not = icmp eq i32 %_159.1.i60.i, 3, !dbg !21255
  br i1 %exitcond12519.3.not, label %panic.i62.i, label %bb14.i63.i.3, !dbg !21255

bb14.i63.i.3:                                     ; preds = %bb36.i55.i.3
  %435 = getelementptr inbounds nuw i8, ptr %_159.0.i64.i, i32 44, !dbg !21255
  %_42.i65.i.3 = load i32, ptr %435, align 4, !dbg !21255, !noalias !21243, !noundef !10
  %436 = add i32 %_42.i65.i.3, %ring_cursor.sroa.0.1.i9498232, !dbg !21266
  %_45.not.i66.i.3 = icmp ult i32 %436, %_91.i, !dbg !21267
  %437 = select i1 %_45.not.i66.i.3, i32 0, i32 %_91.i, !dbg !21267
  %spec.select.i67.i.3 = sub nuw i32 %436, %437, !dbg !21267
  %_49.i68.i.3 = mul i32 %spec.select.i67.i.3, %width.i32.i, !dbg !21268
  %_48.i69.i.3 = add i32 %_49.i68.i.3, 3, !dbg !21268
  %_51.i71.i.3 = icmp ult i32 %_48.i69.i.3, %_163.1.i81.i.pre, !dbg !21269
  br i1 %_51.i71.i.3, label %bb18.i73.i.3, label %panic1.i72.i, !dbg !21269

bb18.i73.i.3:                                     ; preds = %bb14.i63.i.3
  %438 = getelementptr inbounds nuw float, ptr %_161.0.i74.i, i32 %_48.i69.i.3, !dbg !21269
  %_47.i75.i.3 = load float, ptr %438, align 4, !dbg !21269, !noalias !21243, !noundef !10
  store float %_47.i75.i.3, ptr %iter.sroa.0.0.ptr.i54.i8217.3, align 4, !dbg !21270, !alias.scope !21150, !noalias !21271
  %439 = icmp eq i32 %width.i32.i, 4, !dbg !21252
  br i1 %439, label %bb53.i76.i, label %bb36.i55.i.4, !dbg !21252

bb36.i55.i.4:                                     ; preds = %bb18.i73.i.3
  %exitcond12519.4.not = icmp eq i32 %_159.1.i60.i, 4, !dbg !21255
  br i1 %exitcond12519.4.not, label %panic.i62.i, label %bb14.i63.i.4, !dbg !21255

bb14.i63.i.4:                                     ; preds = %bb36.i55.i.4
  %440 = getelementptr inbounds nuw i8, ptr %_159.0.i64.i, i32 56, !dbg !21255
  %_42.i65.i.4 = load i32, ptr %440, align 4, !dbg !21255, !noalias !21243, !noundef !10
  %441 = add i32 %_42.i65.i.4, %ring_cursor.sroa.0.1.i9498232, !dbg !21266
  %_45.not.i66.i.4 = icmp ult i32 %441, %_91.i, !dbg !21267
  %442 = select i1 %_45.not.i66.i.4, i32 0, i32 %_91.i, !dbg !21267
  %spec.select.i67.i.4 = sub nuw i32 %441, %442, !dbg !21267
  %_49.i68.i.4 = mul i32 %spec.select.i67.i.4, %width.i32.i, !dbg !21268
  %_48.i69.i.4 = add i32 %_49.i68.i.4, 4, !dbg !21268
  %_51.i71.i.4 = icmp ult i32 %_48.i69.i.4, %_163.1.i81.i.pre, !dbg !21269
  br i1 %_51.i71.i.4, label %bb18.i73.i.4, label %panic1.i72.i, !dbg !21269

bb18.i73.i.4:                                     ; preds = %bb14.i63.i.4
  %443 = getelementptr inbounds nuw float, ptr %_161.0.i74.i, i32 %_48.i69.i.4, !dbg !21269
  %_47.i75.i.4 = load float, ptr %443, align 4, !dbg !21269, !noalias !21243, !noundef !10
  store float %_47.i75.i.4, ptr %iter.sroa.0.0.ptr.i54.i8217.4, align 4, !dbg !21270, !alias.scope !21150, !noalias !21271
  %444 = icmp eq i32 %width.i32.i, 5, !dbg !21252
  br i1 %444, label %bb53.i76.i, label %bb36.i55.i.5, !dbg !21252

bb36.i55.i.5:                                     ; preds = %bb18.i73.i.4
  %exitcond12519.5.not = icmp eq i32 %_159.1.i60.i, 5, !dbg !21255
  br i1 %exitcond12519.5.not, label %panic.i62.i, label %bb14.i63.i.5, !dbg !21255

bb14.i63.i.5:                                     ; preds = %bb36.i55.i.5
  %445 = getelementptr inbounds nuw i8, ptr %_159.0.i64.i, i32 68, !dbg !21255
  %_42.i65.i.5 = load i32, ptr %445, align 4, !dbg !21255, !noalias !21243, !noundef !10
  %446 = add i32 %_42.i65.i.5, %ring_cursor.sroa.0.1.i9498232, !dbg !21266
  %_45.not.i66.i.5 = icmp ult i32 %446, %_91.i, !dbg !21267
  %447 = select i1 %_45.not.i66.i.5, i32 0, i32 %_91.i, !dbg !21267
  %spec.select.i67.i.5 = sub nuw i32 %446, %447, !dbg !21267
  %_49.i68.i.5 = mul i32 %spec.select.i67.i.5, %width.i32.i, !dbg !21268
  %_48.i69.i.5 = add i32 %_49.i68.i.5, 5, !dbg !21268
  %_51.i71.i.5 = icmp ult i32 %_48.i69.i.5, %_163.1.i81.i.pre, !dbg !21269
  br i1 %_51.i71.i.5, label %bb18.i73.i.5, label %panic1.i72.i, !dbg !21269

bb18.i73.i.5:                                     ; preds = %bb14.i63.i.5
  %448 = getelementptr inbounds nuw float, ptr %_161.0.i74.i, i32 %_48.i69.i.5, !dbg !21269
  %_47.i75.i.5 = load float, ptr %448, align 4, !dbg !21269, !noalias !21243, !noundef !10
  store float %_47.i75.i.5, ptr %iter.sroa.0.0.ptr.i54.i8217.5, align 4, !dbg !21270, !alias.scope !21150, !noalias !21271
  %449 = icmp eq i32 %width.i32.i, 6, !dbg !21252
  br i1 %449, label %bb53.i76.i, label %bb36.i55.i.6, !dbg !21252

bb36.i55.i.6:                                     ; preds = %bb18.i73.i.5
  %exitcond12519.6.not = icmp eq i32 %_159.1.i60.i, 6, !dbg !21255
  br i1 %exitcond12519.6.not, label %panic.i62.i, label %bb14.i63.i.6, !dbg !21255

bb14.i63.i.6:                                     ; preds = %bb36.i55.i.6
  %450 = getelementptr inbounds nuw i8, ptr %_159.0.i64.i, i32 80, !dbg !21255
  %_42.i65.i.6 = load i32, ptr %450, align 4, !dbg !21255, !noalias !21243, !noundef !10
  %451 = add i32 %_42.i65.i.6, %ring_cursor.sroa.0.1.i9498232, !dbg !21266
  %_45.not.i66.i.6 = icmp ult i32 %451, %_91.i, !dbg !21267
  %452 = select i1 %_45.not.i66.i.6, i32 0, i32 %_91.i, !dbg !21267
  %spec.select.i67.i.6 = sub nuw i32 %451, %452, !dbg !21267
  %_49.i68.i.6 = mul i32 %spec.select.i67.i.6, %width.i32.i, !dbg !21268
  %_48.i69.i.6 = add i32 %_49.i68.i.6, 6, !dbg !21268
  %_51.i71.i.6 = icmp ult i32 %_48.i69.i.6, %_163.1.i81.i.pre, !dbg !21269
  br i1 %_51.i71.i.6, label %bb18.i73.i.6, label %panic1.i72.i, !dbg !21269

bb18.i73.i.6:                                     ; preds = %bb14.i63.i.6
  %453 = getelementptr inbounds nuw float, ptr %_161.0.i74.i, i32 %_48.i69.i.6, !dbg !21269
  %_47.i75.i.6 = load float, ptr %453, align 4, !dbg !21269, !noalias !21243, !noundef !10
  store float %_47.i75.i.6, ptr %iter.sroa.0.0.ptr.i54.i8217.6, align 4, !dbg !21270, !alias.scope !21150, !noalias !21271
  %454 = icmp eq i32 %width.i32.i, 7, !dbg !21252
  br i1 %454, label %bb53.i76.i, label %bb36.i55.i.7, !dbg !21252

bb36.i55.i.7:                                     ; preds = %bb18.i73.i.6
  %exitcond12519.7.not = icmp eq i32 %_159.1.i60.i, 7, !dbg !21255
  br i1 %exitcond12519.7.not, label %panic.i62.i, label %bb14.i63.i.7, !dbg !21255

bb14.i63.i.7:                                     ; preds = %bb36.i55.i.7
  %455 = getelementptr inbounds nuw i8, ptr %_159.0.i64.i, i32 92, !dbg !21255
  %_42.i65.i.7 = load i32, ptr %455, align 4, !dbg !21255, !noalias !21243, !noundef !10
  %456 = add i32 %_42.i65.i.7, %ring_cursor.sroa.0.1.i9498232, !dbg !21266
  %_45.not.i66.i.7 = icmp ult i32 %456, %_91.i, !dbg !21267
  %457 = select i1 %_45.not.i66.i.7, i32 0, i32 %_91.i, !dbg !21267
  %spec.select.i67.i.7 = sub nuw i32 %456, %457, !dbg !21267
  %_49.i68.i.7 = mul i32 %spec.select.i67.i.7, %width.i32.i, !dbg !21268
  %_48.i69.i.7 = add i32 %_49.i68.i.7, 7, !dbg !21268
  %_51.i71.i.7 = icmp ult i32 %_48.i69.i.7, %_163.1.i81.i.pre, !dbg !21269
  br i1 %_51.i71.i.7, label %bb18.i73.i.7, label %panic1.i72.i, !dbg !21269

bb18.i73.i.7:                                     ; preds = %bb14.i63.i.7
  %458 = getelementptr inbounds nuw float, ptr %_161.0.i74.i, i32 %_48.i69.i.7, !dbg !21269
  %_47.i75.i.7 = load float, ptr %458, align 4, !dbg !21269, !noalias !21243, !noundef !10
  store float %_47.i75.i.7, ptr %iter.sroa.0.0.ptr.i54.i8217.7, align 4, !dbg !21270, !alias.scope !21150, !noalias !21271
  br label %bb53.i76.i, !dbg !21252

panic1.i72.i:                                     ; preds = %bb14.i63.i.7, %bb14.i63.i.6, %bb14.i63.i.5, %bb14.i63.i.4, %bb14.i63.i.3, %bb14.i63.i.2, %bb14.i63.i.1, %bb14.i63.i
  %_48.i69.i.lcssa.ph = phi i32 [ %_48.i69.i.7, %bb14.i63.i.7 ], [ %_48.i69.i.6, %bb14.i63.i.6 ], [ %_48.i69.i.5, %bb14.i63.i.5 ], [ %_48.i69.i.4, %bb14.i63.i.4 ], [ %_48.i69.i.3, %bb14.i63.i.3 ], [ %_48.i69.i.2, %bb14.i63.i.2 ], [ %_48.i69.i.1, %bb14.i63.i.1 ], [ %_49.i68.i, %bb14.i63.i ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_48.i69.i.lcssa.ph, i32 noundef %_163.1.i81.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_0e76a26fbb751dfb8cf8a069b5b0d39a) #32, !dbg !21269, !noalias !21243
  unreachable, !dbg !21269

bb42.i83.i:                                       ; preds = %bb53.i76.i
  %_163.0.i84.i = load ptr, ptr %358, align 4, !dbg !21254, !alias.scope !21153, !noalias !21154, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21272), !dbg !21275
  %_4.not.i3646 = icmp eq i32 %_163.1.i81.i.pre, %_22.i38.i, !dbg !21276
  br i1 %_4.not.i3646, label %panic.i3648, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3649, !dbg !21276

panic.i3648:                                      ; preds = %bb42.i83.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #32, !dbg !21276, !noalias !21278
  unreachable, !dbg !21276

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3649: ; preds = %bb42.i83.i
  %_130.i86.i = getelementptr inbounds nuw float, ptr %_163.0.i84.i, i32 %_22.i38.i, !dbg !21279
  store float %_0.i3037, ptr %_130.i86.i, align 4, !dbg !21276, !alias.scope !21272, !noalias !21243
  %_0.i2905 = fdiv float %_0.i3362, %_62.i88.i, !dbg !21281
  %_0.i3361 = fsub float 1.000000e+00, %_0.i2905, !dbg !21283
  %_0.i3360 = fsub float %_0.i3361, %_0.i37368306, !dbg !21285
  %_4.i2921 = fmul float %_9.i31.i, %_0.i3360, !dbg !21287
  %_0.i2922 = fadd float %_0.i37368306, %_4.i2921, !dbg !21287
  %_3.i.i4118.inv = fcmp ogt float %_0.i3361, %_0.i2922, !dbg !21289
  %_4.i.i4125.v = select i1 %_3.i.i4118.inv, float %_0.i3361, float %_0.i2922, !dbg !21289
  %459 = tail call noundef float @llvm.fabs.f32(float %_4.i.i4125.v), !dbg !21292
  %460 = fcmp uge float %459, 0x3BC79CA100000000, !dbg !21295
  %_0.i3736 = select i1 %460, float %_4.i.i4125.v, float 0.000000e+00, !dbg !21297
  %_0.i3359 = fsub float 1.000000e+00, %_0.i3736, !dbg !21298
  %_164.1.i100.i = load i32, ptr %362, align 4, !dbg !21300, !alias.scope !21153, !noalias !21154, !noundef !10
  %_74.i101.i = mul i32 %width.i32.i, %main_cursor.sroa.0.1.i9508233, !dbg !21301
  %_134.i102.i = icmp ugt i32 %_74.i101.i, %_164.1.i100.i, !dbg !21302
  br i1 %_134.i102.i, label %bb47.i121.i, label %bb48.i103.i, !dbg !21302, !prof !787

bb41.i122.i:                                      ; preds = %bb53.i76.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i38.i, i32 noundef %_163.1.i81.i.pre, i32 noundef %_163.1.i81.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c2286ff41a33b8c11aa270fe215441de) #32, !dbg !21305, !noalias !21243
  unreachable, !dbg !21305

bb48.i103.i:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3649
  %_164.0.i104.i = load ptr, ptr %363, align 4, !dbg !21300, !alias.scope !21153, !noalias !21154, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21306), !dbg !21309
  %_3.not.i3486 = icmp eq i32 %_164.1.i100.i, %_74.i101.i, !dbg !21310
  br i1 %_3.not.i3486, label %panic.i3489, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3641, !dbg !21310

panic.i3489:                                      ; preds = %bb48.i103.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #32, !dbg !21310, !noalias !21312
  unreachable, !dbg !21310

bb47.i121.i:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3649
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_74.i101.i, i32 noundef %_164.1.i100.i, i32 noundef %_164.1.i100.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a3a21efd18b403ec25db545d522c479) #32, !dbg !21313, !noalias !21243
  unreachable, !dbg !21313

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3641: ; preds = %bb48.i103.i
  %_141.i106.i = getelementptr inbounds nuw float, ptr %_164.0.i104.i, i32 %_74.i101.i, !dbg !21314
  %_0.i3488 = load float, ptr %_141.i106.i, align 4, !dbg !21310, !alias.scope !21306, !noalias !21243, !noundef !10
  store float %_0.i3497, ptr %_141.i106.i, align 4, !dbg !21316, !alias.scope !21318, !noalias !21243
  %_0.i3036 = fmul float %_0.i3359, %_0.i3488, !dbg !21321
  %_6.i3907 = bitcast float %_0.i3488 to i32, !dbg !21323
  %_5.i3908 = and i32 %_6.i3907, %all.sroa.0.0.i741, !dbg !21326
  %_8.i3909 = bitcast float %_0.i3036 to i32, !dbg !21327
  %_7.i3911 = and i32 %_9.i3910, %_8.i3909, !dbg !21329
  %_4.i3912 = or disjoint i32 %_7.i3911, %_5.i3908, !dbg !21326
  store i32 %_4.i3912, ptr %_171.i, align 4, !dbg !21330, !alias.scope !21332, !noalias !21335
  %_172.i = icmp ugt i32 %_64.i, %right_io.1, !dbg !21336
  br i1 %_172.i, label %bb56.i981, label %bb57.i961, !dbg !21336, !prof !787

bb54.i:                                           ; preds = %bb50.i951
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_64.i, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_adfae95437a74b76017017a3a282b9f2) #32, !dbg !21340, !noalias !20557
  unreachable, !dbg !21340

bb57.i961:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3641
  %_178.i = getelementptr inbounds nuw float, ptr %right_io.0, i32 %_64.i, !dbg !21341
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21346), !dbg !21349
  %_3.not.i3481 = icmp eq i32 %right_io.1, %_64.i, !dbg !21350
  br i1 %_3.not.i3481, label %panic.i3484, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3485, !dbg !21350

panic.i3484:                                      ; preds = %bb57.i961
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #32, !dbg !21350, !noalias !21352
  unreachable, !dbg !21350

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3485: ; preds = %bb57.i961
  %_0.i3483 = load float, ptr %_178.i, align 4, !dbg !21350, !alias.scope !21346, !noalias !20557, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21353), !dbg !21356
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21357), !dbg !21356
  %width.i.i = load i32, ptr %364, align 4, !dbg !21359, !alias.scope !21360, !noalias !21361, !noundef !10
  %_3.i2471 = fcmp uge float %_8.i.i954, %_0.i3926, !dbg !21364
  %_0.i2904 = fdiv float %_8.i.i954, %_0.i3926, !dbg !21366
  %_0.i3906 = select i1 %_3.i2471, float 1.000000e+00, float %_0.i2904, !dbg !21368
  %_158.1.i.i = load i32, ptr %365, align 4, !dbg !21370, !alias.scope !21360, !noalias !21361, !noundef !10
  %_22.i.i966 = mul i32 %width.i.i, %ring_cursor.sroa.0.1.i9498232, !dbg !21371
  %_90.i.i = icmp ugt i32 %_22.i.i966, %_158.1.i.i, !dbg !21372
  br i1 %_90.i.i, label %bb34.i.i, label %bb35.i.i, !dbg !21372, !prof !787

bb35.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3485
  %_158.0.i.i = load ptr, ptr %366, align 4, !dbg !21370, !alias.scope !21360, !noalias !21361, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21375), !dbg !21378
  %_4.not.i3634 = icmp eq i32 %_158.1.i.i, %_22.i.i966, !dbg !21379
  br i1 %_4.not.i3634, label %panic.i3636, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3637, !dbg !21379

panic.i3636:                                      ; preds = %bb35.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #32, !dbg !21379, !noalias !21381
  unreachable, !dbg !21379

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3637: ; preds = %bb35.i.i
  %_97.i.i = getelementptr inbounds nuw float, ptr %_158.0.i.i, i32 %_22.i.i966, !dbg !21382
  store float %_0.i3906, ptr %_97.i.i, align 4, !dbg !21379, !alias.scope !21375, !noalias !21384
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21385), !dbg !21388
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21389), !dbg !21388
  %width.i2097 = load i32, ptr %364, align 4, !dbg !21391, !alias.scope !21385, !noalias !21393, !noundef !10
  %461 = icmp eq i32 %width.i2097, 0, !dbg !21394
  br i1 %461, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2203, label %bb29.i2103.lr.ph, !dbg !21394

bb29.i2103.lr.ph:                                 ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3637
  %_126.1.i2108 = load i32, ptr %367, align 4, !alias.scope !21385, !noalias !21393, !noundef !10
  %_126.0.i2112 = load ptr, ptr %368, align 4, !nonnull !10
  %462 = add i32 %ring_cursor.sroa.0.1.i9498232, 1
  %_21.not.i2117 = icmp ult i32 %462, %_91.i
  %463 = select i1 %_21.not.i2117, i32 0, i32 %_91.i
  %start1.sroa.0.0.i2118 = sub nuw i32 %462, %463
  %_128.1.i2121 = load i32, ptr %365, align 4
  %_128.0.i2125 = load ptr, ptr %366, align 4, !nonnull !10
  %_130.1.i2126 = load i32, ptr %369, align 4
  %_130.0.i2130 = load ptr, ptr %370, align 4, !nonnull !10
  %_132.1.i2133 = load i32, ptr %371, align 4
  %_132.0.i2137 = load ptr, ptr %372, align 4, !nonnull !10
  %_43.i2150 = mul i32 %width.i2097, %start1.sroa.0.0.i2118
  br label %bb29.i2103, !dbg !21394

bb29.i2103:                                       ; preds = %bb29.i2103.lr.ph, %bb28.i2165
  %iter.sroa.0.0.idx.i21018224 = phi i32 [ 0, %bb29.i2103.lr.ph ], [ %iter.sroa.0.0.add.i2106, %bb28.i2165 ]
  %iter.sroa.4.0.i21008223 = phi i32 [ 0, %bb29.i2103.lr.ph ], [ %_102.0.i2107, %bb28.i2165 ]
  %iter.sroa.7.0.i20998222 = phi i32 [ %width.i2097, %bb29.i2103.lr.ph ], [ %464, %bb28.i2165 ]
  %iter.sroa.0.0.ptr.i21028225 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 %iter.sroa.0.0.idx.i21018224, !dbg !21396
  %464 = add i32 %iter.sroa.7.0.i20998222, -1, !dbg !21396
  %_109.i2104 = icmp eq i32 %iter.sroa.0.0.idx.i21018224, 32, !dbg !21397
  br i1 %_109.i2104, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2203.loopexit, label %bb33.i2105, !dbg !21401

bb33.i2105:                                       ; preds = %bb29.i2103
  %iter.sroa.0.0.add.i2106 = add nuw nsw i32 %iter.sroa.0.0.idx.i21018224, 4, !dbg !21402
  %_102.0.i2107 = add nuw nsw i32 %iter.sroa.4.0.i21008223, 1, !dbg !21404
  %exitcond12521.not = icmp eq i32 %iter.sroa.4.0.i21008223, %_126.1.i2108, !dbg !21405
  br i1 %exitcond12521.not, label %panic.i2110, label %bb2.i2111, !dbg !21405

bb2.i2111:                                        ; preds = %bb33.i2105
  %465 = getelementptr inbounds nuw %LaneShape, ptr %_126.0.i2112, i32 %iter.sroa.4.0.i21008223, !dbg !21405
  %shape.i2113 = load i32, ptr %465, align 4, !dbg !21405, !noalias !21406, !noundef !10
  %466 = getelementptr inbounds nuw i8, ptr %465, i32 4, !dbg !21405
  %shape3.i2114 = load i32, ptr %466, align 4, !dbg !21405, !noalias !21406, !noundef !10
  %467 = add i32 %shape3.i2114, %ring_cursor.sroa.0.1.i9498232, !dbg !21407
  %_18.not.i2115 = icmp ult i32 %467, %_91.i, !dbg !21408
  %468 = select i1 %_18.not.i2115, i32 0, i32 %_91.i, !dbg !21408
  %spec.select.i2116 = sub nuw i32 %467, %468, !dbg !21408
  %_25.i2119 = mul i32 %spec.select.i2116, %width.i2097, !dbg !21409
  %_24.i2120 = add i32 %_25.i2119, %iter.sroa.4.0.i21008223, !dbg !21409
  %_28.i2122 = icmp ult i32 %_24.i2120, %_128.1.i2121, !dbg !21410
  br i1 %_28.i2122, label %bb9.i2124, label %panic5.i2123, !dbg !21410

panic.i2110:                                      ; preds = %bb33.i2105
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_126.1.i2108, i32 noundef %_126.1.i2108, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_12856b5f9033764dbf23e04eb77c5c46) #32, !dbg !21405, !noalias !21406
  unreachable, !dbg !21405

bb9.i2124:                                        ; preds = %bb2.i2111
  %469 = getelementptr inbounds nuw float, ptr %_128.0.i2125, i32 %_24.i2120, !dbg !21410
  %470 = load float, ptr %469, align 4, !dbg !21410, !noalias !21406, !noundef !10
  %exitcond12522.not = icmp eq i32 %iter.sroa.4.0.i21008223, %_130.1.i2126, !dbg !21411
  br i1 %exitcond12522.not, label %panic6.i2128, label %bb10.i2129, !dbg !21411

panic5.i2123:                                     ; preds = %bb2.i2111
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_24.i2120, i32 noundef %_128.1.i2121, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70ebf0cf1c90c6161926611ccf249a32) #32, !dbg !21410, !noalias !21406
  unreachable, !dbg !21410

bb10.i2129:                                       ; preds = %bb9.i2124
  %471 = getelementptr inbounds nuw i32, ptr %_130.0.i2130, i32 %iter.sroa.4.0.i21008223, !dbg !21411
  %_30.i2131 = load i32, ptr %471, align 4, !dbg !21411, !noalias !21406, !noundef !10
  %472 = icmp eq i32 %_30.i2131, 0, !dbg !21412
  br i1 %472, label %bb14.i2140, label %bb12.i2132, !dbg !21412

panic6.i2128:                                     ; preds = %bb9.i2124
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_130.1.i2126, i32 noundef %_130.1.i2126, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_023d591aaad7f5cf482ac80a9401eca1) #32, !dbg !21411, !noalias !21406
  unreachable, !dbg !21411

bb12.i2132:                                       ; preds = %bb10.i2129
  %_35.i2134 = icmp ult i32 %iter.sroa.4.0.i21008223, %_132.1.i2133, !dbg !21413
  br i1 %_35.i2134, label %bb13.i2136, label %panic7.i2135, !dbg !21413

bb14.i2140:                                       ; preds = %bb34.i2201, %bb13.i2136, %bb10.i2129
  %newest.sroa.0.0.i2141 = phi float [ %470, %bb10.i2129 ], [ %_33.i2138, %bb34.i2201 ], [ %470, %bb13.i2136 ], !dbg !21414
  %exitcond12523.not = icmp eq i32 %iter.sroa.4.0.i21008223, %_132.1.i2133, !dbg !21415
  br i1 %exitcond12523.not, label %panic8.i2144, label %bb15.i2145, !dbg !21415

bb13.i2136:                                       ; preds = %bb12.i2132
  %473 = getelementptr inbounds nuw float, ptr %_132.0.i2137, i32 %iter.sroa.4.0.i21008223, !dbg !21413
  %_33.i2138 = load float, ptr %473, align 4, !dbg !21413, !noalias !21406, !noundef !10
  %_116.i2139 = fcmp olt float %_33.i2138, %470, !dbg !21416
  br i1 %_116.i2139, label %bb34.i2201, label %bb14.i2140, !dbg !21416

panic7.i2135:                                     ; preds = %bb12.i2132
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %iter.sroa.4.0.i21008223, i32 noundef %_132.1.i2133, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a228cac3279aae3a34886f59f51fbe9e) #32, !dbg !21413, !noalias !21406
  unreachable, !dbg !21413

bb34.i2201:                                       ; preds = %bb13.i2136
  br label %bb14.i2140, !dbg !21418

bb15.i2145:                                       ; preds = %bb14.i2140
  %474 = getelementptr inbounds nuw float, ptr %_132.0.i2137, i32 %iter.sroa.4.0.i21008223, !dbg !21415
  store float %newest.sroa.0.0.i2141, ptr %474, align 4, !dbg !21415, !noalias !21406
  %_40.i2147 = add i32 %_30.i2131, 1, !dbg !21419
  %complete.i2148 = icmp eq i32 %_40.i2147, %shape.i2113, !dbg !21419
  br i1 %complete.i2148, label %bb19.i2170, label %bb17.i2149, !dbg !21420

panic8.i2144:                                     ; preds = %bb14.i2140
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_132.1.i2133, i32 noundef %_132.1.i2133, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1fa5d88c1a2cc292a2767c48606a38fe) #32, !dbg !21415, !noalias !21406
  unreachable, !dbg !21415

bb17.i2149:                                       ; preds = %bb15.i2145
  %_42.i2151 = add i32 %iter.sroa.4.0.i21008223, %_43.i2150, !dbg !21421
  %_45.i2153 = icmp ult i32 %_42.i2151, %_128.1.i2121, !dbg !21422
  br i1 %_45.i2153, label %bb27.i2163, label %panic9.i2154, !dbg !21422

panic9.i2154:                                     ; preds = %bb17.i2149
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_42.i2151, i32 noundef %_128.1.i2121, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70d40ae2302f3ea19fa0c4a65fe82786) #32, !dbg !21422, !noalias !21406
  unreachable, !dbg !21422

bb27.i2163:                                       ; preds = %bb17.i2149
  %475 = getelementptr inbounds nuw float, ptr %_128.0.i2125, i32 %_42.i2151, !dbg !21422
  %_41.i2157 = load float, ptr %475, align 4, !dbg !21422, !noalias !21406, !noundef !10
  %_117.i2158 = fcmp olt float %_41.i2157, %newest.sroa.0.0.i2141, !dbg !21423
  %newest.sroa.0.1.i2159 = select i1 %_117.i2158, float %_41.i2157, float %newest.sroa.0.0.i2141, !dbg !21423
  store float %newest.sroa.0.1.i2159, ptr %iter.sroa.0.0.ptr.i21028225, align 4, !dbg !21425, !alias.scope !21389, !noalias !21426
  br label %bb28.i2165, !dbg !21427

bb28.i2165:                                       ; preds = %bb22.i2198, %bb19.i2170, %bb27.i2163
  %storemerge5365 = phi i32 [ %_40.i2147, %bb27.i2163 ], [ 0, %bb19.i2170 ], [ 0, %bb22.i2198 ], !dbg !21428
  store i32 %storemerge5365, ptr %471, align 4, !dbg !21428, !noalias !21406
  %476 = icmp eq i32 %464, 0, !dbg !21394
  br i1 %476, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2203.loopexit, label %bb29.i2103, !dbg !21394

bb19.i2170:                                       ; preds = %bb15.i2145
  store float %newest.sroa.0.0.i2141, ptr %iter.sroa.0.0.ptr.i21028225, align 4, !dbg !21425, !alias.scope !21389, !noalias !21426
  %_118.i21768218.not = icmp eq i32 %shape.i2113, 0, !dbg !21429
  br i1 %_118.i21768218.not, label %bb28.i2165, label %bb40.i2183.preheader, !dbg !21433

bb40.i2183.preheader:                             ; preds = %bb19.i2170
  %477 = load float, ptr %469, align 4, !dbg !21434, !noalias !21406, !noundef !10
  br label %bb40.i2183, !dbg !21435

bb40.i2183:                                       ; preds = %bb40.i2183.preheader, %bb22.i2198
  %iter2.sroa.0.0.i21758221 = phi i32 [ %_119.i2184, %bb22.i2198 ], [ 0, %bb40.i2183.preheader ]
  %suffix.sroa.0.0.i21748220 = phi float [ %suffix.sroa.0.1.i2194, %bb22.i2198 ], [ %477, %bb40.i2183.preheader ]
  %end.sroa.0.1.i21738219 = phi i32 [ %480, %bb22.i2198 ], [ %spec.select.i2116, %bb40.i2183.preheader ]
  %_54.i2185 = mul i32 %end.sroa.0.1.i21738219, %width.i2097, !dbg !21436
  %_53.i2186 = add i32 %_54.i2185, %iter.sroa.4.0.i21008223, !dbg !21436
  %_57.i2188 = icmp ult i32 %_53.i2186, %_128.1.i2121, !dbg !21435
  br i1 %_57.i2188, label %bb22.i2198, label %panic13.i2189, !dbg !21435

panic13.i2189:                                    ; preds = %bb40.i2183
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_53.i2186, i32 noundef %_128.1.i2121, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a271bbb7fd83c3605ea58a06b7065fa4) #32, !dbg !21435, !noalias !21406
  unreachable, !dbg !21435

bb22.i2198:                                       ; preds = %bb40.i2183
  %_119.i2184 = add nuw i32 %iter2.sroa.0.0.i21758221, 1, !dbg !21437
  %478 = getelementptr inbounds nuw float, ptr %_128.0.i2125, i32 %_53.i2186, !dbg !21435
  %_52.i2192 = load float, ptr %478, align 4, !dbg !21435, !noalias !21406, !noundef !10
  %_121.i2193 = fcmp olt float %suffix.sroa.0.0.i21748220, %_52.i2192, !dbg !21440
  %suffix.sroa.0.1.i2194 = select i1 %_121.i2193, float %suffix.sroa.0.0.i21748220, float %_52.i2192, !dbg !21440
  store float %suffix.sroa.0.1.i2194, ptr %478, align 4, !dbg !21442, !noalias !21406
  %479 = icmp eq i32 %end.sroa.0.1.i21738219, 0, !dbg !21443
  %spec.store.select.i2200 = select i1 %479, i32 %_91.i, i32 %end.sroa.0.1.i21738219, !dbg !21443
  %480 = add i32 %spec.store.select.i2200, -1, !dbg !21444
  %exitcond12520.not = icmp eq i32 %_119.i2184, %shape.i2113, !dbg !21429
  br i1 %exitcond12520.not, label %bb28.i2165, label %bb40.i2183, !dbg !21433

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2203.loopexit: ; preds = %bb28.i2165, %bb29.i2103
  %_0.i3480.pre = load float, ptr %scratch.i, align 4, !dbg !21445, !alias.scope !21447, !noalias !21450
  br label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2203, !dbg !21445

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2203: ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2203.loopexit, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3637
  %_0.i3480 = phi float [ %_0.i3480.pre, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2203.loopexit ], [ %_0.i3492, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3637 ], !dbg !21445
  %_0.i3035 = fmul float %_0.i3480, 1.638400e+04, !dbg !21451
  %481 = tail call noundef float @llvm.floor.f32(float %_0.i3035), !dbg !21453
  %_0.i3034 = fmul float %481, 0x3F10000000000000, !dbg !21457
  %482 = icmp eq i32 %width.i.i, 0, !dbg !21459
  %_163.1.i.i.pre = load i32, ptr %373, align 4, !dbg !21461, !alias.scope !21360, !noalias !21361
  br i1 %482, label %bb53.i.i, label %bb36.i.i.lr.ph, !dbg !21459

bb36.i.i.lr.ph:                                   ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2203
  %_159.1.i.i = load i32, ptr %367, align 4, !alias.scope !21360, !noalias !21361, !noundef !10
  %_159.0.i.i = load ptr, ptr %368, align 4, !nonnull !10
  %_161.0.i.i = load ptr, ptr %374, align 4, !nonnull !10
  %exitcond12524.not = icmp eq i32 %_159.1.i.i, 0, !dbg !21462
  br i1 %exitcond12524.not, label %panic.i.i, label %bb14.i.i, !dbg !21462

bb34.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3485
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i966, i32 noundef %_158.1.i.i, i32 noundef %_158.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_db7c42041d7aaaba4839906f37294547) #32, !dbg !21463, !noalias !21384
  unreachable, !dbg !21463

bb53.i.i:                                         ; preds = %bb18.i.i.7, %bb18.i.i, %bb18.i.i.1, %bb18.i.i.2, %bb18.i.i.3, %bb18.i.i.4, %bb18.i.i.5, %bb18.i.i.6, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2203
  %_0.i3478 = phi float [ %_0.i3480, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2203 ], [ %_47.i.i, %bb18.i.i ], [ %_47.i.i, %bb18.i.i.7 ], [ %_47.i.i, %bb18.i.i.6 ], [ %_47.i.i, %bb18.i.i.5 ], [ %_47.i.i, %bb18.i.i.4 ], [ %_47.i.i, %bb18.i.i.3 ], [ %_47.i.i, %bb18.i.i.2 ], [ %_47.i.i, %bb18.i.i.1 ], !dbg !21464
  %_0.i2605 = fadd float %_0.i3034, %_0.i33588375, !dbg !21466
  %_0.i3358 = fsub float %_0.i2605, %_0.i3478, !dbg !21468
  %_123.i.i = icmp ugt i32 %_22.i.i966, %_163.1.i.i.pre, !dbg !21470
  br i1 %_123.i.i, label %bb41.i.i, label %bb42.i.i, !dbg !21470, !prof !787

bb14.i.i:                                         ; preds = %bb36.i.i.lr.ph
  %483 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 8, !dbg !21462
  %_42.i.i = load i32, ptr %483, align 4, !dbg !21462, !noalias !21450, !noundef !10
  %484 = add i32 %_42.i.i, %ring_cursor.sroa.0.1.i9498232, !dbg !21473
  %_45.not.i.i = icmp ult i32 %484, %_91.i, !dbg !21474
  %485 = select i1 %_45.not.i.i, i32 0, i32 %_91.i, !dbg !21474
  %spec.select.i.i = sub nuw i32 %484, %485, !dbg !21474
  %_49.i.i972 = mul i32 %spec.select.i.i, %width.i.i, !dbg !21475
  %_51.i.i = icmp ult i32 %_49.i.i972, %_163.1.i.i.pre, !dbg !21476
  br i1 %_51.i.i, label %bb18.i.i, label %panic1.i.i, !dbg !21476

panic.i.i:                                        ; preds = %bb36.i.i.7, %bb36.i.i.6, %bb36.i.i.5, %bb36.i.i.4, %bb36.i.i.3, %bb36.i.i.2, %bb36.i.i.1, %bb36.i.i.lr.ph
  %_159.1.i.i.lcssa.ph = phi i32 [ 7, %bb36.i.i.7 ], [ 6, %bb36.i.i.6 ], [ 5, %bb36.i.i.5 ], [ 4, %bb36.i.i.4 ], [ 3, %bb36.i.i.3 ], [ 2, %bb36.i.i.2 ], [ 1, %bb36.i.i.1 ], [ 0, %bb36.i.i.lr.ph ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_159.1.i.i.lcssa.ph, i32 noundef %_159.1.i.i.lcssa.ph, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_33746731a929bad63709ddedbca82f5f) #32, !dbg !21462, !noalias !21450
  unreachable, !dbg !21462

bb18.i.i:                                         ; preds = %bb14.i.i
  %486 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_49.i.i972, !dbg !21476
  %_47.i.i = load float, ptr %486, align 4, !dbg !21476, !noalias !21450, !noundef !10
  store float %_47.i.i, ptr %scratch.i, align 4, !dbg !21477, !alias.scope !21357, !noalias !21478
  %487 = icmp eq i32 %width.i.i, 1, !dbg !21459
  br i1 %487, label %bb53.i.i, label %bb36.i.i.1, !dbg !21459

bb36.i.i.1:                                       ; preds = %bb18.i.i
  %exitcond12524.1.not = icmp eq i32 %_159.1.i.i, 1, !dbg !21462
  br i1 %exitcond12524.1.not, label %panic.i.i, label %bb14.i.i.1, !dbg !21462

bb14.i.i.1:                                       ; preds = %bb36.i.i.1
  %488 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 20, !dbg !21462
  %_42.i.i.1 = load i32, ptr %488, align 4, !dbg !21462, !noalias !21450, !noundef !10
  %489 = add i32 %_42.i.i.1, %ring_cursor.sroa.0.1.i9498232, !dbg !21473
  %_45.not.i.i.1 = icmp ult i32 %489, %_91.i, !dbg !21474
  %490 = select i1 %_45.not.i.i.1, i32 0, i32 %_91.i, !dbg !21474
  %spec.select.i.i.1 = sub nuw i32 %489, %490, !dbg !21474
  %_49.i.i972.1 = mul i32 %spec.select.i.i.1, %width.i.i, !dbg !21475
  %_48.i.i.1 = add i32 %_49.i.i972.1, 1, !dbg !21475
  %_51.i.i.1 = icmp ult i32 %_48.i.i.1, %_163.1.i.i.pre, !dbg !21476
  br i1 %_51.i.i.1, label %bb18.i.i.1, label %panic1.i.i, !dbg !21476

bb18.i.i.1:                                       ; preds = %bb14.i.i.1
  %491 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.1, !dbg !21476
  %_47.i.i.1 = load float, ptr %491, align 4, !dbg !21476, !noalias !21450, !noundef !10
  store float %_47.i.i.1, ptr %iter.sroa.0.0.ptr.i.i8229.1, align 4, !dbg !21477, !alias.scope !21357, !noalias !21478
  %492 = icmp eq i32 %width.i.i, 2, !dbg !21459
  br i1 %492, label %bb53.i.i, label %bb36.i.i.2, !dbg !21459

bb36.i.i.2:                                       ; preds = %bb18.i.i.1
  %exitcond12524.2.not = icmp eq i32 %_159.1.i.i, 2, !dbg !21462
  br i1 %exitcond12524.2.not, label %panic.i.i, label %bb14.i.i.2, !dbg !21462

bb14.i.i.2:                                       ; preds = %bb36.i.i.2
  %493 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 32, !dbg !21462
  %_42.i.i.2 = load i32, ptr %493, align 4, !dbg !21462, !noalias !21450, !noundef !10
  %494 = add i32 %_42.i.i.2, %ring_cursor.sroa.0.1.i9498232, !dbg !21473
  %_45.not.i.i.2 = icmp ult i32 %494, %_91.i, !dbg !21474
  %495 = select i1 %_45.not.i.i.2, i32 0, i32 %_91.i, !dbg !21474
  %spec.select.i.i.2 = sub nuw i32 %494, %495, !dbg !21474
  %_49.i.i972.2 = mul i32 %spec.select.i.i.2, %width.i.i, !dbg !21475
  %_48.i.i.2 = add i32 %_49.i.i972.2, 2, !dbg !21475
  %_51.i.i.2 = icmp ult i32 %_48.i.i.2, %_163.1.i.i.pre, !dbg !21476
  br i1 %_51.i.i.2, label %bb18.i.i.2, label %panic1.i.i, !dbg !21476

bb18.i.i.2:                                       ; preds = %bb14.i.i.2
  %496 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.2, !dbg !21476
  %_47.i.i.2 = load float, ptr %496, align 4, !dbg !21476, !noalias !21450, !noundef !10
  store float %_47.i.i.2, ptr %iter.sroa.0.0.ptr.i.i8229.2, align 4, !dbg !21477, !alias.scope !21357, !noalias !21478
  %497 = icmp eq i32 %width.i.i, 3, !dbg !21459
  br i1 %497, label %bb53.i.i, label %bb36.i.i.3, !dbg !21459

bb36.i.i.3:                                       ; preds = %bb18.i.i.2
  %exitcond12524.3.not = icmp eq i32 %_159.1.i.i, 3, !dbg !21462
  br i1 %exitcond12524.3.not, label %panic.i.i, label %bb14.i.i.3, !dbg !21462

bb14.i.i.3:                                       ; preds = %bb36.i.i.3
  %498 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 44, !dbg !21462
  %_42.i.i.3 = load i32, ptr %498, align 4, !dbg !21462, !noalias !21450, !noundef !10
  %499 = add i32 %_42.i.i.3, %ring_cursor.sroa.0.1.i9498232, !dbg !21473
  %_45.not.i.i.3 = icmp ult i32 %499, %_91.i, !dbg !21474
  %500 = select i1 %_45.not.i.i.3, i32 0, i32 %_91.i, !dbg !21474
  %spec.select.i.i.3 = sub nuw i32 %499, %500, !dbg !21474
  %_49.i.i972.3 = mul i32 %spec.select.i.i.3, %width.i.i, !dbg !21475
  %_48.i.i.3 = add i32 %_49.i.i972.3, 3, !dbg !21475
  %_51.i.i.3 = icmp ult i32 %_48.i.i.3, %_163.1.i.i.pre, !dbg !21476
  br i1 %_51.i.i.3, label %bb18.i.i.3, label %panic1.i.i, !dbg !21476

bb18.i.i.3:                                       ; preds = %bb14.i.i.3
  %501 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.3, !dbg !21476
  %_47.i.i.3 = load float, ptr %501, align 4, !dbg !21476, !noalias !21450, !noundef !10
  store float %_47.i.i.3, ptr %iter.sroa.0.0.ptr.i.i8229.3, align 4, !dbg !21477, !alias.scope !21357, !noalias !21478
  %502 = icmp eq i32 %width.i.i, 4, !dbg !21459
  br i1 %502, label %bb53.i.i, label %bb36.i.i.4, !dbg !21459

bb36.i.i.4:                                       ; preds = %bb18.i.i.3
  %exitcond12524.4.not = icmp eq i32 %_159.1.i.i, 4, !dbg !21462
  br i1 %exitcond12524.4.not, label %panic.i.i, label %bb14.i.i.4, !dbg !21462

bb14.i.i.4:                                       ; preds = %bb36.i.i.4
  %503 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 56, !dbg !21462
  %_42.i.i.4 = load i32, ptr %503, align 4, !dbg !21462, !noalias !21450, !noundef !10
  %504 = add i32 %_42.i.i.4, %ring_cursor.sroa.0.1.i9498232, !dbg !21473
  %_45.not.i.i.4 = icmp ult i32 %504, %_91.i, !dbg !21474
  %505 = select i1 %_45.not.i.i.4, i32 0, i32 %_91.i, !dbg !21474
  %spec.select.i.i.4 = sub nuw i32 %504, %505, !dbg !21474
  %_49.i.i972.4 = mul i32 %spec.select.i.i.4, %width.i.i, !dbg !21475
  %_48.i.i.4 = add i32 %_49.i.i972.4, 4, !dbg !21475
  %_51.i.i.4 = icmp ult i32 %_48.i.i.4, %_163.1.i.i.pre, !dbg !21476
  br i1 %_51.i.i.4, label %bb18.i.i.4, label %panic1.i.i, !dbg !21476

bb18.i.i.4:                                       ; preds = %bb14.i.i.4
  %506 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.4, !dbg !21476
  %_47.i.i.4 = load float, ptr %506, align 4, !dbg !21476, !noalias !21450, !noundef !10
  store float %_47.i.i.4, ptr %iter.sroa.0.0.ptr.i.i8229.4, align 4, !dbg !21477, !alias.scope !21357, !noalias !21478
  %507 = icmp eq i32 %width.i.i, 5, !dbg !21459
  br i1 %507, label %bb53.i.i, label %bb36.i.i.5, !dbg !21459

bb36.i.i.5:                                       ; preds = %bb18.i.i.4
  %exitcond12524.5.not = icmp eq i32 %_159.1.i.i, 5, !dbg !21462
  br i1 %exitcond12524.5.not, label %panic.i.i, label %bb14.i.i.5, !dbg !21462

bb14.i.i.5:                                       ; preds = %bb36.i.i.5
  %508 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 68, !dbg !21462
  %_42.i.i.5 = load i32, ptr %508, align 4, !dbg !21462, !noalias !21450, !noundef !10
  %509 = add i32 %_42.i.i.5, %ring_cursor.sroa.0.1.i9498232, !dbg !21473
  %_45.not.i.i.5 = icmp ult i32 %509, %_91.i, !dbg !21474
  %510 = select i1 %_45.not.i.i.5, i32 0, i32 %_91.i, !dbg !21474
  %spec.select.i.i.5 = sub nuw i32 %509, %510, !dbg !21474
  %_49.i.i972.5 = mul i32 %spec.select.i.i.5, %width.i.i, !dbg !21475
  %_48.i.i.5 = add i32 %_49.i.i972.5, 5, !dbg !21475
  %_51.i.i.5 = icmp ult i32 %_48.i.i.5, %_163.1.i.i.pre, !dbg !21476
  br i1 %_51.i.i.5, label %bb18.i.i.5, label %panic1.i.i, !dbg !21476

bb18.i.i.5:                                       ; preds = %bb14.i.i.5
  %511 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.5, !dbg !21476
  %_47.i.i.5 = load float, ptr %511, align 4, !dbg !21476, !noalias !21450, !noundef !10
  store float %_47.i.i.5, ptr %iter.sroa.0.0.ptr.i.i8229.5, align 4, !dbg !21477, !alias.scope !21357, !noalias !21478
  %512 = icmp eq i32 %width.i.i, 6, !dbg !21459
  br i1 %512, label %bb53.i.i, label %bb36.i.i.6, !dbg !21459

bb36.i.i.6:                                       ; preds = %bb18.i.i.5
  %exitcond12524.6.not = icmp eq i32 %_159.1.i.i, 6, !dbg !21462
  br i1 %exitcond12524.6.not, label %panic.i.i, label %bb14.i.i.6, !dbg !21462

bb14.i.i.6:                                       ; preds = %bb36.i.i.6
  %513 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 80, !dbg !21462
  %_42.i.i.6 = load i32, ptr %513, align 4, !dbg !21462, !noalias !21450, !noundef !10
  %514 = add i32 %_42.i.i.6, %ring_cursor.sroa.0.1.i9498232, !dbg !21473
  %_45.not.i.i.6 = icmp ult i32 %514, %_91.i, !dbg !21474
  %515 = select i1 %_45.not.i.i.6, i32 0, i32 %_91.i, !dbg !21474
  %spec.select.i.i.6 = sub nuw i32 %514, %515, !dbg !21474
  %_49.i.i972.6 = mul i32 %spec.select.i.i.6, %width.i.i, !dbg !21475
  %_48.i.i.6 = add i32 %_49.i.i972.6, 6, !dbg !21475
  %_51.i.i.6 = icmp ult i32 %_48.i.i.6, %_163.1.i.i.pre, !dbg !21476
  br i1 %_51.i.i.6, label %bb18.i.i.6, label %panic1.i.i, !dbg !21476

bb18.i.i.6:                                       ; preds = %bb14.i.i.6
  %516 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.6, !dbg !21476
  %_47.i.i.6 = load float, ptr %516, align 4, !dbg !21476, !noalias !21450, !noundef !10
  store float %_47.i.i.6, ptr %iter.sroa.0.0.ptr.i.i8229.6, align 4, !dbg !21477, !alias.scope !21357, !noalias !21478
  %517 = icmp eq i32 %width.i.i, 7, !dbg !21459
  br i1 %517, label %bb53.i.i, label %bb36.i.i.7, !dbg !21459

bb36.i.i.7:                                       ; preds = %bb18.i.i.6
  %exitcond12524.7.not = icmp eq i32 %_159.1.i.i, 7, !dbg !21462
  br i1 %exitcond12524.7.not, label %panic.i.i, label %bb14.i.i.7, !dbg !21462

bb14.i.i.7:                                       ; preds = %bb36.i.i.7
  %518 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 92, !dbg !21462
  %_42.i.i.7 = load i32, ptr %518, align 4, !dbg !21462, !noalias !21450, !noundef !10
  %519 = add i32 %_42.i.i.7, %ring_cursor.sroa.0.1.i9498232, !dbg !21473
  %_45.not.i.i.7 = icmp ult i32 %519, %_91.i, !dbg !21474
  %520 = select i1 %_45.not.i.i.7, i32 0, i32 %_91.i, !dbg !21474
  %spec.select.i.i.7 = sub nuw i32 %519, %520, !dbg !21474
  %_49.i.i972.7 = mul i32 %spec.select.i.i.7, %width.i.i, !dbg !21475
  %_48.i.i.7 = add i32 %_49.i.i972.7, 7, !dbg !21475
  %_51.i.i.7 = icmp ult i32 %_48.i.i.7, %_163.1.i.i.pre, !dbg !21476
  br i1 %_51.i.i.7, label %bb18.i.i.7, label %panic1.i.i, !dbg !21476

bb18.i.i.7:                                       ; preds = %bb14.i.i.7
  %521 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.7, !dbg !21476
  %_47.i.i.7 = load float, ptr %521, align 4, !dbg !21476, !noalias !21450, !noundef !10
  store float %_47.i.i.7, ptr %iter.sroa.0.0.ptr.i.i8229.7, align 4, !dbg !21477, !alias.scope !21357, !noalias !21478
  br label %bb53.i.i, !dbg !21459

panic1.i.i:                                       ; preds = %bb14.i.i.7, %bb14.i.i.6, %bb14.i.i.5, %bb14.i.i.4, %bb14.i.i.3, %bb14.i.i.2, %bb14.i.i.1, %bb14.i.i
  %_48.i.i.lcssa.ph = phi i32 [ %_48.i.i.7, %bb14.i.i.7 ], [ %_48.i.i.6, %bb14.i.i.6 ], [ %_48.i.i.5, %bb14.i.i.5 ], [ %_48.i.i.4, %bb14.i.i.4 ], [ %_48.i.i.3, %bb14.i.i.3 ], [ %_48.i.i.2, %bb14.i.i.2 ], [ %_48.i.i.1, %bb14.i.i.1 ], [ %_49.i.i972, %bb14.i.i ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_48.i.i.lcssa.ph, i32 noundef %_163.1.i.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_0e76a26fbb751dfb8cf8a069b5b0d39a) #32, !dbg !21476, !noalias !21450
  unreachable, !dbg !21476

bb42.i.i:                                         ; preds = %bb53.i.i
  %_163.0.i.i = load ptr, ptr %374, align 4, !dbg !21461, !alias.scope !21360, !noalias !21361, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21479), !dbg !21482
  %_4.not.i3630 = icmp eq i32 %_163.1.i.i.pre, %_22.i.i966, !dbg !21483
  br i1 %_4.not.i3630, label %panic.i3632, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3633, !dbg !21483

panic.i3632:                                      ; preds = %bb42.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #32, !dbg !21483, !noalias !21485
  unreachable, !dbg !21483

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3633: ; preds = %bb42.i.i
  %_130.i.i = getelementptr inbounds nuw float, ptr %_163.0.i.i, i32 %_22.i.i966, !dbg !21486
  store float %_0.i3034, ptr %_130.i.i, align 4, !dbg !21483, !alias.scope !21479, !noalias !21450
  %_0.i2903 = fdiv float %_0.i3358, %_62.i.i975, !dbg !21488
  %_0.i3357 = fsub float 1.000000e+00, %_0.i2903, !dbg !21490
  %_0.i3356 = fsub float %_0.i3357, %_0.i37328444, !dbg !21492
  %_4.i2919 = fmul float %_9.i.i955, %_0.i3356, !dbg !21494
  %_0.i2920 = fadd float %_0.i37328444, %_4.i2919, !dbg !21494
  %_3.i.i4109.inv = fcmp ogt float %_0.i3357, %_0.i2920, !dbg !21496
  %_4.i.i4116.v = select i1 %_3.i.i4109.inv, float %_0.i3357, float %_0.i2920, !dbg !21496
  %522 = tail call noundef float @llvm.fabs.f32(float %_4.i.i4116.v), !dbg !21499
  %523 = fcmp uge float %522, 0x3BC79CA100000000, !dbg !21502
  %_0.i3732 = select i1 %523, float %_4.i.i4116.v, float 0.000000e+00, !dbg !21504
  %_0.i3355 = fsub float 1.000000e+00, %_0.i3732, !dbg !21505
  %_164.1.i.i = load i32, ptr %378, align 4, !dbg !21507, !alias.scope !21360, !noalias !21361, !noundef !10
  %_74.i.i = mul i32 %width.i.i, %main_cursor.sroa.0.1.i9508233, !dbg !21508
  %_134.i.i = icmp ugt i32 %_74.i.i, %_164.1.i.i, !dbg !21509
  br i1 %_134.i.i, label %bb47.i.i, label %bb48.i.i, !dbg !21509, !prof !787

bb41.i.i:                                         ; preds = %bb53.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i966, i32 noundef %_163.1.i.i.pre, i32 noundef %_163.1.i.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c2286ff41a33b8c11aa270fe215441de) #32, !dbg !21512, !noalias !21450
  unreachable, !dbg !21512

bb48.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3633
  %_164.0.i.i = load ptr, ptr %379, align 4, !dbg !21507, !alias.scope !21360, !noalias !21361, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21513), !dbg !21516
  %_3.not.i3472 = icmp eq i32 %_164.1.i.i, %_74.i.i, !dbg !21517
  br i1 %_3.not.i3472, label %panic.i3475, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3625, !dbg !21517

panic.i3475:                                      ; preds = %bb48.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #32, !dbg !21517, !noalias !21519
  unreachable, !dbg !21517

bb47.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3633
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_74.i.i, i32 noundef %_164.1.i.i, i32 noundef %_164.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a3a21efd18b403ec25db545d522c479) #32, !dbg !21520, !noalias !21450
  unreachable, !dbg !21520

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3625: ; preds = %bb48.i.i
  %_141.i.i = getelementptr inbounds nuw float, ptr %_164.0.i.i, i32 %_74.i.i, !dbg !21521
  %_0.i3474 = load float, ptr %_141.i.i, align 4, !dbg !21517, !alias.scope !21513, !noalias !21450, !noundef !10
  store float %_0.i3483, ptr %_141.i.i, align 4, !dbg !21523, !alias.scope !21525, !noalias !21450
  %_0.i3033 = fmul float %_0.i3355, %_0.i3474, !dbg !21528
  %_6.i3894 = bitcast float %_0.i3474 to i32, !dbg !21530
  %_5.i3895 = and i32 %_6.i3894, %all.sroa.0.0.i741, !dbg !21533
  %_8.i3896 = bitcast float %_0.i3033 to i32, !dbg !21534
  %_7.i3898 = and i32 %_9.i3910, %_8.i3896, !dbg !21536
  %_4.i3899 = or disjoint i32 %_7.i3898, %_5.i3895, !dbg !21533
  store i32 %_4.i3899, ptr %_178.i, align 4, !dbg !21537, !alias.scope !21539, !noalias !21542
  %524 = add i32 %main_cursor.sroa.0.1.i9508233, 1, !dbg !21543
  %_104.i = icmp eq i32 %524, %_106.i, !dbg !21544
  %spec.store.select11.i = select i1 %_104.i, i32 0, i32 %524, !dbg !21544
  %525 = add i32 %ring_cursor.sroa.0.1.i9498232, 1, !dbg !21545
  %_107.i = icmp eq i32 %525, %_91.i, !dbg !21546
  %spec.store.select12.i = select i1 %_107.i, i32 0, i32 %525, !dbg !21546
  %exitcond12527.not = icmp eq i32 %397, %umax12526, !dbg !21547
  br i1 %exitcond12527.not, label %bb16.i.bb13.i.loopexit_crit_edge, label %bb50.i951, !dbg !20529

bb56.i981:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3641
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_64.i, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_7e50e0c93e87b60da469fc303486c501) #32, !dbg !21550, !noalias !20557
  unreachable, !dbg !21550

_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit: ; preds = %bb13.i.loopexit, %bb11.i
  %ring_cursor.sroa.0.0.i748.lcssa = phi i32 [ %_37.i742, %bb11.i ], [ %ring_cursor.sroa.0.1.i949.lcssa, %bb13.i.loopexit ], !dbg !20481
  %main_cursor.sroa.0.0.i749.lcssa = phi i32 [ %_35.i, %bb11.i ], [ %main_cursor.sroa.0.1.i950.lcssa, %bb13.i.loopexit ], !dbg !20478
  call void @llvm.lifetime.start.p0(ptr nonnull %_110.i), !dbg !21551, !noalias !20461
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(92) %_110.i, ptr noundef nonnull align 4 dereferenceable(92) %hot_left.i734, i32 92, i1 false), !dbg !21551, !noalias !20461
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_110.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33) #31, !dbg !21552, !noalias !20557
  call void @llvm.lifetime.end.p0(ptr nonnull %_110.i), !dbg !21553, !noalias !20461
  call void @llvm.lifetime.start.p0(ptr nonnull %_112.i), !dbg !21554, !noalias !20461
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(92) %_112.i, ptr noundef nonnull align 4 dereferenceable(92) %hot_right.i733, i32 92, i1 false), !dbg !21554, !noalias !20461
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_112.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34) #31, !dbg !21555, !noalias !20557
  call void @llvm.lifetime.end.p0(ptr nonnull %_112.i), !dbg !21556, !noalias !20461
  store i32 %main_cursor.sroa.0.0.i749.lcssa, ptr %_35, align 4, !dbg !21557, !alias.scope !20455, !noalias !20480
  store i32 %ring_cursor.sroa.0.0.i748.lcssa, ptr %81, align 4, !dbg !21558, !alias.scope !20455, !noalias !20480
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i731), !dbg !21559, !noalias !20461
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i732), !dbg !21560, !noalias !20461
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i), !dbg !21561, !noalias !20461
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_right.i733), !dbg !21562, !noalias !20461
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i734), !dbg !21563, !noalias !20461
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockfEB2_.exit, !dbg !20450

bb7.i:                                            ; preds = %bb5.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21564), !dbg !21567
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21568), !dbg !21567
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21570), !dbg !21567
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21572), !dbg !21567
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21574), !dbg !21567
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i55), !dbg !21576, !noalias !21580
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_left.i55, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #31, !dbg !21583, !noalias !21584
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_right.i54), !dbg !21585, !noalias !21580
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_right.i54, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #31, !dbg !21587, !noalias !21588
  %526 = getelementptr inbounds nuw i8, ptr %self, i32 320, !dbg !21589
  %527 = load i8, ptr %526, align 4, !dbg !21589, !range !4667, !alias.scope !21564, !noalias !21593, !noundef !10
  %528 = getelementptr inbounds nuw i8, ptr %self, i32 321, !dbg !21594
  %529 = load i8, ptr %528, align 1, !dbg !21594, !range !4667, !alias.scope !21564, !noalias !21593, !noundef !10
  %530 = getelementptr inbounds nuw i8, ptr %self, i32 528, !dbg !21596
  %ring.i63 = load i32, ptr %530, align 4, !dbg !21596, !alias.scope !21568, !noalias !21598, !noundef !10
  %531 = getelementptr inbounds nuw i8, ptr %self, i32 532, !dbg !21599
  %main.i64 = load i32, ptr %531, align 4, !dbg !21599, !alias.scope !21568, !noalias !21598, !noundef !10
  %_36.i65 = load i32, ptr %_35, align 4, !dbg !21601, !alias.scope !21574, !noalias !21603, !noundef !10
  %532 = getelementptr inbounds nuw i8, ptr %self, i32 108, !dbg !21604
  %_37.i66 = load i32, ptr %532, align 4, !dbg !21604, !alias.scope !21574, !noalias !21603, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i53), !dbg !21606, !noalias !21580
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i53, i8 0, i32 1024, i1 false), !noalias !21580
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i52), !dbg !21608, !noalias !21580
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i52, i8 0, i32 1024, i1 false), !noalias !21580
  %_32.i59 = zext nneg i8 %527 to i32, !dbg !21589
  %.none.i60 = sub nsw i32 0, %_32.i59, !dbg !21610
  %_33.i61 = zext nneg i8 %529 to i32, !dbg !21594
  %all.sroa.0.0.i62 = sub nsw i32 0, %_33.i61, !dbg !21594
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i51), !dbg !21611, !noalias !21580
; call <true_peak_limiter::UniformHot<f32>>::new
  call fastcc void @_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotfE3newB5_(ptr noalias noundef align 4 captures(none) dereferenceable(44) %uniform_left.i51, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33, i32 %ring.i63, i32 %main.i64) #31, !dbg !21613
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_right.i50), !dbg !21614, !noalias !21580
  %_32.val = load i32, ptr %530, align 4, !dbg !21616, !noundef !10
  %_32.val4353 = load i32, ptr %531, align 4, !dbg !21616, !noundef !10
; call <true_peak_limiter::UniformHot<f32>>::new
  call fastcc void @_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotfE3newB5_(ptr noalias noundef align 4 captures(none) dereferenceable(44) %uniform_right.i50, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34, i32 %_32.val, i32 %_32.val4353) #31, !dbg !21616
  %_166.not.i779310 = icmp eq i32 %frames, 0, !dbg !21617
  br i1 %_166.not.i779310, label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit, label %bb44.i78.lr.ph, !dbg !21617

bb44.i78.lr.ph:                                   ; preds = %bb7.i
  %d9.i4662 = lshr i32 %frames, 5, !dbg !21627
  %r2.i4663 = and i32 %frames, 31, !dbg !21634
  %_19.not.i4664 = icmp ne i32 %r2.i4663, 0, !dbg !21635
  %533 = zext i1 %_19.not.i4664 to i32, !dbg !21635
  %yield_count.sroa.0.0.i4665 = add nuw nsw i32 %d9.i4662, %533, !dbg !21635
  %history.i44.i18.sroa.7.0.hot_left.i55.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i55, i32 4
  %history.i44.i18.sroa.10.0.hot_left.i55.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i55, i32 8
  %history.i44.i18.sroa.13.0.hot_left.i55.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i55, i32 12
  %history.i44.i18.sroa.16.0.hot_left.i55.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i55, i32 16
  %history.i44.i18.sroa.19.0.hot_left.i55.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i55, i32 20
  %history.i44.i18.sroa.22.0.hot_left.i55.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i55, i32 24
  %history.i44.i18.sroa.26.0.hot_left.i55.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i55, i32 28
  %history.i44.i18.sroa.29.0.hot_left.i55.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i55, i32 32
  %history.i44.i18.sroa.32.0.hot_left.i55.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i55, i32 36
  %history.i44.i18.sroa.35.0.hot_left.i55.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i55, i32 40
  %history.i44.i18.sroa.38.0.hot_left.i55.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i55, i32 44
  %534 = getelementptr inbounds nuw i8, ptr %self, i32 132
  %535 = getelementptr inbounds nuw i8, ptr %self, i32 136
  %536 = getelementptr inbounds nuw i8, ptr %self, i32 140
  %row1.i.i.i77.i118 = getelementptr inbounds nuw i8, ptr %self, i32 144
  %537 = getelementptr inbounds nuw i8, ptr %self, i32 148
  %538 = getelementptr inbounds nuw i8, ptr %self, i32 152
  %539 = getelementptr inbounds nuw i8, ptr %self, i32 156
  %row3.i.i.i91.i132 = getelementptr inbounds nuw i8, ptr %self, i32 160
  %540 = getelementptr inbounds nuw i8, ptr %self, i32 164
  %541 = getelementptr inbounds nuw i8, ptr %self, i32 168
  %542 = getelementptr inbounds nuw i8, ptr %self, i32 172
  %row5.i.i.i105.i146 = getelementptr inbounds nuw i8, ptr %self, i32 176
  %543 = getelementptr inbounds nuw i8, ptr %self, i32 180
  %544 = getelementptr inbounds nuw i8, ptr %self, i32 184
  %545 = getelementptr inbounds nuw i8, ptr %self, i32 188
  %row7.i.i.i119.i160 = getelementptr inbounds nuw i8, ptr %self, i32 192
  %546 = getelementptr inbounds nuw i8, ptr %self, i32 196
  %547 = getelementptr inbounds nuw i8, ptr %self, i32 200
  %548 = getelementptr inbounds nuw i8, ptr %self, i32 204
  %row9.i.i.i133.i174 = getelementptr inbounds nuw i8, ptr %self, i32 208
  %549 = getelementptr inbounds nuw i8, ptr %self, i32 212
  %550 = getelementptr inbounds nuw i8, ptr %self, i32 216
  %551 = getelementptr inbounds nuw i8, ptr %self, i32 220
  %row11.i.i.i147.i188 = getelementptr inbounds nuw i8, ptr %self, i32 224
  %552 = getelementptr inbounds nuw i8, ptr %self, i32 228
  %553 = getelementptr inbounds nuw i8, ptr %self, i32 232
  %554 = getelementptr inbounds nuw i8, ptr %self, i32 236
  %row13.i.i.i161.i202 = getelementptr inbounds nuw i8, ptr %self, i32 240
  %555 = getelementptr inbounds nuw i8, ptr %self, i32 244
  %556 = getelementptr inbounds nuw i8, ptr %self, i32 248
  %557 = getelementptr inbounds nuw i8, ptr %self, i32 252
  %row15.i.i.i175.i216 = getelementptr inbounds nuw i8, ptr %self, i32 256
  %558 = getelementptr inbounds nuw i8, ptr %self, i32 260
  %559 = getelementptr inbounds nuw i8, ptr %self, i32 264
  %560 = getelementptr inbounds nuw i8, ptr %self, i32 268
  %row17.i.i.i189.i230 = getelementptr inbounds nuw i8, ptr %self, i32 272
  %561 = getelementptr inbounds nuw i8, ptr %self, i32 276
  %562 = getelementptr inbounds nuw i8, ptr %self, i32 280
  %563 = getelementptr inbounds nuw i8, ptr %self, i32 284
  %row19.i.i.i203.i244 = getelementptr inbounds nuw i8, ptr %self, i32 288
  %564 = getelementptr inbounds nuw i8, ptr %self, i32 292
  %565 = getelementptr inbounds nuw i8, ptr %self, i32 296
  %566 = getelementptr inbounds nuw i8, ptr %self, i32 300
  %row21.i.i.i217.i258 = getelementptr inbounds nuw i8, ptr %self, i32 304
  %567 = getelementptr inbounds nuw i8, ptr %self, i32 308
  %568 = getelementptr inbounds nuw i8, ptr %self, i32 312
  %569 = getelementptr inbounds nuw i8, ptr %self, i32 316
  %history.i.i32.sroa.7.0.hot_right.i54.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i54, i32 4
  %history.i.i32.sroa.10.0.hot_right.i54.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i54, i32 8
  %history.i.i32.sroa.13.0.hot_right.i54.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i54, i32 12
  %history.i.i32.sroa.16.0.hot_right.i54.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i54, i32 16
  %history.i.i32.sroa.19.0.hot_right.i54.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i54, i32 20
  %history.i.i32.sroa.22.0.hot_right.i54.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i54, i32 24
  %history.i.i32.sroa.26.0.hot_right.i54.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i54, i32 28
  %history.i.i32.sroa.29.0.hot_right.i54.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i54, i32 32
  %history.i.i32.sroa.32.0.hot_right.i54.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i54, i32 36
  %history.i.i32.sroa.35.0.hot_right.i54.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i54, i32 40
  %history.i.i32.sroa.38.0.hot_right.i54.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i54, i32 44
  %570 = getelementptr inbounds nuw i8, ptr %uniform_left.i51, i32 32
  %_72.i48.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i51, i32 36
  %_72.i48.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i51, i32 40
  %571 = getelementptr inbounds nuw i8, ptr %uniform_right.i50, i32 32
  %_73.i47.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i50, i32 36
  %_73.i47.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i50, i32 40
  %_114.i525 = getelementptr inbounds nuw i8, ptr %hot_left.i55, i32 48
  %_115.i526 = getelementptr inbounds nuw i8, ptr %hot_left.i55, i32 64
  %572 = getelementptr inbounds nuw i8, ptr %hot_left.i55, i32 60
  %573 = getelementptr inbounds nuw i8, ptr %hot_left.i55, i32 56
  %574 = getelementptr inbounds nuw i8, ptr %hot_left.i55, i32 52
  %575 = getelementptr inbounds nuw i8, ptr %hot_left.i55, i32 76
  %576 = getelementptr inbounds nuw i8, ptr %hot_left.i55, i32 72
  %577 = getelementptr inbounds nuw i8, ptr %hot_left.i55, i32 68
  %_119.i527 = getelementptr inbounds nuw i8, ptr %hot_right.i54, i32 48
  %_120.i528 = getelementptr inbounds nuw i8, ptr %hot_right.i54, i32 64
  %578 = getelementptr inbounds nuw i8, ptr %hot_right.i54, i32 60
  %579 = getelementptr inbounds nuw i8, ptr %hot_right.i54, i32 56
  %580 = getelementptr inbounds nuw i8, ptr %hot_right.i54, i32 52
  %581 = getelementptr inbounds nuw i8, ptr %hot_right.i54, i32 76
  %582 = getelementptr inbounds nuw i8, ptr %hot_right.i54, i32 72
  %583 = getelementptr inbounds nuw i8, ptr %hot_right.i54, i32 68
  %_9.i3970 = add nsw i32 %_32.i59, -1
  %584 = getelementptr inbounds nuw i8, ptr %uniform_left.i51, i32 4
  %_21.i264.i564 = getelementptr inbounds nuw i8, ptr %uniform_left.i51, i32 24
  %_22.i265.i565 = getelementptr inbounds nuw i8, ptr %uniform_left.i51, i32 28
  %585 = getelementptr inbounds nuw i8, ptr %uniform_left.i51, i32 8
  %586 = getelementptr inbounds nuw i8, ptr %uniform_left.i51, i32 12
  %587 = getelementptr inbounds nuw i8, ptr %hot_left.i55, i32 84
  %588 = getelementptr inbounds nuw i8, ptr %hot_left.i55, i32 88
  %589 = getelementptr inbounds nuw i8, ptr %hot_left.i55, i32 80
  %590 = getelementptr inbounds nuw i8, ptr %uniform_left.i51, i32 20
  %591 = getelementptr inbounds nuw i8, ptr %uniform_left.i51, i32 16
  %_9.i3950 = add nsw i32 %_33.i61, -1
  %592 = getelementptr inbounds nuw i8, ptr %uniform_right.i50, i32 4
  %_21.i.i639 = getelementptr inbounds nuw i8, ptr %uniform_right.i50, i32 24
  %_22.i.i640 = getelementptr inbounds nuw i8, ptr %uniform_right.i50, i32 28
  %593 = getelementptr inbounds nuw i8, ptr %uniform_right.i50, i32 8
  %594 = getelementptr inbounds nuw i8, ptr %uniform_right.i50, i32 12
  %595 = getelementptr inbounds nuw i8, ptr %hot_right.i54, i32 84
  %596 = getelementptr inbounds nuw i8, ptr %hot_right.i54, i32 88
  %597 = getelementptr inbounds nuw i8, ptr %hot_right.i54, i32 80
  %598 = getelementptr inbounds nuw i8, ptr %uniform_right.i50, i32 20
  %599 = getelementptr inbounds nuw i8, ptr %uniform_right.i50, i32 16
  br label %bb44.i78, !dbg !21617

bb19.i479.bb15.i72.loopexit_crit_edge:            ; preds = %bb74.i692
  store float %_0.i.i.lcssa1321114661, ptr %572, align 4
  store float %_0.i3763.lcssa1319714679, ptr %_114.i525, align 4
  store float %_0.i3757.lcssa1318314697, ptr %573, align 4
  store float %_0.i.i4022.lcssa1316914715, ptr %575, align 4
  store float %_0.i3776.lcssa1315514733, ptr %_115.i526, align 4
  store float %_0.i3769.lcssa1314114751, ptr %576, align 4
  store float %_0.i.i4029.lcssa1312714769, ptr %578, align 4
  store float %_0.i3789.lcssa1311314787, ptr %_119.i527, align 4
  store float %_0.i3782.lcssa1309914805, ptr %579, align 4
  store float %_0.i.i4036.lcssa1308514823, ptr %581, align 4
  store float %_0.i3802.lcssa1307114841, ptr %_120.i528, align 4
  store float %_0.i3795.lcssa1305714859, ptr %582, align 4
  store float %_0.i3370.lcssa1324214877, ptr %587, align 4
  store float %_0.i3744.lcssa1325814895, ptr %589, align 4
  store float %_0.i3366.lcssa1328214913, ptr %595, align 4
  store float %_0.i3740.lcssa1328314931, ptr %597, align 4
  store i32 %storemerge.i1694.lcssa89589167, ptr %_22.i265.i565, align 4
  store float %running.sroa.0.0.i1689.lcssa89859203, ptr %_21.i264.i564, align 4
  store i32 %storemerge.i.lcssa90709239, ptr %_22.i.i640, align 4
  store float %running.sroa.0.0.i.lcssa90979275, ptr %_21.i.i639, align 4
  br label %bb15.i72.loopexit, !dbg !21636

bb15.i72.loopexit:                                ; preds = %bb19.i479.bb15.i72.loopexit_crit_edge, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i478
  %ring_cursor.sroa.0.1.i480.lcssa = phi i32 [ %ring_cursor.sroa.0.2.i695, %bb19.i479.bb15.i72.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i739311, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i478 ], !dbg !21642
  %main_cursor.sroa.0.1.i481.lcssa = phi i32 [ %main_cursor.sroa.0.2.i698, %bb19.i479.bb15.i72.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i749312, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i478 ], !dbg !21643
  %_166.not.i77 = icmp eq i32 %601, 0, !dbg !21617
  %indvars.iv.next12529 = add i32 %indvars.iv12528, -32, !dbg !21617
  br i1 %_166.not.i77, label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit, label %bb44.i78, !dbg !21617

bb44.i78:                                         ; preds = %bb44.i78.lr.ph, %bb15.i72.loopexit
  %indvars.iv12528 = phi i32 [ %frames, %bb44.i78.lr.ph ], [ %indvars.iv.next12529, %bb15.i72.loopexit ]
  %iter2.sroa.0.0.i769314 = phi i32 [ %yield_count.sroa.0.0.i4665, %bb44.i78.lr.ph ], [ %601, %bb15.i72.loopexit ]
  %iter1.sroa.0.0.i759313 = phi i32 [ 0, %bb44.i78.lr.ph ], [ %600, %bb15.i72.loopexit ]
  %main_cursor.sroa.0.0.i749312 = phi i32 [ %_36.i65, %bb44.i78.lr.ph ], [ %main_cursor.sroa.0.1.i481.lcssa, %bb15.i72.loopexit ]
  %ring_cursor.sroa.0.0.i739311 = phi i32 [ %_37.i66, %bb44.i78.lr.ph ], [ %ring_cursor.sroa.0.1.i480.lcssa, %bb15.i72.loopexit ]
  %umin12548 = call i32 @llvm.umin.i32(i32 %indvars.iv12528, i32 32), !dbg !21644
  %umax12534 = call i32 @llvm.umax.i32(i32 %umin12548, i32 1), !dbg !21644
  %600 = add i32 %iter1.sroa.0.0.i759313, 32, !dbg !21644
  %601 = add nsw i32 %iter2.sroa.0.0.i769314, -1, !dbg !21648
  %602 = sub i32 %frames, %iter1.sroa.0.0.i759313, !dbg !21649
  %spec.store.select.i79 = tail call i32 @llvm.umin.i32(i32 %602, i32 32), !dbg !21650
  %_52.i80 = add i32 %spec.store.select.i79, %iter1.sroa.0.0.i759313, !dbg !21655
  %_176.i81 = icmp ult i32 %_52.i80, %iter1.sroa.0.0.i759313, !dbg !21656
  %_170.not.i82 = icmp ugt i32 %_52.i80, %left_io.1
  %or.cond.i83 = or i1 %_176.i81, %_170.not.i82, !dbg !21656
  br i1 %or.cond.i83, label %bb50.i702, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4680, !dbg !21656, !prof !4596

bb50.i702:                                        ; preds = %bb44.i78
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %iter1.sroa.0.0.i759313, i32 noundef %_52.i80, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_b74a748963eaf51a410c7eb21835ee21) #32, !dbg !21663, !noalias !21574
  unreachable, !dbg !21663

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4680: ; preds = %bb44.i78
  %_179.i85 = getelementptr inbounds nuw float, ptr %left_io.0, i32 %iter1.sroa.0.0.i759313, !dbg !21664
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21668), !dbg !21671
  %history.i44.i18.sroa.0.0.copyload = load float, ptr %hot_left.i55, align 4, !dbg !21672, !noalias !21674
  %history.i44.i18.sroa.7.0.copyload = load float, ptr %history.i44.i18.sroa.7.0.hot_left.i55.sroa_idx, align 4, !dbg !21672, !noalias !21674
  %history.i44.i18.sroa.10.0.copyload = load float, ptr %history.i44.i18.sroa.10.0.hot_left.i55.sroa_idx, align 4, !dbg !21672, !noalias !21674
  %history.i44.i18.sroa.13.0.copyload = load float, ptr %history.i44.i18.sroa.13.0.hot_left.i55.sroa_idx, align 4, !dbg !21672, !noalias !21674
  %history.i44.i18.sroa.16.0.copyload = load float, ptr %history.i44.i18.sroa.16.0.hot_left.i55.sroa_idx, align 4, !dbg !21672, !noalias !21674
  %history.i44.i18.sroa.19.0.copyload = load float, ptr %history.i44.i18.sroa.19.0.hot_left.i55.sroa_idx, align 4, !dbg !21672, !noalias !21674
  %history.i44.i18.sroa.22.0.copyload = load float, ptr %history.i44.i18.sroa.22.0.hot_left.i55.sroa_idx, align 4, !dbg !21672, !noalias !21674
  %history.i44.i18.sroa.26.0.copyload = load float, ptr %history.i44.i18.sroa.26.0.hot_left.i55.sroa_idx, align 4, !dbg !21672, !noalias !21674
  %history.i44.i18.sroa.29.0.copyload = load float, ptr %history.i44.i18.sroa.29.0.hot_left.i55.sroa_idx, align 4, !dbg !21672, !noalias !21674
  %history.i44.i18.sroa.32.0.copyload = load float, ptr %history.i44.i18.sroa.32.0.hot_left.i55.sroa_idx, align 4, !dbg !21672, !noalias !21674
  %history.i44.i18.sroa.35.0.copyload = load float, ptr %history.i44.i18.sroa.35.0.hot_left.i55.sroa_idx, align 4, !dbg !21672, !noalias !21674
  %history.i44.i18.sroa.38.0.copyload = load float, ptr %history.i44.i18.sroa.38.0.hot_left.i55.sroa_idx, align 4, !dbg !21672, !noalias !21674
  %_2.i46838521.not = icmp eq i32 %frames, %iter1.sroa.0.0.i759313, !dbg !21677
  br i1 %_2.i46838521.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit239.i280, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549.lr.ph, !dbg !21677

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549.lr.ph: ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4680
  %_11.i.i.i65.i106 = load float, ptr %_31, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_14.i.i.i68.i109 = load float, ptr %534, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_17.i.i.i71.i112 = load float, ptr %535, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_20.i.i.i74.i115 = load float, ptr %536, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_25.i.i.i79.i120 = load float, ptr %row1.i.i.i77.i118, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_28.i.i.i82.i123 = load float, ptr %537, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_31.i.i.i85.i126 = load float, ptr %538, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_34.i.i.i88.i129 = load float, ptr %539, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_39.i.i.i93.i134 = load float, ptr %row3.i.i.i91.i132, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_42.i.i.i96.i137 = load float, ptr %540, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_45.i.i.i99.i140 = load float, ptr %541, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_48.i.i.i102.i143 = load float, ptr %542, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_53.i.i.i107.i148 = load float, ptr %row5.i.i.i105.i146, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_56.i.i.i110.i151 = load float, ptr %543, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_59.i.i.i113.i154 = load float, ptr %544, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_62.i.i.i116.i157 = load float, ptr %545, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_67.i.i.i121.i162 = load float, ptr %row7.i.i.i119.i160, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_70.i.i.i124.i165 = load float, ptr %546, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_73.i.i.i127.i168 = load float, ptr %547, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_76.i.i.i130.i171 = load float, ptr %548, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_81.i.i.i135.i176 = load float, ptr %row9.i.i.i133.i174, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_84.i.i.i138.i179 = load float, ptr %549, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_87.i.i.i141.i182 = load float, ptr %550, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_90.i.i.i144.i185 = load float, ptr %551, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_95.i.i.i149.i190 = load float, ptr %row11.i.i.i147.i188, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_98.i.i.i152.i193 = load float, ptr %552, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_101.i.i.i155.i196 = load float, ptr %553, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_104.i.i.i158.i199 = load float, ptr %554, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_109.i.i.i163.i204 = load float, ptr %row13.i.i.i161.i202, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_112.i.i.i166.i207 = load float, ptr %555, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_115.i.i.i169.i210 = load float, ptr %556, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_118.i.i.i172.i213 = load float, ptr %557, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_123.i.i.i177.i218 = load float, ptr %row15.i.i.i175.i216, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_126.i.i.i180.i221 = load float, ptr %558, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_129.i.i.i183.i224 = load float, ptr %559, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_132.i.i.i186.i227 = load float, ptr %560, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_137.i.i.i191.i232 = load float, ptr %row17.i.i.i189.i230, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_140.i.i.i194.i235 = load float, ptr %561, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_143.i.i.i197.i238 = load float, ptr %562, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_146.i.i.i200.i241 = load float, ptr %563, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_151.i.i.i205.i246 = load float, ptr %row19.i.i.i203.i244, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_154.i.i.i208.i249 = load float, ptr %564, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_157.i.i.i211.i252 = load float, ptr %565, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_160.i.i.i214.i255 = load float, ptr %566, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_165.i.i.i219.i260 = load float, ptr %row21.i.i.i217.i258, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_168.i.i.i222.i263 = load float, ptr %567, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_171.i.i.i225.i266 = load float, ptr %568, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  %_174.i.i.i228.i269 = load float, ptr %569, align 4, !alias.scope !21680, !noalias !21685, !noundef !10
  br label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549, !dbg !21677

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549
  %iter.i40.i14.sroa.16.08533 = phi i32 [ 0, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549.lr.ph ], [ %608, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549 ]
  %history.i44.i18.sroa.35.08532 = phi float [ %history.i44.i18.sroa.35.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549.lr.ph ], [ %history.i44.i18.sroa.32.08531, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549 ]
  %history.i44.i18.sroa.32.08531 = phi float [ %history.i44.i18.sroa.32.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549.lr.ph ], [ %history.i44.i18.sroa.29.08530, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549 ]
  %history.i44.i18.sroa.29.08530 = phi float [ %history.i44.i18.sroa.29.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549.lr.ph ], [ %history.i44.i18.sroa.26.08529, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549 ]
  %history.i44.i18.sroa.26.08529 = phi float [ %history.i44.i18.sroa.26.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549.lr.ph ], [ %history.i44.i18.sroa.22.08528, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549 ]
  %history.i44.i18.sroa.22.08528 = phi float [ %history.i44.i18.sroa.22.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549.lr.ph ], [ %history.i44.i18.sroa.19.08527, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549 ]
  %history.i44.i18.sroa.19.08527 = phi float [ %history.i44.i18.sroa.19.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549.lr.ph ], [ %history.i44.i18.sroa.16.08526, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549 ]
  %history.i44.i18.sroa.16.08526 = phi float [ %history.i44.i18.sroa.16.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549.lr.ph ], [ %history.i44.i18.sroa.13.08525, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549 ]
  %history.i44.i18.sroa.13.08525 = phi float [ %history.i44.i18.sroa.13.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549.lr.ph ], [ %history.i44.i18.sroa.10.08524, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549 ]
  %history.i44.i18.sroa.10.08524 = phi float [ %history.i44.i18.sroa.10.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549.lr.ph ], [ %history.i44.i18.sroa.7.08523, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549 ]
  %history.i44.i18.sroa.7.08523 = phi float [ %history.i44.i18.sroa.7.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549.lr.ph ], [ %history.i44.i18.sroa.0.08522, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549 ]
  %history.i44.i18.sroa.0.08522 = phi float [ %history.i44.i18.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549.lr.ph ], [ %_0.i3547, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549 ]
  %data.i.i4690 = getelementptr inbounds nuw float, ptr %_179.i85, i32 %iter.i40.i14.sroa.16.08533, !dbg !21690
  %_0.i3547 = load float, ptr %data.i.i4690, align 4, !dbg !21693, !alias.scope !21695, !noalias !21698, !noundef !10
  %603 = tail call noundef float @llvm.fabs.f32(float %history.i44.i18.sroa.19.08527), !dbg !21699
  %_0.i3236 = fmul float %_0.i3547, %_11.i.i.i65.i106, !dbg !21702
  %_0.i2800 = fadd float %_0.i3236, 0.000000e+00, !dbg !21705
  %_0.i3235 = fmul float %_0.i3547, %_14.i.i.i68.i109, !dbg !21707
  %_0.i2799 = fadd float %_0.i3235, 0.000000e+00, !dbg !21709
  %_0.i3234 = fmul float %_0.i3547, %_17.i.i.i71.i112, !dbg !21711
  %_0.i2798 = fadd float %_0.i3234, 0.000000e+00, !dbg !21713
  %_0.i3233 = fmul float %_0.i3547, %_20.i.i.i74.i115, !dbg !21715
  %_0.i2797 = fadd float %_0.i3233, 0.000000e+00, !dbg !21717
  %_0.i3232 = fmul float %history.i44.i18.sroa.0.08522, %_25.i.i.i79.i120, !dbg !21719
  %_0.i2796 = fadd float %_0.i2800, %_0.i3232, !dbg !21721
  %_0.i3231 = fmul float %history.i44.i18.sroa.0.08522, %_28.i.i.i82.i123, !dbg !21723
  %_0.i2795 = fadd float %_0.i2799, %_0.i3231, !dbg !21725
  %_0.i3230 = fmul float %history.i44.i18.sroa.0.08522, %_31.i.i.i85.i126, !dbg !21727
  %_0.i2794 = fadd float %_0.i2798, %_0.i3230, !dbg !21729
  %_0.i3229 = fmul float %history.i44.i18.sroa.0.08522, %_34.i.i.i88.i129, !dbg !21731
  %_0.i2793 = fadd float %_0.i2797, %_0.i3229, !dbg !21733
  %_0.i3228 = fmul float %history.i44.i18.sroa.7.08523, %_39.i.i.i93.i134, !dbg !21735
  %_0.i2792 = fadd float %_0.i2796, %_0.i3228, !dbg !21737
  %_0.i3227 = fmul float %history.i44.i18.sroa.7.08523, %_42.i.i.i96.i137, !dbg !21739
  %_0.i2791 = fadd float %_0.i2795, %_0.i3227, !dbg !21741
  %_0.i3226 = fmul float %history.i44.i18.sroa.7.08523, %_45.i.i.i99.i140, !dbg !21743
  %_0.i2790 = fadd float %_0.i2794, %_0.i3226, !dbg !21745
  %_0.i3225 = fmul float %history.i44.i18.sroa.7.08523, %_48.i.i.i102.i143, !dbg !21747
  %_0.i2789 = fadd float %_0.i2793, %_0.i3225, !dbg !21749
  %_0.i3224 = fmul float %history.i44.i18.sroa.10.08524, %_53.i.i.i107.i148, !dbg !21751
  %_0.i2788 = fadd float %_0.i2792, %_0.i3224, !dbg !21753
  %_0.i3223 = fmul float %history.i44.i18.sroa.10.08524, %_56.i.i.i110.i151, !dbg !21755
  %_0.i2787 = fadd float %_0.i2791, %_0.i3223, !dbg !21757
  %_0.i3222 = fmul float %history.i44.i18.sroa.10.08524, %_59.i.i.i113.i154, !dbg !21759
  %_0.i2786 = fadd float %_0.i2790, %_0.i3222, !dbg !21761
  %_0.i3221 = fmul float %history.i44.i18.sroa.10.08524, %_62.i.i.i116.i157, !dbg !21763
  %_0.i2785 = fadd float %_0.i2789, %_0.i3221, !dbg !21765
  %_0.i3220 = fmul float %history.i44.i18.sroa.13.08525, %_67.i.i.i121.i162, !dbg !21767
  %_0.i2784 = fadd float %_0.i2788, %_0.i3220, !dbg !21769
  %_0.i3219 = fmul float %history.i44.i18.sroa.13.08525, %_70.i.i.i124.i165, !dbg !21771
  %_0.i2783 = fadd float %_0.i2787, %_0.i3219, !dbg !21773
  %_0.i3218 = fmul float %history.i44.i18.sroa.13.08525, %_73.i.i.i127.i168, !dbg !21775
  %_0.i2782 = fadd float %_0.i2786, %_0.i3218, !dbg !21777
  %_0.i3217 = fmul float %history.i44.i18.sroa.13.08525, %_76.i.i.i130.i171, !dbg !21779
  %_0.i2781 = fadd float %_0.i2785, %_0.i3217, !dbg !21781
  %_0.i3216 = fmul float %history.i44.i18.sroa.16.08526, %_81.i.i.i135.i176, !dbg !21783
  %_0.i2780 = fadd float %_0.i2784, %_0.i3216, !dbg !21785
  %_0.i3215 = fmul float %history.i44.i18.sroa.16.08526, %_84.i.i.i138.i179, !dbg !21787
  %_0.i2779 = fadd float %_0.i2783, %_0.i3215, !dbg !21789
  %_0.i3214 = fmul float %history.i44.i18.sroa.16.08526, %_87.i.i.i141.i182, !dbg !21791
  %_0.i2778 = fadd float %_0.i2782, %_0.i3214, !dbg !21793
  %_0.i3213 = fmul float %history.i44.i18.sroa.16.08526, %_90.i.i.i144.i185, !dbg !21795
  %_0.i2777 = fadd float %_0.i2781, %_0.i3213, !dbg !21797
  %_0.i3212 = fmul float %history.i44.i18.sroa.19.08527, %_95.i.i.i149.i190, !dbg !21799
  %_0.i2776 = fadd float %_0.i2780, %_0.i3212, !dbg !21801
  %_0.i3211 = fmul float %history.i44.i18.sroa.19.08527, %_98.i.i.i152.i193, !dbg !21803
  %_0.i2775 = fadd float %_0.i2779, %_0.i3211, !dbg !21805
  %_0.i3210 = fmul float %history.i44.i18.sroa.19.08527, %_101.i.i.i155.i196, !dbg !21807
  %_0.i2774 = fadd float %_0.i2778, %_0.i3210, !dbg !21809
  %_0.i3209 = fmul float %history.i44.i18.sroa.19.08527, %_104.i.i.i158.i199, !dbg !21811
  %_0.i2773 = fadd float %_0.i2777, %_0.i3209, !dbg !21813
  %_0.i3208 = fmul float %history.i44.i18.sroa.22.08528, %_109.i.i.i163.i204, !dbg !21815
  %_0.i2772 = fadd float %_0.i2776, %_0.i3208, !dbg !21817
  %_0.i3207 = fmul float %history.i44.i18.sroa.22.08528, %_112.i.i.i166.i207, !dbg !21819
  %_0.i2771 = fadd float %_0.i2775, %_0.i3207, !dbg !21821
  %_0.i3206 = fmul float %history.i44.i18.sroa.22.08528, %_115.i.i.i169.i210, !dbg !21823
  %_0.i2770 = fadd float %_0.i2774, %_0.i3206, !dbg !21825
  %_0.i3205 = fmul float %history.i44.i18.sroa.22.08528, %_118.i.i.i172.i213, !dbg !21827
  %_0.i2769 = fadd float %_0.i2773, %_0.i3205, !dbg !21829
  %_0.i3204 = fmul float %history.i44.i18.sroa.26.08529, %_123.i.i.i177.i218, !dbg !21831
  %_0.i2768 = fadd float %_0.i2772, %_0.i3204, !dbg !21833
  %_0.i3203 = fmul float %history.i44.i18.sroa.26.08529, %_126.i.i.i180.i221, !dbg !21835
  %_0.i2767 = fadd float %_0.i2771, %_0.i3203, !dbg !21837
  %_0.i3202 = fmul float %history.i44.i18.sroa.26.08529, %_129.i.i.i183.i224, !dbg !21839
  %_0.i2766 = fadd float %_0.i2770, %_0.i3202, !dbg !21841
  %_0.i3201 = fmul float %history.i44.i18.sroa.26.08529, %_132.i.i.i186.i227, !dbg !21843
  %_0.i2765 = fadd float %_0.i2769, %_0.i3201, !dbg !21845
  %_0.i3200 = fmul float %history.i44.i18.sroa.29.08530, %_137.i.i.i191.i232, !dbg !21847
  %_0.i2764 = fadd float %_0.i2768, %_0.i3200, !dbg !21849
  %_0.i3199 = fmul float %history.i44.i18.sroa.29.08530, %_140.i.i.i194.i235, !dbg !21851
  %_0.i2763 = fadd float %_0.i2767, %_0.i3199, !dbg !21853
  %_0.i3198 = fmul float %history.i44.i18.sroa.29.08530, %_143.i.i.i197.i238, !dbg !21855
  %_0.i2762 = fadd float %_0.i2766, %_0.i3198, !dbg !21857
  %_0.i3197 = fmul float %history.i44.i18.sroa.29.08530, %_146.i.i.i200.i241, !dbg !21859
  %_0.i2761 = fadd float %_0.i2765, %_0.i3197, !dbg !21861
  %_0.i3196 = fmul float %history.i44.i18.sroa.32.08531, %_151.i.i.i205.i246, !dbg !21863
  %_0.i2760 = fadd float %_0.i2764, %_0.i3196, !dbg !21865
  %_0.i3195 = fmul float %history.i44.i18.sroa.32.08531, %_154.i.i.i208.i249, !dbg !21867
  %_0.i2759 = fadd float %_0.i2763, %_0.i3195, !dbg !21869
  %_0.i3194 = fmul float %history.i44.i18.sroa.32.08531, %_157.i.i.i211.i252, !dbg !21871
  %_0.i2758 = fadd float %_0.i2762, %_0.i3194, !dbg !21873
  %_0.i3193 = fmul float %history.i44.i18.sroa.32.08531, %_160.i.i.i214.i255, !dbg !21875
  %_0.i2757 = fadd float %_0.i2761, %_0.i3193, !dbg !21877
  %_0.i3192 = fmul float %history.i44.i18.sroa.35.08532, %_165.i.i.i219.i260, !dbg !21879
  %_0.i2756 = fadd float %_0.i2760, %_0.i3192, !dbg !21881
  %_0.i3191 = fmul float %history.i44.i18.sroa.35.08532, %_168.i.i.i222.i263, !dbg !21883
  %_0.i2755 = fadd float %_0.i2759, %_0.i3191, !dbg !21885
  %_0.i3190 = fmul float %history.i44.i18.sroa.35.08532, %_171.i.i.i225.i266, !dbg !21887
  %_0.i2754 = fadd float %_0.i2758, %_0.i3190, !dbg !21889
  %_0.i3189 = fmul float %history.i44.i18.sroa.35.08532, %_174.i.i.i228.i269, !dbg !21891
  %_0.i2753 = fadd float %_0.i2757, %_0.i3189, !dbg !21893
  %604 = tail call noundef float @llvm.fabs.f32(float %_0.i2756), !dbg !21895
  %_3.i.i4190.inv = fcmp ogt float %603, %604, !dbg !21897
  %_4.i.i4197.v = select i1 %_3.i.i4190.inv, float %603, float %604, !dbg !21897
  %605 = tail call noundef float @llvm.fabs.f32(float %_0.i2755), !dbg !21895
  %_3.i.i4190.inv.1 = fcmp ogt float %_4.i.i4197.v, %605, !dbg !21897
  %_4.i.i4197.v.1 = select i1 %_3.i.i4190.inv.1, float %_4.i.i4197.v, float %605, !dbg !21897
  %606 = tail call noundef float @llvm.fabs.f32(float %_0.i2754), !dbg !21895
  %_3.i.i4190.inv.2 = fcmp ogt float %_4.i.i4197.v.1, %606, !dbg !21897
  %_4.i.i4197.v.2 = select i1 %_3.i.i4190.inv.2, float %_4.i.i4197.v.1, float %606, !dbg !21897
  %607 = tail call noundef float @llvm.fabs.f32(float %_0.i2753), !dbg !21895
  %_3.i.i4190.inv.3 = fcmp ogt float %_4.i.i4197.v.2, %607, !dbg !21897
  %_4.i.i4197.v.3 = select i1 %_3.i.i4190.inv.3, float %_4.i.i4197.v.2, float %607, !dbg !21897
  %608 = add nuw nsw i32 %iter.i40.i14.sroa.16.08533, 1, !dbg !21900
  %data.i4.i4694 = getelementptr inbounds nuw float, ptr %peaks_left.i53, i32 %iter.i40.i14.sroa.16.08533, !dbg !21901
  store float %_4.i.i4197.v.3, ptr %data.i4.i4694, align 4, !dbg !21904, !alias.scope !21906, !noalias !21698
  %exitcond12532.not = icmp eq i32 %608, %umax12534, !dbg !21677
  br i1 %exitcond12532.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit239.i280, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549, !dbg !21677

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit239.i280: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4680
  %history.i44.i18.sroa.0.0.lcssa = phi float [ %history.i44.i18.sroa.0.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4680 ], [ %_0.i3547, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549 ], !dbg !21909
  %history.i44.i18.sroa.7.0.lcssa = phi float [ %history.i44.i18.sroa.7.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4680 ], [ %history.i44.i18.sroa.0.08522, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549 ], !dbg !21909
  %history.i44.i18.sroa.10.0.lcssa = phi float [ %history.i44.i18.sroa.10.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4680 ], [ %history.i44.i18.sroa.7.08523, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549 ], !dbg !21909
  %history.i44.i18.sroa.13.0.lcssa = phi float [ %history.i44.i18.sroa.13.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4680 ], [ %history.i44.i18.sroa.10.08524, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549 ], !dbg !21909
  %history.i44.i18.sroa.16.0.lcssa = phi float [ %history.i44.i18.sroa.16.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4680 ], [ %history.i44.i18.sroa.13.08525, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549 ], !dbg !21909
  %history.i44.i18.sroa.19.0.lcssa = phi float [ %history.i44.i18.sroa.19.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4680 ], [ %history.i44.i18.sroa.16.08526, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549 ], !dbg !21909
  %history.i44.i18.sroa.22.0.lcssa = phi float [ %history.i44.i18.sroa.22.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4680 ], [ %history.i44.i18.sroa.19.08527, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549 ], !dbg !21909
  %history.i44.i18.sroa.26.0.lcssa = phi float [ %history.i44.i18.sroa.26.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4680 ], [ %history.i44.i18.sroa.22.08528, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549 ], !dbg !21909
  %history.i44.i18.sroa.29.0.lcssa = phi float [ %history.i44.i18.sroa.29.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4680 ], [ %history.i44.i18.sroa.26.08529, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549 ], !dbg !21909
  %history.i44.i18.sroa.32.0.lcssa = phi float [ %history.i44.i18.sroa.32.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4680 ], [ %history.i44.i18.sroa.29.08530, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549 ], !dbg !21909
  %history.i44.i18.sroa.35.0.lcssa = phi float [ %history.i44.i18.sroa.35.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4680 ], [ %history.i44.i18.sroa.32.08531, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549 ], !dbg !21909
  %history.i44.i18.sroa.38.0.lcssa = phi float [ %history.i44.i18.sroa.38.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4680 ], [ %history.i44.i18.sroa.35.08532, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3549 ], !dbg !21909
  store float %history.i44.i18.sroa.0.0.lcssa, ptr %hot_left.i55, align 4, !dbg !21910, !noalias !21674
  store float %history.i44.i18.sroa.7.0.lcssa, ptr %history.i44.i18.sroa.7.0.hot_left.i55.sroa_idx, align 4, !dbg !21910, !noalias !21674
  store float %history.i44.i18.sroa.10.0.lcssa, ptr %history.i44.i18.sroa.10.0.hot_left.i55.sroa_idx, align 4, !dbg !21910, !noalias !21674
  store float %history.i44.i18.sroa.13.0.lcssa, ptr %history.i44.i18.sroa.13.0.hot_left.i55.sroa_idx, align 4, !dbg !21910, !noalias !21674
  store float %history.i44.i18.sroa.16.0.lcssa, ptr %history.i44.i18.sroa.16.0.hot_left.i55.sroa_idx, align 4, !dbg !21910, !noalias !21674
  store float %history.i44.i18.sroa.19.0.lcssa, ptr %history.i44.i18.sroa.19.0.hot_left.i55.sroa_idx, align 4, !dbg !21910, !noalias !21674
  store float %history.i44.i18.sroa.22.0.lcssa, ptr %history.i44.i18.sroa.22.0.hot_left.i55.sroa_idx, align 4, !dbg !21910, !noalias !21674
  store float %history.i44.i18.sroa.26.0.lcssa, ptr %history.i44.i18.sroa.26.0.hot_left.i55.sroa_idx, align 4, !dbg !21910, !noalias !21674
  store float %history.i44.i18.sroa.29.0.lcssa, ptr %history.i44.i18.sroa.29.0.hot_left.i55.sroa_idx, align 4, !dbg !21910, !noalias !21674
  store float %history.i44.i18.sroa.32.0.lcssa, ptr %history.i44.i18.sroa.32.0.hot_left.i55.sroa_idx, align 4, !dbg !21910, !noalias !21674
  store float %history.i44.i18.sroa.35.0.lcssa, ptr %history.i44.i18.sroa.35.0.hot_left.i55.sroa_idx, align 4, !dbg !21910, !noalias !21674
  store float %history.i44.i18.sroa.38.0.lcssa, ptr %history.i44.i18.sroa.38.0.hot_left.i55.sroa_idx, align 4, !dbg !21910, !noalias !21674
  %_187.not.i281 = icmp ugt i32 %_52.i80, %right_io.1, !dbg !21911
  br i1 %_187.not.i281, label %bb56.i701, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4725, !dbg !21911, !prof !787

bb56.i701:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit239.i280
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %iter1.sroa.0.0.i759313, i32 noundef %_52.i80, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_8715d54bb9ab90680506cd3587af9682) #32, !dbg !21915, !noalias !21574
  unreachable, !dbg !21915

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4725: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit239.i280
  %_194.i283 = getelementptr inbounds nuw float, ptr %right_io.0, i32 %iter1.sroa.0.0.i759313, !dbg !21916
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21920), !dbg !21923
  %history.i.i32.sroa.0.0.copyload = load float, ptr %hot_right.i54, align 4, !dbg !21924, !noalias !21926
  %history.i.i32.sroa.7.0.copyload = load float, ptr %history.i.i32.sroa.7.0.hot_right.i54.sroa_idx, align 4, !dbg !21924, !noalias !21926
  %history.i.i32.sroa.10.0.copyload = load float, ptr %history.i.i32.sroa.10.0.hot_right.i54.sroa_idx, align 4, !dbg !21924, !noalias !21926
  %history.i.i32.sroa.13.0.copyload = load float, ptr %history.i.i32.sroa.13.0.hot_right.i54.sroa_idx, align 4, !dbg !21924, !noalias !21926
  %history.i.i32.sroa.16.0.copyload = load float, ptr %history.i.i32.sroa.16.0.hot_right.i54.sroa_idx, align 4, !dbg !21924, !noalias !21926
  %history.i.i32.sroa.19.0.copyload = load float, ptr %history.i.i32.sroa.19.0.hot_right.i54.sroa_idx, align 4, !dbg !21924, !noalias !21926
  %history.i.i32.sroa.22.0.copyload = load float, ptr %history.i.i32.sroa.22.0.hot_right.i54.sroa_idx, align 4, !dbg !21924, !noalias !21926
  %history.i.i32.sroa.26.0.copyload = load float, ptr %history.i.i32.sroa.26.0.hot_right.i54.sroa_idx, align 4, !dbg !21924, !noalias !21926
  %history.i.i32.sroa.29.0.copyload = load float, ptr %history.i.i32.sroa.29.0.hot_right.i54.sroa_idx, align 4, !dbg !21924, !noalias !21926
  %history.i.i32.sroa.32.0.copyload = load float, ptr %history.i.i32.sroa.32.0.hot_right.i54.sroa_idx, align 4, !dbg !21924, !noalias !21926
  %history.i.i32.sroa.35.0.copyload = load float, ptr %history.i.i32.sroa.35.0.hot_right.i54.sroa_idx, align 4, !dbg !21924, !noalias !21926
  %history.i.i32.sroa.38.0.copyload = load float, ptr %history.i.i32.sroa.38.0.hot_right.i54.sroa_idx, align 4, !dbg !21924, !noalias !21926
  br i1 %_2.i46838521.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i478, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544.lr.ph, !dbg !21929

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544.lr.ph: ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4725
  %_11.i.i.i.i304 = load float, ptr %_31, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_14.i.i.i.i307 = load float, ptr %534, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_17.i.i.i.i310 = load float, ptr %535, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_20.i.i.i.i313 = load float, ptr %536, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_25.i.i.i.i318 = load float, ptr %row1.i.i.i77.i118, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_28.i.i.i.i321 = load float, ptr %537, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_31.i.i.i.i324 = load float, ptr %538, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_34.i.i.i.i327 = load float, ptr %539, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_39.i.i.i.i332 = load float, ptr %row3.i.i.i91.i132, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_42.i.i.i.i335 = load float, ptr %540, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_45.i.i.i.i338 = load float, ptr %541, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_48.i.i.i.i341 = load float, ptr %542, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_53.i.i.i.i346 = load float, ptr %row5.i.i.i105.i146, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_56.i.i.i.i349 = load float, ptr %543, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_59.i.i.i.i352 = load float, ptr %544, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_62.i.i.i.i355 = load float, ptr %545, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_67.i.i.i.i360 = load float, ptr %row7.i.i.i119.i160, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_70.i.i.i.i363 = load float, ptr %546, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_73.i.i.i.i366 = load float, ptr %547, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_76.i.i.i.i369 = load float, ptr %548, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_81.i.i.i.i374 = load float, ptr %row9.i.i.i133.i174, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_84.i.i.i.i377 = load float, ptr %549, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_87.i.i.i.i380 = load float, ptr %550, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_90.i.i.i.i383 = load float, ptr %551, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_95.i.i.i.i388 = load float, ptr %row11.i.i.i147.i188, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_98.i.i.i.i391 = load float, ptr %552, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_101.i.i.i.i394 = load float, ptr %553, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_104.i.i.i.i397 = load float, ptr %554, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_109.i.i.i.i402 = load float, ptr %row13.i.i.i161.i202, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_112.i.i.i.i405 = load float, ptr %555, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_115.i.i.i.i408 = load float, ptr %556, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_118.i.i.i.i411 = load float, ptr %557, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_123.i.i.i.i416 = load float, ptr %row15.i.i.i175.i216, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_126.i.i.i.i419 = load float, ptr %558, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_129.i.i.i.i422 = load float, ptr %559, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_132.i.i.i.i425 = load float, ptr %560, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_137.i.i.i.i430 = load float, ptr %row17.i.i.i189.i230, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_140.i.i.i.i433 = load float, ptr %561, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_143.i.i.i.i436 = load float, ptr %562, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_146.i.i.i.i439 = load float, ptr %563, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_151.i.i.i.i444 = load float, ptr %row19.i.i.i203.i244, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_154.i.i.i.i447 = load float, ptr %564, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_157.i.i.i.i450 = load float, ptr %565, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_160.i.i.i.i453 = load float, ptr %566, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_165.i.i.i.i458 = load float, ptr %row21.i.i.i217.i258, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_168.i.i.i.i461 = load float, ptr %567, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_171.i.i.i.i464 = load float, ptr %568, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  %_174.i.i.i.i467 = load float, ptr %569, align 4, !alias.scope !21932, !noalias !21937, !noundef !10
  br label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544, !dbg !21929

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544
  %iter.i.i28.sroa.16.08560 = phi i32 [ 0, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544.lr.ph ], [ %614, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544 ]
  %history.i.i32.sroa.35.08559 = phi float [ %history.i.i32.sroa.35.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544.lr.ph ], [ %history.i.i32.sroa.32.08558, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544 ]
  %history.i.i32.sroa.32.08558 = phi float [ %history.i.i32.sroa.32.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544.lr.ph ], [ %history.i.i32.sroa.29.08557, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544 ]
  %history.i.i32.sroa.29.08557 = phi float [ %history.i.i32.sroa.29.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544.lr.ph ], [ %history.i.i32.sroa.26.08556, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544 ]
  %history.i.i32.sroa.26.08556 = phi float [ %history.i.i32.sroa.26.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544.lr.ph ], [ %history.i.i32.sroa.22.08555, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544 ]
  %history.i.i32.sroa.22.08555 = phi float [ %history.i.i32.sroa.22.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544.lr.ph ], [ %history.i.i32.sroa.19.08554, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544 ]
  %history.i.i32.sroa.19.08554 = phi float [ %history.i.i32.sroa.19.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544.lr.ph ], [ %history.i.i32.sroa.16.08553, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544 ]
  %history.i.i32.sroa.16.08553 = phi float [ %history.i.i32.sroa.16.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544.lr.ph ], [ %history.i.i32.sroa.13.08552, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544 ]
  %history.i.i32.sroa.13.08552 = phi float [ %history.i.i32.sroa.13.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544.lr.ph ], [ %history.i.i32.sroa.10.08551, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544 ]
  %history.i.i32.sroa.10.08551 = phi float [ %history.i.i32.sroa.10.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544.lr.ph ], [ %history.i.i32.sroa.7.08550, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544 ]
  %history.i.i32.sroa.7.08550 = phi float [ %history.i.i32.sroa.7.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544.lr.ph ], [ %history.i.i32.sroa.0.08549, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544 ]
  %history.i.i32.sroa.0.08549 = phi float [ %history.i.i32.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544.lr.ph ], [ %_0.i3542, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544 ]
  %data.i.i4735 = getelementptr inbounds nuw float, ptr %_194.i283, i32 %iter.i.i28.sroa.16.08560, !dbg !21942
  %_0.i3542 = load float, ptr %data.i.i4735, align 4, !dbg !21945, !alias.scope !21947, !noalias !21950, !noundef !10
  %609 = tail call noundef float @llvm.fabs.f32(float %history.i.i32.sroa.19.08554), !dbg !21951
  %_0.i3188 = fmul float %_0.i3542, %_11.i.i.i.i304, !dbg !21954
  %_0.i2752 = fadd float %_0.i3188, 0.000000e+00, !dbg !21957
  %_0.i3187 = fmul float %_0.i3542, %_14.i.i.i.i307, !dbg !21959
  %_0.i2751 = fadd float %_0.i3187, 0.000000e+00, !dbg !21961
  %_0.i3186 = fmul float %_0.i3542, %_17.i.i.i.i310, !dbg !21963
  %_0.i2750 = fadd float %_0.i3186, 0.000000e+00, !dbg !21965
  %_0.i3185 = fmul float %_0.i3542, %_20.i.i.i.i313, !dbg !21967
  %_0.i2749 = fadd float %_0.i3185, 0.000000e+00, !dbg !21969
  %_0.i3184 = fmul float %history.i.i32.sroa.0.08549, %_25.i.i.i.i318, !dbg !21971
  %_0.i2748 = fadd float %_0.i2752, %_0.i3184, !dbg !21973
  %_0.i3183 = fmul float %history.i.i32.sroa.0.08549, %_28.i.i.i.i321, !dbg !21975
  %_0.i2747 = fadd float %_0.i2751, %_0.i3183, !dbg !21977
  %_0.i3182 = fmul float %history.i.i32.sroa.0.08549, %_31.i.i.i.i324, !dbg !21979
  %_0.i2746 = fadd float %_0.i2750, %_0.i3182, !dbg !21981
  %_0.i3181 = fmul float %history.i.i32.sroa.0.08549, %_34.i.i.i.i327, !dbg !21983
  %_0.i2745 = fadd float %_0.i2749, %_0.i3181, !dbg !21985
  %_0.i3180 = fmul float %history.i.i32.sroa.7.08550, %_39.i.i.i.i332, !dbg !21987
  %_0.i2744 = fadd float %_0.i2748, %_0.i3180, !dbg !21989
  %_0.i3179 = fmul float %history.i.i32.sroa.7.08550, %_42.i.i.i.i335, !dbg !21991
  %_0.i2743 = fadd float %_0.i2747, %_0.i3179, !dbg !21993
  %_0.i3178 = fmul float %history.i.i32.sroa.7.08550, %_45.i.i.i.i338, !dbg !21995
  %_0.i2742 = fadd float %_0.i2746, %_0.i3178, !dbg !21997
  %_0.i3177 = fmul float %history.i.i32.sroa.7.08550, %_48.i.i.i.i341, !dbg !21999
  %_0.i2741 = fadd float %_0.i2745, %_0.i3177, !dbg !22001
  %_0.i3176 = fmul float %history.i.i32.sroa.10.08551, %_53.i.i.i.i346, !dbg !22003
  %_0.i2740 = fadd float %_0.i2744, %_0.i3176, !dbg !22005
  %_0.i3175 = fmul float %history.i.i32.sroa.10.08551, %_56.i.i.i.i349, !dbg !22007
  %_0.i2739 = fadd float %_0.i2743, %_0.i3175, !dbg !22009
  %_0.i3174 = fmul float %history.i.i32.sroa.10.08551, %_59.i.i.i.i352, !dbg !22011
  %_0.i2738 = fadd float %_0.i2742, %_0.i3174, !dbg !22013
  %_0.i3173 = fmul float %history.i.i32.sroa.10.08551, %_62.i.i.i.i355, !dbg !22015
  %_0.i2737 = fadd float %_0.i2741, %_0.i3173, !dbg !22017
  %_0.i3172 = fmul float %history.i.i32.sroa.13.08552, %_67.i.i.i.i360, !dbg !22019
  %_0.i2736 = fadd float %_0.i2740, %_0.i3172, !dbg !22021
  %_0.i3171 = fmul float %history.i.i32.sroa.13.08552, %_70.i.i.i.i363, !dbg !22023
  %_0.i2735 = fadd float %_0.i2739, %_0.i3171, !dbg !22025
  %_0.i3170 = fmul float %history.i.i32.sroa.13.08552, %_73.i.i.i.i366, !dbg !22027
  %_0.i2734 = fadd float %_0.i2738, %_0.i3170, !dbg !22029
  %_0.i3169 = fmul float %history.i.i32.sroa.13.08552, %_76.i.i.i.i369, !dbg !22031
  %_0.i2733 = fadd float %_0.i2737, %_0.i3169, !dbg !22033
  %_0.i3168 = fmul float %history.i.i32.sroa.16.08553, %_81.i.i.i.i374, !dbg !22035
  %_0.i2732 = fadd float %_0.i2736, %_0.i3168, !dbg !22037
  %_0.i3167 = fmul float %history.i.i32.sroa.16.08553, %_84.i.i.i.i377, !dbg !22039
  %_0.i2731 = fadd float %_0.i2735, %_0.i3167, !dbg !22041
  %_0.i3166 = fmul float %history.i.i32.sroa.16.08553, %_87.i.i.i.i380, !dbg !22043
  %_0.i2730 = fadd float %_0.i2734, %_0.i3166, !dbg !22045
  %_0.i3165 = fmul float %history.i.i32.sroa.16.08553, %_90.i.i.i.i383, !dbg !22047
  %_0.i2729 = fadd float %_0.i2733, %_0.i3165, !dbg !22049
  %_0.i3164 = fmul float %history.i.i32.sroa.19.08554, %_95.i.i.i.i388, !dbg !22051
  %_0.i2728 = fadd float %_0.i2732, %_0.i3164, !dbg !22053
  %_0.i3163 = fmul float %history.i.i32.sroa.19.08554, %_98.i.i.i.i391, !dbg !22055
  %_0.i2727 = fadd float %_0.i2731, %_0.i3163, !dbg !22057
  %_0.i3162 = fmul float %history.i.i32.sroa.19.08554, %_101.i.i.i.i394, !dbg !22059
  %_0.i2726 = fadd float %_0.i2730, %_0.i3162, !dbg !22061
  %_0.i3161 = fmul float %history.i.i32.sroa.19.08554, %_104.i.i.i.i397, !dbg !22063
  %_0.i2725 = fadd float %_0.i2729, %_0.i3161, !dbg !22065
  %_0.i3160 = fmul float %history.i.i32.sroa.22.08555, %_109.i.i.i.i402, !dbg !22067
  %_0.i2724 = fadd float %_0.i2728, %_0.i3160, !dbg !22069
  %_0.i3159 = fmul float %history.i.i32.sroa.22.08555, %_112.i.i.i.i405, !dbg !22071
  %_0.i2723 = fadd float %_0.i2727, %_0.i3159, !dbg !22073
  %_0.i3158 = fmul float %history.i.i32.sroa.22.08555, %_115.i.i.i.i408, !dbg !22075
  %_0.i2722 = fadd float %_0.i2726, %_0.i3158, !dbg !22077
  %_0.i3157 = fmul float %history.i.i32.sroa.22.08555, %_118.i.i.i.i411, !dbg !22079
  %_0.i2721 = fadd float %_0.i2725, %_0.i3157, !dbg !22081
  %_0.i3156 = fmul float %history.i.i32.sroa.26.08556, %_123.i.i.i.i416, !dbg !22083
  %_0.i2720 = fadd float %_0.i2724, %_0.i3156, !dbg !22085
  %_0.i3155 = fmul float %history.i.i32.sroa.26.08556, %_126.i.i.i.i419, !dbg !22087
  %_0.i2719 = fadd float %_0.i2723, %_0.i3155, !dbg !22089
  %_0.i3154 = fmul float %history.i.i32.sroa.26.08556, %_129.i.i.i.i422, !dbg !22091
  %_0.i2718 = fadd float %_0.i2722, %_0.i3154, !dbg !22093
  %_0.i3153 = fmul float %history.i.i32.sroa.26.08556, %_132.i.i.i.i425, !dbg !22095
  %_0.i2717 = fadd float %_0.i2721, %_0.i3153, !dbg !22097
  %_0.i3152 = fmul float %history.i.i32.sroa.29.08557, %_137.i.i.i.i430, !dbg !22099
  %_0.i2716 = fadd float %_0.i2720, %_0.i3152, !dbg !22101
  %_0.i3151 = fmul float %history.i.i32.sroa.29.08557, %_140.i.i.i.i433, !dbg !22103
  %_0.i2715 = fadd float %_0.i2719, %_0.i3151, !dbg !22105
  %_0.i3150 = fmul float %history.i.i32.sroa.29.08557, %_143.i.i.i.i436, !dbg !22107
  %_0.i2714 = fadd float %_0.i2718, %_0.i3150, !dbg !22109
  %_0.i3149 = fmul float %history.i.i32.sroa.29.08557, %_146.i.i.i.i439, !dbg !22111
  %_0.i2713 = fadd float %_0.i2717, %_0.i3149, !dbg !22113
  %_0.i3148 = fmul float %history.i.i32.sroa.32.08558, %_151.i.i.i.i444, !dbg !22115
  %_0.i2712 = fadd float %_0.i2716, %_0.i3148, !dbg !22117
  %_0.i3147 = fmul float %history.i.i32.sroa.32.08558, %_154.i.i.i.i447, !dbg !22119
  %_0.i2711 = fadd float %_0.i2715, %_0.i3147, !dbg !22121
  %_0.i3146 = fmul float %history.i.i32.sroa.32.08558, %_157.i.i.i.i450, !dbg !22123
  %_0.i2710 = fadd float %_0.i2714, %_0.i3146, !dbg !22125
  %_0.i3145 = fmul float %history.i.i32.sroa.32.08558, %_160.i.i.i.i453, !dbg !22127
  %_0.i2709 = fadd float %_0.i2713, %_0.i3145, !dbg !22129
  %_0.i3144 = fmul float %history.i.i32.sroa.35.08559, %_165.i.i.i.i458, !dbg !22131
  %_0.i2708 = fadd float %_0.i2712, %_0.i3144, !dbg !22133
  %_0.i3143 = fmul float %history.i.i32.sroa.35.08559, %_168.i.i.i.i461, !dbg !22135
  %_0.i2707 = fadd float %_0.i2711, %_0.i3143, !dbg !22137
  %_0.i3142 = fmul float %history.i.i32.sroa.35.08559, %_171.i.i.i.i464, !dbg !22139
  %_0.i2706 = fadd float %_0.i2710, %_0.i3142, !dbg !22141
  %_0.i3141 = fmul float %history.i.i32.sroa.35.08559, %_174.i.i.i.i467, !dbg !22143
  %_0.i2705 = fadd float %_0.i2709, %_0.i3141, !dbg !22145
  %610 = tail call noundef float @llvm.fabs.f32(float %_0.i2708), !dbg !22147
  %_3.i.i4181.inv = fcmp ogt float %609, %610, !dbg !22149
  %_4.i.i4188.v = select i1 %_3.i.i4181.inv, float %609, float %610, !dbg !22149
  %611 = tail call noundef float @llvm.fabs.f32(float %_0.i2707), !dbg !22147
  %_3.i.i4181.inv.1 = fcmp ogt float %_4.i.i4188.v, %611, !dbg !22149
  %_4.i.i4188.v.1 = select i1 %_3.i.i4181.inv.1, float %_4.i.i4188.v, float %611, !dbg !22149
  %612 = tail call noundef float @llvm.fabs.f32(float %_0.i2706), !dbg !22147
  %_3.i.i4181.inv.2 = fcmp ogt float %_4.i.i4188.v.1, %612, !dbg !22149
  %_4.i.i4188.v.2 = select i1 %_3.i.i4181.inv.2, float %_4.i.i4188.v.1, float %612, !dbg !22149
  %613 = tail call noundef float @llvm.fabs.f32(float %_0.i2705), !dbg !22147
  %_3.i.i4181.inv.3 = fcmp ogt float %_4.i.i4188.v.2, %613, !dbg !22149
  %_4.i.i4188.v.3 = select i1 %_3.i.i4181.inv.3, float %_4.i.i4188.v.2, float %613, !dbg !22149
  %614 = add nuw nsw i32 %iter.i.i28.sroa.16.08560, 1, !dbg !22152
  %data.i4.i4739 = getelementptr inbounds nuw float, ptr %peaks_right.i52, i32 %iter.i.i28.sroa.16.08560, !dbg !22153
  store float %_4.i.i4188.v.3, ptr %data.i4.i4739, align 4, !dbg !22156, !alias.scope !22158, !noalias !21950
  %exitcond12535.not = icmp eq i32 %614, %umax12534, !dbg !21929
  br i1 %exitcond12535.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i478, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544, !dbg !21929

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i478: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4725
  %history.i.i32.sroa.0.0.lcssa = phi float [ %history.i.i32.sroa.0.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4725 ], [ %_0.i3542, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544 ], !dbg !22161
  %history.i.i32.sroa.7.0.lcssa = phi float [ %history.i.i32.sroa.7.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4725 ], [ %history.i.i32.sroa.0.08549, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544 ], !dbg !22161
  %history.i.i32.sroa.10.0.lcssa = phi float [ %history.i.i32.sroa.10.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4725 ], [ %history.i.i32.sroa.7.08550, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544 ], !dbg !22161
  %history.i.i32.sroa.13.0.lcssa = phi float [ %history.i.i32.sroa.13.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4725 ], [ %history.i.i32.sroa.10.08551, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544 ], !dbg !22161
  %history.i.i32.sroa.16.0.lcssa = phi float [ %history.i.i32.sroa.16.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4725 ], [ %history.i.i32.sroa.13.08552, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544 ], !dbg !22161
  %history.i.i32.sroa.19.0.lcssa = phi float [ %history.i.i32.sroa.19.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4725 ], [ %history.i.i32.sroa.16.08553, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544 ], !dbg !22161
  %history.i.i32.sroa.22.0.lcssa = phi float [ %history.i.i32.sroa.22.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4725 ], [ %history.i.i32.sroa.19.08554, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544 ], !dbg !22161
  %history.i.i32.sroa.26.0.lcssa = phi float [ %history.i.i32.sroa.26.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4725 ], [ %history.i.i32.sroa.22.08555, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544 ], !dbg !22161
  %history.i.i32.sroa.29.0.lcssa = phi float [ %history.i.i32.sroa.29.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4725 ], [ %history.i.i32.sroa.26.08556, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544 ], !dbg !22161
  %history.i.i32.sroa.32.0.lcssa = phi float [ %history.i.i32.sroa.32.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4725 ], [ %history.i.i32.sroa.29.08557, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544 ], !dbg !22161
  %history.i.i32.sroa.35.0.lcssa = phi float [ %history.i.i32.sroa.35.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4725 ], [ %history.i.i32.sroa.32.08558, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544 ], !dbg !22161
  %history.i.i32.sroa.38.0.lcssa = phi float [ %history.i.i32.sroa.38.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4725 ], [ %history.i.i32.sroa.35.08559, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3544 ], !dbg !22161
  store float %history.i.i32.sroa.0.0.lcssa, ptr %hot_right.i54, align 4, !dbg !22162, !noalias !21926
  store float %history.i.i32.sroa.7.0.lcssa, ptr %history.i.i32.sroa.7.0.hot_right.i54.sroa_idx, align 4, !dbg !22162, !noalias !21926
  store float %history.i.i32.sroa.10.0.lcssa, ptr %history.i.i32.sroa.10.0.hot_right.i54.sroa_idx, align 4, !dbg !22162, !noalias !21926
  store float %history.i.i32.sroa.13.0.lcssa, ptr %history.i.i32.sroa.13.0.hot_right.i54.sroa_idx, align 4, !dbg !22162, !noalias !21926
  store float %history.i.i32.sroa.16.0.lcssa, ptr %history.i.i32.sroa.16.0.hot_right.i54.sroa_idx, align 4, !dbg !22162, !noalias !21926
  store float %history.i.i32.sroa.19.0.lcssa, ptr %history.i.i32.sroa.19.0.hot_right.i54.sroa_idx, align 4, !dbg !22162, !noalias !21926
  store float %history.i.i32.sroa.22.0.lcssa, ptr %history.i.i32.sroa.22.0.hot_right.i54.sroa_idx, align 4, !dbg !22162, !noalias !21926
  store float %history.i.i32.sroa.26.0.lcssa, ptr %history.i.i32.sroa.26.0.hot_right.i54.sroa_idx, align 4, !dbg !22162, !noalias !21926
  store float %history.i.i32.sroa.29.0.lcssa, ptr %history.i.i32.sroa.29.0.hot_right.i54.sroa_idx, align 4, !dbg !22162, !noalias !21926
  store float %history.i.i32.sroa.32.0.lcssa, ptr %history.i.i32.sroa.32.0.hot_right.i54.sroa_idx, align 4, !dbg !22162, !noalias !21926
  store float %history.i.i32.sroa.35.0.lcssa, ptr %history.i.i32.sroa.35.0.hot_right.i54.sroa_idx, align 4, !dbg !22162, !noalias !21926
  store float %history.i.i32.sroa.38.0.lcssa, ptr %history.i.i32.sroa.38.0.hot_right.i54.sroa_idx, align 4, !dbg !22162, !noalias !21926
  br i1 %_2.i46838521.not, label %bb15.i72.loopexit, label %bb20.i484.lr.ph, !dbg !21636

bb20.i484.lr.ph:                                  ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i478
  %_72.i48.sroa.3.0.copyload = load i32, ptr %_72.i48.sroa.3.0..sroa_idx, align 4, !noalias !21580
  %_72.i48.sroa.4.0.copyload = load i32, ptr %_72.i48.sroa.4.0..sroa_idx, align 4, !noalias !21580
  %_73.i47.sroa.3.0.copyload = load i32, ptr %_73.i47.sroa.3.0..sroa_idx, align 4, !noalias !21580
  %_73.i47.sroa.4.0.copyload = load i32, ptr %_73.i47.sroa.4.0..sroa_idx, align 4, !noalias !21580
  %_54.0.i250.i550 = load ptr, ptr %uniform_left.i51, align 4, !nonnull !10, !align !10173
  %_54.1.i251.i551 = load i32, ptr %584, align 4
  %_18.i261.i561 = load i32, ptr %570, align 4
  %_29.i17018573.not = icmp eq i32 %_18.i261.i561, 0
  %_56.0.i272.i572 = load ptr, ptr %585, align 4, !nonnull !10, !align !10173
  %_56.1.i273.i573 = load i32, ptr %586, align 4
  %_58.1.i298.i598 = load i32, ptr %590, align 4
  %_58.0.i297.i597 = load ptr, ptr %591, align 4, !nonnull !10, !align !10173
  %_54.0.i.i625 = load ptr, ptr %uniform_right.i50, align 4, !nonnull !10, !align !10173
  %_54.1.i.i626 = load i32, ptr %592, align 4
  %_18.i.i636 = load i32, ptr %571, align 4
  %_29.i16798577.not = icmp eq i32 %_18.i.i636, 0
  %_56.0.i.i647 = load ptr, ptr %593, align 4, !nonnull !10, !align !10173
  %_56.1.i.i648 = load i32, ptr %594, align 4
  %_58.1.i.i673 = load i32, ptr %598, align 4
  %_58.0.i.i672 = load ptr, ptr %599, align 4, !nonnull !10, !align !10173
  %_22.i265.i565.promoted9166 = load i32, ptr %_22.i265.i565, align 4
  %_21.i264.i564.promoted9202 = load float, ptr %_21.i264.i564, align 4
  %_22.i.i640.promoted9238 = load i32, ptr %_22.i.i640, align 4
  %_21.i.i639.promoted9274 = load float, ptr %_21.i.i639, align 4
  %_13.i244953695371 = load float, ptr %574, align 4
  %_13.i243753725374 = load float, ptr %577, align 4
  %_13.i242553755377 = load float, ptr %580, align 4
  %_13.i241353785380 = load float, ptr %583, align 4
  %_37.i285.i585 = load float, ptr %588, align 4
  %_37.i.i660 = load float, ptr %596, align 4
  %.promoted14660 = load float, ptr %572, align 4
  %_114.i525.promoted14678 = load float, ptr %_114.i525, align 4
  %.promoted14696 = load float, ptr %573, align 4
  %.promoted14714 = load float, ptr %575, align 4
  %_115.i526.promoted14732 = load float, ptr %_115.i526, align 4
  %.promoted14750 = load float, ptr %576, align 4
  %.promoted14768 = load float, ptr %578, align 4
  %_119.i527.promoted14786 = load float, ptr %_119.i527, align 4
  %.promoted14804 = load float, ptr %579, align 4
  %.promoted14822 = load float, ptr %581, align 4
  %_120.i528.promoted14840 = load float, ptr %_120.i528, align 4
  %.promoted14858 = load float, ptr %582, align 4
  %.promoted14876 = load float, ptr %587, align 4
  %.promoted14894 = load float, ptr %589, align 4
  %.promoted14912 = load float, ptr %595, align 4
  %.promoted14930 = load float, ptr %597, align 4
  br label %bb20.i484, !dbg !21636

bb20.i484:                                        ; preds = %bb20.i484.lr.ph, %bb74.i692
  %_0.i3740.lcssa1328314932 = phi float [ %.promoted14930, %bb20.i484.lr.ph ], [ %_0.i3740.lcssa1328314931, %bb74.i692 ]
  %_0.i3366.lcssa1328214914 = phi float [ %.promoted14912, %bb20.i484.lr.ph ], [ %_0.i3366.lcssa1328214913, %bb74.i692 ]
  %_0.i3744.lcssa1325814896 = phi float [ %.promoted14894, %bb20.i484.lr.ph ], [ %_0.i3744.lcssa1325814895, %bb74.i692 ]
  %_0.i3370.lcssa1324214878 = phi float [ %.promoted14876, %bb20.i484.lr.ph ], [ %_0.i3370.lcssa1324214877, %bb74.i692 ]
  %_0.i3795.lcssa1305714860 = phi float [ %.promoted14858, %bb20.i484.lr.ph ], [ %_0.i3795.lcssa1305714859, %bb74.i692 ]
  %_0.i3802.lcssa1307114842 = phi float [ %_120.i528.promoted14840, %bb20.i484.lr.ph ], [ %_0.i3802.lcssa1307114841, %bb74.i692 ]
  %_0.i.i4036.lcssa1308514824 = phi float [ %.promoted14822, %bb20.i484.lr.ph ], [ %_0.i.i4036.lcssa1308514823, %bb74.i692 ]
  %_0.i3782.lcssa1309914806 = phi float [ %.promoted14804, %bb20.i484.lr.ph ], [ %_0.i3782.lcssa1309914805, %bb74.i692 ]
  %_0.i3789.lcssa1311314788 = phi float [ %_119.i527.promoted14786, %bb20.i484.lr.ph ], [ %_0.i3789.lcssa1311314787, %bb74.i692 ]
  %_0.i.i4029.lcssa1312714770 = phi float [ %.promoted14768, %bb20.i484.lr.ph ], [ %_0.i.i4029.lcssa1312714769, %bb74.i692 ]
  %_0.i3769.lcssa1314114752 = phi float [ %.promoted14750, %bb20.i484.lr.ph ], [ %_0.i3769.lcssa1314114751, %bb74.i692 ]
  %_0.i3776.lcssa1315514734 = phi float [ %_115.i526.promoted14732, %bb20.i484.lr.ph ], [ %_0.i3776.lcssa1315514733, %bb74.i692 ]
  %_0.i.i4022.lcssa1316914716 = phi float [ %.promoted14714, %bb20.i484.lr.ph ], [ %_0.i.i4022.lcssa1316914715, %bb74.i692 ]
  %_0.i3757.lcssa1318314698 = phi float [ %.promoted14696, %bb20.i484.lr.ph ], [ %_0.i3757.lcssa1318314697, %bb74.i692 ]
  %_0.i3763.lcssa1319714680 = phi float [ %_114.i525.promoted14678, %bb20.i484.lr.ph ], [ %_0.i3763.lcssa1319714679, %bb74.i692 ]
  %_0.i.i.lcssa1321114662 = phi float [ %.promoted14660, %bb20.i484.lr.ph ], [ %_0.i.i.lcssa1321114661, %bb74.i692 ]
  %running.sroa.0.0.i.lcssa90979276 = phi float [ %_21.i.i639.promoted9274, %bb20.i484.lr.ph ], [ %running.sroa.0.0.i.lcssa90979275, %bb74.i692 ]
  %storemerge.i.lcssa90709240 = phi i32 [ %_22.i.i640.promoted9238, %bb20.i484.lr.ph ], [ %storemerge.i.lcssa90709239, %bb74.i692 ]
  %running.sroa.0.0.i1689.lcssa89859204 = phi float [ %_21.i264.i564.promoted9202, %bb20.i484.lr.ph ], [ %running.sroa.0.0.i1689.lcssa89859203, %bb74.i692 ]
  %storemerge.i1694.lcssa89589168 = phi i32 [ %_22.i265.i565.promoted9166, %bb20.i484.lr.ph ], [ %storemerge.i1694.lcssa89589167, %bb74.i692 ]
  %frame.sroa.0.0.i4829163 = phi i32 [ 0, %bb20.i484.lr.ph ], [ %_87.i497, %bb74.i692 ]
  %main_cursor.sroa.0.1.i4819162 = phi i32 [ %main_cursor.sroa.0.0.i749312, %bb20.i484.lr.ph ], [ %main_cursor.sroa.0.2.i698, %bb74.i692 ]
  %ring_cursor.sroa.0.1.i4809161 = phi i32 [ %ring_cursor.sroa.0.0.i739311, %bb20.i484.lr.ph ], [ %ring_cursor.sroa.0.2.i695, %bb74.i692 ]
  %_69.i485 = sub nuw nsw i32 %spec.store.select.i79, %frame.sroa.0.0.i4829163, !dbg !22163
  %ring.i2311 = load i32, ptr %530, align 4, !dbg !22164, !alias.scope !22166, !noalias !22169, !noundef !10
  %main.i2312 = load i32, ptr %531, align 4, !dbg !22173, !alias.scope !22166, !noalias !22169, !noundef !10
  %_10.i = add i32 %ring_cursor.sroa.0.1.i4809161, 1, !dbg !22174
  %_38.not.i = icmp ult i32 %_10.i, %ring.i2311, !dbg !22175
  %615 = select i1 %_38.not.i, i32 0, i32 %ring.i2311, !dbg !22175
  %start1.sroa.0.0.i2313 = sub nuw i32 %_10.i, %615, !dbg !22175
  %_12.i2314 = add i32 %_72.i48.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i4809161, !dbg !22177
  %_39.not.i = icmp ult i32 %_12.i2314, %ring.i2311, !dbg !22178
  %616 = select i1 %_39.not.i, i32 0, i32 %ring.i2311, !dbg !22178
  %left_end.sroa.0.0.i = sub nuw i32 %_12.i2314, %616, !dbg !22178
  %_15.i2315 = add i32 %_73.i47.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i4809161, !dbg !22180
  %_40.not.i = icmp ult i32 %_15.i2315, %ring.i2311, !dbg !22181
  %617 = select i1 %_40.not.i, i32 0, i32 %ring.i2311, !dbg !22181
  %right_end.sroa.0.0.i = sub nuw i32 %_15.i2315, %617, !dbg !22181
  %_18.i = add i32 %_72.i48.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i4809161, !dbg !22183
  %_41.not.i = icmp ult i32 %_18.i, %ring.i2311, !dbg !22184
  %618 = select i1 %_41.not.i, i32 0, i32 %ring.i2311, !dbg !22184
  %left_expiring.sroa.0.0.i = sub nuw i32 %_18.i, %618, !dbg !22184
  %_21.i = add i32 %_73.i47.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i4809161, !dbg !22186
  %_42.not.i = icmp ult i32 %_21.i, %ring.i2311, !dbg !22187
  %619 = select i1 %_42.not.i, i32 0, i32 %ring.i2311, !dbg !22187
  %right_expiring.sroa.0.0.i = sub nuw i32 %_21.i, %619, !dbg !22187
  %620 = sub i32 %ring.i2311, %ring_cursor.sroa.0.1.i4809161, !dbg !22189
  %spec.store.select.i2316 = tail call i32 @llvm.umin.i32(i32 %620, i32 %_69.i485), !dbg !22190
  %621 = sub i32 %main.i2312, %main_cursor.sroa.0.1.i4819162, !dbg !22192
  %_24.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %621, i32 %spec.store.select.i2316), !dbg !22193
  %622 = sub i32 %ring.i2311, %start1.sroa.0.0.i2313, !dbg !22195
  %_25.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %622, i32 %_24.sroa.0.0.i), !dbg !22196
  %623 = sub i32 %ring.i2311, %left_end.sroa.0.0.i, !dbg !22198
  %_27.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %623, i32 %_25.sroa.0.0.i), !dbg !22199
  %624 = sub i32 %ring.i2311, %right_end.sroa.0.0.i, !dbg !22201
  %_29.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %624, i32 %_27.sroa.0.0.i), !dbg !22202
  %625 = sub i32 %ring.i2311, %left_expiring.sroa.0.0.i, !dbg !22204
  %_31.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %625, i32 %_29.sroa.0.0.i), !dbg !22205
  %626 = sub i32 %ring.i2311, %right_expiring.sroa.0.0.i, !dbg !22207
  %run.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %626, i32 %_31.sroa.0.0.i), !dbg !22208
  %_76.i487 = add i32 %frame.sroa.0.0.i4829163, %iter1.sroa.0.0.i759313, !dbg !22210
  %_80.i488 = add i32 %run.sroa.0.0.i, %_76.i487, !dbg !22213
  %_203.i489 = icmp ult i32 %_80.i488, %_76.i487, !dbg !22216
  %_199.not.i490 = icmp ugt i32 %_80.i488, %left_io.1
  %or.cond27.i491 = or i1 %_203.i489, %_199.not.i490, !dbg !22216
  br i1 %or.cond27.i491, label %bb58.i700, label %bb57.i492, !dbg !22216, !prof !4596

bb58.i700:                                        ; preds = %bb20.i484
  store float %_0.i.i.lcssa1321114662, ptr %572, align 4
  store float %_0.i3763.lcssa1319714680, ptr %_114.i525, align 4
  store float %_0.i3757.lcssa1318314698, ptr %573, align 4
  store float %_0.i.i4022.lcssa1316914716, ptr %575, align 4
  store float %_0.i3776.lcssa1315514734, ptr %_115.i526, align 4
  store float %_0.i3769.lcssa1314114752, ptr %576, align 4
  store float %_0.i.i4029.lcssa1312714770, ptr %578, align 4
  store float %_0.i3789.lcssa1311314788, ptr %_119.i527, align 4
  store float %_0.i3782.lcssa1309914806, ptr %579, align 4
  store float %_0.i.i4036.lcssa1308514824, ptr %581, align 4
  store float %_0.i3802.lcssa1307114842, ptr %_120.i528, align 4
  store float %_0.i3795.lcssa1305714860, ptr %582, align 4
  store float %_0.i3370.lcssa1324214878, ptr %587, align 4
  store float %_0.i3744.lcssa1325814896, ptr %589, align 4
  store float %_0.i3366.lcssa1328214914, ptr %595, align 4
  store float %_0.i3740.lcssa1328314932, ptr %597, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_76.i487, i32 noundef %_80.i488, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb67056a871aedf25dd2ba0a06d720f) #32, !dbg !22224, !noalias !21574
  unreachable, !dbg !22224

bb57.i492:                                        ; preds = %bb20.i484
  %_206.i493 = getelementptr inbounds nuw float, ptr %left_io.0, i32 %_76.i487, !dbg !22225
  %_207.not.i494 = icmp ugt i32 %_80.i488, %right_io.1, !dbg !22229
  br i1 %_207.not.i494, label %bb61.i699, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit, !dbg !22229, !prof !787

bb61.i699:                                        ; preds = %bb57.i492
  store float %_0.i.i.lcssa1321114662, ptr %572, align 4
  store float %_0.i3763.lcssa1319714680, ptr %_114.i525, align 4
  store float %_0.i3757.lcssa1318314698, ptr %573, align 4
  store float %_0.i.i4022.lcssa1316914716, ptr %575, align 4
  store float %_0.i3776.lcssa1315514734, ptr %_115.i526, align 4
  store float %_0.i3769.lcssa1314114752, ptr %576, align 4
  store float %_0.i.i4029.lcssa1312714770, ptr %578, align 4
  store float %_0.i3789.lcssa1311314788, ptr %_119.i527, align 4
  store float %_0.i3782.lcssa1309914806, ptr %579, align 4
  store float %_0.i.i4036.lcssa1308514824, ptr %581, align 4
  store float %_0.i3802.lcssa1307114842, ptr %_120.i528, align 4
  store float %_0.i3795.lcssa1305714860, ptr %582, align 4
  store float %_0.i3370.lcssa1324214878, ptr %587, align 4
  store float %_0.i3744.lcssa1325814896, ptr %589, align 4
  store float %_0.i3366.lcssa1328214914, ptr %595, align 4
  store float %_0.i3740.lcssa1328314932, ptr %597, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_76.i487, i32 noundef %_80.i488, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1d0fce16c93a3aa07bd2f89733f587ef) #32, !dbg !22234, !noalias !21574
  unreachable, !dbg !22234

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit: ; preds = %bb57.i492
  %_212.i496 = getelementptr inbounds nuw float, ptr %right_io.0, i32 %_76.i487, !dbg !22235
  %_87.i497 = add nuw nsw i32 %run.sroa.0.0.i, %frame.sroa.0.0.i4829163, !dbg !22239
  %_221.i503 = getelementptr inbounds nuw float, ptr %peaks_left.i53, i32 %frame.sroa.0.0.i4829163, !dbg !22241
  %_230.i504 = getelementptr inbounds nuw float, ptr %peaks_right.i52, i32 %frame.sroa.0.0.i4829163, !dbg !22250
  %_2.i47878581.not = icmp eq i32 %run.sroa.0.0.i, 0, !dbg !22260
  br i1 %_2.i47878581.not, label %bb74.i692, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524.lr.ph, !dbg !22260

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524.lr.ph: ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit
  %umax12538 = call i32 @llvm.umax.i32(i32 %ring_cursor.sroa.0.1.i4809161, i32 %_54.1.i251.i551), !dbg !22260
  %umax12539 = call i32 @llvm.umax.i32(i32 %main_cursor.sroa.0.1.i4819162, i32 %_58.1.i298.i598), !dbg !22260
  %627 = sub i32 %umax12538, %ring_cursor.sroa.0.1.i4809161, !dbg !22260
  %628 = sub i32 %umax12539, %main_cursor.sroa.0.1.i4819162, !dbg !22260
  %umin12542 = call i32 @llvm.umin.i32(i32 %623, i32 %624), !dbg !22260
  %umin12543 = call i32 @llvm.umin.i32(i32 %umin12542, i32 %625), !dbg !22260
  %umin12544 = call i32 @llvm.umin.i32(i32 %umin12543, i32 %626), !dbg !22260
  %umin12545 = call i32 @llvm.umin.i32(i32 %umin12544, i32 %622), !dbg !22260
  %umin12546 = call i32 @llvm.umin.i32(i32 %umin12545, i32 %620), !dbg !22260
  %umin12547 = call i32 @llvm.umin.i32(i32 %umin12546, i32 %621), !dbg !22260
  %629 = sub nsw i32 %umin12548, %frame.sroa.0.0.i4829163, !dbg !22260
  %umin12549 = call i32 @llvm.umin.i32(i32 %umin12547, i32 %629), !dbg !22260
  br label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524, !dbg !22260

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673
  %_0.i37409132 = phi float [ %_0.i3740.lcssa1328314932, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524.lr.ph ], [ %_0.i3740, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %_0.i33669103 = phi float [ %_0.i3366.lcssa1328214914, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524.lr.ph ], [ %_0.i3366, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %running.sroa.0.0.i9075 = phi float [ %running.sroa.0.0.i.lcssa90979276, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524.lr.ph ], [ %running.sroa.0.0.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %storemerge.i9048 = phi i32 [ %storemerge.i.lcssa90709240, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524.lr.ph ], [ %storemerge.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %_0.i37449020 = phi float [ %_0.i3744.lcssa1325814896, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524.lr.ph ], [ %_0.i3744, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %_0.i33708991 = phi float [ %_0.i3370.lcssa1324214878, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524.lr.ph ], [ %_0.i3370, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %running.sroa.0.0.i16898963 = phi float [ %running.sroa.0.0.i1689.lcssa89859204, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524.lr.ph ], [ %running.sroa.0.0.i1689, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %storemerge.i16948936 = phi i32 [ %storemerge.i1694.lcssa89589168, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524.lr.ph ], [ %storemerge.i1694, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %_12.i24118907 = phi float [ %_0.i3795.lcssa1305714860, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524.lr.ph ], [ %_0.i3795, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ], !dbg !22269
  %_0.i38028878 = phi float [ %_0.i3802.lcssa1307114842, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524.lr.ph ], [ %_0.i3802, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ], !dbg !22269
  %_5.i24058849 = phi float [ %_0.i.i4036.lcssa1308514824, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524.lr.ph ], [ %_0.i.i4036, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ], !dbg !22269
  %_12.i24238819 = phi float [ %_0.i3782.lcssa1309914806, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524.lr.ph ], [ %_0.i3782, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ], !dbg !22269
  %_0.i37898790 = phi float [ %_0.i3789.lcssa1311314788, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524.lr.ph ], [ %_0.i3789, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ], !dbg !22269
  %_5.i24178761 = phi float [ %_0.i.i4029.lcssa1312714770, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524.lr.ph ], [ %_0.i.i4029, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ], !dbg !22269
  %_12.i24358731 = phi float [ %_0.i3769.lcssa1314114752, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524.lr.ph ], [ %_0.i3769, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ], !dbg !22269
  %_0.i37768702 = phi float [ %_0.i3776.lcssa1315514734, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524.lr.ph ], [ %_0.i3776, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ], !dbg !22269
  %_5.i24298673 = phi float [ %_0.i.i4022.lcssa1316914716, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524.lr.ph ], [ %_0.i.i4022, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ], !dbg !22269
  %_12.i24478643 = phi float [ %_0.i3757.lcssa1318314698, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524.lr.ph ], [ %_0.i3757, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ], !dbg !22269
  %_0.i37638614 = phi float [ %_0.i3763.lcssa1319714680, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524.lr.ph ], [ %_0.i3763, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ], !dbg !22269
  %_5.i24418585 = phi float [ %_0.i.i.lcssa1321114662, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524.lr.ph ], [ %_0.i.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ], !dbg !22269
  %iter.i38.sroa.41.08583 = phi i32 [ 0, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524.lr.ph ], [ %_235.0.i524, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %_235.0.i524 = add nuw i32 %iter.i38.sroa.41.08583, 1, !dbg !22270
  %data.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_206.i493, i32 %iter.i38.sroa.41.08583, !dbg !22273
  %data.i5.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_212.i496, i32 %iter.i38.sroa.41.08583, !dbg !22280
  %data.i.i.i.i = getelementptr inbounds nuw float, ptr %_221.i503, i32 %iter.i38.sroa.41.08583, !dbg !22283
  %data.i.i4796 = getelementptr inbounds nuw float, ptr %_230.i504, i32 %iter.i38.sroa.41.08583, !dbg !22286
  %_0.i3339 = fadd float %_5.i24418585, -1.000000e+00, !dbg !22289
  %_3.i.i = fcmp ogt float %_0.i3339, 0.000000e+00, !dbg !22294
  %_0.i.i = select i1 %_3.i.i, float %_0.i3339, float 0.000000e+00, !dbg !22297
  %_0.i2499 = fadd float %_0.i37638614, %_12.i24478643, !dbg !22299
  %_0.i3763 = select i1 %_3.i.i, float %_0.i2499, float %_13.i244953695371, !dbg !22301
  %_0.i3757 = select i1 %_3.i.i, float %_12.i24478643, float 0.000000e+00, !dbg !22303
  %_0.i3340 = fadd float %_5.i24298673, -1.000000e+00, !dbg !22305
  %_3.i.i4016 = fcmp ogt float %_0.i3340, 0.000000e+00, !dbg !22308
  %_0.i.i4022 = select i1 %_3.i.i4016, float %_0.i3340, float 0.000000e+00, !dbg !22311
  %_0.i2500 = fadd float %_0.i37768702, %_12.i24358731, !dbg !22313
  %_0.i3776 = select i1 %_3.i.i4016, float %_0.i2500, float %_13.i243753725374, !dbg !22315
  %_0.i3769 = select i1 %_3.i.i4016, float %_12.i24358731, float 0.000000e+00, !dbg !22317
  %_0.i3341 = fadd float %_5.i24178761, -1.000000e+00, !dbg !22319
  %_3.i.i4023 = fcmp ogt float %_0.i3341, 0.000000e+00, !dbg !22324
  %_0.i.i4029 = select i1 %_3.i.i4023, float %_0.i3341, float 0.000000e+00, !dbg !22327
  %_0.i2501 = fadd float %_0.i37898790, %_12.i24238819, !dbg !22329
  %_0.i3789 = select i1 %_3.i.i4023, float %_0.i2501, float %_13.i242553755377, !dbg !22331
  %_0.i3782 = select i1 %_3.i.i4023, float %_12.i24238819, float 0.000000e+00, !dbg !22333
  %_0.i3342 = fadd float %_5.i24058849, -1.000000e+00, !dbg !22335
  %_3.i.i4030 = fcmp ogt float %_0.i3342, 0.000000e+00, !dbg !22338
  %_0.i.i4036 = select i1 %_3.i.i4030, float %_0.i3342, float 0.000000e+00, !dbg !22341
  %_0.i2502 = fadd float %_0.i38028878, %_12.i24118907, !dbg !22343
  %_0.i3802 = select i1 %_3.i.i4030, float %_0.i2502, float %_13.i241353785380, !dbg !22345
  %_0.i3795 = select i1 %_3.i.i4030, float %_12.i24118907, float 0.000000e+00, !dbg !22347
  %_0.i3537 = load float, ptr %data.i.i.i.i, align 4, !dbg !22349, !alias.scope !22352, !noalias !21574, !noundef !10
  %_0.i3532 = load float, ptr %data.i.i4796, align 4, !dbg !22355, !alias.scope !22358, !noalias !21574, !noundef !10
  %_3.i.i4172 = fcmp ule float %_0.i3532, %_0.i3537, !dbg !22361
  %_6.i.i4174 = bitcast float %_0.i3532 to i32, !dbg !22365
  %_8.i.i4176 = bitcast float %_0.i3537 to i32, !dbg !22368
  %_4.i.i4179 = select i1 %_3.i.i4172, i32 %_8.i.i4176, i32 %_6.i.i4174, !dbg !22370
  %_5.i3968 = and i32 %_4.i.i4179, %.none.i60, !dbg !22371
  %_7.i3964 = and i32 %_9.i3970, %_6.i.i4174, !dbg !22374
  %_4.i3965 = or disjoint i32 %_5.i3968, %_7.i3964, !dbg !22377
  %_0.i3966 = bitcast i32 %_4.i3965 to float, !dbg !22378
  %_0.i3527 = load float, ptr %data.i.i.i.i.i.i, align 4, !dbg !22380, !alias.scope !22383, !noalias !21574, !noundef !10
  %_0.i3522 = load float, ptr %data.i5.i.i.i.i.i, align 4, !dbg !22386, !alias.scope !22389, !noalias !21574, !noundef !10
  %_242.i541 = add nuw i32 %iter.i38.sroa.41.08583, %ring_cursor.sroa.0.1.i4809161, !dbg !22392
  %_243.i542 = add nuw i32 %iter.i38.sroa.41.08583, %main_cursor.sroa.0.1.i4819162, !dbg !22396
  %_244.i543 = add nuw i32 %iter.i38.sroa.41.08583, %left_end.sroa.0.0.i, !dbg !22397
  %_245.i544 = add i32 %iter.i38.sroa.41.08583, %start1.sroa.0.0.i2313, !dbg !22398
  %_246.i545 = add nuw i32 %iter.i38.sroa.41.08583, %left_expiring.sroa.0.0.i, !dbg !22399
  %_7.i8.i253.i553 = add i32 %_242.i541, 1, !dbg !22400
  %exitcond12540.not = icmp eq i32 %iter.i38.sroa.41.08583, %627, !dbg !22408
  br i1 %exitcond12540.not, label %bb4.i13.i312.i691, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i257.i557, !dbg !22408, !prof !4596

bb4.i13.i312.i691:                                ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524
  store float %_0.i.i.lcssa1321114662, ptr %572, align 4
  store float %_0.i3763.lcssa1319714680, ptr %_114.i525, align 4
  store float %_0.i3757.lcssa1318314698, ptr %573, align 4
  store float %_0.i.i4022.lcssa1316914716, ptr %575, align 4
  store float %_0.i3776.lcssa1315514734, ptr %_115.i526, align 4
  store float %_0.i3769.lcssa1314114752, ptr %576, align 4
  store float %_0.i.i4029.lcssa1312714770, ptr %578, align 4
  store float %_0.i3789.lcssa1311314788, ptr %_119.i527, align 4
  store float %_0.i3782.lcssa1309914806, ptr %579, align 4
  store float %_0.i.i4036.lcssa1308514824, ptr %581, align 4
  store float %_0.i3802.lcssa1307114842, ptr %_120.i528, align 4
  store float %_0.i3795.lcssa1305714860, ptr %582, align 4
  store float %_0.i3370.lcssa1324214878, ptr %587, align 4
  store float %_0.i3744.lcssa1325814896, ptr %589, align 4
  store float %_0.i3366.lcssa1328214914, ptr %595, align 4
  store float %_0.i3740.lcssa1328314932, ptr %597, align 4
  %630 = add i32 %umax12538, 1, !dbg !22260
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_242.i541, i32 noundef %630, i32 noundef range(i32 0, 536870912) %_54.1.i251.i551, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #32, !dbg !22415, !noalias !22416
  unreachable, !dbg !22415

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i257.i557: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524
  %_7.i3971 = and i32 %_9.i3970, %_8.i.i4176, !dbg !22424
  %_4.i3972 = or disjoint i32 %_5.i3968, %_7.i3971, !dbg !22371
  %_0.i3973 = bitcast i32 %_4.i3972 to float, !dbg !22425
  %_0.i2910 = fdiv float %_0.i3763, %_0.i3973, !dbg !22427
  %_3.i2477 = fcmp uge float %_0.i3763, %_0.i3973, !dbg !22429
  %_0.i3959 = select i1 %_3.i2477, float 1.000000e+00, float %_0.i2910, !dbg !22431
  %_17.i12.i258.i558 = getelementptr inbounds nuw float, ptr %_54.0.i250.i550, i32 %_242.i541, !dbg !22433
  store float %_0.i3959, ptr %_17.i12.i258.i558, align 4, !dbg !22437, !alias.scope !22439, !noalias !22442
  %or.cond.i1860.not = icmp ult i32 %_244.i543, %_54.1.i251.i551, !dbg !22443
  br i1 %or.cond.i1860.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1864, label %bb4.i1863, !dbg !22443, !prof !10564

bb4.i1863:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i257.i557
  store float %_0.i.i.lcssa1321114662, ptr %572, align 4
  store float %_0.i3763.lcssa1319714680, ptr %_114.i525, align 4
  store float %_0.i3757.lcssa1318314698, ptr %573, align 4
  store float %_0.i.i4022.lcssa1316914716, ptr %575, align 4
  store float %_0.i3776.lcssa1315514734, ptr %_115.i526, align 4
  store float %_0.i3769.lcssa1314114752, ptr %576, align 4
  store float %_0.i.i4029.lcssa1312714770, ptr %578, align 4
  store float %_0.i3789.lcssa1311314788, ptr %_119.i527, align 4
  store float %_0.i3782.lcssa1309914806, ptr %579, align 4
  store float %_0.i.i4036.lcssa1308514824, ptr %581, align 4
  store float %_0.i3802.lcssa1307114842, ptr %_120.i528, align 4
  store float %_0.i3795.lcssa1305714860, ptr %582, align 4
  store float %_0.i3370.lcssa1324214878, ptr %587, align 4
  store float %_0.i3744.lcssa1325814896, ptr %589, align 4
  store float %_0.i3366.lcssa1328214914, ptr %595, align 4
  store float %_0.i3740.lcssa1328314932, ptr %597, align 4
  %_5.i1857 = add i32 %_244.i543, 1, !dbg !22455
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_244.i543, i32 noundef %_5.i1857, i32 noundef range(i32 0, 536870912) %_54.1.i251.i551, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !22456, !noalias !22457
  unreachable, !dbg !22456

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1864: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i257.i557
  %_15.i1861 = getelementptr inbounds nuw float, ptr %_54.0.i250.i550, i32 %_244.i543, !dbg !22463
  %_0.i3404 = load float, ptr %_15.i1861, align 4, !dbg !22467, !alias.scope !22469, !noalias !22472, !noundef !10
  %631 = icmp eq i32 %storemerge.i16948936, 0, !dbg !22473
  %_3.i.i4316.inv = fcmp olt float %running.sroa.0.0.i16898963, %_0.i3404, !dbg !22473
  %_4.i.i4323.v = select i1 %_3.i.i4316.inv, float %running.sroa.0.0.i16898963, float %_0.i3404, !dbg !22473
  %running.sroa.0.0.i1689 = select i1 %631, float %_0.i3404, float %_4.i.i4323.v, !dbg !22473
  %_15.i1690 = add i32 %storemerge.i16948936, 1, !dbg !22476
  %complete.i1691 = icmp eq i32 %_15.i1690, %_18.i261.i561, !dbg !22476
  br i1 %complete.i1691, label %bb11.i1697.preheader, label %bb7.i1692, !dbg !22478

bb11.i1697.preheader:                             ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1864
  br i1 %_29.i17018573.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1713, label %bb19.i1702, !dbg !22480

bb7.i1692:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1864
  %or.cond.i1852.not = icmp ult i32 %_245.i544, %_54.1.i251.i551, !dbg !22490
  br i1 %or.cond.i1852.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1856, label %bb4.i1855, !dbg !22490, !prof !10564

bb4.i1855:                                        ; preds = %bb7.i1692
  store float %_0.i.i.lcssa1321114662, ptr %572, align 4
  store float %_0.i3763.lcssa1319714680, ptr %_114.i525, align 4
  store float %_0.i3757.lcssa1318314698, ptr %573, align 4
  store float %_0.i.i4022.lcssa1316914716, ptr %575, align 4
  store float %_0.i3776.lcssa1315514734, ptr %_115.i526, align 4
  store float %_0.i3769.lcssa1314114752, ptr %576, align 4
  store float %_0.i.i4029.lcssa1312714770, ptr %578, align 4
  store float %_0.i3789.lcssa1311314788, ptr %_119.i527, align 4
  store float %_0.i3782.lcssa1309914806, ptr %579, align 4
  store float %_0.i.i4036.lcssa1308514824, ptr %581, align 4
  store float %_0.i3802.lcssa1307114842, ptr %_120.i528, align 4
  store float %_0.i3795.lcssa1305714860, ptr %582, align 4
  store float %_0.i3370.lcssa1324214878, ptr %587, align 4
  store float %_0.i3744.lcssa1325814896, ptr %589, align 4
  store float %_0.i3366.lcssa1328214914, ptr %595, align 4
  store float %_0.i3740.lcssa1328314932, ptr %597, align 4
  %_5.i1849 = add i32 %_245.i544, 1, !dbg !22495
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_245.i544, i32 noundef %_5.i1849, i32 noundef range(i32 0, 536870912) %_54.1.i251.i551, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !22496, !noalias !22497
  unreachable, !dbg !22496

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1856: ; preds = %bb7.i1692
  %_15.i1853 = getelementptr inbounds nuw float, ptr %_54.0.i250.i550, i32 %_245.i544, !dbg !22500
  %_0.i3406 = load float, ptr %_15.i1853, align 4, !dbg !22502, !alias.scope !22504, !noalias !22472, !noundef !10
  %_3.i.i4307.inv = fcmp olt float %_0.i3406, %running.sroa.0.0.i1689, !dbg !22507
  %_4.i.i4314.v = select i1 %_3.i.i4307.inv, float %_0.i3406, float %running.sroa.0.0.i1689, !dbg !22507
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1713, !dbg !22511

bb19.i1702:                                       ; preds = %bb11.i1697.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1709
  %end.sroa.0.0.i17008576 = phi i32 [ %633, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1709 ], [ %_244.i543, %bb11.i1697.preheader ]
  %suffix.sroa.0.0.i16998575 = phi float [ %_4.i.i4305.v, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1709 ], [ %_0.i3404, %bb11.i1697.preheader ]
  %iter.sroa.0.0.i16988574 = phi i32 [ %_30.i1703, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1709 ], [ 0, %bb11.i1697.preheader ]
  %or.cond.i1836.not = icmp ult i32 %end.sroa.0.0.i17008576, %_54.1.i251.i551, !dbg !22512
  br i1 %or.cond.i1836.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1709, label %bb4.i1839, !dbg !22512, !prof !10564

bb4.i1839:                                        ; preds = %bb19.i1702
  store float %_0.i.i.lcssa1321114662, ptr %572, align 4
  store float %_0.i3763.lcssa1319714680, ptr %_114.i525, align 4
  store float %_0.i3757.lcssa1318314698, ptr %573, align 4
  store float %_0.i.i4022.lcssa1316914716, ptr %575, align 4
  store float %_0.i3776.lcssa1315514734, ptr %_115.i526, align 4
  store float %_0.i3769.lcssa1314114752, ptr %576, align 4
  store float %_0.i.i4029.lcssa1312714770, ptr %578, align 4
  store float %_0.i3789.lcssa1311314788, ptr %_119.i527, align 4
  store float %_0.i3782.lcssa1309914806, ptr %579, align 4
  store float %_0.i.i4036.lcssa1308514824, ptr %581, align 4
  store float %_0.i3802.lcssa1307114842, ptr %_120.i528, align 4
  store float %_0.i3795.lcssa1305714860, ptr %582, align 4
  store float %_0.i3370.lcssa1324214878, ptr %587, align 4
  store float %_0.i3744.lcssa1325814896, ptr %589, align 4
  store float %_0.i3366.lcssa1328214914, ptr %595, align 4
  store float %_0.i3740.lcssa1328314932, ptr %597, align 4
  %_5.i1833 = add i32 %end.sroa.0.0.i17008576, 1, !dbg !22517
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %end.sroa.0.0.i17008576, i32 noundef %_5.i1833, i32 noundef range(i32 0, 536870912) %_54.1.i251.i551, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !22518, !noalias !22519
  unreachable, !dbg !22518

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1709: ; preds = %bb19.i1702
  %_30.i1703 = add nuw i32 %iter.sroa.0.0.i16988574, 1, !dbg !22522
  %_15.i1837 = getelementptr inbounds nuw float, ptr %_54.0.i250.i550, i32 %end.sroa.0.0.i17008576, !dbg !22528
  %_0.i3410 = load float, ptr %_15.i1837, align 4, !dbg !22530, !alias.scope !22532, !noalias !22472, !noundef !10
  %_3.i.i4298.inv = fcmp olt float %suffix.sroa.0.0.i16998575, %_0.i3410, !dbg !22535
  %_4.i.i4305.v = select i1 %_3.i.i4298.inv, float %suffix.sroa.0.0.i16998575, float %_0.i3410, !dbg !22535
  store float %_4.i.i4305.v, ptr %_15.i1837, align 4, !dbg !22538, !alias.scope !22541, !noalias !22472
  %632 = icmp eq i32 %end.sroa.0.0.i17008576, 0, !dbg !22544
  %spec.store.select.i1711 = select i1 %632, i32 %ring.i63, i32 %end.sroa.0.0.i17008576, !dbg !22544
  %633 = add i32 %spec.store.select.i1711, -1, !dbg !22545
  %exitcond12536.not = icmp eq i32 %_30.i1703, %_18.i261.i561, !dbg !22546
  br i1 %exitcond12536.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1713, label %bb19.i1702, !dbg !22480

_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1713: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1709, %bb11.i1697.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1856
  %storemerge.i1694 = phi i32 [ %_15.i1690, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1856 ], [ 0, %bb11.i1697.preheader ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1709 ], !dbg !22549
  %running.sroa.0.1.i1695 = phi float [ %_4.i.i4314.v, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1856 ], [ %running.sroa.0.0.i1689, %bb11.i1697.preheader ], [ %running.sroa.0.0.i1689, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1709 ], !dbg !22550
  %_0.i3140 = fmul float %running.sroa.0.1.i1695, 1.638400e+04, !dbg !22551
  %634 = tail call noundef float @llvm.floor.f32(float %_0.i3140), !dbg !22554
  %_0.i3139 = fmul float %634, 0x3F10000000000000, !dbg !22558
  %or.cond.i1924.not = icmp ult i32 %_246.i545, %_56.1.i273.i573, !dbg !22560
  br i1 %or.cond.i1924.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1928, label %bb4.i1927, !dbg !22560, !prof !10564

bb4.i1927:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1713
  store float %_0.i.i.lcssa1321114662, ptr %572, align 4
  store float %_0.i3763.lcssa1319714680, ptr %_114.i525, align 4
  store float %_0.i3757.lcssa1318314698, ptr %573, align 4
  store float %_0.i.i4022.lcssa1316914716, ptr %575, align 4
  store float %_0.i3776.lcssa1315514734, ptr %_115.i526, align 4
  store float %_0.i3769.lcssa1314114752, ptr %576, align 4
  store float %_0.i.i4029.lcssa1312714770, ptr %578, align 4
  store float %_0.i3789.lcssa1311314788, ptr %_119.i527, align 4
  store float %_0.i3782.lcssa1309914806, ptr %579, align 4
  store float %_0.i.i4036.lcssa1308514824, ptr %581, align 4
  store float %_0.i3802.lcssa1307114842, ptr %_120.i528, align 4
  store float %_0.i3795.lcssa1305714860, ptr %582, align 4
  store float %_0.i3370.lcssa1324214878, ptr %587, align 4
  store float %_0.i3744.lcssa1325814896, ptr %589, align 4
  store float %_0.i3366.lcssa1328214914, ptr %595, align 4
  store float %_0.i3740.lcssa1328314932, ptr %597, align 4
  %_5.i1921 = add i32 %_246.i545, 1, !dbg !22566
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_246.i545, i32 noundef %_5.i1921, i32 noundef range(i32 0, 536870912) %_56.1.i273.i573, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !22567, !noalias !22568
  unreachable, !dbg !22567

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1928: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1713
  %_15.i1925 = getelementptr inbounds nuw float, ptr %_56.0.i272.i572, i32 %_246.i545, !dbg !22571
  %_0.i3388 = load float, ptr %_15.i1925, align 4, !dbg !22573, !alias.scope !22575, !noalias !22578, !noundef !10
  %_0.i2704 = fadd float %_0.i3139, %_0.i33708991, !dbg !22579
  %_0.i3370 = fsub float %_0.i2704, %_0.i3388, !dbg !22582
  %_8.not.i3.i282.i582 = icmp ugt i32 %_7.i8.i253.i553, %_56.1.i273.i573
  br i1 %_8.not.i3.i282.i582, label %bb4.i6.i311.i690, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i283.i583, !dbg !22584, !prof !4596

bb4.i6.i311.i690:                                 ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1928
  store float %_0.i.i.lcssa1321114662, ptr %572, align 4
  store float %_0.i3763.lcssa1319714680, ptr %_114.i525, align 4
  store float %_0.i3757.lcssa1318314698, ptr %573, align 4
  store float %_0.i.i4022.lcssa1316914716, ptr %575, align 4
  store float %_0.i3776.lcssa1315514734, ptr %_115.i526, align 4
  store float %_0.i3769.lcssa1314114752, ptr %576, align 4
  store float %_0.i.i4029.lcssa1312714770, ptr %578, align 4
  store float %_0.i3789.lcssa1311314788, ptr %_119.i527, align 4
  store float %_0.i3782.lcssa1309914806, ptr %579, align 4
  store float %_0.i.i4036.lcssa1308514824, ptr %581, align 4
  store float %_0.i3802.lcssa1307114842, ptr %_120.i528, align 4
  store float %_0.i3795.lcssa1305714860, ptr %582, align 4
  store float %_0.i3370.lcssa1324214878, ptr %587, align 4
  store float %_0.i3744.lcssa1325814896, ptr %589, align 4
  store float %_0.i3366.lcssa1328214914, ptr %595, align 4
  store float %_0.i3740.lcssa1328314932, ptr %597, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_242.i541, i32 noundef %_7.i8.i253.i553, i32 noundef range(i32 0, 536870912) %_56.1.i273.i573, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #32, !dbg !22589, !noalias !22590
  unreachable, !dbg !22589

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i283.i583: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1928
  %_17.i5.i284.i584 = getelementptr inbounds nuw float, ptr %_56.0.i272.i572, i32 %_242.i541, !dbg !22593
  store float %_0.i3139, ptr %_17.i5.i284.i584, align 4, !dbg !22595, !alias.scope !22597, !noalias !22578
  %_0.i2909 = fdiv float %_0.i3370, %_37.i285.i585, !dbg !22600
  %_0.i3369 = fsub float 1.000000e+00, %_0.i2909, !dbg !22602
  %_0.i3368 = fsub float %_0.i3369, %_0.i37449020, !dbg !22605
  %_4.i2925 = fmul float %_0.i3776, %_0.i3368, !dbg !22608
  %_0.i2926 = fadd float %_0.i37449020, %_4.i2925, !dbg !22608
  %_3.i.i4163.inv = fcmp ogt float %_0.i3369, %_0.i2926, !dbg !22610
  %_4.i.i4170.v = select i1 %_3.i.i4163.inv, float %_0.i3369, float %_0.i2926, !dbg !22610
  %635 = tail call noundef float @llvm.fabs.f32(float %_4.i.i4170.v), !dbg !22614
  %636 = fcmp uge float %635, 0x3BC79CA100000000, !dbg !22618
  %_0.i3744 = select i1 %636, float %_4.i.i4170.v, float 0.000000e+00, !dbg !22620
  %_5.i1913 = add i32 %_243.i542, 1, !dbg !22621
  %exitcond12541.not = icmp eq i32 %iter.i38.sroa.41.08583, %628, !dbg !22624
  br i1 %exitcond12541.not, label %bb4.i1919, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3680, !dbg !22624, !prof !4596

bb4.i1919:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i283.i583
  store float %_0.i.i.lcssa1321114662, ptr %572, align 4
  store float %_0.i3763.lcssa1319714680, ptr %_114.i525, align 4
  store float %_0.i3757.lcssa1318314698, ptr %573, align 4
  store float %_0.i.i4022.lcssa1316914716, ptr %575, align 4
  store float %_0.i3776.lcssa1315514734, ptr %_115.i526, align 4
  store float %_0.i3769.lcssa1314114752, ptr %576, align 4
  store float %_0.i.i4029.lcssa1312714770, ptr %578, align 4
  store float %_0.i3789.lcssa1311314788, ptr %_119.i527, align 4
  store float %_0.i3782.lcssa1309914806, ptr %579, align 4
  store float %_0.i.i4036.lcssa1308514824, ptr %581, align 4
  store float %_0.i3802.lcssa1307114842, ptr %_120.i528, align 4
  store float %_0.i3795.lcssa1305714860, ptr %582, align 4
  store float %_0.i3370.lcssa1324214878, ptr %587, align 4
  store float %_0.i3744.lcssa1325814896, ptr %589, align 4
  store float %_0.i3366.lcssa1328214914, ptr %595, align 4
  store float %_0.i3740.lcssa1328314932, ptr %597, align 4
  %637 = add i32 %umax12539, 1, !dbg !22260
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_243.i542, i32 noundef %637, i32 noundef range(i32 0, 536870912) %_58.1.i298.i598, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !22628, !noalias !22629
  unreachable, !dbg !22628

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3680: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i283.i583
  %_0.i3367 = fsub float 1.000000e+00, %_0.i3744, !dbg !22632
  %_15.i1917 = getelementptr inbounds nuw float, ptr %_58.0.i297.i597, i32 %_243.i542, !dbg !22634
  %_0.i3390 = load float, ptr %_15.i1917, align 4, !dbg !22636, !alias.scope !22638, !noalias !22578, !noundef !10
  store float %_0.i3527, ptr %_15.i1917, align 4, !dbg !22641, !alias.scope !22645, !noalias !22578
  %_0.i3138 = fmul float %_0.i3367, %_0.i3390, !dbg !22648
  %_6.i3947 = bitcast float %_0.i3390 to i32, !dbg !22650
  %_5.i3948 = and i32 %_6.i3947, %all.sroa.0.0.i62, !dbg !22653
  %_8.i3949 = bitcast float %_0.i3138 to i32, !dbg !22654
  %_7.i3951 = and i32 %_9.i3950, %_8.i3949, !dbg !22656
  %_4.i3952 = or disjoint i32 %_7.i3951, %_5.i3948, !dbg !22653
  store i32 %_4.i3952, ptr %data.i.i.i.i.i.i, align 4, !dbg !22657, !alias.scope !22659, !noalias !22662
  %_249.i618 = add nuw i32 %iter.i38.sroa.41.08583, %right_end.sroa.0.0.i, !dbg !22663
  %_251.i620 = add nuw i32 %iter.i38.sroa.41.08583, %right_expiring.sroa.0.0.i, !dbg !22665
  %_8.not.i10.i.i630 = icmp ugt i32 %_7.i8.i253.i553, %_54.1.i.i626
  br i1 %_8.not.i10.i.i630, label %bb4.i13.i.i688, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i632, !dbg !22666, !prof !4596

bb4.i13.i.i688:                                   ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3680
  store float %_0.i.i.lcssa1321114662, ptr %572, align 4
  store float %_0.i3763.lcssa1319714680, ptr %_114.i525, align 4
  store float %_0.i3757.lcssa1318314698, ptr %573, align 4
  store float %_0.i.i4022.lcssa1316914716, ptr %575, align 4
  store float %_0.i3776.lcssa1315514734, ptr %_115.i526, align 4
  store float %_0.i3769.lcssa1314114752, ptr %576, align 4
  store float %_0.i.i4029.lcssa1312714770, ptr %578, align 4
  store float %_0.i3789.lcssa1311314788, ptr %_119.i527, align 4
  store float %_0.i3782.lcssa1309914806, ptr %579, align 4
  store float %_0.i.i4036.lcssa1308514824, ptr %581, align 4
  store float %_0.i3802.lcssa1307114842, ptr %_120.i528, align 4
  store float %_0.i3795.lcssa1305714860, ptr %582, align 4
  store float %_0.i3370.lcssa1324214878, ptr %587, align 4
  store float %_0.i3744.lcssa1325814896, ptr %589, align 4
  store float %_0.i3366.lcssa1328214914, ptr %595, align 4
  store float %_0.i3740.lcssa1328314932, ptr %597, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_242.i541, i32 noundef %_7.i8.i253.i553, i32 noundef range(i32 0, 536870912) %_54.1.i.i626, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #32, !dbg !22672, !noalias !22673
  unreachable, !dbg !22672

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i632: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3680
  %_0.i2908 = fdiv float %_0.i3789, %_0.i3966, !dbg !22681
  %_3.i2475 = fcmp uge float %_0.i3789, %_0.i3966, !dbg !22683
  %_0.i3946 = select i1 %_3.i2475, float 1.000000e+00, float %_0.i2908, !dbg !22685
  %_17.i12.i.i633 = getelementptr inbounds nuw float, ptr %_54.0.i.i625, i32 %_242.i541, !dbg !22687
  store float %_0.i3946, ptr %_17.i12.i.i633, align 4, !dbg !22689, !alias.scope !22691, !noalias !22694
  %or.cond.i1892.not = icmp ult i32 %_249.i618, %_54.1.i.i626, !dbg !22695
  br i1 %or.cond.i1892.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1896, label %bb4.i1895, !dbg !22695, !prof !10564

bb4.i1895:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i632
  store float %_0.i.i.lcssa1321114662, ptr %572, align 4
  store float %_0.i3763.lcssa1319714680, ptr %_114.i525, align 4
  store float %_0.i3757.lcssa1318314698, ptr %573, align 4
  store float %_0.i.i4022.lcssa1316914716, ptr %575, align 4
  store float %_0.i3776.lcssa1315514734, ptr %_115.i526, align 4
  store float %_0.i3769.lcssa1314114752, ptr %576, align 4
  store float %_0.i.i4029.lcssa1312714770, ptr %578, align 4
  store float %_0.i3789.lcssa1311314788, ptr %_119.i527, align 4
  store float %_0.i3782.lcssa1309914806, ptr %579, align 4
  store float %_0.i.i4036.lcssa1308514824, ptr %581, align 4
  store float %_0.i3802.lcssa1307114842, ptr %_120.i528, align 4
  store float %_0.i3795.lcssa1305714860, ptr %582, align 4
  store float %_0.i3370.lcssa1324214878, ptr %587, align 4
  store float %_0.i3744.lcssa1325814896, ptr %589, align 4
  store float %_0.i3366.lcssa1328214914, ptr %595, align 4
  store float %_0.i3740.lcssa1328314932, ptr %597, align 4
  %_5.i1889 = add i32 %_249.i618, 1, !dbg !22701
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_249.i618, i32 noundef %_5.i1889, i32 noundef range(i32 0, 536870912) %_54.1.i.i626, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !22702, !noalias !22703
  unreachable, !dbg !22702

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1896: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i632
  %_15.i1893 = getelementptr inbounds nuw float, ptr %_54.0.i.i625, i32 %_249.i618, !dbg !22709
  %_0.i3396 = load float, ptr %_15.i1893, align 4, !dbg !22711, !alias.scope !22713, !noalias !22716, !noundef !10
  %638 = icmp eq i32 %storemerge.i9048, 0, !dbg !22717
  %_3.i.i4343.inv = fcmp olt float %running.sroa.0.0.i9075, %_0.i3396, !dbg !22717
  %_4.i.i4350.v = select i1 %_3.i.i4343.inv, float %running.sroa.0.0.i9075, float %_0.i3396, !dbg !22717
  %running.sroa.0.0.i = select i1 %638, float %_0.i3396, float %_4.i.i4350.v, !dbg !22717
  %_15.i = add i32 %storemerge.i9048, 1, !dbg !22718
  %complete.i = icmp eq i32 %_15.i, %_18.i.i636, !dbg !22718
  br i1 %complete.i, label %bb11.i1677.preheader, label %bb7.i1675, !dbg !22719

bb11.i1677.preheader:                             ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1896
  br i1 %_29.i16798577.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit, label %bb19.i1680, !dbg !22720

bb7.i1675:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1896
  %or.cond.i1884.not = icmp ult i32 %_245.i544, %_54.1.i.i626, !dbg !22723
  br i1 %or.cond.i1884.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1888, label %bb4.i1887, !dbg !22723, !prof !10564

bb4.i1887:                                        ; preds = %bb7.i1675
  store float %_0.i.i.lcssa1321114662, ptr %572, align 4
  store float %_0.i3763.lcssa1319714680, ptr %_114.i525, align 4
  store float %_0.i3757.lcssa1318314698, ptr %573, align 4
  store float %_0.i.i4022.lcssa1316914716, ptr %575, align 4
  store float %_0.i3776.lcssa1315514734, ptr %_115.i526, align 4
  store float %_0.i3769.lcssa1314114752, ptr %576, align 4
  store float %_0.i.i4029.lcssa1312714770, ptr %578, align 4
  store float %_0.i3789.lcssa1311314788, ptr %_119.i527, align 4
  store float %_0.i3782.lcssa1309914806, ptr %579, align 4
  store float %_0.i.i4036.lcssa1308514824, ptr %581, align 4
  store float %_0.i3802.lcssa1307114842, ptr %_120.i528, align 4
  store float %_0.i3795.lcssa1305714860, ptr %582, align 4
  store float %_0.i3370.lcssa1324214878, ptr %587, align 4
  store float %_0.i3744.lcssa1325814896, ptr %589, align 4
  store float %_0.i3366.lcssa1328214914, ptr %595, align 4
  store float %_0.i3740.lcssa1328314932, ptr %597, align 4
  %_5.i1881 = add i32 %_245.i544, 1, !dbg !22728
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_245.i544, i32 noundef %_5.i1881, i32 noundef range(i32 0, 536870912) %_54.1.i.i626, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !22729, !noalias !22730
  unreachable, !dbg !22729

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1888: ; preds = %bb7.i1675
  %_15.i1885 = getelementptr inbounds nuw float, ptr %_54.0.i.i625, i32 %_245.i544, !dbg !22733
  %_0.i3398 = load float, ptr %_15.i1885, align 4, !dbg !22735, !alias.scope !22737, !noalias !22716, !noundef !10
  %_3.i.i4334.inv = fcmp olt float %_0.i3398, %running.sroa.0.0.i, !dbg !22740
  %_4.i.i4341.v = select i1 %_3.i.i4334.inv, float %_0.i3398, float %running.sroa.0.0.i, !dbg !22740
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit, !dbg !22743

bb19.i1680:                                       ; preds = %bb11.i1677.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i
  %end.sroa.0.0.i8580 = phi i32 [ %640, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], [ %_249.i618, %bb11.i1677.preheader ]
  %suffix.sroa.0.0.i8579 = phi float [ %_4.i.i4332.v, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], [ %_0.i3396, %bb11.i1677.preheader ]
  %iter.sroa.0.0.i16788578 = phi i32 [ %_30.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], [ 0, %bb11.i1677.preheader ]
  %or.cond.i1868.not = icmp ult i32 %end.sroa.0.0.i8580, %_54.1.i.i626, !dbg !22744
  br i1 %or.cond.i1868.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i, label %bb4.i1871, !dbg !22744, !prof !10564

bb4.i1871:                                        ; preds = %bb19.i1680
  store float %_0.i.i.lcssa1321114662, ptr %572, align 4
  store float %_0.i3763.lcssa1319714680, ptr %_114.i525, align 4
  store float %_0.i3757.lcssa1318314698, ptr %573, align 4
  store float %_0.i.i4022.lcssa1316914716, ptr %575, align 4
  store float %_0.i3776.lcssa1315514734, ptr %_115.i526, align 4
  store float %_0.i3769.lcssa1314114752, ptr %576, align 4
  store float %_0.i.i4029.lcssa1312714770, ptr %578, align 4
  store float %_0.i3789.lcssa1311314788, ptr %_119.i527, align 4
  store float %_0.i3782.lcssa1309914806, ptr %579, align 4
  store float %_0.i.i4036.lcssa1308514824, ptr %581, align 4
  store float %_0.i3802.lcssa1307114842, ptr %_120.i528, align 4
  store float %_0.i3795.lcssa1305714860, ptr %582, align 4
  store float %_0.i3370.lcssa1324214878, ptr %587, align 4
  store float %_0.i3744.lcssa1325814896, ptr %589, align 4
  store float %_0.i3366.lcssa1328214914, ptr %595, align 4
  store float %_0.i3740.lcssa1328314932, ptr %597, align 4
  %_5.i1865 = add i32 %end.sroa.0.0.i8580, 1, !dbg !22749
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %end.sroa.0.0.i8580, i32 noundef %_5.i1865, i32 noundef range(i32 0, 536870912) %_54.1.i.i626, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !22750, !noalias !22751
  unreachable, !dbg !22750

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i: ; preds = %bb19.i1680
  %_30.i = add nuw i32 %iter.sroa.0.0.i16788578, 1, !dbg !22754
  %_15.i1869 = getelementptr inbounds nuw float, ptr %_54.0.i.i625, i32 %end.sroa.0.0.i8580, !dbg !22757
  %_0.i3402 = load float, ptr %_15.i1869, align 4, !dbg !22759, !alias.scope !22761, !noalias !22716, !noundef !10
  %_3.i.i4325.inv = fcmp olt float %suffix.sroa.0.0.i8579, %_0.i3402, !dbg !22764
  %_4.i.i4332.v = select i1 %_3.i.i4325.inv, float %suffix.sroa.0.0.i8579, float %_0.i3402, !dbg !22764
  store float %_4.i.i4332.v, ptr %_15.i1869, align 4, !dbg !22767, !alias.scope !22770, !noalias !22716
  %639 = icmp eq i32 %end.sroa.0.0.i8580, 0, !dbg !22773
  %spec.store.select.i1683 = select i1 %639, i32 %ring.i63, i32 %end.sroa.0.0.i8580, !dbg !22773
  %640 = add i32 %spec.store.select.i1683, -1, !dbg !22774
  %exitcond12537.not = icmp eq i32 %_30.i, %_18.i.i636, !dbg !22775
  br i1 %exitcond12537.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit, label %bb19.i1680, !dbg !22720

_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i, %bb11.i1677.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1888
  %storemerge.i = phi i32 [ %_15.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1888 ], [ 0, %bb11.i1677.preheader ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], !dbg !22777
  %running.sroa.0.1.i = phi float [ %_4.i.i4341.v, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1888 ], [ %running.sroa.0.0.i, %bb11.i1677.preheader ], [ %running.sroa.0.0.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], !dbg !22778
  %_0.i3137 = fmul float %running.sroa.0.1.i, 1.638400e+04, !dbg !22779
  %641 = tail call noundef float @llvm.floor.f32(float %_0.i3137), !dbg !22781
  %_0.i3136 = fmul float %641, 0x3F10000000000000, !dbg !22785
  %or.cond.i1908.not = icmp ult i32 %_251.i620, %_56.1.i.i648, !dbg !22787
  br i1 %or.cond.i1908.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1912, label %bb4.i1911, !dbg !22787, !prof !10564

bb4.i1911:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit
  store float %_0.i.i.lcssa1321114662, ptr %572, align 4
  store float %_0.i3763.lcssa1319714680, ptr %_114.i525, align 4
  store float %_0.i3757.lcssa1318314698, ptr %573, align 4
  store float %_0.i.i4022.lcssa1316914716, ptr %575, align 4
  store float %_0.i3776.lcssa1315514734, ptr %_115.i526, align 4
  store float %_0.i3769.lcssa1314114752, ptr %576, align 4
  store float %_0.i.i4029.lcssa1312714770, ptr %578, align 4
  store float %_0.i3789.lcssa1311314788, ptr %_119.i527, align 4
  store float %_0.i3782.lcssa1309914806, ptr %579, align 4
  store float %_0.i.i4036.lcssa1308514824, ptr %581, align 4
  store float %_0.i3802.lcssa1307114842, ptr %_120.i528, align 4
  store float %_0.i3795.lcssa1305714860, ptr %582, align 4
  store float %_0.i3370.lcssa1324214878, ptr %587, align 4
  store float %_0.i3744.lcssa1325814896, ptr %589, align 4
  store float %_0.i3366.lcssa1328214914, ptr %595, align 4
  store float %_0.i3740.lcssa1328314932, ptr %597, align 4
  %_5.i1905 = add i32 %_251.i620, 1, !dbg !22792
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_251.i620, i32 noundef %_5.i1905, i32 noundef range(i32 0, 536870912) %_56.1.i.i648, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !22793, !noalias !22794
  unreachable, !dbg !22793

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1912: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit
  %_15.i1909 = getelementptr inbounds nuw float, ptr %_56.0.i.i647, i32 %_251.i620, !dbg !22797
  %_0.i3392 = load float, ptr %_15.i1909, align 4, !dbg !22799, !alias.scope !22801, !noalias !22804, !noundef !10
  %_0.i2703 = fadd float %_0.i3136, %_0.i33669103, !dbg !22805
  %_0.i3366 = fsub float %_0.i2703, %_0.i3392, !dbg !22807
  %_8.not.i3.i.i657 = icmp ugt i32 %_7.i8.i253.i553, %_56.1.i.i648
  br i1 %_8.not.i3.i.i657, label %bb4.i6.i.i687, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i658, !dbg !22809, !prof !4596

bb4.i6.i.i687:                                    ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1912
  store float %_0.i.i.lcssa1321114662, ptr %572, align 4
  store float %_0.i3763.lcssa1319714680, ptr %_114.i525, align 4
  store float %_0.i3757.lcssa1318314698, ptr %573, align 4
  store float %_0.i.i4022.lcssa1316914716, ptr %575, align 4
  store float %_0.i3776.lcssa1315514734, ptr %_115.i526, align 4
  store float %_0.i3769.lcssa1314114752, ptr %576, align 4
  store float %_0.i.i4029.lcssa1312714770, ptr %578, align 4
  store float %_0.i3789.lcssa1311314788, ptr %_119.i527, align 4
  store float %_0.i3782.lcssa1309914806, ptr %579, align 4
  store float %_0.i.i4036.lcssa1308514824, ptr %581, align 4
  store float %_0.i3802.lcssa1307114842, ptr %_120.i528, align 4
  store float %_0.i3795.lcssa1305714860, ptr %582, align 4
  store float %_0.i3370.lcssa1324214878, ptr %587, align 4
  store float %_0.i3744.lcssa1325814896, ptr %589, align 4
  store float %_0.i3366.lcssa1328214914, ptr %595, align 4
  store float %_0.i3740.lcssa1328314932, ptr %597, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_242.i541, i32 noundef %_7.i8.i253.i553, i32 noundef range(i32 0, 536870912) %_56.1.i.i648, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #32, !dbg !22814, !noalias !22815
  unreachable, !dbg !22814

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i658: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1912
  %_17.i5.i.i659 = getelementptr inbounds nuw float, ptr %_56.0.i.i647, i32 %_242.i541, !dbg !22818
  store float %_0.i3136, ptr %_17.i5.i.i659, align 4, !dbg !22820, !alias.scope !22822, !noalias !22804
  %_6.not.i1899 = icmp ugt i32 %_5.i1913, %_58.1.i.i673
  br i1 %_6.not.i1899, label %bb4.i1903, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673, !dbg !22825, !prof !4596

bb4.i1903:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i658
  store float %_0.i.i.lcssa1321114662, ptr %572, align 4
  store float %_0.i3763.lcssa1319714680, ptr %_114.i525, align 4
  store float %_0.i3757.lcssa1318314698, ptr %573, align 4
  store float %_0.i.i4022.lcssa1316914716, ptr %575, align 4
  store float %_0.i3776.lcssa1315514734, ptr %_115.i526, align 4
  store float %_0.i3769.lcssa1314114752, ptr %576, align 4
  store float %_0.i.i4029.lcssa1312714770, ptr %578, align 4
  store float %_0.i3789.lcssa1311314788, ptr %_119.i527, align 4
  store float %_0.i3782.lcssa1309914806, ptr %579, align 4
  store float %_0.i.i4036.lcssa1308514824, ptr %581, align 4
  store float %_0.i3802.lcssa1307114842, ptr %_120.i528, align 4
  store float %_0.i3795.lcssa1305714860, ptr %582, align 4
  store float %_0.i3370.lcssa1324214878, ptr %587, align 4
  store float %_0.i3744.lcssa1325814896, ptr %589, align 4
  store float %_0.i3366.lcssa1328214914, ptr %595, align 4
  store float %_0.i3740.lcssa1328314932, ptr %597, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_243.i542, i32 noundef %_5.i1913, i32 noundef range(i32 0, 536870912) %_58.1.i.i673, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !22830, !noalias !22831
  unreachable, !dbg !22830

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i658
  %_0.i2907 = fdiv float %_0.i3366, %_37.i.i660, !dbg !22834
  %_0.i3365 = fsub float 1.000000e+00, %_0.i2907, !dbg !22836
  %_0.i3364 = fsub float %_0.i3365, %_0.i37409132, !dbg !22838
  %_4.i2923 = fmul float %_0.i3802, %_0.i3364, !dbg !22840
  %_0.i2924 = fadd float %_0.i37409132, %_4.i2923, !dbg !22840
  %_3.i.i4154.inv = fcmp ogt float %_0.i3365, %_0.i2924, !dbg !22842
  %_4.i.i4161.v = select i1 %_3.i.i4154.inv, float %_0.i3365, float %_0.i2924, !dbg !22842
  %642 = tail call noundef float @llvm.fabs.f32(float %_4.i.i4161.v), !dbg !22845
  %643 = fcmp uge float %642, 0x3BC79CA100000000, !dbg !22848
  %_0.i3740 = select i1 %643, float %_4.i.i4161.v, float 0.000000e+00, !dbg !22850
  %_0.i3363 = fsub float 1.000000e+00, %_0.i3740, !dbg !22851
  %_15.i1901 = getelementptr inbounds nuw float, ptr %_58.0.i.i672, i32 %_243.i542, !dbg !22853
  %_0.i3394 = load float, ptr %_15.i1901, align 4, !dbg !22855, !alias.scope !22857, !noalias !22804, !noundef !10
  store float %_0.i3522, ptr %_15.i1901, align 4, !dbg !22860, !alias.scope !22863, !noalias !22804
  %_0.i3135 = fmul float %_0.i3363, %_0.i3394, !dbg !22866
  %_6.i3934 = bitcast float %_0.i3394 to i32, !dbg !22868
  %_5.i3935 = and i32 %_6.i3934, %all.sroa.0.0.i62, !dbg !22871
  %_8.i3936 = bitcast float %_0.i3135 to i32, !dbg !22872
  %_7.i3938 = and i32 %_9.i3950, %_8.i3936, !dbg !22874
  %_4.i3939 = or disjoint i32 %_7.i3938, %_5.i3935, !dbg !22871
  store i32 %_4.i3939, ptr %data.i5.i.i.i.i.i, align 4, !dbg !22875, !alias.scope !22877, !noalias !22880
  %exitcond12550.not = icmp eq i32 %_235.0.i524, %umin12549, !dbg !22260
  br i1 %exitcond12550.not, label %bb74.i692, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3524, !dbg !22260

bb74.i692:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit
  %_0.i3740.lcssa1328314931 = phi float [ %_0.i3740.lcssa1328314932, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3740, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %_0.i3366.lcssa1328214913 = phi float [ %_0.i3366.lcssa1328214914, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3366, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %_0.i3744.lcssa1325814895 = phi float [ %_0.i3744.lcssa1325814896, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3744, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %_0.i3370.lcssa1324214877 = phi float [ %_0.i3370.lcssa1324214878, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3370, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %_0.i3795.lcssa1305714859 = phi float [ %_0.i3795.lcssa1305714860, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3795, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %_0.i3802.lcssa1307114841 = phi float [ %_0.i3802.lcssa1307114842, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3802, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %_0.i.i4036.lcssa1308514823 = phi float [ %_0.i.i4036.lcssa1308514824, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i.i4036, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %_0.i3782.lcssa1309914805 = phi float [ %_0.i3782.lcssa1309914806, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3782, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %_0.i3789.lcssa1311314787 = phi float [ %_0.i3789.lcssa1311314788, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3789, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %_0.i.i4029.lcssa1312714769 = phi float [ %_0.i.i4029.lcssa1312714770, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i.i4029, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %_0.i3769.lcssa1314114751 = phi float [ %_0.i3769.lcssa1314114752, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3769, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %_0.i3776.lcssa1315514733 = phi float [ %_0.i3776.lcssa1315514734, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3776, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %_0.i.i4022.lcssa1316914715 = phi float [ %_0.i.i4022.lcssa1316914716, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i.i4022, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %_0.i3757.lcssa1318314697 = phi float [ %_0.i3757.lcssa1318314698, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3757, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %_0.i3763.lcssa1319714679 = phi float [ %_0.i3763.lcssa1319714680, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3763, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %_0.i.i.lcssa1321114661 = phi float [ %_0.i.i.lcssa1321114662, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %running.sroa.0.0.i.lcssa90979275 = phi float [ %running.sroa.0.0.i.lcssa90979276, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %running.sroa.0.0.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %storemerge.i.lcssa90709239 = phi i32 [ %storemerge.i.lcssa90709240, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %storemerge.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %running.sroa.0.0.i1689.lcssa89859203 = phi float [ %running.sroa.0.0.i1689.lcssa89859204, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %running.sroa.0.0.i1689, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %storemerge.i1694.lcssa89589167 = phi i32 [ %storemerge.i1694.lcssa89589168, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %storemerge.i1694, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3673 ]
  %_143.i693 = add i32 %run.sroa.0.0.i, %ring_cursor.sroa.0.1.i4809161, !dbg !22881
  %_241.not.i694 = icmp ult i32 %_143.i693, %ring.i63, !dbg !22882
  %644 = select i1 %_241.not.i694, i32 0, i32 %ring.i63, !dbg !22882
  %ring_cursor.sroa.0.2.i695 = sub nuw i32 %_143.i693, %644, !dbg !22882
  %_145.i696 = add i32 %run.sroa.0.0.i, %main_cursor.sroa.0.1.i4819162, !dbg !22885
  %_252.not.i697 = icmp ult i32 %_145.i696, %main.i64, !dbg !22886
  %645 = select i1 %_252.not.i697, i32 0, i32 %main.i64, !dbg !22886
  %main_cursor.sroa.0.2.i698 = sub nuw i32 %_145.i696, %645, !dbg !22886
  %_63.i483 = icmp ult i32 %_87.i497, %spec.store.select.i79, !dbg !21636
  br i1 %_63.i483, label %bb20.i484, label %bb19.i479.bb15.i72.loopexit_crit_edge, !dbg !21636

_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit: ; preds = %bb15.i72.loopexit, %bb7.i
  %ring_cursor.sroa.0.0.i73.lcssa = phi i32 [ %_37.i66, %bb7.i ], [ %ring_cursor.sroa.0.1.i480.lcssa, %bb15.i72.loopexit ], !dbg !21604
  %main_cursor.sroa.0.0.i74.lcssa = phi i32 [ %_36.i65, %bb7.i ], [ %main_cursor.sroa.0.1.i481.lcssa, %bb15.i72.loopexit ], !dbg !21601
  %646 = getelementptr inbounds nuw i8, ptr %uniform_left.i51, i32 24, !dbg !22888
  %left_prefix.i703 = load float, ptr %646, align 4, !dbg !22888, !noalias !21580, !noundef !10
  %647 = getelementptr inbounds nuw i8, ptr %uniform_left.i51, i32 28, !dbg !22889
  %left_phase.i704 = load i32, ptr %647, align 4, !dbg !22889, !noalias !21580, !noundef !10
  %648 = getelementptr inbounds nuw i8, ptr %uniform_right.i50, i32 24, !dbg !22890
  %right_prefix.i705 = load float, ptr %648, align 4, !dbg !22890, !noalias !21580, !noundef !10
  %649 = getelementptr inbounds nuw i8, ptr %uniform_right.i50, i32 28, !dbg !22891
  %right_phase.i706 = load i32, ptr %649, align 4, !dbg !22891, !noalias !21580, !noundef !10
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_right.i50), !dbg !22892, !noalias !21580
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i51), !dbg !22893, !noalias !21580
  %650 = getelementptr inbounds nuw i8, ptr %self, i32 364, !dbg !22894
  %_265.0.i707 = load ptr, ptr %650, align 4, !dbg !22894, !alias.scope !21570, !noalias !22896, !nonnull !10, !noundef !10
  %651 = getelementptr inbounds nuw i8, ptr %self, i32 368, !dbg !22894
  %_265.1.i708 = load i32, ptr %651, align 4, !dbg !22894, !alias.scope !21570, !noalias !22896, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !22897), !dbg !22900
  %_4.not.i3666 = icmp eq i32 %_265.1.i708, 0, !dbg !22901
  br i1 %_4.not.i3666, label %panic.i3668, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3669, !dbg !22901

panic.i3668:                                      ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #32, !dbg !22901, !noalias !22903
  unreachable, !dbg !22901

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3669: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit
  store float %left_prefix.i703, ptr %_265.0.i707, align 4, !dbg !22901, !alias.scope !22897, !noalias !21574
  %_266.0.i709 = load ptr, ptr %68, align 4, !dbg !22904, !alias.scope !21570, !noalias !22896, !nonnull !10, !noundef !10
  %_266.1.i710 = load i32, ptr %69, align 4, !dbg !22904, !alias.scope !21570, !noalias !22896, !noundef !10
  %652 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i704), !dbg !22905
  br i1 %652, label %bb2.i4803, label %bb6.i4799, !dbg !22905

bb6.i4799:                                        ; preds = %bb2.i4803, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3669
  %end_or_len.idx.i = shl nuw nsw i32 %_266.1.i710, 2, !dbg !22909
  %end_or_len.i = getelementptr inbounds nuw i8, ptr %_266.0.i709, i32 %end_or_len.idx.i, !dbg !22909
  %_293.i = icmp eq i32 %_266.1.i710, 0, !dbg !22913
  br i1 %_293.i, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i4800, !dbg !22916

bb2.i4803:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3669
  %bytes1.sroa.0.0.zext.i = and i32 %left_phase.i704, 255, !dbg !22917
  %bytes1.sroa.0.0.isplat.i = mul nuw i32 %bytes1.sroa.0.0.zext.i, 16843009, !dbg !22917
  %_5.i4804 = icmp eq i32 %left_phase.i704, %bytes1.sroa.0.0.isplat.i, !dbg !22918
  br i1 %_5.i4804, label %bb3.i4805, label %bb6.i4799, !dbg !22918

bb3.i4805:                                        ; preds = %bb2.i4803
  %bytes.sroa.0.0.extract.trunc.i = trunc i32 %left_phase.i704 to i8, !dbg !22919
  %653 = shl nuw nsw i32 %_266.1.i710, 2, !dbg !22921
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %_266.0.i709, i8 %bytes.sroa.0.0.extract.trunc.i, i32 %653, i1 false), !dbg !22921, !alias.scope !22922, !noalias !21574
  br label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, !dbg !22925

bb10.i4800:                                       ; preds = %bb6.i4799, %bb10.i4800
  %iter.sroa.0.04.i = phi ptr [ %_38.i4801, %bb10.i4800 ], [ %_266.0.i709, %bb6.i4799 ]
  %_38.i4801 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i, i32 4, !dbg !22926
  store i32 %left_phase.i704, ptr %iter.sroa.0.04.i, align 4, !dbg !22928, !alias.scope !22922, !noalias !21574
  %_29.i4802 = icmp eq ptr %_38.i4801, %end_or_len.i, !dbg !22913
  br i1 %_29.i4802, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i4800, !dbg !22916

_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit: ; preds = %bb10.i4800, %bb6.i4799, %bb3.i4805
  %654 = getelementptr inbounds nuw i8, ptr %self, i32 464, !dbg !22929
  %_267.0.i711 = load ptr, ptr %654, align 4, !dbg !22929, !alias.scope !21572, !noalias !22930, !nonnull !10, !noundef !10
  %655 = getelementptr inbounds nuw i8, ptr %self, i32 468, !dbg !22929
  %_267.1.i712 = load i32, ptr %655, align 4, !dbg !22929, !alias.scope !21572, !noalias !22930, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !22931), !dbg !22934
  %_4.not.i3662 = icmp eq i32 %_267.1.i712, 0, !dbg !22935
  br i1 %_4.not.i3662, label %panic.i3664, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3665, !dbg !22935

panic.i3664:                                      ; preds = %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #32, !dbg !22935, !noalias !22937
  unreachable, !dbg !22935

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3665: ; preds = %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit
  store float %right_prefix.i705, ptr %_267.0.i711, align 4, !dbg !22935, !alias.scope !22931, !noalias !21574
  %_268.0.i713 = load ptr, ptr %77, align 4, !dbg !22938, !alias.scope !21572, !noalias !22930, !nonnull !10, !noundef !10
  %_268.1.i714 = load i32, ptr %78, align 4, !dbg !22938, !alias.scope !21572, !noalias !22930, !noundef !10
  %656 = tail call i1 @llvm.is.constant.i32(i32 %right_phase.i706), !dbg !22939
  br i1 %656, label %bb2.i4814, label %bb6.i4806, !dbg !22939

bb6.i4806:                                        ; preds = %bb2.i4814, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3665
  %end_or_len.idx.i4807 = shl nuw nsw i32 %_268.1.i714, 2, !dbg !22942
  %end_or_len.i4808 = getelementptr inbounds nuw i8, ptr %_268.0.i713, i32 %end_or_len.idx.i4807, !dbg !22942
  %_293.i4809 = icmp eq i32 %_268.1.i714, 0, !dbg !22946
  br i1 %_293.i4809, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4820, label %bb10.i4810, !dbg !22949

bb2.i4814:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3665
  %bytes1.sroa.0.0.zext.i4815 = and i32 %right_phase.i706, 255, !dbg !22950
  %bytes1.sroa.0.0.isplat.i4816 = mul nuw i32 %bytes1.sroa.0.0.zext.i4815, 16843009, !dbg !22950
  %_5.i4817 = icmp eq i32 %right_phase.i706, %bytes1.sroa.0.0.isplat.i4816, !dbg !22951
  br i1 %_5.i4817, label %bb3.i4818, label %bb6.i4806, !dbg !22951

bb3.i4818:                                        ; preds = %bb2.i4814
  %bytes.sroa.0.0.extract.trunc.i4819 = trunc i32 %right_phase.i706 to i8, !dbg !22952
  %657 = shl nuw nsw i32 %_268.1.i714, 2, !dbg !22954
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %_268.0.i713, i8 %bytes.sroa.0.0.extract.trunc.i4819, i32 %657, i1 false), !dbg !22954, !alias.scope !22955, !noalias !21574
  br label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4820, !dbg !22958

bb10.i4810:                                       ; preds = %bb6.i4806, %bb10.i4810
  %iter.sroa.0.04.i4811 = phi ptr [ %_38.i4812, %bb10.i4810 ], [ %_268.0.i713, %bb6.i4806 ]
  %_38.i4812 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i4811, i32 4, !dbg !22959
  store i32 %right_phase.i706, ptr %iter.sroa.0.04.i4811, align 4, !dbg !22961, !alias.scope !22955, !noalias !21574
  %_29.i4813 = icmp eq ptr %_38.i4812, %end_or_len.i4808, !dbg !22946
  br i1 %_29.i4813, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4820, label %bb10.i4810, !dbg !22949

_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4820: ; preds = %bb10.i4810, %bb6.i4806, %bb3.i4818
  call void @llvm.lifetime.start.p0(ptr nonnull %_157.i35), !dbg !22962, !noalias !21580
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(92) %_157.i35, ptr noundef nonnull align 4 dereferenceable(92) %hot_left.i55, i32 92, i1 false), !dbg !22962, !noalias !21580
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_157.i35, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33) #31, !dbg !22963, !noalias !21574
  call void @llvm.lifetime.end.p0(ptr nonnull %_157.i35), !dbg !22964, !noalias !21580
  call void @llvm.lifetime.start.p0(ptr nonnull %_159.i34), !dbg !22965, !noalias !21580
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(92) %_159.i34, ptr noundef nonnull align 4 dereferenceable(92) %hot_right.i54, i32 92, i1 false), !dbg !22965, !noalias !21580
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_159.i34, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34) #31, !dbg !22966, !noalias !21574
  call void @llvm.lifetime.end.p0(ptr nonnull %_159.i34), !dbg !22967, !noalias !21580
  store i32 %main_cursor.sroa.0.0.i74.lcssa, ptr %_35, align 4, !dbg !22968, !alias.scope !21574, !noalias !21603
  store i32 %ring_cursor.sroa.0.0.i73.lcssa, ptr %532, align 4, !dbg !22969, !alias.scope !21574, !noalias !21603
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i52), !dbg !22970, !noalias !21580
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i53), !dbg !22971, !noalias !21580
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_right.i54), !dbg !22972, !noalias !21580
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i55), !dbg !22973, !noalias !21580
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockfEB2_.exit, !dbg !21567

bb6.i:                                            ; preds = %bb5.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !22974), !dbg !22977
  tail call void @llvm.experimental.noalias.scope.decl(metadata !22978), !dbg !22977
  tail call void @llvm.experimental.noalias.scope.decl(metadata !22980), !dbg !22977
  tail call void @llvm.experimental.noalias.scope.decl(metadata !22982), !dbg !22977
  tail call void @llvm.experimental.noalias.scope.decl(metadata !22984), !dbg !22977
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_left.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #31, !dbg !22986
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_right.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #31, !dbg !22990
  %658 = getelementptr inbounds nuw i8, ptr %self, i32 320, !dbg !22992
  %659 = load i8, ptr %658, align 4, !dbg !22992, !range !4667, !alias.scope !22974, !noalias !22996, !noundef !10
  %660 = getelementptr inbounds nuw i8, ptr %self, i32 321, !dbg !22999
  %661 = load i8, ptr %660, align 1, !dbg !22999, !range !4667, !alias.scope !22974, !noalias !22996, !noundef !10
  %662 = getelementptr inbounds nuw i8, ptr %self, i32 528, !dbg !23001
  %ring.i = load i32, ptr %662, align 4, !dbg !23001, !alias.scope !22978, !noalias !23003, !noundef !10
  %663 = getelementptr inbounds nuw i8, ptr %self, i32 532, !dbg !23004
  %main.i = load i32, ptr %663, align 4, !dbg !23004, !alias.scope !22978, !noalias !23003, !noundef !10
  %_36.i = load i32, ptr %_35, align 4, !dbg !23006, !alias.scope !22984, !noalias !23008, !noundef !10
  %664 = getelementptr inbounds nuw i8, ptr %self, i32 108, !dbg !23009
  %_37.i = load i32, ptr %664, align 4, !dbg !23009, !alias.scope !22984, !noalias !23008, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i), !dbg !23011, !noalias !23013
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i, i8 0, i32 1024, i1 false), !noalias !23013
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i), !dbg !23014, !noalias !23013
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i, i8 0, i32 1024, i1 false), !noalias !23013
  %_32.i = zext nneg i8 %659 to i32, !dbg !22992
  %.none.i = sub nsw i32 0, %_32.i, !dbg !23016
  %_33.i = zext nneg i8 %661 to i32, !dbg !22999
  %all.sroa.0.0.i = sub nsw i32 0, %_33.i, !dbg !22999
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i), !dbg !23017, !noalias !23013
; call <true_peak_limiter::UniformHot<f32>>::new
  call fastcc void @_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotfE3newB5_(ptr noalias noundef align 4 captures(none) dereferenceable(44) %uniform_left.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33, i32 %ring.i, i32 %main.i) #31, !dbg !23019
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_right.i), !dbg !23020, !noalias !23013
  %_32.val4356 = load i32, ptr %662, align 4, !dbg !23022, !noundef !10
  %_32.val4357 = load i32, ptr %663, align 4, !dbg !23022, !noundef !10
; call <true_peak_limiter::UniformHot<f32>>::new
  call fastcc void @_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotfE3newB5_(ptr noalias noundef align 4 captures(none) dereferenceable(44) %uniform_right.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34, i32 %_32.val4356, i32 %_32.val4357) #31, !dbg !23022
  %_166.not.i9756 = icmp eq i32 %frames, 0, !dbg !23023
  br i1 %_166.not.i9756, label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit, label %bb44.i.lr.ph, !dbg !23023

bb44.i.lr.ph:                                     ; preds = %bb6.i
  %d9.i4821 = lshr i32 %frames, 5, !dbg !23033
  %r2.i4822 = and i32 %frames, 31, !dbg !23040
  %_19.not.i4823 = icmp ne i32 %r2.i4822, 0, !dbg !23041
  %665 = zext i1 %_19.not.i4823 to i32, !dbg !23041
  %yield_count.sroa.0.0.i4824 = add nuw nsw i32 %d9.i4821, %665, !dbg !23041
  %history.i44.i.sroa.7.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 4
  %history.i44.i.sroa.10.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 8
  %history.i44.i.sroa.13.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 12
  %history.i44.i.sroa.16.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 16
  %history.i44.i.sroa.19.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 20
  %history.i44.i.sroa.22.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 24
  %history.i44.i.sroa.26.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 28
  %history.i44.i.sroa.29.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 32
  %history.i44.i.sroa.32.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 36
  %history.i44.i.sroa.35.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 40
  %history.i44.i.sroa.38.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 44
  %666 = getelementptr inbounds nuw i8, ptr %self, i32 132
  %667 = getelementptr inbounds nuw i8, ptr %self, i32 136
  %668 = getelementptr inbounds nuw i8, ptr %self, i32 140
  %row1.i.i.i77.i = getelementptr inbounds nuw i8, ptr %self, i32 144
  %669 = getelementptr inbounds nuw i8, ptr %self, i32 148
  %670 = getelementptr inbounds nuw i8, ptr %self, i32 152
  %671 = getelementptr inbounds nuw i8, ptr %self, i32 156
  %row3.i.i.i91.i = getelementptr inbounds nuw i8, ptr %self, i32 160
  %672 = getelementptr inbounds nuw i8, ptr %self, i32 164
  %673 = getelementptr inbounds nuw i8, ptr %self, i32 168
  %674 = getelementptr inbounds nuw i8, ptr %self, i32 172
  %row5.i.i.i105.i = getelementptr inbounds nuw i8, ptr %self, i32 176
  %675 = getelementptr inbounds nuw i8, ptr %self, i32 180
  %676 = getelementptr inbounds nuw i8, ptr %self, i32 184
  %677 = getelementptr inbounds nuw i8, ptr %self, i32 188
  %row7.i.i.i119.i = getelementptr inbounds nuw i8, ptr %self, i32 192
  %678 = getelementptr inbounds nuw i8, ptr %self, i32 196
  %679 = getelementptr inbounds nuw i8, ptr %self, i32 200
  %680 = getelementptr inbounds nuw i8, ptr %self, i32 204
  %row9.i.i.i133.i = getelementptr inbounds nuw i8, ptr %self, i32 208
  %681 = getelementptr inbounds nuw i8, ptr %self, i32 212
  %682 = getelementptr inbounds nuw i8, ptr %self, i32 216
  %683 = getelementptr inbounds nuw i8, ptr %self, i32 220
  %row11.i.i.i147.i = getelementptr inbounds nuw i8, ptr %self, i32 224
  %684 = getelementptr inbounds nuw i8, ptr %self, i32 228
  %685 = getelementptr inbounds nuw i8, ptr %self, i32 232
  %686 = getelementptr inbounds nuw i8, ptr %self, i32 236
  %row13.i.i.i161.i = getelementptr inbounds nuw i8, ptr %self, i32 240
  %687 = getelementptr inbounds nuw i8, ptr %self, i32 244
  %688 = getelementptr inbounds nuw i8, ptr %self, i32 248
  %689 = getelementptr inbounds nuw i8, ptr %self, i32 252
  %row15.i.i.i175.i = getelementptr inbounds nuw i8, ptr %self, i32 256
  %690 = getelementptr inbounds nuw i8, ptr %self, i32 260
  %691 = getelementptr inbounds nuw i8, ptr %self, i32 264
  %692 = getelementptr inbounds nuw i8, ptr %self, i32 268
  %row17.i.i.i189.i = getelementptr inbounds nuw i8, ptr %self, i32 272
  %693 = getelementptr inbounds nuw i8, ptr %self, i32 276
  %694 = getelementptr inbounds nuw i8, ptr %self, i32 280
  %695 = getelementptr inbounds nuw i8, ptr %self, i32 284
  %row19.i.i.i203.i = getelementptr inbounds nuw i8, ptr %self, i32 288
  %696 = getelementptr inbounds nuw i8, ptr %self, i32 292
  %697 = getelementptr inbounds nuw i8, ptr %self, i32 296
  %698 = getelementptr inbounds nuw i8, ptr %self, i32 300
  %row21.i.i.i217.i = getelementptr inbounds nuw i8, ptr %self, i32 304
  %699 = getelementptr inbounds nuw i8, ptr %self, i32 308
  %700 = getelementptr inbounds nuw i8, ptr %self, i32 312
  %701 = getelementptr inbounds nuw i8, ptr %self, i32 316
  %history.i.i.sroa.7.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 4
  %history.i.i.sroa.10.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 8
  %history.i.i.sroa.13.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 12
  %history.i.i.sroa.16.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 16
  %history.i.i.sroa.19.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 20
  %history.i.i.sroa.22.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 24
  %history.i.i.sroa.26.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 28
  %history.i.i.sroa.29.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 32
  %history.i.i.sroa.32.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 36
  %history.i.i.sroa.35.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 40
  %history.i.i.sroa.38.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 44
  %702 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 32
  %_72.i.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 36
  %_72.i.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 40
  %703 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 32
  %_73.i.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 36
  %_73.i.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 40
  %_114.i = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 48
  %_115.i = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 64
  %_119.i = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 48
  %_120.i = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 64
  %_9.i4010 = add nsw i32 %_32.i, -1
  %704 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 4
  %_21.i264.i = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 24
  %_22.i265.i = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 28
  %705 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 8
  %706 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 12
  %707 = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 84
  %708 = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 88
  %709 = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 80
  %710 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 20
  %711 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 16
  %_9.i3990 = add nsw i32 %_33.i, -1
  %712 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 4
  %_21.i.i = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 24
  %_22.i.i = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 28
  %713 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 8
  %714 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 12
  %715 = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 84
  %716 = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 88
  %717 = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 80
  %718 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 20
  %719 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 16
  %hot_left.i.promoted = load float, ptr %hot_left.i, align 4
  %history.i44.i.sroa.7.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4
  %history.i44.i.sroa.10.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i44.i.sroa.10.0.hot_left.i.sroa_idx, align 4
  %history.i44.i.sroa.13.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i44.i.sroa.13.0.hot_left.i.sroa_idx, align 4
  %history.i44.i.sroa.16.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i44.i.sroa.16.0.hot_left.i.sroa_idx, align 4
  %history.i44.i.sroa.19.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i44.i.sroa.19.0.hot_left.i.sroa_idx, align 4
  %history.i44.i.sroa.22.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i44.i.sroa.22.0.hot_left.i.sroa_idx, align 4
  %history.i44.i.sroa.26.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i44.i.sroa.26.0.hot_left.i.sroa_idx, align 4
  %history.i44.i.sroa.29.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i44.i.sroa.29.0.hot_left.i.sroa_idx, align 4
  %history.i44.i.sroa.32.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i44.i.sroa.32.0.hot_left.i.sroa_idx, align 4
  %history.i44.i.sroa.35.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i44.i.sroa.35.0.hot_left.i.sroa_idx, align 4
  %history.i44.i.sroa.38.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i44.i.sroa.38.0.hot_left.i.sroa_idx, align 4
  %hot_right.i.promoted = load float, ptr %hot_right.i, align 4
  %history.i.i.sroa.7.0.hot_right.i.sroa_idx.promoted = load float, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4
  %history.i.i.sroa.10.0.hot_right.i.sroa_idx.promoted = load float, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4
  %history.i.i.sroa.13.0.hot_right.i.sroa_idx.promoted = load float, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4
  %history.i.i.sroa.16.0.hot_right.i.sroa_idx.promoted = load float, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4
  %history.i.i.sroa.19.0.hot_right.i.sroa_idx.promoted = load float, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4
  %history.i.i.sroa.22.0.hot_right.i.sroa_idx.promoted = load float, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4
  %history.i.i.sroa.26.0.hot_right.i.sroa_idx.promoted = load float, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4
  %history.i.i.sroa.29.0.hot_right.i.sroa_idx.promoted = load float, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4
  %history.i.i.sroa.32.0.hot_right.i.sroa_idx.promoted = load float, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4
  %history.i.i.sroa.35.0.hot_right.i.sroa_idx.promoted = load float, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4
  %history.i.i.sroa.38.0.hot_right.i.sroa_idx.promoted = load float, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4
  %_22.i265.i.promoted = load i32, ptr %_22.i265.i, align 4
  %_21.i264.i.promoted = load float, ptr %_21.i264.i, align 4
  %_22.i.i.promoted = load i32, ptr %_22.i.i, align 4
  %_21.i.i.promoted = load float, ptr %_21.i.i, align 4
  %.promoted15528 = load float, ptr %707, align 4
  %.promoted15548 = load float, ptr %709, align 4
  %.promoted15568 = load float, ptr %715, align 4
  %.promoted15588 = load float, ptr %717, align 4
  br label %bb44.i, !dbg !23023

bb15.i.loopexit:                                  ; preds = %bb74.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i
  %_0.i3748.lcssa1277315003.lcssa15589 = phi float [ %_0.i3748.lcssa1277315003.lcssa15590, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %_0.i3748.lcssa1277315003, %bb74.i ]
  %_0.i3374.lcssa1277214985.lcssa15569 = phi float [ %_0.i3374.lcssa1277214985.lcssa15570, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %_0.i3374.lcssa1277214985, %bb74.i ]
  %_0.i3752.lcssa1274814967.lcssa15549 = phi float [ %_0.i3752.lcssa1274814967.lcssa15550, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %_0.i3752.lcssa1274814967, %bb74.i ]
  %_0.i3378.lcssa1273214949.lcssa15529 = phi float [ %_0.i3378.lcssa1273214949.lcssa15530, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %_0.i3378.lcssa1273214949, %bb74.i ]
  %running.sroa.0.0.i1719.lcssa95439721.lcssa15509 = phi float [ %running.sroa.0.0.i1719.lcssa95439721.lcssa15510, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %running.sroa.0.0.i1719.lcssa95439721, %bb74.i ]
  %storemerge.i1724.lcssa95169685.lcssa15490 = phi i32 [ %storemerge.i1724.lcssa95169685.lcssa15491, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %storemerge.i1724.lcssa95169685, %bb74.i ]
  %running.sroa.0.0.i1749.lcssa94319649.lcssa15471 = phi float [ %running.sroa.0.0.i1749.lcssa94319649.lcssa15472, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %running.sroa.0.0.i1749.lcssa94319649, %bb74.i ]
  %storemerge.i1754.lcssa94049613.lcssa15452 = phi i32 [ %storemerge.i1754.lcssa94049613.lcssa15453, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %storemerge.i1754.lcssa94049613, %bb74.i ]
  %ring_cursor.sroa.0.1.i.lcssa = phi i32 [ %ring_cursor.sroa.0.0.i9757, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %ring_cursor.sroa.0.2.i, %bb74.i ], !dbg !23042
  %main_cursor.sroa.0.1.i.lcssa = phi i32 [ %main_cursor.sroa.0.0.i9758, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %main_cursor.sroa.0.2.i, %bb74.i ], !dbg !23043
  %_166.not.i = icmp eq i32 %721, 0, !dbg !23023
  %indvars.iv.next12552 = add i32 %indvars.iv12551, -32, !dbg !23023
  br i1 %_166.not.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit, label %bb44.i, !dbg !23023

bb44.i:                                           ; preds = %bb44.i.lr.ph, %bb15.i.loopexit
  %_0.i3748.lcssa1277315003.lcssa15590 = phi float [ %.promoted15588, %bb44.i.lr.ph ], [ %_0.i3748.lcssa1277315003.lcssa15589, %bb15.i.loopexit ]
  %_0.i3374.lcssa1277214985.lcssa15570 = phi float [ %.promoted15568, %bb44.i.lr.ph ], [ %_0.i3374.lcssa1277214985.lcssa15569, %bb15.i.loopexit ]
  %_0.i3752.lcssa1274814967.lcssa15550 = phi float [ %.promoted15548, %bb44.i.lr.ph ], [ %_0.i3752.lcssa1274814967.lcssa15549, %bb15.i.loopexit ]
  %_0.i3378.lcssa1273214949.lcssa15530 = phi float [ %.promoted15528, %bb44.i.lr.ph ], [ %_0.i3378.lcssa1273214949.lcssa15529, %bb15.i.loopexit ]
  %running.sroa.0.0.i1719.lcssa95439721.lcssa15510 = phi float [ %_21.i.i.promoted, %bb44.i.lr.ph ], [ %running.sroa.0.0.i1719.lcssa95439721.lcssa15509, %bb15.i.loopexit ]
  %storemerge.i1724.lcssa95169685.lcssa15491 = phi i32 [ %_22.i.i.promoted, %bb44.i.lr.ph ], [ %storemerge.i1724.lcssa95169685.lcssa15490, %bb15.i.loopexit ]
  %running.sroa.0.0.i1749.lcssa94319649.lcssa15472 = phi float [ %_21.i264.i.promoted, %bb44.i.lr.ph ], [ %running.sroa.0.0.i1749.lcssa94319649.lcssa15471, %bb15.i.loopexit ]
  %storemerge.i1754.lcssa94049613.lcssa15453 = phi i32 [ %_22.i265.i.promoted, %bb44.i.lr.ph ], [ %storemerge.i1754.lcssa94049613.lcssa15452, %bb15.i.loopexit ]
  %history.i.i.sroa.38.0.lcssa15434 = phi float [ %history.i.i.sroa.38.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.38.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.35.0.lcssa15416 = phi float [ %history.i.i.sroa.35.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.35.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.32.0.lcssa15398 = phi float [ %history.i.i.sroa.32.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.32.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.29.0.lcssa15380 = phi float [ %history.i.i.sroa.29.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.29.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.26.0.lcssa15362 = phi float [ %history.i.i.sroa.26.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.26.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.22.0.lcssa15344 = phi float [ %history.i.i.sroa.22.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.22.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.19.0.lcssa15326 = phi float [ %history.i.i.sroa.19.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.19.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.16.0.lcssa15308 = phi float [ %history.i.i.sroa.16.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.16.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.13.0.lcssa15290 = phi float [ %history.i.i.sroa.13.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.13.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.10.0.lcssa15272 = phi float [ %history.i.i.sroa.10.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.10.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.7.0.lcssa15254 = phi float [ %history.i.i.sroa.7.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.7.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.0.0.lcssa15236 = phi float [ %hot_right.i.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i44.i.sroa.38.0.lcssa15218 = phi float [ %history.i44.i.sroa.38.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i44.i.sroa.38.0.lcssa, %bb15.i.loopexit ]
  %history.i44.i.sroa.35.0.lcssa15200 = phi float [ %history.i44.i.sroa.35.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i44.i.sroa.35.0.lcssa, %bb15.i.loopexit ]
  %history.i44.i.sroa.32.0.lcssa15182 = phi float [ %history.i44.i.sroa.32.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i44.i.sroa.32.0.lcssa, %bb15.i.loopexit ]
  %history.i44.i.sroa.29.0.lcssa15164 = phi float [ %history.i44.i.sroa.29.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i44.i.sroa.29.0.lcssa, %bb15.i.loopexit ]
  %history.i44.i.sroa.26.0.lcssa15146 = phi float [ %history.i44.i.sroa.26.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i44.i.sroa.26.0.lcssa, %bb15.i.loopexit ]
  %history.i44.i.sroa.22.0.lcssa15128 = phi float [ %history.i44.i.sroa.22.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i44.i.sroa.22.0.lcssa, %bb15.i.loopexit ]
  %history.i44.i.sroa.19.0.lcssa15110 = phi float [ %history.i44.i.sroa.19.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i44.i.sroa.19.0.lcssa, %bb15.i.loopexit ]
  %history.i44.i.sroa.16.0.lcssa15092 = phi float [ %history.i44.i.sroa.16.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i44.i.sroa.16.0.lcssa, %bb15.i.loopexit ]
  %history.i44.i.sroa.13.0.lcssa15074 = phi float [ %history.i44.i.sroa.13.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i44.i.sroa.13.0.lcssa, %bb15.i.loopexit ]
  %history.i44.i.sroa.10.0.lcssa15056 = phi float [ %history.i44.i.sroa.10.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i44.i.sroa.10.0.lcssa, %bb15.i.loopexit ]
  %history.i44.i.sroa.7.0.lcssa15038 = phi float [ %history.i44.i.sroa.7.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i44.i.sroa.7.0.lcssa, %bb15.i.loopexit ]
  %history.i44.i.sroa.0.0.lcssa15020 = phi float [ %hot_left.i.promoted, %bb44.i.lr.ph ], [ %history.i44.i.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %indvars.iv12551 = phi i32 [ %frames, %bb44.i.lr.ph ], [ %indvars.iv.next12552, %bb15.i.loopexit ]
  %iter2.sroa.0.0.i9760 = phi i32 [ %yield_count.sroa.0.0.i4824, %bb44.i.lr.ph ], [ %721, %bb15.i.loopexit ]
  %iter1.sroa.0.0.i9759 = phi i32 [ 0, %bb44.i.lr.ph ], [ %720, %bb15.i.loopexit ]
  %main_cursor.sroa.0.0.i9758 = phi i32 [ %_36.i, %bb44.i.lr.ph ], [ %main_cursor.sroa.0.1.i.lcssa, %bb15.i.loopexit ]
  %ring_cursor.sroa.0.0.i9757 = phi i32 [ %_37.i, %bb44.i.lr.ph ], [ %ring_cursor.sroa.0.1.i.lcssa, %bb15.i.loopexit ]
  %umin12571 = call i32 @llvm.umin.i32(i32 %indvars.iv12551, i32 32), !dbg !23044
  %umax12557 = call i32 @llvm.umax.i32(i32 %umin12571, i32 1), !dbg !23044
  %720 = add i32 %iter1.sroa.0.0.i9759, 32, !dbg !23044
  %721 = add nsw i32 %iter2.sroa.0.0.i9760, -1, !dbg !23048
  %722 = sub i32 %frames, %iter1.sroa.0.0.i9759, !dbg !23049
  %spec.store.select.i = tail call i32 @llvm.umin.i32(i32 %722, i32 32), !dbg !23051
  %_52.i = add i32 %spec.store.select.i, %iter1.sroa.0.0.i9759, !dbg !23056
  %_176.i = icmp ult i32 %_52.i, %iter1.sroa.0.0.i9759, !dbg !23060
  %_170.not.i = icmp ugt i32 %_52.i, %left_io.1
  %or.cond.i = or i1 %_176.i, %_170.not.i, !dbg !23060
  br i1 %or.cond.i, label %bb50.i, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4840, !dbg !23060, !prof !4596

bb50.i:                                           ; preds = %bb44.i
  store float %history.i44.i.sroa.0.0.lcssa15020, ptr %hot_left.i, align 4, !dbg !23067
  store float %history.i44.i.sroa.7.0.lcssa15038, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.10.0.lcssa15056, ptr %history.i44.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.13.0.lcssa15074, ptr %history.i44.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.16.0.lcssa15092, ptr %history.i44.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.19.0.lcssa15110, ptr %history.i44.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.22.0.lcssa15128, ptr %history.i44.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.26.0.lcssa15146, ptr %history.i44.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.29.0.lcssa15164, ptr %history.i44.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.32.0.lcssa15182, ptr %history.i44.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.35.0.lcssa15200, ptr %history.i44.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.38.0.lcssa15218, ptr %history.i44.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i.i.sroa.0.0.lcssa15236, ptr %hot_right.i, align 4, !dbg !23069
  store float %history.i.i.sroa.7.0.lcssa15254, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.10.0.lcssa15272, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.13.0.lcssa15290, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.16.0.lcssa15308, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.19.0.lcssa15326, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.22.0.lcssa15344, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.26.0.lcssa15362, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.29.0.lcssa15380, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.32.0.lcssa15398, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.35.0.lcssa15416, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.38.0.lcssa15434, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store i32 %storemerge.i1754.lcssa94049613.lcssa15453, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1749.lcssa94319649.lcssa15472, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1724.lcssa95169685.lcssa15491, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1719.lcssa95439721.lcssa15510, ptr %_21.i.i, align 4
  store float %_0.i3378.lcssa1273214949.lcssa15530, ptr %707, align 4
  store float %_0.i3752.lcssa1274814967.lcssa15550, ptr %709, align 4
  store float %_0.i3374.lcssa1277214985.lcssa15570, ptr %715, align 4
  store float %_0.i3748.lcssa1277315003.lcssa15590, ptr %717, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %iter1.sroa.0.0.i9759, i32 noundef %_52.i, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_b74a748963eaf51a410c7eb21835ee21) #32, !dbg !23071, !noalias !22984
  unreachable, !dbg !23071

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4840: ; preds = %bb44.i
  %_179.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %iter1.sroa.0.0.i9759, !dbg !23072
  tail call void @llvm.experimental.noalias.scope.decl(metadata !23076), !dbg !23079
  %_2.i48439319.not = icmp eq i32 %frames, %iter1.sroa.0.0.i9759, !dbg !23080
  br i1 %_2.i48439319.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit239.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579.lr.ph, !dbg !23080

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579.lr.ph: ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4840
  %_11.i.i.i65.i = load float, ptr %_31, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_14.i.i.i68.i = load float, ptr %666, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_17.i.i.i71.i = load float, ptr %667, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_20.i.i.i74.i = load float, ptr %668, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_25.i.i.i79.i = load float, ptr %row1.i.i.i77.i, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_28.i.i.i82.i = load float, ptr %669, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_31.i.i.i85.i = load float, ptr %670, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_34.i.i.i88.i = load float, ptr %671, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_39.i.i.i93.i = load float, ptr %row3.i.i.i91.i, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_42.i.i.i96.i = load float, ptr %672, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_45.i.i.i99.i = load float, ptr %673, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_48.i.i.i102.i = load float, ptr %674, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_53.i.i.i107.i = load float, ptr %row5.i.i.i105.i, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_56.i.i.i110.i = load float, ptr %675, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_59.i.i.i113.i = load float, ptr %676, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_62.i.i.i116.i = load float, ptr %677, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_67.i.i.i121.i = load float, ptr %row7.i.i.i119.i, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_70.i.i.i124.i = load float, ptr %678, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_73.i.i.i127.i = load float, ptr %679, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_76.i.i.i130.i = load float, ptr %680, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_81.i.i.i135.i = load float, ptr %row9.i.i.i133.i, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_84.i.i.i138.i = load float, ptr %681, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_87.i.i.i141.i = load float, ptr %682, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_90.i.i.i144.i = load float, ptr %683, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_95.i.i.i149.i = load float, ptr %row11.i.i.i147.i, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_98.i.i.i152.i = load float, ptr %684, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_101.i.i.i155.i = load float, ptr %685, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_104.i.i.i158.i = load float, ptr %686, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_109.i.i.i163.i = load float, ptr %row13.i.i.i161.i, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_112.i.i.i166.i = load float, ptr %687, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_115.i.i.i169.i = load float, ptr %688, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_118.i.i.i172.i = load float, ptr %689, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_123.i.i.i177.i = load float, ptr %row15.i.i.i175.i, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_126.i.i.i180.i = load float, ptr %690, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_129.i.i.i183.i = load float, ptr %691, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_132.i.i.i186.i = load float, ptr %692, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_137.i.i.i191.i = load float, ptr %row17.i.i.i189.i, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_140.i.i.i194.i = load float, ptr %693, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_143.i.i.i197.i = load float, ptr %694, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_146.i.i.i200.i = load float, ptr %695, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_151.i.i.i205.i = load float, ptr %row19.i.i.i203.i, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_154.i.i.i208.i = load float, ptr %696, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_157.i.i.i211.i = load float, ptr %697, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_160.i.i.i214.i = load float, ptr %698, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_165.i.i.i219.i = load float, ptr %row21.i.i.i217.i, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_168.i.i.i222.i = load float, ptr %699, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_171.i.i.i225.i = load float, ptr %700, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  %_174.i.i.i228.i = load float, ptr %701, align 4, !alias.scope !23083, !noalias !23088, !noundef !10
  br label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579, !dbg !23080

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579
  %iter.i40.i.sroa.16.09331 = phi i32 [ 0, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579.lr.ph ], [ %728, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579 ]
  %history.i44.i.sroa.35.09330 = phi float [ %history.i44.i.sroa.35.0.lcssa15200, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579.lr.ph ], [ %history.i44.i.sroa.32.09329, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579 ]
  %history.i44.i.sroa.32.09329 = phi float [ %history.i44.i.sroa.32.0.lcssa15182, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579.lr.ph ], [ %history.i44.i.sroa.29.09328, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579 ]
  %history.i44.i.sroa.29.09328 = phi float [ %history.i44.i.sroa.29.0.lcssa15164, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579.lr.ph ], [ %history.i44.i.sroa.26.09327, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579 ]
  %history.i44.i.sroa.26.09327 = phi float [ %history.i44.i.sroa.26.0.lcssa15146, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579.lr.ph ], [ %history.i44.i.sroa.22.09326, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579 ]
  %history.i44.i.sroa.22.09326 = phi float [ %history.i44.i.sroa.22.0.lcssa15128, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579.lr.ph ], [ %history.i44.i.sroa.19.09325, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579 ]
  %history.i44.i.sroa.19.09325 = phi float [ %history.i44.i.sroa.19.0.lcssa15110, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579.lr.ph ], [ %history.i44.i.sroa.16.09324, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579 ]
  %history.i44.i.sroa.16.09324 = phi float [ %history.i44.i.sroa.16.0.lcssa15092, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579.lr.ph ], [ %history.i44.i.sroa.13.09323, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579 ]
  %history.i44.i.sroa.13.09323 = phi float [ %history.i44.i.sroa.13.0.lcssa15074, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579.lr.ph ], [ %history.i44.i.sroa.10.09322, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579 ]
  %history.i44.i.sroa.10.09322 = phi float [ %history.i44.i.sroa.10.0.lcssa15056, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579.lr.ph ], [ %history.i44.i.sroa.7.09321, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579 ]
  %history.i44.i.sroa.7.09321 = phi float [ %history.i44.i.sroa.7.0.lcssa15038, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579.lr.ph ], [ %history.i44.i.sroa.0.09320, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579 ]
  %history.i44.i.sroa.0.09320 = phi float [ %history.i44.i.sroa.0.0.lcssa15020, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579.lr.ph ], [ %_0.i3577, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579 ]
  %data.i.i4850 = getelementptr inbounds nuw float, ptr %_179.i, i32 %iter.i40.i.sroa.16.09331, !dbg !23095
  %_0.i3577 = load float, ptr %data.i.i4850, align 4, !dbg !23098, !alias.scope !23100, !noalias !23103, !noundef !10
  %723 = tail call noundef float @llvm.fabs.f32(float %history.i44.i.sroa.19.09325), !dbg !23104
  %_0.i3338 = fmul float %_0.i3577, %_11.i.i.i65.i, !dbg !23107
  %_0.i2898 = fadd float %_0.i3338, 0.000000e+00, !dbg !23110
  %_0.i3337 = fmul float %_0.i3577, %_14.i.i.i68.i, !dbg !23112
  %_0.i2897 = fadd float %_0.i3337, 0.000000e+00, !dbg !23114
  %_0.i3336 = fmul float %_0.i3577, %_17.i.i.i71.i, !dbg !23116
  %_0.i2896 = fadd float %_0.i3336, 0.000000e+00, !dbg !23118
  %_0.i3335 = fmul float %_0.i3577, %_20.i.i.i74.i, !dbg !23120
  %_0.i2895 = fadd float %_0.i3335, 0.000000e+00, !dbg !23122
  %_0.i3334 = fmul float %history.i44.i.sroa.0.09320, %_25.i.i.i79.i, !dbg !23124
  %_0.i2894 = fadd float %_0.i2898, %_0.i3334, !dbg !23126
  %_0.i3333 = fmul float %history.i44.i.sroa.0.09320, %_28.i.i.i82.i, !dbg !23128
  %_0.i2893 = fadd float %_0.i2897, %_0.i3333, !dbg !23130
  %_0.i3332 = fmul float %history.i44.i.sroa.0.09320, %_31.i.i.i85.i, !dbg !23132
  %_0.i2892 = fadd float %_0.i2896, %_0.i3332, !dbg !23134
  %_0.i3331 = fmul float %history.i44.i.sroa.0.09320, %_34.i.i.i88.i, !dbg !23136
  %_0.i2891 = fadd float %_0.i2895, %_0.i3331, !dbg !23138
  %_0.i3330 = fmul float %history.i44.i.sroa.7.09321, %_39.i.i.i93.i, !dbg !23140
  %_0.i2890 = fadd float %_0.i2894, %_0.i3330, !dbg !23142
  %_0.i3329 = fmul float %history.i44.i.sroa.7.09321, %_42.i.i.i96.i, !dbg !23144
  %_0.i2889 = fadd float %_0.i2893, %_0.i3329, !dbg !23146
  %_0.i3328 = fmul float %history.i44.i.sroa.7.09321, %_45.i.i.i99.i, !dbg !23148
  %_0.i2888 = fadd float %_0.i2892, %_0.i3328, !dbg !23150
  %_0.i3327 = fmul float %history.i44.i.sroa.7.09321, %_48.i.i.i102.i, !dbg !23152
  %_0.i2887 = fadd float %_0.i2891, %_0.i3327, !dbg !23154
  %_0.i3326 = fmul float %history.i44.i.sroa.10.09322, %_53.i.i.i107.i, !dbg !23156
  %_0.i2886 = fadd float %_0.i2890, %_0.i3326, !dbg !23158
  %_0.i3325 = fmul float %history.i44.i.sroa.10.09322, %_56.i.i.i110.i, !dbg !23160
  %_0.i2885 = fadd float %_0.i2889, %_0.i3325, !dbg !23162
  %_0.i3324 = fmul float %history.i44.i.sroa.10.09322, %_59.i.i.i113.i, !dbg !23164
  %_0.i2884 = fadd float %_0.i2888, %_0.i3324, !dbg !23166
  %_0.i3323 = fmul float %history.i44.i.sroa.10.09322, %_62.i.i.i116.i, !dbg !23168
  %_0.i2883 = fadd float %_0.i2887, %_0.i3323, !dbg !23170
  %_0.i3322 = fmul float %history.i44.i.sroa.13.09323, %_67.i.i.i121.i, !dbg !23172
  %_0.i2882 = fadd float %_0.i2886, %_0.i3322, !dbg !23174
  %_0.i3321 = fmul float %history.i44.i.sroa.13.09323, %_70.i.i.i124.i, !dbg !23176
  %_0.i2881 = fadd float %_0.i2885, %_0.i3321, !dbg !23178
  %_0.i3320 = fmul float %history.i44.i.sroa.13.09323, %_73.i.i.i127.i, !dbg !23180
  %_0.i2880 = fadd float %_0.i2884, %_0.i3320, !dbg !23182
  %_0.i3319 = fmul float %history.i44.i.sroa.13.09323, %_76.i.i.i130.i, !dbg !23184
  %_0.i2879 = fadd float %_0.i2883, %_0.i3319, !dbg !23186
  %_0.i3318 = fmul float %history.i44.i.sroa.16.09324, %_81.i.i.i135.i, !dbg !23188
  %_0.i2878 = fadd float %_0.i2882, %_0.i3318, !dbg !23190
  %_0.i3317 = fmul float %history.i44.i.sroa.16.09324, %_84.i.i.i138.i, !dbg !23192
  %_0.i2877 = fadd float %_0.i2881, %_0.i3317, !dbg !23194
  %_0.i3316 = fmul float %history.i44.i.sroa.16.09324, %_87.i.i.i141.i, !dbg !23196
  %_0.i2876 = fadd float %_0.i2880, %_0.i3316, !dbg !23198
  %_0.i3315 = fmul float %history.i44.i.sroa.16.09324, %_90.i.i.i144.i, !dbg !23200
  %_0.i2875 = fadd float %_0.i2879, %_0.i3315, !dbg !23202
  %_0.i3314 = fmul float %history.i44.i.sroa.19.09325, %_95.i.i.i149.i, !dbg !23204
  %_0.i2874 = fadd float %_0.i2878, %_0.i3314, !dbg !23206
  %_0.i3313 = fmul float %history.i44.i.sroa.19.09325, %_98.i.i.i152.i, !dbg !23208
  %_0.i2873 = fadd float %_0.i2877, %_0.i3313, !dbg !23210
  %_0.i3312 = fmul float %history.i44.i.sroa.19.09325, %_101.i.i.i155.i, !dbg !23212
  %_0.i2872 = fadd float %_0.i2876, %_0.i3312, !dbg !23214
  %_0.i3311 = fmul float %history.i44.i.sroa.19.09325, %_104.i.i.i158.i, !dbg !23216
  %_0.i2871 = fadd float %_0.i2875, %_0.i3311, !dbg !23218
  %_0.i3310 = fmul float %history.i44.i.sroa.22.09326, %_109.i.i.i163.i, !dbg !23220
  %_0.i2870 = fadd float %_0.i2874, %_0.i3310, !dbg !23222
  %_0.i3309 = fmul float %history.i44.i.sroa.22.09326, %_112.i.i.i166.i, !dbg !23224
  %_0.i2869 = fadd float %_0.i2873, %_0.i3309, !dbg !23226
  %_0.i3308 = fmul float %history.i44.i.sroa.22.09326, %_115.i.i.i169.i, !dbg !23228
  %_0.i2868 = fadd float %_0.i2872, %_0.i3308, !dbg !23230
  %_0.i3307 = fmul float %history.i44.i.sroa.22.09326, %_118.i.i.i172.i, !dbg !23232
  %_0.i2867 = fadd float %_0.i2871, %_0.i3307, !dbg !23234
  %_0.i3306 = fmul float %history.i44.i.sroa.26.09327, %_123.i.i.i177.i, !dbg !23236
  %_0.i2866 = fadd float %_0.i2870, %_0.i3306, !dbg !23238
  %_0.i3305 = fmul float %history.i44.i.sroa.26.09327, %_126.i.i.i180.i, !dbg !23240
  %_0.i2865 = fadd float %_0.i2869, %_0.i3305, !dbg !23242
  %_0.i3304 = fmul float %history.i44.i.sroa.26.09327, %_129.i.i.i183.i, !dbg !23244
  %_0.i2864 = fadd float %_0.i2868, %_0.i3304, !dbg !23246
  %_0.i3303 = fmul float %history.i44.i.sroa.26.09327, %_132.i.i.i186.i, !dbg !23248
  %_0.i2863 = fadd float %_0.i2867, %_0.i3303, !dbg !23250
  %_0.i3302 = fmul float %history.i44.i.sroa.29.09328, %_137.i.i.i191.i, !dbg !23252
  %_0.i2862 = fadd float %_0.i2866, %_0.i3302, !dbg !23254
  %_0.i3301 = fmul float %history.i44.i.sroa.29.09328, %_140.i.i.i194.i, !dbg !23256
  %_0.i2861 = fadd float %_0.i2865, %_0.i3301, !dbg !23258
  %_0.i3300 = fmul float %history.i44.i.sroa.29.09328, %_143.i.i.i197.i, !dbg !23260
  %_0.i2860 = fadd float %_0.i2864, %_0.i3300, !dbg !23262
  %_0.i3299 = fmul float %history.i44.i.sroa.29.09328, %_146.i.i.i200.i, !dbg !23264
  %_0.i2859 = fadd float %_0.i2863, %_0.i3299, !dbg !23266
  %_0.i3298 = fmul float %history.i44.i.sroa.32.09329, %_151.i.i.i205.i, !dbg !23268
  %_0.i2858 = fadd float %_0.i2862, %_0.i3298, !dbg !23270
  %_0.i3297 = fmul float %history.i44.i.sroa.32.09329, %_154.i.i.i208.i, !dbg !23272
  %_0.i2857 = fadd float %_0.i2861, %_0.i3297, !dbg !23274
  %_0.i3296 = fmul float %history.i44.i.sroa.32.09329, %_157.i.i.i211.i, !dbg !23276
  %_0.i2856 = fadd float %_0.i2860, %_0.i3296, !dbg !23278
  %_0.i3295 = fmul float %history.i44.i.sroa.32.09329, %_160.i.i.i214.i, !dbg !23280
  %_0.i2855 = fadd float %_0.i2859, %_0.i3295, !dbg !23282
  %_0.i3294 = fmul float %history.i44.i.sroa.35.09330, %_165.i.i.i219.i, !dbg !23284
  %_0.i2854 = fadd float %_0.i2858, %_0.i3294, !dbg !23286
  %_0.i3293 = fmul float %history.i44.i.sroa.35.09330, %_168.i.i.i222.i, !dbg !23288
  %_0.i2853 = fadd float %_0.i2857, %_0.i3293, !dbg !23290
  %_0.i3292 = fmul float %history.i44.i.sroa.35.09330, %_171.i.i.i225.i, !dbg !23292
  %_0.i2852 = fadd float %_0.i2856, %_0.i3292, !dbg !23294
  %_0.i3291 = fmul float %history.i44.i.sroa.35.09330, %_174.i.i.i228.i, !dbg !23296
  %_0.i2851 = fadd float %_0.i2855, %_0.i3291, !dbg !23298
  %724 = tail call noundef float @llvm.fabs.f32(float %_0.i2854), !dbg !23300
  %_3.i.i4235.inv = fcmp ogt float %723, %724, !dbg !23302
  %_4.i.i4242.v = select i1 %_3.i.i4235.inv, float %723, float %724, !dbg !23302
  %725 = tail call noundef float @llvm.fabs.f32(float %_0.i2853), !dbg !23300
  %_3.i.i4235.inv.1 = fcmp ogt float %_4.i.i4242.v, %725, !dbg !23302
  %_4.i.i4242.v.1 = select i1 %_3.i.i4235.inv.1, float %_4.i.i4242.v, float %725, !dbg !23302
  %726 = tail call noundef float @llvm.fabs.f32(float %_0.i2852), !dbg !23300
  %_3.i.i4235.inv.2 = fcmp ogt float %_4.i.i4242.v.1, %726, !dbg !23302
  %_4.i.i4242.v.2 = select i1 %_3.i.i4235.inv.2, float %_4.i.i4242.v.1, float %726, !dbg !23302
  %727 = tail call noundef float @llvm.fabs.f32(float %_0.i2851), !dbg !23300
  %_3.i.i4235.inv.3 = fcmp ogt float %_4.i.i4242.v.2, %727, !dbg !23302
  %_4.i.i4242.v.3 = select i1 %_3.i.i4235.inv.3, float %_4.i.i4242.v.2, float %727, !dbg !23302
  %728 = add nuw nsw i32 %iter.i40.i.sroa.16.09331, 1, !dbg !23305
  %data.i4.i4854 = getelementptr inbounds nuw float, ptr %peaks_left.i, i32 %iter.i40.i.sroa.16.09331, !dbg !23306
  store float %_4.i.i4242.v.3, ptr %data.i4.i4854, align 4, !dbg !23309, !alias.scope !23311, !noalias !23103
  %exitcond12555.not = icmp eq i32 %728, %umax12557, !dbg !23080
  br i1 %exitcond12555.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit239.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579, !dbg !23080

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit239.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4840
  %history.i44.i.sroa.0.0.lcssa = phi float [ %history.i44.i.sroa.0.0.lcssa15020, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4840 ], [ %_0.i3577, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579 ], !dbg !23067
  %history.i44.i.sroa.7.0.lcssa = phi float [ %history.i44.i.sroa.7.0.lcssa15038, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4840 ], [ %history.i44.i.sroa.0.09320, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579 ], !dbg !23067
  %history.i44.i.sroa.10.0.lcssa = phi float [ %history.i44.i.sroa.10.0.lcssa15056, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4840 ], [ %history.i44.i.sroa.7.09321, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579 ], !dbg !23067
  %history.i44.i.sroa.13.0.lcssa = phi float [ %history.i44.i.sroa.13.0.lcssa15074, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4840 ], [ %history.i44.i.sroa.10.09322, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579 ], !dbg !23067
  %history.i44.i.sroa.16.0.lcssa = phi float [ %history.i44.i.sroa.16.0.lcssa15092, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4840 ], [ %history.i44.i.sroa.13.09323, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579 ], !dbg !23067
  %history.i44.i.sroa.19.0.lcssa = phi float [ %history.i44.i.sroa.19.0.lcssa15110, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4840 ], [ %history.i44.i.sroa.16.09324, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579 ], !dbg !23067
  %history.i44.i.sroa.22.0.lcssa = phi float [ %history.i44.i.sroa.22.0.lcssa15128, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4840 ], [ %history.i44.i.sroa.19.09325, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579 ], !dbg !23067
  %history.i44.i.sroa.26.0.lcssa = phi float [ %history.i44.i.sroa.26.0.lcssa15146, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4840 ], [ %history.i44.i.sroa.22.09326, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579 ], !dbg !23067
  %history.i44.i.sroa.29.0.lcssa = phi float [ %history.i44.i.sroa.29.0.lcssa15164, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4840 ], [ %history.i44.i.sroa.26.09327, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579 ], !dbg !23067
  %history.i44.i.sroa.32.0.lcssa = phi float [ %history.i44.i.sroa.32.0.lcssa15182, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4840 ], [ %history.i44.i.sroa.29.09328, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579 ], !dbg !23067
  %history.i44.i.sroa.35.0.lcssa = phi float [ %history.i44.i.sroa.35.0.lcssa15200, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4840 ], [ %history.i44.i.sroa.32.09329, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579 ], !dbg !23067
  %history.i44.i.sroa.38.0.lcssa = phi float [ %history.i44.i.sroa.38.0.lcssa15218, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4840 ], [ %history.i44.i.sroa.35.09330, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3579 ], !dbg !23067
  %_187.not.i = icmp ugt i32 %_52.i, %right_io.1, !dbg !23314
  br i1 %_187.not.i, label %bb56.i, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4886, !dbg !23314, !prof !787

bb56.i:                                           ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit239.i
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !23067
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.10.0.lcssa, ptr %history.i44.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.13.0.lcssa, ptr %history.i44.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.16.0.lcssa, ptr %history.i44.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.19.0.lcssa, ptr %history.i44.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.22.0.lcssa, ptr %history.i44.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.26.0.lcssa, ptr %history.i44.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.29.0.lcssa, ptr %history.i44.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.32.0.lcssa, ptr %history.i44.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.35.0.lcssa, ptr %history.i44.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.38.0.lcssa, ptr %history.i44.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i.i.sroa.0.0.lcssa15236, ptr %hot_right.i, align 4, !dbg !23069
  store float %history.i.i.sroa.7.0.lcssa15254, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.10.0.lcssa15272, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.13.0.lcssa15290, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.16.0.lcssa15308, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.19.0.lcssa15326, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.22.0.lcssa15344, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.26.0.lcssa15362, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.29.0.lcssa15380, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.32.0.lcssa15398, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.35.0.lcssa15416, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.38.0.lcssa15434, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store i32 %storemerge.i1754.lcssa94049613.lcssa15453, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1749.lcssa94319649.lcssa15472, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1724.lcssa95169685.lcssa15491, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1719.lcssa95439721.lcssa15510, ptr %_21.i.i, align 4
  store float %_0.i3378.lcssa1273214949.lcssa15530, ptr %707, align 4
  store float %_0.i3752.lcssa1274814967.lcssa15550, ptr %709, align 4
  store float %_0.i3374.lcssa1277214985.lcssa15570, ptr %715, align 4
  store float %_0.i3748.lcssa1277315003.lcssa15590, ptr %717, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %iter1.sroa.0.0.i9759, i32 noundef %_52.i, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_8715d54bb9ab90680506cd3587af9682) #32, !dbg !23318, !noalias !22984
  unreachable, !dbg !23318

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4886: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit239.i
  %_194.i = getelementptr inbounds nuw float, ptr %right_io.0, i32 %iter1.sroa.0.0.i9759, !dbg !23319
  tail call void @llvm.experimental.noalias.scope.decl(metadata !23323), !dbg !23326
  br i1 %_2.i48439319.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574.lr.ph, !dbg !23327

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574.lr.ph: ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4886
  %_11.i.i.i.i = load float, ptr %_31, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_14.i.i.i.i = load float, ptr %666, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_17.i.i.i.i = load float, ptr %667, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_20.i.i.i.i = load float, ptr %668, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_25.i.i.i.i = load float, ptr %row1.i.i.i77.i, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_28.i.i.i.i = load float, ptr %669, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_31.i.i.i.i = load float, ptr %670, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_34.i.i.i.i = load float, ptr %671, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_39.i.i.i.i = load float, ptr %row3.i.i.i91.i, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_42.i.i.i.i = load float, ptr %672, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_45.i.i.i.i = load float, ptr %673, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_48.i.i.i.i = load float, ptr %674, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_53.i.i.i.i = load float, ptr %row5.i.i.i105.i, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_56.i.i.i.i = load float, ptr %675, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_59.i.i.i.i = load float, ptr %676, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_62.i.i.i.i = load float, ptr %677, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_67.i.i.i.i = load float, ptr %row7.i.i.i119.i, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_70.i.i.i.i = load float, ptr %678, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_73.i.i.i.i = load float, ptr %679, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_76.i.i.i.i = load float, ptr %680, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_81.i.i.i.i = load float, ptr %row9.i.i.i133.i, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_84.i.i.i.i = load float, ptr %681, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_87.i.i.i.i = load float, ptr %682, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_90.i.i.i.i = load float, ptr %683, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_95.i.i.i.i = load float, ptr %row11.i.i.i147.i, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_98.i.i.i.i = load float, ptr %684, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_101.i.i.i.i = load float, ptr %685, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_104.i.i.i.i = load float, ptr %686, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_109.i.i.i.i = load float, ptr %row13.i.i.i161.i, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_112.i.i.i.i = load float, ptr %687, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_115.i.i.i.i = load float, ptr %688, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_118.i.i.i.i = load float, ptr %689, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_123.i.i.i.i = load float, ptr %row15.i.i.i175.i, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_126.i.i.i.i = load float, ptr %690, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_129.i.i.i.i = load float, ptr %691, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_132.i.i.i.i = load float, ptr %692, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_137.i.i.i.i = load float, ptr %row17.i.i.i189.i, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_140.i.i.i.i = load float, ptr %693, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_143.i.i.i.i = load float, ptr %694, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_146.i.i.i.i = load float, ptr %695, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_151.i.i.i.i = load float, ptr %row19.i.i.i203.i, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_154.i.i.i.i = load float, ptr %696, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_157.i.i.i.i = load float, ptr %697, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_160.i.i.i.i = load float, ptr %698, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_165.i.i.i.i = load float, ptr %row21.i.i.i217.i, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_168.i.i.i.i = load float, ptr %699, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_171.i.i.i.i = load float, ptr %700, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  %_174.i.i.i.i = load float, ptr %701, align 4, !alias.scope !23330, !noalias !23335, !noundef !10
  br label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574, !dbg !23327

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574
  %iter.i.i.sroa.16.09358 = phi i32 [ 0, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574.lr.ph ], [ %734, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574 ]
  %history.i.i.sroa.35.09357 = phi float [ %history.i.i.sroa.35.0.lcssa15416, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574.lr.ph ], [ %history.i.i.sroa.32.09356, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574 ]
  %history.i.i.sroa.32.09356 = phi float [ %history.i.i.sroa.32.0.lcssa15398, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574.lr.ph ], [ %history.i.i.sroa.29.09355, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574 ]
  %history.i.i.sroa.29.09355 = phi float [ %history.i.i.sroa.29.0.lcssa15380, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574.lr.ph ], [ %history.i.i.sroa.26.09354, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574 ]
  %history.i.i.sroa.26.09354 = phi float [ %history.i.i.sroa.26.0.lcssa15362, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574.lr.ph ], [ %history.i.i.sroa.22.09353, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574 ]
  %history.i.i.sroa.22.09353 = phi float [ %history.i.i.sroa.22.0.lcssa15344, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574.lr.ph ], [ %history.i.i.sroa.19.09352, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574 ]
  %history.i.i.sroa.19.09352 = phi float [ %history.i.i.sroa.19.0.lcssa15326, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574.lr.ph ], [ %history.i.i.sroa.16.09351, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574 ]
  %history.i.i.sroa.16.09351 = phi float [ %history.i.i.sroa.16.0.lcssa15308, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574.lr.ph ], [ %history.i.i.sroa.13.09350, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574 ]
  %history.i.i.sroa.13.09350 = phi float [ %history.i.i.sroa.13.0.lcssa15290, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574.lr.ph ], [ %history.i.i.sroa.10.09349, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574 ]
  %history.i.i.sroa.10.09349 = phi float [ %history.i.i.sroa.10.0.lcssa15272, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574.lr.ph ], [ %history.i.i.sroa.7.09348, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574 ]
  %history.i.i.sroa.7.09348 = phi float [ %history.i.i.sroa.7.0.lcssa15254, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574.lr.ph ], [ %history.i.i.sroa.0.09347, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574 ]
  %history.i.i.sroa.0.09347 = phi float [ %history.i.i.sroa.0.0.lcssa15236, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574.lr.ph ], [ %_0.i3572, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574 ]
  %data.i.i4896 = getelementptr inbounds nuw float, ptr %_194.i, i32 %iter.i.i.sroa.16.09358, !dbg !23342
  %_0.i3572 = load float, ptr %data.i.i4896, align 4, !dbg !23345, !alias.scope !23347, !noalias !23350, !noundef !10
  %729 = tail call noundef float @llvm.fabs.f32(float %history.i.i.sroa.19.09352), !dbg !23351
  %_0.i3290 = fmul float %_0.i3572, %_11.i.i.i.i, !dbg !23354
  %_0.i2850 = fadd float %_0.i3290, 0.000000e+00, !dbg !23357
  %_0.i3289 = fmul float %_0.i3572, %_14.i.i.i.i, !dbg !23359
  %_0.i2849 = fadd float %_0.i3289, 0.000000e+00, !dbg !23361
  %_0.i3288 = fmul float %_0.i3572, %_17.i.i.i.i, !dbg !23363
  %_0.i2848 = fadd float %_0.i3288, 0.000000e+00, !dbg !23365
  %_0.i3287 = fmul float %_0.i3572, %_20.i.i.i.i, !dbg !23367
  %_0.i2847 = fadd float %_0.i3287, 0.000000e+00, !dbg !23369
  %_0.i3286 = fmul float %history.i.i.sroa.0.09347, %_25.i.i.i.i, !dbg !23371
  %_0.i2846 = fadd float %_0.i2850, %_0.i3286, !dbg !23373
  %_0.i3285 = fmul float %history.i.i.sroa.0.09347, %_28.i.i.i.i, !dbg !23375
  %_0.i2845 = fadd float %_0.i2849, %_0.i3285, !dbg !23377
  %_0.i3284 = fmul float %history.i.i.sroa.0.09347, %_31.i.i.i.i, !dbg !23379
  %_0.i2844 = fadd float %_0.i2848, %_0.i3284, !dbg !23381
  %_0.i3283 = fmul float %history.i.i.sroa.0.09347, %_34.i.i.i.i, !dbg !23383
  %_0.i2843 = fadd float %_0.i2847, %_0.i3283, !dbg !23385
  %_0.i3282 = fmul float %history.i.i.sroa.7.09348, %_39.i.i.i.i, !dbg !23387
  %_0.i2842 = fadd float %_0.i2846, %_0.i3282, !dbg !23389
  %_0.i3281 = fmul float %history.i.i.sroa.7.09348, %_42.i.i.i.i, !dbg !23391
  %_0.i2841 = fadd float %_0.i2845, %_0.i3281, !dbg !23393
  %_0.i3280 = fmul float %history.i.i.sroa.7.09348, %_45.i.i.i.i, !dbg !23395
  %_0.i2840 = fadd float %_0.i2844, %_0.i3280, !dbg !23397
  %_0.i3279 = fmul float %history.i.i.sroa.7.09348, %_48.i.i.i.i, !dbg !23399
  %_0.i2839 = fadd float %_0.i2843, %_0.i3279, !dbg !23401
  %_0.i3278 = fmul float %history.i.i.sroa.10.09349, %_53.i.i.i.i, !dbg !23403
  %_0.i2838 = fadd float %_0.i2842, %_0.i3278, !dbg !23405
  %_0.i3277 = fmul float %history.i.i.sroa.10.09349, %_56.i.i.i.i, !dbg !23407
  %_0.i2837 = fadd float %_0.i2841, %_0.i3277, !dbg !23409
  %_0.i3276 = fmul float %history.i.i.sroa.10.09349, %_59.i.i.i.i, !dbg !23411
  %_0.i2836 = fadd float %_0.i2840, %_0.i3276, !dbg !23413
  %_0.i3275 = fmul float %history.i.i.sroa.10.09349, %_62.i.i.i.i, !dbg !23415
  %_0.i2835 = fadd float %_0.i2839, %_0.i3275, !dbg !23417
  %_0.i3274 = fmul float %history.i.i.sroa.13.09350, %_67.i.i.i.i, !dbg !23419
  %_0.i2834 = fadd float %_0.i2838, %_0.i3274, !dbg !23421
  %_0.i3273 = fmul float %history.i.i.sroa.13.09350, %_70.i.i.i.i, !dbg !23423
  %_0.i2833 = fadd float %_0.i2837, %_0.i3273, !dbg !23425
  %_0.i3272 = fmul float %history.i.i.sroa.13.09350, %_73.i.i.i.i, !dbg !23427
  %_0.i2832 = fadd float %_0.i2836, %_0.i3272, !dbg !23429
  %_0.i3271 = fmul float %history.i.i.sroa.13.09350, %_76.i.i.i.i, !dbg !23431
  %_0.i2831 = fadd float %_0.i2835, %_0.i3271, !dbg !23433
  %_0.i3270 = fmul float %history.i.i.sroa.16.09351, %_81.i.i.i.i, !dbg !23435
  %_0.i2830 = fadd float %_0.i2834, %_0.i3270, !dbg !23437
  %_0.i3269 = fmul float %history.i.i.sroa.16.09351, %_84.i.i.i.i, !dbg !23439
  %_0.i2829 = fadd float %_0.i2833, %_0.i3269, !dbg !23441
  %_0.i3268 = fmul float %history.i.i.sroa.16.09351, %_87.i.i.i.i, !dbg !23443
  %_0.i2828 = fadd float %_0.i2832, %_0.i3268, !dbg !23445
  %_0.i3267 = fmul float %history.i.i.sroa.16.09351, %_90.i.i.i.i, !dbg !23447
  %_0.i2827 = fadd float %_0.i2831, %_0.i3267, !dbg !23449
  %_0.i3266 = fmul float %history.i.i.sroa.19.09352, %_95.i.i.i.i, !dbg !23451
  %_0.i2826 = fadd float %_0.i2830, %_0.i3266, !dbg !23453
  %_0.i3265 = fmul float %history.i.i.sroa.19.09352, %_98.i.i.i.i, !dbg !23455
  %_0.i2825 = fadd float %_0.i2829, %_0.i3265, !dbg !23457
  %_0.i3264 = fmul float %history.i.i.sroa.19.09352, %_101.i.i.i.i, !dbg !23459
  %_0.i2824 = fadd float %_0.i2828, %_0.i3264, !dbg !23461
  %_0.i3263 = fmul float %history.i.i.sroa.19.09352, %_104.i.i.i.i, !dbg !23463
  %_0.i2823 = fadd float %_0.i2827, %_0.i3263, !dbg !23465
  %_0.i3262 = fmul float %history.i.i.sroa.22.09353, %_109.i.i.i.i, !dbg !23467
  %_0.i2822 = fadd float %_0.i2826, %_0.i3262, !dbg !23469
  %_0.i3261 = fmul float %history.i.i.sroa.22.09353, %_112.i.i.i.i, !dbg !23471
  %_0.i2821 = fadd float %_0.i2825, %_0.i3261, !dbg !23473
  %_0.i3260 = fmul float %history.i.i.sroa.22.09353, %_115.i.i.i.i, !dbg !23475
  %_0.i2820 = fadd float %_0.i2824, %_0.i3260, !dbg !23477
  %_0.i3259 = fmul float %history.i.i.sroa.22.09353, %_118.i.i.i.i, !dbg !23479
  %_0.i2819 = fadd float %_0.i2823, %_0.i3259, !dbg !23481
  %_0.i3258 = fmul float %history.i.i.sroa.26.09354, %_123.i.i.i.i, !dbg !23483
  %_0.i2818 = fadd float %_0.i2822, %_0.i3258, !dbg !23485
  %_0.i3257 = fmul float %history.i.i.sroa.26.09354, %_126.i.i.i.i, !dbg !23487
  %_0.i2817 = fadd float %_0.i2821, %_0.i3257, !dbg !23489
  %_0.i3256 = fmul float %history.i.i.sroa.26.09354, %_129.i.i.i.i, !dbg !23491
  %_0.i2816 = fadd float %_0.i2820, %_0.i3256, !dbg !23493
  %_0.i3255 = fmul float %history.i.i.sroa.26.09354, %_132.i.i.i.i, !dbg !23495
  %_0.i2815 = fadd float %_0.i2819, %_0.i3255, !dbg !23497
  %_0.i3254 = fmul float %history.i.i.sroa.29.09355, %_137.i.i.i.i, !dbg !23499
  %_0.i2814 = fadd float %_0.i2818, %_0.i3254, !dbg !23501
  %_0.i3253 = fmul float %history.i.i.sroa.29.09355, %_140.i.i.i.i, !dbg !23503
  %_0.i2813 = fadd float %_0.i2817, %_0.i3253, !dbg !23505
  %_0.i3252 = fmul float %history.i.i.sroa.29.09355, %_143.i.i.i.i, !dbg !23507
  %_0.i2812 = fadd float %_0.i2816, %_0.i3252, !dbg !23509
  %_0.i3251 = fmul float %history.i.i.sroa.29.09355, %_146.i.i.i.i, !dbg !23511
  %_0.i2811 = fadd float %_0.i2815, %_0.i3251, !dbg !23513
  %_0.i3250 = fmul float %history.i.i.sroa.32.09356, %_151.i.i.i.i, !dbg !23515
  %_0.i2810 = fadd float %_0.i2814, %_0.i3250, !dbg !23517
  %_0.i3249 = fmul float %history.i.i.sroa.32.09356, %_154.i.i.i.i, !dbg !23519
  %_0.i2809 = fadd float %_0.i2813, %_0.i3249, !dbg !23521
  %_0.i3248 = fmul float %history.i.i.sroa.32.09356, %_157.i.i.i.i, !dbg !23523
  %_0.i2808 = fadd float %_0.i2812, %_0.i3248, !dbg !23525
  %_0.i3247 = fmul float %history.i.i.sroa.32.09356, %_160.i.i.i.i, !dbg !23527
  %_0.i2807 = fadd float %_0.i2811, %_0.i3247, !dbg !23529
  %_0.i3246 = fmul float %history.i.i.sroa.35.09357, %_165.i.i.i.i, !dbg !23531
  %_0.i2806 = fadd float %_0.i2810, %_0.i3246, !dbg !23533
  %_0.i3245 = fmul float %history.i.i.sroa.35.09357, %_168.i.i.i.i, !dbg !23535
  %_0.i2805 = fadd float %_0.i2809, %_0.i3245, !dbg !23537
  %_0.i3244 = fmul float %history.i.i.sroa.35.09357, %_171.i.i.i.i, !dbg !23539
  %_0.i2804 = fadd float %_0.i2808, %_0.i3244, !dbg !23541
  %_0.i3243 = fmul float %history.i.i.sroa.35.09357, %_174.i.i.i.i, !dbg !23543
  %_0.i2803 = fadd float %_0.i2807, %_0.i3243, !dbg !23545
  %730 = tail call noundef float @llvm.fabs.f32(float %_0.i2806), !dbg !23547
  %_3.i.i4226.inv = fcmp ogt float %729, %730, !dbg !23549
  %_4.i.i4233.v = select i1 %_3.i.i4226.inv, float %729, float %730, !dbg !23549
  %731 = tail call noundef float @llvm.fabs.f32(float %_0.i2805), !dbg !23547
  %_3.i.i4226.inv.1 = fcmp ogt float %_4.i.i4233.v, %731, !dbg !23549
  %_4.i.i4233.v.1 = select i1 %_3.i.i4226.inv.1, float %_4.i.i4233.v, float %731, !dbg !23549
  %732 = tail call noundef float @llvm.fabs.f32(float %_0.i2804), !dbg !23547
  %_3.i.i4226.inv.2 = fcmp ogt float %_4.i.i4233.v.1, %732, !dbg !23549
  %_4.i.i4233.v.2 = select i1 %_3.i.i4226.inv.2, float %_4.i.i4233.v.1, float %732, !dbg !23549
  %733 = tail call noundef float @llvm.fabs.f32(float %_0.i2803), !dbg !23547
  %_3.i.i4226.inv.3 = fcmp ogt float %_4.i.i4233.v.2, %733, !dbg !23549
  %_4.i.i4233.v.3 = select i1 %_3.i.i4226.inv.3, float %_4.i.i4233.v.2, float %733, !dbg !23549
  %734 = add nuw nsw i32 %iter.i.i.sroa.16.09358, 1, !dbg !23552
  %data.i4.i4900 = getelementptr inbounds nuw float, ptr %peaks_right.i, i32 %iter.i.i.sroa.16.09358, !dbg !23553
  store float %_4.i.i4233.v.3, ptr %data.i4.i4900, align 4, !dbg !23556, !alias.scope !23558, !noalias !23350
  %exitcond12558.not = icmp eq i32 %734, %umax12557, !dbg !23327
  br i1 %exitcond12558.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574, !dbg !23327

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4886
  %history.i.i.sroa.0.0.lcssa = phi float [ %history.i.i.sroa.0.0.lcssa15236, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4886 ], [ %_0.i3572, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574 ], !dbg !23069
  %history.i.i.sroa.7.0.lcssa = phi float [ %history.i.i.sroa.7.0.lcssa15254, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4886 ], [ %history.i.i.sroa.0.09347, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574 ], !dbg !23069
  %history.i.i.sroa.10.0.lcssa = phi float [ %history.i.i.sroa.10.0.lcssa15272, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4886 ], [ %history.i.i.sroa.7.09348, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574 ], !dbg !23069
  %history.i.i.sroa.13.0.lcssa = phi float [ %history.i.i.sroa.13.0.lcssa15290, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4886 ], [ %history.i.i.sroa.10.09349, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574 ], !dbg !23069
  %history.i.i.sroa.16.0.lcssa = phi float [ %history.i.i.sroa.16.0.lcssa15308, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4886 ], [ %history.i.i.sroa.13.09350, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574 ], !dbg !23069
  %history.i.i.sroa.19.0.lcssa = phi float [ %history.i.i.sroa.19.0.lcssa15326, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4886 ], [ %history.i.i.sroa.16.09351, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574 ], !dbg !23069
  %history.i.i.sroa.22.0.lcssa = phi float [ %history.i.i.sroa.22.0.lcssa15344, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4886 ], [ %history.i.i.sroa.19.09352, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574 ], !dbg !23069
  %history.i.i.sroa.26.0.lcssa = phi float [ %history.i.i.sroa.26.0.lcssa15362, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4886 ], [ %history.i.i.sroa.22.09353, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574 ], !dbg !23069
  %history.i.i.sroa.29.0.lcssa = phi float [ %history.i.i.sroa.29.0.lcssa15380, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4886 ], [ %history.i.i.sroa.26.09354, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574 ], !dbg !23069
  %history.i.i.sroa.32.0.lcssa = phi float [ %history.i.i.sroa.32.0.lcssa15398, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4886 ], [ %history.i.i.sroa.29.09355, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574 ], !dbg !23069
  %history.i.i.sroa.35.0.lcssa = phi float [ %history.i.i.sroa.35.0.lcssa15416, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4886 ], [ %history.i.i.sroa.32.09356, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574 ], !dbg !23069
  %history.i.i.sroa.38.0.lcssa = phi float [ %history.i.i.sroa.38.0.lcssa15434, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4886 ], [ %history.i.i.sroa.35.09357, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3574 ], !dbg !23069
  br i1 %_2.i48439319.not, label %bb15.i.loopexit, label %bb20.i.lr.ph, !dbg !23561

bb20.i.lr.ph:                                     ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i
  %_72.i.sroa.3.0.copyload = load i32, ptr %_72.i.sroa.3.0..sroa_idx, align 4, !noalias !23013
  %_72.i.sroa.4.0.copyload = load i32, ptr %_72.i.sroa.4.0..sroa_idx, align 4, !noalias !23013
  %_73.i.sroa.3.0.copyload = load i32, ptr %_73.i.sroa.3.0..sroa_idx, align 4, !noalias !23013
  %_73.i.sroa.4.0.copyload = load i32, ptr %_73.i.sroa.4.0..sroa_idx, align 4, !noalias !23013
  %_54.0.i250.i = load ptr, ptr %uniform_left.i, align 4, !nonnull !10, !align !10173
  %_54.1.i251.i = load i32, ptr %704, align 4
  %_18.i261.i = load i32, ptr %702, align 4
  %_29.i17619371.not = icmp eq i32 %_18.i261.i, 0
  %_56.0.i272.i = load ptr, ptr %705, align 4, !nonnull !10, !align !10173
  %_56.1.i273.i = load i32, ptr %706, align 4
  %_58.1.i298.i = load i32, ptr %710, align 4
  %_58.0.i297.i = load ptr, ptr %711, align 4, !nonnull !10, !align !10173
  %_54.0.i.i = load ptr, ptr %uniform_right.i, align 4, !nonnull !10, !align !10173
  %_54.1.i.i = load i32, ptr %712, align 4
  %_18.i.i = load i32, ptr %703, align 4
  %_29.i17319375.not = icmp eq i32 %_18.i.i, 0
  %_56.0.i.i = load ptr, ptr %713, align 4, !nonnull !10, !align !10173
  %_56.1.i.i = load i32, ptr %714, align 4
  %_58.1.i.i = load i32, ptr %718, align 4
  %_58.0.i.i = load ptr, ptr %719, align 4, !nonnull !10, !align !10173
  %_8.i29.i = load float, ptr %_114.i, align 4
  %_9.i30.i = load float, ptr %_115.i, align 4
  %_8.i.i = load float, ptr %_119.i, align 4
  %_9.i.i = load float, ptr %_120.i, align 4
  %_37.i285.i = load float, ptr %708, align 4
  %_37.i.i = load float, ptr %716, align 4
  br label %bb20.i, !dbg !23561

bb20.i:                                           ; preds = %bb20.i.lr.ph, %bb74.i
  %_0.i3748.lcssa1277315004 = phi float [ %_0.i3748.lcssa1277315003.lcssa15590, %bb20.i.lr.ph ], [ %_0.i3748.lcssa1277315003, %bb74.i ]
  %_0.i3374.lcssa1277214986 = phi float [ %_0.i3374.lcssa1277214985.lcssa15570, %bb20.i.lr.ph ], [ %_0.i3374.lcssa1277214985, %bb74.i ]
  %_0.i3752.lcssa1274814968 = phi float [ %_0.i3752.lcssa1274814967.lcssa15550, %bb20.i.lr.ph ], [ %_0.i3752.lcssa1274814967, %bb74.i ]
  %_0.i3378.lcssa1273214950 = phi float [ %_0.i3378.lcssa1273214949.lcssa15530, %bb20.i.lr.ph ], [ %_0.i3378.lcssa1273214949, %bb74.i ]
  %running.sroa.0.0.i1719.lcssa95439722 = phi float [ %running.sroa.0.0.i1719.lcssa95439721.lcssa15510, %bb20.i.lr.ph ], [ %running.sroa.0.0.i1719.lcssa95439721, %bb74.i ]
  %storemerge.i1724.lcssa95169686 = phi i32 [ %storemerge.i1724.lcssa95169685.lcssa15491, %bb20.i.lr.ph ], [ %storemerge.i1724.lcssa95169685, %bb74.i ]
  %running.sroa.0.0.i1749.lcssa94319650 = phi float [ %running.sroa.0.0.i1749.lcssa94319649.lcssa15472, %bb20.i.lr.ph ], [ %running.sroa.0.0.i1749.lcssa94319649, %bb74.i ]
  %storemerge.i1754.lcssa94049614 = phi i32 [ %storemerge.i1754.lcssa94049613.lcssa15453, %bb20.i.lr.ph ], [ %storemerge.i1754.lcssa94049613, %bb74.i ]
  %frame.sroa.0.0.i9609 = phi i32 [ 0, %bb20.i.lr.ph ], [ %_87.i, %bb74.i ]
  %main_cursor.sroa.0.1.i9608 = phi i32 [ %main_cursor.sroa.0.0.i9758, %bb20.i.lr.ph ], [ %main_cursor.sroa.0.2.i, %bb74.i ]
  %ring_cursor.sroa.0.1.i9607 = phi i32 [ %ring_cursor.sroa.0.0.i9757, %bb20.i.lr.ph ], [ %ring_cursor.sroa.0.2.i, %bb74.i ]
  %_69.i = sub nuw nsw i32 %spec.store.select.i, %frame.sroa.0.0.i9609, !dbg !23563
  %ring.i2319 = load i32, ptr %662, align 4, !dbg !23564, !alias.scope !23566, !noalias !23569, !noundef !10
  %main.i2320 = load i32, ptr %663, align 4, !dbg !23573, !alias.scope !23566, !noalias !23569, !noundef !10
  %_10.i2321 = add i32 %ring_cursor.sroa.0.1.i9607, 1, !dbg !23574
  %_38.not.i2322 = icmp ult i32 %_10.i2321, %ring.i2319, !dbg !23575
  %735 = select i1 %_38.not.i2322, i32 0, i32 %ring.i2319, !dbg !23575
  %start1.sroa.0.0.i2323 = sub nuw i32 %_10.i2321, %735, !dbg !23575
  %_12.i2325 = add i32 %_72.i.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i9607, !dbg !23577
  %_39.not.i2326 = icmp ult i32 %_12.i2325, %ring.i2319, !dbg !23578
  %736 = select i1 %_39.not.i2326, i32 0, i32 %ring.i2319, !dbg !23578
  %left_end.sroa.0.0.i2327 = sub nuw i32 %_12.i2325, %736, !dbg !23578
  %_15.i2329 = add i32 %_73.i.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i9607, !dbg !23580
  %_40.not.i2330 = icmp ult i32 %_15.i2329, %ring.i2319, !dbg !23581
  %737 = select i1 %_40.not.i2330, i32 0, i32 %ring.i2319, !dbg !23581
  %right_end.sroa.0.0.i2331 = sub nuw i32 %_15.i2329, %737, !dbg !23581
  %_18.i2333 = add i32 %_72.i.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i9607, !dbg !23583
  %_41.not.i2334 = icmp ult i32 %_18.i2333, %ring.i2319, !dbg !23584
  %738 = select i1 %_41.not.i2334, i32 0, i32 %ring.i2319, !dbg !23584
  %left_expiring.sroa.0.0.i2335 = sub nuw i32 %_18.i2333, %738, !dbg !23584
  %_21.i2337 = add i32 %_73.i.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i9607, !dbg !23586
  %_42.not.i2338 = icmp ult i32 %_21.i2337, %ring.i2319, !dbg !23587
  %739 = select i1 %_42.not.i2338, i32 0, i32 %ring.i2319, !dbg !23587
  %right_expiring.sroa.0.0.i2339 = sub nuw i32 %_21.i2337, %739, !dbg !23587
  %740 = sub i32 %ring.i2319, %ring_cursor.sroa.0.1.i9607, !dbg !23589
  %spec.store.select.i2340 = tail call i32 @llvm.umin.i32(i32 %740, i32 %_69.i), !dbg !23590
  %741 = sub i32 %main.i2320, %main_cursor.sroa.0.1.i9608, !dbg !23592
  %_24.sroa.0.0.i2342 = tail call i32 @llvm.umin.i32(i32 %741, i32 %spec.store.select.i2340), !dbg !23593
  %742 = sub i32 %ring.i2319, %start1.sroa.0.0.i2323, !dbg !23595
  %_25.sroa.0.0.i2344 = tail call i32 @llvm.umin.i32(i32 %742, i32 %_24.sroa.0.0.i2342), !dbg !23596
  %743 = sub i32 %ring.i2319, %left_end.sroa.0.0.i2327, !dbg !23598
  %_27.sroa.0.0.i2346 = tail call i32 @llvm.umin.i32(i32 %743, i32 %_25.sroa.0.0.i2344), !dbg !23599
  %744 = sub i32 %ring.i2319, %right_end.sroa.0.0.i2331, !dbg !23601
  %_29.sroa.0.0.i2348 = tail call i32 @llvm.umin.i32(i32 %744, i32 %_27.sroa.0.0.i2346), !dbg !23602
  %745 = sub i32 %ring.i2319, %left_expiring.sroa.0.0.i2335, !dbg !23604
  %_31.sroa.0.0.i2350 = tail call i32 @llvm.umin.i32(i32 %745, i32 %_29.sroa.0.0.i2348), !dbg !23605
  %746 = sub i32 %ring.i2319, %right_expiring.sroa.0.0.i2339, !dbg !23607
  %run.sroa.0.0.i2352 = tail call i32 @llvm.umin.i32(i32 %746, i32 %_31.sroa.0.0.i2350), !dbg !23608
  %_76.i = add i32 %frame.sroa.0.0.i9609, %iter1.sroa.0.0.i9759, !dbg !23610
  %_80.i = add i32 %run.sroa.0.0.i2352, %_76.i, !dbg !23613
  %_203.i = icmp ult i32 %_80.i, %_76.i, !dbg !23616
  %_199.not.i = icmp ugt i32 %_80.i, %left_io.1
  %or.cond27.i = or i1 %_203.i, %_199.not.i, !dbg !23616
  br i1 %or.cond27.i, label %bb58.i, label %bb57.i, !dbg !23616, !prof !4596

bb58.i:                                           ; preds = %bb20.i
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !23067
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.10.0.lcssa, ptr %history.i44.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.13.0.lcssa, ptr %history.i44.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.16.0.lcssa, ptr %history.i44.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.19.0.lcssa, ptr %history.i44.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.22.0.lcssa, ptr %history.i44.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.26.0.lcssa, ptr %history.i44.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.29.0.lcssa, ptr %history.i44.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.32.0.lcssa, ptr %history.i44.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.35.0.lcssa, ptr %history.i44.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.38.0.lcssa, ptr %history.i44.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !23069
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store i32 %storemerge.i1754.lcssa94049613.lcssa15453, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1749.lcssa94319649.lcssa15472, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1724.lcssa95169685.lcssa15491, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1719.lcssa95439721.lcssa15510, ptr %_21.i.i, align 4
  store float %_0.i3378.lcssa1273214949.lcssa15530, ptr %707, align 4
  store float %_0.i3752.lcssa1274814967.lcssa15550, ptr %709, align 4
  store float %_0.i3374.lcssa1277214985.lcssa15570, ptr %715, align 4
  store float %_0.i3748.lcssa1277315003.lcssa15590, ptr %717, align 4
  store float %_0.i3378.lcssa1273214950, ptr %707, align 4
  store float %_0.i3752.lcssa1274814968, ptr %709, align 4
  store float %_0.i3374.lcssa1277214986, ptr %715, align 4
  store float %_0.i3748.lcssa1277315004, ptr %717, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_76.i, i32 noundef %_80.i, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb67056a871aedf25dd2ba0a06d720f) #32, !dbg !23624, !noalias !22984
  unreachable, !dbg !23624

bb57.i:                                           ; preds = %bb20.i
  %_206.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %_76.i, !dbg !23625
  %_207.not.i = icmp ugt i32 %_80.i, %right_io.1, !dbg !23629
  br i1 %_207.not.i, label %bb61.i, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4955, !dbg !23629, !prof !787

bb61.i:                                           ; preds = %bb57.i
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !23067
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.10.0.lcssa, ptr %history.i44.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.13.0.lcssa, ptr %history.i44.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.16.0.lcssa, ptr %history.i44.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.19.0.lcssa, ptr %history.i44.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.22.0.lcssa, ptr %history.i44.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.26.0.lcssa, ptr %history.i44.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.29.0.lcssa, ptr %history.i44.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.32.0.lcssa, ptr %history.i44.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.35.0.lcssa, ptr %history.i44.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.38.0.lcssa, ptr %history.i44.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !23069
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store i32 %storemerge.i1754.lcssa94049613.lcssa15453, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1749.lcssa94319649.lcssa15472, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1724.lcssa95169685.lcssa15491, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1719.lcssa95439721.lcssa15510, ptr %_21.i.i, align 4
  store float %_0.i3378.lcssa1273214949.lcssa15530, ptr %707, align 4
  store float %_0.i3752.lcssa1274814967.lcssa15550, ptr %709, align 4
  store float %_0.i3374.lcssa1277214985.lcssa15570, ptr %715, align 4
  store float %_0.i3748.lcssa1277315003.lcssa15590, ptr %717, align 4
  store float %_0.i3378.lcssa1273214950, ptr %707, align 4
  store float %_0.i3752.lcssa1274814968, ptr %709, align 4
  store float %_0.i3374.lcssa1277214986, ptr %715, align 4
  store float %_0.i3748.lcssa1277315004, ptr %717, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_76.i, i32 noundef %_80.i, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1d0fce16c93a3aa07bd2f89733f587ef) #32, !dbg !23634, !noalias !22984
  unreachable, !dbg !23634

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4955: ; preds = %bb57.i
  %_212.i = getelementptr inbounds nuw float, ptr %right_io.0, i32 %_76.i, !dbg !23635
  %_87.i = add nuw nsw i32 %run.sroa.0.0.i2352, %frame.sroa.0.0.i9609, !dbg !23639
  %_221.i = getelementptr inbounds nuw float, ptr %peaks_left.i, i32 %frame.sroa.0.0.i9609, !dbg !23641
  %_230.i = getelementptr inbounds nuw float, ptr %peaks_right.i, i32 %frame.sroa.0.0.i9609, !dbg !23650
  %_2.i49589379.not = icmp eq i32 %run.sroa.0.0.i2352, 0, !dbg !23660
  br i1 %_2.i49589379.not, label %bb74.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3554.lr.ph, !dbg !23660

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3554.lr.ph: ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4955
  %umax12561 = call i32 @llvm.umax.i32(i32 %ring_cursor.sroa.0.1.i9607, i32 %_54.1.i251.i), !dbg !23660
  %umax12562 = call i32 @llvm.umax.i32(i32 %main_cursor.sroa.0.1.i9608, i32 %_58.1.i298.i), !dbg !23660
  %747 = sub i32 %umax12561, %ring_cursor.sroa.0.1.i9607, !dbg !23660
  %748 = sub i32 %umax12562, %main_cursor.sroa.0.1.i9608, !dbg !23660
  %umin12565 = call i32 @llvm.umin.i32(i32 %743, i32 %744), !dbg !23660
  %umin12566 = call i32 @llvm.umin.i32(i32 %umin12565, i32 %745), !dbg !23660
  %umin12567 = call i32 @llvm.umin.i32(i32 %umin12566, i32 %746), !dbg !23660
  %umin12568 = call i32 @llvm.umin.i32(i32 %umin12567, i32 %742), !dbg !23660
  %umin12569 = call i32 @llvm.umin.i32(i32 %umin12568, i32 %740), !dbg !23660
  %umin12570 = call i32 @llvm.umin.i32(i32 %umin12569, i32 %741), !dbg !23660
  %749 = sub nsw i32 %umin12571, %frame.sroa.0.0.i9609, !dbg !23660
  %umin12572 = call i32 @llvm.umin.i32(i32 %umin12570, i32 %749), !dbg !23660
  br label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3554, !dbg !23660

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3554: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3554.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3703
  %_0.i37489578 = phi float [ %_0.i3748.lcssa1277315004, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3554.lr.ph ], [ %_0.i3748, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3703 ]
  %_0.i33749549 = phi float [ %_0.i3374.lcssa1277214986, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3554.lr.ph ], [ %_0.i3374, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3703 ]
  %running.sroa.0.0.i17199521 = phi float [ %running.sroa.0.0.i1719.lcssa95439722, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3554.lr.ph ], [ %running.sroa.0.0.i1719, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3703 ]
  %storemerge.i17249494 = phi i32 [ %storemerge.i1724.lcssa95169686, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3554.lr.ph ], [ %storemerge.i1724, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3703 ]
  %_0.i37529466 = phi float [ %_0.i3752.lcssa1274814968, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3554.lr.ph ], [ %_0.i3752, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3703 ]
  %_0.i33789437 = phi float [ %_0.i3378.lcssa1273214950, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3554.lr.ph ], [ %_0.i3378, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3703 ]
  %running.sroa.0.0.i17499409 = phi float [ %running.sroa.0.0.i1749.lcssa94319650, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3554.lr.ph ], [ %running.sroa.0.0.i1749, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3703 ]
  %storemerge.i17549382 = phi i32 [ %storemerge.i1754.lcssa94049614, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3554.lr.ph ], [ %storemerge.i1754, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3703 ]
  %iter.i.sroa.41.09381 = phi i32 [ 0, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3554.lr.ph ], [ %_235.0.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3703 ]
  %_235.0.i = add nuw i32 %iter.i.sroa.41.09381, 1, !dbg !23669
  %data.i.i.i.i.i.i4969 = getelementptr inbounds nuw float, ptr %_206.i, i32 %iter.i.sroa.41.09381, !dbg !23672
  %data.i5.i.i.i.i.i4973 = getelementptr inbounds nuw float, ptr %_212.i, i32 %iter.i.sroa.41.09381, !dbg !23679
  %data.i.i.i.i4978 = getelementptr inbounds nuw float, ptr %_221.i, i32 %iter.i.sroa.41.09381, !dbg !23682
  %data.i.i4983 = getelementptr inbounds nuw float, ptr %_230.i, i32 %iter.i.sroa.41.09381, !dbg !23685
  %_0.i3567 = load float, ptr %data.i.i.i.i4978, align 4, !dbg !23688, !alias.scope !23693, !noalias !22984, !noundef !10
  %_0.i3562 = load float, ptr %data.i.i4983, align 4, !dbg !23696, !alias.scope !23699, !noalias !22984, !noundef !10
  %_3.i.i4217 = fcmp ule float %_0.i3562, %_0.i3567, !dbg !23702
  %_6.i.i4219 = bitcast float %_0.i3562 to i32, !dbg !23706
  %_8.i.i4221 = bitcast float %_0.i3567 to i32, !dbg !23709
  %_4.i.i4224 = select i1 %_3.i.i4217, i32 %_8.i.i4221, i32 %_6.i.i4219, !dbg !23711
  %_5.i4008 = and i32 %_4.i.i4224, %.none.i, !dbg !23712
  %_7.i4004 = and i32 %_9.i4010, %_6.i.i4219, !dbg !23715
  %_4.i4005 = or disjoint i32 %_5.i4008, %_7.i4004, !dbg !23718
  %_0.i4006 = bitcast i32 %_4.i4005 to float, !dbg !23719
  %_0.i3557 = load float, ptr %data.i.i.i.i.i.i4969, align 4, !dbg !23721, !alias.scope !23724, !noalias !22984, !noundef !10
  %_0.i3552 = load float, ptr %data.i5.i.i.i.i.i4973, align 4, !dbg !23727, !alias.scope !23730, !noalias !22984, !noundef !10
  %_242.i = add nuw i32 %iter.i.sroa.41.09381, %ring_cursor.sroa.0.1.i9607, !dbg !23733
  %_243.i = add nuw i32 %iter.i.sroa.41.09381, %main_cursor.sroa.0.1.i9608, !dbg !23737
  %_244.i = add nuw i32 %iter.i.sroa.41.09381, %left_end.sroa.0.0.i2327, !dbg !23738
  %_245.i = add i32 %iter.i.sroa.41.09381, %start1.sroa.0.0.i2323, !dbg !23739
  %_246.i = add nuw i32 %iter.i.sroa.41.09381, %left_expiring.sroa.0.0.i2335, !dbg !23740
  %_7.i8.i253.i = add i32 %_242.i, 1, !dbg !23741
  %exitcond12563.not = icmp eq i32 %iter.i.sroa.41.09381, %747, !dbg !23744
  br i1 %exitcond12563.not, label %bb4.i13.i312.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i257.i, !dbg !23744, !prof !4596

bb4.i13.i312.i:                                   ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3554
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !23067
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.10.0.lcssa, ptr %history.i44.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.13.0.lcssa, ptr %history.i44.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.16.0.lcssa, ptr %history.i44.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.19.0.lcssa, ptr %history.i44.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.22.0.lcssa, ptr %history.i44.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.26.0.lcssa, ptr %history.i44.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.29.0.lcssa, ptr %history.i44.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.32.0.lcssa, ptr %history.i44.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.35.0.lcssa, ptr %history.i44.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.38.0.lcssa, ptr %history.i44.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !23069
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store i32 %storemerge.i1754.lcssa94049613.lcssa15453, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1749.lcssa94319649.lcssa15472, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1724.lcssa95169685.lcssa15491, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1719.lcssa95439721.lcssa15510, ptr %_21.i.i, align 4
  store float %_0.i3378.lcssa1273214949.lcssa15530, ptr %707, align 4
  store float %_0.i3752.lcssa1274814967.lcssa15550, ptr %709, align 4
  store float %_0.i3374.lcssa1277214985.lcssa15570, ptr %715, align 4
  store float %_0.i3748.lcssa1277315003.lcssa15590, ptr %717, align 4
  store float %_0.i3378.lcssa1273214950, ptr %707, align 4
  store float %_0.i3752.lcssa1274814968, ptr %709, align 4
  store float %_0.i3374.lcssa1277214986, ptr %715, align 4
  store float %_0.i3748.lcssa1277315004, ptr %717, align 4
  %750 = add i32 %umax12561, 1, !dbg !23660
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_242.i, i32 noundef %750, i32 noundef range(i32 0, 536870912) %_54.1.i251.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #32, !dbg !23748, !noalias !23749
  unreachable, !dbg !23748

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i257.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3554
  %_7.i4011 = and i32 %_9.i4010, %_8.i.i4221, !dbg !23757
  %_4.i4012 = or disjoint i32 %_5.i4008, %_7.i4011, !dbg !23712
  %_0.i4013 = bitcast i32 %_4.i4012 to float, !dbg !23758
  %_0.i2914 = fdiv float %_8.i29.i, %_0.i4013, !dbg !23760
  %_3.i2481 = fcmp uge float %_8.i29.i, %_0.i4013, !dbg !23762
  %_0.i3999 = select i1 %_3.i2481, float 1.000000e+00, float %_0.i2914, !dbg !23764
  %_17.i12.i258.i = getelementptr inbounds nuw float, ptr %_54.0.i250.i, i32 %_242.i, !dbg !23766
  store float %_0.i3999, ptr %_17.i12.i258.i, align 4, !dbg !23768, !alias.scope !23770, !noalias !23773
  %or.cond.i1796.not = icmp ult i32 %_244.i, %_54.1.i251.i, !dbg !23774
  br i1 %or.cond.i1796.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1800, label %bb4.i1799, !dbg !23774, !prof !10564

bb4.i1799:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i257.i
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !23067
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.10.0.lcssa, ptr %history.i44.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.13.0.lcssa, ptr %history.i44.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.16.0.lcssa, ptr %history.i44.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.19.0.lcssa, ptr %history.i44.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.22.0.lcssa, ptr %history.i44.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.26.0.lcssa, ptr %history.i44.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.29.0.lcssa, ptr %history.i44.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.32.0.lcssa, ptr %history.i44.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.35.0.lcssa, ptr %history.i44.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.38.0.lcssa, ptr %history.i44.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !23069
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store i32 %storemerge.i1754.lcssa94049613.lcssa15453, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1749.lcssa94319649.lcssa15472, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1724.lcssa95169685.lcssa15491, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1719.lcssa95439721.lcssa15510, ptr %_21.i.i, align 4
  store float %_0.i3378.lcssa1273214949.lcssa15530, ptr %707, align 4
  store float %_0.i3752.lcssa1274814967.lcssa15550, ptr %709, align 4
  store float %_0.i3374.lcssa1277214985.lcssa15570, ptr %715, align 4
  store float %_0.i3748.lcssa1277315003.lcssa15590, ptr %717, align 4
  store float %_0.i3378.lcssa1273214950, ptr %707, align 4
  store float %_0.i3752.lcssa1274814968, ptr %709, align 4
  store float %_0.i3374.lcssa1277214986, ptr %715, align 4
  store float %_0.i3748.lcssa1277315004, ptr %717, align 4
  %_5.i1793 = add i32 %_244.i, 1, !dbg !23780
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_244.i, i32 noundef %_5.i1793, i32 noundef range(i32 0, 536870912) %_54.1.i251.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !23781, !noalias !23782
  unreachable, !dbg !23781

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1800: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i257.i
  %_15.i1797 = getelementptr inbounds nuw float, ptr %_54.0.i250.i, i32 %_244.i, !dbg !23788
  %_0.i3420 = load float, ptr %_15.i1797, align 4, !dbg !23790, !alias.scope !23792, !noalias !23795, !noundef !10
  %751 = icmp eq i32 %storemerge.i17549382, 0, !dbg !23796
  %_3.i.i4262.inv = fcmp olt float %running.sroa.0.0.i17499409, %_0.i3420, !dbg !23796
  %_4.i.i4269.v = select i1 %_3.i.i4262.inv, float %running.sroa.0.0.i17499409, float %_0.i3420, !dbg !23796
  %running.sroa.0.0.i1749 = select i1 %751, float %_0.i3420, float %_4.i.i4269.v, !dbg !23796
  %_15.i1750 = add i32 %storemerge.i17549382, 1, !dbg !23797
  %complete.i1751 = icmp eq i32 %_15.i1750, %_18.i261.i, !dbg !23797
  br i1 %complete.i1751, label %bb11.i1757.preheader, label %bb7.i1752, !dbg !23798

bb11.i1757.preheader:                             ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1800
  br i1 %_29.i17619371.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1773, label %bb19.i1762, !dbg !23799

bb7.i1752:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1800
  %or.cond.i1788.not = icmp ult i32 %_245.i, %_54.1.i251.i, !dbg !23802
  br i1 %or.cond.i1788.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1792, label %bb4.i1791, !dbg !23802, !prof !10564

bb4.i1791:                                        ; preds = %bb7.i1752
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !23067
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.10.0.lcssa, ptr %history.i44.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.13.0.lcssa, ptr %history.i44.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.16.0.lcssa, ptr %history.i44.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.19.0.lcssa, ptr %history.i44.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.22.0.lcssa, ptr %history.i44.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.26.0.lcssa, ptr %history.i44.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.29.0.lcssa, ptr %history.i44.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.32.0.lcssa, ptr %history.i44.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.35.0.lcssa, ptr %history.i44.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.38.0.lcssa, ptr %history.i44.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !23069
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store i32 %storemerge.i1754.lcssa94049613.lcssa15453, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1749.lcssa94319649.lcssa15472, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1724.lcssa95169685.lcssa15491, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1719.lcssa95439721.lcssa15510, ptr %_21.i.i, align 4
  store float %_0.i3378.lcssa1273214949.lcssa15530, ptr %707, align 4
  store float %_0.i3752.lcssa1274814967.lcssa15550, ptr %709, align 4
  store float %_0.i3374.lcssa1277214985.lcssa15570, ptr %715, align 4
  store float %_0.i3748.lcssa1277315003.lcssa15590, ptr %717, align 4
  store float %_0.i3378.lcssa1273214950, ptr %707, align 4
  store float %_0.i3752.lcssa1274814968, ptr %709, align 4
  store float %_0.i3374.lcssa1277214986, ptr %715, align 4
  store float %_0.i3748.lcssa1277315004, ptr %717, align 4
  %_5.i1785 = add i32 %_245.i, 1, !dbg !23807
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_245.i, i32 noundef %_5.i1785, i32 noundef range(i32 0, 536870912) %_54.1.i251.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !23808, !noalias !23809
  unreachable, !dbg !23808

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1792: ; preds = %bb7.i1752
  %_15.i1789 = getelementptr inbounds nuw float, ptr %_54.0.i250.i, i32 %_245.i, !dbg !23812
  %_0.i3422 = load float, ptr %_15.i1789, align 4, !dbg !23814, !alias.scope !23816, !noalias !23795, !noundef !10
  %_3.i.i4253.inv = fcmp olt float %_0.i3422, %running.sroa.0.0.i1749, !dbg !23819
  %_4.i.i4260.v = select i1 %_3.i.i4253.inv, float %_0.i3422, float %running.sroa.0.0.i1749, !dbg !23819
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1773, !dbg !23822

bb19.i1762:                                       ; preds = %bb11.i1757.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1769
  %end.sroa.0.0.i17609374 = phi i32 [ %753, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1769 ], [ %_244.i, %bb11.i1757.preheader ]
  %suffix.sroa.0.0.i17599373 = phi float [ %_4.i.i4251.v, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1769 ], [ %_0.i3420, %bb11.i1757.preheader ]
  %iter.sroa.0.0.i17589372 = phi i32 [ %_30.i1763, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1769 ], [ 0, %bb11.i1757.preheader ]
  %or.cond.i1775.not = icmp ult i32 %end.sroa.0.0.i17609374, %_54.1.i251.i, !dbg !23823
  br i1 %or.cond.i1775.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1769, label %bb4.i, !dbg !23823, !prof !10564

bb4.i:                                            ; preds = %bb19.i1762
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !23067
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.10.0.lcssa, ptr %history.i44.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.13.0.lcssa, ptr %history.i44.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.16.0.lcssa, ptr %history.i44.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.19.0.lcssa, ptr %history.i44.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.22.0.lcssa, ptr %history.i44.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.26.0.lcssa, ptr %history.i44.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.29.0.lcssa, ptr %history.i44.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.32.0.lcssa, ptr %history.i44.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.35.0.lcssa, ptr %history.i44.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.38.0.lcssa, ptr %history.i44.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !23069
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store i32 %storemerge.i1754.lcssa94049613.lcssa15453, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1749.lcssa94319649.lcssa15472, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1724.lcssa95169685.lcssa15491, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1719.lcssa95439721.lcssa15510, ptr %_21.i.i, align 4
  store float %_0.i3378.lcssa1273214949.lcssa15530, ptr %707, align 4
  store float %_0.i3752.lcssa1274814967.lcssa15550, ptr %709, align 4
  store float %_0.i3374.lcssa1277214985.lcssa15570, ptr %715, align 4
  store float %_0.i3748.lcssa1277315003.lcssa15590, ptr %717, align 4
  store float %_0.i3378.lcssa1273214950, ptr %707, align 4
  store float %_0.i3752.lcssa1274814968, ptr %709, align 4
  store float %_0.i3374.lcssa1277214986, ptr %715, align 4
  store float %_0.i3748.lcssa1277315004, ptr %717, align 4
  %_5.i = add i32 %end.sroa.0.0.i17609374, 1, !dbg !23828
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %end.sroa.0.0.i17609374, i32 noundef %_5.i, i32 noundef range(i32 0, 536870912) %_54.1.i251.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !23829, !noalias !23830
  unreachable, !dbg !23829

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1769: ; preds = %bb19.i1762
  %_30.i1763 = add nuw i32 %iter.sroa.0.0.i17589372, 1, !dbg !23833
  %_15.i1776 = getelementptr inbounds nuw float, ptr %_54.0.i250.i, i32 %end.sroa.0.0.i17609374, !dbg !23836
  %_0.i3426 = load float, ptr %_15.i1776, align 4, !dbg !23838, !alias.scope !23840, !noalias !23795, !noundef !10
  %_3.i.i4244.inv = fcmp olt float %suffix.sroa.0.0.i17599373, %_0.i3426, !dbg !23843
  %_4.i.i4251.v = select i1 %_3.i.i4244.inv, float %suffix.sroa.0.0.i17599373, float %_0.i3426, !dbg !23843
  store float %_4.i.i4251.v, ptr %_15.i1776, align 4, !dbg !23846, !alias.scope !23849, !noalias !23795
  %752 = icmp eq i32 %end.sroa.0.0.i17609374, 0, !dbg !23852
  %spec.store.select.i1771 = select i1 %752, i32 %ring.i, i32 %end.sroa.0.0.i17609374, !dbg !23852
  %753 = add i32 %spec.store.select.i1771, -1, !dbg !23853
  %exitcond12559.not = icmp eq i32 %_30.i1763, %_18.i261.i, !dbg !23854
  br i1 %exitcond12559.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1773, label %bb19.i1762, !dbg !23799

_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1773: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1769, %bb11.i1757.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1792
  %storemerge.i1754 = phi i32 [ %_15.i1750, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1792 ], [ 0, %bb11.i1757.preheader ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1769 ], !dbg !23856
  %running.sroa.0.1.i1755 = phi float [ %_4.i.i4260.v, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1792 ], [ %running.sroa.0.0.i1749, %bb11.i1757.preheader ], [ %running.sroa.0.0.i1749, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1769 ], !dbg !23857
  %_0.i3242 = fmul float %running.sroa.0.1.i1755, 1.638400e+04, !dbg !23858
  %754 = tail call noundef float @llvm.floor.f32(float %_0.i3242), !dbg !23860
  %_0.i3241 = fmul float %754, 0x3F10000000000000, !dbg !23864
  %or.cond.i1956.not = icmp ult i32 %_246.i, %_56.1.i273.i, !dbg !23866
  br i1 %or.cond.i1956.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1960, label %bb4.i1959, !dbg !23866, !prof !10564

bb4.i1959:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1773
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !23067
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.10.0.lcssa, ptr %history.i44.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.13.0.lcssa, ptr %history.i44.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.16.0.lcssa, ptr %history.i44.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.19.0.lcssa, ptr %history.i44.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.22.0.lcssa, ptr %history.i44.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.26.0.lcssa, ptr %history.i44.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.29.0.lcssa, ptr %history.i44.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.32.0.lcssa, ptr %history.i44.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.35.0.lcssa, ptr %history.i44.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.38.0.lcssa, ptr %history.i44.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !23069
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store i32 %storemerge.i1754.lcssa94049613.lcssa15453, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1749.lcssa94319649.lcssa15472, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1724.lcssa95169685.lcssa15491, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1719.lcssa95439721.lcssa15510, ptr %_21.i.i, align 4
  store float %_0.i3378.lcssa1273214949.lcssa15530, ptr %707, align 4
  store float %_0.i3752.lcssa1274814967.lcssa15550, ptr %709, align 4
  store float %_0.i3374.lcssa1277214985.lcssa15570, ptr %715, align 4
  store float %_0.i3748.lcssa1277315003.lcssa15590, ptr %717, align 4
  store float %_0.i3378.lcssa1273214950, ptr %707, align 4
  store float %_0.i3752.lcssa1274814968, ptr %709, align 4
  store float %_0.i3374.lcssa1277214986, ptr %715, align 4
  store float %_0.i3748.lcssa1277315004, ptr %717, align 4
  %_5.i1953 = add i32 %_246.i, 1, !dbg !23871
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_246.i, i32 noundef %_5.i1953, i32 noundef range(i32 0, 536870912) %_56.1.i273.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !23872, !noalias !23873
  unreachable, !dbg !23872

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1960: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1773
  %_15.i1957 = getelementptr inbounds nuw float, ptr %_56.0.i272.i, i32 %_246.i, !dbg !23876
  %_0.i3380 = load float, ptr %_15.i1957, align 4, !dbg !23878, !alias.scope !23880, !noalias !23883, !noundef !10
  %_0.i2802 = fadd float %_0.i3241, %_0.i33789437, !dbg !23884
  %_0.i3378 = fsub float %_0.i2802, %_0.i3380, !dbg !23886
  %_8.not.i3.i282.i = icmp ugt i32 %_7.i8.i253.i, %_56.1.i273.i
  br i1 %_8.not.i3.i282.i, label %bb4.i6.i311.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i283.i, !dbg !23888, !prof !4596

bb4.i6.i311.i:                                    ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1960
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !23067
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.10.0.lcssa, ptr %history.i44.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.13.0.lcssa, ptr %history.i44.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.16.0.lcssa, ptr %history.i44.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.19.0.lcssa, ptr %history.i44.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.22.0.lcssa, ptr %history.i44.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.26.0.lcssa, ptr %history.i44.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.29.0.lcssa, ptr %history.i44.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.32.0.lcssa, ptr %history.i44.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.35.0.lcssa, ptr %history.i44.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.38.0.lcssa, ptr %history.i44.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !23069
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store i32 %storemerge.i1754.lcssa94049613.lcssa15453, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1749.lcssa94319649.lcssa15472, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1724.lcssa95169685.lcssa15491, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1719.lcssa95439721.lcssa15510, ptr %_21.i.i, align 4
  store float %_0.i3378.lcssa1273214949.lcssa15530, ptr %707, align 4
  store float %_0.i3752.lcssa1274814967.lcssa15550, ptr %709, align 4
  store float %_0.i3374.lcssa1277214985.lcssa15570, ptr %715, align 4
  store float %_0.i3748.lcssa1277315003.lcssa15590, ptr %717, align 4
  store float %_0.i3378.lcssa1273214950, ptr %707, align 4
  store float %_0.i3752.lcssa1274814968, ptr %709, align 4
  store float %_0.i3374.lcssa1277214986, ptr %715, align 4
  store float %_0.i3748.lcssa1277315004, ptr %717, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_242.i, i32 noundef %_7.i8.i253.i, i32 noundef range(i32 0, 536870912) %_56.1.i273.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #32, !dbg !23893, !noalias !23894
  unreachable, !dbg !23893

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i283.i: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1960
  %_17.i5.i284.i = getelementptr inbounds nuw float, ptr %_56.0.i272.i, i32 %_242.i, !dbg !23897
  store float %_0.i3241, ptr %_17.i5.i284.i, align 4, !dbg !23899, !alias.scope !23901, !noalias !23883
  %_0.i2913 = fdiv float %_0.i3378, %_37.i285.i, !dbg !23904
  %_0.i3377 = fsub float 1.000000e+00, %_0.i2913, !dbg !23906
  %_0.i3376 = fsub float %_0.i3377, %_0.i37529466, !dbg !23908
  %_4.i2929 = fmul float %_9.i30.i, %_0.i3376, !dbg !23910
  %_0.i2930 = fadd float %_0.i37529466, %_4.i2929, !dbg !23910
  %_3.i.i4208.inv = fcmp ogt float %_0.i3377, %_0.i2930, !dbg !23912
  %_4.i.i4215.v = select i1 %_3.i.i4208.inv, float %_0.i3377, float %_0.i2930, !dbg !23912
  %755 = tail call noundef float @llvm.fabs.f32(float %_4.i.i4215.v), !dbg !23915
  %756 = fcmp uge float %755, 0x3BC79CA100000000, !dbg !23918
  %_0.i3752 = select i1 %756, float %_4.i.i4215.v, float 0.000000e+00, !dbg !23920
  %_5.i1945 = add i32 %_243.i, 1, !dbg !23921
  %exitcond12564.not = icmp eq i32 %iter.i.sroa.41.09381, %748, !dbg !23923
  br i1 %exitcond12564.not, label %bb4.i1951, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3710, !dbg !23923, !prof !4596

bb4.i1951:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i283.i
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !23067
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.10.0.lcssa, ptr %history.i44.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.13.0.lcssa, ptr %history.i44.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.16.0.lcssa, ptr %history.i44.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.19.0.lcssa, ptr %history.i44.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.22.0.lcssa, ptr %history.i44.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.26.0.lcssa, ptr %history.i44.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.29.0.lcssa, ptr %history.i44.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.32.0.lcssa, ptr %history.i44.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.35.0.lcssa, ptr %history.i44.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.38.0.lcssa, ptr %history.i44.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !23069
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store i32 %storemerge.i1754.lcssa94049613.lcssa15453, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1749.lcssa94319649.lcssa15472, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1724.lcssa95169685.lcssa15491, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1719.lcssa95439721.lcssa15510, ptr %_21.i.i, align 4
  store float %_0.i3378.lcssa1273214949.lcssa15530, ptr %707, align 4
  store float %_0.i3752.lcssa1274814967.lcssa15550, ptr %709, align 4
  store float %_0.i3374.lcssa1277214985.lcssa15570, ptr %715, align 4
  store float %_0.i3748.lcssa1277315003.lcssa15590, ptr %717, align 4
  store float %_0.i3378.lcssa1273214950, ptr %707, align 4
  store float %_0.i3752.lcssa1274814968, ptr %709, align 4
  store float %_0.i3374.lcssa1277214986, ptr %715, align 4
  store float %_0.i3748.lcssa1277315004, ptr %717, align 4
  %757 = add i32 %umax12562, 1, !dbg !23660
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_243.i, i32 noundef %757, i32 noundef range(i32 0, 536870912) %_58.1.i298.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !23927, !noalias !23928
  unreachable, !dbg !23927

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3710: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i283.i
  %_0.i3375 = fsub float 1.000000e+00, %_0.i3752, !dbg !23931
  %_15.i1949 = getelementptr inbounds nuw float, ptr %_58.0.i297.i, i32 %_243.i, !dbg !23933
  %_0.i3382 = load float, ptr %_15.i1949, align 4, !dbg !23935, !alias.scope !23937, !noalias !23883, !noundef !10
  store float %_0.i3557, ptr %_15.i1949, align 4, !dbg !23940, !alias.scope !23943, !noalias !23883
  %_0.i3240 = fmul float %_0.i3375, %_0.i3382, !dbg !23946
  %_6.i3987 = bitcast float %_0.i3382 to i32, !dbg !23948
  %_5.i3988 = and i32 %_6.i3987, %all.sroa.0.0.i, !dbg !23951
  %_8.i3989 = bitcast float %_0.i3240 to i32, !dbg !23952
  %_7.i3991 = and i32 %_9.i3990, %_8.i3989, !dbg !23954
  %_4.i3992 = or disjoint i32 %_7.i3991, %_5.i3988, !dbg !23951
  store i32 %_4.i3992, ptr %data.i.i.i.i.i.i4969, align 4, !dbg !23955, !alias.scope !23957, !noalias !23960
  %_249.i = add nuw i32 %iter.i.sroa.41.09381, %right_end.sroa.0.0.i2331, !dbg !23961
  %_251.i = add nuw i32 %iter.i.sroa.41.09381, %right_expiring.sroa.0.0.i2339, !dbg !23963
  %_8.not.i10.i.i = icmp ugt i32 %_7.i8.i253.i, %_54.1.i.i
  br i1 %_8.not.i10.i.i, label %bb4.i13.i.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i, !dbg !23964, !prof !4596

bb4.i13.i.i:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3710
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !23067
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.10.0.lcssa, ptr %history.i44.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.13.0.lcssa, ptr %history.i44.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.16.0.lcssa, ptr %history.i44.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.19.0.lcssa, ptr %history.i44.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.22.0.lcssa, ptr %history.i44.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.26.0.lcssa, ptr %history.i44.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.29.0.lcssa, ptr %history.i44.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.32.0.lcssa, ptr %history.i44.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.35.0.lcssa, ptr %history.i44.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.38.0.lcssa, ptr %history.i44.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !23069
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store i32 %storemerge.i1754.lcssa94049613.lcssa15453, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1749.lcssa94319649.lcssa15472, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1724.lcssa95169685.lcssa15491, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1719.lcssa95439721.lcssa15510, ptr %_21.i.i, align 4
  store float %_0.i3378.lcssa1273214949.lcssa15530, ptr %707, align 4
  store float %_0.i3752.lcssa1274814967.lcssa15550, ptr %709, align 4
  store float %_0.i3374.lcssa1277214985.lcssa15570, ptr %715, align 4
  store float %_0.i3748.lcssa1277315003.lcssa15590, ptr %717, align 4
  store float %_0.i3378.lcssa1273214950, ptr %707, align 4
  store float %_0.i3752.lcssa1274814968, ptr %709, align 4
  store float %_0.i3374.lcssa1277214986, ptr %715, align 4
  store float %_0.i3748.lcssa1277315004, ptr %717, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_242.i, i32 noundef %_7.i8.i253.i, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #32, !dbg !23970, !noalias !23971
  unreachable, !dbg !23970

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3710
  %_0.i2912 = fdiv float %_8.i.i, %_0.i4006, !dbg !23979
  %_3.i2479 = fcmp uge float %_8.i.i, %_0.i4006, !dbg !23981
  %_0.i3986 = select i1 %_3.i2479, float 1.000000e+00, float %_0.i2912, !dbg !23983
  %_17.i12.i.i = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %_242.i, !dbg !23985
  store float %_0.i3986, ptr %_17.i12.i.i, align 4, !dbg !23987, !alias.scope !23989, !noalias !23992
  %or.cond.i1828.not = icmp ult i32 %_249.i, %_54.1.i.i, !dbg !23993
  br i1 %or.cond.i1828.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1832, label %bb4.i1831, !dbg !23993, !prof !10564

bb4.i1831:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !23067
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.10.0.lcssa, ptr %history.i44.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.13.0.lcssa, ptr %history.i44.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.16.0.lcssa, ptr %history.i44.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.19.0.lcssa, ptr %history.i44.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.22.0.lcssa, ptr %history.i44.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.26.0.lcssa, ptr %history.i44.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.29.0.lcssa, ptr %history.i44.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.32.0.lcssa, ptr %history.i44.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.35.0.lcssa, ptr %history.i44.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.38.0.lcssa, ptr %history.i44.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !23069
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store i32 %storemerge.i1754.lcssa94049613.lcssa15453, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1749.lcssa94319649.lcssa15472, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1724.lcssa95169685.lcssa15491, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1719.lcssa95439721.lcssa15510, ptr %_21.i.i, align 4
  store float %_0.i3378.lcssa1273214949.lcssa15530, ptr %707, align 4
  store float %_0.i3752.lcssa1274814967.lcssa15550, ptr %709, align 4
  store float %_0.i3374.lcssa1277214985.lcssa15570, ptr %715, align 4
  store float %_0.i3748.lcssa1277315003.lcssa15590, ptr %717, align 4
  store float %_0.i3378.lcssa1273214950, ptr %707, align 4
  store float %_0.i3752.lcssa1274814968, ptr %709, align 4
  store float %_0.i3374.lcssa1277214986, ptr %715, align 4
  store float %_0.i3748.lcssa1277315004, ptr %717, align 4
  %_5.i1825 = add i32 %_249.i, 1, !dbg !23999
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_249.i, i32 noundef %_5.i1825, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !24000, !noalias !24001
  unreachable, !dbg !24000

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1832: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i
  %_15.i1829 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %_249.i, !dbg !24007
  %_0.i3412 = load float, ptr %_15.i1829, align 4, !dbg !24009, !alias.scope !24011, !noalias !24014, !noundef !10
  %758 = icmp eq i32 %storemerge.i17249494, 0, !dbg !24015
  %_3.i.i4289.inv = fcmp olt float %running.sroa.0.0.i17199521, %_0.i3412, !dbg !24015
  %_4.i.i4296.v = select i1 %_3.i.i4289.inv, float %running.sroa.0.0.i17199521, float %_0.i3412, !dbg !24015
  %running.sroa.0.0.i1719 = select i1 %758, float %_0.i3412, float %_4.i.i4296.v, !dbg !24015
  %_15.i1720 = add i32 %storemerge.i17249494, 1, !dbg !24016
  %complete.i1721 = icmp eq i32 %_15.i1720, %_18.i.i, !dbg !24016
  br i1 %complete.i1721, label %bb11.i1727.preheader, label %bb7.i1722, !dbg !24017

bb11.i1727.preheader:                             ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1832
  br i1 %_29.i17319375.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1743, label %bb19.i1732, !dbg !24018

bb7.i1722:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1832
  %or.cond.i1820.not = icmp ult i32 %_245.i, %_54.1.i.i, !dbg !24021
  br i1 %or.cond.i1820.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1824, label %bb4.i1823, !dbg !24021, !prof !10564

bb4.i1823:                                        ; preds = %bb7.i1722
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !23067
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.10.0.lcssa, ptr %history.i44.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.13.0.lcssa, ptr %history.i44.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.16.0.lcssa, ptr %history.i44.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.19.0.lcssa, ptr %history.i44.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.22.0.lcssa, ptr %history.i44.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.26.0.lcssa, ptr %history.i44.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.29.0.lcssa, ptr %history.i44.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.32.0.lcssa, ptr %history.i44.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.35.0.lcssa, ptr %history.i44.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.38.0.lcssa, ptr %history.i44.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !23069
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store i32 %storemerge.i1754.lcssa94049613.lcssa15453, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1749.lcssa94319649.lcssa15472, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1724.lcssa95169685.lcssa15491, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1719.lcssa95439721.lcssa15510, ptr %_21.i.i, align 4
  store float %_0.i3378.lcssa1273214949.lcssa15530, ptr %707, align 4
  store float %_0.i3752.lcssa1274814967.lcssa15550, ptr %709, align 4
  store float %_0.i3374.lcssa1277214985.lcssa15570, ptr %715, align 4
  store float %_0.i3748.lcssa1277315003.lcssa15590, ptr %717, align 4
  store float %_0.i3378.lcssa1273214950, ptr %707, align 4
  store float %_0.i3752.lcssa1274814968, ptr %709, align 4
  store float %_0.i3374.lcssa1277214986, ptr %715, align 4
  store float %_0.i3748.lcssa1277315004, ptr %717, align 4
  %_5.i1817 = add i32 %_245.i, 1, !dbg !24026
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_245.i, i32 noundef %_5.i1817, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !24027, !noalias !24028
  unreachable, !dbg !24027

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1824: ; preds = %bb7.i1722
  %_15.i1821 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %_245.i, !dbg !24031
  %_0.i3414 = load float, ptr %_15.i1821, align 4, !dbg !24033, !alias.scope !24035, !noalias !24014, !noundef !10
  %_3.i.i4280.inv = fcmp olt float %_0.i3414, %running.sroa.0.0.i1719, !dbg !24038
  %_4.i.i4287.v = select i1 %_3.i.i4280.inv, float %_0.i3414, float %running.sroa.0.0.i1719, !dbg !24038
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1743, !dbg !24041

bb19.i1732:                                       ; preds = %bb11.i1727.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1739
  %end.sroa.0.0.i17309378 = phi i32 [ %760, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1739 ], [ %_249.i, %bb11.i1727.preheader ]
  %suffix.sroa.0.0.i17299377 = phi float [ %_4.i.i4278.v, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1739 ], [ %_0.i3412, %bb11.i1727.preheader ]
  %iter.sroa.0.0.i17289376 = phi i32 [ %_30.i1733, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1739 ], [ 0, %bb11.i1727.preheader ]
  %or.cond.i1804.not = icmp ult i32 %end.sroa.0.0.i17309378, %_54.1.i.i, !dbg !24042
  br i1 %or.cond.i1804.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1739, label %bb4.i1807, !dbg !24042, !prof !10564

bb4.i1807:                                        ; preds = %bb19.i1732
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !23067
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.10.0.lcssa, ptr %history.i44.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.13.0.lcssa, ptr %history.i44.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.16.0.lcssa, ptr %history.i44.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.19.0.lcssa, ptr %history.i44.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.22.0.lcssa, ptr %history.i44.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.26.0.lcssa, ptr %history.i44.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.29.0.lcssa, ptr %history.i44.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.32.0.lcssa, ptr %history.i44.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.35.0.lcssa, ptr %history.i44.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.38.0.lcssa, ptr %history.i44.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !23069
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store i32 %storemerge.i1754.lcssa94049613.lcssa15453, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1749.lcssa94319649.lcssa15472, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1724.lcssa95169685.lcssa15491, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1719.lcssa95439721.lcssa15510, ptr %_21.i.i, align 4
  store float %_0.i3378.lcssa1273214949.lcssa15530, ptr %707, align 4
  store float %_0.i3752.lcssa1274814967.lcssa15550, ptr %709, align 4
  store float %_0.i3374.lcssa1277214985.lcssa15570, ptr %715, align 4
  store float %_0.i3748.lcssa1277315003.lcssa15590, ptr %717, align 4
  store float %_0.i3378.lcssa1273214950, ptr %707, align 4
  store float %_0.i3752.lcssa1274814968, ptr %709, align 4
  store float %_0.i3374.lcssa1277214986, ptr %715, align 4
  store float %_0.i3748.lcssa1277315004, ptr %717, align 4
  %_5.i1801 = add i32 %end.sroa.0.0.i17309378, 1, !dbg !24047
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %end.sroa.0.0.i17309378, i32 noundef %_5.i1801, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !24048, !noalias !24049
  unreachable, !dbg !24048

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1739: ; preds = %bb19.i1732
  %_30.i1733 = add nuw i32 %iter.sroa.0.0.i17289376, 1, !dbg !24052
  %_15.i1805 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %end.sroa.0.0.i17309378, !dbg !24055
  %_0.i3418 = load float, ptr %_15.i1805, align 4, !dbg !24057, !alias.scope !24059, !noalias !24014, !noundef !10
  %_3.i.i4271.inv = fcmp olt float %suffix.sroa.0.0.i17299377, %_0.i3418, !dbg !24062
  %_4.i.i4278.v = select i1 %_3.i.i4271.inv, float %suffix.sroa.0.0.i17299377, float %_0.i3418, !dbg !24062
  store float %_4.i.i4278.v, ptr %_15.i1805, align 4, !dbg !24065, !alias.scope !24068, !noalias !24014
  %759 = icmp eq i32 %end.sroa.0.0.i17309378, 0, !dbg !24071
  %spec.store.select.i1741 = select i1 %759, i32 %ring.i, i32 %end.sroa.0.0.i17309378, !dbg !24071
  %760 = add i32 %spec.store.select.i1741, -1, !dbg !24072
  %exitcond12560.not = icmp eq i32 %_30.i1733, %_18.i.i, !dbg !24073
  br i1 %exitcond12560.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1743, label %bb19.i1732, !dbg !24018

_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1743: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1739, %bb11.i1727.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1824
  %storemerge.i1724 = phi i32 [ %_15.i1720, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1824 ], [ 0, %bb11.i1727.preheader ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1739 ], !dbg !24075
  %running.sroa.0.1.i1725 = phi float [ %_4.i.i4287.v, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1824 ], [ %running.sroa.0.0.i1719, %bb11.i1727.preheader ], [ %running.sroa.0.0.i1719, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1739 ], !dbg !24076
  %_0.i3239 = fmul float %running.sroa.0.1.i1725, 1.638400e+04, !dbg !24077
  %761 = tail call noundef float @llvm.floor.f32(float %_0.i3239), !dbg !24079
  %_0.i3238 = fmul float %761, 0x3F10000000000000, !dbg !24083
  %or.cond.i1940.not = icmp ult i32 %_251.i, %_56.1.i.i, !dbg !24085
  br i1 %or.cond.i1940.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1944, label %bb4.i1943, !dbg !24085, !prof !10564

bb4.i1943:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1743
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !23067
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.10.0.lcssa, ptr %history.i44.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.13.0.lcssa, ptr %history.i44.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.16.0.lcssa, ptr %history.i44.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.19.0.lcssa, ptr %history.i44.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.22.0.lcssa, ptr %history.i44.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.26.0.lcssa, ptr %history.i44.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.29.0.lcssa, ptr %history.i44.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.32.0.lcssa, ptr %history.i44.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.35.0.lcssa, ptr %history.i44.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.38.0.lcssa, ptr %history.i44.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !23069
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store i32 %storemerge.i1754.lcssa94049613.lcssa15453, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1749.lcssa94319649.lcssa15472, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1724.lcssa95169685.lcssa15491, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1719.lcssa95439721.lcssa15510, ptr %_21.i.i, align 4
  store float %_0.i3378.lcssa1273214949.lcssa15530, ptr %707, align 4
  store float %_0.i3752.lcssa1274814967.lcssa15550, ptr %709, align 4
  store float %_0.i3374.lcssa1277214985.lcssa15570, ptr %715, align 4
  store float %_0.i3748.lcssa1277315003.lcssa15590, ptr %717, align 4
  store float %_0.i3378.lcssa1273214950, ptr %707, align 4
  store float %_0.i3752.lcssa1274814968, ptr %709, align 4
  store float %_0.i3374.lcssa1277214986, ptr %715, align 4
  store float %_0.i3748.lcssa1277315004, ptr %717, align 4
  %_5.i1937 = add i32 %_251.i, 1, !dbg !24090
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_251.i, i32 noundef %_5.i1937, i32 noundef range(i32 0, 536870912) %_56.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !24091, !noalias !24092
  unreachable, !dbg !24091

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1944: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1743
  %_15.i1941 = getelementptr inbounds nuw float, ptr %_56.0.i.i, i32 %_251.i, !dbg !24095
  %_0.i3384 = load float, ptr %_15.i1941, align 4, !dbg !24097, !alias.scope !24099, !noalias !24102, !noundef !10
  %_0.i2801 = fadd float %_0.i3238, %_0.i33749549, !dbg !24103
  %_0.i3374 = fsub float %_0.i2801, %_0.i3384, !dbg !24105
  %_8.not.i3.i.i = icmp ugt i32 %_7.i8.i253.i, %_56.1.i.i
  br i1 %_8.not.i3.i.i, label %bb4.i6.i.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i, !dbg !24107, !prof !4596

bb4.i6.i.i:                                       ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1944
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !23067
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.10.0.lcssa, ptr %history.i44.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.13.0.lcssa, ptr %history.i44.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.16.0.lcssa, ptr %history.i44.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.19.0.lcssa, ptr %history.i44.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.22.0.lcssa, ptr %history.i44.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.26.0.lcssa, ptr %history.i44.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.29.0.lcssa, ptr %history.i44.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.32.0.lcssa, ptr %history.i44.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.35.0.lcssa, ptr %history.i44.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.38.0.lcssa, ptr %history.i44.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !23069
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store i32 %storemerge.i1754.lcssa94049613.lcssa15453, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1749.lcssa94319649.lcssa15472, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1724.lcssa95169685.lcssa15491, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1719.lcssa95439721.lcssa15510, ptr %_21.i.i, align 4
  store float %_0.i3378.lcssa1273214949.lcssa15530, ptr %707, align 4
  store float %_0.i3752.lcssa1274814967.lcssa15550, ptr %709, align 4
  store float %_0.i3374.lcssa1277214985.lcssa15570, ptr %715, align 4
  store float %_0.i3748.lcssa1277315003.lcssa15590, ptr %717, align 4
  store float %_0.i3378.lcssa1273214950, ptr %707, align 4
  store float %_0.i3752.lcssa1274814968, ptr %709, align 4
  store float %_0.i3374.lcssa1277214986, ptr %715, align 4
  store float %_0.i3748.lcssa1277315004, ptr %717, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_242.i, i32 noundef %_7.i8.i253.i, i32 noundef range(i32 0, 536870912) %_56.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #32, !dbg !24112, !noalias !24113
  unreachable, !dbg !24112

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1944
  %_17.i5.i.i = getelementptr inbounds nuw float, ptr %_56.0.i.i, i32 %_242.i, !dbg !24116
  store float %_0.i3238, ptr %_17.i5.i.i, align 4, !dbg !24118, !alias.scope !24120, !noalias !24102
  %_6.not.i1931 = icmp ugt i32 %_5.i1945, %_58.1.i.i
  br i1 %_6.not.i1931, label %bb4.i1935, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3703, !dbg !24123, !prof !4596

bb4.i1935:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !23067
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.10.0.lcssa, ptr %history.i44.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.13.0.lcssa, ptr %history.i44.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.16.0.lcssa, ptr %history.i44.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.19.0.lcssa, ptr %history.i44.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.22.0.lcssa, ptr %history.i44.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.26.0.lcssa, ptr %history.i44.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.29.0.lcssa, ptr %history.i44.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.32.0.lcssa, ptr %history.i44.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.35.0.lcssa, ptr %history.i44.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.38.0.lcssa, ptr %history.i44.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !23069
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store i32 %storemerge.i1754.lcssa94049613.lcssa15453, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1749.lcssa94319649.lcssa15472, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1724.lcssa95169685.lcssa15491, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1719.lcssa95439721.lcssa15510, ptr %_21.i.i, align 4
  store float %_0.i3378.lcssa1273214949.lcssa15530, ptr %707, align 4
  store float %_0.i3752.lcssa1274814967.lcssa15550, ptr %709, align 4
  store float %_0.i3374.lcssa1277214985.lcssa15570, ptr %715, align 4
  store float %_0.i3748.lcssa1277315003.lcssa15590, ptr %717, align 4
  store float %_0.i3378.lcssa1273214950, ptr %707, align 4
  store float %_0.i3752.lcssa1274814968, ptr %709, align 4
  store float %_0.i3374.lcssa1277214986, ptr %715, align 4
  store float %_0.i3748.lcssa1277315004, ptr %717, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_243.i, i32 noundef %_5.i1945, i32 noundef range(i32 0, 536870912) %_58.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !24128, !noalias !24129
  unreachable, !dbg !24128

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3703: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i
  %_0.i2911 = fdiv float %_0.i3374, %_37.i.i, !dbg !24132
  %_0.i3373 = fsub float 1.000000e+00, %_0.i2911, !dbg !24134
  %_0.i3372 = fsub float %_0.i3373, %_0.i37489578, !dbg !24136
  %_4.i2927 = fmul float %_9.i.i, %_0.i3372, !dbg !24138
  %_0.i2928 = fadd float %_0.i37489578, %_4.i2927, !dbg !24138
  %_3.i.i4199.inv = fcmp ogt float %_0.i3373, %_0.i2928, !dbg !24140
  %_4.i.i4206.v = select i1 %_3.i.i4199.inv, float %_0.i3373, float %_0.i2928, !dbg !24140
  %762 = tail call noundef float @llvm.fabs.f32(float %_4.i.i4206.v), !dbg !24143
  %763 = fcmp uge float %762, 0x3BC79CA100000000, !dbg !24146
  %_0.i3748 = select i1 %763, float %_4.i.i4206.v, float 0.000000e+00, !dbg !24148
  %_0.i3371 = fsub float 1.000000e+00, %_0.i3748, !dbg !24149
  %_15.i1933 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i32 %_243.i, !dbg !24151
  %_0.i3386 = load float, ptr %_15.i1933, align 4, !dbg !24153, !alias.scope !24155, !noalias !24102, !noundef !10
  store float %_0.i3552, ptr %_15.i1933, align 4, !dbg !24158, !alias.scope !24161, !noalias !24102
  %_0.i3237 = fmul float %_0.i3371, %_0.i3386, !dbg !24164
  %_6.i3974 = bitcast float %_0.i3386 to i32, !dbg !24166
  %_5.i3975 = and i32 %_6.i3974, %all.sroa.0.0.i, !dbg !24169
  %_8.i3976 = bitcast float %_0.i3237 to i32, !dbg !24170
  %_7.i3978 = and i32 %_9.i3990, %_8.i3976, !dbg !24172
  %_4.i3979 = or disjoint i32 %_7.i3978, %_5.i3975, !dbg !24169
  store i32 %_4.i3979, ptr %data.i5.i.i.i.i.i4973, align 4, !dbg !24173, !alias.scope !24175, !noalias !24178
  %exitcond12573.not = icmp eq i32 %_235.0.i, %umin12572, !dbg !23660
  br i1 %exitcond12573.not, label %bb74.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3554, !dbg !23660

bb74.i:                                           ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3703, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4955
  %_0.i3748.lcssa1277315003 = phi float [ %_0.i3748.lcssa1277315004, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4955 ], [ %_0.i3748, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3703 ]
  %_0.i3374.lcssa1277214985 = phi float [ %_0.i3374.lcssa1277214986, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4955 ], [ %_0.i3374, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3703 ]
  %_0.i3752.lcssa1274814967 = phi float [ %_0.i3752.lcssa1274814968, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4955 ], [ %_0.i3752, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3703 ]
  %_0.i3378.lcssa1273214949 = phi float [ %_0.i3378.lcssa1273214950, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4955 ], [ %_0.i3378, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3703 ]
  %running.sroa.0.0.i1719.lcssa95439721 = phi float [ %running.sroa.0.0.i1719.lcssa95439722, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4955 ], [ %running.sroa.0.0.i1719, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3703 ]
  %storemerge.i1724.lcssa95169685 = phi i32 [ %storemerge.i1724.lcssa95169686, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4955 ], [ %storemerge.i1724, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3703 ]
  %running.sroa.0.0.i1749.lcssa94319649 = phi float [ %running.sroa.0.0.i1749.lcssa94319650, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4955 ], [ %running.sroa.0.0.i1749, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3703 ]
  %storemerge.i1754.lcssa94049613 = phi i32 [ %storemerge.i1754.lcssa94049614, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4955 ], [ %storemerge.i1754, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3703 ]
  %_143.i = add i32 %run.sroa.0.0.i2352, %ring_cursor.sroa.0.1.i9607, !dbg !24179
  %_241.not.i = icmp ult i32 %_143.i, %ring.i, !dbg !24180
  %764 = select i1 %_241.not.i, i32 0, i32 %ring.i, !dbg !24180
  %ring_cursor.sroa.0.2.i = sub nuw i32 %_143.i, %764, !dbg !24180
  %_145.i = add i32 %run.sroa.0.0.i2352, %main_cursor.sroa.0.1.i9608, !dbg !24183
  %_252.not.i = icmp ult i32 %_145.i, %main.i, !dbg !24184
  %765 = select i1 %_252.not.i, i32 0, i32 %main.i, !dbg !24184
  %main_cursor.sroa.0.2.i = sub nuw i32 %_145.i, %765, !dbg !24184
  %_63.i = icmp ult i32 %_87.i, %spec.store.select.i, !dbg !23561
  br i1 %_63.i, label %bb20.i, label %bb15.i.loopexit, !dbg !23561

_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit: ; preds = %bb15.i.loopexit
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !23067
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.10.0.lcssa, ptr %history.i44.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.13.0.lcssa, ptr %history.i44.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.16.0.lcssa, ptr %history.i44.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.19.0.lcssa, ptr %history.i44.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.22.0.lcssa, ptr %history.i44.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.26.0.lcssa, ptr %history.i44.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.29.0.lcssa, ptr %history.i44.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.32.0.lcssa, ptr %history.i44.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.35.0.lcssa, ptr %history.i44.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i44.i.sroa.38.0.lcssa, ptr %history.i44.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !23067
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !23069
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !23069
  store i32 %storemerge.i1754.lcssa94049613.lcssa15452, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1749.lcssa94319649.lcssa15471, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1724.lcssa95169685.lcssa15490, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1719.lcssa95439721.lcssa15509, ptr %_21.i.i, align 4
  store float %_0.i3378.lcssa1273214949.lcssa15529, ptr %707, align 4
  store float %_0.i3752.lcssa1274814967.lcssa15549, ptr %709, align 4
  store float %_0.i3374.lcssa1277214985.lcssa15569, ptr %715, align 4
  store float %_0.i3748.lcssa1277315003.lcssa15589, ptr %717, align 4
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit, !dbg !24186

_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit, %bb6.i
  %ring_cursor.sroa.0.0.i.lcssa = phi i32 [ %_37.i, %bb6.i ], [ %ring_cursor.sroa.0.1.i.lcssa, %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit ], !dbg !23009
  %main_cursor.sroa.0.0.i.lcssa = phi i32 [ %_36.i, %bb6.i ], [ %main_cursor.sroa.0.1.i.lcssa, %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit ], !dbg !23006
  %766 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 24, !dbg !24186
  %left_prefix.i = load float, ptr %766, align 4, !dbg !24186, !noalias !23013, !noundef !10
  %767 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 28, !dbg !24187
  %left_phase.i = load i32, ptr %767, align 4, !dbg !24187, !noalias !23013, !noundef !10
  %768 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 24, !dbg !24188
  %right_prefix.i = load float, ptr %768, align 4, !dbg !24188, !noalias !23013, !noundef !10
  %769 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 28, !dbg !24189
  %right_phase.i = load i32, ptr %769, align 4, !dbg !24189, !noalias !23013, !noundef !10
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_right.i), !dbg !24190, !noalias !23013
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i), !dbg !24191, !noalias !23013
  %770 = getelementptr inbounds nuw i8, ptr %self, i32 364, !dbg !24192
  %_265.0.i = load ptr, ptr %770, align 4, !dbg !24192, !alias.scope !22980, !noalias !24194, !nonnull !10, !noundef !10
  %771 = getelementptr inbounds nuw i8, ptr %self, i32 368, !dbg !24192
  %_265.1.i = load i32, ptr %771, align 4, !dbg !24192, !alias.scope !22980, !noalias !24194, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !24195), !dbg !24198
  %_4.not.i3696 = icmp eq i32 %_265.1.i, 0, !dbg !24199
  br i1 %_4.not.i3696, label %panic.i3698, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3699, !dbg !24199

panic.i3698:                                      ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #32, !dbg !24199, !noalias !24201
  unreachable, !dbg !24199

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3699: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit
  store float %left_prefix.i, ptr %_265.0.i, align 4, !dbg !24199, !alias.scope !24195, !noalias !22984
  %_266.0.i = load ptr, ptr %68, align 4, !dbg !24202, !alias.scope !22980, !noalias !24194, !nonnull !10, !noundef !10
  %_266.1.i = load i32, ptr %69, align 4, !dbg !24202, !alias.scope !22980, !noalias !24194, !noundef !10
  %772 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i), !dbg !24203
  br i1 %772, label %bb2.i5000, label %bb6.i4992, !dbg !24203

bb6.i4992:                                        ; preds = %bb2.i5000, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3699
  %end_or_len.idx.i4993 = shl nuw nsw i32 %_266.1.i, 2, !dbg !24207
  %end_or_len.i4994 = getelementptr inbounds nuw i8, ptr %_266.0.i, i32 %end_or_len.idx.i4993, !dbg !24207
  %_293.i4995 = icmp eq i32 %_266.1.i, 0, !dbg !24211
  br i1 %_293.i4995, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5006, label %bb10.i4996, !dbg !24214

bb2.i5000:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3699
  %bytes1.sroa.0.0.zext.i5001 = and i32 %left_phase.i, 255, !dbg !24215
  %bytes1.sroa.0.0.isplat.i5002 = mul nuw i32 %bytes1.sroa.0.0.zext.i5001, 16843009, !dbg !24215
  %_5.i5003 = icmp eq i32 %left_phase.i, %bytes1.sroa.0.0.isplat.i5002, !dbg !24216
  br i1 %_5.i5003, label %bb3.i5004, label %bb6.i4992, !dbg !24216

bb3.i5004:                                        ; preds = %bb2.i5000
  %bytes.sroa.0.0.extract.trunc.i5005 = trunc i32 %left_phase.i to i8, !dbg !24217
  %773 = shl nuw nsw i32 %_266.1.i, 2, !dbg !24219
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %_266.0.i, i8 %bytes.sroa.0.0.extract.trunc.i5005, i32 %773, i1 false), !dbg !24219, !alias.scope !24220, !noalias !22984
  br label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5006, !dbg !24223

bb10.i4996:                                       ; preds = %bb6.i4992, %bb10.i4996
  %iter.sroa.0.04.i4997 = phi ptr [ %_38.i4998, %bb10.i4996 ], [ %_266.0.i, %bb6.i4992 ]
  %_38.i4998 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i4997, i32 4, !dbg !24224
  store i32 %left_phase.i, ptr %iter.sroa.0.04.i4997, align 4, !dbg !24226, !alias.scope !24220, !noalias !22984
  %_29.i4999 = icmp eq ptr %_38.i4998, %end_or_len.i4994, !dbg !24211
  br i1 %_29.i4999, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5006, label %bb10.i4996, !dbg !24214

_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5006: ; preds = %bb10.i4996, %bb6.i4992, %bb3.i5004
  %774 = getelementptr inbounds nuw i8, ptr %self, i32 464, !dbg !24227
  %_267.0.i = load ptr, ptr %774, align 4, !dbg !24227, !alias.scope !22982, !noalias !24228, !nonnull !10, !noundef !10
  %775 = getelementptr inbounds nuw i8, ptr %self, i32 468, !dbg !24227
  %_267.1.i = load i32, ptr %775, align 4, !dbg !24227, !alias.scope !22982, !noalias !24228, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !24229), !dbg !24232
  %_4.not.i3692 = icmp eq i32 %_267.1.i, 0, !dbg !24233
  br i1 %_4.not.i3692, label %panic.i3694, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3695, !dbg !24233

panic.i3694:                                      ; preds = %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5006
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #32, !dbg !24233, !noalias !24235
  unreachable, !dbg !24233

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3695: ; preds = %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5006
  store float %right_prefix.i, ptr %_267.0.i, align 4, !dbg !24233, !alias.scope !24229, !noalias !22984
  %_268.0.i = load ptr, ptr %77, align 4, !dbg !24236, !alias.scope !22982, !noalias !24228, !nonnull !10, !noundef !10
  %_268.1.i = load i32, ptr %78, align 4, !dbg !24236, !alias.scope !22982, !noalias !24228, !noundef !10
  %776 = tail call i1 @llvm.is.constant.i32(i32 %right_phase.i), !dbg !24237
  br i1 %776, label %bb2.i5015, label %bb6.i5007, !dbg !24237

bb6.i5007:                                        ; preds = %bb2.i5015, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3695
  %end_or_len.idx.i5008 = shl nuw nsw i32 %_268.1.i, 2, !dbg !24240
  %end_or_len.i5009 = getelementptr inbounds nuw i8, ptr %_268.0.i, i32 %end_or_len.idx.i5008, !dbg !24240
  %_293.i5010 = icmp eq i32 %_268.1.i, 0, !dbg !24244
  br i1 %_293.i5010, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5021, label %bb10.i5011, !dbg !24247

bb2.i5015:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3695
  %bytes1.sroa.0.0.zext.i5016 = and i32 %right_phase.i, 255, !dbg !24248
  %bytes1.sroa.0.0.isplat.i5017 = mul nuw i32 %bytes1.sroa.0.0.zext.i5016, 16843009, !dbg !24248
  %_5.i5018 = icmp eq i32 %right_phase.i, %bytes1.sroa.0.0.isplat.i5017, !dbg !24249
  br i1 %_5.i5018, label %bb3.i5019, label %bb6.i5007, !dbg !24249

bb3.i5019:                                        ; preds = %bb2.i5015
  %bytes.sroa.0.0.extract.trunc.i5020 = trunc i32 %right_phase.i to i8, !dbg !24250
  %777 = shl nuw nsw i32 %_268.1.i, 2, !dbg !24252
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %_268.0.i, i8 %bytes.sroa.0.0.extract.trunc.i5020, i32 %777, i1 false), !dbg !24252, !alias.scope !24253, !noalias !22984
  br label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5021, !dbg !24256

bb10.i5011:                                       ; preds = %bb6.i5007, %bb10.i5011
  %iter.sroa.0.04.i5012 = phi ptr [ %_38.i5013, %bb10.i5011 ], [ %_268.0.i, %bb6.i5007 ]
  %_38.i5013 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i5012, i32 4, !dbg !24257
  store i32 %right_phase.i, ptr %iter.sroa.0.04.i5012, align 4, !dbg !24259, !alias.scope !24253, !noalias !22984
  %_29.i5014 = icmp eq ptr %_38.i5013, %end_or_len.i5009, !dbg !24244
  br i1 %_29.i5014, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5021, label %bb10.i5011, !dbg !24247

_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5021: ; preds = %bb10.i5011, %bb6.i5007, %bb3.i5019
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %hot_left.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33) #31, !dbg !24260
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %hot_right.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34) #31, !dbg !24261
  store i32 %main_cursor.sroa.0.0.i.lcssa, ptr %_35, align 4, !dbg !24262, !alias.scope !22984, !noalias !23008
  store i32 %ring_cursor.sroa.0.0.i.lcssa, ptr %664, align 4, !dbg !24263, !alias.scope !22984, !noalias !23008
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i), !dbg !24264, !noalias !23013
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i), !dbg !24265, !noalias !23013
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockfEB2_.exit, !dbg !22977

_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockfEB2_.exit: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit, %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit, %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4820, %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5021
  br i1 %quiet.sroa.0.0.off05208, label %bb28, label %bb40, !dbg !24266

bb22:                                             ; preds = %bb20
  tail call void @llvm.experimental.noalias.scope.decl(metadata !24267), !dbg !24270
  %778 = getelementptr inbounds nuw i8, ptr %self, i32 380, !dbg !24271
  %_40.0.i = load ptr, ptr %778, align 4, !dbg !24271, !alias.scope !24267, !nonnull !10, !noundef !10
  %779 = getelementptr inbounds nuw i8, ptr %self, i32 384, !dbg !24271
  %_40.1.i = load i32, ptr %779, align 4, !dbg !24271, !alias.scope !24267, !noundef !10
  %780 = getelementptr inbounds nuw i8, ptr %self, i32 412, !dbg !24273
  %_41.0.i = load ptr, ptr %780, align 4, !dbg !24273, !alias.scope !24267, !nonnull !10, !noundef !10
  %781 = getelementptr inbounds nuw i8, ptr %self, i32 416, !dbg !24273
  %_41.1.i = load i32, ptr %781, align 4, !dbg !24273, !alias.scope !24267, !noundef !10
  %spec.store.select.i.i = tail call i32 @llvm.umin.i32(i32 %_41.1.i, i32 %_40.1.i), !dbg !24274
  %_2.i6.not.i = icmp eq i32 %spec.store.select.i.i, 0, !dbg !24280
  br i1 %_2.i6.not.i, label %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb3.i5022, !dbg !24280

bb3.i5022:                                        ; preds = %bb22, %bb5.i5024
  %iter.sroa.8.07.i = phi i32 [ %782, %bb5.i5024 ], [ 0, %bb22 ]
  %_3.i1.i.i = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i, i32 %iter.sroa.8.07.i, !dbg !24283
  %_14.i5023 = load i32, ptr %_3.i1.i.i, align 4, !dbg !24286, !noalias !24267, !noundef !10
  %_20.i = icmp eq i32 %_14.i5023, 0, !dbg !24287
  br i1 %_20.i, label %panic.i5031, label %bb5.i5024, !dbg !24287

bb5.i5024:                                        ; preds = %bb3.i5022
  %_3.i.i.i5025 = getelementptr inbounds nuw i32, ptr %_40.0.i, i32 %iter.sroa.8.07.i, !dbg !24288
  %782 = add nuw i32 %iter.sroa.8.07.i, 1, !dbg !24291
  %_18.i5026 = load i32, ptr %_3.i.i.i5025, align 4, !dbg !24292, !noalias !24267, !noundef !10
  %_19.i5027 = urem i32 %frames, %_14.i5023, !dbg !24287
  %_16.i5028 = add i32 %_19.i5027, %_18.i5026, !dbg !24293
  %_15.i5029 = urem i32 %_16.i5028, %_14.i5023, !dbg !24294
  store i32 %_15.i5029, ptr %_3.i.i.i5025, align 4, !dbg !24295, !noalias !24267
  %exitcond.not.i = icmp eq i32 %782, %spec.store.select.i.i, !dbg !24280
  br i1 %exitcond.not.i, label %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb3.i5022, !dbg !24280

panic.i5031:                                      ; preds = %bb3.i5022
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_f5b0427df9b659e554a697ca46ce8b5a) #32, !dbg !24287, !noalias !24267
  unreachable, !dbg !24287

_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit: ; preds = %bb5.i5024, %bb22
  tail call void @llvm.experimental.noalias.scope.decl(metadata !24296), !dbg !24299
  %783 = getelementptr inbounds nuw i8, ptr %self, i32 480, !dbg !24300
  %_40.0.i5032 = load ptr, ptr %783, align 4, !dbg !24300, !alias.scope !24296, !nonnull !10, !noundef !10
  %784 = getelementptr inbounds nuw i8, ptr %self, i32 484, !dbg !24300
  %_40.1.i5033 = load i32, ptr %784, align 4, !dbg !24300, !alias.scope !24296, !noundef !10
  %785 = getelementptr inbounds nuw i8, ptr %self, i32 512, !dbg !24302
  %_41.0.i5034 = load ptr, ptr %785, align 4, !dbg !24302, !alias.scope !24296, !nonnull !10, !noundef !10
  %786 = getelementptr inbounds nuw i8, ptr %self, i32 516, !dbg !24302
  %_41.1.i5035 = load i32, ptr %786, align 4, !dbg !24302, !alias.scope !24296, !noundef !10
  %spec.store.select.i.i5036 = tail call i32 @llvm.umin.i32(i32 %_41.1.i5035, i32 %_40.1.i5033), !dbg !24303
  %_2.i6.not.i5037 = icmp eq i32 %spec.store.select.i.i5036, 0, !dbg !24309
  br i1 %_2.i6.not.i5037, label %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit5052, label %bb3.i5038, !dbg !24309

bb3.i5038:                                        ; preds = %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, %bb5.i5043
  %iter.sroa.8.07.i5039 = phi i32 [ %787, %bb5.i5043 ], [ 0, %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit ]
  %_3.i1.i.i5040 = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i5034, i32 %iter.sroa.8.07.i5039, !dbg !24312
  %_14.i5041 = load i32, ptr %_3.i1.i.i5040, align 4, !dbg !24315, !noalias !24296, !noundef !10
  %_20.i5042 = icmp eq i32 %_14.i5041, 0, !dbg !24316
  br i1 %_20.i5042, label %panic.i5051, label %bb5.i5043, !dbg !24316

bb5.i5043:                                        ; preds = %bb3.i5038
  %_3.i.i.i5044 = getelementptr inbounds nuw i32, ptr %_40.0.i5032, i32 %iter.sroa.8.07.i5039, !dbg !24317
  %787 = add nuw i32 %iter.sroa.8.07.i5039, 1, !dbg !24320
  %_18.i5045 = load i32, ptr %_3.i.i.i5044, align 4, !dbg !24321, !noalias !24296, !noundef !10
  %_19.i5046 = urem i32 %frames, %_14.i5041, !dbg !24316
  %_16.i5047 = add i32 %_19.i5046, %_18.i5045, !dbg !24322
  %_15.i5048 = urem i32 %_16.i5047, %_14.i5041, !dbg !24323
  store i32 %_15.i5048, ptr %_3.i.i.i5044, align 4, !dbg !24324, !noalias !24296
  %exitcond.not.i5049 = icmp eq i32 %787, %spec.store.select.i.i5036, !dbg !24309
  br i1 %exitcond.not.i5049, label %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit5052, label %bb3.i5038, !dbg !24309

panic.i5051:                                      ; preds = %bb3.i5038
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_f5b0427df9b659e554a697ca46ce8b5a) #32, !dbg !24316, !noalias !24296
  unreachable, !dbg !24316

_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit5052: ; preds = %bb5.i5043, %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit
  %788 = getelementptr inbounds nuw i8, ptr %self, i32 528, !dbg !24325
  %_29.val = load i32, ptr %788, align 4, !dbg !24325
  %789 = getelementptr inbounds nuw i8, ptr %self, i32 532, !dbg !24325
  %_29.val4352 = load i32, ptr %789, align 4, !dbg !24325, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !24326), !dbg !24325
  %_10.i5053 = icmp eq i32 %_29.val4352, 0, !dbg !24329
  br i1 %_10.i5053, label %panic.i5063, label %bb1.i5054, !dbg !24329

bb1.i5054:                                        ; preds = %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit5052
  %_28 = getelementptr inbounds nuw i8, ptr %self, i32 104, !dbg !24331
  %_7.i = load i32, ptr %_28, align 4, !dbg !24332, !alias.scope !24326, !noundef !10
  %_8.i5055 = urem i32 %frames, %_29.val4352, !dbg !24329
  %_5.i5056 = add i32 %_8.i5055, %_7.i, !dbg !24333
  %_4.i5057 = urem i32 %_5.i5056, %_29.val4352, !dbg !24334
  store i32 %_4.i5057, ptr %_28, align 4, !dbg !24335, !alias.scope !24326
  %_17.i5058 = icmp eq i32 %_29.val, 0, !dbg !24336
  br i1 %_17.i5058, label %panic2.i, label %_RNvMs1_CsjLJhryqjeDL_17true_peak_limiterNtB5_7Cursors7advance.exit, !dbg !24336

panic.i5063:                                      ; preds = %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit5052
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c7e64e9e8489bc8e648659d0098d136c) #32, !dbg !24329, !noalias !24326
  unreachable, !dbg !24329

panic2.i:                                         ; preds = %bb1.i5054
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_34b23597fad9d85cb3bee6d64ebf77d5) #32, !dbg !24336, !noalias !24326
  unreachable, !dbg !24336

_RNvMs1_CsjLJhryqjeDL_17true_peak_limiterNtB5_7Cursors7advance.exit: ; preds = %bb1.i5054
  %790 = getelementptr inbounds nuw i8, ptr %self, i32 108, !dbg !24337
  %_14.i5060 = load i32, ptr %790, align 4, !dbg !24337, !alias.scope !24326, !noundef !10
  %_15.i5061 = urem i32 %frames, %_29.val, !dbg !24336
  %_12.i = add i32 %_15.i5061, %_14.i5060, !dbg !24338
  %_11.i5062 = urem i32 %_12.i, %_29.val, !dbg !24339
  store i32 %_11.i5062, ptr %790, align 4, !dbg !24340, !alias.scope !24326
  br label %bb42, !dbg !24341

bb28:                                             ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockfEB2_.exit
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_37 = tail call noundef zeroext i1 @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #31, !dbg !24342
  br i1 %_37, label %bb30, label %bb40, !dbg !24343

bb30:                                             ; preds = %bb28
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_39 = tail call noundef zeroext i1 @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #31, !dbg !24344
  br i1 %_39, label %bb32, label %bb40, !dbg !24345

bb32:                                             ; preds = %bb30
  %_80.not = icmp ugt i32 %frames, %left_io.1
  br i1 %_80.not, label %bb51, label %bb1.i5064, !dbg !24346, !prof !4596

bb51:                                             ; preds = %bb32
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef %frames, i32 noundef %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_fd9a647e86e53fac40c0d38dd38d80a3) #32, !dbg !24354
  unreachable, !dbg !24354

bb1.i5064:                                        ; preds = %bb32, %bb12.i5078
  %io.sroa.5.0.i5065 = phi i32 [ %len.i.i.i5071, %bb12.i5078 ], [ %frames, %bb32 ]
  %io.sroa.0.0.i5066 = phi ptr [ %data.i.i.i5070, %bb12.i5078 ], [ %left_io.0, %bb32 ]
  %791 = icmp eq i32 %io.sroa.5.0.i5065, 0, !dbg !24355
  br i1 %791, label %bb34, label %bb13.preheader.i5067, !dbg !24355

bb13.preheader.i5067:                             ; preds = %bb1.i5064
  %spec.store.select.i5068 = tail call i32 @llvm.umin.i32(i32 %io.sroa.5.0.i5065, i32 32), !dbg !24358
  %data.i.i.idx.i5069 = shl nuw nsw i32 %spec.store.select.i5068, 2, !dbg !24361
  %data.i.i.i5070 = getelementptr inbounds nuw i8, ptr %io.sroa.0.0.i5066, i32 %data.i.i.idx.i5069, !dbg !24361
  br label %bb13.i5072, !dbg !24366

bb13.i5072:                                       ; preds = %bb13.i5072, %bb13.preheader.i5067
  %iter.sroa.0.08.i5073 = phi ptr [ %_35.i5075, %bb13.i5072 ], [ %io.sroa.0.0.i5066, %bb13.preheader.i5067 ]
  %bits.sroa.0.07.i5074 = phi i32 [ %792, %bb13.i5072 ], [ 0, %bb13.preheader.i5067 ]
  %_35.i5075 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i5073, i32 4, !dbg !24368
  %_95.i5076 = load i32, ptr %iter.sroa.0.08.i5073, align 4, !dbg !24370, !alias.scope !24371, !noundef !10
  %792 = or i32 %_95.i5076, %bits.sroa.0.07.i5074, !dbg !24374
  %_29.i5077 = icmp eq ptr %_35.i5075, %data.i.i.i5070, !dbg !24375
  br i1 %_29.i5077, label %bb12.i5078, label %bb13.i5072, !dbg !24366

bb12.i5078:                                       ; preds = %bb13.i5072
  %len.i.i.i5071 = sub nuw nsw i32 %io.sroa.5.0.i5065, %spec.store.select.i5068, !dbg !24377
  %793 = icmp eq i32 %792, 0, !dbg !24378
  br i1 %793, label %bb1.i5064, label %bb40, !dbg !24378

bb34:                                             ; preds = %bb1.i5064
  %_88.not = icmp ugt i32 %frames, %right_io.1, !dbg !24379
  br i1 %_88.not, label %bb54, label %bb1.i5092, !dbg !24379, !prof !787

bb40:                                             ; preds = %bb12.i5078, %bb12.i5106, %bb1.i5092, %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockfEB2_.exit, %bb28, %bb30
  %_36.sroa.0.0.off0 = phi i1 [ false, %bb12.i5106 ], [ false, %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockfEB2_.exit ], [ false, %bb30 ], [ false, %bb28 ], [ true, %bb1.i5092 ], [ false, %bb12.i5078 ]
  %794 = zext i1 %_36.sroa.0.0.off0 to i8, !dbg !24385
  store i8 %794, ptr %38, align 8, !dbg !24385
  %795 = load i8, ptr %2, align 8, !dbg !24386, !range !4667, !noundef !10
  store i8 %795, ptr %0, align 1, !dbg !24387
  call void @llvm.lifetime.start.p0(ptr nonnull %shape), !dbg !24388
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(12) %shape, ptr noundef nonnull align 4 dereferenceable(12) %_32, i32 12, i1 false), !dbg !24389
  %796 = getelementptr inbounds nuw i8, ptr %self, i32 56, !dbg !24390
  %797 = load i32, ptr %796, align 8, !dbg !24390, !noundef !10
  %_53 = getelementptr inbounds nuw i8, ptr %self, i32 112, !dbg !24392
  %798 = getelementptr inbounds nuw i8, ptr %self, i32 88, !dbg !24399
  %_99.0 = load ptr, ptr %798, align 8, !dbg !24399, !nonnull !10, !noundef !10
  %799 = getelementptr inbounds nuw i8, ptr %self, i32 92, !dbg !24399
  %_99.1 = load i32, ptr %799, align 4, !dbg !24399, !noundef !10
  %800 = getelementptr inbounds nuw i8, ptr %self, i32 96, !dbg !24399
  %_100.0 = load ptr, ptr %800, align 8, !dbg !24399, !nonnull !10, !noundef !10
  %801 = getelementptr inbounds nuw i8, ptr %self, i32 100, !dbg !24399
  %_100.1 = load i32, ptr %801, align 4, !dbg !24399, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !24400), !dbg !24403
  tail call void @llvm.experimental.noalias.scope.decl(metadata !24404), !dbg !24403
  tail call void @llvm.experimental.noalias.scope.decl(metadata !24406), !dbg !24403
  %_22.not.i1463.i = icmp eq i32 %left_io.1, 0, !dbg !24408
  br i1 %_22.not.i1463.i, label %bb6.i.preheader.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i, !dbg !24408

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i: ; preds = %bb40, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i
  %ok.sroa.0.0.i1366.i = phi i32 [ %_0.i33.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i ], [ -1, %bb40 ]
  %iter.sroa.0.0.i1265.i = phi ptr [ %_27.i16.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i ], [ %left_io.0, %bb40 ]
  %iter.sroa.5.0.i1164.i = phi i32 [ %_28.i17.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i ], [ %left_io.1, %bb40 ]
  %_27.i16.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i1265.i, i32 4, !dbg !24421
  %_28.i17.i = add nsw i32 %iter.sroa.5.0.i1164.i, -1, !dbg !24428
  %_0.i28.i = load float, ptr %iter.sroa.0.0.i1265.i, align 4, !dbg !24429, !alias.scope !24432, !noalias !24435, !noundef !10
  %802 = tail call noundef float @llvm.fabs.f32(float %_0.i28.i), !dbg !24437
  %_3.i.i5081 = fcmp olt float %802, 0x46293E5940000000, !dbg !24440
  %_0.i33.i = select i1 %_3.i.i5081, i32 %ok.sroa.0.0.i1366.i, i32 0, !dbg !24443
  %_22.not.i14.i = icmp eq i32 %_28.i17.i, 0, !dbg !24408
  br i1 %_22.not.i14.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockfECsjLJhryqjeDL_17true_peak_limiter.exit25.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i, !dbg !24408

_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockfECsjLJhryqjeDL_17true_peak_limiter.exit25.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i
  %803 = icmp eq i32 %_0.i33.i, -1, !dbg !24446
  br i1 %803, label %bb6.i.preheader.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i, !dbg !24449

bb6.i.preheader.i:                                ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockfECsjLJhryqjeDL_17true_peak_limiter.exit25.i, %bb40
  %_22.not.i67.i = icmp eq i32 %right_io.1, 0, !dbg !24450
  br i1 %_22.not.i67.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockfNCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit32.i, !dbg !24450

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit32.i: ; preds = %bb6.i.preheader.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit32.i
  %ok.sroa.0.0.i70.i = phi i32 [ %_0.i34.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit32.i ], [ -1, %bb6.i.preheader.i ]
  %iter.sroa.0.0.i69.i = phi ptr [ %_27.i.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit32.i ], [ %right_io.0, %bb6.i.preheader.i ]
  %iter.sroa.5.0.i68.i = phi i32 [ %_28.i.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit32.i ], [ %right_io.1, %bb6.i.preheader.i ]
  %_27.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i69.i, i32 4, !dbg !24454
  %_28.i.i = add nsw i32 %iter.sroa.5.0.i68.i, -1, !dbg !24457
  %_0.i30.i = load float, ptr %iter.sroa.0.0.i69.i, align 4, !dbg !24458, !alias.scope !24460, !noalias !24463, !noundef !10
  %804 = tail call noundef float @llvm.fabs.f32(float %_0.i30.i), !dbg !24464
  %_3.i26.i = fcmp olt float %804, 0x46293E5940000000, !dbg !24466
  %_0.i34.i = select i1 %_3.i26.i, i32 %ok.sroa.0.0.i70.i, i32 0, !dbg !24468
  %_22.not.i.i = icmp eq i32 %_28.i.i, 0, !dbg !24450
  br i1 %_22.not.i.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockfECsjLJhryqjeDL_17true_peak_limiter.exit.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit32.i, !dbg !24450

_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockfECsjLJhryqjeDL_17true_peak_limiter.exit.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit32.i
  %805 = icmp eq i32 %_0.i34.i, -1, !dbg !24470
  br i1 %805, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockfNCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit, label %bb7.i5091, !dbg !24472

bb7.i5091:                                        ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockfECsjLJhryqjeDL_17true_peak_limiter.exit.i
  br i1 %_22.not.i1463.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i, !dbg !24473

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i: ; preds = %bb7.i5091, %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockfECsjLJhryqjeDL_17true_peak_limiter.exit25.i
  br label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.i, !dbg !24473

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i
  %ok.sroa.0.012.i.i = phi i32 [ %_0.i8.i.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ -1, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i ]
  %iter.sroa.0.011.i.i = phi ptr [ %_42.i.i5082, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ %left_io.0, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i ]
  %iter.sroa.5.010.i.i = phi i32 [ %_43.i.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ %left_io.1, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i ]
  %_42.i.i5082 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.011.i.i, i32 4, !dbg !24484
  %_43.i.i = add nsw i32 %iter.sroa.5.010.i.i, -1, !dbg !24491
  %_0.i.i.i5083 = load float, ptr %iter.sroa.0.011.i.i, align 4, !dbg !24492, !alias.scope !24495, !noalias !24435, !noundef !10
  %806 = tail call noundef float @llvm.fabs.f32(float %_0.i.i.i5083), !dbg !24500
  %_3.i.i.i5084 = fcmp olt float %806, 0x46293E5940000000, !dbg !24503
  %_0.i8.i.i = select i1 %_3.i.i.i5084, i32 %ok.sroa.0.012.i.i, i32 0, !dbg !24505
  %_37.not.i.i = icmp eq i32 %_43.i.i, 0, !dbg !24473
  br i1 %_37.not.i.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.i, !dbg !24473

_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.i
  %807 = and i32 %_0.i8.i.i, 1065353216, !dbg !24507
  %808 = icmp ne i32 %807, 1065353216, !dbg !24510
  %809 = zext i1 %808 to i32, !dbg !24510
  %_37.not9.i40.i = icmp eq i32 %right_io.1, 0, !dbg !24514
  br i1 %_37.not9.i40.i, label %bb12.i.preheader.thread.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i, !dbg !24514

bb12.i.preheader.thread.i:                        ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit.i
  %810 = getelementptr inbounds nuw i8, ptr %self, i32 120, !dbg !24518
  store i32 %809, ptr %810, align 8, !dbg !24518, !alias.scope !24406, !noalias !24519
  %_1481.i = load i64, ptr %_53, align 8, !dbg !24520, !alias.scope !24406, !noalias !24519, !noundef !10
  %811 = tail call i64 @llvm.uadd.sat.i64(i64 %_1481.i, i64 1), !dbg !24521
  store i64 %811, ptr %_53, align 8, !dbg !24524, !alias.scope !24406, !noalias !24519
  br label %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit60.i, !dbg !24525

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i: ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit.i, %bb7.i5091
  %ok.sroa.0.0.lcssa.i76.i = phi i32 [ %809, %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit.i ], [ 0, %bb7.i5091 ]
  br label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.i, !dbg !24514

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i
  %ok.sroa.0.012.i42.i = phi i32 [ %_0.i8.i49.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.i ], [ -1, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i ]
  %iter.sroa.0.011.i43.i = phi ptr [ %_42.i45.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.i ], [ %right_io.0, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i ]
  %iter.sroa.5.010.i44.i = phi i32 [ %_43.i46.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.i ], [ %right_io.1, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i ]
  %_42.i45.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.011.i43.i, i32 4, !dbg !24530
  %_43.i46.i = add nsw i32 %iter.sroa.5.010.i44.i, -1, !dbg !24533
  %_0.i.i47.i = load float, ptr %iter.sroa.0.011.i43.i, align 4, !dbg !24534, !alias.scope !24536, !noalias !24463, !noundef !10
  %812 = tail call noundef float @llvm.fabs.f32(float %_0.i.i47.i), !dbg !24541
  %_3.i.i48.i = fcmp olt float %812, 0x46293E5940000000, !dbg !24543
  %_0.i8.i49.i = select i1 %_3.i.i48.i, i32 %ok.sroa.0.012.i42.i, i32 0, !dbg !24545
  %_37.not.i50.i = icmp eq i32 %_43.i46.i, 0, !dbg !24514
  br i1 %_37.not.i50.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit53.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.i, !dbg !24514

_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit53.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.i
  %813 = and i32 %_0.i8.i49.i, 1065353216, !dbg !24547
  %814 = icmp ne i32 %813, 1065353216, !dbg !24549
  %815 = zext i1 %814 to i32, !dbg !24549
  %816 = or i32 %ok.sroa.0.0.lcssa.i76.i, %815, !dbg !24518
  %817 = getelementptr inbounds nuw i8, ptr %self, i32 120, !dbg !24518
  store i32 %816, ptr %817, align 8, !dbg !24518, !alias.scope !24406, !noalias !24519
  %_14.i5085 = load i64, ptr %_53, align 8, !dbg !24520, !alias.scope !24406, !noalias !24519, !noundef !10
  %818 = tail call i64 @llvm.uadd.sat.i64(i64 %_14.i5085, i64 1), !dbg !24521
  store i64 %818, ptr %_53, align 8, !dbg !24524, !alias.scope !24406, !noalias !24519
  br i1 %_22.not.i1463.i, label %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit60.i, label %bb12.i.preheader.i, !dbg !24550

bb12.i.preheader.i:                               ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit53.i
  %.idx.i.i = shl nuw nsw i32 %left_io.1, 2, !dbg !24554
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %left_io.0, i8 0, i32 %.idx.i.i, i1 false), !dbg !24558, !alias.scope !24559, !noalias !24435
  br label %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit60.i, !dbg !24525

_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit60.i: ; preds = %bb12.i.preheader.i, %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit53.i, %bb12.i.preheader.thread.i
  %left.1.sink.i = phi i32 [ %left_io.1, %bb12.i.preheader.thread.i ], [ %right_io.1, %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit53.i ], [ %right_io.1, %bb12.i.preheader.i ]
  %left.0.sink.i = phi ptr [ %left_io.0, %bb12.i.preheader.thread.i ], [ %right_io.0, %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit53.i ], [ %right_io.0, %bb12.i.preheader.i ]
  %.idx.i85.i = shl nuw nsw i32 %left.1.sink.i, 2, !dbg !24562
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %left.0.sink.i, i8 0, i32 %.idx.i85.i, i1 false), !dbg !24568, !alias.scope !24569, !noalias !24570
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 4 dereferenceable(100) %_33, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(12) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_99.0, i32 noundef %_99.1, i32 noundef %797) #31, !dbg !24571, !noalias !24574
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 4 dereferenceable(100) %_34, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(12) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_100.0, i32 noundef %_100.1, i32 noundef %797) #31, !dbg !24577, !noalias !24574
  store i32 0, ptr %_35, align 4, !dbg !24578, !noalias !24574
  %819 = getelementptr inbounds nuw i8, ptr %self, i32 108, !dbg !24578
  store i32 0, ptr %819, align 4, !dbg !24578, !noalias !24574
  br label %_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockfNCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit, !dbg !24579

_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockfNCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit: ; preds = %bb6.i.preheader.i, %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockfECsjLJhryqjeDL_17true_peak_limiter.exit.i, %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit60.i
  call void @llvm.lifetime.end.p0(ptr nonnull %shape), !dbg !24580
  br label %bb42, !dbg !24341

bb54:                                             ; preds = %bb34
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef %frames, i32 noundef %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_35f4c396f0d0e2784e6b286770694d2b) #32, !dbg !24581
  unreachable, !dbg !24581

bb1.i5092:                                        ; preds = %bb34, %bb12.i5106
  %io.sroa.5.0.i5093 = phi i32 [ %len.i.i.i5099, %bb12.i5106 ], [ %frames, %bb34 ]
  %io.sroa.0.0.i5094 = phi ptr [ %data.i.i.i5098, %bb12.i5106 ], [ %right_io.0, %bb34 ]
  %820 = icmp eq i32 %io.sroa.5.0.i5093, 0, !dbg !24582
  br i1 %820, label %bb40, label %bb13.preheader.i5095, !dbg !24582

bb13.preheader.i5095:                             ; preds = %bb1.i5092
  %spec.store.select.i5096 = tail call i32 @llvm.umin.i32(i32 %io.sroa.5.0.i5093, i32 32), !dbg !24585
  %data.i.i.idx.i5097 = shl nuw nsw i32 %spec.store.select.i5096, 2, !dbg !24588
  %data.i.i.i5098 = getelementptr inbounds nuw i8, ptr %io.sroa.0.0.i5094, i32 %data.i.i.idx.i5097, !dbg !24588
  br label %bb13.i5100, !dbg !24593

bb13.i5100:                                       ; preds = %bb13.i5100, %bb13.preheader.i5095
  %iter.sroa.0.08.i5101 = phi ptr [ %_35.i5103, %bb13.i5100 ], [ %io.sroa.0.0.i5094, %bb13.preheader.i5095 ]
  %bits.sroa.0.07.i5102 = phi i32 [ %821, %bb13.i5100 ], [ 0, %bb13.preheader.i5095 ]
  %_35.i5103 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i5101, i32 4, !dbg !24595
  %_95.i5104 = load i32, ptr %iter.sroa.0.08.i5101, align 4, !dbg !24597, !alias.scope !24598, !noundef !10
  %821 = or i32 %_95.i5104, %bits.sroa.0.07.i5102, !dbg !24601
  %_29.i5105 = icmp eq ptr %_35.i5103, %data.i.i.i5098, !dbg !24602
  br i1 %_29.i5105, label %bb12.i5106, label %bb13.i5100, !dbg !24593

bb12.i5106:                                       ; preds = %bb13.i5100
  %len.i.i.i5099 = sub nuw nsw i32 %io.sroa.5.0.i5093, %spec.store.select.i5096, !dbg !24604
  %822 = icmp eq i32 %821, 0, !dbg !24605
  br i1 %822, label %bb1.i5092, label %bb40, !dbg !24605

bb42:                                             ; preds = %_RNvMs1_CsjLJhryqjeDL_17true_peak_limiterNtB5_7Cursors7advance.exit, %_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockfNCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit
  ret void, !dbg !24341
}
