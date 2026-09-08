define internal fastcc void @_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCorefE13process_blockB5_(ptr noalias noundef nonnull align 8 dereferenceable(784) %self, ptr noalias noundef nonnull align 4 captures(address) %left_io.0, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef nonnull align 4 captures(address) %right_io.0, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef %frames) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !36563 {
start:
  %_112.i1010 = alloca [92 x i8], align 4
  %_110.i1011 = alloca [92 x i8], align 4
  %peaks_right.i1013 = alloca [1024 x i8], align 4
  %peaks_left.i1014 = alloca [1024 x i8], align 4
  %scratch.i1015 = alloca [32 x i8], align 4
  %hot_right.i1016 = alloca [92 x i8], align 4
  %hot_left.i1017 = alloca [92 x i8], align 4
  %peaks_right.i729 = alloca [1024 x i8], align 4
  %peaks_left.i730 = alloca [1024 x i8], align 4
  %scratch.i = alloca [32 x i8], align 4
  %hot_right.i731 = alloca [92 x i8], align 4
  %hot_left.i732 = alloca [92 x i8], align 4
  %_159.i33 = alloca [92 x i8], align 4
  %_157.i34 = alloca [92 x i8], align 4
  %uniform_right.i51 = alloca [80 x i8], align 8
  %uniform_left.i52 = alloca [80 x i8], align 8
  %peaks_right.i53 = alloca [1024 x i8], align 4
  %peaks_left.i54 = alloca [1024 x i8], align 4
  %hot_right.i55 = alloca [92 x i8], align 4
  %hot_left.i56 = alloca [92 x i8], align 4
  %uniform_right.i = alloca [80 x i8], align 8
  %uniform_left.i = alloca [80 x i8], align 8
  %peaks_right.i = alloca [1024 x i8], align 4
  %peaks_left.i = alloca [1024 x i8], align 4
  %hot_right.i = alloca [92 x i8], align 4
  %hot_left.i = alloca [92 x i8], align 4
  %shape = alloca [24 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 781, !dbg !36564
  %1 = load i8, ptr %0, align 1, !dbg !36564, !range !17, !noundef !12
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 96, !dbg !36566
  %3 = load i8, ptr %2, align 8, !dbg !36566, !range !17, !noundef !12
  %_7 = icmp eq i8 %1, %3, !dbg !36564
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 264
  %_95.0 = load ptr, ptr %4, align 8, !dbg !36567
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 272
  %_95.1 = load i64, ptr %5, align 8, !dbg !36567
  br i1 %_7, label %bb1, label %bb20.thread, !dbg !36564

bb1:                                              ; preds = %start
  %_8.i4396 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_95.0, i64 %_95.1, !dbg !36568
  br label %bb1.i.i, !dbg !36573

bb1.i.i:                                          ; preds = %bb13.i.i4398, %bb1
  %_221.i.i = phi ptr [ %_22.i.i4399, %bb13.i.i4398 ], [ %_95.0, %bb1 ]
  %_12.i.i4397 = icmp eq ptr %_221.i.i, %_8.i4396, !dbg !36575
  br i1 %_12.i.i4397, label %bb3, label %bb13.i.i4398, !dbg !36578

bb13.i.i4398:                                     ; preds = %bb1.i.i
  %_22.i.i4399 = getelementptr inbounds nuw i8, ptr %_221.i.i, i64 16, !dbg !36579
  %6 = getelementptr inbounds nuw i8, ptr %_221.i.i, i64 12, !dbg !36581
  %_3.i.i.i = load i32, ptr %6, align 4, !dbg !36581, !alias.scope !36583, !noalias !36588, !noundef !12
  %7 = icmp eq i32 %_3.i.i.i, 0, !dbg !36581
  %_51.i.i.i = load i32, ptr %_221.i.i, align 4, !dbg !36581, !alias.scope !36583, !noalias !36588
  %8 = getelementptr inbounds nuw i8, ptr %_221.i.i, i64 4, !dbg !36581
  %_72.i.i.i = load i32, ptr %8, align 4, !dbg !36581, !alias.scope !36583, !noalias !36588
  %9 = icmp eq i32 %_51.i.i.i, %_72.i.i.i, !dbg !36581
  %_0.sroa.0.0.i.i.i = select i1 %7, i1 %9, i1 false, !dbg !36581
  br i1 %_0.sroa.0.0.i.i.i, label %bb1.i.i, label %bb20.thread, !dbg !36591

bb3:                                              ; preds = %bb1.i.i
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 280, !dbg !36592
  %_96.0 = load ptr, ptr %10, align 8, !dbg !36592, !nonnull !12, !noundef !12
  %11 = getelementptr inbounds nuw i8, ptr %self, i64 288, !dbg !36592
  %_96.1 = load i64, ptr %11, align 8, !dbg !36592, !noundef !12
  %_8.i4400 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_96.0, i64 %_96.1, !dbg !36593
  br label %bb1.i.i4401, !dbg !36598

bb1.i.i4401:                                      ; preds = %bb13.i.i4404, %bb3
  %_221.i.i4402 = phi ptr [ %_22.i.i4405, %bb13.i.i4404 ], [ %_96.0, %bb3 ]
  %_12.i.i4403 = icmp eq ptr %_221.i.i4402, %_8.i4400, !dbg !36600
  br i1 %_12.i.i4403, label %bb5, label %bb13.i.i4404, !dbg !36603

bb13.i.i4404:                                     ; preds = %bb1.i.i4401
  %_22.i.i4405 = getelementptr inbounds nuw i8, ptr %_221.i.i4402, i64 16, !dbg !36604
  %12 = getelementptr inbounds nuw i8, ptr %_221.i.i4402, i64 12, !dbg !36606
  %_3.i.i.i4406 = load i32, ptr %12, align 4, !dbg !36606, !alias.scope !36608, !noalias !36613, !noundef !12
  %13 = icmp eq i32 %_3.i.i.i4406, 0, !dbg !36606
  %_51.i.i.i4407 = load i32, ptr %_221.i.i4402, align 4, !dbg !36606, !alias.scope !36608, !noalias !36613
  %14 = getelementptr inbounds nuw i8, ptr %_221.i.i4402, i64 4, !dbg !36606
  %_72.i.i.i4408 = load i32, ptr %14, align 4, !dbg !36606, !alias.scope !36608, !noalias !36613
  %15 = icmp eq i32 %_51.i.i.i4407, %_72.i.i.i4408, !dbg !36606
  %_0.sroa.0.0.i.i.i4409 = select i1 %13, i1 %15, i1 false, !dbg !36606
  br i1 %_0.sroa.0.0.i.i.i4409, label %bb1.i.i4401, label %bb20.thread, !dbg !36616

bb5:                                              ; preds = %bb1.i.i4401
  %16 = getelementptr inbounds nuw i8, ptr %self, i64 464, !dbg !36617
  %_97.0 = load ptr, ptr %16, align 8, !dbg !36617, !nonnull !12, !noundef !12
  %17 = getelementptr inbounds nuw i8, ptr %self, i64 472, !dbg !36617
  %_97.1 = load i64, ptr %17, align 8, !dbg !36617, !noundef !12
  %_8.i4411 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_97.0, i64 %_97.1, !dbg !36618
  br label %bb1.i.i4412, !dbg !36623

bb1.i.i4412:                                      ; preds = %bb13.i.i4415, %bb5
  %_221.i.i4413 = phi ptr [ %_22.i.i4416, %bb13.i.i4415 ], [ %_97.0, %bb5 ]
  %_12.i.i4414 = icmp eq ptr %_221.i.i4413, %_8.i4411, !dbg !36625
  br i1 %_12.i.i4414, label %bb7, label %bb13.i.i4415, !dbg !36628

bb13.i.i4415:                                     ; preds = %bb1.i.i4412
  %_22.i.i4416 = getelementptr inbounds nuw i8, ptr %_221.i.i4413, i64 16, !dbg !36629
  %18 = getelementptr inbounds nuw i8, ptr %_221.i.i4413, i64 12, !dbg !36631
  %_3.i.i.i4417 = load i32, ptr %18, align 4, !dbg !36631, !alias.scope !36633, !noalias !36638, !noundef !12
  %19 = icmp eq i32 %_3.i.i.i4417, 0, !dbg !36631
  %_51.i.i.i4418 = load i32, ptr %_221.i.i4413, align 4, !dbg !36631, !alias.scope !36633, !noalias !36638
  %20 = getelementptr inbounds nuw i8, ptr %_221.i.i4413, i64 4, !dbg !36631
  %_72.i.i.i4419 = load i32, ptr %20, align 4, !dbg !36631, !alias.scope !36633, !noalias !36638
  %21 = icmp eq i32 %_51.i.i.i4418, %_72.i.i.i4419, !dbg !36631
  %_0.sroa.0.0.i.i.i4420 = select i1 %19, i1 %21, i1 false, !dbg !36631
  br i1 %_0.sroa.0.0.i.i.i4420, label %bb1.i.i4412, label %bb20.thread, !dbg !36641

bb7:                                              ; preds = %bb1.i.i4412
  %22 = getelementptr inbounds nuw i8, ptr %self, i64 480, !dbg !36642
  %_98.0 = load ptr, ptr %22, align 8, !dbg !36642, !nonnull !12, !noundef !12
  %23 = getelementptr inbounds nuw i8, ptr %self, i64 488, !dbg !36642
  %_98.1 = load i64, ptr %23, align 8, !dbg !36642, !noundef !12
  %_8.i4422 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_98.0, i64 %_98.1, !dbg !36643
  br label %bb1.i.i4423, !dbg !36648

bb1.i.i4423:                                      ; preds = %bb13.i.i4426, %bb7
  %_221.i.i4424 = phi ptr [ %_22.i.i4427, %bb13.i.i4426 ], [ %_98.0, %bb7 ]
  %_12.i.i4425 = icmp eq ptr %_221.i.i4424, %_8.i4422, !dbg !36650
  br i1 %_12.i.i4425, label %bb9, label %bb13.i.i4426, !dbg !36653

bb13.i.i4426:                                     ; preds = %bb1.i.i4423
  %_22.i.i4427 = getelementptr inbounds nuw i8, ptr %_221.i.i4424, i64 16, !dbg !36654
  %24 = getelementptr inbounds nuw i8, ptr %_221.i.i4424, i64 12, !dbg !36656
  %_3.i.i.i4428 = load i32, ptr %24, align 4, !dbg !36656, !alias.scope !36658, !noalias !36663, !noundef !12
  %25 = icmp eq i32 %_3.i.i.i4428, 0, !dbg !36656
  %_51.i.i.i4429 = load i32, ptr %_221.i.i4424, align 4, !dbg !36656, !alias.scope !36658, !noalias !36663
  %26 = getelementptr inbounds nuw i8, ptr %_221.i.i4424, i64 4, !dbg !36656
  %_72.i.i.i4430 = load i32, ptr %26, align 4, !dbg !36656, !alias.scope !36658, !noalias !36663
  %27 = icmp eq i32 %_51.i.i.i4429, %_72.i.i.i4430, !dbg !36656
  %_0.sroa.0.0.i.i.i4431 = select i1 %25, i1 %27, i1 false, !dbg !36656
  br i1 %_0.sroa.0.0.i.i.i4431, label %bb1.i.i4423, label %bb20.thread, !dbg !36666

bb9:                                              ; preds = %bb1.i.i4423
  %_65.not = icmp ugt i64 %frames, %left_io.1
  br i1 %_65.not, label %bb45, label %bb1.i4433, !dbg !36667, !prof !165

bb45:                                             ; preds = %bb9
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %frames, i64 noundef %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_34e4746889305f9c657ea46721b345d0) #30, !dbg !36676
  unreachable, !dbg !36676

bb1.i4433:                                        ; preds = %bb9, %bb10.i4439
  %iter.sroa.6.0.i = phi i64 [ %len.i.i.i.i, %bb10.i4439 ], [ %frames, %bb9 ], !dbg !36677
  %iter.sroa.0.0.i4434 = phi ptr [ %data.i.i.i.i, %bb10.i4439 ], [ %left_io.0, %bb9 ], !dbg !36677
  %28 = icmp eq i64 %iter.sroa.6.0.i, 0, !dbg !36679
  br i1 %28, label %bb11, label %bb11.preheader.i, !dbg !36679

bb11.preheader.i:                                 ; preds = %bb1.i4433
  %..i.i.i = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i, i64 32), !dbg !36681
  %_18.idx.i = shl nuw nsw i64 %..i.i.i, 2, !dbg !36684
  %_18.i4435 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i4434, i64 %_18.idx.i, !dbg !36684
  br label %bb11.i4436, !dbg !36689

bb11.i4436:                                       ; preds = %bb11.i4436, %bb11.preheader.i
  %iter1.sroa.0.014.i = phi ptr [ %_31.i4437, %bb11.i4436 ], [ %iter.sroa.0.0.i4434, %bb11.preheader.i ]
  %bits.sroa.0.013.i = phi i32 [ %29, %bb11.i4436 ], [ 0, %bb11.preheader.i ]
  %_31.i4437 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i, i64 4, !dbg !36691
  %_134.i = load i32, ptr %iter1.sroa.0.014.i, align 4, !dbg !36693, !alias.scope !36694, !noundef !12
  %29 = or i32 %_134.i, %bits.sroa.0.013.i, !dbg !36697
  %_25.i4438 = icmp eq ptr %_31.i4437, %_18.i4435, !dbg !36698
  br i1 %_25.i4438, label %bb10.i4439, label %bb11.i4436, !dbg !36689

bb10.i4439:                                       ; preds = %bb11.i4436
  %data.i.i.i.i = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i4434, i64 %..i.i.i, !dbg !36700
  %len.i.i.i.i = sub nuw nsw i64 %iter.sroa.6.0.i, %..i.i.i, !dbg !36705
  %30 = icmp eq i32 %29, 0, !dbg !36706
  br i1 %30, label %bb1.i4433, label %bb20.thread, !dbg !36706

bb11:                                             ; preds = %bb1.i4433
  %_73.not = icmp ugt i64 %frames, %right_io.1, !dbg !36707
  br i1 %_73.not, label %bb48, label %bb1.i4440, !dbg !36707, !prof !1406

bb20.thread:                                      ; preds = %bb13.i.i4398, %bb13.i.i4404, %bb13.i.i4415, %bb13.i.i4426, %bb10.i4439, %start
  %31 = getelementptr inbounds nuw i8, ptr %self, i64 780
  br label %bb26, !dbg !36713

bb20:                                             ; preds = %bb1.i4440
  %32 = getelementptr inbounds nuw i8, ptr %self, i64 780
  %33 = load i8, ptr %32, align 4, !range !17
  %_22 = trunc nuw i8 %33 to i1
  br i1 %_22, label %bb22, label %bb26, !dbg !36713

bb48:                                             ; preds = %bb11
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %frames, i64 noundef %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_22a7c5212c1e2b1f20d68ba89a9d6a79) #30, !dbg !36715
  unreachable, !dbg !36715

bb1.i4440:                                        ; preds = %bb11, %bb10.i4455
  %iter.sroa.6.0.i4441 = phi i64 [ %len.i.i.i.i4446, %bb10.i4455 ], [ %frames, %bb11 ], !dbg !36716
  %iter.sroa.0.0.i4442 = phi ptr [ %data.i.i.i.i4445, %bb10.i4455 ], [ %right_io.0, %bb11 ], !dbg !36716
  %34 = icmp eq i64 %iter.sroa.6.0.i4441, 0, !dbg !36718
  br i1 %34, label %bb20, label %bb11.preheader.i4443, !dbg !36718

bb11.preheader.i4443:                             ; preds = %bb1.i4440
  %..i.i.i4444 = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i4441, i64 32), !dbg !36720
  %_18.idx.i4447 = shl nuw nsw i64 %..i.i.i4444, 2, !dbg !36723
  %_18.i4448 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i4442, i64 %_18.idx.i4447, !dbg !36723
  br label %bb11.i4449, !dbg !36728

bb11.i4449:                                       ; preds = %bb11.i4449, %bb11.preheader.i4443
  %iter1.sroa.0.014.i4450 = phi ptr [ %_31.i4452, %bb11.i4449 ], [ %iter.sroa.0.0.i4442, %bb11.preheader.i4443 ]
  %bits.sroa.0.013.i4451 = phi i32 [ %35, %bb11.i4449 ], [ 0, %bb11.preheader.i4443 ]
  %_31.i4452 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i4450, i64 4, !dbg !36730
  %_134.i4453 = load i32, ptr %iter1.sroa.0.014.i4450, align 4, !dbg !36732, !alias.scope !36733, !noundef !12
  %35 = or i32 %_134.i4453, %bits.sroa.0.013.i4451, !dbg !36736
  %_25.i4454 = icmp eq ptr %_31.i4452, %_18.i4448, !dbg !36737
  br i1 %_25.i4454, label %bb10.i4455, label %bb11.i4449, !dbg !36728

bb10.i4455:                                       ; preds = %bb11.i4449
  %data.i.i.i.i4445 = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i4442, i64 %..i.i.i4444, !dbg !36739
  %len.i.i.i.i4446 = sub nuw nsw i64 %iter.sroa.6.0.i4441, %..i.i.i4444, !dbg !36744
  %36 = icmp eq i32 %35, 0, !dbg !36745
  br i1 %36, label %bb1.i4440, label %bb20.thread5484, !dbg !36745

bb20.thread5484:                                  ; preds = %bb10.i4455
  %37 = getelementptr inbounds nuw i8, ptr %self, i64 780
  br label %bb26, !dbg !36713

bb26:                                             ; preds = %bb20.thread5484, %bb20.thread, %bb20
  %38 = phi ptr [ %31, %bb20.thread ], [ %32, %bb20 ], [ %37, %bb20.thread5484 ]
  %quiet.sroa.0.05483 = phi i1 [ false, %bb20.thread ], [ true, %bb20 ], [ false, %bb20.thread5484 ]
  %_31 = getelementptr inbounds nuw i8, ptr %self, i64 584, !dbg !36746
  %_32 = getelementptr inbounds nuw i8, ptr %self, i64 536, !dbg !36747
  %_33 = getelementptr inbounds nuw i8, ptr %self, i64 136, !dbg !36748
  %_34 = getelementptr inbounds nuw i8, ptr %self, i64 336, !dbg !36749
  %_35 = getelementptr inbounds nuw i8, ptr %self, i64 560, !dbg !36750
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36751), !dbg !36754
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36757), !dbg !36754
  %_8.i4457 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_95.0, i64 %_95.1, !dbg !36759
  br label %bb1.i.i4458, !dbg !36765

bb1.i.i4458:                                      ; preds = %bb13.i.i4461, %bb26
  %_221.i.i4459 = phi ptr [ %_22.i.i4462, %bb13.i.i4461 ], [ %_95.0, %bb26 ]
  %_12.i.i4460 = icmp eq ptr %_221.i.i4459, %_8.i4457, !dbg !36767
  br i1 %_12.i.i4460, label %bb2.i, label %bb13.i.i4461, !dbg !36770

bb13.i.i4461:                                     ; preds = %bb1.i.i4458
  %_22.i.i4462 = getelementptr inbounds nuw i8, ptr %_221.i.i4459, i64 16, !dbg !36771
  %39 = getelementptr inbounds nuw i8, ptr %_221.i.i4459, i64 12, !dbg !36773
  %_3.i.i.i4463 = load i32, ptr %39, align 4, !dbg !36773, !alias.scope !36775, !noalias !36780, !noundef !12
  %40 = icmp eq i32 %_3.i.i.i4463, 0, !dbg !36773
  %_51.i.i.i4464 = load i32, ptr %_221.i.i4459, align 4, !dbg !36773, !alias.scope !36775, !noalias !36780
  %41 = getelementptr inbounds nuw i8, ptr %_221.i.i4459, i64 4, !dbg !36773
  %_72.i.i.i4465 = load i32, ptr %41, align 4, !dbg !36773, !alias.scope !36775, !noalias !36780
  %42 = icmp eq i32 %_51.i.i.i4464, %_72.i.i.i4465, !dbg !36773
  %_0.sroa.0.0.i.i.i4466 = select i1 %40, i1 %42, i1 false, !dbg !36773
  br i1 %_0.sroa.0.0.i.i.i4466, label %bb1.i.i4458, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit, !dbg !36789

bb2.i:                                            ; preds = %bb1.i.i4458
  %43 = getelementptr inbounds nuw i8, ptr %self, i64 280, !dbg !36790
  %_15.0.i = load ptr, ptr %43, align 8, !dbg !36790, !alias.scope !36751, !noalias !36791, !nonnull !12, !noundef !12
  %44 = getelementptr inbounds nuw i8, ptr %self, i64 288, !dbg !36790
  %_15.1.i = load i64, ptr %44, align 8, !dbg !36790, !alias.scope !36751, !noalias !36791, !noundef !12
  %_8.i4468 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_15.0.i, i64 %_15.1.i, !dbg !36792
  br label %bb1.i.i4469, !dbg !36797

bb1.i.i4469:                                      ; preds = %bb13.i.i4472, %bb2.i
  %_221.i.i4470 = phi ptr [ %_22.i.i4473, %bb13.i.i4472 ], [ %_15.0.i, %bb2.i ]
  %_12.i.i4471 = icmp eq ptr %_221.i.i4470, %_8.i4468, !dbg !36799
  br i1 %_12.i.i4471, label %bb4.i1970, label %bb13.i.i4472, !dbg !36802

bb13.i.i4472:                                     ; preds = %bb1.i.i4469
  %_22.i.i4473 = getelementptr inbounds nuw i8, ptr %_221.i.i4470, i64 16, !dbg !36803
  %45 = getelementptr inbounds nuw i8, ptr %_221.i.i4470, i64 12, !dbg !36805
  %_3.i.i.i4474 = load i32, ptr %45, align 4, !dbg !36805, !alias.scope !36807, !noalias !36812, !noundef !12
  %46 = icmp eq i32 %_3.i.i.i4474, 0, !dbg !36805
  %_51.i.i.i4475 = load i32, ptr %_221.i.i4470, align 4, !dbg !36805, !alias.scope !36807, !noalias !36812
  %47 = getelementptr inbounds nuw i8, ptr %_221.i.i4470, i64 4, !dbg !36805
  %_72.i.i.i4476 = load i32, ptr %47, align 4, !dbg !36805, !alias.scope !36807, !noalias !36812
  %48 = icmp eq i32 %_51.i.i.i4475, %_72.i.i.i4476, !dbg !36805
  %_0.sroa.0.0.i.i.i4477 = select i1 %46, i1 %48, i1 false, !dbg !36805
  br i1 %_0.sroa.0.0.i.i.i4477, label %bb1.i.i4469, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit, !dbg !36815

bb4.i1970:                                        ; preds = %bb1.i.i4469
  %49 = getelementptr inbounds nuw i8, ptr %self, i64 464, !dbg !36816
  %_16.0.i = load ptr, ptr %49, align 8, !dbg !36816, !alias.scope !36757, !noalias !36817, !nonnull !12, !noundef !12
  %50 = getelementptr inbounds nuw i8, ptr %self, i64 472, !dbg !36816
  %_16.1.i = load i64, ptr %50, align 8, !dbg !36816, !alias.scope !36757, !noalias !36817, !noundef !12
  %_8.i4479 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_16.0.i, i64 %_16.1.i, !dbg !36818
  br label %bb1.i.i4480, !dbg !36823

bb1.i.i4480:                                      ; preds = %bb13.i.i4483, %bb4.i1970
  %_221.i.i4481 = phi ptr [ %_22.i.i4484, %bb13.i.i4483 ], [ %_16.0.i, %bb4.i1970 ]
  %_12.i.i4482 = icmp eq ptr %_221.i.i4481, %_8.i4479, !dbg !36825
  br i1 %_12.i.i4482, label %bb6.i1971, label %bb13.i.i4483, !dbg !36828

bb13.i.i4483:                                     ; preds = %bb1.i.i4480
  %_22.i.i4484 = getelementptr inbounds nuw i8, ptr %_221.i.i4481, i64 16, !dbg !36829
  %51 = getelementptr inbounds nuw i8, ptr %_221.i.i4481, i64 12, !dbg !36831
  %_3.i.i.i4485 = load i32, ptr %51, align 4, !dbg !36831, !alias.scope !36833, !noalias !36838, !noundef !12
  %52 = icmp eq i32 %_3.i.i.i4485, 0, !dbg !36831
  %_51.i.i.i4486 = load i32, ptr %_221.i.i4481, align 4, !dbg !36831, !alias.scope !36833, !noalias !36838
  %53 = getelementptr inbounds nuw i8, ptr %_221.i.i4481, i64 4, !dbg !36831
  %_72.i.i.i4487 = load i32, ptr %53, align 4, !dbg !36831, !alias.scope !36833, !noalias !36838
  %54 = icmp eq i32 %_51.i.i.i4486, %_72.i.i.i4487, !dbg !36831
  %_0.sroa.0.0.i.i.i4488 = select i1 %52, i1 %54, i1 false, !dbg !36831
  br i1 %_0.sroa.0.0.i.i.i4488, label %bb1.i.i4480, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit, !dbg !36841

bb6.i1971:                                        ; preds = %bb1.i.i4480
  %55 = getelementptr inbounds nuw i8, ptr %self, i64 480, !dbg !36842
  %_17.0.i = load ptr, ptr %55, align 8, !dbg !36842, !alias.scope !36757, !noalias !36817, !nonnull !12, !noundef !12
  %56 = getelementptr inbounds nuw i8, ptr %self, i64 488, !dbg !36842
  %_17.1.i = load i64, ptr %56, align 8, !dbg !36842, !alias.scope !36757, !noalias !36817, !noundef !12
  %_8.i4490 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_17.0.i, i64 %_17.1.i, !dbg !36843
  br label %bb1.i.i4491, !dbg !36848

bb1.i.i4491:                                      ; preds = %bb13.i.i4494, %bb6.i1971
  %_221.i.i4492 = phi ptr [ %_22.i.i4495, %bb13.i.i4494 ], [ %_17.0.i, %bb6.i1971 ]
  %_12.i.i4493 = icmp eq ptr %_221.i.i4492, %_8.i4490, !dbg !36850
  br i1 %_12.i.i4493, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit, label %bb13.i.i4494, !dbg !36853

bb13.i.i4494:                                     ; preds = %bb1.i.i4491
  %_22.i.i4495 = getelementptr inbounds nuw i8, ptr %_221.i.i4492, i64 16, !dbg !36854
  %57 = getelementptr inbounds nuw i8, ptr %_221.i.i4492, i64 12, !dbg !36856
  %_3.i.i.i4496 = load i32, ptr %57, align 4, !dbg !36856, !alias.scope !36858, !noalias !36863, !noundef !12
  %58 = icmp eq i32 %_3.i.i.i4496, 0, !dbg !36856
  %_51.i.i.i4497 = load i32, ptr %_221.i.i4492, align 4, !dbg !36856, !alias.scope !36858, !noalias !36863
  %59 = getelementptr inbounds nuw i8, ptr %_221.i.i4492, i64 4, !dbg !36856
  %_72.i.i.i4498 = load i32, ptr %59, align 4, !dbg !36856, !alias.scope !36858, !noalias !36863
  %60 = icmp eq i32 %_51.i.i.i4497, %_72.i.i.i4498, !dbg !36856
  %_0.sroa.0.0.i.i.i4499 = select i1 %58, i1 %60, i1 false, !dbg !36856
  br i1 %_0.sroa.0.0.i.i.i4499, label %bb1.i.i4491, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit, !dbg !36866

_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit: ; preds = %bb13.i.i4461, %bb13.i.i4472, %bb13.i.i4483, %bb13.i.i4494, %bb1.i.i4491
  %_0.sroa.0.0.i = phi i1 [ false, %bb13.i.i4483 ], [ false, %bb13.i.i4472 ], [ false, %bb13.i.i4494 ], [ true, %bb1.i.i4491 ], [ false, %bb13.i.i4461 ], !dbg !36867
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36868), !dbg !36871
  %61 = getelementptr inbounds nuw i8, ptr %self, i64 312, !dbg !36873
  %_31.0.i = load ptr, ptr %61, align 8, !dbg !36873, !alias.scope !36868, !noalias !36875, !nonnull !12, !noundef !12
  %62 = getelementptr inbounds nuw i8, ptr %self, i64 320, !dbg !36873
  %_31.1.i = load i64, ptr %62, align 8, !dbg !36873, !alias.scope !36868, !noalias !36875, !noundef !12
  %_17.idx.i = mul nuw nsw i64 %_31.1.i, 12, !dbg !36876
  %_17.i = getelementptr inbounds nuw i8, ptr %_31.0.i, i64 %_17.idx.i, !dbg !36876
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36880), !dbg !36883, !noalias !36875
  %_5.not.i.i.i = icmp eq i64 %_31.1.i, 0
  %63 = getelementptr inbounds nuw i8, ptr %_31.0.i, i64 4
  %64 = getelementptr inbounds nuw i8, ptr %_31.0.i, i64 8
  br i1 %_5.not.i.i.i, label %bb2.i4509, label %bb1.i.i4501

bb1.i.i4501:                                      ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i
  %_224.i.i = phi ptr [ %_22.i.i4504, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i ], [ %_31.0.i, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit ]
  %_12.i.i4502 = icmp eq ptr %_224.i.i, %_17.i, !dbg !36884
  br i1 %_12.i.i4502, label %bb2.i4509, label %bb13.i.i4503, !dbg !36888

bb13.i.i4503:                                     ; preds = %bb1.i.i4501
  %_22.i.i4504 = getelementptr inbounds nuw i8, ptr %_224.i.i, i64 12, !dbg !36889
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36891), !dbg !36894, !noalias !36875
  %_9.i.i.i = load i32, ptr %_224.i.i, align 4, !dbg !36895, !alias.scope !36891, !noalias !36898, !noundef !12
  %_10.i.i.i = load i32, ptr %_31.0.i, align 4, !dbg !36895, !alias.scope !36880, !noalias !36900, !noundef !12
  %_8.i.i.i = icmp eq i32 %_9.i.i.i, %_10.i.i.i, !dbg !36895
  br i1 %_8.i.i.i, label %bb2.i.i.i, label %bb10.i, !dbg !36895

bb2.i.i.i:                                        ; preds = %bb13.i.i4503
  %65 = getelementptr inbounds nuw i8, ptr %_224.i.i, i64 4, !dbg !36895
  %_12.i.i.i = load i32, ptr %65, align 4, !dbg !36895, !alias.scope !36891, !noalias !36898, !noundef !12
  %_13.i.i.i = load i32, ptr %63, align 4, !dbg !36895, !alias.scope !36880, !noalias !36900, !noundef !12
  %_11.i.i.i = icmp eq i32 %_12.i.i.i, %_13.i.i.i, !dbg !36895
  br i1 %_11.i.i.i, label %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, label %bb10.i, !dbg !36895

_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i: ; preds = %bb2.i.i.i
  %66 = getelementptr inbounds nuw i8, ptr %_224.i.i, i64 8, !dbg !36895
  %_14.i.i.i4507 = load i32, ptr %66, align 4, !dbg !36895, !alias.scope !36891, !noalias !36898, !noundef !12
  %_15.i.i.i4508 = load i32, ptr %64, align 4, !dbg !36895, !alias.scope !36880, !noalias !36900, !noundef !12
  %67 = icmp eq i32 %_14.i.i.i4507, %_15.i.i.i4508, !dbg !36895
  br i1 %67, label %bb1.i.i4501, label %bb10.i, !dbg !36894

bb2.i4509:                                        ; preds = %bb1.i.i4501, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit
  %68 = getelementptr inbounds nuw i8, ptr %self, i64 248, !dbg !36901
  %_32.0.i = load ptr, ptr %68, align 8, !dbg !36901, !alias.scope !36868, !noalias !36875, !nonnull !12, !noundef !12
  %69 = getelementptr inbounds nuw i8, ptr %self, i64 256, !dbg !36901
  %_32.1.i = load i64, ptr %69, align 8, !dbg !36901, !alias.scope !36868, !noalias !36875, !noundef !12
  %_26.idx.i = shl nuw nsw i64 %_32.1.i, 2, !dbg !36902
  %_26.i4510 = getelementptr inbounds nuw i8, ptr %_32.0.i, i64 %_26.idx.i, !dbg !36902
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36906), !dbg !36909, !noalias !36875
  %_6.not.i.i.i = icmp eq i64 %_32.1.i, 0
  br i1 %_6.not.i.i.i, label %bb3.i, label %bb1.i3.i

bb1.i3.i:                                         ; preds = %bb2.i4509, %bb13.i5.i
  %_223.i.i = phi ptr [ %_22.i6.i, %bb13.i5.i ], [ %_32.0.i, %bb2.i4509 ]
  %_12.i4.i = icmp eq ptr %_223.i.i, %_26.i4510, !dbg !36910
  br i1 %_12.i4.i, label %bb3.i, label %bb13.i5.i, !dbg !36914

bb13.i5.i:                                        ; preds = %bb1.i3.i
  %_22.i6.i = getelementptr inbounds nuw i8, ptr %_223.i.i, i64 4, !dbg !36915
  %ptr.val.i.i = load i32, ptr %_223.i.i, align 4, !dbg !36917, !noalias !36918
  %_4.i.i.i4511 = load i32, ptr %_32.0.i, align 4, !dbg !36920, !alias.scope !36906, !noalias !36922, !noundef !12
  %_0.i.i.i = icmp eq i32 %ptr.val.i.i, %_4.i.i.i4511, !dbg !36923
  br i1 %_0.i.i.i, label %bb1.i3.i, label %bb10.i, !dbg !36917

bb3.i:                                            ; preds = %bb1.i3.i, %bb2.i4509
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36924), !dbg !36927
  %70 = getelementptr inbounds nuw i8, ptr %self, i64 512, !dbg !36928
  %_31.0.i4512 = load ptr, ptr %70, align 8, !dbg !36928, !alias.scope !36924, !noalias !36875, !nonnull !12, !noundef !12
  %71 = getelementptr inbounds nuw i8, ptr %self, i64 520, !dbg !36928
  %_31.1.i4513 = load i64, ptr %71, align 8, !dbg !36928, !alias.scope !36924, !noalias !36875, !noundef !12
  %_17.idx.i4514 = mul nuw nsw i64 %_31.1.i4513, 12, !dbg !36930
  %_17.i4515 = getelementptr inbounds nuw i8, ptr %_31.0.i4512, i64 %_17.idx.i4514, !dbg !36930
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36934), !dbg !36937, !noalias !36875
  %_5.not.i.i.i4516 = icmp eq i64 %_31.1.i4513, 0
  %72 = getelementptr inbounds nuw i8, ptr %_31.0.i4512, i64 4
  %73 = getelementptr inbounds nuw i8, ptr %_31.0.i4512, i64 8
  br i1 %_5.not.i.i.i4516, label %bb2.i4534, label %bb1.i.i4517

bb1.i.i4517:                                      ; preds = %bb3.i, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i4531
  %_224.i.i4518 = phi ptr [ %_22.i.i4521, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i4531 ], [ %_31.0.i4512, %bb3.i ]
  %_12.i.i4519 = icmp eq ptr %_224.i.i4518, %_17.i4515, !dbg !36938
  br i1 %_12.i.i4519, label %bb2.i4534, label %bb13.i.i4520, !dbg !36942

bb13.i.i4520:                                     ; preds = %bb1.i.i4517
  %_22.i.i4521 = getelementptr inbounds nuw i8, ptr %_224.i.i4518, i64 12, !dbg !36943
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36945), !dbg !36948, !noalias !36875
  %_9.i.i.i4522 = load i32, ptr %_224.i.i4518, align 4, !dbg !36949, !alias.scope !36945, !noalias !36952, !noundef !12
  %_10.i.i.i4523 = load i32, ptr %_31.0.i4512, align 4, !dbg !36949, !alias.scope !36934, !noalias !36954, !noundef !12
  %_8.i.i.i4524 = icmp eq i32 %_9.i.i.i4522, %_10.i.i.i4523, !dbg !36949
  br i1 %_8.i.i.i4524, label %bb2.i.i.i4527, label %bb10.i, !dbg !36949

bb2.i.i.i4527:                                    ; preds = %bb13.i.i4520
  %74 = getelementptr inbounds nuw i8, ptr %_224.i.i4518, i64 4, !dbg !36949
  %_12.i.i.i4528 = load i32, ptr %74, align 4, !dbg !36949, !alias.scope !36945, !noalias !36952, !noundef !12
  %_13.i.i.i4529 = load i32, ptr %72, align 4, !dbg !36949, !alias.scope !36934, !noalias !36954, !noundef !12
  %_11.i.i.i4530 = icmp eq i32 %_12.i.i.i4528, %_13.i.i.i4529, !dbg !36949
  br i1 %_11.i.i.i4530, label %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i4531, label %bb10.i, !dbg !36949

_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i4531: ; preds = %bb2.i.i.i4527
  %75 = getelementptr inbounds nuw i8, ptr %_224.i.i4518, i64 8, !dbg !36949
  %_14.i.i.i4532 = load i32, ptr %75, align 4, !dbg !36949, !alias.scope !36945, !noalias !36952, !noundef !12
  %_15.i.i.i4533 = load i32, ptr %73, align 4, !dbg !36949, !alias.scope !36934, !noalias !36954, !noundef !12
  %76 = icmp eq i32 %_14.i.i.i4532, %_15.i.i.i4533, !dbg !36949
  br i1 %76, label %bb1.i.i4517, label %bb10.i, !dbg !36948

bb2.i4534:                                        ; preds = %bb1.i.i4517, %bb3.i
  %77 = getelementptr inbounds nuw i8, ptr %self, i64 448, !dbg !36955
  %_32.0.i4535 = load ptr, ptr %77, align 8, !dbg !36955, !alias.scope !36924, !noalias !36875, !nonnull !12, !noundef !12
  %78 = getelementptr inbounds nuw i8, ptr %self, i64 456, !dbg !36955
  %_32.1.i4536 = load i64, ptr %78, align 8, !dbg !36955, !alias.scope !36924, !noalias !36875, !noundef !12
  %_26.idx.i4537 = shl nuw nsw i64 %_32.1.i4536, 2, !dbg !36956
  %_26.i4538 = getelementptr inbounds nuw i8, ptr %_32.0.i4535, i64 %_26.idx.i4537, !dbg !36956
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36960), !dbg !36963, !noalias !36875
  %_6.not.i.i.i4539 = icmp eq i64 %_32.1.i4536, 0
  br i1 %_6.not.i.i.i4539, label %bb5.i, label %bb1.i3.i4540

bb1.i3.i4540:                                     ; preds = %bb2.i4534, %bb13.i5.i4543
  %_223.i.i4541 = phi ptr [ %_22.i6.i4544, %bb13.i5.i4543 ], [ %_32.0.i4535, %bb2.i4534 ]
  %_12.i4.i4542 = icmp eq ptr %_223.i.i4541, %_26.i4538, !dbg !36964
  br i1 %_12.i4.i4542, label %bb5.i, label %bb13.i5.i4543, !dbg !36968

bb13.i5.i4543:                                    ; preds = %bb1.i3.i4540
  %_22.i6.i4544 = getelementptr inbounds nuw i8, ptr %_223.i.i4541, i64 4, !dbg !36969
  %ptr.val.i.i4545 = load i32, ptr %_223.i.i4541, align 4, !dbg !36971, !noalias !36972
  %_4.i.i.i4546 = load i32, ptr %_32.0.i4535, align 4, !dbg !36974, !alias.scope !36960, !noalias !36976, !noundef !12
  %_0.i.i.i4547 = icmp eq i32 %ptr.val.i.i4545, %_4.i.i.i4546, !dbg !36977
  br i1 %_0.i.i.i4547, label %bb1.i3.i4540, label %bb10.i, !dbg !36971

bb10.i:                                           ; preds = %bb13.i.i4503, %bb2.i.i.i, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, %bb13.i5.i, %bb13.i.i4520, %bb2.i.i.i4527, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i4531, %bb13.i5.i4543
  br i1 %_0.sroa.0.0.i, label %bb11.i, label %bb12.i, !dbg !36978

bb5.i:                                            ; preds = %bb1.i3.i4540, %bb2.i4534
  br i1 %_0.sroa.0.0.i, label %bb6.i, label %bb7.i, !dbg !36979

bb12.i:                                           ; preds = %bb10.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36980), !dbg !36983
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36984), !dbg !36983
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36986), !dbg !36983
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36988), !dbg !36983
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i1017), !dbg !36990, !noalias !36994
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_left.i1017, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #31, !dbg !36998, !noalias !36999
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_right.i1016), !dbg !37000, !noalias !36994
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_right.i1016, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #31, !dbg !37002, !noalias !37003
  %79 = getelementptr inbounds nuw i8, ptr %self, i64 776, !dbg !37004
  %80 = load i8, ptr %79, align 4, !dbg !37004, !range !17, !alias.scope !36980, !noalias !37008, !noundef !12
  %81 = getelementptr inbounds nuw i8, ptr %self, i64 777, !dbg !37009
  %82 = load i8, ptr %81, align 1, !dbg !37009, !range !17, !alias.scope !36980, !noalias !37008, !noundef !12
  %_34.i1025 = load i32, ptr %_35, align 4, !dbg !37011, !alias.scope !36988, !noalias !37013, !noundef !12
  %83 = getelementptr inbounds nuw i8, ptr %self, i64 564, !dbg !37014
  %_36.i1026 = load i32, ptr %83, align 4, !dbg !37014, !alias.scope !36988, !noalias !37013, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i1015), !dbg !37016, !noalias !36994
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i1015, i8 0, i64 32, i1 false), !noalias !36994
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i1014), !dbg !37018, !noalias !36994
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i1014, i8 0, i64 1024, i1 false), !noalias !36994
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i1013), !dbg !37020, !noalias !36994
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i1013, i8 0, i64 1024, i1 false), !noalias !36994
  %_31.i1021 = zext nneg i8 %80 to i32, !dbg !37004
  %.none.i1022 = sub nsw i32 0, %_31.i1021, !dbg !37022
  %_32.i1023 = zext nneg i8 %82 to i32, !dbg !37009
  %all.sroa.0.0.i1024 = sub nsw i32 0, %_32.i1023, !dbg !37009
  %_116.not.i10398192 = icmp eq i64 %frames, 0, !dbg !37023
  br i1 %_116.not.i10398192, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit, label %bb37.i1040.lr.ph, !dbg !37023

bb37.i1040.lr.ph:                                 ; preds = %bb12.i
  %84 = zext i32 %_36.i1026 to i64, !dbg !37014
  %85 = zext i32 %_34.i1025 to i64, !dbg !37011
  %d9.i.i = lshr i64 %frames, 5, !dbg !37033
  %r2.i.i = and i64 %frames, 31, !dbg !37039
  %_19.not.i.i = icmp ne i64 %r2.i.i, 0, !dbg !37040
  %86 = zext i1 %_19.not.i.i to i64, !dbg !37040
  %yield_count.sroa.0.0.i.i = add nuw nsw i64 %d9.i.i, %86, !dbg !37040
  %history.i141.i993.sroa.7.0.hot_left.i1017.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1017, i64 4
  %history.i141.i993.sroa.10.0.hot_left.i1017.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1017, i64 8
  %history.i141.i993.sroa.13.0.hot_left.i1017.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1017, i64 12
  %history.i141.i993.sroa.16.0.hot_left.i1017.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1017, i64 16
  %history.i141.i993.sroa.19.0.hot_left.i1017.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1017, i64 20
  %history.i141.i993.sroa.22.0.hot_left.i1017.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1017, i64 24
  %history.i141.i993.sroa.26.0.hot_left.i1017.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1017, i64 28
  %history.i141.i993.sroa.29.0.hot_left.i1017.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1017, i64 32
  %history.i141.i993.sroa.32.0.hot_left.i1017.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1017, i64 36
  %history.i141.i993.sroa.35.0.hot_left.i1017.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1017, i64 40
  %history.i141.i993.sroa.38.0.hot_left.i1017.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1017, i64 44
  %87 = getelementptr inbounds nuw i8, ptr %self, i64 588
  %88 = getelementptr inbounds nuw i8, ptr %self, i64 592
  %89 = getelementptr inbounds nuw i8, ptr %self, i64 596
  %row1.i.i.i174.i1085 = getelementptr inbounds nuw i8, ptr %self, i64 600
  %90 = getelementptr inbounds nuw i8, ptr %self, i64 604
  %91 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %92 = getelementptr inbounds nuw i8, ptr %self, i64 612
  %row3.i.i.i188.i1099 = getelementptr inbounds nuw i8, ptr %self, i64 616
  %93 = getelementptr inbounds nuw i8, ptr %self, i64 620
  %94 = getelementptr inbounds nuw i8, ptr %self, i64 624
  %95 = getelementptr inbounds nuw i8, ptr %self, i64 628
  %row5.i.i.i202.i1113 = getelementptr inbounds nuw i8, ptr %self, i64 632
  %96 = getelementptr inbounds nuw i8, ptr %self, i64 636
  %97 = getelementptr inbounds nuw i8, ptr %self, i64 640
  %98 = getelementptr inbounds nuw i8, ptr %self, i64 644
  %row7.i.i.i216.i1127 = getelementptr inbounds nuw i8, ptr %self, i64 648
  %99 = getelementptr inbounds nuw i8, ptr %self, i64 652
  %100 = getelementptr inbounds nuw i8, ptr %self, i64 656
  %101 = getelementptr inbounds nuw i8, ptr %self, i64 660
  %row9.i.i.i230.i1141 = getelementptr inbounds nuw i8, ptr %self, i64 664
  %102 = getelementptr inbounds nuw i8, ptr %self, i64 668
  %103 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %104 = getelementptr inbounds nuw i8, ptr %self, i64 676
  %row11.i.i.i244.i1155 = getelementptr inbounds nuw i8, ptr %self, i64 680
  %105 = getelementptr inbounds nuw i8, ptr %self, i64 684
  %106 = getelementptr inbounds nuw i8, ptr %self, i64 688
  %107 = getelementptr inbounds nuw i8, ptr %self, i64 692
  %row13.i.i.i258.i1169 = getelementptr inbounds nuw i8, ptr %self, i64 696
  %108 = getelementptr inbounds nuw i8, ptr %self, i64 700
  %109 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %110 = getelementptr inbounds nuw i8, ptr %self, i64 708
  %row15.i.i.i272.i1183 = getelementptr inbounds nuw i8, ptr %self, i64 712
  %111 = getelementptr inbounds nuw i8, ptr %self, i64 716
  %112 = getelementptr inbounds nuw i8, ptr %self, i64 720
  %113 = getelementptr inbounds nuw i8, ptr %self, i64 724
  %row17.i.i.i286.i1197 = getelementptr inbounds nuw i8, ptr %self, i64 728
  %114 = getelementptr inbounds nuw i8, ptr %self, i64 732
  %115 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %116 = getelementptr inbounds nuw i8, ptr %self, i64 740
  %row19.i.i.i300.i1211 = getelementptr inbounds nuw i8, ptr %self, i64 744
  %117 = getelementptr inbounds nuw i8, ptr %self, i64 748
  %118 = getelementptr inbounds nuw i8, ptr %self, i64 752
  %119 = getelementptr inbounds nuw i8, ptr %self, i64 756
  %row21.i.i.i314.i1225 = getelementptr inbounds nuw i8, ptr %self, i64 760
  %120 = getelementptr inbounds nuw i8, ptr %self, i64 764
  %121 = getelementptr inbounds nuw i8, ptr %self, i64 768
  %122 = getelementptr inbounds nuw i8, ptr %self, i64 772
  %history.i.i1007.sroa.7.0.hot_right.i1016.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1016, i64 4
  %history.i.i1007.sroa.10.0.hot_right.i1016.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1016, i64 8
  %history.i.i1007.sroa.13.0.hot_right.i1016.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1016, i64 12
  %history.i.i1007.sroa.16.0.hot_right.i1016.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1016, i64 16
  %history.i.i1007.sroa.19.0.hot_right.i1016.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1016, i64 20
  %history.i.i1007.sroa.22.0.hot_right.i1016.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1016, i64 24
  %history.i.i1007.sroa.26.0.hot_right.i1016.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1016, i64 28
  %history.i.i1007.sroa.29.0.hot_right.i1016.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1016, i64 32
  %history.i.i1007.sroa.32.0.hot_right.i1016.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1016, i64 36
  %history.i.i1007.sroa.35.0.hot_right.i1016.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1016, i64 40
  %history.i.i1007.sroa.38.0.hot_right.i1016.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1016, i64 44
  %_68.i1453 = getelementptr inbounds nuw i8, ptr %hot_left.i1017, i64 48
  %_69.i1454 = getelementptr inbounds nuw i8, ptr %hot_left.i1017, i64 64
  %123 = getelementptr inbounds nuw i8, ptr %hot_left.i1017, i64 60
  %124 = getelementptr inbounds nuw i8, ptr %hot_left.i1017, i64 56
  %125 = getelementptr inbounds nuw i8, ptr %hot_left.i1017, i64 52
  %126 = getelementptr inbounds nuw i8, ptr %hot_left.i1017, i64 76
  %127 = getelementptr inbounds nuw i8, ptr %hot_left.i1017, i64 72
  %128 = getelementptr inbounds nuw i8, ptr %hot_left.i1017, i64 68
  %_73.i1455 = getelementptr inbounds nuw i8, ptr %hot_right.i1016, i64 48
  %_74.i1456 = getelementptr inbounds nuw i8, ptr %hot_right.i1016, i64 64
  %129 = getelementptr inbounds nuw i8, ptr %hot_right.i1016, i64 60
  %130 = getelementptr inbounds nuw i8, ptr %hot_right.i1016, i64 56
  %131 = getelementptr inbounds nuw i8, ptr %hot_right.i1016, i64 52
  %132 = getelementptr inbounds nuw i8, ptr %hot_right.i1016, i64 76
  %133 = getelementptr inbounds nuw i8, ptr %hot_right.i1016, i64 72
  %134 = getelementptr inbounds nuw i8, ptr %hot_right.i1016, i64 68
  %_9.i3926 = add nsw i32 %_31.i1021, -1
  %135 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %136 = getelementptr inbounds nuw i8, ptr %self, i64 328
  %137 = getelementptr inbounds nuw i8, ptr %self, i64 176
  %138 = getelementptr inbounds nuw i8, ptr %self, i64 168
  %139 = getelementptr inbounds nuw i8, ptr %self, i64 256
  %140 = getelementptr inbounds nuw i8, ptr %self, i64 248
  %141 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %142 = getelementptr inbounds nuw i8, ptr %self, i64 216
  %143 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %144 = getelementptr inbounds nuw i8, ptr %self, i64 184
  %145 = getelementptr inbounds nuw i8, ptr %hot_left.i1017, i64 84
  %146 = getelementptr inbounds nuw i8, ptr %hot_left.i1017, i64 88
  %147 = getelementptr inbounds nuw i8, ptr %hot_left.i1017, i64 80
  %148 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %149 = getelementptr inbounds nuw i8, ptr %self, i64 152
  %_9.i3906 = add nsw i32 %_32.i1023, -1
  %150 = getelementptr inbounds nuw i8, ptr %self, i64 528
  %151 = getelementptr inbounds nuw i8, ptr %self, i64 376
  %152 = getelementptr inbounds nuw i8, ptr %self, i64 368
  %153 = getelementptr inbounds nuw i8, ptr %self, i64 520
  %154 = getelementptr inbounds nuw i8, ptr %self, i64 512
  %155 = getelementptr inbounds nuw i8, ptr %self, i64 456
  %156 = getelementptr inbounds nuw i8, ptr %self, i64 448
  %157 = getelementptr inbounds nuw i8, ptr %self, i64 424
  %158 = getelementptr inbounds nuw i8, ptr %self, i64 416
  %159 = getelementptr inbounds nuw i8, ptr %self, i64 392
  %160 = getelementptr inbounds nuw i8, ptr %self, i64 384
  %161 = getelementptr inbounds nuw i8, ptr %hot_right.i1016, i64 84
  %162 = getelementptr inbounds nuw i8, ptr %hot_right.i1016, i64 88
  %163 = getelementptr inbounds nuw i8, ptr %hot_right.i1016, i64 80
  %164 = getelementptr inbounds nuw i8, ptr %self, i64 360
  %165 = getelementptr inbounds nuw i8, ptr %self, i64 352
  %166 = getelementptr inbounds nuw i8, ptr %self, i64 552
  %iter.i32.i1008.sroa.0.0.ptr7335.1 = getelementptr inbounds nuw i8, ptr %scratch.i1015, i64 4
  %iter.i32.i1008.sroa.0.0.ptr7335.2 = getelementptr inbounds nuw i8, ptr %scratch.i1015, i64 8
  %iter.i32.i1008.sroa.0.0.ptr7335.3 = getelementptr inbounds nuw i8, ptr %scratch.i1015, i64 12
  %iter.i32.i1008.sroa.0.0.ptr7335.4 = getelementptr inbounds nuw i8, ptr %scratch.i1015, i64 16
  %iter.i32.i1008.sroa.0.0.ptr7335.5 = getelementptr inbounds nuw i8, ptr %scratch.i1015, i64 20
  %iter.i32.i1008.sroa.0.0.ptr7335.6 = getelementptr inbounds nuw i8, ptr %scratch.i1015, i64 24
  %iter.i32.i1008.sroa.0.0.ptr7335.7 = getelementptr inbounds nuw i8, ptr %scratch.i1015, i64 28
  %iter.i.i1009.sroa.0.0.ptr7346.1 = getelementptr inbounds nuw i8, ptr %scratch.i1015, i64 4
  %iter.i.i1009.sroa.0.0.ptr7346.2 = getelementptr inbounds nuw i8, ptr %scratch.i1015, i64 8
  %iter.i.i1009.sroa.0.0.ptr7346.3 = getelementptr inbounds nuw i8, ptr %scratch.i1015, i64 12
  %iter.i.i1009.sroa.0.0.ptr7346.4 = getelementptr inbounds nuw i8, ptr %scratch.i1015, i64 16
  %iter.i.i1009.sroa.0.0.ptr7346.5 = getelementptr inbounds nuw i8, ptr %scratch.i1015, i64 20
  %iter.i.i1009.sroa.0.0.ptr7346.6 = getelementptr inbounds nuw i8, ptr %scratch.i1015, i64 24
  %iter.i.i1009.sroa.0.0.ptr7346.7 = getelementptr inbounds nuw i8, ptr %scratch.i1015, i64 28
  br label %bb37.i1040, !dbg !37023

bb16.i1446.bb13.i1034.loopexit_crit_edge:         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit
  store float %_0.i3390, ptr %145, align 1, !dbg !37041
  store float %_0.i3386, ptr %161, align 1, !dbg !37064
  store float %_0.i.i4079, ptr %123, align 4, !dbg !37066, !alias.scope !37072, !noalias !37075
  store float %_0.i3851, ptr %_68.i1453, align 4, !dbg !37078, !alias.scope !37072, !noalias !37075
  store float %_0.i3844, ptr %124, align 4, !dbg !37080, !alias.scope !37072, !noalias !37075
  store float %_0.i.i4086, ptr %126, align 4, !dbg !37081, !alias.scope !37083, !noalias !37086
  store float %_0.i3864, ptr %_69.i1454, align 4, !dbg !37087, !alias.scope !37083, !noalias !37086
  store float %_0.i3857, ptr %127, align 4, !dbg !37088, !alias.scope !37083, !noalias !37086
  store float %_0.i.i4093, ptr %129, align 4, !dbg !37089, !alias.scope !37092, !noalias !37095
  store float %_0.i3877, ptr %_73.i1455, align 4, !dbg !37098, !alias.scope !37092, !noalias !37095
  store float %_0.i3870, ptr %130, align 4, !dbg !37099, !alias.scope !37092, !noalias !37095
  store float %_0.i.i4100, ptr %132, align 4, !dbg !37100, !alias.scope !37102, !noalias !37086
  store float %_0.i3890, ptr %_74.i1456, align 4, !dbg !37105, !alias.scope !37102, !noalias !37086
  store float %_0.i3883, ptr %133, align 4, !dbg !37106, !alias.scope !37102, !noalias !37086
  br label %bb13.i1034.loopexit, !dbg !37107

bb13.i1034.loopexit:                              ; preds = %bb16.i1446.bb13.i1034.loopexit_crit_edge, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i1445
  %ring_cursor.sroa.0.1.i1448.lcssa = phi i64 [ %spec.store.select13.i1658, %bb16.i1446.bb13.i1034.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i10378195, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i1445 ], !dbg !37113
  %main_cursor.sroa.0.1.i1449.lcssa = phi i64 [ %spec.store.select.i1656, %bb16.i1446.bb13.i1034.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i10388196, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i1445 ], !dbg !37114
  %_116.not.i1039 = icmp eq i64 %169, 0, !dbg !37023
  %indvars.iv.next = add i64 %indvars.iv, -32, !dbg !37023
  br i1 %_116.not.i1039, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit.loopexit, label %bb37.i1040, !dbg !37023

bb37.i1040:                                       ; preds = %bb37.i1040.lr.ph, %bb13.i1034.loopexit
  %indvars.iv = phi i64 [ %frames, %bb37.i1040.lr.ph ], [ %indvars.iv.next, %bb13.i1034.loopexit ]
  %main_cursor.sroa.0.0.i10388196 = phi i64 [ %85, %bb37.i1040.lr.ph ], [ %main_cursor.sroa.0.1.i1449.lcssa, %bb13.i1034.loopexit ]
  %ring_cursor.sroa.0.0.i10378195 = phi i64 [ %84, %bb37.i1040.lr.ph ], [ %ring_cursor.sroa.0.1.i1448.lcssa, %bb13.i1034.loopexit ]
  %iter2.sroa.0.0.i10368194 = phi i64 [ %yield_count.sroa.0.0.i.i, %bb37.i1040.lr.ph ], [ %169, %bb13.i1034.loopexit ]
  %iter.sroa.0.0.i10358193 = phi i64 [ 0, %bb37.i1040.lr.ph ], [ %168, %bb13.i1034.loopexit ]
  %167 = call i64 @llvm.umax.i64(i64 %indvars.iv, i64 1), !dbg !37115
  %umax11759 = call i64 @llvm.umin.i64(i64 %167, i64 32), !dbg !37115
  %168 = add i64 %iter.sroa.0.0.i10358193, 32, !dbg !37115
  %169 = add i64 %iter2.sroa.0.0.i10368194, -1, !dbg !37119
  %_45.i1042 = sub i64 %frames, %iter.sroa.0.0.i10358193, !dbg !37120
  %..i4549 = tail call noundef i64 @llvm.umin.i64(i64 %_45.i1042, i64 32), !dbg !37121
  %_51.i1044 = add i64 %..i4549, %iter.sroa.0.0.i10358193, !dbg !37125
  %_128.i1045 = icmp ult i64 %_51.i1044, %iter.sroa.0.0.i10358193, !dbg !37126
  %_122.not.i1046 = icmp ugt i64 %_51.i1044, %left_io.1
  %or.cond.i1047 = or i1 %_128.i1045, %_122.not.i1046, !dbg !37126
  br i1 %or.cond.i1047, label %bb41.i1672, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit, !dbg !37126, !prof !165

bb41.i1672:                                       ; preds = %bb37.i1040
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %iter.sroa.0.0.i10358193, i64 noundef %_51.i1044, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5cbcacccdbb7b907c55dbc42154fcf08) #30, !dbg !37133, !noalias !37086
  unreachable, !dbg !37133

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit: ; preds = %bb37.i1040
  %_131.i1052 = getelementptr inbounds nuw float, ptr %left_io.0, i64 %iter.sroa.0.0.i10358193, !dbg !37134
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37138), !dbg !37141
  %history.i141.i993.sroa.0.0.copyload = load float, ptr %hot_left.i1017, align 4, !dbg !37142, !noalias !37146
  %history.i141.i993.sroa.7.0.copyload = load float, ptr %history.i141.i993.sroa.7.0.hot_left.i1017.sroa_idx, align 4, !dbg !37142, !noalias !37146
  %history.i141.i993.sroa.10.0.copyload = load float, ptr %history.i141.i993.sroa.10.0.hot_left.i1017.sroa_idx, align 4, !dbg !37142, !noalias !37146
  %history.i141.i993.sroa.13.0.copyload = load float, ptr %history.i141.i993.sroa.13.0.hot_left.i1017.sroa_idx, align 4, !dbg !37142, !noalias !37146
  %history.i141.i993.sroa.16.0.copyload = load float, ptr %history.i141.i993.sroa.16.0.hot_left.i1017.sroa_idx, align 4, !dbg !37142, !noalias !37146
  %history.i141.i993.sroa.19.0.copyload = load float, ptr %history.i141.i993.sroa.19.0.hot_left.i1017.sroa_idx, align 4, !dbg !37142, !noalias !37146
  %history.i141.i993.sroa.22.0.copyload = load float, ptr %history.i141.i993.sroa.22.0.hot_left.i1017.sroa_idx, align 4, !dbg !37142, !noalias !37146
  %history.i141.i993.sroa.26.0.copyload = load float, ptr %history.i141.i993.sroa.26.0.hot_left.i1017.sroa_idx, align 4, !dbg !37142, !noalias !37146
  %history.i141.i993.sroa.29.0.copyload = load float, ptr %history.i141.i993.sroa.29.0.hot_left.i1017.sroa_idx, align 4, !dbg !37142, !noalias !37146
  %history.i141.i993.sroa.32.0.copyload = load float, ptr %history.i141.i993.sroa.32.0.hot_left.i1017.sroa_idx, align 4, !dbg !37142, !noalias !37146
  %history.i141.i993.sroa.35.0.copyload = load float, ptr %history.i141.i993.sroa.35.0.hot_left.i1017.sroa_idx, align 4, !dbg !37142, !noalias !37146
  %history.i141.i993.sroa.38.0.copyload = load float, ptr %history.i141.i993.sroa.38.0.hot_left.i1017.sroa_idx, align 4, !dbg !37142, !noalias !37146
  %_2.i7274.not = icmp eq i64 %frames, %iter.sroa.0.0.i10358193, !dbg !37149
  br i1 %_2.i7274.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit336.i1247, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507.lr.ph, !dbg !37149

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507.lr.ph: ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit
  %_11.i.i.i162.i1073 = load float, ptr %_31, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_14.i.i.i165.i1076 = load float, ptr %87, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_17.i.i.i168.i1079 = load float, ptr %88, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_20.i.i.i171.i1082 = load float, ptr %89, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_25.i.i.i176.i1087 = load float, ptr %row1.i.i.i174.i1085, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_28.i.i.i179.i1090 = load float, ptr %90, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_31.i.i.i182.i1093 = load float, ptr %91, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_34.i.i.i185.i1096 = load float, ptr %92, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_39.i.i.i190.i1101 = load float, ptr %row3.i.i.i188.i1099, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_42.i.i.i193.i1104 = load float, ptr %93, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_45.i.i.i196.i1107 = load float, ptr %94, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_48.i.i.i199.i1110 = load float, ptr %95, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_53.i.i.i204.i1115 = load float, ptr %row5.i.i.i202.i1113, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_56.i.i.i207.i1118 = load float, ptr %96, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_59.i.i.i210.i1121 = load float, ptr %97, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_62.i.i.i213.i1124 = load float, ptr %98, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_67.i.i.i218.i1129 = load float, ptr %row7.i.i.i216.i1127, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_70.i.i.i221.i1132 = load float, ptr %99, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_73.i.i.i224.i1135 = load float, ptr %100, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_76.i.i.i227.i1138 = load float, ptr %101, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_81.i.i.i232.i1143 = load float, ptr %row9.i.i.i230.i1141, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_84.i.i.i235.i1146 = load float, ptr %102, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_87.i.i.i238.i1149 = load float, ptr %103, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_90.i.i.i241.i1152 = load float, ptr %104, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_95.i.i.i246.i1157 = load float, ptr %row11.i.i.i244.i1155, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_98.i.i.i249.i1160 = load float, ptr %105, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_101.i.i.i252.i1163 = load float, ptr %106, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_104.i.i.i255.i1166 = load float, ptr %107, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_109.i.i.i260.i1171 = load float, ptr %row13.i.i.i258.i1169, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_112.i.i.i263.i1174 = load float, ptr %108, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_115.i.i.i266.i1177 = load float, ptr %109, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_118.i.i.i269.i1180 = load float, ptr %110, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_123.i.i.i274.i1185 = load float, ptr %row15.i.i.i272.i1183, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_126.i.i.i277.i1188 = load float, ptr %111, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_129.i.i.i280.i1191 = load float, ptr %112, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_132.i.i.i283.i1194 = load float, ptr %113, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_137.i.i.i288.i1199 = load float, ptr %row17.i.i.i286.i1197, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_140.i.i.i291.i1202 = load float, ptr %114, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_143.i.i.i294.i1205 = load float, ptr %115, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_146.i.i.i297.i1208 = load float, ptr %116, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_151.i.i.i302.i1213 = load float, ptr %row19.i.i.i300.i1211, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_154.i.i.i305.i1216 = load float, ptr %117, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_157.i.i.i308.i1219 = load float, ptr %118, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_160.i.i.i311.i1222 = load float, ptr %119, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_165.i.i.i316.i1227 = load float, ptr %row21.i.i.i314.i1225, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_168.i.i.i319.i1230 = load float, ptr %120, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_171.i.i.i322.i1233 = load float, ptr %121, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  %_174.i.i.i325.i1236 = load float, ptr %122, align 4, !alias.scope !37156, !noalias !37161, !noundef !12
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507, !dbg !37149

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507
  %iter.i137.i989.sroa.16.07286 = phi i64 [ 0, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507.lr.ph ], [ %175, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507 ]
  %history.i141.i993.sroa.35.07285 = phi float [ %history.i141.i993.sroa.35.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507.lr.ph ], [ %history.i141.i993.sroa.32.07284, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507 ]
  %history.i141.i993.sroa.32.07284 = phi float [ %history.i141.i993.sroa.32.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507.lr.ph ], [ %history.i141.i993.sroa.29.07283, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507 ]
  %history.i141.i993.sroa.29.07283 = phi float [ %history.i141.i993.sroa.29.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507.lr.ph ], [ %history.i141.i993.sroa.26.07282, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507 ]
  %history.i141.i993.sroa.26.07282 = phi float [ %history.i141.i993.sroa.26.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507.lr.ph ], [ %history.i141.i993.sroa.22.07281, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507 ]
  %history.i141.i993.sroa.22.07281 = phi float [ %history.i141.i993.sroa.22.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507.lr.ph ], [ %history.i141.i993.sroa.19.07280, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507 ]
  %history.i141.i993.sroa.19.07280 = phi float [ %history.i141.i993.sroa.19.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507.lr.ph ], [ %history.i141.i993.sroa.16.07279, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507 ]
  %history.i141.i993.sroa.16.07279 = phi float [ %history.i141.i993.sroa.16.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507.lr.ph ], [ %history.i141.i993.sroa.13.07278, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507 ]
  %history.i141.i993.sroa.13.07278 = phi float [ %history.i141.i993.sroa.13.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507.lr.ph ], [ %history.i141.i993.sroa.10.07277, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507 ]
  %history.i141.i993.sroa.10.07277 = phi float [ %history.i141.i993.sroa.10.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507.lr.ph ], [ %history.i141.i993.sroa.7.07276, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507 ]
  %history.i141.i993.sroa.7.07276 = phi float [ %history.i141.i993.sroa.7.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507.lr.ph ], [ %history.i141.i993.sroa.0.07275, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507 ]
  %history.i141.i993.sroa.0.07275 = phi float [ %history.i141.i993.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507.lr.ph ], [ %_0.i3505, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507 ]
  %data.i.i4555 = getelementptr inbounds nuw float, ptr %_131.i1052, i64 %iter.i137.i989.sroa.16.07286, !dbg !37166
  %_0.i3505 = load float, ptr %data.i.i4555, align 4, !dbg !37169, !alias.scope !37172, !noalias !37175, !noundef !12
  %170 = tail call noundef float @llvm.fabs.f32(float %history.i141.i993.sroa.19.07280), !dbg !37176
  %_0.i3068 = fmul float %_0.i3505, %_11.i.i.i162.i1073, !dbg !37182
  %_0.i2640 = fadd float %_0.i3068, 0.000000e+00, !dbg !37194
  %_0.i3067 = fmul float %_0.i3505, %_14.i.i.i165.i1076, !dbg !37197
  %_0.i2639 = fadd float %_0.i3067, 0.000000e+00, !dbg !37199
  %_0.i3066 = fmul float %_0.i3505, %_17.i.i.i168.i1079, !dbg !37201
  %_0.i2638 = fadd float %_0.i3066, 0.000000e+00, !dbg !37203
  %_0.i3065 = fmul float %_0.i3505, %_20.i.i.i171.i1082, !dbg !37205
  %_0.i2637 = fadd float %_0.i3065, 0.000000e+00, !dbg !37207
  %_0.i3064 = fmul float %history.i141.i993.sroa.0.07275, %_25.i.i.i176.i1087, !dbg !37209
  %_0.i2636 = fadd float %_0.i2640, %_0.i3064, !dbg !37213
  %_0.i3063 = fmul float %history.i141.i993.sroa.0.07275, %_28.i.i.i179.i1090, !dbg !37215
  %_0.i2635 = fadd float %_0.i2639, %_0.i3063, !dbg !37217
  %_0.i3062 = fmul float %history.i141.i993.sroa.0.07275, %_31.i.i.i182.i1093, !dbg !37219
  %_0.i2634 = fadd float %_0.i2638, %_0.i3062, !dbg !37221
  %_0.i3061 = fmul float %history.i141.i993.sroa.0.07275, %_34.i.i.i185.i1096, !dbg !37223
  %_0.i2633 = fadd float %_0.i2637, %_0.i3061, !dbg !37225
  %_0.i3060 = fmul float %history.i141.i993.sroa.7.07276, %_39.i.i.i190.i1101, !dbg !37227
  %_0.i2632 = fadd float %_0.i2636, %_0.i3060, !dbg !37231
  %_0.i3059 = fmul float %history.i141.i993.sroa.7.07276, %_42.i.i.i193.i1104, !dbg !37233
  %_0.i2631 = fadd float %_0.i2635, %_0.i3059, !dbg !37235
  %_0.i3058 = fmul float %history.i141.i993.sroa.7.07276, %_45.i.i.i196.i1107, !dbg !37237
  %_0.i2630 = fadd float %_0.i2634, %_0.i3058, !dbg !37239
  %_0.i3057 = fmul float %history.i141.i993.sroa.7.07276, %_48.i.i.i199.i1110, !dbg !37241
  %_0.i2629 = fadd float %_0.i2633, %_0.i3057, !dbg !37243
  %_0.i3056 = fmul float %history.i141.i993.sroa.10.07277, %_53.i.i.i204.i1115, !dbg !37245
  %_0.i2628 = fadd float %_0.i2632, %_0.i3056, !dbg !37249
  %_0.i3055 = fmul float %history.i141.i993.sroa.10.07277, %_56.i.i.i207.i1118, !dbg !37251
  %_0.i2627 = fadd float %_0.i2631, %_0.i3055, !dbg !37253
  %_0.i3054 = fmul float %history.i141.i993.sroa.10.07277, %_59.i.i.i210.i1121, !dbg !37255
  %_0.i2626 = fadd float %_0.i2630, %_0.i3054, !dbg !37257
  %_0.i3053 = fmul float %history.i141.i993.sroa.10.07277, %_62.i.i.i213.i1124, !dbg !37259
  %_0.i2625 = fadd float %_0.i2629, %_0.i3053, !dbg !37261
  %_0.i3052 = fmul float %history.i141.i993.sroa.13.07278, %_67.i.i.i218.i1129, !dbg !37263
  %_0.i2624 = fadd float %_0.i2628, %_0.i3052, !dbg !37267
  %_0.i3051 = fmul float %history.i141.i993.sroa.13.07278, %_70.i.i.i221.i1132, !dbg !37269
  %_0.i2623 = fadd float %_0.i2627, %_0.i3051, !dbg !37271
  %_0.i3050 = fmul float %history.i141.i993.sroa.13.07278, %_73.i.i.i224.i1135, !dbg !37273
  %_0.i2622 = fadd float %_0.i2626, %_0.i3050, !dbg !37275
  %_0.i3049 = fmul float %history.i141.i993.sroa.13.07278, %_76.i.i.i227.i1138, !dbg !37277
  %_0.i2621 = fadd float %_0.i2625, %_0.i3049, !dbg !37279
  %_0.i3048 = fmul float %history.i141.i993.sroa.16.07279, %_81.i.i.i232.i1143, !dbg !37281
  %_0.i2620 = fadd float %_0.i2624, %_0.i3048, !dbg !37285
  %_0.i3047 = fmul float %history.i141.i993.sroa.16.07279, %_84.i.i.i235.i1146, !dbg !37287
  %_0.i2619 = fadd float %_0.i2623, %_0.i3047, !dbg !37289
  %_0.i3046 = fmul float %history.i141.i993.sroa.16.07279, %_87.i.i.i238.i1149, !dbg !37291
  %_0.i2618 = fadd float %_0.i2622, %_0.i3046, !dbg !37293
  %_0.i3045 = fmul float %history.i141.i993.sroa.16.07279, %_90.i.i.i241.i1152, !dbg !37295
  %_0.i2617 = fadd float %_0.i2621, %_0.i3045, !dbg !37297
  %_0.i3044 = fmul float %history.i141.i993.sroa.19.07280, %_95.i.i.i246.i1157, !dbg !37299
  %_0.i2616 = fadd float %_0.i2620, %_0.i3044, !dbg !37303
  %_0.i3043 = fmul float %history.i141.i993.sroa.19.07280, %_98.i.i.i249.i1160, !dbg !37305
  %_0.i2615 = fadd float %_0.i2619, %_0.i3043, !dbg !37307
  %_0.i3042 = fmul float %history.i141.i993.sroa.19.07280, %_101.i.i.i252.i1163, !dbg !37309
  %_0.i2614 = fadd float %_0.i2618, %_0.i3042, !dbg !37311
  %_0.i3041 = fmul float %history.i141.i993.sroa.19.07280, %_104.i.i.i255.i1166, !dbg !37313
  %_0.i2613 = fadd float %_0.i2617, %_0.i3041, !dbg !37315
  %_0.i3040 = fmul float %history.i141.i993.sroa.22.07281, %_109.i.i.i260.i1171, !dbg !37317
  %_0.i2612 = fadd float %_0.i2616, %_0.i3040, !dbg !37321
  %_0.i3039 = fmul float %history.i141.i993.sroa.22.07281, %_112.i.i.i263.i1174, !dbg !37323
  %_0.i2611 = fadd float %_0.i2615, %_0.i3039, !dbg !37325
  %_0.i3038 = fmul float %history.i141.i993.sroa.22.07281, %_115.i.i.i266.i1177, !dbg !37327
  %_0.i2610 = fadd float %_0.i2614, %_0.i3038, !dbg !37329
  %_0.i3037 = fmul float %history.i141.i993.sroa.22.07281, %_118.i.i.i269.i1180, !dbg !37331
  %_0.i2609 = fadd float %_0.i2613, %_0.i3037, !dbg !37333
  %_0.i3036 = fmul float %history.i141.i993.sroa.26.07282, %_123.i.i.i274.i1185, !dbg !37335
  %_0.i2608 = fadd float %_0.i2612, %_0.i3036, !dbg !37339
  %_0.i3035 = fmul float %history.i141.i993.sroa.26.07282, %_126.i.i.i277.i1188, !dbg !37341
  %_0.i2607 = fadd float %_0.i2611, %_0.i3035, !dbg !37343
  %_0.i3034 = fmul float %history.i141.i993.sroa.26.07282, %_129.i.i.i280.i1191, !dbg !37345
  %_0.i2606 = fadd float %_0.i2610, %_0.i3034, !dbg !37347
  %_0.i3033 = fmul float %history.i141.i993.sroa.26.07282, %_132.i.i.i283.i1194, !dbg !37349
  %_0.i2605 = fadd float %_0.i2609, %_0.i3033, !dbg !37351
  %_0.i3032 = fmul float %history.i141.i993.sroa.29.07283, %_137.i.i.i288.i1199, !dbg !37353
  %_0.i2604 = fadd float %_0.i2608, %_0.i3032, !dbg !37357
  %_0.i3031 = fmul float %history.i141.i993.sroa.29.07283, %_140.i.i.i291.i1202, !dbg !37359
  %_0.i2603 = fadd float %_0.i2607, %_0.i3031, !dbg !37361
  %_0.i3030 = fmul float %history.i141.i993.sroa.29.07283, %_143.i.i.i294.i1205, !dbg !37363
  %_0.i2602 = fadd float %_0.i2606, %_0.i3030, !dbg !37365
  %_0.i3029 = fmul float %history.i141.i993.sroa.29.07283, %_146.i.i.i297.i1208, !dbg !37367
  %_0.i2601 = fadd float %_0.i2605, %_0.i3029, !dbg !37369
  %_0.i3028 = fmul float %history.i141.i993.sroa.32.07284, %_151.i.i.i302.i1213, !dbg !37371
  %_0.i2600 = fadd float %_0.i2604, %_0.i3028, !dbg !37375
  %_0.i3027 = fmul float %history.i141.i993.sroa.32.07284, %_154.i.i.i305.i1216, !dbg !37377
  %_0.i2599 = fadd float %_0.i2603, %_0.i3027, !dbg !37379
  %_0.i3026 = fmul float %history.i141.i993.sroa.32.07284, %_157.i.i.i308.i1219, !dbg !37381
  %_0.i2598 = fadd float %_0.i2602, %_0.i3026, !dbg !37383
  %_0.i3025 = fmul float %history.i141.i993.sroa.32.07284, %_160.i.i.i311.i1222, !dbg !37385
  %_0.i2597 = fadd float %_0.i2601, %_0.i3025, !dbg !37387
  %_0.i3024 = fmul float %history.i141.i993.sroa.35.07285, %_165.i.i.i316.i1227, !dbg !37389
  %_0.i2596 = fadd float %_0.i2600, %_0.i3024, !dbg !37393
  %_0.i3023 = fmul float %history.i141.i993.sroa.35.07285, %_168.i.i.i319.i1230, !dbg !37395
  %_0.i2595 = fadd float %_0.i2599, %_0.i3023, !dbg !37397
  %_0.i3022 = fmul float %history.i141.i993.sroa.35.07285, %_171.i.i.i322.i1233, !dbg !37399
  %_0.i2594 = fadd float %_0.i2598, %_0.i3022, !dbg !37401
  %_0.i3021 = fmul float %history.i141.i993.sroa.35.07285, %_174.i.i.i325.i1236, !dbg !37403
  %_0.i2593 = fadd float %_0.i2597, %_0.i3021, !dbg !37405
  %171 = tail call noundef float @llvm.fabs.f32(float %_0.i2596), !dbg !37407
  %_3.i.i4136.inv = fcmp ogt float %170, %171, !dbg !37411
  %_4.i.i4143.v = select i1 %_3.i.i4136.inv, float %170, float %171, !dbg !37411
  %172 = tail call noundef float @llvm.fabs.f32(float %_0.i2595), !dbg !37407
  %_3.i.i4136.inv.1 = fcmp ogt float %_4.i.i4143.v, %172, !dbg !37411
  %_4.i.i4143.v.1 = select i1 %_3.i.i4136.inv.1, float %_4.i.i4143.v, float %172, !dbg !37411
  %173 = tail call noundef float @llvm.fabs.f32(float %_0.i2594), !dbg !37407
  %_3.i.i4136.inv.2 = fcmp ogt float %_4.i.i4143.v.1, %173, !dbg !37411
  %_4.i.i4143.v.2 = select i1 %_3.i.i4136.inv.2, float %_4.i.i4143.v.1, float %173, !dbg !37411
  %174 = tail call noundef float @llvm.fabs.f32(float %_0.i2593), !dbg !37407
  %_3.i.i4136.inv.3 = fcmp ogt float %_4.i.i4143.v.2, %174, !dbg !37411
  %_4.i.i4143.v.3 = select i1 %_3.i.i4136.inv.3, float %_4.i.i4143.v.2, float %174, !dbg !37411
  %175 = add nuw nsw i64 %iter.i137.i989.sroa.16.07286, 1, !dbg !37417
  %data.i4.i = getelementptr inbounds nuw float, ptr %peaks_left.i1014, i64 %iter.i137.i989.sroa.16.07286, !dbg !37418
  store float %_4.i.i4143.v.3, ptr %data.i4.i, align 4, !dbg !37421, !alias.scope !37423, !noalias !37175
  %exitcond.not = icmp eq i64 %175, %umax11759, !dbg !37149
  br i1 %exitcond.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit336.i1247, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507, !dbg !37149

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit336.i1247: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit
  %history.i141.i993.sroa.0.0.lcssa = phi float [ %history.i141.i993.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit ], [ %_0.i3505, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507 ], !dbg !37426
  %history.i141.i993.sroa.7.0.lcssa = phi float [ %history.i141.i993.sroa.7.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit ], [ %history.i141.i993.sroa.0.07275, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507 ], !dbg !37426
  %history.i141.i993.sroa.10.0.lcssa = phi float [ %history.i141.i993.sroa.10.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit ], [ %history.i141.i993.sroa.7.07276, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507 ], !dbg !37426
  %history.i141.i993.sroa.13.0.lcssa = phi float [ %history.i141.i993.sroa.13.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit ], [ %history.i141.i993.sroa.10.07277, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507 ], !dbg !37426
  %history.i141.i993.sroa.16.0.lcssa = phi float [ %history.i141.i993.sroa.16.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit ], [ %history.i141.i993.sroa.13.07278, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507 ], !dbg !37426
  %history.i141.i993.sroa.19.0.lcssa = phi float [ %history.i141.i993.sroa.19.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit ], [ %history.i141.i993.sroa.16.07279, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507 ], !dbg !37426
  %history.i141.i993.sroa.22.0.lcssa = phi float [ %history.i141.i993.sroa.22.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit ], [ %history.i141.i993.sroa.19.07280, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507 ], !dbg !37426
  %history.i141.i993.sroa.26.0.lcssa = phi float [ %history.i141.i993.sroa.26.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit ], [ %history.i141.i993.sroa.22.07281, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507 ], !dbg !37426
  %history.i141.i993.sroa.29.0.lcssa = phi float [ %history.i141.i993.sroa.29.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit ], [ %history.i141.i993.sroa.26.07282, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507 ], !dbg !37426
  %history.i141.i993.sroa.32.0.lcssa = phi float [ %history.i141.i993.sroa.32.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit ], [ %history.i141.i993.sroa.29.07283, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507 ], !dbg !37426
  %history.i141.i993.sroa.35.0.lcssa = phi float [ %history.i141.i993.sroa.35.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit ], [ %history.i141.i993.sroa.32.07284, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507 ], !dbg !37426
  %history.i141.i993.sroa.38.0.lcssa = phi float [ %history.i141.i993.sroa.38.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit ], [ %history.i141.i993.sroa.35.07285, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3507 ], !dbg !37426
  store float %history.i141.i993.sroa.0.0.lcssa, ptr %hot_left.i1017, align 4, !dbg !37427, !noalias !37146
  store float %history.i141.i993.sroa.7.0.lcssa, ptr %history.i141.i993.sroa.7.0.hot_left.i1017.sroa_idx, align 4, !dbg !37427, !noalias !37146
  store float %history.i141.i993.sroa.10.0.lcssa, ptr %history.i141.i993.sroa.10.0.hot_left.i1017.sroa_idx, align 4, !dbg !37427, !noalias !37146
  store float %history.i141.i993.sroa.13.0.lcssa, ptr %history.i141.i993.sroa.13.0.hot_left.i1017.sroa_idx, align 4, !dbg !37427, !noalias !37146
  store float %history.i141.i993.sroa.16.0.lcssa, ptr %history.i141.i993.sroa.16.0.hot_left.i1017.sroa_idx, align 4, !dbg !37427, !noalias !37146
  store float %history.i141.i993.sroa.19.0.lcssa, ptr %history.i141.i993.sroa.19.0.hot_left.i1017.sroa_idx, align 4, !dbg !37427, !noalias !37146
  store float %history.i141.i993.sroa.22.0.lcssa, ptr %history.i141.i993.sroa.22.0.hot_left.i1017.sroa_idx, align 4, !dbg !37427, !noalias !37146
  store float %history.i141.i993.sroa.26.0.lcssa, ptr %history.i141.i993.sroa.26.0.hot_left.i1017.sroa_idx, align 4, !dbg !37427, !noalias !37146
  store float %history.i141.i993.sroa.29.0.lcssa, ptr %history.i141.i993.sroa.29.0.hot_left.i1017.sroa_idx, align 4, !dbg !37427, !noalias !37146
  store float %history.i141.i993.sroa.32.0.lcssa, ptr %history.i141.i993.sroa.32.0.hot_left.i1017.sroa_idx, align 4, !dbg !37427, !noalias !37146
  store float %history.i141.i993.sroa.35.0.lcssa, ptr %history.i141.i993.sroa.35.0.hot_left.i1017.sroa_idx, align 4, !dbg !37427, !noalias !37146
  store float %history.i141.i993.sroa.38.0.lcssa, ptr %history.i141.i993.sroa.38.0.hot_left.i1017.sroa_idx, align 4, !dbg !37427, !noalias !37146
  %_139.not.i1248 = icmp ugt i64 %_51.i1044, %right_io.1, !dbg !37428
  br i1 %_139.not.i1248, label %bb47.i1671, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4588, !dbg !37428, !prof !1406

bb47.i1671:                                       ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit336.i1247
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %iter.sroa.0.0.i10358193, i64 noundef %_51.i1044, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f6bbc99b95dcf27c100d71299ef7abde) #30, !dbg !37432, !noalias !37086
  unreachable, !dbg !37432

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4588: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit336.i1247
  %_146.i1250 = getelementptr inbounds nuw float, ptr %right_io.0, i64 %iter.sroa.0.0.i10358193, !dbg !37433
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37437), !dbg !37440
  %history.i.i1007.sroa.0.0.copyload = load float, ptr %hot_right.i1016, align 4, !dbg !37441, !noalias !37443
  %history.i.i1007.sroa.7.0.copyload = load float, ptr %history.i.i1007.sroa.7.0.hot_right.i1016.sroa_idx, align 4, !dbg !37441, !noalias !37443
  %history.i.i1007.sroa.10.0.copyload = load float, ptr %history.i.i1007.sroa.10.0.hot_right.i1016.sroa_idx, align 4, !dbg !37441, !noalias !37443
  %history.i.i1007.sroa.13.0.copyload = load float, ptr %history.i.i1007.sroa.13.0.hot_right.i1016.sroa_idx, align 4, !dbg !37441, !noalias !37443
  %history.i.i1007.sroa.16.0.copyload = load float, ptr %history.i.i1007.sroa.16.0.hot_right.i1016.sroa_idx, align 4, !dbg !37441, !noalias !37443
  %history.i.i1007.sroa.19.0.copyload = load float, ptr %history.i.i1007.sroa.19.0.hot_right.i1016.sroa_idx, align 4, !dbg !37441, !noalias !37443
  %history.i.i1007.sroa.22.0.copyload = load float, ptr %history.i.i1007.sroa.22.0.hot_right.i1016.sroa_idx, align 4, !dbg !37441, !noalias !37443
  %history.i.i1007.sroa.26.0.copyload = load float, ptr %history.i.i1007.sroa.26.0.hot_right.i1016.sroa_idx, align 4, !dbg !37441, !noalias !37443
  %history.i.i1007.sroa.29.0.copyload = load float, ptr %history.i.i1007.sroa.29.0.hot_right.i1016.sroa_idx, align 4, !dbg !37441, !noalias !37443
  %history.i.i1007.sroa.32.0.copyload = load float, ptr %history.i.i1007.sroa.32.0.hot_right.i1016.sroa_idx, align 4, !dbg !37441, !noalias !37443
  %history.i.i1007.sroa.35.0.copyload = load float, ptr %history.i.i1007.sroa.35.0.hot_right.i1016.sroa_idx, align 4, !dbg !37441, !noalias !37443
  %history.i.i1007.sroa.38.0.copyload = load float, ptr %history.i.i1007.sroa.38.0.hot_right.i1016.sroa_idx, align 4, !dbg !37441, !noalias !37443
  br i1 %_2.i7274.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i1445, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502.lr.ph, !dbg !37446

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502.lr.ph: ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4588
  %_11.i.i.i.i1271 = load float, ptr %_31, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_14.i.i.i.i1274 = load float, ptr %87, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_17.i.i.i.i1277 = load float, ptr %88, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_20.i.i.i.i1280 = load float, ptr %89, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_25.i.i.i.i1285 = load float, ptr %row1.i.i.i174.i1085, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_28.i.i.i.i1288 = load float, ptr %90, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_31.i.i.i.i1291 = load float, ptr %91, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_34.i.i.i.i1294 = load float, ptr %92, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_39.i.i.i.i1299 = load float, ptr %row3.i.i.i188.i1099, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_42.i.i.i.i1302 = load float, ptr %93, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_45.i.i.i.i1305 = load float, ptr %94, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_48.i.i.i.i1308 = load float, ptr %95, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_53.i.i.i.i1313 = load float, ptr %row5.i.i.i202.i1113, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_56.i.i.i.i1316 = load float, ptr %96, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_59.i.i.i.i1319 = load float, ptr %97, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_62.i.i.i.i1322 = load float, ptr %98, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_67.i.i.i.i1327 = load float, ptr %row7.i.i.i216.i1127, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_70.i.i.i.i1330 = load float, ptr %99, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_73.i.i.i.i1333 = load float, ptr %100, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_76.i.i.i.i1336 = load float, ptr %101, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_81.i.i.i.i1341 = load float, ptr %row9.i.i.i230.i1141, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_84.i.i.i.i1344 = load float, ptr %102, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_87.i.i.i.i1347 = load float, ptr %103, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_90.i.i.i.i1350 = load float, ptr %104, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_95.i.i.i.i1355 = load float, ptr %row11.i.i.i244.i1155, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_98.i.i.i.i1358 = load float, ptr %105, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_101.i.i.i.i1361 = load float, ptr %106, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_104.i.i.i.i1364 = load float, ptr %107, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_109.i.i.i.i1369 = load float, ptr %row13.i.i.i258.i1169, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_112.i.i.i.i1372 = load float, ptr %108, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_115.i.i.i.i1375 = load float, ptr %109, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_118.i.i.i.i1378 = load float, ptr %110, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_123.i.i.i.i1383 = load float, ptr %row15.i.i.i272.i1183, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_126.i.i.i.i1386 = load float, ptr %111, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_129.i.i.i.i1389 = load float, ptr %112, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_132.i.i.i.i1392 = load float, ptr %113, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_137.i.i.i.i1397 = load float, ptr %row17.i.i.i286.i1197, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_140.i.i.i.i1400 = load float, ptr %114, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_143.i.i.i.i1403 = load float, ptr %115, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_146.i.i.i.i1406 = load float, ptr %116, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_151.i.i.i.i1411 = load float, ptr %row19.i.i.i300.i1211, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_154.i.i.i.i1414 = load float, ptr %117, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_157.i.i.i.i1417 = load float, ptr %118, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_160.i.i.i.i1420 = load float, ptr %119, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_165.i.i.i.i1425 = load float, ptr %row21.i.i.i314.i1225, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_168.i.i.i.i1428 = load float, ptr %120, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_171.i.i.i.i1431 = load float, ptr %121, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  %_174.i.i.i.i1434 = load float, ptr %122, align 4, !alias.scope !37449, !noalias !37454, !noundef !12
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502, !dbg !37446

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502
  %iter.i126.i1003.sroa.16.07312 = phi i64 [ 0, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502.lr.ph ], [ %181, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502 ]
  %history.i.i1007.sroa.35.07311 = phi float [ %history.i.i1007.sroa.35.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502.lr.ph ], [ %history.i.i1007.sroa.32.07310, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502 ]
  %history.i.i1007.sroa.32.07310 = phi float [ %history.i.i1007.sroa.32.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502.lr.ph ], [ %history.i.i1007.sroa.29.07309, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502 ]
  %history.i.i1007.sroa.29.07309 = phi float [ %history.i.i1007.sroa.29.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502.lr.ph ], [ %history.i.i1007.sroa.26.07308, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502 ]
  %history.i.i1007.sroa.26.07308 = phi float [ %history.i.i1007.sroa.26.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502.lr.ph ], [ %history.i.i1007.sroa.22.07307, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502 ]
  %history.i.i1007.sroa.22.07307 = phi float [ %history.i.i1007.sroa.22.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502.lr.ph ], [ %history.i.i1007.sroa.19.07306, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502 ]
  %history.i.i1007.sroa.19.07306 = phi float [ %history.i.i1007.sroa.19.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502.lr.ph ], [ %history.i.i1007.sroa.16.07305, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502 ]
  %history.i.i1007.sroa.16.07305 = phi float [ %history.i.i1007.sroa.16.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502.lr.ph ], [ %history.i.i1007.sroa.13.07304, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502 ]
  %history.i.i1007.sroa.13.07304 = phi float [ %history.i.i1007.sroa.13.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502.lr.ph ], [ %history.i.i1007.sroa.10.07303, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502 ]
  %history.i.i1007.sroa.10.07303 = phi float [ %history.i.i1007.sroa.10.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502.lr.ph ], [ %history.i.i1007.sroa.7.07302, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502 ]
  %history.i.i1007.sroa.7.07302 = phi float [ %history.i.i1007.sroa.7.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502.lr.ph ], [ %history.i.i1007.sroa.0.07301, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502 ]
  %history.i.i1007.sroa.0.07301 = phi float [ %history.i.i1007.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502.lr.ph ], [ %_0.i3500, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502 ]
  %data.i.i4598 = getelementptr inbounds nuw float, ptr %_146.i1250, i64 %iter.i126.i1003.sroa.16.07312, !dbg !37459
  %_0.i3500 = load float, ptr %data.i.i4598, align 4, !dbg !37462, !alias.scope !37464, !noalias !37467, !noundef !12
  %176 = tail call noundef float @llvm.fabs.f32(float %history.i.i1007.sroa.19.07306), !dbg !37468
  %_0.i3020 = fmul float %_0.i3500, %_11.i.i.i.i1271, !dbg !37471
  %_0.i2592 = fadd float %_0.i3020, 0.000000e+00, !dbg !37474
  %_0.i3019 = fmul float %_0.i3500, %_14.i.i.i.i1274, !dbg !37476
  %_0.i2591 = fadd float %_0.i3019, 0.000000e+00, !dbg !37478
  %_0.i3018 = fmul float %_0.i3500, %_17.i.i.i.i1277, !dbg !37480
  %_0.i2590 = fadd float %_0.i3018, 0.000000e+00, !dbg !37482
  %_0.i3017 = fmul float %_0.i3500, %_20.i.i.i.i1280, !dbg !37484
  %_0.i2589 = fadd float %_0.i3017, 0.000000e+00, !dbg !37486
  %_0.i3016 = fmul float %history.i.i1007.sroa.0.07301, %_25.i.i.i.i1285, !dbg !37488
  %_0.i2588 = fadd float %_0.i2592, %_0.i3016, !dbg !37490
  %_0.i3015 = fmul float %history.i.i1007.sroa.0.07301, %_28.i.i.i.i1288, !dbg !37492
  %_0.i2587 = fadd float %_0.i2591, %_0.i3015, !dbg !37494
  %_0.i3014 = fmul float %history.i.i1007.sroa.0.07301, %_31.i.i.i.i1291, !dbg !37496
  %_0.i2586 = fadd float %_0.i2590, %_0.i3014, !dbg !37498
  %_0.i3013 = fmul float %history.i.i1007.sroa.0.07301, %_34.i.i.i.i1294, !dbg !37500
  %_0.i2585 = fadd float %_0.i2589, %_0.i3013, !dbg !37502
  %_0.i3012 = fmul float %history.i.i1007.sroa.7.07302, %_39.i.i.i.i1299, !dbg !37504
  %_0.i2584 = fadd float %_0.i2588, %_0.i3012, !dbg !37506
  %_0.i3011 = fmul float %history.i.i1007.sroa.7.07302, %_42.i.i.i.i1302, !dbg !37508
  %_0.i2583 = fadd float %_0.i2587, %_0.i3011, !dbg !37510
  %_0.i3010 = fmul float %history.i.i1007.sroa.7.07302, %_45.i.i.i.i1305, !dbg !37512
  %_0.i2582 = fadd float %_0.i2586, %_0.i3010, !dbg !37514
  %_0.i3009 = fmul float %history.i.i1007.sroa.7.07302, %_48.i.i.i.i1308, !dbg !37516
  %_0.i2581 = fadd float %_0.i2585, %_0.i3009, !dbg !37518
  %_0.i3008 = fmul float %history.i.i1007.sroa.10.07303, %_53.i.i.i.i1313, !dbg !37520
  %_0.i2580 = fadd float %_0.i2584, %_0.i3008, !dbg !37522
  %_0.i3007 = fmul float %history.i.i1007.sroa.10.07303, %_56.i.i.i.i1316, !dbg !37524
  %_0.i2579 = fadd float %_0.i2583, %_0.i3007, !dbg !37526
  %_0.i3006 = fmul float %history.i.i1007.sroa.10.07303, %_59.i.i.i.i1319, !dbg !37528
  %_0.i2578 = fadd float %_0.i2582, %_0.i3006, !dbg !37530
  %_0.i3005 = fmul float %history.i.i1007.sroa.10.07303, %_62.i.i.i.i1322, !dbg !37532
  %_0.i2577 = fadd float %_0.i2581, %_0.i3005, !dbg !37534
  %_0.i3004 = fmul float %history.i.i1007.sroa.13.07304, %_67.i.i.i.i1327, !dbg !37536
  %_0.i2576 = fadd float %_0.i2580, %_0.i3004, !dbg !37538
  %_0.i3003 = fmul float %history.i.i1007.sroa.13.07304, %_70.i.i.i.i1330, !dbg !37540
  %_0.i2575 = fadd float %_0.i2579, %_0.i3003, !dbg !37542
  %_0.i3002 = fmul float %history.i.i1007.sroa.13.07304, %_73.i.i.i.i1333, !dbg !37544
  %_0.i2574 = fadd float %_0.i2578, %_0.i3002, !dbg !37546
  %_0.i3001 = fmul float %history.i.i1007.sroa.13.07304, %_76.i.i.i.i1336, !dbg !37548
  %_0.i2573 = fadd float %_0.i2577, %_0.i3001, !dbg !37550
  %_0.i3000 = fmul float %history.i.i1007.sroa.16.07305, %_81.i.i.i.i1341, !dbg !37552
  %_0.i2572 = fadd float %_0.i2576, %_0.i3000, !dbg !37554
  %_0.i2999 = fmul float %history.i.i1007.sroa.16.07305, %_84.i.i.i.i1344, !dbg !37556
  %_0.i2571 = fadd float %_0.i2575, %_0.i2999, !dbg !37558
  %_0.i2998 = fmul float %history.i.i1007.sroa.16.07305, %_87.i.i.i.i1347, !dbg !37560
  %_0.i2570 = fadd float %_0.i2574, %_0.i2998, !dbg !37562
  %_0.i2997 = fmul float %history.i.i1007.sroa.16.07305, %_90.i.i.i.i1350, !dbg !37564
  %_0.i2569 = fadd float %_0.i2573, %_0.i2997, !dbg !37566
  %_0.i2996 = fmul float %history.i.i1007.sroa.19.07306, %_95.i.i.i.i1355, !dbg !37568
  %_0.i2568 = fadd float %_0.i2572, %_0.i2996, !dbg !37570
  %_0.i2995 = fmul float %history.i.i1007.sroa.19.07306, %_98.i.i.i.i1358, !dbg !37572
  %_0.i2567 = fadd float %_0.i2571, %_0.i2995, !dbg !37574
  %_0.i2994 = fmul float %history.i.i1007.sroa.19.07306, %_101.i.i.i.i1361, !dbg !37576
  %_0.i2566 = fadd float %_0.i2570, %_0.i2994, !dbg !37578
  %_0.i2993 = fmul float %history.i.i1007.sroa.19.07306, %_104.i.i.i.i1364, !dbg !37580
  %_0.i2565 = fadd float %_0.i2569, %_0.i2993, !dbg !37582
  %_0.i2992 = fmul float %history.i.i1007.sroa.22.07307, %_109.i.i.i.i1369, !dbg !37584
  %_0.i2564 = fadd float %_0.i2568, %_0.i2992, !dbg !37586
  %_0.i2991 = fmul float %history.i.i1007.sroa.22.07307, %_112.i.i.i.i1372, !dbg !37588
  %_0.i2563 = fadd float %_0.i2567, %_0.i2991, !dbg !37590
  %_0.i2990 = fmul float %history.i.i1007.sroa.22.07307, %_115.i.i.i.i1375, !dbg !37592
  %_0.i2562 = fadd float %_0.i2566, %_0.i2990, !dbg !37594
  %_0.i2989 = fmul float %history.i.i1007.sroa.22.07307, %_118.i.i.i.i1378, !dbg !37596
  %_0.i2561 = fadd float %_0.i2565, %_0.i2989, !dbg !37598
  %_0.i2988 = fmul float %history.i.i1007.sroa.26.07308, %_123.i.i.i.i1383, !dbg !37600
  %_0.i2560 = fadd float %_0.i2564, %_0.i2988, !dbg !37602
  %_0.i2987 = fmul float %history.i.i1007.sroa.26.07308, %_126.i.i.i.i1386, !dbg !37604
  %_0.i2559 = fadd float %_0.i2563, %_0.i2987, !dbg !37606
  %_0.i2986 = fmul float %history.i.i1007.sroa.26.07308, %_129.i.i.i.i1389, !dbg !37608
  %_0.i2558 = fadd float %_0.i2562, %_0.i2986, !dbg !37610
  %_0.i2985 = fmul float %history.i.i1007.sroa.26.07308, %_132.i.i.i.i1392, !dbg !37612
  %_0.i2557 = fadd float %_0.i2561, %_0.i2985, !dbg !37614
  %_0.i2984 = fmul float %history.i.i1007.sroa.29.07309, %_137.i.i.i.i1397, !dbg !37616
  %_0.i2556 = fadd float %_0.i2560, %_0.i2984, !dbg !37618
  %_0.i2983 = fmul float %history.i.i1007.sroa.29.07309, %_140.i.i.i.i1400, !dbg !37620
  %_0.i2555 = fadd float %_0.i2559, %_0.i2983, !dbg !37622
  %_0.i2982 = fmul float %history.i.i1007.sroa.29.07309, %_143.i.i.i.i1403, !dbg !37624
  %_0.i2554 = fadd float %_0.i2558, %_0.i2982, !dbg !37626
  %_0.i2981 = fmul float %history.i.i1007.sroa.29.07309, %_146.i.i.i.i1406, !dbg !37628
  %_0.i2553 = fadd float %_0.i2557, %_0.i2981, !dbg !37630
  %_0.i2980 = fmul float %history.i.i1007.sroa.32.07310, %_151.i.i.i.i1411, !dbg !37632
  %_0.i2552 = fadd float %_0.i2556, %_0.i2980, !dbg !37634
  %_0.i2979 = fmul float %history.i.i1007.sroa.32.07310, %_154.i.i.i.i1414, !dbg !37636
  %_0.i2551 = fadd float %_0.i2555, %_0.i2979, !dbg !37638
  %_0.i2978 = fmul float %history.i.i1007.sroa.32.07310, %_157.i.i.i.i1417, !dbg !37640
  %_0.i2550 = fadd float %_0.i2554, %_0.i2978, !dbg !37642
  %_0.i2977 = fmul float %history.i.i1007.sroa.32.07310, %_160.i.i.i.i1420, !dbg !37644
  %_0.i2549 = fadd float %_0.i2553, %_0.i2977, !dbg !37646
  %_0.i2976 = fmul float %history.i.i1007.sroa.35.07311, %_165.i.i.i.i1425, !dbg !37648
  %_0.i2548 = fadd float %_0.i2552, %_0.i2976, !dbg !37650
  %_0.i2975 = fmul float %history.i.i1007.sroa.35.07311, %_168.i.i.i.i1428, !dbg !37652
  %_0.i2547 = fadd float %_0.i2551, %_0.i2975, !dbg !37654
  %_0.i2974 = fmul float %history.i.i1007.sroa.35.07311, %_171.i.i.i.i1431, !dbg !37656
  %_0.i2546 = fadd float %_0.i2550, %_0.i2974, !dbg !37658
  %_0.i2973 = fmul float %history.i.i1007.sroa.35.07311, %_174.i.i.i.i1434, !dbg !37660
  %_0.i2545 = fadd float %_0.i2549, %_0.i2973, !dbg !37662
  %177 = tail call noundef float @llvm.fabs.f32(float %_0.i2548), !dbg !37664
  %_3.i.i4127.inv = fcmp ogt float %176, %177, !dbg !37666
  %_4.i.i4134.v = select i1 %_3.i.i4127.inv, float %176, float %177, !dbg !37666
  %178 = tail call noundef float @llvm.fabs.f32(float %_0.i2547), !dbg !37664
  %_3.i.i4127.inv.1 = fcmp ogt float %_4.i.i4134.v, %178, !dbg !37666
  %_4.i.i4134.v.1 = select i1 %_3.i.i4127.inv.1, float %_4.i.i4134.v, float %178, !dbg !37666
  %179 = tail call noundef float @llvm.fabs.f32(float %_0.i2546), !dbg !37664
  %_3.i.i4127.inv.2 = fcmp ogt float %_4.i.i4134.v.1, %179, !dbg !37666
  %_4.i.i4134.v.2 = select i1 %_3.i.i4127.inv.2, float %_4.i.i4134.v.1, float %179, !dbg !37666
  %180 = tail call noundef float @llvm.fabs.f32(float %_0.i2545), !dbg !37664
  %_3.i.i4127.inv.3 = fcmp ogt float %_4.i.i4134.v.2, %180, !dbg !37666
  %_4.i.i4134.v.3 = select i1 %_3.i.i4127.inv.3, float %_4.i.i4134.v.2, float %180, !dbg !37666
  %181 = add nuw nsw i64 %iter.i126.i1003.sroa.16.07312, 1, !dbg !37669
  %data.i4.i4602 = getelementptr inbounds nuw float, ptr %peaks_right.i1013, i64 %iter.i126.i1003.sroa.16.07312, !dbg !37670
  store float %_4.i.i4134.v.3, ptr %data.i4.i4602, align 4, !dbg !37673, !alias.scope !37675, !noalias !37467
  %exitcond11745.not = icmp eq i64 %181, %umax11759, !dbg !37446
  br i1 %exitcond11745.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i1445, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502, !dbg !37446

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i1445: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4588
  %history.i.i1007.sroa.0.0.lcssa = phi float [ %history.i.i1007.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4588 ], [ %_0.i3500, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502 ], !dbg !37678
  %history.i.i1007.sroa.7.0.lcssa = phi float [ %history.i.i1007.sroa.7.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4588 ], [ %history.i.i1007.sroa.0.07301, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502 ], !dbg !37678
  %history.i.i1007.sroa.10.0.lcssa = phi float [ %history.i.i1007.sroa.10.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4588 ], [ %history.i.i1007.sroa.7.07302, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502 ], !dbg !37678
  %history.i.i1007.sroa.13.0.lcssa = phi float [ %history.i.i1007.sroa.13.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4588 ], [ %history.i.i1007.sroa.10.07303, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502 ], !dbg !37678
  %history.i.i1007.sroa.16.0.lcssa = phi float [ %history.i.i1007.sroa.16.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4588 ], [ %history.i.i1007.sroa.13.07304, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502 ], !dbg !37678
  %history.i.i1007.sroa.19.0.lcssa = phi float [ %history.i.i1007.sroa.19.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4588 ], [ %history.i.i1007.sroa.16.07305, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502 ], !dbg !37678
  %history.i.i1007.sroa.22.0.lcssa = phi float [ %history.i.i1007.sroa.22.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4588 ], [ %history.i.i1007.sroa.19.07306, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502 ], !dbg !37678
  %history.i.i1007.sroa.26.0.lcssa = phi float [ %history.i.i1007.sroa.26.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4588 ], [ %history.i.i1007.sroa.22.07307, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502 ], !dbg !37678
  %history.i.i1007.sroa.29.0.lcssa = phi float [ %history.i.i1007.sroa.29.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4588 ], [ %history.i.i1007.sroa.26.07308, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502 ], !dbg !37678
  %history.i.i1007.sroa.32.0.lcssa = phi float [ %history.i.i1007.sroa.32.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4588 ], [ %history.i.i1007.sroa.29.07309, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502 ], !dbg !37678
  %history.i.i1007.sroa.35.0.lcssa = phi float [ %history.i.i1007.sroa.35.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4588 ], [ %history.i.i1007.sroa.32.07310, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502 ], !dbg !37678
  %history.i.i1007.sroa.38.0.lcssa = phi float [ %history.i.i1007.sroa.38.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4588 ], [ %history.i.i1007.sroa.35.07311, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3502 ], !dbg !37678
  store float %history.i.i1007.sroa.0.0.lcssa, ptr %hot_right.i1016, align 4, !dbg !37679, !noalias !37443
  store float %history.i.i1007.sroa.7.0.lcssa, ptr %history.i.i1007.sroa.7.0.hot_right.i1016.sroa_idx, align 4, !dbg !37679, !noalias !37443
  store float %history.i.i1007.sroa.10.0.lcssa, ptr %history.i.i1007.sroa.10.0.hot_right.i1016.sroa_idx, align 4, !dbg !37679, !noalias !37443
  store float %history.i.i1007.sroa.13.0.lcssa, ptr %history.i.i1007.sroa.13.0.hot_right.i1016.sroa_idx, align 4, !dbg !37679, !noalias !37443
  store float %history.i.i1007.sroa.16.0.lcssa, ptr %history.i.i1007.sroa.16.0.hot_right.i1016.sroa_idx, align 4, !dbg !37679, !noalias !37443
  store float %history.i.i1007.sroa.19.0.lcssa, ptr %history.i.i1007.sroa.19.0.hot_right.i1016.sroa_idx, align 4, !dbg !37679, !noalias !37443
  store float %history.i.i1007.sroa.22.0.lcssa, ptr %history.i.i1007.sroa.22.0.hot_right.i1016.sroa_idx, align 4, !dbg !37679, !noalias !37443
  store float %history.i.i1007.sroa.26.0.lcssa, ptr %history.i.i1007.sroa.26.0.hot_right.i1016.sroa_idx, align 4, !dbg !37679, !noalias !37443
  store float %history.i.i1007.sroa.29.0.lcssa, ptr %history.i.i1007.sroa.29.0.hot_right.i1016.sroa_idx, align 4, !dbg !37679, !noalias !37443
  store float %history.i.i1007.sroa.32.0.lcssa, ptr %history.i.i1007.sroa.32.0.hot_right.i1016.sroa_idx, align 4, !dbg !37679, !noalias !37443
  store float %history.i.i1007.sroa.35.0.lcssa, ptr %history.i.i1007.sroa.35.0.hot_right.i1016.sroa_idx, align 4, !dbg !37679, !noalias !37443
  store float %history.i.i1007.sroa.38.0.lcssa, ptr %history.i.i1007.sroa.38.0.hot_right.i1016.sroa_idx, align 4, !dbg !37679, !noalias !37443
  br i1 %_2.i7274.not, label %bb13.i1034.loopexit, label %bb48.i1451.lr.ph, !dbg !37107

bb48.i1451.lr.ph:                                 ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i1445
  %_13.i243756575659 = load float, ptr %125, align 4, !alias.scope !37072, !noalias !37075, !noundef !12
  %_13.i242556605662 = load float, ptr %128, align 4, !alias.scope !37083, !noalias !37086, !noundef !12
  %_13.i241356635665 = load float, ptr %131, align 4, !alias.scope !37092, !noalias !37095, !noundef !12
  %_13.i240256665668 = load float, ptr %134, align 4, !alias.scope !37102, !noalias !37086, !noundef !12
  %_91.i1472 = load i64, ptr %135, align 8
  %_64.i89.i1529 = load float, ptr %146, align 4
  %_64.i.i1622 = load float, ptr %162, align 4
  %_106.i1654 = load i64, ptr %166, align 8
  %.promoted = load float, ptr %123, align 4, !alias.scope !37072, !noalias !37075
  %_68.i1453.promoted = load float, ptr %_68.i1453, align 4, !alias.scope !37072, !noalias !37075
  %.promoted7490 = load float, ptr %124, align 4, !alias.scope !37072, !noalias !37075
  %.promoted7560 = load float, ptr %126, align 4, !alias.scope !37083, !noalias !37086
  %_69.i1454.promoted = load float, ptr %_69.i1454, align 4, !alias.scope !37083, !noalias !37086
  %.promoted7698 = load float, ptr %127, align 4, !alias.scope !37083, !noalias !37086
  %.promoted7768 = load float, ptr %129, align 4, !alias.scope !37092, !noalias !37095
  %_73.i1455.promoted = load float, ptr %_73.i1455, align 4, !alias.scope !37092, !noalias !37095
  %.promoted7906 = load float, ptr %130, align 4, !alias.scope !37092, !noalias !37095
  %.promoted7976 = load float, ptr %132, align 4, !alias.scope !37102, !noalias !37086
  %_74.i1456.promoted = load float, ptr %_74.i1456, align 4, !alias.scope !37102, !noalias !37086
  %.promoted8114 = load float, ptr %133, align 4, !alias.scope !37102, !noalias !37086
  %.promoted8184 = load float, ptr %145, align 4
  %.promoted8186 = load float, ptr %147, align 4
  %.promoted8188 = load float, ptr %161, align 4
  %.promoted8190 = load float, ptr %163, align 4
  br label %bb48.i1451, !dbg !37107

bb48.i1451:                                       ; preds = %bb48.i1451.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit
  %_0.i37608191 = phi float [ %.promoted8190, %bb48.i1451.lr.ph ], [ %_0.i3760, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ]
  %_0.i33868189 = phi float [ %.promoted8188, %bb48.i1451.lr.ph ], [ %_0.i3386, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ]
  %_0.i37648187 = phi float [ %.promoted8186, %bb48.i1451.lr.ph ], [ %_0.i3764, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ]
  %_0.i33908185 = phi float [ %.promoted8184, %bb48.i1451.lr.ph ], [ %_0.i3390, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ]
  %_12.i24008115 = phi float [ %.promoted8114, %bb48.i1451.lr.ph ], [ %_0.i3883, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !37680
  %_0.i38908046 = phi float [ %_74.i1456.promoted, %bb48.i1451.lr.ph ], [ %_0.i3890, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !37680
  %_5.i23977977 = phi float [ %.promoted7976, %bb48.i1451.lr.ph ], [ %_0.i.i4100, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !37680
  %_12.i24117907 = phi float [ %.promoted7906, %bb48.i1451.lr.ph ], [ %_0.i3870, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !37680
  %_0.i38777838 = phi float [ %_73.i1455.promoted, %bb48.i1451.lr.ph ], [ %_0.i3877, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !37680
  %_5.i24057769 = phi float [ %.promoted7768, %bb48.i1451.lr.ph ], [ %_0.i.i4093, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !37680
  %_12.i24237699 = phi float [ %.promoted7698, %bb48.i1451.lr.ph ], [ %_0.i3857, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !37680
  %_0.i38647630 = phi float [ %_69.i1454.promoted, %bb48.i1451.lr.ph ], [ %_0.i3864, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !37680
  %_5.i24177561 = phi float [ %.promoted7560, %bb48.i1451.lr.ph ], [ %_0.i.i4086, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !37680
  %_12.i24357491 = phi float [ %.promoted7490, %bb48.i1451.lr.ph ], [ %_0.i3844, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !37680
  %_0.i38517422 = phi float [ %_68.i1453.promoted, %bb48.i1451.lr.ph ], [ %_0.i3851, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !37680
  %_5.i24297353 = phi float [ %.promoted, %bb48.i1451.lr.ph ], [ %_0.i.i4079, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !37680
  %main_cursor.sroa.0.1.i14497350 = phi i64 [ %main_cursor.sroa.0.0.i10388196, %bb48.i1451.lr.ph ], [ %spec.store.select.i1656, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ]
  %ring_cursor.sroa.0.1.i14487349 = phi i64 [ %ring_cursor.sroa.0.0.i10378195, %bb48.i1451.lr.ph ], [ %spec.store.select13.i1658, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ]
  %iter1.sroa.0.0.i14477348 = phi i64 [ 0, %bb48.i1451.lr.ph ], [ %182, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ]
  %182 = add nuw nsw i64 %iter1.sroa.0.0.i14477348, 1, !dbg !37680
  %_64.i1452 = add nuw nsw i64 %iter1.sroa.0.0.i14477348, %iter.sroa.0.0.i10358193, !dbg !37686
  %_0.i3379 = fadd float %_5.i24297353, -1.000000e+00, !dbg !37687
  %_3.i.i4073 = fcmp ogt float %_0.i3379, 0.000000e+00, !dbg !37690
  %_0.i.i4079 = select i1 %_3.i.i4073, float %_0.i3379, float 0.000000e+00, !dbg !37694
  %_0.i2539 = fadd float %_0.i38517422, %_12.i24357491, !dbg !37696
  %_0.i3851 = select i1 %_3.i.i4073, float %_0.i2539, float %_13.i243756575659, !dbg !37698
  %_0.i3844 = select i1 %_3.i.i4073, float %_12.i24357491, float 0.000000e+00, !dbg !37700
  %_0.i3380 = fadd float %_5.i24177561, -1.000000e+00, !dbg !37702
  %_3.i.i4080 = fcmp ogt float %_0.i3380, 0.000000e+00, !dbg !37704
  %_0.i.i4086 = select i1 %_3.i.i4080, float %_0.i3380, float 0.000000e+00, !dbg !37707
  %_0.i2540 = fadd float %_0.i38647630, %_12.i24237699, !dbg !37709
  %_0.i3864 = select i1 %_3.i.i4080, float %_0.i2540, float %_13.i242556605662, !dbg !37711
  %_0.i3857 = select i1 %_3.i.i4080, float %_12.i24237699, float 0.000000e+00, !dbg !37713
  %_0.i3381 = fadd float %_5.i24057769, -1.000000e+00, !dbg !37715
  %_3.i.i4087 = fcmp ogt float %_0.i3381, 0.000000e+00, !dbg !37717
  %_0.i.i4093 = select i1 %_3.i.i4087, float %_0.i3381, float 0.000000e+00, !dbg !37720
  %_0.i2541 = fadd float %_0.i38777838, %_12.i24117907, !dbg !37722
  %_0.i3877 = select i1 %_3.i.i4087, float %_0.i2541, float %_13.i241356635665, !dbg !37724
  %_0.i3870 = select i1 %_3.i.i4087, float %_12.i24117907, float 0.000000e+00, !dbg !37726
  %_0.i3382 = fadd float %_5.i23977977, -1.000000e+00, !dbg !37728
  %_3.i.i4094 = fcmp ogt float %_0.i3382, 0.000000e+00, !dbg !37730
  %_0.i.i4100 = select i1 %_3.i.i4094, float %_0.i3382, float 0.000000e+00, !dbg !37733
  %_0.i2542 = fadd float %_0.i38908046, %_12.i24008115, !dbg !37735
  %_0.i3890 = select i1 %_3.i.i4094, float %_0.i2542, float %_13.i240256665668, !dbg !37737
  %_0.i3883 = select i1 %_3.i.i4094, float %_12.i24008115, float 0.000000e+00, !dbg !37739
  %_162.i1460 = getelementptr inbounds nuw float, ptr %peaks_left.i1014, i64 %iter1.sroa.0.0.i14477348, !dbg !37741
  %_0.i3495 = load float, ptr %_162.i1460, align 4, !dbg !37752, !alias.scope !37754, !noalias !37086, !noundef !12
  %_167.i1462 = getelementptr inbounds nuw float, ptr %peaks_right.i1013, i64 %iter1.sroa.0.0.i14477348, !dbg !37757
  %_0.i3490 = load float, ptr %_167.i1462, align 4, !dbg !37767, !alias.scope !37769, !noalias !37086, !noundef !12
  %_3.i.i4118 = fcmp ule float %_0.i3490, %_0.i3495, !dbg !37772
  %_6.i.i4120 = bitcast float %_0.i3490 to i32, !dbg !37775
  %_8.i.i4122 = bitcast float %_0.i3495 to i32, !dbg !37779
  %_4.i.i4125 = select i1 %_3.i.i4118, i32 %_8.i.i4122, i32 %_6.i.i4120, !dbg !37781
  %_5.i3924 = and i32 %_4.i.i4125, %.none.i1022, !dbg !37782
  %_7.i3927 = and i32 %_9.i3926, %_8.i.i4122, !dbg !37784
  %_4.i3928 = or disjoint i32 %_5.i3924, %_7.i3927, !dbg !37782
  %_0.i3929 = bitcast i32 %_4.i3928 to float, !dbg !37785
  %_7.i3920 = and i32 %_9.i3926, %_6.i.i4120, !dbg !37788
  %_4.i3921 = or disjoint i32 %_5.i3924, %_7.i3920, !dbg !37790
  %_0.i3922 = bitcast i32 %_4.i3921 to float, !dbg !37791
  %_168.i1467 = icmp ugt i64 %_64.i1452, %left_io.1, !dbg !37793
  br i1 %_168.i1467, label %bb52.i1670, label %bb53.i1468, !dbg !37793, !prof !1406

bb53.i1468:                                       ; preds = %bb48.i1451
  %_174.i1470 = getelementptr inbounds nuw float, ptr %left_io.0, i64 %_64.i1452, !dbg !37797
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37802), !dbg !37805
  %_3.not.i3483 = icmp eq i64 %left_io.1, %_64.i1452, !dbg !37806
  br i1 %_3.not.i3483, label %panic.i3486, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3487, !dbg !37806

panic.i3486:                                      ; preds = %bb53.i1468
  store float %_0.i33908185, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !37806, !noalias !37808
  unreachable, !dbg !37806

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3487: ; preds = %bb53.i1468
  %_0.i3485 = load float, ptr %_174.i1470, align 4, !dbg !37806, !alias.scope !37802, !noalias !37086, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37809), !dbg !37812
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37813), !dbg !37812
  %width.i33.i1473 = load i64, ptr %136, align 8, !dbg !37815, !alias.scope !37816, !noalias !37817, !noundef !12
  %_3.i2505 = fcmp uge float %_0.i3851, %_0.i3929, !dbg !37820
  %_0.i2938 = fdiv float %_0.i3851, %_0.i3929, !dbg !37822
  %_0.i3915 = select i1 %_3.i2505, float 1.000000e+00, float %_0.i2938, !dbg !37825
  %_144.1.i38.i1478 = load i64, ptr %137, align 8, !dbg !37827, !alias.scope !37816, !noalias !37817, !noundef !12
  %_22.i39.i1479 = mul i64 %width.i33.i1473, %ring_cursor.sroa.0.1.i14487349, !dbg !37828
  %_92.i40.i1480 = icmp ugt i64 %_22.i39.i1479, %_144.1.i38.i1478, !dbg !37829
  br i1 %_92.i40.i1480, label %bb37.i124.i1669, label %bb38.i41.i1481, !dbg !37829, !prof !1406

bb38.i41.i1481:                                   ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3487
  %_144.0.i42.i1482 = load ptr, ptr %138, align 8, !dbg !37827, !alias.scope !37816, !noalias !37817, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37834), !dbg !37837
  %_4.not.i3646 = icmp eq i64 %_144.1.i38.i1478, %_22.i39.i1479, !dbg !37838
  br i1 %_4.not.i3646, label %panic.i3648, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3649, !dbg !37838

panic.i3648:                                      ; preds = %bb38.i41.i1481
  store float %_0.i33908185, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !37838, !noalias !37840
  unreachable, !dbg !37838

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3649: ; preds = %bb38.i41.i1481
  %_99.i44.i1484 = getelementptr inbounds nuw float, ptr %_144.0.i42.i1482, i64 %_22.i39.i1479, !dbg !37841
  store float %_0.i3915, ptr %_99.i44.i1484, align 4, !dbg !37838, !alias.scope !37834, !noalias !37846
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37847), !dbg !37850
  %width.i2007 = load i64, ptr %136, align 8, !dbg !37851, !alias.scope !37847, !noalias !37853, !noundef !12
  %183 = icmp eq i64 %width.i2007, 0, !dbg !37855
  br i1 %183, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2115, label %bb32.i2014.lr.ph, !dbg !37855

bb32.i2014.lr.ph:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3649
  %_112.1.i2017 = load i64, ptr %62, align 8, !alias.scope !37847, !noalias !37853, !noundef !12
  %_112.0.i2021 = load ptr, ptr %61, align 8, !nonnull !12
  %184 = add i64 %ring_cursor.sroa.0.1.i14487349, 1
  %_23.not.i2028 = icmp ult i64 %184, %_91.i1472
  %185 = select i1 %_23.not.i2028, i64 0, i64 %_91.i1472
  %start1.sroa.0.0.i2029 = sub nuw i64 %184, %185
  %_114.1.i2032 = load i64, ptr %137, align 8
  %_114.0.i2036 = load ptr, ptr %138, align 8, !nonnull !12
  %_116.1.i2037 = load i64, ptr %139, align 8
  %_116.0.i2041 = load ptr, ptr %140, align 8, !nonnull !12
  %_118.1.i2045 = load i64, ptr %141, align 8
  %_118.0.i2049 = load ptr, ptr %142, align 8, !nonnull !12
  %_45.i2062 = mul i64 %width.i2007, %start1.sroa.0.0.i2029
  br label %bb32.i2014, !dbg !37855

bb32.i2014:                                       ; preds = %bb32.i2014.lr.ph, %bb31.i2077
  %iter.i2006.sroa.10.07330 = phi i64 [ %width.i2007, %bb32.i2014.lr.ph ], [ %186, %bb31.i2077 ]
  %iter.i2006.sroa.7.07329 = phi i64 [ 0, %bb32.i2014.lr.ph ], [ %_9.0.i, %bb31.i2077 ]
  %iter.i2006.sroa.0.0.idx7328 = phi i64 [ 0, %bb32.i2014.lr.ph ], [ %iter.i2006.sroa.0.0.add, %bb31.i2077 ]
  %iter.i2006.sroa.0.0.ptr7331 = getelementptr inbounds nuw i8, ptr %scratch.i1015, i64 %iter.i2006.sroa.0.0.idx7328, !dbg !37857
  %186 = add i64 %iter.i2006.sroa.10.07330, -1, !dbg !37857
  %_7.i.i4619 = icmp eq i64 %iter.i2006.sroa.0.0.idx7328, 32, !dbg !37858
  br i1 %_7.i.i4619, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2115, label %bb3.i2016, !dbg !37862

bb3.i2016:                                        ; preds = %bb32.i2014
  %iter.i2006.sroa.0.0.add = add nuw nsw i64 %iter.i2006.sroa.0.0.idx7328, 4, !dbg !37863
  %_9.0.i = add nuw nsw i64 %iter.i2006.sroa.7.07329, 1, !dbg !37865
  %exitcond11748.not = icmp eq i64 %iter.i2006.sroa.7.07329, %_112.1.i2017, !dbg !37866
  br i1 %exitcond11748.not, label %panic.i2019, label %bb5.i2020, !dbg !37866

bb5.i2020:                                        ; preds = %bb3.i2016
  %187 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i2021, i64 %iter.i2006.sroa.7.07329, !dbg !37866
  %shape.i2022 = load i32, ptr %187, align 4, !dbg !37866, !noalias !37867, !noundef !12
  %188 = getelementptr inbounds nuw i8, ptr %187, i64 4, !dbg !37866
  %shape3.i2023 = load i32, ptr %188, align 4, !dbg !37866, !noalias !37867, !noundef !12
  %window.i2024 = zext i32 %shape.i2022 to i64, !dbg !37868
  %_19.i2025 = zext i32 %shape3.i2023 to i64, !dbg !37869
  %189 = add i64 %ring_cursor.sroa.0.1.i14487349, %_19.i2025, !dbg !37870
  %_20.not.i2026 = icmp ult i64 %189, %_91.i1472, !dbg !37871
  %190 = select i1 %_20.not.i2026, i64 0, i64 %_91.i1472, !dbg !37871
  %spec.select.i2027 = sub nuw i64 %189, %190, !dbg !37871
  %_27.i2030 = mul i64 %spec.select.i2027, %width.i2007, !dbg !37872
  %_26.i2031 = add i64 %_27.i2030, %iter.i2006.sroa.7.07329, !dbg !37872
  %_30.i2033 = icmp ult i64 %_26.i2031, %_114.1.i2032, !dbg !37873
  br i1 %_30.i2033, label %bb12.i2035, label %panic5.i2034, !dbg !37873

panic.i2019:                                      ; preds = %bb3.i2016
  store float %_0.i33908185, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i2017, i64 noundef %_112.1.i2017, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4a8785a681d008a9bfd0cd82628ea9cb) #30, !dbg !37866, !noalias !37867
  unreachable, !dbg !37866

bb12.i2035:                                       ; preds = %bb5.i2020
  %191 = getelementptr inbounds nuw float, ptr %_114.0.i2036, i64 %_26.i2031, !dbg !37873
  %192 = load float, ptr %191, align 4, !dbg !37873, !noalias !37867, !noundef !12
  %exitcond11749.not = icmp eq i64 %iter.i2006.sroa.7.07329, %_116.1.i2037, !dbg !37874
  br i1 %exitcond11749.not, label %panic6.i2039, label %bb13.i2040, !dbg !37874

panic5.i2034:                                     ; preds = %bb5.i2020
  store float %_0.i33908185, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i2031, i64 noundef %_114.1.i2032, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cbce7773ac40979e4ba2385da3aec116) #30, !dbg !37873, !noalias !37867
  unreachable, !dbg !37873

bb13.i2040:                                       ; preds = %bb12.i2035
  %193 = getelementptr inbounds nuw i32, ptr %_116.0.i2041, i64 %iter.i2006.sroa.7.07329, !dbg !37874
  %_32.i2042 = load i32, ptr %193, align 4, !dbg !37874, !noalias !37867, !noundef !12
  %position.i2043 = zext i32 %_32.i2042 to i64, !dbg !37874
  %194 = icmp eq i32 %_32.i2042, 0, !dbg !37875
  br i1 %194, label %bb17.i2052, label %bb15.i2044, !dbg !37875

panic6.i2039:                                     ; preds = %bb12.i2035
  store float %_0.i33908185, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i2037, i64 noundef %_116.1.i2037, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ec0d48f73ebfc2755df5cedaa60b5c0a) #30, !dbg !37874, !noalias !37867
  unreachable, !dbg !37874

bb15.i2044:                                       ; preds = %bb13.i2040
  %_37.i2046 = icmp ult i64 %iter.i2006.sroa.7.07329, %_118.1.i2045, !dbg !37876
  br i1 %_37.i2046, label %bb16.i2048, label %panic7.i2047, !dbg !37876

bb17.i2052:                                       ; preds = %bb35.i2113, %bb16.i2048, %bb13.i2040
  %newest.sroa.0.0.i2053 = phi float [ %192, %bb13.i2040 ], [ %_35.i2050, %bb35.i2113 ], [ %192, %bb16.i2048 ], !dbg !37877
  %exitcond11750.not = icmp eq i64 %iter.i2006.sroa.7.07329, %_118.1.i2045, !dbg !37878
  br i1 %exitcond11750.not, label %panic8.i2056, label %bb18.i2057, !dbg !37878

bb16.i2048:                                       ; preds = %bb15.i2044
  %195 = getelementptr inbounds nuw float, ptr %_118.0.i2049, i64 %iter.i2006.sroa.7.07329, !dbg !37876
  %_35.i2050 = load float, ptr %195, align 4, !dbg !37876, !noalias !37867, !noundef !12
  %_102.i2051 = fcmp olt float %_35.i2050, %192, !dbg !37879
  br i1 %_102.i2051, label %bb35.i2113, label %bb17.i2052, !dbg !37879

panic7.i2047:                                     ; preds = %bb15.i2044
  store float %_0.i33908185, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i2006.sroa.7.07329, i64 noundef %_118.1.i2045, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2c461872bb652d4796cdcf89c28c82c8) #30, !dbg !37876, !noalias !37867
  unreachable, !dbg !37876

bb35.i2113:                                       ; preds = %bb16.i2048
  br label %bb17.i2052, !dbg !37881

bb18.i2057:                                       ; preds = %bb17.i2052
  %196 = getelementptr inbounds nuw float, ptr %_118.0.i2049, i64 %iter.i2006.sroa.7.07329, !dbg !37878
  store float %newest.sroa.0.0.i2053, ptr %196, align 4, !dbg !37878, !noalias !37867
  %_42.i2059 = add nuw nsw i64 %position.i2043, 1, !dbg !37882
  %complete.i2060 = icmp eq i64 %_42.i2059, %window.i2024, !dbg !37882
  br i1 %complete.i2060, label %bb22.i2082, label %bb20.i2061, !dbg !37883

panic8.i2056:                                     ; preds = %bb17.i2052
  store float %_0.i33908185, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i2045, i64 noundef %_118.1.i2045, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2b690e2c7763f11809942906fc2ca813) #30, !dbg !37878, !noalias !37867
  unreachable, !dbg !37878

bb20.i2061:                                       ; preds = %bb18.i2057
  %_44.i2063 = add i64 %iter.i2006.sroa.7.07329, %_45.i2062, !dbg !37884
  %_47.i2065 = icmp ult i64 %_44.i2063, %_114.1.i2032, !dbg !37885
  br i1 %_47.i2065, label %bb30.i2075, label %panic9.i2066, !dbg !37885

panic9.i2066:                                     ; preds = %bb20.i2061
  store float %_0.i33908185, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i2063, i64 noundef %_114.1.i2032, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9fa421ae81817f58fcfc4a3243223891) #30, !dbg !37885, !noalias !37867
  unreachable, !dbg !37885

bb30.i2075:                                       ; preds = %bb20.i2061
  %197 = getelementptr inbounds nuw float, ptr %_114.0.i2036, i64 %_44.i2063, !dbg !37885
  %_43.i2069 = load float, ptr %197, align 4, !dbg !37885, !noalias !37867, !noundef !12
  %_103.i2070 = fcmp olt float %_43.i2069, %newest.sroa.0.0.i2053, !dbg !37886
  %newest.sroa.0.1.i2071 = select i1 %_103.i2070, float %_43.i2069, float %newest.sroa.0.0.i2053, !dbg !37886
  store float %newest.sroa.0.1.i2071, ptr %iter.i2006.sroa.0.0.ptr7331, align 4, !dbg !37888, !noalias !37867
  %198 = trunc i64 %_42.i2059 to i32, !dbg !37889
  br label %bb31.i2077, !dbg !37890

bb31.i2077:                                       ; preds = %bb25.i2110, %bb30.i2075
  %storemerge = phi i32 [ %198, %bb30.i2075 ], [ 0, %bb25.i2110 ], !dbg !37891
  store i32 %storemerge, ptr %193, align 4, !dbg !37891, !noalias !37867
  %199 = icmp eq i64 %186, 0, !dbg !37855
  br i1 %199, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2115, label %bb32.i2014, !dbg !37855

bb22.i2082:                                       ; preds = %bb18.i2057
  store float %newest.sroa.0.0.i2053, ptr %iter.i2006.sroa.0.0.ptr7331, align 4, !dbg !37888, !noalias !37867
  %200 = load float, ptr %191, align 4, !dbg !37892, !noalias !37867, !noundef !12
  br label %bb41.i2095, !dbg !37893

bb41.i2095:                                       ; preds = %bb22.i2082, %bb25.i2110
  %iter2.sroa.0.0.i20877327 = phi i64 [ 0, %bb22.i2082 ], [ %_105.i2096, %bb25.i2110 ]
  %suffix.sroa.0.0.i20867326 = phi float [ %200, %bb22.i2082 ], [ %suffix.sroa.0.1.i2106, %bb25.i2110 ]
  %end.sroa.0.1.i20857325 = phi i64 [ %spec.select.i2027, %bb22.i2082 ], [ %203, %bb25.i2110 ]
  %_56.i2097 = mul i64 %end.sroa.0.1.i20857325, %width.i2007, !dbg !37896
  %_55.i2098 = add i64 %_56.i2097, %iter.i2006.sroa.7.07329, !dbg !37896
  %_59.i2100 = icmp ult i64 %_55.i2098, %_114.1.i2032, !dbg !37897
  br i1 %_59.i2100, label %bb25.i2110, label %panic13.i2101, !dbg !37897

panic13.i2101:                                    ; preds = %bb41.i2095
  store float %_0.i33908185, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i2098, i64 noundef %_114.1.i2032, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_91a4c6b9b17ebf4d863f9a70b6dc929a) #30, !dbg !37897, !noalias !37867
  unreachable, !dbg !37897

bb25.i2110:                                       ; preds = %bb41.i2095
  %_105.i2096 = add nuw nsw i64 %iter2.sroa.0.0.i20877327, 1, !dbg !37898
  %201 = getelementptr inbounds nuw float, ptr %_114.0.i2036, i64 %_55.i2098, !dbg !37897
  %_54.i2104 = load float, ptr %201, align 4, !dbg !37897, !noalias !37867, !noundef !12
  %_107.i2105 = fcmp olt float %suffix.sroa.0.0.i20867326, %_54.i2104, !dbg !37901
  %suffix.sroa.0.1.i2106 = select i1 %_107.i2105, float %suffix.sroa.0.0.i20867326, float %_54.i2104, !dbg !37901
  store float %suffix.sroa.0.1.i2106, ptr %201, align 4, !dbg !37903, !noalias !37867
  %202 = icmp eq i64 %end.sroa.0.1.i20857325, 0, !dbg !37904
  %spec.store.select.i2112 = select i1 %202, i64 %_91.i1472, i64 %end.sroa.0.1.i20857325, !dbg !37904
  %203 = add i64 %spec.store.select.i2112, -1, !dbg !37905
  %exitcond11747.not = icmp eq i64 %_105.i2096, %window.i2024, !dbg !37906
  br i1 %exitcond11747.not, label %bb31.i2077, label %bb41.i2095, !dbg !37893

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2115: ; preds = %bb31.i2077, %bb32.i2014, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3649
  %_0.i3482 = load float, ptr %scratch.i1015, align 4, !dbg !37908, !alias.scope !37910, !noalias !37913, !noundef !12
  %_0.i2972 = fmul float %_0.i3482, 1.638400e+04, !dbg !37914
  %204 = tail call noundef float @llvm.floor.f32(float %_0.i2972), !dbg !37916
  %_0.i2971 = fmul float %204, 0x3F10000000000000, !dbg !37923
  %205 = icmp eq i64 %width.i33.i1473, 0, !dbg !37925
  %_149.1.i82.i1522.pre = load i64, ptr %143, align 8, !dbg !37930, !alias.scope !37816, !noalias !37817
  br i1 %205, label %bb16.i77.i1517, label %bb39.i57.i1497.lr.ph, !dbg !37925

bb39.i57.i1497.lr.ph:                             ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2115
  %_145.1.i60.i1500 = load i64, ptr %62, align 8, !alias.scope !37816, !noalias !37817, !noundef !12
  %_145.0.i64.i1504 = load ptr, ptr %61, align 8, !nonnull !12
  %_147.0.i75.i1515 = load ptr, ptr %144, align 8, !nonnull !12
  %exitcond11751.not = icmp eq i64 %_145.1.i60.i1500, 0, !dbg !37931
  br i1 %exitcond11751.not, label %panic.i62.i1502, label %bb17.i63.i1503, !dbg !37931

bb37.i124.i1669:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3487
  store float %_0.i33908185, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i39.i1479, i64 noundef %_144.1.i38.i1478, i64 noundef %_144.1.i38.i1478, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_56df7c041d29359441bca272bf4e38e3) #30, !dbg !37933, !noalias !37846
  unreachable, !dbg !37933

bb16.i77.i1517:                                   ; preds = %bb21.i74.i1514.7, %bb21.i74.i1514, %bb21.i74.i1514.1, %bb21.i74.i1514.2, %bb21.i74.i1514.3, %bb21.i74.i1514.4, %bb21.i74.i1514.5, %bb21.i74.i1514.6, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2115
  %_0.i3480 = phi float [ %_0.i3482, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2115 ], [ %_49.i76.i1516, %bb21.i74.i1514 ], [ %_49.i76.i1516, %bb21.i74.i1514.7 ], [ %_49.i76.i1516, %bb21.i74.i1514.6 ], [ %_49.i76.i1516, %bb21.i74.i1514.5 ], [ %_49.i76.i1516, %bb21.i74.i1514.4 ], [ %_49.i76.i1516, %bb21.i74.i1514.3 ], [ %_49.i76.i1516, %bb21.i74.i1514.2 ], [ %_49.i76.i1516, %bb21.i74.i1514.1 ], !dbg !37934
  %_0.i2544 = fadd float %_0.i2971, %_0.i33908185, !dbg !37936
  %_0.i3390 = fsub float %_0.i2544, %_0.i3480, !dbg !37938
  %_109.i83.i1523 = icmp ugt i64 %_22.i39.i1479, %_149.1.i82.i1522.pre, !dbg !37940
  br i1 %_109.i83.i1523, label %bb42.i123.i1668, label %bb43.i84.i1524, !dbg !37940, !prof !1406

bb43.i84.i1524:                                   ; preds = %bb16.i77.i1517
  %_149.0.i85.i1525 = load ptr, ptr %144, align 8, !dbg !37930, !alias.scope !37816, !noalias !37817, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37944), !dbg !37947
  %_4.not.i3642 = icmp eq i64 %_149.1.i82.i1522.pre, %_22.i39.i1479, !dbg !37948
  br i1 %_4.not.i3642, label %panic.i3644, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3645, !dbg !37948

panic.i3644:                                      ; preds = %bb43.i84.i1524
  store float %_0.i3390, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !37948, !noalias !37950
  unreachable, !dbg !37948

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3645: ; preds = %bb43.i84.i1524
  %_116.i87.i1527 = getelementptr inbounds nuw float, ptr %_149.0.i85.i1525, i64 %_22.i39.i1479, !dbg !37951
  store float %_0.i2971, ptr %_116.i87.i1527, align 4, !dbg !37948, !alias.scope !37944, !noalias !37913
  %_0.i2937 = fdiv float %_0.i3390, %_64.i89.i1529, !dbg !37956
  %_0.i3389 = fsub float 1.000000e+00, %_0.i2937, !dbg !37958
  %_0.i3388 = fsub float %_0.i3389, %_0.i37648187, !dbg !37961
  %_4.i2953 = fmul float %_0.i3864, %_0.i3388, !dbg !37964
  %_0.i2954 = fadd float %_0.i37648187, %_4.i2953, !dbg !37964
  %_3.i.i4109.inv = fcmp ogt float %_0.i3389, %_0.i2954, !dbg !37967
  %_4.i.i4116.v = select i1 %_3.i.i4109.inv, float %_0.i3389, float %_0.i2954, !dbg !37967
  %206 = tail call noundef float @llvm.fabs.f32(float %_4.i.i4116.v), !dbg !37971
  %207 = fcmp uge float %206, 0x3BC79CA100000000, !dbg !37975
  %_0.i3764 = select i1 %207, float %_4.i.i4116.v, float 0.000000e+00, !dbg !37978
  store float %_0.i3764, ptr %147, align 4, !dbg !37979, !alias.scope !37809, !noalias !37980
  %_0.i3387 = fsub float 1.000000e+00, %_0.i3764, !dbg !37981
  %_150.1.i101.i1541 = load i64, ptr %148, align 8, !dbg !37983, !alias.scope !37816, !noalias !37817, !noundef !12
  %_76.i102.i1542 = mul i64 %width.i33.i1473, %main_cursor.sroa.0.1.i14497350, !dbg !37985
  %_120.i103.i1543 = icmp ugt i64 %_76.i102.i1542, %_150.1.i101.i1541, !dbg !37986
  br i1 %_120.i103.i1543, label %bb48.i122.i1667, label %bb49.i104.i1544, !dbg !37986, !prof !1406

bb42.i123.i1668:                                  ; preds = %bb16.i77.i1517
  store float %_0.i3390, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i39.i1479, i64 noundef %_149.1.i82.i1522.pre, i64 noundef %_149.1.i82.i1522.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_90498045d73339daaf9e4f537508f58b) #30, !dbg !37991, !noalias !37913
  unreachable, !dbg !37991

bb49.i104.i1544:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3645
  %_150.0.i105.i1545 = load ptr, ptr %149, align 8, !dbg !37983, !alias.scope !37816, !noalias !37817, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37992), !dbg !37995
  %_3.not.i3474 = icmp eq i64 %_150.1.i101.i1541, %_76.i102.i1542, !dbg !37996
  br i1 %_3.not.i3474, label %panic.i3477, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3637, !dbg !37996

panic.i3477:                                      ; preds = %bb49.i104.i1544
  store float %_0.i3390, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !37996, !noalias !37998
  unreachable, !dbg !37996

bb48.i122.i1667:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3645
  store float %_0.i3390, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i102.i1542, i64 noundef %_150.1.i101.i1541, i64 noundef %_150.1.i101.i1541, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_da8af254b6d507a8e2ca31e544bfd21d) #30, !dbg !37999, !noalias !37913
  unreachable, !dbg !37999

bb17.i63.i1503:                                   ; preds = %bb39.i57.i1497.lr.ph
  %208 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i1504, i64 8, !dbg !37931
  %_44.i65.i1505 = load i32, ptr %208, align 4, !dbg !37931, !noalias !37913, !noundef !12
  %_43.i66.i1506 = zext i32 %_44.i65.i1505 to i64, !dbg !37931
  %209 = add i64 %ring_cursor.sroa.0.1.i14487349, %_43.i66.i1506, !dbg !38000
  %_47.not.i67.i1507 = icmp ult i64 %209, %_91.i1472, !dbg !38001
  %210 = select i1 %_47.not.i67.i1507, i64 0, i64 %_91.i1472, !dbg !38001
  %spec.select.i68.i1508 = sub nuw i64 %209, %210, !dbg !38001
  %_51.i69.i1509 = mul i64 %spec.select.i68.i1508, %width.i33.i1473, !dbg !38003
  %_53.i72.i1512 = icmp ult i64 %_51.i69.i1509, %_149.1.i82.i1522.pre, !dbg !38004
  br i1 %_53.i72.i1512, label %bb21.i74.i1514, label %panic1.i73.i1513, !dbg !38004

panic.i62.i1502:                                  ; preds = %bb39.i57.i1497.7, %bb39.i57.i1497.6, %bb39.i57.i1497.5, %bb39.i57.i1497.4, %bb39.i57.i1497.3, %bb39.i57.i1497.2, %bb39.i57.i1497.1, %bb39.i57.i1497.lr.ph
  %_145.1.i60.i1500.lcssa.ph = phi i64 [ 7, %bb39.i57.i1497.7 ], [ 6, %bb39.i57.i1497.6 ], [ 5, %bb39.i57.i1497.5 ], [ 4, %bb39.i57.i1497.4 ], [ 3, %bb39.i57.i1497.3 ], [ 2, %bb39.i57.i1497.2 ], [ 1, %bb39.i57.i1497.1 ], [ 0, %bb39.i57.i1497.lr.ph ]
  store float %_0.i33908185, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i60.i1500.lcssa.ph, i64 noundef %_145.1.i60.i1500.lcssa.ph, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a6feb40b34112df5f214f84424dd2c7c) #30, !dbg !37931, !noalias !37913
  unreachable, !dbg !37931

bb21.i74.i1514:                                   ; preds = %bb17.i63.i1503
  %211 = getelementptr inbounds nuw float, ptr %_147.0.i75.i1515, i64 %_51.i69.i1509, !dbg !38004
  %_49.i76.i1516 = load float, ptr %211, align 4, !dbg !38004, !noalias !37913, !noundef !12
  store float %_49.i76.i1516, ptr %scratch.i1015, align 4, !dbg !38005, !noalias !37913
  %212 = icmp eq i64 %width.i33.i1473, 1, !dbg !37925
  br i1 %212, label %bb16.i77.i1517, label %bb39.i57.i1497.1, !dbg !37925

bb39.i57.i1497.1:                                 ; preds = %bb21.i74.i1514
  %exitcond11751.1.not = icmp eq i64 %_145.1.i60.i1500, 1, !dbg !37931
  br i1 %exitcond11751.1.not, label %panic.i62.i1502, label %bb17.i63.i1503.1, !dbg !37931

bb17.i63.i1503.1:                                 ; preds = %bb39.i57.i1497.1
  %213 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i1504, i64 20, !dbg !37931
  %_44.i65.i1505.1 = load i32, ptr %213, align 4, !dbg !37931, !noalias !37913, !noundef !12
  %_43.i66.i1506.1 = zext i32 %_44.i65.i1505.1 to i64, !dbg !37931
  %214 = add i64 %ring_cursor.sroa.0.1.i14487349, %_43.i66.i1506.1, !dbg !38000
  %_47.not.i67.i1507.1 = icmp ult i64 %214, %_91.i1472, !dbg !38001
  %215 = select i1 %_47.not.i67.i1507.1, i64 0, i64 %_91.i1472, !dbg !38001
  %spec.select.i68.i1508.1 = sub nuw i64 %214, %215, !dbg !38001
  %_51.i69.i1509.1 = mul i64 %spec.select.i68.i1508.1, %width.i33.i1473, !dbg !38003
  %_50.i70.i1510.1 = add i64 %_51.i69.i1509.1, 1, !dbg !38003
  %_53.i72.i1512.1 = icmp ult i64 %_50.i70.i1510.1, %_149.1.i82.i1522.pre, !dbg !38004
  br i1 %_53.i72.i1512.1, label %bb21.i74.i1514.1, label %panic1.i73.i1513, !dbg !38004

bb21.i74.i1514.1:                                 ; preds = %bb17.i63.i1503.1
  %216 = getelementptr inbounds nuw float, ptr %_147.0.i75.i1515, i64 %_50.i70.i1510.1, !dbg !38004
  %_49.i76.i1516.1 = load float, ptr %216, align 4, !dbg !38004, !noalias !37913, !noundef !12
  store float %_49.i76.i1516.1, ptr %iter.i32.i1008.sroa.0.0.ptr7335.1, align 4, !dbg !38005, !noalias !37913
  %217 = icmp eq i64 %width.i33.i1473, 2, !dbg !37925
  br i1 %217, label %bb16.i77.i1517, label %bb39.i57.i1497.2, !dbg !37925

bb39.i57.i1497.2:                                 ; preds = %bb21.i74.i1514.1
  %exitcond11751.2.not = icmp eq i64 %_145.1.i60.i1500, 2, !dbg !37931
  br i1 %exitcond11751.2.not, label %panic.i62.i1502, label %bb17.i63.i1503.2, !dbg !37931

bb17.i63.i1503.2:                                 ; preds = %bb39.i57.i1497.2
  %218 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i1504, i64 32, !dbg !37931
  %_44.i65.i1505.2 = load i32, ptr %218, align 4, !dbg !37931, !noalias !37913, !noundef !12
  %_43.i66.i1506.2 = zext i32 %_44.i65.i1505.2 to i64, !dbg !37931
  %219 = add i64 %ring_cursor.sroa.0.1.i14487349, %_43.i66.i1506.2, !dbg !38000
  %_47.not.i67.i1507.2 = icmp ult i64 %219, %_91.i1472, !dbg !38001
  %220 = select i1 %_47.not.i67.i1507.2, i64 0, i64 %_91.i1472, !dbg !38001
  %spec.select.i68.i1508.2 = sub nuw i64 %219, %220, !dbg !38001
  %_51.i69.i1509.2 = mul i64 %spec.select.i68.i1508.2, %width.i33.i1473, !dbg !38003
  %_50.i70.i1510.2 = add i64 %_51.i69.i1509.2, 2, !dbg !38003
  %_53.i72.i1512.2 = icmp ult i64 %_50.i70.i1510.2, %_149.1.i82.i1522.pre, !dbg !38004
  br i1 %_53.i72.i1512.2, label %bb21.i74.i1514.2, label %panic1.i73.i1513, !dbg !38004

bb21.i74.i1514.2:                                 ; preds = %bb17.i63.i1503.2
  %221 = getelementptr inbounds nuw float, ptr %_147.0.i75.i1515, i64 %_50.i70.i1510.2, !dbg !38004
  %_49.i76.i1516.2 = load float, ptr %221, align 4, !dbg !38004, !noalias !37913, !noundef !12
  store float %_49.i76.i1516.2, ptr %iter.i32.i1008.sroa.0.0.ptr7335.2, align 4, !dbg !38005, !noalias !37913
  %222 = icmp eq i64 %width.i33.i1473, 3, !dbg !37925
  br i1 %222, label %bb16.i77.i1517, label %bb39.i57.i1497.3, !dbg !37925

bb39.i57.i1497.3:                                 ; preds = %bb21.i74.i1514.2
  %exitcond11751.3.not = icmp eq i64 %_145.1.i60.i1500, 3, !dbg !37931
  br i1 %exitcond11751.3.not, label %panic.i62.i1502, label %bb17.i63.i1503.3, !dbg !37931

bb17.i63.i1503.3:                                 ; preds = %bb39.i57.i1497.3
  %223 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i1504, i64 44, !dbg !37931
  %_44.i65.i1505.3 = load i32, ptr %223, align 4, !dbg !37931, !noalias !37913, !noundef !12
  %_43.i66.i1506.3 = zext i32 %_44.i65.i1505.3 to i64, !dbg !37931
  %224 = add i64 %ring_cursor.sroa.0.1.i14487349, %_43.i66.i1506.3, !dbg !38000
  %_47.not.i67.i1507.3 = icmp ult i64 %224, %_91.i1472, !dbg !38001
  %225 = select i1 %_47.not.i67.i1507.3, i64 0, i64 %_91.i1472, !dbg !38001
  %spec.select.i68.i1508.3 = sub nuw i64 %224, %225, !dbg !38001
  %_51.i69.i1509.3 = mul i64 %spec.select.i68.i1508.3, %width.i33.i1473, !dbg !38003
  %_50.i70.i1510.3 = add i64 %_51.i69.i1509.3, 3, !dbg !38003
  %_53.i72.i1512.3 = icmp ult i64 %_50.i70.i1510.3, %_149.1.i82.i1522.pre, !dbg !38004
  br i1 %_53.i72.i1512.3, label %bb21.i74.i1514.3, label %panic1.i73.i1513, !dbg !38004

bb21.i74.i1514.3:                                 ; preds = %bb17.i63.i1503.3
  %226 = getelementptr inbounds nuw float, ptr %_147.0.i75.i1515, i64 %_50.i70.i1510.3, !dbg !38004
  %_49.i76.i1516.3 = load float, ptr %226, align 4, !dbg !38004, !noalias !37913, !noundef !12
  store float %_49.i76.i1516.3, ptr %iter.i32.i1008.sroa.0.0.ptr7335.3, align 4, !dbg !38005, !noalias !37913
  %227 = icmp eq i64 %width.i33.i1473, 4, !dbg !37925
  br i1 %227, label %bb16.i77.i1517, label %bb39.i57.i1497.4, !dbg !37925

bb39.i57.i1497.4:                                 ; preds = %bb21.i74.i1514.3
  %exitcond11751.4.not = icmp eq i64 %_145.1.i60.i1500, 4, !dbg !37931
  br i1 %exitcond11751.4.not, label %panic.i62.i1502, label %bb17.i63.i1503.4, !dbg !37931

bb17.i63.i1503.4:                                 ; preds = %bb39.i57.i1497.4
  %228 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i1504, i64 56, !dbg !37931
  %_44.i65.i1505.4 = load i32, ptr %228, align 4, !dbg !37931, !noalias !37913, !noundef !12
  %_43.i66.i1506.4 = zext i32 %_44.i65.i1505.4 to i64, !dbg !37931
  %229 = add i64 %ring_cursor.sroa.0.1.i14487349, %_43.i66.i1506.4, !dbg !38000
  %_47.not.i67.i1507.4 = icmp ult i64 %229, %_91.i1472, !dbg !38001
  %230 = select i1 %_47.not.i67.i1507.4, i64 0, i64 %_91.i1472, !dbg !38001
  %spec.select.i68.i1508.4 = sub nuw i64 %229, %230, !dbg !38001
  %_51.i69.i1509.4 = mul i64 %spec.select.i68.i1508.4, %width.i33.i1473, !dbg !38003
  %_50.i70.i1510.4 = add i64 %_51.i69.i1509.4, 4, !dbg !38003
  %_53.i72.i1512.4 = icmp ult i64 %_50.i70.i1510.4, %_149.1.i82.i1522.pre, !dbg !38004
  br i1 %_53.i72.i1512.4, label %bb21.i74.i1514.4, label %panic1.i73.i1513, !dbg !38004

bb21.i74.i1514.4:                                 ; preds = %bb17.i63.i1503.4
  %231 = getelementptr inbounds nuw float, ptr %_147.0.i75.i1515, i64 %_50.i70.i1510.4, !dbg !38004
  %_49.i76.i1516.4 = load float, ptr %231, align 4, !dbg !38004, !noalias !37913, !noundef !12
  store float %_49.i76.i1516.4, ptr %iter.i32.i1008.sroa.0.0.ptr7335.4, align 4, !dbg !38005, !noalias !37913
  %232 = icmp eq i64 %width.i33.i1473, 5, !dbg !37925
  br i1 %232, label %bb16.i77.i1517, label %bb39.i57.i1497.5, !dbg !37925

bb39.i57.i1497.5:                                 ; preds = %bb21.i74.i1514.4
  %exitcond11751.5.not = icmp eq i64 %_145.1.i60.i1500, 5, !dbg !37931
  br i1 %exitcond11751.5.not, label %panic.i62.i1502, label %bb17.i63.i1503.5, !dbg !37931

bb17.i63.i1503.5:                                 ; preds = %bb39.i57.i1497.5
  %233 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i1504, i64 68, !dbg !37931
  %_44.i65.i1505.5 = load i32, ptr %233, align 4, !dbg !37931, !noalias !37913, !noundef !12
  %_43.i66.i1506.5 = zext i32 %_44.i65.i1505.5 to i64, !dbg !37931
  %234 = add i64 %ring_cursor.sroa.0.1.i14487349, %_43.i66.i1506.5, !dbg !38000
  %_47.not.i67.i1507.5 = icmp ult i64 %234, %_91.i1472, !dbg !38001
  %235 = select i1 %_47.not.i67.i1507.5, i64 0, i64 %_91.i1472, !dbg !38001
  %spec.select.i68.i1508.5 = sub nuw i64 %234, %235, !dbg !38001
  %_51.i69.i1509.5 = mul i64 %spec.select.i68.i1508.5, %width.i33.i1473, !dbg !38003
  %_50.i70.i1510.5 = add i64 %_51.i69.i1509.5, 5, !dbg !38003
  %_53.i72.i1512.5 = icmp ult i64 %_50.i70.i1510.5, %_149.1.i82.i1522.pre, !dbg !38004
  br i1 %_53.i72.i1512.5, label %bb21.i74.i1514.5, label %panic1.i73.i1513, !dbg !38004

bb21.i74.i1514.5:                                 ; preds = %bb17.i63.i1503.5
  %236 = getelementptr inbounds nuw float, ptr %_147.0.i75.i1515, i64 %_50.i70.i1510.5, !dbg !38004
  %_49.i76.i1516.5 = load float, ptr %236, align 4, !dbg !38004, !noalias !37913, !noundef !12
  store float %_49.i76.i1516.5, ptr %iter.i32.i1008.sroa.0.0.ptr7335.5, align 4, !dbg !38005, !noalias !37913
  %237 = icmp eq i64 %width.i33.i1473, 6, !dbg !37925
  br i1 %237, label %bb16.i77.i1517, label %bb39.i57.i1497.6, !dbg !37925

bb39.i57.i1497.6:                                 ; preds = %bb21.i74.i1514.5
  %exitcond11751.6.not = icmp eq i64 %_145.1.i60.i1500, 6, !dbg !37931
  br i1 %exitcond11751.6.not, label %panic.i62.i1502, label %bb17.i63.i1503.6, !dbg !37931

bb17.i63.i1503.6:                                 ; preds = %bb39.i57.i1497.6
  %238 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i1504, i64 80, !dbg !37931
  %_44.i65.i1505.6 = load i32, ptr %238, align 4, !dbg !37931, !noalias !37913, !noundef !12
  %_43.i66.i1506.6 = zext i32 %_44.i65.i1505.6 to i64, !dbg !37931
  %239 = add i64 %ring_cursor.sroa.0.1.i14487349, %_43.i66.i1506.6, !dbg !38000
  %_47.not.i67.i1507.6 = icmp ult i64 %239, %_91.i1472, !dbg !38001
  %240 = select i1 %_47.not.i67.i1507.6, i64 0, i64 %_91.i1472, !dbg !38001
  %spec.select.i68.i1508.6 = sub nuw i64 %239, %240, !dbg !38001
  %_51.i69.i1509.6 = mul i64 %spec.select.i68.i1508.6, %width.i33.i1473, !dbg !38003
  %_50.i70.i1510.6 = add i64 %_51.i69.i1509.6, 6, !dbg !38003
  %_53.i72.i1512.6 = icmp ult i64 %_50.i70.i1510.6, %_149.1.i82.i1522.pre, !dbg !38004
  br i1 %_53.i72.i1512.6, label %bb21.i74.i1514.6, label %panic1.i73.i1513, !dbg !38004

bb21.i74.i1514.6:                                 ; preds = %bb17.i63.i1503.6
  %241 = getelementptr inbounds nuw float, ptr %_147.0.i75.i1515, i64 %_50.i70.i1510.6, !dbg !38004
  %_49.i76.i1516.6 = load float, ptr %241, align 4, !dbg !38004, !noalias !37913, !noundef !12
  store float %_49.i76.i1516.6, ptr %iter.i32.i1008.sroa.0.0.ptr7335.6, align 4, !dbg !38005, !noalias !37913
  %242 = icmp eq i64 %width.i33.i1473, 7, !dbg !37925
  br i1 %242, label %bb16.i77.i1517, label %bb39.i57.i1497.7, !dbg !37925

bb39.i57.i1497.7:                                 ; preds = %bb21.i74.i1514.6
  %exitcond11751.7.not = icmp eq i64 %_145.1.i60.i1500, 7, !dbg !37931
  br i1 %exitcond11751.7.not, label %panic.i62.i1502, label %bb17.i63.i1503.7, !dbg !37931

bb17.i63.i1503.7:                                 ; preds = %bb39.i57.i1497.7
  %243 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i1504, i64 92, !dbg !37931
  %_44.i65.i1505.7 = load i32, ptr %243, align 4, !dbg !37931, !noalias !37913, !noundef !12
  %_43.i66.i1506.7 = zext i32 %_44.i65.i1505.7 to i64, !dbg !37931
  %244 = add i64 %ring_cursor.sroa.0.1.i14487349, %_43.i66.i1506.7, !dbg !38000
  %_47.not.i67.i1507.7 = icmp ult i64 %244, %_91.i1472, !dbg !38001
  %245 = select i1 %_47.not.i67.i1507.7, i64 0, i64 %_91.i1472, !dbg !38001
  %spec.select.i68.i1508.7 = sub nuw i64 %244, %245, !dbg !38001
  %_51.i69.i1509.7 = mul i64 %spec.select.i68.i1508.7, %width.i33.i1473, !dbg !38003
  %_50.i70.i1510.7 = add i64 %_51.i69.i1509.7, 7, !dbg !38003
  %_53.i72.i1512.7 = icmp ult i64 %_50.i70.i1510.7, %_149.1.i82.i1522.pre, !dbg !38004
  br i1 %_53.i72.i1512.7, label %bb21.i74.i1514.7, label %panic1.i73.i1513, !dbg !38004

bb21.i74.i1514.7:                                 ; preds = %bb17.i63.i1503.7
  %246 = getelementptr inbounds nuw float, ptr %_147.0.i75.i1515, i64 %_50.i70.i1510.7, !dbg !38004
  %_49.i76.i1516.7 = load float, ptr %246, align 4, !dbg !38004, !noalias !37913, !noundef !12
  store float %_49.i76.i1516.7, ptr %iter.i32.i1008.sroa.0.0.ptr7335.7, align 4, !dbg !38005, !noalias !37913
  br label %bb16.i77.i1517, !dbg !37925

panic1.i73.i1513:                                 ; preds = %bb17.i63.i1503.7, %bb17.i63.i1503.6, %bb17.i63.i1503.5, %bb17.i63.i1503.4, %bb17.i63.i1503.3, %bb17.i63.i1503.2, %bb17.i63.i1503.1, %bb17.i63.i1503
  %_50.i70.i1510.lcssa.ph = phi i64 [ %_50.i70.i1510.7, %bb17.i63.i1503.7 ], [ %_50.i70.i1510.6, %bb17.i63.i1503.6 ], [ %_50.i70.i1510.5, %bb17.i63.i1503.5 ], [ %_50.i70.i1510.4, %bb17.i63.i1503.4 ], [ %_50.i70.i1510.3, %bb17.i63.i1503.3 ], [ %_50.i70.i1510.2, %bb17.i63.i1503.2 ], [ %_50.i70.i1510.1, %bb17.i63.i1503.1 ], [ %_51.i69.i1509, %bb17.i63.i1503 ]
  store float %_0.i33908185, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i70.i1510.lcssa.ph, i64 noundef %_149.1.i82.i1522.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b305c1483509cfb31fdec21ff8752674) #30, !dbg !38004, !noalias !37913
  unreachable, !dbg !38004

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3637: ; preds = %bb49.i104.i1544
  %_127.i107.i1547 = getelementptr inbounds nuw float, ptr %_150.0.i105.i1545, i64 %_76.i102.i1542, !dbg !38006
  %_0.i3476 = load float, ptr %_127.i107.i1547, align 4, !dbg !37996, !alias.scope !37992, !noalias !37913, !noundef !12
  store float %_0.i3485, ptr %_127.i107.i1547, align 4, !dbg !38011, !alias.scope !38014, !noalias !37913
  %_0.i2970 = fmul float %_0.i3387, %_0.i3476, !dbg !38017
  %_6.i3903 = bitcast float %_0.i3476 to i32, !dbg !38019
  %_5.i3904 = and i32 %_6.i3903, %all.sroa.0.0.i1024, !dbg !38022
  %_8.i3905 = bitcast float %_0.i2970 to i32, !dbg !38023
  %_7.i3907 = and i32 %_9.i3906, %_8.i3905, !dbg !38025
  %_4.i3908 = or disjoint i32 %_7.i3907, %_5.i3904, !dbg !38022
  store i32 %_4.i3908, ptr %_174.i1470, align 4, !dbg !38026, !alias.scope !38028, !noalias !38031
  %_175.i1561 = icmp ugt i64 %_64.i1452, %right_io.1, !dbg !38032
  br i1 %_175.i1561, label %bb54.i1664, label %bb55.i1562, !dbg !38032, !prof !1406

bb52.i1670:                                       ; preds = %bb48.i1451
  store float %_0.i33908185, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_64.i1452, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_422064f3ca430d31d9007f55b436c6ca) #30, !dbg !38036, !noalias !37086
  unreachable, !dbg !38036

bb55.i1562:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3637
  %_181.i1564 = getelementptr inbounds nuw float, ptr %right_io.0, i64 %_64.i1452, !dbg !38037
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38042), !dbg !38045
  %_3.not.i3469 = icmp eq i64 %right_io.1, %_64.i1452, !dbg !38046
  br i1 %_3.not.i3469, label %panic.i3472, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3473, !dbg !38046

panic.i3472:                                      ; preds = %bb55.i1562
  store float %_0.i3390, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !38046, !noalias !38048
  unreachable, !dbg !38046

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3473: ; preds = %bb55.i1562
  %_0.i3471 = load float, ptr %_181.i1564, align 4, !dbg !38046, !alias.scope !38042, !noalias !37086, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38049), !dbg !38052
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38053), !dbg !38052
  %width.i.i1566 = load i64, ptr %150, align 8, !dbg !38055, !alias.scope !38056, !noalias !38057, !noundef !12
  %_3.i2503 = fcmp uge float %_0.i3877, %_0.i3922, !dbg !38060
  %_0.i2936 = fdiv float %_0.i3877, %_0.i3922, !dbg !38062
  %_0.i3902 = select i1 %_3.i2503, float 1.000000e+00, float %_0.i2936, !dbg !38064
  %_144.1.i.i1571 = load i64, ptr %151, align 8, !dbg !38066, !alias.scope !38056, !noalias !38057, !noundef !12
  %_22.i.i1572 = mul i64 %width.i.i1566, %ring_cursor.sroa.0.1.i14487349, !dbg !38067
  %_92.i.i1573 = icmp ugt i64 %_22.i.i1572, %_144.1.i.i1571, !dbg !38068
  br i1 %_92.i.i1573, label %bb37.i.i1663, label %bb38.i.i1574, !dbg !38068, !prof !1406

bb38.i.i1574:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3473
  %_144.0.i.i1575 = load ptr, ptr %152, align 8, !dbg !38066, !alias.scope !38056, !noalias !38057, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38071), !dbg !38074
  %_4.not.i3630 = icmp eq i64 %_144.1.i.i1571, %_22.i.i1572, !dbg !38075
  br i1 %_4.not.i3630, label %panic.i3632, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3633, !dbg !38075

panic.i3632:                                      ; preds = %bb38.i.i1574
  store float %_0.i3390, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !38075, !noalias !38077
  unreachable, !dbg !38075

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3633: ; preds = %bb38.i.i1574
  %_99.i.i1577 = getelementptr inbounds nuw float, ptr %_144.0.i.i1575, i64 %_22.i.i1572, !dbg !38078
  store float %_0.i3902, ptr %_99.i.i1577, align 4, !dbg !38075, !alias.scope !38071, !noalias !38080
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38081), !dbg !38084
  %width.i = load i64, ptr %150, align 8, !dbg !38085, !alias.scope !38081, !noalias !38087, !noundef !12
  %247 = icmp eq i64 %width.i, 0, !dbg !38089
  br i1 %247, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit, label %bb32.i1975.lr.ph, !dbg !38089

bb32.i1975.lr.ph:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3633
  %_112.1.i = load i64, ptr %153, align 8, !alias.scope !38081, !noalias !38087, !noundef !12
  %_112.0.i = load ptr, ptr %154, align 8, !nonnull !12
  %248 = add i64 %ring_cursor.sroa.0.1.i14487349, 1
  %_23.not.i = icmp ult i64 %248, %_91.i1472
  %249 = select i1 %_23.not.i, i64 0, i64 %_91.i1472
  %start1.sroa.0.0.i = sub nuw i64 %248, %249
  %_114.1.i = load i64, ptr %151, align 8
  %_114.0.i = load ptr, ptr %152, align 8, !nonnull !12
  %_116.1.i = load i64, ptr %155, align 8
  %_116.0.i = load ptr, ptr %156, align 8, !nonnull !12
  %_118.1.i = load i64, ptr %157, align 8
  %_118.0.i = load ptr, ptr %158, align 8, !nonnull !12
  %_45.i1992 = mul i64 %width.i, %start1.sroa.0.0.i
  br label %bb32.i1975, !dbg !38089

bb32.i1975:                                       ; preds = %bb32.i1975.lr.ph, %bb31.i1995
  %iter.i1972.sroa.10.07341 = phi i64 [ %width.i, %bb32.i1975.lr.ph ], [ %250, %bb31.i1995 ]
  %iter.i1972.sroa.7.07340 = phi i64 [ 0, %bb32.i1975.lr.ph ], [ %_9.0.i4639, %bb31.i1995 ]
  %iter.i1972.sroa.0.0.idx7339 = phi i64 [ 0, %bb32.i1975.lr.ph ], [ %iter.i1972.sroa.0.0.add, %bb31.i1995 ]
  %iter.i1972.sroa.0.0.ptr7342 = getelementptr inbounds nuw i8, ptr %scratch.i1015, i64 %iter.i1972.sroa.0.0.idx7339, !dbg !38091
  %250 = add i64 %iter.i1972.sroa.10.07341, -1, !dbg !38091
  %_7.i.i4635 = icmp eq i64 %iter.i1972.sroa.0.0.idx7339, 32, !dbg !38092
  br i1 %_7.i.i4635, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit, label %bb3.i1977, !dbg !38096

bb3.i1977:                                        ; preds = %bb32.i1975
  %iter.i1972.sroa.0.0.add = add nuw nsw i64 %iter.i1972.sroa.0.0.idx7339, 4, !dbg !38097
  %_9.0.i4639 = add nuw nsw i64 %iter.i1972.sroa.7.07340, 1, !dbg !38099
  %exitcond11754.not = icmp eq i64 %iter.i1972.sroa.7.07340, %_112.1.i, !dbg !38100
  br i1 %exitcond11754.not, label %panic.i, label %bb5.i1978, !dbg !38100

bb5.i1978:                                        ; preds = %bb3.i1977
  %251 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i, i64 %iter.i1972.sroa.7.07340, !dbg !38100
  %shape.i = load i32, ptr %251, align 4, !dbg !38100, !noalias !38101, !noundef !12
  %252 = getelementptr inbounds nuw i8, ptr %251, i64 4, !dbg !38100
  %shape3.i = load i32, ptr %252, align 4, !dbg !38100, !noalias !38101, !noundef !12
  %window.i = zext i32 %shape.i to i64, !dbg !38102
  %_19.i = zext i32 %shape3.i to i64, !dbg !38103
  %253 = add i64 %ring_cursor.sroa.0.1.i14487349, %_19.i, !dbg !38104
  %_20.not.i = icmp ult i64 %253, %_91.i1472, !dbg !38105
  %254 = select i1 %_20.not.i, i64 0, i64 %_91.i1472, !dbg !38105
  %spec.select.i = sub nuw i64 %253, %254, !dbg !38105
  %_27.i1979 = mul i64 %spec.select.i, %width.i, !dbg !38106
  %_26.i = add i64 %_27.i1979, %iter.i1972.sroa.7.07340, !dbg !38106
  %_30.i1980 = icmp ult i64 %_26.i, %_114.1.i, !dbg !38107
  br i1 %_30.i1980, label %bb12.i1981, label %panic5.i, !dbg !38107

panic.i:                                          ; preds = %bb3.i1977
  store float %_0.i3390, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i, i64 noundef %_112.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4a8785a681d008a9bfd0cd82628ea9cb) #30, !dbg !38100, !noalias !38101
  unreachable, !dbg !38100

bb12.i1981:                                       ; preds = %bb5.i1978
  %255 = getelementptr inbounds nuw float, ptr %_114.0.i, i64 %_26.i, !dbg !38107
  %256 = load float, ptr %255, align 4, !dbg !38107, !noalias !38101, !noundef !12
  %exitcond11755.not = icmp eq i64 %iter.i1972.sroa.7.07340, %_116.1.i, !dbg !38108
  br i1 %exitcond11755.not, label %panic6.i, label %bb13.i1983, !dbg !38108

panic5.i:                                         ; preds = %bb5.i1978
  store float %_0.i3390, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i, i64 noundef %_114.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cbce7773ac40979e4ba2385da3aec116) #30, !dbg !38107, !noalias !38101
  unreachable, !dbg !38107

bb13.i1983:                                       ; preds = %bb12.i1981
  %257 = getelementptr inbounds nuw i32, ptr %_116.0.i, i64 %iter.i1972.sroa.7.07340, !dbg !38108
  %_32.i1984 = load i32, ptr %257, align 4, !dbg !38108, !noalias !38101, !noundef !12
  %position.i1985 = zext i32 %_32.i1984 to i64, !dbg !38108
  %258 = icmp eq i32 %_32.i1984, 0, !dbg !38109
  br i1 %258, label %bb17.i, label %bb15.i1986, !dbg !38109

panic6.i:                                         ; preds = %bb12.i1981
  store float %_0.i3390, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i, i64 noundef %_116.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ec0d48f73ebfc2755df5cedaa60b5c0a) #30, !dbg !38108, !noalias !38101
  unreachable, !dbg !38108

bb15.i1986:                                       ; preds = %bb13.i1983
  %_37.i = icmp ult i64 %iter.i1972.sroa.7.07340, %_118.1.i, !dbg !38110
  br i1 %_37.i, label %bb16.i1987, label %panic7.i, !dbg !38110

bb17.i:                                           ; preds = %bb35.i, %bb16.i1987, %bb13.i1983
  %newest.sroa.0.0.i = phi float [ %256, %bb13.i1983 ], [ %_35.i1988, %bb35.i ], [ %256, %bb16.i1987 ], !dbg !38111
  %exitcond11756.not = icmp eq i64 %iter.i1972.sroa.7.07340, %_118.1.i, !dbg !38112
  br i1 %exitcond11756.not, label %panic8.i, label %bb18.i, !dbg !38112

bb16.i1987:                                       ; preds = %bb15.i1986
  %259 = getelementptr inbounds nuw float, ptr %_118.0.i, i64 %iter.i1972.sroa.7.07340, !dbg !38110
  %_35.i1988 = load float, ptr %259, align 4, !dbg !38110, !noalias !38101, !noundef !12
  %_102.i = fcmp olt float %_35.i1988, %256, !dbg !38113
  br i1 %_102.i, label %bb35.i, label %bb17.i, !dbg !38113

panic7.i:                                         ; preds = %bb15.i1986
  store float %_0.i3390, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i1972.sroa.7.07340, i64 noundef %_118.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2c461872bb652d4796cdcf89c28c82c8) #30, !dbg !38110, !noalias !38101
  unreachable, !dbg !38110

bb35.i:                                           ; preds = %bb16.i1987
  br label %bb17.i, !dbg !38115

bb18.i:                                           ; preds = %bb17.i
  %260 = getelementptr inbounds nuw float, ptr %_118.0.i, i64 %iter.i1972.sroa.7.07340, !dbg !38112
  store float %newest.sroa.0.0.i, ptr %260, align 4, !dbg !38112, !noalias !38101
  %_42.i = add nuw nsw i64 %position.i1985, 1, !dbg !38116
  %complete.i1990 = icmp eq i64 %_42.i, %window.i, !dbg !38116
  br i1 %complete.i1990, label %bb22.i, label %bb20.i1991, !dbg !38117

panic8.i:                                         ; preds = %bb17.i
  store float %_0.i3390, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i, i64 noundef %_118.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2b690e2c7763f11809942906fc2ca813) #30, !dbg !38112, !noalias !38101
  unreachable, !dbg !38112

bb20.i1991:                                       ; preds = %bb18.i
  %_44.i = add i64 %iter.i1972.sroa.7.07340, %_45.i1992, !dbg !38118
  %_47.i = icmp ult i64 %_44.i, %_114.1.i, !dbg !38119
  br i1 %_47.i, label %bb30.i, label %panic9.i, !dbg !38119

panic9.i:                                         ; preds = %bb20.i1991
  store float %_0.i3390, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i, i64 noundef %_114.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9fa421ae81817f58fcfc4a3243223891) #30, !dbg !38119, !noalias !38101
  unreachable, !dbg !38119

bb30.i:                                           ; preds = %bb20.i1991
  %261 = getelementptr inbounds nuw float, ptr %_114.0.i, i64 %_44.i, !dbg !38119
  %_43.i = load float, ptr %261, align 4, !dbg !38119, !noalias !38101, !noundef !12
  %_103.i1993 = fcmp olt float %_43.i, %newest.sroa.0.0.i, !dbg !38120
  %newest.sroa.0.1.i = select i1 %_103.i1993, float %_43.i, float %newest.sroa.0.0.i, !dbg !38120
  store float %newest.sroa.0.1.i, ptr %iter.i1972.sroa.0.0.ptr7342, align 4, !dbg !38122, !noalias !38101
  %262 = trunc i64 %_42.i to i32, !dbg !38123
  br label %bb31.i1995, !dbg !38124

bb31.i1995:                                       ; preds = %bb25.i, %bb30.i
  %storemerge5673 = phi i32 [ %262, %bb30.i ], [ 0, %bb25.i ], !dbg !38125
  store i32 %storemerge5673, ptr %257, align 4, !dbg !38125, !noalias !38101
  %263 = icmp eq i64 %250, 0, !dbg !38089
  br i1 %263, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit, label %bb32.i1975, !dbg !38089

bb22.i:                                           ; preds = %bb18.i
  store float %newest.sroa.0.0.i, ptr %iter.i1972.sroa.0.0.ptr7342, align 4, !dbg !38122, !noalias !38101
  %264 = load float, ptr %255, align 4, !dbg !38126, !noalias !38101, !noundef !12
  br label %bb41.i2002, !dbg !38127

bb41.i2002:                                       ; preds = %bb22.i, %bb25.i
  %iter2.sroa.0.0.i19987338 = phi i64 [ 0, %bb22.i ], [ %_105.i, %bb25.i ]
  %suffix.sroa.0.0.i19977337 = phi float [ %264, %bb22.i ], [ %suffix.sroa.0.1.i, %bb25.i ]
  %end.sroa.0.1.i7336 = phi i64 [ %spec.select.i, %bb22.i ], [ %267, %bb25.i ]
  %_56.i = mul i64 %end.sroa.0.1.i7336, %width.i, !dbg !38130
  %_55.i = add i64 %_56.i, %iter.i1972.sroa.7.07340, !dbg !38130
  %_59.i = icmp ult i64 %_55.i, %_114.1.i, !dbg !38131
  br i1 %_59.i, label %bb25.i, label %panic13.i, !dbg !38131

panic13.i:                                        ; preds = %bb41.i2002
  store float %_0.i3390, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i, i64 noundef %_114.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_91a4c6b9b17ebf4d863f9a70b6dc929a) #30, !dbg !38131, !noalias !38101
  unreachable, !dbg !38131

bb25.i:                                           ; preds = %bb41.i2002
  %_105.i = add nuw nsw i64 %iter2.sroa.0.0.i19987338, 1, !dbg !38132
  %265 = getelementptr inbounds nuw float, ptr %_114.0.i, i64 %_55.i, !dbg !38131
  %_54.i = load float, ptr %265, align 4, !dbg !38131, !noalias !38101, !noundef !12
  %_107.i2003 = fcmp olt float %suffix.sroa.0.0.i19977337, %_54.i, !dbg !38135
  %suffix.sroa.0.1.i = select i1 %_107.i2003, float %suffix.sroa.0.0.i19977337, float %_54.i, !dbg !38135
  store float %suffix.sroa.0.1.i, ptr %265, align 4, !dbg !38137, !noalias !38101
  %266 = icmp eq i64 %end.sroa.0.1.i7336, 0, !dbg !38138
  %spec.store.select.i2004 = select i1 %266, i64 %_91.i1472, i64 %end.sroa.0.1.i7336, !dbg !38138
  %267 = add i64 %spec.store.select.i2004, -1, !dbg !38139
  %exitcond11753.not = icmp eq i64 %_105.i, %window.i, !dbg !38140
  br i1 %exitcond11753.not, label %bb31.i1995, label %bb41.i2002, !dbg !38127

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit: ; preds = %bb32.i1975, %bb31.i1995
  %_0.i3468.pre = load float, ptr %scratch.i1015, align 4, !dbg !38142, !alias.scope !38144, !noalias !38147
  br label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit, !dbg !38142

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit: ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3633
  %_0.i3468 = phi float [ %_0.i3468.pre, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit ], [ %_0.i3480, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3633 ], !dbg !38142
  %_0.i2969 = fmul float %_0.i3468, 1.638400e+04, !dbg !38148
  %268 = tail call noundef float @llvm.floor.f32(float %_0.i2969), !dbg !38150
  %_0.i2968 = fmul float %268, 0x3F10000000000000, !dbg !38154
  %269 = icmp eq i64 %width.i.i1566, 0, !dbg !38156
  %_149.1.i.i1615.pre = load i64, ptr %159, align 8, !dbg !38158, !alias.scope !38056, !noalias !38057
  br i1 %269, label %bb16.i.i1610, label %bb39.i.i1590.lr.ph, !dbg !38156

bb39.i.i1590.lr.ph:                               ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit
  %_145.1.i.i1593 = load i64, ptr %153, align 8, !alias.scope !38056, !noalias !38057, !noundef !12
  %_145.0.i.i1597 = load ptr, ptr %154, align 8, !nonnull !12
  %_147.0.i.i1608 = load ptr, ptr %160, align 8, !nonnull !12
  %exitcond11757.not = icmp eq i64 %_145.1.i.i1593, 0, !dbg !38159
  br i1 %exitcond11757.not, label %panic.i.i1595, label %bb17.i.i1596, !dbg !38159

bb37.i.i1663:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3473
  store float %_0.i3390, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i1572, i64 noundef %_144.1.i.i1571, i64 noundef %_144.1.i.i1571, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_56df7c041d29359441bca272bf4e38e3) #30, !dbg !38160, !noalias !38080
  unreachable, !dbg !38160

bb16.i.i1610:                                     ; preds = %bb21.i.i1607.7, %bb21.i.i1607, %bb21.i.i1607.1, %bb21.i.i1607.2, %bb21.i.i1607.3, %bb21.i.i1607.4, %bb21.i.i1607.5, %bb21.i.i1607.6, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit
  %_0.i3466 = phi float [ %_0.i3468, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit ], [ %_49.i.i1609, %bb21.i.i1607 ], [ %_49.i.i1609, %bb21.i.i1607.7 ], [ %_49.i.i1609, %bb21.i.i1607.6 ], [ %_49.i.i1609, %bb21.i.i1607.5 ], [ %_49.i.i1609, %bb21.i.i1607.4 ], [ %_49.i.i1609, %bb21.i.i1607.3 ], [ %_49.i.i1609, %bb21.i.i1607.2 ], [ %_49.i.i1609, %bb21.i.i1607.1 ], !dbg !38161
  %_0.i2543 = fadd float %_0.i2968, %_0.i33868189, !dbg !38163
  %_0.i3386 = fsub float %_0.i2543, %_0.i3466, !dbg !38165
  %_109.i.i1616 = icmp ugt i64 %_22.i.i1572, %_149.1.i.i1615.pre, !dbg !38167
  br i1 %_109.i.i1616, label %bb42.i.i1662, label %bb43.i.i1617, !dbg !38167, !prof !1406

bb43.i.i1617:                                     ; preds = %bb16.i.i1610
  %_149.0.i.i1618 = load ptr, ptr %160, align 8, !dbg !38158, !alias.scope !38056, !noalias !38057, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38170), !dbg !38173
  %_4.not.i3626 = icmp eq i64 %_149.1.i.i1615.pre, %_22.i.i1572, !dbg !38174
  br i1 %_4.not.i3626, label %panic.i3628, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3629, !dbg !38174

panic.i3628:                                      ; preds = %bb43.i.i1617
  store float %_0.i3390, ptr %145, align 1, !dbg !37041
  store float %_0.i3386, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !38174, !noalias !38176
  unreachable, !dbg !38174

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3629: ; preds = %bb43.i.i1617
  %_116.i.i1620 = getelementptr inbounds nuw float, ptr %_149.0.i.i1618, i64 %_22.i.i1572, !dbg !38177
  store float %_0.i2968, ptr %_116.i.i1620, align 4, !dbg !38174, !alias.scope !38170, !noalias !38147
  %_0.i2935 = fdiv float %_0.i3386, %_64.i.i1622, !dbg !38179
  %_0.i3385 = fsub float 1.000000e+00, %_0.i2935, !dbg !38181
  %_0.i3384 = fsub float %_0.i3385, %_0.i37608191, !dbg !38183
  %_4.i2951 = fmul float %_0.i3890, %_0.i3384, !dbg !38185
  %_0.i2952 = fadd float %_0.i37608191, %_4.i2951, !dbg !38185
  %_3.i.i4101.inv = fcmp ogt float %_0.i3385, %_0.i2952, !dbg !38187
  %_4.i.i.v = select i1 %_3.i.i4101.inv, float %_0.i3385, float %_0.i2952, !dbg !38187
  %270 = tail call noundef float @llvm.fabs.f32(float %_4.i.i.v), !dbg !38190
  %271 = fcmp uge float %270, 0x3BC79CA100000000, !dbg !38193
  %_0.i3760 = select i1 %271, float %_4.i.i.v, float 0.000000e+00, !dbg !38195
  store float %_0.i3760, ptr %163, align 4, !dbg !38196, !alias.scope !38049, !noalias !38197
  %_0.i3383 = fsub float 1.000000e+00, %_0.i3760, !dbg !38198
  %_150.1.i.i1634 = load i64, ptr %164, align 8, !dbg !38200, !alias.scope !38056, !noalias !38057, !noundef !12
  %_76.i.i1635 = mul i64 %width.i.i1566, %main_cursor.sroa.0.1.i14497350, !dbg !38201
  %_120.i.i1636 = icmp ugt i64 %_76.i.i1635, %_150.1.i.i1634, !dbg !38202
  br i1 %_120.i.i1636, label %bb48.i.i1661, label %bb49.i.i1637, !dbg !38202, !prof !1406

bb42.i.i1662:                                     ; preds = %bb16.i.i1610
  store float %_0.i3390, ptr %145, align 1, !dbg !37041
  store float %_0.i3386, ptr %161, align 1, !dbg !37064
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i1572, i64 noundef %_149.1.i.i1615.pre, i64 noundef %_149.1.i.i1615.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_90498045d73339daaf9e4f537508f58b) #30, !dbg !38205, !noalias !38147
  unreachable, !dbg !38205

bb49.i.i1637:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3629
  %_150.0.i.i1638 = load ptr, ptr %165, align 8, !dbg !38200, !alias.scope !38056, !noalias !38057, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38206), !dbg !38209
  %_3.not.i = icmp eq i64 %_150.1.i.i1634, %_76.i.i1635, !dbg !38210
  br i1 %_3.not.i, label %panic.i3464, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit, !dbg !38210

panic.i3464:                                      ; preds = %bb49.i.i1637
  store float %_0.i3390, ptr %145, align 1, !dbg !37041
  store float %_0.i3386, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !38210, !noalias !38212
  unreachable, !dbg !38210

bb48.i.i1661:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3629
  store float %_0.i3390, ptr %145, align 1, !dbg !37041
  store float %_0.i3386, ptr %161, align 1, !dbg !37064
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i.i1635, i64 noundef %_150.1.i.i1634, i64 noundef %_150.1.i.i1634, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_da8af254b6d507a8e2ca31e544bfd21d) #30, !dbg !38213, !noalias !38147
  unreachable, !dbg !38213

bb17.i.i1596:                                     ; preds = %bb39.i.i1590.lr.ph
  %272 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1597, i64 8, !dbg !38159
  %_44.i.i1598 = load i32, ptr %272, align 4, !dbg !38159, !noalias !38147, !noundef !12
  %_43.i.i1599 = zext i32 %_44.i.i1598 to i64, !dbg !38159
  %273 = add i64 %ring_cursor.sroa.0.1.i14487349, %_43.i.i1599, !dbg !38214
  %_47.not.i.i1600 = icmp ult i64 %273, %_91.i1472, !dbg !38215
  %274 = select i1 %_47.not.i.i1600, i64 0, i64 %_91.i1472, !dbg !38215
  %spec.select.i.i1601 = sub nuw i64 %273, %274, !dbg !38215
  %_51.i.i1602 = mul i64 %spec.select.i.i1601, %width.i.i1566, !dbg !38216
  %_53.i.i1605 = icmp ult i64 %_51.i.i1602, %_149.1.i.i1615.pre, !dbg !38217
  br i1 %_53.i.i1605, label %bb21.i.i1607, label %panic1.i.i1606, !dbg !38217

panic.i.i1595:                                    ; preds = %bb39.i.i1590.7, %bb39.i.i1590.6, %bb39.i.i1590.5, %bb39.i.i1590.4, %bb39.i.i1590.3, %bb39.i.i1590.2, %bb39.i.i1590.1, %bb39.i.i1590.lr.ph
  %_145.1.i.i1593.lcssa.ph = phi i64 [ 7, %bb39.i.i1590.7 ], [ 6, %bb39.i.i1590.6 ], [ 5, %bb39.i.i1590.5 ], [ 4, %bb39.i.i1590.4 ], [ 3, %bb39.i.i1590.3 ], [ 2, %bb39.i.i1590.2 ], [ 1, %bb39.i.i1590.1 ], [ 0, %bb39.i.i1590.lr.ph ]
  store float %_0.i3390, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i.i1593.lcssa.ph, i64 noundef %_145.1.i.i1593.lcssa.ph, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a6feb40b34112df5f214f84424dd2c7c) #30, !dbg !38159, !noalias !38147
  unreachable, !dbg !38159

bb21.i.i1607:                                     ; preds = %bb17.i.i1596
  %275 = getelementptr inbounds nuw float, ptr %_147.0.i.i1608, i64 %_51.i.i1602, !dbg !38217
  %_49.i.i1609 = load float, ptr %275, align 4, !dbg !38217, !noalias !38147, !noundef !12
  store float %_49.i.i1609, ptr %scratch.i1015, align 4, !dbg !38218, !noalias !38147
  %276 = icmp eq i64 %width.i.i1566, 1, !dbg !38156
  br i1 %276, label %bb16.i.i1610, label %bb39.i.i1590.1, !dbg !38156

bb39.i.i1590.1:                                   ; preds = %bb21.i.i1607
  %exitcond11757.1.not = icmp eq i64 %_145.1.i.i1593, 1, !dbg !38159
  br i1 %exitcond11757.1.not, label %panic.i.i1595, label %bb17.i.i1596.1, !dbg !38159

bb17.i.i1596.1:                                   ; preds = %bb39.i.i1590.1
  %277 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1597, i64 20, !dbg !38159
  %_44.i.i1598.1 = load i32, ptr %277, align 4, !dbg !38159, !noalias !38147, !noundef !12
  %_43.i.i1599.1 = zext i32 %_44.i.i1598.1 to i64, !dbg !38159
  %278 = add i64 %ring_cursor.sroa.0.1.i14487349, %_43.i.i1599.1, !dbg !38214
  %_47.not.i.i1600.1 = icmp ult i64 %278, %_91.i1472, !dbg !38215
  %279 = select i1 %_47.not.i.i1600.1, i64 0, i64 %_91.i1472, !dbg !38215
  %spec.select.i.i1601.1 = sub nuw i64 %278, %279, !dbg !38215
  %_51.i.i1602.1 = mul i64 %spec.select.i.i1601.1, %width.i.i1566, !dbg !38216
  %_50.i.i1603.1 = add i64 %_51.i.i1602.1, 1, !dbg !38216
  %_53.i.i1605.1 = icmp ult i64 %_50.i.i1603.1, %_149.1.i.i1615.pre, !dbg !38217
  br i1 %_53.i.i1605.1, label %bb21.i.i1607.1, label %panic1.i.i1606, !dbg !38217

bb21.i.i1607.1:                                   ; preds = %bb17.i.i1596.1
  %280 = getelementptr inbounds nuw float, ptr %_147.0.i.i1608, i64 %_50.i.i1603.1, !dbg !38217
  %_49.i.i1609.1 = load float, ptr %280, align 4, !dbg !38217, !noalias !38147, !noundef !12
  store float %_49.i.i1609.1, ptr %iter.i.i1009.sroa.0.0.ptr7346.1, align 4, !dbg !38218, !noalias !38147
  %281 = icmp eq i64 %width.i.i1566, 2, !dbg !38156
  br i1 %281, label %bb16.i.i1610, label %bb39.i.i1590.2, !dbg !38156

bb39.i.i1590.2:                                   ; preds = %bb21.i.i1607.1
  %exitcond11757.2.not = icmp eq i64 %_145.1.i.i1593, 2, !dbg !38159
  br i1 %exitcond11757.2.not, label %panic.i.i1595, label %bb17.i.i1596.2, !dbg !38159

bb17.i.i1596.2:                                   ; preds = %bb39.i.i1590.2
  %282 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1597, i64 32, !dbg !38159
  %_44.i.i1598.2 = load i32, ptr %282, align 4, !dbg !38159, !noalias !38147, !noundef !12
  %_43.i.i1599.2 = zext i32 %_44.i.i1598.2 to i64, !dbg !38159
  %283 = add i64 %ring_cursor.sroa.0.1.i14487349, %_43.i.i1599.2, !dbg !38214
  %_47.not.i.i1600.2 = icmp ult i64 %283, %_91.i1472, !dbg !38215
  %284 = select i1 %_47.not.i.i1600.2, i64 0, i64 %_91.i1472, !dbg !38215
  %spec.select.i.i1601.2 = sub nuw i64 %283, %284, !dbg !38215
  %_51.i.i1602.2 = mul i64 %spec.select.i.i1601.2, %width.i.i1566, !dbg !38216
  %_50.i.i1603.2 = add i64 %_51.i.i1602.2, 2, !dbg !38216
  %_53.i.i1605.2 = icmp ult i64 %_50.i.i1603.2, %_149.1.i.i1615.pre, !dbg !38217
  br i1 %_53.i.i1605.2, label %bb21.i.i1607.2, label %panic1.i.i1606, !dbg !38217

bb21.i.i1607.2:                                   ; preds = %bb17.i.i1596.2
  %285 = getelementptr inbounds nuw float, ptr %_147.0.i.i1608, i64 %_50.i.i1603.2, !dbg !38217
  %_49.i.i1609.2 = load float, ptr %285, align 4, !dbg !38217, !noalias !38147, !noundef !12
  store float %_49.i.i1609.2, ptr %iter.i.i1009.sroa.0.0.ptr7346.2, align 4, !dbg !38218, !noalias !38147
  %286 = icmp eq i64 %width.i.i1566, 3, !dbg !38156
  br i1 %286, label %bb16.i.i1610, label %bb39.i.i1590.3, !dbg !38156

bb39.i.i1590.3:                                   ; preds = %bb21.i.i1607.2
  %exitcond11757.3.not = icmp eq i64 %_145.1.i.i1593, 3, !dbg !38159
  br i1 %exitcond11757.3.not, label %panic.i.i1595, label %bb17.i.i1596.3, !dbg !38159

bb17.i.i1596.3:                                   ; preds = %bb39.i.i1590.3
  %287 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1597, i64 44, !dbg !38159
  %_44.i.i1598.3 = load i32, ptr %287, align 4, !dbg !38159, !noalias !38147, !noundef !12
  %_43.i.i1599.3 = zext i32 %_44.i.i1598.3 to i64, !dbg !38159
  %288 = add i64 %ring_cursor.sroa.0.1.i14487349, %_43.i.i1599.3, !dbg !38214
  %_47.not.i.i1600.3 = icmp ult i64 %288, %_91.i1472, !dbg !38215
  %289 = select i1 %_47.not.i.i1600.3, i64 0, i64 %_91.i1472, !dbg !38215
  %spec.select.i.i1601.3 = sub nuw i64 %288, %289, !dbg !38215
  %_51.i.i1602.3 = mul i64 %spec.select.i.i1601.3, %width.i.i1566, !dbg !38216
  %_50.i.i1603.3 = add i64 %_51.i.i1602.3, 3, !dbg !38216
  %_53.i.i1605.3 = icmp ult i64 %_50.i.i1603.3, %_149.1.i.i1615.pre, !dbg !38217
  br i1 %_53.i.i1605.3, label %bb21.i.i1607.3, label %panic1.i.i1606, !dbg !38217

bb21.i.i1607.3:                                   ; preds = %bb17.i.i1596.3
  %290 = getelementptr inbounds nuw float, ptr %_147.0.i.i1608, i64 %_50.i.i1603.3, !dbg !38217
  %_49.i.i1609.3 = load float, ptr %290, align 4, !dbg !38217, !noalias !38147, !noundef !12
  store float %_49.i.i1609.3, ptr %iter.i.i1009.sroa.0.0.ptr7346.3, align 4, !dbg !38218, !noalias !38147
  %291 = icmp eq i64 %width.i.i1566, 4, !dbg !38156
  br i1 %291, label %bb16.i.i1610, label %bb39.i.i1590.4, !dbg !38156

bb39.i.i1590.4:                                   ; preds = %bb21.i.i1607.3
  %exitcond11757.4.not = icmp eq i64 %_145.1.i.i1593, 4, !dbg !38159
  br i1 %exitcond11757.4.not, label %panic.i.i1595, label %bb17.i.i1596.4, !dbg !38159

bb17.i.i1596.4:                                   ; preds = %bb39.i.i1590.4
  %292 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1597, i64 56, !dbg !38159
  %_44.i.i1598.4 = load i32, ptr %292, align 4, !dbg !38159, !noalias !38147, !noundef !12
  %_43.i.i1599.4 = zext i32 %_44.i.i1598.4 to i64, !dbg !38159
  %293 = add i64 %ring_cursor.sroa.0.1.i14487349, %_43.i.i1599.4, !dbg !38214
  %_47.not.i.i1600.4 = icmp ult i64 %293, %_91.i1472, !dbg !38215
  %294 = select i1 %_47.not.i.i1600.4, i64 0, i64 %_91.i1472, !dbg !38215
  %spec.select.i.i1601.4 = sub nuw i64 %293, %294, !dbg !38215
  %_51.i.i1602.4 = mul i64 %spec.select.i.i1601.4, %width.i.i1566, !dbg !38216
  %_50.i.i1603.4 = add i64 %_51.i.i1602.4, 4, !dbg !38216
  %_53.i.i1605.4 = icmp ult i64 %_50.i.i1603.4, %_149.1.i.i1615.pre, !dbg !38217
  br i1 %_53.i.i1605.4, label %bb21.i.i1607.4, label %panic1.i.i1606, !dbg !38217

bb21.i.i1607.4:                                   ; preds = %bb17.i.i1596.4
  %295 = getelementptr inbounds nuw float, ptr %_147.0.i.i1608, i64 %_50.i.i1603.4, !dbg !38217
  %_49.i.i1609.4 = load float, ptr %295, align 4, !dbg !38217, !noalias !38147, !noundef !12
  store float %_49.i.i1609.4, ptr %iter.i.i1009.sroa.0.0.ptr7346.4, align 4, !dbg !38218, !noalias !38147
  %296 = icmp eq i64 %width.i.i1566, 5, !dbg !38156
  br i1 %296, label %bb16.i.i1610, label %bb39.i.i1590.5, !dbg !38156

bb39.i.i1590.5:                                   ; preds = %bb21.i.i1607.4
  %exitcond11757.5.not = icmp eq i64 %_145.1.i.i1593, 5, !dbg !38159
  br i1 %exitcond11757.5.not, label %panic.i.i1595, label %bb17.i.i1596.5, !dbg !38159

bb17.i.i1596.5:                                   ; preds = %bb39.i.i1590.5
  %297 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1597, i64 68, !dbg !38159
  %_44.i.i1598.5 = load i32, ptr %297, align 4, !dbg !38159, !noalias !38147, !noundef !12
  %_43.i.i1599.5 = zext i32 %_44.i.i1598.5 to i64, !dbg !38159
  %298 = add i64 %ring_cursor.sroa.0.1.i14487349, %_43.i.i1599.5, !dbg !38214
  %_47.not.i.i1600.5 = icmp ult i64 %298, %_91.i1472, !dbg !38215
  %299 = select i1 %_47.not.i.i1600.5, i64 0, i64 %_91.i1472, !dbg !38215
  %spec.select.i.i1601.5 = sub nuw i64 %298, %299, !dbg !38215
  %_51.i.i1602.5 = mul i64 %spec.select.i.i1601.5, %width.i.i1566, !dbg !38216
  %_50.i.i1603.5 = add i64 %_51.i.i1602.5, 5, !dbg !38216
  %_53.i.i1605.5 = icmp ult i64 %_50.i.i1603.5, %_149.1.i.i1615.pre, !dbg !38217
  br i1 %_53.i.i1605.5, label %bb21.i.i1607.5, label %panic1.i.i1606, !dbg !38217

bb21.i.i1607.5:                                   ; preds = %bb17.i.i1596.5
  %300 = getelementptr inbounds nuw float, ptr %_147.0.i.i1608, i64 %_50.i.i1603.5, !dbg !38217
  %_49.i.i1609.5 = load float, ptr %300, align 4, !dbg !38217, !noalias !38147, !noundef !12
  store float %_49.i.i1609.5, ptr %iter.i.i1009.sroa.0.0.ptr7346.5, align 4, !dbg !38218, !noalias !38147
  %301 = icmp eq i64 %width.i.i1566, 6, !dbg !38156
  br i1 %301, label %bb16.i.i1610, label %bb39.i.i1590.6, !dbg !38156

bb39.i.i1590.6:                                   ; preds = %bb21.i.i1607.5
  %exitcond11757.6.not = icmp eq i64 %_145.1.i.i1593, 6, !dbg !38159
  br i1 %exitcond11757.6.not, label %panic.i.i1595, label %bb17.i.i1596.6, !dbg !38159

bb17.i.i1596.6:                                   ; preds = %bb39.i.i1590.6
  %302 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1597, i64 80, !dbg !38159
  %_44.i.i1598.6 = load i32, ptr %302, align 4, !dbg !38159, !noalias !38147, !noundef !12
  %_43.i.i1599.6 = zext i32 %_44.i.i1598.6 to i64, !dbg !38159
  %303 = add i64 %ring_cursor.sroa.0.1.i14487349, %_43.i.i1599.6, !dbg !38214
  %_47.not.i.i1600.6 = icmp ult i64 %303, %_91.i1472, !dbg !38215
  %304 = select i1 %_47.not.i.i1600.6, i64 0, i64 %_91.i1472, !dbg !38215
  %spec.select.i.i1601.6 = sub nuw i64 %303, %304, !dbg !38215
  %_51.i.i1602.6 = mul i64 %spec.select.i.i1601.6, %width.i.i1566, !dbg !38216
  %_50.i.i1603.6 = add i64 %_51.i.i1602.6, 6, !dbg !38216
  %_53.i.i1605.6 = icmp ult i64 %_50.i.i1603.6, %_149.1.i.i1615.pre, !dbg !38217
  br i1 %_53.i.i1605.6, label %bb21.i.i1607.6, label %panic1.i.i1606, !dbg !38217

bb21.i.i1607.6:                                   ; preds = %bb17.i.i1596.6
  %305 = getelementptr inbounds nuw float, ptr %_147.0.i.i1608, i64 %_50.i.i1603.6, !dbg !38217
  %_49.i.i1609.6 = load float, ptr %305, align 4, !dbg !38217, !noalias !38147, !noundef !12
  store float %_49.i.i1609.6, ptr %iter.i.i1009.sroa.0.0.ptr7346.6, align 4, !dbg !38218, !noalias !38147
  %306 = icmp eq i64 %width.i.i1566, 7, !dbg !38156
  br i1 %306, label %bb16.i.i1610, label %bb39.i.i1590.7, !dbg !38156

bb39.i.i1590.7:                                   ; preds = %bb21.i.i1607.6
  %exitcond11757.7.not = icmp eq i64 %_145.1.i.i1593, 7, !dbg !38159
  br i1 %exitcond11757.7.not, label %panic.i.i1595, label %bb17.i.i1596.7, !dbg !38159

bb17.i.i1596.7:                                   ; preds = %bb39.i.i1590.7
  %307 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1597, i64 92, !dbg !38159
  %_44.i.i1598.7 = load i32, ptr %307, align 4, !dbg !38159, !noalias !38147, !noundef !12
  %_43.i.i1599.7 = zext i32 %_44.i.i1598.7 to i64, !dbg !38159
  %308 = add i64 %ring_cursor.sroa.0.1.i14487349, %_43.i.i1599.7, !dbg !38214
  %_47.not.i.i1600.7 = icmp ult i64 %308, %_91.i1472, !dbg !38215
  %309 = select i1 %_47.not.i.i1600.7, i64 0, i64 %_91.i1472, !dbg !38215
  %spec.select.i.i1601.7 = sub nuw i64 %308, %309, !dbg !38215
  %_51.i.i1602.7 = mul i64 %spec.select.i.i1601.7, %width.i.i1566, !dbg !38216
  %_50.i.i1603.7 = add i64 %_51.i.i1602.7, 7, !dbg !38216
  %_53.i.i1605.7 = icmp ult i64 %_50.i.i1603.7, %_149.1.i.i1615.pre, !dbg !38217
  br i1 %_53.i.i1605.7, label %bb21.i.i1607.7, label %panic1.i.i1606, !dbg !38217

bb21.i.i1607.7:                                   ; preds = %bb17.i.i1596.7
  %310 = getelementptr inbounds nuw float, ptr %_147.0.i.i1608, i64 %_50.i.i1603.7, !dbg !38217
  %_49.i.i1609.7 = load float, ptr %310, align 4, !dbg !38217, !noalias !38147, !noundef !12
  store float %_49.i.i1609.7, ptr %iter.i.i1009.sroa.0.0.ptr7346.7, align 4, !dbg !38218, !noalias !38147
  br label %bb16.i.i1610, !dbg !38156

panic1.i.i1606:                                   ; preds = %bb17.i.i1596.7, %bb17.i.i1596.6, %bb17.i.i1596.5, %bb17.i.i1596.4, %bb17.i.i1596.3, %bb17.i.i1596.2, %bb17.i.i1596.1, %bb17.i.i1596
  %_50.i.i1603.lcssa.ph = phi i64 [ %_50.i.i1603.7, %bb17.i.i1596.7 ], [ %_50.i.i1603.6, %bb17.i.i1596.6 ], [ %_50.i.i1603.5, %bb17.i.i1596.5 ], [ %_50.i.i1603.4, %bb17.i.i1596.4 ], [ %_50.i.i1603.3, %bb17.i.i1596.3 ], [ %_50.i.i1603.2, %bb17.i.i1596.2 ], [ %_50.i.i1603.1, %bb17.i.i1596.1 ], [ %_51.i.i1602, %bb17.i.i1596 ]
  store float %_0.i3390, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i.i1603.lcssa.ph, i64 noundef %_149.1.i.i1615.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b305c1483509cfb31fdec21ff8752674) #30, !dbg !38217, !noalias !38147
  unreachable, !dbg !38217

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit: ; preds = %bb49.i.i1637
  %_127.i.i1640 = getelementptr inbounds nuw float, ptr %_150.0.i.i1638, i64 %_76.i.i1635, !dbg !38219
  %_0.i = load float, ptr %_127.i.i1640, align 4, !dbg !38210, !alias.scope !38206, !noalias !38147, !noundef !12
  store float %_0.i3471, ptr %_127.i.i1640, align 4, !dbg !38221, !alias.scope !38223, !noalias !38147
  %_0.i2967 = fmul float %_0.i3383, %_0.i, !dbg !38226
  %_6.i3891 = bitcast float %_0.i to i32, !dbg !38228
  %_5.i3892 = and i32 %_6.i3891, %all.sroa.0.0.i1024, !dbg !38231
  %_8.i3893 = bitcast float %_0.i2967 to i32, !dbg !38232
  %_7.i3894 = and i32 %_9.i3906, %_8.i3893, !dbg !38234
  %_4.i3895 = or disjoint i32 %_7.i3894, %_5.i3892, !dbg !38231
  store i32 %_4.i3895, ptr %_181.i1564, align 4, !dbg !38235, !alias.scope !38237, !noalias !38240
  %311 = add i64 %main_cursor.sroa.0.1.i14497350, 1, !dbg !38241
  %_104.i1655 = icmp eq i64 %311, %_106.i1654, !dbg !38242
  %spec.store.select.i1656 = select i1 %_104.i1655, i64 0, i64 %311, !dbg !38242
  %312 = add i64 %ring_cursor.sroa.0.1.i14487349, 1, !dbg !38243
  %_107.i1657 = icmp eq i64 %312, %_91.i1472, !dbg !38244
  %spec.store.select13.i1658 = select i1 %_107.i1657, i64 0, i64 %312, !dbg !38244
  %exitcond11760.not = icmp eq i64 %182, %umax11759, !dbg !38245
  br i1 %exitcond11760.not, label %bb16.i1446.bb13.i1034.loopexit_crit_edge, label %bb48.i1451, !dbg !37107

bb54.i1664:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3637
  store float %_0.i3390, ptr %145, align 1, !dbg !37041
  store float %_0.i33868189, ptr %161, align 1, !dbg !37064
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_64.i1452, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_54f0ebc64763f3f022885a8e201aab88) #30, !dbg !38248, !noalias !37086
  unreachable, !dbg !38248

_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit.loopexit: ; preds = %bb13.i1034.loopexit
  %313 = trunc i64 %main_cursor.sroa.0.1.i1449.lcssa to i32, !dbg !38249
  %314 = trunc i64 %ring_cursor.sroa.0.1.i1448.lcssa to i32, !dbg !38250
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit, !dbg !38251

_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit.loopexit, %bb12.i
  %ring_cursor.sroa.0.0.i1037.lcssa = phi i32 [ %_36.i1026, %bb12.i ], [ %314, %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit.loopexit ], !dbg !37014
  %main_cursor.sroa.0.0.i1038.lcssa = phi i32 [ %_34.i1025, %bb12.i ], [ %313, %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit.loopexit ], !dbg !37011
  call void @llvm.lifetime.start.p0(ptr nonnull %_110.i1011), !dbg !38251, !noalias !36994
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(92) %_110.i1011, ptr noundef nonnull align 4 dereferenceable(92) %hot_left.i1017, i64 92, i1 false), !dbg !38251, !noalias !36994
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_110.i1011, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33) #31, !dbg !38252, !noalias !37086
  call void @llvm.lifetime.end.p0(ptr nonnull %_110.i1011), !dbg !38253, !noalias !36994
  call void @llvm.lifetime.start.p0(ptr nonnull %_112.i1010), !dbg !38254, !noalias !36994
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(92) %_112.i1010, ptr noundef nonnull align 4 dereferenceable(92) %hot_right.i1016, i64 92, i1 false), !dbg !38254, !noalias !36994
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_112.i1010, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34) #31, !dbg !38255, !noalias !37086
  call void @llvm.lifetime.end.p0(ptr nonnull %_112.i1010), !dbg !38256, !noalias !36994
  store i32 %main_cursor.sroa.0.0.i1038.lcssa, ptr %_35, align 4, !dbg !38249, !alias.scope !36988, !noalias !37013
  store i32 %ring_cursor.sroa.0.0.i1037.lcssa, ptr %83, align 4, !dbg !38250, !alias.scope !36988, !noalias !37013
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i1013), !dbg !38257, !noalias !36994
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i1014), !dbg !38258, !noalias !36994
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i1015), !dbg !38259, !noalias !36994
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_right.i1016), !dbg !38260, !noalias !36994
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i1017), !dbg !38261, !noalias !36994
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockfEB2_.exit, !dbg !36983

bb11.i:                                           ; preds = %bb10.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38262), !dbg !38265
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38266), !dbg !38265
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38268), !dbg !38265
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38270), !dbg !38265
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_left.i732, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #31, !dbg !38272
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_right.i731, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #31, !dbg !38276
  %315 = getelementptr inbounds nuw i8, ptr %self, i64 776, !dbg !38278
  %316 = load i8, ptr %315, align 4, !dbg !38278, !range !17, !alias.scope !38262, !noalias !38282, !noundef !12
  %317 = getelementptr inbounds nuw i8, ptr %self, i64 777, !dbg !38286
  %318 = load i8, ptr %317, align 1, !dbg !38286, !range !17, !alias.scope !38262, !noalias !38282, !noundef !12
  %_34.i = load i32, ptr %_35, align 4, !dbg !38288, !alias.scope !38270, !noalias !38290, !noundef !12
  %319 = getelementptr inbounds nuw i8, ptr %self, i64 564, !dbg !38291
  %_36.i740 = load i32, ptr %319, align 4, !dbg !38291, !alias.scope !38270, !noalias !38290, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i), !dbg !38293, !noalias !38295
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i, i8 0, i64 32, i1 false), !noalias !38295
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i730), !dbg !38296, !noalias !38295
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i730, i8 0, i64 1024, i1 false), !noalias !38295
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i729), !dbg !38298, !noalias !38295
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i729, i8 0, i64 1024, i1 false), !noalias !38295
  %_31.i736 = zext nneg i8 %316 to i32, !dbg !38278
  %.none.i737 = sub nsw i32 0, %_31.i736, !dbg !38300
  %_32.i738 = zext nneg i8 %318 to i32, !dbg !38286
  %all.sroa.0.0.i739 = sub nsw i32 0, %_32.i738, !dbg !38286
  %_116.not.i8423 = icmp eq i64 %frames, 0, !dbg !38301
  br i1 %_116.not.i8423, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit, label %bb37.i.lr.ph, !dbg !38301

bb37.i.lr.ph:                                     ; preds = %bb11.i
  %320 = zext i32 %_36.i740 to i64, !dbg !38291
  %321 = zext i32 %_34.i to i64, !dbg !38288
  %d9.i.i4655 = lshr i64 %frames, 5, !dbg !38311
  %r2.i.i4656 = and i64 %frames, 31, !dbg !38317
  %_19.not.i.i4657 = icmp ne i64 %r2.i.i4656, 0, !dbg !38318
  %322 = zext i1 %_19.not.i.i4657 to i64, !dbg !38318
  %yield_count.sroa.0.0.i.i4658 = add nuw nsw i64 %d9.i.i4655, %322, !dbg !38318
  %history.i141.i.sroa.7.0.hot_left.i732.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i732, i64 4
  %history.i141.i.sroa.10.0.hot_left.i732.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i732, i64 8
  %history.i141.i.sroa.13.0.hot_left.i732.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i732, i64 12
  %history.i141.i.sroa.16.0.hot_left.i732.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i732, i64 16
  %history.i141.i.sroa.19.0.hot_left.i732.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i732, i64 20
  %history.i141.i.sroa.22.0.hot_left.i732.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i732, i64 24
  %history.i141.i.sroa.26.0.hot_left.i732.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i732, i64 28
  %history.i141.i.sroa.29.0.hot_left.i732.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i732, i64 32
  %history.i141.i.sroa.32.0.hot_left.i732.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i732, i64 36
  %history.i141.i.sroa.35.0.hot_left.i732.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i732, i64 40
  %history.i141.i.sroa.38.0.hot_left.i732.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i732, i64 44
  %323 = getelementptr inbounds nuw i8, ptr %self, i64 588
  %324 = getelementptr inbounds nuw i8, ptr %self, i64 592
  %325 = getelementptr inbounds nuw i8, ptr %self, i64 596
  %row1.i.i.i174.i = getelementptr inbounds nuw i8, ptr %self, i64 600
  %326 = getelementptr inbounds nuw i8, ptr %self, i64 604
  %327 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %328 = getelementptr inbounds nuw i8, ptr %self, i64 612
  %row3.i.i.i188.i = getelementptr inbounds nuw i8, ptr %self, i64 616
  %329 = getelementptr inbounds nuw i8, ptr %self, i64 620
  %330 = getelementptr inbounds nuw i8, ptr %self, i64 624
  %331 = getelementptr inbounds nuw i8, ptr %self, i64 628
  %row5.i.i.i202.i = getelementptr inbounds nuw i8, ptr %self, i64 632
  %332 = getelementptr inbounds nuw i8, ptr %self, i64 636
  %333 = getelementptr inbounds nuw i8, ptr %self, i64 640
  %334 = getelementptr inbounds nuw i8, ptr %self, i64 644
  %row7.i.i.i216.i = getelementptr inbounds nuw i8, ptr %self, i64 648
  %335 = getelementptr inbounds nuw i8, ptr %self, i64 652
  %336 = getelementptr inbounds nuw i8, ptr %self, i64 656
  %337 = getelementptr inbounds nuw i8, ptr %self, i64 660
  %row9.i.i.i230.i = getelementptr inbounds nuw i8, ptr %self, i64 664
  %338 = getelementptr inbounds nuw i8, ptr %self, i64 668
  %339 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %340 = getelementptr inbounds nuw i8, ptr %self, i64 676
  %row11.i.i.i244.i = getelementptr inbounds nuw i8, ptr %self, i64 680
  %341 = getelementptr inbounds nuw i8, ptr %self, i64 684
  %342 = getelementptr inbounds nuw i8, ptr %self, i64 688
  %343 = getelementptr inbounds nuw i8, ptr %self, i64 692
  %row13.i.i.i258.i = getelementptr inbounds nuw i8, ptr %self, i64 696
  %344 = getelementptr inbounds nuw i8, ptr %self, i64 700
  %345 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %346 = getelementptr inbounds nuw i8, ptr %self, i64 708
  %row15.i.i.i272.i = getelementptr inbounds nuw i8, ptr %self, i64 712
  %347 = getelementptr inbounds nuw i8, ptr %self, i64 716
  %348 = getelementptr inbounds nuw i8, ptr %self, i64 720
  %349 = getelementptr inbounds nuw i8, ptr %self, i64 724
  %row17.i.i.i286.i = getelementptr inbounds nuw i8, ptr %self, i64 728
  %350 = getelementptr inbounds nuw i8, ptr %self, i64 732
  %351 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %352 = getelementptr inbounds nuw i8, ptr %self, i64 740
  %row19.i.i.i300.i = getelementptr inbounds nuw i8, ptr %self, i64 744
  %353 = getelementptr inbounds nuw i8, ptr %self, i64 748
  %354 = getelementptr inbounds nuw i8, ptr %self, i64 752
  %355 = getelementptr inbounds nuw i8, ptr %self, i64 756
  %row21.i.i.i314.i = getelementptr inbounds nuw i8, ptr %self, i64 760
  %356 = getelementptr inbounds nuw i8, ptr %self, i64 764
  %357 = getelementptr inbounds nuw i8, ptr %self, i64 768
  %358 = getelementptr inbounds nuw i8, ptr %self, i64 772
  %history.i.i727.sroa.7.0.hot_right.i731.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i731, i64 4
  %history.i.i727.sroa.10.0.hot_right.i731.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i731, i64 8
  %history.i.i727.sroa.13.0.hot_right.i731.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i731, i64 12
  %history.i.i727.sroa.16.0.hot_right.i731.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i731, i64 16
  %history.i.i727.sroa.19.0.hot_right.i731.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i731, i64 20
  %history.i.i727.sroa.22.0.hot_right.i731.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i731, i64 24
  %history.i.i727.sroa.26.0.hot_right.i731.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i731, i64 28
  %history.i.i727.sroa.29.0.hot_right.i731.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i731, i64 32
  %history.i.i727.sroa.32.0.hot_right.i731.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i731, i64 36
  %history.i.i727.sroa.35.0.hot_right.i731.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i731, i64 40
  %history.i.i727.sroa.38.0.hot_right.i731.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i731, i64 44
  %_68.i948 = getelementptr inbounds nuw i8, ptr %hot_left.i732, i64 48
  %_69.i = getelementptr inbounds nuw i8, ptr %hot_left.i732, i64 64
  %_73.i = getelementptr inbounds nuw i8, ptr %hot_right.i731, i64 48
  %_74.i949 = getelementptr inbounds nuw i8, ptr %hot_right.i731, i64 64
  %_9.i3966 = add nsw i32 %_31.i736, -1
  %359 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %360 = getelementptr inbounds nuw i8, ptr %self, i64 328
  %361 = getelementptr inbounds nuw i8, ptr %self, i64 176
  %362 = getelementptr inbounds nuw i8, ptr %self, i64 168
  %363 = getelementptr inbounds nuw i8, ptr %self, i64 256
  %364 = getelementptr inbounds nuw i8, ptr %self, i64 248
  %365 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %366 = getelementptr inbounds nuw i8, ptr %self, i64 216
  %367 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %368 = getelementptr inbounds nuw i8, ptr %self, i64 184
  %369 = getelementptr inbounds nuw i8, ptr %hot_left.i732, i64 84
  %370 = getelementptr inbounds nuw i8, ptr %hot_left.i732, i64 88
  %371 = getelementptr inbounds nuw i8, ptr %hot_left.i732, i64 80
  %372 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %373 = getelementptr inbounds nuw i8, ptr %self, i64 152
  %_9.i3946 = add nsw i32 %_32.i738, -1
  %374 = getelementptr inbounds nuw i8, ptr %self, i64 528
  %375 = getelementptr inbounds nuw i8, ptr %self, i64 376
  %376 = getelementptr inbounds nuw i8, ptr %self, i64 368
  %377 = getelementptr inbounds nuw i8, ptr %self, i64 520
  %378 = getelementptr inbounds nuw i8, ptr %self, i64 512
  %379 = getelementptr inbounds nuw i8, ptr %self, i64 456
  %380 = getelementptr inbounds nuw i8, ptr %self, i64 448
  %381 = getelementptr inbounds nuw i8, ptr %self, i64 424
  %382 = getelementptr inbounds nuw i8, ptr %self, i64 416
  %383 = getelementptr inbounds nuw i8, ptr %self, i64 392
  %384 = getelementptr inbounds nuw i8, ptr %self, i64 384
  %385 = getelementptr inbounds nuw i8, ptr %hot_right.i731, i64 84
  %386 = getelementptr inbounds nuw i8, ptr %hot_right.i731, i64 88
  %387 = getelementptr inbounds nuw i8, ptr %hot_right.i731, i64 80
  %388 = getelementptr inbounds nuw i8, ptr %self, i64 360
  %389 = getelementptr inbounds nuw i8, ptr %self, i64 352
  %390 = getelementptr inbounds nuw i8, ptr %self, i64 552
  %iter.i32.i.sroa.0.0.ptr8263.1 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 4
  %iter.i32.i.sroa.0.0.ptr8263.2 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 8
  %iter.i32.i.sroa.0.0.ptr8263.3 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 12
  %iter.i32.i.sroa.0.0.ptr8263.4 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 16
  %iter.i32.i.sroa.0.0.ptr8263.5 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 20
  %iter.i32.i.sroa.0.0.ptr8263.6 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 24
  %iter.i32.i.sroa.0.0.ptr8263.7 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 28
  %iter.i.i728.sroa.0.0.ptr8274.1 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 4
  %iter.i.i728.sroa.0.0.ptr8274.2 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 8
  %iter.i.i728.sroa.0.0.ptr8274.3 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 12
  %iter.i.i728.sroa.0.0.ptr8274.4 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 16
  %iter.i.i728.sroa.0.0.ptr8274.5 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 20
  %iter.i.i728.sroa.0.0.ptr8274.6 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 24
  %iter.i.i728.sroa.0.0.ptr8274.7 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 28
  br label %bb37.i, !dbg !38301

bb16.i.bb13.i.loopexit_crit_edge:                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3661
  store float %_0.i3772, ptr %371, align 1, !dbg !38319
  store float %_0.i3768, ptr %387, align 1, !dbg !38335
  store float %_0.i3398, ptr %369, align 4, !dbg !38337
  store float %_0.i3394, ptr %385, align 4, !dbg !38338
  br label %bb13.i.loopexit, !dbg !38339

bb13.i.loopexit:                                  ; preds = %bb16.i.bb13.i.loopexit_crit_edge, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i944
  %ring_cursor.sroa.0.1.i946.lcssa = phi i64 [ %spec.store.select13.i, %bb16.i.bb13.i.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i7468426, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i944 ], !dbg !38345
  %main_cursor.sroa.0.1.i947.lcssa = phi i64 [ %spec.store.select.i, %bb16.i.bb13.i.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i7478427, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i944 ], !dbg !38346
  %_116.not.i = icmp eq i64 %393, 0, !dbg !38301
  %indvars.iv.next11762 = add i64 %indvars.iv11761, -32, !dbg !38301
  br i1 %_116.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit.loopexit, label %bb37.i, !dbg !38301

bb37.i:                                           ; preds = %bb37.i.lr.ph, %bb13.i.loopexit
  %indvars.iv11761 = phi i64 [ %frames, %bb37.i.lr.ph ], [ %indvars.iv.next11762, %bb13.i.loopexit ]
  %main_cursor.sroa.0.0.i7478427 = phi i64 [ %321, %bb37.i.lr.ph ], [ %main_cursor.sroa.0.1.i947.lcssa, %bb13.i.loopexit ]
  %ring_cursor.sroa.0.0.i7468426 = phi i64 [ %320, %bb37.i.lr.ph ], [ %ring_cursor.sroa.0.1.i946.lcssa, %bb13.i.loopexit ]
  %iter2.sroa.0.0.i7458425 = phi i64 [ %yield_count.sroa.0.0.i.i4658, %bb37.i.lr.ph ], [ %393, %bb13.i.loopexit ]
  %iter.sroa.0.0.i8424 = phi i64 [ 0, %bb37.i.lr.ph ], [ %392, %bb13.i.loopexit ]
  %391 = call i64 @llvm.umax.i64(i64 %indvars.iv11761, i64 1), !dbg !38347
  %umax11782 = call i64 @llvm.umin.i64(i64 %391, i64 32), !dbg !38347
  %392 = add i64 %iter.sroa.0.0.i8424, 32, !dbg !38347
  %393 = add i64 %iter2.sroa.0.0.i7458425, -1, !dbg !38351
  %_45.i = sub i64 %frames, %iter.sroa.0.0.i8424, !dbg !38352
  %..i4659 = tail call noundef i64 @llvm.umin.i64(i64 %_45.i, i64 32), !dbg !38353
  %_51.i = add i64 %..i4659, %iter.sroa.0.0.i8424, !dbg !38357
  %_128.i = icmp ult i64 %_51.i, %iter.sroa.0.0.i8424, !dbg !38358
  %_122.not.i = icmp ugt i64 %_51.i, %left_io.1
  %or.cond.i750 = or i1 %_128.i, %_122.not.i, !dbg !38358
  br i1 %or.cond.i750, label %bb41.i, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4685, !dbg !38358, !prof !165

bb41.i:                                           ; preds = %bb37.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %iter.sroa.0.0.i8424, i64 noundef %_51.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5cbcacccdbb7b907c55dbc42154fcf08) #30, !dbg !38365, !noalias !38366
  unreachable, !dbg !38365

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4685: ; preds = %bb37.i
  %_131.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %iter.sroa.0.0.i8424, !dbg !38367
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38371), !dbg !38374
  %history.i141.i.sroa.0.0.copyload = load float, ptr %hot_left.i732, align 4, !dbg !38375
  %history.i141.i.sroa.7.0.copyload = load float, ptr %history.i141.i.sroa.7.0.hot_left.i732.sroa_idx, align 4, !dbg !38375
  %history.i141.i.sroa.10.0.copyload = load float, ptr %history.i141.i.sroa.10.0.hot_left.i732.sroa_idx, align 4, !dbg !38375
  %history.i141.i.sroa.13.0.copyload = load float, ptr %history.i141.i.sroa.13.0.hot_left.i732.sroa_idx, align 4, !dbg !38375
  %history.i141.i.sroa.16.0.copyload = load float, ptr %history.i141.i.sroa.16.0.hot_left.i732.sroa_idx, align 4, !dbg !38375
  %history.i141.i.sroa.19.0.copyload = load float, ptr %history.i141.i.sroa.19.0.hot_left.i732.sroa_idx, align 4, !dbg !38375
  %history.i141.i.sroa.22.0.copyload = load float, ptr %history.i141.i.sroa.22.0.hot_left.i732.sroa_idx, align 4, !dbg !38375
  %history.i141.i.sroa.26.0.copyload = load float, ptr %history.i141.i.sroa.26.0.hot_left.i732.sroa_idx, align 4, !dbg !38375
  %history.i141.i.sroa.29.0.copyload = load float, ptr %history.i141.i.sroa.29.0.hot_left.i732.sroa_idx, align 4, !dbg !38375
  %history.i141.i.sroa.32.0.copyload = load float, ptr %history.i141.i.sroa.32.0.hot_left.i732.sroa_idx, align 4, !dbg !38375
  %history.i141.i.sroa.35.0.copyload = load float, ptr %history.i141.i.sroa.35.0.hot_left.i732.sroa_idx, align 4, !dbg !38375
  %history.i141.i.sroa.38.0.copyload = load float, ptr %history.i141.i.sroa.38.0.hot_left.i732.sroa_idx, align 4, !dbg !38375
  %_2.i46888201.not = icmp eq i64 %frames, %iter.sroa.0.0.i8424, !dbg !38377
  br i1 %_2.i46888201.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit336.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555.lr.ph, !dbg !38377

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555.lr.ph: ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4685
  %_11.i.i.i162.i = load float, ptr %_31, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_14.i.i.i165.i = load float, ptr %323, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_17.i.i.i168.i = load float, ptr %324, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_20.i.i.i171.i = load float, ptr %325, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_25.i.i.i176.i = load float, ptr %row1.i.i.i174.i, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_28.i.i.i179.i = load float, ptr %326, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_31.i.i.i182.i = load float, ptr %327, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_34.i.i.i185.i = load float, ptr %328, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_39.i.i.i190.i = load float, ptr %row3.i.i.i188.i, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_42.i.i.i193.i = load float, ptr %329, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_45.i.i.i196.i = load float, ptr %330, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_48.i.i.i199.i = load float, ptr %331, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_53.i.i.i204.i = load float, ptr %row5.i.i.i202.i, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_56.i.i.i207.i = load float, ptr %332, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_59.i.i.i210.i = load float, ptr %333, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_62.i.i.i213.i = load float, ptr %334, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_67.i.i.i218.i = load float, ptr %row7.i.i.i216.i, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_70.i.i.i221.i = load float, ptr %335, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_73.i.i.i224.i = load float, ptr %336, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_76.i.i.i227.i = load float, ptr %337, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_81.i.i.i232.i = load float, ptr %row9.i.i.i230.i, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_84.i.i.i235.i = load float, ptr %338, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_87.i.i.i238.i = load float, ptr %339, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_90.i.i.i241.i = load float, ptr %340, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_95.i.i.i246.i = load float, ptr %row11.i.i.i244.i, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_98.i.i.i249.i = load float, ptr %341, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_101.i.i.i252.i = load float, ptr %342, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_104.i.i.i255.i = load float, ptr %343, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_109.i.i.i260.i = load float, ptr %row13.i.i.i258.i, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_112.i.i.i263.i = load float, ptr %344, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_115.i.i.i266.i = load float, ptr %345, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_118.i.i.i269.i = load float, ptr %346, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_123.i.i.i274.i = load float, ptr %row15.i.i.i272.i, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_126.i.i.i277.i = load float, ptr %347, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_129.i.i.i280.i = load float, ptr %348, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_132.i.i.i283.i = load float, ptr %349, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_137.i.i.i288.i = load float, ptr %row17.i.i.i286.i, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_140.i.i.i291.i = load float, ptr %350, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_143.i.i.i294.i = load float, ptr %351, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_146.i.i.i297.i = load float, ptr %352, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_151.i.i.i302.i = load float, ptr %row19.i.i.i300.i, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_154.i.i.i305.i = load float, ptr %353, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_157.i.i.i308.i = load float, ptr %354, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_160.i.i.i311.i = load float, ptr %355, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_165.i.i.i316.i = load float, ptr %row21.i.i.i314.i, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_168.i.i.i319.i = load float, ptr %356, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_171.i.i.i322.i = load float, ptr %357, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  %_174.i.i.i325.i = load float, ptr %358, align 4, !alias.scope !38380, !noalias !38385, !noundef !12
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555, !dbg !38377

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555
  %iter.i137.i.sroa.16.08213 = phi i64 [ 0, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555.lr.ph ], [ %399, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555 ]
  %history.i141.i.sroa.35.08212 = phi float [ %history.i141.i.sroa.35.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555.lr.ph ], [ %history.i141.i.sroa.32.08211, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555 ]
  %history.i141.i.sroa.32.08211 = phi float [ %history.i141.i.sroa.32.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555.lr.ph ], [ %history.i141.i.sroa.29.08210, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555 ]
  %history.i141.i.sroa.29.08210 = phi float [ %history.i141.i.sroa.29.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555.lr.ph ], [ %history.i141.i.sroa.26.08209, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555 ]
  %history.i141.i.sroa.26.08209 = phi float [ %history.i141.i.sroa.26.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555.lr.ph ], [ %history.i141.i.sroa.22.08208, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555 ]
  %history.i141.i.sroa.22.08208 = phi float [ %history.i141.i.sroa.22.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555.lr.ph ], [ %history.i141.i.sroa.19.08207, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555 ]
  %history.i141.i.sroa.19.08207 = phi float [ %history.i141.i.sroa.19.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555.lr.ph ], [ %history.i141.i.sroa.16.08206, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555 ]
  %history.i141.i.sroa.16.08206 = phi float [ %history.i141.i.sroa.16.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555.lr.ph ], [ %history.i141.i.sroa.13.08205, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555 ]
  %history.i141.i.sroa.13.08205 = phi float [ %history.i141.i.sroa.13.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555.lr.ph ], [ %history.i141.i.sroa.10.08204, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555 ]
  %history.i141.i.sroa.10.08204 = phi float [ %history.i141.i.sroa.10.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555.lr.ph ], [ %history.i141.i.sroa.7.08203, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555 ]
  %history.i141.i.sroa.7.08203 = phi float [ %history.i141.i.sroa.7.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555.lr.ph ], [ %history.i141.i.sroa.0.08202, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555 ]
  %history.i141.i.sroa.0.08202 = phi float [ %history.i141.i.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555.lr.ph ], [ %_0.i3553, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555 ]
  %data.i.i4695 = getelementptr inbounds nuw float, ptr %_131.i, i64 %iter.i137.i.sroa.16.08213, !dbg !38392
  %_0.i3553 = load float, ptr %data.i.i4695, align 4, !dbg !38395, !alias.scope !38397, !noalias !38400, !noundef !12
  %394 = tail call noundef float @llvm.fabs.f32(float %history.i141.i.sroa.19.08207), !dbg !38401
  %_0.i3170 = fmul float %_0.i3553, %_11.i.i.i162.i, !dbg !38404
  %_0.i2738 = fadd float %_0.i3170, 0.000000e+00, !dbg !38407
  %_0.i3169 = fmul float %_0.i3553, %_14.i.i.i165.i, !dbg !38409
  %_0.i2737 = fadd float %_0.i3169, 0.000000e+00, !dbg !38411
  %_0.i3168 = fmul float %_0.i3553, %_17.i.i.i168.i, !dbg !38413
  %_0.i2736 = fadd float %_0.i3168, 0.000000e+00, !dbg !38415
  %_0.i3167 = fmul float %_0.i3553, %_20.i.i.i171.i, !dbg !38417
  %_0.i2735 = fadd float %_0.i3167, 0.000000e+00, !dbg !38419
  %_0.i3166 = fmul float %history.i141.i.sroa.0.08202, %_25.i.i.i176.i, !dbg !38421
  %_0.i2734 = fadd float %_0.i2738, %_0.i3166, !dbg !38423
  %_0.i3165 = fmul float %history.i141.i.sroa.0.08202, %_28.i.i.i179.i, !dbg !38425
  %_0.i2733 = fadd float %_0.i2737, %_0.i3165, !dbg !38427
  %_0.i3164 = fmul float %history.i141.i.sroa.0.08202, %_31.i.i.i182.i, !dbg !38429
  %_0.i2732 = fadd float %_0.i2736, %_0.i3164, !dbg !38431
  %_0.i3163 = fmul float %history.i141.i.sroa.0.08202, %_34.i.i.i185.i, !dbg !38433
  %_0.i2731 = fadd float %_0.i2735, %_0.i3163, !dbg !38435
  %_0.i3162 = fmul float %history.i141.i.sroa.7.08203, %_39.i.i.i190.i, !dbg !38437
  %_0.i2730 = fadd float %_0.i2734, %_0.i3162, !dbg !38439
  %_0.i3161 = fmul float %history.i141.i.sroa.7.08203, %_42.i.i.i193.i, !dbg !38441
  %_0.i2729 = fadd float %_0.i2733, %_0.i3161, !dbg !38443
  %_0.i3160 = fmul float %history.i141.i.sroa.7.08203, %_45.i.i.i196.i, !dbg !38445
  %_0.i2728 = fadd float %_0.i2732, %_0.i3160, !dbg !38447
  %_0.i3159 = fmul float %history.i141.i.sroa.7.08203, %_48.i.i.i199.i, !dbg !38449
  %_0.i2727 = fadd float %_0.i2731, %_0.i3159, !dbg !38451
  %_0.i3158 = fmul float %history.i141.i.sroa.10.08204, %_53.i.i.i204.i, !dbg !38453
  %_0.i2726 = fadd float %_0.i2730, %_0.i3158, !dbg !38455
  %_0.i3157 = fmul float %history.i141.i.sroa.10.08204, %_56.i.i.i207.i, !dbg !38457
  %_0.i2725 = fadd float %_0.i2729, %_0.i3157, !dbg !38459
  %_0.i3156 = fmul float %history.i141.i.sroa.10.08204, %_59.i.i.i210.i, !dbg !38461
  %_0.i2724 = fadd float %_0.i2728, %_0.i3156, !dbg !38463
  %_0.i3155 = fmul float %history.i141.i.sroa.10.08204, %_62.i.i.i213.i, !dbg !38465
  %_0.i2723 = fadd float %_0.i2727, %_0.i3155, !dbg !38467
  %_0.i3154 = fmul float %history.i141.i.sroa.13.08205, %_67.i.i.i218.i, !dbg !38469
  %_0.i2722 = fadd float %_0.i2726, %_0.i3154, !dbg !38471
  %_0.i3153 = fmul float %history.i141.i.sroa.13.08205, %_70.i.i.i221.i, !dbg !38473
  %_0.i2721 = fadd float %_0.i2725, %_0.i3153, !dbg !38475
  %_0.i3152 = fmul float %history.i141.i.sroa.13.08205, %_73.i.i.i224.i, !dbg !38477
  %_0.i2720 = fadd float %_0.i2724, %_0.i3152, !dbg !38479
  %_0.i3151 = fmul float %history.i141.i.sroa.13.08205, %_76.i.i.i227.i, !dbg !38481
  %_0.i2719 = fadd float %_0.i2723, %_0.i3151, !dbg !38483
  %_0.i3150 = fmul float %history.i141.i.sroa.16.08206, %_81.i.i.i232.i, !dbg !38485
  %_0.i2718 = fadd float %_0.i2722, %_0.i3150, !dbg !38487
  %_0.i3149 = fmul float %history.i141.i.sroa.16.08206, %_84.i.i.i235.i, !dbg !38489
  %_0.i2717 = fadd float %_0.i2721, %_0.i3149, !dbg !38491
  %_0.i3148 = fmul float %history.i141.i.sroa.16.08206, %_87.i.i.i238.i, !dbg !38493
  %_0.i2716 = fadd float %_0.i2720, %_0.i3148, !dbg !38495
  %_0.i3147 = fmul float %history.i141.i.sroa.16.08206, %_90.i.i.i241.i, !dbg !38497
  %_0.i2715 = fadd float %_0.i2719, %_0.i3147, !dbg !38499
  %_0.i3146 = fmul float %history.i141.i.sroa.19.08207, %_95.i.i.i246.i, !dbg !38501
  %_0.i2714 = fadd float %_0.i2718, %_0.i3146, !dbg !38503
  %_0.i3145 = fmul float %history.i141.i.sroa.19.08207, %_98.i.i.i249.i, !dbg !38505
  %_0.i2713 = fadd float %_0.i2717, %_0.i3145, !dbg !38507
  %_0.i3144 = fmul float %history.i141.i.sroa.19.08207, %_101.i.i.i252.i, !dbg !38509
  %_0.i2712 = fadd float %_0.i2716, %_0.i3144, !dbg !38511
  %_0.i3143 = fmul float %history.i141.i.sroa.19.08207, %_104.i.i.i255.i, !dbg !38513
  %_0.i2711 = fadd float %_0.i2715, %_0.i3143, !dbg !38515
  %_0.i3142 = fmul float %history.i141.i.sroa.22.08208, %_109.i.i.i260.i, !dbg !38517
  %_0.i2710 = fadd float %_0.i2714, %_0.i3142, !dbg !38519
  %_0.i3141 = fmul float %history.i141.i.sroa.22.08208, %_112.i.i.i263.i, !dbg !38521
  %_0.i2709 = fadd float %_0.i2713, %_0.i3141, !dbg !38523
  %_0.i3140 = fmul float %history.i141.i.sroa.22.08208, %_115.i.i.i266.i, !dbg !38525
  %_0.i2708 = fadd float %_0.i2712, %_0.i3140, !dbg !38527
  %_0.i3139 = fmul float %history.i141.i.sroa.22.08208, %_118.i.i.i269.i, !dbg !38529
  %_0.i2707 = fadd float %_0.i2711, %_0.i3139, !dbg !38531
  %_0.i3138 = fmul float %history.i141.i.sroa.26.08209, %_123.i.i.i274.i, !dbg !38533
  %_0.i2706 = fadd float %_0.i2710, %_0.i3138, !dbg !38535
  %_0.i3137 = fmul float %history.i141.i.sroa.26.08209, %_126.i.i.i277.i, !dbg !38537
  %_0.i2705 = fadd float %_0.i2709, %_0.i3137, !dbg !38539
  %_0.i3136 = fmul float %history.i141.i.sroa.26.08209, %_129.i.i.i280.i, !dbg !38541
  %_0.i2704 = fadd float %_0.i2708, %_0.i3136, !dbg !38543
  %_0.i3135 = fmul float %history.i141.i.sroa.26.08209, %_132.i.i.i283.i, !dbg !38545
  %_0.i2703 = fadd float %_0.i2707, %_0.i3135, !dbg !38547
  %_0.i3134 = fmul float %history.i141.i.sroa.29.08210, %_137.i.i.i288.i, !dbg !38549
  %_0.i2702 = fadd float %_0.i2706, %_0.i3134, !dbg !38551
  %_0.i3133 = fmul float %history.i141.i.sroa.29.08210, %_140.i.i.i291.i, !dbg !38553
  %_0.i2701 = fadd float %_0.i2705, %_0.i3133, !dbg !38555
  %_0.i3132 = fmul float %history.i141.i.sroa.29.08210, %_143.i.i.i294.i, !dbg !38557
  %_0.i2700 = fadd float %_0.i2704, %_0.i3132, !dbg !38559
  %_0.i3131 = fmul float %history.i141.i.sroa.29.08210, %_146.i.i.i297.i, !dbg !38561
  %_0.i2699 = fadd float %_0.i2703, %_0.i3131, !dbg !38563
  %_0.i3130 = fmul float %history.i141.i.sroa.32.08211, %_151.i.i.i302.i, !dbg !38565
  %_0.i2698 = fadd float %_0.i2702, %_0.i3130, !dbg !38567
  %_0.i3129 = fmul float %history.i141.i.sroa.32.08211, %_154.i.i.i305.i, !dbg !38569
  %_0.i2697 = fadd float %_0.i2701, %_0.i3129, !dbg !38571
  %_0.i3128 = fmul float %history.i141.i.sroa.32.08211, %_157.i.i.i308.i, !dbg !38573
  %_0.i2696 = fadd float %_0.i2700, %_0.i3128, !dbg !38575
  %_0.i3127 = fmul float %history.i141.i.sroa.32.08211, %_160.i.i.i311.i, !dbg !38577
  %_0.i2695 = fadd float %_0.i2699, %_0.i3127, !dbg !38579
  %_0.i3126 = fmul float %history.i141.i.sroa.35.08212, %_165.i.i.i316.i, !dbg !38581
  %_0.i2694 = fadd float %_0.i2698, %_0.i3126, !dbg !38583
  %_0.i3125 = fmul float %history.i141.i.sroa.35.08212, %_168.i.i.i319.i, !dbg !38585
  %_0.i2693 = fadd float %_0.i2697, %_0.i3125, !dbg !38587
  %_0.i3124 = fmul float %history.i141.i.sroa.35.08212, %_171.i.i.i322.i, !dbg !38589
  %_0.i2692 = fadd float %_0.i2696, %_0.i3124, !dbg !38591
  %_0.i3123 = fmul float %history.i141.i.sroa.35.08212, %_174.i.i.i325.i, !dbg !38593
  %_0.i2691 = fadd float %_0.i2695, %_0.i3123, !dbg !38595
  %395 = tail call noundef float @llvm.fabs.f32(float %_0.i2694), !dbg !38597
  %_3.i.i4181.inv = fcmp ogt float %394, %395, !dbg !38599
  %_4.i.i4188.v = select i1 %_3.i.i4181.inv, float %394, float %395, !dbg !38599
  %396 = tail call noundef float @llvm.fabs.f32(float %_0.i2693), !dbg !38597
  %_3.i.i4181.inv.1 = fcmp ogt float %_4.i.i4188.v, %396, !dbg !38599
  %_4.i.i4188.v.1 = select i1 %_3.i.i4181.inv.1, float %_4.i.i4188.v, float %396, !dbg !38599
  %397 = tail call noundef float @llvm.fabs.f32(float %_0.i2692), !dbg !38597
  %_3.i.i4181.inv.2 = fcmp ogt float %_4.i.i4188.v.1, %397, !dbg !38599
  %_4.i.i4188.v.2 = select i1 %_3.i.i4181.inv.2, float %_4.i.i4188.v.1, float %397, !dbg !38599
  %398 = tail call noundef float @llvm.fabs.f32(float %_0.i2691), !dbg !38597
  %_3.i.i4181.inv.3 = fcmp ogt float %_4.i.i4188.v.2, %398, !dbg !38599
  %_4.i.i4188.v.3 = select i1 %_3.i.i4181.inv.3, float %_4.i.i4188.v.2, float %398, !dbg !38599
  %399 = add nuw nsw i64 %iter.i137.i.sroa.16.08213, 1, !dbg !38602
  %data.i4.i4699 = getelementptr inbounds nuw float, ptr %peaks_left.i730, i64 %iter.i137.i.sroa.16.08213, !dbg !38603
  store float %_4.i.i4188.v.3, ptr %data.i4.i4699, align 4, !dbg !38606, !alias.scope !38608, !noalias !38400
  %exitcond11765.not = icmp eq i64 %399, %umax11782, !dbg !38377
  br i1 %exitcond11765.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit336.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555, !dbg !38377

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit336.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4685
  %history.i141.i.sroa.0.0.lcssa = phi float [ %history.i141.i.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4685 ], [ %_0.i3553, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555 ], !dbg !38611
  %history.i141.i.sroa.7.0.lcssa = phi float [ %history.i141.i.sroa.7.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4685 ], [ %history.i141.i.sroa.0.08202, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555 ], !dbg !38611
  %history.i141.i.sroa.10.0.lcssa = phi float [ %history.i141.i.sroa.10.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4685 ], [ %history.i141.i.sroa.7.08203, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555 ], !dbg !38611
  %history.i141.i.sroa.13.0.lcssa = phi float [ %history.i141.i.sroa.13.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4685 ], [ %history.i141.i.sroa.10.08204, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555 ], !dbg !38611
  %history.i141.i.sroa.16.0.lcssa = phi float [ %history.i141.i.sroa.16.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4685 ], [ %history.i141.i.sroa.13.08205, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555 ], !dbg !38611
  %history.i141.i.sroa.19.0.lcssa = phi float [ %history.i141.i.sroa.19.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4685 ], [ %history.i141.i.sroa.16.08206, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555 ], !dbg !38611
  %history.i141.i.sroa.22.0.lcssa = phi float [ %history.i141.i.sroa.22.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4685 ], [ %history.i141.i.sroa.19.08207, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555 ], !dbg !38611
  %history.i141.i.sroa.26.0.lcssa = phi float [ %history.i141.i.sroa.26.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4685 ], [ %history.i141.i.sroa.22.08208, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555 ], !dbg !38611
  %history.i141.i.sroa.29.0.lcssa = phi float [ %history.i141.i.sroa.29.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4685 ], [ %history.i141.i.sroa.26.08209, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555 ], !dbg !38611
  %history.i141.i.sroa.32.0.lcssa = phi float [ %history.i141.i.sroa.32.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4685 ], [ %history.i141.i.sroa.29.08210, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555 ], !dbg !38611
  %history.i141.i.sroa.35.0.lcssa = phi float [ %history.i141.i.sroa.35.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4685 ], [ %history.i141.i.sroa.32.08211, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555 ], !dbg !38611
  %history.i141.i.sroa.38.0.lcssa = phi float [ %history.i141.i.sroa.38.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4685 ], [ %history.i141.i.sroa.35.08212, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3555 ], !dbg !38611
  store float %history.i141.i.sroa.0.0.lcssa, ptr %hot_left.i732, align 4, !dbg !38612
  store float %history.i141.i.sroa.7.0.lcssa, ptr %history.i141.i.sroa.7.0.hot_left.i732.sroa_idx, align 4, !dbg !38612
  store float %history.i141.i.sroa.10.0.lcssa, ptr %history.i141.i.sroa.10.0.hot_left.i732.sroa_idx, align 4, !dbg !38612
  store float %history.i141.i.sroa.13.0.lcssa, ptr %history.i141.i.sroa.13.0.hot_left.i732.sroa_idx, align 4, !dbg !38612
  store float %history.i141.i.sroa.16.0.lcssa, ptr %history.i141.i.sroa.16.0.hot_left.i732.sroa_idx, align 4, !dbg !38612
  store float %history.i141.i.sroa.19.0.lcssa, ptr %history.i141.i.sroa.19.0.hot_left.i732.sroa_idx, align 4, !dbg !38612
  store float %history.i141.i.sroa.22.0.lcssa, ptr %history.i141.i.sroa.22.0.hot_left.i732.sroa_idx, align 4, !dbg !38612
  store float %history.i141.i.sroa.26.0.lcssa, ptr %history.i141.i.sroa.26.0.hot_left.i732.sroa_idx, align 4, !dbg !38612
  store float %history.i141.i.sroa.29.0.lcssa, ptr %history.i141.i.sroa.29.0.hot_left.i732.sroa_idx, align 4, !dbg !38612
  store float %history.i141.i.sroa.32.0.lcssa, ptr %history.i141.i.sroa.32.0.hot_left.i732.sroa_idx, align 4, !dbg !38612
  store float %history.i141.i.sroa.35.0.lcssa, ptr %history.i141.i.sroa.35.0.hot_left.i732.sroa_idx, align 4, !dbg !38612
  store float %history.i141.i.sroa.38.0.lcssa, ptr %history.i141.i.sroa.38.0.hot_left.i732.sroa_idx, align 4, !dbg !38612
  %_139.not.i = icmp ugt i64 %_51.i, %right_io.1, !dbg !38613
  br i1 %_139.not.i, label %bb47.i, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4741, !dbg !38613, !prof !1406

bb47.i:                                           ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit336.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %iter.sroa.0.0.i8424, i64 noundef %_51.i, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f6bbc99b95dcf27c100d71299ef7abde) #30, !dbg !38617, !noalias !38366
  unreachable, !dbg !38617

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4741: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit336.i
  %_146.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %iter.sroa.0.0.i8424, !dbg !38618
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38622), !dbg !38625
  %history.i.i727.sroa.0.0.copyload = load float, ptr %hot_right.i731, align 4, !dbg !38626
  %history.i.i727.sroa.7.0.copyload = load float, ptr %history.i.i727.sroa.7.0.hot_right.i731.sroa_idx, align 4, !dbg !38626
  %history.i.i727.sroa.10.0.copyload = load float, ptr %history.i.i727.sroa.10.0.hot_right.i731.sroa_idx, align 4, !dbg !38626
  %history.i.i727.sroa.13.0.copyload = load float, ptr %history.i.i727.sroa.13.0.hot_right.i731.sroa_idx, align 4, !dbg !38626
  %history.i.i727.sroa.16.0.copyload = load float, ptr %history.i.i727.sroa.16.0.hot_right.i731.sroa_idx, align 4, !dbg !38626
  %history.i.i727.sroa.19.0.copyload = load float, ptr %history.i.i727.sroa.19.0.hot_right.i731.sroa_idx, align 4, !dbg !38626
  %history.i.i727.sroa.22.0.copyload = load float, ptr %history.i.i727.sroa.22.0.hot_right.i731.sroa_idx, align 4, !dbg !38626
  %history.i.i727.sroa.26.0.copyload = load float, ptr %history.i.i727.sroa.26.0.hot_right.i731.sroa_idx, align 4, !dbg !38626
  %history.i.i727.sroa.29.0.copyload = load float, ptr %history.i.i727.sroa.29.0.hot_right.i731.sroa_idx, align 4, !dbg !38626
  %history.i.i727.sroa.32.0.copyload = load float, ptr %history.i.i727.sroa.32.0.hot_right.i731.sroa_idx, align 4, !dbg !38626
  %history.i.i727.sroa.35.0.copyload = load float, ptr %history.i.i727.sroa.35.0.hot_right.i731.sroa_idx, align 4, !dbg !38626
  %history.i.i727.sroa.38.0.copyload = load float, ptr %history.i.i727.sroa.38.0.hot_right.i731.sroa_idx, align 4, !dbg !38626
  br i1 %_2.i46888201.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i944, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550.lr.ph, !dbg !38628

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550.lr.ph: ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4741
  %_11.i.i.i.i770 = load float, ptr %_31, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_14.i.i.i.i773 = load float, ptr %323, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_17.i.i.i.i776 = load float, ptr %324, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_20.i.i.i.i779 = load float, ptr %325, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_25.i.i.i.i784 = load float, ptr %row1.i.i.i174.i, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_28.i.i.i.i787 = load float, ptr %326, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_31.i.i.i.i790 = load float, ptr %327, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_34.i.i.i.i793 = load float, ptr %328, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_39.i.i.i.i798 = load float, ptr %row3.i.i.i188.i, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_42.i.i.i.i801 = load float, ptr %329, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_45.i.i.i.i804 = load float, ptr %330, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_48.i.i.i.i807 = load float, ptr %331, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_53.i.i.i.i812 = load float, ptr %row5.i.i.i202.i, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_56.i.i.i.i815 = load float, ptr %332, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_59.i.i.i.i818 = load float, ptr %333, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_62.i.i.i.i821 = load float, ptr %334, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_67.i.i.i.i826 = load float, ptr %row7.i.i.i216.i, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_70.i.i.i.i829 = load float, ptr %335, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_73.i.i.i.i832 = load float, ptr %336, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_76.i.i.i.i835 = load float, ptr %337, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_81.i.i.i.i840 = load float, ptr %row9.i.i.i230.i, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_84.i.i.i.i843 = load float, ptr %338, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_87.i.i.i.i846 = load float, ptr %339, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_90.i.i.i.i849 = load float, ptr %340, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_95.i.i.i.i854 = load float, ptr %row11.i.i.i244.i, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_98.i.i.i.i857 = load float, ptr %341, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_101.i.i.i.i860 = load float, ptr %342, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_104.i.i.i.i863 = load float, ptr %343, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_109.i.i.i.i868 = load float, ptr %row13.i.i.i258.i, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_112.i.i.i.i871 = load float, ptr %344, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_115.i.i.i.i874 = load float, ptr %345, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_118.i.i.i.i877 = load float, ptr %346, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_123.i.i.i.i882 = load float, ptr %row15.i.i.i272.i, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_126.i.i.i.i885 = load float, ptr %347, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_129.i.i.i.i888 = load float, ptr %348, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_132.i.i.i.i891 = load float, ptr %349, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_137.i.i.i.i896 = load float, ptr %row17.i.i.i286.i, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_140.i.i.i.i899 = load float, ptr %350, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_143.i.i.i.i902 = load float, ptr %351, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_146.i.i.i.i905 = load float, ptr %352, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_151.i.i.i.i910 = load float, ptr %row19.i.i.i300.i, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_154.i.i.i.i913 = load float, ptr %353, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_157.i.i.i.i916 = load float, ptr %354, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_160.i.i.i.i919 = load float, ptr %355, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_165.i.i.i.i924 = load float, ptr %row21.i.i.i314.i, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_168.i.i.i.i927 = load float, ptr %356, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_171.i.i.i.i930 = load float, ptr %357, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  %_174.i.i.i.i933 = load float, ptr %358, align 4, !alias.scope !38631, !noalias !38636, !noundef !12
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550, !dbg !38628

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550
  %iter.i126.i.sroa.16.08240 = phi i64 [ 0, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550.lr.ph ], [ %405, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550 ]
  %history.i.i727.sroa.35.08239 = phi float [ %history.i.i727.sroa.35.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550.lr.ph ], [ %history.i.i727.sroa.32.08238, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550 ]
  %history.i.i727.sroa.32.08238 = phi float [ %history.i.i727.sroa.32.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550.lr.ph ], [ %history.i.i727.sroa.29.08237, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550 ]
  %history.i.i727.sroa.29.08237 = phi float [ %history.i.i727.sroa.29.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550.lr.ph ], [ %history.i.i727.sroa.26.08236, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550 ]
  %history.i.i727.sroa.26.08236 = phi float [ %history.i.i727.sroa.26.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550.lr.ph ], [ %history.i.i727.sroa.22.08235, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550 ]
  %history.i.i727.sroa.22.08235 = phi float [ %history.i.i727.sroa.22.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550.lr.ph ], [ %history.i.i727.sroa.19.08234, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550 ]
  %history.i.i727.sroa.19.08234 = phi float [ %history.i.i727.sroa.19.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550.lr.ph ], [ %history.i.i727.sroa.16.08233, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550 ]
  %history.i.i727.sroa.16.08233 = phi float [ %history.i.i727.sroa.16.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550.lr.ph ], [ %history.i.i727.sroa.13.08232, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550 ]
  %history.i.i727.sroa.13.08232 = phi float [ %history.i.i727.sroa.13.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550.lr.ph ], [ %history.i.i727.sroa.10.08231, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550 ]
  %history.i.i727.sroa.10.08231 = phi float [ %history.i.i727.sroa.10.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550.lr.ph ], [ %history.i.i727.sroa.7.08230, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550 ]
  %history.i.i727.sroa.7.08230 = phi float [ %history.i.i727.sroa.7.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550.lr.ph ], [ %history.i.i727.sroa.0.08229, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550 ]
  %history.i.i727.sroa.0.08229 = phi float [ %history.i.i727.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550.lr.ph ], [ %_0.i3548, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550 ]
  %data.i.i4751 = getelementptr inbounds nuw float, ptr %_146.i, i64 %iter.i126.i.sroa.16.08240, !dbg !38643
  %_0.i3548 = load float, ptr %data.i.i4751, align 4, !dbg !38646, !alias.scope !38648, !noalias !38651, !noundef !12
  %400 = tail call noundef float @llvm.fabs.f32(float %history.i.i727.sroa.19.08234), !dbg !38652
  %_0.i3122 = fmul float %_0.i3548, %_11.i.i.i.i770, !dbg !38655
  %_0.i2690 = fadd float %_0.i3122, 0.000000e+00, !dbg !38658
  %_0.i3121 = fmul float %_0.i3548, %_14.i.i.i.i773, !dbg !38660
  %_0.i2689 = fadd float %_0.i3121, 0.000000e+00, !dbg !38662
  %_0.i3120 = fmul float %_0.i3548, %_17.i.i.i.i776, !dbg !38664
  %_0.i2688 = fadd float %_0.i3120, 0.000000e+00, !dbg !38666
  %_0.i3119 = fmul float %_0.i3548, %_20.i.i.i.i779, !dbg !38668
  %_0.i2687 = fadd float %_0.i3119, 0.000000e+00, !dbg !38670
  %_0.i3118 = fmul float %history.i.i727.sroa.0.08229, %_25.i.i.i.i784, !dbg !38672
  %_0.i2686 = fadd float %_0.i2690, %_0.i3118, !dbg !38674
  %_0.i3117 = fmul float %history.i.i727.sroa.0.08229, %_28.i.i.i.i787, !dbg !38676
  %_0.i2685 = fadd float %_0.i2689, %_0.i3117, !dbg !38678
  %_0.i3116 = fmul float %history.i.i727.sroa.0.08229, %_31.i.i.i.i790, !dbg !38680
  %_0.i2684 = fadd float %_0.i2688, %_0.i3116, !dbg !38682
  %_0.i3115 = fmul float %history.i.i727.sroa.0.08229, %_34.i.i.i.i793, !dbg !38684
  %_0.i2683 = fadd float %_0.i2687, %_0.i3115, !dbg !38686
  %_0.i3114 = fmul float %history.i.i727.sroa.7.08230, %_39.i.i.i.i798, !dbg !38688
  %_0.i2682 = fadd float %_0.i2686, %_0.i3114, !dbg !38690
  %_0.i3113 = fmul float %history.i.i727.sroa.7.08230, %_42.i.i.i.i801, !dbg !38692
  %_0.i2681 = fadd float %_0.i2685, %_0.i3113, !dbg !38694
  %_0.i3112 = fmul float %history.i.i727.sroa.7.08230, %_45.i.i.i.i804, !dbg !38696
  %_0.i2680 = fadd float %_0.i2684, %_0.i3112, !dbg !38698
  %_0.i3111 = fmul float %history.i.i727.sroa.7.08230, %_48.i.i.i.i807, !dbg !38700
  %_0.i2679 = fadd float %_0.i2683, %_0.i3111, !dbg !38702
  %_0.i3110 = fmul float %history.i.i727.sroa.10.08231, %_53.i.i.i.i812, !dbg !38704
  %_0.i2678 = fadd float %_0.i2682, %_0.i3110, !dbg !38706
  %_0.i3109 = fmul float %history.i.i727.sroa.10.08231, %_56.i.i.i.i815, !dbg !38708
  %_0.i2677 = fadd float %_0.i2681, %_0.i3109, !dbg !38710
  %_0.i3108 = fmul float %history.i.i727.sroa.10.08231, %_59.i.i.i.i818, !dbg !38712
  %_0.i2676 = fadd float %_0.i2680, %_0.i3108, !dbg !38714
  %_0.i3107 = fmul float %history.i.i727.sroa.10.08231, %_62.i.i.i.i821, !dbg !38716
  %_0.i2675 = fadd float %_0.i2679, %_0.i3107, !dbg !38718
  %_0.i3106 = fmul float %history.i.i727.sroa.13.08232, %_67.i.i.i.i826, !dbg !38720
  %_0.i2674 = fadd float %_0.i2678, %_0.i3106, !dbg !38722
  %_0.i3105 = fmul float %history.i.i727.sroa.13.08232, %_70.i.i.i.i829, !dbg !38724
  %_0.i2673 = fadd float %_0.i2677, %_0.i3105, !dbg !38726
  %_0.i3104 = fmul float %history.i.i727.sroa.13.08232, %_73.i.i.i.i832, !dbg !38728
  %_0.i2672 = fadd float %_0.i2676, %_0.i3104, !dbg !38730
  %_0.i3103 = fmul float %history.i.i727.sroa.13.08232, %_76.i.i.i.i835, !dbg !38732
  %_0.i2671 = fadd float %_0.i2675, %_0.i3103, !dbg !38734
  %_0.i3102 = fmul float %history.i.i727.sroa.16.08233, %_81.i.i.i.i840, !dbg !38736
  %_0.i2670 = fadd float %_0.i2674, %_0.i3102, !dbg !38738
  %_0.i3101 = fmul float %history.i.i727.sroa.16.08233, %_84.i.i.i.i843, !dbg !38740
  %_0.i2669 = fadd float %_0.i2673, %_0.i3101, !dbg !38742
  %_0.i3100 = fmul float %history.i.i727.sroa.16.08233, %_87.i.i.i.i846, !dbg !38744
  %_0.i2668 = fadd float %_0.i2672, %_0.i3100, !dbg !38746
  %_0.i3099 = fmul float %history.i.i727.sroa.16.08233, %_90.i.i.i.i849, !dbg !38748
  %_0.i2667 = fadd float %_0.i2671, %_0.i3099, !dbg !38750
  %_0.i3098 = fmul float %history.i.i727.sroa.19.08234, %_95.i.i.i.i854, !dbg !38752
  %_0.i2666 = fadd float %_0.i2670, %_0.i3098, !dbg !38754
  %_0.i3097 = fmul float %history.i.i727.sroa.19.08234, %_98.i.i.i.i857, !dbg !38756
  %_0.i2665 = fadd float %_0.i2669, %_0.i3097, !dbg !38758
  %_0.i3096 = fmul float %history.i.i727.sroa.19.08234, %_101.i.i.i.i860, !dbg !38760
  %_0.i2664 = fadd float %_0.i2668, %_0.i3096, !dbg !38762
  %_0.i3095 = fmul float %history.i.i727.sroa.19.08234, %_104.i.i.i.i863, !dbg !38764
  %_0.i2663 = fadd float %_0.i2667, %_0.i3095, !dbg !38766
  %_0.i3094 = fmul float %history.i.i727.sroa.22.08235, %_109.i.i.i.i868, !dbg !38768
  %_0.i2662 = fadd float %_0.i2666, %_0.i3094, !dbg !38770
  %_0.i3093 = fmul float %history.i.i727.sroa.22.08235, %_112.i.i.i.i871, !dbg !38772
  %_0.i2661 = fadd float %_0.i2665, %_0.i3093, !dbg !38774
  %_0.i3092 = fmul float %history.i.i727.sroa.22.08235, %_115.i.i.i.i874, !dbg !38776
  %_0.i2660 = fadd float %_0.i2664, %_0.i3092, !dbg !38778
  %_0.i3091 = fmul float %history.i.i727.sroa.22.08235, %_118.i.i.i.i877, !dbg !38780
  %_0.i2659 = fadd float %_0.i2663, %_0.i3091, !dbg !38782
  %_0.i3090 = fmul float %history.i.i727.sroa.26.08236, %_123.i.i.i.i882, !dbg !38784
  %_0.i2658 = fadd float %_0.i2662, %_0.i3090, !dbg !38786
  %_0.i3089 = fmul float %history.i.i727.sroa.26.08236, %_126.i.i.i.i885, !dbg !38788
  %_0.i2657 = fadd float %_0.i2661, %_0.i3089, !dbg !38790
  %_0.i3088 = fmul float %history.i.i727.sroa.26.08236, %_129.i.i.i.i888, !dbg !38792
  %_0.i2656 = fadd float %_0.i2660, %_0.i3088, !dbg !38794
  %_0.i3087 = fmul float %history.i.i727.sroa.26.08236, %_132.i.i.i.i891, !dbg !38796
  %_0.i2655 = fadd float %_0.i2659, %_0.i3087, !dbg !38798
  %_0.i3086 = fmul float %history.i.i727.sroa.29.08237, %_137.i.i.i.i896, !dbg !38800
  %_0.i2654 = fadd float %_0.i2658, %_0.i3086, !dbg !38802
  %_0.i3085 = fmul float %history.i.i727.sroa.29.08237, %_140.i.i.i.i899, !dbg !38804
  %_0.i2653 = fadd float %_0.i2657, %_0.i3085, !dbg !38806
  %_0.i3084 = fmul float %history.i.i727.sroa.29.08237, %_143.i.i.i.i902, !dbg !38808
  %_0.i2652 = fadd float %_0.i2656, %_0.i3084, !dbg !38810
  %_0.i3083 = fmul float %history.i.i727.sroa.29.08237, %_146.i.i.i.i905, !dbg !38812
  %_0.i2651 = fadd float %_0.i2655, %_0.i3083, !dbg !38814
  %_0.i3082 = fmul float %history.i.i727.sroa.32.08238, %_151.i.i.i.i910, !dbg !38816
  %_0.i2650 = fadd float %_0.i2654, %_0.i3082, !dbg !38818
  %_0.i3081 = fmul float %history.i.i727.sroa.32.08238, %_154.i.i.i.i913, !dbg !38820
  %_0.i2649 = fadd float %_0.i2653, %_0.i3081, !dbg !38822
  %_0.i3080 = fmul float %history.i.i727.sroa.32.08238, %_157.i.i.i.i916, !dbg !38824
  %_0.i2648 = fadd float %_0.i2652, %_0.i3080, !dbg !38826
  %_0.i3079 = fmul float %history.i.i727.sroa.32.08238, %_160.i.i.i.i919, !dbg !38828
  %_0.i2647 = fadd float %_0.i2651, %_0.i3079, !dbg !38830
  %_0.i3078 = fmul float %history.i.i727.sroa.35.08239, %_165.i.i.i.i924, !dbg !38832
  %_0.i2646 = fadd float %_0.i2650, %_0.i3078, !dbg !38834
  %_0.i3077 = fmul float %history.i.i727.sroa.35.08239, %_168.i.i.i.i927, !dbg !38836
  %_0.i2645 = fadd float %_0.i2649, %_0.i3077, !dbg !38838
  %_0.i3076 = fmul float %history.i.i727.sroa.35.08239, %_171.i.i.i.i930, !dbg !38840
  %_0.i2644 = fadd float %_0.i2648, %_0.i3076, !dbg !38842
  %_0.i3075 = fmul float %history.i.i727.sroa.35.08239, %_174.i.i.i.i933, !dbg !38844
  %_0.i2643 = fadd float %_0.i2647, %_0.i3075, !dbg !38846
  %401 = tail call noundef float @llvm.fabs.f32(float %_0.i2646), !dbg !38848
  %_3.i.i4172.inv = fcmp ogt float %400, %401, !dbg !38850
  %_4.i.i4179.v = select i1 %_3.i.i4172.inv, float %400, float %401, !dbg !38850
  %402 = tail call noundef float @llvm.fabs.f32(float %_0.i2645), !dbg !38848
  %_3.i.i4172.inv.1 = fcmp ogt float %_4.i.i4179.v, %402, !dbg !38850
  %_4.i.i4179.v.1 = select i1 %_3.i.i4172.inv.1, float %_4.i.i4179.v, float %402, !dbg !38850
  %403 = tail call noundef float @llvm.fabs.f32(float %_0.i2644), !dbg !38848
  %_3.i.i4172.inv.2 = fcmp ogt float %_4.i.i4179.v.1, %403, !dbg !38850
  %_4.i.i4179.v.2 = select i1 %_3.i.i4172.inv.2, float %_4.i.i4179.v.1, float %403, !dbg !38850
  %404 = tail call noundef float @llvm.fabs.f32(float %_0.i2643), !dbg !38848
  %_3.i.i4172.inv.3 = fcmp ogt float %_4.i.i4179.v.2, %404, !dbg !38850
  %_4.i.i4179.v.3 = select i1 %_3.i.i4172.inv.3, float %_4.i.i4179.v.2, float %404, !dbg !38850
  %405 = add nuw nsw i64 %iter.i126.i.sroa.16.08240, 1, !dbg !38853
  %data.i4.i4755 = getelementptr inbounds nuw float, ptr %peaks_right.i729, i64 %iter.i126.i.sroa.16.08240, !dbg !38854
  store float %_4.i.i4179.v.3, ptr %data.i4.i4755, align 4, !dbg !38857, !alias.scope !38859, !noalias !38651
  %exitcond11768.not = icmp eq i64 %405, %umax11782, !dbg !38628
  br i1 %exitcond11768.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i944, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550, !dbg !38628

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i944: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4741
  %history.i.i727.sroa.0.0.lcssa = phi float [ %history.i.i727.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4741 ], [ %_0.i3548, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550 ], !dbg !38862
  %history.i.i727.sroa.7.0.lcssa = phi float [ %history.i.i727.sroa.7.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4741 ], [ %history.i.i727.sroa.0.08229, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550 ], !dbg !38862
  %history.i.i727.sroa.10.0.lcssa = phi float [ %history.i.i727.sroa.10.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4741 ], [ %history.i.i727.sroa.7.08230, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550 ], !dbg !38862
  %history.i.i727.sroa.13.0.lcssa = phi float [ %history.i.i727.sroa.13.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4741 ], [ %history.i.i727.sroa.10.08231, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550 ], !dbg !38862
  %history.i.i727.sroa.16.0.lcssa = phi float [ %history.i.i727.sroa.16.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4741 ], [ %history.i.i727.sroa.13.08232, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550 ], !dbg !38862
  %history.i.i727.sroa.19.0.lcssa = phi float [ %history.i.i727.sroa.19.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4741 ], [ %history.i.i727.sroa.16.08233, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550 ], !dbg !38862
  %history.i.i727.sroa.22.0.lcssa = phi float [ %history.i.i727.sroa.22.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4741 ], [ %history.i.i727.sroa.19.08234, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550 ], !dbg !38862
  %history.i.i727.sroa.26.0.lcssa = phi float [ %history.i.i727.sroa.26.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4741 ], [ %history.i.i727.sroa.22.08235, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550 ], !dbg !38862
  %history.i.i727.sroa.29.0.lcssa = phi float [ %history.i.i727.sroa.29.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4741 ], [ %history.i.i727.sroa.26.08236, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550 ], !dbg !38862
  %history.i.i727.sroa.32.0.lcssa = phi float [ %history.i.i727.sroa.32.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4741 ], [ %history.i.i727.sroa.29.08237, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550 ], !dbg !38862
  %history.i.i727.sroa.35.0.lcssa = phi float [ %history.i.i727.sroa.35.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4741 ], [ %history.i.i727.sroa.32.08238, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550 ], !dbg !38862
  %history.i.i727.sroa.38.0.lcssa = phi float [ %history.i.i727.sroa.38.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4741 ], [ %history.i.i727.sroa.35.08239, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3550 ], !dbg !38862
  store float %history.i.i727.sroa.0.0.lcssa, ptr %hot_right.i731, align 4, !dbg !38863
  store float %history.i.i727.sroa.7.0.lcssa, ptr %history.i.i727.sroa.7.0.hot_right.i731.sroa_idx, align 4, !dbg !38863
  store float %history.i.i727.sroa.10.0.lcssa, ptr %history.i.i727.sroa.10.0.hot_right.i731.sroa_idx, align 4, !dbg !38863
  store float %history.i.i727.sroa.13.0.lcssa, ptr %history.i.i727.sroa.13.0.hot_right.i731.sroa_idx, align 4, !dbg !38863
  store float %history.i.i727.sroa.16.0.lcssa, ptr %history.i.i727.sroa.16.0.hot_right.i731.sroa_idx, align 4, !dbg !38863
  store float %history.i.i727.sroa.19.0.lcssa, ptr %history.i.i727.sroa.19.0.hot_right.i731.sroa_idx, align 4, !dbg !38863
  store float %history.i.i727.sroa.22.0.lcssa, ptr %history.i.i727.sroa.22.0.hot_right.i731.sroa_idx, align 4, !dbg !38863
  store float %history.i.i727.sroa.26.0.lcssa, ptr %history.i.i727.sroa.26.0.hot_right.i731.sroa_idx, align 4, !dbg !38863
  store float %history.i.i727.sroa.29.0.lcssa, ptr %history.i.i727.sroa.29.0.hot_right.i731.sroa_idx, align 4, !dbg !38863
  store float %history.i.i727.sroa.32.0.lcssa, ptr %history.i.i727.sroa.32.0.hot_right.i731.sroa_idx, align 4, !dbg !38863
  store float %history.i.i727.sroa.35.0.lcssa, ptr %history.i.i727.sroa.35.0.hot_right.i731.sroa_idx, align 4, !dbg !38863
  store float %history.i.i727.sroa.38.0.lcssa, ptr %history.i.i727.sroa.38.0.hot_right.i731.sroa_idx, align 4, !dbg !38863
  br i1 %_2.i46888201.not, label %bb13.i.loopexit, label %bb48.i.lr.ph, !dbg !38339

bb48.i.lr.ph:                                     ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i944
  %_8.i30.i = load float, ptr %_68.i948, align 4, !noundef !12
  %_9.i31.i = load float, ptr %_69.i, align 4, !noundef !12
  %_8.i.i950 = load float, ptr %_73.i, align 4, !noundef !12
  %_9.i.i951 = load float, ptr %_74.i949, align 4, !noundef !12
  %_91.i = load i64, ptr %359, align 8
  %_64.i89.i = load float, ptr %370, align 4
  %_64.i.i = load float, ptr %386, align 4
  %_106.i = load i64, ptr %390, align 8
  %.promoted8281 = load float, ptr %369, align 4
  %.promoted8350 = load float, ptr %371, align 4
  %.promoted8352 = load float, ptr %385, align 4
  %.promoted8421 = load float, ptr %387, align 4
  br label %bb48.i, !dbg !38339

bb48.i:                                           ; preds = %bb48.i.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3661
  %_0.i37688422 = phi float [ %.promoted8421, %bb48.i.lr.ph ], [ %_0.i3768, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3661 ]
  %_0.i33948353 = phi float [ %.promoted8352, %bb48.i.lr.ph ], [ %_0.i3394, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3661 ]
  %_0.i37728351 = phi float [ %.promoted8350, %bb48.i.lr.ph ], [ %_0.i3772, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3661 ]
  %_0.i33988282 = phi float [ %.promoted8281, %bb48.i.lr.ph ], [ %_0.i3398, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3661 ]
  %main_cursor.sroa.0.1.i9478278 = phi i64 [ %main_cursor.sroa.0.0.i7478427, %bb48.i.lr.ph ], [ %spec.store.select.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3661 ]
  %ring_cursor.sroa.0.1.i9468277 = phi i64 [ %ring_cursor.sroa.0.0.i7468426, %bb48.i.lr.ph ], [ %spec.store.select13.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3661 ]
  %iter1.sroa.0.0.i9458276 = phi i64 [ 0, %bb48.i.lr.ph ], [ %406, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3661 ]
  %406 = add nuw nsw i64 %iter1.sroa.0.0.i9458276, 1, !dbg !38864
  %_64.i = add nuw nsw i64 %iter1.sroa.0.0.i9458276, %iter.sroa.0.0.i8424, !dbg !38870
  %_162.i = getelementptr inbounds nuw float, ptr %peaks_left.i730, i64 %iter1.sroa.0.0.i9458276, !dbg !38871
  %_0.i3543 = load float, ptr %_162.i, align 4, !dbg !38882, !alias.scope !38884, !noalias !38366, !noundef !12
  %_167.i = getelementptr inbounds nuw float, ptr %peaks_right.i729, i64 %iter1.sroa.0.0.i9458276, !dbg !38887
  %_0.i3538 = load float, ptr %_167.i, align 4, !dbg !38897, !alias.scope !38899, !noalias !38366, !noundef !12
  %_3.i.i4163 = fcmp ule float %_0.i3538, %_0.i3543, !dbg !38902
  %_6.i.i4165 = bitcast float %_0.i3538 to i32, !dbg !38905
  %_8.i.i4167 = bitcast float %_0.i3543 to i32, !dbg !38908
  %_4.i.i4170 = select i1 %_3.i.i4163, i32 %_8.i.i4167, i32 %_6.i.i4165, !dbg !38910
  %_5.i3964 = and i32 %_4.i.i4170, %.none.i737, !dbg !38911
  %_7.i3967 = and i32 %_9.i3966, %_8.i.i4167, !dbg !38913
  %_4.i3968 = or disjoint i32 %_5.i3964, %_7.i3967, !dbg !38911
  %_0.i3969 = bitcast i32 %_4.i3968 to float, !dbg !38914
  %_7.i3960 = and i32 %_9.i3966, %_6.i.i4165, !dbg !38916
  %_4.i3961 = or disjoint i32 %_5.i3964, %_7.i3960, !dbg !38918
  %_0.i3962 = bitcast i32 %_4.i3961 to float, !dbg !38919
  %_168.i = icmp ugt i64 %_64.i, %left_io.1, !dbg !38921
  br i1 %_168.i, label %bb52.i, label %bb53.i955, !dbg !38921, !prof !1406

bb53.i955:                                        ; preds = %bb48.i
  %_174.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %_64.i, !dbg !38925
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38930), !dbg !38933
  %_3.not.i3531 = icmp eq i64 %left_io.1, %_64.i, !dbg !38934
  br i1 %_3.not.i3531, label %panic.i3534, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3535, !dbg !38934

panic.i3534:                                      ; preds = %bb53.i955
  store float %_0.i37728351, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !38934, !noalias !38936
  unreachable, !dbg !38934

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3535: ; preds = %bb53.i955
  %_0.i3533 = load float, ptr %_174.i, align 4, !dbg !38934, !alias.scope !38930, !noalias !38366, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38937), !dbg !38940
  %width.i33.i = load i64, ptr %360, align 8, !dbg !38941, !alias.scope !38942, !noalias !38943, !noundef !12
  %_3.i2509 = fcmp uge float %_8.i30.i, %_0.i3969, !dbg !38947
  %_0.i2942 = fdiv float %_8.i30.i, %_0.i3969, !dbg !38949
  %_0.i3955 = select i1 %_3.i2509, float 1.000000e+00, float %_0.i2942, !dbg !38951
  %_144.1.i38.i = load i64, ptr %361, align 8, !dbg !38953, !alias.scope !38942, !noalias !38943, !noundef !12
  %_22.i39.i = mul i64 %width.i33.i, %ring_cursor.sroa.0.1.i9468277, !dbg !38954
  %_92.i40.i = icmp ugt i64 %_22.i39.i, %_144.1.i38.i, !dbg !38955
  br i1 %_92.i40.i, label %bb37.i124.i, label %bb38.i41.i, !dbg !38955, !prof !1406

bb38.i41.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3535
  %_144.0.i42.i = load ptr, ptr %362, align 8, !dbg !38953, !alias.scope !38942, !noalias !38943, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38958), !dbg !38961
  %_4.not.i3686 = icmp eq i64 %_144.1.i38.i, %_22.i39.i, !dbg !38962
  br i1 %_4.not.i3686, label %panic.i3688, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3689, !dbg !38962

panic.i3688:                                      ; preds = %bb38.i41.i
  store float %_0.i37728351, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !38962, !noalias !38964
  unreachable, !dbg !38962

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3689: ; preds = %bb38.i41.i
  %_99.i44.i = getelementptr inbounds nuw float, ptr %_144.0.i42.i, i64 %_22.i39.i, !dbg !38965
  store float %_0.i3955, ptr %_99.i44.i, align 4, !dbg !38962, !alias.scope !38958, !noalias !38967
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38968), !dbg !38971
  %width.i2227 = load i64, ptr %360, align 8, !dbg !38972, !alias.scope !38968, !noalias !38974, !noundef !12
  %407 = icmp eq i64 %width.i2227, 0, !dbg !38976
  br i1 %407, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2335, label %bb32.i2234.lr.ph, !dbg !38976

bb32.i2234.lr.ph:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3689
  %_112.1.i2237 = load i64, ptr %62, align 8, !alias.scope !38968, !noalias !38974, !noundef !12
  %_112.0.i2241 = load ptr, ptr %61, align 8, !nonnull !12
  %408 = add i64 %ring_cursor.sroa.0.1.i9468277, 1
  %_23.not.i2248 = icmp ult i64 %408, %_91.i
  %409 = select i1 %_23.not.i2248, i64 0, i64 %_91.i
  %start1.sroa.0.0.i2249 = sub nuw i64 %408, %409
  %_114.1.i2252 = load i64, ptr %361, align 8
  %_114.0.i2256 = load ptr, ptr %362, align 8, !nonnull !12
  %_116.1.i2257 = load i64, ptr %363, align 8
  %_116.0.i2261 = load ptr, ptr %364, align 8, !nonnull !12
  %_118.1.i2265 = load i64, ptr %365, align 8
  %_118.0.i2269 = load ptr, ptr %366, align 8, !nonnull !12
  %_45.i2282 = mul i64 %width.i2227, %start1.sroa.0.0.i2249
  br label %bb32.i2234, !dbg !38976

bb32.i2234:                                       ; preds = %bb32.i2234.lr.ph, %bb31.i2297
  %iter.i2226.sroa.10.08258 = phi i64 [ %width.i2227, %bb32.i2234.lr.ph ], [ %410, %bb31.i2297 ]
  %iter.i2226.sroa.7.08257 = phi i64 [ 0, %bb32.i2234.lr.ph ], [ %_9.0.i4778, %bb31.i2297 ]
  %iter.i2226.sroa.0.0.idx8256 = phi i64 [ 0, %bb32.i2234.lr.ph ], [ %iter.i2226.sroa.0.0.add, %bb31.i2297 ]
  %iter.i2226.sroa.0.0.ptr8259 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 %iter.i2226.sroa.0.0.idx8256, !dbg !38978
  %410 = add i64 %iter.i2226.sroa.10.08258, -1, !dbg !38978
  %_7.i.i4774 = icmp eq i64 %iter.i2226.sroa.0.0.idx8256, 32, !dbg !38979
  br i1 %_7.i.i4774, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2335, label %bb3.i2236, !dbg !38983

bb3.i2236:                                        ; preds = %bb32.i2234
  %iter.i2226.sroa.0.0.add = add nuw nsw i64 %iter.i2226.sroa.0.0.idx8256, 4, !dbg !38984
  %_9.0.i4778 = add nuw nsw i64 %iter.i2226.sroa.7.08257, 1, !dbg !38986
  %exitcond11771.not = icmp eq i64 %iter.i2226.sroa.7.08257, %_112.1.i2237, !dbg !38987
  br i1 %exitcond11771.not, label %panic.i2239, label %bb5.i2240, !dbg !38987

bb5.i2240:                                        ; preds = %bb3.i2236
  %411 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i2241, i64 %iter.i2226.sroa.7.08257, !dbg !38987
  %shape.i2242 = load i32, ptr %411, align 4, !dbg !38987, !noalias !38988, !noundef !12
  %412 = getelementptr inbounds nuw i8, ptr %411, i64 4, !dbg !38987
  %shape3.i2243 = load i32, ptr %412, align 4, !dbg !38987, !noalias !38988, !noundef !12
  %window.i2244 = zext i32 %shape.i2242 to i64, !dbg !38989
  %_19.i2245 = zext i32 %shape3.i2243 to i64, !dbg !38990
  %413 = add i64 %ring_cursor.sroa.0.1.i9468277, %_19.i2245, !dbg !38991
  %_20.not.i2246 = icmp ult i64 %413, %_91.i, !dbg !38992
  %414 = select i1 %_20.not.i2246, i64 0, i64 %_91.i, !dbg !38992
  %spec.select.i2247 = sub nuw i64 %413, %414, !dbg !38992
  %_27.i2250 = mul i64 %spec.select.i2247, %width.i2227, !dbg !38993
  %_26.i2251 = add i64 %_27.i2250, %iter.i2226.sroa.7.08257, !dbg !38993
  %_30.i2253 = icmp ult i64 %_26.i2251, %_114.1.i2252, !dbg !38994
  br i1 %_30.i2253, label %bb12.i2255, label %panic5.i2254, !dbg !38994

panic.i2239:                                      ; preds = %bb3.i2236
  store float %_0.i37728351, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i2237, i64 noundef %_112.1.i2237, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4a8785a681d008a9bfd0cd82628ea9cb) #30, !dbg !38987, !noalias !38988
  unreachable, !dbg !38987

bb12.i2255:                                       ; preds = %bb5.i2240
  %415 = getelementptr inbounds nuw float, ptr %_114.0.i2256, i64 %_26.i2251, !dbg !38994
  %416 = load float, ptr %415, align 4, !dbg !38994, !noalias !38988, !noundef !12
  %exitcond11772.not = icmp eq i64 %iter.i2226.sroa.7.08257, %_116.1.i2257, !dbg !38995
  br i1 %exitcond11772.not, label %panic6.i2259, label %bb13.i2260, !dbg !38995

panic5.i2254:                                     ; preds = %bb5.i2240
  store float %_0.i37728351, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i2251, i64 noundef %_114.1.i2252, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cbce7773ac40979e4ba2385da3aec116) #30, !dbg !38994, !noalias !38988
  unreachable, !dbg !38994

bb13.i2260:                                       ; preds = %bb12.i2255
  %417 = getelementptr inbounds nuw i32, ptr %_116.0.i2261, i64 %iter.i2226.sroa.7.08257, !dbg !38995
  %_32.i2262 = load i32, ptr %417, align 4, !dbg !38995, !noalias !38988, !noundef !12
  %position.i2263 = zext i32 %_32.i2262 to i64, !dbg !38995
  %418 = icmp eq i32 %_32.i2262, 0, !dbg !38996
  br i1 %418, label %bb17.i2272, label %bb15.i2264, !dbg !38996

panic6.i2259:                                     ; preds = %bb12.i2255
  store float %_0.i37728351, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i2257, i64 noundef %_116.1.i2257, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ec0d48f73ebfc2755df5cedaa60b5c0a) #30, !dbg !38995, !noalias !38988
  unreachable, !dbg !38995

bb15.i2264:                                       ; preds = %bb13.i2260
  %_37.i2266 = icmp ult i64 %iter.i2226.sroa.7.08257, %_118.1.i2265, !dbg !38997
  br i1 %_37.i2266, label %bb16.i2268, label %panic7.i2267, !dbg !38997

bb17.i2272:                                       ; preds = %bb35.i2333, %bb16.i2268, %bb13.i2260
  %newest.sroa.0.0.i2273 = phi float [ %416, %bb13.i2260 ], [ %_35.i2270, %bb35.i2333 ], [ %416, %bb16.i2268 ], !dbg !38998
  %exitcond11773.not = icmp eq i64 %iter.i2226.sroa.7.08257, %_118.1.i2265, !dbg !38999
  br i1 %exitcond11773.not, label %panic8.i2276, label %bb18.i2277, !dbg !38999

bb16.i2268:                                       ; preds = %bb15.i2264
  %419 = getelementptr inbounds nuw float, ptr %_118.0.i2269, i64 %iter.i2226.sroa.7.08257, !dbg !38997
  %_35.i2270 = load float, ptr %419, align 4, !dbg !38997, !noalias !38988, !noundef !12
  %_102.i2271 = fcmp olt float %_35.i2270, %416, !dbg !39000
  br i1 %_102.i2271, label %bb35.i2333, label %bb17.i2272, !dbg !39000

panic7.i2267:                                     ; preds = %bb15.i2264
  store float %_0.i37728351, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i2226.sroa.7.08257, i64 noundef %_118.1.i2265, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2c461872bb652d4796cdcf89c28c82c8) #30, !dbg !38997, !noalias !38988
  unreachable, !dbg !38997

bb35.i2333:                                       ; preds = %bb16.i2268
  br label %bb17.i2272, !dbg !39002

bb18.i2277:                                       ; preds = %bb17.i2272
  %420 = getelementptr inbounds nuw float, ptr %_118.0.i2269, i64 %iter.i2226.sroa.7.08257, !dbg !38999
  store float %newest.sroa.0.0.i2273, ptr %420, align 4, !dbg !38999, !noalias !38988
  %_42.i2279 = add nuw nsw i64 %position.i2263, 1, !dbg !39003
  %complete.i2280 = icmp eq i64 %_42.i2279, %window.i2244, !dbg !39003
  br i1 %complete.i2280, label %bb22.i2302, label %bb20.i2281, !dbg !39004

panic8.i2276:                                     ; preds = %bb17.i2272
  store float %_0.i37728351, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i2265, i64 noundef %_118.1.i2265, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2b690e2c7763f11809942906fc2ca813) #30, !dbg !38999, !noalias !38988
  unreachable, !dbg !38999

bb20.i2281:                                       ; preds = %bb18.i2277
  %_44.i2283 = add i64 %iter.i2226.sroa.7.08257, %_45.i2282, !dbg !39005
  %_47.i2285 = icmp ult i64 %_44.i2283, %_114.1.i2252, !dbg !39006
  br i1 %_47.i2285, label %bb30.i2295, label %panic9.i2286, !dbg !39006

panic9.i2286:                                     ; preds = %bb20.i2281
  store float %_0.i37728351, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i2283, i64 noundef %_114.1.i2252, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9fa421ae81817f58fcfc4a3243223891) #30, !dbg !39006, !noalias !38988
  unreachable, !dbg !39006

bb30.i2295:                                       ; preds = %bb20.i2281
  %421 = getelementptr inbounds nuw float, ptr %_114.0.i2256, i64 %_44.i2283, !dbg !39006
  %_43.i2289 = load float, ptr %421, align 4, !dbg !39006, !noalias !38988, !noundef !12
  %_103.i2290 = fcmp olt float %_43.i2289, %newest.sroa.0.0.i2273, !dbg !39007
  %newest.sroa.0.1.i2291 = select i1 %_103.i2290, float %_43.i2289, float %newest.sroa.0.0.i2273, !dbg !39007
  store float %newest.sroa.0.1.i2291, ptr %iter.i2226.sroa.0.0.ptr8259, align 4, !dbg !39009, !noalias !38988
  %422 = trunc i64 %_42.i2279 to i32, !dbg !39010
  br label %bb31.i2297, !dbg !39011

bb31.i2297:                                       ; preds = %bb25.i2330, %bb30.i2295
  %storemerge5679 = phi i32 [ %422, %bb30.i2295 ], [ 0, %bb25.i2330 ], !dbg !39012
  store i32 %storemerge5679, ptr %417, align 4, !dbg !39012, !noalias !38988
  %423 = icmp eq i64 %410, 0, !dbg !38976
  br i1 %423, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2335, label %bb32.i2234, !dbg !38976

bb22.i2302:                                       ; preds = %bb18.i2277
  store float %newest.sroa.0.0.i2273, ptr %iter.i2226.sroa.0.0.ptr8259, align 4, !dbg !39009, !noalias !38988
  %424 = load float, ptr %415, align 4, !dbg !39013, !noalias !38988, !noundef !12
  br label %bb41.i2315, !dbg !39014

bb41.i2315:                                       ; preds = %bb22.i2302, %bb25.i2330
  %iter2.sroa.0.0.i23078255 = phi i64 [ 0, %bb22.i2302 ], [ %_105.i2316, %bb25.i2330 ]
  %suffix.sroa.0.0.i23068254 = phi float [ %424, %bb22.i2302 ], [ %suffix.sroa.0.1.i2326, %bb25.i2330 ]
  %end.sroa.0.1.i23058253 = phi i64 [ %spec.select.i2247, %bb22.i2302 ], [ %427, %bb25.i2330 ]
  %_56.i2317 = mul i64 %end.sroa.0.1.i23058253, %width.i2227, !dbg !39017
  %_55.i2318 = add i64 %_56.i2317, %iter.i2226.sroa.7.08257, !dbg !39017
  %_59.i2320 = icmp ult i64 %_55.i2318, %_114.1.i2252, !dbg !39018
  br i1 %_59.i2320, label %bb25.i2330, label %panic13.i2321, !dbg !39018

panic13.i2321:                                    ; preds = %bb41.i2315
  store float %_0.i37728351, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i2318, i64 noundef %_114.1.i2252, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_91a4c6b9b17ebf4d863f9a70b6dc929a) #30, !dbg !39018, !noalias !38988
  unreachable, !dbg !39018

bb25.i2330:                                       ; preds = %bb41.i2315
  %_105.i2316 = add nuw nsw i64 %iter2.sroa.0.0.i23078255, 1, !dbg !39019
  %425 = getelementptr inbounds nuw float, ptr %_114.0.i2256, i64 %_55.i2318, !dbg !39018
  %_54.i2324 = load float, ptr %425, align 4, !dbg !39018, !noalias !38988, !noundef !12
  %_107.i2325 = fcmp olt float %suffix.sroa.0.0.i23068254, %_54.i2324, !dbg !39022
  %suffix.sroa.0.1.i2326 = select i1 %_107.i2325, float %suffix.sroa.0.0.i23068254, float %_54.i2324, !dbg !39022
  store float %suffix.sroa.0.1.i2326, ptr %425, align 4, !dbg !39024, !noalias !38988
  %426 = icmp eq i64 %end.sroa.0.1.i23058253, 0, !dbg !39025
  %spec.store.select.i2332 = select i1 %426, i64 %_91.i, i64 %end.sroa.0.1.i23058253, !dbg !39025
  %427 = add i64 %spec.store.select.i2332, -1, !dbg !39026
  %exitcond11770.not = icmp eq i64 %_105.i2316, %window.i2244, !dbg !39027
  br i1 %exitcond11770.not, label %bb31.i2297, label %bb41.i2315, !dbg !39014

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2335: ; preds = %bb31.i2297, %bb32.i2234, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3689
  %_0.i3530 = load float, ptr %scratch.i, align 4, !dbg !39029, !alias.scope !39031, !noalias !39034, !noundef !12
  %_0.i3074 = fmul float %_0.i3530, 1.638400e+04, !dbg !39035
  %428 = tail call noundef float @llvm.floor.f32(float %_0.i3074), !dbg !39037
  %_0.i3073 = fmul float %428, 0x3F10000000000000, !dbg !39041
  %429 = icmp eq i64 %width.i33.i, 0, !dbg !39043
  %_149.1.i82.i.pre = load i64, ptr %367, align 8, !dbg !39045, !alias.scope !38942, !noalias !38943
  br i1 %429, label %bb16.i77.i, label %bb39.i57.i.lr.ph, !dbg !39043

bb39.i57.i.lr.ph:                                 ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2335
  %_145.1.i60.i = load i64, ptr %62, align 8, !alias.scope !38942, !noalias !38943, !noundef !12
  %_145.0.i64.i = load ptr, ptr %61, align 8, !nonnull !12
  %_147.0.i75.i = load ptr, ptr %368, align 8, !nonnull !12
  %exitcond11774.not = icmp eq i64 %_145.1.i60.i, 0, !dbg !39046
  br i1 %exitcond11774.not, label %panic.i62.i, label %bb17.i63.i, !dbg !39046

bb37.i124.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3535
  store float %_0.i37728351, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i39.i, i64 noundef %_144.1.i38.i, i64 noundef %_144.1.i38.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_56df7c041d29359441bca272bf4e38e3) #30, !dbg !39047, !noalias !38967
  unreachable, !dbg !39047

bb16.i77.i:                                       ; preds = %bb21.i74.i.7, %bb21.i74.i, %bb21.i74.i.1, %bb21.i74.i.2, %bb21.i74.i.3, %bb21.i74.i.4, %bb21.i74.i.5, %bb21.i74.i.6, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2335
  %_0.i3528 = phi float [ %_0.i3530, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2335 ], [ %_49.i76.i, %bb21.i74.i ], [ %_49.i76.i, %bb21.i74.i.7 ], [ %_49.i76.i, %bb21.i74.i.6 ], [ %_49.i76.i, %bb21.i74.i.5 ], [ %_49.i76.i, %bb21.i74.i.4 ], [ %_49.i76.i, %bb21.i74.i.3 ], [ %_49.i76.i, %bb21.i74.i.2 ], [ %_49.i76.i, %bb21.i74.i.1 ], !dbg !39048
  %_0.i2642 = fadd float %_0.i3073, %_0.i33988282, !dbg !39050
  %_0.i3398 = fsub float %_0.i2642, %_0.i3528, !dbg !39052
  %_109.i83.i = icmp ugt i64 %_22.i39.i, %_149.1.i82.i.pre, !dbg !39054
  br i1 %_109.i83.i, label %bb42.i123.i, label %bb43.i84.i, !dbg !39054, !prof !1406

bb43.i84.i:                                       ; preds = %bb16.i77.i
  %_149.0.i85.i = load ptr, ptr %368, align 8, !dbg !39045, !alias.scope !38942, !noalias !38943, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39057), !dbg !39060
  %_4.not.i3682 = icmp eq i64 %_149.1.i82.i.pre, %_22.i39.i, !dbg !39061
  br i1 %_4.not.i3682, label %panic.i3684, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3685, !dbg !39061

panic.i3684:                                      ; preds = %bb43.i84.i
  store float %_0.i37728351, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !39061, !noalias !39063
  unreachable, !dbg !39061

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3685: ; preds = %bb43.i84.i
  %_116.i87.i = getelementptr inbounds nuw float, ptr %_149.0.i85.i, i64 %_22.i39.i, !dbg !39064
  store float %_0.i3073, ptr %_116.i87.i, align 4, !dbg !39061, !alias.scope !39057, !noalias !39034
  %_0.i2941 = fdiv float %_0.i3398, %_64.i89.i, !dbg !39066
  %_0.i3397 = fsub float 1.000000e+00, %_0.i2941, !dbg !39068
  %_0.i3396 = fsub float %_0.i3397, %_0.i37728351, !dbg !39070
  %_4.i2957 = fmul float %_9.i31.i, %_0.i3396, !dbg !39072
  %_0.i2958 = fadd float %_0.i37728351, %_4.i2957, !dbg !39072
  %_3.i.i4154.inv = fcmp ogt float %_0.i3397, %_0.i2958, !dbg !39074
  %_4.i.i4161.v = select i1 %_3.i.i4154.inv, float %_0.i3397, float %_0.i2958, !dbg !39074
  %430 = tail call noundef float @llvm.fabs.f32(float %_4.i.i4161.v), !dbg !39077
  %431 = fcmp uge float %430, 0x3BC79CA100000000, !dbg !39080
  %_0.i3772 = select i1 %431, float %_4.i.i4161.v, float 0.000000e+00, !dbg !39082
  %_0.i3395 = fsub float 1.000000e+00, %_0.i3772, !dbg !39083
  %_150.1.i101.i = load i64, ptr %372, align 8, !dbg !39085, !alias.scope !38942, !noalias !38943, !noundef !12
  %_76.i102.i = mul i64 %width.i33.i, %main_cursor.sroa.0.1.i9478278, !dbg !39086
  %_120.i103.i = icmp ugt i64 %_76.i102.i, %_150.1.i101.i, !dbg !39087
  br i1 %_120.i103.i, label %bb48.i122.i, label %bb49.i104.i, !dbg !39087, !prof !1406

bb42.i123.i:                                      ; preds = %bb16.i77.i
  store float %_0.i37728351, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i39.i, i64 noundef %_149.1.i82.i.pre, i64 noundef %_149.1.i82.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_90498045d73339daaf9e4f537508f58b) #30, !dbg !39090, !noalias !39034
  unreachable, !dbg !39090

bb49.i104.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3685
  %_150.0.i105.i = load ptr, ptr %373, align 8, !dbg !39085, !alias.scope !38942, !noalias !38943, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39091), !dbg !39094
  %_3.not.i3522 = icmp eq i64 %_150.1.i101.i, %_76.i102.i, !dbg !39095
  br i1 %_3.not.i3522, label %panic.i3525, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3677, !dbg !39095

panic.i3525:                                      ; preds = %bb49.i104.i
  store float %_0.i3772, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !39095, !noalias !39097
  unreachable, !dbg !39095

bb48.i122.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3685
  store float %_0.i3772, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i102.i, i64 noundef %_150.1.i101.i, i64 noundef %_150.1.i101.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_da8af254b6d507a8e2ca31e544bfd21d) #30, !dbg !39098, !noalias !39034
  unreachable, !dbg !39098

bb17.i63.i:                                       ; preds = %bb39.i57.i.lr.ph
  %432 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i, i64 8, !dbg !39046
  %_44.i65.i = load i32, ptr %432, align 4, !dbg !39046, !noalias !39034, !noundef !12
  %_43.i66.i = zext i32 %_44.i65.i to i64, !dbg !39046
  %433 = add i64 %ring_cursor.sroa.0.1.i9468277, %_43.i66.i, !dbg !39099
  %_47.not.i67.i = icmp ult i64 %433, %_91.i, !dbg !39100
  %434 = select i1 %_47.not.i67.i, i64 0, i64 %_91.i, !dbg !39100
  %spec.select.i68.i = sub nuw i64 %433, %434, !dbg !39100
  %_51.i69.i = mul i64 %spec.select.i68.i, %width.i33.i, !dbg !39101
  %_53.i72.i = icmp ult i64 %_51.i69.i, %_149.1.i82.i.pre, !dbg !39102
  br i1 %_53.i72.i, label %bb21.i74.i, label %panic1.i73.i, !dbg !39102

panic.i62.i:                                      ; preds = %bb39.i57.i.7, %bb39.i57.i.6, %bb39.i57.i.5, %bb39.i57.i.4, %bb39.i57.i.3, %bb39.i57.i.2, %bb39.i57.i.1, %bb39.i57.i.lr.ph
  %_145.1.i60.i.lcssa.ph = phi i64 [ 7, %bb39.i57.i.7 ], [ 6, %bb39.i57.i.6 ], [ 5, %bb39.i57.i.5 ], [ 4, %bb39.i57.i.4 ], [ 3, %bb39.i57.i.3 ], [ 2, %bb39.i57.i.2 ], [ 1, %bb39.i57.i.1 ], [ 0, %bb39.i57.i.lr.ph ]
  store float %_0.i37728351, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i60.i.lcssa.ph, i64 noundef %_145.1.i60.i.lcssa.ph, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a6feb40b34112df5f214f84424dd2c7c) #30, !dbg !39046, !noalias !39034
  unreachable, !dbg !39046

bb21.i74.i:                                       ; preds = %bb17.i63.i
  %435 = getelementptr inbounds nuw float, ptr %_147.0.i75.i, i64 %_51.i69.i, !dbg !39102
  %_49.i76.i = load float, ptr %435, align 4, !dbg !39102, !noalias !39034, !noundef !12
  store float %_49.i76.i, ptr %scratch.i, align 4, !dbg !39103, !noalias !39034
  %436 = icmp eq i64 %width.i33.i, 1, !dbg !39043
  br i1 %436, label %bb16.i77.i, label %bb39.i57.i.1, !dbg !39043

bb39.i57.i.1:                                     ; preds = %bb21.i74.i
  %exitcond11774.1.not = icmp eq i64 %_145.1.i60.i, 1, !dbg !39046
  br i1 %exitcond11774.1.not, label %panic.i62.i, label %bb17.i63.i.1, !dbg !39046

bb17.i63.i.1:                                     ; preds = %bb39.i57.i.1
  %437 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i, i64 20, !dbg !39046
  %_44.i65.i.1 = load i32, ptr %437, align 4, !dbg !39046, !noalias !39034, !noundef !12
  %_43.i66.i.1 = zext i32 %_44.i65.i.1 to i64, !dbg !39046
  %438 = add i64 %ring_cursor.sroa.0.1.i9468277, %_43.i66.i.1, !dbg !39099
  %_47.not.i67.i.1 = icmp ult i64 %438, %_91.i, !dbg !39100
  %439 = select i1 %_47.not.i67.i.1, i64 0, i64 %_91.i, !dbg !39100
  %spec.select.i68.i.1 = sub nuw i64 %438, %439, !dbg !39100
  %_51.i69.i.1 = mul i64 %spec.select.i68.i.1, %width.i33.i, !dbg !39101
  %_50.i70.i.1 = add i64 %_51.i69.i.1, 1, !dbg !39101
  %_53.i72.i.1 = icmp ult i64 %_50.i70.i.1, %_149.1.i82.i.pre, !dbg !39102
  br i1 %_53.i72.i.1, label %bb21.i74.i.1, label %panic1.i73.i, !dbg !39102

bb21.i74.i.1:                                     ; preds = %bb17.i63.i.1
  %440 = getelementptr inbounds nuw float, ptr %_147.0.i75.i, i64 %_50.i70.i.1, !dbg !39102
  %_49.i76.i.1 = load float, ptr %440, align 4, !dbg !39102, !noalias !39034, !noundef !12
  store float %_49.i76.i.1, ptr %iter.i32.i.sroa.0.0.ptr8263.1, align 4, !dbg !39103, !noalias !39034
  %441 = icmp eq i64 %width.i33.i, 2, !dbg !39043
  br i1 %441, label %bb16.i77.i, label %bb39.i57.i.2, !dbg !39043

bb39.i57.i.2:                                     ; preds = %bb21.i74.i.1
  %exitcond11774.2.not = icmp eq i64 %_145.1.i60.i, 2, !dbg !39046
  br i1 %exitcond11774.2.not, label %panic.i62.i, label %bb17.i63.i.2, !dbg !39046

bb17.i63.i.2:                                     ; preds = %bb39.i57.i.2
  %442 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i, i64 32, !dbg !39046
  %_44.i65.i.2 = load i32, ptr %442, align 4, !dbg !39046, !noalias !39034, !noundef !12
  %_43.i66.i.2 = zext i32 %_44.i65.i.2 to i64, !dbg !39046
  %443 = add i64 %ring_cursor.sroa.0.1.i9468277, %_43.i66.i.2, !dbg !39099
  %_47.not.i67.i.2 = icmp ult i64 %443, %_91.i, !dbg !39100
  %444 = select i1 %_47.not.i67.i.2, i64 0, i64 %_91.i, !dbg !39100
  %spec.select.i68.i.2 = sub nuw i64 %443, %444, !dbg !39100
  %_51.i69.i.2 = mul i64 %spec.select.i68.i.2, %width.i33.i, !dbg !39101
  %_50.i70.i.2 = add i64 %_51.i69.i.2, 2, !dbg !39101
  %_53.i72.i.2 = icmp ult i64 %_50.i70.i.2, %_149.1.i82.i.pre, !dbg !39102
  br i1 %_53.i72.i.2, label %bb21.i74.i.2, label %panic1.i73.i, !dbg !39102

bb21.i74.i.2:                                     ; preds = %bb17.i63.i.2
  %445 = getelementptr inbounds nuw float, ptr %_147.0.i75.i, i64 %_50.i70.i.2, !dbg !39102
  %_49.i76.i.2 = load float, ptr %445, align 4, !dbg !39102, !noalias !39034, !noundef !12
  store float %_49.i76.i.2, ptr %iter.i32.i.sroa.0.0.ptr8263.2, align 4, !dbg !39103, !noalias !39034
  %446 = icmp eq i64 %width.i33.i, 3, !dbg !39043
  br i1 %446, label %bb16.i77.i, label %bb39.i57.i.3, !dbg !39043

bb39.i57.i.3:                                     ; preds = %bb21.i74.i.2
  %exitcond11774.3.not = icmp eq i64 %_145.1.i60.i, 3, !dbg !39046
  br i1 %exitcond11774.3.not, label %panic.i62.i, label %bb17.i63.i.3, !dbg !39046

bb17.i63.i.3:                                     ; preds = %bb39.i57.i.3
  %447 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i, i64 44, !dbg !39046
  %_44.i65.i.3 = load i32, ptr %447, align 4, !dbg !39046, !noalias !39034, !noundef !12
  %_43.i66.i.3 = zext i32 %_44.i65.i.3 to i64, !dbg !39046
  %448 = add i64 %ring_cursor.sroa.0.1.i9468277, %_43.i66.i.3, !dbg !39099
  %_47.not.i67.i.3 = icmp ult i64 %448, %_91.i, !dbg !39100
  %449 = select i1 %_47.not.i67.i.3, i64 0, i64 %_91.i, !dbg !39100
  %spec.select.i68.i.3 = sub nuw i64 %448, %449, !dbg !39100
  %_51.i69.i.3 = mul i64 %spec.select.i68.i.3, %width.i33.i, !dbg !39101
  %_50.i70.i.3 = add i64 %_51.i69.i.3, 3, !dbg !39101
  %_53.i72.i.3 = icmp ult i64 %_50.i70.i.3, %_149.1.i82.i.pre, !dbg !39102
  br i1 %_53.i72.i.3, label %bb21.i74.i.3, label %panic1.i73.i, !dbg !39102

bb21.i74.i.3:                                     ; preds = %bb17.i63.i.3
  %450 = getelementptr inbounds nuw float, ptr %_147.0.i75.i, i64 %_50.i70.i.3, !dbg !39102
  %_49.i76.i.3 = load float, ptr %450, align 4, !dbg !39102, !noalias !39034, !noundef !12
  store float %_49.i76.i.3, ptr %iter.i32.i.sroa.0.0.ptr8263.3, align 4, !dbg !39103, !noalias !39034
  %451 = icmp eq i64 %width.i33.i, 4, !dbg !39043
  br i1 %451, label %bb16.i77.i, label %bb39.i57.i.4, !dbg !39043

bb39.i57.i.4:                                     ; preds = %bb21.i74.i.3
  %exitcond11774.4.not = icmp eq i64 %_145.1.i60.i, 4, !dbg !39046
  br i1 %exitcond11774.4.not, label %panic.i62.i, label %bb17.i63.i.4, !dbg !39046

bb17.i63.i.4:                                     ; preds = %bb39.i57.i.4
  %452 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i, i64 56, !dbg !39046
  %_44.i65.i.4 = load i32, ptr %452, align 4, !dbg !39046, !noalias !39034, !noundef !12
  %_43.i66.i.4 = zext i32 %_44.i65.i.4 to i64, !dbg !39046
  %453 = add i64 %ring_cursor.sroa.0.1.i9468277, %_43.i66.i.4, !dbg !39099
  %_47.not.i67.i.4 = icmp ult i64 %453, %_91.i, !dbg !39100
  %454 = select i1 %_47.not.i67.i.4, i64 0, i64 %_91.i, !dbg !39100
  %spec.select.i68.i.4 = sub nuw i64 %453, %454, !dbg !39100
  %_51.i69.i.4 = mul i64 %spec.select.i68.i.4, %width.i33.i, !dbg !39101
  %_50.i70.i.4 = add i64 %_51.i69.i.4, 4, !dbg !39101
  %_53.i72.i.4 = icmp ult i64 %_50.i70.i.4, %_149.1.i82.i.pre, !dbg !39102
  br i1 %_53.i72.i.4, label %bb21.i74.i.4, label %panic1.i73.i, !dbg !39102

bb21.i74.i.4:                                     ; preds = %bb17.i63.i.4
  %455 = getelementptr inbounds nuw float, ptr %_147.0.i75.i, i64 %_50.i70.i.4, !dbg !39102
  %_49.i76.i.4 = load float, ptr %455, align 4, !dbg !39102, !noalias !39034, !noundef !12
  store float %_49.i76.i.4, ptr %iter.i32.i.sroa.0.0.ptr8263.4, align 4, !dbg !39103, !noalias !39034
  %456 = icmp eq i64 %width.i33.i, 5, !dbg !39043
  br i1 %456, label %bb16.i77.i, label %bb39.i57.i.5, !dbg !39043

bb39.i57.i.5:                                     ; preds = %bb21.i74.i.4
  %exitcond11774.5.not = icmp eq i64 %_145.1.i60.i, 5, !dbg !39046
  br i1 %exitcond11774.5.not, label %panic.i62.i, label %bb17.i63.i.5, !dbg !39046

bb17.i63.i.5:                                     ; preds = %bb39.i57.i.5
  %457 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i, i64 68, !dbg !39046
  %_44.i65.i.5 = load i32, ptr %457, align 4, !dbg !39046, !noalias !39034, !noundef !12
  %_43.i66.i.5 = zext i32 %_44.i65.i.5 to i64, !dbg !39046
  %458 = add i64 %ring_cursor.sroa.0.1.i9468277, %_43.i66.i.5, !dbg !39099
  %_47.not.i67.i.5 = icmp ult i64 %458, %_91.i, !dbg !39100
  %459 = select i1 %_47.not.i67.i.5, i64 0, i64 %_91.i, !dbg !39100
  %spec.select.i68.i.5 = sub nuw i64 %458, %459, !dbg !39100
  %_51.i69.i.5 = mul i64 %spec.select.i68.i.5, %width.i33.i, !dbg !39101
  %_50.i70.i.5 = add i64 %_51.i69.i.5, 5, !dbg !39101
  %_53.i72.i.5 = icmp ult i64 %_50.i70.i.5, %_149.1.i82.i.pre, !dbg !39102
  br i1 %_53.i72.i.5, label %bb21.i74.i.5, label %panic1.i73.i, !dbg !39102

bb21.i74.i.5:                                     ; preds = %bb17.i63.i.5
  %460 = getelementptr inbounds nuw float, ptr %_147.0.i75.i, i64 %_50.i70.i.5, !dbg !39102
  %_49.i76.i.5 = load float, ptr %460, align 4, !dbg !39102, !noalias !39034, !noundef !12
  store float %_49.i76.i.5, ptr %iter.i32.i.sroa.0.0.ptr8263.5, align 4, !dbg !39103, !noalias !39034
  %461 = icmp eq i64 %width.i33.i, 6, !dbg !39043
  br i1 %461, label %bb16.i77.i, label %bb39.i57.i.6, !dbg !39043

bb39.i57.i.6:                                     ; preds = %bb21.i74.i.5
  %exitcond11774.6.not = icmp eq i64 %_145.1.i60.i, 6, !dbg !39046
  br i1 %exitcond11774.6.not, label %panic.i62.i, label %bb17.i63.i.6, !dbg !39046

bb17.i63.i.6:                                     ; preds = %bb39.i57.i.6
  %462 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i, i64 80, !dbg !39046
  %_44.i65.i.6 = load i32, ptr %462, align 4, !dbg !39046, !noalias !39034, !noundef !12
  %_43.i66.i.6 = zext i32 %_44.i65.i.6 to i64, !dbg !39046
  %463 = add i64 %ring_cursor.sroa.0.1.i9468277, %_43.i66.i.6, !dbg !39099
  %_47.not.i67.i.6 = icmp ult i64 %463, %_91.i, !dbg !39100
  %464 = select i1 %_47.not.i67.i.6, i64 0, i64 %_91.i, !dbg !39100
  %spec.select.i68.i.6 = sub nuw i64 %463, %464, !dbg !39100
  %_51.i69.i.6 = mul i64 %spec.select.i68.i.6, %width.i33.i, !dbg !39101
  %_50.i70.i.6 = add i64 %_51.i69.i.6, 6, !dbg !39101
  %_53.i72.i.6 = icmp ult i64 %_50.i70.i.6, %_149.1.i82.i.pre, !dbg !39102
  br i1 %_53.i72.i.6, label %bb21.i74.i.6, label %panic1.i73.i, !dbg !39102

bb21.i74.i.6:                                     ; preds = %bb17.i63.i.6
  %465 = getelementptr inbounds nuw float, ptr %_147.0.i75.i, i64 %_50.i70.i.6, !dbg !39102
  %_49.i76.i.6 = load float, ptr %465, align 4, !dbg !39102, !noalias !39034, !noundef !12
  store float %_49.i76.i.6, ptr %iter.i32.i.sroa.0.0.ptr8263.6, align 4, !dbg !39103, !noalias !39034
  %466 = icmp eq i64 %width.i33.i, 7, !dbg !39043
  br i1 %466, label %bb16.i77.i, label %bb39.i57.i.7, !dbg !39043

bb39.i57.i.7:                                     ; preds = %bb21.i74.i.6
  %exitcond11774.7.not = icmp eq i64 %_145.1.i60.i, 7, !dbg !39046
  br i1 %exitcond11774.7.not, label %panic.i62.i, label %bb17.i63.i.7, !dbg !39046

bb17.i63.i.7:                                     ; preds = %bb39.i57.i.7
  %467 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i, i64 92, !dbg !39046
  %_44.i65.i.7 = load i32, ptr %467, align 4, !dbg !39046, !noalias !39034, !noundef !12
  %_43.i66.i.7 = zext i32 %_44.i65.i.7 to i64, !dbg !39046
  %468 = add i64 %ring_cursor.sroa.0.1.i9468277, %_43.i66.i.7, !dbg !39099
  %_47.not.i67.i.7 = icmp ult i64 %468, %_91.i, !dbg !39100
  %469 = select i1 %_47.not.i67.i.7, i64 0, i64 %_91.i, !dbg !39100
  %spec.select.i68.i.7 = sub nuw i64 %468, %469, !dbg !39100
  %_51.i69.i.7 = mul i64 %spec.select.i68.i.7, %width.i33.i, !dbg !39101
  %_50.i70.i.7 = add i64 %_51.i69.i.7, 7, !dbg !39101
  %_53.i72.i.7 = icmp ult i64 %_50.i70.i.7, %_149.1.i82.i.pre, !dbg !39102
  br i1 %_53.i72.i.7, label %bb21.i74.i.7, label %panic1.i73.i, !dbg !39102

bb21.i74.i.7:                                     ; preds = %bb17.i63.i.7
  %470 = getelementptr inbounds nuw float, ptr %_147.0.i75.i, i64 %_50.i70.i.7, !dbg !39102
  %_49.i76.i.7 = load float, ptr %470, align 4, !dbg !39102, !noalias !39034, !noundef !12
  store float %_49.i76.i.7, ptr %iter.i32.i.sroa.0.0.ptr8263.7, align 4, !dbg !39103, !noalias !39034
  br label %bb16.i77.i, !dbg !39043

panic1.i73.i:                                     ; preds = %bb17.i63.i.7, %bb17.i63.i.6, %bb17.i63.i.5, %bb17.i63.i.4, %bb17.i63.i.3, %bb17.i63.i.2, %bb17.i63.i.1, %bb17.i63.i
  %_50.i70.i.lcssa.ph = phi i64 [ %_50.i70.i.7, %bb17.i63.i.7 ], [ %_50.i70.i.6, %bb17.i63.i.6 ], [ %_50.i70.i.5, %bb17.i63.i.5 ], [ %_50.i70.i.4, %bb17.i63.i.4 ], [ %_50.i70.i.3, %bb17.i63.i.3 ], [ %_50.i70.i.2, %bb17.i63.i.2 ], [ %_50.i70.i.1, %bb17.i63.i.1 ], [ %_51.i69.i, %bb17.i63.i ]
  store float %_0.i37728351, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i70.i.lcssa.ph, i64 noundef %_149.1.i82.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b305c1483509cfb31fdec21ff8752674) #30, !dbg !39102, !noalias !39034
  unreachable, !dbg !39102

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3677: ; preds = %bb49.i104.i
  %_127.i107.i = getelementptr inbounds nuw float, ptr %_150.0.i105.i, i64 %_76.i102.i, !dbg !39104
  %_0.i3524 = load float, ptr %_127.i107.i, align 4, !dbg !39095, !alias.scope !39091, !noalias !39034, !noundef !12
  store float %_0.i3533, ptr %_127.i107.i, align 4, !dbg !39106, !alias.scope !39108, !noalias !39034
  %_0.i3072 = fmul float %_0.i3395, %_0.i3524, !dbg !39111
  %_6.i3943 = bitcast float %_0.i3524 to i32, !dbg !39113
  %_5.i3944 = and i32 %_6.i3943, %all.sroa.0.0.i739, !dbg !39116
  %_8.i3945 = bitcast float %_0.i3072 to i32, !dbg !39117
  %_7.i3947 = and i32 %_9.i3946, %_8.i3945, !dbg !39119
  %_4.i3948 = or disjoint i32 %_7.i3947, %_5.i3944, !dbg !39116
  store i32 %_4.i3948, ptr %_174.i, align 4, !dbg !39120, !alias.scope !39122, !noalias !39125
  %_175.i = icmp ugt i64 %_64.i, %right_io.1, !dbg !39126
  br i1 %_175.i, label %bb54.i977, label %bb55.i, !dbg !39126, !prof !1406

bb52.i:                                           ; preds = %bb48.i
  store float %_0.i37728351, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_64.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_422064f3ca430d31d9007f55b436c6ca) #30, !dbg !39130, !noalias !38366
  unreachable, !dbg !39130

bb55.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3677
  %_181.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %_64.i, !dbg !39131
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39136), !dbg !39139
  %_3.not.i3517 = icmp eq i64 %right_io.1, %_64.i, !dbg !39140
  br i1 %_3.not.i3517, label %panic.i3520, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3521, !dbg !39140

panic.i3520:                                      ; preds = %bb55.i
  store float %_0.i3772, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !39140, !noalias !39142
  unreachable, !dbg !39140

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3521: ; preds = %bb55.i
  %_0.i3519 = load float, ptr %_181.i, align 4, !dbg !39140, !alias.scope !39136, !noalias !38366, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39143), !dbg !39146
  %width.i.i = load i64, ptr %374, align 8, !dbg !39147, !alias.scope !39148, !noalias !39149, !noundef !12
  %_3.i2507 = fcmp uge float %_8.i.i950, %_0.i3962, !dbg !39153
  %_0.i2940 = fdiv float %_8.i.i950, %_0.i3962, !dbg !39155
  %_0.i3942 = select i1 %_3.i2507, float 1.000000e+00, float %_0.i2940, !dbg !39157
  %_144.1.i.i = load i64, ptr %375, align 8, !dbg !39159, !alias.scope !39148, !noalias !39149, !noundef !12
  %_22.i.i961 = mul i64 %width.i.i, %ring_cursor.sroa.0.1.i9468277, !dbg !39160
  %_92.i.i = icmp ugt i64 %_22.i.i961, %_144.1.i.i, !dbg !39161
  br i1 %_92.i.i, label %bb37.i.i, label %bb38.i.i, !dbg !39161, !prof !1406

bb38.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3521
  %_144.0.i.i = load ptr, ptr %376, align 8, !dbg !39159, !alias.scope !39148, !noalias !39149, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39164), !dbg !39167
  %_4.not.i3670 = icmp eq i64 %_144.1.i.i, %_22.i.i961, !dbg !39168
  br i1 %_4.not.i3670, label %panic.i3672, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3673, !dbg !39168

panic.i3672:                                      ; preds = %bb38.i.i
  store float %_0.i3772, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !39168, !noalias !39170
  unreachable, !dbg !39168

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3673: ; preds = %bb38.i.i
  %_99.i.i = getelementptr inbounds nuw float, ptr %_144.0.i.i, i64 %_22.i.i961, !dbg !39171
  store float %_0.i3942, ptr %_99.i.i, align 4, !dbg !39168, !alias.scope !39164, !noalias !39173
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39174), !dbg !39177
  %width.i2117 = load i64, ptr %374, align 8, !dbg !39178, !alias.scope !39174, !noalias !39180, !noundef !12
  %471 = icmp eq i64 %width.i2117, 0, !dbg !39182
  br i1 %471, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2225, label %bb32.i2124.lr.ph, !dbg !39182

bb32.i2124.lr.ph:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3673
  %_112.1.i2127 = load i64, ptr %377, align 8, !alias.scope !39174, !noalias !39180, !noundef !12
  %_112.0.i2131 = load ptr, ptr %378, align 8, !nonnull !12
  %472 = add i64 %ring_cursor.sroa.0.1.i9468277, 1
  %_23.not.i2138 = icmp ult i64 %472, %_91.i
  %473 = select i1 %_23.not.i2138, i64 0, i64 %_91.i
  %start1.sroa.0.0.i2139 = sub nuw i64 %472, %473
  %_114.1.i2142 = load i64, ptr %375, align 8
  %_114.0.i2146 = load ptr, ptr %376, align 8, !nonnull !12
  %_116.1.i2147 = load i64, ptr %379, align 8
  %_116.0.i2151 = load ptr, ptr %380, align 8, !nonnull !12
  %_118.1.i2155 = load i64, ptr %381, align 8
  %_118.0.i2159 = load ptr, ptr %382, align 8, !nonnull !12
  %_45.i2172 = mul i64 %width.i2117, %start1.sroa.0.0.i2139
  br label %bb32.i2124, !dbg !39182

bb32.i2124:                                       ; preds = %bb32.i2124.lr.ph, %bb31.i2187
  %iter.i2116.sroa.10.08269 = phi i64 [ %width.i2117, %bb32.i2124.lr.ph ], [ %474, %bb31.i2187 ]
  %iter.i2116.sroa.7.08268 = phi i64 [ 0, %bb32.i2124.lr.ph ], [ %_9.0.i4800, %bb31.i2187 ]
  %iter.i2116.sroa.0.0.idx8267 = phi i64 [ 0, %bb32.i2124.lr.ph ], [ %iter.i2116.sroa.0.0.add, %bb31.i2187 ]
  %iter.i2116.sroa.0.0.ptr8270 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 %iter.i2116.sroa.0.0.idx8267, !dbg !39184
  %474 = add i64 %iter.i2116.sroa.10.08269, -1, !dbg !39184
  %_7.i.i4796 = icmp eq i64 %iter.i2116.sroa.0.0.idx8267, 32, !dbg !39185
  br i1 %_7.i.i4796, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2225.loopexit, label %bb3.i2126, !dbg !39189

bb3.i2126:                                        ; preds = %bb32.i2124
  %iter.i2116.sroa.0.0.add = add nuw nsw i64 %iter.i2116.sroa.0.0.idx8267, 4, !dbg !39190
  %_9.0.i4800 = add nuw nsw i64 %iter.i2116.sroa.7.08268, 1, !dbg !39192
  %exitcond11777.not = icmp eq i64 %iter.i2116.sroa.7.08268, %_112.1.i2127, !dbg !39193
  br i1 %exitcond11777.not, label %panic.i2129, label %bb5.i2130, !dbg !39193

bb5.i2130:                                        ; preds = %bb3.i2126
  %475 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i2131, i64 %iter.i2116.sroa.7.08268, !dbg !39193
  %shape.i2132 = load i32, ptr %475, align 4, !dbg !39193, !noalias !39194, !noundef !12
  %476 = getelementptr inbounds nuw i8, ptr %475, i64 4, !dbg !39193
  %shape3.i2133 = load i32, ptr %476, align 4, !dbg !39193, !noalias !39194, !noundef !12
  %window.i2134 = zext i32 %shape.i2132 to i64, !dbg !39195
  %_19.i2135 = zext i32 %shape3.i2133 to i64, !dbg !39196
  %477 = add i64 %ring_cursor.sroa.0.1.i9468277, %_19.i2135, !dbg !39197
  %_20.not.i2136 = icmp ult i64 %477, %_91.i, !dbg !39198
  %478 = select i1 %_20.not.i2136, i64 0, i64 %_91.i, !dbg !39198
  %spec.select.i2137 = sub nuw i64 %477, %478, !dbg !39198
  %_27.i2140 = mul i64 %spec.select.i2137, %width.i2117, !dbg !39199
  %_26.i2141 = add i64 %_27.i2140, %iter.i2116.sroa.7.08268, !dbg !39199
  %_30.i2143 = icmp ult i64 %_26.i2141, %_114.1.i2142, !dbg !39200
  br i1 %_30.i2143, label %bb12.i2145, label %panic5.i2144, !dbg !39200

panic.i2129:                                      ; preds = %bb3.i2126
  store float %_0.i3772, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i2127, i64 noundef %_112.1.i2127, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4a8785a681d008a9bfd0cd82628ea9cb) #30, !dbg !39193, !noalias !39194
  unreachable, !dbg !39193

bb12.i2145:                                       ; preds = %bb5.i2130
  %479 = getelementptr inbounds nuw float, ptr %_114.0.i2146, i64 %_26.i2141, !dbg !39200
  %480 = load float, ptr %479, align 4, !dbg !39200, !noalias !39194, !noundef !12
  %exitcond11778.not = icmp eq i64 %iter.i2116.sroa.7.08268, %_116.1.i2147, !dbg !39201
  br i1 %exitcond11778.not, label %panic6.i2149, label %bb13.i2150, !dbg !39201

panic5.i2144:                                     ; preds = %bb5.i2130
  store float %_0.i3772, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i2141, i64 noundef %_114.1.i2142, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cbce7773ac40979e4ba2385da3aec116) #30, !dbg !39200, !noalias !39194
  unreachable, !dbg !39200

bb13.i2150:                                       ; preds = %bb12.i2145
  %481 = getelementptr inbounds nuw i32, ptr %_116.0.i2151, i64 %iter.i2116.sroa.7.08268, !dbg !39201
  %_32.i2152 = load i32, ptr %481, align 4, !dbg !39201, !noalias !39194, !noundef !12
  %position.i2153 = zext i32 %_32.i2152 to i64, !dbg !39201
  %482 = icmp eq i32 %_32.i2152, 0, !dbg !39202
  br i1 %482, label %bb17.i2162, label %bb15.i2154, !dbg !39202

panic6.i2149:                                     ; preds = %bb12.i2145
  store float %_0.i3772, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i2147, i64 noundef %_116.1.i2147, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ec0d48f73ebfc2755df5cedaa60b5c0a) #30, !dbg !39201, !noalias !39194
  unreachable, !dbg !39201

bb15.i2154:                                       ; preds = %bb13.i2150
  %_37.i2156 = icmp ult i64 %iter.i2116.sroa.7.08268, %_118.1.i2155, !dbg !39203
  br i1 %_37.i2156, label %bb16.i2158, label %panic7.i2157, !dbg !39203

bb17.i2162:                                       ; preds = %bb35.i2223, %bb16.i2158, %bb13.i2150
  %newest.sroa.0.0.i2163 = phi float [ %480, %bb13.i2150 ], [ %_35.i2160, %bb35.i2223 ], [ %480, %bb16.i2158 ], !dbg !39204
  %exitcond11779.not = icmp eq i64 %iter.i2116.sroa.7.08268, %_118.1.i2155, !dbg !39205
  br i1 %exitcond11779.not, label %panic8.i2166, label %bb18.i2167, !dbg !39205

bb16.i2158:                                       ; preds = %bb15.i2154
  %483 = getelementptr inbounds nuw float, ptr %_118.0.i2159, i64 %iter.i2116.sroa.7.08268, !dbg !39203
  %_35.i2160 = load float, ptr %483, align 4, !dbg !39203, !noalias !39194, !noundef !12
  %_102.i2161 = fcmp olt float %_35.i2160, %480, !dbg !39206
  br i1 %_102.i2161, label %bb35.i2223, label %bb17.i2162, !dbg !39206

panic7.i2157:                                     ; preds = %bb15.i2154
  store float %_0.i3772, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i2116.sroa.7.08268, i64 noundef %_118.1.i2155, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2c461872bb652d4796cdcf89c28c82c8) #30, !dbg !39203, !noalias !39194
  unreachable, !dbg !39203

bb35.i2223:                                       ; preds = %bb16.i2158
  br label %bb17.i2162, !dbg !39208

bb18.i2167:                                       ; preds = %bb17.i2162
  %484 = getelementptr inbounds nuw float, ptr %_118.0.i2159, i64 %iter.i2116.sroa.7.08268, !dbg !39205
  store float %newest.sroa.0.0.i2163, ptr %484, align 4, !dbg !39205, !noalias !39194
  %_42.i2169 = add nuw nsw i64 %position.i2153, 1, !dbg !39209
  %complete.i2170 = icmp eq i64 %_42.i2169, %window.i2134, !dbg !39209
  br i1 %complete.i2170, label %bb22.i2192, label %bb20.i2171, !dbg !39210

panic8.i2166:                                     ; preds = %bb17.i2162
  store float %_0.i3772, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i2155, i64 noundef %_118.1.i2155, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2b690e2c7763f11809942906fc2ca813) #30, !dbg !39205, !noalias !39194
  unreachable, !dbg !39205

bb20.i2171:                                       ; preds = %bb18.i2167
  %_44.i2173 = add i64 %iter.i2116.sroa.7.08268, %_45.i2172, !dbg !39211
  %_47.i2175 = icmp ult i64 %_44.i2173, %_114.1.i2142, !dbg !39212
  br i1 %_47.i2175, label %bb30.i2185, label %panic9.i2176, !dbg !39212

panic9.i2176:                                     ; preds = %bb20.i2171
  store float %_0.i3772, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i2173, i64 noundef %_114.1.i2142, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9fa421ae81817f58fcfc4a3243223891) #30, !dbg !39212, !noalias !39194
  unreachable, !dbg !39212

bb30.i2185:                                       ; preds = %bb20.i2171
  %485 = getelementptr inbounds nuw float, ptr %_114.0.i2146, i64 %_44.i2173, !dbg !39212
  %_43.i2179 = load float, ptr %485, align 4, !dbg !39212, !noalias !39194, !noundef !12
  %_103.i2180 = fcmp olt float %_43.i2179, %newest.sroa.0.0.i2163, !dbg !39213
  %newest.sroa.0.1.i2181 = select i1 %_103.i2180, float %_43.i2179, float %newest.sroa.0.0.i2163, !dbg !39213
  store float %newest.sroa.0.1.i2181, ptr %iter.i2116.sroa.0.0.ptr8270, align 4, !dbg !39215, !noalias !39194
  %486 = trunc i64 %_42.i2169 to i32, !dbg !39216
  br label %bb31.i2187, !dbg !39217

bb31.i2187:                                       ; preds = %bb25.i2220, %bb30.i2185
  %storemerge5682 = phi i32 [ %486, %bb30.i2185 ], [ 0, %bb25.i2220 ], !dbg !39218
  store i32 %storemerge5682, ptr %481, align 4, !dbg !39218, !noalias !39194
  %487 = icmp eq i64 %474, 0, !dbg !39182
  br i1 %487, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2225.loopexit, label %bb32.i2124, !dbg !39182

bb22.i2192:                                       ; preds = %bb18.i2167
  store float %newest.sroa.0.0.i2163, ptr %iter.i2116.sroa.0.0.ptr8270, align 4, !dbg !39215, !noalias !39194
  %488 = load float, ptr %479, align 4, !dbg !39219, !noalias !39194, !noundef !12
  br label %bb41.i2205, !dbg !39220

bb41.i2205:                                       ; preds = %bb22.i2192, %bb25.i2220
  %iter2.sroa.0.0.i21978266 = phi i64 [ 0, %bb22.i2192 ], [ %_105.i2206, %bb25.i2220 ]
  %suffix.sroa.0.0.i21968265 = phi float [ %488, %bb22.i2192 ], [ %suffix.sroa.0.1.i2216, %bb25.i2220 ]
  %end.sroa.0.1.i21958264 = phi i64 [ %spec.select.i2137, %bb22.i2192 ], [ %491, %bb25.i2220 ]
  %_56.i2207 = mul i64 %end.sroa.0.1.i21958264, %width.i2117, !dbg !39223
  %_55.i2208 = add i64 %_56.i2207, %iter.i2116.sroa.7.08268, !dbg !39223
  %_59.i2210 = icmp ult i64 %_55.i2208, %_114.1.i2142, !dbg !39224
  br i1 %_59.i2210, label %bb25.i2220, label %panic13.i2211, !dbg !39224

panic13.i2211:                                    ; preds = %bb41.i2205
  store float %_0.i3772, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i2208, i64 noundef %_114.1.i2142, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_91a4c6b9b17ebf4d863f9a70b6dc929a) #30, !dbg !39224, !noalias !39194
  unreachable, !dbg !39224

bb25.i2220:                                       ; preds = %bb41.i2205
  %_105.i2206 = add nuw nsw i64 %iter2.sroa.0.0.i21978266, 1, !dbg !39225
  %489 = getelementptr inbounds nuw float, ptr %_114.0.i2146, i64 %_55.i2208, !dbg !39224
  %_54.i2214 = load float, ptr %489, align 4, !dbg !39224, !noalias !39194, !noundef !12
  %_107.i2215 = fcmp olt float %suffix.sroa.0.0.i21968265, %_54.i2214, !dbg !39228
  %suffix.sroa.0.1.i2216 = select i1 %_107.i2215, float %suffix.sroa.0.0.i21968265, float %_54.i2214, !dbg !39228
  store float %suffix.sroa.0.1.i2216, ptr %489, align 4, !dbg !39230, !noalias !39194
  %490 = icmp eq i64 %end.sroa.0.1.i21958264, 0, !dbg !39231
  %spec.store.select.i2222 = select i1 %490, i64 %_91.i, i64 %end.sroa.0.1.i21958264, !dbg !39231
  %491 = add i64 %spec.store.select.i2222, -1, !dbg !39232
  %exitcond11776.not = icmp eq i64 %_105.i2206, %window.i2134, !dbg !39233
  br i1 %exitcond11776.not, label %bb31.i2187, label %bb41.i2205, !dbg !39220

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2225.loopexit: ; preds = %bb32.i2124, %bb31.i2187
  %_0.i3516.pre = load float, ptr %scratch.i, align 4, !dbg !39235, !alias.scope !39237, !noalias !39240
  br label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2225, !dbg !39235

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2225: ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2225.loopexit, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3673
  %_0.i3516 = phi float [ %_0.i3516.pre, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2225.loopexit ], [ %_0.i3528, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3673 ], !dbg !39235
  %_0.i3071 = fmul float %_0.i3516, 1.638400e+04, !dbg !39241
  %492 = tail call noundef float @llvm.floor.f32(float %_0.i3071), !dbg !39243
  %_0.i3070 = fmul float %492, 0x3F10000000000000, !dbg !39247
  %493 = icmp eq i64 %width.i.i, 0, !dbg !39249
  %_149.1.i.i.pre = load i64, ptr %383, align 8, !dbg !39251, !alias.scope !39148, !noalias !39149
  br i1 %493, label %bb16.i.i, label %bb39.i.i.lr.ph, !dbg !39249

bb39.i.i.lr.ph:                                   ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2225
  %_145.1.i.i = load i64, ptr %377, align 8, !alias.scope !39148, !noalias !39149, !noundef !12
  %_145.0.i.i = load ptr, ptr %378, align 8, !nonnull !12
  %_147.0.i.i = load ptr, ptr %384, align 8, !nonnull !12
  %exitcond11780.not = icmp eq i64 %_145.1.i.i, 0, !dbg !39252
  br i1 %exitcond11780.not, label %panic.i.i, label %bb17.i.i, !dbg !39252

bb37.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3521
  store float %_0.i3772, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i961, i64 noundef %_144.1.i.i, i64 noundef %_144.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_56df7c041d29359441bca272bf4e38e3) #30, !dbg !39253, !noalias !39173
  unreachable, !dbg !39253

bb16.i.i:                                         ; preds = %bb21.i.i.7, %bb21.i.i, %bb21.i.i.1, %bb21.i.i.2, %bb21.i.i.3, %bb21.i.i.4, %bb21.i.i.5, %bb21.i.i.6, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2225
  %_0.i3514 = phi float [ %_0.i3516, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2225 ], [ %_49.i.i970, %bb21.i.i ], [ %_49.i.i970, %bb21.i.i.7 ], [ %_49.i.i970, %bb21.i.i.6 ], [ %_49.i.i970, %bb21.i.i.5 ], [ %_49.i.i970, %bb21.i.i.4 ], [ %_49.i.i970, %bb21.i.i.3 ], [ %_49.i.i970, %bb21.i.i.2 ], [ %_49.i.i970, %bb21.i.i.1 ], !dbg !39254
  %_0.i2641 = fadd float %_0.i3070, %_0.i33948353, !dbg !39256
  %_0.i3394 = fsub float %_0.i2641, %_0.i3514, !dbg !39258
  %_109.i.i = icmp ugt i64 %_22.i.i961, %_149.1.i.i.pre, !dbg !39260
  br i1 %_109.i.i, label %bb42.i.i, label %bb43.i.i, !dbg !39260, !prof !1406

bb43.i.i:                                         ; preds = %bb16.i.i
  %_149.0.i.i = load ptr, ptr %384, align 8, !dbg !39251, !alias.scope !39148, !noalias !39149, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39263), !dbg !39266
  %_4.not.i3666 = icmp eq i64 %_149.1.i.i.pre, %_22.i.i961, !dbg !39267
  br i1 %_4.not.i3666, label %panic.i3668, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3669, !dbg !39267

panic.i3668:                                      ; preds = %bb43.i.i
  store float %_0.i3772, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !39267, !noalias !39269
  unreachable, !dbg !39267

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3669: ; preds = %bb43.i.i
  %_116.i.i = getelementptr inbounds nuw float, ptr %_149.0.i.i, i64 %_22.i.i961, !dbg !39270
  store float %_0.i3070, ptr %_116.i.i, align 4, !dbg !39267, !alias.scope !39263, !noalias !39240
  %_0.i2939 = fdiv float %_0.i3394, %_64.i.i, !dbg !39272
  %_0.i3393 = fsub float 1.000000e+00, %_0.i2939, !dbg !39274
  %_0.i3392 = fsub float %_0.i3393, %_0.i37688422, !dbg !39276
  %_4.i2955 = fmul float %_9.i.i951, %_0.i3392, !dbg !39278
  %_0.i2956 = fadd float %_0.i37688422, %_4.i2955, !dbg !39278
  %_3.i.i4145.inv = fcmp ogt float %_0.i3393, %_0.i2956, !dbg !39280
  %_4.i.i4152.v = select i1 %_3.i.i4145.inv, float %_0.i3393, float %_0.i2956, !dbg !39280
  %494 = tail call noundef float @llvm.fabs.f32(float %_4.i.i4152.v), !dbg !39283
  %495 = fcmp uge float %494, 0x3BC79CA100000000, !dbg !39286
  %_0.i3768 = select i1 %495, float %_4.i.i4152.v, float 0.000000e+00, !dbg !39288
  %_0.i3391 = fsub float 1.000000e+00, %_0.i3768, !dbg !39289
  %_150.1.i.i = load i64, ptr %388, align 8, !dbg !39291, !alias.scope !39148, !noalias !39149, !noundef !12
  %_76.i.i = mul i64 %width.i.i, %main_cursor.sroa.0.1.i9478278, !dbg !39292
  %_120.i.i = icmp ugt i64 %_76.i.i, %_150.1.i.i, !dbg !39293
  br i1 %_120.i.i, label %bb48.i.i, label %bb49.i.i, !dbg !39293, !prof !1406

bb42.i.i:                                         ; preds = %bb16.i.i
  store float %_0.i3772, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i961, i64 noundef %_149.1.i.i.pre, i64 noundef %_149.1.i.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_90498045d73339daaf9e4f537508f58b) #30, !dbg !39296, !noalias !39240
  unreachable, !dbg !39296

bb49.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3669
  %_150.0.i.i = load ptr, ptr %389, align 8, !dbg !39291, !alias.scope !39148, !noalias !39149, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39297), !dbg !39300
  %_3.not.i3508 = icmp eq i64 %_150.1.i.i, %_76.i.i, !dbg !39301
  br i1 %_3.not.i3508, label %panic.i3511, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3661, !dbg !39301

panic.i3511:                                      ; preds = %bb49.i.i
  store float %_0.i3772, ptr %371, align 1, !dbg !38319
  store float %_0.i3768, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !39301, !noalias !39303
  unreachable, !dbg !39301

bb48.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3669
  store float %_0.i3772, ptr %371, align 1, !dbg !38319
  store float %_0.i3768, ptr %387, align 1, !dbg !38335
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i.i, i64 noundef %_150.1.i.i, i64 noundef %_150.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_da8af254b6d507a8e2ca31e544bfd21d) #30, !dbg !39304, !noalias !39240
  unreachable, !dbg !39304

bb17.i.i:                                         ; preds = %bb39.i.i.lr.ph
  %496 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 8, !dbg !39252
  %_44.i.i967 = load i32, ptr %496, align 4, !dbg !39252, !noalias !39240, !noundef !12
  %_43.i.i968 = zext i32 %_44.i.i967 to i64, !dbg !39252
  %497 = add i64 %ring_cursor.sroa.0.1.i9468277, %_43.i.i968, !dbg !39305
  %_47.not.i.i = icmp ult i64 %497, %_91.i, !dbg !39306
  %498 = select i1 %_47.not.i.i, i64 0, i64 %_91.i, !dbg !39306
  %spec.select.i.i = sub nuw i64 %497, %498, !dbg !39306
  %_51.i.i = mul i64 %spec.select.i.i, %width.i.i, !dbg !39307
  %_53.i.i969 = icmp ult i64 %_51.i.i, %_149.1.i.i.pre, !dbg !39308
  br i1 %_53.i.i969, label %bb21.i.i, label %panic1.i.i, !dbg !39308

panic.i.i:                                        ; preds = %bb39.i.i.7, %bb39.i.i.6, %bb39.i.i.5, %bb39.i.i.4, %bb39.i.i.3, %bb39.i.i.2, %bb39.i.i.1, %bb39.i.i.lr.ph
  %_145.1.i.i.lcssa.ph = phi i64 [ 7, %bb39.i.i.7 ], [ 6, %bb39.i.i.6 ], [ 5, %bb39.i.i.5 ], [ 4, %bb39.i.i.4 ], [ 3, %bb39.i.i.3 ], [ 2, %bb39.i.i.2 ], [ 1, %bb39.i.i.1 ], [ 0, %bb39.i.i.lr.ph ]
  store float %_0.i3772, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i.i.lcssa.ph, i64 noundef %_145.1.i.i.lcssa.ph, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a6feb40b34112df5f214f84424dd2c7c) #30, !dbg !39252, !noalias !39240
  unreachable, !dbg !39252

bb21.i.i:                                         ; preds = %bb17.i.i
  %499 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_51.i.i, !dbg !39308
  %_49.i.i970 = load float, ptr %499, align 4, !dbg !39308, !noalias !39240, !noundef !12
  store float %_49.i.i970, ptr %scratch.i, align 4, !dbg !39309, !noalias !39240
  %500 = icmp eq i64 %width.i.i, 1, !dbg !39249
  br i1 %500, label %bb16.i.i, label %bb39.i.i.1, !dbg !39249

bb39.i.i.1:                                       ; preds = %bb21.i.i
  %exitcond11780.1.not = icmp eq i64 %_145.1.i.i, 1, !dbg !39252
  br i1 %exitcond11780.1.not, label %panic.i.i, label %bb17.i.i.1, !dbg !39252

bb17.i.i.1:                                       ; preds = %bb39.i.i.1
  %501 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 20, !dbg !39252
  %_44.i.i967.1 = load i32, ptr %501, align 4, !dbg !39252, !noalias !39240, !noundef !12
  %_43.i.i968.1 = zext i32 %_44.i.i967.1 to i64, !dbg !39252
  %502 = add i64 %ring_cursor.sroa.0.1.i9468277, %_43.i.i968.1, !dbg !39305
  %_47.not.i.i.1 = icmp ult i64 %502, %_91.i, !dbg !39306
  %503 = select i1 %_47.not.i.i.1, i64 0, i64 %_91.i, !dbg !39306
  %spec.select.i.i.1 = sub nuw i64 %502, %503, !dbg !39306
  %_51.i.i.1 = mul i64 %spec.select.i.i.1, %width.i.i, !dbg !39307
  %_50.i.i.1 = add i64 %_51.i.i.1, 1, !dbg !39307
  %_53.i.i969.1 = icmp ult i64 %_50.i.i.1, %_149.1.i.i.pre, !dbg !39308
  br i1 %_53.i.i969.1, label %bb21.i.i.1, label %panic1.i.i, !dbg !39308

bb21.i.i.1:                                       ; preds = %bb17.i.i.1
  %504 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.1, !dbg !39308
  %_49.i.i970.1 = load float, ptr %504, align 4, !dbg !39308, !noalias !39240, !noundef !12
  store float %_49.i.i970.1, ptr %iter.i.i728.sroa.0.0.ptr8274.1, align 4, !dbg !39309, !noalias !39240
  %505 = icmp eq i64 %width.i.i, 2, !dbg !39249
  br i1 %505, label %bb16.i.i, label %bb39.i.i.2, !dbg !39249

bb39.i.i.2:                                       ; preds = %bb21.i.i.1
  %exitcond11780.2.not = icmp eq i64 %_145.1.i.i, 2, !dbg !39252
  br i1 %exitcond11780.2.not, label %panic.i.i, label %bb17.i.i.2, !dbg !39252

bb17.i.i.2:                                       ; preds = %bb39.i.i.2
  %506 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 32, !dbg !39252
  %_44.i.i967.2 = load i32, ptr %506, align 4, !dbg !39252, !noalias !39240, !noundef !12
  %_43.i.i968.2 = zext i32 %_44.i.i967.2 to i64, !dbg !39252
  %507 = add i64 %ring_cursor.sroa.0.1.i9468277, %_43.i.i968.2, !dbg !39305
  %_47.not.i.i.2 = icmp ult i64 %507, %_91.i, !dbg !39306
  %508 = select i1 %_47.not.i.i.2, i64 0, i64 %_91.i, !dbg !39306
  %spec.select.i.i.2 = sub nuw i64 %507, %508, !dbg !39306
  %_51.i.i.2 = mul i64 %spec.select.i.i.2, %width.i.i, !dbg !39307
  %_50.i.i.2 = add i64 %_51.i.i.2, 2, !dbg !39307
  %_53.i.i969.2 = icmp ult i64 %_50.i.i.2, %_149.1.i.i.pre, !dbg !39308
  br i1 %_53.i.i969.2, label %bb21.i.i.2, label %panic1.i.i, !dbg !39308

bb21.i.i.2:                                       ; preds = %bb17.i.i.2
  %509 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.2, !dbg !39308
  %_49.i.i970.2 = load float, ptr %509, align 4, !dbg !39308, !noalias !39240, !noundef !12
  store float %_49.i.i970.2, ptr %iter.i.i728.sroa.0.0.ptr8274.2, align 4, !dbg !39309, !noalias !39240
  %510 = icmp eq i64 %width.i.i, 3, !dbg !39249
  br i1 %510, label %bb16.i.i, label %bb39.i.i.3, !dbg !39249

bb39.i.i.3:                                       ; preds = %bb21.i.i.2
  %exitcond11780.3.not = icmp eq i64 %_145.1.i.i, 3, !dbg !39252
  br i1 %exitcond11780.3.not, label %panic.i.i, label %bb17.i.i.3, !dbg !39252

bb17.i.i.3:                                       ; preds = %bb39.i.i.3
  %511 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 44, !dbg !39252
  %_44.i.i967.3 = load i32, ptr %511, align 4, !dbg !39252, !noalias !39240, !noundef !12
  %_43.i.i968.3 = zext i32 %_44.i.i967.3 to i64, !dbg !39252
  %512 = add i64 %ring_cursor.sroa.0.1.i9468277, %_43.i.i968.3, !dbg !39305
  %_47.not.i.i.3 = icmp ult i64 %512, %_91.i, !dbg !39306
  %513 = select i1 %_47.not.i.i.3, i64 0, i64 %_91.i, !dbg !39306
  %spec.select.i.i.3 = sub nuw i64 %512, %513, !dbg !39306
  %_51.i.i.3 = mul i64 %spec.select.i.i.3, %width.i.i, !dbg !39307
  %_50.i.i.3 = add i64 %_51.i.i.3, 3, !dbg !39307
  %_53.i.i969.3 = icmp ult i64 %_50.i.i.3, %_149.1.i.i.pre, !dbg !39308
  br i1 %_53.i.i969.3, label %bb21.i.i.3, label %panic1.i.i, !dbg !39308

bb21.i.i.3:                                       ; preds = %bb17.i.i.3
  %514 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.3, !dbg !39308
  %_49.i.i970.3 = load float, ptr %514, align 4, !dbg !39308, !noalias !39240, !noundef !12
  store float %_49.i.i970.3, ptr %iter.i.i728.sroa.0.0.ptr8274.3, align 4, !dbg !39309, !noalias !39240
  %515 = icmp eq i64 %width.i.i, 4, !dbg !39249
  br i1 %515, label %bb16.i.i, label %bb39.i.i.4, !dbg !39249

bb39.i.i.4:                                       ; preds = %bb21.i.i.3
  %exitcond11780.4.not = icmp eq i64 %_145.1.i.i, 4, !dbg !39252
  br i1 %exitcond11780.4.not, label %panic.i.i, label %bb17.i.i.4, !dbg !39252

bb17.i.i.4:                                       ; preds = %bb39.i.i.4
  %516 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 56, !dbg !39252
  %_44.i.i967.4 = load i32, ptr %516, align 4, !dbg !39252, !noalias !39240, !noundef !12
  %_43.i.i968.4 = zext i32 %_44.i.i967.4 to i64, !dbg !39252
  %517 = add i64 %ring_cursor.sroa.0.1.i9468277, %_43.i.i968.4, !dbg !39305
  %_47.not.i.i.4 = icmp ult i64 %517, %_91.i, !dbg !39306
  %518 = select i1 %_47.not.i.i.4, i64 0, i64 %_91.i, !dbg !39306
  %spec.select.i.i.4 = sub nuw i64 %517, %518, !dbg !39306
  %_51.i.i.4 = mul i64 %spec.select.i.i.4, %width.i.i, !dbg !39307
  %_50.i.i.4 = add i64 %_51.i.i.4, 4, !dbg !39307
  %_53.i.i969.4 = icmp ult i64 %_50.i.i.4, %_149.1.i.i.pre, !dbg !39308
  br i1 %_53.i.i969.4, label %bb21.i.i.4, label %panic1.i.i, !dbg !39308

bb21.i.i.4:                                       ; preds = %bb17.i.i.4
  %519 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.4, !dbg !39308
  %_49.i.i970.4 = load float, ptr %519, align 4, !dbg !39308, !noalias !39240, !noundef !12
  store float %_49.i.i970.4, ptr %iter.i.i728.sroa.0.0.ptr8274.4, align 4, !dbg !39309, !noalias !39240
  %520 = icmp eq i64 %width.i.i, 5, !dbg !39249
  br i1 %520, label %bb16.i.i, label %bb39.i.i.5, !dbg !39249

bb39.i.i.5:                                       ; preds = %bb21.i.i.4
  %exitcond11780.5.not = icmp eq i64 %_145.1.i.i, 5, !dbg !39252
  br i1 %exitcond11780.5.not, label %panic.i.i, label %bb17.i.i.5, !dbg !39252

bb17.i.i.5:                                       ; preds = %bb39.i.i.5
  %521 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 68, !dbg !39252
  %_44.i.i967.5 = load i32, ptr %521, align 4, !dbg !39252, !noalias !39240, !noundef !12
  %_43.i.i968.5 = zext i32 %_44.i.i967.5 to i64, !dbg !39252
  %522 = add i64 %ring_cursor.sroa.0.1.i9468277, %_43.i.i968.5, !dbg !39305
  %_47.not.i.i.5 = icmp ult i64 %522, %_91.i, !dbg !39306
  %523 = select i1 %_47.not.i.i.5, i64 0, i64 %_91.i, !dbg !39306
  %spec.select.i.i.5 = sub nuw i64 %522, %523, !dbg !39306
  %_51.i.i.5 = mul i64 %spec.select.i.i.5, %width.i.i, !dbg !39307
  %_50.i.i.5 = add i64 %_51.i.i.5, 5, !dbg !39307
  %_53.i.i969.5 = icmp ult i64 %_50.i.i.5, %_149.1.i.i.pre, !dbg !39308
  br i1 %_53.i.i969.5, label %bb21.i.i.5, label %panic1.i.i, !dbg !39308

bb21.i.i.5:                                       ; preds = %bb17.i.i.5
  %524 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.5, !dbg !39308
  %_49.i.i970.5 = load float, ptr %524, align 4, !dbg !39308, !noalias !39240, !noundef !12
  store float %_49.i.i970.5, ptr %iter.i.i728.sroa.0.0.ptr8274.5, align 4, !dbg !39309, !noalias !39240
  %525 = icmp eq i64 %width.i.i, 6, !dbg !39249
  br i1 %525, label %bb16.i.i, label %bb39.i.i.6, !dbg !39249

bb39.i.i.6:                                       ; preds = %bb21.i.i.5
  %exitcond11780.6.not = icmp eq i64 %_145.1.i.i, 6, !dbg !39252
  br i1 %exitcond11780.6.not, label %panic.i.i, label %bb17.i.i.6, !dbg !39252

bb17.i.i.6:                                       ; preds = %bb39.i.i.6
  %526 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 80, !dbg !39252
  %_44.i.i967.6 = load i32, ptr %526, align 4, !dbg !39252, !noalias !39240, !noundef !12
  %_43.i.i968.6 = zext i32 %_44.i.i967.6 to i64, !dbg !39252
  %527 = add i64 %ring_cursor.sroa.0.1.i9468277, %_43.i.i968.6, !dbg !39305
  %_47.not.i.i.6 = icmp ult i64 %527, %_91.i, !dbg !39306
  %528 = select i1 %_47.not.i.i.6, i64 0, i64 %_91.i, !dbg !39306
  %spec.select.i.i.6 = sub nuw i64 %527, %528, !dbg !39306
  %_51.i.i.6 = mul i64 %spec.select.i.i.6, %width.i.i, !dbg !39307
  %_50.i.i.6 = add i64 %_51.i.i.6, 6, !dbg !39307
  %_53.i.i969.6 = icmp ult i64 %_50.i.i.6, %_149.1.i.i.pre, !dbg !39308
  br i1 %_53.i.i969.6, label %bb21.i.i.6, label %panic1.i.i, !dbg !39308

bb21.i.i.6:                                       ; preds = %bb17.i.i.6
  %529 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.6, !dbg !39308
  %_49.i.i970.6 = load float, ptr %529, align 4, !dbg !39308, !noalias !39240, !noundef !12
  store float %_49.i.i970.6, ptr %iter.i.i728.sroa.0.0.ptr8274.6, align 4, !dbg !39309, !noalias !39240
  %530 = icmp eq i64 %width.i.i, 7, !dbg !39249
  br i1 %530, label %bb16.i.i, label %bb39.i.i.7, !dbg !39249

bb39.i.i.7:                                       ; preds = %bb21.i.i.6
  %exitcond11780.7.not = icmp eq i64 %_145.1.i.i, 7, !dbg !39252
  br i1 %exitcond11780.7.not, label %panic.i.i, label %bb17.i.i.7, !dbg !39252

bb17.i.i.7:                                       ; preds = %bb39.i.i.7
  %531 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 92, !dbg !39252
  %_44.i.i967.7 = load i32, ptr %531, align 4, !dbg !39252, !noalias !39240, !noundef !12
  %_43.i.i968.7 = zext i32 %_44.i.i967.7 to i64, !dbg !39252
  %532 = add i64 %ring_cursor.sroa.0.1.i9468277, %_43.i.i968.7, !dbg !39305
  %_47.not.i.i.7 = icmp ult i64 %532, %_91.i, !dbg !39306
  %533 = select i1 %_47.not.i.i.7, i64 0, i64 %_91.i, !dbg !39306
  %spec.select.i.i.7 = sub nuw i64 %532, %533, !dbg !39306
  %_51.i.i.7 = mul i64 %spec.select.i.i.7, %width.i.i, !dbg !39307
  %_50.i.i.7 = add i64 %_51.i.i.7, 7, !dbg !39307
  %_53.i.i969.7 = icmp ult i64 %_50.i.i.7, %_149.1.i.i.pre, !dbg !39308
  br i1 %_53.i.i969.7, label %bb21.i.i.7, label %panic1.i.i, !dbg !39308

bb21.i.i.7:                                       ; preds = %bb17.i.i.7
  %534 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.7, !dbg !39308
  %_49.i.i970.7 = load float, ptr %534, align 4, !dbg !39308, !noalias !39240, !noundef !12
  store float %_49.i.i970.7, ptr %iter.i.i728.sroa.0.0.ptr8274.7, align 4, !dbg !39309, !noalias !39240
  br label %bb16.i.i, !dbg !39249

panic1.i.i:                                       ; preds = %bb17.i.i.7, %bb17.i.i.6, %bb17.i.i.5, %bb17.i.i.4, %bb17.i.i.3, %bb17.i.i.2, %bb17.i.i.1, %bb17.i.i
  %_50.i.i.lcssa.ph = phi i64 [ %_50.i.i.7, %bb17.i.i.7 ], [ %_50.i.i.6, %bb17.i.i.6 ], [ %_50.i.i.5, %bb17.i.i.5 ], [ %_50.i.i.4, %bb17.i.i.4 ], [ %_50.i.i.3, %bb17.i.i.3 ], [ %_50.i.i.2, %bb17.i.i.2 ], [ %_50.i.i.1, %bb17.i.i.1 ], [ %_51.i.i, %bb17.i.i ]
  store float %_0.i3772, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i.i.lcssa.ph, i64 noundef %_149.1.i.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b305c1483509cfb31fdec21ff8752674) #30, !dbg !39308, !noalias !39240
  unreachable, !dbg !39308

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3661: ; preds = %bb49.i.i
  %_127.i.i = getelementptr inbounds nuw float, ptr %_150.0.i.i, i64 %_76.i.i, !dbg !39310
  %_0.i3510 = load float, ptr %_127.i.i, align 4, !dbg !39301, !alias.scope !39297, !noalias !39240, !noundef !12
  store float %_0.i3519, ptr %_127.i.i, align 4, !dbg !39312, !alias.scope !39314, !noalias !39240
  %_0.i3069 = fmul float %_0.i3391, %_0.i3510, !dbg !39317
  %_6.i3930 = bitcast float %_0.i3510 to i32, !dbg !39319
  %_5.i3931 = and i32 %_6.i3930, %all.sroa.0.0.i739, !dbg !39322
  %_8.i3932 = bitcast float %_0.i3069 to i32, !dbg !39323
  %_7.i3934 = and i32 %_9.i3946, %_8.i3932, !dbg !39325
  %_4.i3935 = or disjoint i32 %_7.i3934, %_5.i3931, !dbg !39322
  store i32 %_4.i3935, ptr %_181.i, align 4, !dbg !39326, !alias.scope !39328, !noalias !39331
  %535 = add i64 %main_cursor.sroa.0.1.i9478278, 1, !dbg !39332
  %_104.i = icmp eq i64 %535, %_106.i, !dbg !39333
  %spec.store.select.i = select i1 %_104.i, i64 0, i64 %535, !dbg !39333
  %536 = add i64 %ring_cursor.sroa.0.1.i9468277, 1, !dbg !39334
  %_107.i = icmp eq i64 %536, %_91.i, !dbg !39335
  %spec.store.select13.i = select i1 %_107.i, i64 0, i64 %536, !dbg !39335
  %exitcond11783.not = icmp eq i64 %406, %umax11782, !dbg !39336
  br i1 %exitcond11783.not, label %bb16.i.bb13.i.loopexit_crit_edge, label %bb48.i, !dbg !38339

bb54.i977:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3677
  store float %_0.i3772, ptr %371, align 1, !dbg !38319
  store float %_0.i37688422, ptr %387, align 1, !dbg !38335
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_64.i, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_54f0ebc64763f3f022885a8e201aab88) #30, !dbg !39339, !noalias !38366
  unreachable, !dbg !39339

_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit.loopexit: ; preds = %bb13.i.loopexit
  %537 = trunc i64 %main_cursor.sroa.0.1.i947.lcssa to i32, !dbg !39340
  %538 = trunc i64 %ring_cursor.sroa.0.1.i946.lcssa to i32, !dbg !39341
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit, !dbg !39342

_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit.loopexit, %bb11.i
  %ring_cursor.sroa.0.0.i746.lcssa = phi i32 [ %_36.i740, %bb11.i ], [ %538, %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit.loopexit ], !dbg !38291
  %main_cursor.sroa.0.0.i747.lcssa = phi i32 [ %_34.i, %bb11.i ], [ %537, %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit.loopexit ], !dbg !38288
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %hot_left.i732, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33) #31, !dbg !39343
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %hot_right.i731, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34) #31, !dbg !39344
  store i32 %main_cursor.sroa.0.0.i747.lcssa, ptr %_35, align 4, !dbg !39340, !alias.scope !38270, !noalias !38290
  store i32 %ring_cursor.sroa.0.0.i746.lcssa, ptr %319, align 4, !dbg !39341, !alias.scope !38270, !noalias !38290
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i729), !dbg !39345, !noalias !38295
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i730), !dbg !39346, !noalias !38295
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i), !dbg !39347, !noalias !38295
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockfEB2_.exit, !dbg !38265

bb7.i:                                            ; preds = %bb5.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39348), !dbg !39351
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39352), !dbg !39351
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39354), !dbg !39351
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39356), !dbg !39351
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39358), !dbg !39351
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i56), !dbg !39360, !noalias !39364
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_left.i56, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #31, !dbg !39367, !noalias !39368
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_right.i55), !dbg !39369, !noalias !39364
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_right.i55, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #31, !dbg !39371, !noalias !39372
  %539 = getelementptr inbounds nuw i8, ptr %self, i64 776, !dbg !39373
  %540 = load i8, ptr %539, align 4, !dbg !39373, !range !17, !alias.scope !39348, !noalias !39377, !noundef !12
  %541 = getelementptr inbounds nuw i8, ptr %self, i64 777, !dbg !39378
  %542 = load i8, ptr %541, align 1, !dbg !39378, !range !17, !alias.scope !39348, !noalias !39377, !noundef !12
  %543 = getelementptr inbounds nuw i8, ptr %self, i64 544, !dbg !39380
  %ring.i64 = load i64, ptr %543, align 8, !dbg !39380, !alias.scope !39352, !noalias !39382, !noundef !12
  %544 = getelementptr inbounds nuw i8, ptr %self, i64 552, !dbg !39383
  %main.i65 = load i64, ptr %544, align 8, !dbg !39383, !alias.scope !39352, !noalias !39382, !noundef !12
  %_35.i66 = load i32, ptr %_35, align 4, !dbg !39385, !alias.scope !39358, !noalias !39387, !noundef !12
  %545 = getelementptr inbounds nuw i8, ptr %self, i64 564, !dbg !39388
  %_36.i67 = load i32, ptr %545, align 4, !dbg !39388, !alias.scope !39358, !noalias !39387, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i54), !dbg !39390, !noalias !39364
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i54, i8 0, i64 1024, i1 false), !noalias !39364
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i53), !dbg !39392, !noalias !39364
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i53, i8 0, i64 1024, i1 false), !noalias !39364
  %_31.i60 = zext nneg i8 %540 to i32, !dbg !39373
  %.none.i61 = sub nsw i32 0, %_31.i60, !dbg !39394
  %_32.i62 = zext nneg i8 %542 to i32, !dbg !39378
  %all.sroa.0.0.i63 = sub nsw i32 0, %_32.i62, !dbg !39378
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i52), !dbg !39395, !noalias !39364
; call <true_peak_limiter::UniformHot<f32>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotfE3newB5_(ptr noalias noundef align 8 captures(none) dereferenceable(80) %uniform_left.i52, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33, i64 %ring.i64, i64 %main.i65) #31, !dbg !39397
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_right.i51), !dbg !39398, !noalias !39364
  %_32.val = load i64, ptr %543, align 8, !dbg !39400, !noundef !12
  %_32.val4389 = load i64, ptr %544, align 8, !dbg !39400, !noundef !12
; call <true_peak_limiter::UniformHot<f32>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotfE3newB5_(ptr noalias noundef align 8 captures(none) dereferenceable(80) %uniform_right.i51, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34, i64 %_32.val, i64 %_32.val4389) #31, !dbg !39400
  %_167.not.i789045 = icmp eq i64 %frames, 0, !dbg !39401
  br i1 %_167.not.i789045, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit, label %bb50.i79.lr.ph, !dbg !39401

bb50.i79.lr.ph:                                   ; preds = %bb7.i
  %546 = zext i32 %_36.i67 to i64, !dbg !39388
  %547 = zext i32 %_35.i66 to i64, !dbg !39385
  %d9.i.i4816 = lshr i64 %frames, 5, !dbg !39411
  %r2.i.i4817 = and i64 %frames, 31, !dbg !39417
  %_19.not.i.i4818 = icmp ne i64 %r2.i.i4817, 0, !dbg !39418
  %548 = zext i1 %_19.not.i.i4818 to i64, !dbg !39418
  %yield_count.sroa.0.0.i.i4819 = add nuw nsw i64 %d9.i.i4816, %548, !dbg !39418
  %history.i44.i18.sroa.7.0.hot_left.i56.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i56, i64 4
  %history.i44.i18.sroa.10.0.hot_left.i56.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i56, i64 8
  %history.i44.i18.sroa.13.0.hot_left.i56.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i56, i64 12
  %history.i44.i18.sroa.16.0.hot_left.i56.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i56, i64 16
  %history.i44.i18.sroa.19.0.hot_left.i56.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i56, i64 20
  %history.i44.i18.sroa.22.0.hot_left.i56.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i56, i64 24
  %history.i44.i18.sroa.26.0.hot_left.i56.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i56, i64 28
  %history.i44.i18.sroa.29.0.hot_left.i56.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i56, i64 32
  %history.i44.i18.sroa.32.0.hot_left.i56.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i56, i64 36
  %history.i44.i18.sroa.35.0.hot_left.i56.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i56, i64 40
  %history.i44.i18.sroa.38.0.hot_left.i56.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i56, i64 44
  %549 = getelementptr inbounds nuw i8, ptr %self, i64 588
  %550 = getelementptr inbounds nuw i8, ptr %self, i64 592
  %551 = getelementptr inbounds nuw i8, ptr %self, i64 596
  %row1.i.i.i77.i124 = getelementptr inbounds nuw i8, ptr %self, i64 600
  %552 = getelementptr inbounds nuw i8, ptr %self, i64 604
  %553 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %554 = getelementptr inbounds nuw i8, ptr %self, i64 612
  %row3.i.i.i91.i138 = getelementptr inbounds nuw i8, ptr %self, i64 616
  %555 = getelementptr inbounds nuw i8, ptr %self, i64 620
  %556 = getelementptr inbounds nuw i8, ptr %self, i64 624
  %557 = getelementptr inbounds nuw i8, ptr %self, i64 628
  %row5.i.i.i105.i152 = getelementptr inbounds nuw i8, ptr %self, i64 632
  %558 = getelementptr inbounds nuw i8, ptr %self, i64 636
  %559 = getelementptr inbounds nuw i8, ptr %self, i64 640
  %560 = getelementptr inbounds nuw i8, ptr %self, i64 644
  %row7.i.i.i119.i166 = getelementptr inbounds nuw i8, ptr %self, i64 648
  %561 = getelementptr inbounds nuw i8, ptr %self, i64 652
  %562 = getelementptr inbounds nuw i8, ptr %self, i64 656
  %563 = getelementptr inbounds nuw i8, ptr %self, i64 660
  %row9.i.i.i133.i180 = getelementptr inbounds nuw i8, ptr %self, i64 664
  %564 = getelementptr inbounds nuw i8, ptr %self, i64 668
  %565 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %566 = getelementptr inbounds nuw i8, ptr %self, i64 676
  %row11.i.i.i147.i194 = getelementptr inbounds nuw i8, ptr %self, i64 680
  %567 = getelementptr inbounds nuw i8, ptr %self, i64 684
  %568 = getelementptr inbounds nuw i8, ptr %self, i64 688
  %569 = getelementptr inbounds nuw i8, ptr %self, i64 692
  %row13.i.i.i161.i208 = getelementptr inbounds nuw i8, ptr %self, i64 696
  %570 = getelementptr inbounds nuw i8, ptr %self, i64 700
  %571 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %572 = getelementptr inbounds nuw i8, ptr %self, i64 708
  %row15.i.i.i175.i222 = getelementptr inbounds nuw i8, ptr %self, i64 712
  %573 = getelementptr inbounds nuw i8, ptr %self, i64 716
  %574 = getelementptr inbounds nuw i8, ptr %self, i64 720
  %575 = getelementptr inbounds nuw i8, ptr %self, i64 724
  %row17.i.i.i189.i236 = getelementptr inbounds nuw i8, ptr %self, i64 728
  %576 = getelementptr inbounds nuw i8, ptr %self, i64 732
  %577 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %578 = getelementptr inbounds nuw i8, ptr %self, i64 740
  %row19.i.i.i203.i250 = getelementptr inbounds nuw i8, ptr %self, i64 744
  %579 = getelementptr inbounds nuw i8, ptr %self, i64 748
  %580 = getelementptr inbounds nuw i8, ptr %self, i64 752
  %581 = getelementptr inbounds nuw i8, ptr %self, i64 756
  %row21.i.i.i217.i264 = getelementptr inbounds nuw i8, ptr %self, i64 760
  %582 = getelementptr inbounds nuw i8, ptr %self, i64 764
  %583 = getelementptr inbounds nuw i8, ptr %self, i64 768
  %584 = getelementptr inbounds nuw i8, ptr %self, i64 772
  %history.i.i32.sroa.7.0.hot_right.i55.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i55, i64 4
  %history.i.i32.sroa.10.0.hot_right.i55.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i55, i64 8
  %history.i.i32.sroa.13.0.hot_right.i55.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i55, i64 12
  %history.i.i32.sroa.16.0.hot_right.i55.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i55, i64 16
  %history.i.i32.sroa.19.0.hot_right.i55.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i55, i64 20
  %history.i.i32.sroa.22.0.hot_right.i55.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i55, i64 24
  %history.i.i32.sroa.26.0.hot_right.i55.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i55, i64 28
  %history.i.i32.sroa.29.0.hot_right.i55.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i55, i64 32
  %history.i.i32.sroa.32.0.hot_right.i55.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i55, i64 36
  %history.i.i32.sroa.35.0.hot_right.i55.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i55, i64 40
  %history.i.i32.sroa.38.0.hot_right.i55.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i55, i64 44
  %585 = getelementptr inbounds nuw i8, ptr %uniform_left.i52, i64 48
  %_70.i48.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i52, i64 56
  %_70.i48.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i52, i64 64
  %586 = getelementptr inbounds nuw i8, ptr %uniform_right.i51, i64 48
  %_71.i47.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i51, i64 56
  %_71.i47.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i51, i64 64
  %_114.i523 = getelementptr inbounds nuw i8, ptr %hot_left.i56, i64 48
  %_115.i524 = getelementptr inbounds nuw i8, ptr %hot_left.i56, i64 64
  %587 = getelementptr inbounds nuw i8, ptr %hot_left.i56, i64 60
  %588 = getelementptr inbounds nuw i8, ptr %hot_left.i56, i64 56
  %589 = getelementptr inbounds nuw i8, ptr %hot_left.i56, i64 52
  %590 = getelementptr inbounds nuw i8, ptr %hot_left.i56, i64 76
  %591 = getelementptr inbounds nuw i8, ptr %hot_left.i56, i64 72
  %592 = getelementptr inbounds nuw i8, ptr %hot_left.i56, i64 68
  %_119.i525 = getelementptr inbounds nuw i8, ptr %hot_right.i55, i64 48
  %_120.i526 = getelementptr inbounds nuw i8, ptr %hot_right.i55, i64 64
  %593 = getelementptr inbounds nuw i8, ptr %hot_right.i55, i64 60
  %594 = getelementptr inbounds nuw i8, ptr %hot_right.i55, i64 56
  %595 = getelementptr inbounds nuw i8, ptr %hot_right.i55, i64 52
  %596 = getelementptr inbounds nuw i8, ptr %hot_right.i55, i64 76
  %597 = getelementptr inbounds nuw i8, ptr %hot_right.i55, i64 72
  %598 = getelementptr inbounds nuw i8, ptr %hot_right.i55, i64 68
  %_9.i4006 = add nsw i32 %_31.i60, -1
  %599 = getelementptr inbounds nuw i8, ptr %uniform_left.i52, i64 8
  %_21.i264.i562 = getelementptr inbounds nuw i8, ptr %uniform_left.i52, i64 72
  %_22.i265.i563 = getelementptr inbounds nuw i8, ptr %uniform_left.i52, i64 76
  %600 = getelementptr inbounds nuw i8, ptr %uniform_left.i52, i64 16
  %601 = getelementptr inbounds nuw i8, ptr %uniform_left.i52, i64 24
  %602 = getelementptr inbounds nuw i8, ptr %hot_left.i56, i64 84
  %603 = getelementptr inbounds nuw i8, ptr %hot_left.i56, i64 88
  %604 = getelementptr inbounds nuw i8, ptr %hot_left.i56, i64 80
  %605 = getelementptr inbounds nuw i8, ptr %uniform_left.i52, i64 40
  %606 = getelementptr inbounds nuw i8, ptr %uniform_left.i52, i64 32
  %_9.i3986 = add nsw i32 %_32.i62, -1
  %607 = getelementptr inbounds nuw i8, ptr %uniform_right.i51, i64 8
  %_21.i.i637 = getelementptr inbounds nuw i8, ptr %uniform_right.i51, i64 72
  %_22.i.i638 = getelementptr inbounds nuw i8, ptr %uniform_right.i51, i64 76
  %608 = getelementptr inbounds nuw i8, ptr %uniform_right.i51, i64 16
  %609 = getelementptr inbounds nuw i8, ptr %uniform_right.i51, i64 24
  %610 = getelementptr inbounds nuw i8, ptr %hot_right.i55, i64 84
  %611 = getelementptr inbounds nuw i8, ptr %hot_right.i55, i64 88
  %612 = getelementptr inbounds nuw i8, ptr %hot_right.i55, i64 80
  %613 = getelementptr inbounds nuw i8, ptr %uniform_right.i51, i64 40
  %614 = getelementptr inbounds nuw i8, ptr %uniform_right.i51, i64 32
  br label %bb50.i79, !dbg !39401

bb19.i485.bb15.i73.loopexit_crit_edge:            ; preds = %bb32.i690
  store float %_0.i.i.lcssa1247414043, ptr %587, align 4
  store float %_0.i.i4065.lcssa1239014076, ptr %593, align 4
  store float %running.sroa.0.0.i1694.lcssa1248614120, ptr %_21.i264.i562, align 1, !dbg !39419
  store float %running.sroa.0.0.i.lcssa1252314137, ptr %_21.i.i637, align 1, !dbg !39452
  store i32 %storemerge.i1699.lcssa88678968, ptr %_22.i265.i563, align 4
  store i32 %storemerge.i.lcssa89259007, ptr %_22.i.i638, align 4
  br label %bb15.i73.loopexit, !dbg !39455

bb15.i73.loopexit:                                ; preds = %bb19.i485.bb15.i73.loopexit_crit_edge, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i484
  %ring_cursor.sroa.0.1.i486.lcssa = phi i64 [ %ring_cursor.sroa.0.2.i693, %bb19.i485.bb15.i73.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i749046, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i484 ], !dbg !39456
  %main_cursor.sroa.0.1.i487.lcssa = phi i64 [ %main_cursor.sroa.0.2.i696, %bb19.i485.bb15.i73.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i759047, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i484 ], !dbg !39457
  %_167.not.i78 = icmp eq i64 %616, 0, !dbg !39401
  %indvars.iv.next11785 = add i64 %indvars.iv11784, -32, !dbg !39401
  br i1 %_167.not.i78, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit.loopexit, label %bb50.i79, !dbg !39401

bb50.i79:                                         ; preds = %bb50.i79.lr.ph, %bb15.i73.loopexit
  %indvars.iv11784 = phi i64 [ %frames, %bb50.i79.lr.ph ], [ %indvars.iv.next11785, %bb15.i73.loopexit ]
  %iter2.sroa.0.0.i779049 = phi i64 [ %yield_count.sroa.0.0.i.i4819, %bb50.i79.lr.ph ], [ %616, %bb15.i73.loopexit ]
  %iter1.sroa.0.0.i769048 = phi i64 [ 0, %bb50.i79.lr.ph ], [ %615, %bb15.i73.loopexit ]
  %main_cursor.sroa.0.0.i759047 = phi i64 [ %547, %bb50.i79.lr.ph ], [ %main_cursor.sroa.0.1.i487.lcssa, %bb15.i73.loopexit ]
  %ring_cursor.sroa.0.0.i749046 = phi i64 [ %546, %bb50.i79.lr.ph ], [ %ring_cursor.sroa.0.1.i486.lcssa, %bb15.i73.loopexit ]
  %umin11804 = call i64 @llvm.umin.i64(i64 %indvars.iv11784, i64 32), !dbg !39458
  %umax11790 = call i64 @llvm.umax.i64(i64 %umin11804, i64 1), !dbg !39458
  %615 = add i64 %iter1.sroa.0.0.i769048, 32, !dbg !39458
  %616 = add i64 %iter2.sroa.0.0.i779049, -1, !dbg !39462
  %_46.i81 = sub i64 %frames, %iter1.sroa.0.0.i769048, !dbg !39463
  %..i4820 = tail call noundef i64 @llvm.umin.i64(i64 %_46.i81, i64 32), !dbg !39464
  %_52.i83 = add i64 %..i4820, %iter1.sroa.0.0.i769048, !dbg !39468
  %_179.i84 = icmp ult i64 %_52.i83, %iter1.sroa.0.0.i769048, !dbg !39469
  %_173.not.i85 = icmp ugt i64 %_52.i83, %left_io.1
  %or.cond.i86 = or i1 %_179.i84, %_173.not.i85, !dbg !39469
  br i1 %or.cond.i86, label %bb54.i700, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4846, !dbg !39469, !prof !165

bb54.i700:                                        ; preds = %bb50.i79
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %iter1.sroa.0.0.i769048, i64 noundef %_52.i83, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e7f134ea71d3d762bf72c0ef5353d5ff) #30, !dbg !39476, !noalias !39358
  unreachable, !dbg !39476

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4846: ; preds = %bb50.i79
  %_182.i91 = getelementptr inbounds nuw float, ptr %left_io.0, i64 %iter1.sroa.0.0.i769048, !dbg !39477
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39481), !dbg !39484
  %history.i44.i18.sroa.0.0.copyload = load float, ptr %hot_left.i56, align 4, !dbg !39485, !noalias !39487
  %history.i44.i18.sroa.7.0.copyload = load float, ptr %history.i44.i18.sroa.7.0.hot_left.i56.sroa_idx, align 4, !dbg !39485, !noalias !39487
  %history.i44.i18.sroa.10.0.copyload = load float, ptr %history.i44.i18.sroa.10.0.hot_left.i56.sroa_idx, align 4, !dbg !39485, !noalias !39487
  %history.i44.i18.sroa.13.0.copyload = load float, ptr %history.i44.i18.sroa.13.0.hot_left.i56.sroa_idx, align 4, !dbg !39485, !noalias !39487
  %history.i44.i18.sroa.16.0.copyload = load float, ptr %history.i44.i18.sroa.16.0.hot_left.i56.sroa_idx, align 4, !dbg !39485, !noalias !39487
  %history.i44.i18.sroa.19.0.copyload = load float, ptr %history.i44.i18.sroa.19.0.hot_left.i56.sroa_idx, align 4, !dbg !39485, !noalias !39487
  %history.i44.i18.sroa.22.0.copyload = load float, ptr %history.i44.i18.sroa.22.0.hot_left.i56.sroa_idx, align 4, !dbg !39485, !noalias !39487
  %history.i44.i18.sroa.26.0.copyload = load float, ptr %history.i44.i18.sroa.26.0.hot_left.i56.sroa_idx, align 4, !dbg !39485, !noalias !39487
  %history.i44.i18.sroa.29.0.copyload = load float, ptr %history.i44.i18.sroa.29.0.hot_left.i56.sroa_idx, align 4, !dbg !39485, !noalias !39487
  %history.i44.i18.sroa.32.0.copyload = load float, ptr %history.i44.i18.sroa.32.0.hot_left.i56.sroa_idx, align 4, !dbg !39485, !noalias !39487
  %history.i44.i18.sroa.35.0.copyload = load float, ptr %history.i44.i18.sroa.35.0.hot_left.i56.sroa_idx, align 4, !dbg !39485, !noalias !39487
  %history.i44.i18.sroa.38.0.copyload = load float, ptr %history.i44.i18.sroa.38.0.hot_left.i56.sroa_idx, align 4, !dbg !39485, !noalias !39487
  %_2.i48498432.not = icmp eq i64 %frames, %iter1.sroa.0.0.i769048, !dbg !39490
  br i1 %_2.i48498432.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit239.i286, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585.lr.ph, !dbg !39490

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585.lr.ph: ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4846
  %_11.i.i.i65.i112 = load float, ptr %_31, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_14.i.i.i68.i115 = load float, ptr %549, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_17.i.i.i71.i118 = load float, ptr %550, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_20.i.i.i74.i121 = load float, ptr %551, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_25.i.i.i79.i126 = load float, ptr %row1.i.i.i77.i124, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_28.i.i.i82.i129 = load float, ptr %552, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_31.i.i.i85.i132 = load float, ptr %553, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_34.i.i.i88.i135 = load float, ptr %554, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_39.i.i.i93.i140 = load float, ptr %row3.i.i.i91.i138, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_42.i.i.i96.i143 = load float, ptr %555, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_45.i.i.i99.i146 = load float, ptr %556, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_48.i.i.i102.i149 = load float, ptr %557, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_53.i.i.i107.i154 = load float, ptr %row5.i.i.i105.i152, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_56.i.i.i110.i157 = load float, ptr %558, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_59.i.i.i113.i160 = load float, ptr %559, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_62.i.i.i116.i163 = load float, ptr %560, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_67.i.i.i121.i168 = load float, ptr %row7.i.i.i119.i166, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_70.i.i.i124.i171 = load float, ptr %561, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_73.i.i.i127.i174 = load float, ptr %562, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_76.i.i.i130.i177 = load float, ptr %563, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_81.i.i.i135.i182 = load float, ptr %row9.i.i.i133.i180, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_84.i.i.i138.i185 = load float, ptr %564, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_87.i.i.i141.i188 = load float, ptr %565, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_90.i.i.i144.i191 = load float, ptr %566, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_95.i.i.i149.i196 = load float, ptr %row11.i.i.i147.i194, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_98.i.i.i152.i199 = load float, ptr %567, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_101.i.i.i155.i202 = load float, ptr %568, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_104.i.i.i158.i205 = load float, ptr %569, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_109.i.i.i163.i210 = load float, ptr %row13.i.i.i161.i208, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_112.i.i.i166.i213 = load float, ptr %570, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_115.i.i.i169.i216 = load float, ptr %571, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_118.i.i.i172.i219 = load float, ptr %572, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_123.i.i.i177.i224 = load float, ptr %row15.i.i.i175.i222, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_126.i.i.i180.i227 = load float, ptr %573, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_129.i.i.i183.i230 = load float, ptr %574, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_132.i.i.i186.i233 = load float, ptr %575, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_137.i.i.i191.i238 = load float, ptr %row17.i.i.i189.i236, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_140.i.i.i194.i241 = load float, ptr %576, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_143.i.i.i197.i244 = load float, ptr %577, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_146.i.i.i200.i247 = load float, ptr %578, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_151.i.i.i205.i252 = load float, ptr %row19.i.i.i203.i250, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_154.i.i.i208.i255 = load float, ptr %579, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_157.i.i.i211.i258 = load float, ptr %580, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_160.i.i.i214.i261 = load float, ptr %581, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_165.i.i.i219.i266 = load float, ptr %row21.i.i.i217.i264, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_168.i.i.i222.i269 = load float, ptr %582, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_171.i.i.i225.i272 = load float, ptr %583, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  %_174.i.i.i228.i275 = load float, ptr %584, align 4, !alias.scope !39493, !noalias !39498, !noundef !12
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585, !dbg !39490

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585
  %iter.i40.i14.sroa.16.08444 = phi i64 [ 0, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585.lr.ph ], [ %622, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585 ]
  %history.i44.i18.sroa.35.08443 = phi float [ %history.i44.i18.sroa.35.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585.lr.ph ], [ %history.i44.i18.sroa.32.08442, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585 ]
  %history.i44.i18.sroa.32.08442 = phi float [ %history.i44.i18.sroa.32.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585.lr.ph ], [ %history.i44.i18.sroa.29.08441, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585 ]
  %history.i44.i18.sroa.29.08441 = phi float [ %history.i44.i18.sroa.29.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585.lr.ph ], [ %history.i44.i18.sroa.26.08440, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585 ]
  %history.i44.i18.sroa.26.08440 = phi float [ %history.i44.i18.sroa.26.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585.lr.ph ], [ %history.i44.i18.sroa.22.08439, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585 ]
  %history.i44.i18.sroa.22.08439 = phi float [ %history.i44.i18.sroa.22.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585.lr.ph ], [ %history.i44.i18.sroa.19.08438, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585 ]
  %history.i44.i18.sroa.19.08438 = phi float [ %history.i44.i18.sroa.19.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585.lr.ph ], [ %history.i44.i18.sroa.16.08437, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585 ]
  %history.i44.i18.sroa.16.08437 = phi float [ %history.i44.i18.sroa.16.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585.lr.ph ], [ %history.i44.i18.sroa.13.08436, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585 ]
  %history.i44.i18.sroa.13.08436 = phi float [ %history.i44.i18.sroa.13.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585.lr.ph ], [ %history.i44.i18.sroa.10.08435, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585 ]
  %history.i44.i18.sroa.10.08435 = phi float [ %history.i44.i18.sroa.10.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585.lr.ph ], [ %history.i44.i18.sroa.7.08434, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585 ]
  %history.i44.i18.sroa.7.08434 = phi float [ %history.i44.i18.sroa.7.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585.lr.ph ], [ %history.i44.i18.sroa.0.08433, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585 ]
  %history.i44.i18.sroa.0.08433 = phi float [ %history.i44.i18.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585.lr.ph ], [ %_0.i3583, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585 ]
  %data.i.i4856 = getelementptr inbounds nuw float, ptr %_182.i91, i64 %iter.i40.i14.sroa.16.08444, !dbg !39503
  %_0.i3583 = load float, ptr %data.i.i4856, align 4, !dbg !39506, !alias.scope !39508, !noalias !39511, !noundef !12
  %617 = tail call noundef float @llvm.fabs.f32(float %history.i44.i18.sroa.19.08438), !dbg !39512
  %_0.i3272 = fmul float %_0.i3583, %_11.i.i.i65.i112, !dbg !39515
  %_0.i2836 = fadd float %_0.i3272, 0.000000e+00, !dbg !39518
  %_0.i3271 = fmul float %_0.i3583, %_14.i.i.i68.i115, !dbg !39520
  %_0.i2835 = fadd float %_0.i3271, 0.000000e+00, !dbg !39522
  %_0.i3270 = fmul float %_0.i3583, %_17.i.i.i71.i118, !dbg !39524
  %_0.i2834 = fadd float %_0.i3270, 0.000000e+00, !dbg !39526
  %_0.i3269 = fmul float %_0.i3583, %_20.i.i.i74.i121, !dbg !39528
  %_0.i2833 = fadd float %_0.i3269, 0.000000e+00, !dbg !39530
  %_0.i3268 = fmul float %history.i44.i18.sroa.0.08433, %_25.i.i.i79.i126, !dbg !39532
  %_0.i2832 = fadd float %_0.i2836, %_0.i3268, !dbg !39534
  %_0.i3267 = fmul float %history.i44.i18.sroa.0.08433, %_28.i.i.i82.i129, !dbg !39536
  %_0.i2831 = fadd float %_0.i2835, %_0.i3267, !dbg !39538
  %_0.i3266 = fmul float %history.i44.i18.sroa.0.08433, %_31.i.i.i85.i132, !dbg !39540
  %_0.i2830 = fadd float %_0.i2834, %_0.i3266, !dbg !39542
  %_0.i3265 = fmul float %history.i44.i18.sroa.0.08433, %_34.i.i.i88.i135, !dbg !39544
  %_0.i2829 = fadd float %_0.i2833, %_0.i3265, !dbg !39546
  %_0.i3264 = fmul float %history.i44.i18.sroa.7.08434, %_39.i.i.i93.i140, !dbg !39548
  %_0.i2828 = fadd float %_0.i2832, %_0.i3264, !dbg !39550
  %_0.i3263 = fmul float %history.i44.i18.sroa.7.08434, %_42.i.i.i96.i143, !dbg !39552
  %_0.i2827 = fadd float %_0.i2831, %_0.i3263, !dbg !39554
  %_0.i3262 = fmul float %history.i44.i18.sroa.7.08434, %_45.i.i.i99.i146, !dbg !39556
  %_0.i2826 = fadd float %_0.i2830, %_0.i3262, !dbg !39558
  %_0.i3261 = fmul float %history.i44.i18.sroa.7.08434, %_48.i.i.i102.i149, !dbg !39560
  %_0.i2825 = fadd float %_0.i2829, %_0.i3261, !dbg !39562
  %_0.i3260 = fmul float %history.i44.i18.sroa.10.08435, %_53.i.i.i107.i154, !dbg !39564
  %_0.i2824 = fadd float %_0.i2828, %_0.i3260, !dbg !39566
  %_0.i3259 = fmul float %history.i44.i18.sroa.10.08435, %_56.i.i.i110.i157, !dbg !39568
  %_0.i2823 = fadd float %_0.i2827, %_0.i3259, !dbg !39570
  %_0.i3258 = fmul float %history.i44.i18.sroa.10.08435, %_59.i.i.i113.i160, !dbg !39572
  %_0.i2822 = fadd float %_0.i2826, %_0.i3258, !dbg !39574
  %_0.i3257 = fmul float %history.i44.i18.sroa.10.08435, %_62.i.i.i116.i163, !dbg !39576
  %_0.i2821 = fadd float %_0.i2825, %_0.i3257, !dbg !39578
  %_0.i3256 = fmul float %history.i44.i18.sroa.13.08436, %_67.i.i.i121.i168, !dbg !39580
  %_0.i2820 = fadd float %_0.i2824, %_0.i3256, !dbg !39582
  %_0.i3255 = fmul float %history.i44.i18.sroa.13.08436, %_70.i.i.i124.i171, !dbg !39584
  %_0.i2819 = fadd float %_0.i2823, %_0.i3255, !dbg !39586
  %_0.i3254 = fmul float %history.i44.i18.sroa.13.08436, %_73.i.i.i127.i174, !dbg !39588
  %_0.i2818 = fadd float %_0.i2822, %_0.i3254, !dbg !39590
  %_0.i3253 = fmul float %history.i44.i18.sroa.13.08436, %_76.i.i.i130.i177, !dbg !39592
  %_0.i2817 = fadd float %_0.i2821, %_0.i3253, !dbg !39594
  %_0.i3252 = fmul float %history.i44.i18.sroa.16.08437, %_81.i.i.i135.i182, !dbg !39596
  %_0.i2816 = fadd float %_0.i2820, %_0.i3252, !dbg !39598
  %_0.i3251 = fmul float %history.i44.i18.sroa.16.08437, %_84.i.i.i138.i185, !dbg !39600
  %_0.i2815 = fadd float %_0.i2819, %_0.i3251, !dbg !39602
  %_0.i3250 = fmul float %history.i44.i18.sroa.16.08437, %_87.i.i.i141.i188, !dbg !39604
  %_0.i2814 = fadd float %_0.i2818, %_0.i3250, !dbg !39606
  %_0.i3249 = fmul float %history.i44.i18.sroa.16.08437, %_90.i.i.i144.i191, !dbg !39608
  %_0.i2813 = fadd float %_0.i2817, %_0.i3249, !dbg !39610
  %_0.i3248 = fmul float %history.i44.i18.sroa.19.08438, %_95.i.i.i149.i196, !dbg !39612
  %_0.i2812 = fadd float %_0.i2816, %_0.i3248, !dbg !39614
  %_0.i3247 = fmul float %history.i44.i18.sroa.19.08438, %_98.i.i.i152.i199, !dbg !39616
  %_0.i2811 = fadd float %_0.i2815, %_0.i3247, !dbg !39618
  %_0.i3246 = fmul float %history.i44.i18.sroa.19.08438, %_101.i.i.i155.i202, !dbg !39620
  %_0.i2810 = fadd float %_0.i2814, %_0.i3246, !dbg !39622
  %_0.i3245 = fmul float %history.i44.i18.sroa.19.08438, %_104.i.i.i158.i205, !dbg !39624
  %_0.i2809 = fadd float %_0.i2813, %_0.i3245, !dbg !39626
  %_0.i3244 = fmul float %history.i44.i18.sroa.22.08439, %_109.i.i.i163.i210, !dbg !39628
  %_0.i2808 = fadd float %_0.i2812, %_0.i3244, !dbg !39630
  %_0.i3243 = fmul float %history.i44.i18.sroa.22.08439, %_112.i.i.i166.i213, !dbg !39632
  %_0.i2807 = fadd float %_0.i2811, %_0.i3243, !dbg !39634
  %_0.i3242 = fmul float %history.i44.i18.sroa.22.08439, %_115.i.i.i169.i216, !dbg !39636
  %_0.i2806 = fadd float %_0.i2810, %_0.i3242, !dbg !39638
  %_0.i3241 = fmul float %history.i44.i18.sroa.22.08439, %_118.i.i.i172.i219, !dbg !39640
  %_0.i2805 = fadd float %_0.i2809, %_0.i3241, !dbg !39642
  %_0.i3240 = fmul float %history.i44.i18.sroa.26.08440, %_123.i.i.i177.i224, !dbg !39644
  %_0.i2804 = fadd float %_0.i2808, %_0.i3240, !dbg !39646
  %_0.i3239 = fmul float %history.i44.i18.sroa.26.08440, %_126.i.i.i180.i227, !dbg !39648
  %_0.i2803 = fadd float %_0.i2807, %_0.i3239, !dbg !39650
  %_0.i3238 = fmul float %history.i44.i18.sroa.26.08440, %_129.i.i.i183.i230, !dbg !39652
  %_0.i2802 = fadd float %_0.i2806, %_0.i3238, !dbg !39654
  %_0.i3237 = fmul float %history.i44.i18.sroa.26.08440, %_132.i.i.i186.i233, !dbg !39656
  %_0.i2801 = fadd float %_0.i2805, %_0.i3237, !dbg !39658
  %_0.i3236 = fmul float %history.i44.i18.sroa.29.08441, %_137.i.i.i191.i238, !dbg !39660
  %_0.i2800 = fadd float %_0.i2804, %_0.i3236, !dbg !39662
  %_0.i3235 = fmul float %history.i44.i18.sroa.29.08441, %_140.i.i.i194.i241, !dbg !39664
  %_0.i2799 = fadd float %_0.i2803, %_0.i3235, !dbg !39666
  %_0.i3234 = fmul float %history.i44.i18.sroa.29.08441, %_143.i.i.i197.i244, !dbg !39668
  %_0.i2798 = fadd float %_0.i2802, %_0.i3234, !dbg !39670
  %_0.i3233 = fmul float %history.i44.i18.sroa.29.08441, %_146.i.i.i200.i247, !dbg !39672
  %_0.i2797 = fadd float %_0.i2801, %_0.i3233, !dbg !39674
  %_0.i3232 = fmul float %history.i44.i18.sroa.32.08442, %_151.i.i.i205.i252, !dbg !39676
  %_0.i2796 = fadd float %_0.i2800, %_0.i3232, !dbg !39678
  %_0.i3231 = fmul float %history.i44.i18.sroa.32.08442, %_154.i.i.i208.i255, !dbg !39680
  %_0.i2795 = fadd float %_0.i2799, %_0.i3231, !dbg !39682
  %_0.i3230 = fmul float %history.i44.i18.sroa.32.08442, %_157.i.i.i211.i258, !dbg !39684
  %_0.i2794 = fadd float %_0.i2798, %_0.i3230, !dbg !39686
  %_0.i3229 = fmul float %history.i44.i18.sroa.32.08442, %_160.i.i.i214.i261, !dbg !39688
  %_0.i2793 = fadd float %_0.i2797, %_0.i3229, !dbg !39690
  %_0.i3228 = fmul float %history.i44.i18.sroa.35.08443, %_165.i.i.i219.i266, !dbg !39692
  %_0.i2792 = fadd float %_0.i2796, %_0.i3228, !dbg !39694
  %_0.i3227 = fmul float %history.i44.i18.sroa.35.08443, %_168.i.i.i222.i269, !dbg !39696
  %_0.i2791 = fadd float %_0.i2795, %_0.i3227, !dbg !39698
  %_0.i3226 = fmul float %history.i44.i18.sroa.35.08443, %_171.i.i.i225.i272, !dbg !39700
  %_0.i2790 = fadd float %_0.i2794, %_0.i3226, !dbg !39702
  %_0.i3225 = fmul float %history.i44.i18.sroa.35.08443, %_174.i.i.i228.i275, !dbg !39704
  %_0.i2789 = fadd float %_0.i2793, %_0.i3225, !dbg !39706
  %618 = tail call noundef float @llvm.fabs.f32(float %_0.i2792), !dbg !39708
  %_3.i.i4226.inv = fcmp ogt float %617, %618, !dbg !39710
  %_4.i.i4233.v = select i1 %_3.i.i4226.inv, float %617, float %618, !dbg !39710
  %619 = tail call noundef float @llvm.fabs.f32(float %_0.i2791), !dbg !39708
  %_3.i.i4226.inv.1 = fcmp ogt float %_4.i.i4233.v, %619, !dbg !39710
  %_4.i.i4233.v.1 = select i1 %_3.i.i4226.inv.1, float %_4.i.i4233.v, float %619, !dbg !39710
  %620 = tail call noundef float @llvm.fabs.f32(float %_0.i2790), !dbg !39708
  %_3.i.i4226.inv.2 = fcmp ogt float %_4.i.i4233.v.1, %620, !dbg !39710
  %_4.i.i4233.v.2 = select i1 %_3.i.i4226.inv.2, float %_4.i.i4233.v.1, float %620, !dbg !39710
  %621 = tail call noundef float @llvm.fabs.f32(float %_0.i2789), !dbg !39708
  %_3.i.i4226.inv.3 = fcmp ogt float %_4.i.i4233.v.2, %621, !dbg !39710
  %_4.i.i4233.v.3 = select i1 %_3.i.i4226.inv.3, float %_4.i.i4233.v.2, float %621, !dbg !39710
  %622 = add nuw nsw i64 %iter.i40.i14.sroa.16.08444, 1, !dbg !39713
  %data.i4.i4860 = getelementptr inbounds nuw float, ptr %peaks_left.i54, i64 %iter.i40.i14.sroa.16.08444, !dbg !39714
  store float %_4.i.i4233.v.3, ptr %data.i4.i4860, align 4, !dbg !39717, !alias.scope !39719, !noalias !39511
  %exitcond11788.not = icmp eq i64 %622, %umax11790, !dbg !39490
  br i1 %exitcond11788.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit239.i286, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585, !dbg !39490

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit239.i286: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4846
  %history.i44.i18.sroa.0.0.lcssa = phi float [ %history.i44.i18.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4846 ], [ %_0.i3583, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585 ], !dbg !39722
  %history.i44.i18.sroa.7.0.lcssa = phi float [ %history.i44.i18.sroa.7.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4846 ], [ %history.i44.i18.sroa.0.08433, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585 ], !dbg !39722
  %history.i44.i18.sroa.10.0.lcssa = phi float [ %history.i44.i18.sroa.10.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4846 ], [ %history.i44.i18.sroa.7.08434, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585 ], !dbg !39722
  %history.i44.i18.sroa.13.0.lcssa = phi float [ %history.i44.i18.sroa.13.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4846 ], [ %history.i44.i18.sroa.10.08435, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585 ], !dbg !39722
  %history.i44.i18.sroa.16.0.lcssa = phi float [ %history.i44.i18.sroa.16.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4846 ], [ %history.i44.i18.sroa.13.08436, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585 ], !dbg !39722
  %history.i44.i18.sroa.19.0.lcssa = phi float [ %history.i44.i18.sroa.19.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4846 ], [ %history.i44.i18.sroa.16.08437, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585 ], !dbg !39722
  %history.i44.i18.sroa.22.0.lcssa = phi float [ %history.i44.i18.sroa.22.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4846 ], [ %history.i44.i18.sroa.19.08438, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585 ], !dbg !39722
  %history.i44.i18.sroa.26.0.lcssa = phi float [ %history.i44.i18.sroa.26.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4846 ], [ %history.i44.i18.sroa.22.08439, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585 ], !dbg !39722
  %history.i44.i18.sroa.29.0.lcssa = phi float [ %history.i44.i18.sroa.29.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4846 ], [ %history.i44.i18.sroa.26.08440, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585 ], !dbg !39722
  %history.i44.i18.sroa.32.0.lcssa = phi float [ %history.i44.i18.sroa.32.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4846 ], [ %history.i44.i18.sroa.29.08441, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585 ], !dbg !39722
  %history.i44.i18.sroa.35.0.lcssa = phi float [ %history.i44.i18.sroa.35.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4846 ], [ %history.i44.i18.sroa.32.08442, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585 ], !dbg !39722
  %history.i44.i18.sroa.38.0.lcssa = phi float [ %history.i44.i18.sroa.38.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4846 ], [ %history.i44.i18.sroa.35.08443, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3585 ], !dbg !39722
  store float %history.i44.i18.sroa.0.0.lcssa, ptr %hot_left.i56, align 4, !dbg !39723, !noalias !39487
  store float %history.i44.i18.sroa.7.0.lcssa, ptr %history.i44.i18.sroa.7.0.hot_left.i56.sroa_idx, align 4, !dbg !39723, !noalias !39487
  store float %history.i44.i18.sroa.10.0.lcssa, ptr %history.i44.i18.sroa.10.0.hot_left.i56.sroa_idx, align 4, !dbg !39723, !noalias !39487
  store float %history.i44.i18.sroa.13.0.lcssa, ptr %history.i44.i18.sroa.13.0.hot_left.i56.sroa_idx, align 4, !dbg !39723, !noalias !39487
  store float %history.i44.i18.sroa.16.0.lcssa, ptr %history.i44.i18.sroa.16.0.hot_left.i56.sroa_idx, align 4, !dbg !39723, !noalias !39487
  store float %history.i44.i18.sroa.19.0.lcssa, ptr %history.i44.i18.sroa.19.0.hot_left.i56.sroa_idx, align 4, !dbg !39723, !noalias !39487
  store float %history.i44.i18.sroa.22.0.lcssa, ptr %history.i44.i18.sroa.22.0.hot_left.i56.sroa_idx, align 4, !dbg !39723, !noalias !39487
  store float %history.i44.i18.sroa.26.0.lcssa, ptr %history.i44.i18.sroa.26.0.hot_left.i56.sroa_idx, align 4, !dbg !39723, !noalias !39487
  store float %history.i44.i18.sroa.29.0.lcssa, ptr %history.i44.i18.sroa.29.0.hot_left.i56.sroa_idx, align 4, !dbg !39723, !noalias !39487
  store float %history.i44.i18.sroa.32.0.lcssa, ptr %history.i44.i18.sroa.32.0.hot_left.i56.sroa_idx, align 4, !dbg !39723, !noalias !39487
  store float %history.i44.i18.sroa.35.0.lcssa, ptr %history.i44.i18.sroa.35.0.hot_left.i56.sroa_idx, align 4, !dbg !39723, !noalias !39487
  store float %history.i44.i18.sroa.38.0.lcssa, ptr %history.i44.i18.sroa.38.0.hot_left.i56.sroa_idx, align 4, !dbg !39723, !noalias !39487
  %_190.not.i287 = icmp ugt i64 %_52.i83, %right_io.1, !dbg !39724
  br i1 %_190.not.i287, label %bb60.i699, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4902, !dbg !39724, !prof !1406

bb60.i699:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit239.i286
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %iter1.sroa.0.0.i769048, i64 noundef %_52.i83, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_480b8302a9d65cc7541e43747df38ed7) #30, !dbg !39728, !noalias !39358
  unreachable, !dbg !39728

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4902: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit239.i286
  %_197.i289 = getelementptr inbounds nuw float, ptr %right_io.0, i64 %iter1.sroa.0.0.i769048, !dbg !39729
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39733), !dbg !39736
  %history.i.i32.sroa.0.0.copyload = load float, ptr %hot_right.i55, align 4, !dbg !39737, !noalias !39739
  %history.i.i32.sroa.7.0.copyload = load float, ptr %history.i.i32.sroa.7.0.hot_right.i55.sroa_idx, align 4, !dbg !39737, !noalias !39739
  %history.i.i32.sroa.10.0.copyload = load float, ptr %history.i.i32.sroa.10.0.hot_right.i55.sroa_idx, align 4, !dbg !39737, !noalias !39739
  %history.i.i32.sroa.13.0.copyload = load float, ptr %history.i.i32.sroa.13.0.hot_right.i55.sroa_idx, align 4, !dbg !39737, !noalias !39739
  %history.i.i32.sroa.16.0.copyload = load float, ptr %history.i.i32.sroa.16.0.hot_right.i55.sroa_idx, align 4, !dbg !39737, !noalias !39739
  %history.i.i32.sroa.19.0.copyload = load float, ptr %history.i.i32.sroa.19.0.hot_right.i55.sroa_idx, align 4, !dbg !39737, !noalias !39739
  %history.i.i32.sroa.22.0.copyload = load float, ptr %history.i.i32.sroa.22.0.hot_right.i55.sroa_idx, align 4, !dbg !39737, !noalias !39739
  %history.i.i32.sroa.26.0.copyload = load float, ptr %history.i.i32.sroa.26.0.hot_right.i55.sroa_idx, align 4, !dbg !39737, !noalias !39739
  %history.i.i32.sroa.29.0.copyload = load float, ptr %history.i.i32.sroa.29.0.hot_right.i55.sroa_idx, align 4, !dbg !39737, !noalias !39739
  %history.i.i32.sroa.32.0.copyload = load float, ptr %history.i.i32.sroa.32.0.hot_right.i55.sroa_idx, align 4, !dbg !39737, !noalias !39739
  %history.i.i32.sroa.35.0.copyload = load float, ptr %history.i.i32.sroa.35.0.hot_right.i55.sroa_idx, align 4, !dbg !39737, !noalias !39739
  %history.i.i32.sroa.38.0.copyload = load float, ptr %history.i.i32.sroa.38.0.hot_right.i55.sroa_idx, align 4, !dbg !39737, !noalias !39739
  br i1 %_2.i48498432.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i484, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580.lr.ph, !dbg !39742

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580.lr.ph: ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4902
  %_11.i.i.i.i310 = load float, ptr %_31, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_14.i.i.i.i313 = load float, ptr %549, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_17.i.i.i.i316 = load float, ptr %550, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_20.i.i.i.i319 = load float, ptr %551, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_25.i.i.i.i324 = load float, ptr %row1.i.i.i77.i124, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_28.i.i.i.i327 = load float, ptr %552, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_31.i.i.i.i330 = load float, ptr %553, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_34.i.i.i.i333 = load float, ptr %554, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_39.i.i.i.i338 = load float, ptr %row3.i.i.i91.i138, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_42.i.i.i.i341 = load float, ptr %555, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_45.i.i.i.i344 = load float, ptr %556, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_48.i.i.i.i347 = load float, ptr %557, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_53.i.i.i.i352 = load float, ptr %row5.i.i.i105.i152, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_56.i.i.i.i355 = load float, ptr %558, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_59.i.i.i.i358 = load float, ptr %559, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_62.i.i.i.i361 = load float, ptr %560, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_67.i.i.i.i366 = load float, ptr %row7.i.i.i119.i166, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_70.i.i.i.i369 = load float, ptr %561, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_73.i.i.i.i372 = load float, ptr %562, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_76.i.i.i.i375 = load float, ptr %563, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_81.i.i.i.i380 = load float, ptr %row9.i.i.i133.i180, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_84.i.i.i.i383 = load float, ptr %564, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_87.i.i.i.i386 = load float, ptr %565, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_90.i.i.i.i389 = load float, ptr %566, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_95.i.i.i.i394 = load float, ptr %row11.i.i.i147.i194, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_98.i.i.i.i397 = load float, ptr %567, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_101.i.i.i.i400 = load float, ptr %568, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_104.i.i.i.i403 = load float, ptr %569, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_109.i.i.i.i408 = load float, ptr %row13.i.i.i161.i208, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_112.i.i.i.i411 = load float, ptr %570, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_115.i.i.i.i414 = load float, ptr %571, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_118.i.i.i.i417 = load float, ptr %572, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_123.i.i.i.i422 = load float, ptr %row15.i.i.i175.i222, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_126.i.i.i.i425 = load float, ptr %573, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_129.i.i.i.i428 = load float, ptr %574, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_132.i.i.i.i431 = load float, ptr %575, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_137.i.i.i.i436 = load float, ptr %row17.i.i.i189.i236, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_140.i.i.i.i439 = load float, ptr %576, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_143.i.i.i.i442 = load float, ptr %577, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_146.i.i.i.i445 = load float, ptr %578, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_151.i.i.i.i450 = load float, ptr %row19.i.i.i203.i250, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_154.i.i.i.i453 = load float, ptr %579, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_157.i.i.i.i456 = load float, ptr %580, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_160.i.i.i.i459 = load float, ptr %581, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_165.i.i.i.i464 = load float, ptr %row21.i.i.i217.i264, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_168.i.i.i.i467 = load float, ptr %582, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_171.i.i.i.i470 = load float, ptr %583, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  %_174.i.i.i.i473 = load float, ptr %584, align 4, !alias.scope !39745, !noalias !39750, !noundef !12
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580, !dbg !39742

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580
  %iter.i.i28.sroa.16.08471 = phi i64 [ 0, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580.lr.ph ], [ %628, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580 ]
  %history.i.i32.sroa.35.08470 = phi float [ %history.i.i32.sroa.35.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580.lr.ph ], [ %history.i.i32.sroa.32.08469, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580 ]
  %history.i.i32.sroa.32.08469 = phi float [ %history.i.i32.sroa.32.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580.lr.ph ], [ %history.i.i32.sroa.29.08468, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580 ]
  %history.i.i32.sroa.29.08468 = phi float [ %history.i.i32.sroa.29.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580.lr.ph ], [ %history.i.i32.sroa.26.08467, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580 ]
  %history.i.i32.sroa.26.08467 = phi float [ %history.i.i32.sroa.26.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580.lr.ph ], [ %history.i.i32.sroa.22.08466, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580 ]
  %history.i.i32.sroa.22.08466 = phi float [ %history.i.i32.sroa.22.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580.lr.ph ], [ %history.i.i32.sroa.19.08465, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580 ]
  %history.i.i32.sroa.19.08465 = phi float [ %history.i.i32.sroa.19.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580.lr.ph ], [ %history.i.i32.sroa.16.08464, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580 ]
  %history.i.i32.sroa.16.08464 = phi float [ %history.i.i32.sroa.16.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580.lr.ph ], [ %history.i.i32.sroa.13.08463, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580 ]
  %history.i.i32.sroa.13.08463 = phi float [ %history.i.i32.sroa.13.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580.lr.ph ], [ %history.i.i32.sroa.10.08462, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580 ]
  %history.i.i32.sroa.10.08462 = phi float [ %history.i.i32.sroa.10.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580.lr.ph ], [ %history.i.i32.sroa.7.08461, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580 ]
  %history.i.i32.sroa.7.08461 = phi float [ %history.i.i32.sroa.7.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580.lr.ph ], [ %history.i.i32.sroa.0.08460, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580 ]
  %history.i.i32.sroa.0.08460 = phi float [ %history.i.i32.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580.lr.ph ], [ %_0.i3578, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580 ]
  %data.i.i4912 = getelementptr inbounds nuw float, ptr %_197.i289, i64 %iter.i.i28.sroa.16.08471, !dbg !39755
  %_0.i3578 = load float, ptr %data.i.i4912, align 4, !dbg !39758, !alias.scope !39760, !noalias !39763, !noundef !12
  %623 = tail call noundef float @llvm.fabs.f32(float %history.i.i32.sroa.19.08465), !dbg !39764
  %_0.i3224 = fmul float %_0.i3578, %_11.i.i.i.i310, !dbg !39767
  %_0.i2788 = fadd float %_0.i3224, 0.000000e+00, !dbg !39770
  %_0.i3223 = fmul float %_0.i3578, %_14.i.i.i.i313, !dbg !39772
  %_0.i2787 = fadd float %_0.i3223, 0.000000e+00, !dbg !39774
  %_0.i3222 = fmul float %_0.i3578, %_17.i.i.i.i316, !dbg !39776
  %_0.i2786 = fadd float %_0.i3222, 0.000000e+00, !dbg !39778
  %_0.i3221 = fmul float %_0.i3578, %_20.i.i.i.i319, !dbg !39780
  %_0.i2785 = fadd float %_0.i3221, 0.000000e+00, !dbg !39782
  %_0.i3220 = fmul float %history.i.i32.sroa.0.08460, %_25.i.i.i.i324, !dbg !39784
  %_0.i2784 = fadd float %_0.i2788, %_0.i3220, !dbg !39786
  %_0.i3219 = fmul float %history.i.i32.sroa.0.08460, %_28.i.i.i.i327, !dbg !39788
  %_0.i2783 = fadd float %_0.i2787, %_0.i3219, !dbg !39790
  %_0.i3218 = fmul float %history.i.i32.sroa.0.08460, %_31.i.i.i.i330, !dbg !39792
  %_0.i2782 = fadd float %_0.i2786, %_0.i3218, !dbg !39794
  %_0.i3217 = fmul float %history.i.i32.sroa.0.08460, %_34.i.i.i.i333, !dbg !39796
  %_0.i2781 = fadd float %_0.i2785, %_0.i3217, !dbg !39798
  %_0.i3216 = fmul float %history.i.i32.sroa.7.08461, %_39.i.i.i.i338, !dbg !39800
  %_0.i2780 = fadd float %_0.i2784, %_0.i3216, !dbg !39802
  %_0.i3215 = fmul float %history.i.i32.sroa.7.08461, %_42.i.i.i.i341, !dbg !39804
  %_0.i2779 = fadd float %_0.i2783, %_0.i3215, !dbg !39806
  %_0.i3214 = fmul float %history.i.i32.sroa.7.08461, %_45.i.i.i.i344, !dbg !39808
  %_0.i2778 = fadd float %_0.i2782, %_0.i3214, !dbg !39810
  %_0.i3213 = fmul float %history.i.i32.sroa.7.08461, %_48.i.i.i.i347, !dbg !39812
  %_0.i2777 = fadd float %_0.i2781, %_0.i3213, !dbg !39814
  %_0.i3212 = fmul float %history.i.i32.sroa.10.08462, %_53.i.i.i.i352, !dbg !39816
  %_0.i2776 = fadd float %_0.i2780, %_0.i3212, !dbg !39818
  %_0.i3211 = fmul float %history.i.i32.sroa.10.08462, %_56.i.i.i.i355, !dbg !39820
  %_0.i2775 = fadd float %_0.i2779, %_0.i3211, !dbg !39822
  %_0.i3210 = fmul float %history.i.i32.sroa.10.08462, %_59.i.i.i.i358, !dbg !39824
  %_0.i2774 = fadd float %_0.i2778, %_0.i3210, !dbg !39826
  %_0.i3209 = fmul float %history.i.i32.sroa.10.08462, %_62.i.i.i.i361, !dbg !39828
  %_0.i2773 = fadd float %_0.i2777, %_0.i3209, !dbg !39830
  %_0.i3208 = fmul float %history.i.i32.sroa.13.08463, %_67.i.i.i.i366, !dbg !39832
  %_0.i2772 = fadd float %_0.i2776, %_0.i3208, !dbg !39834
  %_0.i3207 = fmul float %history.i.i32.sroa.13.08463, %_70.i.i.i.i369, !dbg !39836
  %_0.i2771 = fadd float %_0.i2775, %_0.i3207, !dbg !39838
  %_0.i3206 = fmul float %history.i.i32.sroa.13.08463, %_73.i.i.i.i372, !dbg !39840
  %_0.i2770 = fadd float %_0.i2774, %_0.i3206, !dbg !39842
  %_0.i3205 = fmul float %history.i.i32.sroa.13.08463, %_76.i.i.i.i375, !dbg !39844
  %_0.i2769 = fadd float %_0.i2773, %_0.i3205, !dbg !39846
  %_0.i3204 = fmul float %history.i.i32.sroa.16.08464, %_81.i.i.i.i380, !dbg !39848
  %_0.i2768 = fadd float %_0.i2772, %_0.i3204, !dbg !39850
  %_0.i3203 = fmul float %history.i.i32.sroa.16.08464, %_84.i.i.i.i383, !dbg !39852
  %_0.i2767 = fadd float %_0.i2771, %_0.i3203, !dbg !39854
  %_0.i3202 = fmul float %history.i.i32.sroa.16.08464, %_87.i.i.i.i386, !dbg !39856
  %_0.i2766 = fadd float %_0.i2770, %_0.i3202, !dbg !39858
  %_0.i3201 = fmul float %history.i.i32.sroa.16.08464, %_90.i.i.i.i389, !dbg !39860
  %_0.i2765 = fadd float %_0.i2769, %_0.i3201, !dbg !39862
  %_0.i3200 = fmul float %history.i.i32.sroa.19.08465, %_95.i.i.i.i394, !dbg !39864
  %_0.i2764 = fadd float %_0.i2768, %_0.i3200, !dbg !39866
  %_0.i3199 = fmul float %history.i.i32.sroa.19.08465, %_98.i.i.i.i397, !dbg !39868
  %_0.i2763 = fadd float %_0.i2767, %_0.i3199, !dbg !39870
  %_0.i3198 = fmul float %history.i.i32.sroa.19.08465, %_101.i.i.i.i400, !dbg !39872
  %_0.i2762 = fadd float %_0.i2766, %_0.i3198, !dbg !39874
  %_0.i3197 = fmul float %history.i.i32.sroa.19.08465, %_104.i.i.i.i403, !dbg !39876
  %_0.i2761 = fadd float %_0.i2765, %_0.i3197, !dbg !39878
  %_0.i3196 = fmul float %history.i.i32.sroa.22.08466, %_109.i.i.i.i408, !dbg !39880
  %_0.i2760 = fadd float %_0.i2764, %_0.i3196, !dbg !39882
  %_0.i3195 = fmul float %history.i.i32.sroa.22.08466, %_112.i.i.i.i411, !dbg !39884
  %_0.i2759 = fadd float %_0.i2763, %_0.i3195, !dbg !39886
  %_0.i3194 = fmul float %history.i.i32.sroa.22.08466, %_115.i.i.i.i414, !dbg !39888
  %_0.i2758 = fadd float %_0.i2762, %_0.i3194, !dbg !39890
  %_0.i3193 = fmul float %history.i.i32.sroa.22.08466, %_118.i.i.i.i417, !dbg !39892
  %_0.i2757 = fadd float %_0.i2761, %_0.i3193, !dbg !39894
  %_0.i3192 = fmul float %history.i.i32.sroa.26.08467, %_123.i.i.i.i422, !dbg !39896
  %_0.i2756 = fadd float %_0.i2760, %_0.i3192, !dbg !39898
  %_0.i3191 = fmul float %history.i.i32.sroa.26.08467, %_126.i.i.i.i425, !dbg !39900
  %_0.i2755 = fadd float %_0.i2759, %_0.i3191, !dbg !39902
  %_0.i3190 = fmul float %history.i.i32.sroa.26.08467, %_129.i.i.i.i428, !dbg !39904
  %_0.i2754 = fadd float %_0.i2758, %_0.i3190, !dbg !39906
  %_0.i3189 = fmul float %history.i.i32.sroa.26.08467, %_132.i.i.i.i431, !dbg !39908
  %_0.i2753 = fadd float %_0.i2757, %_0.i3189, !dbg !39910
  %_0.i3188 = fmul float %history.i.i32.sroa.29.08468, %_137.i.i.i.i436, !dbg !39912
  %_0.i2752 = fadd float %_0.i2756, %_0.i3188, !dbg !39914
  %_0.i3187 = fmul float %history.i.i32.sroa.29.08468, %_140.i.i.i.i439, !dbg !39916
  %_0.i2751 = fadd float %_0.i2755, %_0.i3187, !dbg !39918
  %_0.i3186 = fmul float %history.i.i32.sroa.29.08468, %_143.i.i.i.i442, !dbg !39920
  %_0.i2750 = fadd float %_0.i2754, %_0.i3186, !dbg !39922
  %_0.i3185 = fmul float %history.i.i32.sroa.29.08468, %_146.i.i.i.i445, !dbg !39924
  %_0.i2749 = fadd float %_0.i2753, %_0.i3185, !dbg !39926
  %_0.i3184 = fmul float %history.i.i32.sroa.32.08469, %_151.i.i.i.i450, !dbg !39928
  %_0.i2748 = fadd float %_0.i2752, %_0.i3184, !dbg !39930
  %_0.i3183 = fmul float %history.i.i32.sroa.32.08469, %_154.i.i.i.i453, !dbg !39932
  %_0.i2747 = fadd float %_0.i2751, %_0.i3183, !dbg !39934
  %_0.i3182 = fmul float %history.i.i32.sroa.32.08469, %_157.i.i.i.i456, !dbg !39936
  %_0.i2746 = fadd float %_0.i2750, %_0.i3182, !dbg !39938
  %_0.i3181 = fmul float %history.i.i32.sroa.32.08469, %_160.i.i.i.i459, !dbg !39940
  %_0.i2745 = fadd float %_0.i2749, %_0.i3181, !dbg !39942
  %_0.i3180 = fmul float %history.i.i32.sroa.35.08470, %_165.i.i.i.i464, !dbg !39944
  %_0.i2744 = fadd float %_0.i2748, %_0.i3180, !dbg !39946
  %_0.i3179 = fmul float %history.i.i32.sroa.35.08470, %_168.i.i.i.i467, !dbg !39948
  %_0.i2743 = fadd float %_0.i2747, %_0.i3179, !dbg !39950
  %_0.i3178 = fmul float %history.i.i32.sroa.35.08470, %_171.i.i.i.i470, !dbg !39952
  %_0.i2742 = fadd float %_0.i2746, %_0.i3178, !dbg !39954
  %_0.i3177 = fmul float %history.i.i32.sroa.35.08470, %_174.i.i.i.i473, !dbg !39956
  %_0.i2741 = fadd float %_0.i2745, %_0.i3177, !dbg !39958
  %624 = tail call noundef float @llvm.fabs.f32(float %_0.i2744), !dbg !39960
  %_3.i.i4217.inv = fcmp ogt float %623, %624, !dbg !39962
  %_4.i.i4224.v = select i1 %_3.i.i4217.inv, float %623, float %624, !dbg !39962
  %625 = tail call noundef float @llvm.fabs.f32(float %_0.i2743), !dbg !39960
  %_3.i.i4217.inv.1 = fcmp ogt float %_4.i.i4224.v, %625, !dbg !39962
  %_4.i.i4224.v.1 = select i1 %_3.i.i4217.inv.1, float %_4.i.i4224.v, float %625, !dbg !39962
  %626 = tail call noundef float @llvm.fabs.f32(float %_0.i2742), !dbg !39960
  %_3.i.i4217.inv.2 = fcmp ogt float %_4.i.i4224.v.1, %626, !dbg !39962
  %_4.i.i4224.v.2 = select i1 %_3.i.i4217.inv.2, float %_4.i.i4224.v.1, float %626, !dbg !39962
  %627 = tail call noundef float @llvm.fabs.f32(float %_0.i2741), !dbg !39960
  %_3.i.i4217.inv.3 = fcmp ogt float %_4.i.i4224.v.2, %627, !dbg !39962
  %_4.i.i4224.v.3 = select i1 %_3.i.i4217.inv.3, float %_4.i.i4224.v.2, float %627, !dbg !39962
  %628 = add nuw nsw i64 %iter.i.i28.sroa.16.08471, 1, !dbg !39965
  %data.i4.i4916 = getelementptr inbounds nuw float, ptr %peaks_right.i53, i64 %iter.i.i28.sroa.16.08471, !dbg !39966
  store float %_4.i.i4224.v.3, ptr %data.i4.i4916, align 4, !dbg !39969, !alias.scope !39971, !noalias !39763
  %exitcond11791.not = icmp eq i64 %628, %umax11790, !dbg !39742
  br i1 %exitcond11791.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i484, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580, !dbg !39742

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i484: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4902
  %history.i.i32.sroa.0.0.lcssa = phi float [ %history.i.i32.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4902 ], [ %_0.i3578, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580 ], !dbg !39974
  %history.i.i32.sroa.7.0.lcssa = phi float [ %history.i.i32.sroa.7.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4902 ], [ %history.i.i32.sroa.0.08460, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580 ], !dbg !39974
  %history.i.i32.sroa.10.0.lcssa = phi float [ %history.i.i32.sroa.10.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4902 ], [ %history.i.i32.sroa.7.08461, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580 ], !dbg !39974
  %history.i.i32.sroa.13.0.lcssa = phi float [ %history.i.i32.sroa.13.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4902 ], [ %history.i.i32.sroa.10.08462, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580 ], !dbg !39974
  %history.i.i32.sroa.16.0.lcssa = phi float [ %history.i.i32.sroa.16.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4902 ], [ %history.i.i32.sroa.13.08463, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580 ], !dbg !39974
  %history.i.i32.sroa.19.0.lcssa = phi float [ %history.i.i32.sroa.19.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4902 ], [ %history.i.i32.sroa.16.08464, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580 ], !dbg !39974
  %history.i.i32.sroa.22.0.lcssa = phi float [ %history.i.i32.sroa.22.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4902 ], [ %history.i.i32.sroa.19.08465, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580 ], !dbg !39974
  %history.i.i32.sroa.26.0.lcssa = phi float [ %history.i.i32.sroa.26.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4902 ], [ %history.i.i32.sroa.22.08466, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580 ], !dbg !39974
  %history.i.i32.sroa.29.0.lcssa = phi float [ %history.i.i32.sroa.29.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4902 ], [ %history.i.i32.sroa.26.08467, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580 ], !dbg !39974
  %history.i.i32.sroa.32.0.lcssa = phi float [ %history.i.i32.sroa.32.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4902 ], [ %history.i.i32.sroa.29.08468, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580 ], !dbg !39974
  %history.i.i32.sroa.35.0.lcssa = phi float [ %history.i.i32.sroa.35.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4902 ], [ %history.i.i32.sroa.32.08469, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580 ], !dbg !39974
  %history.i.i32.sroa.38.0.lcssa = phi float [ %history.i.i32.sroa.38.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit4902 ], [ %history.i.i32.sroa.35.08470, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3580 ], !dbg !39974
  store float %history.i.i32.sroa.0.0.lcssa, ptr %hot_right.i55, align 4, !dbg !39975, !noalias !39739
  store float %history.i.i32.sroa.7.0.lcssa, ptr %history.i.i32.sroa.7.0.hot_right.i55.sroa_idx, align 4, !dbg !39975, !noalias !39739
  store float %history.i.i32.sroa.10.0.lcssa, ptr %history.i.i32.sroa.10.0.hot_right.i55.sroa_idx, align 4, !dbg !39975, !noalias !39739
  store float %history.i.i32.sroa.13.0.lcssa, ptr %history.i.i32.sroa.13.0.hot_right.i55.sroa_idx, align 4, !dbg !39975, !noalias !39739
  store float %history.i.i32.sroa.16.0.lcssa, ptr %history.i.i32.sroa.16.0.hot_right.i55.sroa_idx, align 4, !dbg !39975, !noalias !39739
  store float %history.i.i32.sroa.19.0.lcssa, ptr %history.i.i32.sroa.19.0.hot_right.i55.sroa_idx, align 4, !dbg !39975, !noalias !39739
  store float %history.i.i32.sroa.22.0.lcssa, ptr %history.i.i32.sroa.22.0.hot_right.i55.sroa_idx, align 4, !dbg !39975, !noalias !39739
  store float %history.i.i32.sroa.26.0.lcssa, ptr %history.i.i32.sroa.26.0.hot_right.i55.sroa_idx, align 4, !dbg !39975, !noalias !39739
  store float %history.i.i32.sroa.29.0.lcssa, ptr %history.i.i32.sroa.29.0.hot_right.i55.sroa_idx, align 4, !dbg !39975, !noalias !39739
  store float %history.i.i32.sroa.32.0.lcssa, ptr %history.i.i32.sroa.32.0.hot_right.i55.sroa_idx, align 4, !dbg !39975, !noalias !39739
  store float %history.i.i32.sroa.35.0.lcssa, ptr %history.i.i32.sroa.35.0.hot_right.i55.sroa_idx, align 4, !dbg !39975, !noalias !39739
  store float %history.i.i32.sroa.38.0.lcssa, ptr %history.i.i32.sroa.38.0.hot_right.i55.sroa_idx, align 4, !dbg !39975, !noalias !39739
  br i1 %_2.i48498432.not, label %bb15.i73.loopexit, label %bb20.i490.lr.ph, !dbg !39455

bb20.i490.lr.ph:                                  ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i484
  %_70.i48.sroa.3.0.copyload = load i64, ptr %_70.i48.sroa.3.0..sroa_idx, align 8, !noalias !39364
  %_70.i48.sroa.4.0.copyload = load i64, ptr %_70.i48.sroa.4.0..sroa_idx, align 8, !noalias !39364
  %_71.i47.sroa.3.0.copyload = load i64, ptr %_71.i47.sroa.3.0..sroa_idx, align 8, !noalias !39364
  %_71.i47.sroa.4.0.copyload = load i64, ptr %_71.i47.sroa.4.0..sroa_idx, align 8, !noalias !39364
  %_54.0.i250.i548 = load ptr, ptr %uniform_left.i52, align 8, !nonnull !12, !align !24
  %_54.1.i251.i549 = load i64, ptr %599, align 8
  %_18.i261.i559 = load i64, ptr %585, align 8
  %_56.0.i272.i570 = load ptr, ptr %600, align 8, !nonnull !12, !align !24
  %_56.1.i273.i571 = load i64, ptr %601, align 8
  %_58.1.i298.i596 = load i64, ptr %605, align 8
  %_58.0.i297.i595 = load ptr, ptr %606, align 8, !nonnull !12, !align !24
  %_54.0.i.i623 = load ptr, ptr %uniform_right.i51, align 8, !nonnull !12, !align !24
  %_54.1.i.i624 = load i64, ptr %607, align 8
  %_18.i.i634 = load i64, ptr %586, align 8
  %_56.0.i.i645 = load ptr, ptr %608, align 8, !nonnull !12, !align !24
  %_56.1.i.i646 = load i64, ptr %609, align 8
  %_58.1.i.i671 = load i64, ptr %613, align 8
  %_58.0.i.i670 = load ptr, ptr %614, align 8, !nonnull !12, !align !24
  %_22.i265.i563.promoted8967 = load i32, ptr %_22.i265.i563, align 4
  %_21.i264.i562.promoted9003 = load float, ptr %_21.i264.i562, align 4
  %_22.i.i638.promoted9006 = load i32, ptr %_22.i.i638, align 4
  %_21.i.i637.promoted9042 = load float, ptr %_21.i.i637, align 4
  %umax11792 = call i64 @llvm.umax.i64(i64 %_18.i261.i559, i64 1), !dbg !39455
  %umax11794 = call i64 @llvm.umax.i64(i64 %_18.i.i634, i64 1), !dbg !39455
  %_13.i248556865688 = load float, ptr %589, align 4
  %_13.i247356895691 = load float, ptr %592, align 4
  %_13.i246156925694 = load float, ptr %595, align 4
  %_13.i244956955697 = load float, ptr %598, align 4
  %_37.i285.i583 = load float, ptr %603, align 4
  %_37.i.i658 = load float, ptr %611, align 4
  %.promoted14042 = load float, ptr %587, align 4
  %_114.i523.promoted14060 = load float, ptr %_114.i523, align 4
  %.promoted14063 = load float, ptr %588, align 4
  %.promoted14066 = load float, ptr %590, align 4
  %_115.i524.promoted14069 = load float, ptr %_115.i524, align 4
  %.promoted14072 = load float, ptr %591, align 4
  %.promoted14075 = load float, ptr %593, align 4
  %_119.i525.promoted14093 = load float, ptr %_119.i525, align 4
  %.promoted14096 = load float, ptr %594, align 4
  %.promoted14099 = load float, ptr %596, align 4
  %_120.i526.promoted14102 = load float, ptr %_120.i526, align 4
  %.promoted14105 = load float, ptr %597, align 4
  %.promoted14108 = load float, ptr %602, align 4
  %.promoted14111 = load float, ptr %604, align 4
  %.promoted14114 = load float, ptr %610, align 4
  %.promoted14117 = load float, ptr %612, align 4
  %_21.i264.i562.promoted = load float, ptr %_21.i264.i562, align 1
  %_21.i.i637.promoted = load float, ptr %_21.i.i637, align 1
  br label %bb20.i490, !dbg !39455

bb20.i490:                                        ; preds = %bb20.i490.lr.ph, %bb32.i690
  %running.sroa.0.0.i.lcssa1252314138 = phi float [ %_21.i.i637.promoted, %bb20.i490.lr.ph ], [ %running.sroa.0.0.i.lcssa1252314137, %bb32.i690 ]
  %running.sroa.0.0.i1694.lcssa1248614121 = phi float [ %_21.i264.i562.promoted, %bb20.i490.lr.ph ], [ %running.sroa.0.0.i1694.lcssa1248614120, %bb32.i690 ]
  %_0.i3776.lcssa1404114119 = phi float [ %.promoted14117, %bb20.i490.lr.ph ], [ %_0.i3776.lcssa1404114118, %bb32.i690 ]
  %_0.i3402.lcssa1402614116 = phi float [ %.promoted14114, %bb20.i490.lr.ph ], [ %_0.i3402.lcssa1402614115, %bb32.i690 ]
  %_0.i3780.lcssa1401114113 = phi float [ %.promoted14111, %bb20.i490.lr.ph ], [ %_0.i3780.lcssa1401114112, %bb32.i690 ]
  %_0.i3406.lcssa1399614110 = phi float [ %.promoted14108, %bb20.i490.lr.ph ], [ %_0.i3406.lcssa1399614109, %bb32.i690 ]
  %_0.i3831.lcssa1232014107 = phi float [ %.promoted14105, %bb20.i490.lr.ph ], [ %_0.i3831.lcssa1232014106, %bb32.i690 ]
  %_0.i3838.lcssa1233414104 = phi float [ %_120.i526.promoted14102, %bb20.i490.lr.ph ], [ %_0.i3838.lcssa1233414103, %bb32.i690 ]
  %_0.i.i4072.lcssa1234814101 = phi float [ %.promoted14099, %bb20.i490.lr.ph ], [ %_0.i.i4072.lcssa1234814100, %bb32.i690 ]
  %_0.i3818.lcssa1236214098 = phi float [ %.promoted14096, %bb20.i490.lr.ph ], [ %_0.i3818.lcssa1236214097, %bb32.i690 ]
  %_0.i3825.lcssa1237614095 = phi float [ %_119.i525.promoted14093, %bb20.i490.lr.ph ], [ %_0.i3825.lcssa1237614094, %bb32.i690 ]
  %_0.i.i4065.lcssa1239014077 = phi float [ %.promoted14075, %bb20.i490.lr.ph ], [ %_0.i.i4065.lcssa1239014076, %bb32.i690 ]
  %_0.i3805.lcssa1240414074 = phi float [ %.promoted14072, %bb20.i490.lr.ph ], [ %_0.i3805.lcssa1240414073, %bb32.i690 ]
  %_0.i3812.lcssa1241814071 = phi float [ %_115.i524.promoted14069, %bb20.i490.lr.ph ], [ %_0.i3812.lcssa1241814070, %bb32.i690 ]
  %_0.i.i4058.lcssa1243214068 = phi float [ %.promoted14066, %bb20.i490.lr.ph ], [ %_0.i.i4058.lcssa1243214067, %bb32.i690 ]
  %_0.i3793.lcssa1244614065 = phi float [ %.promoted14063, %bb20.i490.lr.ph ], [ %_0.i3793.lcssa1244614064, %bb32.i690 ]
  %_0.i3799.lcssa1246014062 = phi float [ %_114.i523.promoted14060, %bb20.i490.lr.ph ], [ %_0.i3799.lcssa1246014061, %bb32.i690 ]
  %_0.i.i.lcssa1247414044 = phi float [ %.promoted14042, %bb20.i490.lr.ph ], [ %_0.i.i.lcssa1247414043, %bb32.i690 ]
  %running.sroa.0.0.i.lcssa89529044 = phi float [ %_21.i.i637.promoted9042, %bb20.i490.lr.ph ], [ %running.sroa.0.0.i.lcssa89529043, %bb32.i690 ]
  %storemerge.i.lcssa89259008 = phi i32 [ %_22.i.i638.promoted9006, %bb20.i490.lr.ph ], [ %storemerge.i.lcssa89259007, %bb32.i690 ]
  %running.sroa.0.0.i1694.lcssa88949005 = phi float [ %_21.i264.i562.promoted9003, %bb20.i490.lr.ph ], [ %running.sroa.0.0.i1694.lcssa88949004, %bb32.i690 ]
  %storemerge.i1699.lcssa88678969 = phi i32 [ %_22.i265.i563.promoted8967, %bb20.i490.lr.ph ], [ %storemerge.i1699.lcssa88678968, %bb32.i690 ]
  %frame.sroa.0.0.i4888964 = phi i64 [ 0, %bb20.i490.lr.ph ], [ %_85.i503, %bb32.i690 ]
  %main_cursor.sroa.0.1.i4878963 = phi i64 [ %main_cursor.sroa.0.0.i759047, %bb20.i490.lr.ph ], [ %main_cursor.sroa.0.2.i696, %bb32.i690 ]
  %ring_cursor.sroa.0.1.i4868962 = phi i64 [ %ring_cursor.sroa.0.0.i749046, %bb20.i490.lr.ph ], [ %ring_cursor.sroa.0.2.i693, %bb32.i690 ]
  %_68.i491 = sub nuw nsw i64 %..i4820, %frame.sroa.0.0.i4888964, !dbg !39976
  %ring.i2336 = load i64, ptr %543, align 8, !dbg !39977, !alias.scope !39979, !noalias !39982, !noundef !12
  %main.i2337 = load i64, ptr %544, align 8, !dbg !39986, !alias.scope !39979, !noalias !39982, !noundef !12
  %_10.i = add i64 %ring_cursor.sroa.0.1.i4868962, 1, !dbg !39987
  %_45.not.i = icmp ult i64 %_10.i, %ring.i2336, !dbg !39988
  %629 = select i1 %_45.not.i, i64 0, i64 %ring.i2336, !dbg !39988
  %start1.sroa.0.0.i2338 = sub nuw i64 %_10.i, %629, !dbg !39988
  %_12.i2339 = add i64 %_70.i48.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i4868962, !dbg !39990
  %_46.not.i = icmp ult i64 %_12.i2339, %ring.i2336, !dbg !39991
  %630 = select i1 %_46.not.i, i64 0, i64 %ring.i2336, !dbg !39991
  %left_end.sroa.0.0.i = sub nuw i64 %_12.i2339, %630, !dbg !39991
  %_15.i2341 = add i64 %_71.i47.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i4868962, !dbg !39993
  %_47.not.i = icmp ult i64 %_15.i2341, %ring.i2336, !dbg !39994
  %631 = select i1 %_47.not.i, i64 0, i64 %ring.i2336, !dbg !39994
  %right_end.sroa.0.0.i = sub nuw i64 %_15.i2341, %631, !dbg !39994
  %_18.i = add i64 %_70.i48.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i4868962, !dbg !39996
  %_48.not.i = icmp ult i64 %_18.i, %ring.i2336, !dbg !39997
  %632 = select i1 %_48.not.i, i64 0, i64 %ring.i2336, !dbg !39997
  %left_expiring.sroa.0.0.i = sub nuw i64 %_18.i, %632, !dbg !39997
  %_21.i = add i64 %_71.i47.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i4868962, !dbg !39999
  %_49.not.i = icmp ult i64 %_21.i, %ring.i2336, !dbg !40000
  %633 = select i1 %_49.not.i, i64 0, i64 %ring.i2336, !dbg !40000
  %right_expiring.sroa.0.0.i = sub nuw i64 %_21.i, %633, !dbg !40000
  %_30.i2343 = sub i64 %ring.i2336, %ring_cursor.sroa.0.1.i4868962, !dbg !40002
  %..i4933 = tail call noundef i64 @llvm.umin.i64(i64 %_30.i2343, i64 %_68.i491), !dbg !40003
  %_31.i2345 = sub i64 %main.i2337, %main_cursor.sroa.0.1.i4878963, !dbg !40005
  %..i4934 = tail call noundef i64 @llvm.umin.i64(i64 %_31.i2345, i64 %..i4933), !dbg !40006
  %_32.i2347 = sub i64 %ring.i2336, %start1.sroa.0.0.i2338, !dbg !40008
  %..i4935 = tail call noundef i64 @llvm.umin.i64(i64 %_32.i2347, i64 %..i4934), !dbg !40009
  %_34.i2349 = sub i64 %ring.i2336, %left_end.sroa.0.0.i, !dbg !40011
  %..i4936 = tail call noundef i64 @llvm.umin.i64(i64 %_34.i2349, i64 %..i4935), !dbg !40012
  %_36.i2351 = sub i64 %ring.i2336, %right_end.sroa.0.0.i, !dbg !40014
  %..i4937 = tail call noundef i64 @llvm.umin.i64(i64 %_36.i2351, i64 %..i4936), !dbg !40015
  %_38.i = sub i64 %ring.i2336, %left_expiring.sroa.0.0.i, !dbg !40017
  %..i4938 = tail call noundef i64 @llvm.umin.i64(i64 %_38.i, i64 %..i4937), !dbg !40018
  %_40.i2352 = sub i64 %ring.i2336, %right_expiring.sroa.0.0.i, !dbg !40020
  %..i4939 = tail call noundef i64 @llvm.umin.i64(i64 %_40.i2352, i64 %..i4938), !dbg !40021
  %_74.i493 = add i64 %frame.sroa.0.0.i4888964, %iter1.sroa.0.0.i769048, !dbg !40023
  %_78.i494 = add i64 %..i4939, %_74.i493, !dbg !40024
  %_206.i495 = icmp ult i64 %_78.i494, %_74.i493, !dbg !40025
  %_202.not.i496 = icmp ugt i64 %_78.i494, %left_io.1
  %or.cond27.i497 = or i1 %_206.i495, %_202.not.i496, !dbg !40025
  br i1 %or.cond27.i497, label %bb62.i698, label %bb61.i498, !dbg !40025, !prof !165

bb62.i698:                                        ; preds = %bb20.i490
  store float %_0.i.i.lcssa1247414044, ptr %587, align 4
  store float %_0.i.i4065.lcssa1239014077, ptr %593, align 4
  store float %running.sroa.0.0.i1694.lcssa1248614121, ptr %_21.i264.i562, align 1, !dbg !39419
  store float %running.sroa.0.0.i.lcssa1252314138, ptr %_21.i.i637, align 1, !dbg !39452
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_74.i493, i64 noundef %_78.i494, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_debdb702bca99cd8ca93ec127a5c320a) #30, !dbg !40033, !noalias !39358
  unreachable, !dbg !40033

bb61.i498:                                        ; preds = %bb20.i490
  %_209.i499 = getelementptr inbounds nuw float, ptr %left_io.0, i64 %_74.i493, !dbg !40034
  %_210.not.i500 = icmp ugt i64 %_78.i494, %right_io.1, !dbg !40038
  br i1 %_210.not.i500, label %bb65.i697, label %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit, !dbg !40038, !prof !1406

bb65.i697:                                        ; preds = %bb61.i498
  store float %_0.i.i.lcssa1247414044, ptr %587, align 4
  store float %_0.i.i4065.lcssa1239014077, ptr %593, align 4
  store float %running.sroa.0.0.i1694.lcssa1248614121, ptr %_21.i264.i562, align 1, !dbg !39419
  store float %running.sroa.0.0.i.lcssa1252314138, ptr %_21.i.i637, align 1, !dbg !39452
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_74.i493, i64 noundef %_78.i494, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_01912ef845d3ed33a157086defa4d008) #30, !dbg !40042, !noalias !39358
  unreachable, !dbg !40042

_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit: ; preds = %bb61.i498
  %_215.i502 = getelementptr inbounds nuw float, ptr %right_io.0, i64 %_74.i493, !dbg !40043
  %_85.i503 = add nuw nsw i64 %..i4939, %frame.sroa.0.0.i4888964, !dbg !40047
  %_224.i509 = getelementptr inbounds nuw float, ptr %peaks_left.i54, i64 %frame.sroa.0.0.i4888964, !dbg !40048
  %_233.i510 = getelementptr inbounds nuw float, ptr %peaks_right.i53, i64 %frame.sroa.0.0.i4888964, !dbg !40057
  %_2.i.i.i8490.not = icmp eq i64 %..i4939, 0, !dbg !40066
  br i1 %_2.i.i.i8490.not, label %bb32.i690, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560.lr.ph, !dbg !40066

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560.lr.ph: ; preds = %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit
  %umin11798 = call i64 @llvm.umin.i64(i64 %_34.i2349, i64 %_36.i2351), !dbg !40066
  %umin11799 = call i64 @llvm.umin.i64(i64 %umin11798, i64 %_38.i), !dbg !40066
  %umin11800 = call i64 @llvm.umin.i64(i64 %umin11799, i64 %_40.i2352), !dbg !40066
  %umin11801 = call i64 @llvm.umin.i64(i64 %umin11800, i64 %_32.i2347), !dbg !40066
  %umin11802 = call i64 @llvm.umin.i64(i64 %umin11801, i64 %_30.i2343), !dbg !40066
  %umin11803 = call i64 @llvm.umin.i64(i64 %umin11802, i64 %_31.i2345), !dbg !40066
  %634 = sub nsw i64 %umin11804, %frame.sroa.0.0.i4888964, !dbg !40066
  %umin11805 = call i64 @llvm.umin.i64(i64 %umin11803, i64 %634), !dbg !40066
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560, !dbg !40066

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709
  %_0.i377614028 = phi float [ %_0.i3776.lcssa1404114119, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560.lr.ph ], [ %_0.i3776, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709 ]
  %_0.i340214013 = phi float [ %_0.i3402.lcssa1402614116, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560.lr.ph ], [ %_0.i3402, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709 ]
  %_0.i378013998 = phi float [ %_0.i3780.lcssa1401114113, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560.lr.ph ], [ %_0.i3780, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709 ]
  %_0.i340613983 = phi float [ %_0.i3406.lcssa1399614110, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560.lr.ph ], [ %_0.i3406, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709 ]
  %running.sroa.0.0.i8930 = phi float [ %running.sroa.0.0.i.lcssa89529044, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560.lr.ph ], [ %running.sroa.0.0.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709 ]
  %storemerge.i8903 = phi i32 [ %storemerge.i.lcssa89259008, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560.lr.ph ], [ %storemerge.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709 ]
  %running.sroa.0.0.i16948872 = phi float [ %running.sroa.0.0.i1694.lcssa88949005, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560.lr.ph ], [ %running.sroa.0.0.i1694, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709 ]
  %storemerge.i16998845 = phi i32 [ %storemerge.i1699.lcssa88678969, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560.lr.ph ], [ %storemerge.i1699, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709 ]
  %_12.i24478816 = phi float [ %_0.i3831.lcssa1232014107, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560.lr.ph ], [ %_0.i3831, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709 ], !dbg !40070
  %_0.i38388787 = phi float [ %_0.i3838.lcssa1233414104, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560.lr.ph ], [ %_0.i3838, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709 ], !dbg !40070
  %_5.i24418758 = phi float [ %_0.i.i4072.lcssa1234814101, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560.lr.ph ], [ %_0.i.i4072, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709 ], !dbg !40070
  %_12.i24598728 = phi float [ %_0.i3818.lcssa1236214098, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560.lr.ph ], [ %_0.i3818, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709 ], !dbg !40070
  %_0.i38258699 = phi float [ %_0.i3825.lcssa1237614095, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560.lr.ph ], [ %_0.i3825, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709 ], !dbg !40070
  %_5.i24538670 = phi float [ %_0.i.i4065.lcssa1239014077, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560.lr.ph ], [ %_0.i.i4065, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709 ], !dbg !40070
  %_12.i24718640 = phi float [ %_0.i3805.lcssa1240414074, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560.lr.ph ], [ %_0.i3805, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709 ], !dbg !40070
  %_0.i38128611 = phi float [ %_0.i3812.lcssa1241814071, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560.lr.ph ], [ %_0.i3812, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709 ], !dbg !40070
  %_5.i24658582 = phi float [ %_0.i.i4058.lcssa1243214068, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560.lr.ph ], [ %_0.i.i4058, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709 ], !dbg !40070
  %_12.i24838552 = phi float [ %_0.i3793.lcssa1244614065, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560.lr.ph ], [ %_0.i3793, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709 ], !dbg !40070
  %_0.i37998523 = phi float [ %_0.i3799.lcssa1246014062, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560.lr.ph ], [ %_0.i3799, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709 ], !dbg !40070
  %_5.i24778494 = phi float [ %_0.i.i.lcssa1247414044, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560.lr.ph ], [ %_0.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709 ], !dbg !40070
  %iter.i38.sroa.41.08492 = phi i64 [ 0, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560.lr.ph ], [ %_9.0.i5003, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709 ]
  %_9.0.i5003 = add nuw i64 %iter.i38.sroa.41.08492, 1, !dbg !40071
  %data.i.i.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_209.i499, i64 %iter.i38.sroa.41.08492, !dbg !40072
  %data.i.i.i.i5001 = getelementptr inbounds nuw float, ptr %_233.i510, i64 %iter.i38.sroa.41.08492, !dbg !40079
  %data.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_224.i509, i64 %iter.i38.sroa.41.08492, !dbg !40082
  %data.i5.i.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_215.i502, i64 %iter.i38.sroa.41.08492, !dbg !40085
  %_0.i3375 = fadd float %_5.i24778494, -1.000000e+00, !dbg !40088
  %_3.i.i = fcmp ogt float %_0.i3375, 0.000000e+00, !dbg !40092
  %_0.i.i = select i1 %_3.i.i, float %_0.i3375, float 0.000000e+00, !dbg !40095
  %_0.i2535 = fadd float %_0.i37998523, %_12.i24838552, !dbg !40097
  %_0.i3799 = select i1 %_3.i.i, float %_0.i2535, float %_13.i248556865688, !dbg !40099
  %_0.i3793 = select i1 %_3.i.i, float %_12.i24838552, float 0.000000e+00, !dbg !40101
  %_0.i3376 = fadd float %_5.i24658582, -1.000000e+00, !dbg !40103
  %_3.i.i4052 = fcmp ogt float %_0.i3376, 0.000000e+00, !dbg !40106
  %_0.i.i4058 = select i1 %_3.i.i4052, float %_0.i3376, float 0.000000e+00, !dbg !40109
  %_0.i2536 = fadd float %_0.i38128611, %_12.i24718640, !dbg !40111
  %_0.i3812 = select i1 %_3.i.i4052, float %_0.i2536, float %_13.i247356895691, !dbg !40113
  %_0.i3805 = select i1 %_3.i.i4052, float %_12.i24718640, float 0.000000e+00, !dbg !40115
  %_0.i3377 = fadd float %_5.i24538670, -1.000000e+00, !dbg !40117
  %_3.i.i4059 = fcmp ogt float %_0.i3377, 0.000000e+00, !dbg !40121
  %_0.i.i4065 = select i1 %_3.i.i4059, float %_0.i3377, float 0.000000e+00, !dbg !40124
  %_0.i2537 = fadd float %_0.i38258699, %_12.i24598728, !dbg !40126
  %_0.i3825 = select i1 %_3.i.i4059, float %_0.i2537, float %_13.i246156925694, !dbg !40128
  %_0.i3818 = select i1 %_3.i.i4059, float %_12.i24598728, float 0.000000e+00, !dbg !40130
  %_0.i3378 = fadd float %_5.i24418758, -1.000000e+00, !dbg !40132
  %_3.i.i4066 = fcmp ogt float %_0.i3378, 0.000000e+00, !dbg !40135
  %_0.i.i4072 = select i1 %_3.i.i4066, float %_0.i3378, float 0.000000e+00, !dbg !40138
  %_0.i2538 = fadd float %_0.i38388787, %_12.i24478816, !dbg !40140
  %_0.i3838 = select i1 %_3.i.i4066, float %_0.i2538, float %_13.i244956955697, !dbg !40142
  %_0.i3831 = select i1 %_3.i.i4066, float %_12.i24478816, float 0.000000e+00, !dbg !40144
  %_0.i3573 = load float, ptr %data.i.i.i.i.i.i, align 4, !dbg !40146, !alias.scope !40148, !noalias !39358, !noundef !12
  %_0.i3568 = load float, ptr %data.i.i.i.i5001, align 4, !dbg !40151, !alias.scope !40153, !noalias !39358, !noundef !12
  %_3.i.i4208 = fcmp ule float %_0.i3568, %_0.i3573, !dbg !40156
  %_6.i.i4210 = bitcast float %_0.i3568 to i32, !dbg !40159
  %_8.i.i4212 = bitcast float %_0.i3573 to i32, !dbg !40162
  %_4.i.i4215 = select i1 %_3.i.i4208, i32 %_8.i.i4212, i32 %_6.i.i4210, !dbg !40164
  %_5.i4004 = and i32 %_4.i.i4215, %.none.i61, !dbg !40165
  %_7.i4000 = and i32 %_9.i4006, %_6.i.i4210, !dbg !40167
  %_4.i4001 = or disjoint i32 %_5.i4004, %_7.i4000, !dbg !40169
  %_0.i4002 = bitcast i32 %_4.i4001 to float, !dbg !40170
  %_0.i3563 = load float, ptr %data.i.i.i.i.i.i.i.i, align 4, !dbg !40172, !alias.scope !40174, !noalias !39358, !noundef !12
  %_0.i3558 = load float, ptr %data.i5.i.i.i.i.i.i.i, align 4, !dbg !40177, !alias.scope !40179, !noalias !39358, !noundef !12
  %_235.i539 = add nuw i64 %iter.i38.sroa.41.08492, %ring_cursor.sroa.0.1.i4868962, !dbg !40182
  %_236.i540 = add nuw i64 %iter.i38.sroa.41.08492, %main_cursor.sroa.0.1.i4878963, !dbg !40185
  %_237.i541 = add nuw i64 %iter.i38.sroa.41.08492, %left_end.sroa.0.0.i, !dbg !40186
  %_238.i542 = add i64 %iter.i38.sroa.41.08492, %start1.sroa.0.0.i2338, !dbg !40187
  %_239.i543 = add nuw i64 %iter.i38.sroa.41.08492, %left_expiring.sroa.0.0.i, !dbg !40188
  %_7.i8.i253.i551 = add i64 %_235.i539, 1, !dbg !40189
  %or.cond.i11.i256.i554.not = icmp ult i64 %_235.i539, %_54.1.i251.i549, !dbg !40193
  br i1 %or.cond.i11.i256.i554.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i257.i555, label %bb4.i13.i312.i689, !dbg !40193, !prof !2740

bb4.i13.i312.i689:                                ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560
  store float %_0.i.i.lcssa1247414044, ptr %587, align 4
  store float %_0.i.i4065.lcssa1239014077, ptr %593, align 4
  store float %running.sroa.0.0.i1694.lcssa1248614121, ptr %_21.i264.i562, align 1, !dbg !39419
  store float %running.sroa.0.0.i.lcssa1252314138, ptr %_21.i.i637, align 1, !dbg !39452
  store float %_0.i340613983, ptr %602, align 1, !dbg !40200
  store float %_0.i378013998, ptr %604, align 1, !dbg !40204
  store float %_0.i340214013, ptr %610, align 1, !dbg !40208
  store float %_0.i377614028, ptr %612, align 1, !dbg !40209
  %umax11796 = call i64 @llvm.umax.i64(i64 %ring_cursor.sroa.0.1.i4868962, i64 %_54.1.i251.i549), !dbg !40066
  %635 = add i64 %umax11796, 1, !dbg !40066
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_235.i539, i64 noundef %635, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i251.i549, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !40210, !noalias !40211
  unreachable, !dbg !40210

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i257.i555: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560
  %_7.i4007 = and i32 %_9.i4006, %_8.i.i4212, !dbg !40219
  %_4.i4008 = or disjoint i32 %_5.i4004, %_7.i4007, !dbg !40165
  %_0.i4009 = bitcast i32 %_4.i4008 to float, !dbg !40220
  %_0.i2946 = fdiv float %_0.i3799, %_0.i4009, !dbg !40222
  %_3.i2513 = fcmp uge float %_0.i3799, %_0.i4009, !dbg !40224
  %_0.i3995 = select i1 %_3.i2513, float 1.000000e+00, float %_0.i2946, !dbg !40226
  %_17.i12.i258.i556 = getelementptr inbounds nuw float, ptr %_54.0.i250.i548, i64 %_235.i539, !dbg !40228
  store float %_0.i3995, ptr %_17.i12.i258.i556, align 4, !dbg !40232, !alias.scope !40234, !noalias !40237
  %or.cond.i1867.not = icmp ult i64 %_237.i541, %_54.1.i251.i549, !dbg !40238
  br i1 %or.cond.i1867.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1871, label %bb4.i1870, !dbg !40238, !prof !2740

bb4.i1870:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i257.i555
  store float %_0.i.i.lcssa1247414044, ptr %587, align 4
  store float %_0.i.i4065.lcssa1239014077, ptr %593, align 4
  store float %running.sroa.0.0.i1694.lcssa1248614121, ptr %_21.i264.i562, align 1, !dbg !39419
  store float %running.sroa.0.0.i.lcssa1252314138, ptr %_21.i.i637, align 1, !dbg !39452
  store float %_0.i340613983, ptr %602, align 1, !dbg !40200
  store float %_0.i378013998, ptr %604, align 1, !dbg !40204
  store float %_0.i340214013, ptr %610, align 1, !dbg !40208
  store float %_0.i377614028, ptr %612, align 1, !dbg !40209
  %_5.i1864 = add i64 %_237.i541, 1, !dbg !40248
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_237.i541, i64 noundef %_5.i1864, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i251.i549, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !40249, !noalias !40250
  unreachable, !dbg !40249

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1871: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i257.i555
  %_15.i1868 = getelementptr inbounds nuw float, ptr %_54.0.i250.i548, i64 %_237.i541, !dbg !40256
  %_0.i3440 = load float, ptr %_15.i1868, align 4, !dbg !40260, !alias.scope !40262, !noalias !40265, !noundef !12
  %position.i1690 = zext i32 %storemerge.i16998845 to i64, !dbg !40266
  %636 = icmp eq i32 %storemerge.i16998845, 0, !dbg !40267
  %_3.i.i4352.inv = fcmp olt float %running.sroa.0.0.i16948872, %_0.i3440, !dbg !40267
  %_4.i.i4359.v = select i1 %_3.i.i4352.inv, float %running.sroa.0.0.i16948872, float %_0.i3440, !dbg !40267
  %running.sroa.0.0.i1694 = select i1 %636, float %_0.i3440, float %_4.i.i4359.v, !dbg !40267
  %_15.i1695 = add nuw nsw i64 %position.i1690, 1, !dbg !40268
  %complete.i1696 = icmp eq i64 %_15.i1695, %_18.i261.i559, !dbg !40268
  br i1 %complete.i1696, label %bb19.i1707, label %bb7.i1697, !dbg !40270

bb7.i1697:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1871
  %or.cond.i1859.not = icmp ult i64 %_238.i542, %_54.1.i251.i549, !dbg !40272
  br i1 %or.cond.i1859.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1863, label %bb4.i1862, !dbg !40272, !prof !2740

bb4.i1862:                                        ; preds = %bb7.i1697
  store float %_0.i.i.lcssa1247414044, ptr %587, align 4
  store float %_0.i.i4065.lcssa1239014077, ptr %593, align 4
  store float %running.sroa.0.0.i1694.lcssa1248614121, ptr %_21.i264.i562, align 1, !dbg !39419
  store float %running.sroa.0.0.i.lcssa1252314138, ptr %_21.i.i637, align 1, !dbg !39452
  store float %_0.i340613983, ptr %602, align 1, !dbg !40200
  store float %_0.i378013998, ptr %604, align 1, !dbg !40204
  store float %_0.i340214013, ptr %610, align 1, !dbg !40208
  store float %_0.i377614028, ptr %612, align 1, !dbg !40209
  %_5.i1856 = add i64 %_238.i542, 1, !dbg !40277
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_238.i542, i64 noundef %_5.i1856, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i251.i549, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !40278, !noalias !40279
  unreachable, !dbg !40278

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1863: ; preds = %bb7.i1697
  %_15.i1860 = getelementptr inbounds nuw float, ptr %_54.0.i250.i548, i64 %_238.i542, !dbg !40282
  %_0.i3442 = load float, ptr %_15.i1860, align 4, !dbg !40284, !alias.scope !40286, !noalias !40265, !noundef !12
  %_3.i.i4343.inv = fcmp olt float %_0.i3442, %running.sroa.0.0.i1694, !dbg !40289
  %_4.i.i4350.v = select i1 %_3.i.i4343.inv, float %_0.i3442, float %running.sroa.0.0.i1694, !dbg !40289
  %637 = trunc i64 %_15.i1695 to i32, !dbg !40293
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1718, !dbg !40295

bb19.i1707:                                       ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1871, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1714
  %end.sroa.0.0.i17058486 = phi i64 [ %639, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1714 ], [ %_237.i541, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1871 ]
  %suffix.sroa.0.0.i17048485 = phi float [ %_4.i.i4341.v, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1714 ], [ %_0.i3440, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1871 ]
  %iter.sroa.0.0.i17038484 = phi i64 [ %_30.i1708, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1714 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1871 ]
  %or.cond.i1843.not = icmp ult i64 %end.sroa.0.0.i17058486, %_54.1.i251.i549, !dbg !40296
  br i1 %or.cond.i1843.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1714, label %bb4.i1846, !dbg !40296, !prof !2740

bb4.i1846:                                        ; preds = %bb19.i1707
  store float %_0.i.i.lcssa1247414044, ptr %587, align 4
  store float %_0.i.i4065.lcssa1239014077, ptr %593, align 4
  store float %running.sroa.0.0.i1694.lcssa1248614121, ptr %_21.i264.i562, align 1, !dbg !39419
  store float %running.sroa.0.0.i.lcssa1252314138, ptr %_21.i.i637, align 1, !dbg !39452
  store float %_0.i340613983, ptr %602, align 1, !dbg !40200
  store float %_0.i378013998, ptr %604, align 1, !dbg !40204
  store float %_0.i340214013, ptr %610, align 1, !dbg !40208
  store float %_0.i377614028, ptr %612, align 1, !dbg !40209
  %_5.i1840 = add i64 %end.sroa.0.0.i17058486, 1, !dbg !40304
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %end.sroa.0.0.i17058486, i64 noundef %_5.i1840, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i251.i549, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !40305, !noalias !40306
  unreachable, !dbg !40305

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1714: ; preds = %bb19.i1707
  %_30.i1708 = add nuw i64 %iter.sroa.0.0.i17038484, 1, !dbg !40309
  %_15.i1844 = getelementptr inbounds nuw float, ptr %_54.0.i250.i548, i64 %end.sroa.0.0.i17058486, !dbg !40320
  %_0.i3446 = load float, ptr %_15.i1844, align 4, !dbg !40322, !alias.scope !40324, !noalias !40265, !noundef !12
  %_3.i.i4334.inv = fcmp olt float %suffix.sroa.0.0.i17048485, %_0.i3446, !dbg !40327
  %_4.i.i4341.v = select i1 %_3.i.i4334.inv, float %suffix.sroa.0.0.i17048485, float %_0.i3446, !dbg !40327
  store float %_4.i.i4341.v, ptr %_15.i1844, align 4, !dbg !40330, !alias.scope !40333, !noalias !40265
  %638 = icmp eq i64 %end.sroa.0.0.i17058486, 0, !dbg !40336
  %spec.store.select.i1716 = select i1 %638, i64 %ring.i64, i64 %end.sroa.0.0.i17058486, !dbg !40336
  %639 = add i64 %spec.store.select.i1716, -1, !dbg !40337
  %exitcond11793.not = icmp eq i64 %_30.i1708, %umax11792, !dbg !40338
  br i1 %exitcond11793.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1718, label %bb19.i1707, !dbg !40341

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1718: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1714, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1863
  %storemerge.i1699 = phi i32 [ %637, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1863 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1714 ], !dbg !40342
  %running.sroa.0.1.i1700 = phi float [ %_4.i.i4350.v, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1863 ], [ %running.sroa.0.0.i1694, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1714 ], !dbg !39419
  %_0.i3176 = fmul float %running.sroa.0.1.i1700, 1.638400e+04, !dbg !40343
  %640 = tail call noundef float @llvm.floor.f32(float %_0.i3176), !dbg !40345
  %_0.i3175 = fmul float %640, 0x3F10000000000000, !dbg !40349
  %or.cond.i1931.not = icmp ult i64 %_239.i543, %_56.1.i273.i571, !dbg !40351
  br i1 %or.cond.i1931.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1935, label %bb4.i1934, !dbg !40351, !prof !2740

bb4.i1934:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1718
  store float %_0.i.i.lcssa1247414044, ptr %587, align 4
  store float %_0.i.i4065.lcssa1239014077, ptr %593, align 4
  store float %running.sroa.0.0.i1694.lcssa1248614121, ptr %_21.i264.i562, align 1, !dbg !39419
  store float %running.sroa.0.0.i.lcssa1252314138, ptr %_21.i.i637, align 1, !dbg !39452
  store float %_0.i340613983, ptr %602, align 1, !dbg !40200
  store float %_0.i378013998, ptr %604, align 1, !dbg !40204
  store float %_0.i340214013, ptr %610, align 1, !dbg !40208
  store float %_0.i377614028, ptr %612, align 1, !dbg !40209
  %_5.i1928 = add i64 %_239.i543, 1, !dbg !40356
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_239.i543, i64 noundef %_5.i1928, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i273.i571, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !40357, !noalias !40358
  unreachable, !dbg !40357

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1935: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1718
  %_15.i1932 = getelementptr inbounds nuw float, ptr %_56.0.i272.i570, i64 %_239.i543, !dbg !40361
  %_0.i3424 = load float, ptr %_15.i1932, align 4, !dbg !40363, !alias.scope !40365, !noalias !40368, !noundef !12
  %_0.i2740 = fadd float %_0.i3175, %_0.i340613983, !dbg !40369
  %_0.i3406 = fsub float %_0.i2740, %_0.i3424, !dbg !40371
  %_8.not.i3.i282.i580 = icmp ugt i64 %_7.i8.i253.i551, %_56.1.i273.i571
  br i1 %_8.not.i3.i282.i580, label %bb4.i6.i311.i688, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i283.i581, !dbg !40373, !prof !165

bb4.i6.i311.i688:                                 ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1935
  store float %_0.i.i.lcssa1247414044, ptr %587, align 4
  store float %_0.i.i4065.lcssa1239014077, ptr %593, align 4
  store float %running.sroa.0.0.i1694.lcssa1248614121, ptr %_21.i264.i562, align 1, !dbg !39419
  store float %running.sroa.0.0.i.lcssa1252314138, ptr %_21.i.i637, align 1, !dbg !39452
  store float %_0.i3406, ptr %602, align 1, !dbg !40200
  store float %_0.i378013998, ptr %604, align 1, !dbg !40204
  store float %_0.i340214013, ptr %610, align 1, !dbg !40208
  store float %_0.i377614028, ptr %612, align 1, !dbg !40209
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_235.i539, i64 noundef %_7.i8.i253.i551, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i273.i571, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !40378, !noalias !40379
  unreachable, !dbg !40378

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i283.i581: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1935
  %_17.i5.i284.i582 = getelementptr inbounds nuw float, ptr %_56.0.i272.i570, i64 %_235.i539, !dbg !40382
  store float %_0.i3175, ptr %_17.i5.i284.i582, align 4, !dbg !40384, !alias.scope !40386, !noalias !40368
  %_0.i2945 = fdiv float %_0.i3406, %_37.i285.i583, !dbg !40389
  %_0.i3405 = fsub float 1.000000e+00, %_0.i2945, !dbg !40391
  %_0.i3404 = fsub float %_0.i3405, %_0.i378013998, !dbg !40393
  %_4.i2961 = fmul float %_0.i3812, %_0.i3404, !dbg !40395
  %_0.i2962 = fadd float %_0.i378013998, %_4.i2961, !dbg !40395
  %_3.i.i4199.inv = fcmp ogt float %_0.i3405, %_0.i2962, !dbg !40397
  %_4.i.i4206.v = select i1 %_3.i.i4199.inv, float %_0.i3405, float %_0.i2962, !dbg !40397
  %641 = tail call noundef float @llvm.fabs.f32(float %_4.i.i4206.v), !dbg !40400
  %642 = fcmp uge float %641, 0x3BC79CA100000000, !dbg !40404
  %_0.i3780 = select i1 %642, float %_4.i.i4206.v, float 0.000000e+00, !dbg !40406
  %_5.i1920 = add i64 %_236.i540, 1, !dbg !40407
  %or.cond.i1923.not = icmp ult i64 %_236.i540, %_58.1.i298.i596, !dbg !40410
  br i1 %or.cond.i1923.not, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3716, label %bb4.i1926, !dbg !40410, !prof !2740

bb4.i1926:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i283.i581
  store float %_0.i.i.lcssa1247414044, ptr %587, align 4
  store float %_0.i.i4065.lcssa1239014077, ptr %593, align 4
  store float %running.sroa.0.0.i1694.lcssa1248614121, ptr %_21.i264.i562, align 1, !dbg !39419
  store float %running.sroa.0.0.i.lcssa1252314138, ptr %_21.i.i637, align 1, !dbg !39452
  store float %_0.i3406, ptr %602, align 1, !dbg !40200
  store float %_0.i3780, ptr %604, align 1, !dbg !40204
  store float %_0.i340214013, ptr %610, align 1, !dbg !40208
  store float %_0.i377614028, ptr %612, align 1, !dbg !40209
  %umax11797 = call i64 @llvm.umax.i64(i64 %main_cursor.sroa.0.1.i4878963, i64 %_58.1.i298.i596), !dbg !40066
  %643 = add i64 %umax11797, 1, !dbg !40066
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_236.i540, i64 noundef %643, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i298.i596, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !40414, !noalias !40415
  unreachable, !dbg !40414

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3716: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i283.i581
  %_0.i3403 = fsub float 1.000000e+00, %_0.i3780, !dbg !40418
  %_15.i1924 = getelementptr inbounds nuw float, ptr %_58.0.i297.i595, i64 %_236.i540, !dbg !40420
  %_0.i3426 = load float, ptr %_15.i1924, align 4, !dbg !40422, !alias.scope !40424, !noalias !40368, !noundef !12
  store float %_0.i3563, ptr %_15.i1924, align 4, !dbg !40427, !alias.scope !40431, !noalias !40368
  %_0.i3174 = fmul float %_0.i3403, %_0.i3426, !dbg !40434
  %_6.i3983 = bitcast float %_0.i3426 to i32, !dbg !40436
  %_5.i3984 = and i32 %_6.i3983, %all.sroa.0.0.i63, !dbg !40439
  %_8.i3985 = bitcast float %_0.i3174 to i32, !dbg !40440
  %_7.i3987 = and i32 %_9.i3986, %_8.i3985, !dbg !40442
  %_4.i3988 = or disjoint i32 %_7.i3987, %_5.i3984, !dbg !40439
  store i32 %_4.i3988, ptr %data.i.i.i.i.i.i.i.i, align 4, !dbg !40443, !alias.scope !40445, !noalias !40448
  %_242.i616 = add nuw i64 %iter.i38.sroa.41.08492, %right_end.sroa.0.0.i, !dbg !40449
  %_244.i618 = add nuw i64 %iter.i38.sroa.41.08492, %right_expiring.sroa.0.0.i, !dbg !40451
  %_8.not.i10.i.i628 = icmp ugt i64 %_7.i8.i253.i551, %_54.1.i.i624
  br i1 %_8.not.i10.i.i628, label %bb4.i13.i.i686, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i630, !dbg !40452, !prof !165

bb4.i13.i.i686:                                   ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3716
  store float %_0.i.i.lcssa1247414044, ptr %587, align 4
  store float %_0.i.i4065.lcssa1239014077, ptr %593, align 4
  store float %running.sroa.0.0.i1694.lcssa1248614121, ptr %_21.i264.i562, align 1, !dbg !39419
  store float %running.sroa.0.0.i.lcssa1252314138, ptr %_21.i.i637, align 1, !dbg !39452
  store float %_0.i3406, ptr %602, align 1, !dbg !40200
  store float %_0.i3780, ptr %604, align 1, !dbg !40204
  store float %_0.i340214013, ptr %610, align 1, !dbg !40208
  store float %_0.i377614028, ptr %612, align 1, !dbg !40209
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_235.i539, i64 noundef %_7.i8.i253.i551, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i624, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !40457, !noalias !40458
  unreachable, !dbg !40457

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i630: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3716
  %_0.i2944 = fdiv float %_0.i3825, %_0.i4002, !dbg !40466
  %_3.i2511 = fcmp uge float %_0.i3825, %_0.i4002, !dbg !40468
  %_0.i3982 = select i1 %_3.i2511, float 1.000000e+00, float %_0.i2944, !dbg !40470
  %_17.i12.i.i631 = getelementptr inbounds nuw float, ptr %_54.0.i.i623, i64 %_235.i539, !dbg !40472
  store float %_0.i3982, ptr %_17.i12.i.i631, align 4, !dbg !40474, !alias.scope !40476, !noalias !40479
  %or.cond.i1899.not = icmp ult i64 %_242.i616, %_54.1.i.i624, !dbg !40480
  br i1 %or.cond.i1899.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1903, label %bb4.i1902, !dbg !40480, !prof !2740

bb4.i1902:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i630
  store float %_0.i.i.lcssa1247414044, ptr %587, align 4
  store float %_0.i.i4065.lcssa1239014077, ptr %593, align 4
  store float %running.sroa.0.0.i1694.lcssa1248614121, ptr %_21.i264.i562, align 1, !dbg !39419
  store float %running.sroa.0.0.i.lcssa1252314138, ptr %_21.i.i637, align 1, !dbg !39452
  store float %_0.i3406, ptr %602, align 1, !dbg !40200
  store float %_0.i3780, ptr %604, align 1, !dbg !40204
  store float %_0.i340214013, ptr %610, align 1, !dbg !40208
  store float %_0.i377614028, ptr %612, align 1, !dbg !40209
  %_5.i1896 = add i64 %_242.i616, 1, !dbg !40485
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_242.i616, i64 noundef %_5.i1896, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i624, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !40486, !noalias !40487
  unreachable, !dbg !40486

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1903: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i630
  %_15.i1900 = getelementptr inbounds nuw float, ptr %_54.0.i.i623, i64 %_242.i616, !dbg !40493
  %_0.i3432 = load float, ptr %_15.i1900, align 4, !dbg !40495, !alias.scope !40497, !noalias !40500, !noundef !12
  %position.i = zext i32 %storemerge.i8903 to i64, !dbg !40501
  %644 = icmp eq i32 %storemerge.i8903, 0, !dbg !40502
  %_3.i.i4379.inv = fcmp olt float %running.sroa.0.0.i8930, %_0.i3432, !dbg !40502
  %_4.i.i4386.v = select i1 %_3.i.i4379.inv, float %running.sroa.0.0.i8930, float %_0.i3432, !dbg !40502
  %running.sroa.0.0.i = select i1 %644, float %_0.i3432, float %_4.i.i4386.v, !dbg !40502
  %_15.i = add nuw nsw i64 %position.i, 1, !dbg !40503
  %complete.i = icmp eq i64 %_15.i, %_18.i.i634, !dbg !40503
  br i1 %complete.i, label %bb19.i1683, label %bb7.i1679, !dbg !40504

bb7.i1679:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1903
  %or.cond.i1891.not = icmp ult i64 %_238.i542, %_54.1.i.i624, !dbg !40505
  br i1 %or.cond.i1891.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1895, label %bb4.i1894, !dbg !40505, !prof !2740

bb4.i1894:                                        ; preds = %bb7.i1679
  store float %_0.i.i.lcssa1247414044, ptr %587, align 4
  store float %_0.i.i4065.lcssa1239014077, ptr %593, align 4
  store float %running.sroa.0.0.i1694.lcssa1248614121, ptr %_21.i264.i562, align 1, !dbg !39419
  store float %running.sroa.0.0.i.lcssa1252314138, ptr %_21.i.i637, align 1, !dbg !39452
  store float %_0.i3406, ptr %602, align 1, !dbg !40200
  store float %_0.i3780, ptr %604, align 1, !dbg !40204
  store float %_0.i340214013, ptr %610, align 1, !dbg !40208
  store float %_0.i377614028, ptr %612, align 1, !dbg !40209
  %_5.i1888 = add i64 %_238.i542, 1, !dbg !40510
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_238.i542, i64 noundef %_5.i1888, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i624, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !40511, !noalias !40512
  unreachable, !dbg !40511

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1895: ; preds = %bb7.i1679
  %_15.i1892 = getelementptr inbounds nuw float, ptr %_54.0.i.i623, i64 %_238.i542, !dbg !40515
  %_0.i3434 = load float, ptr %_15.i1892, align 4, !dbg !40517, !alias.scope !40519, !noalias !40500, !noundef !12
  %_3.i.i4370.inv = fcmp olt float %_0.i3434, %running.sroa.0.0.i, !dbg !40522
  %_4.i.i4377.v = select i1 %_3.i.i4370.inv, float %_0.i3434, float %running.sroa.0.0.i, !dbg !40522
  %645 = trunc i64 %_15.i to i32, !dbg !40525
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit, !dbg !40526

bb19.i1683:                                       ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1903, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i
  %end.sroa.0.0.i8489 = phi i64 [ %647, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], [ %_242.i616, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1903 ]
  %suffix.sroa.0.0.i8488 = phi float [ %_4.i.i4368.v, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], [ %_0.i3432, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1903 ]
  %iter.sroa.0.0.i16828487 = phi i64 [ %_30.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1903 ]
  %or.cond.i1875.not = icmp ult i64 %end.sroa.0.0.i8489, %_54.1.i.i624, !dbg !40527
  br i1 %or.cond.i1875.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i, label %bb4.i1878, !dbg !40527, !prof !2740

bb4.i1878:                                        ; preds = %bb19.i1683
  store float %_0.i.i.lcssa1247414044, ptr %587, align 4
  store float %_0.i.i4065.lcssa1239014077, ptr %593, align 4
  store float %running.sroa.0.0.i1694.lcssa1248614121, ptr %_21.i264.i562, align 1, !dbg !39419
  store float %running.sroa.0.0.i.lcssa1252314138, ptr %_21.i.i637, align 1, !dbg !39452
  store float %_0.i3406, ptr %602, align 1, !dbg !40200
  store float %_0.i3780, ptr %604, align 1, !dbg !40204
  store float %_0.i340214013, ptr %610, align 1, !dbg !40208
  store float %_0.i377614028, ptr %612, align 1, !dbg !40209
  %_5.i1872 = add i64 %end.sroa.0.0.i8489, 1, !dbg !40532
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %end.sroa.0.0.i8489, i64 noundef %_5.i1872, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i624, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !40533, !noalias !40534
  unreachable, !dbg !40533

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i: ; preds = %bb19.i1683
  %_30.i = add nuw i64 %iter.sroa.0.0.i16828487, 1, !dbg !40537
  %_15.i1876 = getelementptr inbounds nuw float, ptr %_54.0.i.i623, i64 %end.sroa.0.0.i8489, !dbg !40542
  %_0.i3438 = load float, ptr %_15.i1876, align 4, !dbg !40544, !alias.scope !40546, !noalias !40500, !noundef !12
  %_3.i.i4361.inv = fcmp olt float %suffix.sroa.0.0.i8488, %_0.i3438, !dbg !40549
  %_4.i.i4368.v = select i1 %_3.i.i4361.inv, float %suffix.sroa.0.0.i8488, float %_0.i3438, !dbg !40549
  store float %_4.i.i4368.v, ptr %_15.i1876, align 4, !dbg !40552, !alias.scope !40555, !noalias !40500
  %646 = icmp eq i64 %end.sroa.0.0.i8489, 0, !dbg !40558
  %spec.store.select.i1686 = select i1 %646, i64 %ring.i64, i64 %end.sroa.0.0.i8489, !dbg !40558
  %647 = add i64 %spec.store.select.i1686, -1, !dbg !40559
  %exitcond11795.not = icmp eq i64 %_30.i, %umax11794, !dbg !40560
  br i1 %exitcond11795.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit, label %bb19.i1683, !dbg !40562

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1895
  %storemerge.i = phi i32 [ %645, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1895 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], !dbg !40563
  %running.sroa.0.1.i = phi float [ %_4.i.i4377.v, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1895 ], [ %running.sroa.0.0.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], !dbg !39452
  %_0.i3173 = fmul float %running.sroa.0.1.i, 1.638400e+04, !dbg !40564
  %648 = tail call noundef float @llvm.floor.f32(float %_0.i3173), !dbg !40566
  %_0.i3172 = fmul float %648, 0x3F10000000000000, !dbg !40570
  %or.cond.i1915.not = icmp ult i64 %_244.i618, %_56.1.i.i646, !dbg !40572
  br i1 %or.cond.i1915.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1919, label %bb4.i1918, !dbg !40572, !prof !2740

bb4.i1918:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit
  store float %_0.i.i.lcssa1247414044, ptr %587, align 4
  store float %_0.i.i4065.lcssa1239014077, ptr %593, align 4
  store float %running.sroa.0.0.i1694.lcssa1248614121, ptr %_21.i264.i562, align 1, !dbg !39419
  store float %running.sroa.0.0.i.lcssa1252314138, ptr %_21.i.i637, align 1, !dbg !39452
  store float %_0.i3406, ptr %602, align 1, !dbg !40200
  store float %_0.i3780, ptr %604, align 1, !dbg !40204
  store float %_0.i340214013, ptr %610, align 1, !dbg !40208
  store float %_0.i377614028, ptr %612, align 1, !dbg !40209
  %_5.i1912 = add i64 %_244.i618, 1, !dbg !40577
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_244.i618, i64 noundef %_5.i1912, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i646, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !40578, !noalias !40579
  unreachable, !dbg !40578

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1919: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit
  %_15.i1916 = getelementptr inbounds nuw float, ptr %_56.0.i.i645, i64 %_244.i618, !dbg !40582
  %_0.i3428 = load float, ptr %_15.i1916, align 4, !dbg !40584, !alias.scope !40586, !noalias !40589, !noundef !12
  %_0.i2739 = fadd float %_0.i3172, %_0.i340214013, !dbg !40590
  %_0.i3402 = fsub float %_0.i2739, %_0.i3428, !dbg !40592
  %_8.not.i3.i.i655 = icmp ugt i64 %_7.i8.i253.i551, %_56.1.i.i646
  br i1 %_8.not.i3.i.i655, label %bb4.i6.i.i685, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i656, !dbg !40594, !prof !165

bb4.i6.i.i685:                                    ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1919
  store float %_0.i.i.lcssa1247414044, ptr %587, align 4
  store float %_0.i.i4065.lcssa1239014077, ptr %593, align 4
  store float %running.sroa.0.0.i1694.lcssa1248614121, ptr %_21.i264.i562, align 1, !dbg !39419
  store float %running.sroa.0.0.i.lcssa1252314138, ptr %_21.i.i637, align 1, !dbg !39452
  store float %_0.i3406, ptr %602, align 1, !dbg !40200
  store float %_0.i3780, ptr %604, align 1, !dbg !40204
  store float %_0.i3402, ptr %610, align 1, !dbg !40208
  store float %_0.i377614028, ptr %612, align 1, !dbg !40209
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_235.i539, i64 noundef %_7.i8.i253.i551, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i646, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !40599, !noalias !40600
  unreachable, !dbg !40599

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i656: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1919
  %_17.i5.i.i657 = getelementptr inbounds nuw float, ptr %_56.0.i.i645, i64 %_235.i539, !dbg !40603
  store float %_0.i3172, ptr %_17.i5.i.i657, align 4, !dbg !40605, !alias.scope !40607, !noalias !40589
  %_0.i2943 = fdiv float %_0.i3402, %_37.i.i658, !dbg !40610
  %_0.i3401 = fsub float 1.000000e+00, %_0.i2943, !dbg !40612
  %_0.i3400 = fsub float %_0.i3401, %_0.i377614028, !dbg !40614
  %_4.i2959 = fmul float %_0.i3838, %_0.i3400, !dbg !40616
  %_0.i2960 = fadd float %_0.i377614028, %_4.i2959, !dbg !40616
  %_3.i.i4190.inv = fcmp ogt float %_0.i3401, %_0.i2960, !dbg !40618
  %_4.i.i4197.v = select i1 %_3.i.i4190.inv, float %_0.i3401, float %_0.i2960, !dbg !40618
  %649 = tail call noundef float @llvm.fabs.f32(float %_4.i.i4197.v), !dbg !40621
  %650 = fcmp uge float %649, 0x3BC79CA100000000, !dbg !40624
  %_0.i3776 = select i1 %650, float %_4.i.i4197.v, float 0.000000e+00, !dbg !40626
  %_6.not.i1906 = icmp ugt i64 %_5.i1920, %_58.1.i.i671
  br i1 %_6.not.i1906, label %bb4.i1910, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709, !dbg !40627, !prof !165

bb4.i1910:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i656
  store float %_0.i.i.lcssa1247414044, ptr %587, align 4
  store float %_0.i.i4065.lcssa1239014077, ptr %593, align 4
  store float %running.sroa.0.0.i1694.lcssa1248614121, ptr %_21.i264.i562, align 1, !dbg !39419
  store float %running.sroa.0.0.i.lcssa1252314138, ptr %_21.i.i637, align 1, !dbg !39452
  store float %_0.i3406, ptr %602, align 1, !dbg !40200
  store float %_0.i3780, ptr %604, align 1, !dbg !40204
  store float %_0.i3402, ptr %610, align 1, !dbg !40208
  store float %_0.i3776, ptr %612, align 1, !dbg !40209
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_236.i540, i64 noundef %_5.i1920, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i.i671, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !40632, !noalias !40633
  unreachable, !dbg !40632

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i656
  %_0.i3399 = fsub float 1.000000e+00, %_0.i3776, !dbg !40636
  %_15.i1908 = getelementptr inbounds nuw float, ptr %_58.0.i.i670, i64 %_236.i540, !dbg !40638
  %_0.i3430 = load float, ptr %_15.i1908, align 4, !dbg !40640, !alias.scope !40642, !noalias !40589, !noundef !12
  store float %_0.i3558, ptr %_15.i1908, align 4, !dbg !40645, !alias.scope !40648, !noalias !40589
  %_0.i3171 = fmul float %_0.i3399, %_0.i3430, !dbg !40651
  %_6.i3970 = bitcast float %_0.i3430 to i32, !dbg !40653
  %_5.i3971 = and i32 %_6.i3970, %all.sroa.0.0.i63, !dbg !40656
  %_8.i3972 = bitcast float %_0.i3171 to i32, !dbg !40657
  %_7.i3974 = and i32 %_9.i3986, %_8.i3972, !dbg !40659
  %_4.i3975 = or disjoint i32 %_7.i3974, %_5.i3971, !dbg !40656
  store i32 %_4.i3975, ptr %data.i5.i.i.i.i.i.i.i, align 4, !dbg !40660, !alias.scope !40662, !noalias !40665
  %exitcond11806.not = icmp eq i64 %_9.0.i5003, %umin11805, !dbg !40066
  br i1 %exitcond11806.not, label %bb29.i512.bb32.i690_crit_edge, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3560, !dbg !40066

bb29.i512.bb32.i690_crit_edge:                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3709
  store float %_0.i3406, ptr %602, align 1, !dbg !40200
  store float %_0.i3780, ptr %604, align 1, !dbg !40204
  store float %_0.i3402, ptr %610, align 1, !dbg !40208
  store float %_0.i3776, ptr %612, align 1, !dbg !40209
  store float %_0.i3799, ptr %_114.i523, align 4, !dbg !40666, !alias.scope !40667, !noalias !40670
  store float %_0.i3793, ptr %588, align 4, !dbg !40673, !alias.scope !40667, !noalias !40670
  store float %_0.i.i4058, ptr %590, align 4, !dbg !40674, !alias.scope !40675, !noalias !39358
  store float %_0.i3812, ptr %_115.i524, align 4, !dbg !40678, !alias.scope !40675, !noalias !39358
  store float %_0.i3805, ptr %591, align 4, !dbg !40679, !alias.scope !40675, !noalias !39358
  store float %_0.i3825, ptr %_119.i525, align 4, !dbg !40680, !alias.scope !40681, !noalias !40684
  store float %_0.i3818, ptr %594, align 4, !dbg !40687, !alias.scope !40681, !noalias !40684
  store float %_0.i.i4072, ptr %596, align 4, !dbg !40688, !alias.scope !40689, !noalias !39358
  store float %_0.i3838, ptr %_120.i526, align 4, !dbg !40692, !alias.scope !40689, !noalias !39358
  store float %_0.i3831, ptr %597, align 4, !dbg !40693, !alias.scope !40689, !noalias !39358
  br label %bb32.i690, !dbg !40066

bb32.i690:                                        ; preds = %bb29.i512.bb32.i690_crit_edge, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit
  %running.sroa.0.0.i.lcssa1252314137 = phi float [ %running.sroa.0.0.i, %bb29.i512.bb32.i690_crit_edge ], [ %running.sroa.0.0.i.lcssa1252314138, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %running.sroa.0.0.i1694.lcssa1248614120 = phi float [ %running.sroa.0.0.i1694, %bb29.i512.bb32.i690_crit_edge ], [ %running.sroa.0.0.i1694.lcssa1248614121, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i3776.lcssa1404114118 = phi float [ %_0.i3776, %bb29.i512.bb32.i690_crit_edge ], [ %_0.i3776.lcssa1404114119, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i3402.lcssa1402614115 = phi float [ %_0.i3402, %bb29.i512.bb32.i690_crit_edge ], [ %_0.i3402.lcssa1402614116, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i3780.lcssa1401114112 = phi float [ %_0.i3780, %bb29.i512.bb32.i690_crit_edge ], [ %_0.i3780.lcssa1401114113, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i3406.lcssa1399614109 = phi float [ %_0.i3406, %bb29.i512.bb32.i690_crit_edge ], [ %_0.i3406.lcssa1399614110, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i3831.lcssa1232014106 = phi float [ %_0.i3831, %bb29.i512.bb32.i690_crit_edge ], [ %_0.i3831.lcssa1232014107, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i3838.lcssa1233414103 = phi float [ %_0.i3838, %bb29.i512.bb32.i690_crit_edge ], [ %_0.i3838.lcssa1233414104, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i.i4072.lcssa1234814100 = phi float [ %_0.i.i4072, %bb29.i512.bb32.i690_crit_edge ], [ %_0.i.i4072.lcssa1234814101, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i3818.lcssa1236214097 = phi float [ %_0.i3818, %bb29.i512.bb32.i690_crit_edge ], [ %_0.i3818.lcssa1236214098, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i3825.lcssa1237614094 = phi float [ %_0.i3825, %bb29.i512.bb32.i690_crit_edge ], [ %_0.i3825.lcssa1237614095, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i.i4065.lcssa1239014076 = phi float [ %_0.i.i4065, %bb29.i512.bb32.i690_crit_edge ], [ %_0.i.i4065.lcssa1239014077, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i3805.lcssa1240414073 = phi float [ %_0.i3805, %bb29.i512.bb32.i690_crit_edge ], [ %_0.i3805.lcssa1240414074, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i3812.lcssa1241814070 = phi float [ %_0.i3812, %bb29.i512.bb32.i690_crit_edge ], [ %_0.i3812.lcssa1241814071, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i.i4058.lcssa1243214067 = phi float [ %_0.i.i4058, %bb29.i512.bb32.i690_crit_edge ], [ %_0.i.i4058.lcssa1243214068, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i3793.lcssa1244614064 = phi float [ %_0.i3793, %bb29.i512.bb32.i690_crit_edge ], [ %_0.i3793.lcssa1244614065, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i3799.lcssa1246014061 = phi float [ %_0.i3799, %bb29.i512.bb32.i690_crit_edge ], [ %_0.i3799.lcssa1246014062, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i.i.lcssa1247414043 = phi float [ %_0.i.i, %bb29.i512.bb32.i690_crit_edge ], [ %_0.i.i.lcssa1247414044, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %running.sroa.0.0.i.lcssa89529043 = phi float [ %running.sroa.0.0.i, %bb29.i512.bb32.i690_crit_edge ], [ %running.sroa.0.0.i.lcssa89529044, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %storemerge.i.lcssa89259007 = phi i32 [ %storemerge.i, %bb29.i512.bb32.i690_crit_edge ], [ %storemerge.i.lcssa89259008, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %running.sroa.0.0.i1694.lcssa88949004 = phi float [ %running.sroa.0.0.i1694, %bb29.i512.bb32.i690_crit_edge ], [ %running.sroa.0.0.i1694.lcssa88949005, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %storemerge.i1699.lcssa88678968 = phi i32 [ %storemerge.i1699, %bb29.i512.bb32.i690_crit_edge ], [ %storemerge.i1699.lcssa88678969, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_143.i691 = add i64 %..i4939, %ring_cursor.sroa.0.1.i4868962, !dbg !40694
  %_234.not.i692 = icmp ult i64 %_143.i691, %ring.i64, !dbg !40695
  %651 = select i1 %_234.not.i692, i64 0, i64 %ring.i64, !dbg !40695
  %ring_cursor.sroa.0.2.i693 = sub nuw i64 %_143.i691, %651, !dbg !40695
  %_145.i694 = add i64 %..i4939, %main_cursor.sroa.0.1.i4878963, !dbg !40698
  %_245.not.i695 = icmp ult i64 %_145.i694, %main.i65, !dbg !40699
  %652 = select i1 %_245.not.i695, i64 0, i64 %main.i65, !dbg !40699
  %main_cursor.sroa.0.2.i696 = sub nuw i64 %_145.i694, %652, !dbg !40699
  %_63.i489 = icmp ult i64 %_85.i503, %..i4820, !dbg !39455
  br i1 %_63.i489, label %bb20.i490, label %bb19.i485.bb15.i73.loopexit_crit_edge, !dbg !39455

_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit.loopexit: ; preds = %bb15.i73.loopexit
  %653 = trunc i64 %main_cursor.sroa.0.1.i487.lcssa to i32, !dbg !40701
  %654 = trunc i64 %ring_cursor.sroa.0.1.i486.lcssa to i32, !dbg !40703
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit, !dbg !40704

_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit.loopexit, %bb7.i
  %ring_cursor.sroa.0.0.i74.lcssa = phi i32 [ %_36.i67, %bb7.i ], [ %654, %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit.loopexit ], !dbg !39388
  %main_cursor.sroa.0.0.i75.lcssa = phi i32 [ %_35.i66, %bb7.i ], [ %653, %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit.loopexit ], !dbg !39385
  %655 = getelementptr inbounds nuw i8, ptr %uniform_left.i52, i64 72, !dbg !40704
  %left_prefix.i701 = load float, ptr %655, align 8, !dbg !40704, !noalias !39364, !noundef !12
  %656 = getelementptr inbounds nuw i8, ptr %uniform_left.i52, i64 76, !dbg !40705
  %left_phase.i702 = load i32, ptr %656, align 4, !dbg !40705, !noalias !39364, !noundef !12
  %657 = getelementptr inbounds nuw i8, ptr %uniform_right.i51, i64 72, !dbg !40706
  %right_prefix.i703 = load float, ptr %657, align 8, !dbg !40706, !noalias !39364, !noundef !12
  %658 = getelementptr inbounds nuw i8, ptr %uniform_right.i51, i64 76, !dbg !40707
  %right_phase.i704 = load i32, ptr %658, align 4, !dbg !40707, !noalias !39364, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_right.i51), !dbg !40708, !noalias !39364
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i52), !dbg !40709, !noalias !39364
  %659 = getelementptr inbounds nuw i8, ptr %self, i64 216, !dbg !40710
  %_260.0.i705 = load ptr, ptr %659, align 8, !dbg !40710, !alias.scope !39354, !noalias !40711, !nonnull !12, !noundef !12
  %660 = getelementptr inbounds nuw i8, ptr %self, i64 224, !dbg !40710
  %_260.1.i706 = load i64, ptr %660, align 8, !dbg !40710, !alias.scope !39354, !noalias !40711, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !40712), !dbg !40715
  %_4.not.i3702 = icmp eq i64 %_260.1.i706, 0, !dbg !40716
  br i1 %_4.not.i3702, label %panic.i3704, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3705, !dbg !40716

panic.i3704:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !40716, !noalias !40718
  unreachable, !dbg !40716

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3705: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit
  store float %left_prefix.i701, ptr %_260.0.i705, align 4, !dbg !40716, !alias.scope !40712, !noalias !39358
  %_261.0.i707 = load ptr, ptr %68, align 8, !dbg !40719, !alias.scope !39354, !noalias !40711, !nonnull !12, !noundef !12
  %_261.1.i708 = load i64, ptr %69, align 8, !dbg !40719, !alias.scope !39354, !noalias !40711, !noundef !12
  %661 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i702), !dbg !40720
  br i1 %661, label %bb2.i5009, label %bb6.i5004, !dbg !40720

bb6.i5004:                                        ; preds = %bb2.i5009, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3705
  %end_or_len.idx.i = shl nuw nsw i64 %_261.1.i708, 2, !dbg !40724
  %end_or_len.i = getelementptr inbounds nuw i8, ptr %_261.0.i707, i64 %end_or_len.idx.i, !dbg !40724
  %_293.i = icmp eq i64 %_261.1.i708, 0, !dbg !40728
  br i1 %_293.i, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i5005, !dbg !40731

bb2.i5009:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3705
  %bytes1.sroa.0.0.zext.i = and i32 %left_phase.i702, 255, !dbg !40732
  %bytes1.sroa.0.0.isplat.i = mul nuw i32 %bytes1.sroa.0.0.zext.i, 16843009, !dbg !40732
  %_5.i5010 = icmp eq i32 %left_phase.i702, %bytes1.sroa.0.0.isplat.i, !dbg !40733
  br i1 %_5.i5010, label %bb3.i5011, label %bb6.i5004, !dbg !40733

bb3.i5011:                                        ; preds = %bb2.i5009
  %bytes.sroa.0.0.extract.trunc.i = trunc i32 %left_phase.i702 to i8, !dbg !40734
  %662 = shl nuw nsw i64 %_261.1.i708, 2, !dbg !40736
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_261.0.i707, i8 %bytes.sroa.0.0.extract.trunc.i, i64 %662, i1 false), !dbg !40736, !alias.scope !40737, !noalias !39358
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, !dbg !40740

bb10.i5005:                                       ; preds = %bb6.i5004, %bb10.i5005
  %iter.sroa.0.04.i = phi ptr [ %_38.i5006, %bb10.i5005 ], [ %_261.0.i707, %bb6.i5004 ]
  %_38.i5006 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i, i64 4, !dbg !40741
  store i32 %left_phase.i702, ptr %iter.sroa.0.04.i, align 4, !dbg !40743, !alias.scope !40737, !noalias !39358
  %_29.i5007 = icmp eq ptr %_38.i5006, %end_or_len.i, !dbg !40728
  br i1 %_29.i5007, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i5005, !dbg !40731

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit: ; preds = %bb10.i5005, %bb6.i5004, %bb3.i5011
  %663 = getelementptr inbounds nuw i8, ptr %self, i64 416, !dbg !40744
  %_262.0.i709 = load ptr, ptr %663, align 8, !dbg !40744, !alias.scope !39356, !noalias !40745, !nonnull !12, !noundef !12
  %664 = getelementptr inbounds nuw i8, ptr %self, i64 424, !dbg !40744
  %_262.1.i710 = load i64, ptr %664, align 8, !dbg !40744, !alias.scope !39356, !noalias !40745, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !40746), !dbg !40749
  %_4.not.i3698 = icmp eq i64 %_262.1.i710, 0, !dbg !40750
  br i1 %_4.not.i3698, label %panic.i3700, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3701, !dbg !40750

panic.i3700:                                      ; preds = %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !40750, !noalias !40752
  unreachable, !dbg !40750

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3701: ; preds = %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit
  store float %right_prefix.i703, ptr %_262.0.i709, align 4, !dbg !40750, !alias.scope !40746, !noalias !39358
  %_263.0.i711 = load ptr, ptr %77, align 8, !dbg !40753, !alias.scope !39356, !noalias !40745, !nonnull !12, !noundef !12
  %_263.1.i712 = load i64, ptr %78, align 8, !dbg !40753, !alias.scope !39356, !noalias !40745, !noundef !12
  %665 = tail call i1 @llvm.is.constant.i32(i32 %right_phase.i704), !dbg !40754
  br i1 %665, label %bb2.i5021, label %bb6.i5012, !dbg !40754

bb6.i5012:                                        ; preds = %bb2.i5021, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3701
  %end_or_len.idx.i5013 = shl nuw nsw i64 %_263.1.i712, 2, !dbg !40757
  %end_or_len.i5014 = getelementptr inbounds nuw i8, ptr %_263.0.i711, i64 %end_or_len.idx.i5013, !dbg !40757
  %_293.i5015 = icmp eq i64 %_263.1.i712, 0, !dbg !40761
  br i1 %_293.i5015, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5027, label %bb10.i5016, !dbg !40764

bb2.i5021:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3701
  %bytes1.sroa.0.0.zext.i5022 = and i32 %right_phase.i704, 255, !dbg !40765
  %bytes1.sroa.0.0.isplat.i5023 = mul nuw i32 %bytes1.sroa.0.0.zext.i5022, 16843009, !dbg !40765
  %_5.i5024 = icmp eq i32 %right_phase.i704, %bytes1.sroa.0.0.isplat.i5023, !dbg !40766
  br i1 %_5.i5024, label %bb3.i5025, label %bb6.i5012, !dbg !40766

bb3.i5025:                                        ; preds = %bb2.i5021
  %bytes.sroa.0.0.extract.trunc.i5026 = trunc i32 %right_phase.i704 to i8, !dbg !40767
  %666 = shl nuw nsw i64 %_263.1.i712, 2, !dbg !40769
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_263.0.i711, i8 %bytes.sroa.0.0.extract.trunc.i5026, i64 %666, i1 false), !dbg !40769, !alias.scope !40770, !noalias !39358
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5027, !dbg !40773

bb10.i5016:                                       ; preds = %bb6.i5012, %bb10.i5016
  %iter.sroa.0.04.i5017 = phi ptr [ %_38.i5018, %bb10.i5016 ], [ %_263.0.i711, %bb6.i5012 ]
  %_38.i5018 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i5017, i64 4, !dbg !40774
  store i32 %right_phase.i704, ptr %iter.sroa.0.04.i5017, align 4, !dbg !40776, !alias.scope !40770, !noalias !39358
  %_29.i5019 = icmp eq ptr %_38.i5018, %end_or_len.i5014, !dbg !40761
  br i1 %_29.i5019, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5027, label %bb10.i5016, !dbg !40764

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5027: ; preds = %bb10.i5016, %bb6.i5012, %bb3.i5025
  call void @llvm.lifetime.start.p0(ptr nonnull %_157.i34), !dbg !40777, !noalias !39364
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(92) %_157.i34, ptr noundef nonnull align 4 dereferenceable(92) %hot_left.i56, i64 92, i1 false), !dbg !40777, !noalias !39364
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_157.i34, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33) #31, !dbg !40778, !noalias !39358
  call void @llvm.lifetime.end.p0(ptr nonnull %_157.i34), !dbg !40779, !noalias !39364
  call void @llvm.lifetime.start.p0(ptr nonnull %_159.i33), !dbg !40780, !noalias !39364
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(92) %_159.i33, ptr noundef nonnull align 4 dereferenceable(92) %hot_right.i55, i64 92, i1 false), !dbg !40780, !noalias !39364
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_159.i33, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34) #31, !dbg !40781, !noalias !39358
  call void @llvm.lifetime.end.p0(ptr nonnull %_159.i33), !dbg !40782, !noalias !39364
  store i32 %main_cursor.sroa.0.0.i75.lcssa, ptr %_35, align 4, !dbg !40701, !alias.scope !39358, !noalias !39387
  store i32 %ring_cursor.sroa.0.0.i74.lcssa, ptr %545, align 4, !dbg !40703, !alias.scope !39358, !noalias !39387
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i53), !dbg !40783, !noalias !39364
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i54), !dbg !40784, !noalias !39364
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_right.i55), !dbg !40785, !noalias !39364
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i56), !dbg !40786, !noalias !39364
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockfEB2_.exit, !dbg !39351

bb6.i:                                            ; preds = %bb5.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !40787), !dbg !40790
  tail call void @llvm.experimental.noalias.scope.decl(metadata !40791), !dbg !40790
  tail call void @llvm.experimental.noalias.scope.decl(metadata !40793), !dbg !40790
  tail call void @llvm.experimental.noalias.scope.decl(metadata !40795), !dbg !40790
  tail call void @llvm.experimental.noalias.scope.decl(metadata !40797), !dbg !40790
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_left.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #31, !dbg !40799
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_right.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #31, !dbg !40803
  %667 = getelementptr inbounds nuw i8, ptr %self, i64 776, !dbg !40805
  %668 = load i8, ptr %667, align 4, !dbg !40805, !range !17, !alias.scope !40787, !noalias !40809, !noundef !12
  %669 = getelementptr inbounds nuw i8, ptr %self, i64 777, !dbg !40812
  %670 = load i8, ptr %669, align 1, !dbg !40812, !range !17, !alias.scope !40787, !noalias !40809, !noundef !12
  %671 = getelementptr inbounds nuw i8, ptr %self, i64 544, !dbg !40814
  %ring.i = load i64, ptr %671, align 8, !dbg !40814, !alias.scope !40791, !noalias !40816, !noundef !12
  %672 = getelementptr inbounds nuw i8, ptr %self, i64 552, !dbg !40817
  %main.i = load i64, ptr %672, align 8, !dbg !40817, !alias.scope !40791, !noalias !40816, !noundef !12
  %_35.i = load i32, ptr %_35, align 4, !dbg !40819, !alias.scope !40797, !noalias !40821, !noundef !12
  %673 = getelementptr inbounds nuw i8, ptr %self, i64 564, !dbg !40822
  %_36.i = load i32, ptr %673, align 4, !dbg !40822, !alias.scope !40797, !noalias !40821, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i), !dbg !40824, !noalias !40826
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i, i8 0, i64 1024, i1 false), !noalias !40826
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i), !dbg !40827, !noalias !40826
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i, i8 0, i64 1024, i1 false), !noalias !40826
  %_31.i = zext nneg i8 %668 to i32, !dbg !40805
  %.none.i = sub nsw i32 0, %_31.i, !dbg !40829
  %_32.i = zext nneg i8 %670 to i32, !dbg !40812
  %all.sroa.0.0.i = sub nsw i32 0, %_32.i, !dbg !40812
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i), !dbg !40830, !noalias !40826
; call <true_peak_limiter::UniformHot<f32>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotfE3newB5_(ptr noalias noundef align 8 captures(none) dereferenceable(80) %uniform_left.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33, i64 %ring.i, i64 %main.i) #31, !dbg !40832
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_right.i), !dbg !40833, !noalias !40826
  %_32.val4392 = load i64, ptr %671, align 8, !dbg !40835, !noundef !12
  %_32.val4393 = load i64, ptr %672, align 8, !dbg !40835, !noundef !12
; call <true_peak_limiter::UniformHot<f32>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotfE3newB5_(ptr noalias noundef align 8 captures(none) dereferenceable(80) %uniform_right.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34, i64 %_32.val4392, i64 %_32.val4393) #31, !dbg !40835
  %_167.not.i9423 = icmp eq i64 %frames, 0, !dbg !40836
  br i1 %_167.not.i9423, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit, label %bb50.i.lr.ph, !dbg !40836

bb50.i.lr.ph:                                     ; preds = %bb6.i
  %674 = zext i32 %_36.i to i64, !dbg !40822
  %675 = zext i32 %_35.i to i64, !dbg !40819
  %d9.i.i5028 = lshr i64 %frames, 5, !dbg !40846
  %r2.i.i5029 = and i64 %frames, 31, !dbg !40852
  %_19.not.i.i5030 = icmp ne i64 %r2.i.i5029, 0, !dbg !40853
  %676 = zext i1 %_19.not.i.i5030 to i64, !dbg !40853
  %yield_count.sroa.0.0.i.i5031 = add nuw nsw i64 %d9.i.i5028, %676, !dbg !40853
  %history.i44.i.sroa.7.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 4
  %history.i44.i.sroa.10.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 8
  %history.i44.i.sroa.13.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 12
  %history.i44.i.sroa.16.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 16
  %history.i44.i.sroa.19.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 20
  %history.i44.i.sroa.22.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 24
  %history.i44.i.sroa.26.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 28
  %history.i44.i.sroa.29.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 32
  %history.i44.i.sroa.32.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 36
  %history.i44.i.sroa.35.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 40
  %history.i44.i.sroa.38.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 44
  %677 = getelementptr inbounds nuw i8, ptr %self, i64 588
  %678 = getelementptr inbounds nuw i8, ptr %self, i64 592
  %679 = getelementptr inbounds nuw i8, ptr %self, i64 596
  %row1.i.i.i77.i = getelementptr inbounds nuw i8, ptr %self, i64 600
  %680 = getelementptr inbounds nuw i8, ptr %self, i64 604
  %681 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %682 = getelementptr inbounds nuw i8, ptr %self, i64 612
  %row3.i.i.i91.i = getelementptr inbounds nuw i8, ptr %self, i64 616
  %683 = getelementptr inbounds nuw i8, ptr %self, i64 620
  %684 = getelementptr inbounds nuw i8, ptr %self, i64 624
  %685 = getelementptr inbounds nuw i8, ptr %self, i64 628
  %row5.i.i.i105.i = getelementptr inbounds nuw i8, ptr %self, i64 632
  %686 = getelementptr inbounds nuw i8, ptr %self, i64 636
  %687 = getelementptr inbounds nuw i8, ptr %self, i64 640
  %688 = getelementptr inbounds nuw i8, ptr %self, i64 644
  %row7.i.i.i119.i = getelementptr inbounds nuw i8, ptr %self, i64 648
  %689 = getelementptr inbounds nuw i8, ptr %self, i64 652
  %690 = getelementptr inbounds nuw i8, ptr %self, i64 656
  %691 = getelementptr inbounds nuw i8, ptr %self, i64 660
  %row9.i.i.i133.i = getelementptr inbounds nuw i8, ptr %self, i64 664
  %692 = getelementptr inbounds nuw i8, ptr %self, i64 668
  %693 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %694 = getelementptr inbounds nuw i8, ptr %self, i64 676
  %row11.i.i.i147.i = getelementptr inbounds nuw i8, ptr %self, i64 680
  %695 = getelementptr inbounds nuw i8, ptr %self, i64 684
  %696 = getelementptr inbounds nuw i8, ptr %self, i64 688
  %697 = getelementptr inbounds nuw i8, ptr %self, i64 692
  %row13.i.i.i161.i = getelementptr inbounds nuw i8, ptr %self, i64 696
  %698 = getelementptr inbounds nuw i8, ptr %self, i64 700
  %699 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %700 = getelementptr inbounds nuw i8, ptr %self, i64 708
  %row15.i.i.i175.i = getelementptr inbounds nuw i8, ptr %self, i64 712
  %701 = getelementptr inbounds nuw i8, ptr %self, i64 716
  %702 = getelementptr inbounds nuw i8, ptr %self, i64 720
  %703 = getelementptr inbounds nuw i8, ptr %self, i64 724
  %row17.i.i.i189.i = getelementptr inbounds nuw i8, ptr %self, i64 728
  %704 = getelementptr inbounds nuw i8, ptr %self, i64 732
  %705 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %706 = getelementptr inbounds nuw i8, ptr %self, i64 740
  %row19.i.i.i203.i = getelementptr inbounds nuw i8, ptr %self, i64 744
  %707 = getelementptr inbounds nuw i8, ptr %self, i64 748
  %708 = getelementptr inbounds nuw i8, ptr %self, i64 752
  %709 = getelementptr inbounds nuw i8, ptr %self, i64 756
  %row21.i.i.i217.i = getelementptr inbounds nuw i8, ptr %self, i64 760
  %710 = getelementptr inbounds nuw i8, ptr %self, i64 764
  %711 = getelementptr inbounds nuw i8, ptr %self, i64 768
  %712 = getelementptr inbounds nuw i8, ptr %self, i64 772
  %history.i.i.sroa.7.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 4
  %history.i.i.sroa.10.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 8
  %history.i.i.sroa.13.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 12
  %history.i.i.sroa.16.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 16
  %history.i.i.sroa.19.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 20
  %history.i.i.sroa.22.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 24
  %history.i.i.sroa.26.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 28
  %history.i.i.sroa.29.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 32
  %history.i.i.sroa.32.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 36
  %history.i.i.sroa.35.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 40
  %history.i.i.sroa.38.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 44
  %713 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 48
  %_70.i.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 56
  %_70.i.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 64
  %714 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 48
  %_71.i.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 56
  %_71.i.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 64
  %_114.i = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 48
  %_115.i = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 64
  %_119.i = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 48
  %_120.i = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 64
  %_9.i4046 = add nsw i32 %_31.i, -1
  %715 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 8
  %_21.i264.i = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 72
  %_22.i265.i = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 76
  %716 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 16
  %717 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 24
  %718 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 84
  %719 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 88
  %720 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 80
  %721 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 40
  %722 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 32
  %_9.i4026 = add nsw i32 %_32.i, -1
  %723 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 8
  %_21.i.i = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 72
  %_22.i.i = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 76
  %724 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 16
  %725 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 24
  %726 = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 84
  %727 = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 88
  %728 = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 80
  %729 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 40
  %730 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 32
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
  %_21.i264.i.promoted14371 = load float, ptr %_21.i264.i, align 4
  %_22.i.i.promoted = load i32, ptr %_22.i.i, align 4
  %_21.i.i.promoted14410 = load float, ptr %_21.i.i, align 4
  %.promoted14430 = load float, ptr %718, align 4
  %.promoted14433 = load float, ptr %720, align 4
  %.promoted14436 = load float, ptr %726, align 4
  %.promoted14439 = load float, ptr %728, align 4
  br label %bb50.i, !dbg !40836

bb19.i.bb15.i.loopexit_crit_edge:                 ; preds = %bb32.i
  store float %_0.i3414.lcssa1198914155, ptr %718, align 4
  store float %_0.i3788.lcssa1200514173, ptr %720, align 4
  store float %_0.i3410.lcssa1202914191, ptr %726, align 4
  store float %_0.i3784.lcssa1203014209, ptr %728, align 4
  br label %bb15.i.loopexit, !dbg !40854

bb15.i.loopexit:                                  ; preds = %bb19.i.bb15.i.loopexit_crit_edge, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i
  %_0.i3784.lcssa1203014209.lcssa14440 = phi float [ %_0.i3784.lcssa1203014209, %bb19.i.bb15.i.loopexit_crit_edge ], [ %_0.i3784.lcssa1203014209.lcssa14441, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i ]
  %_0.i3410.lcssa1202914191.lcssa14437 = phi float [ %_0.i3410.lcssa1202914191, %bb19.i.bb15.i.loopexit_crit_edge ], [ %_0.i3410.lcssa1202914191.lcssa14438, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i ]
  %_0.i3788.lcssa1200514173.lcssa14434 = phi float [ %_0.i3788.lcssa1200514173, %bb19.i.bb15.i.loopexit_crit_edge ], [ %_0.i3788.lcssa1200514173.lcssa14435, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i ]
  %_0.i3414.lcssa1198914155.lcssa14431 = phi float [ %_0.i3414.lcssa1198914155, %bb19.i.bb15.i.loopexit_crit_edge ], [ %_0.i3414.lcssa1198914155.lcssa14432, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i ]
  %running.sroa.0.0.i1725.lcssa1202414243.lcssa14411 = phi float [ %running.sroa.0.0.i1725.lcssa1202414243, %bb19.i.bb15.i.loopexit_crit_edge ], [ %running.sroa.0.0.i1725.lcssa1202414243.lcssa14412, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i ]
  %storemerge.i1730.lcssa92499385.lcssa14391 = phi i32 [ %storemerge.i1730.lcssa92499385, %bb19.i.bb15.i.loopexit_crit_edge ], [ %storemerge.i1730.lcssa92499385.lcssa14392, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i ]
  %running.sroa.0.0.i1756.lcssa1197014226.lcssa14372 = phi float [ %running.sroa.0.0.i1756.lcssa1197014226, %bb19.i.bb15.i.loopexit_crit_edge ], [ %running.sroa.0.0.i1756.lcssa1197014226.lcssa14373, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i ]
  %storemerge.i1761.lcssa91379346.lcssa14352 = phi i32 [ %storemerge.i1761.lcssa91379346, %bb19.i.bb15.i.loopexit_crit_edge ], [ %storemerge.i1761.lcssa91379346.lcssa14353, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i ]
  %ring_cursor.sroa.0.1.i.lcssa = phi i64 [ %ring_cursor.sroa.0.2.i, %bb19.i.bb15.i.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i9424, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i ], !dbg !40860
  %main_cursor.sroa.0.1.i.lcssa = phi i64 [ %main_cursor.sroa.0.2.i, %bb19.i.bb15.i.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i9425, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i ], !dbg !40861
  %_167.not.i = icmp eq i64 %732, 0, !dbg !40836
  %indvars.iv.next11808 = add i64 %indvars.iv11807, -32, !dbg !40836
  br i1 %_167.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit, label %bb50.i, !dbg !40836

bb50.i:                                           ; preds = %bb50.i.lr.ph, %bb15.i.loopexit
  %_0.i3784.lcssa1203014209.lcssa14441 = phi float [ %.promoted14439, %bb50.i.lr.ph ], [ %_0.i3784.lcssa1203014209.lcssa14440, %bb15.i.loopexit ]
  %_0.i3410.lcssa1202914191.lcssa14438 = phi float [ %.promoted14436, %bb50.i.lr.ph ], [ %_0.i3410.lcssa1202914191.lcssa14437, %bb15.i.loopexit ]
  %_0.i3788.lcssa1200514173.lcssa14435 = phi float [ %.promoted14433, %bb50.i.lr.ph ], [ %_0.i3788.lcssa1200514173.lcssa14434, %bb15.i.loopexit ]
  %_0.i3414.lcssa1198914155.lcssa14432 = phi float [ %.promoted14430, %bb50.i.lr.ph ], [ %_0.i3414.lcssa1198914155.lcssa14431, %bb15.i.loopexit ]
  %running.sroa.0.0.i1725.lcssa1202414243.lcssa14412 = phi float [ %_21.i.i.promoted14410, %bb50.i.lr.ph ], [ %running.sroa.0.0.i1725.lcssa1202414243.lcssa14411, %bb15.i.loopexit ]
  %storemerge.i1730.lcssa92499385.lcssa14392 = phi i32 [ %_22.i.i.promoted, %bb50.i.lr.ph ], [ %storemerge.i1730.lcssa92499385.lcssa14391, %bb15.i.loopexit ]
  %running.sroa.0.0.i1756.lcssa1197014226.lcssa14373 = phi float [ %_21.i264.i.promoted14371, %bb50.i.lr.ph ], [ %running.sroa.0.0.i1756.lcssa1197014226.lcssa14372, %bb15.i.loopexit ]
  %storemerge.i1761.lcssa91379346.lcssa14353 = phi i32 [ %_22.i265.i.promoted, %bb50.i.lr.ph ], [ %storemerge.i1761.lcssa91379346.lcssa14352, %bb15.i.loopexit ]
  %history.i.i.sroa.38.0.lcssa14351 = phi float [ %history.i.i.sroa.38.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.38.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.35.0.lcssa14350 = phi float [ %history.i.i.sroa.35.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.35.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.32.0.lcssa14349 = phi float [ %history.i.i.sroa.32.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.32.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.29.0.lcssa14348 = phi float [ %history.i.i.sroa.29.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.29.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.26.0.lcssa14347 = phi float [ %history.i.i.sroa.26.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.26.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.22.0.lcssa14346 = phi float [ %history.i.i.sroa.22.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.22.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.19.0.lcssa14345 = phi float [ %history.i.i.sroa.19.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.19.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.16.0.lcssa14344 = phi float [ %history.i.i.sroa.16.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.16.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.13.0.lcssa14343 = phi float [ %history.i.i.sroa.13.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.13.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.10.0.lcssa14342 = phi float [ %history.i.i.sroa.10.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.10.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.7.0.lcssa14324 = phi float [ %history.i.i.sroa.7.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.7.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.0.0.lcssa14306 = phi float [ %hot_right.i.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i44.i.sroa.38.0.lcssa14305 = phi float [ %history.i44.i.sroa.38.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i44.i.sroa.38.0.lcssa, %bb15.i.loopexit ]
  %history.i44.i.sroa.35.0.lcssa14304 = phi float [ %history.i44.i.sroa.35.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i44.i.sroa.35.0.lcssa, %bb15.i.loopexit ]
  %history.i44.i.sroa.32.0.lcssa14303 = phi float [ %history.i44.i.sroa.32.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i44.i.sroa.32.0.lcssa, %bb15.i.loopexit ]
  %history.i44.i.sroa.29.0.lcssa14302 = phi float [ %history.i44.i.sroa.29.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i44.i.sroa.29.0.lcssa, %bb15.i.loopexit ]
  %history.i44.i.sroa.26.0.lcssa14301 = phi float [ %history.i44.i.sroa.26.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i44.i.sroa.26.0.lcssa, %bb15.i.loopexit ]
  %history.i44.i.sroa.22.0.lcssa14300 = phi float [ %history.i44.i.sroa.22.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i44.i.sroa.22.0.lcssa, %bb15.i.loopexit ]
  %history.i44.i.sroa.19.0.lcssa14299 = phi float [ %history.i44.i.sroa.19.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i44.i.sroa.19.0.lcssa, %bb15.i.loopexit ]
  %history.i44.i.sroa.16.0.lcssa14298 = phi float [ %history.i44.i.sroa.16.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i44.i.sroa.16.0.lcssa, %bb15.i.loopexit ]
  %history.i44.i.sroa.13.0.lcssa14297 = phi float [ %history.i44.i.sroa.13.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i44.i.sroa.13.0.lcssa, %bb15.i.loopexit ]
  %history.i44.i.sroa.10.0.lcssa14296 = phi float [ %history.i44.i.sroa.10.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i44.i.sroa.10.0.lcssa, %bb15.i.loopexit ]
  %history.i44.i.sroa.7.0.lcssa14278 = phi float [ %history.i44.i.sroa.7.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i44.i.sroa.7.0.lcssa, %bb15.i.loopexit ]
  %history.i44.i.sroa.0.0.lcssa14260 = phi float [ %hot_left.i.promoted, %bb50.i.lr.ph ], [ %history.i44.i.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %indvars.iv11807 = phi i64 [ %frames, %bb50.i.lr.ph ], [ %indvars.iv.next11808, %bb15.i.loopexit ]
  %iter2.sroa.0.0.i9427 = phi i64 [ %yield_count.sroa.0.0.i.i5031, %bb50.i.lr.ph ], [ %732, %bb15.i.loopexit ]
  %iter1.sroa.0.0.i9426 = phi i64 [ 0, %bb50.i.lr.ph ], [ %731, %bb15.i.loopexit ]
  %main_cursor.sroa.0.0.i9425 = phi i64 [ %675, %bb50.i.lr.ph ], [ %main_cursor.sroa.0.1.i.lcssa, %bb15.i.loopexit ]
  %ring_cursor.sroa.0.0.i9424 = phi i64 [ %674, %bb50.i.lr.ph ], [ %ring_cursor.sroa.0.1.i.lcssa, %bb15.i.loopexit ]
  %umin11827 = call i64 @llvm.umin.i64(i64 %indvars.iv11807, i64 32), !dbg !40862
  %umax11813 = call i64 @llvm.umax.i64(i64 %umin11827, i64 1), !dbg !40862
  %731 = add i64 %iter1.sroa.0.0.i9426, 32, !dbg !40862
  %732 = add i64 %iter2.sroa.0.0.i9427, -1, !dbg !40866
  %_46.i = sub i64 %frames, %iter1.sroa.0.0.i9426, !dbg !40867
  %..i5032 = tail call noundef i64 @llvm.umin.i64(i64 %_46.i, i64 32), !dbg !40868
  %_52.i = add i64 %..i5032, %iter1.sroa.0.0.i9426, !dbg !40872
  %_179.i = icmp ult i64 %_52.i, %iter1.sroa.0.0.i9426, !dbg !40873
  %_173.not.i = icmp ugt i64 %_52.i, %left_io.1
  %or.cond.i = or i1 %_179.i, %_173.not.i, !dbg !40873
  br i1 %or.cond.i, label %bb54.i, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5058, !dbg !40873, !prof !165

bb54.i:                                           ; preds = %bb50.i
  store float %history.i44.i.sroa.0.0.lcssa14260, ptr %hot_left.i, align 4, !dbg !40880
  store float %history.i44.i.sroa.7.0.lcssa14278, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !40880
  store float %history.i.i.sroa.0.0.lcssa14306, ptr %hot_right.i, align 4, !dbg !40882
  store float %history.i.i.sroa.7.0.lcssa14324, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !40882
  store i32 %storemerge.i1761.lcssa91379346.lcssa14353, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014226.lcssa14373, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1730.lcssa92499385.lcssa14392, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1725.lcssa1202414243.lcssa14412, ptr %_21.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %iter1.sroa.0.0.i9426, i64 noundef %_52.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e7f134ea71d3d762bf72c0ef5353d5ff) #30, !dbg !40884, !noalias !40797
  unreachable, !dbg !40884

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5058: ; preds = %bb50.i
  %_182.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %iter1.sroa.0.0.i9426, !dbg !40885
  tail call void @llvm.experimental.noalias.scope.decl(metadata !40889), !dbg !40892
  %_2.i50619054.not = icmp eq i64 %frames, %iter1.sroa.0.0.i9426, !dbg !40893
  br i1 %_2.i50619054.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit239.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615.lr.ph, !dbg !40893

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615.lr.ph: ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5058
  %_11.i.i.i65.i = load float, ptr %_31, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_14.i.i.i68.i = load float, ptr %677, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_17.i.i.i71.i = load float, ptr %678, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_20.i.i.i74.i = load float, ptr %679, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_25.i.i.i79.i = load float, ptr %row1.i.i.i77.i, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_28.i.i.i82.i = load float, ptr %680, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_31.i.i.i85.i = load float, ptr %681, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_34.i.i.i88.i = load float, ptr %682, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_39.i.i.i93.i = load float, ptr %row3.i.i.i91.i, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_42.i.i.i96.i = load float, ptr %683, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_45.i.i.i99.i = load float, ptr %684, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_48.i.i.i102.i = load float, ptr %685, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_53.i.i.i107.i = load float, ptr %row5.i.i.i105.i, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_56.i.i.i110.i = load float, ptr %686, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_59.i.i.i113.i = load float, ptr %687, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_62.i.i.i116.i = load float, ptr %688, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_67.i.i.i121.i = load float, ptr %row7.i.i.i119.i, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_70.i.i.i124.i = load float, ptr %689, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_73.i.i.i127.i = load float, ptr %690, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_76.i.i.i130.i = load float, ptr %691, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_81.i.i.i135.i = load float, ptr %row9.i.i.i133.i, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_84.i.i.i138.i = load float, ptr %692, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_87.i.i.i141.i = load float, ptr %693, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_90.i.i.i144.i = load float, ptr %694, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_95.i.i.i149.i = load float, ptr %row11.i.i.i147.i, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_98.i.i.i152.i = load float, ptr %695, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_101.i.i.i155.i = load float, ptr %696, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_104.i.i.i158.i = load float, ptr %697, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_109.i.i.i163.i = load float, ptr %row13.i.i.i161.i, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_112.i.i.i166.i = load float, ptr %698, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_115.i.i.i169.i = load float, ptr %699, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_118.i.i.i172.i = load float, ptr %700, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_123.i.i.i177.i = load float, ptr %row15.i.i.i175.i, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_126.i.i.i180.i = load float, ptr %701, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_129.i.i.i183.i = load float, ptr %702, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_132.i.i.i186.i = load float, ptr %703, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_137.i.i.i191.i = load float, ptr %row17.i.i.i189.i, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_140.i.i.i194.i = load float, ptr %704, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_143.i.i.i197.i = load float, ptr %705, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_146.i.i.i200.i = load float, ptr %706, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_151.i.i.i205.i = load float, ptr %row19.i.i.i203.i, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_154.i.i.i208.i = load float, ptr %707, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_157.i.i.i211.i = load float, ptr %708, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_160.i.i.i214.i = load float, ptr %709, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_165.i.i.i219.i = load float, ptr %row21.i.i.i217.i, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_168.i.i.i222.i = load float, ptr %710, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_171.i.i.i225.i = load float, ptr %711, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  %_174.i.i.i228.i = load float, ptr %712, align 4, !alias.scope !40896, !noalias !40901, !noundef !12
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615, !dbg !40893

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615
  %iter.i40.i.sroa.16.09066 = phi i64 [ 0, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615.lr.ph ], [ %738, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615 ]
  %history.i44.i.sroa.35.09065 = phi float [ %history.i44.i.sroa.35.0.lcssa14304, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615.lr.ph ], [ %history.i44.i.sroa.32.09064, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615 ]
  %history.i44.i.sroa.32.09064 = phi float [ %history.i44.i.sroa.32.0.lcssa14303, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615.lr.ph ], [ %history.i44.i.sroa.29.09063, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615 ]
  %history.i44.i.sroa.29.09063 = phi float [ %history.i44.i.sroa.29.0.lcssa14302, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615.lr.ph ], [ %history.i44.i.sroa.26.09062, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615 ]
  %history.i44.i.sroa.26.09062 = phi float [ %history.i44.i.sroa.26.0.lcssa14301, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615.lr.ph ], [ %history.i44.i.sroa.22.09061, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615 ]
  %history.i44.i.sroa.22.09061 = phi float [ %history.i44.i.sroa.22.0.lcssa14300, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615.lr.ph ], [ %history.i44.i.sroa.19.09060, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615 ]
  %history.i44.i.sroa.19.09060 = phi float [ %history.i44.i.sroa.19.0.lcssa14299, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615.lr.ph ], [ %history.i44.i.sroa.16.09059, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615 ]
  %history.i44.i.sroa.16.09059 = phi float [ %history.i44.i.sroa.16.0.lcssa14298, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615.lr.ph ], [ %history.i44.i.sroa.13.09058, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615 ]
  %history.i44.i.sroa.13.09058 = phi float [ %history.i44.i.sroa.13.0.lcssa14297, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615.lr.ph ], [ %history.i44.i.sroa.10.09057, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615 ]
  %history.i44.i.sroa.10.09057 = phi float [ %history.i44.i.sroa.10.0.lcssa14296, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615.lr.ph ], [ %history.i44.i.sroa.7.09056, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615 ]
  %history.i44.i.sroa.7.09056 = phi float [ %history.i44.i.sroa.7.0.lcssa14278, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615.lr.ph ], [ %history.i44.i.sroa.0.09055, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615 ]
  %history.i44.i.sroa.0.09055 = phi float [ %history.i44.i.sroa.0.0.lcssa14260, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615.lr.ph ], [ %_0.i3613, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615 ]
  %data.i.i5068 = getelementptr inbounds nuw float, ptr %_182.i, i64 %iter.i40.i.sroa.16.09066, !dbg !40908
  %_0.i3613 = load float, ptr %data.i.i5068, align 4, !dbg !40911, !alias.scope !40913, !noalias !40916, !noundef !12
  %733 = tail call noundef float @llvm.fabs.f32(float %history.i44.i.sroa.19.09060), !dbg !40917
  %_0.i3374 = fmul float %_0.i3613, %_11.i.i.i65.i, !dbg !40920
  %_0.i2934 = fadd float %_0.i3374, 0.000000e+00, !dbg !40923
  %_0.i3373 = fmul float %_0.i3613, %_14.i.i.i68.i, !dbg !40925
  %_0.i2933 = fadd float %_0.i3373, 0.000000e+00, !dbg !40927
  %_0.i3372 = fmul float %_0.i3613, %_17.i.i.i71.i, !dbg !40929
  %_0.i2932 = fadd float %_0.i3372, 0.000000e+00, !dbg !40931
  %_0.i3371 = fmul float %_0.i3613, %_20.i.i.i74.i, !dbg !40933
  %_0.i2931 = fadd float %_0.i3371, 0.000000e+00, !dbg !40935
  %_0.i3370 = fmul float %history.i44.i.sroa.0.09055, %_25.i.i.i79.i, !dbg !40937
  %_0.i2930 = fadd float %_0.i2934, %_0.i3370, !dbg !40939
  %_0.i3369 = fmul float %history.i44.i.sroa.0.09055, %_28.i.i.i82.i, !dbg !40941
  %_0.i2929 = fadd float %_0.i2933, %_0.i3369, !dbg !40943
  %_0.i3368 = fmul float %history.i44.i.sroa.0.09055, %_31.i.i.i85.i, !dbg !40945
  %_0.i2928 = fadd float %_0.i2932, %_0.i3368, !dbg !40947
  %_0.i3367 = fmul float %history.i44.i.sroa.0.09055, %_34.i.i.i88.i, !dbg !40949
  %_0.i2927 = fadd float %_0.i2931, %_0.i3367, !dbg !40951
  %_0.i3366 = fmul float %history.i44.i.sroa.7.09056, %_39.i.i.i93.i, !dbg !40953
  %_0.i2926 = fadd float %_0.i2930, %_0.i3366, !dbg !40955
  %_0.i3365 = fmul float %history.i44.i.sroa.7.09056, %_42.i.i.i96.i, !dbg !40957
  %_0.i2925 = fadd float %_0.i2929, %_0.i3365, !dbg !40959
  %_0.i3364 = fmul float %history.i44.i.sroa.7.09056, %_45.i.i.i99.i, !dbg !40961
  %_0.i2924 = fadd float %_0.i2928, %_0.i3364, !dbg !40963
  %_0.i3363 = fmul float %history.i44.i.sroa.7.09056, %_48.i.i.i102.i, !dbg !40965
  %_0.i2923 = fadd float %_0.i2927, %_0.i3363, !dbg !40967
  %_0.i3362 = fmul float %history.i44.i.sroa.10.09057, %_53.i.i.i107.i, !dbg !40969
  %_0.i2922 = fadd float %_0.i2926, %_0.i3362, !dbg !40971
  %_0.i3361 = fmul float %history.i44.i.sroa.10.09057, %_56.i.i.i110.i, !dbg !40973
  %_0.i2921 = fadd float %_0.i2925, %_0.i3361, !dbg !40975
  %_0.i3360 = fmul float %history.i44.i.sroa.10.09057, %_59.i.i.i113.i, !dbg !40977
  %_0.i2920 = fadd float %_0.i2924, %_0.i3360, !dbg !40979
  %_0.i3359 = fmul float %history.i44.i.sroa.10.09057, %_62.i.i.i116.i, !dbg !40981
  %_0.i2919 = fadd float %_0.i2923, %_0.i3359, !dbg !40983
  %_0.i3358 = fmul float %history.i44.i.sroa.13.09058, %_67.i.i.i121.i, !dbg !40985
  %_0.i2918 = fadd float %_0.i2922, %_0.i3358, !dbg !40987
  %_0.i3357 = fmul float %history.i44.i.sroa.13.09058, %_70.i.i.i124.i, !dbg !40989
  %_0.i2917 = fadd float %_0.i2921, %_0.i3357, !dbg !40991
  %_0.i3356 = fmul float %history.i44.i.sroa.13.09058, %_73.i.i.i127.i, !dbg !40993
  %_0.i2916 = fadd float %_0.i2920, %_0.i3356, !dbg !40995
  %_0.i3355 = fmul float %history.i44.i.sroa.13.09058, %_76.i.i.i130.i, !dbg !40997
  %_0.i2915 = fadd float %_0.i2919, %_0.i3355, !dbg !40999
  %_0.i3354 = fmul float %history.i44.i.sroa.16.09059, %_81.i.i.i135.i, !dbg !41001
  %_0.i2914 = fadd float %_0.i2918, %_0.i3354, !dbg !41003
  %_0.i3353 = fmul float %history.i44.i.sroa.16.09059, %_84.i.i.i138.i, !dbg !41005
  %_0.i2913 = fadd float %_0.i2917, %_0.i3353, !dbg !41007
  %_0.i3352 = fmul float %history.i44.i.sroa.16.09059, %_87.i.i.i141.i, !dbg !41009
  %_0.i2912 = fadd float %_0.i2916, %_0.i3352, !dbg !41011
  %_0.i3351 = fmul float %history.i44.i.sroa.16.09059, %_90.i.i.i144.i, !dbg !41013
  %_0.i2911 = fadd float %_0.i2915, %_0.i3351, !dbg !41015
  %_0.i3350 = fmul float %history.i44.i.sroa.19.09060, %_95.i.i.i149.i, !dbg !41017
  %_0.i2910 = fadd float %_0.i2914, %_0.i3350, !dbg !41019
  %_0.i3349 = fmul float %history.i44.i.sroa.19.09060, %_98.i.i.i152.i, !dbg !41021
  %_0.i2909 = fadd float %_0.i2913, %_0.i3349, !dbg !41023
  %_0.i3348 = fmul float %history.i44.i.sroa.19.09060, %_101.i.i.i155.i, !dbg !41025
  %_0.i2908 = fadd float %_0.i2912, %_0.i3348, !dbg !41027
  %_0.i3347 = fmul float %history.i44.i.sroa.19.09060, %_104.i.i.i158.i, !dbg !41029
  %_0.i2907 = fadd float %_0.i2911, %_0.i3347, !dbg !41031
  %_0.i3346 = fmul float %history.i44.i.sroa.22.09061, %_109.i.i.i163.i, !dbg !41033
  %_0.i2906 = fadd float %_0.i2910, %_0.i3346, !dbg !41035
  %_0.i3345 = fmul float %history.i44.i.sroa.22.09061, %_112.i.i.i166.i, !dbg !41037
  %_0.i2905 = fadd float %_0.i2909, %_0.i3345, !dbg !41039
  %_0.i3344 = fmul float %history.i44.i.sroa.22.09061, %_115.i.i.i169.i, !dbg !41041
  %_0.i2904 = fadd float %_0.i2908, %_0.i3344, !dbg !41043
  %_0.i3343 = fmul float %history.i44.i.sroa.22.09061, %_118.i.i.i172.i, !dbg !41045
  %_0.i2903 = fadd float %_0.i2907, %_0.i3343, !dbg !41047
  %_0.i3342 = fmul float %history.i44.i.sroa.26.09062, %_123.i.i.i177.i, !dbg !41049
  %_0.i2902 = fadd float %_0.i2906, %_0.i3342, !dbg !41051
  %_0.i3341 = fmul float %history.i44.i.sroa.26.09062, %_126.i.i.i180.i, !dbg !41053
  %_0.i2901 = fadd float %_0.i2905, %_0.i3341, !dbg !41055
  %_0.i3340 = fmul float %history.i44.i.sroa.26.09062, %_129.i.i.i183.i, !dbg !41057
  %_0.i2900 = fadd float %_0.i2904, %_0.i3340, !dbg !41059
  %_0.i3339 = fmul float %history.i44.i.sroa.26.09062, %_132.i.i.i186.i, !dbg !41061
  %_0.i2899 = fadd float %_0.i2903, %_0.i3339, !dbg !41063
  %_0.i3338 = fmul float %history.i44.i.sroa.29.09063, %_137.i.i.i191.i, !dbg !41065
  %_0.i2898 = fadd float %_0.i2902, %_0.i3338, !dbg !41067
  %_0.i3337 = fmul float %history.i44.i.sroa.29.09063, %_140.i.i.i194.i, !dbg !41069
  %_0.i2897 = fadd float %_0.i2901, %_0.i3337, !dbg !41071
  %_0.i3336 = fmul float %history.i44.i.sroa.29.09063, %_143.i.i.i197.i, !dbg !41073
  %_0.i2896 = fadd float %_0.i2900, %_0.i3336, !dbg !41075
  %_0.i3335 = fmul float %history.i44.i.sroa.29.09063, %_146.i.i.i200.i, !dbg !41077
  %_0.i2895 = fadd float %_0.i2899, %_0.i3335, !dbg !41079
  %_0.i3334 = fmul float %history.i44.i.sroa.32.09064, %_151.i.i.i205.i, !dbg !41081
  %_0.i2894 = fadd float %_0.i2898, %_0.i3334, !dbg !41083
  %_0.i3333 = fmul float %history.i44.i.sroa.32.09064, %_154.i.i.i208.i, !dbg !41085
  %_0.i2893 = fadd float %_0.i2897, %_0.i3333, !dbg !41087
  %_0.i3332 = fmul float %history.i44.i.sroa.32.09064, %_157.i.i.i211.i, !dbg !41089
  %_0.i2892 = fadd float %_0.i2896, %_0.i3332, !dbg !41091
  %_0.i3331 = fmul float %history.i44.i.sroa.32.09064, %_160.i.i.i214.i, !dbg !41093
  %_0.i2891 = fadd float %_0.i2895, %_0.i3331, !dbg !41095
  %_0.i3330 = fmul float %history.i44.i.sroa.35.09065, %_165.i.i.i219.i, !dbg !41097
  %_0.i2890 = fadd float %_0.i2894, %_0.i3330, !dbg !41099
  %_0.i3329 = fmul float %history.i44.i.sroa.35.09065, %_168.i.i.i222.i, !dbg !41101
  %_0.i2889 = fadd float %_0.i2893, %_0.i3329, !dbg !41103
  %_0.i3328 = fmul float %history.i44.i.sroa.35.09065, %_171.i.i.i225.i, !dbg !41105
  %_0.i2888 = fadd float %_0.i2892, %_0.i3328, !dbg !41107
  %_0.i3327 = fmul float %history.i44.i.sroa.35.09065, %_174.i.i.i228.i, !dbg !41109
  %_0.i2887 = fadd float %_0.i2891, %_0.i3327, !dbg !41111
  %734 = tail call noundef float @llvm.fabs.f32(float %_0.i2890), !dbg !41113
  %_3.i.i4271.inv = fcmp ogt float %733, %734, !dbg !41115
  %_4.i.i4278.v = select i1 %_3.i.i4271.inv, float %733, float %734, !dbg !41115
  %735 = tail call noundef float @llvm.fabs.f32(float %_0.i2889), !dbg !41113
  %_3.i.i4271.inv.1 = fcmp ogt float %_4.i.i4278.v, %735, !dbg !41115
  %_4.i.i4278.v.1 = select i1 %_3.i.i4271.inv.1, float %_4.i.i4278.v, float %735, !dbg !41115
  %736 = tail call noundef float @llvm.fabs.f32(float %_0.i2888), !dbg !41113
  %_3.i.i4271.inv.2 = fcmp ogt float %_4.i.i4278.v.1, %736, !dbg !41115
  %_4.i.i4278.v.2 = select i1 %_3.i.i4271.inv.2, float %_4.i.i4278.v.1, float %736, !dbg !41115
  %737 = tail call noundef float @llvm.fabs.f32(float %_0.i2887), !dbg !41113
  %_3.i.i4271.inv.3 = fcmp ogt float %_4.i.i4278.v.2, %737, !dbg !41115
  %_4.i.i4278.v.3 = select i1 %_3.i.i4271.inv.3, float %_4.i.i4278.v.2, float %737, !dbg !41115
  %738 = add nuw nsw i64 %iter.i40.i.sroa.16.09066, 1, !dbg !41118
  %data.i4.i5072 = getelementptr inbounds nuw float, ptr %peaks_left.i, i64 %iter.i40.i.sroa.16.09066, !dbg !41119
  store float %_4.i.i4278.v.3, ptr %data.i4.i5072, align 4, !dbg !41122, !alias.scope !41124, !noalias !40916
  %exitcond11811.not = icmp eq i64 %738, %umax11813, !dbg !40893
  br i1 %exitcond11811.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit239.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615, !dbg !40893

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit239.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5058
  %history.i44.i.sroa.0.0.lcssa = phi float [ %history.i44.i.sroa.0.0.lcssa14260, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5058 ], [ %_0.i3613, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615 ], !dbg !40880
  %history.i44.i.sroa.7.0.lcssa = phi float [ %history.i44.i.sroa.7.0.lcssa14278, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5058 ], [ %history.i44.i.sroa.0.09055, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615 ], !dbg !40880
  %history.i44.i.sroa.10.0.lcssa = phi float [ %history.i44.i.sroa.10.0.lcssa14296, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5058 ], [ %history.i44.i.sroa.7.09056, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615 ], !dbg !40880
  %history.i44.i.sroa.13.0.lcssa = phi float [ %history.i44.i.sroa.13.0.lcssa14297, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5058 ], [ %history.i44.i.sroa.10.09057, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615 ], !dbg !40880
  %history.i44.i.sroa.16.0.lcssa = phi float [ %history.i44.i.sroa.16.0.lcssa14298, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5058 ], [ %history.i44.i.sroa.13.09058, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615 ], !dbg !40880
  %history.i44.i.sroa.19.0.lcssa = phi float [ %history.i44.i.sroa.19.0.lcssa14299, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5058 ], [ %history.i44.i.sroa.16.09059, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615 ], !dbg !40880
  %history.i44.i.sroa.22.0.lcssa = phi float [ %history.i44.i.sroa.22.0.lcssa14300, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5058 ], [ %history.i44.i.sroa.19.09060, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615 ], !dbg !40880
  %history.i44.i.sroa.26.0.lcssa = phi float [ %history.i44.i.sroa.26.0.lcssa14301, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5058 ], [ %history.i44.i.sroa.22.09061, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615 ], !dbg !40880
  %history.i44.i.sroa.29.0.lcssa = phi float [ %history.i44.i.sroa.29.0.lcssa14302, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5058 ], [ %history.i44.i.sroa.26.09062, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615 ], !dbg !40880
  %history.i44.i.sroa.32.0.lcssa = phi float [ %history.i44.i.sroa.32.0.lcssa14303, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5058 ], [ %history.i44.i.sroa.29.09063, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615 ], !dbg !40880
  %history.i44.i.sroa.35.0.lcssa = phi float [ %history.i44.i.sroa.35.0.lcssa14304, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5058 ], [ %history.i44.i.sroa.32.09064, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615 ], !dbg !40880
  %history.i44.i.sroa.38.0.lcssa = phi float [ %history.i44.i.sroa.38.0.lcssa14305, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5058 ], [ %history.i44.i.sroa.35.09065, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3615 ], !dbg !40880
  store float %history.i44.i.sroa.10.0.lcssa, ptr %history.i44.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !41127
  store float %history.i44.i.sroa.13.0.lcssa, ptr %history.i44.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !41127
  store float %history.i44.i.sroa.16.0.lcssa, ptr %history.i44.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !41127
  store float %history.i44.i.sroa.19.0.lcssa, ptr %history.i44.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !41127
  store float %history.i44.i.sroa.22.0.lcssa, ptr %history.i44.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !41127
  store float %history.i44.i.sroa.26.0.lcssa, ptr %history.i44.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !41127
  store float %history.i44.i.sroa.29.0.lcssa, ptr %history.i44.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !41127
  store float %history.i44.i.sroa.32.0.lcssa, ptr %history.i44.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !41127
  store float %history.i44.i.sroa.35.0.lcssa, ptr %history.i44.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !41127
  store float %history.i44.i.sroa.38.0.lcssa, ptr %history.i44.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !41127
  %_190.not.i = icmp ugt i64 %_52.i, %right_io.1, !dbg !41128
  br i1 %_190.not.i, label %bb60.i, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5114, !dbg !41128, !prof !1406

bb60.i:                                           ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit239.i
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40880
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !40880
  store float %history.i.i.sroa.0.0.lcssa14306, ptr %hot_right.i, align 4, !dbg !40882
  store float %history.i.i.sroa.7.0.lcssa14324, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !40882
  store i32 %storemerge.i1761.lcssa91379346.lcssa14353, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014226.lcssa14373, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1730.lcssa92499385.lcssa14392, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1725.lcssa1202414243.lcssa14412, ptr %_21.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %iter1.sroa.0.0.i9426, i64 noundef %_52.i, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_480b8302a9d65cc7541e43747df38ed7) #30, !dbg !41132, !noalias !40797
  unreachable, !dbg !41132

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5114: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit239.i
  %_197.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %iter1.sroa.0.0.i9426, !dbg !41133
  tail call void @llvm.experimental.noalias.scope.decl(metadata !41137), !dbg !41140
  br i1 %_2.i50619054.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610.lr.ph, !dbg !41141

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610.lr.ph: ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5114
  %_11.i.i.i.i = load float, ptr %_31, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_14.i.i.i.i = load float, ptr %677, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_17.i.i.i.i = load float, ptr %678, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_20.i.i.i.i = load float, ptr %679, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_25.i.i.i.i = load float, ptr %row1.i.i.i77.i, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_28.i.i.i.i = load float, ptr %680, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_31.i.i.i.i = load float, ptr %681, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_34.i.i.i.i = load float, ptr %682, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_39.i.i.i.i = load float, ptr %row3.i.i.i91.i, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_42.i.i.i.i = load float, ptr %683, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_45.i.i.i.i = load float, ptr %684, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_48.i.i.i.i = load float, ptr %685, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_53.i.i.i.i = load float, ptr %row5.i.i.i105.i, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_56.i.i.i.i = load float, ptr %686, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_59.i.i.i.i = load float, ptr %687, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_62.i.i.i.i = load float, ptr %688, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_67.i.i.i.i = load float, ptr %row7.i.i.i119.i, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_70.i.i.i.i = load float, ptr %689, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_73.i.i.i.i = load float, ptr %690, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_76.i.i.i.i = load float, ptr %691, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_81.i.i.i.i = load float, ptr %row9.i.i.i133.i, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_84.i.i.i.i = load float, ptr %692, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_87.i.i.i.i = load float, ptr %693, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_90.i.i.i.i = load float, ptr %694, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_95.i.i.i.i = load float, ptr %row11.i.i.i147.i, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_98.i.i.i.i = load float, ptr %695, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_101.i.i.i.i = load float, ptr %696, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_104.i.i.i.i = load float, ptr %697, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_109.i.i.i.i = load float, ptr %row13.i.i.i161.i, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_112.i.i.i.i = load float, ptr %698, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_115.i.i.i.i = load float, ptr %699, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_118.i.i.i.i = load float, ptr %700, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_123.i.i.i.i = load float, ptr %row15.i.i.i175.i, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_126.i.i.i.i = load float, ptr %701, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_129.i.i.i.i = load float, ptr %702, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_132.i.i.i.i = load float, ptr %703, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_137.i.i.i.i = load float, ptr %row17.i.i.i189.i, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_140.i.i.i.i = load float, ptr %704, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_143.i.i.i.i = load float, ptr %705, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_146.i.i.i.i = load float, ptr %706, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_151.i.i.i.i = load float, ptr %row19.i.i.i203.i, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_154.i.i.i.i = load float, ptr %707, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_157.i.i.i.i = load float, ptr %708, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_160.i.i.i.i = load float, ptr %709, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_165.i.i.i.i = load float, ptr %row21.i.i.i217.i, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_168.i.i.i.i = load float, ptr %710, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_171.i.i.i.i = load float, ptr %711, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  %_174.i.i.i.i = load float, ptr %712, align 4, !alias.scope !41144, !noalias !41149, !noundef !12
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610, !dbg !41141

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610
  %iter.i.i.sroa.16.09093 = phi i64 [ 0, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610.lr.ph ], [ %744, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610 ]
  %history.i.i.sroa.35.09092 = phi float [ %history.i.i.sroa.35.0.lcssa14350, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610.lr.ph ], [ %history.i.i.sroa.32.09091, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610 ]
  %history.i.i.sroa.32.09091 = phi float [ %history.i.i.sroa.32.0.lcssa14349, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610.lr.ph ], [ %history.i.i.sroa.29.09090, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610 ]
  %history.i.i.sroa.29.09090 = phi float [ %history.i.i.sroa.29.0.lcssa14348, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610.lr.ph ], [ %history.i.i.sroa.26.09089, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610 ]
  %history.i.i.sroa.26.09089 = phi float [ %history.i.i.sroa.26.0.lcssa14347, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610.lr.ph ], [ %history.i.i.sroa.22.09088, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610 ]
  %history.i.i.sroa.22.09088 = phi float [ %history.i.i.sroa.22.0.lcssa14346, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610.lr.ph ], [ %history.i.i.sroa.19.09087, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610 ]
  %history.i.i.sroa.19.09087 = phi float [ %history.i.i.sroa.19.0.lcssa14345, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610.lr.ph ], [ %history.i.i.sroa.16.09086, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610 ]
  %history.i.i.sroa.16.09086 = phi float [ %history.i.i.sroa.16.0.lcssa14344, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610.lr.ph ], [ %history.i.i.sroa.13.09085, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610 ]
  %history.i.i.sroa.13.09085 = phi float [ %history.i.i.sroa.13.0.lcssa14343, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610.lr.ph ], [ %history.i.i.sroa.10.09084, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610 ]
  %history.i.i.sroa.10.09084 = phi float [ %history.i.i.sroa.10.0.lcssa14342, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610.lr.ph ], [ %history.i.i.sroa.7.09083, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610 ]
  %history.i.i.sroa.7.09083 = phi float [ %history.i.i.sroa.7.0.lcssa14324, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610.lr.ph ], [ %history.i.i.sroa.0.09082, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610 ]
  %history.i.i.sroa.0.09082 = phi float [ %history.i.i.sroa.0.0.lcssa14306, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610.lr.ph ], [ %_0.i3608, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610 ]
  %data.i.i5124 = getelementptr inbounds nuw float, ptr %_197.i, i64 %iter.i.i.sroa.16.09093, !dbg !41156
  %_0.i3608 = load float, ptr %data.i.i5124, align 4, !dbg !41159, !alias.scope !41161, !noalias !41164, !noundef !12
  %739 = tail call noundef float @llvm.fabs.f32(float %history.i.i.sroa.19.09087), !dbg !41165
  %_0.i3326 = fmul float %_0.i3608, %_11.i.i.i.i, !dbg !41168
  %_0.i2886 = fadd float %_0.i3326, 0.000000e+00, !dbg !41171
  %_0.i3325 = fmul float %_0.i3608, %_14.i.i.i.i, !dbg !41173
  %_0.i2885 = fadd float %_0.i3325, 0.000000e+00, !dbg !41175
  %_0.i3324 = fmul float %_0.i3608, %_17.i.i.i.i, !dbg !41177
  %_0.i2884 = fadd float %_0.i3324, 0.000000e+00, !dbg !41179
  %_0.i3323 = fmul float %_0.i3608, %_20.i.i.i.i, !dbg !41181
  %_0.i2883 = fadd float %_0.i3323, 0.000000e+00, !dbg !41183
  %_0.i3322 = fmul float %history.i.i.sroa.0.09082, %_25.i.i.i.i, !dbg !41185
  %_0.i2882 = fadd float %_0.i2886, %_0.i3322, !dbg !41187
  %_0.i3321 = fmul float %history.i.i.sroa.0.09082, %_28.i.i.i.i, !dbg !41189
  %_0.i2881 = fadd float %_0.i2885, %_0.i3321, !dbg !41191
  %_0.i3320 = fmul float %history.i.i.sroa.0.09082, %_31.i.i.i.i, !dbg !41193
  %_0.i2880 = fadd float %_0.i2884, %_0.i3320, !dbg !41195
  %_0.i3319 = fmul float %history.i.i.sroa.0.09082, %_34.i.i.i.i, !dbg !41197
  %_0.i2879 = fadd float %_0.i2883, %_0.i3319, !dbg !41199
  %_0.i3318 = fmul float %history.i.i.sroa.7.09083, %_39.i.i.i.i, !dbg !41201
  %_0.i2878 = fadd float %_0.i2882, %_0.i3318, !dbg !41203
  %_0.i3317 = fmul float %history.i.i.sroa.7.09083, %_42.i.i.i.i, !dbg !41205
  %_0.i2877 = fadd float %_0.i2881, %_0.i3317, !dbg !41207
  %_0.i3316 = fmul float %history.i.i.sroa.7.09083, %_45.i.i.i.i, !dbg !41209
  %_0.i2876 = fadd float %_0.i2880, %_0.i3316, !dbg !41211
  %_0.i3315 = fmul float %history.i.i.sroa.7.09083, %_48.i.i.i.i, !dbg !41213
  %_0.i2875 = fadd float %_0.i2879, %_0.i3315, !dbg !41215
  %_0.i3314 = fmul float %history.i.i.sroa.10.09084, %_53.i.i.i.i, !dbg !41217
  %_0.i2874 = fadd float %_0.i2878, %_0.i3314, !dbg !41219
  %_0.i3313 = fmul float %history.i.i.sroa.10.09084, %_56.i.i.i.i, !dbg !41221
  %_0.i2873 = fadd float %_0.i2877, %_0.i3313, !dbg !41223
  %_0.i3312 = fmul float %history.i.i.sroa.10.09084, %_59.i.i.i.i, !dbg !41225
  %_0.i2872 = fadd float %_0.i2876, %_0.i3312, !dbg !41227
  %_0.i3311 = fmul float %history.i.i.sroa.10.09084, %_62.i.i.i.i, !dbg !41229
  %_0.i2871 = fadd float %_0.i2875, %_0.i3311, !dbg !41231
  %_0.i3310 = fmul float %history.i.i.sroa.13.09085, %_67.i.i.i.i, !dbg !41233
  %_0.i2870 = fadd float %_0.i2874, %_0.i3310, !dbg !41235
  %_0.i3309 = fmul float %history.i.i.sroa.13.09085, %_70.i.i.i.i, !dbg !41237
  %_0.i2869 = fadd float %_0.i2873, %_0.i3309, !dbg !41239
  %_0.i3308 = fmul float %history.i.i.sroa.13.09085, %_73.i.i.i.i, !dbg !41241
  %_0.i2868 = fadd float %_0.i2872, %_0.i3308, !dbg !41243
  %_0.i3307 = fmul float %history.i.i.sroa.13.09085, %_76.i.i.i.i, !dbg !41245
  %_0.i2867 = fadd float %_0.i2871, %_0.i3307, !dbg !41247
  %_0.i3306 = fmul float %history.i.i.sroa.16.09086, %_81.i.i.i.i, !dbg !41249
  %_0.i2866 = fadd float %_0.i2870, %_0.i3306, !dbg !41251
  %_0.i3305 = fmul float %history.i.i.sroa.16.09086, %_84.i.i.i.i, !dbg !41253
  %_0.i2865 = fadd float %_0.i2869, %_0.i3305, !dbg !41255
  %_0.i3304 = fmul float %history.i.i.sroa.16.09086, %_87.i.i.i.i, !dbg !41257
  %_0.i2864 = fadd float %_0.i2868, %_0.i3304, !dbg !41259
  %_0.i3303 = fmul float %history.i.i.sroa.16.09086, %_90.i.i.i.i, !dbg !41261
  %_0.i2863 = fadd float %_0.i2867, %_0.i3303, !dbg !41263
  %_0.i3302 = fmul float %history.i.i.sroa.19.09087, %_95.i.i.i.i, !dbg !41265
  %_0.i2862 = fadd float %_0.i2866, %_0.i3302, !dbg !41267
  %_0.i3301 = fmul float %history.i.i.sroa.19.09087, %_98.i.i.i.i, !dbg !41269
  %_0.i2861 = fadd float %_0.i2865, %_0.i3301, !dbg !41271
  %_0.i3300 = fmul float %history.i.i.sroa.19.09087, %_101.i.i.i.i, !dbg !41273
  %_0.i2860 = fadd float %_0.i2864, %_0.i3300, !dbg !41275
  %_0.i3299 = fmul float %history.i.i.sroa.19.09087, %_104.i.i.i.i, !dbg !41277
  %_0.i2859 = fadd float %_0.i2863, %_0.i3299, !dbg !41279
  %_0.i3298 = fmul float %history.i.i.sroa.22.09088, %_109.i.i.i.i, !dbg !41281
  %_0.i2858 = fadd float %_0.i2862, %_0.i3298, !dbg !41283
  %_0.i3297 = fmul float %history.i.i.sroa.22.09088, %_112.i.i.i.i, !dbg !41285
  %_0.i2857 = fadd float %_0.i2861, %_0.i3297, !dbg !41287
  %_0.i3296 = fmul float %history.i.i.sroa.22.09088, %_115.i.i.i.i, !dbg !41289
  %_0.i2856 = fadd float %_0.i2860, %_0.i3296, !dbg !41291
  %_0.i3295 = fmul float %history.i.i.sroa.22.09088, %_118.i.i.i.i, !dbg !41293
  %_0.i2855 = fadd float %_0.i2859, %_0.i3295, !dbg !41295
  %_0.i3294 = fmul float %history.i.i.sroa.26.09089, %_123.i.i.i.i, !dbg !41297
  %_0.i2854 = fadd float %_0.i2858, %_0.i3294, !dbg !41299
  %_0.i3293 = fmul float %history.i.i.sroa.26.09089, %_126.i.i.i.i, !dbg !41301
  %_0.i2853 = fadd float %_0.i2857, %_0.i3293, !dbg !41303
  %_0.i3292 = fmul float %history.i.i.sroa.26.09089, %_129.i.i.i.i, !dbg !41305
  %_0.i2852 = fadd float %_0.i2856, %_0.i3292, !dbg !41307
  %_0.i3291 = fmul float %history.i.i.sroa.26.09089, %_132.i.i.i.i, !dbg !41309
  %_0.i2851 = fadd float %_0.i2855, %_0.i3291, !dbg !41311
  %_0.i3290 = fmul float %history.i.i.sroa.29.09090, %_137.i.i.i.i, !dbg !41313
  %_0.i2850 = fadd float %_0.i2854, %_0.i3290, !dbg !41315
  %_0.i3289 = fmul float %history.i.i.sroa.29.09090, %_140.i.i.i.i, !dbg !41317
  %_0.i2849 = fadd float %_0.i2853, %_0.i3289, !dbg !41319
  %_0.i3288 = fmul float %history.i.i.sroa.29.09090, %_143.i.i.i.i, !dbg !41321
  %_0.i2848 = fadd float %_0.i2852, %_0.i3288, !dbg !41323
  %_0.i3287 = fmul float %history.i.i.sroa.29.09090, %_146.i.i.i.i, !dbg !41325
  %_0.i2847 = fadd float %_0.i2851, %_0.i3287, !dbg !41327
  %_0.i3286 = fmul float %history.i.i.sroa.32.09091, %_151.i.i.i.i, !dbg !41329
  %_0.i2846 = fadd float %_0.i2850, %_0.i3286, !dbg !41331
  %_0.i3285 = fmul float %history.i.i.sroa.32.09091, %_154.i.i.i.i, !dbg !41333
  %_0.i2845 = fadd float %_0.i2849, %_0.i3285, !dbg !41335
  %_0.i3284 = fmul float %history.i.i.sroa.32.09091, %_157.i.i.i.i, !dbg !41337
  %_0.i2844 = fadd float %_0.i2848, %_0.i3284, !dbg !41339
  %_0.i3283 = fmul float %history.i.i.sroa.32.09091, %_160.i.i.i.i, !dbg !41341
  %_0.i2843 = fadd float %_0.i2847, %_0.i3283, !dbg !41343
  %_0.i3282 = fmul float %history.i.i.sroa.35.09092, %_165.i.i.i.i, !dbg !41345
  %_0.i2842 = fadd float %_0.i2846, %_0.i3282, !dbg !41347
  %_0.i3281 = fmul float %history.i.i.sroa.35.09092, %_168.i.i.i.i, !dbg !41349
  %_0.i2841 = fadd float %_0.i2845, %_0.i3281, !dbg !41351
  %_0.i3280 = fmul float %history.i.i.sroa.35.09092, %_171.i.i.i.i, !dbg !41353
  %_0.i2840 = fadd float %_0.i2844, %_0.i3280, !dbg !41355
  %_0.i3279 = fmul float %history.i.i.sroa.35.09092, %_174.i.i.i.i, !dbg !41357
  %_0.i2839 = fadd float %_0.i2843, %_0.i3279, !dbg !41359
  %740 = tail call noundef float @llvm.fabs.f32(float %_0.i2842), !dbg !41361
  %_3.i.i4262.inv = fcmp ogt float %739, %740, !dbg !41363
  %_4.i.i4269.v = select i1 %_3.i.i4262.inv, float %739, float %740, !dbg !41363
  %741 = tail call noundef float @llvm.fabs.f32(float %_0.i2841), !dbg !41361
  %_3.i.i4262.inv.1 = fcmp ogt float %_4.i.i4269.v, %741, !dbg !41363
  %_4.i.i4269.v.1 = select i1 %_3.i.i4262.inv.1, float %_4.i.i4269.v, float %741, !dbg !41363
  %742 = tail call noundef float @llvm.fabs.f32(float %_0.i2840), !dbg !41361
  %_3.i.i4262.inv.2 = fcmp ogt float %_4.i.i4269.v.1, %742, !dbg !41363
  %_4.i.i4269.v.2 = select i1 %_3.i.i4262.inv.2, float %_4.i.i4269.v.1, float %742, !dbg !41363
  %743 = tail call noundef float @llvm.fabs.f32(float %_0.i2839), !dbg !41361
  %_3.i.i4262.inv.3 = fcmp ogt float %_4.i.i4269.v.2, %743, !dbg !41363
  %_4.i.i4269.v.3 = select i1 %_3.i.i4262.inv.3, float %_4.i.i4269.v.2, float %743, !dbg !41363
  %744 = add nuw nsw i64 %iter.i.i.sroa.16.09093, 1, !dbg !41366
  %data.i4.i5128 = getelementptr inbounds nuw float, ptr %peaks_right.i, i64 %iter.i.i.sroa.16.09093, !dbg !41367
  store float %_4.i.i4269.v.3, ptr %data.i4.i5128, align 4, !dbg !41370, !alias.scope !41372, !noalias !41164
  %exitcond11814.not = icmp eq i64 %744, %umax11813, !dbg !41141
  br i1 %exitcond11814.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610, !dbg !41141

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5114
  %history.i.i.sroa.0.0.lcssa = phi float [ %history.i.i.sroa.0.0.lcssa14306, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5114 ], [ %_0.i3608, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610 ], !dbg !40882
  %history.i.i.sroa.7.0.lcssa = phi float [ %history.i.i.sroa.7.0.lcssa14324, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5114 ], [ %history.i.i.sroa.0.09082, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610 ], !dbg !40882
  %history.i.i.sroa.10.0.lcssa = phi float [ %history.i.i.sroa.10.0.lcssa14342, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5114 ], [ %history.i.i.sroa.7.09083, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610 ], !dbg !40882
  %history.i.i.sroa.13.0.lcssa = phi float [ %history.i.i.sroa.13.0.lcssa14343, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5114 ], [ %history.i.i.sroa.10.09084, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610 ], !dbg !40882
  %history.i.i.sroa.16.0.lcssa = phi float [ %history.i.i.sroa.16.0.lcssa14344, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5114 ], [ %history.i.i.sroa.13.09085, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610 ], !dbg !40882
  %history.i.i.sroa.19.0.lcssa = phi float [ %history.i.i.sroa.19.0.lcssa14345, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5114 ], [ %history.i.i.sroa.16.09086, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610 ], !dbg !40882
  %history.i.i.sroa.22.0.lcssa = phi float [ %history.i.i.sroa.22.0.lcssa14346, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5114 ], [ %history.i.i.sroa.19.09087, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610 ], !dbg !40882
  %history.i.i.sroa.26.0.lcssa = phi float [ %history.i.i.sroa.26.0.lcssa14347, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5114 ], [ %history.i.i.sroa.22.09088, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610 ], !dbg !40882
  %history.i.i.sroa.29.0.lcssa = phi float [ %history.i.i.sroa.29.0.lcssa14348, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5114 ], [ %history.i.i.sroa.26.09089, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610 ], !dbg !40882
  %history.i.i.sroa.32.0.lcssa = phi float [ %history.i.i.sroa.32.0.lcssa14349, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5114 ], [ %history.i.i.sroa.29.09090, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610 ], !dbg !40882
  %history.i.i.sroa.35.0.lcssa = phi float [ %history.i.i.sroa.35.0.lcssa14350, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5114 ], [ %history.i.i.sroa.32.09091, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610 ], !dbg !40882
  %history.i.i.sroa.38.0.lcssa = phi float [ %history.i.i.sroa.38.0.lcssa14351, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit5114 ], [ %history.i.i.sroa.35.09092, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3610 ], !dbg !40882
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !41375
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !41375
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !41375
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !41375
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !41375
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !41375
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !41375
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !41375
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !41375
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !41375
  br i1 %_2.i50619054.not, label %bb15.i.loopexit, label %bb20.i.lr.ph, !dbg !40854

bb20.i.lr.ph:                                     ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i
  %_70.i.sroa.3.0.copyload = load i64, ptr %_70.i.sroa.3.0..sroa_idx, align 8, !noalias !40826
  %_70.i.sroa.4.0.copyload = load i64, ptr %_70.i.sroa.4.0..sroa_idx, align 8, !noalias !40826
  %_71.i.sroa.3.0.copyload = load i64, ptr %_71.i.sroa.3.0..sroa_idx, align 8, !noalias !40826
  %_71.i.sroa.4.0.copyload = load i64, ptr %_71.i.sroa.4.0..sroa_idx, align 8, !noalias !40826
  %_54.0.i250.i = load ptr, ptr %uniform_left.i, align 8, !nonnull !12, !align !24
  %_54.1.i251.i = load i64, ptr %715, align 8
  %_18.i261.i = load i64, ptr %713, align 8
  %_56.0.i272.i = load ptr, ptr %716, align 8, !nonnull !12, !align !24
  %_56.1.i273.i = load i64, ptr %717, align 8
  %_58.1.i298.i = load i64, ptr %721, align 8
  %_58.0.i297.i = load ptr, ptr %722, align 8, !nonnull !12, !align !24
  %_54.0.i.i = load ptr, ptr %uniform_right.i, align 8, !nonnull !12, !align !24
  %_54.1.i.i = load i64, ptr %723, align 8
  %_18.i.i = load i64, ptr %714, align 8
  %_56.0.i.i = load ptr, ptr %724, align 8, !nonnull !12, !align !24
  %_56.1.i.i = load i64, ptr %725, align 8
  %_58.1.i.i = load i64, ptr %729, align 8
  %_58.0.i.i = load ptr, ptr %730, align 8, !nonnull !12, !align !24
  %umax11815 = call i64 @llvm.umax.i64(i64 %_18.i261.i, i64 1), !dbg !40854
  %umax11817 = call i64 @llvm.umax.i64(i64 %_18.i.i, i64 1), !dbg !40854
  %_8.i29.i = load float, ptr %_114.i, align 4
  %_9.i30.i = load float, ptr %_115.i, align 4
  %_8.i.i = load float, ptr %_119.i, align 4
  %_9.i.i = load float, ptr %_120.i, align 4
  %_37.i285.i = load float, ptr %719, align 4
  %_37.i.i = load float, ptr %727, align 4
  br label %bb20.i, !dbg !40854

bb20.i:                                           ; preds = %bb20.i.lr.ph, %bb32.i
  %running.sroa.0.0.i1725.lcssa1202414244 = phi float [ %running.sroa.0.0.i1725.lcssa1202414243.lcssa14412, %bb20.i.lr.ph ], [ %running.sroa.0.0.i1725.lcssa1202414243, %bb32.i ]
  %running.sroa.0.0.i1756.lcssa1197014227 = phi float [ %running.sroa.0.0.i1756.lcssa1197014226.lcssa14373, %bb20.i.lr.ph ], [ %running.sroa.0.0.i1756.lcssa1197014226, %bb32.i ]
  %_0.i3784.lcssa1203014210 = phi float [ %_0.i3784.lcssa1203014209.lcssa14441, %bb20.i.lr.ph ], [ %_0.i3784.lcssa1203014209, %bb32.i ]
  %_0.i3410.lcssa1202914192 = phi float [ %_0.i3410.lcssa1202914191.lcssa14438, %bb20.i.lr.ph ], [ %_0.i3410.lcssa1202914191, %bb32.i ]
  %_0.i3788.lcssa1200514174 = phi float [ %_0.i3788.lcssa1200514173.lcssa14435, %bb20.i.lr.ph ], [ %_0.i3788.lcssa1200514173, %bb32.i ]
  %_0.i3414.lcssa1198914156 = phi float [ %_0.i3414.lcssa1198914155.lcssa14432, %bb20.i.lr.ph ], [ %_0.i3414.lcssa1198914155, %bb32.i ]
  %running.sroa.0.0.i1725.lcssa92769422 = phi float [ %running.sroa.0.0.i1725.lcssa1202414243.lcssa14412, %bb20.i.lr.ph ], [ %running.sroa.0.0.i1725.lcssa92769421, %bb32.i ]
  %storemerge.i1730.lcssa92499386 = phi i32 [ %storemerge.i1730.lcssa92499385.lcssa14392, %bb20.i.lr.ph ], [ %storemerge.i1730.lcssa92499385, %bb32.i ]
  %running.sroa.0.0.i1756.lcssa91649383 = phi float [ %running.sroa.0.0.i1756.lcssa1197014226.lcssa14373, %bb20.i.lr.ph ], [ %running.sroa.0.0.i1756.lcssa91649382, %bb32.i ]
  %storemerge.i1761.lcssa91379347 = phi i32 [ %storemerge.i1761.lcssa91379346.lcssa14353, %bb20.i.lr.ph ], [ %storemerge.i1761.lcssa91379346, %bb32.i ]
  %frame.sroa.0.0.i9342 = phi i64 [ 0, %bb20.i.lr.ph ], [ %_85.i, %bb32.i ]
  %main_cursor.sroa.0.1.i9341 = phi i64 [ %main_cursor.sroa.0.0.i9425, %bb20.i.lr.ph ], [ %main_cursor.sroa.0.2.i, %bb32.i ]
  %ring_cursor.sroa.0.1.i9340 = phi i64 [ %ring_cursor.sroa.0.0.i9424, %bb20.i.lr.ph ], [ %ring_cursor.sroa.0.2.i, %bb32.i ]
  %_68.i = sub nuw nsw i64 %..i5032, %frame.sroa.0.0.i9342, !dbg !41376
  %ring.i2354 = load i64, ptr %671, align 8, !dbg !41377, !alias.scope !41379, !noalias !41382, !noundef !12
  %main.i2355 = load i64, ptr %672, align 8, !dbg !41386, !alias.scope !41379, !noalias !41382, !noundef !12
  %_10.i2356 = add i64 %ring_cursor.sroa.0.1.i9340, 1, !dbg !41387
  %_45.not.i2357 = icmp ult i64 %_10.i2356, %ring.i2354, !dbg !41388
  %745 = select i1 %_45.not.i2357, i64 0, i64 %ring.i2354, !dbg !41388
  %start1.sroa.0.0.i2358 = sub nuw i64 %_10.i2356, %745, !dbg !41388
  %_12.i2360 = add i64 %_70.i.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i9340, !dbg !41390
  %_46.not.i2361 = icmp ult i64 %_12.i2360, %ring.i2354, !dbg !41391
  %746 = select i1 %_46.not.i2361, i64 0, i64 %ring.i2354, !dbg !41391
  %left_end.sroa.0.0.i2362 = sub nuw i64 %_12.i2360, %746, !dbg !41391
  %_15.i2364 = add i64 %_71.i.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i9340, !dbg !41393
  %_47.not.i2365 = icmp ult i64 %_15.i2364, %ring.i2354, !dbg !41394
  %747 = select i1 %_47.not.i2365, i64 0, i64 %ring.i2354, !dbg !41394
  %right_end.sroa.0.0.i2366 = sub nuw i64 %_15.i2364, %747, !dbg !41394
  %_18.i2368 = add i64 %_70.i.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i9340, !dbg !41396
  %_48.not.i2369 = icmp ult i64 %_18.i2368, %ring.i2354, !dbg !41397
  %748 = select i1 %_48.not.i2369, i64 0, i64 %ring.i2354, !dbg !41397
  %left_expiring.sroa.0.0.i2370 = sub nuw i64 %_18.i2368, %748, !dbg !41397
  %_21.i2372 = add i64 %_71.i.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i9340, !dbg !41399
  %_49.not.i2373 = icmp ult i64 %_21.i2372, %ring.i2354, !dbg !41400
  %749 = select i1 %_49.not.i2373, i64 0, i64 %ring.i2354, !dbg !41400
  %right_expiring.sroa.0.0.i2374 = sub nuw i64 %_21.i2372, %749, !dbg !41400
  %_30.i2375 = sub i64 %ring.i2354, %ring_cursor.sroa.0.1.i9340, !dbg !41402
  %..i5145 = tail call noundef i64 @llvm.umin.i64(i64 %_30.i2375, i64 %_68.i), !dbg !41403
  %_31.i2377 = sub i64 %main.i2355, %main_cursor.sroa.0.1.i9341, !dbg !41405
  %..i5146 = tail call noundef i64 @llvm.umin.i64(i64 %_31.i2377, i64 %..i5145), !dbg !41406
  %_32.i2379 = sub i64 %ring.i2354, %start1.sroa.0.0.i2358, !dbg !41408
  %..i5147 = tail call noundef i64 @llvm.umin.i64(i64 %_32.i2379, i64 %..i5146), !dbg !41409
  %_34.i2381 = sub i64 %ring.i2354, %left_end.sroa.0.0.i2362, !dbg !41411
  %..i5148 = tail call noundef i64 @llvm.umin.i64(i64 %_34.i2381, i64 %..i5147), !dbg !41412
  %_36.i2383 = sub i64 %ring.i2354, %right_end.sroa.0.0.i2366, !dbg !41414
  %..i5149 = tail call noundef i64 @llvm.umin.i64(i64 %_36.i2383, i64 %..i5148), !dbg !41415
  %_38.i2385 = sub i64 %ring.i2354, %left_expiring.sroa.0.0.i2370, !dbg !41417
  %..i5150 = tail call noundef i64 @llvm.umin.i64(i64 %_38.i2385, i64 %..i5149), !dbg !41418
  %_40.i2387 = sub i64 %ring.i2354, %right_expiring.sroa.0.0.i2374, !dbg !41420
  %..i5151 = tail call noundef i64 @llvm.umin.i64(i64 %_40.i2387, i64 %..i5150), !dbg !41421
  %_74.i = add i64 %frame.sroa.0.0.i9342, %iter1.sroa.0.0.i9426, !dbg !41423
  %_78.i = add i64 %..i5151, %_74.i, !dbg !41426
  %_206.i = icmp ult i64 %_78.i, %_74.i, !dbg !41429
  %_202.not.i = icmp ugt i64 %_78.i, %left_io.1
  %or.cond27.i = or i1 %_206.i, %_202.not.i, !dbg !41429
  br i1 %or.cond27.i, label %bb62.i, label %bb61.i, !dbg !41429, !prof !165

bb62.i:                                           ; preds = %bb20.i
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40880
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !40880
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40882
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !40882
  store i32 %storemerge.i1761.lcssa91379346.lcssa14353, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014226.lcssa14373, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1730.lcssa92499385.lcssa14392, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1725.lcssa1202414243.lcssa14412, ptr %_21.i.i, align 4
  store float %_0.i3414.lcssa1198914156, ptr %718, align 4
  store float %_0.i3788.lcssa1200514174, ptr %720, align 4
  store float %_0.i3410.lcssa1202914192, ptr %726, align 4
  store float %_0.i3784.lcssa1203014210, ptr %728, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014227, ptr %_21.i264.i, align 1, !dbg !41437
  store float %running.sroa.0.0.i1725.lcssa1202414244, ptr %_21.i.i, align 1, !dbg !41455
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_74.i, i64 noundef %_78.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_debdb702bca99cd8ca93ec127a5c320a) #30, !dbg !41458, !noalias !40797
  unreachable, !dbg !41458

bb61.i:                                           ; preds = %bb20.i
  %_209.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %_74.i, !dbg !41459
  %_210.not.i = icmp ugt i64 %_78.i, %right_io.1, !dbg !41463
  br i1 %_210.not.i, label %bb65.i, label %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit5218, !dbg !41463, !prof !1406

bb65.i:                                           ; preds = %bb61.i
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40880
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !40880
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40882
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !40882
  store i32 %storemerge.i1761.lcssa91379346.lcssa14353, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014226.lcssa14373, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1730.lcssa92499385.lcssa14392, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1725.lcssa1202414243.lcssa14412, ptr %_21.i.i, align 4
  store float %_0.i3414.lcssa1198914156, ptr %718, align 4
  store float %_0.i3788.lcssa1200514174, ptr %720, align 4
  store float %_0.i3410.lcssa1202914192, ptr %726, align 4
  store float %_0.i3784.lcssa1203014210, ptr %728, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014227, ptr %_21.i264.i, align 1, !dbg !41437
  store float %running.sroa.0.0.i1725.lcssa1202414244, ptr %_21.i.i, align 1, !dbg !41455
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_74.i, i64 noundef %_78.i, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_01912ef845d3ed33a157086defa4d008) #30, !dbg !41467, !noalias !40797
  unreachable, !dbg !41467

_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit5218: ; preds = %bb61.i
  %_215.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %_74.i, !dbg !41468
  %_85.i = add nuw nsw i64 %..i5151, %frame.sroa.0.0.i9342, !dbg !41472
  %_224.i = getelementptr inbounds nuw float, ptr %peaks_left.i, i64 %frame.sroa.0.0.i9342, !dbg !41473
  %_233.i = getelementptr inbounds nuw float, ptr %peaks_right.i, i64 %frame.sroa.0.0.i9342, !dbg !41482
  %_2.i.i.i52219112.not = icmp eq i64 %..i5151, 0, !dbg !41491
  br i1 %_2.i.i.i52219112.not, label %bb32.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3590.lr.ph, !dbg !41491

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3590.lr.ph: ; preds = %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit5218
  %umin11821 = call i64 @llvm.umin.i64(i64 %_34.i2381, i64 %_36.i2383), !dbg !41491
  %umin11822 = call i64 @llvm.umin.i64(i64 %umin11821, i64 %_38.i2385), !dbg !41491
  %umin11823 = call i64 @llvm.umin.i64(i64 %umin11822, i64 %_40.i2387), !dbg !41491
  %umin11824 = call i64 @llvm.umin.i64(i64 %umin11823, i64 %_32.i2379), !dbg !41491
  %umin11825 = call i64 @llvm.umin.i64(i64 %umin11824, i64 %_30.i2375), !dbg !41491
  %umin11826 = call i64 @llvm.umin.i64(i64 %umin11825, i64 %_31.i2377), !dbg !41491
  %750 = sub nsw i64 %umin11827, %frame.sroa.0.0.i9342, !dbg !41491
  %umin11828 = call i64 @llvm.umin.i64(i64 %umin11826, i64 %750), !dbg !41491
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3590, !dbg !41491

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3590: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3590.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3739
  %_0.i37849311 = phi float [ %_0.i3784.lcssa1203014210, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3590.lr.ph ], [ %_0.i3784, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3739 ]
  %_0.i34109282 = phi float [ %_0.i3410.lcssa1202914192, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3590.lr.ph ], [ %_0.i3410, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3739 ]
  %running.sroa.0.0.i17259254 = phi float [ %running.sroa.0.0.i1725.lcssa92769422, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3590.lr.ph ], [ %running.sroa.0.0.i1725, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3739 ]
  %storemerge.i17309227 = phi i32 [ %storemerge.i1730.lcssa92499386, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3590.lr.ph ], [ %storemerge.i1730, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3739 ]
  %_0.i37889199 = phi float [ %_0.i3788.lcssa1200514174, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3590.lr.ph ], [ %_0.i3788, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3739 ]
  %_0.i34149170 = phi float [ %_0.i3414.lcssa1198914156, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3590.lr.ph ], [ %_0.i3414, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3739 ]
  %running.sroa.0.0.i17569142 = phi float [ %running.sroa.0.0.i1756.lcssa91649383, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3590.lr.ph ], [ %running.sroa.0.0.i1756, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3739 ]
  %storemerge.i17619115 = phi i32 [ %storemerge.i1761.lcssa91379347, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3590.lr.ph ], [ %storemerge.i1761, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3739 ]
  %iter.i.sroa.36.09114 = phi i64 [ 0, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3590.lr.ph ], [ %751, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3739 ]
  %751 = add nuw i64 %iter.i.sroa.36.09114, 1, !dbg !41495
  %data.i.i.i.i.i.i.i.i5234 = getelementptr inbounds nuw float, ptr %_209.i, i64 %iter.i.sroa.36.09114, !dbg !41496
  %data.i.i.i.i5239 = getelementptr inbounds nuw float, ptr %_233.i, i64 %iter.i.sroa.36.09114, !dbg !41503
  %data.i.i.i.i.i.i5244 = getelementptr inbounds nuw float, ptr %_224.i, i64 %iter.i.sroa.36.09114, !dbg !41506
  %data.i5.i.i.i.i.i.i.i5248 = getelementptr inbounds nuw float, ptr %_215.i, i64 %iter.i.sroa.36.09114, !dbg !41509
  %_0.i3603 = load float, ptr %data.i.i.i.i.i.i5244, align 4, !dbg !41512, !alias.scope !41514, !noalias !40797, !noundef !12
  %_0.i3598 = load float, ptr %data.i.i.i.i5239, align 4, !dbg !41517, !alias.scope !41519, !noalias !40797, !noundef !12
  %_3.i.i4253 = fcmp ule float %_0.i3598, %_0.i3603, !dbg !41522
  %_6.i.i4255 = bitcast float %_0.i3598 to i32, !dbg !41525
  %_8.i.i4257 = bitcast float %_0.i3603 to i32, !dbg !41528
  %_4.i.i4260 = select i1 %_3.i.i4253, i32 %_8.i.i4257, i32 %_6.i.i4255, !dbg !41530
  %_5.i4044 = and i32 %_4.i.i4260, %.none.i, !dbg !41531
  %_7.i4040 = and i32 %_9.i4046, %_6.i.i4255, !dbg !41533
  %_4.i4041 = or disjoint i32 %_5.i4044, %_7.i4040, !dbg !41535
  %_0.i4042 = bitcast i32 %_4.i4041 to float, !dbg !41536
  %_0.i3593 = load float, ptr %data.i.i.i.i.i.i.i.i5234, align 4, !dbg !41538, !alias.scope !41540, !noalias !40797, !noundef !12
  %_0.i3588 = load float, ptr %data.i5.i.i.i.i.i.i.i5248, align 4, !dbg !41543, !alias.scope !41545, !noalias !40797, !noundef !12
  %_235.i = add nuw i64 %iter.i.sroa.36.09114, %ring_cursor.sroa.0.1.i9340, !dbg !41548
  %_236.i = add nuw i64 %iter.i.sroa.36.09114, %main_cursor.sroa.0.1.i9341, !dbg !41551
  %_237.i = add nuw i64 %iter.i.sroa.36.09114, %left_end.sroa.0.0.i2362, !dbg !41552
  %_238.i = add i64 %iter.i.sroa.36.09114, %start1.sroa.0.0.i2358, !dbg !41553
  %_239.i = add nuw i64 %iter.i.sroa.36.09114, %left_expiring.sroa.0.0.i2370, !dbg !41554
  %_7.i8.i253.i = add i64 %_235.i, 1, !dbg !41555
  %or.cond.i11.i256.i.not = icmp ult i64 %_235.i, %_54.1.i251.i, !dbg !41557
  br i1 %or.cond.i11.i256.i.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i257.i, label %bb4.i13.i312.i, !dbg !41557, !prof !2740

bb4.i13.i312.i:                                   ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3590
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40880
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !40880
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40882
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !40882
  store i32 %storemerge.i1761.lcssa91379346.lcssa14353, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014226.lcssa14373, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1730.lcssa92499385.lcssa14392, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1725.lcssa1202414243.lcssa14412, ptr %_21.i.i, align 4
  store float %_0.i3414.lcssa1198914156, ptr %718, align 4
  store float %_0.i3788.lcssa1200514174, ptr %720, align 4
  store float %_0.i3410.lcssa1202914192, ptr %726, align 4
  store float %_0.i3784.lcssa1203014210, ptr %728, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014227, ptr %_21.i264.i, align 1, !dbg !41437
  store float %running.sroa.0.0.i1725.lcssa1202414244, ptr %_21.i.i, align 1, !dbg !41455
  %umax11819 = call i64 @llvm.umax.i64(i64 %ring_cursor.sroa.0.1.i9340, i64 %_54.1.i251.i), !dbg !41491
  %752 = add i64 %umax11819, 1, !dbg !41491
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_235.i, i64 noundef %752, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i251.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !41561, !noalias !41562
  unreachable, !dbg !41561

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i257.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3590
  %_7.i4047 = and i32 %_9.i4046, %_8.i.i4257, !dbg !41570
  %_4.i4048 = or disjoint i32 %_5.i4044, %_7.i4047, !dbg !41531
  %_0.i4049 = bitcast i32 %_4.i4048 to float, !dbg !41571
  %_0.i2950 = fdiv float %_8.i29.i, %_0.i4049, !dbg !41573
  %_3.i2517 = fcmp uge float %_8.i29.i, %_0.i4049, !dbg !41575
  %_0.i4035 = select i1 %_3.i2517, float 1.000000e+00, float %_0.i2950, !dbg !41577
  %_17.i12.i258.i = getelementptr inbounds nuw float, ptr %_54.0.i250.i, i64 %_235.i, !dbg !41579
  store float %_0.i4035, ptr %_17.i12.i258.i, align 4, !dbg !41581, !alias.scope !41583, !noalias !41586
  %or.cond.i1803.not = icmp ult i64 %_237.i, %_54.1.i251.i, !dbg !41587
  br i1 %or.cond.i1803.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1807, label %bb4.i1806, !dbg !41587, !prof !2740

bb4.i1806:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i257.i
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40880
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !40880
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40882
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !40882
  store i32 %storemerge.i1761.lcssa91379346.lcssa14353, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014226.lcssa14373, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1730.lcssa92499385.lcssa14392, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1725.lcssa1202414243.lcssa14412, ptr %_21.i.i, align 4
  store float %_0.i3414.lcssa1198914156, ptr %718, align 4
  store float %_0.i3788.lcssa1200514174, ptr %720, align 4
  store float %_0.i3410.lcssa1202914192, ptr %726, align 4
  store float %_0.i3784.lcssa1203014210, ptr %728, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014227, ptr %_21.i264.i, align 1, !dbg !41437
  store float %running.sroa.0.0.i1725.lcssa1202414244, ptr %_21.i.i, align 1, !dbg !41455
  %_5.i1800 = add i64 %_237.i, 1, !dbg !41592
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_237.i, i64 noundef %_5.i1800, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i251.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !41593, !noalias !41594
  unreachable, !dbg !41593

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1807: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i257.i
  %_15.i1804 = getelementptr inbounds nuw float, ptr %_54.0.i250.i, i64 %_237.i, !dbg !41600
  %_0.i3456 = load float, ptr %_15.i1804, align 4, !dbg !41602, !alias.scope !41604, !noalias !41607, !noundef !12
  %position.i1752 = zext i32 %storemerge.i17619115 to i64, !dbg !41608
  %753 = icmp eq i32 %storemerge.i17619115, 0, !dbg !41609
  %_3.i.i4298.inv = fcmp olt float %running.sroa.0.0.i17569142, %_0.i3456, !dbg !41609
  %_4.i.i4305.v = select i1 %_3.i.i4298.inv, float %running.sroa.0.0.i17569142, float %_0.i3456, !dbg !41609
  %running.sroa.0.0.i1756 = select i1 %753, float %_0.i3456, float %_4.i.i4305.v, !dbg !41609
  %_15.i1757 = add nuw nsw i64 %position.i1752, 1, !dbg !41610
  %complete.i1758 = icmp eq i64 %_15.i1757, %_18.i261.i, !dbg !41610
  br i1 %complete.i1758, label %bb19.i1769, label %bb7.i1759, !dbg !41611

bb7.i1759:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1807
  %or.cond.i1795.not = icmp ult i64 %_238.i, %_54.1.i251.i, !dbg !41612
  br i1 %or.cond.i1795.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1799, label %bb4.i1798, !dbg !41612, !prof !2740

bb4.i1798:                                        ; preds = %bb7.i1759
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40880
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !40880
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40882
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !40882
  store i32 %storemerge.i1761.lcssa91379346.lcssa14353, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014226.lcssa14373, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1730.lcssa92499385.lcssa14392, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1725.lcssa1202414243.lcssa14412, ptr %_21.i.i, align 4
  store float %_0.i3414.lcssa1198914156, ptr %718, align 4
  store float %_0.i3788.lcssa1200514174, ptr %720, align 4
  store float %_0.i3410.lcssa1202914192, ptr %726, align 4
  store float %_0.i3784.lcssa1203014210, ptr %728, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014227, ptr %_21.i264.i, align 1, !dbg !41437
  store float %running.sroa.0.0.i1725.lcssa1202414244, ptr %_21.i.i, align 1, !dbg !41455
  %_5.i1792 = add i64 %_238.i, 1, !dbg !41617
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_238.i, i64 noundef %_5.i1792, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i251.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !41618, !noalias !41619
  unreachable, !dbg !41618

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1799: ; preds = %bb7.i1759
  %_15.i1796 = getelementptr inbounds nuw float, ptr %_54.0.i250.i, i64 %_238.i, !dbg !41622
  %_0.i3458 = load float, ptr %_15.i1796, align 4, !dbg !41624, !alias.scope !41626, !noalias !41607, !noundef !12
  %_3.i.i4289.inv = fcmp olt float %_0.i3458, %running.sroa.0.0.i1756, !dbg !41629
  %_4.i.i4296.v = select i1 %_3.i.i4289.inv, float %_0.i3458, float %running.sroa.0.0.i1756, !dbg !41629
  %754 = trunc i64 %_15.i1757 to i32, !dbg !41632
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1780, !dbg !41633

bb19.i1769:                                       ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1807, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1776
  %end.sroa.0.0.i17679108 = phi i64 [ %756, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1776 ], [ %_237.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1807 ]
  %suffix.sroa.0.0.i17669107 = phi float [ %_4.i.i4287.v, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1776 ], [ %_0.i3456, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1807 ]
  %iter.sroa.0.0.i17659106 = phi i64 [ %_30.i1770, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1776 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1807 ]
  %or.cond.i1782.not = icmp ult i64 %end.sroa.0.0.i17679108, %_54.1.i251.i, !dbg !41634
  br i1 %or.cond.i1782.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1776, label %bb4.i, !dbg !41634, !prof !2740

bb4.i:                                            ; preds = %bb19.i1769
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40880
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !40880
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40882
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !40882
  store i32 %storemerge.i1761.lcssa91379346.lcssa14353, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014226.lcssa14373, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1730.lcssa92499385.lcssa14392, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1725.lcssa1202414243.lcssa14412, ptr %_21.i.i, align 4
  store float %_0.i3414.lcssa1198914156, ptr %718, align 4
  store float %_0.i3788.lcssa1200514174, ptr %720, align 4
  store float %_0.i3410.lcssa1202914192, ptr %726, align 4
  store float %_0.i3784.lcssa1203014210, ptr %728, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014227, ptr %_21.i264.i, align 1, !dbg !41437
  store float %running.sroa.0.0.i1725.lcssa1202414244, ptr %_21.i.i, align 1, !dbg !41455
  %_5.i = add i64 %end.sroa.0.0.i17679108, 1, !dbg !41639
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %end.sroa.0.0.i17679108, i64 noundef %_5.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i251.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !41640, !noalias !41641
  unreachable, !dbg !41640

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1776: ; preds = %bb19.i1769
  %_30.i1770 = add nuw i64 %iter.sroa.0.0.i17659106, 1, !dbg !41644
  %_15.i1783 = getelementptr inbounds nuw float, ptr %_54.0.i250.i, i64 %end.sroa.0.0.i17679108, !dbg !41649
  %_0.i3462 = load float, ptr %_15.i1783, align 4, !dbg !41651, !alias.scope !41653, !noalias !41607, !noundef !12
  %_3.i.i4280.inv = fcmp olt float %suffix.sroa.0.0.i17669107, %_0.i3462, !dbg !41656
  %_4.i.i4287.v = select i1 %_3.i.i4280.inv, float %suffix.sroa.0.0.i17669107, float %_0.i3462, !dbg !41656
  store float %_4.i.i4287.v, ptr %_15.i1783, align 4, !dbg !41659, !alias.scope !41662, !noalias !41607
  %755 = icmp eq i64 %end.sroa.0.0.i17679108, 0, !dbg !41665
  %spec.store.select.i1778 = select i1 %755, i64 %ring.i, i64 %end.sroa.0.0.i17679108, !dbg !41665
  %756 = add i64 %spec.store.select.i1778, -1, !dbg !41666
  %exitcond11816.not = icmp eq i64 %_30.i1770, %umax11815, !dbg !41667
  br i1 %exitcond11816.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1780, label %bb19.i1769, !dbg !41669

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1780: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1776, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1799
  %storemerge.i1761 = phi i32 [ %754, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1799 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1776 ], !dbg !41670
  %running.sroa.0.1.i1762 = phi float [ %_4.i.i4296.v, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1799 ], [ %running.sroa.0.0.i1756, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1776 ], !dbg !41437
  %_0.i3278 = fmul float %running.sroa.0.1.i1762, 1.638400e+04, !dbg !41671
  %757 = tail call noundef float @llvm.floor.f32(float %_0.i3278), !dbg !41673
  %_0.i3277 = fmul float %757, 0x3F10000000000000, !dbg !41677
  %or.cond.i1963.not = icmp ult i64 %_239.i, %_56.1.i273.i, !dbg !41679
  br i1 %or.cond.i1963.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1967, label %bb4.i1966, !dbg !41679, !prof !2740

bb4.i1966:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1780
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40880
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !40880
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40882
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !40882
  store i32 %storemerge.i1761.lcssa91379346.lcssa14353, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014226.lcssa14373, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1730.lcssa92499385.lcssa14392, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1725.lcssa1202414243.lcssa14412, ptr %_21.i.i, align 4
  store float %_0.i3414.lcssa1198914156, ptr %718, align 4
  store float %_0.i3788.lcssa1200514174, ptr %720, align 4
  store float %_0.i3410.lcssa1202914192, ptr %726, align 4
  store float %_0.i3784.lcssa1203014210, ptr %728, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014227, ptr %_21.i264.i, align 1, !dbg !41437
  store float %running.sroa.0.0.i1725.lcssa1202414244, ptr %_21.i.i, align 1, !dbg !41455
  %_5.i1960 = add i64 %_239.i, 1, !dbg !41684
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_239.i, i64 noundef %_5.i1960, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i273.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !41685, !noalias !41686
  unreachable, !dbg !41685

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1967: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1780
  %_15.i1964 = getelementptr inbounds nuw float, ptr %_56.0.i272.i, i64 %_239.i, !dbg !41689
  %_0.i3416 = load float, ptr %_15.i1964, align 4, !dbg !41691, !alias.scope !41693, !noalias !41696, !noundef !12
  %_0.i2838 = fadd float %_0.i3277, %_0.i34149170, !dbg !41697
  %_0.i3414 = fsub float %_0.i2838, %_0.i3416, !dbg !41699
  %_8.not.i3.i282.i = icmp ugt i64 %_7.i8.i253.i, %_56.1.i273.i
  br i1 %_8.not.i3.i282.i, label %bb4.i6.i311.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i283.i, !dbg !41701, !prof !165

bb4.i6.i311.i:                                    ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1967
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40880
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !40880
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40882
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !40882
  store i32 %storemerge.i1761.lcssa91379346.lcssa14353, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014226.lcssa14373, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1730.lcssa92499385.lcssa14392, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1725.lcssa1202414243.lcssa14412, ptr %_21.i.i, align 4
  store float %_0.i3414.lcssa1198914156, ptr %718, align 4
  store float %_0.i3788.lcssa1200514174, ptr %720, align 4
  store float %_0.i3410.lcssa1202914192, ptr %726, align 4
  store float %_0.i3784.lcssa1203014210, ptr %728, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014227, ptr %_21.i264.i, align 1, !dbg !41437
  store float %running.sroa.0.0.i1725.lcssa1202414244, ptr %_21.i.i, align 1, !dbg !41455
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_235.i, i64 noundef %_7.i8.i253.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i273.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !41706, !noalias !41707
  unreachable, !dbg !41706

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i283.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1967
  %_17.i5.i284.i = getelementptr inbounds nuw float, ptr %_56.0.i272.i, i64 %_235.i, !dbg !41710
  store float %_0.i3277, ptr %_17.i5.i284.i, align 4, !dbg !41712, !alias.scope !41714, !noalias !41696
  %_0.i2949 = fdiv float %_0.i3414, %_37.i285.i, !dbg !41717
  %_0.i3413 = fsub float 1.000000e+00, %_0.i2949, !dbg !41719
  %_0.i3412 = fsub float %_0.i3413, %_0.i37889199, !dbg !41721
  %_4.i2965 = fmul float %_9.i30.i, %_0.i3412, !dbg !41723
  %_0.i2966 = fadd float %_0.i37889199, %_4.i2965, !dbg !41723
  %_3.i.i4244.inv = fcmp ogt float %_0.i3413, %_0.i2966, !dbg !41725
  %_4.i.i4251.v = select i1 %_3.i.i4244.inv, float %_0.i3413, float %_0.i2966, !dbg !41725
  %758 = tail call noundef float @llvm.fabs.f32(float %_4.i.i4251.v), !dbg !41728
  %759 = fcmp uge float %758, 0x3BC79CA100000000, !dbg !41731
  %_0.i3788 = select i1 %759, float %_4.i.i4251.v, float 0.000000e+00, !dbg !41733
  %_5.i1952 = add i64 %_236.i, 1, !dbg !41734
  %or.cond.i1955.not = icmp ult i64 %_236.i, %_58.1.i298.i, !dbg !41736
  br i1 %or.cond.i1955.not, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3746, label %bb4.i1958, !dbg !41736, !prof !2740

bb4.i1958:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i283.i
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40880
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !40880
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40882
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !40882
  store i32 %storemerge.i1761.lcssa91379346.lcssa14353, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014226.lcssa14373, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1730.lcssa92499385.lcssa14392, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1725.lcssa1202414243.lcssa14412, ptr %_21.i.i, align 4
  store float %_0.i3414.lcssa1198914156, ptr %718, align 4
  store float %_0.i3788.lcssa1200514174, ptr %720, align 4
  store float %_0.i3410.lcssa1202914192, ptr %726, align 4
  store float %_0.i3784.lcssa1203014210, ptr %728, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014227, ptr %_21.i264.i, align 1, !dbg !41437
  store float %running.sroa.0.0.i1725.lcssa1202414244, ptr %_21.i.i, align 1, !dbg !41455
  %umax11820 = call i64 @llvm.umax.i64(i64 %main_cursor.sroa.0.1.i9341, i64 %_58.1.i298.i), !dbg !41491
  %760 = add i64 %umax11820, 1, !dbg !41491
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_236.i, i64 noundef %760, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i298.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !41740, !noalias !41741
  unreachable, !dbg !41740

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3746: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i283.i
  %_0.i3411 = fsub float 1.000000e+00, %_0.i3788, !dbg !41744
  %_15.i1956 = getelementptr inbounds nuw float, ptr %_58.0.i297.i, i64 %_236.i, !dbg !41746
  %_0.i3418 = load float, ptr %_15.i1956, align 4, !dbg !41748, !alias.scope !41750, !noalias !41696, !noundef !12
  store float %_0.i3593, ptr %_15.i1956, align 4, !dbg !41753, !alias.scope !41756, !noalias !41696
  %_0.i3276 = fmul float %_0.i3411, %_0.i3418, !dbg !41759
  %_6.i4023 = bitcast float %_0.i3418 to i32, !dbg !41761
  %_5.i4024 = and i32 %_6.i4023, %all.sroa.0.0.i, !dbg !41764
  %_8.i4025 = bitcast float %_0.i3276 to i32, !dbg !41765
  %_7.i4027 = and i32 %_9.i4026, %_8.i4025, !dbg !41767
  %_4.i4028 = or disjoint i32 %_7.i4027, %_5.i4024, !dbg !41764
  store i32 %_4.i4028, ptr %data.i.i.i.i.i.i.i.i5234, align 4, !dbg !41768, !alias.scope !41770, !noalias !41773
  %_242.i = add nuw i64 %iter.i.sroa.36.09114, %right_end.sroa.0.0.i2366, !dbg !41774
  %_244.i = add nuw i64 %iter.i.sroa.36.09114, %right_expiring.sroa.0.0.i2374, !dbg !41776
  %_8.not.i10.i.i = icmp ugt i64 %_7.i8.i253.i, %_54.1.i.i
  br i1 %_8.not.i10.i.i, label %bb4.i13.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i, !dbg !41777, !prof !165

bb4.i13.i.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3746
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40880
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !40880
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40882
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !40882
  store i32 %storemerge.i1761.lcssa91379346.lcssa14353, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014226.lcssa14373, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1730.lcssa92499385.lcssa14392, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1725.lcssa1202414243.lcssa14412, ptr %_21.i.i, align 4
  store float %_0.i3414.lcssa1198914156, ptr %718, align 4
  store float %_0.i3788.lcssa1200514174, ptr %720, align 4
  store float %_0.i3410.lcssa1202914192, ptr %726, align 4
  store float %_0.i3784.lcssa1203014210, ptr %728, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014227, ptr %_21.i264.i, align 1, !dbg !41437
  store float %running.sroa.0.0.i1725.lcssa1202414244, ptr %_21.i.i, align 1, !dbg !41455
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_235.i, i64 noundef %_7.i8.i253.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !41782, !noalias !41783
  unreachable, !dbg !41782

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3746
  %_0.i2948 = fdiv float %_8.i.i, %_0.i4042, !dbg !41791
  %_3.i2515 = fcmp uge float %_8.i.i, %_0.i4042, !dbg !41793
  %_0.i4022 = select i1 %_3.i2515, float 1.000000e+00, float %_0.i2948, !dbg !41795
  %_17.i12.i.i = getelementptr inbounds nuw float, ptr %_54.0.i.i, i64 %_235.i, !dbg !41797
  store float %_0.i4022, ptr %_17.i12.i.i, align 4, !dbg !41799, !alias.scope !41801, !noalias !41804
  %or.cond.i1835.not = icmp ult i64 %_242.i, %_54.1.i.i, !dbg !41805
  br i1 %or.cond.i1835.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1839, label %bb4.i1838, !dbg !41805, !prof !2740

bb4.i1838:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40880
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !40880
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40882
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !40882
  store i32 %storemerge.i1761.lcssa91379346.lcssa14353, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014226.lcssa14373, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1730.lcssa92499385.lcssa14392, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1725.lcssa1202414243.lcssa14412, ptr %_21.i.i, align 4
  store float %_0.i3414.lcssa1198914156, ptr %718, align 4
  store float %_0.i3788.lcssa1200514174, ptr %720, align 4
  store float %_0.i3410.lcssa1202914192, ptr %726, align 4
  store float %_0.i3784.lcssa1203014210, ptr %728, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014227, ptr %_21.i264.i, align 1, !dbg !41437
  store float %running.sroa.0.0.i1725.lcssa1202414244, ptr %_21.i.i, align 1, !dbg !41455
  %_5.i1832 = add i64 %_242.i, 1, !dbg !41810
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_242.i, i64 noundef %_5.i1832, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !41811, !noalias !41812
  unreachable, !dbg !41811

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1839: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i
  %_15.i1836 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i64 %_242.i, !dbg !41818
  %_0.i3448 = load float, ptr %_15.i1836, align 4, !dbg !41820, !alias.scope !41822, !noalias !41825, !noundef !12
  %position.i1721 = zext i32 %storemerge.i17309227 to i64, !dbg !41826
  %761 = icmp eq i32 %storemerge.i17309227, 0, !dbg !41827
  %_3.i.i4325.inv = fcmp olt float %running.sroa.0.0.i17259254, %_0.i3448, !dbg !41827
  %_4.i.i4332.v = select i1 %_3.i.i4325.inv, float %running.sroa.0.0.i17259254, float %_0.i3448, !dbg !41827
  %running.sroa.0.0.i1725 = select i1 %761, float %_0.i3448, float %_4.i.i4332.v, !dbg !41827
  %_15.i1726 = add nuw nsw i64 %position.i1721, 1, !dbg !41828
  %complete.i1727 = icmp eq i64 %_15.i1726, %_18.i.i, !dbg !41828
  br i1 %complete.i1727, label %bb19.i1738, label %bb7.i1728, !dbg !41829

bb7.i1728:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1839
  %or.cond.i1827.not = icmp ult i64 %_238.i, %_54.1.i.i, !dbg !41830
  br i1 %or.cond.i1827.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1831, label %bb4.i1830, !dbg !41830, !prof !2740

bb4.i1830:                                        ; preds = %bb7.i1728
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40880
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !40880
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40882
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !40882
  store i32 %storemerge.i1761.lcssa91379346.lcssa14353, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014226.lcssa14373, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1730.lcssa92499385.lcssa14392, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1725.lcssa1202414243.lcssa14412, ptr %_21.i.i, align 4
  store float %_0.i3414.lcssa1198914156, ptr %718, align 4
  store float %_0.i3788.lcssa1200514174, ptr %720, align 4
  store float %_0.i3410.lcssa1202914192, ptr %726, align 4
  store float %_0.i3784.lcssa1203014210, ptr %728, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014227, ptr %_21.i264.i, align 1, !dbg !41437
  store float %running.sroa.0.0.i1725.lcssa1202414244, ptr %_21.i.i, align 1, !dbg !41455
  %_5.i1824 = add i64 %_238.i, 1, !dbg !41835
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_238.i, i64 noundef %_5.i1824, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !41836, !noalias !41837
  unreachable, !dbg !41836

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1831: ; preds = %bb7.i1728
  %_15.i1828 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i64 %_238.i, !dbg !41840
  %_0.i3450 = load float, ptr %_15.i1828, align 4, !dbg !41842, !alias.scope !41844, !noalias !41825, !noundef !12
  %_3.i.i4316.inv = fcmp olt float %_0.i3450, %running.sroa.0.0.i1725, !dbg !41847
  %_4.i.i4323.v = select i1 %_3.i.i4316.inv, float %_0.i3450, float %running.sroa.0.0.i1725, !dbg !41847
  %762 = trunc i64 %_15.i1726 to i32, !dbg !41850
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1749, !dbg !41851

bb19.i1738:                                       ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1839, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1745
  %end.sroa.0.0.i17369111 = phi i64 [ %764, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1745 ], [ %_242.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1839 ]
  %suffix.sroa.0.0.i17359110 = phi float [ %_4.i.i4314.v, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1745 ], [ %_0.i3448, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1839 ]
  %iter.sroa.0.0.i17349109 = phi i64 [ %_30.i1739, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1745 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1839 ]
  %or.cond.i1811.not = icmp ult i64 %end.sroa.0.0.i17369111, %_54.1.i.i, !dbg !41852
  br i1 %or.cond.i1811.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1745, label %bb4.i1814, !dbg !41852, !prof !2740

bb4.i1814:                                        ; preds = %bb19.i1738
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40880
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !40880
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40882
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !40882
  store i32 %storemerge.i1761.lcssa91379346.lcssa14353, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014226.lcssa14373, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1730.lcssa92499385.lcssa14392, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1725.lcssa1202414243.lcssa14412, ptr %_21.i.i, align 4
  store float %_0.i3414.lcssa1198914156, ptr %718, align 4
  store float %_0.i3788.lcssa1200514174, ptr %720, align 4
  store float %_0.i3410.lcssa1202914192, ptr %726, align 4
  store float %_0.i3784.lcssa1203014210, ptr %728, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014227, ptr %_21.i264.i, align 1, !dbg !41437
  store float %running.sroa.0.0.i1725.lcssa1202414244, ptr %_21.i.i, align 1, !dbg !41455
  %_5.i1808 = add i64 %end.sroa.0.0.i17369111, 1, !dbg !41857
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %end.sroa.0.0.i17369111, i64 noundef %_5.i1808, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !41858, !noalias !41859
  unreachable, !dbg !41858

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1745: ; preds = %bb19.i1738
  %_30.i1739 = add nuw i64 %iter.sroa.0.0.i17349109, 1, !dbg !41862
  %_15.i1812 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i64 %end.sroa.0.0.i17369111, !dbg !41867
  %_0.i3454 = load float, ptr %_15.i1812, align 4, !dbg !41869, !alias.scope !41871, !noalias !41825, !noundef !12
  %_3.i.i4307.inv = fcmp olt float %suffix.sroa.0.0.i17359110, %_0.i3454, !dbg !41874
  %_4.i.i4314.v = select i1 %_3.i.i4307.inv, float %suffix.sroa.0.0.i17359110, float %_0.i3454, !dbg !41874
  store float %_4.i.i4314.v, ptr %_15.i1812, align 4, !dbg !41877, !alias.scope !41880, !noalias !41825
  %763 = icmp eq i64 %end.sroa.0.0.i17369111, 0, !dbg !41883
  %spec.store.select.i1747 = select i1 %763, i64 %ring.i, i64 %end.sroa.0.0.i17369111, !dbg !41883
  %764 = add i64 %spec.store.select.i1747, -1, !dbg !41884
  %exitcond11818.not = icmp eq i64 %_30.i1739, %umax11817, !dbg !41885
  br i1 %exitcond11818.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1749, label %bb19.i1738, !dbg !41887

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1749: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1745, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1831
  %storemerge.i1730 = phi i32 [ %762, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1831 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1745 ], !dbg !41888
  %running.sroa.0.1.i1731 = phi float [ %_4.i.i4323.v, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1831 ], [ %running.sroa.0.0.i1725, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1745 ], !dbg !41455
  %_0.i3275 = fmul float %running.sroa.0.1.i1731, 1.638400e+04, !dbg !41889
  %765 = tail call noundef float @llvm.floor.f32(float %_0.i3275), !dbg !41891
  %_0.i3274 = fmul float %765, 0x3F10000000000000, !dbg !41895
  %or.cond.i1947.not = icmp ult i64 %_244.i, %_56.1.i.i, !dbg !41897
  br i1 %or.cond.i1947.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1951, label %bb4.i1950, !dbg !41897, !prof !2740

bb4.i1950:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1749
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40880
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !40880
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40882
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !40882
  store i32 %storemerge.i1761.lcssa91379346.lcssa14353, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014226.lcssa14373, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1730.lcssa92499385.lcssa14392, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1725.lcssa1202414243.lcssa14412, ptr %_21.i.i, align 4
  store float %_0.i3414.lcssa1198914156, ptr %718, align 4
  store float %_0.i3788.lcssa1200514174, ptr %720, align 4
  store float %_0.i3410.lcssa1202914192, ptr %726, align 4
  store float %_0.i3784.lcssa1203014210, ptr %728, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014227, ptr %_21.i264.i, align 1, !dbg !41437
  store float %running.sroa.0.0.i1725.lcssa1202414244, ptr %_21.i.i, align 1, !dbg !41455
  %_5.i1944 = add i64 %_244.i, 1, !dbg !41902
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_244.i, i64 noundef %_5.i1944, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !41903, !noalias !41904
  unreachable, !dbg !41903

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1951: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1749
  %_15.i1948 = getelementptr inbounds nuw float, ptr %_56.0.i.i, i64 %_244.i, !dbg !41907
  %_0.i3420 = load float, ptr %_15.i1948, align 4, !dbg !41909, !alias.scope !41911, !noalias !41914, !noundef !12
  %_0.i2837 = fadd float %_0.i3274, %_0.i34109282, !dbg !41915
  %_0.i3410 = fsub float %_0.i2837, %_0.i3420, !dbg !41917
  %_8.not.i3.i.i = icmp ugt i64 %_7.i8.i253.i, %_56.1.i.i
  br i1 %_8.not.i3.i.i, label %bb4.i6.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i, !dbg !41919, !prof !165

bb4.i6.i.i:                                       ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1951
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40880
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !40880
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40882
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !40882
  store i32 %storemerge.i1761.lcssa91379346.lcssa14353, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014226.lcssa14373, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1730.lcssa92499385.lcssa14392, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1725.lcssa1202414243.lcssa14412, ptr %_21.i.i, align 4
  store float %_0.i3414.lcssa1198914156, ptr %718, align 4
  store float %_0.i3788.lcssa1200514174, ptr %720, align 4
  store float %_0.i3410.lcssa1202914192, ptr %726, align 4
  store float %_0.i3784.lcssa1203014210, ptr %728, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014227, ptr %_21.i264.i, align 1, !dbg !41437
  store float %running.sroa.0.0.i1725.lcssa1202414244, ptr %_21.i.i, align 1, !dbg !41455
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_235.i, i64 noundef %_7.i8.i253.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !41924, !noalias !41925
  unreachable, !dbg !41924

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1951
  %_17.i5.i.i = getelementptr inbounds nuw float, ptr %_56.0.i.i, i64 %_235.i, !dbg !41928
  store float %_0.i3274, ptr %_17.i5.i.i, align 4, !dbg !41930, !alias.scope !41932, !noalias !41914
  %_6.not.i1938 = icmp ugt i64 %_5.i1952, %_58.1.i.i
  br i1 %_6.not.i1938, label %bb4.i1942, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3739, !dbg !41935, !prof !165

bb4.i1942:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40880
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !40880
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40882
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !40882
  store i32 %storemerge.i1761.lcssa91379346.lcssa14353, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014226.lcssa14373, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1730.lcssa92499385.lcssa14392, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1725.lcssa1202414243.lcssa14412, ptr %_21.i.i, align 4
  store float %_0.i3414.lcssa1198914156, ptr %718, align 4
  store float %_0.i3788.lcssa1200514174, ptr %720, align 4
  store float %_0.i3410.lcssa1202914192, ptr %726, align 4
  store float %_0.i3784.lcssa1203014210, ptr %728, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014227, ptr %_21.i264.i, align 1, !dbg !41437
  store float %running.sroa.0.0.i1725.lcssa1202414244, ptr %_21.i.i, align 1, !dbg !41455
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_236.i, i64 noundef %_5.i1952, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !41940, !noalias !41941
  unreachable, !dbg !41940

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3739: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i
  %_0.i2947 = fdiv float %_0.i3410, %_37.i.i, !dbg !41944
  %_0.i3409 = fsub float 1.000000e+00, %_0.i2947, !dbg !41946
  %_0.i3408 = fsub float %_0.i3409, %_0.i37849311, !dbg !41948
  %_4.i2963 = fmul float %_9.i.i, %_0.i3408, !dbg !41950
  %_0.i2964 = fadd float %_0.i37849311, %_4.i2963, !dbg !41950
  %_3.i.i4235.inv = fcmp ogt float %_0.i3409, %_0.i2964, !dbg !41952
  %_4.i.i4242.v = select i1 %_3.i.i4235.inv, float %_0.i3409, float %_0.i2964, !dbg !41952
  %766 = tail call noundef float @llvm.fabs.f32(float %_4.i.i4242.v), !dbg !41955
  %767 = fcmp uge float %766, 0x3BC79CA100000000, !dbg !41958
  %_0.i3784 = select i1 %767, float %_4.i.i4242.v, float 0.000000e+00, !dbg !41960
  %_0.i3407 = fsub float 1.000000e+00, %_0.i3784, !dbg !41961
  %_15.i1940 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %_236.i, !dbg !41963
  %_0.i3422 = load float, ptr %_15.i1940, align 4, !dbg !41965, !alias.scope !41967, !noalias !41914, !noundef !12
  store float %_0.i3588, ptr %_15.i1940, align 4, !dbg !41970, !alias.scope !41973, !noalias !41914
  %_0.i3273 = fmul float %_0.i3407, %_0.i3422, !dbg !41976
  %_6.i4010 = bitcast float %_0.i3422 to i32, !dbg !41978
  %_5.i4011 = and i32 %_6.i4010, %all.sroa.0.0.i, !dbg !41981
  %_8.i4012 = bitcast float %_0.i3273 to i32, !dbg !41982
  %_7.i4014 = and i32 %_9.i4026, %_8.i4012, !dbg !41984
  %_4.i4015 = or disjoint i32 %_7.i4014, %_5.i4011, !dbg !41981
  store i32 %_4.i4015, ptr %data.i5.i.i.i.i.i.i.i5248, align 4, !dbg !41985, !alias.scope !41987, !noalias !41990
  %exitcond11829.not = icmp eq i64 %751, %umin11828, !dbg !41491
  br i1 %exitcond11829.not, label %bb32.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3590, !dbg !41491

bb32.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3739, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit5218
  %running.sroa.0.0.i1725.lcssa1202414243 = phi float [ %running.sroa.0.0.i1725.lcssa1202414244, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit5218 ], [ %running.sroa.0.0.i1725, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3739 ]
  %running.sroa.0.0.i1756.lcssa1197014226 = phi float [ %running.sroa.0.0.i1756.lcssa1197014227, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit5218 ], [ %running.sroa.0.0.i1756, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3739 ]
  %_0.i3784.lcssa1203014209 = phi float [ %_0.i3784.lcssa1203014210, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit5218 ], [ %_0.i3784, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3739 ]
  %_0.i3410.lcssa1202914191 = phi float [ %_0.i3410.lcssa1202914192, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit5218 ], [ %_0.i3410, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3739 ]
  %_0.i3788.lcssa1200514173 = phi float [ %_0.i3788.lcssa1200514174, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit5218 ], [ %_0.i3788, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3739 ]
  %_0.i3414.lcssa1198914155 = phi float [ %_0.i3414.lcssa1198914156, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit5218 ], [ %_0.i3414, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3739 ]
  %running.sroa.0.0.i1725.lcssa92769421 = phi float [ %running.sroa.0.0.i1725.lcssa92769422, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit5218 ], [ %running.sroa.0.0.i1725, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3739 ]
  %storemerge.i1730.lcssa92499385 = phi i32 [ %storemerge.i1730.lcssa92499386, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit5218 ], [ %storemerge.i1730, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3739 ]
  %running.sroa.0.0.i1756.lcssa91649382 = phi float [ %running.sroa.0.0.i1756.lcssa91649383, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit5218 ], [ %running.sroa.0.0.i1756, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3739 ]
  %storemerge.i1761.lcssa91379346 = phi i32 [ %storemerge.i1761.lcssa91379347, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit5218 ], [ %storemerge.i1761, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3739 ]
  %_143.i = add i64 %..i5151, %ring_cursor.sroa.0.1.i9340, !dbg !41991
  %_234.not.i = icmp ult i64 %_143.i, %ring.i, !dbg !41992
  %768 = select i1 %_234.not.i, i64 0, i64 %ring.i, !dbg !41992
  %ring_cursor.sroa.0.2.i = sub nuw i64 %_143.i, %768, !dbg !41992
  %_145.i = add i64 %..i5151, %main_cursor.sroa.0.1.i9341, !dbg !41995
  %_245.not.i = icmp ult i64 %_145.i, %main.i, !dbg !41996
  %769 = select i1 %_245.not.i, i64 0, i64 %main.i, !dbg !41996
  %main_cursor.sroa.0.2.i = sub nuw i64 %_145.i, %769, !dbg !41996
  %_63.i = icmp ult i64 %_85.i, %..i5032, !dbg !40854
  br i1 %_63.i, label %bb20.i, label %bb19.i.bb15.i.loopexit_crit_edge, !dbg !40854

_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit: ; preds = %bb15.i.loopexit
  store float %history.i44.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40880
  store float %history.i44.i.sroa.7.0.lcssa, ptr %history.i44.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !40880
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40882
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !40882
  store i32 %storemerge.i1761.lcssa91379346.lcssa14352, ptr %_22.i265.i, align 4
  store float %running.sroa.0.0.i1756.lcssa1197014226.lcssa14372, ptr %_21.i264.i, align 4
  store i32 %storemerge.i1730.lcssa92499385.lcssa14391, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1725.lcssa1202414243.lcssa14411, ptr %_21.i.i, align 4
  %770 = trunc i64 %main_cursor.sroa.0.1.i.lcssa to i32, !dbg !41998
  %771 = trunc i64 %ring_cursor.sroa.0.1.i.lcssa to i32, !dbg !42000
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit, !dbg !42001

_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit, %bb6.i
  %ring_cursor.sroa.0.0.i.lcssa = phi i32 [ %_36.i, %bb6.i ], [ %771, %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit ], !dbg !40822
  %main_cursor.sroa.0.0.i.lcssa = phi i32 [ %_35.i, %bb6.i ], [ %770, %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit ], !dbg !40819
  %772 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 72, !dbg !42001
  %left_prefix.i = load float, ptr %772, align 8, !dbg !42001, !noalias !40826, !noundef !12
  %773 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 76, !dbg !42002
  %left_phase.i = load i32, ptr %773, align 4, !dbg !42002, !noalias !40826, !noundef !12
  %774 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 72, !dbg !42003
  %right_prefix.i = load float, ptr %774, align 8, !dbg !42003, !noalias !40826, !noundef !12
  %775 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 76, !dbg !42004
  %right_phase.i = load i32, ptr %775, align 4, !dbg !42004, !noalias !40826, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_right.i), !dbg !42005, !noalias !40826
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i), !dbg !42006, !noalias !40826
  %776 = getelementptr inbounds nuw i8, ptr %self, i64 216, !dbg !42007
  %_260.0.i = load ptr, ptr %776, align 8, !dbg !42007, !alias.scope !40793, !noalias !42008, !nonnull !12, !noundef !12
  %777 = getelementptr inbounds nuw i8, ptr %self, i64 224, !dbg !42007
  %_260.1.i = load i64, ptr %777, align 8, !dbg !42007, !alias.scope !40793, !noalias !42008, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !42009), !dbg !42012
  %_4.not.i3732 = icmp eq i64 %_260.1.i, 0, !dbg !42013
  br i1 %_4.not.i3732, label %panic.i3734, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3735, !dbg !42013

panic.i3734:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !42013, !noalias !42015
  unreachable, !dbg !42013

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3735: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit
  store float %left_prefix.i, ptr %_260.0.i, align 4, !dbg !42013, !alias.scope !42009, !noalias !40797
  %_261.0.i = load ptr, ptr %68, align 8, !dbg !42016, !alias.scope !40793, !noalias !42008, !nonnull !12, !noundef !12
  %_261.1.i = load i64, ptr %69, align 8, !dbg !42016, !alias.scope !40793, !noalias !42008, !noundef !12
  %778 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i), !dbg !42017
  br i1 %778, label %bb2.i5269, label %bb6.i5260, !dbg !42017

bb6.i5260:                                        ; preds = %bb2.i5269, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3735
  %end_or_len.idx.i5261 = shl nuw nsw i64 %_261.1.i, 2, !dbg !42021
  %end_or_len.i5262 = getelementptr inbounds nuw i8, ptr %_261.0.i, i64 %end_or_len.idx.i5261, !dbg !42021
  %_293.i5263 = icmp eq i64 %_261.1.i, 0, !dbg !42025
  br i1 %_293.i5263, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5275, label %bb10.i5264, !dbg !42028

bb2.i5269:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3735
  %bytes1.sroa.0.0.zext.i5270 = and i32 %left_phase.i, 255, !dbg !42029
  %bytes1.sroa.0.0.isplat.i5271 = mul nuw i32 %bytes1.sroa.0.0.zext.i5270, 16843009, !dbg !42029
  %_5.i5272 = icmp eq i32 %left_phase.i, %bytes1.sroa.0.0.isplat.i5271, !dbg !42030
  br i1 %_5.i5272, label %bb3.i5273, label %bb6.i5260, !dbg !42030

bb3.i5273:                                        ; preds = %bb2.i5269
  %bytes.sroa.0.0.extract.trunc.i5274 = trunc i32 %left_phase.i to i8, !dbg !42031
  %779 = shl nuw nsw i64 %_261.1.i, 2, !dbg !42033
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_261.0.i, i8 %bytes.sroa.0.0.extract.trunc.i5274, i64 %779, i1 false), !dbg !42033, !alias.scope !42034, !noalias !40797
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5275, !dbg !42037

bb10.i5264:                                       ; preds = %bb6.i5260, %bb10.i5264
  %iter.sroa.0.04.i5265 = phi ptr [ %_38.i5266, %bb10.i5264 ], [ %_261.0.i, %bb6.i5260 ]
  %_38.i5266 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i5265, i64 4, !dbg !42038
  store i32 %left_phase.i, ptr %iter.sroa.0.04.i5265, align 4, !dbg !42040, !alias.scope !42034, !noalias !40797
  %_29.i5267 = icmp eq ptr %_38.i5266, %end_or_len.i5262, !dbg !42025
  br i1 %_29.i5267, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5275, label %bb10.i5264, !dbg !42028

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5275: ; preds = %bb10.i5264, %bb6.i5260, %bb3.i5273
  %780 = getelementptr inbounds nuw i8, ptr %self, i64 416, !dbg !42041
  %_262.0.i = load ptr, ptr %780, align 8, !dbg !42041, !alias.scope !40795, !noalias !42042, !nonnull !12, !noundef !12
  %781 = getelementptr inbounds nuw i8, ptr %self, i64 424, !dbg !42041
  %_262.1.i = load i64, ptr %781, align 8, !dbg !42041, !alias.scope !40795, !noalias !42042, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !42043), !dbg !42046
  %_4.not.i3728 = icmp eq i64 %_262.1.i, 0, !dbg !42047
  br i1 %_4.not.i3728, label %panic.i3730, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3731, !dbg !42047

panic.i3730:                                      ; preds = %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5275
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !42047, !noalias !42049
  unreachable, !dbg !42047

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3731: ; preds = %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5275
  store float %right_prefix.i, ptr %_262.0.i, align 4, !dbg !42047, !alias.scope !42043, !noalias !40797
  %_263.0.i = load ptr, ptr %77, align 8, !dbg !42050, !alias.scope !40795, !noalias !42042, !nonnull !12, !noundef !12
  %_263.1.i = load i64, ptr %78, align 8, !dbg !42050, !alias.scope !40795, !noalias !42042, !noundef !12
  %782 = tail call i1 @llvm.is.constant.i32(i32 %right_phase.i), !dbg !42051
  br i1 %782, label %bb2.i5285, label %bb6.i5276, !dbg !42051

bb6.i5276:                                        ; preds = %bb2.i5285, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3731
  %end_or_len.idx.i5277 = shl nuw nsw i64 %_263.1.i, 2, !dbg !42054
  %end_or_len.i5278 = getelementptr inbounds nuw i8, ptr %_263.0.i, i64 %end_or_len.idx.i5277, !dbg !42054
  %_293.i5279 = icmp eq i64 %_263.1.i, 0, !dbg !42058
  br i1 %_293.i5279, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5291, label %bb10.i5280, !dbg !42061

bb2.i5285:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3731
  %bytes1.sroa.0.0.zext.i5286 = and i32 %right_phase.i, 255, !dbg !42062
  %bytes1.sroa.0.0.isplat.i5287 = mul nuw i32 %bytes1.sroa.0.0.zext.i5286, 16843009, !dbg !42062
  %_5.i5288 = icmp eq i32 %right_phase.i, %bytes1.sroa.0.0.isplat.i5287, !dbg !42063
  br i1 %_5.i5288, label %bb3.i5289, label %bb6.i5276, !dbg !42063

bb3.i5289:                                        ; preds = %bb2.i5285
  %bytes.sroa.0.0.extract.trunc.i5290 = trunc i32 %right_phase.i to i8, !dbg !42064
  %783 = shl nuw nsw i64 %_263.1.i, 2, !dbg !42066
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_263.0.i, i8 %bytes.sroa.0.0.extract.trunc.i5290, i64 %783, i1 false), !dbg !42066, !alias.scope !42067, !noalias !40797
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5291, !dbg !42070

bb10.i5280:                                       ; preds = %bb6.i5276, %bb10.i5280
  %iter.sroa.0.04.i5281 = phi ptr [ %_38.i5282, %bb10.i5280 ], [ %_263.0.i, %bb6.i5276 ]
  %_38.i5282 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i5281, i64 4, !dbg !42071
  store i32 %right_phase.i, ptr %iter.sroa.0.04.i5281, align 4, !dbg !42073, !alias.scope !42067, !noalias !40797
  %_29.i5283 = icmp eq ptr %_38.i5282, %end_or_len.i5278, !dbg !42058
  br i1 %_29.i5283, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5291, label %bb10.i5280, !dbg !42061

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5291: ; preds = %bb10.i5280, %bb6.i5276, %bb3.i5289
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %hot_left.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33) #31, !dbg !42074
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %hot_right.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34) #31, !dbg !42075
  store i32 %main_cursor.sroa.0.0.i.lcssa, ptr %_35, align 4, !dbg !41998, !alias.scope !40797, !noalias !40821
  store i32 %ring_cursor.sroa.0.0.i.lcssa, ptr %673, align 4, !dbg !42000, !alias.scope !40797, !noalias !40821
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i), !dbg !42076, !noalias !40826
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i), !dbg !42077, !noalias !40826
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockfEB2_.exit, !dbg !40790

_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockfEB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit, %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit, %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5027, %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit5291
  br i1 %quiet.sroa.0.05483, label %bb28, label %bb40, !dbg !42078

bb22:                                             ; preds = %bb20
  tail call void @llvm.experimental.noalias.scope.decl(metadata !42079), !dbg !42082
  %784 = getelementptr inbounds nuw i8, ptr %self, i64 248, !dbg !42083
  %_40.0.i = load ptr, ptr %784, align 8, !dbg !42083, !alias.scope !42079, !nonnull !12, !noundef !12
  %785 = getelementptr inbounds nuw i8, ptr %self, i64 256, !dbg !42083
  %_40.1.i = load i64, ptr %785, align 8, !dbg !42083, !alias.scope !42079, !noundef !12
  %786 = getelementptr inbounds nuw i8, ptr %self, i64 312, !dbg !42085
  %_41.0.i = load ptr, ptr %786, align 8, !dbg !42085, !alias.scope !42079, !nonnull !12, !noundef !12
  %787 = getelementptr inbounds nuw i8, ptr %self, i64 320, !dbg !42085
  %_41.1.i = load i64, ptr %787, align 8, !dbg !42085, !alias.scope !42079, !noundef !12
  %..i.i.i.i = tail call noundef i64 @llvm.umin.i64(i64 %_41.1.i, i64 %_40.1.i), !dbg !42086
  %_2.i6.not.i = icmp eq i64 %..i.i.i.i, 0, !dbg !42092
  br i1 %_2.i6.not.i, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb4.i5292, !dbg !42092

bb4.i5292:                                        ; preds = %bb22, %bb6.i5294
  %iter.sroa.8.07.i = phi i64 [ %788, %bb6.i5294 ], [ 0, %bb22 ]
  %_3.i1.i.i = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i, i64 %iter.sroa.8.07.i, !dbg !42095
  %_14.i5293 = load i32, ptr %_3.i1.i.i, align 4, !dbg !42098, !noalias !42079, !noundef !12
  %_20.i = icmp eq i32 %_14.i5293, 0, !dbg !42099
  br i1 %_20.i, label %panic.i5303, label %bb6.i5294, !dbg !42099

bb6.i5294:                                        ; preds = %bb4.i5292
  %_3.i.i.i5295 = getelementptr inbounds nuw i32, ptr %_40.0.i, i64 %iter.sroa.8.07.i, !dbg !42100
  %788 = add nuw i64 %iter.sroa.8.07.i, 1, !dbg !42103
  %window.i5296 = zext i32 %_14.i5293 to i64, !dbg !42098
  %_18.i5297 = load i32, ptr %_3.i.i.i5295, align 4, !dbg !42104, !noalias !42079, !noundef !12
  %_17.i5298 = zext i32 %_18.i5297 to i64, !dbg !42104
  %_19.i5299 = urem i64 %frames, %window.i5296, !dbg !42099
  %_16.i5300 = add nuw nsw i64 %_19.i5299, %_17.i5298, !dbg !42105
  %_15.i5301 = urem i64 %_16.i5300, %window.i5296, !dbg !42106
  %789 = trunc nuw i64 %_15.i5301 to i32, !dbg !42107
  store i32 %789, ptr %_3.i.i.i5295, align 4, !dbg !42107, !noalias !42079
  %exitcond.not.i = icmp eq i64 %788, %..i.i.i.i, !dbg !42092
  br i1 %exitcond.not.i, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb4.i5292, !dbg !42092

panic.i5303:                                      ; preds = %bb4.i5292
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bac57976a2bdbfad4a3a85d5d1c7648c) #30, !dbg !42099, !noalias !42079
  unreachable, !dbg !42099

_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit: ; preds = %bb6.i5294, %bb22
  tail call void @llvm.experimental.noalias.scope.decl(metadata !42108), !dbg !42111
  %790 = getelementptr inbounds nuw i8, ptr %self, i64 448, !dbg !42112
  %_40.0.i5304 = load ptr, ptr %790, align 8, !dbg !42112, !alias.scope !42108, !nonnull !12, !noundef !12
  %791 = getelementptr inbounds nuw i8, ptr %self, i64 456, !dbg !42112
  %_40.1.i5305 = load i64, ptr %791, align 8, !dbg !42112, !alias.scope !42108, !noundef !12
  %792 = getelementptr inbounds nuw i8, ptr %self, i64 512, !dbg !42114
  %_41.0.i5306 = load ptr, ptr %792, align 8, !dbg !42114, !alias.scope !42108, !nonnull !12, !noundef !12
  %793 = getelementptr inbounds nuw i8, ptr %self, i64 520, !dbg !42114
  %_41.1.i5307 = load i64, ptr %793, align 8, !dbg !42114, !alias.scope !42108, !noundef !12
  %..i.i.i.i5308 = tail call noundef i64 @llvm.umin.i64(i64 %_41.1.i5307, i64 %_40.1.i5305), !dbg !42115
  %_2.i6.not.i5309 = icmp eq i64 %..i.i.i.i5308, 0, !dbg !42121
  br i1 %_2.i6.not.i5309, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit5326, label %bb4.i5310, !dbg !42121

bb4.i5310:                                        ; preds = %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, %bb6.i5315
  %iter.sroa.8.07.i5311 = phi i64 [ %794, %bb6.i5315 ], [ 0, %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit ]
  %_3.i1.i.i5312 = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i5306, i64 %iter.sroa.8.07.i5311, !dbg !42124
  %_14.i5313 = load i32, ptr %_3.i1.i.i5312, align 4, !dbg !42127, !noalias !42108, !noundef !12
  %_20.i5314 = icmp eq i32 %_14.i5313, 0, !dbg !42128
  br i1 %_20.i5314, label %panic.i5325, label %bb6.i5315, !dbg !42128

bb6.i5315:                                        ; preds = %bb4.i5310
  %_3.i.i.i5316 = getelementptr inbounds nuw i32, ptr %_40.0.i5304, i64 %iter.sroa.8.07.i5311, !dbg !42129
  %794 = add nuw i64 %iter.sroa.8.07.i5311, 1, !dbg !42132
  %window.i5317 = zext i32 %_14.i5313 to i64, !dbg !42127
  %_18.i5318 = load i32, ptr %_3.i.i.i5316, align 4, !dbg !42133, !noalias !42108, !noundef !12
  %_17.i5319 = zext i32 %_18.i5318 to i64, !dbg !42133
  %_19.i5320 = urem i64 %frames, %window.i5317, !dbg !42128
  %_16.i5321 = add nuw nsw i64 %_19.i5320, %_17.i5319, !dbg !42134
  %_15.i5322 = urem i64 %_16.i5321, %window.i5317, !dbg !42135
  %795 = trunc nuw i64 %_15.i5322 to i32, !dbg !42136
  store i32 %795, ptr %_3.i.i.i5316, align 4, !dbg !42136, !noalias !42108
  %exitcond.not.i5323 = icmp eq i64 %794, %..i.i.i.i5308, !dbg !42121
  br i1 %exitcond.not.i5323, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit5326, label %bb4.i5310, !dbg !42121

panic.i5325:                                      ; preds = %bb4.i5310
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bac57976a2bdbfad4a3a85d5d1c7648c) #30, !dbg !42128, !noalias !42108
  unreachable, !dbg !42128

_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit5326: ; preds = %bb6.i5315, %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit
  %796 = getelementptr inbounds nuw i8, ptr %self, i64 544, !dbg !42137
  %_29.val = load i64, ptr %796, align 8, !dbg !42137
  %797 = getelementptr inbounds nuw i8, ptr %self, i64 552, !dbg !42137
  %_29.val4388 = load i64, ptr %797, align 8, !dbg !42137, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !42138), !dbg !42137
  %_10.i5327 = icmp eq i64 %_29.val4388, 0, !dbg !42141
  br i1 %_10.i5327, label %panic.i5339, label %bb1.i5328, !dbg !42141

bb1.i5328:                                        ; preds = %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit5326
  %_28 = getelementptr inbounds nuw i8, ptr %self, i64 560, !dbg !42143
  %_7.i = load i32, ptr %_28, align 4, !dbg !42144, !alias.scope !42138, !noundef !12
  %_6.i5329 = zext i32 %_7.i to i64, !dbg !42144
  %_8.i5330 = urem i64 %frames, %_29.val4388, !dbg !42141
  %_5.i5331 = add nuw nsw i64 %_8.i5330, %_6.i5329, !dbg !42145
  %_4.i5332 = urem i64 %_5.i5331, %_29.val4388, !dbg !42146
  %798 = trunc i64 %_4.i5332 to i32, !dbg !42147
  store i32 %798, ptr %_28, align 4, !dbg !42147, !alias.scope !42138
  %_17.i5333 = icmp eq i64 %_29.val, 0, !dbg !42148
  br i1 %_17.i5333, label %panic2.i, label %_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit, !dbg !42148

panic.i5339:                                      ; preds = %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit5326
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f0ee36f67d9a332211aa5518dd2ebfd5) #30, !dbg !42141, !noalias !42138
  unreachable, !dbg !42141

panic2.i:                                         ; preds = %bb1.i5328
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_33d4d33e0a850133578789055882dcf9) #30, !dbg !42148, !noalias !42138
  unreachable, !dbg !42148

_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit: ; preds = %bb1.i5328
  %799 = getelementptr inbounds nuw i8, ptr %self, i64 564, !dbg !42149
  %_14.i5335 = load i32, ptr %799, align 4, !dbg !42149, !alias.scope !42138, !noundef !12
  %_13.i5336 = zext i32 %_14.i5335 to i64, !dbg !42149
  %_15.i5337 = urem i64 %frames, %_29.val, !dbg !42148
  %_12.i = add nuw nsw i64 %_15.i5337, %_13.i5336, !dbg !42150
  %_11.i5338 = urem i64 %_12.i, %_29.val, !dbg !42151
  %800 = trunc i64 %_11.i5338 to i32, !dbg !42152
  store i32 %800, ptr %799, align 4, !dbg !42152, !alias.scope !42138
  br label %bb42, !dbg !42153

bb28:                                             ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockfEB2_.exit
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_37 = tail call noundef zeroext i1 @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #31, !dbg !42154
  br i1 %_37, label %bb30, label %bb40, !dbg !42155

bb30:                                             ; preds = %bb28
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_39 = tail call noundef zeroext i1 @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #31, !dbg !42156
  br i1 %_39, label %bb32, label %bb40, !dbg !42157

bb32:                                             ; preds = %bb30
  %_80.not = icmp ugt i64 %frames, %left_io.1
  br i1 %_80.not, label %bb51, label %bb1.i5340, !dbg !42158, !prof !165

bb51:                                             ; preds = %bb32
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %frames, i64 noundef %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5792a3affd2091ba045d8adc49b05663) #30, !dbg !42166
  unreachable, !dbg !42166

bb1.i5340:                                        ; preds = %bb32, %bb10.i5354
  %iter.sroa.6.0.i5341 = phi i64 [ %len.i.i.i.i5346, %bb10.i5354 ], [ %frames, %bb32 ], !dbg !42167
  %iter.sroa.0.0.i5342 = phi ptr [ %data.i.i.i.i5345, %bb10.i5354 ], [ %left_io.0, %bb32 ], !dbg !42167
  %801 = icmp eq i64 %iter.sroa.6.0.i5341, 0, !dbg !42169
  br i1 %801, label %bb34, label %bb11.preheader.i5343, !dbg !42169

bb11.preheader.i5343:                             ; preds = %bb1.i5340
  %..i.i.i5344 = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i5341, i64 32), !dbg !42171
  %_18.idx.i5347 = shl nuw nsw i64 %..i.i.i5344, 2, !dbg !42174
  %_18.i5348 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i5342, i64 %_18.idx.i5347, !dbg !42174
  br label %bb11.i5349, !dbg !42179

bb11.i5349:                                       ; preds = %bb11.i5349, %bb11.preheader.i5343
  %iter1.sroa.0.014.i5350 = phi ptr [ %_31.i5352, %bb11.i5349 ], [ %iter.sroa.0.0.i5342, %bb11.preheader.i5343 ]
  %bits.sroa.0.013.i5351 = phi i32 [ %802, %bb11.i5349 ], [ 0, %bb11.preheader.i5343 ]
  %_31.i5352 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i5350, i64 4, !dbg !42181
  %_134.i5353 = load i32, ptr %iter1.sroa.0.014.i5350, align 4, !dbg !42183, !alias.scope !42184, !noundef !12
  %802 = or i32 %_134.i5353, %bits.sroa.0.013.i5351, !dbg !42187
  %_25.i = icmp eq ptr %_31.i5352, %_18.i5348, !dbg !42188
  br i1 %_25.i, label %bb10.i5354, label %bb11.i5349, !dbg !42179

bb10.i5354:                                       ; preds = %bb11.i5349
  %data.i.i.i.i5345 = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i5342, i64 %..i.i.i5344, !dbg !42190
  %len.i.i.i.i5346 = sub nuw nsw i64 %iter.sroa.6.0.i5341, %..i.i.i5344, !dbg !42195
  %803 = icmp eq i32 %802, 0, !dbg !42196
  br i1 %803, label %bb1.i5340, label %bb40, !dbg !42196

bb34:                                             ; preds = %bb1.i5340
  %_88.not = icmp ugt i64 %frames, %right_io.1, !dbg !42197
  br i1 %_88.not, label %bb54, label %bb1.i5367, !dbg !42197, !prof !1406

bb40:                                             ; preds = %bb10.i5354, %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockfEB2_.exit, %bb28, %bb30, %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit5383
  %_36.sroa.0.0 = phi i8 [ %832, %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit5383 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockfEB2_.exit ], [ 0, %bb30 ], [ 0, %bb28 ], [ 0, %bb10.i5354 ], !dbg !42203
  store i8 %_36.sroa.0.0, ptr %38, align 4, !dbg !42204
  %804 = load i8, ptr %2, align 8, !dbg !42205, !range !17, !noundef !12
  store i8 %804, ptr %0, align 1, !dbg !42206
  call void @llvm.lifetime.start.p0(ptr nonnull %shape), !dbg !42207
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %shape, ptr noundef nonnull align 8 dereferenceable(24) %_32, i64 24, i1 false), !dbg !42208
  %805 = getelementptr inbounds nuw i8, ptr %self, i64 72, !dbg !42209
  %806 = load i32, ptr %805, align 8, !dbg !42209, !noundef !12
  %_53 = getelementptr inbounds nuw i8, ptr %self, i64 568, !dbg !42211
  %807 = getelementptr inbounds nuw i8, ptr %self, i64 104, !dbg !42218
  %_99.0 = load ptr, ptr %807, align 8, !dbg !42218, !nonnull !12, !noundef !12
  %808 = getelementptr inbounds nuw i8, ptr %self, i64 112, !dbg !42218
  %_99.1 = load i64, ptr %808, align 8, !dbg !42218, !noundef !12
  %809 = getelementptr inbounds nuw i8, ptr %self, i64 120, !dbg !42218
  %_100.0 = load ptr, ptr %809, align 8, !dbg !42218, !nonnull !12, !noundef !12
  %810 = getelementptr inbounds nuw i8, ptr %self, i64 128, !dbg !42218
  %_100.1 = load i64, ptr %810, align 8, !dbg !42218, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !42219), !dbg !42222
  tail call void @llvm.experimental.noalias.scope.decl(metadata !42223), !dbg !42222
  tail call void @llvm.experimental.noalias.scope.decl(metadata !42225), !dbg !42222
  %_22.not.i1463.i = icmp eq i64 %left_io.1, 0, !dbg !42227
  br i1 %_22.not.i1463.i, label %bb6.i.preheader.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i, !dbg !42227

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i: ; preds = %bb40, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i
  %ok.sroa.0.0.i1366.i = phi i32 [ %_0.i33.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i ], [ -1, %bb40 ]
  %iter.sroa.0.0.i1265.i = phi ptr [ %_27.i16.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i ], [ %left_io.0, %bb40 ]
  %iter.sroa.5.0.i1164.i = phi i64 [ %_28.i17.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i ], [ %left_io.1, %bb40 ]
  %_27.i16.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i1265.i, i64 4, !dbg !42240
  %_28.i17.i = add nsw i64 %iter.sroa.5.0.i1164.i, -1, !dbg !42247
  %_0.i28.i = load float, ptr %iter.sroa.0.0.i1265.i, align 4, !dbg !42248, !alias.scope !42251, !noalias !42254, !noundef !12
  %811 = tail call noundef float @llvm.fabs.f32(float %_0.i28.i), !dbg !42256
  %_3.i.i5356 = fcmp olt float %811, 0x46293E5940000000, !dbg !42259
  %_0.i33.i = select i1 %_3.i.i5356, i32 %ok.sroa.0.0.i1366.i, i32 0, !dbg !42262
  %_22.not.i14.i = icmp eq i64 %_28.i17.i, 0, !dbg !42227
  br i1 %_22.not.i14.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdvPQf9CMsz3_17true_peak_limiter.exit25.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i, !dbg !42227

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdvPQf9CMsz3_17true_peak_limiter.exit25.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i
  %812 = icmp eq i32 %_0.i33.i, -1, !dbg !42265
  br i1 %812, label %bb6.i.preheader.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i, !dbg !42268

bb6.i.preheader.i:                                ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdvPQf9CMsz3_17true_peak_limiter.exit25.i, %bb40
  %_22.not.i67.i = icmp eq i64 %right_io.1, 0, !dbg !42269
  br i1 %_22.not.i67.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockfNCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i, !dbg !42269

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i: ; preds = %bb6.i.preheader.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i
  %ok.sroa.0.0.i70.i = phi i32 [ %_0.i34.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i ], [ -1, %bb6.i.preheader.i ]
  %iter.sroa.0.0.i69.i = phi ptr [ %_27.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i ], [ %right_io.0, %bb6.i.preheader.i ]
  %iter.sroa.5.0.i68.i = phi i64 [ %_28.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i ], [ %right_io.1, %bb6.i.preheader.i ]
  %_27.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i69.i, i64 4, !dbg !42273
  %_28.i.i = add nsw i64 %iter.sroa.5.0.i68.i, -1, !dbg !42276
  %_0.i30.i = load float, ptr %iter.sroa.0.0.i69.i, align 4, !dbg !42277, !alias.scope !42279, !noalias !42282, !noundef !12
  %813 = tail call noundef float @llvm.fabs.f32(float %_0.i30.i), !dbg !42283
  %_3.i26.i = fcmp olt float %813, 0x46293E5940000000, !dbg !42285
  %_0.i34.i = select i1 %_3.i26.i, i32 %ok.sroa.0.0.i70.i, i32 0, !dbg !42287
  %_22.not.i.i = icmp eq i64 %_28.i.i, 0, !dbg !42269
  br i1 %_22.not.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i, !dbg !42269

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdvPQf9CMsz3_17true_peak_limiter.exit.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i
  %814 = icmp eq i32 %_0.i34.i, -1, !dbg !42289
  br i1 %814, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockfNCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit, label %bb7.i5366, !dbg !42291

bb7.i5366:                                        ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdvPQf9CMsz3_17true_peak_limiter.exit.i
  br i1 %_22.not.i1463.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i, !dbg !42292

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i: ; preds = %bb7.i5366, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdvPQf9CMsz3_17true_peak_limiter.exit25.i
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i, !dbg !42292

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i
  %ok.sroa.0.017.i.i = phi i32 [ %_0.i7.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ -1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i ]
  %iter.sroa.0.016.i.i = phi ptr [ %_45.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ %left_io.0, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i ]
  %iter.sroa.5.015.i.i = phi i64 [ %_46.i.i5357, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ %left_io.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i ]
  %_45.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.016.i.i, i64 4, !dbg !42303
  %_46.i.i5357 = add nsw i64 %iter.sroa.5.015.i.i, -1, !dbg !42310
  %_0.i.i.i5358 = load float, ptr %iter.sroa.0.016.i.i, align 4, !dbg !42311, !alias.scope !42314, !noalias !42254, !noundef !12
  %815 = tail call noundef float @llvm.fabs.f32(float %_0.i.i.i5358), !dbg !42319
  %_3.i.i.i5359 = fcmp olt float %815, 0x46293E5940000000, !dbg !42322
  %_0.i7.i.i = select i1 %_3.i.i.i5359, i32 %ok.sroa.0.017.i.i, i32 0, !dbg !42324
  %_40.not.i.i = icmp eq i64 %_46.i.i5357, 0, !dbg !42292
  br i1 %_40.not.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i, !dbg !42292

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i
  %816 = and i32 %_0.i7.i.i, 1065353216, !dbg !42326
  %817 = icmp ne i32 %816, 1065353216, !dbg !42329
  %818 = zext i1 %817 to i32, !dbg !42329
  %_40.not14.i40.i = icmp eq i64 %right_io.1, 0, !dbg !42333
  br i1 %_40.not14.i40.i, label %bb14.i.preheader.thread.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i, !dbg !42333

bb14.i.preheader.thread.i:                        ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit.i
  %819 = getelementptr inbounds nuw i8, ptr %self, i64 576, !dbg !42337
  store i32 %818, ptr %819, align 8, !dbg !42337, !alias.scope !42225, !noalias !42338
  %_1481.i = load i64, ptr %_53, align 8, !dbg !42339, !alias.scope !42225, !noalias !42338, !noundef !12
  %820 = tail call i64 @llvm.uadd.sat.i64(i64 %_1481.i, i64 1), !dbg !42340
  store i64 %820, ptr %_53, align 8, !dbg !42343, !alias.scope !42225, !noalias !42338
  br label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit60.i, !dbg !42344

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i: ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit.i, %bb7.i5366
  %ok.sroa.0.0.lcssa.i76.i = phi i32 [ %818, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit.i ], [ 0, %bb7.i5366 ]
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.i, !dbg !42333

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i
  %ok.sroa.0.017.i42.i = phi i32 [ %_0.i7.i49.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.i ], [ -1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i ]
  %iter.sroa.0.016.i43.i = phi ptr [ %_45.i45.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.i ], [ %right_io.0, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i ]
  %iter.sroa.5.015.i44.i = phi i64 [ %_46.i46.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.i ], [ %right_io.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i ]
  %_45.i45.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.016.i43.i, i64 4, !dbg !42349
  %_46.i46.i = add nsw i64 %iter.sroa.5.015.i44.i, -1, !dbg !42352
  %_0.i.i47.i = load float, ptr %iter.sroa.0.016.i43.i, align 4, !dbg !42353, !alias.scope !42355, !noalias !42282, !noundef !12
  %821 = tail call noundef float @llvm.fabs.f32(float %_0.i.i47.i), !dbg !42360
  %_3.i.i48.i = fcmp olt float %821, 0x46293E5940000000, !dbg !42362
  %_0.i7.i49.i = select i1 %_3.i.i48.i, i32 %ok.sroa.0.017.i42.i, i32 0, !dbg !42364
  %_40.not.i50.i = icmp eq i64 %_46.i46.i, 0, !dbg !42333
  br i1 %_40.not.i50.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit53.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.i, !dbg !42333

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit53.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.i
  %822 = and i32 %_0.i7.i49.i, 1065353216, !dbg !42366
  %823 = icmp ne i32 %822, 1065353216, !dbg !42368
  %824 = zext i1 %823 to i32, !dbg !42368
  %825 = or i32 %ok.sroa.0.0.lcssa.i76.i, %824, !dbg !42337
  %826 = getelementptr inbounds nuw i8, ptr %self, i64 576, !dbg !42337
  store i32 %825, ptr %826, align 8, !dbg !42337, !alias.scope !42225, !noalias !42338
  %_14.i5360 = load i64, ptr %_53, align 8, !dbg !42339, !alias.scope !42225, !noalias !42338, !noundef !12
  %827 = tail call i64 @llvm.uadd.sat.i64(i64 %_14.i5360, i64 1), !dbg !42340
  store i64 %827, ptr %_53, align 8, !dbg !42343, !alias.scope !42225, !noalias !42338
  br i1 %_22.not.i1463.i, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit60.i, label %bb14.i.preheader.i, !dbg !42369

bb14.i.preheader.i:                               ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit53.i
  %.idx.i.i = shl nuw nsw i64 %left_io.1, 2, !dbg !42373
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %left_io.0, i8 0, i64 %.idx.i.i, i1 false), !dbg !42377, !alias.scope !42378, !noalias !42254
  br label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit60.i, !dbg !42344

_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit60.i: ; preds = %bb14.i.preheader.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit53.i, %bb14.i.preheader.thread.i
  %left.1.sink.i = phi i64 [ %left_io.1, %bb14.i.preheader.thread.i ], [ %right_io.1, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit53.i ], [ %right_io.1, %bb14.i.preheader.i ]
  %left.0.sink.i = phi ptr [ %left_io.0, %bb14.i.preheader.thread.i ], [ %right_io.0, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit53.i ], [ %right_io.0, %bb14.i.preheader.i ]
  %.idx.i85.i = shl nuw nsw i64 %left.1.sink.i, 2, !dbg !42381
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %left.0.sink.i, i8 0, i64 %.idx.i85.i, i1 false), !dbg !42387, !alias.scope !42388, !noalias !42389
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 8 dereferenceable(200) %_33, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_99.0, i64 noundef %_99.1, i32 noundef %806) #31, !dbg !42390, !noalias !42393
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 8 dereferenceable(200) %_34, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_100.0, i64 noundef %_100.1, i32 noundef %806) #31, !dbg !42396, !noalias !42393
  store i32 0, ptr %_35, align 4, !dbg !42397, !noalias !42393
  %828 = getelementptr inbounds nuw i8, ptr %self, i64 564, !dbg !42397
  store i32 0, ptr %828, align 4, !dbg !42397, !noalias !42393
  br label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockfNCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit, !dbg !42398

_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockfNCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit: ; preds = %bb6.i.preheader.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdvPQf9CMsz3_17true_peak_limiter.exit.i, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit60.i
  call void @llvm.lifetime.end.p0(ptr nonnull %shape), !dbg !42399
  br label %bb42, !dbg !42153

bb54:                                             ; preds = %bb34
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %frames, i64 noundef %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e8e5016f8f28771294ff88b89dfac103) #30, !dbg !42400
  unreachable, !dbg !42400

bb1.i5367:                                        ; preds = %bb34, %bb10.i5382
  %iter.sroa.6.0.i5368 = phi i64 [ %len.i.i.i.i5373, %bb10.i5382 ], [ %frames, %bb34 ], !dbg !42401
  %iter.sroa.0.0.i5369 = phi ptr [ %data.i.i.i.i5372, %bb10.i5382 ], [ %right_io.0, %bb34 ], !dbg !42401
  %829 = icmp eq i64 %iter.sroa.6.0.i5368, 0, !dbg !42403
  br i1 %829, label %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit5383, label %bb11.preheader.i5370, !dbg !42403

bb11.preheader.i5370:                             ; preds = %bb1.i5367
  %..i.i.i5371 = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i5368, i64 32), !dbg !42405
  %_18.idx.i5374 = shl nuw nsw i64 %..i.i.i5371, 2, !dbg !42408
  %_18.i5375 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i5369, i64 %_18.idx.i5374, !dbg !42408
  br label %bb11.i5376, !dbg !42413

bb11.i5376:                                       ; preds = %bb11.i5376, %bb11.preheader.i5370
  %iter1.sroa.0.014.i5377 = phi ptr [ %_31.i5379, %bb11.i5376 ], [ %iter.sroa.0.0.i5369, %bb11.preheader.i5370 ]
  %bits.sroa.0.013.i5378 = phi i32 [ %830, %bb11.i5376 ], [ 0, %bb11.preheader.i5370 ]
  %_31.i5379 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i5377, i64 4, !dbg !42415
  %_134.i5380 = load i32, ptr %iter1.sroa.0.014.i5377, align 4, !dbg !42417, !alias.scope !42418, !noundef !12
  %830 = or i32 %_134.i5380, %bits.sroa.0.013.i5378, !dbg !42421
  %_25.i5381 = icmp eq ptr %_31.i5379, %_18.i5375, !dbg !42422
  br i1 %_25.i5381, label %bb10.i5382, label %bb11.i5376, !dbg !42413

bb10.i5382:                                       ; preds = %bb11.i5376
  %data.i.i.i.i5372 = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i5369, i64 %..i.i.i5371, !dbg !42424
  %len.i.i.i.i5373 = sub nuw nsw i64 %iter.sroa.6.0.i5368, %..i.i.i5371, !dbg !42429
  %831 = icmp eq i32 %830, 0, !dbg !42430
  br i1 %831, label %bb1.i5367, label %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit5383, !dbg !42430

_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit5383: ; preds = %bb1.i5367, %bb10.i5382
  %832 = zext i1 %829 to i8, !dbg !42204
  br label %bb40, !dbg !42078

bb42:                                             ; preds = %_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockfNCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit
  ret void, !dbg !42153
}
