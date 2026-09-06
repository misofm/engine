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
  %5 = bitcast <8 x i32> %4 to <8 x float>, !dbg !1711
  switch i32 %link, label %bb13.i645 [
    i32 1, label %bb14.i646
    i32 3, label %bb14.i646.fold.split
  ], !dbg !1732

bb13.i645:                                        ; preds = %bb7
  br label %bb14.i646, !dbg !1736

bb14.i646.fold.split:                             ; preds = %bb7
  br label %bb14.i646, !dbg !1737

bb14.i646:                                        ; preds = %bb7, %bb14.i646.fold.split, %bb13.i645
  %_11.i637.sroa.0.04101 = phi <8 x float> [ %5, %bb7 ], [ %2, %bb13.i645 ], [ %2, %bb14.i646.fold.split ]
  %_13.i636.sroa.0.0 = phi <8 x i32> [ %4, %bb7 ], [ %4, %bb13.i645 ], [ %3, %bb14.i646.fold.split ], !dbg !1738
  %_15.i42 = getelementptr inbounds nuw i8, ptr %channels.0, i64 256, !dbg !1739
  %_4.i746 = getelementptr inbounds nuw i8, ptr %channels.0, i64 448, !dbg !1741
  %_7.i747 = getelementptr inbounds nuw i8, ptr %channels.0, i64 480, !dbg !1745
  %_14.i748 = getelementptr inbounds nuw i8, ptr %channels.0, i64 288, !dbg !1747
  %_17.i749 = getelementptr inbounds nuw i8, ptr %channels.0, i64 320, !dbg !1749
  %_20.i750 = getelementptr inbounds nuw i8, ptr %channels.0, i64 352, !dbg !1750
  %_23.i751 = getelementptr inbounds nuw i8, ptr %channels.0, i64 384, !dbg !1751
  %_26.i752 = getelementptr inbounds nuw i8, ptr %channels.0, i64 416, !dbg !1752
  %_17.i43 = getelementptr inbounds nuw i8, ptr %channels.1, i64 256, !dbg !1753
  %_4.i724 = getelementptr inbounds nuw i8, ptr %channels.1, i64 448, !dbg !1755
  %_7.i725 = getelementptr inbounds nuw i8, ptr %channels.1, i64 480, !dbg !1757
  %_14.i726 = getelementptr inbounds nuw i8, ptr %channels.1, i64 288, !dbg !1758
  %_17.i727 = getelementptr inbounds nuw i8, ptr %channels.1, i64 320, !dbg !1759
  %_20.i728 = getelementptr inbounds nuw i8, ptr %channels.1, i64 352, !dbg !1760
  %_23.i729 = getelementptr inbounds nuw i8, ptr %channels.1, i64 384, !dbg !1761
  %_26.i730 = getelementptr inbounds nuw i8, ptr %channels.1, i64 416, !dbg !1762
  %6 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1440, !dbg !1763
  %_91.i46 = load i32, ptr %6, align 32, !dbg !1763, !alias.scope !1687, !noalias !1696, !noundef !11
  %_90.i47 = zext i32 %_91.i46 to i64, !dbg !1763
  %7 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1444, !dbg !1768
  %_94.i148 = load i32, ptr %7, align 4, !dbg !1768, !alias.scope !1687, !noalias !1696, !noundef !11
  %_92.not.i149 = icmp ne i32 %_94.i148, %_91.i46, !dbg !1768
  %8 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1448, !dbg !1768
  %_94.i148.1 = load i32, ptr %8, align 4, !dbg !1768
  %_92.not.i149.1 = icmp ne i32 %_94.i148.1, %_91.i46, !dbg !1768
  %or.cond.not7232 = select i1 %_92.not.i149, i1 true, i1 %_92.not.i149.1, !dbg !1768
  %9 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1452, !dbg !1768
  %_94.i148.2 = load i32, ptr %9, align 4, !dbg !1768
  %_92.not.i149.2 = icmp ne i32 %_94.i148.2, %_91.i46, !dbg !1768
  %or.cond7202.not7231 = select i1 %or.cond.not7232, i1 true, i1 %_92.not.i149.2, !dbg !1768
  %10 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1456, !dbg !1768
  %_94.i148.3 = load i32, ptr %10, align 4, !dbg !1768
  %_92.not.i149.3 = icmp ne i32 %_94.i148.3, %_91.i46, !dbg !1768
  %or.cond7203.not7230 = select i1 %or.cond7202.not7231, i1 true, i1 %_92.not.i149.3, !dbg !1768
  %11 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1460, !dbg !1768
  %_94.i148.4 = load i32, ptr %11, align 4, !dbg !1768
  %_92.not.i149.4 = icmp ne i32 %_94.i148.4, %_91.i46, !dbg !1768
  %or.cond7204.not7229 = select i1 %or.cond7203.not7230, i1 true, i1 %_92.not.i149.4, !dbg !1768
  %12 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1464, !dbg !1768
  %_94.i148.5 = load i32, ptr %12, align 4, !dbg !1768
  %_92.not.i149.5 = icmp ne i32 %_94.i148.5, %_91.i46, !dbg !1768
  %or.cond7205.not7228 = select i1 %or.cond7204.not7229, i1 true, i1 %_92.not.i149.5, !dbg !1768
  %13 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1468, !dbg !1768
  %_94.i148.6 = load i32, ptr %13, align 4, !dbg !1768
  %_92.not.i149.6 = icmp ne i32 %_94.i148.6, %_91.i46, !dbg !1768
  %or.cond7206.not = select i1 %or.cond7205.not7228, i1 true, i1 %_92.not.i149.6, !dbg !1768
  %14 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1440, !dbg !1772
  %_101.i53 = load i32, ptr %14, align 32, !dbg !1772, !alias.scope !1690, !noalias !1775, !noundef !11
  %_100.i54 = zext i32 %_101.i53 to i64, !dbg !1772
  %15 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1444, !dbg !1776
  %_104.i145 = load i32, ptr %15, align 4, !dbg !1776, !alias.scope !1690, !noalias !1775, !noundef !11
  %_102.not.i146 = icmp ne i32 %_104.i145, %_101.i53, !dbg !1776
  %16 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1448, !dbg !1776
  %_104.i145.1 = load i32, ptr %16, align 4, !dbg !1776
  %_102.not.i146.1 = icmp ne i32 %_104.i145.1, %_101.i53, !dbg !1776
  %or.cond7207.not7237 = select i1 %_102.not.i146, i1 true, i1 %_102.not.i146.1, !dbg !1776
  %17 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1452, !dbg !1776
  %_104.i145.2 = load i32, ptr %17, align 4, !dbg !1776
  %_102.not.i146.2 = icmp ne i32 %_104.i145.2, %_101.i53, !dbg !1776
  %or.cond7208.not7236 = select i1 %or.cond7207.not7237, i1 true, i1 %_102.not.i146.2, !dbg !1776
  %18 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1456, !dbg !1776
  %_104.i145.3 = load i32, ptr %18, align 4, !dbg !1776
  %_102.not.i146.3 = icmp ne i32 %_104.i145.3, %_101.i53, !dbg !1776
  %or.cond7209.not7235 = select i1 %or.cond7208.not7236, i1 true, i1 %_102.not.i146.3, !dbg !1776
  %19 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1460, !dbg !1776
  %_104.i145.4 = load i32, ptr %19, align 4, !dbg !1776
  %_102.not.i146.4 = icmp ne i32 %_104.i145.4, %_101.i53, !dbg !1776
  %or.cond7210.not7234 = select i1 %or.cond7209.not7235, i1 true, i1 %_102.not.i146.4, !dbg !1776
  %20 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1464, !dbg !1776
  %_104.i145.5 = load i32, ptr %20, align 4, !dbg !1776
  %_102.not.i146.5 = icmp ne i32 %_104.i145.5, %_101.i53, !dbg !1776
  %or.cond7211.not7233 = select i1 %or.cond7210.not7234, i1 true, i1 %_102.not.i146.5, !dbg !1776
  %21 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1468, !dbg !1776
  %_104.i145.6 = load i32, ptr %21, align 4, !dbg !1776
  %_102.not.i146.6 = icmp ne i32 %_104.i145.6, %_101.i53, !dbg !1776
  %or.cond7212.not = select i1 %or.cond7211.not7233, i1 true, i1 %_102.not.i146.6, !dbg !1776
  %22 = icmp ne ptr %.sroa.6.0.copyload, null
  %23 = icmp ne ptr %.sroa.4.0.copyload, null
  %24 = icmp slt <8 x i32> %_13.i636.sroa.0.0, zeroinitializer
  %25 = bitcast <8 x float> %_11.i637.sroa.0.04101 to <8 x i32>
  %26 = icmp slt <8 x i32> %25, zeroinitializer
  %27 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1536
  %28 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1504
  %29 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1512
  %30 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1528
  %31 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1520
  %32 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1504
  %33 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1512
  %34 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1528
  %35 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1520
  %36 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1540
  %_73.i130 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1472
  %37 = select i1 %bypass, <8 x i32> %3, <8 x i32> %4
  %_77.i131 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1472
  %38 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1536
  %39 = lshr i64 %left.1, 3, !dbg !1780
  %40 = lshr i64 %right.1, 3, !dbg !1780
  %41 = add nuw nsw i64 %left.1, 8, !dbg !1780
  %42 = lshr i64 %41, 3, !dbg !1780
  %43 = add nuw nsw i64 %right.1, 8, !dbg !1780
  %44 = lshr i64 %43, 3, !dbg !1780
  %45 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1448
  %46 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1452
  %47 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1456
  %48 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1460
  %49 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1464
  %50 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1468
  %51 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1448
  %52 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1452
  %53 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1456
  %54 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1460
  %55 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1464
  %56 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1468
  br label %bb41.i63, !dbg !1780

bb41.i63:                                         ; preds = %bb14.i646, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit562
  %start1.sroa.0.0.i615030 = phi i64 [ 0, %bb14.i646 ], [ %57, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit562 ]
; call <compressor::kernel::Channel<wide::f32x8_::f32x8>>::advance_ramps
  tail call fastcc void @_RNvMNtCse3bfmKSZS8Y_10compressor6kernelINtB2_7ChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13advance_rampsB4_(ptr noalias noundef nonnull align 32 dereferenceable(1568) %channels.0, i32 noundef %sample_rate) #22, !dbg !1789, !noalias !1791
; call <compressor::kernel::Channel<wide::f32x8_::f32x8>>::advance_ramps
  tail call fastcc void @_RNvMNtCse3bfmKSZS8Y_10compressor6kernelINtB2_7ChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13advance_rampsB4_(ptr noalias noundef nonnull align 32 dereferenceable(1568) %channels.1, i32 noundef %sample_rate) #22, !dbg !1792, !noalias !1791
  %lanes.i1774.sroa.0.0.copyload = load <8 x float>, ptr %_4.i746, align 32, !dbg !1793, !alias.scope !1799, !noalias !1803
  %lanes.i1767.sroa.0.0.copyload = load <8 x float>, ptr %_7.i747, align 32, !dbg !1809, !alias.scope !1814, !noalias !1818
  %lanes.i1760.sroa.0.0.copyload = load <8 x float>, ptr %_15.i42, align 32, !dbg !1822, !alias.scope !1827, !noalias !1831
  %lanes.i1753.sroa.0.0.copyload = load <8 x float>, ptr %_14.i748, align 32, !dbg !1835, !alias.scope !1840, !noalias !1844
  %lanes.i1746.sroa.0.0.copyload = load <8 x float>, ptr %_17.i749, align 32, !dbg !1848, !alias.scope !1853, !noalias !1857
  %lanes.i1739.sroa.0.0.copyload = load <8 x float>, ptr %_20.i750, align 32, !dbg !1861, !alias.scope !1866, !noalias !1870
  %lanes.i1732.sroa.0.0.copyload = load <8 x float>, ptr %_23.i751, align 32, !dbg !1874, !alias.scope !1879, !noalias !1883
  %lanes.i1725.sroa.0.0.copyload = load <8 x float>, ptr %_26.i752, align 32, !dbg !1887, !alias.scope !1892, !noalias !1896
  %lanes.i1830.sroa.0.0.copyload = load <8 x float>, ptr %_4.i724, align 32, !dbg !1900, !alias.scope !1906, !noalias !1910
  %lanes.i1823.sroa.0.0.copyload = load <8 x float>, ptr %_7.i725, align 32, !dbg !1916, !alias.scope !1921, !noalias !1925
  %lanes.i1816.sroa.0.0.copyload = load <8 x float>, ptr %_17.i43, align 32, !dbg !1929, !alias.scope !1934, !noalias !1938
  %lanes.i1809.sroa.0.0.copyload = load <8 x float>, ptr %_14.i726, align 32, !dbg !1942, !alias.scope !1947, !noalias !1951
  %lanes.i1802.sroa.0.0.copyload = load <8 x float>, ptr %_17.i727, align 32, !dbg !1955, !alias.scope !1960, !noalias !1964
  %lanes.i1795.sroa.0.0.copyload = load <8 x float>, ptr %_20.i728, align 32, !dbg !1968, !alias.scope !1973, !noalias !1977
  %lanes.i1788.sroa.0.0.copyload = load <8 x float>, ptr %_23.i729, align 32, !dbg !1981, !alias.scope !1986, !noalias !1990
  %lanes.i1781.sroa.0.0.copyload = load <8 x float>, ptr %_26.i730, align 32, !dbg !1994, !alias.scope !1999, !noalias !2003
  %57 = add nuw nsw i64 %start1.sroa.0.0.i615030, 1, !dbg !2007
  %58 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i1767.sroa.0.0.copyload, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !2015
  %59 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i1767.sroa.0.0.copyload, <8 x float> zeroinitializer, i8 0), !dbg !2021
  %60 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i1774.sroa.0.0.copyload, <8 x float> zeroinitializer, i8 0), !dbg !2027
  %61 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i1823.sroa.0.0.copyload, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !2033
  %62 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i1823.sroa.0.0.copyload, <8 x float> zeroinitializer, i8 0), !dbg !2039
  %63 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i1830.sroa.0.0.copyload, <8 x float> zeroinitializer, i8 0), !dbg !2045
  %slot.i64 = shl nuw nsw i64 %start1.sroa.0.0.i615030, 3, !dbg !2051
  %exitcond = icmp eq i64 %start1.sroa.0.0.i615030, %42, !dbg !2052
  br i1 %exitcond, label %bb43.i143, label %bb44.i66, !dbg !2052, !prof !161

bb44.i66:                                         ; preds = %bb41.i63
  %_120.i68 = getelementptr inbounds nuw float, ptr %left.0, i64 %slot.i64, !dbg !2059
  %exitcond5941.not = icmp eq i64 %start1.sroa.0.0.i615030, %39, !dbg !2064
  br i1 %exitcond5941.not, label %bb2.i1949, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1952, !dbg !2064, !prof !161

bb2.i1949:                                        ; preds = %bb44.i66
  %64 = and i64 %left.1, 7, !dbg !1780
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %64, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !2072, !noalias !2073
  unreachable, !dbg !2072

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1952: ; preds = %bb44.i66
  %lanes.i1945.sroa.0.0.copyload = load <8 x float>, ptr %_120.i68, align 4, !dbg !2077, !alias.scope !2081, !noalias !2085
  %exitcond5942 = icmp eq i64 %start1.sroa.0.0.i615030, %44, !dbg !2087
  br i1 %exitcond5942, label %bb45.i142, label %bb46.i70, !dbg !2087, !prof !161

bb43.i143:                                        ; preds = %bb41.i63
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %slot.i64, i64 noundef range(i64 8, 34359738361) %left.1, i64 noundef range(i64 8, 34359738361) %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c46c6dde8c9e0eb71c8d3a58ebc90caa) #23, !dbg !2092, !noalias !1791
  unreachable, !dbg !2092

bb46.i70:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1952
  %_128.i72 = getelementptr inbounds nuw float, ptr %right.0, i64 %slot.i64, !dbg !2093
  %exitcond5943.not = icmp eq i64 %start1.sroa.0.0.i615030, %40, !dbg !2098
  br i1 %exitcond5943.not, label %bb2.i1940, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1943, !dbg !2098, !prof !161

bb2.i1940:                                        ; preds = %bb46.i70
  %65 = and i64 %right.1, 7, !dbg !1780
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %65, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !2103, !noalias !2104
  unreachable, !dbg !2103

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1943: ; preds = %bb46.i70
  %lanes.i1936.sroa.0.0.copyload = load <8 x float>, ptr %_128.i72, align 4, !dbg !2108, !alias.scope !2112, !noalias !2116
  switch i64 %.sroa.0.0.copyload, label %default.unreachable.i.i141 [
    i64 0, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i87
    i64 1, label %bb3.i.i140
    i64 2, label %bb2.i.i74
  ], !dbg !2118

default.unreachable.i.i141:                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1943
  unreachable

bb3.i.i140:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1943
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i87, !dbg !2122

bb2.i.i74:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1943
  tail call void @llvm.assume(i1 %22)
  %_25.i.i78 = icmp ugt i64 %slot.i64, %.sroa.5.0.copyload, !dbg !2123
  br i1 %_25.i.i78, label %bb17.i.i139, label %bb18.i.i79, !dbg !2123, !prof !161

bb18.i.i79:                                       ; preds = %bb2.i.i74
  tail call void @llvm.assume(i1 %23)
  %_28.i.i81 = sub nuw i64 %.sroa.5.0.copyload, %slot.i64, !dbg !2129
  %_8.i1930 = icmp samesign ugt i64 %_28.i.i81, 7, !dbg !2130
  br i1 %_8.i1930, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1934, label %bb2.i1931, !dbg !2130, !prof !2135

bb2.i1931:                                        ; preds = %bb18.i.i79
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_28.i.i81, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !2136, !noalias !2137
  unreachable, !dbg !2136

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1934: ; preds = %bb18.i.i79
  %_32.i.i82 = getelementptr inbounds nuw float, ptr %.sroa.4.0.copyload, i64 %slot.i64, !dbg !2146
  %lanes.i1927.sroa.0.0.copyload = load <8 x float>, ptr %_32.i.i82, align 4, !dbg !2151, !alias.scope !2155, !noalias !2159
  %_33.i.i83 = icmp ugt i64 %slot.i64, %.sroa.7.0.copyload, !dbg !2161
  br i1 %_33.i.i83, label %bb19.i.i138, label %bb20.i.i84, !dbg !2161, !prof !161

bb17.i.i139:                                      ; preds = %bb2.i.i74
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 34359738361) %slot.i64, i64 noundef %.sroa.5.0.copyload, i64 noundef %.sroa.5.0.copyload, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7fe5a91ec575deda110c046c84630100) #23, !dbg !2165, !noalias !2166
  unreachable, !dbg !2165

bb20.i.i84:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1934
  %_36.i.i85 = sub nuw i64 %.sroa.7.0.copyload, %slot.i64, !dbg !2168
  %_8.i1921 = icmp samesign ugt i64 %_36.i.i85, 7, !dbg !2169
  br i1 %_8.i1921, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1925, label %bb2.i1922, !dbg !2169, !prof !2135

bb2.i1922:                                        ; preds = %bb20.i.i84
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_36.i.i85, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !2174, !noalias !2175
  unreachable, !dbg !2174

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1925: ; preds = %bb20.i.i84
  %_40.i.i86 = getelementptr inbounds nuw float, ptr %.sroa.6.0.copyload, i64 %slot.i64, !dbg !2179
  %lanes.i1918.sroa.0.0.copyload = load <8 x float>, ptr %_40.i.i86, align 4, !dbg !2184, !alias.scope !2188, !noalias !2192
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i87, !dbg !2194

bb19.i.i138:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1934
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 34359738361) %slot.i64, i64 noundef %.sroa.7.0.copyload, i64 noundef %.sroa.7.0.copyload, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f2cba1f89421a726339097d2eba48536) #23, !dbg !2195, !noalias !2166
  unreachable, !dbg !2195

_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i87: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1925, %bb3.i.i140, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1943
  %.sroa.02890.0 = phi <8 x float> [ %lanes.i1936.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1943 ], [ zeroinitializer, %bb3.i.i140 ], [ %lanes.i1918.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1925 ], !dbg !2196
  %.sroa.02887.0 = phi <8 x float> [ %lanes.i1945.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1943 ], [ zeroinitializer, %bb3.i.i140 ], [ %lanes.i1927.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1925 ], !dbg !2196
  %66 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %.sroa.02887.0), !dbg !2197
  %67 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %.sroa.02890.0), !dbg !2204
  %68 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %66, <8 x float> %67), !dbg !2211
  %69 = fmul <8 x float> %66, splat (float 5.000000e-01), !dbg !2221
  %70 = fmul <8 x float> %67, splat (float 5.000000e-01), !dbg !2232
  %71 = fadd <8 x float> %69, %70, !dbg !2237
  %72 = select <8 x i1> %24, <8 x float> %71, <8 x float> %68, !dbg !2247
  %73 = select <8 x i1> %26, <8 x float> %72, <8 x float> %66, !dbg !2253
  %74 = select <8 x i1> %26, <8 x float> %72, <8 x float> %67, !dbg !2259
  %_39.i88 = load i32, ptr %27, align 32, !dbg !2264, !alias.scope !1687, !noalias !1696, !noundef !11
  %write.i89 = zext i32 %_39.i88 to i64, !dbg !2264
  %75 = add nuw nsw i64 %write.i89, 1, !dbg !2266
  %_41.i90 = icmp eq i64 %75, %ring_length.i41, !dbg !2268
  %spec.store.select.i91 = select i1 %_41.i90, i64 0, i64 %75, !dbg !2268
  %_189.1.i92 = load i64, ptr %29, align 8, !dbg !2270, !alias.scope !1687, !noalias !1696, !noundef !11
  %_45.i93 = shl nuw nsw i64 %write.i89, 3, !dbg !2272
  %_129.i94 = icmp ugt i64 %_45.i93, %_189.1.i92, !dbg !2273
  br i1 %_129.i94, label %bb47.i137, label %bb48.i95, !dbg !2273, !prof !161

bb45.i142:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1952
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %slot.i64, i64 noundef range(i64 8, 34359738361) %right.1, i64 noundef range(i64 8, 34359738361) %right.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9b27971f08fa00763b6dc53a9f4dbd4c) #23, !dbg !2279, !noalias !1791
  unreachable, !dbg !2279

bb48.i95:                                         ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i87
  %_132.i97 = sub nuw i64 %_189.1.i92, %_45.i93, !dbg !2280
  %_8.i2546 = icmp samesign ugt i64 %_132.i97, 7, !dbg !2281
  br i1 %_8.i2546, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2549, label %bb2.i2547, !dbg !2281, !prof !2135

bb2.i2547:                                        ; preds = %bb48.i95
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_132.i97, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !2290, !noalias !2291
  unreachable, !dbg !2290

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2549: ; preds = %bb48.i95
  %_189.0.i96 = load ptr, ptr %28, align 32, !dbg !2270, !alias.scope !1687, !noalias !1696, !nonnull !11, !noundef !11
  %_136.i98 = getelementptr inbounds nuw float, ptr %_189.0.i96, i64 %_45.i93, !dbg !2295
  store <8 x float> %lanes.i1945.sroa.0.0.copyload, ptr %_136.i98, align 4, !dbg !2300, !alias.scope !2305, !noalias !2309
  %_190.1.i99 = load i64, ptr %30, align 8, !dbg !2311, !alias.scope !1687, !noalias !1696, !noundef !11
  %_137.i100 = icmp ugt i64 %_45.i93, %_190.1.i99, !dbg !2312
  br i1 %_137.i100, label %bb49.i136, label %bb50.i101, !dbg !2312, !prof !161

bb47.i137:                                        ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i87
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_45.i93, i64 noundef %_189.1.i92, i64 noundef %_189.1.i92, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bb9110d0bc8cedfe643d9bd892c8c620) #23, !dbg !2316, !noalias !1791
  unreachable, !dbg !2316

bb50.i101:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2549
  %_140.i103 = sub nuw i64 %_190.1.i99, %_45.i93, !dbg !2317
  %_8.i2541 = icmp samesign ugt i64 %_140.i103, 7, !dbg !2318
  br i1 %_8.i2541, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2544, label %bb2.i2542, !dbg !2318, !prof !2135

bb2.i2542:                                        ; preds = %bb50.i101
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_140.i103, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !2323, !noalias !2324
  unreachable, !dbg !2323

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2544: ; preds = %bb50.i101
  %_190.0.i102 = load ptr, ptr %31, align 16, !dbg !2311, !alias.scope !1687, !noalias !1696, !nonnull !11, !noundef !11
  %_144.i104 = getelementptr inbounds nuw float, ptr %_190.0.i102, i64 %_45.i93, !dbg !2328
  store <8 x float> %73, ptr %_144.i104, align 4, !dbg !2333, !alias.scope !2337, !noalias !2341
  %_191.1.i105 = load i64, ptr %33, align 8, !dbg !2343, !alias.scope !1690, !noalias !1775, !noundef !11
  %_145.i106 = icmp ugt i64 %_45.i93, %_191.1.i105, !dbg !2344
  br i1 %_145.i106, label %bb51.i135, label %bb52.i107, !dbg !2344, !prof !161

bb49.i136:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2549
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_45.i93, i64 noundef %_190.1.i99, i64 noundef %_190.1.i99, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_da5a912ec6b29b38396e4eb206c99989) #23, !dbg !2348, !noalias !1791
  unreachable, !dbg !2348

bb52.i107:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2544
  %_148.i109 = sub nuw i64 %_191.1.i105, %_45.i93, !dbg !2349
  %_8.i2536 = icmp samesign ugt i64 %_148.i109, 7, !dbg !2350
  br i1 %_8.i2536, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2539, label %bb2.i2537, !dbg !2350, !prof !2135

bb2.i2537:                                        ; preds = %bb52.i107
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_148.i109, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !2355, !noalias !2356
  unreachable, !dbg !2355

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2539: ; preds = %bb52.i107
  %_191.0.i108 = load ptr, ptr %32, align 32, !dbg !2343, !alias.scope !1690, !noalias !1775, !nonnull !11, !noundef !11
  %_152.i110 = getelementptr inbounds nuw float, ptr %_191.0.i108, i64 %_45.i93, !dbg !2360
  store <8 x float> %lanes.i1936.sroa.0.0.copyload, ptr %_152.i110, align 4, !dbg !2365, !alias.scope !2369, !noalias !2373
  %_192.1.i111 = load i64, ptr %34, align 8, !dbg !2375, !alias.scope !1690, !noalias !1775, !noundef !11
  %_153.i112 = icmp ugt i64 %_45.i93, %_192.1.i111, !dbg !2376
  br i1 %_153.i112, label %bb53.i134, label %bb54.i113, !dbg !2376, !prof !161

bb51.i135:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2544
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_45.i93, i64 noundef %_191.1.i105, i64 noundef %_191.1.i105, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b46755d5101951cde0ca1c710062f433) #23, !dbg !2380, !noalias !1791
  unreachable, !dbg !2380

bb54.i113:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2539
  %_156.i115 = sub nuw i64 %_192.1.i111, %_45.i93, !dbg !2381
  %_8.i2531 = icmp samesign ugt i64 %_156.i115, 7, !dbg !2382
  br i1 %_8.i2531, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2534, label %bb2.i2532, !dbg !2382, !prof !2135

bb2.i2532:                                        ; preds = %bb54.i113
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_156.i115, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !2387, !noalias !2388
  unreachable, !dbg !2387

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2534: ; preds = %bb54.i113
  %_192.0.i114 = load ptr, ptr %35, align 16, !dbg !2375, !alias.scope !1690, !noalias !1775, !nonnull !11, !noundef !11
  %_160.i116 = getelementptr inbounds nuw float, ptr %_192.0.i114, i64 %_45.i93, !dbg !2392
  store <8 x float> %74, ptr %_160.i116, align 4, !dbg !2397, !alias.scope !2401, !noalias !2405
  %_193.1.i117 = load i64, ptr %29, align 8, !dbg !2407, !alias.scope !1687, !noalias !1696, !noundef !11
  %_57.i118 = shl nuw nsw i64 %spec.store.select.i91, 3, !dbg !2408
  %_161.i119 = icmp ugt i64 %_57.i118, %_193.1.i117, !dbg !2409
  br i1 %_161.i119, label %bb55.i133, label %bb56.i120, !dbg !2409, !prof !161

bb53.i134:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2539
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_45.i93, i64 noundef %_192.1.i111, i64 noundef %_192.1.i111, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2fb73b04dc26e2fd60c7be20365496fe) #23, !dbg !2413, !noalias !1791
  unreachable, !dbg !2413

bb56.i120:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2534
  %_164.i122 = sub nuw i64 %_193.1.i117, %_57.i118, !dbg !2414
  %_8.i1912 = icmp samesign ugt i64 %_164.i122, 7, !dbg !2415
  br i1 %_8.i1912, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1916, label %bb2.i1913, !dbg !2415, !prof !2135

bb2.i1913:                                        ; preds = %bb56.i120
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_164.i122, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !2420, !noalias !2421
  unreachable, !dbg !2420

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1916: ; preds = %bb56.i120
  %_193.0.i121 = load ptr, ptr %28, align 32, !dbg !2407, !alias.scope !1687, !noalias !1696, !nonnull !11, !noundef !11
  %_168.i123 = getelementptr inbounds nuw float, ptr %_193.0.i121, i64 %_57.i118, !dbg !2425
  %lanes.i1909.sroa.0.0.copyload = load <8 x float>, ptr %_168.i123, align 4, !dbg !2430, !alias.scope !2434, !noalias !2438
  %_194.1.i124 = load i64, ptr %33, align 8, !dbg !2440, !alias.scope !1690, !noalias !1775, !noundef !11
  %_169.i125 = icmp ugt i64 %_57.i118, %_194.1.i124, !dbg !2442
  br i1 %_169.i125, label %bb57.i132, label %bb58.i126, !dbg !2442, !prof !161

bb55.i133:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2534
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_57.i118, i64 noundef %_193.1.i117, i64 noundef %_193.1.i117, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_322bbb2a85bd2347c4d3c1d32e8c0bc7) #23, !dbg !2446, !noalias !1791
  unreachable, !dbg !2446

bb58.i126:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1916
  %_172.i128 = sub nuw i64 %_194.1.i124, %_57.i118, !dbg !2447
  %_8.i1903 = icmp samesign ugt i64 %_172.i128, 7, !dbg !2448
  br i1 %_8.i1903, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1907, label %bb2.i1904, !dbg !2448, !prof !2135

bb2.i1904:                                        ; preds = %bb58.i126
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_172.i128, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !2453, !noalias !2454
  unreachable, !dbg !2453

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1907: ; preds = %bb58.i126
  %_194.0.i127 = load ptr, ptr %32, align 32, !dbg !2440, !alias.scope !1690, !noalias !1775, !nonnull !11, !noundef !11
  %_176.i129 = getelementptr inbounds nuw float, ptr %_194.0.i127, i64 %_57.i118, !dbg !2458
  %lanes.i1900.sroa.0.0.copyload = load <8 x float>, ptr %_176.i129, align 4, !dbg !2463, !alias.scope !2467, !noalias !2471
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2473), !dbg !2476
  %_6.i193 = load i32, ptr %1, align 4, !dbg !2478, !alias.scope !2473, !noalias !2482, !noundef !11
  %ring_length.i194 = zext i32 %_6.i193 to i64, !dbg !2478
  br i1 %or.cond7206.not, label %bb7.i213.preheader, label %bb1.i195, !dbg !2485

bb7.i213.preheader:                               ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1907
  %_56.1.i227 = load i64, ptr %30, align 8
  %_56.0.i231 = load ptr, ptr %31, align 16, !nonnull !11
  %_25.i219 = load i32, ptr %6, align 4, !dbg !2488, !alias.scope !2473, !noalias !2482, !noundef !11
  %delay1.i220 = zext i32 %_25.i219 to i64, !dbg !2488
  %_28.not.i221 = icmp ult i32 %_39.i88, %_25.i219, !dbg !2491
  %_29.i222 = select i1 %_28.not.i221, i64 %ring_length.i194, i64 0, !dbg !2491
  %write.pn7.i223 = sub nsw i64 %write.i89, %delay1.i220, !dbg !2491
  %tap.sroa.0.0.i224 = add nsw i64 %write.pn7.i223, %_29.i222, !dbg !2493
  %_32.i225 = shl nsw i64 %tap.sroa.0.0.i224, 3, !dbg !2494
  %_34.i228 = icmp ult i64 %_32.i225, %_56.1.i227, !dbg !2496
  br i1 %_34.i228, label %bb16.i230, label %panic2.i229, !dbg !2496

bb1.i195:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1907
  %_10.not.i196 = icmp ult i32 %_39.i88, %_91.i46, !dbg !2497
  %_11.i197 = select i1 %_10.not.i196, i64 %ring_length.i194, i64 0, !dbg !2497
  %write.pn.i198 = sub nsw i64 %write.i89, %_90.i47, !dbg !2497
  %row.sroa.0.0.i199 = add nsw i64 %write.pn.i198, %_11.i197, !dbg !2498
  %_55.1.i200 = load i64, ptr %30, align 8, !dbg !2499, !alias.scope !2473, !noalias !2482, !noundef !11
  %_13.i201 = shl nsw i64 %row.sroa.0.0.i199, 3, !dbg !2501
  %_40.i202 = icmp ugt i64 %_13.i201, %_55.1.i200, !dbg !2502
  br i1 %_40.i202, label %bb19.i207, label %bb20.i203, !dbg !2502, !prof !161

bb20.i203:                                        ; preds = %bb1.i195
  %_43.i205 = sub nuw i64 %_55.1.i200, %_13.i201, !dbg !2507
  %_8.i1878 = icmp samesign ugt i64 %_43.i205, 7, !dbg !2508
  br i1 %_8.i1878, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1882, label %bb2.i1879, !dbg !2508, !prof !2135

bb2.i1879:                                        ; preds = %bb20.i203
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_43.i205, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !2513, !noalias !2514
  unreachable, !dbg !2513

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1882: ; preds = %bb20.i203
  %_55.0.i204 = load ptr, ptr %31, align 16, !dbg !2499, !alias.scope !2473, !noalias !2482, !nonnull !11, !noundef !11
  %_47.i206 = getelementptr inbounds nuw float, ptr %_55.0.i204, i64 %_13.i201, !dbg !2518
  %lanes.i1875.sroa.0.0.copyload = load <8 x float>, ptr %_47.i206, align 4, !dbg !2523, !alias.scope !2527, !noalias !2531
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit234, !dbg !2533

bb19.i207:                                        ; preds = %bb1.i195
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_13.i201, i64 noundef %_55.1.i200, i64 noundef %_55.1.i200, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_942b76445104c042a726096e85fe3fcb) #23, !dbg !2534, !noalias !2535
  unreachable, !dbg !2534

bb11.i233:                                        ; preds = %bb12.i218.7
  %76 = getelementptr inbounds nuw float, ptr %_56.0.i231, i64 %_31.i226.6, !dbg !2496
  %_30.i232.6 = load float, ptr %76, align 4, !dbg !2496, !noalias !2536, !noundef !11
  %77 = getelementptr inbounds nuw float, ptr %_56.0.i231, i64 %_31.i226.7, !dbg !2496
  %_30.i232.7 = load float, ptr %77, align 4, !dbg !2496, !noalias !2536, !noundef !11
  %lanes.i1868.sroa.0.0.vec.insert = insertelement <8 x float> poison, float %_30.i232, i64 0, !dbg !2537
  %lanes.i1868.sroa.0.4.vec.insert = insertelement <8 x float> %lanes.i1868.sroa.0.0.vec.insert, float %_30.i232.1, i64 1, !dbg !2537
  %lanes.i1868.sroa.0.8.vec.insert = insertelement <8 x float> %lanes.i1868.sroa.0.4.vec.insert, float %_30.i232.2, i64 2, !dbg !2537
  %lanes.i1868.sroa.0.12.vec.insert = insertelement <8 x float> %lanes.i1868.sroa.0.8.vec.insert, float %_30.i232.3, i64 3, !dbg !2537
  %lanes.i1868.sroa.0.16.vec.insert = insertelement <8 x float> %lanes.i1868.sroa.0.12.vec.insert, float %_30.i232.4, i64 4, !dbg !2537
  %lanes.i1868.sroa.0.20.vec.insert = insertelement <8 x float> %lanes.i1868.sroa.0.16.vec.insert, float %_30.i232.5, i64 5, !dbg !2537
  %lanes.i1868.sroa.0.24.vec.insert = insertelement <8 x float> %lanes.i1868.sroa.0.20.vec.insert, float %_30.i232.6, i64 6, !dbg !2537
  %lanes.i1868.sroa.0.28.vec.insert = insertelement <8 x float> %lanes.i1868.sroa.0.24.vec.insert, float %_30.i232.7, i64 7, !dbg !2537
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit234, !dbg !2533

bb16.i230:                                        ; preds = %bb7.i213.preheader
  %78 = getelementptr inbounds nuw float, ptr %_56.0.i231, i64 %_32.i225, !dbg !2496
  %_30.i232 = load float, ptr %78, align 4, !dbg !2496, !noalias !2536, !noundef !11
  %_25.i219.1 = load i32, ptr %7, align 4, !dbg !2488, !alias.scope !2473, !noalias !2482, !noundef !11
  %delay1.i220.1 = zext i32 %_25.i219.1 to i64, !dbg !2488
  %_28.not.i221.1 = icmp ult i32 %_39.i88, %_25.i219.1, !dbg !2491
  %_29.i222.1 = select i1 %_28.not.i221.1, i64 %ring_length.i194, i64 0, !dbg !2491
  %write.pn7.i223.1 = sub nsw i64 %write.i89, %delay1.i220.1, !dbg !2491
  %tap.sroa.0.0.i224.1 = add nsw i64 %write.pn7.i223.1, %_29.i222.1, !dbg !2493
  %_32.i225.1 = shl nsw i64 %tap.sroa.0.0.i224.1, 3, !dbg !2494
  %_31.i226.1 = or disjoint i64 %_32.i225.1, 1, !dbg !2494
  %_34.i228.1 = icmp ult i64 %_31.i226.1, %_56.1.i227, !dbg !2496
  br i1 %_34.i228.1, label %bb16.i230.1, label %panic2.i229, !dbg !2496

bb16.i230.1:                                      ; preds = %bb16.i230
  %79 = getelementptr inbounds nuw float, ptr %_56.0.i231, i64 %_31.i226.1, !dbg !2496
  %_30.i232.1 = load float, ptr %79, align 4, !dbg !2496, !noalias !2536, !noundef !11
  %_25.i219.2 = load i32, ptr %45, align 4, !dbg !2488, !alias.scope !2473, !noalias !2482, !noundef !11
  %delay1.i220.2 = zext i32 %_25.i219.2 to i64, !dbg !2488
  %_28.not.i221.2 = icmp ult i32 %_39.i88, %_25.i219.2, !dbg !2491
  %_29.i222.2 = select i1 %_28.not.i221.2, i64 %ring_length.i194, i64 0, !dbg !2491
  %write.pn7.i223.2 = sub nsw i64 %write.i89, %delay1.i220.2, !dbg !2491
  %tap.sroa.0.0.i224.2 = add nsw i64 %write.pn7.i223.2, %_29.i222.2, !dbg !2493
  %_32.i225.2 = shl nsw i64 %tap.sroa.0.0.i224.2, 3, !dbg !2494
  %_31.i226.2 = or disjoint i64 %_32.i225.2, 2, !dbg !2494
  %_34.i228.2 = icmp ult i64 %_31.i226.2, %_56.1.i227, !dbg !2496
  br i1 %_34.i228.2, label %bb16.i230.2, label %panic2.i229, !dbg !2496

bb16.i230.2:                                      ; preds = %bb16.i230.1
  %80 = getelementptr inbounds nuw float, ptr %_56.0.i231, i64 %_31.i226.2, !dbg !2496
  %_30.i232.2 = load float, ptr %80, align 4, !dbg !2496, !noalias !2536, !noundef !11
  %_25.i219.3 = load i32, ptr %46, align 4, !dbg !2488, !alias.scope !2473, !noalias !2482, !noundef !11
  %delay1.i220.3 = zext i32 %_25.i219.3 to i64, !dbg !2488
  %_28.not.i221.3 = icmp ult i32 %_39.i88, %_25.i219.3, !dbg !2491
  %_29.i222.3 = select i1 %_28.not.i221.3, i64 %ring_length.i194, i64 0, !dbg !2491
  %write.pn7.i223.3 = sub nsw i64 %write.i89, %delay1.i220.3, !dbg !2491
  %tap.sroa.0.0.i224.3 = add nsw i64 %write.pn7.i223.3, %_29.i222.3, !dbg !2493
  %_32.i225.3 = shl nsw i64 %tap.sroa.0.0.i224.3, 3, !dbg !2494
  %_31.i226.3 = or disjoint i64 %_32.i225.3, 3, !dbg !2494
  %_34.i228.3 = icmp ult i64 %_31.i226.3, %_56.1.i227, !dbg !2496
  br i1 %_34.i228.3, label %bb16.i230.3, label %panic2.i229, !dbg !2496

bb16.i230.3:                                      ; preds = %bb16.i230.2
  %81 = getelementptr inbounds nuw float, ptr %_56.0.i231, i64 %_31.i226.3, !dbg !2496
  %_30.i232.3 = load float, ptr %81, align 4, !dbg !2496, !noalias !2536, !noundef !11
  %_25.i219.4 = load i32, ptr %47, align 4, !dbg !2488, !alias.scope !2473, !noalias !2482, !noundef !11
  %delay1.i220.4 = zext i32 %_25.i219.4 to i64, !dbg !2488
  %_28.not.i221.4 = icmp ult i32 %_39.i88, %_25.i219.4, !dbg !2491
  %_29.i222.4 = select i1 %_28.not.i221.4, i64 %ring_length.i194, i64 0, !dbg !2491
  %write.pn7.i223.4 = sub nsw i64 %write.i89, %delay1.i220.4, !dbg !2491
  %tap.sroa.0.0.i224.4 = add nsw i64 %write.pn7.i223.4, %_29.i222.4, !dbg !2493
  %_32.i225.4 = shl nsw i64 %tap.sroa.0.0.i224.4, 3, !dbg !2494
  %_31.i226.4 = or disjoint i64 %_32.i225.4, 4, !dbg !2494
  %_34.i228.4 = icmp ult i64 %_31.i226.4, %_56.1.i227, !dbg !2496
  br i1 %_34.i228.4, label %bb16.i230.4, label %panic2.i229, !dbg !2496

bb16.i230.4:                                      ; preds = %bb16.i230.3
  %82 = getelementptr inbounds nuw float, ptr %_56.0.i231, i64 %_31.i226.4, !dbg !2496
  %_30.i232.4 = load float, ptr %82, align 4, !dbg !2496, !noalias !2536, !noundef !11
  %_25.i219.5 = load i32, ptr %48, align 4, !dbg !2488, !alias.scope !2473, !noalias !2482, !noundef !11
  %delay1.i220.5 = zext i32 %_25.i219.5 to i64, !dbg !2488
  %_28.not.i221.5 = icmp ult i32 %_39.i88, %_25.i219.5, !dbg !2491
  %_29.i222.5 = select i1 %_28.not.i221.5, i64 %ring_length.i194, i64 0, !dbg !2491
  %write.pn7.i223.5 = sub nsw i64 %write.i89, %delay1.i220.5, !dbg !2491
  %tap.sroa.0.0.i224.5 = add nsw i64 %write.pn7.i223.5, %_29.i222.5, !dbg !2493
  %_32.i225.5 = shl nsw i64 %tap.sroa.0.0.i224.5, 3, !dbg !2494
  %_31.i226.5 = or disjoint i64 %_32.i225.5, 5, !dbg !2494
  %_34.i228.5 = icmp ult i64 %_31.i226.5, %_56.1.i227, !dbg !2496
  br i1 %_34.i228.5, label %bb16.i230.5, label %panic2.i229, !dbg !2496

bb16.i230.5:                                      ; preds = %bb16.i230.4
  %83 = getelementptr inbounds nuw float, ptr %_56.0.i231, i64 %_31.i226.5, !dbg !2496
  %_30.i232.5 = load float, ptr %83, align 4, !dbg !2496, !noalias !2536, !noundef !11
  %_25.i219.6 = load i32, ptr %49, align 4, !dbg !2488, !alias.scope !2473, !noalias !2482, !noundef !11
  %delay1.i220.6 = zext i32 %_25.i219.6 to i64, !dbg !2488
  %_28.not.i221.6 = icmp ult i32 %_39.i88, %_25.i219.6, !dbg !2491
  %_29.i222.6 = select i1 %_28.not.i221.6, i64 %ring_length.i194, i64 0, !dbg !2491
  %write.pn7.i223.6 = sub nsw i64 %write.i89, %delay1.i220.6, !dbg !2491
  %tap.sroa.0.0.i224.6 = add nsw i64 %write.pn7.i223.6, %_29.i222.6, !dbg !2493
  %_32.i225.6 = shl nsw i64 %tap.sroa.0.0.i224.6, 3, !dbg !2494
  %_31.i226.6 = or disjoint i64 %_32.i225.6, 6, !dbg !2494
  %_34.i228.6 = icmp ult i64 %_31.i226.6, %_56.1.i227, !dbg !2496
  br i1 %_34.i228.6, label %bb12.i218.7, label %panic2.i229, !dbg !2496

bb12.i218.7:                                      ; preds = %bb16.i230.5
  %_25.i219.7 = load i32, ptr %50, align 4, !dbg !2488, !alias.scope !2473, !noalias !2482, !noundef !11
  %delay1.i220.7 = zext i32 %_25.i219.7 to i64, !dbg !2488
  %_28.not.i221.7 = icmp ult i32 %_39.i88, %_25.i219.7, !dbg !2491
  %_29.i222.7 = select i1 %_28.not.i221.7, i64 %ring_length.i194, i64 0, !dbg !2491
  %write.pn7.i223.7 = sub nsw i64 %write.i89, %delay1.i220.7, !dbg !2491
  %tap.sroa.0.0.i224.7 = add nsw i64 %write.pn7.i223.7, %_29.i222.7, !dbg !2493
  %_32.i225.7 = shl nsw i64 %tap.sroa.0.0.i224.7, 3, !dbg !2494
  %_31.i226.7 = or disjoint i64 %_32.i225.7, 7, !dbg !2494
  %_34.i228.7 = icmp ult i64 %_31.i226.7, %_56.1.i227, !dbg !2496
  br i1 %_34.i228.7, label %bb11.i233, label %panic2.i229, !dbg !2496

panic2.i229:                                      ; preds = %bb12.i218.7, %bb16.i230.5, %bb16.i230.4, %bb16.i230.3, %bb16.i230.2, %bb16.i230.1, %bb16.i230, %bb7.i213.preheader
  %_31.i226.lcssa = phi i64 [ %_32.i225, %bb7.i213.preheader ], [ %_31.i226.1, %bb16.i230 ], [ %_31.i226.2, %bb16.i230.1 ], [ %_31.i226.3, %bb16.i230.2 ], [ %_31.i226.4, %bb16.i230.3 ], [ %_31.i226.5, %bb16.i230.4 ], [ %_31.i226.6, %bb16.i230.5 ], [ %_31.i226.7, %bb12.i218.7 ], !dbg !2494
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_31.i226.lcssa, i64 noundef %_56.1.i227, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c4f76d0874ad32f224dcaa5d3453a9e1) #23, !dbg !2496, !noalias !2536
  unreachable, !dbg !2496

_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit234: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1882, %bb11.i233
  %detected_left.i28.sroa.0.0 = phi <8 x float> [ %lanes.i1868.sroa.0.28.vec.insert, %bb11.i233 ], [ %lanes.i1875.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1882 ], !dbg !2542
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2546), !dbg !2549
  %_6.i = load i32, ptr %36, align 4, !dbg !2551, !alias.scope !2546, !noalias !2553, !noundef !11
  %ring_length.i189 = zext i32 %_6.i to i64, !dbg !2551
  br i1 %or.cond7212.not, label %bb7.i.preheader, label %bb1.i, !dbg !2556

bb7.i.preheader:                                  ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit234
  %_56.1.i = load i64, ptr %34, align 8
  %_56.0.i = load ptr, ptr %35, align 16, !nonnull !11
  %_25.i191 = load i32, ptr %14, align 4, !dbg !2557, !alias.scope !2546, !noalias !2553, !noundef !11
  %delay1.i = zext i32 %_25.i191 to i64, !dbg !2557
  %_28.not.i = icmp ult i32 %_39.i88, %_25.i191, !dbg !2558
  %_29.i = select i1 %_28.not.i, i64 %ring_length.i189, i64 0, !dbg !2558
  %write.pn7.i = sub nsw i64 %write.i89, %delay1.i, !dbg !2558
  %tap.sroa.0.0.i = add nsw i64 %write.pn7.i, %_29.i, !dbg !2559
  %_32.i = shl nsw i64 %tap.sroa.0.0.i, 3, !dbg !2560
  %_34.i = icmp ult i64 %_32.i, %_56.1.i, !dbg !2561
  br i1 %_34.i, label %bb16.i, label %panic2.i, !dbg !2561

bb1.i:                                            ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit234
  %_10.not.i = icmp ult i32 %_39.i88, %_101.i53, !dbg !2562
  %_11.i190 = select i1 %_10.not.i, i64 %ring_length.i189, i64 0, !dbg !2562
  %write.pn.i = sub nsw i64 %write.i89, %_100.i54, !dbg !2562
  %row.sroa.0.0.i = add nsw i64 %write.pn.i, %_11.i190, !dbg !2563
  %_55.1.i = load i64, ptr %34, align 8, !dbg !2564, !alias.scope !2546, !noalias !2553, !noundef !11
  %_13.i = shl nsw i64 %row.sroa.0.0.i, 3, !dbg !2565
  %_40.i = icmp ugt i64 %_13.i, %_55.1.i, !dbg !2566
  br i1 %_40.i, label %bb19.i, label %bb20.i, !dbg !2566, !prof !161

bb20.i:                                           ; preds = %bb1.i
  %_43.i = sub nuw i64 %_55.1.i, %_13.i, !dbg !2569
  %_8.i1894 = icmp samesign ugt i64 %_43.i, 7, !dbg !2570
  br i1 %_8.i1894, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1898, label %bb2.i1895, !dbg !2570, !prof !2135

bb2.i1895:                                        ; preds = %bb20.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_43.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !2575, !noalias !2576
  unreachable, !dbg !2575

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1898: ; preds = %bb20.i
  %_55.0.i = load ptr, ptr %35, align 16, !dbg !2564, !alias.scope !2546, !noalias !2553, !nonnull !11, !noundef !11
  %_47.i = getelementptr inbounds nuw float, ptr %_55.0.i, i64 %_13.i, !dbg !2580
  %lanes.i1891.sroa.0.0.copyload = load <8 x float>, ptr %_47.i, align 4, !dbg !2582, !alias.scope !2586, !noalias !2590
  br label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit562, !dbg !2592

bb19.i:                                           ; preds = %bb1.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_13.i, i64 noundef %_55.1.i, i64 noundef %_55.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_942b76445104c042a726096e85fe3fcb) #23, !dbg !2593, !noalias !2594
  unreachable, !dbg !2593

bb11.i:                                           ; preds = %bb12.i.7
  %84 = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_31.i.6, !dbg !2561
  %_30.i.6 = load float, ptr %84, align 4, !dbg !2561, !noalias !2595, !noundef !11
  %85 = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_31.i.7, !dbg !2561
  %_30.i.7 = load float, ptr %85, align 4, !dbg !2561, !noalias !2595, !noundef !11
  %lanes.i1884.sroa.0.0.vec.insert = insertelement <8 x float> poison, float %_30.i, i64 0, !dbg !2596
  %lanes.i1884.sroa.0.4.vec.insert = insertelement <8 x float> %lanes.i1884.sroa.0.0.vec.insert, float %_30.i.1, i64 1, !dbg !2596
  %lanes.i1884.sroa.0.8.vec.insert = insertelement <8 x float> %lanes.i1884.sroa.0.4.vec.insert, float %_30.i.2, i64 2, !dbg !2596
  %lanes.i1884.sroa.0.12.vec.insert = insertelement <8 x float> %lanes.i1884.sroa.0.8.vec.insert, float %_30.i.3, i64 3, !dbg !2596
  %lanes.i1884.sroa.0.16.vec.insert = insertelement <8 x float> %lanes.i1884.sroa.0.12.vec.insert, float %_30.i.4, i64 4, !dbg !2596
  %lanes.i1884.sroa.0.20.vec.insert = insertelement <8 x float> %lanes.i1884.sroa.0.16.vec.insert, float %_30.i.5, i64 5, !dbg !2596
  %lanes.i1884.sroa.0.24.vec.insert = insertelement <8 x float> %lanes.i1884.sroa.0.20.vec.insert, float %_30.i.6, i64 6, !dbg !2596
  %lanes.i1884.sroa.0.28.vec.insert = insertelement <8 x float> %lanes.i1884.sroa.0.24.vec.insert, float %_30.i.7, i64 7, !dbg !2596
  br label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit562, !dbg !2592

bb16.i:                                           ; preds = %bb7.i.preheader
  %86 = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_32.i, !dbg !2561
  %_30.i = load float, ptr %86, align 4, !dbg !2561, !noalias !2595, !noundef !11
  %_25.i191.1 = load i32, ptr %15, align 4, !dbg !2557, !alias.scope !2546, !noalias !2553, !noundef !11
  %delay1.i.1 = zext i32 %_25.i191.1 to i64, !dbg !2557
  %_28.not.i.1 = icmp ult i32 %_39.i88, %_25.i191.1, !dbg !2558
  %_29.i.1 = select i1 %_28.not.i.1, i64 %ring_length.i189, i64 0, !dbg !2558
  %write.pn7.i.1 = sub nsw i64 %write.i89, %delay1.i.1, !dbg !2558
  %tap.sroa.0.0.i.1 = add nsw i64 %write.pn7.i.1, %_29.i.1, !dbg !2559
  %_32.i.1 = shl nsw i64 %tap.sroa.0.0.i.1, 3, !dbg !2560
  %_31.i.1 = or disjoint i64 %_32.i.1, 1, !dbg !2560
  %_34.i.1 = icmp ult i64 %_31.i.1, %_56.1.i, !dbg !2561
  br i1 %_34.i.1, label %bb16.i.1, label %panic2.i, !dbg !2561

bb16.i.1:                                         ; preds = %bb16.i
  %87 = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_31.i.1, !dbg !2561
  %_30.i.1 = load float, ptr %87, align 4, !dbg !2561, !noalias !2595, !noundef !11
  %_25.i191.2 = load i32, ptr %51, align 4, !dbg !2557, !alias.scope !2546, !noalias !2553, !noundef !11
  %delay1.i.2 = zext i32 %_25.i191.2 to i64, !dbg !2557
  %_28.not.i.2 = icmp ult i32 %_39.i88, %_25.i191.2, !dbg !2558
  %_29.i.2 = select i1 %_28.not.i.2, i64 %ring_length.i189, i64 0, !dbg !2558
  %write.pn7.i.2 = sub nsw i64 %write.i89, %delay1.i.2, !dbg !2558
  %tap.sroa.0.0.i.2 = add nsw i64 %write.pn7.i.2, %_29.i.2, !dbg !2559
  %_32.i.2 = shl nsw i64 %tap.sroa.0.0.i.2, 3, !dbg !2560
  %_31.i.2 = or disjoint i64 %_32.i.2, 2, !dbg !2560
  %_34.i.2 = icmp ult i64 %_31.i.2, %_56.1.i, !dbg !2561
  br i1 %_34.i.2, label %bb16.i.2, label %panic2.i, !dbg !2561

bb16.i.2:                                         ; preds = %bb16.i.1
  %88 = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_31.i.2, !dbg !2561
  %_30.i.2 = load float, ptr %88, align 4, !dbg !2561, !noalias !2595, !noundef !11
  %_25.i191.3 = load i32, ptr %52, align 4, !dbg !2557, !alias.scope !2546, !noalias !2553, !noundef !11
  %delay1.i.3 = zext i32 %_25.i191.3 to i64, !dbg !2557
  %_28.not.i.3 = icmp ult i32 %_39.i88, %_25.i191.3, !dbg !2558
  %_29.i.3 = select i1 %_28.not.i.3, i64 %ring_length.i189, i64 0, !dbg !2558
  %write.pn7.i.3 = sub nsw i64 %write.i89, %delay1.i.3, !dbg !2558
  %tap.sroa.0.0.i.3 = add nsw i64 %write.pn7.i.3, %_29.i.3, !dbg !2559
  %_32.i.3 = shl nsw i64 %tap.sroa.0.0.i.3, 3, !dbg !2560
  %_31.i.3 = or disjoint i64 %_32.i.3, 3, !dbg !2560
  %_34.i.3 = icmp ult i64 %_31.i.3, %_56.1.i, !dbg !2561
  br i1 %_34.i.3, label %bb16.i.3, label %panic2.i, !dbg !2561

bb16.i.3:                                         ; preds = %bb16.i.2
  %89 = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_31.i.3, !dbg !2561
  %_30.i.3 = load float, ptr %89, align 4, !dbg !2561, !noalias !2595, !noundef !11
  %_25.i191.4 = load i32, ptr %53, align 4, !dbg !2557, !alias.scope !2546, !noalias !2553, !noundef !11
  %delay1.i.4 = zext i32 %_25.i191.4 to i64, !dbg !2557
  %_28.not.i.4 = icmp ult i32 %_39.i88, %_25.i191.4, !dbg !2558
  %_29.i.4 = select i1 %_28.not.i.4, i64 %ring_length.i189, i64 0, !dbg !2558
  %write.pn7.i.4 = sub nsw i64 %write.i89, %delay1.i.4, !dbg !2558
  %tap.sroa.0.0.i.4 = add nsw i64 %write.pn7.i.4, %_29.i.4, !dbg !2559
  %_32.i.4 = shl nsw i64 %tap.sroa.0.0.i.4, 3, !dbg !2560
  %_31.i.4 = or disjoint i64 %_32.i.4, 4, !dbg !2560
  %_34.i.4 = icmp ult i64 %_31.i.4, %_56.1.i, !dbg !2561
  br i1 %_34.i.4, label %bb16.i.4, label %panic2.i, !dbg !2561

bb16.i.4:                                         ; preds = %bb16.i.3
  %90 = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_31.i.4, !dbg !2561
  %_30.i.4 = load float, ptr %90, align 4, !dbg !2561, !noalias !2595, !noundef !11
  %_25.i191.5 = load i32, ptr %54, align 4, !dbg !2557, !alias.scope !2546, !noalias !2553, !noundef !11
  %delay1.i.5 = zext i32 %_25.i191.5 to i64, !dbg !2557
  %_28.not.i.5 = icmp ult i32 %_39.i88, %_25.i191.5, !dbg !2558
  %_29.i.5 = select i1 %_28.not.i.5, i64 %ring_length.i189, i64 0, !dbg !2558
  %write.pn7.i.5 = sub nsw i64 %write.i89, %delay1.i.5, !dbg !2558
  %tap.sroa.0.0.i.5 = add nsw i64 %write.pn7.i.5, %_29.i.5, !dbg !2559
  %_32.i.5 = shl nsw i64 %tap.sroa.0.0.i.5, 3, !dbg !2560
  %_31.i.5 = or disjoint i64 %_32.i.5, 5, !dbg !2560
  %_34.i.5 = icmp ult i64 %_31.i.5, %_56.1.i, !dbg !2561
  br i1 %_34.i.5, label %bb16.i.5, label %panic2.i, !dbg !2561

bb16.i.5:                                         ; preds = %bb16.i.4
  %91 = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_31.i.5, !dbg !2561
  %_30.i.5 = load float, ptr %91, align 4, !dbg !2561, !noalias !2595, !noundef !11
  %_25.i191.6 = load i32, ptr %55, align 4, !dbg !2557, !alias.scope !2546, !noalias !2553, !noundef !11
  %delay1.i.6 = zext i32 %_25.i191.6 to i64, !dbg !2557
  %_28.not.i.6 = icmp ult i32 %_39.i88, %_25.i191.6, !dbg !2558
  %_29.i.6 = select i1 %_28.not.i.6, i64 %ring_length.i189, i64 0, !dbg !2558
  %write.pn7.i.6 = sub nsw i64 %write.i89, %delay1.i.6, !dbg !2558
  %tap.sroa.0.0.i.6 = add nsw i64 %write.pn7.i.6, %_29.i.6, !dbg !2559
  %_32.i.6 = shl nsw i64 %tap.sroa.0.0.i.6, 3, !dbg !2560
  %_31.i.6 = or disjoint i64 %_32.i.6, 6, !dbg !2560
  %_34.i.6 = icmp ult i64 %_31.i.6, %_56.1.i, !dbg !2561
  br i1 %_34.i.6, label %bb12.i.7, label %panic2.i, !dbg !2561

bb12.i.7:                                         ; preds = %bb16.i.5
  %_25.i191.7 = load i32, ptr %56, align 4, !dbg !2557, !alias.scope !2546, !noalias !2553, !noundef !11
  %delay1.i.7 = zext i32 %_25.i191.7 to i64, !dbg !2557
  %_28.not.i.7 = icmp ult i32 %_39.i88, %_25.i191.7, !dbg !2558
  %_29.i.7 = select i1 %_28.not.i.7, i64 %ring_length.i189, i64 0, !dbg !2558
  %write.pn7.i.7 = sub nsw i64 %write.i89, %delay1.i.7, !dbg !2558
  %tap.sroa.0.0.i.7 = add nsw i64 %write.pn7.i.7, %_29.i.7, !dbg !2559
  %_32.i.7 = shl nsw i64 %tap.sroa.0.0.i.7, 3, !dbg !2560
  %_31.i.7 = or disjoint i64 %_32.i.7, 7, !dbg !2560
  %_34.i.7 = icmp ult i64 %_31.i.7, %_56.1.i, !dbg !2561
  br i1 %_34.i.7, label %bb11.i, label %panic2.i, !dbg !2561

panic2.i:                                         ; preds = %bb12.i.7, %bb16.i.5, %bb16.i.4, %bb16.i.3, %bb16.i.2, %bb16.i.1, %bb16.i, %bb7.i.preheader
  %_31.i.lcssa = phi i64 [ %_32.i, %bb7.i.preheader ], [ %_31.i.1, %bb16.i ], [ %_31.i.2, %bb16.i.1 ], [ %_31.i.3, %bb16.i.2 ], [ %_31.i.4, %bb16.i.3 ], [ %_31.i.5, %bb16.i.4 ], [ %_31.i.6, %bb16.i.5 ], [ %_31.i.7, %bb12.i.7 ], !dbg !2560
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_31.i.lcssa, i64 noundef %_56.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c4f76d0874ad32f224dcaa5d3453a9e1) #23, !dbg !2561, !noalias !2595
  unreachable, !dbg !2561

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit562: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1898, %bb11.i
  %detected_right.i27.sroa.0.0 = phi <8 x float> [ %lanes.i1884.sroa.0.28.vec.insert, %bb11.i ], [ %lanes.i1891.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1898 ], !dbg !2601
  %92 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %detected_left.i28.sroa.0.0, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !2604
  %93 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %92, <8 x float> splat (float 0x3810000000000000)), !dbg !2614
  %94 = bitcast <8 x float> %93 to <4 x i64>, !dbg !2626
  %95 = and <4 x i64> %94, splat (i64 36028792732385279), !dbg !2627
  %96 = or disjoint <4 x i64> %95, splat (i64 4575657222473777152), !dbg !2645
  %97 = bitcast <4 x i64> %96 to <8 x float>, !dbg !2654
  %98 = fadd <8 x float> %97, splat (float -1.000000e+00), !dbg !2655
  %99 = fmul <8 x float> %98, splat (float 0xBF9B17A960000000), !dbg !2666
  %100 = fadd <8 x float> %99, splat (float 0x3FBF9A8440000000), !dbg !2674
  %101 = fmul <8 x float> %98, %100, !dbg !2666
  %102 = fadd <8 x float> %101, splat (float 0xBFD1E3F400000000), !dbg !2674
  %103 = fmul <8 x float> %98, %102, !dbg !2666
  %104 = fadd <8 x float> %103, splat (float 0x3FDD544F20000000), !dbg !2674
  %105 = fmul <8 x float> %98, %104, !dbg !2666
  %106 = fadd <8 x float> %105, splat (float 0xBFE6FC2A60000000), !dbg !2674
  %107 = fmul <8 x float> %98, %106, !dbg !2666
  %108 = fadd <8 x float> %107, splat (float 0x3FF714B2A0000000), !dbg !2674
  %109 = bitcast <8 x float> %93 to <8 x i32>, !dbg !2679
  %_3.i2764 = lshr <8 x i32> %109, splat (i32 23), !dbg !2689
  %110 = or disjoint <8 x i32> %_3.i2764, splat (i32 1258291200), !dbg !2690
  %111 = bitcast <8 x i32> %110 to <8 x float>, !dbg !2696
  %112 = fadd <8 x float> %111, splat (float 0xC160000FE0000000), !dbg !2697
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2703), !dbg !2706
  %_6.i404.sroa.0.0.copyload = load <8 x float>, ptr %_73.i130, align 32, !dbg !2708, !noalias !2711
  %113 = fmul <8 x float> %98, %108, !dbg !2714
  %114 = fadd <8 x float> %112, %113, !dbg !2719
  %115 = fmul <8 x float> %114, splat (float 0x4018151820000000), !dbg !2724
  %116 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %115, <8 x float> splat (float -1.600000e+02)), !dbg !2729
  %117 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %116, <8 x float> splat (float 2.400000e+01)), !dbg !2734
  %118 = fsub <8 x float> %117, %lanes.i1760.sroa.0.0.copyload, !dbg !2743
  %119 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %118, <8 x float> %lanes.i1746.sroa.0.0.copyload, i8 30), !dbg !2751
  %120 = fneg <8 x float> %lanes.i1746.sroa.0.0.copyload, !dbg !2764
  %121 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %118, <8 x float> %120, i8 18), !dbg !2774
  %122 = fadd <8 x float> %lanes.i1746.sroa.0.0.copyload, %118, !dbg !2786
  %123 = fmul <8 x float> %122, %122, !dbg !2793
  %124 = fmul <8 x float> %lanes.i1739.sroa.0.0.copyload, %123, !dbg !2799
  %125 = bitcast <8 x float> %119 to <8 x i32>, !dbg !2804
  %126 = icmp slt <8 x i32> %125, zeroinitializer, !dbg !2809
  %.v = select <8 x i1> %126, <8 x float> %118, <8 x float> %124, !dbg !2809
  %127 = fmul <8 x float> %lanes.i1753.sroa.0.0.copyload, %.v, !dbg !2809
  %128 = bitcast <8 x float> %121 to <8 x i32>, !dbg !2811
  %129 = icmp slt <8 x i32> %128, zeroinitializer, !dbg !2815
  %130 = select <8 x i1> %129, <8 x float> zeroinitializer, <8 x float> %127, !dbg !2815
  %131 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %130, <8 x float> splat (float -1.000000e+02)), !dbg !2817
  %132 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %131, <8 x float> zeroinitializer), !dbg !2822
  %133 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %132, <8 x float> %_6.i404.sroa.0.0.copyload, i8 17), !dbg !2827
  %134 = bitcast <8 x float> %133 to <8 x i32>, !dbg !2833
  %135 = icmp slt <8 x i32> %134, zeroinitializer, !dbg !2837
  %136 = select <8 x i1> %135, <8 x float> %lanes.i1732.sroa.0.0.copyload, <8 x float> %lanes.i1725.sroa.0.0.copyload, !dbg !2837
  %137 = fsub <8 x float> %132, %_6.i404.sroa.0.0.copyload, !dbg !2839
  %138 = fmul <8 x float> %137, %136, !dbg !2849
  %139 = fadd <8 x float> %_6.i404.sroa.0.0.copyload, %138, !dbg !2858
  %140 = bitcast <8 x float> %139 to <8 x i32>, !dbg !2865
  %141 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %139), !dbg !2873
  %142 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %141, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !2875
  %143 = bitcast <8 x float> %142 to <8 x i32>, !dbg !2881
  %144 = xor <8 x i32> %143, splat (i32 -1), !dbg !2893
  %145 = and <8 x i32> %144, %140, !dbg !2895
  store <8 x i32> %145, ptr %_73.i130, align 32, !dbg !2901, !alias.scope !2903, !noalias !2905
  %146 = bitcast <8 x i32> %145 to <8 x float>, !dbg !2906
  %147 = fadd <8 x float> %lanes.i1774.sroa.0.0.copyload, %146, !dbg !2907
  %148 = fmul <8 x float> %147, splat (float 0x3FC542A5A0000000), !dbg !2915
  %149 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %148, <8 x float> splat (float -1.260000e+02)), !dbg !2922
  %150 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %149, <8 x float> splat (float 1.270000e+02)), !dbg !2929
  %151 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %150), !dbg !2934
  %152 = fsub <8 x float> %150, %151, !dbg !2944
  %153 = fmul <8 x float> %152, splat (float 0x3F5E974FA0000000), !dbg !2950
  %154 = fadd <8 x float> %153, splat (float 0x3F82778560000000), !dbg !2958
  %155 = fmul <8 x float> %152, %154, !dbg !2950
  %156 = fadd <8 x float> %155, splat (float 0x3FAC91CE60000000), !dbg !2958
  %157 = fmul <8 x float> %152, %156, !dbg !2950
  %158 = fadd <8 x float> %157, splat (float 0x3FCEBDB560000000), !dbg !2958
  %159 = fmul <8 x float> %152, %158, !dbg !2950
  %160 = fadd <8 x float> %159, splat (float 0x3FE62E4BA0000000), !dbg !2958
  %161 = fmul <8 x float> %152, %160, !dbg !2963
  %162 = fadd <8 x float> %161, splat (float 1.000000e+00), !dbg !2968
  %163 = fadd <8 x float> %151, splat (float 0x4160000FE0000000), !dbg !2973
  %164 = bitcast <8 x float> %163 to <8 x i32>, !dbg !2982
  %_3.i2765 = shl <8 x i32> %164, splat (i32 23), !dbg !2992
  %165 = bitcast <8 x i32> %_3.i2765 to <8 x float>, !dbg !2993
  %166 = fmul <8 x float> %162, %165, !dbg !2995
  %167 = fmul <8 x float> %lanes.i1909.sroa.0.0.copyload, %166, !dbg !2999
  %168 = fsub <8 x float> %167, %lanes.i1909.sroa.0.0.copyload, !dbg !3005
  %169 = fmul <8 x float> %lanes.i1767.sroa.0.0.copyload, %168, !dbg !3016
  %170 = fadd <8 x float> %lanes.i1909.sroa.0.0.copyload, %169, !dbg !3022
  %171 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %146, <8 x float> zeroinitializer, i8 0), !dbg !3026
  %172 = bitcast <8 x float> %171 to <8 x i32>, !dbg !3033
  %173 = bitcast <8 x float> %60 to <8 x i32>, !dbg !3033
  %174 = and <8 x i32> %172, %173, !dbg !3037
  %175 = bitcast <8 x float> %59 to <8 x i32>, !dbg !3039
  %176 = or <8 x i32> %174, %175, !dbg !3047
  %177 = or <8 x i32> %176, %37, !dbg !3047
  %178 = bitcast <8 x float> %58 to <8 x i32>, !dbg !3055
  %179 = icmp slt <8 x i32> %178, zeroinitializer, !dbg !3060
  %180 = select <8 x i1> %179, <8 x float> %167, <8 x float> %170, !dbg !3060
  %181 = icmp slt <8 x i32> %177, zeroinitializer, !dbg !3062
  %182 = select <8 x i1> %181, <8 x float> %lanes.i1909.sroa.0.0.copyload, <8 x float> %180, !dbg !3062
  %183 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %detected_right.i27.sroa.0.0, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !3068
  %184 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %183, <8 x float> splat (float 0x3810000000000000)), !dbg !3076
  %185 = bitcast <8 x float> %184 to <4 x i64>, !dbg !3083
  %186 = and <4 x i64> %185, splat (i64 36028792732385279), !dbg !3084
  %187 = or disjoint <4 x i64> %186, splat (i64 4575657222473777152), !dbg !3089
  %188 = bitcast <4 x i64> %187 to <8 x float>, !dbg !3093
  %189 = fadd <8 x float> %188, splat (float -1.000000e+00), !dbg !3094
  %190 = fmul <8 x float> %189, splat (float 0xBF9B17A960000000), !dbg !3099
  %191 = fadd <8 x float> %190, splat (float 0x3FBF9A8440000000), !dbg !3104
  %192 = fmul <8 x float> %189, %191, !dbg !3099
  %193 = fadd <8 x float> %192, splat (float 0xBFD1E3F400000000), !dbg !3104
  %194 = fmul <8 x float> %189, %193, !dbg !3099
  %195 = fadd <8 x float> %194, splat (float 0x3FDD544F20000000), !dbg !3104
  %196 = fmul <8 x float> %189, %195, !dbg !3099
  %197 = fadd <8 x float> %196, splat (float 0xBFE6FC2A60000000), !dbg !3104
  %198 = fmul <8 x float> %189, %197, !dbg !3099
  %199 = fadd <8 x float> %198, splat (float 0x3FF714B2A0000000), !dbg !3104
  %200 = bitcast <8 x float> %184 to <8 x i32>, !dbg !3109
  %_3.i2766 = lshr <8 x i32> %200, splat (i32 23), !dbg !3113
  %201 = or disjoint <8 x i32> %_3.i2766, splat (i32 1258291200), !dbg !3114
  %202 = bitcast <8 x i32> %201 to <8 x float>, !dbg !3118
  %203 = fadd <8 x float> %202, splat (float 0xC160000FE0000000), !dbg !3119
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3123), !dbg !3126
  %_6.i394.sroa.0.0.copyload = load <8 x float>, ptr %_77.i131, align 32, !dbg !3128, !noalias !3130
  %204 = fmul <8 x float> %189, %199, !dbg !3133
  %205 = fadd <8 x float> %203, %204, !dbg !3138
  %206 = fmul <8 x float> %205, splat (float 0x4018151820000000), !dbg !3143
  %207 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %206, <8 x float> splat (float -1.600000e+02)), !dbg !3148
  %208 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %207, <8 x float> splat (float 2.400000e+01)), !dbg !3153
  %209 = fsub <8 x float> %208, %lanes.i1816.sroa.0.0.copyload, !dbg !3158
  %210 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %209, <8 x float> %lanes.i1802.sroa.0.0.copyload, i8 30), !dbg !3164
  %211 = fneg <8 x float> %lanes.i1802.sroa.0.0.copyload, !dbg !3170
  %212 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %209, <8 x float> %211, i8 18), !dbg !3175
  %213 = fadd <8 x float> %lanes.i1802.sroa.0.0.copyload, %209, !dbg !3181
  %214 = fmul <8 x float> %213, %213, !dbg !3186
  %215 = fmul <8 x float> %lanes.i1795.sroa.0.0.copyload, %214, !dbg !3191
  %216 = bitcast <8 x float> %210 to <8 x i32>, !dbg !3196
  %217 = icmp slt <8 x i32> %216, zeroinitializer, !dbg !3200
  %.v4146 = select <8 x i1> %217, <8 x float> %209, <8 x float> %215, !dbg !3200
  %218 = fmul <8 x float> %lanes.i1809.sroa.0.0.copyload, %.v4146, !dbg !3200
  %219 = bitcast <8 x float> %212 to <8 x i32>, !dbg !3202
  %220 = icmp slt <8 x i32> %219, zeroinitializer, !dbg !3206
  %221 = select <8 x i1> %220, <8 x float> zeroinitializer, <8 x float> %218, !dbg !3206
  %222 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %221, <8 x float> splat (float -1.000000e+02)), !dbg !3208
  %223 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %222, <8 x float> zeroinitializer), !dbg !3213
  %224 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %223, <8 x float> %_6.i394.sroa.0.0.copyload, i8 17), !dbg !3218
  %225 = bitcast <8 x float> %224 to <8 x i32>, !dbg !3224
  %226 = icmp slt <8 x i32> %225, zeroinitializer, !dbg !3228
  %227 = select <8 x i1> %226, <8 x float> %lanes.i1788.sroa.0.0.copyload, <8 x float> %lanes.i1781.sroa.0.0.copyload, !dbg !3228
  %228 = fsub <8 x float> %223, %_6.i394.sroa.0.0.copyload, !dbg !3230
  %229 = fmul <8 x float> %228, %227, !dbg !3236
  %230 = fadd <8 x float> %_6.i394.sroa.0.0.copyload, %229, !dbg !3241
  %231 = bitcast <8 x float> %230 to <8 x i32>, !dbg !3245
  %232 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %230), !dbg !3251
  %233 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %232, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !3253
  %234 = bitcast <8 x float> %233 to <8 x i32>, !dbg !3259
  %235 = xor <8 x i32> %234, splat (i32 -1), !dbg !3265
  %236 = and <8 x i32> %235, %231, !dbg !3267
  store <8 x i32> %236, ptr %_77.i131, align 32, !dbg !3271, !alias.scope !3272, !noalias !3274
  %237 = bitcast <8 x i32> %236 to <8 x float>, !dbg !3275
  %238 = fadd <8 x float> %lanes.i1830.sroa.0.0.copyload, %237, !dbg !3276
  %239 = fmul <8 x float> %238, splat (float 0x3FC542A5A0000000), !dbg !3283
  %240 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %239, <8 x float> splat (float -1.260000e+02)), !dbg !3289
  %241 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %240, <8 x float> splat (float 1.270000e+02)), !dbg !3295
  %242 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %241), !dbg !3300
  %243 = fsub <8 x float> %241, %242, !dbg !3305
  %244 = fmul <8 x float> %243, splat (float 0x3F5E974FA0000000), !dbg !3310
  %245 = fadd <8 x float> %244, splat (float 0x3F82778560000000), !dbg !3315
  %246 = fmul <8 x float> %243, %245, !dbg !3310
  %247 = fadd <8 x float> %246, splat (float 0x3FAC91CE60000000), !dbg !3315
  %248 = fmul <8 x float> %243, %247, !dbg !3310
  %249 = fadd <8 x float> %248, splat (float 0x3FCEBDB560000000), !dbg !3315
  %250 = fmul <8 x float> %243, %249, !dbg !3310
  %251 = fadd <8 x float> %250, splat (float 0x3FE62E4BA0000000), !dbg !3315
  %252 = fmul <8 x float> %243, %251, !dbg !3320
  %253 = fadd <8 x float> %252, splat (float 1.000000e+00), !dbg !3325
  %254 = fadd <8 x float> %242, splat (float 0x4160000FE0000000), !dbg !3330
  %255 = bitcast <8 x float> %254 to <8 x i32>, !dbg !3335
  %_3.i2767 = shl <8 x i32> %255, splat (i32 23), !dbg !3339
  %256 = bitcast <8 x i32> %_3.i2767 to <8 x float>, !dbg !3340
  %257 = fmul <8 x float> %253, %256, !dbg !3342
  %258 = fmul <8 x float> %lanes.i1900.sroa.0.0.copyload, %257, !dbg !3346
  %259 = fsub <8 x float> %258, %lanes.i1900.sroa.0.0.copyload, !dbg !3351
  %260 = fmul <8 x float> %lanes.i1823.sroa.0.0.copyload, %259, !dbg !3357
  %261 = fadd <8 x float> %lanes.i1900.sroa.0.0.copyload, %260, !dbg !3362
  %262 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %237, <8 x float> zeroinitializer, i8 0), !dbg !3366
  %263 = bitcast <8 x float> %262 to <8 x i32>, !dbg !3372
  %264 = bitcast <8 x float> %63 to <8 x i32>, !dbg !3372
  %265 = and <8 x i32> %263, %264, !dbg !3376
  %266 = bitcast <8 x float> %62 to <8 x i32>, !dbg !3378
  %267 = or <8 x i32> %265, %266, !dbg !3382
  %268 = or <8 x i32> %267, %37, !dbg !3382
  %269 = bitcast <8 x float> %61 to <8 x i32>, !dbg !3387
  %270 = icmp slt <8 x i32> %269, zeroinitializer, !dbg !3391
  %271 = select <8 x i1> %270, <8 x float> %258, <8 x float> %261, !dbg !3391
  %272 = icmp slt <8 x i32> %268, zeroinitializer, !dbg !3393
  %273 = select <8 x i1> %272, <8 x float> %lanes.i1900.sroa.0.0.copyload, <8 x float> %271, !dbg !3393
  store <8 x float> %182, ptr %_120.i68, align 4, !dbg !3398, !alias.scope !3404, !noalias !3408
  store <8 x float> %273, ptr %_128.i72, align 4, !dbg !3412, !alias.scope !3417, !noalias !3421
  %274 = trunc i64 %spec.store.select.i91 to i32, !dbg !3425
  store i32 %274, ptr %27, align 32, !dbg !3425, !alias.scope !1687, !noalias !1696
  store i32 %274, ptr %38, align 32, !dbg !3426, !alias.scope !1690, !noalias !1775
  %exitcond5944.not = icmp eq i64 %57, %spec.store.select, !dbg !3427
  br i1 %exitcond5944.not, label %bb10, label %bb41.i63, !dbg !1780

bb57.i132:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1916
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_57.i118, i64 noundef %_194.1.i124, i64 noundef %_194.1.i124, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b5bf0b379ba879ef3901108f927dd528) #23, !dbg !3431, !noalias !1791
  unreachable, !dbg !3431

bb10:                                             ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit562, %start
  br i1 %_24, label %bb11, label %bb19, !dbg !3432

bb11:                                             ; preds = %bb10
  %_34 = sub nsw i64 %frames, %spec.store.select, !dbg !3433
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3434), !dbg !3437
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3438), !dbg !3437
  %_4.i2768 = icmp ult i64 %_34, 129, !dbg !3440
  br i1 %_4.i2768, label %bb1.i1.preheader.i, label %bb15, !dbg !3440

bb1.i1.preheader.i:                               ; preds = %bb11
  %275 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1440
  %_5.i7.i = load i32, ptr %275, align 4, !dbg !3443, !alias.scope !3449, !noalias !3438, !noundef !11
  %276 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1444, !dbg !3443
  %_5.i7.1.i = load i32, ptr %276, align 4, !dbg !3443, !alias.scope !3449, !noalias !3438, !noundef !11
  %least.sroa.0.1.i9.1.i = tail call i32 @llvm.umin.i32(i32 %_5.i7.1.i, i32 %_5.i7.i), !dbg !3443
  %277 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1448, !dbg !3443
  %_5.i7.2.i = load i32, ptr %277, align 4, !dbg !3443, !alias.scope !3449, !noalias !3438, !noundef !11
  %least.sroa.0.1.i9.2.i = tail call i32 @llvm.umin.i32(i32 %_5.i7.2.i, i32 %least.sroa.0.1.i9.1.i), !dbg !3443
  %278 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1452, !dbg !3443
  %_5.i7.3.i = load i32, ptr %278, align 4, !dbg !3443, !alias.scope !3449, !noalias !3438, !noundef !11
  %least.sroa.0.1.i9.3.i = tail call i32 @llvm.umin.i32(i32 %_5.i7.3.i, i32 %least.sroa.0.1.i9.2.i), !dbg !3443
  %279 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1456, !dbg !3443
  %_5.i7.4.i = load i32, ptr %279, align 4, !dbg !3443, !alias.scope !3449, !noalias !3438, !noundef !11
  %least.sroa.0.1.i9.4.i = tail call i32 @llvm.umin.i32(i32 %_5.i7.4.i, i32 %least.sroa.0.1.i9.3.i), !dbg !3443
  %280 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1460, !dbg !3443
  %_5.i7.5.i = load i32, ptr %280, align 4, !dbg !3443, !alias.scope !3449, !noalias !3438, !noundef !11
  %least.sroa.0.1.i9.5.i = tail call i32 @llvm.umin.i32(i32 %_5.i7.5.i, i32 %least.sroa.0.1.i9.4.i), !dbg !3443
  %281 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1464, !dbg !3443
  %_5.i7.6.i = load i32, ptr %281, align 4, !dbg !3443, !alias.scope !3449, !noalias !3438, !noundef !11
  %least.sroa.0.1.i9.6.i = tail call i32 @llvm.umin.i32(i32 %_5.i7.6.i, i32 %least.sroa.0.1.i9.5.i), !dbg !3443
  %282 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1468, !dbg !3443
  %_5.i7.7.i = load i32, ptr %282, align 4, !dbg !3443, !alias.scope !3449, !noalias !3438, !noundef !11
  %least.sroa.0.1.i9.7.i = tail call i32 @llvm.umin.i32(i32 %_5.i7.7.i, i32 %least.sroa.0.1.i9.6.i), !dbg !3443
  %_0.i5.i = zext i32 %least.sroa.0.1.i9.7.i to i64, !dbg !3452
  %_5.not.i = icmp samesign ugt i64 %_34, %_0.i5.i, !dbg !3453
  br i1 %_5.not.i, label %bb15, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel20segment_is_stageableNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit, !dbg !3453

_RINvNtCse3bfmKSZS8Y_10compressor6kernel20segment_is_stageableNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit: ; preds = %bb1.i1.preheader.i
  %283 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1440
  %_5.i.i = load i32, ptr %283, align 4, !dbg !3454, !alias.scope !3456, !noalias !3434, !noundef !11
  %284 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1444, !dbg !3454
  %_5.i.1.i = load i32, ptr %284, align 4, !dbg !3454, !alias.scope !3456, !noalias !3434, !noundef !11
  %least.sroa.0.1.i.1.i = tail call i32 @llvm.umin.i32(i32 %_5.i.1.i, i32 %_5.i.i), !dbg !3454
  %285 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1448, !dbg !3454
  %_5.i.2.i = load i32, ptr %285, align 4, !dbg !3454, !alias.scope !3456, !noalias !3434, !noundef !11
  %least.sroa.0.1.i.2.i = tail call i32 @llvm.umin.i32(i32 %_5.i.2.i, i32 %least.sroa.0.1.i.1.i), !dbg !3454
  %286 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1452, !dbg !3454
  %_5.i.3.i = load i32, ptr %286, align 4, !dbg !3454, !alias.scope !3456, !noalias !3434, !noundef !11
  %least.sroa.0.1.i.3.i = tail call i32 @llvm.umin.i32(i32 %_5.i.3.i, i32 %least.sroa.0.1.i.2.i), !dbg !3454
  %287 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1456, !dbg !3454
  %_5.i.4.i = load i32, ptr %287, align 4, !dbg !3454, !alias.scope !3456, !noalias !3434, !noundef !11
  %least.sroa.0.1.i.4.i = tail call i32 @llvm.umin.i32(i32 %_5.i.4.i, i32 %least.sroa.0.1.i.3.i), !dbg !3454
  %288 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1460, !dbg !3454
  %_5.i.5.i = load i32, ptr %288, align 4, !dbg !3454, !alias.scope !3456, !noalias !3434, !noundef !11
  %least.sroa.0.1.i.5.i = tail call i32 @llvm.umin.i32(i32 %_5.i.5.i, i32 %least.sroa.0.1.i.4.i), !dbg !3454
  %289 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1464, !dbg !3454
  %_5.i.6.i = load i32, ptr %289, align 4, !dbg !3454, !alias.scope !3456, !noalias !3434, !noundef !11
  %least.sroa.0.1.i.6.i = tail call i32 @llvm.umin.i32(i32 %_5.i.6.i, i32 %least.sroa.0.1.i.5.i), !dbg !3454
  %290 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1468, !dbg !3454
  %_5.i.7.i = load i32, ptr %290, align 4, !dbg !3454, !alias.scope !3456, !noalias !3434, !noundef !11
  %least.sroa.0.1.i.7.i = tail call i32 @llvm.umin.i32(i32 %_5.i.7.i, i32 %least.sroa.0.1.i.6.i), !dbg !3454
  %_0.i.i = zext i32 %least.sroa.0.1.i.7.i to i64, !dbg !3459
  %.not = icmp samesign ugt i64 %_34, %_0.i.i, !dbg !3460
  br i1 %.not, label %bb15, label %bb13, !dbg !3437

bb19:                                             ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit144.i, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit520, %bb10
  ret void, !dbg !3461

bb15:                                             ; preds = %bb11, %bb1.i1.preheader.i, %_RINvNtCse3bfmKSZS8Y_10compressor6kernel20segment_is_stageableNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3462), !dbg !3465
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3466), !dbg !3465
  %291 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1540, !dbg !3468
  %_12.i = load i32, ptr %291, align 4, !dbg !3468, !alias.scope !3462, !noalias !3472, !noundef !11
  %ring_length.i = zext i32 %_12.i to i64, !dbg !3468
  %292 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !3476
  %293 = bitcast <8 x float> %292 to <8 x i32>, !dbg !3484
  %294 = xor <8 x i32> %293, splat (i32 -1), !dbg !3490
  %295 = bitcast <8 x i32> %294 to <8 x float>, !dbg !3484
  switch i32 %link, label %bb13.i662 [
    i32 1, label %bb14.i663
    i32 3, label %bb14.i663.fold.split
  ], !dbg !3492

bb13.i662:                                        ; preds = %bb15
  br label %bb14.i663, !dbg !3493

bb14.i663.fold.split:                             ; preds = %bb15
  br label %bb14.i663, !dbg !3494

bb14.i663:                                        ; preds = %bb15, %bb14.i663.fold.split, %bb13.i662
  %_11.i651.sroa.0.04117 = phi <8 x float> [ %295, %bb15 ], [ %292, %bb13.i662 ], [ %292, %bb14.i663.fold.split ]
  %_13.i650.sroa.0.0 = phi <8 x i32> [ %294, %bb15 ], [ %294, %bb13.i662 ], [ %293, %bb14.i663.fold.split ], !dbg !3495
  %_4.i790 = getelementptr inbounds nuw i8, ptr %channels.0, i64 448, !dbg !3496
  %lanes.i1550.sroa.0.0.copyload = load <8 x float>, ptr %_4.i790, align 32, !dbg !3499, !alias.scope !3504, !noalias !3508
  %_7.i791 = getelementptr inbounds nuw i8, ptr %channels.0, i64 480, !dbg !3514
  %lanes.i1543.sroa.0.0.copyload = load <8 x float>, ptr %_7.i791, align 32, !dbg !3515, !alias.scope !3520, !noalias !3524
  %_15.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 256, !dbg !3528
  %lanes.i1536.sroa.0.0.copyload = load <8 x float>, ptr %_15.i, align 32, !dbg !3529, !alias.scope !3534, !noalias !3538
  %_14.i792 = getelementptr inbounds nuw i8, ptr %channels.0, i64 288, !dbg !3542
  %lanes.i1529.sroa.0.0.copyload = load <8 x float>, ptr %_14.i792, align 32, !dbg !3543, !alias.scope !3548, !noalias !3552
  %_17.i793 = getelementptr inbounds nuw i8, ptr %channels.0, i64 320, !dbg !3556
  %lanes.i1522.sroa.0.0.copyload = load <8 x float>, ptr %_17.i793, align 32, !dbg !3557, !alias.scope !3562, !noalias !3566
  %_20.i794 = getelementptr inbounds nuw i8, ptr %channels.0, i64 352, !dbg !3570
  %lanes.i1515.sroa.0.0.copyload = load <8 x float>, ptr %_20.i794, align 32, !dbg !3571, !alias.scope !3576, !noalias !3580
  %_23.i795 = getelementptr inbounds nuw i8, ptr %channels.0, i64 384, !dbg !3584
  %lanes.i1508.sroa.0.0.copyload = load <8 x float>, ptr %_23.i795, align 32, !dbg !3585, !alias.scope !3590, !noalias !3594
  %_26.i796 = getelementptr inbounds nuw i8, ptr %channels.0, i64 416, !dbg !3598
  %lanes.i.sroa.0.0.copyload = load <8 x float>, ptr %_26.i796, align 32, !dbg !3599, !alias.scope !3604, !noalias !3608
  %_4.i768 = getelementptr inbounds nuw i8, ptr %channels.1, i64 448, !dbg !3612
  %lanes.i1606.sroa.0.0.copyload = load <8 x float>, ptr %_4.i768, align 32, !dbg !3615, !alias.scope !3620, !noalias !3624
  %_7.i769 = getelementptr inbounds nuw i8, ptr %channels.1, i64 480, !dbg !3630
  %lanes.i1599.sroa.0.0.copyload = load <8 x float>, ptr %_7.i769, align 32, !dbg !3631, !alias.scope !3636, !noalias !3640
  %_17.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 256, !dbg !3644
  %lanes.i1592.sroa.0.0.copyload = load <8 x float>, ptr %_17.i, align 32, !dbg !3645, !alias.scope !3650, !noalias !3654
  %_14.i770 = getelementptr inbounds nuw i8, ptr %channels.1, i64 288, !dbg !3658
  %lanes.i1585.sroa.0.0.copyload = load <8 x float>, ptr %_14.i770, align 32, !dbg !3659, !alias.scope !3664, !noalias !3668
  %_17.i771 = getelementptr inbounds nuw i8, ptr %channels.1, i64 320, !dbg !3672
  %lanes.i1578.sroa.0.0.copyload = load <8 x float>, ptr %_17.i771, align 32, !dbg !3673, !alias.scope !3678, !noalias !3682
  %_20.i772 = getelementptr inbounds nuw i8, ptr %channels.1, i64 352, !dbg !3686
  %lanes.i1571.sroa.0.0.copyload = load <8 x float>, ptr %_20.i772, align 32, !dbg !3687, !alias.scope !3692, !noalias !3696
  %_23.i773 = getelementptr inbounds nuw i8, ptr %channels.1, i64 384, !dbg !3700
  %lanes.i1564.sroa.0.0.copyload = load <8 x float>, ptr %_23.i773, align 32, !dbg !3701, !alias.scope !3706, !noalias !3710
  %_26.i774 = getelementptr inbounds nuw i8, ptr %channels.1, i64 416, !dbg !3714
  %lanes.i1557.sroa.0.0.copyload = load <8 x float>, ptr %_26.i774, align 32, !dbg !3715, !alias.scope !3720, !noalias !3724
  %296 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i1543.sroa.0.0.copyload, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !3728
  %297 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i1543.sroa.0.0.copyload, <8 x float> zeroinitializer, i8 0), !dbg !3734
  %298 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i1550.sroa.0.0.copyload, <8 x float> zeroinitializer, i8 0), !dbg !3740
  %299 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i1599.sroa.0.0.copyload, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !3746
  %300 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i1599.sroa.0.0.copyload, <8 x float> zeroinitializer, i8 0), !dbg !3752
  %301 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i1606.sroa.0.0.copyload, <8 x float> zeroinitializer, i8 0), !dbg !3758
  %302 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1440, !dbg !3764
  %_91.i = load i32, ptr %302, align 32, !dbg !3764, !alias.scope !3462, !noalias !3472, !noundef !11
  %_90.i = zext i32 %_91.i to i64, !dbg !3764
  %303 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1444, !dbg !3769
  %_94.i = load i32, ptr %303, align 4, !dbg !3769, !alias.scope !3462, !noalias !3472, !noundef !11
  %_92.not.i = icmp ne i32 %_94.i, %_91.i, !dbg !3769
  %304 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1448, !dbg !3769
  %_94.i.1 = load i32, ptr %304, align 4, !dbg !3769
  %_92.not.i.1 = icmp ne i32 %_94.i.1, %_91.i, !dbg !3769
  %or.cond7213.not7242 = select i1 %_92.not.i, i1 true, i1 %_92.not.i.1, !dbg !3769
  %305 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1452, !dbg !3769
  %_94.i.2 = load i32, ptr %305, align 4, !dbg !3769
  %_92.not.i.2 = icmp ne i32 %_94.i.2, %_91.i, !dbg !3769
  %or.cond7214.not7241 = select i1 %or.cond7213.not7242, i1 true, i1 %_92.not.i.2, !dbg !3769
  %306 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1456, !dbg !3769
  %_94.i.3 = load i32, ptr %306, align 4, !dbg !3769
  %_92.not.i.3 = icmp ne i32 %_94.i.3, %_91.i, !dbg !3769
  %or.cond7215.not7240 = select i1 %or.cond7214.not7241, i1 true, i1 %_92.not.i.3, !dbg !3769
  %307 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1460, !dbg !3769
  %_94.i.4 = load i32, ptr %307, align 4, !dbg !3769
  %_92.not.i.4 = icmp ne i32 %_94.i.4, %_91.i, !dbg !3769
  %or.cond7216.not7239 = select i1 %or.cond7215.not7240, i1 true, i1 %_92.not.i.4, !dbg !3769
  %308 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1464, !dbg !3769
  %_94.i.5 = load i32, ptr %308, align 4, !dbg !3769
  %_92.not.i.5 = icmp ne i32 %_94.i.5, %_91.i, !dbg !3769
  %or.cond7217.not7238 = select i1 %or.cond7216.not7239, i1 true, i1 %_92.not.i.5, !dbg !3769
  %309 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1468, !dbg !3769
  %_94.i.6 = load i32, ptr %309, align 4, !dbg !3769
  %_92.not.i.6 = icmp ne i32 %_94.i.6, %_91.i, !dbg !3769
  %or.cond7218.not = select i1 %or.cond7217.not7238, i1 true, i1 %_92.not.i.6, !dbg !3769
  %310 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1440, !dbg !3773
  %_101.i = load i32, ptr %310, align 32, !dbg !3773, !alias.scope !3466, !noalias !3776, !noundef !11
  %_100.i = zext i32 %_101.i to i64, !dbg !3773
  %311 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1444, !dbg !3777
  %_104.i = load i32, ptr %311, align 4, !dbg !3777, !alias.scope !3466, !noalias !3776, !noundef !11
  %_102.not.i = icmp ne i32 %_104.i, %_101.i, !dbg !3777
  %312 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1448, !dbg !3777
  %_104.i.1 = load i32, ptr %312, align 4, !dbg !3777
  %_102.not.i.1 = icmp ne i32 %_104.i.1, %_101.i, !dbg !3777
  %or.cond7219.not7247 = select i1 %_102.not.i, i1 true, i1 %_102.not.i.1, !dbg !3777
  %313 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1452, !dbg !3777
  %_104.i.2 = load i32, ptr %313, align 4, !dbg !3777
  %_102.not.i.2 = icmp ne i32 %_104.i.2, %_101.i, !dbg !3777
  %or.cond7220.not7246 = select i1 %or.cond7219.not7247, i1 true, i1 %_102.not.i.2, !dbg !3777
  %314 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1456, !dbg !3777
  %_104.i.3 = load i32, ptr %314, align 4, !dbg !3777
  %_102.not.i.3 = icmp ne i32 %_104.i.3, %_101.i, !dbg !3777
  %or.cond7221.not7245 = select i1 %or.cond7220.not7246, i1 true, i1 %_102.not.i.3, !dbg !3777
  %315 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1460, !dbg !3777
  %_104.i.4 = load i32, ptr %315, align 4, !dbg !3777
  %_102.not.i.4 = icmp ne i32 %_104.i.4, %_101.i, !dbg !3777
  %or.cond7222.not7244 = select i1 %or.cond7221.not7245, i1 true, i1 %_102.not.i.4, !dbg !3777
  %316 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1464, !dbg !3777
  %_104.i.5 = load i32, ptr %316, align 4, !dbg !3777
  %_102.not.i.5 = icmp ne i32 %_104.i.5, %_101.i, !dbg !3777
  %or.cond7223.not7243 = select i1 %or.cond7222.not7244, i1 true, i1 %_102.not.i.5, !dbg !3777
  %317 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1468, !dbg !3777
  %_104.i.6 = load i32, ptr %317, align 4, !dbg !3777
  %_102.not.i.6 = icmp ne i32 %_104.i.6, %_101.i, !dbg !3777
  %or.cond7224.not = select i1 %or.cond7223.not7243, i1 true, i1 %_102.not.i.6, !dbg !3777
  %_6.i.i = load i64, ptr %detector, align 8, !range !220
  %318 = getelementptr inbounds nuw i8, ptr %detector, i64 16
  %sidechain_left.1.i.i = load i64, ptr %318, align 8
  %319 = getelementptr inbounds nuw i8, ptr %detector, i64 24
  %sidechain_right.0.i.i = load ptr, ptr %319, align 8, !nonnull !11, !align !3781
  %320 = getelementptr inbounds nuw i8, ptr %detector, i64 32
  %sidechain_right.1.i.i = load i64, ptr %320, align 8
  %321 = getelementptr inbounds nuw i8, ptr %detector, i64 8
  %sidechain_left.0.i.i = load ptr, ptr %321, align 8, !nonnull !11, !align !3781
  %322 = icmp slt <8 x i32> %_13.i650.sroa.0.0, zeroinitializer
  %323 = bitcast <8 x float> %_11.i651.sroa.0.04117 to <8 x i32>
  %324 = icmp slt <8 x i32> %323, zeroinitializer
  %325 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1536
  %326 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1504
  %327 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1512
  %328 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1528
  %329 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1520
  %330 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1504
  %331 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1512
  %332 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1528
  %333 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1520
  %334 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1540
  %_73.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 1472
  %335 = bitcast <8 x float> %298 to <8 x i32>
  %336 = bitcast <8 x float> %297 to <8 x i32>
  %337 = select i1 %bypass, <8 x i32> %293, <8 x i32> %294
  %338 = bitcast <8 x float> %296 to <8 x i32>
  %339 = icmp slt <8 x i32> %338, zeroinitializer
  %_77.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 1472
  %340 = bitcast <8 x float> %301 to <8 x i32>
  %341 = bitcast <8 x float> %300 to <8 x i32>
  %342 = bitcast <8 x float> %299 to <8 x i32>
  %343 = icmp slt <8 x i32> %342, zeroinitializer
  %344 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1536
  %345 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1448
  %346 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1452
  %347 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1456
  %348 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1460
  %349 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1464
  %350 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1468
  %351 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1448
  %352 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1452
  %353 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1456
  %354 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1460
  %355 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1464
  %356 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1468
  %357 = fneg <8 x float> %lanes.i1522.sroa.0.0.copyload
  %358 = fneg <8 x float> %lanes.i1578.sroa.0.0.copyload
  br label %bb41.i, !dbg !3782

bb41.i:                                           ; preds = %bb14.i663, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit520
  %start1.sroa.0.0.i5044 = phi i64 [ %spec.store.select, %bb14.i663 ], [ %359, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit520 ]
  %359 = add nuw nsw i64 %start1.sroa.0.0.i5044, 1, !dbg !3791
  %slot.i = shl nuw nsw i64 %start1.sroa.0.0.i5044, 3, !dbg !3799
  %_113.i = icmp samesign ugt i64 %slot.i, %left.1, !dbg !3801
  br i1 %_113.i, label %bb43.i, label %bb44.i, !dbg !3801, !prof !161

bb44.i:                                           ; preds = %bb41.i
  %_116.i = sub nuw nsw i64 %left.1, %slot.i, !dbg !3807
  %_120.i = getelementptr inbounds nuw float, ptr %left.0, i64 %slot.i, !dbg !3808
  %_8.i2002 = icmp samesign ugt i64 %_116.i, 7, !dbg !3813
  br i1 %_8.i2002, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit2006, label %bb2.i2003, !dbg !3813, !prof !2135

bb2.i2003:                                        ; preds = %bb44.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_116.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !3818, !noalias !3819
  unreachable, !dbg !3818

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit2006: ; preds = %bb44.i
  %lanes.i1999.sroa.0.0.copyload = load <8 x float>, ptr %_120.i, align 4, !dbg !3823, !alias.scope !3827, !noalias !3831
  %_121.i = icmp samesign ugt i64 %slot.i, %right.1, !dbg !3833
  br i1 %_121.i, label %bb45.i, label %bb46.i, !dbg !3833, !prof !161

bb43.i:                                           ; preds = %bb41.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %slot.i, i64 noundef range(i64 8, 34359738361) %left.1, i64 noundef range(i64 8, 34359738361) %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c46c6dde8c9e0eb71c8d3a58ebc90caa) #23, !dbg !3838, !noalias !3839
  unreachable, !dbg !3838

bb46.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit2006
  %_124.i = sub nuw nsw i64 %right.1, %slot.i, !dbg !3840
  %_128.i = getelementptr inbounds nuw float, ptr %right.0, i64 %slot.i, !dbg !3841
  %_8.i1993 = icmp samesign ugt i64 %_124.i, 7, !dbg !3846
  br i1 %_8.i1993, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1997, label %bb2.i1994, !dbg !3846, !prof !2135

bb2.i1994:                                        ; preds = %bb46.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_124.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !3851, !noalias !3852
  unreachable, !dbg !3851

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1997: ; preds = %bb46.i
  %lanes.i1990.sroa.0.0.copyload = load <8 x float>, ptr %_128.i, align 4, !dbg !3856, !alias.scope !3860, !noalias !3864
  switch i64 %_6.i.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1997.unreachabledefault [
    i64 0, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i
    i64 1, label %bb3.i.i
    i64 2, label %bb2.i.i
  ], !dbg !3866

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1997.unreachabledefault: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1997
  unreachable

default.unreachable:                              ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit686.i
  unreachable

bb3.i.i:                                          ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1997
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i, !dbg !3869

bb2.i.i:                                          ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1997
  %_25.i.i = icmp ugt i64 %slot.i, %sidechain_left.1.i.i, !dbg !3870
  br i1 %_25.i.i, label %bb17.i.i, label %bb18.i.i, !dbg !3870, !prof !161

bb18.i.i:                                         ; preds = %bb2.i.i
  %_28.i.i = sub nuw i64 %sidechain_left.1.i.i, %slot.i, !dbg !3873
  %_8.i1984 = icmp samesign ugt i64 %_28.i.i, 7, !dbg !3874
  br i1 %_8.i1984, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1988, label %bb2.i1985, !dbg !3874, !prof !2135

bb2.i1985:                                        ; preds = %bb18.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_28.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !3879, !noalias !3880
  unreachable, !dbg !3879

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1988: ; preds = %bb18.i.i
  %_32.i.i = getelementptr inbounds nuw float, ptr %sidechain_left.0.i.i, i64 %slot.i, !dbg !3889
  %lanes.i1981.sroa.0.0.copyload = load <8 x float>, ptr %_32.i.i, align 4, !dbg !3891, !alias.scope !3895, !noalias !3899
  %_33.i.i = icmp ugt i64 %slot.i, %sidechain_right.1.i.i, !dbg !3901
  br i1 %_33.i.i, label %bb19.i.i, label %bb20.i.i, !dbg !3901, !prof !161

bb17.i.i:                                         ; preds = %bb2.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 34359738361) %slot.i, i64 noundef %sidechain_left.1.i.i, i64 noundef %sidechain_left.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7fe5a91ec575deda110c046c84630100) #23, !dbg !3904, !noalias !3905
  unreachable, !dbg !3904

bb20.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1988
  %_36.i.i = sub nuw i64 %sidechain_right.1.i.i, %slot.i, !dbg !3907
  %_8.i1975 = icmp samesign ugt i64 %_36.i.i, 7, !dbg !3908
  br i1 %_8.i1975, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1979, label %bb2.i1976, !dbg !3908, !prof !2135

bb2.i1976:                                        ; preds = %bb20.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_36.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !3913, !noalias !3914
  unreachable, !dbg !3913

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1979: ; preds = %bb20.i.i
  %_40.i.i = getelementptr inbounds nuw float, ptr %sidechain_right.0.i.i, i64 %slot.i, !dbg !3918
  %lanes.i1972.sroa.0.0.copyload = load <8 x float>, ptr %_40.i.i, align 4, !dbg !3920, !alias.scope !3924, !noalias !3928
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i, !dbg !3930

bb19.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1988
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 34359738361) %slot.i, i64 noundef %sidechain_right.1.i.i, i64 noundef %sidechain_right.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f2cba1f89421a726339097d2eba48536) #23, !dbg !3931, !noalias !3905
  unreachable, !dbg !3931

_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1997, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1979, %bb3.i.i
  %.sroa.02856.0 = phi <8 x float> [ %lanes.i1990.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1997 ], [ zeroinitializer, %bb3.i.i ], [ %lanes.i1972.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1979 ], !dbg !3932
  %.sroa.02853.0 = phi <8 x float> [ %lanes.i1999.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1997 ], [ zeroinitializer, %bb3.i.i ], [ %lanes.i1981.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1979 ], !dbg !3932
  %360 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %.sroa.02853.0), !dbg !3933
  %361 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %.sroa.02856.0), !dbg !3939
  %362 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %360, <8 x float> %361), !dbg !3945
  %363 = fmul <8 x float> %360, splat (float 5.000000e-01), !dbg !3950
  %364 = fmul <8 x float> %361, splat (float 5.000000e-01), !dbg !3955
  %365 = fadd <8 x float> %363, %364, !dbg !3960
  %366 = select <8 x i1> %322, <8 x float> %365, <8 x float> %362, !dbg !3965
  %367 = select <8 x i1> %324, <8 x float> %366, <8 x float> %360, !dbg !3970
  %368 = select <8 x i1> %324, <8 x float> %366, <8 x float> %361, !dbg !3975
  %_39.i = load i32, ptr %325, align 32, !dbg !3980, !alias.scope !3462, !noalias !3472, !noundef !11
  %write.i = zext i32 %_39.i to i64, !dbg !3980
  %369 = add nuw nsw i64 %write.i, 1, !dbg !3982
  %_41.i = icmp eq i64 %369, %ring_length.i, !dbg !3984
  %spec.store.select.i = select i1 %_41.i, i64 0, i64 %369, !dbg !3984
  %_189.1.i = load i64, ptr %327, align 8, !dbg !3986, !alias.scope !3462, !noalias !3472, !noundef !11
  %_45.i = shl nuw nsw i64 %write.i, 3, !dbg !3988
  %_129.i = icmp ugt i64 %_45.i, %_189.1.i, !dbg !3989
  br i1 %_129.i, label %bb47.i, label %bb48.i, !dbg !3989, !prof !161

bb45.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit2006
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %slot.i, i64 noundef range(i64 8, 34359738361) %right.1, i64 noundef range(i64 8, 34359738361) %right.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9b27971f08fa00763b6dc53a9f4dbd4c) #23, !dbg !3994, !noalias !3839
  unreachable, !dbg !3994

bb48.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i
  %_132.i = sub nuw i64 %_189.1.i, %_45.i, !dbg !3995
  %_8.i2576 = icmp samesign ugt i64 %_132.i, 7, !dbg !3996
  br i1 %_8.i2576, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2579, label %bb2.i2577, !dbg !3996, !prof !2135

bb2.i2577:                                        ; preds = %bb48.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_132.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !4001, !noalias !4002
  unreachable, !dbg !4001

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2579: ; preds = %bb48.i
  %_189.0.i = load ptr, ptr %326, align 32, !dbg !3986, !alias.scope !3462, !noalias !3472, !nonnull !11, !noundef !11
  %_136.i = getelementptr inbounds nuw float, ptr %_189.0.i, i64 %_45.i, !dbg !4006
  store <8 x float> %lanes.i1999.sroa.0.0.copyload, ptr %_136.i, align 4, !dbg !4011, !alias.scope !4015, !noalias !4019
  %_190.1.i = load i64, ptr %328, align 8, !dbg !4021, !alias.scope !3462, !noalias !3472, !noundef !11
  %_137.i = icmp ugt i64 %_45.i, %_190.1.i, !dbg !4022
  br i1 %_137.i, label %bb49.i, label %bb50.i, !dbg !4022, !prof !161

bb47.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_45.i, i64 noundef %_189.1.i, i64 noundef %_189.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bb9110d0bc8cedfe643d9bd892c8c620) #23, !dbg !4026, !noalias !3839
  unreachable, !dbg !4026

bb50.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2579
  %_140.i = sub nuw i64 %_190.1.i, %_45.i, !dbg !4027
  %_8.i2571 = icmp samesign ugt i64 %_140.i, 7, !dbg !4028
  br i1 %_8.i2571, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2574, label %bb2.i2572, !dbg !4028, !prof !2135

bb2.i2572:                                        ; preds = %bb50.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_140.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !4033, !noalias !4034
  unreachable, !dbg !4033

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2574: ; preds = %bb50.i
  %_190.0.i = load ptr, ptr %329, align 16, !dbg !4021, !alias.scope !3462, !noalias !3472, !nonnull !11, !noundef !11
  %_144.i = getelementptr inbounds nuw float, ptr %_190.0.i, i64 %_45.i, !dbg !4038
  store <8 x float> %367, ptr %_144.i, align 4, !dbg !4043, !alias.scope !4047, !noalias !4051
  %_191.1.i = load i64, ptr %331, align 8, !dbg !4053, !alias.scope !3466, !noalias !3776, !noundef !11
  %_145.i = icmp ugt i64 %_45.i, %_191.1.i, !dbg !4054
  br i1 %_145.i, label %bb51.i, label %bb52.i, !dbg !4054, !prof !161

bb49.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2579
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_45.i, i64 noundef %_190.1.i, i64 noundef %_190.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_da5a912ec6b29b38396e4eb206c99989) #23, !dbg !4058, !noalias !3839
  unreachable, !dbg !4058

bb52.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2574
  %_148.i = sub nuw i64 %_191.1.i, %_45.i, !dbg !4059
  %_8.i2566 = icmp samesign ugt i64 %_148.i, 7, !dbg !4060
  br i1 %_8.i2566, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2569, label %bb2.i2567, !dbg !4060, !prof !2135

bb2.i2567:                                        ; preds = %bb52.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_148.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !4065, !noalias !4066
  unreachable, !dbg !4065

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2569: ; preds = %bb52.i
  %_191.0.i = load ptr, ptr %330, align 32, !dbg !4053, !alias.scope !3466, !noalias !3776, !nonnull !11, !noundef !11
  %_152.i = getelementptr inbounds nuw float, ptr %_191.0.i, i64 %_45.i, !dbg !4070
  store <8 x float> %lanes.i1990.sroa.0.0.copyload, ptr %_152.i, align 4, !dbg !4075, !alias.scope !4079, !noalias !4083
  %_192.1.i = load i64, ptr %332, align 8, !dbg !4085, !alias.scope !3466, !noalias !3776, !noundef !11
  %_153.i = icmp ugt i64 %_45.i, %_192.1.i, !dbg !4086
  br i1 %_153.i, label %bb53.i, label %bb54.i, !dbg !4086, !prof !161

bb51.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2574
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_45.i, i64 noundef %_191.1.i, i64 noundef %_191.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b46755d5101951cde0ca1c710062f433) #23, !dbg !4090, !noalias !3839
  unreachable, !dbg !4090

bb54.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2569
  %_156.i = sub nuw i64 %_192.1.i, %_45.i, !dbg !4091
  %_8.i2561 = icmp samesign ugt i64 %_156.i, 7, !dbg !4092
  br i1 %_8.i2561, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2564, label %bb2.i2562, !dbg !4092, !prof !2135

bb2.i2562:                                        ; preds = %bb54.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_156.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !4097, !noalias !4098
  unreachable, !dbg !4097

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2564: ; preds = %bb54.i
  %_192.0.i = load ptr, ptr %333, align 16, !dbg !4085, !alias.scope !3466, !noalias !3776, !nonnull !11, !noundef !11
  %_160.i = getelementptr inbounds nuw float, ptr %_192.0.i, i64 %_45.i, !dbg !4102
  store <8 x float> %368, ptr %_160.i, align 4, !dbg !4107, !alias.scope !4111, !noalias !4115
  %_193.1.i = load i64, ptr %327, align 8, !dbg !4117, !alias.scope !3462, !noalias !3472, !noundef !11
  %_57.i = shl nuw nsw i64 %spec.store.select.i, 3, !dbg !4118
  %_161.i = icmp ugt i64 %_57.i, %_193.1.i, !dbg !4119
  br i1 %_161.i, label %bb55.i, label %bb56.i, !dbg !4119, !prof !161

bb53.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2569
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_45.i, i64 noundef %_192.1.i, i64 noundef %_192.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2fb73b04dc26e2fd60c7be20365496fe) #23, !dbg !4123, !noalias !3839
  unreachable, !dbg !4123

bb56.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2564
  %_164.i = sub nuw i64 %_193.1.i, %_57.i, !dbg !4124
  %_8.i1966 = icmp samesign ugt i64 %_164.i, 7, !dbg !4125
  br i1 %_8.i1966, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1970, label %bb2.i1967, !dbg !4125, !prof !2135

bb2.i1967:                                        ; preds = %bb56.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_164.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !4130, !noalias !4131
  unreachable, !dbg !4130

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1970: ; preds = %bb56.i
  %_193.0.i = load ptr, ptr %326, align 32, !dbg !4117, !alias.scope !3462, !noalias !3472, !nonnull !11, !noundef !11
  %_168.i = getelementptr inbounds nuw float, ptr %_193.0.i, i64 %_57.i, !dbg !4135
  %lanes.i1963.sroa.0.0.copyload = load <8 x float>, ptr %_168.i, align 4, !dbg !4140, !alias.scope !4144, !noalias !4148
  %_194.1.i = load i64, ptr %331, align 8, !dbg !4150, !alias.scope !3466, !noalias !3776, !noundef !11
  %_169.i = icmp ugt i64 %_57.i, %_194.1.i, !dbg !4152
  br i1 %_169.i, label %bb57.i, label %bb58.i, !dbg !4152, !prof !161

bb55.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2564
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_57.i, i64 noundef %_193.1.i, i64 noundef %_193.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_322bbb2a85bd2347c4d3c1d32e8c0bc7) #23, !dbg !4156, !noalias !3839
  unreachable, !dbg !4156

bb58.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1970
  %_172.i = sub nuw i64 %_194.1.i, %_57.i, !dbg !4157
  %_8.i1957 = icmp samesign ugt i64 %_172.i, 7, !dbg !4158
  br i1 %_8.i1957, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1961, label %bb2.i1958, !dbg !4158, !prof !2135

bb2.i1958:                                        ; preds = %bb58.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_172.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !4163, !noalias !4164
  unreachable, !dbg !4163

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1961: ; preds = %bb58.i
  %_194.0.i = load ptr, ptr %330, align 32, !dbg !4150, !alias.scope !3466, !noalias !3776, !nonnull !11, !noundef !11
  %_176.i = getelementptr inbounds nuw float, ptr %_194.0.i, i64 %_57.i, !dbg !4168
  %lanes.i1954.sroa.0.0.copyload = load <8 x float>, ptr %_176.i, align 4, !dbg !4173, !alias.scope !4177, !noalias !4181
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4183), !dbg !4186
  %_6.i279 = load i32, ptr %291, align 4, !dbg !4188, !alias.scope !4183, !noalias !4190, !noundef !11
  %ring_length.i280 = zext i32 %_6.i279 to i64, !dbg !4188
  br i1 %or.cond7218.not, label %bb7.i299.preheader, label %bb1.i281, !dbg !4193

bb7.i299.preheader:                               ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1961
  %_56.1.i313 = load i64, ptr %328, align 8
  %_56.0.i317 = load ptr, ptr %329, align 16, !nonnull !11
  %_25.i305 = load i32, ptr %302, align 4, !dbg !4194, !alias.scope !4183, !noalias !4190, !noundef !11
  %delay1.i306 = zext i32 %_25.i305 to i64, !dbg !4194
  %_28.not.i307 = icmp ult i32 %_39.i, %_25.i305, !dbg !4195
  %_29.i308 = select i1 %_28.not.i307, i64 %ring_length.i280, i64 0, !dbg !4195
  %write.pn7.i309 = sub nsw i64 %write.i, %delay1.i306, !dbg !4195
  %tap.sroa.0.0.i310 = add nsw i64 %write.pn7.i309, %_29.i308, !dbg !4196
  %_32.i311 = shl nsw i64 %tap.sroa.0.0.i310, 3, !dbg !4197
  %_34.i314 = icmp ult i64 %_32.i311, %_56.1.i313, !dbg !4198
  br i1 %_34.i314, label %bb16.i316, label %panic2.i315, !dbg !4198

bb1.i281:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1961
  %_10.not.i282 = icmp ult i32 %_39.i, %_91.i, !dbg !4199
  %_11.i283 = select i1 %_10.not.i282, i64 %ring_length.i280, i64 0, !dbg !4199
  %write.pn.i284 = sub nsw i64 %write.i, %_90.i, !dbg !4199
  %row.sroa.0.0.i285 = add nsw i64 %write.pn.i284, %_11.i283, !dbg !4200
  %_55.1.i286 = load i64, ptr %328, align 8, !dbg !4201, !alias.scope !4183, !noalias !4190, !noundef !11
  %_13.i287 = shl nsw i64 %row.sroa.0.0.i285, 3, !dbg !4202
  %_40.i288 = icmp ugt i64 %_13.i287, %_55.1.i286, !dbg !4203
  br i1 %_40.i288, label %bb19.i293, label %bb20.i289, !dbg !4203, !prof !161

bb20.i289:                                        ; preds = %bb1.i281
  %_43.i291 = sub nuw i64 %_55.1.i286, %_13.i287, !dbg !4206
  %_8.i1847 = icmp samesign ugt i64 %_43.i291, 7, !dbg !4207
  br i1 %_8.i1847, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1850, label %bb2.i, !dbg !4207, !prof !2135

bb2.i:                                            ; preds = %bb20.i289
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_43.i291, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !4212, !noalias !4213
  unreachable, !dbg !4212

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1850: ; preds = %bb20.i289
  %_55.0.i290 = load ptr, ptr %329, align 16, !dbg !4201, !alias.scope !4183, !noalias !4190, !nonnull !11, !noundef !11
  %_47.i292 = getelementptr inbounds nuw float, ptr %_55.0.i290, i64 %_13.i287, !dbg !4217
  %lanes.i1844.sroa.0.0.copyload = load <8 x float>, ptr %_47.i292, align 4, !dbg !4219, !alias.scope !4223, !noalias !4227
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit320, !dbg !4229

bb19.i293:                                        ; preds = %bb1.i281
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_13.i287, i64 noundef %_55.1.i286, i64 noundef %_55.1.i286, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_942b76445104c042a726096e85fe3fcb) #23, !dbg !4230, !noalias !4231
  unreachable, !dbg !4230

bb11.i319:                                        ; preds = %bb12.i304.7
  %370 = getelementptr inbounds nuw float, ptr %_56.0.i317, i64 %_31.i312.6, !dbg !4198
  %_30.i318.6 = load float, ptr %370, align 4, !dbg !4198, !noalias !4232, !noundef !11
  %371 = getelementptr inbounds nuw float, ptr %_56.0.i317, i64 %_31.i312.7, !dbg !4198
  %_30.i318.7 = load float, ptr %371, align 4, !dbg !4198, !noalias !4232, !noundef !11
  %lanes.i1837.sroa.0.0.vec.insert = insertelement <8 x float> poison, float %_30.i318, i64 0, !dbg !4233
  %lanes.i1837.sroa.0.4.vec.insert = insertelement <8 x float> %lanes.i1837.sroa.0.0.vec.insert, float %_30.i318.1, i64 1, !dbg !4233
  %lanes.i1837.sroa.0.8.vec.insert = insertelement <8 x float> %lanes.i1837.sroa.0.4.vec.insert, float %_30.i318.2, i64 2, !dbg !4233
  %lanes.i1837.sroa.0.12.vec.insert = insertelement <8 x float> %lanes.i1837.sroa.0.8.vec.insert, float %_30.i318.3, i64 3, !dbg !4233
  %lanes.i1837.sroa.0.16.vec.insert = insertelement <8 x float> %lanes.i1837.sroa.0.12.vec.insert, float %_30.i318.4, i64 4, !dbg !4233
  %lanes.i1837.sroa.0.20.vec.insert = insertelement <8 x float> %lanes.i1837.sroa.0.16.vec.insert, float %_30.i318.5, i64 5, !dbg !4233
  %lanes.i1837.sroa.0.24.vec.insert = insertelement <8 x float> %lanes.i1837.sroa.0.20.vec.insert, float %_30.i318.6, i64 6, !dbg !4233
  %lanes.i1837.sroa.0.28.vec.insert = insertelement <8 x float> %lanes.i1837.sroa.0.24.vec.insert, float %_30.i318.7, i64 7, !dbg !4233
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit320, !dbg !4229

bb16.i316:                                        ; preds = %bb7.i299.preheader
  %372 = getelementptr inbounds nuw float, ptr %_56.0.i317, i64 %_32.i311, !dbg !4198
  %_30.i318 = load float, ptr %372, align 4, !dbg !4198, !noalias !4232, !noundef !11
  %_25.i305.1 = load i32, ptr %303, align 4, !dbg !4194, !alias.scope !4183, !noalias !4190, !noundef !11
  %delay1.i306.1 = zext i32 %_25.i305.1 to i64, !dbg !4194
  %_28.not.i307.1 = icmp ult i32 %_39.i, %_25.i305.1, !dbg !4195
  %_29.i308.1 = select i1 %_28.not.i307.1, i64 %ring_length.i280, i64 0, !dbg !4195
  %write.pn7.i309.1 = sub nsw i64 %write.i, %delay1.i306.1, !dbg !4195
  %tap.sroa.0.0.i310.1 = add nsw i64 %write.pn7.i309.1, %_29.i308.1, !dbg !4196
  %_32.i311.1 = shl nsw i64 %tap.sroa.0.0.i310.1, 3, !dbg !4197
  %_31.i312.1 = or disjoint i64 %_32.i311.1, 1, !dbg !4197
  %_34.i314.1 = icmp ult i64 %_31.i312.1, %_56.1.i313, !dbg !4198
  br i1 %_34.i314.1, label %bb16.i316.1, label %panic2.i315, !dbg !4198

bb16.i316.1:                                      ; preds = %bb16.i316
  %373 = getelementptr inbounds nuw float, ptr %_56.0.i317, i64 %_31.i312.1, !dbg !4198
  %_30.i318.1 = load float, ptr %373, align 4, !dbg !4198, !noalias !4232, !noundef !11
  %_25.i305.2 = load i32, ptr %345, align 4, !dbg !4194, !alias.scope !4183, !noalias !4190, !noundef !11
  %delay1.i306.2 = zext i32 %_25.i305.2 to i64, !dbg !4194
  %_28.not.i307.2 = icmp ult i32 %_39.i, %_25.i305.2, !dbg !4195
  %_29.i308.2 = select i1 %_28.not.i307.2, i64 %ring_length.i280, i64 0, !dbg !4195
  %write.pn7.i309.2 = sub nsw i64 %write.i, %delay1.i306.2, !dbg !4195
  %tap.sroa.0.0.i310.2 = add nsw i64 %write.pn7.i309.2, %_29.i308.2, !dbg !4196
  %_32.i311.2 = shl nsw i64 %tap.sroa.0.0.i310.2, 3, !dbg !4197
  %_31.i312.2 = or disjoint i64 %_32.i311.2, 2, !dbg !4197
  %_34.i314.2 = icmp ult i64 %_31.i312.2, %_56.1.i313, !dbg !4198
  br i1 %_34.i314.2, label %bb16.i316.2, label %panic2.i315, !dbg !4198

bb16.i316.2:                                      ; preds = %bb16.i316.1
  %374 = getelementptr inbounds nuw float, ptr %_56.0.i317, i64 %_31.i312.2, !dbg !4198
  %_30.i318.2 = load float, ptr %374, align 4, !dbg !4198, !noalias !4232, !noundef !11
  %_25.i305.3 = load i32, ptr %346, align 4, !dbg !4194, !alias.scope !4183, !noalias !4190, !noundef !11
  %delay1.i306.3 = zext i32 %_25.i305.3 to i64, !dbg !4194
  %_28.not.i307.3 = icmp ult i32 %_39.i, %_25.i305.3, !dbg !4195
  %_29.i308.3 = select i1 %_28.not.i307.3, i64 %ring_length.i280, i64 0, !dbg !4195
  %write.pn7.i309.3 = sub nsw i64 %write.i, %delay1.i306.3, !dbg !4195
  %tap.sroa.0.0.i310.3 = add nsw i64 %write.pn7.i309.3, %_29.i308.3, !dbg !4196
  %_32.i311.3 = shl nsw i64 %tap.sroa.0.0.i310.3, 3, !dbg !4197
  %_31.i312.3 = or disjoint i64 %_32.i311.3, 3, !dbg !4197
  %_34.i314.3 = icmp ult i64 %_31.i312.3, %_56.1.i313, !dbg !4198
  br i1 %_34.i314.3, label %bb16.i316.3, label %panic2.i315, !dbg !4198

bb16.i316.3:                                      ; preds = %bb16.i316.2
  %375 = getelementptr inbounds nuw float, ptr %_56.0.i317, i64 %_31.i312.3, !dbg !4198
  %_30.i318.3 = load float, ptr %375, align 4, !dbg !4198, !noalias !4232, !noundef !11
  %_25.i305.4 = load i32, ptr %347, align 4, !dbg !4194, !alias.scope !4183, !noalias !4190, !noundef !11
  %delay1.i306.4 = zext i32 %_25.i305.4 to i64, !dbg !4194
  %_28.not.i307.4 = icmp ult i32 %_39.i, %_25.i305.4, !dbg !4195
  %_29.i308.4 = select i1 %_28.not.i307.4, i64 %ring_length.i280, i64 0, !dbg !4195
  %write.pn7.i309.4 = sub nsw i64 %write.i, %delay1.i306.4, !dbg !4195
  %tap.sroa.0.0.i310.4 = add nsw i64 %write.pn7.i309.4, %_29.i308.4, !dbg !4196
  %_32.i311.4 = shl nsw i64 %tap.sroa.0.0.i310.4, 3, !dbg !4197
  %_31.i312.4 = or disjoint i64 %_32.i311.4, 4, !dbg !4197
  %_34.i314.4 = icmp ult i64 %_31.i312.4, %_56.1.i313, !dbg !4198
  br i1 %_34.i314.4, label %bb16.i316.4, label %panic2.i315, !dbg !4198

bb16.i316.4:                                      ; preds = %bb16.i316.3
  %376 = getelementptr inbounds nuw float, ptr %_56.0.i317, i64 %_31.i312.4, !dbg !4198
  %_30.i318.4 = load float, ptr %376, align 4, !dbg !4198, !noalias !4232, !noundef !11
  %_25.i305.5 = load i32, ptr %348, align 4, !dbg !4194, !alias.scope !4183, !noalias !4190, !noundef !11
  %delay1.i306.5 = zext i32 %_25.i305.5 to i64, !dbg !4194
  %_28.not.i307.5 = icmp ult i32 %_39.i, %_25.i305.5, !dbg !4195
  %_29.i308.5 = select i1 %_28.not.i307.5, i64 %ring_length.i280, i64 0, !dbg !4195
  %write.pn7.i309.5 = sub nsw i64 %write.i, %delay1.i306.5, !dbg !4195
  %tap.sroa.0.0.i310.5 = add nsw i64 %write.pn7.i309.5, %_29.i308.5, !dbg !4196
  %_32.i311.5 = shl nsw i64 %tap.sroa.0.0.i310.5, 3, !dbg !4197
  %_31.i312.5 = or disjoint i64 %_32.i311.5, 5, !dbg !4197
  %_34.i314.5 = icmp ult i64 %_31.i312.5, %_56.1.i313, !dbg !4198
  br i1 %_34.i314.5, label %bb16.i316.5, label %panic2.i315, !dbg !4198

bb16.i316.5:                                      ; preds = %bb16.i316.4
  %377 = getelementptr inbounds nuw float, ptr %_56.0.i317, i64 %_31.i312.5, !dbg !4198
  %_30.i318.5 = load float, ptr %377, align 4, !dbg !4198, !noalias !4232, !noundef !11
  %_25.i305.6 = load i32, ptr %349, align 4, !dbg !4194, !alias.scope !4183, !noalias !4190, !noundef !11
  %delay1.i306.6 = zext i32 %_25.i305.6 to i64, !dbg !4194
  %_28.not.i307.6 = icmp ult i32 %_39.i, %_25.i305.6, !dbg !4195
  %_29.i308.6 = select i1 %_28.not.i307.6, i64 %ring_length.i280, i64 0, !dbg !4195
  %write.pn7.i309.6 = sub nsw i64 %write.i, %delay1.i306.6, !dbg !4195
  %tap.sroa.0.0.i310.6 = add nsw i64 %write.pn7.i309.6, %_29.i308.6, !dbg !4196
  %_32.i311.6 = shl nsw i64 %tap.sroa.0.0.i310.6, 3, !dbg !4197
  %_31.i312.6 = or disjoint i64 %_32.i311.6, 6, !dbg !4197
  %_34.i314.6 = icmp ult i64 %_31.i312.6, %_56.1.i313, !dbg !4198
  br i1 %_34.i314.6, label %bb12.i304.7, label %panic2.i315, !dbg !4198

bb12.i304.7:                                      ; preds = %bb16.i316.5
  %_25.i305.7 = load i32, ptr %350, align 4, !dbg !4194, !alias.scope !4183, !noalias !4190, !noundef !11
  %delay1.i306.7 = zext i32 %_25.i305.7 to i64, !dbg !4194
  %_28.not.i307.7 = icmp ult i32 %_39.i, %_25.i305.7, !dbg !4195
  %_29.i308.7 = select i1 %_28.not.i307.7, i64 %ring_length.i280, i64 0, !dbg !4195
  %write.pn7.i309.7 = sub nsw i64 %write.i, %delay1.i306.7, !dbg !4195
  %tap.sroa.0.0.i310.7 = add nsw i64 %write.pn7.i309.7, %_29.i308.7, !dbg !4196
  %_32.i311.7 = shl nsw i64 %tap.sroa.0.0.i310.7, 3, !dbg !4197
  %_31.i312.7 = or disjoint i64 %_32.i311.7, 7, !dbg !4197
  %_34.i314.7 = icmp ult i64 %_31.i312.7, %_56.1.i313, !dbg !4198
  br i1 %_34.i314.7, label %bb11.i319, label %panic2.i315, !dbg !4198

panic2.i315:                                      ; preds = %bb12.i304.7, %bb16.i316.5, %bb16.i316.4, %bb16.i316.3, %bb16.i316.2, %bb16.i316.1, %bb16.i316, %bb7.i299.preheader
  %_31.i312.lcssa = phi i64 [ %_32.i311, %bb7.i299.preheader ], [ %_31.i312.1, %bb16.i316 ], [ %_31.i312.2, %bb16.i316.1 ], [ %_31.i312.3, %bb16.i316.2 ], [ %_31.i312.4, %bb16.i316.3 ], [ %_31.i312.5, %bb16.i316.4 ], [ %_31.i312.6, %bb16.i316.5 ], [ %_31.i312.7, %bb12.i304.7 ], !dbg !4197
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_31.i312.lcssa, i64 noundef %_56.1.i313, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c4f76d0874ad32f224dcaa5d3453a9e1) #23, !dbg !4198, !noalias !4232
  unreachable, !dbg !4198

_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit320: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1850, %bb11.i319
  %detected_left.i.sroa.0.0 = phi <8 x float> [ %lanes.i1837.sroa.0.28.vec.insert, %bb11.i319 ], [ %lanes.i1844.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1850 ], !dbg !4238
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4241), !dbg !4244
  %_6.i236 = load i32, ptr %334, align 4, !dbg !4246, !alias.scope !4241, !noalias !4248, !noundef !11
  %ring_length.i237 = zext i32 %_6.i236 to i64, !dbg !4246
  br i1 %or.cond7224.not, label %bb7.i256.preheader, label %bb1.i238, !dbg !4251

bb7.i256.preheader:                               ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit320
  %_56.1.i270 = load i64, ptr %332, align 8
  %_56.0.i274 = load ptr, ptr %333, align 16, !nonnull !11
  %_25.i262 = load i32, ptr %310, align 4, !dbg !4252, !alias.scope !4241, !noalias !4248, !noundef !11
  %delay1.i263 = zext i32 %_25.i262 to i64, !dbg !4252
  %_28.not.i264 = icmp ult i32 %_39.i, %_25.i262, !dbg !4253
  %_29.i265 = select i1 %_28.not.i264, i64 %ring_length.i237, i64 0, !dbg !4253
  %write.pn7.i266 = sub nsw i64 %write.i, %delay1.i263, !dbg !4253
  %tap.sroa.0.0.i267 = add nsw i64 %write.pn7.i266, %_29.i265, !dbg !4254
  %_32.i268 = shl nsw i64 %tap.sroa.0.0.i267, 3, !dbg !4255
  %_34.i271 = icmp ult i64 %_32.i268, %_56.1.i270, !dbg !4256
  br i1 %_34.i271, label %bb16.i273, label %panic2.i272, !dbg !4256

bb1.i238:                                         ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit320
  %_10.not.i239 = icmp ult i32 %_39.i, %_101.i, !dbg !4257
  %_11.i240 = select i1 %_10.not.i239, i64 %ring_length.i237, i64 0, !dbg !4257
  %write.pn.i241 = sub nsw i64 %write.i, %_100.i, !dbg !4257
  %row.sroa.0.0.i242 = add nsw i64 %write.pn.i241, %_11.i240, !dbg !4258
  %_55.1.i243 = load i64, ptr %332, align 8, !dbg !4259, !alias.scope !4241, !noalias !4248, !noundef !11
  %_13.i244 = shl nsw i64 %row.sroa.0.0.i242, 3, !dbg !4260
  %_40.i245 = icmp ugt i64 %_13.i244, %_55.1.i243, !dbg !4261
  br i1 %_40.i245, label %bb19.i250, label %bb20.i246, !dbg !4261, !prof !161

bb20.i246:                                        ; preds = %bb1.i238
  %_43.i248 = sub nuw i64 %_55.1.i243, %_13.i244, !dbg !4264
  %_8.i1862 = icmp samesign ugt i64 %_43.i248, 7, !dbg !4265
  br i1 %_8.i1862, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1866, label %bb2.i1863, !dbg !4265, !prof !2135

bb2.i1863:                                        ; preds = %bb20.i246
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_43.i248, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !4270, !noalias !4271
  unreachable, !dbg !4270

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1866: ; preds = %bb20.i246
  %_55.0.i247 = load ptr, ptr %333, align 16, !dbg !4259, !alias.scope !4241, !noalias !4248, !nonnull !11, !noundef !11
  %_47.i249 = getelementptr inbounds nuw float, ptr %_55.0.i247, i64 %_13.i244, !dbg !4275
  %lanes.i1859.sroa.0.0.copyload = load <8 x float>, ptr %_47.i249, align 4, !dbg !4277, !alias.scope !4281, !noalias !4285
  br label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit520, !dbg !4287

bb19.i250:                                        ; preds = %bb1.i238
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_13.i244, i64 noundef %_55.1.i243, i64 noundef %_55.1.i243, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_942b76445104c042a726096e85fe3fcb) #23, !dbg !4288, !noalias !4289
  unreachable, !dbg !4288

bb11.i276:                                        ; preds = %bb12.i261.7
  %378 = getelementptr inbounds nuw float, ptr %_56.0.i274, i64 %_31.i269.6, !dbg !4256
  %_30.i275.6 = load float, ptr %378, align 4, !dbg !4256, !noalias !4290, !noundef !11
  %379 = getelementptr inbounds nuw float, ptr %_56.0.i274, i64 %_31.i269.7, !dbg !4256
  %_30.i275.7 = load float, ptr %379, align 4, !dbg !4256, !noalias !4290, !noundef !11
  %lanes.i1852.sroa.0.0.vec.insert = insertelement <8 x float> poison, float %_30.i275, i64 0, !dbg !4291
  %lanes.i1852.sroa.0.4.vec.insert = insertelement <8 x float> %lanes.i1852.sroa.0.0.vec.insert, float %_30.i275.1, i64 1, !dbg !4291
  %lanes.i1852.sroa.0.8.vec.insert = insertelement <8 x float> %lanes.i1852.sroa.0.4.vec.insert, float %_30.i275.2, i64 2, !dbg !4291
  %lanes.i1852.sroa.0.12.vec.insert = insertelement <8 x float> %lanes.i1852.sroa.0.8.vec.insert, float %_30.i275.3, i64 3, !dbg !4291
  %lanes.i1852.sroa.0.16.vec.insert = insertelement <8 x float> %lanes.i1852.sroa.0.12.vec.insert, float %_30.i275.4, i64 4, !dbg !4291
  %lanes.i1852.sroa.0.20.vec.insert = insertelement <8 x float> %lanes.i1852.sroa.0.16.vec.insert, float %_30.i275.5, i64 5, !dbg !4291
  %lanes.i1852.sroa.0.24.vec.insert = insertelement <8 x float> %lanes.i1852.sroa.0.20.vec.insert, float %_30.i275.6, i64 6, !dbg !4291
  %lanes.i1852.sroa.0.28.vec.insert = insertelement <8 x float> %lanes.i1852.sroa.0.24.vec.insert, float %_30.i275.7, i64 7, !dbg !4291
  br label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit520, !dbg !4287

bb16.i273:                                        ; preds = %bb7.i256.preheader
  %380 = getelementptr inbounds nuw float, ptr %_56.0.i274, i64 %_32.i268, !dbg !4256
  %_30.i275 = load float, ptr %380, align 4, !dbg !4256, !noalias !4290, !noundef !11
  %_25.i262.1 = load i32, ptr %311, align 4, !dbg !4252, !alias.scope !4241, !noalias !4248, !noundef !11
  %delay1.i263.1 = zext i32 %_25.i262.1 to i64, !dbg !4252
  %_28.not.i264.1 = icmp ult i32 %_39.i, %_25.i262.1, !dbg !4253
  %_29.i265.1 = select i1 %_28.not.i264.1, i64 %ring_length.i237, i64 0, !dbg !4253
  %write.pn7.i266.1 = sub nsw i64 %write.i, %delay1.i263.1, !dbg !4253
  %tap.sroa.0.0.i267.1 = add nsw i64 %write.pn7.i266.1, %_29.i265.1, !dbg !4254
  %_32.i268.1 = shl nsw i64 %tap.sroa.0.0.i267.1, 3, !dbg !4255
  %_31.i269.1 = or disjoint i64 %_32.i268.1, 1, !dbg !4255
  %_34.i271.1 = icmp ult i64 %_31.i269.1, %_56.1.i270, !dbg !4256
  br i1 %_34.i271.1, label %bb16.i273.1, label %panic2.i272, !dbg !4256

bb16.i273.1:                                      ; preds = %bb16.i273
  %381 = getelementptr inbounds nuw float, ptr %_56.0.i274, i64 %_31.i269.1, !dbg !4256
  %_30.i275.1 = load float, ptr %381, align 4, !dbg !4256, !noalias !4290, !noundef !11
  %_25.i262.2 = load i32, ptr %351, align 4, !dbg !4252, !alias.scope !4241, !noalias !4248, !noundef !11
  %delay1.i263.2 = zext i32 %_25.i262.2 to i64, !dbg !4252
  %_28.not.i264.2 = icmp ult i32 %_39.i, %_25.i262.2, !dbg !4253
  %_29.i265.2 = select i1 %_28.not.i264.2, i64 %ring_length.i237, i64 0, !dbg !4253
  %write.pn7.i266.2 = sub nsw i64 %write.i, %delay1.i263.2, !dbg !4253
  %tap.sroa.0.0.i267.2 = add nsw i64 %write.pn7.i266.2, %_29.i265.2, !dbg !4254
  %_32.i268.2 = shl nsw i64 %tap.sroa.0.0.i267.2, 3, !dbg !4255
  %_31.i269.2 = or disjoint i64 %_32.i268.2, 2, !dbg !4255
  %_34.i271.2 = icmp ult i64 %_31.i269.2, %_56.1.i270, !dbg !4256
  br i1 %_34.i271.2, label %bb16.i273.2, label %panic2.i272, !dbg !4256

bb16.i273.2:                                      ; preds = %bb16.i273.1
  %382 = getelementptr inbounds nuw float, ptr %_56.0.i274, i64 %_31.i269.2, !dbg !4256
  %_30.i275.2 = load float, ptr %382, align 4, !dbg !4256, !noalias !4290, !noundef !11
  %_25.i262.3 = load i32, ptr %352, align 4, !dbg !4252, !alias.scope !4241, !noalias !4248, !noundef !11
  %delay1.i263.3 = zext i32 %_25.i262.3 to i64, !dbg !4252
  %_28.not.i264.3 = icmp ult i32 %_39.i, %_25.i262.3, !dbg !4253
  %_29.i265.3 = select i1 %_28.not.i264.3, i64 %ring_length.i237, i64 0, !dbg !4253
  %write.pn7.i266.3 = sub nsw i64 %write.i, %delay1.i263.3, !dbg !4253
  %tap.sroa.0.0.i267.3 = add nsw i64 %write.pn7.i266.3, %_29.i265.3, !dbg !4254
  %_32.i268.3 = shl nsw i64 %tap.sroa.0.0.i267.3, 3, !dbg !4255
  %_31.i269.3 = or disjoint i64 %_32.i268.3, 3, !dbg !4255
  %_34.i271.3 = icmp ult i64 %_31.i269.3, %_56.1.i270, !dbg !4256
  br i1 %_34.i271.3, label %bb16.i273.3, label %panic2.i272, !dbg !4256

bb16.i273.3:                                      ; preds = %bb16.i273.2
  %383 = getelementptr inbounds nuw float, ptr %_56.0.i274, i64 %_31.i269.3, !dbg !4256
  %_30.i275.3 = load float, ptr %383, align 4, !dbg !4256, !noalias !4290, !noundef !11
  %_25.i262.4 = load i32, ptr %353, align 4, !dbg !4252, !alias.scope !4241, !noalias !4248, !noundef !11
  %delay1.i263.4 = zext i32 %_25.i262.4 to i64, !dbg !4252
  %_28.not.i264.4 = icmp ult i32 %_39.i, %_25.i262.4, !dbg !4253
  %_29.i265.4 = select i1 %_28.not.i264.4, i64 %ring_length.i237, i64 0, !dbg !4253
  %write.pn7.i266.4 = sub nsw i64 %write.i, %delay1.i263.4, !dbg !4253
  %tap.sroa.0.0.i267.4 = add nsw i64 %write.pn7.i266.4, %_29.i265.4, !dbg !4254
  %_32.i268.4 = shl nsw i64 %tap.sroa.0.0.i267.4, 3, !dbg !4255
  %_31.i269.4 = or disjoint i64 %_32.i268.4, 4, !dbg !4255
  %_34.i271.4 = icmp ult i64 %_31.i269.4, %_56.1.i270, !dbg !4256
  br i1 %_34.i271.4, label %bb16.i273.4, label %panic2.i272, !dbg !4256

bb16.i273.4:                                      ; preds = %bb16.i273.3
  %384 = getelementptr inbounds nuw float, ptr %_56.0.i274, i64 %_31.i269.4, !dbg !4256
  %_30.i275.4 = load float, ptr %384, align 4, !dbg !4256, !noalias !4290, !noundef !11
  %_25.i262.5 = load i32, ptr %354, align 4, !dbg !4252, !alias.scope !4241, !noalias !4248, !noundef !11
  %delay1.i263.5 = zext i32 %_25.i262.5 to i64, !dbg !4252
  %_28.not.i264.5 = icmp ult i32 %_39.i, %_25.i262.5, !dbg !4253
  %_29.i265.5 = select i1 %_28.not.i264.5, i64 %ring_length.i237, i64 0, !dbg !4253
  %write.pn7.i266.5 = sub nsw i64 %write.i, %delay1.i263.5, !dbg !4253
  %tap.sroa.0.0.i267.5 = add nsw i64 %write.pn7.i266.5, %_29.i265.5, !dbg !4254
  %_32.i268.5 = shl nsw i64 %tap.sroa.0.0.i267.5, 3, !dbg !4255
  %_31.i269.5 = or disjoint i64 %_32.i268.5, 5, !dbg !4255
  %_34.i271.5 = icmp ult i64 %_31.i269.5, %_56.1.i270, !dbg !4256
  br i1 %_34.i271.5, label %bb16.i273.5, label %panic2.i272, !dbg !4256

bb16.i273.5:                                      ; preds = %bb16.i273.4
  %385 = getelementptr inbounds nuw float, ptr %_56.0.i274, i64 %_31.i269.5, !dbg !4256
  %_30.i275.5 = load float, ptr %385, align 4, !dbg !4256, !noalias !4290, !noundef !11
  %_25.i262.6 = load i32, ptr %355, align 4, !dbg !4252, !alias.scope !4241, !noalias !4248, !noundef !11
  %delay1.i263.6 = zext i32 %_25.i262.6 to i64, !dbg !4252
  %_28.not.i264.6 = icmp ult i32 %_39.i, %_25.i262.6, !dbg !4253
  %_29.i265.6 = select i1 %_28.not.i264.6, i64 %ring_length.i237, i64 0, !dbg !4253
  %write.pn7.i266.6 = sub nsw i64 %write.i, %delay1.i263.6, !dbg !4253
  %tap.sroa.0.0.i267.6 = add nsw i64 %write.pn7.i266.6, %_29.i265.6, !dbg !4254
  %_32.i268.6 = shl nsw i64 %tap.sroa.0.0.i267.6, 3, !dbg !4255
  %_31.i269.6 = or disjoint i64 %_32.i268.6, 6, !dbg !4255
  %_34.i271.6 = icmp ult i64 %_31.i269.6, %_56.1.i270, !dbg !4256
  br i1 %_34.i271.6, label %bb12.i261.7, label %panic2.i272, !dbg !4256

bb12.i261.7:                                      ; preds = %bb16.i273.5
  %_25.i262.7 = load i32, ptr %356, align 4, !dbg !4252, !alias.scope !4241, !noalias !4248, !noundef !11
  %delay1.i263.7 = zext i32 %_25.i262.7 to i64, !dbg !4252
  %_28.not.i264.7 = icmp ult i32 %_39.i, %_25.i262.7, !dbg !4253
  %_29.i265.7 = select i1 %_28.not.i264.7, i64 %ring_length.i237, i64 0, !dbg !4253
  %write.pn7.i266.7 = sub nsw i64 %write.i, %delay1.i263.7, !dbg !4253
  %tap.sroa.0.0.i267.7 = add nsw i64 %write.pn7.i266.7, %_29.i265.7, !dbg !4254
  %_32.i268.7 = shl nsw i64 %tap.sroa.0.0.i267.7, 3, !dbg !4255
  %_31.i269.7 = or disjoint i64 %_32.i268.7, 7, !dbg !4255
  %_34.i271.7 = icmp ult i64 %_31.i269.7, %_56.1.i270, !dbg !4256
  br i1 %_34.i271.7, label %bb11.i276, label %panic2.i272, !dbg !4256

panic2.i272:                                      ; preds = %bb12.i261.7, %bb16.i273.5, %bb16.i273.4, %bb16.i273.3, %bb16.i273.2, %bb16.i273.1, %bb16.i273, %bb7.i256.preheader
  %_31.i269.lcssa = phi i64 [ %_32.i268, %bb7.i256.preheader ], [ %_31.i269.1, %bb16.i273 ], [ %_31.i269.2, %bb16.i273.1 ], [ %_31.i269.3, %bb16.i273.2 ], [ %_31.i269.4, %bb16.i273.3 ], [ %_31.i269.5, %bb16.i273.4 ], [ %_31.i269.6, %bb16.i273.5 ], [ %_31.i269.7, %bb12.i261.7 ], !dbg !4255
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_31.i269.lcssa, i64 noundef %_56.1.i270, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c4f76d0874ad32f224dcaa5d3453a9e1) #23, !dbg !4256, !noalias !4290
  unreachable, !dbg !4256

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit520: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1866, %bb11.i276
  %detected_right.i.sroa.0.0 = phi <8 x float> [ %lanes.i1852.sroa.0.28.vec.insert, %bb11.i276 ], [ %lanes.i1859.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1866 ], !dbg !4296
  %386 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %detected_left.i.sroa.0.0, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !4299
  %387 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %386, <8 x float> splat (float 0x3810000000000000)), !dbg !4308
  %388 = bitcast <8 x float> %387 to <4 x i64>, !dbg !4315
  %389 = and <4 x i64> %388, splat (i64 36028792732385279), !dbg !4316
  %390 = or disjoint <4 x i64> %389, splat (i64 4575657222473777152), !dbg !4321
  %391 = bitcast <4 x i64> %390 to <8 x float>, !dbg !4325
  %392 = fadd <8 x float> %391, splat (float -1.000000e+00), !dbg !4326
  %393 = fmul <8 x float> %392, splat (float 0xBF9B17A960000000), !dbg !4331
  %394 = fadd <8 x float> %393, splat (float 0x3FBF9A8440000000), !dbg !4336
  %395 = fmul <8 x float> %392, %394, !dbg !4331
  %396 = fadd <8 x float> %395, splat (float 0xBFD1E3F400000000), !dbg !4336
  %397 = fmul <8 x float> %392, %396, !dbg !4331
  %398 = fadd <8 x float> %397, splat (float 0x3FDD544F20000000), !dbg !4336
  %399 = fmul <8 x float> %392, %398, !dbg !4331
  %400 = fadd <8 x float> %399, splat (float 0xBFE6FC2A60000000), !dbg !4336
  %401 = fmul <8 x float> %392, %400, !dbg !4331
  %402 = fadd <8 x float> %401, splat (float 0x3FF714B2A0000000), !dbg !4336
  %403 = bitcast <8 x float> %387 to <8 x i32>, !dbg !4341
  %_3.i2797 = lshr <8 x i32> %403, splat (i32 23), !dbg !4345
  %404 = or disjoint <8 x i32> %_3.i2797, splat (i32 1258291200), !dbg !4346
  %405 = bitcast <8 x i32> %404 to <8 x float>, !dbg !4350
  %406 = fadd <8 x float> %405, splat (float 0xC160000FE0000000), !dbg !4351
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4355), !dbg !4358
  %_6.i426.sroa.0.0.copyload = load <8 x float>, ptr %_73.i, align 32, !dbg !4360, !noalias !4362
  %407 = fmul <8 x float> %392, %402, !dbg !4365
  %408 = fadd <8 x float> %406, %407, !dbg !4370
  %409 = fmul <8 x float> %408, splat (float 0x4018151820000000), !dbg !4375
  %410 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %409, <8 x float> splat (float -1.600000e+02)), !dbg !4380
  %411 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %410, <8 x float> splat (float 2.400000e+01)), !dbg !4385
  %412 = fsub <8 x float> %411, %lanes.i1536.sroa.0.0.copyload, !dbg !4390
  %413 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %412, <8 x float> %lanes.i1522.sroa.0.0.copyload, i8 30), !dbg !4396
  %414 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %412, <8 x float> %357, i8 18), !dbg !4402
  %415 = fadd <8 x float> %lanes.i1522.sroa.0.0.copyload, %412, !dbg !4408
  %416 = fmul <8 x float> %415, %415, !dbg !4413
  %417 = fmul <8 x float> %lanes.i1515.sroa.0.0.copyload, %416, !dbg !4418
  %418 = bitcast <8 x float> %413 to <8 x i32>, !dbg !4423
  %419 = icmp slt <8 x i32> %418, zeroinitializer, !dbg !4427
  %.v4154 = select <8 x i1> %419, <8 x float> %412, <8 x float> %417, !dbg !4427
  %420 = fmul <8 x float> %lanes.i1529.sroa.0.0.copyload, %.v4154, !dbg !4427
  %421 = bitcast <8 x float> %414 to <8 x i32>, !dbg !4429
  %422 = icmp slt <8 x i32> %421, zeroinitializer, !dbg !4433
  %423 = select <8 x i1> %422, <8 x float> zeroinitializer, <8 x float> %420, !dbg !4433
  %424 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %423, <8 x float> splat (float -1.000000e+02)), !dbg !4435
  %425 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %424, <8 x float> zeroinitializer), !dbg !4440
  %426 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %425, <8 x float> %_6.i426.sroa.0.0.copyload, i8 17), !dbg !4445
  %427 = bitcast <8 x float> %426 to <8 x i32>, !dbg !4451
  %428 = icmp slt <8 x i32> %427, zeroinitializer, !dbg !4455
  %429 = select <8 x i1> %428, <8 x float> %lanes.i1508.sroa.0.0.copyload, <8 x float> %lanes.i.sroa.0.0.copyload, !dbg !4455
  %430 = fsub <8 x float> %425, %_6.i426.sroa.0.0.copyload, !dbg !4457
  %431 = fmul <8 x float> %430, %429, !dbg !4463
  %432 = fadd <8 x float> %_6.i426.sroa.0.0.copyload, %431, !dbg !4468
  %433 = bitcast <8 x float> %432 to <8 x i32>, !dbg !4472
  %434 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %432), !dbg !4478
  %435 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %434, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !4480
  %436 = bitcast <8 x float> %435 to <8 x i32>, !dbg !4486
  %437 = xor <8 x i32> %436, splat (i32 -1), !dbg !4492
  %438 = and <8 x i32> %437, %433, !dbg !4494
  store <8 x i32> %438, ptr %_73.i, align 32, !dbg !4498, !alias.scope !4499, !noalias !4501
  %439 = bitcast <8 x i32> %438 to <8 x float>, !dbg !4502
  %440 = fadd <8 x float> %lanes.i1550.sroa.0.0.copyload, %439, !dbg !4503
  %441 = fmul <8 x float> %440, splat (float 0x3FC542A5A0000000), !dbg !4510
  %442 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %441, <8 x float> splat (float -1.260000e+02)), !dbg !4516
  %443 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %442, <8 x float> splat (float 1.270000e+02)), !dbg !4522
  %444 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %443), !dbg !4527
  %445 = fsub <8 x float> %443, %444, !dbg !4532
  %446 = fmul <8 x float> %445, splat (float 0x3F5E974FA0000000), !dbg !4537
  %447 = fadd <8 x float> %446, splat (float 0x3F82778560000000), !dbg !4542
  %448 = fmul <8 x float> %445, %447, !dbg !4537
  %449 = fadd <8 x float> %448, splat (float 0x3FAC91CE60000000), !dbg !4542
  %450 = fmul <8 x float> %445, %449, !dbg !4537
  %451 = fadd <8 x float> %450, splat (float 0x3FCEBDB560000000), !dbg !4542
  %452 = fmul <8 x float> %445, %451, !dbg !4537
  %453 = fadd <8 x float> %452, splat (float 0x3FE62E4BA0000000), !dbg !4542
  %454 = fmul <8 x float> %445, %453, !dbg !4547
  %455 = fadd <8 x float> %454, splat (float 1.000000e+00), !dbg !4552
  %456 = fadd <8 x float> %444, splat (float 0x4160000FE0000000), !dbg !4557
  %457 = bitcast <8 x float> %456 to <8 x i32>, !dbg !4562
  %_3.i2798 = shl <8 x i32> %457, splat (i32 23), !dbg !4566
  %458 = bitcast <8 x i32> %_3.i2798 to <8 x float>, !dbg !4567
  %459 = fmul <8 x float> %455, %458, !dbg !4569
  %460 = fmul <8 x float> %lanes.i1963.sroa.0.0.copyload, %459, !dbg !4573
  %461 = fsub <8 x float> %460, %lanes.i1963.sroa.0.0.copyload, !dbg !4578
  %462 = fmul <8 x float> %lanes.i1543.sroa.0.0.copyload, %461, !dbg !4584
  %463 = fadd <8 x float> %lanes.i1963.sroa.0.0.copyload, %462, !dbg !4589
  %464 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %439, <8 x float> zeroinitializer, i8 0), !dbg !4593
  %465 = bitcast <8 x float> %464 to <8 x i32>, !dbg !4599
  %466 = and <8 x i32> %465, %335, !dbg !4603
  %467 = or <8 x i32> %466, %336
  %.reass = or <8 x i32> %467, %337
  %468 = select <8 x i1> %339, <8 x float> %460, <8 x float> %463, !dbg !4605
  %469 = icmp slt <8 x i32> %.reass, zeroinitializer, !dbg !4610
  %470 = select <8 x i1> %469, <8 x float> %lanes.i1963.sroa.0.0.copyload, <8 x float> %468, !dbg !4610
  %471 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %detected_right.i.sroa.0.0, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !4615
  %472 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %471, <8 x float> splat (float 0x3810000000000000)), !dbg !4623
  %473 = bitcast <8 x float> %472 to <4 x i64>, !dbg !4630
  %474 = and <4 x i64> %473, splat (i64 36028792732385279), !dbg !4631
  %475 = or disjoint <4 x i64> %474, splat (i64 4575657222473777152), !dbg !4636
  %476 = bitcast <4 x i64> %475 to <8 x float>, !dbg !4640
  %477 = fadd <8 x float> %476, splat (float -1.000000e+00), !dbg !4641
  %478 = fmul <8 x float> %477, splat (float 0xBF9B17A960000000), !dbg !4646
  %479 = fadd <8 x float> %478, splat (float 0x3FBF9A8440000000), !dbg !4651
  %480 = fmul <8 x float> %477, %479, !dbg !4646
  %481 = fadd <8 x float> %480, splat (float 0xBFD1E3F400000000), !dbg !4651
  %482 = fmul <8 x float> %477, %481, !dbg !4646
  %483 = fadd <8 x float> %482, splat (float 0x3FDD544F20000000), !dbg !4651
  %484 = fmul <8 x float> %477, %483, !dbg !4646
  %485 = fadd <8 x float> %484, splat (float 0xBFE6FC2A60000000), !dbg !4651
  %486 = fmul <8 x float> %477, %485, !dbg !4646
  %487 = fadd <8 x float> %486, splat (float 0x3FF714B2A0000000), !dbg !4651
  %488 = bitcast <8 x float> %472 to <8 x i32>, !dbg !4656
  %_3.i2799 = lshr <8 x i32> %488, splat (i32 23), !dbg !4660
  %489 = or disjoint <8 x i32> %_3.i2799, splat (i32 1258291200), !dbg !4661
  %490 = bitcast <8 x i32> %489 to <8 x float>, !dbg !4665
  %491 = fadd <8 x float> %490, splat (float 0xC160000FE0000000), !dbg !4666
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4670), !dbg !4673
  %_6.i415.sroa.0.0.copyload = load <8 x float>, ptr %_77.i, align 32, !dbg !4675, !noalias !4677
  %492 = fmul <8 x float> %477, %487, !dbg !4680
  %493 = fadd <8 x float> %491, %492, !dbg !4685
  %494 = fmul <8 x float> %493, splat (float 0x4018151820000000), !dbg !4690
  %495 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %494, <8 x float> splat (float -1.600000e+02)), !dbg !4695
  %496 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %495, <8 x float> splat (float 2.400000e+01)), !dbg !4700
  %497 = fsub <8 x float> %496, %lanes.i1592.sroa.0.0.copyload, !dbg !4705
  %498 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %497, <8 x float> %lanes.i1578.sroa.0.0.copyload, i8 30), !dbg !4711
  %499 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %497, <8 x float> %358, i8 18), !dbg !4717
  %500 = fadd <8 x float> %lanes.i1578.sroa.0.0.copyload, %497, !dbg !4723
  %501 = fmul <8 x float> %500, %500, !dbg !4728
  %502 = fmul <8 x float> %lanes.i1571.sroa.0.0.copyload, %501, !dbg !4733
  %503 = bitcast <8 x float> %498 to <8 x i32>, !dbg !4738
  %504 = icmp slt <8 x i32> %503, zeroinitializer, !dbg !4742
  %.v4160 = select <8 x i1> %504, <8 x float> %497, <8 x float> %502, !dbg !4742
  %505 = fmul <8 x float> %lanes.i1585.sroa.0.0.copyload, %.v4160, !dbg !4742
  %506 = bitcast <8 x float> %499 to <8 x i32>, !dbg !4744
  %507 = icmp slt <8 x i32> %506, zeroinitializer, !dbg !4748
  %508 = select <8 x i1> %507, <8 x float> zeroinitializer, <8 x float> %505, !dbg !4748
  %509 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %508, <8 x float> splat (float -1.000000e+02)), !dbg !4750
  %510 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %509, <8 x float> zeroinitializer), !dbg !4755
  %511 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %510, <8 x float> %_6.i415.sroa.0.0.copyload, i8 17), !dbg !4760
  %512 = bitcast <8 x float> %511 to <8 x i32>, !dbg !4766
  %513 = icmp slt <8 x i32> %512, zeroinitializer, !dbg !4770
  %514 = select <8 x i1> %513, <8 x float> %lanes.i1564.sroa.0.0.copyload, <8 x float> %lanes.i1557.sroa.0.0.copyload, !dbg !4770
  %515 = fsub <8 x float> %510, %_6.i415.sroa.0.0.copyload, !dbg !4772
  %516 = fmul <8 x float> %515, %514, !dbg !4778
  %517 = fadd <8 x float> %_6.i415.sroa.0.0.copyload, %516, !dbg !4783
  %518 = bitcast <8 x float> %517 to <8 x i32>, !dbg !4787
  %519 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %517), !dbg !4793
  %520 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %519, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !4795
  %521 = bitcast <8 x float> %520 to <8 x i32>, !dbg !4801
  %522 = xor <8 x i32> %521, splat (i32 -1), !dbg !4807
  %523 = and <8 x i32> %522, %518, !dbg !4809
  store <8 x i32> %523, ptr %_77.i, align 32, !dbg !4813, !alias.scope !4814, !noalias !4816
  %524 = bitcast <8 x i32> %523 to <8 x float>, !dbg !4817
  %525 = fadd <8 x float> %lanes.i1606.sroa.0.0.copyload, %524, !dbg !4818
  %526 = fmul <8 x float> %525, splat (float 0x3FC542A5A0000000), !dbg !4825
  %527 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %526, <8 x float> splat (float -1.260000e+02)), !dbg !4831
  %528 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %527, <8 x float> splat (float 1.270000e+02)), !dbg !4837
  %529 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %528), !dbg !4842
  %530 = fsub <8 x float> %528, %529, !dbg !4847
  %531 = fmul <8 x float> %530, splat (float 0x3F5E974FA0000000), !dbg !4852
  %532 = fadd <8 x float> %531, splat (float 0x3F82778560000000), !dbg !4857
  %533 = fmul <8 x float> %530, %532, !dbg !4852
  %534 = fadd <8 x float> %533, splat (float 0x3FAC91CE60000000), !dbg !4857
  %535 = fmul <8 x float> %530, %534, !dbg !4852
  %536 = fadd <8 x float> %535, splat (float 0x3FCEBDB560000000), !dbg !4857
  %537 = fmul <8 x float> %530, %536, !dbg !4852
  %538 = fadd <8 x float> %537, splat (float 0x3FE62E4BA0000000), !dbg !4857
  %539 = fmul <8 x float> %530, %538, !dbg !4862
  %540 = fadd <8 x float> %539, splat (float 1.000000e+00), !dbg !4867
  %541 = fadd <8 x float> %529, splat (float 0x4160000FE0000000), !dbg !4872
  %542 = bitcast <8 x float> %541 to <8 x i32>, !dbg !4877
  %_3.i2800 = shl <8 x i32> %542, splat (i32 23), !dbg !4881
  %543 = bitcast <8 x i32> %_3.i2800 to <8 x float>, !dbg !4882
  %544 = fmul <8 x float> %540, %543, !dbg !4884
  %545 = fmul <8 x float> %lanes.i1954.sroa.0.0.copyload, %544, !dbg !4888
  %546 = fsub <8 x float> %545, %lanes.i1954.sroa.0.0.copyload, !dbg !4893
  %547 = fmul <8 x float> %lanes.i1599.sroa.0.0.copyload, %546, !dbg !4899
  %548 = fadd <8 x float> %lanes.i1954.sroa.0.0.copyload, %547, !dbg !4904
  %549 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %524, <8 x float> zeroinitializer, i8 0), !dbg !4908
  %550 = bitcast <8 x float> %549 to <8 x i32>, !dbg !4914
  %551 = and <8 x i32> %550, %340, !dbg !4918
  %552 = or <8 x i32> %551, %341
  %.reass7201 = or <8 x i32> %552, %337
  %553 = select <8 x i1> %343, <8 x float> %545, <8 x float> %548, !dbg !4920
  %554 = icmp slt <8 x i32> %.reass7201, zeroinitializer, !dbg !4925
  %555 = select <8 x i1> %554, <8 x float> %lanes.i1954.sroa.0.0.copyload, <8 x float> %553, !dbg !4925
  store <8 x float> %470, ptr %_120.i, align 4, !dbg !4930, !alias.scope !4936, !noalias !4940
  store <8 x float> %555, ptr %_128.i, align 4, !dbg !4944, !alias.scope !4949, !noalias !4953
  %556 = trunc i64 %spec.store.select.i to i32, !dbg !4957
  store i32 %556, ptr %325, align 32, !dbg !4957, !alias.scope !3462, !noalias !3472
  store i32 %556, ptr %344, align 32, !dbg !4958, !alias.scope !3466, !noalias !3776
  %exitcond5994.not = icmp eq i64 %359, %frames, !dbg !4959
  br i1 %exitcond5994.not, label %bb19, label %bb41.i, !dbg !3782

bb57.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1970
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_57.i, i64 noundef %_194.1.i, i64 noundef %_194.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b5bf0b379ba879ef3901108f927dd528) #23, !dbg !4963, !noalias !3839
  unreachable, !dbg !4963

bb13:                                             ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel20segment_is_stageableNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit
  %_50.0 = load ptr, ptr %staged, align 8, !dbg !4964, !nonnull !11, !noundef !11
  %557 = getelementptr inbounds nuw i8, ptr %staged, i64 8, !dbg !4964
  %_50.1 = load i64, ptr %557, align 8, !dbg !4964, !noundef !11
  %558 = getelementptr inbounds nuw i8, ptr %staged, i64 16, !dbg !4966
  %_51.0 = load ptr, ptr %558, align 8, !dbg !4966, !nonnull !11, !noundef !11
  %559 = getelementptr inbounds nuw i8, ptr %staged, i64 24, !dbg !4966
  %_51.1 = load i64, ptr %559, align 8, !dbg !4966, !noundef !11
  %560 = getelementptr inbounds nuw i8, ptr %staged, i64 32, !dbg !4967
  %_52.0 = load ptr, ptr %560, align 8, !dbg !4967, !nonnull !11, !noundef !11
  %561 = getelementptr inbounds nuw i8, ptr %staged, i64 40, !dbg !4967
  %_52.1 = load i64, ptr %561, align 8, !dbg !4967, !noundef !11
  %562 = getelementptr inbounds nuw i8, ptr %staged, i64 48, !dbg !4968
  %_53.0 = load ptr, ptr %562, align 8, !dbg !4968, !nonnull !11, !noundef !11
  %563 = getelementptr inbounds nuw i8, ptr %staged, i64 56, !dbg !4968
  %_53.1 = load i64, ptr %563, align 8, !dbg !4968, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4969), !dbg !4972
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4973), !dbg !4972
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4975), !dbg !4972
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4977), !dbg !4972
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4979), !dbg !4972
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4981), !dbg !4972
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4983), !dbg !4972
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4985), !dbg !4972
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4987), !dbg !4972
  %564 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1540, !dbg !4989
  %_15.i2801 = load i32, ptr %564, align 4, !dbg !4989, !alias.scope !4977, !noalias !4993, !noundef !11
  %ring_length.i2802 = zext i32 %_15.i2801 to i64, !dbg !4989
  %565 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !4994
  %566 = bitcast <8 x float> %565 to <8 x i32>, !dbg !5003
  %567 = xor <8 x i32> %566, splat (i32 -1), !dbg !5009
  %568 = bitcast <8 x i32> %567 to <8 x float>, !dbg !5003
  switch i32 %link, label %bb13.i183.i [
    i32 1, label %bb14.i184.i
    i32 3, label %bb14.i184.fold.split.i
  ], !dbg !5011

bb13.i183.i:                                      ; preds = %bb13
  br label %bb14.i184.i, !dbg !5012

bb14.i184.fold.split.i:                           ; preds = %bb13
  br label %bb14.i184.i, !dbg !5013

bb14.i184.i:                                      ; preds = %bb14.i184.fold.split.i, %bb13.i183.i, %bb13
  %_11.i176.sroa.0.01708.i = phi <8 x float> [ %568, %bb13 ], [ %565, %bb13.i183.i ], [ %565, %bb14.i184.fold.split.i ]
  %_13.i175.sroa.0.0.i = phi <8 x i32> [ %567, %bb13 ], [ %567, %bb13.i183.i ], [ %566, %bb14.i184.fold.split.i ], !dbg !5014
  %_4.i217.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 448, !dbg !5015
  %lanes.i588.sroa.0.0.copyload.i = load <8 x float>, ptr %_4.i217.i, align 32, !dbg !5018, !alias.scope !5023, !noalias !5027
  %_7.i218.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 480, !dbg !5033
  %lanes.i583.sroa.0.0.copyload.i = load <8 x float>, ptr %_7.i218.i, align 32, !dbg !5034, !alias.scope !5039, !noalias !5043
  %_19.i2803 = getelementptr inbounds nuw i8, ptr %channels.0, i64 256, !dbg !5047
  %lanes.i578.sroa.0.0.copyload.i = load <8 x float>, ptr %_19.i2803, align 32, !dbg !5048, !alias.scope !5053, !noalias !5057
  %_14.i219.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 288, !dbg !5061
  %lanes.i573.sroa.0.0.copyload.i = load <8 x float>, ptr %_14.i219.i, align 32, !dbg !5062, !alias.scope !5067, !noalias !5071
  %_17.i220.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 320, !dbg !5075
  %lanes.i568.sroa.0.0.copyload.i = load <8 x float>, ptr %_17.i220.i, align 32, !dbg !5076, !alias.scope !5081, !noalias !5085
  %_20.i221.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 352, !dbg !5089
  %lanes.i563.sroa.0.0.copyload.i = load <8 x float>, ptr %_20.i221.i, align 32, !dbg !5090, !alias.scope !5095, !noalias !5099
  %_23.i222.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 384, !dbg !5103
  %lanes.i558.sroa.0.0.copyload.i = load <8 x float>, ptr %_23.i222.i, align 32, !dbg !5104, !alias.scope !5109, !noalias !5113
  %_26.i223.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 416, !dbg !5117
  %lanes.i.sroa.0.0.copyload.i = load <8 x float>, ptr %_26.i223.i, align 32, !dbg !5118, !alias.scope !5123, !noalias !5127
  %_4.i196.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 448, !dbg !5131
  %lanes.i628.sroa.0.0.copyload.i = load <8 x float>, ptr %_4.i196.i, align 32, !dbg !5134, !alias.scope !5139, !noalias !5143
  %_7.i197.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 480, !dbg !5149
  %lanes.i623.sroa.0.0.copyload.i = load <8 x float>, ptr %_7.i197.i, align 32, !dbg !5150, !alias.scope !5155, !noalias !5159
  %_21.i2804 = getelementptr inbounds nuw i8, ptr %channels.1, i64 256, !dbg !5163
  %lanes.i618.sroa.0.0.copyload.i = load <8 x float>, ptr %_21.i2804, align 32, !dbg !5164, !alias.scope !5169, !noalias !5173
  %_14.i198.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 288, !dbg !5177
  %lanes.i613.sroa.0.0.copyload.i = load <8 x float>, ptr %_14.i198.i, align 32, !dbg !5178, !alias.scope !5183, !noalias !5187
  %_17.i199.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 320, !dbg !5191
  %lanes.i608.sroa.0.0.copyload.i = load <8 x float>, ptr %_17.i199.i, align 32, !dbg !5192, !alias.scope !5197, !noalias !5201
  %_20.i200.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 352, !dbg !5205
  %lanes.i603.sroa.0.0.copyload.i = load <8 x float>, ptr %_20.i200.i, align 32, !dbg !5206, !alias.scope !5211, !noalias !5215
  %_23.i201.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 384, !dbg !5219
  %lanes.i598.sroa.0.0.copyload.i = load <8 x float>, ptr %_23.i201.i, align 32, !dbg !5220, !alias.scope !5225, !noalias !5229
  %_26.i.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 416, !dbg !5233
  %lanes.i593.sroa.0.0.copyload.i = load <8 x float>, ptr %_26.i.i, align 32, !dbg !5234, !alias.scope !5239, !noalias !5243
  %569 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i583.sroa.0.0.copyload.i, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !5247
  %570 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i583.sroa.0.0.copyload.i, <8 x float> zeroinitializer, i8 0), !dbg !5253
  %571 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i588.sroa.0.0.copyload.i, <8 x float> zeroinitializer, i8 0), !dbg !5259
  %572 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i623.sroa.0.0.copyload.i, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !5265
  %573 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i623.sroa.0.0.copyload.i, <8 x float> zeroinitializer, i8 0), !dbg !5271
  %574 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i628.sroa.0.0.copyload.i, <8 x float> zeroinitializer, i8 0), !dbg !5277
  %575 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1536, !dbg !5283
  %_23.i2805 = load i32, ptr %575, align 32, !dbg !5283, !alias.scope !4977, !noalias !4993, !noundef !11
  %576 = zext i32 %_23.i2805 to i64, !dbg !5283
; call compressor::kernel::fill_taps::<wide::f32x8_::f32x8>
  tail call fastcc void @_RINvNtCse3bfmKSZS8Y_10compressor6kernel9fill_tapsNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_(ptr noalias noundef nonnull readonly align 32 captures(address, read_provenance) dereferenceable(1568) %channels.0, i64 noundef %576, i64 noundef %_34, ptr noalias noundef nonnull align 4 %_50.0, i64 noundef range(i64 0, 2305843009213693952) %_50.1) #22, !dbg !5285, !noalias !5287
; call compressor::kernel::fill_taps::<wide::f32x8_::f32x8>
  tail call fastcc void @_RINvNtCse3bfmKSZS8Y_10compressor6kernel9fill_tapsNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_(ptr noalias noundef nonnull readonly align 32 captures(address, read_provenance) dereferenceable(1568) %channels.1, i64 noundef %576, i64 noundef %_34, ptr noalias noundef nonnull align 4 %_51.0, i64 noundef range(i64 0, 2305843009213693952) %_51.1) #22, !dbg !5288, !noalias !5289
  %_34.i2806 = shl nuw nsw i64 %_34, 3, !dbg !5290
  %_161.not.i = icmp ugt i64 %_34.i2806, %_50.1
  br i1 %_161.not.i, label %bb53.i2835, label %bb51.i2807, !dbg !5292, !prof !239

bb53.i2835:                                       ; preds = %bb14.i184.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_34.i2806, i64 noundef range(i64 0, 2305843009213693952) %_50.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_0e330bd00bf71e8be75312f2965d63a1) #23, !dbg !5301, !noalias !5302
  unreachable, !dbg !5301

bb51.i2807:                                       ; preds = %bb14.i184.i
  %_169.not.i = icmp samesign ugt i64 %_34.i2806, %_51.1, !dbg !5303
  br i1 %_169.not.i, label %bb56.i2834, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipB3_ECse3bfmKSZS8Y_10compressor.exit.i, !dbg !5303, !prof !161

bb56.i2834:                                       ; preds = %bb51.i2807
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_34.i2806, i64 noundef range(i64 0, 2305843009213693952) %_51.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f7aa9e7391a43a547574d029ed2ff54d) #23, !dbg !5309, !noalias !5302
  unreachable, !dbg !5309

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipB3_ECse3bfmKSZS8Y_10compressor.exit.i: ; preds = %bb51.i2807
  %_176.not.i = icmp ugt i64 %_34, %_52.1
  br i1 %_176.not.i, label %bb58.i2833, label %bb57.i2808, !dbg !5310, !prof !239

bb58.i2833:                                       ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipB3_ECse3bfmKSZS8Y_10compressor.exit.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_34, i64 noundef range(i64 0, 288230376151711744) %_52.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1feb33c78e17db086948326d854ee987) #23, !dbg !5318, !noalias !5302
  unreachable, !dbg !5318

bb57.i2808:                                       ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipB3_ECse3bfmKSZS8Y_10compressor.exit.i
  %_191.not.i = icmp samesign ugt i64 %_34, %_53.1, !dbg !5319
  br i1 %_191.not.i, label %bb64.i, label %bb14.lr.ph.i, !dbg !5319, !prof !161

bb64.i:                                           ; preds = %bb57.i2808
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_34, i64 noundef range(i64 0, 288230376151711744) %_53.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6943f86fcff04ee5505532b3b35f50d3) #23, !dbg !5325, !noalias !5302
  unreachable, !dbg !5325

bb14.lr.ph.i:                                     ; preds = %bb57.i2808
  %_6.i.i2809 = load i64, ptr %detector, align 8, !range !220, !alias.scope !4975, !noalias !5326
  %577 = getelementptr inbounds nuw i8, ptr %detector, i64 16
  %sidechain_left.1.i.i2810 = load i64, ptr %577, align 8, !alias.scope !4975, !noalias !5326
  %578 = getelementptr inbounds nuw i8, ptr %detector, i64 24
  %sidechain_right.0.i.i2811 = load ptr, ptr %578, align 8, !alias.scope !4975, !noalias !5326, !nonnull !11, !align !3781
  %579 = getelementptr inbounds nuw i8, ptr %detector, i64 32
  %sidechain_right.1.i.i2812 = load i64, ptr %579, align 8, !alias.scope !4975, !noalias !5326
  %580 = getelementptr inbounds nuw i8, ptr %detector, i64 8
  %sidechain_left.0.i.i2813 = load ptr, ptr %580, align 8, !alias.scope !4975, !noalias !5326, !nonnull !11, !align !3781
  %581 = icmp slt <8 x i32> %_13.i175.sroa.0.0.i, zeroinitializer
  %582 = bitcast <8 x float> %_11.i176.sroa.0.01708.i to <8 x i32>
  %583 = icmp slt <8 x i32> %582, zeroinitializer
  %584 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1512
  %585 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1504
  %586 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1528
  %587 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1520
  %588 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1512
  %589 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1504
  %590 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1528
  %591 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1520
  %_335.1.i = load i64, ptr %584, align 8, !alias.scope !4977, !noalias !4993
  %_335.0.i = load ptr, ptr %585, align 32, !alias.scope !4977, !noalias !4993, !nonnull !11
  %_336.1.i = load i64, ptr %586, align 8, !alias.scope !4977, !noalias !4993
  %_336.0.i = load ptr, ptr %587, align 16, !alias.scope !4977, !noalias !4993, !nonnull !11
  %_337.1.i = load i64, ptr %588, align 8, !alias.scope !4979, !noalias !5327
  %_337.0.i = load ptr, ptr %589, align 32, !alias.scope !4979, !noalias !5327, !nonnull !11
  %_338.1.i = load i64, ptr %590, align 8, !alias.scope !4979, !noalias !5327
  %_338.0.i = load ptr, ptr %591, align 16, !alias.scope !4979, !noalias !5327, !nonnull !11
  %592 = fneg <8 x float> %lanes.i568.sroa.0.0.copyload.i
  %593 = fneg <8 x float> %lanes.i608.sroa.0.0.copyload.i
  br label %bb14.i2814, !dbg !5328

bb14.i2814:                                       ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_log2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit174.i, %bb14.lr.ph.i
  %head.sroa.0.01959.i = phi i64 [ %576, %bb14.lr.ph.i ], [ %spec.store.select.i2825, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_log2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit174.i ]
  %iter.sroa.33.01958.i = phi i64 [ 0, %bb14.lr.ph.i ], [ %_9.0.i.i, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_log2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit174.i ]
  %_9.0.i.i = add nuw nsw i64 %iter.sroa.33.01958.i, 1, !dbg !5336
  %start1.i.i.i.i.i.i.i.i.i = shl i64 %iter.sroa.33.01958.i, 3, !dbg !5339
  %data.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_50.0, i64 %start1.i.i.i.i.i.i.i.i.i, !dbg !5354
  %data.i5.i.i.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_51.0, i64 %start1.i.i.i.i.i.i.i.i.i, !dbg !5358
  %_3.i.i.i.i.i.i.i = getelementptr inbounds nuw %"wide::f32x8_::f32x8", ptr %_52.0, i64 %iter.sroa.33.01958.i, !dbg !5361
  %_3.i.i.i.i.i = getelementptr inbounds nuw %"wide::f32x8_::f32x8", ptr %_53.0, i64 %iter.sroa.33.01958.i, !dbg !5366
  %_51.i = add nuw nsw i64 %iter.sroa.33.01958.i, %spec.store.select, !dbg !5369
  %slot7.i = shl i64 %_51.i, 3, !dbg !5369
  %_205.i = icmp samesign ugt i64 %slot7.i, %left.1, !dbg !5371
  br i1 %_205.i, label %bb68.i, label %bb69.i, !dbg !5371, !prof !161

bb15.i:                                           ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_log2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit174.i
  %594 = trunc i64 %spec.store.select.i2825 to i32, !dbg !5378
  store i32 %594, ptr %575, align 32, !dbg !5378, !alias.scope !4977, !noalias !4993
  %595 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1536, !dbg !5379
  store i32 %594, ptr %595, align 32, !dbg !5379, !alias.scope !4979, !noalias !5327
  %596 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1472, !dbg !5380
  %gain_left.sroa.0.0.copyload.i = load <8 x float>, ptr %596, align 32, !dbg !5380, !alias.scope !4977, !noalias !4993
  %597 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1472, !dbg !5381
  %gain_right.sroa.0.0.copyload.i = load <8 x float>, ptr %597, align 32, !dbg !5381, !alias.scope !4979, !noalias !5327
  br label %bb36.i2826, !dbg !5383

bb36.i2826:                                       ; preds = %bb15.i, %bb36.i2826
  %gain_right.sroa.0.01963.i = phi <8 x float> [ %626, %bb36.i2826 ], [ %gain_right.sroa.0.0.copyload.i, %bb15.i ]
  %gain_left.sroa.0.01962.i = phi <8 x float> [ %612, %bb36.i2826 ], [ %gain_left.sroa.0.0.copyload.i, %bb15.i ]
  %iter2.sroa.8.01961.i = phi i64 [ %611, %bb36.i2826 ], [ 0, %bb15.i ]
  %_3.i.i.i = getelementptr inbounds nuw %"wide::f32x8_::f32x8", ptr %_52.0, i64 %iter2.sroa.8.01961.i, !dbg !5391
  %_117.sroa.0.0.copyload.i = load <8 x float>, ptr %_3.i.i.i, align 32, !dbg !5395, !alias.scope !4985, !noalias !5397
  %_3.i1.i.i = getelementptr inbounds nuw %"wide::f32x8_::f32x8", ptr %_53.0, i64 %iter2.sroa.8.01961.i, !dbg !5398
  %598 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_117.sroa.0.0.copyload.i, <8 x float> %gain_left.sroa.0.01962.i, i8 17), !dbg !5401
  %599 = bitcast <8 x float> %598 to <8 x i32>, !dbg !5408
  %600 = icmp slt <8 x i32> %599, zeroinitializer, !dbg !5412
  %601 = select <8 x i1> %600, <8 x float> %lanes.i558.sroa.0.0.copyload.i, <8 x float> %lanes.i.sroa.0.0.copyload.i, !dbg !5412
  %602 = fsub <8 x float> %_117.sroa.0.0.copyload.i, %gain_left.sroa.0.01962.i, !dbg !5414
  %603 = fmul <8 x float> %602, %601, !dbg !5420
  %604 = fadd <8 x float> %gain_left.sroa.0.01962.i, %603, !dbg !5425
  %605 = bitcast <8 x float> %604 to <8 x i32>, !dbg !5429
  %606 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %604), !dbg !5435
  %607 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %606, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !5437
  %608 = bitcast <8 x float> %607 to <8 x i32>, !dbg !5443
  %609 = xor <8 x i32> %608, splat (i32 -1), !dbg !5449
  %610 = and <8 x i32> %605, %609, !dbg !5451
  store <8 x i32> %610, ptr %_3.i.i.i, align 32, !dbg !5455, !alias.scope !4985, !noalias !5397
  %_121.sroa.0.0.copyload.i = load <8 x float>, ptr %_3.i1.i.i, align 32, !dbg !5456, !alias.scope !4987, !noalias !5457
  %611 = add nuw i64 %iter2.sroa.8.01961.i, 1, !dbg !5458
  %612 = bitcast <8 x i32> %610 to <8 x float>, !dbg !5459
  %613 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_121.sroa.0.0.copyload.i, <8 x float> %gain_right.sroa.0.01963.i, i8 17), !dbg !5460
  %614 = bitcast <8 x float> %613 to <8 x i32>, !dbg !5467
  %615 = icmp slt <8 x i32> %614, zeroinitializer, !dbg !5471
  %616 = select <8 x i1> %615, <8 x float> %lanes.i598.sroa.0.0.copyload.i, <8 x float> %lanes.i593.sroa.0.0.copyload.i, !dbg !5471
  %617 = fsub <8 x float> %_121.sroa.0.0.copyload.i, %gain_right.sroa.0.01963.i, !dbg !5473
  %618 = fmul <8 x float> %617, %616, !dbg !5479
  %619 = fadd <8 x float> %gain_right.sroa.0.01963.i, %618, !dbg !5484
  %620 = bitcast <8 x float> %619 to <8 x i32>, !dbg !5488
  %621 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %619), !dbg !5494
  %622 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %621, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !5496
  %623 = bitcast <8 x float> %622 to <8 x i32>, !dbg !5502
  %624 = xor <8 x i32> %623, splat (i32 -1), !dbg !5508
  %625 = and <8 x i32> %620, %624, !dbg !5510
  %626 = bitcast <8 x i32> %625 to <8 x float>, !dbg !5514
  store <8 x i32> %625, ptr %_3.i1.i.i, align 32, !dbg !5515, !alias.scope !4987, !noalias !5457
  %exitcond2194.not.i = icmp eq i64 %611, %_34, !dbg !5383
  br i1 %exitcond2194.not.i, label %bb43.lr.ph.i, label %bb36.i2826, !dbg !5383

bb43.lr.ph.i:                                     ; preds = %bb36.i2826
  store <8 x i32> %610, ptr %596, align 32, !dbg !5516, !alias.scope !4977, !noalias !4993
  store <8 x i32> %625, ptr %597, align 32, !dbg !5517, !alias.scope !4979, !noalias !5327
  %627 = bitcast <8 x float> %571 to <8 x i32>
  %628 = bitcast <8 x float> %570 to <8 x i32>
  %629 = select i1 %bypass, <8 x i32> %566, <8 x i32> %567
  %630 = bitcast <8 x float> %569 to <8 x i32>
  %631 = icmp slt <8 x i32> %630, zeroinitializer
  %632 = bitcast <8 x float> %574 to <8 x i32>
  %633 = bitcast <8 x float> %573 to <8 x i32>
  %634 = bitcast <8 x float> %572 to <8 x i32>
  %635 = icmp slt <8 x i32> %634, zeroinitializer
  br label %bb43.i2827, !dbg !5518

bb43.i2827:                                       ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit144.i, %bb43.lr.ph.i
  %iter3.sroa.13.01970.i = phi i64 [ 0, %bb43.lr.ph.i ], [ %_9.0.i1091.i, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit144.i ]
  %_9.0.i1091.i = add nuw nsw i64 %iter3.sroa.13.01970.i, 1, !dbg !5526
  %_3.i.i.i.i1087.i = getelementptr inbounds nuw %"wide::f32x8_::f32x8", ptr %_52.0, i64 %iter3.sroa.13.01970.i, !dbg !5529
  %_3.i1.i.i.i.i = getelementptr inbounds nuw %"wide::f32x8_::f32x8", ptr %_53.0, i64 %iter3.sroa.13.01970.i, !dbg !5535
  %_138.i = add nuw nsw i64 %iter3.sroa.13.01970.i, %spec.store.select, !dbg !5538
  %slot.i2828 = shl i64 %_138.i, 3, !dbg !5538
  %_311.i = icmp samesign ugt i64 %slot.i2828, %left.1, !dbg !5540
  br i1 %_311.i, label %bb94.i, label %bb95.i, !dbg !5540, !prof !161

bb95.i:                                           ; preds = %bb43.i2827
  %_314.i = sub nuw nsw i64 %left.1, %slot.i2828, !dbg !5545
  %_8.i707.i = icmp samesign ugt i64 %_314.i, 7, !dbg !5546
  br i1 %_8.i707.i, label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i, label %bb2.i708.i, !dbg !5546, !prof !2135

bb2.i708.i:                                       ; preds = %bb95.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_314.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !5551, !noalias !5552
  unreachable, !dbg !5551

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i: ; preds = %bb95.i
  %_318.i = getelementptr inbounds nuw float, ptr %left.0, i64 %slot.i2828, !dbg !5556
  %_143.sroa.0.0.copyload.i = load <8 x float>, ptr %_3.i.i.i.i1087.i, align 32, !dbg !5562, !alias.scope !4985, !noalias !5397
  %636 = fadd <8 x float> %lanes.i588.sroa.0.0.copyload.i, %_143.sroa.0.0.copyload.i, !dbg !5564
  %637 = fmul <8 x float> %636, splat (float 0x3FC542A5A0000000), !dbg !5570
  %638 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %637, <8 x float> splat (float -1.260000e+02)), !dbg !5576
  %639 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %638, <8 x float> splat (float 1.270000e+02)), !dbg !5582
  %640 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %639), !dbg !5587
  %641 = fsub <8 x float> %639, %640, !dbg !5592
  %642 = fmul <8 x float> %641, splat (float 0x3F5E974FA0000000), !dbg !5597
  %643 = fadd <8 x float> %642, splat (float 0x3F82778560000000), !dbg !5602
  %644 = fmul <8 x float> %641, %643, !dbg !5597
  %645 = fadd <8 x float> %644, splat (float 0x3FAC91CE60000000), !dbg !5602
  %646 = fmul <8 x float> %641, %645, !dbg !5597
  %647 = fadd <8 x float> %646, splat (float 0x3FCEBDB560000000), !dbg !5602
  %648 = fmul <8 x float> %641, %647, !dbg !5597
  %649 = fadd <8 x float> %648, splat (float 0x3FE62E4BA0000000), !dbg !5602
  %lanes.i704.sroa.0.0.copyload.i = load <8 x float>, ptr %_318.i, align 4, !dbg !5607, !alias.scope !5611, !noalias !5615
  %650 = fmul <8 x float> %641, %649, !dbg !5617
  %651 = fadd <8 x float> %650, splat (float 1.000000e+00), !dbg !5622
  %652 = fadd <8 x float> %640, splat (float 0x4160000FE0000000), !dbg !5627
  %653 = bitcast <8 x float> %652 to <8 x i32>, !dbg !5632
  %_3.i1092.i = shl <8 x i32> %653, splat (i32 23), !dbg !5636
  %654 = bitcast <8 x i32> %_3.i1092.i to <8 x float>, !dbg !5637
  %655 = fmul <8 x float> %651, %654, !dbg !5639
  %656 = fmul <8 x float> %lanes.i704.sroa.0.0.copyload.i, %655, !dbg !5643
  %657 = fsub <8 x float> %656, %lanes.i704.sroa.0.0.copyload.i, !dbg !5648
  %658 = fmul <8 x float> %lanes.i583.sroa.0.0.copyload.i, %657, !dbg !5654
  %659 = fadd <8 x float> %lanes.i704.sroa.0.0.copyload.i, %658, !dbg !5659
  %660 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_143.sroa.0.0.copyload.i, <8 x float> zeroinitializer, i8 0), !dbg !5663
  %661 = bitcast <8 x float> %660 to <8 x i32>, !dbg !5669
  %662 = and <8 x i32> %661, %627, !dbg !5673
  %663 = or <8 x i32> %662, %628
  %.reass.i.reass = or <8 x i32> %663, %629
  %664 = select <8 x i1> %631, <8 x float> %656, <8 x float> %659, !dbg !5675
  %665 = icmp slt <8 x i32> %.reass.i.reass, zeroinitializer, !dbg !5680
  %666 = select <8 x i1> %665, <8 x float> %lanes.i704.sroa.0.0.copyload.i, <8 x float> %664, !dbg !5680
  store <8 x float> %666, ptr %_318.i, align 4, !dbg !5685, !alias.scope !5690, !noalias !5694
  %_323.i = icmp samesign ugt i64 %slot.i2828, %right.1, !dbg !5698
  br i1 %_323.i, label %bb96.i, label %bb97.i, !dbg !5698, !prof !161

bb94.i:                                           ; preds = %bb43.i2827
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %slot.i2828, i64 noundef range(i64 8, 34359738361) %left.1, i64 noundef range(i64 8, 34359738361) %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d76c0cfb43c310b376f3b4a5d480172c) #23, !dbg !5702, !noalias !5302
  unreachable, !dbg !5702

bb97.i:                                           ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i
  %_326.i = sub nuw nsw i64 %right.1, %slot.i2828, !dbg !5703
  %_8.i699.i = icmp samesign ugt i64 %_326.i, 7, !dbg !5704
  br i1 %_8.i699.i, label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit144.i, label %bb2.i700.i, !dbg !5704, !prof !2135

bb2.i700.i:                                       ; preds = %bb97.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_326.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !5709, !noalias !5710
  unreachable, !dbg !5709

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit144.i: ; preds = %bb97.i
  %_330.i = getelementptr inbounds nuw float, ptr %right.0, i64 %slot.i2828, !dbg !5714
  %_151.sroa.0.0.copyload.i = load <8 x float>, ptr %_3.i1.i.i.i.i, align 32, !dbg !5719, !alias.scope !4987, !noalias !5457
  %667 = fadd <8 x float> %lanes.i628.sroa.0.0.copyload.i, %_151.sroa.0.0.copyload.i, !dbg !5721
  %668 = fmul <8 x float> %667, splat (float 0x3FC542A5A0000000), !dbg !5727
  %669 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %668, <8 x float> splat (float -1.260000e+02)), !dbg !5733
  %670 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %669, <8 x float> splat (float 1.270000e+02)), !dbg !5739
  %671 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %670), !dbg !5744
  %672 = fsub <8 x float> %670, %671, !dbg !5749
  %673 = fmul <8 x float> %672, splat (float 0x3F5E974FA0000000), !dbg !5754
  %674 = fadd <8 x float> %673, splat (float 0x3F82778560000000), !dbg !5759
  %675 = fmul <8 x float> %672, %674, !dbg !5754
  %676 = fadd <8 x float> %675, splat (float 0x3FAC91CE60000000), !dbg !5759
  %677 = fmul <8 x float> %672, %676, !dbg !5754
  %678 = fadd <8 x float> %677, splat (float 0x3FCEBDB560000000), !dbg !5759
  %679 = fmul <8 x float> %672, %678, !dbg !5754
  %680 = fadd <8 x float> %679, splat (float 0x3FE62E4BA0000000), !dbg !5759
  %lanes.i696.sroa.0.0.copyload.i = load <8 x float>, ptr %_330.i, align 4, !dbg !5764, !alias.scope !5768, !noalias !5772
  %681 = fmul <8 x float> %672, %680, !dbg !5774
  %682 = fadd <8 x float> %681, splat (float 1.000000e+00), !dbg !5779
  %683 = fadd <8 x float> %671, splat (float 0x4160000FE0000000), !dbg !5784
  %684 = bitcast <8 x float> %683 to <8 x i32>, !dbg !5789
  %_3.i1093.i = shl <8 x i32> %684, splat (i32 23), !dbg !5793
  %685 = bitcast <8 x i32> %_3.i1093.i to <8 x float>, !dbg !5794
  %686 = fmul <8 x float> %682, %685, !dbg !5796
  %687 = fmul <8 x float> %lanes.i696.sroa.0.0.copyload.i, %686, !dbg !5800
  %688 = fsub <8 x float> %687, %lanes.i696.sroa.0.0.copyload.i, !dbg !5805
  %689 = fmul <8 x float> %lanes.i623.sroa.0.0.copyload.i, %688, !dbg !5811
  %690 = fadd <8 x float> %lanes.i696.sroa.0.0.copyload.i, %689, !dbg !5816
  %691 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_151.sroa.0.0.copyload.i, <8 x float> zeroinitializer, i8 0), !dbg !5820
  %692 = bitcast <8 x float> %691 to <8 x i32>, !dbg !5826
  %693 = and <8 x i32> %692, %632, !dbg !5830
  %694 = or <8 x i32> %693, %633
  %.reass2668.i.reass = or <8 x i32> %694, %629
  %695 = select <8 x i1> %635, <8 x float> %687, <8 x float> %690, !dbg !5832
  %696 = icmp slt <8 x i32> %.reass2668.i.reass, zeroinitializer, !dbg !5837
  %697 = select <8 x i1> %696, <8 x float> %lanes.i696.sroa.0.0.copyload.i, <8 x float> %695, !dbg !5837
  store <8 x float> %697, ptr %_330.i, align 4, !dbg !5842, !alias.scope !5847, !noalias !5851
  %exitcond2209.not.i = icmp eq i64 %_9.0.i1091.i, %_34, !dbg !5518
  br i1 %exitcond2209.not.i, label %bb19, label %bb43.i2827, !dbg !5518

bb96.i:                                           ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %slot.i2828, i64 noundef range(i64 8, 34359738361) %right.1, i64 noundef range(i64 8, 34359738361) %right.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7159ff60b58ea8ffb865744fc2de0bda) #23, !dbg !5855, !noalias !5302
  unreachable, !dbg !5855

bb69.i:                                           ; preds = %bb14.i2814
  %_208.i = sub nuw nsw i64 %left.1, %slot7.i, !dbg !5856
  %_212.i = getelementptr inbounds nuw float, ptr %left.0, i64 %slot7.i, !dbg !5857
  %_8.i691.i = icmp samesign ugt i64 %_208.i, 7, !dbg !5861
  br i1 %_8.i691.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit694.i, label %bb2.i692.i, !dbg !5861, !prof !2135

bb2.i692.i:                                       ; preds = %bb69.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_208.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !5866, !noalias !5867
  unreachable, !dbg !5866

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit694.i: ; preds = %bb69.i
  %lanes.i688.sroa.0.0.copyload.i = load <8 x float>, ptr %_212.i, align 4, !dbg !5871, !alias.scope !5875, !noalias !5879
  %_216.i = icmp samesign ugt i64 %slot7.i, %right.1, !dbg !5881
  br i1 %_216.i, label %bb70.i, label %bb71.i, !dbg !5881, !prof !161

bb68.i:                                           ; preds = %bb14.i2814
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %slot7.i, i64 noundef range(i64 8, 34359738361) %left.1, i64 noundef range(i64 8, 34359738361) %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_142618cca8840ae208db8a98d223f1fd) #23, !dbg !5886, !noalias !5302
  unreachable, !dbg !5886

bb71.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit694.i
  %_219.i = sub nuw nsw i64 %right.1, %slot7.i, !dbg !5887
  %_223.i = getelementptr inbounds nuw float, ptr %right.0, i64 %slot7.i, !dbg !5888
  %_8.i683.i = icmp samesign ugt i64 %_219.i, 7, !dbg !5893
  br i1 %_8.i683.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit686.i, label %bb2.i684.i, !dbg !5893, !prof !2135

bb2.i684.i:                                       ; preds = %bb71.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_219.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !5898, !noalias !5899
  unreachable, !dbg !5898

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit686.i: ; preds = %bb71.i
  %lanes.i680.sroa.0.0.copyload.i = load <8 x float>, ptr %_223.i, align 4, !dbg !5903, !alias.scope !5907, !noalias !5911
  switch i64 %_6.i.i2809, label %default.unreachable [
    i64 0, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i2824
    i64 1, label %bb3.i.i2832
    i64 2, label %bb2.i.i2815
  ], !dbg !5913

bb3.i.i2832:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit686.i
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i2824, !dbg !5916

bb2.i.i2815:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit686.i
  %_25.i.i2816 = icmp ugt i64 %slot7.i, %sidechain_left.1.i.i2810, !dbg !5917
  br i1 %_25.i.i2816, label %bb17.i.i2831, label %bb18.i.i2817, !dbg !5917, !prof !161

bb18.i.i2817:                                     ; preds = %bb2.i.i2815
  %_28.i.i2818 = sub nuw i64 %sidechain_left.1.i.i2810, %slot7.i, !dbg !5920
  %_8.i643.i = icmp samesign ugt i64 %_28.i.i2818, 7, !dbg !5921
  br i1 %_8.i643.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit646.i, label %bb2.i644.i, !dbg !5921, !prof !2135

bb2.i644.i:                                       ; preds = %bb18.i.i2817
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_28.i.i2818, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !5926, !noalias !5927
  unreachable, !dbg !5926

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit646.i: ; preds = %bb18.i.i2817
  %_32.i.i2819 = getelementptr inbounds nuw float, ptr %sidechain_left.0.i.i2813, i64 %slot7.i, !dbg !5936
  %lanes.i640.sroa.0.0.copyload.i = load <8 x float>, ptr %_32.i.i2819, align 4, !dbg !5938, !alias.scope !5942, !noalias !5946
  %_33.i.i2820 = icmp ugt i64 %slot7.i, %sidechain_right.1.i.i2812, !dbg !5948
  br i1 %_33.i.i2820, label %bb19.i.i2830, label %bb20.i.i2821, !dbg !5948, !prof !161

bb17.i.i2831:                                     ; preds = %bb2.i.i2815
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 34359738361) %slot7.i, i64 noundef %sidechain_left.1.i.i2810, i64 noundef %sidechain_left.1.i.i2810, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7fe5a91ec575deda110c046c84630100) #23, !dbg !5951, !noalias !5952
  unreachable, !dbg !5951

bb20.i.i2821:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit646.i
  %_36.i.i2822 = sub nuw i64 %sidechain_right.1.i.i2812, %slot7.i, !dbg !5954
  %_8.i635.i = icmp samesign ugt i64 %_36.i.i2822, 7, !dbg !5955
  br i1 %_8.i635.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit638.i, label %bb2.i636.i, !dbg !5955, !prof !2135

bb2.i636.i:                                       ; preds = %bb20.i.i2821
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_36.i.i2822, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !5960, !noalias !5961
  unreachable, !dbg !5960

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit638.i: ; preds = %bb20.i.i2821
  %_40.i.i2823 = getelementptr inbounds nuw float, ptr %sidechain_right.0.i.i2811, i64 %slot7.i, !dbg !5965
  %lanes.i633.sroa.0.0.copyload.i = load <8 x float>, ptr %_40.i.i2823, align 4, !dbg !5967, !alias.scope !5971, !noalias !5975
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i2824, !dbg !5977

bb19.i.i2830:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit646.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 34359738361) %slot7.i, i64 noundef %sidechain_right.1.i.i2812, i64 noundef %sidechain_right.1.i.i2812, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f2cba1f89421a726339097d2eba48536) #23, !dbg !5978, !noalias !5952
  unreachable, !dbg !5978

_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i2824: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit638.i, %bb3.i.i2832, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit686.i
  %.sroa.0.0.i = phi <8 x float> [ %lanes.i688.sroa.0.0.copyload.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit686.i ], [ zeroinitializer, %bb3.i.i2832 ], [ %lanes.i640.sroa.0.0.copyload.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit638.i ], !dbg !5979
  %.sroa.01123.0.i = phi <8 x float> [ %lanes.i680.sroa.0.0.copyload.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit686.i ], [ zeroinitializer, %bb3.i.i2832 ], [ %lanes.i633.sroa.0.0.copyload.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit638.i ], !dbg !5979
  %698 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %.sroa.0.0.i), !dbg !5980
  %699 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %.sroa.01123.0.i), !dbg !5986
  %700 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %698, <8 x float> %699), !dbg !5992
  %701 = fmul <8 x float> %698, splat (float 5.000000e-01), !dbg !5997
  %702 = fmul <8 x float> %699, splat (float 5.000000e-01), !dbg !6002
  %703 = fadd <8 x float> %701, %702, !dbg !6007
  %704 = select <8 x i1> %581, <8 x float> %703, <8 x float> %700, !dbg !6012
  %705 = select <8 x i1> %583, <8 x float> %704, <8 x float> %698, !dbg !6017
  %706 = select <8 x i1> %583, <8 x float> %704, <8 x float> %699, !dbg !6022
  %707 = add i64 %head.sroa.0.01959.i, 1, !dbg !6027
  %_62.i = icmp eq i64 %707, %ring_length.i2802, !dbg !6029
  %spec.store.select.i2825 = select i1 %_62.i, i64 0, i64 %707, !dbg !6029
  %_66.i = shl i64 %head.sroa.0.01959.i, 3, !dbg !6031
  %_224.i = icmp ugt i64 %_66.i, %_335.1.i, !dbg !6033
  br i1 %_224.i, label %bb72.i, label %bb73.i, !dbg !6033, !prof !161

bb70.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit694.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %slot7.i, i64 noundef range(i64 8, 34359738361) %right.1, i64 noundef range(i64 8, 34359738361) %right.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e7322f5a021c15e2c563763633ec7262) #23, !dbg !6039, !noalias !5302
  unreachable, !dbg !6039

bb73.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i2824
  %_227.i = sub nuw i64 %_335.1.i, %_66.i, !dbg !6040
  %_8.i963.i = icmp samesign ugt i64 %_227.i, 7, !dbg !6041
  br i1 %_8.i963.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit965.i, label %bb2.i964.i, !dbg !6041, !prof !2135

bb2.i964.i:                                       ; preds = %bb73.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_227.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !6046, !noalias !6047
  unreachable, !dbg !6046

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit965.i: ; preds = %bb73.i
  %_231.i = getelementptr inbounds nuw float, ptr %_335.0.i, i64 %_66.i, !dbg !6051
  store <8 x float> %lanes.i688.sroa.0.0.copyload.i, ptr %_231.i, align 4, !dbg !6056, !alias.scope !6060, !noalias !6064
  %_232.i = icmp ugt i64 %_66.i, %_336.1.i, !dbg !6066
  br i1 %_232.i, label %bb74.i, label %bb75.i, !dbg !6066, !prof !161

bb72.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i2824
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_66.i, i64 noundef %_335.1.i, i64 noundef %_335.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_797e6d44639a19f9bc9393121579dd3c) #23, !dbg !6070, !noalias !5302
  unreachable, !dbg !6070

bb75.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit965.i
  %_235.i = sub nuw i64 %_336.1.i, %_66.i, !dbg !6071
  %_8.i959.i = icmp samesign ugt i64 %_235.i, 7, !dbg !6072
  br i1 %_8.i959.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit961.i, label %bb2.i960.i, !dbg !6072, !prof !2135

bb2.i960.i:                                       ; preds = %bb75.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_235.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !6077, !noalias !6078
  unreachable, !dbg !6077

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit961.i: ; preds = %bb75.i
  %_239.i = getelementptr inbounds nuw float, ptr %_336.0.i, i64 %_66.i, !dbg !6082
  store <8 x float> %705, ptr %_239.i, align 4, !dbg !6087, !alias.scope !6091, !noalias !6095
  %_240.i = icmp ugt i64 %_66.i, %_337.1.i, !dbg !6097
  br i1 %_240.i, label %bb76.i, label %bb77.i, !dbg !6097, !prof !161

bb74.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit965.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_66.i, i64 noundef %_336.1.i, i64 noundef %_336.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8cd1878abf6085d06f7a836874ffa4c2) #23, !dbg !6101, !noalias !5302
  unreachable, !dbg !6101

bb77.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit961.i
  %_243.i = sub nuw i64 %_337.1.i, %_66.i, !dbg !6102
  %_8.i955.i = icmp samesign ugt i64 %_243.i, 7, !dbg !6103
  br i1 %_8.i955.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit957.i, label %bb2.i956.i, !dbg !6103, !prof !2135

bb2.i956.i:                                       ; preds = %bb77.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_243.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !6108, !noalias !6109
  unreachable, !dbg !6108

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit957.i: ; preds = %bb77.i
  %_247.i = getelementptr inbounds nuw float, ptr %_337.0.i, i64 %_66.i, !dbg !6113
  store <8 x float> %lanes.i680.sroa.0.0.copyload.i, ptr %_247.i, align 4, !dbg !6118, !alias.scope !6122, !noalias !6126
  %_248.i = icmp ugt i64 %_66.i, %_338.1.i, !dbg !6128
  br i1 %_248.i, label %bb78.i, label %bb79.i, !dbg !6128, !prof !161

bb76.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit961.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_66.i, i64 noundef %_337.1.i, i64 noundef %_337.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8529cd63341985e7fb6803b2c00d96ec) #23, !dbg !6132, !noalias !5302
  unreachable, !dbg !6132

bb79.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit957.i
  %_251.i = sub nuw i64 %_338.1.i, %_66.i, !dbg !6133
  %_8.i951.i = icmp samesign ugt i64 %_251.i, 7, !dbg !6134
  br i1 %_8.i951.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit953.i, label %bb2.i952.i, !dbg !6134, !prof !2135

bb2.i952.i:                                       ; preds = %bb79.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_251.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !6139, !noalias !6140
  unreachable, !dbg !6139

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit953.i: ; preds = %bb79.i
  %_255.i = getelementptr inbounds nuw float, ptr %_338.0.i, i64 %_66.i, !dbg !6144
  store <8 x float> %706, ptr %_255.i, align 4, !dbg !6149, !alias.scope !6153, !noalias !6157
  %_85.i = shl i64 %spec.store.select.i2825, 3, !dbg !6159
  %_256.i = icmp ugt i64 %_85.i, %_335.1.i, !dbg !6160
  br i1 %_256.i, label %bb80.i, label %bb81.i, !dbg !6160, !prof !161

bb78.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit957.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_66.i, i64 noundef %_338.1.i, i64 noundef %_338.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_50f064baf8dd86aaabd3fcd49badf48e) #23, !dbg !6164, !noalias !5302
  unreachable, !dbg !6164

bb81.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit953.i
  %_259.i = sub nuw i64 %_335.1.i, %_85.i, !dbg !6165
  %_8.i675.i = icmp samesign ugt i64 %_259.i, 7, !dbg !6166
  br i1 %_8.i675.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit949.i, label %bb2.i676.i, !dbg !6166, !prof !2135

bb2.i676.i:                                       ; preds = %bb81.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_259.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !6171, !noalias !6172
  unreachable, !dbg !6171

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit949.i: ; preds = %bb81.i
  %_263.i = getelementptr inbounds nuw float, ptr %_335.0.i, i64 %_85.i, !dbg !6176
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_212.i, ptr noundef nonnull align 4 dereferenceable(32) %_263.i, i64 32, i1 false), !dbg !6181, !noalias !6186
  %_268.i = icmp ugt i64 %_85.i, %_337.1.i, !dbg !6187
  br i1 %_268.i, label %bb82.i, label %bb83.i, !dbg !6187, !prof !161

bb80.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit953.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_85.i, i64 noundef %_335.1.i, i64 noundef %_335.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_0171cce559b4754ea869128a8d469334) #23, !dbg !6191, !noalias !5302
  unreachable, !dbg !6191

bb83.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit949.i
  %_271.i = sub nuw i64 %_337.1.i, %_85.i, !dbg !6192
  %_8.i667.i = icmp samesign ugt i64 %_271.i, 7, !dbg !6193
  br i1 %_8.i667.i, label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_log2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit174.i, label %bb2.i668.i, !dbg !6193, !prof !2135

bb2.i668.i:                                       ; preds = %bb83.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_271.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !6198, !noalias !6199
  unreachable, !dbg !6198

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_log2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit174.i: ; preds = %bb83.i
  %_275.i = getelementptr inbounds nuw float, ptr %_337.0.i, i64 %_85.i, !dbg !6203
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_223.i, ptr noundef nonnull align 4 dereferenceable(32) %_275.i, i64 32, i1 false), !dbg !6208, !noalias !6213
  %lanes.i656.sroa.0.0.copyload.i = load <8 x float>, ptr %data.i.i.i.i.i.i.i.i.i, align 4, !dbg !6214, !alias.scope !6219, !noalias !6223
  %708 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i656.sroa.0.0.copyload.i, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !6227
  %709 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %708, <8 x float> splat (float 0x3810000000000000)), !dbg !6233
  %710 = bitcast <8 x float> %709 to <4 x i64>, !dbg !6240
  %711 = and <4 x i64> %710, splat (i64 36028792732385279), !dbg !6241
  %712 = or disjoint <4 x i64> %711, splat (i64 4575657222473777152), !dbg !6246
  %713 = bitcast <4 x i64> %712 to <8 x float>, !dbg !6250
  %714 = fadd <8 x float> %713, splat (float -1.000000e+00), !dbg !6251
  %715 = fmul <8 x float> %714, splat (float 0x3F9B17A960000000), !dbg !6256
  %716 = fsub <8 x float> splat (float 0x3FBF9A8440000000), %715, !dbg !6261
  %717 = fmul <8 x float> %714, %716, !dbg !6256
  %718 = fadd <8 x float> %717, splat (float 0xBFD1E3F400000000), !dbg !6261
  %719 = fmul <8 x float> %714, %718, !dbg !6256
  %720 = fadd <8 x float> %719, splat (float 0x3FDD544F20000000), !dbg !6261
  %721 = fmul <8 x float> %714, %720, !dbg !6256
  %722 = fadd <8 x float> %721, splat (float 0xBFE6FC2A60000000), !dbg !6261
  %723 = fmul <8 x float> %714, %722, !dbg !6256
  %724 = fadd <8 x float> %723, splat (float 0x3FF714B2A0000000), !dbg !6261
  %725 = bitcast <8 x float> %709 to <8 x i32>, !dbg !6266
  %_3.i1094.i = lshr <8 x i32> %725, splat (i32 23), !dbg !6270
  %726 = or disjoint <8 x i32> %_3.i1094.i, splat (i32 1258291200), !dbg !6271
  %727 = bitcast <8 x i32> %726 to <8 x float>, !dbg !6275
  %728 = fadd <8 x float> %727, splat (float 0xC160000FE0000000), !dbg !6276
  %729 = fmul <8 x float> %714, %724, !dbg !6280
  %730 = fadd <8 x float> %728, %729, !dbg !6285
  %731 = fmul <8 x float> %730, splat (float 0x4018151820000000), !dbg !6290
  %732 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %731, <8 x float> splat (float -1.600000e+02)), !dbg !6295
  %733 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %732, <8 x float> splat (float 2.400000e+01)), !dbg !6300
  %734 = fsub <8 x float> %733, %lanes.i578.sroa.0.0.copyload.i, !dbg !6305
  %735 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %734, <8 x float> %lanes.i568.sroa.0.0.copyload.i, i8 30), !dbg !6311
  %736 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %734, <8 x float> %592, i8 18), !dbg !6317
  %737 = fadd <8 x float> %lanes.i568.sroa.0.0.copyload.i, %734, !dbg !6323
  %738 = fmul <8 x float> %737, %737, !dbg !6328
  %739 = fmul <8 x float> %lanes.i563.sroa.0.0.copyload.i, %738, !dbg !6333
  %740 = bitcast <8 x float> %735 to <8 x i32>, !dbg !6338
  %741 = icmp slt <8 x i32> %740, zeroinitializer, !dbg !6342
  %.v.i = select <8 x i1> %741, <8 x float> %734, <8 x float> %739, !dbg !6342
  %742 = fmul <8 x float> %lanes.i573.sroa.0.0.copyload.i, %.v.i, !dbg !6342
  %743 = bitcast <8 x float> %736 to <8 x i32>, !dbg !6344
  %744 = icmp slt <8 x i32> %743, zeroinitializer, !dbg !6348
  %745 = select <8 x i1> %744, <8 x float> zeroinitializer, <8 x float> %742, !dbg !6348
  %746 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %745, <8 x float> splat (float -1.000000e+02)), !dbg !6350
  %747 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %746, <8 x float> zeroinitializer), !dbg !6355
  store <8 x float> %747, ptr %_3.i.i.i.i.i.i.i, align 32, !dbg !6360, !alias.scope !4985, !noalias !5397
  %lanes.i648.sroa.0.0.copyload.i = load <8 x float>, ptr %data.i5.i.i.i.i.i.i.i.i, align 4, !dbg !6361, !alias.scope !6366, !noalias !6370
  %748 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i648.sroa.0.0.copyload.i, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !6374
  %749 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %748, <8 x float> splat (float 0x3810000000000000)), !dbg !6380
  %750 = bitcast <8 x float> %749 to <4 x i64>, !dbg !6387
  %751 = and <4 x i64> %750, splat (i64 36028792732385279), !dbg !6388
  %752 = or disjoint <4 x i64> %751, splat (i64 4575657222473777152), !dbg !6393
  %753 = bitcast <4 x i64> %752 to <8 x float>, !dbg !6397
  %754 = fadd <8 x float> %753, splat (float -1.000000e+00), !dbg !6398
  %755 = fmul <8 x float> %754, splat (float 0x3F9B17A960000000), !dbg !6403
  %756 = fsub <8 x float> splat (float 0x3FBF9A8440000000), %755, !dbg !6408
  %757 = fmul <8 x float> %754, %756, !dbg !6403
  %758 = fadd <8 x float> %757, splat (float 0xBFD1E3F400000000), !dbg !6408
  %759 = fmul <8 x float> %754, %758, !dbg !6403
  %760 = fadd <8 x float> %759, splat (float 0x3FDD544F20000000), !dbg !6408
  %761 = fmul <8 x float> %754, %760, !dbg !6403
  %762 = fadd <8 x float> %761, splat (float 0xBFE6FC2A60000000), !dbg !6408
  %763 = fmul <8 x float> %754, %762, !dbg !6403
  %764 = fadd <8 x float> %763, splat (float 0x3FF714B2A0000000), !dbg !6408
  %765 = bitcast <8 x float> %749 to <8 x i32>, !dbg !6413
  %_3.i1095.i = lshr <8 x i32> %765, splat (i32 23), !dbg !6417
  %766 = or disjoint <8 x i32> %_3.i1095.i, splat (i32 1258291200), !dbg !6418
  %767 = bitcast <8 x i32> %766 to <8 x float>, !dbg !6422
  %768 = fadd <8 x float> %767, splat (float 0xC160000FE0000000), !dbg !6423
  %769 = fmul <8 x float> %754, %764, !dbg !6427
  %770 = fadd <8 x float> %768, %769, !dbg !6432
  %771 = fmul <8 x float> %770, splat (float 0x4018151820000000), !dbg !6437
  %772 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %771, <8 x float> splat (float -1.600000e+02)), !dbg !6442
  %773 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %772, <8 x float> splat (float 2.400000e+01)), !dbg !6447
  %774 = fsub <8 x float> %773, %lanes.i618.sroa.0.0.copyload.i, !dbg !6452
  %775 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %774, <8 x float> %lanes.i608.sroa.0.0.copyload.i, i8 30), !dbg !6458
  %776 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %774, <8 x float> %593, i8 18), !dbg !6464
  %777 = fadd <8 x float> %lanes.i608.sroa.0.0.copyload.i, %774, !dbg !6470
  %778 = fmul <8 x float> %777, %777, !dbg !6475
  %779 = fmul <8 x float> %lanes.i603.sroa.0.0.copyload.i, %778, !dbg !6480
  %780 = bitcast <8 x float> %775 to <8 x i32>, !dbg !6485
  %781 = icmp slt <8 x i32> %780, zeroinitializer, !dbg !6489
  %.v1743.i = select <8 x i1> %781, <8 x float> %774, <8 x float> %779, !dbg !6489
  %782 = fmul <8 x float> %lanes.i613.sroa.0.0.copyload.i, %.v1743.i, !dbg !6489
  %783 = bitcast <8 x float> %776 to <8 x i32>, !dbg !6491
  %784 = icmp slt <8 x i32> %783, zeroinitializer, !dbg !6495
  %785 = select <8 x i1> %784, <8 x float> zeroinitializer, <8 x float> %782, !dbg !6495
  %786 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %785, <8 x float> splat (float -1.000000e+02)), !dbg !6497
  %787 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %786, <8 x float> zeroinitializer), !dbg !6502
  store <8 x float> %787, ptr %_3.i.i.i.i.i, align 32, !dbg !6507, !alias.scope !4987, !noalias !5457
  %exitcond.not.i = icmp eq i64 %_9.0.i.i, %_34, !dbg !5328
  br i1 %exitcond.not.i, label %bb15.i, label %bb14.i2814, !dbg !5328

bb82.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit949.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_85.i, i64 noundef %_337.1.i, i64 noundef %_337.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_24cf5413af95bfbc1de92cceabc9bf67) #23, !dbg !6508, !noalias !5302
  unreachable, !dbg !6508
}

define internal fastcc void @_RINvNtCse3bfmKSZS8Y_10compressor6kernel13process_blockfEB4_(ptr noalias noundef nonnull align 4 captures(none) %left.0, i64 noundef range(i64 0, 2305843009213693952) %left.1, ptr noalias noundef nonnull align 4 captures(none) %right.0, i64 noundef range(i64 0, 2305843009213693952) %right.1, ptr dead_on_return noalias noundef nonnull readonly align 8 captures(none) dereferenceable(40) %detector, i64 noundef %frames, i32 noundef range(i32 1, 4) %link, i1 noundef zeroext %bypass, i32 noundef %sample_rate, ptr noalias noundef nonnull align 8 dereferenceable(1520) %channels.0, ptr noalias noundef nonnull align 8 dereferenceable(1520) %channels.1, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(64) %staged) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !6509 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %channels.0, i64 556, !dbg !6510
  %_12.i1561 = load i32, ptr %0, align 4, !dbg !6510, !alias.scope !6519, !noundef !11
  %1 = getelementptr inbounds nuw i8, ptr %channels.0, i64 684, !dbg !6510
  %_12.1.i = load i32, ptr %1, align 4, !dbg !6510, !alias.scope !6519, !noundef !11
  %2 = getelementptr inbounds nuw i8, ptr %channels.0, i64 812, !dbg !6510
  %_12.2.i = load i32, ptr %2, align 4, !dbg !6510, !alias.scope !6519, !noundef !11
  %3 = getelementptr inbounds nuw i8, ptr %channels.0, i64 940, !dbg !6510
  %_12.3.i = load i32, ptr %3, align 4, !dbg !6510, !alias.scope !6519, !noundef !11
  %4 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1068, !dbg !6510
  %_12.4.i = load i32, ptr %4, align 4, !dbg !6510, !alias.scope !6519, !noundef !11
  %5 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1196, !dbg !6510
  %_12.5.i = load i32, ptr %5, align 4, !dbg !6510, !alias.scope !6519, !noundef !11
  %6 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1324, !dbg !6510
  %_12.6.i = load i32, ptr %6, align 4, !dbg !6510, !alias.scope !6519, !noundef !11
  %spec.store.select.1.i = tail call i32 @llvm.umax.i32(i32 %_12.1.i, i32 %_12.i1561), !dbg !6510
  %spec.store.select.2.i = tail call i32 @llvm.umax.i32(i32 %_12.2.i, i32 %spec.store.select.1.i), !dbg !6510
  %spec.store.select.3.i = tail call i32 @llvm.umax.i32(i32 %_12.3.i, i32 %spec.store.select.2.i), !dbg !6510
  %spec.store.select.4.i = tail call i32 @llvm.umax.i32(i32 %_12.4.i, i32 %spec.store.select.3.i), !dbg !6510
  %spec.store.select.5.i = tail call i32 @llvm.umax.i32(i32 %_12.5.i, i32 %spec.store.select.4.i), !dbg !6510
  %spec.store.select.6.i = tail call noundef i32 @llvm.umax.i32(i32 %_12.6.i, i32 %spec.store.select.5.i), !dbg !6510
  %7 = getelementptr inbounds nuw i8, ptr %channels.1, i64 556, !dbg !6522
  %_12.i1562 = load i32, ptr %7, align 4, !dbg !6522, !alias.scope !6524, !noundef !11
  %8 = getelementptr inbounds nuw i8, ptr %channels.1, i64 684, !dbg !6522
  %_12.1.i1563 = load i32, ptr %8, align 4, !dbg !6522, !alias.scope !6524, !noundef !11
  %9 = getelementptr inbounds nuw i8, ptr %channels.1, i64 812, !dbg !6522
  %_12.2.i1564 = load i32, ptr %9, align 4, !dbg !6522, !alias.scope !6524, !noundef !11
  %10 = getelementptr inbounds nuw i8, ptr %channels.1, i64 940, !dbg !6522
  %_12.3.i1565 = load i32, ptr %10, align 4, !dbg !6522, !alias.scope !6524, !noundef !11
  %11 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1068, !dbg !6522
  %_12.4.i1566 = load i32, ptr %11, align 4, !dbg !6522, !alias.scope !6524, !noundef !11
  %12 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1196, !dbg !6522
  %_12.5.i1567 = load i32, ptr %12, align 4, !dbg !6522, !alias.scope !6524, !noundef !11
  %13 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1324, !dbg !6522
  %_12.6.i1568 = load i32, ptr %13, align 4, !dbg !6522, !alias.scope !6524, !noundef !11
  %spec.store.select.1.i1569 = tail call i32 @llvm.umax.i32(i32 %_12.1.i1563, i32 %_12.i1562), !dbg !6522
  %spec.store.select.2.i1570 = tail call i32 @llvm.umax.i32(i32 %_12.2.i1564, i32 %spec.store.select.1.i1569), !dbg !6522
  %spec.store.select.3.i1571 = tail call i32 @llvm.umax.i32(i32 %_12.3.i1565, i32 %spec.store.select.2.i1570), !dbg !6522
  %spec.store.select.4.i1572 = tail call i32 @llvm.umax.i32(i32 %_12.4.i1566, i32 %spec.store.select.3.i1571), !dbg !6522
  %spec.store.select.5.i1573 = tail call i32 @llvm.umax.i32(i32 %_12.5.i1567, i32 %spec.store.select.4.i1572), !dbg !6522
  %spec.store.select.6.i1574 = tail call noundef i32 @llvm.umax.i32(i32 %_12.6.i1568, i32 %spec.store.select.5.i1573), !dbg !6522
  %..i1575 = tail call noundef i32 @llvm.umax.i32(i32 %spec.store.select.6.i1574, i32 %spec.store.select.6.i), !dbg !6527
  %14 = zext i32 %..i1575 to i64, !dbg !6529
  %_24 = icmp ugt i64 %frames, %14, !dbg !6530
  %spec.store.select = tail call i64 @llvm.umin.i64(i64 %frames, i64 %14), !dbg !6530
  %_25.not = icmp eq i64 %spec.store.select, 0, !dbg !6532
  br i1 %_25.not, label %bb10, label %bb7, !dbg !6532

bb7:                                              ; preds = %start
  %.sroa.0.0.copyload = load i64, ptr %detector, align 8, !dbg !6534
  %.sroa.4.0.detector.sroa_idx = getelementptr inbounds nuw i8, ptr %detector, i64 8, !dbg !6534
  %.sroa.4.0.copyload = load ptr, ptr %.sroa.4.0.detector.sroa_idx, align 8, !dbg !6534
  %.sroa.5.0.detector.sroa_idx = getelementptr inbounds nuw i8, ptr %detector, i64 16, !dbg !6534
  %.sroa.5.0.copyload = load i64, ptr %.sroa.5.0.detector.sroa_idx, align 8, !dbg !6534
  %.sroa.6.0.detector.sroa_idx = getelementptr inbounds nuw i8, ptr %detector, i64 24, !dbg !6534
  %.sroa.6.0.copyload = load ptr, ptr %.sroa.6.0.detector.sroa_idx, align 8, !dbg !6534
  %.sroa.7.0.detector.sroa_idx = getelementptr inbounds nuw i8, ptr %detector, i64 32, !dbg !6534
  %.sroa.7.0.copyload = load i64, ptr %.sroa.7.0.detector.sroa_idx, align 8, !dbg !6534
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6535), !dbg !6534
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6538), !dbg !6534
  %15 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1512, !dbg !6540
  %_12.i11 = load i32, ptr %15, align 8, !dbg !6540, !alias.scope !6535, !noalias !6544, !noundef !11
  %ring_length.i12 = zext i32 %_12.i11 to i64, !dbg !6540
  %16 = icmp eq i32 %link, 1, !dbg !6548
  %.not = icmp eq i32 %link, 3, !dbg !6557
  %_15.i13 = getelementptr inbounds nuw i8, ptr %channels.0, i64 288, !dbg !6560
  %_4.i617 = getelementptr inbounds nuw i8, ptr %channels.0, i64 480, !dbg !6562
  %_7.i619 = getelementptr inbounds nuw i8, ptr %channels.0, i64 512, !dbg !6565
  %_14.i622 = getelementptr inbounds nuw i8, ptr %channels.0, i64 320, !dbg !6567
  %_17.i624 = getelementptr inbounds nuw i8, ptr %channels.0, i64 352, !dbg !6569
  %_20.i626 = getelementptr inbounds nuw i8, ptr %channels.0, i64 384, !dbg !6570
  %_23.i628 = getelementptr inbounds nuw i8, ptr %channels.0, i64 416, !dbg !6571
  %_26.i630 = getelementptr inbounds nuw i8, ptr %channels.0, i64 448, !dbg !6572
  %_17.i14 = getelementptr inbounds nuw i8, ptr %channels.1, i64 288, !dbg !6573
  %_4.i593 = getelementptr inbounds nuw i8, ptr %channels.1, i64 480, !dbg !6575
  %_7.i595 = getelementptr inbounds nuw i8, ptr %channels.1, i64 512, !dbg !6577
  %_14.i598 = getelementptr inbounds nuw i8, ptr %channels.1, i64 320, !dbg !6578
  %_17.i600 = getelementptr inbounds nuw i8, ptr %channels.1, i64 352, !dbg !6579
  %_20.i602 = getelementptr inbounds nuw i8, ptr %channels.1, i64 384, !dbg !6580
  %_23.i604 = getelementptr inbounds nuw i8, ptr %channels.1, i64 416, !dbg !6581
  %_26.i606 = getelementptr inbounds nuw i8, ptr %channels.1, i64 448, !dbg !6582
  %17 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1472, !dbg !6583
  %_91.i17 = load i32, ptr %17, align 8, !dbg !6583, !alias.scope !6535, !noalias !6544, !noundef !11
  %_90.i18 = zext i32 %_91.i17 to i64, !dbg !6583
  %18 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1472, !dbg !6588
  %_101.i19 = load i32, ptr %18, align 8, !dbg !6588, !alias.scope !6538, !noalias !6591, !noundef !11
  %_100.i20 = zext i32 %_101.i19 to i64, !dbg !6588
  %19 = icmp ne ptr %.sroa.6.0.copyload, null
  %20 = icmp ne ptr %.sroa.4.0.copyload, null
  %21 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1508
  %22 = getelementptr inbounds nuw i8, ptr %channels.0, i64 8
  %23 = getelementptr inbounds nuw i8, ptr %channels.0, i64 24
  %24 = getelementptr inbounds nuw i8, ptr %channels.0, i64 16
  %25 = getelementptr inbounds nuw i8, ptr %channels.1, i64 8
  %26 = getelementptr inbounds nuw i8, ptr %channels.1, i64 24
  %27 = getelementptr inbounds nuw i8, ptr %channels.1, i64 16
  %28 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1512
  %_73.i108 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1504
  %_77.i112 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1504
  %29 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1508
  %30 = add nuw nsw i64 %left.1, 1, !dbg !6592
  %31 = add nuw nsw i64 %right.1, 1, !dbg !6592
  br label %bb41.i24, !dbg !6592

bb41.i24:                                         ; preds = %bb7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit958
  %start1.sroa.0.0.i221998 = phi i64 [ 0, %bb7 ], [ %32, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit958 ]
  %32 = add nuw nsw i64 %start1.sroa.0.0.i221998, 1, !dbg !6601
; call <compressor::kernel::Channel<f32>>::advance_ramps
  tail call fastcc void @_RNvMNtCse3bfmKSZS8Y_10compressor6kernelINtB2_7ChannelfE13advance_rampsB4_(ptr noalias noundef nonnull align 8 dereferenceable(1520) %channels.0, i32 noundef %sample_rate) #22, !dbg !6609, !noalias !6611
; call <compressor::kernel::Channel<f32>>::advance_ramps
  tail call fastcc void @_RNvMNtCse3bfmKSZS8Y_10compressor6kernelINtB2_7ChannelfE13advance_rampsB4_(ptr noalias noundef nonnull align 8 dereferenceable(1520) %channels.1, i32 noundef %sample_rate) #22, !dbg !6612, !noalias !6611
  %_0.i936 = load float, ptr %_4.i617, align 4, !dbg !6613, !alias.scope !6620, !noalias !6623, !noundef !11
  %_0.i935 = load float, ptr %_7.i619, align 4, !dbg !6626, !alias.scope !6628, !noalias !6623, !noundef !11
  %_0.i934 = load float, ptr %_15.i13, align 4, !dbg !6631, !alias.scope !6633, !noalias !6623, !noundef !11
  %_0.i933 = load float, ptr %_14.i622, align 4, !dbg !6636, !alias.scope !6638, !noalias !6623, !noundef !11
  %_0.i932 = load float, ptr %_17.i624, align 4, !dbg !6641, !alias.scope !6643, !noalias !6623, !noundef !11
  %_0.i931 = load float, ptr %_20.i626, align 4, !dbg !6646, !alias.scope !6648, !noalias !6623, !noundef !11
  %_0.i9301610 = load i32, ptr %_23.i628, align 4, !dbg !6651, !alias.scope !6653, !noalias !6623, !noundef !11
  %_0.i9291611 = load i32, ptr %_26.i630, align 4, !dbg !6656, !alias.scope !6658, !noalias !6623, !noundef !11
  %_3.i731 = fcmp une float %_0.i935, 1.000000e+00, !dbg !6661
  %_3.i729 = fcmp oeq float %_0.i935, 0.000000e+00, !dbg !6664
  %_3.i727 = fcmp oeq float %_0.i936, 0.000000e+00, !dbg !6666
  %_0.i944 = load float, ptr %_4.i593, align 4, !dbg !6668, !alias.scope !6671, !noalias !6674, !noundef !11
  %_0.i943 = load float, ptr %_7.i595, align 4, !dbg !6677, !alias.scope !6679, !noalias !6674, !noundef !11
  %_0.i942 = load float, ptr %_17.i14, align 4, !dbg !6682, !alias.scope !6684, !noalias !6674, !noundef !11
  %_0.i941 = load float, ptr %_14.i598, align 4, !dbg !6687, !alias.scope !6689, !noalias !6674, !noundef !11
  %_0.i940 = load float, ptr %_17.i600, align 4, !dbg !6692, !alias.scope !6694, !noalias !6674, !noundef !11
  %_0.i939 = load float, ptr %_20.i602, align 4, !dbg !6697, !alias.scope !6699, !noalias !6674, !noundef !11
  %_0.i9381612 = load i32, ptr %_23.i604, align 4, !dbg !6702, !alias.scope !6704, !noalias !6674, !noundef !11
  %_0.i9371613 = load i32, ptr %_26.i606, align 4, !dbg !6707, !alias.scope !6709, !noalias !6674, !noundef !11
  %_3.i737 = fcmp une float %_0.i943, 1.000000e+00, !dbg !6712
  %_3.i735 = fcmp oeq float %_0.i943, 0.000000e+00, !dbg !6714
  %_3.i733 = fcmp oeq float %_0.i944, 0.000000e+00, !dbg !6716
  %exitcond = icmp eq i64 %start1.sroa.0.0.i221998, %30, !dbg !6718
  br i1 %exitcond, label %bb43.i127, label %bb44.i26, !dbg !6718, !prof !161

bb44.i26:                                         ; preds = %bb41.i24
  %_120.i28 = getelementptr inbounds nuw float, ptr %left.0, i64 %start1.sroa.0.0.i221998, !dbg !6724
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6729), !dbg !6732
  %_3.not.i979 = icmp eq i64 %left.1, %start1.sroa.0.0.i221998, !dbg !6733
  br i1 %_3.not.i979, label %panic.i981, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit982, !dbg !6733

panic.i981:                                       ; preds = %bb44.i26
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !6733, !noalias !6735
  unreachable, !dbg !6733

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit982: ; preds = %bb44.i26
  %_0.i980 = load float, ptr %_120.i28, align 4, !dbg !6733, !alias.scope !6729, !noalias !6611, !noundef !11
  %exitcond2333 = icmp eq i64 %start1.sroa.0.0.i221998, %31, !dbg !6736
  br i1 %exitcond2333, label %bb45.i126, label %bb46.i31, !dbg !6736, !prof !161

bb43.i127:                                        ; preds = %bb41.i24
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %30, i64 noundef range(i64 0, 2305843009213693952) %left.1, i64 noundef range(i64 0, 2305843009213693952) %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c46c6dde8c9e0eb71c8d3a58ebc90caa) #23, !dbg !6741, !noalias !6611
  unreachable, !dbg !6741

bb46.i31:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit982
  %_128.i33 = getelementptr inbounds nuw float, ptr %right.0, i64 %start1.sroa.0.0.i221998, !dbg !6742
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6747), !dbg !6750
  %_3.not.i975 = icmp eq i64 %right.1, %start1.sroa.0.0.i221998, !dbg !6751
  br i1 %_3.not.i975, label %panic.i977, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit978, !dbg !6751

panic.i977:                                       ; preds = %bb46.i31
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !6751, !noalias !6753
  unreachable, !dbg !6751

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit978: ; preds = %bb46.i31
  %_0.i976 = load float, ptr %_128.i33, align 4, !dbg !6751, !alias.scope !6747, !noalias !6611, !noundef !11
  switch i64 %.sroa.0.0.copyload, label %default.unreachable.i.i125 [
    i64 0, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i49
    i64 1, label %bb3.i.i124
    i64 2, label %bb2.i.i36
  ], !dbg !6754

default.unreachable.i.i125:                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit978
  unreachable

bb3.i.i124:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit978
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i49, !dbg !6758

bb2.i.i36:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit978
  tail call void @llvm.assume(i1 %19)
  %_25.i.i40 = icmp ugt i64 %start1.sroa.0.0.i221998, %.sroa.5.0.copyload, !dbg !6759
  br i1 %_25.i.i40, label %bb17.i.i123, label %bb18.i.i41, !dbg !6759, !prof !161

bb18.i.i41:                                       ; preds = %bb2.i.i36
  tail call void @llvm.assume(i1 %20)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6765), !dbg !6768
  %_3.not.i971 = icmp eq i64 %.sroa.5.0.copyload, %start1.sroa.0.0.i221998, !dbg !6769
  br i1 %_3.not.i971, label %panic.i973, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit974, !dbg !6769

panic.i973:                                       ; preds = %bb18.i.i41
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !6769, !noalias !6771
  unreachable, !dbg !6769

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit974: ; preds = %bb18.i.i41
  %_32.i.i44 = getelementptr inbounds nuw float, ptr %.sroa.4.0.copyload, i64 %start1.sroa.0.0.i221998, !dbg !6775
  %_0.i972 = load float, ptr %_32.i.i44, align 4, !dbg !6769, !alias.scope !6765, !noalias !6780, !noundef !11
  %_33.i.i45 = icmp ugt i64 %start1.sroa.0.0.i221998, %.sroa.7.0.copyload, !dbg !6781
  br i1 %_33.i.i45, label %bb19.i.i122, label %bb20.i.i46, !dbg !6781, !prof !161

bb17.i.i123:                                      ; preds = %bb2.i.i36
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 2305843009213693952) %start1.sroa.0.0.i221998, i64 noundef %.sroa.5.0.copyload, i64 noundef %.sroa.5.0.copyload, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7fe5a91ec575deda110c046c84630100) #23, !dbg !6785, !noalias !6780
  unreachable, !dbg !6785

bb20.i.i46:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit974
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6786), !dbg !6789
  %_3.not.i967 = icmp eq i64 %.sroa.7.0.copyload, %start1.sroa.0.0.i221998, !dbg !6790
  br i1 %_3.not.i967, label %panic.i969, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit970, !dbg !6790

panic.i969:                                       ; preds = %bb20.i.i46
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !6790, !noalias !6792
  unreachable, !dbg !6790

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit970: ; preds = %bb20.i.i46
  %_40.i.i48 = getelementptr inbounds nuw float, ptr %.sroa.6.0.copyload, i64 %start1.sroa.0.0.i221998, !dbg !6793
  %_0.i968 = load float, ptr %_40.i.i48, align 4, !dbg !6790, !alias.scope !6786, !noalias !6780, !noundef !11
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i49, !dbg !6798

bb19.i.i122:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit974
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 2305843009213693952) %start1.sroa.0.0.i221998, i64 noundef %.sroa.7.0.copyload, i64 noundef %.sroa.7.0.copyload, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f2cba1f89421a726339097d2eba48536) #23, !dbg !6799, !noalias !6780
  unreachable, !dbg !6799

_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i49: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit970, %bb3.i.i124, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit978
  %main_right.sroa.0.0.i.i50 = phi float [ %_0.i976, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit978 ], [ 0.000000e+00, %bb3.i.i124 ], [ %_0.i968, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit970 ]
  %main_left.sroa.0.0.i.i51 = phi float [ %_0.i980, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit978 ], [ 0.000000e+00, %bb3.i.i124 ], [ %_0.i972, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit970 ]
  %33 = tail call noundef float @llvm.fabs.f32(float %main_left.sroa.0.0.i.i51), !dbg !6800
  %34 = tail call noundef float @llvm.fabs.f32(float %main_right.sroa.0.0.i.i50), !dbg !6804
  %_3.i.i1439 = fcmp ule float %33, %34, !dbg !6807
  %_6.i.i1441 = bitcast float %33 to i32, !dbg !6814
  %_8.i.i1443 = bitcast float %34 to i32, !dbg !6819
  %_4.i.i1446 = select i1 %_3.i.i1439, i32 %_8.i.i1443, i32 %_6.i.i1441, !dbg !6821
  %_0.i874 = fmul float %33, 5.000000e-01, !dbg !6822
  %_0.i873 = fmul float %34, 5.000000e-01, !dbg !6826
  %_0.i803 = fadd float %_0.i873, %_0.i874, !dbg !6828
  %_6.i1235 = bitcast float %_0.i803 to i32, !dbg !6831
  %_4.i1240 = select i1 %.not, i32 %_6.i1235, i32 %_4.i.i1446, !dbg !6835
  %_4.i1233 = select i1 %16, i32 %_6.i.i1441, i32 %_4.i1240, !dbg !6836
  %_4.i1226 = select i1 %16, i32 %_8.i.i1443, i32 %_4.i1240, !dbg !6839
  %_39.i64 = load i32, ptr %21, align 4, !dbg !6841, !alias.scope !6535, !noalias !6544, !noundef !11
  %write.i65 = zext i32 %_39.i64 to i64, !dbg !6841
  %35 = add nuw nsw i64 %write.i65, 1, !dbg !6843
  %_41.i66 = icmp eq i64 %35, %ring_length.i12, !dbg !6845
  %spec.store.select.i67 = select i1 %_41.i66, i64 0, i64 %35, !dbg !6845
  %_189.1.i68 = load i64, ptr %22, align 8, !dbg !6847, !alias.scope !6535, !noalias !6544, !noundef !11
  %_129.i69 = icmp ult i64 %_189.1.i68, %write.i65, !dbg !6849
  br i1 %_129.i69, label %bb47.i121, label %bb48.i70, !dbg !6849, !prof !161

bb45.i126:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit982
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %31, i64 noundef range(i64 0, 2305843009213693952) %right.1, i64 noundef range(i64 0, 2305843009213693952) %right.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9b27971f08fa00763b6dc53a9f4dbd4c) #23, !dbg !6854, !noalias !6611
  unreachable, !dbg !6854

bb48.i70:                                         ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i49
  %_189.0.i71 = load ptr, ptr %channels.0, align 8, !dbg !6847, !alias.scope !6535, !noalias !6544, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6855), !dbg !6858
  %_4.not.i1048 = icmp eq i64 %_189.1.i68, %write.i65, !dbg !6859
  br i1 %_4.not.i1048, label %panic.i1049, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1050, !dbg !6859

panic.i1049:                                      ; preds = %bb48.i70
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #23, !dbg !6859, !noalias !6862
  unreachable, !dbg !6859

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1050: ; preds = %bb48.i70
  %_136.i73 = getelementptr inbounds nuw float, ptr %_189.0.i71, i64 %write.i65, !dbg !6863
  store float %_0.i980, ptr %_136.i73, align 4, !dbg !6859, !alias.scope !6855, !noalias !6611
  %_190.1.i74 = load i64, ptr %23, align 8, !dbg !6868, !alias.scope !6535, !noalias !6544, !noundef !11
  %_137.i75 = icmp ult i64 %_190.1.i74, %write.i65, !dbg !6869
  br i1 %_137.i75, label %bb49.i120, label %bb50.i76, !dbg !6869, !prof !161

bb47.i121:                                        ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i49
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i65, i64 noundef %_189.1.i68, i64 noundef %_189.1.i68, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bb9110d0bc8cedfe643d9bd892c8c620) #23, !dbg !6873, !noalias !6611
  unreachable, !dbg !6873

bb50.i76:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1050
  %_190.0.i77 = load ptr, ptr %24, align 8, !dbg !6868, !alias.scope !6535, !noalias !6544, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6874), !dbg !6877
  %_4.not.i1045 = icmp eq i64 %_190.1.i74, %write.i65, !dbg !6878
  br i1 %_4.not.i1045, label %panic.i1046, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1047, !dbg !6878

panic.i1046:                                      ; preds = %bb50.i76
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #23, !dbg !6878, !noalias !6880
  unreachable, !dbg !6878

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1047: ; preds = %bb50.i76
  %_144.i79 = getelementptr inbounds nuw float, ptr %_190.0.i77, i64 %write.i65, !dbg !6881
  store i32 %_4.i1233, ptr %_144.i79, align 4, !dbg !6878, !alias.scope !6874, !noalias !6611
  %_191.1.i80 = load i64, ptr %25, align 8, !dbg !6886, !alias.scope !6538, !noalias !6591, !noundef !11
  %_145.i81 = icmp ult i64 %_191.1.i80, %write.i65, !dbg !6887
  br i1 %_145.i81, label %bb51.i119, label %bb52.i82, !dbg !6887, !prof !161

bb49.i120:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1050
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i65, i64 noundef %_190.1.i74, i64 noundef %_190.1.i74, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_da5a912ec6b29b38396e4eb206c99989) #23, !dbg !6891, !noalias !6611
  unreachable, !dbg !6891

bb52.i82:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1047
  %_191.0.i83 = load ptr, ptr %channels.1, align 8, !dbg !6886, !alias.scope !6538, !noalias !6591, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6892), !dbg !6895
  %_4.not.i1042 = icmp eq i64 %_191.1.i80, %write.i65, !dbg !6896
  br i1 %_4.not.i1042, label %panic.i1043, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1044, !dbg !6896

panic.i1043:                                      ; preds = %bb52.i82
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #23, !dbg !6896, !noalias !6898
  unreachable, !dbg !6896

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1044: ; preds = %bb52.i82
  %_152.i85 = getelementptr inbounds nuw float, ptr %_191.0.i83, i64 %write.i65, !dbg !6899
  store float %_0.i976, ptr %_152.i85, align 4, !dbg !6896, !alias.scope !6892, !noalias !6611
  %_192.1.i86 = load i64, ptr %26, align 8, !dbg !6904, !alias.scope !6538, !noalias !6591, !noundef !11
  %_153.i87 = icmp ult i64 %_192.1.i86, %write.i65, !dbg !6905
  br i1 %_153.i87, label %bb53.i118, label %bb54.i88, !dbg !6905, !prof !161

bb51.i119:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1047
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i65, i64 noundef %_191.1.i80, i64 noundef %_191.1.i80, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b46755d5101951cde0ca1c710062f433) #23, !dbg !6909, !noalias !6611
  unreachable, !dbg !6909

bb54.i88:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1044
  %_192.0.i89 = load ptr, ptr %27, align 8, !dbg !6904, !alias.scope !6538, !noalias !6591, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6910), !dbg !6913
  %_4.not.i1039 = icmp eq i64 %_192.1.i86, %write.i65, !dbg !6914
  br i1 %_4.not.i1039, label %panic.i1040, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1041, !dbg !6914

panic.i1040:                                      ; preds = %bb54.i88
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #23, !dbg !6914, !noalias !6916
  unreachable, !dbg !6914

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1041: ; preds = %bb54.i88
  %_160.i91 = getelementptr inbounds nuw float, ptr %_192.0.i89, i64 %write.i65, !dbg !6917
  store i32 %_4.i1226, ptr %_160.i91, align 4, !dbg !6914, !alias.scope !6910, !noalias !6611
  %_193.1.i92 = load i64, ptr %22, align 8, !dbg !6922, !alias.scope !6535, !noalias !6544, !noundef !11
  %_161.i93 = icmp ugt i64 %spec.store.select.i67, %_193.1.i92, !dbg !6923
  br i1 %_161.i93, label %bb55.i117, label %bb56.i94, !dbg !6923, !prof !161

bb53.i118:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1044
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i65, i64 noundef %_192.1.i86, i64 noundef %_192.1.i86, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2fb73b04dc26e2fd60c7be20365496fe) #23, !dbg !6927, !noalias !6611
  unreachable, !dbg !6927

bb56.i94:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1041
  %_193.0.i95 = load ptr, ptr %channels.0, align 8, !dbg !6922, !alias.scope !6535, !noalias !6544, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6928), !dbg !6931
  %_3.not.i963 = icmp eq i64 %_193.1.i92, %spec.store.select.i67, !dbg !6932
  br i1 %_3.not.i963, label %panic.i965, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit966, !dbg !6932

panic.i965:                                       ; preds = %bb56.i94
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !6932, !noalias !6934
  unreachable, !dbg !6932

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit966: ; preds = %bb56.i94
  %_168.i97 = getelementptr inbounds nuw float, ptr %_193.0.i95, i64 %spec.store.select.i67, !dbg !6935
  %_0.i964 = load float, ptr %_168.i97, align 4, !dbg !6932, !alias.scope !6928, !noalias !6611, !noundef !11
  %_194.1.i99 = load i64, ptr %25, align 8, !dbg !6940, !alias.scope !6538, !noalias !6591, !noundef !11
  %_169.i100 = icmp ugt i64 %spec.store.select.i67, %_194.1.i99, !dbg !6942
  br i1 %_169.i100, label %bb57.i116, label %bb58.i101, !dbg !6942, !prof !161

bb55.i117:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1041
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %spec.store.select.i67, i64 noundef %_193.1.i92, i64 noundef %_193.1.i92, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_322bbb2a85bd2347c4d3c1d32e8c0bc7) #23, !dbg !6946, !noalias !6611
  unreachable, !dbg !6946

bb58.i101:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit966
  %_194.0.i102 = load ptr, ptr %channels.1, align 8, !dbg !6940, !alias.scope !6538, !noalias !6591, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6947), !dbg !6950
  %_3.not.i959 = icmp eq i64 %_194.1.i99, %spec.store.select.i67, !dbg !6951
  br i1 %_3.not.i959, label %panic.i961, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit962, !dbg !6951

panic.i961:                                       ; preds = %bb58.i101
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !6951, !noalias !6953
  unreachable, !dbg !6951

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit962: ; preds = %bb58.i101
  %_176.i104 = getelementptr inbounds nuw float, ptr %_194.0.i102, i64 %spec.store.select.i67, !dbg !6954
  %_0.i960 = load float, ptr %_176.i104, align 4, !dbg !6951, !alias.scope !6947, !noalias !6611, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6959), !dbg !6962
  %_6.i176 = load i32, ptr %15, align 8, !dbg !6964, !alias.scope !6959, !noalias !6611, !noundef !11
  %_10.not.i178 = icmp ult i32 %_39.i64, %_91.i17, !dbg !6968
  %narrow = select i1 %_10.not.i178, i32 %_6.i176, i32 0, !dbg !6968
  %_11.i179 = zext i32 %narrow to i64, !dbg !6968
  %write.pn.i180 = sub nsw i64 %write.i65, %_90.i18, !dbg !6968
  %row.sroa.0.0.i181 = add nsw i64 %write.pn.i180, %_11.i179, !dbg !6971
  %_55.1.i182 = load i64, ptr %23, align 8, !dbg !6972, !alias.scope !6959, !noalias !6611, !noundef !11
  %_40.i183 = icmp ugt i64 %row.sroa.0.0.i181, %_55.1.i182, !dbg !6974
  br i1 %_40.i183, label %bb19.i187, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit188, !dbg !6974, !prof !161

bb19.i187:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit962
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i181, i64 noundef %_55.1.i182, i64 noundef %_55.1.i182, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_942b76445104c042a726096e85fe3fcb) #23, !dbg !6979, !noalias !6980
  unreachable, !dbg !6979

_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit188: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit962
  %_55.0.i184 = load ptr, ptr %24, align 8, !dbg !6972, !alias.scope !6959, !noalias !6611, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6981), !dbg !6984
  %_3.not.i951 = icmp eq i64 %_55.1.i182, %row.sroa.0.0.i181, !dbg !6985
  br i1 %_3.not.i951, label %panic.i953, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit954, !dbg !6985

panic.i953:                                       ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit188
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !6985, !noalias !6987
  unreachable, !dbg !6985

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit954: ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit188
  %_47.i186 = getelementptr inbounds nuw float, ptr %_55.0.i184, i64 %row.sroa.0.0.i181, !dbg !6988
  %_0.i952 = load float, ptr %_47.i186, align 4, !dbg !6985, !alias.scope !6981, !noalias !6980, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6993), !dbg !6996
  %_6.i = load i32, ptr %28, align 8, !dbg !6998, !alias.scope !6993, !noalias !6611, !noundef !11
  %_10.not.i = icmp ult i32 %_39.i64, %_101.i19, !dbg !7000
  %narrow1618 = select i1 %_10.not.i, i32 %_6.i, i32 0, !dbg !7000
  %_11.i175 = zext i32 %narrow1618 to i64, !dbg !7000
  %write.pn.i = sub nsw i64 %write.i65, %_100.i20, !dbg !7000
  %row.sroa.0.0.i = add nsw i64 %write.pn.i, %_11.i175, !dbg !7001
  %_55.1.i = load i64, ptr %26, align 8, !dbg !7002, !alias.scope !6993, !noalias !6611, !noundef !11
  %_40.i = icmp ugt i64 %row.sroa.0.0.i, %_55.1.i, !dbg !7003
  br i1 %_40.i, label %bb19.i, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit, !dbg !7003, !prof !161

bb19.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit954
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i, i64 noundef %_55.1.i, i64 noundef %_55.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_942b76445104c042a726096e85fe3fcb) #23, !dbg !7006, !noalias !7007
  unreachable, !dbg !7006

_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit954
  %_55.0.i = load ptr, ptr %27, align 8, !dbg !7002, !alias.scope !6993, !noalias !6611, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7008), !dbg !7011
  %_3.not.i955 = icmp eq i64 %_55.1.i, %row.sroa.0.0.i, !dbg !7012
  br i1 %_3.not.i955, label %panic.i957, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit958, !dbg !7012

panic.i957:                                       ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !7012, !noalias !7014
  unreachable, !dbg !7012

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit958: ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit
  %_47.i = getelementptr inbounds nuw float, ptr %_55.0.i, i64 %row.sroa.0.0.i, !dbg !7015
  %_0.i956 = load float, ptr %_47.i, align 4, !dbg !7012, !alias.scope !7008, !noalias !7007, !noundef !11
  %_3.i.i1403 = fcmp ule float %_0.i952, 0x3E45798EE0000000, !dbg !7017
  %_6.i.i1405 = bitcast float %_0.i952 to i32, !dbg !7025
  %_4.i.i1410 = select i1 %_3.i.i1403, i32 841731191, i32 %_6.i.i1405, !dbg !7028
  %_0.i.i1411 = bitcast i32 %_4.i.i1410 to float, !dbg !7029
  %_3.i.i1276 = fcmp ule float %_0.i.i1411, 0x3810000000000000, !dbg !7032
  %_4.i.i1282 = select i1 %_3.i.i1276, i32 8388608, i32 %_4.i.i1410, !dbg !7040
  %_5.i1012 = and i32 %_4.i.i1282, 8388607, !dbg !7042
  %_4.i1013 = or disjoint i32 %_5.i1012, 1065353216, !dbg !7042
  %significand.i1014 = bitcast i32 %_4.i1013 to float, !dbg !7047
  %_0.i878 = fadd float %significand.i1014, -1.000000e+00, !dbg !7050
  %_0.i824 = fmul float %_0.i878, 0xBF9B17A960000000, !dbg !7054
  %_0.i782 = fadd float %_0.i824, 0x3FBF9A8440000000, !dbg !7059
  %_0.i824.1 = fmul float %_0.i878, %_0.i782, !dbg !7054
  %_0.i782.1 = fadd float %_0.i824.1, 0xBFD1E3F400000000, !dbg !7059
  %_0.i824.2 = fmul float %_0.i878, %_0.i782.1, !dbg !7054
  %_0.i782.2 = fadd float %_0.i824.2, 0x3FDD544F20000000, !dbg !7059
  %_0.i824.3 = fmul float %_0.i878, %_0.i782.2, !dbg !7054
  %_0.i782.3 = fadd float %_0.i824.3, 0xBFE6FC2A60000000, !dbg !7059
  %_0.i824.4 = fmul float %_0.i878, %_0.i782.3, !dbg !7054
  %_0.i782.4 = fadd float %_0.i824.4, 0x3FF714B2A0000000, !dbg !7059
  %_9.i1015 = lshr i32 %_4.i.i1282, 23, !dbg !7061
  %_8.i1016 = or disjoint i32 %_9.i1015, 1258291200, !dbg !7061
  %_7.i1017 = bitcast i32 %_8.i1016 to float, !dbg !7063
  %exponent.i1018 = fadd float %_7.i1017, 0xC160000FE0000000, !dbg !7065
  %_0.i823 = fmul float %_0.i878, %_0.i782.4, !dbg !7066
  %_0.i781 = fadd float %exponent.i1018, %_0.i823, !dbg !7068
  %_0.i871 = fmul float %_0.i781, 0x4018151820000000, !dbg !7070
  %_3.i.i1394.inv = fcmp ogt float %_0.i871, -1.600000e+02, !dbg !7072
  %_0.i.i1402 = select i1 %_3.i.i1394.inv, float %_0.i871, float -1.600000e+02, !dbg !7072
  %_3.i.i1534.inv = fcmp olt float %_0.i.i1402, 2.400000e+01, !dbg !7075
  %_0.i.i1542 = select i1 %_3.i.i1534.inv, float %_0.i.i1402, float 2.400000e+01, !dbg !7075
  %_0.i886 = fsub float %_0.i.i1542, %_0.i934, !dbg !7079
  %_3.i749 = fcmp ule float %_0.i886, %_0.i932, !dbg !7084
  %36 = fneg float %_0.i932, !dbg !7087
  %_0.i796 = fadd float %_0.i932, %_0.i886, !dbg !7091
  %_0.i847 = fmul float %_0.i796, %_0.i796, !dbg !7095
  %_0.i846 = fmul float %_0.i931, %_0.i847, !dbg !7098
  %_4.i1109.v.v = select i1 %_3.i749, float %_0.i846, float %_0.i886, !dbg !7100
  %_4.i1109.v = fmul float %_0.i933, %_4.i1109.v.v, !dbg !7100
  %_4.i1109 = bitcast float %_4.i1109.v to i32, !dbg !7100
  %37 = fcmp ugt float %_0.i886, %36, !dbg !7103
  %_7.i1101 = select i1 %37, i32 %_4.i1109, i32 0, !dbg !7105
  %_0.i1103 = bitcast i32 %_7.i1101 to float, !dbg !7106
  %_3.i.i1385 = fcmp ule float %_0.i1103, -1.000000e+02, !dbg !7108
  %38 = bitcast i32 %_7.i1101 to float, !dbg !7111
  %_0.i.i1393 = select i1 %_3.i.i1385, float -1.000000e+02, float %38, !dbg !7114
  %_3.i.i1525 = fcmp olt float %_0.i.i1393, 0.000000e+00, !dbg !7115
  %_0.i.i1533 = select i1 %_3.i.i1525, float %_0.i.i1393, float 0.000000e+00, !dbg !7119
  %_6.i297 = load float, ptr %_73.i108, align 4, !dbg !7121, !alias.scope !7125, !noalias !7128, !noundef !11
  %_3.i773 = fcmp uge float %_0.i.i1533, %_6.i297, !dbg !7130
  %_4.i1156 = select i1 %_3.i773, i32 %_0.i9291611, i32 %_0.i9301610, !dbg !7132
  %_0.i1157 = bitcast i32 %_4.i1156 to float, !dbg !7134
  %_0.i891 = fsub float %_0.i.i1533, %_6.i297, !dbg !7136
  %_4.i809 = fmul float %_0.i891, %_0.i1157, !dbg !7141
  %_0.i810 = fadd float %_6.i297, %_4.i809, !dbg !7141
  %39 = tail call noundef float @llvm.fabs.f32(float %_0.i810), !dbg !7145
  %40 = fcmp uge float %39, 0x3BC79CA100000000, !dbg !7149
  %_0.i1080 = select i1 %40, float %_0.i810, float 0.000000e+00, !dbg !7152
  store float %_0.i1080, ptr %_73.i108, align 4, !dbg !7153, !alias.scope !7125, !noalias !7128
  %_0.i801 = fadd float %_0.i936, %_0.i1080, !dbg !7155
  %_0.i865 = fmul float %_0.i801, 0x3FC542A5A0000000, !dbg !7160
  %_3.i.i1308.inv = fcmp ogt float %_0.i865, -1.260000e+02, !dbg !7164
  %_0.i.i1315 = select i1 %_3.i.i1308.inv, float %_0.i865, float -1.260000e+02, !dbg !7164
  %_3.i.i1465.inv = fcmp olt float %_0.i.i1315, 1.270000e+02, !dbg !7169
  %_0.i.i1472 = select i1 %_3.i.i1465.inv, float %_0.i.i1315, float 1.270000e+02, !dbg !7169
  %41 = tail call noundef float @llvm.floor.f32(float %_0.i.i1472), !dbg !7172
  %_0.i882 = fsub float %_0.i.i1472, %41, !dbg !7184
  %_3.i.i1430 = fcmp ule float %_0.i956, 0x3E45798EE0000000, !dbg !7187
  %_6.i.i1432 = bitcast float %_0.i956 to i32, !dbg !7193
  %_4.i.i1437 = select i1 %_3.i.i1430, i32 841731191, i32 %_6.i.i1432, !dbg !7196
  %_0.i.i1438 = bitcast i32 %_4.i.i1437 to float, !dbg !7197
  %_3.i.i = fcmp ule float %_0.i.i1438, 0x3810000000000000, !dbg !7199
  %_4.i.i = select i1 %_3.i.i, i32 8388608, i32 %_4.i.i1437, !dbg !7204
  %_5.i1007 = and i32 %_4.i.i, 8388607, !dbg !7206
  %_4.i1008 = or disjoint i32 %_5.i1007, 1065353216, !dbg !7206
  %significand.i = bitcast i32 %_4.i1008 to float, !dbg !7208
  %_0.i877 = fadd float %significand.i, -1.000000e+00, !dbg !7210
  %_0.i822 = fmul float %_0.i877, 0xBF9B17A960000000, !dbg !7212
  %_0.i780 = fadd float %_0.i822, 0x3FBF9A8440000000, !dbg !7214
  %_0.i822.1 = fmul float %_0.i877, %_0.i780, !dbg !7212
  %_0.i780.1 = fadd float %_0.i822.1, 0xBFD1E3F400000000, !dbg !7214
  %_0.i822.2 = fmul float %_0.i877, %_0.i780.1, !dbg !7212
  %_0.i780.2 = fadd float %_0.i822.2, 0x3FDD544F20000000, !dbg !7214
  %_0.i822.3 = fmul float %_0.i877, %_0.i780.2, !dbg !7212
  %_0.i780.3 = fadd float %_0.i822.3, 0xBFE6FC2A60000000, !dbg !7214
  %_0.i822.4 = fmul float %_0.i877, %_0.i780.3, !dbg !7212
  %_0.i780.4 = fadd float %_0.i822.4, 0x3FF714B2A0000000, !dbg !7214
  %_9.i1009 = lshr i32 %_4.i.i, 23, !dbg !7216
  %_8.i = or disjoint i32 %_9.i1009, 1258291200, !dbg !7216
  %_7.i1010 = bitcast i32 %_8.i to float, !dbg !7217
  %exponent.i = fadd float %_7.i1010, 0xC160000FE0000000, !dbg !7219
  %_0.i821 = fmul float %_0.i877, %_0.i780.4, !dbg !7220
  %_0.i779 = fadd float %exponent.i, %_0.i821, !dbg !7222
  %_0.i872 = fmul float %_0.i779, 0x4018151820000000, !dbg !7224
  %_3.i.i1421.inv = fcmp ogt float %_0.i872, -1.600000e+02, !dbg !7226
  %_0.i.i1429 = select i1 %_3.i.i1421.inv, float %_0.i872, float -1.600000e+02, !dbg !7226
  %_3.i.i1552.inv = fcmp olt float %_0.i.i1429, 2.400000e+01, !dbg !7229
  %_0.i.i1560 = select i1 %_3.i.i1552.inv, float %_0.i.i1429, float 2.400000e+01, !dbg !7229
  %_0.i885 = fsub float %_0.i.i1560, %_0.i942, !dbg !7232
  %_3.i747 = fcmp ule float %_0.i885, %_0.i940, !dbg !7235
  %42 = fneg float %_0.i940, !dbg !7237
  %_0.i795 = fadd float %_0.i940, %_0.i885, !dbg !7239
  %_0.i843 = fmul float %_0.i795, %_0.i795, !dbg !7241
  %_0.i842 = fmul float %_0.i939, %_0.i843, !dbg !7243
  %_4.i1096.v.v = select i1 %_3.i747, float %_0.i842, float %_0.i885, !dbg !7245
  %_4.i1096.v = fmul float %_0.i941, %_4.i1096.v.v, !dbg !7245
  %_4.i1096 = bitcast float %_4.i1096.v to i32, !dbg !7245
  %43 = fcmp ugt float %_0.i885, %42, !dbg !7247
  %_7.i1088 = select i1 %43, i32 %_4.i1096, i32 0, !dbg !7249
  %_0.i1090 = bitcast i32 %_7.i1088 to float, !dbg !7250
  %_3.i.i1412 = fcmp ule float %_0.i1090, -1.000000e+02, !dbg !7252
  %44 = bitcast i32 %_7.i1088 to float, !dbg !7255
  %_0.i.i1420 = select i1 %_3.i.i1412, float -1.000000e+02, float %44, !dbg !7258
  %_3.i.i1543 = fcmp olt float %_0.i.i1420, 0.000000e+00, !dbg !7259
  %_0.i.i1551 = select i1 %_3.i.i1543, float %_0.i.i1420, float 0.000000e+00, !dbg !7262
  %_6.i287 = load float, ptr %_77.i112, align 4, !dbg !7264, !alias.scope !7267, !noalias !7270, !noundef !11
  %_3.i777 = fcmp uge float %_0.i.i1551, %_6.i287, !dbg !7272
  %_4.i1163 = select i1 %_3.i777, i32 %_0.i9371613, i32 %_0.i9381612, !dbg !7274
  %_0.i1164 = bitcast i32 %_4.i1163 to float, !dbg !7276
  %_0.i892 = fsub float %_0.i.i1551, %_6.i287, !dbg !7278
  %_4.i811 = fmul float %_0.i892, %_0.i1164, !dbg !7281
  %_0.i812 = fadd float %_6.i287, %_4.i811, !dbg !7281
  %45 = tail call noundef float @llvm.fabs.f32(float %_0.i812), !dbg !7283
  %46 = fcmp uge float %45, 0x3BC79CA100000000, !dbg !7286
  %_0.i1084 = select i1 %46, float %_0.i812, float 0.000000e+00, !dbg !7288
  store float %_0.i1084, ptr %_77.i112, align 4, !dbg !7289, !alias.scope !7267, !noalias !7270
  %_0.i802 = fadd float %_0.i944, %_0.i1084, !dbg !7290
  %_0.i868 = fmul float %_0.i802, 0x3FC542A5A0000000, !dbg !7294
  %_3.i.i1300.inv = fcmp ogt float %_0.i868, -1.260000e+02, !dbg !7297
  %_0.i.i1307 = select i1 %_3.i.i1300.inv, float %_0.i868, float -1.260000e+02, !dbg !7297
  %_3.i.i1457.inv = fcmp olt float %_0.i.i1307, 1.270000e+02, !dbg !7301
  %_0.i.i1464 = select i1 %_3.i.i1457.inv, float %_0.i.i1307, float 1.270000e+02, !dbg !7301
  %47 = tail call noundef float @llvm.floor.f32(float %_0.i.i1464), !dbg !7304
  %_0.i881 = fsub float %_0.i.i1464, %47, !dbg !7308
  %_0.i831 = fmul float %_0.i881, 0x3F5E974FA0000000, !dbg !7310
  %_0.i788 = fadd float %_0.i831, 0x3F82778560000000, !dbg !7315
  %_0.i831.1 = fmul float %_0.i881, %_0.i788, !dbg !7310
  %_0.i788.1 = fadd float %_0.i831.1, 0x3FAC91CE60000000, !dbg !7315
  %_0.i831.2 = fmul float %_0.i881, %_0.i788.1, !dbg !7310
  %_0.i788.2 = fadd float %_0.i831.2, 0x3FCEBDB560000000, !dbg !7315
  %_0.i831.3 = fmul float %_0.i881, %_0.i788.2, !dbg !7310
  %_0.i788.3 = fadd float %_0.i831.3, 0x3FE62E4BA0000000, !dbg !7315
  %_0.i834 = fmul float %_0.i882, 0x3F5E974FA0000000, !dbg !7317
  %_0.i790 = fadd float %_0.i834, 0x3F82778560000000, !dbg !7319
  %_0.i834.1 = fmul float %_0.i882, %_0.i790, !dbg !7317
  %_0.i790.1 = fadd float %_0.i834.1, 0x3FAC91CE60000000, !dbg !7319
  %_0.i834.2 = fmul float %_0.i882, %_0.i790.1, !dbg !7317
  %_0.i790.2 = fadd float %_0.i834.2, 0x3FCEBDB560000000, !dbg !7319
  %_0.i834.3 = fmul float %_0.i882, %_0.i790.2, !dbg !7317
  %_0.i790.3 = fadd float %_0.i834.3, 0x3FE62E4BA0000000, !dbg !7319
  %_0.i833 = fmul float %_0.i882, %_0.i790.3, !dbg !7321
  %_0.i789 = fadd float %_0.i833, 1.000000e+00, !dbg !7323
  %biased.i692 = fadd float %41, 0x4160000FE0000000, !dbg !7325
  %_4.i693 = bitcast float %biased.i692 to i32, !dbg !7329
  %_3.i694 = shl i32 %_4.i693, 23, !dbg !7333
  %_0.i695 = bitcast i32 %_3.i694 to float, !dbg !7334
  %_0.i832 = fmul float %_0.i789, %_0.i695, !dbg !7337
  %_0.i864 = fmul float %_0.i964, %_0.i832, !dbg !7339
  %_0.i895 = fsub float %_0.i864, %_0.i964, !dbg !7342
  %_4.i817 = fmul float %_0.i935, %_0.i895, !dbg !7348
  %_0.i818 = fadd float %_0.i964, %_4.i817, !dbg !7348
  %_3.i743 = fcmp oeq float %_0.i1080, 0.000000e+00, !dbg !7351
  %_0.i12731628 = and i1 %_3.i727, %_3.i743, !dbg !7354
  %_0.i12681629 = or i1 %_3.i729, %_0.i12731628, !dbg !7357
  %_0.i12671630 = or i1 %bypass, %_0.i12681629, !dbg !7360
  %_4.i1205.v = select i1 %_3.i731, float %_0.i818, float %_0.i864, !dbg !7362
  %_4.i1198.v = select i1 %_0.i12671630, float %_0.i964, float %_4.i1205.v, !dbg !7365
  %_0.i830 = fmul float %_0.i881, %_0.i788.3, !dbg !7368
  %_0.i787 = fadd float %_0.i830, 1.000000e+00, !dbg !7370
  %biased.i = fadd float %47, 0x4160000FE0000000, !dbg !7372
  %_4.i689 = bitcast float %biased.i to i32, !dbg !7374
  %_3.i690 = shl i32 %_4.i689, 23, !dbg !7376
  %_0.i691 = bitcast i32 %_3.i690 to float, !dbg !7377
  %_0.i829 = fmul float %_0.i787, %_0.i691, !dbg !7379
  %_0.i867 = fmul float %_0.i960, %_0.i829, !dbg !7381
  %_0.i896 = fsub float %_0.i867, %_0.i960, !dbg !7383
  %_4.i819 = fmul float %_0.i943, %_0.i896, !dbg !7386
  %_0.i820 = fadd float %_0.i960, %_4.i819, !dbg !7386
  %_3.i745 = fcmp oeq float %_0.i1084, 0.000000e+00, !dbg !7388
  %_0.i12741642 = and i1 %_3.i733, %_3.i745, !dbg !7390
  %_0.i12701643 = or i1 %_3.i735, %_0.i12741642, !dbg !7392
  %_0.i12691644 = or i1 %bypass, %_0.i12701643, !dbg !7394
  %_4.i1219.v = select i1 %_3.i737, float %_0.i820, float %_0.i867, !dbg !7396
  %_4.i1212.v = select i1 %_0.i12691644, float %_0.i960, float %_4.i1219.v, !dbg !7398
  store float %_4.i1198.v, ptr %_120.i28, align 4, !dbg !7400, !alias.scope !7403, !noalias !6611
  store float %_4.i1212.v, ptr %_128.i33, align 4, !dbg !7406, !alias.scope !7408, !noalias !6611
  %48 = trunc i64 %spec.store.select.i67 to i32, !dbg !7411
  store i32 %48, ptr %21, align 4, !dbg !7411, !alias.scope !6535, !noalias !6544
  store i32 %48, ptr %29, align 4, !dbg !7412, !alias.scope !6538, !noalias !6591
  %exitcond2334.not = icmp eq i64 %32, %spec.store.select, !dbg !7413
  br i1 %exitcond2334.not, label %bb10, label %bb41.i24, !dbg !6592

bb57.i116:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit966
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %spec.store.select.i67, i64 noundef %_194.1.i99, i64 noundef %_194.1.i99, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b5bf0b379ba879ef3901108f927dd528) #23, !dbg !7417, !noalias !6611
  unreachable, !dbg !7417

bb10:                                             ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit958, %start
  br i1 %_24, label %bb11, label %bb19, !dbg !7418

bb11:                                             ; preds = %bb10
  %_34 = sub i64 %frames, %spec.store.select, !dbg !7419
  %49 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1472, !dbg !7420
  %channels.0.val = load i32, ptr %49, align 4, !dbg !7420
  %50 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1472, !dbg !7420
  %channels.1.val = load i32, ptr %50, align 4, !dbg !7420
  %_4.i1576 = icmp ult i64 %_34, 129, !dbg !7421
  %_0.i4.i = zext i32 %channels.0.val to i64
  %_5.not.i = icmp samesign ule i64 %_34, %_0.i4.i
  %or.cond.i.not1647 = select i1 %_4.i1576, i1 %_5.not.i, i1 false, !dbg !7421
  %_0.i.i1577 = zext i32 %channels.1.val to i64
  %51 = icmp samesign ule i64 %_34, %_0.i.i1577
  %or.cond = select i1 %or.cond.i.not1647, i1 %51, i1 false, !dbg !7421
  br i1 %or.cond, label %bb13, label %bb41.i.lr.ph, !dbg !7421

bb19:                                             ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit950, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit373.i, %bb10
  ret void, !dbg !7424

bb41.i.lr.ph:                                     ; preds = %bb11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7425), !dbg !7428
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7429), !dbg !7428
  %52 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1512, !dbg !7431
  %_12.i = load i32, ptr %52, align 8, !dbg !7431, !alias.scope !7425, !noalias !7435, !noundef !11
  %_15.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 288, !dbg !7439
  %_4.i665 = getelementptr inbounds nuw i8, ptr %channels.0, i64 480, !dbg !7442
  %_0.i904 = load float, ptr %_4.i665, align 4, !dbg !7444, !alias.scope !7446, !noalias !7449, !noundef !11
  %_7.i667 = getelementptr inbounds nuw i8, ptr %channels.0, i64 512, !dbg !7452
  %_0.i903 = load float, ptr %_7.i667, align 4, !dbg !7453, !alias.scope !7455, !noalias !7449, !noundef !11
  %_0.i902 = load float, ptr %_15.i, align 4, !dbg !7458, !alias.scope !7460, !noalias !7449, !noundef !11
  %_14.i670 = getelementptr inbounds nuw i8, ptr %channels.0, i64 320, !dbg !7463
  %_0.i901 = load float, ptr %_14.i670, align 4, !dbg !7464, !alias.scope !7466, !noalias !7449, !noundef !11
  %_17.i672 = getelementptr inbounds nuw i8, ptr %channels.0, i64 352, !dbg !7469
  %_0.i900 = load float, ptr %_17.i672, align 4, !dbg !7470, !alias.scope !7472, !noalias !7449, !noundef !11
  %_20.i674 = getelementptr inbounds nuw i8, ptr %channels.0, i64 384, !dbg !7475
  %_0.i899 = load float, ptr %_20.i674, align 4, !dbg !7476, !alias.scope !7478, !noalias !7449, !noundef !11
  %_23.i676 = getelementptr inbounds nuw i8, ptr %channels.0, i64 416, !dbg !7481
  %_0.i8981648 = load i32, ptr %_23.i676, align 4, !dbg !7482, !alias.scope !7484, !noalias !7449, !noundef !11
  %_26.i678 = getelementptr inbounds nuw i8, ptr %channels.0, i64 448, !dbg !7487
  %_0.i8971649 = load i32, ptr %_26.i678, align 4, !dbg !7488, !alias.scope !7490, !noalias !7449, !noundef !11
  %_17.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 288, !dbg !7493
  %_4.i641 = getelementptr inbounds nuw i8, ptr %channels.1, i64 480, !dbg !7495
  %_0.i912 = load float, ptr %_4.i641, align 4, !dbg !7497, !alias.scope !7499, !noalias !7502, !noundef !11
  %_7.i643 = getelementptr inbounds nuw i8, ptr %channels.1, i64 512, !dbg !7505
  %_0.i911 = load float, ptr %_7.i643, align 4, !dbg !7506, !alias.scope !7508, !noalias !7502, !noundef !11
  %_0.i910 = load float, ptr %_17.i, align 4, !dbg !7511, !alias.scope !7513, !noalias !7502, !noundef !11
  %_14.i646 = getelementptr inbounds nuw i8, ptr %channels.1, i64 320, !dbg !7516
  %_0.i909 = load float, ptr %_14.i646, align 4, !dbg !7517, !alias.scope !7519, !noalias !7502, !noundef !11
  %_17.i648 = getelementptr inbounds nuw i8, ptr %channels.1, i64 352, !dbg !7522
  %_0.i908 = load float, ptr %_17.i648, align 4, !dbg !7523, !alias.scope !7525, !noalias !7502, !noundef !11
  %_20.i650 = getelementptr inbounds nuw i8, ptr %channels.1, i64 384, !dbg !7528
  %_0.i907 = load float, ptr %_20.i650, align 4, !dbg !7529, !alias.scope !7531, !noalias !7502, !noundef !11
  %_23.i652 = getelementptr inbounds nuw i8, ptr %channels.1, i64 416, !dbg !7534
  %_0.i9061650 = load i32, ptr %_23.i652, align 4, !dbg !7535, !alias.scope !7537, !noalias !7502, !noundef !11
  %_26.i654 = getelementptr inbounds nuw i8, ptr %channels.1, i64 448, !dbg !7540
  %_0.i9051651 = load i32, ptr %_26.i654, align 4, !dbg !7541, !alias.scope !7543, !noalias !7502, !noundef !11
  %ring_length.i = zext i32 %_12.i to i64, !dbg !7431
  %53 = icmp eq i32 %link, 1, !dbg !7546
  %.not1653 = icmp eq i32 %link, 3, !dbg !7548
  %_3.i707 = fcmp une float %_0.i903, 1.000000e+00, !dbg !7549
  %_3.i705 = fcmp oeq float %_0.i903, 0.000000e+00, !dbg !7551
  %_3.i704 = fcmp oeq float %_0.i904, 0.000000e+00, !dbg !7553
  %_3.i713 = fcmp une float %_0.i911, 1.000000e+00, !dbg !7555
  %_3.i711 = fcmp oeq float %_0.i911, 0.000000e+00, !dbg !7557
  %_3.i709 = fcmp oeq float %_0.i912, 0.000000e+00, !dbg !7559
  %_6.i.i = load i64, ptr %detector, align 8, !range !220
  %54 = getelementptr inbounds nuw i8, ptr %detector, i64 16
  %sidechain_left.1.i.i = load i64, ptr %54, align 8
  %55 = getelementptr inbounds nuw i8, ptr %detector, i64 24
  %sidechain_right.0.i.i = load ptr, ptr %55, align 8, !nonnull !11, !align !3781
  %56 = getelementptr inbounds nuw i8, ptr %detector, i64 32
  %sidechain_right.1.i.i = load i64, ptr %56, align 8
  %57 = getelementptr inbounds nuw i8, ptr %detector, i64 8
  %sidechain_left.0.i.i = load ptr, ptr %57, align 8, !nonnull !11, !align !3781
  %58 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1508
  %59 = getelementptr inbounds nuw i8, ptr %channels.0, i64 8
  %60 = getelementptr inbounds nuw i8, ptr %channels.0, i64 24
  %61 = getelementptr inbounds nuw i8, ptr %channels.0, i64 16
  %62 = getelementptr inbounds nuw i8, ptr %channels.1, i64 8
  %63 = getelementptr inbounds nuw i8, ptr %channels.1, i64 24
  %64 = getelementptr inbounds nuw i8, ptr %channels.1, i64 16
  %65 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1512
  %_73.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 1504
  %66 = fneg float %_0.i900
  %_77.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 1504
  %67 = fneg float %_0.i908
  %68 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1508
  br label %bb41.i, !dbg !7561

bb41.i:                                           ; preds = %bb41.i.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit950
  %start1.sroa.0.0.i2001 = phi i64 [ %spec.store.select, %bb41.i.lr.ph ], [ %69, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit950 ]
  %69 = add nuw nsw i64 %start1.sroa.0.0.i2001, 1, !dbg !7573
  %_113.i = icmp samesign ugt i64 %start1.sroa.0.0.i2001, %left.1, !dbg !7581
  br i1 %_113.i, label %bb43.i, label %bb44.i, !dbg !7581, !prof !161

bb44.i:                                           ; preds = %bb41.i
  %_120.i = getelementptr inbounds nuw float, ptr %left.0, i64 %start1.sroa.0.0.i2001, !dbg !7588
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7593), !dbg !7596
  %_3.not.i1003 = icmp eq i64 %left.1, %start1.sroa.0.0.i2001, !dbg !7597
  br i1 %_3.not.i1003, label %panic.i1005, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1006, !dbg !7597

panic.i1005:                                      ; preds = %bb44.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !7597, !noalias !7599
  unreachable, !dbg !7597

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1006: ; preds = %bb44.i
  %_0.i1004 = load float, ptr %_120.i, align 4, !dbg !7597, !alias.scope !7593, !noalias !7600, !noundef !11
  %_121.i = icmp samesign ugt i64 %start1.sroa.0.0.i2001, %right.1, !dbg !7601
  br i1 %_121.i, label %bb45.i, label %bb46.i, !dbg !7601, !prof !161

bb43.i:                                           ; preds = %bb41.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %start1.sroa.0.0.i2001, i64 noundef range(i64 0, 2305843009213693952) %left.1, i64 noundef range(i64 0, 2305843009213693952) %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c46c6dde8c9e0eb71c8d3a58ebc90caa) #23, !dbg !7606, !noalias !7600
  unreachable, !dbg !7606

bb46.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1006
  %_128.i = getelementptr inbounds nuw float, ptr %right.0, i64 %start1.sroa.0.0.i2001, !dbg !7607
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7612), !dbg !7615
  %_3.not.i999 = icmp eq i64 %right.1, %start1.sroa.0.0.i2001, !dbg !7616
  br i1 %_3.not.i999, label %panic.i1001, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1002, !dbg !7616

panic.i1001:                                      ; preds = %bb46.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !7616, !noalias !7618
  unreachable, !dbg !7616

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1002: ; preds = %bb46.i
  %_0.i1000 = load float, ptr %_128.i, align 4, !dbg !7616, !alias.scope !7612, !noalias !7600, !noundef !11
  switch i64 %_6.i.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1002.unreachabledefault [
    i64 0, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i
    i64 1, label %bb3.i.i
    i64 2, label %bb2.i.i
  ], !dbg !7619

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1002.unreachabledefault: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1002
  unreachable

default.unreachable:                              ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit365.i
  unreachable

bb3.i.i:                                          ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1002
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i, !dbg !7622

bb2.i.i:                                          ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1002
  %_25.i.i = icmp ugt i64 %start1.sroa.0.0.i2001, %sidechain_left.1.i.i, !dbg !7623
  br i1 %_25.i.i, label %bb17.i.i, label %bb18.i.i, !dbg !7623, !prof !161

bb18.i.i:                                         ; preds = %bb2.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7626), !dbg !7629
  %_3.not.i995 = icmp eq i64 %sidechain_left.1.i.i, %start1.sroa.0.0.i2001, !dbg !7630
  br i1 %_3.not.i995, label %panic.i997, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit998, !dbg !7630

panic.i997:                                       ; preds = %bb18.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !7630, !noalias !7632
  unreachable, !dbg !7630

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit998: ; preds = %bb18.i.i
  %_32.i.i = getelementptr inbounds nuw float, ptr %sidechain_left.0.i.i, i64 %start1.sroa.0.0.i2001, !dbg !7636
  %_0.i996 = load float, ptr %_32.i.i, align 4, !dbg !7630, !alias.scope !7626, !noalias !7638, !noundef !11
  %_33.i.i = icmp ugt i64 %start1.sroa.0.0.i2001, %sidechain_right.1.i.i, !dbg !7639
  br i1 %_33.i.i, label %bb19.i.i, label %bb20.i.i, !dbg !7639, !prof !161

bb17.i.i:                                         ; preds = %bb2.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 2305843009213693952) %start1.sroa.0.0.i2001, i64 noundef %sidechain_left.1.i.i, i64 noundef %sidechain_left.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7fe5a91ec575deda110c046c84630100) #23, !dbg !7642, !noalias !7638
  unreachable, !dbg !7642

bb20.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit998
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7643), !dbg !7646
  %_3.not.i991 = icmp eq i64 %sidechain_right.1.i.i, %start1.sroa.0.0.i2001, !dbg !7647
  br i1 %_3.not.i991, label %panic.i993, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit994, !dbg !7647

panic.i993:                                       ; preds = %bb20.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !7647, !noalias !7649
  unreachable, !dbg !7647

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit994: ; preds = %bb20.i.i
  %_40.i.i = getelementptr inbounds nuw float, ptr %sidechain_right.0.i.i, i64 %start1.sroa.0.0.i2001, !dbg !7650
  %_0.i992 = load float, ptr %_40.i.i, align 4, !dbg !7647, !alias.scope !7643, !noalias !7638, !noundef !11
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i, !dbg !7652

bb19.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit998
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 2305843009213693952) %start1.sroa.0.0.i2001, i64 noundef %sidechain_right.1.i.i, i64 noundef %sidechain_right.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f2cba1f89421a726339097d2eba48536) #23, !dbg !7653, !noalias !7638
  unreachable, !dbg !7653

_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1002, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit994, %bb3.i.i
  %main_right.sroa.0.0.i.i = phi float [ %_0.i1000, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1002 ], [ 0.000000e+00, %bb3.i.i ], [ %_0.i992, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit994 ]
  %main_left.sroa.0.0.i.i = phi float [ %_0.i1004, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1002 ], [ 0.000000e+00, %bb3.i.i ], [ %_0.i996, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit994 ]
  %70 = tail call noundef float @llvm.fabs.f32(float %main_left.sroa.0.0.i.i), !dbg !7654
  %71 = tail call noundef float @llvm.fabs.f32(float %main_right.sroa.0.0.i.i), !dbg !7656
  %_3.i.i1448 = fcmp ule float %70, %71, !dbg !7658
  %_6.i.i1450 = bitcast float %70 to i32, !dbg !7661
  %_8.i.i1452 = bitcast float %71 to i32, !dbg !7664
  %_4.i.i1455 = select i1 %_3.i.i1448, i32 %_8.i.i1452, i32 %_6.i.i1450, !dbg !7666
  %_0.i876 = fmul float %70, 5.000000e-01, !dbg !7667
  %_0.i875 = fmul float %71, 5.000000e-01, !dbg !7669
  %_0.i804 = fadd float %_0.i875, %_0.i876, !dbg !7671
  %_6.i1256 = bitcast float %_0.i804 to i32, !dbg !7673
  %_4.i1261 = select i1 %.not1653, i32 %_6.i1256, i32 %_4.i.i1455, !dbg !7676
  %_4.i1254 = select i1 %53, i32 %_6.i.i1450, i32 %_4.i1261, !dbg !7677
  %_4.i1247 = select i1 %53, i32 %_8.i.i1452, i32 %_4.i1261, !dbg !7679
  %_39.i = load i32, ptr %58, align 4, !dbg !7681, !alias.scope !7425, !noalias !7435, !noundef !11
  %write.i = zext i32 %_39.i to i64, !dbg !7681
  %72 = add nuw nsw i64 %write.i, 1, !dbg !7683
  %_41.i = icmp eq i64 %72, %ring_length.i, !dbg !7685
  %spec.store.select.i = select i1 %_41.i, i64 0, i64 %72, !dbg !7685
  %_189.1.i = load i64, ptr %59, align 8, !dbg !7687, !alias.scope !7425, !noalias !7435, !noundef !11
  %_129.i = icmp ult i64 %_189.1.i, %write.i, !dbg !7689
  br i1 %_129.i, label %bb47.i, label %bb48.i, !dbg !7689, !prof !161

bb45.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit1006
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %start1.sroa.0.0.i2001, i64 noundef range(i64 0, 2305843009213693952) %right.1, i64 noundef range(i64 0, 2305843009213693952) %right.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9b27971f08fa00763b6dc53a9f4dbd4c) #23, !dbg !7694, !noalias !7600
  unreachable, !dbg !7694

bb48.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i
  %_189.0.i = load ptr, ptr %channels.0, align 8, !dbg !7687, !alias.scope !7425, !noalias !7435, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7695), !dbg !7698
  %_4.not.i1066 = icmp eq i64 %_189.1.i, %write.i, !dbg !7699
  br i1 %_4.not.i1066, label %panic.i1067, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1068, !dbg !7699

panic.i1067:                                      ; preds = %bb48.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #23, !dbg !7699, !noalias !7701
  unreachable, !dbg !7699

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1068: ; preds = %bb48.i
  %_136.i = getelementptr inbounds nuw float, ptr %_189.0.i, i64 %write.i, !dbg !7702
  store float %_0.i1004, ptr %_136.i, align 4, !dbg !7699, !alias.scope !7695, !noalias !7600
  %_190.1.i = load i64, ptr %60, align 8, !dbg !7707, !alias.scope !7425, !noalias !7435, !noundef !11
  %_137.i = icmp ult i64 %_190.1.i, %write.i, !dbg !7708
  br i1 %_137.i, label %bb49.i, label %bb50.i, !dbg !7708, !prof !161

bb47.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i, i64 noundef %_189.1.i, i64 noundef %_189.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bb9110d0bc8cedfe643d9bd892c8c620) #23, !dbg !7712, !noalias !7600
  unreachable, !dbg !7712

bb50.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1068
  %_190.0.i = load ptr, ptr %61, align 8, !dbg !7707, !alias.scope !7425, !noalias !7435, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7713), !dbg !7716
  %_4.not.i1063 = icmp eq i64 %_190.1.i, %write.i, !dbg !7717
  br i1 %_4.not.i1063, label %panic.i1064, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1065, !dbg !7717

panic.i1064:                                      ; preds = %bb50.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #23, !dbg !7717, !noalias !7719
  unreachable, !dbg !7717

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1065: ; preds = %bb50.i
  %_144.i = getelementptr inbounds nuw float, ptr %_190.0.i, i64 %write.i, !dbg !7720
  store i32 %_4.i1254, ptr %_144.i, align 4, !dbg !7717, !alias.scope !7713, !noalias !7600
  %_191.1.i = load i64, ptr %62, align 8, !dbg !7725, !alias.scope !7429, !noalias !7726, !noundef !11
  %_145.i = icmp ult i64 %_191.1.i, %write.i, !dbg !7727
  br i1 %_145.i, label %bb51.i, label %bb52.i, !dbg !7727, !prof !161

bb49.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1068
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i, i64 noundef %_190.1.i, i64 noundef %_190.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_da5a912ec6b29b38396e4eb206c99989) #23, !dbg !7731, !noalias !7600
  unreachable, !dbg !7731

bb52.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1065
  %_191.0.i = load ptr, ptr %channels.1, align 8, !dbg !7725, !alias.scope !7429, !noalias !7726, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7732), !dbg !7735
  %_4.not.i1060 = icmp eq i64 %_191.1.i, %write.i, !dbg !7736
  br i1 %_4.not.i1060, label %panic.i1061, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1062, !dbg !7736

panic.i1061:                                      ; preds = %bb52.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #23, !dbg !7736, !noalias !7738
  unreachable, !dbg !7736

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1062: ; preds = %bb52.i
  %_152.i = getelementptr inbounds nuw float, ptr %_191.0.i, i64 %write.i, !dbg !7739
  store float %_0.i1000, ptr %_152.i, align 4, !dbg !7736, !alias.scope !7732, !noalias !7600
  %_192.1.i = load i64, ptr %63, align 8, !dbg !7744, !alias.scope !7429, !noalias !7726, !noundef !11
  %_153.i = icmp ult i64 %_192.1.i, %write.i, !dbg !7745
  br i1 %_153.i, label %bb53.i, label %bb54.i, !dbg !7745, !prof !161

bb51.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1065
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i, i64 noundef %_191.1.i, i64 noundef %_191.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b46755d5101951cde0ca1c710062f433) #23, !dbg !7749, !noalias !7600
  unreachable, !dbg !7749

bb54.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1062
  %_192.0.i = load ptr, ptr %64, align 8, !dbg !7744, !alias.scope !7429, !noalias !7726, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7750), !dbg !7753
  %_4.not.i1057 = icmp eq i64 %_192.1.i, %write.i, !dbg !7754
  br i1 %_4.not.i1057, label %panic.i1058, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1059, !dbg !7754

panic.i1058:                                      ; preds = %bb54.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #23, !dbg !7754, !noalias !7756
  unreachable, !dbg !7754

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1059: ; preds = %bb54.i
  %_160.i = getelementptr inbounds nuw float, ptr %_192.0.i, i64 %write.i, !dbg !7757
  store i32 %_4.i1247, ptr %_160.i, align 4, !dbg !7754, !alias.scope !7750, !noalias !7600
  %_193.1.i = load i64, ptr %59, align 8, !dbg !7762, !alias.scope !7425, !noalias !7435, !noundef !11
  %_161.i = icmp ugt i64 %spec.store.select.i, %_193.1.i, !dbg !7763
  br i1 %_161.i, label %bb55.i, label %bb56.i, !dbg !7763, !prof !161

bb53.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1062
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i, i64 noundef %_192.1.i, i64 noundef %_192.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2fb73b04dc26e2fd60c7be20365496fe) #23, !dbg !7767, !noalias !7600
  unreachable, !dbg !7767

bb56.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1059
  %_193.0.i = load ptr, ptr %channels.0, align 8, !dbg !7762, !alias.scope !7425, !noalias !7435, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7768), !dbg !7771
  %_3.not.i987 = icmp eq i64 %_193.1.i, %spec.store.select.i, !dbg !7772
  br i1 %_3.not.i987, label %panic.i989, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit990, !dbg !7772

panic.i989:                                       ; preds = %bb56.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !7772, !noalias !7774
  unreachable, !dbg !7772

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit990: ; preds = %bb56.i
  %_168.i = getelementptr inbounds nuw float, ptr %_193.0.i, i64 %spec.store.select.i, !dbg !7775
  %_0.i988 = load float, ptr %_168.i, align 4, !dbg !7772, !alias.scope !7768, !noalias !7600, !noundef !11
  %_194.1.i = load i64, ptr %62, align 8, !dbg !7780, !alias.scope !7429, !noalias !7726, !noundef !11
  %_169.i = icmp ugt i64 %spec.store.select.i, %_194.1.i, !dbg !7782
  br i1 %_169.i, label %bb57.i, label %bb58.i, !dbg !7782, !prof !161

bb55.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit1059
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %spec.store.select.i, i64 noundef %_193.1.i, i64 noundef %_193.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_322bbb2a85bd2347c4d3c1d32e8c0bc7) #23, !dbg !7786, !noalias !7600
  unreachable, !dbg !7786

bb58.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit990
  %_194.0.i = load ptr, ptr %channels.1, align 8, !dbg !7780, !alias.scope !7429, !noalias !7726, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7787), !dbg !7790
  %_3.not.i983 = icmp eq i64 %_194.1.i, %spec.store.select.i, !dbg !7791
  br i1 %_3.not.i983, label %panic.i985, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit986, !dbg !7791

panic.i985:                                       ; preds = %bb58.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !7791, !noalias !7793
  unreachable, !dbg !7791

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit986: ; preds = %bb58.i
  %_176.i = getelementptr inbounds nuw float, ptr %_194.0.i, i64 %spec.store.select.i, !dbg !7794
  %_0.i984 = load float, ptr %_176.i, align 4, !dbg !7791, !alias.scope !7787, !noalias !7600, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7799), !dbg !7802
  %_6.i202 = load i32, ptr %52, align 8, !dbg !7804, !alias.scope !7799, !noalias !7600, !noundef !11
  %_10.not.i204 = icmp ult i32 %_39.i, %channels.0.val, !dbg !7806
  %narrow1657 = select i1 %_10.not.i204, i32 %_6.i202, i32 0, !dbg !7806
  %_11.i205 = zext i32 %narrow1657 to i64, !dbg !7806
  %write.pn.i206 = sub nsw i64 %write.i, %_0.i4.i, !dbg !7806
  %row.sroa.0.0.i207 = add nsw i64 %write.pn.i206, %_11.i205, !dbg !7807
  %_55.1.i208 = load i64, ptr %60, align 8, !dbg !7808, !alias.scope !7799, !noalias !7600, !noundef !11
  %_40.i209 = icmp ugt i64 %row.sroa.0.0.i207, %_55.1.i208, !dbg !7809
  br i1 %_40.i209, label %bb19.i213, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit214, !dbg !7809, !prof !161

bb19.i213:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit986
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i207, i64 noundef %_55.1.i208, i64 noundef %_55.1.i208, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_942b76445104c042a726096e85fe3fcb) #23, !dbg !7812, !noalias !7813
  unreachable, !dbg !7812

_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit214: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit986
  %_55.0.i210 = load ptr, ptr %61, align 8, !dbg !7808, !alias.scope !7799, !noalias !7600, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7814), !dbg !7817
  %_3.not.i = icmp eq i64 %_55.1.i208, %row.sroa.0.0.i207, !dbg !7818
  br i1 %_3.not.i, label %panic.i946, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit, !dbg !7818

panic.i946:                                       ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit214
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !7818, !noalias !7820
  unreachable, !dbg !7818

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit: ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit214
  %_47.i212 = getelementptr inbounds nuw float, ptr %_55.0.i210, i64 %row.sroa.0.0.i207, !dbg !7821
  %_0.i945 = load float, ptr %_47.i212, align 4, !dbg !7818, !alias.scope !7814, !noalias !7813, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7823), !dbg !7826
  %_6.i189 = load i32, ptr %65, align 8, !dbg !7828, !alias.scope !7823, !noalias !7600, !noundef !11
  %_10.not.i191 = icmp ult i32 %_39.i, %channels.1.val, !dbg !7830
  %narrow1658 = select i1 %_10.not.i191, i32 %_6.i189, i32 0, !dbg !7830
  %_11.i192 = zext i32 %narrow1658 to i64, !dbg !7830
  %write.pn.i193 = sub nsw i64 %write.i, %_0.i.i1577, !dbg !7830
  %row.sroa.0.0.i194 = add nsw i64 %write.pn.i193, %_11.i192, !dbg !7831
  %_55.1.i195 = load i64, ptr %63, align 8, !dbg !7832, !alias.scope !7823, !noalias !7600, !noundef !11
  %_40.i196 = icmp ugt i64 %row.sroa.0.0.i194, %_55.1.i195, !dbg !7833
  br i1 %_40.i196, label %bb19.i200, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit201, !dbg !7833, !prof !161

bb19.i200:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i194, i64 noundef %_55.1.i195, i64 noundef %_55.1.i195, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_942b76445104c042a726096e85fe3fcb) #23, !dbg !7836, !noalias !7837
  unreachable, !dbg !7836

_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit201: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit
  %_55.0.i197 = load ptr, ptr %64, align 8, !dbg !7832, !alias.scope !7823, !noalias !7600, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7838), !dbg !7841
  %_3.not.i947 = icmp eq i64 %_55.1.i195, %row.sroa.0.0.i194, !dbg !7842
  br i1 %_3.not.i947, label %panic.i949, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit950, !dbg !7842

panic.i949:                                       ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit201
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !7842, !noalias !7844
  unreachable, !dbg !7842

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit950: ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel15gather_detectorfEB4_.exit201
  %_47.i199 = getelementptr inbounds nuw float, ptr %_55.0.i197, i64 %row.sroa.0.0.i194, !dbg !7845
  %_0.i948 = load float, ptr %_47.i199, align 4, !dbg !7842, !alias.scope !7838, !noalias !7837, !noundef !11
  %_3.i.i1349 = fcmp ule float %_0.i945, 0x3E45798EE0000000, !dbg !7847
  %_6.i.i1351 = bitcast float %_0.i945 to i32, !dbg !7854
  %_4.i.i1356 = select i1 %_3.i.i1349, i32 841731191, i32 %_6.i.i1351, !dbg !7857
  %_0.i.i1357 = bitcast i32 %_4.i.i1356 to float, !dbg !7858
  %_3.i.i1292 = fcmp ule float %_0.i.i1357, 0x3810000000000000, !dbg !7860
  %_4.i.i1298 = select i1 %_3.i.i1292, i32 8388608, i32 %_4.i.i1356, !dbg !7865
  %_5.i1028 = and i32 %_4.i.i1298, 8388607, !dbg !7867
  %_4.i1029 = or disjoint i32 %_5.i1028, 1065353216, !dbg !7867
  %significand.i1030 = bitcast i32 %_4.i1029 to float, !dbg !7869
  %_0.i880 = fadd float %significand.i1030, -1.000000e+00, !dbg !7871
  %_0.i828 = fmul float %_0.i880, 0xBF9B17A960000000, !dbg !7873
  %_0.i786 = fadd float %_0.i828, 0x3FBF9A8440000000, !dbg !7875
  %_0.i828.1 = fmul float %_0.i880, %_0.i786, !dbg !7873
  %_0.i786.1 = fadd float %_0.i828.1, 0xBFD1E3F400000000, !dbg !7875
  %_0.i828.2 = fmul float %_0.i880, %_0.i786.1, !dbg !7873
  %_0.i786.2 = fadd float %_0.i828.2, 0x3FDD544F20000000, !dbg !7875
  %_0.i828.3 = fmul float %_0.i880, %_0.i786.2, !dbg !7873
  %_0.i786.3 = fadd float %_0.i828.3, 0xBFE6FC2A60000000, !dbg !7875
  %_0.i828.4 = fmul float %_0.i880, %_0.i786.3, !dbg !7873
  %_0.i786.4 = fadd float %_0.i828.4, 0x3FF714B2A0000000, !dbg !7875
  %_9.i1031 = lshr i32 %_4.i.i1298, 23, !dbg !7877
  %_8.i1032 = or disjoint i32 %_9.i1031, 1258291200, !dbg !7877
  %_7.i1033 = bitcast i32 %_8.i1032 to float, !dbg !7878
  %exponent.i1034 = fadd float %_7.i1033, 0xC160000FE0000000, !dbg !7880
  %_0.i827 = fmul float %_0.i880, %_0.i786.4, !dbg !7881
  %_0.i785 = fadd float %exponent.i1034, %_0.i827, !dbg !7883
  %_0.i869 = fmul float %_0.i785, 0x4018151820000000, !dbg !7885
  %_3.i.i1340.inv = fcmp ogt float %_0.i869, -1.600000e+02, !dbg !7887
  %_0.i.i1348 = select i1 %_3.i.i1340.inv, float %_0.i869, float -1.600000e+02, !dbg !7887
  %_3.i.i1498.inv = fcmp olt float %_0.i.i1348, 2.400000e+01, !dbg !7890
  %_0.i.i1506 = select i1 %_3.i.i1498.inv, float %_0.i.i1348, float 2.400000e+01, !dbg !7890
  %_0.i888 = fsub float %_0.i.i1506, %_0.i902, !dbg !7893
  %_3.i753 = fcmp ule float %_0.i888, %_0.i900, !dbg !7896
  %_0.i798 = fadd float %_0.i900, %_0.i888, !dbg !7898
  %_0.i855 = fmul float %_0.i798, %_0.i798, !dbg !7900
  %_0.i854 = fmul float %_0.i899, %_0.i855, !dbg !7902
  %_4.i1135.v.v = select i1 %_3.i753, float %_0.i854, float %_0.i888, !dbg !7904
  %_4.i1135.v = fmul float %_0.i901, %_4.i1135.v.v, !dbg !7904
  %_4.i1135 = bitcast float %_4.i1135.v to i32, !dbg !7904
  %73 = fcmp ugt float %_0.i888, %66, !dbg !7906
  %_7.i1127 = select i1 %73, i32 %_4.i1135, i32 0, !dbg !7908
  %_0.i1129 = bitcast i32 %_7.i1127 to float, !dbg !7909
  %_3.i.i1332 = fcmp ule float %_0.i1129, -1.000000e+02, !dbg !7911
  %74 = bitcast i32 %_7.i1127 to float, !dbg !7914
  %_0.i.i1339 = select i1 %_3.i.i1332, float -1.000000e+02, float %74, !dbg !7917
  %_3.i.i1489 = fcmp olt float %_0.i.i1339, 0.000000e+00, !dbg !7918
  %_0.i.i1497 = select i1 %_3.i.i1489, float %_0.i.i1339, float 0.000000e+00, !dbg !7921
  %_6.i321 = load float, ptr %_73.i, align 4, !dbg !7923, !alias.scope !7926, !noalias !7929, !noundef !11
  %_3.i765 = fcmp uge float %_0.i.i1497, %_6.i321, !dbg !7931
  %_4.i1142 = select i1 %_3.i765, i32 %_0.i8971649, i32 %_0.i8981648, !dbg !7933
  %_0.i1143 = bitcast i32 %_4.i1142 to float, !dbg !7935
  %_0.i889 = fsub float %_0.i.i1497, %_6.i321, !dbg !7937
  %_4.i805 = fmul float %_0.i889, %_0.i1143, !dbg !7940
  %_0.i806 = fadd float %_6.i321, %_4.i805, !dbg !7940
  %75 = tail call noundef float @llvm.fabs.f32(float %_0.i806), !dbg !7942
  %76 = fcmp uge float %75, 0x3BC79CA100000000, !dbg !7945
  %_0.i1072 = select i1 %76, float %_0.i806, float 0.000000e+00, !dbg !7947
  store float %_0.i1072, ptr %_73.i, align 4, !dbg !7948, !alias.scope !7926, !noalias !7929
  %_0.i799 = fadd float %_0.i904, %_0.i1072, !dbg !7949
  %_0.i859 = fmul float %_0.i799, 0x3FC542A5A0000000, !dbg !7953
  %_3.i.i1324.inv = fcmp ogt float %_0.i859, -1.260000e+02, !dbg !7956
  %_0.i.i1331 = select i1 %_3.i.i1324.inv, float %_0.i859, float -1.260000e+02, !dbg !7956
  %_3.i.i1481.inv = fcmp olt float %_0.i.i1331, 1.270000e+02, !dbg !7960
  %_0.i.i1488 = select i1 %_3.i.i1481.inv, float %_0.i.i1331, float 1.270000e+02, !dbg !7960
  %77 = tail call noundef float @llvm.floor.f32(float %_0.i.i1488), !dbg !7963
  %_0.i884 = fsub float %_0.i.i1488, %77, !dbg !7967
  %_3.i.i1376 = fcmp ule float %_0.i948, 0x3E45798EE0000000, !dbg !7969
  %_6.i.i1378 = bitcast float %_0.i948 to i32, !dbg !7975
  %_4.i.i1383 = select i1 %_3.i.i1376, i32 841731191, i32 %_6.i.i1378, !dbg !7978
  %_0.i.i1384 = bitcast i32 %_4.i.i1383 to float, !dbg !7979
  %_3.i.i1284 = fcmp ule float %_0.i.i1384, 0x3810000000000000, !dbg !7981
  %_4.i.i1290 = select i1 %_3.i.i1284, i32 8388608, i32 %_4.i.i1383, !dbg !7986
  %_5.i1020 = and i32 %_4.i.i1290, 8388607, !dbg !7988
  %_4.i1021 = or disjoint i32 %_5.i1020, 1065353216, !dbg !7988
  %significand.i1022 = bitcast i32 %_4.i1021 to float, !dbg !7990
  %_0.i879 = fadd float %significand.i1022, -1.000000e+00, !dbg !7992
  %_0.i826 = fmul float %_0.i879, 0xBF9B17A960000000, !dbg !7994
  %_0.i784 = fadd float %_0.i826, 0x3FBF9A8440000000, !dbg !7996
  %_0.i826.1 = fmul float %_0.i879, %_0.i784, !dbg !7994
  %_0.i784.1 = fadd float %_0.i826.1, 0xBFD1E3F400000000, !dbg !7996
  %_0.i826.2 = fmul float %_0.i879, %_0.i784.1, !dbg !7994
  %_0.i784.2 = fadd float %_0.i826.2, 0x3FDD544F20000000, !dbg !7996
  %_0.i826.3 = fmul float %_0.i879, %_0.i784.2, !dbg !7994
  %_0.i784.3 = fadd float %_0.i826.3, 0xBFE6FC2A60000000, !dbg !7996
  %_0.i826.4 = fmul float %_0.i879, %_0.i784.3, !dbg !7994
  %_0.i784.4 = fadd float %_0.i826.4, 0x3FF714B2A0000000, !dbg !7996
  %_9.i1023 = lshr i32 %_4.i.i1290, 23, !dbg !7998
  %_8.i1024 = or disjoint i32 %_9.i1023, 1258291200, !dbg !7998
  %_7.i1025 = bitcast i32 %_8.i1024 to float, !dbg !7999
  %exponent.i1026 = fadd float %_7.i1025, 0xC160000FE0000000, !dbg !8001
  %_0.i825 = fmul float %_0.i879, %_0.i784.4, !dbg !8002
  %_0.i783 = fadd float %exponent.i1026, %_0.i825, !dbg !8004
  %_0.i870 = fmul float %_0.i783, 0x4018151820000000, !dbg !8006
  %_3.i.i1367.inv = fcmp ogt float %_0.i870, -1.600000e+02, !dbg !8008
  %_0.i.i1375 = select i1 %_3.i.i1367.inv, float %_0.i870, float -1.600000e+02, !dbg !8008
  %_3.i.i1516.inv = fcmp olt float %_0.i.i1375, 2.400000e+01, !dbg !8011
  %_0.i.i1524 = select i1 %_3.i.i1516.inv, float %_0.i.i1375, float 2.400000e+01, !dbg !8011
  %_0.i887 = fsub float %_0.i.i1524, %_0.i910, !dbg !8014
  %_3.i751 = fcmp ule float %_0.i887, %_0.i908, !dbg !8017
  %_0.i797 = fadd float %_0.i908, %_0.i887, !dbg !8019
  %_0.i851 = fmul float %_0.i797, %_0.i797, !dbg !8021
  %_0.i850 = fmul float %_0.i907, %_0.i851, !dbg !8023
  %_4.i1122.v.v = select i1 %_3.i751, float %_0.i850, float %_0.i887, !dbg !8025
  %_4.i1122.v = fmul float %_0.i909, %_4.i1122.v.v, !dbg !8025
  %_4.i1122 = bitcast float %_4.i1122.v to i32, !dbg !8025
  %78 = fcmp ugt float %_0.i887, %67, !dbg !8027
  %_7.i1114 = select i1 %78, i32 %_4.i1122, i32 0, !dbg !8029
  %_0.i1116 = bitcast i32 %_7.i1114 to float, !dbg !8030
  %_3.i.i1358 = fcmp ule float %_0.i1116, -1.000000e+02, !dbg !8032
  %79 = bitcast i32 %_7.i1114 to float, !dbg !8035
  %_0.i.i1366 = select i1 %_3.i.i1358, float -1.000000e+02, float %79, !dbg !8038
  %_3.i.i1507 = fcmp olt float %_0.i.i1366, 0.000000e+00, !dbg !8039
  %_0.i.i1515 = select i1 %_3.i.i1507, float %_0.i.i1366, float 0.000000e+00, !dbg !8042
  %_6.i309 = load float, ptr %_77.i, align 4, !dbg !8044, !alias.scope !8047, !noalias !8050, !noundef !11
  %_3.i769 = fcmp uge float %_0.i.i1515, %_6.i309, !dbg !8052
  %_4.i1149 = select i1 %_3.i769, i32 %_0.i9051651, i32 %_0.i9061650, !dbg !8054
  %_0.i1150 = bitcast i32 %_4.i1149 to float, !dbg !8056
  %_0.i890 = fsub float %_0.i.i1515, %_6.i309, !dbg !8058
  %_4.i807 = fmul float %_0.i890, %_0.i1150, !dbg !8061
  %_0.i808 = fadd float %_6.i309, %_4.i807, !dbg !8061
  %80 = tail call noundef float @llvm.fabs.f32(float %_0.i808), !dbg !8063
  %81 = fcmp uge float %80, 0x3BC79CA100000000, !dbg !8066
  %_0.i1076 = select i1 %81, float %_0.i808, float 0.000000e+00, !dbg !8068
  store float %_0.i1076, ptr %_77.i, align 4, !dbg !8069, !alias.scope !8047, !noalias !8050
  %_0.i800 = fadd float %_0.i912, %_0.i1076, !dbg !8070
  %_0.i862 = fmul float %_0.i800, 0x3FC542A5A0000000, !dbg !8074
  %_3.i.i1316.inv = fcmp ogt float %_0.i862, -1.260000e+02, !dbg !8077
  %_0.i.i1323 = select i1 %_3.i.i1316.inv, float %_0.i862, float -1.260000e+02, !dbg !8077
  %_3.i.i1473.inv = fcmp olt float %_0.i.i1323, 1.270000e+02, !dbg !8081
  %_0.i.i1480 = select i1 %_3.i.i1473.inv, float %_0.i.i1323, float 1.270000e+02, !dbg !8081
  %82 = tail call noundef float @llvm.floor.f32(float %_0.i.i1480), !dbg !8084
  %_0.i883 = fsub float %_0.i.i1480, %82, !dbg !8088
  %_0.i837 = fmul float %_0.i883, 0x3F5E974FA0000000, !dbg !8090
  %_0.i792 = fadd float %_0.i837, 0x3F82778560000000, !dbg !8092
  %_0.i837.1 = fmul float %_0.i883, %_0.i792, !dbg !8090
  %_0.i792.1 = fadd float %_0.i837.1, 0x3FAC91CE60000000, !dbg !8092
  %_0.i837.2 = fmul float %_0.i883, %_0.i792.1, !dbg !8090
  %_0.i792.2 = fadd float %_0.i837.2, 0x3FCEBDB560000000, !dbg !8092
  %_0.i837.3 = fmul float %_0.i883, %_0.i792.2, !dbg !8090
  %_0.i792.3 = fadd float %_0.i837.3, 0x3FE62E4BA0000000, !dbg !8092
  %_0.i840 = fmul float %_0.i884, 0x3F5E974FA0000000, !dbg !8094
  %_0.i794 = fadd float %_0.i840, 0x3F82778560000000, !dbg !8096
  %_0.i840.1 = fmul float %_0.i884, %_0.i794, !dbg !8094
  %_0.i794.1 = fadd float %_0.i840.1, 0x3FAC91CE60000000, !dbg !8096
  %_0.i840.2 = fmul float %_0.i884, %_0.i794.1, !dbg !8094
  %_0.i794.2 = fadd float %_0.i840.2, 0x3FCEBDB560000000, !dbg !8096
  %_0.i840.3 = fmul float %_0.i884, %_0.i794.2, !dbg !8094
  %_0.i794.3 = fadd float %_0.i840.3, 0x3FE62E4BA0000000, !dbg !8096
  %_0.i839 = fmul float %_0.i884, %_0.i794.3, !dbg !8098
  %_0.i793 = fadd float %_0.i839, 1.000000e+00, !dbg !8100
  %biased.i700 = fadd float %77, 0x4160000FE0000000, !dbg !8102
  %_4.i701 = bitcast float %biased.i700 to i32, !dbg !8104
  %_3.i702 = shl i32 %_4.i701, 23, !dbg !8106
  %_0.i703 = bitcast i32 %_3.i702 to float, !dbg !8107
  %_0.i838 = fmul float %_0.i793, %_0.i703, !dbg !8109
  %_0.i858 = fmul float %_0.i988, %_0.i838, !dbg !8111
  %_0.i893 = fsub float %_0.i858, %_0.i988, !dbg !8113
  %_4.i813 = fmul float %_0.i903, %_0.i893, !dbg !8116
  %_0.i814 = fadd float %_0.i988, %_4.i813, !dbg !8116
  %_3.i739 = fcmp oeq float %_0.i1072, 0.000000e+00, !dbg !8118
  %_0.i12711668 = and i1 %_3.i704, %_3.i739, !dbg !8120
  %83 = or i1 %_3.i705, %_0.i12711668
  %_0.i12631670.reass.reass = or i1 %83, %bypass
  %_4.i1177.v = select i1 %_3.i707, float %_0.i814, float %_0.i858, !dbg !8122
  %_4.i1170.v = select i1 %_0.i12631670.reass.reass, float %_0.i988, float %_4.i1177.v, !dbg !8124
  %_0.i836 = fmul float %_0.i883, %_0.i792.3, !dbg !8126
  %_0.i791 = fadd float %_0.i836, 1.000000e+00, !dbg !8128
  %biased.i696 = fadd float %82, 0x4160000FE0000000, !dbg !8130
  %_4.i697 = bitcast float %biased.i696 to i32, !dbg !8132
  %_3.i698 = shl i32 %_4.i697, 23, !dbg !8134
  %_0.i699 = bitcast i32 %_3.i698 to float, !dbg !8135
  %_0.i835 = fmul float %_0.i791, %_0.i699, !dbg !8137
  %_0.i861 = fmul float %_0.i984, %_0.i835, !dbg !8139
  %_0.i894 = fsub float %_0.i861, %_0.i984, !dbg !8141
  %_4.i815 = fmul float %_0.i911, %_0.i894, !dbg !8144
  %_0.i816 = fadd float %_0.i984, %_4.i815, !dbg !8144
  %_3.i741 = fcmp oeq float %_0.i1076, 0.000000e+00, !dbg !8146
  %_0.i12721682 = and i1 %_3.i709, %_3.i741, !dbg !8148
  %84 = or i1 %_3.i711, %_0.i12721682
  %_0.i12651684.reass.reass = or i1 %84, %bypass
  %_4.i1191.v = select i1 %_3.i713, float %_0.i816, float %_0.i861, !dbg !8150
  %_4.i1184.v = select i1 %_0.i12651684.reass.reass, float %_0.i984, float %_4.i1191.v, !dbg !8152
  store float %_4.i1170.v, ptr %_120.i, align 4, !dbg !8154, !alias.scope !8157, !noalias !7600
  store float %_4.i1184.v, ptr %_128.i, align 4, !dbg !8160, !alias.scope !8162, !noalias !7600
  %85 = trunc i64 %spec.store.select.i to i32, !dbg !8165
  store i32 %85, ptr %58, align 4, !dbg !8165, !alias.scope !7425, !noalias !7435
  store i32 %85, ptr %68, align 4, !dbg !8166, !alias.scope !7429, !noalias !7726
  %exitcond2335.not = icmp eq i64 %69, %frames, !dbg !8167
  br i1 %exitcond2335.not, label %bb19, label %bb41.i, !dbg !7561

bb57.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit990
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %spec.store.select.i, i64 noundef %_194.1.i, i64 noundef %_194.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b5bf0b379ba879ef3901108f927dd528) #23, !dbg !8171, !noalias !7600
  unreachable, !dbg !8171

bb13:                                             ; preds = %bb11
  %_50.0 = load ptr, ptr %staged, align 8, !dbg !8172, !nonnull !11, !noundef !11
  %86 = getelementptr inbounds nuw i8, ptr %staged, i64 8, !dbg !8172
  %_50.1 = load i64, ptr %86, align 8, !dbg !8172, !noundef !11
  %87 = getelementptr inbounds nuw i8, ptr %staged, i64 16, !dbg !8174
  %_51.0 = load ptr, ptr %87, align 8, !dbg !8174, !nonnull !11, !noundef !11
  %88 = getelementptr inbounds nuw i8, ptr %staged, i64 24, !dbg !8174
  %_51.1 = load i64, ptr %88, align 8, !dbg !8174, !noundef !11
  %89 = getelementptr inbounds nuw i8, ptr %staged, i64 32, !dbg !8175
  %_52.0 = load ptr, ptr %89, align 8, !dbg !8175, !nonnull !11, !noundef !11
  %90 = getelementptr inbounds nuw i8, ptr %staged, i64 40, !dbg !8175
  %_52.1 = load i64, ptr %90, align 8, !dbg !8175, !noundef !11
  %91 = getelementptr inbounds nuw i8, ptr %staged, i64 48, !dbg !8176
  %_53.0 = load ptr, ptr %91, align 8, !dbg !8176, !nonnull !11, !noundef !11
  %92 = getelementptr inbounds nuw i8, ptr %staged, i64 56, !dbg !8176
  %_53.1 = load i64, ptr %92, align 8, !dbg !8176, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8177), !dbg !8180
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8181), !dbg !8180
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8183), !dbg !8180
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8185), !dbg !8180
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8187), !dbg !8180
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8189), !dbg !8180
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8191), !dbg !8180
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8193), !dbg !8180
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8195), !dbg !8180
  %93 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1512, !dbg !8197
  %_15.i1579 = load i32, ptr %93, align 8, !dbg !8197, !alias.scope !8185, !noalias !8201, !noundef !11
  %ring_length.i1580 = zext i32 %_15.i1579 to i64, !dbg !8197
  %94 = icmp eq i32 %link, 1, !dbg !8202
  %.not.i = icmp eq i32 %link, 3, !dbg !8206
  %_19.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 288, !dbg !8207
  %_4.i203.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 480, !dbg !8209
  %_0.i331.i = load float, ptr %_4.i203.i, align 4, !dbg !8211, !alias.scope !8213, !noalias !8216, !noundef !11
  %_7.i205.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 512, !dbg !8219
  %_0.i330.i = load float, ptr %_7.i205.i, align 4, !dbg !8220, !alias.scope !8222, !noalias !8216, !noundef !11
  %_0.i329.i = load float, ptr %_19.i, align 4, !dbg !8225, !alias.scope !8227, !noalias !8216, !noundef !11
  %_14.i208.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 320, !dbg !8230
  %_0.i328.i = load float, ptr %_14.i208.i, align 4, !dbg !8231, !alias.scope !8233, !noalias !8216, !noundef !11
  %_17.i210.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 352, !dbg !8236
  %_0.i327.i = load float, ptr %_17.i210.i, align 4, !dbg !8237, !alias.scope !8239, !noalias !8216, !noundef !11
  %_20.i212.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 384, !dbg !8242
  %_0.i326.i = load float, ptr %_20.i212.i, align 4, !dbg !8243, !alias.scope !8245, !noalias !8216, !noundef !11
  %_23.i214.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 416, !dbg !8248
  %_0.i325736.i = load i32, ptr %_23.i214.i, align 4, !dbg !8249, !alias.scope !8251, !noalias !8216, !noundef !11
  %_26.i216.i = getelementptr inbounds nuw i8, ptr %channels.0, i64 448, !dbg !8254
  %_0.i324737.i = load i32, ptr %_26.i216.i, align 4, !dbg !8255, !alias.scope !8257, !noalias !8216, !noundef !11
  %_3.i237.i = fcmp une float %_0.i330.i, 1.000000e+00, !dbg !8260
  %_3.i235.i = fcmp oeq float %_0.i330.i, 0.000000e+00, !dbg !8262
  %_3.i234.i = fcmp oeq float %_0.i331.i, 0.000000e+00, !dbg !8264
  %_21.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 288, !dbg !8266
  %_4.i189.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 480, !dbg !8268
  %_0.i339.i = load float, ptr %_4.i189.i, align 4, !dbg !8270, !alias.scope !8272, !noalias !8275, !noundef !11
  %_7.i190.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 512, !dbg !8278
  %_0.i338.i = load float, ptr %_7.i190.i, align 4, !dbg !8279, !alias.scope !8281, !noalias !8275, !noundef !11
  %_0.i337.i = load float, ptr %_21.i, align 4, !dbg !8284, !alias.scope !8286, !noalias !8275, !noundef !11
  %_14.i192.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 320, !dbg !8289
  %_0.i336.i = load float, ptr %_14.i192.i, align 4, !dbg !8290, !alias.scope !8292, !noalias !8275, !noundef !11
  %_17.i194.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 352, !dbg !8295
  %_0.i335.i = load float, ptr %_17.i194.i, align 4, !dbg !8296, !alias.scope !8298, !noalias !8275, !noundef !11
  %_20.i196.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 384, !dbg !8301
  %_0.i334.i = load float, ptr %_20.i196.i, align 4, !dbg !8302, !alias.scope !8304, !noalias !8275, !noundef !11
  %_23.i198.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 416, !dbg !8307
  %_0.i333738.i = load i32, ptr %_23.i198.i, align 4, !dbg !8308, !alias.scope !8310, !noalias !8275, !noundef !11
  %_26.i.i = getelementptr inbounds nuw i8, ptr %channels.1, i64 448, !dbg !8313
  %_0.i332739.i = load i32, ptr %_26.i.i, align 4, !dbg !8314, !alias.scope !8316, !noalias !8275, !noundef !11
  %_3.i243.i = fcmp une float %_0.i338.i, 1.000000e+00, !dbg !8319
  %_3.i241.i = fcmp oeq float %_0.i338.i, 0.000000e+00, !dbg !8321
  %_3.i239.i = fcmp oeq float %_0.i339.i, 0.000000e+00, !dbg !8323
  %95 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1508, !dbg !8325
  %_23.i = load i32, ptr %95, align 4, !dbg !8325, !alias.scope !8185, !noalias !8201, !noundef !11
  %96 = zext i32 %_23.i to i64, !dbg !8325
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8327), !dbg !8330
  %_58.not.i.i = icmp ugt i64 %_34, %_50.1
  br i1 %_58.not.i.i, label %bb16.i.i, label %bb25.i.i, !dbg !8332, !prof !239

bb16.i.i:                                         ; preds = %bb13
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_34, i64 noundef range(i64 0, 2305843009213693952) %_50.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_706ebfc02db290a50140e9081d0f8b39) #23, !dbg !8345, !noalias !8346
  unreachable, !dbg !8345

bb25.i.i:                                         ; preds = %bb13
  %_12.not.i.i = icmp ult i32 %_23.i, %channels.0.val, !dbg !8348
  %_13.i.i = select i1 %_12.not.i.i, i64 %ring_length.i1580, i64 0, !dbg !8348
  %write.pn.i.i = sub nsw i64 %96, %_0.i4.i, !dbg !8348
  %row.sroa.0.0.i.i = add nsw i64 %write.pn.i.i, %_13.i.i, !dbg !8351
  %_16.i654.i = sub nsw i64 %ring_length.i1580, %row.sroa.0.0.i.i, !dbg !8352
  %..i.i655.i = tail call noundef i64 @llvm.umin.i64(i64 %_34, i64 %_16.i654.i), !dbg !8354
  %97 = getelementptr inbounds nuw i8, ptr %channels.0, i64 16, !dbg !8356
  %_143.0.i.i = load ptr, ptr %97, align 8, !dbg !8356, !alias.scope !8358, !noalias !8359, !nonnull !11, !noundef !11
  %98 = getelementptr inbounds nuw i8, ptr %channels.0, i64 24, !dbg !8356
  %_143.1.i.i = load i64, ptr %98, align 8, !dbg !8356, !alias.scope !8358, !noalias !8359, !noundef !11
  %_23.i.i = add nsw i64 %..i.i655.i, %row.sroa.0.0.i.i, !dbg !8360
  %_88.i.i = icmp ult i64 %_23.i.i, %row.sroa.0.0.i.i, !dbg !8361
  %_82.not.i.i = icmp ugt i64 %_23.i.i, %_143.1.i.i
  %or.cond.i.i = or i1 %_88.i.i, %_82.not.i.i, !dbg !8361
  br i1 %or.cond.i.i, label %bb31.i.i, label %bb35.i.i, !dbg !8361, !prof !239

bb31.i.i:                                         ; preds = %bb25.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i.i, i64 noundef %_23.i.i, i64 noundef %_143.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_49704929b3df5dee298aef29445db88f) #23, !dbg !8367, !noalias !8346
  unreachable, !dbg !8367

bb35.i.i:                                         ; preds = %bb25.i.i
  %_91.i.i = getelementptr inbounds nuw float, ptr %_143.0.i.i, i64 %row.sroa.0.0.i.i, !dbg !8368
  %99 = shl nuw nsw i64 %..i.i655.i, 2, !dbg !8372
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 4 %_50.0, ptr nonnull readonly align 4 %_91.i.i, i64 %99, i1 false), !dbg !8372, !alias.scope !8377, !noalias !8381
  %rest.i.i = sub nsw i64 %_34, %..i.i655.i, !dbg !8383
  %_100.not.i.i = icmp ugt i64 %rest.i.i, %_143.1.i.i
  br i1 %_100.not.i.i, label %bb41.i.i, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel9fill_tapsfEB4_.exit.i, !dbg !8384, !prof !239

bb41.i.i:                                         ; preds = %bb35.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %rest.i.i, i64 noundef %_143.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c5f07f7510c4245a135cf142a70cfe98) #23, !dbg !8394, !noalias !8346
  unreachable, !dbg !8394

_RINvNtCse3bfmKSZS8Y_10compressor6kernel9fill_tapsfEB4_.exit.i: ; preds = %bb35.i.i
  %_99.i.i = getelementptr inbounds nuw float, ptr %_50.0, i64 %..i.i655.i, !dbg !8395
  %100 = shl nuw nsw i64 %rest.i.i, 2, !dbg !8404
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 4 %_99.i.i, ptr nonnull readonly align 4 %_143.0.i.i, i64 %100, i1 false), !dbg !8404, !alias.scope !8408, !noalias !8412
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8414), !dbg !8417
  %_58.not.i656.i = icmp samesign ugt i64 %_34, %_51.1
  br i1 %_58.not.i656.i, label %bb16.i681.i, label %bb25.i657.i, !dbg !8418, !prof !239

bb16.i681.i:                                      ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel9fill_tapsfEB4_.exit.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_34, i64 noundef range(i64 0, 2305843009213693952) %_51.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_706ebfc02db290a50140e9081d0f8b39) #23, !dbg !8424, !noalias !8425
  unreachable, !dbg !8424

bb25.i657.i:                                      ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel9fill_tapsfEB4_.exit.i
  %101 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1512, !dbg !8427
  %_6.i658.i = load i32, ptr %101, align 8, !dbg !8427, !alias.scope !8428, !noalias !8429, !noundef !11
  %ring_length.i659.i = zext i32 %_6.i658.i to i64, !dbg !8427
  %_12.not.i662.i = icmp ult i32 %_23.i, %channels.1.val, !dbg !8430
  %_13.i663.i = select i1 %_12.not.i662.i, i64 %ring_length.i659.i, i64 0, !dbg !8430
  %write.pn.i664.i = sub nsw i64 %96, %_0.i.i1577, !dbg !8430
  %row.sroa.0.0.i665.i = add nsw i64 %_13.i663.i, %write.pn.i664.i, !dbg !8431
  %_16.i666.i = sub nsw i64 %ring_length.i659.i, %row.sroa.0.0.i665.i, !dbg !8432
  %..i.i667.i = tail call noundef i64 @llvm.umin.i64(i64 %_34, i64 %_16.i666.i), !dbg !8433
  %102 = getelementptr inbounds nuw i8, ptr %channels.1, i64 16, !dbg !8435
  %_143.0.i668.i = load ptr, ptr %102, align 8, !dbg !8435, !alias.scope !8428, !noalias !8429, !nonnull !11, !noundef !11
  %103 = getelementptr inbounds nuw i8, ptr %channels.1, i64 24, !dbg !8435
  %_143.1.i669.i = load i64, ptr %103, align 8, !dbg !8435, !alias.scope !8428, !noalias !8429, !noundef !11
  %_23.i670.i = add nsw i64 %..i.i667.i, %row.sroa.0.0.i665.i, !dbg !8436
  %_88.i671.i = icmp ult i64 %_23.i670.i, %row.sroa.0.0.i665.i, !dbg !8437
  %_82.not.i672.i = icmp ugt i64 %_23.i670.i, %_143.1.i669.i
  %or.cond.i673.i = or i1 %_88.i671.i, %_82.not.i672.i, !dbg !8437
  br i1 %or.cond.i673.i, label %bb31.i680.i, label %bb35.i674.i, !dbg !8437, !prof !239

bb31.i680.i:                                      ; preds = %bb25.i657.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i665.i, i64 noundef %_23.i670.i, i64 noundef %_143.1.i669.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_49704929b3df5dee298aef29445db88f) #23, !dbg !8441, !noalias !8425
  unreachable, !dbg !8441

bb35.i674.i:                                      ; preds = %bb25.i657.i
  %_91.i675.i = getelementptr inbounds nuw float, ptr %_143.0.i668.i, i64 %row.sroa.0.0.i665.i, !dbg !8442
  %104 = shl nuw nsw i64 %..i.i667.i, 2, !dbg !8444
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 4 %_51.0, ptr nonnull readonly align 4 %_91.i675.i, i64 %104, i1 false), !dbg !8444, !alias.scope !8448, !noalias !8452
  %rest.i676.i = sub nsw i64 %_34, %..i.i667.i, !dbg !8454
  %_100.not.i677.i = icmp ugt i64 %rest.i676.i, %_143.1.i669.i
  br i1 %_100.not.i677.i, label %bb41.i679.i, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipB3_ECse3bfmKSZS8Y_10compressor.exit.i, !dbg !8455, !prof !239

bb41.i679.i:                                      ; preds = %bb35.i674.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %rest.i676.i, i64 noundef %_143.1.i669.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c5f07f7510c4245a135cf142a70cfe98) #23, !dbg !8460, !noalias !8425
  unreachable, !dbg !8460

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipB3_ECse3bfmKSZS8Y_10compressor.exit.i: ; preds = %bb35.i674.i
  %_99.i678.i = getelementptr inbounds nuw float, ptr %_51.0, i64 %..i.i667.i, !dbg !8461
  %105 = shl nuw nsw i64 %rest.i676.i, 2, !dbg !8465
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 4 %_99.i678.i, ptr nonnull readonly align 4 %_143.0.i668.i, i64 %105, i1 false), !dbg !8465, !alias.scope !8469, !noalias !8473
  %_176.not.i = icmp samesign ugt i64 %_34, %_52.1
  br i1 %_176.not.i, label %bb58.i1606, label %bb57.i1581, !dbg !8475, !prof !239

bb58.i1606:                                       ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipB3_ECse3bfmKSZS8Y_10compressor.exit.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_34, i64 noundef range(i64 0, 2305843009213693952) %_52.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1feb33c78e17db086948326d854ee987) #23, !dbg !8485, !noalias !8486
  unreachable, !dbg !8485

bb57.i1581:                                       ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipB3_ECse3bfmKSZS8Y_10compressor.exit.i
  %_191.not.i = icmp samesign ugt i64 %_34, %_53.1, !dbg !8487
  br i1 %_191.not.i, label %bb64.i, label %bb14.lr.ph.i, !dbg !8487, !prof !161

bb14.lr.ph.i:                                     ; preds = %bb57.i1581
  %_6.i.i1582 = load i64, ptr %detector, align 8, !range !220, !alias.scope !8183, !noalias !8493
  %106 = getelementptr inbounds nuw i8, ptr %detector, i64 16
  %sidechain_left.1.i.i1583 = load i64, ptr %106, align 8, !alias.scope !8183, !noalias !8493
  %107 = getelementptr inbounds nuw i8, ptr %detector, i64 24
  %sidechain_right.0.i.i1584 = load ptr, ptr %107, align 8, !alias.scope !8183, !noalias !8493, !nonnull !11, !align !3781
  %108 = getelementptr inbounds nuw i8, ptr %detector, i64 32
  %sidechain_right.1.i.i1585 = load i64, ptr %108, align 8, !alias.scope !8183, !noalias !8493
  %109 = getelementptr inbounds nuw i8, ptr %detector, i64 8
  %sidechain_left.0.i.i1586 = load ptr, ptr %109, align 8, !alias.scope !8183, !noalias !8493, !nonnull !11, !align !3781
  %110 = getelementptr inbounds nuw i8, ptr %channels.0, i64 8
  %_335.1.i = load i64, ptr %110, align 8, !alias.scope !8185, !noalias !8201
  %_335.0.i = load ptr, ptr %channels.0, align 8, !alias.scope !8185, !noalias !8201, !nonnull !11
  %111 = getelementptr inbounds nuw i8, ptr %channels.1, i64 8
  %_337.1.i = load i64, ptr %111, align 8, !alias.scope !8187, !noalias !8494
  %_337.0.i = load ptr, ptr %channels.1, align 8, !alias.scope !8187, !noalias !8494, !nonnull !11
  %112 = fneg float %_0.i327.i
  %113 = fneg float %_0.i335.i
  br label %bb14.i1587, !dbg !8495

bb64.i:                                           ; preds = %bb57.i1581
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_34, i64 noundef range(i64 0, 2305843009213693952) %_53.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6943f86fcff04ee5505532b3b35f50d3) #23, !dbg !8503, !noalias !8486
  unreachable, !dbg !8503

bb14.i1587:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit353.i, %bb14.lr.ph.i
  %head.sroa.0.0868.i = phi i64 [ %96, %bb14.lr.ph.i ], [ %spec.store.select.i1598, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit353.i ]
  %iter.sroa.33.0867.i = phi i64 [ 0, %bb14.lr.ph.i ], [ %_9.0.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit353.i ]
  %_9.0.i.i = add nuw nsw i64 %iter.sroa.33.0867.i, 1, !dbg !8504
  %data.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_50.0, i64 %iter.sroa.33.0867.i, !dbg !8507
  %data.i5.i.i.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_51.0, i64 %iter.sroa.33.0867.i, !dbg !8518
  %_3.i.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_52.0, i64 %iter.sroa.33.0867.i, !dbg !8521
  %_3.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_53.0, i64 %iter.sroa.33.0867.i, !dbg !8526
  %_51.i = add nuw nsw i64 %iter.sroa.33.0867.i, %spec.store.select, !dbg !8529
  %_205.i = icmp samesign ugt i64 %_51.i, %left.1, !dbg !8531
  br i1 %_205.i, label %bb68.i, label %bb69.i, !dbg !8531, !prof !161

bb15.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit353.i
  %114 = trunc i64 %spec.store.select.i1598 to i32, !dbg !8538
  store i32 %114, ptr %95, align 4, !dbg !8538, !alias.scope !8185, !noalias !8201
  %115 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1508, !dbg !8539
  store i32 %114, ptr %115, align 4, !dbg !8539, !alias.scope !8187, !noalias !8494
  %116 = getelementptr inbounds nuw i8, ptr %channels.0, i64 1504, !dbg !8540
  %117 = load float, ptr %116, align 8, !dbg !8540, !alias.scope !8185, !noalias !8201, !noundef !11
  %118 = getelementptr inbounds nuw i8, ptr %channels.1, i64 1504, !dbg !8541
  %119 = load float, ptr %118, align 8, !dbg !8541, !alias.scope !8187, !noalias !8494, !noundef !11
  br label %bb36.i, !dbg !8543

bb36.i:                                           ; preds = %bb36.i, %bb15.i
  %iter2.sroa.8.0872.i = phi i64 [ %120, %bb36.i ], [ 0, %bb15.i ]
  %gain_right.sroa.0.0871.i = phi float [ %_0.i419.i, %bb36.i ], [ %119, %bb15.i ]
  %gain_left.sroa.0.0870.i = phi float [ %_0.i415.i, %bb36.i ], [ %117, %bb15.i ]
  %120 = add nuw i64 %iter2.sroa.8.0872.i, 1, !dbg !8551
  %_3.i.i701.i = getelementptr inbounds nuw float, ptr %_52.0, i64 %iter2.sroa.8.0872.i, !dbg !8553
  %_3.i1.i.i = getelementptr inbounds nuw float, ptr %_53.0, i64 %iter2.sroa.8.0872.i, !dbg !8556
  %_117.i = load float, ptr %_3.i.i701.i, align 4, !dbg !8559, !alias.scope !8193, !noalias !8561, !noundef !11
  %_3.i259.i = fcmp uge float %_117.i, %gain_left.sroa.0.0870.i, !dbg !8562
  %_4.i451.i = select i1 %_3.i259.i, i32 %_0.i324737.i, i32 %_0.i325736.i, !dbg !8565
  %_0.i452.i = bitcast i32 %_4.i451.i to float, !dbg !8567
  %_0.i320.i = fsub float %_117.i, %gain_left.sroa.0.0870.i, !dbg !8569
  %_4.i278.i = fmul float %_0.i320.i, %_0.i452.i, !dbg !8572
  %_0.i279.i = fadd float %gain_left.sroa.0.0870.i, %_4.i278.i, !dbg !8572
  %121 = tail call noundef float @llvm.fabs.f32(float %_0.i279.i), !dbg !8574
  %122 = fcmp uge float %121, 0x3BC79CA100000000, !dbg !8577
  %_0.i415.i = select i1 %122, float %_0.i279.i, float 0.000000e+00, !dbg !8579
  store float %_0.i415.i, ptr %_3.i.i701.i, align 4, !dbg !8580, !alias.scope !8193, !noalias !8561
  %_121.i1600 = load float, ptr %_3.i1.i.i, align 4, !dbg !8581, !alias.scope !8195, !noalias !8582, !noundef !11
  %_3.i263.i = fcmp uge float %_121.i1600, %gain_right.sroa.0.0871.i, !dbg !8583
  %_4.i458.i = select i1 %_3.i263.i, i32 %_0.i332739.i, i32 %_0.i333738.i, !dbg !8586
  %_0.i459.i = bitcast i32 %_4.i458.i to float, !dbg !8588
  %_0.i321.i = fsub float %_121.i1600, %gain_right.sroa.0.0871.i, !dbg !8590
  %_4.i280.i = fmul float %_0.i321.i, %_0.i459.i, !dbg !8593
  %_0.i281.i = fadd float %gain_right.sroa.0.0871.i, %_4.i280.i, !dbg !8593
  %123 = tail call noundef float @llvm.fabs.f32(float %_0.i281.i), !dbg !8595
  %124 = fcmp uge float %123, 0x3BC79CA100000000, !dbg !8598
  %_0.i419.i = select i1 %124, float %_0.i281.i, float 0.000000e+00, !dbg !8600
  store float %_0.i419.i, ptr %_3.i1.i.i, align 4, !dbg !8601, !alias.scope !8195, !noalias !8582
  %exitcond937.not.i = icmp eq i64 %120, %_34, !dbg !8543
  br i1 %exitcond937.not.i, label %bb43.preheader.i, label %bb36.i, !dbg !8543

bb43.preheader.i:                                 ; preds = %bb36.i
  store float %_0.i415.i, ptr %116, align 8, !dbg !8602, !alias.scope !8185, !noalias !8201
  store float %_0.i419.i, ptr %118, align 8, !dbg !8603, !alias.scope !8187, !noalias !8494
  br label %bb43.i1601, !dbg !8604

bb43.i1601:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit373.i, %bb43.preheader.i
  %iter3.sroa.13.0878.i = phi i64 [ %_9.0.i714.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit373.i ], [ 0, %bb43.preheader.i ]
  %_9.0.i714.i = add nuw nsw i64 %iter3.sroa.13.0878.i, 1, !dbg !8611
  %_3.i.i.i.i710.i = getelementptr inbounds nuw float, ptr %_52.0, i64 %iter3.sroa.13.0878.i, !dbg !8616
  %_3.i1.i.i.i.i = getelementptr inbounds nuw float, ptr %_53.0, i64 %iter3.sroa.13.0878.i, !dbg !8626
  %_138.i = add nuw nsw i64 %iter3.sroa.13.0878.i, %spec.store.select, !dbg !8629
  %_311.i = icmp samesign ugt i64 %_138.i, %left.1, !dbg !8604
  br i1 %_311.i, label %bb94.i, label %bb95.i, !dbg !8604, !prof !161

bb95.i:                                           ; preds = %bb43.i1601
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8630), !dbg !8633
  %_3.not.i374.i = icmp eq i64 %left.1, %_138.i, !dbg !8634
  br i1 %_3.not.i374.i, label %panic.i376.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit377.i, !dbg !8634

panic.i376.i:                                     ; preds = %bb95.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !8634, !noalias !8636
  unreachable, !dbg !8634

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit377.i: ; preds = %bb95.i
  %_318.i = getelementptr inbounds nuw float, ptr %left.0, i64 %_138.i, !dbg !8637
  %_0.i375.i = load float, ptr %_318.i, align 4, !dbg !8634, !alias.scope !8643, !noalias !8644, !noundef !11
  %_143.i = load float, ptr %_3.i.i.i.i710.i, align 4, !dbg !8645, !alias.scope !8193, !noalias !8561, !noundef !11
  %_0.i275.i = fadd float %_0.i331.i, %_143.i, !dbg !8647
  %_0.i306.i = fmul float %_0.i275.i, 0x3FC542A5A0000000, !dbg !8650
  %_3.i.i531.inv.i = fcmp ogt float %_0.i306.i, -1.260000e+02, !dbg !8653
  %_0.i.i538.i = select i1 %_3.i.i531.inv.i, float %_0.i306.i, float -1.260000e+02, !dbg !8653
  %_3.i.i609.inv.i = fcmp olt float %_0.i.i538.i, 1.270000e+02, !dbg !8657
  %_0.i.i616.i = select i1 %_3.i.i609.inv.i, float %_0.i.i538.i, float 1.270000e+02, !dbg !8657
  %125 = tail call noundef float @llvm.floor.f32(float %_0.i.i616.i), !dbg !8660
  %_0.i317.i = fsub float %_0.i.i616.i, %125, !dbg !8664
  %_0.i295.i = fmul float %_0.i317.i, 0x3F5E974FA0000000, !dbg !8666
  %_0.i272.i = fadd float %_0.i295.i, 0x3F82778560000000, !dbg !8668
  %_0.i295.1.i = fmul float %_0.i317.i, %_0.i272.i, !dbg !8666
  %_0.i272.1.i = fadd float %_0.i295.1.i, 0x3FAC91CE60000000, !dbg !8668
  %_0.i295.2.i = fmul float %_0.i317.i, %_0.i272.1.i, !dbg !8666
  %_0.i272.2.i = fadd float %_0.i295.2.i, 0x3FCEBDB560000000, !dbg !8668
  %_0.i295.3.i = fmul float %_0.i317.i, %_0.i272.2.i, !dbg !8666
  %_0.i272.3.i = fadd float %_0.i295.3.i, 0x3FE62E4BA0000000, !dbg !8668
  %_0.i294.i = fmul float %_0.i317.i, %_0.i272.3.i, !dbg !8670
  %_0.i271.i = fadd float %_0.i294.i, 1.000000e+00, !dbg !8672
  %biased.i230.i = fadd float %125, 0x4160000FE0000000, !dbg !8674
  %_4.i231.i = bitcast float %biased.i230.i to i32, !dbg !8676
  %_3.i232.i = shl i32 %_4.i231.i, 23, !dbg !8678
  %_0.i233.i = bitcast i32 %_3.i232.i to float, !dbg !8679
  %_0.i293.i = fmul float %_0.i271.i, %_0.i233.i, !dbg !8681
  %_0.i305.i = fmul float %_0.i375.i, %_0.i293.i, !dbg !8683
  %_0.i322.i = fsub float %_0.i305.i, %_0.i375.i, !dbg !8685
  %_4.i282.i = fmul float %_0.i330.i, %_0.i322.i, !dbg !8688
  %_0.i283.i = fadd float %_0.i375.i, %_4.i282.i, !dbg !8688
  %_3.i245.i = fcmp oeq float %_143.i, 0.000000e+00, !dbg !8690
  %_0.i513742.i = and i1 %_3.i234.i, %_3.i245.i, !dbg !8692
  %126 = or i1 %_3.i235.i, %_0.i513742.i
  %_0.i509744.reass.reass.i.reass.reass = or i1 %126, %bypass
  %_4.i472.v.i = select i1 %_3.i237.i, float %_0.i283.i, float %_0.i305.i, !dbg !8694
  %_4.i465.v.i = select i1 %_0.i509744.reass.reass.i.reass.reass, float %_0.i375.i, float %_4.i472.v.i, !dbg !8696
  store float %_4.i465.v.i, ptr %_318.i, align 4, !dbg !8698, !alias.scope !8700, !noalias !8644
  %_323.i = icmp samesign ugt i64 %_138.i, %right.1, !dbg !8703
  br i1 %_323.i, label %bb96.i, label %bb97.i, !dbg !8703, !prof !161

bb94.i:                                           ; preds = %bb43.i1601
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_138.i, i64 noundef range(i64 0, 2305843009213693952) %left.1, i64 noundef range(i64 0, 2305843009213693952) %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d76c0cfb43c310b376f3b4a5d480172c) #23, !dbg !8707, !noalias !8486
  unreachable, !dbg !8707

bb97.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit377.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8708), !dbg !8711
  %_3.not.i370.i = icmp eq i64 %right.1, %_138.i, !dbg !8712
  br i1 %_3.not.i370.i, label %panic.i372.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit373.i, !dbg !8712

panic.i372.i:                                     ; preds = %bb97.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !8712, !noalias !8714
  unreachable, !dbg !8712

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit373.i: ; preds = %bb97.i
  %_330.i = getelementptr inbounds nuw float, ptr %right.0, i64 %_138.i, !dbg !8715
  %_0.i371.i = load float, ptr %_330.i, align 4, !dbg !8712, !alias.scope !8720, !noalias !8721, !noundef !11
  %_151.i = load float, ptr %_3.i1.i.i.i.i, align 4, !dbg !8722, !alias.scope !8195, !noalias !8582, !noundef !11
  %_0.i276.i = fadd float %_0.i339.i, %_151.i, !dbg !8724
  %_0.i309.i = fmul float %_0.i276.i, 0x3FC542A5A0000000, !dbg !8727
  %_3.i.i523.inv.i = fcmp ogt float %_0.i309.i, -1.260000e+02, !dbg !8730
  %_0.i.i530.i = select i1 %_3.i.i523.inv.i, float %_0.i309.i, float -1.260000e+02, !dbg !8730
  %_3.i.i601.inv.i = fcmp olt float %_0.i.i530.i, 1.270000e+02, !dbg !8734
  %_0.i.i608.i = select i1 %_3.i.i601.inv.i, float %_0.i.i530.i, float 1.270000e+02, !dbg !8734
  %127 = tail call noundef float @llvm.floor.f32(float %_0.i.i608.i), !dbg !8737
  %_0.i316.i = fsub float %_0.i.i608.i, %127, !dbg !8741
  %_0.i292.i = fmul float %_0.i316.i, 0x3F5E974FA0000000, !dbg !8743
  %_0.i270.i = fadd float %_0.i292.i, 0x3F82778560000000, !dbg !8745
  %_0.i292.1.i = fmul float %_0.i316.i, %_0.i270.i, !dbg !8743
  %_0.i270.1.i = fadd float %_0.i292.1.i, 0x3FAC91CE60000000, !dbg !8745
  %_0.i292.2.i = fmul float %_0.i316.i, %_0.i270.1.i, !dbg !8743
  %_0.i270.2.i = fadd float %_0.i292.2.i, 0x3FCEBDB560000000, !dbg !8745
  %_0.i292.3.i = fmul float %_0.i316.i, %_0.i270.2.i, !dbg !8743
  %_0.i270.3.i = fadd float %_0.i292.3.i, 0x3FE62E4BA0000000, !dbg !8745
  %_0.i291.i = fmul float %_0.i316.i, %_0.i270.3.i, !dbg !8747
  %_0.i269.i = fadd float %_0.i291.i, 1.000000e+00, !dbg !8749
  %biased.i.i = fadd float %127, 0x4160000FE0000000, !dbg !8751
  %_4.i227.i = bitcast float %biased.i.i to i32, !dbg !8753
  %_3.i228.i = shl i32 %_4.i227.i, 23, !dbg !8755
  %_0.i229.i = bitcast i32 %_3.i228.i to float, !dbg !8756
  %_0.i290.i = fmul float %_0.i269.i, %_0.i229.i, !dbg !8758
  %_0.i308.i = fmul float %_0.i371.i, %_0.i290.i, !dbg !8760
  %_0.i323.i = fsub float %_0.i308.i, %_0.i371.i, !dbg !8762
  %_4.i284.i = fmul float %_0.i338.i, %_0.i323.i, !dbg !8765
  %_0.i285.i = fadd float %_0.i371.i, %_4.i284.i, !dbg !8765
  %_3.i247.i = fcmp oeq float %_151.i, 0.000000e+00, !dbg !8767
  %_0.i514749.i = and i1 %_3.i239.i, %_3.i247.i, !dbg !8769
  %128 = or i1 %_3.i241.i, %_0.i514749.i
  %_0.i511751.reass.reass.i.reass.reass = or i1 %128, %bypass
  %_4.i486.v.i = select i1 %_3.i243.i, float %_0.i285.i, float %_0.i308.i, !dbg !8771
  %_4.i479.v.i = select i1 %_0.i511751.reass.reass.i.reass.reass, float %_0.i371.i, float %_4.i486.v.i, !dbg !8773
  store float %_4.i479.v.i, ptr %_330.i, align 4, !dbg !8775, !alias.scope !8777, !noalias !8721
  %exitcond938.not.i = icmp eq i64 %_9.0.i714.i, %_34, !dbg !8780
  br i1 %exitcond938.not.i, label %bb19, label %bb43.i1601, !dbg !8780

bb96.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit377.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_138.i, i64 noundef range(i64 0, 2305843009213693952) %right.1, i64 noundef range(i64 0, 2305843009213693952) %right.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7159ff60b58ea8ffb865744fc2de0bda) #23, !dbg !8781, !noalias !8486
  unreachable, !dbg !8781

bb69.i:                                           ; preds = %bb14.i1587
  %_212.i = getelementptr inbounds nuw float, ptr %left.0, i64 %_51.i, !dbg !8782
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8786), !dbg !8789
  %_3.not.i366.i = icmp eq i64 %left.1, %_51.i, !dbg !8790
  br i1 %_3.not.i366.i, label %panic.i368.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit369.i, !dbg !8790

panic.i368.i:                                     ; preds = %bb69.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !8790, !noalias !8792
  unreachable, !dbg !8790

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit369.i: ; preds = %bb69.i
  %_0.i367.i = load float, ptr %_212.i, align 4, !dbg !8790, !alias.scope !8793, !noalias !8644, !noundef !11
  %_216.i = icmp samesign ugt i64 %_51.i, %right.1, !dbg !8794
  br i1 %_216.i, label %bb70.i, label %bb71.i, !dbg !8794, !prof !161

bb68.i:                                           ; preds = %bb14.i1587
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_51.i, i64 noundef range(i64 0, 2305843009213693952) %left.1, i64 noundef range(i64 0, 2305843009213693952) %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_142618cca8840ae208db8a98d223f1fd) #23, !dbg !8799, !noalias !8486
  unreachable, !dbg !8799

bb71.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit369.i
  %_223.i = getelementptr inbounds nuw float, ptr %right.0, i64 %_51.i, !dbg !8800
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8805), !dbg !8808
  %_3.not.i362.i = icmp eq i64 %right.1, %_51.i, !dbg !8809
  br i1 %_3.not.i362.i, label %panic.i364.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit365.i, !dbg !8809

panic.i364.i:                                     ; preds = %bb71.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !8809, !noalias !8811
  unreachable, !dbg !8809

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit365.i: ; preds = %bb71.i
  %_0.i363.i = load float, ptr %_223.i, align 4, !dbg !8809, !alias.scope !8812, !noalias !8721, !noundef !11
  switch i64 %_6.i.i1582, label %default.unreachable [
    i64 0, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i1595
    i64 1, label %bb3.i.i1605
    i64 2, label %bb2.i.i1588
  ], !dbg !8813

bb3.i.i1605:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit365.i
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i1595, !dbg !8816

bb2.i.i1588:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit365.i
  %_25.i.i1589 = icmp ugt i64 %_51.i, %sidechain_left.1.i.i1583, !dbg !8817
  br i1 %_25.i.i1589, label %bb17.i.i1604, label %bb18.i.i1590, !dbg !8817, !prof !161

bb18.i.i1590:                                     ; preds = %bb2.i.i1588
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8820), !dbg !8823
  %_3.not.i342.i = icmp eq i64 %sidechain_left.1.i.i1583, %_51.i, !dbg !8824
  br i1 %_3.not.i342.i, label %panic.i344.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit345.i, !dbg !8824

panic.i344.i:                                     ; preds = %bb18.i.i1590
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !8824, !noalias !8826
  unreachable, !dbg !8824

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit345.i: ; preds = %bb18.i.i1590
  %_32.i.i1591 = getelementptr inbounds nuw float, ptr %sidechain_left.0.i.i1586, i64 %_51.i, !dbg !8830
  %_0.i343.i = load float, ptr %_32.i.i1591, align 4, !dbg !8824, !alias.scope !8820, !noalias !8832, !noundef !11
  %_33.i.i1592 = icmp ugt i64 %_51.i, %sidechain_right.1.i.i1585, !dbg !8833
  br i1 %_33.i.i1592, label %bb19.i.i1603, label %bb20.i.i1593, !dbg !8833, !prof !161

bb17.i.i1604:                                     ; preds = %bb2.i.i1588
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 2305843009213693952) %_51.i, i64 noundef %sidechain_left.1.i.i1583, i64 noundef %sidechain_left.1.i.i1583, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7fe5a91ec575deda110c046c84630100) #23, !dbg !8836, !noalias !8832
  unreachable, !dbg !8836

bb20.i.i1593:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit345.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8837), !dbg !8840
  %_3.not.i.i = icmp eq i64 %sidechain_right.1.i.i1585, %_51.i, !dbg !8841
  br i1 %_3.not.i.i, label %panic.i341.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i, !dbg !8841

panic.i341.i:                                     ; preds = %bb20.i.i1593
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !8841, !noalias !8843
  unreachable, !dbg !8841

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i: ; preds = %bb20.i.i1593
  %_40.i.i1594 = getelementptr inbounds nuw float, ptr %sidechain_right.0.i.i1584, i64 %_51.i, !dbg !8844
  %_0.i340.i = load float, ptr %_40.i.i1594, align 4, !dbg !8841, !alias.scope !8837, !noalias !8832, !noundef !11
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i1595, !dbg !8846

bb19.i.i1603:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit345.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 2305843009213693952) %_51.i, i64 noundef %sidechain_right.1.i.i1585, i64 noundef %sidechain_right.1.i.i1585, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f2cba1f89421a726339097d2eba48536) #23, !dbg !8847, !noalias !8832
  unreachable, !dbg !8847

_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i1595: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i, %bb3.i.i1605, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit365.i
  %main_right.sroa.0.0.i.i1596 = phi float [ %_0.i363.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit365.i ], [ 0.000000e+00, %bb3.i.i1605 ], [ %_0.i340.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i ]
  %main_left.sroa.0.0.i.i1597 = phi float [ %_0.i367.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit365.i ], [ 0.000000e+00, %bb3.i.i1605 ], [ %_0.i343.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i ]
  %129 = tail call noundef float @llvm.fabs.f32(float %main_left.sroa.0.0.i.i1597), !dbg !8848
  %130 = tail call noundef float @llvm.fabs.f32(float %main_right.sroa.0.0.i.i1596), !dbg !8850
  %_3.i.i592.i = fcmp ule float %129, %130, !dbg !8852
  %_6.i.i594.i = bitcast float %129 to i32, !dbg !8855
  %_8.i.i596.i = bitcast float %130 to i32, !dbg !8858
  %_4.i.i599.i = select i1 %_3.i.i592.i, i32 %_8.i.i596.i, i32 %_6.i.i594.i, !dbg !8860
  %_0.i313.i = fmul float %129, 5.000000e-01, !dbg !8861
  %_0.i312.i = fmul float %130, 5.000000e-01, !dbg !8863
  %_0.i277.i = fadd float %_0.i312.i, %_0.i313.i, !dbg !8865
  %_6.i502.i = bitcast float %_0.i277.i to i32, !dbg !8867
  %_4.i507.i = select i1 %.not.i, i32 %_6.i502.i, i32 %_4.i.i599.i, !dbg !8870
  %_4.i500.i = select i1 %94, i32 %_6.i.i594.i, i32 %_4.i507.i, !dbg !8871
  %_4.i493.i = select i1 %94, i32 %_8.i.i596.i, i32 %_4.i507.i, !dbg !8873
  %131 = add i64 %head.sroa.0.0868.i, 1, !dbg !8875
  %_62.i = icmp eq i64 %131, %ring_length.i1580, !dbg !8877
  %spec.store.select.i1598 = select i1 %_62.i, i64 0, i64 %131, !dbg !8877
  %_224.i = icmp ugt i64 %head.sroa.0.0868.i, %_335.1.i, !dbg !8879
  br i1 %_224.i, label %bb72.i, label %bb73.i, !dbg !8879, !prof !161

bb70.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit369.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_51.i, i64 noundef range(i64 0, 2305843009213693952) %right.1, i64 noundef range(i64 0, 2305843009213693952) %right.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e7322f5a021c15e2c563763633ec7262) #23, !dbg !8886, !noalias !8486
  unreachable, !dbg !8886

bb73.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i1595
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8887), !dbg !8890
  %_4.not.i403.i = icmp eq i64 %_335.1.i, %head.sroa.0.0868.i, !dbg !8891
  br i1 %_4.not.i403.i, label %panic.i404.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit405.i, !dbg !8891

panic.i404.i:                                     ; preds = %bb73.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #23, !dbg !8891, !noalias !8893
  unreachable, !dbg !8891

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit405.i: ; preds = %bb73.i
  %_231.i = getelementptr inbounds nuw float, ptr %_335.0.i, i64 %head.sroa.0.0868.i, !dbg !8894
  store float %_0.i367.i, ptr %_231.i, align 4, !dbg !8891, !alias.scope !8887, !noalias !8486
  %_232.i = icmp ugt i64 %head.sroa.0.0868.i, %_143.1.i.i, !dbg !8899
  br i1 %_232.i, label %bb74.i, label %bb75.i, !dbg !8899, !prof !161

bb72.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_framefEB4_.exit.i1595
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %head.sroa.0.0868.i, i64 noundef %_335.1.i, i64 noundef %_335.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_797e6d44639a19f9bc9393121579dd3c) #23, !dbg !8903, !noalias !8486
  unreachable, !dbg !8903

bb75.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit405.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8904), !dbg !8907
  %_4.not.i400.i = icmp eq i64 %_143.1.i.i, %head.sroa.0.0868.i, !dbg !8908
  br i1 %_4.not.i400.i, label %panic.i401.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit402.i, !dbg !8908

panic.i401.i:                                     ; preds = %bb75.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #23, !dbg !8908, !noalias !8910
  unreachable, !dbg !8908

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit402.i: ; preds = %bb75.i
  %_239.i = getelementptr inbounds nuw float, ptr %_143.0.i.i, i64 %head.sroa.0.0868.i, !dbg !8911
  store i32 %_4.i500.i, ptr %_239.i, align 4, !dbg !8908, !alias.scope !8904, !noalias !8486
  %_240.i = icmp ugt i64 %head.sroa.0.0868.i, %_337.1.i, !dbg !8916
  br i1 %_240.i, label %bb76.i, label %bb77.i, !dbg !8916, !prof !161

bb74.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit405.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %head.sroa.0.0868.i, i64 noundef %_143.1.i.i, i64 noundef %_143.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8cd1878abf6085d06f7a836874ffa4c2) #23, !dbg !8920, !noalias !8486
  unreachable, !dbg !8920

bb77.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit402.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8921), !dbg !8924
  %_4.not.i397.i = icmp eq i64 %_337.1.i, %head.sroa.0.0868.i, !dbg !8925
  br i1 %_4.not.i397.i, label %panic.i398.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit399.i, !dbg !8925

panic.i398.i:                                     ; preds = %bb77.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #23, !dbg !8925, !noalias !8927
  unreachable, !dbg !8925

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit399.i: ; preds = %bb77.i
  %_247.i = getelementptr inbounds nuw float, ptr %_337.0.i, i64 %head.sroa.0.0868.i, !dbg !8928
  store float %_0.i363.i, ptr %_247.i, align 4, !dbg !8925, !alias.scope !8921, !noalias !8486
  %_248.i = icmp ugt i64 %head.sroa.0.0868.i, %_143.1.i669.i, !dbg !8933
  br i1 %_248.i, label %bb78.i, label %bb79.i, !dbg !8933, !prof !161

bb76.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit402.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %head.sroa.0.0868.i, i64 noundef %_337.1.i, i64 noundef %_337.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8529cd63341985e7fb6803b2c00d96ec) #23, !dbg !8937, !noalias !8486
  unreachable, !dbg !8937

bb79.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit399.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8938), !dbg !8941
  %_4.not.i394.i = icmp eq i64 %_143.1.i669.i, %head.sroa.0.0868.i, !dbg !8942
  br i1 %_4.not.i394.i, label %panic.i395.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit396.i, !dbg !8942

panic.i395.i:                                     ; preds = %bb79.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #23, !dbg !8942, !noalias !8944
  unreachable, !dbg !8942

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit396.i: ; preds = %bb79.i
  %_255.i = getelementptr inbounds nuw float, ptr %_143.0.i668.i, i64 %head.sroa.0.0868.i, !dbg !8945
  store i32 %_4.i493.i, ptr %_255.i, align 4, !dbg !8942, !alias.scope !8938, !noalias !8486
  %_256.i = icmp ugt i64 %spec.store.select.i1598, %_335.1.i, !dbg !8950
  br i1 %_256.i, label %bb80.i, label %bb81.i, !dbg !8950, !prof !161

bb78.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit399.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %head.sroa.0.0868.i, i64 noundef %_143.1.i669.i, i64 noundef %_143.1.i669.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_50f064baf8dd86aaabd3fcd49badf48e) #23, !dbg !8954, !noalias !8486
  unreachable, !dbg !8954

bb81.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit396.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8955), !dbg !8958
  %_3.not.i358.i = icmp eq i64 %_335.1.i, %spec.store.select.i1598, !dbg !8959
  br i1 %_3.not.i358.i, label %panic.i360.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit393.i, !dbg !8959

panic.i360.i:                                     ; preds = %bb81.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !8959, !noalias !8961
  unreachable, !dbg !8959

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit393.i: ; preds = %bb81.i
  %_263.i = getelementptr inbounds nuw float, ptr %_335.0.i, i64 %spec.store.select.i1598, !dbg !8962
  %_0.i359.i = load float, ptr %_263.i, align 4, !dbg !8959, !alias.scope !8955, !noalias !8486, !noundef !11
  store float %_0.i359.i, ptr %_212.i, align 4, !dbg !8967, !alias.scope !8969, !noalias !8644
  %_268.i = icmp ugt i64 %spec.store.select.i1598, %_337.1.i, !dbg !8972
  br i1 %_268.i, label %bb82.i, label %bb83.i, !dbg !8972, !prof !161

bb80.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit396.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %spec.store.select.i1598, i64 noundef %_335.1.i, i64 noundef %_335.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_0171cce559b4754ea869128a8d469334) #23, !dbg !8976, !noalias !8486
  unreachable, !dbg !8976

bb83.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit393.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8977), !dbg !8980
  %_3.not.i354.i = icmp eq i64 %_337.1.i, %spec.store.select.i1598, !dbg !8981
  br i1 %_3.not.i354.i, label %panic.i356.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit353.i, !dbg !8981

panic.i356.i:                                     ; preds = %bb83.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #23, !dbg !8981, !noalias !8983
  unreachable, !dbg !8981

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit353.i: ; preds = %bb83.i
  %_275.i = getelementptr inbounds nuw float, ptr %_337.0.i, i64 %spec.store.select.i1598, !dbg !8984
  %_0.i355.i = load float, ptr %_275.i, align 4, !dbg !8981, !alias.scope !8977, !noalias !8486, !noundef !11
  store float %_0.i355.i, ptr %_223.i, align 4, !dbg !8989, !alias.scope !8991, !noalias !8721
  %_0.i351.i = load float, ptr %data.i.i.i.i.i.i.i.i.i, align 4, !dbg !8994, !alias.scope !8996, !noalias !8999, !noundef !11
  %_3.i.i556.i = fcmp ule float %_0.i351.i, 0x3E45798EE0000000, !dbg !9000
  %_6.i.i558.i = bitcast float %_0.i351.i to i32, !dbg !9004
  %_4.i.i563.i = select i1 %_3.i.i556.i, i32 841731191, i32 %_6.i.i558.i, !dbg !9007
  %_0.i.i564.i = bitcast i32 %_4.i.i563.i to float, !dbg !9008
  %_3.i.i515.i = fcmp ule float %_0.i.i564.i, 0x3810000000000000, !dbg !9010
  %_4.i.i521.i = select i1 %_3.i.i515.i, i32 8388608, i32 %_4.i.i563.i, !dbg !9015
  %_5.i383.i = and i32 %_4.i.i521.i, 8388607, !dbg !9017
  %_4.i384.i = or disjoint i32 %_5.i383.i, 1065353216, !dbg !9017
  %significand.i385.i = bitcast i32 %_4.i384.i to float, !dbg !9019
  %_0.i315.i = fadd float %significand.i385.i, -1.000000e+00, !dbg !9021
  %_0.i289.i = fmul float %_0.i315.i, 0x3F9B17A960000000, !dbg !9023
  %132 = fsub float 0x3FBF9A8440000000, %_0.i289.i, !dbg !9025
  %_0.i289.1.i = fmul float %_0.i315.i, %132, !dbg !9023
  %_0.i268.1.i = fadd float %_0.i289.1.i, 0xBFD1E3F400000000, !dbg !9025
  %_0.i289.2.i = fmul float %_0.i315.i, %_0.i268.1.i, !dbg !9023
  %_0.i268.2.i = fadd float %_0.i289.2.i, 0x3FDD544F20000000, !dbg !9025
  %_0.i289.3.i = fmul float %_0.i315.i, %_0.i268.2.i, !dbg !9023
  %_0.i268.3.i = fadd float %_0.i289.3.i, 0xBFE6FC2A60000000, !dbg !9025
  %_0.i289.4.i = fmul float %_0.i315.i, %_0.i268.3.i, !dbg !9023
  %_0.i268.4.i = fadd float %_0.i289.4.i, 0x3FF714B2A0000000, !dbg !9025
  %_9.i386.i = lshr i32 %_4.i.i521.i, 23, !dbg !9027
  %_8.i387.i = or disjoint i32 %_9.i386.i, 1258291200, !dbg !9027
  %_7.i388.i = bitcast i32 %_8.i387.i to float, !dbg !9028
  %exponent.i389.i = fadd float %_7.i388.i, 0xC160000FE0000000, !dbg !9030
  %_0.i288.i = fmul float %_0.i315.i, %_0.i268.4.i, !dbg !9031
  %_0.i267.i = fadd float %exponent.i389.i, %_0.i288.i, !dbg !9033
  %_0.i310.i = fmul float %_0.i267.i, 0x4018151820000000, !dbg !9035
  %_3.i.i547.inv.i = fcmp ogt float %_0.i310.i, -1.600000e+02, !dbg !9037
  %_0.i.i555.i = select i1 %_3.i.i547.inv.i, float %_0.i310.i, float -1.600000e+02, !dbg !9037
  %_3.i.i626.inv.i = fcmp olt float %_0.i.i555.i, 2.400000e+01, !dbg !9040
  %_0.i.i634.i = select i1 %_3.i.i626.inv.i, float %_0.i.i555.i, float 2.400000e+01, !dbg !9040
  %_0.i319.i = fsub float %_0.i.i634.i, %_0.i329.i, !dbg !9043
  %_3.i251.i = fcmp ule float %_0.i319.i, %_0.i327.i, !dbg !9046
  %_0.i274.i = fadd float %_0.i327.i, %_0.i319.i, !dbg !9048
  %_0.i302.i = fmul float %_0.i274.i, %_0.i274.i, !dbg !9050
  %_0.i301.i = fmul float %_0.i326.i, %_0.i302.i, !dbg !9052
  %_4.i444.v.v.i = select i1 %_3.i251.i, float %_0.i301.i, float %_0.i319.i, !dbg !9054
  %_4.i444.v.i = fmul float %_0.i328.i, %_4.i444.v.v.i, !dbg !9054
  %133 = fcmp ugt float %_0.i319.i, %112, !dbg !9056
  %_0.i438.i = select i1 %133, float %_4.i444.v.i, float 0.000000e+00, !dbg !9058
  %_3.i.i539.i.inv = fcmp ogt float %_0.i438.i, -1.000000e+02, !dbg !9059
  %_0.i.i546.i = select i1 %_3.i.i539.i.inv, float %_0.i438.i, float -1.000000e+02, !dbg !9059
  %_3.i.i617.i = fcmp olt float %_0.i.i546.i, 0.000000e+00, !dbg !9062
  %_0.i.i625.i = select i1 %_3.i.i617.i, float %_0.i.i546.i, float 0.000000e+00, !dbg !9065
  store float %_0.i.i625.i, ptr %_3.i.i.i.i.i.i.i, align 4, !dbg !9067, !alias.scope !8193, !noalias !8561
  %_0.i347.i = load float, ptr %data.i5.i.i.i.i.i.i.i.i, align 4, !dbg !9068, !alias.scope !9070, !noalias !9073, !noundef !11
  %_3.i.i583.i = fcmp ule float %_0.i347.i, 0x3E45798EE0000000, !dbg !9074
  %_6.i.i585.i = bitcast float %_0.i347.i to i32, !dbg !9078
  %_4.i.i590.i = select i1 %_3.i.i583.i, i32 841731191, i32 %_6.i.i585.i, !dbg !9081
  %_0.i.i591.i = bitcast i32 %_4.i.i590.i to float, !dbg !9082
  %_3.i.i.i = fcmp ule float %_0.i.i591.i, 0x3810000000000000, !dbg !9084
  %_4.i.i.i = select i1 %_3.i.i.i, i32 8388608, i32 %_4.i.i590.i, !dbg !9089
  %_5.i378.i = and i32 %_4.i.i.i, 8388607, !dbg !9091
  %_4.i379.i = or disjoint i32 %_5.i378.i, 1065353216, !dbg !9091
  %significand.i.i = bitcast i32 %_4.i379.i to float, !dbg !9093
  %_0.i314.i = fadd float %significand.i.i, -1.000000e+00, !dbg !9095
  %_0.i287.i = fmul float %_0.i314.i, 0x3F9B17A960000000, !dbg !9097
  %134 = fsub float 0x3FBF9A8440000000, %_0.i287.i, !dbg !9099
  %_0.i287.1.i = fmul float %_0.i314.i, %134, !dbg !9097
  %_0.i266.1.i = fadd float %_0.i287.1.i, 0xBFD1E3F400000000, !dbg !9099
  %_0.i287.2.i = fmul float %_0.i314.i, %_0.i266.1.i, !dbg !9097
  %_0.i266.2.i = fadd float %_0.i287.2.i, 0x3FDD544F20000000, !dbg !9099
  %_0.i287.3.i = fmul float %_0.i314.i, %_0.i266.2.i, !dbg !9097
  %_0.i266.3.i = fadd float %_0.i287.3.i, 0xBFE6FC2A60000000, !dbg !9099
  %_0.i287.4.i = fmul float %_0.i314.i, %_0.i266.3.i, !dbg !9097
  %_0.i266.4.i = fadd float %_0.i287.4.i, 0x3FF714B2A0000000, !dbg !9099
  %_9.i380.i = lshr i32 %_4.i.i.i, 23, !dbg !9101
  %_8.i.i1599 = or disjoint i32 %_9.i380.i, 1258291200, !dbg !9101
  %_7.i381.i = bitcast i32 %_8.i.i1599 to float, !dbg !9102
  %exponent.i.i = fadd float %_7.i381.i, 0xC160000FE0000000, !dbg !9104
  %_0.i286.i = fmul float %_0.i314.i, %_0.i266.4.i, !dbg !9105
  %_0.i265.i = fadd float %exponent.i.i, %_0.i286.i, !dbg !9107
  %_0.i311.i = fmul float %_0.i265.i, 0x4018151820000000, !dbg !9109
  %_3.i.i574.inv.i = fcmp ogt float %_0.i311.i, -1.600000e+02, !dbg !9111
  %_0.i.i582.i = select i1 %_3.i.i574.inv.i, float %_0.i311.i, float -1.600000e+02, !dbg !9111
  %_3.i.i644.inv.i = fcmp olt float %_0.i.i582.i, 2.400000e+01, !dbg !9114
  %_0.i.i652.i = select i1 %_3.i.i644.inv.i, float %_0.i.i582.i, float 2.400000e+01, !dbg !9114
  %_0.i318.i = fsub float %_0.i.i652.i, %_0.i337.i, !dbg !9117
  %_3.i249.i = fcmp ule float %_0.i318.i, %_0.i335.i, !dbg !9120
  %_0.i273.i = fadd float %_0.i335.i, %_0.i318.i, !dbg !9122
  %_0.i298.i = fmul float %_0.i273.i, %_0.i273.i, !dbg !9124
  %_0.i297.i = fmul float %_0.i334.i, %_0.i298.i, !dbg !9126
  %_4.i431.v.v.i = select i1 %_3.i249.i, float %_0.i297.i, float %_0.i318.i, !dbg !9128
  %_4.i431.v.i = fmul float %_0.i336.i, %_4.i431.v.v.i, !dbg !9128
  %135 = fcmp ugt float %_0.i318.i, %113, !dbg !9130
  %_0.i425.i = select i1 %135, float %_4.i431.v.i, float 0.000000e+00, !dbg !9132
  %_3.i.i565.i.inv = fcmp ogt float %_0.i425.i, -1.000000e+02, !dbg !9133
  %_0.i.i573.i = select i1 %_3.i.i565.i.inv, float %_0.i425.i, float -1.000000e+02, !dbg !9133
  %_3.i.i635.i = fcmp olt float %_0.i.i573.i, 0.000000e+00, !dbg !9136
  %_0.i.i643.i = select i1 %_3.i.i635.i, float %_0.i.i573.i, float 0.000000e+00, !dbg !9139
  store float %_0.i.i643.i, ptr %_3.i.i.i.i.i, align 4, !dbg !9141, !alias.scope !8195, !noalias !8582
  %exitcond.not.i = icmp eq i64 %_9.0.i.i, %_34, !dbg !8495
  br i1 %exitcond.not.i, label %bb15.i, label %bb14.i1587, !dbg !8495

bb82.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit393.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %spec.store.select.i1598, i64 noundef %_337.1.i, i64 noundef %_337.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_24cf5413af95bfbc1de92cceabc9bf67) #23, !dbg !9142, !noalias !8486
  unreachable, !dbg !9142
}

define internal fastcc void @_RINvNtCse3bfmKSZS8Y_10compressor6kernel18process_block_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_(ptr noalias noundef nonnull align 4 captures(none) %left.0, i64 noundef range(i64 8, 34359738361) %left.1, ptr dead_on_return noalias noundef nonnull readonly align 8 captures(none) dereferenceable(40) %detector, i64 noundef range(i64 1, 4294967296) %frames, i32 noundef range(i32 1, 4) %link, i1 noundef zeroext %bypass, i32 noundef %sample_rate, ptr noalias noundef nonnull align 32 dereferenceable(1568) %channel_left, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(64) %staged) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !9143 {
start:
; call <compressor::kernel::Channel<wide::f32x8_::f32x8>>::max_remaining
  %_13 = tail call fastcc noundef i32 @_RNvMNtCse3bfmKSZS8Y_10compressor6kernelINtB2_7ChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13max_remainingB4_(ptr noalias noundef readonly align 32 captures(address, read_provenance) dereferenceable(1568) %channel_left) #22, !dbg !9144
  %0 = zext i32 %_13 to i64, !dbg !9145
  %_15 = icmp samesign ugt i64 %frames, %0, !dbg !9146
  %spec.store.select = tail call i64 @llvm.umin.i64(i64 %frames, i64 %0), !dbg !9146
  %_16.not = icmp eq i32 %_13, 0, !dbg !9148
  br i1 %_16.not, label %bb8, label %bb5, !dbg !9148

bb5:                                              ; preds = %start
  %.sroa.0.0.copyload = load i64, ptr %detector, align 8, !dbg !9150
  %.sroa.4.0.detector.sroa_idx = getelementptr inbounds nuw i8, ptr %detector, i64 8, !dbg !9150
  %.sroa.4.0.copyload = load ptr, ptr %.sroa.4.0.detector.sroa_idx, align 8, !dbg !9150
  %.sroa.5.0.detector.sroa_idx = getelementptr inbounds nuw i8, ptr %detector, i64 16, !dbg !9150
  %.sroa.5.0.copyload = load i64, ptr %.sroa.5.0.detector.sroa_idx, align 8, !dbg !9150
  %.sroa.6.0.detector.sroa_idx = getelementptr inbounds nuw i8, ptr %detector, i64 24, !dbg !9150
  %.sroa.6.0.copyload = load ptr, ptr %.sroa.6.0.detector.sroa_idx, align 8, !dbg !9150
  %.sroa.7.0.detector.sroa_idx = getelementptr inbounds nuw i8, ptr %detector, i64 32, !dbg !9150
  %.sroa.7.0.copyload = load i64, ptr %.sroa.7.0.detector.sroa_idx, align 8, !dbg !9150
  %1 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1540, !dbg !9151
  %_10.i45 = load i32, ptr %1, align 4, !dbg !9151, !alias.scope !9155, !noalias !9158, !noundef !11
  %ring_length.i46 = zext i32 %_10.i45 to i64, !dbg !9151
  %2 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !9161
  %3 = bitcast <8 x float> %2 to <8 x i32>, !dbg !9169
  %4 = xor <8 x i32> %3, splat (i32 -1), !dbg !9175
  %5 = bitcast <8 x i32> %4 to <8 x float>, !dbg !9169
  switch i32 %link, label %bb13.i296 [
    i32 1, label %bb14.i297
    i32 3, label %bb14.i297.fold.split
  ], !dbg !9177

bb13.i296:                                        ; preds = %bb5
  br label %bb14.i297, !dbg !9178

bb14.i297.fold.split:                             ; preds = %bb5
  br label %bb14.i297, !dbg !9179

bb14.i297:                                        ; preds = %bb5, %bb14.i297.fold.split, %bb13.i296
  %_11.i289.sroa.0.02184 = phi <8 x float> [ %5, %bb5 ], [ %2, %bb13.i296 ], [ %2, %bb14.i297.fold.split ]
  %_13.i288.sroa.0.0 = phi <8 x i32> [ %4, %bb5 ], [ %4, %bb13.i296 ], [ %3, %bb14.i297.fold.split ], !dbg !9180
  %_13.i47 = getelementptr inbounds nuw i8, ptr %channel_left, i64 256, !dbg !9181
  %_4.i348 = getelementptr inbounds nuw i8, ptr %channel_left, i64 448, !dbg !9183
  %_7.i349 = getelementptr inbounds nuw i8, ptr %channel_left, i64 480, !dbg !9185
  %_14.i350 = getelementptr inbounds nuw i8, ptr %channel_left, i64 288, !dbg !9186
  %_17.i351 = getelementptr inbounds nuw i8, ptr %channel_left, i64 320, !dbg !9187
  %_20.i352 = getelementptr inbounds nuw i8, ptr %channel_left, i64 352, !dbg !9188
  %_23.i353 = getelementptr inbounds nuw i8, ptr %channel_left, i64 384, !dbg !9189
  %_26.i354 = getelementptr inbounds nuw i8, ptr %channel_left, i64 416, !dbg !9190
  %6 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1440, !dbg !9191
  %_57.i50 = load i32, ptr %6, align 32, !dbg !9191, !alias.scope !9155, !noalias !9158, !noundef !11
  %_56.i51 = zext i32 %_57.i50 to i64, !dbg !9191
  %7 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1444, !dbg !9196
  %_60.i157 = load i32, ptr %7, align 4, !dbg !9196, !alias.scope !9155, !noalias !9158, !noundef !11
  %_58.not.i158 = icmp ne i32 %_60.i157, %_57.i50, !dbg !9196
  %8 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1448, !dbg !9196
  %_60.i157.1 = load i32, ptr %8, align 4, !dbg !9196
  %_58.not.i158.1 = icmp ne i32 %_60.i157.1, %_57.i50, !dbg !9196
  %or.cond.not3393 = select i1 %_58.not.i158, i1 true, i1 %_58.not.i158.1, !dbg !9196
  %9 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1452, !dbg !9196
  %_60.i157.2 = load i32, ptr %9, align 4, !dbg !9196
  %_58.not.i158.2 = icmp ne i32 %_60.i157.2, %_57.i50, !dbg !9196
  %or.cond3377.not3392 = select i1 %or.cond.not3393, i1 true, i1 %_58.not.i158.2, !dbg !9196
  %10 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1456, !dbg !9196
  %_60.i157.3 = load i32, ptr %10, align 4, !dbg !9196
  %_58.not.i158.3 = icmp ne i32 %_60.i157.3, %_57.i50, !dbg !9196
  %or.cond3378.not3391 = select i1 %or.cond3377.not3392, i1 true, i1 %_58.not.i158.3, !dbg !9196
  %11 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1460, !dbg !9196
  %_60.i157.4 = load i32, ptr %11, align 4, !dbg !9196
  %_58.not.i158.4 = icmp ne i32 %_60.i157.4, %_57.i50, !dbg !9196
  %or.cond3379.not3390 = select i1 %or.cond3378.not3391, i1 true, i1 %_58.not.i158.4, !dbg !9196
  %12 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1464, !dbg !9196
  %_60.i157.5 = load i32, ptr %12, align 4, !dbg !9196
  %_58.not.i158.5 = icmp ne i32 %_60.i157.5, %_57.i50, !dbg !9196
  %or.cond3380.not3389 = select i1 %or.cond3379.not3390, i1 true, i1 %_58.not.i158.5, !dbg !9196
  %13 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1468, !dbg !9196
  %_60.i157.6 = load i32, ptr %13, align 4, !dbg !9196
  %_58.not.i158.6 = icmp ne i32 %_60.i157.6, %_57.i50, !dbg !9196
  %or.cond3381.not = select i1 %or.cond3380.not3389, i1 true, i1 %_58.not.i158.6, !dbg !9196
  %14 = icmp ne ptr %.sroa.6.0.copyload, null
  %15 = icmp ne ptr %.sroa.4.0.copyload, null
  %16 = icmp slt <8 x i32> %_13.i288.sroa.0.0, zeroinitializer
  %17 = bitcast <8 x float> %_11.i289.sroa.0.02184 to <8 x i32>
  %18 = icmp slt <8 x i32> %17, zeroinitializer
  %19 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1536
  %20 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1504
  %21 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1512
  %22 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1528
  %23 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1520
  %_49.i120 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1472
  %24 = select i1 %bypass, <8 x i32> %3, <8 x i32> %4
  %25 = lshr i64 %left.1, 3, !dbg !9200
  %26 = add nuw nsw i64 %left.1, 8, !dbg !9200
  %27 = lshr i64 %26, 3, !dbg !9200
  %28 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1448
  %29 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1452
  %30 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1456
  %31 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1460
  %32 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1464
  %33 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1468
  br label %bb25.i60, !dbg !9200

bb25.i60:                                         ; preds = %bb14.i297, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit256
  %start1.sroa.0.0.i582512 = phi i64 [ 0, %bb14.i297 ], [ %34, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit256 ]
; call <compressor::kernel::Channel<wide::f32x8_::f32x8>>::advance_ramps
  tail call fastcc void @_RNvMNtCse3bfmKSZS8Y_10compressor6kernelINtB2_7ChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13advance_rampsB4_(ptr noalias noundef nonnull align 32 dereferenceable(1568) %channel_left, i32 noundef %sample_rate) #22, !dbg !9209, !noalias !9211
  %lanes.i918.sroa.0.0.copyload = load <8 x float>, ptr %_4.i348, align 32, !dbg !9212, !alias.scope !9218, !noalias !9222
  %lanes.i911.sroa.0.0.copyload = load <8 x float>, ptr %_7.i349, align 32, !dbg !9228, !alias.scope !9233, !noalias !9237
  %lanes.i904.sroa.0.0.copyload = load <8 x float>, ptr %_13.i47, align 32, !dbg !9241, !alias.scope !9246, !noalias !9250
  %lanes.i897.sroa.0.0.copyload = load <8 x float>, ptr %_14.i350, align 32, !dbg !9254, !alias.scope !9259, !noalias !9263
  %lanes.i890.sroa.0.0.copyload = load <8 x float>, ptr %_17.i351, align 32, !dbg !9267, !alias.scope !9272, !noalias !9276
  %lanes.i883.sroa.0.0.copyload = load <8 x float>, ptr %_20.i352, align 32, !dbg !9280, !alias.scope !9285, !noalias !9289
  %lanes.i876.sroa.0.0.copyload = load <8 x float>, ptr %_23.i353, align 32, !dbg !9293, !alias.scope !9298, !noalias !9302
  %lanes.i869.sroa.0.0.copyload = load <8 x float>, ptr %_26.i354, align 32, !dbg !9306, !alias.scope !9311, !noalias !9315
  %34 = add nuw nsw i64 %start1.sroa.0.0.i582512, 1, !dbg !9319
  %35 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i911.sroa.0.0.copyload, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !9327
  %36 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i911.sroa.0.0.copyload, <8 x float> zeroinitializer, i8 0), !dbg !9333
  %37 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i918.sroa.0.0.copyload, <8 x float> zeroinitializer, i8 0), !dbg !9339
  %slot.i61 = shl nuw nsw i64 %start1.sroa.0.0.i582512, 3, !dbg !9345
  %exitcond = icmp eq i64 %start1.sroa.0.0.i582512, %27, !dbg !9346
  br i1 %exitcond, label %bb27.i155, label %bb28.i63, !dbg !9346, !prof !161

bb28.i63:                                         ; preds = %bb25.i60
  %_76.i65 = getelementptr inbounds nuw float, ptr %left.0, i64 %slot.i61, !dbg !9352
  %exitcond2830.not = icmp eq i64 %start1.sroa.0.0.i582512, %25, !dbg !9357
  br i1 %exitcond2830.not, label %bb2.i971, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit974, !dbg !9357, !prof !161

bb2.i971:                                         ; preds = %bb28.i63
  %38 = and i64 %left.1, 7, !dbg !9200
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %38, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !9362, !noalias !9363
  unreachable, !dbg !9362

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit974: ; preds = %bb28.i63
  %lanes.i967.sroa.0.0.copyload = load <8 x float>, ptr %_76.i65, align 4, !dbg !9367, !alias.scope !9371, !noalias !9375
  switch i64 %.sroa.0.0.copyload, label %default.unreachable.i.i154 [
    i64 0, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i80
    i64 1, label %bb3.i.i153
    i64 2, label %bb2.i.i67
  ], !dbg !9377

default.unreachable.i.i154:                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit974
  unreachable

bb3.i.i153:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit974
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i80, !dbg !9380

bb2.i.i67:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit974
  tail call void @llvm.assume(i1 %14)
  %_25.i.i71 = icmp ugt i64 %slot.i61, %.sroa.5.0.copyload, !dbg !9381
  br i1 %_25.i.i71, label %bb17.i.i152, label %bb18.i.i72, !dbg !9381, !prof !161

bb18.i.i72:                                       ; preds = %bb2.i.i67
  tail call void @llvm.assume(i1 %15)
  %_28.i.i74 = sub nuw i64 %.sroa.5.0.copyload, %slot.i61, !dbg !9384
  %_8.i961 = icmp samesign ugt i64 %_28.i.i74, 7, !dbg !9385
  br i1 %_8.i961, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit965, label %bb2.i962, !dbg !9385, !prof !2135

bb2.i962:                                         ; preds = %bb18.i.i72
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_28.i.i74, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !9390, !noalias !9391
  unreachable, !dbg !9390

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit965: ; preds = %bb18.i.i72
  %_32.i.i75 = getelementptr inbounds nuw float, ptr %.sroa.4.0.copyload, i64 %slot.i61, !dbg !9400
  %lanes.i958.sroa.0.0.copyload = load <8 x float>, ptr %_32.i.i75, align 4, !dbg !9402, !alias.scope !9406, !noalias !9410
  %_33.i.i76 = icmp ugt i64 %slot.i61, %.sroa.7.0.copyload, !dbg !9412
  br i1 %_33.i.i76, label %bb19.i.i151, label %bb20.i.i77, !dbg !9412, !prof !161

bb17.i.i152:                                      ; preds = %bb2.i.i67
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 34359738361) %slot.i61, i64 noundef %.sroa.5.0.copyload, i64 noundef %.sroa.5.0.copyload, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7fe5a91ec575deda110c046c84630100) #23, !dbg !9415, !noalias !9416
  unreachable, !dbg !9415

bb20.i.i77:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit965
  %_36.i.i78 = sub nuw i64 %.sroa.7.0.copyload, %slot.i61, !dbg !9418
  %_8.i952 = icmp samesign ugt i64 %_36.i.i78, 7, !dbg !9419
  br i1 %_8.i952, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit956, label %bb2.i953, !dbg !9419, !prof !2135

bb2.i953:                                         ; preds = %bb20.i.i77
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_36.i.i78, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !9424, !noalias !9425
  unreachable, !dbg !9424

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit956: ; preds = %bb20.i.i77
  %_40.i.i79 = getelementptr inbounds nuw float, ptr %.sroa.6.0.copyload, i64 %slot.i61, !dbg !9429
  %lanes.i949.sroa.0.0.copyload = load <8 x float>, ptr %_40.i.i79, align 4, !dbg !9431, !alias.scope !9435, !noalias !9439
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i80, !dbg !9441

bb19.i.i151:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit965
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 34359738361) %slot.i61, i64 noundef %.sroa.7.0.copyload, i64 noundef %.sroa.7.0.copyload, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f2cba1f89421a726339097d2eba48536) #23, !dbg !9442, !noalias !9416
  unreachable, !dbg !9442

_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i80: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit956, %bb3.i.i153, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit974
  %.sroa.01522.0 = phi <8 x float> [ %lanes.i967.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit974 ], [ zeroinitializer, %bb3.i.i153 ], [ %lanes.i949.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit956 ], !dbg !9443
  %.sroa.01518.0 = phi <8 x float> [ %lanes.i967.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit974 ], [ zeroinitializer, %bb3.i.i153 ], [ %lanes.i958.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit956 ], !dbg !9443
  %39 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %.sroa.01518.0), !dbg !9444
  %40 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %.sroa.01522.0), !dbg !9450
  %41 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %39, <8 x float> %40), !dbg !9456
  %42 = fmul <8 x float> %39, splat (float 5.000000e-01), !dbg !9461
  %43 = fmul <8 x float> %40, splat (float 5.000000e-01), !dbg !9466
  %44 = fadd <8 x float> %42, %43, !dbg !9471
  %45 = select <8 x i1> %16, <8 x float> %44, <8 x float> %41, !dbg !9476
  %46 = select <8 x i1> %18, <8 x float> %45, <8 x float> %39, !dbg !9481
  %_29.i81 = load i32, ptr %19, align 32, !dbg !9486, !alias.scope !9155, !noalias !9158, !noundef !11
  %write.i82 = zext i32 %_29.i81 to i64, !dbg !9486
  %47 = add nuw nsw i64 %write.i82, 1, !dbg !9488
  %_31.i83 = icmp eq i64 %47, %ring_length.i46, !dbg !9490
  %spec.store.select.i84 = select i1 %_31.i83, i64 0, i64 %47, !dbg !9490
  %_107.1.i85 = load i64, ptr %21, align 8, !dbg !9492, !alias.scope !9155, !noalias !9158, !noundef !11
  %_35.i86 = shl nuw nsw i64 %write.i82, 3, !dbg !9494
  %_77.i87 = icmp ugt i64 %_35.i86, %_107.1.i85, !dbg !9495
  br i1 %_77.i87, label %bb29.i150, label %bb30.i88, !dbg !9495, !prof !161

bb27.i155:                                        ; preds = %bb25.i60
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %slot.i61, i64 noundef range(i64 8, 34359738361) %left.1, i64 noundef range(i64 8, 34359738361) %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8632ba88614e3313b76c3c784bc00e8d) #23, !dbg !9500, !noalias !9211
  unreachable, !dbg !9500

bb30.i88:                                         ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i80
  %_80.i90 = sub nuw i64 %_107.1.i85, %_35.i86, !dbg !9501
  %_8.i1314 = icmp samesign ugt i64 %_80.i90, 7, !dbg !9502
  br i1 %_8.i1314, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1317, label %bb2.i1315, !dbg !9502, !prof !2135

bb2.i1315:                                        ; preds = %bb30.i88
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_80.i90, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !9507, !noalias !9508
  unreachable, !dbg !9507

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1317: ; preds = %bb30.i88
  %_107.0.i89 = load ptr, ptr %20, align 32, !dbg !9492, !alias.scope !9155, !noalias !9158, !nonnull !11, !noundef !11
  %_84.i91 = getelementptr inbounds nuw float, ptr %_107.0.i89, i64 %_35.i86, !dbg !9512
  store <8 x float> %lanes.i967.sroa.0.0.copyload, ptr %_84.i91, align 4, !dbg !9517, !alias.scope !9521, !noalias !9525
  %_108.1.i92 = load i64, ptr %22, align 8, !dbg !9527, !alias.scope !9155, !noalias !9158, !noundef !11
  %_85.i93 = icmp ugt i64 %_35.i86, %_108.1.i92, !dbg !9528
  br i1 %_85.i93, label %bb31.i149, label %bb32.i94, !dbg !9528, !prof !161

bb29.i150:                                        ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i80
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_35.i86, i64 noundef %_107.1.i85, i64 noundef %_107.1.i85, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d362267d85009f8e23c4d340b28a745d) #23, !dbg !9532, !noalias !9211
  unreachable, !dbg !9532

bb32.i94:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1317
  %_88.i96 = sub nuw i64 %_108.1.i92, %_35.i86, !dbg !9533
  %_8.i1309 = icmp samesign ugt i64 %_88.i96, 7, !dbg !9534
  br i1 %_8.i1309, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1312, label %bb2.i1310, !dbg !9534, !prof !2135

bb2.i1310:                                        ; preds = %bb32.i94
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_88.i96, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !9539, !noalias !9540
  unreachable, !dbg !9539

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1312: ; preds = %bb32.i94
  %_108.0.i95 = load ptr, ptr %23, align 16, !dbg !9527, !alias.scope !9155, !noalias !9158, !nonnull !11, !noundef !11
  %_92.i97 = getelementptr inbounds nuw float, ptr %_108.0.i95, i64 %_35.i86, !dbg !9544
  store <8 x float> %46, ptr %_92.i97, align 4, !dbg !9549, !alias.scope !9553, !noalias !9557
  %_109.1.i98 = load i64, ptr %21, align 8, !dbg !9559, !alias.scope !9155, !noalias !9158, !noundef !11
  %_41.i99 = shl nuw nsw i64 %spec.store.select.i84, 3, !dbg !9560
  %_93.i100 = icmp ugt i64 %_41.i99, %_109.1.i98, !dbg !9561
  br i1 %_93.i100, label %bb33.i148, label %bb34.i101, !dbg !9561, !prof !161

bb31.i149:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1317
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_35.i86, i64 noundef %_108.1.i92, i64 noundef %_108.1.i92, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1ce527b0bad7f9d2c89100bcea389654) #23, !dbg !9565, !noalias !9211
  unreachable, !dbg !9565

bb34.i101:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1312
  %_96.i103 = sub nuw i64 %_109.1.i98, %_41.i99, !dbg !9566
  %_8.i943 = icmp samesign ugt i64 %_96.i103, 7, !dbg !9567
  br i1 %_8.i943, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit947, label %bb2.i944, !dbg !9567, !prof !2135

bb2.i944:                                         ; preds = %bb34.i101
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_96.i103, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !9572, !noalias !9573
  unreachable, !dbg !9572

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit947: ; preds = %bb34.i101
  %_109.0.i102 = load ptr, ptr %20, align 32, !dbg !9559, !alias.scope !9155, !noalias !9158, !nonnull !11, !noundef !11
  %_100.i104 = getelementptr inbounds nuw float, ptr %_109.0.i102, i64 %_41.i99, !dbg !9577
  %lanes.i940.sroa.0.0.copyload = load <8 x float>, ptr %_100.i104, align 4, !dbg !9582, !alias.scope !9586, !noalias !9590
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9592), !dbg !9595
  %_6.i8.i105 = load i32, ptr %1, align 4, !dbg !9597, !alias.scope !9599, !noalias !9600, !noundef !11
  %ring_length.i.i106 = zext i32 %_6.i8.i105 to i64, !dbg !9597
  br i1 %or.cond3381.not, label %bb7.i.i127.preheader, label %bb1.i.i107, !dbg !9603

bb7.i.i127.preheader:                             ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit947
  %_56.1.i.i141 = load i64, ptr %22, align 8
  %_56.0.i.i145 = load ptr, ptr %23, align 16, !nonnull !11
  %_25.i14.i133 = load i32, ptr %6, align 4, !dbg !9604, !alias.scope !9599, !noalias !9600, !noundef !11
  %delay1.i.i134 = zext i32 %_25.i14.i133 to i64, !dbg !9604
  %_28.not.i.i135 = icmp ult i32 %_29.i81, %_25.i14.i133, !dbg !9605
  %_29.i.i136 = select i1 %_28.not.i.i135, i64 %ring_length.i.i106, i64 0, !dbg !9605
  %write.pn7.i.i137 = sub nsw i64 %write.i82, %delay1.i.i134, !dbg !9605
  %tap.sroa.0.0.i.i138 = add nsw i64 %write.pn7.i.i137, %_29.i.i136, !dbg !9606
  %_32.i15.i139 = shl nsw i64 %tap.sroa.0.0.i.i138, 3, !dbg !9607
  %_34.i.i142 = icmp ult i64 %_32.i15.i139, %_56.1.i.i141, !dbg !9608
  br i1 %_34.i.i142, label %bb16.i.i144, label %panic2.i.i143, !dbg !9608

bb1.i.i107:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit947
  %_10.not.i.i108 = icmp ult i32 %_29.i81, %_57.i50, !dbg !9609
  %_11.i9.i109 = select i1 %_10.not.i.i108, i64 %ring_length.i.i106, i64 0, !dbg !9609
  %write.pn.i.i110 = sub nsw i64 %write.i82, %_56.i51, !dbg !9609
  %row.sroa.0.0.i.i111 = add nsw i64 %write.pn.i.i110, %_11.i9.i109, !dbg !9610
  %_55.1.i.i112 = load i64, ptr %22, align 8, !dbg !9611, !alias.scope !9599, !noalias !9600, !noundef !11
  %_13.i.i113 = shl nsw i64 %row.sroa.0.0.i.i111, 3, !dbg !9612
  %_40.i10.i114 = icmp ugt i64 %_13.i.i113, %_55.1.i.i112, !dbg !9613
  br i1 %_40.i10.i114, label %bb19.i13.i121, label %bb20.i11.i115, !dbg !9613, !prof !161

bb20.i11.i115:                                    ; preds = %bb1.i.i107
  %_43.i.i117 = sub nuw i64 %_55.1.i.i112, %_13.i.i113, !dbg !9616
  %_8.i935 = icmp samesign ugt i64 %_43.i.i117, 7, !dbg !9617
  br i1 %_8.i935, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit938, label %bb2.i, !dbg !9617, !prof !2135

bb2.i:                                            ; preds = %bb20.i11.i115
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_43.i.i117, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !9622, !noalias !9623
  unreachable, !dbg !9622

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit938: ; preds = %bb20.i11.i115
  %_55.0.i.i116 = load ptr, ptr %23, align 16, !dbg !9611, !alias.scope !9599, !noalias !9600, !nonnull !11, !noundef !11
  %_47.i.i118 = getelementptr inbounds nuw float, ptr %_55.0.i.i116, i64 %_13.i.i113, !dbg !9627
  %lanes.i932.sroa.0.0.copyload = load <8 x float>, ptr %_47.i.i118, align 4, !dbg !9629, !alias.scope !9633, !noalias !9637
  br label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit256, !dbg !9639

bb19.i13.i121:                                    ; preds = %bb1.i.i107
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_13.i.i113, i64 noundef %_55.1.i.i112, i64 noundef %_55.1.i.i112, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_942b76445104c042a726096e85fe3fcb) #23, !dbg !9640, !noalias !9641
  unreachable, !dbg !9640

bb11.i.i147:                                      ; preds = %bb12.i.i132.7
  %48 = getelementptr inbounds nuw float, ptr %_56.0.i.i145, i64 %_31.i.i140.6, !dbg !9608
  %_30.i.i146.6 = load float, ptr %48, align 4, !dbg !9608, !noalias !9642, !noundef !11
  %49 = getelementptr inbounds nuw float, ptr %_56.0.i.i145, i64 %_31.i.i140.7, !dbg !9608
  %_30.i.i146.7 = load float, ptr %49, align 4, !dbg !9608, !noalias !9642, !noundef !11
  %lanes.i925.sroa.0.0.vec.insert = insertelement <8 x float> poison, float %_30.i.i146, i64 0, !dbg !9643
  %lanes.i925.sroa.0.4.vec.insert = insertelement <8 x float> %lanes.i925.sroa.0.0.vec.insert, float %_30.i.i146.1, i64 1, !dbg !9643
  %lanes.i925.sroa.0.8.vec.insert = insertelement <8 x float> %lanes.i925.sroa.0.4.vec.insert, float %_30.i.i146.2, i64 2, !dbg !9643
  %lanes.i925.sroa.0.12.vec.insert = insertelement <8 x float> %lanes.i925.sroa.0.8.vec.insert, float %_30.i.i146.3, i64 3, !dbg !9643
  %lanes.i925.sroa.0.16.vec.insert = insertelement <8 x float> %lanes.i925.sroa.0.12.vec.insert, float %_30.i.i146.4, i64 4, !dbg !9643
  %lanes.i925.sroa.0.20.vec.insert = insertelement <8 x float> %lanes.i925.sroa.0.16.vec.insert, float %_30.i.i146.5, i64 5, !dbg !9643
  %lanes.i925.sroa.0.24.vec.insert = insertelement <8 x float> %lanes.i925.sroa.0.20.vec.insert, float %_30.i.i146.6, i64 6, !dbg !9643
  %lanes.i925.sroa.0.28.vec.insert = insertelement <8 x float> %lanes.i925.sroa.0.24.vec.insert, float %_30.i.i146.7, i64 7, !dbg !9643
  br label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit256, !dbg !9639

bb16.i.i144:                                      ; preds = %bb7.i.i127.preheader
  %50 = getelementptr inbounds nuw float, ptr %_56.0.i.i145, i64 %_32.i15.i139, !dbg !9608
  %_30.i.i146 = load float, ptr %50, align 4, !dbg !9608, !noalias !9642, !noundef !11
  %_25.i14.i133.1 = load i32, ptr %7, align 4, !dbg !9604, !alias.scope !9599, !noalias !9600, !noundef !11
  %delay1.i.i134.1 = zext i32 %_25.i14.i133.1 to i64, !dbg !9604
  %_28.not.i.i135.1 = icmp ult i32 %_29.i81, %_25.i14.i133.1, !dbg !9605
  %_29.i.i136.1 = select i1 %_28.not.i.i135.1, i64 %ring_length.i.i106, i64 0, !dbg !9605
  %write.pn7.i.i137.1 = sub nsw i64 %write.i82, %delay1.i.i134.1, !dbg !9605
  %tap.sroa.0.0.i.i138.1 = add nsw i64 %write.pn7.i.i137.1, %_29.i.i136.1, !dbg !9606
  %_32.i15.i139.1 = shl nsw i64 %tap.sroa.0.0.i.i138.1, 3, !dbg !9607
  %_31.i.i140.1 = or disjoint i64 %_32.i15.i139.1, 1, !dbg !9607
  %_34.i.i142.1 = icmp ult i64 %_31.i.i140.1, %_56.1.i.i141, !dbg !9608
  br i1 %_34.i.i142.1, label %bb16.i.i144.1, label %panic2.i.i143, !dbg !9608

bb16.i.i144.1:                                    ; preds = %bb16.i.i144
  %51 = getelementptr inbounds nuw float, ptr %_56.0.i.i145, i64 %_31.i.i140.1, !dbg !9608
  %_30.i.i146.1 = load float, ptr %51, align 4, !dbg !9608, !noalias !9642, !noundef !11
  %_25.i14.i133.2 = load i32, ptr %28, align 4, !dbg !9604, !alias.scope !9599, !noalias !9600, !noundef !11
  %delay1.i.i134.2 = zext i32 %_25.i14.i133.2 to i64, !dbg !9604
  %_28.not.i.i135.2 = icmp ult i32 %_29.i81, %_25.i14.i133.2, !dbg !9605
  %_29.i.i136.2 = select i1 %_28.not.i.i135.2, i64 %ring_length.i.i106, i64 0, !dbg !9605
  %write.pn7.i.i137.2 = sub nsw i64 %write.i82, %delay1.i.i134.2, !dbg !9605
  %tap.sroa.0.0.i.i138.2 = add nsw i64 %write.pn7.i.i137.2, %_29.i.i136.2, !dbg !9606
  %_32.i15.i139.2 = shl nsw i64 %tap.sroa.0.0.i.i138.2, 3, !dbg !9607
  %_31.i.i140.2 = or disjoint i64 %_32.i15.i139.2, 2, !dbg !9607
  %_34.i.i142.2 = icmp ult i64 %_31.i.i140.2, %_56.1.i.i141, !dbg !9608
  br i1 %_34.i.i142.2, label %bb16.i.i144.2, label %panic2.i.i143, !dbg !9608

bb16.i.i144.2:                                    ; preds = %bb16.i.i144.1
  %52 = getelementptr inbounds nuw float, ptr %_56.0.i.i145, i64 %_31.i.i140.2, !dbg !9608
  %_30.i.i146.2 = load float, ptr %52, align 4, !dbg !9608, !noalias !9642, !noundef !11
  %_25.i14.i133.3 = load i32, ptr %29, align 4, !dbg !9604, !alias.scope !9599, !noalias !9600, !noundef !11
  %delay1.i.i134.3 = zext i32 %_25.i14.i133.3 to i64, !dbg !9604
  %_28.not.i.i135.3 = icmp ult i32 %_29.i81, %_25.i14.i133.3, !dbg !9605
  %_29.i.i136.3 = select i1 %_28.not.i.i135.3, i64 %ring_length.i.i106, i64 0, !dbg !9605
  %write.pn7.i.i137.3 = sub nsw i64 %write.i82, %delay1.i.i134.3, !dbg !9605
  %tap.sroa.0.0.i.i138.3 = add nsw i64 %write.pn7.i.i137.3, %_29.i.i136.3, !dbg !9606
  %_32.i15.i139.3 = shl nsw i64 %tap.sroa.0.0.i.i138.3, 3, !dbg !9607
  %_31.i.i140.3 = or disjoint i64 %_32.i15.i139.3, 3, !dbg !9607
  %_34.i.i142.3 = icmp ult i64 %_31.i.i140.3, %_56.1.i.i141, !dbg !9608
  br i1 %_34.i.i142.3, label %bb16.i.i144.3, label %panic2.i.i143, !dbg !9608

bb16.i.i144.3:                                    ; preds = %bb16.i.i144.2
  %53 = getelementptr inbounds nuw float, ptr %_56.0.i.i145, i64 %_31.i.i140.3, !dbg !9608
  %_30.i.i146.3 = load float, ptr %53, align 4, !dbg !9608, !noalias !9642, !noundef !11
  %_25.i14.i133.4 = load i32, ptr %30, align 4, !dbg !9604, !alias.scope !9599, !noalias !9600, !noundef !11
  %delay1.i.i134.4 = zext i32 %_25.i14.i133.4 to i64, !dbg !9604
  %_28.not.i.i135.4 = icmp ult i32 %_29.i81, %_25.i14.i133.4, !dbg !9605
  %_29.i.i136.4 = select i1 %_28.not.i.i135.4, i64 %ring_length.i.i106, i64 0, !dbg !9605
  %write.pn7.i.i137.4 = sub nsw i64 %write.i82, %delay1.i.i134.4, !dbg !9605
  %tap.sroa.0.0.i.i138.4 = add nsw i64 %write.pn7.i.i137.4, %_29.i.i136.4, !dbg !9606
  %_32.i15.i139.4 = shl nsw i64 %tap.sroa.0.0.i.i138.4, 3, !dbg !9607
  %_31.i.i140.4 = or disjoint i64 %_32.i15.i139.4, 4, !dbg !9607
  %_34.i.i142.4 = icmp ult i64 %_31.i.i140.4, %_56.1.i.i141, !dbg !9608
  br i1 %_34.i.i142.4, label %bb16.i.i144.4, label %panic2.i.i143, !dbg !9608

bb16.i.i144.4:                                    ; preds = %bb16.i.i144.3
  %54 = getelementptr inbounds nuw float, ptr %_56.0.i.i145, i64 %_31.i.i140.4, !dbg !9608
  %_30.i.i146.4 = load float, ptr %54, align 4, !dbg !9608, !noalias !9642, !noundef !11
  %_25.i14.i133.5 = load i32, ptr %31, align 4, !dbg !9604, !alias.scope !9599, !noalias !9600, !noundef !11
  %delay1.i.i134.5 = zext i32 %_25.i14.i133.5 to i64, !dbg !9604
  %_28.not.i.i135.5 = icmp ult i32 %_29.i81, %_25.i14.i133.5, !dbg !9605
  %_29.i.i136.5 = select i1 %_28.not.i.i135.5, i64 %ring_length.i.i106, i64 0, !dbg !9605
  %write.pn7.i.i137.5 = sub nsw i64 %write.i82, %delay1.i.i134.5, !dbg !9605
  %tap.sroa.0.0.i.i138.5 = add nsw i64 %write.pn7.i.i137.5, %_29.i.i136.5, !dbg !9606
  %_32.i15.i139.5 = shl nsw i64 %tap.sroa.0.0.i.i138.5, 3, !dbg !9607
  %_31.i.i140.5 = or disjoint i64 %_32.i15.i139.5, 5, !dbg !9607
  %_34.i.i142.5 = icmp ult i64 %_31.i.i140.5, %_56.1.i.i141, !dbg !9608
  br i1 %_34.i.i142.5, label %bb16.i.i144.5, label %panic2.i.i143, !dbg !9608

bb16.i.i144.5:                                    ; preds = %bb16.i.i144.4
  %55 = getelementptr inbounds nuw float, ptr %_56.0.i.i145, i64 %_31.i.i140.5, !dbg !9608
  %_30.i.i146.5 = load float, ptr %55, align 4, !dbg !9608, !noalias !9642, !noundef !11
  %_25.i14.i133.6 = load i32, ptr %32, align 4, !dbg !9604, !alias.scope !9599, !noalias !9600, !noundef !11
  %delay1.i.i134.6 = zext i32 %_25.i14.i133.6 to i64, !dbg !9604
  %_28.not.i.i135.6 = icmp ult i32 %_29.i81, %_25.i14.i133.6, !dbg !9605
  %_29.i.i136.6 = select i1 %_28.not.i.i135.6, i64 %ring_length.i.i106, i64 0, !dbg !9605
  %write.pn7.i.i137.6 = sub nsw i64 %write.i82, %delay1.i.i134.6, !dbg !9605
  %tap.sroa.0.0.i.i138.6 = add nsw i64 %write.pn7.i.i137.6, %_29.i.i136.6, !dbg !9606
  %_32.i15.i139.6 = shl nsw i64 %tap.sroa.0.0.i.i138.6, 3, !dbg !9607
  %_31.i.i140.6 = or disjoint i64 %_32.i15.i139.6, 6, !dbg !9607
  %_34.i.i142.6 = icmp ult i64 %_31.i.i140.6, %_56.1.i.i141, !dbg !9608
  br i1 %_34.i.i142.6, label %bb12.i.i132.7, label %panic2.i.i143, !dbg !9608

bb12.i.i132.7:                                    ; preds = %bb16.i.i144.5
  %_25.i14.i133.7 = load i32, ptr %33, align 4, !dbg !9604, !alias.scope !9599, !noalias !9600, !noundef !11
  %delay1.i.i134.7 = zext i32 %_25.i14.i133.7 to i64, !dbg !9604
  %_28.not.i.i135.7 = icmp ult i32 %_29.i81, %_25.i14.i133.7, !dbg !9605
  %_29.i.i136.7 = select i1 %_28.not.i.i135.7, i64 %ring_length.i.i106, i64 0, !dbg !9605
  %write.pn7.i.i137.7 = sub nsw i64 %write.i82, %delay1.i.i134.7, !dbg !9605
  %tap.sroa.0.0.i.i138.7 = add nsw i64 %write.pn7.i.i137.7, %_29.i.i136.7, !dbg !9606
  %_32.i15.i139.7 = shl nsw i64 %tap.sroa.0.0.i.i138.7, 3, !dbg !9607
  %_31.i.i140.7 = or disjoint i64 %_32.i15.i139.7, 7, !dbg !9607
  %_34.i.i142.7 = icmp ult i64 %_31.i.i140.7, %_56.1.i.i141, !dbg !9608
  br i1 %_34.i.i142.7, label %bb11.i.i147, label %panic2.i.i143, !dbg !9608

panic2.i.i143:                                    ; preds = %bb12.i.i132.7, %bb16.i.i144.5, %bb16.i.i144.4, %bb16.i.i144.3, %bb16.i.i144.2, %bb16.i.i144.1, %bb16.i.i144, %bb7.i.i127.preheader
  %_31.i.i140.lcssa = phi i64 [ %_32.i15.i139, %bb7.i.i127.preheader ], [ %_31.i.i140.1, %bb16.i.i144 ], [ %_31.i.i140.2, %bb16.i.i144.1 ], [ %_31.i.i140.3, %bb16.i.i144.2 ], [ %_31.i.i140.4, %bb16.i.i144.3 ], [ %_31.i.i140.5, %bb16.i.i144.4 ], [ %_31.i.i140.6, %bb16.i.i144.5 ], [ %_31.i.i140.7, %bb12.i.i132.7 ], !dbg !9607
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_31.i.i140.lcssa, i64 noundef %_56.1.i.i141, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c4f76d0874ad32f224dcaa5d3453a9e1) #23, !dbg !9608, !noalias !9642
  unreachable, !dbg !9608

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit256: ; preds = %bb11.i.i147, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit938
  %detected_left.i37.sroa.0.0 = phi <8 x float> [ %lanes.i925.sroa.0.28.vec.insert, %bb11.i.i147 ], [ %lanes.i932.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit938 ], !dbg !9648
  %56 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %detected_left.i37.sroa.0.0, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !9651
  %57 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %56, <8 x float> splat (float 0x3810000000000000)), !dbg !9660
  %58 = bitcast <8 x float> %57 to <4 x i64>, !dbg !9667
  %59 = and <4 x i64> %58, splat (i64 36028792732385279), !dbg !9668
  %60 = or disjoint <4 x i64> %59, splat (i64 4575657222473777152), !dbg !9673
  %61 = bitcast <4 x i64> %60 to <8 x float>, !dbg !9677
  %62 = fadd <8 x float> %61, splat (float -1.000000e+00), !dbg !9678
  %63 = fmul <8 x float> %62, splat (float 0xBF9B17A960000000), !dbg !9683
  %64 = fadd <8 x float> %63, splat (float 0x3FBF9A8440000000), !dbg !9688
  %65 = fmul <8 x float> %62, %64, !dbg !9683
  %66 = fadd <8 x float> %65, splat (float 0xBFD1E3F400000000), !dbg !9688
  %67 = fmul <8 x float> %62, %66, !dbg !9683
  %68 = fadd <8 x float> %67, splat (float 0x3FDD544F20000000), !dbg !9688
  %69 = fmul <8 x float> %62, %68, !dbg !9683
  %70 = fadd <8 x float> %69, splat (float 0xBFE6FC2A60000000), !dbg !9688
  %71 = fmul <8 x float> %62, %70, !dbg !9683
  %72 = fadd <8 x float> %71, splat (float 0x3FF714B2A0000000), !dbg !9688
  %73 = bitcast <8 x float> %57 to <8 x i32>, !dbg !9693
  %_3.i1436 = lshr <8 x i32> %73, splat (i32 23), !dbg !9697
  %74 = or disjoint <8 x i32> %_3.i1436, splat (i32 1258291200), !dbg !9698
  %75 = bitcast <8 x i32> %74 to <8 x float>, !dbg !9702
  %76 = fadd <8 x float> %75, splat (float 0xC160000FE0000000), !dbg !9703
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9707), !dbg !9710
  %_6.i186.sroa.0.0.copyload = load <8 x float>, ptr %_49.i120, align 32, !dbg !9712, !noalias !9714
  %77 = fmul <8 x float> %62, %72, !dbg !9717
  %78 = fadd <8 x float> %76, %77, !dbg !9722
  %79 = fmul <8 x float> %78, splat (float 0x4018151820000000), !dbg !9727
  %80 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %79, <8 x float> splat (float -1.600000e+02)), !dbg !9732
  %81 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %80, <8 x float> splat (float 2.400000e+01)), !dbg !9737
  %82 = fsub <8 x float> %81, %lanes.i904.sroa.0.0.copyload, !dbg !9742
  %83 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %82, <8 x float> %lanes.i890.sroa.0.0.copyload, i8 30), !dbg !9748
  %84 = fneg <8 x float> %lanes.i890.sroa.0.0.copyload, !dbg !9754
  %85 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %82, <8 x float> %84, i8 18), !dbg !9759
  %86 = fadd <8 x float> %lanes.i890.sroa.0.0.copyload, %82, !dbg !9765
  %87 = fmul <8 x float> %86, %86, !dbg !9770
  %88 = fmul <8 x float> %lanes.i883.sroa.0.0.copyload, %87, !dbg !9775
  %89 = bitcast <8 x float> %83 to <8 x i32>, !dbg !9780
  %90 = icmp slt <8 x i32> %89, zeroinitializer, !dbg !9784
  %.v = select <8 x i1> %90, <8 x float> %82, <8 x float> %88, !dbg !9784
  %91 = fmul <8 x float> %lanes.i897.sroa.0.0.copyload, %.v, !dbg !9784
  %92 = bitcast <8 x float> %85 to <8 x i32>, !dbg !9786
  %93 = icmp slt <8 x i32> %92, zeroinitializer, !dbg !9790
  %94 = select <8 x i1> %93, <8 x float> zeroinitializer, <8 x float> %91, !dbg !9790
  %95 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %94, <8 x float> splat (float -1.000000e+02)), !dbg !9792
  %96 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %95, <8 x float> zeroinitializer), !dbg !9797
  %97 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %96, <8 x float> %_6.i186.sroa.0.0.copyload, i8 17), !dbg !9802
  %98 = bitcast <8 x float> %97 to <8 x i32>, !dbg !9808
  %99 = icmp slt <8 x i32> %98, zeroinitializer, !dbg !9812
  %100 = select <8 x i1> %99, <8 x float> %lanes.i876.sroa.0.0.copyload, <8 x float> %lanes.i869.sroa.0.0.copyload, !dbg !9812
  %101 = fsub <8 x float> %96, %_6.i186.sroa.0.0.copyload, !dbg !9814
  %102 = fmul <8 x float> %101, %100, !dbg !9820
  %103 = fadd <8 x float> %_6.i186.sroa.0.0.copyload, %102, !dbg !9825
  %104 = bitcast <8 x float> %103 to <8 x i32>, !dbg !9829
  %105 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %103), !dbg !9835
  %106 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %105, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !9837
  %107 = bitcast <8 x float> %106 to <8 x i32>, !dbg !9843
  %108 = xor <8 x i32> %107, splat (i32 -1), !dbg !9849
  %109 = and <8 x i32> %108, %104, !dbg !9851
  store <8 x i32> %109, ptr %_49.i120, align 32, !dbg !9855, !alias.scope !9856, !noalias !9858
  %110 = bitcast <8 x i32> %109 to <8 x float>, !dbg !9859
  %111 = fadd <8 x float> %lanes.i918.sroa.0.0.copyload, %110, !dbg !9860
  %112 = fmul <8 x float> %111, splat (float 0x3FC542A5A0000000), !dbg !9867
  %113 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %112, <8 x float> splat (float -1.260000e+02)), !dbg !9873
  %114 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %113, <8 x float> splat (float 1.270000e+02)), !dbg !9879
  %115 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %114), !dbg !9884
  %116 = fsub <8 x float> %114, %115, !dbg !9889
  %117 = fmul <8 x float> %116, splat (float 0x3F5E974FA0000000), !dbg !9894
  %118 = fadd <8 x float> %117, splat (float 0x3F82778560000000), !dbg !9899
  %119 = fmul <8 x float> %116, %118, !dbg !9894
  %120 = fadd <8 x float> %119, splat (float 0x3FAC91CE60000000), !dbg !9899
  %121 = fmul <8 x float> %116, %120, !dbg !9894
  %122 = fadd <8 x float> %121, splat (float 0x3FCEBDB560000000), !dbg !9899
  %123 = fmul <8 x float> %116, %122, !dbg !9894
  %124 = fadd <8 x float> %123, splat (float 0x3FE62E4BA0000000), !dbg !9899
  %125 = fmul <8 x float> %116, %124, !dbg !9904
  %126 = fadd <8 x float> %125, splat (float 1.000000e+00), !dbg !9909
  %127 = fadd <8 x float> %115, splat (float 0x4160000FE0000000), !dbg !9914
  %128 = bitcast <8 x float> %127 to <8 x i32>, !dbg !9919
  %_3.i1437 = shl <8 x i32> %128, splat (i32 23), !dbg !9923
  %129 = bitcast <8 x i32> %_3.i1437 to <8 x float>, !dbg !9924
  %130 = fmul <8 x float> %126, %129, !dbg !9926
  %131 = fmul <8 x float> %lanes.i940.sroa.0.0.copyload, %130, !dbg !9930
  %132 = fsub <8 x float> %131, %lanes.i940.sroa.0.0.copyload, !dbg !9935
  %133 = fmul <8 x float> %lanes.i911.sroa.0.0.copyload, %132, !dbg !9941
  %134 = fadd <8 x float> %lanes.i940.sroa.0.0.copyload, %133, !dbg !9946
  %135 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %110, <8 x float> zeroinitializer, i8 0), !dbg !9950
  %136 = bitcast <8 x float> %135 to <8 x i32>, !dbg !9956
  %137 = bitcast <8 x float> %37 to <8 x i32>, !dbg !9956
  %138 = and <8 x i32> %136, %137, !dbg !9960
  %139 = bitcast <8 x float> %36 to <8 x i32>, !dbg !9962
  %140 = or <8 x i32> %24, %139, !dbg !9966
  %141 = or <8 x i32> %140, %138, !dbg !9968
  %142 = bitcast <8 x float> %35 to <8 x i32>, !dbg !9973
  %143 = icmp slt <8 x i32> %142, zeroinitializer, !dbg !9977
  %144 = select <8 x i1> %143, <8 x float> %131, <8 x float> %134, !dbg !9977
  %145 = icmp slt <8 x i32> %141, zeroinitializer, !dbg !9979
  %146 = select <8 x i1> %145, <8 x float> %lanes.i940.sroa.0.0.copyload, <8 x float> %144, !dbg !9979
  store <8 x float> %146, ptr %_76.i65, align 4, !dbg !9984, !alias.scope !9990, !noalias !9994
  %147 = trunc i64 %spec.store.select.i84 to i32, !dbg !9998
  store i32 %147, ptr %19, align 32, !dbg !9998, !alias.scope !9155, !noalias !9158
  %exitcond2831.not = icmp eq i64 %34, %spec.store.select, !dbg !9999
  br i1 %exitcond2831.not, label %bb8, label %bb25.i60, !dbg !9200

bb33.i148:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1312
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.i99, i64 noundef %_109.1.i98, i64 noundef %_109.1.i98, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_824b3214541c2b0b46a15e52dc0ae67e) #23, !dbg !10003, !noalias !9211
  unreachable, !dbg !10003

bb8:                                              ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit256, %start
  br i1 %_15, label %bb9, label %bb16, !dbg !10004

bb9:                                              ; preds = %bb8
  %_24 = sub nsw i64 %frames, %spec.store.select, !dbg !10005
  %_34 = icmp ult i64 %_24, 129, !dbg !10006
  br i1 %_34, label %bb1.i.preheader, label %bb12, !dbg !10006

bb1.i.preheader:                                  ; preds = %bb9
  %148 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1440
  %_5.i199 = load i32, ptr %148, align 4, !dbg !10009, !alias.scope !10011, !noundef !11
  %149 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1444, !dbg !10009
  %_5.i199.1 = load i32, ptr %149, align 4, !dbg !10009, !alias.scope !10011, !noundef !11
  %least.sroa.0.1.i.1 = tail call i32 @llvm.umin.i32(i32 %_5.i199.1, i32 %_5.i199), !dbg !10009
  %150 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1448, !dbg !10009
  %_5.i199.2 = load i32, ptr %150, align 4, !dbg !10009, !alias.scope !10011, !noundef !11
  %least.sroa.0.1.i.2 = tail call i32 @llvm.umin.i32(i32 %_5.i199.2, i32 %least.sroa.0.1.i.1), !dbg !10009
  %151 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1452, !dbg !10009
  %_5.i199.3 = load i32, ptr %151, align 4, !dbg !10009, !alias.scope !10011, !noundef !11
  %least.sroa.0.1.i.3 = tail call i32 @llvm.umin.i32(i32 %_5.i199.3, i32 %least.sroa.0.1.i.2), !dbg !10009
  %152 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1456, !dbg !10009
  %_5.i199.4 = load i32, ptr %152, align 4, !dbg !10009, !alias.scope !10011, !noundef !11
  %least.sroa.0.1.i.4 = tail call i32 @llvm.umin.i32(i32 %_5.i199.4, i32 %least.sroa.0.1.i.3), !dbg !10009
  %153 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1460, !dbg !10009
  %_5.i199.5 = load i32, ptr %153, align 4, !dbg !10009, !alias.scope !10011, !noundef !11
  %least.sroa.0.1.i.5 = tail call i32 @llvm.umin.i32(i32 %_5.i199.5, i32 %least.sroa.0.1.i.4), !dbg !10009
  %154 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1464, !dbg !10009
  %_5.i199.6 = load i32, ptr %154, align 4, !dbg !10009, !alias.scope !10011, !noundef !11
  %least.sroa.0.1.i.6 = tail call i32 @llvm.umin.i32(i32 %_5.i199.6, i32 %least.sroa.0.1.i.5), !dbg !10009
  %155 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1468, !dbg !10009
  %_5.i199.7 = load i32, ptr %155, align 4, !dbg !10009, !alias.scope !10011, !noundef !11
  %least.sroa.0.1.i.7 = tail call i32 @llvm.umin.i32(i32 %_5.i199.7, i32 %least.sroa.0.1.i.6), !dbg !10009
  %_0.i = zext i32 %least.sroa.0.1.i.7 to i64, !dbg !10014
  %_22.not = icmp samesign ugt i64 %_24, %_0.i, !dbg !10015
  br i1 %_22.not, label %bb12, label %bb10, !dbg !10008

bb16:                                             ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit, %bb8
  ret void, !dbg !10016

bb12:                                             ; preds = %bb9, %bb1.i.preheader
  %156 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1540, !dbg !10017
  %_10.i = load i32, ptr %156, align 4, !dbg !10017, !alias.scope !10021, !noalias !10024, !noundef !11
  %ring_length.i = zext i32 %_10.i to i64, !dbg !10017
  %157 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !10027
  %158 = bitcast <8 x float> %157 to <8 x i32>, !dbg !10035
  %159 = xor <8 x i32> %158, splat (i32 -1), !dbg !10041
  %160 = bitcast <8 x i32> %159 to <8 x float>, !dbg !10035
  switch i32 %link, label %bb13.i311 [
    i32 1, label %bb14.i312
    i32 3, label %bb14.i312.fold.split
  ], !dbg !10043

bb13.i311:                                        ; preds = %bb12
  br label %bb14.i312, !dbg !10044

bb14.i312.fold.split:                             ; preds = %bb12
  br label %bb14.i312, !dbg !10045

bb14.i312:                                        ; preds = %bb12, %bb14.i312.fold.split, %bb13.i311
  %_11.i300.sroa.0.02193 = phi <8 x float> [ %160, %bb12 ], [ %157, %bb13.i311 ], [ %157, %bb14.i312.fold.split ]
  %_13.i299.sroa.0.0 = phi <8 x i32> [ %159, %bb12 ], [ %159, %bb13.i311 ], [ %158, %bb14.i312.fold.split ], !dbg !10046
  %_4.i370 = getelementptr inbounds nuw i8, ptr %channel_left, i64 448, !dbg !10047
  %lanes.i806.sroa.0.0.copyload = load <8 x float>, ptr %_4.i370, align 32, !dbg !10050, !alias.scope !10055, !noalias !10059
  %_7.i371 = getelementptr inbounds nuw i8, ptr %channel_left, i64 480, !dbg !10065
  %lanes.i799.sroa.0.0.copyload = load <8 x float>, ptr %_7.i371, align 32, !dbg !10066, !alias.scope !10071, !noalias !10075
  %_13.i = getelementptr inbounds nuw i8, ptr %channel_left, i64 256, !dbg !10079
  %lanes.i792.sroa.0.0.copyload = load <8 x float>, ptr %_13.i, align 32, !dbg !10080, !alias.scope !10085, !noalias !10089
  %_14.i372 = getelementptr inbounds nuw i8, ptr %channel_left, i64 288, !dbg !10093
  %lanes.i785.sroa.0.0.copyload = load <8 x float>, ptr %_14.i372, align 32, !dbg !10094, !alias.scope !10099, !noalias !10103
  %_17.i373 = getelementptr inbounds nuw i8, ptr %channel_left, i64 320, !dbg !10107
  %lanes.i778.sroa.0.0.copyload = load <8 x float>, ptr %_17.i373, align 32, !dbg !10108, !alias.scope !10113, !noalias !10117
  %_20.i374 = getelementptr inbounds nuw i8, ptr %channel_left, i64 352, !dbg !10121
  %lanes.i771.sroa.0.0.copyload = load <8 x float>, ptr %_20.i374, align 32, !dbg !10122, !alias.scope !10127, !noalias !10131
  %_23.i375 = getelementptr inbounds nuw i8, ptr %channel_left, i64 384, !dbg !10135
  %lanes.i764.sroa.0.0.copyload = load <8 x float>, ptr %_23.i375, align 32, !dbg !10136, !alias.scope !10141, !noalias !10145
  %_26.i376 = getelementptr inbounds nuw i8, ptr %channel_left, i64 416, !dbg !10149
  %lanes.i.sroa.0.0.copyload = load <8 x float>, ptr %_26.i376, align 32, !dbg !10150, !alias.scope !10155, !noalias !10159
  %161 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i799.sroa.0.0.copyload, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !10163
  %162 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i799.sroa.0.0.copyload, <8 x float> zeroinitializer, i8 0), !dbg !10169
  %163 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i806.sroa.0.0.copyload, <8 x float> zeroinitializer, i8 0), !dbg !10175
  %164 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1440, !dbg !10181
  %_57.i = load i32, ptr %164, align 32, !dbg !10181, !alias.scope !10021, !noalias !10024, !noundef !11
  %_56.i = zext i32 %_57.i to i64, !dbg !10181
  %165 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1444, !dbg !10186
  %_60.i = load i32, ptr %165, align 4, !dbg !10186, !alias.scope !10021, !noalias !10024, !noundef !11
  %_58.not.i = icmp ne i32 %_60.i, %_57.i, !dbg !10186
  %166 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1448, !dbg !10186
  %_60.i.1 = load i32, ptr %166, align 4, !dbg !10186
  %_58.not.i.1 = icmp ne i32 %_60.i.1, %_57.i, !dbg !10186
  %or.cond3382.not3398 = select i1 %_58.not.i, i1 true, i1 %_58.not.i.1, !dbg !10186
  %167 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1452, !dbg !10186
  %_60.i.2 = load i32, ptr %167, align 4, !dbg !10186
  %_58.not.i.2 = icmp ne i32 %_60.i.2, %_57.i, !dbg !10186
  %or.cond3383.not3397 = select i1 %or.cond3382.not3398, i1 true, i1 %_58.not.i.2, !dbg !10186
  %168 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1456, !dbg !10186
  %_60.i.3 = load i32, ptr %168, align 4, !dbg !10186
  %_58.not.i.3 = icmp ne i32 %_60.i.3, %_57.i, !dbg !10186
  %or.cond3384.not3396 = select i1 %or.cond3383.not3397, i1 true, i1 %_58.not.i.3, !dbg !10186
  %169 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1460, !dbg !10186
  %_60.i.4 = load i32, ptr %169, align 4, !dbg !10186
  %_58.not.i.4 = icmp ne i32 %_60.i.4, %_57.i, !dbg !10186
  %or.cond3385.not3395 = select i1 %or.cond3384.not3396, i1 true, i1 %_58.not.i.4, !dbg !10186
  %170 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1464, !dbg !10186
  %_60.i.5 = load i32, ptr %170, align 4, !dbg !10186
  %_58.not.i.5 = icmp ne i32 %_60.i.5, %_57.i, !dbg !10186
  %or.cond3386.not3394 = select i1 %or.cond3385.not3395, i1 true, i1 %_58.not.i.5, !dbg !10186
  %171 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1468, !dbg !10186
  %_60.i.6 = load i32, ptr %171, align 4, !dbg !10186
  %_58.not.i.6 = icmp ne i32 %_60.i.6, %_57.i, !dbg !10186
  %or.cond3387.not = select i1 %or.cond3386.not3394, i1 true, i1 %_58.not.i.6, !dbg !10186
  %_6.i.i = load i64, ptr %detector, align 8, !range !220
  %172 = getelementptr inbounds nuw i8, ptr %detector, i64 16
  %sidechain_left.1.i.i = load i64, ptr %172, align 8
  %173 = getelementptr inbounds nuw i8, ptr %detector, i64 24
  %sidechain_right.0.i.i = load ptr, ptr %173, align 8, !nonnull !11, !align !3781
  %174 = getelementptr inbounds nuw i8, ptr %detector, i64 32
  %sidechain_right.1.i.i = load i64, ptr %174, align 8
  %175 = getelementptr inbounds nuw i8, ptr %detector, i64 8
  %sidechain_left.0.i.i = load ptr, ptr %175, align 8, !nonnull !11, !align !3781
  %176 = icmp slt <8 x i32> %_13.i299.sroa.0.0, zeroinitializer
  %177 = bitcast <8 x float> %_11.i300.sroa.0.02193 to <8 x i32>
  %178 = icmp slt <8 x i32> %177, zeroinitializer
  %179 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1536
  %180 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1504
  %181 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1512
  %182 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1528
  %183 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1520
  %_49.i = getelementptr inbounds nuw i8, ptr %channel_left, i64 1472
  %184 = bitcast <8 x float> %163 to <8 x i32>
  %185 = bitcast <8 x float> %162 to <8 x i32>
  %186 = select i1 %bypass, <8 x i32> %158, <8 x i32> %159
  %187 = or <8 x i32> %186, %185
  %188 = bitcast <8 x float> %161 to <8 x i32>
  %189 = icmp slt <8 x i32> %188, zeroinitializer
  %190 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1448
  %191 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1452
  %192 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1456
  %193 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1460
  %194 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1464
  %195 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1468
  %196 = fneg <8 x float> %lanes.i778.sroa.0.0.copyload
  br label %bb25.i, !dbg !10190

bb25.i:                                           ; preds = %bb14.i312, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit
  %start1.sroa.0.0.i2522 = phi i64 [ %spec.store.select, %bb14.i312 ], [ %197, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit ]
  %197 = add nuw nsw i64 %start1.sroa.0.0.i2522, 1, !dbg !10199
  %slot.i = shl nuw nsw i64 %start1.sroa.0.0.i2522, 3, !dbg !10207
  %_69.i = icmp samesign ugt i64 %slot.i, %left.1, !dbg !10209
  br i1 %_69.i, label %bb27.i, label %bb28.i, !dbg !10209, !prof !161

bb28.i:                                           ; preds = %bb25.i
  %_72.i = sub nuw nsw i64 %left.1, %slot.i, !dbg !10215
  %_76.i = getelementptr inbounds nuw float, ptr %left.0, i64 %slot.i, !dbg !10216
  %_8.i1022 = icmp samesign ugt i64 %_72.i, 7, !dbg !10221
  br i1 %_8.i1022, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1026, label %bb2.i1023, !dbg !10221, !prof !2135

bb2.i1023:                                        ; preds = %bb28.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_72.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !10226, !noalias !10227
  unreachable, !dbg !10226

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1026: ; preds = %bb28.i
  %lanes.i1019.sroa.0.0.copyload = load <8 x float>, ptr %_76.i, align 4, !dbg !10231, !alias.scope !10235, !noalias !10239
  switch i64 %_6.i.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1026.unreachabledefault [
    i64 0, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i
    i64 1, label %bb3.i.i
    i64 2, label %bb2.i.i
  ], !dbg !10241

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1026.unreachabledefault: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1026
  unreachable

default.unreachable:                              ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit349.i
  unreachable

bb3.i.i:                                          ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1026
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i, !dbg !10244

bb2.i.i:                                          ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1026
  %_25.i.i = icmp ugt i64 %slot.i, %sidechain_left.1.i.i, !dbg !10245
  br i1 %_25.i.i, label %bb17.i.i, label %bb18.i.i, !dbg !10245, !prof !161

bb18.i.i:                                         ; preds = %bb2.i.i
  %_28.i.i = sub nuw i64 %sidechain_left.1.i.i, %slot.i, !dbg !10248
  %_8.i1013 = icmp samesign ugt i64 %_28.i.i, 7, !dbg !10249
  br i1 %_8.i1013, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1017, label %bb2.i1014, !dbg !10249, !prof !2135

bb2.i1014:                                        ; preds = %bb18.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_28.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !10254, !noalias !10255
  unreachable, !dbg !10254

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1017: ; preds = %bb18.i.i
  %_32.i.i = getelementptr inbounds nuw float, ptr %sidechain_left.0.i.i, i64 %slot.i, !dbg !10264
  %lanes.i1010.sroa.0.0.copyload = load <8 x float>, ptr %_32.i.i, align 4, !dbg !10266, !alias.scope !10270, !noalias !10274
  %_33.i.i = icmp ugt i64 %slot.i, %sidechain_right.1.i.i, !dbg !10276
  br i1 %_33.i.i, label %bb19.i.i, label %bb20.i.i, !dbg !10276, !prof !161

bb17.i.i:                                         ; preds = %bb2.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 34359738361) %slot.i, i64 noundef %sidechain_left.1.i.i, i64 noundef %sidechain_left.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7fe5a91ec575deda110c046c84630100) #23, !dbg !10279, !noalias !10280
  unreachable, !dbg !10279

bb20.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1017
  %_36.i.i = sub nuw i64 %sidechain_right.1.i.i, %slot.i, !dbg !10282
  %_8.i1004 = icmp samesign ugt i64 %_36.i.i, 7, !dbg !10283
  br i1 %_8.i1004, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1008, label %bb2.i1005, !dbg !10283, !prof !2135

bb2.i1005:                                        ; preds = %bb20.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_36.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !10288, !noalias !10289
  unreachable, !dbg !10288

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1008: ; preds = %bb20.i.i
  %_40.i.i = getelementptr inbounds nuw float, ptr %sidechain_right.0.i.i, i64 %slot.i, !dbg !10293
  %lanes.i1001.sroa.0.0.copyload = load <8 x float>, ptr %_40.i.i, align 4, !dbg !10295, !alias.scope !10299, !noalias !10303
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i, !dbg !10305

bb19.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1017
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 34359738361) %slot.i, i64 noundef %sidechain_right.1.i.i, i64 noundef %sidechain_right.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f2cba1f89421a726339097d2eba48536) #23, !dbg !10306, !noalias !10280
  unreachable, !dbg !10306

_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1026, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1008, %bb3.i.i
  %.sroa.01498.0 = phi <8 x float> [ %lanes.i1019.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1026 ], [ zeroinitializer, %bb3.i.i ], [ %lanes.i1001.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1008 ], !dbg !10307
  %.sroa.01494.0 = phi <8 x float> [ %lanes.i1019.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1026 ], [ zeroinitializer, %bb3.i.i ], [ %lanes.i1010.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit1008 ], !dbg !10307
  %198 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %.sroa.01494.0), !dbg !10308
  %199 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %.sroa.01498.0), !dbg !10314
  %200 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %198, <8 x float> %199), !dbg !10320
  %201 = fmul <8 x float> %198, splat (float 5.000000e-01), !dbg !10325
  %202 = fmul <8 x float> %199, splat (float 5.000000e-01), !dbg !10330
  %203 = fadd <8 x float> %201, %202, !dbg !10335
  %204 = select <8 x i1> %176, <8 x float> %203, <8 x float> %200, !dbg !10340
  %205 = select <8 x i1> %178, <8 x float> %204, <8 x float> %198, !dbg !10345
  %_29.i = load i32, ptr %179, align 32, !dbg !10350, !alias.scope !10021, !noalias !10024, !noundef !11
  %write.i = zext i32 %_29.i to i64, !dbg !10350
  %206 = add nuw nsw i64 %write.i, 1, !dbg !10352
  %_31.i = icmp eq i64 %206, %ring_length.i, !dbg !10354
  %spec.store.select.i = select i1 %_31.i, i64 0, i64 %206, !dbg !10354
  %_107.1.i = load i64, ptr %181, align 8, !dbg !10356, !alias.scope !10021, !noalias !10024, !noundef !11
  %_35.i = shl nuw nsw i64 %write.i, 3, !dbg !10358
  %_77.i = icmp ugt i64 %_35.i, %_107.1.i, !dbg !10359
  br i1 %_77.i, label %bb29.i, label %bb30.i, !dbg !10359, !prof !161

bb27.i:                                           ; preds = %bb25.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %slot.i, i64 noundef range(i64 8, 34359738361) %left.1, i64 noundef range(i64 8, 34359738361) %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8632ba88614e3313b76c3c784bc00e8d) #23, !dbg !10364, !noalias !10365
  unreachable, !dbg !10364

bb30.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i
  %_80.i = sub nuw i64 %_107.1.i, %_35.i, !dbg !10366
  %_8.i1329 = icmp samesign ugt i64 %_80.i, 7, !dbg !10367
  br i1 %_8.i1329, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1332, label %bb2.i1330, !dbg !10367, !prof !2135

bb2.i1330:                                        ; preds = %bb30.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_80.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !10372, !noalias !10373
  unreachable, !dbg !10372

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1332: ; preds = %bb30.i
  %_107.0.i = load ptr, ptr %180, align 32, !dbg !10356, !alias.scope !10021, !noalias !10024, !nonnull !11, !noundef !11
  %_84.i = getelementptr inbounds nuw float, ptr %_107.0.i, i64 %_35.i, !dbg !10377
  store <8 x float> %lanes.i1019.sroa.0.0.copyload, ptr %_84.i, align 4, !dbg !10382, !alias.scope !10386, !noalias !10390
  %_108.1.i = load i64, ptr %182, align 8, !dbg !10392, !alias.scope !10021, !noalias !10024, !noundef !11
  %_85.i = icmp ugt i64 %_35.i, %_108.1.i, !dbg !10393
  br i1 %_85.i, label %bb31.i, label %bb32.i, !dbg !10393, !prof !161

bb29.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_35.i, i64 noundef %_107.1.i, i64 noundef %_107.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d362267d85009f8e23c4d340b28a745d) #23, !dbg !10397, !noalias !10365
  unreachable, !dbg !10397

bb32.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1332
  %_88.i = sub nuw i64 %_108.1.i, %_35.i, !dbg !10398
  %_8.i1324 = icmp samesign ugt i64 %_88.i, 7, !dbg !10399
  br i1 %_8.i1324, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1327, label %bb2.i1325, !dbg !10399, !prof !2135

bb2.i1325:                                        ; preds = %bb32.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_88.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !10404, !noalias !10405
  unreachable, !dbg !10404

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1327: ; preds = %bb32.i
  %_108.0.i = load ptr, ptr %183, align 16, !dbg !10392, !alias.scope !10021, !noalias !10024, !nonnull !11, !noundef !11
  %_92.i = getelementptr inbounds nuw float, ptr %_108.0.i, i64 %_35.i, !dbg !10409
  store <8 x float> %205, ptr %_92.i, align 4, !dbg !10414, !alias.scope !10418, !noalias !10422
  %_109.1.i = load i64, ptr %181, align 8, !dbg !10424, !alias.scope !10021, !noalias !10024, !noundef !11
  %_41.i = shl nuw nsw i64 %spec.store.select.i, 3, !dbg !10425
  %_93.i = icmp ugt i64 %_41.i, %_109.1.i, !dbg !10426
  br i1 %_93.i, label %bb33.i, label %bb34.i, !dbg !10426, !prof !161

bb31.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1332
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_35.i, i64 noundef %_108.1.i, i64 noundef %_108.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1ce527b0bad7f9d2c89100bcea389654) #23, !dbg !10430, !noalias !10365
  unreachable, !dbg !10430

bb34.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1327
  %_96.i = sub nuw i64 %_109.1.i, %_41.i, !dbg !10431
  %_8.i995 = icmp samesign ugt i64 %_96.i, 7, !dbg !10432
  br i1 %_8.i995, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit999, label %bb2.i996, !dbg !10432, !prof !2135

bb2.i996:                                         ; preds = %bb34.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_96.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !10437, !noalias !10438
  unreachable, !dbg !10437

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit999: ; preds = %bb34.i
  %_109.0.i = load ptr, ptr %180, align 32, !dbg !10424, !alias.scope !10021, !noalias !10024, !nonnull !11, !noundef !11
  %_100.i = getelementptr inbounds nuw float, ptr %_109.0.i, i64 %_41.i, !dbg !10442
  %lanes.i992.sroa.0.0.copyload = load <8 x float>, ptr %_100.i, align 4, !dbg !10447, !alias.scope !10451, !noalias !10455
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10457), !dbg !10460
  %_6.i8.i = load i32, ptr %156, align 4, !dbg !10462, !alias.scope !10464, !noalias !10465, !noundef !11
  %ring_length.i.i = zext i32 %_6.i8.i to i64, !dbg !10462
  br i1 %or.cond3387.not, label %bb7.i.i.preheader, label %bb1.i.i, !dbg !10468

bb7.i.i.preheader:                                ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit999
  %_56.1.i.i = load i64, ptr %182, align 8
  %_56.0.i.i = load ptr, ptr %183, align 16, !nonnull !11
  %_25.i14.i = load i32, ptr %164, align 4, !dbg !10469, !alias.scope !10464, !noalias !10465, !noundef !11
  %delay1.i.i = zext i32 %_25.i14.i to i64, !dbg !10469
  %_28.not.i.i = icmp ult i32 %_29.i, %_25.i14.i, !dbg !10470
  %_29.i.i = select i1 %_28.not.i.i, i64 %ring_length.i.i, i64 0, !dbg !10470
  %write.pn7.i.i = sub nsw i64 %write.i, %delay1.i.i, !dbg !10470
  %tap.sroa.0.0.i.i = add nsw i64 %write.pn7.i.i, %_29.i.i, !dbg !10471
  %_32.i15.i = shl nsw i64 %tap.sroa.0.0.i.i, 3, !dbg !10472
  %_34.i.i = icmp ult i64 %_32.i15.i, %_56.1.i.i, !dbg !10473
  br i1 %_34.i.i, label %bb16.i.i, label %panic2.i.i, !dbg !10473

bb1.i.i:                                          ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit999
  %_10.not.i.i = icmp ult i32 %_29.i, %_57.i, !dbg !10474
  %_11.i9.i = select i1 %_10.not.i.i, i64 %ring_length.i.i, i64 0, !dbg !10474
  %write.pn.i.i = sub nsw i64 %write.i, %_56.i, !dbg !10474
  %row.sroa.0.0.i.i = add nsw i64 %write.pn.i.i, %_11.i9.i, !dbg !10475
  %_55.1.i.i = load i64, ptr %182, align 8, !dbg !10476, !alias.scope !10464, !noalias !10465, !noundef !11
  %_13.i.i = shl nsw i64 %row.sroa.0.0.i.i, 3, !dbg !10477
  %_40.i10.i = icmp ugt i64 %_13.i.i, %_55.1.i.i, !dbg !10478
  br i1 %_40.i10.i, label %bb19.i13.i, label %bb20.i11.i, !dbg !10478, !prof !161

bb20.i11.i:                                       ; preds = %bb1.i.i
  %_43.i.i = sub nuw i64 %_55.1.i.i, %_13.i.i, !dbg !10481
  %_8.i986 = icmp samesign ugt i64 %_43.i.i, 7, !dbg !10482
  br i1 %_8.i986, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit990, label %bb2.i987, !dbg !10482, !prof !2135

bb2.i987:                                         ; preds = %bb20.i11.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_43.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !10487, !noalias !10488
  unreachable, !dbg !10487

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit990: ; preds = %bb20.i11.i
  %_55.0.i.i = load ptr, ptr %183, align 16, !dbg !10476, !alias.scope !10464, !noalias !10465, !nonnull !11, !noundef !11
  %_47.i.i = getelementptr inbounds nuw float, ptr %_55.0.i.i, i64 %_13.i.i, !dbg !10492
  %lanes.i983.sroa.0.0.copyload = load <8 x float>, ptr %_47.i.i, align 4, !dbg !10494, !alias.scope !10498, !noalias !10502
  br label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit, !dbg !10504

bb19.i13.i:                                       ; preds = %bb1.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_13.i.i, i64 noundef %_55.1.i.i, i64 noundef %_55.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_942b76445104c042a726096e85fe3fcb) #23, !dbg !10505, !noalias !10506
  unreachable, !dbg !10505

bb11.i.i:                                         ; preds = %bb12.i.i.7
  %207 = getelementptr inbounds nuw float, ptr %_56.0.i.i, i64 %_31.i.i.6, !dbg !10473
  %_30.i.i.6 = load float, ptr %207, align 4, !dbg !10473, !noalias !10507, !noundef !11
  %208 = getelementptr inbounds nuw float, ptr %_56.0.i.i, i64 %_31.i.i.7, !dbg !10473
  %_30.i.i.7 = load float, ptr %208, align 4, !dbg !10473, !noalias !10507, !noundef !11
  %lanes.i976.sroa.0.0.vec.insert = insertelement <8 x float> poison, float %_30.i.i, i64 0, !dbg !10508
  %lanes.i976.sroa.0.4.vec.insert = insertelement <8 x float> %lanes.i976.sroa.0.0.vec.insert, float %_30.i.i.1, i64 1, !dbg !10508
  %lanes.i976.sroa.0.8.vec.insert = insertelement <8 x float> %lanes.i976.sroa.0.4.vec.insert, float %_30.i.i.2, i64 2, !dbg !10508
  %lanes.i976.sroa.0.12.vec.insert = insertelement <8 x float> %lanes.i976.sroa.0.8.vec.insert, float %_30.i.i.3, i64 3, !dbg !10508
  %lanes.i976.sroa.0.16.vec.insert = insertelement <8 x float> %lanes.i976.sroa.0.12.vec.insert, float %_30.i.i.4, i64 4, !dbg !10508
  %lanes.i976.sroa.0.20.vec.insert = insertelement <8 x float> %lanes.i976.sroa.0.16.vec.insert, float %_30.i.i.5, i64 5, !dbg !10508
  %lanes.i976.sroa.0.24.vec.insert = insertelement <8 x float> %lanes.i976.sroa.0.20.vec.insert, float %_30.i.i.6, i64 6, !dbg !10508
  %lanes.i976.sroa.0.28.vec.insert = insertelement <8 x float> %lanes.i976.sroa.0.24.vec.insert, float %_30.i.i.7, i64 7, !dbg !10508
  br label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit, !dbg !10504

bb16.i.i:                                         ; preds = %bb7.i.i.preheader
  %209 = getelementptr inbounds nuw float, ptr %_56.0.i.i, i64 %_32.i15.i, !dbg !10473
  %_30.i.i = load float, ptr %209, align 4, !dbg !10473, !noalias !10507, !noundef !11
  %_25.i14.i.1 = load i32, ptr %165, align 4, !dbg !10469, !alias.scope !10464, !noalias !10465, !noundef !11
  %delay1.i.i.1 = zext i32 %_25.i14.i.1 to i64, !dbg !10469
  %_28.not.i.i.1 = icmp ult i32 %_29.i, %_25.i14.i.1, !dbg !10470
  %_29.i.i.1 = select i1 %_28.not.i.i.1, i64 %ring_length.i.i, i64 0, !dbg !10470
  %write.pn7.i.i.1 = sub nsw i64 %write.i, %delay1.i.i.1, !dbg !10470
  %tap.sroa.0.0.i.i.1 = add nsw i64 %write.pn7.i.i.1, %_29.i.i.1, !dbg !10471
  %_32.i15.i.1 = shl nsw i64 %tap.sroa.0.0.i.i.1, 3, !dbg !10472
  %_31.i.i.1 = or disjoint i64 %_32.i15.i.1, 1, !dbg !10472
  %_34.i.i.1 = icmp ult i64 %_31.i.i.1, %_56.1.i.i, !dbg !10473
  br i1 %_34.i.i.1, label %bb16.i.i.1, label %panic2.i.i, !dbg !10473

bb16.i.i.1:                                       ; preds = %bb16.i.i
  %210 = getelementptr inbounds nuw float, ptr %_56.0.i.i, i64 %_31.i.i.1, !dbg !10473
  %_30.i.i.1 = load float, ptr %210, align 4, !dbg !10473, !noalias !10507, !noundef !11
  %_25.i14.i.2 = load i32, ptr %190, align 4, !dbg !10469, !alias.scope !10464, !noalias !10465, !noundef !11
  %delay1.i.i.2 = zext i32 %_25.i14.i.2 to i64, !dbg !10469
  %_28.not.i.i.2 = icmp ult i32 %_29.i, %_25.i14.i.2, !dbg !10470
  %_29.i.i.2 = select i1 %_28.not.i.i.2, i64 %ring_length.i.i, i64 0, !dbg !10470
  %write.pn7.i.i.2 = sub nsw i64 %write.i, %delay1.i.i.2, !dbg !10470
  %tap.sroa.0.0.i.i.2 = add nsw i64 %write.pn7.i.i.2, %_29.i.i.2, !dbg !10471
  %_32.i15.i.2 = shl nsw i64 %tap.sroa.0.0.i.i.2, 3, !dbg !10472
  %_31.i.i.2 = or disjoint i64 %_32.i15.i.2, 2, !dbg !10472
  %_34.i.i.2 = icmp ult i64 %_31.i.i.2, %_56.1.i.i, !dbg !10473
  br i1 %_34.i.i.2, label %bb16.i.i.2, label %panic2.i.i, !dbg !10473

bb16.i.i.2:                                       ; preds = %bb16.i.i.1
  %211 = getelementptr inbounds nuw float, ptr %_56.0.i.i, i64 %_31.i.i.2, !dbg !10473
  %_30.i.i.2 = load float, ptr %211, align 4, !dbg !10473, !noalias !10507, !noundef !11
  %_25.i14.i.3 = load i32, ptr %191, align 4, !dbg !10469, !alias.scope !10464, !noalias !10465, !noundef !11
  %delay1.i.i.3 = zext i32 %_25.i14.i.3 to i64, !dbg !10469
  %_28.not.i.i.3 = icmp ult i32 %_29.i, %_25.i14.i.3, !dbg !10470
  %_29.i.i.3 = select i1 %_28.not.i.i.3, i64 %ring_length.i.i, i64 0, !dbg !10470
  %write.pn7.i.i.3 = sub nsw i64 %write.i, %delay1.i.i.3, !dbg !10470
  %tap.sroa.0.0.i.i.3 = add nsw i64 %write.pn7.i.i.3, %_29.i.i.3, !dbg !10471
  %_32.i15.i.3 = shl nsw i64 %tap.sroa.0.0.i.i.3, 3, !dbg !10472
  %_31.i.i.3 = or disjoint i64 %_32.i15.i.3, 3, !dbg !10472
  %_34.i.i.3 = icmp ult i64 %_31.i.i.3, %_56.1.i.i, !dbg !10473
  br i1 %_34.i.i.3, label %bb16.i.i.3, label %panic2.i.i, !dbg !10473

bb16.i.i.3:                                       ; preds = %bb16.i.i.2
  %212 = getelementptr inbounds nuw float, ptr %_56.0.i.i, i64 %_31.i.i.3, !dbg !10473
  %_30.i.i.3 = load float, ptr %212, align 4, !dbg !10473, !noalias !10507, !noundef !11
  %_25.i14.i.4 = load i32, ptr %192, align 4, !dbg !10469, !alias.scope !10464, !noalias !10465, !noundef !11
  %delay1.i.i.4 = zext i32 %_25.i14.i.4 to i64, !dbg !10469
  %_28.not.i.i.4 = icmp ult i32 %_29.i, %_25.i14.i.4, !dbg !10470
  %_29.i.i.4 = select i1 %_28.not.i.i.4, i64 %ring_length.i.i, i64 0, !dbg !10470
  %write.pn7.i.i.4 = sub nsw i64 %write.i, %delay1.i.i.4, !dbg !10470
  %tap.sroa.0.0.i.i.4 = add nsw i64 %write.pn7.i.i.4, %_29.i.i.4, !dbg !10471
  %_32.i15.i.4 = shl nsw i64 %tap.sroa.0.0.i.i.4, 3, !dbg !10472
  %_31.i.i.4 = or disjoint i64 %_32.i15.i.4, 4, !dbg !10472
  %_34.i.i.4 = icmp ult i64 %_31.i.i.4, %_56.1.i.i, !dbg !10473
  br i1 %_34.i.i.4, label %bb16.i.i.4, label %panic2.i.i, !dbg !10473

bb16.i.i.4:                                       ; preds = %bb16.i.i.3
  %213 = getelementptr inbounds nuw float, ptr %_56.0.i.i, i64 %_31.i.i.4, !dbg !10473
  %_30.i.i.4 = load float, ptr %213, align 4, !dbg !10473, !noalias !10507, !noundef !11
  %_25.i14.i.5 = load i32, ptr %193, align 4, !dbg !10469, !alias.scope !10464, !noalias !10465, !noundef !11
  %delay1.i.i.5 = zext i32 %_25.i14.i.5 to i64, !dbg !10469
  %_28.not.i.i.5 = icmp ult i32 %_29.i, %_25.i14.i.5, !dbg !10470
  %_29.i.i.5 = select i1 %_28.not.i.i.5, i64 %ring_length.i.i, i64 0, !dbg !10470
  %write.pn7.i.i.5 = sub nsw i64 %write.i, %delay1.i.i.5, !dbg !10470
  %tap.sroa.0.0.i.i.5 = add nsw i64 %write.pn7.i.i.5, %_29.i.i.5, !dbg !10471
  %_32.i15.i.5 = shl nsw i64 %tap.sroa.0.0.i.i.5, 3, !dbg !10472
  %_31.i.i.5 = or disjoint i64 %_32.i15.i.5, 5, !dbg !10472
  %_34.i.i.5 = icmp ult i64 %_31.i.i.5, %_56.1.i.i, !dbg !10473
  br i1 %_34.i.i.5, label %bb16.i.i.5, label %panic2.i.i, !dbg !10473

bb16.i.i.5:                                       ; preds = %bb16.i.i.4
  %214 = getelementptr inbounds nuw float, ptr %_56.0.i.i, i64 %_31.i.i.5, !dbg !10473
  %_30.i.i.5 = load float, ptr %214, align 4, !dbg !10473, !noalias !10507, !noundef !11
  %_25.i14.i.6 = load i32, ptr %194, align 4, !dbg !10469, !alias.scope !10464, !noalias !10465, !noundef !11
  %delay1.i.i.6 = zext i32 %_25.i14.i.6 to i64, !dbg !10469
  %_28.not.i.i.6 = icmp ult i32 %_29.i, %_25.i14.i.6, !dbg !10470
  %_29.i.i.6 = select i1 %_28.not.i.i.6, i64 %ring_length.i.i, i64 0, !dbg !10470
  %write.pn7.i.i.6 = sub nsw i64 %write.i, %delay1.i.i.6, !dbg !10470
  %tap.sroa.0.0.i.i.6 = add nsw i64 %write.pn7.i.i.6, %_29.i.i.6, !dbg !10471
  %_32.i15.i.6 = shl nsw i64 %tap.sroa.0.0.i.i.6, 3, !dbg !10472
  %_31.i.i.6 = or disjoint i64 %_32.i15.i.6, 6, !dbg !10472
  %_34.i.i.6 = icmp ult i64 %_31.i.i.6, %_56.1.i.i, !dbg !10473
  br i1 %_34.i.i.6, label %bb12.i.i.7, label %panic2.i.i, !dbg !10473

bb12.i.i.7:                                       ; preds = %bb16.i.i.5
  %_25.i14.i.7 = load i32, ptr %195, align 4, !dbg !10469, !alias.scope !10464, !noalias !10465, !noundef !11
  %delay1.i.i.7 = zext i32 %_25.i14.i.7 to i64, !dbg !10469
  %_28.not.i.i.7 = icmp ult i32 %_29.i, %_25.i14.i.7, !dbg !10470
  %_29.i.i.7 = select i1 %_28.not.i.i.7, i64 %ring_length.i.i, i64 0, !dbg !10470
  %write.pn7.i.i.7 = sub nsw i64 %write.i, %delay1.i.i.7, !dbg !10470
  %tap.sroa.0.0.i.i.7 = add nsw i64 %write.pn7.i.i.7, %_29.i.i.7, !dbg !10471
  %_32.i15.i.7 = shl nsw i64 %tap.sroa.0.0.i.i.7, 3, !dbg !10472
  %_31.i.i.7 = or disjoint i64 %_32.i15.i.7, 7, !dbg !10472
  %_34.i.i.7 = icmp ult i64 %_31.i.i.7, %_56.1.i.i, !dbg !10473
  br i1 %_34.i.i.7, label %bb11.i.i, label %panic2.i.i, !dbg !10473

panic2.i.i:                                       ; preds = %bb12.i.i.7, %bb16.i.i.5, %bb16.i.i.4, %bb16.i.i.3, %bb16.i.i.2, %bb16.i.i.1, %bb16.i.i, %bb7.i.i.preheader
  %_31.i.i.lcssa = phi i64 [ %_32.i15.i, %bb7.i.i.preheader ], [ %_31.i.i.1, %bb16.i.i ], [ %_31.i.i.2, %bb16.i.i.1 ], [ %_31.i.i.3, %bb16.i.i.2 ], [ %_31.i.i.4, %bb16.i.i.3 ], [ %_31.i.i.5, %bb16.i.i.4 ], [ %_31.i.i.6, %bb16.i.i.5 ], [ %_31.i.i.7, %bb12.i.i.7 ], !dbg !10472
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_31.i.i.lcssa, i64 noundef %_56.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c4f76d0874ad32f224dcaa5d3453a9e1) #23, !dbg !10473, !noalias !10507
  unreachable, !dbg !10473

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit: ; preds = %bb11.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit990
  %detected_left.i.sroa.0.0 = phi <8 x float> [ %lanes.i976.sroa.0.28.vec.insert, %bb11.i.i ], [ %lanes.i983.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit990 ], !dbg !10513
  %215 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %detected_left.i.sroa.0.0, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !10516
  %216 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %215, <8 x float> splat (float 0x3810000000000000)), !dbg !10525
  %217 = bitcast <8 x float> %216 to <4 x i64>, !dbg !10532
  %218 = and <4 x i64> %217, splat (i64 36028792732385279), !dbg !10533
  %219 = or disjoint <4 x i64> %218, splat (i64 4575657222473777152), !dbg !10538
  %220 = bitcast <4 x i64> %219 to <8 x float>, !dbg !10542
  %221 = fadd <8 x float> %220, splat (float -1.000000e+00), !dbg !10543
  %222 = fmul <8 x float> %221, splat (float 0xBF9B17A960000000), !dbg !10548
  %223 = fadd <8 x float> %222, splat (float 0x3FBF9A8440000000), !dbg !10553
  %224 = fmul <8 x float> %221, %223, !dbg !10548
  %225 = fadd <8 x float> %224, splat (float 0xBFD1E3F400000000), !dbg !10553
  %226 = fmul <8 x float> %221, %225, !dbg !10548
  %227 = fadd <8 x float> %226, splat (float 0x3FDD544F20000000), !dbg !10553
  %228 = fmul <8 x float> %221, %227, !dbg !10548
  %229 = fadd <8 x float> %228, splat (float 0xBFE6FC2A60000000), !dbg !10553
  %230 = fmul <8 x float> %221, %229, !dbg !10548
  %231 = fadd <8 x float> %230, splat (float 0x3FF714B2A0000000), !dbg !10553
  %232 = bitcast <8 x float> %216 to <8 x i32>, !dbg !10558
  %_3.i1450 = lshr <8 x i32> %232, splat (i32 23), !dbg !10562
  %233 = or disjoint <8 x i32> %_3.i1450, splat (i32 1258291200), !dbg !10563
  %234 = bitcast <8 x i32> %233 to <8 x float>, !dbg !10567
  %235 = fadd <8 x float> %234, splat (float 0xC160000FE0000000), !dbg !10568
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10572), !dbg !10575
  %_6.i195.sroa.0.0.copyload = load <8 x float>, ptr %_49.i, align 32, !dbg !10577, !noalias !10579
  %236 = fmul <8 x float> %221, %231, !dbg !10582
  %237 = fadd <8 x float> %235, %236, !dbg !10587
  %238 = fmul <8 x float> %237, splat (float 0x4018151820000000), !dbg !10592
  %239 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %238, <8 x float> splat (float -1.600000e+02)), !dbg !10597
  %240 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %239, <8 x float> splat (float 2.400000e+01)), !dbg !10602
  %241 = fsub <8 x float> %240, %lanes.i792.sroa.0.0.copyload, !dbg !10607
  %242 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %241, <8 x float> %lanes.i778.sroa.0.0.copyload, i8 30), !dbg !10613
  %243 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %241, <8 x float> %196, i8 18), !dbg !10619
  %244 = fadd <8 x float> %lanes.i778.sroa.0.0.copyload, %241, !dbg !10625
  %245 = fmul <8 x float> %244, %244, !dbg !10630
  %246 = fmul <8 x float> %lanes.i771.sroa.0.0.copyload, %245, !dbg !10635
  %247 = bitcast <8 x float> %242 to <8 x i32>, !dbg !10640
  %248 = icmp slt <8 x i32> %247, zeroinitializer, !dbg !10644
  %.v2217 = select <8 x i1> %248, <8 x float> %241, <8 x float> %246, !dbg !10644
  %249 = fmul <8 x float> %lanes.i785.sroa.0.0.copyload, %.v2217, !dbg !10644
  %250 = bitcast <8 x float> %243 to <8 x i32>, !dbg !10646
  %251 = icmp slt <8 x i32> %250, zeroinitializer, !dbg !10650
  %252 = select <8 x i1> %251, <8 x float> zeroinitializer, <8 x float> %249, !dbg !10650
  %253 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %252, <8 x float> splat (float -1.000000e+02)), !dbg !10652
  %254 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %253, <8 x float> zeroinitializer), !dbg !10657
  %255 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %254, <8 x float> %_6.i195.sroa.0.0.copyload, i8 17), !dbg !10662
  %256 = bitcast <8 x float> %255 to <8 x i32>, !dbg !10668
  %257 = icmp slt <8 x i32> %256, zeroinitializer, !dbg !10672
  %258 = select <8 x i1> %257, <8 x float> %lanes.i764.sroa.0.0.copyload, <8 x float> %lanes.i.sroa.0.0.copyload, !dbg !10672
  %259 = fsub <8 x float> %254, %_6.i195.sroa.0.0.copyload, !dbg !10674
  %260 = fmul <8 x float> %259, %258, !dbg !10680
  %261 = fadd <8 x float> %_6.i195.sroa.0.0.copyload, %260, !dbg !10685
  %262 = bitcast <8 x float> %261 to <8 x i32>, !dbg !10689
  %263 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %261), !dbg !10695
  %264 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %263, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !10697
  %265 = bitcast <8 x float> %264 to <8 x i32>, !dbg !10703
  %266 = xor <8 x i32> %265, splat (i32 -1), !dbg !10709
  %267 = and <8 x i32> %266, %262, !dbg !10711
  store <8 x i32> %267, ptr %_49.i, align 32, !dbg !10715, !alias.scope !10716, !noalias !10718
  %268 = bitcast <8 x i32> %267 to <8 x float>, !dbg !10719
  %269 = fadd <8 x float> %lanes.i806.sroa.0.0.copyload, %268, !dbg !10720
  %270 = fmul <8 x float> %269, splat (float 0x3FC542A5A0000000), !dbg !10727
  %271 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %270, <8 x float> splat (float -1.260000e+02)), !dbg !10733
  %272 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %271, <8 x float> splat (float 1.270000e+02)), !dbg !10739
  %273 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %272), !dbg !10744
  %274 = fsub <8 x float> %272, %273, !dbg !10749
  %275 = fmul <8 x float> %274, splat (float 0x3F5E974FA0000000), !dbg !10754
  %276 = fadd <8 x float> %275, splat (float 0x3F82778560000000), !dbg !10759
  %277 = fmul <8 x float> %274, %276, !dbg !10754
  %278 = fadd <8 x float> %277, splat (float 0x3FAC91CE60000000), !dbg !10759
  %279 = fmul <8 x float> %274, %278, !dbg !10754
  %280 = fadd <8 x float> %279, splat (float 0x3FCEBDB560000000), !dbg !10759
  %281 = fmul <8 x float> %274, %280, !dbg !10754
  %282 = fadd <8 x float> %281, splat (float 0x3FE62E4BA0000000), !dbg !10759
  %283 = fmul <8 x float> %274, %282, !dbg !10764
  %284 = fadd <8 x float> %283, splat (float 1.000000e+00), !dbg !10769
  %285 = fadd <8 x float> %273, splat (float 0x4160000FE0000000), !dbg !10774
  %286 = bitcast <8 x float> %285 to <8 x i32>, !dbg !10779
  %_3.i1451 = shl <8 x i32> %286, splat (i32 23), !dbg !10783
  %287 = bitcast <8 x i32> %_3.i1451 to <8 x float>, !dbg !10784
  %288 = fmul <8 x float> %284, %287, !dbg !10786
  %289 = fmul <8 x float> %lanes.i992.sroa.0.0.copyload, %288, !dbg !10790
  %290 = fsub <8 x float> %289, %lanes.i992.sroa.0.0.copyload, !dbg !10795
  %291 = fmul <8 x float> %lanes.i799.sroa.0.0.copyload, %290, !dbg !10801
  %292 = fadd <8 x float> %lanes.i992.sroa.0.0.copyload, %291, !dbg !10806
  %293 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %268, <8 x float> zeroinitializer, i8 0), !dbg !10810
  %294 = bitcast <8 x float> %293 to <8 x i32>, !dbg !10816
  %295 = and <8 x i32> %294, %184, !dbg !10820
  %296 = or <8 x i32> %187, %295, !dbg !10822
  %297 = select <8 x i1> %189, <8 x float> %289, <8 x float> %292, !dbg !10827
  %298 = icmp slt <8 x i32> %296, zeroinitializer, !dbg !10832
  %299 = select <8 x i1> %298, <8 x float> %lanes.i992.sroa.0.0.copyload, <8 x float> %297, !dbg !10832
  store <8 x float> %299, ptr %_76.i, align 4, !dbg !10837, !alias.scope !10843, !noalias !10847
  %300 = trunc i64 %spec.store.select.i to i32, !dbg !10851
  store i32 %300, ptr %179, align 32, !dbg !10851, !alias.scope !10021, !noalias !10024
  %exitcond2861.not = icmp eq i64 %197, %frames, !dbg !10852
  br i1 %exitcond2861.not, label %bb16, label %bb25.i, !dbg !10190

bb33.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit1327
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.i, i64 noundef %_109.1.i, i64 noundef %_109.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_824b3214541c2b0b46a15e52dc0ae67e) #23, !dbg !10856, !noalias !10365
  unreachable, !dbg !10856

bb10:                                             ; preds = %bb1.i.preheader
  %_36.0 = load ptr, ptr %staged, align 8, !dbg !10857, !nonnull !11, !noundef !11
  %301 = getelementptr inbounds nuw i8, ptr %staged, i64 8, !dbg !10857
  %_36.1 = load i64, ptr %301, align 8, !dbg !10857, !noundef !11
  %302 = getelementptr inbounds nuw i8, ptr %staged, i64 32, !dbg !10859
  %_37.0 = load ptr, ptr %302, align 8, !dbg !10859, !nonnull !11, !noundef !11
  %303 = getelementptr inbounds nuw i8, ptr %staged, i64 40, !dbg !10859
  %_37.1 = load i64, ptr %303, align 8, !dbg !10859, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10860), !dbg !10863
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10864), !dbg !10863
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10866), !dbg !10863
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10868), !dbg !10863
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10870), !dbg !10863
  %304 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1540, !dbg !10872
  %_11.i1452 = load i32, ptr %304, align 4, !dbg !10872, !alias.scope !10866, !noalias !10876, !noundef !11
  %ring_length.i1453 = zext i32 %_11.i1452 to i64, !dbg !10872
  %305 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !10877
  %306 = bitcast <8 x float> %305 to <8 x i32>, !dbg !10886
  %307 = xor <8 x i32> %306, splat (i32 -1), !dbg !10892
  %308 = bitcast <8 x i32> %307 to <8 x float>, !dbg !10886
  switch i32 %link, label %bb13.i79.i [
    i32 1, label %bb14.i80.i
    i32 3, label %bb14.i80.fold.split.i
  ], !dbg !10894

bb13.i79.i:                                       ; preds = %bb10
  br label %bb14.i80.i, !dbg !10895

bb14.i80.fold.split.i:                            ; preds = %bb10
  br label %bb14.i80.i, !dbg !10896

bb14.i80.i:                                       ; preds = %bb14.i80.fold.split.i, %bb13.i79.i, %bb10
  %_11.i72.sroa.0.0889.i = phi <8 x float> [ %308, %bb10 ], [ %305, %bb13.i79.i ], [ %305, %bb14.i80.fold.split.i ]
  %_13.i71.sroa.0.0.i = phi <8 x i32> [ %307, %bb10 ], [ %307, %bb13.i79.i ], [ %306, %bb14.i80.fold.split.i ], !dbg !10897
  %_4.i92.i = getelementptr inbounds nuw i8, ptr %channel_left, i64 448, !dbg !10898
  %lanes.i307.sroa.0.0.copyload.i = load <8 x float>, ptr %_4.i92.i, align 32, !dbg !10901, !alias.scope !10906, !noalias !10910
  %_7.i93.i = getelementptr inbounds nuw i8, ptr %channel_left, i64 480, !dbg !10916
  %lanes.i302.sroa.0.0.copyload.i = load <8 x float>, ptr %_7.i93.i, align 32, !dbg !10917, !alias.scope !10922, !noalias !10926
  %_15.i1454 = getelementptr inbounds nuw i8, ptr %channel_left, i64 256, !dbg !10930
  %lanes.i297.sroa.0.0.copyload.i = load <8 x float>, ptr %_15.i1454, align 32, !dbg !10931, !alias.scope !10936, !noalias !10940
  %_14.i94.i = getelementptr inbounds nuw i8, ptr %channel_left, i64 288, !dbg !10944
  %lanes.i292.sroa.0.0.copyload.i = load <8 x float>, ptr %_14.i94.i, align 32, !dbg !10945, !alias.scope !10950, !noalias !10954
  %_17.i95.i = getelementptr inbounds nuw i8, ptr %channel_left, i64 320, !dbg !10958
  %lanes.i287.sroa.0.0.copyload.i = load <8 x float>, ptr %_17.i95.i, align 32, !dbg !10959, !alias.scope !10964, !noalias !10968
  %_20.i96.i = getelementptr inbounds nuw i8, ptr %channel_left, i64 352, !dbg !10972
  %lanes.i282.sroa.0.0.copyload.i = load <8 x float>, ptr %_20.i96.i, align 32, !dbg !10973, !alias.scope !10978, !noalias !10982
  %_23.i97.i = getelementptr inbounds nuw i8, ptr %channel_left, i64 384, !dbg !10986
  %lanes.i277.sroa.0.0.copyload.i = load <8 x float>, ptr %_23.i97.i, align 32, !dbg !10987, !alias.scope !10992, !noalias !10996
  %_26.i.i1455 = getelementptr inbounds nuw i8, ptr %channel_left, i64 416, !dbg !11000
  %lanes.i.sroa.0.0.copyload.i = load <8 x float>, ptr %_26.i.i1455, align 32, !dbg !11001, !alias.scope !11006, !noalias !11010
  %309 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i302.sroa.0.0.copyload.i, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !11014
  %310 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i302.sroa.0.0.copyload.i, <8 x float> zeroinitializer, i8 0), !dbg !11020
  %311 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %lanes.i307.sroa.0.0.copyload.i, <8 x float> zeroinitializer, i8 0), !dbg !11026
  %312 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1536, !dbg !11032
  %_17.i1456 = load i32, ptr %312, align 32, !dbg !11032, !alias.scope !10866, !noalias !10876, !noundef !11
  %313 = zext i32 %_17.i1456 to i64, !dbg !11032
; call compressor::kernel::fill_taps::<wide::f32x8_::f32x8>
  tail call fastcc void @_RINvNtCse3bfmKSZS8Y_10compressor6kernel9fill_tapsNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_(ptr noalias noundef nonnull readonly align 32 captures(address, read_provenance) dereferenceable(1568) %channel_left, i64 noundef %313, i64 noundef %_24, ptr noalias noundef nonnull align 4 %_36.0, i64 noundef range(i64 0, 2305843009213693952) %_36.1) #22, !dbg !11034, !noalias !11036
  %_24.i1457 = shl nuw nsw i64 %_24, 3, !dbg !11037
  %_94.not.i = icmp ugt i64 %_24.i1457, %_36.1
  br i1 %_94.not.i, label %bb33.i1484, label %bb31.i1458, !dbg !11039, !prof !239

bb31.i1458:                                       ; preds = %bb14.i80.i
  %_102.not.i = icmp ugt i64 %_24, %_37.1
  br i1 %_102.not.i, label %bb37.i, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_7IterMutNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EECse3bfmKSZS8Y_10compressor.exit.i, !dbg !11048, !prof !239

bb33.i1484:                                       ; preds = %bb14.i80.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_24.i1457, i64 noundef range(i64 0, 2305843009213693952) %_36.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d851e1472101570563b3a14c717a93e2) #23, !dbg !11056, !noalias !11057
  unreachable, !dbg !11056

bb37.i:                                           ; preds = %bb31.i1458
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_24, i64 noundef range(i64 0, 288230376151711744) %_37.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_42c99f388f13882f265c1080ab9aeee2) #23, !dbg !11058, !noalias !11057
  unreachable, !dbg !11058

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_7IterMutNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EECse3bfmKSZS8Y_10compressor.exit.i: ; preds = %bb31.i1458
  %.idx.i = shl nuw nsw i64 %_24, 5, !dbg !11059
  %314 = getelementptr inbounds nuw i8, ptr %_37.0, i64 %.idx.i, !dbg !11059
  %_6.i.i1459 = load i64, ptr %detector, align 8, !range !220, !alias.scope !10864, !noalias !11068
  %315 = getelementptr inbounds nuw i8, ptr %detector, i64 16
  %sidechain_left.1.i.i1460 = load i64, ptr %315, align 8, !alias.scope !10864, !noalias !11068
  %316 = getelementptr inbounds nuw i8, ptr %detector, i64 24
  %sidechain_right.0.i.i1461 = load ptr, ptr %316, align 8, !alias.scope !10864, !noalias !11068, !nonnull !11, !align !3781
  %317 = getelementptr inbounds nuw i8, ptr %detector, i64 32
  %sidechain_right.1.i.i1462 = load i64, ptr %317, align 8, !alias.scope !10864, !noalias !11068
  %318 = getelementptr inbounds nuw i8, ptr %detector, i64 8
  %sidechain_left.0.i.i1463 = load ptr, ptr %318, align 8, !alias.scope !10864, !noalias !11068, !nonnull !11, !align !3781
  %319 = icmp slt <8 x i32> %_13.i71.sroa.0.0.i, zeroinitializer
  %320 = bitcast <8 x float> %_11.i72.sroa.0.0889.i to <8 x i32>
  %321 = icmp slt <8 x i32> %320, zeroinitializer
  %322 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1512
  %_197.1.i = load i64, ptr %322, align 8, !alias.scope !10866, !noalias !10876
  %323 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1504
  %_197.0.i = load ptr, ptr %323, align 32, !alias.scope !10866, !noalias !10876, !nonnull !11
  %324 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1528
  %_198.1.i = load i64, ptr %324, align 8, !alias.scope !10866, !noalias !10876
  %325 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1520
  %_198.0.i = load ptr, ptr %325, align 16, !alias.scope !10866, !noalias !10876, !nonnull !11
  %326 = fneg <8 x float> %lanes.i287.sroa.0.0.copyload.i
  br label %bb9.i1464, !dbg !11069

bb9.i1464:                                        ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_log2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_7IterMutNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EECse3bfmKSZS8Y_10compressor.exit.i
  %head.sroa.0.0994.i = phi i64 [ %313, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_7IterMutNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EECse3bfmKSZS8Y_10compressor.exit.i ], [ %spec.store.select.i1476, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_log2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i ]
  %iter.sroa.12.0993.i = phi i64 [ 0, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_7IterMutNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EECse3bfmKSZS8Y_10compressor.exit.i ], [ %327, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_log2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i ]
  %327 = add nuw nsw i64 %iter.sroa.12.0993.i, 1, !dbg !11077
  %data.i.i.i.i.idx.i = shl i64 %iter.sroa.12.0993.i, 5, !dbg !11079
  %data.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_36.0, i64 %data.i.i.i.i.idx.i, !dbg !11079
  %_3.i.i.i.i.i = getelementptr inbounds nuw %"wide::f32x8_::f32x8", ptr %_37.0, i64 %iter.sroa.12.0993.i, !dbg !11082
  %_35.i1465 = add nuw nsw i64 %iter.sroa.12.0993.i, %spec.store.select, !dbg !11085
  %slot7.i = shl i64 %_35.i1465, 3, !dbg !11085
  %_117.i = icmp samesign ugt i64 %slot7.i, %left.1, !dbg !11087
  br i1 %_117.i, label %bb42.i, label %bb43.i, !dbg !11087, !prof !161

bb10.i1478:                                       ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_log2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i
  %328 = trunc i64 %spec.store.select.i1476 to i32, !dbg !11094
  store i32 %328, ptr %312, align 32, !dbg !11094, !alias.scope !10866, !noalias !10876
  %329 = getelementptr inbounds nuw i8, ptr %channel_left, i64 1472, !dbg !11095
  %gain_left.sroa.0.0.copyload.i = load <8 x float>, ptr %329, align 32, !dbg !11095, !alias.scope !10866, !noalias !10876
  br label %bb58.i, !dbg !11096

bb58.i:                                           ; preds = %bb10.i1478, %bb58.i
  %iter4.sroa.0.0997.i = phi ptr [ %_172.i, %bb58.i ], [ %_37.0, %bb10.i1478 ]
  %gain_left.sroa.0.0996.i = phi <8 x float> [ %343, %bb58.i ], [ %gain_left.sroa.0.0.copyload.i, %bb10.i1478 ]
  %_69.sroa.0.0.copyload.i = load <8 x float>, ptr %iter4.sroa.0.0997.i, align 32, !dbg !11104, !alias.scope !10870, !noalias !11106
  %_172.i = getelementptr inbounds nuw i8, ptr %iter4.sroa.0.0997.i, i64 32, !dbg !11107
  %330 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_69.sroa.0.0.copyload.i, <8 x float> %gain_left.sroa.0.0996.i, i8 17), !dbg !11110
  %331 = bitcast <8 x float> %330 to <8 x i32>, !dbg !11117
  %332 = icmp slt <8 x i32> %331, zeroinitializer, !dbg !11121
  %333 = select <8 x i1> %332, <8 x float> %lanes.i277.sroa.0.0.copyload.i, <8 x float> %lanes.i.sroa.0.0.copyload.i, !dbg !11121
  %334 = fsub <8 x float> %_69.sroa.0.0.copyload.i, %gain_left.sroa.0.0996.i, !dbg !11123
  %335 = fmul <8 x float> %334, %333, !dbg !11129
  %336 = fadd <8 x float> %gain_left.sroa.0.0996.i, %335, !dbg !11134
  %337 = bitcast <8 x float> %336 to <8 x i32>, !dbg !11138
  %338 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %336), !dbg !11144
  %339 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %338, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !11146
  %340 = bitcast <8 x float> %339 to <8 x i32>, !dbg !11152
  %341 = xor <8 x i32> %340, splat (i32 -1), !dbg !11158
  %342 = and <8 x i32> %337, %341, !dbg !11160
  %343 = bitcast <8 x i32> %342 to <8 x float>, !dbg !11164
  store <8 x i32> %342, ptr %iter4.sroa.0.0997.i, align 32, !dbg !11165, !alias.scope !10870, !noalias !11106
  %_166.i = icmp eq ptr %_172.i, %314, !dbg !11166
  br i1 %_166.i, label %bb26.lr.ph.i, label %bb58.i, !dbg !11096

bb26.lr.ph.i:                                     ; preds = %bb58.i
  store <8 x i32> %342, ptr %329, align 32, !dbg !11169, !alias.scope !10866, !noalias !10876
  %344 = bitcast <8 x float> %311 to <8 x i32>
  %345 = bitcast <8 x float> %310 to <8 x i32>
  %346 = select i1 %bypass, <8 x i32> %306, <8 x i32> %307
  %347 = or <8 x i32> %346, %345
  %348 = bitcast <8 x float> %309 to <8 x i32>
  %349 = icmp slt <8 x i32> %348, zeroinitializer
  br label %bb26.i, !dbg !11170

bb26.i:                                           ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i, %bb26.lr.ph.i
  %iter2.sroa.0.01002.i = phi ptr [ %_37.0, %bb26.lr.ph.i ], [ %_16.i.i.i, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i ]
  %iter2.sroa.7.01001.i = phi i64 [ 0, %bb26.lr.ph.i ], [ %_9.0.i547.i, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i ]
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %iter2.sroa.0.01002.i, i64 32, !dbg !11178
  %_9.0.i547.i = add nuw nsw i64 %iter2.sroa.7.01001.i, 1, !dbg !11181
  %_82.i = add nuw nsw i64 %iter2.sroa.7.01001.i, %spec.store.select, !dbg !11184
  %slot.i1479 = shl i64 %_82.i, 3, !dbg !11184
  %_185.i = icmp samesign ugt i64 %slot.i1479, %left.1, !dbg !11186
  br i1 %_185.i, label %bb64.i, label %bb65.i, !dbg !11186, !prof !161

bb65.i:                                           ; preds = %bb26.i
  %_188.i = sub nuw nsw i64 %left.1, %slot.i1479, !dbg !11191
  %_8.i354.i = icmp samesign ugt i64 %_188.i, 7, !dbg !11192
  br i1 %_8.i354.i, label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i, label %bb2.i355.i, !dbg !11192, !prof !2135

bb2.i355.i:                                       ; preds = %bb65.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_188.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !11197, !noalias !11198
  unreachable, !dbg !11197

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i: ; preds = %bb65.i
  %_192.i = getelementptr inbounds nuw float, ptr %left.0, i64 %slot.i1479, !dbg !11202
  %_87.sroa.0.0.copyload.i = load <8 x float>, ptr %iter2.sroa.0.01002.i, align 32, !dbg !11208, !alias.scope !10870, !noalias !11106
  %350 = fadd <8 x float> %lanes.i307.sroa.0.0.copyload.i, %_87.sroa.0.0.copyload.i, !dbg !11210
  %351 = fmul <8 x float> %350, splat (float 0x3FC542A5A0000000), !dbg !11216
  %352 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %351, <8 x float> splat (float -1.260000e+02)), !dbg !11222
  %353 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %352, <8 x float> splat (float 1.270000e+02)), !dbg !11228
  %354 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %353), !dbg !11233
  %355 = fsub <8 x float> %353, %354, !dbg !11238
  %356 = fmul <8 x float> %355, splat (float 0x3F5E974FA0000000), !dbg !11243
  %357 = fadd <8 x float> %356, splat (float 0x3F82778560000000), !dbg !11248
  %358 = fmul <8 x float> %355, %357, !dbg !11243
  %359 = fadd <8 x float> %358, splat (float 0x3FAC91CE60000000), !dbg !11248
  %360 = fmul <8 x float> %355, %359, !dbg !11243
  %361 = fadd <8 x float> %360, splat (float 0x3FCEBDB560000000), !dbg !11248
  %362 = fmul <8 x float> %355, %361, !dbg !11243
  %363 = fadd <8 x float> %362, splat (float 0x3FE62E4BA0000000), !dbg !11248
  %lanes.i351.sroa.0.0.copyload.i = load <8 x float>, ptr %_192.i, align 4, !dbg !11253, !alias.scope !11257, !noalias !11261
  %364 = fmul <8 x float> %355, %363, !dbg !11263
  %365 = fadd <8 x float> %364, splat (float 1.000000e+00), !dbg !11268
  %366 = fadd <8 x float> %354, splat (float 0x4160000FE0000000), !dbg !11273
  %367 = bitcast <8 x float> %366 to <8 x i32>, !dbg !11278
  %_3.i548.i = shl <8 x i32> %367, splat (i32 23), !dbg !11282
  %368 = bitcast <8 x i32> %_3.i548.i to <8 x float>, !dbg !11283
  %369 = fmul <8 x float> %365, %368, !dbg !11285
  %370 = fmul <8 x float> %lanes.i351.sroa.0.0.copyload.i, %369, !dbg !11289
  %371 = fsub <8 x float> %370, %lanes.i351.sroa.0.0.copyload.i, !dbg !11294
  %372 = fmul <8 x float> %lanes.i302.sroa.0.0.copyload.i, %371, !dbg !11300
  %373 = fadd <8 x float> %lanes.i351.sroa.0.0.copyload.i, %372, !dbg !11305
  %374 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_87.sroa.0.0.copyload.i, <8 x float> zeroinitializer, i8 0), !dbg !11309
  %375 = bitcast <8 x float> %374 to <8 x i32>, !dbg !11315
  %376 = and <8 x i32> %375, %344, !dbg !11319
  %377 = or <8 x i32> %347, %376, !dbg !11321
  %378 = select <8 x i1> %349, <8 x float> %370, <8 x float> %373, !dbg !11326
  %379 = icmp slt <8 x i32> %377, zeroinitializer, !dbg !11331
  %380 = select <8 x i1> %379, <8 x float> %lanes.i351.sroa.0.0.copyload.i, <8 x float> %378, !dbg !11331
  store <8 x float> %380, ptr %_192.i, align 4, !dbg !11336, !alias.scope !11341, !noalias !11345
  %_6.i.i.i = icmp eq ptr %_16.i.i.i, %314, !dbg !11349
  br i1 %_6.i.i.i, label %bb16, label %bb26.i, !dbg !11170

bb64.i:                                           ; preds = %bb26.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %slot.i1479, i64 noundef range(i64 8, 34359738361) %left.1, i64 noundef range(i64 8, 34359738361) %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_91aa884084556021633da71af92442da) #23, !dbg !11352, !noalias !11057
  unreachable, !dbg !11352

bb43.i:                                           ; preds = %bb9.i1464
  %_120.i = sub nuw nsw i64 %left.1, %slot7.i, !dbg !11353
  %_124.i = getelementptr inbounds nuw float, ptr %left.0, i64 %slot7.i, !dbg !11354
  %_8.i346.i = icmp samesign ugt i64 %_120.i, 7, !dbg !11358
  br i1 %_8.i346.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit349.i, label %bb2.i347.i, !dbg !11358, !prof !2135

bb2.i347.i:                                       ; preds = %bb43.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_120.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !11363, !noalias !11364
  unreachable, !dbg !11363

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit349.i: ; preds = %bb43.i
  %lanes.i343.sroa.0.0.copyload.i = load <8 x float>, ptr %_124.i, align 4, !dbg !11368, !alias.scope !11372, !noalias !11376
  switch i64 %_6.i.i1459, label %default.unreachable [
    i64 0, label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i1475
    i64 1, label %bb3.i.i1483
    i64 2, label %bb2.i.i1466
  ], !dbg !11378

bb3.i.i1483:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit349.i
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i1475, !dbg !11381

bb2.i.i1466:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit349.i
  %_25.i.i1467 = icmp ugt i64 %slot7.i, %sidechain_left.1.i.i1460, !dbg !11382
  br i1 %_25.i.i1467, label %bb17.i.i1482, label %bb18.i.i1468, !dbg !11382, !prof !161

bb18.i.i1468:                                     ; preds = %bb2.i.i1466
  %_28.i.i1469 = sub nuw i64 %sidechain_left.1.i.i1460, %slot7.i, !dbg !11385
  %_8.i322.i = icmp samesign ugt i64 %_28.i.i1469, 7, !dbg !11386
  br i1 %_8.i322.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit325.i, label %bb2.i323.i, !dbg !11386, !prof !2135

bb2.i323.i:                                       ; preds = %bb18.i.i1468
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_28.i.i1469, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !11391, !noalias !11392
  unreachable, !dbg !11391

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit325.i: ; preds = %bb18.i.i1468
  %_32.i.i1470 = getelementptr inbounds nuw float, ptr %sidechain_left.0.i.i1463, i64 %slot7.i, !dbg !11401
  %lanes.i319.sroa.0.0.copyload.i = load <8 x float>, ptr %_32.i.i1470, align 4, !dbg !11403, !alias.scope !11407, !noalias !11411
  %_33.i.i1471 = icmp ugt i64 %slot7.i, %sidechain_right.1.i.i1462, !dbg !11413
  br i1 %_33.i.i1471, label %bb19.i.i1481, label %bb20.i.i1472, !dbg !11413, !prof !161

bb17.i.i1482:                                     ; preds = %bb2.i.i1466
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 34359738361) %slot7.i, i64 noundef %sidechain_left.1.i.i1460, i64 noundef %sidechain_left.1.i.i1460, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7fe5a91ec575deda110c046c84630100) #23, !dbg !11416, !noalias !11417
  unreachable, !dbg !11416

bb20.i.i1472:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit325.i
  %_36.i.i1473 = sub nuw i64 %sidechain_right.1.i.i1462, %slot7.i, !dbg !11419
  %_8.i314.i = icmp samesign ugt i64 %_36.i.i1473, 7, !dbg !11420
  br i1 %_8.i314.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit317.i, label %bb2.i315.i, !dbg !11420, !prof !2135

bb2.i315.i:                                       ; preds = %bb20.i.i1472
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_36.i.i1473, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !11425, !noalias !11426
  unreachable, !dbg !11425

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit317.i: ; preds = %bb20.i.i1472
  %_40.i.i1474 = getelementptr inbounds nuw float, ptr %sidechain_right.0.i.i1461, i64 %slot7.i, !dbg !11430
  %lanes.i312.sroa.0.0.copyload.i = load <8 x float>, ptr %_40.i.i1474, align 4, !dbg !11432, !alias.scope !11436, !noalias !11440
  br label %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i1475, !dbg !11442

bb19.i.i1481:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit325.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 34359738361) %slot7.i, i64 noundef %sidechain_right.1.i.i1462, i64 noundef %sidechain_right.1.i.i1462, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f2cba1f89421a726339097d2eba48536) #23, !dbg !11443, !noalias !11417
  unreachable, !dbg !11443

_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i1475: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit317.i, %bb3.i.i1483, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit349.i
  %.sroa.0.0.i = phi <8 x float> [ %lanes.i343.sroa.0.0.copyload.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit349.i ], [ zeroinitializer, %bb3.i.i1483 ], [ %lanes.i319.sroa.0.0.copyload.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit317.i ], !dbg !11444
  %.sroa.0566.0.i = phi <8 x float> [ %lanes.i343.sroa.0.0.copyload.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit349.i ], [ zeroinitializer, %bb3.i.i1483 ], [ %lanes.i312.sroa.0.0.copyload.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit317.i ], !dbg !11444
  %381 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %.sroa.0.0.i), !dbg !11445
  %382 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %.sroa.0566.0.i), !dbg !11451
  %383 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %381, <8 x float> %382), !dbg !11457
  %384 = fmul <8 x float> %381, splat (float 5.000000e-01), !dbg !11462
  %385 = fmul <8 x float> %382, splat (float 5.000000e-01), !dbg !11467
  %386 = fadd <8 x float> %384, %385, !dbg !11472
  %387 = select <8 x i1> %319, <8 x float> %386, <8 x float> %383, !dbg !11477
  %388 = select <8 x i1> %321, <8 x float> %387, <8 x float> %381, !dbg !11482
  %389 = add i64 %head.sroa.0.0994.i, 1, !dbg !11487
  %_44.i = icmp eq i64 %389, %ring_length.i1453, !dbg !11489
  %spec.store.select.i1476 = select i1 %_44.i, i64 0, i64 %389, !dbg !11489
  %_48.i = shl i64 %head.sroa.0.0994.i, 3, !dbg !11491
  %_128.i = icmp ugt i64 %_48.i, %_197.1.i, !dbg !11493
  br i1 %_128.i, label %bb44.i, label %bb45.i, !dbg !11493, !prof !161

bb42.i:                                           ; preds = %bb9.i1464
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %slot7.i, i64 noundef range(i64 8, 34359738361) %left.1, i64 noundef range(i64 8, 34359738361) %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4f540afd076612488ef0f518fdb4bc49) #23, !dbg !11499, !noalias !11057
  unreachable, !dbg !11499

bb45.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i1475
  %_131.i = sub nuw i64 %_197.1.i, %_48.i, !dbg !11500
  %_8.i487.i = icmp samesign ugt i64 %_131.i, 7, !dbg !11501
  br i1 %_8.i487.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit489.i, label %bb2.i488.i, !dbg !11501, !prof !2135

bb2.i488.i:                                       ; preds = %bb45.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_131.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !11506, !noalias !11507
  unreachable, !dbg !11506

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit489.i: ; preds = %bb45.i
  %_135.i = getelementptr inbounds nuw float, ptr %_197.0.i, i64 %_48.i, !dbg !11511
  store <8 x float> %lanes.i343.sroa.0.0.copyload.i, ptr %_135.i, align 4, !dbg !11516, !alias.scope !11520, !noalias !11524
  %_136.i = icmp ugt i64 %_48.i, %_198.1.i, !dbg !11526
  br i1 %_136.i, label %bb46.i, label %bb47.i, !dbg !11526, !prof !161

bb44.i:                                           ; preds = %_RINvNtCse3bfmKSZS8Y_10compressor6kernel10link_frameNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_.exit.i1475
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_48.i, i64 noundef %_197.1.i, i64 noundef %_197.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_fb598d860754a178f92db37ff597f6f5) #23, !dbg !11530, !noalias !11057
  unreachable, !dbg !11530

bb47.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit489.i
  %_139.i = sub nuw i64 %_198.1.i, %_48.i, !dbg !11531
  %_8.i483.i = icmp samesign ugt i64 %_139.i, 7, !dbg !11532
  br i1 %_8.i483.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit485.i, label %bb2.i484.i, !dbg !11532, !prof !2135

bb2.i484.i:                                       ; preds = %bb47.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_139.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !11537, !noalias !11538
  unreachable, !dbg !11537

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit485.i: ; preds = %bb47.i
  %_143.i = getelementptr inbounds nuw float, ptr %_198.0.i, i64 %_48.i, !dbg !11542
  store <8 x float> %388, ptr %_143.i, align 4, !dbg !11547, !alias.scope !11551, !noalias !11555
  %_57.i1477 = shl i64 %spec.store.select.i1476, 3, !dbg !11557
  %_144.i = icmp ugt i64 %_57.i1477, %_197.1.i, !dbg !11558
  br i1 %_144.i, label %bb48.i, label %bb49.i, !dbg !11558, !prof !161

bb46.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit489.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_48.i, i64 noundef %_198.1.i, i64 noundef %_198.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b5f6c5fe8b00b28a9ec3012fadf9414a) #23, !dbg !11562, !noalias !11057
  unreachable, !dbg !11562

bb49.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit485.i
  %_147.i = sub nuw i64 %_197.1.i, %_57.i1477, !dbg !11563
  %_8.i338.i = icmp samesign ugt i64 %_147.i, 7, !dbg !11564
  br i1 %_8.i338.i, label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_log2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i, label %bb2.i339.i, !dbg !11564, !prof !2135

bb2.i339.i:                                       ; preds = %bb49.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_147.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #23, !dbg !11569, !noalias !11570
  unreachable, !dbg !11569

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_log2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECse3bfmKSZS8Y_10compressor.exit.i: ; preds = %bb49.i
  %_151.i = getelementptr inbounds nuw float, ptr %_197.0.i, i64 %_57.i1477, !dbg !11574
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_124.i, ptr noundef nonnull align 4 dereferenceable(32) %_151.i, i64 32, i1 false), !dbg !11579, !noalias !11584
  %lanes.i327.sroa.0.0.copyload.i = load <8 x float>, ptr %data.i.i.i.i.i, align 4, !dbg !11585, !alias.scope !11590, !noalias !11594
  %390 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i327.sroa.0.0.copyload.i, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !11598
  %391 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %390, <8 x float> splat (float 0x3810000000000000)), !dbg !11604
  %392 = bitcast <8 x float> %391 to <4 x i64>, !dbg !11611
  %393 = and <4 x i64> %392, splat (i64 36028792732385279), !dbg !11612
  %394 = or disjoint <4 x i64> %393, splat (i64 4575657222473777152), !dbg !11617
  %395 = bitcast <4 x i64> %394 to <8 x float>, !dbg !11621
  %396 = fadd <8 x float> %395, splat (float -1.000000e+00), !dbg !11622
  %397 = fmul <8 x float> %396, splat (float 0x3F9B17A960000000), !dbg !11627
  %398 = fsub <8 x float> splat (float 0x3FBF9A8440000000), %397, !dbg !11632
  %399 = fmul <8 x float> %396, %398, !dbg !11627
  %400 = fadd <8 x float> %399, splat (float 0xBFD1E3F400000000), !dbg !11632
  %401 = fmul <8 x float> %396, %400, !dbg !11627
  %402 = fadd <8 x float> %401, splat (float 0x3FDD544F20000000), !dbg !11632
  %403 = fmul <8 x float> %396, %402, !dbg !11627
  %404 = fadd <8 x float> %403, splat (float 0xBFE6FC2A60000000), !dbg !11632
  %405 = fmul <8 x float> %396, %404, !dbg !11627
  %406 = fadd <8 x float> %405, splat (float 0x3FF714B2A0000000), !dbg !11632
  %407 = bitcast <8 x float> %391 to <8 x i32>, !dbg !11637
  %_3.i549.i = lshr <8 x i32> %407, splat (i32 23), !dbg !11641
  %408 = or disjoint <8 x i32> %_3.i549.i, splat (i32 1258291200), !dbg !11642
  %409 = bitcast <8 x i32> %408 to <8 x float>, !dbg !11646
  %410 = fadd <8 x float> %409, splat (float 0xC160000FE0000000), !dbg !11647
  %411 = fmul <8 x float> %396, %406, !dbg !11651
  %412 = fadd <8 x float> %410, %411, !dbg !11656
  %413 = fmul <8 x float> %412, splat (float 0x4018151820000000), !dbg !11661
  %414 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %413, <8 x float> splat (float -1.600000e+02)), !dbg !11666
  %415 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %414, <8 x float> splat (float 2.400000e+01)), !dbg !11671
  %416 = fsub <8 x float> %415, %lanes.i297.sroa.0.0.copyload.i, !dbg !11676
  %417 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %416, <8 x float> %lanes.i287.sroa.0.0.copyload.i, i8 30), !dbg !11682
  %418 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %416, <8 x float> %326, i8 18), !dbg !11688
  %419 = fadd <8 x float> %lanes.i287.sroa.0.0.copyload.i, %416, !dbg !11694
  %420 = fmul <8 x float> %419, %419, !dbg !11699
  %421 = fmul <8 x float> %lanes.i282.sroa.0.0.copyload.i, %420, !dbg !11704
  %422 = bitcast <8 x float> %417 to <8 x i32>, !dbg !11709
  %423 = icmp slt <8 x i32> %422, zeroinitializer, !dbg !11713
  %.v.i = select <8 x i1> %423, <8 x float> %416, <8 x float> %421, !dbg !11713
  %424 = fmul <8 x float> %lanes.i292.sroa.0.0.copyload.i, %.v.i, !dbg !11713
  %425 = bitcast <8 x float> %418 to <8 x i32>, !dbg !11715
  %426 = icmp slt <8 x i32> %425, zeroinitializer, !dbg !11719
  %427 = select <8 x i1> %426, <8 x float> zeroinitializer, <8 x float> %424, !dbg !11719
  %428 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %427, <8 x float> splat (float -1.000000e+02)), !dbg !11721
  %429 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %428, <8 x float> zeroinitializer), !dbg !11726
  store <8 x float> %429, ptr %_3.i.i.i.i.i, align 32, !dbg !11731, !alias.scope !10870, !noalias !11106
  %exitcond.not.i = icmp eq i64 %327, %_24, !dbg !11069
  br i1 %exitcond.not.i, label %bb10.i1478, label %bb9.i1464, !dbg !11069

bb48.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit485.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_57.i1477, i64 noundef %_197.1.i, i64 noundef %_197.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ba97830f78f889c3a4fb287f709b1dd7) #23, !dbg !11732, !noalias !11057
  unreachable, !dbg !11732
}

define internal fastcc void @_RINvNtCse3bfmKSZS8Y_10compressor6kernel9fill_tapsNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB4_(ptr noalias noundef nonnull readonly align 32 captures(none) dereferenceable(1568) %channel, i64 noundef range(i64 0, 4294967296) %write, i64 noundef range(i64 -4294967294, 4294967296) %len, ptr noalias noundef nonnull writeonly align 4 captures(none) %scratch.0, i64 noundef range(i64 0, 2305843009213693952) %scratch.1) unnamed_addr #1 personality ptr @rust_eh_personality !dbg !11733 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %channel, i64 1540, !dbg !11734
  %_6 = load i32, ptr %0, align 4, !dbg !11734, !noundef !11
  %ring_length = zext i32 %_6 to i64, !dbg !11734
  %_8 = shl nsw i64 %len, 3, !dbg !11736
  %_58.not = icmp ugt i64 %_8, %scratch.1
  br i1 %_58.not, label %bb16, label %bb14, !dbg !11738, !prof !239

bb14:                                             ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %channel, i64 1440, !dbg !11747
  %_67 = load i32, ptr %1, align 32, !dbg !11747, !noundef !11
  %2 = getelementptr inbounds nuw i8, ptr %channel, i64 1444, !dbg !11752
  %_70 = load i32, ptr %2, align 4, !dbg !11752, !noundef !11
  %_68.not = icmp eq i32 %_70, %_67, !dbg !11752
  %3 = getelementptr inbounds nuw i8, ptr %channel, i64 1448, !dbg !11752
  %_70.1 = load i32, ptr %3, align 4, !dbg !11752
  %_68.not.1 = icmp eq i32 %_70.1, %_67, !dbg !11752
  %or.cond143 = select i1 %_68.not, i1 %_68.not.1, i1 false, !dbg !11752
  %4 = getelementptr inbounds nuw i8, ptr %channel, i64 1452, !dbg !11752
  %_70.2 = load i32, ptr %4, align 4, !dbg !11752
  %_68.not.2 = icmp eq i32 %_70.2, %_67, !dbg !11752
  %or.cond144 = select i1 %or.cond143, i1 %_68.not.2, i1 false, !dbg !11752
  %5 = getelementptr inbounds nuw i8, ptr %channel, i64 1456, !dbg !11752
  %_70.3 = load i32, ptr %5, align 4, !dbg !11752
  %_68.not.3 = icmp eq i32 %_70.3, %_67, !dbg !11752
  %or.cond145 = select i1 %or.cond144, i1 %_68.not.3, i1 false, !dbg !11752
  %6 = getelementptr inbounds nuw i8, ptr %channel, i64 1460, !dbg !11752
  %_70.4 = load i32, ptr %6, align 4, !dbg !11752
  %_68.not.4 = icmp eq i32 %_70.4, %_67, !dbg !11752
  %or.cond146 = select i1 %or.cond145, i1 %_68.not.4, i1 false, !dbg !11752
  %7 = getelementptr inbounds nuw i8, ptr %channel, i64 1464, !dbg !11752
  %_70.5 = load i32, ptr %7, align 4, !dbg !11752
  %_68.not.5 = icmp eq i32 %_70.5, %_67, !dbg !11752
  %or.cond147 = select i1 %or.cond146, i1 %_68.not.5, i1 false, !dbg !11752
  %8 = getelementptr inbounds nuw i8, ptr %channel, i64 1468, !dbg !11752
  %_70.6 = load i32, ptr %8, align 4, !dbg !11752
  %_68.not.6 = icmp eq i32 %_70.6, %_67, !dbg !11752
  %or.cond148 = select i1 %or.cond147, i1 %_68.not.6, i1 false, !dbg !11752
  br i1 %or.cond148, label %bb25, label %bb5.preheader, !dbg !11752

bb16:                                             ; preds = %start
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_8, i64 noundef %scratch.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_706ebfc02db290a50140e9081d0f8b39) #23, !dbg !11756
  unreachable, !dbg !11756

bb25:                                             ; preds = %bb14
  %_66 = zext i32 %_67 to i64, !dbg !11747
  %_12.not = icmp samesign ult i64 %write, %_66, !dbg !11757
  %_13 = select i1 %_12.not, i64 %ring_length, i64 0, !dbg !11757
  %write.pn = sub nsw i64 %write, %_66, !dbg !11757
  %row.sroa.0.0 = add nsw i64 %write.pn, %_13, !dbg !11758
  %_16 = sub nsw i64 %ring_length, %row.sroa.0.0, !dbg !11759
  %..i = tail call noundef i64 @llvm.umin.i64(i64 %len, i64 %_16), !dbg !11761
  %first = shl nsw i64 %..i, 3, !dbg !11759
  %9 = getelementptr inbounds nuw i8, ptr %channel, i64 1520, !dbg !11763
  %_143.0 = load ptr, ptr %9, align 16, !dbg !11763, !nonnull !11, !noundef !11
  %10 = getelementptr inbounds nuw i8, ptr %channel, i64 1528, !dbg !11763
  %_143.1 = load i64, ptr %10, align 8, !dbg !11763, !noundef !11
  %_21 = shl nsw i64 %row.sroa.0.0, 3, !dbg !11765
  %_23 = add nsw i64 %first, %_21, !dbg !11766
  %_88 = icmp ult i64 %_23, %_21, !dbg !11767
  %_82.not = icmp ugt i64 %_23, %_143.1
  %or.cond = or i1 %_88, %_82.not, !dbg !11767
  br i1 %or.cond, label %bb31, label %bb35, !dbg !11767, !prof !239

bb31:                                             ; preds = %bb25
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_21, i64 noundef %_23, i64 noundef %_143.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_49704929b3df5dee298aef29445db88f) #23, !dbg !11773
  unreachable, !dbg !11773

bb35:                                             ; preds = %bb25
  %_91 = getelementptr inbounds nuw float, ptr %_143.0, i64 %_21, !dbg !11774
  %11 = shl nsw i64 %..i, 5, !dbg !11778
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 4 %scratch.0, ptr nonnull readonly align 4 %_91, i64 %11, i1 false), !dbg !11778, !alias.scope !11783, !noalias !11787
  %rest = sub nsw i64 %_8, %first, !dbg !11789
  %_100.not = icmp ugt i64 %rest, %_143.1
  br i1 %_100.not, label %bb41, label %bb39, !dbg !11790, !prof !239

bb39:                                             ; preds = %bb35
  %_99 = getelementptr inbounds nuw float, ptr %scratch.0, i64 %first, !dbg !11800
  %12 = shl nuw nsw i64 %rest, 2, !dbg !11809
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 4 %_99, ptr nonnull readonly align 4 %_143.0, i64 %12, i1 false), !dbg !11809, !alias.scope !11813, !noalias !11817
  br label %bb13, !dbg !11819

bb41:                                             ; preds = %bb35
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %rest, i64 noundef %_143.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c5f07f7510c4245a135cf142a70cfe98) #23, !dbg !11820
  unreachable, !dbg !11820

bb13:                                             ; preds = %bb5.loopexit, %bb39
  ret void, !dbg !11819

bb5.preheader:                                    ; preds = %bb14
  %13 = getelementptr inbounds nuw i8, ptr %channel, i64 1520
  %_145.0 = load ptr, ptr %13, align 16, !nonnull !11, !noundef !11
  %14 = getelementptr inbounds nuw i8, ptr %channel, i64 1528
  %_145.1 = load i64, ptr %14, align 8, !noundef !11
  br label %bb45, !dbg !11821

bb5.loopexit:                                     ; preds = %bb9.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter14ChunksExactMutfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_11ChunksExactfEECse3bfmKSZS8Y_10compressor.exit68
  %exitcond137.not = icmp eq i64 %15, 8, !dbg !11829
  br i1 %exitcond137.not, label %bb13, label %bb45, !dbg !11821

bb45:                                             ; preds = %bb5.preheader, %bb5.loopexit
  %iter.sroa.0.0126 = phi i64 [ 0, %bb5.preheader ], [ %15, %bb5.loopexit ]
  %15 = add nuw nsw i64 %iter.sroa.0.0126, 1, !dbg !11833
  %16 = getelementptr inbounds nuw i32, ptr %1, i64 %iter.sroa.0.0126, !dbg !11841
  %_33 = load i32, ptr %16, align 4, !dbg !11841, !noundef !11
  %delay3 = zext i32 %_33 to i64, !dbg !11841
  %_36.not = icmp samesign ult i64 %write, %delay3, !dbg !11843
  %_37 = select i1 %_36.not, i64 %ring_length, i64 0, !dbg !11843
  %write.pn15 = sub nsw i64 %write, %delay3, !dbg !11843
  %row1.sroa.0.0 = add nsw i64 %write.pn15, %_37, !dbg !11845
  %_39 = sub nsw i64 %ring_length, %row1.sroa.0.0, !dbg !11846
  %..i35 = tail call noundef i64 @llvm.umin.i64(i64 %len, i64 %_39), !dbg !11848
  %_43 = shl nsw i64 %row1.sroa.0.0, 3, !dbg !11850
  %_46 = add nsw i64 %..i35, %row1.sroa.0.0, !dbg !11852
  %_45 = shl nsw i64 %_46, 3, !dbg !11852
  %_118 = icmp ult i64 %_45, %_43, !dbg !11853
  br i1 %_118, label %bb49, label %bb51, !dbg !11853, !prof !161

bb51:                                             ; preds = %bb45
  %_119 = shl nsw i64 %..i35, 3, !dbg !11859
  %_112.not = icmp ugt i64 %_45, %_145.1, !dbg !11860
  br i1 %_112.not, label %bb49, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter14ChunksExactMutfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_11ChunksExactfEECse3bfmKSZS8Y_10compressor.exit, !dbg !11860, !prof !161

bb49:                                             ; preds = %bb51, %bb45
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_43, i64 noundef %_45, i64 noundef %_145.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b4e7fbc9d705c950adfbded99f17716a) #23, !dbg !11861
  unreachable, !dbg !11861

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter14ChunksExactMutfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_11ChunksExactfEECse3bfmKSZS8Y_10compressor.exit: ; preds = %bb51
  %_121 = getelementptr inbounds nuw float, ptr %_145.0, i64 %_43, !dbg !11862
  %n.i.i.i.i105 = and i64 %..i35, 4294967295, !dbg !11865
  %invariant.gep = getelementptr float, ptr %_121, i64 %iter.sroa.0.0126, !dbg !11885
  %invariant.gep116 = getelementptr float, ptr %scratch.0, i64 %iter.sroa.0.0126, !dbg !11885
  %_2.i118.not = icmp eq i64 %n.i.i.i.i105, 0, !dbg !11887
  br i1 %_2.i118.not, label %_RNvNtCse3bfmKSZS8Y_10compressor6kernel9copy_lane.exit34, label %bb9.i31, !dbg !11887

bb9.i31:                                          ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter14ChunksExactMutfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_11ChunksExactfEECse3bfmKSZS8Y_10compressor.exit, %bb9.i31
  %iter.i17.sroa.16.0119 = phi i64 [ %17, %bb9.i31 ], [ 0, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter14ChunksExactMutfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_11ChunksExactfEECse3bfmKSZS8Y_10compressor.exit ]
  %17 = add nuw nsw i64 %iter.i17.sroa.16.0119, 1, !dbg !11893
  %start1.i.i = shl i64 %iter.i17.sroa.16.0119, 3, !dbg !11895
  %gep = getelementptr float, ptr %invariant.gep, i64 %start1.i.i, !dbg !11898
  %_14.i33 = load float, ptr %gep, align 4, !dbg !11898, !noundef !11
  %gep117 = getelementptr float, ptr %invariant.gep116, i64 %start1.i.i, !dbg !11900
  store float %_14.i33, ptr %gep117, align 4, !dbg !11900
  %exitcond.not = icmp eq i64 %17, %n.i.i.i.i105, !dbg !11887
  br i1 %exitcond.not, label %_RNvNtCse3bfmKSZS8Y_10compressor6kernel9copy_lane.exit34, label %bb9.i31, !dbg !11887

_RNvNtCse3bfmKSZS8Y_10compressor6kernel9copy_lane.exit34: ; preds = %bb9.i31, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter14ChunksExactMutfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_11ChunksExactfEECse3bfmKSZS8Y_10compressor.exit
  %_52 = sub nsw i64 %len, %..i35, !dbg !11901
  %_51 = shl nsw i64 %_52, 3, !dbg !11901
  %_128.not = icmp ugt i64 %_51, %_145.1
  br i1 %_128.not, label %bb58, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter14ChunksExactMutfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_11ChunksExactfEECse3bfmKSZS8Y_10compressor.exit68, !dbg !11902, !prof !239

bb58:                                             ; preds = %_RNvNtCse3bfmKSZS8Y_10compressor6kernel9copy_lane.exit34
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_51, i64 noundef %_145.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_08bdeba311673054f2790786a49cc860) #23, !dbg !11910
  unreachable, !dbg !11910

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter14ChunksExactMutfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_11ChunksExactfEECse3bfmKSZS8Y_10compressor.exit68: ; preds = %_RNvNtCse3bfmKSZS8Y_10compressor6kernel9copy_lane.exit34
  %_138 = sub nuw nsw i64 %_8, %_119, !dbg !11911
  %_142 = getelementptr inbounds nuw float, ptr %scratch.0, i64 %_119, !dbg !11917
  %fst_len.i39 = lshr exact i64 %_138, 3, !dbg !11921
  %n.i.i3.i.i59108 = and i64 %_52, 2305843009213693951, !dbg !11928
  %..i.i.i60 = tail call noundef i64 @llvm.umin.i64(i64 %n.i.i3.i.i59108, i64 %fst_len.i39), !dbg !11933
  %invariant.gep120 = getelementptr float, ptr %_145.0, i64 %iter.sroa.0.0126, !dbg !11937
  %invariant.gep122 = getelementptr float, ptr %_142, i64 %iter.sroa.0.0126, !dbg !11937
  %_2.i71124.not = icmp eq i64 %..i.i.i60, 0, !dbg !11938
  br i1 %_2.i71124.not, label %bb5.loopexit, label %bb9.i, !dbg !11938

bb9.i:                                            ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter14ChunksExactMutfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_11ChunksExactfEECse3bfmKSZS8Y_10compressor.exit68, %bb9.i
  %iter.i.sroa.16.0125 = phi i64 [ %18, %bb9.i ], [ 0, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter14ChunksExactMutfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_11ChunksExactfEECse3bfmKSZS8Y_10compressor.exit68 ]
  %18 = add nuw nsw i64 %iter.i.sroa.16.0125, 1, !dbg !11941
  %start1.i.i76 = shl i64 %iter.i.sroa.16.0125, 3, !dbg !11942
  %gep121 = getelementptr float, ptr %invariant.gep120, i64 %start1.i.i76, !dbg !11944
  %_14.i = load float, ptr %gep121, align 4, !dbg !11944, !noundef !11
  %gep123 = getelementptr float, ptr %invariant.gep122, i64 %start1.i.i76, !dbg !11945
  store float %_14.i, ptr %gep123, align 4, !dbg !11945
  %exitcond136.not = icmp eq i64 %18, %..i.i.i60, !dbg !11938
  br i1 %exitcond136.not, label %bb5.loopexit, label %bb9.i, !dbg !11938
}

