define noundef zeroext i1 @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef readonly align 8 captures(none) dereferenceable(200) %self) unnamed_addr #5 personality ptr @rust_eh_personality !dbg !13125 {
start:
  %_46.0 = load ptr, ptr %self, align 8, !dbg !13126, !nonnull !12, !noundef !12
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8, !dbg !13126
  %_46.1 = load i64, ptr %0, align 8, !dbg !13126, !noundef !12
  br label %bb1.i, !dbg !13127

bb1.i:                                            ; preds = %bb10.i, %start
  %iter.sroa.6.0.i = phi i64 [ %_46.1, %start ], [ %len.i.i.i.i, %bb10.i ], !dbg !13129
  %iter.sroa.0.0.i = phi ptr [ %_46.0, %start ], [ %data.i.i.i.i, %bb10.i ], !dbg !13129
  %1 = icmp eq i64 %iter.sroa.6.0.i, 0, !dbg !13130
  br i1 %1, label %bb2, label %bb11.preheader.i, !dbg !13130

bb11.preheader.i:                                 ; preds = %bb1.i
  %..i.i.i = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i, i64 32), !dbg !13132
  %_18.idx.i = shl nuw nsw i64 %..i.i.i, 2, !dbg !13135
  %_18.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i, i64 %_18.idx.i, !dbg !13135
  br label %bb11.i, !dbg !13140

bb11.i:                                           ; preds = %bb11.i, %bb11.preheader.i
  %iter1.sroa.0.014.i = phi ptr [ %_31.i, %bb11.i ], [ %iter.sroa.0.0.i, %bb11.preheader.i ]
  %bits.sroa.0.013.i = phi i32 [ %2, %bb11.i ], [ 0, %bb11.preheader.i ]
  %_31.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i, i64 4, !dbg !13142
  %_134.i = load i32, ptr %iter1.sroa.0.014.i, align 4, !dbg !13144, !alias.scope !13145, !noundef !12
  %2 = or i32 %_134.i, %bits.sroa.0.013.i, !dbg !13148
  %_25.i = icmp eq ptr %_31.i, %_18.i, !dbg !13149
  br i1 %_25.i, label %bb10.i, label %bb11.i, !dbg !13140

bb10.i:                                           ; preds = %bb11.i
  %data.i.i.i.i = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i, i64 %..i.i.i, !dbg !13151
  %len.i.i.i.i = sub nuw nsw i64 %iter.sroa.6.0.i, %..i.i.i, !dbg !13156
  %3 = icmp eq i32 %2, 0, !dbg !13157
  br i1 %3, label %bb1.i, label %bb21, !dbg !13157

bb2:                                              ; preds = %bb1.i
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 16, !dbg !13158
  %_47.0 = load ptr, ptr %4, align 8, !dbg !13158, !nonnull !12, !noundef !12
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 24, !dbg !13158
  %_47.1 = load i64, ptr %5, align 8, !dbg !13158, !noundef !12
  br label %bb1.i1, !dbg !13159

bb1.i1:                                           ; preds = %bb10.i16, %bb2
  %iter.sroa.6.0.i2 = phi i64 [ %_47.1, %bb2 ], [ %len.i.i.i.i7, %bb10.i16 ], !dbg !13161
  %iter.sroa.0.0.i3 = phi ptr [ %_47.0, %bb2 ], [ %data.i.i.i.i6, %bb10.i16 ], !dbg !13161
  %6 = icmp eq i64 %iter.sroa.6.0.i2, 0, !dbg !13162
  br i1 %6, label %bb4, label %bb11.preheader.i4, !dbg !13162

bb11.preheader.i4:                                ; preds = %bb1.i1
  %..i.i.i5 = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i2, i64 32), !dbg !13164
  %_18.idx.i8 = shl nuw nsw i64 %..i.i.i5, 2, !dbg !13167
  %_18.i9 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i3, i64 %_18.idx.i8, !dbg !13167
  br label %bb11.i10, !dbg !13172

bb11.i10:                                         ; preds = %bb11.i10, %bb11.preheader.i4
  %iter1.sroa.0.014.i11 = phi ptr [ %_31.i13, %bb11.i10 ], [ %iter.sroa.0.0.i3, %bb11.preheader.i4 ]
  %bits.sroa.0.013.i12 = phi i32 [ %7, %bb11.i10 ], [ 0, %bb11.preheader.i4 ]
  %_31.i13 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i11, i64 4, !dbg !13174
  %_134.i14 = load i32, ptr %iter1.sroa.0.014.i11, align 4, !dbg !13176, !alias.scope !13177, !noundef !12
  %7 = or i32 %_134.i14, %bits.sroa.0.013.i12, !dbg !13180
  %_25.i15 = icmp eq ptr %_31.i13, %_18.i9, !dbg !13181
  br i1 %_25.i15, label %bb10.i16, label %bb11.i10, !dbg !13172

bb10.i16:                                         ; preds = %bb11.i10
  %data.i.i.i.i6 = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i3, i64 %..i.i.i5, !dbg !13183
  %len.i.i.i.i7 = sub nuw nsw i64 %iter.sroa.6.0.i2, %..i.i.i5, !dbg !13188
  %8 = icmp eq i32 %7, 0, !dbg !13189
  br i1 %8, label %bb1.i1, label %bb21, !dbg !13189

bb4:                                              ; preds = %bb1.i1
  %9 = getelementptr inbounds nuw i8, ptr %self, i64 64, !dbg !13190
  %_48.0 = load ptr, ptr %9, align 8, !dbg !13190, !nonnull !12, !noundef !12
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 72, !dbg !13190
  %_48.1 = load i64, ptr %10, align 8, !dbg !13190, !noundef !12
  br label %bb1.i18, !dbg !13191

bb1.i18:                                          ; preds = %bb10.i33, %bb4
  %iter.sroa.6.0.i19 = phi i64 [ %_48.1, %bb4 ], [ %len.i.i.i.i24, %bb10.i33 ], !dbg !13193
  %iter.sroa.0.0.i20 = phi ptr [ %_48.0, %bb4 ], [ %data.i.i.i.i23, %bb10.i33 ], !dbg !13193
  %11 = icmp eq i64 %iter.sroa.6.0.i19, 0, !dbg !13194
  br i1 %11, label %bb6, label %bb11.preheader.i21, !dbg !13194

bb11.preheader.i21:                               ; preds = %bb1.i18
  %..i.i.i22 = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i19, i64 32), !dbg !13196
  %_18.idx.i25 = shl nuw nsw i64 %..i.i.i22, 2, !dbg !13199
  %_18.i26 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i20, i64 %_18.idx.i25, !dbg !13199
  br label %bb11.i27, !dbg !13204

bb11.i27:                                         ; preds = %bb11.i27, %bb11.preheader.i21
  %iter1.sroa.0.014.i28 = phi ptr [ %_31.i30, %bb11.i27 ], [ %iter.sroa.0.0.i20, %bb11.preheader.i21 ]
  %bits.sroa.0.013.i29 = phi i32 [ %12, %bb11.i27 ], [ 0, %bb11.preheader.i21 ]
  %_31.i30 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i28, i64 4, !dbg !13206
  %_134.i31 = load i32, ptr %iter1.sroa.0.014.i28, align 4, !dbg !13208, !alias.scope !13209, !noundef !12
  %12 = or i32 %_134.i31, %bits.sroa.0.013.i29, !dbg !13212
  %_25.i32 = icmp eq ptr %_31.i30, %_18.i26, !dbg !13213
  br i1 %_25.i32, label %bb10.i33, label %bb11.i27, !dbg !13204

bb10.i33:                                         ; preds = %bb11.i27
  %data.i.i.i.i23 = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i20, i64 %..i.i.i22, !dbg !13215
  %len.i.i.i.i24 = sub nuw nsw i64 %iter.sroa.6.0.i19, %..i.i.i22, !dbg !13220
  %13 = icmp eq i32 %12, 0, !dbg !13221
  br i1 %13, label %bb1.i18, label %bb21, !dbg !13221

bb6:                                              ; preds = %bb1.i18
  %14 = getelementptr inbounds nuw i8, ptr %self, i64 32, !dbg !13222
  %_49.0 = load ptr, ptr %14, align 8, !dbg !13222, !nonnull !12, !noundef !12
  %15 = getelementptr inbounds nuw i8, ptr %self, i64 40, !dbg !13222
  %_49.1 = load i64, ptr %15, align 8, !dbg !13222, !noundef !12
  %_8.i = getelementptr inbounds nuw float, ptr %_49.0, i64 %_49.1, !dbg !13223
  br label %bb1.i.i, !dbg !13234

bb1.i.i:                                          ; preds = %bb13.i.i, %bb6
  %_221.i.i = phi ptr [ %_22.i.i, %bb13.i.i ], [ %_49.0, %bb6 ]
  %_12.i.i = icmp eq ptr %_221.i.i, %_8.i, !dbg !13237
  br i1 %_12.i.i, label %bb8, label %bb13.i.i, !dbg !13245

bb13.i.i:                                         ; preds = %bb1.i.i
  %_22.i.i = getelementptr inbounds nuw i8, ptr %_221.i.i, i64 4, !dbg !13246
  %ptr.val.i.i = load i32, ptr %_221.i.i, align 4, !dbg !13249, !alias.scope !13250, !noalias !13253, !noundef !12
  %_0.i.i.i = icmp eq i32 %ptr.val.i.i, 1065353216, !dbg !13256
  br i1 %_0.i.i.i, label %bb1.i.i, label %bb21, !dbg !13249

bb8:                                              ; preds = %bb1.i.i
  %16 = getelementptr inbounds nuw i8, ptr %self, i64 48, !dbg !13260
  %_50.0 = load ptr, ptr %16, align 8, !dbg !13260, !nonnull !12, !noundef !12
  %17 = getelementptr inbounds nuw i8, ptr %self, i64 56, !dbg !13260
  %_50.1 = load i64, ptr %17, align 8, !dbg !13260, !noundef !12
  %_8.i35 = getelementptr inbounds nuw float, ptr %_50.0, i64 %_50.1, !dbg !13261
  br label %bb1.i.i36, !dbg !13266

bb1.i.i36:                                        ; preds = %bb13.i.i39, %bb8
  %_221.i.i37 = phi ptr [ %_22.i.i40, %bb13.i.i39 ], [ %_50.0, %bb8 ]
  %_12.i.i38 = icmp eq ptr %_221.i.i37, %_8.i35, !dbg !13268
  br i1 %_12.i.i38, label %bb10, label %bb13.i.i39, !dbg !13271

bb13.i.i39:                                       ; preds = %bb1.i.i36
  %_22.i.i40 = getelementptr inbounds nuw i8, ptr %_221.i.i37, i64 4, !dbg !13272
  %ptr.val.i.i41 = load i32, ptr %_221.i.i37, align 4, !dbg !13274, !alias.scope !13275, !noalias !13278, !noundef !12
  %_0.i.i.i42 = icmp eq i32 %ptr.val.i.i41, 1065353216, !dbg !13281
  br i1 %_0.i.i.i42, label %bb1.i.i36, label %bb21, !dbg !13274

bb10:                                             ; preds = %bb1.i.i36
  %18 = getelementptr inbounds nuw i8, ptr %self, i64 80, !dbg !13283
  %_51.0 = load ptr, ptr %18, align 8, !dbg !13283, !nonnull !12, !noundef !12
  %19 = getelementptr inbounds nuw i8, ptr %self, i64 88, !dbg !13283
  %_51.1 = load i64, ptr %19, align 8, !dbg !13283, !noundef !12
  %_8.i44 = getelementptr inbounds nuw float, ptr %_51.0, i64 %_51.1, !dbg !13284
  br label %bb1.i.i45, !dbg !13289

bb1.i.i45:                                        ; preds = %bb13.i.i48, %bb10
  %_221.i.i46 = phi ptr [ %_22.i.i49, %bb13.i.i48 ], [ %_51.0, %bb10 ]
  %_12.i.i47 = icmp eq ptr %_221.i.i46, %_8.i44, !dbg !13291
  br i1 %_12.i.i47, label %bb12, label %bb13.i.i48, !dbg !13294

bb13.i.i48:                                       ; preds = %bb1.i.i45
  %_22.i.i49 = getelementptr inbounds nuw i8, ptr %_221.i.i46, i64 4, !dbg !13295
  %ptr.val.i.i50 = load i32, ptr %_221.i.i46, align 4, !dbg !13297, !alias.scope !13298, !noalias !13301, !noundef !12
  %_0.i.i.i51 = icmp eq i32 %ptr.val.i.i50, 1065353216, !dbg !13304
  br i1 %_0.i.i.i51, label %bb1.i.i45, label %bb21, !dbg !13297

bb12:                                             ; preds = %bb1.i.i45
  %20 = getelementptr inbounds nuw i8, ptr %self, i64 96, !dbg !13306
  %_52.0 = load ptr, ptr %20, align 8, !dbg !13306, !nonnull !12, !noundef !12
  %21 = getelementptr inbounds nuw i8, ptr %self, i64 104, !dbg !13306
  %_52.1 = load i64, ptr %21, align 8, !dbg !13306, !noundef !12
  %22 = getelementptr inbounds nuw i8, ptr %self, i64 176, !dbg !13307
  %_53.0 = load ptr, ptr %22, align 8, !dbg !13307, !nonnull !12, !noundef !12
  %23 = getelementptr inbounds nuw i8, ptr %self, i64 184, !dbg !13307
  %_53.1 = load i64, ptr %23, align 8, !dbg !13307, !noundef !12
  %..i.i.i53 = tail call noundef i64 @llvm.umin.i64(i64 %_53.1, i64 %_52.1), !dbg !13308
  br label %bb1.i54, !dbg !13318

bb1.i54:                                          ; preds = %bb3.i, %bb12
  %24 = phi i64 [ %25, %bb3.i ], [ 0, %bb12 ]
  %exitcond.not = icmp eq i64 %24, %..i.i.i53, !dbg !13324
  br i1 %exitcond.not, label %bb21, label %bb3.i, !dbg !13324

bb3.i:                                            ; preds = %bb1.i54
  %25 = add i64 %24, 1, !dbg !13330
  %_3.i.i.i.i = getelementptr inbounds nuw float, ptr %_52.0, i64 %24, !dbg !13332
  %_3.i1.i.i.i = getelementptr inbounds nuw %LaneShape, ptr %_53.0, i64 %24, !dbg !13337
  %.val.i = load i32, ptr %_3.i.i.i.i, align 4, !dbg !13340, !noalias !13341, !noundef !12
  %.val1.i = load i32, ptr %_3.i1.i.i.i, align 4, !dbg !13340, !noalias !13341, !noundef !12
  %_8.i.i.i = uitofp i32 %.val1.i to float, !dbg !13344
  %_7.i.i.i = bitcast float %_8.i.i.i to i32, !dbg !13354
  %_0.i.i.not.i = icmp eq i32 %.val.i, %_7.i.i.i, !dbg !13357
  br i1 %_0.i.i.not.i, label %bb1.i54, label %bb21, !dbg !13340

bb21:                                             ; preds = %bb10.i, %bb10.i16, %bb10.i33, %bb13.i.i, %bb13.i.i39, %bb13.i.i48, %bb3.i, %bb1.i54
  %_0.sroa.0.0 = phi i1 [ false, %bb13.i.i48 ], [ false, %bb13.i.i39 ], [ false, %bb13.i.i ], [ false, %bb10.i33 ], [ false, %bb10.i16 ], [ false, %bb3.i ], [ true, %bb1.i54 ], [ false, %bb10.i ], !dbg !13358
  ret i1 %_0.sroa.0.0, !dbg !13359
}
