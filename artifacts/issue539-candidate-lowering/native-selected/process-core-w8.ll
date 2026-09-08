define internal fastcc void @_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13process_blockB5_(ptr noalias noundef nonnull align 32 dereferenceable(2176) %self, ptr noalias noundef nonnull align 4 captures(address) %left_io.0, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef nonnull align 4 captures(address) %right_io.0, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 4294967296) %frames) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !26435 {
start:
  %peaks_right.i1015 = alloca [1024 x i8], align 4
  %peaks_left.i1016 = alloca [1024 x i8], align 4
  %scratch.i1017 = alloca [32 x i8], align 4
  %hot_right.i1023 = alloca [736 x i8], align 32
  %hot_left.i1024 = alloca [736 x i8], align 32
  %peaks_right.i667 = alloca [1024 x i8], align 4
  %peaks_left.i668 = alloca [1024 x i8], align 4
  %scratch.i = alloca [32 x i8], align 4
  %hot_right.i674 = alloca [736 x i8], align 32
  %hot_left.i675 = alloca [736 x i8], align 32
  %right_prefix.i224 = alloca [32 x i8], align 32
  %left_prefix.i225 = alloca [32 x i8], align 32
  %uniform_right.i259 = alloca [128 x i8], align 32
  %uniform_left.i260 = alloca [128 x i8], align 32
  %peaks_right.i261 = alloca [1024 x i8], align 4
  %peaks_left.i262 = alloca [1024 x i8], align 4
  %hot_right.i268 = alloca [736 x i8], align 32
  %hot_left.i269 = alloca [736 x i8], align 32
  %right_prefix.i = alloca [32 x i8], align 32
  %left_prefix.i = alloca [32 x i8], align 32
  %uniform_right.i = alloca [128 x i8], align 32
  %uniform_left.i = alloca [128 x i8], align 32
  %peaks_right.i = alloca [1024 x i8], align 4
  %peaks_left.i = alloca [1024 x i8], align 4
  %hot_right.i = alloca [736 x i8], align 32
  %hot_left.i = alloca [736 x i8], align 32
  %shape = alloca [24 x i8], align 8
  %words = shl nuw nsw i64 %frames, 3, !dbg !26436
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 2153, !dbg !26437
  %1 = load i8, ptr %0, align 1, !dbg !26437, !range !17, !noundef !12
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 2144, !dbg !26439
  %3 = load i8, ptr %2, align 32, !dbg !26439, !range !17, !noundef !12
  %_7 = icmp eq i8 %1, %3, !dbg !26437
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 1776
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 1784
  %_95.1 = load i64, ptr %5, align 8, !dbg !26440
  br i1 %_7, label %bb1, label %start.bb20.thread_crit_edge, !dbg !26437

start.bb20.thread_crit_edge:                      ; preds = %start
  %_14.0.i.pre.pre = load ptr, ptr %4, align 8, !dbg !26441, !alias.scope !26446, !noalias !26449
  br label %bb20.thread, !dbg !26437

bb1:                                              ; preds = %start
  %_95.0 = load ptr, ptr %4, align 16, !dbg !26457, !nonnull !12, !noundef !12
  %_8.i6486 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_95.0, i64 %_95.1, !dbg !26458
  br label %bb1.i.i6487, !dbg !26463

bb1.i.i6487:                                      ; preds = %bb13.i.i6488, %bb1
  %_221.i.i = phi ptr [ %_22.i.i6489, %bb13.i.i6488 ], [ %_95.0, %bb1 ]
  %_12.i.i = icmp eq ptr %_221.i.i, %_8.i6486, !dbg !26465
  br i1 %_12.i.i, label %bb3, label %bb13.i.i6488, !dbg !26468

bb13.i.i6488:                                     ; preds = %bb1.i.i6487
  %_22.i.i6489 = getelementptr inbounds nuw i8, ptr %_221.i.i, i64 16, !dbg !26469
  %6 = getelementptr inbounds nuw i8, ptr %_221.i.i, i64 12, !dbg !26471
  %_3.i.i.i = load i32, ptr %6, align 4, !dbg !26471, !alias.scope !26473, !noalias !26478, !noundef !12
  %7 = icmp eq i32 %_3.i.i.i, 0, !dbg !26471
  %_51.i.i.i = load i32, ptr %_221.i.i, align 4, !dbg !26471, !alias.scope !26473, !noalias !26478
  %8 = getelementptr inbounds nuw i8, ptr %_221.i.i, i64 4, !dbg !26471
  %_72.i.i.i = load i32, ptr %8, align 4, !dbg !26471, !alias.scope !26473, !noalias !26478
  %9 = icmp eq i32 %_51.i.i.i, %_72.i.i.i, !dbg !26471
  %_0.sroa.0.0.i.i.i = select i1 %7, i1 %9, i1 false, !dbg !26471
  br i1 %_0.sroa.0.0.i.i.i, label %bb1.i.i6487, label %bb20.thread, !dbg !26481

bb3:                                              ; preds = %bb1.i.i6487
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 1792, !dbg !26482
  %_96.0 = load ptr, ptr %10, align 16, !dbg !26482, !nonnull !12, !noundef !12
  %11 = getelementptr inbounds nuw i8, ptr %self, i64 1800, !dbg !26482
  %_96.1 = load i64, ptr %11, align 8, !dbg !26482, !noundef !12
  %_8.i6490 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_96.0, i64 %_96.1, !dbg !26483
  br label %bb1.i.i6491, !dbg !26488

bb1.i.i6491:                                      ; preds = %bb13.i.i6494, %bb3
  %_221.i.i6492 = phi ptr [ %_22.i.i6495, %bb13.i.i6494 ], [ %_96.0, %bb3 ]
  %_12.i.i6493 = icmp eq ptr %_221.i.i6492, %_8.i6490, !dbg !26490
  br i1 %_12.i.i6493, label %bb5, label %bb13.i.i6494, !dbg !26493

bb13.i.i6494:                                     ; preds = %bb1.i.i6491
  %_22.i.i6495 = getelementptr inbounds nuw i8, ptr %_221.i.i6492, i64 16, !dbg !26494
  %12 = getelementptr inbounds nuw i8, ptr %_221.i.i6492, i64 12, !dbg !26496
  %_3.i.i.i6496 = load i32, ptr %12, align 4, !dbg !26496, !alias.scope !26498, !noalias !26503, !noundef !12
  %13 = icmp eq i32 %_3.i.i.i6496, 0, !dbg !26496
  %_51.i.i.i6497 = load i32, ptr %_221.i.i6492, align 4, !dbg !26496, !alias.scope !26498, !noalias !26503
  %14 = getelementptr inbounds nuw i8, ptr %_221.i.i6492, i64 4, !dbg !26496
  %_72.i.i.i6498 = load i32, ptr %14, align 4, !dbg !26496, !alias.scope !26498, !noalias !26503
  %15 = icmp eq i32 %_51.i.i.i6497, %_72.i.i.i6498, !dbg !26496
  %_0.sroa.0.0.i.i.i6499 = select i1 %13, i1 %15, i1 false, !dbg !26496
  br i1 %_0.sroa.0.0.i.i.i6499, label %bb1.i.i6491, label %bb20.thread, !dbg !26506

bb5:                                              ; preds = %bb1.i.i6491
  %16 = getelementptr inbounds nuw i8, ptr %self, i64 1976, !dbg !26507
  %_97.0 = load ptr, ptr %16, align 8, !dbg !26507, !nonnull !12, !noundef !12
  %17 = getelementptr inbounds nuw i8, ptr %self, i64 1984, !dbg !26507
  %_97.1 = load i64, ptr %17, align 8, !dbg !26507, !noundef !12
  %_8.i6501 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_97.0, i64 %_97.1, !dbg !26508
  br label %bb1.i.i6502, !dbg !26513

bb1.i.i6502:                                      ; preds = %bb13.i.i6505, %bb5
  %_221.i.i6503 = phi ptr [ %_22.i.i6506, %bb13.i.i6505 ], [ %_97.0, %bb5 ]
  %_12.i.i6504 = icmp eq ptr %_221.i.i6503, %_8.i6501, !dbg !26515
  br i1 %_12.i.i6504, label %bb7, label %bb13.i.i6505, !dbg !26518

bb13.i.i6505:                                     ; preds = %bb1.i.i6502
  %_22.i.i6506 = getelementptr inbounds nuw i8, ptr %_221.i.i6503, i64 16, !dbg !26519
  %18 = getelementptr inbounds nuw i8, ptr %_221.i.i6503, i64 12, !dbg !26521
  %_3.i.i.i6507 = load i32, ptr %18, align 4, !dbg !26521, !alias.scope !26523, !noalias !26528, !noundef !12
  %19 = icmp eq i32 %_3.i.i.i6507, 0, !dbg !26521
  %_51.i.i.i6508 = load i32, ptr %_221.i.i6503, align 4, !dbg !26521, !alias.scope !26523, !noalias !26528
  %20 = getelementptr inbounds nuw i8, ptr %_221.i.i6503, i64 4, !dbg !26521
  %_72.i.i.i6509 = load i32, ptr %20, align 4, !dbg !26521, !alias.scope !26523, !noalias !26528
  %21 = icmp eq i32 %_51.i.i.i6508, %_72.i.i.i6509, !dbg !26521
  %_0.sroa.0.0.i.i.i6510 = select i1 %19, i1 %21, i1 false, !dbg !26521
  br i1 %_0.sroa.0.0.i.i.i6510, label %bb1.i.i6502, label %bb20.thread, !dbg !26531

bb7:                                              ; preds = %bb1.i.i6502
  %22 = getelementptr inbounds nuw i8, ptr %self, i64 1992, !dbg !26532
  %_98.0 = load ptr, ptr %22, align 8, !dbg !26532, !nonnull !12, !noundef !12
  %23 = getelementptr inbounds nuw i8, ptr %self, i64 2000, !dbg !26532
  %_98.1 = load i64, ptr %23, align 8, !dbg !26532, !noundef !12
  %_8.i6512 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_98.0, i64 %_98.1, !dbg !26533
  br label %bb1.i.i6513, !dbg !26538

bb1.i.i6513:                                      ; preds = %bb13.i.i6516, %bb7
  %_221.i.i6514 = phi ptr [ %_22.i.i6517, %bb13.i.i6516 ], [ %_98.0, %bb7 ]
  %_12.i.i6515 = icmp eq ptr %_221.i.i6514, %_8.i6512, !dbg !26540
  br i1 %_12.i.i6515, label %bb9, label %bb13.i.i6516, !dbg !26543

bb13.i.i6516:                                     ; preds = %bb1.i.i6513
  %_22.i.i6517 = getelementptr inbounds nuw i8, ptr %_221.i.i6514, i64 16, !dbg !26544
  %24 = getelementptr inbounds nuw i8, ptr %_221.i.i6514, i64 12, !dbg !26546
  %_3.i.i.i6518 = load i32, ptr %24, align 4, !dbg !26546, !alias.scope !26548, !noalias !26553, !noundef !12
  %25 = icmp eq i32 %_3.i.i.i6518, 0, !dbg !26546
  %_51.i.i.i6519 = load i32, ptr %_221.i.i6514, align 4, !dbg !26546, !alias.scope !26548, !noalias !26553
  %26 = getelementptr inbounds nuw i8, ptr %_221.i.i6514, i64 4, !dbg !26546
  %_72.i.i.i6520 = load i32, ptr %26, align 4, !dbg !26546, !alias.scope !26548, !noalias !26553
  %27 = icmp eq i32 %_51.i.i.i6519, %_72.i.i.i6520, !dbg !26546
  %_0.sroa.0.0.i.i.i6521 = select i1 %25, i1 %27, i1 false, !dbg !26546
  br i1 %_0.sroa.0.0.i.i.i6521, label %bb1.i.i6513, label %bb20.thread, !dbg !26556

bb9:                                              ; preds = %bb1.i.i6513
  %_65.not = icmp samesign ugt i64 %words, %left_io.1
  br i1 %_65.not, label %bb45, label %bb1.i6523, !dbg !26557, !prof !165

bb45:                                             ; preds = %bb9
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %words, i64 noundef %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_76fe406493636db615b3ae498abf3aec) #30, !dbg !26566
  unreachable, !dbg !26566

bb1.i6523:                                        ; preds = %bb9, %bb10.i6530
  %iter.sroa.6.0.i = phi i64 [ %len.i.i.i.i, %bb10.i6530 ], [ %words, %bb9 ], !dbg !26567
  %iter.sroa.0.0.i6524 = phi ptr [ %data.i.i.i.i, %bb10.i6530 ], [ %left_io.0, %bb9 ], !dbg !26567
  %28 = icmp eq i64 %iter.sroa.6.0.i, 0, !dbg !26569
  br i1 %28, label %bb11, label %bb11.preheader.i, !dbg !26569

bb11.preheader.i:                                 ; preds = %bb1.i6523
  %..i.i.i = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i, i64 32), !dbg !26571
  %_18.idx.i = shl nuw nsw i64 %..i.i.i, 2, !dbg !26574
  %_18.i6525 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i6524, i64 %_18.idx.i, !dbg !26574
  br label %bb11.i6526, !dbg !26579

bb11.i6526:                                       ; preds = %bb11.i6526, %bb11.preheader.i
  %iter1.sroa.0.014.i = phi ptr [ %_31.i6527, %bb11.i6526 ], [ %iter.sroa.0.0.i6524, %bb11.preheader.i ]
  %bits.sroa.0.013.i = phi i32 [ %29, %bb11.i6526 ], [ 0, %bb11.preheader.i ]
  %_31.i6527 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i, i64 4, !dbg !26581
  %_134.i6528 = load i32, ptr %iter1.sroa.0.014.i, align 4, !dbg !26583, !alias.scope !26584, !noundef !12
  %29 = or i32 %_134.i6528, %bits.sroa.0.013.i, !dbg !26587
  %_25.i6529 = icmp eq ptr %_31.i6527, %_18.i6525, !dbg !26588
  br i1 %_25.i6529, label %bb10.i6530, label %bb11.i6526, !dbg !26579

bb10.i6530:                                       ; preds = %bb11.i6526
  %data.i.i.i.i = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i6524, i64 %..i.i.i, !dbg !26590
  %len.i.i.i.i = sub nuw nsw i64 %iter.sroa.6.0.i, %..i.i.i, !dbg !26595
  %30 = icmp eq i32 %29, 0, !dbg !26596
  br i1 %30, label %bb1.i6523, label %bb20.thread, !dbg !26596

bb11:                                             ; preds = %bb1.i6523
  %_73.not = icmp samesign ugt i64 %words, %right_io.1, !dbg !26597
  br i1 %_73.not, label %bb48, label %bb1.i6532, !dbg !26597, !prof !639

bb20.thread:                                      ; preds = %bb13.i.i6488, %bb13.i.i6494, %bb13.i.i6505, %bb13.i.i6516, %bb10.i6530, %start.bb20.thread_crit_edge
  %_14.0.i.pre = phi ptr [ %_14.0.i.pre.pre, %start.bb20.thread_crit_edge ], [ %_95.0, %bb13.i.i6516 ], [ %_95.0, %bb10.i6530 ], [ %_95.0, %bb13.i.i6494 ], [ %_95.0, %bb13.i.i6505 ], [ %_95.0, %bb13.i.i6488 ], !dbg !26441
  %31 = getelementptr inbounds nuw i8, ptr %self, i64 2152
  br label %bb26, !dbg !26603

bb20:                                             ; preds = %bb1.i6532
  %32 = getelementptr inbounds nuw i8, ptr %self, i64 2152
  %33 = load i8, ptr %32, align 8, !range !17
  %_22 = trunc nuw i8 %33 to i1
  br i1 %_22, label %bb22, label %bb26, !dbg !26603

bb48:                                             ; preds = %bb11
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %words, i64 noundef %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a95969ba91c39576d44da837974b8fd5) #30, !dbg !26604
  unreachable, !dbg !26604

bb1.i6532:                                        ; preds = %bb11, %bb10.i6547
  %iter.sroa.6.0.i6533 = phi i64 [ %len.i.i.i.i6538, %bb10.i6547 ], [ %words, %bb11 ], !dbg !26605
  %iter.sroa.0.0.i6534 = phi ptr [ %data.i.i.i.i6537, %bb10.i6547 ], [ %right_io.0, %bb11 ], !dbg !26605
  %34 = icmp eq i64 %iter.sroa.6.0.i6533, 0, !dbg !26607
  br i1 %34, label %bb20, label %bb11.preheader.i6535, !dbg !26607

bb11.preheader.i6535:                             ; preds = %bb1.i6532
  %..i.i.i6536 = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i6533, i64 32), !dbg !26609
  %_18.idx.i6539 = shl nuw nsw i64 %..i.i.i6536, 2, !dbg !26612
  %_18.i6540 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i6534, i64 %_18.idx.i6539, !dbg !26612
  br label %bb11.i6541, !dbg !26617

bb11.i6541:                                       ; preds = %bb11.i6541, %bb11.preheader.i6535
  %iter1.sroa.0.014.i6542 = phi ptr [ %_31.i6544, %bb11.i6541 ], [ %iter.sroa.0.0.i6534, %bb11.preheader.i6535 ]
  %bits.sroa.0.013.i6543 = phi i32 [ %35, %bb11.i6541 ], [ 0, %bb11.preheader.i6535 ]
  %_31.i6544 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i6542, i64 4, !dbg !26619
  %_134.i6545 = load i32, ptr %iter1.sroa.0.014.i6542, align 4, !dbg !26621, !alias.scope !26622, !noundef !12
  %35 = or i32 %_134.i6545, %bits.sroa.0.013.i6543, !dbg !26625
  %_25.i6546 = icmp eq ptr %_31.i6544, %_18.i6540, !dbg !26626
  br i1 %_25.i6546, label %bb10.i6547, label %bb11.i6541, !dbg !26617

bb10.i6547:                                       ; preds = %bb11.i6541
  %data.i.i.i.i6537 = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i6534, i64 %..i.i.i6536, !dbg !26628
  %len.i.i.i.i6538 = sub nuw nsw i64 %iter.sroa.6.0.i6533, %..i.i.i6536, !dbg !26633
  %36 = icmp eq i32 %35, 0, !dbg !26634
  br i1 %36, label %bb1.i6532, label %bb20.thread12329, !dbg !26634

bb20.thread12329:                                 ; preds = %bb10.i6547
  %37 = getelementptr inbounds nuw i8, ptr %self, i64 2152
  br label %bb26, !dbg !26603

bb26:                                             ; preds = %bb20.thread12329, %bb20.thread, %bb20
  %_14.0.i = phi ptr [ %_14.0.i.pre, %bb20.thread ], [ %_95.0, %bb20 ], [ %_95.0, %bb20.thread12329 ], !dbg !26441
  %38 = phi ptr [ %31, %bb20.thread ], [ %32, %bb20 ], [ %37, %bb20.thread12329 ]
  %quiet.sroa.0.012328 = phi i1 [ false, %bb20.thread ], [ true, %bb20 ], [ false, %bb20.thread12329 ]
  %_32 = getelementptr inbounds nuw i8, ptr %self, i64 1616, !dbg !26635
  %_33 = getelementptr inbounds nuw i8, ptr %self, i64 1648, !dbg !26636
  %_34 = getelementptr inbounds nuw i8, ptr %self, i64 1848, !dbg !26637
  %_35 = getelementptr inbounds nuw i8, ptr %self, i64 1640, !dbg !26638
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26446), !dbg !26639
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26640), !dbg !26639
  %_8.i6550 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_14.0.i, i64 %_95.1, !dbg !26641
  br label %bb1.i.i6551, !dbg !26646

bb1.i.i6551:                                      ; preds = %bb13.i.i6554, %bb26
  %_221.i.i6552 = phi ptr [ %_22.i.i6555, %bb13.i.i6554 ], [ %_14.0.i, %bb26 ]
  %_12.i.i6553 = icmp eq ptr %_221.i.i6552, %_8.i6550, !dbg !26648
  br i1 %_12.i.i6553, label %bb2.i1578, label %bb13.i.i6554, !dbg !26651

bb13.i.i6554:                                     ; preds = %bb1.i.i6551
  %_22.i.i6555 = getelementptr inbounds nuw i8, ptr %_221.i.i6552, i64 16, !dbg !26652
  %39 = getelementptr inbounds nuw i8, ptr %_221.i.i6552, i64 12, !dbg !26654
  %_3.i.i.i6556 = load i32, ptr %39, align 4, !dbg !26654, !alias.scope !26656, !noalias !26661, !noundef !12
  %40 = icmp eq i32 %_3.i.i.i6556, 0, !dbg !26654
  %_51.i.i.i6557 = load i32, ptr %_221.i.i6552, align 4, !dbg !26654, !alias.scope !26656, !noalias !26661
  %41 = getelementptr inbounds nuw i8, ptr %_221.i.i6552, i64 4, !dbg !26654
  %_72.i.i.i6558 = load i32, ptr %41, align 4, !dbg !26654, !alias.scope !26656, !noalias !26661
  %42 = icmp eq i32 %_51.i.i.i6557, %_72.i.i.i6558, !dbg !26654
  %_0.sroa.0.0.i.i.i6559 = select i1 %40, i1 %42, i1 false, !dbg !26654
  br i1 %_0.sroa.0.0.i.i.i6559, label %bb1.i.i6551, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit, !dbg !26664

bb2.i1578:                                        ; preds = %bb1.i.i6551
  %43 = getelementptr inbounds nuw i8, ptr %self, i64 1792, !dbg !26665
  %_15.0.i = load ptr, ptr %43, align 8, !dbg !26665, !alias.scope !26446, !noalias !26449, !nonnull !12, !noundef !12
  %44 = getelementptr inbounds nuw i8, ptr %self, i64 1800, !dbg !26665
  %_15.1.i = load i64, ptr %44, align 8, !dbg !26665, !alias.scope !26446, !noalias !26449, !noundef !12
  %_8.i6561 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_15.0.i, i64 %_15.1.i, !dbg !26666
  br label %bb1.i.i6562, !dbg !26671

bb1.i.i6562:                                      ; preds = %bb13.i.i6565, %bb2.i1578
  %_221.i.i6563 = phi ptr [ %_22.i.i6566, %bb13.i.i6565 ], [ %_15.0.i, %bb2.i1578 ]
  %_12.i.i6564 = icmp eq ptr %_221.i.i6563, %_8.i6561, !dbg !26673
  br i1 %_12.i.i6564, label %bb4.i1580, label %bb13.i.i6565, !dbg !26676

bb13.i.i6565:                                     ; preds = %bb1.i.i6562
  %_22.i.i6566 = getelementptr inbounds nuw i8, ptr %_221.i.i6563, i64 16, !dbg !26677
  %45 = getelementptr inbounds nuw i8, ptr %_221.i.i6563, i64 12, !dbg !26679
  %_3.i.i.i6567 = load i32, ptr %45, align 4, !dbg !26679, !alias.scope !26681, !noalias !26686, !noundef !12
  %46 = icmp eq i32 %_3.i.i.i6567, 0, !dbg !26679
  %_51.i.i.i6568 = load i32, ptr %_221.i.i6563, align 4, !dbg !26679, !alias.scope !26681, !noalias !26686
  %47 = getelementptr inbounds nuw i8, ptr %_221.i.i6563, i64 4, !dbg !26679
  %_72.i.i.i6569 = load i32, ptr %47, align 4, !dbg !26679, !alias.scope !26681, !noalias !26686
  %48 = icmp eq i32 %_51.i.i.i6568, %_72.i.i.i6569, !dbg !26679
  %_0.sroa.0.0.i.i.i6570 = select i1 %46, i1 %48, i1 false, !dbg !26679
  br i1 %_0.sroa.0.0.i.i.i6570, label %bb1.i.i6562, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit, !dbg !26689

bb4.i1580:                                        ; preds = %bb1.i.i6562
  %49 = getelementptr inbounds nuw i8, ptr %self, i64 1976, !dbg !26690
  %_16.0.i = load ptr, ptr %49, align 8, !dbg !26690, !alias.scope !26640, !noalias !26691, !nonnull !12, !noundef !12
  %50 = getelementptr inbounds nuw i8, ptr %self, i64 1984, !dbg !26690
  %_16.1.i = load i64, ptr %50, align 8, !dbg !26690, !alias.scope !26640, !noalias !26691, !noundef !12
  %_8.i6572 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_16.0.i, i64 %_16.1.i, !dbg !26692
  br label %bb1.i.i6573, !dbg !26697

bb1.i.i6573:                                      ; preds = %bb13.i.i6576, %bb4.i1580
  %_221.i.i6574 = phi ptr [ %_22.i.i6577, %bb13.i.i6576 ], [ %_16.0.i, %bb4.i1580 ]
  %_12.i.i6575 = icmp eq ptr %_221.i.i6574, %_8.i6572, !dbg !26699
  br i1 %_12.i.i6575, label %bb6.i1581, label %bb13.i.i6576, !dbg !26702

bb13.i.i6576:                                     ; preds = %bb1.i.i6573
  %_22.i.i6577 = getelementptr inbounds nuw i8, ptr %_221.i.i6574, i64 16, !dbg !26703
  %51 = getelementptr inbounds nuw i8, ptr %_221.i.i6574, i64 12, !dbg !26705
  %_3.i.i.i6578 = load i32, ptr %51, align 4, !dbg !26705, !alias.scope !26707, !noalias !26712, !noundef !12
  %52 = icmp eq i32 %_3.i.i.i6578, 0, !dbg !26705
  %_51.i.i.i6579 = load i32, ptr %_221.i.i6574, align 4, !dbg !26705, !alias.scope !26707, !noalias !26712
  %53 = getelementptr inbounds nuw i8, ptr %_221.i.i6574, i64 4, !dbg !26705
  %_72.i.i.i6580 = load i32, ptr %53, align 4, !dbg !26705, !alias.scope !26707, !noalias !26712
  %54 = icmp eq i32 %_51.i.i.i6579, %_72.i.i.i6580, !dbg !26705
  %_0.sroa.0.0.i.i.i6581 = select i1 %52, i1 %54, i1 false, !dbg !26705
  br i1 %_0.sroa.0.0.i.i.i6581, label %bb1.i.i6573, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit, !dbg !26715

bb6.i1581:                                        ; preds = %bb1.i.i6573
  %55 = getelementptr inbounds nuw i8, ptr %self, i64 1992, !dbg !26716
  %_17.0.i = load ptr, ptr %55, align 8, !dbg !26716, !alias.scope !26640, !noalias !26691, !nonnull !12, !noundef !12
  %56 = getelementptr inbounds nuw i8, ptr %self, i64 2000, !dbg !26716
  %_17.1.i = load i64, ptr %56, align 8, !dbg !26716, !alias.scope !26640, !noalias !26691, !noundef !12
  %_8.i6583 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_17.0.i, i64 %_17.1.i, !dbg !26717
  br label %bb1.i.i6584, !dbg !26722

bb1.i.i6584:                                      ; preds = %bb13.i.i6587, %bb6.i1581
  %_221.i.i6585 = phi ptr [ %_22.i.i6588, %bb13.i.i6587 ], [ %_17.0.i, %bb6.i1581 ]
  %_12.i.i6586 = icmp eq ptr %_221.i.i6585, %_8.i6583, !dbg !26724
  br i1 %_12.i.i6586, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit, label %bb13.i.i6587, !dbg !26727

bb13.i.i6587:                                     ; preds = %bb1.i.i6584
  %_22.i.i6588 = getelementptr inbounds nuw i8, ptr %_221.i.i6585, i64 16, !dbg !26728
  %57 = getelementptr inbounds nuw i8, ptr %_221.i.i6585, i64 12, !dbg !26730
  %_3.i.i.i6589 = load i32, ptr %57, align 4, !dbg !26730, !alias.scope !26732, !noalias !26737, !noundef !12
  %58 = icmp eq i32 %_3.i.i.i6589, 0, !dbg !26730
  %_51.i.i.i6590 = load i32, ptr %_221.i.i6585, align 4, !dbg !26730, !alias.scope !26732, !noalias !26737
  %59 = getelementptr inbounds nuw i8, ptr %_221.i.i6585, i64 4, !dbg !26730
  %_72.i.i.i6591 = load i32, ptr %59, align 4, !dbg !26730, !alias.scope !26732, !noalias !26737
  %60 = icmp eq i32 %_51.i.i.i6590, %_72.i.i.i6591, !dbg !26730
  %_0.sroa.0.0.i.i.i6592 = select i1 %58, i1 %60, i1 false, !dbg !26730
  br i1 %_0.sroa.0.0.i.i.i6592, label %bb1.i.i6584, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit, !dbg !26740

_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit: ; preds = %bb13.i.i6554, %bb13.i.i6565, %bb13.i.i6576, %bb13.i.i6587, %bb1.i.i6584
  %_0.sroa.0.0.i = phi i1 [ false, %bb13.i.i6576 ], [ false, %bb13.i.i6565 ], [ false, %bb13.i.i6587 ], [ true, %bb1.i.i6584 ], [ false, %bb13.i.i6554 ], !dbg !26741
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26742), !dbg !26745
  %61 = getelementptr inbounds nuw i8, ptr %self, i64 1824, !dbg !26747
  %_31.0.i = load ptr, ptr %61, align 8, !dbg !26747, !alias.scope !26742, !noalias !26749, !nonnull !12, !noundef !12
  %62 = getelementptr inbounds nuw i8, ptr %self, i64 1832, !dbg !26747
  %_31.1.i = load i64, ptr %62, align 8, !dbg !26747, !alias.scope !26742, !noalias !26749, !noundef !12
  %_17.idx.i = mul nuw nsw i64 %_31.1.i, 12, !dbg !26750
  %_17.i = getelementptr inbounds nuw i8, ptr %_31.0.i, i64 %_17.idx.i, !dbg !26750
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26754), !dbg !26757, !noalias !26749
  %_5.not.i.i.i = icmp eq i64 %_31.1.i, 0
  %63 = getelementptr inbounds nuw i8, ptr %_31.0.i, i64 4
  %64 = getelementptr inbounds nuw i8, ptr %_31.0.i, i64 8
  br i1 %_5.not.i.i.i, label %bb2.i6605, label %bb1.i.i6594

bb1.i.i6594:                                      ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i
  %_224.i.i = phi ptr [ %_22.i.i6597, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i ], [ %_31.0.i, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit ]
  %_12.i.i6595 = icmp eq ptr %_224.i.i, %_17.i, !dbg !26758
  br i1 %_12.i.i6595, label %bb2.i6605, label %bb13.i.i6596, !dbg !26762

bb13.i.i6596:                                     ; preds = %bb1.i.i6594
  %_22.i.i6597 = getelementptr inbounds nuw i8, ptr %_224.i.i, i64 12, !dbg !26763
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26765), !dbg !26768, !noalias !26749
  %_9.i.i.i6598 = load i32, ptr %_224.i.i, align 4, !dbg !26769, !alias.scope !26765, !noalias !26772, !noundef !12
  %_10.i.i.i = load i32, ptr %_31.0.i, align 4, !dbg !26769, !alias.scope !26754, !noalias !26774, !noundef !12
  %_8.i.i.i = icmp eq i32 %_9.i.i.i6598, %_10.i.i.i, !dbg !26769
  br i1 %_8.i.i.i, label %bb2.i.i.i, label %bb10.i, !dbg !26769

bb2.i.i.i:                                        ; preds = %bb13.i.i6596
  %65 = getelementptr inbounds nuw i8, ptr %_224.i.i, i64 4, !dbg !26769
  %_12.i.i.i6601 = load i32, ptr %65, align 4, !dbg !26769, !alias.scope !26765, !noalias !26772, !noundef !12
  %_13.i.i.i6602 = load i32, ptr %63, align 4, !dbg !26769, !alias.scope !26754, !noalias !26774, !noundef !12
  %_11.i.i.i = icmp eq i32 %_12.i.i.i6601, %_13.i.i.i6602, !dbg !26769
  br i1 %_11.i.i.i, label %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, label %bb10.i, !dbg !26769

_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i: ; preds = %bb2.i.i.i
  %66 = getelementptr inbounds nuw i8, ptr %_224.i.i, i64 8, !dbg !26769
  %_14.i.i.i6603 = load i32, ptr %66, align 4, !dbg !26769, !alias.scope !26765, !noalias !26772, !noundef !12
  %_15.i.i.i6604 = load i32, ptr %64, align 4, !dbg !26769, !alias.scope !26754, !noalias !26774, !noundef !12
  %67 = icmp eq i32 %_14.i.i.i6603, %_15.i.i.i6604, !dbg !26769
  br i1 %67, label %bb1.i.i6594, label %bb10.i, !dbg !26768

bb2.i6605:                                        ; preds = %bb1.i.i6594, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit
  %68 = getelementptr inbounds nuw i8, ptr %self, i64 1760, !dbg !26775
  %_32.0.i = load ptr, ptr %68, align 8, !dbg !26775, !alias.scope !26742, !noalias !26749, !nonnull !12, !noundef !12
  %69 = getelementptr inbounds nuw i8, ptr %self, i64 1768, !dbg !26775
  %_32.1.i = load i64, ptr %69, align 8, !dbg !26775, !alias.scope !26742, !noalias !26749, !noundef !12
  %_26.idx.i = shl nuw nsw i64 %_32.1.i, 2, !dbg !26776
  %_26.i6606 = getelementptr inbounds nuw i8, ptr %_32.0.i, i64 %_26.idx.i, !dbg !26776
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26780), !dbg !26783, !noalias !26749
  %_6.not.i.i.i = icmp eq i64 %_32.1.i, 0
  br i1 %_6.not.i.i.i, label %bb3.i, label %bb1.i3.i

bb1.i3.i:                                         ; preds = %bb2.i6605, %bb13.i5.i
  %_223.i.i = phi ptr [ %_22.i6.i, %bb13.i5.i ], [ %_32.0.i, %bb2.i6605 ]
  %_12.i4.i = icmp eq ptr %_223.i.i, %_26.i6606, !dbg !26784
  br i1 %_12.i4.i, label %bb3.i, label %bb13.i5.i, !dbg !26788

bb13.i5.i:                                        ; preds = %bb1.i3.i
  %_22.i6.i = getelementptr inbounds nuw i8, ptr %_223.i.i, i64 4, !dbg !26789
  %ptr.val.i.i = load i32, ptr %_223.i.i, align 4, !dbg !26791, !noalias !26792
  %_4.i.i.i6607 = load i32, ptr %_32.0.i, align 4, !dbg !26794, !alias.scope !26780, !noalias !26796, !noundef !12
  %_0.i.i.i = icmp eq i32 %ptr.val.i.i, %_4.i.i.i6607, !dbg !26797
  br i1 %_0.i.i.i, label %bb1.i3.i, label %bb10.i, !dbg !26791

bb3.i:                                            ; preds = %bb1.i3.i, %bb2.i6605
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26798), !dbg !26801
  %70 = getelementptr inbounds nuw i8, ptr %self, i64 2024, !dbg !26802
  %_31.0.i6608 = load ptr, ptr %70, align 8, !dbg !26802, !alias.scope !26798, !noalias !26749, !nonnull !12, !noundef !12
  %71 = getelementptr inbounds nuw i8, ptr %self, i64 2032, !dbg !26802
  %_31.1.i6609 = load i64, ptr %71, align 8, !dbg !26802, !alias.scope !26798, !noalias !26749, !noundef !12
  %_17.idx.i6610 = mul nuw nsw i64 %_31.1.i6609, 12, !dbg !26804
  %_17.i6611 = getelementptr inbounds nuw i8, ptr %_31.0.i6608, i64 %_17.idx.i6610, !dbg !26804
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26808), !dbg !26811, !noalias !26749
  %_5.not.i.i.i6612 = icmp eq i64 %_31.1.i6609, 0
  %72 = getelementptr inbounds nuw i8, ptr %_31.0.i6608, i64 4
  %73 = getelementptr inbounds nuw i8, ptr %_31.0.i6608, i64 8
  br i1 %_5.not.i.i.i6612, label %bb2.i6630, label %bb1.i.i6613

bb1.i.i6613:                                      ; preds = %bb3.i, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i6627
  %_224.i.i6614 = phi ptr [ %_22.i.i6617, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i6627 ], [ %_31.0.i6608, %bb3.i ]
  %_12.i.i6615 = icmp eq ptr %_224.i.i6614, %_17.i6611, !dbg !26812
  br i1 %_12.i.i6615, label %bb2.i6630, label %bb13.i.i6616, !dbg !26816

bb13.i.i6616:                                     ; preds = %bb1.i.i6613
  %_22.i.i6617 = getelementptr inbounds nuw i8, ptr %_224.i.i6614, i64 12, !dbg !26817
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26819), !dbg !26822, !noalias !26749
  %_9.i.i.i6618 = load i32, ptr %_224.i.i6614, align 4, !dbg !26823, !alias.scope !26819, !noalias !26826, !noundef !12
  %_10.i.i.i6619 = load i32, ptr %_31.0.i6608, align 4, !dbg !26823, !alias.scope !26808, !noalias !26828, !noundef !12
  %_8.i.i.i6620 = icmp eq i32 %_9.i.i.i6618, %_10.i.i.i6619, !dbg !26823
  br i1 %_8.i.i.i6620, label %bb2.i.i.i6623, label %bb10.i, !dbg !26823

bb2.i.i.i6623:                                    ; preds = %bb13.i.i6616
  %74 = getelementptr inbounds nuw i8, ptr %_224.i.i6614, i64 4, !dbg !26823
  %_12.i.i.i6624 = load i32, ptr %74, align 4, !dbg !26823, !alias.scope !26819, !noalias !26826, !noundef !12
  %_13.i.i.i6625 = load i32, ptr %72, align 4, !dbg !26823, !alias.scope !26808, !noalias !26828, !noundef !12
  %_11.i.i.i6626 = icmp eq i32 %_12.i.i.i6624, %_13.i.i.i6625, !dbg !26823
  br i1 %_11.i.i.i6626, label %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i6627, label %bb10.i, !dbg !26823

_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i6627: ; preds = %bb2.i.i.i6623
  %75 = getelementptr inbounds nuw i8, ptr %_224.i.i6614, i64 8, !dbg !26823
  %_14.i.i.i6628 = load i32, ptr %75, align 4, !dbg !26823, !alias.scope !26819, !noalias !26826, !noundef !12
  %_15.i.i.i6629 = load i32, ptr %73, align 4, !dbg !26823, !alias.scope !26808, !noalias !26828, !noundef !12
  %76 = icmp eq i32 %_14.i.i.i6628, %_15.i.i.i6629, !dbg !26823
  br i1 %76, label %bb1.i.i6613, label %bb10.i, !dbg !26822

bb2.i6630:                                        ; preds = %bb1.i.i6613, %bb3.i
  %77 = getelementptr inbounds nuw i8, ptr %self, i64 1960, !dbg !26829
  %_32.0.i6631 = load ptr, ptr %77, align 8, !dbg !26829, !alias.scope !26798, !noalias !26749, !nonnull !12, !noundef !12
  %78 = getelementptr inbounds nuw i8, ptr %self, i64 1968, !dbg !26829
  %_32.1.i6632 = load i64, ptr %78, align 8, !dbg !26829, !alias.scope !26798, !noalias !26749, !noundef !12
  %_26.idx.i6633 = shl nuw nsw i64 %_32.1.i6632, 2, !dbg !26830
  %_26.i6634 = getelementptr inbounds nuw i8, ptr %_32.0.i6631, i64 %_26.idx.i6633, !dbg !26830
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26834), !dbg !26837, !noalias !26749
  %_6.not.i.i.i6635 = icmp eq i64 %_32.1.i6632, 0
  br i1 %_6.not.i.i.i6635, label %bb5.i, label %bb1.i3.i6636

bb1.i3.i6636:                                     ; preds = %bb2.i6630, %bb13.i5.i6639
  %_223.i.i6637 = phi ptr [ %_22.i6.i6640, %bb13.i5.i6639 ], [ %_32.0.i6631, %bb2.i6630 ]
  %_12.i4.i6638 = icmp eq ptr %_223.i.i6637, %_26.i6634, !dbg !26838
  br i1 %_12.i4.i6638, label %bb5.i, label %bb13.i5.i6639, !dbg !26842

bb13.i5.i6639:                                    ; preds = %bb1.i3.i6636
  %_22.i6.i6640 = getelementptr inbounds nuw i8, ptr %_223.i.i6637, i64 4, !dbg !26843
  %ptr.val.i.i6641 = load i32, ptr %_223.i.i6637, align 4, !dbg !26845, !noalias !26846
  %_4.i.i.i6642 = load i32, ptr %_32.0.i6631, align 4, !dbg !26848, !alias.scope !26834, !noalias !26850, !noundef !12
  %_0.i.i.i6643 = icmp eq i32 %ptr.val.i.i6641, %_4.i.i.i6642, !dbg !26851
  br i1 %_0.i.i.i6643, label %bb1.i3.i6636, label %bb10.i, !dbg !26845

bb10.i:                                           ; preds = %bb13.i.i6596, %bb2.i.i.i, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, %bb13.i5.i, %bb13.i.i6616, %bb2.i.i.i6623, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i6627, %bb13.i5.i6639
  %79 = getelementptr inbounds nuw i8, ptr %self, i64 1536, !dbg !26852
  %80 = getelementptr inbounds nuw i8, ptr %self, i64 1537, !dbg !26852
  %81 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !26852
  %82 = add nuw nsw i64 %frames, 31, !dbg !26852
  %yield_count.sroa.0.0.i.i6724 = lshr i64 %82, 5, !dbg !26852
  %_111.not.i14884 = icmp eq i64 %yield_count.sroa.0.0.i.i6724, 0, !dbg !26852
  br i1 %_0.sroa.0.0.i, label %bb11.i, label %bb12.i, !dbg !26853

bb5.i:                                            ; preds = %bb1.i3.i6636, %bb2.i6630
  %83 = getelementptr inbounds nuw i8, ptr %self, i64 1536, !dbg !26852
  %84 = getelementptr inbounds nuw i8, ptr %self, i64 1537, !dbg !26852
  %85 = getelementptr inbounds nuw i8, ptr %self, i64 1624, !dbg !26852
  %86 = getelementptr inbounds nuw i8, ptr %self, i64 1632, !dbg !26852
  %87 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !26852
  br i1 %_0.sroa.0.0.i, label %bb6.i, label %bb7.i, !dbg !26854

bb12.i:                                           ; preds = %bb10.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26855), !dbg !26858
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26859), !dbg !26858
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26861), !dbg !26858
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26863), !dbg !26858
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26865), !dbg !26858
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_left.i1024, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #31, !dbg !26867
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_right.i1023, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #31, !dbg !26871
  %88 = load i8, ptr %79, align 32, !dbg !26873, !range !17, !alias.scope !26855, !noalias !26877, !noundef !12
  %89 = load i8, ptr %80, align 1, !dbg !26880, !range !17, !alias.scope !26855, !noalias !26877, !noundef !12
  %_34.i1031 = load i32, ptr %_35, align 4, !dbg !26882, !alias.scope !26865, !noalias !26884, !noundef !12
  %_36.i1032 = load i32, ptr %81, align 4, !dbg !26885, !alias.scope !26865, !noalias !26884, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i1017), !dbg !26887, !noalias !26889
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i1017, i8 0, i64 32, i1 false), !noalias !26889
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i1016), !dbg !26890, !noalias !26889
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i1016, i8 0, i64 1024, i1 false), !noalias !26889
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i1015), !dbg !26892, !noalias !26889
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i1015, i8 0, i64 1024, i1 false), !noalias !26889
  br i1 %_111.not.i14884, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, label %bb37.i1046.lr.ph, !dbg !26894

bb37.i1046.lr.ph:                                 ; preds = %bb12.i
  %90 = zext i32 %_36.i1032 to i64, !dbg !26885
  %91 = zext i32 %_34.i1031 to i64, !dbg !26882
  %_32.i1028 = trunc nuw i8 %89 to i1, !dbg !26880
  %_31.i1025 = trunc nuw i8 %88 to i1, !dbg !26873
  %92 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !26904
  %93 = bitcast <8 x float> %92 to <8 x i32>, !dbg !26910
  %94 = xor <8 x i32> %93, splat (i32 -1), !dbg !26916
  %history.i311.i.sroa.10.0.hot_left.i1024.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1024, i64 32
  %history.i311.i.sroa.13.0.hot_left.i1024.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1024, i64 64
  %history.i311.i.sroa.16.0.hot_left.i1024.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1024, i64 96
  %history.i311.i.sroa.19.0.hot_left.i1024.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1024, i64 128
  %history.i311.i.sroa.22.0.hot_left.i1024.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1024, i64 160
  %history.i311.i.sroa.25.0.hot_left.i1024.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1024, i64 192
  %history.i311.i.sroa.29.0.hot_left.i1024.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1024, i64 224
  %history.i311.i.sroa.32.0.hot_left.i1024.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1024, i64 256
  %history.i311.i.sroa.35.0.hot_left.i1024.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1024, i64 288
  %history.i311.i.sroa.38.0.hot_left.i1024.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1024, i64 320
  %history.i311.i.sroa.41.0.hot_left.i1024.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1024, i64 352
  %95 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %96 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %97 = getelementptr inbounds nuw i8, ptr %self, i64 96
  %row12.i.i.i322.i = getelementptr inbounds nuw i8, ptr %self, i64 128
  %98 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %99 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %100 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %row13.i.i.i323.i = getelementptr inbounds nuw i8, ptr %self, i64 256
  %101 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %102 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %103 = getelementptr inbounds nuw i8, ptr %self, i64 352
  %row14.i.i.i324.i = getelementptr inbounds nuw i8, ptr %self, i64 384
  %104 = getelementptr inbounds nuw i8, ptr %self, i64 416
  %105 = getelementptr inbounds nuw i8, ptr %self, i64 448
  %106 = getelementptr inbounds nuw i8, ptr %self, i64 480
  %row15.i.i.i325.i = getelementptr inbounds nuw i8, ptr %self, i64 512
  %107 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %108 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %109 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %row16.i.i.i326.i = getelementptr inbounds nuw i8, ptr %self, i64 640
  %110 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %111 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %112 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %row17.i.i.i327.i = getelementptr inbounds nuw i8, ptr %self, i64 768
  %113 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %114 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %115 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %row18.i.i.i328.i = getelementptr inbounds nuw i8, ptr %self, i64 896
  %116 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %117 = getelementptr inbounds nuw i8, ptr %self, i64 960
  %118 = getelementptr inbounds nuw i8, ptr %self, i64 992
  %row19.i.i.i329.i = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %119 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %120 = getelementptr inbounds nuw i8, ptr %self, i64 1088
  %121 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %row20.i.i.i330.i = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %122 = getelementptr inbounds nuw i8, ptr %self, i64 1184
  %123 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %124 = getelementptr inbounds nuw i8, ptr %self, i64 1248
  %row21.i.i.i331.i = getelementptr inbounds nuw i8, ptr %self, i64 1280
  %125 = getelementptr inbounds nuw i8, ptr %self, i64 1312
  %126 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %127 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %row22.i.i.i332.i = getelementptr inbounds nuw i8, ptr %self, i64 1408
  %128 = getelementptr inbounds nuw i8, ptr %self, i64 1440
  %129 = getelementptr inbounds nuw i8, ptr %self, i64 1472
  %130 = getelementptr inbounds nuw i8, ptr %self, i64 1504
  %history.i.i926.sroa.10.0.hot_right.i1023.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1023, i64 32
  %history.i.i926.sroa.13.0.hot_right.i1023.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1023, i64 64
  %history.i.i926.sroa.16.0.hot_right.i1023.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1023, i64 96
  %history.i.i926.sroa.19.0.hot_right.i1023.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1023, i64 128
  %history.i.i926.sroa.22.0.hot_right.i1023.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1023, i64 160
  %history.i.i926.sroa.25.0.hot_right.i1023.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1023, i64 192
  %history.i.i926.sroa.29.0.hot_right.i1023.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1023, i64 224
  %history.i.i926.sroa.32.0.hot_right.i1023.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1023, i64 256
  %history.i.i926.sroa.35.0.hot_right.i1023.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1023, i64 288
  %history.i.i926.sroa.38.0.hot_right.i1023.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1023, i64 320
  %history.i.i926.sroa.41.0.hot_right.i1023.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1023, i64 352
  %_63.i1062 = getelementptr inbounds nuw i8, ptr %hot_left.i1024, i64 384
  %_64.i1063 = getelementptr inbounds nuw i8, ptr %hot_left.i1024, i64 512
  %131 = getelementptr inbounds nuw i8, ptr %hot_left.i1024, i64 480
  %132 = getelementptr inbounds nuw i8, ptr %hot_left.i1024, i64 448
  %133 = getelementptr inbounds nuw i8, ptr %hot_left.i1024, i64 416
  %134 = getelementptr inbounds nuw i8, ptr %hot_left.i1024, i64 608
  %135 = getelementptr inbounds nuw i8, ptr %hot_left.i1024, i64 576
  %136 = getelementptr inbounds nuw i8, ptr %hot_left.i1024, i64 544
  %_68.i1064 = getelementptr inbounds nuw i8, ptr %hot_right.i1023, i64 384
  %_69.i1065 = getelementptr inbounds nuw i8, ptr %hot_right.i1023, i64 512
  %137 = getelementptr inbounds nuw i8, ptr %hot_right.i1023, i64 480
  %138 = getelementptr inbounds nuw i8, ptr %hot_right.i1023, i64 448
  %139 = getelementptr inbounds nuw i8, ptr %hot_right.i1023, i64 416
  %140 = getelementptr inbounds nuw i8, ptr %hot_right.i1023, i64 608
  %141 = getelementptr inbounds nuw i8, ptr %hot_right.i1023, i64 576
  %142 = getelementptr inbounds nuw i8, ptr %hot_right.i1023, i64 544
  %143 = select i1 %_31.i1025, <8 x i32> %93, <8 x i32> %94
  %144 = icmp slt <8 x i32> %143, zeroinitializer
  %145 = getelementptr inbounds nuw i8, ptr %self, i64 1624
  %146 = getelementptr inbounds nuw i8, ptr %self, i64 1840
  %147 = getelementptr inbounds nuw i8, ptr %self, i64 1688
  %148 = getelementptr inbounds nuw i8, ptr %self, i64 1680
  %149 = getelementptr inbounds nuw i8, ptr %self, i64 1768
  %150 = getelementptr inbounds nuw i8, ptr %self, i64 1760
  %151 = getelementptr inbounds nuw i8, ptr %self, i64 1736
  %152 = getelementptr inbounds nuw i8, ptr %self, i64 1728
  %153 = getelementptr inbounds nuw i8, ptr %self, i64 1704
  %154 = getelementptr inbounds nuw i8, ptr %self, i64 1696
  %155 = getelementptr inbounds nuw i8, ptr %hot_left.i1024, i64 672
  %156 = getelementptr inbounds nuw i8, ptr %hot_left.i1024, i64 704
  %157 = getelementptr inbounds nuw i8, ptr %hot_left.i1024, i64 640
  %158 = getelementptr inbounds nuw i8, ptr %self, i64 1672
  %159 = getelementptr inbounds nuw i8, ptr %self, i64 1664
  %160 = select i1 %_32.i1028, <8 x i32> %93, <8 x i32> %94
  %161 = icmp slt <8 x i32> %160, zeroinitializer
  %162 = getelementptr inbounds nuw i8, ptr %self, i64 2040
  %163 = getelementptr inbounds nuw i8, ptr %self, i64 1888
  %164 = getelementptr inbounds nuw i8, ptr %self, i64 1880
  %165 = getelementptr inbounds nuw i8, ptr %self, i64 2032
  %166 = getelementptr inbounds nuw i8, ptr %self, i64 2024
  %167 = getelementptr inbounds nuw i8, ptr %self, i64 1968
  %168 = getelementptr inbounds nuw i8, ptr %self, i64 1960
  %169 = getelementptr inbounds nuw i8, ptr %self, i64 1936
  %170 = getelementptr inbounds nuw i8, ptr %self, i64 1928
  %171 = getelementptr inbounds nuw i8, ptr %self, i64 1904
  %172 = getelementptr inbounds nuw i8, ptr %self, i64 1896
  %173 = getelementptr inbounds nuw i8, ptr %hot_right.i1023, i64 672
  %174 = getelementptr inbounds nuw i8, ptr %hot_right.i1023, i64 704
  %175 = getelementptr inbounds nuw i8, ptr %hot_right.i1023, i64 640
  %176 = getelementptr inbounds nuw i8, ptr %self, i64 1872
  %177 = getelementptr inbounds nuw i8, ptr %self, i64 1864
  %178 = getelementptr inbounds nuw i8, ptr %self, i64 1632
  %iter.i49.i948.sroa.0.0.ptr14630.1 = getelementptr inbounds nuw i8, ptr %scratch.i1017, i64 4
  %iter.i49.i948.sroa.0.0.ptr14630.2 = getelementptr inbounds nuw i8, ptr %scratch.i1017, i64 8
  %iter.i49.i948.sroa.0.0.ptr14630.3 = getelementptr inbounds nuw i8, ptr %scratch.i1017, i64 12
  %iter.i49.i948.sroa.0.0.ptr14630.4 = getelementptr inbounds nuw i8, ptr %scratch.i1017, i64 16
  %iter.i49.i948.sroa.0.0.ptr14630.5 = getelementptr inbounds nuw i8, ptr %scratch.i1017, i64 20
  %iter.i49.i948.sroa.0.0.ptr14630.6 = getelementptr inbounds nuw i8, ptr %scratch.i1017, i64 24
  %iter.i49.i948.sroa.0.0.ptr14630.7 = getelementptr inbounds nuw i8, ptr %scratch.i1017, i64 28
  %iter.i.i981.sroa.0.0.ptr14641.1 = getelementptr inbounds nuw i8, ptr %scratch.i1017, i64 4
  %iter.i.i981.sroa.0.0.ptr14641.2 = getelementptr inbounds nuw i8, ptr %scratch.i1017, i64 8
  %iter.i.i981.sroa.0.0.ptr14641.3 = getelementptr inbounds nuw i8, ptr %scratch.i1017, i64 12
  %iter.i.i981.sroa.0.0.ptr14641.4 = getelementptr inbounds nuw i8, ptr %scratch.i1017, i64 16
  %iter.i.i981.sroa.0.0.ptr14641.5 = getelementptr inbounds nuw i8, ptr %scratch.i1017, i64 20
  %iter.i.i981.sroa.0.0.ptr14641.6 = getelementptr inbounds nuw i8, ptr %scratch.i1017, i64 24
  %iter.i.i981.sroa.0.0.ptr14641.7 = getelementptr inbounds nuw i8, ptr %scratch.i1017, i64 28
  br label %bb37.i1046, !dbg !26894

bb13.i1040.loopexit.loopexit:                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6089
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
  br label %bb13.i1040.loopexit, !dbg !26894

bb13.i1040.loopexit:                              ; preds = %bb13.i1040.loopexit.loopexit, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1053
  %ring_cursor.sroa.0.1.i1056.lcssa = phi i64 [ %ring_cursor.sroa.0.0.i104314651, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1053 ], [ %spec.store.select13.i1201, %bb13.i1040.loopexit.loopexit ], !dbg !26942
  %main_cursor.sroa.0.1.i1057.lcssa = phi i64 [ %main_cursor.sroa.0.0.i104414652, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1053 ], [ %spec.store.select.i1199, %bb13.i1040.loopexit.loopexit ], !dbg !26943
  %_111.not.i1045 = icmp eq i64 %181, 0, !dbg !26894
  %indvars.iv.next = add nsw i64 %indvars.iv, -32, !dbg !26894
  br i1 %_111.not.i1045, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit, label %bb37.i1046, !dbg !26894

bb37.i1046:                                       ; preds = %bb37.i1046.lr.ph, %bb13.i1040.loopexit
  %indvars.iv = phi i64 [ %frames, %bb37.i1046.lr.ph ], [ %indvars.iv.next, %bb13.i1040.loopexit ]
  %main_cursor.sroa.0.0.i104414652 = phi i64 [ %91, %bb37.i1046.lr.ph ], [ %main_cursor.sroa.0.1.i1057.lcssa, %bb13.i1040.loopexit ]
  %ring_cursor.sroa.0.0.i104314651 = phi i64 [ %90, %bb37.i1046.lr.ph ], [ %ring_cursor.sroa.0.1.i1056.lcssa, %bb13.i1040.loopexit ]
  %iter4.sroa.0.0.i104214650 = phi i64 [ %yield_count.sroa.0.0.i.i6724, %bb37.i1046.lr.ph ], [ %181, %bb13.i1040.loopexit ]
  %iter.sroa.0.0.i104114649 = phi i64 [ 0, %bb37.i1046.lr.ph ], [ %180, %bb13.i1040.loopexit ]
  %179 = call i64 @llvm.umax.i64(i64 %indvars.iv, i64 1), !dbg !26944
  %umax17478 = call i64 @llvm.umin.i64(i64 %179, i64 32), !dbg !26944
  %180 = add nuw nsw i64 %iter.sroa.0.0.i104114649, 32, !dbg !26944
  %181 = add nsw i64 %iter4.sroa.0.0.i104214650, -1, !dbg !26948
  %history.i311.i.sroa.0.0.copyload = load <8 x float>, ptr %hot_left.i1024, align 32, !dbg !26949
  %history.i311.i.sroa.10.sroa.0.0.copyload = load <8 x float>, ptr %history.i311.i.sroa.10.0.hot_left.i1024.sroa_idx, align 32, !dbg !26949
  %history.i311.i.sroa.13.sroa.0.0.copyload = load <8 x float>, ptr %history.i311.i.sroa.13.0.hot_left.i1024.sroa_idx, align 32, !dbg !26949
  %history.i311.i.sroa.16.sroa.0.0.copyload = load <8 x float>, ptr %history.i311.i.sroa.16.0.hot_left.i1024.sroa_idx, align 32, !dbg !26949
  %history.i311.i.sroa.19.sroa.0.0.copyload = load <8 x float>, ptr %history.i311.i.sroa.19.0.hot_left.i1024.sroa_idx, align 32, !dbg !26949
  %history.i311.i.sroa.22.sroa.0.0.copyload = load <8 x float>, ptr %history.i311.i.sroa.22.0.hot_left.i1024.sroa_idx, align 32, !dbg !26949
  %history.i311.i.sroa.25.sroa.0.0.copyload = load <8 x float>, ptr %history.i311.i.sroa.25.0.hot_left.i1024.sroa_idx, align 32, !dbg !26949
  %history.i311.i.sroa.29.sroa.0.0.copyload = load <8 x float>, ptr %history.i311.i.sroa.29.0.hot_left.i1024.sroa_idx, align 32, !dbg !26949
  %history.i311.i.sroa.32.sroa.0.0.copyload = load <8 x float>, ptr %history.i311.i.sroa.32.0.hot_left.i1024.sroa_idx, align 32, !dbg !26949
  %history.i311.i.sroa.35.sroa.0.0.copyload = load <8 x float>, ptr %history.i311.i.sroa.35.0.hot_left.i1024.sroa_idx, align 32, !dbg !26949
  %history.i311.i.sroa.38.sroa.0.0.copyload = load <8 x float>, ptr %history.i311.i.sroa.38.0.hot_left.i1024.sroa_idx, align 32, !dbg !26949
  %history.i311.i.sroa.41.sroa.0.0.copyload = load <8 x float>, ptr %history.i311.i.sroa.41.0.hot_left.i1024.sroa_idx, align 32, !dbg !26949
  %_20.i314.i14569.not = icmp eq i64 %frames, %iter.sroa.0.0.i104114649, !dbg !26951
  br i1 %_20.i314.i14569.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit346.i, label %bb5.i315.i.lr.ph, !dbg !26955

bb5.i315.i.lr.ph:                                 ; preds = %bb37.i1046
  %_5.i3939 = load <8 x float>, ptr %self, align 32
  %_14.i.i.i276.i.sroa.0.0.copyload = load <8 x float>, ptr %95, align 32
  %_17.i.i.i273.i.sroa.0.0.copyload = load <8 x float>, ptr %96, align 32
  %_20.i.i.i270.i.sroa.0.0.copyload = load <8 x float>, ptr %97, align 32
  %_25.i.i.i266.i.sroa.0.0.copyload = load <8 x float>, ptr %row12.i.i.i322.i, align 32
  %_28.i.i.i263.i.sroa.0.0.copyload = load <8 x float>, ptr %98, align 32
  %_31.i.i.i260.i.sroa.0.0.copyload = load <8 x float>, ptr %99, align 32
  %_34.i.i.i257.i.sroa.0.0.copyload = load <8 x float>, ptr %100, align 32
  %_39.i.i.i253.i.sroa.0.0.copyload = load <8 x float>, ptr %row13.i.i.i323.i, align 32
  %_42.i.i.i250.i.sroa.0.0.copyload = load <8 x float>, ptr %101, align 32
  %_45.i.i.i247.i.sroa.0.0.copyload = load <8 x float>, ptr %102, align 32
  %_48.i.i.i244.i.sroa.0.0.copyload = load <8 x float>, ptr %103, align 32
  %_53.i.i.i240.i.sroa.0.0.copyload = load <8 x float>, ptr %row14.i.i.i324.i, align 32
  %_56.i.i.i237.i.sroa.0.0.copyload = load <8 x float>, ptr %104, align 32
  %_59.i.i.i234.i.sroa.0.0.copyload = load <8 x float>, ptr %105, align 32
  %_62.i.i.i231.i.sroa.0.0.copyload = load <8 x float>, ptr %106, align 32
  %_67.i.i.i227.i.sroa.0.0.copyload = load <8 x float>, ptr %row15.i.i.i325.i, align 32
  %_70.i.i.i224.i.sroa.0.0.copyload = load <8 x float>, ptr %107, align 32
  %_73.i.i.i221.i.sroa.0.0.copyload = load <8 x float>, ptr %108, align 32
  %_76.i.i.i218.i.sroa.0.0.copyload = load <8 x float>, ptr %109, align 32
  %_81.i.i.i214.i.sroa.0.0.copyload = load <8 x float>, ptr %row16.i.i.i326.i, align 32
  %_84.i.i.i211.i.sroa.0.0.copyload = load <8 x float>, ptr %110, align 32
  %_87.i.i.i208.i.sroa.0.0.copyload = load <8 x float>, ptr %111, align 32
  %_90.i.i.i205.i.sroa.0.0.copyload = load <8 x float>, ptr %112, align 32
  %_95.i.i.i201.i.sroa.0.0.copyload = load <8 x float>, ptr %row17.i.i.i327.i, align 32
  %_98.i.i.i198.i.sroa.0.0.copyload = load <8 x float>, ptr %113, align 32
  %_101.i.i.i195.i.sroa.0.0.copyload = load <8 x float>, ptr %114, align 32
  %_104.i.i.i192.i.sroa.0.0.copyload = load <8 x float>, ptr %115, align 32
  %_109.i.i.i188.i.sroa.0.0.copyload = load <8 x float>, ptr %row18.i.i.i328.i, align 32
  %_112.i.i.i185.i.sroa.0.0.copyload = load <8 x float>, ptr %116, align 32
  %_115.i.i.i182.i.sroa.0.0.copyload = load <8 x float>, ptr %117, align 32
  %_118.i.i.i179.i.sroa.0.0.copyload = load <8 x float>, ptr %118, align 32
  %_123.i.i.i175.i.sroa.0.0.copyload = load <8 x float>, ptr %row19.i.i.i329.i, align 32
  %_126.i.i.i172.i.sroa.0.0.copyload = load <8 x float>, ptr %119, align 32
  %_129.i.i.i169.i.sroa.0.0.copyload = load <8 x float>, ptr %120, align 32
  %_132.i.i.i166.i.sroa.0.0.copyload = load <8 x float>, ptr %121, align 32
  %_137.i.i.i162.i.sroa.0.0.copyload = load <8 x float>, ptr %row20.i.i.i330.i, align 32
  %_140.i.i.i159.i.sroa.0.0.copyload = load <8 x float>, ptr %122, align 32
  %_143.i.i.i156.i.sroa.0.0.copyload = load <8 x float>, ptr %123, align 32
  %_146.i.i.i153.i.sroa.0.0.copyload = load <8 x float>, ptr %124, align 32
  %_151.i.i.i149.i.sroa.0.0.copyload = load <8 x float>, ptr %row21.i.i.i331.i, align 32
  %_154.i.i.i146.i.sroa.0.0.copyload = load <8 x float>, ptr %125, align 32
  %_157.i.i.i143.i.sroa.0.0.copyload = load <8 x float>, ptr %126, align 32
  %_160.i.i.i140.i.sroa.0.0.copyload = load <8 x float>, ptr %127, align 32
  %_165.i.i.i136.i.sroa.0.0.copyload = load <8 x float>, ptr %row22.i.i.i332.i, align 32
  %_168.i.i.i133.i.sroa.0.0.copyload = load <8 x float>, ptr %128, align 32
  %_171.i.i.i130.i.sroa.0.0.copyload = load <8 x float>, ptr %129, align 32
  %_174.i.i.i127.i.sroa.0.0.copyload = load <8 x float>, ptr %130, align 32
  br label %bb5.i315.i, !dbg !26955

bb5.i315.i:                                       ; preds = %bb5.i315.i.lr.ph, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit
  %iter.sroa.0.0.i313.i14581 = phi i64 [ 0, %bb5.i315.i.lr.ph ], [ %182, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %history.i311.i.sroa.10.sroa.0.014580 = phi <8 x float> [ %history.i311.i.sroa.10.sroa.0.0.copyload, %bb5.i315.i.lr.ph ], [ %history.i311.i.sroa.0.014570, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %history.i311.i.sroa.13.sroa.0.014579 = phi <8 x float> [ %history.i311.i.sroa.13.sroa.0.0.copyload, %bb5.i315.i.lr.ph ], [ %history.i311.i.sroa.10.sroa.0.014580, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %history.i311.i.sroa.16.sroa.0.014578 = phi <8 x float> [ %history.i311.i.sroa.16.sroa.0.0.copyload, %bb5.i315.i.lr.ph ], [ %history.i311.i.sroa.13.sroa.0.014579, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %history.i311.i.sroa.19.sroa.0.014577 = phi <8 x float> [ %history.i311.i.sroa.19.sroa.0.0.copyload, %bb5.i315.i.lr.ph ], [ %history.i311.i.sroa.16.sroa.0.014578, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %history.i311.i.sroa.22.sroa.0.014576 = phi <8 x float> [ %history.i311.i.sroa.22.sroa.0.0.copyload, %bb5.i315.i.lr.ph ], [ %history.i311.i.sroa.19.sroa.0.014577, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %history.i311.i.sroa.38.sroa.0.014575 = phi <8 x float> [ %history.i311.i.sroa.38.sroa.0.0.copyload, %bb5.i315.i.lr.ph ], [ %history.i311.i.sroa.35.sroa.0.014574, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %history.i311.i.sroa.35.sroa.0.014574 = phi <8 x float> [ %history.i311.i.sroa.35.sroa.0.0.copyload, %bb5.i315.i.lr.ph ], [ %history.i311.i.sroa.32.sroa.0.014573, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %history.i311.i.sroa.32.sroa.0.014573 = phi <8 x float> [ %history.i311.i.sroa.32.sroa.0.0.copyload, %bb5.i315.i.lr.ph ], [ %history.i311.i.sroa.29.sroa.0.014572, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %history.i311.i.sroa.29.sroa.0.014572 = phi <8 x float> [ %history.i311.i.sroa.29.sroa.0.0.copyload, %bb5.i315.i.lr.ph ], [ %history.i311.i.sroa.25.sroa.0.014571, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %history.i311.i.sroa.25.sroa.0.014571 = phi <8 x float> [ %history.i311.i.sroa.25.sroa.0.0.copyload, %bb5.i315.i.lr.ph ], [ %history.i311.i.sroa.22.sroa.0.014576, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %history.i311.i.sroa.0.014570 = phi <8 x float> [ %history.i311.i.sroa.0.0.copyload, %bb5.i315.i.lr.ph ], [ %lanes.i5309.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %182 = add nuw nsw i64 %iter.sroa.0.0.i313.i14581, 1, !dbg !26956
  %_11.i316.i = add nuw nsw i64 %iter.sroa.0.0.i313.i14581, %iter.sroa.0.0.i104114649, !dbg !26959
  %base.i317.i = shl i64 %_11.i316.i, 3, !dbg !26959
  %_24.i318.i = icmp samesign ugt i64 %base.i317.i, %left_io.1, !dbg !26960
  br i1 %_24.i318.i, label %bb7.i345.i, label %bb8.i319.i, !dbg !26960, !prof !639

bb8.i319.i:                                       ; preds = %bb5.i315.i
  %_27.i320.i = sub nuw nsw i64 %left_io.1, %base.i317.i, !dbg !26963
  %_8.i5312 = icmp samesign ugt i64 %_27.i320.i, 7, !dbg !26964
  br i1 %_8.i5312, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit, label %bb2.i5313, !dbg !26964, !prof !651

bb2.i5313:                                        ; preds = %bb8.i319.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_27.i320.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !26969, !noalias !26970
  unreachable, !dbg !26969

bb7.i345.i:                                       ; preds = %bb5.i315.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i317.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_fc26f793d85338b5649d38df0c19e7e0) #30, !dbg !26977, !noalias !26978
  unreachable, !dbg !26977

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit: ; preds = %bb8.i319.i
  %_31.i321.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %base.i317.i, !dbg !26979
  %lanes.i5309.sroa.0.0.copyload = load <8 x float>, ptr %_31.i321.i, align 4, !dbg !26981, !alias.scope !26985, !noalias !26989
  %183 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i311.i.sroa.22.sroa.0.014576), !dbg !26991
  %184 = fmul <8 x float> %lanes.i5309.sroa.0.0.copyload, %_5.i3939, !dbg !26998
  %185 = fadd <8 x float> %184, zeroinitializer, !dbg !27004
  %186 = fmul <8 x float> %lanes.i5309.sroa.0.0.copyload, %_14.i.i.i276.i.sroa.0.0.copyload, !dbg !27009
  %187 = fadd <8 x float> %186, zeroinitializer, !dbg !27014
  %188 = fmul <8 x float> %lanes.i5309.sroa.0.0.copyload, %_17.i.i.i273.i.sroa.0.0.copyload, !dbg !27019
  %189 = fadd <8 x float> %188, zeroinitializer, !dbg !27024
  %190 = fmul <8 x float> %lanes.i5309.sroa.0.0.copyload, %_20.i.i.i270.i.sroa.0.0.copyload, !dbg !27029
  %191 = fadd <8 x float> %190, zeroinitializer, !dbg !27034
  %192 = fmul <8 x float> %history.i311.i.sroa.0.014570, %_25.i.i.i266.i.sroa.0.0.copyload, !dbg !27039
  %193 = fadd <8 x float> %185, %192, !dbg !27044
  %194 = fmul <8 x float> %history.i311.i.sroa.0.014570, %_28.i.i.i263.i.sroa.0.0.copyload, !dbg !27049
  %195 = fadd <8 x float> %187, %194, !dbg !27054
  %196 = fmul <8 x float> %history.i311.i.sroa.0.014570, %_31.i.i.i260.i.sroa.0.0.copyload, !dbg !27059
  %197 = fadd <8 x float> %189, %196, !dbg !27064
  %198 = fmul <8 x float> %history.i311.i.sroa.0.014570, %_34.i.i.i257.i.sroa.0.0.copyload, !dbg !27069
  %199 = fadd <8 x float> %191, %198, !dbg !27074
  %200 = fmul <8 x float> %history.i311.i.sroa.10.sroa.0.014580, %_39.i.i.i253.i.sroa.0.0.copyload, !dbg !27079
  %201 = fadd <8 x float> %193, %200, !dbg !27084
  %202 = fmul <8 x float> %history.i311.i.sroa.10.sroa.0.014580, %_42.i.i.i250.i.sroa.0.0.copyload, !dbg !27089
  %203 = fadd <8 x float> %195, %202, !dbg !27094
  %204 = fmul <8 x float> %history.i311.i.sroa.10.sroa.0.014580, %_45.i.i.i247.i.sroa.0.0.copyload, !dbg !27099
  %205 = fadd <8 x float> %197, %204, !dbg !27104
  %206 = fmul <8 x float> %history.i311.i.sroa.10.sroa.0.014580, %_48.i.i.i244.i.sroa.0.0.copyload, !dbg !27109
  %207 = fadd <8 x float> %199, %206, !dbg !27114
  %208 = fmul <8 x float> %history.i311.i.sroa.13.sroa.0.014579, %_53.i.i.i240.i.sroa.0.0.copyload, !dbg !27119
  %209 = fadd <8 x float> %201, %208, !dbg !27124
  %210 = fmul <8 x float> %history.i311.i.sroa.13.sroa.0.014579, %_56.i.i.i237.i.sroa.0.0.copyload, !dbg !27129
  %211 = fadd <8 x float> %203, %210, !dbg !27134
  %212 = fmul <8 x float> %history.i311.i.sroa.13.sroa.0.014579, %_59.i.i.i234.i.sroa.0.0.copyload, !dbg !27139
  %213 = fadd <8 x float> %205, %212, !dbg !27144
  %214 = fmul <8 x float> %history.i311.i.sroa.13.sroa.0.014579, %_62.i.i.i231.i.sroa.0.0.copyload, !dbg !27149
  %215 = fadd <8 x float> %207, %214, !dbg !27154
  %216 = fmul <8 x float> %history.i311.i.sroa.16.sroa.0.014578, %_67.i.i.i227.i.sroa.0.0.copyload, !dbg !27159
  %217 = fadd <8 x float> %209, %216, !dbg !27164
  %218 = fmul <8 x float> %history.i311.i.sroa.16.sroa.0.014578, %_70.i.i.i224.i.sroa.0.0.copyload, !dbg !27169
  %219 = fadd <8 x float> %211, %218, !dbg !27174
  %220 = fmul <8 x float> %history.i311.i.sroa.16.sroa.0.014578, %_73.i.i.i221.i.sroa.0.0.copyload, !dbg !27179
  %221 = fadd <8 x float> %213, %220, !dbg !27184
  %222 = fmul <8 x float> %history.i311.i.sroa.16.sroa.0.014578, %_76.i.i.i218.i.sroa.0.0.copyload, !dbg !27189
  %223 = fadd <8 x float> %215, %222, !dbg !27194
  %224 = fmul <8 x float> %history.i311.i.sroa.19.sroa.0.014577, %_81.i.i.i214.i.sroa.0.0.copyload, !dbg !27199
  %225 = fadd <8 x float> %217, %224, !dbg !27204
  %226 = fmul <8 x float> %history.i311.i.sroa.19.sroa.0.014577, %_84.i.i.i211.i.sroa.0.0.copyload, !dbg !27209
  %227 = fadd <8 x float> %219, %226, !dbg !27214
  %228 = fmul <8 x float> %history.i311.i.sroa.19.sroa.0.014577, %_87.i.i.i208.i.sroa.0.0.copyload, !dbg !27219
  %229 = fadd <8 x float> %221, %228, !dbg !27224
  %230 = fmul <8 x float> %history.i311.i.sroa.19.sroa.0.014577, %_90.i.i.i205.i.sroa.0.0.copyload, !dbg !27229
  %231 = fadd <8 x float> %223, %230, !dbg !27234
  %232 = fmul <8 x float> %history.i311.i.sroa.22.sroa.0.014576, %_95.i.i.i201.i.sroa.0.0.copyload, !dbg !27239
  %233 = fadd <8 x float> %225, %232, !dbg !27244
  %234 = fmul <8 x float> %history.i311.i.sroa.22.sroa.0.014576, %_98.i.i.i198.i.sroa.0.0.copyload, !dbg !27249
  %235 = fadd <8 x float> %227, %234, !dbg !27254
  %236 = fmul <8 x float> %history.i311.i.sroa.22.sroa.0.014576, %_101.i.i.i195.i.sroa.0.0.copyload, !dbg !27259
  %237 = fadd <8 x float> %229, %236, !dbg !27264
  %238 = fmul <8 x float> %history.i311.i.sroa.22.sroa.0.014576, %_104.i.i.i192.i.sroa.0.0.copyload, !dbg !27269
  %239 = fadd <8 x float> %231, %238, !dbg !27274
  %240 = fmul <8 x float> %history.i311.i.sroa.25.sroa.0.014571, %_109.i.i.i188.i.sroa.0.0.copyload, !dbg !27279
  %241 = fadd <8 x float> %233, %240, !dbg !27284
  %242 = fmul <8 x float> %history.i311.i.sroa.25.sroa.0.014571, %_112.i.i.i185.i.sroa.0.0.copyload, !dbg !27289
  %243 = fadd <8 x float> %235, %242, !dbg !27294
  %244 = fmul <8 x float> %history.i311.i.sroa.25.sroa.0.014571, %_115.i.i.i182.i.sroa.0.0.copyload, !dbg !27299
  %245 = fadd <8 x float> %237, %244, !dbg !27304
  %246 = fmul <8 x float> %history.i311.i.sroa.25.sroa.0.014571, %_118.i.i.i179.i.sroa.0.0.copyload, !dbg !27309
  %247 = fadd <8 x float> %239, %246, !dbg !27314
  %248 = fmul <8 x float> %history.i311.i.sroa.29.sroa.0.014572, %_123.i.i.i175.i.sroa.0.0.copyload, !dbg !27319
  %249 = fadd <8 x float> %241, %248, !dbg !27324
  %250 = fmul <8 x float> %history.i311.i.sroa.29.sroa.0.014572, %_126.i.i.i172.i.sroa.0.0.copyload, !dbg !27329
  %251 = fadd <8 x float> %243, %250, !dbg !27334
  %252 = fmul <8 x float> %history.i311.i.sroa.29.sroa.0.014572, %_129.i.i.i169.i.sroa.0.0.copyload, !dbg !27339
  %253 = fadd <8 x float> %245, %252, !dbg !27344
  %254 = fmul <8 x float> %history.i311.i.sroa.29.sroa.0.014572, %_132.i.i.i166.i.sroa.0.0.copyload, !dbg !27349
  %255 = fadd <8 x float> %247, %254, !dbg !27354
  %256 = fmul <8 x float> %history.i311.i.sroa.32.sroa.0.014573, %_137.i.i.i162.i.sroa.0.0.copyload, !dbg !27359
  %257 = fadd <8 x float> %249, %256, !dbg !27364
  %258 = fmul <8 x float> %history.i311.i.sroa.32.sroa.0.014573, %_140.i.i.i159.i.sroa.0.0.copyload, !dbg !27369
  %259 = fadd <8 x float> %251, %258, !dbg !27374
  %260 = fmul <8 x float> %history.i311.i.sroa.32.sroa.0.014573, %_143.i.i.i156.i.sroa.0.0.copyload, !dbg !27379
  %261 = fadd <8 x float> %253, %260, !dbg !27384
  %262 = fmul <8 x float> %history.i311.i.sroa.32.sroa.0.014573, %_146.i.i.i153.i.sroa.0.0.copyload, !dbg !27389
  %263 = fadd <8 x float> %255, %262, !dbg !27394
  %264 = fmul <8 x float> %history.i311.i.sroa.35.sroa.0.014574, %_151.i.i.i149.i.sroa.0.0.copyload, !dbg !27399
  %265 = fadd <8 x float> %257, %264, !dbg !27404
  %266 = fmul <8 x float> %history.i311.i.sroa.35.sroa.0.014574, %_154.i.i.i146.i.sroa.0.0.copyload, !dbg !27409
  %267 = fadd <8 x float> %259, %266, !dbg !27414
  %268 = fmul <8 x float> %history.i311.i.sroa.35.sroa.0.014574, %_157.i.i.i143.i.sroa.0.0.copyload, !dbg !27419
  %269 = fadd <8 x float> %261, %268, !dbg !27424
  %270 = fmul <8 x float> %history.i311.i.sroa.35.sroa.0.014574, %_160.i.i.i140.i.sroa.0.0.copyload, !dbg !27429
  %271 = fadd <8 x float> %263, %270, !dbg !27434
  %272 = fmul <8 x float> %history.i311.i.sroa.38.sroa.0.014575, %_165.i.i.i136.i.sroa.0.0.copyload, !dbg !27439
  %273 = fadd <8 x float> %265, %272, !dbg !27444
  %274 = fmul <8 x float> %history.i311.i.sroa.38.sroa.0.014575, %_168.i.i.i133.i.sroa.0.0.copyload, !dbg !27449
  %275 = fadd <8 x float> %267, %274, !dbg !27454
  %276 = fmul <8 x float> %history.i311.i.sroa.38.sroa.0.014575, %_171.i.i.i130.i.sroa.0.0.copyload, !dbg !27459
  %277 = fadd <8 x float> %269, %276, !dbg !27464
  %278 = fmul <8 x float> %history.i311.i.sroa.38.sroa.0.014575, %_174.i.i.i127.i.sroa.0.0.copyload, !dbg !27469
  %279 = fadd <8 x float> %271, %278, !dbg !27474
  %280 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %273), !dbg !27479
  %281 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %183, <8 x float> %280), !dbg !27485
  %282 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %275), !dbg !27479
  %283 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %281, <8 x float> %282), !dbg !27485
  %284 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %277), !dbg !27479
  %285 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %283, <8 x float> %284), !dbg !27485
  %286 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %279), !dbg !27479
  %287 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %285, <8 x float> %286), !dbg !27485
  %_39.i342.i.idx = shl i64 %iter.sroa.0.0.i313.i14581, 5, !dbg !27490
  %_39.i342.i = getelementptr inbounds nuw i8, ptr %peaks_left.i1016, i64 %_39.i342.i.idx, !dbg !27490
  store <8 x float> %287, ptr %_39.i342.i, align 4, !dbg !27495, !alias.scope !27500, !noalias !27504
  %exitcond.not = icmp eq i64 %182, %umax17478, !dbg !26951
  br i1 %exitcond.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit346.i, label %bb5.i315.i, !dbg !26955

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit346.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit, %bb37.i1046
  %history.i311.i.sroa.0.0.lcssa = phi <8 x float> [ %history.i311.i.sroa.0.0.copyload, %bb37.i1046 ], [ %lanes.i5309.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !27508
  %history.i311.i.sroa.25.sroa.0.0.lcssa = phi <8 x float> [ %history.i311.i.sroa.25.sroa.0.0.copyload, %bb37.i1046 ], [ %history.i311.i.sroa.22.sroa.0.014576, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !27508
  %history.i311.i.sroa.29.sroa.0.0.lcssa = phi <8 x float> [ %history.i311.i.sroa.29.sroa.0.0.copyload, %bb37.i1046 ], [ %history.i311.i.sroa.25.sroa.0.014571, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !27508
  %history.i311.i.sroa.32.sroa.0.0.lcssa = phi <8 x float> [ %history.i311.i.sroa.32.sroa.0.0.copyload, %bb37.i1046 ], [ %history.i311.i.sroa.29.sroa.0.014572, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !27508
  %history.i311.i.sroa.35.sroa.0.0.lcssa = phi <8 x float> [ %history.i311.i.sroa.35.sroa.0.0.copyload, %bb37.i1046 ], [ %history.i311.i.sroa.32.sroa.0.014573, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !27508
  %history.i311.i.sroa.38.sroa.0.0.lcssa = phi <8 x float> [ %history.i311.i.sroa.38.sroa.0.0.copyload, %bb37.i1046 ], [ %history.i311.i.sroa.35.sroa.0.014574, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !27508
  %history.i311.i.sroa.41.sroa.0.0.lcssa = phi <8 x float> [ %history.i311.i.sroa.41.sroa.0.0.copyload, %bb37.i1046 ], [ %history.i311.i.sroa.38.sroa.0.014575, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !27508
  %history.i311.i.sroa.22.sroa.0.0.lcssa = phi <8 x float> [ %history.i311.i.sroa.22.sroa.0.0.copyload, %bb37.i1046 ], [ %history.i311.i.sroa.19.sroa.0.014577, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !27508
  %history.i311.i.sroa.19.sroa.0.0.lcssa = phi <8 x float> [ %history.i311.i.sroa.19.sroa.0.0.copyload, %bb37.i1046 ], [ %history.i311.i.sroa.16.sroa.0.014578, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !27508
  %history.i311.i.sroa.16.sroa.0.0.lcssa = phi <8 x float> [ %history.i311.i.sroa.16.sroa.0.0.copyload, %bb37.i1046 ], [ %history.i311.i.sroa.13.sroa.0.014579, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !27508
  %history.i311.i.sroa.13.sroa.0.0.lcssa = phi <8 x float> [ %history.i311.i.sroa.13.sroa.0.0.copyload, %bb37.i1046 ], [ %history.i311.i.sroa.10.sroa.0.014580, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !27508
  %history.i311.i.sroa.10.sroa.0.0.lcssa = phi <8 x float> [ %history.i311.i.sroa.10.sroa.0.0.copyload, %bb37.i1046 ], [ %history.i311.i.sroa.0.014570, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !27508
  store <8 x float> %history.i311.i.sroa.0.0.lcssa, ptr %hot_left.i1024, align 32, !dbg !27509
  store <8 x float> %history.i311.i.sroa.10.sroa.0.0.lcssa, ptr %history.i311.i.sroa.10.0.hot_left.i1024.sroa_idx, align 32, !dbg !27509
  store <8 x float> %history.i311.i.sroa.13.sroa.0.0.lcssa, ptr %history.i311.i.sroa.13.0.hot_left.i1024.sroa_idx, align 32, !dbg !27509
  store <8 x float> %history.i311.i.sroa.16.sroa.0.0.lcssa, ptr %history.i311.i.sroa.16.0.hot_left.i1024.sroa_idx, align 32, !dbg !27509
  store <8 x float> %history.i311.i.sroa.19.sroa.0.0.lcssa, ptr %history.i311.i.sroa.19.0.hot_left.i1024.sroa_idx, align 32, !dbg !27509
  store <8 x float> %history.i311.i.sroa.22.sroa.0.0.lcssa, ptr %history.i311.i.sroa.22.0.hot_left.i1024.sroa_idx, align 32, !dbg !27509
  store <8 x float> %history.i311.i.sroa.25.sroa.0.0.lcssa, ptr %history.i311.i.sroa.25.0.hot_left.i1024.sroa_idx, align 32, !dbg !27509
  store <8 x float> %history.i311.i.sroa.29.sroa.0.0.lcssa, ptr %history.i311.i.sroa.29.0.hot_left.i1024.sroa_idx, align 32, !dbg !27509
  store <8 x float> %history.i311.i.sroa.32.sroa.0.0.lcssa, ptr %history.i311.i.sroa.32.0.hot_left.i1024.sroa_idx, align 32, !dbg !27509
  store <8 x float> %history.i311.i.sroa.35.sroa.0.0.lcssa, ptr %history.i311.i.sroa.35.0.hot_left.i1024.sroa_idx, align 32, !dbg !27509
  store <8 x float> %history.i311.i.sroa.38.sroa.0.0.lcssa, ptr %history.i311.i.sroa.38.0.hot_left.i1024.sroa_idx, align 32, !dbg !27509
  store <8 x float> %history.i311.i.sroa.41.sroa.0.0.lcssa, ptr %history.i311.i.sroa.41.0.hot_left.i1024.sroa_idx, align 32, !dbg !27509
  %history.i.i926.sroa.0.0.copyload = load <8 x float>, ptr %hot_right.i1023, align 32, !dbg !27510
  %history.i.i926.sroa.10.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i926.sroa.10.0.hot_right.i1023.sroa_idx, align 32, !dbg !27510
  %history.i.i926.sroa.13.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i926.sroa.13.0.hot_right.i1023.sroa_idx, align 32, !dbg !27510
  %history.i.i926.sroa.16.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i926.sroa.16.0.hot_right.i1023.sroa_idx, align 32, !dbg !27510
  %history.i.i926.sroa.19.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i926.sroa.19.0.hot_right.i1023.sroa_idx, align 32, !dbg !27510
  %history.i.i926.sroa.22.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i926.sroa.22.0.hot_right.i1023.sroa_idx, align 32, !dbg !27510
  %history.i.i926.sroa.25.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i926.sroa.25.0.hot_right.i1023.sroa_idx, align 32, !dbg !27510
  %history.i.i926.sroa.29.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i926.sroa.29.0.hot_right.i1023.sroa_idx, align 32, !dbg !27510
  %history.i.i926.sroa.32.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i926.sroa.32.0.hot_right.i1023.sroa_idx, align 32, !dbg !27510
  %history.i.i926.sroa.35.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i926.sroa.35.0.hot_right.i1023.sroa_idx, align 32, !dbg !27510
  %history.i.i926.sroa.38.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i926.sroa.38.0.hot_right.i1023.sroa_idx, align 32, !dbg !27510
  %history.i.i926.sroa.41.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i926.sroa.41.0.hot_right.i1023.sroa_idx, align 32, !dbg !27510
  br i1 %_20.i314.i14569.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1053, label %bb5.i.i1215.lr.ph, !dbg !27512

bb5.i.i1215.lr.ph:                                ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit346.i
  %_5.i4083 = load <8 x float>, ptr %self, align 32
  %_14.i.i.i.i891.sroa.0.0.copyload = load <8 x float>, ptr %95, align 32
  %_17.i.i.i.i888.sroa.0.0.copyload = load <8 x float>, ptr %96, align 32
  %_20.i.i.i.i885.sroa.0.0.copyload = load <8 x float>, ptr %97, align 32
  %_25.i.i.i.i881.sroa.0.0.copyload = load <8 x float>, ptr %row12.i.i.i322.i, align 32
  %_28.i.i.i.i878.sroa.0.0.copyload = load <8 x float>, ptr %98, align 32
  %_31.i.i.i.i875.sroa.0.0.copyload = load <8 x float>, ptr %99, align 32
  %_34.i.i.i.i872.sroa.0.0.copyload = load <8 x float>, ptr %100, align 32
  %_39.i.i.i.i868.sroa.0.0.copyload = load <8 x float>, ptr %row13.i.i.i323.i, align 32
  %_42.i.i.i.i865.sroa.0.0.copyload = load <8 x float>, ptr %101, align 32
  %_45.i.i.i.i862.sroa.0.0.copyload = load <8 x float>, ptr %102, align 32
  %_48.i.i.i.i859.sroa.0.0.copyload = load <8 x float>, ptr %103, align 32
  %_53.i.i.i.i855.sroa.0.0.copyload = load <8 x float>, ptr %row14.i.i.i324.i, align 32
  %_56.i.i.i.i852.sroa.0.0.copyload = load <8 x float>, ptr %104, align 32
  %_59.i.i.i.i849.sroa.0.0.copyload = load <8 x float>, ptr %105, align 32
  %_62.i.i.i.i846.sroa.0.0.copyload = load <8 x float>, ptr %106, align 32
  %_67.i.i.i.i842.sroa.0.0.copyload = load <8 x float>, ptr %row15.i.i.i325.i, align 32
  %_70.i.i.i.i839.sroa.0.0.copyload = load <8 x float>, ptr %107, align 32
  %_73.i.i.i.i836.sroa.0.0.copyload = load <8 x float>, ptr %108, align 32
  %_76.i.i.i.i833.sroa.0.0.copyload = load <8 x float>, ptr %109, align 32
  %_81.i.i.i.i829.sroa.0.0.copyload = load <8 x float>, ptr %row16.i.i.i326.i, align 32
  %_84.i.i.i.i826.sroa.0.0.copyload = load <8 x float>, ptr %110, align 32
  %_87.i.i.i.i823.sroa.0.0.copyload = load <8 x float>, ptr %111, align 32
  %_90.i.i.i.i820.sroa.0.0.copyload = load <8 x float>, ptr %112, align 32
  %_95.i.i.i.i816.sroa.0.0.copyload = load <8 x float>, ptr %row17.i.i.i327.i, align 32
  %_98.i.i.i.i813.sroa.0.0.copyload = load <8 x float>, ptr %113, align 32
  %_101.i.i.i.i810.sroa.0.0.copyload = load <8 x float>, ptr %114, align 32
  %_104.i.i.i.i807.sroa.0.0.copyload = load <8 x float>, ptr %115, align 32
  %_109.i.i.i.i803.sroa.0.0.copyload = load <8 x float>, ptr %row18.i.i.i328.i, align 32
  %_112.i.i.i.i800.sroa.0.0.copyload = load <8 x float>, ptr %116, align 32
  %_115.i.i.i.i797.sroa.0.0.copyload = load <8 x float>, ptr %117, align 32
  %_118.i.i.i.i794.sroa.0.0.copyload = load <8 x float>, ptr %118, align 32
  %_123.i.i.i.i790.sroa.0.0.copyload = load <8 x float>, ptr %row19.i.i.i329.i, align 32
  %_126.i.i.i.i787.sroa.0.0.copyload = load <8 x float>, ptr %119, align 32
  %_129.i.i.i.i784.sroa.0.0.copyload = load <8 x float>, ptr %120, align 32
  %_132.i.i.i.i781.sroa.0.0.copyload = load <8 x float>, ptr %121, align 32
  %_137.i.i.i.i777.sroa.0.0.copyload = load <8 x float>, ptr %row20.i.i.i330.i, align 32
  %_140.i.i.i.i774.sroa.0.0.copyload = load <8 x float>, ptr %122, align 32
  %_143.i.i.i.i771.sroa.0.0.copyload = load <8 x float>, ptr %123, align 32
  %_146.i.i.i.i768.sroa.0.0.copyload = load <8 x float>, ptr %124, align 32
  %_151.i.i.i.i764.sroa.0.0.copyload = load <8 x float>, ptr %row21.i.i.i331.i, align 32
  %_154.i.i.i.i761.sroa.0.0.copyload = load <8 x float>, ptr %125, align 32
  %_157.i.i.i.i758.sroa.0.0.copyload = load <8 x float>, ptr %126, align 32
  %_160.i.i.i.i755.sroa.0.0.copyload = load <8 x float>, ptr %127, align 32
  %_165.i.i.i.i751.sroa.0.0.copyload = load <8 x float>, ptr %row22.i.i.i332.i, align 32
  %_168.i.i.i.i748.sroa.0.0.copyload = load <8 x float>, ptr %128, align 32
  %_171.i.i.i.i745.sroa.0.0.copyload = load <8 x float>, ptr %129, align 32
  %_174.i.i.i.i742.sroa.0.0.copyload = load <8 x float>, ptr %130, align 32
  br label %bb5.i.i1215, !dbg !27512

bb5.i.i1215:                                      ; preds = %bb5.i.i1215.lr.ph, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084
  %iter.sroa.0.0.i.i105114607 = phi i64 [ 0, %bb5.i.i1215.lr.ph ], [ %288, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084 ]
  %history.i.i926.sroa.10.sroa.0.014606 = phi <8 x float> [ %history.i.i926.sroa.10.sroa.0.0.copyload, %bb5.i.i1215.lr.ph ], [ %history.i.i926.sroa.0.014596, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084 ]
  %history.i.i926.sroa.13.sroa.0.014605 = phi <8 x float> [ %history.i.i926.sroa.13.sroa.0.0.copyload, %bb5.i.i1215.lr.ph ], [ %history.i.i926.sroa.10.sroa.0.014606, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084 ]
  %history.i.i926.sroa.16.sroa.0.014604 = phi <8 x float> [ %history.i.i926.sroa.16.sroa.0.0.copyload, %bb5.i.i1215.lr.ph ], [ %history.i.i926.sroa.13.sroa.0.014605, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084 ]
  %history.i.i926.sroa.19.sroa.0.014603 = phi <8 x float> [ %history.i.i926.sroa.19.sroa.0.0.copyload, %bb5.i.i1215.lr.ph ], [ %history.i.i926.sroa.16.sroa.0.014604, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084 ]
  %history.i.i926.sroa.22.sroa.0.014602 = phi <8 x float> [ %history.i.i926.sroa.22.sroa.0.0.copyload, %bb5.i.i1215.lr.ph ], [ %history.i.i926.sroa.19.sroa.0.014603, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084 ]
  %history.i.i926.sroa.38.sroa.0.014601 = phi <8 x float> [ %history.i.i926.sroa.38.sroa.0.0.copyload, %bb5.i.i1215.lr.ph ], [ %history.i.i926.sroa.35.sroa.0.014600, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084 ]
  %history.i.i926.sroa.35.sroa.0.014600 = phi <8 x float> [ %history.i.i926.sroa.35.sroa.0.0.copyload, %bb5.i.i1215.lr.ph ], [ %history.i.i926.sroa.32.sroa.0.014599, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084 ]
  %history.i.i926.sroa.32.sroa.0.014599 = phi <8 x float> [ %history.i.i926.sroa.32.sroa.0.0.copyload, %bb5.i.i1215.lr.ph ], [ %history.i.i926.sroa.29.sroa.0.014598, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084 ]
  %history.i.i926.sroa.29.sroa.0.014598 = phi <8 x float> [ %history.i.i926.sroa.29.sroa.0.0.copyload, %bb5.i.i1215.lr.ph ], [ %history.i.i926.sroa.25.sroa.0.014597, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084 ]
  %history.i.i926.sroa.25.sroa.0.014597 = phi <8 x float> [ %history.i.i926.sroa.25.sroa.0.0.copyload, %bb5.i.i1215.lr.ph ], [ %history.i.i926.sroa.22.sroa.0.014602, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084 ]
  %history.i.i926.sroa.0.014596 = phi <8 x float> [ %history.i.i926.sroa.0.0.copyload, %bb5.i.i1215.lr.ph ], [ %lanes.i5318.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084 ]
  %288 = add nuw nsw i64 %iter.sroa.0.0.i.i105114607, 1, !dbg !27515
  %_11.i124.i = add nuw nsw i64 %iter.sroa.0.0.i.i105114607, %iter.sroa.0.0.i104114649, !dbg !27518
  %base.i.i1216 = shl i64 %_11.i124.i, 3, !dbg !27518
  %_24.i.i1217 = icmp samesign ugt i64 %base.i.i1216, %right_io.1, !dbg !27519
  br i1 %_24.i.i1217, label %bb7.i.i1242, label %bb8.i.i1218, !dbg !27519, !prof !639

bb8.i.i1218:                                      ; preds = %bb5.i.i1215
  %_27.i.i1219 = sub nuw nsw i64 %right_io.1, %base.i.i1216, !dbg !27522
  %_8.i5321 = icmp samesign ugt i64 %_27.i.i1219, 7, !dbg !27523
  br i1 %_8.i5321, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084, label %bb2.i5322, !dbg !27523, !prof !651

bb2.i5322:                                        ; preds = %bb8.i.i1218
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_27.i.i1219, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !27528, !noalias !27529
  unreachable, !dbg !27528

bb7.i.i1242:                                      ; preds = %bb5.i.i1215
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i.i1216, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_fc26f793d85338b5649d38df0c19e7e0) #30, !dbg !27536, !noalias !27537
  unreachable, !dbg !27536

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084: ; preds = %bb8.i.i1218
  %_31.i125.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %base.i.i1216, !dbg !27538
  %lanes.i5318.sroa.0.0.copyload = load <8 x float>, ptr %_31.i125.i, align 4, !dbg !27540, !alias.scope !27544, !noalias !27548
  %289 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i.i926.sroa.22.sroa.0.014602), !dbg !27550
  %290 = fmul <8 x float> %lanes.i5318.sroa.0.0.copyload, %_5.i4083, !dbg !27557
  %291 = fadd <8 x float> %290, zeroinitializer, !dbg !27563
  %292 = fmul <8 x float> %lanes.i5318.sroa.0.0.copyload, %_14.i.i.i.i891.sroa.0.0.copyload, !dbg !27568
  %293 = fadd <8 x float> %292, zeroinitializer, !dbg !27573
  %294 = fmul <8 x float> %lanes.i5318.sroa.0.0.copyload, %_17.i.i.i.i888.sroa.0.0.copyload, !dbg !27578
  %295 = fadd <8 x float> %294, zeroinitializer, !dbg !27583
  %296 = fmul <8 x float> %lanes.i5318.sroa.0.0.copyload, %_20.i.i.i.i885.sroa.0.0.copyload, !dbg !27588
  %297 = fadd <8 x float> %296, zeroinitializer, !dbg !27593
  %298 = fmul <8 x float> %history.i.i926.sroa.0.014596, %_25.i.i.i.i881.sroa.0.0.copyload, !dbg !27598
  %299 = fadd <8 x float> %291, %298, !dbg !27603
  %300 = fmul <8 x float> %history.i.i926.sroa.0.014596, %_28.i.i.i.i878.sroa.0.0.copyload, !dbg !27608
  %301 = fadd <8 x float> %293, %300, !dbg !27613
  %302 = fmul <8 x float> %history.i.i926.sroa.0.014596, %_31.i.i.i.i875.sroa.0.0.copyload, !dbg !27618
  %303 = fadd <8 x float> %295, %302, !dbg !27623
  %304 = fmul <8 x float> %history.i.i926.sroa.0.014596, %_34.i.i.i.i872.sroa.0.0.copyload, !dbg !27628
  %305 = fadd <8 x float> %297, %304, !dbg !27633
  %306 = fmul <8 x float> %history.i.i926.sroa.10.sroa.0.014606, %_39.i.i.i.i868.sroa.0.0.copyload, !dbg !27638
  %307 = fadd <8 x float> %299, %306, !dbg !27643
  %308 = fmul <8 x float> %history.i.i926.sroa.10.sroa.0.014606, %_42.i.i.i.i865.sroa.0.0.copyload, !dbg !27648
  %309 = fadd <8 x float> %301, %308, !dbg !27653
  %310 = fmul <8 x float> %history.i.i926.sroa.10.sroa.0.014606, %_45.i.i.i.i862.sroa.0.0.copyload, !dbg !27658
  %311 = fadd <8 x float> %303, %310, !dbg !27663
  %312 = fmul <8 x float> %history.i.i926.sroa.10.sroa.0.014606, %_48.i.i.i.i859.sroa.0.0.copyload, !dbg !27668
  %313 = fadd <8 x float> %305, %312, !dbg !27673
  %314 = fmul <8 x float> %history.i.i926.sroa.13.sroa.0.014605, %_53.i.i.i.i855.sroa.0.0.copyload, !dbg !27678
  %315 = fadd <8 x float> %307, %314, !dbg !27683
  %316 = fmul <8 x float> %history.i.i926.sroa.13.sroa.0.014605, %_56.i.i.i.i852.sroa.0.0.copyload, !dbg !27688
  %317 = fadd <8 x float> %309, %316, !dbg !27693
  %318 = fmul <8 x float> %history.i.i926.sroa.13.sroa.0.014605, %_59.i.i.i.i849.sroa.0.0.copyload, !dbg !27698
  %319 = fadd <8 x float> %311, %318, !dbg !27703
  %320 = fmul <8 x float> %history.i.i926.sroa.13.sroa.0.014605, %_62.i.i.i.i846.sroa.0.0.copyload, !dbg !27708
  %321 = fadd <8 x float> %313, %320, !dbg !27713
  %322 = fmul <8 x float> %history.i.i926.sroa.16.sroa.0.014604, %_67.i.i.i.i842.sroa.0.0.copyload, !dbg !27718
  %323 = fadd <8 x float> %315, %322, !dbg !27723
  %324 = fmul <8 x float> %history.i.i926.sroa.16.sroa.0.014604, %_70.i.i.i.i839.sroa.0.0.copyload, !dbg !27728
  %325 = fadd <8 x float> %317, %324, !dbg !27733
  %326 = fmul <8 x float> %history.i.i926.sroa.16.sroa.0.014604, %_73.i.i.i.i836.sroa.0.0.copyload, !dbg !27738
  %327 = fadd <8 x float> %319, %326, !dbg !27743
  %328 = fmul <8 x float> %history.i.i926.sroa.16.sroa.0.014604, %_76.i.i.i.i833.sroa.0.0.copyload, !dbg !27748
  %329 = fadd <8 x float> %321, %328, !dbg !27753
  %330 = fmul <8 x float> %history.i.i926.sroa.19.sroa.0.014603, %_81.i.i.i.i829.sroa.0.0.copyload, !dbg !27758
  %331 = fadd <8 x float> %323, %330, !dbg !27763
  %332 = fmul <8 x float> %history.i.i926.sroa.19.sroa.0.014603, %_84.i.i.i.i826.sroa.0.0.copyload, !dbg !27768
  %333 = fadd <8 x float> %325, %332, !dbg !27773
  %334 = fmul <8 x float> %history.i.i926.sroa.19.sroa.0.014603, %_87.i.i.i.i823.sroa.0.0.copyload, !dbg !27778
  %335 = fadd <8 x float> %327, %334, !dbg !27783
  %336 = fmul <8 x float> %history.i.i926.sroa.19.sroa.0.014603, %_90.i.i.i.i820.sroa.0.0.copyload, !dbg !27788
  %337 = fadd <8 x float> %329, %336, !dbg !27793
  %338 = fmul <8 x float> %history.i.i926.sroa.22.sroa.0.014602, %_95.i.i.i.i816.sroa.0.0.copyload, !dbg !27798
  %339 = fadd <8 x float> %331, %338, !dbg !27803
  %340 = fmul <8 x float> %history.i.i926.sroa.22.sroa.0.014602, %_98.i.i.i.i813.sroa.0.0.copyload, !dbg !27808
  %341 = fadd <8 x float> %333, %340, !dbg !27813
  %342 = fmul <8 x float> %history.i.i926.sroa.22.sroa.0.014602, %_101.i.i.i.i810.sroa.0.0.copyload, !dbg !27818
  %343 = fadd <8 x float> %335, %342, !dbg !27823
  %344 = fmul <8 x float> %history.i.i926.sroa.22.sroa.0.014602, %_104.i.i.i.i807.sroa.0.0.copyload, !dbg !27828
  %345 = fadd <8 x float> %337, %344, !dbg !27833
  %346 = fmul <8 x float> %history.i.i926.sroa.25.sroa.0.014597, %_109.i.i.i.i803.sroa.0.0.copyload, !dbg !27838
  %347 = fadd <8 x float> %339, %346, !dbg !27843
  %348 = fmul <8 x float> %history.i.i926.sroa.25.sroa.0.014597, %_112.i.i.i.i800.sroa.0.0.copyload, !dbg !27848
  %349 = fadd <8 x float> %341, %348, !dbg !27853
  %350 = fmul <8 x float> %history.i.i926.sroa.25.sroa.0.014597, %_115.i.i.i.i797.sroa.0.0.copyload, !dbg !27858
  %351 = fadd <8 x float> %343, %350, !dbg !27863
  %352 = fmul <8 x float> %history.i.i926.sroa.25.sroa.0.014597, %_118.i.i.i.i794.sroa.0.0.copyload, !dbg !27868
  %353 = fadd <8 x float> %345, %352, !dbg !27873
  %354 = fmul <8 x float> %history.i.i926.sroa.29.sroa.0.014598, %_123.i.i.i.i790.sroa.0.0.copyload, !dbg !27878
  %355 = fadd <8 x float> %347, %354, !dbg !27883
  %356 = fmul <8 x float> %history.i.i926.sroa.29.sroa.0.014598, %_126.i.i.i.i787.sroa.0.0.copyload, !dbg !27888
  %357 = fadd <8 x float> %349, %356, !dbg !27893
  %358 = fmul <8 x float> %history.i.i926.sroa.29.sroa.0.014598, %_129.i.i.i.i784.sroa.0.0.copyload, !dbg !27898
  %359 = fadd <8 x float> %351, %358, !dbg !27903
  %360 = fmul <8 x float> %history.i.i926.sroa.29.sroa.0.014598, %_132.i.i.i.i781.sroa.0.0.copyload, !dbg !27908
  %361 = fadd <8 x float> %353, %360, !dbg !27913
  %362 = fmul <8 x float> %history.i.i926.sroa.32.sroa.0.014599, %_137.i.i.i.i777.sroa.0.0.copyload, !dbg !27918
  %363 = fadd <8 x float> %355, %362, !dbg !27923
  %364 = fmul <8 x float> %history.i.i926.sroa.32.sroa.0.014599, %_140.i.i.i.i774.sroa.0.0.copyload, !dbg !27928
  %365 = fadd <8 x float> %357, %364, !dbg !27933
  %366 = fmul <8 x float> %history.i.i926.sroa.32.sroa.0.014599, %_143.i.i.i.i771.sroa.0.0.copyload, !dbg !27938
  %367 = fadd <8 x float> %359, %366, !dbg !27943
  %368 = fmul <8 x float> %history.i.i926.sroa.32.sroa.0.014599, %_146.i.i.i.i768.sroa.0.0.copyload, !dbg !27948
  %369 = fadd <8 x float> %361, %368, !dbg !27953
  %370 = fmul <8 x float> %history.i.i926.sroa.35.sroa.0.014600, %_151.i.i.i.i764.sroa.0.0.copyload, !dbg !27958
  %371 = fadd <8 x float> %363, %370, !dbg !27963
  %372 = fmul <8 x float> %history.i.i926.sroa.35.sroa.0.014600, %_154.i.i.i.i761.sroa.0.0.copyload, !dbg !27968
  %373 = fadd <8 x float> %365, %372, !dbg !27973
  %374 = fmul <8 x float> %history.i.i926.sroa.35.sroa.0.014600, %_157.i.i.i.i758.sroa.0.0.copyload, !dbg !27978
  %375 = fadd <8 x float> %367, %374, !dbg !27983
  %376 = fmul <8 x float> %history.i.i926.sroa.35.sroa.0.014600, %_160.i.i.i.i755.sroa.0.0.copyload, !dbg !27988
  %377 = fadd <8 x float> %369, %376, !dbg !27993
  %378 = fmul <8 x float> %history.i.i926.sroa.38.sroa.0.014601, %_165.i.i.i.i751.sroa.0.0.copyload, !dbg !27998
  %379 = fadd <8 x float> %371, %378, !dbg !28003
  %380 = fmul <8 x float> %history.i.i926.sroa.38.sroa.0.014601, %_168.i.i.i.i748.sroa.0.0.copyload, !dbg !28008
  %381 = fadd <8 x float> %373, %380, !dbg !28013
  %382 = fmul <8 x float> %history.i.i926.sroa.38.sroa.0.014601, %_171.i.i.i.i745.sroa.0.0.copyload, !dbg !28018
  %383 = fadd <8 x float> %375, %382, !dbg !28023
  %384 = fmul <8 x float> %history.i.i926.sroa.38.sroa.0.014601, %_174.i.i.i.i742.sroa.0.0.copyload, !dbg !28028
  %385 = fadd <8 x float> %377, %384, !dbg !28033
  %386 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %379), !dbg !28038
  %387 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %289, <8 x float> %386), !dbg !28044
  %388 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %381), !dbg !28038
  %389 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %387, <8 x float> %388), !dbg !28044
  %390 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %383), !dbg !28038
  %391 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %389, <8 x float> %390), !dbg !28044
  %392 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %385), !dbg !28038
  %393 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %391, <8 x float> %392), !dbg !28044
  %_39.i.i1239.idx = shl i64 %iter.sroa.0.0.i.i105114607, 5, !dbg !28049
  %_39.i.i1239 = getelementptr inbounds nuw i8, ptr %peaks_right.i1015, i64 %_39.i.i1239.idx, !dbg !28049
  store <8 x float> %393, ptr %_39.i.i1239, align 4, !dbg !28054, !alias.scope !28059, !noalias !28063
  %exitcond17450.not = icmp eq i64 %288, %umax17478, !dbg !28067
  br i1 %exitcond17450.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1053, label %bb5.i.i1215, !dbg !27512

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1053: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit346.i
  %history.i.i926.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i926.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit346.i ], [ %lanes.i5318.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084 ], !dbg !28069
  %history.i.i926.sroa.25.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i926.sroa.25.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit346.i ], [ %history.i.i926.sroa.22.sroa.0.014602, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084 ], !dbg !28069
  %history.i.i926.sroa.29.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i926.sroa.29.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit346.i ], [ %history.i.i926.sroa.25.sroa.0.014597, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084 ], !dbg !28069
  %history.i.i926.sroa.32.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i926.sroa.32.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit346.i ], [ %history.i.i926.sroa.29.sroa.0.014598, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084 ], !dbg !28069
  %history.i.i926.sroa.35.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i926.sroa.35.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit346.i ], [ %history.i.i926.sroa.32.sroa.0.014599, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084 ], !dbg !28069
  %history.i.i926.sroa.38.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i926.sroa.38.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit346.i ], [ %history.i.i926.sroa.35.sroa.0.014600, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084 ], !dbg !28069
  %history.i.i926.sroa.41.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i926.sroa.41.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit346.i ], [ %history.i.i926.sroa.38.sroa.0.014601, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084 ], !dbg !28069
  %history.i.i926.sroa.22.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i926.sroa.22.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit346.i ], [ %history.i.i926.sroa.19.sroa.0.014603, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084 ], !dbg !28069
  %history.i.i926.sroa.19.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i926.sroa.19.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit346.i ], [ %history.i.i926.sroa.16.sroa.0.014604, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084 ], !dbg !28069
  %history.i.i926.sroa.16.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i926.sroa.16.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit346.i ], [ %history.i.i926.sroa.13.sroa.0.014605, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084 ], !dbg !28069
  %history.i.i926.sroa.13.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i926.sroa.13.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit346.i ], [ %history.i.i926.sroa.10.sroa.0.014606, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084 ], !dbg !28069
  %history.i.i926.sroa.10.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i926.sroa.10.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit346.i ], [ %history.i.i926.sroa.0.014596, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6084 ], !dbg !28069
  store <8 x float> %history.i.i926.sroa.0.0.lcssa, ptr %hot_right.i1023, align 32, !dbg !28070
  store <8 x float> %history.i.i926.sroa.10.sroa.0.0.lcssa, ptr %history.i.i926.sroa.10.0.hot_right.i1023.sroa_idx, align 32, !dbg !28070
  store <8 x float> %history.i.i926.sroa.13.sroa.0.0.lcssa, ptr %history.i.i926.sroa.13.0.hot_right.i1023.sroa_idx, align 32, !dbg !28070
  store <8 x float> %history.i.i926.sroa.16.sroa.0.0.lcssa, ptr %history.i.i926.sroa.16.0.hot_right.i1023.sroa_idx, align 32, !dbg !28070
  store <8 x float> %history.i.i926.sroa.19.sroa.0.0.lcssa, ptr %history.i.i926.sroa.19.0.hot_right.i1023.sroa_idx, align 32, !dbg !28070
  store <8 x float> %history.i.i926.sroa.22.sroa.0.0.lcssa, ptr %history.i.i926.sroa.22.0.hot_right.i1023.sroa_idx, align 32, !dbg !28070
  store <8 x float> %history.i.i926.sroa.25.sroa.0.0.lcssa, ptr %history.i.i926.sroa.25.0.hot_right.i1023.sroa_idx, align 32, !dbg !28070
  store <8 x float> %history.i.i926.sroa.29.sroa.0.0.lcssa, ptr %history.i.i926.sroa.29.0.hot_right.i1023.sroa_idx, align 32, !dbg !28070
  store <8 x float> %history.i.i926.sroa.32.sroa.0.0.lcssa, ptr %history.i.i926.sroa.32.0.hot_right.i1023.sroa_idx, align 32, !dbg !28070
  store <8 x float> %history.i.i926.sroa.35.sroa.0.0.lcssa, ptr %history.i.i926.sroa.35.0.hot_right.i1023.sroa_idx, align 32, !dbg !28070
  store <8 x float> %history.i.i926.sroa.38.sroa.0.0.lcssa, ptr %history.i.i926.sroa.38.0.hot_right.i1023.sroa_idx, align 32, !dbg !28070
  store <8 x float> %history.i.i926.sroa.41.sroa.0.0.lcssa, ptr %history.i.i926.sroa.41.0.hot_right.i1023.sroa_idx, align 32, !dbg !28070
  br i1 %_20.i314.i14569.not, label %bb13.i1040.loopexit, label %bb40.i1059.preheader, !dbg !28071

bb40.i1059.preheader:                             ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1053
  %_5.i2056.sroa.0.0.copyload.pre = load <8 x float>, ptr %131, align 32, !dbg !28077
  %_11.i2050.sroa.0.0.copyload.pre = load <8 x float>, ptr %_63.i1062, align 32, !dbg !28078
  %_12.i2049.sroa.0.0.copyload.pre = load <8 x float>, ptr %132, align 32, !dbg !28079
  %_13.i2048.sroa.0.0.copyload.pre = load <8 x float>, ptr %133, align 32, !dbg !28080
  %_5.i2042.sroa.0.0.copyload.pre = load <8 x float>, ptr %134, align 32, !dbg !28081
  %_11.i2036.sroa.0.0.copyload.pre = load <8 x float>, ptr %_64.i1063, align 32, !dbg !28082
  %_12.i2035.sroa.0.0.copyload.pre = load <8 x float>, ptr %135, align 32, !dbg !28083
  %_13.i2034.sroa.0.0.copyload.pre = load <8 x float>, ptr %136, align 32, !dbg !28084
  %_5.i2028.sroa.0.0.copyload.pre = load <8 x float>, ptr %137, align 32, !dbg !28085
  %_11.i2022.sroa.0.0.copyload.pre = load <8 x float>, ptr %_68.i1064, align 32, !dbg !28086
  %_12.i2021.sroa.0.0.copyload.pre = load <8 x float>, ptr %138, align 32, !dbg !28087
  %_13.i2020.sroa.0.0.copyload.pre = load <8 x float>, ptr %139, align 32, !dbg !28088
  %_5.i2016.sroa.0.0.copyload.pre = load <8 x float>, ptr %140, align 32, !dbg !28089
  %_69.i1065.promoted = load <8 x float>, ptr %_69.i1065, align 32
  %.promoted20015 = load <8 x float>, ptr %141, align 32
  %.promoted20052 = load <8 x float>, ptr %155, align 32
  %.promoted20054 = load <8 x float>, ptr %173, align 32
  br label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5407

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5407: ; preds = %bb40.i1059.preheader, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6089
  %_58.i.i978.sroa.0.0.copyload20055 = phi <8 x float> [ %.promoted20054, %bb40.i1059.preheader ], [ %548, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6089 ]
  %_58.i46.i945.sroa.0.0.copyload20053 = phi <8 x float> [ %.promoted20052, %bb40.i1059.preheader ], [ %461, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6089 ]
  %_12.i2013.sroa.0.0.copyload20016 = phi <8 x float> [ %.promoted20015, %bb40.i1059.preheader ], [ %426, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6089 ], !dbg !28090
  %_11.i2014.sroa.0.0.copyload19979 = phi <8 x float> [ %_69.i1065.promoted, %bb40.i1059.preheader ], [ %425, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6089 ], !dbg !28090
  %_5.i2016.sroa.0.0.copyload = phi <8 x float> [ %_5.i2016.sroa.0.0.copyload.pre, %bb40.i1059.preheader ], [ %420, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6089 ], !dbg !28089
  %_12.i2021.sroa.0.0.copyload = phi <8 x float> [ %_12.i2021.sroa.0.0.copyload.pre, %bb40.i1059.preheader ], [ %417, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6089 ], !dbg !28087
  %_11.i2022.sroa.0.0.copyload = phi <8 x float> [ %_11.i2022.sroa.0.0.copyload.pre, %bb40.i1059.preheader ], [ %416, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6089 ], !dbg !28086
  %_5.i2028.sroa.0.0.copyload = phi <8 x float> [ %_5.i2028.sroa.0.0.copyload.pre, %bb40.i1059.preheader ], [ %411, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6089 ], !dbg !28085
  %_12.i2035.sroa.0.0.copyload = phi <8 x float> [ %_12.i2035.sroa.0.0.copyload.pre, %bb40.i1059.preheader ], [ %409, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6089 ], !dbg !28083
  %_11.i2036.sroa.0.0.copyload = phi <8 x float> [ %_11.i2036.sroa.0.0.copyload.pre, %bb40.i1059.preheader ], [ %408, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6089 ], !dbg !28082
  %_5.i2042.sroa.0.0.copyload = phi <8 x float> [ %_5.i2042.sroa.0.0.copyload.pre, %bb40.i1059.preheader ], [ %403, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6089 ], !dbg !28081
  %_12.i2049.sroa.0.0.copyload = phi <8 x float> [ %_12.i2049.sroa.0.0.copyload.pre, %bb40.i1059.preheader ], [ %401, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6089 ], !dbg !28079
  %_11.i2050.sroa.0.0.copyload = phi <8 x float> [ %_11.i2050.sroa.0.0.copyload.pre, %bb40.i1059.preheader ], [ %400, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6089 ], !dbg !28078
  %_5.i2056.sroa.0.0.copyload = phi <8 x float> [ %_5.i2056.sroa.0.0.copyload.pre, %bb40.i1059.preheader ], [ %395, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6089 ], !dbg !28077
  %main_cursor.sroa.0.1.i105714645 = phi i64 [ %main_cursor.sroa.0.0.i104414652, %bb40.i1059.preheader ], [ %spec.store.select.i1199, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6089 ]
  %ring_cursor.sroa.0.1.i105614644 = phi i64 [ %ring_cursor.sroa.0.0.i104314651, %bb40.i1059.preheader ], [ %spec.store.select13.i1201, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6089 ]
  %iter3.sroa.0.0.i105514643 = phi i64 [ 0, %bb40.i1059.preheader ], [ %418, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6089 ]
  %394 = fadd <8 x float> %_5.i2056.sroa.0.0.copyload, splat (float -1.000000e+00), !dbg !28090
  %395 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %394, <8 x float> zeroinitializer), !dbg !28095
  %396 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %395, <8 x float> zeroinitializer, i8 30), !dbg !28100
  %397 = fadd <8 x float> %_11.i2050.sroa.0.0.copyload, %_12.i2049.sroa.0.0.copyload, !dbg !28106
  %398 = bitcast <8 x float> %396 to <8 x i32>, !dbg !28111
  %399 = icmp slt <8 x i32> %398, zeroinitializer, !dbg !28115
  %400 = select <8 x i1> %399, <8 x float> %397, <8 x float> %_13.i2048.sroa.0.0.copyload.pre, !dbg !28115
  %401 = select <8 x i1> %399, <8 x float> %_12.i2049.sroa.0.0.copyload, <8 x float> zeroinitializer, !dbg !28117
  %402 = fadd <8 x float> %_5.i2042.sroa.0.0.copyload, splat (float -1.000000e+00), !dbg !28122
  %403 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %402, <8 x float> zeroinitializer), !dbg !28127
  %404 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %403, <8 x float> zeroinitializer, i8 30), !dbg !28132
  %405 = fadd <8 x float> %_11.i2036.sroa.0.0.copyload, %_12.i2035.sroa.0.0.copyload, !dbg !28138
  %406 = bitcast <8 x float> %404 to <8 x i32>, !dbg !28143
  %407 = icmp slt <8 x i32> %406, zeroinitializer, !dbg !28147
  %408 = select <8 x i1> %407, <8 x float> %405, <8 x float> %_13.i2034.sroa.0.0.copyload.pre, !dbg !28147
  %409 = select <8 x i1> %407, <8 x float> %_12.i2035.sroa.0.0.copyload, <8 x float> zeroinitializer, !dbg !28149
  %410 = fadd <8 x float> %_5.i2028.sroa.0.0.copyload, splat (float -1.000000e+00), !dbg !28154
  %411 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %410, <8 x float> zeroinitializer), !dbg !28159
  %412 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %411, <8 x float> zeroinitializer, i8 30), !dbg !28164
  %413 = fadd <8 x float> %_11.i2022.sroa.0.0.copyload, %_12.i2021.sroa.0.0.copyload, !dbg !28170
  %414 = bitcast <8 x float> %412 to <8 x i32>, !dbg !28175
  %415 = icmp slt <8 x i32> %414, zeroinitializer, !dbg !28179
  %416 = select <8 x i1> %415, <8 x float> %413, <8 x float> %_13.i2020.sroa.0.0.copyload.pre, !dbg !28179
  %417 = select <8 x i1> %415, <8 x float> %_12.i2021.sroa.0.0.copyload, <8 x float> zeroinitializer, !dbg !28181
  %418 = add nuw nsw i64 %iter3.sroa.0.0.i105514643, 1, !dbg !28186
  %_59.i1060 = add nuw nsw i64 %iter3.sroa.0.0.i105514643, %iter.sroa.0.0.i104114649, !dbg !28192
  %base.i1061 = shl i64 %_59.i1060, 3, !dbg !28192
  %419 = fadd <8 x float> %_5.i2016.sroa.0.0.copyload, splat (float -1.000000e+00), !dbg !28193
  %420 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %419, <8 x float> zeroinitializer), !dbg !28198
  %421 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %420, <8 x float> zeroinitializer, i8 30), !dbg !28203
  %422 = fadd <8 x float> %_11.i2014.sroa.0.0.copyload19979, %_12.i2013.sroa.0.0.copyload20016, !dbg !28209
  %_13.i2012.sroa.0.0.copyload = load <8 x float>, ptr %142, align 32, !dbg !28214
  %423 = bitcast <8 x float> %421 to <8 x i32>, !dbg !28215
  %424 = icmp slt <8 x i32> %423, zeroinitializer, !dbg !28219
  %425 = select <8 x i1> %424, <8 x float> %422, <8 x float> %_13.i2012.sroa.0.0.copyload, !dbg !28219
  %426 = select <8 x i1> %424, <8 x float> %_12.i2013.sroa.0.0.copyload20016, <8 x float> zeroinitializer, !dbg !28221
  %_73.i1066 = shl i64 %iter3.sroa.0.0.i105514643, 3, !dbg !28226
  %_128.i1070 = getelementptr inbounds nuw float, ptr %peaks_left.i1016, i64 %_73.i1066, !dbg !28228
  %lanes.i5400.sroa.0.0.copyload = load <8 x float>, ptr %_128.i1070, align 4, !dbg !28239, !alias.scope !28244, !noalias !28248
  %_133.i1071 = getelementptr inbounds nuw float, ptr %peaks_right.i1015, i64 %_73.i1066, !dbg !28252
  %lanes.i5391.sroa.0.0.copyload = load <8 x float>, ptr %_133.i1071, align 4, !dbg !28263, !alias.scope !28268, !noalias !28272
  %427 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i5391.sroa.0.0.copyload, <8 x float> %lanes.i5400.sroa.0.0.copyload), !dbg !28276
  %428 = select <8 x i1> %144, <8 x float> %427, <8 x float> %lanes.i5400.sroa.0.0.copyload, !dbg !28282
  %429 = select <8 x i1> %144, <8 x float> %427, <8 x float> %lanes.i5391.sroa.0.0.copyload, !dbg !28288
  %_134.i1072 = icmp samesign ugt i64 %base.i1061, %left_io.1, !dbg !28294
  br i1 %_134.i1072, label %bb44.i1213, label %bb45.i1073, !dbg !28294, !prof !639

bb45.i1073:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5407
  %_137.i1074 = sub nuw nsw i64 %left_io.1, %base.i1061, !dbg !28299
  %_141.i1075 = getelementptr inbounds nuw float, ptr %left_io.0, i64 %base.i1061, !dbg !28300
  %_8.i5385 = icmp samesign ugt i64 %_137.i1074, 7, !dbg !28305
  br i1 %_8.i5385, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5389, label %bb2.i5386, !dbg !28305, !prof !651

bb2.i5386:                                        ; preds = %bb45.i1073
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_137.i1074, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !28310, !noalias !28311
  unreachable, !dbg !28310

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5389: ; preds = %bb45.i1073
  %_86.i1076 = load i64, ptr %145, align 8, !dbg !28315, !alias.scope !26859, !noalias !28316, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !28317), !dbg !28320
  %width.i61.i1077 = load i64, ptr %146, align 8, !dbg !28321, !alias.scope !28323, !noalias !28324, !noundef !12
  %430 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %428, <8 x float> %400, i8 30), !dbg !28333
  %431 = fdiv <8 x float> %400, %428, !dbg !28339
  %432 = bitcast <8 x float> %430 to <8 x i32>, !dbg !28344
  %433 = icmp slt <8 x i32> %432, zeroinitializer, !dbg !28348
  %434 = select <8 x i1> %433, <8 x float> %431, <8 x float> splat (float 1.000000e+00), !dbg !28348
  %_144.1.i62.i1078 = load i64, ptr %147, align 8, !dbg !28350, !alias.scope !28323, !noalias !28324, !noundef !12
  %_22.i63.i1079 = mul i64 %width.i61.i1077, %ring_cursor.sroa.0.1.i105614644, !dbg !28351
  %_92.i64.i1080 = icmp ugt i64 %_22.i63.i1079, %_144.1.i62.i1078, !dbg !28352
  br i1 %_92.i64.i1080, label %bb37.i122.i1212, label %bb38.i65.i1081, !dbg !28352, !prof !639

bb38.i65.i1081:                                   ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5389
  %_95.i67.i1083 = sub nuw i64 %_144.1.i62.i1078, %_22.i63.i1079, !dbg !28355
  %_8.i6121 = icmp samesign ugt i64 %_95.i67.i1083, 7, !dbg !28356
  br i1 %_8.i6121, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6124, label %bb2.i6122, !dbg !28356, !prof !651

bb2.i6122:                                        ; preds = %bb38.i65.i1081
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_95.i67.i1083, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !28361, !noalias !28362
  unreachable, !dbg !28361

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6124: ; preds = %bb38.i65.i1081
  %_144.0.i66.i1082 = load ptr, ptr %148, align 8, !dbg !28350, !alias.scope !28323, !noalias !28324, !nonnull !12, !noundef !12
  %_99.i68.i1084 = getelementptr inbounds nuw float, ptr %_144.0.i66.i1082, i64 %_22.i63.i1079, !dbg !28366
  store <8 x float> %434, ptr %_99.i68.i1084, align 4, !dbg !28368, !alias.scope !28372, !noalias !28376
  tail call void @llvm.experimental.noalias.scope.decl(metadata !28378), !dbg !28381
  %width.i1616 = load i64, ptr %146, align 8, !dbg !28382, !alias.scope !28378, !noalias !28384, !noundef !12
  %435 = icmp eq i64 %width.i1616, 0, !dbg !28386
  br i1 %435, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1724, label %bb32.i1623.lr.ph, !dbg !28386

bb32.i1623.lr.ph:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6124
  %_112.1.i1626 = load i64, ptr %62, align 8, !alias.scope !28378, !noalias !28384, !noundef !12
  %_112.0.i1630 = load ptr, ptr %61, align 8, !nonnull !12
  %436 = add i64 %ring_cursor.sroa.0.1.i105614644, 1
  %_23.not.i1637 = icmp ult i64 %436, %_86.i1076
  %437 = select i1 %_23.not.i1637, i64 0, i64 %_86.i1076
  %start1.sroa.0.0.i1638 = sub nuw i64 %436, %437
  %_114.1.i1641 = load i64, ptr %147, align 8
  %_114.0.i1645 = load ptr, ptr %148, align 8, !nonnull !12
  %_116.1.i1646 = load i64, ptr %149, align 8
  %_116.0.i1650 = load ptr, ptr %150, align 8, !nonnull !12
  %_118.1.i1654 = load i64, ptr %151, align 8
  %_118.0.i1658 = load ptr, ptr %152, align 8, !nonnull !12
  %_45.i1671 = mul i64 %width.i1616, %start1.sroa.0.0.i1638
  br label %bb32.i1623, !dbg !28386

bb32.i1623:                                       ; preds = %bb32.i1623.lr.ph, %bb31.i1686
  %iter.i1615.sroa.10.014625 = phi i64 [ %width.i1616, %bb32.i1623.lr.ph ], [ %438, %bb31.i1686 ]
  %iter.i1615.sroa.7.014624 = phi i64 [ 0, %bb32.i1623.lr.ph ], [ %_9.0.i, %bb31.i1686 ]
  %iter.i1615.sroa.0.0.idx14623 = phi i64 [ 0, %bb32.i1623.lr.ph ], [ %iter.i1615.sroa.0.0.add, %bb31.i1686 ]
  %iter.i1615.sroa.0.0.ptr14626 = getelementptr inbounds nuw i8, ptr %scratch.i1017, i64 %iter.i1615.sroa.0.0.idx14623, !dbg !28388
  %438 = add i64 %iter.i1615.sroa.10.014625, -1, !dbg !28388
  %_7.i.i6669 = icmp eq i64 %iter.i1615.sroa.0.0.idx14623, 32, !dbg !28389
  br i1 %_7.i.i6669, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1724, label %bb3.i1625, !dbg !28393

bb3.i1625:                                        ; preds = %bb32.i1623
  %iter.i1615.sroa.0.0.add = add nuw nsw i64 %iter.i1615.sroa.0.0.idx14623, 4, !dbg !28394
  %_9.0.i = add nuw nsw i64 %iter.i1615.sroa.7.014624, 1, !dbg !28396
  %exitcond17458.not = icmp eq i64 %iter.i1615.sroa.7.014624, %_112.1.i1626, !dbg !28397
  br i1 %exitcond17458.not, label %panic.i1628, label %bb5.i1629, !dbg !28397

bb5.i1629:                                        ; preds = %bb3.i1625
  %439 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i1630, i64 %iter.i1615.sroa.7.014624, !dbg !28397
  %shape.i1631 = load i32, ptr %439, align 4, !dbg !28397, !noalias !28398, !noundef !12
  %440 = getelementptr inbounds nuw i8, ptr %439, i64 4, !dbg !28397
  %shape3.i1632 = load i32, ptr %440, align 4, !dbg !28397, !noalias !28398, !noundef !12
  %window.i1633 = zext i32 %shape.i1631 to i64, !dbg !28399
  %_19.i1634 = zext i32 %shape3.i1632 to i64, !dbg !28400
  %441 = add i64 %ring_cursor.sroa.0.1.i105614644, %_19.i1634, !dbg !28401
  %_20.not.i1635 = icmp ult i64 %441, %_86.i1076, !dbg !28402
  %442 = select i1 %_20.not.i1635, i64 0, i64 %_86.i1076, !dbg !28402
  %spec.select.i1636 = sub nuw i64 %441, %442, !dbg !28402
  %_27.i1639 = mul i64 %spec.select.i1636, %width.i1616, !dbg !28403
  %_26.i1640 = add i64 %_27.i1639, %iter.i1615.sroa.7.014624, !dbg !28403
  %_30.i1642 = icmp ult i64 %_26.i1640, %_114.1.i1641, !dbg !28404
  br i1 %_30.i1642, label %bb12.i1644, label %panic5.i1643, !dbg !28404

panic.i1628:                                      ; preds = %bb3.i1625
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i1626, i64 noundef %_112.1.i1626, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8013bf8450ffb218032f1e27d334efa7) #30, !dbg !28397, !noalias !28398
  unreachable, !dbg !28397

bb12.i1644:                                       ; preds = %bb5.i1629
  %443 = getelementptr inbounds nuw float, ptr %_114.0.i1645, i64 %_26.i1640, !dbg !28404
  %444 = load float, ptr %443, align 4, !dbg !28404, !noalias !28398, !noundef !12
  %exitcond17459.not = icmp eq i64 %iter.i1615.sroa.7.014624, %_116.1.i1646, !dbg !28405
  br i1 %exitcond17459.not, label %panic6.i1648, label %bb13.i1649, !dbg !28405

panic5.i1643:                                     ; preds = %bb5.i1629
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i1640, i64 noundef %_114.1.i1641, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d297cbdfce2474defb1d7f11394e8cb6) #30, !dbg !28404, !noalias !28398
  unreachable, !dbg !28404

bb13.i1649:                                       ; preds = %bb12.i1644
  %445 = getelementptr inbounds nuw i32, ptr %_116.0.i1650, i64 %iter.i1615.sroa.7.014624, !dbg !28405
  %_32.i1651 = load i32, ptr %445, align 4, !dbg !28405, !noalias !28398, !noundef !12
  %position.i1652 = zext i32 %_32.i1651 to i64, !dbg !28405
  %446 = icmp eq i32 %_32.i1651, 0, !dbg !28406
  br i1 %446, label %bb17.i1661, label %bb15.i1653, !dbg !28406

panic6.i1648:                                     ; preds = %bb12.i1644
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i1646, i64 noundef %_116.1.i1646, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ac016620dad255f0d323022a5b7f5883) #30, !dbg !28405, !noalias !28398
  unreachable, !dbg !28405

bb15.i1653:                                       ; preds = %bb13.i1649
  %_37.i1655 = icmp ult i64 %iter.i1615.sroa.7.014624, %_118.1.i1654, !dbg !28407
  br i1 %_37.i1655, label %bb16.i1657, label %panic7.i1656, !dbg !28407

bb17.i1661:                                       ; preds = %bb35.i1722, %bb16.i1657, %bb13.i1649
  %newest.sroa.0.0.i1662 = phi float [ %444, %bb13.i1649 ], [ %_35.i1659, %bb35.i1722 ], [ %444, %bb16.i1657 ], !dbg !28408
  %exitcond17460.not = icmp eq i64 %iter.i1615.sroa.7.014624, %_118.1.i1654, !dbg !28409
  br i1 %exitcond17460.not, label %panic8.i1665, label %bb18.i1666, !dbg !28409

bb16.i1657:                                       ; preds = %bb15.i1653
  %447 = getelementptr inbounds nuw float, ptr %_118.0.i1658, i64 %iter.i1615.sroa.7.014624, !dbg !28407
  %_35.i1659 = load float, ptr %447, align 4, !dbg !28407, !noalias !28398, !noundef !12
  %_102.i1660 = fcmp olt float %_35.i1659, %444, !dbg !28410
  br i1 %_102.i1660, label %bb35.i1722, label %bb17.i1661, !dbg !28410

panic7.i1656:                                     ; preds = %bb15.i1653
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i1615.sroa.7.014624, i64 noundef %_118.1.i1654, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e5355958980a78279123102c3494b10a) #30, !dbg !28407, !noalias !28398
  unreachable, !dbg !28407

bb35.i1722:                                       ; preds = %bb16.i1657
  br label %bb17.i1661, !dbg !28412

bb18.i1666:                                       ; preds = %bb17.i1661
  %448 = getelementptr inbounds nuw float, ptr %_118.0.i1658, i64 %iter.i1615.sroa.7.014624, !dbg !28409
  store float %newest.sroa.0.0.i1662, ptr %448, align 4, !dbg !28409, !noalias !28398
  %_42.i1668 = add nuw nsw i64 %position.i1652, 1, !dbg !28413
  %complete.i1669 = icmp eq i64 %_42.i1668, %window.i1633, !dbg !28413
  br i1 %complete.i1669, label %bb22.i1691, label %bb20.i1670, !dbg !28414

panic8.i1665:                                     ; preds = %bb17.i1661
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i1654, i64 noundef %_118.1.i1654, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2d2a28b8cb03afaaebfea48ffa77c925) #30, !dbg !28409, !noalias !28398
  unreachable, !dbg !28409

bb20.i1670:                                       ; preds = %bb18.i1666
  %_44.i1672 = add i64 %iter.i1615.sroa.7.014624, %_45.i1671, !dbg !28415
  %_47.i1674 = icmp ult i64 %_44.i1672, %_114.1.i1641, !dbg !28416
  br i1 %_47.i1674, label %bb30.i1684, label %panic9.i1675, !dbg !28416

panic9.i1675:                                     ; preds = %bb20.i1670
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i1672, i64 noundef %_114.1.i1641, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_53d3a5c3ecf31ea5f14c0a7f8bf40921) #30, !dbg !28416, !noalias !28398
  unreachable, !dbg !28416

bb30.i1684:                                       ; preds = %bb20.i1670
  %449 = getelementptr inbounds nuw float, ptr %_114.0.i1645, i64 %_44.i1672, !dbg !28416
  %_43.i1678 = load float, ptr %449, align 4, !dbg !28416, !noalias !28398, !noundef !12
  %_103.i1679 = fcmp olt float %_43.i1678, %newest.sroa.0.0.i1662, !dbg !28417
  %newest.sroa.0.1.i1680 = select i1 %_103.i1679, float %_43.i1678, float %newest.sroa.0.0.i1662, !dbg !28417
  store float %newest.sroa.0.1.i1680, ptr %iter.i1615.sroa.0.0.ptr14626, align 4, !dbg !28419, !noalias !28398
  %450 = trunc i64 %_42.i1668 to i32, !dbg !28420
  br label %bb31.i1686, !dbg !28421

bb31.i1686:                                       ; preds = %bb25.i1719, %bb30.i1684
  %storemerge = phi i32 [ %450, %bb30.i1684 ], [ 0, %bb25.i1719 ], !dbg !28422
  store i32 %storemerge, ptr %445, align 4, !dbg !28422, !noalias !28398
  %451 = icmp eq i64 %438, 0, !dbg !28386
  br i1 %451, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1724, label %bb32.i1623, !dbg !28386

bb22.i1691:                                       ; preds = %bb18.i1666
  store float %newest.sroa.0.0.i1662, ptr %iter.i1615.sroa.0.0.ptr14626, align 4, !dbg !28419, !noalias !28398
  %452 = load float, ptr %443, align 4, !dbg !28423, !noalias !28398, !noundef !12
  br label %bb41.i1704, !dbg !28424

bb41.i1704:                                       ; preds = %bb22.i1691, %bb25.i1719
  %iter2.sroa.0.0.i169614622 = phi i64 [ 0, %bb22.i1691 ], [ %_105.i1705, %bb25.i1719 ]
  %suffix.sroa.0.0.i169514621 = phi float [ %452, %bb22.i1691 ], [ %suffix.sroa.0.1.i1715, %bb25.i1719 ]
  %end.sroa.0.1.i169414620 = phi i64 [ %spec.select.i1636, %bb22.i1691 ], [ %455, %bb25.i1719 ]
  %_56.i1706 = mul i64 %end.sroa.0.1.i169414620, %width.i1616, !dbg !28427
  %_55.i1707 = add i64 %_56.i1706, %iter.i1615.sroa.7.014624, !dbg !28427
  %_59.i1709 = icmp ult i64 %_55.i1707, %_114.1.i1641, !dbg !28428
  br i1 %_59.i1709, label %bb25.i1719, label %panic13.i1710, !dbg !28428

panic13.i1710:                                    ; preds = %bb41.i1704
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i1707, i64 noundef %_114.1.i1641, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b22b5c926aed02a79660ac772e5ad40d) #30, !dbg !28428, !noalias !28398
  unreachable, !dbg !28428

bb25.i1719:                                       ; preds = %bb41.i1704
  %_105.i1705 = add nuw nsw i64 %iter2.sroa.0.0.i169614622, 1, !dbg !28429
  %453 = getelementptr inbounds nuw float, ptr %_114.0.i1645, i64 %_55.i1707, !dbg !28428
  %_54.i1713 = load float, ptr %453, align 4, !dbg !28428, !noalias !28398, !noundef !12
  %_107.i1714 = fcmp olt float %suffix.sroa.0.0.i169514621, %_54.i1713, !dbg !28432
  %suffix.sroa.0.1.i1715 = select i1 %_107.i1714, float %suffix.sroa.0.0.i169514621, float %_54.i1713, !dbg !28432
  store float %suffix.sroa.0.1.i1715, ptr %453, align 4, !dbg !28434, !noalias !28398
  %454 = icmp eq i64 %end.sroa.0.1.i169414620, 0, !dbg !28435
  %spec.store.select.i1721 = select i1 %454, i64 %_86.i1076, i64 %end.sroa.0.1.i169414620, !dbg !28435
  %455 = add i64 %spec.store.select.i1721, -1, !dbg !28436
  %exitcond17457.not = icmp eq i64 %_105.i1705, %window.i1633, !dbg !28437
  br i1 %exitcond17457.not, label %bb31.i1686, label %bb41.i1704, !dbg !28424

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1724: ; preds = %bb31.i1686, %bb32.i1623, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6124
  %lanes.i5375.sroa.0.0.copyload = load <8 x float>, ptr %scratch.i1017, align 4, !dbg !28439, !alias.scope !28444, !noalias !28448
  %456 = fmul <8 x float> %lanes.i5375.sroa.0.0.copyload, splat (float 1.638400e+04), !dbg !28452
  %457 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %456), !dbg !28457
  %458 = fmul <8 x float> %457, splat (float 0x3F10000000000000), !dbg !28462
  %459 = icmp eq i64 %width.i61.i1077, 0, !dbg !28467
  %_149.1.i96.i1112.pre = load i64, ptr %153, align 8, !dbg !28469, !alias.scope !28323, !noalias !28324
  br i1 %459, label %bb16.i95.i1111, label %bb39.i75.i1091.lr.ph, !dbg !28467

bb39.i75.i1091.lr.ph:                             ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1724
  %_145.1.i78.i1094 = load i64, ptr %62, align 8, !alias.scope !28323, !noalias !28324, !noundef !12
  %_145.0.i82.i1098 = load ptr, ptr %61, align 8, !nonnull !12
  %_147.0.i93.i1109 = load ptr, ptr %154, align 8, !nonnull !12
  %exitcond17463.not = icmp eq i64 %_145.1.i78.i1094, 0, !dbg !28470
  br i1 %exitcond17463.not, label %panic.i80.i1096, label %bb17.i81.i1097, !dbg !28470

bb37.i122.i1212:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5389
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i63.i1079, i64 noundef %_144.1.i62.i1078, i64 noundef %_144.1.i62.i1078, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_913d17a5751fc2956adecdab98dac09f) #30, !dbg !28471, !noalias !28472
  unreachable, !dbg !28471

bb16.i95.i1111.loopexit:                          ; preds = %bb21.i92.i1108.7, %bb21.i92.i1108.6, %bb21.i92.i1108.5, %bb21.i92.i1108.4, %bb21.i92.i1108.3, %bb21.i92.i1108.2, %bb21.i92.i1108.1, %bb21.i92.i1108
  %lanes.i5368.sroa.0.0.copyload.pre = load <8 x float>, ptr %scratch.i1017, align 4, !dbg !28473, !alias.scope !28478, !noalias !28482
  br label %bb16.i95.i1111, !dbg !28486

bb16.i95.i1111:                                   ; preds = %bb16.i95.i1111.loopexit, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1724
  %lanes.i5368.sroa.0.0.copyload = phi <8 x float> [ %lanes.i5368.sroa.0.0.copyload.pre, %bb16.i95.i1111.loopexit ], [ %lanes.i5375.sroa.0.0.copyload, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1724 ], !dbg !28473
  %460 = fadd <8 x float> %458, %_58.i46.i945.sroa.0.0.copyload20053, !dbg !28487
  %461 = fsub <8 x float> %460, %lanes.i5368.sroa.0.0.copyload, !dbg !28492
  store <8 x float> %461, ptr %155, align 32, !dbg !28497
  %_109.i97.i1113 = icmp ugt i64 %_22.i63.i1079, %_149.1.i96.i1112.pre, !dbg !28498
  br i1 %_109.i97.i1113, label %bb42.i121.i1211, label %bb43.i98.i1114, !dbg !28498, !prof !639

bb43.i98.i1114:                                   ; preds = %bb16.i95.i1111
  %_112.i100.i1116 = sub nuw i64 %_149.1.i96.i1112.pre, %_22.i63.i1079, !dbg !28501
  %_8.i6116 = icmp samesign ugt i64 %_112.i100.i1116, 7, !dbg !28502
  br i1 %_8.i6116, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6119, label %bb2.i6117, !dbg !28502, !prof !651

bb2.i6117:                                        ; preds = %bb43.i98.i1114
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_112.i100.i1116, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !28507, !noalias !28508
  unreachable, !dbg !28507

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6119: ; preds = %bb43.i98.i1114
  %_149.0.i99.i1115 = load ptr, ptr %154, align 8, !dbg !28469, !alias.scope !28323, !noalias !28324, !nonnull !12, !noundef !12
  %_116.i101.i1117 = getelementptr inbounds nuw float, ptr %_149.0.i99.i1115, i64 %_22.i63.i1079, !dbg !28512
  store <8 x float> %458, ptr %_116.i101.i1117, align 4, !dbg !28514, !alias.scope !28518, !noalias !28522
  %_64.i43.i942.sroa.0.0.copyload = load <8 x float>, ptr %156, align 32, !dbg !28524
  %_68.i39.i938.sroa.0.0.copyload = load <8 x float>, ptr %157, align 32, !dbg !28525
  %462 = fdiv <8 x float> %461, %_64.i43.i942.sroa.0.0.copyload, !dbg !28526
  %463 = fsub <8 x float> splat (float 1.000000e+00), %462, !dbg !28531
  %464 = fsub <8 x float> %463, %_68.i39.i938.sroa.0.0.copyload, !dbg !28536
  %465 = fmul <8 x float> %408, %464, !dbg !28541
  %466 = fadd <8 x float> %_68.i39.i938.sroa.0.0.copyload, %465, !dbg !28546
  %467 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %463, <8 x float> %466), !dbg !28550
  %468 = bitcast <8 x float> %467 to <8 x i32>, !dbg !28555
  %469 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %467), !dbg !28561
  %470 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %469, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !28563
  %471 = bitcast <8 x float> %470 to <8 x i32>, !dbg !28569
  %472 = xor <8 x i32> %471, splat (i32 -1), !dbg !28575
  %473 = and <8 x i32> %472, %468, !dbg !28577
  %474 = bitcast <8 x i32> %473 to <8 x float>, !dbg !28581
  store <8 x i32> %473, ptr %157, align 32, !dbg !28582
  %475 = fsub <8 x float> splat (float 1.000000e+00), %474, !dbg !28583
  %_150.1.i102.i1118 = load i64, ptr %158, align 8, !dbg !28588, !alias.scope !28323, !noalias !28324, !noundef !12
  %_76.i103.i1119 = mul i64 %width.i61.i1077, %main_cursor.sroa.0.1.i105714645, !dbg !28589
  %_120.i104.i1120 = icmp ugt i64 %_76.i103.i1119, %_150.1.i102.i1118, !dbg !28590
  br i1 %_120.i104.i1120, label %bb48.i120.i1210, label %bb49.i105.i1121, !dbg !28590, !prof !639

bb42.i121.i1211:                                  ; preds = %bb16.i95.i1111
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i63.i1079, i64 noundef %_149.1.i96.i1112.pre, i64 noundef %_149.1.i96.i1112.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8a0dcf875eae79f6708bcf79c3cbff55) #30, !dbg !28593, !noalias !28594
  unreachable, !dbg !28593

bb49.i105.i1121:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6119
  %_123.i107.i1123 = sub nuw i64 %_150.1.i102.i1118, %_76.i103.i1119, !dbg !28595
  %_8.i5362 = icmp samesign ugt i64 %_123.i107.i1123, 7, !dbg !28596
  br i1 %_8.i5362, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6109, label %bb2.i5363, !dbg !28596, !prof !651

bb2.i5363:                                        ; preds = %bb49.i105.i1121
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_123.i107.i1123, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !28601, !noalias !28602
  unreachable, !dbg !28601

bb48.i120.i1210:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6119
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i103.i1119, i64 noundef %_150.1.i102.i1118, i64 noundef %_150.1.i102.i1118, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e5e0b8406fbb9ac3f5f26ac6469c25f7) #30, !dbg !28606, !noalias !28594
  unreachable, !dbg !28606

bb17.i81.i1097:                                   ; preds = %bb39.i75.i1091.lr.ph
  %476 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i1098, i64 8, !dbg !28470
  %_44.i83.i1099 = load i32, ptr %476, align 4, !dbg !28470, !noalias !28594, !noundef !12
  %_43.i84.i1100 = zext i32 %_44.i83.i1099 to i64, !dbg !28470
  %477 = add i64 %ring_cursor.sroa.0.1.i105614644, %_43.i84.i1100, !dbg !28607
  %_47.not.i85.i1101 = icmp ult i64 %477, %_86.i1076, !dbg !28608
  %478 = select i1 %_47.not.i85.i1101, i64 0, i64 %_86.i1076, !dbg !28608
  %spec.select.i86.i1102 = sub nuw i64 %477, %478, !dbg !28608
  %_51.i87.i1103 = mul i64 %spec.select.i86.i1102, %width.i61.i1077, !dbg !28609
  %_53.i90.i1106 = icmp ult i64 %_51.i87.i1103, %_149.1.i96.i1112.pre, !dbg !28610
  br i1 %_53.i90.i1106, label %bb21.i92.i1108, label %panic1.i91.i1107, !dbg !28610

panic.i80.i1096:                                  ; preds = %bb39.i75.i1091.7, %bb39.i75.i1091.6, %bb39.i75.i1091.5, %bb39.i75.i1091.4, %bb39.i75.i1091.3, %bb39.i75.i1091.2, %bb39.i75.i1091.1, %bb39.i75.i1091.lr.ph
  %_145.1.i78.i1094.lcssa.ph = phi i64 [ 7, %bb39.i75.i1091.7 ], [ 6, %bb39.i75.i1091.6 ], [ 5, %bb39.i75.i1091.5 ], [ 4, %bb39.i75.i1091.4 ], [ 3, %bb39.i75.i1091.3 ], [ 2, %bb39.i75.i1091.2 ], [ 1, %bb39.i75.i1091.1 ], [ 0, %bb39.i75.i1091.lr.ph ]
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i78.i1094.lcssa.ph, i64 noundef %_145.1.i78.i1094.lcssa.ph, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7ff2ed40a8d2df224a47a35441e5e2fe) #30, !dbg !28470, !noalias !28594
  unreachable, !dbg !28470

bb21.i92.i1108:                                   ; preds = %bb17.i81.i1097
  %479 = getelementptr inbounds nuw float, ptr %_147.0.i93.i1109, i64 %_51.i87.i1103, !dbg !28610
  %_49.i94.i1110 = load float, ptr %479, align 4, !dbg !28610, !noalias !28594, !noundef !12
  store float %_49.i94.i1110, ptr %scratch.i1017, align 4, !dbg !28611, !noalias !28594
  %480 = icmp eq i64 %width.i61.i1077, 1, !dbg !28467
  br i1 %480, label %bb16.i95.i1111.loopexit, label %bb39.i75.i1091.1, !dbg !28467

bb39.i75.i1091.1:                                 ; preds = %bb21.i92.i1108
  %exitcond17463.1.not = icmp eq i64 %_145.1.i78.i1094, 1, !dbg !28470
  br i1 %exitcond17463.1.not, label %panic.i80.i1096, label %bb17.i81.i1097.1, !dbg !28470

bb17.i81.i1097.1:                                 ; preds = %bb39.i75.i1091.1
  %481 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i1098, i64 20, !dbg !28470
  %_44.i83.i1099.1 = load i32, ptr %481, align 4, !dbg !28470, !noalias !28594, !noundef !12
  %_43.i84.i1100.1 = zext i32 %_44.i83.i1099.1 to i64, !dbg !28470
  %482 = add i64 %ring_cursor.sroa.0.1.i105614644, %_43.i84.i1100.1, !dbg !28607
  %_47.not.i85.i1101.1 = icmp ult i64 %482, %_86.i1076, !dbg !28608
  %483 = select i1 %_47.not.i85.i1101.1, i64 0, i64 %_86.i1076, !dbg !28608
  %spec.select.i86.i1102.1 = sub nuw i64 %482, %483, !dbg !28608
  %_51.i87.i1103.1 = mul i64 %spec.select.i86.i1102.1, %width.i61.i1077, !dbg !28609
  %_50.i88.i1104.1 = add i64 %_51.i87.i1103.1, 1, !dbg !28609
  %_53.i90.i1106.1 = icmp ult i64 %_50.i88.i1104.1, %_149.1.i96.i1112.pre, !dbg !28610
  br i1 %_53.i90.i1106.1, label %bb21.i92.i1108.1, label %panic1.i91.i1107, !dbg !28610

bb21.i92.i1108.1:                                 ; preds = %bb17.i81.i1097.1
  %484 = getelementptr inbounds nuw float, ptr %_147.0.i93.i1109, i64 %_50.i88.i1104.1, !dbg !28610
  %_49.i94.i1110.1 = load float, ptr %484, align 4, !dbg !28610, !noalias !28594, !noundef !12
  store float %_49.i94.i1110.1, ptr %iter.i49.i948.sroa.0.0.ptr14630.1, align 4, !dbg !28611, !noalias !28594
  %485 = icmp eq i64 %width.i61.i1077, 2, !dbg !28467
  br i1 %485, label %bb16.i95.i1111.loopexit, label %bb39.i75.i1091.2, !dbg !28467

bb39.i75.i1091.2:                                 ; preds = %bb21.i92.i1108.1
  %exitcond17463.2.not = icmp eq i64 %_145.1.i78.i1094, 2, !dbg !28470
  br i1 %exitcond17463.2.not, label %panic.i80.i1096, label %bb17.i81.i1097.2, !dbg !28470

bb17.i81.i1097.2:                                 ; preds = %bb39.i75.i1091.2
  %486 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i1098, i64 32, !dbg !28470
  %_44.i83.i1099.2 = load i32, ptr %486, align 4, !dbg !28470, !noalias !28594, !noundef !12
  %_43.i84.i1100.2 = zext i32 %_44.i83.i1099.2 to i64, !dbg !28470
  %487 = add i64 %ring_cursor.sroa.0.1.i105614644, %_43.i84.i1100.2, !dbg !28607
  %_47.not.i85.i1101.2 = icmp ult i64 %487, %_86.i1076, !dbg !28608
  %488 = select i1 %_47.not.i85.i1101.2, i64 0, i64 %_86.i1076, !dbg !28608
  %spec.select.i86.i1102.2 = sub nuw i64 %487, %488, !dbg !28608
  %_51.i87.i1103.2 = mul i64 %spec.select.i86.i1102.2, %width.i61.i1077, !dbg !28609
  %_50.i88.i1104.2 = add i64 %_51.i87.i1103.2, 2, !dbg !28609
  %_53.i90.i1106.2 = icmp ult i64 %_50.i88.i1104.2, %_149.1.i96.i1112.pre, !dbg !28610
  br i1 %_53.i90.i1106.2, label %bb21.i92.i1108.2, label %panic1.i91.i1107, !dbg !28610

bb21.i92.i1108.2:                                 ; preds = %bb17.i81.i1097.2
  %489 = getelementptr inbounds nuw float, ptr %_147.0.i93.i1109, i64 %_50.i88.i1104.2, !dbg !28610
  %_49.i94.i1110.2 = load float, ptr %489, align 4, !dbg !28610, !noalias !28594, !noundef !12
  store float %_49.i94.i1110.2, ptr %iter.i49.i948.sroa.0.0.ptr14630.2, align 4, !dbg !28611, !noalias !28594
  %490 = icmp eq i64 %width.i61.i1077, 3, !dbg !28467
  br i1 %490, label %bb16.i95.i1111.loopexit, label %bb39.i75.i1091.3, !dbg !28467

bb39.i75.i1091.3:                                 ; preds = %bb21.i92.i1108.2
  %exitcond17463.3.not = icmp eq i64 %_145.1.i78.i1094, 3, !dbg !28470
  br i1 %exitcond17463.3.not, label %panic.i80.i1096, label %bb17.i81.i1097.3, !dbg !28470

bb17.i81.i1097.3:                                 ; preds = %bb39.i75.i1091.3
  %491 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i1098, i64 44, !dbg !28470
  %_44.i83.i1099.3 = load i32, ptr %491, align 4, !dbg !28470, !noalias !28594, !noundef !12
  %_43.i84.i1100.3 = zext i32 %_44.i83.i1099.3 to i64, !dbg !28470
  %492 = add i64 %ring_cursor.sroa.0.1.i105614644, %_43.i84.i1100.3, !dbg !28607
  %_47.not.i85.i1101.3 = icmp ult i64 %492, %_86.i1076, !dbg !28608
  %493 = select i1 %_47.not.i85.i1101.3, i64 0, i64 %_86.i1076, !dbg !28608
  %spec.select.i86.i1102.3 = sub nuw i64 %492, %493, !dbg !28608
  %_51.i87.i1103.3 = mul i64 %spec.select.i86.i1102.3, %width.i61.i1077, !dbg !28609
  %_50.i88.i1104.3 = add i64 %_51.i87.i1103.3, 3, !dbg !28609
  %_53.i90.i1106.3 = icmp ult i64 %_50.i88.i1104.3, %_149.1.i96.i1112.pre, !dbg !28610
  br i1 %_53.i90.i1106.3, label %bb21.i92.i1108.3, label %panic1.i91.i1107, !dbg !28610

bb21.i92.i1108.3:                                 ; preds = %bb17.i81.i1097.3
  %494 = getelementptr inbounds nuw float, ptr %_147.0.i93.i1109, i64 %_50.i88.i1104.3, !dbg !28610
  %_49.i94.i1110.3 = load float, ptr %494, align 4, !dbg !28610, !noalias !28594, !noundef !12
  store float %_49.i94.i1110.3, ptr %iter.i49.i948.sroa.0.0.ptr14630.3, align 4, !dbg !28611, !noalias !28594
  %495 = icmp eq i64 %width.i61.i1077, 4, !dbg !28467
  br i1 %495, label %bb16.i95.i1111.loopexit, label %bb39.i75.i1091.4, !dbg !28467

bb39.i75.i1091.4:                                 ; preds = %bb21.i92.i1108.3
  %exitcond17463.4.not = icmp eq i64 %_145.1.i78.i1094, 4, !dbg !28470
  br i1 %exitcond17463.4.not, label %panic.i80.i1096, label %bb17.i81.i1097.4, !dbg !28470

bb17.i81.i1097.4:                                 ; preds = %bb39.i75.i1091.4
  %496 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i1098, i64 56, !dbg !28470
  %_44.i83.i1099.4 = load i32, ptr %496, align 4, !dbg !28470, !noalias !28594, !noundef !12
  %_43.i84.i1100.4 = zext i32 %_44.i83.i1099.4 to i64, !dbg !28470
  %497 = add i64 %ring_cursor.sroa.0.1.i105614644, %_43.i84.i1100.4, !dbg !28607
  %_47.not.i85.i1101.4 = icmp ult i64 %497, %_86.i1076, !dbg !28608
  %498 = select i1 %_47.not.i85.i1101.4, i64 0, i64 %_86.i1076, !dbg !28608
  %spec.select.i86.i1102.4 = sub nuw i64 %497, %498, !dbg !28608
  %_51.i87.i1103.4 = mul i64 %spec.select.i86.i1102.4, %width.i61.i1077, !dbg !28609
  %_50.i88.i1104.4 = add i64 %_51.i87.i1103.4, 4, !dbg !28609
  %_53.i90.i1106.4 = icmp ult i64 %_50.i88.i1104.4, %_149.1.i96.i1112.pre, !dbg !28610
  br i1 %_53.i90.i1106.4, label %bb21.i92.i1108.4, label %panic1.i91.i1107, !dbg !28610

bb21.i92.i1108.4:                                 ; preds = %bb17.i81.i1097.4
  %499 = getelementptr inbounds nuw float, ptr %_147.0.i93.i1109, i64 %_50.i88.i1104.4, !dbg !28610
  %_49.i94.i1110.4 = load float, ptr %499, align 4, !dbg !28610, !noalias !28594, !noundef !12
  store float %_49.i94.i1110.4, ptr %iter.i49.i948.sroa.0.0.ptr14630.4, align 4, !dbg !28611, !noalias !28594
  %500 = icmp eq i64 %width.i61.i1077, 5, !dbg !28467
  br i1 %500, label %bb16.i95.i1111.loopexit, label %bb39.i75.i1091.5, !dbg !28467

bb39.i75.i1091.5:                                 ; preds = %bb21.i92.i1108.4
  %exitcond17463.5.not = icmp eq i64 %_145.1.i78.i1094, 5, !dbg !28470
  br i1 %exitcond17463.5.not, label %panic.i80.i1096, label %bb17.i81.i1097.5, !dbg !28470

bb17.i81.i1097.5:                                 ; preds = %bb39.i75.i1091.5
  %501 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i1098, i64 68, !dbg !28470
  %_44.i83.i1099.5 = load i32, ptr %501, align 4, !dbg !28470, !noalias !28594, !noundef !12
  %_43.i84.i1100.5 = zext i32 %_44.i83.i1099.5 to i64, !dbg !28470
  %502 = add i64 %ring_cursor.sroa.0.1.i105614644, %_43.i84.i1100.5, !dbg !28607
  %_47.not.i85.i1101.5 = icmp ult i64 %502, %_86.i1076, !dbg !28608
  %503 = select i1 %_47.not.i85.i1101.5, i64 0, i64 %_86.i1076, !dbg !28608
  %spec.select.i86.i1102.5 = sub nuw i64 %502, %503, !dbg !28608
  %_51.i87.i1103.5 = mul i64 %spec.select.i86.i1102.5, %width.i61.i1077, !dbg !28609
  %_50.i88.i1104.5 = add i64 %_51.i87.i1103.5, 5, !dbg !28609
  %_53.i90.i1106.5 = icmp ult i64 %_50.i88.i1104.5, %_149.1.i96.i1112.pre, !dbg !28610
  br i1 %_53.i90.i1106.5, label %bb21.i92.i1108.5, label %panic1.i91.i1107, !dbg !28610

bb21.i92.i1108.5:                                 ; preds = %bb17.i81.i1097.5
  %504 = getelementptr inbounds nuw float, ptr %_147.0.i93.i1109, i64 %_50.i88.i1104.5, !dbg !28610
  %_49.i94.i1110.5 = load float, ptr %504, align 4, !dbg !28610, !noalias !28594, !noundef !12
  store float %_49.i94.i1110.5, ptr %iter.i49.i948.sroa.0.0.ptr14630.5, align 4, !dbg !28611, !noalias !28594
  %505 = icmp eq i64 %width.i61.i1077, 6, !dbg !28467
  br i1 %505, label %bb16.i95.i1111.loopexit, label %bb39.i75.i1091.6, !dbg !28467

bb39.i75.i1091.6:                                 ; preds = %bb21.i92.i1108.5
  %exitcond17463.6.not = icmp eq i64 %_145.1.i78.i1094, 6, !dbg !28470
  br i1 %exitcond17463.6.not, label %panic.i80.i1096, label %bb17.i81.i1097.6, !dbg !28470

bb17.i81.i1097.6:                                 ; preds = %bb39.i75.i1091.6
  %506 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i1098, i64 80, !dbg !28470
  %_44.i83.i1099.6 = load i32, ptr %506, align 4, !dbg !28470, !noalias !28594, !noundef !12
  %_43.i84.i1100.6 = zext i32 %_44.i83.i1099.6 to i64, !dbg !28470
  %507 = add i64 %ring_cursor.sroa.0.1.i105614644, %_43.i84.i1100.6, !dbg !28607
  %_47.not.i85.i1101.6 = icmp ult i64 %507, %_86.i1076, !dbg !28608
  %508 = select i1 %_47.not.i85.i1101.6, i64 0, i64 %_86.i1076, !dbg !28608
  %spec.select.i86.i1102.6 = sub nuw i64 %507, %508, !dbg !28608
  %_51.i87.i1103.6 = mul i64 %spec.select.i86.i1102.6, %width.i61.i1077, !dbg !28609
  %_50.i88.i1104.6 = add i64 %_51.i87.i1103.6, 6, !dbg !28609
  %_53.i90.i1106.6 = icmp ult i64 %_50.i88.i1104.6, %_149.1.i96.i1112.pre, !dbg !28610
  br i1 %_53.i90.i1106.6, label %bb21.i92.i1108.6, label %panic1.i91.i1107, !dbg !28610

bb21.i92.i1108.6:                                 ; preds = %bb17.i81.i1097.6
  %509 = getelementptr inbounds nuw float, ptr %_147.0.i93.i1109, i64 %_50.i88.i1104.6, !dbg !28610
  %_49.i94.i1110.6 = load float, ptr %509, align 4, !dbg !28610, !noalias !28594, !noundef !12
  store float %_49.i94.i1110.6, ptr %iter.i49.i948.sroa.0.0.ptr14630.6, align 4, !dbg !28611, !noalias !28594
  %510 = icmp eq i64 %width.i61.i1077, 7, !dbg !28467
  br i1 %510, label %bb16.i95.i1111.loopexit, label %bb39.i75.i1091.7, !dbg !28467

bb39.i75.i1091.7:                                 ; preds = %bb21.i92.i1108.6
  %exitcond17463.7.not = icmp eq i64 %_145.1.i78.i1094, 7, !dbg !28470
  br i1 %exitcond17463.7.not, label %panic.i80.i1096, label %bb17.i81.i1097.7, !dbg !28470

bb17.i81.i1097.7:                                 ; preds = %bb39.i75.i1091.7
  %511 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i1098, i64 92, !dbg !28470
  %_44.i83.i1099.7 = load i32, ptr %511, align 4, !dbg !28470, !noalias !28594, !noundef !12
  %_43.i84.i1100.7 = zext i32 %_44.i83.i1099.7 to i64, !dbg !28470
  %512 = add i64 %ring_cursor.sroa.0.1.i105614644, %_43.i84.i1100.7, !dbg !28607
  %_47.not.i85.i1101.7 = icmp ult i64 %512, %_86.i1076, !dbg !28608
  %513 = select i1 %_47.not.i85.i1101.7, i64 0, i64 %_86.i1076, !dbg !28608
  %spec.select.i86.i1102.7 = sub nuw i64 %512, %513, !dbg !28608
  %_51.i87.i1103.7 = mul i64 %spec.select.i86.i1102.7, %width.i61.i1077, !dbg !28609
  %_50.i88.i1104.7 = add i64 %_51.i87.i1103.7, 7, !dbg !28609
  %_53.i90.i1106.7 = icmp ult i64 %_50.i88.i1104.7, %_149.1.i96.i1112.pre, !dbg !28610
  br i1 %_53.i90.i1106.7, label %bb21.i92.i1108.7, label %panic1.i91.i1107, !dbg !28610

bb21.i92.i1108.7:                                 ; preds = %bb17.i81.i1097.7
  %514 = getelementptr inbounds nuw float, ptr %_147.0.i93.i1109, i64 %_50.i88.i1104.7, !dbg !28610
  %_49.i94.i1110.7 = load float, ptr %514, align 4, !dbg !28610, !noalias !28594, !noundef !12
  store float %_49.i94.i1110.7, ptr %iter.i49.i948.sroa.0.0.ptr14630.7, align 4, !dbg !28611, !noalias !28594
  br label %bb16.i95.i1111.loopexit, !dbg !28467

panic1.i91.i1107:                                 ; preds = %bb17.i81.i1097.7, %bb17.i81.i1097.6, %bb17.i81.i1097.5, %bb17.i81.i1097.4, %bb17.i81.i1097.3, %bb17.i81.i1097.2, %bb17.i81.i1097.1, %bb17.i81.i1097
  %_50.i88.i1104.lcssa.ph = phi i64 [ %_50.i88.i1104.7, %bb17.i81.i1097.7 ], [ %_50.i88.i1104.6, %bb17.i81.i1097.6 ], [ %_50.i88.i1104.5, %bb17.i81.i1097.5 ], [ %_50.i88.i1104.4, %bb17.i81.i1097.4 ], [ %_50.i88.i1104.3, %bb17.i81.i1097.3 ], [ %_50.i88.i1104.2, %bb17.i81.i1097.2 ], [ %_50.i88.i1104.1, %bb17.i81.i1097.1 ], [ %_51.i87.i1103, %bb17.i81.i1097 ]
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i88.i1104.lcssa.ph, i64 noundef %_149.1.i96.i1112.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7d7f2b4ff3f37cf08b84adcf6762221c) #30, !dbg !28610, !noalias !28594
  unreachable, !dbg !28610

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6109: ; preds = %bb49.i105.i1121
  %_150.0.i106.i1122 = load ptr, ptr %159, align 8, !dbg !28588, !alias.scope !28323, !noalias !28324, !nonnull !12, !noundef !12
  %_127.i108.i1124 = getelementptr inbounds nuw float, ptr %_150.0.i106.i1122, i64 %_76.i103.i1119, !dbg !28612
  %lanes.i5359.sroa.0.0.copyload = load <8 x float>, ptr %_127.i108.i1124, align 4, !dbg !28614, !alias.scope !28618, !noalias !28622
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_127.i108.i1124, ptr noundef nonnull align 4 dereferenceable(32) %_141.i1075, i64 32, i1 false), !dbg !28624
  %515 = fmul <8 x float> %475, %lanes.i5359.sroa.0.0.copyload, !dbg !28629
  %516 = select <8 x i1> %161, <8 x float> %lanes.i5359.sroa.0.0.copyload, <8 x float> %515, !dbg !28634
  store <8 x float> %516, ptr %_141.i1075, align 4, !dbg !28639, !alias.scope !28644, !noalias !28648
  %_142.i1135 = icmp samesign ugt i64 %base.i1061, %right_io.1, !dbg !28652
  br i1 %_142.i1135, label %bb46.i1207, label %bb47.i1136, !dbg !28652, !prof !639

bb44.i1213:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5407
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1061, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_82ebe3a409d1fcceb2641bd874c3a328) #30, !dbg !28656, !noalias !28657
  unreachable, !dbg !28656

bb47.i1136:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6109
  %_145.i1137 = sub nuw nsw i64 %right_io.1, %base.i1061, !dbg !28658
  %_149.i1138 = getelementptr inbounds nuw float, ptr %right_io.0, i64 %base.i1061, !dbg !28659
  %_8.i5353 = icmp samesign ugt i64 %_145.i1137, 7, !dbg !28664
  br i1 %_8.i5353, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5357, label %bb2.i5354, !dbg !28664, !prof !651

bb2.i5354:                                        ; preds = %bb47.i1136
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_145.i1137, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !28669, !noalias !28670
  unreachable, !dbg !28669

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5357: ; preds = %bb47.i1136
  tail call void @llvm.experimental.noalias.scope.decl(metadata !28674), !dbg !28677
  %width.i.i1139 = load i64, ptr %162, align 8, !dbg !28678, !alias.scope !28680, !noalias !28681, !noundef !12
  %517 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %429, <8 x float> %416, i8 30), !dbg !28690
  %518 = fdiv <8 x float> %416, %429, !dbg !28696
  %519 = bitcast <8 x float> %517 to <8 x i32>, !dbg !28701
  %520 = icmp slt <8 x i32> %519, zeroinitializer, !dbg !28705
  %521 = select <8 x i1> %520, <8 x float> %518, <8 x float> splat (float 1.000000e+00), !dbg !28705
  %_144.1.i.i1140 = load i64, ptr %163, align 8, !dbg !28707, !alias.scope !28680, !noalias !28681, !noundef !12
  %_22.i.i1141 = mul i64 %width.i.i1139, %ring_cursor.sroa.0.1.i105614644, !dbg !28708
  %_92.i.i1142 = icmp ugt i64 %_22.i.i1141, %_144.1.i.i1140, !dbg !28709
  br i1 %_92.i.i1142, label %bb37.i.i1206, label %bb38.i.i1143, !dbg !28709, !prof !639

bb38.i.i1143:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5357
  %_95.i.i1145 = sub nuw i64 %_144.1.i.i1140, %_22.i.i1141, !dbg !28712
  %_8.i6101 = icmp samesign ugt i64 %_95.i.i1145, 7, !dbg !28713
  br i1 %_8.i6101, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6104, label %bb2.i6102, !dbg !28713, !prof !651

bb2.i6102:                                        ; preds = %bb38.i.i1143
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_95.i.i1145, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !28718, !noalias !28719
  unreachable, !dbg !28718

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6104: ; preds = %bb38.i.i1143
  %_144.0.i.i1144 = load ptr, ptr %164, align 8, !dbg !28707, !alias.scope !28680, !noalias !28681, !nonnull !12, !noundef !12
  %_99.i.i1146 = getelementptr inbounds nuw float, ptr %_144.0.i.i1144, i64 %_22.i.i1141, !dbg !28723
  store <8 x float> %521, ptr %_99.i.i1146, align 4, !dbg !28725, !alias.scope !28729, !noalias !28733
  tail call void @llvm.experimental.noalias.scope.decl(metadata !28735), !dbg !28738
  %width.i = load i64, ptr %162, align 8, !dbg !28739, !alias.scope !28735, !noalias !28741, !noundef !12
  %522 = icmp eq i64 %width.i, 0, !dbg !28743
  br i1 %522, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit, label %bb32.i1584.lr.ph, !dbg !28743

bb32.i1584.lr.ph:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6104
  %_112.1.i = load i64, ptr %165, align 8, !alias.scope !28735, !noalias !28741, !noundef !12
  %_112.0.i = load ptr, ptr %166, align 8, !nonnull !12
  %523 = add i64 %ring_cursor.sroa.0.1.i105614644, 1
  %_23.not.i = icmp ult i64 %523, %_86.i1076
  %524 = select i1 %_23.not.i, i64 0, i64 %_86.i1076
  %start1.sroa.0.0.i = sub nuw i64 %523, %524
  %_114.1.i = load i64, ptr %163, align 8
  %_114.0.i = load ptr, ptr %164, align 8, !nonnull !12
  %_116.1.i = load i64, ptr %167, align 8
  %_116.0.i = load ptr, ptr %168, align 8, !nonnull !12
  %_118.1.i = load i64, ptr %169, align 8
  %_118.0.i = load ptr, ptr %170, align 8, !nonnull !12
  %_45.i1603 = mul i64 %width.i, %start1.sroa.0.0.i
  br label %bb32.i1584, !dbg !28743

bb32.i1584:                                       ; preds = %bb32.i1584.lr.ph, %bb31.i1604
  %iter.i1582.sroa.10.014636 = phi i64 [ %width.i, %bb32.i1584.lr.ph ], [ %525, %bb31.i1604 ]
  %iter.i1582.sroa.7.014635 = phi i64 [ 0, %bb32.i1584.lr.ph ], [ %_9.0.i6699, %bb31.i1604 ]
  %iter.i1582.sroa.0.0.idx14634 = phi i64 [ 0, %bb32.i1584.lr.ph ], [ %iter.i1582.sroa.0.0.add, %bb31.i1604 ]
  %iter.i1582.sroa.0.0.ptr14637 = getelementptr inbounds nuw i8, ptr %scratch.i1017, i64 %iter.i1582.sroa.0.0.idx14634, !dbg !28745
  %525 = add i64 %iter.i1582.sroa.10.014636, -1, !dbg !28745
  %_7.i.i6695 = icmp eq i64 %iter.i1582.sroa.0.0.idx14634, 32, !dbg !28746
  br i1 %_7.i.i6695, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit, label %bb3.i1586, !dbg !28750

bb3.i1586:                                        ; preds = %bb32.i1584
  %iter.i1582.sroa.0.0.add = add nuw nsw i64 %iter.i1582.sroa.0.0.idx14634, 4, !dbg !28751
  %_9.0.i6699 = add nuw nsw i64 %iter.i1582.sroa.7.014635, 1, !dbg !28753
  %exitcond17469.not = icmp eq i64 %iter.i1582.sroa.7.014635, %_112.1.i, !dbg !28754
  br i1 %exitcond17469.not, label %panic.i, label %bb5.i1588, !dbg !28754

bb5.i1588:                                        ; preds = %bb3.i1586
  %526 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i, i64 %iter.i1582.sroa.7.014635, !dbg !28754
  %shape.i = load i32, ptr %526, align 4, !dbg !28754, !noalias !28755, !noundef !12
  %527 = getelementptr inbounds nuw i8, ptr %526, i64 4, !dbg !28754
  %shape3.i = load i32, ptr %527, align 4, !dbg !28754, !noalias !28755, !noundef !12
  %window.i = zext i32 %shape.i to i64, !dbg !28756
  %_19.i = zext i32 %shape3.i to i64, !dbg !28757
  %528 = add i64 %ring_cursor.sroa.0.1.i105614644, %_19.i, !dbg !28758
  %_20.not.i = icmp ult i64 %528, %_86.i1076, !dbg !28759
  %529 = select i1 %_20.not.i, i64 0, i64 %_86.i1076, !dbg !28759
  %spec.select.i = sub nuw i64 %528, %529, !dbg !28759
  %_27.i1589 = mul i64 %spec.select.i, %width.i, !dbg !28760
  %_26.i = add i64 %_27.i1589, %iter.i1582.sroa.7.014635, !dbg !28760
  %_30.i1590 = icmp ult i64 %_26.i, %_114.1.i, !dbg !28761
  br i1 %_30.i1590, label %bb12.i1591, label %panic5.i, !dbg !28761

panic.i:                                          ; preds = %bb3.i1586
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i, i64 noundef %_112.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8013bf8450ffb218032f1e27d334efa7) #30, !dbg !28754, !noalias !28755
  unreachable, !dbg !28754

bb12.i1591:                                       ; preds = %bb5.i1588
  %530 = getelementptr inbounds nuw float, ptr %_114.0.i, i64 %_26.i, !dbg !28761
  %531 = load float, ptr %530, align 4, !dbg !28761, !noalias !28755, !noundef !12
  %exitcond17470.not = icmp eq i64 %iter.i1582.sroa.7.014635, %_116.1.i, !dbg !28762
  br i1 %exitcond17470.not, label %panic6.i, label %bb13.i1593, !dbg !28762

panic5.i:                                         ; preds = %bb5.i1588
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i, i64 noundef %_114.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d297cbdfce2474defb1d7f11394e8cb6) #30, !dbg !28761, !noalias !28755
  unreachable, !dbg !28761

bb13.i1593:                                       ; preds = %bb12.i1591
  %532 = getelementptr inbounds nuw i32, ptr %_116.0.i, i64 %iter.i1582.sroa.7.014635, !dbg !28762
  %_32.i1594 = load i32, ptr %532, align 4, !dbg !28762, !noalias !28755, !noundef !12
  %position.i1595 = zext i32 %_32.i1594 to i64, !dbg !28762
  %533 = icmp eq i32 %_32.i1594, 0, !dbg !28763
  br i1 %533, label %bb17.i, label %bb15.i1596, !dbg !28763

panic6.i:                                         ; preds = %bb12.i1591
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i, i64 noundef %_116.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ac016620dad255f0d323022a5b7f5883) #30, !dbg !28762, !noalias !28755
  unreachable, !dbg !28762

bb15.i1596:                                       ; preds = %bb13.i1593
  %_37.i = icmp ult i64 %iter.i1582.sroa.7.014635, %_118.1.i, !dbg !28764
  br i1 %_37.i, label %bb16.i1597, label %panic7.i, !dbg !28764

bb17.i:                                           ; preds = %bb35.i, %bb16.i1597, %bb13.i1593
  %newest.sroa.0.0.i = phi float [ %531, %bb13.i1593 ], [ %_35.i1598, %bb35.i ], [ %531, %bb16.i1597 ], !dbg !28765
  %exitcond17471.not = icmp eq i64 %iter.i1582.sroa.7.014635, %_118.1.i, !dbg !28766
  br i1 %exitcond17471.not, label %panic8.i, label %bb18.i, !dbg !28766

bb16.i1597:                                       ; preds = %bb15.i1596
  %534 = getelementptr inbounds nuw float, ptr %_118.0.i, i64 %iter.i1582.sroa.7.014635, !dbg !28764
  %_35.i1598 = load float, ptr %534, align 4, !dbg !28764, !noalias !28755, !noundef !12
  %_102.i1599 = fcmp olt float %_35.i1598, %531, !dbg !28767
  br i1 %_102.i1599, label %bb35.i, label %bb17.i, !dbg !28767

panic7.i:                                         ; preds = %bb15.i1596
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i1582.sroa.7.014635, i64 noundef %_118.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e5355958980a78279123102c3494b10a) #30, !dbg !28764, !noalias !28755
  unreachable, !dbg !28764

bb35.i:                                           ; preds = %bb16.i1597
  br label %bb17.i, !dbg !28769

bb18.i:                                           ; preds = %bb17.i
  %535 = getelementptr inbounds nuw float, ptr %_118.0.i, i64 %iter.i1582.sroa.7.014635, !dbg !28766
  store float %newest.sroa.0.0.i, ptr %535, align 4, !dbg !28766, !noalias !28755
  %_42.i = add nuw nsw i64 %position.i1595, 1, !dbg !28770
  %complete.i1601 = icmp eq i64 %_42.i, %window.i, !dbg !28770
  br i1 %complete.i1601, label %bb22.i, label %bb20.i1602, !dbg !28771

panic8.i:                                         ; preds = %bb17.i
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i, i64 noundef %_118.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2d2a28b8cb03afaaebfea48ffa77c925) #30, !dbg !28766, !noalias !28755
  unreachable, !dbg !28766

bb20.i1602:                                       ; preds = %bb18.i
  %_44.i = add i64 %iter.i1582.sroa.7.014635, %_45.i1603, !dbg !28772
  %_47.i = icmp ult i64 %_44.i, %_114.1.i, !dbg !28773
  br i1 %_47.i, label %bb30.i, label %panic9.i, !dbg !28773

panic9.i:                                         ; preds = %bb20.i1602
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i, i64 noundef %_114.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_53d3a5c3ecf31ea5f14c0a7f8bf40921) #30, !dbg !28773, !noalias !28755
  unreachable, !dbg !28773

bb30.i:                                           ; preds = %bb20.i1602
  %536 = getelementptr inbounds nuw float, ptr %_114.0.i, i64 %_44.i, !dbg !28773
  %_43.i = load float, ptr %536, align 4, !dbg !28773, !noalias !28755, !noundef !12
  %_103.i = fcmp olt float %_43.i, %newest.sroa.0.0.i, !dbg !28774
  %newest.sroa.0.1.i = select i1 %_103.i, float %_43.i, float %newest.sroa.0.0.i, !dbg !28774
  store float %newest.sroa.0.1.i, ptr %iter.i1582.sroa.0.0.ptr14637, align 4, !dbg !28776, !noalias !28755
  %537 = trunc i64 %_42.i to i32, !dbg !28777
  br label %bb31.i1604, !dbg !28778

bb31.i1604:                                       ; preds = %bb25.i, %bb30.i
  %storemerge12435 = phi i32 [ %537, %bb30.i ], [ 0, %bb25.i ], !dbg !28779
  store i32 %storemerge12435, ptr %532, align 4, !dbg !28779, !noalias !28755
  %538 = icmp eq i64 %525, 0, !dbg !28743
  br i1 %538, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit, label %bb32.i1584, !dbg !28743

bb22.i:                                           ; preds = %bb18.i
  store float %newest.sroa.0.0.i, ptr %iter.i1582.sroa.0.0.ptr14637, align 4, !dbg !28776, !noalias !28755
  %539 = load float, ptr %530, align 4, !dbg !28780, !noalias !28755, !noundef !12
  br label %bb41.i, !dbg !28781

bb41.i:                                           ; preds = %bb22.i, %bb25.i
  %iter2.sroa.0.0.i14633 = phi i64 [ 0, %bb22.i ], [ %_105.i1609, %bb25.i ]
  %suffix.sroa.0.0.i14632 = phi float [ %539, %bb22.i ], [ %suffix.sroa.0.1.i, %bb25.i ]
  %end.sroa.0.1.i14631 = phi i64 [ %spec.select.i, %bb22.i ], [ %542, %bb25.i ]
  %_56.i = mul i64 %end.sroa.0.1.i14631, %width.i, !dbg !28784
  %_55.i = add i64 %_56.i, %iter.i1582.sroa.7.014635, !dbg !28784
  %_59.i1610 = icmp ult i64 %_55.i, %_114.1.i, !dbg !28785
  br i1 %_59.i1610, label %bb25.i, label %panic13.i, !dbg !28785

panic13.i:                                        ; preds = %bb41.i
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i, i64 noundef %_114.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b22b5c926aed02a79660ac772e5ad40d) #30, !dbg !28785, !noalias !28755
  unreachable, !dbg !28785

bb25.i:                                           ; preds = %bb41.i
  %_105.i1609 = add nuw nsw i64 %iter2.sroa.0.0.i14633, 1, !dbg !28786
  %540 = getelementptr inbounds nuw float, ptr %_114.0.i, i64 %_55.i, !dbg !28785
  %_54.i = load float, ptr %540, align 4, !dbg !28785, !noalias !28755, !noundef !12
  %_107.i1611 = fcmp olt float %suffix.sroa.0.0.i14632, %_54.i, !dbg !28789
  %suffix.sroa.0.1.i = select i1 %_107.i1611, float %suffix.sroa.0.0.i14632, float %_54.i, !dbg !28789
  store float %suffix.sroa.0.1.i, ptr %540, align 4, !dbg !28791, !noalias !28755
  %541 = icmp eq i64 %end.sroa.0.1.i14631, 0, !dbg !28792
  %spec.store.select.i1613 = select i1 %541, i64 %_86.i1076, i64 %end.sroa.0.1.i14631, !dbg !28792
  %542 = add i64 %spec.store.select.i1613, -1, !dbg !28793
  %exitcond17468.not = icmp eq i64 %_105.i1609, %window.i, !dbg !28794
  br i1 %exitcond17468.not, label %bb31.i1604, label %bb41.i, !dbg !28781

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit: ; preds = %bb32.i1584, %bb31.i1604
  %lanes.i5343.sroa.0.0.copyload.pre = load <8 x float>, ptr %scratch.i1017, align 4, !dbg !28796, !alias.scope !28801, !noalias !28805
  br label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit, !dbg !28809

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit: ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6104
  %lanes.i5343.sroa.0.0.copyload = phi <8 x float> [ %lanes.i5343.sroa.0.0.copyload.pre, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit ], [ %lanes.i5368.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6104 ], !dbg !28796
  %543 = fmul <8 x float> %lanes.i5343.sroa.0.0.copyload, splat (float 1.638400e+04), !dbg !28810
  %544 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %543), !dbg !28815
  %545 = fmul <8 x float> %544, splat (float 0x3F10000000000000), !dbg !28820
  %546 = icmp eq i64 %width.i.i1139, 0, !dbg !28825
  %_149.1.i.i1174.pre = load i64, ptr %171, align 8, !dbg !28827, !alias.scope !28680, !noalias !28681
  br i1 %546, label %bb16.i.i1173, label %bb39.i.i1153.lr.ph, !dbg !28825

bb39.i.i1153.lr.ph:                               ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit
  %_145.1.i.i1156 = load i64, ptr %165, align 8, !alias.scope !28680, !noalias !28681, !noundef !12
  %_145.0.i.i1160 = load ptr, ptr %166, align 8, !nonnull !12
  %_147.0.i.i1171 = load ptr, ptr %172, align 8, !nonnull !12
  %exitcond17474.not = icmp eq i64 %_145.1.i.i1156, 0, !dbg !28828
  br i1 %exitcond17474.not, label %panic.i.i1158, label %bb17.i.i1159, !dbg !28828

bb37.i.i1206:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5357
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i1141, i64 noundef %_144.1.i.i1140, i64 noundef %_144.1.i.i1140, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_913d17a5751fc2956adecdab98dac09f) #30, !dbg !28829, !noalias !28830
  unreachable, !dbg !28829

bb16.i.i1173.loopexit:                            ; preds = %bb21.i.i1170.7, %bb21.i.i1170.6, %bb21.i.i1170.5, %bb21.i.i1170.4, %bb21.i.i1170.3, %bb21.i.i1170.2, %bb21.i.i1170.1, %bb21.i.i1170
  %lanes.i5336.sroa.0.0.copyload.pre = load <8 x float>, ptr %scratch.i1017, align 4, !dbg !28831, !alias.scope !28836, !noalias !28840
  br label %bb16.i.i1173, !dbg !28844

bb16.i.i1173:                                     ; preds = %bb16.i.i1173.loopexit, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit
  %lanes.i5336.sroa.0.0.copyload = phi <8 x float> [ %lanes.i5336.sroa.0.0.copyload.pre, %bb16.i.i1173.loopexit ], [ %lanes.i5343.sroa.0.0.copyload, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit ], !dbg !28831
  %547 = fadd <8 x float> %545, %_58.i.i978.sroa.0.0.copyload20055, !dbg !28845
  %548 = fsub <8 x float> %547, %lanes.i5336.sroa.0.0.copyload, !dbg !28850
  store <8 x float> %548, ptr %173, align 32, !dbg !28855
  %_109.i.i1175 = icmp ugt i64 %_22.i.i1141, %_149.1.i.i1174.pre, !dbg !28856
  br i1 %_109.i.i1175, label %bb42.i.i1205, label %bb43.i.i1176, !dbg !28856, !prof !639

bb43.i.i1176:                                     ; preds = %bb16.i.i1173
  %_112.i.i1178 = sub nuw i64 %_149.1.i.i1174.pre, %_22.i.i1141, !dbg !28859
  %_8.i6096 = icmp samesign ugt i64 %_112.i.i1178, 7, !dbg !28860
  br i1 %_8.i6096, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6099, label %bb2.i6097, !dbg !28860, !prof !651

bb2.i6097:                                        ; preds = %bb43.i.i1176
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_112.i.i1178, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !28865, !noalias !28866
  unreachable, !dbg !28865

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6099: ; preds = %bb43.i.i1176
  %_149.0.i.i1177 = load ptr, ptr %172, align 8, !dbg !28827, !alias.scope !28680, !noalias !28681, !nonnull !12, !noundef !12
  %_116.i.i1179 = getelementptr inbounds nuw float, ptr %_149.0.i.i1177, i64 %_22.i.i1141, !dbg !28870
  store <8 x float> %545, ptr %_116.i.i1179, align 4, !dbg !28872, !alias.scope !28876, !noalias !28880
  %_64.i.i975.sroa.0.0.copyload = load <8 x float>, ptr %174, align 32, !dbg !28882
  %_68.i.i971.sroa.0.0.copyload = load <8 x float>, ptr %175, align 32, !dbg !28883
  %549 = fdiv <8 x float> %548, %_64.i.i975.sroa.0.0.copyload, !dbg !28884
  %550 = fsub <8 x float> splat (float 1.000000e+00), %549, !dbg !28889
  %551 = fsub <8 x float> %550, %_68.i.i971.sroa.0.0.copyload, !dbg !28894
  %552 = fmul <8 x float> %425, %551, !dbg !28899
  %553 = fadd <8 x float> %_68.i.i971.sroa.0.0.copyload, %552, !dbg !28904
  %554 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %550, <8 x float> %553), !dbg !28908
  %555 = bitcast <8 x float> %554 to <8 x i32>, !dbg !28913
  %556 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %554), !dbg !28919
  %557 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %556, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !28921
  %558 = bitcast <8 x float> %557 to <8 x i32>, !dbg !28927
  %559 = xor <8 x i32> %558, splat (i32 -1), !dbg !28933
  %560 = and <8 x i32> %559, %555, !dbg !28935
  %561 = bitcast <8 x i32> %560 to <8 x float>, !dbg !28939
  store <8 x i32> %560, ptr %175, align 32, !dbg !28940
  %562 = fsub <8 x float> splat (float 1.000000e+00), %561, !dbg !28941
  %_150.1.i.i1180 = load i64, ptr %176, align 8, !dbg !28946, !alias.scope !28680, !noalias !28681, !noundef !12
  %_76.i.i1181 = mul i64 %width.i.i1139, %main_cursor.sroa.0.1.i105714645, !dbg !28947
  %_120.i.i1182 = icmp ugt i64 %_76.i.i1181, %_150.1.i.i1180, !dbg !28948
  br i1 %_120.i.i1182, label %bb48.i.i1204, label %bb49.i.i1183, !dbg !28948, !prof !639

bb42.i.i1205:                                     ; preds = %bb16.i.i1173
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i1141, i64 noundef %_149.1.i.i1174.pre, i64 noundef %_149.1.i.i1174.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8a0dcf875eae79f6708bcf79c3cbff55) #30, !dbg !28951, !noalias !28952
  unreachable, !dbg !28951

bb49.i.i1183:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6099
  %_123.i.i1185 = sub nuw i64 %_150.1.i.i1180, %_76.i.i1181, !dbg !28953
  %_8.i5330 = icmp samesign ugt i64 %_123.i.i1185, 7, !dbg !28954
  br i1 %_8.i5330, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6089, label %bb2.i5331, !dbg !28954, !prof !651

bb2.i5331:                                        ; preds = %bb49.i.i1183
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_123.i.i1185, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !28959, !noalias !28960
  unreachable, !dbg !28959

bb48.i.i1204:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6099
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i.i1181, i64 noundef %_150.1.i.i1180, i64 noundef %_150.1.i.i1180, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e5e0b8406fbb9ac3f5f26ac6469c25f7) #30, !dbg !28964, !noalias !28952
  unreachable, !dbg !28964

bb17.i.i1159:                                     ; preds = %bb39.i.i1153.lr.ph
  %563 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1160, i64 8, !dbg !28828
  %_44.i.i1161 = load i32, ptr %563, align 4, !dbg !28828, !noalias !28952, !noundef !12
  %_43.i.i1162 = zext i32 %_44.i.i1161 to i64, !dbg !28828
  %564 = add i64 %ring_cursor.sroa.0.1.i105614644, %_43.i.i1162, !dbg !28965
  %_47.not.i.i1163 = icmp ult i64 %564, %_86.i1076, !dbg !28966
  %565 = select i1 %_47.not.i.i1163, i64 0, i64 %_86.i1076, !dbg !28966
  %spec.select.i.i1164 = sub nuw i64 %564, %565, !dbg !28966
  %_51.i.i1165 = mul i64 %spec.select.i.i1164, %width.i.i1139, !dbg !28967
  %_53.i.i1168 = icmp ult i64 %_51.i.i1165, %_149.1.i.i1174.pre, !dbg !28968
  br i1 %_53.i.i1168, label %bb21.i.i1170, label %panic1.i.i1169, !dbg !28968

panic.i.i1158:                                    ; preds = %bb39.i.i1153.7, %bb39.i.i1153.6, %bb39.i.i1153.5, %bb39.i.i1153.4, %bb39.i.i1153.3, %bb39.i.i1153.2, %bb39.i.i1153.1, %bb39.i.i1153.lr.ph
  %_145.1.i.i1156.lcssa.ph = phi i64 [ 7, %bb39.i.i1153.7 ], [ 6, %bb39.i.i1153.6 ], [ 5, %bb39.i.i1153.5 ], [ 4, %bb39.i.i1153.4 ], [ 3, %bb39.i.i1153.3 ], [ 2, %bb39.i.i1153.2 ], [ 1, %bb39.i.i1153.1 ], [ 0, %bb39.i.i1153.lr.ph ]
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i.i1156.lcssa.ph, i64 noundef %_145.1.i.i1156.lcssa.ph, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7ff2ed40a8d2df224a47a35441e5e2fe) #30, !dbg !28828, !noalias !28952
  unreachable, !dbg !28828

bb21.i.i1170:                                     ; preds = %bb17.i.i1159
  %566 = getelementptr inbounds nuw float, ptr %_147.0.i.i1171, i64 %_51.i.i1165, !dbg !28968
  %_49.i.i1172 = load float, ptr %566, align 4, !dbg !28968, !noalias !28952, !noundef !12
  store float %_49.i.i1172, ptr %scratch.i1017, align 4, !dbg !28969, !noalias !28952
  %567 = icmp eq i64 %width.i.i1139, 1, !dbg !28825
  br i1 %567, label %bb16.i.i1173.loopexit, label %bb39.i.i1153.1, !dbg !28825

bb39.i.i1153.1:                                   ; preds = %bb21.i.i1170
  %exitcond17474.1.not = icmp eq i64 %_145.1.i.i1156, 1, !dbg !28828
  br i1 %exitcond17474.1.not, label %panic.i.i1158, label %bb17.i.i1159.1, !dbg !28828

bb17.i.i1159.1:                                   ; preds = %bb39.i.i1153.1
  %568 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1160, i64 20, !dbg !28828
  %_44.i.i1161.1 = load i32, ptr %568, align 4, !dbg !28828, !noalias !28952, !noundef !12
  %_43.i.i1162.1 = zext i32 %_44.i.i1161.1 to i64, !dbg !28828
  %569 = add i64 %ring_cursor.sroa.0.1.i105614644, %_43.i.i1162.1, !dbg !28965
  %_47.not.i.i1163.1 = icmp ult i64 %569, %_86.i1076, !dbg !28966
  %570 = select i1 %_47.not.i.i1163.1, i64 0, i64 %_86.i1076, !dbg !28966
  %spec.select.i.i1164.1 = sub nuw i64 %569, %570, !dbg !28966
  %_51.i.i1165.1 = mul i64 %spec.select.i.i1164.1, %width.i.i1139, !dbg !28967
  %_50.i.i1166.1 = add i64 %_51.i.i1165.1, 1, !dbg !28967
  %_53.i.i1168.1 = icmp ult i64 %_50.i.i1166.1, %_149.1.i.i1174.pre, !dbg !28968
  br i1 %_53.i.i1168.1, label %bb21.i.i1170.1, label %panic1.i.i1169, !dbg !28968

bb21.i.i1170.1:                                   ; preds = %bb17.i.i1159.1
  %571 = getelementptr inbounds nuw float, ptr %_147.0.i.i1171, i64 %_50.i.i1166.1, !dbg !28968
  %_49.i.i1172.1 = load float, ptr %571, align 4, !dbg !28968, !noalias !28952, !noundef !12
  store float %_49.i.i1172.1, ptr %iter.i.i981.sroa.0.0.ptr14641.1, align 4, !dbg !28969, !noalias !28952
  %572 = icmp eq i64 %width.i.i1139, 2, !dbg !28825
  br i1 %572, label %bb16.i.i1173.loopexit, label %bb39.i.i1153.2, !dbg !28825

bb39.i.i1153.2:                                   ; preds = %bb21.i.i1170.1
  %exitcond17474.2.not = icmp eq i64 %_145.1.i.i1156, 2, !dbg !28828
  br i1 %exitcond17474.2.not, label %panic.i.i1158, label %bb17.i.i1159.2, !dbg !28828

bb17.i.i1159.2:                                   ; preds = %bb39.i.i1153.2
  %573 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1160, i64 32, !dbg !28828
  %_44.i.i1161.2 = load i32, ptr %573, align 4, !dbg !28828, !noalias !28952, !noundef !12
  %_43.i.i1162.2 = zext i32 %_44.i.i1161.2 to i64, !dbg !28828
  %574 = add i64 %ring_cursor.sroa.0.1.i105614644, %_43.i.i1162.2, !dbg !28965
  %_47.not.i.i1163.2 = icmp ult i64 %574, %_86.i1076, !dbg !28966
  %575 = select i1 %_47.not.i.i1163.2, i64 0, i64 %_86.i1076, !dbg !28966
  %spec.select.i.i1164.2 = sub nuw i64 %574, %575, !dbg !28966
  %_51.i.i1165.2 = mul i64 %spec.select.i.i1164.2, %width.i.i1139, !dbg !28967
  %_50.i.i1166.2 = add i64 %_51.i.i1165.2, 2, !dbg !28967
  %_53.i.i1168.2 = icmp ult i64 %_50.i.i1166.2, %_149.1.i.i1174.pre, !dbg !28968
  br i1 %_53.i.i1168.2, label %bb21.i.i1170.2, label %panic1.i.i1169, !dbg !28968

bb21.i.i1170.2:                                   ; preds = %bb17.i.i1159.2
  %576 = getelementptr inbounds nuw float, ptr %_147.0.i.i1171, i64 %_50.i.i1166.2, !dbg !28968
  %_49.i.i1172.2 = load float, ptr %576, align 4, !dbg !28968, !noalias !28952, !noundef !12
  store float %_49.i.i1172.2, ptr %iter.i.i981.sroa.0.0.ptr14641.2, align 4, !dbg !28969, !noalias !28952
  %577 = icmp eq i64 %width.i.i1139, 3, !dbg !28825
  br i1 %577, label %bb16.i.i1173.loopexit, label %bb39.i.i1153.3, !dbg !28825

bb39.i.i1153.3:                                   ; preds = %bb21.i.i1170.2
  %exitcond17474.3.not = icmp eq i64 %_145.1.i.i1156, 3, !dbg !28828
  br i1 %exitcond17474.3.not, label %panic.i.i1158, label %bb17.i.i1159.3, !dbg !28828

bb17.i.i1159.3:                                   ; preds = %bb39.i.i1153.3
  %578 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1160, i64 44, !dbg !28828
  %_44.i.i1161.3 = load i32, ptr %578, align 4, !dbg !28828, !noalias !28952, !noundef !12
  %_43.i.i1162.3 = zext i32 %_44.i.i1161.3 to i64, !dbg !28828
  %579 = add i64 %ring_cursor.sroa.0.1.i105614644, %_43.i.i1162.3, !dbg !28965
  %_47.not.i.i1163.3 = icmp ult i64 %579, %_86.i1076, !dbg !28966
  %580 = select i1 %_47.not.i.i1163.3, i64 0, i64 %_86.i1076, !dbg !28966
  %spec.select.i.i1164.3 = sub nuw i64 %579, %580, !dbg !28966
  %_51.i.i1165.3 = mul i64 %spec.select.i.i1164.3, %width.i.i1139, !dbg !28967
  %_50.i.i1166.3 = add i64 %_51.i.i1165.3, 3, !dbg !28967
  %_53.i.i1168.3 = icmp ult i64 %_50.i.i1166.3, %_149.1.i.i1174.pre, !dbg !28968
  br i1 %_53.i.i1168.3, label %bb21.i.i1170.3, label %panic1.i.i1169, !dbg !28968

bb21.i.i1170.3:                                   ; preds = %bb17.i.i1159.3
  %581 = getelementptr inbounds nuw float, ptr %_147.0.i.i1171, i64 %_50.i.i1166.3, !dbg !28968
  %_49.i.i1172.3 = load float, ptr %581, align 4, !dbg !28968, !noalias !28952, !noundef !12
  store float %_49.i.i1172.3, ptr %iter.i.i981.sroa.0.0.ptr14641.3, align 4, !dbg !28969, !noalias !28952
  %582 = icmp eq i64 %width.i.i1139, 4, !dbg !28825
  br i1 %582, label %bb16.i.i1173.loopexit, label %bb39.i.i1153.4, !dbg !28825

bb39.i.i1153.4:                                   ; preds = %bb21.i.i1170.3
  %exitcond17474.4.not = icmp eq i64 %_145.1.i.i1156, 4, !dbg !28828
  br i1 %exitcond17474.4.not, label %panic.i.i1158, label %bb17.i.i1159.4, !dbg !28828

bb17.i.i1159.4:                                   ; preds = %bb39.i.i1153.4
  %583 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1160, i64 56, !dbg !28828
  %_44.i.i1161.4 = load i32, ptr %583, align 4, !dbg !28828, !noalias !28952, !noundef !12
  %_43.i.i1162.4 = zext i32 %_44.i.i1161.4 to i64, !dbg !28828
  %584 = add i64 %ring_cursor.sroa.0.1.i105614644, %_43.i.i1162.4, !dbg !28965
  %_47.not.i.i1163.4 = icmp ult i64 %584, %_86.i1076, !dbg !28966
  %585 = select i1 %_47.not.i.i1163.4, i64 0, i64 %_86.i1076, !dbg !28966
  %spec.select.i.i1164.4 = sub nuw i64 %584, %585, !dbg !28966
  %_51.i.i1165.4 = mul i64 %spec.select.i.i1164.4, %width.i.i1139, !dbg !28967
  %_50.i.i1166.4 = add i64 %_51.i.i1165.4, 4, !dbg !28967
  %_53.i.i1168.4 = icmp ult i64 %_50.i.i1166.4, %_149.1.i.i1174.pre, !dbg !28968
  br i1 %_53.i.i1168.4, label %bb21.i.i1170.4, label %panic1.i.i1169, !dbg !28968

bb21.i.i1170.4:                                   ; preds = %bb17.i.i1159.4
  %586 = getelementptr inbounds nuw float, ptr %_147.0.i.i1171, i64 %_50.i.i1166.4, !dbg !28968
  %_49.i.i1172.4 = load float, ptr %586, align 4, !dbg !28968, !noalias !28952, !noundef !12
  store float %_49.i.i1172.4, ptr %iter.i.i981.sroa.0.0.ptr14641.4, align 4, !dbg !28969, !noalias !28952
  %587 = icmp eq i64 %width.i.i1139, 5, !dbg !28825
  br i1 %587, label %bb16.i.i1173.loopexit, label %bb39.i.i1153.5, !dbg !28825

bb39.i.i1153.5:                                   ; preds = %bb21.i.i1170.4
  %exitcond17474.5.not = icmp eq i64 %_145.1.i.i1156, 5, !dbg !28828
  br i1 %exitcond17474.5.not, label %panic.i.i1158, label %bb17.i.i1159.5, !dbg !28828

bb17.i.i1159.5:                                   ; preds = %bb39.i.i1153.5
  %588 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1160, i64 68, !dbg !28828
  %_44.i.i1161.5 = load i32, ptr %588, align 4, !dbg !28828, !noalias !28952, !noundef !12
  %_43.i.i1162.5 = zext i32 %_44.i.i1161.5 to i64, !dbg !28828
  %589 = add i64 %ring_cursor.sroa.0.1.i105614644, %_43.i.i1162.5, !dbg !28965
  %_47.not.i.i1163.5 = icmp ult i64 %589, %_86.i1076, !dbg !28966
  %590 = select i1 %_47.not.i.i1163.5, i64 0, i64 %_86.i1076, !dbg !28966
  %spec.select.i.i1164.5 = sub nuw i64 %589, %590, !dbg !28966
  %_51.i.i1165.5 = mul i64 %spec.select.i.i1164.5, %width.i.i1139, !dbg !28967
  %_50.i.i1166.5 = add i64 %_51.i.i1165.5, 5, !dbg !28967
  %_53.i.i1168.5 = icmp ult i64 %_50.i.i1166.5, %_149.1.i.i1174.pre, !dbg !28968
  br i1 %_53.i.i1168.5, label %bb21.i.i1170.5, label %panic1.i.i1169, !dbg !28968

bb21.i.i1170.5:                                   ; preds = %bb17.i.i1159.5
  %591 = getelementptr inbounds nuw float, ptr %_147.0.i.i1171, i64 %_50.i.i1166.5, !dbg !28968
  %_49.i.i1172.5 = load float, ptr %591, align 4, !dbg !28968, !noalias !28952, !noundef !12
  store float %_49.i.i1172.5, ptr %iter.i.i981.sroa.0.0.ptr14641.5, align 4, !dbg !28969, !noalias !28952
  %592 = icmp eq i64 %width.i.i1139, 6, !dbg !28825
  br i1 %592, label %bb16.i.i1173.loopexit, label %bb39.i.i1153.6, !dbg !28825

bb39.i.i1153.6:                                   ; preds = %bb21.i.i1170.5
  %exitcond17474.6.not = icmp eq i64 %_145.1.i.i1156, 6, !dbg !28828
  br i1 %exitcond17474.6.not, label %panic.i.i1158, label %bb17.i.i1159.6, !dbg !28828

bb17.i.i1159.6:                                   ; preds = %bb39.i.i1153.6
  %593 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1160, i64 80, !dbg !28828
  %_44.i.i1161.6 = load i32, ptr %593, align 4, !dbg !28828, !noalias !28952, !noundef !12
  %_43.i.i1162.6 = zext i32 %_44.i.i1161.6 to i64, !dbg !28828
  %594 = add i64 %ring_cursor.sroa.0.1.i105614644, %_43.i.i1162.6, !dbg !28965
  %_47.not.i.i1163.6 = icmp ult i64 %594, %_86.i1076, !dbg !28966
  %595 = select i1 %_47.not.i.i1163.6, i64 0, i64 %_86.i1076, !dbg !28966
  %spec.select.i.i1164.6 = sub nuw i64 %594, %595, !dbg !28966
  %_51.i.i1165.6 = mul i64 %spec.select.i.i1164.6, %width.i.i1139, !dbg !28967
  %_50.i.i1166.6 = add i64 %_51.i.i1165.6, 6, !dbg !28967
  %_53.i.i1168.6 = icmp ult i64 %_50.i.i1166.6, %_149.1.i.i1174.pre, !dbg !28968
  br i1 %_53.i.i1168.6, label %bb21.i.i1170.6, label %panic1.i.i1169, !dbg !28968

bb21.i.i1170.6:                                   ; preds = %bb17.i.i1159.6
  %596 = getelementptr inbounds nuw float, ptr %_147.0.i.i1171, i64 %_50.i.i1166.6, !dbg !28968
  %_49.i.i1172.6 = load float, ptr %596, align 4, !dbg !28968, !noalias !28952, !noundef !12
  store float %_49.i.i1172.6, ptr %iter.i.i981.sroa.0.0.ptr14641.6, align 4, !dbg !28969, !noalias !28952
  %597 = icmp eq i64 %width.i.i1139, 7, !dbg !28825
  br i1 %597, label %bb16.i.i1173.loopexit, label %bb39.i.i1153.7, !dbg !28825

bb39.i.i1153.7:                                   ; preds = %bb21.i.i1170.6
  %exitcond17474.7.not = icmp eq i64 %_145.1.i.i1156, 7, !dbg !28828
  br i1 %exitcond17474.7.not, label %panic.i.i1158, label %bb17.i.i1159.7, !dbg !28828

bb17.i.i1159.7:                                   ; preds = %bb39.i.i1153.7
  %598 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1160, i64 92, !dbg !28828
  %_44.i.i1161.7 = load i32, ptr %598, align 4, !dbg !28828, !noalias !28952, !noundef !12
  %_43.i.i1162.7 = zext i32 %_44.i.i1161.7 to i64, !dbg !28828
  %599 = add i64 %ring_cursor.sroa.0.1.i105614644, %_43.i.i1162.7, !dbg !28965
  %_47.not.i.i1163.7 = icmp ult i64 %599, %_86.i1076, !dbg !28966
  %600 = select i1 %_47.not.i.i1163.7, i64 0, i64 %_86.i1076, !dbg !28966
  %spec.select.i.i1164.7 = sub nuw i64 %599, %600, !dbg !28966
  %_51.i.i1165.7 = mul i64 %spec.select.i.i1164.7, %width.i.i1139, !dbg !28967
  %_50.i.i1166.7 = add i64 %_51.i.i1165.7, 7, !dbg !28967
  %_53.i.i1168.7 = icmp ult i64 %_50.i.i1166.7, %_149.1.i.i1174.pre, !dbg !28968
  br i1 %_53.i.i1168.7, label %bb21.i.i1170.7, label %panic1.i.i1169, !dbg !28968

bb21.i.i1170.7:                                   ; preds = %bb17.i.i1159.7
  %601 = getelementptr inbounds nuw float, ptr %_147.0.i.i1171, i64 %_50.i.i1166.7, !dbg !28968
  %_49.i.i1172.7 = load float, ptr %601, align 4, !dbg !28968, !noalias !28952, !noundef !12
  store float %_49.i.i1172.7, ptr %iter.i.i981.sroa.0.0.ptr14641.7, align 4, !dbg !28969, !noalias !28952
  br label %bb16.i.i1173.loopexit, !dbg !28825

panic1.i.i1169:                                   ; preds = %bb17.i.i1159.7, %bb17.i.i1159.6, %bb17.i.i1159.5, %bb17.i.i1159.4, %bb17.i.i1159.3, %bb17.i.i1159.2, %bb17.i.i1159.1, %bb17.i.i1159
  %_50.i.i1166.lcssa.ph = phi i64 [ %_50.i.i1166.7, %bb17.i.i1159.7 ], [ %_50.i.i1166.6, %bb17.i.i1159.6 ], [ %_50.i.i1166.5, %bb17.i.i1159.5 ], [ %_50.i.i1166.4, %bb17.i.i1159.4 ], [ %_50.i.i1166.3, %bb17.i.i1159.3 ], [ %_50.i.i1166.2, %bb17.i.i1159.2 ], [ %_50.i.i1166.1, %bb17.i.i1159.1 ], [ %_51.i.i1165, %bb17.i.i1159 ]
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i.i1166.lcssa.ph, i64 noundef %_149.1.i.i1174.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7d7f2b4ff3f37cf08b84adcf6762221c) #30, !dbg !28968, !noalias !28952
  unreachable, !dbg !28968

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6089: ; preds = %bb49.i.i1183
  %_150.0.i.i1184 = load ptr, ptr %177, align 8, !dbg !28946, !alias.scope !28680, !noalias !28681, !nonnull !12, !noundef !12
  %_127.i.i1186 = getelementptr inbounds nuw float, ptr %_150.0.i.i1184, i64 %_76.i.i1181, !dbg !28970
  %lanes.i5327.sroa.0.0.copyload = load <8 x float>, ptr %_127.i.i1186, align 4, !dbg !28972, !alias.scope !28976, !noalias !28980
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_127.i.i1186, ptr noundef nonnull align 4 dereferenceable(32) %_149.i1138, i64 32, i1 false), !dbg !28982
  %602 = fmul <8 x float> %562, %lanes.i5327.sroa.0.0.copyload, !dbg !28987
  %603 = select <8 x i1> %161, <8 x float> %lanes.i5327.sroa.0.0.copyload, <8 x float> %602, !dbg !28992
  store <8 x float> %603, ptr %_149.i1138, align 4, !dbg !28997, !alias.scope !29002, !noalias !29006
  %604 = add i64 %main_cursor.sroa.0.1.i105714645, 1, !dbg !29010
  %_101.i1197 = load i64, ptr %178, align 8, !dbg !29011, !alias.scope !26859, !noalias !28316, !noundef !12
  %_99.i1198 = icmp eq i64 %604, %_101.i1197, !dbg !29012
  %spec.store.select.i1199 = select i1 %_99.i1198, i64 0, i64 %604, !dbg !29012
  %605 = add i64 %ring_cursor.sroa.0.1.i105614644, 1, !dbg !29013
  %_102.i1200 = icmp eq i64 %605, %_86.i1076, !dbg !29014
  %spec.store.select13.i1201 = select i1 %_102.i1200, i64 0, i64 %605, !dbg !29014
  %exitcond17479.not = icmp eq i64 %418, %umax17478, !dbg !29015
  br i1 %exitcond17479.not, label %bb13.i1040.loopexit.loopexit, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5407, !dbg !28071

bb46.i1207:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6109
  store <8 x float> %395, ptr %131, align 32, !dbg !26918
  store <8 x float> %400, ptr %_63.i1062, align 32, !dbg !26926
  store <8 x float> %401, ptr %132, align 32, !dbg !26927
  store <8 x float> %403, ptr %134, align 32, !dbg !26928
  store <8 x float> %408, ptr %_64.i1063, align 32, !dbg !26930
  store <8 x float> %409, ptr %135, align 32, !dbg !26931
  store <8 x float> %411, ptr %137, align 32, !dbg !26932
  store <8 x float> %416, ptr %_68.i1064, align 32, !dbg !26936
  store <8 x float> %417, ptr %138, align 32, !dbg !26937
  store <8 x float> %420, ptr %140, align 32, !dbg !26938
  store <8 x float> %425, ptr %_69.i1065, align 32, !dbg !26940
  store <8 x float> %426, ptr %141, align 32, !dbg !26941
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1061, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_77ae5a1ade955db4654bdc779dedbc90) #30, !dbg !29018, !noalias !28657
  unreachable, !dbg !29018

_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit: ; preds = %bb13.i1040.loopexit
  %606 = trunc i64 %main_cursor.sroa.0.1.i1057.lcssa to i32, !dbg !29019
  %607 = trunc i64 %ring_cursor.sroa.0.1.i1056.lcssa to i32, !dbg !29020
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, !dbg !29021

_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit, %bb12.i
  %ring_cursor.sroa.0.0.i1043.lcssa = phi i32 [ %_36.i1032, %bb12.i ], [ %607, %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit ], !dbg !26885
  %main_cursor.sroa.0.0.i1044.lcssa = phi i32 [ %_34.i1031, %bb12.i ], [ %606, %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit ], !dbg !26882
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_left.i1024, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33) #31, !dbg !29022
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_right.i1023, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34) #31, !dbg !29023
  store i32 %main_cursor.sroa.0.0.i1044.lcssa, ptr %_35, align 4, !dbg !29019, !alias.scope !26865, !noalias !26884
  store i32 %ring_cursor.sroa.0.0.i1043.lcssa, ptr %81, align 4, !dbg !29020, !alias.scope !26865, !noalias !26884
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i1015), !dbg !29024, !noalias !26889
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i1016), !dbg !29025, !noalias !26889
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i1017), !dbg !29026, !noalias !26889
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, !dbg !26858

bb11.i:                                           ; preds = %bb10.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !29027), !dbg !29030
  tail call void @llvm.experimental.noalias.scope.decl(metadata !29031), !dbg !29030
  tail call void @llvm.experimental.noalias.scope.decl(metadata !29033), !dbg !29030
  tail call void @llvm.experimental.noalias.scope.decl(metadata !29035), !dbg !29030
  tail call void @llvm.experimental.noalias.scope.decl(metadata !29037), !dbg !29030
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_left.i675, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #31, !dbg !29039
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_right.i674, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #31, !dbg !29043
  %608 = load i8, ptr %79, align 32, !dbg !29045, !range !17, !alias.scope !29027, !noalias !29049, !noundef !12
  %609 = load i8, ptr %80, align 1, !dbg !29052, !range !17, !alias.scope !29027, !noalias !29049, !noundef !12
  %_34.i = load i32, ptr %_35, align 4, !dbg !29054, !alias.scope !29037, !noalias !29056, !noundef !12
  %_36.i682 = load i32, ptr %81, align 4, !dbg !29057, !alias.scope !29037, !noalias !29056, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i), !dbg !29059, !noalias !29061
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i, i8 0, i64 32, i1 false), !noalias !29061
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i668), !dbg !29062, !noalias !29061
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i668, i8 0, i64 1024, i1 false), !noalias !29061
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i667), !dbg !29064, !noalias !29061
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i667, i8 0, i64 1024, i1 false), !noalias !29061
  br i1 %_111.not.i14884, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, label %bb37.i.lr.ph, !dbg !29066

bb37.i.lr.ph:                                     ; preds = %bb11.i
  %610 = zext i32 %_36.i682 to i64, !dbg !29057
  %611 = zext i32 %_34.i to i64, !dbg !29054
  %_32.i679 = trunc nuw i8 %609 to i1, !dbg !29052
  %_31.i676 = trunc nuw i8 %608 to i1, !dbg !29045
  %612 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !29076
  %613 = bitcast <8 x float> %612 to <8 x i32>, !dbg !29082
  %614 = xor <8 x i32> %613, splat (i32 -1), !dbg !29088
  %history.i310.i.sroa.10.0.hot_left.i675.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i675, i64 32
  %history.i310.i.sroa.13.0.hot_left.i675.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i675, i64 64
  %history.i310.i.sroa.16.0.hot_left.i675.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i675, i64 96
  %history.i310.i.sroa.19.0.hot_left.i675.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i675, i64 128
  %history.i310.i.sroa.22.0.hot_left.i675.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i675, i64 160
  %history.i310.i.sroa.25.0.hot_left.i675.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i675, i64 192
  %history.i310.i.sroa.29.0.hot_left.i675.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i675, i64 224
  %history.i310.i.sroa.32.0.hot_left.i675.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i675, i64 256
  %history.i310.i.sroa.35.0.hot_left.i675.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i675, i64 288
  %history.i310.i.sroa.38.0.hot_left.i675.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i675, i64 320
  %history.i310.i.sroa.41.0.hot_left.i675.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i675, i64 352
  %615 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %616 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %617 = getelementptr inbounds nuw i8, ptr %self, i64 96
  %row12.i.i.i321.i = getelementptr inbounds nuw i8, ptr %self, i64 128
  %618 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %619 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %620 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %row13.i.i.i322.i = getelementptr inbounds nuw i8, ptr %self, i64 256
  %621 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %622 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %623 = getelementptr inbounds nuw i8, ptr %self, i64 352
  %row14.i.i.i323.i = getelementptr inbounds nuw i8, ptr %self, i64 384
  %624 = getelementptr inbounds nuw i8, ptr %self, i64 416
  %625 = getelementptr inbounds nuw i8, ptr %self, i64 448
  %626 = getelementptr inbounds nuw i8, ptr %self, i64 480
  %row15.i.i.i324.i = getelementptr inbounds nuw i8, ptr %self, i64 512
  %627 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %628 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %629 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %row16.i.i.i325.i = getelementptr inbounds nuw i8, ptr %self, i64 640
  %630 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %631 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %632 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %row17.i.i.i326.i = getelementptr inbounds nuw i8, ptr %self, i64 768
  %633 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %634 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %635 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %row18.i.i.i327.i = getelementptr inbounds nuw i8, ptr %self, i64 896
  %636 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %637 = getelementptr inbounds nuw i8, ptr %self, i64 960
  %638 = getelementptr inbounds nuw i8, ptr %self, i64 992
  %row19.i.i.i328.i = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %639 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %640 = getelementptr inbounds nuw i8, ptr %self, i64 1088
  %641 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %row20.i.i.i329.i = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %642 = getelementptr inbounds nuw i8, ptr %self, i64 1184
  %643 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %644 = getelementptr inbounds nuw i8, ptr %self, i64 1248
  %row21.i.i.i330.i = getelementptr inbounds nuw i8, ptr %self, i64 1280
  %645 = getelementptr inbounds nuw i8, ptr %self, i64 1312
  %646 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %647 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %row22.i.i.i331.i = getelementptr inbounds nuw i8, ptr %self, i64 1408
  %648 = getelementptr inbounds nuw i8, ptr %self, i64 1440
  %649 = getelementptr inbounds nuw i8, ptr %self, i64 1472
  %650 = getelementptr inbounds nuw i8, ptr %self, i64 1504
  %history.i.i640.sroa.10.0.hot_right.i674.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i674, i64 32
  %history.i.i640.sroa.13.0.hot_right.i674.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i674, i64 64
  %history.i.i640.sroa.16.0.hot_right.i674.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i674, i64 96
  %history.i.i640.sroa.19.0.hot_right.i674.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i674, i64 128
  %history.i.i640.sroa.22.0.hot_right.i674.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i674, i64 160
  %history.i.i640.sroa.25.0.hot_right.i674.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i674, i64 192
  %history.i.i640.sroa.29.0.hot_right.i674.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i674, i64 224
  %history.i.i640.sroa.32.0.hot_right.i674.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i674, i64 256
  %history.i.i640.sroa.35.0.hot_right.i674.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i674, i64 288
  %history.i.i640.sroa.38.0.hot_right.i674.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i674, i64 320
  %history.i.i640.sroa.41.0.hot_right.i674.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i674, i64 352
  %_63.i700 = getelementptr inbounds nuw i8, ptr %hot_left.i675, i64 384
  %_64.i = getelementptr inbounds nuw i8, ptr %hot_left.i675, i64 512
  %_68.i = getelementptr inbounds nuw i8, ptr %hot_right.i674, i64 384
  %_69.i701 = getelementptr inbounds nuw i8, ptr %hot_right.i674, i64 512
  %651 = select i1 %_31.i676, <8 x i32> %613, <8 x i32> %614
  %652 = icmp slt <8 x i32> %651, zeroinitializer
  %653 = getelementptr inbounds nuw i8, ptr %self, i64 1624
  %654 = getelementptr inbounds nuw i8, ptr %self, i64 1840
  %655 = getelementptr inbounds nuw i8, ptr %self, i64 1688
  %656 = getelementptr inbounds nuw i8, ptr %self, i64 1680
  %657 = getelementptr inbounds nuw i8, ptr %self, i64 1768
  %658 = getelementptr inbounds nuw i8, ptr %self, i64 1760
  %659 = getelementptr inbounds nuw i8, ptr %self, i64 1736
  %660 = getelementptr inbounds nuw i8, ptr %self, i64 1728
  %661 = getelementptr inbounds nuw i8, ptr %self, i64 1704
  %662 = getelementptr inbounds nuw i8, ptr %self, i64 1696
  %663 = getelementptr inbounds nuw i8, ptr %hot_left.i675, i64 672
  %664 = getelementptr inbounds nuw i8, ptr %hot_left.i675, i64 704
  %665 = getelementptr inbounds nuw i8, ptr %hot_left.i675, i64 640
  %666 = getelementptr inbounds nuw i8, ptr %self, i64 1672
  %667 = getelementptr inbounds nuw i8, ptr %self, i64 1664
  %668 = select i1 %_32.i679, <8 x i32> %613, <8 x i32> %614
  %669 = icmp slt <8 x i32> %668, zeroinitializer
  %670 = getelementptr inbounds nuw i8, ptr %self, i64 2040
  %671 = getelementptr inbounds nuw i8, ptr %self, i64 1888
  %672 = getelementptr inbounds nuw i8, ptr %self, i64 1880
  %673 = getelementptr inbounds nuw i8, ptr %self, i64 2032
  %674 = getelementptr inbounds nuw i8, ptr %self, i64 2024
  %675 = getelementptr inbounds nuw i8, ptr %self, i64 1968
  %676 = getelementptr inbounds nuw i8, ptr %self, i64 1960
  %677 = getelementptr inbounds nuw i8, ptr %self, i64 1936
  %678 = getelementptr inbounds nuw i8, ptr %self, i64 1928
  %679 = getelementptr inbounds nuw i8, ptr %self, i64 1904
  %680 = getelementptr inbounds nuw i8, ptr %self, i64 1896
  %681 = getelementptr inbounds nuw i8, ptr %hot_right.i674, i64 672
  %682 = getelementptr inbounds nuw i8, ptr %hot_right.i674, i64 704
  %683 = getelementptr inbounds nuw i8, ptr %hot_right.i674, i64 640
  %684 = getelementptr inbounds nuw i8, ptr %self, i64 1872
  %685 = getelementptr inbounds nuw i8, ptr %self, i64 1864
  %686 = getelementptr inbounds nuw i8, ptr %self, i64 1632
  %iter.i49.i.sroa.0.0.ptr14719.1 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 4
  %iter.i49.i.sroa.0.0.ptr14719.2 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 8
  %iter.i49.i.sroa.0.0.ptr14719.3 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 12
  %iter.i49.i.sroa.0.0.ptr14719.4 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 16
  %iter.i49.i.sroa.0.0.ptr14719.5 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 20
  %iter.i49.i.sroa.0.0.ptr14719.6 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 24
  %iter.i49.i.sroa.0.0.ptr14719.7 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 28
  %iter.i.i.sroa.0.0.ptr14730.1 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 4
  %iter.i.i.sroa.0.0.ptr14730.2 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 8
  %iter.i.i.sroa.0.0.ptr14730.3 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 12
  %iter.i.i.sroa.0.0.ptr14730.4 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 16
  %iter.i.i.sroa.0.0.ptr14730.5 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 20
  %iter.i.i.sroa.0.0.ptr14730.6 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 24
  %iter.i.i.sroa.0.0.ptr14730.7 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 28
  br label %bb37.i, !dbg !29066

bb16.i.bb13.i.loopexit_crit_edge:                 ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6139
  store <8 x float> %937, ptr %663, align 32, !dbg !29090
  store <8 x float> %1024, ptr %681, align 32, !dbg !29104
  br label %bb13.i.loopexit, !dbg !29106

bb13.i.loopexit:                                  ; preds = %bb16.i.bb13.i.loopexit_crit_edge, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i695
  %ring_cursor.sroa.0.1.i697.lcssa = phi i64 [ %spec.store.select13.i, %bb16.i.bb13.i.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i68814887, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i695 ], !dbg !29112
  %main_cursor.sroa.0.1.i698.lcssa = phi i64 [ %spec.store.select.i, %bb16.i.bb13.i.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i68914888, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i695 ], !dbg !29113
  %_111.not.i = icmp eq i64 %689, 0, !dbg !29066
  %indvars.iv.next17483 = add nsw i64 %indvars.iv17482, -32, !dbg !29066
  br i1 %_111.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit, label %bb37.i, !dbg !29066

bb37.i:                                           ; preds = %bb37.i.lr.ph, %bb13.i.loopexit
  %indvars.iv17482 = phi i64 [ %frames, %bb37.i.lr.ph ], [ %indvars.iv.next17483, %bb13.i.loopexit ]
  %main_cursor.sroa.0.0.i68914888 = phi i64 [ %611, %bb37.i.lr.ph ], [ %main_cursor.sroa.0.1.i698.lcssa, %bb13.i.loopexit ]
  %ring_cursor.sroa.0.0.i68814887 = phi i64 [ %610, %bb37.i.lr.ph ], [ %ring_cursor.sroa.0.1.i697.lcssa, %bb13.i.loopexit ]
  %iter4.sroa.0.0.i68714886 = phi i64 [ %yield_count.sroa.0.0.i.i6724, %bb37.i.lr.ph ], [ %689, %bb13.i.loopexit ]
  %iter.sroa.0.0.i14885 = phi i64 [ 0, %bb37.i.lr.ph ], [ %688, %bb13.i.loopexit ]
  %687 = call i64 @llvm.umax.i64(i64 %indvars.iv17482, i64 1), !dbg !29114
  %umax17515 = call i64 @llvm.umin.i64(i64 %687, i64 32), !dbg !29114
  %688 = add nuw nsw i64 %iter.sroa.0.0.i14885, 32, !dbg !29114
  %689 = add nsw i64 %iter4.sroa.0.0.i68714886, -1, !dbg !29118
  %history.i310.i.sroa.0.0.copyload = load <8 x float>, ptr %hot_left.i675, align 32, !dbg !29119
  %history.i310.i.sroa.10.sroa.0.0.copyload = load <8 x float>, ptr %history.i310.i.sroa.10.0.hot_left.i675.sroa_idx, align 32, !dbg !29119
  %history.i310.i.sroa.13.sroa.0.0.copyload = load <8 x float>, ptr %history.i310.i.sroa.13.0.hot_left.i675.sroa_idx, align 32, !dbg !29119
  %history.i310.i.sroa.16.sroa.0.0.copyload = load <8 x float>, ptr %history.i310.i.sroa.16.0.hot_left.i675.sroa_idx, align 32, !dbg !29119
  %history.i310.i.sroa.19.sroa.0.0.copyload = load <8 x float>, ptr %history.i310.i.sroa.19.0.hot_left.i675.sroa_idx, align 32, !dbg !29119
  %history.i310.i.sroa.22.sroa.0.0.copyload = load <8 x float>, ptr %history.i310.i.sroa.22.0.hot_left.i675.sroa_idx, align 32, !dbg !29119
  %history.i310.i.sroa.25.sroa.0.0.copyload = load <8 x float>, ptr %history.i310.i.sroa.25.0.hot_left.i675.sroa_idx, align 32, !dbg !29119
  %history.i310.i.sroa.29.sroa.0.0.copyload = load <8 x float>, ptr %history.i310.i.sroa.29.0.hot_left.i675.sroa_idx, align 32, !dbg !29119
  %history.i310.i.sroa.32.sroa.0.0.copyload = load <8 x float>, ptr %history.i310.i.sroa.32.0.hot_left.i675.sroa_idx, align 32, !dbg !29119
  %history.i310.i.sroa.35.sroa.0.0.copyload = load <8 x float>, ptr %history.i310.i.sroa.35.0.hot_left.i675.sroa_idx, align 32, !dbg !29119
  %history.i310.i.sroa.38.sroa.0.0.copyload = load <8 x float>, ptr %history.i310.i.sroa.38.0.hot_left.i675.sroa_idx, align 32, !dbg !29119
  %history.i310.i.sroa.41.sroa.0.0.copyload = load <8 x float>, ptr %history.i310.i.sroa.41.0.hot_left.i675.sroa_idx, align 32, !dbg !29119
  %_20.i313.i14657.not = icmp eq i64 %frames, %iter.sroa.0.0.i14885, !dbg !29121
  br i1 %_20.i313.i14657.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit345.i, label %bb5.i314.i.lr.ph, !dbg !29125

bb5.i314.i.lr.ph:                                 ; preds = %bb37.i
  %_5.i4245 = load <8 x float>, ptr %self, align 32
  %_14.i.i.i275.i.sroa.0.0.copyload = load <8 x float>, ptr %615, align 32
  %_17.i.i.i272.i.sroa.0.0.copyload = load <8 x float>, ptr %616, align 32
  %_20.i.i.i269.i.sroa.0.0.copyload = load <8 x float>, ptr %617, align 32
  %_25.i.i.i265.i.sroa.0.0.copyload = load <8 x float>, ptr %row12.i.i.i321.i, align 32
  %_28.i.i.i262.i.sroa.0.0.copyload = load <8 x float>, ptr %618, align 32
  %_31.i.i.i259.i.sroa.0.0.copyload = load <8 x float>, ptr %619, align 32
  %_34.i.i.i256.i.sroa.0.0.copyload = load <8 x float>, ptr %620, align 32
  %_39.i.i.i252.i.sroa.0.0.copyload = load <8 x float>, ptr %row13.i.i.i322.i, align 32
  %_42.i.i.i249.i.sroa.0.0.copyload = load <8 x float>, ptr %621, align 32
  %_45.i.i.i246.i.sroa.0.0.copyload = load <8 x float>, ptr %622, align 32
  %_48.i.i.i243.i.sroa.0.0.copyload = load <8 x float>, ptr %623, align 32
  %_53.i.i.i239.i.sroa.0.0.copyload = load <8 x float>, ptr %row14.i.i.i323.i, align 32
  %_56.i.i.i236.i.sroa.0.0.copyload = load <8 x float>, ptr %624, align 32
  %_59.i.i.i233.i.sroa.0.0.copyload = load <8 x float>, ptr %625, align 32
  %_62.i.i.i230.i.sroa.0.0.copyload = load <8 x float>, ptr %626, align 32
  %_67.i.i.i226.i.sroa.0.0.copyload = load <8 x float>, ptr %row15.i.i.i324.i, align 32
  %_70.i.i.i223.i.sroa.0.0.copyload = load <8 x float>, ptr %627, align 32
  %_73.i.i.i220.i.sroa.0.0.copyload = load <8 x float>, ptr %628, align 32
  %_76.i.i.i217.i.sroa.0.0.copyload = load <8 x float>, ptr %629, align 32
  %_81.i.i.i213.i.sroa.0.0.copyload = load <8 x float>, ptr %row16.i.i.i325.i, align 32
  %_84.i.i.i210.i.sroa.0.0.copyload = load <8 x float>, ptr %630, align 32
  %_87.i.i.i207.i.sroa.0.0.copyload = load <8 x float>, ptr %631, align 32
  %_90.i.i.i204.i.sroa.0.0.copyload = load <8 x float>, ptr %632, align 32
  %_95.i.i.i200.i.sroa.0.0.copyload = load <8 x float>, ptr %row17.i.i.i326.i, align 32
  %_98.i.i.i197.i.sroa.0.0.copyload = load <8 x float>, ptr %633, align 32
  %_101.i.i.i194.i.sroa.0.0.copyload = load <8 x float>, ptr %634, align 32
  %_104.i.i.i191.i.sroa.0.0.copyload = load <8 x float>, ptr %635, align 32
  %_109.i.i.i187.i.sroa.0.0.copyload = load <8 x float>, ptr %row18.i.i.i327.i, align 32
  %_112.i.i.i184.i.sroa.0.0.copyload = load <8 x float>, ptr %636, align 32
  %_115.i.i.i181.i.sroa.0.0.copyload = load <8 x float>, ptr %637, align 32
  %_118.i.i.i178.i.sroa.0.0.copyload = load <8 x float>, ptr %638, align 32
  %_123.i.i.i174.i.sroa.0.0.copyload = load <8 x float>, ptr %row19.i.i.i328.i, align 32
  %_126.i.i.i171.i.sroa.0.0.copyload = load <8 x float>, ptr %639, align 32
  %_129.i.i.i168.i.sroa.0.0.copyload = load <8 x float>, ptr %640, align 32
  %_132.i.i.i165.i.sroa.0.0.copyload = load <8 x float>, ptr %641, align 32
  %_137.i.i.i161.i.sroa.0.0.copyload = load <8 x float>, ptr %row20.i.i.i329.i, align 32
  %_140.i.i.i158.i.sroa.0.0.copyload = load <8 x float>, ptr %642, align 32
  %_143.i.i.i155.i.sroa.0.0.copyload = load <8 x float>, ptr %643, align 32
  %_146.i.i.i152.i.sroa.0.0.copyload = load <8 x float>, ptr %644, align 32
  %_151.i.i.i148.i.sroa.0.0.copyload = load <8 x float>, ptr %row21.i.i.i330.i, align 32
  %_154.i.i.i145.i.sroa.0.0.copyload = load <8 x float>, ptr %645, align 32
  %_157.i.i.i142.i.sroa.0.0.copyload = load <8 x float>, ptr %646, align 32
  %_160.i.i.i139.i.sroa.0.0.copyload = load <8 x float>, ptr %647, align 32
  %_165.i.i.i135.i.sroa.0.0.copyload = load <8 x float>, ptr %row22.i.i.i331.i, align 32
  %_168.i.i.i132.i.sroa.0.0.copyload = load <8 x float>, ptr %648, align 32
  %_171.i.i.i129.i.sroa.0.0.copyload = load <8 x float>, ptr %649, align 32
  %_174.i.i.i126.i.sroa.0.0.copyload = load <8 x float>, ptr %650, align 32
  br label %bb5.i314.i, !dbg !29125

bb5.i314.i:                                       ; preds = %bb5.i314.i.lr.ph, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129
  %iter.sroa.0.0.i312.i14669 = phi i64 [ 0, %bb5.i314.i.lr.ph ], [ %690, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129 ]
  %history.i310.i.sroa.10.sroa.0.014668 = phi <8 x float> [ %history.i310.i.sroa.10.sroa.0.0.copyload, %bb5.i314.i.lr.ph ], [ %history.i310.i.sroa.0.014658, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129 ]
  %history.i310.i.sroa.13.sroa.0.014667 = phi <8 x float> [ %history.i310.i.sroa.13.sroa.0.0.copyload, %bb5.i314.i.lr.ph ], [ %history.i310.i.sroa.10.sroa.0.014668, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129 ]
  %history.i310.i.sroa.16.sroa.0.014666 = phi <8 x float> [ %history.i310.i.sroa.16.sroa.0.0.copyload, %bb5.i314.i.lr.ph ], [ %history.i310.i.sroa.13.sroa.0.014667, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129 ]
  %history.i310.i.sroa.19.sroa.0.014665 = phi <8 x float> [ %history.i310.i.sroa.19.sroa.0.0.copyload, %bb5.i314.i.lr.ph ], [ %history.i310.i.sroa.16.sroa.0.014666, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129 ]
  %history.i310.i.sroa.22.sroa.0.014664 = phi <8 x float> [ %history.i310.i.sroa.22.sroa.0.0.copyload, %bb5.i314.i.lr.ph ], [ %history.i310.i.sroa.19.sroa.0.014665, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129 ]
  %history.i310.i.sroa.38.sroa.0.014663 = phi <8 x float> [ %history.i310.i.sroa.38.sroa.0.0.copyload, %bb5.i314.i.lr.ph ], [ %history.i310.i.sroa.35.sroa.0.014662, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129 ]
  %history.i310.i.sroa.35.sroa.0.014662 = phi <8 x float> [ %history.i310.i.sroa.35.sroa.0.0.copyload, %bb5.i314.i.lr.ph ], [ %history.i310.i.sroa.32.sroa.0.014661, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129 ]
  %history.i310.i.sroa.32.sroa.0.014661 = phi <8 x float> [ %history.i310.i.sroa.32.sroa.0.0.copyload, %bb5.i314.i.lr.ph ], [ %history.i310.i.sroa.29.sroa.0.014660, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129 ]
  %history.i310.i.sroa.29.sroa.0.014660 = phi <8 x float> [ %history.i310.i.sroa.29.sroa.0.0.copyload, %bb5.i314.i.lr.ph ], [ %history.i310.i.sroa.25.sroa.0.014659, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129 ]
  %history.i310.i.sroa.25.sroa.0.014659 = phi <8 x float> [ %history.i310.i.sroa.25.sroa.0.0.copyload, %bb5.i314.i.lr.ph ], [ %history.i310.i.sroa.22.sroa.0.014664, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129 ]
  %history.i310.i.sroa.0.014658 = phi <8 x float> [ %history.i310.i.sroa.0.0.copyload, %bb5.i314.i.lr.ph ], [ %lanes.i5409.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129 ]
  %690 = add nuw nsw i64 %iter.sroa.0.0.i312.i14669, 1, !dbg !29126
  %_11.i315.i = add nuw nsw i64 %iter.sroa.0.0.i312.i14669, %iter.sroa.0.0.i14885, !dbg !29129
  %base.i316.i = shl i64 %_11.i315.i, 3, !dbg !29129
  %_24.i317.i = icmp samesign ugt i64 %base.i316.i, %left_io.1, !dbg !29130
  br i1 %_24.i317.i, label %bb7.i344.i, label %bb8.i318.i, !dbg !29130, !prof !639

bb8.i318.i:                                       ; preds = %bb5.i314.i
  %_27.i319.i = sub nuw nsw i64 %left_io.1, %base.i316.i, !dbg !29133
  %_8.i5412 = icmp samesign ugt i64 %_27.i319.i, 7, !dbg !29134
  br i1 %_8.i5412, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129, label %bb2.i5413, !dbg !29134, !prof !651

bb2.i5413:                                        ; preds = %bb8.i318.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_27.i319.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !29139, !noalias !29140
  unreachable, !dbg !29139

bb7.i344.i:                                       ; preds = %bb5.i314.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i316.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_fc26f793d85338b5649d38df0c19e7e0) #30, !dbg !29147, !noalias !29148
  unreachable, !dbg !29147

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129: ; preds = %bb8.i318.i
  %_31.i320.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %base.i316.i, !dbg !29149
  %lanes.i5409.sroa.0.0.copyload = load <8 x float>, ptr %_31.i320.i, align 4, !dbg !29151, !alias.scope !29155, !noalias !29159
  %691 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i310.i.sroa.22.sroa.0.014664), !dbg !29161
  %692 = fmul <8 x float> %lanes.i5409.sroa.0.0.copyload, %_5.i4245, !dbg !29168
  %693 = fadd <8 x float> %692, zeroinitializer, !dbg !29174
  %694 = fmul <8 x float> %lanes.i5409.sroa.0.0.copyload, %_14.i.i.i275.i.sroa.0.0.copyload, !dbg !29179
  %695 = fadd <8 x float> %694, zeroinitializer, !dbg !29184
  %696 = fmul <8 x float> %lanes.i5409.sroa.0.0.copyload, %_17.i.i.i272.i.sroa.0.0.copyload, !dbg !29189
  %697 = fadd <8 x float> %696, zeroinitializer, !dbg !29194
  %698 = fmul <8 x float> %lanes.i5409.sroa.0.0.copyload, %_20.i.i.i269.i.sroa.0.0.copyload, !dbg !29199
  %699 = fadd <8 x float> %698, zeroinitializer, !dbg !29204
  %700 = fmul <8 x float> %history.i310.i.sroa.0.014658, %_25.i.i.i265.i.sroa.0.0.copyload, !dbg !29209
  %701 = fadd <8 x float> %693, %700, !dbg !29214
  %702 = fmul <8 x float> %history.i310.i.sroa.0.014658, %_28.i.i.i262.i.sroa.0.0.copyload, !dbg !29219
  %703 = fadd <8 x float> %695, %702, !dbg !29224
  %704 = fmul <8 x float> %history.i310.i.sroa.0.014658, %_31.i.i.i259.i.sroa.0.0.copyload, !dbg !29229
  %705 = fadd <8 x float> %697, %704, !dbg !29234
  %706 = fmul <8 x float> %history.i310.i.sroa.0.014658, %_34.i.i.i256.i.sroa.0.0.copyload, !dbg !29239
  %707 = fadd <8 x float> %699, %706, !dbg !29244
  %708 = fmul <8 x float> %history.i310.i.sroa.10.sroa.0.014668, %_39.i.i.i252.i.sroa.0.0.copyload, !dbg !29249
  %709 = fadd <8 x float> %701, %708, !dbg !29254
  %710 = fmul <8 x float> %history.i310.i.sroa.10.sroa.0.014668, %_42.i.i.i249.i.sroa.0.0.copyload, !dbg !29259
  %711 = fadd <8 x float> %703, %710, !dbg !29264
  %712 = fmul <8 x float> %history.i310.i.sroa.10.sroa.0.014668, %_45.i.i.i246.i.sroa.0.0.copyload, !dbg !29269
  %713 = fadd <8 x float> %705, %712, !dbg !29274
  %714 = fmul <8 x float> %history.i310.i.sroa.10.sroa.0.014668, %_48.i.i.i243.i.sroa.0.0.copyload, !dbg !29279
  %715 = fadd <8 x float> %707, %714, !dbg !29284
  %716 = fmul <8 x float> %history.i310.i.sroa.13.sroa.0.014667, %_53.i.i.i239.i.sroa.0.0.copyload, !dbg !29289
  %717 = fadd <8 x float> %709, %716, !dbg !29294
  %718 = fmul <8 x float> %history.i310.i.sroa.13.sroa.0.014667, %_56.i.i.i236.i.sroa.0.0.copyload, !dbg !29299
  %719 = fadd <8 x float> %711, %718, !dbg !29304
  %720 = fmul <8 x float> %history.i310.i.sroa.13.sroa.0.014667, %_59.i.i.i233.i.sroa.0.0.copyload, !dbg !29309
  %721 = fadd <8 x float> %713, %720, !dbg !29314
  %722 = fmul <8 x float> %history.i310.i.sroa.13.sroa.0.014667, %_62.i.i.i230.i.sroa.0.0.copyload, !dbg !29319
  %723 = fadd <8 x float> %715, %722, !dbg !29324
  %724 = fmul <8 x float> %history.i310.i.sroa.16.sroa.0.014666, %_67.i.i.i226.i.sroa.0.0.copyload, !dbg !29329
  %725 = fadd <8 x float> %717, %724, !dbg !29334
  %726 = fmul <8 x float> %history.i310.i.sroa.16.sroa.0.014666, %_70.i.i.i223.i.sroa.0.0.copyload, !dbg !29339
  %727 = fadd <8 x float> %719, %726, !dbg !29344
  %728 = fmul <8 x float> %history.i310.i.sroa.16.sroa.0.014666, %_73.i.i.i220.i.sroa.0.0.copyload, !dbg !29349
  %729 = fadd <8 x float> %721, %728, !dbg !29354
  %730 = fmul <8 x float> %history.i310.i.sroa.16.sroa.0.014666, %_76.i.i.i217.i.sroa.0.0.copyload, !dbg !29359
  %731 = fadd <8 x float> %723, %730, !dbg !29364
  %732 = fmul <8 x float> %history.i310.i.sroa.19.sroa.0.014665, %_81.i.i.i213.i.sroa.0.0.copyload, !dbg !29369
  %733 = fadd <8 x float> %725, %732, !dbg !29374
  %734 = fmul <8 x float> %history.i310.i.sroa.19.sroa.0.014665, %_84.i.i.i210.i.sroa.0.0.copyload, !dbg !29379
  %735 = fadd <8 x float> %727, %734, !dbg !29384
  %736 = fmul <8 x float> %history.i310.i.sroa.19.sroa.0.014665, %_87.i.i.i207.i.sroa.0.0.copyload, !dbg !29389
  %737 = fadd <8 x float> %729, %736, !dbg !29394
  %738 = fmul <8 x float> %history.i310.i.sroa.19.sroa.0.014665, %_90.i.i.i204.i.sroa.0.0.copyload, !dbg !29399
  %739 = fadd <8 x float> %731, %738, !dbg !29404
  %740 = fmul <8 x float> %history.i310.i.sroa.22.sroa.0.014664, %_95.i.i.i200.i.sroa.0.0.copyload, !dbg !29409
  %741 = fadd <8 x float> %733, %740, !dbg !29414
  %742 = fmul <8 x float> %history.i310.i.sroa.22.sroa.0.014664, %_98.i.i.i197.i.sroa.0.0.copyload, !dbg !29419
  %743 = fadd <8 x float> %735, %742, !dbg !29424
  %744 = fmul <8 x float> %history.i310.i.sroa.22.sroa.0.014664, %_101.i.i.i194.i.sroa.0.0.copyload, !dbg !29429
  %745 = fadd <8 x float> %737, %744, !dbg !29434
  %746 = fmul <8 x float> %history.i310.i.sroa.22.sroa.0.014664, %_104.i.i.i191.i.sroa.0.0.copyload, !dbg !29439
  %747 = fadd <8 x float> %739, %746, !dbg !29444
  %748 = fmul <8 x float> %history.i310.i.sroa.25.sroa.0.014659, %_109.i.i.i187.i.sroa.0.0.copyload, !dbg !29449
  %749 = fadd <8 x float> %741, %748, !dbg !29454
  %750 = fmul <8 x float> %history.i310.i.sroa.25.sroa.0.014659, %_112.i.i.i184.i.sroa.0.0.copyload, !dbg !29459
  %751 = fadd <8 x float> %743, %750, !dbg !29464
  %752 = fmul <8 x float> %history.i310.i.sroa.25.sroa.0.014659, %_115.i.i.i181.i.sroa.0.0.copyload, !dbg !29469
  %753 = fadd <8 x float> %745, %752, !dbg !29474
  %754 = fmul <8 x float> %history.i310.i.sroa.25.sroa.0.014659, %_118.i.i.i178.i.sroa.0.0.copyload, !dbg !29479
  %755 = fadd <8 x float> %747, %754, !dbg !29484
  %756 = fmul <8 x float> %history.i310.i.sroa.29.sroa.0.014660, %_123.i.i.i174.i.sroa.0.0.copyload, !dbg !29489
  %757 = fadd <8 x float> %749, %756, !dbg !29494
  %758 = fmul <8 x float> %history.i310.i.sroa.29.sroa.0.014660, %_126.i.i.i171.i.sroa.0.0.copyload, !dbg !29499
  %759 = fadd <8 x float> %751, %758, !dbg !29504
  %760 = fmul <8 x float> %history.i310.i.sroa.29.sroa.0.014660, %_129.i.i.i168.i.sroa.0.0.copyload, !dbg !29509
  %761 = fadd <8 x float> %753, %760, !dbg !29514
  %762 = fmul <8 x float> %history.i310.i.sroa.29.sroa.0.014660, %_132.i.i.i165.i.sroa.0.0.copyload, !dbg !29519
  %763 = fadd <8 x float> %755, %762, !dbg !29524
  %764 = fmul <8 x float> %history.i310.i.sroa.32.sroa.0.014661, %_137.i.i.i161.i.sroa.0.0.copyload, !dbg !29529
  %765 = fadd <8 x float> %757, %764, !dbg !29534
  %766 = fmul <8 x float> %history.i310.i.sroa.32.sroa.0.014661, %_140.i.i.i158.i.sroa.0.0.copyload, !dbg !29539
  %767 = fadd <8 x float> %759, %766, !dbg !29544
  %768 = fmul <8 x float> %history.i310.i.sroa.32.sroa.0.014661, %_143.i.i.i155.i.sroa.0.0.copyload, !dbg !29549
  %769 = fadd <8 x float> %761, %768, !dbg !29554
  %770 = fmul <8 x float> %history.i310.i.sroa.32.sroa.0.014661, %_146.i.i.i152.i.sroa.0.0.copyload, !dbg !29559
  %771 = fadd <8 x float> %763, %770, !dbg !29564
  %772 = fmul <8 x float> %history.i310.i.sroa.35.sroa.0.014662, %_151.i.i.i148.i.sroa.0.0.copyload, !dbg !29569
  %773 = fadd <8 x float> %765, %772, !dbg !29574
  %774 = fmul <8 x float> %history.i310.i.sroa.35.sroa.0.014662, %_154.i.i.i145.i.sroa.0.0.copyload, !dbg !29579
  %775 = fadd <8 x float> %767, %774, !dbg !29584
  %776 = fmul <8 x float> %history.i310.i.sroa.35.sroa.0.014662, %_157.i.i.i142.i.sroa.0.0.copyload, !dbg !29589
  %777 = fadd <8 x float> %769, %776, !dbg !29594
  %778 = fmul <8 x float> %history.i310.i.sroa.35.sroa.0.014662, %_160.i.i.i139.i.sroa.0.0.copyload, !dbg !29599
  %779 = fadd <8 x float> %771, %778, !dbg !29604
  %780 = fmul <8 x float> %history.i310.i.sroa.38.sroa.0.014663, %_165.i.i.i135.i.sroa.0.0.copyload, !dbg !29609
  %781 = fadd <8 x float> %773, %780, !dbg !29614
  %782 = fmul <8 x float> %history.i310.i.sroa.38.sroa.0.014663, %_168.i.i.i132.i.sroa.0.0.copyload, !dbg !29619
  %783 = fadd <8 x float> %775, %782, !dbg !29624
  %784 = fmul <8 x float> %history.i310.i.sroa.38.sroa.0.014663, %_171.i.i.i129.i.sroa.0.0.copyload, !dbg !29629
  %785 = fadd <8 x float> %777, %784, !dbg !29634
  %786 = fmul <8 x float> %history.i310.i.sroa.38.sroa.0.014663, %_174.i.i.i126.i.sroa.0.0.copyload, !dbg !29639
  %787 = fadd <8 x float> %779, %786, !dbg !29644
  %788 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %781), !dbg !29649
  %789 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %691, <8 x float> %788), !dbg !29655
  %790 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %783), !dbg !29649
  %791 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %789, <8 x float> %790), !dbg !29655
  %792 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %785), !dbg !29649
  %793 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %791, <8 x float> %792), !dbg !29655
  %794 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %787), !dbg !29649
  %795 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %793, <8 x float> %794), !dbg !29655
  %_39.i341.i.idx = shl i64 %iter.sroa.0.0.i312.i14669, 5, !dbg !29660
  %_39.i341.i = getelementptr inbounds nuw i8, ptr %peaks_left.i668, i64 %_39.i341.i.idx, !dbg !29660
  store <8 x float> %795, ptr %_39.i341.i, align 4, !dbg !29665, !alias.scope !29670, !noalias !29674
  %exitcond17486.not = icmp eq i64 %690, %umax17515, !dbg !29121
  br i1 %exitcond17486.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit345.i, label %bb5.i314.i, !dbg !29125

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit345.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129, %bb37.i
  %history.i310.i.sroa.0.0.lcssa = phi <8 x float> [ %history.i310.i.sroa.0.0.copyload, %bb37.i ], [ %lanes.i5409.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129 ], !dbg !29678
  %history.i310.i.sroa.25.sroa.0.0.lcssa = phi <8 x float> [ %history.i310.i.sroa.25.sroa.0.0.copyload, %bb37.i ], [ %history.i310.i.sroa.22.sroa.0.014664, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129 ], !dbg !29678
  %history.i310.i.sroa.29.sroa.0.0.lcssa = phi <8 x float> [ %history.i310.i.sroa.29.sroa.0.0.copyload, %bb37.i ], [ %history.i310.i.sroa.25.sroa.0.014659, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129 ], !dbg !29678
  %history.i310.i.sroa.32.sroa.0.0.lcssa = phi <8 x float> [ %history.i310.i.sroa.32.sroa.0.0.copyload, %bb37.i ], [ %history.i310.i.sroa.29.sroa.0.014660, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129 ], !dbg !29678
  %history.i310.i.sroa.35.sroa.0.0.lcssa = phi <8 x float> [ %history.i310.i.sroa.35.sroa.0.0.copyload, %bb37.i ], [ %history.i310.i.sroa.32.sroa.0.014661, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129 ], !dbg !29678
  %history.i310.i.sroa.38.sroa.0.0.lcssa = phi <8 x float> [ %history.i310.i.sroa.38.sroa.0.0.copyload, %bb37.i ], [ %history.i310.i.sroa.35.sroa.0.014662, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129 ], !dbg !29678
  %history.i310.i.sroa.41.sroa.0.0.lcssa = phi <8 x float> [ %history.i310.i.sroa.41.sroa.0.0.copyload, %bb37.i ], [ %history.i310.i.sroa.38.sroa.0.014663, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129 ], !dbg !29678
  %history.i310.i.sroa.22.sroa.0.0.lcssa = phi <8 x float> [ %history.i310.i.sroa.22.sroa.0.0.copyload, %bb37.i ], [ %history.i310.i.sroa.19.sroa.0.014665, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129 ], !dbg !29678
  %history.i310.i.sroa.19.sroa.0.0.lcssa = phi <8 x float> [ %history.i310.i.sroa.19.sroa.0.0.copyload, %bb37.i ], [ %history.i310.i.sroa.16.sroa.0.014666, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129 ], !dbg !29678
  %history.i310.i.sroa.16.sroa.0.0.lcssa = phi <8 x float> [ %history.i310.i.sroa.16.sroa.0.0.copyload, %bb37.i ], [ %history.i310.i.sroa.13.sroa.0.014667, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129 ], !dbg !29678
  %history.i310.i.sroa.13.sroa.0.0.lcssa = phi <8 x float> [ %history.i310.i.sroa.13.sroa.0.0.copyload, %bb37.i ], [ %history.i310.i.sroa.10.sroa.0.014668, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129 ], !dbg !29678
  %history.i310.i.sroa.10.sroa.0.0.lcssa = phi <8 x float> [ %history.i310.i.sroa.10.sroa.0.0.copyload, %bb37.i ], [ %history.i310.i.sroa.0.014658, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6129 ], !dbg !29678
  store <8 x float> %history.i310.i.sroa.0.0.lcssa, ptr %hot_left.i675, align 32, !dbg !29679
  store <8 x float> %history.i310.i.sroa.10.sroa.0.0.lcssa, ptr %history.i310.i.sroa.10.0.hot_left.i675.sroa_idx, align 32, !dbg !29679
  store <8 x float> %history.i310.i.sroa.13.sroa.0.0.lcssa, ptr %history.i310.i.sroa.13.0.hot_left.i675.sroa_idx, align 32, !dbg !29679
  store <8 x float> %history.i310.i.sroa.16.sroa.0.0.lcssa, ptr %history.i310.i.sroa.16.0.hot_left.i675.sroa_idx, align 32, !dbg !29679
  store <8 x float> %history.i310.i.sroa.19.sroa.0.0.lcssa, ptr %history.i310.i.sroa.19.0.hot_left.i675.sroa_idx, align 32, !dbg !29679
  store <8 x float> %history.i310.i.sroa.22.sroa.0.0.lcssa, ptr %history.i310.i.sroa.22.0.hot_left.i675.sroa_idx, align 32, !dbg !29679
  store <8 x float> %history.i310.i.sroa.25.sroa.0.0.lcssa, ptr %history.i310.i.sroa.25.0.hot_left.i675.sroa_idx, align 32, !dbg !29679
  store <8 x float> %history.i310.i.sroa.29.sroa.0.0.lcssa, ptr %history.i310.i.sroa.29.0.hot_left.i675.sroa_idx, align 32, !dbg !29679
  store <8 x float> %history.i310.i.sroa.32.sroa.0.0.lcssa, ptr %history.i310.i.sroa.32.0.hot_left.i675.sroa_idx, align 32, !dbg !29679
  store <8 x float> %history.i310.i.sroa.35.sroa.0.0.lcssa, ptr %history.i310.i.sroa.35.0.hot_left.i675.sroa_idx, align 32, !dbg !29679
  store <8 x float> %history.i310.i.sroa.38.sroa.0.0.lcssa, ptr %history.i310.i.sroa.38.0.hot_left.i675.sroa_idx, align 32, !dbg !29679
  store <8 x float> %history.i310.i.sroa.41.sroa.0.0.lcssa, ptr %history.i310.i.sroa.41.0.hot_left.i675.sroa_idx, align 32, !dbg !29679
  %history.i.i640.sroa.0.0.copyload = load <8 x float>, ptr %hot_right.i674, align 32, !dbg !29680
  %history.i.i640.sroa.10.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i640.sroa.10.0.hot_right.i674.sroa_idx, align 32, !dbg !29680
  %history.i.i640.sroa.13.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i640.sroa.13.0.hot_right.i674.sroa_idx, align 32, !dbg !29680
  %history.i.i640.sroa.16.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i640.sroa.16.0.hot_right.i674.sroa_idx, align 32, !dbg !29680
  %history.i.i640.sroa.19.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i640.sroa.19.0.hot_right.i674.sroa_idx, align 32, !dbg !29680
  %history.i.i640.sroa.22.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i640.sroa.22.0.hot_right.i674.sroa_idx, align 32, !dbg !29680
  %history.i.i640.sroa.25.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i640.sroa.25.0.hot_right.i674.sroa_idx, align 32, !dbg !29680
  %history.i.i640.sroa.29.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i640.sroa.29.0.hot_right.i674.sroa_idx, align 32, !dbg !29680
  %history.i.i640.sroa.32.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i640.sroa.32.0.hot_right.i674.sroa_idx, align 32, !dbg !29680
  %history.i.i640.sroa.35.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i640.sroa.35.0.hot_right.i674.sroa_idx, align 32, !dbg !29680
  %history.i.i640.sroa.38.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i640.sroa.38.0.hot_right.i674.sroa_idx, align 32, !dbg !29680
  %history.i.i640.sroa.41.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i640.sroa.41.0.hot_right.i674.sroa_idx, align 32, !dbg !29680
  br i1 %_20.i313.i14657.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i695, label %bb5.i.i710.lr.ph, !dbg !29682

bb5.i.i710.lr.ph:                                 ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit345.i
  %_5.i4389 = load <8 x float>, ptr %self, align 32
  %_14.i.i.i.i605.sroa.0.0.copyload = load <8 x float>, ptr %615, align 32
  %_17.i.i.i.i602.sroa.0.0.copyload = load <8 x float>, ptr %616, align 32
  %_20.i.i.i.i599.sroa.0.0.copyload = load <8 x float>, ptr %617, align 32
  %_25.i.i.i.i595.sroa.0.0.copyload = load <8 x float>, ptr %row12.i.i.i321.i, align 32
  %_28.i.i.i.i592.sroa.0.0.copyload = load <8 x float>, ptr %618, align 32
  %_31.i.i.i.i589.sroa.0.0.copyload = load <8 x float>, ptr %619, align 32
  %_34.i.i.i.i586.sroa.0.0.copyload = load <8 x float>, ptr %620, align 32
  %_39.i.i.i.i582.sroa.0.0.copyload = load <8 x float>, ptr %row13.i.i.i322.i, align 32
  %_42.i.i.i.i579.sroa.0.0.copyload = load <8 x float>, ptr %621, align 32
  %_45.i.i.i.i576.sroa.0.0.copyload = load <8 x float>, ptr %622, align 32
  %_48.i.i.i.i573.sroa.0.0.copyload = load <8 x float>, ptr %623, align 32
  %_53.i.i.i.i569.sroa.0.0.copyload = load <8 x float>, ptr %row14.i.i.i323.i, align 32
  %_56.i.i.i.i566.sroa.0.0.copyload = load <8 x float>, ptr %624, align 32
  %_59.i.i.i.i563.sroa.0.0.copyload = load <8 x float>, ptr %625, align 32
  %_62.i.i.i.i560.sroa.0.0.copyload = load <8 x float>, ptr %626, align 32
  %_67.i.i.i.i556.sroa.0.0.copyload = load <8 x float>, ptr %row15.i.i.i324.i, align 32
  %_70.i.i.i.i553.sroa.0.0.copyload = load <8 x float>, ptr %627, align 32
  %_73.i.i.i.i550.sroa.0.0.copyload = load <8 x float>, ptr %628, align 32
  %_76.i.i.i.i547.sroa.0.0.copyload = load <8 x float>, ptr %629, align 32
  %_81.i.i.i.i543.sroa.0.0.copyload = load <8 x float>, ptr %row16.i.i.i325.i, align 32
  %_84.i.i.i.i540.sroa.0.0.copyload = load <8 x float>, ptr %630, align 32
  %_87.i.i.i.i537.sroa.0.0.copyload = load <8 x float>, ptr %631, align 32
  %_90.i.i.i.i534.sroa.0.0.copyload = load <8 x float>, ptr %632, align 32
  %_95.i.i.i.i530.sroa.0.0.copyload = load <8 x float>, ptr %row17.i.i.i326.i, align 32
  %_98.i.i.i.i527.sroa.0.0.copyload = load <8 x float>, ptr %633, align 32
  %_101.i.i.i.i524.sroa.0.0.copyload = load <8 x float>, ptr %634, align 32
  %_104.i.i.i.i521.sroa.0.0.copyload = load <8 x float>, ptr %635, align 32
  %_109.i.i.i.i517.sroa.0.0.copyload = load <8 x float>, ptr %row18.i.i.i327.i, align 32
  %_112.i.i.i.i514.sroa.0.0.copyload = load <8 x float>, ptr %636, align 32
  %_115.i.i.i.i511.sroa.0.0.copyload = load <8 x float>, ptr %637, align 32
  %_118.i.i.i.i508.sroa.0.0.copyload = load <8 x float>, ptr %638, align 32
  %_123.i.i.i.i504.sroa.0.0.copyload = load <8 x float>, ptr %row19.i.i.i328.i, align 32
  %_126.i.i.i.i501.sroa.0.0.copyload = load <8 x float>, ptr %639, align 32
  %_129.i.i.i.i498.sroa.0.0.copyload = load <8 x float>, ptr %640, align 32
  %_132.i.i.i.i495.sroa.0.0.copyload = load <8 x float>, ptr %641, align 32
  %_137.i.i.i.i491.sroa.0.0.copyload = load <8 x float>, ptr %row20.i.i.i329.i, align 32
  %_140.i.i.i.i488.sroa.0.0.copyload = load <8 x float>, ptr %642, align 32
  %_143.i.i.i.i485.sroa.0.0.copyload = load <8 x float>, ptr %643, align 32
  %_146.i.i.i.i482.sroa.0.0.copyload = load <8 x float>, ptr %644, align 32
  %_151.i.i.i.i478.sroa.0.0.copyload = load <8 x float>, ptr %row21.i.i.i330.i, align 32
  %_154.i.i.i.i475.sroa.0.0.copyload = load <8 x float>, ptr %645, align 32
  %_157.i.i.i.i472.sroa.0.0.copyload = load <8 x float>, ptr %646, align 32
  %_160.i.i.i.i469.sroa.0.0.copyload = load <8 x float>, ptr %647, align 32
  %_165.i.i.i.i465.sroa.0.0.copyload = load <8 x float>, ptr %row22.i.i.i331.i, align 32
  %_168.i.i.i.i462.sroa.0.0.copyload = load <8 x float>, ptr %648, align 32
  %_171.i.i.i.i459.sroa.0.0.copyload = load <8 x float>, ptr %649, align 32
  %_174.i.i.i.i456.sroa.0.0.copyload = load <8 x float>, ptr %650, align 32
  br label %bb5.i.i710, !dbg !29682

bb5.i.i710:                                       ; preds = %bb5.i.i710.lr.ph, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134
  %iter.sroa.0.0.i.i69314696 = phi i64 [ 0, %bb5.i.i710.lr.ph ], [ %796, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134 ]
  %history.i.i640.sroa.10.sroa.0.014695 = phi <8 x float> [ %history.i.i640.sroa.10.sroa.0.0.copyload, %bb5.i.i710.lr.ph ], [ %history.i.i640.sroa.0.014685, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134 ]
  %history.i.i640.sroa.13.sroa.0.014694 = phi <8 x float> [ %history.i.i640.sroa.13.sroa.0.0.copyload, %bb5.i.i710.lr.ph ], [ %history.i.i640.sroa.10.sroa.0.014695, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134 ]
  %history.i.i640.sroa.16.sroa.0.014693 = phi <8 x float> [ %history.i.i640.sroa.16.sroa.0.0.copyload, %bb5.i.i710.lr.ph ], [ %history.i.i640.sroa.13.sroa.0.014694, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134 ]
  %history.i.i640.sroa.19.sroa.0.014692 = phi <8 x float> [ %history.i.i640.sroa.19.sroa.0.0.copyload, %bb5.i.i710.lr.ph ], [ %history.i.i640.sroa.16.sroa.0.014693, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134 ]
  %history.i.i640.sroa.22.sroa.0.014691 = phi <8 x float> [ %history.i.i640.sroa.22.sroa.0.0.copyload, %bb5.i.i710.lr.ph ], [ %history.i.i640.sroa.19.sroa.0.014692, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134 ]
  %history.i.i640.sroa.38.sroa.0.014690 = phi <8 x float> [ %history.i.i640.sroa.38.sroa.0.0.copyload, %bb5.i.i710.lr.ph ], [ %history.i.i640.sroa.35.sroa.0.014689, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134 ]
  %history.i.i640.sroa.35.sroa.0.014689 = phi <8 x float> [ %history.i.i640.sroa.35.sroa.0.0.copyload, %bb5.i.i710.lr.ph ], [ %history.i.i640.sroa.32.sroa.0.014688, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134 ]
  %history.i.i640.sroa.32.sroa.0.014688 = phi <8 x float> [ %history.i.i640.sroa.32.sroa.0.0.copyload, %bb5.i.i710.lr.ph ], [ %history.i.i640.sroa.29.sroa.0.014687, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134 ]
  %history.i.i640.sroa.29.sroa.0.014687 = phi <8 x float> [ %history.i.i640.sroa.29.sroa.0.0.copyload, %bb5.i.i710.lr.ph ], [ %history.i.i640.sroa.25.sroa.0.014686, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134 ]
  %history.i.i640.sroa.25.sroa.0.014686 = phi <8 x float> [ %history.i.i640.sroa.25.sroa.0.0.copyload, %bb5.i.i710.lr.ph ], [ %history.i.i640.sroa.22.sroa.0.014691, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134 ]
  %history.i.i640.sroa.0.014685 = phi <8 x float> [ %history.i.i640.sroa.0.0.copyload, %bb5.i.i710.lr.ph ], [ %lanes.i5418.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134 ]
  %796 = add nuw nsw i64 %iter.sroa.0.0.i.i69314696, 1, !dbg !29685
  %_11.i.i711 = add nuw nsw i64 %iter.sroa.0.0.i.i69314696, %iter.sroa.0.0.i14885, !dbg !29688
  %base.i.i712 = shl i64 %_11.i.i711, 3, !dbg !29688
  %_24.i.i713 = icmp samesign ugt i64 %base.i.i712, %right_io.1, !dbg !29689
  br i1 %_24.i.i713, label %bb7.i.i738, label %bb8.i.i714, !dbg !29689, !prof !639

bb8.i.i714:                                       ; preds = %bb5.i.i710
  %_27.i.i715 = sub nuw nsw i64 %right_io.1, %base.i.i712, !dbg !29692
  %_8.i5421 = icmp samesign ugt i64 %_27.i.i715, 7, !dbg !29693
  br i1 %_8.i5421, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134, label %bb2.i5422, !dbg !29693, !prof !651

bb2.i5422:                                        ; preds = %bb8.i.i714
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_27.i.i715, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !29698, !noalias !29699
  unreachable, !dbg !29698

bb7.i.i738:                                       ; preds = %bb5.i.i710
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i.i712, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_fc26f793d85338b5649d38df0c19e7e0) #30, !dbg !29706, !noalias !29707
  unreachable, !dbg !29706

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134: ; preds = %bb8.i.i714
  %_31.i124.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %base.i.i712, !dbg !29708
  %lanes.i5418.sroa.0.0.copyload = load <8 x float>, ptr %_31.i124.i, align 4, !dbg !29710, !alias.scope !29714, !noalias !29718
  %797 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i.i640.sroa.22.sroa.0.014691), !dbg !29720
  %798 = fmul <8 x float> %lanes.i5418.sroa.0.0.copyload, %_5.i4389, !dbg !29727
  %799 = fadd <8 x float> %798, zeroinitializer, !dbg !29733
  %800 = fmul <8 x float> %lanes.i5418.sroa.0.0.copyload, %_14.i.i.i.i605.sroa.0.0.copyload, !dbg !29738
  %801 = fadd <8 x float> %800, zeroinitializer, !dbg !29743
  %802 = fmul <8 x float> %lanes.i5418.sroa.0.0.copyload, %_17.i.i.i.i602.sroa.0.0.copyload, !dbg !29748
  %803 = fadd <8 x float> %802, zeroinitializer, !dbg !29753
  %804 = fmul <8 x float> %lanes.i5418.sroa.0.0.copyload, %_20.i.i.i.i599.sroa.0.0.copyload, !dbg !29758
  %805 = fadd <8 x float> %804, zeroinitializer, !dbg !29763
  %806 = fmul <8 x float> %history.i.i640.sroa.0.014685, %_25.i.i.i.i595.sroa.0.0.copyload, !dbg !29768
  %807 = fadd <8 x float> %799, %806, !dbg !29773
  %808 = fmul <8 x float> %history.i.i640.sroa.0.014685, %_28.i.i.i.i592.sroa.0.0.copyload, !dbg !29778
  %809 = fadd <8 x float> %801, %808, !dbg !29783
  %810 = fmul <8 x float> %history.i.i640.sroa.0.014685, %_31.i.i.i.i589.sroa.0.0.copyload, !dbg !29788
  %811 = fadd <8 x float> %803, %810, !dbg !29793
  %812 = fmul <8 x float> %history.i.i640.sroa.0.014685, %_34.i.i.i.i586.sroa.0.0.copyload, !dbg !29798
  %813 = fadd <8 x float> %805, %812, !dbg !29803
  %814 = fmul <8 x float> %history.i.i640.sroa.10.sroa.0.014695, %_39.i.i.i.i582.sroa.0.0.copyload, !dbg !29808
  %815 = fadd <8 x float> %807, %814, !dbg !29813
  %816 = fmul <8 x float> %history.i.i640.sroa.10.sroa.0.014695, %_42.i.i.i.i579.sroa.0.0.copyload, !dbg !29818
  %817 = fadd <8 x float> %809, %816, !dbg !29823
  %818 = fmul <8 x float> %history.i.i640.sroa.10.sroa.0.014695, %_45.i.i.i.i576.sroa.0.0.copyload, !dbg !29828
  %819 = fadd <8 x float> %811, %818, !dbg !29833
  %820 = fmul <8 x float> %history.i.i640.sroa.10.sroa.0.014695, %_48.i.i.i.i573.sroa.0.0.copyload, !dbg !29838
  %821 = fadd <8 x float> %813, %820, !dbg !29843
  %822 = fmul <8 x float> %history.i.i640.sroa.13.sroa.0.014694, %_53.i.i.i.i569.sroa.0.0.copyload, !dbg !29848
  %823 = fadd <8 x float> %815, %822, !dbg !29853
  %824 = fmul <8 x float> %history.i.i640.sroa.13.sroa.0.014694, %_56.i.i.i.i566.sroa.0.0.copyload, !dbg !29858
  %825 = fadd <8 x float> %817, %824, !dbg !29863
  %826 = fmul <8 x float> %history.i.i640.sroa.13.sroa.0.014694, %_59.i.i.i.i563.sroa.0.0.copyload, !dbg !29868
  %827 = fadd <8 x float> %819, %826, !dbg !29873
  %828 = fmul <8 x float> %history.i.i640.sroa.13.sroa.0.014694, %_62.i.i.i.i560.sroa.0.0.copyload, !dbg !29878
  %829 = fadd <8 x float> %821, %828, !dbg !29883
  %830 = fmul <8 x float> %history.i.i640.sroa.16.sroa.0.014693, %_67.i.i.i.i556.sroa.0.0.copyload, !dbg !29888
  %831 = fadd <8 x float> %823, %830, !dbg !29893
  %832 = fmul <8 x float> %history.i.i640.sroa.16.sroa.0.014693, %_70.i.i.i.i553.sroa.0.0.copyload, !dbg !29898
  %833 = fadd <8 x float> %825, %832, !dbg !29903
  %834 = fmul <8 x float> %history.i.i640.sroa.16.sroa.0.014693, %_73.i.i.i.i550.sroa.0.0.copyload, !dbg !29908
  %835 = fadd <8 x float> %827, %834, !dbg !29913
  %836 = fmul <8 x float> %history.i.i640.sroa.16.sroa.0.014693, %_76.i.i.i.i547.sroa.0.0.copyload, !dbg !29918
  %837 = fadd <8 x float> %829, %836, !dbg !29923
  %838 = fmul <8 x float> %history.i.i640.sroa.19.sroa.0.014692, %_81.i.i.i.i543.sroa.0.0.copyload, !dbg !29928
  %839 = fadd <8 x float> %831, %838, !dbg !29933
  %840 = fmul <8 x float> %history.i.i640.sroa.19.sroa.0.014692, %_84.i.i.i.i540.sroa.0.0.copyload, !dbg !29938
  %841 = fadd <8 x float> %833, %840, !dbg !29943
  %842 = fmul <8 x float> %history.i.i640.sroa.19.sroa.0.014692, %_87.i.i.i.i537.sroa.0.0.copyload, !dbg !29948
  %843 = fadd <8 x float> %835, %842, !dbg !29953
  %844 = fmul <8 x float> %history.i.i640.sroa.19.sroa.0.014692, %_90.i.i.i.i534.sroa.0.0.copyload, !dbg !29958
  %845 = fadd <8 x float> %837, %844, !dbg !29963
  %846 = fmul <8 x float> %history.i.i640.sroa.22.sroa.0.014691, %_95.i.i.i.i530.sroa.0.0.copyload, !dbg !29968
  %847 = fadd <8 x float> %839, %846, !dbg !29973
  %848 = fmul <8 x float> %history.i.i640.sroa.22.sroa.0.014691, %_98.i.i.i.i527.sroa.0.0.copyload, !dbg !29978
  %849 = fadd <8 x float> %841, %848, !dbg !29983
  %850 = fmul <8 x float> %history.i.i640.sroa.22.sroa.0.014691, %_101.i.i.i.i524.sroa.0.0.copyload, !dbg !29988
  %851 = fadd <8 x float> %843, %850, !dbg !29993
  %852 = fmul <8 x float> %history.i.i640.sroa.22.sroa.0.014691, %_104.i.i.i.i521.sroa.0.0.copyload, !dbg !29998
  %853 = fadd <8 x float> %845, %852, !dbg !30003
  %854 = fmul <8 x float> %history.i.i640.sroa.25.sroa.0.014686, %_109.i.i.i.i517.sroa.0.0.copyload, !dbg !30008
  %855 = fadd <8 x float> %847, %854, !dbg !30013
  %856 = fmul <8 x float> %history.i.i640.sroa.25.sroa.0.014686, %_112.i.i.i.i514.sroa.0.0.copyload, !dbg !30018
  %857 = fadd <8 x float> %849, %856, !dbg !30023
  %858 = fmul <8 x float> %history.i.i640.sroa.25.sroa.0.014686, %_115.i.i.i.i511.sroa.0.0.copyload, !dbg !30028
  %859 = fadd <8 x float> %851, %858, !dbg !30033
  %860 = fmul <8 x float> %history.i.i640.sroa.25.sroa.0.014686, %_118.i.i.i.i508.sroa.0.0.copyload, !dbg !30038
  %861 = fadd <8 x float> %853, %860, !dbg !30043
  %862 = fmul <8 x float> %history.i.i640.sroa.29.sroa.0.014687, %_123.i.i.i.i504.sroa.0.0.copyload, !dbg !30048
  %863 = fadd <8 x float> %855, %862, !dbg !30053
  %864 = fmul <8 x float> %history.i.i640.sroa.29.sroa.0.014687, %_126.i.i.i.i501.sroa.0.0.copyload, !dbg !30058
  %865 = fadd <8 x float> %857, %864, !dbg !30063
  %866 = fmul <8 x float> %history.i.i640.sroa.29.sroa.0.014687, %_129.i.i.i.i498.sroa.0.0.copyload, !dbg !30068
  %867 = fadd <8 x float> %859, %866, !dbg !30073
  %868 = fmul <8 x float> %history.i.i640.sroa.29.sroa.0.014687, %_132.i.i.i.i495.sroa.0.0.copyload, !dbg !30078
  %869 = fadd <8 x float> %861, %868, !dbg !30083
  %870 = fmul <8 x float> %history.i.i640.sroa.32.sroa.0.014688, %_137.i.i.i.i491.sroa.0.0.copyload, !dbg !30088
  %871 = fadd <8 x float> %863, %870, !dbg !30093
  %872 = fmul <8 x float> %history.i.i640.sroa.32.sroa.0.014688, %_140.i.i.i.i488.sroa.0.0.copyload, !dbg !30098
  %873 = fadd <8 x float> %865, %872, !dbg !30103
  %874 = fmul <8 x float> %history.i.i640.sroa.32.sroa.0.014688, %_143.i.i.i.i485.sroa.0.0.copyload, !dbg !30108
  %875 = fadd <8 x float> %867, %874, !dbg !30113
  %876 = fmul <8 x float> %history.i.i640.sroa.32.sroa.0.014688, %_146.i.i.i.i482.sroa.0.0.copyload, !dbg !30118
  %877 = fadd <8 x float> %869, %876, !dbg !30123
  %878 = fmul <8 x float> %history.i.i640.sroa.35.sroa.0.014689, %_151.i.i.i.i478.sroa.0.0.copyload, !dbg !30128
  %879 = fadd <8 x float> %871, %878, !dbg !30133
  %880 = fmul <8 x float> %history.i.i640.sroa.35.sroa.0.014689, %_154.i.i.i.i475.sroa.0.0.copyload, !dbg !30138
  %881 = fadd <8 x float> %873, %880, !dbg !30143
  %882 = fmul <8 x float> %history.i.i640.sroa.35.sroa.0.014689, %_157.i.i.i.i472.sroa.0.0.copyload, !dbg !30148
  %883 = fadd <8 x float> %875, %882, !dbg !30153
  %884 = fmul <8 x float> %history.i.i640.sroa.35.sroa.0.014689, %_160.i.i.i.i469.sroa.0.0.copyload, !dbg !30158
  %885 = fadd <8 x float> %877, %884, !dbg !30163
  %886 = fmul <8 x float> %history.i.i640.sroa.38.sroa.0.014690, %_165.i.i.i.i465.sroa.0.0.copyload, !dbg !30168
  %887 = fadd <8 x float> %879, %886, !dbg !30173
  %888 = fmul <8 x float> %history.i.i640.sroa.38.sroa.0.014690, %_168.i.i.i.i462.sroa.0.0.copyload, !dbg !30178
  %889 = fadd <8 x float> %881, %888, !dbg !30183
  %890 = fmul <8 x float> %history.i.i640.sroa.38.sroa.0.014690, %_171.i.i.i.i459.sroa.0.0.copyload, !dbg !30188
  %891 = fadd <8 x float> %883, %890, !dbg !30193
  %892 = fmul <8 x float> %history.i.i640.sroa.38.sroa.0.014690, %_174.i.i.i.i456.sroa.0.0.copyload, !dbg !30198
  %893 = fadd <8 x float> %885, %892, !dbg !30203
  %894 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %887), !dbg !30208
  %895 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %797, <8 x float> %894), !dbg !30214
  %896 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %889), !dbg !30208
  %897 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %895, <8 x float> %896), !dbg !30214
  %898 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %891), !dbg !30208
  %899 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %897, <8 x float> %898), !dbg !30214
  %900 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %893), !dbg !30208
  %901 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %899, <8 x float> %900), !dbg !30214
  %_39.i.i735.idx = shl i64 %iter.sroa.0.0.i.i69314696, 5, !dbg !30219
  %_39.i.i735 = getelementptr inbounds nuw i8, ptr %peaks_right.i667, i64 %_39.i.i735.idx, !dbg !30219
  store <8 x float> %901, ptr %_39.i.i735, align 4, !dbg !30224, !alias.scope !30229, !noalias !30233
  %exitcond17491.not = icmp eq i64 %796, %umax17515, !dbg !30237
  br i1 %exitcond17491.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i695, label %bb5.i.i710, !dbg !29682

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i695: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit345.i
  %history.i.i640.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i640.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit345.i ], [ %lanes.i5418.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134 ], !dbg !30239
  %history.i.i640.sroa.25.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i640.sroa.25.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit345.i ], [ %history.i.i640.sroa.22.sroa.0.014691, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134 ], !dbg !30239
  %history.i.i640.sroa.29.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i640.sroa.29.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit345.i ], [ %history.i.i640.sroa.25.sroa.0.014686, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134 ], !dbg !30239
  %history.i.i640.sroa.32.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i640.sroa.32.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit345.i ], [ %history.i.i640.sroa.29.sroa.0.014687, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134 ], !dbg !30239
  %history.i.i640.sroa.35.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i640.sroa.35.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit345.i ], [ %history.i.i640.sroa.32.sroa.0.014688, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134 ], !dbg !30239
  %history.i.i640.sroa.38.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i640.sroa.38.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit345.i ], [ %history.i.i640.sroa.35.sroa.0.014689, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134 ], !dbg !30239
  %history.i.i640.sroa.41.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i640.sroa.41.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit345.i ], [ %history.i.i640.sroa.38.sroa.0.014690, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134 ], !dbg !30239
  %history.i.i640.sroa.22.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i640.sroa.22.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit345.i ], [ %history.i.i640.sroa.19.sroa.0.014692, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134 ], !dbg !30239
  %history.i.i640.sroa.19.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i640.sroa.19.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit345.i ], [ %history.i.i640.sroa.16.sroa.0.014693, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134 ], !dbg !30239
  %history.i.i640.sroa.16.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i640.sroa.16.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit345.i ], [ %history.i.i640.sroa.13.sroa.0.014694, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134 ], !dbg !30239
  %history.i.i640.sroa.13.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i640.sroa.13.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit345.i ], [ %history.i.i640.sroa.10.sroa.0.014695, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134 ], !dbg !30239
  %history.i.i640.sroa.10.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i640.sroa.10.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit345.i ], [ %history.i.i640.sroa.0.014685, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6134 ], !dbg !30239
  store <8 x float> %history.i.i640.sroa.0.0.lcssa, ptr %hot_right.i674, align 32, !dbg !30240
  store <8 x float> %history.i.i640.sroa.10.sroa.0.0.lcssa, ptr %history.i.i640.sroa.10.0.hot_right.i674.sroa_idx, align 32, !dbg !30240
  store <8 x float> %history.i.i640.sroa.13.sroa.0.0.lcssa, ptr %history.i.i640.sroa.13.0.hot_right.i674.sroa_idx, align 32, !dbg !30240
  store <8 x float> %history.i.i640.sroa.16.sroa.0.0.lcssa, ptr %history.i.i640.sroa.16.0.hot_right.i674.sroa_idx, align 32, !dbg !30240
  store <8 x float> %history.i.i640.sroa.19.sroa.0.0.lcssa, ptr %history.i.i640.sroa.19.0.hot_right.i674.sroa_idx, align 32, !dbg !30240
  store <8 x float> %history.i.i640.sroa.22.sroa.0.0.lcssa, ptr %history.i.i640.sroa.22.0.hot_right.i674.sroa_idx, align 32, !dbg !30240
  store <8 x float> %history.i.i640.sroa.25.sroa.0.0.lcssa, ptr %history.i.i640.sroa.25.0.hot_right.i674.sroa_idx, align 32, !dbg !30240
  store <8 x float> %history.i.i640.sroa.29.sroa.0.0.lcssa, ptr %history.i.i640.sroa.29.0.hot_right.i674.sroa_idx, align 32, !dbg !30240
  store <8 x float> %history.i.i640.sroa.32.sroa.0.0.lcssa, ptr %history.i.i640.sroa.32.0.hot_right.i674.sroa_idx, align 32, !dbg !30240
  store <8 x float> %history.i.i640.sroa.35.sroa.0.0.lcssa, ptr %history.i.i640.sroa.35.0.hot_right.i674.sroa_idx, align 32, !dbg !30240
  store <8 x float> %history.i.i640.sroa.38.sroa.0.0.lcssa, ptr %history.i.i640.sroa.38.0.hot_right.i674.sroa_idx, align 32, !dbg !30240
  store <8 x float> %history.i.i640.sroa.41.sroa.0.0.lcssa, ptr %history.i.i640.sroa.41.0.hot_right.i674.sroa_idx, align 32, !dbg !30240
  br i1 %_20.i313.i14657.not, label %bb13.i.loopexit, label %bb40.i.lr.ph, !dbg !29106

bb40.i.lr.ph:                                     ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i695
  %.promoted = load <8 x float>, ptr %663, align 32
  %.promoted14810 = load <8 x float>, ptr %681, align 32
  %_8.i27.i.sroa.0.0.copyload.pre = load <8 x float>, ptr %_63.i700, align 32, !dbg !30241
  %_9.i26.i.sroa.0.0.copyload.pre = load <8 x float>, ptr %_64.i, align 32, !dbg !30246
  %_8.i.i655.sroa.0.0.copyload.pre = load <8 x float>, ptr %_68.i, align 32, !dbg !30248
  %_9.i.i654.sroa.0.0.copyload.pre = load <8 x float>, ptr %_69.i701, align 32, !dbg !30251
  %_64.i43.i.sroa.0.0.copyload = load <8 x float>, ptr %664, align 32
  %_64.i.i.sroa.0.0.copyload = load <8 x float>, ptr %682, align 32
  br label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5507, !dbg !29106

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5507: ; preds = %bb40.i.lr.ph, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6139
  %_58.i.i.sroa.0.0.copyload14811 = phi <8 x float> [ %.promoted14810, %bb40.i.lr.ph ], [ %1024, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6139 ]
  %_58.i46.i.sroa.0.0.copyload14737 = phi <8 x float> [ %.promoted, %bb40.i.lr.ph ], [ %937, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6139 ]
  %main_cursor.sroa.0.1.i69814734 = phi i64 [ %main_cursor.sroa.0.0.i68914888, %bb40.i.lr.ph ], [ %spec.store.select.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6139 ]
  %ring_cursor.sroa.0.1.i69714733 = phi i64 [ %ring_cursor.sroa.0.0.i68814887, %bb40.i.lr.ph ], [ %spec.store.select13.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6139 ]
  %iter3.sroa.0.0.i69614732 = phi i64 [ 0, %bb40.i.lr.ph ], [ %902, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6139 ]
  %902 = add nuw nsw i64 %iter3.sroa.0.0.i69614732, 1, !dbg !30253
  %_59.i = add nuw nsw i64 %iter3.sroa.0.0.i69614732, %iter.sroa.0.0.i14885, !dbg !30259
  %base.i699 = shl i64 %_59.i, 3, !dbg !30259
  %_73.i702 = shl i64 %iter3.sroa.0.0.i69614732, 3, !dbg !30260
  %_128.i = getelementptr inbounds nuw float, ptr %peaks_left.i668, i64 %_73.i702, !dbg !30261
  %lanes.i5500.sroa.0.0.copyload = load <8 x float>, ptr %_128.i, align 4, !dbg !30272, !alias.scope !30277, !noalias !30281
  %_133.i = getelementptr inbounds nuw float, ptr %peaks_right.i667, i64 %_73.i702, !dbg !30285
  %lanes.i5491.sroa.0.0.copyload = load <8 x float>, ptr %_133.i, align 4, !dbg !30295, !alias.scope !30300, !noalias !30304
  %903 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i5491.sroa.0.0.copyload, <8 x float> %lanes.i5500.sroa.0.0.copyload), !dbg !30308
  %904 = select <8 x i1> %652, <8 x float> %903, <8 x float> %lanes.i5500.sroa.0.0.copyload, !dbg !30313
  %905 = select <8 x i1> %652, <8 x float> %903, <8 x float> %lanes.i5491.sroa.0.0.copyload, !dbg !30318
  %_134.i = icmp samesign ugt i64 %base.i699, %left_io.1, !dbg !30323
  br i1 %_134.i, label %bb44.i, label %bb45.i, !dbg !30323, !prof !639

bb45.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5507
  %_137.i = sub nuw nsw i64 %left_io.1, %base.i699, !dbg !30327
  %_141.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %base.i699, !dbg !30328
  %_8.i5485 = icmp samesign ugt i64 %_137.i, 7, !dbg !30333
  br i1 %_8.i5485, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5489, label %bb2.i5486, !dbg !30333, !prof !651

bb2.i5486:                                        ; preds = %bb45.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_137.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !30338, !noalias !30339
  unreachable, !dbg !30338

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5489: ; preds = %bb45.i
  %_86.i = load i64, ptr %653, align 8, !dbg !30343, !alias.scope !29031, !noalias !30344, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !30345), !dbg !30348
  %width.i61.i = load i64, ptr %654, align 8, !dbg !30349, !alias.scope !30350, !noalias !30351, !noundef !12
  %906 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %904, <8 x float> %_8.i27.i.sroa.0.0.copyload.pre, i8 30), !dbg !30360
  %907 = fdiv <8 x float> %_8.i27.i.sroa.0.0.copyload.pre, %904, !dbg !30366
  %908 = bitcast <8 x float> %906 to <8 x i32>, !dbg !30371
  %909 = icmp slt <8 x i32> %908, zeroinitializer, !dbg !30375
  %910 = select <8 x i1> %909, <8 x float> %907, <8 x float> splat (float 1.000000e+00), !dbg !30375
  %_144.1.i62.i = load i64, ptr %655, align 8, !dbg !30377, !alias.scope !30350, !noalias !30351, !noundef !12
  %_22.i63.i = mul i64 %width.i61.i, %ring_cursor.sroa.0.1.i69714733, !dbg !30378
  %_92.i64.i = icmp ugt i64 %_22.i63.i, %_144.1.i62.i, !dbg !30379
  br i1 %_92.i64.i, label %bb37.i122.i, label %bb38.i65.i, !dbg !30379, !prof !639

bb38.i65.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5489
  %_95.i67.i = sub nuw i64 %_144.1.i62.i, %_22.i63.i, !dbg !30382
  %_8.i6171 = icmp samesign ugt i64 %_95.i67.i, 7, !dbg !30383
  br i1 %_8.i6171, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6174, label %bb2.i6172, !dbg !30383, !prof !651

bb2.i6172:                                        ; preds = %bb38.i65.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_95.i67.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !30388, !noalias !30389
  unreachable, !dbg !30388

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6174: ; preds = %bb38.i65.i
  %_144.0.i66.i = load ptr, ptr %656, align 8, !dbg !30377, !alias.scope !30350, !noalias !30351, !nonnull !12, !noundef !12
  %_99.i68.i = getelementptr inbounds nuw float, ptr %_144.0.i66.i, i64 %_22.i63.i, !dbg !30393
  store <8 x float> %910, ptr %_99.i68.i, align 4, !dbg !30395, !alias.scope !30399, !noalias !30403
  tail call void @llvm.experimental.noalias.scope.decl(metadata !30405), !dbg !30408
  %width.i1836 = load i64, ptr %654, align 8, !dbg !30409, !alias.scope !30405, !noalias !30411, !noundef !12
  %911 = icmp eq i64 %width.i1836, 0, !dbg !30413
  br i1 %911, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1944, label %bb32.i1843.lr.ph, !dbg !30413

bb32.i1843.lr.ph:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6174
  %_112.1.i1846 = load i64, ptr %62, align 8, !alias.scope !30405, !noalias !30411, !noundef !12
  %_112.0.i1850 = load ptr, ptr %61, align 8, !nonnull !12
  %912 = add i64 %ring_cursor.sroa.0.1.i69714733, 1
  %_23.not.i1857 = icmp ult i64 %912, %_86.i
  %913 = select i1 %_23.not.i1857, i64 0, i64 %_86.i
  %start1.sroa.0.0.i1858 = sub nuw i64 %912, %913
  %_114.1.i1861 = load i64, ptr %655, align 8
  %_114.0.i1865 = load ptr, ptr %656, align 8, !nonnull !12
  %_116.1.i1866 = load i64, ptr %657, align 8
  %_116.0.i1870 = load ptr, ptr %658, align 8, !nonnull !12
  %_118.1.i1874 = load i64, ptr %659, align 8
  %_118.0.i1878 = load ptr, ptr %660, align 8, !nonnull !12
  %_45.i1891 = mul i64 %width.i1836, %start1.sroa.0.0.i1858
  br label %bb32.i1843, !dbg !30413

bb32.i1843:                                       ; preds = %bb32.i1843.lr.ph, %bb31.i1906
  %iter.i1835.sroa.10.014714 = phi i64 [ %width.i1836, %bb32.i1843.lr.ph ], [ %914, %bb31.i1906 ]
  %iter.i1835.sroa.7.014713 = phi i64 [ 0, %bb32.i1843.lr.ph ], [ %_9.0.i6760, %bb31.i1906 ]
  %iter.i1835.sroa.0.0.idx14712 = phi i64 [ 0, %bb32.i1843.lr.ph ], [ %iter.i1835.sroa.0.0.add, %bb31.i1906 ]
  %iter.i1835.sroa.0.0.ptr14715 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 %iter.i1835.sroa.0.0.idx14712, !dbg !30415
  %914 = add i64 %iter.i1835.sroa.10.014714, -1, !dbg !30415
  %_7.i.i6756 = icmp eq i64 %iter.i1835.sroa.0.0.idx14712, 32, !dbg !30416
  br i1 %_7.i.i6756, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1944, label %bb3.i1845, !dbg !30420

bb3.i1845:                                        ; preds = %bb32.i1843
  %iter.i1835.sroa.0.0.add = add nuw nsw i64 %iter.i1835.sroa.0.0.idx14712, 4, !dbg !30421
  %_9.0.i6760 = add nuw nsw i64 %iter.i1835.sroa.7.014713, 1, !dbg !30423
  %exitcond17495.not = icmp eq i64 %iter.i1835.sroa.7.014713, %_112.1.i1846, !dbg !30424
  br i1 %exitcond17495.not, label %panic.i1848, label %bb5.i1849, !dbg !30424

bb5.i1849:                                        ; preds = %bb3.i1845
  %915 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i1850, i64 %iter.i1835.sroa.7.014713, !dbg !30424
  %shape.i1851 = load i32, ptr %915, align 4, !dbg !30424, !noalias !30425, !noundef !12
  %916 = getelementptr inbounds nuw i8, ptr %915, i64 4, !dbg !30424
  %shape3.i1852 = load i32, ptr %916, align 4, !dbg !30424, !noalias !30425, !noundef !12
  %window.i1853 = zext i32 %shape.i1851 to i64, !dbg !30426
  %_19.i1854 = zext i32 %shape3.i1852 to i64, !dbg !30427
  %917 = add i64 %ring_cursor.sroa.0.1.i69714733, %_19.i1854, !dbg !30428
  %_20.not.i1855 = icmp ult i64 %917, %_86.i, !dbg !30429
  %918 = select i1 %_20.not.i1855, i64 0, i64 %_86.i, !dbg !30429
  %spec.select.i1856 = sub nuw i64 %917, %918, !dbg !30429
  %_27.i1859 = mul i64 %spec.select.i1856, %width.i1836, !dbg !30430
  %_26.i1860 = add i64 %_27.i1859, %iter.i1835.sroa.7.014713, !dbg !30430
  %_30.i1862 = icmp ult i64 %_26.i1860, %_114.1.i1861, !dbg !30431
  br i1 %_30.i1862, label %bb12.i1864, label %panic5.i1863, !dbg !30431

panic.i1848:                                      ; preds = %bb3.i1845
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i1846, i64 noundef %_112.1.i1846, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8013bf8450ffb218032f1e27d334efa7) #30, !dbg !30424, !noalias !30425
  unreachable, !dbg !30424

bb12.i1864:                                       ; preds = %bb5.i1849
  %919 = getelementptr inbounds nuw float, ptr %_114.0.i1865, i64 %_26.i1860, !dbg !30431
  %920 = load float, ptr %919, align 4, !dbg !30431, !noalias !30425, !noundef !12
  %exitcond17496.not = icmp eq i64 %iter.i1835.sroa.7.014713, %_116.1.i1866, !dbg !30432
  br i1 %exitcond17496.not, label %panic6.i1868, label %bb13.i1869, !dbg !30432

panic5.i1863:                                     ; preds = %bb5.i1849
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i1860, i64 noundef %_114.1.i1861, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d297cbdfce2474defb1d7f11394e8cb6) #30, !dbg !30431, !noalias !30425
  unreachable, !dbg !30431

bb13.i1869:                                       ; preds = %bb12.i1864
  %921 = getelementptr inbounds nuw i32, ptr %_116.0.i1870, i64 %iter.i1835.sroa.7.014713, !dbg !30432
  %_32.i1871 = load i32, ptr %921, align 4, !dbg !30432, !noalias !30425, !noundef !12
  %position.i1872 = zext i32 %_32.i1871 to i64, !dbg !30432
  %922 = icmp eq i32 %_32.i1871, 0, !dbg !30433
  br i1 %922, label %bb17.i1881, label %bb15.i1873, !dbg !30433

panic6.i1868:                                     ; preds = %bb12.i1864
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i1866, i64 noundef %_116.1.i1866, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ac016620dad255f0d323022a5b7f5883) #30, !dbg !30432, !noalias !30425
  unreachable, !dbg !30432

bb15.i1873:                                       ; preds = %bb13.i1869
  %_37.i1875 = icmp ult i64 %iter.i1835.sroa.7.014713, %_118.1.i1874, !dbg !30434
  br i1 %_37.i1875, label %bb16.i1877, label %panic7.i1876, !dbg !30434

bb17.i1881:                                       ; preds = %bb35.i1942, %bb16.i1877, %bb13.i1869
  %newest.sroa.0.0.i1882 = phi float [ %920, %bb13.i1869 ], [ %_35.i1879, %bb35.i1942 ], [ %920, %bb16.i1877 ], !dbg !30435
  %exitcond17497.not = icmp eq i64 %iter.i1835.sroa.7.014713, %_118.1.i1874, !dbg !30436
  br i1 %exitcond17497.not, label %panic8.i1885, label %bb18.i1886, !dbg !30436

bb16.i1877:                                       ; preds = %bb15.i1873
  %923 = getelementptr inbounds nuw float, ptr %_118.0.i1878, i64 %iter.i1835.sroa.7.014713, !dbg !30434
  %_35.i1879 = load float, ptr %923, align 4, !dbg !30434, !noalias !30425, !noundef !12
  %_102.i1880 = fcmp olt float %_35.i1879, %920, !dbg !30437
  br i1 %_102.i1880, label %bb35.i1942, label %bb17.i1881, !dbg !30437

panic7.i1876:                                     ; preds = %bb15.i1873
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i1835.sroa.7.014713, i64 noundef %_118.1.i1874, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e5355958980a78279123102c3494b10a) #30, !dbg !30434, !noalias !30425
  unreachable, !dbg !30434

bb35.i1942:                                       ; preds = %bb16.i1877
  br label %bb17.i1881, !dbg !30439

bb18.i1886:                                       ; preds = %bb17.i1881
  %924 = getelementptr inbounds nuw float, ptr %_118.0.i1878, i64 %iter.i1835.sroa.7.014713, !dbg !30436
  store float %newest.sroa.0.0.i1882, ptr %924, align 4, !dbg !30436, !noalias !30425
  %_42.i1888 = add nuw nsw i64 %position.i1872, 1, !dbg !30440
  %complete.i1889 = icmp eq i64 %_42.i1888, %window.i1853, !dbg !30440
  br i1 %complete.i1889, label %bb22.i1911, label %bb20.i1890, !dbg !30441

panic8.i1885:                                     ; preds = %bb17.i1881
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i1874, i64 noundef %_118.1.i1874, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2d2a28b8cb03afaaebfea48ffa77c925) #30, !dbg !30436, !noalias !30425
  unreachable, !dbg !30436

bb20.i1890:                                       ; preds = %bb18.i1886
  %_44.i1892 = add i64 %iter.i1835.sroa.7.014713, %_45.i1891, !dbg !30442
  %_47.i1894 = icmp ult i64 %_44.i1892, %_114.1.i1861, !dbg !30443
  br i1 %_47.i1894, label %bb30.i1904, label %panic9.i1895, !dbg !30443

panic9.i1895:                                     ; preds = %bb20.i1890
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i1892, i64 noundef %_114.1.i1861, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_53d3a5c3ecf31ea5f14c0a7f8bf40921) #30, !dbg !30443, !noalias !30425
  unreachable, !dbg !30443

bb30.i1904:                                       ; preds = %bb20.i1890
  %925 = getelementptr inbounds nuw float, ptr %_114.0.i1865, i64 %_44.i1892, !dbg !30443
  %_43.i1898 = load float, ptr %925, align 4, !dbg !30443, !noalias !30425, !noundef !12
  %_103.i1899 = fcmp olt float %_43.i1898, %newest.sroa.0.0.i1882, !dbg !30444
  %newest.sroa.0.1.i1900 = select i1 %_103.i1899, float %_43.i1898, float %newest.sroa.0.0.i1882, !dbg !30444
  store float %newest.sroa.0.1.i1900, ptr %iter.i1835.sroa.0.0.ptr14715, align 4, !dbg !30446, !noalias !30425
  %926 = trunc i64 %_42.i1888 to i32, !dbg !30447
  br label %bb31.i1906, !dbg !30448

bb31.i1906:                                       ; preds = %bb25.i1939, %bb30.i1904
  %storemerge12443 = phi i32 [ %926, %bb30.i1904 ], [ 0, %bb25.i1939 ], !dbg !30449
  store i32 %storemerge12443, ptr %921, align 4, !dbg !30449, !noalias !30425
  %927 = icmp eq i64 %914, 0, !dbg !30413
  br i1 %927, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1944, label %bb32.i1843, !dbg !30413

bb22.i1911:                                       ; preds = %bb18.i1886
  store float %newest.sroa.0.0.i1882, ptr %iter.i1835.sroa.0.0.ptr14715, align 4, !dbg !30446, !noalias !30425
  %928 = load float, ptr %919, align 4, !dbg !30450, !noalias !30425, !noundef !12
  br label %bb41.i1924, !dbg !30451

bb41.i1924:                                       ; preds = %bb22.i1911, %bb25.i1939
  %iter2.sroa.0.0.i191614711 = phi i64 [ 0, %bb22.i1911 ], [ %_105.i1925, %bb25.i1939 ]
  %suffix.sroa.0.0.i191514710 = phi float [ %928, %bb22.i1911 ], [ %suffix.sroa.0.1.i1935, %bb25.i1939 ]
  %end.sroa.0.1.i191414709 = phi i64 [ %spec.select.i1856, %bb22.i1911 ], [ %931, %bb25.i1939 ]
  %_56.i1926 = mul i64 %end.sroa.0.1.i191414709, %width.i1836, !dbg !30454
  %_55.i1927 = add i64 %_56.i1926, %iter.i1835.sroa.7.014713, !dbg !30454
  %_59.i1929 = icmp ult i64 %_55.i1927, %_114.1.i1861, !dbg !30455
  br i1 %_59.i1929, label %bb25.i1939, label %panic13.i1930, !dbg !30455

panic13.i1930:                                    ; preds = %bb41.i1924
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i1927, i64 noundef %_114.1.i1861, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b22b5c926aed02a79660ac772e5ad40d) #30, !dbg !30455, !noalias !30425
  unreachable, !dbg !30455

bb25.i1939:                                       ; preds = %bb41.i1924
  %_105.i1925 = add nuw nsw i64 %iter2.sroa.0.0.i191614711, 1, !dbg !30456
  %929 = getelementptr inbounds nuw float, ptr %_114.0.i1865, i64 %_55.i1927, !dbg !30455
  %_54.i1933 = load float, ptr %929, align 4, !dbg !30455, !noalias !30425, !noundef !12
  %_107.i1934 = fcmp olt float %suffix.sroa.0.0.i191514710, %_54.i1933, !dbg !30459
  %suffix.sroa.0.1.i1935 = select i1 %_107.i1934, float %suffix.sroa.0.0.i191514710, float %_54.i1933, !dbg !30459
  store float %suffix.sroa.0.1.i1935, ptr %929, align 4, !dbg !30461, !noalias !30425
  %930 = icmp eq i64 %end.sroa.0.1.i191414709, 0, !dbg !30462
  %spec.store.select.i1941 = select i1 %930, i64 %_86.i, i64 %end.sroa.0.1.i191414709, !dbg !30462
  %931 = add i64 %spec.store.select.i1941, -1, !dbg !30463
  %exitcond17494.not = icmp eq i64 %_105.i1925, %window.i1853, !dbg !30464
  br i1 %exitcond17494.not, label %bb31.i1906, label %bb41.i1924, !dbg !30451

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1944: ; preds = %bb31.i1906, %bb32.i1843, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6174
  %lanes.i5475.sroa.0.0.copyload = load <8 x float>, ptr %scratch.i, align 4, !dbg !30466, !alias.scope !30471, !noalias !30475
  %932 = fmul <8 x float> %lanes.i5475.sroa.0.0.copyload, splat (float 1.638400e+04), !dbg !30479
  %933 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %932), !dbg !30484
  %934 = fmul <8 x float> %933, splat (float 0x3F10000000000000), !dbg !30489
  %935 = icmp eq i64 %width.i61.i, 0, !dbg !30494
  %_149.1.i96.i.pre = load i64, ptr %661, align 8, !dbg !30496, !alias.scope !30350, !noalias !30351
  br i1 %935, label %bb16.i95.i, label %bb39.i75.i.lr.ph, !dbg !30494

bb39.i75.i.lr.ph:                                 ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1944
  %_145.1.i78.i = load i64, ptr %62, align 8, !alias.scope !30350, !noalias !30351, !noundef !12
  %_145.0.i82.i = load ptr, ptr %61, align 8, !nonnull !12
  %_147.0.i93.i = load ptr, ptr %662, align 8, !nonnull !12
  %exitcond17500.not = icmp eq i64 %_145.1.i78.i, 0, !dbg !30497
  br i1 %exitcond17500.not, label %panic.i80.i, label %bb17.i81.i, !dbg !30497

bb37.i122.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5489
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i63.i, i64 noundef %_144.1.i62.i, i64 noundef %_144.1.i62.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_913d17a5751fc2956adecdab98dac09f) #30, !dbg !30498, !noalias !30499
  unreachable, !dbg !30498

bb16.i95.i.loopexit:                              ; preds = %bb21.i92.i.7, %bb21.i92.i.6, %bb21.i92.i.5, %bb21.i92.i.4, %bb21.i92.i.3, %bb21.i92.i.2, %bb21.i92.i.1, %bb21.i92.i
  %lanes.i5468.sroa.0.0.copyload.pre = load <8 x float>, ptr %scratch.i, align 4, !dbg !30500, !alias.scope !30505, !noalias !30509
  br label %bb16.i95.i, !dbg !30513

bb16.i95.i:                                       ; preds = %bb16.i95.i.loopexit, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1944
  %lanes.i5468.sroa.0.0.copyload = phi <8 x float> [ %lanes.i5468.sroa.0.0.copyload.pre, %bb16.i95.i.loopexit ], [ %lanes.i5475.sroa.0.0.copyload, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1944 ], !dbg !30500
  %936 = fadd <8 x float> %934, %_58.i46.i.sroa.0.0.copyload14737, !dbg !30514
  %937 = fsub <8 x float> %936, %lanes.i5468.sroa.0.0.copyload, !dbg !30519
  %_109.i97.i = icmp ugt i64 %_22.i63.i, %_149.1.i96.i.pre, !dbg !30524
  br i1 %_109.i97.i, label %bb42.i121.i, label %bb43.i98.i, !dbg !30524, !prof !639

bb43.i98.i:                                       ; preds = %bb16.i95.i
  %_112.i100.i = sub nuw i64 %_149.1.i96.i.pre, %_22.i63.i, !dbg !30527
  %_8.i6166 = icmp samesign ugt i64 %_112.i100.i, 7, !dbg !30528
  br i1 %_8.i6166, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6169, label %bb2.i6167, !dbg !30528, !prof !651

bb2.i6167:                                        ; preds = %bb43.i98.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_112.i100.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !30533, !noalias !30534
  unreachable, !dbg !30533

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6169: ; preds = %bb43.i98.i
  %_149.0.i99.i = load ptr, ptr %662, align 8, !dbg !30496, !alias.scope !30350, !noalias !30351, !nonnull !12, !noundef !12
  %_116.i101.i = getelementptr inbounds nuw float, ptr %_149.0.i99.i, i64 %_22.i63.i, !dbg !30538
  store <8 x float> %934, ptr %_116.i101.i, align 4, !dbg !30540, !alias.scope !30544, !noalias !30548
  %_68.i39.i.sroa.0.0.copyload = load <8 x float>, ptr %665, align 32, !dbg !30550
  %938 = fdiv <8 x float> %937, %_64.i43.i.sroa.0.0.copyload, !dbg !30551
  %939 = fsub <8 x float> splat (float 1.000000e+00), %938, !dbg !30556
  %940 = fsub <8 x float> %939, %_68.i39.i.sroa.0.0.copyload, !dbg !30561
  %941 = fmul <8 x float> %_9.i26.i.sroa.0.0.copyload.pre, %940, !dbg !30566
  %942 = fadd <8 x float> %_68.i39.i.sroa.0.0.copyload, %941, !dbg !30571
  %943 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %939, <8 x float> %942), !dbg !30575
  %944 = bitcast <8 x float> %943 to <8 x i32>, !dbg !30580
  %945 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %943), !dbg !30586
  %946 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %945, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !30588
  %947 = bitcast <8 x float> %946 to <8 x i32>, !dbg !30594
  %948 = xor <8 x i32> %947, splat (i32 -1), !dbg !30600
  %949 = and <8 x i32> %948, %944, !dbg !30602
  %950 = bitcast <8 x i32> %949 to <8 x float>, !dbg !30606
  store <8 x i32> %949, ptr %665, align 32, !dbg !30607
  %951 = fsub <8 x float> splat (float 1.000000e+00), %950, !dbg !30608
  %_150.1.i102.i = load i64, ptr %666, align 8, !dbg !30613, !alias.scope !30350, !noalias !30351, !noundef !12
  %_76.i103.i = mul i64 %width.i61.i, %main_cursor.sroa.0.1.i69814734, !dbg !30614
  %_120.i104.i = icmp ugt i64 %_76.i103.i, %_150.1.i102.i, !dbg !30615
  br i1 %_120.i104.i, label %bb48.i120.i, label %bb49.i105.i, !dbg !30615, !prof !639

bb42.i121.i:                                      ; preds = %bb16.i95.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i63.i, i64 noundef %_149.1.i96.i.pre, i64 noundef %_149.1.i96.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8a0dcf875eae79f6708bcf79c3cbff55) #30, !dbg !30618, !noalias !30619
  unreachable, !dbg !30618

bb49.i105.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6169
  %_123.i107.i = sub nuw i64 %_150.1.i102.i, %_76.i103.i, !dbg !30620
  %_8.i5462 = icmp samesign ugt i64 %_123.i107.i, 7, !dbg !30621
  br i1 %_8.i5462, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6159, label %bb2.i5463, !dbg !30621, !prof !651

bb2.i5463:                                        ; preds = %bb49.i105.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_123.i107.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !30626, !noalias !30627
  unreachable, !dbg !30626

bb48.i120.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6169
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i103.i, i64 noundef %_150.1.i102.i, i64 noundef %_150.1.i102.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e5e0b8406fbb9ac3f5f26ac6469c25f7) #30, !dbg !30631, !noalias !30619
  unreachable, !dbg !30631

bb17.i81.i:                                       ; preds = %bb39.i75.i.lr.ph
  %952 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i, i64 8, !dbg !30497
  %_44.i83.i = load i32, ptr %952, align 4, !dbg !30497, !noalias !30619, !noundef !12
  %_43.i84.i = zext i32 %_44.i83.i to i64, !dbg !30497
  %953 = add i64 %ring_cursor.sroa.0.1.i69714733, %_43.i84.i, !dbg !30632
  %_47.not.i85.i = icmp ult i64 %953, %_86.i, !dbg !30633
  %954 = select i1 %_47.not.i85.i, i64 0, i64 %_86.i, !dbg !30633
  %spec.select.i86.i = sub nuw i64 %953, %954, !dbg !30633
  %_51.i87.i = mul i64 %spec.select.i86.i, %width.i61.i, !dbg !30634
  %_53.i90.i = icmp ult i64 %_51.i87.i, %_149.1.i96.i.pre, !dbg !30635
  br i1 %_53.i90.i, label %bb21.i92.i, label %panic1.i91.i, !dbg !30635

panic.i80.i:                                      ; preds = %bb39.i75.i.7, %bb39.i75.i.6, %bb39.i75.i.5, %bb39.i75.i.4, %bb39.i75.i.3, %bb39.i75.i.2, %bb39.i75.i.1, %bb39.i75.i.lr.ph
  %_145.1.i78.i.lcssa.ph = phi i64 [ 7, %bb39.i75.i.7 ], [ 6, %bb39.i75.i.6 ], [ 5, %bb39.i75.i.5 ], [ 4, %bb39.i75.i.4 ], [ 3, %bb39.i75.i.3 ], [ 2, %bb39.i75.i.2 ], [ 1, %bb39.i75.i.1 ], [ 0, %bb39.i75.i.lr.ph ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i78.i.lcssa.ph, i64 noundef %_145.1.i78.i.lcssa.ph, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7ff2ed40a8d2df224a47a35441e5e2fe) #30, !dbg !30497, !noalias !30619
  unreachable, !dbg !30497

bb21.i92.i:                                       ; preds = %bb17.i81.i
  %955 = getelementptr inbounds nuw float, ptr %_147.0.i93.i, i64 %_51.i87.i, !dbg !30635
  %_49.i94.i = load float, ptr %955, align 4, !dbg !30635, !noalias !30619, !noundef !12
  store float %_49.i94.i, ptr %scratch.i, align 4, !dbg !30636, !noalias !30619
  %956 = icmp eq i64 %width.i61.i, 1, !dbg !30494
  br i1 %956, label %bb16.i95.i.loopexit, label %bb39.i75.i.1, !dbg !30494

bb39.i75.i.1:                                     ; preds = %bb21.i92.i
  %exitcond17500.1.not = icmp eq i64 %_145.1.i78.i, 1, !dbg !30497
  br i1 %exitcond17500.1.not, label %panic.i80.i, label %bb17.i81.i.1, !dbg !30497

bb17.i81.i.1:                                     ; preds = %bb39.i75.i.1
  %957 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i, i64 20, !dbg !30497
  %_44.i83.i.1 = load i32, ptr %957, align 4, !dbg !30497, !noalias !30619, !noundef !12
  %_43.i84.i.1 = zext i32 %_44.i83.i.1 to i64, !dbg !30497
  %958 = add i64 %ring_cursor.sroa.0.1.i69714733, %_43.i84.i.1, !dbg !30632
  %_47.not.i85.i.1 = icmp ult i64 %958, %_86.i, !dbg !30633
  %959 = select i1 %_47.not.i85.i.1, i64 0, i64 %_86.i, !dbg !30633
  %spec.select.i86.i.1 = sub nuw i64 %958, %959, !dbg !30633
  %_51.i87.i.1 = mul i64 %spec.select.i86.i.1, %width.i61.i, !dbg !30634
  %_50.i88.i.1 = add i64 %_51.i87.i.1, 1, !dbg !30634
  %_53.i90.i.1 = icmp ult i64 %_50.i88.i.1, %_149.1.i96.i.pre, !dbg !30635
  br i1 %_53.i90.i.1, label %bb21.i92.i.1, label %panic1.i91.i, !dbg !30635

bb21.i92.i.1:                                     ; preds = %bb17.i81.i.1
  %960 = getelementptr inbounds nuw float, ptr %_147.0.i93.i, i64 %_50.i88.i.1, !dbg !30635
  %_49.i94.i.1 = load float, ptr %960, align 4, !dbg !30635, !noalias !30619, !noundef !12
  store float %_49.i94.i.1, ptr %iter.i49.i.sroa.0.0.ptr14719.1, align 4, !dbg !30636, !noalias !30619
  %961 = icmp eq i64 %width.i61.i, 2, !dbg !30494
  br i1 %961, label %bb16.i95.i.loopexit, label %bb39.i75.i.2, !dbg !30494

bb39.i75.i.2:                                     ; preds = %bb21.i92.i.1
  %exitcond17500.2.not = icmp eq i64 %_145.1.i78.i, 2, !dbg !30497
  br i1 %exitcond17500.2.not, label %panic.i80.i, label %bb17.i81.i.2, !dbg !30497

bb17.i81.i.2:                                     ; preds = %bb39.i75.i.2
  %962 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i, i64 32, !dbg !30497
  %_44.i83.i.2 = load i32, ptr %962, align 4, !dbg !30497, !noalias !30619, !noundef !12
  %_43.i84.i.2 = zext i32 %_44.i83.i.2 to i64, !dbg !30497
  %963 = add i64 %ring_cursor.sroa.0.1.i69714733, %_43.i84.i.2, !dbg !30632
  %_47.not.i85.i.2 = icmp ult i64 %963, %_86.i, !dbg !30633
  %964 = select i1 %_47.not.i85.i.2, i64 0, i64 %_86.i, !dbg !30633
  %spec.select.i86.i.2 = sub nuw i64 %963, %964, !dbg !30633
  %_51.i87.i.2 = mul i64 %spec.select.i86.i.2, %width.i61.i, !dbg !30634
  %_50.i88.i.2 = add i64 %_51.i87.i.2, 2, !dbg !30634
  %_53.i90.i.2 = icmp ult i64 %_50.i88.i.2, %_149.1.i96.i.pre, !dbg !30635
  br i1 %_53.i90.i.2, label %bb21.i92.i.2, label %panic1.i91.i, !dbg !30635

bb21.i92.i.2:                                     ; preds = %bb17.i81.i.2
  %965 = getelementptr inbounds nuw float, ptr %_147.0.i93.i, i64 %_50.i88.i.2, !dbg !30635
  %_49.i94.i.2 = load float, ptr %965, align 4, !dbg !30635, !noalias !30619, !noundef !12
  store float %_49.i94.i.2, ptr %iter.i49.i.sroa.0.0.ptr14719.2, align 4, !dbg !30636, !noalias !30619
  %966 = icmp eq i64 %width.i61.i, 3, !dbg !30494
  br i1 %966, label %bb16.i95.i.loopexit, label %bb39.i75.i.3, !dbg !30494

bb39.i75.i.3:                                     ; preds = %bb21.i92.i.2
  %exitcond17500.3.not = icmp eq i64 %_145.1.i78.i, 3, !dbg !30497
  br i1 %exitcond17500.3.not, label %panic.i80.i, label %bb17.i81.i.3, !dbg !30497

bb17.i81.i.3:                                     ; preds = %bb39.i75.i.3
  %967 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i, i64 44, !dbg !30497
  %_44.i83.i.3 = load i32, ptr %967, align 4, !dbg !30497, !noalias !30619, !noundef !12
  %_43.i84.i.3 = zext i32 %_44.i83.i.3 to i64, !dbg !30497
  %968 = add i64 %ring_cursor.sroa.0.1.i69714733, %_43.i84.i.3, !dbg !30632
  %_47.not.i85.i.3 = icmp ult i64 %968, %_86.i, !dbg !30633
  %969 = select i1 %_47.not.i85.i.3, i64 0, i64 %_86.i, !dbg !30633
  %spec.select.i86.i.3 = sub nuw i64 %968, %969, !dbg !30633
  %_51.i87.i.3 = mul i64 %spec.select.i86.i.3, %width.i61.i, !dbg !30634
  %_50.i88.i.3 = add i64 %_51.i87.i.3, 3, !dbg !30634
  %_53.i90.i.3 = icmp ult i64 %_50.i88.i.3, %_149.1.i96.i.pre, !dbg !30635
  br i1 %_53.i90.i.3, label %bb21.i92.i.3, label %panic1.i91.i, !dbg !30635

bb21.i92.i.3:                                     ; preds = %bb17.i81.i.3
  %970 = getelementptr inbounds nuw float, ptr %_147.0.i93.i, i64 %_50.i88.i.3, !dbg !30635
  %_49.i94.i.3 = load float, ptr %970, align 4, !dbg !30635, !noalias !30619, !noundef !12
  store float %_49.i94.i.3, ptr %iter.i49.i.sroa.0.0.ptr14719.3, align 4, !dbg !30636, !noalias !30619
  %971 = icmp eq i64 %width.i61.i, 4, !dbg !30494
  br i1 %971, label %bb16.i95.i.loopexit, label %bb39.i75.i.4, !dbg !30494

bb39.i75.i.4:                                     ; preds = %bb21.i92.i.3
  %exitcond17500.4.not = icmp eq i64 %_145.1.i78.i, 4, !dbg !30497
  br i1 %exitcond17500.4.not, label %panic.i80.i, label %bb17.i81.i.4, !dbg !30497

bb17.i81.i.4:                                     ; preds = %bb39.i75.i.4
  %972 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i, i64 56, !dbg !30497
  %_44.i83.i.4 = load i32, ptr %972, align 4, !dbg !30497, !noalias !30619, !noundef !12
  %_43.i84.i.4 = zext i32 %_44.i83.i.4 to i64, !dbg !30497
  %973 = add i64 %ring_cursor.sroa.0.1.i69714733, %_43.i84.i.4, !dbg !30632
  %_47.not.i85.i.4 = icmp ult i64 %973, %_86.i, !dbg !30633
  %974 = select i1 %_47.not.i85.i.4, i64 0, i64 %_86.i, !dbg !30633
  %spec.select.i86.i.4 = sub nuw i64 %973, %974, !dbg !30633
  %_51.i87.i.4 = mul i64 %spec.select.i86.i.4, %width.i61.i, !dbg !30634
  %_50.i88.i.4 = add i64 %_51.i87.i.4, 4, !dbg !30634
  %_53.i90.i.4 = icmp ult i64 %_50.i88.i.4, %_149.1.i96.i.pre, !dbg !30635
  br i1 %_53.i90.i.4, label %bb21.i92.i.4, label %panic1.i91.i, !dbg !30635

bb21.i92.i.4:                                     ; preds = %bb17.i81.i.4
  %975 = getelementptr inbounds nuw float, ptr %_147.0.i93.i, i64 %_50.i88.i.4, !dbg !30635
  %_49.i94.i.4 = load float, ptr %975, align 4, !dbg !30635, !noalias !30619, !noundef !12
  store float %_49.i94.i.4, ptr %iter.i49.i.sroa.0.0.ptr14719.4, align 4, !dbg !30636, !noalias !30619
  %976 = icmp eq i64 %width.i61.i, 5, !dbg !30494
  br i1 %976, label %bb16.i95.i.loopexit, label %bb39.i75.i.5, !dbg !30494

bb39.i75.i.5:                                     ; preds = %bb21.i92.i.4
  %exitcond17500.5.not = icmp eq i64 %_145.1.i78.i, 5, !dbg !30497
  br i1 %exitcond17500.5.not, label %panic.i80.i, label %bb17.i81.i.5, !dbg !30497

bb17.i81.i.5:                                     ; preds = %bb39.i75.i.5
  %977 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i, i64 68, !dbg !30497
  %_44.i83.i.5 = load i32, ptr %977, align 4, !dbg !30497, !noalias !30619, !noundef !12
  %_43.i84.i.5 = zext i32 %_44.i83.i.5 to i64, !dbg !30497
  %978 = add i64 %ring_cursor.sroa.0.1.i69714733, %_43.i84.i.5, !dbg !30632
  %_47.not.i85.i.5 = icmp ult i64 %978, %_86.i, !dbg !30633
  %979 = select i1 %_47.not.i85.i.5, i64 0, i64 %_86.i, !dbg !30633
  %spec.select.i86.i.5 = sub nuw i64 %978, %979, !dbg !30633
  %_51.i87.i.5 = mul i64 %spec.select.i86.i.5, %width.i61.i, !dbg !30634
  %_50.i88.i.5 = add i64 %_51.i87.i.5, 5, !dbg !30634
  %_53.i90.i.5 = icmp ult i64 %_50.i88.i.5, %_149.1.i96.i.pre, !dbg !30635
  br i1 %_53.i90.i.5, label %bb21.i92.i.5, label %panic1.i91.i, !dbg !30635

bb21.i92.i.5:                                     ; preds = %bb17.i81.i.5
  %980 = getelementptr inbounds nuw float, ptr %_147.0.i93.i, i64 %_50.i88.i.5, !dbg !30635
  %_49.i94.i.5 = load float, ptr %980, align 4, !dbg !30635, !noalias !30619, !noundef !12
  store float %_49.i94.i.5, ptr %iter.i49.i.sroa.0.0.ptr14719.5, align 4, !dbg !30636, !noalias !30619
  %981 = icmp eq i64 %width.i61.i, 6, !dbg !30494
  br i1 %981, label %bb16.i95.i.loopexit, label %bb39.i75.i.6, !dbg !30494

bb39.i75.i.6:                                     ; preds = %bb21.i92.i.5
  %exitcond17500.6.not = icmp eq i64 %_145.1.i78.i, 6, !dbg !30497
  br i1 %exitcond17500.6.not, label %panic.i80.i, label %bb17.i81.i.6, !dbg !30497

bb17.i81.i.6:                                     ; preds = %bb39.i75.i.6
  %982 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i, i64 80, !dbg !30497
  %_44.i83.i.6 = load i32, ptr %982, align 4, !dbg !30497, !noalias !30619, !noundef !12
  %_43.i84.i.6 = zext i32 %_44.i83.i.6 to i64, !dbg !30497
  %983 = add i64 %ring_cursor.sroa.0.1.i69714733, %_43.i84.i.6, !dbg !30632
  %_47.not.i85.i.6 = icmp ult i64 %983, %_86.i, !dbg !30633
  %984 = select i1 %_47.not.i85.i.6, i64 0, i64 %_86.i, !dbg !30633
  %spec.select.i86.i.6 = sub nuw i64 %983, %984, !dbg !30633
  %_51.i87.i.6 = mul i64 %spec.select.i86.i.6, %width.i61.i, !dbg !30634
  %_50.i88.i.6 = add i64 %_51.i87.i.6, 6, !dbg !30634
  %_53.i90.i.6 = icmp ult i64 %_50.i88.i.6, %_149.1.i96.i.pre, !dbg !30635
  br i1 %_53.i90.i.6, label %bb21.i92.i.6, label %panic1.i91.i, !dbg !30635

bb21.i92.i.6:                                     ; preds = %bb17.i81.i.6
  %985 = getelementptr inbounds nuw float, ptr %_147.0.i93.i, i64 %_50.i88.i.6, !dbg !30635
  %_49.i94.i.6 = load float, ptr %985, align 4, !dbg !30635, !noalias !30619, !noundef !12
  store float %_49.i94.i.6, ptr %iter.i49.i.sroa.0.0.ptr14719.6, align 4, !dbg !30636, !noalias !30619
  %986 = icmp eq i64 %width.i61.i, 7, !dbg !30494
  br i1 %986, label %bb16.i95.i.loopexit, label %bb39.i75.i.7, !dbg !30494

bb39.i75.i.7:                                     ; preds = %bb21.i92.i.6
  %exitcond17500.7.not = icmp eq i64 %_145.1.i78.i, 7, !dbg !30497
  br i1 %exitcond17500.7.not, label %panic.i80.i, label %bb17.i81.i.7, !dbg !30497

bb17.i81.i.7:                                     ; preds = %bb39.i75.i.7
  %987 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i, i64 92, !dbg !30497
  %_44.i83.i.7 = load i32, ptr %987, align 4, !dbg !30497, !noalias !30619, !noundef !12
  %_43.i84.i.7 = zext i32 %_44.i83.i.7 to i64, !dbg !30497
  %988 = add i64 %ring_cursor.sroa.0.1.i69714733, %_43.i84.i.7, !dbg !30632
  %_47.not.i85.i.7 = icmp ult i64 %988, %_86.i, !dbg !30633
  %989 = select i1 %_47.not.i85.i.7, i64 0, i64 %_86.i, !dbg !30633
  %spec.select.i86.i.7 = sub nuw i64 %988, %989, !dbg !30633
  %_51.i87.i.7 = mul i64 %spec.select.i86.i.7, %width.i61.i, !dbg !30634
  %_50.i88.i.7 = add i64 %_51.i87.i.7, 7, !dbg !30634
  %_53.i90.i.7 = icmp ult i64 %_50.i88.i.7, %_149.1.i96.i.pre, !dbg !30635
  br i1 %_53.i90.i.7, label %bb21.i92.i.7, label %panic1.i91.i, !dbg !30635

bb21.i92.i.7:                                     ; preds = %bb17.i81.i.7
  %990 = getelementptr inbounds nuw float, ptr %_147.0.i93.i, i64 %_50.i88.i.7, !dbg !30635
  %_49.i94.i.7 = load float, ptr %990, align 4, !dbg !30635, !noalias !30619, !noundef !12
  store float %_49.i94.i.7, ptr %iter.i49.i.sroa.0.0.ptr14719.7, align 4, !dbg !30636, !noalias !30619
  br label %bb16.i95.i.loopexit, !dbg !30494

panic1.i91.i:                                     ; preds = %bb17.i81.i.7, %bb17.i81.i.6, %bb17.i81.i.5, %bb17.i81.i.4, %bb17.i81.i.3, %bb17.i81.i.2, %bb17.i81.i.1, %bb17.i81.i
  %_50.i88.i.lcssa.ph = phi i64 [ %_50.i88.i.7, %bb17.i81.i.7 ], [ %_50.i88.i.6, %bb17.i81.i.6 ], [ %_50.i88.i.5, %bb17.i81.i.5 ], [ %_50.i88.i.4, %bb17.i81.i.4 ], [ %_50.i88.i.3, %bb17.i81.i.3 ], [ %_50.i88.i.2, %bb17.i81.i.2 ], [ %_50.i88.i.1, %bb17.i81.i.1 ], [ %_51.i87.i, %bb17.i81.i ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i88.i.lcssa.ph, i64 noundef %_149.1.i96.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7d7f2b4ff3f37cf08b84adcf6762221c) #30, !dbg !30635, !noalias !30619
  unreachable, !dbg !30635

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6159: ; preds = %bb49.i105.i
  %_150.0.i106.i = load ptr, ptr %667, align 8, !dbg !30613, !alias.scope !30350, !noalias !30351, !nonnull !12, !noundef !12
  %_127.i108.i = getelementptr inbounds nuw float, ptr %_150.0.i106.i, i64 %_76.i103.i, !dbg !30637
  %lanes.i5459.sroa.0.0.copyload = load <8 x float>, ptr %_127.i108.i, align 4, !dbg !30639, !alias.scope !30643, !noalias !30647
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_127.i108.i, ptr noundef nonnull align 4 dereferenceable(32) %_141.i, i64 32, i1 false), !dbg !30649
  %991 = fmul <8 x float> %951, %lanes.i5459.sroa.0.0.copyload, !dbg !30654
  %992 = select <8 x i1> %669, <8 x float> %lanes.i5459.sroa.0.0.copyload, <8 x float> %991, !dbg !30659
  store <8 x float> %992, ptr %_141.i, align 4, !dbg !30664, !alias.scope !30669, !noalias !30673
  %_142.i = icmp samesign ugt i64 %base.i699, %right_io.1, !dbg !30677
  br i1 %_142.i, label %bb46.i, label %bb47.i, !dbg !30677, !prof !639

bb44.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5507
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i699, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_82ebe3a409d1fcceb2641bd874c3a328) #30, !dbg !30681, !noalias !30682
  unreachable, !dbg !30681

bb47.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6159
  %_145.i = sub nuw nsw i64 %right_io.1, %base.i699, !dbg !30683
  %_149.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %base.i699, !dbg !30684
  %_8.i5453 = icmp samesign ugt i64 %_145.i, 7, !dbg !30689
  br i1 %_8.i5453, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5457, label %bb2.i5454, !dbg !30689, !prof !651

bb2.i5454:                                        ; preds = %bb47.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_145.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !30694, !noalias !30695
  unreachable, !dbg !30694

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5457: ; preds = %bb47.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !30699), !dbg !30702
  %width.i.i = load i64, ptr %670, align 8, !dbg !30703, !alias.scope !30704, !noalias !30705, !noundef !12
  %993 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %905, <8 x float> %_8.i.i655.sroa.0.0.copyload.pre, i8 30), !dbg !30714
  %994 = fdiv <8 x float> %_8.i.i655.sroa.0.0.copyload.pre, %905, !dbg !30720
  %995 = bitcast <8 x float> %993 to <8 x i32>, !dbg !30725
  %996 = icmp slt <8 x i32> %995, zeroinitializer, !dbg !30729
  %997 = select <8 x i1> %996, <8 x float> %994, <8 x float> splat (float 1.000000e+00), !dbg !30729
  %_144.1.i.i = load i64, ptr %671, align 8, !dbg !30731, !alias.scope !30704, !noalias !30705, !noundef !12
  %_22.i.i704 = mul i64 %width.i.i, %ring_cursor.sroa.0.1.i69714733, !dbg !30732
  %_92.i.i = icmp ugt i64 %_22.i.i704, %_144.1.i.i, !dbg !30733
  br i1 %_92.i.i, label %bb37.i.i, label %bb38.i.i, !dbg !30733, !prof !639

bb38.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5457
  %_95.i.i = sub nuw i64 %_144.1.i.i, %_22.i.i704, !dbg !30736
  %_8.i6151 = icmp samesign ugt i64 %_95.i.i, 7, !dbg !30737
  br i1 %_8.i6151, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6154, label %bb2.i6152, !dbg !30737, !prof !651

bb2.i6152:                                        ; preds = %bb38.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_95.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !30742, !noalias !30743
  unreachable, !dbg !30742

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6154: ; preds = %bb38.i.i
  %_144.0.i.i = load ptr, ptr %672, align 8, !dbg !30731, !alias.scope !30704, !noalias !30705, !nonnull !12, !noundef !12
  %_99.i.i = getelementptr inbounds nuw float, ptr %_144.0.i.i, i64 %_22.i.i704, !dbg !30747
  store <8 x float> %997, ptr %_99.i.i, align 4, !dbg !30749, !alias.scope !30753, !noalias !30757
  tail call void @llvm.experimental.noalias.scope.decl(metadata !30759), !dbg !30762
  %width.i1726 = load i64, ptr %670, align 8, !dbg !30763, !alias.scope !30759, !noalias !30765, !noundef !12
  %998 = icmp eq i64 %width.i1726, 0, !dbg !30767
  br i1 %998, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1834, label %bb32.i1733.lr.ph, !dbg !30767

bb32.i1733.lr.ph:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6154
  %_112.1.i1736 = load i64, ptr %673, align 8, !alias.scope !30759, !noalias !30765, !noundef !12
  %_112.0.i1740 = load ptr, ptr %674, align 8, !nonnull !12
  %999 = add i64 %ring_cursor.sroa.0.1.i69714733, 1
  %_23.not.i1747 = icmp ult i64 %999, %_86.i
  %1000 = select i1 %_23.not.i1747, i64 0, i64 %_86.i
  %start1.sroa.0.0.i1748 = sub nuw i64 %999, %1000
  %_114.1.i1751 = load i64, ptr %671, align 8
  %_114.0.i1755 = load ptr, ptr %672, align 8, !nonnull !12
  %_116.1.i1756 = load i64, ptr %675, align 8
  %_116.0.i1760 = load ptr, ptr %676, align 8, !nonnull !12
  %_118.1.i1764 = load i64, ptr %677, align 8
  %_118.0.i1768 = load ptr, ptr %678, align 8, !nonnull !12
  %_45.i1781 = mul i64 %width.i1726, %start1.sroa.0.0.i1748
  br label %bb32.i1733, !dbg !30767

bb32.i1733:                                       ; preds = %bb32.i1733.lr.ph, %bb31.i1796
  %iter.i1725.sroa.10.014725 = phi i64 [ %width.i1726, %bb32.i1733.lr.ph ], [ %1001, %bb31.i1796 ]
  %iter.i1725.sroa.7.014724 = phi i64 [ 0, %bb32.i1733.lr.ph ], [ %_9.0.i6790, %bb31.i1796 ]
  %iter.i1725.sroa.0.0.idx14723 = phi i64 [ 0, %bb32.i1733.lr.ph ], [ %iter.i1725.sroa.0.0.add, %bb31.i1796 ]
  %iter.i1725.sroa.0.0.ptr14726 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 %iter.i1725.sroa.0.0.idx14723, !dbg !30769
  %1001 = add i64 %iter.i1725.sroa.10.014725, -1, !dbg !30769
  %_7.i.i6786 = icmp eq i64 %iter.i1725.sroa.0.0.idx14723, 32, !dbg !30770
  br i1 %_7.i.i6786, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1834.loopexit, label %bb3.i1735, !dbg !30774

bb3.i1735:                                        ; preds = %bb32.i1733
  %iter.i1725.sroa.0.0.add = add nuw nsw i64 %iter.i1725.sroa.0.0.idx14723, 4, !dbg !30775
  %_9.0.i6790 = add nuw nsw i64 %iter.i1725.sroa.7.014724, 1, !dbg !30777
  %exitcond17506.not = icmp eq i64 %iter.i1725.sroa.7.014724, %_112.1.i1736, !dbg !30778
  br i1 %exitcond17506.not, label %panic.i1738, label %bb5.i1739, !dbg !30778

bb5.i1739:                                        ; preds = %bb3.i1735
  %1002 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i1740, i64 %iter.i1725.sroa.7.014724, !dbg !30778
  %shape.i1741 = load i32, ptr %1002, align 4, !dbg !30778, !noalias !30779, !noundef !12
  %1003 = getelementptr inbounds nuw i8, ptr %1002, i64 4, !dbg !30778
  %shape3.i1742 = load i32, ptr %1003, align 4, !dbg !30778, !noalias !30779, !noundef !12
  %window.i1743 = zext i32 %shape.i1741 to i64, !dbg !30780
  %_19.i1744 = zext i32 %shape3.i1742 to i64, !dbg !30781
  %1004 = add i64 %ring_cursor.sroa.0.1.i69714733, %_19.i1744, !dbg !30782
  %_20.not.i1745 = icmp ult i64 %1004, %_86.i, !dbg !30783
  %1005 = select i1 %_20.not.i1745, i64 0, i64 %_86.i, !dbg !30783
  %spec.select.i1746 = sub nuw i64 %1004, %1005, !dbg !30783
  %_27.i1749 = mul i64 %spec.select.i1746, %width.i1726, !dbg !30784
  %_26.i1750 = add i64 %_27.i1749, %iter.i1725.sroa.7.014724, !dbg !30784
  %_30.i1752 = icmp ult i64 %_26.i1750, %_114.1.i1751, !dbg !30785
  br i1 %_30.i1752, label %bb12.i1754, label %panic5.i1753, !dbg !30785

panic.i1738:                                      ; preds = %bb3.i1735
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i1736, i64 noundef %_112.1.i1736, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8013bf8450ffb218032f1e27d334efa7) #30, !dbg !30778, !noalias !30779
  unreachable, !dbg !30778

bb12.i1754:                                       ; preds = %bb5.i1739
  %1006 = getelementptr inbounds nuw float, ptr %_114.0.i1755, i64 %_26.i1750, !dbg !30785
  %1007 = load float, ptr %1006, align 4, !dbg !30785, !noalias !30779, !noundef !12
  %exitcond17507.not = icmp eq i64 %iter.i1725.sroa.7.014724, %_116.1.i1756, !dbg !30786
  br i1 %exitcond17507.not, label %panic6.i1758, label %bb13.i1759, !dbg !30786

panic5.i1753:                                     ; preds = %bb5.i1739
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i1750, i64 noundef %_114.1.i1751, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d297cbdfce2474defb1d7f11394e8cb6) #30, !dbg !30785, !noalias !30779
  unreachable, !dbg !30785

bb13.i1759:                                       ; preds = %bb12.i1754
  %1008 = getelementptr inbounds nuw i32, ptr %_116.0.i1760, i64 %iter.i1725.sroa.7.014724, !dbg !30786
  %_32.i1761 = load i32, ptr %1008, align 4, !dbg !30786, !noalias !30779, !noundef !12
  %position.i1762 = zext i32 %_32.i1761 to i64, !dbg !30786
  %1009 = icmp eq i32 %_32.i1761, 0, !dbg !30787
  br i1 %1009, label %bb17.i1771, label %bb15.i1763, !dbg !30787

panic6.i1758:                                     ; preds = %bb12.i1754
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i1756, i64 noundef %_116.1.i1756, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ac016620dad255f0d323022a5b7f5883) #30, !dbg !30786, !noalias !30779
  unreachable, !dbg !30786

bb15.i1763:                                       ; preds = %bb13.i1759
  %_37.i1765 = icmp ult i64 %iter.i1725.sroa.7.014724, %_118.1.i1764, !dbg !30788
  br i1 %_37.i1765, label %bb16.i1767, label %panic7.i1766, !dbg !30788

bb17.i1771:                                       ; preds = %bb35.i1832, %bb16.i1767, %bb13.i1759
  %newest.sroa.0.0.i1772 = phi float [ %1007, %bb13.i1759 ], [ %_35.i1769, %bb35.i1832 ], [ %1007, %bb16.i1767 ], !dbg !30789
  %exitcond17508.not = icmp eq i64 %iter.i1725.sroa.7.014724, %_118.1.i1764, !dbg !30790
  br i1 %exitcond17508.not, label %panic8.i1775, label %bb18.i1776, !dbg !30790

bb16.i1767:                                       ; preds = %bb15.i1763
  %1010 = getelementptr inbounds nuw float, ptr %_118.0.i1768, i64 %iter.i1725.sroa.7.014724, !dbg !30788
  %_35.i1769 = load float, ptr %1010, align 4, !dbg !30788, !noalias !30779, !noundef !12
  %_102.i1770 = fcmp olt float %_35.i1769, %1007, !dbg !30791
  br i1 %_102.i1770, label %bb35.i1832, label %bb17.i1771, !dbg !30791

panic7.i1766:                                     ; preds = %bb15.i1763
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i1725.sroa.7.014724, i64 noundef %_118.1.i1764, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e5355958980a78279123102c3494b10a) #30, !dbg !30788, !noalias !30779
  unreachable, !dbg !30788

bb35.i1832:                                       ; preds = %bb16.i1767
  br label %bb17.i1771, !dbg !30793

bb18.i1776:                                       ; preds = %bb17.i1771
  %1011 = getelementptr inbounds nuw float, ptr %_118.0.i1768, i64 %iter.i1725.sroa.7.014724, !dbg !30790
  store float %newest.sroa.0.0.i1772, ptr %1011, align 4, !dbg !30790, !noalias !30779
  %_42.i1778 = add nuw nsw i64 %position.i1762, 1, !dbg !30794
  %complete.i1779 = icmp eq i64 %_42.i1778, %window.i1743, !dbg !30794
  br i1 %complete.i1779, label %bb22.i1801, label %bb20.i1780, !dbg !30795

panic8.i1775:                                     ; preds = %bb17.i1771
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i1764, i64 noundef %_118.1.i1764, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2d2a28b8cb03afaaebfea48ffa77c925) #30, !dbg !30790, !noalias !30779
  unreachable, !dbg !30790

bb20.i1780:                                       ; preds = %bb18.i1776
  %_44.i1782 = add i64 %iter.i1725.sroa.7.014724, %_45.i1781, !dbg !30796
  %_47.i1784 = icmp ult i64 %_44.i1782, %_114.1.i1751, !dbg !30797
  br i1 %_47.i1784, label %bb30.i1794, label %panic9.i1785, !dbg !30797

panic9.i1785:                                     ; preds = %bb20.i1780
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i1782, i64 noundef %_114.1.i1751, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_53d3a5c3ecf31ea5f14c0a7f8bf40921) #30, !dbg !30797, !noalias !30779
  unreachable, !dbg !30797

bb30.i1794:                                       ; preds = %bb20.i1780
  %1012 = getelementptr inbounds nuw float, ptr %_114.0.i1755, i64 %_44.i1782, !dbg !30797
  %_43.i1788 = load float, ptr %1012, align 4, !dbg !30797, !noalias !30779, !noundef !12
  %_103.i1789 = fcmp olt float %_43.i1788, %newest.sroa.0.0.i1772, !dbg !30798
  %newest.sroa.0.1.i1790 = select i1 %_103.i1789, float %_43.i1788, float %newest.sroa.0.0.i1772, !dbg !30798
  store float %newest.sroa.0.1.i1790, ptr %iter.i1725.sroa.0.0.ptr14726, align 4, !dbg !30800, !noalias !30779
  %1013 = trunc i64 %_42.i1778 to i32, !dbg !30801
  br label %bb31.i1796, !dbg !30802

bb31.i1796:                                       ; preds = %bb25.i1829, %bb30.i1794
  %storemerge12445 = phi i32 [ %1013, %bb30.i1794 ], [ 0, %bb25.i1829 ], !dbg !30803
  store i32 %storemerge12445, ptr %1008, align 4, !dbg !30803, !noalias !30779
  %1014 = icmp eq i64 %1001, 0, !dbg !30767
  br i1 %1014, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1834.loopexit, label %bb32.i1733, !dbg !30767

bb22.i1801:                                       ; preds = %bb18.i1776
  store float %newest.sroa.0.0.i1772, ptr %iter.i1725.sroa.0.0.ptr14726, align 4, !dbg !30800, !noalias !30779
  %1015 = load float, ptr %1006, align 4, !dbg !30804, !noalias !30779, !noundef !12
  br label %bb41.i1814, !dbg !30805

bb41.i1814:                                       ; preds = %bb22.i1801, %bb25.i1829
  %iter2.sroa.0.0.i180614722 = phi i64 [ 0, %bb22.i1801 ], [ %_105.i1815, %bb25.i1829 ]
  %suffix.sroa.0.0.i180514721 = phi float [ %1015, %bb22.i1801 ], [ %suffix.sroa.0.1.i1825, %bb25.i1829 ]
  %end.sroa.0.1.i180414720 = phi i64 [ %spec.select.i1746, %bb22.i1801 ], [ %1018, %bb25.i1829 ]
  %_56.i1816 = mul i64 %end.sroa.0.1.i180414720, %width.i1726, !dbg !30808
  %_55.i1817 = add i64 %_56.i1816, %iter.i1725.sroa.7.014724, !dbg !30808
  %_59.i1819 = icmp ult i64 %_55.i1817, %_114.1.i1751, !dbg !30809
  br i1 %_59.i1819, label %bb25.i1829, label %panic13.i1820, !dbg !30809

panic13.i1820:                                    ; preds = %bb41.i1814
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i1817, i64 noundef %_114.1.i1751, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b22b5c926aed02a79660ac772e5ad40d) #30, !dbg !30809, !noalias !30779
  unreachable, !dbg !30809

bb25.i1829:                                       ; preds = %bb41.i1814
  %_105.i1815 = add nuw nsw i64 %iter2.sroa.0.0.i180614722, 1, !dbg !30810
  %1016 = getelementptr inbounds nuw float, ptr %_114.0.i1755, i64 %_55.i1817, !dbg !30809
  %_54.i1823 = load float, ptr %1016, align 4, !dbg !30809, !noalias !30779, !noundef !12
  %_107.i1824 = fcmp olt float %suffix.sroa.0.0.i180514721, %_54.i1823, !dbg !30813
  %suffix.sroa.0.1.i1825 = select i1 %_107.i1824, float %suffix.sroa.0.0.i180514721, float %_54.i1823, !dbg !30813
  store float %suffix.sroa.0.1.i1825, ptr %1016, align 4, !dbg !30815, !noalias !30779
  %1017 = icmp eq i64 %end.sroa.0.1.i180414720, 0, !dbg !30816
  %spec.store.select.i1831 = select i1 %1017, i64 %_86.i, i64 %end.sroa.0.1.i180414720, !dbg !30816
  %1018 = add i64 %spec.store.select.i1831, -1, !dbg !30817
  %exitcond17505.not = icmp eq i64 %_105.i1815, %window.i1743, !dbg !30818
  br i1 %exitcond17505.not, label %bb31.i1796, label %bb41.i1814, !dbg !30805

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1834.loopexit: ; preds = %bb32.i1733, %bb31.i1796
  %lanes.i5443.sroa.0.0.copyload.pre = load <8 x float>, ptr %scratch.i, align 4, !dbg !30820, !alias.scope !30825, !noalias !30829
  br label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1834, !dbg !30833

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1834: ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1834.loopexit, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6154
  %lanes.i5443.sroa.0.0.copyload = phi <8 x float> [ %lanes.i5443.sroa.0.0.copyload.pre, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1834.loopexit ], [ %lanes.i5468.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6154 ], !dbg !30820
  %1019 = fmul <8 x float> %lanes.i5443.sroa.0.0.copyload, splat (float 1.638400e+04), !dbg !30834
  %1020 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %1019), !dbg !30839
  %1021 = fmul <8 x float> %1020, splat (float 0x3F10000000000000), !dbg !30844
  %1022 = icmp eq i64 %width.i.i, 0, !dbg !30849
  %_149.1.i.i.pre = load i64, ptr %679, align 8, !dbg !30851, !alias.scope !30704, !noalias !30705
  br i1 %1022, label %bb16.i.i, label %bb39.i.i.lr.ph, !dbg !30849

bb39.i.i.lr.ph:                                   ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1834
  %_145.1.i.i = load i64, ptr %673, align 8, !alias.scope !30704, !noalias !30705, !noundef !12
  %_145.0.i.i = load ptr, ptr %674, align 8, !nonnull !12
  %_147.0.i.i = load ptr, ptr %680, align 8, !nonnull !12
  %exitcond17511.not = icmp eq i64 %_145.1.i.i, 0, !dbg !30852
  br i1 %exitcond17511.not, label %panic.i.i, label %bb17.i.i, !dbg !30852

bb37.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5457
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i704, i64 noundef %_144.1.i.i, i64 noundef %_144.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_913d17a5751fc2956adecdab98dac09f) #30, !dbg !30853, !noalias !30854
  unreachable, !dbg !30853

bb16.i.i.loopexit:                                ; preds = %bb21.i.i.7, %bb21.i.i.6, %bb21.i.i.5, %bb21.i.i.4, %bb21.i.i.3, %bb21.i.i.2, %bb21.i.i.1, %bb21.i.i
  %lanes.i5436.sroa.0.0.copyload.pre = load <8 x float>, ptr %scratch.i, align 4, !dbg !30855, !alias.scope !30860, !noalias !30864
  br label %bb16.i.i, !dbg !30868

bb16.i.i:                                         ; preds = %bb16.i.i.loopexit, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1834
  %lanes.i5436.sroa.0.0.copyload = phi <8 x float> [ %lanes.i5436.sroa.0.0.copyload.pre, %bb16.i.i.loopexit ], [ %lanes.i5443.sroa.0.0.copyload, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1834 ], !dbg !30855
  %1023 = fadd <8 x float> %1021, %_58.i.i.sroa.0.0.copyload14811, !dbg !30869
  %1024 = fsub <8 x float> %1023, %lanes.i5436.sroa.0.0.copyload, !dbg !30874
  %_109.i.i = icmp ugt i64 %_22.i.i704, %_149.1.i.i.pre, !dbg !30879
  br i1 %_109.i.i, label %bb42.i.i, label %bb43.i.i, !dbg !30879, !prof !639

bb43.i.i:                                         ; preds = %bb16.i.i
  %_112.i.i = sub nuw i64 %_149.1.i.i.pre, %_22.i.i704, !dbg !30882
  %_8.i6146 = icmp samesign ugt i64 %_112.i.i, 7, !dbg !30883
  br i1 %_8.i6146, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6149, label %bb2.i6147, !dbg !30883, !prof !651

bb2.i6147:                                        ; preds = %bb43.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_112.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !30888, !noalias !30889
  unreachable, !dbg !30888

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6149: ; preds = %bb43.i.i
  %_149.0.i.i = load ptr, ptr %680, align 8, !dbg !30851, !alias.scope !30704, !noalias !30705, !nonnull !12, !noundef !12
  %_116.i.i = getelementptr inbounds nuw float, ptr %_149.0.i.i, i64 %_22.i.i704, !dbg !30893
  store <8 x float> %1021, ptr %_116.i.i, align 4, !dbg !30895, !alias.scope !30899, !noalias !30903
  %_68.i.i.sroa.0.0.copyload = load <8 x float>, ptr %683, align 32, !dbg !30905
  %1025 = fdiv <8 x float> %1024, %_64.i.i.sroa.0.0.copyload, !dbg !30906
  %1026 = fsub <8 x float> splat (float 1.000000e+00), %1025, !dbg !30911
  %1027 = fsub <8 x float> %1026, %_68.i.i.sroa.0.0.copyload, !dbg !30916
  %1028 = fmul <8 x float> %_9.i.i654.sroa.0.0.copyload.pre, %1027, !dbg !30921
  %1029 = fadd <8 x float> %_68.i.i.sroa.0.0.copyload, %1028, !dbg !30926
  %1030 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1026, <8 x float> %1029), !dbg !30930
  %1031 = bitcast <8 x float> %1030 to <8 x i32>, !dbg !30935
  %1032 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1030), !dbg !30941
  %1033 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1032, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !30943
  %1034 = bitcast <8 x float> %1033 to <8 x i32>, !dbg !30949
  %1035 = xor <8 x i32> %1034, splat (i32 -1), !dbg !30955
  %1036 = and <8 x i32> %1035, %1031, !dbg !30957
  %1037 = bitcast <8 x i32> %1036 to <8 x float>, !dbg !30961
  store <8 x i32> %1036, ptr %683, align 32, !dbg !30962
  %1038 = fsub <8 x float> splat (float 1.000000e+00), %1037, !dbg !30963
  %_150.1.i.i = load i64, ptr %684, align 8, !dbg !30968, !alias.scope !30704, !noalias !30705, !noundef !12
  %_76.i.i = mul i64 %width.i.i, %main_cursor.sroa.0.1.i69814734, !dbg !30969
  %_120.i.i = icmp ugt i64 %_76.i.i, %_150.1.i.i, !dbg !30970
  br i1 %_120.i.i, label %bb48.i.i, label %bb49.i.i, !dbg !30970, !prof !639

bb42.i.i:                                         ; preds = %bb16.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i704, i64 noundef %_149.1.i.i.pre, i64 noundef %_149.1.i.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8a0dcf875eae79f6708bcf79c3cbff55) #30, !dbg !30973, !noalias !30974
  unreachable, !dbg !30973

bb49.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6149
  %_123.i.i = sub nuw i64 %_150.1.i.i, %_76.i.i, !dbg !30975
  %_8.i5430 = icmp samesign ugt i64 %_123.i.i, 7, !dbg !30976
  br i1 %_8.i5430, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6139, label %bb2.i5431, !dbg !30976, !prof !651

bb2.i5431:                                        ; preds = %bb49.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_123.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !30981, !noalias !30982
  unreachable, !dbg !30981

bb48.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6149
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i.i, i64 noundef %_150.1.i.i, i64 noundef %_150.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e5e0b8406fbb9ac3f5f26ac6469c25f7) #30, !dbg !30986, !noalias !30974
  unreachable, !dbg !30986

bb17.i.i:                                         ; preds = %bb39.i.i.lr.ph
  %1039 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 8, !dbg !30852
  %_44.i.i706 = load i32, ptr %1039, align 4, !dbg !30852, !noalias !30974, !noundef !12
  %_43.i.i707 = zext i32 %_44.i.i706 to i64, !dbg !30852
  %1040 = add i64 %ring_cursor.sroa.0.1.i69714733, %_43.i.i707, !dbg !30987
  %_47.not.i.i = icmp ult i64 %1040, %_86.i, !dbg !30988
  %1041 = select i1 %_47.not.i.i, i64 0, i64 %_86.i, !dbg !30988
  %spec.select.i.i = sub nuw i64 %1040, %1041, !dbg !30988
  %_51.i.i = mul i64 %spec.select.i.i, %width.i.i, !dbg !30989
  %_53.i.i708 = icmp ult i64 %_51.i.i, %_149.1.i.i.pre, !dbg !30990
  br i1 %_53.i.i708, label %bb21.i.i, label %panic1.i.i, !dbg !30990

panic.i.i:                                        ; preds = %bb39.i.i.7, %bb39.i.i.6, %bb39.i.i.5, %bb39.i.i.4, %bb39.i.i.3, %bb39.i.i.2, %bb39.i.i.1, %bb39.i.i.lr.ph
  %_145.1.i.i.lcssa.ph = phi i64 [ 7, %bb39.i.i.7 ], [ 6, %bb39.i.i.6 ], [ 5, %bb39.i.i.5 ], [ 4, %bb39.i.i.4 ], [ 3, %bb39.i.i.3 ], [ 2, %bb39.i.i.2 ], [ 1, %bb39.i.i.1 ], [ 0, %bb39.i.i.lr.ph ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i.i.lcssa.ph, i64 noundef %_145.1.i.i.lcssa.ph, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7ff2ed40a8d2df224a47a35441e5e2fe) #30, !dbg !30852, !noalias !30974
  unreachable, !dbg !30852

bb21.i.i:                                         ; preds = %bb17.i.i
  %1042 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_51.i.i, !dbg !30990
  %_49.i.i709 = load float, ptr %1042, align 4, !dbg !30990, !noalias !30974, !noundef !12
  store float %_49.i.i709, ptr %scratch.i, align 4, !dbg !30991, !noalias !30974
  %1043 = icmp eq i64 %width.i.i, 1, !dbg !30849
  br i1 %1043, label %bb16.i.i.loopexit, label %bb39.i.i.1, !dbg !30849

bb39.i.i.1:                                       ; preds = %bb21.i.i
  %exitcond17511.1.not = icmp eq i64 %_145.1.i.i, 1, !dbg !30852
  br i1 %exitcond17511.1.not, label %panic.i.i, label %bb17.i.i.1, !dbg !30852

bb17.i.i.1:                                       ; preds = %bb39.i.i.1
  %1044 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 20, !dbg !30852
  %_44.i.i706.1 = load i32, ptr %1044, align 4, !dbg !30852, !noalias !30974, !noundef !12
  %_43.i.i707.1 = zext i32 %_44.i.i706.1 to i64, !dbg !30852
  %1045 = add i64 %ring_cursor.sroa.0.1.i69714733, %_43.i.i707.1, !dbg !30987
  %_47.not.i.i.1 = icmp ult i64 %1045, %_86.i, !dbg !30988
  %1046 = select i1 %_47.not.i.i.1, i64 0, i64 %_86.i, !dbg !30988
  %spec.select.i.i.1 = sub nuw i64 %1045, %1046, !dbg !30988
  %_51.i.i.1 = mul i64 %spec.select.i.i.1, %width.i.i, !dbg !30989
  %_50.i.i.1 = add i64 %_51.i.i.1, 1, !dbg !30989
  %_53.i.i708.1 = icmp ult i64 %_50.i.i.1, %_149.1.i.i.pre, !dbg !30990
  br i1 %_53.i.i708.1, label %bb21.i.i.1, label %panic1.i.i, !dbg !30990

bb21.i.i.1:                                       ; preds = %bb17.i.i.1
  %1047 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.1, !dbg !30990
  %_49.i.i709.1 = load float, ptr %1047, align 4, !dbg !30990, !noalias !30974, !noundef !12
  store float %_49.i.i709.1, ptr %iter.i.i.sroa.0.0.ptr14730.1, align 4, !dbg !30991, !noalias !30974
  %1048 = icmp eq i64 %width.i.i, 2, !dbg !30849
  br i1 %1048, label %bb16.i.i.loopexit, label %bb39.i.i.2, !dbg !30849

bb39.i.i.2:                                       ; preds = %bb21.i.i.1
  %exitcond17511.2.not = icmp eq i64 %_145.1.i.i, 2, !dbg !30852
  br i1 %exitcond17511.2.not, label %panic.i.i, label %bb17.i.i.2, !dbg !30852

bb17.i.i.2:                                       ; preds = %bb39.i.i.2
  %1049 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 32, !dbg !30852
  %_44.i.i706.2 = load i32, ptr %1049, align 4, !dbg !30852, !noalias !30974, !noundef !12
  %_43.i.i707.2 = zext i32 %_44.i.i706.2 to i64, !dbg !30852
  %1050 = add i64 %ring_cursor.sroa.0.1.i69714733, %_43.i.i707.2, !dbg !30987
  %_47.not.i.i.2 = icmp ult i64 %1050, %_86.i, !dbg !30988
  %1051 = select i1 %_47.not.i.i.2, i64 0, i64 %_86.i, !dbg !30988
  %spec.select.i.i.2 = sub nuw i64 %1050, %1051, !dbg !30988
  %_51.i.i.2 = mul i64 %spec.select.i.i.2, %width.i.i, !dbg !30989
  %_50.i.i.2 = add i64 %_51.i.i.2, 2, !dbg !30989
  %_53.i.i708.2 = icmp ult i64 %_50.i.i.2, %_149.1.i.i.pre, !dbg !30990
  br i1 %_53.i.i708.2, label %bb21.i.i.2, label %panic1.i.i, !dbg !30990

bb21.i.i.2:                                       ; preds = %bb17.i.i.2
  %1052 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.2, !dbg !30990
  %_49.i.i709.2 = load float, ptr %1052, align 4, !dbg !30990, !noalias !30974, !noundef !12
  store float %_49.i.i709.2, ptr %iter.i.i.sroa.0.0.ptr14730.2, align 4, !dbg !30991, !noalias !30974
  %1053 = icmp eq i64 %width.i.i, 3, !dbg !30849
  br i1 %1053, label %bb16.i.i.loopexit, label %bb39.i.i.3, !dbg !30849

bb39.i.i.3:                                       ; preds = %bb21.i.i.2
  %exitcond17511.3.not = icmp eq i64 %_145.1.i.i, 3, !dbg !30852
  br i1 %exitcond17511.3.not, label %panic.i.i, label %bb17.i.i.3, !dbg !30852

bb17.i.i.3:                                       ; preds = %bb39.i.i.3
  %1054 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 44, !dbg !30852
  %_44.i.i706.3 = load i32, ptr %1054, align 4, !dbg !30852, !noalias !30974, !noundef !12
  %_43.i.i707.3 = zext i32 %_44.i.i706.3 to i64, !dbg !30852
  %1055 = add i64 %ring_cursor.sroa.0.1.i69714733, %_43.i.i707.3, !dbg !30987
  %_47.not.i.i.3 = icmp ult i64 %1055, %_86.i, !dbg !30988
  %1056 = select i1 %_47.not.i.i.3, i64 0, i64 %_86.i, !dbg !30988
  %spec.select.i.i.3 = sub nuw i64 %1055, %1056, !dbg !30988
  %_51.i.i.3 = mul i64 %spec.select.i.i.3, %width.i.i, !dbg !30989
  %_50.i.i.3 = add i64 %_51.i.i.3, 3, !dbg !30989
  %_53.i.i708.3 = icmp ult i64 %_50.i.i.3, %_149.1.i.i.pre, !dbg !30990
  br i1 %_53.i.i708.3, label %bb21.i.i.3, label %panic1.i.i, !dbg !30990

bb21.i.i.3:                                       ; preds = %bb17.i.i.3
  %1057 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.3, !dbg !30990
  %_49.i.i709.3 = load float, ptr %1057, align 4, !dbg !30990, !noalias !30974, !noundef !12
  store float %_49.i.i709.3, ptr %iter.i.i.sroa.0.0.ptr14730.3, align 4, !dbg !30991, !noalias !30974
  %1058 = icmp eq i64 %width.i.i, 4, !dbg !30849
  br i1 %1058, label %bb16.i.i.loopexit, label %bb39.i.i.4, !dbg !30849

bb39.i.i.4:                                       ; preds = %bb21.i.i.3
  %exitcond17511.4.not = icmp eq i64 %_145.1.i.i, 4, !dbg !30852
  br i1 %exitcond17511.4.not, label %panic.i.i, label %bb17.i.i.4, !dbg !30852

bb17.i.i.4:                                       ; preds = %bb39.i.i.4
  %1059 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 56, !dbg !30852
  %_44.i.i706.4 = load i32, ptr %1059, align 4, !dbg !30852, !noalias !30974, !noundef !12
  %_43.i.i707.4 = zext i32 %_44.i.i706.4 to i64, !dbg !30852
  %1060 = add i64 %ring_cursor.sroa.0.1.i69714733, %_43.i.i707.4, !dbg !30987
  %_47.not.i.i.4 = icmp ult i64 %1060, %_86.i, !dbg !30988
  %1061 = select i1 %_47.not.i.i.4, i64 0, i64 %_86.i, !dbg !30988
  %spec.select.i.i.4 = sub nuw i64 %1060, %1061, !dbg !30988
  %_51.i.i.4 = mul i64 %spec.select.i.i.4, %width.i.i, !dbg !30989
  %_50.i.i.4 = add i64 %_51.i.i.4, 4, !dbg !30989
  %_53.i.i708.4 = icmp ult i64 %_50.i.i.4, %_149.1.i.i.pre, !dbg !30990
  br i1 %_53.i.i708.4, label %bb21.i.i.4, label %panic1.i.i, !dbg !30990

bb21.i.i.4:                                       ; preds = %bb17.i.i.4
  %1062 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.4, !dbg !30990
  %_49.i.i709.4 = load float, ptr %1062, align 4, !dbg !30990, !noalias !30974, !noundef !12
  store float %_49.i.i709.4, ptr %iter.i.i.sroa.0.0.ptr14730.4, align 4, !dbg !30991, !noalias !30974
  %1063 = icmp eq i64 %width.i.i, 5, !dbg !30849
  br i1 %1063, label %bb16.i.i.loopexit, label %bb39.i.i.5, !dbg !30849

bb39.i.i.5:                                       ; preds = %bb21.i.i.4
  %exitcond17511.5.not = icmp eq i64 %_145.1.i.i, 5, !dbg !30852
  br i1 %exitcond17511.5.not, label %panic.i.i, label %bb17.i.i.5, !dbg !30852

bb17.i.i.5:                                       ; preds = %bb39.i.i.5
  %1064 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 68, !dbg !30852
  %_44.i.i706.5 = load i32, ptr %1064, align 4, !dbg !30852, !noalias !30974, !noundef !12
  %_43.i.i707.5 = zext i32 %_44.i.i706.5 to i64, !dbg !30852
  %1065 = add i64 %ring_cursor.sroa.0.1.i69714733, %_43.i.i707.5, !dbg !30987
  %_47.not.i.i.5 = icmp ult i64 %1065, %_86.i, !dbg !30988
  %1066 = select i1 %_47.not.i.i.5, i64 0, i64 %_86.i, !dbg !30988
  %spec.select.i.i.5 = sub nuw i64 %1065, %1066, !dbg !30988
  %_51.i.i.5 = mul i64 %spec.select.i.i.5, %width.i.i, !dbg !30989
  %_50.i.i.5 = add i64 %_51.i.i.5, 5, !dbg !30989
  %_53.i.i708.5 = icmp ult i64 %_50.i.i.5, %_149.1.i.i.pre, !dbg !30990
  br i1 %_53.i.i708.5, label %bb21.i.i.5, label %panic1.i.i, !dbg !30990

bb21.i.i.5:                                       ; preds = %bb17.i.i.5
  %1067 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.5, !dbg !30990
  %_49.i.i709.5 = load float, ptr %1067, align 4, !dbg !30990, !noalias !30974, !noundef !12
  store float %_49.i.i709.5, ptr %iter.i.i.sroa.0.0.ptr14730.5, align 4, !dbg !30991, !noalias !30974
  %1068 = icmp eq i64 %width.i.i, 6, !dbg !30849
  br i1 %1068, label %bb16.i.i.loopexit, label %bb39.i.i.6, !dbg !30849

bb39.i.i.6:                                       ; preds = %bb21.i.i.5
  %exitcond17511.6.not = icmp eq i64 %_145.1.i.i, 6, !dbg !30852
  br i1 %exitcond17511.6.not, label %panic.i.i, label %bb17.i.i.6, !dbg !30852

bb17.i.i.6:                                       ; preds = %bb39.i.i.6
  %1069 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 80, !dbg !30852
  %_44.i.i706.6 = load i32, ptr %1069, align 4, !dbg !30852, !noalias !30974, !noundef !12
  %_43.i.i707.6 = zext i32 %_44.i.i706.6 to i64, !dbg !30852
  %1070 = add i64 %ring_cursor.sroa.0.1.i69714733, %_43.i.i707.6, !dbg !30987
  %_47.not.i.i.6 = icmp ult i64 %1070, %_86.i, !dbg !30988
  %1071 = select i1 %_47.not.i.i.6, i64 0, i64 %_86.i, !dbg !30988
  %spec.select.i.i.6 = sub nuw i64 %1070, %1071, !dbg !30988
  %_51.i.i.6 = mul i64 %spec.select.i.i.6, %width.i.i, !dbg !30989
  %_50.i.i.6 = add i64 %_51.i.i.6, 6, !dbg !30989
  %_53.i.i708.6 = icmp ult i64 %_50.i.i.6, %_149.1.i.i.pre, !dbg !30990
  br i1 %_53.i.i708.6, label %bb21.i.i.6, label %panic1.i.i, !dbg !30990

bb21.i.i.6:                                       ; preds = %bb17.i.i.6
  %1072 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.6, !dbg !30990
  %_49.i.i709.6 = load float, ptr %1072, align 4, !dbg !30990, !noalias !30974, !noundef !12
  store float %_49.i.i709.6, ptr %iter.i.i.sroa.0.0.ptr14730.6, align 4, !dbg !30991, !noalias !30974
  %1073 = icmp eq i64 %width.i.i, 7, !dbg !30849
  br i1 %1073, label %bb16.i.i.loopexit, label %bb39.i.i.7, !dbg !30849

bb39.i.i.7:                                       ; preds = %bb21.i.i.6
  %exitcond17511.7.not = icmp eq i64 %_145.1.i.i, 7, !dbg !30852
  br i1 %exitcond17511.7.not, label %panic.i.i, label %bb17.i.i.7, !dbg !30852

bb17.i.i.7:                                       ; preds = %bb39.i.i.7
  %1074 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 92, !dbg !30852
  %_44.i.i706.7 = load i32, ptr %1074, align 4, !dbg !30852, !noalias !30974, !noundef !12
  %_43.i.i707.7 = zext i32 %_44.i.i706.7 to i64, !dbg !30852
  %1075 = add i64 %ring_cursor.sroa.0.1.i69714733, %_43.i.i707.7, !dbg !30987
  %_47.not.i.i.7 = icmp ult i64 %1075, %_86.i, !dbg !30988
  %1076 = select i1 %_47.not.i.i.7, i64 0, i64 %_86.i, !dbg !30988
  %spec.select.i.i.7 = sub nuw i64 %1075, %1076, !dbg !30988
  %_51.i.i.7 = mul i64 %spec.select.i.i.7, %width.i.i, !dbg !30989
  %_50.i.i.7 = add i64 %_51.i.i.7, 7, !dbg !30989
  %_53.i.i708.7 = icmp ult i64 %_50.i.i.7, %_149.1.i.i.pre, !dbg !30990
  br i1 %_53.i.i708.7, label %bb21.i.i.7, label %panic1.i.i, !dbg !30990

bb21.i.i.7:                                       ; preds = %bb17.i.i.7
  %1077 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.7, !dbg !30990
  %_49.i.i709.7 = load float, ptr %1077, align 4, !dbg !30990, !noalias !30974, !noundef !12
  store float %_49.i.i709.7, ptr %iter.i.i.sroa.0.0.ptr14730.7, align 4, !dbg !30991, !noalias !30974
  br label %bb16.i.i.loopexit, !dbg !30849

panic1.i.i:                                       ; preds = %bb17.i.i.7, %bb17.i.i.6, %bb17.i.i.5, %bb17.i.i.4, %bb17.i.i.3, %bb17.i.i.2, %bb17.i.i.1, %bb17.i.i
  %_50.i.i.lcssa.ph = phi i64 [ %_50.i.i.7, %bb17.i.i.7 ], [ %_50.i.i.6, %bb17.i.i.6 ], [ %_50.i.i.5, %bb17.i.i.5 ], [ %_50.i.i.4, %bb17.i.i.4 ], [ %_50.i.i.3, %bb17.i.i.3 ], [ %_50.i.i.2, %bb17.i.i.2 ], [ %_50.i.i.1, %bb17.i.i.1 ], [ %_51.i.i, %bb17.i.i ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i.i.lcssa.ph, i64 noundef %_149.1.i.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7d7f2b4ff3f37cf08b84adcf6762221c) #30, !dbg !30990, !noalias !30974
  unreachable, !dbg !30990

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6139: ; preds = %bb49.i.i
  %_150.0.i.i = load ptr, ptr %685, align 8, !dbg !30968, !alias.scope !30704, !noalias !30705, !nonnull !12, !noundef !12
  %_127.i.i = getelementptr inbounds nuw float, ptr %_150.0.i.i, i64 %_76.i.i, !dbg !30992
  %lanes.i5427.sroa.0.0.copyload = load <8 x float>, ptr %_127.i.i, align 4, !dbg !30994, !alias.scope !30998, !noalias !31002
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_127.i.i, ptr noundef nonnull align 4 dereferenceable(32) %_149.i, i64 32, i1 false), !dbg !31004
  %1078 = fmul <8 x float> %1038, %lanes.i5427.sroa.0.0.copyload, !dbg !31009
  %1079 = select <8 x i1> %669, <8 x float> %lanes.i5427.sroa.0.0.copyload, <8 x float> %1078, !dbg !31014
  store <8 x float> %1079, ptr %_149.i, align 4, !dbg !31019, !alias.scope !31024, !noalias !31028
  %1080 = add i64 %main_cursor.sroa.0.1.i69814734, 1, !dbg !31032
  %_101.i = load i64, ptr %686, align 8, !dbg !31033, !alias.scope !29031, !noalias !30344, !noundef !12
  %_99.i = icmp eq i64 %1080, %_101.i, !dbg !31034
  %spec.store.select.i = select i1 %_99.i, i64 0, i64 %1080, !dbg !31034
  %1081 = add i64 %ring_cursor.sroa.0.1.i69714733, 1, !dbg !31035
  %_102.i = icmp eq i64 %1081, %_86.i, !dbg !31036
  %spec.store.select13.i = select i1 %_102.i, i64 0, i64 %1081, !dbg !31036
  %exitcond17516.not = icmp eq i64 %902, %umax17515, !dbg !31037
  br i1 %exitcond17516.not, label %bb16.i.bb13.i.loopexit_crit_edge, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5507, !dbg !29106

bb46.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6159
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i699, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_77ae5a1ade955db4654bdc779dedbc90) #30, !dbg !31040, !noalias !30682
  unreachable, !dbg !31040

_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit: ; preds = %bb13.i.loopexit
  %1082 = trunc i64 %main_cursor.sroa.0.1.i698.lcssa to i32, !dbg !31041
  %1083 = trunc i64 %ring_cursor.sroa.0.1.i697.lcssa to i32, !dbg !31042
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, !dbg !31043

_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit, %bb11.i
  %ring_cursor.sroa.0.0.i688.lcssa = phi i32 [ %_36.i682, %bb11.i ], [ %1083, %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit ], !dbg !29057
  %main_cursor.sroa.0.0.i689.lcssa = phi i32 [ %_34.i, %bb11.i ], [ %1082, %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit ], !dbg !29054
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_left.i675, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33) #31, !dbg !31044
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_right.i674, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34) #31, !dbg !31045
  store i32 %main_cursor.sroa.0.0.i689.lcssa, ptr %_35, align 4, !dbg !31041, !alias.scope !29037, !noalias !29056
  store i32 %ring_cursor.sroa.0.0.i688.lcssa, ptr %81, align 4, !dbg !31042, !alias.scope !29037, !noalias !29056
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i667), !dbg !31046, !noalias !29061
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i668), !dbg !31047, !noalias !29061
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i), !dbg !31048, !noalias !29061
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, !dbg !29030

bb7.i:                                            ; preds = %bb5.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !31049), !dbg !31052
  tail call void @llvm.experimental.noalias.scope.decl(metadata !31053), !dbg !31052
  tail call void @llvm.experimental.noalias.scope.decl(metadata !31055), !dbg !31052
  tail call void @llvm.experimental.noalias.scope.decl(metadata !31057), !dbg !31052
  tail call void @llvm.experimental.noalias.scope.decl(metadata !31059), !dbg !31052
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_left.i269, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #31, !dbg !31061
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_right.i268, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #31, !dbg !31065
  %1084 = load i8, ptr %83, align 32, !dbg !31067, !range !17, !alias.scope !31049, !noalias !31071, !noundef !12
  %1085 = load i8, ptr %84, align 1, !dbg !31074, !range !17, !alias.scope !31049, !noalias !31071, !noundef !12
  %ring.i276 = load i64, ptr %85, align 8, !dbg !31076, !alias.scope !31053, !noalias !31078, !noundef !12
  %main.i277 = load i64, ptr %86, align 8, !dbg !31079, !alias.scope !31053, !noalias !31078, !noundef !12
  %_35.i278 = load i32, ptr %_35, align 4, !dbg !31081, !alias.scope !31059, !noalias !31083, !noundef !12
  %_36.i279 = load i32, ptr %87, align 4, !dbg !31084, !alias.scope !31059, !noalias !31083, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i262), !dbg !31086, !noalias !31088
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i262, i8 0, i64 1024, i1 false), !noalias !31088
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i261), !dbg !31089, !noalias !31088
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i261, i8 0, i64 1024, i1 false), !noalias !31088
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i260), !dbg !31091, !noalias !31088
; call <true_peak_limiter::UniformHot<wide::f32x8_::f32x8>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_(ptr noalias noundef align 32 captures(none) dereferenceable(128) %uniform_left.i260, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33, i64 %ring.i276, i64 %main.i277) #31, !dbg !31093
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_right.i259), !dbg !31094, !noalias !31088
  %_32.val = load i64, ptr %85, align 8, !dbg !31096, !noundef !12
  %_32.val6479 = load i64, ptr %86, align 8, !dbg !31096, !noundef !12
; call <true_peak_limiter::UniformHot<wide::f32x8_::f32x8>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_(ptr noalias noundef align 32 captures(none) dereferenceable(128) %uniform_right.i259, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34, i64 %_32.val, i64 %_32.val6479) #31, !dbg !31096
  %1086 = add nuw nsw i64 %frames, 31, !dbg !31097
  %yield_count.sroa.0.0.i.i6815 = lshr i64 %1086, 5, !dbg !31097
  %_162.not.i29014962 = icmp eq i64 %yield_count.sroa.0.0.i.i6815, 0, !dbg !31104
  br i1 %_162.not.i29014962, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, label %bb50.i291.lr.ph, !dbg !31104

bb50.i291.lr.ph:                                  ; preds = %bb7.i
  %1087 = zext i32 %_36.i279 to i64, !dbg !31084
  %1088 = zext i32 %_35.i278 to i64, !dbg !31081
  %_32.i273 = trunc nuw i8 %1085 to i1, !dbg !31074
  %_31.i270 = trunc nuw i8 %1084 to i1, !dbg !31067
  %1089 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !31113
  %1090 = bitcast <8 x float> %1089 to <8 x i32>, !dbg !31119
  %1091 = xor <8 x i32> %1090, splat (i32 -1), !dbg !31125
  %history.i210.i.sroa.10.0.hot_left.i269.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i269, i64 32
  %history.i210.i.sroa.13.0.hot_left.i269.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i269, i64 64
  %history.i210.i.sroa.16.0.hot_left.i269.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i269, i64 96
  %history.i210.i.sroa.19.0.hot_left.i269.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i269, i64 128
  %history.i210.i.sroa.22.0.hot_left.i269.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i269, i64 160
  %history.i210.i.sroa.25.0.hot_left.i269.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i269, i64 192
  %history.i210.i.sroa.29.0.hot_left.i269.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i269, i64 224
  %history.i210.i.sroa.32.0.hot_left.i269.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i269, i64 256
  %history.i210.i.sroa.35.0.hot_left.i269.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i269, i64 288
  %history.i210.i.sroa.38.0.hot_left.i269.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i269, i64 320
  %history.i210.i.sroa.41.0.hot_left.i269.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i269, i64 352
  %1092 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %1093 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %1094 = getelementptr inbounds nuw i8, ptr %self, i64 96
  %row12.i.i.i221.i = getelementptr inbounds nuw i8, ptr %self, i64 128
  %1095 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %1096 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %1097 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %row13.i.i.i222.i = getelementptr inbounds nuw i8, ptr %self, i64 256
  %1098 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %1099 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %1100 = getelementptr inbounds nuw i8, ptr %self, i64 352
  %row14.i.i.i223.i = getelementptr inbounds nuw i8, ptr %self, i64 384
  %1101 = getelementptr inbounds nuw i8, ptr %self, i64 416
  %1102 = getelementptr inbounds nuw i8, ptr %self, i64 448
  %1103 = getelementptr inbounds nuw i8, ptr %self, i64 480
  %row15.i.i.i224.i = getelementptr inbounds nuw i8, ptr %self, i64 512
  %1104 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %1105 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %1106 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %row16.i.i.i225.i = getelementptr inbounds nuw i8, ptr %self, i64 640
  %1107 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %1108 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %1109 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %row17.i.i.i226.i = getelementptr inbounds nuw i8, ptr %self, i64 768
  %1110 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %1111 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %1112 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %row18.i.i.i227.i = getelementptr inbounds nuw i8, ptr %self, i64 896
  %1113 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %1114 = getelementptr inbounds nuw i8, ptr %self, i64 960
  %1115 = getelementptr inbounds nuw i8, ptr %self, i64 992
  %row19.i.i.i228.i = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %1116 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %1117 = getelementptr inbounds nuw i8, ptr %self, i64 1088
  %1118 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %row20.i.i.i229.i = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %1119 = getelementptr inbounds nuw i8, ptr %self, i64 1184
  %1120 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %1121 = getelementptr inbounds nuw i8, ptr %self, i64 1248
  %row21.i.i.i230.i = getelementptr inbounds nuw i8, ptr %self, i64 1280
  %1122 = getelementptr inbounds nuw i8, ptr %self, i64 1312
  %1123 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %1124 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %row22.i.i.i231.i = getelementptr inbounds nuw i8, ptr %self, i64 1408
  %1125 = getelementptr inbounds nuw i8, ptr %self, i64 1440
  %1126 = getelementptr inbounds nuw i8, ptr %self, i64 1472
  %1127 = getelementptr inbounds nuw i8, ptr %self, i64 1504
  %history.i.i220.sroa.10.0.hot_right.i268.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i268, i64 32
  %history.i.i220.sroa.13.0.hot_right.i268.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i268, i64 64
  %history.i.i220.sroa.16.0.hot_right.i268.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i268, i64 96
  %history.i.i220.sroa.19.0.hot_right.i268.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i268, i64 128
  %history.i.i220.sroa.22.0.hot_right.i268.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i268, i64 160
  %history.i.i220.sroa.25.0.hot_right.i268.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i268, i64 192
  %history.i.i220.sroa.29.0.hot_right.i268.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i268, i64 224
  %history.i.i220.sroa.32.0.hot_right.i268.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i268, i64 256
  %history.i.i220.sroa.35.0.hot_right.i268.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i268, i64 288
  %history.i.i220.sroa.38.0.hot_right.i268.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i268, i64 320
  %history.i.i220.sroa.41.0.hot_right.i268.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i268, i64 352
  %1128 = getelementptr inbounds nuw i8, ptr %uniform_left.i260, i64 80
  %_65.i256.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i260, i64 88
  %_65.i256.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i260, i64 96
  %1129 = getelementptr inbounds nuw i8, ptr %uniform_right.i259, i64 80
  %_66.i255.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i259, i64 88
  %_66.i255.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i259, i64 96
  %_109.i342 = getelementptr inbounds nuw i8, ptr %hot_left.i269, i64 384
  %_110.i343 = getelementptr inbounds nuw i8, ptr %hot_left.i269, i64 512
  %1130 = getelementptr inbounds nuw i8, ptr %hot_left.i269, i64 480
  %1131 = getelementptr inbounds nuw i8, ptr %hot_left.i269, i64 448
  %1132 = getelementptr inbounds nuw i8, ptr %hot_left.i269, i64 416
  %1133 = getelementptr inbounds nuw i8, ptr %hot_left.i269, i64 608
  %1134 = getelementptr inbounds nuw i8, ptr %hot_left.i269, i64 576
  %1135 = getelementptr inbounds nuw i8, ptr %hot_left.i269, i64 544
  %_114.i344 = getelementptr inbounds nuw i8, ptr %hot_right.i268, i64 384
  %_115.i345 = getelementptr inbounds nuw i8, ptr %hot_right.i268, i64 512
  %1136 = getelementptr inbounds nuw i8, ptr %hot_right.i268, i64 480
  %1137 = getelementptr inbounds nuw i8, ptr %hot_right.i268, i64 448
  %1138 = getelementptr inbounds nuw i8, ptr %hot_right.i268, i64 416
  %1139 = getelementptr inbounds nuw i8, ptr %hot_right.i268, i64 608
  %1140 = getelementptr inbounds nuw i8, ptr %hot_right.i268, i64 576
  %1141 = getelementptr inbounds nuw i8, ptr %hot_right.i268, i64 544
  %1142 = select i1 %_31.i270, <8 x i32> %1090, <8 x i32> %1091
  %1143 = icmp slt <8 x i32> %1142, zeroinitializer
  %1144 = getelementptr inbounds nuw i8, ptr %uniform_left.i260, i64 32
  %1145 = getelementptr inbounds nuw i8, ptr %uniform_left.i260, i64 40
  %_22.i302.i = getelementptr inbounds nuw i8, ptr %uniform_left.i260, i64 104
  %1146 = getelementptr inbounds nuw i8, ptr %uniform_left.i260, i64 48
  %1147 = getelementptr inbounds nuw i8, ptr %uniform_left.i260, i64 56
  %1148 = getelementptr inbounds nuw i8, ptr %hot_left.i269, i64 672
  %1149 = getelementptr inbounds nuw i8, ptr %hot_left.i269, i64 704
  %1150 = getelementptr inbounds nuw i8, ptr %hot_left.i269, i64 640
  %1151 = getelementptr inbounds nuw i8, ptr %uniform_left.i260, i64 72
  %1152 = getelementptr inbounds nuw i8, ptr %uniform_left.i260, i64 64
  %1153 = select i1 %_32.i273, <8 x i32> %1090, <8 x i32> %1091
  %1154 = icmp slt <8 x i32> %1153, zeroinitializer
  %1155 = getelementptr inbounds nuw i8, ptr %uniform_right.i259, i64 32
  %1156 = getelementptr inbounds nuw i8, ptr %uniform_right.i259, i64 40
  %_22.i.i378 = getelementptr inbounds nuw i8, ptr %uniform_right.i259, i64 104
  %1157 = getelementptr inbounds nuw i8, ptr %uniform_right.i259, i64 48
  %1158 = getelementptr inbounds nuw i8, ptr %uniform_right.i259, i64 56
  %1159 = getelementptr inbounds nuw i8, ptr %hot_right.i268, i64 672
  %1160 = getelementptr inbounds nuw i8, ptr %hot_right.i268, i64 704
  %1161 = getelementptr inbounds nuw i8, ptr %hot_right.i268, i64 640
  %1162 = getelementptr inbounds nuw i8, ptr %uniform_right.i259, i64 72
  %1163 = getelementptr inbounds nuw i8, ptr %uniform_right.i259, i64 64
  br label %bb50.i291, !dbg !31104

bb15.i285.loopexit.loopexit:                      ; preds = %bb32.i404
  store i32 %storemerge.i1276.lcssa2025720305, ptr %_22.i302.i, align 4
  store <8 x float> %minimum.i282.i.sroa.0.0.lcssa2027120324, ptr %uniform_left.i260, align 32
  store i32 %storemerge.i.lcssa2028720343, ptr %_22.i.i378, align 4
  store <8 x float> %minimum.i.i31.sroa.0.0.lcssa2030120362, ptr %uniform_right.i259, align 32
  br label %bb15.i285.loopexit, !dbg !31104

bb15.i285.loopexit:                               ; preds = %bb15.i285.loopexit.loopexit, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i298
  %ring_cursor.sroa.0.1.i300.lcssa = phi i64 [ %ring_cursor.sroa.0.0.i28614963, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i298 ], [ %ring_cursor.sroa.0.2.i407, %bb15.i285.loopexit.loopexit ], !dbg !31127
  %main_cursor.sroa.0.1.i301.lcssa = phi i64 [ %main_cursor.sroa.0.0.i28714964, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i298 ], [ %main_cursor.sroa.0.2.i410, %bb15.i285.loopexit.loopexit ], !dbg !31128
  %_162.not.i290 = icmp eq i64 %1165, 0, !dbg !31104
  %indvars.iv.next17520 = add nsw i64 %indvars.iv17519, -32, !dbg !31104
  br i1 %_162.not.i290, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit, label %bb50.i291, !dbg !31104

bb50.i291:                                        ; preds = %bb50.i291.lr.ph, %bb15.i285.loopexit
  %indvars.iv17519 = phi i64 [ %frames, %bb50.i291.lr.ph ], [ %indvars.iv.next17520, %bb15.i285.loopexit ]
  %iter4.sroa.0.0.i28914966 = phi i64 [ %yield_count.sroa.0.0.i.i6815, %bb50.i291.lr.ph ], [ %1165, %bb15.i285.loopexit ]
  %iter3.sroa.0.0.i28814965 = phi i64 [ 0, %bb50.i291.lr.ph ], [ %1164, %bb15.i285.loopexit ]
  %main_cursor.sroa.0.0.i28714964 = phi i64 [ %1088, %bb50.i291.lr.ph ], [ %main_cursor.sroa.0.1.i301.lcssa, %bb15.i285.loopexit ]
  %ring_cursor.sroa.0.0.i28614963 = phi i64 [ %1087, %bb50.i291.lr.ph ], [ %ring_cursor.sroa.0.1.i300.lcssa, %bb15.i285.loopexit ]
  %umin17551 = call i64 @llvm.umin.i64(i64 %indvars.iv17519, i64 32), !dbg !31129
  %umax17527 = call i64 @llvm.umax.i64(i64 %umin17551, i64 1), !dbg !31129
  %1164 = add nuw nsw i64 %iter3.sroa.0.0.i28814965, 32, !dbg !31129
  %1165 = add nsw i64 %iter4.sroa.0.0.i28914966, -1, !dbg !31133
  %_46.i293 = sub nsw i64 %frames, %iter3.sroa.0.0.i28814965, !dbg !31134
  %..i6816 = tail call noundef i64 @llvm.umin.i64(i64 %_46.i293, i64 32), !dbg !31136
  %history.i210.i.sroa.0.0.copyload = load <8 x float>, ptr %hot_left.i269, align 32, !dbg !31140
  %history.i210.i.sroa.10.sroa.0.0.copyload = load <8 x float>, ptr %history.i210.i.sroa.10.0.hot_left.i269.sroa_idx, align 32, !dbg !31140
  %history.i210.i.sroa.13.sroa.0.0.copyload = load <8 x float>, ptr %history.i210.i.sroa.13.0.hot_left.i269.sroa_idx, align 32, !dbg !31140
  %history.i210.i.sroa.16.sroa.0.0.copyload = load <8 x float>, ptr %history.i210.i.sroa.16.0.hot_left.i269.sroa_idx, align 32, !dbg !31140
  %history.i210.i.sroa.19.sroa.0.0.copyload = load <8 x float>, ptr %history.i210.i.sroa.19.0.hot_left.i269.sroa_idx, align 32, !dbg !31140
  %history.i210.i.sroa.22.sroa.0.0.copyload = load <8 x float>, ptr %history.i210.i.sroa.22.0.hot_left.i269.sroa_idx, align 32, !dbg !31140
  %history.i210.i.sroa.25.sroa.0.0.copyload = load <8 x float>, ptr %history.i210.i.sroa.25.0.hot_left.i269.sroa_idx, align 32, !dbg !31140
  %history.i210.i.sroa.29.sroa.0.0.copyload = load <8 x float>, ptr %history.i210.i.sroa.29.0.hot_left.i269.sroa_idx, align 32, !dbg !31140
  %history.i210.i.sroa.32.sroa.0.0.copyload = load <8 x float>, ptr %history.i210.i.sroa.32.0.hot_left.i269.sroa_idx, align 32, !dbg !31140
  %history.i210.i.sroa.35.sroa.0.0.copyload = load <8 x float>, ptr %history.i210.i.sroa.35.0.hot_left.i269.sroa_idx, align 32, !dbg !31140
  %history.i210.i.sroa.38.sroa.0.0.copyload = load <8 x float>, ptr %history.i210.i.sroa.38.0.hot_left.i269.sroa_idx, align 32, !dbg !31140
  %history.i210.i.sroa.41.sroa.0.0.copyload = load <8 x float>, ptr %history.i210.i.sroa.41.0.hot_left.i269.sroa_idx, align 32, !dbg !31140
  %_20.i213.i14893.not = icmp eq i64 %frames, %iter3.sroa.0.0.i28814965, !dbg !31143
  br i1 %_20.i213.i14893.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit245.i, label %bb5.i214.i.lr.ph, !dbg !31147

bb5.i214.i.lr.ph:                                 ; preds = %bb50.i291
  %_5.i4551 = load <8 x float>, ptr %self, align 32
  %_14.i.i.i175.i.sroa.0.0.copyload = load <8 x float>, ptr %1092, align 32
  %_17.i.i.i172.i.sroa.0.0.copyload = load <8 x float>, ptr %1093, align 32
  %_20.i.i.i169.i.sroa.0.0.copyload = load <8 x float>, ptr %1094, align 32
  %_25.i.i.i165.i.sroa.0.0.copyload = load <8 x float>, ptr %row12.i.i.i221.i, align 32
  %_28.i.i.i162.i.sroa.0.0.copyload = load <8 x float>, ptr %1095, align 32
  %_31.i.i.i159.i.sroa.0.0.copyload = load <8 x float>, ptr %1096, align 32
  %_34.i.i.i156.i.sroa.0.0.copyload = load <8 x float>, ptr %1097, align 32
  %_39.i.i.i152.i.sroa.0.0.copyload = load <8 x float>, ptr %row13.i.i.i222.i, align 32
  %_42.i.i.i149.i.sroa.0.0.copyload = load <8 x float>, ptr %1098, align 32
  %_45.i.i.i146.i.sroa.0.0.copyload = load <8 x float>, ptr %1099, align 32
  %_48.i.i.i143.i.sroa.0.0.copyload = load <8 x float>, ptr %1100, align 32
  %_53.i.i.i139.i.sroa.0.0.copyload = load <8 x float>, ptr %row14.i.i.i223.i, align 32
  %_56.i.i.i136.i.sroa.0.0.copyload = load <8 x float>, ptr %1101, align 32
  %_59.i.i.i133.i.sroa.0.0.copyload = load <8 x float>, ptr %1102, align 32
  %_62.i.i.i130.i.sroa.0.0.copyload = load <8 x float>, ptr %1103, align 32
  %_67.i.i.i126.i.sroa.0.0.copyload = load <8 x float>, ptr %row15.i.i.i224.i, align 32
  %_70.i.i.i123.i.sroa.0.0.copyload = load <8 x float>, ptr %1104, align 32
  %_73.i.i.i120.i.sroa.0.0.copyload = load <8 x float>, ptr %1105, align 32
  %_76.i.i.i117.i.sroa.0.0.copyload = load <8 x float>, ptr %1106, align 32
  %_81.i.i.i113.i.sroa.0.0.copyload = load <8 x float>, ptr %row16.i.i.i225.i, align 32
  %_84.i.i.i110.i.sroa.0.0.copyload = load <8 x float>, ptr %1107, align 32
  %_87.i.i.i107.i.sroa.0.0.copyload = load <8 x float>, ptr %1108, align 32
  %_90.i.i.i104.i.sroa.0.0.copyload = load <8 x float>, ptr %1109, align 32
  %_95.i.i.i100.i.sroa.0.0.copyload = load <8 x float>, ptr %row17.i.i.i226.i, align 32
  %_98.i.i.i97.i.sroa.0.0.copyload = load <8 x float>, ptr %1110, align 32
  %_101.i.i.i94.i.sroa.0.0.copyload = load <8 x float>, ptr %1111, align 32
  %_104.i.i.i91.i.sroa.0.0.copyload = load <8 x float>, ptr %1112, align 32
  %_109.i.i.i87.i.sroa.0.0.copyload = load <8 x float>, ptr %row18.i.i.i227.i, align 32
  %_112.i.i.i84.i.sroa.0.0.copyload = load <8 x float>, ptr %1113, align 32
  %_115.i.i.i81.i.sroa.0.0.copyload = load <8 x float>, ptr %1114, align 32
  %_118.i.i.i78.i.sroa.0.0.copyload = load <8 x float>, ptr %1115, align 32
  %_123.i.i.i74.i.sroa.0.0.copyload = load <8 x float>, ptr %row19.i.i.i228.i, align 32
  %_126.i.i.i71.i.sroa.0.0.copyload = load <8 x float>, ptr %1116, align 32
  %_129.i.i.i68.i.sroa.0.0.copyload = load <8 x float>, ptr %1117, align 32
  %_132.i.i.i65.i.sroa.0.0.copyload = load <8 x float>, ptr %1118, align 32
  %_137.i.i.i61.i.sroa.0.0.copyload = load <8 x float>, ptr %row20.i.i.i229.i, align 32
  %_140.i.i.i58.i.sroa.0.0.copyload = load <8 x float>, ptr %1119, align 32
  %_143.i.i.i55.i.sroa.0.0.copyload = load <8 x float>, ptr %1120, align 32
  %_146.i.i.i52.i.sroa.0.0.copyload = load <8 x float>, ptr %1121, align 32
  %_151.i.i.i48.i.sroa.0.0.copyload = load <8 x float>, ptr %row21.i.i.i230.i, align 32
  %_154.i.i.i45.i.sroa.0.0.copyload = load <8 x float>, ptr %1122, align 32
  %_157.i.i.i42.i.sroa.0.0.copyload = load <8 x float>, ptr %1123, align 32
  %_160.i.i.i39.i.sroa.0.0.copyload = load <8 x float>, ptr %1124, align 32
  %_165.i.i.i35.i.sroa.0.0.copyload = load <8 x float>, ptr %row22.i.i.i231.i, align 32
  %_168.i.i.i32.i.sroa.0.0.copyload = load <8 x float>, ptr %1125, align 32
  %_171.i.i.i29.i.sroa.0.0.copyload = load <8 x float>, ptr %1126, align 32
  %_174.i.i.i26.i.sroa.0.0.copyload = load <8 x float>, ptr %1127, align 32
  br label %bb5.i214.i, !dbg !31147

bb5.i214.i:                                       ; preds = %bb5.i214.i.lr.ph, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189
  %iter.sroa.0.0.i212.i14905 = phi i64 [ 0, %bb5.i214.i.lr.ph ], [ %1166, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189 ]
  %history.i210.i.sroa.10.sroa.0.014904 = phi <8 x float> [ %history.i210.i.sroa.10.sroa.0.0.copyload, %bb5.i214.i.lr.ph ], [ %history.i210.i.sroa.0.014894, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189 ]
  %history.i210.i.sroa.13.sroa.0.014903 = phi <8 x float> [ %history.i210.i.sroa.13.sroa.0.0.copyload, %bb5.i214.i.lr.ph ], [ %history.i210.i.sroa.10.sroa.0.014904, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189 ]
  %history.i210.i.sroa.16.sroa.0.014902 = phi <8 x float> [ %history.i210.i.sroa.16.sroa.0.0.copyload, %bb5.i214.i.lr.ph ], [ %history.i210.i.sroa.13.sroa.0.014903, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189 ]
  %history.i210.i.sroa.19.sroa.0.014901 = phi <8 x float> [ %history.i210.i.sroa.19.sroa.0.0.copyload, %bb5.i214.i.lr.ph ], [ %history.i210.i.sroa.16.sroa.0.014902, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189 ]
  %history.i210.i.sroa.22.sroa.0.014900 = phi <8 x float> [ %history.i210.i.sroa.22.sroa.0.0.copyload, %bb5.i214.i.lr.ph ], [ %history.i210.i.sroa.19.sroa.0.014901, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189 ]
  %history.i210.i.sroa.38.sroa.0.014899 = phi <8 x float> [ %history.i210.i.sroa.38.sroa.0.0.copyload, %bb5.i214.i.lr.ph ], [ %history.i210.i.sroa.35.sroa.0.014898, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189 ]
  %history.i210.i.sroa.35.sroa.0.014898 = phi <8 x float> [ %history.i210.i.sroa.35.sroa.0.0.copyload, %bb5.i214.i.lr.ph ], [ %history.i210.i.sroa.32.sroa.0.014897, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189 ]
  %history.i210.i.sroa.32.sroa.0.014897 = phi <8 x float> [ %history.i210.i.sroa.32.sroa.0.0.copyload, %bb5.i214.i.lr.ph ], [ %history.i210.i.sroa.29.sroa.0.014896, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189 ]
  %history.i210.i.sroa.29.sroa.0.014896 = phi <8 x float> [ %history.i210.i.sroa.29.sroa.0.0.copyload, %bb5.i214.i.lr.ph ], [ %history.i210.i.sroa.25.sroa.0.014895, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189 ]
  %history.i210.i.sroa.25.sroa.0.014895 = phi <8 x float> [ %history.i210.i.sroa.25.sroa.0.0.copyload, %bb5.i214.i.lr.ph ], [ %history.i210.i.sroa.22.sroa.0.014900, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189 ]
  %history.i210.i.sroa.0.014894 = phi <8 x float> [ %history.i210.i.sroa.0.0.copyload, %bb5.i214.i.lr.ph ], [ %lanes.i5509.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189 ]
  %1166 = add nuw nsw i64 %iter.sroa.0.0.i212.i14905, 1, !dbg !31148
  %_11.i215.i = add nuw nsw i64 %iter.sroa.0.0.i212.i14905, %iter3.sroa.0.0.i28814965, !dbg !31151
  %base.i216.i = shl i64 %_11.i215.i, 3, !dbg !31151
  %_24.i217.i = icmp samesign ugt i64 %base.i216.i, %left_io.1, !dbg !31152
  br i1 %_24.i217.i, label %bb7.i244.i, label %bb8.i218.i, !dbg !31152, !prof !639

bb8.i218.i:                                       ; preds = %bb5.i214.i
  %_27.i219.i = sub nuw nsw i64 %left_io.1, %base.i216.i, !dbg !31155
  %_8.i5512 = icmp samesign ugt i64 %_27.i219.i, 7, !dbg !31156
  br i1 %_8.i5512, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189, label %bb2.i5513, !dbg !31156, !prof !651

bb2.i5513:                                        ; preds = %bb8.i218.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_27.i219.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !31161, !noalias !31162
  unreachable, !dbg !31161

bb7.i244.i:                                       ; preds = %bb5.i214.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i216.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_fc26f793d85338b5649d38df0c19e7e0) #30, !dbg !31169, !noalias !31170
  unreachable, !dbg !31169

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189: ; preds = %bb8.i218.i
  %_31.i220.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %base.i216.i, !dbg !31171
  %lanes.i5509.sroa.0.0.copyload = load <8 x float>, ptr %_31.i220.i, align 4, !dbg !31173, !alias.scope !31177, !noalias !31181
  %1167 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i210.i.sroa.22.sroa.0.014900), !dbg !31183
  %1168 = fmul <8 x float> %lanes.i5509.sroa.0.0.copyload, %_5.i4551, !dbg !31190
  %1169 = fadd <8 x float> %1168, zeroinitializer, !dbg !31196
  %1170 = fmul <8 x float> %lanes.i5509.sroa.0.0.copyload, %_14.i.i.i175.i.sroa.0.0.copyload, !dbg !31201
  %1171 = fadd <8 x float> %1170, zeroinitializer, !dbg !31206
  %1172 = fmul <8 x float> %lanes.i5509.sroa.0.0.copyload, %_17.i.i.i172.i.sroa.0.0.copyload, !dbg !31211
  %1173 = fadd <8 x float> %1172, zeroinitializer, !dbg !31216
  %1174 = fmul <8 x float> %lanes.i5509.sroa.0.0.copyload, %_20.i.i.i169.i.sroa.0.0.copyload, !dbg !31221
  %1175 = fadd <8 x float> %1174, zeroinitializer, !dbg !31226
  %1176 = fmul <8 x float> %history.i210.i.sroa.0.014894, %_25.i.i.i165.i.sroa.0.0.copyload, !dbg !31231
  %1177 = fadd <8 x float> %1169, %1176, !dbg !31236
  %1178 = fmul <8 x float> %history.i210.i.sroa.0.014894, %_28.i.i.i162.i.sroa.0.0.copyload, !dbg !31241
  %1179 = fadd <8 x float> %1171, %1178, !dbg !31246
  %1180 = fmul <8 x float> %history.i210.i.sroa.0.014894, %_31.i.i.i159.i.sroa.0.0.copyload, !dbg !31251
  %1181 = fadd <8 x float> %1173, %1180, !dbg !31256
  %1182 = fmul <8 x float> %history.i210.i.sroa.0.014894, %_34.i.i.i156.i.sroa.0.0.copyload, !dbg !31261
  %1183 = fadd <8 x float> %1175, %1182, !dbg !31266
  %1184 = fmul <8 x float> %history.i210.i.sroa.10.sroa.0.014904, %_39.i.i.i152.i.sroa.0.0.copyload, !dbg !31271
  %1185 = fadd <8 x float> %1177, %1184, !dbg !31276
  %1186 = fmul <8 x float> %history.i210.i.sroa.10.sroa.0.014904, %_42.i.i.i149.i.sroa.0.0.copyload, !dbg !31281
  %1187 = fadd <8 x float> %1179, %1186, !dbg !31286
  %1188 = fmul <8 x float> %history.i210.i.sroa.10.sroa.0.014904, %_45.i.i.i146.i.sroa.0.0.copyload, !dbg !31291
  %1189 = fadd <8 x float> %1181, %1188, !dbg !31296
  %1190 = fmul <8 x float> %history.i210.i.sroa.10.sroa.0.014904, %_48.i.i.i143.i.sroa.0.0.copyload, !dbg !31301
  %1191 = fadd <8 x float> %1183, %1190, !dbg !31306
  %1192 = fmul <8 x float> %history.i210.i.sroa.13.sroa.0.014903, %_53.i.i.i139.i.sroa.0.0.copyload, !dbg !31311
  %1193 = fadd <8 x float> %1185, %1192, !dbg !31316
  %1194 = fmul <8 x float> %history.i210.i.sroa.13.sroa.0.014903, %_56.i.i.i136.i.sroa.0.0.copyload, !dbg !31321
  %1195 = fadd <8 x float> %1187, %1194, !dbg !31326
  %1196 = fmul <8 x float> %history.i210.i.sroa.13.sroa.0.014903, %_59.i.i.i133.i.sroa.0.0.copyload, !dbg !31331
  %1197 = fadd <8 x float> %1189, %1196, !dbg !31336
  %1198 = fmul <8 x float> %history.i210.i.sroa.13.sroa.0.014903, %_62.i.i.i130.i.sroa.0.0.copyload, !dbg !31341
  %1199 = fadd <8 x float> %1191, %1198, !dbg !31346
  %1200 = fmul <8 x float> %history.i210.i.sroa.16.sroa.0.014902, %_67.i.i.i126.i.sroa.0.0.copyload, !dbg !31351
  %1201 = fadd <8 x float> %1193, %1200, !dbg !31356
  %1202 = fmul <8 x float> %history.i210.i.sroa.16.sroa.0.014902, %_70.i.i.i123.i.sroa.0.0.copyload, !dbg !31361
  %1203 = fadd <8 x float> %1195, %1202, !dbg !31366
  %1204 = fmul <8 x float> %history.i210.i.sroa.16.sroa.0.014902, %_73.i.i.i120.i.sroa.0.0.copyload, !dbg !31371
  %1205 = fadd <8 x float> %1197, %1204, !dbg !31376
  %1206 = fmul <8 x float> %history.i210.i.sroa.16.sroa.0.014902, %_76.i.i.i117.i.sroa.0.0.copyload, !dbg !31381
  %1207 = fadd <8 x float> %1199, %1206, !dbg !31386
  %1208 = fmul <8 x float> %history.i210.i.sroa.19.sroa.0.014901, %_81.i.i.i113.i.sroa.0.0.copyload, !dbg !31391
  %1209 = fadd <8 x float> %1201, %1208, !dbg !31396
  %1210 = fmul <8 x float> %history.i210.i.sroa.19.sroa.0.014901, %_84.i.i.i110.i.sroa.0.0.copyload, !dbg !31401
  %1211 = fadd <8 x float> %1203, %1210, !dbg !31406
  %1212 = fmul <8 x float> %history.i210.i.sroa.19.sroa.0.014901, %_87.i.i.i107.i.sroa.0.0.copyload, !dbg !31411
  %1213 = fadd <8 x float> %1205, %1212, !dbg !31416
  %1214 = fmul <8 x float> %history.i210.i.sroa.19.sroa.0.014901, %_90.i.i.i104.i.sroa.0.0.copyload, !dbg !31421
  %1215 = fadd <8 x float> %1207, %1214, !dbg !31426
  %1216 = fmul <8 x float> %history.i210.i.sroa.22.sroa.0.014900, %_95.i.i.i100.i.sroa.0.0.copyload, !dbg !31431
  %1217 = fadd <8 x float> %1209, %1216, !dbg !31436
  %1218 = fmul <8 x float> %history.i210.i.sroa.22.sroa.0.014900, %_98.i.i.i97.i.sroa.0.0.copyload, !dbg !31441
  %1219 = fadd <8 x float> %1211, %1218, !dbg !31446
  %1220 = fmul <8 x float> %history.i210.i.sroa.22.sroa.0.014900, %_101.i.i.i94.i.sroa.0.0.copyload, !dbg !31451
  %1221 = fadd <8 x float> %1213, %1220, !dbg !31456
  %1222 = fmul <8 x float> %history.i210.i.sroa.22.sroa.0.014900, %_104.i.i.i91.i.sroa.0.0.copyload, !dbg !31461
  %1223 = fadd <8 x float> %1215, %1222, !dbg !31466
  %1224 = fmul <8 x float> %history.i210.i.sroa.25.sroa.0.014895, %_109.i.i.i87.i.sroa.0.0.copyload, !dbg !31471
  %1225 = fadd <8 x float> %1217, %1224, !dbg !31476
  %1226 = fmul <8 x float> %history.i210.i.sroa.25.sroa.0.014895, %_112.i.i.i84.i.sroa.0.0.copyload, !dbg !31481
  %1227 = fadd <8 x float> %1219, %1226, !dbg !31486
  %1228 = fmul <8 x float> %history.i210.i.sroa.25.sroa.0.014895, %_115.i.i.i81.i.sroa.0.0.copyload, !dbg !31491
  %1229 = fadd <8 x float> %1221, %1228, !dbg !31496
  %1230 = fmul <8 x float> %history.i210.i.sroa.25.sroa.0.014895, %_118.i.i.i78.i.sroa.0.0.copyload, !dbg !31501
  %1231 = fadd <8 x float> %1223, %1230, !dbg !31506
  %1232 = fmul <8 x float> %history.i210.i.sroa.29.sroa.0.014896, %_123.i.i.i74.i.sroa.0.0.copyload, !dbg !31511
  %1233 = fadd <8 x float> %1225, %1232, !dbg !31516
  %1234 = fmul <8 x float> %history.i210.i.sroa.29.sroa.0.014896, %_126.i.i.i71.i.sroa.0.0.copyload, !dbg !31521
  %1235 = fadd <8 x float> %1227, %1234, !dbg !31526
  %1236 = fmul <8 x float> %history.i210.i.sroa.29.sroa.0.014896, %_129.i.i.i68.i.sroa.0.0.copyload, !dbg !31531
  %1237 = fadd <8 x float> %1229, %1236, !dbg !31536
  %1238 = fmul <8 x float> %history.i210.i.sroa.29.sroa.0.014896, %_132.i.i.i65.i.sroa.0.0.copyload, !dbg !31541
  %1239 = fadd <8 x float> %1231, %1238, !dbg !31546
  %1240 = fmul <8 x float> %history.i210.i.sroa.32.sroa.0.014897, %_137.i.i.i61.i.sroa.0.0.copyload, !dbg !31551
  %1241 = fadd <8 x float> %1233, %1240, !dbg !31556
  %1242 = fmul <8 x float> %history.i210.i.sroa.32.sroa.0.014897, %_140.i.i.i58.i.sroa.0.0.copyload, !dbg !31561
  %1243 = fadd <8 x float> %1235, %1242, !dbg !31566
  %1244 = fmul <8 x float> %history.i210.i.sroa.32.sroa.0.014897, %_143.i.i.i55.i.sroa.0.0.copyload, !dbg !31571
  %1245 = fadd <8 x float> %1237, %1244, !dbg !31576
  %1246 = fmul <8 x float> %history.i210.i.sroa.32.sroa.0.014897, %_146.i.i.i52.i.sroa.0.0.copyload, !dbg !31581
  %1247 = fadd <8 x float> %1239, %1246, !dbg !31586
  %1248 = fmul <8 x float> %history.i210.i.sroa.35.sroa.0.014898, %_151.i.i.i48.i.sroa.0.0.copyload, !dbg !31591
  %1249 = fadd <8 x float> %1241, %1248, !dbg !31596
  %1250 = fmul <8 x float> %history.i210.i.sroa.35.sroa.0.014898, %_154.i.i.i45.i.sroa.0.0.copyload, !dbg !31601
  %1251 = fadd <8 x float> %1243, %1250, !dbg !31606
  %1252 = fmul <8 x float> %history.i210.i.sroa.35.sroa.0.014898, %_157.i.i.i42.i.sroa.0.0.copyload, !dbg !31611
  %1253 = fadd <8 x float> %1245, %1252, !dbg !31616
  %1254 = fmul <8 x float> %history.i210.i.sroa.35.sroa.0.014898, %_160.i.i.i39.i.sroa.0.0.copyload, !dbg !31621
  %1255 = fadd <8 x float> %1247, %1254, !dbg !31626
  %1256 = fmul <8 x float> %history.i210.i.sroa.38.sroa.0.014899, %_165.i.i.i35.i.sroa.0.0.copyload, !dbg !31631
  %1257 = fadd <8 x float> %1249, %1256, !dbg !31636
  %1258 = fmul <8 x float> %history.i210.i.sroa.38.sroa.0.014899, %_168.i.i.i32.i.sroa.0.0.copyload, !dbg !31641
  %1259 = fadd <8 x float> %1251, %1258, !dbg !31646
  %1260 = fmul <8 x float> %history.i210.i.sroa.38.sroa.0.014899, %_171.i.i.i29.i.sroa.0.0.copyload, !dbg !31651
  %1261 = fadd <8 x float> %1253, %1260, !dbg !31656
  %1262 = fmul <8 x float> %history.i210.i.sroa.38.sroa.0.014899, %_174.i.i.i26.i.sroa.0.0.copyload, !dbg !31661
  %1263 = fadd <8 x float> %1255, %1262, !dbg !31666
  %1264 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1257), !dbg !31671
  %1265 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1167, <8 x float> %1264), !dbg !31677
  %1266 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1259), !dbg !31671
  %1267 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1265, <8 x float> %1266), !dbg !31677
  %1268 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1261), !dbg !31671
  %1269 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1267, <8 x float> %1268), !dbg !31677
  %1270 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1263), !dbg !31671
  %1271 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1269, <8 x float> %1270), !dbg !31677
  %_39.i241.i.idx = shl i64 %iter.sroa.0.0.i212.i14905, 5, !dbg !31682
  %_39.i241.i = getelementptr inbounds nuw i8, ptr %peaks_left.i262, i64 %_39.i241.i.idx, !dbg !31682
  store <8 x float> %1271, ptr %_39.i241.i, align 4, !dbg !31687, !alias.scope !31692, !noalias !31696
  %exitcond17523.not = icmp eq i64 %1166, %umax17527, !dbg !31143
  br i1 %exitcond17523.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit245.i, label %bb5.i214.i, !dbg !31147

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit245.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189, %bb50.i291
  %history.i210.i.sroa.0.0.lcssa = phi <8 x float> [ %history.i210.i.sroa.0.0.copyload, %bb50.i291 ], [ %lanes.i5509.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189 ], !dbg !31700
  %history.i210.i.sroa.25.sroa.0.0.lcssa = phi <8 x float> [ %history.i210.i.sroa.25.sroa.0.0.copyload, %bb50.i291 ], [ %history.i210.i.sroa.22.sroa.0.014900, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189 ], !dbg !31700
  %history.i210.i.sroa.29.sroa.0.0.lcssa = phi <8 x float> [ %history.i210.i.sroa.29.sroa.0.0.copyload, %bb50.i291 ], [ %history.i210.i.sroa.25.sroa.0.014895, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189 ], !dbg !31700
  %history.i210.i.sroa.32.sroa.0.0.lcssa = phi <8 x float> [ %history.i210.i.sroa.32.sroa.0.0.copyload, %bb50.i291 ], [ %history.i210.i.sroa.29.sroa.0.014896, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189 ], !dbg !31700
  %history.i210.i.sroa.35.sroa.0.0.lcssa = phi <8 x float> [ %history.i210.i.sroa.35.sroa.0.0.copyload, %bb50.i291 ], [ %history.i210.i.sroa.32.sroa.0.014897, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189 ], !dbg !31700
  %history.i210.i.sroa.38.sroa.0.0.lcssa = phi <8 x float> [ %history.i210.i.sroa.38.sroa.0.0.copyload, %bb50.i291 ], [ %history.i210.i.sroa.35.sroa.0.014898, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189 ], !dbg !31700
  %history.i210.i.sroa.41.sroa.0.0.lcssa = phi <8 x float> [ %history.i210.i.sroa.41.sroa.0.0.copyload, %bb50.i291 ], [ %history.i210.i.sroa.38.sroa.0.014899, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189 ], !dbg !31700
  %history.i210.i.sroa.22.sroa.0.0.lcssa = phi <8 x float> [ %history.i210.i.sroa.22.sroa.0.0.copyload, %bb50.i291 ], [ %history.i210.i.sroa.19.sroa.0.014901, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189 ], !dbg !31700
  %history.i210.i.sroa.19.sroa.0.0.lcssa = phi <8 x float> [ %history.i210.i.sroa.19.sroa.0.0.copyload, %bb50.i291 ], [ %history.i210.i.sroa.16.sroa.0.014902, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189 ], !dbg !31700
  %history.i210.i.sroa.16.sroa.0.0.lcssa = phi <8 x float> [ %history.i210.i.sroa.16.sroa.0.0.copyload, %bb50.i291 ], [ %history.i210.i.sroa.13.sroa.0.014903, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189 ], !dbg !31700
  %history.i210.i.sroa.13.sroa.0.0.lcssa = phi <8 x float> [ %history.i210.i.sroa.13.sroa.0.0.copyload, %bb50.i291 ], [ %history.i210.i.sroa.10.sroa.0.014904, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189 ], !dbg !31700
  %history.i210.i.sroa.10.sroa.0.0.lcssa = phi <8 x float> [ %history.i210.i.sroa.10.sroa.0.0.copyload, %bb50.i291 ], [ %history.i210.i.sroa.0.014894, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6189 ], !dbg !31700
  store <8 x float> %history.i210.i.sroa.0.0.lcssa, ptr %hot_left.i269, align 32, !dbg !31701
  store <8 x float> %history.i210.i.sroa.10.sroa.0.0.lcssa, ptr %history.i210.i.sroa.10.0.hot_left.i269.sroa_idx, align 32, !dbg !31701
  store <8 x float> %history.i210.i.sroa.13.sroa.0.0.lcssa, ptr %history.i210.i.sroa.13.0.hot_left.i269.sroa_idx, align 32, !dbg !31701
  store <8 x float> %history.i210.i.sroa.16.sroa.0.0.lcssa, ptr %history.i210.i.sroa.16.0.hot_left.i269.sroa_idx, align 32, !dbg !31701
  store <8 x float> %history.i210.i.sroa.19.sroa.0.0.lcssa, ptr %history.i210.i.sroa.19.0.hot_left.i269.sroa_idx, align 32, !dbg !31701
  store <8 x float> %history.i210.i.sroa.22.sroa.0.0.lcssa, ptr %history.i210.i.sroa.22.0.hot_left.i269.sroa_idx, align 32, !dbg !31701
  store <8 x float> %history.i210.i.sroa.25.sroa.0.0.lcssa, ptr %history.i210.i.sroa.25.0.hot_left.i269.sroa_idx, align 32, !dbg !31701
  store <8 x float> %history.i210.i.sroa.29.sroa.0.0.lcssa, ptr %history.i210.i.sroa.29.0.hot_left.i269.sroa_idx, align 32, !dbg !31701
  store <8 x float> %history.i210.i.sroa.32.sroa.0.0.lcssa, ptr %history.i210.i.sroa.32.0.hot_left.i269.sroa_idx, align 32, !dbg !31701
  store <8 x float> %history.i210.i.sroa.35.sroa.0.0.lcssa, ptr %history.i210.i.sroa.35.0.hot_left.i269.sroa_idx, align 32, !dbg !31701
  store <8 x float> %history.i210.i.sroa.38.sroa.0.0.lcssa, ptr %history.i210.i.sroa.38.0.hot_left.i269.sroa_idx, align 32, !dbg !31701
  store <8 x float> %history.i210.i.sroa.41.sroa.0.0.lcssa, ptr %history.i210.i.sroa.41.0.hot_left.i269.sroa_idx, align 32, !dbg !31701
  %history.i.i220.sroa.0.0.copyload = load <8 x float>, ptr %hot_right.i268, align 32, !dbg !31702
  %history.i.i220.sroa.10.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i220.sroa.10.0.hot_right.i268.sroa_idx, align 32, !dbg !31702
  %history.i.i220.sroa.13.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i220.sroa.13.0.hot_right.i268.sroa_idx, align 32, !dbg !31702
  %history.i.i220.sroa.16.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i220.sroa.16.0.hot_right.i268.sroa_idx, align 32, !dbg !31702
  %history.i.i220.sroa.19.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i220.sroa.19.0.hot_right.i268.sroa_idx, align 32, !dbg !31702
  %history.i.i220.sroa.22.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i220.sroa.22.0.hot_right.i268.sroa_idx, align 32, !dbg !31702
  %history.i.i220.sroa.25.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i220.sroa.25.0.hot_right.i268.sroa_idx, align 32, !dbg !31702
  %history.i.i220.sroa.29.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i220.sroa.29.0.hot_right.i268.sroa_idx, align 32, !dbg !31702
  %history.i.i220.sroa.32.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i220.sroa.32.0.hot_right.i268.sroa_idx, align 32, !dbg !31702
  %history.i.i220.sroa.35.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i220.sroa.35.0.hot_right.i268.sroa_idx, align 32, !dbg !31702
  %history.i.i220.sroa.38.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i220.sroa.38.0.hot_right.i268.sroa_idx, align 32, !dbg !31702
  %history.i.i220.sroa.41.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i220.sroa.41.0.hot_right.i268.sroa_idx, align 32, !dbg !31702
  br i1 %_20.i213.i14893.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i298, label %bb5.i.i413.lr.ph, !dbg !31704

bb5.i.i413.lr.ph:                                 ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit245.i
  %_5.i4695 = load <8 x float>, ptr %self, align 32
  %_14.i.i.i.i185.sroa.0.0.copyload = load <8 x float>, ptr %1092, align 32
  %_17.i.i.i.i182.sroa.0.0.copyload = load <8 x float>, ptr %1093, align 32
  %_20.i.i.i.i179.sroa.0.0.copyload = load <8 x float>, ptr %1094, align 32
  %_25.i.i.i.i175.sroa.0.0.copyload = load <8 x float>, ptr %row12.i.i.i221.i, align 32
  %_28.i.i.i.i172.sroa.0.0.copyload = load <8 x float>, ptr %1095, align 32
  %_31.i.i.i.i169.sroa.0.0.copyload = load <8 x float>, ptr %1096, align 32
  %_34.i.i.i.i166.sroa.0.0.copyload = load <8 x float>, ptr %1097, align 32
  %_39.i.i.i.i162.sroa.0.0.copyload = load <8 x float>, ptr %row13.i.i.i222.i, align 32
  %_42.i.i.i.i159.sroa.0.0.copyload = load <8 x float>, ptr %1098, align 32
  %_45.i.i.i.i156.sroa.0.0.copyload = load <8 x float>, ptr %1099, align 32
  %_48.i.i.i.i153.sroa.0.0.copyload = load <8 x float>, ptr %1100, align 32
  %_53.i.i.i.i149.sroa.0.0.copyload = load <8 x float>, ptr %row14.i.i.i223.i, align 32
  %_56.i.i.i.i146.sroa.0.0.copyload = load <8 x float>, ptr %1101, align 32
  %_59.i.i.i.i143.sroa.0.0.copyload = load <8 x float>, ptr %1102, align 32
  %_62.i.i.i.i140.sroa.0.0.copyload = load <8 x float>, ptr %1103, align 32
  %_67.i.i.i.i136.sroa.0.0.copyload = load <8 x float>, ptr %row15.i.i.i224.i, align 32
  %_70.i.i.i.i133.sroa.0.0.copyload = load <8 x float>, ptr %1104, align 32
  %_73.i.i.i.i130.sroa.0.0.copyload = load <8 x float>, ptr %1105, align 32
  %_76.i.i.i.i127.sroa.0.0.copyload = load <8 x float>, ptr %1106, align 32
  %_81.i.i.i.i123.sroa.0.0.copyload = load <8 x float>, ptr %row16.i.i.i225.i, align 32
  %_84.i.i.i.i120.sroa.0.0.copyload = load <8 x float>, ptr %1107, align 32
  %_87.i.i.i.i117.sroa.0.0.copyload = load <8 x float>, ptr %1108, align 32
  %_90.i.i.i.i114.sroa.0.0.copyload = load <8 x float>, ptr %1109, align 32
  %_95.i.i.i.i110.sroa.0.0.copyload = load <8 x float>, ptr %row17.i.i.i226.i, align 32
  %_98.i.i.i.i107.sroa.0.0.copyload = load <8 x float>, ptr %1110, align 32
  %_101.i.i.i.i104.sroa.0.0.copyload = load <8 x float>, ptr %1111, align 32
  %_104.i.i.i.i101.sroa.0.0.copyload = load <8 x float>, ptr %1112, align 32
  %_109.i.i.i.i97.sroa.0.0.copyload = load <8 x float>, ptr %row18.i.i.i227.i, align 32
  %_112.i.i.i.i94.sroa.0.0.copyload = load <8 x float>, ptr %1113, align 32
  %_115.i.i.i.i91.sroa.0.0.copyload = load <8 x float>, ptr %1114, align 32
  %_118.i.i.i.i88.sroa.0.0.copyload = load <8 x float>, ptr %1115, align 32
  %_123.i.i.i.i84.sroa.0.0.copyload = load <8 x float>, ptr %row19.i.i.i228.i, align 32
  %_126.i.i.i.i81.sroa.0.0.copyload = load <8 x float>, ptr %1116, align 32
  %_129.i.i.i.i78.sroa.0.0.copyload = load <8 x float>, ptr %1117, align 32
  %_132.i.i.i.i75.sroa.0.0.copyload = load <8 x float>, ptr %1118, align 32
  %_137.i.i.i.i71.sroa.0.0.copyload = load <8 x float>, ptr %row20.i.i.i229.i, align 32
  %_140.i.i.i.i68.sroa.0.0.copyload = load <8 x float>, ptr %1119, align 32
  %_143.i.i.i.i65.sroa.0.0.copyload = load <8 x float>, ptr %1120, align 32
  %_146.i.i.i.i62.sroa.0.0.copyload = load <8 x float>, ptr %1121, align 32
  %_151.i.i.i.i58.sroa.0.0.copyload = load <8 x float>, ptr %row21.i.i.i230.i, align 32
  %_154.i.i.i.i55.sroa.0.0.copyload = load <8 x float>, ptr %1122, align 32
  %_157.i.i.i.i52.sroa.0.0.copyload = load <8 x float>, ptr %1123, align 32
  %_160.i.i.i.i49.sroa.0.0.copyload = load <8 x float>, ptr %1124, align 32
  %_165.i.i.i.i45.sroa.0.0.copyload = load <8 x float>, ptr %row22.i.i.i231.i, align 32
  %_168.i.i.i.i42.sroa.0.0.copyload = load <8 x float>, ptr %1125, align 32
  %_171.i.i.i.i39.sroa.0.0.copyload = load <8 x float>, ptr %1126, align 32
  %_174.i.i.i.i36.sroa.0.0.copyload = load <8 x float>, ptr %1127, align 32
  br label %bb5.i.i413, !dbg !31704

bb5.i.i413:                                       ; preds = %bb5.i.i413.lr.ph, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194
  %iter.sroa.0.0.i.i29614932 = phi i64 [ 0, %bb5.i.i413.lr.ph ], [ %1272, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194 ]
  %history.i.i220.sroa.10.sroa.0.014931 = phi <8 x float> [ %history.i.i220.sroa.10.sroa.0.0.copyload, %bb5.i.i413.lr.ph ], [ %history.i.i220.sroa.0.014921, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194 ]
  %history.i.i220.sroa.13.sroa.0.014930 = phi <8 x float> [ %history.i.i220.sroa.13.sroa.0.0.copyload, %bb5.i.i413.lr.ph ], [ %history.i.i220.sroa.10.sroa.0.014931, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194 ]
  %history.i.i220.sroa.16.sroa.0.014929 = phi <8 x float> [ %history.i.i220.sroa.16.sroa.0.0.copyload, %bb5.i.i413.lr.ph ], [ %history.i.i220.sroa.13.sroa.0.014930, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194 ]
  %history.i.i220.sroa.19.sroa.0.014928 = phi <8 x float> [ %history.i.i220.sroa.19.sroa.0.0.copyload, %bb5.i.i413.lr.ph ], [ %history.i.i220.sroa.16.sroa.0.014929, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194 ]
  %history.i.i220.sroa.22.sroa.0.014927 = phi <8 x float> [ %history.i.i220.sroa.22.sroa.0.0.copyload, %bb5.i.i413.lr.ph ], [ %history.i.i220.sroa.19.sroa.0.014928, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194 ]
  %history.i.i220.sroa.38.sroa.0.014926 = phi <8 x float> [ %history.i.i220.sroa.38.sroa.0.0.copyload, %bb5.i.i413.lr.ph ], [ %history.i.i220.sroa.35.sroa.0.014925, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194 ]
  %history.i.i220.sroa.35.sroa.0.014925 = phi <8 x float> [ %history.i.i220.sroa.35.sroa.0.0.copyload, %bb5.i.i413.lr.ph ], [ %history.i.i220.sroa.32.sroa.0.014924, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194 ]
  %history.i.i220.sroa.32.sroa.0.014924 = phi <8 x float> [ %history.i.i220.sroa.32.sroa.0.0.copyload, %bb5.i.i413.lr.ph ], [ %history.i.i220.sroa.29.sroa.0.014923, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194 ]
  %history.i.i220.sroa.29.sroa.0.014923 = phi <8 x float> [ %history.i.i220.sroa.29.sroa.0.0.copyload, %bb5.i.i413.lr.ph ], [ %history.i.i220.sroa.25.sroa.0.014922, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194 ]
  %history.i.i220.sroa.25.sroa.0.014922 = phi <8 x float> [ %history.i.i220.sroa.25.sroa.0.0.copyload, %bb5.i.i413.lr.ph ], [ %history.i.i220.sroa.22.sroa.0.014927, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194 ]
  %history.i.i220.sroa.0.014921 = phi <8 x float> [ %history.i.i220.sroa.0.0.copyload, %bb5.i.i413.lr.ph ], [ %lanes.i5518.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194 ]
  %1272 = add nuw nsw i64 %iter.sroa.0.0.i.i29614932, 1, !dbg !31707
  %_11.i25.i = add nuw nsw i64 %iter.sroa.0.0.i.i29614932, %iter3.sroa.0.0.i28814965, !dbg !31710
  %base.i.i414 = shl i64 %_11.i25.i, 3, !dbg !31710
  %_24.i.i415 = icmp samesign ugt i64 %base.i.i414, %right_io.1, !dbg !31711
  br i1 %_24.i.i415, label %bb7.i.i442, label %bb8.i.i416, !dbg !31711, !prof !639

bb8.i.i416:                                       ; preds = %bb5.i.i413
  %_27.i.i417 = sub nuw nsw i64 %right_io.1, %base.i.i414, !dbg !31714
  %_8.i5521 = icmp samesign ugt i64 %_27.i.i417, 7, !dbg !31715
  br i1 %_8.i5521, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194, label %bb2.i5522, !dbg !31715, !prof !651

bb2.i5522:                                        ; preds = %bb8.i.i416
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_27.i.i417, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !31720, !noalias !31721
  unreachable, !dbg !31720

bb7.i.i442:                                       ; preds = %bb5.i.i413
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i.i414, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_fc26f793d85338b5649d38df0c19e7e0) #30, !dbg !31728, !noalias !31729
  unreachable, !dbg !31728

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194: ; preds = %bb8.i.i416
  %_31.i.i418 = getelementptr inbounds nuw float, ptr %right_io.0, i64 %base.i.i414, !dbg !31730
  %lanes.i5518.sroa.0.0.copyload = load <8 x float>, ptr %_31.i.i418, align 4, !dbg !31732, !alias.scope !31736, !noalias !31740
  %1273 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i.i220.sroa.22.sroa.0.014927), !dbg !31742
  %1274 = fmul <8 x float> %lanes.i5518.sroa.0.0.copyload, %_5.i4695, !dbg !31749
  %1275 = fadd <8 x float> %1274, zeroinitializer, !dbg !31755
  %1276 = fmul <8 x float> %lanes.i5518.sroa.0.0.copyload, %_14.i.i.i.i185.sroa.0.0.copyload, !dbg !31760
  %1277 = fadd <8 x float> %1276, zeroinitializer, !dbg !31765
  %1278 = fmul <8 x float> %lanes.i5518.sroa.0.0.copyload, %_17.i.i.i.i182.sroa.0.0.copyload, !dbg !31770
  %1279 = fadd <8 x float> %1278, zeroinitializer, !dbg !31775
  %1280 = fmul <8 x float> %lanes.i5518.sroa.0.0.copyload, %_20.i.i.i.i179.sroa.0.0.copyload, !dbg !31780
  %1281 = fadd <8 x float> %1280, zeroinitializer, !dbg !31785
  %1282 = fmul <8 x float> %history.i.i220.sroa.0.014921, %_25.i.i.i.i175.sroa.0.0.copyload, !dbg !31790
  %1283 = fadd <8 x float> %1275, %1282, !dbg !31795
  %1284 = fmul <8 x float> %history.i.i220.sroa.0.014921, %_28.i.i.i.i172.sroa.0.0.copyload, !dbg !31800
  %1285 = fadd <8 x float> %1277, %1284, !dbg !31805
  %1286 = fmul <8 x float> %history.i.i220.sroa.0.014921, %_31.i.i.i.i169.sroa.0.0.copyload, !dbg !31810
  %1287 = fadd <8 x float> %1279, %1286, !dbg !31815
  %1288 = fmul <8 x float> %history.i.i220.sroa.0.014921, %_34.i.i.i.i166.sroa.0.0.copyload, !dbg !31820
  %1289 = fadd <8 x float> %1281, %1288, !dbg !31825
  %1290 = fmul <8 x float> %history.i.i220.sroa.10.sroa.0.014931, %_39.i.i.i.i162.sroa.0.0.copyload, !dbg !31830
  %1291 = fadd <8 x float> %1283, %1290, !dbg !31835
  %1292 = fmul <8 x float> %history.i.i220.sroa.10.sroa.0.014931, %_42.i.i.i.i159.sroa.0.0.copyload, !dbg !31840
  %1293 = fadd <8 x float> %1285, %1292, !dbg !31845
  %1294 = fmul <8 x float> %history.i.i220.sroa.10.sroa.0.014931, %_45.i.i.i.i156.sroa.0.0.copyload, !dbg !31850
  %1295 = fadd <8 x float> %1287, %1294, !dbg !31855
  %1296 = fmul <8 x float> %history.i.i220.sroa.10.sroa.0.014931, %_48.i.i.i.i153.sroa.0.0.copyload, !dbg !31860
  %1297 = fadd <8 x float> %1289, %1296, !dbg !31865
  %1298 = fmul <8 x float> %history.i.i220.sroa.13.sroa.0.014930, %_53.i.i.i.i149.sroa.0.0.copyload, !dbg !31870
  %1299 = fadd <8 x float> %1291, %1298, !dbg !31875
  %1300 = fmul <8 x float> %history.i.i220.sroa.13.sroa.0.014930, %_56.i.i.i.i146.sroa.0.0.copyload, !dbg !31880
  %1301 = fadd <8 x float> %1293, %1300, !dbg !31885
  %1302 = fmul <8 x float> %history.i.i220.sroa.13.sroa.0.014930, %_59.i.i.i.i143.sroa.0.0.copyload, !dbg !31890
  %1303 = fadd <8 x float> %1295, %1302, !dbg !31895
  %1304 = fmul <8 x float> %history.i.i220.sroa.13.sroa.0.014930, %_62.i.i.i.i140.sroa.0.0.copyload, !dbg !31900
  %1305 = fadd <8 x float> %1297, %1304, !dbg !31905
  %1306 = fmul <8 x float> %history.i.i220.sroa.16.sroa.0.014929, %_67.i.i.i.i136.sroa.0.0.copyload, !dbg !31910
  %1307 = fadd <8 x float> %1299, %1306, !dbg !31915
  %1308 = fmul <8 x float> %history.i.i220.sroa.16.sroa.0.014929, %_70.i.i.i.i133.sroa.0.0.copyload, !dbg !31920
  %1309 = fadd <8 x float> %1301, %1308, !dbg !31925
  %1310 = fmul <8 x float> %history.i.i220.sroa.16.sroa.0.014929, %_73.i.i.i.i130.sroa.0.0.copyload, !dbg !31930
  %1311 = fadd <8 x float> %1303, %1310, !dbg !31935
  %1312 = fmul <8 x float> %history.i.i220.sroa.16.sroa.0.014929, %_76.i.i.i.i127.sroa.0.0.copyload, !dbg !31940
  %1313 = fadd <8 x float> %1305, %1312, !dbg !31945
  %1314 = fmul <8 x float> %history.i.i220.sroa.19.sroa.0.014928, %_81.i.i.i.i123.sroa.0.0.copyload, !dbg !31950
  %1315 = fadd <8 x float> %1307, %1314, !dbg !31955
  %1316 = fmul <8 x float> %history.i.i220.sroa.19.sroa.0.014928, %_84.i.i.i.i120.sroa.0.0.copyload, !dbg !31960
  %1317 = fadd <8 x float> %1309, %1316, !dbg !31965
  %1318 = fmul <8 x float> %history.i.i220.sroa.19.sroa.0.014928, %_87.i.i.i.i117.sroa.0.0.copyload, !dbg !31970
  %1319 = fadd <8 x float> %1311, %1318, !dbg !31975
  %1320 = fmul <8 x float> %history.i.i220.sroa.19.sroa.0.014928, %_90.i.i.i.i114.sroa.0.0.copyload, !dbg !31980
  %1321 = fadd <8 x float> %1313, %1320, !dbg !31985
  %1322 = fmul <8 x float> %history.i.i220.sroa.22.sroa.0.014927, %_95.i.i.i.i110.sroa.0.0.copyload, !dbg !31990
  %1323 = fadd <8 x float> %1315, %1322, !dbg !31995
  %1324 = fmul <8 x float> %history.i.i220.sroa.22.sroa.0.014927, %_98.i.i.i.i107.sroa.0.0.copyload, !dbg !32000
  %1325 = fadd <8 x float> %1317, %1324, !dbg !32005
  %1326 = fmul <8 x float> %history.i.i220.sroa.22.sroa.0.014927, %_101.i.i.i.i104.sroa.0.0.copyload, !dbg !32010
  %1327 = fadd <8 x float> %1319, %1326, !dbg !32015
  %1328 = fmul <8 x float> %history.i.i220.sroa.22.sroa.0.014927, %_104.i.i.i.i101.sroa.0.0.copyload, !dbg !32020
  %1329 = fadd <8 x float> %1321, %1328, !dbg !32025
  %1330 = fmul <8 x float> %history.i.i220.sroa.25.sroa.0.014922, %_109.i.i.i.i97.sroa.0.0.copyload, !dbg !32030
  %1331 = fadd <8 x float> %1323, %1330, !dbg !32035
  %1332 = fmul <8 x float> %history.i.i220.sroa.25.sroa.0.014922, %_112.i.i.i.i94.sroa.0.0.copyload, !dbg !32040
  %1333 = fadd <8 x float> %1325, %1332, !dbg !32045
  %1334 = fmul <8 x float> %history.i.i220.sroa.25.sroa.0.014922, %_115.i.i.i.i91.sroa.0.0.copyload, !dbg !32050
  %1335 = fadd <8 x float> %1327, %1334, !dbg !32055
  %1336 = fmul <8 x float> %history.i.i220.sroa.25.sroa.0.014922, %_118.i.i.i.i88.sroa.0.0.copyload, !dbg !32060
  %1337 = fadd <8 x float> %1329, %1336, !dbg !32065
  %1338 = fmul <8 x float> %history.i.i220.sroa.29.sroa.0.014923, %_123.i.i.i.i84.sroa.0.0.copyload, !dbg !32070
  %1339 = fadd <8 x float> %1331, %1338, !dbg !32075
  %1340 = fmul <8 x float> %history.i.i220.sroa.29.sroa.0.014923, %_126.i.i.i.i81.sroa.0.0.copyload, !dbg !32080
  %1341 = fadd <8 x float> %1333, %1340, !dbg !32085
  %1342 = fmul <8 x float> %history.i.i220.sroa.29.sroa.0.014923, %_129.i.i.i.i78.sroa.0.0.copyload, !dbg !32090
  %1343 = fadd <8 x float> %1335, %1342, !dbg !32095
  %1344 = fmul <8 x float> %history.i.i220.sroa.29.sroa.0.014923, %_132.i.i.i.i75.sroa.0.0.copyload, !dbg !32100
  %1345 = fadd <8 x float> %1337, %1344, !dbg !32105
  %1346 = fmul <8 x float> %history.i.i220.sroa.32.sroa.0.014924, %_137.i.i.i.i71.sroa.0.0.copyload, !dbg !32110
  %1347 = fadd <8 x float> %1339, %1346, !dbg !32115
  %1348 = fmul <8 x float> %history.i.i220.sroa.32.sroa.0.014924, %_140.i.i.i.i68.sroa.0.0.copyload, !dbg !32120
  %1349 = fadd <8 x float> %1341, %1348, !dbg !32125
  %1350 = fmul <8 x float> %history.i.i220.sroa.32.sroa.0.014924, %_143.i.i.i.i65.sroa.0.0.copyload, !dbg !32130
  %1351 = fadd <8 x float> %1343, %1350, !dbg !32135
  %1352 = fmul <8 x float> %history.i.i220.sroa.32.sroa.0.014924, %_146.i.i.i.i62.sroa.0.0.copyload, !dbg !32140
  %1353 = fadd <8 x float> %1345, %1352, !dbg !32145
  %1354 = fmul <8 x float> %history.i.i220.sroa.35.sroa.0.014925, %_151.i.i.i.i58.sroa.0.0.copyload, !dbg !32150
  %1355 = fadd <8 x float> %1347, %1354, !dbg !32155
  %1356 = fmul <8 x float> %history.i.i220.sroa.35.sroa.0.014925, %_154.i.i.i.i55.sroa.0.0.copyload, !dbg !32160
  %1357 = fadd <8 x float> %1349, %1356, !dbg !32165
  %1358 = fmul <8 x float> %history.i.i220.sroa.35.sroa.0.014925, %_157.i.i.i.i52.sroa.0.0.copyload, !dbg !32170
  %1359 = fadd <8 x float> %1351, %1358, !dbg !32175
  %1360 = fmul <8 x float> %history.i.i220.sroa.35.sroa.0.014925, %_160.i.i.i.i49.sroa.0.0.copyload, !dbg !32180
  %1361 = fadd <8 x float> %1353, %1360, !dbg !32185
  %1362 = fmul <8 x float> %history.i.i220.sroa.38.sroa.0.014926, %_165.i.i.i.i45.sroa.0.0.copyload, !dbg !32190
  %1363 = fadd <8 x float> %1355, %1362, !dbg !32195
  %1364 = fmul <8 x float> %history.i.i220.sroa.38.sroa.0.014926, %_168.i.i.i.i42.sroa.0.0.copyload, !dbg !32200
  %1365 = fadd <8 x float> %1357, %1364, !dbg !32205
  %1366 = fmul <8 x float> %history.i.i220.sroa.38.sroa.0.014926, %_171.i.i.i.i39.sroa.0.0.copyload, !dbg !32210
  %1367 = fadd <8 x float> %1359, %1366, !dbg !32215
  %1368 = fmul <8 x float> %history.i.i220.sroa.38.sroa.0.014926, %_174.i.i.i.i36.sroa.0.0.copyload, !dbg !32220
  %1369 = fadd <8 x float> %1361, %1368, !dbg !32225
  %1370 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1363), !dbg !32230
  %1371 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1273, <8 x float> %1370), !dbg !32236
  %1372 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1365), !dbg !32230
  %1373 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1371, <8 x float> %1372), !dbg !32236
  %1374 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1367), !dbg !32230
  %1375 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1373, <8 x float> %1374), !dbg !32236
  %1376 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1369), !dbg !32230
  %1377 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1375, <8 x float> %1376), !dbg !32236
  %_39.i.i439.idx = shl i64 %iter.sroa.0.0.i.i29614932, 5, !dbg !32241
  %_39.i.i439 = getelementptr inbounds nuw i8, ptr %peaks_right.i261, i64 %_39.i.i439.idx, !dbg !32241
  store <8 x float> %1377, ptr %_39.i.i439, align 4, !dbg !32246, !alias.scope !32251, !noalias !32255
  %exitcond17528.not = icmp eq i64 %1272, %umax17527, !dbg !32259
  br i1 %exitcond17528.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i298, label %bb5.i.i413, !dbg !31704

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i298: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit245.i
  %history.i.i220.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i220.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit245.i ], [ %lanes.i5518.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194 ], !dbg !32261
  %history.i.i220.sroa.25.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i220.sroa.25.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit245.i ], [ %history.i.i220.sroa.22.sroa.0.014927, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194 ], !dbg !32261
  %history.i.i220.sroa.29.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i220.sroa.29.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit245.i ], [ %history.i.i220.sroa.25.sroa.0.014922, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194 ], !dbg !32261
  %history.i.i220.sroa.32.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i220.sroa.32.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit245.i ], [ %history.i.i220.sroa.29.sroa.0.014923, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194 ], !dbg !32261
  %history.i.i220.sroa.35.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i220.sroa.35.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit245.i ], [ %history.i.i220.sroa.32.sroa.0.014924, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194 ], !dbg !32261
  %history.i.i220.sroa.38.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i220.sroa.38.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit245.i ], [ %history.i.i220.sroa.35.sroa.0.014925, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194 ], !dbg !32261
  %history.i.i220.sroa.41.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i220.sroa.41.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit245.i ], [ %history.i.i220.sroa.38.sroa.0.014926, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194 ], !dbg !32261
  %history.i.i220.sroa.22.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i220.sroa.22.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit245.i ], [ %history.i.i220.sroa.19.sroa.0.014928, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194 ], !dbg !32261
  %history.i.i220.sroa.19.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i220.sroa.19.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit245.i ], [ %history.i.i220.sroa.16.sroa.0.014929, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194 ], !dbg !32261
  %history.i.i220.sroa.16.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i220.sroa.16.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit245.i ], [ %history.i.i220.sroa.13.sroa.0.014930, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194 ], !dbg !32261
  %history.i.i220.sroa.13.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i220.sroa.13.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit245.i ], [ %history.i.i220.sroa.10.sroa.0.014931, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194 ], !dbg !32261
  %history.i.i220.sroa.10.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i220.sroa.10.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit245.i ], [ %history.i.i220.sroa.0.014921, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6194 ], !dbg !32261
  store <8 x float> %history.i.i220.sroa.0.0.lcssa, ptr %hot_right.i268, align 32, !dbg !32262
  store <8 x float> %history.i.i220.sroa.10.sroa.0.0.lcssa, ptr %history.i.i220.sroa.10.0.hot_right.i268.sroa_idx, align 32, !dbg !32262
  store <8 x float> %history.i.i220.sroa.13.sroa.0.0.lcssa, ptr %history.i.i220.sroa.13.0.hot_right.i268.sroa_idx, align 32, !dbg !32262
  store <8 x float> %history.i.i220.sroa.16.sroa.0.0.lcssa, ptr %history.i.i220.sroa.16.0.hot_right.i268.sroa_idx, align 32, !dbg !32262
  store <8 x float> %history.i.i220.sroa.19.sroa.0.0.lcssa, ptr %history.i.i220.sroa.19.0.hot_right.i268.sroa_idx, align 32, !dbg !32262
  store <8 x float> %history.i.i220.sroa.22.sroa.0.0.lcssa, ptr %history.i.i220.sroa.22.0.hot_right.i268.sroa_idx, align 32, !dbg !32262
  store <8 x float> %history.i.i220.sroa.25.sroa.0.0.lcssa, ptr %history.i.i220.sroa.25.0.hot_right.i268.sroa_idx, align 32, !dbg !32262
  store <8 x float> %history.i.i220.sroa.29.sroa.0.0.lcssa, ptr %history.i.i220.sroa.29.0.hot_right.i268.sroa_idx, align 32, !dbg !32262
  store <8 x float> %history.i.i220.sroa.32.sroa.0.0.lcssa, ptr %history.i.i220.sroa.32.0.hot_right.i268.sroa_idx, align 32, !dbg !32262
  store <8 x float> %history.i.i220.sroa.35.sroa.0.0.lcssa, ptr %history.i.i220.sroa.35.0.hot_right.i268.sroa_idx, align 32, !dbg !32262
  store <8 x float> %history.i.i220.sroa.38.sroa.0.0.lcssa, ptr %history.i.i220.sroa.38.0.hot_right.i268.sroa_idx, align 32, !dbg !32262
  store <8 x float> %history.i.i220.sroa.41.sroa.0.0.lcssa, ptr %history.i.i220.sroa.41.0.hot_right.i268.sroa_idx, align 32, !dbg !32262
  br i1 %_20.i213.i14893.not, label %bb15.i285.loopexit, label %bb20.i304.preheader, !dbg !32263

bb20.i304.preheader:                              ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i298
  %_65.i256.sroa.3.0.copyload = load i64, ptr %_65.i256.sroa.3.0..sroa_idx, align 8, !noalias !31088
  %_65.i256.sroa.4.0.copyload = load i64, ptr %_65.i256.sroa.4.0..sroa_idx, align 16, !noalias !31088
  %_66.i255.sroa.3.0.copyload = load i64, ptr %_66.i255.sroa.3.0..sroa_idx, align 8, !noalias !31088
  %_66.i255.sroa.4.0.copyload = load i64, ptr %_66.i255.sroa.4.0..sroa_idx, align 16, !noalias !31088
  %_54.0.i287.i = load ptr, ptr %1144, align 32, !nonnull !12, !align !24
  %_54.1.i288.i = load i64, ptr %1145, align 8
  %_18.i299.i = load i64, ptr %1128, align 16
  %_56.0.i303.i = load ptr, ptr %1146, align 16, !nonnull !12, !align !24
  %_56.1.i304.i = load i64, ptr %1147, align 8
  %_58.1.i316.i = load i64, ptr %1151, align 8
  %_58.0.i315.i = load ptr, ptr %1152, align 32, !nonnull !12, !align !24
  %_54.0.i.i366 = load ptr, ptr %1155, align 32, !nonnull !12, !align !24
  %_54.1.i.i367 = load i64, ptr %1156, align 8
  %_18.i250.i = load i64, ptr %1129, align 16
  %_56.0.i.i379 = load ptr, ptr %1157, align 16, !nonnull !12, !align !24
  %_56.1.i.i380 = load i64, ptr %1158, align 8
  %_58.1.i.i392 = load i64, ptr %1162, align 8
  %_58.0.i.i391 = load ptr, ptr %1163, align 32, !nonnull !12, !align !24
  %_22.i302.i.promoted20304 = load i32, ptr %_22.i302.i, align 4
  %uniform_left.i260.promoted20323 = load <8 x float>, ptr %uniform_left.i260, align 32
  %_22.i.i378.promoted20342 = load i32, ptr %_22.i.i378, align 4
  %uniform_right.i259.promoted20361 = load <8 x float>, ptr %uniform_right.i259, align 32
  br label %bb20.i304, !dbg !32265

bb20.i304:                                        ; preds = %bb20.i304.preheader, %bb32.i404
  %minimum.i.i31.sroa.0.0.lcssa2030120363 = phi <8 x float> [ %uniform_right.i259.promoted20361, %bb20.i304.preheader ], [ %minimum.i.i31.sroa.0.0.lcssa2030120362, %bb32.i404 ]
  %storemerge.i.lcssa2028720344 = phi i32 [ %_22.i.i378.promoted20342, %bb20.i304.preheader ], [ %storemerge.i.lcssa2028720343, %bb32.i404 ]
  %minimum.i282.i.sroa.0.0.lcssa2027120325 = phi <8 x float> [ %uniform_left.i260.promoted20323, %bb20.i304.preheader ], [ %minimum.i282.i.sroa.0.0.lcssa2027120324, %bb32.i404 ]
  %storemerge.i1276.lcssa2025720306 = phi i32 [ %_22.i302.i.promoted20304, %bb20.i304.preheader ], [ %storemerge.i1276.lcssa2025720305, %bb32.i404 ]
  %frame.sroa.0.0.i30214959 = phi i64 [ 0, %bb20.i304.preheader ], [ %_80.i320, %bb32.i404 ]
  %main_cursor.sroa.0.1.i30114958 = phi i64 [ %main_cursor.sroa.0.0.i28714964, %bb20.i304.preheader ], [ %main_cursor.sroa.0.2.i410, %bb32.i404 ]
  %ring_cursor.sroa.0.1.i30014957 = phi i64 [ %ring_cursor.sroa.0.0.i28614963, %bb20.i304.preheader ], [ %ring_cursor.sroa.0.2.i407, %bb32.i404 ]
  %_63.i305 = sub nuw nsw i64 %..i6816, %frame.sroa.0.0.i30214959, !dbg !32276
  %ring.i1945 = load i64, ptr %85, align 8, !dbg !32277, !alias.scope !32279, !noalias !32282, !noundef !12
  %main.i1946 = load i64, ptr %86, align 8, !dbg !32286, !alias.scope !32279, !noalias !32282, !noundef !12
  %_10.i = add i64 %ring_cursor.sroa.0.1.i30014957, 1, !dbg !32287
  %_45.not.i = icmp ult i64 %_10.i, %ring.i1945, !dbg !32288
  %1378 = select i1 %_45.not.i, i64 0, i64 %ring.i1945, !dbg !32288
  %start1.sroa.0.0.i1947 = sub nuw i64 %_10.i, %1378, !dbg !32288
  %_12.i1949 = add i64 %_65.i256.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i30014957, !dbg !32290
  %_46.not.i = icmp ult i64 %_12.i1949, %ring.i1945, !dbg !32291
  %1379 = select i1 %_46.not.i, i64 0, i64 %ring.i1945, !dbg !32291
  %left_end.sroa.0.0.i = sub nuw i64 %_12.i1949, %1379, !dbg !32291
  %_15.i1951 = add i64 %_66.i255.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i30014957, !dbg !32293
  %_47.not.i = icmp ult i64 %_15.i1951, %ring.i1945, !dbg !32294
  %1380 = select i1 %_47.not.i, i64 0, i64 %ring.i1945, !dbg !32294
  %right_end.sroa.0.0.i = sub nuw i64 %_15.i1951, %1380, !dbg !32294
  %_18.i1953 = add i64 %_65.i256.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i30014957, !dbg !32296
  %_48.not.i = icmp ult i64 %_18.i1953, %ring.i1945, !dbg !32297
  %1381 = select i1 %_48.not.i, i64 0, i64 %ring.i1945, !dbg !32297
  %left_expiring.sroa.0.0.i = sub nuw i64 %_18.i1953, %1381, !dbg !32297
  %_21.i1955 = add i64 %_66.i255.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i30014957, !dbg !32299
  %_49.not.i = icmp ult i64 %_21.i1955, %ring.i1945, !dbg !32300
  %1382 = select i1 %_49.not.i, i64 0, i64 %ring.i1945, !dbg !32300
  %right_expiring.sroa.0.0.i = sub nuw i64 %_21.i1955, %1382, !dbg !32300
  %_30.i1956 = sub i64 %ring.i1945, %ring_cursor.sroa.0.1.i30014957, !dbg !32302
  %..i6841 = tail call noundef i64 @llvm.umin.i64(i64 %_30.i1956, i64 %_63.i305), !dbg !32303
  %_31.i1958 = sub i64 %main.i1946, %main_cursor.sroa.0.1.i30114958, !dbg !32305
  %..i6842 = tail call noundef i64 @llvm.umin.i64(i64 %_31.i1958, i64 %..i6841), !dbg !32306
  %_32.i1960 = sub i64 %ring.i1945, %start1.sroa.0.0.i1947, !dbg !32308
  %..i6843 = tail call noundef i64 @llvm.umin.i64(i64 %_32.i1960, i64 %..i6842), !dbg !32309
  %_34.i1962 = sub i64 %ring.i1945, %left_end.sroa.0.0.i, !dbg !32311
  %..i6844 = tail call noundef i64 @llvm.umin.i64(i64 %_34.i1962, i64 %..i6843), !dbg !32312
  %_36.i1964 = sub i64 %ring.i1945, %right_end.sroa.0.0.i, !dbg !32314
  %..i6845 = tail call noundef i64 @llvm.umin.i64(i64 %_36.i1964, i64 %..i6844), !dbg !32315
  %_38.i = sub i64 %ring.i1945, %left_expiring.sroa.0.0.i, !dbg !32317
  %..i6846 = tail call noundef i64 @llvm.umin.i64(i64 %_38.i, i64 %..i6845), !dbg !32318
  %_40.i1965 = sub i64 %ring.i1945, %right_expiring.sroa.0.0.i, !dbg !32320
  %..i6847 = tail call noundef i64 @llvm.umin.i64(i64 %_40.i1965, i64 %..i6846), !dbg !32321
  %_69.i307 = add i64 %frame.sroa.0.0.i30214959, %iter3.sroa.0.0.i28814965, !dbg !32323
  %base.i308 = shl i64 %_69.i307, 3, !dbg !32323
  %base.i30812454 = add i64 %..i6847, %_69.i307, !dbg !32324
  %_73.i310 = shl i64 %base.i30812454, 3, !dbg !32324
  %_174.i311 = icmp ult i64 %_73.i310, %base.i308, !dbg !32265
  %_168.not.i312 = icmp ugt i64 %_73.i310, %left_io.1
  %or.cond.i313 = or i1 %_174.i311, %_168.not.i312, !dbg !32265
  br i1 %or.cond.i313, label %bb55.i412, label %bb53.i314, !dbg !32265, !prof !165

bb55.i412:                                        ; preds = %bb20.i304
  store i32 %storemerge.i1276.lcssa2025720306, ptr %_22.i302.i, align 4
  store <8 x float> %minimum.i282.i.sroa.0.0.lcssa2027120325, ptr %uniform_left.i260, align 32
  store i32 %storemerge.i.lcssa2028720344, ptr %_22.i.i378, align 4
  store <8 x float> %minimum.i.i31.sroa.0.0.lcssa2030120363, ptr %uniform_right.i259, align 32
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i308, i64 noundef %_73.i310, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1f23704f6ca4e855056ec66ed6f2587d) #30, !dbg !32325, !noalias !31059
  unreachable, !dbg !32325

bb53.i314:                                        ; preds = %bb20.i304
  %_177.i315 = getelementptr inbounds nuw float, ptr %left_io.0, i64 %base.i308, !dbg !32326
  %_178.not.i316 = icmp ugt i64 %_73.i310, %right_io.1, !dbg !32330
  br i1 %_178.not.i316, label %bb58.i411, label %bb57.i317, !dbg !32330, !prof !639

bb58.i411:                                        ; preds = %bb53.i314
  store i32 %storemerge.i1276.lcssa2025720306, ptr %_22.i302.i, align 4
  store <8 x float> %minimum.i282.i.sroa.0.0.lcssa2027120325, ptr %uniform_left.i260, align 32
  store i32 %storemerge.i.lcssa2028720344, ptr %_22.i.i378, align 4
  store <8 x float> %minimum.i.i31.sroa.0.0.lcssa2030120363, ptr %uniform_right.i259, align 32
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i308, i64 noundef %_73.i310, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5a76e74e04cb182b892fe72abb75dc84) #30, !dbg !32335, !noalias !31059
  unreachable, !dbg !32335

bb57.i317:                                        ; preds = %bb53.i314
  %_185.i318 = getelementptr inbounds nuw float, ptr %right_io.0, i64 %base.i308, !dbg !32336
  %_77.i319 = shl nuw nsw i64 %frame.sroa.0.0.i30214959, 3, !dbg !32340
  %_80.i320 = add nuw nsw i64 %..i6847, %frame.sroa.0.0.i30214959, !dbg !32342
  %_187.i325 = icmp ult i64 %_80.i320, 33
  br i1 %_187.i325, label %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit, label %bb60.i326, !dbg !32343, !prof !2723

bb60.i326:                                        ; preds = %bb57.i317
  store i32 %storemerge.i1276.lcssa2025720306, ptr %_22.i302.i, align 4
  store <8 x float> %minimum.i282.i.sroa.0.0.lcssa2027120325, ptr %uniform_left.i260, align 32
  store i32 %storemerge.i.lcssa2028720344, ptr %_22.i.i378, align 4
  store <8 x float> %minimum.i.i31.sroa.0.0.lcssa2030120363, ptr %uniform_right.i259, align 32
  %_79.i321 = shl nuw nsw i64 %_80.i320, 3, !dbg !32342
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_77.i319, i64 noundef %_79.i321, i64 noundef 256, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_fa989b02c58b19a323a01f96ca250d1c) #30, !dbg !32351, !noalias !31059
  unreachable, !dbg !32351

_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit: ; preds = %bb57.i317
  %_194.i328 = getelementptr inbounds nuw float, ptr %peaks_left.i262, i64 %_77.i319, !dbg !32352
  %_203.i329 = getelementptr inbounds nuw float, ptr %peaks_right.i261, i64 %_77.i319, !dbg !32356
  %_2.i.i.i14953.not = icmp eq i64 %..i6847, 0, !dbg !32366
  br i1 %_2.i.i.i14953.not, label %bb32.i404, label %bb31.i333.preheader, !dbg !32366

bb31.i333.preheader:                              ; preds = %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit
  %umin17545 = call i64 @llvm.umin.i64(i64 %_34.i1962, i64 %_36.i1964)
  %umin17546 = call i64 @llvm.umin.i64(i64 %umin17545, i64 %_38.i)
  %umin17547 = call i64 @llvm.umin.i64(i64 %umin17546, i64 %_40.i1965)
  %umin17548 = call i64 @llvm.umin.i64(i64 %umin17547, i64 %_32.i1960)
  %umin17549 = call i64 @llvm.umin.i64(i64 %umin17548, i64 %_30.i1956)
  %umin17550 = call i64 @llvm.umin.i64(i64 %umin17549, i64 %_31.i1958)
  %1383 = sub nsw i64 %umin17551, %frame.sroa.0.0.i30214959
  %umin17552 = call i64 @llvm.umin.i64(i64 %umin17550, i64 %1383)
  %1384 = and i64 %umin17552, 2305843009213693951
  %_11.i2092.sroa.0.0.copyload.pre = load <8 x float>, ptr %_110.i343, align 32, !dbg !32372
  %_12.i2091.sroa.0.0.copyload.pre = load <8 x float>, ptr %1134, align 32, !dbg !32376
  %_5.i2084.sroa.0.0.copyload.pre = load <8 x float>, ptr %1136, align 32, !dbg !32377
  %_11.i2078.sroa.0.0.copyload.pre = load <8 x float>, ptr %_114.i344, align 32, !dbg !32381
  %_12.i2077.sroa.0.0.copyload.pre = load <8 x float>, ptr %1137, align 32, !dbg !32382
  %_13.i2076.sroa.0.0.copyload.pre = load <8 x float>, ptr %1138, align 32, !dbg !32383
  %_5.i2070.sroa.0.0.copyload.pre = load <8 x float>, ptr %1139, align 32, !dbg !32384
  %_13.i2104.sroa.0.0.copyload = load <8 x float>, ptr %1132, align 32
  %_13.i2090.sroa.0.0.copyload = load <8 x float>, ptr %1135, align 32
  %_13.i2062.sroa.0.0.copyload = load <8 x float>, ptr %1141, align 32
  %_37.i270.i.sroa.0.0.copyload = load <8 x float>, ptr %1149, align 32
  %_37.i.i23.sroa.0.0.copyload = load <8 x float>, ptr %1160, align 32
  %.promoted20056 = load <8 x float>, ptr %1130, align 32
  %_109.i342.promoted = load <8 x float>, ptr %_109.i342, align 32
  %.promoted20088 = load <8 x float>, ptr %1131, align 32
  %.promoted20105 = load <8 x float>, ptr %1133, align 32
  %_115.i345.promoted = load <8 x float>, ptr %_115.i345, align 32
  %.promoted20227 = load <8 x float>, ptr %1140, align 32
  %.promoted20272 = load <8 x float>, ptr %1148, align 32
  %.promoted20302 = load <8 x float>, ptr %1159, align 32
  br label %bb31.i333

bb31.i333:                                        ; preds = %bb31.i333.preheader, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1513
  %_33.i246.i.sroa.0.0.copyload20303 = phi <8 x float> [ %.promoted20302, %bb31.i333.preheader ], [ %1481, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1513 ]
  %minimum.i.i31.sroa.0.020288 = phi <8 x float> [ %minimum.i.i31.sroa.0.0.lcssa2030120363, %bb31.i333.preheader ], [ %minimum.i.i31.sroa.0.0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1513 ]
  %storemerge.i20274 = phi i32 [ %storemerge.i.lcssa2028720344, %bb31.i333.preheader ], [ %storemerge.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1513 ]
  %_33.i273.i.sroa.0.0.copyload20273 = phi <8 x float> [ %.promoted20272, %bb31.i333.preheader ], [ %1443, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1513 ]
  %minimum.i282.i.sroa.0.020258 = phi <8 x float> [ %minimum.i282.i.sroa.0.0.lcssa2027120325, %bb31.i333.preheader ], [ %minimum.i282.i.sroa.0.0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1513 ]
  %storemerge.i127620244 = phi i32 [ %storemerge.i1276.lcssa2025720306, %bb31.i333.preheader ], [ %storemerge.i1276, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1513 ]
  %_12.i2063.sroa.0.0.copyload20228 = phi <8 x float> [ %.promoted20227, %bb31.i333.preheader ], [ %1418, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1513 ], !dbg !32386
  %_11.i2064.sroa.0.0.copyload20211 = phi <8 x float> [ %_115.i345.promoted, %bb31.i333.preheader ], [ %1417, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1513 ], !dbg !32386
  %1385 = phi <8 x float> [ %.promoted20105, %bb31.i333.preheader ], [ %1396, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1513 ], !dbg !32386
  %_12.i2105.sroa.0.0.copyload20089 = phi <8 x float> [ %.promoted20088, %bb31.i333.preheader ], [ %1394, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1513 ], !dbg !32386
  %_11.i2106.sroa.0.0.copyload20072 = phi <8 x float> [ %_109.i342.promoted, %bb31.i333.preheader ], [ %1393, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1513 ], !dbg !32386
  %1386 = phi <8 x float> [ %.promoted20056, %bb31.i333.preheader ], [ %1388, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1513 ], !dbg !32392
  %_5.i2070.sroa.0.0.copyload = phi <8 x float> [ %_5.i2070.sroa.0.0.copyload.pre, %bb31.i333.preheader ], [ %1412, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1513 ], !dbg !32384
  %_12.i2077.sroa.0.0.copyload = phi <8 x float> [ %_12.i2077.sroa.0.0.copyload.pre, %bb31.i333.preheader ], [ %1410, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1513 ], !dbg !32382
  %_11.i2078.sroa.0.0.copyload = phi <8 x float> [ %_11.i2078.sroa.0.0.copyload.pre, %bb31.i333.preheader ], [ %1409, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1513 ], !dbg !32381
  %_5.i2084.sroa.0.0.copyload = phi <8 x float> [ %_5.i2084.sroa.0.0.copyload.pre, %bb31.i333.preheader ], [ %1404, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1513 ], !dbg !32377
  %_12.i2091.sroa.0.0.copyload = phi <8 x float> [ %_12.i2091.sroa.0.0.copyload.pre, %bb31.i333.preheader ], [ %1402, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1513 ], !dbg !32376
  %_11.i2092.sroa.0.0.copyload = phi <8 x float> [ %_11.i2092.sroa.0.0.copyload.pre, %bb31.i333.preheader ], [ %1401, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1513 ], !dbg !32372
  %iter.i246.sroa.41.014955 = phi i64 [ 0, %bb31.i333.preheader ], [ %_9.0.i6901, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1513 ]
  %1387 = fadd <8 x float> %1386, splat (float -1.000000e+00), !dbg !32386
  %1388 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1387, <8 x float> zeroinitializer), !dbg !32393
  %1389 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1388, <8 x float> zeroinitializer, i8 30), !dbg !32398
  %1390 = fadd <8 x float> %_11.i2106.sroa.0.0.copyload20072, %_12.i2105.sroa.0.0.copyload20089, !dbg !32404
  %1391 = bitcast <8 x float> %1389 to <8 x i32>, !dbg !32409
  %1392 = icmp slt <8 x i32> %1391, zeroinitializer, !dbg !32413
  %1393 = select <8 x i1> %1392, <8 x float> %1390, <8 x float> %_13.i2104.sroa.0.0.copyload, !dbg !32413
  %1394 = select <8 x i1> %1392, <8 x float> %_12.i2105.sroa.0.0.copyload20089, <8 x float> zeroinitializer, !dbg !32415
  %1395 = fadd <8 x float> %1385, splat (float -1.000000e+00), !dbg !32420
  %1396 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1395, <8 x float> zeroinitializer), !dbg !32425
  %1397 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1396, <8 x float> zeroinitializer, i8 30), !dbg !32430
  %1398 = fadd <8 x float> %_11.i2092.sroa.0.0.copyload, %_12.i2091.sroa.0.0.copyload, !dbg !32436
  %1399 = bitcast <8 x float> %1397 to <8 x i32>, !dbg !32441
  %1400 = icmp slt <8 x i32> %1399, zeroinitializer, !dbg !32445
  %1401 = select <8 x i1> %1400, <8 x float> %1398, <8 x float> %_13.i2090.sroa.0.0.copyload, !dbg !32445
  %1402 = select <8 x i1> %1400, <8 x float> %_12.i2091.sroa.0.0.copyload, <8 x float> zeroinitializer, !dbg !32447
  %1403 = fadd <8 x float> %_5.i2084.sroa.0.0.copyload, splat (float -1.000000e+00), !dbg !32452
  %1404 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1403, <8 x float> zeroinitializer), !dbg !32457
  %1405 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1404, <8 x float> zeroinitializer, i8 30), !dbg !32462
  %1406 = fadd <8 x float> %_11.i2078.sroa.0.0.copyload, %_12.i2077.sroa.0.0.copyload, !dbg !32468
  %1407 = bitcast <8 x float> %1405 to <8 x i32>, !dbg !32473
  %1408 = icmp slt <8 x i32> %1407, zeroinitializer, !dbg !32477
  %1409 = select <8 x i1> %1408, <8 x float> %1406, <8 x float> %_13.i2076.sroa.0.0.copyload.pre, !dbg !32477
  %1410 = select <8 x i1> %1408, <8 x float> %_12.i2077.sroa.0.0.copyload, <8 x float> zeroinitializer, !dbg !32479
  %1411 = fadd <8 x float> %_5.i2070.sroa.0.0.copyload, splat (float -1.000000e+00), !dbg !32484
  %1412 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1411, <8 x float> zeroinitializer), !dbg !32489
  %1413 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1412, <8 x float> zeroinitializer, i8 30), !dbg !32494
  %1414 = fadd <8 x float> %_11.i2064.sroa.0.0.copyload20211, %_12.i2063.sroa.0.0.copyload20228, !dbg !32500
  %1415 = bitcast <8 x float> %1413 to <8 x i32>, !dbg !32505
  %1416 = icmp slt <8 x i32> %1415, zeroinitializer, !dbg !32509
  %1417 = select <8 x i1> %1416, <8 x float> %1414, <8 x float> %_13.i2062.sroa.0.0.copyload, !dbg !32509
  %1418 = select <8 x i1> %1416, <8 x float> %_12.i2063.sroa.0.0.copyload20228, <8 x float> zeroinitializer, !dbg !32511
  %start1.i.i.i.i.i.i.i.i = shl i64 %iter.i246.sroa.41.014955, 3, !dbg !32516
  %data.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_194.i328, i64 %start1.i.i.i.i.i.i.i.i, !dbg !32522
  %lanes.i5554.sroa.0.0.copyload = load <8 x float>, ptr %data.i.i.i.i.i.i, align 4, !dbg !32525, !alias.scope !32531, !noalias !32535
  %data.i.i.i.i6899 = getelementptr inbounds nuw float, ptr %_203.i329, i64 %start1.i.i.i.i.i.i.i.i, !dbg !32539
  %lanes.i5545.sroa.0.0.copyload = load <8 x float>, ptr %data.i.i.i.i6899, align 4, !dbg !32542, !alias.scope !32548, !noalias !32552
  %data.i.i.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_177.i315, i64 %start1.i.i.i.i.i.i.i.i, !dbg !32556
  %data.i5.i.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_185.i318, i64 %start1.i.i.i.i.i.i.i.i, !dbg !32558
  %_9.0.i6901 = add nuw nsw i64 %iter.i246.sroa.41.014955, 1, !dbg !32561
  %1419 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i5545.sroa.0.0.copyload, <8 x float> %lanes.i5554.sroa.0.0.copyload), !dbg !32562
  %1420 = select <8 x i1> %1143, <8 x float> %1419, <8 x float> %lanes.i5545.sroa.0.0.copyload, !dbg !32568
  %_205.i351 = add i64 %iter.i246.sroa.41.014955, %ring_cursor.sroa.0.1.i30014957, !dbg !32575
  %_206.i352 = add i64 %iter.i246.sroa.41.014955, %main_cursor.sroa.0.1.i30114958, !dbg !32581
  %_207.i353 = add i64 %iter.i246.sroa.41.014955, %left_end.sroa.0.0.i, !dbg !32582
  %_208.i354 = add i64 %iter.i246.sroa.41.014955, %start1.sroa.0.0.i1947, !dbg !32583
  %_209.i355 = add i64 %iter.i246.sroa.41.014955, %left_expiring.sroa.0.0.i, !dbg !32584
  %base.i9.i290.i = shl i64 %_205.i351, 3, !dbg !32585
  %_7.i10.i291.i = add i64 %base.i9.i290.i, 8, !dbg !32588
  %1421 = or disjoint i64 %base.i9.i290.i, 7, !dbg !32589
  %or.cond.i13.i294.i.not = icmp ult i64 %1421, %_54.1.i288.i, !dbg !32589
  br i1 %or.cond.i13.i294.i.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i295.i, label %bb4.i15.i328.i, !dbg !32589, !prof !2723

bb4.i15.i328.i:                                   ; preds = %bb31.i333
  store i32 %storemerge.i1276.lcssa2025720306, ptr %_22.i302.i, align 4
  store <8 x float> %minimum.i282.i.sroa.0.0.lcssa2027120325, ptr %uniform_left.i260, align 32
  store i32 %storemerge.i.lcssa2028720344, ptr %_22.i.i378, align 4
  store <8 x float> %minimum.i.i31.sroa.0.0.lcssa2030120363, ptr %uniform_right.i259, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32593
  store <8 x float> %1393, ptr %_109.i342, align 32, !dbg !32594
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32595
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32596
  store <8 x float> %1401, ptr %_110.i343, align 32, !dbg !32597
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32598
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32599
  store <8 x float> %1409, ptr %_114.i344, align 32, !dbg !32600
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32601
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32602
  store <8 x float> %1417, ptr %_115.i345, align 32, !dbg !32603
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32604
  store i32 %storemerge.i127620244, ptr %_22.i302.i, align 4, !dbg !32605
  store <8 x float> %minimum.i282.i.sroa.0.020258, ptr %uniform_left.i260, align 32, !dbg !32607
  store i32 %storemerge.i20274, ptr %_22.i.i378, align 4, !dbg !32608
  store <8 x float> %minimum.i.i31.sroa.0.020288, ptr %uniform_right.i259, align 32, !dbg !32611
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i290.i, i64 noundef %_7.i10.i291.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i288.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8421dc8ec9e43c41e5b11981eccec9a3) #30, !dbg !32612, !noalias !32613
  unreachable, !dbg !32612

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i295.i: ; preds = %bb31.i333
  %1422 = select <8 x i1> %1143, <8 x float> %1419, <8 x float> %lanes.i5554.sroa.0.0.copyload, !dbg !32627
  %1423 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1422, <8 x float> %1393, i8 30), !dbg !32632
  %1424 = bitcast <8 x float> %1423 to <8 x i32>, !dbg !32638
  %1425 = icmp slt <8 x i32> %1424, zeroinitializer, !dbg !32642
  %1426 = fdiv <8 x float> %1393, %1422, !dbg !32644
  %1427 = select <8 x i1> %1425, <8 x float> %1426, <8 x float> splat (float 1.000000e+00), !dbg !32642
  %_17.i14.i296.i = getelementptr inbounds nuw float, ptr %_54.0.i287.i, i64 %base.i9.i290.i, !dbg !32649
  store <8 x float> %1427, ptr %_17.i14.i296.i, align 4, !dbg !32651, !alias.scope !32656, !noalias !32660
  %base.i1460 = shl i64 %_207.i353, 3, !dbg !32664
  %1428 = or disjoint i64 %base.i1460, 7, !dbg !32666
  %or.cond.i1464.not = icmp ult i64 %1428, %_54.1.i288.i, !dbg !32666
  br i1 %or.cond.i1464.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1468, label %bb4.i1467, !dbg !32666, !prof !2723

bb4.i1467:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i295.i
  store i32 %storemerge.i1276.lcssa2025720306, ptr %_22.i302.i, align 4
  store <8 x float> %minimum.i282.i.sroa.0.0.lcssa2027120325, ptr %uniform_left.i260, align 32
  store i32 %storemerge.i.lcssa2028720344, ptr %_22.i.i378, align 4
  store <8 x float> %minimum.i.i31.sroa.0.0.lcssa2030120363, ptr %uniform_right.i259, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32593
  store <8 x float> %1393, ptr %_109.i342, align 32, !dbg !32594
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32595
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32596
  store <8 x float> %1401, ptr %_110.i343, align 32, !dbg !32597
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32598
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32599
  store <8 x float> %1409, ptr %_114.i344, align 32, !dbg !32600
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32601
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32602
  store <8 x float> %1417, ptr %_115.i345, align 32, !dbg !32603
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32604
  store i32 %storemerge.i127620244, ptr %_22.i302.i, align 4, !dbg !32605
  store <8 x float> %minimum.i282.i.sroa.0.020258, ptr %uniform_left.i260, align 32, !dbg !32607
  store i32 %storemerge.i20274, ptr %_22.i.i378, align 4, !dbg !32608
  store <8 x float> %minimum.i.i31.sroa.0.020288, ptr %uniform_right.i259, align 32, !dbg !32611
  %_5.i1461 = add i64 %base.i1460, 8, !dbg !32670
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1460, i64 noundef %_5.i1461, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i288.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !32671, !noalias !32672
  unreachable, !dbg !32671

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1468: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i295.i
  %_15.i1466 = getelementptr inbounds nuw float, ptr %_54.0.i287.i, i64 %base.i1460, !dbg !32680
  %lanes.i5225.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1466, align 4, !dbg !32682, !alias.scope !32687, !noalias !32691
  %position.i1270 = zext i32 %storemerge.i127620244 to i64, !dbg !32695
  %1429 = icmp eq i32 %storemerge.i127620244, 0, !dbg !32696
  br i1 %1429, label %bb5.i1272, label %bb3.i1271, !dbg !32696

bb3.i1271:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1468
  %1430 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %minimum.i282.i.sroa.0.020258, <8 x float> %lanes.i5225.sroa.0.0.copyload), !dbg !32697
  br label %bb5.i1272, !dbg !32702

bb5.i1272:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1468, %bb3.i1271
  %minimum.i282.i.sroa.0.0 = phi <8 x float> [ %1430, %bb3.i1271 ], [ %lanes.i5225.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1468 ], !dbg !32607
  %_15.i1273 = add nuw nsw i64 %position.i1270, 1, !dbg !32703
  %complete.i1274 = icmp eq i64 %_15.i1273, %_18.i299.i, !dbg !32703
  br i1 %complete.i1274, label %bb19.i1282, label %bb7.i1275, !dbg !32704

bb7.i1275:                                        ; preds = %bb5.i1272
  %base.i1451 = shl i64 %_208.i354, 3, !dbg !32705
  %1431 = or disjoint i64 %base.i1451, 7, !dbg !32707
  %or.cond.i1455.not = icmp ult i64 %1431, %_54.1.i288.i, !dbg !32707
  br i1 %or.cond.i1455.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1459, label %bb4.i1458, !dbg !32707, !prof !2723

bb4.i1458:                                        ; preds = %bb7.i1275
  store i32 %storemerge.i1276.lcssa2025720306, ptr %_22.i302.i, align 4
  store <8 x float> %minimum.i282.i.sroa.0.0.lcssa2027120325, ptr %uniform_left.i260, align 32
  store i32 %storemerge.i.lcssa2028720344, ptr %_22.i.i378, align 4
  store <8 x float> %minimum.i.i31.sroa.0.0.lcssa2030120363, ptr %uniform_right.i259, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32593
  store <8 x float> %1393, ptr %_109.i342, align 32, !dbg !32594
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32595
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32596
  store <8 x float> %1401, ptr %_110.i343, align 32, !dbg !32597
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32598
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32599
  store <8 x float> %1409, ptr %_114.i344, align 32, !dbg !32600
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32601
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32602
  store <8 x float> %1417, ptr %_115.i345, align 32, !dbg !32603
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32604
  store i32 %storemerge.i127620244, ptr %_22.i302.i, align 4, !dbg !32605
  store <8 x float> %minimum.i282.i.sroa.0.0, ptr %uniform_left.i260, align 32, !dbg !32607
  store i32 %storemerge.i20274, ptr %_22.i.i378, align 4, !dbg !32608
  store <8 x float> %minimum.i.i31.sroa.0.020288, ptr %uniform_right.i259, align 32, !dbg !32611
  %_5.i1452 = add i64 %base.i1451, 8, !dbg !32711
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1451, i64 noundef %_5.i1452, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i288.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !32712, !noalias !32713
  unreachable, !dbg !32712

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1459: ; preds = %bb7.i1275
  %_15.i1457 = getelementptr inbounds nuw float, ptr %_54.0.i287.i, i64 %base.i1451, !dbg !32717
  %lanes.i5232.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1457, align 4, !dbg !32719, !alias.scope !32724, !noalias !32728
  %1432 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %lanes.i5232.sroa.0.0.copyload, <8 x float> %minimum.i282.i.sroa.0.0), !dbg !32732
  %1433 = trunc i64 %_15.i1273 to i32, !dbg !32737
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1294, !dbg !32738

bb19.i1282:                                       ; preds = %bb5.i1272, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1441
  %end.sroa.0.0.i128014948 = phi i64 [ %1437, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1441 ], [ %_207.i353, %bb5.i1272 ]
  %iter.sroa.0.0.i127914947 = phi i64 [ %_30.i1283, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1441 ], [ 0, %bb5.i1272 ]
  %suffix.i1263.sroa.0.014946 = phi <8 x float> [ %1435, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1441 ], [ %lanes.i5225.sroa.0.0.copyload, %bb5.i1272 ]
  %base.i1433 = shl i64 %end.sroa.0.0.i128014948, 3, !dbg !32739
  %1434 = or disjoint i64 %base.i1433, 7, !dbg !32741
  %or.cond.i1437.not = icmp ult i64 %1434, %_54.1.i288.i, !dbg !32741
  br i1 %or.cond.i1437.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1441, label %bb4.i1440, !dbg !32741, !prof !2723

bb4.i1440:                                        ; preds = %bb19.i1282
  store i32 %storemerge.i1276.lcssa2025720306, ptr %_22.i302.i, align 4
  store <8 x float> %minimum.i282.i.sroa.0.0.lcssa2027120325, ptr %uniform_left.i260, align 32
  store i32 %storemerge.i.lcssa2028720344, ptr %_22.i.i378, align 4
  store <8 x float> %minimum.i.i31.sroa.0.0.lcssa2030120363, ptr %uniform_right.i259, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32593
  store <8 x float> %1393, ptr %_109.i342, align 32, !dbg !32594
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32595
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32596
  store <8 x float> %1401, ptr %_110.i343, align 32, !dbg !32597
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32598
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32599
  store <8 x float> %1409, ptr %_114.i344, align 32, !dbg !32600
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32601
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32602
  store <8 x float> %1417, ptr %_115.i345, align 32, !dbg !32603
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32604
  store i32 %storemerge.i127620244, ptr %_22.i302.i, align 4, !dbg !32605
  store <8 x float> %minimum.i282.i.sroa.0.0, ptr %uniform_left.i260, align 32, !dbg !32607
  store i32 %storemerge.i20274, ptr %_22.i.i378, align 4, !dbg !32608
  store <8 x float> %minimum.i.i31.sroa.0.020288, ptr %uniform_right.i259, align 32, !dbg !32611
  %_5.i1434 = add i64 %base.i1433, 8, !dbg !32745
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1433, i64 noundef %_5.i1434, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i288.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !32746, !noalias !32747
  unreachable, !dbg !32746

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1441: ; preds = %bb19.i1282
  %_30.i1283 = add nuw i64 %iter.sroa.0.0.i127914947, 1, !dbg !32751
  %_15.i1439 = getelementptr inbounds nuw float, ptr %_54.0.i287.i, i64 %base.i1433, !dbg !32756
  %lanes.i5246.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1439, align 4, !dbg !32758, !alias.scope !32763, !noalias !32767
  %1435 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %suffix.i1263.sroa.0.014946, <8 x float> %lanes.i5246.sroa.0.0.copyload), !dbg !32771
  store <8 x float> %1435, ptr %_15.i1439, align 4, !dbg !32776, !alias.scope !32782, !noalias !32786
  %1436 = icmp eq i64 %end.sroa.0.0.i128014948, 0, !dbg !32790
  %spec.store.select.i1291 = select i1 %1436, i64 %ring.i276, i64 %end.sroa.0.0.i128014948, !dbg !32790
  %1437 = add i64 %spec.store.select.i1291, -1, !dbg !32791
  %exitcond17534.not = icmp eq i64 %_30.i1283, %_18.i299.i, !dbg !32792
  br i1 %exitcond17534.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1294, label %bb19.i1282, !dbg !32794

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1294: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1441, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1459
  %minimum.i282.i.sroa.0.1 = phi <8 x float> [ %1432, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1459 ], [ %minimum.i282.i.sroa.0.0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1441 ], !dbg !32607
  %storemerge.i1276 = phi i32 [ %1433, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1459 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1441 ], !dbg !32795
  %1438 = fmul <8 x float> %minimum.i282.i.sroa.0.1, splat (float 1.638400e+04), !dbg !32796
  %1439 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %1438), !dbg !32801
  %1440 = fmul <8 x float> %1439, splat (float 0x3F10000000000000), !dbg !32806
  %base.i1532 = shl i64 %_209.i355, 3, !dbg !32811
  %1441 = or disjoint i64 %base.i1532, 7, !dbg !32813
  %or.cond.i1536.not = icmp ult i64 %1441, %_56.1.i304.i, !dbg !32813
  br i1 %or.cond.i1536.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1540, label %bb4.i1539, !dbg !32813, !prof !2723

bb4.i1539:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1294
  store i32 %storemerge.i1276.lcssa2025720306, ptr %_22.i302.i, align 4
  store <8 x float> %minimum.i282.i.sroa.0.0.lcssa2027120325, ptr %uniform_left.i260, align 32
  store i32 %storemerge.i.lcssa2028720344, ptr %_22.i.i378, align 4
  store <8 x float> %minimum.i.i31.sroa.0.0.lcssa2030120363, ptr %uniform_right.i259, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32593
  store <8 x float> %1393, ptr %_109.i342, align 32, !dbg !32594
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32595
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32596
  store <8 x float> %1401, ptr %_110.i343, align 32, !dbg !32597
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32598
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32599
  store <8 x float> %1409, ptr %_114.i344, align 32, !dbg !32600
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32601
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32602
  store <8 x float> %1417, ptr %_115.i345, align 32, !dbg !32603
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32604
  store i32 %storemerge.i1276, ptr %_22.i302.i, align 4, !dbg !32605
  store <8 x float> %minimum.i282.i.sroa.0.0, ptr %uniform_left.i260, align 32, !dbg !32607
  store i32 %storemerge.i20274, ptr %_22.i.i378, align 4, !dbg !32608
  store <8 x float> %minimum.i.i31.sroa.0.020288, ptr %uniform_right.i259, align 32, !dbg !32611
  %_5.i1533 = add i64 %base.i1532, 8, !dbg !32817
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1532, i64 noundef %_5.i1533, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i304.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !32818, !noalias !32819
  unreachable, !dbg !32818

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1540: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1294
  %_15.i1538 = getelementptr inbounds nuw float, ptr %_56.0.i303.i, i64 %base.i1532, !dbg !32823
  %lanes.i5169.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1538, align 4, !dbg !32825, !alias.scope !32830, !noalias !32834
  %1442 = fadd <8 x float> %1440, %_33.i273.i.sroa.0.0.copyload20273, !dbg !32838
  %1443 = fsub <8 x float> %1442, %lanes.i5169.sroa.0.0.copyload, !dbg !32843
  store <8 x float> %1443, ptr %1148, align 32, !dbg !32848
  %_8.not.i4.i311.i = icmp ugt i64 %_7.i10.i291.i, %_56.1.i304.i
  br i1 %_8.not.i4.i311.i, label %bb4.i7.i327.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i313.i, !dbg !32849, !prof !165

bb4.i7.i327.i:                                    ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1540
  store i32 %storemerge.i1276.lcssa2025720306, ptr %_22.i302.i, align 4
  store <8 x float> %minimum.i282.i.sroa.0.0.lcssa2027120325, ptr %uniform_left.i260, align 32
  store i32 %storemerge.i.lcssa2028720344, ptr %_22.i.i378, align 4
  store <8 x float> %minimum.i.i31.sroa.0.0.lcssa2030120363, ptr %uniform_right.i259, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32593
  store <8 x float> %1393, ptr %_109.i342, align 32, !dbg !32594
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32595
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32596
  store <8 x float> %1401, ptr %_110.i343, align 32, !dbg !32597
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32598
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32599
  store <8 x float> %1409, ptr %_114.i344, align 32, !dbg !32600
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32601
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32602
  store <8 x float> %1417, ptr %_115.i345, align 32, !dbg !32603
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32604
  store i32 %storemerge.i1276, ptr %_22.i302.i, align 4, !dbg !32605
  store <8 x float> %minimum.i282.i.sroa.0.0, ptr %uniform_left.i260, align 32, !dbg !32607
  store i32 %storemerge.i20274, ptr %_22.i.i378, align 4, !dbg !32608
  store <8 x float> %minimum.i.i31.sroa.0.020288, ptr %uniform_right.i259, align 32, !dbg !32611
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i290.i, i64 noundef %_7.i10.i291.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i304.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8421dc8ec9e43c41e5b11981eccec9a3) #30, !dbg !32854, !noalias !32855
  unreachable, !dbg !32854

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i313.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1540
  %_17.i6.i314.i = getelementptr inbounds nuw float, ptr %_56.0.i303.i, i64 %base.i9.i290.i, !dbg !32859
  store <8 x float> %1440, ptr %_17.i6.i314.i, align 4, !dbg !32861, !alias.scope !32866, !noalias !32870
  %_41.i266.i.sroa.0.0.copyload = load <8 x float>, ptr %1150, align 32, !dbg !32874
  %1444 = fdiv <8 x float> %1443, %_37.i270.i.sroa.0.0.copyload, !dbg !32875
  %1445 = fsub <8 x float> splat (float 1.000000e+00), %1444, !dbg !32880
  %1446 = fsub <8 x float> %1445, %_41.i266.i.sroa.0.0.copyload, !dbg !32885
  %1447 = fmul <8 x float> %1401, %1446, !dbg !32890
  %1448 = fadd <8 x float> %_41.i266.i.sroa.0.0.copyload, %1447, !dbg !32895
  %1449 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1445, <8 x float> %1448), !dbg !32899
  %1450 = bitcast <8 x float> %1449 to <8 x i32>, !dbg !32904
  %1451 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1449), !dbg !32910
  %1452 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1451, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !32912
  %1453 = bitcast <8 x float> %1452 to <8 x i32>, !dbg !32918
  %1454 = xor <8 x i32> %1453, splat (i32 -1), !dbg !32924
  %1455 = and <8 x i32> %1454, %1450, !dbg !32926
  store <8 x i32> %1455, ptr %1150, align 32, !dbg !32930
  %base.i1523 = shl i64 %_206.i352, 3, !dbg !32931
  %_5.i1524 = add i64 %base.i1523, 8, !dbg !32933
  %1456 = or disjoint i64 %base.i1523, 7, !dbg !32934
  %or.cond.i1527.not = icmp ult i64 %1456, %_58.1.i316.i, !dbg !32934
  br i1 %or.cond.i1527.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1531, label %bb4.i1530, !dbg !32934, !prof !2723

bb4.i1530:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i313.i
  store i32 %storemerge.i1276.lcssa2025720306, ptr %_22.i302.i, align 4
  store <8 x float> %minimum.i282.i.sroa.0.0.lcssa2027120325, ptr %uniform_left.i260, align 32
  store i32 %storemerge.i.lcssa2028720344, ptr %_22.i.i378, align 4
  store <8 x float> %minimum.i.i31.sroa.0.0.lcssa2030120363, ptr %uniform_right.i259, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32593
  store <8 x float> %1393, ptr %_109.i342, align 32, !dbg !32594
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32595
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32596
  store <8 x float> %1401, ptr %_110.i343, align 32, !dbg !32597
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32598
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32599
  store <8 x float> %1409, ptr %_114.i344, align 32, !dbg !32600
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32601
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32602
  store <8 x float> %1417, ptr %_115.i345, align 32, !dbg !32603
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32604
  store i32 %storemerge.i1276, ptr %_22.i302.i, align 4, !dbg !32605
  store <8 x float> %minimum.i282.i.sroa.0.0, ptr %uniform_left.i260, align 32, !dbg !32607
  store i32 %storemerge.i20274, ptr %_22.i.i378, align 4, !dbg !32608
  store <8 x float> %minimum.i.i31.sroa.0.020288, ptr %uniform_right.i259, align 32, !dbg !32611
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1523, i64 noundef %_5.i1524, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i316.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !32938, !noalias !32939
  unreachable, !dbg !32938

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1531: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i313.i
  %1457 = bitcast <8 x i32> %1455 to <8 x float>, !dbg !32943
  %1458 = fsub <8 x float> splat (float 1.000000e+00), %1457, !dbg !32944
  %_15.i1529 = getelementptr inbounds nuw float, ptr %_58.0.i315.i, i64 %base.i1523, !dbg !32949
  %lanes.i5176.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1529, align 4, !dbg !32951, !alias.scope !32956, !noalias !32960
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_15.i1529, ptr noundef nonnull align 4 dereferenceable(32) %data.i.i.i.i.i.i.i.i, i64 32, i1 false), !dbg !32964
  %1459 = fmul <8 x float> %1458, %lanes.i5176.sroa.0.0.copyload, !dbg !32970
  %1460 = select <8 x i1> %1154, <8 x float> %lanes.i5176.sroa.0.0.copyload, <8 x float> %1459, !dbg !32975
  store <8 x float> %1460, ptr %data.i.i.i.i.i.i.i.i, align 4, !dbg !32980, !alias.scope !32985, !noalias !32989
  %_212.i363 = add i64 %iter.i246.sroa.41.014955, %right_end.sroa.0.0.i, !dbg !32993
  %_214.i365 = add i64 %iter.i246.sroa.41.014955, %right_expiring.sroa.0.0.i, !dbg !32995
  %_8.not.i12.i.i372 = icmp ugt i64 %_7.i10.i291.i, %_54.1.i.i367
  br i1 %_8.not.i12.i.i372, label %bb4.i15.i.i403, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i374, !dbg !32996, !prof !165

bb4.i15.i.i403:                                   ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1531
  store i32 %storemerge.i1276.lcssa2025720306, ptr %_22.i302.i, align 4
  store <8 x float> %minimum.i282.i.sroa.0.0.lcssa2027120325, ptr %uniform_left.i260, align 32
  store i32 %storemerge.i.lcssa2028720344, ptr %_22.i.i378, align 4
  store <8 x float> %minimum.i.i31.sroa.0.0.lcssa2030120363, ptr %uniform_right.i259, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32593
  store <8 x float> %1393, ptr %_109.i342, align 32, !dbg !32594
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32595
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32596
  store <8 x float> %1401, ptr %_110.i343, align 32, !dbg !32597
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32598
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32599
  store <8 x float> %1409, ptr %_114.i344, align 32, !dbg !32600
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32601
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32602
  store <8 x float> %1417, ptr %_115.i345, align 32, !dbg !32603
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32604
  store i32 %storemerge.i1276, ptr %_22.i302.i, align 4, !dbg !32605
  store <8 x float> %minimum.i282.i.sroa.0.0, ptr %uniform_left.i260, align 32, !dbg !32607
  store i32 %storemerge.i20274, ptr %_22.i.i378, align 4, !dbg !32608
  store <8 x float> %minimum.i.i31.sroa.0.020288, ptr %uniform_right.i259, align 32, !dbg !32611
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i290.i, i64 noundef %_7.i10.i291.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i367, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8421dc8ec9e43c41e5b11981eccec9a3) #30, !dbg !33001, !noalias !33002
  unreachable, !dbg !33001

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i374: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1531
  %1461 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1420, <8 x float> %1409, i8 30), !dbg !33016
  %1462 = bitcast <8 x float> %1461 to <8 x i32>, !dbg !33022
  %1463 = icmp slt <8 x i32> %1462, zeroinitializer, !dbg !33026
  %1464 = fdiv <8 x float> %1409, %1420, !dbg !33028
  %1465 = select <8 x i1> %1463, <8 x float> %1464, <8 x float> splat (float 1.000000e+00), !dbg !33026
  %_17.i14.i.i375 = getelementptr inbounds nuw float, ptr %_54.0.i.i366, i64 %base.i9.i290.i, !dbg !33033
  store <8 x float> %1465, ptr %_17.i14.i.i375, align 4, !dbg !33035, !alias.scope !33040, !noalias !33044
  %base.i1496 = shl i64 %_212.i363, 3, !dbg !33048
  %1466 = or disjoint i64 %base.i1496, 7, !dbg !33050
  %or.cond.i1500.not = icmp ult i64 %1466, %_54.1.i.i367, !dbg !33050
  br i1 %or.cond.i1500.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1504, label %bb4.i1503, !dbg !33050, !prof !2723

bb4.i1503:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i374
  store i32 %storemerge.i1276.lcssa2025720306, ptr %_22.i302.i, align 4
  store <8 x float> %minimum.i282.i.sroa.0.0.lcssa2027120325, ptr %uniform_left.i260, align 32
  store i32 %storemerge.i.lcssa2028720344, ptr %_22.i.i378, align 4
  store <8 x float> %minimum.i.i31.sroa.0.0.lcssa2030120363, ptr %uniform_right.i259, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32593
  store <8 x float> %1393, ptr %_109.i342, align 32, !dbg !32594
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32595
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32596
  store <8 x float> %1401, ptr %_110.i343, align 32, !dbg !32597
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32598
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32599
  store <8 x float> %1409, ptr %_114.i344, align 32, !dbg !32600
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32601
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32602
  store <8 x float> %1417, ptr %_115.i345, align 32, !dbg !32603
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32604
  store i32 %storemerge.i1276, ptr %_22.i302.i, align 4, !dbg !32605
  store <8 x float> %minimum.i282.i.sroa.0.0, ptr %uniform_left.i260, align 32, !dbg !32607
  store i32 %storemerge.i20274, ptr %_22.i.i378, align 4, !dbg !32608
  store <8 x float> %minimum.i.i31.sroa.0.020288, ptr %uniform_right.i259, align 32, !dbg !32611
  %_5.i1497 = add i64 %base.i1496, 8, !dbg !33054
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1496, i64 noundef %_5.i1497, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i367, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !33055, !noalias !33056
  unreachable, !dbg !33055

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1504: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i374
  %_15.i1502 = getelementptr inbounds nuw float, ptr %_54.0.i.i366, i64 %base.i1496, !dbg !33064
  %lanes.i5197.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1502, align 4, !dbg !33066, !alias.scope !33071, !noalias !33075
  %position.i = zext i32 %storemerge.i20274 to i64, !dbg !33079
  %1467 = icmp eq i32 %storemerge.i20274, 0, !dbg !33080
  br i1 %1467, label %bb5.i1250, label %bb3.i1249, !dbg !33080

bb3.i1249:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1504
  %1468 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %minimum.i.i31.sroa.0.020288, <8 x float> %lanes.i5197.sroa.0.0.copyload), !dbg !33081
  br label %bb5.i1250, !dbg !33086

bb5.i1250:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1504, %bb3.i1249
  %minimum.i.i31.sroa.0.0 = phi <8 x float> [ %1468, %bb3.i1249 ], [ %lanes.i5197.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1504 ], !dbg !32611
  %_15.i = add nuw nsw i64 %position.i, 1, !dbg !33087
  %complete.i = icmp eq i64 %_15.i, %_18.i250.i, !dbg !33087
  br i1 %complete.i, label %bb19.i1255, label %bb7.i1251, !dbg !33088

bb7.i1251:                                        ; preds = %bb5.i1250
  %base.i1487 = shl i64 %_208.i354, 3, !dbg !33089
  %1469 = or disjoint i64 %base.i1487, 7, !dbg !33091
  %or.cond.i1491.not = icmp ult i64 %1469, %_54.1.i.i367, !dbg !33091
  br i1 %or.cond.i1491.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1495, label %bb4.i1494, !dbg !33091, !prof !2723

bb4.i1494:                                        ; preds = %bb7.i1251
  store i32 %storemerge.i1276.lcssa2025720306, ptr %_22.i302.i, align 4
  store <8 x float> %minimum.i282.i.sroa.0.0.lcssa2027120325, ptr %uniform_left.i260, align 32
  store i32 %storemerge.i.lcssa2028720344, ptr %_22.i.i378, align 4
  store <8 x float> %minimum.i.i31.sroa.0.0.lcssa2030120363, ptr %uniform_right.i259, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32593
  store <8 x float> %1393, ptr %_109.i342, align 32, !dbg !32594
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32595
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32596
  store <8 x float> %1401, ptr %_110.i343, align 32, !dbg !32597
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32598
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32599
  store <8 x float> %1409, ptr %_114.i344, align 32, !dbg !32600
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32601
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32602
  store <8 x float> %1417, ptr %_115.i345, align 32, !dbg !32603
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32604
  store i32 %storemerge.i1276, ptr %_22.i302.i, align 4, !dbg !32605
  store <8 x float> %minimum.i282.i.sroa.0.0, ptr %uniform_left.i260, align 32, !dbg !32607
  store i32 %storemerge.i20274, ptr %_22.i.i378, align 4, !dbg !32608
  store <8 x float> %minimum.i.i31.sroa.0.0, ptr %uniform_right.i259, align 32, !dbg !32611
  %_5.i1488 = add i64 %base.i1487, 8, !dbg !33095
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1487, i64 noundef %_5.i1488, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i367, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !33096, !noalias !33097
  unreachable, !dbg !33096

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1495: ; preds = %bb7.i1251
  %_15.i1493 = getelementptr inbounds nuw float, ptr %_54.0.i.i366, i64 %base.i1487, !dbg !33101
  %lanes.i5204.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1493, align 4, !dbg !33103, !alias.scope !33108, !noalias !33112
  %1470 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %lanes.i5204.sroa.0.0.copyload, <8 x float> %minimum.i.i31.sroa.0.0), !dbg !33116
  %1471 = trunc i64 %_15.i to i32, !dbg !33121
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, !dbg !33122

bb19.i1255:                                       ; preds = %bb5.i1250, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1477
  %end.sroa.0.0.i14952 = phi i64 [ %1475, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1477 ], [ %_212.i363, %bb5.i1250 ]
  %iter.sroa.0.0.i125414951 = phi i64 [ %_30.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1477 ], [ 0, %bb5.i1250 ]
  %suffix.i.sroa.0.014950 = phi <8 x float> [ %1473, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1477 ], [ %lanes.i5197.sroa.0.0.copyload, %bb5.i1250 ]
  %base.i1469 = shl i64 %end.sroa.0.0.i14952, 3, !dbg !33123
  %1472 = or disjoint i64 %base.i1469, 7, !dbg !33125
  %or.cond.i1473.not = icmp ult i64 %1472, %_54.1.i.i367, !dbg !33125
  br i1 %or.cond.i1473.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1477, label %bb4.i1476, !dbg !33125, !prof !2723

bb4.i1476:                                        ; preds = %bb19.i1255
  store i32 %storemerge.i1276.lcssa2025720306, ptr %_22.i302.i, align 4
  store <8 x float> %minimum.i282.i.sroa.0.0.lcssa2027120325, ptr %uniform_left.i260, align 32
  store i32 %storemerge.i.lcssa2028720344, ptr %_22.i.i378, align 4
  store <8 x float> %minimum.i.i31.sroa.0.0.lcssa2030120363, ptr %uniform_right.i259, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32593
  store <8 x float> %1393, ptr %_109.i342, align 32, !dbg !32594
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32595
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32596
  store <8 x float> %1401, ptr %_110.i343, align 32, !dbg !32597
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32598
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32599
  store <8 x float> %1409, ptr %_114.i344, align 32, !dbg !32600
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32601
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32602
  store <8 x float> %1417, ptr %_115.i345, align 32, !dbg !32603
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32604
  store i32 %storemerge.i1276, ptr %_22.i302.i, align 4, !dbg !32605
  store <8 x float> %minimum.i282.i.sroa.0.0, ptr %uniform_left.i260, align 32, !dbg !32607
  store i32 %storemerge.i20274, ptr %_22.i.i378, align 4, !dbg !32608
  store <8 x float> %minimum.i.i31.sroa.0.0, ptr %uniform_right.i259, align 32, !dbg !32611
  %_5.i1470 = add i64 %base.i1469, 8, !dbg !33129
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1469, i64 noundef %_5.i1470, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i367, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !33130, !noalias !33131
  unreachable, !dbg !33130

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1477: ; preds = %bb19.i1255
  %_30.i = add nuw i64 %iter.sroa.0.0.i125414951, 1, !dbg !33135
  %_15.i1475 = getelementptr inbounds nuw float, ptr %_54.0.i.i366, i64 %base.i1469, !dbg !33140
  %lanes.i5218.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1475, align 4, !dbg !33142, !alias.scope !33147, !noalias !33151
  %1473 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %suffix.i.sroa.0.014950, <8 x float> %lanes.i5218.sroa.0.0.copyload), !dbg !33155
  store <8 x float> %1473, ptr %_15.i1475, align 4, !dbg !33160, !alias.scope !33166, !noalias !33170
  %1474 = icmp eq i64 %end.sroa.0.0.i14952, 0, !dbg !33174
  %spec.store.select.i1259 = select i1 %1474, i64 %ring.i276, i64 %end.sroa.0.0.i14952, !dbg !33174
  %1475 = add i64 %spec.store.select.i1259, -1, !dbg !33175
  %exitcond17540.not = icmp eq i64 %_30.i, %_18.i250.i, !dbg !33176
  br i1 %exitcond17540.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, label %bb19.i1255, !dbg !33178

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1477, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1495
  %minimum.i.i31.sroa.0.1 = phi <8 x float> [ %1470, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1495 ], [ %minimum.i.i31.sroa.0.0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1477 ], !dbg !32611
  %storemerge.i = phi i32 [ %1471, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1495 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1477 ], !dbg !33179
  %1476 = fmul <8 x float> %minimum.i.i31.sroa.0.1, splat (float 1.638400e+04), !dbg !33180
  %1477 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %1476), !dbg !33185
  %1478 = fmul <8 x float> %1477, splat (float 0x3F10000000000000), !dbg !33190
  %base.i1514 = shl i64 %_214.i365, 3, !dbg !33195
  %1479 = or disjoint i64 %base.i1514, 7, !dbg !33197
  %or.cond.i1518.not = icmp ult i64 %1479, %_56.1.i.i380, !dbg !33197
  br i1 %or.cond.i1518.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1522, label %bb4.i1521, !dbg !33197, !prof !2723

bb4.i1521:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit
  store i32 %storemerge.i1276.lcssa2025720306, ptr %_22.i302.i, align 4
  store <8 x float> %minimum.i282.i.sroa.0.0.lcssa2027120325, ptr %uniform_left.i260, align 32
  store i32 %storemerge.i.lcssa2028720344, ptr %_22.i.i378, align 4
  store <8 x float> %minimum.i.i31.sroa.0.0.lcssa2030120363, ptr %uniform_right.i259, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32593
  store <8 x float> %1393, ptr %_109.i342, align 32, !dbg !32594
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32595
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32596
  store <8 x float> %1401, ptr %_110.i343, align 32, !dbg !32597
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32598
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32599
  store <8 x float> %1409, ptr %_114.i344, align 32, !dbg !32600
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32601
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32602
  store <8 x float> %1417, ptr %_115.i345, align 32, !dbg !32603
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32604
  store i32 %storemerge.i1276, ptr %_22.i302.i, align 4, !dbg !32605
  store <8 x float> %minimum.i282.i.sroa.0.0, ptr %uniform_left.i260, align 32, !dbg !32607
  store i32 %storemerge.i, ptr %_22.i.i378, align 4, !dbg !32608
  store <8 x float> %minimum.i.i31.sroa.0.0, ptr %uniform_right.i259, align 32, !dbg !32611
  %_5.i1515 = add i64 %base.i1514, 8, !dbg !33201
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1514, i64 noundef %_5.i1515, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i380, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !33202, !noalias !33203
  unreachable, !dbg !33202

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1522: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit
  %_15.i1520 = getelementptr inbounds nuw float, ptr %_56.0.i.i379, i64 %base.i1514, !dbg !33207
  %lanes.i5183.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1520, align 4, !dbg !33209, !alias.scope !33214, !noalias !33218
  %1480 = fadd <8 x float> %1478, %_33.i246.i.sroa.0.0.copyload20303, !dbg !33222
  %1481 = fsub <8 x float> %1480, %lanes.i5183.sroa.0.0.copyload, !dbg !33227
  store <8 x float> %1481, ptr %1159, align 32, !dbg !33232
  %_8.not.i4.i.i387 = icmp ugt i64 %_7.i10.i291.i, %_56.1.i.i380
  br i1 %_8.not.i4.i.i387, label %bb4.i7.i.i402, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i389, !dbg !33233, !prof !165

bb4.i7.i.i402:                                    ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1522
  store i32 %storemerge.i1276.lcssa2025720306, ptr %_22.i302.i, align 4
  store <8 x float> %minimum.i282.i.sroa.0.0.lcssa2027120325, ptr %uniform_left.i260, align 32
  store i32 %storemerge.i.lcssa2028720344, ptr %_22.i.i378, align 4
  store <8 x float> %minimum.i.i31.sroa.0.0.lcssa2030120363, ptr %uniform_right.i259, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32593
  store <8 x float> %1393, ptr %_109.i342, align 32, !dbg !32594
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32595
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32596
  store <8 x float> %1401, ptr %_110.i343, align 32, !dbg !32597
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32598
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32599
  store <8 x float> %1409, ptr %_114.i344, align 32, !dbg !32600
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32601
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32602
  store <8 x float> %1417, ptr %_115.i345, align 32, !dbg !32603
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32604
  store i32 %storemerge.i1276, ptr %_22.i302.i, align 4, !dbg !32605
  store <8 x float> %minimum.i282.i.sroa.0.0, ptr %uniform_left.i260, align 32, !dbg !32607
  store i32 %storemerge.i, ptr %_22.i.i378, align 4, !dbg !32608
  store <8 x float> %minimum.i.i31.sroa.0.0, ptr %uniform_right.i259, align 32, !dbg !32611
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i290.i, i64 noundef %_7.i10.i291.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i380, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8421dc8ec9e43c41e5b11981eccec9a3) #30, !dbg !33238, !noalias !33239
  unreachable, !dbg !33238

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i389: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1522
  %_17.i6.i.i390 = getelementptr inbounds nuw float, ptr %_56.0.i.i379, i64 %base.i9.i290.i, !dbg !33243
  store <8 x float> %1478, ptr %_17.i6.i.i390, align 4, !dbg !33245, !alias.scope !33250, !noalias !33254
  %_41.i.i19.sroa.0.0.copyload = load <8 x float>, ptr %1161, align 32, !dbg !33258
  %1482 = fdiv <8 x float> %1481, %_37.i.i23.sroa.0.0.copyload, !dbg !33259
  %1483 = fsub <8 x float> splat (float 1.000000e+00), %1482, !dbg !33264
  %1484 = fsub <8 x float> %1483, %_41.i.i19.sroa.0.0.copyload, !dbg !33269
  %1485 = fmul <8 x float> %1417, %1484, !dbg !33274
  %1486 = fadd <8 x float> %_41.i.i19.sroa.0.0.copyload, %1485, !dbg !33279
  %1487 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1483, <8 x float> %1486), !dbg !33283
  %1488 = bitcast <8 x float> %1487 to <8 x i32>, !dbg !33288
  %1489 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1487), !dbg !33294
  %1490 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1489, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !33296
  %1491 = bitcast <8 x float> %1490 to <8 x i32>, !dbg !33302
  %1492 = xor <8 x i32> %1491, splat (i32 -1), !dbg !33308
  %1493 = and <8 x i32> %1492, %1488, !dbg !33310
  store <8 x i32> %1493, ptr %1161, align 32, !dbg !33314
  %_6.not.i1508 = icmp ugt i64 %_5.i1524, %_58.1.i.i392
  br i1 %_6.not.i1508, label %bb4.i1512, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1513, !dbg !33315, !prof !165

bb4.i1512:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i389
  store i32 %storemerge.i1276.lcssa2025720306, ptr %_22.i302.i, align 4
  store <8 x float> %minimum.i282.i.sroa.0.0.lcssa2027120325, ptr %uniform_left.i260, align 32
  store i32 %storemerge.i.lcssa2028720344, ptr %_22.i.i378, align 4
  store <8 x float> %minimum.i.i31.sroa.0.0.lcssa2030120363, ptr %uniform_right.i259, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32593
  store <8 x float> %1393, ptr %_109.i342, align 32, !dbg !32594
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32595
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32596
  store <8 x float> %1401, ptr %_110.i343, align 32, !dbg !32597
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32598
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32599
  store <8 x float> %1409, ptr %_114.i344, align 32, !dbg !32600
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32601
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32602
  store <8 x float> %1417, ptr %_115.i345, align 32, !dbg !32603
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32604
  store i32 %storemerge.i1276, ptr %_22.i302.i, align 4, !dbg !32605
  store <8 x float> %minimum.i282.i.sroa.0.0, ptr %uniform_left.i260, align 32, !dbg !32607
  store i32 %storemerge.i, ptr %_22.i.i378, align 4, !dbg !32608
  store <8 x float> %minimum.i.i31.sroa.0.0, ptr %uniform_right.i259, align 32, !dbg !32611
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1523, i64 noundef %_5.i1524, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i.i392, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !33320, !noalias !33321
  unreachable, !dbg !33320

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1513: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i389
  %1494 = bitcast <8 x i32> %1493 to <8 x float>, !dbg !33325
  %1495 = fsub <8 x float> splat (float 1.000000e+00), %1494, !dbg !33326
  %_15.i1511 = getelementptr inbounds nuw float, ptr %_58.0.i.i391, i64 %base.i1523, !dbg !33331
  %lanes.i5190.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1511, align 4, !dbg !33333, !alias.scope !33338, !noalias !33342
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_15.i1511, ptr noundef nonnull align 4 dereferenceable(32) %data.i5.i.i.i.i.i.i.i, i64 32, i1 false), !dbg !33346
  %1496 = fmul <8 x float> %1495, %lanes.i5190.sroa.0.0.copyload, !dbg !33352
  %1497 = select <8 x i1> %1154, <8 x float> %lanes.i5190.sroa.0.0.copyload, <8 x float> %1496, !dbg !33357
  store <8 x float> %1497, ptr %data.i5.i.i.i.i.i.i.i, align 4, !dbg !33362, !alias.scope !33367, !noalias !33371
  %exitcond17553.not = icmp eq i64 %_9.0.i6901, %1384, !dbg !32366
  br i1 %exitcond17553.not, label %bb32.i404.loopexit, label %bb31.i333, !dbg !32366

bb32.i404.loopexit:                               ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1513
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32593
  store <8 x float> %1393, ptr %_109.i342, align 32, !dbg !32594
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32595
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32596
  store <8 x float> %1401, ptr %_110.i343, align 32, !dbg !32597
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32598
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32599
  store <8 x float> %1409, ptr %_114.i344, align 32, !dbg !32600
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32601
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32602
  store <8 x float> %1417, ptr %_115.i345, align 32, !dbg !32603
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32604
  br label %bb32.i404, !dbg !33375

bb32.i404:                                        ; preds = %bb32.i404.loopexit, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit
  %minimum.i.i31.sroa.0.0.lcssa2030120362 = phi <8 x float> [ %minimum.i.i31.sroa.0.0, %bb32.i404.loopexit ], [ %minimum.i.i31.sroa.0.0.lcssa2030120363, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %storemerge.i.lcssa2028720343 = phi i32 [ %storemerge.i, %bb32.i404.loopexit ], [ %storemerge.i.lcssa2028720344, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %minimum.i282.i.sroa.0.0.lcssa2027120324 = phi <8 x float> [ %minimum.i282.i.sroa.0.0, %bb32.i404.loopexit ], [ %minimum.i282.i.sroa.0.0.lcssa2027120325, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %storemerge.i1276.lcssa2025720305 = phi i32 [ %storemerge.i1276, %bb32.i404.loopexit ], [ %storemerge.i1276.lcssa2025720306, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_138.i405 = add i64 %..i6847, %ring_cursor.sroa.0.1.i30014957, !dbg !33375
  %_204.not.i406 = icmp ult i64 %_138.i405, %ring.i276, !dbg !33376
  %1498 = select i1 %_204.not.i406, i64 0, i64 %ring.i276, !dbg !33376
  %ring_cursor.sroa.0.2.i407 = sub nuw i64 %_138.i405, %1498, !dbg !33376
  %_140.i408 = add i64 %..i6847, %main_cursor.sroa.0.1.i30114958, !dbg !33379
  %_215.not.i409 = icmp ult i64 %_140.i408, %main.i277, !dbg !33380
  %1499 = select i1 %_215.not.i409, i64 0, i64 %main.i277, !dbg !33380
  %main_cursor.sroa.0.2.i410 = sub nuw i64 %_140.i408, %1499, !dbg !33380
  %_58.i303 = icmp ult i64 %_80.i320, %..i6816, !dbg !32263
  br i1 %_58.i303, label %bb20.i304, label %bb15.i285.loopexit.loopexit, !dbg !32263

_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit: ; preds = %bb15.i285.loopexit
  %1500 = trunc i64 %main_cursor.sroa.0.1.i301.lcssa to i32, !dbg !33382
  %1501 = trunc i64 %ring_cursor.sroa.0.1.i300.lcssa to i32, !dbg !33384
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, !dbg !33385

_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit, %bb7.i
  %ring_cursor.sroa.0.0.i286.lcssa = phi i32 [ %_36.i279, %bb7.i ], [ %1501, %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit ], !dbg !31084
  %main_cursor.sroa.0.0.i287.lcssa = phi i32 [ %_35.i278, %bb7.i ], [ %1500, %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit ], !dbg !31081
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %left_prefix.i225, ptr noundef nonnull align 32 dereferenceable(32) %uniform_left.i260, i64 32, i1 false), !dbg !33385
  %1502 = getelementptr inbounds nuw i8, ptr %uniform_left.i260, i64 104, !dbg !33386
  %left_phase.i443 = load i32, ptr %1502, align 8, !dbg !33386, !noalias !31088, !noundef !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %right_prefix.i224, ptr noundef nonnull align 32 dereferenceable(32) %uniform_right.i259, i64 32, i1 false), !dbg !33387
  %1503 = getelementptr inbounds nuw i8, ptr %uniform_right.i259, i64 104, !dbg !33388
  %right_phase.i444 = load i32, ptr %1503, align 8, !dbg !33388, !noalias !31088, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_right.i259), !dbg !33389, !noalias !31088
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i260), !dbg !33390, !noalias !31088
  %1504 = getelementptr inbounds nuw i8, ptr %self, i64 1736, !dbg !33391
  %_230.1.i446 = load i64, ptr %1504, align 8, !dbg !33391, !alias.scope !31055, !noalias !33392, !noundef !12
  %_8.i6181 = icmp samesign ugt i64 %_230.1.i446, 7, !dbg !33393
  br i1 %_8.i6181, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6184, label %bb2.i6182, !dbg !33393, !prof !651

bb2.i6182:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_230.1.i446, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !33398, !noalias !33399
  unreachable, !dbg !33398

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6184: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit
  %1505 = getelementptr inbounds nuw i8, ptr %self, i64 1728, !dbg !33391
  %_230.0.i445 = load ptr, ptr %1505, align 8, !dbg !33391, !alias.scope !31055, !noalias !33392, !nonnull !12, !noundef !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_230.0.i445, ptr noundef nonnull align 32 dereferenceable(32) %left_prefix.i225, i64 32, i1 false), !dbg !33403
  %_231.0.i447 = load ptr, ptr %68, align 8, !dbg !33407, !alias.scope !31055, !noalias !33392, !nonnull !12, !noundef !12
  %_231.1.i448 = load i64, ptr %69, align 8, !dbg !33407, !alias.scope !31055, !noalias !33392, !noundef !12
  %1506 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i443), !dbg !33408
  br i1 %1506, label %bb2.i6934, label %bb6.i6929, !dbg !33408

bb6.i6929:                                        ; preds = %bb2.i6934, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6184
  %end_or_len.idx.i = shl nuw nsw i64 %_231.1.i448, 2, !dbg !33412
  %end_or_len.i = getelementptr inbounds nuw i8, ptr %_231.0.i447, i64 %end_or_len.idx.i, !dbg !33412
  %_293.i = icmp eq i64 %_231.1.i448, 0, !dbg !33416
  br i1 %_293.i, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i6930, !dbg !33419

bb2.i6934:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6184
  %bytes1.sroa.0.0.zext.i = and i32 %left_phase.i443, 255, !dbg !33420
  %bytes1.sroa.0.0.isplat.i = mul nuw i32 %bytes1.sroa.0.0.zext.i, 16843009, !dbg !33420
  %_5.i6935 = icmp eq i32 %left_phase.i443, %bytes1.sroa.0.0.isplat.i, !dbg !33421
  br i1 %_5.i6935, label %bb3.i6936, label %bb6.i6929, !dbg !33421

bb3.i6936:                                        ; preds = %bb2.i6934
  %bytes.sroa.0.0.extract.trunc.i = trunc i32 %left_phase.i443 to i8, !dbg !33422
  %1507 = shl nuw nsw i64 %_231.1.i448, 2, !dbg !33424
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_231.0.i447, i8 %bytes.sroa.0.0.extract.trunc.i, i64 %1507, i1 false), !dbg !33424, !alias.scope !33425, !noalias !31059
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, !dbg !33428

bb10.i6930:                                       ; preds = %bb6.i6929, %bb10.i6930
  %iter.sroa.0.04.i = phi ptr [ %_38.i6931, %bb10.i6930 ], [ %_231.0.i447, %bb6.i6929 ]
  %_38.i6931 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i, i64 4, !dbg !33429
  store i32 %left_phase.i443, ptr %iter.sroa.0.04.i, align 4, !dbg !33431, !alias.scope !33425, !noalias !31059
  %_29.i6932 = icmp eq ptr %_38.i6931, %end_or_len.i, !dbg !33416
  br i1 %_29.i6932, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i6930, !dbg !33419

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit: ; preds = %bb10.i6930, %bb6.i6929, %bb3.i6936
  %1508 = getelementptr inbounds nuw i8, ptr %self, i64 1936, !dbg !33432
  %_232.1.i450 = load i64, ptr %1508, align 8, !dbg !33432, !alias.scope !31057, !noalias !33433, !noundef !12
  %_8.i6176 = icmp samesign ugt i64 %_232.1.i450, 7, !dbg !33434
  br i1 %_8.i6176, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6179, label %bb2.i6177, !dbg !33434, !prof !651

bb2.i6177:                                        ; preds = %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_232.1.i450, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !33439, !noalias !33440
  unreachable, !dbg !33439

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6179: ; preds = %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit
  %1509 = getelementptr inbounds nuw i8, ptr %self, i64 1928, !dbg !33432
  %_232.0.i449 = load ptr, ptr %1509, align 8, !dbg !33432, !alias.scope !31057, !noalias !33433, !nonnull !12, !noundef !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_232.0.i449, ptr noundef nonnull align 32 dereferenceable(32) %right_prefix.i224, i64 32, i1 false), !dbg !33444
  %_233.0.i451 = load ptr, ptr %77, align 8, !dbg !33448, !alias.scope !31057, !noalias !33433, !nonnull !12, !noundef !12
  %_233.1.i452 = load i64, ptr %78, align 8, !dbg !33448, !alias.scope !31057, !noalias !33433, !noundef !12
  %1510 = tail call i1 @llvm.is.constant.i32(i32 %right_phase.i444), !dbg !33449
  br i1 %1510, label %bb2.i6947, label %bb6.i6938, !dbg !33449

bb6.i6938:                                        ; preds = %bb2.i6947, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6179
  %end_or_len.idx.i6939 = shl nuw nsw i64 %_233.1.i452, 2, !dbg !33452
  %end_or_len.i6940 = getelementptr inbounds nuw i8, ptr %_233.0.i451, i64 %end_or_len.idx.i6939, !dbg !33452
  %_293.i6941 = icmp eq i64 %_233.1.i452, 0, !dbg !33456
  br i1 %_293.i6941, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit6953, label %bb10.i6942, !dbg !33459

bb2.i6947:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6179
  %bytes1.sroa.0.0.zext.i6948 = and i32 %right_phase.i444, 255, !dbg !33460
  %bytes1.sroa.0.0.isplat.i6949 = mul nuw i32 %bytes1.sroa.0.0.zext.i6948, 16843009, !dbg !33460
  %_5.i6950 = icmp eq i32 %right_phase.i444, %bytes1.sroa.0.0.isplat.i6949, !dbg !33461
  br i1 %_5.i6950, label %bb3.i6951, label %bb6.i6938, !dbg !33461

bb3.i6951:                                        ; preds = %bb2.i6947
  %bytes.sroa.0.0.extract.trunc.i6952 = trunc i32 %right_phase.i444 to i8, !dbg !33462
  %1511 = shl nuw nsw i64 %_233.1.i452, 2, !dbg !33464
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_233.0.i451, i8 %bytes.sroa.0.0.extract.trunc.i6952, i64 %1511, i1 false), !dbg !33464, !alias.scope !33465, !noalias !31059
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit6953, !dbg !33468

bb10.i6942:                                       ; preds = %bb6.i6938, %bb10.i6942
  %iter.sroa.0.04.i6943 = phi ptr [ %_38.i6944, %bb10.i6942 ], [ %_233.0.i451, %bb6.i6938 ]
  %_38.i6944 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i6943, i64 4, !dbg !33469
  store i32 %right_phase.i444, ptr %iter.sroa.0.04.i6943, align 4, !dbg !33471, !alias.scope !33465, !noalias !31059
  %_29.i6945 = icmp eq ptr %_38.i6944, %end_or_len.i6940, !dbg !33456
  br i1 %_29.i6945, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit6953, label %bb10.i6942, !dbg !33459

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit6953: ; preds = %bb10.i6942, %bb6.i6938, %bb3.i6951
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_left.i269, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33) #31, !dbg !33472
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_right.i268, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34) #31, !dbg !33473
  store i32 %main_cursor.sroa.0.0.i287.lcssa, ptr %_35, align 4, !dbg !33382, !alias.scope !31059, !noalias !31083
  store i32 %ring_cursor.sroa.0.0.i286.lcssa, ptr %87, align 4, !dbg !33384, !alias.scope !31059, !noalias !31083
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i261), !dbg !33474, !noalias !31088
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i262), !dbg !33475, !noalias !31088
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, !dbg !31052

bb6.i:                                            ; preds = %bb5.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !33476), !dbg !33479
  tail call void @llvm.experimental.noalias.scope.decl(metadata !33480), !dbg !33479
  tail call void @llvm.experimental.noalias.scope.decl(metadata !33482), !dbg !33479
  tail call void @llvm.experimental.noalias.scope.decl(metadata !33484), !dbg !33479
  tail call void @llvm.experimental.noalias.scope.decl(metadata !33486), !dbg !33479
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_left.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #31, !dbg !33488
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_right.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #31, !dbg !33492
  %1512 = load i8, ptr %83, align 32, !dbg !33494, !range !17, !alias.scope !33476, !noalias !33498, !noundef !12
  %1513 = load i8, ptr %84, align 1, !dbg !33501, !range !17, !alias.scope !33476, !noalias !33498, !noundef !12
  %ring.i = load i64, ptr %85, align 8, !dbg !33503, !alias.scope !33480, !noalias !33505, !noundef !12
  %main.i = load i64, ptr %86, align 8, !dbg !33506, !alias.scope !33480, !noalias !33505, !noundef !12
  %_35.i = load i32, ptr %_35, align 4, !dbg !33508, !alias.scope !33486, !noalias !33510, !noundef !12
  %_36.i = load i32, ptr %87, align 4, !dbg !33511, !alias.scope !33486, !noalias !33510, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i), !dbg !33513, !noalias !33515
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i, i8 0, i64 1024, i1 false), !noalias !33515
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i), !dbg !33516, !noalias !33515
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i, i8 0, i64 1024, i1 false), !noalias !33515
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i), !dbg !33518, !noalias !33515
; call <true_peak_limiter::UniformHot<wide::f32x8_::f32x8>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_(ptr noalias noundef align 32 captures(none) dereferenceable(128) %uniform_left.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33, i64 %ring.i, i64 %main.i) #31, !dbg !33520
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_right.i), !dbg !33521, !noalias !33515
  %_32.val6482 = load i64, ptr %85, align 8, !dbg !33523, !noundef !12
  %_32.val6483 = load i64, ptr %86, align 8, !dbg !33523, !noundef !12
; call <true_peak_limiter::UniformHot<wide::f32x8_::f32x8>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_(ptr noalias noundef align 32 captures(none) dereferenceable(128) %uniform_right.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34, i64 %_32.val6482, i64 %_32.val6483) #31, !dbg !33523
  %1514 = add nuw nsw i64 %frames, 31, !dbg !33524
  %yield_count.sroa.0.0.i.i6957 = lshr i64 %1514, 5, !dbg !33524
  %_162.not.i15373 = icmp eq i64 %yield_count.sroa.0.0.i.i6957, 0, !dbg !33531
  br i1 %_162.not.i15373, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, label %bb50.i.lr.ph, !dbg !33531

bb50.i.lr.ph:                                     ; preds = %bb6.i
  %1515 = zext i32 %_36.i to i64, !dbg !33511
  %1516 = zext i32 %_35.i to i64, !dbg !33508
  %_32.i = trunc nuw i8 %1513 to i1, !dbg !33501
  %_31.i = trunc nuw i8 %1512 to i1, !dbg !33494
  %1517 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !33540
  %1518 = bitcast <8 x float> %1517 to <8 x i32>, !dbg !33546
  %1519 = xor <8 x i32> %1518, splat (i32 -1), !dbg !33552
  %history.i209.i.sroa.10.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 32
  %history.i209.i.sroa.13.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 64
  %history.i209.i.sroa.16.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 96
  %history.i209.i.sroa.19.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 128
  %history.i209.i.sroa.22.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 160
  %history.i209.i.sroa.25.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 192
  %history.i209.i.sroa.29.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 224
  %history.i209.i.sroa.32.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 256
  %history.i209.i.sroa.35.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 288
  %history.i209.i.sroa.38.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 320
  %history.i209.i.sroa.41.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 352
  %1520 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %1521 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %1522 = getelementptr inbounds nuw i8, ptr %self, i64 96
  %row12.i.i.i220.i = getelementptr inbounds nuw i8, ptr %self, i64 128
  %1523 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %1524 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %1525 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %row13.i.i.i221.i = getelementptr inbounds nuw i8, ptr %self, i64 256
  %1526 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %1527 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %1528 = getelementptr inbounds nuw i8, ptr %self, i64 352
  %row14.i.i.i222.i = getelementptr inbounds nuw i8, ptr %self, i64 384
  %1529 = getelementptr inbounds nuw i8, ptr %self, i64 416
  %1530 = getelementptr inbounds nuw i8, ptr %self, i64 448
  %1531 = getelementptr inbounds nuw i8, ptr %self, i64 480
  %row15.i.i.i223.i = getelementptr inbounds nuw i8, ptr %self, i64 512
  %1532 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %1533 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %1534 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %row16.i.i.i224.i = getelementptr inbounds nuw i8, ptr %self, i64 640
  %1535 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %1536 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %1537 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %row17.i.i.i225.i = getelementptr inbounds nuw i8, ptr %self, i64 768
  %1538 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %1539 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %1540 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %row18.i.i.i226.i = getelementptr inbounds nuw i8, ptr %self, i64 896
  %1541 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %1542 = getelementptr inbounds nuw i8, ptr %self, i64 960
  %1543 = getelementptr inbounds nuw i8, ptr %self, i64 992
  %row19.i.i.i227.i = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %1544 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %1545 = getelementptr inbounds nuw i8, ptr %self, i64 1088
  %1546 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %row20.i.i.i228.i = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %1547 = getelementptr inbounds nuw i8, ptr %self, i64 1184
  %1548 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %1549 = getelementptr inbounds nuw i8, ptr %self, i64 1248
  %row21.i.i.i229.i = getelementptr inbounds nuw i8, ptr %self, i64 1280
  %1550 = getelementptr inbounds nuw i8, ptr %self, i64 1312
  %1551 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %1552 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %row22.i.i.i230.i = getelementptr inbounds nuw i8, ptr %self, i64 1408
  %1553 = getelementptr inbounds nuw i8, ptr %self, i64 1440
  %1554 = getelementptr inbounds nuw i8, ptr %self, i64 1472
  %1555 = getelementptr inbounds nuw i8, ptr %self, i64 1504
  %history.i.i.sroa.10.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 32
  %history.i.i.sroa.13.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 64
  %history.i.i.sroa.16.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 96
  %history.i.i.sroa.19.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 128
  %history.i.i.sroa.22.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 160
  %history.i.i.sroa.25.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 192
  %history.i.i.sroa.29.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 224
  %history.i.i.sroa.32.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 256
  %history.i.i.sroa.35.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 288
  %history.i.i.sroa.38.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 320
  %history.i.i.sroa.41.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 352
  %1556 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 80
  %_65.i.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 88
  %_65.i.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 96
  %1557 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 80
  %_66.i.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 88
  %_66.i.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 96
  %_109.i = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 384
  %_110.i = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 512
  %_114.i = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 384
  %_115.i = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 512
  %1558 = select i1 %_31.i, <8 x i32> %1518, <8 x i32> %1519
  %1559 = icmp slt <8 x i32> %1558, zeroinitializer
  %1560 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 32
  %1561 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 40
  %_22.i301.i = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 104
  %1562 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 48
  %1563 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 56
  %1564 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 672
  %1565 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 704
  %1566 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 640
  %1567 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 72
  %1568 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 64
  %1569 = select i1 %_32.i, <8 x i32> %1518, <8 x i32> %1519
  %1570 = icmp slt <8 x i32> %1569, zeroinitializer
  %1571 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 32
  %1572 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 40
  %_22.i.i = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 104
  %1573 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 48
  %1574 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 56
  %1575 = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 672
  %1576 = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 704
  %1577 = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 640
  %1578 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 72
  %1579 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 64
  %hot_left.i.promoted = load <8 x float>, ptr %hot_left.i, align 32
  %history.i209.i.sroa.10.0.hot_left.i.sroa_idx.promoted = load <8 x float>, ptr %history.i209.i.sroa.10.0.hot_left.i.sroa_idx, align 32
  %history.i209.i.sroa.13.0.hot_left.i.sroa_idx.promoted = load <8 x float>, ptr %history.i209.i.sroa.13.0.hot_left.i.sroa_idx, align 32
  %history.i209.i.sroa.16.0.hot_left.i.sroa_idx.promoted = load <8 x float>, ptr %history.i209.i.sroa.16.0.hot_left.i.sroa_idx, align 32
  %history.i209.i.sroa.19.0.hot_left.i.sroa_idx.promoted = load <8 x float>, ptr %history.i209.i.sroa.19.0.hot_left.i.sroa_idx, align 32
  %history.i209.i.sroa.22.0.hot_left.i.sroa_idx.promoted = load <8 x float>, ptr %history.i209.i.sroa.22.0.hot_left.i.sroa_idx, align 32
  %history.i209.i.sroa.25.0.hot_left.i.sroa_idx.promoted = load <8 x float>, ptr %history.i209.i.sroa.25.0.hot_left.i.sroa_idx, align 32
  %history.i209.i.sroa.29.0.hot_left.i.sroa_idx.promoted = load <8 x float>, ptr %history.i209.i.sroa.29.0.hot_left.i.sroa_idx, align 32
  %history.i209.i.sroa.32.0.hot_left.i.sroa_idx.promoted = load <8 x float>, ptr %history.i209.i.sroa.32.0.hot_left.i.sroa_idx, align 32
  %history.i209.i.sroa.35.0.hot_left.i.sroa_idx.promoted = load <8 x float>, ptr %history.i209.i.sroa.35.0.hot_left.i.sroa_idx, align 32
  %history.i209.i.sroa.38.0.hot_left.i.sroa_idx.promoted = load <8 x float>, ptr %history.i209.i.sroa.38.0.hot_left.i.sroa_idx, align 32
  %history.i209.i.sroa.41.0.hot_left.i.sroa_idx.promoted = load <8 x float>, ptr %history.i209.i.sroa.41.0.hot_left.i.sroa_idx, align 32
  %hot_right.i.promoted = load <8 x float>, ptr %hot_right.i, align 32
  %history.i.i.sroa.10.0.hot_right.i.sroa_idx.promoted = load <8 x float>, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32
  %history.i.i.sroa.13.0.hot_right.i.sroa_idx.promoted = load <8 x float>, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32
  %history.i.i.sroa.16.0.hot_right.i.sroa_idx.promoted = load <8 x float>, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 32
  %history.i.i.sroa.19.0.hot_right.i.sroa_idx.promoted = load <8 x float>, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 32
  %history.i.i.sroa.22.0.hot_right.i.sroa_idx.promoted = load <8 x float>, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 32
  %history.i.i.sroa.25.0.hot_right.i.sroa_idx.promoted = load <8 x float>, ptr %history.i.i.sroa.25.0.hot_right.i.sroa_idx, align 32
  %history.i.i.sroa.29.0.hot_right.i.sroa_idx.promoted = load <8 x float>, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 32
  %history.i.i.sroa.32.0.hot_right.i.sroa_idx.promoted = load <8 x float>, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 32
  %history.i.i.sroa.35.0.hot_right.i.sroa_idx.promoted = load <8 x float>, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 32
  %history.i.i.sroa.38.0.hot_right.i.sroa_idx.promoted = load <8 x float>, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 32
  %history.i.i.sroa.41.0.hot_right.i.sroa_idx.promoted = load <8 x float>, ptr %history.i.i.sroa.41.0.hot_right.i.sroa_idx, align 32
  %uniform_left.i.promoted = load <8 x float>, ptr %uniform_left.i, align 1
  %uniform_right.i.promoted = load <8 x float>, ptr %uniform_right.i, align 1
  %_22.i301.i.promoted = load i32, ptr %_22.i301.i, align 4
  %.promoted20588 = load <8 x float>, ptr %1564, align 32
  %_22.i.i.promoted = load i32, ptr %_22.i.i, align 4
  %.promoted20613 = load <8 x float>, ptr %1575, align 32
  br label %bb50.i, !dbg !33531

bb19.i.bb15.i.loopexit_crit_edge:                 ; preds = %bb32.i
  store <8 x float> %.lcssa1511915266, ptr %1564, align 32
  store <8 x float> %.lcssa1517615338, ptr %1575, align 32
  br label %bb15.i.loopexit, !dbg !33554

bb15.i.loopexit:                                  ; preds = %bb19.i.bb15.i.loopexit_crit_edge, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i
  %.lcssa1517615338.lcssa20614 = phi <8 x float> [ %.lcssa1517615338, %bb19.i.bb15.i.loopexit_crit_edge ], [ %.lcssa1517615338.lcssa20615, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ]
  %storemerge.i1311.lcssa1514615302.lcssa20591 = phi i32 [ %storemerge.i1311.lcssa1514615302, %bb19.i.bb15.i.loopexit_crit_edge ], [ %storemerge.i1311.lcssa1514615302.lcssa20592, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ]
  %.lcssa1511915266.lcssa20589 = phi <8 x float> [ %.lcssa1511915266, %bb19.i.bb15.i.loopexit_crit_edge ], [ %.lcssa1511915266.lcssa20590, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ]
  %storemerge.i1346.lcssa1508915230.lcssa20566 = phi i32 [ %storemerge.i1346.lcssa1508915230, %bb19.i.bb15.i.loopexit_crit_edge ], [ %storemerge.i1346.lcssa1508915230.lcssa20567, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ]
  %minimum.i.i.sroa.0.015045.lcssa15201.lcssa = phi <8 x float> [ %minimum.i.i.sroa.0.015045.lcssa, %bb19.i.bb15.i.loopexit_crit_edge ], [ %minimum.i.i.sroa.0.015045.lcssa15201.lcssa20545, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ]
  %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa = phi <8 x float> [ %minimum.i281.i.sroa.0.015031.lcssa, %bb19.i.bb15.i.loopexit_crit_edge ], [ %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa20524, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ]
  %ring_cursor.sroa.0.1.i.lcssa = phi i64 [ %ring_cursor.sroa.0.2.i, %bb19.i.bb15.i.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i15374, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ], !dbg !33558
  %main_cursor.sroa.0.1.i.lcssa = phi i64 [ %main_cursor.sroa.0.2.i, %bb19.i.bb15.i.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i15375, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ], !dbg !33559
  %_162.not.i = icmp eq i64 %1581, 0, !dbg !33531
  %indvars.iv.next17557 = add nsw i64 %indvars.iv17556, -32, !dbg !33531
  br i1 %_162.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit, label %bb50.i, !dbg !33531

bb50.i:                                           ; preds = %bb50.i.lr.ph, %bb15.i.loopexit
  %.lcssa1517615338.lcssa20615 = phi <8 x float> [ %.promoted20613, %bb50.i.lr.ph ], [ %.lcssa1517615338.lcssa20614, %bb15.i.loopexit ]
  %storemerge.i1311.lcssa1514615302.lcssa20592 = phi i32 [ %_22.i.i.promoted, %bb50.i.lr.ph ], [ %storemerge.i1311.lcssa1514615302.lcssa20591, %bb15.i.loopexit ]
  %.lcssa1511915266.lcssa20590 = phi <8 x float> [ %.promoted20588, %bb50.i.lr.ph ], [ %.lcssa1511915266.lcssa20589, %bb15.i.loopexit ]
  %storemerge.i1346.lcssa1508915230.lcssa20567 = phi i32 [ %_22.i301.i.promoted, %bb50.i.lr.ph ], [ %storemerge.i1346.lcssa1508915230.lcssa20566, %bb15.i.loopexit ]
  %minimum.i.i.sroa.0.015045.lcssa15201.lcssa20545 = phi <8 x float> [ %uniform_right.i.promoted, %bb50.i.lr.ph ], [ %minimum.i.i.sroa.0.015045.lcssa15201.lcssa, %bb15.i.loopexit ]
  %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa20524 = phi <8 x float> [ %uniform_left.i.promoted, %bb50.i.lr.ph ], [ %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.41.sroa.0.0.lcssa20523 = phi <8 x float> [ %history.i.i.sroa.41.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.41.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.38.sroa.0.0.lcssa20522 = phi <8 x float> [ %history.i.i.sroa.38.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.38.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.35.sroa.0.0.lcssa20521 = phi <8 x float> [ %history.i.i.sroa.35.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.35.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.32.sroa.0.0.lcssa20520 = phi <8 x float> [ %history.i.i.sroa.32.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.32.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.29.sroa.0.0.lcssa20519 = phi <8 x float> [ %history.i.i.sroa.29.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.29.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.25.sroa.0.0.lcssa20518 = phi <8 x float> [ %history.i.i.sroa.25.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.25.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.22.sroa.0.0.lcssa20517 = phi <8 x float> [ %history.i.i.sroa.22.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.22.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.19.sroa.0.0.lcssa20516 = phi <8 x float> [ %history.i.i.sroa.19.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.19.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.16.sroa.0.0.lcssa20515 = phi <8 x float> [ %history.i.i.sroa.16.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.16.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.13.sroa.0.0.lcssa20494 = phi <8 x float> [ %history.i.i.sroa.13.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.13.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.10.sroa.0.0.lcssa20473 = phi <8 x float> [ %history.i.i.sroa.10.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.10.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.0.0.lcssa20452 = phi <8 x float> [ %hot_right.i.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i209.i.sroa.41.sroa.0.0.lcssa20451 = phi <8 x float> [ %history.i209.i.sroa.41.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i209.i.sroa.41.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i209.i.sroa.38.sroa.0.0.lcssa20450 = phi <8 x float> [ %history.i209.i.sroa.38.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i209.i.sroa.38.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i209.i.sroa.35.sroa.0.0.lcssa20449 = phi <8 x float> [ %history.i209.i.sroa.35.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i209.i.sroa.35.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i209.i.sroa.32.sroa.0.0.lcssa20448 = phi <8 x float> [ %history.i209.i.sroa.32.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i209.i.sroa.32.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i209.i.sroa.29.sroa.0.0.lcssa20447 = phi <8 x float> [ %history.i209.i.sroa.29.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i209.i.sroa.29.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i209.i.sroa.25.sroa.0.0.lcssa20446 = phi <8 x float> [ %history.i209.i.sroa.25.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i209.i.sroa.25.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i209.i.sroa.22.sroa.0.0.lcssa20445 = phi <8 x float> [ %history.i209.i.sroa.22.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i209.i.sroa.22.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i209.i.sroa.19.sroa.0.0.lcssa20444 = phi <8 x float> [ %history.i209.i.sroa.19.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i209.i.sroa.19.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i209.i.sroa.16.sroa.0.0.lcssa20443 = phi <8 x float> [ %history.i209.i.sroa.16.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i209.i.sroa.16.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i209.i.sroa.13.sroa.0.0.lcssa20422 = phi <8 x float> [ %history.i209.i.sroa.13.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i209.i.sroa.13.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i209.i.sroa.10.sroa.0.0.lcssa20401 = phi <8 x float> [ %history.i209.i.sroa.10.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i209.i.sroa.10.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i209.i.sroa.0.0.lcssa20380 = phi <8 x float> [ %hot_left.i.promoted, %bb50.i.lr.ph ], [ %history.i209.i.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %indvars.iv17556 = phi i64 [ %frames, %bb50.i.lr.ph ], [ %indvars.iv.next17557, %bb15.i.loopexit ]
  %iter4.sroa.0.0.i15377 = phi i64 [ %yield_count.sroa.0.0.i.i6957, %bb50.i.lr.ph ], [ %1581, %bb15.i.loopexit ]
  %iter3.sroa.0.0.i15376 = phi i64 [ 0, %bb50.i.lr.ph ], [ %1580, %bb15.i.loopexit ]
  %main_cursor.sroa.0.0.i15375 = phi i64 [ %1516, %bb50.i.lr.ph ], [ %main_cursor.sroa.0.1.i.lcssa, %bb15.i.loopexit ]
  %ring_cursor.sroa.0.0.i15374 = phi i64 [ %1515, %bb50.i.lr.ph ], [ %ring_cursor.sroa.0.1.i.lcssa, %bb15.i.loopexit ]
  %umin17584 = call i64 @llvm.umin.i64(i64 %indvars.iv17556, i64 32), !dbg !33560
  %umax17564 = call i64 @llvm.umax.i64(i64 %umin17584, i64 1), !dbg !33560
  %1580 = add nuw nsw i64 %iter3.sroa.0.0.i15376, 32, !dbg !33560
  %1581 = add nsw i64 %iter4.sroa.0.0.i15377, -1, !dbg !33564
  %_46.i = sub nsw i64 %frames, %iter3.sroa.0.0.i15376, !dbg !33565
  %..i6958 = tail call noundef i64 @llvm.umin.i64(i64 %_46.i, i64 32), !dbg !33566
  %_20.i212.i14971.not = icmp eq i64 %frames, %iter3.sroa.0.0.i15376, !dbg !33570
  br i1 %_20.i212.i14971.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit244.i, label %bb5.i213.i.lr.ph, !dbg !33575

bb5.i213.i.lr.ph:                                 ; preds = %bb50.i
  %_5.i4857 = load <8 x float>, ptr %self, align 32
  %_14.i.i.i174.i.sroa.0.0.copyload = load <8 x float>, ptr %1520, align 32
  %_17.i.i.i171.i.sroa.0.0.copyload = load <8 x float>, ptr %1521, align 32
  %_20.i.i.i168.i.sroa.0.0.copyload = load <8 x float>, ptr %1522, align 32
  %_25.i.i.i164.i.sroa.0.0.copyload = load <8 x float>, ptr %row12.i.i.i220.i, align 32
  %_28.i.i.i161.i.sroa.0.0.copyload = load <8 x float>, ptr %1523, align 32
  %_31.i.i.i158.i.sroa.0.0.copyload = load <8 x float>, ptr %1524, align 32
  %_34.i.i.i155.i.sroa.0.0.copyload = load <8 x float>, ptr %1525, align 32
  %_39.i.i.i151.i.sroa.0.0.copyload = load <8 x float>, ptr %row13.i.i.i221.i, align 32
  %_42.i.i.i148.i.sroa.0.0.copyload = load <8 x float>, ptr %1526, align 32
  %_45.i.i.i145.i.sroa.0.0.copyload = load <8 x float>, ptr %1527, align 32
  %_48.i.i.i142.i.sroa.0.0.copyload = load <8 x float>, ptr %1528, align 32
  %_53.i.i.i138.i.sroa.0.0.copyload = load <8 x float>, ptr %row14.i.i.i222.i, align 32
  %_56.i.i.i135.i.sroa.0.0.copyload = load <8 x float>, ptr %1529, align 32
  %_59.i.i.i132.i.sroa.0.0.copyload = load <8 x float>, ptr %1530, align 32
  %_62.i.i.i129.i.sroa.0.0.copyload = load <8 x float>, ptr %1531, align 32
  %_67.i.i.i125.i.sroa.0.0.copyload = load <8 x float>, ptr %row15.i.i.i223.i, align 32
  %_70.i.i.i122.i.sroa.0.0.copyload = load <8 x float>, ptr %1532, align 32
  %_73.i.i.i119.i.sroa.0.0.copyload = load <8 x float>, ptr %1533, align 32
  %_76.i.i.i116.i.sroa.0.0.copyload = load <8 x float>, ptr %1534, align 32
  %_81.i.i.i112.i.sroa.0.0.copyload = load <8 x float>, ptr %row16.i.i.i224.i, align 32
  %_84.i.i.i109.i.sroa.0.0.copyload = load <8 x float>, ptr %1535, align 32
  %_87.i.i.i106.i.sroa.0.0.copyload = load <8 x float>, ptr %1536, align 32
  %_90.i.i.i103.i.sroa.0.0.copyload = load <8 x float>, ptr %1537, align 32
  %_95.i.i.i99.i.sroa.0.0.copyload = load <8 x float>, ptr %row17.i.i.i225.i, align 32
  %_98.i.i.i96.i.sroa.0.0.copyload = load <8 x float>, ptr %1538, align 32
  %_101.i.i.i93.i.sroa.0.0.copyload = load <8 x float>, ptr %1539, align 32
  %_104.i.i.i90.i.sroa.0.0.copyload = load <8 x float>, ptr %1540, align 32
  %_109.i.i.i86.i.sroa.0.0.copyload = load <8 x float>, ptr %row18.i.i.i226.i, align 32
  %_112.i.i.i83.i.sroa.0.0.copyload = load <8 x float>, ptr %1541, align 32
  %_115.i.i.i80.i.sroa.0.0.copyload = load <8 x float>, ptr %1542, align 32
  %_118.i.i.i77.i.sroa.0.0.copyload = load <8 x float>, ptr %1543, align 32
  %_123.i.i.i73.i.sroa.0.0.copyload = load <8 x float>, ptr %row19.i.i.i227.i, align 32
  %_126.i.i.i70.i.sroa.0.0.copyload = load <8 x float>, ptr %1544, align 32
  %_129.i.i.i67.i.sroa.0.0.copyload = load <8 x float>, ptr %1545, align 32
  %_132.i.i.i64.i.sroa.0.0.copyload = load <8 x float>, ptr %1546, align 32
  %_137.i.i.i60.i.sroa.0.0.copyload = load <8 x float>, ptr %row20.i.i.i228.i, align 32
  %_140.i.i.i57.i.sroa.0.0.copyload = load <8 x float>, ptr %1547, align 32
  %_143.i.i.i54.i.sroa.0.0.copyload = load <8 x float>, ptr %1548, align 32
  %_146.i.i.i51.i.sroa.0.0.copyload = load <8 x float>, ptr %1549, align 32
  %_151.i.i.i47.i.sroa.0.0.copyload = load <8 x float>, ptr %row21.i.i.i229.i, align 32
  %_154.i.i.i44.i.sroa.0.0.copyload = load <8 x float>, ptr %1550, align 32
  %_157.i.i.i41.i.sroa.0.0.copyload = load <8 x float>, ptr %1551, align 32
  %_160.i.i.i38.i.sroa.0.0.copyload = load <8 x float>, ptr %1552, align 32
  %_165.i.i.i34.i.sroa.0.0.copyload = load <8 x float>, ptr %row22.i.i.i230.i, align 32
  %_168.i.i.i31.i.sroa.0.0.copyload = load <8 x float>, ptr %1553, align 32
  %_171.i.i.i28.i.sroa.0.0.copyload = load <8 x float>, ptr %1554, align 32
  %_174.i.i.i25.i.sroa.0.0.copyload = load <8 x float>, ptr %1555, align 32
  br label %bb5.i213.i, !dbg !33575

bb5.i213.i:                                       ; preds = %bb5.i213.i.lr.ph, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231
  %iter.sroa.0.0.i211.i14983 = phi i64 [ 0, %bb5.i213.i.lr.ph ], [ %1582, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231 ]
  %history.i209.i.sroa.10.sroa.0.014982 = phi <8 x float> [ %history.i209.i.sroa.10.sroa.0.0.lcssa20401, %bb5.i213.i.lr.ph ], [ %history.i209.i.sroa.0.014972, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231 ]
  %history.i209.i.sroa.13.sroa.0.014981 = phi <8 x float> [ %history.i209.i.sroa.13.sroa.0.0.lcssa20422, %bb5.i213.i.lr.ph ], [ %history.i209.i.sroa.10.sroa.0.014982, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231 ]
  %history.i209.i.sroa.16.sroa.0.014980 = phi <8 x float> [ %history.i209.i.sroa.16.sroa.0.0.lcssa20443, %bb5.i213.i.lr.ph ], [ %history.i209.i.sroa.13.sroa.0.014981, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231 ]
  %history.i209.i.sroa.19.sroa.0.014979 = phi <8 x float> [ %history.i209.i.sroa.19.sroa.0.0.lcssa20444, %bb5.i213.i.lr.ph ], [ %history.i209.i.sroa.16.sroa.0.014980, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231 ]
  %history.i209.i.sroa.22.sroa.0.014978 = phi <8 x float> [ %history.i209.i.sroa.22.sroa.0.0.lcssa20445, %bb5.i213.i.lr.ph ], [ %history.i209.i.sroa.19.sroa.0.014979, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231 ]
  %history.i209.i.sroa.38.sroa.0.014977 = phi <8 x float> [ %history.i209.i.sroa.38.sroa.0.0.lcssa20450, %bb5.i213.i.lr.ph ], [ %history.i209.i.sroa.35.sroa.0.014976, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231 ]
  %history.i209.i.sroa.35.sroa.0.014976 = phi <8 x float> [ %history.i209.i.sroa.35.sroa.0.0.lcssa20449, %bb5.i213.i.lr.ph ], [ %history.i209.i.sroa.32.sroa.0.014975, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231 ]
  %history.i209.i.sroa.32.sroa.0.014975 = phi <8 x float> [ %history.i209.i.sroa.32.sroa.0.0.lcssa20448, %bb5.i213.i.lr.ph ], [ %history.i209.i.sroa.29.sroa.0.014974, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231 ]
  %history.i209.i.sroa.29.sroa.0.014974 = phi <8 x float> [ %history.i209.i.sroa.29.sroa.0.0.lcssa20447, %bb5.i213.i.lr.ph ], [ %history.i209.i.sroa.25.sroa.0.014973, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231 ]
  %history.i209.i.sroa.25.sroa.0.014973 = phi <8 x float> [ %history.i209.i.sroa.25.sroa.0.0.lcssa20446, %bb5.i213.i.lr.ph ], [ %history.i209.i.sroa.22.sroa.0.014978, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231 ]
  %history.i209.i.sroa.0.014972 = phi <8 x float> [ %history.i209.i.sroa.0.0.lcssa20380, %bb5.i213.i.lr.ph ], [ %lanes.i5563.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231 ]
  %1582 = add nuw nsw i64 %iter.sroa.0.0.i211.i14983, 1, !dbg !33576
  %_11.i214.i = add nuw nsw i64 %iter.sroa.0.0.i211.i14983, %iter3.sroa.0.0.i15376, !dbg !33579
  %base.i215.i = shl i64 %_11.i214.i, 3, !dbg !33579
  %_24.i216.i = icmp samesign ugt i64 %base.i215.i, %left_io.1, !dbg !33580
  br i1 %_24.i216.i, label %bb7.i243.i, label %bb8.i217.i, !dbg !33580, !prof !639

bb8.i217.i:                                       ; preds = %bb5.i213.i
  %_27.i218.i = sub nuw nsw i64 %left_io.1, %base.i215.i, !dbg !33583
  %_8.i5566 = icmp samesign ugt i64 %_27.i218.i, 7, !dbg !33584
  br i1 %_8.i5566, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231, label %bb2.i5567, !dbg !33584, !prof !651

bb2.i5567:                                        ; preds = %bb8.i217.i
  store <8 x float> %history.i209.i.sroa.0.0.lcssa20380, ptr %hot_left.i, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.10.sroa.0.0.lcssa20401, ptr %history.i209.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.13.sroa.0.0.lcssa20422, ptr %history.i209.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i.i.sroa.0.0.lcssa20452, ptr %hot_right.i, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa20473, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa20494, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa20524, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015045.lcssa15201.lcssa20545, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1346.lcssa1508915230.lcssa20567, ptr %_22.i301.i, align 4
  store i32 %storemerge.i1311.lcssa1514615302.lcssa20592, ptr %_22.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_27.i218.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !33592, !noalias !33593
  unreachable, !dbg !33592

bb7.i243.i:                                       ; preds = %bb5.i213.i
  store <8 x float> %history.i209.i.sroa.0.0.lcssa20380, ptr %hot_left.i, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.10.sroa.0.0.lcssa20401, ptr %history.i209.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.13.sroa.0.0.lcssa20422, ptr %history.i209.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i.i.sroa.0.0.lcssa20452, ptr %hot_right.i, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa20473, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa20494, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa20524, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015045.lcssa15201.lcssa20545, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1346.lcssa1508915230.lcssa20567, ptr %_22.i301.i, align 4
  store i32 %storemerge.i1311.lcssa1514615302.lcssa20592, ptr %_22.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i215.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_fc26f793d85338b5649d38df0c19e7e0) #30, !dbg !33600, !noalias !33601
  unreachable, !dbg !33600

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231: ; preds = %bb8.i217.i
  %_31.i219.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %base.i215.i, !dbg !33602
  %lanes.i5563.sroa.0.0.copyload = load <8 x float>, ptr %_31.i219.i, align 4, !dbg !33604, !alias.scope !33608, !noalias !33612
  %1583 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i209.i.sroa.22.sroa.0.014978), !dbg !33614
  %1584 = fmul <8 x float> %lanes.i5563.sroa.0.0.copyload, %_5.i4857, !dbg !33621
  %1585 = fadd <8 x float> %1584, zeroinitializer, !dbg !33627
  %1586 = fmul <8 x float> %lanes.i5563.sroa.0.0.copyload, %_14.i.i.i174.i.sroa.0.0.copyload, !dbg !33632
  %1587 = fadd <8 x float> %1586, zeroinitializer, !dbg !33637
  %1588 = fmul <8 x float> %lanes.i5563.sroa.0.0.copyload, %_17.i.i.i171.i.sroa.0.0.copyload, !dbg !33642
  %1589 = fadd <8 x float> %1588, zeroinitializer, !dbg !33647
  %1590 = fmul <8 x float> %lanes.i5563.sroa.0.0.copyload, %_20.i.i.i168.i.sroa.0.0.copyload, !dbg !33652
  %1591 = fadd <8 x float> %1590, zeroinitializer, !dbg !33657
  %1592 = fmul <8 x float> %history.i209.i.sroa.0.014972, %_25.i.i.i164.i.sroa.0.0.copyload, !dbg !33662
  %1593 = fadd <8 x float> %1585, %1592, !dbg !33667
  %1594 = fmul <8 x float> %history.i209.i.sroa.0.014972, %_28.i.i.i161.i.sroa.0.0.copyload, !dbg !33672
  %1595 = fadd <8 x float> %1587, %1594, !dbg !33677
  %1596 = fmul <8 x float> %history.i209.i.sroa.0.014972, %_31.i.i.i158.i.sroa.0.0.copyload, !dbg !33682
  %1597 = fadd <8 x float> %1589, %1596, !dbg !33687
  %1598 = fmul <8 x float> %history.i209.i.sroa.0.014972, %_34.i.i.i155.i.sroa.0.0.copyload, !dbg !33692
  %1599 = fadd <8 x float> %1591, %1598, !dbg !33697
  %1600 = fmul <8 x float> %history.i209.i.sroa.10.sroa.0.014982, %_39.i.i.i151.i.sroa.0.0.copyload, !dbg !33702
  %1601 = fadd <8 x float> %1593, %1600, !dbg !33707
  %1602 = fmul <8 x float> %history.i209.i.sroa.10.sroa.0.014982, %_42.i.i.i148.i.sroa.0.0.copyload, !dbg !33712
  %1603 = fadd <8 x float> %1595, %1602, !dbg !33717
  %1604 = fmul <8 x float> %history.i209.i.sroa.10.sroa.0.014982, %_45.i.i.i145.i.sroa.0.0.copyload, !dbg !33722
  %1605 = fadd <8 x float> %1597, %1604, !dbg !33727
  %1606 = fmul <8 x float> %history.i209.i.sroa.10.sroa.0.014982, %_48.i.i.i142.i.sroa.0.0.copyload, !dbg !33732
  %1607 = fadd <8 x float> %1599, %1606, !dbg !33737
  %1608 = fmul <8 x float> %history.i209.i.sroa.13.sroa.0.014981, %_53.i.i.i138.i.sroa.0.0.copyload, !dbg !33742
  %1609 = fadd <8 x float> %1601, %1608, !dbg !33747
  %1610 = fmul <8 x float> %history.i209.i.sroa.13.sroa.0.014981, %_56.i.i.i135.i.sroa.0.0.copyload, !dbg !33752
  %1611 = fadd <8 x float> %1603, %1610, !dbg !33757
  %1612 = fmul <8 x float> %history.i209.i.sroa.13.sroa.0.014981, %_59.i.i.i132.i.sroa.0.0.copyload, !dbg !33762
  %1613 = fadd <8 x float> %1605, %1612, !dbg !33767
  %1614 = fmul <8 x float> %history.i209.i.sroa.13.sroa.0.014981, %_62.i.i.i129.i.sroa.0.0.copyload, !dbg !33772
  %1615 = fadd <8 x float> %1607, %1614, !dbg !33777
  %1616 = fmul <8 x float> %history.i209.i.sroa.16.sroa.0.014980, %_67.i.i.i125.i.sroa.0.0.copyload, !dbg !33782
  %1617 = fadd <8 x float> %1609, %1616, !dbg !33787
  %1618 = fmul <8 x float> %history.i209.i.sroa.16.sroa.0.014980, %_70.i.i.i122.i.sroa.0.0.copyload, !dbg !33792
  %1619 = fadd <8 x float> %1611, %1618, !dbg !33797
  %1620 = fmul <8 x float> %history.i209.i.sroa.16.sroa.0.014980, %_73.i.i.i119.i.sroa.0.0.copyload, !dbg !33802
  %1621 = fadd <8 x float> %1613, %1620, !dbg !33807
  %1622 = fmul <8 x float> %history.i209.i.sroa.16.sroa.0.014980, %_76.i.i.i116.i.sroa.0.0.copyload, !dbg !33812
  %1623 = fadd <8 x float> %1615, %1622, !dbg !33817
  %1624 = fmul <8 x float> %history.i209.i.sroa.19.sroa.0.014979, %_81.i.i.i112.i.sroa.0.0.copyload, !dbg !33822
  %1625 = fadd <8 x float> %1617, %1624, !dbg !33827
  %1626 = fmul <8 x float> %history.i209.i.sroa.19.sroa.0.014979, %_84.i.i.i109.i.sroa.0.0.copyload, !dbg !33832
  %1627 = fadd <8 x float> %1619, %1626, !dbg !33837
  %1628 = fmul <8 x float> %history.i209.i.sroa.19.sroa.0.014979, %_87.i.i.i106.i.sroa.0.0.copyload, !dbg !33842
  %1629 = fadd <8 x float> %1621, %1628, !dbg !33847
  %1630 = fmul <8 x float> %history.i209.i.sroa.19.sroa.0.014979, %_90.i.i.i103.i.sroa.0.0.copyload, !dbg !33852
  %1631 = fadd <8 x float> %1623, %1630, !dbg !33857
  %1632 = fmul <8 x float> %history.i209.i.sroa.22.sroa.0.014978, %_95.i.i.i99.i.sroa.0.0.copyload, !dbg !33862
  %1633 = fadd <8 x float> %1625, %1632, !dbg !33867
  %1634 = fmul <8 x float> %history.i209.i.sroa.22.sroa.0.014978, %_98.i.i.i96.i.sroa.0.0.copyload, !dbg !33872
  %1635 = fadd <8 x float> %1627, %1634, !dbg !33877
  %1636 = fmul <8 x float> %history.i209.i.sroa.22.sroa.0.014978, %_101.i.i.i93.i.sroa.0.0.copyload, !dbg !33882
  %1637 = fadd <8 x float> %1629, %1636, !dbg !33887
  %1638 = fmul <8 x float> %history.i209.i.sroa.22.sroa.0.014978, %_104.i.i.i90.i.sroa.0.0.copyload, !dbg !33892
  %1639 = fadd <8 x float> %1631, %1638, !dbg !33897
  %1640 = fmul <8 x float> %history.i209.i.sroa.25.sroa.0.014973, %_109.i.i.i86.i.sroa.0.0.copyload, !dbg !33902
  %1641 = fadd <8 x float> %1633, %1640, !dbg !33907
  %1642 = fmul <8 x float> %history.i209.i.sroa.25.sroa.0.014973, %_112.i.i.i83.i.sroa.0.0.copyload, !dbg !33912
  %1643 = fadd <8 x float> %1635, %1642, !dbg !33917
  %1644 = fmul <8 x float> %history.i209.i.sroa.25.sroa.0.014973, %_115.i.i.i80.i.sroa.0.0.copyload, !dbg !33922
  %1645 = fadd <8 x float> %1637, %1644, !dbg !33927
  %1646 = fmul <8 x float> %history.i209.i.sroa.25.sroa.0.014973, %_118.i.i.i77.i.sroa.0.0.copyload, !dbg !33932
  %1647 = fadd <8 x float> %1639, %1646, !dbg !33937
  %1648 = fmul <8 x float> %history.i209.i.sroa.29.sroa.0.014974, %_123.i.i.i73.i.sroa.0.0.copyload, !dbg !33942
  %1649 = fadd <8 x float> %1641, %1648, !dbg !33947
  %1650 = fmul <8 x float> %history.i209.i.sroa.29.sroa.0.014974, %_126.i.i.i70.i.sroa.0.0.copyload, !dbg !33952
  %1651 = fadd <8 x float> %1643, %1650, !dbg !33957
  %1652 = fmul <8 x float> %history.i209.i.sroa.29.sroa.0.014974, %_129.i.i.i67.i.sroa.0.0.copyload, !dbg !33962
  %1653 = fadd <8 x float> %1645, %1652, !dbg !33967
  %1654 = fmul <8 x float> %history.i209.i.sroa.29.sroa.0.014974, %_132.i.i.i64.i.sroa.0.0.copyload, !dbg !33972
  %1655 = fadd <8 x float> %1647, %1654, !dbg !33977
  %1656 = fmul <8 x float> %history.i209.i.sroa.32.sroa.0.014975, %_137.i.i.i60.i.sroa.0.0.copyload, !dbg !33982
  %1657 = fadd <8 x float> %1649, %1656, !dbg !33987
  %1658 = fmul <8 x float> %history.i209.i.sroa.32.sroa.0.014975, %_140.i.i.i57.i.sroa.0.0.copyload, !dbg !33992
  %1659 = fadd <8 x float> %1651, %1658, !dbg !33997
  %1660 = fmul <8 x float> %history.i209.i.sroa.32.sroa.0.014975, %_143.i.i.i54.i.sroa.0.0.copyload, !dbg !34002
  %1661 = fadd <8 x float> %1653, %1660, !dbg !34007
  %1662 = fmul <8 x float> %history.i209.i.sroa.32.sroa.0.014975, %_146.i.i.i51.i.sroa.0.0.copyload, !dbg !34012
  %1663 = fadd <8 x float> %1655, %1662, !dbg !34017
  %1664 = fmul <8 x float> %history.i209.i.sroa.35.sroa.0.014976, %_151.i.i.i47.i.sroa.0.0.copyload, !dbg !34022
  %1665 = fadd <8 x float> %1657, %1664, !dbg !34027
  %1666 = fmul <8 x float> %history.i209.i.sroa.35.sroa.0.014976, %_154.i.i.i44.i.sroa.0.0.copyload, !dbg !34032
  %1667 = fadd <8 x float> %1659, %1666, !dbg !34037
  %1668 = fmul <8 x float> %history.i209.i.sroa.35.sroa.0.014976, %_157.i.i.i41.i.sroa.0.0.copyload, !dbg !34042
  %1669 = fadd <8 x float> %1661, %1668, !dbg !34047
  %1670 = fmul <8 x float> %history.i209.i.sroa.35.sroa.0.014976, %_160.i.i.i38.i.sroa.0.0.copyload, !dbg !34052
  %1671 = fadd <8 x float> %1663, %1670, !dbg !34057
  %1672 = fmul <8 x float> %history.i209.i.sroa.38.sroa.0.014977, %_165.i.i.i34.i.sroa.0.0.copyload, !dbg !34062
  %1673 = fadd <8 x float> %1665, %1672, !dbg !34067
  %1674 = fmul <8 x float> %history.i209.i.sroa.38.sroa.0.014977, %_168.i.i.i31.i.sroa.0.0.copyload, !dbg !34072
  %1675 = fadd <8 x float> %1667, %1674, !dbg !34077
  %1676 = fmul <8 x float> %history.i209.i.sroa.38.sroa.0.014977, %_171.i.i.i28.i.sroa.0.0.copyload, !dbg !34082
  %1677 = fadd <8 x float> %1669, %1676, !dbg !34087
  %1678 = fmul <8 x float> %history.i209.i.sroa.38.sroa.0.014977, %_174.i.i.i25.i.sroa.0.0.copyload, !dbg !34092
  %1679 = fadd <8 x float> %1671, %1678, !dbg !34097
  %1680 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1673), !dbg !34102
  %1681 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1583, <8 x float> %1680), !dbg !34108
  %1682 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1675), !dbg !34102
  %1683 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1681, <8 x float> %1682), !dbg !34108
  %1684 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1677), !dbg !34102
  %1685 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1683, <8 x float> %1684), !dbg !34108
  %1686 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1679), !dbg !34102
  %1687 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1685, <8 x float> %1686), !dbg !34108
  %_39.i240.i.idx = shl i64 %iter.sroa.0.0.i211.i14983, 5, !dbg !34113
  %_39.i240.i = getelementptr inbounds nuw i8, ptr %peaks_left.i, i64 %_39.i240.i.idx, !dbg !34113
  store <8 x float> %1687, ptr %_39.i240.i, align 4, !dbg !34118, !alias.scope !34123, !noalias !34127
  %exitcond17560.not = icmp eq i64 %1582, %umax17564, !dbg !33570
  br i1 %exitcond17560.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit244.i, label %bb5.i213.i, !dbg !33575

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit244.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231, %bb50.i
  %history.i209.i.sroa.0.0.lcssa = phi <8 x float> [ %history.i209.i.sroa.0.0.lcssa20380, %bb50.i ], [ %lanes.i5563.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231 ], !dbg !33589
  %history.i209.i.sroa.25.sroa.0.0.lcssa = phi <8 x float> [ %history.i209.i.sroa.25.sroa.0.0.lcssa20446, %bb50.i ], [ %history.i209.i.sroa.22.sroa.0.014978, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231 ], !dbg !33589
  %history.i209.i.sroa.29.sroa.0.0.lcssa = phi <8 x float> [ %history.i209.i.sroa.29.sroa.0.0.lcssa20447, %bb50.i ], [ %history.i209.i.sroa.25.sroa.0.014973, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231 ], !dbg !33589
  %history.i209.i.sroa.32.sroa.0.0.lcssa = phi <8 x float> [ %history.i209.i.sroa.32.sroa.0.0.lcssa20448, %bb50.i ], [ %history.i209.i.sroa.29.sroa.0.014974, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231 ], !dbg !33589
  %history.i209.i.sroa.35.sroa.0.0.lcssa = phi <8 x float> [ %history.i209.i.sroa.35.sroa.0.0.lcssa20449, %bb50.i ], [ %history.i209.i.sroa.32.sroa.0.014975, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231 ], !dbg !33589
  %history.i209.i.sroa.38.sroa.0.0.lcssa = phi <8 x float> [ %history.i209.i.sroa.38.sroa.0.0.lcssa20450, %bb50.i ], [ %history.i209.i.sroa.35.sroa.0.014976, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231 ], !dbg !33589
  %history.i209.i.sroa.41.sroa.0.0.lcssa = phi <8 x float> [ %history.i209.i.sroa.41.sroa.0.0.lcssa20451, %bb50.i ], [ %history.i209.i.sroa.38.sroa.0.014977, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231 ], !dbg !33589
  %history.i209.i.sroa.22.sroa.0.0.lcssa = phi <8 x float> [ %history.i209.i.sroa.22.sroa.0.0.lcssa20445, %bb50.i ], [ %history.i209.i.sroa.19.sroa.0.014979, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231 ], !dbg !33589
  %history.i209.i.sroa.19.sroa.0.0.lcssa = phi <8 x float> [ %history.i209.i.sroa.19.sroa.0.0.lcssa20444, %bb50.i ], [ %history.i209.i.sroa.16.sroa.0.014980, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231 ], !dbg !33589
  %history.i209.i.sroa.16.sroa.0.0.lcssa = phi <8 x float> [ %history.i209.i.sroa.16.sroa.0.0.lcssa20443, %bb50.i ], [ %history.i209.i.sroa.13.sroa.0.014981, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231 ], !dbg !33589
  %history.i209.i.sroa.13.sroa.0.0.lcssa = phi <8 x float> [ %history.i209.i.sroa.13.sroa.0.0.lcssa20422, %bb50.i ], [ %history.i209.i.sroa.10.sroa.0.014982, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231 ], !dbg !33589
  %history.i209.i.sroa.10.sroa.0.0.lcssa = phi <8 x float> [ %history.i209.i.sroa.10.sroa.0.0.lcssa20401, %bb50.i ], [ %history.i209.i.sroa.0.014972, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6231 ], !dbg !33589
  store <8 x float> %history.i209.i.sroa.16.sroa.0.0.lcssa, ptr %history.i209.i.sroa.16.0.hot_left.i.sroa_idx, align 32, !dbg !34131
  store <8 x float> %history.i209.i.sroa.19.sroa.0.0.lcssa, ptr %history.i209.i.sroa.19.0.hot_left.i.sroa_idx, align 32, !dbg !34131
  store <8 x float> %history.i209.i.sroa.22.sroa.0.0.lcssa, ptr %history.i209.i.sroa.22.0.hot_left.i.sroa_idx, align 32, !dbg !34131
  store <8 x float> %history.i209.i.sroa.25.sroa.0.0.lcssa, ptr %history.i209.i.sroa.25.0.hot_left.i.sroa_idx, align 32, !dbg !34131
  store <8 x float> %history.i209.i.sroa.29.sroa.0.0.lcssa, ptr %history.i209.i.sroa.29.0.hot_left.i.sroa_idx, align 32, !dbg !34131
  store <8 x float> %history.i209.i.sroa.32.sroa.0.0.lcssa, ptr %history.i209.i.sroa.32.0.hot_left.i.sroa_idx, align 32, !dbg !34131
  store <8 x float> %history.i209.i.sroa.35.sroa.0.0.lcssa, ptr %history.i209.i.sroa.35.0.hot_left.i.sroa_idx, align 32, !dbg !34131
  store <8 x float> %history.i209.i.sroa.38.sroa.0.0.lcssa, ptr %history.i209.i.sroa.38.0.hot_left.i.sroa_idx, align 32, !dbg !34131
  store <8 x float> %history.i209.i.sroa.41.sroa.0.0.lcssa, ptr %history.i209.i.sroa.41.0.hot_left.i.sroa_idx, align 32, !dbg !34131
  br i1 %_20.i212.i14971.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, label %bb5.i.i.lr.ph, !dbg !34132

bb5.i.i.lr.ph:                                    ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit244.i
  %_5.i5001 = load <8 x float>, ptr %self, align 32
  %_14.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1520, align 32
  %_17.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1521, align 32
  %_20.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1522, align 32
  %_25.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row12.i.i.i220.i, align 32
  %_28.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1523, align 32
  %_31.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1524, align 32
  %_34.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1525, align 32
  %_39.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row13.i.i.i221.i, align 32
  %_42.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1526, align 32
  %_45.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1527, align 32
  %_48.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1528, align 32
  %_53.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row14.i.i.i222.i, align 32
  %_56.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1529, align 32
  %_59.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1530, align 32
  %_62.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1531, align 32
  %_67.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row15.i.i.i223.i, align 32
  %_70.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1532, align 32
  %_73.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1533, align 32
  %_76.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1534, align 32
  %_81.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row16.i.i.i224.i, align 32
  %_84.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1535, align 32
  %_87.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1536, align 32
  %_90.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1537, align 32
  %_95.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row17.i.i.i225.i, align 32
  %_98.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1538, align 32
  %_101.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1539, align 32
  %_104.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1540, align 32
  %_109.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row18.i.i.i226.i, align 32
  %_112.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1541, align 32
  %_115.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1542, align 32
  %_118.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1543, align 32
  %_123.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row19.i.i.i227.i, align 32
  %_126.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1544, align 32
  %_129.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1545, align 32
  %_132.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1546, align 32
  %_137.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row20.i.i.i228.i, align 32
  %_140.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1547, align 32
  %_143.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1548, align 32
  %_146.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1549, align 32
  %_151.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row21.i.i.i229.i, align 32
  %_154.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1550, align 32
  %_157.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1551, align 32
  %_160.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1552, align 32
  %_165.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row22.i.i.i230.i, align 32
  %_168.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1553, align 32
  %_171.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1554, align 32
  %_174.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1555, align 32
  br label %bb5.i.i, !dbg !34132

bb5.i.i:                                          ; preds = %bb5.i.i.lr.ph, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236
  %iter.sroa.0.0.i.i15010 = phi i64 [ 0, %bb5.i.i.lr.ph ], [ %1688, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236 ]
  %history.i.i.sroa.0.015009 = phi <8 x float> [ %history.i.i.sroa.0.0.lcssa20452, %bb5.i.i.lr.ph ], [ %lanes.i5572.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236 ]
  %history.i.i.sroa.25.sroa.0.015008 = phi <8 x float> [ %history.i.i.sroa.25.sroa.0.0.lcssa20518, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.22.sroa.0.015003, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236 ]
  %history.i.i.sroa.29.sroa.0.015007 = phi <8 x float> [ %history.i.i.sroa.29.sroa.0.0.lcssa20519, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.25.sroa.0.015008, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236 ]
  %history.i.i.sroa.32.sroa.0.015006 = phi <8 x float> [ %history.i.i.sroa.32.sroa.0.0.lcssa20520, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.29.sroa.0.015007, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236 ]
  %history.i.i.sroa.35.sroa.0.015005 = phi <8 x float> [ %history.i.i.sroa.35.sroa.0.0.lcssa20521, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.32.sroa.0.015006, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236 ]
  %history.i.i.sroa.38.sroa.0.015004 = phi <8 x float> [ %history.i.i.sroa.38.sroa.0.0.lcssa20522, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.35.sroa.0.015005, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236 ]
  %history.i.i.sroa.22.sroa.0.015003 = phi <8 x float> [ %history.i.i.sroa.22.sroa.0.0.lcssa20517, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.19.sroa.0.015002, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236 ]
  %history.i.i.sroa.19.sroa.0.015002 = phi <8 x float> [ %history.i.i.sroa.19.sroa.0.0.lcssa20516, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.16.sroa.0.015001, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236 ]
  %history.i.i.sroa.16.sroa.0.015001 = phi <8 x float> [ %history.i.i.sroa.16.sroa.0.0.lcssa20515, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.13.sroa.0.015000, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236 ]
  %history.i.i.sroa.13.sroa.0.015000 = phi <8 x float> [ %history.i.i.sroa.13.sroa.0.0.lcssa20494, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.10.sroa.0.014999, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236 ]
  %history.i.i.sroa.10.sroa.0.014999 = phi <8 x float> [ %history.i.i.sroa.10.sroa.0.0.lcssa20473, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.0.015009, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236 ]
  %1688 = add nuw nsw i64 %iter.sroa.0.0.i.i15010, 1, !dbg !34135
  %_11.i.i = add nuw nsw i64 %iter.sroa.0.0.i.i15010, %iter3.sroa.0.0.i15376, !dbg !34138
  %base.i.i = shl i64 %_11.i.i, 3, !dbg !34138
  %_24.i.i = icmp samesign ugt i64 %base.i.i, %right_io.1, !dbg !34139
  br i1 %_24.i.i, label %bb7.i.i, label %bb8.i.i, !dbg !34139, !prof !639

bb8.i.i:                                          ; preds = %bb5.i.i
  %_27.i.i = sub nuw nsw i64 %right_io.1, %base.i.i, !dbg !34142
  %_8.i5575 = icmp samesign ugt i64 %_27.i.i, 7, !dbg !34143
  br i1 %_8.i5575, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236, label %bb2.i5576, !dbg !34143, !prof !651

bb2.i5576:                                        ; preds = %bb8.i.i
  store <8 x float> %history.i209.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.10.sroa.0.0.lcssa, ptr %history.i209.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.13.sroa.0.0.lcssa, ptr %history.i209.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i.i.sroa.0.0.lcssa20452, ptr %hot_right.i, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa20473, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa20494, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa20524, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015045.lcssa15201.lcssa20545, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1346.lcssa1508915230.lcssa20567, ptr %_22.i301.i, align 4
  store i32 %storemerge.i1311.lcssa1514615302.lcssa20592, ptr %_22.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_27.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !34148, !noalias !34149
  unreachable, !dbg !34148

bb7.i.i:                                          ; preds = %bb5.i.i
  store <8 x float> %history.i209.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.10.sroa.0.0.lcssa, ptr %history.i209.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.13.sroa.0.0.lcssa, ptr %history.i209.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i.i.sroa.0.0.lcssa20452, ptr %hot_right.i, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa20473, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa20494, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa20524, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015045.lcssa15201.lcssa20545, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1346.lcssa1508915230.lcssa20567, ptr %_22.i301.i, align 4
  store i32 %storemerge.i1311.lcssa1514615302.lcssa20592, ptr %_22.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i.i, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_fc26f793d85338b5649d38df0c19e7e0) #30, !dbg !34156, !noalias !34157
  unreachable, !dbg !34156

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236: ; preds = %bb8.i.i
  %_31.i.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %base.i.i, !dbg !34158
  %lanes.i5572.sroa.0.0.copyload = load <8 x float>, ptr %_31.i.i, align 4, !dbg !34160, !alias.scope !34164, !noalias !34168
  %1689 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i.i.sroa.22.sroa.0.015003), !dbg !34170
  %1690 = fmul <8 x float> %lanes.i5572.sroa.0.0.copyload, %_5.i5001, !dbg !34177
  %1691 = fadd <8 x float> %1690, zeroinitializer, !dbg !34183
  %1692 = fmul <8 x float> %lanes.i5572.sroa.0.0.copyload, %_14.i.i.i.i.sroa.0.0.copyload, !dbg !34188
  %1693 = fadd <8 x float> %1692, zeroinitializer, !dbg !34193
  %1694 = fmul <8 x float> %lanes.i5572.sroa.0.0.copyload, %_17.i.i.i.i.sroa.0.0.copyload, !dbg !34198
  %1695 = fadd <8 x float> %1694, zeroinitializer, !dbg !34203
  %1696 = fmul <8 x float> %lanes.i5572.sroa.0.0.copyload, %_20.i.i.i.i.sroa.0.0.copyload, !dbg !34208
  %1697 = fadd <8 x float> %1696, zeroinitializer, !dbg !34213
  %1698 = fmul <8 x float> %history.i.i.sroa.0.015009, %_25.i.i.i.i.sroa.0.0.copyload, !dbg !34218
  %1699 = fadd <8 x float> %1691, %1698, !dbg !34223
  %1700 = fmul <8 x float> %history.i.i.sroa.0.015009, %_28.i.i.i.i.sroa.0.0.copyload, !dbg !34228
  %1701 = fadd <8 x float> %1693, %1700, !dbg !34233
  %1702 = fmul <8 x float> %history.i.i.sroa.0.015009, %_31.i.i.i.i.sroa.0.0.copyload, !dbg !34238
  %1703 = fadd <8 x float> %1695, %1702, !dbg !34243
  %1704 = fmul <8 x float> %history.i.i.sroa.0.015009, %_34.i.i.i.i.sroa.0.0.copyload, !dbg !34248
  %1705 = fadd <8 x float> %1697, %1704, !dbg !34253
  %1706 = fmul <8 x float> %history.i.i.sroa.10.sroa.0.014999, %_39.i.i.i.i.sroa.0.0.copyload, !dbg !34258
  %1707 = fadd <8 x float> %1699, %1706, !dbg !34263
  %1708 = fmul <8 x float> %history.i.i.sroa.10.sroa.0.014999, %_42.i.i.i.i.sroa.0.0.copyload, !dbg !34268
  %1709 = fadd <8 x float> %1701, %1708, !dbg !34273
  %1710 = fmul <8 x float> %history.i.i.sroa.10.sroa.0.014999, %_45.i.i.i.i.sroa.0.0.copyload, !dbg !34278
  %1711 = fadd <8 x float> %1703, %1710, !dbg !34283
  %1712 = fmul <8 x float> %history.i.i.sroa.10.sroa.0.014999, %_48.i.i.i.i.sroa.0.0.copyload, !dbg !34288
  %1713 = fadd <8 x float> %1705, %1712, !dbg !34293
  %1714 = fmul <8 x float> %history.i.i.sroa.13.sroa.0.015000, %_53.i.i.i.i.sroa.0.0.copyload, !dbg !34298
  %1715 = fadd <8 x float> %1707, %1714, !dbg !34303
  %1716 = fmul <8 x float> %history.i.i.sroa.13.sroa.0.015000, %_56.i.i.i.i.sroa.0.0.copyload, !dbg !34308
  %1717 = fadd <8 x float> %1709, %1716, !dbg !34313
  %1718 = fmul <8 x float> %history.i.i.sroa.13.sroa.0.015000, %_59.i.i.i.i.sroa.0.0.copyload, !dbg !34318
  %1719 = fadd <8 x float> %1711, %1718, !dbg !34323
  %1720 = fmul <8 x float> %history.i.i.sroa.13.sroa.0.015000, %_62.i.i.i.i.sroa.0.0.copyload, !dbg !34328
  %1721 = fadd <8 x float> %1713, %1720, !dbg !34333
  %1722 = fmul <8 x float> %history.i.i.sroa.16.sroa.0.015001, %_67.i.i.i.i.sroa.0.0.copyload, !dbg !34338
  %1723 = fadd <8 x float> %1715, %1722, !dbg !34343
  %1724 = fmul <8 x float> %history.i.i.sroa.16.sroa.0.015001, %_70.i.i.i.i.sroa.0.0.copyload, !dbg !34348
  %1725 = fadd <8 x float> %1717, %1724, !dbg !34353
  %1726 = fmul <8 x float> %history.i.i.sroa.16.sroa.0.015001, %_73.i.i.i.i.sroa.0.0.copyload, !dbg !34358
  %1727 = fadd <8 x float> %1719, %1726, !dbg !34363
  %1728 = fmul <8 x float> %history.i.i.sroa.16.sroa.0.015001, %_76.i.i.i.i.sroa.0.0.copyload, !dbg !34368
  %1729 = fadd <8 x float> %1721, %1728, !dbg !34373
  %1730 = fmul <8 x float> %history.i.i.sroa.19.sroa.0.015002, %_81.i.i.i.i.sroa.0.0.copyload, !dbg !34378
  %1731 = fadd <8 x float> %1723, %1730, !dbg !34383
  %1732 = fmul <8 x float> %history.i.i.sroa.19.sroa.0.015002, %_84.i.i.i.i.sroa.0.0.copyload, !dbg !34388
  %1733 = fadd <8 x float> %1725, %1732, !dbg !34393
  %1734 = fmul <8 x float> %history.i.i.sroa.19.sroa.0.015002, %_87.i.i.i.i.sroa.0.0.copyload, !dbg !34398
  %1735 = fadd <8 x float> %1727, %1734, !dbg !34403
  %1736 = fmul <8 x float> %history.i.i.sroa.19.sroa.0.015002, %_90.i.i.i.i.sroa.0.0.copyload, !dbg !34408
  %1737 = fadd <8 x float> %1729, %1736, !dbg !34413
  %1738 = fmul <8 x float> %history.i.i.sroa.22.sroa.0.015003, %_95.i.i.i.i.sroa.0.0.copyload, !dbg !34418
  %1739 = fadd <8 x float> %1731, %1738, !dbg !34423
  %1740 = fmul <8 x float> %history.i.i.sroa.22.sroa.0.015003, %_98.i.i.i.i.sroa.0.0.copyload, !dbg !34428
  %1741 = fadd <8 x float> %1733, %1740, !dbg !34433
  %1742 = fmul <8 x float> %history.i.i.sroa.22.sroa.0.015003, %_101.i.i.i.i.sroa.0.0.copyload, !dbg !34438
  %1743 = fadd <8 x float> %1735, %1742, !dbg !34443
  %1744 = fmul <8 x float> %history.i.i.sroa.22.sroa.0.015003, %_104.i.i.i.i.sroa.0.0.copyload, !dbg !34448
  %1745 = fadd <8 x float> %1737, %1744, !dbg !34453
  %1746 = fmul <8 x float> %history.i.i.sroa.25.sroa.0.015008, %_109.i.i.i.i.sroa.0.0.copyload, !dbg !34458
  %1747 = fadd <8 x float> %1739, %1746, !dbg !34463
  %1748 = fmul <8 x float> %history.i.i.sroa.25.sroa.0.015008, %_112.i.i.i.i.sroa.0.0.copyload, !dbg !34468
  %1749 = fadd <8 x float> %1741, %1748, !dbg !34473
  %1750 = fmul <8 x float> %history.i.i.sroa.25.sroa.0.015008, %_115.i.i.i.i.sroa.0.0.copyload, !dbg !34478
  %1751 = fadd <8 x float> %1743, %1750, !dbg !34483
  %1752 = fmul <8 x float> %history.i.i.sroa.25.sroa.0.015008, %_118.i.i.i.i.sroa.0.0.copyload, !dbg !34488
  %1753 = fadd <8 x float> %1745, %1752, !dbg !34493
  %1754 = fmul <8 x float> %history.i.i.sroa.29.sroa.0.015007, %_123.i.i.i.i.sroa.0.0.copyload, !dbg !34498
  %1755 = fadd <8 x float> %1747, %1754, !dbg !34503
  %1756 = fmul <8 x float> %history.i.i.sroa.29.sroa.0.015007, %_126.i.i.i.i.sroa.0.0.copyload, !dbg !34508
  %1757 = fadd <8 x float> %1749, %1756, !dbg !34513
  %1758 = fmul <8 x float> %history.i.i.sroa.29.sroa.0.015007, %_129.i.i.i.i.sroa.0.0.copyload, !dbg !34518
  %1759 = fadd <8 x float> %1751, %1758, !dbg !34523
  %1760 = fmul <8 x float> %history.i.i.sroa.29.sroa.0.015007, %_132.i.i.i.i.sroa.0.0.copyload, !dbg !34528
  %1761 = fadd <8 x float> %1753, %1760, !dbg !34533
  %1762 = fmul <8 x float> %history.i.i.sroa.32.sroa.0.015006, %_137.i.i.i.i.sroa.0.0.copyload, !dbg !34538
  %1763 = fadd <8 x float> %1755, %1762, !dbg !34543
  %1764 = fmul <8 x float> %history.i.i.sroa.32.sroa.0.015006, %_140.i.i.i.i.sroa.0.0.copyload, !dbg !34548
  %1765 = fadd <8 x float> %1757, %1764, !dbg !34553
  %1766 = fmul <8 x float> %history.i.i.sroa.32.sroa.0.015006, %_143.i.i.i.i.sroa.0.0.copyload, !dbg !34558
  %1767 = fadd <8 x float> %1759, %1766, !dbg !34563
  %1768 = fmul <8 x float> %history.i.i.sroa.32.sroa.0.015006, %_146.i.i.i.i.sroa.0.0.copyload, !dbg !34568
  %1769 = fadd <8 x float> %1761, %1768, !dbg !34573
  %1770 = fmul <8 x float> %history.i.i.sroa.35.sroa.0.015005, %_151.i.i.i.i.sroa.0.0.copyload, !dbg !34578
  %1771 = fadd <8 x float> %1763, %1770, !dbg !34583
  %1772 = fmul <8 x float> %history.i.i.sroa.35.sroa.0.015005, %_154.i.i.i.i.sroa.0.0.copyload, !dbg !34588
  %1773 = fadd <8 x float> %1765, %1772, !dbg !34593
  %1774 = fmul <8 x float> %history.i.i.sroa.35.sroa.0.015005, %_157.i.i.i.i.sroa.0.0.copyload, !dbg !34598
  %1775 = fadd <8 x float> %1767, %1774, !dbg !34603
  %1776 = fmul <8 x float> %history.i.i.sroa.35.sroa.0.015005, %_160.i.i.i.i.sroa.0.0.copyload, !dbg !34608
  %1777 = fadd <8 x float> %1769, %1776, !dbg !34613
  %1778 = fmul <8 x float> %history.i.i.sroa.38.sroa.0.015004, %_165.i.i.i.i.sroa.0.0.copyload, !dbg !34618
  %1779 = fadd <8 x float> %1771, %1778, !dbg !34623
  %1780 = fmul <8 x float> %history.i.i.sroa.38.sroa.0.015004, %_168.i.i.i.i.sroa.0.0.copyload, !dbg !34628
  %1781 = fadd <8 x float> %1773, %1780, !dbg !34633
  %1782 = fmul <8 x float> %history.i.i.sroa.38.sroa.0.015004, %_171.i.i.i.i.sroa.0.0.copyload, !dbg !34638
  %1783 = fadd <8 x float> %1775, %1782, !dbg !34643
  %1784 = fmul <8 x float> %history.i.i.sroa.38.sroa.0.015004, %_174.i.i.i.i.sroa.0.0.copyload, !dbg !34648
  %1785 = fadd <8 x float> %1777, %1784, !dbg !34653
  %1786 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1779), !dbg !34658
  %1787 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1689, <8 x float> %1786), !dbg !34664
  %1788 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1781), !dbg !34658
  %1789 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1787, <8 x float> %1788), !dbg !34664
  %1790 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1783), !dbg !34658
  %1791 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1789, <8 x float> %1790), !dbg !34664
  %1792 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1785), !dbg !34658
  %1793 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1791, <8 x float> %1792), !dbg !34664
  %_39.i.i.idx = shl i64 %iter.sroa.0.0.i.i15010, 5, !dbg !34669
  %_39.i.i = getelementptr inbounds nuw i8, ptr %peaks_right.i, i64 %_39.i.i.idx, !dbg !34669
  store <8 x float> %1793, ptr %_39.i.i, align 4, !dbg !34674, !alias.scope !34679, !noalias !34683
  %exitcond17565.not = icmp eq i64 %1688, %umax17564, !dbg !34687
  br i1 %exitcond17565.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, label %bb5.i.i, !dbg !34132

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit244.i
  %history.i.i.sroa.10.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.10.sroa.0.0.lcssa20473, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit244.i ], [ %history.i.i.sroa.0.015009, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236 ], !dbg !33590
  %history.i.i.sroa.13.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.13.sroa.0.0.lcssa20494, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit244.i ], [ %history.i.i.sroa.10.sroa.0.014999, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236 ], !dbg !33590
  %history.i.i.sroa.16.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.16.sroa.0.0.lcssa20515, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit244.i ], [ %history.i.i.sroa.13.sroa.0.015000, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236 ], !dbg !33590
  %history.i.i.sroa.19.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.19.sroa.0.0.lcssa20516, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit244.i ], [ %history.i.i.sroa.16.sroa.0.015001, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236 ], !dbg !33590
  %history.i.i.sroa.22.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.22.sroa.0.0.lcssa20517, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit244.i ], [ %history.i.i.sroa.19.sroa.0.015002, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236 ], !dbg !33590
  %history.i.i.sroa.41.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.41.sroa.0.0.lcssa20523, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit244.i ], [ %history.i.i.sroa.38.sroa.0.015004, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236 ], !dbg !33590
  %history.i.i.sroa.38.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.38.sroa.0.0.lcssa20522, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit244.i ], [ %history.i.i.sroa.35.sroa.0.015005, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236 ], !dbg !33590
  %history.i.i.sroa.35.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.35.sroa.0.0.lcssa20521, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit244.i ], [ %history.i.i.sroa.32.sroa.0.015006, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236 ], !dbg !33590
  %history.i.i.sroa.32.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.32.sroa.0.0.lcssa20520, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit244.i ], [ %history.i.i.sroa.29.sroa.0.015007, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236 ], !dbg !33590
  %history.i.i.sroa.29.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.29.sroa.0.0.lcssa20519, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit244.i ], [ %history.i.i.sroa.25.sroa.0.015008, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236 ], !dbg !33590
  %history.i.i.sroa.25.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.25.sroa.0.0.lcssa20518, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit244.i ], [ %history.i.i.sroa.22.sroa.0.015003, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236 ], !dbg !33590
  %history.i.i.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.0.0.lcssa20452, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit244.i ], [ %lanes.i5572.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6236 ], !dbg !33590
  store <8 x float> %history.i.i.sroa.16.sroa.0.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 32, !dbg !34689
  store <8 x float> %history.i.i.sroa.19.sroa.0.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 32, !dbg !34689
  store <8 x float> %history.i.i.sroa.22.sroa.0.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 32, !dbg !34689
  store <8 x float> %history.i.i.sroa.25.sroa.0.0.lcssa, ptr %history.i.i.sroa.25.0.hot_right.i.sroa_idx, align 32, !dbg !34689
  store <8 x float> %history.i.i.sroa.29.sroa.0.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 32, !dbg !34689
  store <8 x float> %history.i.i.sroa.32.sroa.0.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 32, !dbg !34689
  store <8 x float> %history.i.i.sroa.35.sroa.0.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 32, !dbg !34689
  store <8 x float> %history.i.i.sroa.38.sroa.0.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 32, !dbg !34689
  store <8 x float> %history.i.i.sroa.41.sroa.0.0.lcssa, ptr %history.i.i.sroa.41.0.hot_right.i.sroa_idx, align 32, !dbg !34689
  br i1 %_20.i212.i14971.not, label %bb15.i.loopexit, label %bb20.i.lr.ph, !dbg !33554

bb20.i.lr.ph:                                     ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i
  %_65.i.sroa.3.0.copyload.pre = load i64, ptr %_65.i.sroa.3.0..sroa_idx, align 8, !dbg !34690, !noalias !33515
  %_65.i.sroa.4.0.copyload.pre = load i64, ptr %_65.i.sroa.4.0..sroa_idx, align 16, !dbg !34690, !noalias !33515
  %_66.i.sroa.3.0.copyload.pre = load i64, ptr %_66.i.sroa.3.0..sroa_idx, align 8, !dbg !34691, !noalias !33515
  %_66.i.sroa.4.0.copyload.pre = load i64, ptr %_66.i.sroa.4.0..sroa_idx, align 16, !dbg !34691, !noalias !33515
  %_8.i24.i.sroa.0.0.copyload.pre = load <8 x float>, ptr %_109.i, align 32
  %_9.i23.i.sroa.0.0.copyload.pre = load <8 x float>, ptr %_110.i, align 32
  %_8.i.i.sroa.0.0.copyload.pre = load <8 x float>, ptr %_114.i, align 32
  %_9.i.i.sroa.0.0.copyload.pre = load <8 x float>, ptr %_115.i, align 32
  %_54.0.i286.i.pre = load ptr, ptr %1560, align 32
  %_54.1.i287.i.pre = load i64, ptr %1561, align 8
  %_18.i298.i = load i64, ptr %1556, align 16
  %_56.0.i302.i = load ptr, ptr %1562, align 16, !nonnull !12, !align !24
  %_56.1.i303.i = load i64, ptr %1563, align 8
  %_37.i269.i.sroa.0.0.copyload = load <8 x float>, ptr %1565, align 32
  %_58.1.i315.i = load i64, ptr %1567, align 8
  %_58.0.i314.i = load ptr, ptr %1568, align 32, !nonnull !12, !align !24
  %_54.0.i.i = load ptr, ptr %1571, align 32, !nonnull !12, !align !24
  %_54.1.i.i = load i64, ptr %1572, align 8
  %_18.i249.i = load i64, ptr %1557, align 16
  %_56.0.i.i = load ptr, ptr %1573, align 16, !nonnull !12, !align !24
  %_56.1.i.i = load i64, ptr %1574, align 8
  %_37.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1576, align 32
  %_58.1.i.i = load i64, ptr %1578, align 8
  %_58.0.i.i = load ptr, ptr %1579, align 32, !nonnull !12, !align !24
  br label %bb20.i, !dbg !33554

bb20.i:                                           ; preds = %bb20.i.lr.ph, %bb32.i
  %.lcssa1517615339 = phi <8 x float> [ %.lcssa1517615338.lcssa20615, %bb20.i.lr.ph ], [ %.lcssa1517615338, %bb32.i ]
  %storemerge.i1311.lcssa1514615303 = phi i32 [ %storemerge.i1311.lcssa1514615302.lcssa20592, %bb20.i.lr.ph ], [ %storemerge.i1311.lcssa1514615302, %bb32.i ]
  %.lcssa1511915267 = phi <8 x float> [ %.lcssa1511915266.lcssa20590, %bb20.i.lr.ph ], [ %.lcssa1511915266, %bb32.i ]
  %storemerge.i1346.lcssa1508915231 = phi i32 [ %storemerge.i1346.lcssa1508915230.lcssa20567, %bb20.i.lr.ph ], [ %storemerge.i1346.lcssa1508915230, %bb32.i ]
  %frame.sroa.0.0.i15224 = phi i64 [ 0, %bb20.i.lr.ph ], [ %_80.i, %bb32.i ]
  %main_cursor.sroa.0.1.i15223 = phi i64 [ %main_cursor.sroa.0.0.i15375, %bb20.i.lr.ph ], [ %main_cursor.sroa.0.2.i, %bb32.i ]
  %ring_cursor.sroa.0.1.i15222 = phi i64 [ %ring_cursor.sroa.0.0.i15374, %bb20.i.lr.ph ], [ %ring_cursor.sroa.0.2.i, %bb32.i ]
  %minimum.i281.i.sroa.0.015031.lcssa1518215221 = phi <8 x float> [ %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa20524, %bb20.i.lr.ph ], [ %minimum.i281.i.sroa.0.015031.lcssa, %bb32.i ]
  %minimum.i.i.sroa.0.015045.lcssa1520115220 = phi <8 x float> [ %minimum.i.i.sroa.0.015045.lcssa15201.lcssa20545, %bb20.i.lr.ph ], [ %minimum.i.i.sroa.0.015045.lcssa, %bb32.i ]
  %_63.i = sub nuw nsw i64 %..i6958, %frame.sroa.0.0.i15224, !dbg !34692
  %ring.i1967 = load i64, ptr %85, align 8, !dbg !34693, !alias.scope !34695, !noalias !34698, !noundef !12
  %main.i1968 = load i64, ptr %86, align 8, !dbg !34702, !alias.scope !34695, !noalias !34698, !noundef !12
  %_10.i1969 = add i64 %ring_cursor.sroa.0.1.i15222, 1, !dbg !34703
  %_45.not.i1970 = icmp ult i64 %_10.i1969, %ring.i1967, !dbg !34704
  %1794 = select i1 %_45.not.i1970, i64 0, i64 %ring.i1967, !dbg !34704
  %start1.sroa.0.0.i1971 = sub nuw i64 %_10.i1969, %1794, !dbg !34704
  %_12.i1973 = add i64 %_65.i.sroa.3.0.copyload.pre, %ring_cursor.sroa.0.1.i15222, !dbg !34706
  %_46.not.i1974 = icmp ult i64 %_12.i1973, %ring.i1967, !dbg !34707
  %1795 = select i1 %_46.not.i1974, i64 0, i64 %ring.i1967, !dbg !34707
  %left_end.sroa.0.0.i1975 = sub nuw i64 %_12.i1973, %1795, !dbg !34707
  %_15.i1977 = add i64 %_66.i.sroa.3.0.copyload.pre, %ring_cursor.sroa.0.1.i15222, !dbg !34709
  %_47.not.i1978 = icmp ult i64 %_15.i1977, %ring.i1967, !dbg !34710
  %1796 = select i1 %_47.not.i1978, i64 0, i64 %ring.i1967, !dbg !34710
  %right_end.sroa.0.0.i1979 = sub nuw i64 %_15.i1977, %1796, !dbg !34710
  %_18.i1981 = add i64 %_65.i.sroa.4.0.copyload.pre, %ring_cursor.sroa.0.1.i15222, !dbg !34712
  %_48.not.i1982 = icmp ult i64 %_18.i1981, %ring.i1967, !dbg !34713
  %1797 = select i1 %_48.not.i1982, i64 0, i64 %ring.i1967, !dbg !34713
  %left_expiring.sroa.0.0.i1983 = sub nuw i64 %_18.i1981, %1797, !dbg !34713
  %_21.i1985 = add i64 %_66.i.sroa.4.0.copyload.pre, %ring_cursor.sroa.0.1.i15222, !dbg !34715
  %_49.not.i1986 = icmp ult i64 %_21.i1985, %ring.i1967, !dbg !34716
  %1798 = select i1 %_49.not.i1986, i64 0, i64 %ring.i1967, !dbg !34716
  %right_expiring.sroa.0.0.i1987 = sub nuw i64 %_21.i1985, %1798, !dbg !34716
  %_30.i1988 = sub i64 %ring.i1967, %ring_cursor.sroa.0.1.i15222, !dbg !34718
  %..i6983 = tail call noundef i64 @llvm.umin.i64(i64 %_30.i1988, i64 %_63.i), !dbg !34719
  %_31.i1990 = sub i64 %main.i1968, %main_cursor.sroa.0.1.i15223, !dbg !34721
  %..i6984 = tail call noundef i64 @llvm.umin.i64(i64 %_31.i1990, i64 %..i6983), !dbg !34722
  %_32.i1992 = sub i64 %ring.i1967, %start1.sroa.0.0.i1971, !dbg !34724
  %..i6985 = tail call noundef i64 @llvm.umin.i64(i64 %_32.i1992, i64 %..i6984), !dbg !34725
  %_34.i1994 = sub i64 %ring.i1967, %left_end.sroa.0.0.i1975, !dbg !34727
  %..i6986 = tail call noundef i64 @llvm.umin.i64(i64 %_34.i1994, i64 %..i6985), !dbg !34728
  %_36.i1996 = sub i64 %ring.i1967, %right_end.sroa.0.0.i1979, !dbg !34730
  %..i6987 = tail call noundef i64 @llvm.umin.i64(i64 %_36.i1996, i64 %..i6986), !dbg !34731
  %_38.i1998 = sub i64 %ring.i1967, %left_expiring.sroa.0.0.i1983, !dbg !34733
  %..i6988 = tail call noundef i64 @llvm.umin.i64(i64 %_38.i1998, i64 %..i6987), !dbg !34734
  %_40.i2000 = sub i64 %ring.i1967, %right_expiring.sroa.0.0.i1987, !dbg !34736
  %..i6989 = tail call noundef i64 @llvm.umin.i64(i64 %_40.i2000, i64 %..i6988), !dbg !34737
  %_69.i = add i64 %frame.sroa.0.0.i15224, %iter3.sroa.0.0.i15376, !dbg !34739
  %base.i = shl i64 %_69.i, 3, !dbg !34739
  %base.i12464 = add i64 %..i6989, %_69.i, !dbg !34742
  %_73.i = shl i64 %base.i12464, 3, !dbg !34742
  %_174.i = icmp ult i64 %_73.i, %base.i, !dbg !34745
  %_168.not.i = icmp ugt i64 %_73.i, %left_io.1
  %or.cond.i = or i1 %_174.i, %_168.not.i, !dbg !34745
  br i1 %or.cond.i, label %bb55.i, label %bb53.i, !dbg !34745, !prof !165

bb55.i:                                           ; preds = %bb20.i
  store <8 x float> %history.i209.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.10.sroa.0.0.lcssa, ptr %history.i209.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.13.sroa.0.0.lcssa, ptr %history.i209.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa20524, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015045.lcssa15201.lcssa20545, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1346.lcssa1508915230.lcssa20567, ptr %_22.i301.i, align 4
  store i32 %storemerge.i1311.lcssa1514615302.lcssa20592, ptr %_22.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i, i64 noundef %_73.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1f23704f6ca4e855056ec66ed6f2587d) #30, !dbg !34752, !noalias !33486
  unreachable, !dbg !34752

bb53.i:                                           ; preds = %bb20.i
  %_177.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %base.i, !dbg !34753
  %_178.not.i = icmp ugt i64 %_73.i, %right_io.1, !dbg !34757
  br i1 %_178.not.i, label %bb58.i, label %bb57.i, !dbg !34757, !prof !639

bb58.i:                                           ; preds = %bb53.i
  store <8 x float> %history.i209.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.10.sroa.0.0.lcssa, ptr %history.i209.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.13.sroa.0.0.lcssa, ptr %history.i209.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa20524, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015045.lcssa15201.lcssa20545, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1346.lcssa1508915230.lcssa20567, ptr %_22.i301.i, align 4
  store i32 %storemerge.i1311.lcssa1514615302.lcssa20592, ptr %_22.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i, i64 noundef %_73.i, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5a76e74e04cb182b892fe72abb75dc84) #30, !dbg !34762, !noalias !33486
  unreachable, !dbg !34762

bb57.i:                                           ; preds = %bb53.i
  %_185.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %base.i, !dbg !34763
  %_77.i = shl nuw nsw i64 %frame.sroa.0.0.i15224, 3, !dbg !34767
  %_80.i = add nuw nsw i64 %..i6989, %frame.sroa.0.0.i15224, !dbg !34769
  %_187.i = icmp ult i64 %_80.i, 33
  br i1 %_187.i, label %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit7064, label %bb60.i, !dbg !34770, !prof !2723

bb60.i:                                           ; preds = %bb57.i
  store <8 x float> %history.i209.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.10.sroa.0.0.lcssa, ptr %history.i209.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.13.sroa.0.0.lcssa, ptr %history.i209.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa20524, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015045.lcssa15201.lcssa20545, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1346.lcssa1508915230.lcssa20567, ptr %_22.i301.i, align 4
  store i32 %storemerge.i1311.lcssa1514615302.lcssa20592, ptr %_22.i.i, align 4
  %_79.i = shl nuw nsw i64 %_80.i, 3, !dbg !34769
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_77.i, i64 noundef %_79.i, i64 noundef 256, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_fa989b02c58b19a323a01f96ca250d1c) #30, !dbg !34778, !noalias !33486
  unreachable, !dbg !34778

_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit7064: ; preds = %bb57.i
  %_194.i = getelementptr inbounds nuw float, ptr %peaks_left.i, i64 %_77.i, !dbg !34779
  %_203.i = getelementptr inbounds nuw float, ptr %peaks_right.i, i64 %_77.i, !dbg !34783
  %_2.i.i.i706715059.not = icmp eq i64 %..i6989, 0, !dbg !34793
  br i1 %_2.i.i.i706715059.not, label %bb32.i, label %bb31.i.preheader, !dbg !34793

bb31.i.preheader:                                 ; preds = %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit7064
  %umin17578 = call i64 @llvm.umin.i64(i64 %_34.i1994, i64 %_36.i1996)
  %umin17579 = call i64 @llvm.umin.i64(i64 %umin17578, i64 %_38.i1998)
  %umin17580 = call i64 @llvm.umin.i64(i64 %umin17579, i64 %_40.i2000)
  %umin17581 = call i64 @llvm.umin.i64(i64 %umin17580, i64 %_32.i1992)
  %umin17582 = call i64 @llvm.umin.i64(i64 %umin17581, i64 %_30.i1988)
  %umin17583 = call i64 @llvm.umin.i64(i64 %umin17582, i64 %_31.i1990)
  %1799 = sub nsw i64 %umin17584, %frame.sroa.0.0.i15224
  %umin17585 = call i64 @llvm.umin.i64(i64 %umin17583, i64 %1799)
  %1800 = and i64 %umin17585, 2305843009213693951
  br label %bb31.i

bb31.i:                                           ; preds = %bb31.i.preheader, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1549
  %_33.i245.i.sroa.0.0.copyload15152 = phi <8 x float> [ %.lcssa1517615339, %bb31.i.preheader ], [ %1864, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1549 ]
  %storemerge.i131115124 = phi i32 [ %storemerge.i1311.lcssa1514615303, %bb31.i.preheader ], [ %storemerge.i1311, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1549 ]
  %_33.i272.i.sroa.0.0.copyload15095 = phi <8 x float> [ %.lcssa1511915267, %bb31.i.preheader ], [ %1826, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1549 ]
  %storemerge.i134615067 = phi i32 [ %storemerge.i1346.lcssa1508915231, %bb31.i.preheader ], [ %storemerge.i1346, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1549 ]
  %iter.i.sroa.36.015064 = phi i64 [ 0, %bb31.i.preheader ], [ %1801, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1549 ]
  %minimum.i281.i.sroa.0.01503115062 = phi <8 x float> [ %minimum.i281.i.sroa.0.015031.lcssa1518215221, %bb31.i.preheader ], [ %minimum.i281.i.sroa.0.0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1549 ]
  %minimum.i.i.sroa.0.01504515060 = phi <8 x float> [ %minimum.i.i.sroa.0.015045.lcssa1520115220, %bb31.i.preheader ], [ %minimum.i.i.sroa.0.0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1549 ]
  %start1.i.i.i.i.i.i.i.i7079 = shl i64 %iter.i.sroa.36.015064, 3, !dbg !34799
  %data.i.i.i.i.i.i7090 = getelementptr inbounds nuw float, ptr %_194.i, i64 %start1.i.i.i.i.i.i.i.i7079, !dbg !34805
  %lanes.i5608.sroa.0.0.copyload = load <8 x float>, ptr %data.i.i.i.i.i.i7090, align 4, !dbg !34808, !alias.scope !34816, !noalias !34820
  %data.i.i.i.i7085 = getelementptr inbounds nuw float, ptr %_203.i, i64 %start1.i.i.i.i.i.i.i.i7079, !dbg !34824
  %lanes.i5599.sroa.0.0.copyload = load <8 x float>, ptr %data.i.i.i.i7085, align 4, !dbg !34827, !alias.scope !34833, !noalias !34837
  %data.i.i.i.i.i.i.i.i7080 = getelementptr inbounds nuw float, ptr %_177.i, i64 %start1.i.i.i.i.i.i.i.i7079, !dbg !34841
  %data.i5.i.i.i.i.i.i.i7094 = getelementptr inbounds nuw float, ptr %_185.i, i64 %start1.i.i.i.i.i.i.i.i7079, !dbg !34843
  %1801 = add nuw nsw i64 %iter.i.sroa.36.015064, 1, !dbg !34846
  %1802 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i5599.sroa.0.0.copyload, <8 x float> %lanes.i5608.sroa.0.0.copyload), !dbg !34847
  %1803 = select <8 x i1> %1559, <8 x float> %1802, <8 x float> %lanes.i5599.sroa.0.0.copyload, !dbg !34853
  %_205.i = add i64 %iter.i.sroa.36.015064, %ring_cursor.sroa.0.1.i15222, !dbg !34860
  %_206.i = add i64 %iter.i.sroa.36.015064, %main_cursor.sroa.0.1.i15223, !dbg !34866
  %_207.i = add i64 %iter.i.sroa.36.015064, %left_end.sroa.0.0.i1975, !dbg !34867
  %_208.i = add i64 %iter.i.sroa.36.015064, %start1.sroa.0.0.i1971, !dbg !34868
  %_209.i = add i64 %iter.i.sroa.36.015064, %left_expiring.sroa.0.0.i1983, !dbg !34869
  %base.i9.i289.i = shl i64 %_205.i, 3, !dbg !34870
  %_7.i10.i290.i = add i64 %base.i9.i289.i, 8, !dbg !34873
  %1804 = or disjoint i64 %base.i9.i289.i, 7, !dbg !34874
  %or.cond.i13.i293.i.not = icmp ult i64 %1804, %_54.1.i287.i.pre, !dbg !34874
  br i1 %or.cond.i13.i293.i.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i294.i, label %bb4.i15.i327.i, !dbg !34874, !prof !2723

bb4.i15.i327.i:                                   ; preds = %bb31.i
  store <8 x float> %history.i209.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.10.sroa.0.0.lcssa, ptr %history.i209.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.13.sroa.0.0.lcssa, ptr %history.i209.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa20524, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015045.lcssa15201.lcssa20545, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1346.lcssa1508915230.lcssa20567, ptr %_22.i301.i, align 4
  store i32 %storemerge.i1311.lcssa1514615302.lcssa20592, ptr %_22.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i289.i, i64 noundef %_7.i10.i290.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i287.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8421dc8ec9e43c41e5b11981eccec9a3) #30, !dbg !34878, !noalias !34879
  unreachable, !dbg !34878

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i294.i: ; preds = %bb31.i
  %1805 = select <8 x i1> %1559, <8 x float> %1802, <8 x float> %lanes.i5608.sroa.0.0.copyload, !dbg !34893
  %1806 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1805, <8 x float> %_8.i24.i.sroa.0.0.copyload.pre, i8 30), !dbg !34898
  %1807 = bitcast <8 x float> %1806 to <8 x i32>, !dbg !34904
  %1808 = icmp slt <8 x i32> %1807, zeroinitializer, !dbg !34908
  %1809 = fdiv <8 x float> %_8.i24.i.sroa.0.0.copyload.pre, %1805, !dbg !34910
  %1810 = select <8 x i1> %1808, <8 x float> %1809, <8 x float> splat (float 1.000000e+00), !dbg !34908
  %_17.i14.i295.i = getelementptr inbounds nuw float, ptr %_54.0.i286.i.pre, i64 %base.i9.i289.i, !dbg !34915
  store <8 x float> %1810, ptr %_17.i14.i295.i, align 4, !dbg !34917, !alias.scope !34922, !noalias !34926
  %base.i1388 = shl i64 %_207.i, 3, !dbg !34930
  %1811 = or disjoint i64 %base.i1388, 7, !dbg !34933
  %or.cond.i1392.not = icmp ult i64 %1811, %_54.1.i287.i.pre, !dbg !34933
  br i1 %or.cond.i1392.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1396, label %bb4.i1395, !dbg !34933, !prof !2723

bb4.i1395:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i294.i
  store <8 x float> %history.i209.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.10.sroa.0.0.lcssa, ptr %history.i209.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.13.sroa.0.0.lcssa, ptr %history.i209.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa20524, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015045.lcssa15201.lcssa20545, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1346.lcssa1508915230.lcssa20567, ptr %_22.i301.i, align 4
  store i32 %storemerge.i1311.lcssa1514615302.lcssa20592, ptr %_22.i.i, align 4
  %_5.i1389 = add i64 %base.i1388, 8, !dbg !34937
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1388, i64 noundef %_5.i1389, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i287.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !34938, !noalias !34939
  unreachable, !dbg !34938

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1396: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i294.i
  %_15.i1394 = getelementptr inbounds nuw float, ptr %_54.0.i286.i.pre, i64 %base.i1388, !dbg !34947
  %lanes.i5281.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1394, align 4, !dbg !34949, !alias.scope !34954, !noalias !34958
  %position.i1340 = zext i32 %storemerge.i134615067 to i64, !dbg !34962
  %1812 = icmp eq i32 %storemerge.i134615067, 0, !dbg !34963
  br i1 %1812, label %bb5.i1342, label %bb3.i1341, !dbg !34963

bb3.i1341:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1396
  %1813 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %minimum.i281.i.sroa.0.01503115062, <8 x float> %lanes.i5281.sroa.0.0.copyload), !dbg !34964
  br label %bb5.i1342, !dbg !34969

bb5.i1342:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1396, %bb3.i1341
  %minimum.i281.i.sroa.0.0 = phi <8 x float> [ %1813, %bb3.i1341 ], [ %lanes.i5281.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1396 ], !dbg !34970
  %_15.i1343 = add nuw nsw i64 %position.i1340, 1, !dbg !34971
  %complete.i1344 = icmp eq i64 %_15.i1343, %_18.i298.i, !dbg !34971
  br i1 %complete.i1344, label %bb19.i1352, label %bb7.i1345, !dbg !34972

bb7.i1345:                                        ; preds = %bb5.i1342
  %base.i1379 = shl i64 %_208.i, 3, !dbg !34973
  %1814 = or disjoint i64 %base.i1379, 7, !dbg !34975
  %or.cond.i1383.not = icmp ult i64 %1814, %_54.1.i287.i.pre, !dbg !34975
  br i1 %or.cond.i1383.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1387, label %bb4.i1386, !dbg !34975, !prof !2723

bb4.i1386:                                        ; preds = %bb7.i1345
  store <8 x float> %history.i209.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.10.sroa.0.0.lcssa, ptr %history.i209.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.13.sroa.0.0.lcssa, ptr %history.i209.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa20524, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015045.lcssa15201.lcssa20545, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1346.lcssa1508915230.lcssa20567, ptr %_22.i301.i, align 4
  store i32 %storemerge.i1311.lcssa1514615302.lcssa20592, ptr %_22.i.i, align 4
  %_5.i1380 = add i64 %base.i1379, 8, !dbg !34979
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1379, i64 noundef %_5.i1380, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i287.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !34980, !noalias !34981
  unreachable, !dbg !34980

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1387: ; preds = %bb7.i1345
  %_15.i1385 = getelementptr inbounds nuw float, ptr %_54.0.i286.i.pre, i64 %base.i1379, !dbg !34985
  %lanes.i5288.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1385, align 4, !dbg !34987, !alias.scope !34992, !noalias !34996
  %1815 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %lanes.i5288.sroa.0.0.copyload, <8 x float> %minimum.i281.i.sroa.0.0), !dbg !35000
  %1816 = trunc i64 %_15.i1343 to i32, !dbg !35005
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1364, !dbg !35006

bb19.i1352:                                       ; preds = %bb5.i1342, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit
  %end.sroa.0.0.i135015026 = phi i64 [ %1820, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit ], [ %_207.i, %bb5.i1342 ]
  %iter.sroa.0.0.i134915025 = phi i64 [ %_30.i1353, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit ], [ 0, %bb5.i1342 ]
  %suffix.i1333.sroa.0.015024 = phi <8 x float> [ %1818, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit ], [ %lanes.i5281.sroa.0.0.copyload, %bb5.i1342 ]
  %base.i1365 = shl i64 %end.sroa.0.0.i135015026, 3, !dbg !35007
  %1817 = or disjoint i64 %base.i1365, 7, !dbg !35009
  %or.cond.i1367.not = icmp ult i64 %1817, %_54.1.i287.i.pre, !dbg !35009
  br i1 %or.cond.i1367.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, label %bb4.i, !dbg !35009, !prof !2723

bb4.i:                                            ; preds = %bb19.i1352
  store <8 x float> %history.i209.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.10.sroa.0.0.lcssa, ptr %history.i209.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.13.sroa.0.0.lcssa, ptr %history.i209.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa20524, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015045.lcssa15201.lcssa20545, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1346.lcssa1508915230.lcssa20567, ptr %_22.i301.i, align 4
  store i32 %storemerge.i1311.lcssa1514615302.lcssa20592, ptr %_22.i.i, align 4
  %_5.i = add i64 %base.i1365, 8, !dbg !35013
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1365, i64 noundef %_5.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i287.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !35014, !noalias !35015
  unreachable, !dbg !35014

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit: ; preds = %bb19.i1352
  %_30.i1353 = add nuw i64 %iter.sroa.0.0.i134915025, 1, !dbg !35019
  %_15.i1369 = getelementptr inbounds nuw float, ptr %_54.0.i286.i.pre, i64 %base.i1365, !dbg !35024
  %lanes.i5302.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1369, align 4, !dbg !35026, !alias.scope !35031, !noalias !35035
  %1818 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %suffix.i1333.sroa.0.015024, <8 x float> %lanes.i5302.sroa.0.0.copyload), !dbg !35039
  store <8 x float> %1818, ptr %_15.i1369, align 4, !dbg !35044, !alias.scope !35050, !noalias !35054
  %1819 = icmp eq i64 %end.sroa.0.0.i135015026, 0, !dbg !35058
  %spec.store.select.i1361 = select i1 %1819, i64 %ring.i, i64 %end.sroa.0.0.i135015026, !dbg !35058
  %1820 = add i64 %spec.store.select.i1361, -1, !dbg !35059
  %exitcond17567.not = icmp eq i64 %_30.i1353, %_18.i298.i, !dbg !35060
  br i1 %exitcond17567.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1364, label %bb19.i1352, !dbg !35062

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1364: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1387
  %minimum.i281.i.sroa.0.1 = phi <8 x float> [ %1815, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1387 ], [ %minimum.i281.i.sroa.0.0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit ], !dbg !34970
  %storemerge.i1346 = phi i32 [ %1816, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1387 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit ], !dbg !35063
  %1821 = fmul <8 x float> %minimum.i281.i.sroa.0.1, splat (float 1.638400e+04), !dbg !35064
  %1822 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %1821), !dbg !35069
  %1823 = fmul <8 x float> %1822, splat (float 0x3F10000000000000), !dbg !35074
  %base.i1568 = shl i64 %_209.i, 3, !dbg !35079
  %1824 = or disjoint i64 %base.i1568, 7, !dbg !35081
  %or.cond.i1572.not = icmp ult i64 %1824, %_56.1.i303.i, !dbg !35081
  br i1 %or.cond.i1572.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1576, label %bb4.i1575, !dbg !35081, !prof !2723

bb4.i1575:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1364
  store <8 x float> %history.i209.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.10.sroa.0.0.lcssa, ptr %history.i209.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.13.sroa.0.0.lcssa, ptr %history.i209.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa20524, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015045.lcssa15201.lcssa20545, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1346.lcssa1508915230.lcssa20567, ptr %_22.i301.i, align 4
  store i32 %storemerge.i1311.lcssa1514615302.lcssa20592, ptr %_22.i.i, align 4
  %_5.i1569 = add i64 %base.i1568, 8, !dbg !35085
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1568, i64 noundef %_5.i1569, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i303.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !35086, !noalias !35087
  unreachable, !dbg !35086

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1576: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1364
  %_15.i1574 = getelementptr inbounds nuw float, ptr %_56.0.i302.i, i64 %base.i1568, !dbg !35091
  %lanes.i.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1574, align 4, !dbg !35093, !alias.scope !35098, !noalias !35102
  %1825 = fadd <8 x float> %1823, %_33.i272.i.sroa.0.0.copyload15095, !dbg !35106
  %1826 = fsub <8 x float> %1825, %lanes.i.sroa.0.0.copyload, !dbg !35111
  %_8.not.i4.i310.i = icmp ugt i64 %_7.i10.i290.i, %_56.1.i303.i
  br i1 %_8.not.i4.i310.i, label %bb4.i7.i326.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i312.i, !dbg !35116, !prof !165

bb4.i7.i326.i:                                    ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1576
  store <8 x float> %history.i209.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.10.sroa.0.0.lcssa, ptr %history.i209.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.13.sroa.0.0.lcssa, ptr %history.i209.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa20524, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015045.lcssa15201.lcssa20545, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1346.lcssa1508915230.lcssa20567, ptr %_22.i301.i, align 4
  store i32 %storemerge.i1311.lcssa1514615302.lcssa20592, ptr %_22.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i289.i, i64 noundef %_7.i10.i290.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i303.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8421dc8ec9e43c41e5b11981eccec9a3) #30, !dbg !35121, !noalias !35122
  unreachable, !dbg !35121

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i312.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1576
  %_17.i6.i313.i = getelementptr inbounds nuw float, ptr %_56.0.i302.i, i64 %base.i9.i289.i, !dbg !35126
  store <8 x float> %1823, ptr %_17.i6.i313.i, align 4, !dbg !35128, !alias.scope !35133, !noalias !35137
  %_41.i265.i.sroa.0.0.copyload = load <8 x float>, ptr %1566, align 32, !dbg !35141
  %1827 = fdiv <8 x float> %1826, %_37.i269.i.sroa.0.0.copyload, !dbg !35142
  %1828 = fsub <8 x float> splat (float 1.000000e+00), %1827, !dbg !35147
  %1829 = fsub <8 x float> %1828, %_41.i265.i.sroa.0.0.copyload, !dbg !35152
  %1830 = fmul <8 x float> %_9.i23.i.sroa.0.0.copyload.pre, %1829, !dbg !35157
  %1831 = fadd <8 x float> %_41.i265.i.sroa.0.0.copyload, %1830, !dbg !35162
  %1832 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1828, <8 x float> %1831), !dbg !35166
  %1833 = bitcast <8 x float> %1832 to <8 x i32>, !dbg !35171
  %1834 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1832), !dbg !35177
  %1835 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1834, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !35179
  %1836 = bitcast <8 x float> %1835 to <8 x i32>, !dbg !35185
  %1837 = xor <8 x i32> %1836, splat (i32 -1), !dbg !35191
  %1838 = and <8 x i32> %1837, %1833, !dbg !35193
  store <8 x i32> %1838, ptr %1566, align 32, !dbg !35197
  %base.i1559 = shl i64 %_206.i, 3, !dbg !35198
  %_5.i1560 = add i64 %base.i1559, 8, !dbg !35200
  %1839 = or disjoint i64 %base.i1559, 7, !dbg !35201
  %or.cond.i1563.not = icmp ult i64 %1839, %_58.1.i315.i, !dbg !35201
  br i1 %or.cond.i1563.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1567, label %bb4.i1566, !dbg !35201, !prof !2723

bb4.i1566:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i312.i
  store <8 x float> %history.i209.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.10.sroa.0.0.lcssa, ptr %history.i209.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.13.sroa.0.0.lcssa, ptr %history.i209.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa20524, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015045.lcssa15201.lcssa20545, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1346.lcssa1508915230.lcssa20567, ptr %_22.i301.i, align 4
  store i32 %storemerge.i1311.lcssa1514615302.lcssa20592, ptr %_22.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1559, i64 noundef %_5.i1560, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i315.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !35205, !noalias !35206
  unreachable, !dbg !35205

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1567: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i312.i
  %1840 = bitcast <8 x i32> %1838 to <8 x float>, !dbg !35210
  %1841 = fsub <8 x float> splat (float 1.000000e+00), %1840, !dbg !35211
  %_15.i1565 = getelementptr inbounds nuw float, ptr %_58.0.i314.i, i64 %base.i1559, !dbg !35216
  %lanes.i5148.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1565, align 4, !dbg !35218, !alias.scope !35223, !noalias !35227
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_15.i1565, ptr noundef nonnull align 4 dereferenceable(32) %data.i.i.i.i.i.i.i.i7080, i64 32, i1 false), !dbg !35231
  %1842 = fmul <8 x float> %1841, %lanes.i5148.sroa.0.0.copyload, !dbg !35237
  %1843 = select <8 x i1> %1570, <8 x float> %lanes.i5148.sroa.0.0.copyload, <8 x float> %1842, !dbg !35242
  store <8 x float> %1843, ptr %data.i.i.i.i.i.i.i.i7080, align 4, !dbg !35247, !alias.scope !35252, !noalias !35256
  %_212.i = add i64 %iter.i.sroa.36.015064, %right_end.sroa.0.0.i1979, !dbg !35260
  %_214.i = add i64 %iter.i.sroa.36.015064, %right_expiring.sroa.0.0.i1987, !dbg !35262
  %_8.not.i12.i.i = icmp ugt i64 %_7.i10.i290.i, %_54.1.i.i
  br i1 %_8.not.i12.i.i, label %bb4.i15.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i, !dbg !35263, !prof !165

bb4.i15.i.i:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1567
  store <8 x float> %history.i209.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.10.sroa.0.0.lcssa, ptr %history.i209.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.13.sroa.0.0.lcssa, ptr %history.i209.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa20524, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015045.lcssa15201.lcssa20545, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1346.lcssa1508915230.lcssa20567, ptr %_22.i301.i, align 4
  store i32 %storemerge.i1311.lcssa1514615302.lcssa20592, ptr %_22.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i289.i, i64 noundef %_7.i10.i290.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8421dc8ec9e43c41e5b11981eccec9a3) #30, !dbg !35269, !noalias !35270
  unreachable, !dbg !35269

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1567
  %1844 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1803, <8 x float> %_8.i.i.sroa.0.0.copyload.pre, i8 30), !dbg !35284
  %1845 = bitcast <8 x float> %1844 to <8 x i32>, !dbg !35290
  %1846 = icmp slt <8 x i32> %1845, zeroinitializer, !dbg !35294
  %1847 = fdiv <8 x float> %_8.i.i.sroa.0.0.copyload.pre, %1803, !dbg !35296
  %1848 = select <8 x i1> %1846, <8 x float> %1847, <8 x float> splat (float 1.000000e+00), !dbg !35294
  %_17.i14.i.i = getelementptr inbounds nuw float, ptr %_54.0.i.i, i64 %base.i9.i289.i, !dbg !35301
  store <8 x float> %1848, ptr %_17.i14.i.i, align 4, !dbg !35303, !alias.scope !35308, !noalias !35312
  %base.i1424 = shl i64 %_212.i, 3, !dbg !35316
  %1849 = or disjoint i64 %base.i1424, 7, !dbg !35319
  %or.cond.i1428.not = icmp ult i64 %1849, %_54.1.i.i, !dbg !35319
  br i1 %or.cond.i1428.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1432, label %bb4.i1431, !dbg !35319, !prof !2723

bb4.i1431:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i
  store <8 x float> %history.i209.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.10.sroa.0.0.lcssa, ptr %history.i209.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.13.sroa.0.0.lcssa, ptr %history.i209.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa20524, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015045.lcssa15201.lcssa20545, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1346.lcssa1508915230.lcssa20567, ptr %_22.i301.i, align 4
  store i32 %storemerge.i1311.lcssa1514615302.lcssa20592, ptr %_22.i.i, align 4
  %_5.i1425 = add i64 %base.i1424, 8, !dbg !35323
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1424, i64 noundef %_5.i1425, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !35324, !noalias !35325
  unreachable, !dbg !35324

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1432: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i
  %_15.i1430 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i64 %base.i1424, !dbg !35333
  %lanes.i5253.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1430, align 4, !dbg !35335, !alias.scope !35340, !noalias !35344
  %position.i1305 = zext i32 %storemerge.i131115124 to i64, !dbg !35348
  %1850 = icmp eq i32 %storemerge.i131115124, 0, !dbg !35349
  br i1 %1850, label %bb5.i1307, label %bb3.i1306, !dbg !35349

bb3.i1306:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1432
  %1851 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %minimum.i.i.sroa.0.01504515060, <8 x float> %lanes.i5253.sroa.0.0.copyload), !dbg !35350
  br label %bb5.i1307, !dbg !35355

bb5.i1307:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1432, %bb3.i1306
  %minimum.i.i.sroa.0.0 = phi <8 x float> [ %1851, %bb3.i1306 ], [ %lanes.i5253.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1432 ], !dbg !35356
  %_15.i1308 = add nuw nsw i64 %position.i1305, 1, !dbg !35357
  %complete.i1309 = icmp eq i64 %_15.i1308, %_18.i249.i, !dbg !35357
  br i1 %complete.i1309, label %bb19.i1317, label %bb7.i1310, !dbg !35358

bb7.i1310:                                        ; preds = %bb5.i1307
  %base.i1415 = shl i64 %_208.i, 3, !dbg !35359
  %1852 = or disjoint i64 %base.i1415, 7, !dbg !35361
  %or.cond.i1419.not = icmp ult i64 %1852, %_54.1.i.i, !dbg !35361
  br i1 %or.cond.i1419.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1423, label %bb4.i1422, !dbg !35361, !prof !2723

bb4.i1422:                                        ; preds = %bb7.i1310
  store <8 x float> %history.i209.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.10.sroa.0.0.lcssa, ptr %history.i209.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.13.sroa.0.0.lcssa, ptr %history.i209.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa20524, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015045.lcssa15201.lcssa20545, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1346.lcssa1508915230.lcssa20567, ptr %_22.i301.i, align 4
  store i32 %storemerge.i1311.lcssa1514615302.lcssa20592, ptr %_22.i.i, align 4
  %_5.i1416 = add i64 %base.i1415, 8, !dbg !35365
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1415, i64 noundef %_5.i1416, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !35366, !noalias !35367
  unreachable, !dbg !35366

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1423: ; preds = %bb7.i1310
  %_15.i1421 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i64 %base.i1415, !dbg !35371
  %lanes.i5260.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1421, align 4, !dbg !35373, !alias.scope !35378, !noalias !35382
  %1853 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %lanes.i5260.sroa.0.0.copyload, <8 x float> %minimum.i.i.sroa.0.0), !dbg !35386
  %1854 = trunc i64 %_15.i1308 to i32, !dbg !35391
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1329, !dbg !35392

bb19.i1317:                                       ; preds = %bb5.i1307, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1405
  %end.sroa.0.0.i131515030 = phi i64 [ %1858, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1405 ], [ %_212.i, %bb5.i1307 ]
  %iter.sroa.0.0.i131415029 = phi i64 [ %_30.i1318, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1405 ], [ 0, %bb5.i1307 ]
  %suffix.i1298.sroa.0.015028 = phi <8 x float> [ %1856, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1405 ], [ %lanes.i5253.sroa.0.0.copyload, %bb5.i1307 ]
  %base.i1397 = shl i64 %end.sroa.0.0.i131515030, 3, !dbg !35393
  %1855 = or disjoint i64 %base.i1397, 7, !dbg !35395
  %or.cond.i1401.not = icmp ult i64 %1855, %_54.1.i.i, !dbg !35395
  br i1 %or.cond.i1401.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1405, label %bb4.i1404, !dbg !35395, !prof !2723

bb4.i1404:                                        ; preds = %bb19.i1317
  store <8 x float> %history.i209.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.10.sroa.0.0.lcssa, ptr %history.i209.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.13.sroa.0.0.lcssa, ptr %history.i209.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa20524, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015045.lcssa15201.lcssa20545, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1346.lcssa1508915230.lcssa20567, ptr %_22.i301.i, align 4
  store i32 %storemerge.i1311.lcssa1514615302.lcssa20592, ptr %_22.i.i, align 4
  %_5.i1398 = add i64 %base.i1397, 8, !dbg !35399
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1397, i64 noundef %_5.i1398, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !35400, !noalias !35401
  unreachable, !dbg !35400

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1405: ; preds = %bb19.i1317
  %_30.i1318 = add nuw i64 %iter.sroa.0.0.i131415029, 1, !dbg !35405
  %_15.i1403 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i64 %base.i1397, !dbg !35410
  %lanes.i5274.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1403, align 4, !dbg !35412, !alias.scope !35417, !noalias !35421
  %1856 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %suffix.i1298.sroa.0.015028, <8 x float> %lanes.i5274.sroa.0.0.copyload), !dbg !35425
  store <8 x float> %1856, ptr %_15.i1403, align 4, !dbg !35430, !alias.scope !35436, !noalias !35440
  %1857 = icmp eq i64 %end.sroa.0.0.i131515030, 0, !dbg !35444
  %spec.store.select.i1326 = select i1 %1857, i64 %ring.i, i64 %end.sroa.0.0.i131515030, !dbg !35444
  %1858 = add i64 %spec.store.select.i1326, -1, !dbg !35445
  %exitcond17573.not = icmp eq i64 %_30.i1318, %_18.i249.i, !dbg !35446
  br i1 %exitcond17573.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1329, label %bb19.i1317, !dbg !35448

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1329: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1405, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1423
  %minimum.i.i.sroa.0.1 = phi <8 x float> [ %1853, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1423 ], [ %minimum.i.i.sroa.0.0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1405 ], !dbg !35356
  %storemerge.i1311 = phi i32 [ %1854, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1423 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1405 ], !dbg !35449
  %1859 = fmul <8 x float> %minimum.i.i.sroa.0.1, splat (float 1.638400e+04), !dbg !35450
  %1860 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %1859), !dbg !35455
  %1861 = fmul <8 x float> %1860, splat (float 0x3F10000000000000), !dbg !35460
  %base.i1550 = shl i64 %_214.i, 3, !dbg !35465
  %1862 = or disjoint i64 %base.i1550, 7, !dbg !35467
  %or.cond.i1554.not = icmp ult i64 %1862, %_56.1.i.i, !dbg !35467
  br i1 %or.cond.i1554.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1558, label %bb4.i1557, !dbg !35467, !prof !2723

bb4.i1557:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1329
  store <8 x float> %history.i209.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.10.sroa.0.0.lcssa, ptr %history.i209.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.13.sroa.0.0.lcssa, ptr %history.i209.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa20524, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015045.lcssa15201.lcssa20545, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1346.lcssa1508915230.lcssa20567, ptr %_22.i301.i, align 4
  store i32 %storemerge.i1311.lcssa1514615302.lcssa20592, ptr %_22.i.i, align 4
  %_5.i1551 = add i64 %base.i1550, 8, !dbg !35471
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1550, i64 noundef %_5.i1551, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !35472, !noalias !35473
  unreachable, !dbg !35472

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1558: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1329
  %_15.i1556 = getelementptr inbounds nuw float, ptr %_56.0.i.i, i64 %base.i1550, !dbg !35477
  %lanes.i5155.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1556, align 4, !dbg !35479, !alias.scope !35484, !noalias !35488
  %1863 = fadd <8 x float> %1861, %_33.i245.i.sroa.0.0.copyload15152, !dbg !35492
  %1864 = fsub <8 x float> %1863, %lanes.i5155.sroa.0.0.copyload, !dbg !35497
  %_8.not.i4.i.i = icmp ugt i64 %_7.i10.i290.i, %_56.1.i.i
  br i1 %_8.not.i4.i.i, label %bb4.i7.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i, !dbg !35502, !prof !165

bb4.i7.i.i:                                       ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1558
  store <8 x float> %history.i209.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.10.sroa.0.0.lcssa, ptr %history.i209.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.13.sroa.0.0.lcssa, ptr %history.i209.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa20524, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015045.lcssa15201.lcssa20545, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1346.lcssa1508915230.lcssa20567, ptr %_22.i301.i, align 4
  store i32 %storemerge.i1311.lcssa1514615302.lcssa20592, ptr %_22.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i289.i, i64 noundef %_7.i10.i290.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8421dc8ec9e43c41e5b11981eccec9a3) #30, !dbg !35507, !noalias !35508
  unreachable, !dbg !35507

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1558
  %_17.i6.i.i = getelementptr inbounds nuw float, ptr %_56.0.i.i, i64 %base.i9.i289.i, !dbg !35512
  store <8 x float> %1861, ptr %_17.i6.i.i, align 4, !dbg !35514, !alias.scope !35519, !noalias !35523
  %_41.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1577, align 32, !dbg !35527
  %1865 = fdiv <8 x float> %1864, %_37.i.i.sroa.0.0.copyload, !dbg !35528
  %1866 = fsub <8 x float> splat (float 1.000000e+00), %1865, !dbg !35533
  %1867 = fsub <8 x float> %1866, %_41.i.i.sroa.0.0.copyload, !dbg !35538
  %1868 = fmul <8 x float> %_9.i.i.sroa.0.0.copyload.pre, %1867, !dbg !35543
  %1869 = fadd <8 x float> %_41.i.i.sroa.0.0.copyload, %1868, !dbg !35548
  %1870 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1866, <8 x float> %1869), !dbg !35552
  %1871 = bitcast <8 x float> %1870 to <8 x i32>, !dbg !35557
  %1872 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1870), !dbg !35563
  %1873 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1872, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !35565
  %1874 = bitcast <8 x float> %1873 to <8 x i32>, !dbg !35571
  %1875 = xor <8 x i32> %1874, splat (i32 -1), !dbg !35577
  %1876 = and <8 x i32> %1875, %1871, !dbg !35579
  store <8 x i32> %1876, ptr %1577, align 32, !dbg !35583
  %_6.not.i1544 = icmp ugt i64 %_5.i1560, %_58.1.i.i
  br i1 %_6.not.i1544, label %bb4.i1548, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1549, !dbg !35584, !prof !165

bb4.i1548:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i
  store <8 x float> %history.i209.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.10.sroa.0.0.lcssa, ptr %history.i209.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.13.sroa.0.0.lcssa, ptr %history.i209.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa20524, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015045.lcssa15201.lcssa20545, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1346.lcssa1508915230.lcssa20567, ptr %_22.i301.i, align 4
  store i32 %storemerge.i1311.lcssa1514615302.lcssa20592, ptr %_22.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1559, i64 noundef %_5.i1560, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !35589, !noalias !35590
  unreachable, !dbg !35589

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1549: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i
  %1877 = bitcast <8 x i32> %1876 to <8 x float>, !dbg !35594
  %1878 = fsub <8 x float> splat (float 1.000000e+00), %1877, !dbg !35595
  %_15.i1547 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %base.i1559, !dbg !35600
  %lanes.i5162.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1547, align 4, !dbg !35602, !alias.scope !35607, !noalias !35611
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_15.i1547, ptr noundef nonnull align 4 dereferenceable(32) %data.i5.i.i.i.i.i.i.i7094, i64 32, i1 false), !dbg !35615
  %1879 = fmul <8 x float> %1878, %lanes.i5162.sroa.0.0.copyload, !dbg !35621
  %1880 = select <8 x i1> %1570, <8 x float> %lanes.i5162.sroa.0.0.copyload, <8 x float> %1879, !dbg !35626
  store <8 x float> %1880, ptr %data.i5.i.i.i.i.i.i.i7094, align 4, !dbg !35631, !alias.scope !35636, !noalias !35640
  %exitcond17586.not = icmp eq i64 %1801, %1800, !dbg !34793
  br i1 %exitcond17586.not, label %bb32.i, label %bb31.i, !dbg !34793

bb32.i:                                           ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1549, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit7064
  %.lcssa1517615338 = phi <8 x float> [ %.lcssa1517615339, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit7064 ], [ %1864, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1549 ]
  %storemerge.i1311.lcssa1514615302 = phi i32 [ %storemerge.i1311.lcssa1514615303, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit7064 ], [ %storemerge.i1311, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1549 ]
  %.lcssa1511915266 = phi <8 x float> [ %.lcssa1511915267, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit7064 ], [ %1826, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1549 ]
  %storemerge.i1346.lcssa1508915230 = phi i32 [ %storemerge.i1346.lcssa1508915231, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit7064 ], [ %storemerge.i1346, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1549 ]
  %minimum.i.i.sroa.0.015045.lcssa = phi <8 x float> [ %minimum.i.i.sroa.0.015045.lcssa1520115220, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit7064 ], [ %minimum.i.i.sroa.0.0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1549 ]
  %minimum.i281.i.sroa.0.015031.lcssa = phi <8 x float> [ %minimum.i281.i.sroa.0.015031.lcssa1518215221, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit7064 ], [ %minimum.i281.i.sroa.0.0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1549 ]
  %_138.i = add i64 %..i6989, %ring_cursor.sroa.0.1.i15222, !dbg !35644
  %_204.not.i = icmp ult i64 %_138.i, %ring.i, !dbg !35645
  %1881 = select i1 %_204.not.i, i64 0, i64 %ring.i, !dbg !35645
  %ring_cursor.sroa.0.2.i = sub nuw i64 %_138.i, %1881, !dbg !35645
  %_140.i = add i64 %..i6989, %main_cursor.sroa.0.1.i15223, !dbg !35648
  %_215.not.i = icmp ult i64 %_140.i, %main.i, !dbg !35649
  %1882 = select i1 %_215.not.i, i64 0, i64 %main.i, !dbg !35649
  %main_cursor.sroa.0.2.i = sub nuw i64 %_140.i, %1882, !dbg !35649
  %_58.i = icmp ult i64 %_80.i, %..i6958, !dbg !33554
  br i1 %_58.i, label %bb20.i, label %bb19.i.bb15.i.loopexit_crit_edge, !dbg !33554

_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit: ; preds = %bb15.i.loopexit
  store <8 x float> %history.i209.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.10.sroa.0.0.lcssa, ptr %history.i209.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i209.i.sroa.13.sroa.0.0.lcssa, ptr %history.i209.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33589
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33590
  store <8 x float> %minimum.i281.i.sroa.0.015031.lcssa15182.lcssa, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015045.lcssa15201.lcssa, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1346.lcssa1508915230.lcssa20566, ptr %_22.i301.i, align 4
  store i32 %storemerge.i1311.lcssa1514615302.lcssa20591, ptr %_22.i.i, align 4
  %1883 = trunc i64 %main_cursor.sroa.0.1.i.lcssa to i32, !dbg !35651
  %1884 = trunc i64 %ring_cursor.sroa.0.1.i.lcssa to i32, !dbg !35653
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, !dbg !35654

_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit, %bb6.i
  %ring_cursor.sroa.0.0.i.lcssa = phi i32 [ %_36.i, %bb6.i ], [ %1884, %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit ], !dbg !33511
  %main_cursor.sroa.0.0.i.lcssa = phi i32 [ %_35.i, %bb6.i ], [ %1883, %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit ], !dbg !33508
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %left_prefix.i, ptr noundef nonnull align 32 dereferenceable(32) %uniform_left.i, i64 32, i1 false), !dbg !35654
  %1885 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 104, !dbg !35655
  %left_phase.i = load i32, ptr %1885, align 8, !dbg !35655, !noalias !33515, !noundef !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %right_prefix.i, ptr noundef nonnull align 32 dereferenceable(32) %uniform_right.i, i64 32, i1 false), !dbg !35656
  %1886 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 104, !dbg !35657
  %right_phase.i = load i32, ptr %1886, align 8, !dbg !35657, !noalias !33515, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_right.i), !dbg !35658, !noalias !33515
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i), !dbg !35659, !noalias !33515
  %1887 = getelementptr inbounds nuw i8, ptr %self, i64 1736, !dbg !35660
  %_230.1.i = load i64, ptr %1887, align 8, !dbg !35660, !alias.scope !33482, !noalias !35661, !noundef !12
  %_8.i6223 = icmp samesign ugt i64 %_230.1.i, 7, !dbg !35662
  br i1 %_8.i6223, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6226, label %bb2.i6224, !dbg !35662, !prof !651

bb2.i6224:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_230.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !35667, !noalias !35668
  unreachable, !dbg !35667

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6226: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit
  %1888 = getelementptr inbounds nuw i8, ptr %self, i64 1728, !dbg !35660
  %_230.0.i = load ptr, ptr %1888, align 8, !dbg !35660, !alias.scope !33482, !noalias !35661, !nonnull !12, !noundef !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_230.0.i, ptr noundef nonnull align 32 dereferenceable(32) %left_prefix.i, i64 32, i1 false), !dbg !35672
  %_231.0.i = load ptr, ptr %68, align 8, !dbg !35676, !alias.scope !33482, !noalias !35661, !nonnull !12, !noundef !12
  %_231.1.i = load i64, ptr %69, align 8, !dbg !35676, !alias.scope !33482, !noalias !35661, !noundef !12
  %1889 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i), !dbg !35677
  br i1 %1889, label %bb2.i7142, label %bb6.i7133, !dbg !35677

bb6.i7133:                                        ; preds = %bb2.i7142, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6226
  %end_or_len.idx.i7134 = shl nuw nsw i64 %_231.1.i, 2, !dbg !35681
  %end_or_len.i7135 = getelementptr inbounds nuw i8, ptr %_231.0.i, i64 %end_or_len.idx.i7134, !dbg !35681
  %_293.i7136 = icmp eq i64 %_231.1.i, 0, !dbg !35685
  br i1 %_293.i7136, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7148, label %bb10.i7137, !dbg !35688

bb2.i7142:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6226
  %bytes1.sroa.0.0.zext.i7143 = and i32 %left_phase.i, 255, !dbg !35689
  %bytes1.sroa.0.0.isplat.i7144 = mul nuw i32 %bytes1.sroa.0.0.zext.i7143, 16843009, !dbg !35689
  %_5.i7145 = icmp eq i32 %left_phase.i, %bytes1.sroa.0.0.isplat.i7144, !dbg !35690
  br i1 %_5.i7145, label %bb3.i7146, label %bb6.i7133, !dbg !35690

bb3.i7146:                                        ; preds = %bb2.i7142
  %bytes.sroa.0.0.extract.trunc.i7147 = trunc i32 %left_phase.i to i8, !dbg !35691
  %1890 = shl nuw nsw i64 %_231.1.i, 2, !dbg !35693
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_231.0.i, i8 %bytes.sroa.0.0.extract.trunc.i7147, i64 %1890, i1 false), !dbg !35693, !alias.scope !35694, !noalias !33486
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7148, !dbg !35697

bb10.i7137:                                       ; preds = %bb6.i7133, %bb10.i7137
  %iter.sroa.0.04.i7138 = phi ptr [ %_38.i7139, %bb10.i7137 ], [ %_231.0.i, %bb6.i7133 ]
  %_38.i7139 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i7138, i64 4, !dbg !35698
  store i32 %left_phase.i, ptr %iter.sroa.0.04.i7138, align 4, !dbg !35700, !alias.scope !35694, !noalias !33486
  %_29.i7140 = icmp eq ptr %_38.i7139, %end_or_len.i7135, !dbg !35685
  br i1 %_29.i7140, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7148, label %bb10.i7137, !dbg !35688

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7148: ; preds = %bb10.i7137, %bb6.i7133, %bb3.i7146
  %1891 = getelementptr inbounds nuw i8, ptr %self, i64 1936, !dbg !35701
  %_232.1.i = load i64, ptr %1891, align 8, !dbg !35701, !alias.scope !33484, !noalias !35702, !noundef !12
  %_8.i6218 = icmp samesign ugt i64 %_232.1.i, 7, !dbg !35703
  br i1 %_8.i6218, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6221, label %bb2.i6219, !dbg !35703, !prof !651

bb2.i6219:                                        ; preds = %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7148
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_232.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !35708, !noalias !35709
  unreachable, !dbg !35708

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6221: ; preds = %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7148
  %1892 = getelementptr inbounds nuw i8, ptr %self, i64 1928, !dbg !35701
  %_232.0.i = load ptr, ptr %1892, align 8, !dbg !35701, !alias.scope !33484, !noalias !35702, !nonnull !12, !noundef !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_232.0.i, ptr noundef nonnull align 32 dereferenceable(32) %right_prefix.i, i64 32, i1 false), !dbg !35713
  %_233.0.i = load ptr, ptr %77, align 8, !dbg !35717, !alias.scope !33484, !noalias !35702, !nonnull !12, !noundef !12
  %_233.1.i = load i64, ptr %78, align 8, !dbg !35717, !alias.scope !33484, !noalias !35702, !noundef !12
  %1893 = tail call i1 @llvm.is.constant.i32(i32 %right_phase.i), !dbg !35718
  br i1 %1893, label %bb2.i7159, label %bb6.i7150, !dbg !35718

bb6.i7150:                                        ; preds = %bb2.i7159, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6221
  %end_or_len.idx.i7151 = shl nuw nsw i64 %_233.1.i, 2, !dbg !35721
  %end_or_len.i7152 = getelementptr inbounds nuw i8, ptr %_233.0.i, i64 %end_or_len.idx.i7151, !dbg !35721
  %_293.i7153 = icmp eq i64 %_233.1.i, 0, !dbg !35725
  br i1 %_293.i7153, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7165, label %bb10.i7154, !dbg !35728

bb2.i7159:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6221
  %bytes1.sroa.0.0.zext.i7160 = and i32 %right_phase.i, 255, !dbg !35729
  %bytes1.sroa.0.0.isplat.i7161 = mul nuw i32 %bytes1.sroa.0.0.zext.i7160, 16843009, !dbg !35729
  %_5.i7162 = icmp eq i32 %right_phase.i, %bytes1.sroa.0.0.isplat.i7161, !dbg !35730
  br i1 %_5.i7162, label %bb3.i7163, label %bb6.i7150, !dbg !35730

bb3.i7163:                                        ; preds = %bb2.i7159
  %bytes.sroa.0.0.extract.trunc.i7164 = trunc i32 %right_phase.i to i8, !dbg !35731
  %1894 = shl nuw nsw i64 %_233.1.i, 2, !dbg !35733
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_233.0.i, i8 %bytes.sroa.0.0.extract.trunc.i7164, i64 %1894, i1 false), !dbg !35733, !alias.scope !35734, !noalias !33486
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7165, !dbg !35737

bb10.i7154:                                       ; preds = %bb6.i7150, %bb10.i7154
  %iter.sroa.0.04.i7155 = phi ptr [ %_38.i7156, %bb10.i7154 ], [ %_233.0.i, %bb6.i7150 ]
  %_38.i7156 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i7155, i64 4, !dbg !35738
  store i32 %right_phase.i, ptr %iter.sroa.0.04.i7155, align 4, !dbg !35740, !alias.scope !35734, !noalias !33486
  %_29.i7157 = icmp eq ptr %_38.i7156, %end_or_len.i7152, !dbg !35725
  br i1 %_29.i7157, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7165, label %bb10.i7154, !dbg !35728

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7165: ; preds = %bb10.i7154, %bb6.i7150, %bb3.i7163
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_left.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33) #31, !dbg !35741
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_right.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34) #31, !dbg !35742
  store i32 %main_cursor.sroa.0.0.i.lcssa, ptr %_35, align 4, !dbg !35651, !alias.scope !33486, !noalias !33510
  store i32 %ring_cursor.sroa.0.0.i.lcssa, ptr %87, align 4, !dbg !35653, !alias.scope !33486, !noalias !33510
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i), !dbg !35743, !noalias !33515
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i), !dbg !35744, !noalias !33515
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, !dbg !33479

_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit6953, %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7165
  br i1 %quiet.sroa.0.012328, label %bb28, label %bb40, !dbg !35745

bb22:                                             ; preds = %bb20
  tail call void @llvm.experimental.noalias.scope.decl(metadata !35746), !dbg !35749
  %1895 = getelementptr inbounds nuw i8, ptr %self, i64 1760, !dbg !35750
  %_40.0.i = load ptr, ptr %1895, align 8, !dbg !35750, !alias.scope !35746, !nonnull !12, !noundef !12
  %1896 = getelementptr inbounds nuw i8, ptr %self, i64 1768, !dbg !35750
  %_40.1.i = load i64, ptr %1896, align 8, !dbg !35750, !alias.scope !35746, !noundef !12
  %1897 = getelementptr inbounds nuw i8, ptr %self, i64 1824, !dbg !35752
  %_41.0.i = load ptr, ptr %1897, align 8, !dbg !35752, !alias.scope !35746, !nonnull !12, !noundef !12
  %1898 = getelementptr inbounds nuw i8, ptr %self, i64 1832, !dbg !35752
  %_41.1.i = load i64, ptr %1898, align 8, !dbg !35752, !alias.scope !35746, !noundef !12
  %..i.i.i.i = tail call noundef i64 @llvm.umin.i64(i64 %_41.1.i, i64 %_40.1.i), !dbg !35753
  %_2.i6.not.i = icmp eq i64 %..i.i.i.i, 0, !dbg !35759
  br i1 %_2.i6.not.i, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb4.i7166, !dbg !35759

bb4.i7166:                                        ; preds = %bb22, %bb6.i7167
  %iter.sroa.8.07.i = phi i64 [ %1899, %bb6.i7167 ], [ 0, %bb22 ]
  %_3.i1.i.i = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i, i64 %iter.sroa.8.07.i, !dbg !35762
  %_14.i = load i32, ptr %_3.i1.i.i, align 4, !dbg !35765, !noalias !35746, !noundef !12
  %_20.i = icmp eq i32 %_14.i, 0, !dbg !35766
  br i1 %_20.i, label %panic.i7176, label %bb6.i7167, !dbg !35766

bb6.i7167:                                        ; preds = %bb4.i7166
  %_3.i.i.i7168 = getelementptr inbounds nuw i32, ptr %_40.0.i, i64 %iter.sroa.8.07.i, !dbg !35767
  %1899 = add nuw i64 %iter.sroa.8.07.i, 1, !dbg !35770
  %window.i7169 = zext i32 %_14.i to i64, !dbg !35765
  %_18.i7170 = load i32, ptr %_3.i.i.i7168, align 4, !dbg !35771, !noalias !35746, !noundef !12
  %_17.i7171 = zext i32 %_18.i7170 to i64, !dbg !35771
  %_19.i7172 = urem i64 %frames, %window.i7169, !dbg !35766
  %_16.i7173 = add nuw nsw i64 %_19.i7172, %_17.i7171, !dbg !35772
  %_15.i7174 = urem i64 %_16.i7173, %window.i7169, !dbg !35773
  %1900 = trunc nuw i64 %_15.i7174 to i32, !dbg !35774
  store i32 %1900, ptr %_3.i.i.i7168, align 4, !dbg !35774, !noalias !35746
  %exitcond.not.i = icmp eq i64 %1899, %..i.i.i.i, !dbg !35759
  br i1 %exitcond.not.i, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb4.i7166, !dbg !35759

panic.i7176:                                      ; preds = %bb4.i7166
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bac57976a2bdbfad4a3a85d5d1c7648c) #30, !dbg !35766, !noalias !35746
  unreachable, !dbg !35766

_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit: ; preds = %bb6.i7167, %bb22
  tail call void @llvm.experimental.noalias.scope.decl(metadata !35775), !dbg !35778
  %1901 = getelementptr inbounds nuw i8, ptr %self, i64 1960, !dbg !35779
  %_40.0.i7177 = load ptr, ptr %1901, align 8, !dbg !35779, !alias.scope !35775, !nonnull !12, !noundef !12
  %1902 = getelementptr inbounds nuw i8, ptr %self, i64 1968, !dbg !35779
  %_40.1.i7178 = load i64, ptr %1902, align 8, !dbg !35779, !alias.scope !35775, !noundef !12
  %1903 = getelementptr inbounds nuw i8, ptr %self, i64 2024, !dbg !35781
  %_41.0.i7179 = load ptr, ptr %1903, align 8, !dbg !35781, !alias.scope !35775, !nonnull !12, !noundef !12
  %1904 = getelementptr inbounds nuw i8, ptr %self, i64 2032, !dbg !35781
  %_41.1.i7180 = load i64, ptr %1904, align 8, !dbg !35781, !alias.scope !35775, !noundef !12
  %..i.i.i.i7181 = tail call noundef i64 @llvm.umin.i64(i64 %_41.1.i7180, i64 %_40.1.i7178), !dbg !35782
  %_2.i6.not.i7182 = icmp eq i64 %..i.i.i.i7181, 0, !dbg !35788
  br i1 %_2.i6.not.i7182, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit7199, label %bb4.i7183, !dbg !35788

bb4.i7183:                                        ; preds = %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, %bb6.i7188
  %iter.sroa.8.07.i7184 = phi i64 [ %1905, %bb6.i7188 ], [ 0, %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit ]
  %_3.i1.i.i7185 = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i7179, i64 %iter.sroa.8.07.i7184, !dbg !35791
  %_14.i7186 = load i32, ptr %_3.i1.i.i7185, align 4, !dbg !35794, !noalias !35775, !noundef !12
  %_20.i7187 = icmp eq i32 %_14.i7186, 0, !dbg !35795
  br i1 %_20.i7187, label %panic.i7198, label %bb6.i7188, !dbg !35795

bb6.i7188:                                        ; preds = %bb4.i7183
  %_3.i.i.i7189 = getelementptr inbounds nuw i32, ptr %_40.0.i7177, i64 %iter.sroa.8.07.i7184, !dbg !35796
  %1905 = add nuw i64 %iter.sroa.8.07.i7184, 1, !dbg !35799
  %window.i7190 = zext i32 %_14.i7186 to i64, !dbg !35794
  %_18.i7191 = load i32, ptr %_3.i.i.i7189, align 4, !dbg !35800, !noalias !35775, !noundef !12
  %_17.i7192 = zext i32 %_18.i7191 to i64, !dbg !35800
  %_19.i7193 = urem i64 %frames, %window.i7190, !dbg !35795
  %_16.i7194 = add nuw nsw i64 %_19.i7193, %_17.i7192, !dbg !35801
  %_15.i7195 = urem i64 %_16.i7194, %window.i7190, !dbg !35802
  %1906 = trunc nuw i64 %_15.i7195 to i32, !dbg !35803
  store i32 %1906, ptr %_3.i.i.i7189, align 4, !dbg !35803, !noalias !35775
  %exitcond.not.i7196 = icmp eq i64 %1905, %..i.i.i.i7181, !dbg !35788
  br i1 %exitcond.not.i7196, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit7199, label %bb4.i7183, !dbg !35788

panic.i7198:                                      ; preds = %bb4.i7183
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bac57976a2bdbfad4a3a85d5d1c7648c) #30, !dbg !35795, !noalias !35775
  unreachable, !dbg !35795

_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit7199: ; preds = %bb6.i7188, %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit
  %1907 = getelementptr inbounds nuw i8, ptr %self, i64 1624, !dbg !35804
  %_29.val = load i64, ptr %1907, align 8, !dbg !35804
  %1908 = getelementptr inbounds nuw i8, ptr %self, i64 1632, !dbg !35804
  %_29.val6478 = load i64, ptr %1908, align 8, !dbg !35804, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !35805), !dbg !35804
  %_10.i7200 = icmp eq i64 %_29.val6478, 0, !dbg !35808
  br i1 %_10.i7200, label %panic.i7212, label %bb1.i7201, !dbg !35808

bb1.i7201:                                        ; preds = %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit7199
  %_28 = getelementptr inbounds nuw i8, ptr %self, i64 1640, !dbg !35810
  %_7.i = load i32, ptr %_28, align 4, !dbg !35811, !alias.scope !35805, !noundef !12
  %_6.i7202 = zext i32 %_7.i to i64, !dbg !35811
  %_8.i7203 = urem i64 %frames, %_29.val6478, !dbg !35808
  %_5.i7204 = add nuw nsw i64 %_8.i7203, %_6.i7202, !dbg !35812
  %_4.i7205 = urem i64 %_5.i7204, %_29.val6478, !dbg !35813
  %1909 = trunc i64 %_4.i7205 to i32, !dbg !35814
  store i32 %1909, ptr %_28, align 4, !dbg !35814, !alias.scope !35805
  %_17.i7206 = icmp eq i64 %_29.val, 0, !dbg !35815
  br i1 %_17.i7206, label %panic2.i, label %_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit, !dbg !35815

panic.i7212:                                      ; preds = %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit7199
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f0ee36f67d9a332211aa5518dd2ebfd5) #30, !dbg !35808, !noalias !35805
  unreachable, !dbg !35808

panic2.i:                                         ; preds = %bb1.i7201
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_33d4d33e0a850133578789055882dcf9) #30, !dbg !35815, !noalias !35805
  unreachable, !dbg !35815

_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit: ; preds = %bb1.i7201
  %1910 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !35816
  %_14.i7208 = load i32, ptr %1910, align 4, !dbg !35816, !alias.scope !35805, !noundef !12
  %_13.i7209 = zext i32 %_14.i7208 to i64, !dbg !35816
  %_15.i7210 = urem i64 %frames, %_29.val, !dbg !35815
  %_12.i = add nuw nsw i64 %_15.i7210, %_13.i7209, !dbg !35817
  %_11.i7211 = urem i64 %_12.i, %_29.val, !dbg !35818
  %1911 = trunc i64 %_11.i7211 to i32, !dbg !35819
  store i32 %1911, ptr %1910, align 4, !dbg !35819, !alias.scope !35805
  br label %bb42, !dbg !35820

bb28:                                             ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_37 = tail call noundef zeroext i1 @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #31, !dbg !35821
  br i1 %_37, label %bb30, label %bb40, !dbg !35822

bb30:                                             ; preds = %bb28
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_39 = tail call noundef zeroext i1 @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #31, !dbg !35823
  br i1 %_39, label %bb32, label %bb40, !dbg !35824

bb32:                                             ; preds = %bb30
  %_80.not = icmp samesign ugt i64 %words, %left_io.1
  br i1 %_80.not, label %bb51, label %bb1.i7213, !dbg !35825, !prof !165

bb51:                                             ; preds = %bb32
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %words, i64 noundef %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_84c34a80eea34d71b95e4ffce11a67a7) #30, !dbg !35833
  unreachable, !dbg !35833

bb1.i7213:                                        ; preds = %bb32, %bb10.i7227
  %iter.sroa.6.0.i7214 = phi i64 [ %len.i.i.i.i7219, %bb10.i7227 ], [ %words, %bb32 ], !dbg !35834
  %iter.sroa.0.0.i7215 = phi ptr [ %data.i.i.i.i7218, %bb10.i7227 ], [ %left_io.0, %bb32 ], !dbg !35834
  %1912 = icmp eq i64 %iter.sroa.6.0.i7214, 0, !dbg !35836
  br i1 %1912, label %bb34, label %bb11.preheader.i7216, !dbg !35836

bb11.preheader.i7216:                             ; preds = %bb1.i7213
  %..i.i.i7217 = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i7214, i64 32), !dbg !35838
  %_18.idx.i7220 = shl nuw nsw i64 %..i.i.i7217, 2, !dbg !35841
  %_18.i7221 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i7215, i64 %_18.idx.i7220, !dbg !35841
  br label %bb11.i7222, !dbg !35846

bb11.i7222:                                       ; preds = %bb11.i7222, %bb11.preheader.i7216
  %iter1.sroa.0.014.i7223 = phi ptr [ %_31.i7225, %bb11.i7222 ], [ %iter.sroa.0.0.i7215, %bb11.preheader.i7216 ]
  %bits.sroa.0.013.i7224 = phi i32 [ %1913, %bb11.i7222 ], [ 0, %bb11.preheader.i7216 ]
  %_31.i7225 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i7223, i64 4, !dbg !35848
  %_134.i7226 = load i32, ptr %iter1.sroa.0.014.i7223, align 4, !dbg !35850, !alias.scope !35851, !noundef !12
  %1913 = or i32 %_134.i7226, %bits.sroa.0.013.i7224, !dbg !35854
  %_25.i = icmp eq ptr %_31.i7225, %_18.i7221, !dbg !35855
  br i1 %_25.i, label %bb10.i7227, label %bb11.i7222, !dbg !35846

bb10.i7227:                                       ; preds = %bb11.i7222
  %data.i.i.i.i7218 = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i7215, i64 %..i.i.i7217, !dbg !35857
  %len.i.i.i.i7219 = sub nuw nsw i64 %iter.sroa.6.0.i7214, %..i.i.i7217, !dbg !35862
  %1914 = icmp eq i32 %1913, 0, !dbg !35863
  br i1 %1914, label %bb1.i7213, label %bb40, !dbg !35863

bb34:                                             ; preds = %bb1.i7213
  %_88.not = icmp samesign ugt i64 %words, %right_io.1, !dbg !35864
  br i1 %_88.not, label %bb54, label %bb1.i7240, !dbg !35864, !prof !639

bb40:                                             ; preds = %bb10.i7227, %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, %bb28, %bb30, %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit7257
  %_36.sroa.0.0 = phi i8 [ %2005, %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit7257 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit ], [ 0, %bb30 ], [ 0, %bb28 ], [ 0, %bb10.i7227 ], !dbg !35870
  store i8 %_36.sroa.0.0, ptr %38, align 8, !dbg !35871
  %1915 = load i8, ptr %2, align 32, !dbg !35872, !range !17, !noundef !12
  store i8 %1915, ptr %0, align 1, !dbg !35873
  call void @llvm.lifetime.start.p0(ptr nonnull %shape), !dbg !35874
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %shape, ptr noundef nonnull align 16 dereferenceable(24) %_32, i64 24, i1 false), !dbg !35875
  %1916 = getelementptr inbounds nuw i8, ptr %self, i64 2120, !dbg !35876
  %1917 = load i32, ptr %1916, align 8, !dbg !35876, !noundef !12
  %_53 = getelementptr inbounds nuw i8, ptr %self, i64 1568, !dbg !35878
  %1918 = getelementptr inbounds nuw i8, ptr %self, i64 1584, !dbg !35885
  %_99.0 = load ptr, ptr %1918, align 16, !dbg !35885, !nonnull !12, !noundef !12
  %1919 = getelementptr inbounds nuw i8, ptr %self, i64 1592, !dbg !35885
  %_99.1 = load i64, ptr %1919, align 8, !dbg !35885, !noundef !12
  %1920 = getelementptr inbounds nuw i8, ptr %self, i64 1600, !dbg !35885
  %_100.0 = load ptr, ptr %1920, align 32, !dbg !35885, !nonnull !12, !noundef !12
  %1921 = getelementptr inbounds nuw i8, ptr %self, i64 1608, !dbg !35885
  %_100.1 = load i64, ptr %1921, align 8, !dbg !35885, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !35886), !dbg !35889
  tail call void @llvm.experimental.noalias.scope.decl(metadata !35890), !dbg !35889
  tail call void @llvm.experimental.noalias.scope.decl(metadata !35892), !dbg !35889
  %1922 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !35894
  %fst_len.i.i = and i64 %left_io.1, 2305843009213693944, !dbg !35903
  %1923 = bitcast <8 x float> %1922 to <8 x i32>, !dbg !35906
  %_22.not.i19207.i = icmp eq i64 %fst_len.i.i, 0, !dbg !35910
  br i1 %_22.not.i19207.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit25.i, label %bb13.i20.i, !dbg !35910

bb13.i20.i:                                       ; preds = %bb40, %bb13.i20.i
  %iter.sroa.0.0.i18210.i = phi ptr [ %_27.i21.i, %bb13.i20.i ], [ %left_io.0, %bb40 ]
  %iter.sroa.5.0.i17209.i = phi i64 [ %_28.i22.i, %bb13.i20.i ], [ %fst_len.i.i, %bb40 ]
  %ok.i9.sroa.0.0208.i = phi <8 x i32> [ %1928, %bb13.i20.i ], [ %1923, %bb40 ]
  %lanes.i.sroa.0.0.copyload.i = load <8 x i32>, ptr %iter.sroa.0.0.i18210.i, align 4, !dbg !35913, !alias.scope !35918, !noalias !35922
  %_27.i21.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i18210.i, i64 32, !dbg !35927
  %_28.i22.i = add i64 %iter.sroa.5.0.i17209.i, -8, !dbg !35930
  %1924 = and <8 x i32> %lanes.i.sroa.0.0.copyload.i, splat (i32 2147483647), !dbg !35931
  %1925 = bitcast <8 x i32> %1924 to <8 x float>, !dbg !35937
  %1926 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1925, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !35938
  %1927 = bitcast <8 x float> %1926 to <8 x i32>, !dbg !35906
  %1928 = and <8 x i32> %ok.i9.sroa.0.0208.i, %1927, !dbg !35944
  %_22.not.i19.i = icmp eq i64 %_28.i22.i, 0, !dbg !35910
  br i1 %_22.not.i19.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit25.i, label %bb13.i20.i, !dbg !35910

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit25.i: ; preds = %bb13.i20.i, %bb40
  %ok.i9.sroa.0.0.lcssa.i = phi <8 x i32> [ %1923, %bb40 ], [ %1928, %bb13.i20.i ], !dbg !35946
  %1929 = icmp sgt <8 x i32> %ok.i9.sroa.0.0.lcssa.i, splat (i32 -1), !dbg !35947
  %1930 = bitcast <8 x i1> %1929 to i8, !dbg !35947
  %_0.i90.not.i = icmp eq i8 %1930, 0, !dbg !35952
  br i1 %_0.i90.not.i, label %bb2.i7236, label %bb7.i7230, !dbg !35953

bb2.i7236:                                        ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit25.i
  %fst_len.i105.i = and i64 %right_io.1, 2305843009213693944, !dbg !35954
  %_22.not.i211.i = icmp eq i64 %fst_len.i105.i, 0, !dbg !35958
  br i1 %_22.not.i211.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %bb13.i.i7237, !dbg !35958

bb13.i.i7237:                                     ; preds = %bb2.i7236, %bb13.i.i7237
  %iter.sroa.0.0.i214.i = phi ptr [ %_27.i.i7238, %bb13.i.i7237 ], [ %right_io.0, %bb2.i7236 ]
  %iter.sroa.5.0.i213.i = phi i64 [ %_28.i.i7239, %bb13.i.i7237 ], [ %fst_len.i105.i, %bb2.i7236 ]
  %ok.i.sroa.0.0212.i = phi <8 x i32> [ %1935, %bb13.i.i7237 ], [ %1923, %bb2.i7236 ]
  %lanes.i49.sroa.0.0.copyload.i = load <8 x i32>, ptr %iter.sroa.0.0.i214.i, align 4, !dbg !35961, !alias.scope !35966, !noalias !35970
  %_27.i.i7238 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i214.i, i64 32, !dbg !35974
  %_28.i.i7239 = add i64 %iter.sroa.5.0.i213.i, -8, !dbg !35977
  %1931 = and <8 x i32> %lanes.i49.sroa.0.0.copyload.i, splat (i32 2147483647), !dbg !35978
  %1932 = bitcast <8 x i32> %1931 to <8 x float>, !dbg !35984
  %1933 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1932, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !35985
  %1934 = bitcast <8 x float> %1933 to <8 x i32>, !dbg !35991
  %1935 = and <8 x i32> %ok.i.sroa.0.0212.i, %1934, !dbg !35995
  %_22.not.i.i = icmp eq i64 %_28.i.i7239, 0, !dbg !35958
  br i1 %_22.not.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %bb13.i.i7237, !dbg !35958

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i: ; preds = %bb13.i.i7237, %bb2.i7236
  %ok.i.sroa.0.0.lcssa.i = phi <8 x i32> [ %1923, %bb2.i7236 ], [ %1935, %bb13.i.i7237 ], !dbg !35997
  %1936 = icmp sgt <8 x i32> %ok.i.sroa.0.0.lcssa.i, splat (i32 -1), !dbg !35998
  %1937 = bitcast <8 x i1> %1936 to i8, !dbg !35998
  %_0.i93.not.i = icmp eq i8 %1937, 0, !dbg !36003
  br i1 %_0.i93.not.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB1z_11LimiterCoreBR_E13process_block0EB1z_.exit, label %bb7.i7230, !dbg !36004

bb7.i7230:                                        ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit25.i
  br i1 %_22.not.i19207.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %bb23.i.i, !dbg !36005

bb23.i.i:                                         ; preds = %bb7.i7230, %bb23.i.i
  %iter.sroa.0.074.i.i = phi ptr [ %_45.i.i, %bb23.i.i ], [ %left_io.0, %bb7.i7230 ]
  %iter.sroa.5.073.i.i = phi i64 [ %_46.i.i7231, %bb23.i.i ], [ %fst_len.i.i, %bb7.i7230 ]
  %ok.sroa.0.072.i.i = phi <8 x i32> [ %1942, %bb23.i.i ], [ %1923, %bb7.i7230 ]
  %lanes.i.sroa.0.0.copyload.i.i = load <8 x i32>, ptr %iter.sroa.0.074.i.i, align 4, !dbg !36009, !alias.scope !36014, !noalias !36020
  %_45.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.074.i.i, i64 32, !dbg !36024
  %_46.i.i7231 = add i64 %iter.sroa.5.073.i.i, -8, !dbg !36027
  %1938 = and <8 x i32> %lanes.i.sroa.0.0.copyload.i.i, splat (i32 2147483647), !dbg !36028
  %1939 = bitcast <8 x i32> %1938 to <8 x float>, !dbg !36034
  %1940 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1939, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !36035
  %1941 = bitcast <8 x float> %1940 to <8 x i32>, !dbg !36041
  %1942 = and <8 x i32> %ok.sroa.0.072.i.i, %1941, !dbg !36045
  %_40.not.i.i = icmp eq i64 %_46.i.i7231, 0, !dbg !36005
  br i1 %_40.not.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %bb23.i.i, !dbg !36005

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i: ; preds = %bb23.i.i, %bb7.i7230
  %ok.sroa.0.0.lcssa.i.i = phi <8 x i32> [ %1923, %bb7.i7230 ], [ %1942, %bb23.i.i ], !dbg !36047
  %1943 = icmp slt <8 x i32> %ok.sroa.0.0.lcssa.i.i, zeroinitializer, !dbg !36048
  %bc.i.i = select <8 x i1> %1943, <8 x i32> zeroinitializer, <8 x i32> splat (i32 1065353216), !dbg !36048
  %1944 = extractelement <8 x i32> %bc.i.i, i64 0, !dbg !36053
  %1945 = icmp ne i32 %1944, 0, !dbg !36053
  %1946 = zext i1 %1945 to i32, !dbg !36053
  %1947 = extractelement <8 x i32> %bc.i.i, i64 1, !dbg !36053
  %1948 = icmp eq i32 %1947, 0, !dbg !36053
  %1949 = select i1 %1948, i32 0, i32 2, !dbg !36053
  %1950 = extractelement <8 x i32> %bc.i.i, i64 2, !dbg !36053
  %1951 = icmp eq i32 %1950, 0, !dbg !36053
  %1952 = select i1 %1951, i32 0, i32 4, !dbg !36053
  %1953 = extractelement <8 x i32> %bc.i.i, i64 3, !dbg !36053
  %1954 = icmp eq i32 %1953, 0, !dbg !36053
  %1955 = select i1 %1954, i32 0, i32 8, !dbg !36053
  %1956 = extractelement <8 x i32> %bc.i.i, i64 4, !dbg !36053
  %1957 = icmp eq i32 %1956, 0, !dbg !36053
  %1958 = select i1 %1957, i32 0, i32 16, !dbg !36053
  %1959 = extractelement <8 x i32> %bc.i.i, i64 5, !dbg !36053
  %1960 = icmp eq i32 %1959, 0, !dbg !36053
  %1961 = select i1 %1960, i32 0, i32 32, !dbg !36053
  %1962 = extractelement <8 x i32> %bc.i.i, i64 6, !dbg !36053
  %1963 = icmp eq i32 %1962, 0, !dbg !36053
  %1964 = select i1 %1963, i32 0, i32 64, !dbg !36053
  %1965 = extractelement <8 x i32> %bc.i.i, i64 7, !dbg !36053
  %1966 = icmp eq i32 %1965, 0, !dbg !36053
  %1967 = select i1 %1966, i32 0, i32 128, !dbg !36053
  %fst_len.i.i108.i = and i64 %right_io.1, 2305843009213693944, !dbg !36054
  %_40.not71.i109.i = icmp eq i64 %fst_len.i.i108.i, 0, !dbg !36058
  br i1 %_40.not71.i109.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit134.i, label %bb23.i110.i, !dbg !36058

bb23.i110.i:                                      ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, %bb23.i110.i
  %iter.sroa.0.074.i111.i = phi ptr [ %_45.i115.i, %bb23.i110.i ], [ %right_io.0, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i ]
  %iter.sroa.5.073.i112.i = phi i64 [ %_46.i116.i, %bb23.i110.i ], [ %fst_len.i.i108.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i ]
  %ok.sroa.0.072.i113.i = phi <8 x i32> [ %1972, %bb23.i110.i ], [ %1923, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i ]
  %lanes.i.sroa.0.0.copyload.i114.i = load <8 x i32>, ptr %iter.sroa.0.074.i111.i, align 4, !dbg !36061, !alias.scope !36066, !noalias !36072
  %_45.i115.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.074.i111.i, i64 32, !dbg !36076
  %_46.i116.i = add i64 %iter.sroa.5.073.i112.i, -8, !dbg !36079
  %1968 = and <8 x i32> %lanes.i.sroa.0.0.copyload.i114.i, splat (i32 2147483647), !dbg !36080
  %1969 = bitcast <8 x i32> %1968 to <8 x float>, !dbg !36086
  %1970 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1969, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !36087
  %1971 = bitcast <8 x float> %1970 to <8 x i32>, !dbg !36093
  %1972 = and <8 x i32> %ok.sroa.0.072.i113.i, %1971, !dbg !36097
  %_40.not.i117.i = icmp eq i64 %_46.i116.i, 0, !dbg !36058
  br i1 %_40.not.i117.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit134.i, label %bb23.i110.i, !dbg !36058

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit134.i: ; preds = %bb23.i110.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i
  %ok.sroa.0.0.lcssa.i118.i = phi <8 x i32> [ %1923, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i ], [ %1972, %bb23.i110.i ], !dbg !36099
  %1973 = icmp slt <8 x i32> %ok.sroa.0.0.lcssa.i118.i, zeroinitializer, !dbg !36100
  %bc.i119.i = select <8 x i1> %1973, <8 x i32> zeroinitializer, <8 x i32> splat (i32 1065353216), !dbg !36100
  %1974 = extractelement <8 x i32> %bc.i119.i, i64 0, !dbg !36105
  %1975 = icmp ne i32 %1974, 0, !dbg !36105
  %1976 = zext i1 %1975 to i32, !dbg !36105
  %1977 = extractelement <8 x i32> %bc.i119.i, i64 1, !dbg !36105
  %1978 = icmp eq i32 %1977, 0, !dbg !36105
  %1979 = select i1 %1978, i32 0, i32 2, !dbg !36105
  %1980 = extractelement <8 x i32> %bc.i119.i, i64 2, !dbg !36105
  %1981 = icmp eq i32 %1980, 0, !dbg !36105
  %1982 = select i1 %1981, i32 0, i32 4, !dbg !36105
  %1983 = extractelement <8 x i32> %bc.i119.i, i64 3, !dbg !36105
  %1984 = icmp eq i32 %1983, 0, !dbg !36105
  %1985 = select i1 %1984, i32 0, i32 8, !dbg !36105
  %1986 = extractelement <8 x i32> %bc.i119.i, i64 4, !dbg !36105
  %1987 = icmp eq i32 %1986, 0, !dbg !36105
  %1988 = select i1 %1987, i32 0, i32 16, !dbg !36105
  %1989 = extractelement <8 x i32> %bc.i119.i, i64 5, !dbg !36105
  %1990 = icmp eq i32 %1989, 0, !dbg !36105
  %1991 = select i1 %1990, i32 0, i32 32, !dbg !36105
  %1992 = extractelement <8 x i32> %bc.i119.i, i64 6, !dbg !36105
  %1993 = icmp eq i32 %1992, 0, !dbg !36105
  %1994 = select i1 %1993, i32 0, i32 64, !dbg !36105
  %1995 = extractelement <8 x i32> %bc.i119.i, i64 7, !dbg !36105
  %1996 = icmp eq i32 %1995, 0, !dbg !36105
  %1997 = select i1 %1996, i32 0, i32 128, !dbg !36105
  %1998 = getelementptr inbounds nuw i8, ptr %self, i64 1576, !dbg !36106
  %mask.sroa.0.1.1.i121.i = or disjoint i32 %1949, %1946, !dbg !36105
  %mask.sroa.0.1.2.i123.i = or disjoint i32 %mask.sroa.0.1.1.i121.i, %1952, !dbg !36105
  %mask.sroa.0.1.3.i125.i = or disjoint i32 %mask.sroa.0.1.2.i123.i, %1955, !dbg !36105
  %mask.sroa.0.1.4.i127.i = or disjoint i32 %mask.sroa.0.1.3.i125.i, %1958, !dbg !36105
  %mask.sroa.0.1.5.i129.i = or disjoint i32 %mask.sroa.0.1.4.i127.i, %1961, !dbg !36105
  %mask.sroa.0.1.6.i131.i = or i32 %mask.sroa.0.1.5.i129.i, %1964, !dbg !36105
  %mask.sroa.0.1.7.i133.i = or i32 %mask.sroa.0.1.6.i131.i, %1967, !dbg !36105
  %mask.sroa.0.1.1.i.i = or i32 %mask.sroa.0.1.7.i133.i, %1976, !dbg !36053
  %mask.sroa.0.1.2.i.i = or i32 %mask.sroa.0.1.1.i.i, %1979, !dbg !36053
  %mask.sroa.0.1.3.i.i = or i32 %mask.sroa.0.1.2.i.i, %1982, !dbg !36053
  %mask.sroa.0.1.4.i.i = or i32 %mask.sroa.0.1.3.i.i, %1985, !dbg !36053
  %mask.sroa.0.1.5.i.i = or i32 %mask.sroa.0.1.4.i.i, %1988, !dbg !36053
  %mask.sroa.0.1.6.i.i = or i32 %mask.sroa.0.1.5.i.i, %1991, !dbg !36053
  %mask.sroa.0.1.7.i.i = or i32 %mask.sroa.0.1.6.i.i, %1994, !dbg !36053
  %1999 = or i32 %mask.sroa.0.1.7.i.i, %1997, !dbg !36106
  store i32 %1999, ptr %1998, align 8, !dbg !36106, !alias.scope !35892, !noalias !36107
  %_14.i7232 = load i64, ptr %_53, align 8, !dbg !36108, !alias.scope !35892, !noalias !36107, !noundef !12
  %2000 = tail call i64 @llvm.uadd.sat.i64(i64 %_14.i7232, i64 1), !dbg !36109
  store i64 %2000, ptr %_53, align 8, !dbg !36112, !alias.scope !35892, !noalias !36107
  %_222.i.i = icmp eq i64 %left_io.1, 0, !dbg !36113
  br i1 %_222.i.i, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %bb14.i.preheader.i, !dbg !36119

bb14.i.preheader.i:                               ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit134.i
  %.idx.i.i = shl nuw nsw i64 %left_io.1, 2, !dbg !36120
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %left_io.0, i8 0, i64 %.idx.i.i, i1 false), !dbg !36124, !alias.scope !36125, !noalias !36128
  br label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i, !dbg !36129

_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i: ; preds = %bb14.i.preheader.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit134.i
  %_222.i136.i = icmp eq i64 %right_io.1, 0, !dbg !36135
  br i1 %_222.i136.i, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit141.i, label %bb14.i137.preheader.i, !dbg !36138

bb14.i137.preheader.i:                            ; preds = %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i
  %.idx.i135.i = shl nuw nsw i64 %right_io.1, 2, !dbg !36129
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %right_io.0, i8 0, i64 %.idx.i135.i, i1 false), !dbg !36139, !alias.scope !36140, !noalias !36143
  br label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit141.i, !dbg !36144

_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit141.i: ; preds = %bb14.i137.preheader.i, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 8 dereferenceable(200) %_33, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_99.0, i64 noundef %_99.1, i32 noundef %1917) #31, !dbg !36145, !noalias !36148
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 8 dereferenceable(200) %_34, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_100.0, i64 noundef %_100.1, i32 noundef %1917) #31, !dbg !36151, !noalias !36148
  store i32 0, ptr %_35, align 4, !dbg !36152, !noalias !36148
  %2001 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !36152
  store i32 0, ptr %2001, align 4, !dbg !36152, !noalias !36148
  br label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB1z_11LimiterCoreBR_E13process_block0EB1z_.exit, !dbg !36153

_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB1z_11LimiterCoreBR_E13process_block0EB1z_.exit: ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit141.i
  call void @llvm.lifetime.end.p0(ptr nonnull %shape), !dbg !36154
  br label %bb42, !dbg !35820

bb54:                                             ; preds = %bb34
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %words, i64 noundef %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1f13fdd6e9523d10fc4ba543bd6c2386) #30, !dbg !36155
  unreachable, !dbg !36155

bb1.i7240:                                        ; preds = %bb34, %bb10.i7255
  %iter.sroa.6.0.i7241 = phi i64 [ %len.i.i.i.i7246, %bb10.i7255 ], [ %words, %bb34 ], !dbg !36156
  %iter.sroa.0.0.i7242 = phi ptr [ %data.i.i.i.i7245, %bb10.i7255 ], [ %right_io.0, %bb34 ], !dbg !36156
  %2002 = icmp eq i64 %iter.sroa.6.0.i7241, 0, !dbg !36158
  br i1 %2002, label %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit7257, label %bb11.preheader.i7243, !dbg !36158

bb11.preheader.i7243:                             ; preds = %bb1.i7240
  %..i.i.i7244 = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i7241, i64 32), !dbg !36160
  %_18.idx.i7247 = shl nuw nsw i64 %..i.i.i7244, 2, !dbg !36163
  %_18.i7248 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i7242, i64 %_18.idx.i7247, !dbg !36163
  br label %bb11.i7249, !dbg !36168

bb11.i7249:                                       ; preds = %bb11.i7249, %bb11.preheader.i7243
  %iter1.sroa.0.014.i7250 = phi ptr [ %_31.i7252, %bb11.i7249 ], [ %iter.sroa.0.0.i7242, %bb11.preheader.i7243 ]
  %bits.sroa.0.013.i7251 = phi i32 [ %2003, %bb11.i7249 ], [ 0, %bb11.preheader.i7243 ]
  %_31.i7252 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i7250, i64 4, !dbg !36170
  %_134.i7253 = load i32, ptr %iter1.sroa.0.014.i7250, align 4, !dbg !36172, !alias.scope !36173, !noundef !12
  %2003 = or i32 %_134.i7253, %bits.sroa.0.013.i7251, !dbg !36176
  %_25.i7254 = icmp eq ptr %_31.i7252, %_18.i7248, !dbg !36177
  br i1 %_25.i7254, label %bb10.i7255, label %bb11.i7249, !dbg !36168

bb10.i7255:                                       ; preds = %bb11.i7249
  %data.i.i.i.i7245 = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i7242, i64 %..i.i.i7244, !dbg !36179
  %len.i.i.i.i7246 = sub nuw nsw i64 %iter.sroa.6.0.i7241, %..i.i.i7244, !dbg !36184
  %2004 = icmp eq i32 %2003, 0, !dbg !36185
  br i1 %2004, label %bb1.i7240, label %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit7257, !dbg !36185

_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit7257: ; preds = %bb1.i7240, %bb10.i7255
  %2005 = zext i1 %2002 to i8, !dbg !35871
  br label %bb40, !dbg !35745

bb42:                                             ; preds = %_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB1z_11LimiterCoreBR_E13process_block0EB1z_.exit
  ret void, !dbg !35820
}
