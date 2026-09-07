define internal fastcc void @_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCorefE13process_blockB5_(ptr noalias noundef nonnull align 8 dereferenceable(784) %self, ptr noalias noundef nonnull align 4 captures(address) %left_io.0, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef nonnull align 4 captures(address) %right_io.0, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef %frames) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !17309 {
start:
  %_113.i = alloca [92 x i8], align 4
  %_111.i = alloca [92 x i8], align 4
  %peaks_right.i15 = alloca [1024 x i8], align 4
  %peaks_left.i16 = alloca [1024 x i8], align 4
  %scratch.i = alloca [32 x i8], align 4
  %hot_right.i17 = alloca [92 x i8], align 4
  %hot_left.i18 = alloca [92 x i8], align 4
  %uniform_right.i = alloca [80 x i8], align 8
  %uniform_left.i = alloca [80 x i8], align 8
  %peaks_right.i = alloca [1024 x i8], align 4
  %peaks_left.i = alloca [1024 x i8], align 4
  %hot_right.i = alloca [92 x i8], align 4
  %hot_left.i = alloca [92 x i8], align 4
  %shape = alloca [24 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 781, !dbg !17310
  %1 = load i8, ptr %0, align 1, !dbg !17310, !range !5399, !noundef !12
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 96, !dbg !17312
  %3 = load i8, ptr %2, align 8, !dbg !17312, !range !5399, !noundef !12
  %_7 = icmp eq i8 %1, %3, !dbg !17310
  br i1 %_7, label %bb1, label %bb20.thread, !dbg !17310

bb1:                                              ; preds = %start
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 264, !dbg !17313
  %_95.0 = load ptr, ptr %4, align 8, !dbg !17313, !nonnull !12, !noundef !12
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 272, !dbg !17313
  %_95.1 = load i64, ptr %5, align 8, !dbg !17313, !noundef !12
  %_8.i1712 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_95.0, i64 %_95.1, !dbg !17314
  br label %bb1.i.i1713, !dbg !17319

bb1.i.i1713:                                      ; preds = %bb13.i.i1714, %bb1
  %_221.i.i = phi ptr [ %_22.i.i1715, %bb13.i.i1714 ], [ %_95.0, %bb1 ]
  %_12.i.i = icmp eq ptr %_221.i.i, %_8.i1712, !dbg !17321
  br i1 %_12.i.i, label %bb3, label %bb13.i.i1714, !dbg !17324

bb13.i.i1714:                                     ; preds = %bb1.i.i1713
  %_22.i.i1715 = getelementptr inbounds nuw i8, ptr %_221.i.i, i64 16, !dbg !17325
  %6 = getelementptr inbounds nuw i8, ptr %_221.i.i, i64 12, !dbg !17327
  %_3.i.i.i = load i32, ptr %6, align 4, !dbg !17327, !alias.scope !17329, !noalias !17334, !noundef !12
  %7 = icmp eq i32 %_3.i.i.i, 0, !dbg !17327
  %_51.i.i.i = load i32, ptr %_221.i.i, align 4, !dbg !17327, !alias.scope !17329, !noalias !17334
  %8 = getelementptr inbounds nuw i8, ptr %_221.i.i, i64 4, !dbg !17327
  %_72.i.i.i = load i32, ptr %8, align 4, !dbg !17327, !alias.scope !17329, !noalias !17334
  %9 = icmp eq i32 %_51.i.i.i, %_72.i.i.i, !dbg !17327
  %_0.sroa.0.0.i.i.i = select i1 %7, i1 %9, i1 false, !dbg !17327
  br i1 %_0.sroa.0.0.i.i.i, label %bb1.i.i1713, label %bb20.thread, !dbg !17337

bb3:                                              ; preds = %bb1.i.i1713
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 280, !dbg !17338
  %_96.0 = load ptr, ptr %10, align 8, !dbg !17338, !nonnull !12, !noundef !12
  %11 = getelementptr inbounds nuw i8, ptr %self, i64 288, !dbg !17338
  %_96.1 = load i64, ptr %11, align 8, !dbg !17338, !noundef !12
  %_8.i1716 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_96.0, i64 %_96.1, !dbg !17339
  br label %bb1.i.i1717, !dbg !17344

bb1.i.i1717:                                      ; preds = %bb13.i.i1720, %bb3
  %_221.i.i1718 = phi ptr [ %_22.i.i1721, %bb13.i.i1720 ], [ %_96.0, %bb3 ]
  %_12.i.i1719 = icmp eq ptr %_221.i.i1718, %_8.i1716, !dbg !17346
  br i1 %_12.i.i1719, label %bb5, label %bb13.i.i1720, !dbg !17349

bb13.i.i1720:                                     ; preds = %bb1.i.i1717
  %_22.i.i1721 = getelementptr inbounds nuw i8, ptr %_221.i.i1718, i64 16, !dbg !17350
  %12 = getelementptr inbounds nuw i8, ptr %_221.i.i1718, i64 12, !dbg !17352
  %_3.i.i.i1722 = load i32, ptr %12, align 4, !dbg !17352, !alias.scope !17354, !noalias !17359, !noundef !12
  %13 = icmp eq i32 %_3.i.i.i1722, 0, !dbg !17352
  %_51.i.i.i1723 = load i32, ptr %_221.i.i1718, align 4, !dbg !17352, !alias.scope !17354, !noalias !17359
  %14 = getelementptr inbounds nuw i8, ptr %_221.i.i1718, i64 4, !dbg !17352
  %_72.i.i.i1724 = load i32, ptr %14, align 4, !dbg !17352, !alias.scope !17354, !noalias !17359
  %15 = icmp eq i32 %_51.i.i.i1723, %_72.i.i.i1724, !dbg !17352
  %_0.sroa.0.0.i.i.i1725 = select i1 %13, i1 %15, i1 false, !dbg !17352
  br i1 %_0.sroa.0.0.i.i.i1725, label %bb1.i.i1717, label %bb20.thread, !dbg !17362

bb5:                                              ; preds = %bb1.i.i1717
  %16 = getelementptr inbounds nuw i8, ptr %self, i64 464, !dbg !17363
  %_97.0 = load ptr, ptr %16, align 8, !dbg !17363, !nonnull !12, !noundef !12
  %17 = getelementptr inbounds nuw i8, ptr %self, i64 472, !dbg !17363
  %_97.1 = load i64, ptr %17, align 8, !dbg !17363, !noundef !12
  %_8.i1727 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_97.0, i64 %_97.1, !dbg !17364
  br label %bb1.i.i1728, !dbg !17369

bb1.i.i1728:                                      ; preds = %bb13.i.i1731, %bb5
  %_221.i.i1729 = phi ptr [ %_22.i.i1732, %bb13.i.i1731 ], [ %_97.0, %bb5 ]
  %_12.i.i1730 = icmp eq ptr %_221.i.i1729, %_8.i1727, !dbg !17371
  br i1 %_12.i.i1730, label %bb7, label %bb13.i.i1731, !dbg !17374

bb13.i.i1731:                                     ; preds = %bb1.i.i1728
  %_22.i.i1732 = getelementptr inbounds nuw i8, ptr %_221.i.i1729, i64 16, !dbg !17375
  %18 = getelementptr inbounds nuw i8, ptr %_221.i.i1729, i64 12, !dbg !17377
  %_3.i.i.i1733 = load i32, ptr %18, align 4, !dbg !17377, !alias.scope !17379, !noalias !17384, !noundef !12
  %19 = icmp eq i32 %_3.i.i.i1733, 0, !dbg !17377
  %_51.i.i.i1734 = load i32, ptr %_221.i.i1729, align 4, !dbg !17377, !alias.scope !17379, !noalias !17384
  %20 = getelementptr inbounds nuw i8, ptr %_221.i.i1729, i64 4, !dbg !17377
  %_72.i.i.i1735 = load i32, ptr %20, align 4, !dbg !17377, !alias.scope !17379, !noalias !17384
  %21 = icmp eq i32 %_51.i.i.i1734, %_72.i.i.i1735, !dbg !17377
  %_0.sroa.0.0.i.i.i1736 = select i1 %19, i1 %21, i1 false, !dbg !17377
  br i1 %_0.sroa.0.0.i.i.i1736, label %bb1.i.i1728, label %bb20.thread, !dbg !17387

bb7:                                              ; preds = %bb1.i.i1728
  %22 = getelementptr inbounds nuw i8, ptr %self, i64 480, !dbg !17388
  %_98.0 = load ptr, ptr %22, align 8, !dbg !17388, !nonnull !12, !noundef !12
  %23 = getelementptr inbounds nuw i8, ptr %self, i64 488, !dbg !17388
  %_98.1 = load i64, ptr %23, align 8, !dbg !17388, !noundef !12
  %_8.i1738 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_98.0, i64 %_98.1, !dbg !17389
  br label %bb1.i.i1739, !dbg !17394

bb1.i.i1739:                                      ; preds = %bb13.i.i1742, %bb7
  %_221.i.i1740 = phi ptr [ %_22.i.i1743, %bb13.i.i1742 ], [ %_98.0, %bb7 ]
  %_12.i.i1741 = icmp eq ptr %_221.i.i1740, %_8.i1738, !dbg !17396
  br i1 %_12.i.i1741, label %bb9, label %bb13.i.i1742, !dbg !17399

bb13.i.i1742:                                     ; preds = %bb1.i.i1739
  %_22.i.i1743 = getelementptr inbounds nuw i8, ptr %_221.i.i1740, i64 16, !dbg !17400
  %24 = getelementptr inbounds nuw i8, ptr %_221.i.i1740, i64 12, !dbg !17402
  %_3.i.i.i1744 = load i32, ptr %24, align 4, !dbg !17402, !alias.scope !17404, !noalias !17409, !noundef !12
  %25 = icmp eq i32 %_3.i.i.i1744, 0, !dbg !17402
  %_51.i.i.i1745 = load i32, ptr %_221.i.i1740, align 4, !dbg !17402, !alias.scope !17404, !noalias !17409
  %26 = getelementptr inbounds nuw i8, ptr %_221.i.i1740, i64 4, !dbg !17402
  %_72.i.i.i1746 = load i32, ptr %26, align 4, !dbg !17402, !alias.scope !17404, !noalias !17409
  %27 = icmp eq i32 %_51.i.i.i1745, %_72.i.i.i1746, !dbg !17402
  %_0.sroa.0.0.i.i.i1747 = select i1 %25, i1 %27, i1 false, !dbg !17402
  br i1 %_0.sroa.0.0.i.i.i1747, label %bb1.i.i1739, label %bb20.thread, !dbg !17412

bb9:                                              ; preds = %bb1.i.i1739
  %_65.not = icmp ugt i64 %frames, %left_io.1
  br i1 %_65.not, label %bb45, label %bb1.i1749, !dbg !17413, !prof !5262

bb45:                                             ; preds = %bb9
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %frames, i64 noundef %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_449c9992b9e28dda1c528a8de6026d01) #31, !dbg !17422
  unreachable, !dbg !17422

bb1.i1749:                                        ; preds = %bb9, %bb10.i
  %iter.sroa.6.0.i = phi i64 [ %len.i.i.i.i, %bb10.i ], [ %frames, %bb9 ], !dbg !17423
  %iter.sroa.0.0.i1750 = phi ptr [ %data.i.i.i.i, %bb10.i ], [ %left_io.0, %bb9 ], !dbg !17423
  %28 = icmp eq i64 %iter.sroa.6.0.i, 0, !dbg !17425
  br i1 %28, label %bb11, label %bb11.preheader.i, !dbg !17425

bb11.preheader.i:                                 ; preds = %bb1.i1749
  %..i.i.i = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i, i64 32), !dbg !17427
  %_18.idx.i = shl nuw nsw i64 %..i.i.i, 2, !dbg !17430
  %_18.i1751 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i1750, i64 %_18.idx.i, !dbg !17430
  br label %bb11.i1752, !dbg !17435

bb11.i1752:                                       ; preds = %bb11.i1752, %bb11.preheader.i
  %iter1.sroa.0.014.i = phi ptr [ %_31.i1753, %bb11.i1752 ], [ %iter.sroa.0.0.i1750, %bb11.preheader.i ]
  %bits.sroa.0.013.i = phi i32 [ %29, %bb11.i1752 ], [ 0, %bb11.preheader.i ]
  %_31.i1753 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i, i64 4, !dbg !17437
  %_134.i1754 = load i32, ptr %iter1.sroa.0.014.i, align 4, !dbg !17439, !alias.scope !17440, !noundef !12
  %29 = or i32 %_134.i1754, %bits.sroa.0.013.i, !dbg !17443
  %_25.i1755 = icmp eq ptr %_31.i1753, %_18.i1751, !dbg !17444
  br i1 %_25.i1755, label %bb10.i, label %bb11.i1752, !dbg !17435

bb10.i:                                           ; preds = %bb11.i1752
  %data.i.i.i.i = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i1750, i64 %..i.i.i, !dbg !17446
  %len.i.i.i.i = sub nuw nsw i64 %iter.sroa.6.0.i, %..i.i.i, !dbg !17451
  %30 = icmp eq i32 %29, 0, !dbg !17452
  br i1 %30, label %bb1.i1749, label %bb20.thread, !dbg !17452

bb11:                                             ; preds = %bb1.i1749
  %_73.not = icmp ugt i64 %frames, %right_io.1, !dbg !17453
  br i1 %_73.not, label %bb48, label %bb1.i1756, !dbg !17453, !prof !905

bb20.thread:                                      ; preds = %bb13.i.i1714, %bb13.i.i1720, %bb13.i.i1731, %bb13.i.i1742, %bb10.i, %start
  %31 = getelementptr inbounds nuw i8, ptr %self, i64 780
  br label %bb26, !dbg !17459

bb20:                                             ; preds = %bb1.i1756
  %32 = getelementptr inbounds nuw i8, ptr %self, i64 780
  %33 = load i8, ptr %32, align 4, !range !5399
  %_22 = trunc nuw i8 %33 to i1
  br i1 %_22, label %bb22, label %bb26, !dbg !17459

bb48:                                             ; preds = %bb11
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %frames, i64 noundef %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_69177cbbe364d953a90a8930cefca3fe) #31, !dbg !17461
  unreachable, !dbg !17461

bb1.i1756:                                        ; preds = %bb11, %bb10.i1771
  %iter.sroa.6.0.i1757 = phi i64 [ %len.i.i.i.i1762, %bb10.i1771 ], [ %frames, %bb11 ], !dbg !17462
  %iter.sroa.0.0.i1758 = phi ptr [ %data.i.i.i.i1761, %bb10.i1771 ], [ %right_io.0, %bb11 ], !dbg !17462
  %34 = icmp eq i64 %iter.sroa.6.0.i1757, 0, !dbg !17464
  br i1 %34, label %bb20, label %bb11.preheader.i1759, !dbg !17464

bb11.preheader.i1759:                             ; preds = %bb1.i1756
  %..i.i.i1760 = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i1757, i64 32), !dbg !17466
  %_18.idx.i1763 = shl nuw nsw i64 %..i.i.i1760, 2, !dbg !17469
  %_18.i1764 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i1758, i64 %_18.idx.i1763, !dbg !17469
  br label %bb11.i1765, !dbg !17474

bb11.i1765:                                       ; preds = %bb11.i1765, %bb11.preheader.i1759
  %iter1.sroa.0.014.i1766 = phi ptr [ %_31.i1768, %bb11.i1765 ], [ %iter.sroa.0.0.i1758, %bb11.preheader.i1759 ]
  %bits.sroa.0.013.i1767 = phi i32 [ %35, %bb11.i1765 ], [ 0, %bb11.preheader.i1759 ]
  %_31.i1768 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i1766, i64 4, !dbg !17476
  %_134.i1769 = load i32, ptr %iter1.sroa.0.014.i1766, align 4, !dbg !17478, !alias.scope !17479, !noundef !12
  %35 = or i32 %_134.i1769, %bits.sroa.0.013.i1767, !dbg !17482
  %_25.i1770 = icmp eq ptr %_31.i1768, %_18.i1764, !dbg !17483
  br i1 %_25.i1770, label %bb10.i1771, label %bb11.i1765, !dbg !17474

bb10.i1771:                                       ; preds = %bb11.i1765
  %data.i.i.i.i1761 = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i1758, i64 %..i.i.i1760, !dbg !17485
  %len.i.i.i.i1762 = sub nuw nsw i64 %iter.sroa.6.0.i1757, %..i.i.i1760, !dbg !17490
  %36 = icmp eq i32 %35, 0, !dbg !17491
  br i1 %36, label %bb1.i1756, label %bb20.thread2219, !dbg !17491

bb20.thread2219:                                  ; preds = %bb10.i1771
  %37 = getelementptr inbounds nuw i8, ptr %self, i64 780
  br label %bb26, !dbg !17459

bb26:                                             ; preds = %bb20.thread2219, %bb20.thread, %bb20
  %38 = phi ptr [ %31, %bb20.thread ], [ %32, %bb20 ], [ %37, %bb20.thread2219 ]
  %quiet.sroa.0.02218 = phi i1 [ false, %bb20.thread ], [ true, %bb20 ], [ false, %bb20.thread2219 ]
  %_31 = getelementptr inbounds nuw i8, ptr %self, i64 584, !dbg !17492
  %_32 = getelementptr inbounds nuw i8, ptr %self, i64 536, !dbg !17493
  %_33 = getelementptr inbounds nuw i8, ptr %self, i64 136, !dbg !17494
  %_34 = getelementptr inbounds nuw i8, ptr %self, i64 336, !dbg !17495
  %_35 = getelementptr inbounds nuw i8, ptr %self, i64 560, !dbg !17496
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17497), !dbg !17500
  %39 = getelementptr inbounds nuw i8, ptr %self, i64 312, !dbg !17503
  %_31.0.i = load ptr, ptr %39, align 8, !dbg !17503, !alias.scope !17497, !noalias !17505, !nonnull !12, !noundef !12
  %40 = getelementptr inbounds nuw i8, ptr %self, i64 320, !dbg !17503
  %_31.1.i = load i64, ptr %40, align 8, !dbg !17503, !alias.scope !17497, !noalias !17505, !noundef !12
  %_17.idx.i = mul nuw nsw i64 %_31.1.i, 12, !dbg !17513
  %_17.i = getelementptr inbounds nuw i8, ptr %_31.0.i, i64 %_17.idx.i, !dbg !17513
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17517), !dbg !17520, !noalias !17505
  %_5.not.i.i.i = icmp eq i64 %_31.1.i, 0
  %41 = getelementptr inbounds nuw i8, ptr %_31.0.i, i64 4
  %42 = getelementptr inbounds nuw i8, ptr %_31.0.i, i64 8
  br i1 %_5.not.i.i.i, label %bb2.i1780, label %bb1.i.i1773

bb1.i.i1773:                                      ; preds = %bb26, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i
  %_224.i.i = phi ptr [ %_22.i.i1776, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i ], [ %_31.0.i, %bb26 ]
  %_12.i.i1774 = icmp eq ptr %_224.i.i, %_17.i, !dbg !17521
  br i1 %_12.i.i1774, label %bb2.i1780, label %bb13.i.i1775, !dbg !17525

bb13.i.i1775:                                     ; preds = %bb1.i.i1773
  %_22.i.i1776 = getelementptr inbounds nuw i8, ptr %_224.i.i, i64 12, !dbg !17526
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17528), !dbg !17531, !noalias !17505
  %_9.i.i.i = load i32, ptr %_224.i.i, align 4, !dbg !17532, !alias.scope !17528, !noalias !17535, !noundef !12
  %_10.i.i.i = load i32, ptr %_31.0.i, align 4, !dbg !17532, !alias.scope !17517, !noalias !17537, !noundef !12
  %_8.i.i.i = icmp eq i32 %_9.i.i.i, %_10.i.i.i, !dbg !17532
  br i1 %_8.i.i.i, label %bb2.i.i.i, label %bb7.i, !dbg !17532

bb2.i.i.i:                                        ; preds = %bb13.i.i1775
  %43 = getelementptr inbounds nuw i8, ptr %_224.i.i, i64 4, !dbg !17532
  %_12.i.i.i = load i32, ptr %43, align 4, !dbg !17532, !alias.scope !17528, !noalias !17535, !noundef !12
  %_13.i.i.i = load i32, ptr %41, align 4, !dbg !17532, !alias.scope !17517, !noalias !17537, !noundef !12
  %_11.i.i.i = icmp eq i32 %_12.i.i.i, %_13.i.i.i, !dbg !17532
  br i1 %_11.i.i.i, label %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, label %bb7.i, !dbg !17532

_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i: ; preds = %bb2.i.i.i
  %44 = getelementptr inbounds nuw i8, ptr %_224.i.i, i64 8, !dbg !17532
  %_14.i.i.i1778 = load i32, ptr %44, align 4, !dbg !17532, !alias.scope !17528, !noalias !17535, !noundef !12
  %_15.i.i.i1779 = load i32, ptr %42, align 4, !dbg !17532, !alias.scope !17517, !noalias !17537, !noundef !12
  %45 = icmp eq i32 %_14.i.i.i1778, %_15.i.i.i1779, !dbg !17532
  br i1 %45, label %bb1.i.i1773, label %bb7.i, !dbg !17531

bb2.i1780:                                        ; preds = %bb1.i.i1773, %bb26
  %46 = getelementptr inbounds nuw i8, ptr %self, i64 248, !dbg !17538
  %_32.0.i = load ptr, ptr %46, align 8, !dbg !17538, !alias.scope !17497, !noalias !17505, !nonnull !12, !noundef !12
  %47 = getelementptr inbounds nuw i8, ptr %self, i64 256, !dbg !17538
  %_32.1.i = load i64, ptr %47, align 8, !dbg !17538, !alias.scope !17497, !noalias !17505, !noundef !12
  %_26.idx.i = shl nuw nsw i64 %_32.1.i, 2, !dbg !17539
  %_26.i1781 = getelementptr inbounds nuw i8, ptr %_32.0.i, i64 %_26.idx.i, !dbg !17539
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17543), !dbg !17546, !noalias !17505
  %_6.not.i.i.i = icmp eq i64 %_32.1.i, 0
  br i1 %_6.not.i.i.i, label %bb2.i, label %bb1.i3.i

bb1.i3.i:                                         ; preds = %bb2.i1780, %bb13.i5.i
  %_223.i.i = phi ptr [ %_22.i6.i, %bb13.i5.i ], [ %_32.0.i, %bb2.i1780 ]
  %_12.i4.i = icmp eq ptr %_223.i.i, %_26.i1781, !dbg !17547
  br i1 %_12.i4.i, label %bb2.i, label %bb13.i5.i, !dbg !17551

bb13.i5.i:                                        ; preds = %bb1.i3.i
  %_22.i6.i = getelementptr inbounds nuw i8, ptr %_223.i.i, i64 4, !dbg !17552
  %ptr.val.i.i = load i32, ptr %_223.i.i, align 4, !dbg !17554, !noalias !17555
  %_4.i.i.i1782 = load i32, ptr %_32.0.i, align 4, !dbg !17557, !alias.scope !17543, !noalias !17559, !noundef !12
  %_0.i.i.i = icmp eq i32 %ptr.val.i.i, %_4.i.i.i1782, !dbg !17560
  br i1 %_0.i.i.i, label %bb1.i3.i, label %bb7.i, !dbg !17554

bb2.i:                                            ; preds = %bb1.i3.i, %bb2.i1780
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17561), !dbg !17564
  %48 = getelementptr inbounds nuw i8, ptr %self, i64 512, !dbg !17565
  %_31.0.i1783 = load ptr, ptr %48, align 8, !dbg !17565, !alias.scope !17561, !noalias !17567, !nonnull !12, !noundef !12
  %49 = getelementptr inbounds nuw i8, ptr %self, i64 520, !dbg !17565
  %_31.1.i1784 = load i64, ptr %49, align 8, !dbg !17565, !alias.scope !17561, !noalias !17567, !noundef !12
  %_17.idx.i1785 = mul nuw nsw i64 %_31.1.i1784, 12, !dbg !17568
  %_17.i1786 = getelementptr inbounds nuw i8, ptr %_31.0.i1783, i64 %_17.idx.i1785, !dbg !17568
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17572), !dbg !17575, !noalias !17567
  %_5.not.i.i.i1787 = icmp eq i64 %_31.1.i1784, 0
  %50 = getelementptr inbounds nuw i8, ptr %_31.0.i1783, i64 4
  %51 = getelementptr inbounds nuw i8, ptr %_31.0.i1783, i64 8
  br i1 %_5.not.i.i.i1787, label %bb2.i1805, label %bb1.i.i1788

bb1.i.i1788:                                      ; preds = %bb2.i, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i1802
  %_224.i.i1789 = phi ptr [ %_22.i.i1792, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i1802 ], [ %_31.0.i1783, %bb2.i ]
  %_12.i.i1790 = icmp eq ptr %_224.i.i1789, %_17.i1786, !dbg !17576
  br i1 %_12.i.i1790, label %bb2.i1805, label %bb13.i.i1791, !dbg !17580

bb13.i.i1791:                                     ; preds = %bb1.i.i1788
  %_22.i.i1792 = getelementptr inbounds nuw i8, ptr %_224.i.i1789, i64 12, !dbg !17581
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17583), !dbg !17586, !noalias !17567
  %_9.i.i.i1793 = load i32, ptr %_224.i.i1789, align 4, !dbg !17587, !alias.scope !17583, !noalias !17590, !noundef !12
  %_10.i.i.i1794 = load i32, ptr %_31.0.i1783, align 4, !dbg !17587, !alias.scope !17572, !noalias !17592, !noundef !12
  %_8.i.i.i1795 = icmp eq i32 %_9.i.i.i1793, %_10.i.i.i1794, !dbg !17587
  br i1 %_8.i.i.i1795, label %bb2.i.i.i1798, label %bb7.i, !dbg !17587

bb2.i.i.i1798:                                    ; preds = %bb13.i.i1791
  %52 = getelementptr inbounds nuw i8, ptr %_224.i.i1789, i64 4, !dbg !17587
  %_12.i.i.i1799 = load i32, ptr %52, align 4, !dbg !17587, !alias.scope !17583, !noalias !17590, !noundef !12
  %_13.i.i.i1800 = load i32, ptr %50, align 4, !dbg !17587, !alias.scope !17572, !noalias !17592, !noundef !12
  %_11.i.i.i1801 = icmp eq i32 %_12.i.i.i1799, %_13.i.i.i1800, !dbg !17587
  br i1 %_11.i.i.i1801, label %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i1802, label %bb7.i, !dbg !17587

_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i1802: ; preds = %bb2.i.i.i1798
  %53 = getelementptr inbounds nuw i8, ptr %_224.i.i1789, i64 8, !dbg !17587
  %_14.i.i.i1803 = load i32, ptr %53, align 4, !dbg !17587, !alias.scope !17583, !noalias !17590, !noundef !12
  %_15.i.i.i1804 = load i32, ptr %51, align 4, !dbg !17587, !alias.scope !17572, !noalias !17592, !noundef !12
  %54 = icmp eq i32 %_14.i.i.i1803, %_15.i.i.i1804, !dbg !17587
  br i1 %54, label %bb1.i.i1788, label %bb7.i, !dbg !17586

bb2.i1805:                                        ; preds = %bb1.i.i1788, %bb2.i
  %55 = getelementptr inbounds nuw i8, ptr %self, i64 448, !dbg !17593
  %_32.0.i1806 = load ptr, ptr %55, align 8, !dbg !17593, !alias.scope !17561, !noalias !17567, !nonnull !12, !noundef !12
  %56 = getelementptr inbounds nuw i8, ptr %self, i64 456, !dbg !17593
  %_32.1.i1807 = load i64, ptr %56, align 8, !dbg !17593, !alias.scope !17561, !noalias !17567, !noundef !12
  %_26.idx.i1808 = shl nuw nsw i64 %_32.1.i1807, 2, !dbg !17594
  %_26.i1809 = getelementptr inbounds nuw i8, ptr %_32.0.i1806, i64 %_26.idx.i1808, !dbg !17594
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17598), !dbg !17601, !noalias !17567
  %_6.not.i.i.i1810 = icmp eq i64 %_32.1.i1807, 0
  br i1 %_6.not.i.i.i1810, label %bb4.i, label %bb1.i3.i1811

bb1.i3.i1811:                                     ; preds = %bb2.i1805, %bb13.i5.i1814
  %_223.i.i1812 = phi ptr [ %_22.i6.i1815, %bb13.i5.i1814 ], [ %_32.0.i1806, %bb2.i1805 ]
  %_12.i4.i1813 = icmp eq ptr %_223.i.i1812, %_26.i1809, !dbg !17602
  br i1 %_12.i4.i1813, label %bb4.i, label %bb13.i5.i1814, !dbg !17606

bb13.i5.i1814:                                    ; preds = %bb1.i3.i1811
  %_22.i6.i1815 = getelementptr inbounds nuw i8, ptr %_223.i.i1812, i64 4, !dbg !17607
  %ptr.val.i.i1816 = load i32, ptr %_223.i.i1812, align 4, !dbg !17609, !noalias !17610
  %_4.i.i.i1817 = load i32, ptr %_32.0.i1806, align 4, !dbg !17612, !alias.scope !17598, !noalias !17614, !noundef !12
  %_0.i.i.i1818 = icmp eq i32 %ptr.val.i.i1816, %_4.i.i.i1817, !dbg !17615
  br i1 %_0.i.i.i1818, label %bb1.i3.i1811, label %bb7.i, !dbg !17609

bb7.i:                                            ; preds = %bb13.i.i1775, %bb2.i.i.i, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, %bb13.i5.i, %bb13.i.i1791, %bb2.i.i.i1798, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i1802, %bb13.i5.i1814
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17616), !dbg !17619
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17620), !dbg !17619
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17622), !dbg !17619
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17624), !dbg !17619
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i18), !dbg !17626, !noalias !17630
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_left.i18, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #30, !dbg !17634, !noalias !17635
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_right.i17), !dbg !17636, !noalias !17630
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_right.i17, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #30, !dbg !17638, !noalias !17639
  %57 = getelementptr inbounds nuw i8, ptr %self, i64 264, !dbg !17640
  %_162.0.i = load ptr, ptr %57, align 8, !dbg !17640, !alias.scope !17620, !noalias !17642, !nonnull !12, !noundef !12
  %58 = getelementptr inbounds nuw i8, ptr %self, i64 272, !dbg !17640
  %_162.1.i = load i64, ptr %58, align 8, !dbg !17640, !alias.scope !17620, !noalias !17642, !noundef !12
  %_8.i1820 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_162.0.i, i64 %_162.1.i, !dbg !17643
  br label %bb1.i.i1821, !dbg !17648

bb1.i.i1821:                                      ; preds = %bb13.i.i1824, %bb7.i
  %_221.i.i1822 = phi ptr [ %_22.i.i1825, %bb13.i.i1824 ], [ %_162.0.i, %bb7.i ]
  %_12.i.i1823 = icmp eq ptr %_221.i.i1822, %_8.i1820, !dbg !17650
  br i1 %_12.i.i1823, label %bb4.i275, label %bb13.i.i1824, !dbg !17653

bb13.i.i1824:                                     ; preds = %bb1.i.i1821
  %_22.i.i1825 = getelementptr inbounds nuw i8, ptr %_221.i.i1822, i64 16, !dbg !17654
  %59 = getelementptr inbounds nuw i8, ptr %_221.i.i1822, i64 12, !dbg !17656
  %_3.i.i.i1826 = load i32, ptr %59, align 4, !dbg !17656, !alias.scope !17658, !noalias !17663, !noundef !12
  %60 = icmp eq i32 %_3.i.i.i1826, 0, !dbg !17656
  %_51.i.i.i1827 = load i32, ptr %_221.i.i1822, align 4, !dbg !17656, !alias.scope !17658, !noalias !17663
  %61 = getelementptr inbounds nuw i8, ptr %_221.i.i1822, i64 4, !dbg !17656
  %_72.i.i.i1828 = load i32, ptr %61, align 4, !dbg !17656, !alias.scope !17658, !noalias !17663
  %62 = icmp eq i32 %_51.i.i.i1827, %_72.i.i.i1828, !dbg !17656
  %_0.sroa.0.0.i.i.i1829 = select i1 %60, i1 %62, i1 false, !dbg !17656
  br i1 %_0.sroa.0.0.i.i.i1829, label %bb1.i.i1821, label %bb14.i20, !dbg !17666

bb4.i275:                                         ; preds = %bb1.i.i1821
  %63 = getelementptr inbounds nuw i8, ptr %self, i64 280, !dbg !17667
  %_163.0.i = load ptr, ptr %63, align 8, !dbg !17667, !alias.scope !17620, !noalias !17642, !nonnull !12, !noundef !12
  %64 = getelementptr inbounds nuw i8, ptr %self, i64 288, !dbg !17667
  %_163.1.i = load i64, ptr %64, align 8, !dbg !17667, !alias.scope !17620, !noalias !17642, !noundef !12
  %_8.i1831 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_163.0.i, i64 %_163.1.i, !dbg !17668
  br label %bb1.i.i1832, !dbg !17673

bb1.i.i1832:                                      ; preds = %bb13.i.i1835, %bb4.i275
  %_221.i.i1833 = phi ptr [ %_22.i.i1836, %bb13.i.i1835 ], [ %_163.0.i, %bb4.i275 ]
  %_12.i.i1834 = icmp eq ptr %_221.i.i1833, %_8.i1831, !dbg !17675
  br i1 %_12.i.i1834, label %bb6.i277, label %bb13.i.i1835, !dbg !17678

bb13.i.i1835:                                     ; preds = %bb1.i.i1832
  %_22.i.i1836 = getelementptr inbounds nuw i8, ptr %_221.i.i1833, i64 16, !dbg !17679
  %65 = getelementptr inbounds nuw i8, ptr %_221.i.i1833, i64 12, !dbg !17681
  %_3.i.i.i1837 = load i32, ptr %65, align 4, !dbg !17681, !alias.scope !17683, !noalias !17688, !noundef !12
  %66 = icmp eq i32 %_3.i.i.i1837, 0, !dbg !17681
  %_51.i.i.i1838 = load i32, ptr %_221.i.i1833, align 4, !dbg !17681, !alias.scope !17683, !noalias !17688
  %67 = getelementptr inbounds nuw i8, ptr %_221.i.i1833, i64 4, !dbg !17681
  %_72.i.i.i1839 = load i32, ptr %67, align 4, !dbg !17681, !alias.scope !17683, !noalias !17688
  %68 = icmp eq i32 %_51.i.i.i1838, %_72.i.i.i1839, !dbg !17681
  %_0.sroa.0.0.i.i.i1840 = select i1 %66, i1 %68, i1 false, !dbg !17681
  br i1 %_0.sroa.0.0.i.i.i1840, label %bb1.i.i1832, label %bb14.i20, !dbg !17691

bb6.i277:                                         ; preds = %bb1.i.i1832
  %69 = getelementptr inbounds nuw i8, ptr %self, i64 464, !dbg !17692
  %_164.0.i = load ptr, ptr %69, align 8, !dbg !17692, !alias.scope !17622, !noalias !17693, !nonnull !12, !noundef !12
  %70 = getelementptr inbounds nuw i8, ptr %self, i64 472, !dbg !17692
  %_164.1.i = load i64, ptr %70, align 8, !dbg !17692, !alias.scope !17622, !noalias !17693, !noundef !12
  %_8.i1842 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_164.0.i, i64 %_164.1.i, !dbg !17694
  br label %bb1.i.i1843, !dbg !17699

bb1.i.i1843:                                      ; preds = %bb13.i.i1846, %bb6.i277
  %_221.i.i1844 = phi ptr [ %_22.i.i1847, %bb13.i.i1846 ], [ %_164.0.i, %bb6.i277 ]
  %_12.i.i1845 = icmp eq ptr %_221.i.i1844, %_8.i1842, !dbg !17701
  br i1 %_12.i.i1845, label %bb8.i279, label %bb13.i.i1846, !dbg !17704

bb13.i.i1846:                                     ; preds = %bb1.i.i1843
  %_22.i.i1847 = getelementptr inbounds nuw i8, ptr %_221.i.i1844, i64 16, !dbg !17705
  %71 = getelementptr inbounds nuw i8, ptr %_221.i.i1844, i64 12, !dbg !17707
  %_3.i.i.i1848 = load i32, ptr %71, align 4, !dbg !17707, !alias.scope !17709, !noalias !17714, !noundef !12
  %72 = icmp eq i32 %_3.i.i.i1848, 0, !dbg !17707
  %_51.i.i.i1849 = load i32, ptr %_221.i.i1844, align 4, !dbg !17707, !alias.scope !17709, !noalias !17714
  %73 = getelementptr inbounds nuw i8, ptr %_221.i.i1844, i64 4, !dbg !17707
  %_72.i.i.i1850 = load i32, ptr %73, align 4, !dbg !17707, !alias.scope !17709, !noalias !17714
  %74 = icmp eq i32 %_51.i.i.i1849, %_72.i.i.i1850, !dbg !17707
  %_0.sroa.0.0.i.i.i1851 = select i1 %72, i1 %74, i1 false, !dbg !17707
  br i1 %_0.sroa.0.0.i.i.i1851, label %bb1.i.i1843, label %bb14.i20, !dbg !17717

bb8.i279:                                         ; preds = %bb1.i.i1843
  %75 = getelementptr inbounds nuw i8, ptr %self, i64 480, !dbg !17718
  %_165.0.i = load ptr, ptr %75, align 8, !dbg !17718, !alias.scope !17622, !noalias !17693, !nonnull !12, !noundef !12
  %76 = getelementptr inbounds nuw i8, ptr %self, i64 488, !dbg !17718
  %_165.1.i = load i64, ptr %76, align 8, !dbg !17718, !alias.scope !17622, !noalias !17693, !noundef !12
  %_8.i1853 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_165.0.i, i64 %_165.1.i, !dbg !17719
  br label %bb1.i.i1854, !dbg !17724

bb1.i.i1854:                                      ; preds = %bb13.i.i1857, %bb8.i279
  %_221.i.i1855 = phi ptr [ %_22.i.i1858, %bb13.i.i1857 ], [ %_165.0.i, %bb8.i279 ]
  %_12.i.i1856 = icmp eq ptr %_221.i.i1855, %_8.i1853, !dbg !17726
  br i1 %_12.i.i1856, label %bb14.i20, label %bb13.i.i1857, !dbg !17729

bb13.i.i1857:                                     ; preds = %bb1.i.i1854
  %_22.i.i1858 = getelementptr inbounds nuw i8, ptr %_221.i.i1855, i64 16, !dbg !17730
  %77 = getelementptr inbounds nuw i8, ptr %_221.i.i1855, i64 12, !dbg !17732
  %_3.i.i.i1859 = load i32, ptr %77, align 4, !dbg !17732, !alias.scope !17734, !noalias !17739, !noundef !12
  %78 = icmp eq i32 %_3.i.i.i1859, 0, !dbg !17732
  %_51.i.i.i1860 = load i32, ptr %_221.i.i1855, align 4, !dbg !17732, !alias.scope !17734, !noalias !17739
  %79 = getelementptr inbounds nuw i8, ptr %_221.i.i1855, i64 4, !dbg !17732
  %_72.i.i.i1861 = load i32, ptr %79, align 4, !dbg !17732, !alias.scope !17734, !noalias !17739
  %80 = icmp eq i32 %_51.i.i.i1860, %_72.i.i.i1861, !dbg !17732
  %_0.sroa.0.0.i.i.i1862 = select i1 %78, i1 %80, i1 false, !dbg !17732
  br i1 %_0.sroa.0.0.i.i.i1862, label %bb1.i.i1854, label %bb14.i20, !dbg !17742

bb14.i20:                                         ; preds = %bb13.i.i1824, %bb13.i.i1835, %bb13.i.i1846, %bb13.i.i1857, %bb1.i.i1854
  %stationary.sroa.0.0.i21 = phi i1 [ false, %bb13.i.i1846 ], [ false, %bb13.i.i1835 ], [ false, %bb13.i.i1857 ], [ true, %bb1.i.i1854 ], [ false, %bb13.i.i1824 ], !dbg !17743
  %81 = getelementptr inbounds nuw i8, ptr %self, i64 776, !dbg !17744
  %82 = load i8, ptr %81, align 4, !dbg !17744, !range !5399, !alias.scope !17616, !noalias !17748, !noundef !12
  %83 = getelementptr inbounds nuw i8, ptr %self, i64 777, !dbg !17749
  %84 = load i8, ptr %83, align 1, !dbg !17749, !range !5399, !alias.scope !17616, !noalias !17748, !noundef !12
  %_41.i = load i32, ptr %_35, align 4, !dbg !17751, !alias.scope !17624, !noalias !17753, !noundef !12
  %85 = getelementptr inbounds nuw i8, ptr %self, i64 564, !dbg !17754
  %_43.i29 = load i32, ptr %85, align 4, !dbg !17754, !alias.scope !17624, !noalias !17753, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i), !dbg !17756, !noalias !17630
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i, i8 0, i64 32, i1 false), !noalias !17630
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i16), !dbg !17758, !noalias !17630
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i16, i8 0, i64 1024, i1 false), !noalias !17630
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i15), !dbg !17760, !noalias !17630
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i15, i8 0, i64 1024, i1 false), !noalias !17630
  %_38.i25 = zext nneg i8 %82 to i32, !dbg !17744
  %.none.i26 = sub nsw i32 0, %_38.i25, !dbg !17762
  %_39.i27 = zext nneg i8 %84 to i32, !dbg !17749
  %all.sroa.0.0.i28 = sub nsw i32 0, %_39.i27, !dbg !17749
  %_121.not.i3543 = icmp eq i64 %frames, 0, !dbg !17763
  br i1 %_121.not.i3543, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_lanefEB2_.exit, label %bb54.i.lr.ph, !dbg !17763

bb54.i.lr.ph:                                     ; preds = %bb14.i20
  %86 = zext i32 %_43.i29 to i64, !dbg !17754
  %87 = zext i32 %_41.i to i64, !dbg !17751
  %d9.i.i = lshr i64 %frames, 5, !dbg !17773
  %r2.i.i = and i64 %frames, 31, !dbg !17779
  %_19.not.i.i = icmp ne i64 %r2.i.i, 0, !dbg !17780
  %88 = zext i1 %_19.not.i.i to i64, !dbg !17780
  %yield_count.sroa.0.0.i.i = add nuw nsw i64 %d9.i.i, %88, !dbg !17780
  %history.i133.i.sroa.7.0.hot_left.i18.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i18, i64 4
  %history.i133.i.sroa.10.0.hot_left.i18.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i18, i64 8
  %history.i133.i.sroa.13.0.hot_left.i18.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i18, i64 12
  %history.i133.i.sroa.16.0.hot_left.i18.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i18, i64 16
  %history.i133.i.sroa.19.0.hot_left.i18.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i18, i64 20
  %history.i133.i.sroa.22.0.hot_left.i18.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i18, i64 24
  %history.i133.i.sroa.26.0.hot_left.i18.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i18, i64 28
  %history.i133.i.sroa.29.0.hot_left.i18.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i18, i64 32
  %history.i133.i.sroa.32.0.hot_left.i18.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i18, i64 36
  %history.i133.i.sroa.35.0.hot_left.i18.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i18, i64 40
  %history.i133.i.sroa.38.0.hot_left.i18.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i18, i64 44
  %89 = getelementptr inbounds nuw i8, ptr %self, i64 588
  %90 = getelementptr inbounds nuw i8, ptr %self, i64 592
  %91 = getelementptr inbounds nuw i8, ptr %self, i64 596
  %row1.i.i.i169.i = getelementptr inbounds nuw i8, ptr %self, i64 600
  %92 = getelementptr inbounds nuw i8, ptr %self, i64 604
  %93 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %94 = getelementptr inbounds nuw i8, ptr %self, i64 612
  %row3.i.i.i183.i = getelementptr inbounds nuw i8, ptr %self, i64 616
  %95 = getelementptr inbounds nuw i8, ptr %self, i64 620
  %96 = getelementptr inbounds nuw i8, ptr %self, i64 624
  %97 = getelementptr inbounds nuw i8, ptr %self, i64 628
  %row5.i.i.i197.i = getelementptr inbounds nuw i8, ptr %self, i64 632
  %98 = getelementptr inbounds nuw i8, ptr %self, i64 636
  %99 = getelementptr inbounds nuw i8, ptr %self, i64 640
  %100 = getelementptr inbounds nuw i8, ptr %self, i64 644
  %row7.i.i.i211.i = getelementptr inbounds nuw i8, ptr %self, i64 648
  %101 = getelementptr inbounds nuw i8, ptr %self, i64 652
  %102 = getelementptr inbounds nuw i8, ptr %self, i64 656
  %103 = getelementptr inbounds nuw i8, ptr %self, i64 660
  %row9.i.i.i225.i = getelementptr inbounds nuw i8, ptr %self, i64 664
  %104 = getelementptr inbounds nuw i8, ptr %self, i64 668
  %105 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %106 = getelementptr inbounds nuw i8, ptr %self, i64 676
  %row11.i.i.i239.i = getelementptr inbounds nuw i8, ptr %self, i64 680
  %107 = getelementptr inbounds nuw i8, ptr %self, i64 684
  %108 = getelementptr inbounds nuw i8, ptr %self, i64 688
  %109 = getelementptr inbounds nuw i8, ptr %self, i64 692
  %row13.i.i.i253.i = getelementptr inbounds nuw i8, ptr %self, i64 696
  %110 = getelementptr inbounds nuw i8, ptr %self, i64 700
  %111 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %112 = getelementptr inbounds nuw i8, ptr %self, i64 708
  %row15.i.i.i267.i = getelementptr inbounds nuw i8, ptr %self, i64 712
  %113 = getelementptr inbounds nuw i8, ptr %self, i64 716
  %114 = getelementptr inbounds nuw i8, ptr %self, i64 720
  %115 = getelementptr inbounds nuw i8, ptr %self, i64 724
  %row17.i.i.i281.i = getelementptr inbounds nuw i8, ptr %self, i64 728
  %116 = getelementptr inbounds nuw i8, ptr %self, i64 732
  %117 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %118 = getelementptr inbounds nuw i8, ptr %self, i64 740
  %row19.i.i.i295.i = getelementptr inbounds nuw i8, ptr %self, i64 744
  %119 = getelementptr inbounds nuw i8, ptr %self, i64 748
  %120 = getelementptr inbounds nuw i8, ptr %self, i64 752
  %121 = getelementptr inbounds nuw i8, ptr %self, i64 756
  %row21.i.i.i309.i = getelementptr inbounds nuw i8, ptr %self, i64 760
  %122 = getelementptr inbounds nuw i8, ptr %self, i64 764
  %123 = getelementptr inbounds nuw i8, ptr %self, i64 768
  %124 = getelementptr inbounds nuw i8, ptr %self, i64 772
  %history.i.i14.sroa.7.0.hot_right.i17.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i17, i64 4
  %history.i.i14.sroa.10.0.hot_right.i17.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i17, i64 8
  %history.i.i14.sroa.13.0.hot_right.i17.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i17, i64 12
  %history.i.i14.sroa.16.0.hot_right.i17.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i17, i64 16
  %history.i.i14.sroa.19.0.hot_right.i17.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i17, i64 20
  %history.i.i14.sroa.22.0.hot_right.i17.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i17, i64 24
  %history.i.i14.sroa.26.0.hot_right.i17.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i17, i64 28
  %history.i.i14.sroa.29.0.hot_right.i17.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i17, i64 32
  %history.i.i14.sroa.32.0.hot_right.i17.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i17, i64 36
  %history.i.i14.sroa.35.0.hot_right.i17.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i17, i64 40
  %history.i.i14.sroa.38.0.hot_right.i17.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i17, i64 44
  %_69.i = getelementptr inbounds nuw i8, ptr %hot_left.i18, i64 48
  %125 = getelementptr inbounds nuw i8, ptr %hot_left.i18, i64 60
  %126 = getelementptr inbounds nuw i8, ptr %hot_left.i18, i64 56
  %127 = getelementptr inbounds nuw i8, ptr %hot_left.i18, i64 52
  %_71.i = getelementptr inbounds nuw i8, ptr %hot_left.i18, i64 64
  %128 = getelementptr inbounds nuw i8, ptr %hot_left.i18, i64 76
  %129 = getelementptr inbounds nuw i8, ptr %hot_left.i18, i64 72
  %130 = getelementptr inbounds nuw i8, ptr %hot_left.i18, i64 68
  %_73.i45 = getelementptr inbounds nuw i8, ptr %hot_right.i17, i64 48
  %131 = getelementptr inbounds nuw i8, ptr %hot_right.i17, i64 60
  %132 = getelementptr inbounds nuw i8, ptr %hot_right.i17, i64 56
  %133 = getelementptr inbounds nuw i8, ptr %hot_right.i17, i64 52
  %_75.i = getelementptr inbounds nuw i8, ptr %hot_right.i17, i64 64
  %134 = getelementptr inbounds nuw i8, ptr %hot_right.i17, i64 76
  %135 = getelementptr inbounds nuw i8, ptr %hot_right.i17, i64 72
  %136 = getelementptr inbounds nuw i8, ptr %hot_right.i17, i64 68
  %_9.i1472 = add nsw i32 %_38.i25, -1
  %137 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %138 = getelementptr inbounds nuw i8, ptr %self, i64 328
  %139 = getelementptr inbounds nuw i8, ptr %self, i64 176
  %140 = getelementptr inbounds nuw i8, ptr %self, i64 168
  %141 = getelementptr inbounds nuw i8, ptr %self, i64 256
  %142 = getelementptr inbounds nuw i8, ptr %self, i64 248
  %143 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %144 = getelementptr inbounds nuw i8, ptr %self, i64 216
  %145 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %146 = getelementptr inbounds nuw i8, ptr %self, i64 184
  %147 = getelementptr inbounds nuw i8, ptr %hot_left.i18, i64 84
  %148 = getelementptr inbounds nuw i8, ptr %hot_left.i18, i64 88
  %149 = getelementptr inbounds nuw i8, ptr %hot_left.i18, i64 80
  %150 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %151 = getelementptr inbounds nuw i8, ptr %self, i64 152
  %_9.i1452 = add nsw i32 %_39.i27, -1
  %152 = getelementptr inbounds nuw i8, ptr %self, i64 528
  %153 = getelementptr inbounds nuw i8, ptr %self, i64 376
  %154 = getelementptr inbounds nuw i8, ptr %self, i64 368
  %155 = getelementptr inbounds nuw i8, ptr %self, i64 520
  %156 = getelementptr inbounds nuw i8, ptr %self, i64 512
  %157 = getelementptr inbounds nuw i8, ptr %self, i64 456
  %158 = getelementptr inbounds nuw i8, ptr %self, i64 448
  %159 = getelementptr inbounds nuw i8, ptr %self, i64 424
  %160 = getelementptr inbounds nuw i8, ptr %self, i64 416
  %161 = getelementptr inbounds nuw i8, ptr %self, i64 392
  %162 = getelementptr inbounds nuw i8, ptr %self, i64 384
  %163 = getelementptr inbounds nuw i8, ptr %hot_right.i17, i64 84
  %164 = getelementptr inbounds nuw i8, ptr %hot_right.i17, i64 88
  %165 = getelementptr inbounds nuw i8, ptr %hot_right.i17, i64 80
  %166 = getelementptr inbounds nuw i8, ptr %self, i64 360
  %167 = getelementptr inbounds nuw i8, ptr %self, i64 352
  %168 = getelementptr inbounds nuw i8, ptr %self, i64 552
  %iter.i30.i.sroa.0.0.ptr3350.1 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 4
  %iter.i30.i.sroa.0.0.ptr3350.2 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 8
  %iter.i30.i.sroa.0.0.ptr3350.3 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 12
  %iter.i30.i.sroa.0.0.ptr3350.4 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 16
  %iter.i30.i.sroa.0.0.ptr3350.5 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 20
  %iter.i30.i.sroa.0.0.ptr3350.6 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 24
  %iter.i30.i.sroa.0.0.ptr3350.7 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 28
  %iter.i.i.sroa.0.0.ptr3361.1 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 4
  %iter.i.i.sroa.0.0.ptr3361.2 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 8
  %iter.i.i.sroa.0.0.ptr3361.3 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 12
  %iter.i.i.sroa.0.0.ptr3361.4 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 16
  %iter.i.i.sroa.0.0.ptr3361.5 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 20
  %iter.i.i.sroa.0.0.ptr3361.6 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 24
  %iter.i.i.sroa.0.0.ptr3361.7 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 28
  br label %bb54.i, !dbg !17763

bb28.i.bb25.i.loopexit_crit_edge:                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
  store float %_5.i6143368, ptr %125, align 4, !dbg !17793
  store float %_5.i5903452, ptr %131, align 4, !dbg !17794
  br label %bb25.i.loopexit, !dbg !17795

bb25.i.loopexit:                                  ; preds = %bb28.i.bb25.i.loopexit_crit_edge, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i41
  %ring_cursor.sroa.0.1.i43.lcssa = phi i64 [ %spec.store.select13.i, %bb28.i.bb25.i.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i353546, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i41 ], !dbg !17801
  %main_cursor.sroa.0.1.i44.lcssa = phi i64 [ %spec.store.select.i, %bb28.i.bb25.i.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i363547, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i41 ], !dbg !17802
  %_121.not.i = icmp eq i64 %171, 0, !dbg !17763
  %indvars.iv.next = add i64 %indvars.iv, -32, !dbg !17763
  br i1 %_121.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_lanefEB2_.exit.loopexit, label %bb54.i, !dbg !17763

bb54.i:                                           ; preds = %bb54.i.lr.ph, %bb25.i.loopexit
  %indvars.iv = phi i64 [ %frames, %bb54.i.lr.ph ], [ %indvars.iv.next, %bb25.i.loopexit ]
  %main_cursor.sroa.0.0.i363547 = phi i64 [ %87, %bb54.i.lr.ph ], [ %main_cursor.sroa.0.1.i44.lcssa, %bb25.i.loopexit ]
  %ring_cursor.sroa.0.0.i353546 = phi i64 [ %86, %bb54.i.lr.ph ], [ %ring_cursor.sroa.0.1.i43.lcssa, %bb25.i.loopexit ]
  %iter2.sroa.0.0.i343545 = phi i64 [ %yield_count.sroa.0.0.i.i, %bb54.i.lr.ph ], [ %171, %bb25.i.loopexit ]
  %iter.sroa.0.0.i3544 = phi i64 [ 0, %bb54.i.lr.ph ], [ %170, %bb25.i.loopexit ]
  %169 = call i64 @llvm.umax.i64(i64 %indvars.iv, i64 1), !dbg !17803
  %umax4884 = call i64 @llvm.umin.i64(i64 %169, i64 32), !dbg !17803
  %170 = add i64 %iter.sroa.0.0.i3544, 32, !dbg !17803
  %171 = add i64 %iter2.sroa.0.0.i343545, -1, !dbg !17807
  %history.i133.i.sroa.0.0.copyload = load float, ptr %hot_left.i18, align 4, !dbg !17808, !noalias !17812
  %history.i133.i.sroa.7.0.copyload = load float, ptr %history.i133.i.sroa.7.0.hot_left.i18.sroa_idx, align 4, !dbg !17808, !noalias !17812
  %history.i133.i.sroa.10.0.copyload = load float, ptr %history.i133.i.sroa.10.0.hot_left.i18.sroa_idx, align 4, !dbg !17808, !noalias !17812
  %history.i133.i.sroa.13.0.copyload = load float, ptr %history.i133.i.sroa.13.0.hot_left.i18.sroa_idx, align 4, !dbg !17808, !noalias !17812
  %history.i133.i.sroa.16.0.copyload = load float, ptr %history.i133.i.sroa.16.0.hot_left.i18.sroa_idx, align 4, !dbg !17808, !noalias !17812
  %history.i133.i.sroa.19.0.copyload = load float, ptr %history.i133.i.sroa.19.0.hot_left.i18.sroa_idx, align 4, !dbg !17808, !noalias !17812
  %history.i133.i.sroa.22.0.copyload = load float, ptr %history.i133.i.sroa.22.0.hot_left.i18.sroa_idx, align 4, !dbg !17808, !noalias !17812
  %history.i133.i.sroa.26.0.copyload = load float, ptr %history.i133.i.sroa.26.0.hot_left.i18.sroa_idx, align 4, !dbg !17808, !noalias !17812
  %history.i133.i.sroa.29.0.copyload = load float, ptr %history.i133.i.sroa.29.0.hot_left.i18.sroa_idx, align 4, !dbg !17808, !noalias !17812
  %history.i133.i.sroa.32.0.copyload = load float, ptr %history.i133.i.sroa.32.0.hot_left.i18.sroa_idx, align 4, !dbg !17808, !noalias !17812
  %history.i133.i.sroa.35.0.copyload = load float, ptr %history.i133.i.sroa.35.0.hot_left.i18.sroa_idx, align 4, !dbg !17808, !noalias !17812
  %history.i133.i.sroa.38.0.copyload = load float, ptr %history.i133.i.sroa.38.0.hot_left.i18.sroa_idx, align 4, !dbg !17808, !noalias !17812
  %_20.i136.i3289.not = icmp eq i64 %frames, %iter.sroa.0.0.i3544, !dbg !17817
  br i1 %_20.i136.i3289.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit337.i, label %bb5.i137.i.lr.ph, !dbg !17827

bb5.i137.i.lr.ph:                                 ; preds = %bb54.i
  %_11.i.i.i157.i = load float, ptr %_31, align 4
  %_14.i.i.i160.i = load float, ptr %89, align 4
  %_17.i.i.i163.i = load float, ptr %90, align 4
  %_20.i.i.i166.i = load float, ptr %91, align 4
  %_25.i.i.i171.i = load float, ptr %row1.i.i.i169.i, align 4
  %_28.i.i.i174.i = load float, ptr %92, align 4
  %_31.i.i.i177.i = load float, ptr %93, align 4
  %_34.i.i.i180.i = load float, ptr %94, align 4
  %_39.i.i.i185.i = load float, ptr %row3.i.i.i183.i, align 4
  %_42.i.i.i188.i = load float, ptr %95, align 4
  %_45.i.i.i191.i = load float, ptr %96, align 4
  %_48.i.i.i194.i = load float, ptr %97, align 4
  %_53.i.i.i199.i = load float, ptr %row5.i.i.i197.i, align 4
  %_56.i.i.i202.i = load float, ptr %98, align 4
  %_59.i.i.i205.i = load float, ptr %99, align 4
  %_62.i.i.i208.i = load float, ptr %100, align 4
  %_67.i.i.i213.i = load float, ptr %row7.i.i.i211.i, align 4
  %_70.i.i.i216.i = load float, ptr %101, align 4
  %_73.i.i.i219.i = load float, ptr %102, align 4
  %_76.i.i.i222.i = load float, ptr %103, align 4
  %_81.i.i.i227.i = load float, ptr %row9.i.i.i225.i, align 4
  %_84.i.i.i230.i = load float, ptr %104, align 4
  %_87.i.i.i233.i = load float, ptr %105, align 4
  %_90.i.i.i236.i = load float, ptr %106, align 4
  %_95.i.i.i241.i = load float, ptr %row11.i.i.i239.i, align 4
  %_98.i.i.i244.i = load float, ptr %107, align 4
  %_101.i.i.i247.i = load float, ptr %108, align 4
  %_104.i.i.i250.i = load float, ptr %109, align 4
  %_109.i.i.i255.i = load float, ptr %row13.i.i.i253.i, align 4
  %_112.i.i.i258.i = load float, ptr %110, align 4
  %_115.i.i.i261.i = load float, ptr %111, align 4
  %_118.i.i.i264.i = load float, ptr %112, align 4
  %_123.i.i.i269.i = load float, ptr %row15.i.i.i267.i, align 4
  %_126.i.i.i272.i = load float, ptr %113, align 4
  %_129.i.i.i275.i = load float, ptr %114, align 4
  %_132.i.i.i278.i = load float, ptr %115, align 4
  %_137.i.i.i283.i = load float, ptr %row17.i.i.i281.i, align 4
  %_140.i.i.i286.i = load float, ptr %116, align 4
  %_143.i.i.i289.i = load float, ptr %117, align 4
  %_146.i.i.i292.i = load float, ptr %118, align 4
  %_151.i.i.i297.i = load float, ptr %row19.i.i.i295.i, align 4
  %_154.i.i.i300.i = load float, ptr %119, align 4
  %_157.i.i.i303.i = load float, ptr %120, align 4
  %_160.i.i.i306.i = load float, ptr %121, align 4
  %_165.i.i.i311.i = load float, ptr %row21.i.i.i309.i, align 4
  %_168.i.i.i314.i = load float, ptr %122, align 4
  %_171.i.i.i317.i = load float, ptr %123, align 4
  %_174.i.i.i320.i = load float, ptr %124, align 4
  br label %bb5.i137.i, !dbg !17827

bb5.i137.i:                                       ; preds = %bb5.i137.i.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit
  %iter.sroa.0.0.i135.i3301 = phi i64 [ 0, %bb5.i137.i.lr.ph ], [ %172, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i133.i.sroa.35.03300 = phi float [ %history.i133.i.sroa.35.0.copyload, %bb5.i137.i.lr.ph ], [ %history.i133.i.sroa.32.03299, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i133.i.sroa.32.03299 = phi float [ %history.i133.i.sroa.32.0.copyload, %bb5.i137.i.lr.ph ], [ %history.i133.i.sroa.29.03298, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i133.i.sroa.29.03298 = phi float [ %history.i133.i.sroa.29.0.copyload, %bb5.i137.i.lr.ph ], [ %history.i133.i.sroa.26.03297, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i133.i.sroa.26.03297 = phi float [ %history.i133.i.sroa.26.0.copyload, %bb5.i137.i.lr.ph ], [ %history.i133.i.sroa.22.03296, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i133.i.sroa.22.03296 = phi float [ %history.i133.i.sroa.22.0.copyload, %bb5.i137.i.lr.ph ], [ %history.i133.i.sroa.19.03295, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i133.i.sroa.19.03295 = phi float [ %history.i133.i.sroa.19.0.copyload, %bb5.i137.i.lr.ph ], [ %history.i133.i.sroa.16.03294, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i133.i.sroa.16.03294 = phi float [ %history.i133.i.sroa.16.0.copyload, %bb5.i137.i.lr.ph ], [ %history.i133.i.sroa.13.03293, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i133.i.sroa.13.03293 = phi float [ %history.i133.i.sroa.13.0.copyload, %bb5.i137.i.lr.ph ], [ %history.i133.i.sroa.10.03292, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i133.i.sroa.10.03292 = phi float [ %history.i133.i.sroa.10.0.copyload, %bb5.i137.i.lr.ph ], [ %history.i133.i.sroa.7.03291, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i133.i.sroa.7.03291 = phi float [ %history.i133.i.sroa.7.0.copyload, %bb5.i137.i.lr.ph ], [ %history.i133.i.sroa.0.03290, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i133.i.sroa.0.03290 = phi float [ %history.i133.i.sroa.0.0.copyload, %bb5.i137.i.lr.ph ], [ %_0.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ]
  %172 = add nuw nsw i64 %iter.sroa.0.0.i135.i3301, 1, !dbg !17828
  %_11.i138.i = add nuw nsw i64 %iter.sroa.0.0.i135.i3301, %iter.sroa.0.0.i3544, !dbg !17834
  %_24.i139.i = icmp ugt i64 %_11.i138.i, %left_io.1, !dbg !17836
  br i1 %_24.i139.i, label %bb7.i336.i, label %bb8.i140.i, !dbg !17836, !prof !905

bb8.i140.i:                                       ; preds = %bb5.i137.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17842), !dbg !17845
  %_3.not.i = icmp eq i64 %left_io.1, %_11.i138.i, !dbg !17846
  br i1 %_3.not.i, label %panic.i1177, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit, !dbg !17846

panic.i1177:                                      ; preds = %bb8.i140.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #31, !dbg !17846, !noalias !17848
  unreachable, !dbg !17846

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit: ; preds = %bb8.i140.i
  %_31.i142.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %_11.i138.i, !dbg !17850
  %_0.i = load float, ptr %_31.i142.i, align 4, !dbg !17846, !alias.scope !17842, !noalias !17855, !noundef !12
  %173 = tail call noundef float @llvm.fabs.f32(float %history.i133.i.sroa.19.03295), !dbg !17856
  %_0.i971 = fmul float %_0.i, %_11.i.i.i157.i, !dbg !17862
  %_0.i759 = fadd float %_0.i971, 0.000000e+00, !dbg !17874
  %_0.i970 = fmul float %_0.i, %_14.i.i.i160.i, !dbg !17877
  %_0.i758 = fadd float %_0.i970, 0.000000e+00, !dbg !17879
  %_0.i969 = fmul float %_0.i, %_17.i.i.i163.i, !dbg !17881
  %_0.i757 = fadd float %_0.i969, 0.000000e+00, !dbg !17883
  %_0.i968 = fmul float %_0.i, %_20.i.i.i166.i, !dbg !17885
  %_0.i756 = fadd float %_0.i968, 0.000000e+00, !dbg !17887
  %_0.i967 = fmul float %history.i133.i.sroa.0.03290, %_25.i.i.i171.i, !dbg !17889
  %_0.i755 = fadd float %_0.i759, %_0.i967, !dbg !17893
  %_0.i966 = fmul float %history.i133.i.sroa.0.03290, %_28.i.i.i174.i, !dbg !17895
  %_0.i754 = fadd float %_0.i758, %_0.i966, !dbg !17897
  %_0.i965 = fmul float %history.i133.i.sroa.0.03290, %_31.i.i.i177.i, !dbg !17899
  %_0.i753 = fadd float %_0.i757, %_0.i965, !dbg !17901
  %_0.i964 = fmul float %history.i133.i.sroa.0.03290, %_34.i.i.i180.i, !dbg !17903
  %_0.i752 = fadd float %_0.i756, %_0.i964, !dbg !17905
  %_0.i963 = fmul float %history.i133.i.sroa.7.03291, %_39.i.i.i185.i, !dbg !17907
  %_0.i751 = fadd float %_0.i755, %_0.i963, !dbg !17911
  %_0.i962 = fmul float %history.i133.i.sroa.7.03291, %_42.i.i.i188.i, !dbg !17913
  %_0.i750 = fadd float %_0.i754, %_0.i962, !dbg !17915
  %_0.i961 = fmul float %history.i133.i.sroa.7.03291, %_45.i.i.i191.i, !dbg !17917
  %_0.i749 = fadd float %_0.i753, %_0.i961, !dbg !17919
  %_0.i960 = fmul float %history.i133.i.sroa.7.03291, %_48.i.i.i194.i, !dbg !17921
  %_0.i748 = fadd float %_0.i752, %_0.i960, !dbg !17923
  %_0.i959 = fmul float %history.i133.i.sroa.10.03292, %_53.i.i.i199.i, !dbg !17925
  %_0.i747 = fadd float %_0.i751, %_0.i959, !dbg !17929
  %_0.i958 = fmul float %history.i133.i.sroa.10.03292, %_56.i.i.i202.i, !dbg !17931
  %_0.i746 = fadd float %_0.i750, %_0.i958, !dbg !17933
  %_0.i957 = fmul float %history.i133.i.sroa.10.03292, %_59.i.i.i205.i, !dbg !17935
  %_0.i745 = fadd float %_0.i749, %_0.i957, !dbg !17937
  %_0.i956 = fmul float %history.i133.i.sroa.10.03292, %_62.i.i.i208.i, !dbg !17939
  %_0.i744 = fadd float %_0.i748, %_0.i956, !dbg !17941
  %_0.i955 = fmul float %history.i133.i.sroa.13.03293, %_67.i.i.i213.i, !dbg !17943
  %_0.i743 = fadd float %_0.i747, %_0.i955, !dbg !17947
  %_0.i954 = fmul float %history.i133.i.sroa.13.03293, %_70.i.i.i216.i, !dbg !17949
  %_0.i742 = fadd float %_0.i746, %_0.i954, !dbg !17951
  %_0.i953 = fmul float %history.i133.i.sroa.13.03293, %_73.i.i.i219.i, !dbg !17953
  %_0.i741 = fadd float %_0.i745, %_0.i953, !dbg !17955
  %_0.i952 = fmul float %history.i133.i.sroa.13.03293, %_76.i.i.i222.i, !dbg !17957
  %_0.i740 = fadd float %_0.i744, %_0.i952, !dbg !17959
  %_0.i951 = fmul float %history.i133.i.sroa.16.03294, %_81.i.i.i227.i, !dbg !17961
  %_0.i739 = fadd float %_0.i743, %_0.i951, !dbg !17965
  %_0.i950 = fmul float %history.i133.i.sroa.16.03294, %_84.i.i.i230.i, !dbg !17967
  %_0.i738 = fadd float %_0.i742, %_0.i950, !dbg !17969
  %_0.i949 = fmul float %history.i133.i.sroa.16.03294, %_87.i.i.i233.i, !dbg !17971
  %_0.i737 = fadd float %_0.i741, %_0.i949, !dbg !17973
  %_0.i948 = fmul float %history.i133.i.sroa.16.03294, %_90.i.i.i236.i, !dbg !17975
  %_0.i736 = fadd float %_0.i740, %_0.i948, !dbg !17977
  %_0.i947 = fmul float %history.i133.i.sroa.19.03295, %_95.i.i.i241.i, !dbg !17979
  %_0.i735 = fadd float %_0.i739, %_0.i947, !dbg !17983
  %_0.i946 = fmul float %history.i133.i.sroa.19.03295, %_98.i.i.i244.i, !dbg !17985
  %_0.i734 = fadd float %_0.i738, %_0.i946, !dbg !17987
  %_0.i945 = fmul float %history.i133.i.sroa.19.03295, %_101.i.i.i247.i, !dbg !17989
  %_0.i733 = fadd float %_0.i737, %_0.i945, !dbg !17991
  %_0.i944 = fmul float %history.i133.i.sroa.19.03295, %_104.i.i.i250.i, !dbg !17993
  %_0.i732 = fadd float %_0.i736, %_0.i944, !dbg !17995
  %_0.i943 = fmul float %history.i133.i.sroa.22.03296, %_109.i.i.i255.i, !dbg !17997
  %_0.i731 = fadd float %_0.i735, %_0.i943, !dbg !18001
  %_0.i942 = fmul float %history.i133.i.sroa.22.03296, %_112.i.i.i258.i, !dbg !18003
  %_0.i730 = fadd float %_0.i734, %_0.i942, !dbg !18005
  %_0.i941 = fmul float %history.i133.i.sroa.22.03296, %_115.i.i.i261.i, !dbg !18007
  %_0.i729 = fadd float %_0.i733, %_0.i941, !dbg !18009
  %_0.i940 = fmul float %history.i133.i.sroa.22.03296, %_118.i.i.i264.i, !dbg !18011
  %_0.i728 = fadd float %_0.i732, %_0.i940, !dbg !18013
  %_0.i939 = fmul float %history.i133.i.sroa.26.03297, %_123.i.i.i269.i, !dbg !18015
  %_0.i727 = fadd float %_0.i731, %_0.i939, !dbg !18019
  %_0.i938 = fmul float %history.i133.i.sroa.26.03297, %_126.i.i.i272.i, !dbg !18021
  %_0.i726 = fadd float %_0.i730, %_0.i938, !dbg !18023
  %_0.i937 = fmul float %history.i133.i.sroa.26.03297, %_129.i.i.i275.i, !dbg !18025
  %_0.i725 = fadd float %_0.i729, %_0.i937, !dbg !18027
  %_0.i936 = fmul float %history.i133.i.sroa.26.03297, %_132.i.i.i278.i, !dbg !18029
  %_0.i724 = fadd float %_0.i728, %_0.i936, !dbg !18031
  %_0.i935 = fmul float %history.i133.i.sroa.29.03298, %_137.i.i.i283.i, !dbg !18033
  %_0.i723 = fadd float %_0.i727, %_0.i935, !dbg !18037
  %_0.i934 = fmul float %history.i133.i.sroa.29.03298, %_140.i.i.i286.i, !dbg !18039
  %_0.i722 = fadd float %_0.i726, %_0.i934, !dbg !18041
  %_0.i933 = fmul float %history.i133.i.sroa.29.03298, %_143.i.i.i289.i, !dbg !18043
  %_0.i721 = fadd float %_0.i725, %_0.i933, !dbg !18045
  %_0.i932 = fmul float %history.i133.i.sroa.29.03298, %_146.i.i.i292.i, !dbg !18047
  %_0.i720 = fadd float %_0.i724, %_0.i932, !dbg !18049
  %_0.i931 = fmul float %history.i133.i.sroa.32.03299, %_151.i.i.i297.i, !dbg !18051
  %_0.i719 = fadd float %_0.i723, %_0.i931, !dbg !18055
  %_0.i930 = fmul float %history.i133.i.sroa.32.03299, %_154.i.i.i300.i, !dbg !18057
  %_0.i718 = fadd float %_0.i722, %_0.i930, !dbg !18059
  %_0.i929 = fmul float %history.i133.i.sroa.32.03299, %_157.i.i.i303.i, !dbg !18061
  %_0.i717 = fadd float %_0.i721, %_0.i929, !dbg !18063
  %_0.i928 = fmul float %history.i133.i.sroa.32.03299, %_160.i.i.i306.i, !dbg !18065
  %_0.i716 = fadd float %_0.i720, %_0.i928, !dbg !18067
  %_0.i927 = fmul float %history.i133.i.sroa.35.03300, %_165.i.i.i311.i, !dbg !18069
  %_0.i715 = fadd float %_0.i719, %_0.i927, !dbg !18073
  %_0.i926 = fmul float %history.i133.i.sroa.35.03300, %_168.i.i.i314.i, !dbg !18075
  %_0.i714 = fadd float %_0.i718, %_0.i926, !dbg !18077
  %_0.i925 = fmul float %history.i133.i.sroa.35.03300, %_171.i.i.i317.i, !dbg !18079
  %_0.i713 = fadd float %_0.i717, %_0.i925, !dbg !18081
  %_0.i924 = fmul float %history.i133.i.sroa.35.03300, %_174.i.i.i320.i, !dbg !18083
  %_0.i712 = fadd float %_0.i716, %_0.i924, !dbg !18085
  %174 = tail call noundef float @llvm.fabs.f32(float %_0.i715), !dbg !18087
  %_3.i.i1566.inv = fcmp ogt float %173, %174, !dbg !18091
  %_4.i.i.v = select i1 %_3.i.i1566.inv, float %173, float %174, !dbg !18091
  %175 = tail call noundef float @llvm.fabs.f32(float %_0.i714), !dbg !18087
  %_3.i.i1566.inv.1 = fcmp ogt float %_4.i.i.v, %175, !dbg !18091
  %_4.i.i.v.1 = select i1 %_3.i.i1566.inv.1, float %_4.i.i.v, float %175, !dbg !18091
  %176 = tail call noundef float @llvm.fabs.f32(float %_0.i713), !dbg !18087
  %_3.i.i1566.inv.2 = fcmp ogt float %_4.i.i.v.1, %176, !dbg !18091
  %_4.i.i.v.2 = select i1 %_3.i.i1566.inv.2, float %_4.i.i.v.1, float %176, !dbg !18091
  %177 = tail call noundef float @llvm.fabs.f32(float %_0.i712), !dbg !18087
  %_3.i.i1566.inv.3 = fcmp ogt float %_4.i.i.v.2, %177, !dbg !18091
  %_4.i.i.v.3 = select i1 %_3.i.i1566.inv.3, float %_4.i.i.v.2, float %177, !dbg !18091
  %_39.i331.i = getelementptr inbounds nuw float, ptr %peaks_left.i16, i64 %iter.sroa.0.0.i135.i3301, !dbg !18097
  store float %_4.i.i.v.3, ptr %_39.i331.i, align 4, !dbg !18108, !alias.scope !18110, !noalias !17855
  %exitcond.not = icmp eq i64 %172, %umax4884, !dbg !17817
  br i1 %exitcond.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit337.i, label %bb5.i137.i, !dbg !17827

bb7.i336.i:                                       ; preds = %bb5.i137.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_11.i138.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e0592aef22128a0ac53753b9632a8183) #31, !dbg !18113, !noalias !17855
  unreachable, !dbg !18113

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit337.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit, %bb54.i
  %history.i133.i.sroa.0.0.lcssa = phi float [ %history.i133.i.sroa.0.0.copyload, %bb54.i ], [ %_0.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !18114
  %history.i133.i.sroa.7.0.lcssa = phi float [ %history.i133.i.sroa.7.0.copyload, %bb54.i ], [ %history.i133.i.sroa.0.03290, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !18114
  %history.i133.i.sroa.10.0.lcssa = phi float [ %history.i133.i.sroa.10.0.copyload, %bb54.i ], [ %history.i133.i.sroa.7.03291, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !18114
  %history.i133.i.sroa.13.0.lcssa = phi float [ %history.i133.i.sroa.13.0.copyload, %bb54.i ], [ %history.i133.i.sroa.10.03292, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !18114
  %history.i133.i.sroa.16.0.lcssa = phi float [ %history.i133.i.sroa.16.0.copyload, %bb54.i ], [ %history.i133.i.sroa.13.03293, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !18114
  %history.i133.i.sroa.19.0.lcssa = phi float [ %history.i133.i.sroa.19.0.copyload, %bb54.i ], [ %history.i133.i.sroa.16.03294, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !18114
  %history.i133.i.sroa.22.0.lcssa = phi float [ %history.i133.i.sroa.22.0.copyload, %bb54.i ], [ %history.i133.i.sroa.19.03295, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !18114
  %history.i133.i.sroa.26.0.lcssa = phi float [ %history.i133.i.sroa.26.0.copyload, %bb54.i ], [ %history.i133.i.sroa.22.03296, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !18114
  %history.i133.i.sroa.29.0.lcssa = phi float [ %history.i133.i.sroa.29.0.copyload, %bb54.i ], [ %history.i133.i.sroa.26.03297, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !18114
  %history.i133.i.sroa.32.0.lcssa = phi float [ %history.i133.i.sroa.32.0.copyload, %bb54.i ], [ %history.i133.i.sroa.29.03298, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !18114
  %history.i133.i.sroa.35.0.lcssa = phi float [ %history.i133.i.sroa.35.0.copyload, %bb54.i ], [ %history.i133.i.sroa.32.03299, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !18114
  %history.i133.i.sroa.38.0.lcssa = phi float [ %history.i133.i.sroa.38.0.copyload, %bb54.i ], [ %history.i133.i.sroa.35.03300, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !18114
  store float %history.i133.i.sroa.0.0.lcssa, ptr %hot_left.i18, align 4, !dbg !18115, !noalias !17812
  store float %history.i133.i.sroa.7.0.lcssa, ptr %history.i133.i.sroa.7.0.hot_left.i18.sroa_idx, align 4, !dbg !18115, !noalias !17812
  store float %history.i133.i.sroa.10.0.lcssa, ptr %history.i133.i.sroa.10.0.hot_left.i18.sroa_idx, align 4, !dbg !18115, !noalias !17812
  store float %history.i133.i.sroa.13.0.lcssa, ptr %history.i133.i.sroa.13.0.hot_left.i18.sroa_idx, align 4, !dbg !18115, !noalias !17812
  store float %history.i133.i.sroa.16.0.lcssa, ptr %history.i133.i.sroa.16.0.hot_left.i18.sroa_idx, align 4, !dbg !18115, !noalias !17812
  store float %history.i133.i.sroa.19.0.lcssa, ptr %history.i133.i.sroa.19.0.hot_left.i18.sroa_idx, align 4, !dbg !18115, !noalias !17812
  store float %history.i133.i.sroa.22.0.lcssa, ptr %history.i133.i.sroa.22.0.hot_left.i18.sroa_idx, align 4, !dbg !18115, !noalias !17812
  store float %history.i133.i.sroa.26.0.lcssa, ptr %history.i133.i.sroa.26.0.hot_left.i18.sroa_idx, align 4, !dbg !18115, !noalias !17812
  store float %history.i133.i.sroa.29.0.lcssa, ptr %history.i133.i.sroa.29.0.hot_left.i18.sroa_idx, align 4, !dbg !18115, !noalias !17812
  store float %history.i133.i.sroa.32.0.lcssa, ptr %history.i133.i.sroa.32.0.hot_left.i18.sroa_idx, align 4, !dbg !18115, !noalias !17812
  store float %history.i133.i.sroa.35.0.lcssa, ptr %history.i133.i.sroa.35.0.hot_left.i18.sroa_idx, align 4, !dbg !18115, !noalias !17812
  store float %history.i133.i.sroa.38.0.lcssa, ptr %history.i133.i.sroa.38.0.hot_left.i18.sroa_idx, align 4, !dbg !18115, !noalias !17812
  %history.i.i14.sroa.0.0.copyload = load float, ptr %hot_right.i17, align 4, !dbg !18116, !noalias !18118
  %history.i.i14.sroa.7.0.copyload = load float, ptr %history.i.i14.sroa.7.0.hot_right.i17.sroa_idx, align 4, !dbg !18116, !noalias !18118
  %history.i.i14.sroa.10.0.copyload = load float, ptr %history.i.i14.sroa.10.0.hot_right.i17.sroa_idx, align 4, !dbg !18116, !noalias !18118
  %history.i.i14.sroa.13.0.copyload = load float, ptr %history.i.i14.sroa.13.0.hot_right.i17.sroa_idx, align 4, !dbg !18116, !noalias !18118
  %history.i.i14.sroa.16.0.copyload = load float, ptr %history.i.i14.sroa.16.0.hot_right.i17.sroa_idx, align 4, !dbg !18116, !noalias !18118
  %history.i.i14.sroa.19.0.copyload = load float, ptr %history.i.i14.sroa.19.0.hot_right.i17.sroa_idx, align 4, !dbg !18116, !noalias !18118
  %history.i.i14.sroa.22.0.copyload = load float, ptr %history.i.i14.sroa.22.0.hot_right.i17.sroa_idx, align 4, !dbg !18116, !noalias !18118
  %history.i.i14.sroa.26.0.copyload = load float, ptr %history.i.i14.sroa.26.0.hot_right.i17.sroa_idx, align 4, !dbg !18116, !noalias !18118
  %history.i.i14.sroa.29.0.copyload = load float, ptr %history.i.i14.sroa.29.0.hot_right.i17.sroa_idx, align 4, !dbg !18116, !noalias !18118
  %history.i.i14.sroa.32.0.copyload = load float, ptr %history.i.i14.sroa.32.0.hot_right.i17.sroa_idx, align 4, !dbg !18116, !noalias !18118
  %history.i.i14.sroa.35.0.copyload = load float, ptr %history.i.i14.sroa.35.0.hot_right.i17.sroa_idx, align 4, !dbg !18116, !noalias !18118
  %history.i.i14.sroa.38.0.copyload = load float, ptr %history.i.i14.sroa.38.0.hot_right.i17.sroa_idx, align 4, !dbg !18116, !noalias !18118
  br i1 %_20.i136.i3289.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i41, label %bb5.i.i74.lr.ph, !dbg !18123

bb5.i.i74.lr.ph:                                  ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit337.i
  %_11.i.i.i.i93 = load float, ptr %_31, align 4
  %_14.i.i.i.i96 = load float, ptr %89, align 4
  %_17.i.i.i.i99 = load float, ptr %90, align 4
  %_20.i.i.i.i102 = load float, ptr %91, align 4
  %_25.i.i.i.i107 = load float, ptr %row1.i.i.i169.i, align 4
  %_28.i.i.i.i110 = load float, ptr %92, align 4
  %_31.i.i.i.i113 = load float, ptr %93, align 4
  %_34.i.i.i.i116 = load float, ptr %94, align 4
  %_39.i.i.i.i121 = load float, ptr %row3.i.i.i183.i, align 4
  %_42.i.i.i.i124 = load float, ptr %95, align 4
  %_45.i.i.i.i127 = load float, ptr %96, align 4
  %_48.i.i.i.i130 = load float, ptr %97, align 4
  %_53.i.i.i.i135 = load float, ptr %row5.i.i.i197.i, align 4
  %_56.i.i.i.i138 = load float, ptr %98, align 4
  %_59.i.i.i.i141 = load float, ptr %99, align 4
  %_62.i.i.i.i144 = load float, ptr %100, align 4
  %_67.i.i.i.i149 = load float, ptr %row7.i.i.i211.i, align 4
  %_70.i.i.i.i152 = load float, ptr %101, align 4
  %_73.i.i.i.i155 = load float, ptr %102, align 4
  %_76.i.i.i.i158 = load float, ptr %103, align 4
  %_81.i.i.i.i163 = load float, ptr %row9.i.i.i225.i, align 4
  %_84.i.i.i.i166 = load float, ptr %104, align 4
  %_87.i.i.i.i169 = load float, ptr %105, align 4
  %_90.i.i.i.i172 = load float, ptr %106, align 4
  %_95.i.i.i.i177 = load float, ptr %row11.i.i.i239.i, align 4
  %_98.i.i.i.i180 = load float, ptr %107, align 4
  %_101.i.i.i.i183 = load float, ptr %108, align 4
  %_104.i.i.i.i186 = load float, ptr %109, align 4
  %_109.i.i.i.i191 = load float, ptr %row13.i.i.i253.i, align 4
  %_112.i.i.i.i194 = load float, ptr %110, align 4
  %_115.i.i.i.i197 = load float, ptr %111, align 4
  %_118.i.i.i.i200 = load float, ptr %112, align 4
  %_123.i.i.i.i205 = load float, ptr %row15.i.i.i267.i, align 4
  %_126.i.i.i.i208 = load float, ptr %113, align 4
  %_129.i.i.i.i211 = load float, ptr %114, align 4
  %_132.i.i.i.i214 = load float, ptr %115, align 4
  %_137.i.i.i.i219 = load float, ptr %row17.i.i.i281.i, align 4
  %_140.i.i.i.i222 = load float, ptr %116, align 4
  %_143.i.i.i.i225 = load float, ptr %117, align 4
  %_146.i.i.i.i228 = load float, ptr %118, align 4
  %_151.i.i.i.i233 = load float, ptr %row19.i.i.i295.i, align 4
  %_154.i.i.i.i236 = load float, ptr %119, align 4
  %_157.i.i.i.i239 = load float, ptr %120, align 4
  %_160.i.i.i.i242 = load float, ptr %121, align 4
  %_165.i.i.i.i247 = load float, ptr %row21.i.i.i309.i, align 4
  %_168.i.i.i.i250 = load float, ptr %122, align 4
  %_171.i.i.i.i253 = load float, ptr %123, align 4
  %_174.i.i.i.i256 = load float, ptr %124, align 4
  br label %bb5.i.i74, !dbg !18123

bb5.i.i74:                                        ; preds = %bb5.i.i74.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182
  %iter.sroa.0.0.i.i393327 = phi i64 [ 0, %bb5.i.i74.lr.ph ], [ %178, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182 ]
  %history.i.i14.sroa.35.03326 = phi float [ %history.i.i14.sroa.35.0.copyload, %bb5.i.i74.lr.ph ], [ %history.i.i14.sroa.32.03325, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182 ]
  %history.i.i14.sroa.32.03325 = phi float [ %history.i.i14.sroa.32.0.copyload, %bb5.i.i74.lr.ph ], [ %history.i.i14.sroa.29.03324, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182 ]
  %history.i.i14.sroa.29.03324 = phi float [ %history.i.i14.sroa.29.0.copyload, %bb5.i.i74.lr.ph ], [ %history.i.i14.sroa.26.03323, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182 ]
  %history.i.i14.sroa.26.03323 = phi float [ %history.i.i14.sroa.26.0.copyload, %bb5.i.i74.lr.ph ], [ %history.i.i14.sroa.22.03322, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182 ]
  %history.i.i14.sroa.22.03322 = phi float [ %history.i.i14.sroa.22.0.copyload, %bb5.i.i74.lr.ph ], [ %history.i.i14.sroa.19.03321, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182 ]
  %history.i.i14.sroa.19.03321 = phi float [ %history.i.i14.sroa.19.0.copyload, %bb5.i.i74.lr.ph ], [ %history.i.i14.sroa.16.03320, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182 ]
  %history.i.i14.sroa.16.03320 = phi float [ %history.i.i14.sroa.16.0.copyload, %bb5.i.i74.lr.ph ], [ %history.i.i14.sroa.13.03319, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182 ]
  %history.i.i14.sroa.13.03319 = phi float [ %history.i.i14.sroa.13.0.copyload, %bb5.i.i74.lr.ph ], [ %history.i.i14.sroa.10.03318, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182 ]
  %history.i.i14.sroa.10.03318 = phi float [ %history.i.i14.sroa.10.0.copyload, %bb5.i.i74.lr.ph ], [ %history.i.i14.sroa.7.03317, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182 ]
  %history.i.i14.sroa.7.03317 = phi float [ %history.i.i14.sroa.7.0.copyload, %bb5.i.i74.lr.ph ], [ %history.i.i14.sroa.0.03316, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182 ]
  %history.i.i14.sroa.0.03316 = phi float [ %history.i.i14.sroa.0.0.copyload, %bb5.i.i74.lr.ph ], [ %_0.i1180, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182 ]
  %178 = add nuw nsw i64 %iter.sroa.0.0.i.i393327, 1, !dbg !18126
  %_11.i.i75 = add nuw nsw i64 %iter.sroa.0.0.i.i393327, %iter.sroa.0.0.i3544, !dbg !18129
  %_24.i.i76 = icmp ugt i64 %_11.i.i75, %right_io.1, !dbg !18130
  br i1 %_24.i.i76, label %bb7.i.i272, label %bb8.i.i77, !dbg !18130, !prof !905

bb8.i.i77:                                        ; preds = %bb5.i.i74
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18133), !dbg !18136
  %_3.not.i1178 = icmp eq i64 %right_io.1, %_11.i.i75, !dbg !18137
  br i1 %_3.not.i1178, label %panic.i1181, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182, !dbg !18137

panic.i1181:                                      ; preds = %bb8.i.i77
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #31, !dbg !18137, !noalias !18139
  unreachable, !dbg !18137

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182: ; preds = %bb8.i.i77
  %_31.i124.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %_11.i.i75, !dbg !18141
  %_0.i1180 = load float, ptr %_31.i124.i, align 4, !dbg !18137, !alias.scope !18133, !noalias !18143, !noundef !12
  %179 = tail call noundef float @llvm.fabs.f32(float %history.i.i14.sroa.19.03321), !dbg !18144
  %_0.i1019 = fmul float %_0.i1180, %_11.i.i.i.i93, !dbg !18147
  %_0.i807 = fadd float %_0.i1019, 0.000000e+00, !dbg !18150
  %_0.i1018 = fmul float %_0.i1180, %_14.i.i.i.i96, !dbg !18152
  %_0.i806 = fadd float %_0.i1018, 0.000000e+00, !dbg !18154
  %_0.i1017 = fmul float %_0.i1180, %_17.i.i.i.i99, !dbg !18156
  %_0.i805 = fadd float %_0.i1017, 0.000000e+00, !dbg !18158
  %_0.i1016 = fmul float %_0.i1180, %_20.i.i.i.i102, !dbg !18160
  %_0.i804 = fadd float %_0.i1016, 0.000000e+00, !dbg !18162
  %_0.i1015 = fmul float %history.i.i14.sroa.0.03316, %_25.i.i.i.i107, !dbg !18164
  %_0.i803 = fadd float %_0.i807, %_0.i1015, !dbg !18166
  %_0.i1014 = fmul float %history.i.i14.sroa.0.03316, %_28.i.i.i.i110, !dbg !18168
  %_0.i802 = fadd float %_0.i806, %_0.i1014, !dbg !18170
  %_0.i1013 = fmul float %history.i.i14.sroa.0.03316, %_31.i.i.i.i113, !dbg !18172
  %_0.i801 = fadd float %_0.i805, %_0.i1013, !dbg !18174
  %_0.i1012 = fmul float %history.i.i14.sroa.0.03316, %_34.i.i.i.i116, !dbg !18176
  %_0.i800 = fadd float %_0.i804, %_0.i1012, !dbg !18178
  %_0.i1011 = fmul float %history.i.i14.sroa.7.03317, %_39.i.i.i.i121, !dbg !18180
  %_0.i799 = fadd float %_0.i803, %_0.i1011, !dbg !18182
  %_0.i1010 = fmul float %history.i.i14.sroa.7.03317, %_42.i.i.i.i124, !dbg !18184
  %_0.i798 = fadd float %_0.i802, %_0.i1010, !dbg !18186
  %_0.i1009 = fmul float %history.i.i14.sroa.7.03317, %_45.i.i.i.i127, !dbg !18188
  %_0.i797 = fadd float %_0.i801, %_0.i1009, !dbg !18190
  %_0.i1008 = fmul float %history.i.i14.sroa.7.03317, %_48.i.i.i.i130, !dbg !18192
  %_0.i796 = fadd float %_0.i800, %_0.i1008, !dbg !18194
  %_0.i1007 = fmul float %history.i.i14.sroa.10.03318, %_53.i.i.i.i135, !dbg !18196
  %_0.i795 = fadd float %_0.i799, %_0.i1007, !dbg !18198
  %_0.i1006 = fmul float %history.i.i14.sroa.10.03318, %_56.i.i.i.i138, !dbg !18200
  %_0.i794 = fadd float %_0.i798, %_0.i1006, !dbg !18202
  %_0.i1005 = fmul float %history.i.i14.sroa.10.03318, %_59.i.i.i.i141, !dbg !18204
  %_0.i793 = fadd float %_0.i797, %_0.i1005, !dbg !18206
  %_0.i1004 = fmul float %history.i.i14.sroa.10.03318, %_62.i.i.i.i144, !dbg !18208
  %_0.i792 = fadd float %_0.i796, %_0.i1004, !dbg !18210
  %_0.i1003 = fmul float %history.i.i14.sroa.13.03319, %_67.i.i.i.i149, !dbg !18212
  %_0.i791 = fadd float %_0.i795, %_0.i1003, !dbg !18214
  %_0.i1002 = fmul float %history.i.i14.sroa.13.03319, %_70.i.i.i.i152, !dbg !18216
  %_0.i790 = fadd float %_0.i794, %_0.i1002, !dbg !18218
  %_0.i1001 = fmul float %history.i.i14.sroa.13.03319, %_73.i.i.i.i155, !dbg !18220
  %_0.i789 = fadd float %_0.i793, %_0.i1001, !dbg !18222
  %_0.i1000 = fmul float %history.i.i14.sroa.13.03319, %_76.i.i.i.i158, !dbg !18224
  %_0.i788 = fadd float %_0.i792, %_0.i1000, !dbg !18226
  %_0.i999 = fmul float %history.i.i14.sroa.16.03320, %_81.i.i.i.i163, !dbg !18228
  %_0.i787 = fadd float %_0.i791, %_0.i999, !dbg !18230
  %_0.i998 = fmul float %history.i.i14.sroa.16.03320, %_84.i.i.i.i166, !dbg !18232
  %_0.i786 = fadd float %_0.i790, %_0.i998, !dbg !18234
  %_0.i997 = fmul float %history.i.i14.sroa.16.03320, %_87.i.i.i.i169, !dbg !18236
  %_0.i785 = fadd float %_0.i789, %_0.i997, !dbg !18238
  %_0.i996 = fmul float %history.i.i14.sroa.16.03320, %_90.i.i.i.i172, !dbg !18240
  %_0.i784 = fadd float %_0.i788, %_0.i996, !dbg !18242
  %_0.i995 = fmul float %history.i.i14.sroa.19.03321, %_95.i.i.i.i177, !dbg !18244
  %_0.i783 = fadd float %_0.i787, %_0.i995, !dbg !18246
  %_0.i994 = fmul float %history.i.i14.sroa.19.03321, %_98.i.i.i.i180, !dbg !18248
  %_0.i782 = fadd float %_0.i786, %_0.i994, !dbg !18250
  %_0.i993 = fmul float %history.i.i14.sroa.19.03321, %_101.i.i.i.i183, !dbg !18252
  %_0.i781 = fadd float %_0.i785, %_0.i993, !dbg !18254
  %_0.i992 = fmul float %history.i.i14.sroa.19.03321, %_104.i.i.i.i186, !dbg !18256
  %_0.i780 = fadd float %_0.i784, %_0.i992, !dbg !18258
  %_0.i991 = fmul float %history.i.i14.sroa.22.03322, %_109.i.i.i.i191, !dbg !18260
  %_0.i779 = fadd float %_0.i783, %_0.i991, !dbg !18262
  %_0.i990 = fmul float %history.i.i14.sroa.22.03322, %_112.i.i.i.i194, !dbg !18264
  %_0.i778 = fadd float %_0.i782, %_0.i990, !dbg !18266
  %_0.i989 = fmul float %history.i.i14.sroa.22.03322, %_115.i.i.i.i197, !dbg !18268
  %_0.i777 = fadd float %_0.i781, %_0.i989, !dbg !18270
  %_0.i988 = fmul float %history.i.i14.sroa.22.03322, %_118.i.i.i.i200, !dbg !18272
  %_0.i776 = fadd float %_0.i780, %_0.i988, !dbg !18274
  %_0.i987 = fmul float %history.i.i14.sroa.26.03323, %_123.i.i.i.i205, !dbg !18276
  %_0.i775 = fadd float %_0.i779, %_0.i987, !dbg !18278
  %_0.i986 = fmul float %history.i.i14.sroa.26.03323, %_126.i.i.i.i208, !dbg !18280
  %_0.i774 = fadd float %_0.i778, %_0.i986, !dbg !18282
  %_0.i985 = fmul float %history.i.i14.sroa.26.03323, %_129.i.i.i.i211, !dbg !18284
  %_0.i773 = fadd float %_0.i777, %_0.i985, !dbg !18286
  %_0.i984 = fmul float %history.i.i14.sroa.26.03323, %_132.i.i.i.i214, !dbg !18288
  %_0.i772 = fadd float %_0.i776, %_0.i984, !dbg !18290
  %_0.i983 = fmul float %history.i.i14.sroa.29.03324, %_137.i.i.i.i219, !dbg !18292
  %_0.i771 = fadd float %_0.i775, %_0.i983, !dbg !18294
  %_0.i982 = fmul float %history.i.i14.sroa.29.03324, %_140.i.i.i.i222, !dbg !18296
  %_0.i770 = fadd float %_0.i774, %_0.i982, !dbg !18298
  %_0.i981 = fmul float %history.i.i14.sroa.29.03324, %_143.i.i.i.i225, !dbg !18300
  %_0.i769 = fadd float %_0.i773, %_0.i981, !dbg !18302
  %_0.i980 = fmul float %history.i.i14.sroa.29.03324, %_146.i.i.i.i228, !dbg !18304
  %_0.i768 = fadd float %_0.i772, %_0.i980, !dbg !18306
  %_0.i979 = fmul float %history.i.i14.sroa.32.03325, %_151.i.i.i.i233, !dbg !18308
  %_0.i767 = fadd float %_0.i771, %_0.i979, !dbg !18310
  %_0.i978 = fmul float %history.i.i14.sroa.32.03325, %_154.i.i.i.i236, !dbg !18312
  %_0.i766 = fadd float %_0.i770, %_0.i978, !dbg !18314
  %_0.i977 = fmul float %history.i.i14.sroa.32.03325, %_157.i.i.i.i239, !dbg !18316
  %_0.i765 = fadd float %_0.i769, %_0.i977, !dbg !18318
  %_0.i976 = fmul float %history.i.i14.sroa.32.03325, %_160.i.i.i.i242, !dbg !18320
  %_0.i764 = fadd float %_0.i768, %_0.i976, !dbg !18322
  %_0.i975 = fmul float %history.i.i14.sroa.35.03326, %_165.i.i.i.i247, !dbg !18324
  %_0.i763 = fadd float %_0.i767, %_0.i975, !dbg !18326
  %_0.i974 = fmul float %history.i.i14.sroa.35.03326, %_168.i.i.i.i250, !dbg !18328
  %_0.i762 = fadd float %_0.i766, %_0.i974, !dbg !18330
  %_0.i973 = fmul float %history.i.i14.sroa.35.03326, %_171.i.i.i.i253, !dbg !18332
  %_0.i761 = fadd float %_0.i765, %_0.i973, !dbg !18334
  %_0.i972 = fmul float %history.i.i14.sroa.35.03326, %_174.i.i.i.i256, !dbg !18336
  %_0.i760 = fadd float %_0.i764, %_0.i972, !dbg !18338
  %180 = tail call noundef float @llvm.fabs.f32(float %_0.i763), !dbg !18340
  %_3.i.i1573.inv = fcmp ogt float %179, %180, !dbg !18342
  %_4.i.i1580.v = select i1 %_3.i.i1573.inv, float %179, float %180, !dbg !18342
  %181 = tail call noundef float @llvm.fabs.f32(float %_0.i762), !dbg !18340
  %_3.i.i1573.inv.1 = fcmp ogt float %_4.i.i1580.v, %181, !dbg !18342
  %_4.i.i1580.v.1 = select i1 %_3.i.i1573.inv.1, float %_4.i.i1580.v, float %181, !dbg !18342
  %182 = tail call noundef float @llvm.fabs.f32(float %_0.i761), !dbg !18340
  %_3.i.i1573.inv.2 = fcmp ogt float %_4.i.i1580.v.1, %182, !dbg !18342
  %_4.i.i1580.v.2 = select i1 %_3.i.i1573.inv.2, float %_4.i.i1580.v.1, float %182, !dbg !18342
  %183 = tail call noundef float @llvm.fabs.f32(float %_0.i760), !dbg !18340
  %_3.i.i1573.inv.3 = fcmp ogt float %_4.i.i1580.v.2, %183, !dbg !18342
  %_4.i.i1580.v.3 = select i1 %_3.i.i1573.inv.3, float %_4.i.i1580.v.2, float %183, !dbg !18342
  %_39.i.i267 = getelementptr inbounds nuw float, ptr %peaks_right.i15, i64 %iter.sroa.0.0.i.i393327, !dbg !18345
  store float %_4.i.i1580.v.3, ptr %_39.i.i267, align 4, !dbg !18350, !alias.scope !18352, !noalias !18143
  %exitcond4870.not = icmp eq i64 %178, %umax4884, !dbg !18355
  br i1 %exitcond4870.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i41, label %bb5.i.i74, !dbg !18123

bb7.i.i272:                                       ; preds = %bb5.i.i74
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_11.i.i75, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e0592aef22128a0ac53753b9632a8183) #31, !dbg !18357, !noalias !18143
  unreachable, !dbg !18357

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i41: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit337.i
  %history.i.i14.sroa.0.0.lcssa = phi float [ %history.i.i14.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit337.i ], [ %_0.i1180, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182 ], !dbg !18358
  %history.i.i14.sroa.7.0.lcssa = phi float [ %history.i.i14.sroa.7.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit337.i ], [ %history.i.i14.sroa.0.03316, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182 ], !dbg !18358
  %history.i.i14.sroa.10.0.lcssa = phi float [ %history.i.i14.sroa.10.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit337.i ], [ %history.i.i14.sroa.7.03317, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182 ], !dbg !18358
  %history.i.i14.sroa.13.0.lcssa = phi float [ %history.i.i14.sroa.13.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit337.i ], [ %history.i.i14.sroa.10.03318, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182 ], !dbg !18358
  %history.i.i14.sroa.16.0.lcssa = phi float [ %history.i.i14.sroa.16.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit337.i ], [ %history.i.i14.sroa.13.03319, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182 ], !dbg !18358
  %history.i.i14.sroa.19.0.lcssa = phi float [ %history.i.i14.sroa.19.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit337.i ], [ %history.i.i14.sroa.16.03320, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182 ], !dbg !18358
  %history.i.i14.sroa.22.0.lcssa = phi float [ %history.i.i14.sroa.22.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit337.i ], [ %history.i.i14.sroa.19.03321, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182 ], !dbg !18358
  %history.i.i14.sroa.26.0.lcssa = phi float [ %history.i.i14.sroa.26.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit337.i ], [ %history.i.i14.sroa.22.03322, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182 ], !dbg !18358
  %history.i.i14.sroa.29.0.lcssa = phi float [ %history.i.i14.sroa.29.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit337.i ], [ %history.i.i14.sroa.26.03323, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182 ], !dbg !18358
  %history.i.i14.sroa.32.0.lcssa = phi float [ %history.i.i14.sroa.32.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit337.i ], [ %history.i.i14.sroa.29.03324, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182 ], !dbg !18358
  %history.i.i14.sroa.35.0.lcssa = phi float [ %history.i.i14.sroa.35.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit337.i ], [ %history.i.i14.sroa.32.03325, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182 ], !dbg !18358
  %history.i.i14.sroa.38.0.lcssa = phi float [ %history.i.i14.sroa.38.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit337.i ], [ %history.i.i14.sroa.35.03326, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1182 ], !dbg !18358
  store float %history.i.i14.sroa.0.0.lcssa, ptr %hot_right.i17, align 4, !dbg !18359, !noalias !18118
  store float %history.i.i14.sroa.7.0.lcssa, ptr %history.i.i14.sroa.7.0.hot_right.i17.sroa_idx, align 4, !dbg !18359, !noalias !18118
  store float %history.i.i14.sroa.10.0.lcssa, ptr %history.i.i14.sroa.10.0.hot_right.i17.sroa_idx, align 4, !dbg !18359, !noalias !18118
  store float %history.i.i14.sroa.13.0.lcssa, ptr %history.i.i14.sroa.13.0.hot_right.i17.sroa_idx, align 4, !dbg !18359, !noalias !18118
  store float %history.i.i14.sroa.16.0.lcssa, ptr %history.i.i14.sroa.16.0.hot_right.i17.sroa_idx, align 4, !dbg !18359, !noalias !18118
  store float %history.i.i14.sroa.19.0.lcssa, ptr %history.i.i14.sroa.19.0.hot_right.i17.sroa_idx, align 4, !dbg !18359, !noalias !18118
  store float %history.i.i14.sroa.22.0.lcssa, ptr %history.i.i14.sroa.22.0.hot_right.i17.sroa_idx, align 4, !dbg !18359, !noalias !18118
  store float %history.i.i14.sroa.26.0.lcssa, ptr %history.i.i14.sroa.26.0.hot_right.i17.sroa_idx, align 4, !dbg !18359, !noalias !18118
  store float %history.i.i14.sroa.29.0.lcssa, ptr %history.i.i14.sroa.29.0.hot_right.i17.sroa_idx, align 4, !dbg !18359, !noalias !18118
  store float %history.i.i14.sroa.32.0.lcssa, ptr %history.i.i14.sroa.32.0.hot_right.i17.sroa_idx, align 4, !dbg !18359, !noalias !18118
  store float %history.i.i14.sroa.35.0.lcssa, ptr %history.i.i14.sroa.35.0.hot_right.i17.sroa_idx, align 4, !dbg !18359, !noalias !18118
  store float %history.i.i14.sroa.38.0.lcssa, ptr %history.i.i14.sroa.38.0.hot_right.i17.sroa_idx, align 4, !dbg !18359, !noalias !18118
  br i1 %_20.i136.i3289.not, label %bb25.i.loopexit, label %bb57.i.lr.ph, !dbg !17795

bb57.i.lr.ph:                                     ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i41
  %_13.i62222762278 = load float, ptr %127, align 4
  %_13.i61022792281 = load float, ptr %130, align 4
  %_13.i59822822284 = load float, ptr %133, align 4
  %_13.i58722852287 = load float, ptr %136, align 4
  %_92.i = load i64, ptr %137, align 8
  %_64.i87.i = load float, ptr %148, align 4
  %_64.i.i = load float, ptr %164, align 4
  %_107.i = load i64, ptr %168, align 8
  %.promoted = load float, ptr %125, align 4
  %_69.i.promoted = load float, ptr %_69.i, align 4
  %.promoted3440 = load float, ptr %126, align 4
  %.promoted3443 = load float, ptr %128, align 4
  %_71.i.promoted = load float, ptr %_71.i, align 4
  %.promoted3448 = load float, ptr %129, align 4
  %.promoted3451 = load float, ptr %131, align 4
  %_73.i45.promoted = load float, ptr %_73.i45, align 4
  %.promoted3524 = load float, ptr %132, align 4
  %.promoted3527 = load float, ptr %134, align 4
  %_75.i.promoted = load float, ptr %_75.i, align 4
  %.promoted3532 = load float, ptr %135, align 4
  %.promoted3535 = load float, ptr %147, align 4
  %.promoted3537 = load float, ptr %149, align 4
  %.promoted3539 = load float, ptr %163, align 4
  %.promoted3541 = load float, ptr %165, align 4
  %_69.i.promoted5728 = load float, ptr %_69.i, align 1
  %_73.i45.promoted5765 = load float, ptr %_73.i45, align 1
  br label %bb57.i, !dbg !17795

bb57.i:                                           ; preds = %bb57.i.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262
  %_0.i14225766 = phi float [ %_73.i45.promoted5765, %bb57.i.lr.ph ], [ %_0.i14225767, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262 ]
  %_0.i13965729 = phi float [ %_69.i.promoted5728, %bb57.i.lr.ph ], [ %_0.i13965730, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262 ]
  %_0.i13233542 = phi float [ %.promoted3541, %bb57.i.lr.ph ], [ %_0.i1323, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262 ]
  %_0.i11393540 = phi float [ %.promoted3539, %bb57.i.lr.ph ], [ %_0.i1139, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262 ]
  %_0.i13273538 = phi float [ %.promoted3537, %bb57.i.lr.ph ], [ %_0.i1327, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262 ]
  %_0.i11433536 = phi float [ %.promoted3535, %bb57.i.lr.ph ], [ %_0.i1143, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262 ]
  %_12.i5853534 = phi float [ %.promoted3532, %bb57.i.lr.ph ], [ %_12.i5853533, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262 ]
  %_0.i14353531 = phi float [ %_75.i.promoted, %bb57.i.lr.ph ], [ %_0.i14353530, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262 ]
  %_5.i5833529 = phi float [ %.promoted3527, %bb57.i.lr.ph ], [ %_5.i5833528, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262 ]
  %_12.i5963526 = phi float [ %.promoted3524, %bb57.i.lr.ph ], [ %_12.i5963525, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262 ]
  %_0.i14223523 = phi float [ %_73.i45.promoted, %bb57.i.lr.ph ], [ %_0.i14223522, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262 ]
  %_5.i5903453 = phi float [ %.promoted3451, %bb57.i.lr.ph ], [ %_5.i5903452, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262 ]
  %_12.i6083450 = phi float [ %.promoted3448, %bb57.i.lr.ph ], [ %_12.i6083449, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262 ]
  %_0.i14093447 = phi float [ %_71.i.promoted, %bb57.i.lr.ph ], [ %_0.i14093446, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262 ]
  %_5.i6023445 = phi float [ %.promoted3443, %bb57.i.lr.ph ], [ %_5.i6023444, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262 ]
  %_12.i6203442 = phi float [ %.promoted3440, %bb57.i.lr.ph ], [ %_12.i6203441, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262 ]
  %_0.i13963439 = phi float [ %_69.i.promoted, %bb57.i.lr.ph ], [ %_0.i13963438, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262 ]
  %_5.i6143369 = phi float [ %.promoted, %bb57.i.lr.ph ], [ %_5.i6143368, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262 ]
  %main_cursor.sroa.0.1.i443365 = phi i64 [ %main_cursor.sroa.0.0.i363547, %bb57.i.lr.ph ], [ %spec.store.select.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262 ]
  %ring_cursor.sroa.0.1.i433364 = phi i64 [ %ring_cursor.sroa.0.0.i353546, %bb57.i.lr.ph ], [ %spec.store.select13.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262 ]
  %iter1.sroa.0.0.i423363 = phi i64 [ 0, %bb57.i.lr.ph ], [ %184, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262 ]
  %184 = add nuw nsw i64 %iter1.sroa.0.0.i423363, 1, !dbg !18360
  %_66.i = add nuw nsw i64 %iter1.sroa.0.0.i423363, %iter.sroa.0.0.i3544, !dbg !18366
  br i1 %stationary.sroa.0.0.i21, label %bb35.i, label %bb30.i, !dbg !18367

bb30.i:                                           ; preds = %bb57.i
  %_0.i1132 = fadd float %_5.i6143369, -1.000000e+00, !dbg !18368
  %_3.i.i1538 = fcmp ogt float %_0.i1132, 0.000000e+00, !dbg !18371
  %_0.i.i1544 = select i1 %_3.i.i1538, float %_0.i1132, float 0.000000e+00, !dbg !18375
  %_0.i708 = fadd float %_0.i13963439, %_12.i6203442, !dbg !18377
  %_0.i1396 = select i1 %_3.i.i1538, float %_0.i708, float %_13.i62222762278, !dbg !18379
  %_0.i1389 = select i1 %_3.i.i1538, float %_12.i6203442, float 0.000000e+00, !dbg !18381
  store float %_0.i1389, ptr %126, align 4, !dbg !18383, !alias.scope !18384, !noalias !18387
  %_0.i1133 = fadd float %_5.i6023445, -1.000000e+00, !dbg !18388
  %_3.i.i1545 = fcmp ogt float %_0.i1133, 0.000000e+00, !dbg !18391
  %_0.i.i1551 = select i1 %_3.i.i1545, float %_0.i1133, float 0.000000e+00, !dbg !18394
  store float %_0.i.i1551, ptr %128, align 4, !dbg !18396, !alias.scope !18397, !noalias !18387
  %_0.i709 = fadd float %_0.i14093447, %_12.i6083450, !dbg !18400
  %_0.i1409 = select i1 %_3.i.i1545, float %_0.i709, float %_13.i61022792281, !dbg !18402
  store float %_0.i1409, ptr %_71.i, align 4, !dbg !18404, !alias.scope !18397, !noalias !18387
  %_0.i1402 = select i1 %_3.i.i1545, float %_12.i6083450, float 0.000000e+00, !dbg !18405
  store float %_0.i1402, ptr %129, align 4, !dbg !18407, !alias.scope !18397, !noalias !18387
  %_0.i1134 = fadd float %_5.i5903453, -1.000000e+00, !dbg !18408
  %_3.i.i1552 = fcmp ogt float %_0.i1134, 0.000000e+00, !dbg !18410
  %_0.i.i1558 = select i1 %_3.i.i1552, float %_0.i1134, float 0.000000e+00, !dbg !18413
  %_0.i710 = fadd float %_0.i14223523, %_12.i5963526, !dbg !18415
  %_0.i1422 = select i1 %_3.i.i1552, float %_0.i710, float %_13.i59822822284, !dbg !18417
  %_0.i1415 = select i1 %_3.i.i1552, float %_12.i5963526, float 0.000000e+00, !dbg !18419
  store float %_0.i1415, ptr %132, align 4, !dbg !18421, !alias.scope !18422, !noalias !18387
  %_0.i1135 = fadd float %_5.i5833529, -1.000000e+00, !dbg !18425
  %_3.i.i1559 = fcmp ogt float %_0.i1135, 0.000000e+00, !dbg !18428
  %_0.i.i1565 = select i1 %_3.i.i1559, float %_0.i1135, float 0.000000e+00, !dbg !18431
  store float %_0.i.i1565, ptr %134, align 4, !dbg !18433, !alias.scope !18434, !noalias !18387
  %_0.i711 = fadd float %_0.i14353531, %_12.i5853534, !dbg !18437
  %_0.i1435 = select i1 %_3.i.i1559, float %_0.i711, float %_13.i58722852287, !dbg !18439
  store float %_0.i1435, ptr %_75.i, align 4, !dbg !18441, !alias.scope !18434, !noalias !18387
  %_0.i1428 = select i1 %_3.i.i1559, float %_12.i5853534, float 0.000000e+00, !dbg !18442
  store float %_0.i1428, ptr %135, align 4, !dbg !18444, !alias.scope !18434, !noalias !18387
  br label %bb35.i, !dbg !18445

bb35.i:                                           ; preds = %bb57.i, %bb30.i
  %_0.i14225767 = phi float [ %_0.i1422, %bb30.i ], [ %_0.i14225766, %bb57.i ]
  %_0.i13965730 = phi float [ %_0.i1396, %bb30.i ], [ %_0.i13965729, %bb57.i ]
  %_12.i5853533 = phi float [ %_0.i1428, %bb30.i ], [ %_12.i5853534, %bb57.i ]
  %_0.i14353530 = phi float [ %_0.i1435, %bb30.i ], [ %_0.i14353531, %bb57.i ]
  %_5.i5833528 = phi float [ %_0.i.i1565, %bb30.i ], [ %_5.i5833529, %bb57.i ]
  %_12.i5963525 = phi float [ %_0.i1415, %bb30.i ], [ %_12.i5963526, %bb57.i ]
  %_0.i14223522 = phi float [ %_0.i1422, %bb30.i ], [ %_0.i14223523, %bb57.i ]
  %_5.i5903452 = phi float [ %_0.i.i1558, %bb30.i ], [ %_5.i5903453, %bb57.i ]
  %_12.i6083449 = phi float [ %_0.i1402, %bb30.i ], [ %_12.i6083450, %bb57.i ]
  %_0.i14093446 = phi float [ %_0.i1409, %bb30.i ], [ %_0.i14093447, %bb57.i ]
  %_5.i6023444 = phi float [ %_0.i.i1551, %bb30.i ], [ %_5.i6023445, %bb57.i ]
  %_12.i6203441 = phi float [ %_0.i1389, %bb30.i ], [ %_12.i6203442, %bb57.i ]
  %_0.i13963438 = phi float [ %_0.i1396, %bb30.i ], [ %_0.i13963439, %bb57.i ]
  %_5.i6143368 = phi float [ %_0.i.i1544, %bb30.i ], [ %_5.i6143369, %bb57.i ]
  %_138.i = getelementptr inbounds nuw float, ptr %peaks_left.i16, i64 %iter1.sroa.0.0.i423363, !dbg !18446
  %_0.i1218 = load float, ptr %_138.i, align 4, !dbg !18458, !alias.scope !18460, !noalias !18387, !noundef !12
  %_143.i = getelementptr inbounds nuw float, ptr %peaks_right.i15, i64 %iter1.sroa.0.0.i423363, !dbg !18463
  %_0.i1213 = load float, ptr %_143.i, align 4, !dbg !18474, !alias.scope !18476, !noalias !18387, !noundef !12
  %_3.i.i1600 = fcmp ule float %_0.i1213, %_0.i1218, !dbg !18479
  %_6.i.i1602 = bitcast float %_0.i1213 to i32, !dbg !18483
  %_8.i.i1604 = bitcast float %_0.i1218 to i32, !dbg !18487
  %_4.i.i1607 = select i1 %_3.i.i1600, i32 %_8.i.i1604, i32 %_6.i.i1602, !dbg !18489
  %_5.i1470 = and i32 %_4.i.i1607, %.none.i26, !dbg !18490
  %_7.i1473 = and i32 %_9.i1472, %_8.i.i1604, !dbg !18493
  %_4.i1474 = or disjoint i32 %_5.i1470, %_7.i1473, !dbg !18490
  %_0.i1475 = bitcast i32 %_4.i1474 to float, !dbg !18494
  %_7.i1466 = and i32 %_9.i1472, %_6.i.i1602, !dbg !18497
  %_4.i1467 = or disjoint i32 %_5.i1470, %_7.i1466, !dbg !18500
  %_0.i1468 = bitcast i32 %_4.i1467 to float, !dbg !18501
  %_144.i53 = icmp ugt i64 %_66.i, %left_io.1, !dbg !18503
  br i1 %_144.i53, label %bb61.i, label %bb62.i, !dbg !18503, !prof !905

bb62.i:                                           ; preds = %bb35.i
  %_151.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %_66.i, !dbg !18508
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18513), !dbg !18516
  %_3.not.i1206 = icmp eq i64 %left_io.1, %_66.i, !dbg !18517
  br i1 %_3.not.i1206, label %panic.i1209, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1210, !dbg !18517

panic.i1209:                                      ; preds = %bb62.i
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #31, !dbg !18517, !noalias !18519
  unreachable, !dbg !18517

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1210: ; preds = %bb62.i
  %_0.i1208 = load float, ptr %_151.i, align 4, !dbg !18517, !alias.scope !18513, !noalias !18387, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18520), !dbg !18523
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18524), !dbg !18523
  %width.i31.i = load i64, ptr %138, align 8, !dbg !18526, !alias.scope !18529, !noalias !18530, !noundef !12
  %_3.i690 = fcmp uge float %_0.i13963438, %_0.i1475, !dbg !18533
  %_0.i911 = fdiv float %_0.i13963438, %_0.i1475, !dbg !18537
  %_0.i1461 = select i1 %_3.i690, float 1.000000e+00, float %_0.i911, !dbg !18540
  %_144.1.i36.i = load i64, ptr %139, align 8, !dbg !18542, !alias.scope !18529, !noalias !18530, !noundef !12
  %_22.i37.i = mul i64 %width.i31.i, %ring_cursor.sroa.0.1.i433364, !dbg !18544
  %_92.i38.i = icmp ugt i64 %_22.i37.i, %_144.1.i36.i, !dbg !18545
  br i1 %_92.i38.i, label %bb37.i122.i, label %bb38.i39.i, !dbg !18545, !prof !905

bb38.i39.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1210
  %_144.0.i40.i = load ptr, ptr %140, align 8, !dbg !18542, !alias.scope !18529, !noalias !18530, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18550), !dbg !18553
  %_4.not.i1287 = icmp eq i64 %_144.1.i36.i, %_22.i37.i, !dbg !18554
  br i1 %_4.not.i1287, label %panic.i1289, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1290, !dbg !18554

panic.i1289:                                      ; preds = %bb38.i39.i
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #31, !dbg !18554, !noalias !18556
  unreachable, !dbg !18554

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1290: ; preds = %bb38.i39.i
  %_99.i42.i = getelementptr inbounds nuw float, ptr %_144.0.i40.i, i64 %_22.i37.i, !dbg !18557
  store float %_0.i1461, ptr %_99.i42.i, align 4, !dbg !18554, !alias.scope !18550, !noalias !18562
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18563), !dbg !18566
  %width.i457 = load i64, ptr %138, align 8, !dbg !18567, !alias.scope !18563, !noalias !18569, !noundef !12
  %185 = icmp eq i64 %width.i457, 0, !dbg !18571
  br i1 %185, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit565, label %bb32.i464.lr.ph, !dbg !18571

bb32.i464.lr.ph:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1290
  %_112.1.i467 = load i64, ptr %40, align 8, !alias.scope !18563, !noalias !18569, !noundef !12
  %_112.0.i471 = load ptr, ptr %39, align 8, !nonnull !12
  %186 = add i64 %ring_cursor.sroa.0.1.i433364, 1
  %_23.not.i478 = icmp ult i64 %186, %_92.i
  %187 = select i1 %_23.not.i478, i64 0, i64 %_92.i
  %start1.sroa.0.0.i479 = sub nuw i64 %186, %187
  %_114.1.i482 = load i64, ptr %139, align 8
  %_114.0.i486 = load ptr, ptr %140, align 8, !nonnull !12
  %_116.1.i487 = load i64, ptr %141, align 8
  %_116.0.i491 = load ptr, ptr %142, align 8, !nonnull !12
  %_118.1.i495 = load i64, ptr %143, align 8
  %_118.0.i499 = load ptr, ptr %144, align 8, !nonnull !12
  %_45.i512 = mul i64 %width.i457, %start1.sroa.0.0.i479
  br label %bb32.i464, !dbg !18571

bb32.i464:                                        ; preds = %bb32.i464.lr.ph, %bb31.i527
  %iter.i456.sroa.10.03345 = phi i64 [ %width.i457, %bb32.i464.lr.ph ], [ %188, %bb31.i527 ]
  %iter.i456.sroa.7.03344 = phi i64 [ 0, %bb32.i464.lr.ph ], [ %_9.0.i, %bb31.i527 ]
  %iter.i456.sroa.0.0.idx3343 = phi i64 [ 0, %bb32.i464.lr.ph ], [ %iter.i456.sroa.0.0.add, %bb31.i527 ]
  %iter.i456.sroa.0.0.ptr3346 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 %iter.i456.sroa.0.0.idx3343, !dbg !18573
  %188 = add i64 %iter.i456.sroa.10.03345, -1, !dbg !18573
  %_7.i.i1882 = icmp eq i64 %iter.i456.sroa.0.0.idx3343, 32, !dbg !18574
  br i1 %_7.i.i1882, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit565, label %bb3.i466, !dbg !18578

bb3.i466:                                         ; preds = %bb32.i464
  %iter.i456.sroa.0.0.add = add nuw nsw i64 %iter.i456.sroa.0.0.idx3343, 4, !dbg !18579
  %_9.0.i = add nuw nsw i64 %iter.i456.sroa.7.03344, 1, !dbg !18581
  %exitcond4873.not = icmp eq i64 %iter.i456.sroa.7.03344, %_112.1.i467, !dbg !18582
  br i1 %exitcond4873.not, label %panic.i469, label %bb5.i470, !dbg !18582

bb5.i470:                                         ; preds = %bb3.i466
  %189 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i471, i64 %iter.i456.sroa.7.03344, !dbg !18582
  %shape.i472 = load i32, ptr %189, align 4, !dbg !18582, !noalias !18583, !noundef !12
  %190 = getelementptr inbounds nuw i8, ptr %189, i64 4, !dbg !18582
  %shape3.i473 = load i32, ptr %190, align 4, !dbg !18582, !noalias !18583, !noundef !12
  %window.i474 = zext i32 %shape.i472 to i64, !dbg !18584
  %_19.i475 = zext i32 %shape3.i473 to i64, !dbg !18585
  %191 = add i64 %ring_cursor.sroa.0.1.i433364, %_19.i475, !dbg !18586
  %_20.not.i476 = icmp ult i64 %191, %_92.i, !dbg !18587
  %192 = select i1 %_20.not.i476, i64 0, i64 %_92.i, !dbg !18587
  %spec.select.i477 = sub nuw i64 %191, %192, !dbg !18587
  %_27.i480 = mul i64 %spec.select.i477, %width.i457, !dbg !18588
  %_26.i481 = add i64 %_27.i480, %iter.i456.sroa.7.03344, !dbg !18588
  %_30.i483 = icmp ult i64 %_26.i481, %_114.1.i482, !dbg !18589
  br i1 %_30.i483, label %bb12.i485, label %panic5.i484, !dbg !18589

panic.i469:                                       ; preds = %bb3.i466
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i467, i64 noundef %_112.1.i467, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_077370d5cece7380867993336836eb69) #31, !dbg !18582, !noalias !18583
  unreachable, !dbg !18582

bb12.i485:                                        ; preds = %bb5.i470
  %193 = getelementptr inbounds nuw float, ptr %_114.0.i486, i64 %_26.i481, !dbg !18589
  %194 = load float, ptr %193, align 4, !dbg !18589, !noalias !18583, !noundef !12
  %exitcond4874.not = icmp eq i64 %iter.i456.sroa.7.03344, %_116.1.i487, !dbg !18590
  br i1 %exitcond4874.not, label %panic6.i489, label %bb13.i490, !dbg !18590

panic5.i484:                                      ; preds = %bb5.i470
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i481, i64 noundef %_114.1.i482, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5cae9ea88ed6362f62dbad10291edbd4) #31, !dbg !18589, !noalias !18583
  unreachable, !dbg !18589

bb13.i490:                                        ; preds = %bb12.i485
  %195 = getelementptr inbounds nuw i32, ptr %_116.0.i491, i64 %iter.i456.sroa.7.03344, !dbg !18590
  %_32.i492 = load i32, ptr %195, align 4, !dbg !18590, !noalias !18583, !noundef !12
  %position.i493 = zext i32 %_32.i492 to i64, !dbg !18590
  %196 = icmp eq i32 %_32.i492, 0, !dbg !18591
  br i1 %196, label %bb17.i502, label %bb15.i494, !dbg !18591

panic6.i489:                                      ; preds = %bb12.i485
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i487, i64 noundef %_116.1.i487, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9fdd00ed7abe4ccfd73f0d145a7b741b) #31, !dbg !18590, !noalias !18583
  unreachable, !dbg !18590

bb15.i494:                                        ; preds = %bb13.i490
  %_37.i496 = icmp ult i64 %iter.i456.sroa.7.03344, %_118.1.i495, !dbg !18592
  br i1 %_37.i496, label %bb16.i498, label %panic7.i497, !dbg !18592

bb17.i502:                                        ; preds = %bb35.i563, %bb16.i498, %bb13.i490
  %newest.sroa.0.0.i503 = phi float [ %194, %bb13.i490 ], [ %_35.i500, %bb35.i563 ], [ %194, %bb16.i498 ], !dbg !18593
  %exitcond4875.not = icmp eq i64 %iter.i456.sroa.7.03344, %_118.1.i495, !dbg !18594
  br i1 %exitcond4875.not, label %panic8.i506, label %bb18.i507, !dbg !18594

bb16.i498:                                        ; preds = %bb15.i494
  %197 = getelementptr inbounds nuw float, ptr %_118.0.i499, i64 %iter.i456.sroa.7.03344, !dbg !18592
  %_35.i500 = load float, ptr %197, align 4, !dbg !18592, !noalias !18583, !noundef !12
  %_102.i501 = fcmp olt float %_35.i500, %194, !dbg !18595
  br i1 %_102.i501, label %bb35.i563, label %bb17.i502, !dbg !18595

panic7.i497:                                      ; preds = %bb15.i494
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i456.sroa.7.03344, i64 noundef %_118.1.i495, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_697dc8945f5e5b040d7e9a0b92cb249f) #31, !dbg !18592, !noalias !18583
  unreachable, !dbg !18592

bb35.i563:                                        ; preds = %bb16.i498
  br label %bb17.i502, !dbg !18597

bb18.i507:                                        ; preds = %bb17.i502
  %198 = getelementptr inbounds nuw float, ptr %_118.0.i499, i64 %iter.i456.sroa.7.03344, !dbg !18594
  store float %newest.sroa.0.0.i503, ptr %198, align 4, !dbg !18594, !noalias !18583
  %_42.i509 = add nuw nsw i64 %position.i493, 1, !dbg !18598
  %complete.i510 = icmp eq i64 %_42.i509, %window.i474, !dbg !18598
  br i1 %complete.i510, label %bb22.i532, label %bb20.i511, !dbg !18599

panic8.i506:                                      ; preds = %bb17.i502
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i495, i64 noundef %_118.1.i495, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_067dce244605df4a33956fbd4d726027) #31, !dbg !18594, !noalias !18583
  unreachable, !dbg !18594

bb20.i511:                                        ; preds = %bb18.i507
  %_44.i513 = add i64 %iter.i456.sroa.7.03344, %_45.i512, !dbg !18600
  %_47.i515 = icmp ult i64 %_44.i513, %_114.1.i482, !dbg !18601
  br i1 %_47.i515, label %bb30.i525, label %panic9.i516, !dbg !18601

panic9.i516:                                      ; preds = %bb20.i511
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i513, i64 noundef %_114.1.i482, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d5ea38731a1cb6311efef2f919234d06) #31, !dbg !18601, !noalias !18583
  unreachable, !dbg !18601

bb30.i525:                                        ; preds = %bb20.i511
  %199 = getelementptr inbounds nuw float, ptr %_114.0.i486, i64 %_44.i513, !dbg !18601
  %_43.i519 = load float, ptr %199, align 4, !dbg !18601, !noalias !18583, !noundef !12
  %_103.i520 = fcmp olt float %_43.i519, %newest.sroa.0.0.i503, !dbg !18602
  %newest.sroa.0.1.i521 = select i1 %_103.i520, float %_43.i519, float %newest.sroa.0.0.i503, !dbg !18602
  store float %newest.sroa.0.1.i521, ptr %iter.i456.sroa.0.0.ptr3346, align 4, !dbg !18604, !noalias !18583
  %200 = trunc i64 %_42.i509 to i32, !dbg !18605
  br label %bb31.i527, !dbg !18606

bb31.i527:                                        ; preds = %bb25.i560, %bb30.i525
  %storemerge = phi i32 [ %200, %bb30.i525 ], [ 0, %bb25.i560 ], !dbg !18607
  store i32 %storemerge, ptr %195, align 4, !dbg !18607, !noalias !18583
  %201 = icmp eq i64 %188, 0, !dbg !18571
  br i1 %201, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit565, label %bb32.i464, !dbg !18571

bb22.i532:                                        ; preds = %bb18.i507
  store float %newest.sroa.0.0.i503, ptr %iter.i456.sroa.0.0.ptr3346, align 4, !dbg !18604, !noalias !18583
  %202 = load float, ptr %193, align 4, !dbg !18608, !noalias !18583, !noundef !12
  br label %bb41.i545, !dbg !18609

bb41.i545:                                        ; preds = %bb22.i532, %bb25.i560
  %iter2.sroa.0.0.i5373342 = phi i64 [ 0, %bb22.i532 ], [ %_105.i546, %bb25.i560 ]
  %suffix.sroa.0.0.i5363341 = phi float [ %202, %bb22.i532 ], [ %suffix.sroa.0.1.i556, %bb25.i560 ]
  %end.sroa.0.1.i5353340 = phi i64 [ %spec.select.i477, %bb22.i532 ], [ %205, %bb25.i560 ]
  %_56.i547 = mul i64 %end.sroa.0.1.i5353340, %width.i457, !dbg !18612
  %_55.i548 = add i64 %_56.i547, %iter.i456.sroa.7.03344, !dbg !18612
  %_59.i550 = icmp ult i64 %_55.i548, %_114.1.i482, !dbg !18613
  br i1 %_59.i550, label %bb25.i560, label %panic13.i551, !dbg !18613

panic13.i551:                                     ; preds = %bb41.i545
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i548, i64 noundef %_114.1.i482, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a19fa7d9077791c9eadbe74fafea8de7) #31, !dbg !18613, !noalias !18583
  unreachable, !dbg !18613

bb25.i560:                                        ; preds = %bb41.i545
  %_105.i546 = add nuw nsw i64 %iter2.sroa.0.0.i5373342, 1, !dbg !18614
  %203 = getelementptr inbounds nuw float, ptr %_114.0.i486, i64 %_55.i548, !dbg !18613
  %_54.i554 = load float, ptr %203, align 4, !dbg !18613, !noalias !18583, !noundef !12
  %_107.i555 = fcmp olt float %suffix.sroa.0.0.i5363341, %_54.i554, !dbg !18617
  %suffix.sroa.0.1.i556 = select i1 %_107.i555, float %suffix.sroa.0.0.i5363341, float %_54.i554, !dbg !18617
  store float %suffix.sroa.0.1.i556, ptr %203, align 4, !dbg !18619, !noalias !18583
  %204 = icmp eq i64 %end.sroa.0.1.i5353340, 0, !dbg !18620
  %spec.store.select.i562 = select i1 %204, i64 %_92.i, i64 %end.sroa.0.1.i5353340, !dbg !18620
  %205 = add i64 %spec.store.select.i562, -1, !dbg !18621
  %exitcond4872.not = icmp eq i64 %_105.i546, %window.i474, !dbg !18622
  br i1 %exitcond4872.not, label %bb31.i527, label %bb41.i545, !dbg !18609

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit565: ; preds = %bb31.i527, %bb32.i464, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1290
  %_0.i1205 = load float, ptr %scratch.i, align 4, !dbg !18624, !alias.scope !18626, !noalias !18629, !noundef !12
  %_0.i1025 = fmul float %_0.i1205, 1.638400e+04, !dbg !18630
  %206 = tail call noundef float @llvm.floor.f32(float %_0.i1025), !dbg !18633
  %_0.i1024 = fmul float %206, 0x3F10000000000000, !dbg !18640
  %207 = icmp eq i64 %width.i31.i, 0, !dbg !18642
  %_149.1.i80.i.pre = load i64, ptr %145, align 8, !dbg !18648, !alias.scope !18529, !noalias !18530
  br i1 %207, label %bb16.i75.i, label %bb39.i55.i.lr.ph, !dbg !18642

bb39.i55.i.lr.ph:                                 ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit565
  %_145.1.i58.i = load i64, ptr %40, align 8, !alias.scope !18529, !noalias !18530, !noundef !12
  %_145.0.i62.i = load ptr, ptr %39, align 8, !nonnull !12
  %_147.0.i73.i = load ptr, ptr %146, align 8, !nonnull !12
  %exitcond4876.not = icmp eq i64 %_145.1.i58.i, 0, !dbg !18650
  br i1 %exitcond4876.not, label %panic.i60.i, label %bb17.i61.i, !dbg !18650

bb37.i122.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1210
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i37.i, i64 noundef %_144.1.i36.i, i64 noundef %_144.1.i36.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9d2713d1692431af37d60082290049ed) #31, !dbg !18652, !noalias !18562
  unreachable, !dbg !18652

bb16.i75.i:                                       ; preds = %bb21.i72.i.7, %bb21.i72.i, %bb21.i72.i.1, %bb21.i72.i.2, %bb21.i72.i.3, %bb21.i72.i.4, %bb21.i72.i.5, %bb21.i72.i.6, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit565
  %_0.i1203 = phi float [ %_0.i1205, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit565 ], [ %_49.i74.i, %bb21.i72.i ], [ %_49.i74.i, %bb21.i72.i.7 ], [ %_49.i74.i, %bb21.i72.i.6 ], [ %_49.i74.i, %bb21.i72.i.5 ], [ %_49.i74.i, %bb21.i72.i.4 ], [ %_49.i74.i, %bb21.i72.i.3 ], [ %_49.i74.i, %bb21.i72.i.2 ], [ %_49.i74.i, %bb21.i72.i.1 ], !dbg !18653
  %_0.i809 = fadd float %_0.i1024, %_0.i11433536, !dbg !18655
  %_0.i1143 = fsub float %_0.i809, %_0.i1203, !dbg !18657
  store float %_0.i1143, ptr %147, align 4, !dbg !18659, !alias.scope !18520, !noalias !18660
  %_109.i81.i = icmp ugt i64 %_22.i37.i, %_149.1.i80.i.pre, !dbg !18661
  br i1 %_109.i81.i, label %bb42.i121.i, label %bb43.i82.i, !dbg !18661, !prof !905

bb43.i82.i:                                       ; preds = %bb16.i75.i
  %_149.0.i83.i = load ptr, ptr %146, align 8, !dbg !18648, !alias.scope !18529, !noalias !18530, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18665), !dbg !18668
  %_4.not.i1283 = icmp eq i64 %_149.1.i80.i.pre, %_22.i37.i, !dbg !18669
  br i1 %_4.not.i1283, label %panic.i1285, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1286, !dbg !18669

panic.i1285:                                      ; preds = %bb43.i82.i
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #31, !dbg !18669, !noalias !18671
  unreachable, !dbg !18669

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1286: ; preds = %bb43.i82.i
  %_116.i85.i = getelementptr inbounds nuw float, ptr %_149.0.i83.i, i64 %_22.i37.i, !dbg !18672
  store float %_0.i1024, ptr %_116.i85.i, align 4, !dbg !18669, !alias.scope !18665, !noalias !18629
  %_0.i910 = fdiv float %_0.i1143, %_64.i87.i, !dbg !18677
  %_0.i1142 = fsub float 1.000000e+00, %_0.i910, !dbg !18679
  %_0.i1141 = fsub float %_0.i1142, %_0.i13273538, !dbg !18682
  %_4.i918 = fmul float %_0.i14093446, %_0.i1141, !dbg !18685
  %_0.i919 = fadd float %_0.i13273538, %_4.i918, !dbg !18685
  %_3.i.i1591.inv = fcmp ogt float %_0.i1142, %_0.i919, !dbg !18688
  %_4.i.i1598.v = select i1 %_3.i.i1591.inv, float %_0.i1142, float %_0.i919, !dbg !18688
  %208 = tail call noundef float @llvm.fabs.f32(float %_4.i.i1598.v), !dbg !18692
  %209 = fcmp uge float %208, 0x3BC79CA100000000, !dbg !18696
  %_0.i1327 = select i1 %209, float %_4.i.i1598.v, float 0.000000e+00, !dbg !18699
  store float %_0.i1327, ptr %149, align 4, !dbg !18700, !alias.scope !18520, !noalias !18660
  %_0.i1140 = fsub float 1.000000e+00, %_0.i1327, !dbg !18701
  %_150.1.i99.i = load i64, ptr %150, align 8, !dbg !18703, !alias.scope !18529, !noalias !18530, !noundef !12
  %_76.i100.i = mul i64 %width.i31.i, %main_cursor.sroa.0.1.i443365, !dbg !18705
  %_120.i101.i = icmp ugt i64 %_76.i100.i, %_150.1.i99.i, !dbg !18706
  br i1 %_120.i101.i, label %bb48.i120.i, label %bb49.i102.i, !dbg !18706, !prof !905

bb42.i121.i:                                      ; preds = %bb16.i75.i
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i37.i, i64 noundef %_149.1.i80.i.pre, i64 noundef %_149.1.i80.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8002ed69501742f3ea2ea25eb68cd581) #31, !dbg !18711, !noalias !18629
  unreachable, !dbg !18711

bb49.i102.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1286
  %_150.0.i103.i = load ptr, ptr %151, align 8, !dbg !18703, !alias.scope !18529, !noalias !18530, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18712), !dbg !18715
  %_3.not.i1197 = icmp eq i64 %_150.1.i99.i, %_76.i100.i, !dbg !18716
  br i1 %_3.not.i1197, label %panic.i1200, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1278, !dbg !18716

panic.i1200:                                      ; preds = %bb49.i102.i
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #31, !dbg !18716, !noalias !18718
  unreachable, !dbg !18716

bb48.i120.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1286
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i100.i, i64 noundef %_150.1.i99.i, i64 noundef %_150.1.i99.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a07d19b424a92543209d516ece5bdf2a) #31, !dbg !18719, !noalias !18629
  unreachable, !dbg !18719

bb17.i61.i:                                       ; preds = %bb39.i55.i.lr.ph
  %210 = getelementptr inbounds nuw i8, ptr %_145.0.i62.i, i64 8, !dbg !18650
  %_44.i63.i = load i32, ptr %210, align 4, !dbg !18650, !noalias !18629, !noundef !12
  %_43.i64.i = zext i32 %_44.i63.i to i64, !dbg !18650
  %211 = add i64 %ring_cursor.sroa.0.1.i433364, %_43.i64.i, !dbg !18720
  %_47.not.i65.i = icmp ult i64 %211, %_92.i, !dbg !18721
  %212 = select i1 %_47.not.i65.i, i64 0, i64 %_92.i, !dbg !18721
  %spec.select.i66.i = sub nuw i64 %211, %212, !dbg !18721
  %_51.i67.i = mul i64 %spec.select.i66.i, %width.i31.i, !dbg !18723
  %_53.i70.i = icmp ult i64 %_51.i67.i, %_149.1.i80.i.pre, !dbg !18724
  br i1 %_53.i70.i, label %bb21.i72.i, label %panic1.i71.i, !dbg !18724

panic.i60.i:                                      ; preds = %bb39.i55.i.7, %bb39.i55.i.6, %bb39.i55.i.5, %bb39.i55.i.4, %bb39.i55.i.3, %bb39.i55.i.2, %bb39.i55.i.1, %bb39.i55.i.lr.ph
  %_145.1.i58.i.lcssa.ph = phi i64 [ 7, %bb39.i55.i.7 ], [ 6, %bb39.i55.i.6 ], [ 5, %bb39.i55.i.5 ], [ 4, %bb39.i55.i.4 ], [ 3, %bb39.i55.i.3 ], [ 2, %bb39.i55.i.2 ], [ 1, %bb39.i55.i.1 ], [ 0, %bb39.i55.i.lr.ph ]
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i58.i.lcssa.ph, i64 noundef %_145.1.i58.i.lcssa.ph, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_891dd683d7367e0a1ec5a22f9c2a2aa8) #31, !dbg !18650, !noalias !18629
  unreachable, !dbg !18650

bb21.i72.i:                                       ; preds = %bb17.i61.i
  %213 = getelementptr inbounds nuw float, ptr %_147.0.i73.i, i64 %_51.i67.i, !dbg !18724
  %_49.i74.i = load float, ptr %213, align 4, !dbg !18724, !noalias !18629, !noundef !12
  store float %_49.i74.i, ptr %scratch.i, align 4, !dbg !18725, !noalias !18629
  %214 = icmp eq i64 %width.i31.i, 1, !dbg !18642
  br i1 %214, label %bb16.i75.i, label %bb39.i55.i.1, !dbg !18642

bb39.i55.i.1:                                     ; preds = %bb21.i72.i
  %exitcond4876.1.not = icmp eq i64 %_145.1.i58.i, 1, !dbg !18650
  br i1 %exitcond4876.1.not, label %panic.i60.i, label %bb17.i61.i.1, !dbg !18650

bb17.i61.i.1:                                     ; preds = %bb39.i55.i.1
  %215 = getelementptr inbounds nuw i8, ptr %_145.0.i62.i, i64 20, !dbg !18650
  %_44.i63.i.1 = load i32, ptr %215, align 4, !dbg !18650, !noalias !18629, !noundef !12
  %_43.i64.i.1 = zext i32 %_44.i63.i.1 to i64, !dbg !18650
  %216 = add i64 %ring_cursor.sroa.0.1.i433364, %_43.i64.i.1, !dbg !18720
  %_47.not.i65.i.1 = icmp ult i64 %216, %_92.i, !dbg !18721
  %217 = select i1 %_47.not.i65.i.1, i64 0, i64 %_92.i, !dbg !18721
  %spec.select.i66.i.1 = sub nuw i64 %216, %217, !dbg !18721
  %_51.i67.i.1 = mul i64 %spec.select.i66.i.1, %width.i31.i, !dbg !18723
  %_50.i68.i.1 = add i64 %_51.i67.i.1, 1, !dbg !18723
  %_53.i70.i.1 = icmp ult i64 %_50.i68.i.1, %_149.1.i80.i.pre, !dbg !18724
  br i1 %_53.i70.i.1, label %bb21.i72.i.1, label %panic1.i71.i, !dbg !18724

bb21.i72.i.1:                                     ; preds = %bb17.i61.i.1
  %218 = getelementptr inbounds nuw float, ptr %_147.0.i73.i, i64 %_50.i68.i.1, !dbg !18724
  %_49.i74.i.1 = load float, ptr %218, align 4, !dbg !18724, !noalias !18629, !noundef !12
  store float %_49.i74.i.1, ptr %iter.i30.i.sroa.0.0.ptr3350.1, align 4, !dbg !18725, !noalias !18629
  %219 = icmp eq i64 %width.i31.i, 2, !dbg !18642
  br i1 %219, label %bb16.i75.i, label %bb39.i55.i.2, !dbg !18642

bb39.i55.i.2:                                     ; preds = %bb21.i72.i.1
  %exitcond4876.2.not = icmp eq i64 %_145.1.i58.i, 2, !dbg !18650
  br i1 %exitcond4876.2.not, label %panic.i60.i, label %bb17.i61.i.2, !dbg !18650

bb17.i61.i.2:                                     ; preds = %bb39.i55.i.2
  %220 = getelementptr inbounds nuw i8, ptr %_145.0.i62.i, i64 32, !dbg !18650
  %_44.i63.i.2 = load i32, ptr %220, align 4, !dbg !18650, !noalias !18629, !noundef !12
  %_43.i64.i.2 = zext i32 %_44.i63.i.2 to i64, !dbg !18650
  %221 = add i64 %ring_cursor.sroa.0.1.i433364, %_43.i64.i.2, !dbg !18720
  %_47.not.i65.i.2 = icmp ult i64 %221, %_92.i, !dbg !18721
  %222 = select i1 %_47.not.i65.i.2, i64 0, i64 %_92.i, !dbg !18721
  %spec.select.i66.i.2 = sub nuw i64 %221, %222, !dbg !18721
  %_51.i67.i.2 = mul i64 %spec.select.i66.i.2, %width.i31.i, !dbg !18723
  %_50.i68.i.2 = add i64 %_51.i67.i.2, 2, !dbg !18723
  %_53.i70.i.2 = icmp ult i64 %_50.i68.i.2, %_149.1.i80.i.pre, !dbg !18724
  br i1 %_53.i70.i.2, label %bb21.i72.i.2, label %panic1.i71.i, !dbg !18724

bb21.i72.i.2:                                     ; preds = %bb17.i61.i.2
  %223 = getelementptr inbounds nuw float, ptr %_147.0.i73.i, i64 %_50.i68.i.2, !dbg !18724
  %_49.i74.i.2 = load float, ptr %223, align 4, !dbg !18724, !noalias !18629, !noundef !12
  store float %_49.i74.i.2, ptr %iter.i30.i.sroa.0.0.ptr3350.2, align 4, !dbg !18725, !noalias !18629
  %224 = icmp eq i64 %width.i31.i, 3, !dbg !18642
  br i1 %224, label %bb16.i75.i, label %bb39.i55.i.3, !dbg !18642

bb39.i55.i.3:                                     ; preds = %bb21.i72.i.2
  %exitcond4876.3.not = icmp eq i64 %_145.1.i58.i, 3, !dbg !18650
  br i1 %exitcond4876.3.not, label %panic.i60.i, label %bb17.i61.i.3, !dbg !18650

bb17.i61.i.3:                                     ; preds = %bb39.i55.i.3
  %225 = getelementptr inbounds nuw i8, ptr %_145.0.i62.i, i64 44, !dbg !18650
  %_44.i63.i.3 = load i32, ptr %225, align 4, !dbg !18650, !noalias !18629, !noundef !12
  %_43.i64.i.3 = zext i32 %_44.i63.i.3 to i64, !dbg !18650
  %226 = add i64 %ring_cursor.sroa.0.1.i433364, %_43.i64.i.3, !dbg !18720
  %_47.not.i65.i.3 = icmp ult i64 %226, %_92.i, !dbg !18721
  %227 = select i1 %_47.not.i65.i.3, i64 0, i64 %_92.i, !dbg !18721
  %spec.select.i66.i.3 = sub nuw i64 %226, %227, !dbg !18721
  %_51.i67.i.3 = mul i64 %spec.select.i66.i.3, %width.i31.i, !dbg !18723
  %_50.i68.i.3 = add i64 %_51.i67.i.3, 3, !dbg !18723
  %_53.i70.i.3 = icmp ult i64 %_50.i68.i.3, %_149.1.i80.i.pre, !dbg !18724
  br i1 %_53.i70.i.3, label %bb21.i72.i.3, label %panic1.i71.i, !dbg !18724

bb21.i72.i.3:                                     ; preds = %bb17.i61.i.3
  %228 = getelementptr inbounds nuw float, ptr %_147.0.i73.i, i64 %_50.i68.i.3, !dbg !18724
  %_49.i74.i.3 = load float, ptr %228, align 4, !dbg !18724, !noalias !18629, !noundef !12
  store float %_49.i74.i.3, ptr %iter.i30.i.sroa.0.0.ptr3350.3, align 4, !dbg !18725, !noalias !18629
  %229 = icmp eq i64 %width.i31.i, 4, !dbg !18642
  br i1 %229, label %bb16.i75.i, label %bb39.i55.i.4, !dbg !18642

bb39.i55.i.4:                                     ; preds = %bb21.i72.i.3
  %exitcond4876.4.not = icmp eq i64 %_145.1.i58.i, 4, !dbg !18650
  br i1 %exitcond4876.4.not, label %panic.i60.i, label %bb17.i61.i.4, !dbg !18650

bb17.i61.i.4:                                     ; preds = %bb39.i55.i.4
  %230 = getelementptr inbounds nuw i8, ptr %_145.0.i62.i, i64 56, !dbg !18650
  %_44.i63.i.4 = load i32, ptr %230, align 4, !dbg !18650, !noalias !18629, !noundef !12
  %_43.i64.i.4 = zext i32 %_44.i63.i.4 to i64, !dbg !18650
  %231 = add i64 %ring_cursor.sroa.0.1.i433364, %_43.i64.i.4, !dbg !18720
  %_47.not.i65.i.4 = icmp ult i64 %231, %_92.i, !dbg !18721
  %232 = select i1 %_47.not.i65.i.4, i64 0, i64 %_92.i, !dbg !18721
  %spec.select.i66.i.4 = sub nuw i64 %231, %232, !dbg !18721
  %_51.i67.i.4 = mul i64 %spec.select.i66.i.4, %width.i31.i, !dbg !18723
  %_50.i68.i.4 = add i64 %_51.i67.i.4, 4, !dbg !18723
  %_53.i70.i.4 = icmp ult i64 %_50.i68.i.4, %_149.1.i80.i.pre, !dbg !18724
  br i1 %_53.i70.i.4, label %bb21.i72.i.4, label %panic1.i71.i, !dbg !18724

bb21.i72.i.4:                                     ; preds = %bb17.i61.i.4
  %233 = getelementptr inbounds nuw float, ptr %_147.0.i73.i, i64 %_50.i68.i.4, !dbg !18724
  %_49.i74.i.4 = load float, ptr %233, align 4, !dbg !18724, !noalias !18629, !noundef !12
  store float %_49.i74.i.4, ptr %iter.i30.i.sroa.0.0.ptr3350.4, align 4, !dbg !18725, !noalias !18629
  %234 = icmp eq i64 %width.i31.i, 5, !dbg !18642
  br i1 %234, label %bb16.i75.i, label %bb39.i55.i.5, !dbg !18642

bb39.i55.i.5:                                     ; preds = %bb21.i72.i.4
  %exitcond4876.5.not = icmp eq i64 %_145.1.i58.i, 5, !dbg !18650
  br i1 %exitcond4876.5.not, label %panic.i60.i, label %bb17.i61.i.5, !dbg !18650

bb17.i61.i.5:                                     ; preds = %bb39.i55.i.5
  %235 = getelementptr inbounds nuw i8, ptr %_145.0.i62.i, i64 68, !dbg !18650
  %_44.i63.i.5 = load i32, ptr %235, align 4, !dbg !18650, !noalias !18629, !noundef !12
  %_43.i64.i.5 = zext i32 %_44.i63.i.5 to i64, !dbg !18650
  %236 = add i64 %ring_cursor.sroa.0.1.i433364, %_43.i64.i.5, !dbg !18720
  %_47.not.i65.i.5 = icmp ult i64 %236, %_92.i, !dbg !18721
  %237 = select i1 %_47.not.i65.i.5, i64 0, i64 %_92.i, !dbg !18721
  %spec.select.i66.i.5 = sub nuw i64 %236, %237, !dbg !18721
  %_51.i67.i.5 = mul i64 %spec.select.i66.i.5, %width.i31.i, !dbg !18723
  %_50.i68.i.5 = add i64 %_51.i67.i.5, 5, !dbg !18723
  %_53.i70.i.5 = icmp ult i64 %_50.i68.i.5, %_149.1.i80.i.pre, !dbg !18724
  br i1 %_53.i70.i.5, label %bb21.i72.i.5, label %panic1.i71.i, !dbg !18724

bb21.i72.i.5:                                     ; preds = %bb17.i61.i.5
  %238 = getelementptr inbounds nuw float, ptr %_147.0.i73.i, i64 %_50.i68.i.5, !dbg !18724
  %_49.i74.i.5 = load float, ptr %238, align 4, !dbg !18724, !noalias !18629, !noundef !12
  store float %_49.i74.i.5, ptr %iter.i30.i.sroa.0.0.ptr3350.5, align 4, !dbg !18725, !noalias !18629
  %239 = icmp eq i64 %width.i31.i, 6, !dbg !18642
  br i1 %239, label %bb16.i75.i, label %bb39.i55.i.6, !dbg !18642

bb39.i55.i.6:                                     ; preds = %bb21.i72.i.5
  %exitcond4876.6.not = icmp eq i64 %_145.1.i58.i, 6, !dbg !18650
  br i1 %exitcond4876.6.not, label %panic.i60.i, label %bb17.i61.i.6, !dbg !18650

bb17.i61.i.6:                                     ; preds = %bb39.i55.i.6
  %240 = getelementptr inbounds nuw i8, ptr %_145.0.i62.i, i64 80, !dbg !18650
  %_44.i63.i.6 = load i32, ptr %240, align 4, !dbg !18650, !noalias !18629, !noundef !12
  %_43.i64.i.6 = zext i32 %_44.i63.i.6 to i64, !dbg !18650
  %241 = add i64 %ring_cursor.sroa.0.1.i433364, %_43.i64.i.6, !dbg !18720
  %_47.not.i65.i.6 = icmp ult i64 %241, %_92.i, !dbg !18721
  %242 = select i1 %_47.not.i65.i.6, i64 0, i64 %_92.i, !dbg !18721
  %spec.select.i66.i.6 = sub nuw i64 %241, %242, !dbg !18721
  %_51.i67.i.6 = mul i64 %spec.select.i66.i.6, %width.i31.i, !dbg !18723
  %_50.i68.i.6 = add i64 %_51.i67.i.6, 6, !dbg !18723
  %_53.i70.i.6 = icmp ult i64 %_50.i68.i.6, %_149.1.i80.i.pre, !dbg !18724
  br i1 %_53.i70.i.6, label %bb21.i72.i.6, label %panic1.i71.i, !dbg !18724

bb21.i72.i.6:                                     ; preds = %bb17.i61.i.6
  %243 = getelementptr inbounds nuw float, ptr %_147.0.i73.i, i64 %_50.i68.i.6, !dbg !18724
  %_49.i74.i.6 = load float, ptr %243, align 4, !dbg !18724, !noalias !18629, !noundef !12
  store float %_49.i74.i.6, ptr %iter.i30.i.sroa.0.0.ptr3350.6, align 4, !dbg !18725, !noalias !18629
  %244 = icmp eq i64 %width.i31.i, 7, !dbg !18642
  br i1 %244, label %bb16.i75.i, label %bb39.i55.i.7, !dbg !18642

bb39.i55.i.7:                                     ; preds = %bb21.i72.i.6
  %exitcond4876.7.not = icmp eq i64 %_145.1.i58.i, 7, !dbg !18650
  br i1 %exitcond4876.7.not, label %panic.i60.i, label %bb17.i61.i.7, !dbg !18650

bb17.i61.i.7:                                     ; preds = %bb39.i55.i.7
  %245 = getelementptr inbounds nuw i8, ptr %_145.0.i62.i, i64 92, !dbg !18650
  %_44.i63.i.7 = load i32, ptr %245, align 4, !dbg !18650, !noalias !18629, !noundef !12
  %_43.i64.i.7 = zext i32 %_44.i63.i.7 to i64, !dbg !18650
  %246 = add i64 %ring_cursor.sroa.0.1.i433364, %_43.i64.i.7, !dbg !18720
  %_47.not.i65.i.7 = icmp ult i64 %246, %_92.i, !dbg !18721
  %247 = select i1 %_47.not.i65.i.7, i64 0, i64 %_92.i, !dbg !18721
  %spec.select.i66.i.7 = sub nuw i64 %246, %247, !dbg !18721
  %_51.i67.i.7 = mul i64 %spec.select.i66.i.7, %width.i31.i, !dbg !18723
  %_50.i68.i.7 = add i64 %_51.i67.i.7, 7, !dbg !18723
  %_53.i70.i.7 = icmp ult i64 %_50.i68.i.7, %_149.1.i80.i.pre, !dbg !18724
  br i1 %_53.i70.i.7, label %bb21.i72.i.7, label %panic1.i71.i, !dbg !18724

bb21.i72.i.7:                                     ; preds = %bb17.i61.i.7
  %248 = getelementptr inbounds nuw float, ptr %_147.0.i73.i, i64 %_50.i68.i.7, !dbg !18724
  %_49.i74.i.7 = load float, ptr %248, align 4, !dbg !18724, !noalias !18629, !noundef !12
  store float %_49.i74.i.7, ptr %iter.i30.i.sroa.0.0.ptr3350.7, align 4, !dbg !18725, !noalias !18629
  br label %bb16.i75.i, !dbg !18642

panic1.i71.i:                                     ; preds = %bb17.i61.i.7, %bb17.i61.i.6, %bb17.i61.i.5, %bb17.i61.i.4, %bb17.i61.i.3, %bb17.i61.i.2, %bb17.i61.i.1, %bb17.i61.i
  %_50.i68.i.lcssa.ph = phi i64 [ %_50.i68.i.7, %bb17.i61.i.7 ], [ %_50.i68.i.6, %bb17.i61.i.6 ], [ %_50.i68.i.5, %bb17.i61.i.5 ], [ %_50.i68.i.4, %bb17.i61.i.4 ], [ %_50.i68.i.3, %bb17.i61.i.3 ], [ %_50.i68.i.2, %bb17.i61.i.2 ], [ %_50.i68.i.1, %bb17.i61.i.1 ], [ %_51.i67.i, %bb17.i61.i ]
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i68.i.lcssa.ph, i64 noundef %_149.1.i80.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aca25255cb99dc5ba482e692667f5878) #31, !dbg !18724, !noalias !18629
  unreachable, !dbg !18724

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1278: ; preds = %bb49.i102.i
  %_127.i105.i = getelementptr inbounds nuw float, ptr %_150.0.i103.i, i64 %_76.i100.i, !dbg !18726
  %_0.i1199 = load float, ptr %_127.i105.i, align 4, !dbg !18716, !alias.scope !18712, !noalias !18629, !noundef !12
  store float %_0.i1208, ptr %_127.i105.i, align 4, !dbg !18731, !alias.scope !18734, !noalias !18629
  %_0.i1023 = fmul float %_0.i1140, %_0.i1199, !dbg !18737
  %_6.i1449 = bitcast float %_0.i1199 to i32, !dbg !18739
  %_5.i1450 = and i32 %_6.i1449, %all.sroa.0.0.i28, !dbg !18742
  %_8.i1451 = bitcast float %_0.i1023 to i32, !dbg !18743
  %_7.i1453 = and i32 %_9.i1452, %_8.i1451, !dbg !18745
  %_4.i1454 = or disjoint i32 %_7.i1453, %_5.i1450, !dbg !18742
  store i32 %_4.i1454, ptr %_151.i, align 4, !dbg !18746, !alias.scope !18748, !noalias !18751
  %_152.i = icmp ugt i64 %_66.i, %right_io.1, !dbg !18752
  br i1 %_152.i, label %bb63.i, label %bb64.i, !dbg !18752, !prof !905

bb61.i:                                           ; preds = %bb35.i
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_66.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f473a90cd9861be1576d1010d795a4d4) #31, !dbg !18756, !noalias !18387
  unreachable, !dbg !18756

bb64.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1278
  %_159.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %_66.i, !dbg !18757
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18762), !dbg !18765
  %_3.not.i1192 = icmp eq i64 %right_io.1, %_66.i, !dbg !18766
  br i1 %_3.not.i1192, label %panic.i1195, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1196, !dbg !18766

panic.i1195:                                      ; preds = %bb64.i
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #31, !dbg !18766, !noalias !18768
  unreachable, !dbg !18766

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1196: ; preds = %bb64.i
  %_0.i1194 = load float, ptr %_159.i, align 4, !dbg !18766, !alias.scope !18762, !noalias !18387, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18769), !dbg !18772
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18773), !dbg !18772
  %width.i.i = load i64, ptr %152, align 8, !dbg !18775, !alias.scope !18777, !noalias !18778, !noundef !12
  %_3.i688 = fcmp uge float %_0.i14223522, %_0.i1468, !dbg !18781
  %_0.i909 = fdiv float %_0.i14223522, %_0.i1468, !dbg !18783
  %_0.i1448 = select i1 %_3.i688, float 1.000000e+00, float %_0.i909, !dbg !18785
  %_144.1.i.i = load i64, ptr %153, align 8, !dbg !18787, !alias.scope !18777, !noalias !18778, !noundef !12
  %_22.i.i58 = mul i64 %width.i.i, %ring_cursor.sroa.0.1.i433364, !dbg !18788
  %_92.i.i = icmp ugt i64 %_22.i.i58, %_144.1.i.i, !dbg !18789
  br i1 %_92.i.i, label %bb37.i.i, label %bb38.i.i, !dbg !18789, !prof !905

bb38.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1196
  %_144.0.i.i = load ptr, ptr %154, align 8, !dbg !18787, !alias.scope !18777, !noalias !18778, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18792), !dbg !18795
  %_4.not.i1271 = icmp eq i64 %_144.1.i.i, %_22.i.i58, !dbg !18796
  br i1 %_4.not.i1271, label %panic.i1273, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1274, !dbg !18796

panic.i1273:                                      ; preds = %bb38.i.i
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #31, !dbg !18796, !noalias !18798
  unreachable, !dbg !18796

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1274: ; preds = %bb38.i.i
  %_99.i.i = getelementptr inbounds nuw float, ptr %_144.0.i.i, i64 %_22.i.i58, !dbg !18799
  store float %_0.i1448, ptr %_99.i.i, align 4, !dbg !18796, !alias.scope !18792, !noalias !18801
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18802), !dbg !18805
  %width.i = load i64, ptr %152, align 8, !dbg !18806, !alias.scope !18802, !noalias !18808, !noundef !12
  %249 = icmp eq i64 %width.i, 0, !dbg !18810
  br i1 %249, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit, label %bb32.i425.lr.ph, !dbg !18810

bb32.i425.lr.ph:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1274
  %_112.1.i = load i64, ptr %155, align 8, !alias.scope !18802, !noalias !18808, !noundef !12
  %_112.0.i = load ptr, ptr %156, align 8, !nonnull !12
  %250 = add i64 %ring_cursor.sroa.0.1.i433364, 1
  %_23.not.i = icmp ult i64 %250, %_92.i
  %251 = select i1 %_23.not.i, i64 0, i64 %_92.i
  %start1.sroa.0.0.i = sub nuw i64 %250, %251
  %_114.1.i = load i64, ptr %153, align 8
  %_114.0.i = load ptr, ptr %154, align 8, !nonnull !12
  %_116.1.i = load i64, ptr %157, align 8
  %_116.0.i = load ptr, ptr %158, align 8, !nonnull !12
  %_118.1.i = load i64, ptr %159, align 8
  %_118.0.i = load ptr, ptr %160, align 8, !nonnull !12
  %_45.i = mul i64 %width.i, %start1.sroa.0.0.i
  br label %bb32.i425, !dbg !18810

bb32.i425:                                        ; preds = %bb32.i425.lr.ph, %bb31.i442
  %iter.i423.sroa.10.03356 = phi i64 [ %width.i, %bb32.i425.lr.ph ], [ %252, %bb31.i442 ]
  %iter.i423.sroa.7.03355 = phi i64 [ 0, %bb32.i425.lr.ph ], [ %_9.0.i1903, %bb31.i442 ]
  %iter.i423.sroa.0.0.idx3354 = phi i64 [ 0, %bb32.i425.lr.ph ], [ %iter.i423.sroa.0.0.add, %bb31.i442 ]
  %iter.i423.sroa.0.0.ptr3357 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 %iter.i423.sroa.0.0.idx3354, !dbg !18812
  %252 = add i64 %iter.i423.sroa.10.03356, -1, !dbg !18812
  %_7.i.i1899 = icmp eq i64 %iter.i423.sroa.0.0.idx3354, 32, !dbg !18813
  br i1 %_7.i.i1899, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit, label %bb3.i427, !dbg !18817

bb3.i427:                                         ; preds = %bb32.i425
  %iter.i423.sroa.0.0.add = add nuw nsw i64 %iter.i423.sroa.0.0.idx3354, 4, !dbg !18818
  %_9.0.i1903 = add nuw nsw i64 %iter.i423.sroa.7.03355, 1, !dbg !18820
  %exitcond4879.not = icmp eq i64 %iter.i423.sroa.7.03355, %_112.1.i, !dbg !18821
  br i1 %exitcond4879.not, label %panic.i, label %bb5.i428, !dbg !18821

bb5.i428:                                         ; preds = %bb3.i427
  %253 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i, i64 %iter.i423.sroa.7.03355, !dbg !18821
  %shape.i = load i32, ptr %253, align 4, !dbg !18821, !noalias !18822, !noundef !12
  %254 = getelementptr inbounds nuw i8, ptr %253, i64 4, !dbg !18821
  %shape3.i = load i32, ptr %254, align 4, !dbg !18821, !noalias !18822, !noundef !12
  %window.i = zext i32 %shape.i to i64, !dbg !18823
  %_19.i = zext i32 %shape3.i to i64, !dbg !18824
  %255 = add i64 %ring_cursor.sroa.0.1.i433364, %_19.i, !dbg !18825
  %_20.not.i = icmp ult i64 %255, %_92.i, !dbg !18826
  %256 = select i1 %_20.not.i, i64 0, i64 %_92.i, !dbg !18826
  %spec.select.i = sub nuw i64 %255, %256, !dbg !18826
  %_27.i = mul i64 %spec.select.i, %width.i, !dbg !18827
  %_26.i429 = add i64 %_27.i, %iter.i423.sroa.7.03355, !dbg !18827
  %_30.i430 = icmp ult i64 %_26.i429, %_114.1.i, !dbg !18828
  br i1 %_30.i430, label %bb12.i, label %panic5.i, !dbg !18828

panic.i:                                          ; preds = %bb3.i427
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i, i64 noundef %_112.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_077370d5cece7380867993336836eb69) #31, !dbg !18821, !noalias !18822
  unreachable, !dbg !18821

bb12.i:                                           ; preds = %bb5.i428
  %257 = getelementptr inbounds nuw float, ptr %_114.0.i, i64 %_26.i429, !dbg !18828
  %258 = load float, ptr %257, align 4, !dbg !18828, !noalias !18822, !noundef !12
  %exitcond4880.not = icmp eq i64 %iter.i423.sroa.7.03355, %_116.1.i, !dbg !18829
  br i1 %exitcond4880.not, label %panic6.i, label %bb13.i, !dbg !18829

panic5.i:                                         ; preds = %bb5.i428
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i429, i64 noundef %_114.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5cae9ea88ed6362f62dbad10291edbd4) #31, !dbg !18828, !noalias !18822
  unreachable, !dbg !18828

bb13.i:                                           ; preds = %bb12.i
  %259 = getelementptr inbounds nuw i32, ptr %_116.0.i, i64 %iter.i423.sroa.7.03355, !dbg !18829
  %_32.i = load i32, ptr %259, align 4, !dbg !18829, !noalias !18822, !noundef !12
  %position.i432 = zext i32 %_32.i to i64, !dbg !18829
  %260 = icmp eq i32 %_32.i, 0, !dbg !18830
  br i1 %260, label %bb17.i, label %bb15.i, !dbg !18830

panic6.i:                                         ; preds = %bb12.i
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i, i64 noundef %_116.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9fdd00ed7abe4ccfd73f0d145a7b741b) #31, !dbg !18829, !noalias !18822
  unreachable, !dbg !18829

bb15.i:                                           ; preds = %bb13.i
  %_37.i = icmp ult i64 %iter.i423.sroa.7.03355, %_118.1.i, !dbg !18831
  br i1 %_37.i, label %bb16.i, label %panic7.i, !dbg !18831

bb17.i:                                           ; preds = %bb35.i454, %bb16.i, %bb13.i
  %newest.sroa.0.0.i = phi float [ %258, %bb13.i ], [ %_35.i433, %bb35.i454 ], [ %258, %bb16.i ], !dbg !18832
  %exitcond4881.not = icmp eq i64 %iter.i423.sroa.7.03355, %_118.1.i, !dbg !18833
  br i1 %exitcond4881.not, label %panic8.i, label %bb18.i, !dbg !18833

bb16.i:                                           ; preds = %bb15.i
  %261 = getelementptr inbounds nuw float, ptr %_118.0.i, i64 %iter.i423.sroa.7.03355, !dbg !18831
  %_35.i433 = load float, ptr %261, align 4, !dbg !18831, !noalias !18822, !noundef !12
  %_102.i434 = fcmp olt float %_35.i433, %258, !dbg !18834
  br i1 %_102.i434, label %bb35.i454, label %bb17.i, !dbg !18834

panic7.i:                                         ; preds = %bb15.i
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i423.sroa.7.03355, i64 noundef %_118.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_697dc8945f5e5b040d7e9a0b92cb249f) #31, !dbg !18831, !noalias !18822
  unreachable, !dbg !18831

bb35.i454:                                        ; preds = %bb16.i
  br label %bb17.i, !dbg !18836

bb18.i:                                           ; preds = %bb17.i
  %262 = getelementptr inbounds nuw float, ptr %_118.0.i, i64 %iter.i423.sroa.7.03355, !dbg !18833
  store float %newest.sroa.0.0.i, ptr %262, align 4, !dbg !18833, !noalias !18822
  %_42.i435 = add nuw nsw i64 %position.i432, 1, !dbg !18837
  %complete.i436 = icmp eq i64 %_42.i435, %window.i, !dbg !18837
  br i1 %complete.i436, label %bb22.i, label %bb20.i, !dbg !18838

panic8.i:                                         ; preds = %bb17.i
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i, i64 noundef %_118.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_067dce244605df4a33956fbd4d726027) #31, !dbg !18833, !noalias !18822
  unreachable, !dbg !18833

bb20.i:                                           ; preds = %bb18.i
  %_44.i = add i64 %iter.i423.sroa.7.03355, %_45.i, !dbg !18839
  %_47.i437 = icmp ult i64 %_44.i, %_114.1.i, !dbg !18840
  br i1 %_47.i437, label %bb30.i441, label %panic9.i, !dbg !18840

panic9.i:                                         ; preds = %bb20.i
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i, i64 noundef %_114.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d5ea38731a1cb6311efef2f919234d06) #31, !dbg !18840, !noalias !18822
  unreachable, !dbg !18840

bb30.i441:                                        ; preds = %bb20.i
  %263 = getelementptr inbounds nuw float, ptr %_114.0.i, i64 %_44.i, !dbg !18840
  %_43.i438 = load float, ptr %263, align 4, !dbg !18840, !noalias !18822, !noundef !12
  %_103.i439 = fcmp olt float %_43.i438, %newest.sroa.0.0.i, !dbg !18841
  %newest.sroa.0.1.i = select i1 %_103.i439, float %_43.i438, float %newest.sroa.0.0.i, !dbg !18841
  store float %newest.sroa.0.1.i, ptr %iter.i423.sroa.0.0.ptr3357, align 4, !dbg !18843, !noalias !18822
  %264 = trunc i64 %_42.i435 to i32, !dbg !18844
  br label %bb31.i442, !dbg !18845

bb31.i442:                                        ; preds = %bb25.i452, %bb30.i441
  %storemerge2292 = phi i32 [ %264, %bb30.i441 ], [ 0, %bb25.i452 ], !dbg !18846
  store i32 %storemerge2292, ptr %259, align 4, !dbg !18846, !noalias !18822
  %265 = icmp eq i64 %252, 0, !dbg !18810
  br i1 %265, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit, label %bb32.i425, !dbg !18810

bb22.i:                                           ; preds = %bb18.i
  store float %newest.sroa.0.0.i, ptr %iter.i423.sroa.0.0.ptr3357, align 4, !dbg !18843, !noalias !18822
  %266 = load float, ptr %257, align 4, !dbg !18847, !noalias !18822, !noundef !12
  br label %bb41.i448, !dbg !18848

bb41.i448:                                        ; preds = %bb22.i, %bb25.i452
  %iter2.sroa.0.0.i4463353 = phi i64 [ 0, %bb22.i ], [ %_105.i449, %bb25.i452 ]
  %suffix.sroa.0.0.i4453352 = phi float [ %266, %bb22.i ], [ %suffix.sroa.0.1.i, %bb25.i452 ]
  %end.sroa.0.1.i3351 = phi i64 [ %spec.select.i, %bb22.i ], [ %269, %bb25.i452 ]
  %_56.i = mul i64 %end.sroa.0.1.i3351, %width.i, !dbg !18851
  %_55.i = add i64 %_56.i, %iter.i423.sroa.7.03355, !dbg !18851
  %_59.i = icmp ult i64 %_55.i, %_114.1.i, !dbg !18852
  br i1 %_59.i, label %bb25.i452, label %panic13.i, !dbg !18852

panic13.i:                                        ; preds = %bb41.i448
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i, i64 noundef %_114.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a19fa7d9077791c9eadbe74fafea8de7) #31, !dbg !18852, !noalias !18822
  unreachable, !dbg !18852

bb25.i452:                                        ; preds = %bb41.i448
  %_105.i449 = add nuw nsw i64 %iter2.sroa.0.0.i4463353, 1, !dbg !18853
  %267 = getelementptr inbounds nuw float, ptr %_114.0.i, i64 %_55.i, !dbg !18852
  %_54.i = load float, ptr %267, align 4, !dbg !18852, !noalias !18822, !noundef !12
  %_107.i450 = fcmp olt float %suffix.sroa.0.0.i4453352, %_54.i, !dbg !18856
  %suffix.sroa.0.1.i = select i1 %_107.i450, float %suffix.sroa.0.0.i4453352, float %_54.i, !dbg !18856
  store float %suffix.sroa.0.1.i, ptr %267, align 4, !dbg !18858, !noalias !18822
  %268 = icmp eq i64 %end.sroa.0.1.i3351, 0, !dbg !18859
  %spec.store.select.i453 = select i1 %268, i64 %_92.i, i64 %end.sroa.0.1.i3351, !dbg !18859
  %269 = add i64 %spec.store.select.i453, -1, !dbg !18860
  %exitcond4878.not = icmp eq i64 %_105.i449, %window.i, !dbg !18861
  br i1 %exitcond4878.not, label %bb31.i442, label %bb41.i448, !dbg !18848

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit: ; preds = %bb32.i425, %bb31.i442
  %_0.i1191.pre = load float, ptr %scratch.i, align 4, !dbg !18863, !alias.scope !18865, !noalias !18868
  br label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit, !dbg !18863

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit: ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1274
  %_0.i1191 = phi float [ %_0.i1191.pre, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit ], [ %_0.i1203, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1274 ], !dbg !18863
  %_0.i1022 = fmul float %_0.i1191, 1.638400e+04, !dbg !18869
  %270 = tail call noundef float @llvm.floor.f32(float %_0.i1022), !dbg !18871
  %_0.i1021 = fmul float %270, 0x3F10000000000000, !dbg !18875
  %271 = icmp eq i64 %width.i.i, 0, !dbg !18877
  %_149.1.i.i.pre = load i64, ptr %161, align 8, !dbg !18879, !alias.scope !18777, !noalias !18778
  br i1 %271, label %bb16.i.i, label %bb39.i.i.lr.ph, !dbg !18877

bb39.i.i.lr.ph:                                   ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit
  %_145.1.i.i = load i64, ptr %155, align 8, !alias.scope !18777, !noalias !18778, !noundef !12
  %_145.0.i.i = load ptr, ptr %156, align 8, !nonnull !12
  %_147.0.i.i = load ptr, ptr %162, align 8, !nonnull !12
  %exitcond4882.not = icmp eq i64 %_145.1.i.i, 0, !dbg !18880
  br i1 %exitcond4882.not, label %panic.i.i, label %bb17.i.i, !dbg !18880

bb37.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1196
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i58, i64 noundef %_144.1.i.i, i64 noundef %_144.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9d2713d1692431af37d60082290049ed) #31, !dbg !18881, !noalias !18801
  unreachable, !dbg !18881

bb16.i.i:                                         ; preds = %bb21.i.i.7, %bb21.i.i, %bb21.i.i.1, %bb21.i.i.2, %bb21.i.i.3, %bb21.i.i.4, %bb21.i.i.5, %bb21.i.i.6, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit
  %_0.i1189 = phi float [ %_0.i1191, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit ], [ %_49.i.i66, %bb21.i.i ], [ %_49.i.i66, %bb21.i.i.7 ], [ %_49.i.i66, %bb21.i.i.6 ], [ %_49.i.i66, %bb21.i.i.5 ], [ %_49.i.i66, %bb21.i.i.4 ], [ %_49.i.i66, %bb21.i.i.3 ], [ %_49.i.i66, %bb21.i.i.2 ], [ %_49.i.i66, %bb21.i.i.1 ], !dbg !18882
  %_0.i808 = fadd float %_0.i1021, %_0.i11393540, !dbg !18884
  %_0.i1139 = fsub float %_0.i808, %_0.i1189, !dbg !18886
  store float %_0.i1139, ptr %163, align 4, !dbg !18888, !alias.scope !18769, !noalias !18889
  %_109.i.i = icmp ugt i64 %_22.i.i58, %_149.1.i.i.pre, !dbg !18890
  br i1 %_109.i.i, label %bb42.i.i, label %bb43.i.i, !dbg !18890, !prof !905

bb43.i.i:                                         ; preds = %bb16.i.i
  %_149.0.i.i = load ptr, ptr %162, align 8, !dbg !18879, !alias.scope !18777, !noalias !18778, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18893), !dbg !18896
  %_4.not.i1267 = icmp eq i64 %_149.1.i.i.pre, %_22.i.i58, !dbg !18897
  br i1 %_4.not.i1267, label %panic.i1269, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1270, !dbg !18897

panic.i1269:                                      ; preds = %bb43.i.i
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #31, !dbg !18897, !noalias !18899
  unreachable, !dbg !18897

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1270: ; preds = %bb43.i.i
  %_116.i.i = getelementptr inbounds nuw float, ptr %_149.0.i.i, i64 %_22.i.i58, !dbg !18900
  store float %_0.i1021, ptr %_116.i.i, align 4, !dbg !18897, !alias.scope !18893, !noalias !18868
  %_0.i908 = fdiv float %_0.i1139, %_64.i.i, !dbg !18902
  %_0.i1138 = fsub float 1.000000e+00, %_0.i908, !dbg !18904
  %_0.i1137 = fsub float %_0.i1138, %_0.i13233542, !dbg !18906
  %_4.i916 = fmul float %_0.i14353530, %_0.i1137, !dbg !18908
  %_0.i917 = fadd float %_0.i13233542, %_4.i916, !dbg !18908
  %_3.i.i1582.inv = fcmp ogt float %_0.i1138, %_0.i917, !dbg !18910
  %_4.i.i1589.v = select i1 %_3.i.i1582.inv, float %_0.i1138, float %_0.i917, !dbg !18910
  %272 = tail call noundef float @llvm.fabs.f32(float %_4.i.i1589.v), !dbg !18913
  %273 = fcmp uge float %272, 0x3BC79CA100000000, !dbg !18916
  %_0.i1323 = select i1 %273, float %_4.i.i1589.v, float 0.000000e+00, !dbg !18918
  store float %_0.i1323, ptr %165, align 4, !dbg !18919, !alias.scope !18769, !noalias !18889
  %_0.i1136 = fsub float 1.000000e+00, %_0.i1323, !dbg !18920
  %_150.1.i.i = load i64, ptr %166, align 8, !dbg !18922, !alias.scope !18777, !noalias !18778, !noundef !12
  %_76.i.i = mul i64 %width.i.i, %main_cursor.sroa.0.1.i443365, !dbg !18923
  %_120.i.i = icmp ugt i64 %_76.i.i, %_150.1.i.i, !dbg !18924
  br i1 %_120.i.i, label %bb48.i.i, label %bb49.i.i, !dbg !18924, !prof !905

bb42.i.i:                                         ; preds = %bb16.i.i
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i58, i64 noundef %_149.1.i.i.pre, i64 noundef %_149.1.i.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8002ed69501742f3ea2ea25eb68cd581) #31, !dbg !18927, !noalias !18868
  unreachable, !dbg !18927

bb49.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1270
  %_150.0.i.i = load ptr, ptr %167, align 8, !dbg !18922, !alias.scope !18777, !noalias !18778, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18928), !dbg !18931
  %_3.not.i1183 = icmp eq i64 %_150.1.i.i, %_76.i.i, !dbg !18932
  br i1 %_3.not.i1183, label %panic.i1186, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262, !dbg !18932

panic.i1186:                                      ; preds = %bb49.i.i
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #31, !dbg !18932, !noalias !18934
  unreachable, !dbg !18932

bb48.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1270
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i.i, i64 noundef %_150.1.i.i, i64 noundef %_150.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a07d19b424a92543209d516ece5bdf2a) #31, !dbg !18935, !noalias !18868
  unreachable, !dbg !18935

bb17.i.i:                                         ; preds = %bb39.i.i.lr.ph
  %274 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 8, !dbg !18880
  %_44.i.i63 = load i32, ptr %274, align 4, !dbg !18880, !noalias !18868, !noundef !12
  %_43.i.i64 = zext i32 %_44.i.i63 to i64, !dbg !18880
  %275 = add i64 %ring_cursor.sroa.0.1.i433364, %_43.i.i64, !dbg !18936
  %_47.not.i.i = icmp ult i64 %275, %_92.i, !dbg !18937
  %276 = select i1 %_47.not.i.i, i64 0, i64 %_92.i, !dbg !18937
  %spec.select.i.i = sub nuw i64 %275, %276, !dbg !18937
  %_51.i.i = mul i64 %spec.select.i.i, %width.i.i, !dbg !18938
  %_53.i.i65 = icmp ult i64 %_51.i.i, %_149.1.i.i.pre, !dbg !18939
  br i1 %_53.i.i65, label %bb21.i.i, label %panic1.i.i, !dbg !18939

panic.i.i:                                        ; preds = %bb39.i.i.7, %bb39.i.i.6, %bb39.i.i.5, %bb39.i.i.4, %bb39.i.i.3, %bb39.i.i.2, %bb39.i.i.1, %bb39.i.i.lr.ph
  %_145.1.i.i.lcssa.ph = phi i64 [ 7, %bb39.i.i.7 ], [ 6, %bb39.i.i.6 ], [ 5, %bb39.i.i.5 ], [ 4, %bb39.i.i.4 ], [ 3, %bb39.i.i.3 ], [ 2, %bb39.i.i.2 ], [ 1, %bb39.i.i.1 ], [ 0, %bb39.i.i.lr.ph ]
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i.i.lcssa.ph, i64 noundef %_145.1.i.i.lcssa.ph, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_891dd683d7367e0a1ec5a22f9c2a2aa8) #31, !dbg !18880, !noalias !18868
  unreachable, !dbg !18880

bb21.i.i:                                         ; preds = %bb17.i.i
  %277 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_51.i.i, !dbg !18939
  %_49.i.i66 = load float, ptr %277, align 4, !dbg !18939, !noalias !18868, !noundef !12
  store float %_49.i.i66, ptr %scratch.i, align 4, !dbg !18940, !noalias !18868
  %278 = icmp eq i64 %width.i.i, 1, !dbg !18877
  br i1 %278, label %bb16.i.i, label %bb39.i.i.1, !dbg !18877

bb39.i.i.1:                                       ; preds = %bb21.i.i
  %exitcond4882.1.not = icmp eq i64 %_145.1.i.i, 1, !dbg !18880
  br i1 %exitcond4882.1.not, label %panic.i.i, label %bb17.i.i.1, !dbg !18880

bb17.i.i.1:                                       ; preds = %bb39.i.i.1
  %279 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 20, !dbg !18880
  %_44.i.i63.1 = load i32, ptr %279, align 4, !dbg !18880, !noalias !18868, !noundef !12
  %_43.i.i64.1 = zext i32 %_44.i.i63.1 to i64, !dbg !18880
  %280 = add i64 %ring_cursor.sroa.0.1.i433364, %_43.i.i64.1, !dbg !18936
  %_47.not.i.i.1 = icmp ult i64 %280, %_92.i, !dbg !18937
  %281 = select i1 %_47.not.i.i.1, i64 0, i64 %_92.i, !dbg !18937
  %spec.select.i.i.1 = sub nuw i64 %280, %281, !dbg !18937
  %_51.i.i.1 = mul i64 %spec.select.i.i.1, %width.i.i, !dbg !18938
  %_50.i.i.1 = add i64 %_51.i.i.1, 1, !dbg !18938
  %_53.i.i65.1 = icmp ult i64 %_50.i.i.1, %_149.1.i.i.pre, !dbg !18939
  br i1 %_53.i.i65.1, label %bb21.i.i.1, label %panic1.i.i, !dbg !18939

bb21.i.i.1:                                       ; preds = %bb17.i.i.1
  %282 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.1, !dbg !18939
  %_49.i.i66.1 = load float, ptr %282, align 4, !dbg !18939, !noalias !18868, !noundef !12
  store float %_49.i.i66.1, ptr %iter.i.i.sroa.0.0.ptr3361.1, align 4, !dbg !18940, !noalias !18868
  %283 = icmp eq i64 %width.i.i, 2, !dbg !18877
  br i1 %283, label %bb16.i.i, label %bb39.i.i.2, !dbg !18877

bb39.i.i.2:                                       ; preds = %bb21.i.i.1
  %exitcond4882.2.not = icmp eq i64 %_145.1.i.i, 2, !dbg !18880
  br i1 %exitcond4882.2.not, label %panic.i.i, label %bb17.i.i.2, !dbg !18880

bb17.i.i.2:                                       ; preds = %bb39.i.i.2
  %284 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 32, !dbg !18880
  %_44.i.i63.2 = load i32, ptr %284, align 4, !dbg !18880, !noalias !18868, !noundef !12
  %_43.i.i64.2 = zext i32 %_44.i.i63.2 to i64, !dbg !18880
  %285 = add i64 %ring_cursor.sroa.0.1.i433364, %_43.i.i64.2, !dbg !18936
  %_47.not.i.i.2 = icmp ult i64 %285, %_92.i, !dbg !18937
  %286 = select i1 %_47.not.i.i.2, i64 0, i64 %_92.i, !dbg !18937
  %spec.select.i.i.2 = sub nuw i64 %285, %286, !dbg !18937
  %_51.i.i.2 = mul i64 %spec.select.i.i.2, %width.i.i, !dbg !18938
  %_50.i.i.2 = add i64 %_51.i.i.2, 2, !dbg !18938
  %_53.i.i65.2 = icmp ult i64 %_50.i.i.2, %_149.1.i.i.pre, !dbg !18939
  br i1 %_53.i.i65.2, label %bb21.i.i.2, label %panic1.i.i, !dbg !18939

bb21.i.i.2:                                       ; preds = %bb17.i.i.2
  %287 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.2, !dbg !18939
  %_49.i.i66.2 = load float, ptr %287, align 4, !dbg !18939, !noalias !18868, !noundef !12
  store float %_49.i.i66.2, ptr %iter.i.i.sroa.0.0.ptr3361.2, align 4, !dbg !18940, !noalias !18868
  %288 = icmp eq i64 %width.i.i, 3, !dbg !18877
  br i1 %288, label %bb16.i.i, label %bb39.i.i.3, !dbg !18877

bb39.i.i.3:                                       ; preds = %bb21.i.i.2
  %exitcond4882.3.not = icmp eq i64 %_145.1.i.i, 3, !dbg !18880
  br i1 %exitcond4882.3.not, label %panic.i.i, label %bb17.i.i.3, !dbg !18880

bb17.i.i.3:                                       ; preds = %bb39.i.i.3
  %289 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 44, !dbg !18880
  %_44.i.i63.3 = load i32, ptr %289, align 4, !dbg !18880, !noalias !18868, !noundef !12
  %_43.i.i64.3 = zext i32 %_44.i.i63.3 to i64, !dbg !18880
  %290 = add i64 %ring_cursor.sroa.0.1.i433364, %_43.i.i64.3, !dbg !18936
  %_47.not.i.i.3 = icmp ult i64 %290, %_92.i, !dbg !18937
  %291 = select i1 %_47.not.i.i.3, i64 0, i64 %_92.i, !dbg !18937
  %spec.select.i.i.3 = sub nuw i64 %290, %291, !dbg !18937
  %_51.i.i.3 = mul i64 %spec.select.i.i.3, %width.i.i, !dbg !18938
  %_50.i.i.3 = add i64 %_51.i.i.3, 3, !dbg !18938
  %_53.i.i65.3 = icmp ult i64 %_50.i.i.3, %_149.1.i.i.pre, !dbg !18939
  br i1 %_53.i.i65.3, label %bb21.i.i.3, label %panic1.i.i, !dbg !18939

bb21.i.i.3:                                       ; preds = %bb17.i.i.3
  %292 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.3, !dbg !18939
  %_49.i.i66.3 = load float, ptr %292, align 4, !dbg !18939, !noalias !18868, !noundef !12
  store float %_49.i.i66.3, ptr %iter.i.i.sroa.0.0.ptr3361.3, align 4, !dbg !18940, !noalias !18868
  %293 = icmp eq i64 %width.i.i, 4, !dbg !18877
  br i1 %293, label %bb16.i.i, label %bb39.i.i.4, !dbg !18877

bb39.i.i.4:                                       ; preds = %bb21.i.i.3
  %exitcond4882.4.not = icmp eq i64 %_145.1.i.i, 4, !dbg !18880
  br i1 %exitcond4882.4.not, label %panic.i.i, label %bb17.i.i.4, !dbg !18880

bb17.i.i.4:                                       ; preds = %bb39.i.i.4
  %294 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 56, !dbg !18880
  %_44.i.i63.4 = load i32, ptr %294, align 4, !dbg !18880, !noalias !18868, !noundef !12
  %_43.i.i64.4 = zext i32 %_44.i.i63.4 to i64, !dbg !18880
  %295 = add i64 %ring_cursor.sroa.0.1.i433364, %_43.i.i64.4, !dbg !18936
  %_47.not.i.i.4 = icmp ult i64 %295, %_92.i, !dbg !18937
  %296 = select i1 %_47.not.i.i.4, i64 0, i64 %_92.i, !dbg !18937
  %spec.select.i.i.4 = sub nuw i64 %295, %296, !dbg !18937
  %_51.i.i.4 = mul i64 %spec.select.i.i.4, %width.i.i, !dbg !18938
  %_50.i.i.4 = add i64 %_51.i.i.4, 4, !dbg !18938
  %_53.i.i65.4 = icmp ult i64 %_50.i.i.4, %_149.1.i.i.pre, !dbg !18939
  br i1 %_53.i.i65.4, label %bb21.i.i.4, label %panic1.i.i, !dbg !18939

bb21.i.i.4:                                       ; preds = %bb17.i.i.4
  %297 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.4, !dbg !18939
  %_49.i.i66.4 = load float, ptr %297, align 4, !dbg !18939, !noalias !18868, !noundef !12
  store float %_49.i.i66.4, ptr %iter.i.i.sroa.0.0.ptr3361.4, align 4, !dbg !18940, !noalias !18868
  %298 = icmp eq i64 %width.i.i, 5, !dbg !18877
  br i1 %298, label %bb16.i.i, label %bb39.i.i.5, !dbg !18877

bb39.i.i.5:                                       ; preds = %bb21.i.i.4
  %exitcond4882.5.not = icmp eq i64 %_145.1.i.i, 5, !dbg !18880
  br i1 %exitcond4882.5.not, label %panic.i.i, label %bb17.i.i.5, !dbg !18880

bb17.i.i.5:                                       ; preds = %bb39.i.i.5
  %299 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 68, !dbg !18880
  %_44.i.i63.5 = load i32, ptr %299, align 4, !dbg !18880, !noalias !18868, !noundef !12
  %_43.i.i64.5 = zext i32 %_44.i.i63.5 to i64, !dbg !18880
  %300 = add i64 %ring_cursor.sroa.0.1.i433364, %_43.i.i64.5, !dbg !18936
  %_47.not.i.i.5 = icmp ult i64 %300, %_92.i, !dbg !18937
  %301 = select i1 %_47.not.i.i.5, i64 0, i64 %_92.i, !dbg !18937
  %spec.select.i.i.5 = sub nuw i64 %300, %301, !dbg !18937
  %_51.i.i.5 = mul i64 %spec.select.i.i.5, %width.i.i, !dbg !18938
  %_50.i.i.5 = add i64 %_51.i.i.5, 5, !dbg !18938
  %_53.i.i65.5 = icmp ult i64 %_50.i.i.5, %_149.1.i.i.pre, !dbg !18939
  br i1 %_53.i.i65.5, label %bb21.i.i.5, label %panic1.i.i, !dbg !18939

bb21.i.i.5:                                       ; preds = %bb17.i.i.5
  %302 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.5, !dbg !18939
  %_49.i.i66.5 = load float, ptr %302, align 4, !dbg !18939, !noalias !18868, !noundef !12
  store float %_49.i.i66.5, ptr %iter.i.i.sroa.0.0.ptr3361.5, align 4, !dbg !18940, !noalias !18868
  %303 = icmp eq i64 %width.i.i, 6, !dbg !18877
  br i1 %303, label %bb16.i.i, label %bb39.i.i.6, !dbg !18877

bb39.i.i.6:                                       ; preds = %bb21.i.i.5
  %exitcond4882.6.not = icmp eq i64 %_145.1.i.i, 6, !dbg !18880
  br i1 %exitcond4882.6.not, label %panic.i.i, label %bb17.i.i.6, !dbg !18880

bb17.i.i.6:                                       ; preds = %bb39.i.i.6
  %304 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 80, !dbg !18880
  %_44.i.i63.6 = load i32, ptr %304, align 4, !dbg !18880, !noalias !18868, !noundef !12
  %_43.i.i64.6 = zext i32 %_44.i.i63.6 to i64, !dbg !18880
  %305 = add i64 %ring_cursor.sroa.0.1.i433364, %_43.i.i64.6, !dbg !18936
  %_47.not.i.i.6 = icmp ult i64 %305, %_92.i, !dbg !18937
  %306 = select i1 %_47.not.i.i.6, i64 0, i64 %_92.i, !dbg !18937
  %spec.select.i.i.6 = sub nuw i64 %305, %306, !dbg !18937
  %_51.i.i.6 = mul i64 %spec.select.i.i.6, %width.i.i, !dbg !18938
  %_50.i.i.6 = add i64 %_51.i.i.6, 6, !dbg !18938
  %_53.i.i65.6 = icmp ult i64 %_50.i.i.6, %_149.1.i.i.pre, !dbg !18939
  br i1 %_53.i.i65.6, label %bb21.i.i.6, label %panic1.i.i, !dbg !18939

bb21.i.i.6:                                       ; preds = %bb17.i.i.6
  %307 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.6, !dbg !18939
  %_49.i.i66.6 = load float, ptr %307, align 4, !dbg !18939, !noalias !18868, !noundef !12
  store float %_49.i.i66.6, ptr %iter.i.i.sroa.0.0.ptr3361.6, align 4, !dbg !18940, !noalias !18868
  %308 = icmp eq i64 %width.i.i, 7, !dbg !18877
  br i1 %308, label %bb16.i.i, label %bb39.i.i.7, !dbg !18877

bb39.i.i.7:                                       ; preds = %bb21.i.i.6
  %exitcond4882.7.not = icmp eq i64 %_145.1.i.i, 7, !dbg !18880
  br i1 %exitcond4882.7.not, label %panic.i.i, label %bb17.i.i.7, !dbg !18880

bb17.i.i.7:                                       ; preds = %bb39.i.i.7
  %309 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 92, !dbg !18880
  %_44.i.i63.7 = load i32, ptr %309, align 4, !dbg !18880, !noalias !18868, !noundef !12
  %_43.i.i64.7 = zext i32 %_44.i.i63.7 to i64, !dbg !18880
  %310 = add i64 %ring_cursor.sroa.0.1.i433364, %_43.i.i64.7, !dbg !18936
  %_47.not.i.i.7 = icmp ult i64 %310, %_92.i, !dbg !18937
  %311 = select i1 %_47.not.i.i.7, i64 0, i64 %_92.i, !dbg !18937
  %spec.select.i.i.7 = sub nuw i64 %310, %311, !dbg !18937
  %_51.i.i.7 = mul i64 %spec.select.i.i.7, %width.i.i, !dbg !18938
  %_50.i.i.7 = add i64 %_51.i.i.7, 7, !dbg !18938
  %_53.i.i65.7 = icmp ult i64 %_50.i.i.7, %_149.1.i.i.pre, !dbg !18939
  br i1 %_53.i.i65.7, label %bb21.i.i.7, label %panic1.i.i, !dbg !18939

bb21.i.i.7:                                       ; preds = %bb17.i.i.7
  %312 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.7, !dbg !18939
  %_49.i.i66.7 = load float, ptr %312, align 4, !dbg !18939, !noalias !18868, !noundef !12
  store float %_49.i.i66.7, ptr %iter.i.i.sroa.0.0.ptr3361.7, align 4, !dbg !18940, !noalias !18868
  br label %bb16.i.i, !dbg !18877

panic1.i.i:                                       ; preds = %bb17.i.i.7, %bb17.i.i.6, %bb17.i.i.5, %bb17.i.i.4, %bb17.i.i.3, %bb17.i.i.2, %bb17.i.i.1, %bb17.i.i
  %_50.i.i.lcssa.ph = phi i64 [ %_50.i.i.7, %bb17.i.i.7 ], [ %_50.i.i.6, %bb17.i.i.6 ], [ %_50.i.i.5, %bb17.i.i.5 ], [ %_50.i.i.4, %bb17.i.i.4 ], [ %_50.i.i.3, %bb17.i.i.3 ], [ %_50.i.i.2, %bb17.i.i.2 ], [ %_50.i.i.1, %bb17.i.i.1 ], [ %_51.i.i, %bb17.i.i ]
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i.i.lcssa.ph, i64 noundef %_149.1.i.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aca25255cb99dc5ba482e692667f5878) #31, !dbg !18939, !noalias !18868
  unreachable, !dbg !18939

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1262: ; preds = %bb49.i.i
  %_127.i.i = getelementptr inbounds nuw float, ptr %_150.0.i.i, i64 %_76.i.i, !dbg !18941
  %_0.i1185 = load float, ptr %_127.i.i, align 4, !dbg !18932, !alias.scope !18928, !noalias !18868, !noundef !12
  store float %_0.i1194, ptr %_127.i.i, align 4, !dbg !18943, !alias.scope !18945, !noalias !18868
  %_0.i1020 = fmul float %_0.i1136, %_0.i1185, !dbg !18948
  %_6.i1436 = bitcast float %_0.i1185 to i32, !dbg !18950
  %_5.i1437 = and i32 %_6.i1436, %all.sroa.0.0.i28, !dbg !18953
  %_8.i1438 = bitcast float %_0.i1020 to i32, !dbg !18954
  %_7.i1440 = and i32 %_9.i1452, %_8.i1438, !dbg !18956
  %_4.i1441 = or disjoint i32 %_7.i1440, %_5.i1437, !dbg !18953
  store i32 %_4.i1441, ptr %_159.i, align 4, !dbg !18957, !alias.scope !18959, !noalias !18962
  %313 = add i64 %main_cursor.sroa.0.1.i443365, 1, !dbg !18963
  %_105.i73 = icmp eq i64 %313, %_107.i, !dbg !18964
  %spec.store.select.i = select i1 %_105.i73, i64 0, i64 %313, !dbg !18964
  %314 = add i64 %ring_cursor.sroa.0.1.i433364, 1, !dbg !18965
  %_108.i = icmp eq i64 %314, %_92.i, !dbg !18966
  %spec.store.select13.i = select i1 %_108.i, i64 0, i64 %314, !dbg !18966
  %exitcond4885.not = icmp eq i64 %184, %umax4884, !dbg !18967
  br i1 %exitcond4885.not, label %bb28.i.bb25.i.loopexit_crit_edge, label %bb57.i, !dbg !17795

bb63.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1278
  store float %_0.i13965730, ptr %_69.i, align 1, !dbg !17781
  store float %_0.i14225767, ptr %_73.i45, align 1, !dbg !17791
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_66.i, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_59d870179e725b921b323beec09137a0) #31, !dbg !18970, !noalias !18387
  unreachable, !dbg !18970

_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_lanefEB2_.exit.loopexit: ; preds = %bb25.i.loopexit
  %315 = trunc i64 %main_cursor.sroa.0.1.i44.lcssa to i32, !dbg !18971
  %316 = trunc i64 %ring_cursor.sroa.0.1.i43.lcssa to i32, !dbg !18972
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_lanefEB2_.exit, !dbg !18973

_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_lanefEB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_lanefEB2_.exit.loopexit, %bb14.i20
  %ring_cursor.sroa.0.0.i35.lcssa = phi i32 [ %_43.i29, %bb14.i20 ], [ %316, %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_lanefEB2_.exit.loopexit ], !dbg !17754
  %main_cursor.sroa.0.0.i36.lcssa = phi i32 [ %_41.i, %bb14.i20 ], [ %315, %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_lanefEB2_.exit.loopexit ], !dbg !17751
  call void @llvm.lifetime.start.p0(ptr nonnull %_111.i), !dbg !18973, !noalias !17630
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(92) %_111.i, ptr noundef nonnull align 4 dereferenceable(92) %hot_left.i18, i64 92, i1 false), !dbg !18973, !noalias !17630
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_111.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33) #30, !dbg !18974, !noalias !18387
  call void @llvm.lifetime.end.p0(ptr nonnull %_111.i), !dbg !18975, !noalias !17630
  call void @llvm.lifetime.start.p0(ptr nonnull %_113.i), !dbg !18976, !noalias !17630
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(92) %_113.i, ptr noundef nonnull align 4 dereferenceable(92) %hot_right.i17, i64 92, i1 false), !dbg !18976, !noalias !17630
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_113.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34) #30, !dbg !18977, !noalias !18387
  call void @llvm.lifetime.end.p0(ptr nonnull %_113.i), !dbg !18978, !noalias !17630
  store i32 %main_cursor.sroa.0.0.i36.lcssa, ptr %_35, align 4, !dbg !18971, !alias.scope !17624, !noalias !17753
  store i32 %ring_cursor.sroa.0.0.i35.lcssa, ptr %85, align 4, !dbg !18972, !alias.scope !17624, !noalias !17753
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i15), !dbg !18979, !noalias !17630
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i16), !dbg !18980, !noalias !17630
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i), !dbg !18981, !noalias !17630
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_right.i17), !dbg !18982, !noalias !17630
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i18), !dbg !18983, !noalias !17630
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockfEB2_.exit, !dbg !17619

bb4.i:                                            ; preds = %bb1.i3.i1811, %bb2.i1805
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18984), !dbg !18987
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18988), !dbg !18987
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18990), !dbg !18987
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18992), !dbg !18987
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18994), !dbg !18987
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_left.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #30, !dbg !18996
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_right.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #30, !dbg !19000
  %317 = getelementptr inbounds nuw i8, ptr %self, i64 264, !dbg !19002
  %_240.0.i = load ptr, ptr %317, align 8, !dbg !19002, !alias.scope !18990, !noalias !19004, !nonnull !12, !noundef !12
  %318 = getelementptr inbounds nuw i8, ptr %self, i64 272, !dbg !19002
  %_240.1.i = load i64, ptr %318, align 8, !dbg !19002, !alias.scope !18990, !noalias !19004, !noundef !12
  %_8.i1919 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_240.0.i, i64 %_240.1.i, !dbg !19007
  br label %bb1.i.i1920, !dbg !19012

bb1.i.i1920:                                      ; preds = %bb13.i.i1923, %bb4.i
  %_221.i.i1921 = phi ptr [ %_22.i.i1924, %bb13.i.i1923 ], [ %_240.0.i, %bb4.i ]
  %_12.i.i1922 = icmp eq ptr %_221.i.i1921, %_8.i1919, !dbg !19014
  br i1 %_12.i.i1922, label %bb4.i5, label %bb13.i.i1923, !dbg !19017

bb13.i.i1923:                                     ; preds = %bb1.i.i1920
  %_22.i.i1924 = getelementptr inbounds nuw i8, ptr %_221.i.i1921, i64 16, !dbg !19018
  %319 = getelementptr inbounds nuw i8, ptr %_221.i.i1921, i64 12, !dbg !19020
  %_3.i.i.i1925 = load i32, ptr %319, align 4, !dbg !19020, !alias.scope !19022, !noalias !19027, !noundef !12
  %320 = icmp eq i32 %_3.i.i.i1925, 0, !dbg !19020
  %_51.i.i.i1926 = load i32, ptr %_221.i.i1921, align 4, !dbg !19020, !alias.scope !19022, !noalias !19027
  %321 = getelementptr inbounds nuw i8, ptr %_221.i.i1921, i64 4, !dbg !19020
  %_72.i.i.i1927 = load i32, ptr %321, align 4, !dbg !19020, !alias.scope !19022, !noalias !19027
  %322 = icmp eq i32 %_51.i.i.i1926, %_72.i.i.i1927, !dbg !19020
  %_0.sroa.0.0.i.i.i1928 = select i1 %320, i1 %322, i1 false, !dbg !19020
  br i1 %_0.sroa.0.0.i.i.i1928, label %bb1.i.i1920, label %bb14.i, !dbg !19030

bb4.i5:                                           ; preds = %bb1.i.i1920
  %323 = getelementptr inbounds nuw i8, ptr %self, i64 280, !dbg !19031
  %_241.0.i = load ptr, ptr %323, align 8, !dbg !19031, !alias.scope !18990, !noalias !19004, !nonnull !12, !noundef !12
  %324 = getelementptr inbounds nuw i8, ptr %self, i64 288, !dbg !19031
  %_241.1.i = load i64, ptr %324, align 8, !dbg !19031, !alias.scope !18990, !noalias !19004, !noundef !12
  %_8.i1930 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_241.0.i, i64 %_241.1.i, !dbg !19032
  br label %bb1.i.i1931, !dbg !19037

bb1.i.i1931:                                      ; preds = %bb13.i.i1934, %bb4.i5
  %_221.i.i1932 = phi ptr [ %_22.i.i1935, %bb13.i.i1934 ], [ %_241.0.i, %bb4.i5 ]
  %_12.i.i1933 = icmp eq ptr %_221.i.i1932, %_8.i1930, !dbg !19039
  br i1 %_12.i.i1933, label %bb6.i, label %bb13.i.i1934, !dbg !19042

bb13.i.i1934:                                     ; preds = %bb1.i.i1931
  %_22.i.i1935 = getelementptr inbounds nuw i8, ptr %_221.i.i1932, i64 16, !dbg !19043
  %325 = getelementptr inbounds nuw i8, ptr %_221.i.i1932, i64 12, !dbg !19045
  %_3.i.i.i1936 = load i32, ptr %325, align 4, !dbg !19045, !alias.scope !19047, !noalias !19052, !noundef !12
  %326 = icmp eq i32 %_3.i.i.i1936, 0, !dbg !19045
  %_51.i.i.i1937 = load i32, ptr %_221.i.i1932, align 4, !dbg !19045, !alias.scope !19047, !noalias !19052
  %327 = getelementptr inbounds nuw i8, ptr %_221.i.i1932, i64 4, !dbg !19045
  %_72.i.i.i1938 = load i32, ptr %327, align 4, !dbg !19045, !alias.scope !19047, !noalias !19052
  %328 = icmp eq i32 %_51.i.i.i1937, %_72.i.i.i1938, !dbg !19045
  %_0.sroa.0.0.i.i.i1939 = select i1 %326, i1 %328, i1 false, !dbg !19045
  br i1 %_0.sroa.0.0.i.i.i1939, label %bb1.i.i1931, label %bb14.i, !dbg !19055

bb6.i:                                            ; preds = %bb1.i.i1931
  %329 = getelementptr inbounds nuw i8, ptr %self, i64 464, !dbg !19056
  %_242.0.i = load ptr, ptr %329, align 8, !dbg !19056, !alias.scope !18992, !noalias !19057, !nonnull !12, !noundef !12
  %330 = getelementptr inbounds nuw i8, ptr %self, i64 472, !dbg !19056
  %_242.1.i = load i64, ptr %330, align 8, !dbg !19056, !alias.scope !18992, !noalias !19057, !noundef !12
  %_8.i1941 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_242.0.i, i64 %_242.1.i, !dbg !19058
  br label %bb1.i.i1942, !dbg !19063

bb1.i.i1942:                                      ; preds = %bb13.i.i1945, %bb6.i
  %_221.i.i1943 = phi ptr [ %_22.i.i1946, %bb13.i.i1945 ], [ %_242.0.i, %bb6.i ]
  %_12.i.i1944 = icmp eq ptr %_221.i.i1943, %_8.i1941, !dbg !19065
  br i1 %_12.i.i1944, label %bb8.i, label %bb13.i.i1945, !dbg !19068

bb13.i.i1945:                                     ; preds = %bb1.i.i1942
  %_22.i.i1946 = getelementptr inbounds nuw i8, ptr %_221.i.i1943, i64 16, !dbg !19069
  %331 = getelementptr inbounds nuw i8, ptr %_221.i.i1943, i64 12, !dbg !19071
  %_3.i.i.i1947 = load i32, ptr %331, align 4, !dbg !19071, !alias.scope !19073, !noalias !19078, !noundef !12
  %332 = icmp eq i32 %_3.i.i.i1947, 0, !dbg !19071
  %_51.i.i.i1948 = load i32, ptr %_221.i.i1943, align 4, !dbg !19071, !alias.scope !19073, !noalias !19078
  %333 = getelementptr inbounds nuw i8, ptr %_221.i.i1943, i64 4, !dbg !19071
  %_72.i.i.i1949 = load i32, ptr %333, align 4, !dbg !19071, !alias.scope !19073, !noalias !19078
  %334 = icmp eq i32 %_51.i.i.i1948, %_72.i.i.i1949, !dbg !19071
  %_0.sroa.0.0.i.i.i1950 = select i1 %332, i1 %334, i1 false, !dbg !19071
  br i1 %_0.sroa.0.0.i.i.i1950, label %bb1.i.i1942, label %bb14.i, !dbg !19081

bb8.i:                                            ; preds = %bb1.i.i1942
  %335 = getelementptr inbounds nuw i8, ptr %self, i64 480, !dbg !19082
  %_243.0.i = load ptr, ptr %335, align 8, !dbg !19082, !alias.scope !18992, !noalias !19057, !nonnull !12, !noundef !12
  %336 = getelementptr inbounds nuw i8, ptr %self, i64 488, !dbg !19082
  %_243.1.i = load i64, ptr %336, align 8, !dbg !19082, !alias.scope !18992, !noalias !19057, !noundef !12
  %_8.i1952 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_243.0.i, i64 %_243.1.i, !dbg !19083
  br label %bb1.i.i1953, !dbg !19088

bb1.i.i1953:                                      ; preds = %bb13.i.i1956, %bb8.i
  %_221.i.i1954 = phi ptr [ %_22.i.i1957, %bb13.i.i1956 ], [ %_243.0.i, %bb8.i ]
  %_12.i.i1955 = icmp eq ptr %_221.i.i1954, %_8.i1952, !dbg !19090
  br i1 %_12.i.i1955, label %bb14.i, label %bb13.i.i1956, !dbg !19093

bb13.i.i1956:                                     ; preds = %bb1.i.i1953
  %_22.i.i1957 = getelementptr inbounds nuw i8, ptr %_221.i.i1954, i64 16, !dbg !19094
  %337 = getelementptr inbounds nuw i8, ptr %_221.i.i1954, i64 12, !dbg !19096
  %_3.i.i.i1958 = load i32, ptr %337, align 4, !dbg !19096, !alias.scope !19098, !noalias !19103, !noundef !12
  %338 = icmp eq i32 %_3.i.i.i1958, 0, !dbg !19096
  %_51.i.i.i1959 = load i32, ptr %_221.i.i1954, align 4, !dbg !19096, !alias.scope !19098, !noalias !19103
  %339 = getelementptr inbounds nuw i8, ptr %_221.i.i1954, i64 4, !dbg !19096
  %_72.i.i.i1960 = load i32, ptr %339, align 4, !dbg !19096, !alias.scope !19098, !noalias !19103
  %340 = icmp eq i32 %_51.i.i.i1959, %_72.i.i.i1960, !dbg !19096
  %_0.sroa.0.0.i.i.i1961 = select i1 %338, i1 %340, i1 false, !dbg !19096
  br i1 %_0.sroa.0.0.i.i.i1961, label %bb1.i.i1953, label %bb14.i, !dbg !19106

bb14.i:                                           ; preds = %bb13.i.i1923, %bb13.i.i1934, %bb13.i.i1945, %bb13.i.i1956, %bb1.i.i1953
  %stationary.sroa.0.0.i = phi i1 [ false, %bb13.i.i1945 ], [ false, %bb13.i.i1934 ], [ false, %bb13.i.i1956 ], [ true, %bb1.i.i1953 ], [ false, %bb13.i.i1923 ], !dbg !19107
  %341 = getelementptr inbounds nuw i8, ptr %self, i64 776, !dbg !19108
  %342 = load i8, ptr %341, align 4, !dbg !19108, !range !5399, !alias.scope !18984, !noalias !19112, !noundef !12
  %343 = getelementptr inbounds nuw i8, ptr %self, i64 777, !dbg !19113
  %344 = load i8, ptr %343, align 1, !dbg !19113, !range !5399, !alias.scope !18984, !noalias !19112, !noundef !12
  %345 = getelementptr inbounds nuw i8, ptr %self, i64 544, !dbg !19115
  %ring.i = load i64, ptr %345, align 8, !dbg !19115, !alias.scope !18988, !noalias !19117, !noundef !12
  %346 = getelementptr inbounds nuw i8, ptr %self, i64 552, !dbg !19118
  %main.i = load i64, ptr %346, align 8, !dbg !19118, !alias.scope !18988, !noalias !19117, !noundef !12
  %_42.i = load i32, ptr %_35, align 4, !dbg !19120, !alias.scope !18994, !noalias !19122, !noundef !12
  %347 = getelementptr inbounds nuw i8, ptr %self, i64 564, !dbg !19123
  %_43.i = load i32, ptr %347, align 4, !dbg !19123, !alias.scope !18994, !noalias !19122, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i), !dbg !19125, !noalias !19127
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i, i8 0, i64 1024, i1 false), !noalias !19127
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i), !dbg !19128, !noalias !19127
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i, i8 0, i64 1024, i1 false), !noalias !19127
  %_38.i = zext nneg i8 %342 to i32, !dbg !19108
  %.none.i = sub nsw i32 0, %_38.i, !dbg !19130
  %_39.i = zext nneg i8 %344 to i32, !dbg !19113
  %all.sroa.0.0.i = sub nsw i32 0, %_39.i, !dbg !19113
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i), !dbg !19131, !noalias !19127
; call <true_peak_limiter::UniformHot<f32>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotfE3newB5_(ptr noalias noundef align 8 captures(none) dereferenceable(80) %uniform_left.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33, i64 %ring.i, i64 %main.i) #30, !dbg !19133
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_right.i), !dbg !19134, !noalias !19127
  %_32.val = load i64, ptr %345, align 8, !dbg !19136, !noundef !12
  %_32.val1709 = load i64, ptr %346, align 8, !dbg !19136, !noundef !12
; call <true_peak_limiter::UniformHot<f32>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotfE3newB5_(ptr noalias noundef align 8 captures(none) dereferenceable(80) %uniform_right.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34, i64 %_32.val, i64 %_32.val1709) #30, !dbg !19136
  %_172.not.i3957 = icmp eq i64 %frames, 0, !dbg !19137
  br i1 %_172.not.i3957, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformfEB2_.exit, label %bb67.i.lr.ph, !dbg !19137

bb67.i.lr.ph:                                     ; preds = %bb14.i
  %348 = zext i32 %_43.i to i64, !dbg !19123
  %349 = zext i32 %_42.i to i64, !dbg !19120
  %d9.i.i1963 = lshr i64 %frames, 5, !dbg !19147
  %r2.i.i1964 = and i64 %frames, 31, !dbg !19153
  %_19.not.i.i1965 = icmp ne i64 %r2.i.i1964, 0, !dbg !19154
  %350 = zext i1 %_19.not.i.i1965 to i64, !dbg !19154
  %yield_count.sroa.0.0.i.i1966 = add nuw nsw i64 %d9.i.i1963, %350, !dbg !19154
  %history.i36.i.sroa.7.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 4
  %history.i36.i.sroa.10.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 8
  %history.i36.i.sroa.13.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 12
  %history.i36.i.sroa.16.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 16
  %history.i36.i.sroa.19.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 20
  %history.i36.i.sroa.22.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 24
  %history.i36.i.sroa.26.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 28
  %history.i36.i.sroa.29.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 32
  %history.i36.i.sroa.32.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 36
  %history.i36.i.sroa.35.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 40
  %history.i36.i.sroa.38.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 44
  %351 = getelementptr inbounds nuw i8, ptr %self, i64 588
  %352 = getelementptr inbounds nuw i8, ptr %self, i64 592
  %353 = getelementptr inbounds nuw i8, ptr %self, i64 596
  %row1.i.i.i72.i = getelementptr inbounds nuw i8, ptr %self, i64 600
  %354 = getelementptr inbounds nuw i8, ptr %self, i64 604
  %355 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %356 = getelementptr inbounds nuw i8, ptr %self, i64 612
  %row3.i.i.i86.i = getelementptr inbounds nuw i8, ptr %self, i64 616
  %357 = getelementptr inbounds nuw i8, ptr %self, i64 620
  %358 = getelementptr inbounds nuw i8, ptr %self, i64 624
  %359 = getelementptr inbounds nuw i8, ptr %self, i64 628
  %row5.i.i.i100.i = getelementptr inbounds nuw i8, ptr %self, i64 632
  %360 = getelementptr inbounds nuw i8, ptr %self, i64 636
  %361 = getelementptr inbounds nuw i8, ptr %self, i64 640
  %362 = getelementptr inbounds nuw i8, ptr %self, i64 644
  %row7.i.i.i114.i = getelementptr inbounds nuw i8, ptr %self, i64 648
  %363 = getelementptr inbounds nuw i8, ptr %self, i64 652
  %364 = getelementptr inbounds nuw i8, ptr %self, i64 656
  %365 = getelementptr inbounds nuw i8, ptr %self, i64 660
  %row9.i.i.i128.i = getelementptr inbounds nuw i8, ptr %self, i64 664
  %366 = getelementptr inbounds nuw i8, ptr %self, i64 668
  %367 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %368 = getelementptr inbounds nuw i8, ptr %self, i64 676
  %row11.i.i.i142.i = getelementptr inbounds nuw i8, ptr %self, i64 680
  %369 = getelementptr inbounds nuw i8, ptr %self, i64 684
  %370 = getelementptr inbounds nuw i8, ptr %self, i64 688
  %371 = getelementptr inbounds nuw i8, ptr %self, i64 692
  %row13.i.i.i156.i = getelementptr inbounds nuw i8, ptr %self, i64 696
  %372 = getelementptr inbounds nuw i8, ptr %self, i64 700
  %373 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %374 = getelementptr inbounds nuw i8, ptr %self, i64 708
  %row15.i.i.i170.i = getelementptr inbounds nuw i8, ptr %self, i64 712
  %375 = getelementptr inbounds nuw i8, ptr %self, i64 716
  %376 = getelementptr inbounds nuw i8, ptr %self, i64 720
  %377 = getelementptr inbounds nuw i8, ptr %self, i64 724
  %row17.i.i.i184.i = getelementptr inbounds nuw i8, ptr %self, i64 728
  %378 = getelementptr inbounds nuw i8, ptr %self, i64 732
  %379 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %380 = getelementptr inbounds nuw i8, ptr %self, i64 740
  %row19.i.i.i198.i = getelementptr inbounds nuw i8, ptr %self, i64 744
  %381 = getelementptr inbounds nuw i8, ptr %self, i64 748
  %382 = getelementptr inbounds nuw i8, ptr %self, i64 752
  %383 = getelementptr inbounds nuw i8, ptr %self, i64 756
  %row21.i.i.i212.i = getelementptr inbounds nuw i8, ptr %self, i64 760
  %384 = getelementptr inbounds nuw i8, ptr %self, i64 764
  %385 = getelementptr inbounds nuw i8, ptr %self, i64 768
  %386 = getelementptr inbounds nuw i8, ptr %self, i64 772
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
  %387 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 48
  %_72.i.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 56
  %_72.i.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 64
  %388 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 48
  %_73.i.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 56
  %_73.i.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 64
  %_115.i = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 48
  %389 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 60
  %390 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 56
  %391 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 52
  %_117.i = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 64
  %392 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 76
  %393 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 72
  %394 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 68
  %_119.i = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 48
  %395 = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 60
  %396 = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 56
  %397 = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 52
  %_121.i = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 64
  %398 = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 76
  %399 = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 72
  %400 = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 68
  %_9.i1512 = add nsw i32 %_38.i, -1
  %401 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 8
  %_21.i267.i = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 72
  %_22.i268.i = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 76
  %402 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 16
  %403 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 24
  %404 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 84
  %405 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 88
  %406 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 80
  %407 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 40
  %408 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 32
  %_9.i1492 = add nsw i32 %_39.i, -1
  %409 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 8
  %_21.i.i = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 72
  %_22.i.i = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 76
  %410 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 16
  %411 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 24
  %412 = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 84
  %413 = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 88
  %414 = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 80
  %415 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 40
  %416 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 32
  br label %bb67.i, !dbg !19137

bb31.i.bb27.i.loopexit_crit_edge:                 ; preds = %bb44.i
  store float %running.sroa.0.0.i294.lcssa51425904, ptr %_21.i267.i, align 1, !dbg !19155
  store float %running.sroa.0.0.i.lcssa51795921, ptr %_21.i.i, align 1, !dbg !19185
  store i32 %storemerge.i299.lcssa37793880, ptr %_22.i268.i, align 4
  store i32 %storemerge.i.lcssa38373919, ptr %_22.i.i, align 4
  br label %bb27.i.loopexit, !dbg !19188

bb27.i.loopexit:                                  ; preds = %bb31.i.bb27.i.loopexit_crit_edge, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i
  %ring_cursor.sroa.0.1.i.lcssa = phi i64 [ %ring_cursor.sroa.0.2.i, %bb31.i.bb27.i.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i3958, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i ], !dbg !19189
  %main_cursor.sroa.0.1.i.lcssa = phi i64 [ %main_cursor.sroa.0.2.i, %bb31.i.bb27.i.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i3959, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i ], !dbg !19190
  %_172.not.i = icmp eq i64 %418, 0, !dbg !19137
  %indvars.iv.next4887 = add i64 %indvars.iv4886, -32, !dbg !19137
  br i1 %_172.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformfEB2_.exit.loopexit, label %bb67.i, !dbg !19137

bb67.i:                                           ; preds = %bb67.i.lr.ph, %bb27.i.loopexit
  %indvars.iv4886 = phi i64 [ %frames, %bb67.i.lr.ph ], [ %indvars.iv.next4887, %bb27.i.loopexit ]
  %iter2.sroa.0.0.i3961 = phi i64 [ %yield_count.sroa.0.0.i.i1966, %bb67.i.lr.ph ], [ %418, %bb27.i.loopexit ]
  %iter1.sroa.0.0.i3960 = phi i64 [ 0, %bb67.i.lr.ph ], [ %417, %bb27.i.loopexit ]
  %main_cursor.sroa.0.0.i3959 = phi i64 [ %349, %bb67.i.lr.ph ], [ %main_cursor.sroa.0.1.i.lcssa, %bb27.i.loopexit ]
  %ring_cursor.sroa.0.0.i3958 = phi i64 [ %348, %bb67.i.lr.ph ], [ %ring_cursor.sroa.0.1.i.lcssa, %bb27.i.loopexit ]
  %umin4906 = call i64 @llvm.umin.i64(i64 %indvars.iv4886, i64 32), !dbg !19191
  %umax4892 = call i64 @llvm.umax.i64(i64 %umin4906, i64 1), !dbg !19191
  %417 = add i64 %iter1.sroa.0.0.i3960, 32, !dbg !19191
  %418 = add i64 %iter2.sroa.0.0.i3961, -1, !dbg !19195
  %_53.i = sub i64 %frames, %iter1.sroa.0.0.i3960, !dbg !19196
  %..i1967 = tail call noundef i64 @llvm.umin.i64(i64 %_53.i, i64 32), !dbg !19197
  %history.i36.i.sroa.0.0.copyload = load float, ptr %hot_left.i, align 4, !dbg !19201
  %history.i36.i.sroa.7.0.copyload = load float, ptr %history.i36.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !19201
  %history.i36.i.sroa.10.0.copyload = load float, ptr %history.i36.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !19201
  %history.i36.i.sroa.13.0.copyload = load float, ptr %history.i36.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !19201
  %history.i36.i.sroa.16.0.copyload = load float, ptr %history.i36.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !19201
  %history.i36.i.sroa.19.0.copyload = load float, ptr %history.i36.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !19201
  %history.i36.i.sroa.22.0.copyload = load float, ptr %history.i36.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !19201
  %history.i36.i.sroa.26.0.copyload = load float, ptr %history.i36.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !19201
  %history.i36.i.sroa.29.0.copyload = load float, ptr %history.i36.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !19201
  %history.i36.i.sroa.32.0.copyload = load float, ptr %history.i36.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !19201
  %history.i36.i.sroa.35.0.copyload = load float, ptr %history.i36.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !19201
  %history.i36.i.sroa.38.0.copyload = load float, ptr %history.i36.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !19201
  %_20.i39.i3552.not = icmp eq i64 %frames, %iter1.sroa.0.0.i3960, !dbg !19203
  br i1 %_20.i39.i3552.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit240.i, label %bb5.i40.i.lr.ph, !dbg !19207

bb5.i40.i.lr.ph:                                  ; preds = %bb67.i
  %_11.i.i.i60.i = load float, ptr %_31, align 4
  %_14.i.i.i63.i = load float, ptr %351, align 4
  %_17.i.i.i66.i = load float, ptr %352, align 4
  %_20.i.i.i69.i = load float, ptr %353, align 4
  %_25.i.i.i74.i = load float, ptr %row1.i.i.i72.i, align 4
  %_28.i.i.i77.i = load float, ptr %354, align 4
  %_31.i.i.i80.i = load float, ptr %355, align 4
  %_34.i.i.i83.i = load float, ptr %356, align 4
  %_39.i.i.i88.i = load float, ptr %row3.i.i.i86.i, align 4
  %_42.i.i.i91.i = load float, ptr %357, align 4
  %_45.i.i.i94.i = load float, ptr %358, align 4
  %_48.i.i.i97.i = load float, ptr %359, align 4
  %_53.i.i.i102.i = load float, ptr %row5.i.i.i100.i, align 4
  %_56.i.i.i105.i = load float, ptr %360, align 4
  %_59.i.i.i108.i = load float, ptr %361, align 4
  %_62.i.i.i111.i = load float, ptr %362, align 4
  %_67.i.i.i116.i = load float, ptr %row7.i.i.i114.i, align 4
  %_70.i.i.i119.i = load float, ptr %363, align 4
  %_73.i.i.i122.i = load float, ptr %364, align 4
  %_76.i.i.i125.i = load float, ptr %365, align 4
  %_81.i.i.i130.i = load float, ptr %row9.i.i.i128.i, align 4
  %_84.i.i.i133.i = load float, ptr %366, align 4
  %_87.i.i.i136.i = load float, ptr %367, align 4
  %_90.i.i.i139.i = load float, ptr %368, align 4
  %_95.i.i.i144.i = load float, ptr %row11.i.i.i142.i, align 4
  %_98.i.i.i147.i = load float, ptr %369, align 4
  %_101.i.i.i150.i = load float, ptr %370, align 4
  %_104.i.i.i153.i = load float, ptr %371, align 4
  %_109.i.i.i158.i = load float, ptr %row13.i.i.i156.i, align 4
  %_112.i.i.i161.i = load float, ptr %372, align 4
  %_115.i.i.i164.i = load float, ptr %373, align 4
  %_118.i.i.i167.i = load float, ptr %374, align 4
  %_123.i.i.i172.i = load float, ptr %row15.i.i.i170.i, align 4
  %_126.i.i.i175.i = load float, ptr %375, align 4
  %_129.i.i.i178.i = load float, ptr %376, align 4
  %_132.i.i.i181.i = load float, ptr %377, align 4
  %_137.i.i.i186.i = load float, ptr %row17.i.i.i184.i, align 4
  %_140.i.i.i189.i = load float, ptr %378, align 4
  %_143.i.i.i192.i = load float, ptr %379, align 4
  %_146.i.i.i195.i = load float, ptr %380, align 4
  %_151.i.i.i200.i = load float, ptr %row19.i.i.i198.i, align 4
  %_154.i.i.i203.i = load float, ptr %381, align 4
  %_157.i.i.i206.i = load float, ptr %382, align 4
  %_160.i.i.i209.i = load float, ptr %383, align 4
  %_165.i.i.i214.i = load float, ptr %row21.i.i.i212.i, align 4
  %_168.i.i.i217.i = load float, ptr %384, align 4
  %_171.i.i.i220.i = load float, ptr %385, align 4
  %_174.i.i.i223.i = load float, ptr %386, align 4
  br label %bb5.i40.i, !dbg !19207

bb5.i40.i:                                        ; preds = %bb5.i40.i.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225
  %iter.sroa.0.0.i38.i3564 = phi i64 [ 0, %bb5.i40.i.lr.ph ], [ %419, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225 ]
  %history.i36.i.sroa.35.03563 = phi float [ %history.i36.i.sroa.35.0.copyload, %bb5.i40.i.lr.ph ], [ %history.i36.i.sroa.32.03562, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225 ]
  %history.i36.i.sroa.32.03562 = phi float [ %history.i36.i.sroa.32.0.copyload, %bb5.i40.i.lr.ph ], [ %history.i36.i.sroa.29.03561, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225 ]
  %history.i36.i.sroa.29.03561 = phi float [ %history.i36.i.sroa.29.0.copyload, %bb5.i40.i.lr.ph ], [ %history.i36.i.sroa.26.03560, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225 ]
  %history.i36.i.sroa.26.03560 = phi float [ %history.i36.i.sroa.26.0.copyload, %bb5.i40.i.lr.ph ], [ %history.i36.i.sroa.22.03559, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225 ]
  %history.i36.i.sroa.22.03559 = phi float [ %history.i36.i.sroa.22.0.copyload, %bb5.i40.i.lr.ph ], [ %history.i36.i.sroa.19.03558, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225 ]
  %history.i36.i.sroa.19.03558 = phi float [ %history.i36.i.sroa.19.0.copyload, %bb5.i40.i.lr.ph ], [ %history.i36.i.sroa.16.03557, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225 ]
  %history.i36.i.sroa.16.03557 = phi float [ %history.i36.i.sroa.16.0.copyload, %bb5.i40.i.lr.ph ], [ %history.i36.i.sroa.13.03556, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225 ]
  %history.i36.i.sroa.13.03556 = phi float [ %history.i36.i.sroa.13.0.copyload, %bb5.i40.i.lr.ph ], [ %history.i36.i.sroa.10.03555, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225 ]
  %history.i36.i.sroa.10.03555 = phi float [ %history.i36.i.sroa.10.0.copyload, %bb5.i40.i.lr.ph ], [ %history.i36.i.sroa.7.03554, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225 ]
  %history.i36.i.sroa.7.03554 = phi float [ %history.i36.i.sroa.7.0.copyload, %bb5.i40.i.lr.ph ], [ %history.i36.i.sroa.0.03553, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225 ]
  %history.i36.i.sroa.0.03553 = phi float [ %history.i36.i.sroa.0.0.copyload, %bb5.i40.i.lr.ph ], [ %_0.i1223, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225 ]
  %419 = add nuw nsw i64 %iter.sroa.0.0.i38.i3564, 1, !dbg !19208
  %_11.i41.i = add nuw nsw i64 %iter.sroa.0.0.i38.i3564, %iter1.sroa.0.0.i3960, !dbg !19211
  %_24.i42.i = icmp ugt i64 %_11.i41.i, %left_io.1, !dbg !19212
  br i1 %_24.i42.i, label %bb7.i239.i, label %bb8.i43.i, !dbg !19212, !prof !905

bb8.i43.i:                                        ; preds = %bb5.i40.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19215), !dbg !19218
  %_3.not.i1221 = icmp eq i64 %left_io.1, %_11.i41.i, !dbg !19219
  br i1 %_3.not.i1221, label %panic.i1224, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225, !dbg !19219

panic.i1224:                                      ; preds = %bb8.i43.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #31, !dbg !19219, !noalias !19221
  unreachable, !dbg !19219

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225: ; preds = %bb8.i43.i
  %_31.i45.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %_11.i41.i, !dbg !19225
  %_0.i1223 = load float, ptr %_31.i45.i, align 4, !dbg !19219, !alias.scope !19215, !noalias !19227, !noundef !12
  %420 = tail call noundef float @llvm.fabs.f32(float %history.i36.i.sroa.19.03558), !dbg !19228
  %_0.i1073 = fmul float %_0.i1223, %_11.i.i.i60.i, !dbg !19231
  %_0.i857 = fadd float %_0.i1073, 0.000000e+00, !dbg !19234
  %_0.i1072 = fmul float %_0.i1223, %_14.i.i.i63.i, !dbg !19236
  %_0.i856 = fadd float %_0.i1072, 0.000000e+00, !dbg !19238
  %_0.i1071 = fmul float %_0.i1223, %_17.i.i.i66.i, !dbg !19240
  %_0.i855 = fadd float %_0.i1071, 0.000000e+00, !dbg !19242
  %_0.i1070 = fmul float %_0.i1223, %_20.i.i.i69.i, !dbg !19244
  %_0.i854 = fadd float %_0.i1070, 0.000000e+00, !dbg !19246
  %_0.i1069 = fmul float %history.i36.i.sroa.0.03553, %_25.i.i.i74.i, !dbg !19248
  %_0.i853 = fadd float %_0.i857, %_0.i1069, !dbg !19250
  %_0.i1068 = fmul float %history.i36.i.sroa.0.03553, %_28.i.i.i77.i, !dbg !19252
  %_0.i852 = fadd float %_0.i856, %_0.i1068, !dbg !19254
  %_0.i1067 = fmul float %history.i36.i.sroa.0.03553, %_31.i.i.i80.i, !dbg !19256
  %_0.i851 = fadd float %_0.i855, %_0.i1067, !dbg !19258
  %_0.i1066 = fmul float %history.i36.i.sroa.0.03553, %_34.i.i.i83.i, !dbg !19260
  %_0.i850 = fadd float %_0.i854, %_0.i1066, !dbg !19262
  %_0.i1065 = fmul float %history.i36.i.sroa.7.03554, %_39.i.i.i88.i, !dbg !19264
  %_0.i849 = fadd float %_0.i853, %_0.i1065, !dbg !19266
  %_0.i1064 = fmul float %history.i36.i.sroa.7.03554, %_42.i.i.i91.i, !dbg !19268
  %_0.i848 = fadd float %_0.i852, %_0.i1064, !dbg !19270
  %_0.i1063 = fmul float %history.i36.i.sroa.7.03554, %_45.i.i.i94.i, !dbg !19272
  %_0.i847 = fadd float %_0.i851, %_0.i1063, !dbg !19274
  %_0.i1062 = fmul float %history.i36.i.sroa.7.03554, %_48.i.i.i97.i, !dbg !19276
  %_0.i846 = fadd float %_0.i850, %_0.i1062, !dbg !19278
  %_0.i1061 = fmul float %history.i36.i.sroa.10.03555, %_53.i.i.i102.i, !dbg !19280
  %_0.i845 = fadd float %_0.i849, %_0.i1061, !dbg !19282
  %_0.i1060 = fmul float %history.i36.i.sroa.10.03555, %_56.i.i.i105.i, !dbg !19284
  %_0.i844 = fadd float %_0.i848, %_0.i1060, !dbg !19286
  %_0.i1059 = fmul float %history.i36.i.sroa.10.03555, %_59.i.i.i108.i, !dbg !19288
  %_0.i843 = fadd float %_0.i847, %_0.i1059, !dbg !19290
  %_0.i1058 = fmul float %history.i36.i.sroa.10.03555, %_62.i.i.i111.i, !dbg !19292
  %_0.i842 = fadd float %_0.i846, %_0.i1058, !dbg !19294
  %_0.i1057 = fmul float %history.i36.i.sroa.13.03556, %_67.i.i.i116.i, !dbg !19296
  %_0.i841 = fadd float %_0.i845, %_0.i1057, !dbg !19298
  %_0.i1056 = fmul float %history.i36.i.sroa.13.03556, %_70.i.i.i119.i, !dbg !19300
  %_0.i840 = fadd float %_0.i844, %_0.i1056, !dbg !19302
  %_0.i1055 = fmul float %history.i36.i.sroa.13.03556, %_73.i.i.i122.i, !dbg !19304
  %_0.i839 = fadd float %_0.i843, %_0.i1055, !dbg !19306
  %_0.i1054 = fmul float %history.i36.i.sroa.13.03556, %_76.i.i.i125.i, !dbg !19308
  %_0.i838 = fadd float %_0.i842, %_0.i1054, !dbg !19310
  %_0.i1053 = fmul float %history.i36.i.sroa.16.03557, %_81.i.i.i130.i, !dbg !19312
  %_0.i837 = fadd float %_0.i841, %_0.i1053, !dbg !19314
  %_0.i1052 = fmul float %history.i36.i.sroa.16.03557, %_84.i.i.i133.i, !dbg !19316
  %_0.i836 = fadd float %_0.i840, %_0.i1052, !dbg !19318
  %_0.i1051 = fmul float %history.i36.i.sroa.16.03557, %_87.i.i.i136.i, !dbg !19320
  %_0.i835 = fadd float %_0.i839, %_0.i1051, !dbg !19322
  %_0.i1050 = fmul float %history.i36.i.sroa.16.03557, %_90.i.i.i139.i, !dbg !19324
  %_0.i834 = fadd float %_0.i838, %_0.i1050, !dbg !19326
  %_0.i1049 = fmul float %history.i36.i.sroa.19.03558, %_95.i.i.i144.i, !dbg !19328
  %_0.i833 = fadd float %_0.i837, %_0.i1049, !dbg !19330
  %_0.i1048 = fmul float %history.i36.i.sroa.19.03558, %_98.i.i.i147.i, !dbg !19332
  %_0.i832 = fadd float %_0.i836, %_0.i1048, !dbg !19334
  %_0.i1047 = fmul float %history.i36.i.sroa.19.03558, %_101.i.i.i150.i, !dbg !19336
  %_0.i831 = fadd float %_0.i835, %_0.i1047, !dbg !19338
  %_0.i1046 = fmul float %history.i36.i.sroa.19.03558, %_104.i.i.i153.i, !dbg !19340
  %_0.i830 = fadd float %_0.i834, %_0.i1046, !dbg !19342
  %_0.i1045 = fmul float %history.i36.i.sroa.22.03559, %_109.i.i.i158.i, !dbg !19344
  %_0.i829 = fadd float %_0.i833, %_0.i1045, !dbg !19346
  %_0.i1044 = fmul float %history.i36.i.sroa.22.03559, %_112.i.i.i161.i, !dbg !19348
  %_0.i828 = fadd float %_0.i832, %_0.i1044, !dbg !19350
  %_0.i1043 = fmul float %history.i36.i.sroa.22.03559, %_115.i.i.i164.i, !dbg !19352
  %_0.i827 = fadd float %_0.i831, %_0.i1043, !dbg !19354
  %_0.i1042 = fmul float %history.i36.i.sroa.22.03559, %_118.i.i.i167.i, !dbg !19356
  %_0.i826 = fadd float %_0.i830, %_0.i1042, !dbg !19358
  %_0.i1041 = fmul float %history.i36.i.sroa.26.03560, %_123.i.i.i172.i, !dbg !19360
  %_0.i825 = fadd float %_0.i829, %_0.i1041, !dbg !19362
  %_0.i1040 = fmul float %history.i36.i.sroa.26.03560, %_126.i.i.i175.i, !dbg !19364
  %_0.i824 = fadd float %_0.i828, %_0.i1040, !dbg !19366
  %_0.i1039 = fmul float %history.i36.i.sroa.26.03560, %_129.i.i.i178.i, !dbg !19368
  %_0.i823 = fadd float %_0.i827, %_0.i1039, !dbg !19370
  %_0.i1038 = fmul float %history.i36.i.sroa.26.03560, %_132.i.i.i181.i, !dbg !19372
  %_0.i822 = fadd float %_0.i826, %_0.i1038, !dbg !19374
  %_0.i1037 = fmul float %history.i36.i.sroa.29.03561, %_137.i.i.i186.i, !dbg !19376
  %_0.i821 = fadd float %_0.i825, %_0.i1037, !dbg !19378
  %_0.i1036 = fmul float %history.i36.i.sroa.29.03561, %_140.i.i.i189.i, !dbg !19380
  %_0.i820 = fadd float %_0.i824, %_0.i1036, !dbg !19382
  %_0.i1035 = fmul float %history.i36.i.sroa.29.03561, %_143.i.i.i192.i, !dbg !19384
  %_0.i819 = fadd float %_0.i823, %_0.i1035, !dbg !19386
  %_0.i1034 = fmul float %history.i36.i.sroa.29.03561, %_146.i.i.i195.i, !dbg !19388
  %_0.i818 = fadd float %_0.i822, %_0.i1034, !dbg !19390
  %_0.i1033 = fmul float %history.i36.i.sroa.32.03562, %_151.i.i.i200.i, !dbg !19392
  %_0.i817 = fadd float %_0.i821, %_0.i1033, !dbg !19394
  %_0.i1032 = fmul float %history.i36.i.sroa.32.03562, %_154.i.i.i203.i, !dbg !19396
  %_0.i816 = fadd float %_0.i820, %_0.i1032, !dbg !19398
  %_0.i1031 = fmul float %history.i36.i.sroa.32.03562, %_157.i.i.i206.i, !dbg !19400
  %_0.i815 = fadd float %_0.i819, %_0.i1031, !dbg !19402
  %_0.i1030 = fmul float %history.i36.i.sroa.32.03562, %_160.i.i.i209.i, !dbg !19404
  %_0.i814 = fadd float %_0.i818, %_0.i1030, !dbg !19406
  %_0.i1029 = fmul float %history.i36.i.sroa.35.03563, %_165.i.i.i214.i, !dbg !19408
  %_0.i813 = fadd float %_0.i817, %_0.i1029, !dbg !19410
  %_0.i1028 = fmul float %history.i36.i.sroa.35.03563, %_168.i.i.i217.i, !dbg !19412
  %_0.i812 = fadd float %_0.i816, %_0.i1028, !dbg !19414
  %_0.i1027 = fmul float %history.i36.i.sroa.35.03563, %_171.i.i.i220.i, !dbg !19416
  %_0.i811 = fadd float %_0.i815, %_0.i1027, !dbg !19418
  %_0.i1026 = fmul float %history.i36.i.sroa.35.03563, %_174.i.i.i223.i, !dbg !19420
  %_0.i810 = fadd float %_0.i814, %_0.i1026, !dbg !19422
  %421 = tail call noundef float @llvm.fabs.f32(float %_0.i813), !dbg !19424
  %_3.i.i1609.inv = fcmp ogt float %420, %421, !dbg !19426
  %_4.i.i1616.v = select i1 %_3.i.i1609.inv, float %420, float %421, !dbg !19426
  %422 = tail call noundef float @llvm.fabs.f32(float %_0.i812), !dbg !19424
  %_3.i.i1609.inv.1 = fcmp ogt float %_4.i.i1616.v, %422, !dbg !19426
  %_4.i.i1616.v.1 = select i1 %_3.i.i1609.inv.1, float %_4.i.i1616.v, float %422, !dbg !19426
  %423 = tail call noundef float @llvm.fabs.f32(float %_0.i811), !dbg !19424
  %_3.i.i1609.inv.2 = fcmp ogt float %_4.i.i1616.v.1, %423, !dbg !19426
  %_4.i.i1616.v.2 = select i1 %_3.i.i1609.inv.2, float %_4.i.i1616.v.1, float %423, !dbg !19426
  %424 = tail call noundef float @llvm.fabs.f32(float %_0.i810), !dbg !19424
  %_3.i.i1609.inv.3 = fcmp ogt float %_4.i.i1616.v.2, %424, !dbg !19426
  %_4.i.i1616.v.3 = select i1 %_3.i.i1609.inv.3, float %_4.i.i1616.v.2, float %424, !dbg !19426
  %_39.i234.i = getelementptr inbounds nuw float, ptr %peaks_left.i, i64 %iter.sroa.0.0.i38.i3564, !dbg !19429
  store float %_4.i.i1616.v.3, ptr %_39.i234.i, align 4, !dbg !19434, !alias.scope !19436, !noalias !19227
  %exitcond4890.not = icmp eq i64 %419, %umax4892, !dbg !19203
  br i1 %exitcond4890.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit240.i, label %bb5.i40.i, !dbg !19207

bb7.i239.i:                                       ; preds = %bb5.i40.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_11.i41.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e0592aef22128a0ac53753b9632a8183) #31, !dbg !19439, !noalias !19227
  unreachable, !dbg !19439

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit240.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225, %bb67.i
  %history.i36.i.sroa.0.0.lcssa = phi float [ %history.i36.i.sroa.0.0.copyload, %bb67.i ], [ %_0.i1223, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225 ], !dbg !19440
  %history.i36.i.sroa.7.0.lcssa = phi float [ %history.i36.i.sroa.7.0.copyload, %bb67.i ], [ %history.i36.i.sroa.0.03553, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225 ], !dbg !19440
  %history.i36.i.sroa.10.0.lcssa = phi float [ %history.i36.i.sroa.10.0.copyload, %bb67.i ], [ %history.i36.i.sroa.7.03554, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225 ], !dbg !19440
  %history.i36.i.sroa.13.0.lcssa = phi float [ %history.i36.i.sroa.13.0.copyload, %bb67.i ], [ %history.i36.i.sroa.10.03555, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225 ], !dbg !19440
  %history.i36.i.sroa.16.0.lcssa = phi float [ %history.i36.i.sroa.16.0.copyload, %bb67.i ], [ %history.i36.i.sroa.13.03556, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225 ], !dbg !19440
  %history.i36.i.sroa.19.0.lcssa = phi float [ %history.i36.i.sroa.19.0.copyload, %bb67.i ], [ %history.i36.i.sroa.16.03557, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225 ], !dbg !19440
  %history.i36.i.sroa.22.0.lcssa = phi float [ %history.i36.i.sroa.22.0.copyload, %bb67.i ], [ %history.i36.i.sroa.19.03558, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225 ], !dbg !19440
  %history.i36.i.sroa.26.0.lcssa = phi float [ %history.i36.i.sroa.26.0.copyload, %bb67.i ], [ %history.i36.i.sroa.22.03559, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225 ], !dbg !19440
  %history.i36.i.sroa.29.0.lcssa = phi float [ %history.i36.i.sroa.29.0.copyload, %bb67.i ], [ %history.i36.i.sroa.26.03560, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225 ], !dbg !19440
  %history.i36.i.sroa.32.0.lcssa = phi float [ %history.i36.i.sroa.32.0.copyload, %bb67.i ], [ %history.i36.i.sroa.29.03561, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225 ], !dbg !19440
  %history.i36.i.sroa.35.0.lcssa = phi float [ %history.i36.i.sroa.35.0.copyload, %bb67.i ], [ %history.i36.i.sroa.32.03562, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225 ], !dbg !19440
  %history.i36.i.sroa.38.0.lcssa = phi float [ %history.i36.i.sroa.38.0.copyload, %bb67.i ], [ %history.i36.i.sroa.35.03563, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1225 ], !dbg !19440
  store float %history.i36.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !19441
  store float %history.i36.i.sroa.7.0.lcssa, ptr %history.i36.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !19441
  store float %history.i36.i.sroa.10.0.lcssa, ptr %history.i36.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !19441
  store float %history.i36.i.sroa.13.0.lcssa, ptr %history.i36.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !19441
  store float %history.i36.i.sroa.16.0.lcssa, ptr %history.i36.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !19441
  store float %history.i36.i.sroa.19.0.lcssa, ptr %history.i36.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !19441
  store float %history.i36.i.sroa.22.0.lcssa, ptr %history.i36.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !19441
  store float %history.i36.i.sroa.26.0.lcssa, ptr %history.i36.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !19441
  store float %history.i36.i.sroa.29.0.lcssa, ptr %history.i36.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !19441
  store float %history.i36.i.sroa.32.0.lcssa, ptr %history.i36.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !19441
  store float %history.i36.i.sroa.35.0.lcssa, ptr %history.i36.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !19441
  store float %history.i36.i.sroa.38.0.lcssa, ptr %history.i36.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !19441
  %history.i.i.sroa.0.0.copyload = load float, ptr %hot_right.i, align 4, !dbg !19442
  %history.i.i.sroa.7.0.copyload = load float, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !19442
  %history.i.i.sroa.10.0.copyload = load float, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !19442
  %history.i.i.sroa.13.0.copyload = load float, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !19442
  %history.i.i.sroa.16.0.copyload = load float, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !19442
  %history.i.i.sroa.19.0.copyload = load float, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !19442
  %history.i.i.sroa.22.0.copyload = load float, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !19442
  %history.i.i.sroa.26.0.copyload = load float, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !19442
  %history.i.i.sroa.29.0.copyload = load float, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !19442
  %history.i.i.sroa.32.0.copyload = load float, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !19442
  %history.i.i.sroa.35.0.copyload = load float, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !19442
  %history.i.i.sroa.38.0.copyload = load float, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !19442
  br i1 %_20.i39.i3552.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i, label %bb5.i.i.lr.ph, !dbg !19444

bb5.i.i.lr.ph:                                    ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit240.i
  %_11.i.i.i.i = load float, ptr %_31, align 4
  %_14.i.i.i.i = load float, ptr %351, align 4
  %_17.i.i.i.i = load float, ptr %352, align 4
  %_20.i.i.i.i = load float, ptr %353, align 4
  %_25.i.i.i.i = load float, ptr %row1.i.i.i72.i, align 4
  %_28.i.i.i.i = load float, ptr %354, align 4
  %_31.i.i.i.i = load float, ptr %355, align 4
  %_34.i.i.i.i = load float, ptr %356, align 4
  %_39.i.i.i.i = load float, ptr %row3.i.i.i86.i, align 4
  %_42.i.i.i.i = load float, ptr %357, align 4
  %_45.i.i.i.i = load float, ptr %358, align 4
  %_48.i.i.i.i = load float, ptr %359, align 4
  %_53.i.i.i.i = load float, ptr %row5.i.i.i100.i, align 4
  %_56.i.i.i.i = load float, ptr %360, align 4
  %_59.i.i.i.i = load float, ptr %361, align 4
  %_62.i.i.i.i = load float, ptr %362, align 4
  %_67.i.i.i.i = load float, ptr %row7.i.i.i114.i, align 4
  %_70.i.i.i.i = load float, ptr %363, align 4
  %_73.i.i.i.i = load float, ptr %364, align 4
  %_76.i.i.i.i = load float, ptr %365, align 4
  %_81.i.i.i.i = load float, ptr %row9.i.i.i128.i, align 4
  %_84.i.i.i.i = load float, ptr %366, align 4
  %_87.i.i.i.i = load float, ptr %367, align 4
  %_90.i.i.i.i = load float, ptr %368, align 4
  %_95.i.i.i.i = load float, ptr %row11.i.i.i142.i, align 4
  %_98.i.i.i.i = load float, ptr %369, align 4
  %_101.i.i.i.i = load float, ptr %370, align 4
  %_104.i.i.i.i = load float, ptr %371, align 4
  %_109.i.i.i.i = load float, ptr %row13.i.i.i156.i, align 4
  %_112.i.i.i.i = load float, ptr %372, align 4
  %_115.i.i.i.i = load float, ptr %373, align 4
  %_118.i.i.i.i = load float, ptr %374, align 4
  %_123.i.i.i.i = load float, ptr %row15.i.i.i170.i, align 4
  %_126.i.i.i.i = load float, ptr %375, align 4
  %_129.i.i.i.i = load float, ptr %376, align 4
  %_132.i.i.i.i = load float, ptr %377, align 4
  %_137.i.i.i.i = load float, ptr %row17.i.i.i184.i, align 4
  %_140.i.i.i.i = load float, ptr %378, align 4
  %_143.i.i.i.i = load float, ptr %379, align 4
  %_146.i.i.i.i = load float, ptr %380, align 4
  %_151.i.i.i.i = load float, ptr %row19.i.i.i198.i, align 4
  %_154.i.i.i.i = load float, ptr %381, align 4
  %_157.i.i.i.i = load float, ptr %382, align 4
  %_160.i.i.i.i = load float, ptr %383, align 4
  %_165.i.i.i.i = load float, ptr %row21.i.i.i212.i, align 4
  %_168.i.i.i.i = load float, ptr %384, align 4
  %_171.i.i.i.i = load float, ptr %385, align 4
  %_174.i.i.i.i = load float, ptr %386, align 4
  br label %bb5.i.i, !dbg !19444

bb5.i.i:                                          ; preds = %bb5.i.i.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230
  %iter.sroa.0.0.i.i3591 = phi i64 [ 0, %bb5.i.i.lr.ph ], [ %425, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230 ]
  %history.i.i.sroa.35.03590 = phi float [ %history.i.i.sroa.35.0.copyload, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.32.03589, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230 ]
  %history.i.i.sroa.32.03589 = phi float [ %history.i.i.sroa.32.0.copyload, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.29.03588, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230 ]
  %history.i.i.sroa.29.03588 = phi float [ %history.i.i.sroa.29.0.copyload, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.26.03587, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230 ]
  %history.i.i.sroa.26.03587 = phi float [ %history.i.i.sroa.26.0.copyload, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.22.03586, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230 ]
  %history.i.i.sroa.22.03586 = phi float [ %history.i.i.sroa.22.0.copyload, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.19.03585, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230 ]
  %history.i.i.sroa.19.03585 = phi float [ %history.i.i.sroa.19.0.copyload, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.16.03584, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230 ]
  %history.i.i.sroa.16.03584 = phi float [ %history.i.i.sroa.16.0.copyload, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.13.03583, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230 ]
  %history.i.i.sroa.13.03583 = phi float [ %history.i.i.sroa.13.0.copyload, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.10.03582, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230 ]
  %history.i.i.sroa.10.03582 = phi float [ %history.i.i.sroa.10.0.copyload, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.7.03581, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230 ]
  %history.i.i.sroa.7.03581 = phi float [ %history.i.i.sroa.7.0.copyload, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.0.03580, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230 ]
  %history.i.i.sroa.0.03580 = phi float [ %history.i.i.sroa.0.0.copyload, %bb5.i.i.lr.ph ], [ %_0.i1228, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230 ]
  %425 = add nuw nsw i64 %iter.sroa.0.0.i.i3591, 1, !dbg !19447
  %_11.i.i = add nuw nsw i64 %iter.sroa.0.0.i.i3591, %iter1.sroa.0.0.i3960, !dbg !19450
  %_24.i.i = icmp ugt i64 %_11.i.i, %right_io.1, !dbg !19451
  br i1 %_24.i.i, label %bb7.i.i, label %bb8.i.i, !dbg !19451, !prof !905

bb8.i.i:                                          ; preds = %bb5.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19454), !dbg !19457
  %_3.not.i1226 = icmp eq i64 %right_io.1, %_11.i.i, !dbg !19458
  br i1 %_3.not.i1226, label %panic.i1229, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230, !dbg !19458

panic.i1229:                                      ; preds = %bb8.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #31, !dbg !19458, !noalias !19460
  unreachable, !dbg !19458

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230: ; preds = %bb8.i.i
  %_31.i.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %_11.i.i, !dbg !19464
  %_0.i1228 = load float, ptr %_31.i.i, align 4, !dbg !19458, !alias.scope !19454, !noalias !19466, !noundef !12
  %426 = tail call noundef float @llvm.fabs.f32(float %history.i.i.sroa.19.03585), !dbg !19467
  %_0.i1121 = fmul float %_0.i1228, %_11.i.i.i.i, !dbg !19470
  %_0.i905 = fadd float %_0.i1121, 0.000000e+00, !dbg !19473
  %_0.i1120 = fmul float %_0.i1228, %_14.i.i.i.i, !dbg !19475
  %_0.i904 = fadd float %_0.i1120, 0.000000e+00, !dbg !19477
  %_0.i1119 = fmul float %_0.i1228, %_17.i.i.i.i, !dbg !19479
  %_0.i903 = fadd float %_0.i1119, 0.000000e+00, !dbg !19481
  %_0.i1118 = fmul float %_0.i1228, %_20.i.i.i.i, !dbg !19483
  %_0.i902 = fadd float %_0.i1118, 0.000000e+00, !dbg !19485
  %_0.i1117 = fmul float %history.i.i.sroa.0.03580, %_25.i.i.i.i, !dbg !19487
  %_0.i901 = fadd float %_0.i905, %_0.i1117, !dbg !19489
  %_0.i1116 = fmul float %history.i.i.sroa.0.03580, %_28.i.i.i.i, !dbg !19491
  %_0.i900 = fadd float %_0.i904, %_0.i1116, !dbg !19493
  %_0.i1115 = fmul float %history.i.i.sroa.0.03580, %_31.i.i.i.i, !dbg !19495
  %_0.i899 = fadd float %_0.i903, %_0.i1115, !dbg !19497
  %_0.i1114 = fmul float %history.i.i.sroa.0.03580, %_34.i.i.i.i, !dbg !19499
  %_0.i898 = fadd float %_0.i902, %_0.i1114, !dbg !19501
  %_0.i1113 = fmul float %history.i.i.sroa.7.03581, %_39.i.i.i.i, !dbg !19503
  %_0.i897 = fadd float %_0.i901, %_0.i1113, !dbg !19505
  %_0.i1112 = fmul float %history.i.i.sroa.7.03581, %_42.i.i.i.i, !dbg !19507
  %_0.i896 = fadd float %_0.i900, %_0.i1112, !dbg !19509
  %_0.i1111 = fmul float %history.i.i.sroa.7.03581, %_45.i.i.i.i, !dbg !19511
  %_0.i895 = fadd float %_0.i899, %_0.i1111, !dbg !19513
  %_0.i1110 = fmul float %history.i.i.sroa.7.03581, %_48.i.i.i.i, !dbg !19515
  %_0.i894 = fadd float %_0.i898, %_0.i1110, !dbg !19517
  %_0.i1109 = fmul float %history.i.i.sroa.10.03582, %_53.i.i.i.i, !dbg !19519
  %_0.i893 = fadd float %_0.i897, %_0.i1109, !dbg !19521
  %_0.i1108 = fmul float %history.i.i.sroa.10.03582, %_56.i.i.i.i, !dbg !19523
  %_0.i892 = fadd float %_0.i896, %_0.i1108, !dbg !19525
  %_0.i1107 = fmul float %history.i.i.sroa.10.03582, %_59.i.i.i.i, !dbg !19527
  %_0.i891 = fadd float %_0.i895, %_0.i1107, !dbg !19529
  %_0.i1106 = fmul float %history.i.i.sroa.10.03582, %_62.i.i.i.i, !dbg !19531
  %_0.i890 = fadd float %_0.i894, %_0.i1106, !dbg !19533
  %_0.i1105 = fmul float %history.i.i.sroa.13.03583, %_67.i.i.i.i, !dbg !19535
  %_0.i889 = fadd float %_0.i893, %_0.i1105, !dbg !19537
  %_0.i1104 = fmul float %history.i.i.sroa.13.03583, %_70.i.i.i.i, !dbg !19539
  %_0.i888 = fadd float %_0.i892, %_0.i1104, !dbg !19541
  %_0.i1103 = fmul float %history.i.i.sroa.13.03583, %_73.i.i.i.i, !dbg !19543
  %_0.i887 = fadd float %_0.i891, %_0.i1103, !dbg !19545
  %_0.i1102 = fmul float %history.i.i.sroa.13.03583, %_76.i.i.i.i, !dbg !19547
  %_0.i886 = fadd float %_0.i890, %_0.i1102, !dbg !19549
  %_0.i1101 = fmul float %history.i.i.sroa.16.03584, %_81.i.i.i.i, !dbg !19551
  %_0.i885 = fadd float %_0.i889, %_0.i1101, !dbg !19553
  %_0.i1100 = fmul float %history.i.i.sroa.16.03584, %_84.i.i.i.i, !dbg !19555
  %_0.i884 = fadd float %_0.i888, %_0.i1100, !dbg !19557
  %_0.i1099 = fmul float %history.i.i.sroa.16.03584, %_87.i.i.i.i, !dbg !19559
  %_0.i883 = fadd float %_0.i887, %_0.i1099, !dbg !19561
  %_0.i1098 = fmul float %history.i.i.sroa.16.03584, %_90.i.i.i.i, !dbg !19563
  %_0.i882 = fadd float %_0.i886, %_0.i1098, !dbg !19565
  %_0.i1097 = fmul float %history.i.i.sroa.19.03585, %_95.i.i.i.i, !dbg !19567
  %_0.i881 = fadd float %_0.i885, %_0.i1097, !dbg !19569
  %_0.i1096 = fmul float %history.i.i.sroa.19.03585, %_98.i.i.i.i, !dbg !19571
  %_0.i880 = fadd float %_0.i884, %_0.i1096, !dbg !19573
  %_0.i1095 = fmul float %history.i.i.sroa.19.03585, %_101.i.i.i.i, !dbg !19575
  %_0.i879 = fadd float %_0.i883, %_0.i1095, !dbg !19577
  %_0.i1094 = fmul float %history.i.i.sroa.19.03585, %_104.i.i.i.i, !dbg !19579
  %_0.i878 = fadd float %_0.i882, %_0.i1094, !dbg !19581
  %_0.i1093 = fmul float %history.i.i.sroa.22.03586, %_109.i.i.i.i, !dbg !19583
  %_0.i877 = fadd float %_0.i881, %_0.i1093, !dbg !19585
  %_0.i1092 = fmul float %history.i.i.sroa.22.03586, %_112.i.i.i.i, !dbg !19587
  %_0.i876 = fadd float %_0.i880, %_0.i1092, !dbg !19589
  %_0.i1091 = fmul float %history.i.i.sroa.22.03586, %_115.i.i.i.i, !dbg !19591
  %_0.i875 = fadd float %_0.i879, %_0.i1091, !dbg !19593
  %_0.i1090 = fmul float %history.i.i.sroa.22.03586, %_118.i.i.i.i, !dbg !19595
  %_0.i874 = fadd float %_0.i878, %_0.i1090, !dbg !19597
  %_0.i1089 = fmul float %history.i.i.sroa.26.03587, %_123.i.i.i.i, !dbg !19599
  %_0.i873 = fadd float %_0.i877, %_0.i1089, !dbg !19601
  %_0.i1088 = fmul float %history.i.i.sroa.26.03587, %_126.i.i.i.i, !dbg !19603
  %_0.i872 = fadd float %_0.i876, %_0.i1088, !dbg !19605
  %_0.i1087 = fmul float %history.i.i.sroa.26.03587, %_129.i.i.i.i, !dbg !19607
  %_0.i871 = fadd float %_0.i875, %_0.i1087, !dbg !19609
  %_0.i1086 = fmul float %history.i.i.sroa.26.03587, %_132.i.i.i.i, !dbg !19611
  %_0.i870 = fadd float %_0.i874, %_0.i1086, !dbg !19613
  %_0.i1085 = fmul float %history.i.i.sroa.29.03588, %_137.i.i.i.i, !dbg !19615
  %_0.i869 = fadd float %_0.i873, %_0.i1085, !dbg !19617
  %_0.i1084 = fmul float %history.i.i.sroa.29.03588, %_140.i.i.i.i, !dbg !19619
  %_0.i868 = fadd float %_0.i872, %_0.i1084, !dbg !19621
  %_0.i1083 = fmul float %history.i.i.sroa.29.03588, %_143.i.i.i.i, !dbg !19623
  %_0.i867 = fadd float %_0.i871, %_0.i1083, !dbg !19625
  %_0.i1082 = fmul float %history.i.i.sroa.29.03588, %_146.i.i.i.i, !dbg !19627
  %_0.i866 = fadd float %_0.i870, %_0.i1082, !dbg !19629
  %_0.i1081 = fmul float %history.i.i.sroa.32.03589, %_151.i.i.i.i, !dbg !19631
  %_0.i865 = fadd float %_0.i869, %_0.i1081, !dbg !19633
  %_0.i1080 = fmul float %history.i.i.sroa.32.03589, %_154.i.i.i.i, !dbg !19635
  %_0.i864 = fadd float %_0.i868, %_0.i1080, !dbg !19637
  %_0.i1079 = fmul float %history.i.i.sroa.32.03589, %_157.i.i.i.i, !dbg !19639
  %_0.i863 = fadd float %_0.i867, %_0.i1079, !dbg !19641
  %_0.i1078 = fmul float %history.i.i.sroa.32.03589, %_160.i.i.i.i, !dbg !19643
  %_0.i862 = fadd float %_0.i866, %_0.i1078, !dbg !19645
  %_0.i1077 = fmul float %history.i.i.sroa.35.03590, %_165.i.i.i.i, !dbg !19647
  %_0.i861 = fadd float %_0.i865, %_0.i1077, !dbg !19649
  %_0.i1076 = fmul float %history.i.i.sroa.35.03590, %_168.i.i.i.i, !dbg !19651
  %_0.i860 = fadd float %_0.i864, %_0.i1076, !dbg !19653
  %_0.i1075 = fmul float %history.i.i.sroa.35.03590, %_171.i.i.i.i, !dbg !19655
  %_0.i859 = fadd float %_0.i863, %_0.i1075, !dbg !19657
  %_0.i1074 = fmul float %history.i.i.sroa.35.03590, %_174.i.i.i.i, !dbg !19659
  %_0.i858 = fadd float %_0.i862, %_0.i1074, !dbg !19661
  %427 = tail call noundef float @llvm.fabs.f32(float %_0.i861), !dbg !19663
  %_3.i.i1618.inv = fcmp ogt float %426, %427, !dbg !19665
  %_4.i.i1625.v = select i1 %_3.i.i1618.inv, float %426, float %427, !dbg !19665
  %428 = tail call noundef float @llvm.fabs.f32(float %_0.i860), !dbg !19663
  %_3.i.i1618.inv.1 = fcmp ogt float %_4.i.i1625.v, %428, !dbg !19665
  %_4.i.i1625.v.1 = select i1 %_3.i.i1618.inv.1, float %_4.i.i1625.v, float %428, !dbg !19665
  %429 = tail call noundef float @llvm.fabs.f32(float %_0.i859), !dbg !19663
  %_3.i.i1618.inv.2 = fcmp ogt float %_4.i.i1625.v.1, %429, !dbg !19665
  %_4.i.i1625.v.2 = select i1 %_3.i.i1618.inv.2, float %_4.i.i1625.v.1, float %429, !dbg !19665
  %430 = tail call noundef float @llvm.fabs.f32(float %_0.i858), !dbg !19663
  %_3.i.i1618.inv.3 = fcmp ogt float %_4.i.i1625.v.2, %430, !dbg !19665
  %_4.i.i1625.v.3 = select i1 %_3.i.i1618.inv.3, float %_4.i.i1625.v.2, float %430, !dbg !19665
  %_39.i.i = getelementptr inbounds nuw float, ptr %peaks_right.i, i64 %iter.sroa.0.0.i.i3591, !dbg !19668
  store float %_4.i.i1625.v.3, ptr %_39.i.i, align 4, !dbg !19673, !alias.scope !19675, !noalias !19466
  %exitcond4893.not = icmp eq i64 %425, %umax4892, !dbg !19678
  br i1 %exitcond4893.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i, label %bb5.i.i, !dbg !19444

bb7.i.i:                                          ; preds = %bb5.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_11.i.i, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e0592aef22128a0ac53753b9632a8183) #31, !dbg !19680, !noalias !19466
  unreachable, !dbg !19680

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit240.i
  %history.i.i.sroa.0.0.lcssa = phi float [ %history.i.i.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit240.i ], [ %_0.i1228, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230 ], !dbg !19681
  %history.i.i.sroa.7.0.lcssa = phi float [ %history.i.i.sroa.7.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit240.i ], [ %history.i.i.sroa.0.03580, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230 ], !dbg !19681
  %history.i.i.sroa.10.0.lcssa = phi float [ %history.i.i.sroa.10.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit240.i ], [ %history.i.i.sroa.7.03581, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230 ], !dbg !19681
  %history.i.i.sroa.13.0.lcssa = phi float [ %history.i.i.sroa.13.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit240.i ], [ %history.i.i.sroa.10.03582, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230 ], !dbg !19681
  %history.i.i.sroa.16.0.lcssa = phi float [ %history.i.i.sroa.16.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit240.i ], [ %history.i.i.sroa.13.03583, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230 ], !dbg !19681
  %history.i.i.sroa.19.0.lcssa = phi float [ %history.i.i.sroa.19.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit240.i ], [ %history.i.i.sroa.16.03584, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230 ], !dbg !19681
  %history.i.i.sroa.22.0.lcssa = phi float [ %history.i.i.sroa.22.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit240.i ], [ %history.i.i.sroa.19.03585, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230 ], !dbg !19681
  %history.i.i.sroa.26.0.lcssa = phi float [ %history.i.i.sroa.26.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit240.i ], [ %history.i.i.sroa.22.03586, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230 ], !dbg !19681
  %history.i.i.sroa.29.0.lcssa = phi float [ %history.i.i.sroa.29.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit240.i ], [ %history.i.i.sroa.26.03587, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230 ], !dbg !19681
  %history.i.i.sroa.32.0.lcssa = phi float [ %history.i.i.sroa.32.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit240.i ], [ %history.i.i.sroa.29.03588, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230 ], !dbg !19681
  %history.i.i.sroa.35.0.lcssa = phi float [ %history.i.i.sroa.35.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit240.i ], [ %history.i.i.sroa.32.03589, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230 ], !dbg !19681
  %history.i.i.sroa.38.0.lcssa = phi float [ %history.i.i.sroa.38.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit240.i ], [ %history.i.i.sroa.35.03590, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1230 ], !dbg !19681
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !19682
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !19682
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !19682
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !19682
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !19682
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !19682
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !19682
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !19682
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !19682
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !19682
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !19682
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !19682
  br i1 %_20.i39.i3552.not, label %bb27.i.loopexit, label %bb32.i.lr.ph, !dbg !19188

bb32.i.lr.ph:                                     ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i
  %_72.i.sroa.3.0.copyload = load i64, ptr %_72.i.sroa.3.0..sroa_idx, align 8, !noalias !19127
  %_72.i.sroa.4.0.copyload = load i64, ptr %_72.i.sroa.4.0..sroa_idx, align 8, !noalias !19127
  %_73.i.sroa.3.0.copyload = load i64, ptr %_73.i.sroa.3.0..sroa_idx, align 8, !noalias !19127
  %_73.i.sroa.4.0.copyload = load i64, ptr %_73.i.sroa.4.0..sroa_idx, align 8, !noalias !19127
  %_54.0.i253.i = load ptr, ptr %uniform_left.i, align 8, !nonnull !12, !align !9542
  %_54.1.i254.i = load i64, ptr %401, align 8
  %_18.i264.i = load i64, ptr %387, align 8
  %_56.0.i275.i = load ptr, ptr %402, align 8, !nonnull !12, !align !9542
  %_56.1.i276.i = load i64, ptr %403, align 8
  %_58.1.i301.i = load i64, ptr %407, align 8
  %_58.0.i300.i = load ptr, ptr %408, align 8, !nonnull !12, !align !9542
  %_54.0.i.i = load ptr, ptr %uniform_right.i, align 8, !nonnull !12, !align !9542
  %_54.1.i.i = load i64, ptr %409, align 8
  %_18.i.i = load i64, ptr %388, align 8
  %_56.0.i.i = load ptr, ptr %410, align 8, !nonnull !12, !align !9542
  %_56.1.i.i = load i64, ptr %411, align 8
  %_58.1.i.i = load i64, ptr %415, align 8
  %_58.0.i.i = load ptr, ptr %416, align 8, !nonnull !12, !align !9542
  %_22.i268.i.promoted3879 = load i32, ptr %_22.i268.i, align 4
  %_21.i267.i.promoted3915 = load float, ptr %_21.i267.i, align 4
  %_22.i.i.promoted3918 = load i32, ptr %_22.i.i, align 4
  %_21.i.i.promoted3954 = load float, ptr %_21.i.i, align 4
  %umax4894 = call i64 @llvm.umax.i64(i64 %_18.i264.i, i64 1), !dbg !19188
  %umax4896 = call i64 @llvm.umax.i64(i64 %_18.i.i, i64 1), !dbg !19188
  br label %bb32.i, !dbg !19188

bb32.i:                                           ; preds = %bb32.i.lr.ph, %bb44.i
  %running.sroa.0.0.i.lcssa51795922 = phi float [ %_21.i.i.promoted3954, %bb32.i.lr.ph ], [ %running.sroa.0.0.i.lcssa51795921, %bb44.i ]
  %running.sroa.0.0.i294.lcssa51425905 = phi float [ %_21.i267.i.promoted3915, %bb32.i.lr.ph ], [ %running.sroa.0.0.i294.lcssa51425904, %bb44.i ]
  %running.sroa.0.0.i.lcssa38643956 = phi float [ %_21.i.i.promoted3954, %bb32.i.lr.ph ], [ %running.sroa.0.0.i.lcssa38643955, %bb44.i ]
  %storemerge.i.lcssa38373920 = phi i32 [ %_22.i.i.promoted3918, %bb32.i.lr.ph ], [ %storemerge.i.lcssa38373919, %bb44.i ]
  %running.sroa.0.0.i294.lcssa38063917 = phi float [ %_21.i267.i.promoted3915, %bb32.i.lr.ph ], [ %running.sroa.0.0.i294.lcssa38063916, %bb44.i ]
  %storemerge.i299.lcssa37793881 = phi i32 [ %_22.i268.i.promoted3879, %bb32.i.lr.ph ], [ %storemerge.i299.lcssa37793880, %bb44.i ]
  %frame.sroa.0.0.i3876 = phi i64 [ 0, %bb32.i.lr.ph ], [ %_87.i, %bb44.i ]
  %main_cursor.sroa.0.1.i3875 = phi i64 [ %main_cursor.sroa.0.0.i3959, %bb32.i.lr.ph ], [ %main_cursor.sroa.0.2.i, %bb44.i ]
  %ring_cursor.sroa.0.1.i3874 = phi i64 [ %ring_cursor.sroa.0.0.i3958, %bb32.i.lr.ph ], [ %ring_cursor.sroa.0.2.i, %bb44.i ]
  %_70.i = sub nuw nsw i64 %..i1967, %frame.sroa.0.0.i3876, !dbg !19683
  %ring.i566 = load i64, ptr %345, align 8, !dbg !19684, !alias.scope !19686, !noalias !19689, !noundef !12
  %main.i567 = load i64, ptr %346, align 8, !dbg !19693, !alias.scope !19686, !noalias !19689, !noundef !12
  %_10.i = add i64 %ring_cursor.sroa.0.1.i3874, 1, !dbg !19694
  %_45.not.i = icmp ult i64 %_10.i, %ring.i566, !dbg !19695
  %431 = select i1 %_45.not.i, i64 0, i64 %ring.i566, !dbg !19695
  %start1.sroa.0.0.i568 = sub nuw i64 %_10.i, %431, !dbg !19695
  %_12.i569 = add i64 %_72.i.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i3874, !dbg !19697
  %_46.not.i = icmp ult i64 %_12.i569, %ring.i566, !dbg !19698
  %432 = select i1 %_46.not.i, i64 0, i64 %ring.i566, !dbg !19698
  %left_end.sroa.0.0.i = sub nuw i64 %_12.i569, %432, !dbg !19698
  %_15.i571 = add i64 %_73.i.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i3874, !dbg !19700
  %_47.not.i = icmp ult i64 %_15.i571, %ring.i566, !dbg !19701
  %433 = select i1 %_47.not.i, i64 0, i64 %ring.i566, !dbg !19701
  %right_end.sroa.0.0.i = sub nuw i64 %_15.i571, %433, !dbg !19701
  %_18.i = add i64 %_72.i.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i3874, !dbg !19703
  %_48.not.i = icmp ult i64 %_18.i, %ring.i566, !dbg !19704
  %434 = select i1 %_48.not.i, i64 0, i64 %ring.i566, !dbg !19704
  %left_expiring.sroa.0.0.i = sub nuw i64 %_18.i, %434, !dbg !19704
  %_21.i = add i64 %_73.i.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i3874, !dbg !19706
  %_49.not.i = icmp ult i64 %_21.i, %ring.i566, !dbg !19707
  %435 = select i1 %_49.not.i, i64 0, i64 %ring.i566, !dbg !19707
  %right_expiring.sroa.0.0.i = sub nuw i64 %_21.i, %435, !dbg !19707
  %_30.i573 = sub i64 %ring.i566, %ring_cursor.sroa.0.1.i3874, !dbg !19709
  %..i1992 = tail call noundef i64 @llvm.umin.i64(i64 %_30.i573, i64 %_70.i), !dbg !19710
  %_31.i = sub i64 %main.i567, %main_cursor.sroa.0.1.i3875, !dbg !19712
  %..i1993 = tail call noundef i64 @llvm.umin.i64(i64 %_31.i, i64 %..i1992), !dbg !19713
  %_32.i576 = sub i64 %ring.i566, %start1.sroa.0.0.i568, !dbg !19715
  %..i1994 = tail call noundef i64 @llvm.umin.i64(i64 %_32.i576, i64 %..i1993), !dbg !19716
  %_34.i578 = sub i64 %ring.i566, %left_end.sroa.0.0.i, !dbg !19718
  %..i1995 = tail call noundef i64 @llvm.umin.i64(i64 %_34.i578, i64 %..i1994), !dbg !19719
  %_36.i = sub i64 %ring.i566, %right_end.sroa.0.0.i, !dbg !19721
  %..i1996 = tail call noundef i64 @llvm.umin.i64(i64 %_36.i, i64 %..i1995), !dbg !19722
  %_38.i580 = sub i64 %ring.i566, %left_expiring.sroa.0.0.i, !dbg !19724
  %..i1997 = tail call noundef i64 @llvm.umin.i64(i64 %_38.i580, i64 %..i1996), !dbg !19725
  %_40.i581 = sub i64 %ring.i566, %right_expiring.sroa.0.0.i, !dbg !19727
  %..i1998 = tail call noundef i64 @llvm.umin.i64(i64 %_40.i581, i64 %..i1997), !dbg !19728
  %_76.i = add i64 %frame.sroa.0.0.i3876, %iter1.sroa.0.0.i3960, !dbg !19730
  %_80.i = add i64 %..i1998, %_76.i, !dbg !19731
  %_184.i = icmp ult i64 %_80.i, %_76.i, !dbg !19732
  %_178.not.i = icmp ugt i64 %_80.i, %left_io.1
  %or.cond.i = or i1 %_184.i, %_178.not.i, !dbg !19732
  br i1 %or.cond.i, label %bb72.i, label %bb70.i, !dbg !19732, !prof !5262

bb72.i:                                           ; preds = %bb32.i
  store float %running.sroa.0.0.i294.lcssa51425905, ptr %_21.i267.i, align 1, !dbg !19155
  store float %running.sroa.0.0.i.lcssa51795922, ptr %_21.i.i, align 1, !dbg !19185
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i, i64 noundef %_80.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_14e3d3ba493695bccf4dcde9be821bfc) #31, !dbg !19739, !noalias !18994
  unreachable, !dbg !19739

bb70.i:                                           ; preds = %bb32.i
  %_187.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %_76.i, !dbg !19740
  %_188.not.i = icmp ugt i64 %_80.i, %right_io.1, !dbg !19744
  br i1 %_188.not.i, label %bb75.i, label %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit, !dbg !19744, !prof !905

bb75.i:                                           ; preds = %bb70.i
  store float %running.sroa.0.0.i294.lcssa51425905, ptr %_21.i267.i, align 1, !dbg !19155
  store float %running.sroa.0.0.i.lcssa51795922, ptr %_21.i.i, align 1, !dbg !19185
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i, i64 noundef %_80.i, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8706d23ef0096dbeb31c5991bb003668) #31, !dbg !19748, !noalias !18994
  unreachable, !dbg !19748

_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit: ; preds = %bb70.i
  %_195.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %_76.i, !dbg !19749
  %_87.i = add nuw nsw i64 %..i1998, %frame.sroa.0.0.i3876, !dbg !19753
  %_204.i = getelementptr inbounds nuw float, ptr %peaks_left.i, i64 %frame.sroa.0.0.i3876, !dbg !19754
  %_213.i = getelementptr inbounds nuw float, ptr %peaks_right.i, i64 %frame.sroa.0.0.i3876, !dbg !19764
  %_2.i.i.i3610.not = icmp eq i64 %..i1998, 0, !dbg !19773
  br i1 %_2.i.i.i3610.not, label %bb44.i, label %bb43.i.lr.ph, !dbg !19773

bb43.i.lr.ph:                                     ; preds = %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit
  %_13.i67022962298 = load float, ptr %391, align 4
  %_13.i65822992301 = load float, ptr %394, align 4
  %_13.i64623022304 = load float, ptr %397, align 4
  %_13.i63423052307 = load float, ptr %400, align 4
  %_37.i288.i = load float, ptr %405, align 4
  %_37.i.i = load float, ptr %413, align 4
  %.promoted3613 = load float, ptr %389, align 4
  %_115.i.promoted = load float, ptr %_115.i, align 4
  %.promoted3646 = load float, ptr %390, align 4
  %.promoted3677 = load float, ptr %392, align 4
  %_117.i.promoted = load float, ptr %_117.i, align 4
  %.promoted3682 = load float, ptr %393, align 4
  %.promoted3685 = load float, ptr %395, align 4
  %_119.i.promoted = load float, ptr %_119.i, align 4
  %.promoted3718 = load float, ptr %396, align 4
  %.promoted3749 = load float, ptr %398, align 4
  %_121.i.promoted = load float, ptr %_121.i, align 4
  %.promoted3754 = load float, ptr %399, align 4
  %.promoted3811 = load float, ptr %404, align 4
  %.promoted3813 = load float, ptr %406, align 4
  %.promoted3869 = load float, ptr %412, align 4
  %.promoted3871 = load float, ptr %414, align 4
  %umin4900 = call i64 @llvm.umin.i64(i64 %_34.i578, i64 %_36.i), !dbg !19773
  %umin4901 = call i64 @llvm.umin.i64(i64 %umin4900, i64 %_38.i580), !dbg !19773
  %umin4902 = call i64 @llvm.umin.i64(i64 %umin4901, i64 %_40.i581), !dbg !19773
  %umin4903 = call i64 @llvm.umin.i64(i64 %umin4902, i64 %_32.i576), !dbg !19773
  %umin4904 = call i64 @llvm.umin.i64(i64 %umin4903, i64 %_30.i573), !dbg !19773
  %umin4905 = call i64 @llvm.umin.i64(i64 %umin4904, i64 %_31.i), !dbg !19773
  %436 = sub nsw i64 %umin4906, %frame.sroa.0.0.i3876, !dbg !19773
  %umin4907 = call i64 @llvm.umin.i64(i64 %umin4905, i64 %436), !dbg !19773
  %_115.i.promoted5802 = load float, ptr %_115.i, align 1
  %.promoted5819 = load float, ptr %392, align 1
  %_117.i.promoted5836 = load float, ptr %_117.i, align 1
  %_119.i.promoted5853 = load float, ptr %_119.i, align 1
  %.promoted5870 = load float, ptr %398, align 1
  %_121.i.promoted5887 = load float, ptr %_121.i, align 1
  br label %bb43.i, !dbg !19773

bb43.i:                                           ; preds = %bb43.i.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310
  %_0.i13835888 = phi float [ %_121.i.promoted5887, %bb43.i.lr.ph ], [ %_0.i13835889, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %_0.i.i15375871 = phi float [ %.promoted5870, %bb43.i.lr.ph ], [ %_0.i.i15375872, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %_0.i13705854 = phi float [ %_119.i.promoted5853, %bb43.i.lr.ph ], [ %_0.i13705855, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %_0.i13575837 = phi float [ %_117.i.promoted5836, %bb43.i.lr.ph ], [ %_0.i13575838, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %_0.i.i15235820 = phi float [ %.promoted5819, %bb43.i.lr.ph ], [ %_0.i.i15235821, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %_0.i13445803 = phi float [ %_115.i.promoted5802, %bb43.i.lr.ph ], [ %_0.i13445804, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %_0.i13313872 = phi float [ %.promoted3871, %bb43.i.lr.ph ], [ %_0.i1331, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %_0.i11473870 = phi float [ %.promoted3869, %bb43.i.lr.ph ], [ %_0.i1147, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %running.sroa.0.0.i3842 = phi float [ %running.sroa.0.0.i.lcssa38643956, %bb43.i.lr.ph ], [ %running.sroa.0.0.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %storemerge.i3815 = phi i32 [ %storemerge.i.lcssa38373920, %bb43.i.lr.ph ], [ %storemerge.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %_0.i13353814 = phi float [ %.promoted3813, %bb43.i.lr.ph ], [ %_0.i1335, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %_0.i11513812 = phi float [ %.promoted3811, %bb43.i.lr.ph ], [ %_0.i1151, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %running.sroa.0.0.i2943784 = phi float [ %running.sroa.0.0.i294.lcssa38063917, %bb43.i.lr.ph ], [ %running.sroa.0.0.i294, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %storemerge.i2993757 = phi i32 [ %storemerge.i299.lcssa37793881, %bb43.i.lr.ph ], [ %storemerge.i299, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %_12.i6323756 = phi float [ %.promoted3754, %bb43.i.lr.ph ], [ %_12.i6323755, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %_0.i13833753 = phi float [ %_121.i.promoted, %bb43.i.lr.ph ], [ %_0.i13833752, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %_5.i6263751 = phi float [ %.promoted3749, %bb43.i.lr.ph ], [ %_5.i6263750, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %_12.i6443720 = phi float [ %.promoted3718, %bb43.i.lr.ph ], [ %_12.i6443719, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %_0.i13703717 = phi float [ %_119.i.promoted, %bb43.i.lr.ph ], [ %_0.i13703716, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %_5.i6383687 = phi float [ %.promoted3685, %bb43.i.lr.ph ], [ %_5.i6383686, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %_12.i6563684 = phi float [ %.promoted3682, %bb43.i.lr.ph ], [ %_12.i6563683, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %_0.i13573681 = phi float [ %_117.i.promoted, %bb43.i.lr.ph ], [ %_0.i13573680, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %_5.i6503679 = phi float [ %.promoted3677, %bb43.i.lr.ph ], [ %_5.i6503678, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %_12.i6683648 = phi float [ %.promoted3646, %bb43.i.lr.ph ], [ %_12.i6683647, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %_0.i13443645 = phi float [ %_115.i.promoted, %bb43.i.lr.ph ], [ %_0.i13443644, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %_5.i6623615 = phi float [ %.promoted3613, %bb43.i.lr.ph ], [ %_5.i6623614, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %iter.i.sroa.41.03612 = phi i64 [ 0, %bb43.i.lr.ph ], [ %_9.0.i2046, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310 ]
  %_9.0.i2046 = add nuw i64 %iter.i.sroa.41.03612, 1, !dbg !19777
  %data.i.i.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_187.i, i64 %iter.i.sroa.41.03612, !dbg !19780
  %data.i.i.i.i2044 = getelementptr inbounds nuw float, ptr %_213.i, i64 %iter.i.sroa.41.03612, !dbg !19787
  %data.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_204.i, i64 %iter.i.sroa.41.03612, !dbg !19790
  %data.i5.i.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_195.i, i64 %iter.i.sroa.41.03612, !dbg !19793
  br i1 %stationary.sroa.0.0.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1235, label %bb46.i, !dbg !19796

bb41.i.bb44.i_crit_edge:                          ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310
  store float %_0.i13445804, ptr %_115.i, align 1, !dbg !19797
  store float %_0.i.i15235821, ptr %392, align 1, !dbg !19799
  store float %_0.i13575838, ptr %_117.i, align 1, !dbg !19801
  store float %_0.i13705855, ptr %_119.i, align 1, !dbg !19802
  store float %_0.i.i15375872, ptr %398, align 1, !dbg !19804
  store float %_0.i13835889, ptr %_121.i, align 1, !dbg !19806
  store float %_5.i6623614, ptr %389, align 4, !dbg !19807
  store float %_12.i6683647, ptr %390, align 4, !dbg !19808
  store float %_5.i6383686, ptr %395, align 4, !dbg !19809
  store float %_12.i6443719, ptr %396, align 4, !dbg !19810
  br label %bb44.i, !dbg !19773

bb44.i:                                           ; preds = %bb41.i.bb44.i_crit_edge, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit
  %running.sroa.0.0.i.lcssa51795921 = phi float [ %running.sroa.0.0.i, %bb41.i.bb44.i_crit_edge ], [ %running.sroa.0.0.i.lcssa51795922, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %running.sroa.0.0.i294.lcssa51425904 = phi float [ %running.sroa.0.0.i294, %bb41.i.bb44.i_crit_edge ], [ %running.sroa.0.0.i294.lcssa51425905, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %running.sroa.0.0.i.lcssa38643955 = phi float [ %running.sroa.0.0.i, %bb41.i.bb44.i_crit_edge ], [ %running.sroa.0.0.i.lcssa38643956, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %storemerge.i.lcssa38373919 = phi i32 [ %storemerge.i, %bb41.i.bb44.i_crit_edge ], [ %storemerge.i.lcssa38373920, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %running.sroa.0.0.i294.lcssa38063916 = phi float [ %running.sroa.0.0.i294, %bb41.i.bb44.i_crit_edge ], [ %running.sroa.0.0.i294.lcssa38063917, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %storemerge.i299.lcssa37793880 = phi i32 [ %storemerge.i299, %bb41.i.bb44.i_crit_edge ], [ %storemerge.i299.lcssa37793881, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_144.i = add i64 %..i1998, %ring_cursor.sroa.0.1.i3874, !dbg !19811
  %_214.not.i = icmp ult i64 %_144.i, %ring.i, !dbg !19812
  %437 = select i1 %_214.not.i, i64 0, i64 %ring.i, !dbg !19812
  %ring_cursor.sroa.0.2.i = sub nuw i64 %_144.i, %437, !dbg !19812
  %_146.i = add i64 %..i1998, %main_cursor.sroa.0.1.i3875, !dbg !19815
  %_225.not.i = icmp ult i64 %_146.i, %main.i, !dbg !19816
  %438 = select i1 %_225.not.i, i64 0, i64 %main.i, !dbg !19816
  %main_cursor.sroa.0.2.i = sub nuw i64 %_146.i, %438, !dbg !19816
  %_65.i = icmp ult i64 %_87.i, %..i1967, !dbg !19188
  br i1 %_65.i, label %bb32.i, label %bb31.i.bb27.i.loopexit_crit_edge, !dbg !19188

bb46.i:                                           ; preds = %bb43.i
  %_0.i1128 = fadd float %_5.i6623615, -1.000000e+00, !dbg !19818
  %_3.i.i = fcmp ogt float %_0.i1128, 0.000000e+00, !dbg !19820
  %_0.i.i = select i1 %_3.i.i, float %_0.i1128, float 0.000000e+00, !dbg !19823
  %_0.i704 = fadd float %_0.i13443645, %_12.i6683648, !dbg !19825
  %_0.i1344 = select i1 %_3.i.i, float %_0.i704, float %_13.i67022962298, !dbg !19827
  %_0.i1339 = select i1 %_3.i.i, float %_12.i6683648, float 0.000000e+00, !dbg !19829
  %_0.i1129 = fadd float %_5.i6503679, -1.000000e+00, !dbg !19831
  %_3.i.i1517 = fcmp ogt float %_0.i1129, 0.000000e+00, !dbg !19833
  %_0.i.i1523 = select i1 %_3.i.i1517, float %_0.i1129, float 0.000000e+00, !dbg !19836
  %_0.i705 = fadd float %_0.i13573681, %_12.i6563684, !dbg !19838
  %_0.i1357 = select i1 %_3.i.i1517, float %_0.i705, float %_13.i65822992301, !dbg !19840
  %_0.i1350 = select i1 %_3.i.i1517, float %_12.i6563684, float 0.000000e+00, !dbg !19842
  store float %_0.i1350, ptr %393, align 4, !dbg !19844
  %_0.i1130 = fadd float %_5.i6383687, -1.000000e+00, !dbg !19845
  %_3.i.i1524 = fcmp ogt float %_0.i1130, 0.000000e+00, !dbg !19847
  %_0.i.i1530 = select i1 %_3.i.i1524, float %_0.i1130, float 0.000000e+00, !dbg !19850
  %_0.i706 = fadd float %_0.i13703717, %_12.i6443720, !dbg !19852
  %_0.i1370 = select i1 %_3.i.i1524, float %_0.i706, float %_13.i64623022304, !dbg !19854
  %_0.i1363 = select i1 %_3.i.i1524, float %_12.i6443720, float 0.000000e+00, !dbg !19856
  %_0.i1131 = fadd float %_5.i6263751, -1.000000e+00, !dbg !19858
  %_3.i.i1531 = fcmp ogt float %_0.i1131, 0.000000e+00, !dbg !19860
  %_0.i.i1537 = select i1 %_3.i.i1531, float %_0.i1131, float 0.000000e+00, !dbg !19863
  %_0.i707 = fadd float %_0.i13833753, %_12.i6323756, !dbg !19865
  %_0.i1383 = select i1 %_3.i.i1531, float %_0.i707, float %_13.i63423052307, !dbg !19867
  %_0.i1376 = select i1 %_3.i.i1531, float %_12.i6323756, float 0.000000e+00, !dbg !19869
  store float %_0.i1376, ptr %399, align 4, !dbg !19871
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1235, !dbg !19872

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1235: ; preds = %bb43.i, %bb46.i
  %_0.i13835889 = phi float [ %_0.i1383, %bb46.i ], [ %_0.i13835888, %bb43.i ]
  %_0.i.i15375872 = phi float [ %_0.i.i1537, %bb46.i ], [ %_0.i.i15375871, %bb43.i ]
  %_0.i13705855 = phi float [ %_0.i1370, %bb46.i ], [ %_0.i13705854, %bb43.i ]
  %_0.i13575838 = phi float [ %_0.i1357, %bb46.i ], [ %_0.i13575837, %bb43.i ]
  %_0.i.i15235821 = phi float [ %_0.i.i1523, %bb46.i ], [ %_0.i.i15235820, %bb43.i ]
  %_0.i13445804 = phi float [ %_0.i1344, %bb46.i ], [ %_0.i13445803, %bb43.i ]
  %_12.i6323755 = phi float [ %_0.i1376, %bb46.i ], [ %_12.i6323756, %bb43.i ]
  %_0.i13833752 = phi float [ %_0.i1383, %bb46.i ], [ %_0.i13833753, %bb43.i ]
  %_5.i6263750 = phi float [ %_0.i.i1537, %bb46.i ], [ %_5.i6263751, %bb43.i ]
  %_12.i6443719 = phi float [ %_0.i1363, %bb46.i ], [ %_12.i6443720, %bb43.i ]
  %_0.i13703716 = phi float [ %_0.i1370, %bb46.i ], [ %_0.i13703717, %bb43.i ]
  %_5.i6383686 = phi float [ %_0.i.i1530, %bb46.i ], [ %_5.i6383687, %bb43.i ]
  %_12.i6563683 = phi float [ %_0.i1350, %bb46.i ], [ %_12.i6563684, %bb43.i ]
  %_0.i13573680 = phi float [ %_0.i1357, %bb46.i ], [ %_0.i13573681, %bb43.i ]
  %_5.i6503678 = phi float [ %_0.i.i1523, %bb46.i ], [ %_5.i6503679, %bb43.i ]
  %_12.i6683647 = phi float [ %_0.i1339, %bb46.i ], [ %_12.i6683648, %bb43.i ]
  %_0.i13443644 = phi float [ %_0.i1344, %bb46.i ], [ %_0.i13443645, %bb43.i ]
  %_5.i6623614 = phi float [ %_0.i.i, %bb46.i ], [ %_5.i6623615, %bb43.i ]
  %_0.i1248 = load float, ptr %data.i.i.i.i.i.i, align 4, !dbg !19873, !alias.scope !19875, !noalias !18994, !noundef !12
  %_0.i1243 = load float, ptr %data.i.i.i.i2044, align 4, !dbg !19878, !alias.scope !19880, !noalias !18994, !noundef !12
  %_3.i.i1645 = fcmp ule float %_0.i1243, %_0.i1248, !dbg !19883
  %_6.i.i1647 = bitcast float %_0.i1243 to i32, !dbg !19886
  %_8.i.i1649 = bitcast float %_0.i1248 to i32, !dbg !19889
  %_4.i.i1652 = select i1 %_3.i.i1645, i32 %_8.i.i1649, i32 %_6.i.i1647, !dbg !19891
  %_5.i1510 = and i32 %_4.i.i1652, %.none.i, !dbg !19892
  %_7.i1506 = and i32 %_9.i1512, %_6.i.i1647, !dbg !19894
  %_4.i1507 = or disjoint i32 %_5.i1510, %_7.i1506, !dbg !19896
  %_0.i1508 = bitcast i32 %_4.i1507 to float, !dbg !19897
  %_0.i1238 = load float, ptr %data.i.i.i.i.i.i.i.i, align 4, !dbg !19899, !alias.scope !19901, !noalias !18994, !noundef !12
  %_0.i1233 = load float, ptr %data.i5.i.i.i.i.i.i.i, align 4, !dbg !19904, !alias.scope !19906, !noalias !18994, !noundef !12
  %_215.i = add nuw i64 %iter.i.sroa.41.03612, %ring_cursor.sroa.0.1.i3874, !dbg !19909
  %_216.i = add nuw i64 %iter.i.sroa.41.03612, %main_cursor.sroa.0.1.i3875, !dbg !19912
  %_217.i = add nuw i64 %iter.i.sroa.41.03612, %left_end.sroa.0.0.i, !dbg !19913
  %_218.i = add i64 %iter.i.sroa.41.03612, %start1.sroa.0.0.i568, !dbg !19914
  %_219.i = add nuw i64 %iter.i.sroa.41.03612, %left_expiring.sroa.0.0.i, !dbg !19915
  %_7.i8.i256.i = add i64 %_215.i, 1, !dbg !19916
  %or.cond.i11.i259.i.not = icmp ult i64 %_215.i, %_54.1.i254.i, !dbg !19920
  br i1 %or.cond.i11.i259.i.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i260.i, label %bb4.i13.i315.i, !dbg !19920, !prof !9914

bb4.i13.i315.i:                                   ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1235
  store float %running.sroa.0.0.i294.lcssa51425905, ptr %_21.i267.i, align 1, !dbg !19155
  store float %running.sroa.0.0.i.lcssa51795922, ptr %_21.i.i, align 1, !dbg !19185
  store float %_0.i13445804, ptr %_115.i, align 1, !dbg !19797
  store float %_0.i.i15235821, ptr %392, align 1, !dbg !19799
  store float %_0.i13575838, ptr %_117.i, align 1, !dbg !19801
  store float %_0.i13705855, ptr %_119.i, align 1, !dbg !19802
  store float %_0.i.i15375872, ptr %398, align 1, !dbg !19804
  store float %_0.i13835889, ptr %_121.i, align 1, !dbg !19806
  %umax4898 = call i64 @llvm.umax.i64(i64 %ring_cursor.sroa.0.1.i3874, i64 %_54.1.i254.i), !dbg !19773
  %439 = add i64 %umax4898, 1, !dbg !19773
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_215.i, i64 noundef %439, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i254.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cd0e502fea74c9eb9984d521d7f3533e) #31, !dbg !19927, !noalias !19928
  unreachable, !dbg !19927

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i260.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1235
  %_7.i1513 = and i32 %_9.i1512, %_8.i.i1649, !dbg !19936
  %_4.i1514 = or disjoint i32 %_5.i1510, %_7.i1513, !dbg !19892
  %_0.i1515 = bitcast i32 %_4.i1514 to float, !dbg !19937
  %_0.i915 = fdiv float %_0.i13443644, %_0.i1515, !dbg !19939
  %_3.i694 = fcmp uge float %_0.i13443644, %_0.i1515, !dbg !19941
  %_0.i1501 = select i1 %_3.i694, float 1.000000e+00, float %_0.i915, !dbg !19943
  %_17.i12.i261.i = getelementptr inbounds nuw float, ptr %_54.0.i253.i, i64 %_215.i, !dbg !19945
  store float %_0.i1501, ptr %_17.i12.i261.i, align 4, !dbg !19949, !alias.scope !19951, !noalias !19954
  %or.cond.i345.not = icmp ult i64 %_217.i, %_54.1.i254.i, !dbg !19955
  br i1 %or.cond.i345.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit350, label %bb4.i349, !dbg !19955, !prof !9914

bb4.i349:                                         ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i260.i
  store float %running.sroa.0.0.i294.lcssa51425905, ptr %_21.i267.i, align 1, !dbg !19155
  store float %running.sroa.0.0.i.lcssa51795922, ptr %_21.i.i, align 1, !dbg !19185
  store float %_0.i13445804, ptr %_115.i, align 1, !dbg !19797
  store float %_0.i.i15235821, ptr %392, align 1, !dbg !19799
  store float %_0.i13575838, ptr %_117.i, align 1, !dbg !19801
  store float %_0.i13705855, ptr %_119.i, align 1, !dbg !19802
  store float %_0.i.i15375872, ptr %398, align 1, !dbg !19804
  store float %_0.i13835889, ptr %_121.i, align 1, !dbg !19806
  %_5.i342 = add i64 %_217.i, 1, !dbg !19965
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_217.i, i64 noundef %_5.i342, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i254.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !19966, !noalias !19967
  unreachable, !dbg !19966

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit350: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i260.i
  %_15.i347 = getelementptr inbounds nuw float, ptr %_54.0.i253.i, i64 %_217.i, !dbg !19973
  %_0.i1169 = load float, ptr %_15.i347, align 4, !dbg !19977, !alias.scope !19979, !noalias !19982, !noundef !12
  %position.i290 = zext i32 %storemerge.i2993757 to i64, !dbg !19983
  %440 = icmp eq i32 %storemerge.i2993757, 0, !dbg !19984
  %_3.i.i1672.inv = fcmp olt float %running.sroa.0.0.i2943784, %_0.i1169, !dbg !19984
  %_4.i.i1679.v = select i1 %_3.i.i1672.inv, float %running.sroa.0.0.i2943784, float %_0.i1169, !dbg !19984
  %running.sroa.0.0.i294 = select i1 %440, float %_0.i1169, float %_4.i.i1679.v, !dbg !19984
  %_15.i295 = add nuw nsw i64 %position.i290, 1, !dbg !19985
  %complete.i296 = icmp eq i64 %_15.i295, %_18.i264.i, !dbg !19985
  br i1 %complete.i296, label %bb19.i307, label %bb7.i297, !dbg !19987

bb7.i297:                                         ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit350
  %or.cond.i336.not = icmp ult i64 %_218.i, %_54.1.i254.i, !dbg !19989
  br i1 %or.cond.i336.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit341, label %bb4.i340, !dbg !19989, !prof !9914

bb4.i340:                                         ; preds = %bb7.i297
  store float %running.sroa.0.0.i294.lcssa51425905, ptr %_21.i267.i, align 1, !dbg !19155
  store float %running.sroa.0.0.i.lcssa51795922, ptr %_21.i.i, align 1, !dbg !19185
  store float %_0.i13445804, ptr %_115.i, align 1, !dbg !19797
  store float %_0.i.i15235821, ptr %392, align 1, !dbg !19799
  store float %_0.i13575838, ptr %_117.i, align 1, !dbg !19801
  store float %_0.i13705855, ptr %_119.i, align 1, !dbg !19802
  store float %_0.i.i15375872, ptr %398, align 1, !dbg !19804
  store float %_0.i13835889, ptr %_121.i, align 1, !dbg !19806
  %_5.i333 = add i64 %_218.i, 1, !dbg !19994
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_218.i, i64 noundef %_5.i333, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i254.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !19995, !noalias !19996
  unreachable, !dbg !19995

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit341: ; preds = %bb7.i297
  %_15.i338 = getelementptr inbounds nuw float, ptr %_54.0.i253.i, i64 %_218.i, !dbg !19999
  %_0.i1171 = load float, ptr %_15.i338, align 4, !dbg !20001, !alias.scope !20003, !noalias !19982, !noundef !12
  %_3.i.i1663.inv = fcmp olt float %_0.i1171, %running.sroa.0.0.i294, !dbg !20006
  %_4.i.i1670.v = select i1 %_3.i.i1663.inv, float %_0.i1171, float %running.sroa.0.0.i294, !dbg !20006
  %441 = trunc i64 %_15.i295 to i32, !dbg !20010
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit318, !dbg !20012

bb19.i307:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit350, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i314
  %end.sroa.0.0.i3053606 = phi i64 [ %443, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i314 ], [ %_217.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit350 ]
  %suffix.sroa.0.0.i3043605 = phi float [ %_4.i.i1661.v, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i314 ], [ %_0.i1169, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit350 ]
  %iter.sroa.0.0.i3033604 = phi i64 [ %_30.i308, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i314 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit350 ]
  %or.cond.i320.not = icmp ult i64 %end.sroa.0.0.i3053606, %_54.1.i254.i, !dbg !20013
  br i1 %or.cond.i320.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i314, label %bb4.i323, !dbg !20013, !prof !9914

bb4.i323:                                         ; preds = %bb19.i307
  store float %running.sroa.0.0.i294.lcssa51425905, ptr %_21.i267.i, align 1, !dbg !19155
  store float %running.sroa.0.0.i.lcssa51795922, ptr %_21.i.i, align 1, !dbg !19185
  store float %_0.i13445804, ptr %_115.i, align 1, !dbg !19797
  store float %_0.i.i15235821, ptr %392, align 1, !dbg !19799
  store float %_0.i13575838, ptr %_117.i, align 1, !dbg !19801
  store float %_0.i13705855, ptr %_119.i, align 1, !dbg !19802
  store float %_0.i.i15375872, ptr %398, align 1, !dbg !19804
  store float %_0.i13835889, ptr %_121.i, align 1, !dbg !19806
  %_5.i = add i64 %end.sroa.0.0.i3053606, 1, !dbg !20021
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %end.sroa.0.0.i3053606, i64 noundef %_5.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i254.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !20022, !noalias !20023
  unreachable, !dbg !20022

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i314: ; preds = %bb19.i307
  %_30.i308 = add nuw i64 %iter.sroa.0.0.i3033604, 1, !dbg !20026
  %_15.i322 = getelementptr inbounds nuw float, ptr %_54.0.i253.i, i64 %end.sroa.0.0.i3053606, !dbg !20037
  %_0.i1175 = load float, ptr %_15.i322, align 4, !dbg !20039, !alias.scope !20041, !noalias !19982, !noundef !12
  %_3.i.i1654.inv = fcmp olt float %suffix.sroa.0.0.i3043605, %_0.i1175, !dbg !20044
  %_4.i.i1661.v = select i1 %_3.i.i1654.inv, float %suffix.sroa.0.0.i3043605, float %_0.i1175, !dbg !20044
  store float %_4.i.i1661.v, ptr %_15.i322, align 4, !dbg !20047, !alias.scope !20050, !noalias !19982
  %442 = icmp eq i64 %end.sroa.0.0.i3053606, 0, !dbg !20053
  %spec.store.select.i316 = select i1 %442, i64 %ring.i, i64 %end.sroa.0.0.i3053606, !dbg !20053
  %443 = add i64 %spec.store.select.i316, -1, !dbg !20054
  %exitcond4895.not = icmp eq i64 %_30.i308, %umax4894, !dbg !20055
  br i1 %exitcond4895.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit318, label %bb19.i307, !dbg !20058

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit318: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i314, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit341
  %storemerge.i299 = phi i32 [ %441, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit341 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i314 ], !dbg !20059
  %running.sroa.0.1.i300 = phi float [ %_4.i.i1670.v, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit341 ], [ %running.sroa.0.0.i294, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i314 ], !dbg !19155
  %_0.i1127 = fmul float %running.sroa.0.1.i300, 1.638400e+04, !dbg !20060
  %444 = tail call noundef float @llvm.floor.f32(float %_0.i1127), !dbg !20063
  %_0.i1126 = fmul float %444, 0x3F10000000000000, !dbg !20067
  %or.cond.i417.not = icmp ult i64 %_219.i, %_56.1.i276.i, !dbg !20069
  br i1 %or.cond.i417.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit422, label %bb4.i421, !dbg !20069, !prof !9914

bb4.i421:                                         ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit318
  store float %running.sroa.0.0.i294.lcssa51425905, ptr %_21.i267.i, align 1, !dbg !19155
  store float %running.sroa.0.0.i.lcssa51795922, ptr %_21.i.i, align 1, !dbg !19185
  store float %_0.i13445804, ptr %_115.i, align 1, !dbg !19797
  store float %_0.i.i15235821, ptr %392, align 1, !dbg !19799
  store float %_0.i13575838, ptr %_117.i, align 1, !dbg !19801
  store float %_0.i13705855, ptr %_119.i, align 1, !dbg !19802
  store float %_0.i.i15375872, ptr %398, align 1, !dbg !19804
  store float %_0.i13835889, ptr %_121.i, align 1, !dbg !19806
  %_5.i414 = add i64 %_219.i, 1, !dbg !20075
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_219.i, i64 noundef %_5.i414, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i276.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !20076, !noalias !20077
  unreachable, !dbg !20076

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit422: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit318
  %_15.i419 = getelementptr inbounds nuw float, ptr %_56.0.i275.i, i64 %_219.i, !dbg !20080
  %_0.i1153 = load float, ptr %_15.i419, align 4, !dbg !20082, !alias.scope !20084, !noalias !20087, !noundef !12
  %_0.i907 = fadd float %_0.i1126, %_0.i11513812, !dbg !20088
  %_0.i1151 = fsub float %_0.i907, %_0.i1153, !dbg !20091
  store float %_0.i1151, ptr %404, align 4, !dbg !20093
  %_8.not.i3.i285.i = icmp ugt i64 %_7.i8.i256.i, %_56.1.i276.i
  br i1 %_8.not.i3.i285.i, label %bb4.i6.i314.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i286.i, !dbg !20094, !prof !5262

bb4.i6.i314.i:                                    ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit422
  store float %running.sroa.0.0.i294.lcssa51425905, ptr %_21.i267.i, align 1, !dbg !19155
  store float %running.sroa.0.0.i.lcssa51795922, ptr %_21.i.i, align 1, !dbg !19185
  store float %_0.i13445804, ptr %_115.i, align 1, !dbg !19797
  store float %_0.i.i15235821, ptr %392, align 1, !dbg !19799
  store float %_0.i13575838, ptr %_117.i, align 1, !dbg !19801
  store float %_0.i13705855, ptr %_119.i, align 1, !dbg !19802
  store float %_0.i.i15375872, ptr %398, align 1, !dbg !19804
  store float %_0.i13835889, ptr %_121.i, align 1, !dbg !19806
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_215.i, i64 noundef %_7.i8.i256.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i276.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cd0e502fea74c9eb9984d521d7f3533e) #31, !dbg !20099, !noalias !20100
  unreachable, !dbg !20099

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i286.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit422
  %_17.i5.i287.i = getelementptr inbounds nuw float, ptr %_56.0.i275.i, i64 %_215.i, !dbg !20103
  store float %_0.i1126, ptr %_17.i5.i287.i, align 4, !dbg !20105, !alias.scope !20107, !noalias !20087
  %_0.i914 = fdiv float %_0.i1151, %_37.i288.i, !dbg !20110
  %_0.i1150 = fsub float 1.000000e+00, %_0.i914, !dbg !20112
  %_0.i1149 = fsub float %_0.i1150, %_0.i13353814, !dbg !20115
  %_4.i922 = fmul float %_0.i13573680, %_0.i1149, !dbg !20118
  %_0.i923 = fadd float %_0.i13353814, %_4.i922, !dbg !20118
  %_3.i.i1636.inv = fcmp ogt float %_0.i1150, %_0.i923, !dbg !20120
  %_4.i.i1643.v = select i1 %_3.i.i1636.inv, float %_0.i1150, float %_0.i923, !dbg !20120
  %445 = tail call noundef float @llvm.fabs.f32(float %_4.i.i1643.v), !dbg !20124
  %446 = fcmp uge float %445, 0x3BC79CA100000000, !dbg !20128
  %_0.i1335 = select i1 %446, float %_4.i.i1643.v, float 0.000000e+00, !dbg !20130
  store float %_0.i1335, ptr %406, align 4, !dbg !20131
  %_5.i405 = add i64 %_216.i, 1, !dbg !20132
  %or.cond.i408.not = icmp ult i64 %_216.i, %_58.1.i301.i, !dbg !20135
  br i1 %or.cond.i408.not, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1317, label %bb4.i412, !dbg !20135, !prof !9914

bb4.i412:                                         ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i286.i
  store float %running.sroa.0.0.i294.lcssa51425905, ptr %_21.i267.i, align 1, !dbg !19155
  store float %running.sroa.0.0.i.lcssa51795922, ptr %_21.i.i, align 1, !dbg !19185
  store float %_0.i13445804, ptr %_115.i, align 1, !dbg !19797
  store float %_0.i.i15235821, ptr %392, align 1, !dbg !19799
  store float %_0.i13575838, ptr %_117.i, align 1, !dbg !19801
  store float %_0.i13705855, ptr %_119.i, align 1, !dbg !19802
  store float %_0.i.i15375872, ptr %398, align 1, !dbg !19804
  store float %_0.i13835889, ptr %_121.i, align 1, !dbg !19806
  %umax4899 = call i64 @llvm.umax.i64(i64 %main_cursor.sroa.0.1.i3875, i64 %_58.1.i301.i), !dbg !19773
  %447 = add i64 %umax4899, 1, !dbg !19773
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_216.i, i64 noundef %447, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i301.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !20139, !noalias !20140
  unreachable, !dbg !20139

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1317: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i286.i
  %_0.i1148 = fsub float 1.000000e+00, %_0.i1335, !dbg !20143
  %_15.i410 = getelementptr inbounds nuw float, ptr %_58.0.i300.i, i64 %_216.i, !dbg !20145
  %_0.i1155 = load float, ptr %_15.i410, align 4, !dbg !20147, !alias.scope !20149, !noalias !20087, !noundef !12
  store float %_0.i1238, ptr %_15.i410, align 4, !dbg !20152, !alias.scope !20156, !noalias !20087
  %_0.i1125 = fmul float %_0.i1148, %_0.i1155, !dbg !20159
  %_6.i1489 = bitcast float %_0.i1155 to i32, !dbg !20161
  %_5.i1490 = and i32 %_6.i1489, %all.sroa.0.0.i, !dbg !20164
  %_8.i1491 = bitcast float %_0.i1125 to i32, !dbg !20165
  %_7.i1493 = and i32 %_9.i1492, %_8.i1491, !dbg !20167
  %_4.i1494 = or disjoint i32 %_7.i1493, %_5.i1490, !dbg !20164
  store i32 %_4.i1494, ptr %data.i.i.i.i.i.i.i.i, align 4, !dbg !20168, !alias.scope !20170, !noalias !20173
  %_222.i = add nuw i64 %iter.i.sroa.41.03612, %right_end.sroa.0.0.i, !dbg !20174
  %_224.i = add nuw i64 %iter.i.sroa.41.03612, %right_expiring.sroa.0.0.i, !dbg !20176
  %_8.not.i10.i.i = icmp ugt i64 %_7.i8.i256.i, %_54.1.i.i
  br i1 %_8.not.i10.i.i, label %bb4.i13.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i, !dbg !20177, !prof !5262

bb4.i13.i.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1317
  store float %running.sroa.0.0.i294.lcssa51425905, ptr %_21.i267.i, align 1, !dbg !19155
  store float %running.sroa.0.0.i.lcssa51795922, ptr %_21.i.i, align 1, !dbg !19185
  store float %_0.i13445804, ptr %_115.i, align 1, !dbg !19797
  store float %_0.i.i15235821, ptr %392, align 1, !dbg !19799
  store float %_0.i13575838, ptr %_117.i, align 1, !dbg !19801
  store float %_0.i13705855, ptr %_119.i, align 1, !dbg !19802
  store float %_0.i.i15375872, ptr %398, align 1, !dbg !19804
  store float %_0.i13835889, ptr %_121.i, align 1, !dbg !19806
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_215.i, i64 noundef %_7.i8.i256.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cd0e502fea74c9eb9984d521d7f3533e) #31, !dbg !20182, !noalias !20183
  unreachable, !dbg !20182

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1317
  %_0.i913 = fdiv float %_0.i13703716, %_0.i1508, !dbg !20191
  %_3.i692 = fcmp uge float %_0.i13703716, %_0.i1508, !dbg !20193
  %_0.i1488 = select i1 %_3.i692, float 1.000000e+00, float %_0.i913, !dbg !20195
  %_17.i12.i.i = getelementptr inbounds nuw float, ptr %_54.0.i.i, i64 %_215.i, !dbg !20197
  store float %_0.i1488, ptr %_17.i12.i.i, align 4, !dbg !20199, !alias.scope !20201, !noalias !20204
  %or.cond.i381.not = icmp ult i64 %_222.i, %_54.1.i.i, !dbg !20205
  br i1 %or.cond.i381.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit386, label %bb4.i385, !dbg !20205, !prof !9914

bb4.i385:                                         ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i
  store float %running.sroa.0.0.i294.lcssa51425905, ptr %_21.i267.i, align 1, !dbg !19155
  store float %running.sroa.0.0.i.lcssa51795922, ptr %_21.i.i, align 1, !dbg !19185
  store float %_0.i13445804, ptr %_115.i, align 1, !dbg !19797
  store float %_0.i.i15235821, ptr %392, align 1, !dbg !19799
  store float %_0.i13575838, ptr %_117.i, align 1, !dbg !19801
  store float %_0.i13705855, ptr %_119.i, align 1, !dbg !19802
  store float %_0.i.i15375872, ptr %398, align 1, !dbg !19804
  store float %_0.i13835889, ptr %_121.i, align 1, !dbg !19806
  %_5.i378 = add i64 %_222.i, 1, !dbg !20210
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_222.i, i64 noundef %_5.i378, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !20211, !noalias !20212
  unreachable, !dbg !20211

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit386: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i
  %_15.i383 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i64 %_222.i, !dbg !20218
  %_0.i1161 = load float, ptr %_15.i383, align 4, !dbg !20220, !alias.scope !20222, !noalias !20225, !noundef !12
  %position.i = zext i32 %storemerge.i3815 to i64, !dbg !20226
  %448 = icmp eq i32 %storemerge.i3815, 0, !dbg !20227
  %_3.i.i1699.inv = fcmp olt float %running.sroa.0.0.i3842, %_0.i1161, !dbg !20227
  %_4.i.i1706.v = select i1 %_3.i.i1699.inv, float %running.sroa.0.0.i3842, float %_0.i1161, !dbg !20227
  %running.sroa.0.0.i = select i1 %448, float %_0.i1161, float %_4.i.i1706.v, !dbg !20227
  %_15.i = add nuw nsw i64 %position.i, 1, !dbg !20228
  %complete.i = icmp eq i64 %_15.i, %_18.i.i, !dbg !20228
  br i1 %complete.i, label %bb19.i, label %bb7.i281, !dbg !20229

bb7.i281:                                         ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit386
  %or.cond.i372.not = icmp ult i64 %_218.i, %_54.1.i.i, !dbg !20230
  br i1 %or.cond.i372.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit377, label %bb4.i376, !dbg !20230, !prof !9914

bb4.i376:                                         ; preds = %bb7.i281
  store float %running.sroa.0.0.i294.lcssa51425905, ptr %_21.i267.i, align 1, !dbg !19155
  store float %running.sroa.0.0.i.lcssa51795922, ptr %_21.i.i, align 1, !dbg !19185
  store float %_0.i13445804, ptr %_115.i, align 1, !dbg !19797
  store float %_0.i.i15235821, ptr %392, align 1, !dbg !19799
  store float %_0.i13575838, ptr %_117.i, align 1, !dbg !19801
  store float %_0.i13705855, ptr %_119.i, align 1, !dbg !19802
  store float %_0.i.i15375872, ptr %398, align 1, !dbg !19804
  store float %_0.i13835889, ptr %_121.i, align 1, !dbg !19806
  %_5.i369 = add i64 %_218.i, 1, !dbg !20235
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_218.i, i64 noundef %_5.i369, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !20236, !noalias !20237
  unreachable, !dbg !20236

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit377: ; preds = %bb7.i281
  %_15.i374 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i64 %_218.i, !dbg !20240
  %_0.i1163 = load float, ptr %_15.i374, align 4, !dbg !20242, !alias.scope !20244, !noalias !20225, !noundef !12
  %_3.i.i1690.inv = fcmp olt float %_0.i1163, %running.sroa.0.0.i, !dbg !20247
  %_4.i.i1697.v = select i1 %_3.i.i1690.inv, float %_0.i1163, float %running.sroa.0.0.i, !dbg !20247
  %449 = trunc i64 %_15.i to i32, !dbg !20250
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit, !dbg !20251

bb19.i:                                           ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit386, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i
  %end.sroa.0.0.i3609 = phi i64 [ %451, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], [ %_222.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit386 ]
  %suffix.sroa.0.0.i3608 = phi float [ %_4.i.i1688.v, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], [ %_0.i1161, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit386 ]
  %iter.sroa.0.0.i2833607 = phi i64 [ %_30.i284, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit386 ]
  %or.cond.i354.not = icmp ult i64 %end.sroa.0.0.i3609, %_54.1.i.i, !dbg !20252
  br i1 %or.cond.i354.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i, label %bb4.i358, !dbg !20252, !prof !9914

bb4.i358:                                         ; preds = %bb19.i
  store float %running.sroa.0.0.i294.lcssa51425905, ptr %_21.i267.i, align 1, !dbg !19155
  store float %running.sroa.0.0.i.lcssa51795922, ptr %_21.i.i, align 1, !dbg !19185
  store float %_0.i13445804, ptr %_115.i, align 1, !dbg !19797
  store float %_0.i.i15235821, ptr %392, align 1, !dbg !19799
  store float %_0.i13575838, ptr %_117.i, align 1, !dbg !19801
  store float %_0.i13705855, ptr %_119.i, align 1, !dbg !19802
  store float %_0.i.i15375872, ptr %398, align 1, !dbg !19804
  store float %_0.i13835889, ptr %_121.i, align 1, !dbg !19806
  %_5.i351 = add i64 %end.sroa.0.0.i3609, 1, !dbg !20257
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %end.sroa.0.0.i3609, i64 noundef %_5.i351, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !20258, !noalias !20259
  unreachable, !dbg !20258

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i: ; preds = %bb19.i
  %_30.i284 = add nuw i64 %iter.sroa.0.0.i2833607, 1, !dbg !20262
  %_15.i356 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i64 %end.sroa.0.0.i3609, !dbg !20267
  %_0.i1167 = load float, ptr %_15.i356, align 4, !dbg !20269, !alias.scope !20271, !noalias !20225, !noundef !12
  %_3.i.i1681.inv = fcmp olt float %suffix.sroa.0.0.i3608, %_0.i1167, !dbg !20274
  %_4.i.i1688.v = select i1 %_3.i.i1681.inv, float %suffix.sroa.0.0.i3608, float %_0.i1167, !dbg !20274
  store float %_4.i.i1688.v, ptr %_15.i356, align 4, !dbg !20277, !alias.scope !20280, !noalias !20225
  %450 = icmp eq i64 %end.sroa.0.0.i3609, 0, !dbg !20283
  %spec.store.select.i287 = select i1 %450, i64 %ring.i, i64 %end.sroa.0.0.i3609, !dbg !20283
  %451 = add i64 %spec.store.select.i287, -1, !dbg !20284
  %exitcond4897.not = icmp eq i64 %_30.i284, %umax4896, !dbg !20285
  br i1 %exitcond4897.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit, label %bb19.i, !dbg !20287

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit377
  %storemerge.i = phi i32 [ %449, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit377 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], !dbg !20288
  %running.sroa.0.1.i = phi float [ %_4.i.i1697.v, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit377 ], [ %running.sroa.0.0.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], !dbg !19185
  %_0.i1124 = fmul float %running.sroa.0.1.i, 1.638400e+04, !dbg !20289
  %452 = tail call noundef float @llvm.floor.f32(float %_0.i1124), !dbg !20291
  %_0.i1123 = fmul float %452, 0x3F10000000000000, !dbg !20295
  %or.cond.i399.not = icmp ult i64 %_224.i, %_56.1.i.i, !dbg !20297
  br i1 %or.cond.i399.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit404, label %bb4.i403, !dbg !20297, !prof !9914

bb4.i403:                                         ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit
  store float %running.sroa.0.0.i294.lcssa51425905, ptr %_21.i267.i, align 1, !dbg !19155
  store float %running.sroa.0.0.i.lcssa51795922, ptr %_21.i.i, align 1, !dbg !19185
  store float %_0.i13445804, ptr %_115.i, align 1, !dbg !19797
  store float %_0.i.i15235821, ptr %392, align 1, !dbg !19799
  store float %_0.i13575838, ptr %_117.i, align 1, !dbg !19801
  store float %_0.i13705855, ptr %_119.i, align 1, !dbg !19802
  store float %_0.i.i15375872, ptr %398, align 1, !dbg !19804
  store float %_0.i13835889, ptr %_121.i, align 1, !dbg !19806
  %_5.i396 = add i64 %_224.i, 1, !dbg !20302
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_224.i, i64 noundef %_5.i396, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !20303, !noalias !20304
  unreachable, !dbg !20303

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit404: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit
  %_15.i401 = getelementptr inbounds nuw float, ptr %_56.0.i.i, i64 %_224.i, !dbg !20307
  %_0.i1157 = load float, ptr %_15.i401, align 4, !dbg !20309, !alias.scope !20311, !noalias !20314, !noundef !12
  %_0.i906 = fadd float %_0.i1123, %_0.i11473870, !dbg !20315
  %_0.i1147 = fsub float %_0.i906, %_0.i1157, !dbg !20317
  store float %_0.i1147, ptr %412, align 4, !dbg !20319
  %_8.not.i3.i.i = icmp ugt i64 %_7.i8.i256.i, %_56.1.i.i
  br i1 %_8.not.i3.i.i, label %bb4.i6.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i, !dbg !20320, !prof !5262

bb4.i6.i.i:                                       ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit404
  store float %running.sroa.0.0.i294.lcssa51425905, ptr %_21.i267.i, align 1, !dbg !19155
  store float %running.sroa.0.0.i.lcssa51795922, ptr %_21.i.i, align 1, !dbg !19185
  store float %_0.i13445804, ptr %_115.i, align 1, !dbg !19797
  store float %_0.i.i15235821, ptr %392, align 1, !dbg !19799
  store float %_0.i13575838, ptr %_117.i, align 1, !dbg !19801
  store float %_0.i13705855, ptr %_119.i, align 1, !dbg !19802
  store float %_0.i.i15375872, ptr %398, align 1, !dbg !19804
  store float %_0.i13835889, ptr %_121.i, align 1, !dbg !19806
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_215.i, i64 noundef %_7.i8.i256.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cd0e502fea74c9eb9984d521d7f3533e) #31, !dbg !20325, !noalias !20326
  unreachable, !dbg !20325

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit404
  %_17.i5.i.i = getelementptr inbounds nuw float, ptr %_56.0.i.i, i64 %_215.i, !dbg !20329
  store float %_0.i1123, ptr %_17.i5.i.i, align 4, !dbg !20331, !alias.scope !20333, !noalias !20314
  %_0.i912 = fdiv float %_0.i1147, %_37.i.i, !dbg !20336
  %_0.i1146 = fsub float 1.000000e+00, %_0.i912, !dbg !20338
  %_0.i1145 = fsub float %_0.i1146, %_0.i13313872, !dbg !20340
  %_4.i920 = fmul float %_0.i13833752, %_0.i1145, !dbg !20342
  %_0.i921 = fadd float %_0.i13313872, %_4.i920, !dbg !20342
  %_3.i.i1627.inv = fcmp ogt float %_0.i1146, %_0.i921, !dbg !20344
  %_4.i.i1634.v = select i1 %_3.i.i1627.inv, float %_0.i1146, float %_0.i921, !dbg !20344
  %453 = tail call noundef float @llvm.fabs.f32(float %_4.i.i1634.v), !dbg !20347
  %454 = fcmp uge float %453, 0x3BC79CA100000000, !dbg !20350
  %_0.i1331 = select i1 %454, float %_4.i.i1634.v, float 0.000000e+00, !dbg !20352
  store float %_0.i1331, ptr %414, align 4, !dbg !20353
  %_6.not.i389 = icmp ugt i64 %_5.i405, %_58.1.i.i
  br i1 %_6.not.i389, label %bb4.i394, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310, !dbg !20354, !prof !5262

bb4.i394:                                         ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i
  store float %running.sroa.0.0.i294.lcssa51425905, ptr %_21.i267.i, align 1, !dbg !19155
  store float %running.sroa.0.0.i.lcssa51795922, ptr %_21.i.i, align 1, !dbg !19185
  store float %_0.i13445804, ptr %_115.i, align 1, !dbg !19797
  store float %_0.i.i15235821, ptr %392, align 1, !dbg !19799
  store float %_0.i13575838, ptr %_117.i, align 1, !dbg !19801
  store float %_0.i13705855, ptr %_119.i, align 1, !dbg !19802
  store float %_0.i.i15375872, ptr %398, align 1, !dbg !19804
  store float %_0.i13835889, ptr %_121.i, align 1, !dbg !19806
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_216.i, i64 noundef %_5.i405, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !20359, !noalias !20360
  unreachable, !dbg !20359

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1310: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i
  %_0.i1144 = fsub float 1.000000e+00, %_0.i1331, !dbg !20363
  %_15.i392 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %_216.i, !dbg !20365
  %_0.i1159 = load float, ptr %_15.i392, align 4, !dbg !20367, !alias.scope !20369, !noalias !20314, !noundef !12
  store float %_0.i1233, ptr %_15.i392, align 4, !dbg !20372, !alias.scope !20375, !noalias !20314
  %_0.i1122 = fmul float %_0.i1144, %_0.i1159, !dbg !20378
  %_6.i1476 = bitcast float %_0.i1159 to i32, !dbg !20380
  %_5.i1477 = and i32 %_6.i1476, %all.sroa.0.0.i, !dbg !20383
  %_8.i1478 = bitcast float %_0.i1122 to i32, !dbg !20384
  %_7.i1480 = and i32 %_9.i1492, %_8.i1478, !dbg !20386
  %_4.i1481 = or disjoint i32 %_7.i1480, %_5.i1477, !dbg !20383
  store i32 %_4.i1481, ptr %data.i5.i.i.i.i.i.i.i, align 4, !dbg !20387, !alias.scope !20389, !noalias !20392
  %exitcond4908.not = icmp eq i64 %_9.0.i2046, %umin4907, !dbg !19773
  br i1 %exitcond4908.not, label %bb41.i.bb44.i_crit_edge, label %bb43.i, !dbg !19773

_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformfEB2_.exit.loopexit: ; preds = %bb27.i.loopexit
  %455 = trunc i64 %main_cursor.sroa.0.1.i.lcssa to i32, !dbg !20393
  %456 = trunc i64 %ring_cursor.sroa.0.1.i.lcssa to i32, !dbg !20395
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformfEB2_.exit, !dbg !20396

_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformfEB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformfEB2_.exit.loopexit, %bb14.i
  %ring_cursor.sroa.0.0.i.lcssa = phi i32 [ %_43.i, %bb14.i ], [ %456, %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformfEB2_.exit.loopexit ], !dbg !19123
  %main_cursor.sroa.0.0.i.lcssa = phi i32 [ %_42.i, %bb14.i ], [ %455, %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformfEB2_.exit.loopexit ], !dbg !19120
  %457 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 72, !dbg !20396
  %left_prefix.i = load float, ptr %457, align 8, !dbg !20396, !noalias !19127, !noundef !12
  %458 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 76, !dbg !20397
  %left_phase.i = load i32, ptr %458, align 4, !dbg !20397, !noalias !19127, !noundef !12
  %459 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 72, !dbg !20398
  %right_prefix.i = load float, ptr %459, align 8, !dbg !20398, !noalias !19127, !noundef !12
  %460 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 76, !dbg !20399
  %right_phase.i = load i32, ptr %460, align 4, !dbg !20399, !noalias !19127, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_right.i), !dbg !20400, !noalias !19127
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i), !dbg !20401, !noalias !19127
  %461 = getelementptr inbounds nuw i8, ptr %self, i64 216, !dbg !20402
  %_244.0.i = load ptr, ptr %461, align 8, !dbg !20402, !alias.scope !18990, !noalias !19004, !nonnull !12, !noundef !12
  %462 = getelementptr inbounds nuw i8, ptr %self, i64 224, !dbg !20402
  %_244.1.i = load i64, ptr %462, align 8, !dbg !20402, !alias.scope !18990, !noalias !19004, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20403), !dbg !20406
  %_4.not.i1295 = icmp eq i64 %_244.1.i, 0, !dbg !20407
  br i1 %_4.not.i1295, label %panic.i1297, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1298, !dbg !20407

panic.i1297:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformfEB2_.exit
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #31, !dbg !20407, !noalias !20409
  unreachable, !dbg !20407

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1298: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformfEB2_.exit
  store float %left_prefix.i, ptr %_244.0.i, align 4, !dbg !20407, !alias.scope !20403, !noalias !18994
  %_245.0.i = load ptr, ptr %46, align 8, !dbg !20410, !alias.scope !18990, !noalias !19004, !nonnull !12, !noundef !12
  %_245.1.i = load i64, ptr %47, align 8, !dbg !20410, !alias.scope !18990, !noalias !19004, !noundef !12
  %463 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i), !dbg !20411
  br i1 %463, label %bb2.i2052, label %bb6.i2047, !dbg !20411

bb6.i2047:                                        ; preds = %bb2.i2052, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1298
  %end_or_len.idx.i = shl nuw nsw i64 %_245.1.i, 2, !dbg !20415
  %end_or_len.i = getelementptr inbounds nuw i8, ptr %_245.0.i, i64 %end_or_len.idx.i, !dbg !20415
  %_293.i = icmp eq i64 %_245.1.i, 0, !dbg !20419
  br i1 %_293.i, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i2048, !dbg !20422

bb2.i2052:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1298
  %bytes1.sroa.0.0.zext.i = and i32 %left_phase.i, 255, !dbg !20423
  %bytes1.sroa.0.0.isplat.i = mul nuw i32 %bytes1.sroa.0.0.zext.i, 16843009, !dbg !20423
  %_5.i2053 = icmp eq i32 %left_phase.i, %bytes1.sroa.0.0.isplat.i, !dbg !20424
  br i1 %_5.i2053, label %bb3.i2054, label %bb6.i2047, !dbg !20424

bb3.i2054:                                        ; preds = %bb2.i2052
  %bytes.sroa.0.0.extract.trunc.i = trunc i32 %left_phase.i to i8, !dbg !20425
  %464 = shl nuw nsw i64 %_245.1.i, 2, !dbg !20427
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_245.0.i, i8 %bytes.sroa.0.0.extract.trunc.i, i64 %464, i1 false), !dbg !20427, !alias.scope !20428, !noalias !18994
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, !dbg !20431

bb10.i2048:                                       ; preds = %bb6.i2047, %bb10.i2048
  %iter.sroa.0.04.i = phi ptr [ %_38.i2049, %bb10.i2048 ], [ %_245.0.i, %bb6.i2047 ]
  %_38.i2049 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i, i64 4, !dbg !20432
  store i32 %left_phase.i, ptr %iter.sroa.0.04.i, align 4, !dbg !20434, !alias.scope !20428, !noalias !18994
  %_29.i2050 = icmp eq ptr %_38.i2049, %end_or_len.i, !dbg !20419
  br i1 %_29.i2050, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i2048, !dbg !20422

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit: ; preds = %bb10.i2048, %bb6.i2047, %bb3.i2054
  %465 = getelementptr inbounds nuw i8, ptr %self, i64 416, !dbg !20435
  %_246.0.i = load ptr, ptr %465, align 8, !dbg !20435, !alias.scope !18992, !noalias !19057, !nonnull !12, !noundef !12
  %466 = getelementptr inbounds nuw i8, ptr %self, i64 424, !dbg !20435
  %_246.1.i = load i64, ptr %466, align 8, !dbg !20435, !alias.scope !18992, !noalias !19057, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20436), !dbg !20439
  %_4.not.i1291 = icmp eq i64 %_246.1.i, 0, !dbg !20440
  br i1 %_4.not.i1291, label %panic.i1293, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1294, !dbg !20440

panic.i1293:                                      ; preds = %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #31, !dbg !20440, !noalias !20442
  unreachable, !dbg !20440

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1294: ; preds = %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit
  store float %right_prefix.i, ptr %_246.0.i, align 4, !dbg !20440, !alias.scope !20436, !noalias !18994
  %_247.0.i = load ptr, ptr %55, align 8, !dbg !20443, !alias.scope !18992, !noalias !19057, !nonnull !12, !noundef !12
  %_247.1.i = load i64, ptr %56, align 8, !dbg !20443, !alias.scope !18992, !noalias !19057, !noundef !12
  %467 = tail call i1 @llvm.is.constant.i32(i32 %right_phase.i), !dbg !20444
  br i1 %467, label %bb2.i2064, label %bb6.i2055, !dbg !20444

bb6.i2055:                                        ; preds = %bb2.i2064, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1294
  %end_or_len.idx.i2056 = shl nuw nsw i64 %_247.1.i, 2, !dbg !20447
  %end_or_len.i2057 = getelementptr inbounds nuw i8, ptr %_247.0.i, i64 %end_or_len.idx.i2056, !dbg !20447
  %_293.i2058 = icmp eq i64 %_247.1.i, 0, !dbg !20451
  br i1 %_293.i2058, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit2070, label %bb10.i2059, !dbg !20454

bb2.i2064:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1294
  %bytes1.sroa.0.0.zext.i2065 = and i32 %right_phase.i, 255, !dbg !20455
  %bytes1.sroa.0.0.isplat.i2066 = mul nuw i32 %bytes1.sroa.0.0.zext.i2065, 16843009, !dbg !20455
  %_5.i2067 = icmp eq i32 %right_phase.i, %bytes1.sroa.0.0.isplat.i2066, !dbg !20456
  br i1 %_5.i2067, label %bb3.i2068, label %bb6.i2055, !dbg !20456

bb3.i2068:                                        ; preds = %bb2.i2064
  %bytes.sroa.0.0.extract.trunc.i2069 = trunc i32 %right_phase.i to i8, !dbg !20457
  %468 = shl nuw nsw i64 %_247.1.i, 2, !dbg !20459
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_247.0.i, i8 %bytes.sroa.0.0.extract.trunc.i2069, i64 %468, i1 false), !dbg !20459, !alias.scope !20460, !noalias !18994
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit2070, !dbg !20463

bb10.i2059:                                       ; preds = %bb6.i2055, %bb10.i2059
  %iter.sroa.0.04.i2060 = phi ptr [ %_38.i2061, %bb10.i2059 ], [ %_247.0.i, %bb6.i2055 ]
  %_38.i2061 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i2060, i64 4, !dbg !20464
  store i32 %right_phase.i, ptr %iter.sroa.0.04.i2060, align 4, !dbg !20466, !alias.scope !20460, !noalias !18994
  %_29.i2062 = icmp eq ptr %_38.i2061, %end_or_len.i2057, !dbg !20451
  br i1 %_29.i2062, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit2070, label %bb10.i2059, !dbg !20454

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit2070: ; preds = %bb10.i2059, %bb6.i2055, %bb3.i2068
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %hot_left.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33) #30, !dbg !20467
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %hot_right.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34) #30, !dbg !20468
  store i32 %main_cursor.sroa.0.0.i.lcssa, ptr %_35, align 4, !dbg !20393, !alias.scope !18994, !noalias !19122
  store i32 %ring_cursor.sroa.0.0.i.lcssa, ptr %347, align 4, !dbg !20395, !alias.scope !18994, !noalias !19122
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i), !dbg !20469, !noalias !19127
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i), !dbg !20470, !noalias !19127
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockfEB2_.exit, !dbg !18987

_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockfEB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_lanefEB2_.exit, %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit2070
  br i1 %quiet.sroa.0.02218, label %bb28, label %bb40, !dbg !20471

bb22:                                             ; preds = %bb20
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20472), !dbg !20475
  %469 = getelementptr inbounds nuw i8, ptr %self, i64 248, !dbg !20476
  %_40.0.i = load ptr, ptr %469, align 8, !dbg !20476, !alias.scope !20472, !nonnull !12, !noundef !12
  %470 = getelementptr inbounds nuw i8, ptr %self, i64 256, !dbg !20476
  %_40.1.i = load i64, ptr %470, align 8, !dbg !20476, !alias.scope !20472, !noundef !12
  %471 = getelementptr inbounds nuw i8, ptr %self, i64 312, !dbg !20478
  %_41.0.i = load ptr, ptr %471, align 8, !dbg !20478, !alias.scope !20472, !nonnull !12, !noundef !12
  %472 = getelementptr inbounds nuw i8, ptr %self, i64 320, !dbg !20478
  %_41.1.i = load i64, ptr %472, align 8, !dbg !20478, !alias.scope !20472, !noundef !12
  %..i.i.i.i = tail call noundef i64 @llvm.umin.i64(i64 %_41.1.i, i64 %_40.1.i), !dbg !20479
  %_2.i6.not.i = icmp eq i64 %..i.i.i.i, 0, !dbg !20485
  br i1 %_2.i6.not.i, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb4.i2071, !dbg !20485

bb4.i2071:                                        ; preds = %bb22, %bb6.i2073
  %iter.sroa.8.07.i = phi i64 [ %473, %bb6.i2073 ], [ 0, %bb22 ]
  %_3.i1.i.i = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i, i64 %iter.sroa.8.07.i, !dbg !20488
  %_14.i2072 = load i32, ptr %_3.i1.i.i, align 4, !dbg !20491, !noalias !20472, !noundef !12
  %_20.i = icmp eq i32 %_14.i2072, 0, !dbg !20492
  br i1 %_20.i, label %panic.i2082, label %bb6.i2073, !dbg !20492

bb6.i2073:                                        ; preds = %bb4.i2071
  %_3.i.i.i2074 = getelementptr inbounds nuw i32, ptr %_40.0.i, i64 %iter.sroa.8.07.i, !dbg !20493
  %473 = add nuw i64 %iter.sroa.8.07.i, 1, !dbg !20496
  %window.i2075 = zext i32 %_14.i2072 to i64, !dbg !20491
  %_18.i2076 = load i32, ptr %_3.i.i.i2074, align 4, !dbg !20497, !noalias !20472, !noundef !12
  %_17.i2077 = zext i32 %_18.i2076 to i64, !dbg !20497
  %_19.i2078 = urem i64 %frames, %window.i2075, !dbg !20492
  %_16.i2079 = add nuw nsw i64 %_19.i2078, %_17.i2077, !dbg !20498
  %_15.i2080 = urem i64 %_16.i2079, %window.i2075, !dbg !20499
  %474 = trunc nuw i64 %_15.i2080 to i32, !dbg !20500
  store i32 %474, ptr %_3.i.i.i2074, align 4, !dbg !20500, !noalias !20472
  %exitcond.not.i = icmp eq i64 %473, %..i.i.i.i, !dbg !20485
  br i1 %exitcond.not.i, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb4.i2071, !dbg !20485

panic.i2082:                                      ; preds = %bb4.i2071
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1781ea1b97b96e9885c590e9b4440a45) #31, !dbg !20492, !noalias !20472
  unreachable, !dbg !20492

_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit: ; preds = %bb6.i2073, %bb22
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20501), !dbg !20504
  %475 = getelementptr inbounds nuw i8, ptr %self, i64 448, !dbg !20505
  %_40.0.i2083 = load ptr, ptr %475, align 8, !dbg !20505, !alias.scope !20501, !nonnull !12, !noundef !12
  %476 = getelementptr inbounds nuw i8, ptr %self, i64 456, !dbg !20505
  %_40.1.i2084 = load i64, ptr %476, align 8, !dbg !20505, !alias.scope !20501, !noundef !12
  %477 = getelementptr inbounds nuw i8, ptr %self, i64 512, !dbg !20507
  %_41.0.i2085 = load ptr, ptr %477, align 8, !dbg !20507, !alias.scope !20501, !nonnull !12, !noundef !12
  %478 = getelementptr inbounds nuw i8, ptr %self, i64 520, !dbg !20507
  %_41.1.i2086 = load i64, ptr %478, align 8, !dbg !20507, !alias.scope !20501, !noundef !12
  %..i.i.i.i2087 = tail call noundef i64 @llvm.umin.i64(i64 %_41.1.i2086, i64 %_40.1.i2084), !dbg !20508
  %_2.i6.not.i2088 = icmp eq i64 %..i.i.i.i2087, 0, !dbg !20514
  br i1 %_2.i6.not.i2088, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit2105, label %bb4.i2089, !dbg !20514

bb4.i2089:                                        ; preds = %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, %bb6.i2094
  %iter.sroa.8.07.i2090 = phi i64 [ %479, %bb6.i2094 ], [ 0, %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit ]
  %_3.i1.i.i2091 = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i2085, i64 %iter.sroa.8.07.i2090, !dbg !20517
  %_14.i2092 = load i32, ptr %_3.i1.i.i2091, align 4, !dbg !20520, !noalias !20501, !noundef !12
  %_20.i2093 = icmp eq i32 %_14.i2092, 0, !dbg !20521
  br i1 %_20.i2093, label %panic.i2104, label %bb6.i2094, !dbg !20521

bb6.i2094:                                        ; preds = %bb4.i2089
  %_3.i.i.i2095 = getelementptr inbounds nuw i32, ptr %_40.0.i2083, i64 %iter.sroa.8.07.i2090, !dbg !20522
  %479 = add nuw i64 %iter.sroa.8.07.i2090, 1, !dbg !20525
  %window.i2096 = zext i32 %_14.i2092 to i64, !dbg !20520
  %_18.i2097 = load i32, ptr %_3.i.i.i2095, align 4, !dbg !20526, !noalias !20501, !noundef !12
  %_17.i2098 = zext i32 %_18.i2097 to i64, !dbg !20526
  %_19.i2099 = urem i64 %frames, %window.i2096, !dbg !20521
  %_16.i2100 = add nuw nsw i64 %_19.i2099, %_17.i2098, !dbg !20527
  %_15.i2101 = urem i64 %_16.i2100, %window.i2096, !dbg !20528
  %480 = trunc nuw i64 %_15.i2101 to i32, !dbg !20529
  store i32 %480, ptr %_3.i.i.i2095, align 4, !dbg !20529, !noalias !20501
  %exitcond.not.i2102 = icmp eq i64 %479, %..i.i.i.i2087, !dbg !20514
  br i1 %exitcond.not.i2102, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit2105, label %bb4.i2089, !dbg !20514

panic.i2104:                                      ; preds = %bb4.i2089
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1781ea1b97b96e9885c590e9b4440a45) #31, !dbg !20521, !noalias !20501
  unreachable, !dbg !20521

_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit2105: ; preds = %bb6.i2094, %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit
  %481 = getelementptr inbounds nuw i8, ptr %self, i64 544, !dbg !20530
  %_29.val = load i64, ptr %481, align 8, !dbg !20530
  %482 = getelementptr inbounds nuw i8, ptr %self, i64 552, !dbg !20530
  %_29.val1708 = load i64, ptr %482, align 8, !dbg !20530, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20531), !dbg !20530
  %_10.i2106 = icmp eq i64 %_29.val1708, 0, !dbg !20534
  br i1 %_10.i2106, label %panic.i2120, label %bb1.i2107, !dbg !20534

bb1.i2107:                                        ; preds = %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit2105
  %_28 = getelementptr inbounds nuw i8, ptr %self, i64 560, !dbg !20536
  %_7.i2108 = load i32, ptr %_28, align 4, !dbg !20537, !alias.scope !20531, !noundef !12
  %_6.i2109 = zext i32 %_7.i2108 to i64, !dbg !20537
  %_8.i2110 = urem i64 %frames, %_29.val1708, !dbg !20534
  %_5.i2111 = add nuw nsw i64 %_8.i2110, %_6.i2109, !dbg !20538
  %_4.i2112 = urem i64 %_5.i2111, %_29.val1708, !dbg !20539
  %483 = trunc i64 %_4.i2112 to i32, !dbg !20540
  store i32 %483, ptr %_28, align 4, !dbg !20540, !alias.scope !20531
  %_17.i2113 = icmp eq i64 %_29.val, 0, !dbg !20541
  br i1 %_17.i2113, label %panic2.i, label %_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit, !dbg !20541

panic.i2120:                                      ; preds = %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit2105
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6b076e9a9e313bc2481504f4da644a5c) #31, !dbg !20534, !noalias !20531
  unreachable, !dbg !20534

panic2.i:                                         ; preds = %bb1.i2107
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f370b9a38141751c9788be81259bacff) #31, !dbg !20541, !noalias !20531
  unreachable, !dbg !20541

_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit: ; preds = %bb1.i2107
  %484 = getelementptr inbounds nuw i8, ptr %self, i64 564, !dbg !20542
  %_14.i2115 = load i32, ptr %484, align 4, !dbg !20542, !alias.scope !20531, !noundef !12
  %_13.i2116 = zext i32 %_14.i2115 to i64, !dbg !20542
  %_15.i2117 = urem i64 %frames, %_29.val, !dbg !20541
  %_12.i2118 = add nuw nsw i64 %_15.i2117, %_13.i2116, !dbg !20543
  %_11.i2119 = urem i64 %_12.i2118, %_29.val, !dbg !20544
  %485 = trunc i64 %_11.i2119 to i32, !dbg !20545
  store i32 %485, ptr %484, align 4, !dbg !20545, !alias.scope !20531
  br label %bb42, !dbg !20546

bb28:                                             ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockfEB2_.exit
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_37 = tail call noundef zeroext i1 @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #30, !dbg !20547
  br i1 %_37, label %bb30, label %bb40, !dbg !20548

bb30:                                             ; preds = %bb28
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_39 = tail call noundef zeroext i1 @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #30, !dbg !20549
  br i1 %_39, label %bb32, label %bb40, !dbg !20550

bb32:                                             ; preds = %bb30
  %_80.not = icmp ugt i64 %frames, %left_io.1
  br i1 %_80.not, label %bb51, label %bb1.i2121, !dbg !20551, !prof !5262

bb51:                                             ; preds = %bb32
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %frames, i64 noundef %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_df34220a19b474a957e74caf6cc3e38d) #31, !dbg !20559
  unreachable, !dbg !20559

bb1.i2121:                                        ; preds = %bb32, %bb10.i2135
  %iter.sroa.6.0.i2122 = phi i64 [ %len.i.i.i.i2127, %bb10.i2135 ], [ %frames, %bb32 ], !dbg !20560
  %iter.sroa.0.0.i2123 = phi ptr [ %data.i.i.i.i2126, %bb10.i2135 ], [ %left_io.0, %bb32 ], !dbg !20560
  %486 = icmp eq i64 %iter.sroa.6.0.i2122, 0, !dbg !20562
  br i1 %486, label %bb34, label %bb11.preheader.i2124, !dbg !20562

bb11.preheader.i2124:                             ; preds = %bb1.i2121
  %..i.i.i2125 = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i2122, i64 32), !dbg !20564
  %_18.idx.i2128 = shl nuw nsw i64 %..i.i.i2125, 2, !dbg !20567
  %_18.i2129 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i2123, i64 %_18.idx.i2128, !dbg !20567
  br label %bb11.i2130, !dbg !20572

bb11.i2130:                                       ; preds = %bb11.i2130, %bb11.preheader.i2124
  %iter1.sroa.0.014.i2131 = phi ptr [ %_31.i2133, %bb11.i2130 ], [ %iter.sroa.0.0.i2123, %bb11.preheader.i2124 ]
  %bits.sroa.0.013.i2132 = phi i32 [ %487, %bb11.i2130 ], [ 0, %bb11.preheader.i2124 ]
  %_31.i2133 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i2131, i64 4, !dbg !20574
  %_134.i2134 = load i32, ptr %iter1.sroa.0.014.i2131, align 4, !dbg !20576, !alias.scope !20577, !noundef !12
  %487 = or i32 %_134.i2134, %bits.sroa.0.013.i2132, !dbg !20580
  %_25.i = icmp eq ptr %_31.i2133, %_18.i2129, !dbg !20581
  br i1 %_25.i, label %bb10.i2135, label %bb11.i2130, !dbg !20572

bb10.i2135:                                       ; preds = %bb11.i2130
  %data.i.i.i.i2126 = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i2123, i64 %..i.i.i2125, !dbg !20583
  %len.i.i.i.i2127 = sub nuw nsw i64 %iter.sroa.6.0.i2122, %..i.i.i2125, !dbg !20588
  %488 = icmp eq i32 %487, 0, !dbg !20589
  br i1 %488, label %bb1.i2121, label %bb40, !dbg !20589

bb34:                                             ; preds = %bb1.i2121
  %_88.not = icmp ugt i64 %frames, %right_io.1, !dbg !20590
  br i1 %_88.not, label %bb54, label %bb1.i2150, !dbg !20590, !prof !905

bb40:                                             ; preds = %bb10.i2135, %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockfEB2_.exit, %bb28, %bb30, %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit2166
  %_36.sroa.0.0 = phi i8 [ %517, %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit2166 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockfEB2_.exit ], [ 0, %bb30 ], [ 0, %bb28 ], [ 0, %bb10.i2135 ], !dbg !20596
  store i8 %_36.sroa.0.0, ptr %38, align 4, !dbg !20597
  %489 = load i8, ptr %2, align 8, !dbg !20598, !range !5399, !noundef !12
  store i8 %489, ptr %0, align 1, !dbg !20599
  call void @llvm.lifetime.start.p0(ptr nonnull %shape), !dbg !20600
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %shape, ptr noundef nonnull align 8 dereferenceable(24) %_32, i64 24, i1 false), !dbg !20601
  %490 = getelementptr inbounds nuw i8, ptr %self, i64 72, !dbg !20602
  %491 = load i32, ptr %490, align 8, !dbg !20602, !noundef !12
  %_53 = getelementptr inbounds nuw i8, ptr %self, i64 568, !dbg !20604
  %492 = getelementptr inbounds nuw i8, ptr %self, i64 104, !dbg !20611
  %_99.0 = load ptr, ptr %492, align 8, !dbg !20611, !nonnull !12, !noundef !12
  %493 = getelementptr inbounds nuw i8, ptr %self, i64 112, !dbg !20611
  %_99.1 = load i64, ptr %493, align 8, !dbg !20611, !noundef !12
  %494 = getelementptr inbounds nuw i8, ptr %self, i64 120, !dbg !20611
  %_100.0 = load ptr, ptr %494, align 8, !dbg !20611, !nonnull !12, !noundef !12
  %495 = getelementptr inbounds nuw i8, ptr %self, i64 128, !dbg !20611
  %_100.1 = load i64, ptr %495, align 8, !dbg !20611, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20612), !dbg !20615
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20616), !dbg !20615
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20618), !dbg !20615
  %_22.not.i1463.i = icmp eq i64 %left_io.1, 0, !dbg !20620
  br i1 %_22.not.i1463.i, label %bb6.i.preheader.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i, !dbg !20620

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i: ; preds = %bb40, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i
  %ok.sroa.0.0.i1366.i = phi i32 [ %_0.i33.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i ], [ -1, %bb40 ]
  %iter.sroa.0.0.i1265.i = phi ptr [ %_27.i16.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i ], [ %left_io.0, %bb40 ]
  %iter.sroa.5.0.i1164.i = phi i64 [ %_28.i17.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i ], [ %left_io.1, %bb40 ]
  %_27.i16.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i1265.i, i64 4, !dbg !20633
  %_28.i17.i = add nsw i64 %iter.sroa.5.0.i1164.i, -1, !dbg !20640
  %_0.i28.i = load float, ptr %iter.sroa.0.0.i1265.i, align 4, !dbg !20641, !alias.scope !20644, !noalias !20647, !noundef !12
  %496 = tail call noundef float @llvm.fabs.f32(float %_0.i28.i), !dbg !20649
  %_3.i.i2137 = fcmp olt float %496, 0x46293E5940000000, !dbg !20652
  %_0.i33.i = select i1 %_3.i.i2137, i32 %ok.sroa.0.0.i1366.i, i32 0, !dbg !20655
  %_22.not.i14.i = icmp eq i64 %_28.i17.i, 0, !dbg !20620
  br i1 %_22.not.i14.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdvPQf9CMsz3_17true_peak_limiter.exit25.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i, !dbg !20620

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdvPQf9CMsz3_17true_peak_limiter.exit25.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i
  %497 = icmp eq i32 %_0.i33.i, -1, !dbg !20658
  br i1 %497, label %bb6.i.preheader.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i, !dbg !20661

bb6.i.preheader.i:                                ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdvPQf9CMsz3_17true_peak_limiter.exit25.i, %bb40
  %_22.not.i67.i = icmp eq i64 %right_io.1, 0, !dbg !20662
  br i1 %_22.not.i67.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockfNCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i, !dbg !20662

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i: ; preds = %bb6.i.preheader.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i
  %ok.sroa.0.0.i70.i = phi i32 [ %_0.i34.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i ], [ -1, %bb6.i.preheader.i ]
  %iter.sroa.0.0.i69.i = phi ptr [ %_27.i.i2148, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i ], [ %right_io.0, %bb6.i.preheader.i ]
  %iter.sroa.5.0.i68.i = phi i64 [ %_28.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i ], [ %right_io.1, %bb6.i.preheader.i ]
  %_27.i.i2148 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i69.i, i64 4, !dbg !20666
  %_28.i.i = add nsw i64 %iter.sroa.5.0.i68.i, -1, !dbg !20669
  %_0.i30.i = load float, ptr %iter.sroa.0.0.i69.i, align 4, !dbg !20670, !alias.scope !20672, !noalias !20675, !noundef !12
  %498 = tail call noundef float @llvm.fabs.f32(float %_0.i30.i), !dbg !20676
  %_3.i26.i = fcmp olt float %498, 0x46293E5940000000, !dbg !20678
  %_0.i34.i = select i1 %_3.i26.i, i32 %ok.sroa.0.0.i70.i, i32 0, !dbg !20680
  %_22.not.i.i = icmp eq i64 %_28.i.i, 0, !dbg !20662
  br i1 %_22.not.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i, !dbg !20662

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdvPQf9CMsz3_17true_peak_limiter.exit.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i
  %499 = icmp eq i32 %_0.i34.i, -1, !dbg !20682
  br i1 %499, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockfNCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit, label %bb7.i2149, !dbg !20684

bb7.i2149:                                        ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdvPQf9CMsz3_17true_peak_limiter.exit.i
  br i1 %_22.not.i1463.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i, !dbg !20685

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i: ; preds = %bb7.i2149, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdvPQf9CMsz3_17true_peak_limiter.exit25.i
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i, !dbg !20685

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i
  %ok.sroa.0.017.i.i = phi i32 [ %_0.i7.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ -1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i ]
  %iter.sroa.0.016.i.i = phi ptr [ %_45.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ %left_io.0, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i ]
  %iter.sroa.5.015.i.i = phi i64 [ %_46.i.i2138, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ %left_io.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i ]
  %_45.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.016.i.i, i64 4, !dbg !20696
  %_46.i.i2138 = add nsw i64 %iter.sroa.5.015.i.i, -1, !dbg !20703
  %_0.i.i.i2139 = load float, ptr %iter.sroa.0.016.i.i, align 4, !dbg !20704, !alias.scope !20707, !noalias !20647, !noundef !12
  %500 = tail call noundef float @llvm.fabs.f32(float %_0.i.i.i2139), !dbg !20712
  %_3.i.i.i2140 = fcmp olt float %500, 0x46293E5940000000, !dbg !20715
  %_0.i7.i.i = select i1 %_3.i.i.i2140, i32 %ok.sroa.0.017.i.i, i32 0, !dbg !20717
  %_40.not.i.i = icmp eq i64 %_46.i.i2138, 0, !dbg !20685
  br i1 %_40.not.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i, !dbg !20685

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i
  %501 = and i32 %_0.i7.i.i, 1065353216, !dbg !20719
  %502 = icmp ne i32 %501, 1065353216, !dbg !20722
  %503 = zext i1 %502 to i32, !dbg !20722
  %_40.not14.i40.i = icmp eq i64 %right_io.1, 0, !dbg !20726
  br i1 %_40.not14.i40.i, label %bb14.i.preheader.thread.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i, !dbg !20726

bb14.i.preheader.thread.i:                        ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit.i
  %504 = getelementptr inbounds nuw i8, ptr %self, i64 576, !dbg !20730
  store i32 %503, ptr %504, align 8, !dbg !20730, !alias.scope !20618, !noalias !20731
  %_1481.i = load i64, ptr %_53, align 8, !dbg !20732, !alias.scope !20618, !noalias !20731, !noundef !12
  %505 = tail call i64 @llvm.uadd.sat.i64(i64 %_1481.i, i64 1), !dbg !20733
  store i64 %505, ptr %_53, align 8, !dbg !20736, !alias.scope !20618, !noalias !20731
  br label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit60.i, !dbg !20737

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i: ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit.i, %bb7.i2149
  %ok.sroa.0.0.lcssa.i76.i = phi i32 [ %503, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit.i ], [ 0, %bb7.i2149 ]
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.i, !dbg !20726

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i
  %ok.sroa.0.017.i42.i = phi i32 [ %_0.i7.i49.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.i ], [ -1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i ]
  %iter.sroa.0.016.i43.i = phi ptr [ %_45.i45.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.i ], [ %right_io.0, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i ]
  %iter.sroa.5.015.i44.i = phi i64 [ %_46.i46.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.i ], [ %right_io.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i ]
  %_45.i45.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.016.i43.i, i64 4, !dbg !20742
  %_46.i46.i = add nsw i64 %iter.sroa.5.015.i44.i, -1, !dbg !20745
  %_0.i.i47.i = load float, ptr %iter.sroa.0.016.i43.i, align 4, !dbg !20746, !alias.scope !20748, !noalias !20675, !noundef !12
  %506 = tail call noundef float @llvm.fabs.f32(float %_0.i.i47.i), !dbg !20753
  %_3.i.i48.i = fcmp olt float %506, 0x46293E5940000000, !dbg !20755
  %_0.i7.i49.i = select i1 %_3.i.i48.i, i32 %ok.sroa.0.017.i42.i, i32 0, !dbg !20757
  %_40.not.i50.i = icmp eq i64 %_46.i46.i, 0, !dbg !20726
  br i1 %_40.not.i50.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit53.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.i, !dbg !20726

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit53.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.i
  %507 = and i32 %_0.i7.i49.i, 1065353216, !dbg !20759
  %508 = icmp ne i32 %507, 1065353216, !dbg !20761
  %509 = zext i1 %508 to i32, !dbg !20761
  %510 = or i32 %ok.sroa.0.0.lcssa.i76.i, %509, !dbg !20730
  %511 = getelementptr inbounds nuw i8, ptr %self, i64 576, !dbg !20730
  store i32 %510, ptr %511, align 8, !dbg !20730, !alias.scope !20618, !noalias !20731
  %_14.i2141 = load i64, ptr %_53, align 8, !dbg !20732, !alias.scope !20618, !noalias !20731, !noundef !12
  %512 = tail call i64 @llvm.uadd.sat.i64(i64 %_14.i2141, i64 1), !dbg !20733
  store i64 %512, ptr %_53, align 8, !dbg !20736, !alias.scope !20618, !noalias !20731
  br i1 %_22.not.i1463.i, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit60.i, label %bb14.i.preheader.i, !dbg !20762

bb14.i.preheader.i:                               ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit53.i
  %.idx.i.i = shl nuw nsw i64 %left_io.1, 2, !dbg !20766
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %left_io.0, i8 0, i64 %.idx.i.i, i1 false), !dbg !20770, !alias.scope !20771, !noalias !20647
  br label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit60.i, !dbg !20737

_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit60.i: ; preds = %bb14.i.preheader.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit53.i, %bb14.i.preheader.thread.i
  %left.1.sink.i = phi i64 [ %left_io.1, %bb14.i.preheader.thread.i ], [ %right_io.1, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit53.i ], [ %right_io.1, %bb14.i.preheader.i ]
  %left.0.sink.i = phi ptr [ %left_io.0, %bb14.i.preheader.thread.i ], [ %right_io.0, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit53.i ], [ %right_io.0, %bb14.i.preheader.i ]
  %.idx.i85.i = shl nuw nsw i64 %left.1.sink.i, 2, !dbg !20774
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %left.0.sink.i, i8 0, i64 %.idx.i85.i, i1 false), !dbg !20780, !alias.scope !20781, !noalias !20782
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 8 dereferenceable(200) %_33, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_99.0, i64 noundef %_99.1, i32 noundef %491) #30, !dbg !20783, !noalias !20786
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 8 dereferenceable(200) %_34, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_100.0, i64 noundef %_100.1, i32 noundef %491) #30, !dbg !20789, !noalias !20786
  store i32 0, ptr %_35, align 4, !dbg !20790, !noalias !20786
  %513 = getelementptr inbounds nuw i8, ptr %self, i64 564, !dbg !20790
  store i32 0, ptr %513, align 4, !dbg !20790, !noalias !20786
  br label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockfNCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit, !dbg !20791

_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockfNCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit: ; preds = %bb6.i.preheader.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdvPQf9CMsz3_17true_peak_limiter.exit.i, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit60.i
  call void @llvm.lifetime.end.p0(ptr nonnull %shape), !dbg !20792
  br label %bb42, !dbg !20546

bb54:                                             ; preds = %bb34
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %frames, i64 noundef %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_983c0eafc7ebd40f6695894c941b193b) #31, !dbg !20793
  unreachable, !dbg !20793

bb1.i2150:                                        ; preds = %bb34, %bb10.i2165
  %iter.sroa.6.0.i2151 = phi i64 [ %len.i.i.i.i2156, %bb10.i2165 ], [ %frames, %bb34 ], !dbg !20794
  %iter.sroa.0.0.i2152 = phi ptr [ %data.i.i.i.i2155, %bb10.i2165 ], [ %right_io.0, %bb34 ], !dbg !20794
  %514 = icmp eq i64 %iter.sroa.6.0.i2151, 0, !dbg !20796
  br i1 %514, label %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit2166, label %bb11.preheader.i2153, !dbg !20796

bb11.preheader.i2153:                             ; preds = %bb1.i2150
  %..i.i.i2154 = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i2151, i64 32), !dbg !20798
  %_18.idx.i2157 = shl nuw nsw i64 %..i.i.i2154, 2, !dbg !20801
  %_18.i2158 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i2152, i64 %_18.idx.i2157, !dbg !20801
  br label %bb11.i2159, !dbg !20806

bb11.i2159:                                       ; preds = %bb11.i2159, %bb11.preheader.i2153
  %iter1.sroa.0.014.i2160 = phi ptr [ %_31.i2162, %bb11.i2159 ], [ %iter.sroa.0.0.i2152, %bb11.preheader.i2153 ]
  %bits.sroa.0.013.i2161 = phi i32 [ %515, %bb11.i2159 ], [ 0, %bb11.preheader.i2153 ]
  %_31.i2162 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i2160, i64 4, !dbg !20808
  %_134.i2163 = load i32, ptr %iter1.sroa.0.014.i2160, align 4, !dbg !20810, !alias.scope !20811, !noundef !12
  %515 = or i32 %_134.i2163, %bits.sroa.0.013.i2161, !dbg !20814
  %_25.i2164 = icmp eq ptr %_31.i2162, %_18.i2158, !dbg !20815
  br i1 %_25.i2164, label %bb10.i2165, label %bb11.i2159, !dbg !20806

bb10.i2165:                                       ; preds = %bb11.i2159
  %data.i.i.i.i2155 = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i2152, i64 %..i.i.i2154, !dbg !20817
  %len.i.i.i.i2156 = sub nuw nsw i64 %iter.sroa.6.0.i2151, %..i.i.i2154, !dbg !20822
  %516 = icmp eq i32 %515, 0, !dbg !20823
  br i1 %516, label %bb1.i2150, label %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit2166, !dbg !20823

_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit2166: ; preds = %bb1.i2150, %bb10.i2165
  %517 = zext i1 %514 to i8, !dbg !20597
  br label %bb40, !dbg !20471

bb42:                                             ; preds = %_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockfNCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit
  ret void, !dbg !20546
}
