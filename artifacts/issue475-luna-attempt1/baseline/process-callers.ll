define internal fastcc void @_RINvMCse3bfmKSZS8Y_10compressorINtB3_8InstanceNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6renderNCINvMs3_B3_INtB3_22PreparedCompressorBankBI_E18process_bank_innerKb0_E0EB3_(ptr noalias noundef nonnull align 32 dereferenceable(3328) %self, ptr noalias noundef nonnull align 4 captures(address) %left.0, i64 noundef range(i64 8, 34359738361) %left.1, ptr noalias noundef nonnull align 4 captures(address) %right.0, i64 noundef range(i64 8, 34359738361) %right.1, ptr dead_on_return noalias noundef nonnull readonly align 8 captures(none) dereferenceable(40) %detector, i64 noundef range(i64 1, 4294967296) %frames, ptr noalias noundef nonnull align 8 captures(none) dereferenceable(328) %0) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !212 {
start:
  %_101 = alloca [64 x i8], align 4
  %_45 = alloca [32 x i8], align 4
  %_40 = alloca [32 x i8], align 4
  %right_before = alloca [32 x i8], align 4
  %left_before = alloca [32 x i8], align 4
  %before.sroa.6 = alloca [64 x i8], align 4
  %words = shl nuw nsw i64 %frames, 3, !dbg !214
  %_10 = getelementptr inbounds nuw i8, ptr %self, i64 64, !dbg !215
; call <compressor::kernel::Channel<wide::f32x8_::f32x8>>::max_remaining
  %_9 = tail call fastcc noundef i32 @_RNvMNtCse3bfmKSZS8Y_10compressor6kernelINtB2_7ChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13max_remainingB4_(ptr noalias noundef readonly align 32 captures(address, read_provenance) dereferenceable(1568) %_10) #22, !dbg !217
  %1 = icmp eq i32 %_9, 0, !dbg !215
  br i1 %1, label %bb2, label %bb14, !dbg !215

bb2:                                              ; preds = %start
  %_12 = getelementptr inbounds nuw i8, ptr %self, i64 1632, !dbg !218
; call <compressor::kernel::Channel<wide::f32x8_::f32x8>>::max_remaining
  %_11 = tail call fastcc noundef i32 @_RNvMNtCse3bfmKSZS8Y_10compressor6kernelINtB2_7ChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13max_remainingB4_(ptr noalias noundef readonly align 32 captures(address, read_provenance) dereferenceable(1568) %_12) #22, !dbg !219
  %2 = icmp ne i32 %_11, 0, !dbg !218
  %_13 = load i64, ptr %detector, align 8, !range !220
  %3 = icmp eq i64 %_13, 2
  %or.cond19 = select i1 %2, i1 true, i1 %3, !dbg !218
  br i1 %or.cond19, label %bb14, label %bb6, !dbg !218

bb6:                                              ; preds = %bb2
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 3305, !dbg !221
  %5 = load i8, ptr %4, align 1, !dbg !221, !range !222, !noundef !11
  %6 = getelementptr inbounds nuw i8, ptr %self, i64 3296, !dbg !223
  %7 = load i8, ptr %6, align 32, !dbg !223, !range !222, !noundef !11
  %_14 = icmp eq i8 %5, %7, !dbg !221
  br i1 %_14, label %bb7, label %bb14, !dbg !221

bb7:                                              ; preds = %bb6
  %_70.not = icmp samesign ugt i64 %words, %left.1
  br i1 %_70.not, label %bb47, label %bb1.i, !dbg !224, !prof !239

bb14:                                             ; preds = %bb10.i, %bb6, %bb2, %start
  call void @llvm.lifetime.start.p0(ptr nonnull %before.sroa.6), !dbg !240
  %.phi.trans.insert = getelementptr inbounds nuw i8, ptr %self, i64 3296
  %.pre = load i8, ptr %.phi.trans.insert, align 32, !dbg !242, !range !222
  br label %bb62, !dbg !244

bb47:                                             ; preds = %bb7
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %words, i64 noundef %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e3a36baac19ecbb60e781bce6e2cb5c2) #23, !dbg !250
  unreachable, !dbg !250

bb1.i:                                            ; preds = %bb7, %bb10.i
  %iter.sroa.6.0.i = phi i64 [ %len.i.i.i.i, %bb10.i ], [ %words, %bb7 ], !dbg !251
  %iter.sroa.0.0.i = phi ptr [ %data.i.i.i.i, %bb10.i ], [ %left.0, %bb7 ], !dbg !251
  %8 = icmp eq i64 %iter.sroa.6.0.i, 0, !dbg !256
  br i1 %8, label %bb9, label %bb11.preheader.i, !dbg !256

bb11.preheader.i:                                 ; preds = %bb1.i
  %..i.i.i = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i, i64 32), !dbg !261
  %_18.idx.i = shl nuw nsw i64 %..i.i.i, 2, !dbg !269
  %_18.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i, i64 %_18.idx.i, !dbg !269
  br label %bb11.i, !dbg !283

bb11.i:                                           ; preds = %bb11.i, %bb11.preheader.i
  %iter1.sroa.0.014.i = phi ptr [ %_31.i, %bb11.i ], [ %iter.sroa.0.0.i, %bb11.preheader.i ]
  %bits.sroa.0.013.i = phi i32 [ %9, %bb11.i ], [ 0, %bb11.preheader.i ]
  %_31.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i, i64 4, !dbg !290
  %_134.i = load i32, ptr %iter1.sroa.0.014.i, align 4, !dbg !293, !alias.scope !295, !noundef !11
  %9 = or i32 %_134.i, %bits.sroa.0.013.i, !dbg !298
  %_25.i = icmp eq ptr %_31.i, %_18.i, !dbg !299
  br i1 %_25.i, label %bb10.i, label %bb11.i, !dbg !283

bb10.i:                                           ; preds = %bb11.i
  %data.i.i.i.i = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i, i64 %..i.i.i, !dbg !302
  %len.i.i.i.i = sub nuw nsw i64 %iter.sroa.6.0.i, %..i.i.i, !dbg !317
  %10 = icmp eq i32 %9, 0, !dbg !318
  br i1 %10, label %bb1.i, label %bb14, !dbg !318

bb9:                                              ; preds = %bb1.i
  %_78.not = icmp samesign ugt i64 %words, %right.1, !dbg !319
  br i1 %_78.not, label %bb50, label %bb1.i20, !dbg !319, !prof !161

bb50:                                             ; preds = %bb9
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %words, i64 noundef %right.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_afe388ffec419f4102be11616e3acf97) #23, !dbg !325
  unreachable, !dbg !325

bb1.i20:                                          ; preds = %bb9, %bb10.i35
  %iter.sroa.6.0.i21 = phi i64 [ %len.i.i.i.i26, %bb10.i35 ], [ %words, %bb9 ], !dbg !326
  %iter.sroa.0.0.i22 = phi ptr [ %data.i.i.i.i25, %bb10.i35 ], [ %right.0, %bb9 ], !dbg !326
  %11 = icmp eq i64 %iter.sroa.6.0.i21, 0, !dbg !328
  br i1 %11, label %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit36, label %bb11.preheader.i23, !dbg !328

bb11.preheader.i23:                               ; preds = %bb1.i20
  %..i.i.i24 = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i21, i64 32), !dbg !330
  %_18.idx.i27 = shl nuw nsw i64 %..i.i.i24, 2, !dbg !333
  %_18.i28 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i22, i64 %_18.idx.i27, !dbg !333
  br label %bb11.i29, !dbg !338

bb11.i29:                                         ; preds = %bb11.i29, %bb11.preheader.i23
  %iter1.sroa.0.014.i30 = phi ptr [ %_31.i32, %bb11.i29 ], [ %iter.sroa.0.0.i22, %bb11.preheader.i23 ]
  %bits.sroa.0.013.i31 = phi i32 [ %12, %bb11.i29 ], [ 0, %bb11.preheader.i23 ]
  %_31.i32 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i30, i64 4, !dbg !340
  %_134.i33 = load i32, ptr %iter1.sroa.0.014.i30, align 4, !dbg !342, !alias.scope !343, !noundef !11
  %12 = or i32 %_134.i33, %bits.sroa.0.013.i31, !dbg !346
  %_25.i34 = icmp eq ptr %_31.i32, %_18.i28, !dbg !347
  br i1 %_25.i34, label %bb10.i35, label %bb11.i29, !dbg !338

bb10.i35:                                         ; preds = %bb11.i29
  %data.i.i.i.i25 = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i22, i64 %..i.i.i24, !dbg !349
  %len.i.i.i.i26 = sub nuw nsw i64 %iter.sroa.6.0.i21, %..i.i.i24, !dbg !354
  %13 = icmp eq i32 %12, 0, !dbg !355
  br i1 %13, label %bb1.i20, label %bb18, !dbg !355

_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit36: ; preds = %bb1.i20
  %14 = getelementptr inbounds nuw i8, ptr %self, i64 3304
  %15 = load i8, ptr %14, align 8, !range !222
  %_21 = trunc nuw i8 %15 to i1
  br i1 %_21, label %bb17, label %bb61, !dbg !356

bb18:                                             ; preds = %bb10.i35
  call void @llvm.lifetime.start.p0(ptr nonnull %before.sroa.6), !dbg !240
  br label %bb62, !dbg !244

bb17:                                             ; preds = %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit36
  %_22 = trunc nuw i64 %frames to i32, !dbg !357
  %16 = getelementptr inbounds nuw i8, ptr %self, i64 1604, !dbg !358
  %_85 = load i32, ptr %16, align 4, !dbg !358, !noundef !11
  %17 = icmp eq i32 %_85, 0, !dbg !358
  br i1 %17, label %bb51, label %bb53, !dbg !358

bb62:                                             ; preds = %bb18, %bb14
  %18 = phi i8 [ %5, %bb18 ], [ %.pre, %bb14 ], !dbg !242
  %19 = getelementptr inbounds nuw i8, ptr %self, i64 3268, !dbg !364
  %20 = load i32, ptr %19, align 4, !dbg !364, !range !65, !noundef !11
  %21 = trunc nuw i8 %18 to i1, !dbg !242
  %22 = getelementptr inbounds nuw i8, ptr %self, i64 3272, !dbg !365
  %23 = load i32, ptr %22, align 8, !dbg !365, !noundef !11
  %24 = getelementptr inbounds nuw i8, ptr %self, i64 1632, !dbg !366
; call compressor::kernel::process_block::<wide::f32x8_::f32x8>
  tail call fastcc void @_RINvNtCse3bfmKSZS8Y_10compressor6kernel13process_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_(ptr noalias noundef nonnull align 4 %left.0, i64 noundef %left.1, ptr noalias noundef nonnull align 4 %right.0, i64 noundef %right.1, ptr noalias noundef align 8 captures(address) dereferenceable(40) %detector, i64 noundef %frames, i32 noundef %20, i1 noundef zeroext %21, i32 noundef %23, ptr noalias noundef align 32 dereferenceable(1568) %_10, ptr noalias noundef align 32 dereferenceable(1568) %24, ptr noalias noundef align 8 dereferenceable(64) %self) #22, !dbg !367
  br label %bb38, !dbg !368

bb61:                                             ; preds = %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit36
  call void @llvm.lifetime.start.p0(ptr nonnull %before.sroa.6), !dbg !240
  call void @llvm.lifetime.start.p0(ptr nonnull %_101), !dbg !369
  %25 = getelementptr inbounds nuw i8, ptr %self, i64 1536, !dbg !370
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_101, ptr noundef nonnull readonly align 32 dereferenceable(32) %25, i64 32, i1 false), !dbg !377
  %26 = getelementptr inbounds nuw i8, ptr %self, i64 3104, !dbg !378
  %27 = getelementptr inbounds nuw i8, ptr %_101, i64 32, !dbg !380
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %27, ptr noundef nonnull readonly align 32 dereferenceable(32) %26, i64 32, i1 false), !dbg !381
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(64) %before.sroa.6, ptr noundef nonnull align 4 dereferenceable(64) %_101, i64 64, i1 false), !dbg !382
  call void @llvm.lifetime.end.p0(ptr nonnull %_101), !dbg !383
  %28 = getelementptr inbounds nuw i8, ptr %self, i64 3268, !dbg !364
  %29 = load i32, ptr %28, align 4, !dbg !364, !range !65, !noundef !11
  %30 = trunc nuw i8 %5 to i1, !dbg !242
  %31 = getelementptr inbounds nuw i8, ptr %self, i64 3272, !dbg !365
  %32 = load i32, ptr %31, align 8, !dbg !365, !noundef !11
; call compressor::kernel::process_block::<wide::f32x8_::f32x8>
  tail call fastcc void @_RINvNtCse3bfmKSZS8Y_10compressor6kernel13process_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_(ptr noalias noundef nonnull align 4 %left.0, i64 noundef %left.1, ptr noalias noundef nonnull align 4 %right.0, i64 noundef %right.1, ptr noalias noundef align 8 captures(address) dereferenceable(40) %detector, i64 noundef %frames, i32 noundef %29, i1 noundef zeroext %30, i32 noundef %32, ptr noalias noundef align 32 dereferenceable(1568) %_10, ptr noalias noundef align 32 dereferenceable(1568) %_12, ptr noalias noundef align 8 dereferenceable(64) %self) #22, !dbg !367
  call void @llvm.lifetime.start.p0(ptr nonnull %left_before), !dbg !384
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %left_before, ptr noundef nonnull align 4 dereferenceable(32) %before.sroa.6, i64 32, i1 false), !dbg !384
  call void @llvm.lifetime.start.p0(ptr nonnull %right_before), !dbg !385
  %before.sroa.6.36..sroa_idx = getelementptr inbounds nuw i8, ptr %before.sroa.6, i64 32, !dbg !385
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %right_before, ptr noundef nonnull align 4 dereferenceable(32) %before.sroa.6.36..sroa_idx, i64 32, i1 false), !dbg !385
  call void @llvm.lifetime.start.p0(ptr nonnull %_40), !dbg !386
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_40, ptr noundef nonnull readonly align 32 dereferenceable(32) %25, i64 32, i1 false), !dbg !388, !alias.scope !390
  %bcmp = call i32 @bcmp(ptr noundef nonnull dereferenceable(32) %left_before, ptr noundef nonnull dereferenceable(32) %_40, i64 32), !dbg !394
  %33 = icmp eq i32 %bcmp, 0, !dbg !394
  call void @llvm.lifetime.end.p0(ptr nonnull %_40), !dbg !404
  br i1 %33, label %bb21, label %bb37, !dbg !403

bb21:                                             ; preds = %bb61
  call void @llvm.lifetime.start.p0(ptr nonnull %_45), !dbg !405
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_45, ptr noundef nonnull readonly align 32 dereferenceable(32) %26, i64 32, i1 false), !dbg !406, !alias.scope !408
  %bcmp18 = call i32 @bcmp(ptr noundef nonnull dereferenceable(32) %right_before, ptr noundef nonnull dereferenceable(32) %_45, i64 32), !dbg !412
  %34 = icmp eq i32 %bcmp18, 0, !dbg !412
  call void @llvm.lifetime.end.p0(ptr nonnull %_45), !dbg !416
  br i1 %34, label %bb23, label %bb37, !dbg !415

bb23:                                             ; preds = %bb21
; call <compressor::kernel::Channel<wide::f32x8_::f32x8>>::rings_are_positive_zero
  %_47 = tail call fastcc noundef zeroext i1 @_RNvMNtCse3bfmKSZS8Y_10compressor6kernelINtB2_7ChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E23rings_are_positive_zeroB4_(ptr noalias noundef readonly align 32 captures(address, read_provenance) dereferenceable(1568) %_10) #22, !dbg !417
  br i1 %_47, label %bb25, label %bb37, !dbg !418

bb25:                                             ; preds = %bb23
; call <compressor::kernel::Channel<wide::f32x8_::f32x8>>::rings_are_positive_zero
  %_49 = tail call fastcc noundef zeroext i1 @_RNvMNtCse3bfmKSZS8Y_10compressor6kernelINtB2_7ChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E23rings_are_positive_zeroB4_(ptr noalias noundef readonly align 32 captures(address, read_provenance) dereferenceable(1568) %_12) #22, !dbg !419
  br i1 %_49, label %bb1.i37, label %bb37, !dbg !420

bb1.i37:                                          ; preds = %bb25, %bb10.i52
  %iter.sroa.6.0.i38 = phi i64 [ %len.i.i.i.i43, %bb10.i52 ], [ %words, %bb25 ], !dbg !421
  %iter.sroa.0.0.i39 = phi ptr [ %data.i.i.i.i42, %bb10.i52 ], [ %left.0, %bb25 ], !dbg !421
  %35 = icmp eq i64 %iter.sroa.6.0.i38, 0, !dbg !423
  br i1 %35, label %bb1.i54, label %bb11.preheader.i40, !dbg !423

bb11.preheader.i40:                               ; preds = %bb1.i37
  %..i.i.i41 = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i38, i64 32), !dbg !425
  %_18.idx.i44 = shl nuw nsw i64 %..i.i.i41, 2, !dbg !428
  %_18.i45 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i39, i64 %_18.idx.i44, !dbg !428
  br label %bb11.i46, !dbg !433

bb11.i46:                                         ; preds = %bb11.i46, %bb11.preheader.i40
  %iter1.sroa.0.014.i47 = phi ptr [ %_31.i49, %bb11.i46 ], [ %iter.sroa.0.0.i39, %bb11.preheader.i40 ]
  %bits.sroa.0.013.i48 = phi i32 [ %36, %bb11.i46 ], [ 0, %bb11.preheader.i40 ]
  %_31.i49 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i47, i64 4, !dbg !435
  %_134.i50 = load i32, ptr %iter1.sroa.0.014.i47, align 4, !dbg !437, !alias.scope !438, !noundef !11
  %36 = or i32 %_134.i50, %bits.sroa.0.013.i48, !dbg !441
  %_25.i51 = icmp eq ptr %_31.i49, %_18.i45, !dbg !442
  br i1 %_25.i51, label %bb10.i52, label %bb11.i46, !dbg !433

bb10.i52:                                         ; preds = %bb11.i46
  %data.i.i.i.i42 = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i39, i64 %..i.i.i41, !dbg !444
  %len.i.i.i.i43 = sub nuw nsw i64 %iter.sroa.6.0.i38, %..i.i.i41, !dbg !449
  %37 = icmp eq i32 %36, 0, !dbg !450
  br i1 %37, label %bb1.i37, label %bb37, !dbg !450

bb37:                                             ; preds = %bb10.i52, %bb21, %bb61, %bb23, %bb25, %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit70
  %_34.sroa.0.0 = phi i8 [ %41, %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit70 ], [ 0, %bb21 ], [ 0, %bb25 ], [ 0, %bb23 ], [ 0, %bb61 ], [ 0, %bb10.i52 ], !dbg !451
  call void @llvm.lifetime.end.p0(ptr nonnull %right_before), !dbg !452
  call void @llvm.lifetime.end.p0(ptr nonnull %left_before), !dbg !452
  br label %bb38, !dbg !452

bb1.i54:                                          ; preds = %bb1.i37, %bb10.i69
  %iter.sroa.6.0.i55 = phi i64 [ %len.i.i.i.i60, %bb10.i69 ], [ %words, %bb1.i37 ], !dbg !453
  %iter.sroa.0.0.i56 = phi ptr [ %data.i.i.i.i59, %bb10.i69 ], [ %right.0, %bb1.i37 ], !dbg !453
  %38 = icmp eq i64 %iter.sroa.6.0.i55, 0, !dbg !455
  br i1 %38, label %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit70, label %bb11.preheader.i57, !dbg !455

bb11.preheader.i57:                               ; preds = %bb1.i54
  %..i.i.i58 = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i55, i64 32), !dbg !457
  %_18.idx.i61 = shl nuw nsw i64 %..i.i.i58, 2, !dbg !460
  %_18.i62 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i56, i64 %_18.idx.i61, !dbg !460
  br label %bb11.i63, !dbg !465

bb11.i63:                                         ; preds = %bb11.i63, %bb11.preheader.i57
  %iter1.sroa.0.014.i64 = phi ptr [ %_31.i66, %bb11.i63 ], [ %iter.sroa.0.0.i56, %bb11.preheader.i57 ]
  %bits.sroa.0.013.i65 = phi i32 [ %39, %bb11.i63 ], [ 0, %bb11.preheader.i57 ]
  %_31.i66 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i64, i64 4, !dbg !467
  %_134.i67 = load i32, ptr %iter1.sroa.0.014.i64, align 4, !dbg !469, !alias.scope !470, !noundef !11
  %39 = or i32 %_134.i67, %bits.sroa.0.013.i65, !dbg !473
  %_25.i68 = icmp eq ptr %_31.i66, %_18.i62, !dbg !474
  br i1 %_25.i68, label %bb10.i69, label %bb11.i63, !dbg !465

bb10.i69:                                         ; preds = %bb11.i63
  %data.i.i.i.i59 = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i56, i64 %..i.i.i58, !dbg !476
  %len.i.i.i.i60 = sub nuw nsw i64 %iter.sroa.6.0.i55, %..i.i.i58, !dbg !481
  %40 = icmp eq i32 %39, 0, !dbg !482
  br i1 %40, label %bb1.i54, label %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit70, !dbg !482

_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit70: ; preds = %bb1.i54, %bb10.i69
  %41 = zext i1 %38 to i8, !dbg !483
  br label %bb37, !dbg !403

bb38:                                             ; preds = %bb62, %bb37
  %_34.sroa.0.1 = phi i8 [ %_34.sroa.0.0, %bb37 ], [ 0, %bb62 ], !dbg !484
  %42 = getelementptr inbounds nuw i8, ptr %self, i64 3304, !dbg !483
  store i8 %_34.sroa.0.1, ptr %42, align 8, !dbg !483
  %43 = getelementptr inbounds nuw i8, ptr %self, i64 3296, !dbg !485
  %44 = load i8, ptr %43, align 32, !dbg !485, !range !222, !noundef !11
  %45 = getelementptr inbounds nuw i8, ptr %self, i64 3305, !dbg !486
  store i8 %44, ptr %45, align 1, !dbg !486
  tail call void @llvm.experimental.noalias.scope.decl(metadata !487), !dbg !490
  tail call void @llvm.experimental.noalias.scope.decl(metadata !493), !dbg !490
  %46 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !495
  %fst_len.i.i = and i64 %left.1, 34359738360, !dbg !529
  %47 = bitcast <8 x float> %46 to <8 x i32>, !dbg !537
  br label %bb13.i.i

bb13.i.i:                                         ; preds = %bb13.i.i, %bb38
  %iter.sroa.0.0.i64.i = phi ptr [ %left.0, %bb38 ], [ %_27.i.i, %bb13.i.i ]
  %iter.sroa.5.0.i63.i = phi i64 [ %fst_len.i.i, %bb38 ], [ %_28.i.i, %bb13.i.i ]
  %ok.i.sroa.0.062.i = phi <8 x i32> [ %47, %bb38 ], [ %52, %bb13.i.i ]
  %lanes.i.sroa.0.0.copyload.i = load <8 x i32>, ptr %iter.sroa.0.0.i64.i, align 4, !dbg !548, !alias.scope !559, !noalias !563
  %_27.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i64.i, i64 32, !dbg !567
  %_28.i.i = add nsw i64 %iter.sroa.5.0.i63.i, -8, !dbg !580
  %48 = and <8 x i32> %lanes.i.sroa.0.0.copyload.i, splat (i32 2147483647), !dbg !581
  %49 = bitcast <8 x i32> %48 to <8 x float>, !dbg !595
  %50 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %49, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !596
  %51 = bitcast <8 x float> %50 to <8 x i32>, !dbg !537
  %52 = and <8 x i32> %ok.i.sroa.0.062.i, %51, !dbg !608
  %_22.not.i.i = icmp eq i64 %_28.i.i, 0, !dbg !610
  br i1 %_22.not.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i, label %bb13.i.i, !dbg !610

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i: ; preds = %bb13.i.i
  %53 = icmp sgt <8 x i32> %52, splat (i32 -1), !dbg !611
  %54 = bitcast <8 x i1> %53 to i8, !dbg !611
  %_0.i26.not.i = icmp eq i8 %54, 0, !dbg !621
  br i1 %_0.i26.not.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank14finish_channelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCINvNtCse3bfmKSZS8Y_10compressor6kernel14finish_channelBT_E0EB1A_.exit, label %bb23.i.i, !dbg !622

bb23.i.i:                                         ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i, %bb23.i.i
  %iter.sroa.0.074.i.i = phi ptr [ %_45.i.i, %bb23.i.i ], [ %left.0, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i ]
  %iter.sroa.5.073.i.i = phi i64 [ %_46.i.i, %bb23.i.i ], [ %fst_len.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i ]
  %ok.sroa.0.072.i.i = phi <8 x i32> [ %59, %bb23.i.i ], [ %47, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i ]
  %lanes.i.sroa.0.0.copyload.i.i = load <8 x i32>, ptr %iter.sroa.0.074.i.i, align 4, !dbg !623, !alias.scope !634, !noalias !640
  %_45.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.074.i.i, i64 32, !dbg !644
  %_46.i.i = add nsw i64 %iter.sroa.5.073.i.i, -8, !dbg !656
  %55 = and <8 x i32> %lanes.i.sroa.0.0.copyload.i.i, splat (i32 2147483647), !dbg !657
  %56 = bitcast <8 x i32> %55 to <8 x float>, !dbg !664
  %57 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %56, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !665
  %58 = bitcast <8 x float> %57 to <8 x i32>, !dbg !671
  %59 = and <8 x i32> %ok.sroa.0.072.i.i, %58, !dbg !675
  %_40.not.i.i = icmp eq i64 %_46.i.i, 0, !dbg !677
  br i1 %_40.not.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i, label %bb23.i.i, !dbg !677

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i: ; preds = %bb23.i.i
  %60 = icmp slt <8 x i32> %59, zeroinitializer, !dbg !678
  %bc.i.i = select <8 x i1> %60, <8 x i32> zeroinitializer, <8 x i32> splat (i32 1065353216), !dbg !678
  %61 = extractelement <8 x i32> %bc.i.i, i64 0, !dbg !689
  %62 = icmp ne i32 %61, 0, !dbg !689
  %63 = zext i1 %62 to i32, !dbg !689
  %64 = extractelement <8 x i32> %bc.i.i, i64 1, !dbg !689
  %65 = icmp eq i32 %64, 0, !dbg !689
  %66 = select i1 %65, i32 0, i32 2, !dbg !689
  %mask.sroa.0.1.1.i.i = or disjoint i32 %66, %63, !dbg !689
  %67 = extractelement <8 x i32> %bc.i.i, i64 2, !dbg !689
  %68 = icmp eq i32 %67, 0, !dbg !689
  %69 = select i1 %68, i32 0, i32 4, !dbg !689
  %mask.sroa.0.1.2.i.i = or disjoint i32 %mask.sroa.0.1.1.i.i, %69, !dbg !689
  %70 = extractelement <8 x i32> %bc.i.i, i64 3, !dbg !689
  %71 = icmp eq i32 %70, 0, !dbg !689
  %72 = select i1 %71, i32 0, i32 8, !dbg !689
  %mask.sroa.0.1.3.i.i = or disjoint i32 %mask.sroa.0.1.2.i.i, %72, !dbg !689
  %73 = extractelement <8 x i32> %bc.i.i, i64 4, !dbg !689
  %74 = icmp eq i32 %73, 0, !dbg !689
  %75 = select i1 %74, i32 0, i32 16, !dbg !689
  %mask.sroa.0.1.4.i.i = or disjoint i32 %mask.sroa.0.1.3.i.i, %75, !dbg !689
  %76 = extractelement <8 x i32> %bc.i.i, i64 5, !dbg !689
  %77 = icmp eq i32 %76, 0, !dbg !689
  %78 = select i1 %77, i32 0, i32 32, !dbg !689
  %mask.sroa.0.1.5.i.i = or disjoint i32 %mask.sroa.0.1.4.i.i, %78, !dbg !689
  %79 = extractelement <8 x i32> %bc.i.i, i64 6, !dbg !689
  %80 = icmp eq i32 %79, 0, !dbg !689
  %81 = select i1 %80, i32 0, i32 64, !dbg !689
  %mask.sroa.0.1.6.i.i = or i32 %mask.sroa.0.1.5.i.i, %81, !dbg !689
  %82 = extractelement <8 x i32> %bc.i.i, i64 7, !dbg !689
  %83 = icmp eq i32 %82, 0, !dbg !689
  %84 = select i1 %83, i32 0, i32 128, !dbg !689
  %mask.sroa.0.1.7.i.i = or i32 %mask.sroa.0.1.6.i.i, %84, !dbg !689
  %.idx.i.i = shl nuw nsw i64 %left.1, 2, !dbg !693
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1) %left.0, i8 0, i64 %.idx.i.i, i1 false), !dbg !711, !alias.scope !714, !noalias !493
  tail call void @llvm.experimental.noalias.scope.decl(metadata !717), !dbg !720
  tail call void @llvm.experimental.noalias.scope.decl(metadata !721), !dbg !724
  %85 = getelementptr inbounds nuw i8, ptr %self, i64 1600, !dbg !728
  store i32 0, ptr %85, align 32, !dbg !728, !alias.scope !731, !noalias !487
  %86 = getelementptr inbounds nuw i8, ptr %self, i64 1536, !dbg !732
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %86, i8 0, i64 32, i1 false), !dbg !732, !alias.scope !731, !noalias !487
  %87 = getelementptr inbounds nuw i8, ptr %self, i64 1576, !dbg !733
  %_9.1.i.i.i = load i64, ptr %87, align 8, !dbg !733, !alias.scope !731, !noalias !487, !noundef !11
  %_222.i.i.i.i = icmp eq i64 %_9.1.i.i.i, 0, !dbg !734
  br i1 %_222.i.i.i.i, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCse3bfmKSZS8Y_10compressor.exit.i.i.i, label %bb14.preheader.i.i.i.i, !dbg !746

bb14.preheader.i.i.i.i:                           ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i
  %88 = getelementptr inbounds nuw i8, ptr %self, i64 1568, !dbg !733
  %_9.0.i.i.i = load ptr, ptr %88, align 32, !dbg !733, !alias.scope !731, !noalias !487, !nonnull !11, !noundef !11
  %.idx.i.i.i.i = shl nuw nsw i64 %_9.1.i.i.i, 2, !dbg !747
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_9.0.i.i.i, i8 0, i64 %.idx.i.i.i.i, i1 false), !dbg !751, !alias.scope !752, !noalias !755
  br label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCse3bfmKSZS8Y_10compressor.exit.i.i.i, !dbg !756

_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCse3bfmKSZS8Y_10compressor.exit.i.i.i: ; preds = %bb14.preheader.i.i.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i
  %89 = getelementptr inbounds nuw i8, ptr %self, i64 1592, !dbg !757
  %_10.1.i.i.i = load i64, ptr %89, align 8, !dbg !757, !alias.scope !731, !noalias !487, !noundef !11
  %_222.i1.i.i.i = icmp eq i64 %_10.1.i.i.i, 0, !dbg !758
  br i1 %_222.i1.i.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank14finish_channelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCINvNtCse3bfmKSZS8Y_10compressor6kernel14finish_channelBT_E0EB1A_.exit, label %bb14.preheader.i2.i.i.i, !dbg !763

bb14.preheader.i2.i.i.i:                          ; preds = %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCse3bfmKSZS8Y_10compressor.exit.i.i.i
  %90 = getelementptr inbounds nuw i8, ptr %self, i64 1584, !dbg !757
  %_10.0.i.i.i = load ptr, ptr %90, align 16, !dbg !757, !alias.scope !731, !noalias !487, !nonnull !11, !noundef !11
  %.idx.i3.i.i.i = shl nuw nsw i64 %_10.1.i.i.i, 2, !dbg !764
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_10.0.i.i.i, i8 0, i64 %.idx.i3.i.i.i, i1 false), !dbg !768, !alias.scope !769, !noalias !755
  br label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank14finish_channelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCINvNtCse3bfmKSZS8Y_10compressor6kernel14finish_channelBT_E0EB1A_.exit, !dbg !772

_RINvNtCseSJggkR25Fr_14effect_runtime4bank14finish_channelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCINvNtCse3bfmKSZS8Y_10compressor6kernel14finish_channelBT_E0EB1A_.exit: ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCse3bfmKSZS8Y_10compressor.exit.i.i.i, %bb14.preheader.i2.i.i.i
  %mask.sroa.0.0.i = phi i32 [ 0, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i ], [ %mask.sroa.0.1.7.i.i, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCse3bfmKSZS8Y_10compressor.exit.i.i.i ], [ %mask.sroa.0.1.7.i.i, %bb14.preheader.i2.i.i.i ], !dbg !773
  tail call void @llvm.experimental.noalias.scope.decl(metadata !774), !dbg !777
  tail call void @llvm.experimental.noalias.scope.decl(metadata !780), !dbg !777
  %fst_len.i.i71 = and i64 %right.1, 34359738360, !dbg !782
  br label %bb13.i.i72

bb13.i.i72:                                       ; preds = %bb13.i.i72, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank14finish_channelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCINvNtCse3bfmKSZS8Y_10compressor6kernel14finish_channelBT_E0EB1A_.exit
  %iter.sroa.0.0.i64.i73 = phi ptr [ %right.0, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank14finish_channelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCINvNtCse3bfmKSZS8Y_10compressor6kernel14finish_channelBT_E0EB1A_.exit ], [ %_27.i.i77, %bb13.i.i72 ]
  %iter.sroa.5.0.i63.i74 = phi i64 [ %fst_len.i.i71, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank14finish_channelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCINvNtCse3bfmKSZS8Y_10compressor6kernel14finish_channelBT_E0EB1A_.exit ], [ %_28.i.i78, %bb13.i.i72 ]
  %ok.i.sroa.0.062.i75 = phi <8 x i32> [ %47, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank14finish_channelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCINvNtCse3bfmKSZS8Y_10compressor6kernel14finish_channelBT_E0EB1A_.exit ], [ %95, %bb13.i.i72 ]
  %lanes.i.sroa.0.0.copyload.i76 = load <8 x i32>, ptr %iter.sroa.0.0.i64.i73, align 4, !dbg !787, !alias.scope !792, !noalias !796
  %_27.i.i77 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i64.i73, i64 32, !dbg !800
  %_28.i.i78 = add nsw i64 %iter.sroa.5.0.i63.i74, -8, !dbg !805
  %91 = and <8 x i32> %lanes.i.sroa.0.0.copyload.i76, splat (i32 2147483647), !dbg !806
  %92 = bitcast <8 x i32> %91 to <8 x float>, !dbg !812
  %93 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %92, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !813
  %94 = bitcast <8 x float> %93 to <8 x i32>, !dbg !819
  %95 = and <8 x i32> %ok.i.sroa.0.062.i75, %94, !dbg !823
  %_22.not.i.i79 = icmp eq i64 %_28.i.i78, 0, !dbg !825
  br i1 %_22.not.i.i79, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i80, label %bb13.i.i72, !dbg !825

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i80: ; preds = %bb13.i.i72
  %96 = icmp sgt <8 x i32> %95, splat (i32 -1), !dbg !826
  %97 = bitcast <8 x i1> %96 to i8, !dbg !826
  %_0.i26.not.i81 = icmp eq i8 %97, 0, !dbg !831
  br i1 %_0.i26.not.i81, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank14finish_channelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCINvNtCse3bfmKSZS8Y_10compressor6kernel14finish_channelBT_E0EB1A_.exit112, label %bb23.i.i82, !dbg !832

bb23.i.i82:                                       ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i80, %bb23.i.i82
  %iter.sroa.0.074.i.i83 = phi ptr [ %_45.i.i87, %bb23.i.i82 ], [ %right.0, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i80 ]
  %iter.sroa.5.073.i.i84 = phi i64 [ %_46.i.i88, %bb23.i.i82 ], [ %fst_len.i.i71, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i80 ]
  %ok.sroa.0.072.i.i85 = phi <8 x i32> [ %102, %bb23.i.i82 ], [ %47, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i80 ]
  %lanes.i.sroa.0.0.copyload.i.i86 = load <8 x i32>, ptr %iter.sroa.0.074.i.i83, align 4, !dbg !833, !alias.scope !839, !noalias !845
  %_45.i.i87 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.074.i.i83, i64 32, !dbg !849
  %_46.i.i88 = add nsw i64 %iter.sroa.5.073.i.i84, -8, !dbg !854
  %98 = and <8 x i32> %lanes.i.sroa.0.0.copyload.i.i86, splat (i32 2147483647), !dbg !855
  %99 = bitcast <8 x i32> %98 to <8 x float>, !dbg !861
  %100 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %99, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !862
  %101 = bitcast <8 x float> %100 to <8 x i32>, !dbg !868
  %102 = and <8 x i32> %ok.sroa.0.072.i.i85, %101, !dbg !872
  %_40.not.i.i89 = icmp eq i64 %_46.i.i88, 0, !dbg !874
  br i1 %_40.not.i.i89, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i90, label %bb23.i.i82, !dbg !874

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i90: ; preds = %bb23.i.i82
  %103 = icmp slt <8 x i32> %102, zeroinitializer, !dbg !875
  %bc.i.i91 = select <8 x i1> %103, <8 x i32> zeroinitializer, <8 x i32> splat (i32 1065353216), !dbg !875
  %104 = extractelement <8 x i32> %bc.i.i91, i64 0, !dbg !880
  %105 = icmp ne i32 %104, 0, !dbg !880
  %106 = zext i1 %105 to i32, !dbg !880
  %107 = extractelement <8 x i32> %bc.i.i91, i64 1, !dbg !880
  %108 = icmp eq i32 %107, 0, !dbg !880
  %109 = select i1 %108, i32 0, i32 2, !dbg !880
  %mask.sroa.0.1.1.i.i92 = or disjoint i32 %109, %106, !dbg !880
  %110 = extractelement <8 x i32> %bc.i.i91, i64 2, !dbg !880
  %111 = icmp eq i32 %110, 0, !dbg !880
  %112 = select i1 %111, i32 0, i32 4, !dbg !880
  %mask.sroa.0.1.2.i.i93 = or disjoint i32 %mask.sroa.0.1.1.i.i92, %112, !dbg !880
  %113 = extractelement <8 x i32> %bc.i.i91, i64 3, !dbg !880
  %114 = icmp eq i32 %113, 0, !dbg !880
  %115 = select i1 %114, i32 0, i32 8, !dbg !880
  %mask.sroa.0.1.3.i.i94 = or disjoint i32 %mask.sroa.0.1.2.i.i93, %115, !dbg !880
  %116 = extractelement <8 x i32> %bc.i.i91, i64 4, !dbg !880
  %117 = icmp eq i32 %116, 0, !dbg !880
  %118 = select i1 %117, i32 0, i32 16, !dbg !880
  %mask.sroa.0.1.4.i.i95 = or disjoint i32 %mask.sroa.0.1.3.i.i94, %118, !dbg !880
  %119 = extractelement <8 x i32> %bc.i.i91, i64 5, !dbg !880
  %120 = icmp eq i32 %119, 0, !dbg !880
  %121 = select i1 %120, i32 0, i32 32, !dbg !880
  %mask.sroa.0.1.5.i.i96 = or disjoint i32 %mask.sroa.0.1.4.i.i95, %121, !dbg !880
  %122 = extractelement <8 x i32> %bc.i.i91, i64 6, !dbg !880
  %123 = icmp eq i32 %122, 0, !dbg !880
  %124 = select i1 %123, i32 0, i32 64, !dbg !880
  %mask.sroa.0.1.6.i.i97 = or i32 %mask.sroa.0.1.5.i.i96, %124, !dbg !880
  %125 = extractelement <8 x i32> %bc.i.i91, i64 7, !dbg !880
  %126 = icmp eq i32 %125, 0, !dbg !880
  %127 = select i1 %126, i32 0, i32 128, !dbg !880
  %mask.sroa.0.1.7.i.i98 = or i32 %mask.sroa.0.1.6.i.i97, %127, !dbg !880
  %.idx.i.i99 = shl nuw nsw i64 %right.1, 2, !dbg !881
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1) %right.0, i8 0, i64 %.idx.i.i99, i1 false), !dbg !887, !alias.scope !888, !noalias !780
  tail call void @llvm.experimental.noalias.scope.decl(metadata !891), !dbg !894
  tail call void @llvm.experimental.noalias.scope.decl(metadata !895), !dbg !898
  %128 = getelementptr inbounds nuw i8, ptr %self, i64 3168, !dbg !900
  store i32 0, ptr %128, align 32, !dbg !900, !alias.scope !902, !noalias !774
  %129 = getelementptr inbounds nuw i8, ptr %self, i64 3104, !dbg !903
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %129, i8 0, i64 32, i1 false), !dbg !903, !alias.scope !902, !noalias !774
  %130 = getelementptr inbounds nuw i8, ptr %self, i64 3144, !dbg !904
  %_9.1.i.i.i100 = load i64, ptr %130, align 8, !dbg !904, !alias.scope !902, !noalias !774, !noundef !11
  %_222.i.i.i.i101 = icmp eq i64 %_9.1.i.i.i100, 0, !dbg !905
  br i1 %_222.i.i.i.i101, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCse3bfmKSZS8Y_10compressor.exit.i.i.i105, label %bb14.preheader.i.i.i.i102, !dbg !910

bb14.preheader.i.i.i.i102:                        ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i90
  %131 = getelementptr inbounds nuw i8, ptr %self, i64 3136, !dbg !904
  %_9.0.i.i.i103 = load ptr, ptr %131, align 32, !dbg !904, !alias.scope !902, !noalias !774, !nonnull !11, !noundef !11
  %.idx.i.i.i.i104 = shl nuw nsw i64 %_9.1.i.i.i100, 2, !dbg !911
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_9.0.i.i.i103, i8 0, i64 %.idx.i.i.i.i104, i1 false), !dbg !915, !alias.scope !916, !noalias !919
  br label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCse3bfmKSZS8Y_10compressor.exit.i.i.i105, !dbg !920

_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCse3bfmKSZS8Y_10compressor.exit.i.i.i105: ; preds = %bb14.preheader.i.i.i.i102, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i90
  %132 = getelementptr inbounds nuw i8, ptr %self, i64 3160, !dbg !921
  %_10.1.i.i.i106 = load i64, ptr %132, align 8, !dbg !921, !alias.scope !902, !noalias !774, !noundef !11
  %_222.i1.i.i.i107 = icmp eq i64 %_10.1.i.i.i106, 0, !dbg !922
  br i1 %_222.i1.i.i.i107, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank14finish_channelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCINvNtCse3bfmKSZS8Y_10compressor6kernel14finish_channelBT_E0EB1A_.exit112, label %bb14.preheader.i2.i.i.i108, !dbg !927

bb14.preheader.i2.i.i.i108:                       ; preds = %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCse3bfmKSZS8Y_10compressor.exit.i.i.i105
  %133 = getelementptr inbounds nuw i8, ptr %self, i64 3152, !dbg !921
  %_10.0.i.i.i109 = load ptr, ptr %133, align 16, !dbg !921, !alias.scope !902, !noalias !774, !nonnull !11, !noundef !11
  %.idx.i3.i.i.i110 = shl nuw nsw i64 %_10.1.i.i.i106, 2, !dbg !928
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_10.0.i.i.i109, i8 0, i64 %.idx.i3.i.i.i110, i1 false), !dbg !932, !alias.scope !933, !noalias !919
  br label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank14finish_channelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCINvNtCse3bfmKSZS8Y_10compressor6kernel14finish_channelBT_E0EB1A_.exit112, !dbg !936

_RINvNtCseSJggkR25Fr_14effect_runtime4bank14finish_channelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCINvNtCse3bfmKSZS8Y_10compressor6kernel14finish_channelBT_E0EB1A_.exit112: ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i80, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCse3bfmKSZS8Y_10compressor.exit.i.i.i105, %bb14.preheader.i2.i.i.i108
  %mask.sroa.0.0.i111 = phi i32 [ 0, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i80 ], [ %mask.sroa.0.1.7.i.i98, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCse3bfmKSZS8Y_10compressor.exit.i.i.i105 ], [ %mask.sroa.0.1.7.i.i98, %bb14.preheader.i2.i.i.i108 ], !dbg !937
  %_59 = or i32 %mask.sroa.0.0.i111, %mask.sroa.0.0.i, !dbg !938
  %134 = icmp eq i32 %_59, 0, !dbg !938
  br i1 %134, label %bb39, label %bb75.preheader, !dbg !938

bb75.preheader:                                   ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank14finish_channelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCINvNtCse3bfmKSZS8Y_10compressor6kernel14finish_channelBT_E0EB1A_.exit112
  %_67 = and i32 %mask.sroa.0.0.i, 1, !dbg !940
  %_66.not = icmp eq i32 %_67, 0, !dbg !940
  %_69 = and i32 %mask.sroa.0.0.i111, 1, !dbg !944
  %_68.not = icmp eq i32 %_69, 0, !dbg !944
  br i1 %_66.not, label %bb4.i, label %bb2.i, !dbg !945

bb53:                                             ; preds = %bb17
  %135 = getelementptr inbounds nuw i8, ptr %self, i64 1600, !dbg !950
  %_87 = load i32, ptr %135, align 32, !dbg !950, !noundef !11
  %_88 = urem i32 %_22, %_85, !dbg !951
  %_86 = add i32 %_88, %_87, !dbg !952
  %136 = urem i32 %_86, %_85, !dbg !953
  store i32 %136, ptr %135, align 32, !dbg !953
  br label %bb51, !dbg !954

bb51:                                             ; preds = %bb17, %bb53
  %137 = getelementptr inbounds nuw i8, ptr %self, i64 3172, !dbg !955
  %_93 = load i32, ptr %137, align 4, !dbg !955, !noundef !11
  %138 = icmp eq i32 %_93, 0, !dbg !955
  br i1 %138, label %bb44, label %bb58, !dbg !955

bb58:                                             ; preds = %bb51
  %139 = getelementptr inbounds nuw i8, ptr %self, i64 3168, !dbg !957
  %_95 = load i32, ptr %139, align 32, !dbg !957, !noundef !11
  %_96 = urem i32 %_22, %_93, !dbg !958
  %_94 = add i32 %_96, %_95, !dbg !959
  %140 = urem i32 %_94, %_93, !dbg !960
  store i32 %140, ptr %139, align 32, !dbg !960
  br label %bb44, !dbg !961

bb39:                                             ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank14finish_channelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCINvNtCse3bfmKSZS8Y_10compressor6kernel14finish_channelBT_E0EB1A_.exit112
  call void @llvm.lifetime.end.p0(ptr nonnull %before.sroa.6), !dbg !962
  br label %bb44, !dbg !963

bb44:                                             ; preds = %bb39, %bb51, %bb58, %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.7
  ret void, !dbg !964

bb4.i:                                            ; preds = %bb2.i, %bb75.preheader
  br i1 %_68.not, label %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit, label %bb6.i, !dbg !965

bb2.i:                                            ; preds = %bb75.preheader
  %141 = getelementptr inbounds nuw i8, ptr %0, i64 24, !dbg !966
  %_6.i = load i64, ptr %141, align 8, !dbg !966, !noundef !11
  %142 = tail call i64 @llvm.uadd.sat.i64(i64 %_6.i, i64 1), !dbg !967
  store i64 %142, ptr %141, align 8, !dbg !970
  br label %bb4.i, !dbg !971

bb6.i:                                            ; preds = %bb4.i
  %143 = getelementptr inbounds nuw i8, ptr %0, i64 32, !dbg !972
  %_9.i = load i64, ptr %143, align 8, !dbg !972, !noundef !11
  %144 = tail call i64 @llvm.uadd.sat.i64(i64 %_9.i, i64 1), !dbg !973
  store i64 %144, ptr %143, align 8, !dbg !975
  br label %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit, !dbg !976

_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit: ; preds = %bb4.i, %bb6.i
  %_67.1 = and i32 %mask.sroa.0.0.i, 2, !dbg !940
  %_66.not.1 = icmp eq i32 %_67.1, 0, !dbg !940
  %_69.1 = and i32 %mask.sroa.0.0.i111, 2, !dbg !944
  %_68.not.1 = icmp eq i32 %_69.1, 0, !dbg !944
  br i1 %_66.not.1, label %bb4.i.1, label %bb2.i.1, !dbg !945

bb2.i.1:                                          ; preds = %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit
  %145 = getelementptr inbounds nuw i8, ptr %0, i64 64, !dbg !966
  %_6.i.1 = load i64, ptr %145, align 8, !dbg !966, !noundef !11
  %146 = tail call i64 @llvm.uadd.sat.i64(i64 %_6.i.1, i64 1), !dbg !967
  store i64 %146, ptr %145, align 8, !dbg !970
  br label %bb4.i.1, !dbg !971

bb4.i.1:                                          ; preds = %bb2.i.1, %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit
  br i1 %_68.not.1, label %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.1, label %bb6.i.1, !dbg !965

bb6.i.1:                                          ; preds = %bb4.i.1
  %147 = getelementptr inbounds nuw i8, ptr %0, i64 72, !dbg !972
  %_9.i.1 = load i64, ptr %147, align 8, !dbg !972, !noundef !11
  %148 = tail call i64 @llvm.uadd.sat.i64(i64 %_9.i.1, i64 1), !dbg !973
  store i64 %148, ptr %147, align 8, !dbg !975
  br label %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.1, !dbg !976

_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.1: ; preds = %bb6.i.1, %bb4.i.1
  %_67.2 = and i32 %mask.sroa.0.0.i, 4, !dbg !940
  %_66.not.2 = icmp eq i32 %_67.2, 0, !dbg !940
  %_69.2 = and i32 %mask.sroa.0.0.i111, 4, !dbg !944
  %_68.not.2 = icmp eq i32 %_69.2, 0, !dbg !944
  br i1 %_66.not.2, label %bb4.i.2, label %bb2.i.2, !dbg !945

bb2.i.2:                                          ; preds = %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.1
  %149 = getelementptr inbounds nuw i8, ptr %0, i64 104, !dbg !966
  %_6.i.2 = load i64, ptr %149, align 8, !dbg !966, !noundef !11
  %150 = tail call i64 @llvm.uadd.sat.i64(i64 %_6.i.2, i64 1), !dbg !967
  store i64 %150, ptr %149, align 8, !dbg !970
  br label %bb4.i.2, !dbg !971

bb4.i.2:                                          ; preds = %bb2.i.2, %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.1
  br i1 %_68.not.2, label %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.2, label %bb6.i.2, !dbg !965

bb6.i.2:                                          ; preds = %bb4.i.2
  %151 = getelementptr inbounds nuw i8, ptr %0, i64 112, !dbg !972
  %_9.i.2 = load i64, ptr %151, align 8, !dbg !972, !noundef !11
  %152 = tail call i64 @llvm.uadd.sat.i64(i64 %_9.i.2, i64 1), !dbg !973
  store i64 %152, ptr %151, align 8, !dbg !975
  br label %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.2, !dbg !976

_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.2: ; preds = %bb6.i.2, %bb4.i.2
  %_67.3 = and i32 %mask.sroa.0.0.i, 8, !dbg !940
  %_66.not.3 = icmp eq i32 %_67.3, 0, !dbg !940
  %_69.3 = and i32 %mask.sroa.0.0.i111, 8, !dbg !944
  %_68.not.3 = icmp eq i32 %_69.3, 0, !dbg !944
  br i1 %_66.not.3, label %bb4.i.3, label %bb2.i.3, !dbg !945

bb2.i.3:                                          ; preds = %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.2
  %153 = getelementptr inbounds nuw i8, ptr %0, i64 144, !dbg !966
  %_6.i.3 = load i64, ptr %153, align 8, !dbg !966, !noundef !11
  %154 = tail call i64 @llvm.uadd.sat.i64(i64 %_6.i.3, i64 1), !dbg !967
  store i64 %154, ptr %153, align 8, !dbg !970
  br label %bb4.i.3, !dbg !971

bb4.i.3:                                          ; preds = %bb2.i.3, %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.2
  br i1 %_68.not.3, label %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.3, label %bb6.i.3, !dbg !965

bb6.i.3:                                          ; preds = %bb4.i.3
  %155 = getelementptr inbounds nuw i8, ptr %0, i64 152, !dbg !972
  %_9.i.3 = load i64, ptr %155, align 8, !dbg !972, !noundef !11
  %156 = tail call i64 @llvm.uadd.sat.i64(i64 %_9.i.3, i64 1), !dbg !973
  store i64 %156, ptr %155, align 8, !dbg !975
  br label %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.3, !dbg !976

_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.3: ; preds = %bb6.i.3, %bb4.i.3
  %_67.4 = and i32 %mask.sroa.0.0.i, 16, !dbg !940
  %_66.not.4 = icmp eq i32 %_67.4, 0, !dbg !940
  %_69.4 = and i32 %mask.sroa.0.0.i111, 16, !dbg !944
  %_68.not.4 = icmp eq i32 %_69.4, 0, !dbg !944
  br i1 %_66.not.4, label %bb4.i.4, label %bb2.i.4, !dbg !945

bb2.i.4:                                          ; preds = %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.3
  %157 = getelementptr inbounds nuw i8, ptr %0, i64 184, !dbg !966
  %_6.i.4 = load i64, ptr %157, align 8, !dbg !966, !noundef !11
  %158 = tail call i64 @llvm.uadd.sat.i64(i64 %_6.i.4, i64 1), !dbg !967
  store i64 %158, ptr %157, align 8, !dbg !970
  br label %bb4.i.4, !dbg !971

bb4.i.4:                                          ; preds = %bb2.i.4, %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.3
  br i1 %_68.not.4, label %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.4, label %bb6.i.4, !dbg !965

bb6.i.4:                                          ; preds = %bb4.i.4
  %159 = getelementptr inbounds nuw i8, ptr %0, i64 192, !dbg !972
  %_9.i.4 = load i64, ptr %159, align 8, !dbg !972, !noundef !11
  %160 = tail call i64 @llvm.uadd.sat.i64(i64 %_9.i.4, i64 1), !dbg !973
  store i64 %160, ptr %159, align 8, !dbg !975
  br label %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.4, !dbg !976

_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.4: ; preds = %bb6.i.4, %bb4.i.4
  %_67.5 = and i32 %mask.sroa.0.0.i, 32, !dbg !940
  %_66.not.5 = icmp eq i32 %_67.5, 0, !dbg !940
  %_69.5 = and i32 %mask.sroa.0.0.i111, 32, !dbg !944
  %_68.not.5 = icmp eq i32 %_69.5, 0, !dbg !944
  br i1 %_66.not.5, label %bb4.i.5, label %bb2.i.5, !dbg !945

bb2.i.5:                                          ; preds = %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.4
  %161 = getelementptr inbounds nuw i8, ptr %0, i64 224, !dbg !966
  %_6.i.5 = load i64, ptr %161, align 8, !dbg !966, !noundef !11
  %162 = tail call i64 @llvm.uadd.sat.i64(i64 %_6.i.5, i64 1), !dbg !967
  store i64 %162, ptr %161, align 8, !dbg !970
  br label %bb4.i.5, !dbg !971

bb4.i.5:                                          ; preds = %bb2.i.5, %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.4
  br i1 %_68.not.5, label %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.5, label %bb6.i.5, !dbg !965

bb6.i.5:                                          ; preds = %bb4.i.5
  %163 = getelementptr inbounds nuw i8, ptr %0, i64 232, !dbg !972
  %_9.i.5 = load i64, ptr %163, align 8, !dbg !972, !noundef !11
  %164 = tail call i64 @llvm.uadd.sat.i64(i64 %_9.i.5, i64 1), !dbg !973
  store i64 %164, ptr %163, align 8, !dbg !975
  br label %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.5, !dbg !976

_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.5: ; preds = %bb6.i.5, %bb4.i.5
  %_67.6 = and i32 %mask.sroa.0.0.i, 64, !dbg !940
  %_66.not.6 = icmp eq i32 %_67.6, 0, !dbg !940
  %_69.6 = and i32 %mask.sroa.0.0.i111, 64, !dbg !944
  %_68.not.6 = icmp eq i32 %_69.6, 0, !dbg !944
  br i1 %_66.not.6, label %bb4.i.6, label %bb2.i.6, !dbg !945

bb2.i.6:                                          ; preds = %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.5
  %165 = getelementptr inbounds nuw i8, ptr %0, i64 264, !dbg !966
  %_6.i.6 = load i64, ptr %165, align 8, !dbg !966, !noundef !11
  %166 = tail call i64 @llvm.uadd.sat.i64(i64 %_6.i.6, i64 1), !dbg !967
  store i64 %166, ptr %165, align 8, !dbg !970
  br label %bb4.i.6, !dbg !971

bb4.i.6:                                          ; preds = %bb2.i.6, %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.5
  br i1 %_68.not.6, label %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.6, label %bb6.i.6, !dbg !965

bb6.i.6:                                          ; preds = %bb4.i.6
  %167 = getelementptr inbounds nuw i8, ptr %0, i64 272, !dbg !972
  %_9.i.6 = load i64, ptr %167, align 8, !dbg !972, !noundef !11
  %168 = tail call i64 @llvm.uadd.sat.i64(i64 %_9.i.6, i64 1), !dbg !973
  store i64 %168, ptr %167, align 8, !dbg !975
  br label %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.6, !dbg !976

_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.6: ; preds = %bb6.i.6, %bb4.i.6
  %_67.7 = and i32 %mask.sroa.0.0.i, 128, !dbg !940
  %_66.not.7 = icmp eq i32 %_67.7, 0, !dbg !940
  %_69.7 = and i32 %mask.sroa.0.0.i111, 128, !dbg !944
  %_68.not.7 = icmp eq i32 %_69.7, 0, !dbg !944
  br i1 %_66.not.7, label %bb4.i.7, label %bb2.i.7, !dbg !945

bb2.i.7:                                          ; preds = %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.6
  %169 = getelementptr inbounds nuw i8, ptr %0, i64 304, !dbg !966
  %_6.i.7 = load i64, ptr %169, align 8, !dbg !966, !noundef !11
  %170 = tail call i64 @llvm.uadd.sat.i64(i64 %_6.i.7, i64 1), !dbg !967
  store i64 %170, ptr %169, align 8, !dbg !970
  br label %bb4.i.7, !dbg !971

bb4.i.7:                                          ; preds = %bb2.i.7, %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.6
  br i1 %_68.not.7, label %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.7, label %bb6.i.7, !dbg !965

bb6.i.7:                                          ; preds = %bb4.i.7
  %171 = getelementptr inbounds nuw i8, ptr %0, i64 312, !dbg !972
  %_9.i.7 = load i64, ptr %171, align 8, !dbg !972, !noundef !11
  %172 = tail call i64 @llvm.uadd.sat.i64(i64 %_9.i.7, i64 1), !dbg !973
  store i64 %172, ptr %171, align 8, !dbg !975
  br label %_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.7, !dbg !976

_RNCINvMs3_Cse3bfmKSZS8Y_10compressorINtB8_22PreparedCompressorBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_E0B8_.exit.7: ; preds = %bb6.i.7, %bb4.i.7
  call void @llvm.lifetime.end.p0(ptr nonnull %before.sroa.6), !dbg !962
  br label %bb44, !dbg !977
}

define internal fastcc void @_RINvNtCse3bfmKSZS8Y_10compressor6kernel13process_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_(ptr noalias noundef nonnull align 4 captures(none) %left.0, i64 noundef range(i64 8, 34359738361) %left.1, ptr noalias noundef nonnull align 4 captures(none) %right.0, i64 noundef range(i64 8, 34359738361) %right.1, ptr dead_on_return noalias noundef nonnull readonly align 8 captures(none) dereferenceable(40) %detector, i64 noundef range(i64 1, 4294967296) %frames, i32 noundef range(i32 1, 4) %link, i1 noundef zeroext %bypass, i32 noundef %sample_rate, ptr noalias noundef nonnull align 32 dereferenceable(1568) %channels.0, ptr noalias noundef nonnull align 32 dereferenceable(1568) %channels.1, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(64) %staged) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !1674 {
start:
; call <compressor::kernel::Channel<wide::f32x8_::f32x8>>::max_remaining
  %_20 = tail call fastcc noundef i32 @_RNvMNtCse3bfmKSZS8Y_10compressor6kernelINtB2_7ChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13max_remainingB4_(ptr noalias noundef readonly align 32 captures(address, read_provenance) dereferenceable(1568) %channels.0) #22, !dbg !1675
; call <compressor::kernel::Channel<wide::f32x8_::f32x8>>::max_remaining
  %_22 = tail call fastcc noundef i32 @_RNvMNtCse3bfmKSZS8Y_10compressor6kernelINtB2_7ChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13max_remainingB4_(ptr noalias noundef readonly align 32 captures(address, read_provenance) dereferenceable(1568) %channels.1) #22, !dbg !1677
  %..i = tail call noundef i32 @llvm.umax.i32(i32 %_22, i32 %_20), !dbg !1678
  %0 = zext i32 %..i to i64, !dbg !1681
  %_24 = icmp samesign ugt i64 %frames, %0, !dbg !1682
  %spec.store.select = tail call i64 @llvm.umin.i64(i64 %frames, i64 %0), !dbg !1682
  %_25.not = icmp eq i32 %..i, 0, !dbg !1684
  br i1 %_25.not, label %bb10, label %bb7, !dbg !1684

bb7:                                              ; preds = %start
  %.sroa.0.0.copyload = load i64, ptr %detector, align 8, !dbg !1686
  %.sroa.4.0.detector.sroa_idx = getelementptr inbounds nuw i8, ptr %detector, i64 8, !dbg !1686
  %.sroa.4.0.copyload = load ptr, ptr %.sroa.4.0.detector.sroa_idx, align 8, !dbg !1686
  %.sroa.5.0.detector.sroa_idx = getelementptr inbounds nuw i8, ptr %detector, i64 16, !dbg !1686
  %.sroa.5.0.copyload = load i64, ptr %.sroa.5.0.detector.sroa_idx, align 8, !dbg !1686
  %.sroa.6.0.detector.sroa_idx = getelementptr inbounds nuw i8, ptr %detector, i64 24, !dbg !1686
  %.sroa.6.0.copyload = load ptr, ptr %.sroa.6.0.detector.sroa_idx, align 8, !dbg !1686
  %.sroa.7.0.detector.sroa_idx = getelementptr inbounds nuw i8, ptr %detector, i64 32, !dbg !1686
  %.sroa.7.0.copyload = load i64, ptr %.sroa.7.0.detector.sroa_idx, align 8, !dbg !1686
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1687), !dbg !1686
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1690), !dbg !1686
  %1 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1540, !dbg !1692
  %_12.i40 = load i32, ptr %1, align 4, !dbg !1692, !alias.scope !1687, !noalias !1696, !noundef !11
  %ring_length.i41 = zext i32 %_12.i40 to i64, !dbg !1692
  %2 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !1700
  %3 = bitcast <8 x float> %2 to <8 x i32>, !dbg !1711
  %4 = xor <8 x i32> %3, splat (i32 -1), !dbg !1727
  switch i32 %link, label %bb13.i578 [
    i32 1, label %bb14.i579
    i32 3, label %bb14.i579.fold.split
  ], !dbg !1732

bb13.i578:                                        ; preds = %bb7
  br label %bb14.i579, !dbg !1736

bb14.i579.fold.split:                             ; preds = %bb7
  br label %bb14.i579, !dbg !1737

bb14.i579:                                        ; preds = %bb7, %bb14.i579.fold.split, %bb13.i578
  %.pre-phi = phi <8 x i32> [ %3, %bb13.i578 ], [ %3, %bb14.i579.fold.split ], [ %4, %bb7 ]
  %_13.i570.sroa.0.0 = phi <8 x i32> [ %4, %bb13.i578 ], [ %3, %bb14.i579.fold.split ], [ %4, %bb7 ], !dbg !1738
  %_15.i42 = getelementptr inbounds nuw i8, ptr %channels.0, i64 256, !dbg !1739
  %_4.i672 = getelementptr inbounds nuw i8, ptr %channels.0, i64 448, !dbg !1741
  %_7.i673 = getelementptr inbounds nuw i8, ptr %channels.0, i64 480, !dbg !1745
  %_14.i674 = getelementptr inbounds nuw i8, ptr %channels.0, i64 288, !dbg !1747
  %_17.i675 = getelementptr inbounds nuw i8, ptr %channels.0, i64 320, !dbg !1749
  %_20.i676 = getelementptr inbounds nuw i8, ptr %channels.0, i64 352, !dbg !1750
  %_23.i677 = getelementptr inbounds nuw i8, ptr %channels.0, i64 384, !dbg !1751
  %_26.i678 = getelementptr inbounds nuw i8, ptr %channels.0, i64 416, !dbg !1752
  %_17.i43 = getelementptr inbounds nuw i8, ptr %channels.1, i64 256, !dbg !1753
  %_4.i650 = getelementptr inbounds nuw i8, ptr %channels.1, i64 448, !dbg !1755
  %_7.i651 = getelementptr inbounds nuw i8, ptr %channels.1, i64 480, !dbg !1757
  %_14.i652 = getelementptr inbounds nuw i8, ptr %channels.1, i64 288, !dbg !1758
  %_17.i653 = getelementptr inbounds nuw i8, ptr %channels.1, i64 320, !dbg !1759
  %_20.i654 = getelementptr inbounds nuw i8, ptr %channels.1, i64 352, !dbg !1760
  %_23.i655 = getelementptr inbounds nuw i8, ptr %channels.1, i64 384, !dbg !1761
  %_26.i656 = getelementptr inbounds nuw i8, ptr %channels.1, i64 416, !dbg !1762
  %5 = icmp ne ptr %.sroa.6.0.copyload, null
  %6 = icmp ne ptr %.sroa.4.0.copyload, null
  %7 = icmp slt <8 x i32> %_13.i570.sroa.0.0, zeroinitializer
  %8 = icmp slt <8 x i32> %.pre-phi, zeroinitializer
  %9 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1536
  %10 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1504
  %11 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1512
  %12 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1528
  %13 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1520
  %14 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1504
  %15 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1512
  %16 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1528
  %17 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1520
  %18 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1440
  %19 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1540
  %20 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1440
  %_69.i115 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1472
  %21 = select i1 %bypass, <8 x i32> %3, <8 x i32> %4
  %_73.i116 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1472
  %22 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1536
  %23 = lshr i64 %left.1, 3, !dbg !1763
  %24 = lshr i64 %right.1, 3, !dbg !1763
  %25 = add nuw nsw i64 %left.1, 8, !dbg !1763
  %26 = lshr i64 %25, 3, !dbg !1763
  %27 = add nuw nsw i64 %right.1, 8, !dbg !1763
  %28 = lshr i64 %27, 3, !dbg !1763
  %29 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1444
  %30 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1448
  %31 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1452
  %32 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1456
  %33 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1460
  %34 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1464
  %35 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1468
  %36 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1444
  %37 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1448
  %38 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1452
  %39 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1456
  %40 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1460
  %41 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1464
  %42 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1468
  br label %bb27.i48, !dbg !1763

bb27.i48:                                         ; preds = %bb14.i579, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit501
  %start1.sroa.0.0.i464701 = phi i64 [ 0, %bb14.i579 ], [ %43, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit501 ]
; call <compressor::kernel::Channel<wide::f32x8_::f32x8>>::advance_ramps
  tail call fastcc void @_RNvMNtCse3bfmKSZS8Y_10compressor6kernelINtB2_7ChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13advance_rampsB4_(ptr noalias noundef nonnull align 32 dereferenceable(1568) %channels.0, i32 noundef %sample_rate) #22, !dbg !1772, !noalias !1774
; call <compressor::kernel::Channel<wide::f32x8_::f32x8>>::advance_ramps
  tail call fastcc void @_RNvMNtCse3bfmKSZS8Y_10compressor6kernelINtB2_7ChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13advance_rampsB4_(ptr noalias noundef nonnull align 32 dereferenceable(1568) %channels.1, i32 noundef %sample_rate) #22, !dbg !1775, !noalias !1774
  %lanes.i1645.sroa.0.0.copyload = load <8 x float>, ptr %_4.i672, align 32, !dbg !1776, !alias.scope !1782, !noalias !1786
  %lanes.i1639.sroa.0.0.copyload = load <8 x float>, ptr %_7.i673, align 32, !dbg !1792, !alias.scope !1797, !noalias !1801
  %lanes.i1633.sroa.0.0.copyload = load <8 x float>, ptr %_15.i42, align 32, !dbg !1805, !alias.scope !1810, !noalias !1814
  %lanes.i1627.sroa.0.0.copyload = load <8 x float>, ptr %_14.i674, align 32, !dbg !1818, !alias.scope !1823, !noalias !1827
  %lanes.i1621.sroa.0.0.copyload = load <8 x float>, ptr %_17.i675, align 32, !dbg !1831, !alias.scope !1836, !noalias !1840
  %lanes.i1615.sroa.0.0.copyload = load <8 x float>, ptr %_20.i676, align 32, !dbg !1844, !alias.scope !1849, !noalias !1853
  %lanes.i1609.sroa.0.0.copyload = load <8 x float>, ptr %_23.i677, align 32, !dbg !1857, !alias.scope !1862, !noalias !1866
  %lanes.i1603.sroa.0.0.copyload = load <8 x float>, ptr %_26.i678, align 32, !dbg !1870, !alias.scope !1875, !noalias !1879
  %lanes.i1693.sroa.0.0.copyload = load <8 x float>, ptr %_4.i650, align 32, !dbg !1883, !alias.scope !1889, !noalias !1893
  %lanes.i1687.sroa.0.0.copyload = load <8 x float>, ptr %_7.i651, align 32, !dbg !1899, !alias.scope !1904, !noalias !1908
  %lanes.i1681.sroa.0.0.copyload = load <8 x float>, ptr %_17.i43, align 32, !dbg !1912, !alias.scope !1917, !noalias !1921
  %lanes.i1675.sroa.0.0.copyload = load <8 x float>, ptr %_14.i652, align 32, !dbg !1925, !alias.scope !1930, !noalias !1934
  %lanes.i1669.sroa.0.0.copyload = load <8 x float>, ptr %_17.i653, align 32, !dbg !1938, !alias.scope !1943, !noalias !1947
  %lanes.i1663.sroa.0.0.copyload = load <8 x float>, ptr %_20.i654, align 32, !dbg !1951, !alias.scope !1956, !noalias !1960
  %lanes.i1657.sroa.0.0.copyload = load <8 x float>, ptr %_23.i655, align 32, !dbg !1964, !alias.scope !1969, !noalias !1973
  %lanes.i1651.sroa.0.0.copyload = load <8 x float>, ptr %_26.i656, align 32, !dbg !1977, !alias.scope !1982, !noalias !1986
  %43 = add nuw nsw i64 %start1.sroa.0.0.i464701, 1, !dbg !1990
  %44 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i1639.sroa.0.0.copyload, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !1996
  %45 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i1639.sroa.0.0.copyload, <8 x float> zeroinitializer, i8 0), !dbg !2002
  %46 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i1645.sroa.0.0.copyload, <8 x float> zeroinitializer, i8 0), !dbg !2008
  %47 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i1687.sroa.0.0.copyload, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !2014
  %48 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i1687.sroa.0.0.copyload, <8 x float> zeroinitializer, i8 0), !dbg !2020
  %49 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i1693.sroa.0.0.copyload, <8 x float> zeroinitializer, i8 0), !dbg !2026
  %slot.i49 = shl nuw nsw i64 %start1.sroa.0.0.i464701, 3, !dbg !2032
  %exitcond = icmp eq i64 %start1.sroa.0.0.i464701, %26, !dbg !2033
  br i1 %exitcond, label %bb29.i128, label %bb30.i51, !dbg !2033, !prof !161

bb30.i51:                                         ; preds = %bb27.i48
  %_96.i53 = getelementptr inbounds nuw float, ptr %left.0, i64 %slot.i49, !dbg !2040
  %exitcond5492.not = icmp eq i64 %start1.sroa.0.0.i464701, %23, !dbg !2045
  br i1 %exitcond5492.not, label %bb2.i1770, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1773, !dbg !2045, !prof !161

bb2.i1770:                                        ; preds = %bb30.i51
  %50 = and i64 %left.1, 7, !dbg !1763
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %50, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !2053, !noalias !2054
  unreachable, !dbg !2053

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1773: ; preds = %bb30.i51
  %lanes.i1766.sroa.0.0.copyload = load <8 x float>, ptr %_96.i53, align 4, !dbg !2058, !alias.scope !2062, !noalias !2066
  %exitcond5493 = icmp eq i64 %start1.sroa.0.0.i464701, %28, !dbg !2068
  br i1 %exitcond5493, label %bb31.i127, label %bb32.i55, !dbg !2068, !prof !161

bb29.i128:                                        ; preds = %bb27.i48
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %slot.i49, i64 noundef range(i64 8, 34359738361) %left.1, i64 noundef range(i64 8, 34359738361) %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c00de08b5cdb434b6310222ddd96a0fc) #23, !dbg !2073, !noalias !1774
  unreachable, !dbg !2073

bb32.i55:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1773
  %_104.i57 = getelementptr inbounds nuw float, ptr %right.0, i64 %slot.i49, !dbg !2074
  %exitcond5494.not = icmp eq i64 %start1.sroa.0.0.i464701, %24, !dbg !2079
  br i1 %exitcond5494.not, label %bb2.i1761, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1764, !dbg !2079, !prof !161

bb2.i1761:                                        ; preds = %bb32.i55
  %51 = and i64 %right.1, 7, !dbg !1763
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %51, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !2084, !noalias !2085
  unreachable, !dbg !2084

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1764: ; preds = %bb32.i55
  %lanes.i1757.sroa.0.0.copyload = load <8 x float>, ptr %_104.i57, align 4, !dbg !2089, !alias.scope !2093, !noalias !2097
  switch i64 %.sroa.0.0.copyload, label %default.unreachable.i.i126 [
    i64 0, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i72
    i64 1, label %bb3.i.i125
    i64 2, label %bb2.i.i59
  ], !dbg !2099

default.unreachable.i.i126:                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1764
  unreachable

bb3.i.i125:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1764
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i72, !dbg !2103

bb2.i.i59:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1764
  tail call void @llvm.assume(i1 %5)
  %_25.i.i63 = icmp ugt i64 %slot.i49, %.sroa.5.0.copyload, !dbg !2104
  br i1 %_25.i.i63, label %bb17.i.i124, label %bb18.i.i64, !dbg !2104, !prof !161

bb18.i.i64:                                       ; preds = %bb2.i.i59
  tail call void @llvm.assume(i1 %6)
  %_28.i.i66 = sub nuw i64 %.sroa.5.0.copyload, %slot.i49, !dbg !2110
  %_8.i1751 = icmp samesign ugt i64 %_28.i.i66, 7, !dbg !2111
  br i1 %_8.i1751, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1755, label %bb2.i1752, !dbg !2111, !prof !2116

bb2.i1752:                                        ; preds = %bb18.i.i64
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_28.i.i66, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !2117, !noalias !2118
  unreachable, !dbg !2117

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1755: ; preds = %bb18.i.i64
  %_32.i.i67 = getelementptr inbounds nuw float, ptr %.sroa.4.0.copyload, i64 %slot.i49, !dbg !2127
  %lanes.i1748.sroa.0.0.copyload = load <8 x float>, ptr %_32.i.i67, align 4, !dbg !2132, !alias.scope !2136, !noalias !2140
  %_33.i.i68 = icmp ugt i64 %slot.i49, %.sroa.7.0.copyload, !dbg !2142
  br i1 %_33.i.i68, label %bb19.i.i123, label %bb20.i.i69, !dbg !2142, !prof !161

bb17.i.i124:                                      ; preds = %bb2.i.i59
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 34359738361) %slot.i49, i64 noundef %.sroa.5.0.copyload, i64 noundef %.sroa.5.0.copyload, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a0a4023c361dd50869bbb08b76482f50) #23, !dbg !2146, !noalias !2147
  unreachable, !dbg !2146

bb20.i.i69:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1755
  %_36.i.i70 = sub nuw i64 %.sroa.7.0.copyload, %slot.i49, !dbg !2149
  %_8.i1742 = icmp samesign ugt i64 %_36.i.i70, 7, !dbg !2150
  br i1 %_8.i1742, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1746, label %bb2.i1743, !dbg !2150, !prof !2116

bb2.i1743:                                        ; preds = %bb20.i.i69
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_36.i.i70, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !2155, !noalias !2156
  unreachable, !dbg !2155

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1746: ; preds = %bb20.i.i69
  %_40.i.i71 = getelementptr inbounds nuw float, ptr %.sroa.6.0.copyload, i64 %slot.i49, !dbg !2160
  %lanes.i1739.sroa.0.0.copyload = load <8 x float>, ptr %_40.i.i71, align 4, !dbg !2165, !alias.scope !2169, !noalias !2173
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i72, !dbg !2175

bb19.i.i123:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1755
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 34359738361) %slot.i49, i64 noundef %.sroa.7.0.copyload, i64 noundef %.sroa.7.0.copyload, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5807a2a07e9839e3d32631b4243eb17f) #23, !dbg !2176, !noalias !2147
  unreachable, !dbg !2176

_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i72: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1746, %bb3.i.i125, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1764
  %.sroa.02702.0 = phi <8 x float> [ %lanes.i1757.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1764 ], [ zeroinitializer, %bb3.i.i125 ], [ %lanes.i1739.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1746 ], !dbg !2177
  %.sroa.02699.0 = phi <8 x float> [ %lanes.i1766.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1764 ], [ zeroinitializer, %bb3.i.i125 ], [ %lanes.i1748.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1746 ], !dbg !2177
  %52 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %.sroa.02699.0), !dbg !2178
  %53 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %.sroa.02702.0), !dbg !2185
  %54 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %52, <8 x float> %53), !dbg !2192
  %55 = fmul <8 x float> %52, splat (float 5.000000e-01), !dbg !2202
  %56 = fmul <8 x float> %53, splat (float 5.000000e-01), !dbg !2213
  %57 = fadd <8 x float> %55, %56, !dbg !2218
  %58 = select <8 x i1> %7, <8 x float> %57, <8 x float> %54, !dbg !2228
  %59 = select <8 x i1> %8, <8 x float> %58, <8 x float> %52, !dbg !2234
  %60 = select <8 x i1> %8, <8 x float> %58, <8 x float> %53, !dbg !2240
  %_37.i73 = load i32, ptr %9, align 32, !dbg !2245, !alias.scope !1687, !noalias !1696, !noundef !11
  %write.i74 = zext i32 %_37.i73 to i64, !dbg !2245
  %61 = add nuw nsw i64 %write.i74, 1, !dbg !2247
  %_39.i75 = icmp eq i64 %61, %ring_length.i41, !dbg !2249
  %spec.store.select.i76 = select i1 %_39.i75, i64 0, i64 %61, !dbg !2249
  %_165.1.i77 = load i64, ptr %11, align 8, !dbg !2251, !alias.scope !1687, !noalias !1696, !noundef !11
  %_43.i78 = shl nuw nsw i64 %write.i74, 3, !dbg !2253
  %_105.i79 = icmp ugt i64 %_43.i78, %_165.1.i77, !dbg !2254
  br i1 %_105.i79, label %bb33.i122, label %bb34.i80, !dbg !2254, !prof !161

bb31.i127:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1773
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %slot.i49, i64 noundef range(i64 8, 34359738361) %right.1, i64 noundef range(i64 8, 34359738361) %right.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_99a388bbc796e7fd7b28fa01c6ed4e6b) #23, !dbg !2260, !noalias !1774
  unreachable, !dbg !2260

bb34.i80:                                         ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i72
  %_108.i82 = sub nuw i64 %_165.1.i77, %_43.i78, !dbg !2261
  %_8.i2367 = icmp samesign ugt i64 %_108.i82, 7, !dbg !2262
  br i1 %_8.i2367, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2370, label %bb2.i2368, !dbg !2262, !prof !2116

bb2.i2368:                                        ; preds = %bb34.i80
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_108.i82, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !2271, !noalias !2272
  unreachable, !dbg !2271

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2370: ; preds = %bb34.i80
  %_165.0.i81 = load ptr, ptr %10, align 32, !dbg !2251, !alias.scope !1687, !noalias !1696, !nonnull !11, !noundef !11
  %_112.i83 = getelementptr inbounds nuw float, ptr %_165.0.i81, i64 %_43.i78, !dbg !2276
  store <8 x float> %lanes.i1766.sroa.0.0.copyload, ptr %_112.i83, align 4, !dbg !2281, !alias.scope !2286, !noalias !2290
  %_166.1.i84 = load i64, ptr %12, align 8, !dbg !2292, !alias.scope !1687, !noalias !1696, !noundef !11
  %_113.i85 = icmp ugt i64 %_43.i78, %_166.1.i84, !dbg !2293
  br i1 %_113.i85, label %bb35.i121, label %bb36.i86, !dbg !2293, !prof !161

bb33.i122:                                        ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i72
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_43.i78, i64 noundef %_165.1.i77, i64 noundef %_165.1.i77, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7fd19a98f6b7f6813fdc6b43fefd82a4) #23, !dbg !2297, !noalias !1774
  unreachable, !dbg !2297

bb36.i86:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2370
  %_116.i88 = sub nuw i64 %_166.1.i84, %_43.i78, !dbg !2298
  %_8.i2362 = icmp samesign ugt i64 %_116.i88, 7, !dbg !2299
  br i1 %_8.i2362, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2365, label %bb2.i2363, !dbg !2299, !prof !2116

bb2.i2363:                                        ; preds = %bb36.i86
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_116.i88, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !2304, !noalias !2305
  unreachable, !dbg !2304

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2365: ; preds = %bb36.i86
  %_166.0.i87 = load ptr, ptr %13, align 16, !dbg !2292, !alias.scope !1687, !noalias !1696, !nonnull !11, !noundef !11
  %_120.i89 = getelementptr inbounds nuw float, ptr %_166.0.i87, i64 %_43.i78, !dbg !2309
  store <8 x float> %59, ptr %_120.i89, align 4, !dbg !2314, !alias.scope !2318, !noalias !2322
  %_167.1.i90 = load i64, ptr %15, align 8, !dbg !2324, !alias.scope !1690, !noalias !2325, !noundef !11
  %_121.i91 = icmp ugt i64 %_43.i78, %_167.1.i90, !dbg !2326
  br i1 %_121.i91, label %bb37.i120, label %bb38.i92, !dbg !2326, !prof !161

bb35.i121:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2370
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_43.i78, i64 noundef %_166.1.i84, i64 noundef %_166.1.i84, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_3898a5b9e77e549bf29e2ec14b43830a) #23, !dbg !2330, !noalias !1774
  unreachable, !dbg !2330

bb38.i92:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2365
  %_124.i94 = sub nuw i64 %_167.1.i90, %_43.i78, !dbg !2331
  %_8.i2357 = icmp samesign ugt i64 %_124.i94, 7, !dbg !2332
  br i1 %_8.i2357, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2360, label %bb2.i2358, !dbg !2332, !prof !2116

bb2.i2358:                                        ; preds = %bb38.i92
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_124.i94, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !2337, !noalias !2338
  unreachable, !dbg !2337

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2360: ; preds = %bb38.i92
  %_167.0.i93 = load ptr, ptr %14, align 32, !dbg !2324, !alias.scope !1690, !noalias !2325, !nonnull !11, !noundef !11
  %_128.i95 = getelementptr inbounds nuw float, ptr %_167.0.i93, i64 %_43.i78, !dbg !2342
  store <8 x float> %lanes.i1757.sroa.0.0.copyload, ptr %_128.i95, align 4, !dbg !2347, !alias.scope !2351, !noalias !2355
  %_168.1.i96 = load i64, ptr %16, align 8, !dbg !2357, !alias.scope !1690, !noalias !2325, !noundef !11
  %_129.i97 = icmp ugt i64 %_43.i78, %_168.1.i96, !dbg !2358
  br i1 %_129.i97, label %bb39.i119, label %bb40.i98, !dbg !2358, !prof !161

bb37.i120:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2365
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_43.i78, i64 noundef %_167.1.i90, i64 noundef %_167.1.i90, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ef27b7b7ba2d23db7b66788c02ac8976) #23, !dbg !2362, !noalias !1774
  unreachable, !dbg !2362

bb40.i98:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2360
  %_132.i100 = sub nuw i64 %_168.1.i96, %_43.i78, !dbg !2363
  %_8.i2352 = icmp samesign ugt i64 %_132.i100, 7, !dbg !2364
  br i1 %_8.i2352, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2355, label %bb2.i2353, !dbg !2364, !prof !2116

bb2.i2353:                                        ; preds = %bb40.i98
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_132.i100, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !2369, !noalias !2370
  unreachable, !dbg !2369

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2355: ; preds = %bb40.i98
  %_168.0.i99 = load ptr, ptr %17, align 16, !dbg !2357, !alias.scope !1690, !noalias !2325, !nonnull !11, !noundef !11
  %_136.i101 = getelementptr inbounds nuw float, ptr %_168.0.i99, i64 %_43.i78, !dbg !2374
  store <8 x float> %60, ptr %_136.i101, align 4, !dbg !2379, !alias.scope !2383, !noalias !2387
  %_169.1.i102 = load i64, ptr %11, align 8, !dbg !2389, !alias.scope !1687, !noalias !1696, !noundef !11
  %_55.i103 = shl nuw nsw i64 %spec.store.select.i76, 3, !dbg !2390
  %_137.i104 = icmp ugt i64 %_55.i103, %_169.1.i102, !dbg !2391
  br i1 %_137.i104, label %bb41.i118, label %bb42.i105, !dbg !2391, !prof !161

bb39.i119:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2360
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_43.i78, i64 noundef %_168.1.i96, i64 noundef %_168.1.i96, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_23eb2a707ff31dba52171dada526a36f) #23, !dbg !2395, !noalias !1774
  unreachable, !dbg !2395

bb42.i105:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2355
  %_140.i107 = sub nuw i64 %_169.1.i102, %_55.i103, !dbg !2396
  %_8.i1733 = icmp samesign ugt i64 %_140.i107, 7, !dbg !2397
  br i1 %_8.i1733, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1737, label %bb2.i1734, !dbg !2397, !prof !2116

bb2.i1734:                                        ; preds = %bb42.i105
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_140.i107, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !2402, !noalias !2403
  unreachable, !dbg !2402

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1737: ; preds = %bb42.i105
  %_169.0.i106 = load ptr, ptr %10, align 32, !dbg !2389, !alias.scope !1687, !noalias !1696, !nonnull !11, !noundef !11
  %_144.i108 = getelementptr inbounds nuw float, ptr %_169.0.i106, i64 %_55.i103, !dbg !2407
  %lanes.i1730.sroa.0.0.copyload = load <8 x float>, ptr %_144.i108, align 4, !dbg !2412, !alias.scope !2416, !noalias !2420
  %_170.1.i109 = load i64, ptr %15, align 8, !dbg !2422, !alias.scope !1690, !noalias !2325, !noundef !11
  %_145.i110 = icmp ugt i64 %_55.i103, %_170.1.i109, !dbg !2424
  br i1 %_145.i110, label %bb43.i117, label %bb44.i111, !dbg !2424, !prof !161

bb41.i118:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2355
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_55.i103, i64 noundef %_169.1.i102, i64 noundef %_169.1.i102, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e74b405e18c5e1dc98b750b7a797cfef) #23, !dbg !2428, !noalias !1774
  unreachable, !dbg !2428

bb44.i111:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1737
  %_148.i113 = sub nuw i64 %_170.1.i109, %_55.i103, !dbg !2429
  %_8.i1725 = icmp samesign ugt i64 %_148.i113, 7, !dbg !2430
  br i1 %_8.i1725, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1728, label %bb2.i, !dbg !2430, !prof !2116

bb2.i:                                            ; preds = %bb44.i111
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_148.i113, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !2435, !noalias !2436
  unreachable, !dbg !2435

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1728: ; preds = %bb44.i111
  %_170.0.i112 = load ptr, ptr %14, align 32, !dbg !2422, !alias.scope !1690, !noalias !2325, !nonnull !11, !noundef !11
  %_152.i114 = getelementptr inbounds nuw float, ptr %_170.0.i112, i64 %_55.i103, !dbg !2440
  %lanes.i1723.sroa.0.0.copyload = load <8 x float>, ptr %_152.i114, align 4, !dbg !2445, !alias.scope !2449, !noalias !2453
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2455), !dbg !2458
  %_5.i174 = load i32, ptr %1, align 4, !dbg !2460, !alias.scope !2455, !noalias !2464, !noundef !11
  %_38.1.i194 = load i64, ptr %12, align 8
  %_38.0.i198 = load ptr, ptr %13, align 16, !nonnull !11
  %_17.i186 = load i32, ptr %18, align 4, !dbg !2467, !alias.scope !2455, !noalias !2464, !noundef !11
  %delay.i187 = zext i32 %_17.i186 to i64, !dbg !2467
  %_20.not.i188 = icmp ult i32 %_37.i73, %_17.i186, !dbg !2471
  %narrow = select i1 %_20.not.i188, i32 %_5.i174, i32 0, !dbg !2471
  %_21.i189 = zext i32 %narrow to i64, !dbg !2471
  %write.pn.i190 = sub nsw i64 %write.i74, %delay.i187, !dbg !2471
  %tap.sroa.0.0.i191 = add nsw i64 %write.pn.i190, %_21.i189, !dbg !2473
  %_24.i192 = shl nsw i64 %tap.sroa.0.0.i191, 3, !dbg !2474
  %_26.i195 = icmp ult i64 %_24.i192, %_38.1.i194, !dbg !2476
  br i1 %_26.i195, label %bb10.i197, label %panic1.i196, !dbg !2476

bb10.i197:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1728
  %62 = getelementptr inbounds nuw float, ptr %_38.0.i198, i64 %_24.i192, !dbg !2476
  %_22.i199 = load float, ptr %62, align 4, !dbg !2476, !noalias !2477, !noundef !11
  %_17.i186.1 = load i32, ptr %29, align 4, !dbg !2467, !alias.scope !2455, !noalias !2464, !noundef !11
  %delay.i187.1 = zext i32 %_17.i186.1 to i64, !dbg !2467
  %_20.not.i188.1 = icmp ult i32 %_37.i73, %_17.i186.1, !dbg !2471
  %narrow.1 = select i1 %_20.not.i188.1, i32 %_5.i174, i32 0, !dbg !2471
  %_21.i189.1 = zext i32 %narrow.1 to i64, !dbg !2471
  %write.pn.i190.1 = sub nsw i64 %write.i74, %delay.i187.1, !dbg !2471
  %tap.sroa.0.0.i191.1 = add nsw i64 %write.pn.i190.1, %_21.i189.1, !dbg !2473
  %_24.i192.1 = shl nsw i64 %tap.sroa.0.0.i191.1, 3, !dbg !2474
  %_23.i193.1 = or disjoint i64 %_24.i192.1, 1, !dbg !2474
  %_26.i195.1 = icmp ult i64 %_23.i193.1, %_38.1.i194, !dbg !2476
  br i1 %_26.i195.1, label %bb10.i197.1, label %panic1.i196, !dbg !2476

bb10.i197.1:                                      ; preds = %bb10.i197
  %63 = getelementptr inbounds nuw float, ptr %_38.0.i198, i64 %_23.i193.1, !dbg !2476
  %_22.i199.1 = load float, ptr %63, align 4, !dbg !2476, !noalias !2477, !noundef !11
  %_17.i186.2 = load i32, ptr %30, align 4, !dbg !2467, !alias.scope !2455, !noalias !2464, !noundef !11
  %delay.i187.2 = zext i32 %_17.i186.2 to i64, !dbg !2467
  %_20.not.i188.2 = icmp ult i32 %_37.i73, %_17.i186.2, !dbg !2471
  %narrow.2 = select i1 %_20.not.i188.2, i32 %_5.i174, i32 0, !dbg !2471
  %_21.i189.2 = zext i32 %narrow.2 to i64, !dbg !2471
  %write.pn.i190.2 = sub nsw i64 %write.i74, %delay.i187.2, !dbg !2471
  %tap.sroa.0.0.i191.2 = add nsw i64 %write.pn.i190.2, %_21.i189.2, !dbg !2473
  %_24.i192.2 = shl nsw i64 %tap.sroa.0.0.i191.2, 3, !dbg !2474
  %_23.i193.2 = or disjoint i64 %_24.i192.2, 2, !dbg !2474
  %_26.i195.2 = icmp ult i64 %_23.i193.2, %_38.1.i194, !dbg !2476
  br i1 %_26.i195.2, label %bb10.i197.2, label %panic1.i196, !dbg !2476

bb10.i197.2:                                      ; preds = %bb10.i197.1
  %64 = getelementptr inbounds nuw float, ptr %_38.0.i198, i64 %_23.i193.2, !dbg !2476
  %_22.i199.2 = load float, ptr %64, align 4, !dbg !2476, !noalias !2477, !noundef !11
  %_17.i186.3 = load i32, ptr %31, align 4, !dbg !2467, !alias.scope !2455, !noalias !2464, !noundef !11
  %delay.i187.3 = zext i32 %_17.i186.3 to i64, !dbg !2467
  %_20.not.i188.3 = icmp ult i32 %_37.i73, %_17.i186.3, !dbg !2471
  %narrow.3 = select i1 %_20.not.i188.3, i32 %_5.i174, i32 0, !dbg !2471
  %_21.i189.3 = zext i32 %narrow.3 to i64, !dbg !2471
  %write.pn.i190.3 = sub nsw i64 %write.i74, %delay.i187.3, !dbg !2471
  %tap.sroa.0.0.i191.3 = add nsw i64 %write.pn.i190.3, %_21.i189.3, !dbg !2473
  %_24.i192.3 = shl nsw i64 %tap.sroa.0.0.i191.3, 3, !dbg !2474
  %_23.i193.3 = or disjoint i64 %_24.i192.3, 3, !dbg !2474
  %_26.i195.3 = icmp ult i64 %_23.i193.3, %_38.1.i194, !dbg !2476
  br i1 %_26.i195.3, label %bb10.i197.3, label %panic1.i196, !dbg !2476

bb10.i197.3:                                      ; preds = %bb10.i197.2
  %65 = getelementptr inbounds nuw float, ptr %_38.0.i198, i64 %_23.i193.3, !dbg !2476
  %_22.i199.3 = load float, ptr %65, align 4, !dbg !2476, !noalias !2477, !noundef !11
  %_17.i186.4 = load i32, ptr %32, align 4, !dbg !2467, !alias.scope !2455, !noalias !2464, !noundef !11
  %delay.i187.4 = zext i32 %_17.i186.4 to i64, !dbg !2467
  %_20.not.i188.4 = icmp ult i32 %_37.i73, %_17.i186.4, !dbg !2471
  %narrow.4 = select i1 %_20.not.i188.4, i32 %_5.i174, i32 0, !dbg !2471
  %_21.i189.4 = zext i32 %narrow.4 to i64, !dbg !2471
  %write.pn.i190.4 = sub nsw i64 %write.i74, %delay.i187.4, !dbg !2471
  %tap.sroa.0.0.i191.4 = add nsw i64 %write.pn.i190.4, %_21.i189.4, !dbg !2473
  %_24.i192.4 = shl nsw i64 %tap.sroa.0.0.i191.4, 3, !dbg !2474
  %_23.i193.4 = or disjoint i64 %_24.i192.4, 4, !dbg !2474
  %_26.i195.4 = icmp ult i64 %_23.i193.4, %_38.1.i194, !dbg !2476
  br i1 %_26.i195.4, label %bb10.i197.4, label %panic1.i196, !dbg !2476

bb10.i197.4:                                      ; preds = %bb10.i197.3
  %66 = getelementptr inbounds nuw float, ptr %_38.0.i198, i64 %_23.i193.4, !dbg !2476
  %_22.i199.4 = load float, ptr %66, align 4, !dbg !2476, !noalias !2477, !noundef !11
  %_17.i186.5 = load i32, ptr %33, align 4, !dbg !2467, !alias.scope !2455, !noalias !2464, !noundef !11
  %delay.i187.5 = zext i32 %_17.i186.5 to i64, !dbg !2467
  %_20.not.i188.5 = icmp ult i32 %_37.i73, %_17.i186.5, !dbg !2471
  %narrow.5 = select i1 %_20.not.i188.5, i32 %_5.i174, i32 0, !dbg !2471
  %_21.i189.5 = zext i32 %narrow.5 to i64, !dbg !2471
  %write.pn.i190.5 = sub nsw i64 %write.i74, %delay.i187.5, !dbg !2471
  %tap.sroa.0.0.i191.5 = add nsw i64 %write.pn.i190.5, %_21.i189.5, !dbg !2473
  %_24.i192.5 = shl nsw i64 %tap.sroa.0.0.i191.5, 3, !dbg !2474
  %_23.i193.5 = or disjoint i64 %_24.i192.5, 5, !dbg !2474
  %_26.i195.5 = icmp ult i64 %_23.i193.5, %_38.1.i194, !dbg !2476
  br i1 %_26.i195.5, label %bb10.i197.5, label %panic1.i196, !dbg !2476

bb10.i197.5:                                      ; preds = %bb10.i197.4
  %67 = getelementptr inbounds nuw float, ptr %_38.0.i198, i64 %_23.i193.5, !dbg !2476
  %_22.i199.5 = load float, ptr %67, align 4, !dbg !2476, !noalias !2477, !noundef !11
  %_17.i186.6 = load i32, ptr %34, align 4, !dbg !2467, !alias.scope !2455, !noalias !2464, !noundef !11
  %delay.i187.6 = zext i32 %_17.i186.6 to i64, !dbg !2467
  %_20.not.i188.6 = icmp ult i32 %_37.i73, %_17.i186.6, !dbg !2471
  %narrow.6 = select i1 %_20.not.i188.6, i32 %_5.i174, i32 0, !dbg !2471
  %_21.i189.6 = zext i32 %narrow.6 to i64, !dbg !2471
  %write.pn.i190.6 = sub nsw i64 %write.i74, %delay.i187.6, !dbg !2471
  %tap.sroa.0.0.i191.6 = add nsw i64 %write.pn.i190.6, %_21.i189.6, !dbg !2473
  %_24.i192.6 = shl nsw i64 %tap.sroa.0.0.i191.6, 3, !dbg !2474
  %_23.i193.6 = or disjoint i64 %_24.i192.6, 6, !dbg !2474
  %_26.i195.6 = icmp ult i64 %_23.i193.6, %_38.1.i194, !dbg !2476
  br i1 %_26.i195.6, label %bb6.i185.7, label %panic1.i196, !dbg !2476

bb6.i185.7:                                       ; preds = %bb10.i197.5
  %_17.i186.7 = load i32, ptr %35, align 4, !dbg !2467, !alias.scope !2455, !noalias !2464, !noundef !11
  %delay.i187.7 = zext i32 %_17.i186.7 to i64, !dbg !2467
  %_20.not.i188.7 = icmp ult i32 %_37.i73, %_17.i186.7, !dbg !2471
  %narrow.7 = select i1 %_20.not.i188.7, i32 %_5.i174, i32 0, !dbg !2471
  %_21.i189.7 = zext i32 %narrow.7 to i64, !dbg !2471
  %write.pn.i190.7 = sub nsw i64 %write.i74, %delay.i187.7, !dbg !2471
  %tap.sroa.0.0.i191.7 = add nsw i64 %write.pn.i190.7, %_21.i189.7, !dbg !2473
  %_24.i192.7 = shl nsw i64 %tap.sroa.0.0.i191.7, 3, !dbg !2474
  %_23.i193.7 = or disjoint i64 %_24.i192.7, 7, !dbg !2474
  %_26.i195.7 = icmp ult i64 %_23.i193.7, %_38.1.i194, !dbg !2476
  br i1 %_26.i195.7, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit200, label %panic1.i196, !dbg !2476

panic1.i196:                                      ; preds = %bb6.i185.7, %bb10.i197.5, %bb10.i197.4, %bb10.i197.3, %bb10.i197.2, %bb10.i197.1, %bb10.i197, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1728
  %_23.i193.lcssa = phi i64 [ %_24.i192, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1728 ], [ %_23.i193.1, %bb10.i197 ], [ %_23.i193.2, %bb10.i197.1 ], [ %_23.i193.3, %bb10.i197.2 ], [ %_23.i193.4, %bb10.i197.3 ], [ %_23.i193.5, %bb10.i197.4 ], [ %_23.i193.6, %bb10.i197.5 ], [ %_23.i193.7, %bb6.i185.7 ], !dbg !2474
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_23.i193.lcssa, i64 noundef %_38.1.i194, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a529a882edb544ae33c686afc6011081) #23, !dbg !2476, !noalias !2477
  unreachable, !dbg !2476

_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit200: ; preds = %bb6.i185.7
  %68 = getelementptr inbounds nuw float, ptr %_38.0.i198, i64 %_23.i193.6, !dbg !2476
  %_22.i199.6 = load float, ptr %68, align 4, !dbg !2476, !noalias !2477, !noundef !11
  %69 = getelementptr inbounds nuw float, ptr %_38.0.i198, i64 %_23.i193.7, !dbg !2476
  %_22.i199.7 = load float, ptr %69, align 4, !dbg !2476, !noalias !2477, !noundef !11
  %lanes.i1711.sroa.0.0.vec.insert = insertelement <8 x float> poison, float %_22.i199, i64 0, !dbg !2478
  %lanes.i1711.sroa.0.4.vec.insert = insertelement <8 x float> %lanes.i1711.sroa.0.0.vec.insert, float %_22.i199.1, i64 1, !dbg !2478
  %lanes.i1711.sroa.0.8.vec.insert = insertelement <8 x float> %lanes.i1711.sroa.0.4.vec.insert, float %_22.i199.2, i64 2, !dbg !2478
  %lanes.i1711.sroa.0.12.vec.insert = insertelement <8 x float> %lanes.i1711.sroa.0.8.vec.insert, float %_22.i199.3, i64 3, !dbg !2478
  %lanes.i1711.sroa.0.16.vec.insert = insertelement <8 x float> %lanes.i1711.sroa.0.12.vec.insert, float %_22.i199.4, i64 4, !dbg !2478
  %lanes.i1711.sroa.0.20.vec.insert = insertelement <8 x float> %lanes.i1711.sroa.0.16.vec.insert, float %_22.i199.5, i64 5, !dbg !2478
  %lanes.i1711.sroa.0.24.vec.insert = insertelement <8 x float> %lanes.i1711.sroa.0.20.vec.insert, float %_22.i199.6, i64 6, !dbg !2478
  %lanes.i1711.sroa.0.28.vec.insert = insertelement <8 x float> %lanes.i1711.sroa.0.24.vec.insert, float %_22.i199.7, i64 7, !dbg !2478
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2483), !dbg !2486
  %_5.i168 = load i32, ptr %19, align 4, !dbg !2488, !alias.scope !2483, !noalias !2490, !noundef !11
  %_38.1.i = load i64, ptr %16, align 8
  %_38.0.i = load ptr, ptr %17, align 16, !nonnull !11
  %_17.i171 = load i32, ptr %20, align 4, !dbg !2493, !alias.scope !2483, !noalias !2490, !noundef !11
  %delay.i = zext i32 %_17.i171 to i64, !dbg !2493
  %_20.not.i = icmp ult i32 %_37.i73, %_17.i171, !dbg !2494
  %narrow3949 = select i1 %_20.not.i, i32 %_5.i168, i32 0, !dbg !2494
  %_21.i = zext i32 %narrow3949 to i64, !dbg !2494
  %write.pn.i = sub nsw i64 %write.i74, %delay.i, !dbg !2494
  %tap.sroa.0.0.i = add nsw i64 %write.pn.i, %_21.i, !dbg !2495
  %_24.i = shl nsw i64 %tap.sroa.0.0.i, 3, !dbg !2496
  %_26.i = icmp ult i64 %_24.i, %_38.1.i, !dbg !2497
  br i1 %_26.i, label %bb10.i, label %panic1.i, !dbg !2497

bb10.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit200
  %70 = getelementptr inbounds nuw float, ptr %_38.0.i, i64 %_24.i, !dbg !2497
  %_22.i = load float, ptr %70, align 4, !dbg !2497, !noalias !2498, !noundef !11
  %_17.i171.1 = load i32, ptr %36, align 4, !dbg !2493, !alias.scope !2483, !noalias !2490, !noundef !11
  %delay.i.1 = zext i32 %_17.i171.1 to i64, !dbg !2493
  %_20.not.i.1 = icmp ult i32 %_37.i73, %_17.i171.1, !dbg !2494
  %narrow3949.1 = select i1 %_20.not.i.1, i32 %_5.i168, i32 0, !dbg !2494
  %_21.i.1 = zext i32 %narrow3949.1 to i64, !dbg !2494
  %write.pn.i.1 = sub nsw i64 %write.i74, %delay.i.1, !dbg !2494
  %tap.sroa.0.0.i.1 = add nsw i64 %write.pn.i.1, %_21.i.1, !dbg !2495
  %_24.i.1 = shl nsw i64 %tap.sroa.0.0.i.1, 3, !dbg !2496
  %_23.i172.1 = or disjoint i64 %_24.i.1, 1, !dbg !2496
  %_26.i.1 = icmp ult i64 %_23.i172.1, %_38.1.i, !dbg !2497
  br i1 %_26.i.1, label %bb10.i.1, label %panic1.i, !dbg !2497

bb10.i.1:                                         ; preds = %bb10.i
  %71 = getelementptr inbounds nuw float, ptr %_38.0.i, i64 %_23.i172.1, !dbg !2497
  %_22.i.1 = load float, ptr %71, align 4, !dbg !2497, !noalias !2498, !noundef !11
  %_17.i171.2 = load i32, ptr %37, align 4, !dbg !2493, !alias.scope !2483, !noalias !2490, !noundef !11
  %delay.i.2 = zext i32 %_17.i171.2 to i64, !dbg !2493
  %_20.not.i.2 = icmp ult i32 %_37.i73, %_17.i171.2, !dbg !2494
  %narrow3949.2 = select i1 %_20.not.i.2, i32 %_5.i168, i32 0, !dbg !2494
  %_21.i.2 = zext i32 %narrow3949.2 to i64, !dbg !2494
  %write.pn.i.2 = sub nsw i64 %write.i74, %delay.i.2, !dbg !2494
  %tap.sroa.0.0.i.2 = add nsw i64 %write.pn.i.2, %_21.i.2, !dbg !2495
  %_24.i.2 = shl nsw i64 %tap.sroa.0.0.i.2, 3, !dbg !2496
  %_23.i172.2 = or disjoint i64 %_24.i.2, 2, !dbg !2496
  %_26.i.2 = icmp ult i64 %_23.i172.2, %_38.1.i, !dbg !2497
  br i1 %_26.i.2, label %bb10.i.2, label %panic1.i, !dbg !2497

bb10.i.2:                                         ; preds = %bb10.i.1
  %72 = getelementptr inbounds nuw float, ptr %_38.0.i, i64 %_23.i172.2, !dbg !2497
  %_22.i.2 = load float, ptr %72, align 4, !dbg !2497, !noalias !2498, !noundef !11
  %_17.i171.3 = load i32, ptr %38, align 4, !dbg !2493, !alias.scope !2483, !noalias !2490, !noundef !11
  %delay.i.3 = zext i32 %_17.i171.3 to i64, !dbg !2493
  %_20.not.i.3 = icmp ult i32 %_37.i73, %_17.i171.3, !dbg !2494
  %narrow3949.3 = select i1 %_20.not.i.3, i32 %_5.i168, i32 0, !dbg !2494
  %_21.i.3 = zext i32 %narrow3949.3 to i64, !dbg !2494
  %write.pn.i.3 = sub nsw i64 %write.i74, %delay.i.3, !dbg !2494
  %tap.sroa.0.0.i.3 = add nsw i64 %write.pn.i.3, %_21.i.3, !dbg !2495
  %_24.i.3 = shl nsw i64 %tap.sroa.0.0.i.3, 3, !dbg !2496
  %_23.i172.3 = or disjoint i64 %_24.i.3, 3, !dbg !2496
  %_26.i.3 = icmp ult i64 %_23.i172.3, %_38.1.i, !dbg !2497
  br i1 %_26.i.3, label %bb10.i.3, label %panic1.i, !dbg !2497

bb10.i.3:                                         ; preds = %bb10.i.2
  %73 = getelementptr inbounds nuw float, ptr %_38.0.i, i64 %_23.i172.3, !dbg !2497
  %_22.i.3 = load float, ptr %73, align 4, !dbg !2497, !noalias !2498, !noundef !11
  %_17.i171.4 = load i32, ptr %39, align 4, !dbg !2493, !alias.scope !2483, !noalias !2490, !noundef !11
  %delay.i.4 = zext i32 %_17.i171.4 to i64, !dbg !2493
  %_20.not.i.4 = icmp ult i32 %_37.i73, %_17.i171.4, !dbg !2494
  %narrow3949.4 = select i1 %_20.not.i.4, i32 %_5.i168, i32 0, !dbg !2494
  %_21.i.4 = zext i32 %narrow3949.4 to i64, !dbg !2494
  %write.pn.i.4 = sub nsw i64 %write.i74, %delay.i.4, !dbg !2494
  %tap.sroa.0.0.i.4 = add nsw i64 %write.pn.i.4, %_21.i.4, !dbg !2495
  %_24.i.4 = shl nsw i64 %tap.sroa.0.0.i.4, 3, !dbg !2496
  %_23.i172.4 = or disjoint i64 %_24.i.4, 4, !dbg !2496
  %_26.i.4 = icmp ult i64 %_23.i172.4, %_38.1.i, !dbg !2497
  br i1 %_26.i.4, label %bb10.i.4, label %panic1.i, !dbg !2497

bb10.i.4:                                         ; preds = %bb10.i.3
  %74 = getelementptr inbounds nuw float, ptr %_38.0.i, i64 %_23.i172.4, !dbg !2497
  %_22.i.4 = load float, ptr %74, align 4, !dbg !2497, !noalias !2498, !noundef !11
  %_17.i171.5 = load i32, ptr %40, align 4, !dbg !2493, !alias.scope !2483, !noalias !2490, !noundef !11
  %delay.i.5 = zext i32 %_17.i171.5 to i64, !dbg !2493
  %_20.not.i.5 = icmp ult i32 %_37.i73, %_17.i171.5, !dbg !2494
  %narrow3949.5 = select i1 %_20.not.i.5, i32 %_5.i168, i32 0, !dbg !2494
  %_21.i.5 = zext i32 %narrow3949.5 to i64, !dbg !2494
  %write.pn.i.5 = sub nsw i64 %write.i74, %delay.i.5, !dbg !2494
  %tap.sroa.0.0.i.5 = add nsw i64 %write.pn.i.5, %_21.i.5, !dbg !2495
  %_24.i.5 = shl nsw i64 %tap.sroa.0.0.i.5, 3, !dbg !2496
  %_23.i172.5 = or disjoint i64 %_24.i.5, 5, !dbg !2496
  %_26.i.5 = icmp ult i64 %_23.i172.5, %_38.1.i, !dbg !2497
  br i1 %_26.i.5, label %bb10.i.5, label %panic1.i, !dbg !2497

bb10.i.5:                                         ; preds = %bb10.i.4
  %75 = getelementptr inbounds nuw float, ptr %_38.0.i, i64 %_23.i172.5, !dbg !2497
  %_22.i.5 = load float, ptr %75, align 4, !dbg !2497, !noalias !2498, !noundef !11
  %_17.i171.6 = load i32, ptr %41, align 4, !dbg !2493, !alias.scope !2483, !noalias !2490, !noundef !11
  %delay.i.6 = zext i32 %_17.i171.6 to i64, !dbg !2493
  %_20.not.i.6 = icmp ult i32 %_37.i73, %_17.i171.6, !dbg !2494
  %narrow3949.6 = select i1 %_20.not.i.6, i32 %_5.i168, i32 0, !dbg !2494
  %_21.i.6 = zext i32 %narrow3949.6 to i64, !dbg !2494
  %write.pn.i.6 = sub nsw i64 %write.i74, %delay.i.6, !dbg !2494
  %tap.sroa.0.0.i.6 = add nsw i64 %write.pn.i.6, %_21.i.6, !dbg !2495
  %_24.i.6 = shl nsw i64 %tap.sroa.0.0.i.6, 3, !dbg !2496
  %_23.i172.6 = or disjoint i64 %_24.i.6, 6, !dbg !2496
  %_26.i.6 = icmp ult i64 %_23.i172.6, %_38.1.i, !dbg !2497
  br i1 %_26.i.6, label %bb6.i.7, label %panic1.i, !dbg !2497

bb6.i.7:                                          ; preds = %bb10.i.5
  %_17.i171.7 = load i32, ptr %42, align 4, !dbg !2493, !alias.scope !2483, !noalias !2490, !noundef !11
  %delay.i.7 = zext i32 %_17.i171.7 to i64, !dbg !2493
  %_20.not.i.7 = icmp ult i32 %_37.i73, %_17.i171.7, !dbg !2494
  %narrow3949.7 = select i1 %_20.not.i.7, i32 %_5.i168, i32 0, !dbg !2494
  %_21.i.7 = zext i32 %narrow3949.7 to i64, !dbg !2494
  %write.pn.i.7 = sub nsw i64 %write.i74, %delay.i.7, !dbg !2494
  %tap.sroa.0.0.i.7 = add nsw i64 %write.pn.i.7, %_21.i.7, !dbg !2495
  %_24.i.7 = shl nsw i64 %tap.sroa.0.0.i.7, 3, !dbg !2496
  %_23.i172.7 = or disjoint i64 %_24.i.7, 7, !dbg !2496
  %_26.i.7 = icmp ult i64 %_23.i172.7, %_38.1.i, !dbg !2497
  br i1 %_26.i.7, label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit501, label %panic1.i, !dbg !2497

panic1.i:                                         ; preds = %bb6.i.7, %bb10.i.5, %bb10.i.4, %bb10.i.3, %bb10.i.2, %bb10.i.1, %bb10.i, %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit200
  %_23.i172.lcssa = phi i64 [ %_24.i, %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit200 ], [ %_23.i172.1, %bb10.i ], [ %_23.i172.2, %bb10.i.1 ], [ %_23.i172.3, %bb10.i.2 ], [ %_23.i172.4, %bb10.i.3 ], [ %_23.i172.5, %bb10.i.4 ], [ %_23.i172.6, %bb10.i.5 ], [ %_23.i172.7, %bb6.i.7 ], !dbg !2496
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_23.i172.lcssa, i64 noundef %_38.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a529a882edb544ae33c686afc6011081) #23, !dbg !2497, !noalias !2498
  unreachable, !dbg !2497

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit501: ; preds = %bb6.i.7
  %76 = getelementptr inbounds nuw float, ptr %_38.0.i, i64 %_23.i172.6, !dbg !2497
  %_22.i.6 = load float, ptr %76, align 4, !dbg !2497, !noalias !2498, !noundef !11
  %77 = getelementptr inbounds nuw float, ptr %_38.0.i, i64 %_23.i172.7, !dbg !2497
  %_22.i.7 = load float, ptr %77, align 4, !dbg !2497, !noalias !2498, !noundef !11
  %lanes.i1717.sroa.0.0.vec.insert = insertelement <8 x float> poison, float %_22.i, i64 0, !dbg !2499
  %lanes.i1717.sroa.0.4.vec.insert = insertelement <8 x float> %lanes.i1717.sroa.0.0.vec.insert, float %_22.i.1, i64 1, !dbg !2499
  %lanes.i1717.sroa.0.8.vec.insert = insertelement <8 x float> %lanes.i1717.sroa.0.4.vec.insert, float %_22.i.2, i64 2, !dbg !2499
  %lanes.i1717.sroa.0.12.vec.insert = insertelement <8 x float> %lanes.i1717.sroa.0.8.vec.insert, float %_22.i.3, i64 3, !dbg !2499
  %lanes.i1717.sroa.0.16.vec.insert = insertelement <8 x float> %lanes.i1717.sroa.0.12.vec.insert, float %_22.i.4, i64 4, !dbg !2499
  %lanes.i1717.sroa.0.20.vec.insert = insertelement <8 x float> %lanes.i1717.sroa.0.16.vec.insert, float %_22.i.5, i64 5, !dbg !2499
  %lanes.i1717.sroa.0.24.vec.insert = insertelement <8 x float> %lanes.i1717.sroa.0.20.vec.insert, float %_22.i.6, i64 6, !dbg !2499
  %lanes.i1717.sroa.0.28.vec.insert = insertelement <8 x float> %lanes.i1717.sroa.0.24.vec.insert, float %_22.i.7, i64 7, !dbg !2499
  %78 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i1711.sroa.0.28.vec.insert, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !2504
  %79 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %78, <8 x float> splat (float 0x3810000000000000)), !dbg !2514
  %80 = bitcast <8 x float> %79 to <4 x i64>, !dbg !2526
  %81 = and <4 x i64> %80, splat (i64 36028792732385279), !dbg !2527
  %82 = or disjoint <4 x i64> %81, splat (i64 4575657222473777152), !dbg !2545
  %83 = bitcast <4 x i64> %82 to <8 x float>, !dbg !2554
  %84 = fadd <8 x float> %83, splat (float -1.000000e+00), !dbg !2555
  %85 = fmul <8 x float> %84, splat (float 0xBF9B17A960000000), !dbg !2566
  %86 = fadd <8 x float> %85, splat (float 0x3FBF9A8440000000), !dbg !2574
  %87 = fmul <8 x float> %84, %86, !dbg !2566
  %88 = fadd <8 x float> %87, splat (float 0xBFD1E3F400000000), !dbg !2574
  %89 = fmul <8 x float> %84, %88, !dbg !2566
  %90 = fadd <8 x float> %89, splat (float 0x3FDD544F20000000), !dbg !2574
  %91 = fmul <8 x float> %84, %90, !dbg !2566
  %92 = fadd <8 x float> %91, splat (float 0xBFE6FC2A60000000), !dbg !2574
  %93 = fmul <8 x float> %84, %92, !dbg !2566
  %94 = fadd <8 x float> %93, splat (float 0x3FF714B2A0000000), !dbg !2574
  %95 = bitcast <8 x float> %79 to <8 x i32>, !dbg !2579
  %_3.i2585 = lshr <8 x i32> %95, splat (i32 23), !dbg !2589
  %96 = or disjoint <8 x i32> %_3.i2585, splat (i32 1258291200), !dbg !2590
  %97 = bitcast <8 x i32> %96 to <8 x float>, !dbg !2596
  %98 = fadd <8 x float> %97, splat (float 0xC160000FE0000000), !dbg !2597
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2603), !dbg !2606
  %_6.i343.sroa.0.0.copyload = load <8 x float>, ptr %_69.i115, align 32, !dbg !2608, !noalias !2611
  %99 = fmul <8 x float> %84, %94, !dbg !2614
  %100 = fadd <8 x float> %98, %99, !dbg !2619
  %101 = fmul <8 x float> %100, splat (float 0x4018151820000000), !dbg !2624
  %102 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %101, <8 x float> splat (float -1.600000e+02)), !dbg !2629
  %103 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %102, <8 x float> splat (float 2.400000e+01)), !dbg !2634
  %104 = fsub <8 x float> %103, %lanes.i1633.sroa.0.0.copyload, !dbg !2643
  %105 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %104, <8 x float> %lanes.i1621.sroa.0.0.copyload, i8 30), !dbg !2651
  %106 = fneg <8 x float> %lanes.i1621.sroa.0.0.copyload, !dbg !2664
  %107 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %104, <8 x float> %106, i8 18), !dbg !2674
  %108 = fadd <8 x float> %lanes.i1621.sroa.0.0.copyload, %104, !dbg !2686
  %109 = fmul <8 x float> %108, %108, !dbg !2693
  %110 = fmul <8 x float> %lanes.i1615.sroa.0.0.copyload, %109, !dbg !2699
  %111 = bitcast <8 x float> %105 to <8 x i32>, !dbg !2704
  %112 = icmp slt <8 x i32> %111, zeroinitializer, !dbg !2709
  %.v = select <8 x i1> %112, <8 x float> %104, <8 x float> %110, !dbg !2709
  %113 = fmul <8 x float> %lanes.i1627.sroa.0.0.copyload, %.v, !dbg !2709
  %114 = bitcast <8 x float> %107 to <8 x i32>, !dbg !2711
  %115 = icmp slt <8 x i32> %114, zeroinitializer, !dbg !2715
  %116 = select <8 x i1> %115, <8 x float> zeroinitializer, <8 x float> %113, !dbg !2715
  %117 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %116, <8 x float> splat (float -1.000000e+02)), !dbg !2717
  %118 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %117, <8 x float> zeroinitializer), !dbg !2722
  %119 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %118, <8 x float> %_6.i343.sroa.0.0.copyload, i8 17), !dbg !2727
  %120 = bitcast <8 x float> %119 to <8 x i32>, !dbg !2733
  %121 = icmp slt <8 x i32> %120, zeroinitializer, !dbg !2737
  %122 = select <8 x i1> %121, <8 x float> %lanes.i1609.sroa.0.0.copyload, <8 x float> %lanes.i1603.sroa.0.0.copyload, !dbg !2737
  %123 = fsub <8 x float> %118, %_6.i343.sroa.0.0.copyload, !dbg !2739
  %124 = fmul <8 x float> %123, %122, !dbg !2749
  %125 = fadd <8 x float> %_6.i343.sroa.0.0.copyload, %124, !dbg !2758
  %126 = bitcast <8 x float> %125 to <8 x i32>, !dbg !2765
  %127 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %125), !dbg !2773
  %128 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %127, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !2775
  %129 = bitcast <8 x float> %128 to <8 x i32>, !dbg !2781
  %130 = xor <8 x i32> %129, splat (i32 -1), !dbg !2793
  %131 = and <8 x i32> %130, %126, !dbg !2795
  store <8 x i32> %131, ptr %_69.i115, align 32, !dbg !2801, !alias.scope !2803, !noalias !2805
  %132 = bitcast <8 x i32> %131 to <8 x float>, !dbg !2806
  %133 = fadd <8 x float> %lanes.i1645.sroa.0.0.copyload, %132, !dbg !2807
  %134 = fmul <8 x float> %133, splat (float 0x3FC542A5A0000000), !dbg !2815
  %135 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %134, <8 x float> splat (float -1.260000e+02)), !dbg !2822
  %136 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %135, <8 x float> splat (float 1.270000e+02)), !dbg !2829
  %137 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %136), !dbg !2834
  %138 = fsub <8 x float> %136, %137, !dbg !2844
  %139 = fmul <8 x float> %138, splat (float 0x3F5E974FA0000000), !dbg !2850
  %140 = fadd <8 x float> %139, splat (float 0x3F82778560000000), !dbg !2858
  %141 = fmul <8 x float> %138, %140, !dbg !2850
  %142 = fadd <8 x float> %141, splat (float 0x3FAC91CE60000000), !dbg !2858
  %143 = fmul <8 x float> %138, %142, !dbg !2850
  %144 = fadd <8 x float> %143, splat (float 0x3FCEBDB560000000), !dbg !2858
  %145 = fmul <8 x float> %138, %144, !dbg !2850
  %146 = fadd <8 x float> %145, splat (float 0x3FE62E4BA0000000), !dbg !2858
  %147 = fmul <8 x float> %138, %146, !dbg !2863
  %148 = fadd <8 x float> %147, splat (float 1.000000e+00), !dbg !2868
  %149 = fadd <8 x float> %137, splat (float 0x4160000FE0000000), !dbg !2873
  %150 = bitcast <8 x float> %149 to <8 x i32>, !dbg !2882
  %_3.i2586 = shl <8 x i32> %150, splat (i32 23), !dbg !2892
  %151 = bitcast <8 x i32> %_3.i2586 to <8 x float>, !dbg !2893
  %152 = fmul <8 x float> %148, %151, !dbg !2895
  %153 = fmul <8 x float> %lanes.i1730.sroa.0.0.copyload, %152, !dbg !2899
  %154 = fsub <8 x float> %153, %lanes.i1730.sroa.0.0.copyload, !dbg !2905
  %155 = fmul <8 x float> %lanes.i1639.sroa.0.0.copyload, %154, !dbg !2916
  %156 = fadd <8 x float> %lanes.i1730.sroa.0.0.copyload, %155, !dbg !2922
  %157 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %132, <8 x float> zeroinitializer, i8 0), !dbg !2926
  %158 = bitcast <8 x float> %157 to <8 x i32>, !dbg !2933
  %159 = bitcast <8 x float> %46 to <8 x i32>, !dbg !2933
  %160 = and <8 x i32> %158, %159, !dbg !2937
  %161 = bitcast <8 x float> %45 to <8 x i32>, !dbg !2939
  %162 = or <8 x i32> %160, %161, !dbg !2947
  %163 = or <8 x i32> %162, %21, !dbg !2947
  %164 = bitcast <8 x float> %44 to <8 x i32>, !dbg !2955
  %165 = icmp slt <8 x i32> %164, zeroinitializer, !dbg !2960
  %166 = select <8 x i1> %165, <8 x float> %153, <8 x float> %156, !dbg !2960
  %167 = icmp slt <8 x i32> %163, zeroinitializer, !dbg !2962
  %168 = select <8 x i1> %167, <8 x float> %lanes.i1730.sroa.0.0.copyload, <8 x float> %166, !dbg !2962
  %169 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i1717.sroa.0.28.vec.insert, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !2968
  %170 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %169, <8 x float> splat (float 0x3810000000000000)), !dbg !2976
  %171 = bitcast <8 x float> %170 to <4 x i64>, !dbg !2983
  %172 = and <4 x i64> %171, splat (i64 36028792732385279), !dbg !2984
  %173 = or disjoint <4 x i64> %172, splat (i64 4575657222473777152), !dbg !2989
  %174 = bitcast <4 x i64> %173 to <8 x float>, !dbg !2993
  %175 = fadd <8 x float> %174, splat (float -1.000000e+00), !dbg !2994
  %176 = fmul <8 x float> %175, splat (float 0xBF9B17A960000000), !dbg !2999
  %177 = fadd <8 x float> %176, splat (float 0x3FBF9A8440000000), !dbg !3004
  %178 = fmul <8 x float> %175, %177, !dbg !2999
  %179 = fadd <8 x float> %178, splat (float 0xBFD1E3F400000000), !dbg !3004
  %180 = fmul <8 x float> %175, %179, !dbg !2999
  %181 = fadd <8 x float> %180, splat (float 0x3FDD544F20000000), !dbg !3004
  %182 = fmul <8 x float> %175, %181, !dbg !2999
  %183 = fadd <8 x float> %182, splat (float 0xBFE6FC2A60000000), !dbg !3004
  %184 = fmul <8 x float> %175, %183, !dbg !2999
  %185 = fadd <8 x float> %184, splat (float 0x3FF714B2A0000000), !dbg !3004
  %186 = bitcast <8 x float> %170 to <8 x i32>, !dbg !3009
  %_3.i2587 = lshr <8 x i32> %186, splat (i32 23), !dbg !3013
  %187 = or disjoint <8 x i32> %_3.i2587, splat (i32 1258291200), !dbg !3014
  %188 = bitcast <8 x i32> %187 to <8 x float>, !dbg !3018
  %189 = fadd <8 x float> %188, splat (float 0xC160000FE0000000), !dbg !3019
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3023), !dbg !3026
  %_6.i333.sroa.0.0.copyload = load <8 x float>, ptr %_73.i116, align 32, !dbg !3028, !noalias !3030
  %190 = fmul <8 x float> %175, %185, !dbg !3033
  %191 = fadd <8 x float> %189, %190, !dbg !3038
  %192 = fmul <8 x float> %191, splat (float 0x4018151820000000), !dbg !3043
  %193 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %192, <8 x float> splat (float -1.600000e+02)), !dbg !3048
  %194 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %193, <8 x float> splat (float 2.400000e+01)), !dbg !3053
  %195 = fsub <8 x float> %194, %lanes.i1681.sroa.0.0.copyload, !dbg !3058
  %196 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %195, <8 x float> %lanes.i1669.sroa.0.0.copyload, i8 30), !dbg !3064
  %197 = fneg <8 x float> %lanes.i1669.sroa.0.0.copyload, !dbg !3070
  %198 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %195, <8 x float> %197, i8 18), !dbg !3075
  %199 = fadd <8 x float> %lanes.i1669.sroa.0.0.copyload, %195, !dbg !3081
  %200 = fmul <8 x float> %199, %199, !dbg !3086
  %201 = fmul <8 x float> %lanes.i1663.sroa.0.0.copyload, %200, !dbg !3091
  %202 = bitcast <8 x float> %196 to <8 x i32>, !dbg !3096
  %203 = icmp slt <8 x i32> %202, zeroinitializer, !dbg !3100
  %.v3959 = select <8 x i1> %203, <8 x float> %195, <8 x float> %201, !dbg !3100
  %204 = fmul <8 x float> %lanes.i1675.sroa.0.0.copyload, %.v3959, !dbg !3100
  %205 = bitcast <8 x float> %198 to <8 x i32>, !dbg !3102
  %206 = icmp slt <8 x i32> %205, zeroinitializer, !dbg !3106
  %207 = select <8 x i1> %206, <8 x float> zeroinitializer, <8 x float> %204, !dbg !3106
  %208 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %207, <8 x float> splat (float -1.000000e+02)), !dbg !3108
  %209 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %208, <8 x float> zeroinitializer), !dbg !3113
  %210 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %209, <8 x float> %_6.i333.sroa.0.0.copyload, i8 17), !dbg !3118
  %211 = bitcast <8 x float> %210 to <8 x i32>, !dbg !3124
  %212 = icmp slt <8 x i32> %211, zeroinitializer, !dbg !3128
  %213 = select <8 x i1> %212, <8 x float> %lanes.i1657.sroa.0.0.copyload, <8 x float> %lanes.i1651.sroa.0.0.copyload, !dbg !3128
  %214 = fsub <8 x float> %209, %_6.i333.sroa.0.0.copyload, !dbg !3130
  %215 = fmul <8 x float> %214, %213, !dbg !3136
  %216 = fadd <8 x float> %_6.i333.sroa.0.0.copyload, %215, !dbg !3141
  %217 = bitcast <8 x float> %216 to <8 x i32>, !dbg !3145
  %218 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %216), !dbg !3151
  %219 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %218, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !3153
  %220 = bitcast <8 x float> %219 to <8 x i32>, !dbg !3159
  %221 = xor <8 x i32> %220, splat (i32 -1), !dbg !3165
  %222 = and <8 x i32> %221, %217, !dbg !3167
  store <8 x i32> %222, ptr %_73.i116, align 32, !dbg !3171, !alias.scope !3172, !noalias !3174
  %223 = bitcast <8 x i32> %222 to <8 x float>, !dbg !3175
  %224 = fadd <8 x float> %lanes.i1693.sroa.0.0.copyload, %223, !dbg !3176
  %225 = fmul <8 x float> %224, splat (float 0x3FC542A5A0000000), !dbg !3183
  %226 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %225, <8 x float> splat (float -1.260000e+02)), !dbg !3189
  %227 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %226, <8 x float> splat (float 1.270000e+02)), !dbg !3195
  %228 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %227), !dbg !3200
  %229 = fsub <8 x float> %227, %228, !dbg !3205
  %230 = fmul <8 x float> %229, splat (float 0x3F5E974FA0000000), !dbg !3210
  %231 = fadd <8 x float> %230, splat (float 0x3F82778560000000), !dbg !3215
  %232 = fmul <8 x float> %229, %231, !dbg !3210
  %233 = fadd <8 x float> %232, splat (float 0x3FAC91CE60000000), !dbg !3215
  %234 = fmul <8 x float> %229, %233, !dbg !3210
  %235 = fadd <8 x float> %234, splat (float 0x3FCEBDB560000000), !dbg !3215
  %236 = fmul <8 x float> %229, %235, !dbg !3210
  %237 = fadd <8 x float> %236, splat (float 0x3FE62E4BA0000000), !dbg !3215
  %238 = fmul <8 x float> %229, %237, !dbg !3220
  %239 = fadd <8 x float> %238, splat (float 1.000000e+00), !dbg !3225
  %240 = fadd <8 x float> %228, splat (float 0x4160000FE0000000), !dbg !3230
  %241 = bitcast <8 x float> %240 to <8 x i32>, !dbg !3235
  %_3.i2588 = shl <8 x i32> %241, splat (i32 23), !dbg !3239
  %242 = bitcast <8 x i32> %_3.i2588 to <8 x float>, !dbg !3240
  %243 = fmul <8 x float> %239, %242, !dbg !3242
  %244 = fmul <8 x float> %lanes.i1723.sroa.0.0.copyload, %243, !dbg !3246
  %245 = fsub <8 x float> %244, %lanes.i1723.sroa.0.0.copyload, !dbg !3251
  %246 = fmul <8 x float> %lanes.i1687.sroa.0.0.copyload, %245, !dbg !3257
  %247 = fadd <8 x float> %lanes.i1723.sroa.0.0.copyload, %246, !dbg !3262
  %248 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %223, <8 x float> zeroinitializer, i8 0), !dbg !3266
  %249 = bitcast <8 x float> %248 to <8 x i32>, !dbg !3272
  %250 = bitcast <8 x float> %49 to <8 x i32>, !dbg !3272
  %251 = and <8 x i32> %249, %250, !dbg !3276
  %252 = bitcast <8 x float> %48 to <8 x i32>, !dbg !3278
  %253 = or <8 x i32> %251, %252, !dbg !3282
  %254 = or <8 x i32> %253, %21, !dbg !3282
  %255 = bitcast <8 x float> %47 to <8 x i32>, !dbg !3287
  %256 = icmp slt <8 x i32> %255, zeroinitializer, !dbg !3291
  %257 = select <8 x i1> %256, <8 x float> %244, <8 x float> %247, !dbg !3291
  %258 = icmp slt <8 x i32> %254, zeroinitializer, !dbg !3293
  %259 = select <8 x i1> %258, <8 x float> %lanes.i1723.sroa.0.0.copyload, <8 x float> %257, !dbg !3293
  store <8 x float> %168, ptr %_96.i53, align 4, !dbg !3298, !alias.scope !3304, !noalias !3308
  store <8 x float> %259, ptr %_104.i57, align 4, !dbg !3312, !alias.scope !3317, !noalias !3321
  %260 = trunc i64 %spec.store.select.i76 to i32, !dbg !3325
  store i32 %260, ptr %9, align 32, !dbg !3325, !alias.scope !1687, !noalias !1696
  store i32 %260, ptr %22, align 32, !dbg !3326, !alias.scope !1690, !noalias !2325
  %exitcond5495.not = icmp eq i64 %43, %spec.store.select, !dbg !3327
  br i1 %exitcond5495.not, label %bb10, label %bb27.i48, !dbg !1763

bb43.i117:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1737
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_55.i103, i64 noundef %_170.1.i109, i64 noundef %_170.1.i109, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_714be7237772529829a33eae50e95afa) #23, !dbg !3330, !noalias !1774
  unreachable, !dbg !3330

bb10:                                             ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit501, %start
  br i1 %_24, label %bb11, label %bb19, !dbg !3331

bb11:                                             ; preds = %bb10
  %_34 = sub nsw i64 %frames, %spec.store.select, !dbg !3332
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3333), !dbg !3336
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3337), !dbg !3336
  %_4.i2589 = icmp ult i64 %_34, 129, !dbg !3339
  br i1 %_4.i2589, label %bb1.i1.preheader.i, label %bb15, !dbg !3339

bb1.i1.preheader.i:                               ; preds = %bb11
  %261 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1440
  %_5.i7.i = load i32, ptr %261, align 4, !dbg !3342, !alias.scope !3348, !noalias !3337, !noundef !11
  %262 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1444, !dbg !3342
  %_5.i7.1.i = load i32, ptr %262, align 4, !dbg !3342, !alias.scope !3348, !noalias !3337, !noundef !11
  %least.sroa.0.1.i9.1.i = tail call i32 @llvm.umin.i32(i32 %_5.i7.1.i, i32 %_5.i7.i), !dbg !3342
  %263 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1448, !dbg !3342
  %_5.i7.2.i = load i32, ptr %263, align 4, !dbg !3342, !alias.scope !3348, !noalias !3337, !noundef !11
  %least.sroa.0.1.i9.2.i = tail call i32 @llvm.umin.i32(i32 %_5.i7.2.i, i32 %least.sroa.0.1.i9.1.i), !dbg !3342
  %264 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1452, !dbg !3342
  %_5.i7.3.i = load i32, ptr %264, align 4, !dbg !3342, !alias.scope !3348, !noalias !3337, !noundef !11
  %least.sroa.0.1.i9.3.i = tail call i32 @llvm.umin.i32(i32 %_5.i7.3.i, i32 %least.sroa.0.1.i9.2.i), !dbg !3342
  %265 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1456, !dbg !3342
  %_5.i7.4.i = load i32, ptr %265, align 4, !dbg !3342, !alias.scope !3348, !noalias !3337, !noundef !11
  %least.sroa.0.1.i9.4.i = tail call i32 @llvm.umin.i32(i32 %_5.i7.4.i, i32 %least.sroa.0.1.i9.3.i), !dbg !3342
  %266 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1460, !dbg !3342
  %_5.i7.5.i = load i32, ptr %266, align 4, !dbg !3342, !alias.scope !3348, !noalias !3337, !noundef !11
  %least.sroa.0.1.i9.5.i = tail call i32 @llvm.umin.i32(i32 %_5.i7.5.i, i32 %least.sroa.0.1.i9.4.i), !dbg !3342
  %267 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1464, !dbg !3342
  %_5.i7.6.i = load i32, ptr %267, align 4, !dbg !3342, !alias.scope !3348, !noalias !3337, !noundef !11
  %least.sroa.0.1.i9.6.i = tail call i32 @llvm.umin.i32(i32 %_5.i7.6.i, i32 %least.sroa.0.1.i9.5.i), !dbg !3342
  %268 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1468, !dbg !3342
  %_5.i7.7.i = load i32, ptr %268, align 4, !dbg !3342, !alias.scope !3348, !noalias !3337, !noundef !11
  %least.sroa.0.1.i9.7.i = tail call i32 @llvm.umin.i32(i32 %_5.i7.7.i, i32 %least.sroa.0.1.i9.6.i), !dbg !3342
  %_0.i5.i = zext i32 %least.sroa.0.1.i9.7.i to i64, !dbg !3351
  %_5.not.i = icmp samesign ugt i64 %_34, %_0.i5.i, !dbg !3352
  br i1 %_5.not.i, label %bb15, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel20segment_is_stageableNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit, !dbg !3352

_RINvNtCse3bfmKSZS8Y_10compressor6kernel20segment_is_stageableNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit: ; preds = %bb1.i1.preheader.i
  %269 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1440
  %_5.i.i = load i32, ptr %269, align 4, !dbg !3353, !alias.scope !3355, !noalias !3333, !noundef !11
  %270 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1444, !dbg !3353
  %_5.i.1.i = load i32, ptr %270, align 4, !dbg !3353, !alias.scope !3355, !noalias !3333, !noundef !11
  %least.sroa.0.1.i.1.i = tail call i32 @llvm.umin.i32(i32 %_5.i.1.i, i32 %_5.i.i), !dbg !3353
  %271 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1448, !dbg !3353
  %_5.i.2.i = load i32, ptr %271, align 4, !dbg !3353, !alias.scope !3355, !noalias !3333, !noundef !11
  %least.sroa.0.1.i.2.i = tail call i32 @llvm.umin.i32(i32 %_5.i.2.i, i32 %least.sroa.0.1.i.1.i), !dbg !3353
  %272 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1452, !dbg !3353
  %_5.i.3.i = load i32, ptr %272, align 4, !dbg !3353, !alias.scope !3355, !noalias !3333, !noundef !11
  %least.sroa.0.1.i.3.i = tail call i32 @llvm.umin.i32(i32 %_5.i.3.i, i32 %least.sroa.0.1.i.2.i), !dbg !3353
  %273 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1456, !dbg !3353
  %_5.i.4.i = load i32, ptr %273, align 4, !dbg !3353, !alias.scope !3355, !noalias !3333, !noundef !11
  %least.sroa.0.1.i.4.i = tail call i32 @llvm.umin.i32(i32 %_5.i.4.i, i32 %least.sroa.0.1.i.3.i), !dbg !3353
  %274 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1460, !dbg !3353
  %_5.i.5.i = load i32, ptr %274, align 4, !dbg !3353, !alias.scope !3355, !noalias !3333, !noundef !11
  %least.sroa.0.1.i.5.i = tail call i32 @llvm.umin.i32(i32 %_5.i.5.i, i32 %least.sroa.0.1.i.4.i), !dbg !3353
  %275 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1464, !dbg !3353
  %_5.i.6.i = load i32, ptr %275, align 4, !dbg !3353, !alias.scope !3355, !noalias !3333, !noundef !11
  %least.sroa.0.1.i.6.i = tail call i32 @llvm.umin.i32(i32 %_5.i.6.i, i32 %least.sroa.0.1.i.5.i), !dbg !3353
  %276 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1468, !dbg !3353
  %_5.i.7.i = load i32, ptr %276, align 4, !dbg !3353, !alias.scope !3355, !noalias !3333, !noundef !11
  %least.sroa.0.1.i.7.i = tail call i32 @llvm.umin.i32(i32 %_5.i.7.i, i32 %least.sroa.0.1.i.6.i), !dbg !3353
  %_0.i.i = zext i32 %least.sroa.0.1.i.7.i to i64, !dbg !3358
  %.not = icmp samesign ugt i64 %_34, %_0.i.i, !dbg !3359
  br i1 %.not, label %bb15, label %bb13, !dbg !3336

bb19:                                             ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit144.i, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit459, %bb10
  ret void, !dbg !3360

bb15:                                             ; preds = %bb11, %bb1.i1.preheader.i, %_RINvNtCse3bfmKSZS8Y_10compressor6kernel20segment_is_stageableNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3361), !dbg !3364
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3365), !dbg !3364
  %277 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1540, !dbg !3367
  %_12.i = load i32, ptr %277, align 4, !dbg !3367, !alias.scope !3361, !noalias !3371, !noundef !11
  %ring_length.i = zext i32 %_12.i to i64, !dbg !3367
  %278 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !3375
  %279 = bitcast <8 x float> %278 to <8 x i32>, !dbg !3383
  %280 = xor <8 x i32> %279, splat (i32 -1), !dbg !3389
  %281 = bitcast <8 x i32> %280 to <8 x float>, !dbg !3383
  switch i32 %link, label %bb13.i593 [
    i32 1, label %bb27.i.lr.ph
    i32 3, label %bb14.i594.fold.split
  ], !dbg !3391

bb13.i593:                                        ; preds = %bb15
  br label %bb27.i.lr.ph, !dbg !3392

bb14.i594.fold.split:                             ; preds = %bb15
  br label %bb27.i.lr.ph, !dbg !3393

bb27.i.lr.ph:                                     ; preds = %bb13.i593, %bb14.i594.fold.split, %bb15
  %_11.i582.sroa.0.03929 = phi <8 x float> [ %281, %bb15 ], [ %278, %bb13.i593 ], [ %278, %bb14.i594.fold.split ]
  %_13.i581.sroa.0.0 = phi <8 x i32> [ %280, %bb15 ], [ %280, %bb13.i593 ], [ %279, %bb14.i594.fold.split ], !dbg !3394
  %_4.i716 = getelementptr inbounds nuw i8, ptr %channels.0, i64 448, !dbg !3395
  %lanes.i1453.sroa.0.0.copyload = load <8 x float>, ptr %_4.i716, align 32, !dbg !3398, !alias.scope !3403, !noalias !3407
  %_7.i717 = getelementptr inbounds nuw i8, ptr %channels.0, i64 480, !dbg !3413
  %lanes.i1447.sroa.0.0.copyload = load <8 x float>, ptr %_7.i717, align 32, !dbg !3414, !alias.scope !3419, !noalias !3423
  %_15.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 256, !dbg !3427
  %lanes.i1441.sroa.0.0.copyload = load <8 x float>, ptr %_15.i, align 32, !dbg !3428, !alias.scope !3433, !noalias !3437
  %_14.i718 = getelementptr inbounds nuw i8, ptr %channels.0, i64 288, !dbg !3441
  %lanes.i1435.sroa.0.0.copyload = load <8 x float>, ptr %_14.i718, align 32, !dbg !3442, !alias.scope !3447, !noalias !3451
  %_17.i719 = getelementptr inbounds nuw i8, ptr %channels.0, i64 320, !dbg !3455
  %lanes.i1429.sroa.0.0.copyload = load <8 x float>, ptr %_17.i719, align 32, !dbg !3456, !alias.scope !3461, !noalias !3465
  %_20.i720 = getelementptr inbounds nuw i8, ptr %channels.0, i64 352, !dbg !3469
  %lanes.i1423.sroa.0.0.copyload = load <8 x float>, ptr %_20.i720, align 32, !dbg !3470, !alias.scope !3475, !noalias !3479
  %_23.i721 = getelementptr inbounds nuw i8, ptr %channels.0, i64 384, !dbg !3483
  %lanes.i1417.sroa.0.0.copyload = load <8 x float>, ptr %_23.i721, align 32, !dbg !3484, !alias.scope !3489, !noalias !3493
  %_26.i722 = getelementptr inbounds nuw i8, ptr %channels.0, i64 416, !dbg !3497
  %lanes.i.sroa.0.0.copyload = load <8 x float>, ptr %_26.i722, align 32, !dbg !3498, !alias.scope !3503, !noalias !3507
  %_4.i694 = getelementptr inbounds nuw i8, ptr %channels.1, i64 448, !dbg !3511
  %lanes.i1501.sroa.0.0.copyload = load <8 x float>, ptr %_4.i694, align 32, !dbg !3514, !alias.scope !3519, !noalias !3523
  %_7.i695 = getelementptr inbounds nuw i8, ptr %channels.1, i64 480, !dbg !3529
  %lanes.i1495.sroa.0.0.copyload = load <8 x float>, ptr %_7.i695, align 32, !dbg !3530, !alias.scope !3535, !noalias !3539
  %_17.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 256, !dbg !3543
  %lanes.i1489.sroa.0.0.copyload = load <8 x float>, ptr %_17.i, align 32, !dbg !3544, !alias.scope !3549, !noalias !3553
  %_14.i696 = getelementptr inbounds nuw i8, ptr %channels.1, i64 288, !dbg !3557
  %lanes.i1483.sroa.0.0.copyload = load <8 x float>, ptr %_14.i696, align 32, !dbg !3558, !alias.scope !3563, !noalias !3567
  %_17.i697 = getelementptr inbounds nuw i8, ptr %channels.1, i64 320, !dbg !3571
  %lanes.i1477.sroa.0.0.copyload = load <8 x float>, ptr %_17.i697, align 32, !dbg !3572, !alias.scope !3577, !noalias !3581
  %_20.i698 = getelementptr inbounds nuw i8, ptr %channels.1, i64 352, !dbg !3585
  %lanes.i1471.sroa.0.0.copyload = load <8 x float>, ptr %_20.i698, align 32, !dbg !3586, !alias.scope !3591, !noalias !3595
  %_23.i699 = getelementptr inbounds nuw i8, ptr %channels.1, i64 384, !dbg !3599
  %lanes.i1465.sroa.0.0.copyload = load <8 x float>, ptr %_23.i699, align 32, !dbg !3600, !alias.scope !3605, !noalias !3609
  %_26.i700 = getelementptr inbounds nuw i8, ptr %channels.1, i64 416, !dbg !3613
  %lanes.i1459.sroa.0.0.copyload = load <8 x float>, ptr %_26.i700, align 32, !dbg !3614, !alias.scope !3619, !noalias !3623
  %282 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i1501.sroa.0.0.copyload, <8 x float> zeroinitializer, i8 0), !dbg !3627
  %283 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i1495.sroa.0.0.copyload, <8 x float> zeroinitializer, i8 0), !dbg !3633
  %284 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i1495.sroa.0.0.copyload, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !3639
  %285 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i1453.sroa.0.0.copyload, <8 x float> zeroinitializer, i8 0), !dbg !3645
  %286 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i1447.sroa.0.0.copyload, <8 x float> zeroinitializer, i8 0), !dbg !3651
  %287 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i1447.sroa.0.0.copyload, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !3657
  %_6.i.i = load i64, ptr %detector, align 8, !range !220
  %288 = getelementptr inbounds nuw i8, ptr %detector, i64 16
  %sidechain_left.1.i.i = load i64, ptr %288, align 8
  %289 = getelementptr inbounds nuw i8, ptr %detector, i64 24
  %sidechain_right.0.i.i = load ptr, ptr %289, align 8, !nonnull !11, !align !3663
  %290 = getelementptr inbounds nuw i8, ptr %detector, i64 32
  %sidechain_right.1.i.i = load i64, ptr %290, align 8
  %291 = getelementptr inbounds nuw i8, ptr %detector, i64 8
  %sidechain_left.0.i.i = load ptr, ptr %291, align 8, !nonnull !11, !align !3663
  %292 = icmp slt <8 x i32> %_13.i581.sroa.0.0, zeroinitializer
  %293 = bitcast <8 x float> %_11.i582.sroa.0.03929 to <8 x i32>
  %294 = icmp slt <8 x i32> %293, zeroinitializer
  %295 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1536
  %296 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1504
  %297 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1512
  %298 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1528
  %299 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1520
  %300 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1504
  %301 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1512
  %302 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1528
  %303 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1520
  %304 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1440
  %305 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1540
  %306 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1440
  %_69.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 1472
  %307 = bitcast <8 x float> %285 to <8 x i32>
  %308 = bitcast <8 x float> %286 to <8 x i32>
  %309 = select i1 %bypass, <8 x i32> %279, <8 x i32> %280
  %310 = bitcast <8 x float> %287 to <8 x i32>
  %311 = icmp slt <8 x i32> %310, zeroinitializer
  %_73.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 1472
  %312 = bitcast <8 x float> %282 to <8 x i32>
  %313 = bitcast <8 x float> %283 to <8 x i32>
  %314 = bitcast <8 x float> %284 to <8 x i32>
  %315 = icmp slt <8 x i32> %314, zeroinitializer
  %316 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1536
  %317 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1444
  %318 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1448
  %319 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1452
  %320 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1456
  %321 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1460
  %322 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1464
  %323 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1468
  %324 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1444
  %325 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1448
  %326 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1452
  %327 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1456
  %328 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1460
  %329 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1464
  %330 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1468
  %331 = fneg <8 x float> %lanes.i1429.sroa.0.0.copyload
  %332 = fneg <8 x float> %lanes.i1477.sroa.0.0.copyload
  br label %bb27.i, !dbg !3664

bb27.i:                                           ; preds = %bb27.i.lr.ph, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit459
  %start1.sroa.0.0.i4715 = phi i64 [ %spec.store.select, %bb27.i.lr.ph ], [ %333, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit459 ]
  %333 = add nuw nsw i64 %start1.sroa.0.0.i4715, 1, !dbg !3673
  %slot.i = shl nuw nsw i64 %start1.sroa.0.0.i4715, 3, !dbg !3679
  %_89.i = icmp samesign ugt i64 %slot.i, %left.1, !dbg !3681
  br i1 %_89.i, label %bb29.i, label %bb30.i, !dbg !3681, !prof !161

bb30.i:                                           ; preds = %bb27.i
  %_92.i = sub nuw nsw i64 %left.1, %slot.i, !dbg !3687
  %_96.i = getelementptr inbounds nuw float, ptr %left.0, i64 %slot.i, !dbg !3688
  %_8.i1823 = icmp samesign ugt i64 %_92.i, 7, !dbg !3693
  br i1 %_8.i1823, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1827, label %bb2.i1824, !dbg !3693, !prof !2116

bb2.i1824:                                        ; preds = %bb30.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_92.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !3698, !noalias !3699
  unreachable, !dbg !3698

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1827: ; preds = %bb30.i
  %lanes.i1820.sroa.0.0.copyload = load <8 x float>, ptr %_96.i, align 4, !dbg !3703, !alias.scope !3707, !noalias !3711
  %_97.i = icmp samesign ugt i64 %slot.i, %right.1, !dbg !3713
  br i1 %_97.i, label %bb31.i, label %bb32.i, !dbg !3713, !prof !161

bb29.i:                                           ; preds = %bb27.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %slot.i, i64 noundef range(i64 8, 34359738361) %left.1, i64 noundef range(i64 8, 34359738361) %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c00de08b5cdb434b6310222ddd96a0fc) #23, !dbg !3718, !noalias !3719
  unreachable, !dbg !3718

bb32.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1827
  %_100.i = sub nuw nsw i64 %right.1, %slot.i, !dbg !3720
  %_104.i = getelementptr inbounds nuw float, ptr %right.0, i64 %slot.i, !dbg !3721
  %_8.i1814 = icmp samesign ugt i64 %_100.i, 7, !dbg !3726
  br i1 %_8.i1814, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1818, label %bb2.i1815, !dbg !3726, !prof !2116

bb2.i1815:                                        ; preds = %bb32.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_100.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !3731, !noalias !3732
  unreachable, !dbg !3731

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1818: ; preds = %bb32.i
  %lanes.i1811.sroa.0.0.copyload = load <8 x float>, ptr %_104.i, align 4, !dbg !3736, !alias.scope !3740, !noalias !3744
  switch i64 %_6.i.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1818.unreachabledefault [
    i64 0, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i
    i64 1, label %bb3.i.i
    i64 2, label %bb2.i.i
  ], !dbg !3746

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1818.unreachabledefault: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1818
  unreachable

default.unreachable:                              ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit686.i
  unreachable

bb3.i.i:                                          ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1818
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i, !dbg !3749

bb2.i.i:                                          ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1818
  %_25.i.i = icmp ugt i64 %slot.i, %sidechain_left.1.i.i, !dbg !3750
  br i1 %_25.i.i, label %bb17.i.i, label %bb18.i.i, !dbg !3750, !prof !161

bb18.i.i:                                         ; preds = %bb2.i.i
  %_28.i.i = sub nuw i64 %sidechain_left.1.i.i, %slot.i, !dbg !3753
  %_8.i1805 = icmp samesign ugt i64 %_28.i.i, 7, !dbg !3754
  br i1 %_8.i1805, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1809, label %bb2.i1806, !dbg !3754, !prof !2116

bb2.i1806:                                        ; preds = %bb18.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_28.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !3759, !noalias !3760
  unreachable, !dbg !3759

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1809: ; preds = %bb18.i.i
  %_32.i.i = getelementptr inbounds nuw float, ptr %sidechain_left.0.i.i, i64 %slot.i, !dbg !3769
  %lanes.i1802.sroa.0.0.copyload = load <8 x float>, ptr %_32.i.i, align 4, !dbg !3771, !alias.scope !3775, !noalias !3779
  %_33.i.i = icmp ugt i64 %slot.i, %sidechain_right.1.i.i, !dbg !3781
  br i1 %_33.i.i, label %bb19.i.i, label %bb20.i.i, !dbg !3781, !prof !161

bb17.i.i:                                         ; preds = %bb2.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 34359738361) %slot.i, i64 noundef %sidechain_left.1.i.i, i64 noundef %sidechain_left.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a0a4023c361dd50869bbb08b76482f50) #23, !dbg !3784, !noalias !3785
  unreachable, !dbg !3784

bb20.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1809
  %_36.i.i = sub nuw i64 %sidechain_right.1.i.i, %slot.i, !dbg !3787
  %_8.i1796 = icmp samesign ugt i64 %_36.i.i, 7, !dbg !3788
  br i1 %_8.i1796, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1800, label %bb2.i1797, !dbg !3788, !prof !2116

bb2.i1797:                                        ; preds = %bb20.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_36.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !3793, !noalias !3794
  unreachable, !dbg !3793

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1800: ; preds = %bb20.i.i
  %_40.i.i = getelementptr inbounds nuw float, ptr %sidechain_right.0.i.i, i64 %slot.i, !dbg !3798
  %lanes.i1793.sroa.0.0.copyload = load <8 x float>, ptr %_40.i.i, align 4, !dbg !3800, !alias.scope !3804, !noalias !3808
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i, !dbg !3810

bb19.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1809
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 34359738361) %slot.i, i64 noundef %sidechain_right.1.i.i, i64 noundef %sidechain_right.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5807a2a07e9839e3d32631b4243eb17f) #23, !dbg !3811, !noalias !3785
  unreachable, !dbg !3811

_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1818, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1800, %bb3.i.i
  %.sroa.02670.0 = phi <8 x float> [ %lanes.i1811.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1818 ], [ zeroinitializer, %bb3.i.i ], [ %lanes.i1793.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1800 ], !dbg !3812
  %.sroa.02667.0 = phi <8 x float> [ %lanes.i1820.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1818 ], [ zeroinitializer, %bb3.i.i ], [ %lanes.i1802.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1800 ], !dbg !3812
  %334 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %.sroa.02667.0), !dbg !3813
  %335 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %.sroa.02670.0), !dbg !3819
  %336 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %334, <8 x float> %335), !dbg !3825
  %337 = fmul <8 x float> %334, splat (float 5.000000e-01), !dbg !3830
  %338 = fmul <8 x float> %335, splat (float 5.000000e-01), !dbg !3835
  %339 = fadd <8 x float> %337, %338, !dbg !3840
  %340 = select <8 x i1> %292, <8 x float> %339, <8 x float> %336, !dbg !3845
  %341 = select <8 x i1> %294, <8 x float> %340, <8 x float> %334, !dbg !3850
  %342 = select <8 x i1> %294, <8 x float> %340, <8 x float> %335, !dbg !3855
  %_37.i = load i32, ptr %295, align 32, !dbg !3860, !alias.scope !3361, !noalias !3371, !noundef !11
  %write.i = zext i32 %_37.i to i64, !dbg !3860
  %343 = add nuw nsw i64 %write.i, 1, !dbg !3862
  %_39.i = icmp eq i64 %343, %ring_length.i, !dbg !3864
  %spec.store.select.i = select i1 %_39.i, i64 0, i64 %343, !dbg !3864
  %_165.1.i = load i64, ptr %297, align 8, !dbg !3866, !alias.scope !3361, !noalias !3371, !noundef !11
  %_43.i = shl nuw nsw i64 %write.i, 3, !dbg !3868
  %_105.i = icmp ugt i64 %_43.i, %_165.1.i, !dbg !3869
  br i1 %_105.i, label %bb33.i, label %bb34.i, !dbg !3869, !prof !161

bb31.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1827
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %slot.i, i64 noundef range(i64 8, 34359738361) %right.1, i64 noundef range(i64 8, 34359738361) %right.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_99a388bbc796e7fd7b28fa01c6ed4e6b) #23, !dbg !3874, !noalias !3719
  unreachable, !dbg !3874

bb34.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i
  %_108.i = sub nuw i64 %_165.1.i, %_43.i, !dbg !3875
  %_8.i2397 = icmp samesign ugt i64 %_108.i, 7, !dbg !3876
  br i1 %_8.i2397, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2400, label %bb2.i2398, !dbg !3876, !prof !2116

bb2.i2398:                                        ; preds = %bb34.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_108.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !3881, !noalias !3882
  unreachable, !dbg !3881

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2400: ; preds = %bb34.i
  %_165.0.i = load ptr, ptr %296, align 32, !dbg !3866, !alias.scope !3361, !noalias !3371, !nonnull !11, !noundef !11
  %_112.i = getelementptr inbounds nuw float, ptr %_165.0.i, i64 %_43.i, !dbg !3886
  store <8 x float> %lanes.i1820.sroa.0.0.copyload, ptr %_112.i, align 4, !dbg !3891, !alias.scope !3895, !noalias !3899
  %_166.1.i = load i64, ptr %298, align 8, !dbg !3901, !alias.scope !3361, !noalias !3371, !noundef !11
  %_113.i = icmp ugt i64 %_43.i, %_166.1.i, !dbg !3902
  br i1 %_113.i, label %bb35.i, label %bb36.i, !dbg !3902, !prof !161

bb33.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_43.i, i64 noundef %_165.1.i, i64 noundef %_165.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7fd19a98f6b7f6813fdc6b43fefd82a4) #23, !dbg !3906, !noalias !3719
  unreachable, !dbg !3906

bb36.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2400
  %_116.i = sub nuw i64 %_166.1.i, %_43.i, !dbg !3907
  %_8.i2392 = icmp samesign ugt i64 %_116.i, 7, !dbg !3908
  br i1 %_8.i2392, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2395, label %bb2.i2393, !dbg !3908, !prof !2116

bb2.i2393:                                        ; preds = %bb36.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_116.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !3913, !noalias !3914
  unreachable, !dbg !3913

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2395: ; preds = %bb36.i
  %_166.0.i = load ptr, ptr %299, align 16, !dbg !3901, !alias.scope !3361, !noalias !3371, !nonnull !11, !noundef !11
  %_120.i = getelementptr inbounds nuw float, ptr %_166.0.i, i64 %_43.i, !dbg !3918
  store <8 x float> %341, ptr %_120.i, align 4, !dbg !3923, !alias.scope !3927, !noalias !3931
  %_167.1.i = load i64, ptr %301, align 8, !dbg !3933, !alias.scope !3365, !noalias !3934, !noundef !11
  %_121.i = icmp ugt i64 %_43.i, %_167.1.i, !dbg !3935
  br i1 %_121.i, label %bb37.i, label %bb38.i, !dbg !3935, !prof !161

bb35.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2400
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_43.i, i64 noundef %_166.1.i, i64 noundef %_166.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_3898a5b9e77e549bf29e2ec14b43830a) #23, !dbg !3939, !noalias !3719
  unreachable, !dbg !3939

bb38.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2395
  %_124.i = sub nuw i64 %_167.1.i, %_43.i, !dbg !3940
  %_8.i2387 = icmp samesign ugt i64 %_124.i, 7, !dbg !3941
  br i1 %_8.i2387, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2390, label %bb2.i2388, !dbg !3941, !prof !2116

bb2.i2388:                                        ; preds = %bb38.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_124.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !3946, !noalias !3947
  unreachable, !dbg !3946

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2390: ; preds = %bb38.i
  %_167.0.i = load ptr, ptr %300, align 32, !dbg !3933, !alias.scope !3365, !noalias !3934, !nonnull !11, !noundef !11
  %_128.i = getelementptr inbounds nuw float, ptr %_167.0.i, i64 %_43.i, !dbg !3951
  store <8 x float> %lanes.i1811.sroa.0.0.copyload, ptr %_128.i, align 4, !dbg !3956, !alias.scope !3960, !noalias !3964
  %_168.1.i = load i64, ptr %302, align 8, !dbg !3966, !alias.scope !3365, !noalias !3934, !noundef !11
  %_129.i = icmp ugt i64 %_43.i, %_168.1.i, !dbg !3967
  br i1 %_129.i, label %bb39.i, label %bb40.i, !dbg !3967, !prof !161

bb37.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2395
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_43.i, i64 noundef %_167.1.i, i64 noundef %_167.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ef27b7b7ba2d23db7b66788c02ac8976) #23, !dbg !3971, !noalias !3719
  unreachable, !dbg !3971

bb40.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2390
  %_132.i = sub nuw i64 %_168.1.i, %_43.i, !dbg !3972
  %_8.i2382 = icmp samesign ugt i64 %_132.i, 7, !dbg !3973
  br i1 %_8.i2382, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2385, label %bb2.i2383, !dbg !3973, !prof !2116

bb2.i2383:                                        ; preds = %bb40.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_132.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !3978, !noalias !3979
  unreachable, !dbg !3978

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2385: ; preds = %bb40.i
  %_168.0.i = load ptr, ptr %303, align 16, !dbg !3966, !alias.scope !3365, !noalias !3934, !nonnull !11, !noundef !11
  %_136.i = getelementptr inbounds nuw float, ptr %_168.0.i, i64 %_43.i, !dbg !3983
  store <8 x float> %342, ptr %_136.i, align 4, !dbg !3988, !alias.scope !3992, !noalias !3996
  %_169.1.i = load i64, ptr %297, align 8, !dbg !3998, !alias.scope !3361, !noalias !3371, !noundef !11
  %_55.i = shl nuw nsw i64 %spec.store.select.i, 3, !dbg !3999
  %_137.i = icmp ugt i64 %_55.i, %_169.1.i, !dbg !4000
  br i1 %_137.i, label %bb41.i, label %bb42.i, !dbg !4000, !prof !161

bb39.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2390
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_43.i, i64 noundef %_168.1.i, i64 noundef %_168.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_23eb2a707ff31dba52171dada526a36f) #23, !dbg !4004, !noalias !3719
  unreachable, !dbg !4004

bb42.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2385
  %_140.i = sub nuw i64 %_169.1.i, %_55.i, !dbg !4005
  %_8.i1787 = icmp samesign ugt i64 %_140.i, 7, !dbg !4006
  br i1 %_8.i1787, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1791, label %bb2.i1788, !dbg !4006, !prof !2116

bb2.i1788:                                        ; preds = %bb42.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_140.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !4011, !noalias !4012
  unreachable, !dbg !4011

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1791: ; preds = %bb42.i
  %_169.0.i = load ptr, ptr %296, align 32, !dbg !3998, !alias.scope !3361, !noalias !3371, !nonnull !11, !noundef !11
  %_144.i = getelementptr inbounds nuw float, ptr %_169.0.i, i64 %_55.i, !dbg !4016
  %lanes.i1784.sroa.0.0.copyload = load <8 x float>, ptr %_144.i, align 4, !dbg !4021, !alias.scope !4025, !noalias !4029
  %_170.1.i = load i64, ptr %301, align 8, !dbg !4031, !alias.scope !3365, !noalias !3934, !noundef !11
  %_145.i = icmp ugt i64 %_55.i, %_170.1.i, !dbg !4033
  br i1 %_145.i, label %bb43.i, label %bb44.i, !dbg !4033, !prof !161

bb41.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2385
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_55.i, i64 noundef %_169.1.i, i64 noundef %_169.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e74b405e18c5e1dc98b750b7a797cfef) #23, !dbg !4037, !noalias !3719
  unreachable, !dbg !4037

bb44.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1791
  %_148.i = sub nuw i64 %_170.1.i, %_55.i, !dbg !4038
  %_8.i1778 = icmp samesign ugt i64 %_148.i, 7, !dbg !4039
  br i1 %_8.i1778, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1782, label %bb2.i1779, !dbg !4039, !prof !2116

bb2.i1779:                                        ; preds = %bb44.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_148.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !4044, !noalias !4045
  unreachable, !dbg !4044

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1782: ; preds = %bb44.i
  %_170.0.i = load ptr, ptr %300, align 32, !dbg !4031, !alias.scope !3365, !noalias !3934, !nonnull !11, !noundef !11
  %_152.i = getelementptr inbounds nuw float, ptr %_170.0.i, i64 %_55.i, !dbg !4049
  %lanes.i1775.sroa.0.0.copyload = load <8 x float>, ptr %_152.i, align 4, !dbg !4054, !alias.scope !4058, !noalias !4062
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4064), !dbg !4067
  %_5.i230 = load i32, ptr %277, align 4, !dbg !4069, !alias.scope !4064, !noalias !4071, !noundef !11
  %_38.1.i250 = load i64, ptr %298, align 8
  %_38.0.i254 = load ptr, ptr %299, align 16, !nonnull !11
  %_17.i242 = load i32, ptr %304, align 4, !dbg !4074, !alias.scope !4064, !noalias !4071, !noundef !11
  %delay.i243 = zext i32 %_17.i242 to i64, !dbg !4074
  %_20.not.i244 = icmp ult i32 %_37.i, %_17.i242, !dbg !4075
  %narrow3963 = select i1 %_20.not.i244, i32 %_5.i230, i32 0, !dbg !4075
  %_21.i245 = zext i32 %narrow3963 to i64, !dbg !4075
  %write.pn.i246 = sub nsw i64 %write.i, %delay.i243, !dbg !4075
  %tap.sroa.0.0.i247 = add nsw i64 %write.pn.i246, %_21.i245, !dbg !4076
  %_24.i248 = shl nsw i64 %tap.sroa.0.0.i247, 3, !dbg !4077
  %_26.i251 = icmp ult i64 %_24.i248, %_38.1.i250, !dbg !4078
  br i1 %_26.i251, label %bb10.i253, label %panic1.i252, !dbg !4078

bb10.i253:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1782
  %344 = getelementptr inbounds nuw float, ptr %_38.0.i254, i64 %_24.i248, !dbg !4078
  %_22.i255 = load float, ptr %344, align 4, !dbg !4078, !noalias !4079, !noundef !11
  %_17.i242.1 = load i32, ptr %317, align 4, !dbg !4074, !alias.scope !4064, !noalias !4071, !noundef !11
  %delay.i243.1 = zext i32 %_17.i242.1 to i64, !dbg !4074
  %_20.not.i244.1 = icmp ult i32 %_37.i, %_17.i242.1, !dbg !4075
  %narrow3963.1 = select i1 %_20.not.i244.1, i32 %_5.i230, i32 0, !dbg !4075
  %_21.i245.1 = zext i32 %narrow3963.1 to i64, !dbg !4075
  %write.pn.i246.1 = sub nsw i64 %write.i, %delay.i243.1, !dbg !4075
  %tap.sroa.0.0.i247.1 = add nsw i64 %write.pn.i246.1, %_21.i245.1, !dbg !4076
  %_24.i248.1 = shl nsw i64 %tap.sroa.0.0.i247.1, 3, !dbg !4077
  %_23.i249.1 = or disjoint i64 %_24.i248.1, 1, !dbg !4077
  %_26.i251.1 = icmp ult i64 %_23.i249.1, %_38.1.i250, !dbg !4078
  br i1 %_26.i251.1, label %bb10.i253.1, label %panic1.i252, !dbg !4078

bb10.i253.1:                                      ; preds = %bb10.i253
  %345 = getelementptr inbounds nuw float, ptr %_38.0.i254, i64 %_23.i249.1, !dbg !4078
  %_22.i255.1 = load float, ptr %345, align 4, !dbg !4078, !noalias !4079, !noundef !11
  %_17.i242.2 = load i32, ptr %318, align 4, !dbg !4074, !alias.scope !4064, !noalias !4071, !noundef !11
  %delay.i243.2 = zext i32 %_17.i242.2 to i64, !dbg !4074
  %_20.not.i244.2 = icmp ult i32 %_37.i, %_17.i242.2, !dbg !4075
  %narrow3963.2 = select i1 %_20.not.i244.2, i32 %_5.i230, i32 0, !dbg !4075
  %_21.i245.2 = zext i32 %narrow3963.2 to i64, !dbg !4075
  %write.pn.i246.2 = sub nsw i64 %write.i, %delay.i243.2, !dbg !4075
  %tap.sroa.0.0.i247.2 = add nsw i64 %write.pn.i246.2, %_21.i245.2, !dbg !4076
  %_24.i248.2 = shl nsw i64 %tap.sroa.0.0.i247.2, 3, !dbg !4077
  %_23.i249.2 = or disjoint i64 %_24.i248.2, 2, !dbg !4077
  %_26.i251.2 = icmp ult i64 %_23.i249.2, %_38.1.i250, !dbg !4078
  br i1 %_26.i251.2, label %bb10.i253.2, label %panic1.i252, !dbg !4078

bb10.i253.2:                                      ; preds = %bb10.i253.1
  %346 = getelementptr inbounds nuw float, ptr %_38.0.i254, i64 %_23.i249.2, !dbg !4078
  %_22.i255.2 = load float, ptr %346, align 4, !dbg !4078, !noalias !4079, !noundef !11
  %_17.i242.3 = load i32, ptr %319, align 4, !dbg !4074, !alias.scope !4064, !noalias !4071, !noundef !11
  %delay.i243.3 = zext i32 %_17.i242.3 to i64, !dbg !4074
  %_20.not.i244.3 = icmp ult i32 %_37.i, %_17.i242.3, !dbg !4075
  %narrow3963.3 = select i1 %_20.not.i244.3, i32 %_5.i230, i32 0, !dbg !4075
  %_21.i245.3 = zext i32 %narrow3963.3 to i64, !dbg !4075
  %write.pn.i246.3 = sub nsw i64 %write.i, %delay.i243.3, !dbg !4075
  %tap.sroa.0.0.i247.3 = add nsw i64 %write.pn.i246.3, %_21.i245.3, !dbg !4076
  %_24.i248.3 = shl nsw i64 %tap.sroa.0.0.i247.3, 3, !dbg !4077
  %_23.i249.3 = or disjoint i64 %_24.i248.3, 3, !dbg !4077
  %_26.i251.3 = icmp ult i64 %_23.i249.3, %_38.1.i250, !dbg !4078
  br i1 %_26.i251.3, label %bb10.i253.3, label %panic1.i252, !dbg !4078

bb10.i253.3:                                      ; preds = %bb10.i253.2
  %347 = getelementptr inbounds nuw float, ptr %_38.0.i254, i64 %_23.i249.3, !dbg !4078
  %_22.i255.3 = load float, ptr %347, align 4, !dbg !4078, !noalias !4079, !noundef !11
  %_17.i242.4 = load i32, ptr %320, align 4, !dbg !4074, !alias.scope !4064, !noalias !4071, !noundef !11
  %delay.i243.4 = zext i32 %_17.i242.4 to i64, !dbg !4074
  %_20.not.i244.4 = icmp ult i32 %_37.i, %_17.i242.4, !dbg !4075
  %narrow3963.4 = select i1 %_20.not.i244.4, i32 %_5.i230, i32 0, !dbg !4075
  %_21.i245.4 = zext i32 %narrow3963.4 to i64, !dbg !4075
  %write.pn.i246.4 = sub nsw i64 %write.i, %delay.i243.4, !dbg !4075
  %tap.sroa.0.0.i247.4 = add nsw i64 %write.pn.i246.4, %_21.i245.4, !dbg !4076
  %_24.i248.4 = shl nsw i64 %tap.sroa.0.0.i247.4, 3, !dbg !4077
  %_23.i249.4 = or disjoint i64 %_24.i248.4, 4, !dbg !4077
  %_26.i251.4 = icmp ult i64 %_23.i249.4, %_38.1.i250, !dbg !4078
  br i1 %_26.i251.4, label %bb10.i253.4, label %panic1.i252, !dbg !4078

bb10.i253.4:                                      ; preds = %bb10.i253.3
  %348 = getelementptr inbounds nuw float, ptr %_38.0.i254, i64 %_23.i249.4, !dbg !4078
  %_22.i255.4 = load float, ptr %348, align 4, !dbg !4078, !noalias !4079, !noundef !11
  %_17.i242.5 = load i32, ptr %321, align 4, !dbg !4074, !alias.scope !4064, !noalias !4071, !noundef !11
  %delay.i243.5 = zext i32 %_17.i242.5 to i64, !dbg !4074
  %_20.not.i244.5 = icmp ult i32 %_37.i, %_17.i242.5, !dbg !4075
  %narrow3963.5 = select i1 %_20.not.i244.5, i32 %_5.i230, i32 0, !dbg !4075
  %_21.i245.5 = zext i32 %narrow3963.5 to i64, !dbg !4075
  %write.pn.i246.5 = sub nsw i64 %write.i, %delay.i243.5, !dbg !4075
  %tap.sroa.0.0.i247.5 = add nsw i64 %write.pn.i246.5, %_21.i245.5, !dbg !4076
  %_24.i248.5 = shl nsw i64 %tap.sroa.0.0.i247.5, 3, !dbg !4077
  %_23.i249.5 = or disjoint i64 %_24.i248.5, 5, !dbg !4077
  %_26.i251.5 = icmp ult i64 %_23.i249.5, %_38.1.i250, !dbg !4078
  br i1 %_26.i251.5, label %bb10.i253.5, label %panic1.i252, !dbg !4078

bb10.i253.5:                                      ; preds = %bb10.i253.4
  %349 = getelementptr inbounds nuw float, ptr %_38.0.i254, i64 %_23.i249.5, !dbg !4078
  %_22.i255.5 = load float, ptr %349, align 4, !dbg !4078, !noalias !4079, !noundef !11
  %_17.i242.6 = load i32, ptr %322, align 4, !dbg !4074, !alias.scope !4064, !noalias !4071, !noundef !11
  %delay.i243.6 = zext i32 %_17.i242.6 to i64, !dbg !4074
  %_20.not.i244.6 = icmp ult i32 %_37.i, %_17.i242.6, !dbg !4075
  %narrow3963.6 = select i1 %_20.not.i244.6, i32 %_5.i230, i32 0, !dbg !4075
  %_21.i245.6 = zext i32 %narrow3963.6 to i64, !dbg !4075
  %write.pn.i246.6 = sub nsw i64 %write.i, %delay.i243.6, !dbg !4075
  %tap.sroa.0.0.i247.6 = add nsw i64 %write.pn.i246.6, %_21.i245.6, !dbg !4076
  %_24.i248.6 = shl nsw i64 %tap.sroa.0.0.i247.6, 3, !dbg !4077
  %_23.i249.6 = or disjoint i64 %_24.i248.6, 6, !dbg !4077
  %_26.i251.6 = icmp ult i64 %_23.i249.6, %_38.1.i250, !dbg !4078
  br i1 %_26.i251.6, label %bb6.i241.7, label %panic1.i252, !dbg !4078

bb6.i241.7:                                       ; preds = %bb10.i253.5
  %_17.i242.7 = load i32, ptr %323, align 4, !dbg !4074, !alias.scope !4064, !noalias !4071, !noundef !11
  %delay.i243.7 = zext i32 %_17.i242.7 to i64, !dbg !4074
  %_20.not.i244.7 = icmp ult i32 %_37.i, %_17.i242.7, !dbg !4075
  %narrow3963.7 = select i1 %_20.not.i244.7, i32 %_5.i230, i32 0, !dbg !4075
  %_21.i245.7 = zext i32 %narrow3963.7 to i64, !dbg !4075
  %write.pn.i246.7 = sub nsw i64 %write.i, %delay.i243.7, !dbg !4075
  %tap.sroa.0.0.i247.7 = add nsw i64 %write.pn.i246.7, %_21.i245.7, !dbg !4076
  %_24.i248.7 = shl nsw i64 %tap.sroa.0.0.i247.7, 3, !dbg !4077
  %_23.i249.7 = or disjoint i64 %_24.i248.7, 7, !dbg !4077
  %_26.i251.7 = icmp ult i64 %_23.i249.7, %_38.1.i250, !dbg !4078
  br i1 %_26.i251.7, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit256, label %panic1.i252, !dbg !4078

panic1.i252:                                      ; preds = %bb6.i241.7, %bb10.i253.5, %bb10.i253.4, %bb10.i253.3, %bb10.i253.2, %bb10.i253.1, %bb10.i253, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1782
  %_23.i249.lcssa = phi i64 [ %_24.i248, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1782 ], [ %_23.i249.1, %bb10.i253 ], [ %_23.i249.2, %bb10.i253.1 ], [ %_23.i249.3, %bb10.i253.2 ], [ %_23.i249.4, %bb10.i253.3 ], [ %_23.i249.5, %bb10.i253.4 ], [ %_23.i249.6, %bb10.i253.5 ], [ %_23.i249.7, %bb6.i241.7 ], !dbg !4077
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_23.i249.lcssa, i64 noundef %_38.1.i250, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a529a882edb544ae33c686afc6011081) #23, !dbg !4078, !noalias !4079
  unreachable, !dbg !4078

_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit256: ; preds = %bb6.i241.7
  %350 = getelementptr inbounds nuw float, ptr %_38.0.i254, i64 %_23.i249.6, !dbg !4078
  %_22.i255.6 = load float, ptr %350, align 4, !dbg !4078, !noalias !4079, !noundef !11
  %351 = getelementptr inbounds nuw float, ptr %_38.0.i254, i64 %_23.i249.7, !dbg !4078
  %_22.i255.7 = load float, ptr %351, align 4, !dbg !4078, !noalias !4079, !noundef !11
  %lanes.i1699.sroa.0.0.vec.insert = insertelement <8 x float> poison, float %_22.i255, i64 0, !dbg !4080
  %lanes.i1699.sroa.0.4.vec.insert = insertelement <8 x float> %lanes.i1699.sroa.0.0.vec.insert, float %_22.i255.1, i64 1, !dbg !4080
  %lanes.i1699.sroa.0.8.vec.insert = insertelement <8 x float> %lanes.i1699.sroa.0.4.vec.insert, float %_22.i255.2, i64 2, !dbg !4080
  %lanes.i1699.sroa.0.12.vec.insert = insertelement <8 x float> %lanes.i1699.sroa.0.8.vec.insert, float %_22.i255.3, i64 3, !dbg !4080
  %lanes.i1699.sroa.0.16.vec.insert = insertelement <8 x float> %lanes.i1699.sroa.0.12.vec.insert, float %_22.i255.4, i64 4, !dbg !4080
  %lanes.i1699.sroa.0.20.vec.insert = insertelement <8 x float> %lanes.i1699.sroa.0.16.vec.insert, float %_22.i255.5, i64 5, !dbg !4080
  %lanes.i1699.sroa.0.24.vec.insert = insertelement <8 x float> %lanes.i1699.sroa.0.20.vec.insert, float %_22.i255.6, i64 6, !dbg !4080
  %lanes.i1699.sroa.0.28.vec.insert = insertelement <8 x float> %lanes.i1699.sroa.0.24.vec.insert, float %_22.i255.7, i64 7, !dbg !4080
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4085), !dbg !4088
  %_5.i202 = load i32, ptr %305, align 4, !dbg !4090, !alias.scope !4085, !noalias !4092, !noundef !11
  %_38.1.i222 = load i64, ptr %302, align 8
  %_38.0.i226 = load ptr, ptr %303, align 16, !nonnull !11
  %_17.i214 = load i32, ptr %306, align 4, !dbg !4095, !alias.scope !4085, !noalias !4092, !noundef !11
  %delay.i215 = zext i32 %_17.i214 to i64, !dbg !4095
  %_20.not.i216 = icmp ult i32 %_37.i, %_17.i214, !dbg !4096
  %narrow3964 = select i1 %_20.not.i216, i32 %_5.i202, i32 0, !dbg !4096
  %_21.i217 = zext i32 %narrow3964 to i64, !dbg !4096
  %write.pn.i218 = sub nsw i64 %write.i, %delay.i215, !dbg !4096
  %tap.sroa.0.0.i219 = add nsw i64 %write.pn.i218, %_21.i217, !dbg !4097
  %_24.i220 = shl nsw i64 %tap.sroa.0.0.i219, 3, !dbg !4098
  %_26.i223 = icmp ult i64 %_24.i220, %_38.1.i222, !dbg !4099
  br i1 %_26.i223, label %bb10.i225, label %panic1.i224, !dbg !4099

bb10.i225:                                        ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit256
  %352 = getelementptr inbounds nuw float, ptr %_38.0.i226, i64 %_24.i220, !dbg !4099
  %_22.i227 = load float, ptr %352, align 4, !dbg !4099, !noalias !4100, !noundef !11
  %_17.i214.1 = load i32, ptr %324, align 4, !dbg !4095, !alias.scope !4085, !noalias !4092, !noundef !11
  %delay.i215.1 = zext i32 %_17.i214.1 to i64, !dbg !4095
  %_20.not.i216.1 = icmp ult i32 %_37.i, %_17.i214.1, !dbg !4096
  %narrow3964.1 = select i1 %_20.not.i216.1, i32 %_5.i202, i32 0, !dbg !4096
  %_21.i217.1 = zext i32 %narrow3964.1 to i64, !dbg !4096
  %write.pn.i218.1 = sub nsw i64 %write.i, %delay.i215.1, !dbg !4096
  %tap.sroa.0.0.i219.1 = add nsw i64 %write.pn.i218.1, %_21.i217.1, !dbg !4097
  %_24.i220.1 = shl nsw i64 %tap.sroa.0.0.i219.1, 3, !dbg !4098
  %_23.i221.1 = or disjoint i64 %_24.i220.1, 1, !dbg !4098
  %_26.i223.1 = icmp ult i64 %_23.i221.1, %_38.1.i222, !dbg !4099
  br i1 %_26.i223.1, label %bb10.i225.1, label %panic1.i224, !dbg !4099

bb10.i225.1:                                      ; preds = %bb10.i225
  %353 = getelementptr inbounds nuw float, ptr %_38.0.i226, i64 %_23.i221.1, !dbg !4099
  %_22.i227.1 = load float, ptr %353, align 4, !dbg !4099, !noalias !4100, !noundef !11
  %_17.i214.2 = load i32, ptr %325, align 4, !dbg !4095, !alias.scope !4085, !noalias !4092, !noundef !11
  %delay.i215.2 = zext i32 %_17.i214.2 to i64, !dbg !4095
  %_20.not.i216.2 = icmp ult i32 %_37.i, %_17.i214.2, !dbg !4096
  %narrow3964.2 = select i1 %_20.not.i216.2, i32 %_5.i202, i32 0, !dbg !4096
  %_21.i217.2 = zext i32 %narrow3964.2 to i64, !dbg !4096
  %write.pn.i218.2 = sub nsw i64 %write.i, %delay.i215.2, !dbg !4096
  %tap.sroa.0.0.i219.2 = add nsw i64 %write.pn.i218.2, %_21.i217.2, !dbg !4097
  %_24.i220.2 = shl nsw i64 %tap.sroa.0.0.i219.2, 3, !dbg !4098
  %_23.i221.2 = or disjoint i64 %_24.i220.2, 2, !dbg !4098
  %_26.i223.2 = icmp ult i64 %_23.i221.2, %_38.1.i222, !dbg !4099
  br i1 %_26.i223.2, label %bb10.i225.2, label %panic1.i224, !dbg !4099

bb10.i225.2:                                      ; preds = %bb10.i225.1
  %354 = getelementptr inbounds nuw float, ptr %_38.0.i226, i64 %_23.i221.2, !dbg !4099
  %_22.i227.2 = load float, ptr %354, align 4, !dbg !4099, !noalias !4100, !noundef !11
  %_17.i214.3 = load i32, ptr %326, align 4, !dbg !4095, !alias.scope !4085, !noalias !4092, !noundef !11
  %delay.i215.3 = zext i32 %_17.i214.3 to i64, !dbg !4095
  %_20.not.i216.3 = icmp ult i32 %_37.i, %_17.i214.3, !dbg !4096
  %narrow3964.3 = select i1 %_20.not.i216.3, i32 %_5.i202, i32 0, !dbg !4096
  %_21.i217.3 = zext i32 %narrow3964.3 to i64, !dbg !4096
  %write.pn.i218.3 = sub nsw i64 %write.i, %delay.i215.3, !dbg !4096
  %tap.sroa.0.0.i219.3 = add nsw i64 %write.pn.i218.3, %_21.i217.3, !dbg !4097
  %_24.i220.3 = shl nsw i64 %tap.sroa.0.0.i219.3, 3, !dbg !4098
  %_23.i221.3 = or disjoint i64 %_24.i220.3, 3, !dbg !4098
  %_26.i223.3 = icmp ult i64 %_23.i221.3, %_38.1.i222, !dbg !4099
  br i1 %_26.i223.3, label %bb10.i225.3, label %panic1.i224, !dbg !4099

bb10.i225.3:                                      ; preds = %bb10.i225.2
  %355 = getelementptr inbounds nuw float, ptr %_38.0.i226, i64 %_23.i221.3, !dbg !4099
  %_22.i227.3 = load float, ptr %355, align 4, !dbg !4099, !noalias !4100, !noundef !11
  %_17.i214.4 = load i32, ptr %327, align 4, !dbg !4095, !alias.scope !4085, !noalias !4092, !noundef !11
  %delay.i215.4 = zext i32 %_17.i214.4 to i64, !dbg !4095
  %_20.not.i216.4 = icmp ult i32 %_37.i, %_17.i214.4, !dbg !4096
  %narrow3964.4 = select i1 %_20.not.i216.4, i32 %_5.i202, i32 0, !dbg !4096
  %_21.i217.4 = zext i32 %narrow3964.4 to i64, !dbg !4096
  %write.pn.i218.4 = sub nsw i64 %write.i, %delay.i215.4, !dbg !4096
  %tap.sroa.0.0.i219.4 = add nsw i64 %write.pn.i218.4, %_21.i217.4, !dbg !4097
  %_24.i220.4 = shl nsw i64 %tap.sroa.0.0.i219.4, 3, !dbg !4098
  %_23.i221.4 = or disjoint i64 %_24.i220.4, 4, !dbg !4098
  %_26.i223.4 = icmp ult i64 %_23.i221.4, %_38.1.i222, !dbg !4099
  br i1 %_26.i223.4, label %bb10.i225.4, label %panic1.i224, !dbg !4099

bb10.i225.4:                                      ; preds = %bb10.i225.3
  %356 = getelementptr inbounds nuw float, ptr %_38.0.i226, i64 %_23.i221.4, !dbg !4099
  %_22.i227.4 = load float, ptr %356, align 4, !dbg !4099, !noalias !4100, !noundef !11
  %_17.i214.5 = load i32, ptr %328, align 4, !dbg !4095, !alias.scope !4085, !noalias !4092, !noundef !11
  %delay.i215.5 = zext i32 %_17.i214.5 to i64, !dbg !4095
  %_20.not.i216.5 = icmp ult i32 %_37.i, %_17.i214.5, !dbg !4096
  %narrow3964.5 = select i1 %_20.not.i216.5, i32 %_5.i202, i32 0, !dbg !4096
  %_21.i217.5 = zext i32 %narrow3964.5 to i64, !dbg !4096
  %write.pn.i218.5 = sub nsw i64 %write.i, %delay.i215.5, !dbg !4096
  %tap.sroa.0.0.i219.5 = add nsw i64 %write.pn.i218.5, %_21.i217.5, !dbg !4097
  %_24.i220.5 = shl nsw i64 %tap.sroa.0.0.i219.5, 3, !dbg !4098
  %_23.i221.5 = or disjoint i64 %_24.i220.5, 5, !dbg !4098
  %_26.i223.5 = icmp ult i64 %_23.i221.5, %_38.1.i222, !dbg !4099
  br i1 %_26.i223.5, label %bb10.i225.5, label %panic1.i224, !dbg !4099

bb10.i225.5:                                      ; preds = %bb10.i225.4
  %357 = getelementptr inbounds nuw float, ptr %_38.0.i226, i64 %_23.i221.5, !dbg !4099
  %_22.i227.5 = load float, ptr %357, align 4, !dbg !4099, !noalias !4100, !noundef !11
  %_17.i214.6 = load i32, ptr %329, align 4, !dbg !4095, !alias.scope !4085, !noalias !4092, !noundef !11
  %delay.i215.6 = zext i32 %_17.i214.6 to i64, !dbg !4095
  %_20.not.i216.6 = icmp ult i32 %_37.i, %_17.i214.6, !dbg !4096
  %narrow3964.6 = select i1 %_20.not.i216.6, i32 %_5.i202, i32 0, !dbg !4096
  %_21.i217.6 = zext i32 %narrow3964.6 to i64, !dbg !4096
  %write.pn.i218.6 = sub nsw i64 %write.i, %delay.i215.6, !dbg !4096
  %tap.sroa.0.0.i219.6 = add nsw i64 %write.pn.i218.6, %_21.i217.6, !dbg !4097
  %_24.i220.6 = shl nsw i64 %tap.sroa.0.0.i219.6, 3, !dbg !4098
  %_23.i221.6 = or disjoint i64 %_24.i220.6, 6, !dbg !4098
  %_26.i223.6 = icmp ult i64 %_23.i221.6, %_38.1.i222, !dbg !4099
  br i1 %_26.i223.6, label %bb6.i213.7, label %panic1.i224, !dbg !4099

bb6.i213.7:                                       ; preds = %bb10.i225.5
  %_17.i214.7 = load i32, ptr %330, align 4, !dbg !4095, !alias.scope !4085, !noalias !4092, !noundef !11
  %delay.i215.7 = zext i32 %_17.i214.7 to i64, !dbg !4095
  %_20.not.i216.7 = icmp ult i32 %_37.i, %_17.i214.7, !dbg !4096
  %narrow3964.7 = select i1 %_20.not.i216.7, i32 %_5.i202, i32 0, !dbg !4096
  %_21.i217.7 = zext i32 %narrow3964.7 to i64, !dbg !4096
  %write.pn.i218.7 = sub nsw i64 %write.i, %delay.i215.7, !dbg !4096
  %tap.sroa.0.0.i219.7 = add nsw i64 %write.pn.i218.7, %_21.i217.7, !dbg !4097
  %_24.i220.7 = shl nsw i64 %tap.sroa.0.0.i219.7, 3, !dbg !4098
  %_23.i221.7 = or disjoint i64 %_24.i220.7, 7, !dbg !4098
  %_26.i223.7 = icmp ult i64 %_23.i221.7, %_38.1.i222, !dbg !4099
  br i1 %_26.i223.7, label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit459, label %panic1.i224, !dbg !4099

panic1.i224:                                      ; preds = %bb6.i213.7, %bb10.i225.5, %bb10.i225.4, %bb10.i225.3, %bb10.i225.2, %bb10.i225.1, %bb10.i225, %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit256
  %_23.i221.lcssa = phi i64 [ %_24.i220, %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit256 ], [ %_23.i221.1, %bb10.i225 ], [ %_23.i221.2, %bb10.i225.1 ], [ %_23.i221.3, %bb10.i225.2 ], [ %_23.i221.4, %bb10.i225.3 ], [ %_23.i221.5, %bb10.i225.4 ], [ %_23.i221.6, %bb10.i225.5 ], [ %_23.i221.7, %bb6.i213.7 ], !dbg !4098
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_23.i221.lcssa, i64 noundef %_38.1.i222, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a529a882edb544ae33c686afc6011081) #23, !dbg !4099, !noalias !4100
  unreachable, !dbg !4099

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit459: ; preds = %bb6.i213.7
  %358 = getelementptr inbounds nuw float, ptr %_38.0.i226, i64 %_23.i221.6, !dbg !4099
  %_22.i227.6 = load float, ptr %358, align 4, !dbg !4099, !noalias !4100, !noundef !11
  %359 = getelementptr inbounds nuw float, ptr %_38.0.i226, i64 %_23.i221.7, !dbg !4099
  %_22.i227.7 = load float, ptr %359, align 4, !dbg !4099, !noalias !4100, !noundef !11
  %lanes.i1705.sroa.0.0.vec.insert = insertelement <8 x float> poison, float %_22.i227, i64 0, !dbg !4101
  %lanes.i1705.sroa.0.4.vec.insert = insertelement <8 x float> %lanes.i1705.sroa.0.0.vec.insert, float %_22.i227.1, i64 1, !dbg !4101
  %lanes.i1705.sroa.0.8.vec.insert = insertelement <8 x float> %lanes.i1705.sroa.0.4.vec.insert, float %_22.i227.2, i64 2, !dbg !4101
  %lanes.i1705.sroa.0.12.vec.insert = insertelement <8 x float> %lanes.i1705.sroa.0.8.vec.insert, float %_22.i227.3, i64 3, !dbg !4101
  %lanes.i1705.sroa.0.16.vec.insert = insertelement <8 x float> %lanes.i1705.sroa.0.12.vec.insert, float %_22.i227.4, i64 4, !dbg !4101
  %lanes.i1705.sroa.0.20.vec.insert = insertelement <8 x float> %lanes.i1705.sroa.0.16.vec.insert, float %_22.i227.5, i64 5, !dbg !4101
  %lanes.i1705.sroa.0.24.vec.insert = insertelement <8 x float> %lanes.i1705.sroa.0.20.vec.insert, float %_22.i227.6, i64 6, !dbg !4101
  %lanes.i1705.sroa.0.28.vec.insert = insertelement <8 x float> %lanes.i1705.sroa.0.24.vec.insert, float %_22.i227.7, i64 7, !dbg !4101
  %360 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i1699.sroa.0.28.vec.insert, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !4106
  %361 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %360, <8 x float> splat (float 0x3810000000000000)), !dbg !4115
  %362 = bitcast <8 x float> %361 to <4 x i64>, !dbg !4122
  %363 = and <4 x i64> %362, splat (i64 36028792732385279), !dbg !4123
  %364 = or disjoint <4 x i64> %363, splat (i64 4575657222473777152), !dbg !4128
  %365 = bitcast <4 x i64> %364 to <8 x float>, !dbg !4132
  %366 = fadd <8 x float> %365, splat (float -1.000000e+00), !dbg !4133
  %367 = fmul <8 x float> %366, splat (float 0xBF9B17A960000000), !dbg !4138
  %368 = fadd <8 x float> %367, splat (float 0x3FBF9A8440000000), !dbg !4143
  %369 = fmul <8 x float> %366, %368, !dbg !4138
  %370 = fadd <8 x float> %369, splat (float 0xBFD1E3F400000000), !dbg !4143
  %371 = fmul <8 x float> %366, %370, !dbg !4138
  %372 = fadd <8 x float> %371, splat (float 0x3FDD544F20000000), !dbg !4143
  %373 = fmul <8 x float> %366, %372, !dbg !4138
  %374 = fadd <8 x float> %373, splat (float 0xBFE6FC2A60000000), !dbg !4143
  %375 = fmul <8 x float> %366, %374, !dbg !4138
  %376 = fadd <8 x float> %375, splat (float 0x3FF714B2A0000000), !dbg !4143
  %377 = bitcast <8 x float> %361 to <8 x i32>, !dbg !4148
  %_3.i2618 = lshr <8 x i32> %377, splat (i32 23), !dbg !4152
  %378 = or disjoint <8 x i32> %_3.i2618, splat (i32 1258291200), !dbg !4153
  %379 = bitcast <8 x i32> %378 to <8 x float>, !dbg !4157
  %380 = fadd <8 x float> %379, splat (float 0xC160000FE0000000), !dbg !4158
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4162), !dbg !4165
  %_6.i365.sroa.0.0.copyload = load <8 x float>, ptr %_69.i, align 32, !dbg !4167, !noalias !4169
  %381 = fmul <8 x float> %366, %376, !dbg !4172
  %382 = fadd <8 x float> %380, %381, !dbg !4177
  %383 = fmul <8 x float> %382, splat (float 0x4018151820000000), !dbg !4182
  %384 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %383, <8 x float> splat (float -1.600000e+02)), !dbg !4187
  %385 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %384, <8 x float> splat (float 2.400000e+01)), !dbg !4192
  %386 = fsub <8 x float> %385, %lanes.i1441.sroa.0.0.copyload, !dbg !4197
  %387 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %386, <8 x float> %lanes.i1429.sroa.0.0.copyload, i8 30), !dbg !4203
  %388 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %386, <8 x float> %331, i8 18), !dbg !4209
  %389 = fadd <8 x float> %lanes.i1429.sroa.0.0.copyload, %386, !dbg !4215
  %390 = fmul <8 x float> %389, %389, !dbg !4220
  %391 = fmul <8 x float> %lanes.i1423.sroa.0.0.copyload, %390, !dbg !4225
  %392 = bitcast <8 x float> %387 to <8 x i32>, !dbg !4230
  %393 = icmp slt <8 x i32> %392, zeroinitializer, !dbg !4234
  %.v3969 = select <8 x i1> %393, <8 x float> %386, <8 x float> %391, !dbg !4234
  %394 = fmul <8 x float> %lanes.i1435.sroa.0.0.copyload, %.v3969, !dbg !4234
  %395 = bitcast <8 x float> %388 to <8 x i32>, !dbg !4236
  %396 = icmp slt <8 x i32> %395, zeroinitializer, !dbg !4240
  %397 = select <8 x i1> %396, <8 x float> zeroinitializer, <8 x float> %394, !dbg !4240
  %398 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %397, <8 x float> splat (float -1.000000e+02)), !dbg !4242
  %399 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %398, <8 x float> zeroinitializer), !dbg !4247
  %400 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %399, <8 x float> %_6.i365.sroa.0.0.copyload, i8 17), !dbg !4252
  %401 = bitcast <8 x float> %400 to <8 x i32>, !dbg !4258
  %402 = icmp slt <8 x i32> %401, zeroinitializer, !dbg !4262
  %403 = select <8 x i1> %402, <8 x float> %lanes.i1417.sroa.0.0.copyload, <8 x float> %lanes.i.sroa.0.0.copyload, !dbg !4262
  %404 = fsub <8 x float> %399, %_6.i365.sroa.0.0.copyload, !dbg !4264
  %405 = fmul <8 x float> %404, %403, !dbg !4270
  %406 = fadd <8 x float> %_6.i365.sroa.0.0.copyload, %405, !dbg !4275
  %407 = bitcast <8 x float> %406 to <8 x i32>, !dbg !4279
  %408 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %406), !dbg !4285
  %409 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %408, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !4287
  %410 = bitcast <8 x float> %409 to <8 x i32>, !dbg !4293
  %411 = xor <8 x i32> %410, splat (i32 -1), !dbg !4299
  %412 = and <8 x i32> %411, %407, !dbg !4301
  store <8 x i32> %412, ptr %_69.i, align 32, !dbg !4305, !alias.scope !4306, !noalias !4308
  %413 = bitcast <8 x i32> %412 to <8 x float>, !dbg !4309
  %414 = fadd <8 x float> %lanes.i1453.sroa.0.0.copyload, %413, !dbg !4310
  %415 = fmul <8 x float> %414, splat (float 0x3FC542A5A0000000), !dbg !4317
  %416 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %415, <8 x float> splat (float -1.260000e+02)), !dbg !4323
  %417 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %416, <8 x float> splat (float 1.270000e+02)), !dbg !4329
  %418 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %417), !dbg !4334
  %419 = fsub <8 x float> %417, %418, !dbg !4339
  %420 = fmul <8 x float> %419, splat (float 0x3F5E974FA0000000), !dbg !4344
  %421 = fadd <8 x float> %420, splat (float 0x3F82778560000000), !dbg !4349
  %422 = fmul <8 x float> %419, %421, !dbg !4344
  %423 = fadd <8 x float> %422, splat (float 0x3FAC91CE60000000), !dbg !4349
  %424 = fmul <8 x float> %419, %423, !dbg !4344
  %425 = fadd <8 x float> %424, splat (float 0x3FCEBDB560000000), !dbg !4349
  %426 = fmul <8 x float> %419, %425, !dbg !4344
  %427 = fadd <8 x float> %426, splat (float 0x3FE62E4BA0000000), !dbg !4349
  %428 = fmul <8 x float> %419, %427, !dbg !4354
  %429 = fadd <8 x float> %428, splat (float 1.000000e+00), !dbg !4359
  %430 = fadd <8 x float> %418, splat (float 0x4160000FE0000000), !dbg !4364
  %431 = bitcast <8 x float> %430 to <8 x i32>, !dbg !4369
  %_3.i2619 = shl <8 x i32> %431, splat (i32 23), !dbg !4373
  %432 = bitcast <8 x i32> %_3.i2619 to <8 x float>, !dbg !4374
  %433 = fmul <8 x float> %429, %432, !dbg !4376
  %434 = fmul <8 x float> %lanes.i1784.sroa.0.0.copyload, %433, !dbg !4380
  %435 = fsub <8 x float> %434, %lanes.i1784.sroa.0.0.copyload, !dbg !4385
  %436 = fmul <8 x float> %lanes.i1447.sroa.0.0.copyload, %435, !dbg !4391
  %437 = fadd <8 x float> %lanes.i1784.sroa.0.0.copyload, %436, !dbg !4396
  %438 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %413, <8 x float> zeroinitializer, i8 0), !dbg !4400
  %439 = bitcast <8 x float> %438 to <8 x i32>, !dbg !4406
  %440 = and <8 x i32> %439, %307, !dbg !4410
  %441 = or <8 x i32> %440, %308
  %.reass = or <8 x i32> %441, %309
  %442 = select <8 x i1> %311, <8 x float> %434, <8 x float> %437, !dbg !4412
  %443 = icmp slt <8 x i32> %.reass, zeroinitializer, !dbg !4417
  %444 = select <8 x i1> %443, <8 x float> %lanes.i1784.sroa.0.0.copyload, <8 x float> %442, !dbg !4417
  %445 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i1705.sroa.0.28.vec.insert, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !4422
  %446 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %445, <8 x float> splat (float 0x3810000000000000)), !dbg !4430
  %447 = bitcast <8 x float> %446 to <4 x i64>, !dbg !4437
  %448 = and <4 x i64> %447, splat (i64 36028792732385279), !dbg !4438
  %449 = or disjoint <4 x i64> %448, splat (i64 4575657222473777152), !dbg !4443
  %450 = bitcast <4 x i64> %449 to <8 x float>, !dbg !4447
  %451 = fadd <8 x float> %450, splat (float -1.000000e+00), !dbg !4448
  %452 = fmul <8 x float> %451, splat (float 0xBF9B17A960000000), !dbg !4453
  %453 = fadd <8 x float> %452, splat (float 0x3FBF9A8440000000), !dbg !4458
  %454 = fmul <8 x float> %451, %453, !dbg !4453
  %455 = fadd <8 x float> %454, splat (float 0xBFD1E3F400000000), !dbg !4458
  %456 = fmul <8 x float> %451, %455, !dbg !4453
  %457 = fadd <8 x float> %456, splat (float 0x3FDD544F20000000), !dbg !4458
  %458 = fmul <8 x float> %451, %457, !dbg !4453
  %459 = fadd <8 x float> %458, splat (float 0xBFE6FC2A60000000), !dbg !4458
  %460 = fmul <8 x float> %451, %459, !dbg !4453
  %461 = fadd <8 x float> %460, splat (float 0x3FF714B2A0000000), !dbg !4458
  %462 = bitcast <8 x float> %446 to <8 x i32>, !dbg !4463
  %_3.i2620 = lshr <8 x i32> %462, splat (i32 23), !dbg !4467
  %463 = or disjoint <8 x i32> %_3.i2620, splat (i32 1258291200), !dbg !4468
  %464 = bitcast <8 x i32> %463 to <8 x float>, !dbg !4472
  %465 = fadd <8 x float> %464, splat (float 0xC160000FE0000000), !dbg !4473
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4477), !dbg !4480
  %_6.i354.sroa.0.0.copyload = load <8 x float>, ptr %_73.i, align 32, !dbg !4482, !noalias !4484
  %466 = fmul <8 x float> %451, %461, !dbg !4487
  %467 = fadd <8 x float> %465, %466, !dbg !4492
  %468 = fmul <8 x float> %467, splat (float 0x4018151820000000), !dbg !4497
  %469 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %468, <8 x float> splat (float -1.600000e+02)), !dbg !4502
  %470 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %469, <8 x float> splat (float 2.400000e+01)), !dbg !4507
  %471 = fsub <8 x float> %470, %lanes.i1489.sroa.0.0.copyload, !dbg !4512
  %472 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %471, <8 x float> %lanes.i1477.sroa.0.0.copyload, i8 30), !dbg !4518
  %473 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %471, <8 x float> %332, i8 18), !dbg !4524
  %474 = fadd <8 x float> %lanes.i1477.sroa.0.0.copyload, %471, !dbg !4530
  %475 = fmul <8 x float> %474, %474, !dbg !4535
  %476 = fmul <8 x float> %lanes.i1471.sroa.0.0.copyload, %475, !dbg !4540
  %477 = bitcast <8 x float> %472 to <8 x i32>, !dbg !4545
  %478 = icmp slt <8 x i32> %477, zeroinitializer, !dbg !4549
  %.v3975 = select <8 x i1> %478, <8 x float> %471, <8 x float> %476, !dbg !4549
  %479 = fmul <8 x float> %lanes.i1483.sroa.0.0.copyload, %.v3975, !dbg !4549
  %480 = bitcast <8 x float> %473 to <8 x i32>, !dbg !4551
  %481 = icmp slt <8 x i32> %480, zeroinitializer, !dbg !4555
  %482 = select <8 x i1> %481, <8 x float> zeroinitializer, <8 x float> %479, !dbg !4555
  %483 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %482, <8 x float> splat (float -1.000000e+02)), !dbg !4557
  %484 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %483, <8 x float> zeroinitializer), !dbg !4562
  %485 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %484, <8 x float> %_6.i354.sroa.0.0.copyload, i8 17), !dbg !4567
  %486 = bitcast <8 x float> %485 to <8 x i32>, !dbg !4573
  %487 = icmp slt <8 x i32> %486, zeroinitializer, !dbg !4577
  %488 = select <8 x i1> %487, <8 x float> %lanes.i1465.sroa.0.0.copyload, <8 x float> %lanes.i1459.sroa.0.0.copyload, !dbg !4577
  %489 = fsub <8 x float> %484, %_6.i354.sroa.0.0.copyload, !dbg !4579
  %490 = fmul <8 x float> %489, %488, !dbg !4585
  %491 = fadd <8 x float> %_6.i354.sroa.0.0.copyload, %490, !dbg !4590
  %492 = bitcast <8 x float> %491 to <8 x i32>, !dbg !4594
  %493 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %491), !dbg !4600
  %494 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %493, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !4602
  %495 = bitcast <8 x float> %494 to <8 x i32>, !dbg !4608
  %496 = xor <8 x i32> %495, splat (i32 -1), !dbg !4614
  %497 = and <8 x i32> %496, %492, !dbg !4616
  store <8 x i32> %497, ptr %_73.i, align 32, !dbg !4620, !alias.scope !4621, !noalias !4623
  %498 = bitcast <8 x i32> %497 to <8 x float>, !dbg !4624
  %499 = fadd <8 x float> %lanes.i1501.sroa.0.0.copyload, %498, !dbg !4625
  %500 = fmul <8 x float> %499, splat (float 0x3FC542A5A0000000), !dbg !4632
  %501 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %500, <8 x float> splat (float -1.260000e+02)), !dbg !4638
  %502 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %501, <8 x float> splat (float 1.270000e+02)), !dbg !4644
  %503 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %502), !dbg !4649
  %504 = fsub <8 x float> %502, %503, !dbg !4654
  %505 = fmul <8 x float> %504, splat (float 0x3F5E974FA0000000), !dbg !4659
  %506 = fadd <8 x float> %505, splat (float 0x3F82778560000000), !dbg !4664
  %507 = fmul <8 x float> %504, %506, !dbg !4659
  %508 = fadd <8 x float> %507, splat (float 0x3FAC91CE60000000), !dbg !4664
  %509 = fmul <8 x float> %504, %508, !dbg !4659
  %510 = fadd <8 x float> %509, splat (float 0x3FCEBDB560000000), !dbg !4664
  %511 = fmul <8 x float> %504, %510, !dbg !4659
  %512 = fadd <8 x float> %511, splat (float 0x3FE62E4BA0000000), !dbg !4664
  %513 = fmul <8 x float> %504, %512, !dbg !4669
  %514 = fadd <8 x float> %513, splat (float 1.000000e+00), !dbg !4674
  %515 = fadd <8 x float> %503, splat (float 0x4160000FE0000000), !dbg !4679
  %516 = bitcast <8 x float> %515 to <8 x i32>, !dbg !4684
  %_3.i2621 = shl <8 x i32> %516, splat (i32 23), !dbg !4688
  %517 = bitcast <8 x i32> %_3.i2621 to <8 x float>, !dbg !4689
  %518 = fmul <8 x float> %514, %517, !dbg !4691
  %519 = fmul <8 x float> %lanes.i1775.sroa.0.0.copyload, %518, !dbg !4695
  %520 = fsub <8 x float> %519, %lanes.i1775.sroa.0.0.copyload, !dbg !4700
  %521 = fmul <8 x float> %lanes.i1495.sroa.0.0.copyload, %520, !dbg !4706
  %522 = fadd <8 x float> %lanes.i1775.sroa.0.0.copyload, %521, !dbg !4711
  %523 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %498, <8 x float> zeroinitializer, i8 0), !dbg !4715
  %524 = bitcast <8 x float> %523 to <8 x i32>, !dbg !4721
  %525 = and <8 x i32> %524, %312, !dbg !4725
  %526 = or <8 x i32> %525, %313
  %.reass6620 = or <8 x i32> %526, %309
  %527 = select <8 x i1> %315, <8 x float> %519, <8 x float> %522, !dbg !4727
  %528 = icmp slt <8 x i32> %.reass6620, zeroinitializer, !dbg !4732
  %529 = select <8 x i1> %528, <8 x float> %lanes.i1775.sroa.0.0.copyload, <8 x float> %527, !dbg !4732
  store <8 x float> %444, ptr %_96.i, align 4, !dbg !4737, !alias.scope !4743, !noalias !4747
  store <8 x float> %529, ptr %_104.i, align 4, !dbg !4751, !alias.scope !4756, !noalias !4760
  %530 = trunc i64 %spec.store.select.i to i32, !dbg !4764
  store i32 %530, ptr %295, align 32, !dbg !4764, !alias.scope !3361, !noalias !3371
  store i32 %530, ptr %316, align 32, !dbg !4765, !alias.scope !3365, !noalias !3934
  %exitcond5543.not = icmp eq i64 %333, %frames, !dbg !4766
  br i1 %exitcond5543.not, label %bb19, label %bb27.i, !dbg !3664

bb43.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1791
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_55.i, i64 noundef %_170.1.i, i64 noundef %_170.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_714be7237772529829a33eae50e95afa) #23, !dbg !4769, !noalias !3719
  unreachable, !dbg !4769

bb13:                                             ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel20segment_is_stageableNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit
  %_50.0 = load ptr, ptr %staged, align 8, !dbg !4770, !nonnull !11, !noundef !11
  %531 = getelementptr inbounds nuw i8, ptr %staged, i64 8, !dbg !4770
  %_50.1 = load i64, ptr %531, align 8, !dbg !4770, !noundef !11
  %532 = getelementptr inbounds nuw i8, ptr %staged, i64 16, !dbg !4772
  %_51.0 = load ptr, ptr %532, align 8, !dbg !4772, !nonnull !11, !noundef !11
  %533 = getelementptr inbounds nuw i8, ptr %staged, i64 24, !dbg !4772
  %_51.1 = load i64, ptr %533, align 8, !dbg !4772, !noundef !11
  %534 = getelementptr inbounds nuw i8, ptr %staged, i64 32, !dbg !4773
  %_52.0 = load ptr, ptr %534, align 8, !dbg !4773, !nonnull !11, !noundef !11
  %535 = getelementptr inbounds nuw i8, ptr %staged, i64 40, !dbg !4773
  %_52.1 = load i64, ptr %535, align 8, !dbg !4773, !noundef !11
  %536 = getelementptr inbounds nuw i8, ptr %staged, i64 48, !dbg !4774
  %_53.0 = load ptr, ptr %536, align 8, !dbg !4774, !nonnull !11, !noundef !11
  %537 = getelementptr inbounds nuw i8, ptr %staged, i64 56, !dbg !4774
  %_53.1 = load i64, ptr %537, align 8, !dbg !4774, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4775), !dbg !4778
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4779), !dbg !4778
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4781), !dbg !4778
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4783), !dbg !4778
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4785), !dbg !4778
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4787), !dbg !4778
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4789), !dbg !4778
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4791), !dbg !4778
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4793), !dbg !4778
  %538 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1540, !dbg !4795
  %_15.i2622 = load i32, ptr %538, align 4, !dbg !4795, !alias.scope !4783, !noalias !4799, !noundef !11
  %ring_length.i2623 = zext i32 %_15.i2622 to i64, !dbg !4795
  %539 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !4800
  %540 = bitcast <8 x float> %539 to <8 x i32>, !dbg !4809
  %541 = xor <8 x i32> %540, splat (i32 -1), !dbg !4815
  %542 = bitcast <8 x i32> %541 to <8 x float>, !dbg !4809
  switch i32 %link, label %bb13.i183.i [
    i32 1, label %bb14.i184.i
    i32 3, label %bb14.i184.fold.split.i
  ], !dbg !4817

bb13.i183.i:                                      ; preds = %bb13
  br label %bb14.i184.i, !dbg !4818

bb14.i184.fold.split.i:                           ; preds = %bb13
  br label %bb14.i184.i, !dbg !4819

bb14.i184.i:                                      ; preds = %bb14.i184.fold.split.i, %bb13.i183.i, %bb13
  %_11.i176.sroa.0.01708.i = phi <8 x float> [ %542, %bb13 ], [ %539, %bb13.i183.i ], [ %539, %bb14.i184.fold.split.i ]
  %_13.i175.sroa.0.0.i = phi <8 x i32> [ %541, %bb13 ], [ %541, %bb13.i183.i ], [ %540, %bb14.i184.fold.split.i ], !dbg !4820
  %_4.i217.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 448, !dbg !4821
  %lanes.i588.sroa.0.0.copyload.i = load <8 x float>, ptr %_4.i217.i, align 32, !dbg !4824, !alias.scope !4829, !noalias !4833
  %_7.i218.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 480, !dbg !4839
  %lanes.i583.sroa.0.0.copyload.i = load <8 x float>, ptr %_7.i218.i, align 32, !dbg !4840, !alias.scope !4845, !noalias !4849
  %_19.i2624 = getelementptr inbounds nuw i8, ptr %channels.0, i64 256, !dbg !4853
  %lanes.i578.sroa.0.0.copyload.i = load <8 x float>, ptr %_19.i2624, align 32, !dbg !4854, !alias.scope !4859, !noalias !4863
  %_14.i219.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 288, !dbg !4867
  %lanes.i573.sroa.0.0.copyload.i = load <8 x float>, ptr %_14.i219.i, align 32, !dbg !4868, !alias.scope !4873, !noalias !4877
  %_17.i220.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 320, !dbg !4881
  %lanes.i568.sroa.0.0.copyload.i = load <8 x float>, ptr %_17.i220.i, align 32, !dbg !4882, !alias.scope !4887, !noalias !4891
  %_20.i221.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 352, !dbg !4895
  %lanes.i563.sroa.0.0.copyload.i = load <8 x float>, ptr %_20.i221.i, align 32, !dbg !4896, !alias.scope !4901, !noalias !4905
  %_23.i222.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 384, !dbg !4909
  %lanes.i558.sroa.0.0.copyload.i = load <8 x float>, ptr %_23.i222.i, align 32, !dbg !4910, !alias.scope !4915, !noalias !4919
  %_26.i223.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 416, !dbg !4923
  %lanes.i.sroa.0.0.copyload.i = load <8 x float>, ptr %_26.i223.i, align 32, !dbg !4924, !alias.scope !4929, !noalias !4933
  %_4.i196.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 448, !dbg !4937
  %lanes.i628.sroa.0.0.copyload.i = load <8 x float>, ptr %_4.i196.i, align 32, !dbg !4940, !alias.scope !4945, !noalias !4949
  %_7.i197.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 480, !dbg !4955
  %lanes.i623.sroa.0.0.copyload.i = load <8 x float>, ptr %_7.i197.i, align 32, !dbg !4956, !alias.scope !4961, !noalias !4965
  %_21.i2625 = getelementptr inbounds nuw i8, ptr %channels.1, i64 256, !dbg !4969
  %lanes.i618.sroa.0.0.copyload.i = load <8 x float>, ptr %_21.i2625, align 32, !dbg !4970, !alias.scope !4975, !noalias !4979
  %_14.i198.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 288, !dbg !4983
  %lanes.i613.sroa.0.0.copyload.i = load <8 x float>, ptr %_14.i198.i, align 32, !dbg !4984, !alias.scope !4989, !noalias !4993
  %_17.i199.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 320, !dbg !4997
  %lanes.i608.sroa.0.0.copyload.i = load <8 x float>, ptr %_17.i199.i, align 32, !dbg !4998, !alias.scope !5003, !noalias !5007
  %_20.i200.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 352, !dbg !5011
  %lanes.i603.sroa.0.0.copyload.i = load <8 x float>, ptr %_20.i200.i, align 32, !dbg !5012, !alias.scope !5017, !noalias !5021
  %_23.i201.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 384, !dbg !5025
  %lanes.i598.sroa.0.0.copyload.i = load <8 x float>, ptr %_23.i201.i, align 32, !dbg !5026, !alias.scope !5031, !noalias !5035
  %_26.i.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 416, !dbg !5039
  %lanes.i593.sroa.0.0.copyload.i = load <8 x float>, ptr %_26.i.i, align 32, !dbg !5040, !alias.scope !5045, !noalias !5049
  %543 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i583.sroa.0.0.copyload.i, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !5053
  %544 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i583.sroa.0.0.copyload.i, <8 x float> zeroinitializer, i8 0), !dbg !5059
  %545 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i588.sroa.0.0.copyload.i, <8 x float> zeroinitializer, i8 0), !dbg !5065
  %546 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i623.sroa.0.0.copyload.i, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !5071
  %547 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i623.sroa.0.0.copyload.i, <8 x float> zeroinitializer, i8 0), !dbg !5077
  %548 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i628.sroa.0.0.copyload.i, <8 x float> zeroinitializer, i8 0), !dbg !5083
  %549 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1536, !dbg !5089
  %_23.i2626 = load i32, ptr %549, align 32, !dbg !5089, !alias.scope !4783, !noalias !4799, !noundef !11
  %550 = zext i32 %_23.i2626 to i64, !dbg !5089
; call compressor::kernel::fill_taps::<wide::f32x8_::f32x8>
  tail call fastcc void @_RINvNtCse3bfmKSZS8Y_10compressor6kernel9fill_tapsNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_(ptr noalias noundef nonnull readonly align 32 captures(address, read_provenance) dereferenceable(1568) %channels.0, i64 noundef %550, i64 noundef %_34, ptr noalias noundef nonnull align 4 %_50.0, i64 noundef range(i64 0, 2305843009213693952) %_50.1) #22, !dbg !5091, !noalias !5093
; call compressor::kernel::fill_taps::<wide::f32x8_::f32x8>
  tail call fastcc void @_RINvNtCse3bfmKSZS8Y_10compressor6kernel9fill_tapsNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_(ptr noalias noundef nonnull readonly align 32 captures(address, read_provenance) dereferenceable(1568) %channels.1, i64 noundef %550, i64 noundef %_34, ptr noalias noundef nonnull align 4 %_51.0, i64 noundef range(i64 0, 2305843009213693952) %_51.1) #22, !dbg !5094, !noalias !5095
  %_34.i2627 = shl nuw nsw i64 %_34, 3, !dbg !5096
  %_161.not.i = icmp ugt i64 %_34.i2627, %_50.1
  br i1 %_161.not.i, label %bb53.i, label %bb51.i, !dbg !5098, !prof !239

bb53.i:                                           ; preds = %bb14.i184.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_34.i2627, i64 noundef range(i64 0, 2305843009213693952) %_50.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bf658cb40e4f7564fec06c1245a38136) #23, !dbg !5107, !noalias !5108
  unreachable, !dbg !5107

bb51.i:                                           ; preds = %bb14.i184.i
  %_169.not.i = icmp samesign ugt i64 %_34.i2627, %_51.1, !dbg !5109
  br i1 %_169.not.i, label %bb56.i, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipB3_ECse3bfmKSZS8Y_10compressor.exit.i, !dbg !5109, !prof !161

bb56.i:                                           ; preds = %bb51.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_34.i2627, i64 noundef range(i64 0, 2305843009213693952) %_51.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7acc31d82704c7720eea4dde146ff5a0) #23, !dbg !5115, !noalias !5108
  unreachable, !dbg !5115

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipB3_ECse3bfmKSZS8Y_10compressor.exit.i: ; preds = %bb51.i
  %_176.not.i = icmp ugt i64 %_34, %_52.1
  br i1 %_176.not.i, label %bb58.i, label %bb57.i, !dbg !5116, !prof !239

bb58.i:                                           ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipB3_ECse3bfmKSZS8Y_10compressor.exit.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_34, i64 noundef range(i64 0, 288230376151711744) %_52.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a3aefd714e95b8f966811cbb71c5f9b5) #23, !dbg !5124, !noalias !5108
  unreachable, !dbg !5124

bb57.i:                                           ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipB3_ECse3bfmKSZS8Y_10compressor.exit.i
  %_191.not.i = icmp samesign ugt i64 %_34, %_53.1, !dbg !5125
  br i1 %_191.not.i, label %bb64.i, label %bb14.lr.ph.i, !dbg !5125, !prof !161

bb64.i:                                           ; preds = %bb57.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_34, i64 noundef range(i64 0, 288230376151711744) %_53.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_39c6596be70c3202efa3964a6ea265b7) #23, !dbg !5131, !noalias !5108
  unreachable, !dbg !5131

bb14.lr.ph.i:                                     ; preds = %bb57.i
  %_6.i.i2628 = load i64, ptr %detector, align 8, !range !220, !alias.scope !4781, !noalias !5132
  %551 = getelementptr inbounds nuw i8, ptr %detector, i64 16
  %sidechain_left.1.i.i2629 = load i64, ptr %551, align 8, !alias.scope !4781, !noalias !5132
  %552 = getelementptr inbounds nuw i8, ptr %detector, i64 24
  %sidechain_right.0.i.i2630 = load ptr, ptr %552, align 8, !alias.scope !4781, !noalias !5132, !nonnull !11, !align !3663
  %553 = getelementptr inbounds nuw i8, ptr %detector, i64 32
  %sidechain_right.1.i.i2631 = load i64, ptr %553, align 8, !alias.scope !4781, !noalias !5132
  %554 = getelementptr inbounds nuw i8, ptr %detector, i64 8
  %sidechain_left.0.i.i2632 = load ptr, ptr %554, align 8, !alias.scope !4781, !noalias !5132, !nonnull !11, !align !3663
  %555 = icmp slt <8 x i32> %_13.i175.sroa.0.0.i, zeroinitializer
  %556 = bitcast <8 x float> %_11.i176.sroa.0.01708.i to <8 x i32>
  %557 = icmp slt <8 x i32> %556, zeroinitializer
  %558 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1512
  %559 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1504
  %560 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1528
  %561 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1520
  %562 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1512
  %563 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1504
  %564 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1528
  %565 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1520
  %_335.1.i = load i64, ptr %558, align 8, !alias.scope !4783, !noalias !4799
  %_335.0.i = load ptr, ptr %559, align 32, !alias.scope !4783, !noalias !4799, !nonnull !11
  %_336.1.i = load i64, ptr %560, align 8, !alias.scope !4783, !noalias !4799
  %_336.0.i = load ptr, ptr %561, align 16, !alias.scope !4783, !noalias !4799, !nonnull !11
  %_337.1.i = load i64, ptr %562, align 8, !alias.scope !4785, !noalias !5133
  %_337.0.i = load ptr, ptr %563, align 32, !alias.scope !4785, !noalias !5133, !nonnull !11
  %_338.1.i = load i64, ptr %564, align 8, !alias.scope !4785, !noalias !5133
  %_338.0.i = load ptr, ptr %565, align 16, !alias.scope !4785, !noalias !5133, !nonnull !11
  %566 = fneg <8 x float> %lanes.i568.sroa.0.0.copyload.i
  %567 = fneg <8 x float> %lanes.i608.sroa.0.0.copyload.i
  br label %bb14.i2633, !dbg !5134

bb14.i2633:                                       ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_log2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit174.i, %bb14.lr.ph.i
  %head.sroa.0.01959.i = phi i64 [ %550, %bb14.lr.ph.i ], [ %spec.store.select.i2644, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_log2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit174.i ]
  %iter.sroa.33.01958.i = phi i64 [ 0, %bb14.lr.ph.i ], [ %_9.0.i.i, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_log2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit174.i ]
  %_9.0.i.i = add nuw nsw i64 %iter.sroa.33.01958.i, 1, !dbg !5142
  %start1.i.i.i.i.i.i.i.i.i = shl i64 %iter.sroa.33.01958.i, 3, !dbg !5145
  %data.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_50.0, i64 %start1.i.i.i.i.i.i.i.i.i, !dbg !5160
  %data.i5.i.i.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_51.0, i64 %start1.i.i.i.i.i.i.i.i.i, !dbg !5164
  %_3.i.i.i.i.i.i.i = getelementptr inbounds nuw %"wide::f32x8_::f32x8", ptr %_52.0, i64 %iter.sroa.33.01958.i, !dbg !5167
  %_3.i.i.i.i.i = getelementptr inbounds nuw %"wide::f32x8_::f32x8", ptr %_53.0, i64 %iter.sroa.33.01958.i, !dbg !5172
  %_51.i = add nuw nsw i64 %iter.sroa.33.01958.i, %spec.store.select, !dbg !5175
  %slot7.i = shl i64 %_51.i, 3, !dbg !5175
  %_205.i = icmp samesign ugt i64 %slot7.i, %left.1, !dbg !5177
  br i1 %_205.i, label %bb68.i, label %bb69.i, !dbg !5177, !prof !161

bb15.i:                                           ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_log2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit174.i
  %568 = trunc i64 %spec.store.select.i2644 to i32, !dbg !5184
  store i32 %568, ptr %549, align 32, !dbg !5184, !alias.scope !4783, !noalias !4799
  %569 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1536, !dbg !5185
  store i32 %568, ptr %569, align 32, !dbg !5185, !alias.scope !4785, !noalias !5133
  %570 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1472, !dbg !5186
  %gain_left.sroa.0.0.copyload.i = load <8 x float>, ptr %570, align 32, !dbg !5186, !alias.scope !4783, !noalias !4799
  %571 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1472, !dbg !5187
  %gain_right.sroa.0.0.copyload.i = load <8 x float>, ptr %571, align 32, !dbg !5187, !alias.scope !4785, !noalias !5133
  br label %bb36.i2645, !dbg !5189

bb36.i2645:                                       ; preds = %bb15.i, %bb36.i2645
  %gain_right.sroa.0.01963.i = phi <8 x float> [ %600, %bb36.i2645 ], [ %gain_right.sroa.0.0.copyload.i, %bb15.i ]
  %gain_left.sroa.0.01962.i = phi <8 x float> [ %586, %bb36.i2645 ], [ %gain_left.sroa.0.0.copyload.i, %bb15.i ]
  %iter2.sroa.8.01961.i = phi i64 [ %585, %bb36.i2645 ], [ 0, %bb15.i ]
  %_3.i.i.i = getelementptr inbounds nuw %"wide::f32x8_::f32x8", ptr %_52.0, i64 %iter2.sroa.8.01961.i, !dbg !5197
  %_117.sroa.0.0.copyload.i = load <8 x float>, ptr %_3.i.i.i, align 32, !dbg !5201, !alias.scope !4791, !noalias !5203
  %_3.i1.i.i = getelementptr inbounds nuw %"wide::f32x8_::f32x8", ptr %_53.0, i64 %iter2.sroa.8.01961.i, !dbg !5204
  %572 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_117.sroa.0.0.copyload.i, <8 x float> %gain_left.sroa.0.01962.i, i8 17), !dbg !5207
  %573 = bitcast <8 x float> %572 to <8 x i32>, !dbg !5214
  %574 = icmp slt <8 x i32> %573, zeroinitializer, !dbg !5218
  %575 = select <8 x i1> %574, <8 x float> %lanes.i558.sroa.0.0.copyload.i, <8 x float> %lanes.i.sroa.0.0.copyload.i, !dbg !5218
  %576 = fsub <8 x float> %_117.sroa.0.0.copyload.i, %gain_left.sroa.0.01962.i, !dbg !5220
  %577 = fmul <8 x float> %576, %575, !dbg !5226
  %578 = fadd <8 x float> %gain_left.sroa.0.01962.i, %577, !dbg !5231
  %579 = bitcast <8 x float> %578 to <8 x i32>, !dbg !5235
  %580 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %578), !dbg !5241
  %581 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %580, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !5243
  %582 = bitcast <8 x float> %581 to <8 x i32>, !dbg !5249
  %583 = xor <8 x i32> %582, splat (i32 -1), !dbg !5255
  %584 = and <8 x i32> %579, %583, !dbg !5257
  store <8 x i32> %584, ptr %_3.i.i.i, align 32, !dbg !5261, !alias.scope !4791, !noalias !5203
  %_121.sroa.0.0.copyload.i = load <8 x float>, ptr %_3.i1.i.i, align 32, !dbg !5262, !alias.scope !4793, !noalias !5263
  %585 = add nuw i64 %iter2.sroa.8.01961.i, 1, !dbg !5264
  %586 = bitcast <8 x i32> %584 to <8 x float>, !dbg !5265
  %587 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_121.sroa.0.0.copyload.i, <8 x float> %gain_right.sroa.0.01963.i, i8 17), !dbg !5266
  %588 = bitcast <8 x float> %587 to <8 x i32>, !dbg !5273
  %589 = icmp slt <8 x i32> %588, zeroinitializer, !dbg !5277
  %590 = select <8 x i1> %589, <8 x float> %lanes.i598.sroa.0.0.copyload.i, <8 x float> %lanes.i593.sroa.0.0.copyload.i, !dbg !5277
  %591 = fsub <8 x float> %_121.sroa.0.0.copyload.i, %gain_right.sroa.0.01963.i, !dbg !5279
  %592 = fmul <8 x float> %591, %590, !dbg !5285
  %593 = fadd <8 x float> %gain_right.sroa.0.01963.i, %592, !dbg !5290
  %594 = bitcast <8 x float> %593 to <8 x i32>, !dbg !5294
  %595 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %593), !dbg !5300
  %596 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %595, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !5302
  %597 = bitcast <8 x float> %596 to <8 x i32>, !dbg !5308
  %598 = xor <8 x i32> %597, splat (i32 -1), !dbg !5314
  %599 = and <8 x i32> %594, %598, !dbg !5316
  %600 = bitcast <8 x i32> %599 to <8 x float>, !dbg !5320
  store <8 x i32> %599, ptr %_3.i1.i.i, align 32, !dbg !5321, !alias.scope !4793, !noalias !5263
  %exitcond2194.not.i = icmp eq i64 %585, %_34, !dbg !5189
  br i1 %exitcond2194.not.i, label %bb43.lr.ph.i, label %bb36.i2645, !dbg !5189

bb43.lr.ph.i:                                     ; preds = %bb36.i2645
  store <8 x i32> %584, ptr %570, align 32, !dbg !5322, !alias.scope !4783, !noalias !4799
  store <8 x i32> %599, ptr %571, align 32, !dbg !5323, !alias.scope !4785, !noalias !5133
  %601 = bitcast <8 x float> %545 to <8 x i32>
  %602 = bitcast <8 x float> %544 to <8 x i32>
  %603 = select i1 %bypass, <8 x i32> %540, <8 x i32> %541
  %604 = bitcast <8 x float> %543 to <8 x i32>
  %605 = icmp slt <8 x i32> %604, zeroinitializer
  %606 = bitcast <8 x float> %548 to <8 x i32>
  %607 = bitcast <8 x float> %547 to <8 x i32>
  %608 = bitcast <8 x float> %546 to <8 x i32>
  %609 = icmp slt <8 x i32> %608, zeroinitializer
  br label %bb43.i2646, !dbg !5324

bb43.i2646:                                       ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit144.i, %bb43.lr.ph.i
  %iter3.sroa.13.01970.i = phi i64 [ 0, %bb43.lr.ph.i ], [ %_9.0.i1091.i, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit144.i ]
  %_9.0.i1091.i = add nuw nsw i64 %iter3.sroa.13.01970.i, 1, !dbg !5332
  %_3.i.i.i.i1087.i = getelementptr inbounds nuw %"wide::f32x8_::f32x8", ptr %_52.0, i64 %iter3.sroa.13.01970.i, !dbg !5335
  %_3.i1.i.i.i.i = getelementptr inbounds nuw %"wide::f32x8_::f32x8", ptr %_53.0, i64 %iter3.sroa.13.01970.i, !dbg !5341
  %_138.i = add nuw nsw i64 %iter3.sroa.13.01970.i, %spec.store.select, !dbg !5344
  %slot.i2647 = shl i64 %_138.i, 3, !dbg !5344
  %_311.i = icmp samesign ugt i64 %slot.i2647, %left.1, !dbg !5346
  br i1 %_311.i, label %bb94.i, label %bb95.i, !dbg !5346, !prof !161

bb95.i:                                           ; preds = %bb43.i2646
  %_314.i = sub nuw nsw i64 %left.1, %slot.i2647, !dbg !5351
  %_8.i707.i = icmp samesign ugt i64 %_314.i, 7, !dbg !5352
  br i1 %_8.i707.i, label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i, label %bb2.i708.i, !dbg !5352, !prof !2116

bb2.i708.i:                                       ; preds = %bb95.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_314.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !5357, !noalias !5358
  unreachable, !dbg !5357

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i: ; preds = %bb95.i
  %_318.i = getelementptr inbounds nuw float, ptr %left.0, i64 %slot.i2647, !dbg !5362
  %_143.sroa.0.0.copyload.i = load <8 x float>, ptr %_3.i.i.i.i1087.i, align 32, !dbg !5368, !alias.scope !4791, !noalias !5203
  %610 = fadd <8 x float> %lanes.i588.sroa.0.0.copyload.i, %_143.sroa.0.0.copyload.i, !dbg !5370
  %611 = fmul <8 x float> %610, splat (float 0x3FC542A5A0000000), !dbg !5376
  %612 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %611, <8 x float> splat (float -1.260000e+02)), !dbg !5382
  %613 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %612, <8 x float> splat (float 1.270000e+02)), !dbg !5388
  %614 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %613), !dbg !5393
  %615 = fsub <8 x float> %613, %614, !dbg !5398
  %616 = fmul <8 x float> %615, splat (float 0x3F5E974FA0000000), !dbg !5403
  %617 = fadd <8 x float> %616, splat (float 0x3F82778560000000), !dbg !5408
  %618 = fmul <8 x float> %615, %617, !dbg !5403
  %619 = fadd <8 x float> %618, splat (float 0x3FAC91CE60000000), !dbg !5408
  %620 = fmul <8 x float> %615, %619, !dbg !5403
  %621 = fadd <8 x float> %620, splat (float 0x3FCEBDB560000000), !dbg !5408
  %622 = fmul <8 x float> %615, %621, !dbg !5403
  %623 = fadd <8 x float> %622, splat (float 0x3FE62E4BA0000000), !dbg !5408
  %lanes.i704.sroa.0.0.copyload.i = load <8 x float>, ptr %_318.i, align 4, !dbg !5413, !alias.scope !5417, !noalias !5421
  %624 = fmul <8 x float> %615, %623, !dbg !5423
  %625 = fadd <8 x float> %624, splat (float 1.000000e+00), !dbg !5428
  %626 = fadd <8 x float> %614, splat (float 0x4160000FE0000000), !dbg !5433
  %627 = bitcast <8 x float> %626 to <8 x i32>, !dbg !5438
  %_3.i1092.i = shl <8 x i32> %627, splat (i32 23), !dbg !5442
  %628 = bitcast <8 x i32> %_3.i1092.i to <8 x float>, !dbg !5443
  %629 = fmul <8 x float> %625, %628, !dbg !5445
  %630 = fmul <8 x float> %lanes.i704.sroa.0.0.copyload.i, %629, !dbg !5449
  %631 = fsub <8 x float> %630, %lanes.i704.sroa.0.0.copyload.i, !dbg !5454
  %632 = fmul <8 x float> %lanes.i583.sroa.0.0.copyload.i, %631, !dbg !5460
  %633 = fadd <8 x float> %lanes.i704.sroa.0.0.copyload.i, %632, !dbg !5465
  %634 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_143.sroa.0.0.copyload.i, <8 x float> zeroinitializer, i8 0), !dbg !5469
  %635 = bitcast <8 x float> %634 to <8 x i32>, !dbg !5475
  %636 = and <8 x i32> %635, %601, !dbg !5479
  %637 = or <8 x i32> %636, %602
  %.reass.i.reass = or <8 x i32> %637, %603
  %638 = select <8 x i1> %605, <8 x float> %630, <8 x float> %633, !dbg !5481
  %639 = icmp slt <8 x i32> %.reass.i.reass, zeroinitializer, !dbg !5486
  %640 = select <8 x i1> %639, <8 x float> %lanes.i704.sroa.0.0.copyload.i, <8 x float> %638, !dbg !5486
  store <8 x float> %640, ptr %_318.i, align 4, !dbg !5491, !alias.scope !5496, !noalias !5500
  %_323.i = icmp samesign ugt i64 %slot.i2647, %right.1, !dbg !5504
  br i1 %_323.i, label %bb96.i, label %bb97.i, !dbg !5504, !prof !161

bb94.i:                                           ; preds = %bb43.i2646
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %slot.i2647, i64 noundef range(i64 8, 34359738361) %left.1, i64 noundef range(i64 8, 34359738361) %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c026f29aed5fe4d7cd9557b4f3b74850) #23, !dbg !5508, !noalias !5108
  unreachable, !dbg !5508

bb97.i:                                           ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i
  %_326.i = sub nuw nsw i64 %right.1, %slot.i2647, !dbg !5509
  %_8.i699.i = icmp samesign ugt i64 %_326.i, 7, !dbg !5510
  br i1 %_8.i699.i, label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit144.i, label %bb2.i700.i, !dbg !5510, !prof !2116

bb2.i700.i:                                       ; preds = %bb97.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_326.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !5515, !noalias !5516
  unreachable, !dbg !5515

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit144.i: ; preds = %bb97.i
  %_330.i = getelementptr inbounds nuw float, ptr %right.0, i64 %slot.i2647, !dbg !5520
  %_151.sroa.0.0.copyload.i = load <8 x float>, ptr %_3.i1.i.i.i.i, align 32, !dbg !5525, !alias.scope !4793, !noalias !5263
  %641 = fadd <8 x float> %lanes.i628.sroa.0.0.copyload.i, %_151.sroa.0.0.copyload.i, !dbg !5527
  %642 = fmul <8 x float> %641, splat (float 0x3FC542A5A0000000), !dbg !5533
  %643 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %642, <8 x float> splat (float -1.260000e+02)), !dbg !5539
  %644 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %643, <8 x float> splat (float 1.270000e+02)), !dbg !5545
  %645 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %644), !dbg !5550
  %646 = fsub <8 x float> %644, %645, !dbg !5555
  %647 = fmul <8 x float> %646, splat (float 0x3F5E974FA0000000), !dbg !5560
  %648 = fadd <8 x float> %647, splat (float 0x3F82778560000000), !dbg !5565
  %649 = fmul <8 x float> %646, %648, !dbg !5560
  %650 = fadd <8 x float> %649, splat (float 0x3FAC91CE60000000), !dbg !5565
  %651 = fmul <8 x float> %646, %650, !dbg !5560
  %652 = fadd <8 x float> %651, splat (float 0x3FCEBDB560000000), !dbg !5565
  %653 = fmul <8 x float> %646, %652, !dbg !5560
  %654 = fadd <8 x float> %653, splat (float 0x3FE62E4BA0000000), !dbg !5565
  %lanes.i696.sroa.0.0.copyload.i = load <8 x float>, ptr %_330.i, align 4, !dbg !5570, !alias.scope !5574, !noalias !5578
  %655 = fmul <8 x float> %646, %654, !dbg !5580
  %656 = fadd <8 x float> %655, splat (float 1.000000e+00), !dbg !5585
  %657 = fadd <8 x float> %645, splat (float 0x4160000FE0000000), !dbg !5590
  %658 = bitcast <8 x float> %657 to <8 x i32>, !dbg !5595
  %_3.i1093.i = shl <8 x i32> %658, splat (i32 23), !dbg !5599
  %659 = bitcast <8 x i32> %_3.i1093.i to <8 x float>, !dbg !5600
  %660 = fmul <8 x float> %656, %659, !dbg !5602
  %661 = fmul <8 x float> %lanes.i696.sroa.0.0.copyload.i, %660, !dbg !5606
  %662 = fsub <8 x float> %661, %lanes.i696.sroa.0.0.copyload.i, !dbg !5611
  %663 = fmul <8 x float> %lanes.i623.sroa.0.0.copyload.i, %662, !dbg !5617
  %664 = fadd <8 x float> %lanes.i696.sroa.0.0.copyload.i, %663, !dbg !5622
  %665 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_151.sroa.0.0.copyload.i, <8 x float> zeroinitializer, i8 0), !dbg !5626
  %666 = bitcast <8 x float> %665 to <8 x i32>, !dbg !5632
  %667 = and <8 x i32> %666, %606, !dbg !5636
  %668 = or <8 x i32> %667, %607
  %.reass2668.i.reass = or <8 x i32> %668, %603
  %669 = select <8 x i1> %609, <8 x float> %661, <8 x float> %664, !dbg !5638
  %670 = icmp slt <8 x i32> %.reass2668.i.reass, zeroinitializer, !dbg !5643
  %671 = select <8 x i1> %670, <8 x float> %lanes.i696.sroa.0.0.copyload.i, <8 x float> %669, !dbg !5643
  store <8 x float> %671, ptr %_330.i, align 4, !dbg !5648, !alias.scope !5653, !noalias !5657
  %exitcond2209.not.i = icmp eq i64 %_9.0.i1091.i, %_34, !dbg !5324
  br i1 %exitcond2209.not.i, label %bb19, label %bb43.i2646, !dbg !5324

bb96.i:                                           ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %slot.i2647, i64 noundef range(i64 8, 34359738361) %right.1, i64 noundef range(i64 8, 34359738361) %right.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5af02affad714ed6461f8ae2fddf49da) #23, !dbg !5661, !noalias !5108
  unreachable, !dbg !5661

bb69.i:                                           ; preds = %bb14.i2633
  %_208.i = sub nuw nsw i64 %left.1, %slot7.i, !dbg !5662
  %_212.i = getelementptr inbounds nuw float, ptr %left.0, i64 %slot7.i, !dbg !5663
  %_8.i691.i = icmp samesign ugt i64 %_208.i, 7, !dbg !5667
  br i1 %_8.i691.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit694.i, label %bb2.i692.i, !dbg !5667, !prof !2116

bb2.i692.i:                                       ; preds = %bb69.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_208.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !5672, !noalias !5673
  unreachable, !dbg !5672

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit694.i: ; preds = %bb69.i
  %lanes.i688.sroa.0.0.copyload.i = load <8 x float>, ptr %_212.i, align 4, !dbg !5677, !alias.scope !5681, !noalias !5685
  %_216.i = icmp samesign ugt i64 %slot7.i, %right.1, !dbg !5687
  br i1 %_216.i, label %bb70.i, label %bb71.i, !dbg !5687, !prof !161

bb68.i:                                           ; preds = %bb14.i2633
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %slot7.i, i64 noundef range(i64 8, 34359738361) %left.1, i64 noundef range(i64 8, 34359738361) %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b2f50309aa881af863c4c5948e01a00d) #23, !dbg !5692, !noalias !5108
  unreachable, !dbg !5692

bb71.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit694.i
  %_219.i = sub nuw nsw i64 %right.1, %slot7.i, !dbg !5693
  %_223.i = getelementptr inbounds nuw float, ptr %right.0, i64 %slot7.i, !dbg !5694
  %_8.i683.i = icmp samesign ugt i64 %_219.i, 7, !dbg !5699
  br i1 %_8.i683.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit686.i, label %bb2.i684.i, !dbg !5699, !prof !2116

bb2.i684.i:                                       ; preds = %bb71.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_219.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !5704, !noalias !5705
  unreachable, !dbg !5704

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit686.i: ; preds = %bb71.i
  %lanes.i680.sroa.0.0.copyload.i = load <8 x float>, ptr %_223.i, align 4, !dbg !5709, !alias.scope !5713, !noalias !5717
  switch i64 %_6.i.i2628, label %default.unreachable [
    i64 0, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i2643
    i64 1, label %bb3.i.i2651
    i64 2, label %bb2.i.i2634
  ], !dbg !5719

bb3.i.i2651:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit686.i
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i2643, !dbg !5722

bb2.i.i2634:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit686.i
  %_25.i.i2635 = icmp ugt i64 %slot7.i, %sidechain_left.1.i.i2629, !dbg !5723
  br i1 %_25.i.i2635, label %bb17.i.i2650, label %bb18.i.i2636, !dbg !5723, !prof !161

bb18.i.i2636:                                     ; preds = %bb2.i.i2634
  %_28.i.i2637 = sub nuw i64 %sidechain_left.1.i.i2629, %slot7.i, !dbg !5726
  %_8.i643.i = icmp samesign ugt i64 %_28.i.i2637, 7, !dbg !5727
  br i1 %_8.i643.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit646.i, label %bb2.i644.i, !dbg !5727, !prof !2116

bb2.i644.i:                                       ; preds = %bb18.i.i2636
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_28.i.i2637, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !5732, !noalias !5733
  unreachable, !dbg !5732

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit646.i: ; preds = %bb18.i.i2636
  %_32.i.i2638 = getelementptr inbounds nuw float, ptr %sidechain_left.0.i.i2632, i64 %slot7.i, !dbg !5742
  %lanes.i640.sroa.0.0.copyload.i = load <8 x float>, ptr %_32.i.i2638, align 4, !dbg !5744, !alias.scope !5748, !noalias !5752
  %_33.i.i2639 = icmp ugt i64 %slot7.i, %sidechain_right.1.i.i2631, !dbg !5754
  br i1 %_33.i.i2639, label %bb19.i.i2649, label %bb20.i.i2640, !dbg !5754, !prof !161

bb17.i.i2650:                                     ; preds = %bb2.i.i2634
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 34359738361) %slot7.i, i64 noundef %sidechain_left.1.i.i2629, i64 noundef %sidechain_left.1.i.i2629, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a0a4023c361dd50869bbb08b76482f50) #23, !dbg !5757, !noalias !5758
  unreachable, !dbg !5757

bb20.i.i2640:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit646.i
  %_36.i.i2641 = sub nuw i64 %sidechain_right.1.i.i2631, %slot7.i, !dbg !5760
  %_8.i635.i = icmp samesign ugt i64 %_36.i.i2641, 7, !dbg !5761
  br i1 %_8.i635.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit638.i, label %bb2.i636.i, !dbg !5761, !prof !2116

bb2.i636.i:                                       ; preds = %bb20.i.i2640
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_36.i.i2641, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !5766, !noalias !5767
  unreachable, !dbg !5766

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit638.i: ; preds = %bb20.i.i2640
  %_40.i.i2642 = getelementptr inbounds nuw float, ptr %sidechain_right.0.i.i2630, i64 %slot7.i, !dbg !5771
  %lanes.i633.sroa.0.0.copyload.i = load <8 x float>, ptr %_40.i.i2642, align 4, !dbg !5773, !alias.scope !5777, !noalias !5781
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i2643, !dbg !5783

bb19.i.i2649:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit646.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 34359738361) %slot7.i, i64 noundef %sidechain_right.1.i.i2631, i64 noundef %sidechain_right.1.i.i2631, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5807a2a07e9839e3d32631b4243eb17f) #23, !dbg !5784, !noalias !5758
  unreachable, !dbg !5784

_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i2643: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit638.i, %bb3.i.i2651, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit686.i
  %.sroa.0.0.i = phi <8 x float> [ %lanes.i688.sroa.0.0.copyload.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit686.i ], [ zeroinitializer, %bb3.i.i2651 ], [ %lanes.i640.sroa.0.0.copyload.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit638.i ], !dbg !5785
  %.sroa.01123.0.i = phi <8 x float> [ %lanes.i680.sroa.0.0.copyload.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit686.i ], [ zeroinitializer, %bb3.i.i2651 ], [ %lanes.i633.sroa.0.0.copyload.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit638.i ], !dbg !5785
  %672 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %.sroa.0.0.i), !dbg !5786
  %673 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %.sroa.01123.0.i), !dbg !5792
  %674 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %672, <8 x float> %673), !dbg !5798
  %675 = fmul <8 x float> %672, splat (float 5.000000e-01), !dbg !5803
  %676 = fmul <8 x float> %673, splat (float 5.000000e-01), !dbg !5808
  %677 = fadd <8 x float> %675, %676, !dbg !5813
  %678 = select <8 x i1> %555, <8 x float> %677, <8 x float> %674, !dbg !5818
  %679 = select <8 x i1> %557, <8 x float> %678, <8 x float> %672, !dbg !5823
  %680 = select <8 x i1> %557, <8 x float> %678, <8 x float> %673, !dbg !5828
  %681 = add i64 %head.sroa.0.01959.i, 1, !dbg !5833
  %_62.i = icmp eq i64 %681, %ring_length.i2623, !dbg !5835
  %spec.store.select.i2644 = select i1 %_62.i, i64 0, i64 %681, !dbg !5835
  %_66.i = shl i64 %head.sroa.0.01959.i, 3, !dbg !5837
  %_224.i = icmp ugt i64 %_66.i, %_335.1.i, !dbg !5839
  br i1 %_224.i, label %bb72.i, label %bb73.i, !dbg !5839, !prof !161

bb70.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit694.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %slot7.i, i64 noundef range(i64 8, 34359738361) %right.1, i64 noundef range(i64 8, 34359738361) %right.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2b558c2a7d8c32f2ec911d93287e9a55) #23, !dbg !5845, !noalias !5108
  unreachable, !dbg !5845

bb73.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i2643
  %_227.i = sub nuw i64 %_335.1.i, %_66.i, !dbg !5846
  %_8.i963.i = icmp samesign ugt i64 %_227.i, 7, !dbg !5847
  br i1 %_8.i963.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit965.i, label %bb2.i964.i, !dbg !5847, !prof !2116

bb2.i964.i:                                       ; preds = %bb73.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_227.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !5852, !noalias !5853
  unreachable, !dbg !5852

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit965.i: ; preds = %bb73.i
  %_231.i = getelementptr inbounds nuw float, ptr %_335.0.i, i64 %_66.i, !dbg !5857
  store <8 x float> %lanes.i688.sroa.0.0.copyload.i, ptr %_231.i, align 4, !dbg !5862, !alias.scope !5866, !noalias !5870
  %_232.i = icmp ugt i64 %_66.i, %_336.1.i, !dbg !5872
  br i1 %_232.i, label %bb74.i, label %bb75.i, !dbg !5872, !prof !161

bb72.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i2643
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_66.i, i64 noundef %_335.1.i, i64 noundef %_335.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bdcb1b3b7b781c0308467c6e4a54ec4e) #23, !dbg !5876, !noalias !5108
  unreachable, !dbg !5876

bb75.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit965.i
  %_235.i = sub nuw i64 %_336.1.i, %_66.i, !dbg !5877
  %_8.i959.i = icmp samesign ugt i64 %_235.i, 7, !dbg !5878
  br i1 %_8.i959.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit961.i, label %bb2.i960.i, !dbg !5878, !prof !2116

bb2.i960.i:                                       ; preds = %bb75.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_235.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !5883, !noalias !5884
  unreachable, !dbg !5883

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit961.i: ; preds = %bb75.i
  %_239.i = getelementptr inbounds nuw float, ptr %_336.0.i, i64 %_66.i, !dbg !5888
  store <8 x float> %679, ptr %_239.i, align 4, !dbg !5893, !alias.scope !5897, !noalias !5901
  %_240.i = icmp ugt i64 %_66.i, %_337.1.i, !dbg !5903
  br i1 %_240.i, label %bb76.i, label %bb77.i, !dbg !5903, !prof !161

bb74.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit965.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_66.i, i64 noundef %_336.1.i, i64 noundef %_336.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_da6aedc6f0cd3ef0ecdf38514bf95c44) #23, !dbg !5907, !noalias !5108
  unreachable, !dbg !5907

bb77.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit961.i
  %_243.i = sub nuw i64 %_337.1.i, %_66.i, !dbg !5908
  %_8.i955.i = icmp samesign ugt i64 %_243.i, 7, !dbg !5909
  br i1 %_8.i955.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit957.i, label %bb2.i956.i, !dbg !5909, !prof !2116

bb2.i956.i:                                       ; preds = %bb77.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_243.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !5914, !noalias !5915
  unreachable, !dbg !5914

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit957.i: ; preds = %bb77.i
  %_247.i = getelementptr inbounds nuw float, ptr %_337.0.i, i64 %_66.i, !dbg !5919
  store <8 x float> %lanes.i680.sroa.0.0.copyload.i, ptr %_247.i, align 4, !dbg !5924, !alias.scope !5928, !noalias !5932
  %_248.i = icmp ugt i64 %_66.i, %_338.1.i, !dbg !5934
  br i1 %_248.i, label %bb78.i, label %bb79.i, !dbg !5934, !prof !161

bb76.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit961.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_66.i, i64 noundef %_337.1.i, i64 noundef %_337.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aca6e49a571217c31ca2447292225430) #23, !dbg !5938, !noalias !5108
  unreachable, !dbg !5938

bb79.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit957.i
  %_251.i = sub nuw i64 %_338.1.i, %_66.i, !dbg !5939
  %_8.i951.i = icmp samesign ugt i64 %_251.i, 7, !dbg !5940
  br i1 %_8.i951.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit953.i, label %bb2.i952.i, !dbg !5940, !prof !2116

bb2.i952.i:                                       ; preds = %bb79.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_251.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !5945, !noalias !5946
  unreachable, !dbg !5945

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit953.i: ; preds = %bb79.i
  %_255.i = getelementptr inbounds nuw float, ptr %_338.0.i, i64 %_66.i, !dbg !5950
  store <8 x float> %680, ptr %_255.i, align 4, !dbg !5955, !alias.scope !5959, !noalias !5963
  %_85.i = shl i64 %spec.store.select.i2644, 3, !dbg !5965
  %_256.i = icmp ugt i64 %_85.i, %_335.1.i, !dbg !5966
  br i1 %_256.i, label %bb80.i, label %bb81.i, !dbg !5966, !prof !161

bb78.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit957.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_66.i, i64 noundef %_338.1.i, i64 noundef %_338.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_537a50830147a702ee833cf38f119351) #23, !dbg !5970, !noalias !5108
  unreachable, !dbg !5970

bb81.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit953.i
  %_259.i = sub nuw i64 %_335.1.i, %_85.i, !dbg !5971
  %_8.i675.i = icmp samesign ugt i64 %_259.i, 7, !dbg !5972
  br i1 %_8.i675.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit949.i, label %bb2.i676.i, !dbg !5972, !prof !2116

bb2.i676.i:                                       ; preds = %bb81.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_259.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !5977, !noalias !5978
  unreachable, !dbg !5977

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit949.i: ; preds = %bb81.i
  %_263.i = getelementptr inbounds nuw float, ptr %_335.0.i, i64 %_85.i, !dbg !5982
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_212.i, ptr noundef nonnull align 4 dereferenceable(32) %_263.i, i64 32, i1 false), !dbg !5987, !noalias !5992
  %_268.i = icmp ugt i64 %_85.i, %_337.1.i, !dbg !5993
  br i1 %_268.i, label %bb82.i, label %bb83.i, !dbg !5993, !prof !161

bb80.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit953.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_85.i, i64 noundef %_335.1.i, i64 noundef %_335.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e878a2c67fc44f18741fecb8548c8ef7) #23, !dbg !5997, !noalias !5108
  unreachable, !dbg !5997

bb83.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit949.i
  %_271.i = sub nuw i64 %_337.1.i, %_85.i, !dbg !5998
  %_8.i667.i = icmp samesign ugt i64 %_271.i, 7, !dbg !5999
  br i1 %_8.i667.i, label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_log2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit174.i, label %bb2.i668.i, !dbg !5999, !prof !2116

bb2.i668.i:                                       ; preds = %bb83.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_271.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !6004, !noalias !6005
  unreachable, !dbg !6004

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_log2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit174.i: ; preds = %bb83.i
  %_275.i = getelementptr inbounds nuw float, ptr %_337.0.i, i64 %_85.i, !dbg !6009
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_223.i, ptr noundef nonnull align 4 dereferenceable(32) %_275.i, i64 32, i1 false), !dbg !6014, !noalias !6019
  %lanes.i656.sroa.0.0.copyload.i = load <8 x float>, ptr %data.i.i.i.i.i.i.i.i.i, align 4, !dbg !6020, !alias.scope !6025, !noalias !6029
  %682 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i656.sroa.0.0.copyload.i, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !6033
  %683 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %682, <8 x float> splat (float 0x3810000000000000)), !dbg !6039
  %684 = bitcast <8 x float> %683 to <4 x i64>, !dbg !6046
  %685 = and <4 x i64> %684, splat (i64 36028792732385279), !dbg !6047
  %686 = or disjoint <4 x i64> %685, splat (i64 4575657222473777152), !dbg !6052
  %687 = bitcast <4 x i64> %686 to <8 x float>, !dbg !6056
  %688 = fadd <8 x float> %687, splat (float -1.000000e+00), !dbg !6057
  %689 = fmul <8 x float> %688, splat (float 0x3F9B17A960000000), !dbg !6062
  %690 = fsub <8 x float> splat (float 0x3FBF9A8440000000), %689, !dbg !6067
  %691 = fmul <8 x float> %688, %690, !dbg !6062
  %692 = fadd <8 x float> %691, splat (float 0xBFD1E3F400000000), !dbg !6067
  %693 = fmul <8 x float> %688, %692, !dbg !6062
  %694 = fadd <8 x float> %693, splat (float 0x3FDD544F20000000), !dbg !6067
  %695 = fmul <8 x float> %688, %694, !dbg !6062
  %696 = fadd <8 x float> %695, splat (float 0xBFE6FC2A60000000), !dbg !6067
  %697 = fmul <8 x float> %688, %696, !dbg !6062
  %698 = fadd <8 x float> %697, splat (float 0x3FF714B2A0000000), !dbg !6067
  %699 = bitcast <8 x float> %683 to <8 x i32>, !dbg !6072
  %_3.i1094.i = lshr <8 x i32> %699, splat (i32 23), !dbg !6076
  %700 = or disjoint <8 x i32> %_3.i1094.i, splat (i32 1258291200), !dbg !6077
  %701 = bitcast <8 x i32> %700 to <8 x float>, !dbg !6081
  %702 = fadd <8 x float> %701, splat (float 0xC160000FE0000000), !dbg !6082
  %703 = fmul <8 x float> %688, %698, !dbg !6086
  %704 = fadd <8 x float> %702, %703, !dbg !6091
  %705 = fmul <8 x float> %704, splat (float 0x4018151820000000), !dbg !6096
  %706 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %705, <8 x float> splat (float -1.600000e+02)), !dbg !6101
  %707 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %706, <8 x float> splat (float 2.400000e+01)), !dbg !6106
  %708 = fsub <8 x float> %707, %lanes.i578.sroa.0.0.copyload.i, !dbg !6111
  %709 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %708, <8 x float> %lanes.i568.sroa.0.0.copyload.i, i8 30), !dbg !6117
  %710 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %708, <8 x float> %566, i8 18), !dbg !6123
  %711 = fadd <8 x float> %lanes.i568.sroa.0.0.copyload.i, %708, !dbg !6129
  %712 = fmul <8 x float> %711, %711, !dbg !6134
  %713 = fmul <8 x float> %lanes.i563.sroa.0.0.copyload.i, %712, !dbg !6139
  %714 = bitcast <8 x float> %709 to <8 x i32>, !dbg !6144
  %715 = icmp slt <8 x i32> %714, zeroinitializer, !dbg !6148
  %.v.i = select <8 x i1> %715, <8 x float> %708, <8 x float> %713, !dbg !6148
  %716 = fmul <8 x float> %lanes.i573.sroa.0.0.copyload.i, %.v.i, !dbg !6148
  %717 = bitcast <8 x float> %710 to <8 x i32>, !dbg !6150
  %718 = icmp slt <8 x i32> %717, zeroinitializer, !dbg !6154
  %719 = select <8 x i1> %718, <8 x float> zeroinitializer, <8 x float> %716, !dbg !6154
  %720 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %719, <8 x float> splat (float -1.000000e+02)), !dbg !6156
  %721 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %720, <8 x float> zeroinitializer), !dbg !6161
  store <8 x float> %721, ptr %_3.i.i.i.i.i.i.i, align 32, !dbg !6166, !alias.scope !4791, !noalias !5203
  %lanes.i648.sroa.0.0.copyload.i = load <8 x float>, ptr %data.i5.i.i.i.i.i.i.i.i, align 4, !dbg !6167, !alias.scope !6172, !noalias !6176
  %722 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i648.sroa.0.0.copyload.i, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !6180
  %723 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %722, <8 x float> splat (float 0x3810000000000000)), !dbg !6186
  %724 = bitcast <8 x float> %723 to <4 x i64>, !dbg !6193
  %725 = and <4 x i64> %724, splat (i64 36028792732385279), !dbg !6194
  %726 = or disjoint <4 x i64> %725, splat (i64 4575657222473777152), !dbg !6199
  %727 = bitcast <4 x i64> %726 to <8 x float>, !dbg !6203
  %728 = fadd <8 x float> %727, splat (float -1.000000e+00), !dbg !6204
  %729 = fmul <8 x float> %728, splat (float 0x3F9B17A960000000), !dbg !6209
  %730 = fsub <8 x float> splat (float 0x3FBF9A8440000000), %729, !dbg !6214
  %731 = fmul <8 x float> %728, %730, !dbg !6209
  %732 = fadd <8 x float> %731, splat (float 0xBFD1E3F400000000), !dbg !6214
  %733 = fmul <8 x float> %728, %732, !dbg !6209
  %734 = fadd <8 x float> %733, splat (float 0x3FDD544F20000000), !dbg !6214
  %735 = fmul <8 x float> %728, %734, !dbg !6209
  %736 = fadd <8 x float> %735, splat (float 0xBFE6FC2A60000000), !dbg !6214
  %737 = fmul <8 x float> %728, %736, !dbg !6209
  %738 = fadd <8 x float> %737, splat (float 0x3FF714B2A0000000), !dbg !6214
  %739 = bitcast <8 x float> %723 to <8 x i32>, !dbg !6219
  %_3.i1095.i = lshr <8 x i32> %739, splat (i32 23), !dbg !6223
  %740 = or disjoint <8 x i32> %_3.i1095.i, splat (i32 1258291200), !dbg !6224
  %741 = bitcast <8 x i32> %740 to <8 x float>, !dbg !6228
  %742 = fadd <8 x float> %741, splat (float 0xC160000FE0000000), !dbg !6229
  %743 = fmul <8 x float> %728, %738, !dbg !6233
  %744 = fadd <8 x float> %742, %743, !dbg !6238
  %745 = fmul <8 x float> %744, splat (float 0x4018151820000000), !dbg !6243
  %746 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %745, <8 x float> splat (float -1.600000e+02)), !dbg !6248
  %747 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %746, <8 x float> splat (float 2.400000e+01)), !dbg !6253
  %748 = fsub <8 x float> %747, %lanes.i618.sroa.0.0.copyload.i, !dbg !6258
  %749 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %748, <8 x float> %lanes.i608.sroa.0.0.copyload.i, i8 30), !dbg !6264
  %750 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %748, <8 x float> %567, i8 18), !dbg !6270
  %751 = fadd <8 x float> %lanes.i608.sroa.0.0.copyload.i, %748, !dbg !6276
  %752 = fmul <8 x float> %751, %751, !dbg !6281
  %753 = fmul <8 x float> %lanes.i603.sroa.0.0.copyload.i, %752, !dbg !6286
  %754 = bitcast <8 x float> %749 to <8 x i32>, !dbg !6291
  %755 = icmp slt <8 x i32> %754, zeroinitializer, !dbg !6295
  %.v1743.i = select <8 x i1> %755, <8 x float> %748, <8 x float> %753, !dbg !6295
  %756 = fmul <8 x float> %lanes.i613.sroa.0.0.copyload.i, %.v1743.i, !dbg !6295
  %757 = bitcast <8 x float> %750 to <8 x i32>, !dbg !6297
  %758 = icmp slt <8 x i32> %757, zeroinitializer, !dbg !6301
  %759 = select <8 x i1> %758, <8 x float> zeroinitializer, <8 x float> %756, !dbg !6301
  %760 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %759, <8 x float> splat (float -1.000000e+02)), !dbg !6303
  %761 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %760, <8 x float> zeroinitializer), !dbg !6308
  store <8 x float> %761, ptr %_3.i.i.i.i.i, align 32, !dbg !6313, !alias.scope !4793, !noalias !5263
  %exitcond.not.i = icmp eq i64 %_9.0.i.i, %_34, !dbg !5134
  br i1 %exitcond.not.i, label %bb15.i, label %bb14.i2633, !dbg !5134

bb82.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit949.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_85.i, i64 noundef %_337.1.i, i64 noundef %_337.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6b0e7efd7567d22a2220c46d452b1d05) #23, !dbg !6314, !noalias !5108
  unreachable, !dbg !6314
}

define internal fastcc void @_RINvNtCse3bfmKSZS8Y_10compressor6kernel13process_blockfEB4_(ptr noalias noundef nonnull align 4 captures(none) %left.0, i64 noundef range(i64 0, 2305843009213693952) %left.1, ptr noalias noundef nonnull align 4 captures(none) %right.0, i64 noundef range(i64 0, 2305843009213693952) %right.1, ptr dead_on_return noalias noundef nonnull readonly align 8 captures(none) dereferenceable(40) %detector, i64 noundef %frames, i32 noundef range(i32 1, 4) %link, i1 noundef zeroext %bypass, i32 noundef %sample_rate, ptr noalias noundef nonnull align 8 dereferenceable(1520) %channels.0, ptr noalias noundef nonnull align 8 dereferenceable(1520) %channels.1, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(64) %staged) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !6315 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %channels.0, i64 556, !dbg !6316
  %_12.i1673 = load i32, ptr %0, align 4, !dbg !6316, !alias.scope !6325, !noundef !11
  %1 = getelementptr inbounds nuw i8, ptr %channels.0, i64 684, !dbg !6316
  %_12.1.i = load i32, ptr %1, align 4, !dbg !6316, !alias.scope !6325, !noundef !11
  %2 = getelementptr inbounds nuw i8, ptr %channels.0, i64 812, !dbg !6316
  %_12.2.i = load i32, ptr %2, align 4, !dbg !6316, !alias.scope !6325, !noundef !11
  %3 = getelementptr inbounds nuw i8, ptr %channels.0, i64 940, !dbg !6316
  %_12.3.i = load i32, ptr %3, align 4, !dbg !6316, !alias.scope !6325, !noundef !11
  %4 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1068, !dbg !6316
  %_12.4.i = load i32, ptr %4, align 4, !dbg !6316, !alias.scope !6325, !noundef !11
  %5 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1196, !dbg !6316
  %_12.5.i = load i32, ptr %5, align 4, !dbg !6316, !alias.scope !6325, !noundef !11
  %6 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1324, !dbg !6316
  %_12.6.i = load i32, ptr %6, align 4, !dbg !6316, !alias.scope !6325, !noundef !11
  %spec.store.select.1.i = tail call i32 @llvm.umax.i32(i32 %_12.1.i, i32 %_12.i1673), !dbg !6316
  %spec.store.select.2.i = tail call i32 @llvm.umax.i32(i32 %_12.2.i, i32 %spec.store.select.1.i), !dbg !6316
  %spec.store.select.3.i = tail call i32 @llvm.umax.i32(i32 %_12.3.i, i32 %spec.store.select.2.i), !dbg !6316
  %spec.store.select.4.i = tail call i32 @llvm.umax.i32(i32 %_12.4.i, i32 %spec.store.select.3.i), !dbg !6316
  %spec.store.select.5.i = tail call i32 @llvm.umax.i32(i32 %_12.5.i, i32 %spec.store.select.4.i), !dbg !6316
  %spec.store.select.6.i = tail call noundef i32 @llvm.umax.i32(i32 %_12.6.i, i32 %spec.store.select.5.i), !dbg !6316
  %7 = getelementptr inbounds nuw i8, ptr %channels.1, i64 556, !dbg !6328
  %_12.i1674 = load i32, ptr %7, align 4, !dbg !6328, !alias.scope !6330, !noundef !11
  %8 = getelementptr inbounds nuw i8, ptr %channels.1, i64 684, !dbg !6328
  %_12.1.i1675 = load i32, ptr %8, align 4, !dbg !6328, !alias.scope !6330, !noundef !11
  %9 = getelementptr inbounds nuw i8, ptr %channels.1, i64 812, !dbg !6328
  %_12.2.i1676 = load i32, ptr %9, align 4, !dbg !6328, !alias.scope !6330, !noundef !11
  %10 = getelementptr inbounds nuw i8, ptr %channels.1, i64 940, !dbg !6328
  %_12.3.i1677 = load i32, ptr %10, align 4, !dbg !6328, !alias.scope !6330, !noundef !11
  %11 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1068, !dbg !6328
  %_12.4.i1678 = load i32, ptr %11, align 4, !dbg !6328, !alias.scope !6330, !noundef !11
  %12 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1196, !dbg !6328
  %_12.5.i1679 = load i32, ptr %12, align 4, !dbg !6328, !alias.scope !6330, !noundef !11
  %13 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1324, !dbg !6328
  %_12.6.i1680 = load i32, ptr %13, align 4, !dbg !6328, !alias.scope !6330, !noundef !11
  %spec.store.select.1.i1681 = tail call i32 @llvm.umax.i32(i32 %_12.1.i1675, i32 %_12.i1674), !dbg !6328
  %spec.store.select.2.i1682 = tail call i32 @llvm.umax.i32(i32 %_12.2.i1676, i32 %spec.store.select.1.i1681), !dbg !6328
  %spec.store.select.3.i1683 = tail call i32 @llvm.umax.i32(i32 %_12.3.i1677, i32 %spec.store.select.2.i1682), !dbg !6328
  %spec.store.select.4.i1684 = tail call i32 @llvm.umax.i32(i32 %_12.4.i1678, i32 %spec.store.select.3.i1683), !dbg !6328
  %spec.store.select.5.i1685 = tail call i32 @llvm.umax.i32(i32 %_12.5.i1679, i32 %spec.store.select.4.i1684), !dbg !6328
  %spec.store.select.6.i1686 = tail call noundef i32 @llvm.umax.i32(i32 %_12.6.i1680, i32 %spec.store.select.5.i1685), !dbg !6328
  %..i1687 = tail call noundef i32 @llvm.umax.i32(i32 %spec.store.select.6.i1686, i32 %spec.store.select.6.i), !dbg !6333
  %14 = zext i32 %..i1687 to i64, !dbg !6335
  %_24 = icmp ugt i64 %frames, %14, !dbg !6336
  %spec.store.select = tail call i64 @llvm.umin.i64(i64 %frames, i64 %14), !dbg !6336
  %_25.not = icmp eq i64 %spec.store.select, 0, !dbg !6338
  br i1 %_25.not, label %bb10, label %bb7, !dbg !6338

bb7:                                              ; preds = %start
  %.sroa.0.0.copyload = load i64, ptr %detector, align 8, !dbg !6340
  %.sroa.4.0.detector.sroa_idx = getelementptr inbounds nuw i8, ptr %detector, i64 8, !dbg !6340
  %.sroa.4.0.copyload = load ptr, ptr %.sroa.4.0.detector.sroa_idx, align 8, !dbg !6340
  %.sroa.5.0.detector.sroa_idx = getelementptr inbounds nuw i8, ptr %detector, i64 16, !dbg !6340
  %.sroa.5.0.copyload = load i64, ptr %.sroa.5.0.detector.sroa_idx, align 8, !dbg !6340
  %.sroa.6.0.detector.sroa_idx = getelementptr inbounds nuw i8, ptr %detector, i64 24, !dbg !6340
  %.sroa.6.0.copyload = load ptr, ptr %.sroa.6.0.detector.sroa_idx, align 8, !dbg !6340
  %.sroa.7.0.detector.sroa_idx = getelementptr inbounds nuw i8, ptr %detector, i64 32, !dbg !6340
  %.sroa.7.0.copyload = load i64, ptr %.sroa.7.0.detector.sroa_idx, align 8, !dbg !6340
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6341), !dbg !6340
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6344), !dbg !6340
  %15 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1512, !dbg !6346
  %_12.i11 = load i32, ptr %15, align 8, !dbg !6346, !alias.scope !6341, !noalias !6350, !noundef !11
  %ring_length.i12 = zext i32 %_12.i11 to i64, !dbg !6346
  %16 = icmp eq i32 %link, 1, !dbg !6354
  %.not = icmp eq i32 %link, 3, !dbg !6363
  %_15.i13 = getelementptr inbounds nuw i8, ptr %channels.0, i64 288, !dbg !6366
  %_4.i666 = getelementptr inbounds nuw i8, ptr %channels.0, i64 480, !dbg !6368
  %_7.i668 = getelementptr inbounds nuw i8, ptr %channels.0, i64 512, !dbg !6371
  %_14.i671 = getelementptr inbounds nuw i8, ptr %channels.0, i64 320, !dbg !6373
  %_17.i673 = getelementptr inbounds nuw i8, ptr %channels.0, i64 352, !dbg !6375
  %_20.i675 = getelementptr inbounds nuw i8, ptr %channels.0, i64 384, !dbg !6376
  %_23.i677 = getelementptr inbounds nuw i8, ptr %channels.0, i64 416, !dbg !6377
  %_26.i679 = getelementptr inbounds nuw i8, ptr %channels.0, i64 448, !dbg !6378
  %_17.i14 = getelementptr inbounds nuw i8, ptr %channels.1, i64 288, !dbg !6379
  %_4.i642 = getelementptr inbounds nuw i8, ptr %channels.1, i64 480, !dbg !6381
  %_7.i644 = getelementptr inbounds nuw i8, ptr %channels.1, i64 512, !dbg !6383
  %_14.i647 = getelementptr inbounds nuw i8, ptr %channels.1, i64 320, !dbg !6384
  %_17.i649 = getelementptr inbounds nuw i8, ptr %channels.1, i64 352, !dbg !6385
  %_20.i651 = getelementptr inbounds nuw i8, ptr %channels.1, i64 384, !dbg !6386
  %_23.i653 = getelementptr inbounds nuw i8, ptr %channels.1, i64 416, !dbg !6387
  %_26.i655 = getelementptr inbounds nuw i8, ptr %channels.1, i64 448, !dbg !6388
  %17 = icmp ne ptr %.sroa.6.0.copyload, null
  %18 = icmp ne ptr %.sroa.4.0.copyload, null
  %19 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1508
  %20 = getelementptr inbounds nuw i8, ptr %channels.0, i64 8
  %21 = getelementptr inbounds nuw i8, ptr %channels.0, i64 24
  %22 = getelementptr inbounds nuw i8, ptr %channels.0, i64 16
  %23 = getelementptr inbounds nuw i8, ptr %channels.1, i64 8
  %24 = getelementptr inbounds nuw i8, ptr %channels.1, i64 24
  %25 = getelementptr inbounds nuw i8, ptr %channels.1, i64 16
  %26 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1472
  %27 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1512
  %28 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1472
  %_69.i103 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1504
  %_73.i107 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1504
  %29 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1508
  %30 = add nuw nsw i64 %left.1, 1, !dbg !6389
  %31 = add nuw nsw i64 %right.1, 1, !dbg !6389
  br label %bb27.i19, !dbg !6389

bb27.i19:                                         ; preds = %bb7, %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit
  %start1.sroa.0.0.i172177 = phi i64 [ 0, %bb7 ], [ %32, %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit ]
  %32 = add nuw nsw i64 %start1.sroa.0.0.i172177, 1, !dbg !6398
; call <compressor::kernel::Channel<f32>>::advance_ramps
  tail call fastcc void @_RNvMNtCse3bfmKSZS8Y_10compressor6kernelINtB2_7ChannelfE13advance_rampsB4_(ptr noalias noundef nonnull align 8 dereferenceable(1520) %channels.0, i32 noundef %sample_rate) #22, !dbg !6404, !noalias !6406
; call <compressor::kernel::Channel<f32>>::advance_ramps
  tail call fastcc void @_RNvMNtCse3bfmKSZS8Y_10compressor6kernelINtB2_7ChannelfE13advance_rampsB4_(ptr noalias noundef nonnull align 8 dereferenceable(1520) %channels.1, i32 noundef %sample_rate) #22, !dbg !6407, !noalias !6406
  %_0.i1025 = load float, ptr %_4.i666, align 4, !dbg !6408, !alias.scope !6415, !noalias !6418, !noundef !11
  %_0.i1023 = load float, ptr %_7.i668, align 4, !dbg !6421, !alias.scope !6423, !noalias !6418, !noundef !11
  %_0.i1021 = load float, ptr %_15.i13, align 4, !dbg !6426, !alias.scope !6428, !noalias !6418, !noundef !11
  %_0.i1019 = load float, ptr %_14.i671, align 4, !dbg !6431, !alias.scope !6433, !noalias !6418, !noundef !11
  %_0.i1017 = load float, ptr %_17.i673, align 4, !dbg !6436, !alias.scope !6438, !noalias !6418, !noundef !11
  %_0.i1015 = load float, ptr %_20.i675, align 4, !dbg !6441, !alias.scope !6443, !noalias !6418, !noundef !11
  %_0.i10131790 = load i32, ptr %_23.i677, align 4, !dbg !6446, !alias.scope !6448, !noalias !6418, !noundef !11
  %_0.i10111791 = load i32, ptr %_26.i679, align 4, !dbg !6451, !alias.scope !6453, !noalias !6418, !noundef !11
  %_3.i780 = fcmp une float %_0.i1023, 1.000000e+00, !dbg !6456
  %_3.i778 = fcmp oeq float %_0.i1023, 0.000000e+00, !dbg !6459
  %_3.i776 = fcmp oeq float %_0.i1025, 0.000000e+00, !dbg !6461
  %_0.i1041 = load float, ptr %_4.i642, align 4, !dbg !6463, !alias.scope !6466, !noalias !6469, !noundef !11
  %_0.i1039 = load float, ptr %_7.i644, align 4, !dbg !6472, !alias.scope !6474, !noalias !6469, !noundef !11
  %_0.i1037 = load float, ptr %_17.i14, align 4, !dbg !6477, !alias.scope !6479, !noalias !6469, !noundef !11
  %_0.i1035 = load float, ptr %_14.i647, align 4, !dbg !6482, !alias.scope !6484, !noalias !6469, !noundef !11
  %_0.i1033 = load float, ptr %_17.i649, align 4, !dbg !6487, !alias.scope !6489, !noalias !6469, !noundef !11
  %_0.i1031 = load float, ptr %_20.i651, align 4, !dbg !6492, !alias.scope !6494, !noalias !6469, !noundef !11
  %_0.i10291792 = load i32, ptr %_23.i653, align 4, !dbg !6497, !alias.scope !6499, !noalias !6469, !noundef !11
  %_0.i10271793 = load i32, ptr %_26.i655, align 4, !dbg !6502, !alias.scope !6504, !noalias !6469, !noundef !11
  %_3.i786 = fcmp une float %_0.i1039, 1.000000e+00, !dbg !6507
  %_3.i784 = fcmp oeq float %_0.i1039, 0.000000e+00, !dbg !6509
  %_3.i782 = fcmp oeq float %_0.i1041, 0.000000e+00, !dbg !6511
  %exitcond = icmp eq i64 %start1.sroa.0.0.i172177, %30, !dbg !6513
  br i1 %exitcond, label %bb29.i122, label %bb30.i21, !dbg !6513, !prof !161

bb30.i21:                                         ; preds = %bb27.i19
  %_96.i23 = getelementptr inbounds nuw float, ptr %left.0, i64 %start1.sroa.0.0.i172177, !dbg !6519
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6524), !dbg !6527
  %_3.not.i1073 = icmp eq i64 %left.1, %start1.sroa.0.0.i172177, !dbg !6528
  br i1 %_3.not.i1073, label %panic.i1076, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1077, !dbg !6528

panic.i1076:                                      ; preds = %bb30.i21
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !6528, !noalias !6530
  unreachable, !dbg !6528

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1077: ; preds = %bb30.i21
  %_0.i1075 = load float, ptr %_96.i23, align 4, !dbg !6528, !alias.scope !6524, !noalias !6406, !noundef !11
  %exitcond2515 = icmp eq i64 %start1.sroa.0.0.i172177, %31, !dbg !6531
  br i1 %exitcond2515, label %bb31.i121, label %bb32.i26, !dbg !6531, !prof !161

bb29.i122:                                        ; preds = %bb27.i19
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %30, i64 noundef range(i64 0, 2305843009213693952) %left.1, i64 noundef range(i64 0, 2305843009213693952) %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c00de08b5cdb434b6310222ddd96a0fc) #23, !dbg !6536, !noalias !6406
  unreachable, !dbg !6536

bb32.i26:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1077
  %_104.i28 = getelementptr inbounds nuw float, ptr %right.0, i64 %start1.sroa.0.0.i172177, !dbg !6537
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6542), !dbg !6545
  %_3.not.i1068 = icmp eq i64 %right.1, %start1.sroa.0.0.i172177, !dbg !6546
  br i1 %_3.not.i1068, label %panic.i1071, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1072, !dbg !6546

panic.i1071:                                      ; preds = %bb32.i26
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !6546, !noalias !6548
  unreachable, !dbg !6546

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1072: ; preds = %bb32.i26
  %_0.i1070 = load float, ptr %_104.i28, align 4, !dbg !6546, !alias.scope !6542, !noalias !6406, !noundef !11
  switch i64 %.sroa.0.0.copyload, label %default.unreachable.i.i120 [
    i64 0, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i44
    i64 1, label %bb3.i.i119
    i64 2, label %bb2.i.i31
  ], !dbg !6549

default.unreachable.i.i120:                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1072
  unreachable

bb3.i.i119:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1072
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i44, !dbg !6553

bb2.i.i31:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1072
  tail call void @llvm.assume(i1 %17)
  %_25.i.i35 = icmp ugt i64 %start1.sroa.0.0.i172177, %.sroa.5.0.copyload, !dbg !6554
  br i1 %_25.i.i35, label %bb17.i.i118, label %bb18.i.i36, !dbg !6554, !prof !161

bb18.i.i36:                                       ; preds = %bb2.i.i31
  tail call void @llvm.assume(i1 %18)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6560), !dbg !6563
  %_3.not.i1063 = icmp eq i64 %.sroa.5.0.copyload, %start1.sroa.0.0.i172177, !dbg !6564
  br i1 %_3.not.i1063, label %panic.i1066, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1067, !dbg !6564

panic.i1066:                                      ; preds = %bb18.i.i36
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !6564, !noalias !6566
  unreachable, !dbg !6564

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1067: ; preds = %bb18.i.i36
  %_32.i.i39 = getelementptr inbounds nuw float, ptr %.sroa.4.0.copyload, i64 %start1.sroa.0.0.i172177, !dbg !6570
  %_0.i1065 = load float, ptr %_32.i.i39, align 4, !dbg !6564, !alias.scope !6560, !noalias !6575, !noundef !11
  %_33.i.i40 = icmp ugt i64 %start1.sroa.0.0.i172177, %.sroa.7.0.copyload, !dbg !6576
  br i1 %_33.i.i40, label %bb19.i.i117, label %bb20.i.i41, !dbg !6576, !prof !161

bb17.i.i118:                                      ; preds = %bb2.i.i31
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 2305843009213693952) %start1.sroa.0.0.i172177, i64 noundef %.sroa.5.0.copyload, i64 noundef %.sroa.5.0.copyload, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a0a4023c361dd50869bbb08b76482f50) #23, !dbg !6580, !noalias !6575
  unreachable, !dbg !6580

bb20.i.i41:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1067
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6581), !dbg !6584
  %_3.not.i1058 = icmp eq i64 %.sroa.7.0.copyload, %start1.sroa.0.0.i172177, !dbg !6585
  br i1 %_3.not.i1058, label %panic.i1061, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1062, !dbg !6585

panic.i1061:                                      ; preds = %bb20.i.i41
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !6585, !noalias !6587
  unreachable, !dbg !6585

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1062: ; preds = %bb20.i.i41
  %_40.i.i43 = getelementptr inbounds nuw float, ptr %.sroa.6.0.copyload, i64 %start1.sroa.0.0.i172177, !dbg !6588
  %_0.i1060 = load float, ptr %_40.i.i43, align 4, !dbg !6585, !alias.scope !6581, !noalias !6575, !noundef !11
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i44, !dbg !6593

bb19.i.i117:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1067
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 2305843009213693952) %start1.sroa.0.0.i172177, i64 noundef %.sroa.7.0.copyload, i64 noundef %.sroa.7.0.copyload, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5807a2a07e9839e3d32631b4243eb17f) #23, !dbg !6594, !noalias !6575
  unreachable, !dbg !6594

_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i44: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1062, %bb3.i.i119, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1072
  %main_right.sroa.0.0.i.i45 = phi float [ %_0.i1070, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1072 ], [ 0.000000e+00, %bb3.i.i119 ], [ %_0.i1060, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1062 ]
  %main_left.sroa.0.0.i.i46 = phi float [ %_0.i1075, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1072 ], [ 0.000000e+00, %bb3.i.i119 ], [ %_0.i1065, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1062 ]
  %33 = tail call noundef float @llvm.fabs.f32(float %main_left.sroa.0.0.i.i46), !dbg !6595
  %34 = tail call noundef float @llvm.fabs.f32(float %main_right.sroa.0.0.i.i45), !dbg !6599
  %_3.i.i1551 = fcmp ule float %33, %34, !dbg !6602
  %_6.i.i1553 = bitcast float %33 to i32, !dbg !6609
  %_8.i.i1555 = bitcast float %34 to i32, !dbg !6614
  %_4.i.i1558 = select i1 %_3.i.i1551, i32 %_8.i.i1555, i32 %_6.i.i1553, !dbg !6616
  %_0.i923 = fmul float %33, 5.000000e-01, !dbg !6617
  %_0.i922 = fmul float %34, 5.000000e-01, !dbg !6621
  %_0.i852 = fadd float %_0.i922, %_0.i923, !dbg !6623
  %_6.i1347 = bitcast float %_0.i852 to i32, !dbg !6626
  %_4.i1352 = select i1 %.not, i32 %_6.i1347, i32 %_4.i.i1558, !dbg !6630
  %_4.i1345 = select i1 %16, i32 %_6.i.i1553, i32 %_4.i1352, !dbg !6631
  %_4.i1338 = select i1 %16, i32 %_8.i.i1555, i32 %_4.i1352, !dbg !6634
  %_37.i59 = load i32, ptr %19, align 4, !dbg !6636, !alias.scope !6341, !noalias !6350, !noundef !11
  %write.i60 = zext i32 %_37.i59 to i64, !dbg !6636
  %35 = add nuw nsw i64 %write.i60, 1, !dbg !6638
  %_39.i61 = icmp eq i64 %35, %ring_length.i12, !dbg !6640
  %spec.store.select.i62 = select i1 %_39.i61, i64 0, i64 %35, !dbg !6640
  %_165.1.i63 = load i64, ptr %20, align 8, !dbg !6642, !alias.scope !6341, !noalias !6350, !noundef !11
  %_105.i64 = icmp ult i64 %_165.1.i63, %write.i60, !dbg !6644
  br i1 %_105.i64, label %bb33.i116, label %bb34.i65, !dbg !6644, !prof !161

bb31.i121:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1077
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %31, i64 noundef range(i64 0, 2305843009213693952) %right.1, i64 noundef range(i64 0, 2305843009213693952) %right.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_99a388bbc796e7fd7b28fa01c6ed4e6b) #23, !dbg !6649, !noalias !6406
  unreachable, !dbg !6649

bb34.i65:                                         ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i44
  %_165.0.i66 = load ptr, ptr %channels.0, align 8, !dbg !6642, !alias.scope !6341, !noalias !6350, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6650), !dbg !6653
  %_4.not.i1154 = icmp eq i64 %_165.1.i63, %write.i60, !dbg !6654
  br i1 %_4.not.i1154, label %panic.i1156, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1157, !dbg !6654

panic.i1156:                                      ; preds = %bb34.i65
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #23, !dbg !6654, !noalias !6657
  unreachable, !dbg !6654

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1157: ; preds = %bb34.i65
  %_112.i68 = getelementptr inbounds nuw float, ptr %_165.0.i66, i64 %write.i60, !dbg !6658
  store float %_0.i1075, ptr %_112.i68, align 4, !dbg !6654, !alias.scope !6650, !noalias !6406
  %_166.1.i69 = load i64, ptr %21, align 8, !dbg !6663, !alias.scope !6341, !noalias !6350, !noundef !11
  %_113.i70 = icmp ult i64 %_166.1.i69, %write.i60, !dbg !6664
  br i1 %_113.i70, label %bb35.i115, label %bb36.i71, !dbg !6664, !prof !161

bb33.i116:                                        ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i44
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i60, i64 noundef %_165.1.i63, i64 noundef %_165.1.i63, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7fd19a98f6b7f6813fdc6b43fefd82a4) #23, !dbg !6668, !noalias !6406
  unreachable, !dbg !6668

bb36.i71:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1157
  %_166.0.i72 = load ptr, ptr %22, align 8, !dbg !6663, !alias.scope !6341, !noalias !6350, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6669), !dbg !6672
  %_4.not.i1150 = icmp eq i64 %_166.1.i69, %write.i60, !dbg !6673
  br i1 %_4.not.i1150, label %panic.i1152, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1153, !dbg !6673

panic.i1152:                                      ; preds = %bb36.i71
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #23, !dbg !6673, !noalias !6675
  unreachable, !dbg !6673

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1153: ; preds = %bb36.i71
  %_120.i74 = getelementptr inbounds nuw float, ptr %_166.0.i72, i64 %write.i60, !dbg !6676
  store i32 %_4.i1345, ptr %_120.i74, align 4, !dbg !6673, !alias.scope !6669, !noalias !6406
  %_167.1.i75 = load i64, ptr %23, align 8, !dbg !6681, !alias.scope !6344, !noalias !6682, !noundef !11
  %_121.i76 = icmp ult i64 %_167.1.i75, %write.i60, !dbg !6683
  br i1 %_121.i76, label %bb37.i114, label %bb38.i77, !dbg !6683, !prof !161

bb35.i115:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1157
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i60, i64 noundef %_166.1.i69, i64 noundef %_166.1.i69, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_3898a5b9e77e549bf29e2ec14b43830a) #23, !dbg !6687, !noalias !6406
  unreachable, !dbg !6687

bb38.i77:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1153
  %_167.0.i78 = load ptr, ptr %channels.1, align 8, !dbg !6681, !alias.scope !6344, !noalias !6682, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6688), !dbg !6691
  %_4.not.i1146 = icmp eq i64 %_167.1.i75, %write.i60, !dbg !6692
  br i1 %_4.not.i1146, label %panic.i1148, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1149, !dbg !6692

panic.i1148:                                      ; preds = %bb38.i77
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #23, !dbg !6692, !noalias !6694
  unreachable, !dbg !6692

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1149: ; preds = %bb38.i77
  %_128.i80 = getelementptr inbounds nuw float, ptr %_167.0.i78, i64 %write.i60, !dbg !6695
  store float %_0.i1070, ptr %_128.i80, align 4, !dbg !6692, !alias.scope !6688, !noalias !6406
  %_168.1.i81 = load i64, ptr %24, align 8, !dbg !6700, !alias.scope !6344, !noalias !6682, !noundef !11
  %_129.i82 = icmp ult i64 %_168.1.i81, %write.i60, !dbg !6701
  br i1 %_129.i82, label %bb39.i113, label %bb40.i83, !dbg !6701, !prof !161

bb37.i114:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1153
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i60, i64 noundef %_167.1.i75, i64 noundef %_167.1.i75, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ef27b7b7ba2d23db7b66788c02ac8976) #23, !dbg !6705, !noalias !6406
  unreachable, !dbg !6705

bb40.i83:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1149
  %_168.0.i84 = load ptr, ptr %25, align 8, !dbg !6700, !alias.scope !6344, !noalias !6682, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6706), !dbg !6709
  %_4.not.i1142 = icmp eq i64 %_168.1.i81, %write.i60, !dbg !6710
  br i1 %_4.not.i1142, label %panic.i1144, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1145, !dbg !6710

panic.i1144:                                      ; preds = %bb40.i83
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #23, !dbg !6710, !noalias !6712
  unreachable, !dbg !6710

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1145: ; preds = %bb40.i83
  %_136.i86 = getelementptr inbounds nuw float, ptr %_168.0.i84, i64 %write.i60, !dbg !6713
  store i32 %_4.i1338, ptr %_136.i86, align 4, !dbg !6710, !alias.scope !6706, !noalias !6406
  %_169.1.i87 = load i64, ptr %20, align 8, !dbg !6718, !alias.scope !6341, !noalias !6350, !noundef !11
  %_137.i88 = icmp ugt i64 %spec.store.select.i62, %_169.1.i87, !dbg !6719
  br i1 %_137.i88, label %bb41.i112, label %bb42.i89, !dbg !6719, !prof !161

bb39.i113:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1149
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i60, i64 noundef %_168.1.i81, i64 noundef %_168.1.i81, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_23eb2a707ff31dba52171dada526a36f) #23, !dbg !6723, !noalias !6406
  unreachable, !dbg !6723

bb42.i89:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1145
  %_169.0.i90 = load ptr, ptr %channels.0, align 8, !dbg !6718, !alias.scope !6341, !noalias !6350, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6724), !dbg !6727
  %_3.not.i1053 = icmp eq i64 %_169.1.i87, %spec.store.select.i62, !dbg !6728
  br i1 %_3.not.i1053, label %panic.i1056, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1057, !dbg !6728

panic.i1056:                                      ; preds = %bb42.i89
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !6728, !noalias !6730
  unreachable, !dbg !6728

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1057: ; preds = %bb42.i89
  %_144.i92 = getelementptr inbounds nuw float, ptr %_169.0.i90, i64 %spec.store.select.i62, !dbg !6731
  %_0.i1055 = load float, ptr %_144.i92, align 4, !dbg !6728, !alias.scope !6724, !noalias !6406, !noundef !11
  %_170.1.i94 = load i64, ptr %23, align 8, !dbg !6736, !alias.scope !6344, !noalias !6682, !noundef !11
  %_145.i95 = icmp ugt i64 %spec.store.select.i62, %_170.1.i94, !dbg !6738
  br i1 %_145.i95, label %bb43.i111, label %bb44.i96, !dbg !6738, !prof !161

bb41.i112:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1145
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %spec.store.select.i62, i64 noundef %_169.1.i87, i64 noundef %_169.1.i87, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e74b405e18c5e1dc98b750b7a797cfef) #23, !dbg !6742, !noalias !6406
  unreachable, !dbg !6742

bb44.i96:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1057
  %_170.0.i97 = load ptr, ptr %channels.1, align 8, !dbg !6736, !alias.scope !6344, !noalias !6682, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6743), !dbg !6746
  %_3.not.i = icmp eq i64 %_170.1.i94, %spec.store.select.i62, !dbg !6747
  br i1 %_3.not.i, label %panic.i1052, label %bb6.i187, !dbg !6747

panic.i1052:                                      ; preds = %bb44.i96
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !6747, !noalias !6749
  unreachable, !dbg !6747

bb6.i187:                                         ; preds = %bb44.i96
  %_152.i99 = getelementptr inbounds nuw float, ptr %_170.0.i97, i64 %spec.store.select.i62, !dbg !6750
  %_0.i1051 = load float, ptr %_152.i99, align 4, !dbg !6747, !alias.scope !6743, !noalias !6406, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6755), !dbg !6758
  %_5.i176 = load i32, ptr %15, align 8, !dbg !6760, !alias.scope !6755, !noalias !6764, !noundef !11
  %_38.1.i195 = load i64, ptr %21, align 8
  %_17.i188 = load i32, ptr %26, align 4, !dbg !6766, !alias.scope !6755, !noalias !6764, !noundef !11
  %delay.i189 = zext i32 %_17.i188 to i64, !dbg !6766
  %_20.not.i190 = icmp ult i32 %_37.i59, %_17.i188, !dbg !6770
  %narrow = select i1 %_20.not.i190, i32 %_5.i176, i32 0, !dbg !6770
  %_21.i191 = zext i32 %narrow to i64, !dbg !6770
  %tap.sroa.0.0.i193 = sub nsw i64 %write.i60, %delay.i189, !dbg !6772
  %_23.i194 = add nsw i64 %tap.sroa.0.0.i193, %_21.i191, !dbg !6773
  %_26.i196 = icmp ult i64 %_23.i194, %_38.1.i195, !dbg !6775
  br i1 %_26.i196, label %bb6.i, label %panic1.i197, !dbg !6775

panic1.i197:                                      ; preds = %bb6.i187
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_23.i194, i64 noundef %_38.1.i195, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a529a882edb544ae33c686afc6011081) #23, !dbg !6775, !noalias !6776
  unreachable, !dbg !6775

bb6.i:                                            ; preds = %bb6.i187
  %_38.0.i199 = load ptr, ptr %22, align 8, !nonnull !11
  %36 = getelementptr inbounds nuw float, ptr %_38.0.i199, i64 %_23.i194, !dbg !6775
  %_22.i200 = load float, ptr %36, align 4, !dbg !6775, !noalias !6776, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6777), !dbg !6780
  %_5.i169 = load i32, ptr %27, align 8, !dbg !6782, !alias.scope !6777, !noalias !6784, !noundef !11
  %_38.1.i = load i64, ptr %24, align 8
  %_17.i172 = load i32, ptr %28, align 4, !dbg !6786, !alias.scope !6777, !noalias !6784, !noundef !11
  %delay.i = zext i32 %_17.i172 to i64, !dbg !6786
  %_20.not.i = icmp ult i32 %_37.i59, %_17.i172, !dbg !6787
  %narrow1798 = select i1 %_20.not.i, i32 %_5.i169, i32 0, !dbg !6787
  %_21.i = zext i32 %narrow1798 to i64, !dbg !6787
  %tap.sroa.0.0.i = sub nsw i64 %write.i60, %delay.i, !dbg !6788
  %_23.i173 = add nsw i64 %tap.sroa.0.0.i, %_21.i, !dbg !6789
  %_26.i = icmp ult i64 %_23.i173, %_38.1.i, !dbg !6790
  br i1 %_26.i, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit, label %panic1.i, !dbg !6790

panic1.i:                                         ; preds = %bb6.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_23.i173, i64 noundef %_38.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a529a882edb544ae33c686afc6011081) #23, !dbg !6790, !noalias !6791
  unreachable, !dbg !6790

_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit: ; preds = %bb6.i
  %_38.0.i = load ptr, ptr %25, align 8, !nonnull !11
  %37 = getelementptr inbounds nuw float, ptr %_38.0.i, i64 %_23.i173, !dbg !6790
  %_22.i = load float, ptr %37, align 4, !dbg !6790, !noalias !6791, !noundef !11
  %_3.i.i1515 = fcmp ule float %_22.i200, 0x3E45798EE0000000, !dbg !6792
  %_6.i.i1517 = bitcast float %_22.i200 to i32, !dbg !6800
  %_4.i.i1522 = select i1 %_3.i.i1515, i32 841731191, i32 %_6.i.i1517, !dbg !6803
  %_0.i.i1523 = bitcast i32 %_4.i.i1522 to float, !dbg !6804
  %_3.i.i1388 = fcmp ule float %_0.i.i1523, 0x3810000000000000, !dbg !6807
  %_4.i.i1394 = select i1 %_3.i.i1388, i32 8388608, i32 %_4.i.i1522, !dbg !6815
  %_5.i1113 = and i32 %_4.i.i1394, 8388607, !dbg !6817
  %_4.i1114 = or disjoint i32 %_5.i1113, 1065353216, !dbg !6817
  %significand.i1115 = bitcast i32 %_4.i1114 to float, !dbg !6822
  %_0.i927 = fadd float %significand.i1115, -1.000000e+00, !dbg !6825
  %_0.i873 = fmul float %_0.i927, 0xBF9B17A960000000, !dbg !6829
  %_0.i831 = fadd float %_0.i873, 0x3FBF9A8440000000, !dbg !6834
  %_0.i873.1 = fmul float %_0.i927, %_0.i831, !dbg !6829
  %_0.i831.1 = fadd float %_0.i873.1, 0xBFD1E3F400000000, !dbg !6834
  %_0.i873.2 = fmul float %_0.i927, %_0.i831.1, !dbg !6829
  %_0.i831.2 = fadd float %_0.i873.2, 0x3FDD544F20000000, !dbg !6834
  %_0.i873.3 = fmul float %_0.i927, %_0.i831.2, !dbg !6829
  %_0.i831.3 = fadd float %_0.i873.3, 0xBFE6FC2A60000000, !dbg !6834
  %_0.i873.4 = fmul float %_0.i927, %_0.i831.3, !dbg !6829
  %_0.i831.4 = fadd float %_0.i873.4, 0x3FF714B2A0000000, !dbg !6834
  %_9.i1116 = lshr i32 %_4.i.i1394, 23, !dbg !6836
  %_8.i1117 = or disjoint i32 %_9.i1116, 1258291200, !dbg !6836
  %_7.i1118 = bitcast i32 %_8.i1117 to float, !dbg !6838
  %exponent.i1119 = fadd float %_7.i1118, 0xC160000FE0000000, !dbg !6840
  %_0.i872 = fmul float %_0.i927, %_0.i831.4, !dbg !6841
  %_0.i830 = fadd float %exponent.i1119, %_0.i872, !dbg !6843
  %_0.i920 = fmul float %_0.i830, 0x4018151820000000, !dbg !6845
  %_3.i.i1506.inv = fcmp ogt float %_0.i920, -1.600000e+02, !dbg !6847
  %_0.i.i1514 = select i1 %_3.i.i1506.inv, float %_0.i920, float -1.600000e+02, !dbg !6847
  %_3.i.i1646.inv = fcmp olt float %_0.i.i1514, 2.400000e+01, !dbg !6850
  %_0.i.i1654 = select i1 %_3.i.i1646.inv, float %_0.i.i1514, float 2.400000e+01, !dbg !6850
  %_0.i935 = fsub float %_0.i.i1654, %_0.i1021, !dbg !6854
  %_3.i798 = fcmp ule float %_0.i935, %_0.i1017, !dbg !6859
  %38 = fneg float %_0.i1017, !dbg !6862
  %_0.i845 = fadd float %_0.i1017, %_0.i935, !dbg !6866
  %_0.i896 = fmul float %_0.i845, %_0.i845, !dbg !6870
  %_0.i895 = fmul float %_0.i1015, %_0.i896, !dbg !6873
  %_4.i1221.v.v = select i1 %_3.i798, float %_0.i895, float %_0.i935, !dbg !6875
  %_4.i1221.v = fmul float %_0.i1019, %_4.i1221.v.v, !dbg !6875
  %_4.i1221 = bitcast float %_4.i1221.v to i32, !dbg !6875
  %39 = fcmp ugt float %_0.i935, %38, !dbg !6878
  %_7.i1213 = select i1 %39, i32 %_4.i1221, i32 0, !dbg !6880
  %_0.i1215 = bitcast i32 %_7.i1213 to float, !dbg !6881
  %_3.i.i1497 = fcmp ule float %_0.i1215, -1.000000e+02, !dbg !6883
  %40 = bitcast i32 %_7.i1213 to float, !dbg !6886
  %_0.i.i1505 = select i1 %_3.i.i1497, float -1.000000e+02, float %40, !dbg !6889
  %_3.i.i1637 = fcmp olt float %_0.i.i1505, 0.000000e+00, !dbg !6890
  %_0.i.i1645 = select i1 %_3.i.i1637, float %_0.i.i1505, float 0.000000e+00, !dbg !6894
  %_6.i344 = load float, ptr %_69.i103, align 4, !dbg !6896, !alias.scope !6900, !noalias !6903, !noundef !11
  %_3.i822 = fcmp uge float %_0.i.i1645, %_6.i344, !dbg !6905
  %_4.i1268 = select i1 %_3.i822, i32 %_0.i10111791, i32 %_0.i10131790, !dbg !6907
  %_0.i1269 = bitcast i32 %_4.i1268 to float, !dbg !6909
  %_0.i940 = fsub float %_0.i.i1645, %_6.i344, !dbg !6911
  %_4.i858 = fmul float %_0.i940, %_0.i1269, !dbg !6916
  %_0.i859 = fadd float %_6.i344, %_4.i858, !dbg !6916
  %41 = tail call noundef float @llvm.fabs.f32(float %_0.i859), !dbg !6920
  %42 = fcmp uge float %41, 0x3BC79CA100000000, !dbg !6924
  %_0.i1193 = select i1 %42, float %_0.i859, float 0.000000e+00, !dbg !6927
  store float %_0.i1193, ptr %_69.i103, align 4, !dbg !6928, !alias.scope !6900, !noalias !6903
  %_0.i850 = fadd float %_0.i1025, %_0.i1193, !dbg !6930
  %_0.i914 = fmul float %_0.i850, 0x3FC542A5A0000000, !dbg !6935
  %_3.i.i1420.inv = fcmp ogt float %_0.i914, -1.260000e+02, !dbg !6939
  %_0.i.i1427 = select i1 %_3.i.i1420.inv, float %_0.i914, float -1.260000e+02, !dbg !6939
  %_3.i.i1577.inv = fcmp olt float %_0.i.i1427, 1.270000e+02, !dbg !6944
  %_0.i.i1584 = select i1 %_3.i.i1577.inv, float %_0.i.i1427, float 1.270000e+02, !dbg !6944
  %43 = tail call noundef float @llvm.floor.f32(float %_0.i.i1584), !dbg !6947
  %_0.i931 = fsub float %_0.i.i1584, %43, !dbg !6959
  %_3.i.i1542 = fcmp ule float %_22.i, 0x3E45798EE0000000, !dbg !6962
  %_6.i.i1544 = bitcast float %_22.i to i32, !dbg !6968
  %_4.i.i1549 = select i1 %_3.i.i1542, i32 841731191, i32 %_6.i.i1544, !dbg !6971
  %_0.i.i1550 = bitcast i32 %_4.i.i1549 to float, !dbg !6972
  %_3.i.i = fcmp ule float %_0.i.i1550, 0x3810000000000000, !dbg !6974
  %_4.i.i = select i1 %_3.i.i, i32 8388608, i32 %_4.i.i1549, !dbg !6979
  %_5.i1108 = and i32 %_4.i.i, 8388607, !dbg !6981
  %_4.i1109 = or disjoint i32 %_5.i1108, 1065353216, !dbg !6981
  %significand.i = bitcast i32 %_4.i1109 to float, !dbg !6983
  %_0.i926 = fadd float %significand.i, -1.000000e+00, !dbg !6985
  %_0.i871 = fmul float %_0.i926, 0xBF9B17A960000000, !dbg !6987
  %_0.i829 = fadd float %_0.i871, 0x3FBF9A8440000000, !dbg !6989
  %_0.i871.1 = fmul float %_0.i926, %_0.i829, !dbg !6987
  %_0.i829.1 = fadd float %_0.i871.1, 0xBFD1E3F400000000, !dbg !6989
  %_0.i871.2 = fmul float %_0.i926, %_0.i829.1, !dbg !6987
  %_0.i829.2 = fadd float %_0.i871.2, 0x3FDD544F20000000, !dbg !6989
  %_0.i871.3 = fmul float %_0.i926, %_0.i829.2, !dbg !6987
  %_0.i829.3 = fadd float %_0.i871.3, 0xBFE6FC2A60000000, !dbg !6989
  %_0.i871.4 = fmul float %_0.i926, %_0.i829.3, !dbg !6987
  %_0.i829.4 = fadd float %_0.i871.4, 0x3FF714B2A0000000, !dbg !6989
  %_9.i1110 = lshr i32 %_4.i.i, 23, !dbg !6991
  %_8.i = or disjoint i32 %_9.i1110, 1258291200, !dbg !6991
  %_7.i1111 = bitcast i32 %_8.i to float, !dbg !6992
  %exponent.i = fadd float %_7.i1111, 0xC160000FE0000000, !dbg !6994
  %_0.i870 = fmul float %_0.i926, %_0.i829.4, !dbg !6995
  %_0.i828 = fadd float %exponent.i, %_0.i870, !dbg !6997
  %_0.i921 = fmul float %_0.i828, 0x4018151820000000, !dbg !6999
  %_3.i.i1533.inv = fcmp ogt float %_0.i921, -1.600000e+02, !dbg !7001
  %_0.i.i1541 = select i1 %_3.i.i1533.inv, float %_0.i921, float -1.600000e+02, !dbg !7001
  %_3.i.i1664.inv = fcmp olt float %_0.i.i1541, 2.400000e+01, !dbg !7004
  %_0.i.i1672 = select i1 %_3.i.i1664.inv, float %_0.i.i1541, float 2.400000e+01, !dbg !7004
  %_0.i934 = fsub float %_0.i.i1672, %_0.i1037, !dbg !7007
  %_3.i796 = fcmp ule float %_0.i934, %_0.i1033, !dbg !7010
  %44 = fneg float %_0.i1033, !dbg !7012
  %_0.i844 = fadd float %_0.i1033, %_0.i934, !dbg !7014
  %_0.i892 = fmul float %_0.i844, %_0.i844, !dbg !7016
  %_0.i891 = fmul float %_0.i1031, %_0.i892, !dbg !7018
  %_4.i1208.v.v = select i1 %_3.i796, float %_0.i891, float %_0.i934, !dbg !7020
  %_4.i1208.v = fmul float %_0.i1035, %_4.i1208.v.v, !dbg !7020
  %_4.i1208 = bitcast float %_4.i1208.v to i32, !dbg !7020
  %45 = fcmp ugt float %_0.i934, %44, !dbg !7022
  %_7.i1201 = select i1 %45, i32 %_4.i1208, i32 0, !dbg !7024
  %_0.i1203 = bitcast i32 %_7.i1201 to float, !dbg !7025
  %_3.i.i1524 = fcmp ule float %_0.i1203, -1.000000e+02, !dbg !7027
  %46 = bitcast i32 %_7.i1201 to float, !dbg !7030
  %_0.i.i1532 = select i1 %_3.i.i1524, float -1.000000e+02, float %46, !dbg !7033
  %_3.i.i1655 = fcmp olt float %_0.i.i1532, 0.000000e+00, !dbg !7034
  %_0.i.i1663 = select i1 %_3.i.i1655, float %_0.i.i1532, float 0.000000e+00, !dbg !7037
  %_6.i334 = load float, ptr %_73.i107, align 4, !dbg !7039, !alias.scope !7042, !noalias !7045, !noundef !11
  %_3.i826 = fcmp uge float %_0.i.i1663, %_6.i334, !dbg !7047
  %_4.i1275 = select i1 %_3.i826, i32 %_0.i10271793, i32 %_0.i10291792, !dbg !7049
  %_0.i1276 = bitcast i32 %_4.i1275 to float, !dbg !7051
  %_0.i941 = fsub float %_0.i.i1663, %_6.i334, !dbg !7053
  %_4.i860 = fmul float %_0.i941, %_0.i1276, !dbg !7056
  %_0.i861 = fadd float %_6.i334, %_4.i860, !dbg !7056
  %47 = tail call noundef float @llvm.fabs.f32(float %_0.i861), !dbg !7058
  %48 = fcmp uge float %47, 0x3BC79CA100000000, !dbg !7061
  %_0.i1197 = select i1 %48, float %_0.i861, float 0.000000e+00, !dbg !7063
  store float %_0.i1197, ptr %_73.i107, align 4, !dbg !7064, !alias.scope !7042, !noalias !7045
  %_0.i851 = fadd float %_0.i1041, %_0.i1197, !dbg !7065
  %_0.i917 = fmul float %_0.i851, 0x3FC542A5A0000000, !dbg !7069
  %_3.i.i1412.inv = fcmp ogt float %_0.i917, -1.260000e+02, !dbg !7072
  %_0.i.i1419 = select i1 %_3.i.i1412.inv, float %_0.i917, float -1.260000e+02, !dbg !7072
  %_3.i.i1569.inv = fcmp olt float %_0.i.i1419, 1.270000e+02, !dbg !7076
  %_0.i.i1576 = select i1 %_3.i.i1569.inv, float %_0.i.i1419, float 1.270000e+02, !dbg !7076
  %49 = tail call noundef float @llvm.floor.f32(float %_0.i.i1576), !dbg !7079
  %_0.i930 = fsub float %_0.i.i1576, %49, !dbg !7083
  %_0.i880 = fmul float %_0.i930, 0x3F5E974FA0000000, !dbg !7085
  %_0.i837 = fadd float %_0.i880, 0x3F82778560000000, !dbg !7090
  %_0.i880.1 = fmul float %_0.i930, %_0.i837, !dbg !7085
  %_0.i837.1 = fadd float %_0.i880.1, 0x3FAC91CE60000000, !dbg !7090
  %_0.i880.2 = fmul float %_0.i930, %_0.i837.1, !dbg !7085
  %_0.i837.2 = fadd float %_0.i880.2, 0x3FCEBDB560000000, !dbg !7090
  %_0.i880.3 = fmul float %_0.i930, %_0.i837.2, !dbg !7085
  %_0.i837.3 = fadd float %_0.i880.3, 0x3FE62E4BA0000000, !dbg !7090
  %_0.i883 = fmul float %_0.i931, 0x3F5E974FA0000000, !dbg !7092
  %_0.i839 = fadd float %_0.i883, 0x3F82778560000000, !dbg !7094
  %_0.i883.1 = fmul float %_0.i931, %_0.i839, !dbg !7092
  %_0.i839.1 = fadd float %_0.i883.1, 0x3FAC91CE60000000, !dbg !7094
  %_0.i883.2 = fmul float %_0.i931, %_0.i839.1, !dbg !7092
  %_0.i839.2 = fadd float %_0.i883.2, 0x3FCEBDB560000000, !dbg !7094
  %_0.i883.3 = fmul float %_0.i931, %_0.i839.2, !dbg !7092
  %_0.i839.3 = fadd float %_0.i883.3, 0x3FE62E4BA0000000, !dbg !7094
  %_0.i882 = fmul float %_0.i931, %_0.i839.3, !dbg !7096
  %_0.i838 = fadd float %_0.i882, 1.000000e+00, !dbg !7098
  %biased.i741 = fadd float %43, 0x4160000FE0000000, !dbg !7100
  %_4.i742 = bitcast float %biased.i741 to i32, !dbg !7104
  %_3.i743 = shl i32 %_4.i742, 23, !dbg !7108
  %_0.i744 = bitcast i32 %_3.i743 to float, !dbg !7109
  %_0.i881 = fmul float %_0.i838, %_0.i744, !dbg !7112
  %_0.i913 = fmul float %_0.i1055, %_0.i881, !dbg !7114
  %_0.i944 = fsub float %_0.i913, %_0.i1055, !dbg !7117
  %_4.i866 = fmul float %_0.i1023, %_0.i944, !dbg !7123
  %_0.i867 = fadd float %_0.i1055, %_4.i866, !dbg !7123
  %_3.i792 = fcmp oeq float %_0.i1193, 0.000000e+00, !dbg !7126
  %_0.i13851808 = and i1 %_3.i776, %_3.i792, !dbg !7129
  %_0.i13801809 = or i1 %_3.i778, %_0.i13851808, !dbg !7132
  %_0.i13791810 = or i1 %bypass, %_0.i13801809, !dbg !7135
  %_4.i1317.v = select i1 %_3.i780, float %_0.i867, float %_0.i913, !dbg !7137
  %_4.i1310.v = select i1 %_0.i13791810, float %_0.i1055, float %_4.i1317.v, !dbg !7140
  %_0.i879 = fmul float %_0.i930, %_0.i837.3, !dbg !7143
  %_0.i836 = fadd float %_0.i879, 1.000000e+00, !dbg !7145
  %biased.i = fadd float %49, 0x4160000FE0000000, !dbg !7147
  %_4.i738 = bitcast float %biased.i to i32, !dbg !7149
  %_3.i739 = shl i32 %_4.i738, 23, !dbg !7151
  %_0.i740 = bitcast i32 %_3.i739 to float, !dbg !7152
  %_0.i878 = fmul float %_0.i836, %_0.i740, !dbg !7154
  %_0.i916 = fmul float %_0.i1051, %_0.i878, !dbg !7156
  %_0.i945 = fsub float %_0.i916, %_0.i1051, !dbg !7158
  %_4.i868 = fmul float %_0.i1039, %_0.i945, !dbg !7161
  %_0.i869 = fadd float %_0.i1051, %_4.i868, !dbg !7161
  %_3.i794 = fcmp oeq float %_0.i1197, 0.000000e+00, !dbg !7163
  %_0.i13861822 = and i1 %_3.i782, %_3.i794, !dbg !7165
  %_0.i13821823 = or i1 %_3.i784, %_0.i13861822, !dbg !7167
  %_0.i13811824 = or i1 %bypass, %_0.i13821823, !dbg !7169
  %_4.i1331.v = select i1 %_3.i786, float %_0.i869, float %_0.i916, !dbg !7171
  %_4.i1324.v = select i1 %_0.i13811824, float %_0.i1051, float %_4.i1331.v, !dbg !7173
  store float %_4.i1310.v, ptr %_96.i23, align 4, !dbg !7175, !alias.scope !7178, !noalias !6406
  store float %_4.i1324.v, ptr %_104.i28, align 4, !dbg !7181, !alias.scope !7183, !noalias !6406
  %50 = trunc i64 %spec.store.select.i62 to i32, !dbg !7186
  store i32 %50, ptr %19, align 4, !dbg !7186, !alias.scope !6341, !noalias !6350
  store i32 %50, ptr %29, align 4, !dbg !7187, !alias.scope !6344, !noalias !6682
  %exitcond2516.not = icmp eq i64 %32, %spec.store.select, !dbg !7188
  br i1 %exitcond2516.not, label %bb10, label %bb27.i19, !dbg !6389

bb43.i111:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1057
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %spec.store.select.i62, i64 noundef %_170.1.i94, i64 noundef %_170.1.i94, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_714be7237772529829a33eae50e95afa) #23, !dbg !7191, !noalias !6406
  unreachable, !dbg !7191

bb10:                                             ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit, %start
  br i1 %_24, label %bb11, label %bb19, !dbg !7192

bb11:                                             ; preds = %bb10
  %_34 = sub i64 %frames, %spec.store.select, !dbg !7193
  %51 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1472, !dbg !7194
  %channels.0.val = load i32, ptr %51, align 4, !dbg !7194
  %52 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1472, !dbg !7194
  %channels.1.val = load i32, ptr %52, align 4, !dbg !7194
  %_4.i1703 = icmp ult i64 %_34, 129, !dbg !7195
  %_0.i4.i = zext i32 %channels.0.val to i64
  %_5.not.i = icmp samesign ule i64 %_34, %_0.i4.i
  %or.cond.i.not1827 = select i1 %_4.i1703, i1 %_5.not.i, i1 false, !dbg !7195
  %_0.i.i1704 = zext i32 %channels.1.val to i64
  %53 = icmp samesign ule i64 %_34, %_0.i.i1704
  %or.cond = select i1 %or.cond.i.not1827, i1 %53, i1 false, !dbg !7195
  br i1 %or.cond, label %bb13, label %bb27.i.lr.ph, !dbg !7195

bb19:                                             ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit230, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit373.i, %bb37.thread.i, %bb10
  ret void, !dbg !7198

bb27.i.lr.ph:                                     ; preds = %bb11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7199), !dbg !7202
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7203), !dbg !7202
  %54 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1512, !dbg !7205
  %_12.i = load i32, ptr %54, align 8, !dbg !7205, !alias.scope !7199, !noalias !7209, !noundef !11
  %_15.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 288, !dbg !7213
  %_4.i714 = getelementptr inbounds nuw i8, ptr %channels.0, i64 480, !dbg !7216
  %_0.i961 = load float, ptr %_4.i714, align 4, !dbg !7218, !alias.scope !7220, !noalias !7223, !noundef !11
  %_7.i716 = getelementptr inbounds nuw i8, ptr %channels.0, i64 512, !dbg !7226
  %_0.i959 = load float, ptr %_7.i716, align 4, !dbg !7227, !alias.scope !7229, !noalias !7223, !noundef !11
  %_0.i957 = load float, ptr %_15.i, align 4, !dbg !7232, !alias.scope !7234, !noalias !7223, !noundef !11
  %_14.i719 = getelementptr inbounds nuw i8, ptr %channels.0, i64 320, !dbg !7237
  %_0.i955 = load float, ptr %_14.i719, align 4, !dbg !7238, !alias.scope !7240, !noalias !7223, !noundef !11
  %_17.i721 = getelementptr inbounds nuw i8, ptr %channels.0, i64 352, !dbg !7243
  %_0.i953 = load float, ptr %_17.i721, align 4, !dbg !7244, !alias.scope !7246, !noalias !7223, !noundef !11
  %_20.i723 = getelementptr inbounds nuw i8, ptr %channels.0, i64 384, !dbg !7249
  %_0.i951 = load float, ptr %_20.i723, align 4, !dbg !7250, !alias.scope !7252, !noalias !7223, !noundef !11
  %_23.i725 = getelementptr inbounds nuw i8, ptr %channels.0, i64 416, !dbg !7255
  %_0.i9491828 = load i32, ptr %_23.i725, align 4, !dbg !7256, !alias.scope !7258, !noalias !7223, !noundef !11
  %_26.i727 = getelementptr inbounds nuw i8, ptr %channels.0, i64 448, !dbg !7261
  %_0.i9471829 = load i32, ptr %_26.i727, align 4, !dbg !7262, !alias.scope !7264, !noalias !7223, !noundef !11
  %_17.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 288, !dbg !7267
  %_4.i690 = getelementptr inbounds nuw i8, ptr %channels.1, i64 480, !dbg !7269
  %_0.i977 = load float, ptr %_4.i690, align 4, !dbg !7271, !alias.scope !7273, !noalias !7276, !noundef !11
  %_7.i692 = getelementptr inbounds nuw i8, ptr %channels.1, i64 512, !dbg !7279
  %_0.i975 = load float, ptr %_7.i692, align 4, !dbg !7280, !alias.scope !7282, !noalias !7276, !noundef !11
  %_0.i973 = load float, ptr %_17.i, align 4, !dbg !7285, !alias.scope !7287, !noalias !7276, !noundef !11
  %_14.i695 = getelementptr inbounds nuw i8, ptr %channels.1, i64 320, !dbg !7290
  %_0.i971 = load float, ptr %_14.i695, align 4, !dbg !7291, !alias.scope !7293, !noalias !7276, !noundef !11
  %_17.i697 = getelementptr inbounds nuw i8, ptr %channels.1, i64 352, !dbg !7296
  %_0.i969 = load float, ptr %_17.i697, align 4, !dbg !7297, !alias.scope !7299, !noalias !7276, !noundef !11
  %_20.i699 = getelementptr inbounds nuw i8, ptr %channels.1, i64 384, !dbg !7302
  %_0.i967 = load float, ptr %_20.i699, align 4, !dbg !7303, !alias.scope !7305, !noalias !7276, !noundef !11
  %_23.i701 = getelementptr inbounds nuw i8, ptr %channels.1, i64 416, !dbg !7308
  %_0.i9651830 = load i32, ptr %_23.i701, align 4, !dbg !7309, !alias.scope !7311, !noalias !7276, !noundef !11
  %_26.i703 = getelementptr inbounds nuw i8, ptr %channels.1, i64 448, !dbg !7314
  %_0.i9631831 = load i32, ptr %_26.i703, align 4, !dbg !7315, !alias.scope !7317, !noalias !7276, !noundef !11
  %ring_length.i = zext i32 %_12.i to i64, !dbg !7205
  %55 = icmp eq i32 %link, 1, !dbg !7320
  %.not1833 = icmp eq i32 %link, 3, !dbg !7322
  %_3.i756 = fcmp une float %_0.i959, 1.000000e+00, !dbg !7323
  %_3.i754 = fcmp oeq float %_0.i959, 0.000000e+00, !dbg !7325
  %_3.i753 = fcmp oeq float %_0.i961, 0.000000e+00, !dbg !7327
  %_3.i762 = fcmp une float %_0.i975, 1.000000e+00, !dbg !7329
  %_3.i760 = fcmp oeq float %_0.i975, 0.000000e+00, !dbg !7331
  %_3.i758 = fcmp oeq float %_0.i977, 0.000000e+00, !dbg !7333
  %_6.i.i = load i64, ptr %detector, align 8, !range !220
  %56 = getelementptr inbounds nuw i8, ptr %detector, i64 16
  %sidechain_left.1.i.i = load i64, ptr %56, align 8
  %57 = getelementptr inbounds nuw i8, ptr %detector, i64 24
  %sidechain_right.0.i.i = load ptr, ptr %57, align 8, !nonnull !11, !align !3663
  %58 = getelementptr inbounds nuw i8, ptr %detector, i64 32
  %sidechain_right.1.i.i = load i64, ptr %58, align 8
  %59 = getelementptr inbounds nuw i8, ptr %detector, i64 8
  %sidechain_left.0.i.i = load ptr, ptr %59, align 8, !nonnull !11, !align !3663
  %60 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1508
  %61 = getelementptr inbounds nuw i8, ptr %channels.0, i64 8
  %62 = getelementptr inbounds nuw i8, ptr %channels.0, i64 24
  %63 = getelementptr inbounds nuw i8, ptr %channels.0, i64 16
  %64 = getelementptr inbounds nuw i8, ptr %channels.1, i64 8
  %65 = getelementptr inbounds nuw i8, ptr %channels.1, i64 24
  %66 = getelementptr inbounds nuw i8, ptr %channels.1, i64 16
  %67 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1512
  %_69.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 1504
  %68 = fneg float %_0.i953
  %_73.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 1504
  %69 = fneg float %_0.i969
  %70 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1508
  br label %bb27.i, !dbg !7335

bb27.i:                                           ; preds = %bb27.i.lr.ph, %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit230
  %start1.sroa.0.0.i2188 = phi i64 [ %spec.store.select, %bb27.i.lr.ph ], [ %71, %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit230 ]
  %71 = add nuw nsw i64 %start1.sroa.0.0.i2188, 1, !dbg !7344
  %_89.i = icmp samesign ugt i64 %start1.sroa.0.0.i2188, %left.1, !dbg !7350
  br i1 %_89.i, label %bb29.i, label %bb30.i, !dbg !7350, !prof !161

bb30.i:                                           ; preds = %bb27.i
  %_96.i = getelementptr inbounds nuw float, ptr %left.0, i64 %start1.sroa.0.0.i2188, !dbg !7357
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7362), !dbg !7365
  %_3.not.i1103 = icmp eq i64 %left.1, %start1.sroa.0.0.i2188, !dbg !7366
  br i1 %_3.not.i1103, label %panic.i1106, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1107, !dbg !7366

panic.i1106:                                      ; preds = %bb30.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !7366, !noalias !7368
  unreachable, !dbg !7366

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1107: ; preds = %bb30.i
  %_0.i1105 = load float, ptr %_96.i, align 4, !dbg !7366, !alias.scope !7362, !noalias !7369, !noundef !11
  %_97.i = icmp samesign ugt i64 %start1.sroa.0.0.i2188, %right.1, !dbg !7370
  br i1 %_97.i, label %bb31.i, label %bb32.i, !dbg !7370, !prof !161

bb29.i:                                           ; preds = %bb27.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %start1.sroa.0.0.i2188, i64 noundef range(i64 0, 2305843009213693952) %left.1, i64 noundef range(i64 0, 2305843009213693952) %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c00de08b5cdb434b6310222ddd96a0fc) #23, !dbg !7375, !noalias !7369
  unreachable, !dbg !7375

bb32.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1107
  %_104.i = getelementptr inbounds nuw float, ptr %right.0, i64 %start1.sroa.0.0.i2188, !dbg !7376
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7381), !dbg !7384
  %_3.not.i1098 = icmp eq i64 %right.1, %start1.sroa.0.0.i2188, !dbg !7385
  br i1 %_3.not.i1098, label %panic.i1101, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1102, !dbg !7385

panic.i1101:                                      ; preds = %bb32.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !7385, !noalias !7387
  unreachable, !dbg !7385

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1102: ; preds = %bb32.i
  %_0.i1100 = load float, ptr %_104.i, align 4, !dbg !7385, !alias.scope !7381, !noalias !7369, !noundef !11
  switch i64 %_6.i.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1102.unreachabledefault [
    i64 0, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i
    i64 1, label %bb3.i.i
    i64 2, label %bb2.i.i
  ], !dbg !7388

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1102.unreachabledefault: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1102
  unreachable

default.unreachable:                              ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit365.i
  unreachable

bb3.i.i:                                          ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1102
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i, !dbg !7391

bb2.i.i:                                          ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1102
  %_25.i.i = icmp ugt i64 %start1.sroa.0.0.i2188, %sidechain_left.1.i.i, !dbg !7392
  br i1 %_25.i.i, label %bb17.i.i, label %bb18.i.i, !dbg !7392, !prof !161

bb18.i.i:                                         ; preds = %bb2.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7395), !dbg !7398
  %_3.not.i1093 = icmp eq i64 %sidechain_left.1.i.i, %start1.sroa.0.0.i2188, !dbg !7399
  br i1 %_3.not.i1093, label %panic.i1096, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1097, !dbg !7399

panic.i1096:                                      ; preds = %bb18.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !7399, !noalias !7401
  unreachable, !dbg !7399

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1097: ; preds = %bb18.i.i
  %_32.i.i = getelementptr inbounds nuw float, ptr %sidechain_left.0.i.i, i64 %start1.sroa.0.0.i2188, !dbg !7405
  %_0.i1095 = load float, ptr %_32.i.i, align 4, !dbg !7399, !alias.scope !7395, !noalias !7407, !noundef !11
  %_33.i.i = icmp ugt i64 %start1.sroa.0.0.i2188, %sidechain_right.1.i.i, !dbg !7408
  br i1 %_33.i.i, label %bb19.i.i, label %bb20.i.i, !dbg !7408, !prof !161

bb17.i.i:                                         ; preds = %bb2.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 2305843009213693952) %start1.sroa.0.0.i2188, i64 noundef %sidechain_left.1.i.i, i64 noundef %sidechain_left.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a0a4023c361dd50869bbb08b76482f50) #23, !dbg !7411, !noalias !7407
  unreachable, !dbg !7411

bb20.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1097
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7412), !dbg !7415
  %_3.not.i1088 = icmp eq i64 %sidechain_right.1.i.i, %start1.sroa.0.0.i2188, !dbg !7416
  br i1 %_3.not.i1088, label %panic.i1091, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1092, !dbg !7416

panic.i1091:                                      ; preds = %bb20.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !7416, !noalias !7418
  unreachable, !dbg !7416

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1092: ; preds = %bb20.i.i
  %_40.i.i = getelementptr inbounds nuw float, ptr %sidechain_right.0.i.i, i64 %start1.sroa.0.0.i2188, !dbg !7419
  %_0.i1090 = load float, ptr %_40.i.i, align 4, !dbg !7416, !alias.scope !7412, !noalias !7407, !noundef !11
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i, !dbg !7421

bb19.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1097
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 2305843009213693952) %start1.sroa.0.0.i2188, i64 noundef %sidechain_right.1.i.i, i64 noundef %sidechain_right.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5807a2a07e9839e3d32631b4243eb17f) #23, !dbg !7422, !noalias !7407
  unreachable, !dbg !7422

_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1102, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1092, %bb3.i.i
  %main_right.sroa.0.0.i.i = phi float [ %_0.i1100, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1102 ], [ 0.000000e+00, %bb3.i.i ], [ %_0.i1090, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1092 ]
  %main_left.sroa.0.0.i.i = phi float [ %_0.i1105, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1102 ], [ 0.000000e+00, %bb3.i.i ], [ %_0.i1095, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1092 ]
  %72 = tail call noundef float @llvm.fabs.f32(float %main_left.sroa.0.0.i.i), !dbg !7423
  %73 = tail call noundef float @llvm.fabs.f32(float %main_right.sroa.0.0.i.i), !dbg !7425
  %_3.i.i1560 = fcmp ule float %72, %73, !dbg !7427
  %_6.i.i1562 = bitcast float %72 to i32, !dbg !7430
  %_8.i.i1564 = bitcast float %73 to i32, !dbg !7433
  %_4.i.i1567 = select i1 %_3.i.i1560, i32 %_8.i.i1564, i32 %_6.i.i1562, !dbg !7435
  %_0.i925 = fmul float %72, 5.000000e-01, !dbg !7436
  %_0.i924 = fmul float %73, 5.000000e-01, !dbg !7438
  %_0.i853 = fadd float %_0.i924, %_0.i925, !dbg !7440
  %_6.i1368 = bitcast float %_0.i853 to i32, !dbg !7442
  %_4.i1373 = select i1 %.not1833, i32 %_6.i1368, i32 %_4.i.i1567, !dbg !7445
  %_4.i1366 = select i1 %55, i32 %_6.i.i1562, i32 %_4.i1373, !dbg !7446
  %_4.i1359 = select i1 %55, i32 %_8.i.i1564, i32 %_4.i1373, !dbg !7448
  %_37.i = load i32, ptr %60, align 4, !dbg !7450, !alias.scope !7199, !noalias !7209, !noundef !11
  %write.i = zext i32 %_37.i to i64, !dbg !7450
  %74 = add nuw nsw i64 %write.i, 1, !dbg !7452
  %_39.i = icmp eq i64 %74, %ring_length.i, !dbg !7454
  %spec.store.select.i = select i1 %_39.i, i64 0, i64 %74, !dbg !7454
  %_165.1.i = load i64, ptr %61, align 8, !dbg !7456, !alias.scope !7199, !noalias !7209, !noundef !11
  %_105.i = icmp ult i64 %_165.1.i, %write.i, !dbg !7458
  br i1 %_105.i, label %bb33.i, label %bb34.i, !dbg !7458, !prof !161

bb31.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1107
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %start1.sroa.0.0.i2188, i64 noundef range(i64 0, 2305843009213693952) %right.1, i64 noundef range(i64 0, 2305843009213693952) %right.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_99a388bbc796e7fd7b28fa01c6ed4e6b) #23, !dbg !7463, !noalias !7369
  unreachable, !dbg !7463

bb34.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i
  %_165.0.i = load ptr, ptr %channels.0, align 8, !dbg !7456, !alias.scope !7199, !noalias !7209, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7464), !dbg !7467
  %_4.not.i1178 = icmp eq i64 %_165.1.i, %write.i, !dbg !7468
  br i1 %_4.not.i1178, label %panic.i1180, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1181, !dbg !7468

panic.i1180:                                      ; preds = %bb34.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #23, !dbg !7468, !noalias !7470
  unreachable, !dbg !7468

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1181: ; preds = %bb34.i
  %_112.i = getelementptr inbounds nuw float, ptr %_165.0.i, i64 %write.i, !dbg !7471
  store float %_0.i1105, ptr %_112.i, align 4, !dbg !7468, !alias.scope !7464, !noalias !7369
  %_166.1.i = load i64, ptr %62, align 8, !dbg !7476, !alias.scope !7199, !noalias !7209, !noundef !11
  %_113.i = icmp ult i64 %_166.1.i, %write.i, !dbg !7477
  br i1 %_113.i, label %bb35.i, label %bb36.i, !dbg !7477, !prof !161

bb33.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i, i64 noundef %_165.1.i, i64 noundef %_165.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7fd19a98f6b7f6813fdc6b43fefd82a4) #23, !dbg !7481, !noalias !7369
  unreachable, !dbg !7481

bb36.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1181
  %_166.0.i = load ptr, ptr %63, align 8, !dbg !7476, !alias.scope !7199, !noalias !7209, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7482), !dbg !7485
  %_4.not.i1174 = icmp eq i64 %_166.1.i, %write.i, !dbg !7486
  br i1 %_4.not.i1174, label %panic.i1176, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1177, !dbg !7486

panic.i1176:                                      ; preds = %bb36.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #23, !dbg !7486, !noalias !7488
  unreachable, !dbg !7486

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1177: ; preds = %bb36.i
  %_120.i = getelementptr inbounds nuw float, ptr %_166.0.i, i64 %write.i, !dbg !7489
  store i32 %_4.i1366, ptr %_120.i, align 4, !dbg !7486, !alias.scope !7482, !noalias !7369
  %_167.1.i = load i64, ptr %64, align 8, !dbg !7494, !alias.scope !7203, !noalias !7495, !noundef !11
  %_121.i = icmp ult i64 %_167.1.i, %write.i, !dbg !7496
  br i1 %_121.i, label %bb37.i, label %bb38.i, !dbg !7496, !prof !161

bb35.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1181
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i, i64 noundef %_166.1.i, i64 noundef %_166.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_3898a5b9e77e549bf29e2ec14b43830a) #23, !dbg !7500, !noalias !7369
  unreachable, !dbg !7500

bb38.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1177
  %_167.0.i = load ptr, ptr %channels.1, align 8, !dbg !7494, !alias.scope !7203, !noalias !7495, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7501), !dbg !7504
  %_4.not.i1170 = icmp eq i64 %_167.1.i, %write.i, !dbg !7505
  br i1 %_4.not.i1170, label %panic.i1172, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1173, !dbg !7505

panic.i1172:                                      ; preds = %bb38.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #23, !dbg !7505, !noalias !7507
  unreachable, !dbg !7505

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1173: ; preds = %bb38.i
  %_128.i = getelementptr inbounds nuw float, ptr %_167.0.i, i64 %write.i, !dbg !7508
  store float %_0.i1100, ptr %_128.i, align 4, !dbg !7505, !alias.scope !7501, !noalias !7369
  %_168.1.i = load i64, ptr %65, align 8, !dbg !7513, !alias.scope !7203, !noalias !7495, !noundef !11
  %_129.i = icmp ult i64 %_168.1.i, %write.i, !dbg !7514
  br i1 %_129.i, label %bb39.i, label %bb40.i, !dbg !7514, !prof !161

bb37.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1177
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i, i64 noundef %_167.1.i, i64 noundef %_167.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ef27b7b7ba2d23db7b66788c02ac8976) #23, !dbg !7518, !noalias !7369
  unreachable, !dbg !7518

bb40.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1173
  %_168.0.i = load ptr, ptr %66, align 8, !dbg !7513, !alias.scope !7203, !noalias !7495, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7519), !dbg !7522
  %_4.not.i1166 = icmp eq i64 %_168.1.i, %write.i, !dbg !7523
  br i1 %_4.not.i1166, label %panic.i1168, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1169, !dbg !7523

panic.i1168:                                      ; preds = %bb40.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #23, !dbg !7523, !noalias !7525
  unreachable, !dbg !7523

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1169: ; preds = %bb40.i
  %_136.i = getelementptr inbounds nuw float, ptr %_168.0.i, i64 %write.i, !dbg !7526
  store i32 %_4.i1359, ptr %_136.i, align 4, !dbg !7523, !alias.scope !7519, !noalias !7369
  %_169.1.i = load i64, ptr %61, align 8, !dbg !7531, !alias.scope !7199, !noalias !7209, !noundef !11
  %_137.i = icmp ugt i64 %spec.store.select.i, %_169.1.i, !dbg !7532
  br i1 %_137.i, label %bb41.i, label %bb42.i, !dbg !7532, !prof !161

bb39.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1173
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i, i64 noundef %_168.1.i, i64 noundef %_168.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_23eb2a707ff31dba52171dada526a36f) #23, !dbg !7536, !noalias !7369
  unreachable, !dbg !7536

bb42.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1169
  %_169.0.i = load ptr, ptr %channels.0, align 8, !dbg !7531, !alias.scope !7199, !noalias !7209, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7537), !dbg !7540
  %_3.not.i1083 = icmp eq i64 %_169.1.i, %spec.store.select.i, !dbg !7541
  br i1 %_3.not.i1083, label %panic.i1086, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1087, !dbg !7541

panic.i1086:                                      ; preds = %bb42.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !7541, !noalias !7543
  unreachable, !dbg !7541

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1087: ; preds = %bb42.i
  %_144.i = getelementptr inbounds nuw float, ptr %_169.0.i, i64 %spec.store.select.i, !dbg !7544
  %_0.i1085 = load float, ptr %_144.i, align 4, !dbg !7541, !alias.scope !7537, !noalias !7369, !noundef !11
  %_170.1.i = load i64, ptr %64, align 8, !dbg !7549, !alias.scope !7203, !noalias !7495, !noundef !11
  %_145.i = icmp ugt i64 %spec.store.select.i, %_170.1.i, !dbg !7551
  br i1 %_145.i, label %bb43.i, label %bb44.i, !dbg !7551, !prof !161

bb41.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1169
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %spec.store.select.i, i64 noundef %_169.1.i, i64 noundef %_169.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e74b405e18c5e1dc98b750b7a797cfef) #23, !dbg !7555, !noalias !7369
  unreachable, !dbg !7555

bb44.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1087
  %_170.0.i = load ptr, ptr %channels.1, align 8, !dbg !7549, !alias.scope !7203, !noalias !7495, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7556), !dbg !7559
  %_3.not.i1078 = icmp eq i64 %_170.1.i, %spec.store.select.i, !dbg !7560
  br i1 %_3.not.i1078, label %panic.i1081, label %bb6.i243, !dbg !7560

panic.i1081:                                      ; preds = %bb44.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !7560, !noalias !7562
  unreachable, !dbg !7560

bb6.i243:                                         ; preds = %bb44.i
  %_152.i = getelementptr inbounds nuw float, ptr %_170.0.i, i64 %spec.store.select.i, !dbg !7563
  %_0.i1080 = load float, ptr %_152.i, align 4, !dbg !7560, !alias.scope !7556, !noalias !7369, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7568), !dbg !7571
  %_5.i232 = load i32, ptr %54, align 8, !dbg !7573, !alias.scope !7568, !noalias !7575, !noundef !11
  %_38.1.i251 = load i64, ptr %62, align 8
  %_17.i244 = load i32, ptr %51, align 4, !dbg !7577, !alias.scope !7568, !noalias !7575, !noundef !11
  %delay.i245 = zext i32 %_17.i244 to i64, !dbg !7577
  %_20.not.i246 = icmp ult i32 %_37.i, %_17.i244, !dbg !7578
  %narrow1837 = select i1 %_20.not.i246, i32 %_5.i232, i32 0, !dbg !7578
  %_21.i247 = zext i32 %narrow1837 to i64, !dbg !7578
  %tap.sroa.0.0.i249 = sub nsw i64 %write.i, %delay.i245, !dbg !7579
  %_23.i250 = add nsw i64 %tap.sroa.0.0.i249, %_21.i247, !dbg !7580
  %_26.i252 = icmp ult i64 %_23.i250, %_38.1.i251, !dbg !7581
  br i1 %_26.i252, label %bb6.i215, label %panic1.i253, !dbg !7581

panic1.i253:                                      ; preds = %bb6.i243
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_23.i250, i64 noundef %_38.1.i251, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a529a882edb544ae33c686afc6011081) #23, !dbg !7581, !noalias !7582
  unreachable, !dbg !7581

bb6.i215:                                         ; preds = %bb6.i243
  %_38.0.i255 = load ptr, ptr %63, align 8, !nonnull !11
  %75 = getelementptr inbounds nuw float, ptr %_38.0.i255, i64 %_23.i250, !dbg !7581
  %_22.i256 = load float, ptr %75, align 4, !dbg !7581, !noalias !7582, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7583), !dbg !7586
  %_5.i204 = load i32, ptr %67, align 8, !dbg !7588, !alias.scope !7583, !noalias !7590, !noundef !11
  %_38.1.i223 = load i64, ptr %65, align 8
  %_17.i216 = load i32, ptr %52, align 4, !dbg !7592, !alias.scope !7583, !noalias !7590, !noundef !11
  %delay.i217 = zext i32 %_17.i216 to i64, !dbg !7592
  %_20.not.i218 = icmp ult i32 %_37.i, %_17.i216, !dbg !7593
  %narrow1838 = select i1 %_20.not.i218, i32 %_5.i204, i32 0, !dbg !7593
  %_21.i219 = zext i32 %narrow1838 to i64, !dbg !7593
  %tap.sroa.0.0.i221 = sub nsw i64 %write.i, %delay.i217, !dbg !7594
  %_23.i222 = add nsw i64 %tap.sroa.0.0.i221, %_21.i219, !dbg !7595
  %_26.i224 = icmp ult i64 %_23.i222, %_38.1.i223, !dbg !7596
  br i1 %_26.i224, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit230, label %panic1.i225, !dbg !7596

panic1.i225:                                      ; preds = %bb6.i215
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_23.i222, i64 noundef %_38.1.i223, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a529a882edb544ae33c686afc6011081) #23, !dbg !7596, !noalias !7597
  unreachable, !dbg !7596

_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit230: ; preds = %bb6.i215
  %_38.0.i227 = load ptr, ptr %66, align 8, !nonnull !11
  %76 = getelementptr inbounds nuw float, ptr %_38.0.i227, i64 %_23.i222, !dbg !7596
  %_22.i228 = load float, ptr %76, align 4, !dbg !7596, !noalias !7597, !noundef !11
  %_3.i.i1461 = fcmp ule float %_22.i256, 0x3E45798EE0000000, !dbg !7598
  %_6.i.i1463 = bitcast float %_22.i256 to i32, !dbg !7605
  %_4.i.i1468 = select i1 %_3.i.i1461, i32 841731191, i32 %_6.i.i1463, !dbg !7608
  %_0.i.i1469 = bitcast i32 %_4.i.i1468 to float, !dbg !7609
  %_3.i.i1404 = fcmp ule float %_0.i.i1469, 0x3810000000000000, !dbg !7611
  %_4.i.i1410 = select i1 %_3.i.i1404, i32 8388608, i32 %_4.i.i1468, !dbg !7616
  %_5.i1129 = and i32 %_4.i.i1410, 8388607, !dbg !7618
  %_4.i1130 = or disjoint i32 %_5.i1129, 1065353216, !dbg !7618
  %significand.i1131 = bitcast i32 %_4.i1130 to float, !dbg !7620
  %_0.i929 = fadd float %significand.i1131, -1.000000e+00, !dbg !7622
  %_0.i877 = fmul float %_0.i929, 0xBF9B17A960000000, !dbg !7624
  %_0.i835 = fadd float %_0.i877, 0x3FBF9A8440000000, !dbg !7626
  %_0.i877.1 = fmul float %_0.i929, %_0.i835, !dbg !7624
  %_0.i835.1 = fadd float %_0.i877.1, 0xBFD1E3F400000000, !dbg !7626
  %_0.i877.2 = fmul float %_0.i929, %_0.i835.1, !dbg !7624
  %_0.i835.2 = fadd float %_0.i877.2, 0x3FDD544F20000000, !dbg !7626
  %_0.i877.3 = fmul float %_0.i929, %_0.i835.2, !dbg !7624
  %_0.i835.3 = fadd float %_0.i877.3, 0xBFE6FC2A60000000, !dbg !7626
  %_0.i877.4 = fmul float %_0.i929, %_0.i835.3, !dbg !7624
  %_0.i835.4 = fadd float %_0.i877.4, 0x3FF714B2A0000000, !dbg !7626
  %_9.i1132 = lshr i32 %_4.i.i1410, 23, !dbg !7628
  %_8.i1133 = or disjoint i32 %_9.i1132, 1258291200, !dbg !7628
  %_7.i1134 = bitcast i32 %_8.i1133 to float, !dbg !7629
  %exponent.i1135 = fadd float %_7.i1134, 0xC160000FE0000000, !dbg !7631
  %_0.i876 = fmul float %_0.i929, %_0.i835.4, !dbg !7632
  %_0.i834 = fadd float %exponent.i1135, %_0.i876, !dbg !7634
  %_0.i918 = fmul float %_0.i834, 0x4018151820000000, !dbg !7636
  %_3.i.i1452.inv = fcmp ogt float %_0.i918, -1.600000e+02, !dbg !7638
  %_0.i.i1460 = select i1 %_3.i.i1452.inv, float %_0.i918, float -1.600000e+02, !dbg !7638
  %_3.i.i1610.inv = fcmp olt float %_0.i.i1460, 2.400000e+01, !dbg !7641
  %_0.i.i1618 = select i1 %_3.i.i1610.inv, float %_0.i.i1460, float 2.400000e+01, !dbg !7641
  %_0.i937 = fsub float %_0.i.i1618, %_0.i957, !dbg !7644
  %_3.i802 = fcmp ule float %_0.i937, %_0.i953, !dbg !7647
  %_0.i847 = fadd float %_0.i953, %_0.i937, !dbg !7649
  %_0.i904 = fmul float %_0.i847, %_0.i847, !dbg !7651
  %_0.i903 = fmul float %_0.i951, %_0.i904, !dbg !7653
  %_4.i1247.v.v = select i1 %_3.i802, float %_0.i903, float %_0.i937, !dbg !7655
  %_4.i1247.v = fmul float %_0.i955, %_4.i1247.v.v, !dbg !7655
  %_4.i1247 = bitcast float %_4.i1247.v to i32, !dbg !7655
  %77 = fcmp ugt float %_0.i937, %68, !dbg !7657
  %_7.i1239 = select i1 %77, i32 %_4.i1247, i32 0, !dbg !7659
  %_0.i1241 = bitcast i32 %_7.i1239 to float, !dbg !7660
  %_3.i.i1444 = fcmp ule float %_0.i1241, -1.000000e+02, !dbg !7662
  %78 = bitcast i32 %_7.i1239 to float, !dbg !7665
  %_0.i.i1451 = select i1 %_3.i.i1444, float -1.000000e+02, float %78, !dbg !7668
  %_3.i.i1601 = fcmp olt float %_0.i.i1451, 0.000000e+00, !dbg !7669
  %_0.i.i1609 = select i1 %_3.i.i1601, float %_0.i.i1451, float 0.000000e+00, !dbg !7672
  %_6.i368 = load float, ptr %_69.i, align 4, !dbg !7674, !alias.scope !7677, !noalias !7680, !noundef !11
  %_3.i814 = fcmp uge float %_0.i.i1609, %_6.i368, !dbg !7682
  %_4.i1254 = select i1 %_3.i814, i32 %_0.i9471829, i32 %_0.i9491828, !dbg !7684
  %_0.i1255 = bitcast i32 %_4.i1254 to float, !dbg !7686
  %_0.i938 = fsub float %_0.i.i1609, %_6.i368, !dbg !7688
  %_4.i854 = fmul float %_0.i938, %_0.i1255, !dbg !7691
  %_0.i855 = fadd float %_6.i368, %_4.i854, !dbg !7691
  %79 = tail call noundef float @llvm.fabs.f32(float %_0.i855), !dbg !7693
  %80 = fcmp uge float %79, 0x3BC79CA100000000, !dbg !7696
  %_0.i1185 = select i1 %80, float %_0.i855, float 0.000000e+00, !dbg !7698
  store float %_0.i1185, ptr %_69.i, align 4, !dbg !7699, !alias.scope !7677, !noalias !7680
  %_0.i848 = fadd float %_0.i961, %_0.i1185, !dbg !7700
  %_0.i908 = fmul float %_0.i848, 0x3FC542A5A0000000, !dbg !7704
  %_3.i.i1436.inv = fcmp ogt float %_0.i908, -1.260000e+02, !dbg !7707
  %_0.i.i1443 = select i1 %_3.i.i1436.inv, float %_0.i908, float -1.260000e+02, !dbg !7707
  %_3.i.i1593.inv = fcmp olt float %_0.i.i1443, 1.270000e+02, !dbg !7711
  %_0.i.i1600 = select i1 %_3.i.i1593.inv, float %_0.i.i1443, float 1.270000e+02, !dbg !7711
  %81 = tail call noundef float @llvm.floor.f32(float %_0.i.i1600), !dbg !7714
  %_0.i933 = fsub float %_0.i.i1600, %81, !dbg !7718
  %_3.i.i1488 = fcmp ule float %_22.i228, 0x3E45798EE0000000, !dbg !7720
  %_6.i.i1490 = bitcast float %_22.i228 to i32, !dbg !7726
  %_4.i.i1495 = select i1 %_3.i.i1488, i32 841731191, i32 %_6.i.i1490, !dbg !7729
  %_0.i.i1496 = bitcast i32 %_4.i.i1495 to float, !dbg !7730
  %_3.i.i1396 = fcmp ule float %_0.i.i1496, 0x3810000000000000, !dbg !7732
  %_4.i.i1402 = select i1 %_3.i.i1396, i32 8388608, i32 %_4.i.i1495, !dbg !7737
  %_5.i1121 = and i32 %_4.i.i1402, 8388607, !dbg !7739
  %_4.i1122 = or disjoint i32 %_5.i1121, 1065353216, !dbg !7739
  %significand.i1123 = bitcast i32 %_4.i1122 to float, !dbg !7741
  %_0.i928 = fadd float %significand.i1123, -1.000000e+00, !dbg !7743
  %_0.i875 = fmul float %_0.i928, 0xBF9B17A960000000, !dbg !7745
  %_0.i833 = fadd float %_0.i875, 0x3FBF9A8440000000, !dbg !7747
  %_0.i875.1 = fmul float %_0.i928, %_0.i833, !dbg !7745
  %_0.i833.1 = fadd float %_0.i875.1, 0xBFD1E3F400000000, !dbg !7747
  %_0.i875.2 = fmul float %_0.i928, %_0.i833.1, !dbg !7745
  %_0.i833.2 = fadd float %_0.i875.2, 0x3FDD544F20000000, !dbg !7747
  %_0.i875.3 = fmul float %_0.i928, %_0.i833.2, !dbg !7745
  %_0.i833.3 = fadd float %_0.i875.3, 0xBFE6FC2A60000000, !dbg !7747
  %_0.i875.4 = fmul float %_0.i928, %_0.i833.3, !dbg !7745
  %_0.i833.4 = fadd float %_0.i875.4, 0x3FF714B2A0000000, !dbg !7747
  %_9.i1124 = lshr i32 %_4.i.i1402, 23, !dbg !7749
  %_8.i1125 = or disjoint i32 %_9.i1124, 1258291200, !dbg !7749
  %_7.i1126 = bitcast i32 %_8.i1125 to float, !dbg !7750
  %exponent.i1127 = fadd float %_7.i1126, 0xC160000FE0000000, !dbg !7752
  %_0.i874 = fmul float %_0.i928, %_0.i833.4, !dbg !7753
  %_0.i832 = fadd float %exponent.i1127, %_0.i874, !dbg !7755
  %_0.i919 = fmul float %_0.i832, 0x4018151820000000, !dbg !7757
  %_3.i.i1479.inv = fcmp ogt float %_0.i919, -1.600000e+02, !dbg !7759
  %_0.i.i1487 = select i1 %_3.i.i1479.inv, float %_0.i919, float -1.600000e+02, !dbg !7759
  %_3.i.i1628.inv = fcmp olt float %_0.i.i1487, 2.400000e+01, !dbg !7762
  %_0.i.i1636 = select i1 %_3.i.i1628.inv, float %_0.i.i1487, float 2.400000e+01, !dbg !7762
  %_0.i936 = fsub float %_0.i.i1636, %_0.i973, !dbg !7765
  %_3.i800 = fcmp ule float %_0.i936, %_0.i969, !dbg !7768
  %_0.i846 = fadd float %_0.i969, %_0.i936, !dbg !7770
  %_0.i900 = fmul float %_0.i846, %_0.i846, !dbg !7772
  %_0.i899 = fmul float %_0.i967, %_0.i900, !dbg !7774
  %_4.i1234.v.v = select i1 %_3.i800, float %_0.i899, float %_0.i936, !dbg !7776
  %_4.i1234.v = fmul float %_0.i971, %_4.i1234.v.v, !dbg !7776
  %_4.i1234 = bitcast float %_4.i1234.v to i32, !dbg !7776
  %82 = fcmp ugt float %_0.i936, %69, !dbg !7778
  %_7.i1226 = select i1 %82, i32 %_4.i1234, i32 0, !dbg !7780
  %_0.i1228 = bitcast i32 %_7.i1226 to float, !dbg !7781
  %_3.i.i1470 = fcmp ule float %_0.i1228, -1.000000e+02, !dbg !7783
  %83 = bitcast i32 %_7.i1226 to float, !dbg !7786
  %_0.i.i1478 = select i1 %_3.i.i1470, float -1.000000e+02, float %83, !dbg !7789
  %_3.i.i1619 = fcmp olt float %_0.i.i1478, 0.000000e+00, !dbg !7790
  %_0.i.i1627 = select i1 %_3.i.i1619, float %_0.i.i1478, float 0.000000e+00, !dbg !7793
  %_6.i356 = load float, ptr %_73.i, align 4, !dbg !7795, !alias.scope !7798, !noalias !7801, !noundef !11
  %_3.i818 = fcmp uge float %_0.i.i1627, %_6.i356, !dbg !7803
  %_4.i1261 = select i1 %_3.i818, i32 %_0.i9631831, i32 %_0.i9651830, !dbg !7805
  %_0.i1262 = bitcast i32 %_4.i1261 to float, !dbg !7807
  %_0.i939 = fsub float %_0.i.i1627, %_6.i356, !dbg !7809
  %_4.i856 = fmul float %_0.i939, %_0.i1262, !dbg !7812
  %_0.i857 = fadd float %_6.i356, %_4.i856, !dbg !7812
  %84 = tail call noundef float @llvm.fabs.f32(float %_0.i857), !dbg !7814
  %85 = fcmp uge float %84, 0x3BC79CA100000000, !dbg !7817
  %_0.i1189 = select i1 %85, float %_0.i857, float 0.000000e+00, !dbg !7819
  store float %_0.i1189, ptr %_73.i, align 4, !dbg !7820, !alias.scope !7798, !noalias !7801
  %_0.i849 = fadd float %_0.i977, %_0.i1189, !dbg !7821
  %_0.i911 = fmul float %_0.i849, 0x3FC542A5A0000000, !dbg !7825
  %_3.i.i1428.inv = fcmp ogt float %_0.i911, -1.260000e+02, !dbg !7828
  %_0.i.i1435 = select i1 %_3.i.i1428.inv, float %_0.i911, float -1.260000e+02, !dbg !7828
  %_3.i.i1585.inv = fcmp olt float %_0.i.i1435, 1.270000e+02, !dbg !7832
  %_0.i.i1592 = select i1 %_3.i.i1585.inv, float %_0.i.i1435, float 1.270000e+02, !dbg !7832
  %86 = tail call noundef float @llvm.floor.f32(float %_0.i.i1592), !dbg !7835
  %_0.i932 = fsub float %_0.i.i1592, %86, !dbg !7839
  %_0.i886 = fmul float %_0.i932, 0x3F5E974FA0000000, !dbg !7841
  %_0.i841 = fadd float %_0.i886, 0x3F82778560000000, !dbg !7843
  %_0.i886.1 = fmul float %_0.i932, %_0.i841, !dbg !7841
  %_0.i841.1 = fadd float %_0.i886.1, 0x3FAC91CE60000000, !dbg !7843
  %_0.i886.2 = fmul float %_0.i932, %_0.i841.1, !dbg !7841
  %_0.i841.2 = fadd float %_0.i886.2, 0x3FCEBDB560000000, !dbg !7843
  %_0.i886.3 = fmul float %_0.i932, %_0.i841.2, !dbg !7841
  %_0.i841.3 = fadd float %_0.i886.3, 0x3FE62E4BA0000000, !dbg !7843
  %_0.i889 = fmul float %_0.i933, 0x3F5E974FA0000000, !dbg !7845
  %_0.i843 = fadd float %_0.i889, 0x3F82778560000000, !dbg !7847
  %_0.i889.1 = fmul float %_0.i933, %_0.i843, !dbg !7845
  %_0.i843.1 = fadd float %_0.i889.1, 0x3FAC91CE60000000, !dbg !7847
  %_0.i889.2 = fmul float %_0.i933, %_0.i843.1, !dbg !7845
  %_0.i843.2 = fadd float %_0.i889.2, 0x3FCEBDB560000000, !dbg !7847
  %_0.i889.3 = fmul float %_0.i933, %_0.i843.2, !dbg !7845
  %_0.i843.3 = fadd float %_0.i889.3, 0x3FE62E4BA0000000, !dbg !7847
  %_0.i888 = fmul float %_0.i933, %_0.i843.3, !dbg !7849
  %_0.i842 = fadd float %_0.i888, 1.000000e+00, !dbg !7851
  %biased.i749 = fadd float %81, 0x4160000FE0000000, !dbg !7853
  %_4.i750 = bitcast float %biased.i749 to i32, !dbg !7855
  %_3.i751 = shl i32 %_4.i750, 23, !dbg !7857
  %_0.i752 = bitcast i32 %_3.i751 to float, !dbg !7858
  %_0.i887 = fmul float %_0.i842, %_0.i752, !dbg !7860
  %_0.i907 = fmul float %_0.i1085, %_0.i887, !dbg !7862
  %_0.i942 = fsub float %_0.i907, %_0.i1085, !dbg !7864
  %_4.i862 = fmul float %_0.i959, %_0.i942, !dbg !7867
  %_0.i863 = fadd float %_0.i1085, %_4.i862, !dbg !7867
  %_3.i788 = fcmp oeq float %_0.i1185, 0.000000e+00, !dbg !7869
  %_0.i13831848 = and i1 %_3.i753, %_3.i788, !dbg !7871
  %87 = or i1 %_3.i754, %_0.i13831848
  %_0.i13751850.reass.reass = or i1 %87, %bypass
  %_4.i1289.v = select i1 %_3.i756, float %_0.i863, float %_0.i907, !dbg !7873
  %_4.i1282.v = select i1 %_0.i13751850.reass.reass, float %_0.i1085, float %_4.i1289.v, !dbg !7875
  %_0.i885 = fmul float %_0.i932, %_0.i841.3, !dbg !7877
  %_0.i840 = fadd float %_0.i885, 1.000000e+00, !dbg !7879
  %biased.i745 = fadd float %86, 0x4160000FE0000000, !dbg !7881
  %_4.i746 = bitcast float %biased.i745 to i32, !dbg !7883
  %_3.i747 = shl i32 %_4.i746, 23, !dbg !7885
  %_0.i748 = bitcast i32 %_3.i747 to float, !dbg !7886
  %_0.i884 = fmul float %_0.i840, %_0.i748, !dbg !7888
  %_0.i910 = fmul float %_0.i1080, %_0.i884, !dbg !7890
  %_0.i943 = fsub float %_0.i910, %_0.i1080, !dbg !7892
  %_4.i864 = fmul float %_0.i975, %_0.i943, !dbg !7895
  %_0.i865 = fadd float %_0.i1080, %_4.i864, !dbg !7895
  %_3.i790 = fcmp oeq float %_0.i1189, 0.000000e+00, !dbg !7897
  %_0.i13841862 = and i1 %_3.i758, %_3.i790, !dbg !7899
  %88 = or i1 %_3.i760, %_0.i13841862
  %_0.i13771864.reass.reass = or i1 %88, %bypass
  %_4.i1303.v = select i1 %_3.i762, float %_0.i865, float %_0.i910, !dbg !7901
  %_4.i1296.v = select i1 %_0.i13771864.reass.reass, float %_0.i1080, float %_4.i1303.v, !dbg !7903
  store float %_4.i1282.v, ptr %_96.i, align 4, !dbg !7905, !alias.scope !7908, !noalias !7369
  store float %_4.i1296.v, ptr %_104.i, align 4, !dbg !7911, !alias.scope !7913, !noalias !7369
  %89 = trunc i64 %spec.store.select.i to i32, !dbg !7916
  store i32 %89, ptr %60, align 4, !dbg !7916, !alias.scope !7199, !noalias !7209
  store i32 %89, ptr %70, align 4, !dbg !7917, !alias.scope !7203, !noalias !7495
  %exitcond2517.not = icmp eq i64 %71, %frames, !dbg !7918
  br i1 %exitcond2517.not, label %bb19, label %bb27.i, !dbg !7335

bb43.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1087
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %spec.store.select.i, i64 noundef %_170.1.i, i64 noundef %_170.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_714be7237772529829a33eae50e95afa) #23, !dbg !7921, !noalias !7369
  unreachable, !dbg !7921

bb13:                                             ; preds = %bb11
  %_50.0 = load ptr, ptr %staged, align 8, !dbg !7922, !nonnull !11, !noundef !11
  %90 = getelementptr inbounds nuw i8, ptr %staged, i64 8, !dbg !7922
  %_50.1 = load i64, ptr %90, align 8, !dbg !7922, !noundef !11
  %91 = getelementptr inbounds nuw i8, ptr %staged, i64 16, !dbg !7924
  %_51.0 = load ptr, ptr %91, align 8, !dbg !7924, !nonnull !11, !noundef !11
  %92 = getelementptr inbounds nuw i8, ptr %staged, i64 24, !dbg !7924
  %_51.1 = load i64, ptr %92, align 8, !dbg !7924, !noundef !11
  %93 = getelementptr inbounds nuw i8, ptr %staged, i64 32, !dbg !7925
  %_52.0 = load ptr, ptr %93, align 8, !dbg !7925, !nonnull !11, !noundef !11
  %94 = getelementptr inbounds nuw i8, ptr %staged, i64 40, !dbg !7925
  %_52.1 = load i64, ptr %94, align 8, !dbg !7925, !noundef !11
  %95 = getelementptr inbounds nuw i8, ptr %staged, i64 48, !dbg !7926
  %_53.0 = load ptr, ptr %95, align 8, !dbg !7926, !nonnull !11, !noundef !11
  %96 = getelementptr inbounds nuw i8, ptr %staged, i64 56, !dbg !7926
  %_53.1 = load i64, ptr %96, align 8, !dbg !7926, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7927), !dbg !7930
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7931), !dbg !7930
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7933), !dbg !7930
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7935), !dbg !7930
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7937), !dbg !7930
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7939), !dbg !7930
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7941), !dbg !7930
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7943), !dbg !7930
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7945), !dbg !7930
  %97 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1512, !dbg !7947
  %_15.i1733 = load i32, ptr %97, align 8, !dbg !7947, !alias.scope !7935, !noalias !7951, !noundef !11
  %ring_length.i1734 = zext i32 %_15.i1733 to i64, !dbg !7947
  %98 = icmp eq i32 %link, 1, !dbg !7952
  %.not.i1735 = icmp eq i32 %link, 3, !dbg !7956
  %_19.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 288, !dbg !7957
  %_4.i203.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 480, !dbg !7959
  %_0.i331.i = load float, ptr %_4.i203.i, align 4, !dbg !7961, !alias.scope !7963, !noalias !7966, !noundef !11
  %_7.i205.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 512, !dbg !7969
  %_0.i330.i = load float, ptr %_7.i205.i, align 4, !dbg !7970, !alias.scope !7972, !noalias !7966, !noundef !11
  %_0.i329.i = load float, ptr %_19.i, align 4, !dbg !7975, !alias.scope !7977, !noalias !7966, !noundef !11
  %_14.i208.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 320, !dbg !7980
  %_0.i328.i = load float, ptr %_14.i208.i, align 4, !dbg !7981, !alias.scope !7983, !noalias !7966, !noundef !11
  %_17.i210.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 352, !dbg !7986
  %_0.i327.i = load float, ptr %_17.i210.i, align 4, !dbg !7987, !alias.scope !7989, !noalias !7966, !noundef !11
  %_20.i212.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 384, !dbg !7992
  %_0.i326.i = load float, ptr %_20.i212.i, align 4, !dbg !7993, !alias.scope !7995, !noalias !7966, !noundef !11
  %_23.i214.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 416, !dbg !7998
  %_0.i325750.i = load i32, ptr %_23.i214.i, align 4, !dbg !7999, !alias.scope !8001, !noalias !7966, !noundef !11
  %_26.i216.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 448, !dbg !8004
  %_0.i324751.i = load i32, ptr %_26.i216.i, align 4, !dbg !8005, !alias.scope !8007, !noalias !7966, !noundef !11
  %_3.i237.i = fcmp une float %_0.i330.i, 1.000000e+00, !dbg !8010
  %_3.i235.i = fcmp oeq float %_0.i330.i, 0.000000e+00, !dbg !8012
  %_3.i234.i = fcmp oeq float %_0.i331.i, 0.000000e+00, !dbg !8014
  %_21.i1736 = getelementptr inbounds nuw i8, ptr %channels.1, i64 288, !dbg !8016
  %_4.i189.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 480, !dbg !8018
  %_0.i339.i = load float, ptr %_4.i189.i, align 4, !dbg !8020, !alias.scope !8022, !noalias !8025, !noundef !11
  %_7.i190.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 512, !dbg !8028
  %_0.i338.i = load float, ptr %_7.i190.i, align 4, !dbg !8029, !alias.scope !8031, !noalias !8025, !noundef !11
  %_0.i337.i = load float, ptr %_21.i1736, align 4, !dbg !8034, !alias.scope !8036, !noalias !8025, !noundef !11
  %_14.i192.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 320, !dbg !8039
  %_0.i336.i = load float, ptr %_14.i192.i, align 4, !dbg !8040, !alias.scope !8042, !noalias !8025, !noundef !11
  %_17.i194.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 352, !dbg !8045
  %_0.i335.i = load float, ptr %_17.i194.i, align 4, !dbg !8046, !alias.scope !8048, !noalias !8025, !noundef !11
  %_20.i196.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 384, !dbg !8051
  %_0.i334.i = load float, ptr %_20.i196.i, align 4, !dbg !8052, !alias.scope !8054, !noalias !8025, !noundef !11
  %_23.i198.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 416, !dbg !8057
  %_0.i333752.i = load i32, ptr %_23.i198.i, align 4, !dbg !8058, !alias.scope !8060, !noalias !8025, !noundef !11
  %_26.i.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 448, !dbg !8063
  %_0.i332753.i = load i32, ptr %_26.i.i, align 4, !dbg !8064, !alias.scope !8066, !noalias !8025, !noundef !11
  %_3.i243.i = fcmp une float %_0.i338.i, 1.000000e+00, !dbg !8069
  %_3.i241.i = fcmp oeq float %_0.i338.i, 0.000000e+00, !dbg !8071
  %_3.i239.i = fcmp oeq float %_0.i339.i, 0.000000e+00, !dbg !8073
  %99 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1508, !dbg !8075
  %_23.i1737 = load i32, ptr %99, align 4, !dbg !8075, !alias.scope !7935, !noalias !7951, !noundef !11
  %100 = zext i32 %_23.i1737 to i64, !dbg !8075
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8077), !dbg !8080
  %_35.not.i.i = icmp ugt i64 %_34, %_50.1
  br i1 %_35.not.i.i, label %bb11.i.i, label %bb1.preheader.i.i, !dbg !8082, !prof !239

bb1.preheader.i.i:                                ; preds = %bb13
  %_15.not.i.i = icmp ult i32 %_23.i1737, %channels.0.val
  %_16.i655.i = select i1 %_15.not.i.i, i64 %ring_length.i1734, i64 0
  %write.pn.i.i = sub nsw i64 %100, %_0.i4.i
  %row.sroa.0.0.i.i = add nsw i64 %write.pn.i.i, %_16.i655.i
  %_18.i656.i = sub nsw i64 %ring_length.i1734, %row.sroa.0.0.i.i
  %..i.i657.i = tail call noundef i64 @llvm.umin.i64(i64 %_34, i64 %_18.i656.i)
  %101 = getelementptr inbounds nuw i8, ptr %channels.0, i64 16
  %_78.0.i.i = load ptr, ptr %101, align 8, !alias.scope !8095, !noalias !8096, !nonnull !11, !noundef !11
  %102 = getelementptr inbounds nuw i8, ptr %channels.0, i64 24
  %_78.1.i.i = load i64, ptr %102, align 8, !alias.scope !8095, !noalias !8096, !noundef !11
  %_25.i658.i = add nsw i64 %..i.i657.i, %row.sroa.0.0.i.i
  %_53.i.i = icmp ult i64 %_25.i658.i, %row.sroa.0.0.i.i
  %_47.not.i.i = icmp ugt i64 %_25.i658.i, %_78.1.i.i
  %or.cond.i.i = or i1 %_53.i.i, %_47.not.i.i
  %_56.i.i = getelementptr float, ptr %_78.0.i.i, i64 %row.sroa.0.0.i.i
  %_2.i92.not.i.i = icmp eq i64 %..i.i657.i, 0
  %_31.i.i = sub nsw i64 %_34, %..i.i657.i
  %_77.i.i = getelementptr float, ptr %_50.0, i64 %..i.i657.i
  %_2.i5594.not.not.i.i = icmp ugt i64 %_34, %_18.i656.i
  br i1 %or.cond.i.i, label %bb18.i659.i, label %bb1.preheader.split.i.i, !dbg !8098, !prof !239

bb1.preheader.split.i.i:                          ; preds = %bb1.preheader.i.i
  %_63.not.i.i = icmp ugt i64 %_31.i.i, %_78.1.i.i
  br i1 %_63.not.i.i, label %bb2.us.i.i, label %bb1.preheader.split.split.i.i, !prof !239

bb2.us.i.i:                                       ; preds = %bb1.preheader.split.i.i
  br i1 %_2.i92.not.i.i, label %bb27.i.i, label %bb9.i21.us.preheader.i.i, !dbg !8110

bb9.i21.us.preheader.i.i:                         ; preds = %bb2.us.i.i
  %103 = shl nuw nsw i64 %..i.i657.i, 2, !dbg !8110
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 4 %_50.0, ptr align 4 %_56.i.i, i64 %103, i1 false), !dbg !8119, !noalias !8121
  br label %bb27.i.i, !dbg !8122

bb1.preheader.split.split.i.i:                    ; preds = %bb1.preheader.split.i.i
  br i1 %_2.i92.not.i.i, label %bb1.preheader.split.split.split.i.i, label %bb1.preheader.split.split.split.us.i.i

bb1.preheader.split.split.split.us.i.i:           ; preds = %bb1.preheader.split.split.i.i
  %104 = shl nuw nsw i64 %..i.i657.i, 2, !dbg !8110
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 4 %_50.0, ptr align 4 %_56.i.i, i64 %104, i1 false), !dbg !8119, !noalias !8121
  br i1 %_2.i5594.not.not.i.i, label %bb15.sink.split.i.i, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel9fill_tapsfEB4_.exit.i

bb1.preheader.split.split.split.i.i:              ; preds = %bb1.preheader.split.split.i.i
  br i1 %_2.i5594.not.not.i.i, label %bb15.sink.split.i.i, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel9fill_tapsfEB4_.exit.i

bb11.i.i:                                         ; preds = %bb13
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_34, i64 noundef range(i64 0, 2305843009213693952) %_50.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6e11da28860dc4315c923646233703e6) #23, !dbg !8129, !noalias !8130
  unreachable, !dbg !8129

bb15.sink.split.i.i:                              ; preds = %bb1.preheader.split.split.split.i.i, %bb1.preheader.split.split.split.us.i.i
  %.sink119.i.i = phi i64 [ %_34, %bb1.preheader.split.split.split.i.i ], [ %_31.i.i, %bb1.preheader.split.split.split.us.i.i ]
  %105 = shl nsw i64 %.sink119.i.i, 2, !dbg !8131
  tail call void @llvm.memcpy.p0.p0.i64(ptr align 4 %_77.i.i, ptr nonnull align 4 %_78.0.i.i, i64 %105, i1 false), !dbg !8135, !noalias !8121
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel9fill_tapsfEB4_.exit.i, !dbg !8136

bb18.i659.i:                                      ; preds = %bb1.preheader.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i.i, i64 noundef %_25.i658.i, i64 noundef %_78.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_22f7da482334bdfa74d39682e51abbdd) #23, !dbg !8137, !noalias !8130
  unreachable, !dbg !8137

bb27.i.i:                                         ; preds = %bb9.i21.us.preheader.i.i, %bb2.us.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_31.i.i, i64 noundef %_78.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6bab2b83b03bbfaea58c986adce7dabd) #23, !dbg !8122, !noalias !8130
  unreachable, !dbg !8122

_RINvNtCse3bfmKSZS8Y_10compressor6kernel9fill_tapsfEB4_.exit.i: ; preds = %bb15.sink.split.i.i, %bb1.preheader.split.split.split.i.i, %bb1.preheader.split.split.split.us.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8138), !dbg !8141
  %_35.not.i660.i = icmp samesign ugt i64 %_34, %_51.1
  br i1 %_35.not.i660.i, label %bb11.i695.i, label %bb1.preheader.i661.i, !dbg !8142, !prof !239

bb1.preheader.i661.i:                             ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel9fill_tapsfEB4_.exit.i
  %106 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1512, !dbg !8148
  %_6.i662.i = load i32, ptr %106, align 8, !dbg !8148, !alias.scope !8149, !noalias !8150, !noundef !11
  %ring_length.i663.i = zext i32 %_6.i662.i to i64, !dbg !8148
  %_15.not.i666.i = icmp ult i32 %_23.i1737, %channels.1.val
  %_16.i667.i = select i1 %_15.not.i666.i, i64 %ring_length.i663.i, i64 0
  %write.pn.i668.i = sub nsw i64 %100, %_0.i.i1704
  %row.sroa.0.0.i669.i = add nsw i64 %_16.i667.i, %write.pn.i668.i
  %_18.i670.i = sub nsw i64 %ring_length.i663.i, %row.sroa.0.0.i669.i
  %..i.i671.i = tail call noundef i64 @llvm.umin.i64(i64 %_34, i64 %_18.i670.i)
  %107 = getelementptr inbounds nuw i8, ptr %channels.1, i64 16
  %_78.0.i672.i = load ptr, ptr %107, align 8, !alias.scope !8149, !noalias !8150, !nonnull !11, !noundef !11
  %108 = getelementptr inbounds nuw i8, ptr %channels.1, i64 24
  %_78.1.i673.i = load i64, ptr %108, align 8, !alias.scope !8149, !noalias !8150, !noundef !11
  %_25.i674.i = add nsw i64 %..i.i671.i, %row.sroa.0.0.i669.i
  %_53.i675.i = icmp ult i64 %_25.i674.i, %row.sroa.0.0.i669.i
  %_47.not.i676.i = icmp ugt i64 %_25.i674.i, %_78.1.i673.i
  %or.cond.i677.i = or i1 %_53.i675.i, %_47.not.i676.i
  %_56.i678.i = getelementptr float, ptr %_78.0.i672.i, i64 %row.sroa.0.0.i669.i
  %_2.i92.not.i679.i = icmp eq i64 %..i.i671.i, 0
  %_31.i680.i = sub nsw i64 %_34, %..i.i671.i
  %_77.i681.i = getelementptr float, ptr %_51.0, i64 %..i.i671.i
  %_2.i5594.not.not.i682.i = icmp ugt i64 %_34, %_18.i670.i
  br i1 %or.cond.i677.i, label %bb18.i694.i, label %bb1.preheader.split.i683.i, !dbg !8152, !prof !239

bb1.preheader.split.i683.i:                       ; preds = %bb1.preheader.i661.i
  %_63.not.i684.i = icmp ugt i64 %_31.i680.i, %_78.1.i673.i
  br i1 %_63.not.i684.i, label %bb2.us.i691.i, label %bb1.preheader.split.split.i685.i, !prof !239

bb2.us.i691.i:                                    ; preds = %bb1.preheader.split.i683.i
  br i1 %_2.i92.not.i679.i, label %bb27.i693.i, label %bb9.i21.us.preheader.i692.i, !dbg !8156

bb9.i21.us.preheader.i692.i:                      ; preds = %bb2.us.i691.i
  %109 = shl nuw nsw i64 %..i.i671.i, 2, !dbg !8156
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 4 %_51.0, ptr align 4 %_56.i678.i, i64 %109, i1 false), !dbg !8160, !noalias !8161
  br label %bb27.i693.i, !dbg !8162

bb1.preheader.split.split.i685.i:                 ; preds = %bb1.preheader.split.i683.i
  br i1 %_2.i92.not.i679.i, label %bb1.preheader.split.split.split.i690.i, label %bb1.preheader.split.split.split.us.i686.i

bb1.preheader.split.split.split.us.i686.i:        ; preds = %bb1.preheader.split.split.i685.i
  %110 = shl nuw nsw i64 %..i.i671.i, 2, !dbg !8156
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 4 %_51.0, ptr align 4 %_56.i678.i, i64 %110, i1 false), !dbg !8160, !noalias !8161
  br i1 %_2.i5594.not.not.i682.i, label %bb15.sink.split.i688.i, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipB3_ECse3bfmKSZS8Y_10compressor.exit.i

bb1.preheader.split.split.split.i690.i:           ; preds = %bb1.preheader.split.split.i685.i
  br i1 %_2.i5594.not.not.i682.i, label %bb15.sink.split.i688.i, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipB3_ECse3bfmKSZS8Y_10compressor.exit.i

bb11.i695.i:                                      ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel9fill_tapsfEB4_.exit.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_34, i64 noundef range(i64 0, 2305843009213693952) %_51.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6e11da28860dc4315c923646233703e6) #23, !dbg !8166, !noalias !8167
  unreachable, !dbg !8166

bb15.sink.split.i688.i:                           ; preds = %bb1.preheader.split.split.split.i690.i, %bb1.preheader.split.split.split.us.i686.i
  %.sink119.i689.i = phi i64 [ %_34, %bb1.preheader.split.split.split.i690.i ], [ %_31.i680.i, %bb1.preheader.split.split.split.us.i686.i ]
  %111 = shl nsw i64 %.sink119.i689.i, 2, !dbg !8168
  tail call void @llvm.memcpy.p0.p0.i64(ptr align 4 %_77.i681.i, ptr nonnull align 4 %_78.0.i672.i, i64 %111, i1 false), !dbg !8172, !noalias !8161
  br label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipB3_ECse3bfmKSZS8Y_10compressor.exit.i, !dbg !8173

bb18.i694.i:                                      ; preds = %bb1.preheader.i661.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i669.i, i64 noundef %_25.i674.i, i64 noundef %_78.1.i673.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_22f7da482334bdfa74d39682e51abbdd) #23, !dbg !8174, !noalias !8167
  unreachable, !dbg !8174

bb27.i693.i:                                      ; preds = %bb9.i21.us.preheader.i692.i, %bb2.us.i691.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_31.i680.i, i64 noundef %_78.1.i673.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6bab2b83b03bbfaea58c986adce7dabd) #23, !dbg !8162, !noalias !8167
  unreachable, !dbg !8162

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipB3_ECse3bfmKSZS8Y_10compressor.exit.i: ; preds = %bb15.sink.split.i688.i, %bb1.preheader.split.split.split.i690.i, %bb1.preheader.split.split.split.us.i686.i
  %_176.not.i = icmp samesign ugt i64 %_34, %_52.1
  br i1 %_176.not.i, label %bb58.i, label %bb57.i, !dbg !8175, !prof !239

bb58.i:                                           ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipB3_ECse3bfmKSZS8Y_10compressor.exit.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_34, i64 noundef range(i64 0, 2305843009213693952) %_52.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a3aefd714e95b8f966811cbb71c5f9b5) #23, !dbg !8185, !noalias !8186
  unreachable, !dbg !8185

bb57.i:                                           ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipB3_ECse3bfmKSZS8Y_10compressor.exit.i
  %_191.not.i = icmp samesign ugt i64 %_34, %_53.1, !dbg !8187
  br i1 %_191.not.i, label %bb64.i, label %bb11.preheader.i, !dbg !8187, !prof !161

bb11.preheader.i:                                 ; preds = %bb57.i
  %_2.i.i.i879.not.i = icmp eq i64 %_34, 0, !dbg !8193
  br i1 %_2.i.i.i879.not.i, label %bb37.thread.i, label %bb14.lr.ph.i, !dbg !8193

bb37.thread.i:                                    ; preds = %bb11.preheader.i
  %112 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1508, !dbg !8201
  store i32 %_23.i1737, ptr %112, align 4, !dbg !8201, !alias.scope !7937, !noalias !8202
  br label %bb19, !dbg !8203

bb14.lr.ph.i:                                     ; preds = %bb11.preheader.i
  %_6.i.i1738 = load i64, ptr %detector, align 8, !range !220, !alias.scope !7933, !noalias !8213
  %113 = getelementptr inbounds nuw i8, ptr %detector, i64 16
  %sidechain_left.1.i.i1739 = load i64, ptr %113, align 8, !alias.scope !7933, !noalias !8213
  %114 = getelementptr inbounds nuw i8, ptr %detector, i64 24
  %sidechain_right.0.i.i1740 = load ptr, ptr %114, align 8, !alias.scope !7933, !noalias !8213, !nonnull !11, !align !3663
  %115 = getelementptr inbounds nuw i8, ptr %detector, i64 32
  %sidechain_right.1.i.i1741 = load i64, ptr %115, align 8, !alias.scope !7933, !noalias !8213
  %116 = getelementptr inbounds nuw i8, ptr %detector, i64 8
  %sidechain_left.0.i.i1742 = load ptr, ptr %116, align 8, !alias.scope !7933, !noalias !8213, !nonnull !11, !align !3663
  %117 = getelementptr inbounds nuw i8, ptr %channels.0, i64 8
  %_335.1.i = load i64, ptr %117, align 8, !alias.scope !7935, !noalias !7951
  %_335.0.i = load ptr, ptr %channels.0, align 8, !alias.scope !7935, !noalias !7951, !nonnull !11
  %118 = getelementptr inbounds nuw i8, ptr %channels.1, i64 8
  %_337.1.i = load i64, ptr %118, align 8, !alias.scope !7937, !noalias !8202
  %_337.0.i = load ptr, ptr %channels.1, align 8, !alias.scope !7937, !noalias !8202, !nonnull !11
  %119 = fneg float %_0.i327.i
  %120 = fneg float %_0.i335.i
  br label %bb14.i1743, !dbg !8193

bb64.i:                                           ; preds = %bb57.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_34, i64 noundef range(i64 0, 2305843009213693952) %_53.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_39c6596be70c3202efa3964a6ea265b7) #23, !dbg !8214, !noalias !8186
  unreachable, !dbg !8214

bb14.i1743:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit353.i, %bb14.lr.ph.i
  %head.sroa.0.0882.i = phi i64 [ %100, %bb14.lr.ph.i ], [ %spec.store.select.i1754, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit353.i ]
  %iter.sroa.33.0881.i = phi i64 [ 0, %bb14.lr.ph.i ], [ %_9.0.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit353.i ]
  %_9.0.i.i = add nuw nsw i64 %iter.sroa.33.0881.i, 1, !dbg !8215
  %data.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_50.0, i64 %iter.sroa.33.0881.i, !dbg !8218
  %data.i5.i.i.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_51.0, i64 %iter.sroa.33.0881.i, !dbg !8229
  %_3.i.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_52.0, i64 %iter.sroa.33.0881.i, !dbg !8232
  %_3.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_53.0, i64 %iter.sroa.33.0881.i, !dbg !8237
  %_51.i = add nuw nsw i64 %iter.sroa.33.0881.i, %spec.store.select, !dbg !8240
  %_205.i = icmp samesign ugt i64 %_51.i, %left.1, !dbg !8242
  br i1 %_205.i, label %bb68.i, label %bb69.i, !dbg !8242, !prof !161

bb36.preheader.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit353.i
  %121 = trunc i64 %spec.store.select.i1754 to i32, !dbg !8249
  store i32 %121, ptr %99, align 4, !dbg !8249, !alias.scope !7935, !noalias !7951
  %122 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1508, !dbg !8201
  store i32 %121, ptr %122, align 4, !dbg !8201, !alias.scope !7937, !noalias !8202
  %123 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1504, !dbg !8250
  %124 = load float, ptr %123, align 8, !dbg !8250, !alias.scope !7935, !noalias !7951, !noundef !11
  %125 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1504, !dbg !8251
  %126 = load float, ptr %125, align 8, !dbg !8251, !alias.scope !7937, !noalias !8202, !noundef !11
  br label %bb36.i1756, !dbg !8252

bb36.i1756:                                       ; preds = %bb36.i1756, %bb36.preheader.i
  %iter2.sroa.8.0886.i = phi i64 [ %127, %bb36.i1756 ], [ 0, %bb36.preheader.i ]
  %gain_right.sroa.0.0885.i = phi float [ %_0.i419.i, %bb36.i1756 ], [ %126, %bb36.preheader.i ]
  %gain_left.sroa.0.0884.i = phi float [ %_0.i415.i, %bb36.i1756 ], [ %124, %bb36.preheader.i ]
  %127 = add nuw i64 %iter2.sroa.8.0886.i, 1, !dbg !8259
  %_3.i.i715.i = getelementptr inbounds nuw float, ptr %_52.0, i64 %iter2.sroa.8.0886.i, !dbg !8261
  %_3.i1.i.i = getelementptr inbounds nuw float, ptr %_53.0, i64 %iter2.sroa.8.0886.i, !dbg !8264
  %_117.i = load float, ptr %_3.i.i715.i, align 4, !dbg !8267, !alias.scope !7943, !noalias !8269, !noundef !11
  %_3.i259.i = fcmp uge float %_117.i, %gain_left.sroa.0.0884.i, !dbg !8270
  %_4.i451.i = select i1 %_3.i259.i, i32 %_0.i324751.i, i32 %_0.i325750.i, !dbg !8273
  %_0.i452.i = bitcast i32 %_4.i451.i to float, !dbg !8275
  %_0.i320.i = fsub float %_117.i, %gain_left.sroa.0.0884.i, !dbg !8277
  %_4.i278.i = fmul float %_0.i320.i, %_0.i452.i, !dbg !8280
  %_0.i279.i = fadd float %gain_left.sroa.0.0884.i, %_4.i278.i, !dbg !8280
  %128 = tail call noundef float @llvm.fabs.f32(float %_0.i279.i), !dbg !8282
  %129 = fcmp uge float %128, 0x3BC79CA100000000, !dbg !8285
  %_0.i415.i = select i1 %129, float %_0.i279.i, float 0.000000e+00, !dbg !8287
  store float %_0.i415.i, ptr %_3.i.i715.i, align 4, !dbg !8288, !alias.scope !7943, !noalias !8269
  %_121.i1757 = load float, ptr %_3.i1.i.i, align 4, !dbg !8289, !alias.scope !7945, !noalias !8290, !noundef !11
  %_3.i263.i = fcmp uge float %_121.i1757, %gain_right.sroa.0.0885.i, !dbg !8291
  %_4.i458.i = select i1 %_3.i263.i, i32 %_0.i332753.i, i32 %_0.i333752.i, !dbg !8294
  %_0.i459.i = bitcast i32 %_4.i458.i to float, !dbg !8296
  %_0.i321.i = fsub float %_121.i1757, %gain_right.sroa.0.0885.i, !dbg !8298
  %_4.i280.i = fmul float %_0.i321.i, %_0.i459.i, !dbg !8301
  %_0.i281.i = fadd float %gain_right.sroa.0.0885.i, %_4.i280.i, !dbg !8301
  %130 = tail call noundef float @llvm.fabs.f32(float %_0.i281.i), !dbg !8303
  %131 = fcmp uge float %130, 0x3BC79CA100000000, !dbg !8306
  %_0.i419.i = select i1 %131, float %_0.i281.i, float 0.000000e+00, !dbg !8308
  store float %_0.i419.i, ptr %_3.i1.i.i, align 4, !dbg !8309, !alias.scope !7945, !noalias !8290
  %exitcond951.not.i = icmp eq i64 %127, %_34, !dbg !8252
  br i1 %exitcond951.not.i, label %bb43.preheader.i, label %bb36.i1756, !dbg !8252

bb43.preheader.i:                                 ; preds = %bb36.i1756
  store float %_0.i415.i, ptr %123, align 8, !dbg !8310, !alias.scope !7935, !noalias !7951
  store float %_0.i419.i, ptr %125, align 8, !dbg !8311, !alias.scope !7937, !noalias !8202
  br label %bb43.i1758, !dbg !8312

bb43.i1758:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit373.i, %bb43.preheader.i
  %iter3.sroa.13.0892.i = phi i64 [ %_9.0.i728.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit373.i ], [ 0, %bb43.preheader.i ]
  %_9.0.i728.i = add nuw nsw i64 %iter3.sroa.13.0892.i, 1, !dbg !8318
  %_3.i.i.i.i724.i = getelementptr inbounds nuw float, ptr %_52.0, i64 %iter3.sroa.13.0892.i, !dbg !8321
  %_3.i1.i.i.i.i = getelementptr inbounds nuw float, ptr %_53.0, i64 %iter3.sroa.13.0892.i, !dbg !8327
  %_138.i = add nuw nsw i64 %iter3.sroa.13.0892.i, %spec.store.select, !dbg !8330
  %_311.i = icmp samesign ugt i64 %_138.i, %left.1, !dbg !8312
  br i1 %_311.i, label %bb94.i, label %bb95.i, !dbg !8312, !prof !161

bb95.i:                                           ; preds = %bb43.i1758
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8331), !dbg !8334
  %_3.not.i374.i = icmp eq i64 %left.1, %_138.i, !dbg !8335
  br i1 %_3.not.i374.i, label %panic.i376.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit377.i, !dbg !8335

panic.i376.i:                                     ; preds = %bb95.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !8335, !noalias !8337
  unreachable, !dbg !8335

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit377.i: ; preds = %bb95.i
  %_318.i = getelementptr inbounds nuw float, ptr %left.0, i64 %_138.i, !dbg !8338
  %_0.i375.i = load float, ptr %_318.i, align 4, !dbg !8335, !alias.scope !8344, !noalias !8345, !noundef !11
  %_143.i = load float, ptr %_3.i.i.i.i724.i, align 4, !dbg !8346, !alias.scope !7943, !noalias !8269, !noundef !11
  %_0.i275.i = fadd float %_0.i331.i, %_143.i, !dbg !8348
  %_0.i306.i = fmul float %_0.i275.i, 0x3FC542A5A0000000, !dbg !8351
  %_3.i.i531.inv.i = fcmp ogt float %_0.i306.i, -1.260000e+02, !dbg !8354
  %_0.i.i538.i = select i1 %_3.i.i531.inv.i, float %_0.i306.i, float -1.260000e+02, !dbg !8354
  %_3.i.i609.inv.i = fcmp olt float %_0.i.i538.i, 1.270000e+02, !dbg !8358
  %_0.i.i616.i = select i1 %_3.i.i609.inv.i, float %_0.i.i538.i, float 1.270000e+02, !dbg !8358
  %132 = tail call noundef float @llvm.floor.f32(float %_0.i.i616.i), !dbg !8361
  %_0.i317.i = fsub float %_0.i.i616.i, %132, !dbg !8365
  %_0.i295.i = fmul float %_0.i317.i, 0x3F5E974FA0000000, !dbg !8367
  %_0.i272.i = fadd float %_0.i295.i, 0x3F82778560000000, !dbg !8369
  %_0.i295.1.i = fmul float %_0.i317.i, %_0.i272.i, !dbg !8367
  %_0.i272.1.i = fadd float %_0.i295.1.i, 0x3FAC91CE60000000, !dbg !8369
  %_0.i295.2.i = fmul float %_0.i317.i, %_0.i272.1.i, !dbg !8367
  %_0.i272.2.i = fadd float %_0.i295.2.i, 0x3FCEBDB560000000, !dbg !8369
  %_0.i295.3.i = fmul float %_0.i317.i, %_0.i272.2.i, !dbg !8367
  %_0.i272.3.i = fadd float %_0.i295.3.i, 0x3FE62E4BA0000000, !dbg !8369
  %_0.i294.i = fmul float %_0.i317.i, %_0.i272.3.i, !dbg !8371
  %_0.i271.i = fadd float %_0.i294.i, 1.000000e+00, !dbg !8373
  %biased.i230.i = fadd float %132, 0x4160000FE0000000, !dbg !8375
  %_4.i231.i = bitcast float %biased.i230.i to i32, !dbg !8377
  %_3.i232.i = shl i32 %_4.i231.i, 23, !dbg !8379
  %_0.i233.i = bitcast i32 %_3.i232.i to float, !dbg !8380
  %_0.i293.i = fmul float %_0.i271.i, %_0.i233.i, !dbg !8382
  %_0.i305.i = fmul float %_0.i375.i, %_0.i293.i, !dbg !8384
  %_0.i322.i = fsub float %_0.i305.i, %_0.i375.i, !dbg !8386
  %_4.i282.i = fmul float %_0.i330.i, %_0.i322.i, !dbg !8389
  %_0.i283.i = fadd float %_0.i375.i, %_4.i282.i, !dbg !8389
  %_3.i245.i = fcmp oeq float %_143.i, 0.000000e+00, !dbg !8391
  %_0.i513756.i = and i1 %_3.i234.i, %_3.i245.i, !dbg !8393
  %133 = or i1 %_3.i235.i, %_0.i513756.i
  %_0.i509758.reass.reass.i.reass.reass = or i1 %133, %bypass
  %_4.i472.v.i = select i1 %_3.i237.i, float %_0.i283.i, float %_0.i305.i, !dbg !8395
  %_4.i465.v.i = select i1 %_0.i509758.reass.reass.i.reass.reass, float %_0.i375.i, float %_4.i472.v.i, !dbg !8397
  store float %_4.i465.v.i, ptr %_318.i, align 4, !dbg !8399, !alias.scope !8401, !noalias !8345
  %_323.i = icmp samesign ugt i64 %_138.i, %right.1, !dbg !8404
  br i1 %_323.i, label %bb96.i, label %bb97.i, !dbg !8404, !prof !161

bb94.i:                                           ; preds = %bb43.i1758
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_138.i, i64 noundef range(i64 0, 2305843009213693952) %left.1, i64 noundef range(i64 0, 2305843009213693952) %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c026f29aed5fe4d7cd9557b4f3b74850) #23, !dbg !8408, !noalias !8186
  unreachable, !dbg !8408

bb97.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit377.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8409), !dbg !8412
  %_3.not.i370.i = icmp eq i64 %right.1, %_138.i, !dbg !8413
  br i1 %_3.not.i370.i, label %panic.i372.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit373.i, !dbg !8413

panic.i372.i:                                     ; preds = %bb97.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !8413, !noalias !8415
  unreachable, !dbg !8413

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit373.i: ; preds = %bb97.i
  %_330.i = getelementptr inbounds nuw float, ptr %right.0, i64 %_138.i, !dbg !8416
  %_0.i371.i = load float, ptr %_330.i, align 4, !dbg !8413, !alias.scope !8421, !noalias !8422, !noundef !11
  %_151.i = load float, ptr %_3.i1.i.i.i.i, align 4, !dbg !8423, !alias.scope !7945, !noalias !8290, !noundef !11
  %_0.i276.i = fadd float %_0.i339.i, %_151.i, !dbg !8425
  %_0.i309.i = fmul float %_0.i276.i, 0x3FC542A5A0000000, !dbg !8428
  %_3.i.i523.inv.i = fcmp ogt float %_0.i309.i, -1.260000e+02, !dbg !8431
  %_0.i.i530.i = select i1 %_3.i.i523.inv.i, float %_0.i309.i, float -1.260000e+02, !dbg !8431
  %_3.i.i601.inv.i = fcmp olt float %_0.i.i530.i, 1.270000e+02, !dbg !8435
  %_0.i.i608.i = select i1 %_3.i.i601.inv.i, float %_0.i.i530.i, float 1.270000e+02, !dbg !8435
  %134 = tail call noundef float @llvm.floor.f32(float %_0.i.i608.i), !dbg !8438
  %_0.i316.i = fsub float %_0.i.i608.i, %134, !dbg !8442
  %_0.i292.i = fmul float %_0.i316.i, 0x3F5E974FA0000000, !dbg !8444
  %_0.i270.i = fadd float %_0.i292.i, 0x3F82778560000000, !dbg !8446
  %_0.i292.1.i = fmul float %_0.i316.i, %_0.i270.i, !dbg !8444
  %_0.i270.1.i = fadd float %_0.i292.1.i, 0x3FAC91CE60000000, !dbg !8446
  %_0.i292.2.i = fmul float %_0.i316.i, %_0.i270.1.i, !dbg !8444
  %_0.i270.2.i = fadd float %_0.i292.2.i, 0x3FCEBDB560000000, !dbg !8446
  %_0.i292.3.i = fmul float %_0.i316.i, %_0.i270.2.i, !dbg !8444
  %_0.i270.3.i = fadd float %_0.i292.3.i, 0x3FE62E4BA0000000, !dbg !8446
  %_0.i291.i = fmul float %_0.i316.i, %_0.i270.3.i, !dbg !8448
  %_0.i269.i = fadd float %_0.i291.i, 1.000000e+00, !dbg !8450
  %biased.i.i = fadd float %134, 0x4160000FE0000000, !dbg !8452
  %_4.i227.i = bitcast float %biased.i.i to i32, !dbg !8454
  %_3.i228.i = shl i32 %_4.i227.i, 23, !dbg !8456
  %_0.i229.i = bitcast i32 %_3.i228.i to float, !dbg !8457
  %_0.i290.i = fmul float %_0.i269.i, %_0.i229.i, !dbg !8459
  %_0.i308.i = fmul float %_0.i371.i, %_0.i290.i, !dbg !8461
  %_0.i323.i = fsub float %_0.i308.i, %_0.i371.i, !dbg !8463
  %_4.i284.i = fmul float %_0.i338.i, %_0.i323.i, !dbg !8466
  %_0.i285.i = fadd float %_0.i371.i, %_4.i284.i, !dbg !8466
  %_3.i247.i = fcmp oeq float %_151.i, 0.000000e+00, !dbg !8468
  %_0.i514763.i = and i1 %_3.i239.i, %_3.i247.i, !dbg !8470
  %135 = or i1 %_3.i241.i, %_0.i514763.i
  %_0.i511765.reass.reass.i.reass.reass = or i1 %135, %bypass
  %_4.i486.v.i = select i1 %_3.i243.i, float %_0.i285.i, float %_0.i308.i, !dbg !8472
  %_4.i479.v.i = select i1 %_0.i511765.reass.reass.i.reass.reass, float %_0.i371.i, float %_4.i486.v.i, !dbg !8474
  store float %_4.i479.v.i, ptr %_330.i, align 4, !dbg !8476, !alias.scope !8478, !noalias !8422
  %exitcond952.not.i = icmp eq i64 %_9.0.i728.i, %_34, !dbg !8203
  br i1 %exitcond952.not.i, label %bb19, label %bb43.i1758, !dbg !8203

bb96.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit377.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_138.i, i64 noundef range(i64 0, 2305843009213693952) %right.1, i64 noundef range(i64 0, 2305843009213693952) %right.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5af02affad714ed6461f8ae2fddf49da) #23, !dbg !8481, !noalias !8186
  unreachable, !dbg !8481

bb69.i:                                           ; preds = %bb14.i1743
  %_212.i = getelementptr inbounds nuw float, ptr %left.0, i64 %_51.i, !dbg !8482
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8486), !dbg !8489
  %_3.not.i366.i = icmp eq i64 %left.1, %_51.i, !dbg !8490
  br i1 %_3.not.i366.i, label %panic.i368.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit369.i, !dbg !8490

panic.i368.i:                                     ; preds = %bb69.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !8490, !noalias !8492
  unreachable, !dbg !8490

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit369.i: ; preds = %bb69.i
  %_0.i367.i = load float, ptr %_212.i, align 4, !dbg !8490, !alias.scope !8493, !noalias !8345, !noundef !11
  %_216.i = icmp samesign ugt i64 %_51.i, %right.1, !dbg !8494
  br i1 %_216.i, label %bb70.i, label %bb71.i, !dbg !8494, !prof !161

bb68.i:                                           ; preds = %bb14.i1743
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_51.i, i64 noundef range(i64 0, 2305843009213693952) %left.1, i64 noundef range(i64 0, 2305843009213693952) %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b2f50309aa881af863c4c5948e01a00d) #23, !dbg !8499, !noalias !8186
  unreachable, !dbg !8499

bb71.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit369.i
  %_223.i = getelementptr inbounds nuw float, ptr %right.0, i64 %_51.i, !dbg !8500
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8505), !dbg !8508
  %_3.not.i362.i = icmp eq i64 %right.1, %_51.i, !dbg !8509
  br i1 %_3.not.i362.i, label %panic.i364.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit365.i, !dbg !8509

panic.i364.i:                                     ; preds = %bb71.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !8509, !noalias !8511
  unreachable, !dbg !8509

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit365.i: ; preds = %bb71.i
  %_0.i363.i = load float, ptr %_223.i, align 4, !dbg !8509, !alias.scope !8512, !noalias !8422, !noundef !11
  switch i64 %_6.i.i1738, label %default.unreachable [
    i64 0, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i1751
    i64 1, label %bb3.i.i1762
    i64 2, label %bb2.i.i1744
  ], !dbg !8513

bb3.i.i1762:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit365.i
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i1751, !dbg !8516

bb2.i.i1744:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit365.i
  %_25.i.i1745 = icmp ugt i64 %_51.i, %sidechain_left.1.i.i1739, !dbg !8517
  br i1 %_25.i.i1745, label %bb17.i.i1761, label %bb18.i.i1746, !dbg !8517, !prof !161

bb18.i.i1746:                                     ; preds = %bb2.i.i1744
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8520), !dbg !8523
  %_3.not.i342.i = icmp eq i64 %sidechain_left.1.i.i1739, %_51.i, !dbg !8524
  br i1 %_3.not.i342.i, label %panic.i344.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit345.i, !dbg !8524

panic.i344.i:                                     ; preds = %bb18.i.i1746
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !8524, !noalias !8526
  unreachable, !dbg !8524

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit345.i: ; preds = %bb18.i.i1746
  %_32.i.i1747 = getelementptr inbounds nuw float, ptr %sidechain_left.0.i.i1742, i64 %_51.i, !dbg !8530
  %_0.i343.i = load float, ptr %_32.i.i1747, align 4, !dbg !8524, !alias.scope !8520, !noalias !8532, !noundef !11
  %_33.i.i1748 = icmp ugt i64 %_51.i, %sidechain_right.1.i.i1741, !dbg !8533
  br i1 %_33.i.i1748, label %bb19.i.i1760, label %bb20.i.i1749, !dbg !8533, !prof !161

bb17.i.i1761:                                     ; preds = %bb2.i.i1744
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 2305843009213693952) %_51.i, i64 noundef %sidechain_left.1.i.i1739, i64 noundef %sidechain_left.1.i.i1739, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a0a4023c361dd50869bbb08b76482f50) #23, !dbg !8536, !noalias !8532
  unreachable, !dbg !8536

bb20.i.i1749:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit345.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8537), !dbg !8540
  %_3.not.i.i = icmp eq i64 %sidechain_right.1.i.i1741, %_51.i, !dbg !8541
  br i1 %_3.not.i.i, label %panic.i341.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i, !dbg !8541

panic.i341.i:                                     ; preds = %bb20.i.i1749
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !8541, !noalias !8543
  unreachable, !dbg !8541

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i: ; preds = %bb20.i.i1749
  %_40.i.i1750 = getelementptr inbounds nuw float, ptr %sidechain_right.0.i.i1740, i64 %_51.i, !dbg !8544
  %_0.i340.i = load float, ptr %_40.i.i1750, align 4, !dbg !8541, !alias.scope !8537, !noalias !8532, !noundef !11
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i1751, !dbg !8546

bb19.i.i1760:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit345.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 2305843009213693952) %_51.i, i64 noundef %sidechain_right.1.i.i1741, i64 noundef %sidechain_right.1.i.i1741, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5807a2a07e9839e3d32631b4243eb17f) #23, !dbg !8547, !noalias !8532
  unreachable, !dbg !8547

_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i1751: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i, %bb3.i.i1762, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit365.i
  %main_right.sroa.0.0.i.i1752 = phi float [ %_0.i363.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit365.i ], [ 0.000000e+00, %bb3.i.i1762 ], [ %_0.i340.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i ]
  %main_left.sroa.0.0.i.i1753 = phi float [ %_0.i367.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit365.i ], [ 0.000000e+00, %bb3.i.i1762 ], [ %_0.i343.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i ]
  %136 = tail call noundef float @llvm.fabs.f32(float %main_left.sroa.0.0.i.i1753), !dbg !8548
  %137 = tail call noundef float @llvm.fabs.f32(float %main_right.sroa.0.0.i.i1752), !dbg !8550
  %_3.i.i592.i = fcmp ule float %136, %137, !dbg !8552
  %_6.i.i594.i = bitcast float %136 to i32, !dbg !8555
  %_8.i.i596.i = bitcast float %137 to i32, !dbg !8558
  %_4.i.i599.i = select i1 %_3.i.i592.i, i32 %_8.i.i596.i, i32 %_6.i.i594.i, !dbg !8560
  %_0.i313.i = fmul float %136, 5.000000e-01, !dbg !8561
  %_0.i312.i = fmul float %137, 5.000000e-01, !dbg !8563
  %_0.i277.i = fadd float %_0.i312.i, %_0.i313.i, !dbg !8565
  %_6.i502.i = bitcast float %_0.i277.i to i32, !dbg !8567
  %_4.i507.i = select i1 %.not.i1735, i32 %_6.i502.i, i32 %_4.i.i599.i, !dbg !8570
  %_4.i500.i = select i1 %98, i32 %_6.i.i594.i, i32 %_4.i507.i, !dbg !8571
  %_4.i493.i = select i1 %98, i32 %_8.i.i596.i, i32 %_4.i507.i, !dbg !8573
  %138 = add i64 %head.sroa.0.0882.i, 1, !dbg !8575
  %_62.i = icmp eq i64 %138, %ring_length.i1734, !dbg !8577
  %spec.store.select.i1754 = select i1 %_62.i, i64 0, i64 %138, !dbg !8577
  %_224.i = icmp ugt i64 %head.sroa.0.0882.i, %_335.1.i, !dbg !8579
  br i1 %_224.i, label %bb72.i, label %bb73.i, !dbg !8579, !prof !161

bb70.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit369.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_51.i, i64 noundef range(i64 0, 2305843009213693952) %right.1, i64 noundef range(i64 0, 2305843009213693952) %right.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2b558c2a7d8c32f2ec911d93287e9a55) #23, !dbg !8586, !noalias !8186
  unreachable, !dbg !8586

bb73.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i1751
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8587), !dbg !8590
  %_4.not.i403.i = icmp eq i64 %_335.1.i, %head.sroa.0.0882.i, !dbg !8591
  br i1 %_4.not.i403.i, label %panic.i404.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit405.i, !dbg !8591

panic.i404.i:                                     ; preds = %bb73.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #23, !dbg !8591, !noalias !8593
  unreachable, !dbg !8591

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit405.i: ; preds = %bb73.i
  %_231.i = getelementptr inbounds nuw float, ptr %_335.0.i, i64 %head.sroa.0.0882.i, !dbg !8594
  store float %_0.i367.i, ptr %_231.i, align 4, !dbg !8591, !alias.scope !8587, !noalias !8186
  %_232.i = icmp ugt i64 %head.sroa.0.0882.i, %_78.1.i.i, !dbg !8599
  br i1 %_232.i, label %bb74.i, label %bb75.i, !dbg !8599, !prof !161

bb72.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i1751
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %head.sroa.0.0882.i, i64 noundef %_335.1.i, i64 noundef %_335.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bdcb1b3b7b781c0308467c6e4a54ec4e) #23, !dbg !8603, !noalias !8186
  unreachable, !dbg !8603

bb75.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit405.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8604), !dbg !8607
  %_4.not.i400.i = icmp eq i64 %_78.1.i.i, %head.sroa.0.0882.i, !dbg !8608
  br i1 %_4.not.i400.i, label %panic.i401.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit402.i, !dbg !8608

panic.i401.i:                                     ; preds = %bb75.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #23, !dbg !8608, !noalias !8610
  unreachable, !dbg !8608

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit402.i: ; preds = %bb75.i
  %_239.i = getelementptr inbounds nuw float, ptr %_78.0.i.i, i64 %head.sroa.0.0882.i, !dbg !8611
  store i32 %_4.i500.i, ptr %_239.i, align 4, !dbg !8608, !alias.scope !8604, !noalias !8186
  %_240.i = icmp ugt i64 %head.sroa.0.0882.i, %_337.1.i, !dbg !8616
  br i1 %_240.i, label %bb76.i, label %bb77.i, !dbg !8616, !prof !161

bb74.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit405.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %head.sroa.0.0882.i, i64 noundef %_78.1.i.i, i64 noundef %_78.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_da6aedc6f0cd3ef0ecdf38514bf95c44) #23, !dbg !8620, !noalias !8186
  unreachable, !dbg !8620

bb77.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit402.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8621), !dbg !8624
  %_4.not.i397.i = icmp eq i64 %_337.1.i, %head.sroa.0.0882.i, !dbg !8625
  br i1 %_4.not.i397.i, label %panic.i398.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit399.i, !dbg !8625

panic.i398.i:                                     ; preds = %bb77.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #23, !dbg !8625, !noalias !8627
  unreachable, !dbg !8625

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit399.i: ; preds = %bb77.i
  %_247.i = getelementptr inbounds nuw float, ptr %_337.0.i, i64 %head.sroa.0.0882.i, !dbg !8628
  store float %_0.i363.i, ptr %_247.i, align 4, !dbg !8625, !alias.scope !8621, !noalias !8186
  %_248.i = icmp ugt i64 %head.sroa.0.0882.i, %_78.1.i673.i, !dbg !8633
  br i1 %_248.i, label %bb78.i, label %bb79.i, !dbg !8633, !prof !161

bb76.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit402.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %head.sroa.0.0882.i, i64 noundef %_337.1.i, i64 noundef %_337.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aca6e49a571217c31ca2447292225430) #23, !dbg !8637, !noalias !8186
  unreachable, !dbg !8637

bb79.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit399.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8638), !dbg !8641
  %_4.not.i394.i = icmp eq i64 %_78.1.i673.i, %head.sroa.0.0882.i, !dbg !8642
  br i1 %_4.not.i394.i, label %panic.i395.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit396.i, !dbg !8642

panic.i395.i:                                     ; preds = %bb79.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #23, !dbg !8642, !noalias !8644
  unreachable, !dbg !8642

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit396.i: ; preds = %bb79.i
  %_255.i = getelementptr inbounds nuw float, ptr %_78.0.i672.i, i64 %head.sroa.0.0882.i, !dbg !8645
  store i32 %_4.i493.i, ptr %_255.i, align 4, !dbg !8642, !alias.scope !8638, !noalias !8186
  %_256.i = icmp ugt i64 %spec.store.select.i1754, %_335.1.i, !dbg !8650
  br i1 %_256.i, label %bb80.i, label %bb81.i, !dbg !8650, !prof !161

bb78.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit399.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %head.sroa.0.0882.i, i64 noundef %_78.1.i673.i, i64 noundef %_78.1.i673.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_537a50830147a702ee833cf38f119351) #23, !dbg !8654, !noalias !8186
  unreachable, !dbg !8654

bb81.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit396.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8655), !dbg !8658
  %_3.not.i358.i = icmp eq i64 %_335.1.i, %spec.store.select.i1754, !dbg !8659
  br i1 %_3.not.i358.i, label %panic.i360.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit393.i, !dbg !8659

panic.i360.i:                                     ; preds = %bb81.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !8659, !noalias !8661
  unreachable, !dbg !8659

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit393.i: ; preds = %bb81.i
  %_263.i = getelementptr inbounds nuw float, ptr %_335.0.i, i64 %spec.store.select.i1754, !dbg !8662
  %_0.i359.i = load float, ptr %_263.i, align 4, !dbg !8659, !alias.scope !8655, !noalias !8186, !noundef !11
  store float %_0.i359.i, ptr %_212.i, align 4, !dbg !8667, !alias.scope !8669, !noalias !8345
  %_268.i = icmp ugt i64 %spec.store.select.i1754, %_337.1.i, !dbg !8672
  br i1 %_268.i, label %bb82.i, label %bb83.i, !dbg !8672, !prof !161

bb80.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit396.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %spec.store.select.i1754, i64 noundef %_335.1.i, i64 noundef %_335.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e878a2c67fc44f18741fecb8548c8ef7) #23, !dbg !8676, !noalias !8186
  unreachable, !dbg !8676

bb83.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit393.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8677), !dbg !8680
  %_3.not.i354.i = icmp eq i64 %_337.1.i, %spec.store.select.i1754, !dbg !8681
  br i1 %_3.not.i354.i, label %panic.i356.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit353.i, !dbg !8681

panic.i356.i:                                     ; preds = %bb83.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !8681, !noalias !8683
  unreachable, !dbg !8681

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit353.i: ; preds = %bb83.i
  %_275.i = getelementptr inbounds nuw float, ptr %_337.0.i, i64 %spec.store.select.i1754, !dbg !8684
  %_0.i355.i = load float, ptr %_275.i, align 4, !dbg !8681, !alias.scope !8677, !noalias !8186, !noundef !11
  store float %_0.i355.i, ptr %_223.i, align 4, !dbg !8689, !alias.scope !8691, !noalias !8422
  %_0.i351.i = load float, ptr %data.i.i.i.i.i.i.i.i.i, align 4, !dbg !8694, !alias.scope !8696, !noalias !8699, !noundef !11
  %_3.i.i556.i = fcmp ule float %_0.i351.i, 0x3E45798EE0000000, !dbg !8700
  %_6.i.i558.i = bitcast float %_0.i351.i to i32, !dbg !8704
  %_4.i.i563.i = select i1 %_3.i.i556.i, i32 841731191, i32 %_6.i.i558.i, !dbg !8707
  %_0.i.i564.i = bitcast i32 %_4.i.i563.i to float, !dbg !8708
  %_3.i.i515.i = fcmp ule float %_0.i.i564.i, 0x3810000000000000, !dbg !8710
  %_4.i.i521.i = select i1 %_3.i.i515.i, i32 8388608, i32 %_4.i.i563.i, !dbg !8715
  %_5.i383.i = and i32 %_4.i.i521.i, 8388607, !dbg !8717
  %_4.i384.i = or disjoint i32 %_5.i383.i, 1065353216, !dbg !8717
  %significand.i385.i = bitcast i32 %_4.i384.i to float, !dbg !8719
  %_0.i315.i = fadd float %significand.i385.i, -1.000000e+00, !dbg !8721
  %_0.i289.i = fmul float %_0.i315.i, 0x3F9B17A960000000, !dbg !8723
  %139 = fsub float 0x3FBF9A8440000000, %_0.i289.i, !dbg !8725
  %_0.i289.1.i = fmul float %_0.i315.i, %139, !dbg !8723
  %_0.i268.1.i = fadd float %_0.i289.1.i, 0xBFD1E3F400000000, !dbg !8725
  %_0.i289.2.i = fmul float %_0.i315.i, %_0.i268.1.i, !dbg !8723
  %_0.i268.2.i = fadd float %_0.i289.2.i, 0x3FDD544F20000000, !dbg !8725
  %_0.i289.3.i = fmul float %_0.i315.i, %_0.i268.2.i, !dbg !8723
  %_0.i268.3.i = fadd float %_0.i289.3.i, 0xBFE6FC2A60000000, !dbg !8725
  %_0.i289.4.i = fmul float %_0.i315.i, %_0.i268.3.i, !dbg !8723
  %_0.i268.4.i = fadd float %_0.i289.4.i, 0x3FF714B2A0000000, !dbg !8725
  %_9.i386.i = lshr i32 %_4.i.i521.i, 23, !dbg !8727
  %_8.i387.i = or disjoint i32 %_9.i386.i, 1258291200, !dbg !8727
  %_7.i388.i = bitcast i32 %_8.i387.i to float, !dbg !8728
  %exponent.i389.i = fadd float %_7.i388.i, 0xC160000FE0000000, !dbg !8730
  %_0.i288.i = fmul float %_0.i315.i, %_0.i268.4.i, !dbg !8731
  %_0.i267.i = fadd float %exponent.i389.i, %_0.i288.i, !dbg !8733
  %_0.i310.i = fmul float %_0.i267.i, 0x4018151820000000, !dbg !8735
  %_3.i.i547.inv.i = fcmp ogt float %_0.i310.i, -1.600000e+02, !dbg !8737
  %_0.i.i555.i = select i1 %_3.i.i547.inv.i, float %_0.i310.i, float -1.600000e+02, !dbg !8737
  %_3.i.i626.inv.i = fcmp olt float %_0.i.i555.i, 2.400000e+01, !dbg !8740
  %_0.i.i634.i = select i1 %_3.i.i626.inv.i, float %_0.i.i555.i, float 2.400000e+01, !dbg !8740
  %_0.i319.i = fsub float %_0.i.i634.i, %_0.i329.i, !dbg !8743
  %_3.i251.i = fcmp ule float %_0.i319.i, %_0.i327.i, !dbg !8746
  %_0.i274.i = fadd float %_0.i327.i, %_0.i319.i, !dbg !8748
  %_0.i302.i = fmul float %_0.i274.i, %_0.i274.i, !dbg !8750
  %_0.i301.i = fmul float %_0.i326.i, %_0.i302.i, !dbg !8752
  %_4.i444.v.v.i = select i1 %_3.i251.i, float %_0.i301.i, float %_0.i319.i, !dbg !8754
  %_4.i444.v.i = fmul float %_0.i328.i, %_4.i444.v.v.i, !dbg !8754
  %140 = fcmp ugt float %_0.i319.i, %119, !dbg !8756
  %_0.i438.i = select i1 %140, float %_4.i444.v.i, float 0.000000e+00, !dbg !8758
  %_3.i.i539.i.inv = fcmp ogt float %_0.i438.i, -1.000000e+02, !dbg !8759
  %_0.i.i546.i = select i1 %_3.i.i539.i.inv, float %_0.i438.i, float -1.000000e+02, !dbg !8759
  %_3.i.i617.i = fcmp olt float %_0.i.i546.i, 0.000000e+00, !dbg !8762
  %_0.i.i625.i = select i1 %_3.i.i617.i, float %_0.i.i546.i, float 0.000000e+00, !dbg !8765
  store float %_0.i.i625.i, ptr %_3.i.i.i.i.i.i.i, align 4, !dbg !8767, !alias.scope !7943, !noalias !8269
  %_0.i347.i = load float, ptr %data.i5.i.i.i.i.i.i.i.i, align 4, !dbg !8768, !alias.scope !8770, !noalias !8773, !noundef !11
  %_3.i.i583.i = fcmp ule float %_0.i347.i, 0x3E45798EE0000000, !dbg !8774
  %_6.i.i585.i = bitcast float %_0.i347.i to i32, !dbg !8778
  %_4.i.i590.i = select i1 %_3.i.i583.i, i32 841731191, i32 %_6.i.i585.i, !dbg !8781
  %_0.i.i591.i = bitcast i32 %_4.i.i590.i to float, !dbg !8782
  %_3.i.i.i = fcmp ule float %_0.i.i591.i, 0x3810000000000000, !dbg !8784
  %_4.i.i.i = select i1 %_3.i.i.i, i32 8388608, i32 %_4.i.i590.i, !dbg !8789
  %_5.i378.i = and i32 %_4.i.i.i, 8388607, !dbg !8791
  %_4.i379.i = or disjoint i32 %_5.i378.i, 1065353216, !dbg !8791
  %significand.i.i = bitcast i32 %_4.i379.i to float, !dbg !8793
  %_0.i314.i = fadd float %significand.i.i, -1.000000e+00, !dbg !8795
  %_0.i287.i = fmul float %_0.i314.i, 0x3F9B17A960000000, !dbg !8797
  %141 = fsub float 0x3FBF9A8440000000, %_0.i287.i, !dbg !8799
  %_0.i287.1.i = fmul float %_0.i314.i, %141, !dbg !8797
  %_0.i266.1.i = fadd float %_0.i287.1.i, 0xBFD1E3F400000000, !dbg !8799
  %_0.i287.2.i = fmul float %_0.i314.i, %_0.i266.1.i, !dbg !8797
  %_0.i266.2.i = fadd float %_0.i287.2.i, 0x3FDD544F20000000, !dbg !8799
  %_0.i287.3.i = fmul float %_0.i314.i, %_0.i266.2.i, !dbg !8797
  %_0.i266.3.i = fadd float %_0.i287.3.i, 0xBFE6FC2A60000000, !dbg !8799
  %_0.i287.4.i = fmul float %_0.i314.i, %_0.i266.3.i, !dbg !8797
  %_0.i266.4.i = fadd float %_0.i287.4.i, 0x3FF714B2A0000000, !dbg !8799
  %_9.i380.i = lshr i32 %_4.i.i.i, 23, !dbg !8801
  %_8.i.i1755 = or disjoint i32 %_9.i380.i, 1258291200, !dbg !8801
  %_7.i381.i = bitcast i32 %_8.i.i1755 to float, !dbg !8802
  %exponent.i.i = fadd float %_7.i381.i, 0xC160000FE0000000, !dbg !8804
  %_0.i286.i = fmul float %_0.i314.i, %_0.i266.4.i, !dbg !8805
  %_0.i265.i = fadd float %exponent.i.i, %_0.i286.i, !dbg !8807
  %_0.i311.i = fmul float %_0.i265.i, 0x4018151820000000, !dbg !8809
  %_3.i.i574.inv.i = fcmp ogt float %_0.i311.i, -1.600000e+02, !dbg !8811
  %_0.i.i582.i = select i1 %_3.i.i574.inv.i, float %_0.i311.i, float -1.600000e+02, !dbg !8811
  %_3.i.i644.inv.i = fcmp olt float %_0.i.i582.i, 2.400000e+01, !dbg !8814
  %_0.i.i652.i = select i1 %_3.i.i644.inv.i, float %_0.i.i582.i, float 2.400000e+01, !dbg !8814
  %_0.i318.i = fsub float %_0.i.i652.i, %_0.i337.i, !dbg !8817
  %_3.i249.i = fcmp ule float %_0.i318.i, %_0.i335.i, !dbg !8820
  %_0.i273.i = fadd float %_0.i335.i, %_0.i318.i, !dbg !8822
  %_0.i298.i = fmul float %_0.i273.i, %_0.i273.i, !dbg !8824
  %_0.i297.i = fmul float %_0.i334.i, %_0.i298.i, !dbg !8826
  %_4.i431.v.v.i = select i1 %_3.i249.i, float %_0.i297.i, float %_0.i318.i, !dbg !8828
  %_4.i431.v.i = fmul float %_0.i336.i, %_4.i431.v.v.i, !dbg !8828
  %142 = fcmp ugt float %_0.i318.i, %120, !dbg !8830
  %_0.i425.i = select i1 %142, float %_4.i431.v.i, float 0.000000e+00, !dbg !8832
  %_3.i.i565.i.inv = fcmp ogt float %_0.i425.i, -1.000000e+02, !dbg !8833
  %_0.i.i573.i = select i1 %_3.i.i565.i.inv, float %_0.i425.i, float -1.000000e+02, !dbg !8833
  %_3.i.i635.i = fcmp olt float %_0.i.i573.i, 0.000000e+00, !dbg !8836
  %_0.i.i643.i = select i1 %_3.i.i635.i, float %_0.i.i573.i, float 0.000000e+00, !dbg !8839
  store float %_0.i.i643.i, ptr %_3.i.i.i.i.i, align 4, !dbg !8841, !alias.scope !7945, !noalias !8290
  %exitcond.not.i = icmp eq i64 %_9.0.i.i, %_34, !dbg !8193
  br i1 %exitcond.not.i, label %bb36.preheader.i, label %bb14.i1743, !dbg !8193

bb82.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit393.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %spec.store.select.i1754, i64 noundef %_337.1.i, i64 noundef %_337.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6b0e7efd7567d22a2220c46d452b1d05) #23, !dbg !8842, !noalias !8186
  unreachable, !dbg !8842
}

define internal fastcc void @_RINvNtCse3bfmKSZS8Y_10compressor6kernel18process_block_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_(ptr noalias noundef nonnull align 4 captures(none) %left.0, i64 noundef range(i64 8, 34359738361) %left.1, ptr dead_on_return noalias noundef nonnull readonly align 8 captures(none) dereferenceable(40) %detector, i64 noundef range(i64 1, 4294967296) %frames, i32 noundef range(i32 1, 4) %link, i1 noundef zeroext %bypass, i32 noundef %sample_rate, ptr noalias noundef nonnull align 32 dereferenceable(1568) %channel_left, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(64) %staged) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !8843 {
start:
; call <compressor::kernel::Channel<wide::f32x8_::f32x8>>::max_remaining
  %_13 = tail call fastcc noundef i32 @_RNvMNtCse3bfmKSZS8Y_10compressor6kernelINtB2_7ChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13max_remainingB4_(ptr noalias noundef readonly align 32 captures(address, read_provenance) dereferenceable(1568) %channel_left) #22, !dbg !8844
  %0 = zext i32 %_13 to i64, !dbg !8845
  %_15 = icmp samesign ugt i64 %frames, %0, !dbg !8846
  %spec.store.select = tail call i64 @llvm.umin.i64(i64 %frames, i64 %0), !dbg !8846
  %_16.not = icmp eq i32 %_13, 0, !dbg !8848
  br i1 %_16.not, label %bb8, label %bb5, !dbg !8848

bb5:                                              ; preds = %start
  %.sroa.0.0.copyload = load i64, ptr %detector, align 8, !dbg !8850
  %.sroa.4.0.detector.sroa_idx = getelementptr inbounds nuw i8, ptr %detector, i64 8, !dbg !8850
  %.sroa.4.0.copyload = load ptr, ptr %.sroa.4.0.detector.sroa_idx, align 8, !dbg !8850
  %.sroa.5.0.detector.sroa_idx = getelementptr inbounds nuw i8, ptr %detector, i64 16, !dbg !8850
  %.sroa.5.0.copyload = load i64, ptr %.sroa.5.0.detector.sroa_idx, align 8, !dbg !8850
  %.sroa.6.0.detector.sroa_idx = getelementptr inbounds nuw i8, ptr %detector, i64 24, !dbg !8850
  %.sroa.6.0.copyload = load ptr, ptr %.sroa.6.0.detector.sroa_idx, align 8, !dbg !8850
  %.sroa.7.0.detector.sroa_idx = getelementptr inbounds nuw i8, ptr %detector, i64 32, !dbg !8850
  %.sroa.7.0.copyload = load i64, ptr %.sroa.7.0.detector.sroa_idx, align 8, !dbg !8850
  %1 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1540, !dbg !8851
  %_10.i45 = load i32, ptr %1, align 4, !dbg !8851, !alias.scope !8855, !noalias !8858, !noundef !11
  %ring_length.i46 = zext i32 %_10.i45 to i64, !dbg !8851
  %2 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !8861
  %3 = bitcast <8 x float> %2 to <8 x i32>, !dbg !8869
  %4 = xor <8 x i32> %3, splat (i32 -1), !dbg !8875
  switch i32 %link, label %bb13.i270 [
    i32 1, label %bb14.i271
    i32 3, label %bb14.i271.fold.split
  ], !dbg !8877

bb13.i270:                                        ; preds = %bb5
  br label %bb14.i271, !dbg !8878

bb14.i271.fold.split:                             ; preds = %bb5
  br label %bb14.i271, !dbg !8879

bb14.i271:                                        ; preds = %bb5, %bb14.i271.fold.split, %bb13.i270
  %.pre-phi = phi <8 x i32> [ %3, %bb13.i270 ], [ %3, %bb14.i271.fold.split ], [ %4, %bb5 ]
  %_13.i262.sroa.0.0 = phi <8 x i32> [ %4, %bb13.i270 ], [ %3, %bb14.i271.fold.split ], [ %4, %bb5 ], !dbg !8880
  %_13.i47 = getelementptr inbounds nuw i8, ptr %channel_left, i64 256, !dbg !8881
  %_4.i321 = getelementptr inbounds nuw i8, ptr %channel_left, i64 448, !dbg !8883
  %_7.i322 = getelementptr inbounds nuw i8, ptr %channel_left, i64 480, !dbg !8885
  %_14.i323 = getelementptr inbounds nuw i8, ptr %channel_left, i64 288, !dbg !8886
  %_17.i324 = getelementptr inbounds nuw i8, ptr %channel_left, i64 320, !dbg !8887
  %_20.i325 = getelementptr inbounds nuw i8, ptr %channel_left, i64 352, !dbg !8888
  %_23.i326 = getelementptr inbounds nuw i8, ptr %channel_left, i64 384, !dbg !8889
  %_26.i327 = getelementptr inbounds nuw i8, ptr %channel_left, i64 416, !dbg !8890
  %5 = icmp ne ptr %.sroa.6.0.copyload, null
  %6 = icmp ne ptr %.sroa.4.0.copyload, null
  %7 = icmp slt <8 x i32> %_13.i262.sroa.0.0, zeroinitializer
  %8 = icmp slt <8 x i32> %.pre-phi, zeroinitializer
  %9 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1536
  %10 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1504
  %11 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1512
  %12 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1528
  %13 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1520
  %14 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1440
  %_47.i124 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1472
  %15 = select i1 %bypass, <8 x i32> %3, <8 x i32> %4
  %16 = lshr i64 %left.1, 3, !dbg !8891
  %17 = add nuw nsw i64 %left.1, 8, !dbg !8891
  %18 = lshr i64 %17, 3, !dbg !8891
  %19 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1444
  %20 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1448
  %21 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1452
  %22 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1456
  %23 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1460
  %24 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1464
  %25 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1468
  br label %bb18.i52, !dbg !8891

bb18.i52:                                         ; preds = %bb14.i271, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit230
  %start1.sroa.0.0.i502384 = phi i64 [ 0, %bb14.i271 ], [ %26, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit230 ]
; call <compressor::kernel::Channel<wide::f32x8_::f32x8>>::advance_ramps
  tail call fastcc void @_RNvMNtCse3bfmKSZS8Y_10compressor6kernelINtB2_7ChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13advance_rampsB4_(ptr noalias noundef nonnull align 32 dereferenceable(1568) %channel_left, i32 noundef %sample_rate) #22, !dbg !8900, !noalias !8902
  %lanes.i858.sroa.0.0.copyload = load <8 x float>, ptr %_4.i321, align 32, !dbg !8903, !alias.scope !8909, !noalias !8913
  %lanes.i852.sroa.0.0.copyload = load <8 x float>, ptr %_7.i322, align 32, !dbg !8919, !alias.scope !8924, !noalias !8928
  %lanes.i846.sroa.0.0.copyload = load <8 x float>, ptr %_13.i47, align 32, !dbg !8932, !alias.scope !8937, !noalias !8941
  %lanes.i840.sroa.0.0.copyload = load <8 x float>, ptr %_14.i323, align 32, !dbg !8945, !alias.scope !8950, !noalias !8954
  %lanes.i834.sroa.0.0.copyload = load <8 x float>, ptr %_17.i324, align 32, !dbg !8958, !alias.scope !8963, !noalias !8967
  %lanes.i828.sroa.0.0.copyload = load <8 x float>, ptr %_20.i325, align 32, !dbg !8971, !alias.scope !8976, !noalias !8980
  %lanes.i822.sroa.0.0.copyload = load <8 x float>, ptr %_23.i326, align 32, !dbg !8984, !alias.scope !8989, !noalias !8993
  %lanes.i816.sroa.0.0.copyload = load <8 x float>, ptr %_26.i327, align 32, !dbg !8997, !alias.scope !9002, !noalias !9006
  %26 = add nuw nsw i64 %start1.sroa.0.0.i502384, 1, !dbg !9010
  %27 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i852.sroa.0.0.copyload, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !9016
  %28 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i852.sroa.0.0.copyload, <8 x float> zeroinitializer, i8 0), !dbg !9022
  %29 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i858.sroa.0.0.copyload, <8 x float> zeroinitializer, i8 0), !dbg !9028
  %slot.i53 = shl nuw nsw i64 %start1.sroa.0.0.i502384, 3, !dbg !9034
  %exitcond = icmp eq i64 %start1.sroa.0.0.i502384, %18, !dbg !9035
  br i1 %exitcond, label %bb20.i132, label %bb21.i55, !dbg !9035, !prof !161

bb21.i55:                                         ; preds = %bb18.i52
  %_64.i57 = getelementptr inbounds nuw float, ptr %left.0, i64 %slot.i53, !dbg !9041
  %exitcond2664.not = icmp eq i64 %start1.sroa.0.0.i502384, %16, !dbg !9046
  br i1 %exitcond2664.not, label %bb2.i899, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit902, !dbg !9046, !prof !161

bb2.i899:                                         ; preds = %bb21.i55
  %30 = and i64 %left.1, 7, !dbg !8891
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %30, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !9051, !noalias !9052
  unreachable, !dbg !9051

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit902: ; preds = %bb21.i55
  %lanes.i895.sroa.0.0.copyload = load <8 x float>, ptr %_64.i57, align 4, !dbg !9056, !alias.scope !9060, !noalias !9064
  switch i64 %.sroa.0.0.copyload, label %default.unreachable.i.i131 [
    i64 0, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i72
    i64 1, label %bb3.i.i130
    i64 2, label %bb2.i.i59
  ], !dbg !9066

default.unreachable.i.i131:                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit902
  unreachable

bb3.i.i130:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit902
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i72, !dbg !9069

bb2.i.i59:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit902
  tail call void @llvm.assume(i1 %5)
  %_25.i.i63 = icmp ugt i64 %slot.i53, %.sroa.5.0.copyload, !dbg !9070
  br i1 %_25.i.i63, label %bb17.i.i129, label %bb18.i.i64, !dbg !9070, !prof !161

bb18.i.i64:                                       ; preds = %bb2.i.i59
  tail call void @llvm.assume(i1 %6)
  %_28.i.i66 = sub nuw i64 %.sroa.5.0.copyload, %slot.i53, !dbg !9073
  %_8.i889 = icmp samesign ugt i64 %_28.i.i66, 7, !dbg !9074
  br i1 %_8.i889, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit893, label %bb2.i890, !dbg !9074, !prof !2116

bb2.i890:                                         ; preds = %bb18.i.i64
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_28.i.i66, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !9079, !noalias !9080
  unreachable, !dbg !9079

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit893: ; preds = %bb18.i.i64
  %_32.i.i67 = getelementptr inbounds nuw float, ptr %.sroa.4.0.copyload, i64 %slot.i53, !dbg !9089
  %lanes.i886.sroa.0.0.copyload = load <8 x float>, ptr %_32.i.i67, align 4, !dbg !9091, !alias.scope !9095, !noalias !9099
  %_33.i.i68 = icmp ugt i64 %slot.i53, %.sroa.7.0.copyload, !dbg !9101
  br i1 %_33.i.i68, label %bb19.i.i128, label %bb20.i.i69, !dbg !9101, !prof !161

bb17.i.i129:                                      ; preds = %bb2.i.i59
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 34359738361) %slot.i53, i64 noundef %.sroa.5.0.copyload, i64 noundef %.sroa.5.0.copyload, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a0a4023c361dd50869bbb08b76482f50) #23, !dbg !9104, !noalias !9105
  unreachable, !dbg !9104

bb20.i.i69:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit893
  %_36.i.i70 = sub nuw i64 %.sroa.7.0.copyload, %slot.i53, !dbg !9107
  %_8.i880 = icmp samesign ugt i64 %_36.i.i70, 7, !dbg !9108
  br i1 %_8.i880, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit884, label %bb2.i881, !dbg !9108, !prof !2116

bb2.i881:                                         ; preds = %bb20.i.i69
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_36.i.i70, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !9113, !noalias !9114
  unreachable, !dbg !9113

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit884: ; preds = %bb20.i.i69
  %_40.i.i71 = getelementptr inbounds nuw float, ptr %.sroa.6.0.copyload, i64 %slot.i53, !dbg !9118
  %lanes.i877.sroa.0.0.copyload = load <8 x float>, ptr %_40.i.i71, align 4, !dbg !9120, !alias.scope !9124, !noalias !9128
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i72, !dbg !9130

bb19.i.i128:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit893
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 34359738361) %slot.i53, i64 noundef %.sroa.7.0.copyload, i64 noundef %.sroa.7.0.copyload, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5807a2a07e9839e3d32631b4243eb17f) #23, !dbg !9131, !noalias !9105
  unreachable, !dbg !9131

_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i72: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit884, %bb3.i.i130, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit902
  %.sroa.01437.0 = phi <8 x float> [ %lanes.i895.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit902 ], [ zeroinitializer, %bb3.i.i130 ], [ %lanes.i877.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit884 ], !dbg !9132
  %.sroa.01433.0 = phi <8 x float> [ %lanes.i895.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit902 ], [ zeroinitializer, %bb3.i.i130 ], [ %lanes.i886.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit884 ], !dbg !9132
  %31 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %.sroa.01433.0), !dbg !9133
  %32 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %.sroa.01437.0), !dbg !9139
  %33 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %31, <8 x float> %32), !dbg !9145
  %34 = fmul <8 x float> %31, splat (float 5.000000e-01), !dbg !9150
  %35 = fmul <8 x float> %32, splat (float 5.000000e-01), !dbg !9155
  %36 = fadd <8 x float> %34, %35, !dbg !9160
  %37 = select <8 x i1> %7, <8 x float> %36, <8 x float> %33, !dbg !9165
  %38 = select <8 x i1> %8, <8 x float> %37, <8 x float> %31, !dbg !9170
  %_28.i73 = load i32, ptr %9, align 32, !dbg !9175, !alias.scope !8855, !noalias !8858, !noundef !11
  %write.i74 = zext i32 %_28.i73 to i64, !dbg !9175
  %39 = add nuw nsw i64 %write.i74, 1, !dbg !9177
  %_30.i75 = icmp eq i64 %39, %ring_length.i46, !dbg !9179
  %spec.store.select.i76 = select i1 %_30.i75, i64 0, i64 %39, !dbg !9179
  %_95.1.i77 = load i64, ptr %11, align 8, !dbg !9181, !alias.scope !8855, !noalias !8858, !noundef !11
  %_34.i78 = shl nuw nsw i64 %write.i74, 3, !dbg !9183
  %_65.i79 = icmp ugt i64 %_34.i78, %_95.1.i77, !dbg !9184
  br i1 %_65.i79, label %bb22.i127, label %bb23.i80, !dbg !9184, !prof !161

bb20.i132:                                        ; preds = %bb18.i52
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %slot.i53, i64 noundef range(i64 8, 34359738361) %left.1, i64 noundef range(i64 8, 34359738361) %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_3b7137b1f9053f34ed465a67bd56da03) #23, !dbg !9189, !noalias !8902
  unreachable, !dbg !9189

bb23.i80:                                         ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i72
  %_68.i82 = sub nuw i64 %_95.1.i77, %_34.i78, !dbg !9190
  %_8.i1233 = icmp samesign ugt i64 %_68.i82, 7, !dbg !9191
  br i1 %_8.i1233, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1236, label %bb2.i1234, !dbg !9191, !prof !2116

bb2.i1234:                                        ; preds = %bb23.i80
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_68.i82, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !9196, !noalias !9197
  unreachable, !dbg !9196

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1236: ; preds = %bb23.i80
  %_95.0.i81 = load ptr, ptr %10, align 32, !dbg !9181, !alias.scope !8855, !noalias !8858, !nonnull !11, !noundef !11
  %_72.i83 = getelementptr inbounds nuw float, ptr %_95.0.i81, i64 %_34.i78, !dbg !9201
  store <8 x float> %lanes.i895.sroa.0.0.copyload, ptr %_72.i83, align 4, !dbg !9206, !alias.scope !9210, !noalias !9214
  %_96.1.i84 = load i64, ptr %12, align 8, !dbg !9216, !alias.scope !8855, !noalias !8858, !noundef !11
  %_73.i85 = icmp ugt i64 %_34.i78, %_96.1.i84, !dbg !9217
  br i1 %_73.i85, label %bb24.i126, label %bb25.i86, !dbg !9217, !prof !161

bb22.i127:                                        ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i72
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_34.i78, i64 noundef %_95.1.i77, i64 noundef %_95.1.i77, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1992fd841b4d79a3d694b8d46102fa04) #23, !dbg !9221, !noalias !8902
  unreachable, !dbg !9221

bb25.i86:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1236
  %_76.i88 = sub nuw i64 %_96.1.i84, %_34.i78, !dbg !9222
  %_8.i1228 = icmp samesign ugt i64 %_76.i88, 7, !dbg !9223
  br i1 %_8.i1228, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1231, label %bb2.i1229, !dbg !9223, !prof !2116

bb2.i1229:                                        ; preds = %bb25.i86
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_76.i88, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !9228, !noalias !9229
  unreachable, !dbg !9228

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1231: ; preds = %bb25.i86
  %_96.0.i87 = load ptr, ptr %13, align 16, !dbg !9216, !alias.scope !8855, !noalias !8858, !nonnull !11, !noundef !11
  %_80.i89 = getelementptr inbounds nuw float, ptr %_96.0.i87, i64 %_34.i78, !dbg !9233
  store <8 x float> %38, ptr %_80.i89, align 4, !dbg !9238, !alias.scope !9242, !noalias !9246
  %_97.1.i90 = load i64, ptr %11, align 8, !dbg !9248, !alias.scope !8855, !noalias !8858, !noundef !11
  %_40.i91 = shl nuw nsw i64 %spec.store.select.i76, 3, !dbg !9249
  %_81.i92 = icmp ugt i64 %_40.i91, %_97.1.i90, !dbg !9250
  br i1 %_81.i92, label %bb26.i125, label %bb27.i93, !dbg !9250, !prof !161

bb24.i126:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1236
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_34.i78, i64 noundef %_96.1.i84, i64 noundef %_96.1.i84, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4989c99a90ecaa46922498d8532f27e6) #23, !dbg !9254, !noalias !8902
  unreachable, !dbg !9254

bb27.i93:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1231
  %_84.i95 = sub nuw i64 %_97.1.i90, %_40.i91, !dbg !9255
  %_8.i872 = icmp samesign ugt i64 %_84.i95, 7, !dbg !9256
  br i1 %_8.i872, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit875, label %bb2.i, !dbg !9256, !prof !2116

bb2.i:                                            ; preds = %bb27.i93
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_84.i95, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !9261, !noalias !9262
  unreachable, !dbg !9261

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit875: ; preds = %bb27.i93
  %_97.0.i94 = load ptr, ptr %10, align 32, !dbg !9248, !alias.scope !8855, !noalias !8858, !nonnull !11, !noundef !11
  %_88.i96 = getelementptr inbounds nuw float, ptr %_97.0.i94, i64 %_40.i91, !dbg !9266
  %lanes.i870.sroa.0.0.copyload = load <8 x float>, ptr %_88.i96, align 4, !dbg !9271, !alias.scope !9275, !noalias !9279
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9281), !dbg !9284
  %_5.i7.i97 = load i32, ptr %1, align 4, !dbg !9286, !alias.scope !9288, !noalias !9289, !noundef !11
  %_38.1.i.i117 = load i64, ptr %12, align 8
  %_38.0.i.i121 = load ptr, ptr %13, align 16, !nonnull !11
  %_17.i10.i109 = load i32, ptr %14, align 4, !dbg !9292, !alias.scope !9288, !noalias !9289, !noundef !11
  %delay.i.i110 = zext i32 %_17.i10.i109 to i64, !dbg !9292
  %_20.not.i.i111 = icmp ult i32 %_28.i73, %_17.i10.i109, !dbg !9293
  %narrow = select i1 %_20.not.i.i111, i32 %_5.i7.i97, i32 0, !dbg !9293
  %_21.i11.i112 = zext i32 %narrow to i64, !dbg !9293
  %write.pn.i.i113 = sub nsw i64 %write.i74, %delay.i.i110, !dbg !9293
  %tap.sroa.0.0.i.i114 = add nsw i64 %write.pn.i.i113, %_21.i11.i112, !dbg !9294
  %_24.i12.i115 = shl nsw i64 %tap.sroa.0.0.i.i114, 3, !dbg !9295
  %_26.i.i118 = icmp ult i64 %_24.i12.i115, %_38.1.i.i117, !dbg !9296
  br i1 %_26.i.i118, label %bb10.i.i120, label %panic1.i.i119, !dbg !9296

bb10.i.i120:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit875
  %40 = getelementptr inbounds nuw float, ptr %_38.0.i.i121, i64 %_24.i12.i115, !dbg !9296
  %_22.i14.i122 = load float, ptr %40, align 4, !dbg !9296, !noalias !9297, !noundef !11
  %_17.i10.i109.1 = load i32, ptr %19, align 4, !dbg !9292, !alias.scope !9288, !noalias !9289, !noundef !11
  %delay.i.i110.1 = zext i32 %_17.i10.i109.1 to i64, !dbg !9292
  %_20.not.i.i111.1 = icmp ult i32 %_28.i73, %_17.i10.i109.1, !dbg !9293
  %narrow.1 = select i1 %_20.not.i.i111.1, i32 %_5.i7.i97, i32 0, !dbg !9293
  %_21.i11.i112.1 = zext i32 %narrow.1 to i64, !dbg !9293
  %write.pn.i.i113.1 = sub nsw i64 %write.i74, %delay.i.i110.1, !dbg !9293
  %tap.sroa.0.0.i.i114.1 = add nsw i64 %write.pn.i.i113.1, %_21.i11.i112.1, !dbg !9294
  %_24.i12.i115.1 = shl nsw i64 %tap.sroa.0.0.i.i114.1, 3, !dbg !9295
  %_23.i13.i116.1 = or disjoint i64 %_24.i12.i115.1, 1, !dbg !9295
  %_26.i.i118.1 = icmp ult i64 %_23.i13.i116.1, %_38.1.i.i117, !dbg !9296
  br i1 %_26.i.i118.1, label %bb10.i.i120.1, label %panic1.i.i119, !dbg !9296

bb10.i.i120.1:                                    ; preds = %bb10.i.i120
  %41 = getelementptr inbounds nuw float, ptr %_38.0.i.i121, i64 %_23.i13.i116.1, !dbg !9296
  %_22.i14.i122.1 = load float, ptr %41, align 4, !dbg !9296, !noalias !9297, !noundef !11
  %_17.i10.i109.2 = load i32, ptr %20, align 4, !dbg !9292, !alias.scope !9288, !noalias !9289, !noundef !11
  %delay.i.i110.2 = zext i32 %_17.i10.i109.2 to i64, !dbg !9292
  %_20.not.i.i111.2 = icmp ult i32 %_28.i73, %_17.i10.i109.2, !dbg !9293
  %narrow.2 = select i1 %_20.not.i.i111.2, i32 %_5.i7.i97, i32 0, !dbg !9293
  %_21.i11.i112.2 = zext i32 %narrow.2 to i64, !dbg !9293
  %write.pn.i.i113.2 = sub nsw i64 %write.i74, %delay.i.i110.2, !dbg !9293
  %tap.sroa.0.0.i.i114.2 = add nsw i64 %write.pn.i.i113.2, %_21.i11.i112.2, !dbg !9294
  %_24.i12.i115.2 = shl nsw i64 %tap.sroa.0.0.i.i114.2, 3, !dbg !9295
  %_23.i13.i116.2 = or disjoint i64 %_24.i12.i115.2, 2, !dbg !9295
  %_26.i.i118.2 = icmp ult i64 %_23.i13.i116.2, %_38.1.i.i117, !dbg !9296
  br i1 %_26.i.i118.2, label %bb10.i.i120.2, label %panic1.i.i119, !dbg !9296

bb10.i.i120.2:                                    ; preds = %bb10.i.i120.1
  %42 = getelementptr inbounds nuw float, ptr %_38.0.i.i121, i64 %_23.i13.i116.2, !dbg !9296
  %_22.i14.i122.2 = load float, ptr %42, align 4, !dbg !9296, !noalias !9297, !noundef !11
  %_17.i10.i109.3 = load i32, ptr %21, align 4, !dbg !9292, !alias.scope !9288, !noalias !9289, !noundef !11
  %delay.i.i110.3 = zext i32 %_17.i10.i109.3 to i64, !dbg !9292
  %_20.not.i.i111.3 = icmp ult i32 %_28.i73, %_17.i10.i109.3, !dbg !9293
  %narrow.3 = select i1 %_20.not.i.i111.3, i32 %_5.i7.i97, i32 0, !dbg !9293
  %_21.i11.i112.3 = zext i32 %narrow.3 to i64, !dbg !9293
  %write.pn.i.i113.3 = sub nsw i64 %write.i74, %delay.i.i110.3, !dbg !9293
  %tap.sroa.0.0.i.i114.3 = add nsw i64 %write.pn.i.i113.3, %_21.i11.i112.3, !dbg !9294
  %_24.i12.i115.3 = shl nsw i64 %tap.sroa.0.0.i.i114.3, 3, !dbg !9295
  %_23.i13.i116.3 = or disjoint i64 %_24.i12.i115.3, 3, !dbg !9295
  %_26.i.i118.3 = icmp ult i64 %_23.i13.i116.3, %_38.1.i.i117, !dbg !9296
  br i1 %_26.i.i118.3, label %bb10.i.i120.3, label %panic1.i.i119, !dbg !9296

bb10.i.i120.3:                                    ; preds = %bb10.i.i120.2
  %43 = getelementptr inbounds nuw float, ptr %_38.0.i.i121, i64 %_23.i13.i116.3, !dbg !9296
  %_22.i14.i122.3 = load float, ptr %43, align 4, !dbg !9296, !noalias !9297, !noundef !11
  %_17.i10.i109.4 = load i32, ptr %22, align 4, !dbg !9292, !alias.scope !9288, !noalias !9289, !noundef !11
  %delay.i.i110.4 = zext i32 %_17.i10.i109.4 to i64, !dbg !9292
  %_20.not.i.i111.4 = icmp ult i32 %_28.i73, %_17.i10.i109.4, !dbg !9293
  %narrow.4 = select i1 %_20.not.i.i111.4, i32 %_5.i7.i97, i32 0, !dbg !9293
  %_21.i11.i112.4 = zext i32 %narrow.4 to i64, !dbg !9293
  %write.pn.i.i113.4 = sub nsw i64 %write.i74, %delay.i.i110.4, !dbg !9293
  %tap.sroa.0.0.i.i114.4 = add nsw i64 %write.pn.i.i113.4, %_21.i11.i112.4, !dbg !9294
  %_24.i12.i115.4 = shl nsw i64 %tap.sroa.0.0.i.i114.4, 3, !dbg !9295
  %_23.i13.i116.4 = or disjoint i64 %_24.i12.i115.4, 4, !dbg !9295
  %_26.i.i118.4 = icmp ult i64 %_23.i13.i116.4, %_38.1.i.i117, !dbg !9296
  br i1 %_26.i.i118.4, label %bb10.i.i120.4, label %panic1.i.i119, !dbg !9296

bb10.i.i120.4:                                    ; preds = %bb10.i.i120.3
  %44 = getelementptr inbounds nuw float, ptr %_38.0.i.i121, i64 %_23.i13.i116.4, !dbg !9296
  %_22.i14.i122.4 = load float, ptr %44, align 4, !dbg !9296, !noalias !9297, !noundef !11
  %_17.i10.i109.5 = load i32, ptr %23, align 4, !dbg !9292, !alias.scope !9288, !noalias !9289, !noundef !11
  %delay.i.i110.5 = zext i32 %_17.i10.i109.5 to i64, !dbg !9292
  %_20.not.i.i111.5 = icmp ult i32 %_28.i73, %_17.i10.i109.5, !dbg !9293
  %narrow.5 = select i1 %_20.not.i.i111.5, i32 %_5.i7.i97, i32 0, !dbg !9293
  %_21.i11.i112.5 = zext i32 %narrow.5 to i64, !dbg !9293
  %write.pn.i.i113.5 = sub nsw i64 %write.i74, %delay.i.i110.5, !dbg !9293
  %tap.sroa.0.0.i.i114.5 = add nsw i64 %write.pn.i.i113.5, %_21.i11.i112.5, !dbg !9294
  %_24.i12.i115.5 = shl nsw i64 %tap.sroa.0.0.i.i114.5, 3, !dbg !9295
  %_23.i13.i116.5 = or disjoint i64 %_24.i12.i115.5, 5, !dbg !9295
  %_26.i.i118.5 = icmp ult i64 %_23.i13.i116.5, %_38.1.i.i117, !dbg !9296
  br i1 %_26.i.i118.5, label %bb10.i.i120.5, label %panic1.i.i119, !dbg !9296

bb10.i.i120.5:                                    ; preds = %bb10.i.i120.4
  %45 = getelementptr inbounds nuw float, ptr %_38.0.i.i121, i64 %_23.i13.i116.5, !dbg !9296
  %_22.i14.i122.5 = load float, ptr %45, align 4, !dbg !9296, !noalias !9297, !noundef !11
  %_17.i10.i109.6 = load i32, ptr %24, align 4, !dbg !9292, !alias.scope !9288, !noalias !9289, !noundef !11
  %delay.i.i110.6 = zext i32 %_17.i10.i109.6 to i64, !dbg !9292
  %_20.not.i.i111.6 = icmp ult i32 %_28.i73, %_17.i10.i109.6, !dbg !9293
  %narrow.6 = select i1 %_20.not.i.i111.6, i32 %_5.i7.i97, i32 0, !dbg !9293
  %_21.i11.i112.6 = zext i32 %narrow.6 to i64, !dbg !9293
  %write.pn.i.i113.6 = sub nsw i64 %write.i74, %delay.i.i110.6, !dbg !9293
  %tap.sroa.0.0.i.i114.6 = add nsw i64 %write.pn.i.i113.6, %_21.i11.i112.6, !dbg !9294
  %_24.i12.i115.6 = shl nsw i64 %tap.sroa.0.0.i.i114.6, 3, !dbg !9295
  %_23.i13.i116.6 = or disjoint i64 %_24.i12.i115.6, 6, !dbg !9295
  %_26.i.i118.6 = icmp ult i64 %_23.i13.i116.6, %_38.1.i.i117, !dbg !9296
  br i1 %_26.i.i118.6, label %bb6.i.i108.7, label %panic1.i.i119, !dbg !9296

bb6.i.i108.7:                                     ; preds = %bb10.i.i120.5
  %_17.i10.i109.7 = load i32, ptr %25, align 4, !dbg !9292, !alias.scope !9288, !noalias !9289, !noundef !11
  %delay.i.i110.7 = zext i32 %_17.i10.i109.7 to i64, !dbg !9292
  %_20.not.i.i111.7 = icmp ult i32 %_28.i73, %_17.i10.i109.7, !dbg !9293
  %narrow.7 = select i1 %_20.not.i.i111.7, i32 %_5.i7.i97, i32 0, !dbg !9293
  %_21.i11.i112.7 = zext i32 %narrow.7 to i64, !dbg !9293
  %write.pn.i.i113.7 = sub nsw i64 %write.i74, %delay.i.i110.7, !dbg !9293
  %tap.sroa.0.0.i.i114.7 = add nsw i64 %write.pn.i.i113.7, %_21.i11.i112.7, !dbg !9294
  %_24.i12.i115.7 = shl nsw i64 %tap.sroa.0.0.i.i114.7, 3, !dbg !9295
  %_23.i13.i116.7 = or disjoint i64 %_24.i12.i115.7, 7, !dbg !9295
  %_26.i.i118.7 = icmp ult i64 %_23.i13.i116.7, %_38.1.i.i117, !dbg !9296
  br i1 %_26.i.i118.7, label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit230, label %panic1.i.i119, !dbg !9296

panic1.i.i119:                                    ; preds = %bb6.i.i108.7, %bb10.i.i120.5, %bb10.i.i120.4, %bb10.i.i120.3, %bb10.i.i120.2, %bb10.i.i120.1, %bb10.i.i120, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit875
  %_23.i13.i116.lcssa = phi i64 [ %_24.i12.i115, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit875 ], [ %_23.i13.i116.1, %bb10.i.i120 ], [ %_23.i13.i116.2, %bb10.i.i120.1 ], [ %_23.i13.i116.3, %bb10.i.i120.2 ], [ %_23.i13.i116.4, %bb10.i.i120.3 ], [ %_23.i13.i116.5, %bb10.i.i120.4 ], [ %_23.i13.i116.6, %bb10.i.i120.5 ], [ %_23.i13.i116.7, %bb6.i.i108.7 ], !dbg !9295
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_23.i13.i116.lcssa, i64 noundef %_38.1.i.i117, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a529a882edb544ae33c686afc6011081) #23, !dbg !9296, !noalias !9297
  unreachable, !dbg !9296

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit230: ; preds = %bb6.i.i108.7
  %46 = getelementptr inbounds nuw float, ptr %_38.0.i.i121, i64 %_23.i13.i116.6, !dbg !9296
  %_22.i14.i122.6 = load float, ptr %46, align 4, !dbg !9296, !noalias !9297, !noundef !11
  %lanes.i864.sroa.0.0.vec.insert = insertelement <8 x float> poison, float %_22.i14.i122, i64 0, !dbg !9298
  %lanes.i864.sroa.0.4.vec.insert = insertelement <8 x float> %lanes.i864.sroa.0.0.vec.insert, float %_22.i14.i122.1, i64 1, !dbg !9298
  %lanes.i864.sroa.0.8.vec.insert = insertelement <8 x float> %lanes.i864.sroa.0.4.vec.insert, float %_22.i14.i122.2, i64 2, !dbg !9298
  %lanes.i864.sroa.0.12.vec.insert = insertelement <8 x float> %lanes.i864.sroa.0.8.vec.insert, float %_22.i14.i122.3, i64 3, !dbg !9298
  %lanes.i864.sroa.0.16.vec.insert = insertelement <8 x float> %lanes.i864.sroa.0.12.vec.insert, float %_22.i14.i122.4, i64 4, !dbg !9298
  %lanes.i864.sroa.0.20.vec.insert = insertelement <8 x float> %lanes.i864.sroa.0.16.vec.insert, float %_22.i14.i122.5, i64 5, !dbg !9298
  %lanes.i864.sroa.0.24.vec.insert = insertelement <8 x float> %lanes.i864.sroa.0.20.vec.insert, float %_22.i14.i122.6, i64 6, !dbg !9298
  %47 = getelementptr inbounds nuw float, ptr %_38.0.i.i121, i64 %_23.i13.i116.7, !dbg !9296
  %_22.i14.i122.7 = load float, ptr %47, align 4, !dbg !9296, !noalias !9297, !noundef !11
  %lanes.i864.sroa.0.28.vec.insert = insertelement <8 x float> %lanes.i864.sroa.0.24.vec.insert, float %_22.i14.i122.7, i64 7, !dbg !9298
  %48 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i864.sroa.0.28.vec.insert, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !9303
  %49 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %48, <8 x float> splat (float 0x3810000000000000)), !dbg !9312
  %50 = bitcast <8 x float> %49 to <4 x i64>, !dbg !9319
  %51 = and <4 x i64> %50, splat (i64 36028792732385279), !dbg !9320
  %52 = or disjoint <4 x i64> %51, splat (i64 4575657222473777152), !dbg !9325
  %53 = bitcast <4 x i64> %52 to <8 x float>, !dbg !9329
  %54 = fadd <8 x float> %53, splat (float -1.000000e+00), !dbg !9330
  %55 = fmul <8 x float> %54, splat (float 0xBF9B17A960000000), !dbg !9335
  %56 = fadd <8 x float> %55, splat (float 0x3FBF9A8440000000), !dbg !9340
  %57 = fmul <8 x float> %54, %56, !dbg !9335
  %58 = fadd <8 x float> %57, splat (float 0xBFD1E3F400000000), !dbg !9340
  %59 = fmul <8 x float> %54, %58, !dbg !9335
  %60 = fadd <8 x float> %59, splat (float 0x3FDD544F20000000), !dbg !9340
  %61 = fmul <8 x float> %54, %60, !dbg !9335
  %62 = fadd <8 x float> %61, splat (float 0xBFE6FC2A60000000), !dbg !9340
  %63 = fmul <8 x float> %54, %62, !dbg !9335
  %64 = fadd <8 x float> %63, splat (float 0x3FF714B2A0000000), !dbg !9340
  %65 = bitcast <8 x float> %49 to <8 x i32>, !dbg !9345
  %_3.i1355 = lshr <8 x i32> %65, splat (i32 23), !dbg !9349
  %66 = or disjoint <8 x i32> %_3.i1355, splat (i32 1258291200), !dbg !9350
  %67 = bitcast <8 x i32> %66 to <8 x float>, !dbg !9354
  %68 = fadd <8 x float> %67, splat (float 0xC160000FE0000000), !dbg !9355
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9359), !dbg !9362
  %_6.i161.sroa.0.0.copyload = load <8 x float>, ptr %_47.i124, align 32, !dbg !9364, !noalias !9366
  %69 = fmul <8 x float> %54, %64, !dbg !9369
  %70 = fadd <8 x float> %68, %69, !dbg !9374
  %71 = fmul <8 x float> %70, splat (float 0x4018151820000000), !dbg !9379
  %72 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %71, <8 x float> splat (float -1.600000e+02)), !dbg !9384
  %73 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %72, <8 x float> splat (float 2.400000e+01)), !dbg !9389
  %74 = fsub <8 x float> %73, %lanes.i846.sroa.0.0.copyload, !dbg !9394
  %75 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %74, <8 x float> %lanes.i834.sroa.0.0.copyload, i8 30), !dbg !9400
  %76 = fneg <8 x float> %lanes.i834.sroa.0.0.copyload, !dbg !9406
  %77 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %74, <8 x float> %76, i8 18), !dbg !9411
  %78 = fadd <8 x float> %lanes.i834.sroa.0.0.copyload, %74, !dbg !9417
  %79 = fmul <8 x float> %78, %78, !dbg !9422
  %80 = fmul <8 x float> %lanes.i828.sroa.0.0.copyload, %79, !dbg !9427
  %81 = bitcast <8 x float> %75 to <8 x i32>, !dbg !9432
  %82 = icmp slt <8 x i32> %81, zeroinitializer, !dbg !9436
  %.v = select <8 x i1> %82, <8 x float> %74, <8 x float> %80, !dbg !9436
  %83 = fmul <8 x float> %lanes.i840.sroa.0.0.copyload, %.v, !dbg !9436
  %84 = bitcast <8 x float> %77 to <8 x i32>, !dbg !9438
  %85 = icmp slt <8 x i32> %84, zeroinitializer, !dbg !9442
  %86 = select <8 x i1> %85, <8 x float> zeroinitializer, <8 x float> %83, !dbg !9442
  %87 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %86, <8 x float> splat (float -1.000000e+02)), !dbg !9444
  %88 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %87, <8 x float> zeroinitializer), !dbg !9449
  %89 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %88, <8 x float> %_6.i161.sroa.0.0.copyload, i8 17), !dbg !9454
  %90 = bitcast <8 x float> %89 to <8 x i32>, !dbg !9460
  %91 = icmp slt <8 x i32> %90, zeroinitializer, !dbg !9464
  %92 = select <8 x i1> %91, <8 x float> %lanes.i822.sroa.0.0.copyload, <8 x float> %lanes.i816.sroa.0.0.copyload, !dbg !9464
  %93 = fsub <8 x float> %88, %_6.i161.sroa.0.0.copyload, !dbg !9466
  %94 = fmul <8 x float> %93, %92, !dbg !9472
  %95 = fadd <8 x float> %_6.i161.sroa.0.0.copyload, %94, !dbg !9477
  %96 = bitcast <8 x float> %95 to <8 x i32>, !dbg !9481
  %97 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %95), !dbg !9487
  %98 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %97, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !9489
  %99 = bitcast <8 x float> %98 to <8 x i32>, !dbg !9495
  %100 = xor <8 x i32> %99, splat (i32 -1), !dbg !9501
  %101 = and <8 x i32> %100, %96, !dbg !9503
  store <8 x i32> %101, ptr %_47.i124, align 32, !dbg !9507, !alias.scope !9508, !noalias !9510
  %102 = bitcast <8 x i32> %101 to <8 x float>, !dbg !9511
  %103 = fadd <8 x float> %lanes.i858.sroa.0.0.copyload, %102, !dbg !9512
  %104 = fmul <8 x float> %103, splat (float 0x3FC542A5A0000000), !dbg !9519
  %105 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %104, <8 x float> splat (float -1.260000e+02)), !dbg !9525
  %106 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %105, <8 x float> splat (float 1.270000e+02)), !dbg !9531
  %107 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %106), !dbg !9536
  %108 = fsub <8 x float> %106, %107, !dbg !9541
  %109 = fmul <8 x float> %108, splat (float 0x3F5E974FA0000000), !dbg !9546
  %110 = fadd <8 x float> %109, splat (float 0x3F82778560000000), !dbg !9551
  %111 = fmul <8 x float> %108, %110, !dbg !9546
  %112 = fadd <8 x float> %111, splat (float 0x3FAC91CE60000000), !dbg !9551
  %113 = fmul <8 x float> %108, %112, !dbg !9546
  %114 = fadd <8 x float> %113, splat (float 0x3FCEBDB560000000), !dbg !9551
  %115 = fmul <8 x float> %108, %114, !dbg !9546
  %116 = fadd <8 x float> %115, splat (float 0x3FE62E4BA0000000), !dbg !9551
  %117 = fmul <8 x float> %108, %116, !dbg !9556
  %118 = fadd <8 x float> %117, splat (float 1.000000e+00), !dbg !9561
  %119 = fadd <8 x float> %107, splat (float 0x4160000FE0000000), !dbg !9566
  %120 = bitcast <8 x float> %119 to <8 x i32>, !dbg !9571
  %_3.i1356 = shl <8 x i32> %120, splat (i32 23), !dbg !9575
  %121 = bitcast <8 x i32> %_3.i1356 to <8 x float>, !dbg !9576
  %122 = fmul <8 x float> %118, %121, !dbg !9578
  %123 = fmul <8 x float> %lanes.i870.sroa.0.0.copyload, %122, !dbg !9582
  %124 = fsub <8 x float> %123, %lanes.i870.sroa.0.0.copyload, !dbg !9587
  %125 = fmul <8 x float> %lanes.i852.sroa.0.0.copyload, %124, !dbg !9593
  %126 = fadd <8 x float> %lanes.i870.sroa.0.0.copyload, %125, !dbg !9598
  %127 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %102, <8 x float> zeroinitializer, i8 0), !dbg !9602
  %128 = bitcast <8 x float> %127 to <8 x i32>, !dbg !9608
  %129 = bitcast <8 x float> %29 to <8 x i32>, !dbg !9608
  %130 = and <8 x i32> %128, %129, !dbg !9612
  %131 = bitcast <8 x float> %28 to <8 x i32>, !dbg !9614
  %132 = or <8 x i32> %15, %131, !dbg !9618
  %133 = or <8 x i32> %132, %130, !dbg !9620
  %134 = bitcast <8 x float> %27 to <8 x i32>, !dbg !9625
  %135 = icmp slt <8 x i32> %134, zeroinitializer, !dbg !9629
  %136 = select <8 x i1> %135, <8 x float> %123, <8 x float> %126, !dbg !9629
  %137 = icmp slt <8 x i32> %133, zeroinitializer, !dbg !9631
  %138 = select <8 x i1> %137, <8 x float> %lanes.i870.sroa.0.0.copyload, <8 x float> %136, !dbg !9631
  store <8 x float> %138, ptr %_64.i57, align 4, !dbg !9636, !alias.scope !9642, !noalias !9646
  %139 = trunc i64 %spec.store.select.i76 to i32, !dbg !9650
  store i32 %139, ptr %9, align 32, !dbg !9650, !alias.scope !8855, !noalias !8858
  %exitcond2665.not = icmp eq i64 %26, %spec.store.select, !dbg !9651
  br i1 %exitcond2665.not, label %bb8, label %bb18.i52, !dbg !8891

bb26.i125:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1231
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_40.i91, i64 noundef %_97.1.i90, i64 noundef %_97.1.i90, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b508431e8547d856227f5f364b1d083b) #23, !dbg !9654, !noalias !8902
  unreachable, !dbg !9654

bb8:                                              ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit230, %start
  br i1 %_15, label %bb9, label %bb16, !dbg !9655

bb9:                                              ; preds = %bb8
  %_24 = sub nsw i64 %frames, %spec.store.select, !dbg !9656
  %_34 = icmp ult i64 %_24, 129, !dbg !9657
  br i1 %_34, label %bb1.i.preheader, label %bb12, !dbg !9657

bb1.i.preheader:                                  ; preds = %bb9
  %140 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1440
  %_5.i174 = load i32, ptr %140, align 4, !dbg !9660, !alias.scope !9662, !noundef !11
  %141 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1444, !dbg !9660
  %_5.i174.1 = load i32, ptr %141, align 4, !dbg !9660, !alias.scope !9662, !noundef !11
  %least.sroa.0.1.i.1 = tail call i32 @llvm.umin.i32(i32 %_5.i174.1, i32 %_5.i174), !dbg !9660
  %142 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1448, !dbg !9660
  %_5.i174.2 = load i32, ptr %142, align 4, !dbg !9660, !alias.scope !9662, !noundef !11
  %least.sroa.0.1.i.2 = tail call i32 @llvm.umin.i32(i32 %_5.i174.2, i32 %least.sroa.0.1.i.1), !dbg !9660
  %143 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1452, !dbg !9660
  %_5.i174.3 = load i32, ptr %143, align 4, !dbg !9660, !alias.scope !9662, !noundef !11
  %least.sroa.0.1.i.3 = tail call i32 @llvm.umin.i32(i32 %_5.i174.3, i32 %least.sroa.0.1.i.2), !dbg !9660
  %144 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1456, !dbg !9660
  %_5.i174.4 = load i32, ptr %144, align 4, !dbg !9660, !alias.scope !9662, !noundef !11
  %least.sroa.0.1.i.4 = tail call i32 @llvm.umin.i32(i32 %_5.i174.4, i32 %least.sroa.0.1.i.3), !dbg !9660
  %145 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1460, !dbg !9660
  %_5.i174.5 = load i32, ptr %145, align 4, !dbg !9660, !alias.scope !9662, !noundef !11
  %least.sroa.0.1.i.5 = tail call i32 @llvm.umin.i32(i32 %_5.i174.5, i32 %least.sroa.0.1.i.4), !dbg !9660
  %146 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1464, !dbg !9660
  %_5.i174.6 = load i32, ptr %146, align 4, !dbg !9660, !alias.scope !9662, !noundef !11
  %least.sroa.0.1.i.6 = tail call i32 @llvm.umin.i32(i32 %_5.i174.6, i32 %least.sroa.0.1.i.5), !dbg !9660
  %147 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1468, !dbg !9660
  %_5.i174.7 = load i32, ptr %147, align 4, !dbg !9660, !alias.scope !9662, !noundef !11
  %least.sroa.0.1.i.7 = tail call i32 @llvm.umin.i32(i32 %_5.i174.7, i32 %least.sroa.0.1.i.6), !dbg !9660
  %_0.i = zext i32 %least.sroa.0.1.i.7 to i64, !dbg !9665
  %_22.not = icmp samesign ugt i64 %_24, %_0.i, !dbg !9666
  br i1 %_22.not, label %bb12, label %bb10, !dbg !9659

bb16:                                             ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit, %bb8
  ret void, !dbg !9667

bb12:                                             ; preds = %bb9, %bb1.i.preheader
  %148 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1540, !dbg !9668
  %_10.i = load i32, ptr %148, align 4, !dbg !9668, !alias.scope !9672, !noalias !9675, !noundef !11
  %ring_length.i = zext i32 %_10.i to i64, !dbg !9668
  %149 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !9678
  %150 = bitcast <8 x float> %149 to <8 x i32>, !dbg !9686
  %151 = xor <8 x i32> %150, splat (i32 -1), !dbg !9692
  %152 = bitcast <8 x i32> %151 to <8 x float>, !dbg !9686
  switch i32 %link, label %bb13.i285 [
    i32 1, label %bb18.i.lr.ph
    i32 3, label %bb14.i286.fold.split
  ], !dbg !9694

bb13.i285:                                        ; preds = %bb12
  br label %bb18.i.lr.ph, !dbg !9695

bb14.i286.fold.split:                             ; preds = %bb12
  br label %bb18.i.lr.ph, !dbg !9696

bb18.i.lr.ph:                                     ; preds = %bb13.i285, %bb14.i286.fold.split, %bb12
  %_11.i274.sroa.0.02108 = phi <8 x float> [ %152, %bb12 ], [ %149, %bb13.i285 ], [ %149, %bb14.i286.fold.split ]
  %_13.i273.sroa.0.0 = phi <8 x i32> [ %151, %bb12 ], [ %151, %bb13.i285 ], [ %150, %bb14.i286.fold.split ], !dbg !9697
  %_4.i343 = getelementptr inbounds nuw i8, ptr %channel_left, i64 448, !dbg !9698
  %lanes.i762.sroa.0.0.copyload = load <8 x float>, ptr %_4.i343, align 32, !dbg !9701, !alias.scope !9706, !noalias !9710
  %_7.i344 = getelementptr inbounds nuw i8, ptr %channel_left, i64 480, !dbg !9716
  %lanes.i756.sroa.0.0.copyload = load <8 x float>, ptr %_7.i344, align 32, !dbg !9717, !alias.scope !9722, !noalias !9726
  %_13.i = getelementptr inbounds nuw i8, ptr %channel_left, i64 256, !dbg !9730
  %lanes.i750.sroa.0.0.copyload = load <8 x float>, ptr %_13.i, align 32, !dbg !9731, !alias.scope !9736, !noalias !9740
  %_14.i345 = getelementptr inbounds nuw i8, ptr %channel_left, i64 288, !dbg !9744
  %lanes.i744.sroa.0.0.copyload = load <8 x float>, ptr %_14.i345, align 32, !dbg !9745, !alias.scope !9750, !noalias !9754
  %_17.i346 = getelementptr inbounds nuw i8, ptr %channel_left, i64 320, !dbg !9758
  %lanes.i738.sroa.0.0.copyload = load <8 x float>, ptr %_17.i346, align 32, !dbg !9759, !alias.scope !9764, !noalias !9768
  %_20.i347 = getelementptr inbounds nuw i8, ptr %channel_left, i64 352, !dbg !9772
  %lanes.i732.sroa.0.0.copyload = load <8 x float>, ptr %_20.i347, align 32, !dbg !9773, !alias.scope !9778, !noalias !9782
  %_23.i348 = getelementptr inbounds nuw i8, ptr %channel_left, i64 384, !dbg !9786
  %lanes.i726.sroa.0.0.copyload = load <8 x float>, ptr %_23.i348, align 32, !dbg !9787, !alias.scope !9792, !noalias !9796
  %_26.i349 = getelementptr inbounds nuw i8, ptr %channel_left, i64 416, !dbg !9800
  %lanes.i.sroa.0.0.copyload = load <8 x float>, ptr %_26.i349, align 32, !dbg !9801, !alias.scope !9806, !noalias !9810
  %153 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i762.sroa.0.0.copyload, <8 x float> zeroinitializer, i8 0), !dbg !9814
  %154 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i756.sroa.0.0.copyload, <8 x float> zeroinitializer, i8 0), !dbg !9820
  %155 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i756.sroa.0.0.copyload, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !9826
  %_6.i.i = load i64, ptr %detector, align 8, !range !220
  %156 = getelementptr inbounds nuw i8, ptr %detector, i64 16
  %sidechain_left.1.i.i = load i64, ptr %156, align 8
  %157 = getelementptr inbounds nuw i8, ptr %detector, i64 24
  %sidechain_right.0.i.i = load ptr, ptr %157, align 8, !nonnull !11, !align !3663
  %158 = getelementptr inbounds nuw i8, ptr %detector, i64 32
  %sidechain_right.1.i.i = load i64, ptr %158, align 8
  %159 = getelementptr inbounds nuw i8, ptr %detector, i64 8
  %sidechain_left.0.i.i = load ptr, ptr %159, align 8, !nonnull !11, !align !3663
  %160 = icmp slt <8 x i32> %_13.i273.sroa.0.0, zeroinitializer
  %161 = bitcast <8 x float> %_11.i274.sroa.0.02108 to <8 x i32>
  %162 = icmp slt <8 x i32> %161, zeroinitializer
  %163 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1536
  %164 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1504
  %165 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1512
  %166 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1528
  %167 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1520
  %168 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1440
  %_47.i = getelementptr inbounds nuw i8, ptr %channel_left, i64 1472
  %169 = bitcast <8 x float> %153 to <8 x i32>
  %170 = bitcast <8 x float> %154 to <8 x i32>
  %171 = select i1 %bypass, <8 x i32> %150, <8 x i32> %151
  %172 = or <8 x i32> %171, %170
  %173 = bitcast <8 x float> %155 to <8 x i32>
  %174 = icmp slt <8 x i32> %173, zeroinitializer
  %175 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1444
  %176 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1448
  %177 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1452
  %178 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1456
  %179 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1460
  %180 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1464
  %181 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1468
  %182 = fneg <8 x float> %lanes.i738.sroa.0.0.copyload
  br label %bb18.i, !dbg !9832

bb18.i:                                           ; preds = %bb18.i.lr.ph, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit
  %start1.sroa.0.0.i2394 = phi i64 [ %spec.store.select, %bb18.i.lr.ph ], [ %183, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit ]
  %183 = add nuw nsw i64 %start1.sroa.0.0.i2394, 1, !dbg !9841
  %slot.i = shl nuw nsw i64 %start1.sroa.0.0.i2394, 3, !dbg !9847
  %_57.i = icmp samesign ugt i64 %slot.i, %left.1, !dbg !9849
  br i1 %_57.i, label %bb20.i, label %bb21.i, !dbg !9849, !prof !161

bb21.i:                                           ; preds = %bb18.i
  %_60.i = sub nuw nsw i64 %left.1, %slot.i, !dbg !9855
  %_64.i = getelementptr inbounds nuw float, ptr %left.0, i64 %slot.i, !dbg !9856
  %_8.i941 = icmp samesign ugt i64 %_60.i, 7, !dbg !9861
  br i1 %_8.i941, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit945, label %bb2.i942, !dbg !9861, !prof !2116

bb2.i942:                                         ; preds = %bb21.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_60.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !9866, !noalias !9867
  unreachable, !dbg !9866

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit945: ; preds = %bb21.i
  %lanes.i938.sroa.0.0.copyload = load <8 x float>, ptr %_64.i, align 4, !dbg !9871, !alias.scope !9875, !noalias !9879
  switch i64 %_6.i.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit945.unreachabledefault [
    i64 0, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i
    i64 1, label %bb3.i.i
    i64 2, label %bb2.i.i
  ], !dbg !9881

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit945.unreachabledefault: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit945
  unreachable

default.unreachable:                              ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit349.i
  unreachable

bb3.i.i:                                          ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit945
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i, !dbg !9884

bb2.i.i:                                          ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit945
  %_25.i.i = icmp ugt i64 %slot.i, %sidechain_left.1.i.i, !dbg !9885
  br i1 %_25.i.i, label %bb17.i.i, label %bb18.i.i, !dbg !9885, !prof !161

bb18.i.i:                                         ; preds = %bb2.i.i
  %_28.i.i = sub nuw i64 %sidechain_left.1.i.i, %slot.i, !dbg !9888
  %_8.i932 = icmp samesign ugt i64 %_28.i.i, 7, !dbg !9889
  br i1 %_8.i932, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit936, label %bb2.i933, !dbg !9889, !prof !2116

bb2.i933:                                         ; preds = %bb18.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_28.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !9894, !noalias !9895
  unreachable, !dbg !9894

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit936: ; preds = %bb18.i.i
  %_32.i.i = getelementptr inbounds nuw float, ptr %sidechain_left.0.i.i, i64 %slot.i, !dbg !9904
  %lanes.i929.sroa.0.0.copyload = load <8 x float>, ptr %_32.i.i, align 4, !dbg !9906, !alias.scope !9910, !noalias !9914
  %_33.i.i = icmp ugt i64 %slot.i, %sidechain_right.1.i.i, !dbg !9916
  br i1 %_33.i.i, label %bb19.i.i, label %bb20.i.i, !dbg !9916, !prof !161

bb17.i.i:                                         ; preds = %bb2.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 34359738361) %slot.i, i64 noundef %sidechain_left.1.i.i, i64 noundef %sidechain_left.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a0a4023c361dd50869bbb08b76482f50) #23, !dbg !9919, !noalias !9920
  unreachable, !dbg !9919

bb20.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit936
  %_36.i.i = sub nuw i64 %sidechain_right.1.i.i, %slot.i, !dbg !9922
  %_8.i923 = icmp samesign ugt i64 %_36.i.i, 7, !dbg !9923
  br i1 %_8.i923, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit927, label %bb2.i924, !dbg !9923, !prof !2116

bb2.i924:                                         ; preds = %bb20.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_36.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !9928, !noalias !9929
  unreachable, !dbg !9928

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit927: ; preds = %bb20.i.i
  %_40.i.i = getelementptr inbounds nuw float, ptr %sidechain_right.0.i.i, i64 %slot.i, !dbg !9933
  %lanes.i920.sroa.0.0.copyload = load <8 x float>, ptr %_40.i.i, align 4, !dbg !9935, !alias.scope !9939, !noalias !9943
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i, !dbg !9945

bb19.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit936
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 34359738361) %slot.i, i64 noundef %sidechain_right.1.i.i, i64 noundef %sidechain_right.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5807a2a07e9839e3d32631b4243eb17f) #23, !dbg !9946, !noalias !9920
  unreachable, !dbg !9946

_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit945, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit927, %bb3.i.i
  %.sroa.01414.0 = phi <8 x float> [ %lanes.i938.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit945 ], [ zeroinitializer, %bb3.i.i ], [ %lanes.i920.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit927 ], !dbg !9947
  %.sroa.01410.0 = phi <8 x float> [ %lanes.i938.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit945 ], [ zeroinitializer, %bb3.i.i ], [ %lanes.i929.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit927 ], !dbg !9947
  %184 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %.sroa.01410.0), !dbg !9948
  %185 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %.sroa.01414.0), !dbg !9954
  %186 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %184, <8 x float> %185), !dbg !9960
  %187 = fmul <8 x float> %184, splat (float 5.000000e-01), !dbg !9965
  %188 = fmul <8 x float> %185, splat (float 5.000000e-01), !dbg !9970
  %189 = fadd <8 x float> %187, %188, !dbg !9975
  %190 = select <8 x i1> %160, <8 x float> %189, <8 x float> %186, !dbg !9980
  %191 = select <8 x i1> %162, <8 x float> %190, <8 x float> %184, !dbg !9985
  %_28.i = load i32, ptr %163, align 32, !dbg !9990, !alias.scope !9672, !noalias !9675, !noundef !11
  %write.i = zext i32 %_28.i to i64, !dbg !9990
  %192 = add nuw nsw i64 %write.i, 1, !dbg !9992
  %_30.i = icmp eq i64 %192, %ring_length.i, !dbg !9994
  %spec.store.select.i = select i1 %_30.i, i64 0, i64 %192, !dbg !9994
  %_95.1.i = load i64, ptr %165, align 8, !dbg !9996, !alias.scope !9672, !noalias !9675, !noundef !11
  %_34.i = shl nuw nsw i64 %write.i, 3, !dbg !9998
  %_65.i = icmp ugt i64 %_34.i, %_95.1.i, !dbg !9999
  br i1 %_65.i, label %bb22.i, label %bb23.i, !dbg !9999, !prof !161

bb20.i:                                           ; preds = %bb18.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %slot.i, i64 noundef range(i64 8, 34359738361) %left.1, i64 noundef range(i64 8, 34359738361) %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_3b7137b1f9053f34ed465a67bd56da03) #23, !dbg !10004, !noalias !10005
  unreachable, !dbg !10004

bb23.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i
  %_68.i = sub nuw i64 %_95.1.i, %_34.i, !dbg !10006
  %_8.i1248 = icmp samesign ugt i64 %_68.i, 7, !dbg !10007
  br i1 %_8.i1248, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1251, label %bb2.i1249, !dbg !10007, !prof !2116

bb2.i1249:                                        ; preds = %bb23.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_68.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !10012, !noalias !10013
  unreachable, !dbg !10012

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1251: ; preds = %bb23.i
  %_95.0.i = load ptr, ptr %164, align 32, !dbg !9996, !alias.scope !9672, !noalias !9675, !nonnull !11, !noundef !11
  %_72.i = getelementptr inbounds nuw float, ptr %_95.0.i, i64 %_34.i, !dbg !10017
  store <8 x float> %lanes.i938.sroa.0.0.copyload, ptr %_72.i, align 4, !dbg !10022, !alias.scope !10026, !noalias !10030
  %_96.1.i = load i64, ptr %166, align 8, !dbg !10032, !alias.scope !9672, !noalias !9675, !noundef !11
  %_73.i = icmp ugt i64 %_34.i, %_96.1.i, !dbg !10033
  br i1 %_73.i, label %bb24.i, label %bb25.i, !dbg !10033, !prof !161

bb22.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_34.i, i64 noundef %_95.1.i, i64 noundef %_95.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1992fd841b4d79a3d694b8d46102fa04) #23, !dbg !10037, !noalias !10005
  unreachable, !dbg !10037

bb25.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1251
  %_76.i = sub nuw i64 %_96.1.i, %_34.i, !dbg !10038
  %_8.i1243 = icmp samesign ugt i64 %_76.i, 7, !dbg !10039
  br i1 %_8.i1243, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1246, label %bb2.i1244, !dbg !10039, !prof !2116

bb2.i1244:                                        ; preds = %bb25.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_76.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !10044, !noalias !10045
  unreachable, !dbg !10044

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1246: ; preds = %bb25.i
  %_96.0.i = load ptr, ptr %167, align 16, !dbg !10032, !alias.scope !9672, !noalias !9675, !nonnull !11, !noundef !11
  %_80.i = getelementptr inbounds nuw float, ptr %_96.0.i, i64 %_34.i, !dbg !10049
  store <8 x float> %191, ptr %_80.i, align 4, !dbg !10054, !alias.scope !10058, !noalias !10062
  %_97.1.i = load i64, ptr %165, align 8, !dbg !10064, !alias.scope !9672, !noalias !9675, !noundef !11
  %_40.i = shl nuw nsw i64 %spec.store.select.i, 3, !dbg !10065
  %_81.i = icmp ugt i64 %_40.i, %_97.1.i, !dbg !10066
  br i1 %_81.i, label %bb26.i, label %bb27.i, !dbg !10066, !prof !161

bb24.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1251
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_34.i, i64 noundef %_96.1.i, i64 noundef %_96.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4989c99a90ecaa46922498d8532f27e6) #23, !dbg !10070, !noalias !10005
  unreachable, !dbg !10070

bb27.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1246
  %_84.i = sub nuw i64 %_97.1.i, %_40.i, !dbg !10071
  %_8.i914 = icmp samesign ugt i64 %_84.i, 7, !dbg !10072
  br i1 %_8.i914, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit918, label %bb2.i915, !dbg !10072, !prof !2116

bb2.i915:                                         ; preds = %bb27.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_84.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !10077, !noalias !10078
  unreachable, !dbg !10077

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit918: ; preds = %bb27.i
  %_97.0.i = load ptr, ptr %164, align 32, !dbg !10064, !alias.scope !9672, !noalias !9675, !nonnull !11, !noundef !11
  %_88.i = getelementptr inbounds nuw float, ptr %_97.0.i, i64 %_40.i, !dbg !10082
  %lanes.i911.sroa.0.0.copyload = load <8 x float>, ptr %_88.i, align 4, !dbg !10087, !alias.scope !10091, !noalias !10095
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10097), !dbg !10100
  %_5.i7.i = load i32, ptr %148, align 4, !dbg !10102, !alias.scope !10104, !noalias !10105, !noundef !11
  %_38.1.i.i = load i64, ptr %166, align 8
  %_38.0.i.i = load ptr, ptr %167, align 16, !nonnull !11
  %_17.i10.i = load i32, ptr %168, align 4, !dbg !10108, !alias.scope !10104, !noalias !10105, !noundef !11
  %delay.i.i = zext i32 %_17.i10.i to i64, !dbg !10108
  %_20.not.i.i = icmp ult i32 %_28.i, %_17.i10.i, !dbg !10109
  %narrow2128 = select i1 %_20.not.i.i, i32 %_5.i7.i, i32 0, !dbg !10109
  %_21.i11.i = zext i32 %narrow2128 to i64, !dbg !10109
  %write.pn.i.i = sub nsw i64 %write.i, %delay.i.i, !dbg !10109
  %tap.sroa.0.0.i.i = add nsw i64 %write.pn.i.i, %_21.i11.i, !dbg !10110
  %_24.i12.i = shl nsw i64 %tap.sroa.0.0.i.i, 3, !dbg !10111
  %_26.i.i = icmp ult i64 %_24.i12.i, %_38.1.i.i, !dbg !10112
  br i1 %_26.i.i, label %bb10.i.i, label %panic1.i.i, !dbg !10112

bb10.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit918
  %193 = getelementptr inbounds nuw float, ptr %_38.0.i.i, i64 %_24.i12.i, !dbg !10112
  %_22.i14.i = load float, ptr %193, align 4, !dbg !10112, !noalias !10113, !noundef !11
  %_17.i10.i.1 = load i32, ptr %175, align 4, !dbg !10108, !alias.scope !10104, !noalias !10105, !noundef !11
  %delay.i.i.1 = zext i32 %_17.i10.i.1 to i64, !dbg !10108
  %_20.not.i.i.1 = icmp ult i32 %_28.i, %_17.i10.i.1, !dbg !10109
  %narrow2128.1 = select i1 %_20.not.i.i.1, i32 %_5.i7.i, i32 0, !dbg !10109
  %_21.i11.i.1 = zext i32 %narrow2128.1 to i64, !dbg !10109
  %write.pn.i.i.1 = sub nsw i64 %write.i, %delay.i.i.1, !dbg !10109
  %tap.sroa.0.0.i.i.1 = add nsw i64 %write.pn.i.i.1, %_21.i11.i.1, !dbg !10110
  %_24.i12.i.1 = shl nsw i64 %tap.sroa.0.0.i.i.1, 3, !dbg !10111
  %_23.i13.i.1 = or disjoint i64 %_24.i12.i.1, 1, !dbg !10111
  %_26.i.i.1 = icmp ult i64 %_23.i13.i.1, %_38.1.i.i, !dbg !10112
  br i1 %_26.i.i.1, label %bb10.i.i.1, label %panic1.i.i, !dbg !10112

bb10.i.i.1:                                       ; preds = %bb10.i.i
  %194 = getelementptr inbounds nuw float, ptr %_38.0.i.i, i64 %_23.i13.i.1, !dbg !10112
  %_22.i14.i.1 = load float, ptr %194, align 4, !dbg !10112, !noalias !10113, !noundef !11
  %_17.i10.i.2 = load i32, ptr %176, align 4, !dbg !10108, !alias.scope !10104, !noalias !10105, !noundef !11
  %delay.i.i.2 = zext i32 %_17.i10.i.2 to i64, !dbg !10108
  %_20.not.i.i.2 = icmp ult i32 %_28.i, %_17.i10.i.2, !dbg !10109
  %narrow2128.2 = select i1 %_20.not.i.i.2, i32 %_5.i7.i, i32 0, !dbg !10109
  %_21.i11.i.2 = zext i32 %narrow2128.2 to i64, !dbg !10109
  %write.pn.i.i.2 = sub nsw i64 %write.i, %delay.i.i.2, !dbg !10109
  %tap.sroa.0.0.i.i.2 = add nsw i64 %write.pn.i.i.2, %_21.i11.i.2, !dbg !10110
  %_24.i12.i.2 = shl nsw i64 %tap.sroa.0.0.i.i.2, 3, !dbg !10111
  %_23.i13.i.2 = or disjoint i64 %_24.i12.i.2, 2, !dbg !10111
  %_26.i.i.2 = icmp ult i64 %_23.i13.i.2, %_38.1.i.i, !dbg !10112
  br i1 %_26.i.i.2, label %bb10.i.i.2, label %panic1.i.i, !dbg !10112

bb10.i.i.2:                                       ; preds = %bb10.i.i.1
  %195 = getelementptr inbounds nuw float, ptr %_38.0.i.i, i64 %_23.i13.i.2, !dbg !10112
  %_22.i14.i.2 = load float, ptr %195, align 4, !dbg !10112, !noalias !10113, !noundef !11
  %_17.i10.i.3 = load i32, ptr %177, align 4, !dbg !10108, !alias.scope !10104, !noalias !10105, !noundef !11
  %delay.i.i.3 = zext i32 %_17.i10.i.3 to i64, !dbg !10108
  %_20.not.i.i.3 = icmp ult i32 %_28.i, %_17.i10.i.3, !dbg !10109
  %narrow2128.3 = select i1 %_20.not.i.i.3, i32 %_5.i7.i, i32 0, !dbg !10109
  %_21.i11.i.3 = zext i32 %narrow2128.3 to i64, !dbg !10109
  %write.pn.i.i.3 = sub nsw i64 %write.i, %delay.i.i.3, !dbg !10109
  %tap.sroa.0.0.i.i.3 = add nsw i64 %write.pn.i.i.3, %_21.i11.i.3, !dbg !10110
  %_24.i12.i.3 = shl nsw i64 %tap.sroa.0.0.i.i.3, 3, !dbg !10111
  %_23.i13.i.3 = or disjoint i64 %_24.i12.i.3, 3, !dbg !10111
  %_26.i.i.3 = icmp ult i64 %_23.i13.i.3, %_38.1.i.i, !dbg !10112
  br i1 %_26.i.i.3, label %bb10.i.i.3, label %panic1.i.i, !dbg !10112

bb10.i.i.3:                                       ; preds = %bb10.i.i.2
  %196 = getelementptr inbounds nuw float, ptr %_38.0.i.i, i64 %_23.i13.i.3, !dbg !10112
  %_22.i14.i.3 = load float, ptr %196, align 4, !dbg !10112, !noalias !10113, !noundef !11
  %_17.i10.i.4 = load i32, ptr %178, align 4, !dbg !10108, !alias.scope !10104, !noalias !10105, !noundef !11
  %delay.i.i.4 = zext i32 %_17.i10.i.4 to i64, !dbg !10108
  %_20.not.i.i.4 = icmp ult i32 %_28.i, %_17.i10.i.4, !dbg !10109
  %narrow2128.4 = select i1 %_20.not.i.i.4, i32 %_5.i7.i, i32 0, !dbg !10109
  %_21.i11.i.4 = zext i32 %narrow2128.4 to i64, !dbg !10109
  %write.pn.i.i.4 = sub nsw i64 %write.i, %delay.i.i.4, !dbg !10109
  %tap.sroa.0.0.i.i.4 = add nsw i64 %write.pn.i.i.4, %_21.i11.i.4, !dbg !10110
  %_24.i12.i.4 = shl nsw i64 %tap.sroa.0.0.i.i.4, 3, !dbg !10111
  %_23.i13.i.4 = or disjoint i64 %_24.i12.i.4, 4, !dbg !10111
  %_26.i.i.4 = icmp ult i64 %_23.i13.i.4, %_38.1.i.i, !dbg !10112
  br i1 %_26.i.i.4, label %bb10.i.i.4, label %panic1.i.i, !dbg !10112

bb10.i.i.4:                                       ; preds = %bb10.i.i.3
  %197 = getelementptr inbounds nuw float, ptr %_38.0.i.i, i64 %_23.i13.i.4, !dbg !10112
  %_22.i14.i.4 = load float, ptr %197, align 4, !dbg !10112, !noalias !10113, !noundef !11
  %_17.i10.i.5 = load i32, ptr %179, align 4, !dbg !10108, !alias.scope !10104, !noalias !10105, !noundef !11
  %delay.i.i.5 = zext i32 %_17.i10.i.5 to i64, !dbg !10108
  %_20.not.i.i.5 = icmp ult i32 %_28.i, %_17.i10.i.5, !dbg !10109
  %narrow2128.5 = select i1 %_20.not.i.i.5, i32 %_5.i7.i, i32 0, !dbg !10109
  %_21.i11.i.5 = zext i32 %narrow2128.5 to i64, !dbg !10109
  %write.pn.i.i.5 = sub nsw i64 %write.i, %delay.i.i.5, !dbg !10109
  %tap.sroa.0.0.i.i.5 = add nsw i64 %write.pn.i.i.5, %_21.i11.i.5, !dbg !10110
  %_24.i12.i.5 = shl nsw i64 %tap.sroa.0.0.i.i.5, 3, !dbg !10111
  %_23.i13.i.5 = or disjoint i64 %_24.i12.i.5, 5, !dbg !10111
  %_26.i.i.5 = icmp ult i64 %_23.i13.i.5, %_38.1.i.i, !dbg !10112
  br i1 %_26.i.i.5, label %bb10.i.i.5, label %panic1.i.i, !dbg !10112

bb10.i.i.5:                                       ; preds = %bb10.i.i.4
  %198 = getelementptr inbounds nuw float, ptr %_38.0.i.i, i64 %_23.i13.i.5, !dbg !10112
  %_22.i14.i.5 = load float, ptr %198, align 4, !dbg !10112, !noalias !10113, !noundef !11
  %_17.i10.i.6 = load i32, ptr %180, align 4, !dbg !10108, !alias.scope !10104, !noalias !10105, !noundef !11
  %delay.i.i.6 = zext i32 %_17.i10.i.6 to i64, !dbg !10108
  %_20.not.i.i.6 = icmp ult i32 %_28.i, %_17.i10.i.6, !dbg !10109
  %narrow2128.6 = select i1 %_20.not.i.i.6, i32 %_5.i7.i, i32 0, !dbg !10109
  %_21.i11.i.6 = zext i32 %narrow2128.6 to i64, !dbg !10109
  %write.pn.i.i.6 = sub nsw i64 %write.i, %delay.i.i.6, !dbg !10109
  %tap.sroa.0.0.i.i.6 = add nsw i64 %write.pn.i.i.6, %_21.i11.i.6, !dbg !10110
  %_24.i12.i.6 = shl nsw i64 %tap.sroa.0.0.i.i.6, 3, !dbg !10111
  %_23.i13.i.6 = or disjoint i64 %_24.i12.i.6, 6, !dbg !10111
  %_26.i.i.6 = icmp ult i64 %_23.i13.i.6, %_38.1.i.i, !dbg !10112
  br i1 %_26.i.i.6, label %bb6.i.i.7, label %panic1.i.i, !dbg !10112

bb6.i.i.7:                                        ; preds = %bb10.i.i.5
  %_17.i10.i.7 = load i32, ptr %181, align 4, !dbg !10108, !alias.scope !10104, !noalias !10105, !noundef !11
  %delay.i.i.7 = zext i32 %_17.i10.i.7 to i64, !dbg !10108
  %_20.not.i.i.7 = icmp ult i32 %_28.i, %_17.i10.i.7, !dbg !10109
  %narrow2128.7 = select i1 %_20.not.i.i.7, i32 %_5.i7.i, i32 0, !dbg !10109
  %_21.i11.i.7 = zext i32 %narrow2128.7 to i64, !dbg !10109
  %write.pn.i.i.7 = sub nsw i64 %write.i, %delay.i.i.7, !dbg !10109
  %tap.sroa.0.0.i.i.7 = add nsw i64 %write.pn.i.i.7, %_21.i11.i.7, !dbg !10110
  %_24.i12.i.7 = shl nsw i64 %tap.sroa.0.0.i.i.7, 3, !dbg !10111
  %_23.i13.i.7 = or disjoint i64 %_24.i12.i.7, 7, !dbg !10111
  %_26.i.i.7 = icmp ult i64 %_23.i13.i.7, %_38.1.i.i, !dbg !10112
  br i1 %_26.i.i.7, label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit, label %panic1.i.i, !dbg !10112

panic1.i.i:                                       ; preds = %bb6.i.i.7, %bb10.i.i.5, %bb10.i.i.4, %bb10.i.i.3, %bb10.i.i.2, %bb10.i.i.1, %bb10.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit918
  %_23.i13.i.lcssa = phi i64 [ %_24.i12.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit918 ], [ %_23.i13.i.1, %bb10.i.i ], [ %_23.i13.i.2, %bb10.i.i.1 ], [ %_23.i13.i.3, %bb10.i.i.2 ], [ %_23.i13.i.4, %bb10.i.i.3 ], [ %_23.i13.i.5, %bb10.i.i.4 ], [ %_23.i13.i.6, %bb10.i.i.5 ], [ %_23.i13.i.7, %bb6.i.i.7 ], !dbg !10111
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_23.i13.i.lcssa, i64 noundef %_38.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a529a882edb544ae33c686afc6011081) #23, !dbg !10112, !noalias !10113
  unreachable, !dbg !10112

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit: ; preds = %bb6.i.i.7
  %199 = getelementptr inbounds nuw float, ptr %_38.0.i.i, i64 %_23.i13.i.6, !dbg !10112
  %_22.i14.i.6 = load float, ptr %199, align 4, !dbg !10112, !noalias !10113, !noundef !11
  %lanes.i904.sroa.0.0.vec.insert = insertelement <8 x float> poison, float %_22.i14.i, i64 0, !dbg !10114
  %lanes.i904.sroa.0.4.vec.insert = insertelement <8 x float> %lanes.i904.sroa.0.0.vec.insert, float %_22.i14.i.1, i64 1, !dbg !10114
  %lanes.i904.sroa.0.8.vec.insert = insertelement <8 x float> %lanes.i904.sroa.0.4.vec.insert, float %_22.i14.i.2, i64 2, !dbg !10114
  %lanes.i904.sroa.0.12.vec.insert = insertelement <8 x float> %lanes.i904.sroa.0.8.vec.insert, float %_22.i14.i.3, i64 3, !dbg !10114
  %lanes.i904.sroa.0.16.vec.insert = insertelement <8 x float> %lanes.i904.sroa.0.12.vec.insert, float %_22.i14.i.4, i64 4, !dbg !10114
  %lanes.i904.sroa.0.20.vec.insert = insertelement <8 x float> %lanes.i904.sroa.0.16.vec.insert, float %_22.i14.i.5, i64 5, !dbg !10114
  %lanes.i904.sroa.0.24.vec.insert = insertelement <8 x float> %lanes.i904.sroa.0.20.vec.insert, float %_22.i14.i.6, i64 6, !dbg !10114
  %200 = getelementptr inbounds nuw float, ptr %_38.0.i.i, i64 %_23.i13.i.7, !dbg !10112
  %_22.i14.i.7 = load float, ptr %200, align 4, !dbg !10112, !noalias !10113, !noundef !11
  %lanes.i904.sroa.0.28.vec.insert = insertelement <8 x float> %lanes.i904.sroa.0.24.vec.insert, float %_22.i14.i.7, i64 7, !dbg !10114
  %201 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i904.sroa.0.28.vec.insert, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !10119
  %202 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %201, <8 x float> splat (float 0x3810000000000000)), !dbg !10128
  %203 = bitcast <8 x float> %202 to <4 x i64>, !dbg !10135
  %204 = and <4 x i64> %203, splat (i64 36028792732385279), !dbg !10136
  %205 = or disjoint <4 x i64> %204, splat (i64 4575657222473777152), !dbg !10141
  %206 = bitcast <4 x i64> %205 to <8 x float>, !dbg !10145
  %207 = fadd <8 x float> %206, splat (float -1.000000e+00), !dbg !10146
  %208 = fmul <8 x float> %207, splat (float 0xBF9B17A960000000), !dbg !10151
  %209 = fadd <8 x float> %208, splat (float 0x3FBF9A8440000000), !dbg !10156
  %210 = fmul <8 x float> %207, %209, !dbg !10151
  %211 = fadd <8 x float> %210, splat (float 0xBFD1E3F400000000), !dbg !10156
  %212 = fmul <8 x float> %207, %211, !dbg !10151
  %213 = fadd <8 x float> %212, splat (float 0x3FDD544F20000000), !dbg !10156
  %214 = fmul <8 x float> %207, %213, !dbg !10151
  %215 = fadd <8 x float> %214, splat (float 0xBFE6FC2A60000000), !dbg !10156
  %216 = fmul <8 x float> %207, %215, !dbg !10151
  %217 = fadd <8 x float> %216, splat (float 0x3FF714B2A0000000), !dbg !10156
  %218 = bitcast <8 x float> %202 to <8 x i32>, !dbg !10161
  %_3.i1369 = lshr <8 x i32> %218, splat (i32 23), !dbg !10165
  %219 = or disjoint <8 x i32> %_3.i1369, splat (i32 1258291200), !dbg !10166
  %220 = bitcast <8 x i32> %219 to <8 x float>, !dbg !10170
  %221 = fadd <8 x float> %220, splat (float 0xC160000FE0000000), !dbg !10171
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10175), !dbg !10178
  %_6.i170.sroa.0.0.copyload = load <8 x float>, ptr %_47.i, align 32, !dbg !10180, !noalias !10182
  %222 = fmul <8 x float> %207, %217, !dbg !10185
  %223 = fadd <8 x float> %221, %222, !dbg !10190
  %224 = fmul <8 x float> %223, splat (float 0x4018151820000000), !dbg !10195
  %225 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %224, <8 x float> splat (float -1.600000e+02)), !dbg !10200
  %226 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %225, <8 x float> splat (float 2.400000e+01)), !dbg !10205
  %227 = fsub <8 x float> %226, %lanes.i750.sroa.0.0.copyload, !dbg !10210
  %228 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %227, <8 x float> %lanes.i738.sroa.0.0.copyload, i8 30), !dbg !10216
  %229 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %227, <8 x float> %182, i8 18), !dbg !10222
  %230 = fadd <8 x float> %lanes.i738.sroa.0.0.copyload, %227, !dbg !10228
  %231 = fmul <8 x float> %230, %230, !dbg !10233
  %232 = fmul <8 x float> %lanes.i732.sroa.0.0.copyload, %231, !dbg !10238
  %233 = bitcast <8 x float> %228 to <8 x i32>, !dbg !10243
  %234 = icmp slt <8 x i32> %233, zeroinitializer, !dbg !10247
  %.v2133 = select <8 x i1> %234, <8 x float> %227, <8 x float> %232, !dbg !10247
  %235 = fmul <8 x float> %lanes.i744.sroa.0.0.copyload, %.v2133, !dbg !10247
  %236 = bitcast <8 x float> %229 to <8 x i32>, !dbg !10249
  %237 = icmp slt <8 x i32> %236, zeroinitializer, !dbg !10253
  %238 = select <8 x i1> %237, <8 x float> zeroinitializer, <8 x float> %235, !dbg !10253
  %239 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %238, <8 x float> splat (float -1.000000e+02)), !dbg !10255
  %240 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %239, <8 x float> zeroinitializer), !dbg !10260
  %241 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %240, <8 x float> %_6.i170.sroa.0.0.copyload, i8 17), !dbg !10265
  %242 = bitcast <8 x float> %241 to <8 x i32>, !dbg !10271
  %243 = icmp slt <8 x i32> %242, zeroinitializer, !dbg !10275
  %244 = select <8 x i1> %243, <8 x float> %lanes.i726.sroa.0.0.copyload, <8 x float> %lanes.i.sroa.0.0.copyload, !dbg !10275
  %245 = fsub <8 x float> %240, %_6.i170.sroa.0.0.copyload, !dbg !10277
  %246 = fmul <8 x float> %245, %244, !dbg !10283
  %247 = fadd <8 x float> %_6.i170.sroa.0.0.copyload, %246, !dbg !10288
  %248 = bitcast <8 x float> %247 to <8 x i32>, !dbg !10292
  %249 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %247), !dbg !10298
  %250 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %249, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !10300
  %251 = bitcast <8 x float> %250 to <8 x i32>, !dbg !10306
  %252 = xor <8 x i32> %251, splat (i32 -1), !dbg !10312
  %253 = and <8 x i32> %252, %248, !dbg !10314
  store <8 x i32> %253, ptr %_47.i, align 32, !dbg !10318, !alias.scope !10319, !noalias !10321
  %254 = bitcast <8 x i32> %253 to <8 x float>, !dbg !10322
  %255 = fadd <8 x float> %lanes.i762.sroa.0.0.copyload, %254, !dbg !10323
  %256 = fmul <8 x float> %255, splat (float 0x3FC542A5A0000000), !dbg !10330
  %257 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %256, <8 x float> splat (float -1.260000e+02)), !dbg !10336
  %258 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %257, <8 x float> splat (float 1.270000e+02)), !dbg !10342
  %259 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %258), !dbg !10347
  %260 = fsub <8 x float> %258, %259, !dbg !10352
  %261 = fmul <8 x float> %260, splat (float 0x3F5E974FA0000000), !dbg !10357
  %262 = fadd <8 x float> %261, splat (float 0x3F82778560000000), !dbg !10362
  %263 = fmul <8 x float> %260, %262, !dbg !10357
  %264 = fadd <8 x float> %263, splat (float 0x3FAC91CE60000000), !dbg !10362
  %265 = fmul <8 x float> %260, %264, !dbg !10357
  %266 = fadd <8 x float> %265, splat (float 0x3FCEBDB560000000), !dbg !10362
  %267 = fmul <8 x float> %260, %266, !dbg !10357
  %268 = fadd <8 x float> %267, splat (float 0x3FE62E4BA0000000), !dbg !10362
  %269 = fmul <8 x float> %260, %268, !dbg !10367
  %270 = fadd <8 x float> %269, splat (float 1.000000e+00), !dbg !10372
  %271 = fadd <8 x float> %259, splat (float 0x4160000FE0000000), !dbg !10377
  %272 = bitcast <8 x float> %271 to <8 x i32>, !dbg !10382
  %_3.i1370 = shl <8 x i32> %272, splat (i32 23), !dbg !10386
  %273 = bitcast <8 x i32> %_3.i1370 to <8 x float>, !dbg !10387
  %274 = fmul <8 x float> %270, %273, !dbg !10389
  %275 = fmul <8 x float> %lanes.i911.sroa.0.0.copyload, %274, !dbg !10393
  %276 = fsub <8 x float> %275, %lanes.i911.sroa.0.0.copyload, !dbg !10398
  %277 = fmul <8 x float> %lanes.i756.sroa.0.0.copyload, %276, !dbg !10404
  %278 = fadd <8 x float> %lanes.i911.sroa.0.0.copyload, %277, !dbg !10409
  %279 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %254, <8 x float> zeroinitializer, i8 0), !dbg !10413
  %280 = bitcast <8 x float> %279 to <8 x i32>, !dbg !10419
  %281 = and <8 x i32> %280, %169, !dbg !10423
  %282 = or <8 x i32> %172, %281, !dbg !10425
  %283 = select <8 x i1> %174, <8 x float> %275, <8 x float> %278, !dbg !10430
  %284 = icmp slt <8 x i32> %282, zeroinitializer, !dbg !10435
  %285 = select <8 x i1> %284, <8 x float> %lanes.i911.sroa.0.0.copyload, <8 x float> %283, !dbg !10435
  store <8 x float> %285, ptr %_64.i, align 4, !dbg !10440, !alias.scope !10446, !noalias !10450
  %286 = trunc i64 %spec.store.select.i to i32, !dbg !10454
  store i32 %286, ptr %163, align 32, !dbg !10454, !alias.scope !9672, !noalias !9675
  %exitcond2694.not = icmp eq i64 %183, %frames, !dbg !10455
  br i1 %exitcond2694.not, label %bb16, label %bb18.i, !dbg !9832

bb26.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1246
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_40.i, i64 noundef %_97.1.i, i64 noundef %_97.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b508431e8547d856227f5f364b1d083b) #23, !dbg !10458, !noalias !10005
  unreachable, !dbg !10458

bb10:                                             ; preds = %bb1.i.preheader
  %_36.0 = load ptr, ptr %staged, align 8, !dbg !10459, !nonnull !11, !noundef !11
  %287 = getelementptr inbounds nuw i8, ptr %staged, i64 8, !dbg !10459
  %_36.1 = load i64, ptr %287, align 8, !dbg !10459, !noundef !11
  %288 = getelementptr inbounds nuw i8, ptr %staged, i64 32, !dbg !10461
  %_37.0 = load ptr, ptr %288, align 8, !dbg !10461, !nonnull !11, !noundef !11
  %289 = getelementptr inbounds nuw i8, ptr %staged, i64 40, !dbg !10461
  %_37.1 = load i64, ptr %289, align 8, !dbg !10461, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10462), !dbg !10465
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10466), !dbg !10465
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10468), !dbg !10465
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10470), !dbg !10465
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10472), !dbg !10465
  %290 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1540, !dbg !10474
  %_11.i1371 = load i32, ptr %290, align 4, !dbg !10474, !alias.scope !10468, !noalias !10478, !noundef !11
  %ring_length.i1372 = zext i32 %_11.i1371 to i64, !dbg !10474
  %291 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !10479
  %292 = bitcast <8 x float> %291 to <8 x i32>, !dbg !10488
  %293 = xor <8 x i32> %292, splat (i32 -1), !dbg !10494
  %294 = bitcast <8 x i32> %293 to <8 x float>, !dbg !10488
  switch i32 %link, label %bb13.i79.i [
    i32 1, label %bb14.i80.i
    i32 3, label %bb14.i80.fold.split.i
  ], !dbg !10496

bb13.i79.i:                                       ; preds = %bb10
  br label %bb14.i80.i, !dbg !10497

bb14.i80.fold.split.i:                            ; preds = %bb10
  br label %bb14.i80.i, !dbg !10498

bb14.i80.i:                                       ; preds = %bb14.i80.fold.split.i, %bb13.i79.i, %bb10
  %_11.i72.sroa.0.0889.i = phi <8 x float> [ %294, %bb10 ], [ %291, %bb13.i79.i ], [ %291, %bb14.i80.fold.split.i ]
  %_13.i71.sroa.0.0.i = phi <8 x i32> [ %293, %bb10 ], [ %293, %bb13.i79.i ], [ %292, %bb14.i80.fold.split.i ], !dbg !10499
  %_4.i92.i = getelementptr inbounds nuw i8, ptr %channel_left, i64 448, !dbg !10500
  %lanes.i307.sroa.0.0.copyload.i = load <8 x float>, ptr %_4.i92.i, align 32, !dbg !10503, !alias.scope !10508, !noalias !10512
  %_7.i93.i = getelementptr inbounds nuw i8, ptr %channel_left, i64 480, !dbg !10518
  %lanes.i302.sroa.0.0.copyload.i = load <8 x float>, ptr %_7.i93.i, align 32, !dbg !10519, !alias.scope !10524, !noalias !10528
  %_15.i1373 = getelementptr inbounds nuw i8, ptr %channel_left, i64 256, !dbg !10532
  %lanes.i297.sroa.0.0.copyload.i = load <8 x float>, ptr %_15.i1373, align 32, !dbg !10533, !alias.scope !10538, !noalias !10542
  %_14.i94.i = getelementptr inbounds nuw i8, ptr %channel_left, i64 288, !dbg !10546
  %lanes.i292.sroa.0.0.copyload.i = load <8 x float>, ptr %_14.i94.i, align 32, !dbg !10547, !alias.scope !10552, !noalias !10556
  %_17.i95.i = getelementptr inbounds nuw i8, ptr %channel_left, i64 320, !dbg !10560
  %lanes.i287.sroa.0.0.copyload.i = load <8 x float>, ptr %_17.i95.i, align 32, !dbg !10561, !alias.scope !10566, !noalias !10570
  %_20.i96.i = getelementptr inbounds nuw i8, ptr %channel_left, i64 352, !dbg !10574
  %lanes.i282.sroa.0.0.copyload.i = load <8 x float>, ptr %_20.i96.i, align 32, !dbg !10575, !alias.scope !10580, !noalias !10584
  %_23.i97.i = getelementptr inbounds nuw i8, ptr %channel_left, i64 384, !dbg !10588
  %lanes.i277.sroa.0.0.copyload.i = load <8 x float>, ptr %_23.i97.i, align 32, !dbg !10589, !alias.scope !10594, !noalias !10598
  %_26.i.i1374 = getelementptr inbounds nuw i8, ptr %channel_left, i64 416, !dbg !10602
  %lanes.i.sroa.0.0.copyload.i = load <8 x float>, ptr %_26.i.i1374, align 32, !dbg !10603, !alias.scope !10608, !noalias !10612
  %295 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i302.sroa.0.0.copyload.i, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !10616
  %296 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i302.sroa.0.0.copyload.i, <8 x float> zeroinitializer, i8 0), !dbg !10622
  %297 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i307.sroa.0.0.copyload.i, <8 x float> zeroinitializer, i8 0), !dbg !10628
  %298 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1536, !dbg !10634
  %_17.i1375 = load i32, ptr %298, align 32, !dbg !10634, !alias.scope !10468, !noalias !10478, !noundef !11
  %299 = zext i32 %_17.i1375 to i64, !dbg !10634
; call compressor::kernel::fill_taps::<wide::f32x8_::f32x8>
  tail call fastcc void @_RINvNtCse3bfmKSZS8Y_10compressor6kernel9fill_tapsNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_(ptr noalias noundef nonnull readonly align 32 captures(address, read_provenance) dereferenceable(1568) %channel_left, i64 noundef %299, i64 noundef %_24, ptr noalias noundef nonnull align 4 %_36.0, i64 noundef range(i64 0, 2305843009213693952) %_36.1) #22, !dbg !10636, !noalias !10638
  %_24.i1376 = shl nuw nsw i64 %_24, 3, !dbg !10639
  %_94.not.i = icmp ugt i64 %_24.i1376, %_36.1
  br i1 %_94.not.i, label %bb33.i, label %bb31.i, !dbg !10641, !prof !239

bb31.i:                                           ; preds = %bb14.i80.i
  %_102.not.i = icmp ugt i64 %_24, %_37.1
  br i1 %_102.not.i, label %bb37.i, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_7IterMutNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EECse3bfmKSZS8Y_10compressor.exit.i, !dbg !10650, !prof !239

bb33.i:                                           ; preds = %bb14.i80.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_24.i1376, i64 noundef range(i64 0, 2305843009213693952) %_36.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_24f339c5043b5246624d94d053f2de71) #23, !dbg !10658, !noalias !10659
  unreachable, !dbg !10658

bb37.i:                                           ; preds = %bb31.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_24, i64 noundef range(i64 0, 288230376151711744) %_37.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7d1c771a0e9f109caa047fb8df8cf940) #23, !dbg !10660, !noalias !10659
  unreachable, !dbg !10660

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_7IterMutNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EECse3bfmKSZS8Y_10compressor.exit.i: ; preds = %bb31.i
  %.idx.i = shl nuw nsw i64 %_24, 5, !dbg !10661
  %300 = getelementptr inbounds nuw i8, ptr %_37.0, i64 %.idx.i, !dbg !10661
  %_6.i.i1377 = load i64, ptr %detector, align 8, !range !220, !alias.scope !10466, !noalias !10670
  %301 = getelementptr inbounds nuw i8, ptr %detector, i64 16
  %sidechain_left.1.i.i1378 = load i64, ptr %301, align 8, !alias.scope !10466, !noalias !10670
  %302 = getelementptr inbounds nuw i8, ptr %detector, i64 24
  %sidechain_right.0.i.i1379 = load ptr, ptr %302, align 8, !alias.scope !10466, !noalias !10670, !nonnull !11, !align !3663
  %303 = getelementptr inbounds nuw i8, ptr %detector, i64 32
  %sidechain_right.1.i.i1380 = load i64, ptr %303, align 8, !alias.scope !10466, !noalias !10670
  %304 = getelementptr inbounds nuw i8, ptr %detector, i64 8
  %sidechain_left.0.i.i1381 = load ptr, ptr %304, align 8, !alias.scope !10466, !noalias !10670, !nonnull !11, !align !3663
  %305 = icmp slt <8 x i32> %_13.i71.sroa.0.0.i, zeroinitializer
  %306 = bitcast <8 x float> %_11.i72.sroa.0.0889.i to <8 x i32>
  %307 = icmp slt <8 x i32> %306, zeroinitializer
  %308 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1512
  %_197.1.i = load i64, ptr %308, align 8, !alias.scope !10468, !noalias !10478
  %309 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1504
  %_197.0.i = load ptr, ptr %309, align 32, !alias.scope !10468, !noalias !10478, !nonnull !11
  %310 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1528
  %_198.1.i = load i64, ptr %310, align 8, !alias.scope !10468, !noalias !10478
  %311 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1520
  %_198.0.i = load ptr, ptr %311, align 16, !alias.scope !10468, !noalias !10478, !nonnull !11
  %312 = fneg <8 x float> %lanes.i287.sroa.0.0.copyload.i
  br label %bb9.i1382, !dbg !10671

bb9.i1382:                                        ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_log2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_7IterMutNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EECse3bfmKSZS8Y_10compressor.exit.i
  %head.sroa.0.0994.i = phi i64 [ %299, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_7IterMutNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EECse3bfmKSZS8Y_10compressor.exit.i ], [ %spec.store.select.i1393, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_log2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i ]
  %iter.sroa.12.0993.i = phi i64 [ 0, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_7IterMutNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EECse3bfmKSZS8Y_10compressor.exit.i ], [ %313, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_log2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i ]
  %313 = add nuw nsw i64 %iter.sroa.12.0993.i, 1, !dbg !10679
  %data.i.i.i.i.idx.i = shl i64 %iter.sroa.12.0993.i, 5, !dbg !10681
  %data.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_36.0, i64 %data.i.i.i.i.idx.i, !dbg !10681
  %_3.i.i.i.i.i = getelementptr inbounds nuw %"wide::f32x8_::f32x8", ptr %_37.0, i64 %iter.sroa.12.0993.i, !dbg !10684
  %_35.i = add nuw nsw i64 %iter.sroa.12.0993.i, %spec.store.select, !dbg !10687
  %slot7.i = shl i64 %_35.i, 3, !dbg !10687
  %_117.i = icmp samesign ugt i64 %slot7.i, %left.1, !dbg !10689
  br i1 %_117.i, label %bb42.i, label %bb43.i, !dbg !10689, !prof !161

bb10.i1395:                                       ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_log2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i
  %314 = trunc i64 %spec.store.select.i1393 to i32, !dbg !10696
  store i32 %314, ptr %298, align 32, !dbg !10696, !alias.scope !10468, !noalias !10478
  %315 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1472, !dbg !10697
  %gain_left.sroa.0.0.copyload.i = load <8 x float>, ptr %315, align 32, !dbg !10697, !alias.scope !10468, !noalias !10478
  br label %bb58.i, !dbg !10698

bb58.i:                                           ; preds = %bb10.i1395, %bb58.i
  %iter4.sroa.0.0997.i = phi ptr [ %_172.i, %bb58.i ], [ %_37.0, %bb10.i1395 ]
  %gain_left.sroa.0.0996.i = phi <8 x float> [ %329, %bb58.i ], [ %gain_left.sroa.0.0.copyload.i, %bb10.i1395 ]
  %_69.sroa.0.0.copyload.i = load <8 x float>, ptr %iter4.sroa.0.0997.i, align 32, !dbg !10706, !alias.scope !10472, !noalias !10708
  %_172.i = getelementptr inbounds nuw i8, ptr %iter4.sroa.0.0997.i, i64 32, !dbg !10709
  %316 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_69.sroa.0.0.copyload.i, <8 x float> %gain_left.sroa.0.0996.i, i8 17), !dbg !10712
  %317 = bitcast <8 x float> %316 to <8 x i32>, !dbg !10719
  %318 = icmp slt <8 x i32> %317, zeroinitializer, !dbg !10723
  %319 = select <8 x i1> %318, <8 x float> %lanes.i277.sroa.0.0.copyload.i, <8 x float> %lanes.i.sroa.0.0.copyload.i, !dbg !10723
  %320 = fsub <8 x float> %_69.sroa.0.0.copyload.i, %gain_left.sroa.0.0996.i, !dbg !10725
  %321 = fmul <8 x float> %320, %319, !dbg !10731
  %322 = fadd <8 x float> %gain_left.sroa.0.0996.i, %321, !dbg !10736
  %323 = bitcast <8 x float> %322 to <8 x i32>, !dbg !10740
  %324 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %322), !dbg !10746
  %325 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %324, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !10748
  %326 = bitcast <8 x float> %325 to <8 x i32>, !dbg !10754
  %327 = xor <8 x i32> %326, splat (i32 -1), !dbg !10760
  %328 = and <8 x i32> %323, %327, !dbg !10762
  %329 = bitcast <8 x i32> %328 to <8 x float>, !dbg !10766
  store <8 x i32> %328, ptr %iter4.sroa.0.0997.i, align 32, !dbg !10767, !alias.scope !10472, !noalias !10708
  %_166.i = icmp eq ptr %_172.i, %300, !dbg !10768
  br i1 %_166.i, label %bb26.lr.ph.i, label %bb58.i, !dbg !10698

bb26.lr.ph.i:                                     ; preds = %bb58.i
  store <8 x i32> %328, ptr %315, align 32, !dbg !10771, !alias.scope !10468, !noalias !10478
  %330 = bitcast <8 x float> %297 to <8 x i32>
  %331 = bitcast <8 x float> %296 to <8 x i32>
  %332 = select i1 %bypass, <8 x i32> %292, <8 x i32> %293
  %333 = or <8 x i32> %332, %331
  %334 = bitcast <8 x float> %295 to <8 x i32>
  %335 = icmp slt <8 x i32> %334, zeroinitializer
  br label %bb26.i1396, !dbg !10772

bb26.i1396:                                       ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i, %bb26.lr.ph.i
  %iter2.sroa.0.01002.i = phi ptr [ %_37.0, %bb26.lr.ph.i ], [ %_16.i.i.i, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i ]
  %iter2.sroa.7.01001.i = phi i64 [ 0, %bb26.lr.ph.i ], [ %_9.0.i547.i, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i ]
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %iter2.sroa.0.01002.i, i64 32, !dbg !10780
  %_9.0.i547.i = add nuw nsw i64 %iter2.sroa.7.01001.i, 1, !dbg !10783
  %_82.i = add nuw nsw i64 %iter2.sroa.7.01001.i, %spec.store.select, !dbg !10786
  %slot.i1397 = shl i64 %_82.i, 3, !dbg !10786
  %_185.i = icmp samesign ugt i64 %slot.i1397, %left.1, !dbg !10788
  br i1 %_185.i, label %bb64.i, label %bb65.i, !dbg !10788, !prof !161

bb65.i:                                           ; preds = %bb26.i1396
  %_188.i = sub nuw nsw i64 %left.1, %slot.i1397, !dbg !10793
  %_8.i354.i = icmp samesign ugt i64 %_188.i, 7, !dbg !10794
  br i1 %_8.i354.i, label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i, label %bb2.i355.i, !dbg !10794, !prof !2116

bb2.i355.i:                                       ; preds = %bb65.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_188.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !10799, !noalias !10800
  unreachable, !dbg !10799

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i: ; preds = %bb65.i
  %_192.i = getelementptr inbounds nuw float, ptr %left.0, i64 %slot.i1397, !dbg !10804
  %_87.sroa.0.0.copyload.i = load <8 x float>, ptr %iter2.sroa.0.01002.i, align 32, !dbg !10810, !alias.scope !10472, !noalias !10708
  %336 = fadd <8 x float> %lanes.i307.sroa.0.0.copyload.i, %_87.sroa.0.0.copyload.i, !dbg !10812
  %337 = fmul <8 x float> %336, splat (float 0x3FC542A5A0000000), !dbg !10818
  %338 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %337, <8 x float> splat (float -1.260000e+02)), !dbg !10824
  %339 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %338, <8 x float> splat (float 1.270000e+02)), !dbg !10830
  %340 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %339), !dbg !10835
  %341 = fsub <8 x float> %339, %340, !dbg !10840
  %342 = fmul <8 x float> %341, splat (float 0x3F5E974FA0000000), !dbg !10845
  %343 = fadd <8 x float> %342, splat (float 0x3F82778560000000), !dbg !10850
  %344 = fmul <8 x float> %341, %343, !dbg !10845
  %345 = fadd <8 x float> %344, splat (float 0x3FAC91CE60000000), !dbg !10850
  %346 = fmul <8 x float> %341, %345, !dbg !10845
  %347 = fadd <8 x float> %346, splat (float 0x3FCEBDB560000000), !dbg !10850
  %348 = fmul <8 x float> %341, %347, !dbg !10845
  %349 = fadd <8 x float> %348, splat (float 0x3FE62E4BA0000000), !dbg !10850
  %lanes.i351.sroa.0.0.copyload.i = load <8 x float>, ptr %_192.i, align 4, !dbg !10855, !alias.scope !10859, !noalias !10863
  %350 = fmul <8 x float> %341, %349, !dbg !10865
  %351 = fadd <8 x float> %350, splat (float 1.000000e+00), !dbg !10870
  %352 = fadd <8 x float> %340, splat (float 0x4160000FE0000000), !dbg !10875
  %353 = bitcast <8 x float> %352 to <8 x i32>, !dbg !10880
  %_3.i548.i = shl <8 x i32> %353, splat (i32 23), !dbg !10884
  %354 = bitcast <8 x i32> %_3.i548.i to <8 x float>, !dbg !10885
  %355 = fmul <8 x float> %351, %354, !dbg !10887
  %356 = fmul <8 x float> %lanes.i351.sroa.0.0.copyload.i, %355, !dbg !10891
  %357 = fsub <8 x float> %356, %lanes.i351.sroa.0.0.copyload.i, !dbg !10896
  %358 = fmul <8 x float> %lanes.i302.sroa.0.0.copyload.i, %357, !dbg !10902
  %359 = fadd <8 x float> %lanes.i351.sroa.0.0.copyload.i, %358, !dbg !10907
  %360 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_87.sroa.0.0.copyload.i, <8 x float> zeroinitializer, i8 0), !dbg !10911
  %361 = bitcast <8 x float> %360 to <8 x i32>, !dbg !10917
  %362 = and <8 x i32> %361, %330, !dbg !10921
  %363 = or <8 x i32> %333, %362, !dbg !10923
  %364 = select <8 x i1> %335, <8 x float> %356, <8 x float> %359, !dbg !10928
  %365 = icmp slt <8 x i32> %363, zeroinitializer, !dbg !10933
  %366 = select <8 x i1> %365, <8 x float> %lanes.i351.sroa.0.0.copyload.i, <8 x float> %364, !dbg !10933
  store <8 x float> %366, ptr %_192.i, align 4, !dbg !10938, !alias.scope !10943, !noalias !10947
  %_6.i.i.i = icmp eq ptr %_16.i.i.i, %300, !dbg !10951
  br i1 %_6.i.i.i, label %bb16, label %bb26.i1396, !dbg !10772

bb64.i:                                           ; preds = %bb26.i1396
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %slot.i1397, i64 noundef range(i64 8, 34359738361) %left.1, i64 noundef range(i64 8, 34359738361) %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_304103f4677a5cc18ac38120aae60310) #23, !dbg !10954, !noalias !10659
  unreachable, !dbg !10954

bb43.i:                                           ; preds = %bb9.i1382
  %_120.i = sub nuw nsw i64 %left.1, %slot7.i, !dbg !10955
  %_124.i = getelementptr inbounds nuw float, ptr %left.0, i64 %slot7.i, !dbg !10956
  %_8.i346.i = icmp samesign ugt i64 %_120.i, 7, !dbg !10960
  br i1 %_8.i346.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit349.i, label %bb2.i347.i, !dbg !10960, !prof !2116

bb2.i347.i:                                       ; preds = %bb43.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_120.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !10965, !noalias !10966
  unreachable, !dbg !10965

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit349.i: ; preds = %bb43.i
  %lanes.i343.sroa.0.0.copyload.i = load <8 x float>, ptr %_124.i, align 4, !dbg !10970, !alias.scope !10974, !noalias !10978
  switch i64 %_6.i.i1377, label %default.unreachable [
    i64 0, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i1392
    i64 1, label %bb3.i.i1401
    i64 2, label %bb2.i.i1383
  ], !dbg !10980

bb3.i.i1401:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit349.i
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i1392, !dbg !10983

bb2.i.i1383:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit349.i
  %_25.i.i1384 = icmp ugt i64 %slot7.i, %sidechain_left.1.i.i1378, !dbg !10984
  br i1 %_25.i.i1384, label %bb17.i.i1400, label %bb18.i.i1385, !dbg !10984, !prof !161

bb18.i.i1385:                                     ; preds = %bb2.i.i1383
  %_28.i.i1386 = sub nuw i64 %sidechain_left.1.i.i1378, %slot7.i, !dbg !10987
  %_8.i322.i = icmp samesign ugt i64 %_28.i.i1386, 7, !dbg !10988
  br i1 %_8.i322.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit325.i, label %bb2.i323.i, !dbg !10988, !prof !2116

bb2.i323.i:                                       ; preds = %bb18.i.i1385
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_28.i.i1386, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !10993, !noalias !10994
  unreachable, !dbg !10993

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit325.i: ; preds = %bb18.i.i1385
  %_32.i.i1387 = getelementptr inbounds nuw float, ptr %sidechain_left.0.i.i1381, i64 %slot7.i, !dbg !11003
  %lanes.i319.sroa.0.0.copyload.i = load <8 x float>, ptr %_32.i.i1387, align 4, !dbg !11005, !alias.scope !11009, !noalias !11013
  %_33.i.i1388 = icmp ugt i64 %slot7.i, %sidechain_right.1.i.i1380, !dbg !11015
  br i1 %_33.i.i1388, label %bb19.i.i1399, label %bb20.i.i1389, !dbg !11015, !prof !161

bb17.i.i1400:                                     ; preds = %bb2.i.i1383
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 34359738361) %slot7.i, i64 noundef %sidechain_left.1.i.i1378, i64 noundef %sidechain_left.1.i.i1378, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a0a4023c361dd50869bbb08b76482f50) #23, !dbg !11018, !noalias !11019
  unreachable, !dbg !11018

bb20.i.i1389:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit325.i
  %_36.i.i1390 = sub nuw i64 %sidechain_right.1.i.i1380, %slot7.i, !dbg !11021
  %_8.i314.i = icmp samesign ugt i64 %_36.i.i1390, 7, !dbg !11022
  br i1 %_8.i314.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit317.i, label %bb2.i315.i, !dbg !11022, !prof !2116

bb2.i315.i:                                       ; preds = %bb20.i.i1389
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_36.i.i1390, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !11027, !noalias !11028
  unreachable, !dbg !11027

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit317.i: ; preds = %bb20.i.i1389
  %_40.i.i1391 = getelementptr inbounds nuw float, ptr %sidechain_right.0.i.i1379, i64 %slot7.i, !dbg !11032
  %lanes.i312.sroa.0.0.copyload.i = load <8 x float>, ptr %_40.i.i1391, align 4, !dbg !11034, !alias.scope !11038, !noalias !11042
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i1392, !dbg !11044

bb19.i.i1399:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit325.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 34359738361) %slot7.i, i64 noundef %sidechain_right.1.i.i1380, i64 noundef %sidechain_right.1.i.i1380, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5807a2a07e9839e3d32631b4243eb17f) #23, !dbg !11045, !noalias !11019
  unreachable, !dbg !11045

_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i1392: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit317.i, %bb3.i.i1401, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit349.i
  %.sroa.0.0.i = phi <8 x float> [ %lanes.i343.sroa.0.0.copyload.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit349.i ], [ zeroinitializer, %bb3.i.i1401 ], [ %lanes.i319.sroa.0.0.copyload.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit317.i ], !dbg !11046
  %.sroa.0566.0.i = phi <8 x float> [ %lanes.i343.sroa.0.0.copyload.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit349.i ], [ zeroinitializer, %bb3.i.i1401 ], [ %lanes.i312.sroa.0.0.copyload.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit317.i ], !dbg !11046
  %367 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %.sroa.0.0.i), !dbg !11047
  %368 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %.sroa.0566.0.i), !dbg !11053
  %369 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %367, <8 x float> %368), !dbg !11059
  %370 = fmul <8 x float> %367, splat (float 5.000000e-01), !dbg !11064
  %371 = fmul <8 x float> %368, splat (float 5.000000e-01), !dbg !11069
  %372 = fadd <8 x float> %370, %371, !dbg !11074
  %373 = select <8 x i1> %305, <8 x float> %372, <8 x float> %369, !dbg !11079
  %374 = select <8 x i1> %307, <8 x float> %373, <8 x float> %367, !dbg !11084
  %375 = add i64 %head.sroa.0.0994.i, 1, !dbg !11089
  %_44.i = icmp eq i64 %375, %ring_length.i1372, !dbg !11091
  %spec.store.select.i1393 = select i1 %_44.i, i64 0, i64 %375, !dbg !11091
  %_48.i = shl i64 %head.sroa.0.0994.i, 3, !dbg !11093
  %_128.i = icmp ugt i64 %_48.i, %_197.1.i, !dbg !11095
  br i1 %_128.i, label %bb44.i, label %bb45.i, !dbg !11095, !prof !161

bb42.i:                                           ; preds = %bb9.i1382
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %slot7.i, i64 noundef range(i64 8, 34359738361) %left.1, i64 noundef range(i64 8, 34359738361) %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d0cf1632f8c73ea3a4ae894eff0af7e2) #23, !dbg !11101, !noalias !10659
  unreachable, !dbg !11101

bb45.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i1392
  %_131.i = sub nuw i64 %_197.1.i, %_48.i, !dbg !11102
  %_8.i487.i = icmp samesign ugt i64 %_131.i, 7, !dbg !11103
  br i1 %_8.i487.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit489.i, label %bb2.i488.i, !dbg !11103, !prof !2116

bb2.i488.i:                                       ; preds = %bb45.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_131.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !11108, !noalias !11109
  unreachable, !dbg !11108

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit489.i: ; preds = %bb45.i
  %_135.i = getelementptr inbounds nuw float, ptr %_197.0.i, i64 %_48.i, !dbg !11113
  store <8 x float> %lanes.i343.sroa.0.0.copyload.i, ptr %_135.i, align 4, !dbg !11118, !alias.scope !11122, !noalias !11126
  %_136.i = icmp ugt i64 %_48.i, %_198.1.i, !dbg !11128
  br i1 %_136.i, label %bb46.i, label %bb47.i, !dbg !11128, !prof !161

bb44.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i1392
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_48.i, i64 noundef %_197.1.i, i64 noundef %_197.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1550388c01c016d45f0fc0e187831359) #23, !dbg !11132, !noalias !10659
  unreachable, !dbg !11132

bb47.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit489.i
  %_139.i = sub nuw i64 %_198.1.i, %_48.i, !dbg !11133
  %_8.i483.i = icmp samesign ugt i64 %_139.i, 7, !dbg !11134
  br i1 %_8.i483.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit485.i, label %bb2.i484.i, !dbg !11134, !prof !2116

bb2.i484.i:                                       ; preds = %bb47.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_139.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !11139, !noalias !11140
  unreachable, !dbg !11139

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit485.i: ; preds = %bb47.i
  %_143.i = getelementptr inbounds nuw float, ptr %_198.0.i, i64 %_48.i, !dbg !11144
  store <8 x float> %374, ptr %_143.i, align 4, !dbg !11149, !alias.scope !11153, !noalias !11157
  %_57.i1394 = shl i64 %spec.store.select.i1393, 3, !dbg !11159
  %_144.i = icmp ugt i64 %_57.i1394, %_197.1.i, !dbg !11160
  br i1 %_144.i, label %bb48.i, label %bb49.i, !dbg !11160, !prof !161

bb46.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit489.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_48.i, i64 noundef %_198.1.i, i64 noundef %_198.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ea3ee8fdebdd5c17ba50662ba5eee9b1) #23, !dbg !11164, !noalias !10659
  unreachable, !dbg !11164

bb49.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit485.i
  %_147.i = sub nuw i64 %_197.1.i, %_57.i1394, !dbg !11165
  %_8.i338.i = icmp samesign ugt i64 %_147.i, 7, !dbg !11166
  br i1 %_8.i338.i, label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_log2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i, label %bb2.i339.i, !dbg !11166, !prof !2116

bb2.i339.i:                                       ; preds = %bb49.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_147.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !11171, !noalias !11172
  unreachable, !dbg !11171

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_log2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i: ; preds = %bb49.i
  %_151.i = getelementptr inbounds nuw float, ptr %_197.0.i, i64 %_57.i1394, !dbg !11176
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_124.i, ptr noundef nonnull align 4 dereferenceable(32) %_151.i, i64 32, i1 false), !dbg !11181, !noalias !11186
  %lanes.i327.sroa.0.0.copyload.i = load <8 x float>, ptr %data.i.i.i.i.i, align 4, !dbg !11187, !alias.scope !11192, !noalias !11196
  %376 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i327.sroa.0.0.copyload.i, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !11200
  %377 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %376, <8 x float> splat (float 0x3810000000000000)), !dbg !11206
  %378 = bitcast <8 x float> %377 to <4 x i64>, !dbg !11213
  %379 = and <4 x i64> %378, splat (i64 36028792732385279), !dbg !11214
  %380 = or disjoint <4 x i64> %379, splat (i64 4575657222473777152), !dbg !11219
  %381 = bitcast <4 x i64> %380 to <8 x float>, !dbg !11223
  %382 = fadd <8 x float> %381, splat (float -1.000000e+00), !dbg !11224
  %383 = fmul <8 x float> %382, splat (float 0x3F9B17A960000000), !dbg !11229
  %384 = fsub <8 x float> splat (float 0x3FBF9A8440000000), %383, !dbg !11234
  %385 = fmul <8 x float> %382, %384, !dbg !11229
  %386 = fadd <8 x float> %385, splat (float 0xBFD1E3F400000000), !dbg !11234
  %387 = fmul <8 x float> %382, %386, !dbg !11229
  %388 = fadd <8 x float> %387, splat (float 0x3FDD544F20000000), !dbg !11234
  %389 = fmul <8 x float> %382, %388, !dbg !11229
  %390 = fadd <8 x float> %389, splat (float 0xBFE6FC2A60000000), !dbg !11234
  %391 = fmul <8 x float> %382, %390, !dbg !11229
  %392 = fadd <8 x float> %391, splat (float 0x3FF714B2A0000000), !dbg !11234
  %393 = bitcast <8 x float> %377 to <8 x i32>, !dbg !11239
  %_3.i549.i = lshr <8 x i32> %393, splat (i32 23), !dbg !11243
  %394 = or disjoint <8 x i32> %_3.i549.i, splat (i32 1258291200), !dbg !11244
  %395 = bitcast <8 x i32> %394 to <8 x float>, !dbg !11248
  %396 = fadd <8 x float> %395, splat (float 0xC160000FE0000000), !dbg !11249
  %397 = fmul <8 x float> %382, %392, !dbg !11253
  %398 = fadd <8 x float> %396, %397, !dbg !11258
  %399 = fmul <8 x float> %398, splat (float 0x4018151820000000), !dbg !11263
  %400 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %399, <8 x float> splat (float -1.600000e+02)), !dbg !11268
  %401 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %400, <8 x float> splat (float 2.400000e+01)), !dbg !11273
  %402 = fsub <8 x float> %401, %lanes.i297.sroa.0.0.copyload.i, !dbg !11278
  %403 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %402, <8 x float> %lanes.i287.sroa.0.0.copyload.i, i8 30), !dbg !11284
  %404 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %402, <8 x float> %312, i8 18), !dbg !11290
  %405 = fadd <8 x float> %lanes.i287.sroa.0.0.copyload.i, %402, !dbg !11296
  %406 = fmul <8 x float> %405, %405, !dbg !11301
  %407 = fmul <8 x float> %lanes.i282.sroa.0.0.copyload.i, %406, !dbg !11306
  %408 = bitcast <8 x float> %403 to <8 x i32>, !dbg !11311
  %409 = icmp slt <8 x i32> %408, zeroinitializer, !dbg !11315
  %.v.i = select <8 x i1> %409, <8 x float> %402, <8 x float> %407, !dbg !11315
  %410 = fmul <8 x float> %lanes.i292.sroa.0.0.copyload.i, %.v.i, !dbg !11315
  %411 = bitcast <8 x float> %404 to <8 x i32>, !dbg !11317
  %412 = icmp slt <8 x i32> %411, zeroinitializer, !dbg !11321
  %413 = select <8 x i1> %412, <8 x float> zeroinitializer, <8 x float> %410, !dbg !11321
  %414 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %413, <8 x float> splat (float -1.000000e+02)), !dbg !11323
  %415 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %414, <8 x float> zeroinitializer), !dbg !11328
  store <8 x float> %415, ptr %_3.i.i.i.i.i, align 32, !dbg !11333, !alias.scope !10472, !noalias !10708
  %exitcond.not.i = icmp eq i64 %313, %_24, !dbg !10671
  br i1 %exitcond.not.i, label %bb10.i1395, label %bb9.i1382, !dbg !10671

bb48.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit485.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_57.i1394, i64 noundef %_197.1.i, i64 noundef %_197.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_132307dc77e2af71f798d527e6fd1d3e) #23, !dbg !11334, !noalias !10659
  unreachable, !dbg !11334
}

define internal fastcc void @_RINvNtCse3bfmKSZS8Y_10compressor6kernel9fill_tapsNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_(ptr noalias noundef nonnull readonly align 32 captures(none) dereferenceable(1568) %channel, i64 noundef range(i64 0, 4294967296) %write, i64 noundef range(i64 -4294967294, 4294967296) %len, ptr noalias noundef nonnull writeonly align 4 captures(none) %scratch.0, i64 noundef range(i64 0, 2305843009213693952) %scratch.1) unnamed_addr #1 personality ptr @rust_eh_personality !dbg !11335 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %channel, i64 1540, !dbg !11336
  %_6 = load i32, ptr %0, align 4, !dbg !11336, !noundef !11
  %ring_length = zext i32 %_6 to i64, !dbg !11336
  %_8 = shl nsw i64 %len, 3, !dbg !11338
  %_35.not = icmp ugt i64 %_8, %scratch.1
  br i1 %_35.not, label %bb11, label %bb1.preheader, !dbg !11340, !prof !239

bb1.preheader:                                    ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %channel, i64 1440
  %2 = getelementptr inbounds nuw i8, ptr %channel, i64 1520
  %_78.0 = load ptr, ptr %2, align 16, !nonnull !11, !noundef !11
  %3 = getelementptr inbounds nuw i8, ptr %channel, i64 1528
  %_78.1 = load i64, ptr %3, align 8, !noundef !11
  br label %bb14, !dbg !11349

bb11:                                             ; preds = %start
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_8, i64 noundef %scratch.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6e11da28860dc4315c923646233703e6) #23, !dbg !11357
  unreachable, !dbg !11357

bb1.loopexit:                                     ; preds = %bb9.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter14ChunksExactMutfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_11ChunksExactfEECse3bfmKSZS8Y_10compressor.exit58
  %exitcond126.not = icmp eq i64 %4, 8, !dbg !11358
  br i1 %exitcond126.not, label %bb15, label %bb14, !dbg !11349

bb15:                                             ; preds = %bb1.loopexit
  ret void, !dbg !11360

bb14:                                             ; preds = %bb1.preheader, %bb1.loopexit
  %iter.sroa.0.0115 = phi i64 [ 0, %bb1.preheader ], [ %4, %bb1.loopexit ]
  %4 = add nuw nsw i64 %iter.sroa.0.0115, 1, !dbg !11361
  %5 = getelementptr inbounds nuw i32, ptr %1, i64 %iter.sroa.0.0115, !dbg !11367
  %_12 = load i32, ptr %5, align 4, !dbg !11367, !noundef !11
  %delay = zext i32 %_12 to i64, !dbg !11367
  %_15.not = icmp samesign ult i64 %write, %delay, !dbg !11369
  %_16 = select i1 %_15.not, i64 %ring_length, i64 0, !dbg !11369
  %write.pn = sub nsw i64 %write, %delay, !dbg !11369
  %row.sroa.0.0 = add nsw i64 %write.pn, %_16, !dbg !11371
  %_18 = sub nsw i64 %ring_length, %row.sroa.0.0, !dbg !11372
  %..i = tail call noundef i64 @llvm.umin.i64(i64 %len, i64 %_18), !dbg !11374
  %_22 = shl nsw i64 %row.sroa.0.0, 3, !dbg !11376
  %_25 = add nsw i64 %..i, %row.sroa.0.0, !dbg !11378
  %_24 = shl nsw i64 %_25, 3, !dbg !11378
  %_53 = icmp ult i64 %_24, %_22, !dbg !11379
  br i1 %_53, label %bb18, label %bb20, !dbg !11379, !prof !161

bb20:                                             ; preds = %bb14
  %_54 = shl nsw i64 %..i, 3, !dbg !11385
  %_47.not = icmp ugt i64 %_24, %_78.1, !dbg !11386
  br i1 %_47.not, label %bb18, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter14ChunksExactMutfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_11ChunksExactfEECse3bfmKSZS8Y_10compressor.exit, !dbg !11386, !prof !161

bb18:                                             ; preds = %bb20, %bb14
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22, i64 noundef %_24, i64 noundef %_78.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_22f7da482334bdfa74d39682e51abbdd) #23, !dbg !11387
  unreachable, !dbg !11387

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter14ChunksExactMutfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_11ChunksExactfEECse3bfmKSZS8Y_10compressor.exit: ; preds = %bb20
  %_56 = getelementptr inbounds nuw float, ptr %_78.0, i64 %_22, !dbg !11388
  %n.i.i.i.i95 = and i64 %..i, 2305843009213693951, !dbg !11392
  %invariant.gep = getelementptr float, ptr %_56, i64 %iter.sroa.0.0115, !dbg !11411
  %invariant.gep105 = getelementptr float, ptr %scratch.0, i64 %iter.sroa.0.0115, !dbg !11411
  %_2.i107.not = icmp eq i64 %n.i.i.i.i95, 0, !dbg !11412
  br i1 %_2.i107.not, label %_RNvNtCse3bfmKSZS8Y_10compressor6kernel9copy_lane.exit24, label %bb9.i21, !dbg !11412

bb9.i21:                                          ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter14ChunksExactMutfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_11ChunksExactfEECse3bfmKSZS8Y_10compressor.exit, %bb9.i21
  %iter.i7.sroa.16.0108 = phi i64 [ %6, %bb9.i21 ], [ 0, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter14ChunksExactMutfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_11ChunksExactfEECse3bfmKSZS8Y_10compressor.exit ]
  %6 = add nuw nsw i64 %iter.i7.sroa.16.0108, 1, !dbg !11415
  %start1.i.i = shl i64 %iter.i7.sroa.16.0108, 3, !dbg !11417
  %gep = getelementptr float, ptr %invariant.gep, i64 %start1.i.i, !dbg !11420
  %_14.i23 = load float, ptr %gep, align 4, !dbg !11420, !noundef !11
  %gep106 = getelementptr float, ptr %invariant.gep105, i64 %start1.i.i, !dbg !11421
  store float %_14.i23, ptr %gep106, align 4, !dbg !11421
  %exitcond.not = icmp eq i64 %6, %n.i.i.i.i95, !dbg !11412
  br i1 %exitcond.not, label %_RNvNtCse3bfmKSZS8Y_10compressor6kernel9copy_lane.exit24, label %bb9.i21, !dbg !11412

_RNvNtCse3bfmKSZS8Y_10compressor6kernel9copy_lane.exit24: ; preds = %bb9.i21, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter14ChunksExactMutfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_11ChunksExactfEECse3bfmKSZS8Y_10compressor.exit
  %_31 = sub nsw i64 %len, %..i, !dbg !11422
  %_30 = shl nsw i64 %_31, 3, !dbg !11422
  %_63.not = icmp ugt i64 %_30, %_78.1
  br i1 %_63.not, label %bb27, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter14ChunksExactMutfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_11ChunksExactfEECse3bfmKSZS8Y_10compressor.exit58, !dbg !11423, !prof !239

bb27:                                             ; preds = %_RNvNtCse3bfmKSZS8Y_10compressor6kernel9copy_lane.exit24
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_30, i64 noundef %_78.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6bab2b83b03bbfaea58c986adce7dabd) #23, !dbg !11432
  unreachable, !dbg !11432

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter14ChunksExactMutfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_11ChunksExactfEECse3bfmKSZS8Y_10compressor.exit58: ; preds = %_RNvNtCse3bfmKSZS8Y_10compressor6kernel9copy_lane.exit24
  %_73 = sub nuw nsw i64 %_8, %_54, !dbg !11433
  %_77 = getelementptr inbounds nuw float, ptr %scratch.0, i64 %_54, !dbg !11439
  %n.i.i.i.i4896 = lshr exact i64 %_73, 3, !dbg !11444
  %n.i.i3.i.i4997 = and i64 %_31, 2305843009213693951, !dbg !11451
  %..i.i.i50 = tail call noundef i64 @llvm.umin.i64(i64 %n.i.i3.i.i4997, i64 %n.i.i.i.i4896), !dbg !11456
  %invariant.gep109 = getelementptr float, ptr %_78.0, i64 %iter.sroa.0.0115, !dbg !11460
  %invariant.gep111 = getelementptr float, ptr %_77, i64 %iter.sroa.0.0115, !dbg !11460
  %_2.i61113.not = icmp eq i64 %..i.i.i50, 0, !dbg !11461
  br i1 %_2.i61113.not, label %bb1.loopexit, label %bb9.i, !dbg !11461

bb9.i:                                            ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter14ChunksExactMutfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_11ChunksExactfEECse3bfmKSZS8Y_10compressor.exit58, %bb9.i
  %iter.i.sroa.16.0114 = phi i64 [ %7, %bb9.i ], [ 0, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter14ChunksExactMutfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_11ChunksExactfEECse3bfmKSZS8Y_10compressor.exit58 ]
  %7 = add nuw nsw i64 %iter.i.sroa.16.0114, 1, !dbg !11464
  %start1.i.i66 = shl i64 %iter.i.sroa.16.0114, 3, !dbg !11465
  %gep110 = getelementptr float, ptr %invariant.gep109, i64 %start1.i.i66, !dbg !11467
  %_14.i = load float, ptr %gep110, align 4, !dbg !11467, !noundef !11
  %gep112 = getelementptr float, ptr %invariant.gep111, i64 %start1.i.i66, !dbg !11468
  store float %_14.i, ptr %gep112, align 4, !dbg !11468
  %exitcond125.not = icmp eq i64 %7, %..i.i.i50, !dbg !11461
  br i1 %exitcond125.not, label %bb1.loopexit, label %bb9.i, !dbg !11461
}

