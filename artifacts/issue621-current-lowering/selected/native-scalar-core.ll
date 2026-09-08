define internal fastcc void @_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCorefE13process_blockB5_(ptr noalias noundef nonnull align 8 dereferenceable(784) %self, ptr noalias noundef nonnull align 4 captures(address) %left_io.0, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef nonnull align 4 captures(address) %right_io.0, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef %frames) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !36286 {
start:
  %_107.i695 = alloca [92 x i8], align 4
  %_105.i696 = alloca [92 x i8], align 4
  %peaks_right.i698 = alloca [1024 x i8], align 4
  %peaks_left.i699 = alloca [1024 x i8], align 4
  %scratch.i700 = alloca [32 x i8], align 4
  %hot_right.i701 = alloca [92 x i8], align 4
  %hot_left.i702 = alloca [92 x i8], align 4
  %peaks_right.i429 = alloca [1024 x i8], align 4
  %peaks_left.i430 = alloca [1024 x i8], align 4
  %scratch.i = alloca [32 x i8], align 4
  %hot_right.i431 = alloca [92 x i8], align 4
  %hot_left.i432 = alloca [92 x i8], align 4
  %_154.i14 = alloca [92 x i8], align 4
  %_152.i15 = alloca [92 x i8], align 4
  %uniform_right.i32 = alloca [80 x i8], align 8
  %uniform_left.i33 = alloca [80 x i8], align 8
  %peaks_right.i34 = alloca [1024 x i8], align 4
  %peaks_left.i35 = alloca [1024 x i8], align 4
  %hot_right.i36 = alloca [92 x i8], align 4
  %hot_left.i37 = alloca [92 x i8], align 4
  %uniform_right.i = alloca [80 x i8], align 8
  %uniform_left.i = alloca [80 x i8], align 8
  %peaks_right.i = alloca [1024 x i8], align 4
  %peaks_left.i = alloca [1024 x i8], align 4
  %hot_right.i = alloca [92 x i8], align 4
  %hot_left.i = alloca [92 x i8], align 4
  %shape = alloca [24 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 781, !dbg !36287
  %1 = load i8, ptr %0, align 1, !dbg !36287, !range !17, !noundef !12
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 96, !dbg !36289
  %3 = load i8, ptr %2, align 8, !dbg !36289, !range !17, !noundef !12
  %_7 = icmp eq i8 %1, %3, !dbg !36287
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 264
  %_95.0 = load ptr, ptr %4, align 8, !dbg !36290
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 272
  %_95.1 = load i64, ptr %5, align 8, !dbg !36290
  br i1 %_7, label %bb1, label %bb20.thread, !dbg !36287

bb1:                                              ; preds = %start
  %_8.i3881 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_95.0, i64 %_95.1, !dbg !36291
  br label %bb1.i.i3882, !dbg !36296

bb1.i.i3882:                                      ; preds = %bb13.i.i3883, %bb1
  %_221.i.i = phi ptr [ %_22.i.i3884, %bb13.i.i3883 ], [ %_95.0, %bb1 ]
  %_12.i.i = icmp eq ptr %_221.i.i, %_8.i3881, !dbg !36298
  br i1 %_12.i.i, label %bb3, label %bb13.i.i3883, !dbg !36301

bb13.i.i3883:                                     ; preds = %bb1.i.i3882
  %_22.i.i3884 = getelementptr inbounds nuw i8, ptr %_221.i.i, i64 16, !dbg !36302
  %6 = getelementptr inbounds nuw i8, ptr %_221.i.i, i64 12, !dbg !36304
  %_3.i.i.i = load i32, ptr %6, align 4, !dbg !36304, !alias.scope !36306, !noalias !36311, !noundef !12
  %7 = icmp eq i32 %_3.i.i.i, 0, !dbg !36304
  %_51.i.i.i = load i32, ptr %_221.i.i, align 4, !dbg !36304, !alias.scope !36306, !noalias !36311
  %8 = getelementptr inbounds nuw i8, ptr %_221.i.i, i64 4, !dbg !36304
  %_72.i.i.i = load i32, ptr %8, align 4, !dbg !36304, !alias.scope !36306, !noalias !36311
  %9 = icmp eq i32 %_51.i.i.i, %_72.i.i.i, !dbg !36304
  %_0.sroa.0.0.i.i.i = select i1 %7, i1 %9, i1 false, !dbg !36304
  br i1 %_0.sroa.0.0.i.i.i, label %bb1.i.i3882, label %bb20.thread, !dbg !36314

bb3:                                              ; preds = %bb1.i.i3882
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 280, !dbg !36315
  %_96.0 = load ptr, ptr %10, align 8, !dbg !36315, !nonnull !12, !noundef !12
  %11 = getelementptr inbounds nuw i8, ptr %self, i64 288, !dbg !36315
  %_96.1 = load i64, ptr %11, align 8, !dbg !36315, !noundef !12
  %_8.i3885 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_96.0, i64 %_96.1, !dbg !36316
  br label %bb1.i.i3886, !dbg !36321

bb1.i.i3886:                                      ; preds = %bb13.i.i3889, %bb3
  %_221.i.i3887 = phi ptr [ %_22.i.i3890, %bb13.i.i3889 ], [ %_96.0, %bb3 ]
  %_12.i.i3888 = icmp eq ptr %_221.i.i3887, %_8.i3885, !dbg !36323
  br i1 %_12.i.i3888, label %bb5, label %bb13.i.i3889, !dbg !36326

bb13.i.i3889:                                     ; preds = %bb1.i.i3886
  %_22.i.i3890 = getelementptr inbounds nuw i8, ptr %_221.i.i3887, i64 16, !dbg !36327
  %12 = getelementptr inbounds nuw i8, ptr %_221.i.i3887, i64 12, !dbg !36329
  %_3.i.i.i3891 = load i32, ptr %12, align 4, !dbg !36329, !alias.scope !36331, !noalias !36336, !noundef !12
  %13 = icmp eq i32 %_3.i.i.i3891, 0, !dbg !36329
  %_51.i.i.i3892 = load i32, ptr %_221.i.i3887, align 4, !dbg !36329, !alias.scope !36331, !noalias !36336
  %14 = getelementptr inbounds nuw i8, ptr %_221.i.i3887, i64 4, !dbg !36329
  %_72.i.i.i3893 = load i32, ptr %14, align 4, !dbg !36329, !alias.scope !36331, !noalias !36336
  %15 = icmp eq i32 %_51.i.i.i3892, %_72.i.i.i3893, !dbg !36329
  %_0.sroa.0.0.i.i.i3894 = select i1 %13, i1 %15, i1 false, !dbg !36329
  br i1 %_0.sroa.0.0.i.i.i3894, label %bb1.i.i3886, label %bb20.thread, !dbg !36339

bb5:                                              ; preds = %bb1.i.i3886
  %16 = getelementptr inbounds nuw i8, ptr %self, i64 464, !dbg !36340
  %_97.0 = load ptr, ptr %16, align 8, !dbg !36340, !nonnull !12, !noundef !12
  %17 = getelementptr inbounds nuw i8, ptr %self, i64 472, !dbg !36340
  %_97.1 = load i64, ptr %17, align 8, !dbg !36340, !noundef !12
  %_8.i3896 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_97.0, i64 %_97.1, !dbg !36341
  br label %bb1.i.i3897, !dbg !36346

bb1.i.i3897:                                      ; preds = %bb13.i.i3900, %bb5
  %_221.i.i3898 = phi ptr [ %_22.i.i3901, %bb13.i.i3900 ], [ %_97.0, %bb5 ]
  %_12.i.i3899 = icmp eq ptr %_221.i.i3898, %_8.i3896, !dbg !36348
  br i1 %_12.i.i3899, label %bb7, label %bb13.i.i3900, !dbg !36351

bb13.i.i3900:                                     ; preds = %bb1.i.i3897
  %_22.i.i3901 = getelementptr inbounds nuw i8, ptr %_221.i.i3898, i64 16, !dbg !36352
  %18 = getelementptr inbounds nuw i8, ptr %_221.i.i3898, i64 12, !dbg !36354
  %_3.i.i.i3902 = load i32, ptr %18, align 4, !dbg !36354, !alias.scope !36356, !noalias !36361, !noundef !12
  %19 = icmp eq i32 %_3.i.i.i3902, 0, !dbg !36354
  %_51.i.i.i3903 = load i32, ptr %_221.i.i3898, align 4, !dbg !36354, !alias.scope !36356, !noalias !36361
  %20 = getelementptr inbounds nuw i8, ptr %_221.i.i3898, i64 4, !dbg !36354
  %_72.i.i.i3904 = load i32, ptr %20, align 4, !dbg !36354, !alias.scope !36356, !noalias !36361
  %21 = icmp eq i32 %_51.i.i.i3903, %_72.i.i.i3904, !dbg !36354
  %_0.sroa.0.0.i.i.i3905 = select i1 %19, i1 %21, i1 false, !dbg !36354
  br i1 %_0.sroa.0.0.i.i.i3905, label %bb1.i.i3897, label %bb20.thread, !dbg !36364

bb7:                                              ; preds = %bb1.i.i3897
  %22 = getelementptr inbounds nuw i8, ptr %self, i64 480, !dbg !36365
  %_98.0 = load ptr, ptr %22, align 8, !dbg !36365, !nonnull !12, !noundef !12
  %23 = getelementptr inbounds nuw i8, ptr %self, i64 488, !dbg !36365
  %_98.1 = load i64, ptr %23, align 8, !dbg !36365, !noundef !12
  %_8.i3907 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_98.0, i64 %_98.1, !dbg !36366
  br label %bb1.i.i3908, !dbg !36371

bb1.i.i3908:                                      ; preds = %bb13.i.i3911, %bb7
  %_221.i.i3909 = phi ptr [ %_22.i.i3912, %bb13.i.i3911 ], [ %_98.0, %bb7 ]
  %_12.i.i3910 = icmp eq ptr %_221.i.i3909, %_8.i3907, !dbg !36373
  br i1 %_12.i.i3910, label %bb9, label %bb13.i.i3911, !dbg !36376

bb13.i.i3911:                                     ; preds = %bb1.i.i3908
  %_22.i.i3912 = getelementptr inbounds nuw i8, ptr %_221.i.i3909, i64 16, !dbg !36377
  %24 = getelementptr inbounds nuw i8, ptr %_221.i.i3909, i64 12, !dbg !36379
  %_3.i.i.i3913 = load i32, ptr %24, align 4, !dbg !36379, !alias.scope !36381, !noalias !36386, !noundef !12
  %25 = icmp eq i32 %_3.i.i.i3913, 0, !dbg !36379
  %_51.i.i.i3914 = load i32, ptr %_221.i.i3909, align 4, !dbg !36379, !alias.scope !36381, !noalias !36386
  %26 = getelementptr inbounds nuw i8, ptr %_221.i.i3909, i64 4, !dbg !36379
  %_72.i.i.i3915 = load i32, ptr %26, align 4, !dbg !36379, !alias.scope !36381, !noalias !36386
  %27 = icmp eq i32 %_51.i.i.i3914, %_72.i.i.i3915, !dbg !36379
  %_0.sroa.0.0.i.i.i3916 = select i1 %25, i1 %27, i1 false, !dbg !36379
  br i1 %_0.sroa.0.0.i.i.i3916, label %bb1.i.i3908, label %bb20.thread, !dbg !36389

bb9:                                              ; preds = %bb1.i.i3908
  %_65.not = icmp ugt i64 %frames, %left_io.1
  br i1 %_65.not, label %bb45, label %bb1.i3918, !dbg !36390, !prof !165

bb45:                                             ; preds = %bb9
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %frames, i64 noundef %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f0379187153dd7a3d2397dd31aa160d9) #30, !dbg !36399
  unreachable, !dbg !36399

bb1.i3918:                                        ; preds = %bb9, %bb10.i3925
  %iter.sroa.6.0.i = phi i64 [ %len.i.i.i.i, %bb10.i3925 ], [ %frames, %bb9 ], !dbg !36400
  %iter.sroa.0.0.i3919 = phi ptr [ %data.i.i.i.i, %bb10.i3925 ], [ %left_io.0, %bb9 ], !dbg !36400
  %28 = icmp eq i64 %iter.sroa.6.0.i, 0, !dbg !36402
  br i1 %28, label %bb11, label %bb11.preheader.i, !dbg !36402

bb11.preheader.i:                                 ; preds = %bb1.i3918
  %..i.i.i = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i, i64 32), !dbg !36404
  %_18.idx.i = shl nuw nsw i64 %..i.i.i, 2, !dbg !36407
  %_18.i3920 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i3919, i64 %_18.idx.i, !dbg !36407
  br label %bb11.i3921, !dbg !36412

bb11.i3921:                                       ; preds = %bb11.i3921, %bb11.preheader.i
  %iter1.sroa.0.014.i = phi ptr [ %_31.i3922, %bb11.i3921 ], [ %iter.sroa.0.0.i3919, %bb11.preheader.i ]
  %bits.sroa.0.013.i = phi i32 [ %29, %bb11.i3921 ], [ 0, %bb11.preheader.i ]
  %_31.i3922 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i, i64 4, !dbg !36414
  %_134.i3923 = load i32, ptr %iter1.sroa.0.014.i, align 4, !dbg !36416, !alias.scope !36417, !noundef !12
  %29 = or i32 %_134.i3923, %bits.sroa.0.013.i, !dbg !36420
  %_25.i3924 = icmp eq ptr %_31.i3922, %_18.i3920, !dbg !36421
  br i1 %_25.i3924, label %bb10.i3925, label %bb11.i3921, !dbg !36412

bb10.i3925:                                       ; preds = %bb11.i3921
  %data.i.i.i.i = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i3919, i64 %..i.i.i, !dbg !36423
  %len.i.i.i.i = sub nuw nsw i64 %iter.sroa.6.0.i, %..i.i.i, !dbg !36428
  %30 = icmp eq i32 %29, 0, !dbg !36429
  br i1 %30, label %bb1.i3918, label %bb20.thread, !dbg !36429

bb11:                                             ; preds = %bb1.i3918
  %_73.not = icmp ugt i64 %frames, %right_io.1, !dbg !36430
  br i1 %_73.not, label %bb48, label %bb1.i3926, !dbg !36430, !prof !639

bb20.thread:                                      ; preds = %bb13.i.i3883, %bb13.i.i3889, %bb13.i.i3900, %bb13.i.i3911, %bb10.i3925, %start
  %31 = getelementptr inbounds nuw i8, ptr %self, i64 780
  br label %bb26, !dbg !36436

bb20:                                             ; preds = %bb1.i3926
  %32 = getelementptr inbounds nuw i8, ptr %self, i64 780
  %33 = load i8, ptr %32, align 4, !range !17
  %_22 = trunc nuw i8 %33 to i1
  br i1 %_22, label %bb22, label %bb26, !dbg !36436

bb48:                                             ; preds = %bb11
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %frames, i64 noundef %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_637d5bfd284109ae56bb817a8357666c) #30, !dbg !36438
  unreachable, !dbg !36438

bb1.i3926:                                        ; preds = %bb11, %bb10.i3941
  %iter.sroa.6.0.i3927 = phi i64 [ %len.i.i.i.i3932, %bb10.i3941 ], [ %frames, %bb11 ], !dbg !36439
  %iter.sroa.0.0.i3928 = phi ptr [ %data.i.i.i.i3931, %bb10.i3941 ], [ %right_io.0, %bb11 ], !dbg !36439
  %34 = icmp eq i64 %iter.sroa.6.0.i3927, 0, !dbg !36441
  br i1 %34, label %bb20, label %bb11.preheader.i3929, !dbg !36441

bb11.preheader.i3929:                             ; preds = %bb1.i3926
  %..i.i.i3930 = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i3927, i64 32), !dbg !36443
  %_18.idx.i3933 = shl nuw nsw i64 %..i.i.i3930, 2, !dbg !36446
  %_18.i3934 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i3928, i64 %_18.idx.i3933, !dbg !36446
  br label %bb11.i3935, !dbg !36451

bb11.i3935:                                       ; preds = %bb11.i3935, %bb11.preheader.i3929
  %iter1.sroa.0.014.i3936 = phi ptr [ %_31.i3938, %bb11.i3935 ], [ %iter.sroa.0.0.i3928, %bb11.preheader.i3929 ]
  %bits.sroa.0.013.i3937 = phi i32 [ %35, %bb11.i3935 ], [ 0, %bb11.preheader.i3929 ]
  %_31.i3938 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i3936, i64 4, !dbg !36453
  %_134.i3939 = load i32, ptr %iter1.sroa.0.014.i3936, align 4, !dbg !36455, !alias.scope !36456, !noundef !12
  %35 = or i32 %_134.i3939, %bits.sroa.0.013.i3937, !dbg !36459
  %_25.i3940 = icmp eq ptr %_31.i3938, %_18.i3934, !dbg !36460
  br i1 %_25.i3940, label %bb10.i3941, label %bb11.i3935, !dbg !36451

bb10.i3941:                                       ; preds = %bb11.i3935
  %data.i.i.i.i3931 = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i3928, i64 %..i.i.i3930, !dbg !36462
  %len.i.i.i.i3932 = sub nuw nsw i64 %iter.sroa.6.0.i3927, %..i.i.i3930, !dbg !36467
  %36 = icmp eq i32 %35, 0, !dbg !36468
  br i1 %36, label %bb1.i3926, label %bb20.thread4642, !dbg !36468

bb20.thread4642:                                  ; preds = %bb10.i3941
  %37 = getelementptr inbounds nuw i8, ptr %self, i64 780
  br label %bb26, !dbg !36436

bb26:                                             ; preds = %bb20.thread4642, %bb20.thread, %bb20
  %38 = phi ptr [ %31, %bb20.thread ], [ %32, %bb20 ], [ %37, %bb20.thread4642 ]
  %quiet.sroa.0.04641 = phi i1 [ false, %bb20.thread ], [ true, %bb20 ], [ false, %bb20.thread4642 ]
  %_31 = getelementptr inbounds nuw i8, ptr %self, i64 584, !dbg !36469
  %_32 = getelementptr inbounds nuw i8, ptr %self, i64 536, !dbg !36470
  %_33 = getelementptr inbounds nuw i8, ptr %self, i64 136, !dbg !36471
  %_34 = getelementptr inbounds nuw i8, ptr %self, i64 336, !dbg !36472
  %_35 = getelementptr inbounds nuw i8, ptr %self, i64 560, !dbg !36473
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36474), !dbg !36477
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36480), !dbg !36477
  %_8.i3943 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_95.0, i64 %_95.1, !dbg !36482
  br label %bb1.i.i3944, !dbg !36488

bb1.i.i3944:                                      ; preds = %bb13.i.i3947, %bb26
  %_221.i.i3945 = phi ptr [ %_22.i.i3948, %bb13.i.i3947 ], [ %_95.0, %bb26 ]
  %_12.i.i3946 = icmp eq ptr %_221.i.i3945, %_8.i3943, !dbg !36490
  br i1 %_12.i.i3946, label %bb2.i, label %bb13.i.i3947, !dbg !36493

bb13.i.i3947:                                     ; preds = %bb1.i.i3944
  %_22.i.i3948 = getelementptr inbounds nuw i8, ptr %_221.i.i3945, i64 16, !dbg !36494
  %39 = getelementptr inbounds nuw i8, ptr %_221.i.i3945, i64 12, !dbg !36496
  %_3.i.i.i3949 = load i32, ptr %39, align 4, !dbg !36496, !alias.scope !36498, !noalias !36503, !noundef !12
  %40 = icmp eq i32 %_3.i.i.i3949, 0, !dbg !36496
  %_51.i.i.i3950 = load i32, ptr %_221.i.i3945, align 4, !dbg !36496, !alias.scope !36498, !noalias !36503
  %41 = getelementptr inbounds nuw i8, ptr %_221.i.i3945, i64 4, !dbg !36496
  %_72.i.i.i3951 = load i32, ptr %41, align 4, !dbg !36496, !alias.scope !36498, !noalias !36503
  %42 = icmp eq i32 %_51.i.i.i3950, %_72.i.i.i3951, !dbg !36496
  %_0.sroa.0.0.i.i.i3952 = select i1 %40, i1 %42, i1 false, !dbg !36496
  br i1 %_0.sroa.0.0.i.i.i3952, label %bb1.i.i3944, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit, !dbg !36512

bb2.i:                                            ; preds = %bb1.i.i3944
  %43 = getelementptr inbounds nuw i8, ptr %self, i64 280, !dbg !36513
  %_15.0.i = load ptr, ptr %43, align 8, !dbg !36513, !alias.scope !36474, !noalias !36514, !nonnull !12, !noundef !12
  %44 = getelementptr inbounds nuw i8, ptr %self, i64 288, !dbg !36513
  %_15.1.i = load i64, ptr %44, align 8, !dbg !36513, !alias.scope !36474, !noalias !36514, !noundef !12
  %_8.i3954 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_15.0.i, i64 %_15.1.i, !dbg !36515
  br label %bb1.i.i3955, !dbg !36520

bb1.i.i3955:                                      ; preds = %bb13.i.i3958, %bb2.i
  %_221.i.i3956 = phi ptr [ %_22.i.i3959, %bb13.i.i3958 ], [ %_15.0.i, %bb2.i ]
  %_12.i.i3957 = icmp eq ptr %_221.i.i3956, %_8.i3954, !dbg !36522
  br i1 %_12.i.i3957, label %bb4.i1455, label %bb13.i.i3958, !dbg !36525

bb13.i.i3958:                                     ; preds = %bb1.i.i3955
  %_22.i.i3959 = getelementptr inbounds nuw i8, ptr %_221.i.i3956, i64 16, !dbg !36526
  %45 = getelementptr inbounds nuw i8, ptr %_221.i.i3956, i64 12, !dbg !36528
  %_3.i.i.i3960 = load i32, ptr %45, align 4, !dbg !36528, !alias.scope !36530, !noalias !36535, !noundef !12
  %46 = icmp eq i32 %_3.i.i.i3960, 0, !dbg !36528
  %_51.i.i.i3961 = load i32, ptr %_221.i.i3956, align 4, !dbg !36528, !alias.scope !36530, !noalias !36535
  %47 = getelementptr inbounds nuw i8, ptr %_221.i.i3956, i64 4, !dbg !36528
  %_72.i.i.i3962 = load i32, ptr %47, align 4, !dbg !36528, !alias.scope !36530, !noalias !36535
  %48 = icmp eq i32 %_51.i.i.i3961, %_72.i.i.i3962, !dbg !36528
  %_0.sroa.0.0.i.i.i3963 = select i1 %46, i1 %48, i1 false, !dbg !36528
  br i1 %_0.sroa.0.0.i.i.i3963, label %bb1.i.i3955, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit, !dbg !36538

bb4.i1455:                                        ; preds = %bb1.i.i3955
  %49 = getelementptr inbounds nuw i8, ptr %self, i64 464, !dbg !36539
  %_16.0.i = load ptr, ptr %49, align 8, !dbg !36539, !alias.scope !36480, !noalias !36540, !nonnull !12, !noundef !12
  %50 = getelementptr inbounds nuw i8, ptr %self, i64 472, !dbg !36539
  %_16.1.i = load i64, ptr %50, align 8, !dbg !36539, !alias.scope !36480, !noalias !36540, !noundef !12
  %_8.i3965 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_16.0.i, i64 %_16.1.i, !dbg !36541
  br label %bb1.i.i3966, !dbg !36546

bb1.i.i3966:                                      ; preds = %bb13.i.i3969, %bb4.i1455
  %_221.i.i3967 = phi ptr [ %_22.i.i3970, %bb13.i.i3969 ], [ %_16.0.i, %bb4.i1455 ]
  %_12.i.i3968 = icmp eq ptr %_221.i.i3967, %_8.i3965, !dbg !36548
  br i1 %_12.i.i3968, label %bb6.i1456, label %bb13.i.i3969, !dbg !36551

bb13.i.i3969:                                     ; preds = %bb1.i.i3966
  %_22.i.i3970 = getelementptr inbounds nuw i8, ptr %_221.i.i3967, i64 16, !dbg !36552
  %51 = getelementptr inbounds nuw i8, ptr %_221.i.i3967, i64 12, !dbg !36554
  %_3.i.i.i3971 = load i32, ptr %51, align 4, !dbg !36554, !alias.scope !36556, !noalias !36561, !noundef !12
  %52 = icmp eq i32 %_3.i.i.i3971, 0, !dbg !36554
  %_51.i.i.i3972 = load i32, ptr %_221.i.i3967, align 4, !dbg !36554, !alias.scope !36556, !noalias !36561
  %53 = getelementptr inbounds nuw i8, ptr %_221.i.i3967, i64 4, !dbg !36554
  %_72.i.i.i3973 = load i32, ptr %53, align 4, !dbg !36554, !alias.scope !36556, !noalias !36561
  %54 = icmp eq i32 %_51.i.i.i3972, %_72.i.i.i3973, !dbg !36554
  %_0.sroa.0.0.i.i.i3974 = select i1 %52, i1 %54, i1 false, !dbg !36554
  br i1 %_0.sroa.0.0.i.i.i3974, label %bb1.i.i3966, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit, !dbg !36564

bb6.i1456:                                        ; preds = %bb1.i.i3966
  %55 = getelementptr inbounds nuw i8, ptr %self, i64 480, !dbg !36565
  %_17.0.i = load ptr, ptr %55, align 8, !dbg !36565, !alias.scope !36480, !noalias !36540, !nonnull !12, !noundef !12
  %56 = getelementptr inbounds nuw i8, ptr %self, i64 488, !dbg !36565
  %_17.1.i = load i64, ptr %56, align 8, !dbg !36565, !alias.scope !36480, !noalias !36540, !noundef !12
  %_8.i3976 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_17.0.i, i64 %_17.1.i, !dbg !36566
  br label %bb1.i.i3977, !dbg !36571

bb1.i.i3977:                                      ; preds = %bb13.i.i3980, %bb6.i1456
  %_221.i.i3978 = phi ptr [ %_22.i.i3981, %bb13.i.i3980 ], [ %_17.0.i, %bb6.i1456 ]
  %_12.i.i3979 = icmp eq ptr %_221.i.i3978, %_8.i3976, !dbg !36573
  br i1 %_12.i.i3979, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit, label %bb13.i.i3980, !dbg !36576

bb13.i.i3980:                                     ; preds = %bb1.i.i3977
  %_22.i.i3981 = getelementptr inbounds nuw i8, ptr %_221.i.i3978, i64 16, !dbg !36577
  %57 = getelementptr inbounds nuw i8, ptr %_221.i.i3978, i64 12, !dbg !36579
  %_3.i.i.i3982 = load i32, ptr %57, align 4, !dbg !36579, !alias.scope !36581, !noalias !36586, !noundef !12
  %58 = icmp eq i32 %_3.i.i.i3982, 0, !dbg !36579
  %_51.i.i.i3983 = load i32, ptr %_221.i.i3978, align 4, !dbg !36579, !alias.scope !36581, !noalias !36586
  %59 = getelementptr inbounds nuw i8, ptr %_221.i.i3978, i64 4, !dbg !36579
  %_72.i.i.i3984 = load i32, ptr %59, align 4, !dbg !36579, !alias.scope !36581, !noalias !36586
  %60 = icmp eq i32 %_51.i.i.i3983, %_72.i.i.i3984, !dbg !36579
  %_0.sroa.0.0.i.i.i3985 = select i1 %58, i1 %60, i1 false, !dbg !36579
  br i1 %_0.sroa.0.0.i.i.i3985, label %bb1.i.i3977, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit, !dbg !36589

_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit: ; preds = %bb13.i.i3947, %bb13.i.i3958, %bb13.i.i3969, %bb13.i.i3980, %bb1.i.i3977
  %_0.sroa.0.0.i = phi i1 [ false, %bb13.i.i3969 ], [ false, %bb13.i.i3958 ], [ false, %bb13.i.i3980 ], [ true, %bb1.i.i3977 ], [ false, %bb13.i.i3947 ], !dbg !36590
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36591), !dbg !36594
  %61 = getelementptr inbounds nuw i8, ptr %self, i64 312, !dbg !36596
  %_31.0.i = load ptr, ptr %61, align 8, !dbg !36596, !alias.scope !36591, !noalias !36598, !nonnull !12, !noundef !12
  %62 = getelementptr inbounds nuw i8, ptr %self, i64 320, !dbg !36596
  %_31.1.i = load i64, ptr %62, align 8, !dbg !36596, !alias.scope !36591, !noalias !36598, !noundef !12
  %_17.idx.i = mul nuw nsw i64 %_31.1.i, 12, !dbg !36599
  %_17.i = getelementptr inbounds nuw i8, ptr %_31.0.i, i64 %_17.idx.i, !dbg !36599
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36603), !dbg !36606, !noalias !36598
  %_5.not.i.i.i = icmp eq i64 %_31.1.i, 0
  %63 = getelementptr inbounds nuw i8, ptr %_31.0.i, i64 4
  %64 = getelementptr inbounds nuw i8, ptr %_31.0.i, i64 8
  br i1 %_5.not.i.i.i, label %bb2.i3995, label %bb1.i.i3987

bb1.i.i3987:                                      ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i
  %_224.i.i = phi ptr [ %_22.i.i3990, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i ], [ %_31.0.i, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit ]
  %_12.i.i3988 = icmp eq ptr %_224.i.i, %_17.i, !dbg !36607
  br i1 %_12.i.i3988, label %bb2.i3995, label %bb13.i.i3989, !dbg !36611

bb13.i.i3989:                                     ; preds = %bb1.i.i3987
  %_22.i.i3990 = getelementptr inbounds nuw i8, ptr %_224.i.i, i64 12, !dbg !36612
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36614), !dbg !36617, !noalias !36598
  %_9.i.i.i = load i32, ptr %_224.i.i, align 4, !dbg !36618, !alias.scope !36614, !noalias !36621, !noundef !12
  %_10.i.i.i = load i32, ptr %_31.0.i, align 4, !dbg !36618, !alias.scope !36603, !noalias !36623, !noundef !12
  %_8.i.i.i = icmp eq i32 %_9.i.i.i, %_10.i.i.i, !dbg !36618
  br i1 %_8.i.i.i, label %bb2.i.i.i, label %bb10.i, !dbg !36618

bb2.i.i.i:                                        ; preds = %bb13.i.i3989
  %65 = getelementptr inbounds nuw i8, ptr %_224.i.i, i64 4, !dbg !36618
  %_12.i.i.i = load i32, ptr %65, align 4, !dbg !36618, !alias.scope !36614, !noalias !36621, !noundef !12
  %_13.i.i.i = load i32, ptr %63, align 4, !dbg !36618, !alias.scope !36603, !noalias !36623, !noundef !12
  %_11.i.i.i = icmp eq i32 %_12.i.i.i, %_13.i.i.i, !dbg !36618
  br i1 %_11.i.i.i, label %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, label %bb10.i, !dbg !36618

_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i: ; preds = %bb2.i.i.i
  %66 = getelementptr inbounds nuw i8, ptr %_224.i.i, i64 8, !dbg !36618
  %_14.i.i.i3993 = load i32, ptr %66, align 4, !dbg !36618, !alias.scope !36614, !noalias !36621, !noundef !12
  %_15.i.i.i3994 = load i32, ptr %64, align 4, !dbg !36618, !alias.scope !36603, !noalias !36623, !noundef !12
  %67 = icmp eq i32 %_14.i.i.i3993, %_15.i.i.i3994, !dbg !36618
  br i1 %67, label %bb1.i.i3987, label %bb10.i, !dbg !36617

bb2.i3995:                                        ; preds = %bb1.i.i3987, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit
  %68 = getelementptr inbounds nuw i8, ptr %self, i64 248, !dbg !36624
  %_32.0.i = load ptr, ptr %68, align 8, !dbg !36624, !alias.scope !36591, !noalias !36598, !nonnull !12, !noundef !12
  %69 = getelementptr inbounds nuw i8, ptr %self, i64 256, !dbg !36624
  %_32.1.i = load i64, ptr %69, align 8, !dbg !36624, !alias.scope !36591, !noalias !36598, !noundef !12
  %_26.idx.i = shl nuw nsw i64 %_32.1.i, 2, !dbg !36625
  %_26.i3996 = getelementptr inbounds nuw i8, ptr %_32.0.i, i64 %_26.idx.i, !dbg !36625
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36629), !dbg !36632, !noalias !36598
  %_6.not.i.i.i = icmp eq i64 %_32.1.i, 0
  br i1 %_6.not.i.i.i, label %bb3.i, label %bb1.i3.i

bb1.i3.i:                                         ; preds = %bb2.i3995, %bb13.i5.i
  %_223.i.i = phi ptr [ %_22.i6.i, %bb13.i5.i ], [ %_32.0.i, %bb2.i3995 ]
  %_12.i4.i = icmp eq ptr %_223.i.i, %_26.i3996, !dbg !36633
  br i1 %_12.i4.i, label %bb3.i, label %bb13.i5.i, !dbg !36637

bb13.i5.i:                                        ; preds = %bb1.i3.i
  %_22.i6.i = getelementptr inbounds nuw i8, ptr %_223.i.i, i64 4, !dbg !36638
  %ptr.val.i.i = load i32, ptr %_223.i.i, align 4, !dbg !36640, !noalias !36641
  %_4.i.i.i3997 = load i32, ptr %_32.0.i, align 4, !dbg !36643, !alias.scope !36629, !noalias !36645, !noundef !12
  %_0.i.i.i = icmp eq i32 %ptr.val.i.i, %_4.i.i.i3997, !dbg !36646
  br i1 %_0.i.i.i, label %bb1.i3.i, label %bb10.i, !dbg !36640

bb3.i:                                            ; preds = %bb1.i3.i, %bb2.i3995
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36647), !dbg !36650
  %70 = getelementptr inbounds nuw i8, ptr %self, i64 512, !dbg !36651
  %_31.0.i3998 = load ptr, ptr %70, align 8, !dbg !36651, !alias.scope !36647, !noalias !36598, !nonnull !12, !noundef !12
  %71 = getelementptr inbounds nuw i8, ptr %self, i64 520, !dbg !36651
  %_31.1.i3999 = load i64, ptr %71, align 8, !dbg !36651, !alias.scope !36647, !noalias !36598, !noundef !12
  %_17.idx.i4000 = mul nuw nsw i64 %_31.1.i3999, 12, !dbg !36653
  %_17.i4001 = getelementptr inbounds nuw i8, ptr %_31.0.i3998, i64 %_17.idx.i4000, !dbg !36653
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36657), !dbg !36660, !noalias !36598
  %_5.not.i.i.i4002 = icmp eq i64 %_31.1.i3999, 0
  %72 = getelementptr inbounds nuw i8, ptr %_31.0.i3998, i64 4
  %73 = getelementptr inbounds nuw i8, ptr %_31.0.i3998, i64 8
  br i1 %_5.not.i.i.i4002, label %bb2.i4020, label %bb1.i.i4003

bb1.i.i4003:                                      ; preds = %bb3.i, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i4017
  %_224.i.i4004 = phi ptr [ %_22.i.i4007, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i4017 ], [ %_31.0.i3998, %bb3.i ]
  %_12.i.i4005 = icmp eq ptr %_224.i.i4004, %_17.i4001, !dbg !36661
  br i1 %_12.i.i4005, label %bb2.i4020, label %bb13.i.i4006, !dbg !36665

bb13.i.i4006:                                     ; preds = %bb1.i.i4003
  %_22.i.i4007 = getelementptr inbounds nuw i8, ptr %_224.i.i4004, i64 12, !dbg !36666
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36668), !dbg !36671, !noalias !36598
  %_9.i.i.i4008 = load i32, ptr %_224.i.i4004, align 4, !dbg !36672, !alias.scope !36668, !noalias !36675, !noundef !12
  %_10.i.i.i4009 = load i32, ptr %_31.0.i3998, align 4, !dbg !36672, !alias.scope !36657, !noalias !36677, !noundef !12
  %_8.i.i.i4010 = icmp eq i32 %_9.i.i.i4008, %_10.i.i.i4009, !dbg !36672
  br i1 %_8.i.i.i4010, label %bb2.i.i.i4013, label %bb10.i, !dbg !36672

bb2.i.i.i4013:                                    ; preds = %bb13.i.i4006
  %74 = getelementptr inbounds nuw i8, ptr %_224.i.i4004, i64 4, !dbg !36672
  %_12.i.i.i4014 = load i32, ptr %74, align 4, !dbg !36672, !alias.scope !36668, !noalias !36675, !noundef !12
  %_13.i.i.i4015 = load i32, ptr %72, align 4, !dbg !36672, !alias.scope !36657, !noalias !36677, !noundef !12
  %_11.i.i.i4016 = icmp eq i32 %_12.i.i.i4014, %_13.i.i.i4015, !dbg !36672
  br i1 %_11.i.i.i4016, label %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i4017, label %bb10.i, !dbg !36672

_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i4017: ; preds = %bb2.i.i.i4013
  %75 = getelementptr inbounds nuw i8, ptr %_224.i.i4004, i64 8, !dbg !36672
  %_14.i.i.i4018 = load i32, ptr %75, align 4, !dbg !36672, !alias.scope !36668, !noalias !36675, !noundef !12
  %_15.i.i.i4019 = load i32, ptr %73, align 4, !dbg !36672, !alias.scope !36657, !noalias !36677, !noundef !12
  %76 = icmp eq i32 %_14.i.i.i4018, %_15.i.i.i4019, !dbg !36672
  br i1 %76, label %bb1.i.i4003, label %bb10.i, !dbg !36671

bb2.i4020:                                        ; preds = %bb1.i.i4003, %bb3.i
  %77 = getelementptr inbounds nuw i8, ptr %self, i64 448, !dbg !36678
  %_32.0.i4021 = load ptr, ptr %77, align 8, !dbg !36678, !alias.scope !36647, !noalias !36598, !nonnull !12, !noundef !12
  %78 = getelementptr inbounds nuw i8, ptr %self, i64 456, !dbg !36678
  %_32.1.i4022 = load i64, ptr %78, align 8, !dbg !36678, !alias.scope !36647, !noalias !36598, !noundef !12
  %_26.idx.i4023 = shl nuw nsw i64 %_32.1.i4022, 2, !dbg !36679
  %_26.i4024 = getelementptr inbounds nuw i8, ptr %_32.0.i4021, i64 %_26.idx.i4023, !dbg !36679
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36683), !dbg !36686, !noalias !36598
  %_6.not.i.i.i4025 = icmp eq i64 %_32.1.i4022, 0
  br i1 %_6.not.i.i.i4025, label %bb5.i, label %bb1.i3.i4026

bb1.i3.i4026:                                     ; preds = %bb2.i4020, %bb13.i5.i4029
  %_223.i.i4027 = phi ptr [ %_22.i6.i4030, %bb13.i5.i4029 ], [ %_32.0.i4021, %bb2.i4020 ]
  %_12.i4.i4028 = icmp eq ptr %_223.i.i4027, %_26.i4024, !dbg !36687
  br i1 %_12.i4.i4028, label %bb5.i, label %bb13.i5.i4029, !dbg !36691

bb13.i5.i4029:                                    ; preds = %bb1.i3.i4026
  %_22.i6.i4030 = getelementptr inbounds nuw i8, ptr %_223.i.i4027, i64 4, !dbg !36692
  %ptr.val.i.i4031 = load i32, ptr %_223.i.i4027, align 4, !dbg !36694, !noalias !36695
  %_4.i.i.i4032 = load i32, ptr %_32.0.i4021, align 4, !dbg !36697, !alias.scope !36683, !noalias !36699, !noundef !12
  %_0.i.i.i4033 = icmp eq i32 %ptr.val.i.i4031, %_4.i.i.i4032, !dbg !36700
  br i1 %_0.i.i.i4033, label %bb1.i3.i4026, label %bb10.i, !dbg !36694

bb10.i:                                           ; preds = %bb13.i.i3989, %bb2.i.i.i, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, %bb13.i5.i, %bb13.i.i4006, %bb2.i.i.i4013, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i4017, %bb13.i5.i4029
  br i1 %_0.sroa.0.0.i, label %bb11.i, label %bb12.i, !dbg !36701

bb5.i:                                            ; preds = %bb1.i3.i4026, %bb2.i4020
  br i1 %_0.sroa.0.0.i, label %bb6.i, label %bb7.i, !dbg !36702

bb12.i:                                           ; preds = %bb10.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36703), !dbg !36706
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36707), !dbg !36706
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36709), !dbg !36706
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36711), !dbg !36706
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i702), !dbg !36713, !noalias !36717
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_left.i702, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #31, !dbg !36721, !noalias !36722
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_right.i701), !dbg !36723, !noalias !36717
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_right.i701, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #31, !dbg !36725, !noalias !36726
  %79 = getelementptr inbounds nuw i8, ptr %self, i64 776, !dbg !36727
  %80 = load i8, ptr %79, align 4, !dbg !36727, !range !17, !alias.scope !36703, !noalias !36731, !noundef !12
  %81 = getelementptr inbounds nuw i8, ptr %self, i64 777, !dbg !36732
  %82 = load i8, ptr %81, align 1, !dbg !36732, !range !17, !alias.scope !36703, !noalias !36731, !noundef !12
  %_34.i710 = load i32, ptr %_35, align 4, !dbg !36734, !alias.scope !36711, !noalias !36736, !noundef !12
  %83 = getelementptr inbounds nuw i8, ptr %self, i64 564, !dbg !36737
  %_36.i711 = load i32, ptr %83, align 4, !dbg !36737, !alias.scope !36711, !noalias !36736, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i700), !dbg !36739, !noalias !36717
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i700, i8 0, i64 32, i1 false), !noalias !36717
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i699), !dbg !36741, !noalias !36717
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i699, i8 0, i64 1024, i1 false), !noalias !36717
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i698), !dbg !36743, !noalias !36717
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i698, i8 0, i64 1024, i1 false), !noalias !36717
  %_31.i706 = zext nneg i8 %80 to i32, !dbg !36727
  %.none.i707 = sub nsw i32 0, %_31.i706, !dbg !36745
  %_32.i708 = zext nneg i8 %82 to i32, !dbg !36732
  %all.sroa.0.0.i709 = sub nsw i32 0, %_32.i708, !dbg !36732
  %_111.not.i7247558 = icmp eq i64 %frames, 0, !dbg !36746
  br i1 %_111.not.i7247558, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit, label %bb37.i725.lr.ph, !dbg !36746

bb37.i725.lr.ph:                                  ; preds = %bb12.i
  %84 = zext i32 %_36.i711 to i64, !dbg !36737
  %85 = zext i32 %_34.i710 to i64, !dbg !36734
  %d9.i.i = lshr i64 %frames, 5, !dbg !36756
  %r2.i.i = and i64 %frames, 31, !dbg !36762
  %_19.not.i.i = icmp ne i64 %r2.i.i, 0, !dbg !36763
  %86 = zext i1 %_19.not.i.i to i64, !dbg !36763
  %yield_count.sroa.0.0.i.i = add nuw nsw i64 %d9.i.i, %86, !dbg !36763
  %history.i136.i.sroa.7.0.hot_left.i702.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i702, i64 4
  %history.i136.i.sroa.10.0.hot_left.i702.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i702, i64 8
  %history.i136.i.sroa.13.0.hot_left.i702.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i702, i64 12
  %history.i136.i.sroa.16.0.hot_left.i702.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i702, i64 16
  %history.i136.i.sroa.19.0.hot_left.i702.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i702, i64 20
  %history.i136.i.sroa.22.0.hot_left.i702.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i702, i64 24
  %history.i136.i.sroa.26.0.hot_left.i702.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i702, i64 28
  %history.i136.i.sroa.29.0.hot_left.i702.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i702, i64 32
  %history.i136.i.sroa.32.0.hot_left.i702.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i702, i64 36
  %history.i136.i.sroa.35.0.hot_left.i702.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i702, i64 40
  %history.i136.i.sroa.38.0.hot_left.i702.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i702, i64 44
  %87 = getelementptr inbounds nuw i8, ptr %self, i64 588
  %88 = getelementptr inbounds nuw i8, ptr %self, i64 592
  %89 = getelementptr inbounds nuw i8, ptr %self, i64 596
  %row1.i.i.i172.i = getelementptr inbounds nuw i8, ptr %self, i64 600
  %90 = getelementptr inbounds nuw i8, ptr %self, i64 604
  %91 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %92 = getelementptr inbounds nuw i8, ptr %self, i64 612
  %row3.i.i.i186.i = getelementptr inbounds nuw i8, ptr %self, i64 616
  %93 = getelementptr inbounds nuw i8, ptr %self, i64 620
  %94 = getelementptr inbounds nuw i8, ptr %self, i64 624
  %95 = getelementptr inbounds nuw i8, ptr %self, i64 628
  %row5.i.i.i200.i = getelementptr inbounds nuw i8, ptr %self, i64 632
  %96 = getelementptr inbounds nuw i8, ptr %self, i64 636
  %97 = getelementptr inbounds nuw i8, ptr %self, i64 640
  %98 = getelementptr inbounds nuw i8, ptr %self, i64 644
  %row7.i.i.i214.i = getelementptr inbounds nuw i8, ptr %self, i64 648
  %99 = getelementptr inbounds nuw i8, ptr %self, i64 652
  %100 = getelementptr inbounds nuw i8, ptr %self, i64 656
  %101 = getelementptr inbounds nuw i8, ptr %self, i64 660
  %row9.i.i.i228.i = getelementptr inbounds nuw i8, ptr %self, i64 664
  %102 = getelementptr inbounds nuw i8, ptr %self, i64 668
  %103 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %104 = getelementptr inbounds nuw i8, ptr %self, i64 676
  %row11.i.i.i242.i = getelementptr inbounds nuw i8, ptr %self, i64 680
  %105 = getelementptr inbounds nuw i8, ptr %self, i64 684
  %106 = getelementptr inbounds nuw i8, ptr %self, i64 688
  %107 = getelementptr inbounds nuw i8, ptr %self, i64 692
  %row13.i.i.i256.i = getelementptr inbounds nuw i8, ptr %self, i64 696
  %108 = getelementptr inbounds nuw i8, ptr %self, i64 700
  %109 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %110 = getelementptr inbounds nuw i8, ptr %self, i64 708
  %row15.i.i.i270.i = getelementptr inbounds nuw i8, ptr %self, i64 712
  %111 = getelementptr inbounds nuw i8, ptr %self, i64 716
  %112 = getelementptr inbounds nuw i8, ptr %self, i64 720
  %113 = getelementptr inbounds nuw i8, ptr %self, i64 724
  %row17.i.i.i284.i = getelementptr inbounds nuw i8, ptr %self, i64 728
  %114 = getelementptr inbounds nuw i8, ptr %self, i64 732
  %115 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %116 = getelementptr inbounds nuw i8, ptr %self, i64 740
  %row19.i.i.i298.i = getelementptr inbounds nuw i8, ptr %self, i64 744
  %117 = getelementptr inbounds nuw i8, ptr %self, i64 748
  %118 = getelementptr inbounds nuw i8, ptr %self, i64 752
  %119 = getelementptr inbounds nuw i8, ptr %self, i64 756
  %row21.i.i.i312.i = getelementptr inbounds nuw i8, ptr %self, i64 760
  %120 = getelementptr inbounds nuw i8, ptr %self, i64 764
  %121 = getelementptr inbounds nuw i8, ptr %self, i64 768
  %122 = getelementptr inbounds nuw i8, ptr %self, i64 772
  %history.i.i692.sroa.7.0.hot_right.i701.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i701, i64 4
  %history.i.i692.sroa.10.0.hot_right.i701.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i701, i64 8
  %history.i.i692.sroa.13.0.hot_right.i701.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i701, i64 12
  %history.i.i692.sroa.16.0.hot_right.i701.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i701, i64 16
  %history.i.i692.sroa.19.0.hot_right.i701.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i701, i64 20
  %history.i.i692.sroa.22.0.hot_right.i701.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i701, i64 24
  %history.i.i692.sroa.26.0.hot_right.i701.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i701, i64 28
  %history.i.i692.sroa.29.0.hot_right.i701.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i701, i64 32
  %history.i.i692.sroa.32.0.hot_right.i701.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i701, i64 36
  %history.i.i692.sroa.35.0.hot_right.i701.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i701, i64 40
  %history.i.i692.sroa.38.0.hot_right.i701.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i701, i64 44
  %_63.i740 = getelementptr inbounds nuw i8, ptr %hot_left.i702, i64 48
  %_64.i741 = getelementptr inbounds nuw i8, ptr %hot_left.i702, i64 64
  %123 = getelementptr inbounds nuw i8, ptr %hot_left.i702, i64 60
  %124 = getelementptr inbounds nuw i8, ptr %hot_left.i702, i64 56
  %125 = getelementptr inbounds nuw i8, ptr %hot_left.i702, i64 52
  %126 = getelementptr inbounds nuw i8, ptr %hot_left.i702, i64 76
  %127 = getelementptr inbounds nuw i8, ptr %hot_left.i702, i64 72
  %128 = getelementptr inbounds nuw i8, ptr %hot_left.i702, i64 68
  %_68.i742 = getelementptr inbounds nuw i8, ptr %hot_right.i701, i64 48
  %_69.i743 = getelementptr inbounds nuw i8, ptr %hot_right.i701, i64 64
  %129 = getelementptr inbounds nuw i8, ptr %hot_right.i701, i64 60
  %130 = getelementptr inbounds nuw i8, ptr %hot_right.i701, i64 56
  %131 = getelementptr inbounds nuw i8, ptr %hot_right.i701, i64 52
  %132 = getelementptr inbounds nuw i8, ptr %hot_right.i701, i64 76
  %133 = getelementptr inbounds nuw i8, ptr %hot_right.i701, i64 72
  %134 = getelementptr inbounds nuw i8, ptr %hot_right.i701, i64 68
  %_9.i3411 = add nsw i32 %_31.i706, -1
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
  %145 = getelementptr inbounds nuw i8, ptr %hot_left.i702, i64 84
  %146 = getelementptr inbounds nuw i8, ptr %hot_left.i702, i64 88
  %147 = getelementptr inbounds nuw i8, ptr %hot_left.i702, i64 80
  %148 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %149 = getelementptr inbounds nuw i8, ptr %self, i64 152
  %_9.i3391 = add nsw i32 %_32.i708, -1
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
  %161 = getelementptr inbounds nuw i8, ptr %hot_right.i701, i64 84
  %162 = getelementptr inbounds nuw i8, ptr %hot_right.i701, i64 88
  %163 = getelementptr inbounds nuw i8, ptr %hot_right.i701, i64 80
  %164 = getelementptr inbounds nuw i8, ptr %self, i64 360
  %165 = getelementptr inbounds nuw i8, ptr %self, i64 352
  %166 = getelementptr inbounds nuw i8, ptr %self, i64 552
  %iter.i32.i693.sroa.0.0.ptr6701.1 = getelementptr inbounds nuw i8, ptr %scratch.i700, i64 4
  %iter.i32.i693.sroa.0.0.ptr6701.2 = getelementptr inbounds nuw i8, ptr %scratch.i700, i64 8
  %iter.i32.i693.sroa.0.0.ptr6701.3 = getelementptr inbounds nuw i8, ptr %scratch.i700, i64 12
  %iter.i32.i693.sroa.0.0.ptr6701.4 = getelementptr inbounds nuw i8, ptr %scratch.i700, i64 16
  %iter.i32.i693.sroa.0.0.ptr6701.5 = getelementptr inbounds nuw i8, ptr %scratch.i700, i64 20
  %iter.i32.i693.sroa.0.0.ptr6701.6 = getelementptr inbounds nuw i8, ptr %scratch.i700, i64 24
  %iter.i32.i693.sroa.0.0.ptr6701.7 = getelementptr inbounds nuw i8, ptr %scratch.i700, i64 28
  %iter.i.i694.sroa.0.0.ptr6712.1 = getelementptr inbounds nuw i8, ptr %scratch.i700, i64 4
  %iter.i.i694.sroa.0.0.ptr6712.2 = getelementptr inbounds nuw i8, ptr %scratch.i700, i64 8
  %iter.i.i694.sroa.0.0.ptr6712.3 = getelementptr inbounds nuw i8, ptr %scratch.i700, i64 12
  %iter.i.i694.sroa.0.0.ptr6712.4 = getelementptr inbounds nuw i8, ptr %scratch.i700, i64 16
  %iter.i.i694.sroa.0.0.ptr6712.5 = getelementptr inbounds nuw i8, ptr %scratch.i700, i64 20
  %iter.i.i694.sroa.0.0.ptr6712.6 = getelementptr inbounds nuw i8, ptr %scratch.i700, i64 24
  %iter.i.i694.sroa.0.0.ptr6712.7 = getelementptr inbounds nuw i8, ptr %scratch.i700, i64 28
  br label %bb37.i725, !dbg !36746

bb16.i733.bb13.i719.loopexit_crit_edge:           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3114
  store float %_0.i2875, ptr %145, align 1, !dbg !36764
  store float %_0.i2871, ptr %161, align 1, !dbg !36785
  store float %_0.i.i3564, ptr %123, align 4, !dbg !36787, !alias.scope !36793, !noalias !36796
  store float %_0.i3336, ptr %_63.i740, align 4, !dbg !36799, !alias.scope !36793, !noalias !36796
  store float %_0.i3329, ptr %124, align 4, !dbg !36801, !alias.scope !36793, !noalias !36796
  store float %_0.i.i3571, ptr %126, align 4, !dbg !36802, !alias.scope !36804, !noalias !36807
  store float %_0.i3349, ptr %_64.i741, align 4, !dbg !36808, !alias.scope !36804, !noalias !36807
  store float %_0.i3342, ptr %127, align 4, !dbg !36809, !alias.scope !36804, !noalias !36807
  store float %_0.i.i3578, ptr %129, align 4, !dbg !36810, !alias.scope !36813, !noalias !36816
  store float %_0.i3362, ptr %_68.i742, align 4, !dbg !36819, !alias.scope !36813, !noalias !36816
  store float %_0.i3355, ptr %130, align 4, !dbg !36820, !alias.scope !36813, !noalias !36816
  store float %_0.i.i3585, ptr %132, align 4, !dbg !36821, !alias.scope !36823, !noalias !36807
  store float %_0.i3375, ptr %_69.i743, align 4, !dbg !36826, !alias.scope !36823, !noalias !36807
  store float %_0.i3368, ptr %133, align 4, !dbg !36827, !alias.scope !36823, !noalias !36807
  br label %bb13.i719.loopexit, !dbg !36828

bb13.i719.loopexit:                               ; preds = %bb16.i733.bb13.i719.loopexit_crit_edge, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i732
  %ring_cursor.sroa.0.1.i735.lcssa = phi i64 [ %spec.store.select13.i947, %bb16.i733.bb13.i719.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i7227561, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i732 ], !dbg !36834
  %main_cursor.sroa.0.1.i736.lcssa = phi i64 [ %spec.store.select.i945, %bb16.i733.bb13.i719.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i7237562, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i732 ], !dbg !36835
  %_111.not.i724 = icmp eq i64 %169, 0, !dbg !36746
  %indvars.iv.next = add i64 %indvars.iv, -32, !dbg !36746
  br i1 %_111.not.i724, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit.loopexit, label %bb37.i725, !dbg !36746

bb37.i725:                                        ; preds = %bb37.i725.lr.ph, %bb13.i719.loopexit
  %indvars.iv = phi i64 [ %frames, %bb37.i725.lr.ph ], [ %indvars.iv.next, %bb13.i719.loopexit ]
  %main_cursor.sroa.0.0.i7237562 = phi i64 [ %85, %bb37.i725.lr.ph ], [ %main_cursor.sroa.0.1.i736.lcssa, %bb13.i719.loopexit ]
  %ring_cursor.sroa.0.0.i7227561 = phi i64 [ %84, %bb37.i725.lr.ph ], [ %ring_cursor.sroa.0.1.i735.lcssa, %bb13.i719.loopexit ]
  %iter2.sroa.0.0.i7217560 = phi i64 [ %yield_count.sroa.0.0.i.i, %bb37.i725.lr.ph ], [ %169, %bb13.i719.loopexit ]
  %iter.sroa.0.0.i7207559 = phi i64 [ 0, %bb37.i725.lr.ph ], [ %168, %bb13.i719.loopexit ]
  %167 = call i64 @llvm.umax.i64(i64 %indvars.iv, i64 1), !dbg !36836
  %umax11097 = call i64 @llvm.umin.i64(i64 %167, i64 32), !dbg !36836
  %168 = add i64 %iter.sroa.0.0.i7207559, 32, !dbg !36836
  %169 = add i64 %iter2.sroa.0.0.i7217560, -1, !dbg !36840
  %history.i136.i.sroa.0.0.copyload = load float, ptr %hot_left.i702, align 4, !dbg !36841, !noalias !36845
  %history.i136.i.sroa.7.0.copyload = load float, ptr %history.i136.i.sroa.7.0.hot_left.i702.sroa_idx, align 4, !dbg !36841, !noalias !36845
  %history.i136.i.sroa.10.0.copyload = load float, ptr %history.i136.i.sroa.10.0.hot_left.i702.sroa_idx, align 4, !dbg !36841, !noalias !36845
  %history.i136.i.sroa.13.0.copyload = load float, ptr %history.i136.i.sroa.13.0.hot_left.i702.sroa_idx, align 4, !dbg !36841, !noalias !36845
  %history.i136.i.sroa.16.0.copyload = load float, ptr %history.i136.i.sroa.16.0.hot_left.i702.sroa_idx, align 4, !dbg !36841, !noalias !36845
  %history.i136.i.sroa.19.0.copyload = load float, ptr %history.i136.i.sroa.19.0.hot_left.i702.sroa_idx, align 4, !dbg !36841, !noalias !36845
  %history.i136.i.sroa.22.0.copyload = load float, ptr %history.i136.i.sroa.22.0.hot_left.i702.sroa_idx, align 4, !dbg !36841, !noalias !36845
  %history.i136.i.sroa.26.0.copyload = load float, ptr %history.i136.i.sroa.26.0.hot_left.i702.sroa_idx, align 4, !dbg !36841, !noalias !36845
  %history.i136.i.sroa.29.0.copyload = load float, ptr %history.i136.i.sroa.29.0.hot_left.i702.sroa_idx, align 4, !dbg !36841, !noalias !36845
  %history.i136.i.sroa.32.0.copyload = load float, ptr %history.i136.i.sroa.32.0.hot_left.i702.sroa_idx, align 4, !dbg !36841, !noalias !36845
  %history.i136.i.sroa.35.0.copyload = load float, ptr %history.i136.i.sroa.35.0.hot_left.i702.sroa_idx, align 4, !dbg !36841, !noalias !36845
  %history.i136.i.sroa.38.0.copyload = load float, ptr %history.i136.i.sroa.38.0.hot_left.i702.sroa_idx, align 4, !dbg !36841, !noalias !36845
  %_20.i139.i6640.not = icmp eq i64 %frames, %iter.sroa.0.0.i7207559, !dbg !36850
  br i1 %_20.i139.i6640.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit340.i, label %bb5.i140.i.lr.ph, !dbg !36860

bb5.i140.i.lr.ph:                                 ; preds = %bb37.i725
  %_11.i.i.i160.i = load float, ptr %_31, align 4
  %_14.i.i.i163.i = load float, ptr %87, align 4
  %_17.i.i.i166.i = load float, ptr %88, align 4
  %_20.i.i.i169.i = load float, ptr %89, align 4
  %_25.i.i.i174.i = load float, ptr %row1.i.i.i172.i, align 4
  %_28.i.i.i177.i = load float, ptr %90, align 4
  %_31.i.i.i180.i = load float, ptr %91, align 4
  %_34.i.i.i183.i = load float, ptr %92, align 4
  %_39.i.i.i188.i = load float, ptr %row3.i.i.i186.i, align 4
  %_42.i.i.i191.i = load float, ptr %93, align 4
  %_45.i.i.i194.i = load float, ptr %94, align 4
  %_48.i.i.i197.i = load float, ptr %95, align 4
  %_53.i.i.i202.i = load float, ptr %row5.i.i.i200.i, align 4
  %_56.i.i.i205.i = load float, ptr %96, align 4
  %_59.i.i.i208.i = load float, ptr %97, align 4
  %_62.i.i.i211.i = load float, ptr %98, align 4
  %_67.i.i.i216.i = load float, ptr %row7.i.i.i214.i, align 4
  %_70.i.i.i219.i = load float, ptr %99, align 4
  %_73.i.i.i222.i = load float, ptr %100, align 4
  %_76.i.i.i225.i = load float, ptr %101, align 4
  %_81.i.i.i230.i = load float, ptr %row9.i.i.i228.i, align 4
  %_84.i.i.i233.i = load float, ptr %102, align 4
  %_87.i.i.i236.i = load float, ptr %103, align 4
  %_90.i.i.i239.i = load float, ptr %104, align 4
  %_95.i.i.i244.i = load float, ptr %row11.i.i.i242.i, align 4
  %_98.i.i.i247.i = load float, ptr %105, align 4
  %_101.i.i.i250.i = load float, ptr %106, align 4
  %_104.i.i.i253.i = load float, ptr %107, align 4
  %_109.i.i.i258.i = load float, ptr %row13.i.i.i256.i, align 4
  %_112.i.i.i261.i = load float, ptr %108, align 4
  %_115.i.i.i264.i = load float, ptr %109, align 4
  %_118.i.i.i267.i = load float, ptr %110, align 4
  %_123.i.i.i272.i = load float, ptr %row15.i.i.i270.i, align 4
  %_126.i.i.i275.i = load float, ptr %111, align 4
  %_129.i.i.i278.i = load float, ptr %112, align 4
  %_132.i.i.i281.i = load float, ptr %113, align 4
  %_137.i.i.i286.i = load float, ptr %row17.i.i.i284.i, align 4
  %_140.i.i.i289.i = load float, ptr %114, align 4
  %_143.i.i.i292.i = load float, ptr %115, align 4
  %_146.i.i.i295.i = load float, ptr %116, align 4
  %_151.i.i.i300.i = load float, ptr %row19.i.i.i298.i, align 4
  %_154.i.i.i303.i = load float, ptr %117, align 4
  %_157.i.i.i306.i = load float, ptr %118, align 4
  %_160.i.i.i309.i = load float, ptr %119, align 4
  %_165.i.i.i314.i = load float, ptr %row21.i.i.i312.i, align 4
  %_168.i.i.i317.i = load float, ptr %120, align 4
  %_171.i.i.i320.i = load float, ptr %121, align 4
  %_174.i.i.i323.i = load float, ptr %122, align 4
  br label %bb5.i140.i, !dbg !36860

bb5.i140.i:                                       ; preds = %bb5.i140.i.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit
  %iter.sroa.0.0.i138.i6652 = phi i64 [ 0, %bb5.i140.i.lr.ph ], [ %170, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.35.06651 = phi float [ %history.i136.i.sroa.35.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.32.06650, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.32.06650 = phi float [ %history.i136.i.sroa.32.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.29.06649, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.29.06649 = phi float [ %history.i136.i.sroa.29.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.26.06648, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.26.06648 = phi float [ %history.i136.i.sroa.26.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.22.06647, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.22.06647 = phi float [ %history.i136.i.sroa.22.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.19.06646, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.19.06646 = phi float [ %history.i136.i.sroa.19.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.16.06645, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.16.06645 = phi float [ %history.i136.i.sroa.16.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.13.06644, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.13.06644 = phi float [ %history.i136.i.sroa.13.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.10.06643, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.10.06643 = phi float [ %history.i136.i.sroa.10.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.7.06642, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.7.06642 = phi float [ %history.i136.i.sroa.7.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.0.06641, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.0.06641 = phi float [ %history.i136.i.sroa.0.0.copyload, %bb5.i140.i.lr.ph ], [ %_0.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ]
  %170 = add nuw nsw i64 %iter.sroa.0.0.i138.i6652, 1, !dbg !36861
  %_11.i141.i = add nuw nsw i64 %iter.sroa.0.0.i138.i6652, %iter.sroa.0.0.i7207559, !dbg !36867
  %_24.i142.i = icmp ugt i64 %_11.i141.i, %left_io.1, !dbg !36869
  br i1 %_24.i142.i, label %bb7.i339.i, label %bb8.i143.i, !dbg !36869, !prof !639

bb8.i143.i:                                       ; preds = %bb5.i140.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36875), !dbg !36878
  %_3.not.i = icmp eq i64 %left_io.1, %_11.i141.i, !dbg !36879
  br i1 %_3.not.i, label %panic.i2949, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit, !dbg !36879

panic.i2949:                                      ; preds = %bb8.i143.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !36879, !noalias !36881
  unreachable, !dbg !36879

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit: ; preds = %bb8.i143.i
  %_31.i145.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %_11.i141.i, !dbg !36883
  %_0.i = load float, ptr %_31.i145.i, align 4, !dbg !36879, !alias.scope !36875, !noalias !36888, !noundef !12
  %171 = tail call noundef float @llvm.fabs.f32(float %history.i136.i.sroa.19.06646), !dbg !36889
  %_0.i2499 = fmul float %_0.i, %_11.i.i.i160.i, !dbg !36895
  %_0.i2075 = fadd float %_0.i2499, 0.000000e+00, !dbg !36907
  %_0.i2498 = fmul float %_0.i, %_14.i.i.i163.i, !dbg !36910
  %_0.i2074 = fadd float %_0.i2498, 0.000000e+00, !dbg !36912
  %_0.i2497 = fmul float %_0.i, %_17.i.i.i166.i, !dbg !36914
  %_0.i2073 = fadd float %_0.i2497, 0.000000e+00, !dbg !36916
  %_0.i2496 = fmul float %_0.i, %_20.i.i.i169.i, !dbg !36918
  %_0.i2072 = fadd float %_0.i2496, 0.000000e+00, !dbg !36920
  %_0.i2495 = fmul float %history.i136.i.sroa.0.06641, %_25.i.i.i174.i, !dbg !36922
  %_0.i2071 = fadd float %_0.i2075, %_0.i2495, !dbg !36926
  %_0.i2494 = fmul float %history.i136.i.sroa.0.06641, %_28.i.i.i177.i, !dbg !36928
  %_0.i2070 = fadd float %_0.i2074, %_0.i2494, !dbg !36930
  %_0.i2493 = fmul float %history.i136.i.sroa.0.06641, %_31.i.i.i180.i, !dbg !36932
  %_0.i2069 = fadd float %_0.i2073, %_0.i2493, !dbg !36934
  %_0.i2492 = fmul float %history.i136.i.sroa.0.06641, %_34.i.i.i183.i, !dbg !36936
  %_0.i2068 = fadd float %_0.i2072, %_0.i2492, !dbg !36938
  %_0.i2491 = fmul float %history.i136.i.sroa.7.06642, %_39.i.i.i188.i, !dbg !36940
  %_0.i2067 = fadd float %_0.i2071, %_0.i2491, !dbg !36944
  %_0.i2490 = fmul float %history.i136.i.sroa.7.06642, %_42.i.i.i191.i, !dbg !36946
  %_0.i2066 = fadd float %_0.i2070, %_0.i2490, !dbg !36948
  %_0.i2489 = fmul float %history.i136.i.sroa.7.06642, %_45.i.i.i194.i, !dbg !36950
  %_0.i2065 = fadd float %_0.i2069, %_0.i2489, !dbg !36952
  %_0.i2488 = fmul float %history.i136.i.sroa.7.06642, %_48.i.i.i197.i, !dbg !36954
  %_0.i2064 = fadd float %_0.i2068, %_0.i2488, !dbg !36956
  %_0.i2487 = fmul float %history.i136.i.sroa.10.06643, %_53.i.i.i202.i, !dbg !36958
  %_0.i2063 = fadd float %_0.i2067, %_0.i2487, !dbg !36962
  %_0.i2486 = fmul float %history.i136.i.sroa.10.06643, %_56.i.i.i205.i, !dbg !36964
  %_0.i2062 = fadd float %_0.i2066, %_0.i2486, !dbg !36966
  %_0.i2485 = fmul float %history.i136.i.sroa.10.06643, %_59.i.i.i208.i, !dbg !36968
  %_0.i2061 = fadd float %_0.i2065, %_0.i2485, !dbg !36970
  %_0.i2484 = fmul float %history.i136.i.sroa.10.06643, %_62.i.i.i211.i, !dbg !36972
  %_0.i2060 = fadd float %_0.i2064, %_0.i2484, !dbg !36974
  %_0.i2483 = fmul float %history.i136.i.sroa.13.06644, %_67.i.i.i216.i, !dbg !36976
  %_0.i2059 = fadd float %_0.i2063, %_0.i2483, !dbg !36980
  %_0.i2482 = fmul float %history.i136.i.sroa.13.06644, %_70.i.i.i219.i, !dbg !36982
  %_0.i2058 = fadd float %_0.i2062, %_0.i2482, !dbg !36984
  %_0.i2481 = fmul float %history.i136.i.sroa.13.06644, %_73.i.i.i222.i, !dbg !36986
  %_0.i2057 = fadd float %_0.i2061, %_0.i2481, !dbg !36988
  %_0.i2480 = fmul float %history.i136.i.sroa.13.06644, %_76.i.i.i225.i, !dbg !36990
  %_0.i2056 = fadd float %_0.i2060, %_0.i2480, !dbg !36992
  %_0.i2479 = fmul float %history.i136.i.sroa.16.06645, %_81.i.i.i230.i, !dbg !36994
  %_0.i2055 = fadd float %_0.i2059, %_0.i2479, !dbg !36998
  %_0.i2478 = fmul float %history.i136.i.sroa.16.06645, %_84.i.i.i233.i, !dbg !37000
  %_0.i2054 = fadd float %_0.i2058, %_0.i2478, !dbg !37002
  %_0.i2477 = fmul float %history.i136.i.sroa.16.06645, %_87.i.i.i236.i, !dbg !37004
  %_0.i2053 = fadd float %_0.i2057, %_0.i2477, !dbg !37006
  %_0.i2476 = fmul float %history.i136.i.sroa.16.06645, %_90.i.i.i239.i, !dbg !37008
  %_0.i2052 = fadd float %_0.i2056, %_0.i2476, !dbg !37010
  %_0.i2475 = fmul float %history.i136.i.sroa.19.06646, %_95.i.i.i244.i, !dbg !37012
  %_0.i2051 = fadd float %_0.i2055, %_0.i2475, !dbg !37016
  %_0.i2474 = fmul float %history.i136.i.sroa.19.06646, %_98.i.i.i247.i, !dbg !37018
  %_0.i2050 = fadd float %_0.i2054, %_0.i2474, !dbg !37020
  %_0.i2473 = fmul float %history.i136.i.sroa.19.06646, %_101.i.i.i250.i, !dbg !37022
  %_0.i2049 = fadd float %_0.i2053, %_0.i2473, !dbg !37024
  %_0.i2472 = fmul float %history.i136.i.sroa.19.06646, %_104.i.i.i253.i, !dbg !37026
  %_0.i2048 = fadd float %_0.i2052, %_0.i2472, !dbg !37028
  %_0.i2471 = fmul float %history.i136.i.sroa.22.06647, %_109.i.i.i258.i, !dbg !37030
  %_0.i2047 = fadd float %_0.i2051, %_0.i2471, !dbg !37034
  %_0.i2470 = fmul float %history.i136.i.sroa.22.06647, %_112.i.i.i261.i, !dbg !37036
  %_0.i2046 = fadd float %_0.i2050, %_0.i2470, !dbg !37038
  %_0.i2469 = fmul float %history.i136.i.sroa.22.06647, %_115.i.i.i264.i, !dbg !37040
  %_0.i2045 = fadd float %_0.i2049, %_0.i2469, !dbg !37042
  %_0.i2468 = fmul float %history.i136.i.sroa.22.06647, %_118.i.i.i267.i, !dbg !37044
  %_0.i2044 = fadd float %_0.i2048, %_0.i2468, !dbg !37046
  %_0.i2467 = fmul float %history.i136.i.sroa.26.06648, %_123.i.i.i272.i, !dbg !37048
  %_0.i2043 = fadd float %_0.i2047, %_0.i2467, !dbg !37052
  %_0.i2466 = fmul float %history.i136.i.sroa.26.06648, %_126.i.i.i275.i, !dbg !37054
  %_0.i2042 = fadd float %_0.i2046, %_0.i2466, !dbg !37056
  %_0.i2465 = fmul float %history.i136.i.sroa.26.06648, %_129.i.i.i278.i, !dbg !37058
  %_0.i2041 = fadd float %_0.i2045, %_0.i2465, !dbg !37060
  %_0.i2464 = fmul float %history.i136.i.sroa.26.06648, %_132.i.i.i281.i, !dbg !37062
  %_0.i2040 = fadd float %_0.i2044, %_0.i2464, !dbg !37064
  %_0.i2463 = fmul float %history.i136.i.sroa.29.06649, %_137.i.i.i286.i, !dbg !37066
  %_0.i2039 = fadd float %_0.i2043, %_0.i2463, !dbg !37070
  %_0.i2462 = fmul float %history.i136.i.sroa.29.06649, %_140.i.i.i289.i, !dbg !37072
  %_0.i2038 = fadd float %_0.i2042, %_0.i2462, !dbg !37074
  %_0.i2461 = fmul float %history.i136.i.sroa.29.06649, %_143.i.i.i292.i, !dbg !37076
  %_0.i2037 = fadd float %_0.i2041, %_0.i2461, !dbg !37078
  %_0.i2460 = fmul float %history.i136.i.sroa.29.06649, %_146.i.i.i295.i, !dbg !37080
  %_0.i2036 = fadd float %_0.i2040, %_0.i2460, !dbg !37082
  %_0.i2459 = fmul float %history.i136.i.sroa.32.06650, %_151.i.i.i300.i, !dbg !37084
  %_0.i2035 = fadd float %_0.i2039, %_0.i2459, !dbg !37088
  %_0.i2458 = fmul float %history.i136.i.sroa.32.06650, %_154.i.i.i303.i, !dbg !37090
  %_0.i2034 = fadd float %_0.i2038, %_0.i2458, !dbg !37092
  %_0.i2457 = fmul float %history.i136.i.sroa.32.06650, %_157.i.i.i306.i, !dbg !37094
  %_0.i2033 = fadd float %_0.i2037, %_0.i2457, !dbg !37096
  %_0.i2456 = fmul float %history.i136.i.sroa.32.06650, %_160.i.i.i309.i, !dbg !37098
  %_0.i2032 = fadd float %_0.i2036, %_0.i2456, !dbg !37100
  %_0.i2455 = fmul float %history.i136.i.sroa.35.06651, %_165.i.i.i314.i, !dbg !37102
  %_0.i2031 = fadd float %_0.i2035, %_0.i2455, !dbg !37106
  %_0.i2454 = fmul float %history.i136.i.sroa.35.06651, %_168.i.i.i317.i, !dbg !37108
  %_0.i2030 = fadd float %_0.i2034, %_0.i2454, !dbg !37110
  %_0.i2453 = fmul float %history.i136.i.sroa.35.06651, %_171.i.i.i320.i, !dbg !37112
  %_0.i2029 = fadd float %_0.i2033, %_0.i2453, !dbg !37114
  %_0.i2452 = fmul float %history.i136.i.sroa.35.06651, %_174.i.i.i323.i, !dbg !37116
  %_0.i2028 = fadd float %_0.i2032, %_0.i2452, !dbg !37118
  %172 = tail call noundef float @llvm.fabs.f32(float %_0.i2031), !dbg !37120
  %_3.i.i3586.inv = fcmp ogt float %171, %172, !dbg !37124
  %_4.i.i.v = select i1 %_3.i.i3586.inv, float %171, float %172, !dbg !37124
  %173 = tail call noundef float @llvm.fabs.f32(float %_0.i2030), !dbg !37120
  %_3.i.i3586.inv.1 = fcmp ogt float %_4.i.i.v, %173, !dbg !37124
  %_4.i.i.v.1 = select i1 %_3.i.i3586.inv.1, float %_4.i.i.v, float %173, !dbg !37124
  %174 = tail call noundef float @llvm.fabs.f32(float %_0.i2029), !dbg !37120
  %_3.i.i3586.inv.2 = fcmp ogt float %_4.i.i.v.1, %174, !dbg !37124
  %_4.i.i.v.2 = select i1 %_3.i.i3586.inv.2, float %_4.i.i.v.1, float %174, !dbg !37124
  %175 = tail call noundef float @llvm.fabs.f32(float %_0.i2028), !dbg !37120
  %_3.i.i3586.inv.3 = fcmp ogt float %_4.i.i.v.2, %175, !dbg !37124
  %_4.i.i.v.3 = select i1 %_3.i.i3586.inv.3, float %_4.i.i.v.2, float %175, !dbg !37124
  %_39.i334.i = getelementptr inbounds nuw float, ptr %peaks_left.i699, i64 %iter.sroa.0.0.i138.i6652, !dbg !37130
  store float %_4.i.i.v.3, ptr %_39.i334.i, align 4, !dbg !37141, !alias.scope !37143, !noalias !36888
  %exitcond.not = icmp eq i64 %170, %umax11097, !dbg !36850
  br i1 %exitcond.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit340.i, label %bb5.i140.i, !dbg !36860

bb7.i339.i:                                       ; preds = %bb5.i140.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_11.i141.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_283c8d9f84e10f29a75d8e6c3a347bea) #30, !dbg !37146, !noalias !36888
  unreachable, !dbg !37146

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit340.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit, %bb37.i725
  %history.i136.i.sroa.0.0.lcssa = phi float [ %history.i136.i.sroa.0.0.copyload, %bb37.i725 ], [ %_0.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !37147
  %history.i136.i.sroa.7.0.lcssa = phi float [ %history.i136.i.sroa.7.0.copyload, %bb37.i725 ], [ %history.i136.i.sroa.0.06641, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !37147
  %history.i136.i.sroa.10.0.lcssa = phi float [ %history.i136.i.sroa.10.0.copyload, %bb37.i725 ], [ %history.i136.i.sroa.7.06642, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !37147
  %history.i136.i.sroa.13.0.lcssa = phi float [ %history.i136.i.sroa.13.0.copyload, %bb37.i725 ], [ %history.i136.i.sroa.10.06643, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !37147
  %history.i136.i.sroa.16.0.lcssa = phi float [ %history.i136.i.sroa.16.0.copyload, %bb37.i725 ], [ %history.i136.i.sroa.13.06644, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !37147
  %history.i136.i.sroa.19.0.lcssa = phi float [ %history.i136.i.sroa.19.0.copyload, %bb37.i725 ], [ %history.i136.i.sroa.16.06645, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !37147
  %history.i136.i.sroa.22.0.lcssa = phi float [ %history.i136.i.sroa.22.0.copyload, %bb37.i725 ], [ %history.i136.i.sroa.19.06646, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !37147
  %history.i136.i.sroa.26.0.lcssa = phi float [ %history.i136.i.sroa.26.0.copyload, %bb37.i725 ], [ %history.i136.i.sroa.22.06647, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !37147
  %history.i136.i.sroa.29.0.lcssa = phi float [ %history.i136.i.sroa.29.0.copyload, %bb37.i725 ], [ %history.i136.i.sroa.26.06648, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !37147
  %history.i136.i.sroa.32.0.lcssa = phi float [ %history.i136.i.sroa.32.0.copyload, %bb37.i725 ], [ %history.i136.i.sroa.29.06649, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !37147
  %history.i136.i.sroa.35.0.lcssa = phi float [ %history.i136.i.sroa.35.0.copyload, %bb37.i725 ], [ %history.i136.i.sroa.32.06650, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !37147
  %history.i136.i.sroa.38.0.lcssa = phi float [ %history.i136.i.sroa.38.0.copyload, %bb37.i725 ], [ %history.i136.i.sroa.35.06651, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !37147
  store float %history.i136.i.sroa.0.0.lcssa, ptr %hot_left.i702, align 4, !dbg !37148, !noalias !36845
  store float %history.i136.i.sroa.7.0.lcssa, ptr %history.i136.i.sroa.7.0.hot_left.i702.sroa_idx, align 4, !dbg !37148, !noalias !36845
  store float %history.i136.i.sroa.10.0.lcssa, ptr %history.i136.i.sroa.10.0.hot_left.i702.sroa_idx, align 4, !dbg !37148, !noalias !36845
  store float %history.i136.i.sroa.13.0.lcssa, ptr %history.i136.i.sroa.13.0.hot_left.i702.sroa_idx, align 4, !dbg !37148, !noalias !36845
  store float %history.i136.i.sroa.16.0.lcssa, ptr %history.i136.i.sroa.16.0.hot_left.i702.sroa_idx, align 4, !dbg !37148, !noalias !36845
  store float %history.i136.i.sroa.19.0.lcssa, ptr %history.i136.i.sroa.19.0.hot_left.i702.sroa_idx, align 4, !dbg !37148, !noalias !36845
  store float %history.i136.i.sroa.22.0.lcssa, ptr %history.i136.i.sroa.22.0.hot_left.i702.sroa_idx, align 4, !dbg !37148, !noalias !36845
  store float %history.i136.i.sroa.26.0.lcssa, ptr %history.i136.i.sroa.26.0.hot_left.i702.sroa_idx, align 4, !dbg !37148, !noalias !36845
  store float %history.i136.i.sroa.29.0.lcssa, ptr %history.i136.i.sroa.29.0.hot_left.i702.sroa_idx, align 4, !dbg !37148, !noalias !36845
  store float %history.i136.i.sroa.32.0.lcssa, ptr %history.i136.i.sroa.32.0.hot_left.i702.sroa_idx, align 4, !dbg !37148, !noalias !36845
  store float %history.i136.i.sroa.35.0.lcssa, ptr %history.i136.i.sroa.35.0.hot_left.i702.sroa_idx, align 4, !dbg !37148, !noalias !36845
  store float %history.i136.i.sroa.38.0.lcssa, ptr %history.i136.i.sroa.38.0.hot_left.i702.sroa_idx, align 4, !dbg !37148, !noalias !36845
  %history.i.i692.sroa.0.0.copyload = load float, ptr %hot_right.i701, align 4, !dbg !37149, !noalias !37151
  %history.i.i692.sroa.7.0.copyload = load float, ptr %history.i.i692.sroa.7.0.hot_right.i701.sroa_idx, align 4, !dbg !37149, !noalias !37151
  %history.i.i692.sroa.10.0.copyload = load float, ptr %history.i.i692.sroa.10.0.hot_right.i701.sroa_idx, align 4, !dbg !37149, !noalias !37151
  %history.i.i692.sroa.13.0.copyload = load float, ptr %history.i.i692.sroa.13.0.hot_right.i701.sroa_idx, align 4, !dbg !37149, !noalias !37151
  %history.i.i692.sroa.16.0.copyload = load float, ptr %history.i.i692.sroa.16.0.hot_right.i701.sroa_idx, align 4, !dbg !37149, !noalias !37151
  %history.i.i692.sroa.19.0.copyload = load float, ptr %history.i.i692.sroa.19.0.hot_right.i701.sroa_idx, align 4, !dbg !37149, !noalias !37151
  %history.i.i692.sroa.22.0.copyload = load float, ptr %history.i.i692.sroa.22.0.hot_right.i701.sroa_idx, align 4, !dbg !37149, !noalias !37151
  %history.i.i692.sroa.26.0.copyload = load float, ptr %history.i.i692.sroa.26.0.hot_right.i701.sroa_idx, align 4, !dbg !37149, !noalias !37151
  %history.i.i692.sroa.29.0.copyload = load float, ptr %history.i.i692.sroa.29.0.hot_right.i701.sroa_idx, align 4, !dbg !37149, !noalias !37151
  %history.i.i692.sroa.32.0.copyload = load float, ptr %history.i.i692.sroa.32.0.hot_right.i701.sroa_idx, align 4, !dbg !37149, !noalias !37151
  %history.i.i692.sroa.35.0.copyload = load float, ptr %history.i.i692.sroa.35.0.hot_right.i701.sroa_idx, align 4, !dbg !37149, !noalias !37151
  %history.i.i692.sroa.38.0.copyload = load float, ptr %history.i.i692.sroa.38.0.hot_right.i701.sroa_idx, align 4, !dbg !37149, !noalias !37151
  br i1 %_20.i139.i6640.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i732, label %bb5.i.i961.lr.ph, !dbg !37156

bb5.i.i961.lr.ph:                                 ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit340.i
  %_11.i.i.i.i979 = load float, ptr %_31, align 4
  %_14.i.i.i.i982 = load float, ptr %87, align 4
  %_17.i.i.i.i985 = load float, ptr %88, align 4
  %_20.i.i.i.i988 = load float, ptr %89, align 4
  %_25.i.i.i.i993 = load float, ptr %row1.i.i.i172.i, align 4
  %_28.i.i.i.i996 = load float, ptr %90, align 4
  %_31.i.i.i.i999 = load float, ptr %91, align 4
  %_34.i.i.i.i1002 = load float, ptr %92, align 4
  %_39.i.i.i.i1007 = load float, ptr %row3.i.i.i186.i, align 4
  %_42.i.i.i.i1010 = load float, ptr %93, align 4
  %_45.i.i.i.i1013 = load float, ptr %94, align 4
  %_48.i.i.i.i1016 = load float, ptr %95, align 4
  %_53.i.i.i.i1021 = load float, ptr %row5.i.i.i200.i, align 4
  %_56.i.i.i.i1024 = load float, ptr %96, align 4
  %_59.i.i.i.i1027 = load float, ptr %97, align 4
  %_62.i.i.i.i1030 = load float, ptr %98, align 4
  %_67.i.i.i.i1035 = load float, ptr %row7.i.i.i214.i, align 4
  %_70.i.i.i.i1038 = load float, ptr %99, align 4
  %_73.i.i.i.i1041 = load float, ptr %100, align 4
  %_76.i.i.i.i1044 = load float, ptr %101, align 4
  %_81.i.i.i.i1049 = load float, ptr %row9.i.i.i228.i, align 4
  %_84.i.i.i.i1052 = load float, ptr %102, align 4
  %_87.i.i.i.i1055 = load float, ptr %103, align 4
  %_90.i.i.i.i1058 = load float, ptr %104, align 4
  %_95.i.i.i.i1063 = load float, ptr %row11.i.i.i242.i, align 4
  %_98.i.i.i.i1066 = load float, ptr %105, align 4
  %_101.i.i.i.i1069 = load float, ptr %106, align 4
  %_104.i.i.i.i1072 = load float, ptr %107, align 4
  %_109.i.i.i.i1077 = load float, ptr %row13.i.i.i256.i, align 4
  %_112.i.i.i.i1080 = load float, ptr %108, align 4
  %_115.i.i.i.i1083 = load float, ptr %109, align 4
  %_118.i.i.i.i1086 = load float, ptr %110, align 4
  %_123.i.i.i.i1091 = load float, ptr %row15.i.i.i270.i, align 4
  %_126.i.i.i.i1094 = load float, ptr %111, align 4
  %_129.i.i.i.i1097 = load float, ptr %112, align 4
  %_132.i.i.i.i1100 = load float, ptr %113, align 4
  %_137.i.i.i.i1105 = load float, ptr %row17.i.i.i284.i, align 4
  %_140.i.i.i.i1108 = load float, ptr %114, align 4
  %_143.i.i.i.i1111 = load float, ptr %115, align 4
  %_146.i.i.i.i1114 = load float, ptr %116, align 4
  %_151.i.i.i.i1119 = load float, ptr %row19.i.i.i298.i, align 4
  %_154.i.i.i.i1122 = load float, ptr %117, align 4
  %_157.i.i.i.i1125 = load float, ptr %118, align 4
  %_160.i.i.i.i1128 = load float, ptr %119, align 4
  %_165.i.i.i.i1133 = load float, ptr %row21.i.i.i312.i, align 4
  %_168.i.i.i.i1136 = load float, ptr %120, align 4
  %_171.i.i.i.i1139 = load float, ptr %121, align 4
  %_174.i.i.i.i1142 = load float, ptr %122, align 4
  br label %bb5.i.i961, !dbg !37156

bb5.i.i961:                                       ; preds = %bb5.i.i961.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954
  %iter.sroa.0.0.i.i7306678 = phi i64 [ 0, %bb5.i.i961.lr.ph ], [ %176, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954 ]
  %history.i.i692.sroa.35.06677 = phi float [ %history.i.i692.sroa.35.0.copyload, %bb5.i.i961.lr.ph ], [ %history.i.i692.sroa.32.06676, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954 ]
  %history.i.i692.sroa.32.06676 = phi float [ %history.i.i692.sroa.32.0.copyload, %bb5.i.i961.lr.ph ], [ %history.i.i692.sroa.29.06675, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954 ]
  %history.i.i692.sroa.29.06675 = phi float [ %history.i.i692.sroa.29.0.copyload, %bb5.i.i961.lr.ph ], [ %history.i.i692.sroa.26.06674, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954 ]
  %history.i.i692.sroa.26.06674 = phi float [ %history.i.i692.sroa.26.0.copyload, %bb5.i.i961.lr.ph ], [ %history.i.i692.sroa.22.06673, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954 ]
  %history.i.i692.sroa.22.06673 = phi float [ %history.i.i692.sroa.22.0.copyload, %bb5.i.i961.lr.ph ], [ %history.i.i692.sroa.19.06672, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954 ]
  %history.i.i692.sroa.19.06672 = phi float [ %history.i.i692.sroa.19.0.copyload, %bb5.i.i961.lr.ph ], [ %history.i.i692.sroa.16.06671, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954 ]
  %history.i.i692.sroa.16.06671 = phi float [ %history.i.i692.sroa.16.0.copyload, %bb5.i.i961.lr.ph ], [ %history.i.i692.sroa.13.06670, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954 ]
  %history.i.i692.sroa.13.06670 = phi float [ %history.i.i692.sroa.13.0.copyload, %bb5.i.i961.lr.ph ], [ %history.i.i692.sroa.10.06669, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954 ]
  %history.i.i692.sroa.10.06669 = phi float [ %history.i.i692.sroa.10.0.copyload, %bb5.i.i961.lr.ph ], [ %history.i.i692.sroa.7.06668, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954 ]
  %history.i.i692.sroa.7.06668 = phi float [ %history.i.i692.sroa.7.0.copyload, %bb5.i.i961.lr.ph ], [ %history.i.i692.sroa.0.06667, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954 ]
  %history.i.i692.sroa.0.06667 = phi float [ %history.i.i692.sroa.0.0.copyload, %bb5.i.i961.lr.ph ], [ %_0.i2952, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954 ]
  %176 = add nuw nsw i64 %iter.sroa.0.0.i.i7306678, 1, !dbg !37159
  %_11.i126.i = add nuw nsw i64 %iter.sroa.0.0.i.i7306678, %iter.sroa.0.0.i7207559, !dbg !37162
  %_24.i.i962 = icmp ugt i64 %_11.i126.i, %right_io.1, !dbg !37163
  br i1 %_24.i.i962, label %bb7.i.i1158, label %bb8.i.i963, !dbg !37163, !prof !639

bb8.i.i963:                                       ; preds = %bb5.i.i961
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37166), !dbg !37169
  %_3.not.i2950 = icmp eq i64 %right_io.1, %_11.i126.i, !dbg !37170
  br i1 %_3.not.i2950, label %panic.i2953, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954, !dbg !37170

panic.i2953:                                      ; preds = %bb8.i.i963
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !37170, !noalias !37172
  unreachable, !dbg !37170

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954: ; preds = %bb8.i.i963
  %_31.i127.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %_11.i126.i, !dbg !37174
  %_0.i2952 = load float, ptr %_31.i127.i, align 4, !dbg !37170, !alias.scope !37166, !noalias !37176, !noundef !12
  %177 = tail call noundef float @llvm.fabs.f32(float %history.i.i692.sroa.19.06672), !dbg !37177
  %_0.i2547 = fmul float %_0.i2952, %_11.i.i.i.i979, !dbg !37180
  %_0.i2123 = fadd float %_0.i2547, 0.000000e+00, !dbg !37183
  %_0.i2546 = fmul float %_0.i2952, %_14.i.i.i.i982, !dbg !37185
  %_0.i2122 = fadd float %_0.i2546, 0.000000e+00, !dbg !37187
  %_0.i2545 = fmul float %_0.i2952, %_17.i.i.i.i985, !dbg !37189
  %_0.i2121 = fadd float %_0.i2545, 0.000000e+00, !dbg !37191
  %_0.i2544 = fmul float %_0.i2952, %_20.i.i.i.i988, !dbg !37193
  %_0.i2120 = fadd float %_0.i2544, 0.000000e+00, !dbg !37195
  %_0.i2543 = fmul float %history.i.i692.sroa.0.06667, %_25.i.i.i.i993, !dbg !37197
  %_0.i2119 = fadd float %_0.i2123, %_0.i2543, !dbg !37199
  %_0.i2542 = fmul float %history.i.i692.sroa.0.06667, %_28.i.i.i.i996, !dbg !37201
  %_0.i2118 = fadd float %_0.i2122, %_0.i2542, !dbg !37203
  %_0.i2541 = fmul float %history.i.i692.sroa.0.06667, %_31.i.i.i.i999, !dbg !37205
  %_0.i2117 = fadd float %_0.i2121, %_0.i2541, !dbg !37207
  %_0.i2540 = fmul float %history.i.i692.sroa.0.06667, %_34.i.i.i.i1002, !dbg !37209
  %_0.i2116 = fadd float %_0.i2120, %_0.i2540, !dbg !37211
  %_0.i2539 = fmul float %history.i.i692.sroa.7.06668, %_39.i.i.i.i1007, !dbg !37213
  %_0.i2115 = fadd float %_0.i2119, %_0.i2539, !dbg !37215
  %_0.i2538 = fmul float %history.i.i692.sroa.7.06668, %_42.i.i.i.i1010, !dbg !37217
  %_0.i2114 = fadd float %_0.i2118, %_0.i2538, !dbg !37219
  %_0.i2537 = fmul float %history.i.i692.sroa.7.06668, %_45.i.i.i.i1013, !dbg !37221
  %_0.i2113 = fadd float %_0.i2117, %_0.i2537, !dbg !37223
  %_0.i2536 = fmul float %history.i.i692.sroa.7.06668, %_48.i.i.i.i1016, !dbg !37225
  %_0.i2112 = fadd float %_0.i2116, %_0.i2536, !dbg !37227
  %_0.i2535 = fmul float %history.i.i692.sroa.10.06669, %_53.i.i.i.i1021, !dbg !37229
  %_0.i2111 = fadd float %_0.i2115, %_0.i2535, !dbg !37231
  %_0.i2534 = fmul float %history.i.i692.sroa.10.06669, %_56.i.i.i.i1024, !dbg !37233
  %_0.i2110 = fadd float %_0.i2114, %_0.i2534, !dbg !37235
  %_0.i2533 = fmul float %history.i.i692.sroa.10.06669, %_59.i.i.i.i1027, !dbg !37237
  %_0.i2109 = fadd float %_0.i2113, %_0.i2533, !dbg !37239
  %_0.i2532 = fmul float %history.i.i692.sroa.10.06669, %_62.i.i.i.i1030, !dbg !37241
  %_0.i2108 = fadd float %_0.i2112, %_0.i2532, !dbg !37243
  %_0.i2531 = fmul float %history.i.i692.sroa.13.06670, %_67.i.i.i.i1035, !dbg !37245
  %_0.i2107 = fadd float %_0.i2111, %_0.i2531, !dbg !37247
  %_0.i2530 = fmul float %history.i.i692.sroa.13.06670, %_70.i.i.i.i1038, !dbg !37249
  %_0.i2106 = fadd float %_0.i2110, %_0.i2530, !dbg !37251
  %_0.i2529 = fmul float %history.i.i692.sroa.13.06670, %_73.i.i.i.i1041, !dbg !37253
  %_0.i2105 = fadd float %_0.i2109, %_0.i2529, !dbg !37255
  %_0.i2528 = fmul float %history.i.i692.sroa.13.06670, %_76.i.i.i.i1044, !dbg !37257
  %_0.i2104 = fadd float %_0.i2108, %_0.i2528, !dbg !37259
  %_0.i2527 = fmul float %history.i.i692.sroa.16.06671, %_81.i.i.i.i1049, !dbg !37261
  %_0.i2103 = fadd float %_0.i2107, %_0.i2527, !dbg !37263
  %_0.i2526 = fmul float %history.i.i692.sroa.16.06671, %_84.i.i.i.i1052, !dbg !37265
  %_0.i2102 = fadd float %_0.i2106, %_0.i2526, !dbg !37267
  %_0.i2525 = fmul float %history.i.i692.sroa.16.06671, %_87.i.i.i.i1055, !dbg !37269
  %_0.i2101 = fadd float %_0.i2105, %_0.i2525, !dbg !37271
  %_0.i2524 = fmul float %history.i.i692.sroa.16.06671, %_90.i.i.i.i1058, !dbg !37273
  %_0.i2100 = fadd float %_0.i2104, %_0.i2524, !dbg !37275
  %_0.i2523 = fmul float %history.i.i692.sroa.19.06672, %_95.i.i.i.i1063, !dbg !37277
  %_0.i2099 = fadd float %_0.i2103, %_0.i2523, !dbg !37279
  %_0.i2522 = fmul float %history.i.i692.sroa.19.06672, %_98.i.i.i.i1066, !dbg !37281
  %_0.i2098 = fadd float %_0.i2102, %_0.i2522, !dbg !37283
  %_0.i2521 = fmul float %history.i.i692.sroa.19.06672, %_101.i.i.i.i1069, !dbg !37285
  %_0.i2097 = fadd float %_0.i2101, %_0.i2521, !dbg !37287
  %_0.i2520 = fmul float %history.i.i692.sroa.19.06672, %_104.i.i.i.i1072, !dbg !37289
  %_0.i2096 = fadd float %_0.i2100, %_0.i2520, !dbg !37291
  %_0.i2519 = fmul float %history.i.i692.sroa.22.06673, %_109.i.i.i.i1077, !dbg !37293
  %_0.i2095 = fadd float %_0.i2099, %_0.i2519, !dbg !37295
  %_0.i2518 = fmul float %history.i.i692.sroa.22.06673, %_112.i.i.i.i1080, !dbg !37297
  %_0.i2094 = fadd float %_0.i2098, %_0.i2518, !dbg !37299
  %_0.i2517 = fmul float %history.i.i692.sroa.22.06673, %_115.i.i.i.i1083, !dbg !37301
  %_0.i2093 = fadd float %_0.i2097, %_0.i2517, !dbg !37303
  %_0.i2516 = fmul float %history.i.i692.sroa.22.06673, %_118.i.i.i.i1086, !dbg !37305
  %_0.i2092 = fadd float %_0.i2096, %_0.i2516, !dbg !37307
  %_0.i2515 = fmul float %history.i.i692.sroa.26.06674, %_123.i.i.i.i1091, !dbg !37309
  %_0.i2091 = fadd float %_0.i2095, %_0.i2515, !dbg !37311
  %_0.i2514 = fmul float %history.i.i692.sroa.26.06674, %_126.i.i.i.i1094, !dbg !37313
  %_0.i2090 = fadd float %_0.i2094, %_0.i2514, !dbg !37315
  %_0.i2513 = fmul float %history.i.i692.sroa.26.06674, %_129.i.i.i.i1097, !dbg !37317
  %_0.i2089 = fadd float %_0.i2093, %_0.i2513, !dbg !37319
  %_0.i2512 = fmul float %history.i.i692.sroa.26.06674, %_132.i.i.i.i1100, !dbg !37321
  %_0.i2088 = fadd float %_0.i2092, %_0.i2512, !dbg !37323
  %_0.i2511 = fmul float %history.i.i692.sroa.29.06675, %_137.i.i.i.i1105, !dbg !37325
  %_0.i2087 = fadd float %_0.i2091, %_0.i2511, !dbg !37327
  %_0.i2510 = fmul float %history.i.i692.sroa.29.06675, %_140.i.i.i.i1108, !dbg !37329
  %_0.i2086 = fadd float %_0.i2090, %_0.i2510, !dbg !37331
  %_0.i2509 = fmul float %history.i.i692.sroa.29.06675, %_143.i.i.i.i1111, !dbg !37333
  %_0.i2085 = fadd float %_0.i2089, %_0.i2509, !dbg !37335
  %_0.i2508 = fmul float %history.i.i692.sroa.29.06675, %_146.i.i.i.i1114, !dbg !37337
  %_0.i2084 = fadd float %_0.i2088, %_0.i2508, !dbg !37339
  %_0.i2507 = fmul float %history.i.i692.sroa.32.06676, %_151.i.i.i.i1119, !dbg !37341
  %_0.i2083 = fadd float %_0.i2087, %_0.i2507, !dbg !37343
  %_0.i2506 = fmul float %history.i.i692.sroa.32.06676, %_154.i.i.i.i1122, !dbg !37345
  %_0.i2082 = fadd float %_0.i2086, %_0.i2506, !dbg !37347
  %_0.i2505 = fmul float %history.i.i692.sroa.32.06676, %_157.i.i.i.i1125, !dbg !37349
  %_0.i2081 = fadd float %_0.i2085, %_0.i2505, !dbg !37351
  %_0.i2504 = fmul float %history.i.i692.sroa.32.06676, %_160.i.i.i.i1128, !dbg !37353
  %_0.i2080 = fadd float %_0.i2084, %_0.i2504, !dbg !37355
  %_0.i2503 = fmul float %history.i.i692.sroa.35.06677, %_165.i.i.i.i1133, !dbg !37357
  %_0.i2079 = fadd float %_0.i2083, %_0.i2503, !dbg !37359
  %_0.i2502 = fmul float %history.i.i692.sroa.35.06677, %_168.i.i.i.i1136, !dbg !37361
  %_0.i2078 = fadd float %_0.i2082, %_0.i2502, !dbg !37363
  %_0.i2501 = fmul float %history.i.i692.sroa.35.06677, %_171.i.i.i.i1139, !dbg !37365
  %_0.i2077 = fadd float %_0.i2081, %_0.i2501, !dbg !37367
  %_0.i2500 = fmul float %history.i.i692.sroa.35.06677, %_174.i.i.i.i1142, !dbg !37369
  %_0.i2076 = fadd float %_0.i2080, %_0.i2500, !dbg !37371
  %178 = tail call noundef float @llvm.fabs.f32(float %_0.i2079), !dbg !37373
  %_3.i.i3594.inv = fcmp ogt float %177, %178, !dbg !37375
  %_4.i.i3601.v = select i1 %_3.i.i3594.inv, float %177, float %178, !dbg !37375
  %179 = tail call noundef float @llvm.fabs.f32(float %_0.i2078), !dbg !37373
  %_3.i.i3594.inv.1 = fcmp ogt float %_4.i.i3601.v, %179, !dbg !37375
  %_4.i.i3601.v.1 = select i1 %_3.i.i3594.inv.1, float %_4.i.i3601.v, float %179, !dbg !37375
  %180 = tail call noundef float @llvm.fabs.f32(float %_0.i2077), !dbg !37373
  %_3.i.i3594.inv.2 = fcmp ogt float %_4.i.i3601.v.1, %180, !dbg !37375
  %_4.i.i3601.v.2 = select i1 %_3.i.i3594.inv.2, float %_4.i.i3601.v.1, float %180, !dbg !37375
  %181 = tail call noundef float @llvm.fabs.f32(float %_0.i2076), !dbg !37373
  %_3.i.i3594.inv.3 = fcmp ogt float %_4.i.i3601.v.2, %181, !dbg !37375
  %_4.i.i3601.v.3 = select i1 %_3.i.i3594.inv.3, float %_4.i.i3601.v.2, float %181, !dbg !37375
  %_39.i.i1153 = getelementptr inbounds nuw float, ptr %peaks_right.i698, i64 %iter.sroa.0.0.i.i7306678, !dbg !37378
  store float %_4.i.i3601.v.3, ptr %_39.i.i1153, align 4, !dbg !37383, !alias.scope !37385, !noalias !37176
  %exitcond11083.not = icmp eq i64 %176, %umax11097, !dbg !37388
  br i1 %exitcond11083.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i732, label %bb5.i.i961, !dbg !37156

bb7.i.i1158:                                      ; preds = %bb5.i.i961
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_11.i126.i, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_283c8d9f84e10f29a75d8e6c3a347bea) #30, !dbg !37390, !noalias !37176
  unreachable, !dbg !37390

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i732: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit340.i
  %history.i.i692.sroa.0.0.lcssa = phi float [ %history.i.i692.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %_0.i2952, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954 ], !dbg !37391
  %history.i.i692.sroa.7.0.lcssa = phi float [ %history.i.i692.sroa.7.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i692.sroa.0.06667, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954 ], !dbg !37391
  %history.i.i692.sroa.10.0.lcssa = phi float [ %history.i.i692.sroa.10.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i692.sroa.7.06668, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954 ], !dbg !37391
  %history.i.i692.sroa.13.0.lcssa = phi float [ %history.i.i692.sroa.13.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i692.sroa.10.06669, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954 ], !dbg !37391
  %history.i.i692.sroa.16.0.lcssa = phi float [ %history.i.i692.sroa.16.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i692.sroa.13.06670, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954 ], !dbg !37391
  %history.i.i692.sroa.19.0.lcssa = phi float [ %history.i.i692.sroa.19.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i692.sroa.16.06671, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954 ], !dbg !37391
  %history.i.i692.sroa.22.0.lcssa = phi float [ %history.i.i692.sroa.22.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i692.sroa.19.06672, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954 ], !dbg !37391
  %history.i.i692.sroa.26.0.lcssa = phi float [ %history.i.i692.sroa.26.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i692.sroa.22.06673, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954 ], !dbg !37391
  %history.i.i692.sroa.29.0.lcssa = phi float [ %history.i.i692.sroa.29.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i692.sroa.26.06674, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954 ], !dbg !37391
  %history.i.i692.sroa.32.0.lcssa = phi float [ %history.i.i692.sroa.32.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i692.sroa.29.06675, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954 ], !dbg !37391
  %history.i.i692.sroa.35.0.lcssa = phi float [ %history.i.i692.sroa.35.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i692.sroa.32.06676, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954 ], !dbg !37391
  %history.i.i692.sroa.38.0.lcssa = phi float [ %history.i.i692.sroa.38.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i692.sroa.35.06677, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2954 ], !dbg !37391
  store float %history.i.i692.sroa.0.0.lcssa, ptr %hot_right.i701, align 4, !dbg !37392, !noalias !37151
  store float %history.i.i692.sroa.7.0.lcssa, ptr %history.i.i692.sroa.7.0.hot_right.i701.sroa_idx, align 4, !dbg !37392, !noalias !37151
  store float %history.i.i692.sroa.10.0.lcssa, ptr %history.i.i692.sroa.10.0.hot_right.i701.sroa_idx, align 4, !dbg !37392, !noalias !37151
  store float %history.i.i692.sroa.13.0.lcssa, ptr %history.i.i692.sroa.13.0.hot_right.i701.sroa_idx, align 4, !dbg !37392, !noalias !37151
  store float %history.i.i692.sroa.16.0.lcssa, ptr %history.i.i692.sroa.16.0.hot_right.i701.sroa_idx, align 4, !dbg !37392, !noalias !37151
  store float %history.i.i692.sroa.19.0.lcssa, ptr %history.i.i692.sroa.19.0.hot_right.i701.sroa_idx, align 4, !dbg !37392, !noalias !37151
  store float %history.i.i692.sroa.22.0.lcssa, ptr %history.i.i692.sroa.22.0.hot_right.i701.sroa_idx, align 4, !dbg !37392, !noalias !37151
  store float %history.i.i692.sroa.26.0.lcssa, ptr %history.i.i692.sroa.26.0.hot_right.i701.sroa_idx, align 4, !dbg !37392, !noalias !37151
  store float %history.i.i692.sroa.29.0.lcssa, ptr %history.i.i692.sroa.29.0.hot_right.i701.sroa_idx, align 4, !dbg !37392, !noalias !37151
  store float %history.i.i692.sroa.32.0.lcssa, ptr %history.i.i692.sroa.32.0.hot_right.i701.sroa_idx, align 4, !dbg !37392, !noalias !37151
  store float %history.i.i692.sroa.35.0.lcssa, ptr %history.i.i692.sroa.35.0.hot_right.i701.sroa_idx, align 4, !dbg !37392, !noalias !37151
  store float %history.i.i692.sroa.38.0.lcssa, ptr %history.i.i692.sroa.38.0.hot_right.i701.sroa_idx, align 4, !dbg !37392, !noalias !37151
  br i1 %_20.i139.i6640.not, label %bb13.i719.loopexit, label %bb43.i747.lr.ph, !dbg !36828

bb43.i747.lr.ph:                                  ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i732
  %_13.i192247514753 = load float, ptr %125, align 4, !alias.scope !36793, !noalias !36796, !noundef !12
  %_13.i191047544756 = load float, ptr %128, align 4, !alias.scope !36804, !noalias !36807, !noundef !12
  %_13.i189847574759 = load float, ptr %131, align 4, !alias.scope !36813, !noalias !36816, !noundef !12
  %_13.i188747604762 = load float, ptr %134, align 4, !alias.scope !36823, !noalias !36807, !noundef !12
  %_86.i761 = load i64, ptr %135, align 8
  %_64.i89.i818 = load float, ptr %146, align 4
  %_64.i.i911 = load float, ptr %162, align 4
  %_101.i943 = load i64, ptr %166, align 8
  %.promoted = load float, ptr %123, align 4, !alias.scope !36793, !noalias !36796
  %_63.i740.promoted = load float, ptr %_63.i740, align 4, !alias.scope !36793, !noalias !36796
  %.promoted6856 = load float, ptr %124, align 4, !alias.scope !36793, !noalias !36796
  %.promoted6926 = load float, ptr %126, align 4, !alias.scope !36804, !noalias !36807
  %_64.i741.promoted = load float, ptr %_64.i741, align 4, !alias.scope !36804, !noalias !36807
  %.promoted7064 = load float, ptr %127, align 4, !alias.scope !36804, !noalias !36807
  %.promoted7134 = load float, ptr %129, align 4, !alias.scope !36813, !noalias !36816
  %_68.i742.promoted = load float, ptr %_68.i742, align 4, !alias.scope !36813, !noalias !36816
  %.promoted7272 = load float, ptr %130, align 4, !alias.scope !36813, !noalias !36816
  %.promoted7342 = load float, ptr %132, align 4, !alias.scope !36823, !noalias !36807
  %_69.i743.promoted = load float, ptr %_69.i743, align 4, !alias.scope !36823, !noalias !36807
  %.promoted7480 = load float, ptr %133, align 4, !alias.scope !36823, !noalias !36807
  %.promoted7550 = load float, ptr %145, align 4
  %.promoted7552 = load float, ptr %147, align 4
  %.promoted7554 = load float, ptr %161, align 4
  %.promoted7556 = load float, ptr %163, align 4
  br label %bb43.i747, !dbg !36828

bb43.i747:                                        ; preds = %bb43.i747.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3114
  %_0.i32457557 = phi float [ %.promoted7556, %bb43.i747.lr.ph ], [ %_0.i3245, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3114 ]
  %_0.i28717555 = phi float [ %.promoted7554, %bb43.i747.lr.ph ], [ %_0.i2871, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3114 ]
  %_0.i32497553 = phi float [ %.promoted7552, %bb43.i747.lr.ph ], [ %_0.i3249, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3114 ]
  %_0.i28757551 = phi float [ %.promoted7550, %bb43.i747.lr.ph ], [ %_0.i2875, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3114 ]
  %_12.i18857481 = phi float [ %.promoted7480, %bb43.i747.lr.ph ], [ %_0.i3368, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3114 ], !dbg !37393
  %_0.i33757412 = phi float [ %_69.i743.promoted, %bb43.i747.lr.ph ], [ %_0.i3375, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3114 ], !dbg !37393
  %_5.i18827343 = phi float [ %.promoted7342, %bb43.i747.lr.ph ], [ %_0.i.i3585, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3114 ], !dbg !37393
  %_12.i18967273 = phi float [ %.promoted7272, %bb43.i747.lr.ph ], [ %_0.i3355, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3114 ], !dbg !37393
  %_0.i33627204 = phi float [ %_68.i742.promoted, %bb43.i747.lr.ph ], [ %_0.i3362, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3114 ], !dbg !37393
  %_5.i18907135 = phi float [ %.promoted7134, %bb43.i747.lr.ph ], [ %_0.i.i3578, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3114 ], !dbg !37393
  %_12.i19087065 = phi float [ %.promoted7064, %bb43.i747.lr.ph ], [ %_0.i3342, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3114 ], !dbg !37393
  %_0.i33496996 = phi float [ %_64.i741.promoted, %bb43.i747.lr.ph ], [ %_0.i3349, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3114 ], !dbg !37393
  %_5.i19026927 = phi float [ %.promoted6926, %bb43.i747.lr.ph ], [ %_0.i.i3571, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3114 ], !dbg !37393
  %_12.i19206857 = phi float [ %.promoted6856, %bb43.i747.lr.ph ], [ %_0.i3329, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3114 ], !dbg !37393
  %_0.i33366788 = phi float [ %_63.i740.promoted, %bb43.i747.lr.ph ], [ %_0.i3336, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3114 ], !dbg !37393
  %_5.i19146719 = phi float [ %.promoted, %bb43.i747.lr.ph ], [ %_0.i.i3564, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3114 ], !dbg !37393
  %main_cursor.sroa.0.1.i7366716 = phi i64 [ %main_cursor.sroa.0.0.i7237562, %bb43.i747.lr.ph ], [ %spec.store.select.i945, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3114 ]
  %ring_cursor.sroa.0.1.i7356715 = phi i64 [ %ring_cursor.sroa.0.0.i7227561, %bb43.i747.lr.ph ], [ %spec.store.select13.i947, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3114 ]
  %iter1.sroa.0.0.i7346714 = phi i64 [ 0, %bb43.i747.lr.ph ], [ %182, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3114 ]
  %182 = add nuw nsw i64 %iter1.sroa.0.0.i7346714, 1, !dbg !37393
  %_59.i739 = add nuw nsw i64 %iter1.sroa.0.0.i7346714, %iter.sroa.0.0.i7207559, !dbg !37399
  %_0.i2864 = fadd float %_5.i19146719, -1.000000e+00, !dbg !37400
  %_3.i.i3558 = fcmp ogt float %_0.i2864, 0.000000e+00, !dbg !37403
  %_0.i.i3564 = select i1 %_3.i.i3558, float %_0.i2864, float 0.000000e+00, !dbg !37407
  %_0.i2024 = fadd float %_0.i33366788, %_12.i19206857, !dbg !37409
  %_0.i3336 = select i1 %_3.i.i3558, float %_0.i2024, float %_13.i192247514753, !dbg !37411
  %_0.i3329 = select i1 %_3.i.i3558, float %_12.i19206857, float 0.000000e+00, !dbg !37413
  %_0.i2865 = fadd float %_5.i19026927, -1.000000e+00, !dbg !37415
  %_3.i.i3565 = fcmp ogt float %_0.i2865, 0.000000e+00, !dbg !37417
  %_0.i.i3571 = select i1 %_3.i.i3565, float %_0.i2865, float 0.000000e+00, !dbg !37420
  %_0.i2025 = fadd float %_0.i33496996, %_12.i19087065, !dbg !37422
  %_0.i3349 = select i1 %_3.i.i3565, float %_0.i2025, float %_13.i191047544756, !dbg !37424
  %_0.i3342 = select i1 %_3.i.i3565, float %_12.i19087065, float 0.000000e+00, !dbg !37426
  %_0.i2866 = fadd float %_5.i18907135, -1.000000e+00, !dbg !37428
  %_3.i.i3572 = fcmp ogt float %_0.i2866, 0.000000e+00, !dbg !37430
  %_0.i.i3578 = select i1 %_3.i.i3572, float %_0.i2866, float 0.000000e+00, !dbg !37433
  %_0.i2026 = fadd float %_0.i33627204, %_12.i18967273, !dbg !37435
  %_0.i3362 = select i1 %_3.i.i3572, float %_0.i2026, float %_13.i189847574759, !dbg !37437
  %_0.i3355 = select i1 %_3.i.i3572, float %_12.i18967273, float 0.000000e+00, !dbg !37439
  %_0.i2867 = fadd float %_5.i18827343, -1.000000e+00, !dbg !37441
  %_3.i.i3579 = fcmp ogt float %_0.i2867, 0.000000e+00, !dbg !37443
  %_0.i.i3585 = select i1 %_3.i.i3579, float %_0.i2867, float 0.000000e+00, !dbg !37446
  %_0.i2027 = fadd float %_0.i33757412, %_12.i18857481, !dbg !37448
  %_0.i3375 = select i1 %_3.i.i3579, float %_0.i2027, float %_13.i188747604762, !dbg !37450
  %_0.i3368 = select i1 %_3.i.i3579, float %_12.i18857481, float 0.000000e+00, !dbg !37452
  %_128.i749 = getelementptr inbounds nuw float, ptr %peaks_left.i699, i64 %iter1.sroa.0.0.i7346714, !dbg !37454
  %_0.i2990 = load float, ptr %_128.i749, align 4, !dbg !37465, !alias.scope !37467, !noalias !36807, !noundef !12
  %_133.i751 = getelementptr inbounds nuw float, ptr %peaks_right.i698, i64 %iter1.sroa.0.0.i7346714, !dbg !37470
  %_0.i2985 = load float, ptr %_133.i751, align 4, !dbg !37480, !alias.scope !37482, !noalias !36807, !noundef !12
  %_3.i.i3621 = fcmp ule float %_0.i2985, %_0.i2990, !dbg !37485
  %_6.i.i3623 = bitcast float %_0.i2985 to i32, !dbg !37488
  %_8.i.i3625 = bitcast float %_0.i2990 to i32, !dbg !37492
  %_4.i.i3628 = select i1 %_3.i.i3621, i32 %_8.i.i3625, i32 %_6.i.i3623, !dbg !37494
  %_5.i3409 = and i32 %_4.i.i3628, %.none.i707, !dbg !37495
  %_7.i3412 = and i32 %_9.i3411, %_8.i.i3625, !dbg !37497
  %_4.i3413 = or disjoint i32 %_5.i3409, %_7.i3412, !dbg !37495
  %_0.i3414 = bitcast i32 %_4.i3413 to float, !dbg !37498
  %_7.i3405 = and i32 %_9.i3411, %_6.i.i3623, !dbg !37501
  %_4.i3406 = or disjoint i32 %_5.i3409, %_7.i3405, !dbg !37503
  %_0.i3407 = bitcast i32 %_4.i3406 to float, !dbg !37504
  %_134.i756 = icmp ugt i64 %_59.i739, %left_io.1, !dbg !37506
  br i1 %_134.i756, label %bb44.i959, label %bb45.i757, !dbg !37506, !prof !639

bb45.i757:                                        ; preds = %bb43.i747
  %_141.i759 = getelementptr inbounds nuw float, ptr %left_io.0, i64 %_59.i739, !dbg !37510
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37515), !dbg !37518
  %_3.not.i2978 = icmp eq i64 %left_io.1, %_59.i739, !dbg !37519
  br i1 %_3.not.i2978, label %panic.i2981, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2982, !dbg !37519

panic.i2981:                                      ; preds = %bb45.i757
  store float %_0.i28757551, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !37519, !noalias !37521
  unreachable, !dbg !37519

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2982: ; preds = %bb45.i757
  %_0.i2980 = load float, ptr %_141.i759, align 4, !dbg !37519, !alias.scope !37515, !noalias !36807, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37522), !dbg !37525
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37526), !dbg !37525
  %width.i33.i762 = load i64, ptr %136, align 8, !dbg !37528, !alias.scope !37529, !noalias !37530, !noundef !12
  %_3.i1990 = fcmp uge float %_0.i3336, %_0.i3414, !dbg !37533
  %_0.i2423 = fdiv float %_0.i3336, %_0.i3414, !dbg !37535
  %_0.i3400 = select i1 %_3.i1990, float 1.000000e+00, float %_0.i2423, !dbg !37538
  %_144.1.i38.i767 = load i64, ptr %137, align 8, !dbg !37540, !alias.scope !37529, !noalias !37530, !noundef !12
  %_22.i39.i768 = mul i64 %width.i33.i762, %ring_cursor.sroa.0.1.i7356715, !dbg !37541
  %_92.i40.i769 = icmp ugt i64 %_22.i39.i768, %_144.1.i38.i767, !dbg !37542
  br i1 %_92.i40.i769, label %bb37.i124.i958, label %bb38.i41.i770, !dbg !37542, !prof !639

bb38.i41.i770:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2982
  %_144.0.i42.i771 = load ptr, ptr %138, align 8, !dbg !37540, !alias.scope !37529, !noalias !37530, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37547), !dbg !37550
  %_4.not.i3139 = icmp eq i64 %_144.1.i38.i767, %_22.i39.i768, !dbg !37551
  br i1 %_4.not.i3139, label %panic.i3141, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3142, !dbg !37551

panic.i3141:                                      ; preds = %bb38.i41.i770
  store float %_0.i28757551, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !37551, !noalias !37553
  unreachable, !dbg !37551

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3142: ; preds = %bb38.i41.i770
  %_99.i44.i773 = getelementptr inbounds nuw float, ptr %_144.0.i42.i771, i64 %_22.i39.i768, !dbg !37554
  store float %_0.i3400, ptr %_99.i44.i773, align 4, !dbg !37551, !alias.scope !37547, !noalias !37559
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37560), !dbg !37563
  %width.i1492 = load i64, ptr %136, align 8, !dbg !37564, !alias.scope !37560, !noalias !37566, !noundef !12
  %183 = icmp eq i64 %width.i1492, 0, !dbg !37568
  br i1 %183, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1600, label %bb32.i1499.lr.ph, !dbg !37568

bb32.i1499.lr.ph:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3142
  %_112.1.i1502 = load i64, ptr %62, align 8, !alias.scope !37560, !noalias !37566, !noundef !12
  %_112.0.i1506 = load ptr, ptr %61, align 8, !nonnull !12
  %184 = add i64 %ring_cursor.sroa.0.1.i7356715, 1
  %_23.not.i1513 = icmp ult i64 %184, %_86.i761
  %185 = select i1 %_23.not.i1513, i64 0, i64 %_86.i761
  %start1.sroa.0.0.i1514 = sub nuw i64 %184, %185
  %_114.1.i1517 = load i64, ptr %137, align 8
  %_114.0.i1521 = load ptr, ptr %138, align 8, !nonnull !12
  %_116.1.i1522 = load i64, ptr %139, align 8
  %_116.0.i1526 = load ptr, ptr %140, align 8, !nonnull !12
  %_118.1.i1530 = load i64, ptr %141, align 8
  %_118.0.i1534 = load ptr, ptr %142, align 8, !nonnull !12
  %_45.i1547 = mul i64 %width.i1492, %start1.sroa.0.0.i1514
  br label %bb32.i1499, !dbg !37568

bb32.i1499:                                       ; preds = %bb32.i1499.lr.ph, %bb31.i1562
  %iter.i1491.sroa.10.06696 = phi i64 [ %width.i1492, %bb32.i1499.lr.ph ], [ %186, %bb31.i1562 ]
  %iter.i1491.sroa.7.06695 = phi i64 [ 0, %bb32.i1499.lr.ph ], [ %_9.0.i, %bb31.i1562 ]
  %iter.i1491.sroa.0.0.idx6694 = phi i64 [ 0, %bb32.i1499.lr.ph ], [ %iter.i1491.sroa.0.0.add, %bb31.i1562 ]
  %iter.i1491.sroa.0.0.ptr6697 = getelementptr inbounds nuw i8, ptr %scratch.i700, i64 %iter.i1491.sroa.0.0.idx6694, !dbg !37570
  %186 = add i64 %iter.i1491.sroa.10.06696, -1, !dbg !37570
  %_7.i.i4055 = icmp eq i64 %iter.i1491.sroa.0.0.idx6694, 32, !dbg !37571
  br i1 %_7.i.i4055, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1600, label %bb3.i1501, !dbg !37575

bb3.i1501:                                        ; preds = %bb32.i1499
  %iter.i1491.sroa.0.0.add = add nuw nsw i64 %iter.i1491.sroa.0.0.idx6694, 4, !dbg !37576
  %_9.0.i = add nuw nsw i64 %iter.i1491.sroa.7.06695, 1, !dbg !37578
  %exitcond11086.not = icmp eq i64 %iter.i1491.sroa.7.06695, %_112.1.i1502, !dbg !37579
  br i1 %exitcond11086.not, label %panic.i1504, label %bb5.i1505, !dbg !37579

bb5.i1505:                                        ; preds = %bb3.i1501
  %187 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i1506, i64 %iter.i1491.sroa.7.06695, !dbg !37579
  %shape.i1507 = load i32, ptr %187, align 4, !dbg !37579, !noalias !37580, !noundef !12
  %188 = getelementptr inbounds nuw i8, ptr %187, i64 4, !dbg !37579
  %shape3.i1508 = load i32, ptr %188, align 4, !dbg !37579, !noalias !37580, !noundef !12
  %window.i1509 = zext i32 %shape.i1507 to i64, !dbg !37581
  %_19.i1510 = zext i32 %shape3.i1508 to i64, !dbg !37582
  %189 = add i64 %ring_cursor.sroa.0.1.i7356715, %_19.i1510, !dbg !37583
  %_20.not.i1511 = icmp ult i64 %189, %_86.i761, !dbg !37584
  %190 = select i1 %_20.not.i1511, i64 0, i64 %_86.i761, !dbg !37584
  %spec.select.i1512 = sub nuw i64 %189, %190, !dbg !37584
  %_27.i1515 = mul i64 %spec.select.i1512, %width.i1492, !dbg !37585
  %_26.i1516 = add i64 %_27.i1515, %iter.i1491.sroa.7.06695, !dbg !37585
  %_30.i1518 = icmp ult i64 %_26.i1516, %_114.1.i1517, !dbg !37586
  br i1 %_30.i1518, label %bb12.i1520, label %panic5.i1519, !dbg !37586

panic.i1504:                                      ; preds = %bb3.i1501
  store float %_0.i28757551, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i1502, i64 noundef %_112.1.i1502, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4a8785a681d008a9bfd0cd82628ea9cb) #30, !dbg !37579, !noalias !37580
  unreachable, !dbg !37579

bb12.i1520:                                       ; preds = %bb5.i1505
  %191 = getelementptr inbounds nuw float, ptr %_114.0.i1521, i64 %_26.i1516, !dbg !37586
  %192 = load float, ptr %191, align 4, !dbg !37586, !noalias !37580, !noundef !12
  %exitcond11087.not = icmp eq i64 %iter.i1491.sroa.7.06695, %_116.1.i1522, !dbg !37587
  br i1 %exitcond11087.not, label %panic6.i1524, label %bb13.i1525, !dbg !37587

panic5.i1519:                                     ; preds = %bb5.i1505
  store float %_0.i28757551, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i1516, i64 noundef %_114.1.i1517, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cbce7773ac40979e4ba2385da3aec116) #30, !dbg !37586, !noalias !37580
  unreachable, !dbg !37586

bb13.i1525:                                       ; preds = %bb12.i1520
  %193 = getelementptr inbounds nuw i32, ptr %_116.0.i1526, i64 %iter.i1491.sroa.7.06695, !dbg !37587
  %_32.i1527 = load i32, ptr %193, align 4, !dbg !37587, !noalias !37580, !noundef !12
  %position.i1528 = zext i32 %_32.i1527 to i64, !dbg !37587
  %194 = icmp eq i32 %_32.i1527, 0, !dbg !37588
  br i1 %194, label %bb17.i1537, label %bb15.i1529, !dbg !37588

panic6.i1524:                                     ; preds = %bb12.i1520
  store float %_0.i28757551, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i1522, i64 noundef %_116.1.i1522, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ec0d48f73ebfc2755df5cedaa60b5c0a) #30, !dbg !37587, !noalias !37580
  unreachable, !dbg !37587

bb15.i1529:                                       ; preds = %bb13.i1525
  %_37.i1531 = icmp ult i64 %iter.i1491.sroa.7.06695, %_118.1.i1530, !dbg !37589
  br i1 %_37.i1531, label %bb16.i1533, label %panic7.i1532, !dbg !37589

bb17.i1537:                                       ; preds = %bb35.i1598, %bb16.i1533, %bb13.i1525
  %newest.sroa.0.0.i1538 = phi float [ %192, %bb13.i1525 ], [ %_35.i1535, %bb35.i1598 ], [ %192, %bb16.i1533 ], !dbg !37590
  %exitcond11088.not = icmp eq i64 %iter.i1491.sroa.7.06695, %_118.1.i1530, !dbg !37591
  br i1 %exitcond11088.not, label %panic8.i1541, label %bb18.i1542, !dbg !37591

bb16.i1533:                                       ; preds = %bb15.i1529
  %195 = getelementptr inbounds nuw float, ptr %_118.0.i1534, i64 %iter.i1491.sroa.7.06695, !dbg !37589
  %_35.i1535 = load float, ptr %195, align 4, !dbg !37589, !noalias !37580, !noundef !12
  %_102.i1536 = fcmp olt float %_35.i1535, %192, !dbg !37592
  br i1 %_102.i1536, label %bb35.i1598, label %bb17.i1537, !dbg !37592

panic7.i1532:                                     ; preds = %bb15.i1529
  store float %_0.i28757551, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i1491.sroa.7.06695, i64 noundef %_118.1.i1530, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2c461872bb652d4796cdcf89c28c82c8) #30, !dbg !37589, !noalias !37580
  unreachable, !dbg !37589

bb35.i1598:                                       ; preds = %bb16.i1533
  br label %bb17.i1537, !dbg !37594

bb18.i1542:                                       ; preds = %bb17.i1537
  %196 = getelementptr inbounds nuw float, ptr %_118.0.i1534, i64 %iter.i1491.sroa.7.06695, !dbg !37591
  store float %newest.sroa.0.0.i1538, ptr %196, align 4, !dbg !37591, !noalias !37580
  %_42.i1544 = add nuw nsw i64 %position.i1528, 1, !dbg !37595
  %complete.i1545 = icmp eq i64 %_42.i1544, %window.i1509, !dbg !37595
  br i1 %complete.i1545, label %bb22.i1567, label %bb20.i1546, !dbg !37596

panic8.i1541:                                     ; preds = %bb17.i1537
  store float %_0.i28757551, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i1530, i64 noundef %_118.1.i1530, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2b690e2c7763f11809942906fc2ca813) #30, !dbg !37591, !noalias !37580
  unreachable, !dbg !37591

bb20.i1546:                                       ; preds = %bb18.i1542
  %_44.i1548 = add i64 %iter.i1491.sroa.7.06695, %_45.i1547, !dbg !37597
  %_47.i1550 = icmp ult i64 %_44.i1548, %_114.1.i1517, !dbg !37598
  br i1 %_47.i1550, label %bb30.i1560, label %panic9.i1551, !dbg !37598

panic9.i1551:                                     ; preds = %bb20.i1546
  store float %_0.i28757551, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i1548, i64 noundef %_114.1.i1517, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9fa421ae81817f58fcfc4a3243223891) #30, !dbg !37598, !noalias !37580
  unreachable, !dbg !37598

bb30.i1560:                                       ; preds = %bb20.i1546
  %197 = getelementptr inbounds nuw float, ptr %_114.0.i1521, i64 %_44.i1548, !dbg !37598
  %_43.i1554 = load float, ptr %197, align 4, !dbg !37598, !noalias !37580, !noundef !12
  %_103.i1555 = fcmp olt float %_43.i1554, %newest.sroa.0.0.i1538, !dbg !37599
  %newest.sroa.0.1.i1556 = select i1 %_103.i1555, float %_43.i1554, float %newest.sroa.0.0.i1538, !dbg !37599
  store float %newest.sroa.0.1.i1556, ptr %iter.i1491.sroa.0.0.ptr6697, align 4, !dbg !37601, !noalias !37580
  %198 = trunc i64 %_42.i1544 to i32, !dbg !37602
  br label %bb31.i1562, !dbg !37603

bb31.i1562:                                       ; preds = %bb25.i1595, %bb30.i1560
  %storemerge = phi i32 [ %198, %bb30.i1560 ], [ 0, %bb25.i1595 ], !dbg !37604
  store i32 %storemerge, ptr %193, align 4, !dbg !37604, !noalias !37580
  %199 = icmp eq i64 %186, 0, !dbg !37568
  br i1 %199, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1600, label %bb32.i1499, !dbg !37568

bb22.i1567:                                       ; preds = %bb18.i1542
  store float %newest.sroa.0.0.i1538, ptr %iter.i1491.sroa.0.0.ptr6697, align 4, !dbg !37601, !noalias !37580
  %200 = load float, ptr %191, align 4, !dbg !37605, !noalias !37580, !noundef !12
  br label %bb41.i1580, !dbg !37606

bb41.i1580:                                       ; preds = %bb22.i1567, %bb25.i1595
  %iter2.sroa.0.0.i15726693 = phi i64 [ 0, %bb22.i1567 ], [ %_105.i1581, %bb25.i1595 ]
  %suffix.sroa.0.0.i15716692 = phi float [ %200, %bb22.i1567 ], [ %suffix.sroa.0.1.i1591, %bb25.i1595 ]
  %end.sroa.0.1.i15706691 = phi i64 [ %spec.select.i1512, %bb22.i1567 ], [ %203, %bb25.i1595 ]
  %_56.i1582 = mul i64 %end.sroa.0.1.i15706691, %width.i1492, !dbg !37609
  %_55.i1583 = add i64 %_56.i1582, %iter.i1491.sroa.7.06695, !dbg !37609
  %_59.i1585 = icmp ult i64 %_55.i1583, %_114.1.i1517, !dbg !37610
  br i1 %_59.i1585, label %bb25.i1595, label %panic13.i1586, !dbg !37610

panic13.i1586:                                    ; preds = %bb41.i1580
  store float %_0.i28757551, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i1583, i64 noundef %_114.1.i1517, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_91a4c6b9b17ebf4d863f9a70b6dc929a) #30, !dbg !37610, !noalias !37580
  unreachable, !dbg !37610

bb25.i1595:                                       ; preds = %bb41.i1580
  %_105.i1581 = add nuw nsw i64 %iter2.sroa.0.0.i15726693, 1, !dbg !37611
  %201 = getelementptr inbounds nuw float, ptr %_114.0.i1521, i64 %_55.i1583, !dbg !37610
  %_54.i1589 = load float, ptr %201, align 4, !dbg !37610, !noalias !37580, !noundef !12
  %_107.i1590 = fcmp olt float %suffix.sroa.0.0.i15716692, %_54.i1589, !dbg !37614
  %suffix.sroa.0.1.i1591 = select i1 %_107.i1590, float %suffix.sroa.0.0.i15716692, float %_54.i1589, !dbg !37614
  store float %suffix.sroa.0.1.i1591, ptr %201, align 4, !dbg !37616, !noalias !37580
  %202 = icmp eq i64 %end.sroa.0.1.i15706691, 0, !dbg !37617
  %spec.store.select.i1597 = select i1 %202, i64 %_86.i761, i64 %end.sroa.0.1.i15706691, !dbg !37617
  %203 = add i64 %spec.store.select.i1597, -1, !dbg !37618
  %exitcond11085.not = icmp eq i64 %_105.i1581, %window.i1509, !dbg !37619
  br i1 %exitcond11085.not, label %bb31.i1562, label %bb41.i1580, !dbg !37606

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1600: ; preds = %bb31.i1562, %bb32.i1499, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3142
  %_0.i2977 = load float, ptr %scratch.i700, align 4, !dbg !37621, !alias.scope !37623, !noalias !37626, !noundef !12
  %_0.i2553 = fmul float %_0.i2977, 1.638400e+04, !dbg !37627
  %204 = tail call noundef float @llvm.floor.f32(float %_0.i2553), !dbg !37629
  %_0.i2552 = fmul float %204, 0x3F10000000000000, !dbg !37636
  %205 = icmp eq i64 %width.i33.i762, 0, !dbg !37638
  %_149.1.i82.i811.pre = load i64, ptr %143, align 8, !dbg !37643, !alias.scope !37529, !noalias !37530
  br i1 %205, label %bb16.i77.i806, label %bb39.i57.i786.lr.ph, !dbg !37638

bb39.i57.i786.lr.ph:                              ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1600
  %_145.1.i60.i789 = load i64, ptr %62, align 8, !alias.scope !37529, !noalias !37530, !noundef !12
  %_145.0.i64.i793 = load ptr, ptr %61, align 8, !nonnull !12
  %_147.0.i75.i804 = load ptr, ptr %144, align 8, !nonnull !12
  %exitcond11089.not = icmp eq i64 %_145.1.i60.i789, 0, !dbg !37644
  br i1 %exitcond11089.not, label %panic.i62.i791, label %bb17.i63.i792, !dbg !37644

bb37.i124.i958:                                   ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2982
  store float %_0.i28757551, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i39.i768, i64 noundef %_144.1.i38.i767, i64 noundef %_144.1.i38.i767, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_56df7c041d29359441bca272bf4e38e3) #30, !dbg !37646, !noalias !37559
  unreachable, !dbg !37646

bb16.i77.i806:                                    ; preds = %bb21.i74.i803.7, %bb21.i74.i803, %bb21.i74.i803.1, %bb21.i74.i803.2, %bb21.i74.i803.3, %bb21.i74.i803.4, %bb21.i74.i803.5, %bb21.i74.i803.6, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1600
  %_0.i2975 = phi float [ %_0.i2977, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1600 ], [ %_49.i76.i805, %bb21.i74.i803 ], [ %_49.i76.i805, %bb21.i74.i803.7 ], [ %_49.i76.i805, %bb21.i74.i803.6 ], [ %_49.i76.i805, %bb21.i74.i803.5 ], [ %_49.i76.i805, %bb21.i74.i803.4 ], [ %_49.i76.i805, %bb21.i74.i803.3 ], [ %_49.i76.i805, %bb21.i74.i803.2 ], [ %_49.i76.i805, %bb21.i74.i803.1 ], !dbg !37647
  %_0.i2125 = fadd float %_0.i2552, %_0.i28757551, !dbg !37649
  %_0.i2875 = fsub float %_0.i2125, %_0.i2975, !dbg !37651
  %_109.i83.i812 = icmp ugt i64 %_22.i39.i768, %_149.1.i82.i811.pre, !dbg !37653
  br i1 %_109.i83.i812, label %bb42.i123.i957, label %bb43.i84.i813, !dbg !37653, !prof !639

bb43.i84.i813:                                    ; preds = %bb16.i77.i806
  %_149.0.i85.i814 = load ptr, ptr %144, align 8, !dbg !37643, !alias.scope !37529, !noalias !37530, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37657), !dbg !37660
  %_4.not.i3135 = icmp eq i64 %_149.1.i82.i811.pre, %_22.i39.i768, !dbg !37661
  br i1 %_4.not.i3135, label %panic.i3137, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3138, !dbg !37661

panic.i3137:                                      ; preds = %bb43.i84.i813
  store float %_0.i2875, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !37661, !noalias !37663
  unreachable, !dbg !37661

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3138: ; preds = %bb43.i84.i813
  %_116.i87.i816 = getelementptr inbounds nuw float, ptr %_149.0.i85.i814, i64 %_22.i39.i768, !dbg !37664
  store float %_0.i2552, ptr %_116.i87.i816, align 4, !dbg !37661, !alias.scope !37657, !noalias !37626
  %_0.i2422 = fdiv float %_0.i2875, %_64.i89.i818, !dbg !37669
  %_0.i2874 = fsub float 1.000000e+00, %_0.i2422, !dbg !37671
  %_0.i2873 = fsub float %_0.i2874, %_0.i32497553, !dbg !37674
  %_4.i2438 = fmul float %_0.i3349, %_0.i2873, !dbg !37677
  %_0.i2439 = fadd float %_0.i32497553, %_4.i2438, !dbg !37677
  %_3.i.i3612.inv = fcmp ogt float %_0.i2874, %_0.i2439, !dbg !37680
  %_4.i.i3619.v = select i1 %_3.i.i3612.inv, float %_0.i2874, float %_0.i2439, !dbg !37680
  %206 = tail call noundef float @llvm.fabs.f32(float %_4.i.i3619.v), !dbg !37684
  %207 = fcmp uge float %206, 0x3BC79CA100000000, !dbg !37688
  %_0.i3249 = select i1 %207, float %_4.i.i3619.v, float 0.000000e+00, !dbg !37691
  store float %_0.i3249, ptr %147, align 4, !dbg !37692, !alias.scope !37522, !noalias !37693
  %_0.i2872 = fsub float 1.000000e+00, %_0.i3249, !dbg !37694
  %_150.1.i101.i830 = load i64, ptr %148, align 8, !dbg !37696, !alias.scope !37529, !noalias !37530, !noundef !12
  %_76.i102.i831 = mul i64 %width.i33.i762, %main_cursor.sroa.0.1.i7366716, !dbg !37698
  %_120.i103.i832 = icmp ugt i64 %_76.i102.i831, %_150.1.i101.i830, !dbg !37699
  br i1 %_120.i103.i832, label %bb48.i122.i956, label %bb49.i104.i833, !dbg !37699, !prof !639

bb42.i123.i957:                                   ; preds = %bb16.i77.i806
  store float %_0.i2875, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i39.i768, i64 noundef %_149.1.i82.i811.pre, i64 noundef %_149.1.i82.i811.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_90498045d73339daaf9e4f537508f58b) #30, !dbg !37704, !noalias !37626
  unreachable, !dbg !37704

bb49.i104.i833:                                   ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3138
  %_150.0.i105.i834 = load ptr, ptr %149, align 8, !dbg !37696, !alias.scope !37529, !noalias !37530, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37705), !dbg !37708
  %_3.not.i2969 = icmp eq i64 %_150.1.i101.i830, %_76.i102.i831, !dbg !37709
  br i1 %_3.not.i2969, label %panic.i2972, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3130, !dbg !37709

panic.i2972:                                      ; preds = %bb49.i104.i833
  store float %_0.i2875, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !37709, !noalias !37711
  unreachable, !dbg !37709

bb48.i122.i956:                                   ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3138
  store float %_0.i2875, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i102.i831, i64 noundef %_150.1.i101.i830, i64 noundef %_150.1.i101.i830, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_da8af254b6d507a8e2ca31e544bfd21d) #30, !dbg !37712, !noalias !37626
  unreachable, !dbg !37712

bb17.i63.i792:                                    ; preds = %bb39.i57.i786.lr.ph
  %208 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i793, i64 8, !dbg !37644
  %_44.i65.i794 = load i32, ptr %208, align 4, !dbg !37644, !noalias !37626, !noundef !12
  %_43.i66.i795 = zext i32 %_44.i65.i794 to i64, !dbg !37644
  %209 = add i64 %ring_cursor.sroa.0.1.i7356715, %_43.i66.i795, !dbg !37713
  %_47.not.i67.i796 = icmp ult i64 %209, %_86.i761, !dbg !37714
  %210 = select i1 %_47.not.i67.i796, i64 0, i64 %_86.i761, !dbg !37714
  %spec.select.i68.i797 = sub nuw i64 %209, %210, !dbg !37714
  %_51.i69.i798 = mul i64 %spec.select.i68.i797, %width.i33.i762, !dbg !37716
  %_53.i72.i801 = icmp ult i64 %_51.i69.i798, %_149.1.i82.i811.pre, !dbg !37717
  br i1 %_53.i72.i801, label %bb21.i74.i803, label %panic1.i73.i802, !dbg !37717

panic.i62.i791:                                   ; preds = %bb39.i57.i786.7, %bb39.i57.i786.6, %bb39.i57.i786.5, %bb39.i57.i786.4, %bb39.i57.i786.3, %bb39.i57.i786.2, %bb39.i57.i786.1, %bb39.i57.i786.lr.ph
  %_145.1.i60.i789.lcssa.ph = phi i64 [ 7, %bb39.i57.i786.7 ], [ 6, %bb39.i57.i786.6 ], [ 5, %bb39.i57.i786.5 ], [ 4, %bb39.i57.i786.4 ], [ 3, %bb39.i57.i786.3 ], [ 2, %bb39.i57.i786.2 ], [ 1, %bb39.i57.i786.1 ], [ 0, %bb39.i57.i786.lr.ph ]
  store float %_0.i28757551, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i60.i789.lcssa.ph, i64 noundef %_145.1.i60.i789.lcssa.ph, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a6feb40b34112df5f214f84424dd2c7c) #30, !dbg !37644, !noalias !37626
  unreachable, !dbg !37644

bb21.i74.i803:                                    ; preds = %bb17.i63.i792
  %211 = getelementptr inbounds nuw float, ptr %_147.0.i75.i804, i64 %_51.i69.i798, !dbg !37717
  %_49.i76.i805 = load float, ptr %211, align 4, !dbg !37717, !noalias !37626, !noundef !12
  store float %_49.i76.i805, ptr %scratch.i700, align 4, !dbg !37718, !noalias !37626
  %212 = icmp eq i64 %width.i33.i762, 1, !dbg !37638
  br i1 %212, label %bb16.i77.i806, label %bb39.i57.i786.1, !dbg !37638

bb39.i57.i786.1:                                  ; preds = %bb21.i74.i803
  %exitcond11089.1.not = icmp eq i64 %_145.1.i60.i789, 1, !dbg !37644
  br i1 %exitcond11089.1.not, label %panic.i62.i791, label %bb17.i63.i792.1, !dbg !37644

bb17.i63.i792.1:                                  ; preds = %bb39.i57.i786.1
  %213 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i793, i64 20, !dbg !37644
  %_44.i65.i794.1 = load i32, ptr %213, align 4, !dbg !37644, !noalias !37626, !noundef !12
  %_43.i66.i795.1 = zext i32 %_44.i65.i794.1 to i64, !dbg !37644
  %214 = add i64 %ring_cursor.sroa.0.1.i7356715, %_43.i66.i795.1, !dbg !37713
  %_47.not.i67.i796.1 = icmp ult i64 %214, %_86.i761, !dbg !37714
  %215 = select i1 %_47.not.i67.i796.1, i64 0, i64 %_86.i761, !dbg !37714
  %spec.select.i68.i797.1 = sub nuw i64 %214, %215, !dbg !37714
  %_51.i69.i798.1 = mul i64 %spec.select.i68.i797.1, %width.i33.i762, !dbg !37716
  %_50.i70.i799.1 = add i64 %_51.i69.i798.1, 1, !dbg !37716
  %_53.i72.i801.1 = icmp ult i64 %_50.i70.i799.1, %_149.1.i82.i811.pre, !dbg !37717
  br i1 %_53.i72.i801.1, label %bb21.i74.i803.1, label %panic1.i73.i802, !dbg !37717

bb21.i74.i803.1:                                  ; preds = %bb17.i63.i792.1
  %216 = getelementptr inbounds nuw float, ptr %_147.0.i75.i804, i64 %_50.i70.i799.1, !dbg !37717
  %_49.i76.i805.1 = load float, ptr %216, align 4, !dbg !37717, !noalias !37626, !noundef !12
  store float %_49.i76.i805.1, ptr %iter.i32.i693.sroa.0.0.ptr6701.1, align 4, !dbg !37718, !noalias !37626
  %217 = icmp eq i64 %width.i33.i762, 2, !dbg !37638
  br i1 %217, label %bb16.i77.i806, label %bb39.i57.i786.2, !dbg !37638

bb39.i57.i786.2:                                  ; preds = %bb21.i74.i803.1
  %exitcond11089.2.not = icmp eq i64 %_145.1.i60.i789, 2, !dbg !37644
  br i1 %exitcond11089.2.not, label %panic.i62.i791, label %bb17.i63.i792.2, !dbg !37644

bb17.i63.i792.2:                                  ; preds = %bb39.i57.i786.2
  %218 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i793, i64 32, !dbg !37644
  %_44.i65.i794.2 = load i32, ptr %218, align 4, !dbg !37644, !noalias !37626, !noundef !12
  %_43.i66.i795.2 = zext i32 %_44.i65.i794.2 to i64, !dbg !37644
  %219 = add i64 %ring_cursor.sroa.0.1.i7356715, %_43.i66.i795.2, !dbg !37713
  %_47.not.i67.i796.2 = icmp ult i64 %219, %_86.i761, !dbg !37714
  %220 = select i1 %_47.not.i67.i796.2, i64 0, i64 %_86.i761, !dbg !37714
  %spec.select.i68.i797.2 = sub nuw i64 %219, %220, !dbg !37714
  %_51.i69.i798.2 = mul i64 %spec.select.i68.i797.2, %width.i33.i762, !dbg !37716
  %_50.i70.i799.2 = add i64 %_51.i69.i798.2, 2, !dbg !37716
  %_53.i72.i801.2 = icmp ult i64 %_50.i70.i799.2, %_149.1.i82.i811.pre, !dbg !37717
  br i1 %_53.i72.i801.2, label %bb21.i74.i803.2, label %panic1.i73.i802, !dbg !37717

bb21.i74.i803.2:                                  ; preds = %bb17.i63.i792.2
  %221 = getelementptr inbounds nuw float, ptr %_147.0.i75.i804, i64 %_50.i70.i799.2, !dbg !37717
  %_49.i76.i805.2 = load float, ptr %221, align 4, !dbg !37717, !noalias !37626, !noundef !12
  store float %_49.i76.i805.2, ptr %iter.i32.i693.sroa.0.0.ptr6701.2, align 4, !dbg !37718, !noalias !37626
  %222 = icmp eq i64 %width.i33.i762, 3, !dbg !37638
  br i1 %222, label %bb16.i77.i806, label %bb39.i57.i786.3, !dbg !37638

bb39.i57.i786.3:                                  ; preds = %bb21.i74.i803.2
  %exitcond11089.3.not = icmp eq i64 %_145.1.i60.i789, 3, !dbg !37644
  br i1 %exitcond11089.3.not, label %panic.i62.i791, label %bb17.i63.i792.3, !dbg !37644

bb17.i63.i792.3:                                  ; preds = %bb39.i57.i786.3
  %223 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i793, i64 44, !dbg !37644
  %_44.i65.i794.3 = load i32, ptr %223, align 4, !dbg !37644, !noalias !37626, !noundef !12
  %_43.i66.i795.3 = zext i32 %_44.i65.i794.3 to i64, !dbg !37644
  %224 = add i64 %ring_cursor.sroa.0.1.i7356715, %_43.i66.i795.3, !dbg !37713
  %_47.not.i67.i796.3 = icmp ult i64 %224, %_86.i761, !dbg !37714
  %225 = select i1 %_47.not.i67.i796.3, i64 0, i64 %_86.i761, !dbg !37714
  %spec.select.i68.i797.3 = sub nuw i64 %224, %225, !dbg !37714
  %_51.i69.i798.3 = mul i64 %spec.select.i68.i797.3, %width.i33.i762, !dbg !37716
  %_50.i70.i799.3 = add i64 %_51.i69.i798.3, 3, !dbg !37716
  %_53.i72.i801.3 = icmp ult i64 %_50.i70.i799.3, %_149.1.i82.i811.pre, !dbg !37717
  br i1 %_53.i72.i801.3, label %bb21.i74.i803.3, label %panic1.i73.i802, !dbg !37717

bb21.i74.i803.3:                                  ; preds = %bb17.i63.i792.3
  %226 = getelementptr inbounds nuw float, ptr %_147.0.i75.i804, i64 %_50.i70.i799.3, !dbg !37717
  %_49.i76.i805.3 = load float, ptr %226, align 4, !dbg !37717, !noalias !37626, !noundef !12
  store float %_49.i76.i805.3, ptr %iter.i32.i693.sroa.0.0.ptr6701.3, align 4, !dbg !37718, !noalias !37626
  %227 = icmp eq i64 %width.i33.i762, 4, !dbg !37638
  br i1 %227, label %bb16.i77.i806, label %bb39.i57.i786.4, !dbg !37638

bb39.i57.i786.4:                                  ; preds = %bb21.i74.i803.3
  %exitcond11089.4.not = icmp eq i64 %_145.1.i60.i789, 4, !dbg !37644
  br i1 %exitcond11089.4.not, label %panic.i62.i791, label %bb17.i63.i792.4, !dbg !37644

bb17.i63.i792.4:                                  ; preds = %bb39.i57.i786.4
  %228 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i793, i64 56, !dbg !37644
  %_44.i65.i794.4 = load i32, ptr %228, align 4, !dbg !37644, !noalias !37626, !noundef !12
  %_43.i66.i795.4 = zext i32 %_44.i65.i794.4 to i64, !dbg !37644
  %229 = add i64 %ring_cursor.sroa.0.1.i7356715, %_43.i66.i795.4, !dbg !37713
  %_47.not.i67.i796.4 = icmp ult i64 %229, %_86.i761, !dbg !37714
  %230 = select i1 %_47.not.i67.i796.4, i64 0, i64 %_86.i761, !dbg !37714
  %spec.select.i68.i797.4 = sub nuw i64 %229, %230, !dbg !37714
  %_51.i69.i798.4 = mul i64 %spec.select.i68.i797.4, %width.i33.i762, !dbg !37716
  %_50.i70.i799.4 = add i64 %_51.i69.i798.4, 4, !dbg !37716
  %_53.i72.i801.4 = icmp ult i64 %_50.i70.i799.4, %_149.1.i82.i811.pre, !dbg !37717
  br i1 %_53.i72.i801.4, label %bb21.i74.i803.4, label %panic1.i73.i802, !dbg !37717

bb21.i74.i803.4:                                  ; preds = %bb17.i63.i792.4
  %231 = getelementptr inbounds nuw float, ptr %_147.0.i75.i804, i64 %_50.i70.i799.4, !dbg !37717
  %_49.i76.i805.4 = load float, ptr %231, align 4, !dbg !37717, !noalias !37626, !noundef !12
  store float %_49.i76.i805.4, ptr %iter.i32.i693.sroa.0.0.ptr6701.4, align 4, !dbg !37718, !noalias !37626
  %232 = icmp eq i64 %width.i33.i762, 5, !dbg !37638
  br i1 %232, label %bb16.i77.i806, label %bb39.i57.i786.5, !dbg !37638

bb39.i57.i786.5:                                  ; preds = %bb21.i74.i803.4
  %exitcond11089.5.not = icmp eq i64 %_145.1.i60.i789, 5, !dbg !37644
  br i1 %exitcond11089.5.not, label %panic.i62.i791, label %bb17.i63.i792.5, !dbg !37644

bb17.i63.i792.5:                                  ; preds = %bb39.i57.i786.5
  %233 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i793, i64 68, !dbg !37644
  %_44.i65.i794.5 = load i32, ptr %233, align 4, !dbg !37644, !noalias !37626, !noundef !12
  %_43.i66.i795.5 = zext i32 %_44.i65.i794.5 to i64, !dbg !37644
  %234 = add i64 %ring_cursor.sroa.0.1.i7356715, %_43.i66.i795.5, !dbg !37713
  %_47.not.i67.i796.5 = icmp ult i64 %234, %_86.i761, !dbg !37714
  %235 = select i1 %_47.not.i67.i796.5, i64 0, i64 %_86.i761, !dbg !37714
  %spec.select.i68.i797.5 = sub nuw i64 %234, %235, !dbg !37714
  %_51.i69.i798.5 = mul i64 %spec.select.i68.i797.5, %width.i33.i762, !dbg !37716
  %_50.i70.i799.5 = add i64 %_51.i69.i798.5, 5, !dbg !37716
  %_53.i72.i801.5 = icmp ult i64 %_50.i70.i799.5, %_149.1.i82.i811.pre, !dbg !37717
  br i1 %_53.i72.i801.5, label %bb21.i74.i803.5, label %panic1.i73.i802, !dbg !37717

bb21.i74.i803.5:                                  ; preds = %bb17.i63.i792.5
  %236 = getelementptr inbounds nuw float, ptr %_147.0.i75.i804, i64 %_50.i70.i799.5, !dbg !37717
  %_49.i76.i805.5 = load float, ptr %236, align 4, !dbg !37717, !noalias !37626, !noundef !12
  store float %_49.i76.i805.5, ptr %iter.i32.i693.sroa.0.0.ptr6701.5, align 4, !dbg !37718, !noalias !37626
  %237 = icmp eq i64 %width.i33.i762, 6, !dbg !37638
  br i1 %237, label %bb16.i77.i806, label %bb39.i57.i786.6, !dbg !37638

bb39.i57.i786.6:                                  ; preds = %bb21.i74.i803.5
  %exitcond11089.6.not = icmp eq i64 %_145.1.i60.i789, 6, !dbg !37644
  br i1 %exitcond11089.6.not, label %panic.i62.i791, label %bb17.i63.i792.6, !dbg !37644

bb17.i63.i792.6:                                  ; preds = %bb39.i57.i786.6
  %238 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i793, i64 80, !dbg !37644
  %_44.i65.i794.6 = load i32, ptr %238, align 4, !dbg !37644, !noalias !37626, !noundef !12
  %_43.i66.i795.6 = zext i32 %_44.i65.i794.6 to i64, !dbg !37644
  %239 = add i64 %ring_cursor.sroa.0.1.i7356715, %_43.i66.i795.6, !dbg !37713
  %_47.not.i67.i796.6 = icmp ult i64 %239, %_86.i761, !dbg !37714
  %240 = select i1 %_47.not.i67.i796.6, i64 0, i64 %_86.i761, !dbg !37714
  %spec.select.i68.i797.6 = sub nuw i64 %239, %240, !dbg !37714
  %_51.i69.i798.6 = mul i64 %spec.select.i68.i797.6, %width.i33.i762, !dbg !37716
  %_50.i70.i799.6 = add i64 %_51.i69.i798.6, 6, !dbg !37716
  %_53.i72.i801.6 = icmp ult i64 %_50.i70.i799.6, %_149.1.i82.i811.pre, !dbg !37717
  br i1 %_53.i72.i801.6, label %bb21.i74.i803.6, label %panic1.i73.i802, !dbg !37717

bb21.i74.i803.6:                                  ; preds = %bb17.i63.i792.6
  %241 = getelementptr inbounds nuw float, ptr %_147.0.i75.i804, i64 %_50.i70.i799.6, !dbg !37717
  %_49.i76.i805.6 = load float, ptr %241, align 4, !dbg !37717, !noalias !37626, !noundef !12
  store float %_49.i76.i805.6, ptr %iter.i32.i693.sroa.0.0.ptr6701.6, align 4, !dbg !37718, !noalias !37626
  %242 = icmp eq i64 %width.i33.i762, 7, !dbg !37638
  br i1 %242, label %bb16.i77.i806, label %bb39.i57.i786.7, !dbg !37638

bb39.i57.i786.7:                                  ; preds = %bb21.i74.i803.6
  %exitcond11089.7.not = icmp eq i64 %_145.1.i60.i789, 7, !dbg !37644
  br i1 %exitcond11089.7.not, label %panic.i62.i791, label %bb17.i63.i792.7, !dbg !37644

bb17.i63.i792.7:                                  ; preds = %bb39.i57.i786.7
  %243 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i793, i64 92, !dbg !37644
  %_44.i65.i794.7 = load i32, ptr %243, align 4, !dbg !37644, !noalias !37626, !noundef !12
  %_43.i66.i795.7 = zext i32 %_44.i65.i794.7 to i64, !dbg !37644
  %244 = add i64 %ring_cursor.sroa.0.1.i7356715, %_43.i66.i795.7, !dbg !37713
  %_47.not.i67.i796.7 = icmp ult i64 %244, %_86.i761, !dbg !37714
  %245 = select i1 %_47.not.i67.i796.7, i64 0, i64 %_86.i761, !dbg !37714
  %spec.select.i68.i797.7 = sub nuw i64 %244, %245, !dbg !37714
  %_51.i69.i798.7 = mul i64 %spec.select.i68.i797.7, %width.i33.i762, !dbg !37716
  %_50.i70.i799.7 = add i64 %_51.i69.i798.7, 7, !dbg !37716
  %_53.i72.i801.7 = icmp ult i64 %_50.i70.i799.7, %_149.1.i82.i811.pre, !dbg !37717
  br i1 %_53.i72.i801.7, label %bb21.i74.i803.7, label %panic1.i73.i802, !dbg !37717

bb21.i74.i803.7:                                  ; preds = %bb17.i63.i792.7
  %246 = getelementptr inbounds nuw float, ptr %_147.0.i75.i804, i64 %_50.i70.i799.7, !dbg !37717
  %_49.i76.i805.7 = load float, ptr %246, align 4, !dbg !37717, !noalias !37626, !noundef !12
  store float %_49.i76.i805.7, ptr %iter.i32.i693.sroa.0.0.ptr6701.7, align 4, !dbg !37718, !noalias !37626
  br label %bb16.i77.i806, !dbg !37638

panic1.i73.i802:                                  ; preds = %bb17.i63.i792.7, %bb17.i63.i792.6, %bb17.i63.i792.5, %bb17.i63.i792.4, %bb17.i63.i792.3, %bb17.i63.i792.2, %bb17.i63.i792.1, %bb17.i63.i792
  %_50.i70.i799.lcssa.ph = phi i64 [ %_50.i70.i799.7, %bb17.i63.i792.7 ], [ %_50.i70.i799.6, %bb17.i63.i792.6 ], [ %_50.i70.i799.5, %bb17.i63.i792.5 ], [ %_50.i70.i799.4, %bb17.i63.i792.4 ], [ %_50.i70.i799.3, %bb17.i63.i792.3 ], [ %_50.i70.i799.2, %bb17.i63.i792.2 ], [ %_50.i70.i799.1, %bb17.i63.i792.1 ], [ %_51.i69.i798, %bb17.i63.i792 ]
  store float %_0.i28757551, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i70.i799.lcssa.ph, i64 noundef %_149.1.i82.i811.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b305c1483509cfb31fdec21ff8752674) #30, !dbg !37717, !noalias !37626
  unreachable, !dbg !37717

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3130: ; preds = %bb49.i104.i833
  %_127.i107.i836 = getelementptr inbounds nuw float, ptr %_150.0.i105.i834, i64 %_76.i102.i831, !dbg !37719
  %_0.i2971 = load float, ptr %_127.i107.i836, align 4, !dbg !37709, !alias.scope !37705, !noalias !37626, !noundef !12
  store float %_0.i2980, ptr %_127.i107.i836, align 4, !dbg !37724, !alias.scope !37727, !noalias !37626
  %_0.i2551 = fmul float %_0.i2872, %_0.i2971, !dbg !37730
  %_6.i3388 = bitcast float %_0.i2971 to i32, !dbg !37732
  %_5.i3389 = and i32 %_6.i3388, %all.sroa.0.0.i709, !dbg !37735
  %_8.i3390 = bitcast float %_0.i2551 to i32, !dbg !37736
  %_7.i3392 = and i32 %_9.i3391, %_8.i3390, !dbg !37738
  %_4.i3393 = or disjoint i32 %_7.i3392, %_5.i3389, !dbg !37735
  store i32 %_4.i3393, ptr %_141.i759, align 4, !dbg !37739, !alias.scope !37741, !noalias !37744
  %_142.i850 = icmp ugt i64 %_59.i739, %right_io.1, !dbg !37745
  br i1 %_142.i850, label %bb46.i953, label %bb47.i851, !dbg !37745, !prof !639

bb44.i959:                                        ; preds = %bb43.i747
  store float %_0.i28757551, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_59.i739, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_0a0a1af21ea56de7dc8d858c82432bb4) #30, !dbg !37749, !noalias !36807
  unreachable, !dbg !37749

bb47.i851:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3130
  %_149.i853 = getelementptr inbounds nuw float, ptr %right_io.0, i64 %_59.i739, !dbg !37750
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37755), !dbg !37758
  %_3.not.i2964 = icmp eq i64 %right_io.1, %_59.i739, !dbg !37759
  br i1 %_3.not.i2964, label %panic.i2967, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2968, !dbg !37759

panic.i2967:                                      ; preds = %bb47.i851
  store float %_0.i2875, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !37759, !noalias !37761
  unreachable, !dbg !37759

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2968: ; preds = %bb47.i851
  %_0.i2966 = load float, ptr %_149.i853, align 4, !dbg !37759, !alias.scope !37755, !noalias !36807, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37762), !dbg !37765
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37766), !dbg !37765
  %width.i.i855 = load i64, ptr %150, align 8, !dbg !37768, !alias.scope !37769, !noalias !37770, !noundef !12
  %_3.i1988 = fcmp uge float %_0.i3362, %_0.i3407, !dbg !37773
  %_0.i2421 = fdiv float %_0.i3362, %_0.i3407, !dbg !37775
  %_0.i3387 = select i1 %_3.i1988, float 1.000000e+00, float %_0.i2421, !dbg !37777
  %_144.1.i.i860 = load i64, ptr %151, align 8, !dbg !37779, !alias.scope !37769, !noalias !37770, !noundef !12
  %_22.i.i861 = mul i64 %width.i.i855, %ring_cursor.sroa.0.1.i7356715, !dbg !37780
  %_92.i.i862 = icmp ugt i64 %_22.i.i861, %_144.1.i.i860, !dbg !37781
  br i1 %_92.i.i862, label %bb37.i.i952, label %bb38.i.i863, !dbg !37781, !prof !639

bb38.i.i863:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2968
  %_144.0.i.i864 = load ptr, ptr %152, align 8, !dbg !37779, !alias.scope !37769, !noalias !37770, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37784), !dbg !37787
  %_4.not.i3123 = icmp eq i64 %_144.1.i.i860, %_22.i.i861, !dbg !37788
  br i1 %_4.not.i3123, label %panic.i3125, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3126, !dbg !37788

panic.i3125:                                      ; preds = %bb38.i.i863
  store float %_0.i2875, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !37788, !noalias !37790
  unreachable, !dbg !37788

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3126: ; preds = %bb38.i.i863
  %_99.i.i866 = getelementptr inbounds nuw float, ptr %_144.0.i.i864, i64 %_22.i.i861, !dbg !37791
  store float %_0.i3387, ptr %_99.i.i866, align 4, !dbg !37788, !alias.scope !37784, !noalias !37793
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37794), !dbg !37797
  %width.i = load i64, ptr %150, align 8, !dbg !37798, !alias.scope !37794, !noalias !37800, !noundef !12
  %247 = icmp eq i64 %width.i, 0, !dbg !37802
  br i1 %247, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit, label %bb32.i1459.lr.ph, !dbg !37802

bb32.i1459.lr.ph:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3126
  %_112.1.i = load i64, ptr %153, align 8, !alias.scope !37794, !noalias !37800, !noundef !12
  %_112.0.i = load ptr, ptr %154, align 8, !nonnull !12
  %248 = add i64 %ring_cursor.sroa.0.1.i7356715, 1
  %_23.not.i = icmp ult i64 %248, %_86.i761
  %249 = select i1 %_23.not.i, i64 0, i64 %_86.i761
  %start1.sroa.0.0.i = sub nuw i64 %248, %249
  %_114.1.i = load i64, ptr %151, align 8
  %_114.0.i = load ptr, ptr %152, align 8, !nonnull !12
  %_116.1.i = load i64, ptr %155, align 8
  %_116.0.i = load ptr, ptr %156, align 8, !nonnull !12
  %_118.1.i = load i64, ptr %157, align 8
  %_118.0.i = load ptr, ptr %158, align 8, !nonnull !12
  %_45.i1477 = mul i64 %width.i, %start1.sroa.0.0.i
  br label %bb32.i1459, !dbg !37802

bb32.i1459:                                       ; preds = %bb32.i1459.lr.ph, %bb31.i1478
  %iter.i1457.sroa.10.06707 = phi i64 [ %width.i, %bb32.i1459.lr.ph ], [ %250, %bb31.i1478 ]
  %iter.i1457.sroa.7.06706 = phi i64 [ 0, %bb32.i1459.lr.ph ], [ %_9.0.i4075, %bb31.i1478 ]
  %iter.i1457.sroa.0.0.idx6705 = phi i64 [ 0, %bb32.i1459.lr.ph ], [ %iter.i1457.sroa.0.0.add, %bb31.i1478 ]
  %iter.i1457.sroa.0.0.ptr6708 = getelementptr inbounds nuw i8, ptr %scratch.i700, i64 %iter.i1457.sroa.0.0.idx6705, !dbg !37804
  %250 = add i64 %iter.i1457.sroa.10.06707, -1, !dbg !37804
  %_7.i.i4071 = icmp eq i64 %iter.i1457.sroa.0.0.idx6705, 32, !dbg !37805
  br i1 %_7.i.i4071, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit, label %bb3.i1461, !dbg !37809

bb3.i1461:                                        ; preds = %bb32.i1459
  %iter.i1457.sroa.0.0.add = add nuw nsw i64 %iter.i1457.sroa.0.0.idx6705, 4, !dbg !37810
  %_9.0.i4075 = add nuw nsw i64 %iter.i1457.sroa.7.06706, 1, !dbg !37812
  %exitcond11092.not = icmp eq i64 %iter.i1457.sroa.7.06706, %_112.1.i, !dbg !37813
  br i1 %exitcond11092.not, label %panic.i, label %bb5.i1462, !dbg !37813

bb5.i1462:                                        ; preds = %bb3.i1461
  %251 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i, i64 %iter.i1457.sroa.7.06706, !dbg !37813
  %shape.i = load i32, ptr %251, align 4, !dbg !37813, !noalias !37814, !noundef !12
  %252 = getelementptr inbounds nuw i8, ptr %251, i64 4, !dbg !37813
  %shape3.i = load i32, ptr %252, align 4, !dbg !37813, !noalias !37814, !noundef !12
  %window.i = zext i32 %shape.i to i64, !dbg !37815
  %_19.i = zext i32 %shape3.i to i64, !dbg !37816
  %253 = add i64 %ring_cursor.sroa.0.1.i7356715, %_19.i, !dbg !37817
  %_20.not.i = icmp ult i64 %253, %_86.i761, !dbg !37818
  %254 = select i1 %_20.not.i, i64 0, i64 %_86.i761, !dbg !37818
  %spec.select.i = sub nuw i64 %253, %254, !dbg !37818
  %_27.i1463 = mul i64 %spec.select.i, %width.i, !dbg !37819
  %_26.i = add i64 %_27.i1463, %iter.i1457.sroa.7.06706, !dbg !37819
  %_30.i1464 = icmp ult i64 %_26.i, %_114.1.i, !dbg !37820
  br i1 %_30.i1464, label %bb12.i1465, label %panic5.i, !dbg !37820

panic.i:                                          ; preds = %bb3.i1461
  store float %_0.i2875, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i, i64 noundef %_112.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4a8785a681d008a9bfd0cd82628ea9cb) #30, !dbg !37813, !noalias !37814
  unreachable, !dbg !37813

bb12.i1465:                                       ; preds = %bb5.i1462
  %255 = getelementptr inbounds nuw float, ptr %_114.0.i, i64 %_26.i, !dbg !37820
  %256 = load float, ptr %255, align 4, !dbg !37820, !noalias !37814, !noundef !12
  %exitcond11093.not = icmp eq i64 %iter.i1457.sroa.7.06706, %_116.1.i, !dbg !37821
  br i1 %exitcond11093.not, label %panic6.i, label %bb13.i1467, !dbg !37821

panic5.i:                                         ; preds = %bb5.i1462
  store float %_0.i2875, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i, i64 noundef %_114.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cbce7773ac40979e4ba2385da3aec116) #30, !dbg !37820, !noalias !37814
  unreachable, !dbg !37820

bb13.i1467:                                       ; preds = %bb12.i1465
  %257 = getelementptr inbounds nuw i32, ptr %_116.0.i, i64 %iter.i1457.sroa.7.06706, !dbg !37821
  %_32.i1468 = load i32, ptr %257, align 4, !dbg !37821, !noalias !37814, !noundef !12
  %position.i1469 = zext i32 %_32.i1468 to i64, !dbg !37821
  %258 = icmp eq i32 %_32.i1468, 0, !dbg !37822
  br i1 %258, label %bb17.i, label %bb15.i1470, !dbg !37822

panic6.i:                                         ; preds = %bb12.i1465
  store float %_0.i2875, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i, i64 noundef %_116.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ec0d48f73ebfc2755df5cedaa60b5c0a) #30, !dbg !37821, !noalias !37814
  unreachable, !dbg !37821

bb15.i1470:                                       ; preds = %bb13.i1467
  %_37.i = icmp ult i64 %iter.i1457.sroa.7.06706, %_118.1.i, !dbg !37823
  br i1 %_37.i, label %bb16.i1471, label %panic7.i, !dbg !37823

bb17.i:                                           ; preds = %bb35.i, %bb16.i1471, %bb13.i1467
  %newest.sroa.0.0.i = phi float [ %256, %bb13.i1467 ], [ %_35.i1472, %bb35.i ], [ %256, %bb16.i1471 ], !dbg !37824
  %exitcond11094.not = icmp eq i64 %iter.i1457.sroa.7.06706, %_118.1.i, !dbg !37825
  br i1 %exitcond11094.not, label %panic8.i, label %bb18.i, !dbg !37825

bb16.i1471:                                       ; preds = %bb15.i1470
  %259 = getelementptr inbounds nuw float, ptr %_118.0.i, i64 %iter.i1457.sroa.7.06706, !dbg !37823
  %_35.i1472 = load float, ptr %259, align 4, !dbg !37823, !noalias !37814, !noundef !12
  %_102.i1473 = fcmp olt float %_35.i1472, %256, !dbg !37826
  br i1 %_102.i1473, label %bb35.i, label %bb17.i, !dbg !37826

panic7.i:                                         ; preds = %bb15.i1470
  store float %_0.i2875, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i1457.sroa.7.06706, i64 noundef %_118.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2c461872bb652d4796cdcf89c28c82c8) #30, !dbg !37823, !noalias !37814
  unreachable, !dbg !37823

bb35.i:                                           ; preds = %bb16.i1471
  br label %bb17.i, !dbg !37828

bb18.i:                                           ; preds = %bb17.i
  %260 = getelementptr inbounds nuw float, ptr %_118.0.i, i64 %iter.i1457.sroa.7.06706, !dbg !37825
  store float %newest.sroa.0.0.i, ptr %260, align 4, !dbg !37825, !noalias !37814
  %_42.i = add nuw nsw i64 %position.i1469, 1, !dbg !37829
  %complete.i1475 = icmp eq i64 %_42.i, %window.i, !dbg !37829
  br i1 %complete.i1475, label %bb22.i, label %bb20.i1476, !dbg !37830

panic8.i:                                         ; preds = %bb17.i
  store float %_0.i2875, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i, i64 noundef %_118.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2b690e2c7763f11809942906fc2ca813) #30, !dbg !37825, !noalias !37814
  unreachable, !dbg !37825

bb20.i1476:                                       ; preds = %bb18.i
  %_44.i = add i64 %iter.i1457.sroa.7.06706, %_45.i1477, !dbg !37831
  %_47.i = icmp ult i64 %_44.i, %_114.1.i, !dbg !37832
  br i1 %_47.i, label %bb30.i, label %panic9.i, !dbg !37832

panic9.i:                                         ; preds = %bb20.i1476
  store float %_0.i2875, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i, i64 noundef %_114.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9fa421ae81817f58fcfc4a3243223891) #30, !dbg !37832, !noalias !37814
  unreachable, !dbg !37832

bb30.i:                                           ; preds = %bb20.i1476
  %261 = getelementptr inbounds nuw float, ptr %_114.0.i, i64 %_44.i, !dbg !37832
  %_43.i = load float, ptr %261, align 4, !dbg !37832, !noalias !37814, !noundef !12
  %_103.i = fcmp olt float %_43.i, %newest.sroa.0.0.i, !dbg !37833
  %newest.sroa.0.1.i = select i1 %_103.i, float %_43.i, float %newest.sroa.0.0.i, !dbg !37833
  store float %newest.sroa.0.1.i, ptr %iter.i1457.sroa.0.0.ptr6708, align 4, !dbg !37835, !noalias !37814
  %262 = trunc i64 %_42.i to i32, !dbg !37836
  br label %bb31.i1478, !dbg !37837

bb31.i1478:                                       ; preds = %bb25.i, %bb30.i
  %storemerge4767 = phi i32 [ %262, %bb30.i ], [ 0, %bb25.i ], !dbg !37838
  store i32 %storemerge4767, ptr %257, align 4, !dbg !37838, !noalias !37814
  %263 = icmp eq i64 %250, 0, !dbg !37802
  br i1 %263, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit, label %bb32.i1459, !dbg !37802

bb22.i:                                           ; preds = %bb18.i
  store float %newest.sroa.0.0.i, ptr %iter.i1457.sroa.0.0.ptr6708, align 4, !dbg !37835, !noalias !37814
  %264 = load float, ptr %255, align 4, !dbg !37839, !noalias !37814, !noundef !12
  br label %bb41.i, !dbg !37840

bb41.i:                                           ; preds = %bb22.i, %bb25.i
  %iter2.sroa.0.0.i14816704 = phi i64 [ 0, %bb22.i ], [ %_105.i1485, %bb25.i ]
  %suffix.sroa.0.0.i14806703 = phi float [ %264, %bb22.i ], [ %suffix.sroa.0.1.i, %bb25.i ]
  %end.sroa.0.1.i6702 = phi i64 [ %spec.select.i, %bb22.i ], [ %267, %bb25.i ]
  %_56.i = mul i64 %end.sroa.0.1.i6702, %width.i, !dbg !37843
  %_55.i = add i64 %_56.i, %iter.i1457.sroa.7.06706, !dbg !37843
  %_59.i1486 = icmp ult i64 %_55.i, %_114.1.i, !dbg !37844
  br i1 %_59.i1486, label %bb25.i, label %panic13.i, !dbg !37844

panic13.i:                                        ; preds = %bb41.i
  store float %_0.i2875, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i, i64 noundef %_114.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_91a4c6b9b17ebf4d863f9a70b6dc929a) #30, !dbg !37844, !noalias !37814
  unreachable, !dbg !37844

bb25.i:                                           ; preds = %bb41.i
  %_105.i1485 = add nuw nsw i64 %iter2.sroa.0.0.i14816704, 1, !dbg !37845
  %265 = getelementptr inbounds nuw float, ptr %_114.0.i, i64 %_55.i, !dbg !37844
  %_54.i = load float, ptr %265, align 4, !dbg !37844, !noalias !37814, !noundef !12
  %_107.i1487 = fcmp olt float %suffix.sroa.0.0.i14806703, %_54.i, !dbg !37848
  %suffix.sroa.0.1.i = select i1 %_107.i1487, float %suffix.sroa.0.0.i14806703, float %_54.i, !dbg !37848
  store float %suffix.sroa.0.1.i, ptr %265, align 4, !dbg !37850, !noalias !37814
  %266 = icmp eq i64 %end.sroa.0.1.i6702, 0, !dbg !37851
  %spec.store.select.i1489 = select i1 %266, i64 %_86.i761, i64 %end.sroa.0.1.i6702, !dbg !37851
  %267 = add i64 %spec.store.select.i1489, -1, !dbg !37852
  %exitcond11091.not = icmp eq i64 %_105.i1485, %window.i, !dbg !37853
  br i1 %exitcond11091.not, label %bb31.i1478, label %bb41.i, !dbg !37840

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit: ; preds = %bb32.i1459, %bb31.i1478
  %_0.i2963.pre = load float, ptr %scratch.i700, align 4, !dbg !37855, !alias.scope !37857, !noalias !37860
  br label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit, !dbg !37855

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit: ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3126
  %_0.i2963 = phi float [ %_0.i2963.pre, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit ], [ %_0.i2975, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3126 ], !dbg !37855
  %_0.i2550 = fmul float %_0.i2963, 1.638400e+04, !dbg !37861
  %268 = tail call noundef float @llvm.floor.f32(float %_0.i2550), !dbg !37863
  %_0.i2549 = fmul float %268, 0x3F10000000000000, !dbg !37867
  %269 = icmp eq i64 %width.i.i855, 0, !dbg !37869
  %_149.1.i.i904.pre = load i64, ptr %159, align 8, !dbg !37871, !alias.scope !37769, !noalias !37770
  br i1 %269, label %bb16.i.i899, label %bb39.i.i879.lr.ph, !dbg !37869

bb39.i.i879.lr.ph:                                ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit
  %_145.1.i.i882 = load i64, ptr %153, align 8, !alias.scope !37769, !noalias !37770, !noundef !12
  %_145.0.i.i886 = load ptr, ptr %154, align 8, !nonnull !12
  %_147.0.i.i897 = load ptr, ptr %160, align 8, !nonnull !12
  %exitcond11095.not = icmp eq i64 %_145.1.i.i882, 0, !dbg !37872
  br i1 %exitcond11095.not, label %panic.i.i884, label %bb17.i.i885, !dbg !37872

bb37.i.i952:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2968
  store float %_0.i2875, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i861, i64 noundef %_144.1.i.i860, i64 noundef %_144.1.i.i860, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_56df7c041d29359441bca272bf4e38e3) #30, !dbg !37873, !noalias !37793
  unreachable, !dbg !37873

bb16.i.i899:                                      ; preds = %bb21.i.i896.7, %bb21.i.i896, %bb21.i.i896.1, %bb21.i.i896.2, %bb21.i.i896.3, %bb21.i.i896.4, %bb21.i.i896.5, %bb21.i.i896.6, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit
  %_0.i2961 = phi float [ %_0.i2963, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit ], [ %_49.i.i898, %bb21.i.i896 ], [ %_49.i.i898, %bb21.i.i896.7 ], [ %_49.i.i898, %bb21.i.i896.6 ], [ %_49.i.i898, %bb21.i.i896.5 ], [ %_49.i.i898, %bb21.i.i896.4 ], [ %_49.i.i898, %bb21.i.i896.3 ], [ %_49.i.i898, %bb21.i.i896.2 ], [ %_49.i.i898, %bb21.i.i896.1 ], !dbg !37874
  %_0.i2124 = fadd float %_0.i2549, %_0.i28717555, !dbg !37876
  %_0.i2871 = fsub float %_0.i2124, %_0.i2961, !dbg !37878
  %_109.i.i905 = icmp ugt i64 %_22.i.i861, %_149.1.i.i904.pre, !dbg !37880
  br i1 %_109.i.i905, label %bb42.i.i951, label %bb43.i.i906, !dbg !37880, !prof !639

bb43.i.i906:                                      ; preds = %bb16.i.i899
  %_149.0.i.i907 = load ptr, ptr %160, align 8, !dbg !37871, !alias.scope !37769, !noalias !37770, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37883), !dbg !37886
  %_4.not.i3119 = icmp eq i64 %_149.1.i.i904.pre, %_22.i.i861, !dbg !37887
  br i1 %_4.not.i3119, label %panic.i3121, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3122, !dbg !37887

panic.i3121:                                      ; preds = %bb43.i.i906
  store float %_0.i2875, ptr %145, align 1, !dbg !36764
  store float %_0.i2871, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !37887, !noalias !37889
  unreachable, !dbg !37887

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3122: ; preds = %bb43.i.i906
  %_116.i.i909 = getelementptr inbounds nuw float, ptr %_149.0.i.i907, i64 %_22.i.i861, !dbg !37890
  store float %_0.i2549, ptr %_116.i.i909, align 4, !dbg !37887, !alias.scope !37883, !noalias !37860
  %_0.i2420 = fdiv float %_0.i2871, %_64.i.i911, !dbg !37892
  %_0.i2870 = fsub float 1.000000e+00, %_0.i2420, !dbg !37894
  %_0.i2869 = fsub float %_0.i2870, %_0.i32457557, !dbg !37896
  %_4.i2436 = fmul float %_0.i3375, %_0.i2869, !dbg !37898
  %_0.i2437 = fadd float %_0.i32457557, %_4.i2436, !dbg !37898
  %_3.i.i3603.inv = fcmp ogt float %_0.i2870, %_0.i2437, !dbg !37900
  %_4.i.i3610.v = select i1 %_3.i.i3603.inv, float %_0.i2870, float %_0.i2437, !dbg !37900
  %270 = tail call noundef float @llvm.fabs.f32(float %_4.i.i3610.v), !dbg !37903
  %271 = fcmp uge float %270, 0x3BC79CA100000000, !dbg !37906
  %_0.i3245 = select i1 %271, float %_4.i.i3610.v, float 0.000000e+00, !dbg !37908
  store float %_0.i3245, ptr %163, align 4, !dbg !37909, !alias.scope !37762, !noalias !37910
  %_0.i2868 = fsub float 1.000000e+00, %_0.i3245, !dbg !37911
  %_150.1.i.i923 = load i64, ptr %164, align 8, !dbg !37913, !alias.scope !37769, !noalias !37770, !noundef !12
  %_76.i.i924 = mul i64 %width.i.i855, %main_cursor.sroa.0.1.i7366716, !dbg !37914
  %_120.i.i925 = icmp ugt i64 %_76.i.i924, %_150.1.i.i923, !dbg !37915
  br i1 %_120.i.i925, label %bb48.i.i950, label %bb49.i.i926, !dbg !37915, !prof !639

bb42.i.i951:                                      ; preds = %bb16.i.i899
  store float %_0.i2875, ptr %145, align 1, !dbg !36764
  store float %_0.i2871, ptr %161, align 1, !dbg !36785
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i861, i64 noundef %_149.1.i.i904.pre, i64 noundef %_149.1.i.i904.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_90498045d73339daaf9e4f537508f58b) #30, !dbg !37918, !noalias !37860
  unreachable, !dbg !37918

bb49.i.i926:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3122
  %_150.0.i.i927 = load ptr, ptr %165, align 8, !dbg !37913, !alias.scope !37769, !noalias !37770, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37919), !dbg !37922
  %_3.not.i2955 = icmp eq i64 %_150.1.i.i923, %_76.i.i924, !dbg !37923
  br i1 %_3.not.i2955, label %panic.i2958, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3114, !dbg !37923

panic.i2958:                                      ; preds = %bb49.i.i926
  store float %_0.i2875, ptr %145, align 1, !dbg !36764
  store float %_0.i2871, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !37923, !noalias !37925
  unreachable, !dbg !37923

bb48.i.i950:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3122
  store float %_0.i2875, ptr %145, align 1, !dbg !36764
  store float %_0.i2871, ptr %161, align 1, !dbg !36785
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i.i924, i64 noundef %_150.1.i.i923, i64 noundef %_150.1.i.i923, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_da8af254b6d507a8e2ca31e544bfd21d) #30, !dbg !37926, !noalias !37860
  unreachable, !dbg !37926

bb17.i.i885:                                      ; preds = %bb39.i.i879.lr.ph
  %272 = getelementptr inbounds nuw i8, ptr %_145.0.i.i886, i64 8, !dbg !37872
  %_44.i.i887 = load i32, ptr %272, align 4, !dbg !37872, !noalias !37860, !noundef !12
  %_43.i.i888 = zext i32 %_44.i.i887 to i64, !dbg !37872
  %273 = add i64 %ring_cursor.sroa.0.1.i7356715, %_43.i.i888, !dbg !37927
  %_47.not.i.i889 = icmp ult i64 %273, %_86.i761, !dbg !37928
  %274 = select i1 %_47.not.i.i889, i64 0, i64 %_86.i761, !dbg !37928
  %spec.select.i.i890 = sub nuw i64 %273, %274, !dbg !37928
  %_51.i.i891 = mul i64 %spec.select.i.i890, %width.i.i855, !dbg !37929
  %_53.i.i894 = icmp ult i64 %_51.i.i891, %_149.1.i.i904.pre, !dbg !37930
  br i1 %_53.i.i894, label %bb21.i.i896, label %panic1.i.i895, !dbg !37930

panic.i.i884:                                     ; preds = %bb39.i.i879.7, %bb39.i.i879.6, %bb39.i.i879.5, %bb39.i.i879.4, %bb39.i.i879.3, %bb39.i.i879.2, %bb39.i.i879.1, %bb39.i.i879.lr.ph
  %_145.1.i.i882.lcssa.ph = phi i64 [ 7, %bb39.i.i879.7 ], [ 6, %bb39.i.i879.6 ], [ 5, %bb39.i.i879.5 ], [ 4, %bb39.i.i879.4 ], [ 3, %bb39.i.i879.3 ], [ 2, %bb39.i.i879.2 ], [ 1, %bb39.i.i879.1 ], [ 0, %bb39.i.i879.lr.ph ]
  store float %_0.i2875, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i.i882.lcssa.ph, i64 noundef %_145.1.i.i882.lcssa.ph, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a6feb40b34112df5f214f84424dd2c7c) #30, !dbg !37872, !noalias !37860
  unreachable, !dbg !37872

bb21.i.i896:                                      ; preds = %bb17.i.i885
  %275 = getelementptr inbounds nuw float, ptr %_147.0.i.i897, i64 %_51.i.i891, !dbg !37930
  %_49.i.i898 = load float, ptr %275, align 4, !dbg !37930, !noalias !37860, !noundef !12
  store float %_49.i.i898, ptr %scratch.i700, align 4, !dbg !37931, !noalias !37860
  %276 = icmp eq i64 %width.i.i855, 1, !dbg !37869
  br i1 %276, label %bb16.i.i899, label %bb39.i.i879.1, !dbg !37869

bb39.i.i879.1:                                    ; preds = %bb21.i.i896
  %exitcond11095.1.not = icmp eq i64 %_145.1.i.i882, 1, !dbg !37872
  br i1 %exitcond11095.1.not, label %panic.i.i884, label %bb17.i.i885.1, !dbg !37872

bb17.i.i885.1:                                    ; preds = %bb39.i.i879.1
  %277 = getelementptr inbounds nuw i8, ptr %_145.0.i.i886, i64 20, !dbg !37872
  %_44.i.i887.1 = load i32, ptr %277, align 4, !dbg !37872, !noalias !37860, !noundef !12
  %_43.i.i888.1 = zext i32 %_44.i.i887.1 to i64, !dbg !37872
  %278 = add i64 %ring_cursor.sroa.0.1.i7356715, %_43.i.i888.1, !dbg !37927
  %_47.not.i.i889.1 = icmp ult i64 %278, %_86.i761, !dbg !37928
  %279 = select i1 %_47.not.i.i889.1, i64 0, i64 %_86.i761, !dbg !37928
  %spec.select.i.i890.1 = sub nuw i64 %278, %279, !dbg !37928
  %_51.i.i891.1 = mul i64 %spec.select.i.i890.1, %width.i.i855, !dbg !37929
  %_50.i.i892.1 = add i64 %_51.i.i891.1, 1, !dbg !37929
  %_53.i.i894.1 = icmp ult i64 %_50.i.i892.1, %_149.1.i.i904.pre, !dbg !37930
  br i1 %_53.i.i894.1, label %bb21.i.i896.1, label %panic1.i.i895, !dbg !37930

bb21.i.i896.1:                                    ; preds = %bb17.i.i885.1
  %280 = getelementptr inbounds nuw float, ptr %_147.0.i.i897, i64 %_50.i.i892.1, !dbg !37930
  %_49.i.i898.1 = load float, ptr %280, align 4, !dbg !37930, !noalias !37860, !noundef !12
  store float %_49.i.i898.1, ptr %iter.i.i694.sroa.0.0.ptr6712.1, align 4, !dbg !37931, !noalias !37860
  %281 = icmp eq i64 %width.i.i855, 2, !dbg !37869
  br i1 %281, label %bb16.i.i899, label %bb39.i.i879.2, !dbg !37869

bb39.i.i879.2:                                    ; preds = %bb21.i.i896.1
  %exitcond11095.2.not = icmp eq i64 %_145.1.i.i882, 2, !dbg !37872
  br i1 %exitcond11095.2.not, label %panic.i.i884, label %bb17.i.i885.2, !dbg !37872

bb17.i.i885.2:                                    ; preds = %bb39.i.i879.2
  %282 = getelementptr inbounds nuw i8, ptr %_145.0.i.i886, i64 32, !dbg !37872
  %_44.i.i887.2 = load i32, ptr %282, align 4, !dbg !37872, !noalias !37860, !noundef !12
  %_43.i.i888.2 = zext i32 %_44.i.i887.2 to i64, !dbg !37872
  %283 = add i64 %ring_cursor.sroa.0.1.i7356715, %_43.i.i888.2, !dbg !37927
  %_47.not.i.i889.2 = icmp ult i64 %283, %_86.i761, !dbg !37928
  %284 = select i1 %_47.not.i.i889.2, i64 0, i64 %_86.i761, !dbg !37928
  %spec.select.i.i890.2 = sub nuw i64 %283, %284, !dbg !37928
  %_51.i.i891.2 = mul i64 %spec.select.i.i890.2, %width.i.i855, !dbg !37929
  %_50.i.i892.2 = add i64 %_51.i.i891.2, 2, !dbg !37929
  %_53.i.i894.2 = icmp ult i64 %_50.i.i892.2, %_149.1.i.i904.pre, !dbg !37930
  br i1 %_53.i.i894.2, label %bb21.i.i896.2, label %panic1.i.i895, !dbg !37930

bb21.i.i896.2:                                    ; preds = %bb17.i.i885.2
  %285 = getelementptr inbounds nuw float, ptr %_147.0.i.i897, i64 %_50.i.i892.2, !dbg !37930
  %_49.i.i898.2 = load float, ptr %285, align 4, !dbg !37930, !noalias !37860, !noundef !12
  store float %_49.i.i898.2, ptr %iter.i.i694.sroa.0.0.ptr6712.2, align 4, !dbg !37931, !noalias !37860
  %286 = icmp eq i64 %width.i.i855, 3, !dbg !37869
  br i1 %286, label %bb16.i.i899, label %bb39.i.i879.3, !dbg !37869

bb39.i.i879.3:                                    ; preds = %bb21.i.i896.2
  %exitcond11095.3.not = icmp eq i64 %_145.1.i.i882, 3, !dbg !37872
  br i1 %exitcond11095.3.not, label %panic.i.i884, label %bb17.i.i885.3, !dbg !37872

bb17.i.i885.3:                                    ; preds = %bb39.i.i879.3
  %287 = getelementptr inbounds nuw i8, ptr %_145.0.i.i886, i64 44, !dbg !37872
  %_44.i.i887.3 = load i32, ptr %287, align 4, !dbg !37872, !noalias !37860, !noundef !12
  %_43.i.i888.3 = zext i32 %_44.i.i887.3 to i64, !dbg !37872
  %288 = add i64 %ring_cursor.sroa.0.1.i7356715, %_43.i.i888.3, !dbg !37927
  %_47.not.i.i889.3 = icmp ult i64 %288, %_86.i761, !dbg !37928
  %289 = select i1 %_47.not.i.i889.3, i64 0, i64 %_86.i761, !dbg !37928
  %spec.select.i.i890.3 = sub nuw i64 %288, %289, !dbg !37928
  %_51.i.i891.3 = mul i64 %spec.select.i.i890.3, %width.i.i855, !dbg !37929
  %_50.i.i892.3 = add i64 %_51.i.i891.3, 3, !dbg !37929
  %_53.i.i894.3 = icmp ult i64 %_50.i.i892.3, %_149.1.i.i904.pre, !dbg !37930
  br i1 %_53.i.i894.3, label %bb21.i.i896.3, label %panic1.i.i895, !dbg !37930

bb21.i.i896.3:                                    ; preds = %bb17.i.i885.3
  %290 = getelementptr inbounds nuw float, ptr %_147.0.i.i897, i64 %_50.i.i892.3, !dbg !37930
  %_49.i.i898.3 = load float, ptr %290, align 4, !dbg !37930, !noalias !37860, !noundef !12
  store float %_49.i.i898.3, ptr %iter.i.i694.sroa.0.0.ptr6712.3, align 4, !dbg !37931, !noalias !37860
  %291 = icmp eq i64 %width.i.i855, 4, !dbg !37869
  br i1 %291, label %bb16.i.i899, label %bb39.i.i879.4, !dbg !37869

bb39.i.i879.4:                                    ; preds = %bb21.i.i896.3
  %exitcond11095.4.not = icmp eq i64 %_145.1.i.i882, 4, !dbg !37872
  br i1 %exitcond11095.4.not, label %panic.i.i884, label %bb17.i.i885.4, !dbg !37872

bb17.i.i885.4:                                    ; preds = %bb39.i.i879.4
  %292 = getelementptr inbounds nuw i8, ptr %_145.0.i.i886, i64 56, !dbg !37872
  %_44.i.i887.4 = load i32, ptr %292, align 4, !dbg !37872, !noalias !37860, !noundef !12
  %_43.i.i888.4 = zext i32 %_44.i.i887.4 to i64, !dbg !37872
  %293 = add i64 %ring_cursor.sroa.0.1.i7356715, %_43.i.i888.4, !dbg !37927
  %_47.not.i.i889.4 = icmp ult i64 %293, %_86.i761, !dbg !37928
  %294 = select i1 %_47.not.i.i889.4, i64 0, i64 %_86.i761, !dbg !37928
  %spec.select.i.i890.4 = sub nuw i64 %293, %294, !dbg !37928
  %_51.i.i891.4 = mul i64 %spec.select.i.i890.4, %width.i.i855, !dbg !37929
  %_50.i.i892.4 = add i64 %_51.i.i891.4, 4, !dbg !37929
  %_53.i.i894.4 = icmp ult i64 %_50.i.i892.4, %_149.1.i.i904.pre, !dbg !37930
  br i1 %_53.i.i894.4, label %bb21.i.i896.4, label %panic1.i.i895, !dbg !37930

bb21.i.i896.4:                                    ; preds = %bb17.i.i885.4
  %295 = getelementptr inbounds nuw float, ptr %_147.0.i.i897, i64 %_50.i.i892.4, !dbg !37930
  %_49.i.i898.4 = load float, ptr %295, align 4, !dbg !37930, !noalias !37860, !noundef !12
  store float %_49.i.i898.4, ptr %iter.i.i694.sroa.0.0.ptr6712.4, align 4, !dbg !37931, !noalias !37860
  %296 = icmp eq i64 %width.i.i855, 5, !dbg !37869
  br i1 %296, label %bb16.i.i899, label %bb39.i.i879.5, !dbg !37869

bb39.i.i879.5:                                    ; preds = %bb21.i.i896.4
  %exitcond11095.5.not = icmp eq i64 %_145.1.i.i882, 5, !dbg !37872
  br i1 %exitcond11095.5.not, label %panic.i.i884, label %bb17.i.i885.5, !dbg !37872

bb17.i.i885.5:                                    ; preds = %bb39.i.i879.5
  %297 = getelementptr inbounds nuw i8, ptr %_145.0.i.i886, i64 68, !dbg !37872
  %_44.i.i887.5 = load i32, ptr %297, align 4, !dbg !37872, !noalias !37860, !noundef !12
  %_43.i.i888.5 = zext i32 %_44.i.i887.5 to i64, !dbg !37872
  %298 = add i64 %ring_cursor.sroa.0.1.i7356715, %_43.i.i888.5, !dbg !37927
  %_47.not.i.i889.5 = icmp ult i64 %298, %_86.i761, !dbg !37928
  %299 = select i1 %_47.not.i.i889.5, i64 0, i64 %_86.i761, !dbg !37928
  %spec.select.i.i890.5 = sub nuw i64 %298, %299, !dbg !37928
  %_51.i.i891.5 = mul i64 %spec.select.i.i890.5, %width.i.i855, !dbg !37929
  %_50.i.i892.5 = add i64 %_51.i.i891.5, 5, !dbg !37929
  %_53.i.i894.5 = icmp ult i64 %_50.i.i892.5, %_149.1.i.i904.pre, !dbg !37930
  br i1 %_53.i.i894.5, label %bb21.i.i896.5, label %panic1.i.i895, !dbg !37930

bb21.i.i896.5:                                    ; preds = %bb17.i.i885.5
  %300 = getelementptr inbounds nuw float, ptr %_147.0.i.i897, i64 %_50.i.i892.5, !dbg !37930
  %_49.i.i898.5 = load float, ptr %300, align 4, !dbg !37930, !noalias !37860, !noundef !12
  store float %_49.i.i898.5, ptr %iter.i.i694.sroa.0.0.ptr6712.5, align 4, !dbg !37931, !noalias !37860
  %301 = icmp eq i64 %width.i.i855, 6, !dbg !37869
  br i1 %301, label %bb16.i.i899, label %bb39.i.i879.6, !dbg !37869

bb39.i.i879.6:                                    ; preds = %bb21.i.i896.5
  %exitcond11095.6.not = icmp eq i64 %_145.1.i.i882, 6, !dbg !37872
  br i1 %exitcond11095.6.not, label %panic.i.i884, label %bb17.i.i885.6, !dbg !37872

bb17.i.i885.6:                                    ; preds = %bb39.i.i879.6
  %302 = getelementptr inbounds nuw i8, ptr %_145.0.i.i886, i64 80, !dbg !37872
  %_44.i.i887.6 = load i32, ptr %302, align 4, !dbg !37872, !noalias !37860, !noundef !12
  %_43.i.i888.6 = zext i32 %_44.i.i887.6 to i64, !dbg !37872
  %303 = add i64 %ring_cursor.sroa.0.1.i7356715, %_43.i.i888.6, !dbg !37927
  %_47.not.i.i889.6 = icmp ult i64 %303, %_86.i761, !dbg !37928
  %304 = select i1 %_47.not.i.i889.6, i64 0, i64 %_86.i761, !dbg !37928
  %spec.select.i.i890.6 = sub nuw i64 %303, %304, !dbg !37928
  %_51.i.i891.6 = mul i64 %spec.select.i.i890.6, %width.i.i855, !dbg !37929
  %_50.i.i892.6 = add i64 %_51.i.i891.6, 6, !dbg !37929
  %_53.i.i894.6 = icmp ult i64 %_50.i.i892.6, %_149.1.i.i904.pre, !dbg !37930
  br i1 %_53.i.i894.6, label %bb21.i.i896.6, label %panic1.i.i895, !dbg !37930

bb21.i.i896.6:                                    ; preds = %bb17.i.i885.6
  %305 = getelementptr inbounds nuw float, ptr %_147.0.i.i897, i64 %_50.i.i892.6, !dbg !37930
  %_49.i.i898.6 = load float, ptr %305, align 4, !dbg !37930, !noalias !37860, !noundef !12
  store float %_49.i.i898.6, ptr %iter.i.i694.sroa.0.0.ptr6712.6, align 4, !dbg !37931, !noalias !37860
  %306 = icmp eq i64 %width.i.i855, 7, !dbg !37869
  br i1 %306, label %bb16.i.i899, label %bb39.i.i879.7, !dbg !37869

bb39.i.i879.7:                                    ; preds = %bb21.i.i896.6
  %exitcond11095.7.not = icmp eq i64 %_145.1.i.i882, 7, !dbg !37872
  br i1 %exitcond11095.7.not, label %panic.i.i884, label %bb17.i.i885.7, !dbg !37872

bb17.i.i885.7:                                    ; preds = %bb39.i.i879.7
  %307 = getelementptr inbounds nuw i8, ptr %_145.0.i.i886, i64 92, !dbg !37872
  %_44.i.i887.7 = load i32, ptr %307, align 4, !dbg !37872, !noalias !37860, !noundef !12
  %_43.i.i888.7 = zext i32 %_44.i.i887.7 to i64, !dbg !37872
  %308 = add i64 %ring_cursor.sroa.0.1.i7356715, %_43.i.i888.7, !dbg !37927
  %_47.not.i.i889.7 = icmp ult i64 %308, %_86.i761, !dbg !37928
  %309 = select i1 %_47.not.i.i889.7, i64 0, i64 %_86.i761, !dbg !37928
  %spec.select.i.i890.7 = sub nuw i64 %308, %309, !dbg !37928
  %_51.i.i891.7 = mul i64 %spec.select.i.i890.7, %width.i.i855, !dbg !37929
  %_50.i.i892.7 = add i64 %_51.i.i891.7, 7, !dbg !37929
  %_53.i.i894.7 = icmp ult i64 %_50.i.i892.7, %_149.1.i.i904.pre, !dbg !37930
  br i1 %_53.i.i894.7, label %bb21.i.i896.7, label %panic1.i.i895, !dbg !37930

bb21.i.i896.7:                                    ; preds = %bb17.i.i885.7
  %310 = getelementptr inbounds nuw float, ptr %_147.0.i.i897, i64 %_50.i.i892.7, !dbg !37930
  %_49.i.i898.7 = load float, ptr %310, align 4, !dbg !37930, !noalias !37860, !noundef !12
  store float %_49.i.i898.7, ptr %iter.i.i694.sroa.0.0.ptr6712.7, align 4, !dbg !37931, !noalias !37860
  br label %bb16.i.i899, !dbg !37869

panic1.i.i895:                                    ; preds = %bb17.i.i885.7, %bb17.i.i885.6, %bb17.i.i885.5, %bb17.i.i885.4, %bb17.i.i885.3, %bb17.i.i885.2, %bb17.i.i885.1, %bb17.i.i885
  %_50.i.i892.lcssa.ph = phi i64 [ %_50.i.i892.7, %bb17.i.i885.7 ], [ %_50.i.i892.6, %bb17.i.i885.6 ], [ %_50.i.i892.5, %bb17.i.i885.5 ], [ %_50.i.i892.4, %bb17.i.i885.4 ], [ %_50.i.i892.3, %bb17.i.i885.3 ], [ %_50.i.i892.2, %bb17.i.i885.2 ], [ %_50.i.i892.1, %bb17.i.i885.1 ], [ %_51.i.i891, %bb17.i.i885 ]
  store float %_0.i2875, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i.i892.lcssa.ph, i64 noundef %_149.1.i.i904.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b305c1483509cfb31fdec21ff8752674) #30, !dbg !37930, !noalias !37860
  unreachable, !dbg !37930

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3114: ; preds = %bb49.i.i926
  %_127.i.i929 = getelementptr inbounds nuw float, ptr %_150.0.i.i927, i64 %_76.i.i924, !dbg !37932
  %_0.i2957 = load float, ptr %_127.i.i929, align 4, !dbg !37923, !alias.scope !37919, !noalias !37860, !noundef !12
  store float %_0.i2966, ptr %_127.i.i929, align 4, !dbg !37934, !alias.scope !37936, !noalias !37860
  %_0.i2548 = fmul float %_0.i2868, %_0.i2957, !dbg !37939
  %_6.i3376 = bitcast float %_0.i2957 to i32, !dbg !37941
  %_5.i3377 = and i32 %_6.i3376, %all.sroa.0.0.i709, !dbg !37944
  %_8.i3378 = bitcast float %_0.i2548 to i32, !dbg !37945
  %_7.i3379 = and i32 %_9.i3391, %_8.i3378, !dbg !37947
  %_4.i3380 = or disjoint i32 %_7.i3379, %_5.i3377, !dbg !37944
  store i32 %_4.i3380, ptr %_149.i853, align 4, !dbg !37948, !alias.scope !37950, !noalias !37953
  %311 = add i64 %main_cursor.sroa.0.1.i7366716, 1, !dbg !37954
  %_99.i944 = icmp eq i64 %311, %_101.i943, !dbg !37955
  %spec.store.select.i945 = select i1 %_99.i944, i64 0, i64 %311, !dbg !37955
  %312 = add i64 %ring_cursor.sroa.0.1.i7356715, 1, !dbg !37956
  %_102.i946 = icmp eq i64 %312, %_86.i761, !dbg !37957
  %spec.store.select13.i947 = select i1 %_102.i946, i64 0, i64 %312, !dbg !37957
  %exitcond11098.not = icmp eq i64 %182, %umax11097, !dbg !37958
  br i1 %exitcond11098.not, label %bb16.i733.bb13.i719.loopexit_crit_edge, label %bb43.i747, !dbg !36828

bb46.i953:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3130
  store float %_0.i2875, ptr %145, align 1, !dbg !36764
  store float %_0.i28717555, ptr %161, align 1, !dbg !36785
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_59.i739, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_af093af980f3691b22b0bed94901ed34) #30, !dbg !37961, !noalias !36807
  unreachable, !dbg !37961

_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit.loopexit: ; preds = %bb13.i719.loopexit
  %313 = trunc i64 %main_cursor.sroa.0.1.i736.lcssa to i32, !dbg !37962
  %314 = trunc i64 %ring_cursor.sroa.0.1.i735.lcssa to i32, !dbg !37963
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit, !dbg !37964

_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit.loopexit, %bb12.i
  %ring_cursor.sroa.0.0.i722.lcssa = phi i32 [ %_36.i711, %bb12.i ], [ %314, %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit.loopexit ], !dbg !36737
  %main_cursor.sroa.0.0.i723.lcssa = phi i32 [ %_34.i710, %bb12.i ], [ %313, %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit.loopexit ], !dbg !36734
  call void @llvm.lifetime.start.p0(ptr nonnull %_105.i696), !dbg !37964, !noalias !36717
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(92) %_105.i696, ptr noundef nonnull align 4 dereferenceable(92) %hot_left.i702, i64 92, i1 false), !dbg !37964, !noalias !36717
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_105.i696, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33) #31, !dbg !37965, !noalias !36807
  call void @llvm.lifetime.end.p0(ptr nonnull %_105.i696), !dbg !37966, !noalias !36717
  call void @llvm.lifetime.start.p0(ptr nonnull %_107.i695), !dbg !37967, !noalias !36717
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(92) %_107.i695, ptr noundef nonnull align 4 dereferenceable(92) %hot_right.i701, i64 92, i1 false), !dbg !37967, !noalias !36717
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_107.i695, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34) #31, !dbg !37968, !noalias !36807
  call void @llvm.lifetime.end.p0(ptr nonnull %_107.i695), !dbg !37969, !noalias !36717
  store i32 %main_cursor.sroa.0.0.i723.lcssa, ptr %_35, align 4, !dbg !37962, !alias.scope !36711, !noalias !36736
  store i32 %ring_cursor.sroa.0.0.i722.lcssa, ptr %83, align 4, !dbg !37963, !alias.scope !36711, !noalias !36736
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i698), !dbg !37970, !noalias !36717
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i699), !dbg !37971, !noalias !36717
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i700), !dbg !37972, !noalias !36717
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_right.i701), !dbg !37973, !noalias !36717
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i702), !dbg !37974, !noalias !36717
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockfEB2_.exit, !dbg !36706

bb11.i:                                           ; preds = %bb10.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37975), !dbg !37978
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37979), !dbg !37978
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37981), !dbg !37978
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37983), !dbg !37978
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_left.i432, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #31, !dbg !37985
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_right.i431, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #31, !dbg !37989
  %315 = getelementptr inbounds nuw i8, ptr %self, i64 776, !dbg !37991
  %316 = load i8, ptr %315, align 4, !dbg !37991, !range !17, !alias.scope !37975, !noalias !37995, !noundef !12
  %317 = getelementptr inbounds nuw i8, ptr %self, i64 777, !dbg !37999
  %318 = load i8, ptr %317, align 1, !dbg !37999, !range !17, !alias.scope !37975, !noalias !37995, !noundef !12
  %_34.i = load i32, ptr %_35, align 4, !dbg !38001, !alias.scope !37983, !noalias !38003, !noundef !12
  %319 = getelementptr inbounds nuw i8, ptr %self, i64 564, !dbg !38004
  %_36.i440 = load i32, ptr %319, align 4, !dbg !38004, !alias.scope !37983, !noalias !38003, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i), !dbg !38006, !noalias !38008
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i, i8 0, i64 32, i1 false), !noalias !38008
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i430), !dbg !38009, !noalias !38008
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i430, i8 0, i64 1024, i1 false), !noalias !38008
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i429), !dbg !38011, !noalias !38008
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i429, i8 0, i64 1024, i1 false), !noalias !38008
  %_31.i436 = zext nneg i8 %316 to i32, !dbg !37991
  %.none.i437 = sub nsw i32 0, %_31.i436, !dbg !38013
  %_32.i438 = zext nneg i8 %318 to i32, !dbg !37999
  %all.sroa.0.0.i439 = sub nsw i32 0, %_32.i438, !dbg !37999
  %_111.not.i7789 = icmp eq i64 %frames, 0, !dbg !38014
  br i1 %_111.not.i7789, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit, label %bb37.i.lr.ph, !dbg !38014

bb37.i.lr.ph:                                     ; preds = %bb11.i
  %320 = zext i32 %_36.i440 to i64, !dbg !38004
  %321 = zext i32 %_34.i to i64, !dbg !38001
  %d9.i.i4091 = lshr i64 %frames, 5, !dbg !38024
  %r2.i.i4092 = and i64 %frames, 31, !dbg !38030
  %_19.not.i.i4093 = icmp ne i64 %r2.i.i4092, 0, !dbg !38031
  %322 = zext i1 %_19.not.i.i4093 to i64, !dbg !38031
  %yield_count.sroa.0.0.i.i4094 = add nuw nsw i64 %d9.i.i4091, %322, !dbg !38031
  %history.i135.i.sroa.7.0.hot_left.i432.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i432, i64 4
  %history.i135.i.sroa.10.0.hot_left.i432.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i432, i64 8
  %history.i135.i.sroa.13.0.hot_left.i432.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i432, i64 12
  %history.i135.i.sroa.16.0.hot_left.i432.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i432, i64 16
  %history.i135.i.sroa.19.0.hot_left.i432.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i432, i64 20
  %history.i135.i.sroa.22.0.hot_left.i432.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i432, i64 24
  %history.i135.i.sroa.26.0.hot_left.i432.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i432, i64 28
  %history.i135.i.sroa.29.0.hot_left.i432.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i432, i64 32
  %history.i135.i.sroa.32.0.hot_left.i432.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i432, i64 36
  %history.i135.i.sroa.35.0.hot_left.i432.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i432, i64 40
  %history.i135.i.sroa.38.0.hot_left.i432.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i432, i64 44
  %323 = getelementptr inbounds nuw i8, ptr %self, i64 588
  %324 = getelementptr inbounds nuw i8, ptr %self, i64 592
  %325 = getelementptr inbounds nuw i8, ptr %self, i64 596
  %row1.i.i.i171.i = getelementptr inbounds nuw i8, ptr %self, i64 600
  %326 = getelementptr inbounds nuw i8, ptr %self, i64 604
  %327 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %328 = getelementptr inbounds nuw i8, ptr %self, i64 612
  %row3.i.i.i185.i = getelementptr inbounds nuw i8, ptr %self, i64 616
  %329 = getelementptr inbounds nuw i8, ptr %self, i64 620
  %330 = getelementptr inbounds nuw i8, ptr %self, i64 624
  %331 = getelementptr inbounds nuw i8, ptr %self, i64 628
  %row5.i.i.i199.i = getelementptr inbounds nuw i8, ptr %self, i64 632
  %332 = getelementptr inbounds nuw i8, ptr %self, i64 636
  %333 = getelementptr inbounds nuw i8, ptr %self, i64 640
  %334 = getelementptr inbounds nuw i8, ptr %self, i64 644
  %row7.i.i.i213.i = getelementptr inbounds nuw i8, ptr %self, i64 648
  %335 = getelementptr inbounds nuw i8, ptr %self, i64 652
  %336 = getelementptr inbounds nuw i8, ptr %self, i64 656
  %337 = getelementptr inbounds nuw i8, ptr %self, i64 660
  %row9.i.i.i227.i = getelementptr inbounds nuw i8, ptr %self, i64 664
  %338 = getelementptr inbounds nuw i8, ptr %self, i64 668
  %339 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %340 = getelementptr inbounds nuw i8, ptr %self, i64 676
  %row11.i.i.i241.i = getelementptr inbounds nuw i8, ptr %self, i64 680
  %341 = getelementptr inbounds nuw i8, ptr %self, i64 684
  %342 = getelementptr inbounds nuw i8, ptr %self, i64 688
  %343 = getelementptr inbounds nuw i8, ptr %self, i64 692
  %row13.i.i.i255.i = getelementptr inbounds nuw i8, ptr %self, i64 696
  %344 = getelementptr inbounds nuw i8, ptr %self, i64 700
  %345 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %346 = getelementptr inbounds nuw i8, ptr %self, i64 708
  %row15.i.i.i269.i = getelementptr inbounds nuw i8, ptr %self, i64 712
  %347 = getelementptr inbounds nuw i8, ptr %self, i64 716
  %348 = getelementptr inbounds nuw i8, ptr %self, i64 720
  %349 = getelementptr inbounds nuw i8, ptr %self, i64 724
  %row17.i.i.i283.i = getelementptr inbounds nuw i8, ptr %self, i64 728
  %350 = getelementptr inbounds nuw i8, ptr %self, i64 732
  %351 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %352 = getelementptr inbounds nuw i8, ptr %self, i64 740
  %row19.i.i.i297.i = getelementptr inbounds nuw i8, ptr %self, i64 744
  %353 = getelementptr inbounds nuw i8, ptr %self, i64 748
  %354 = getelementptr inbounds nuw i8, ptr %self, i64 752
  %355 = getelementptr inbounds nuw i8, ptr %self, i64 756
  %row21.i.i.i311.i = getelementptr inbounds nuw i8, ptr %self, i64 760
  %356 = getelementptr inbounds nuw i8, ptr %self, i64 764
  %357 = getelementptr inbounds nuw i8, ptr %self, i64 768
  %358 = getelementptr inbounds nuw i8, ptr %self, i64 772
  %history.i.i428.sroa.7.0.hot_right.i431.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i431, i64 4
  %history.i.i428.sroa.10.0.hot_right.i431.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i431, i64 8
  %history.i.i428.sroa.13.0.hot_right.i431.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i431, i64 12
  %history.i.i428.sroa.16.0.hot_right.i431.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i431, i64 16
  %history.i.i428.sroa.19.0.hot_right.i431.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i431, i64 20
  %history.i.i428.sroa.22.0.hot_right.i431.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i431, i64 24
  %history.i.i428.sroa.26.0.hot_right.i431.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i431, i64 28
  %history.i.i428.sroa.29.0.hot_right.i431.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i431, i64 32
  %history.i.i428.sroa.32.0.hot_right.i431.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i431, i64 36
  %history.i.i428.sroa.35.0.hot_right.i431.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i431, i64 40
  %history.i.i428.sroa.38.0.hot_right.i431.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i431, i64 44
  %_63.i457 = getelementptr inbounds nuw i8, ptr %hot_left.i432, i64 48
  %_64.i = getelementptr inbounds nuw i8, ptr %hot_left.i432, i64 64
  %_68.i = getelementptr inbounds nuw i8, ptr %hot_right.i431, i64 48
  %_69.i458 = getelementptr inbounds nuw i8, ptr %hot_right.i431, i64 64
  %_9.i3451 = add nsw i32 %_31.i436, -1
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
  %369 = getelementptr inbounds nuw i8, ptr %hot_left.i432, i64 84
  %370 = getelementptr inbounds nuw i8, ptr %hot_left.i432, i64 88
  %371 = getelementptr inbounds nuw i8, ptr %hot_left.i432, i64 80
  %372 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %373 = getelementptr inbounds nuw i8, ptr %self, i64 152
  %_9.i3431 = add nsw i32 %_32.i438, -1
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
  %385 = getelementptr inbounds nuw i8, ptr %hot_right.i431, i64 84
  %386 = getelementptr inbounds nuw i8, ptr %hot_right.i431, i64 88
  %387 = getelementptr inbounds nuw i8, ptr %hot_right.i431, i64 80
  %388 = getelementptr inbounds nuw i8, ptr %self, i64 360
  %389 = getelementptr inbounds nuw i8, ptr %self, i64 352
  %390 = getelementptr inbounds nuw i8, ptr %self, i64 552
  %iter.i32.i.sroa.0.0.ptr7629.1 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 4
  %iter.i32.i.sroa.0.0.ptr7629.2 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 8
  %iter.i32.i.sroa.0.0.ptr7629.3 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 12
  %iter.i32.i.sroa.0.0.ptr7629.4 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 16
  %iter.i32.i.sroa.0.0.ptr7629.5 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 20
  %iter.i32.i.sroa.0.0.ptr7629.6 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 24
  %iter.i32.i.sroa.0.0.ptr7629.7 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 28
  %iter.i.i.sroa.0.0.ptr7640.1 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 4
  %iter.i.i.sroa.0.0.ptr7640.2 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 8
  %iter.i.i.sroa.0.0.ptr7640.3 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 12
  %iter.i.i.sroa.0.0.ptr7640.4 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 16
  %iter.i.i.sroa.0.0.ptr7640.5 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 20
  %iter.i.i.sroa.0.0.ptr7640.6 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 24
  %iter.i.i.sroa.0.0.ptr7640.7 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 28
  br label %bb37.i, !dbg !38014

bb16.i.bb13.i.loopexit_crit_edge:                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3154
  store float %_0.i3257, ptr %371, align 1, !dbg !38032
  store float %_0.i3253, ptr %387, align 1, !dbg !38046
  store float %_0.i2883, ptr %369, align 4, !dbg !38048
  store float %_0.i2879, ptr %385, align 4, !dbg !38049
  br label %bb13.i.loopexit, !dbg !38050

bb13.i.loopexit:                                  ; preds = %bb16.i.bb13.i.loopexit_crit_edge, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i453
  %ring_cursor.sroa.0.1.i455.lcssa = phi i64 [ %spec.store.select13.i, %bb16.i.bb13.i.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i4467792, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i453 ], !dbg !38056
  %main_cursor.sroa.0.1.i456.lcssa = phi i64 [ %spec.store.select.i, %bb16.i.bb13.i.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i4477793, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i453 ], !dbg !38057
  %_111.not.i = icmp eq i64 %393, 0, !dbg !38014
  %indvars.iv.next11100 = add i64 %indvars.iv11099, -32, !dbg !38014
  br i1 %_111.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit.loopexit, label %bb37.i, !dbg !38014

bb37.i:                                           ; preds = %bb37.i.lr.ph, %bb13.i.loopexit
  %indvars.iv11099 = phi i64 [ %frames, %bb37.i.lr.ph ], [ %indvars.iv.next11100, %bb13.i.loopexit ]
  %main_cursor.sroa.0.0.i4477793 = phi i64 [ %321, %bb37.i.lr.ph ], [ %main_cursor.sroa.0.1.i456.lcssa, %bb13.i.loopexit ]
  %ring_cursor.sroa.0.0.i4467792 = phi i64 [ %320, %bb37.i.lr.ph ], [ %ring_cursor.sroa.0.1.i455.lcssa, %bb13.i.loopexit ]
  %iter2.sroa.0.0.i4457791 = phi i64 [ %yield_count.sroa.0.0.i.i4094, %bb37.i.lr.ph ], [ %393, %bb13.i.loopexit ]
  %iter.sroa.0.0.i7790 = phi i64 [ 0, %bb37.i.lr.ph ], [ %392, %bb13.i.loopexit ]
  %391 = call i64 @llvm.umax.i64(i64 %indvars.iv11099, i64 1), !dbg !38058
  %umax11120 = call i64 @llvm.umin.i64(i64 %391, i64 32), !dbg !38058
  %392 = add i64 %iter.sroa.0.0.i7790, 32, !dbg !38058
  %393 = add i64 %iter2.sroa.0.0.i4457791, -1, !dbg !38062
  %history.i135.i.sroa.0.0.copyload = load float, ptr %hot_left.i432, align 4, !dbg !38063
  %history.i135.i.sroa.7.0.copyload = load float, ptr %history.i135.i.sroa.7.0.hot_left.i432.sroa_idx, align 4, !dbg !38063
  %history.i135.i.sroa.10.0.copyload = load float, ptr %history.i135.i.sroa.10.0.hot_left.i432.sroa_idx, align 4, !dbg !38063
  %history.i135.i.sroa.13.0.copyload = load float, ptr %history.i135.i.sroa.13.0.hot_left.i432.sroa_idx, align 4, !dbg !38063
  %history.i135.i.sroa.16.0.copyload = load float, ptr %history.i135.i.sroa.16.0.hot_left.i432.sroa_idx, align 4, !dbg !38063
  %history.i135.i.sroa.19.0.copyload = load float, ptr %history.i135.i.sroa.19.0.hot_left.i432.sroa_idx, align 4, !dbg !38063
  %history.i135.i.sroa.22.0.copyload = load float, ptr %history.i135.i.sroa.22.0.hot_left.i432.sroa_idx, align 4, !dbg !38063
  %history.i135.i.sroa.26.0.copyload = load float, ptr %history.i135.i.sroa.26.0.hot_left.i432.sroa_idx, align 4, !dbg !38063
  %history.i135.i.sroa.29.0.copyload = load float, ptr %history.i135.i.sroa.29.0.hot_left.i432.sroa_idx, align 4, !dbg !38063
  %history.i135.i.sroa.32.0.copyload = load float, ptr %history.i135.i.sroa.32.0.hot_left.i432.sroa_idx, align 4, !dbg !38063
  %history.i135.i.sroa.35.0.copyload = load float, ptr %history.i135.i.sroa.35.0.hot_left.i432.sroa_idx, align 4, !dbg !38063
  %history.i135.i.sroa.38.0.copyload = load float, ptr %history.i135.i.sroa.38.0.hot_left.i432.sroa_idx, align 4, !dbg !38063
  %_20.i138.i7567.not = icmp eq i64 %frames, %iter.sroa.0.0.i7790, !dbg !38065
  br i1 %_20.i138.i7567.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit339.i, label %bb5.i139.i.lr.ph, !dbg !38069

bb5.i139.i.lr.ph:                                 ; preds = %bb37.i
  %_11.i.i.i159.i = load float, ptr %_31, align 4
  %_14.i.i.i162.i = load float, ptr %323, align 4
  %_17.i.i.i165.i = load float, ptr %324, align 4
  %_20.i.i.i168.i = load float, ptr %325, align 4
  %_25.i.i.i173.i = load float, ptr %row1.i.i.i171.i, align 4
  %_28.i.i.i176.i = load float, ptr %326, align 4
  %_31.i.i.i179.i = load float, ptr %327, align 4
  %_34.i.i.i182.i = load float, ptr %328, align 4
  %_39.i.i.i187.i = load float, ptr %row3.i.i.i185.i, align 4
  %_42.i.i.i190.i = load float, ptr %329, align 4
  %_45.i.i.i193.i = load float, ptr %330, align 4
  %_48.i.i.i196.i = load float, ptr %331, align 4
  %_53.i.i.i201.i = load float, ptr %row5.i.i.i199.i, align 4
  %_56.i.i.i204.i = load float, ptr %332, align 4
  %_59.i.i.i207.i = load float, ptr %333, align 4
  %_62.i.i.i210.i = load float, ptr %334, align 4
  %_67.i.i.i215.i = load float, ptr %row7.i.i.i213.i, align 4
  %_70.i.i.i218.i = load float, ptr %335, align 4
  %_73.i.i.i221.i = load float, ptr %336, align 4
  %_76.i.i.i224.i = load float, ptr %337, align 4
  %_81.i.i.i229.i = load float, ptr %row9.i.i.i227.i, align 4
  %_84.i.i.i232.i = load float, ptr %338, align 4
  %_87.i.i.i235.i = load float, ptr %339, align 4
  %_90.i.i.i238.i = load float, ptr %340, align 4
  %_95.i.i.i243.i = load float, ptr %row11.i.i.i241.i, align 4
  %_98.i.i.i246.i = load float, ptr %341, align 4
  %_101.i.i.i249.i = load float, ptr %342, align 4
  %_104.i.i.i252.i = load float, ptr %343, align 4
  %_109.i.i.i257.i = load float, ptr %row13.i.i.i255.i, align 4
  %_112.i.i.i260.i = load float, ptr %344, align 4
  %_115.i.i.i263.i = load float, ptr %345, align 4
  %_118.i.i.i266.i = load float, ptr %346, align 4
  %_123.i.i.i271.i = load float, ptr %row15.i.i.i269.i, align 4
  %_126.i.i.i274.i = load float, ptr %347, align 4
  %_129.i.i.i277.i = load float, ptr %348, align 4
  %_132.i.i.i280.i = load float, ptr %349, align 4
  %_137.i.i.i285.i = load float, ptr %row17.i.i.i283.i, align 4
  %_140.i.i.i288.i = load float, ptr %350, align 4
  %_143.i.i.i291.i = load float, ptr %351, align 4
  %_146.i.i.i294.i = load float, ptr %352, align 4
  %_151.i.i.i299.i = load float, ptr %row19.i.i.i297.i, align 4
  %_154.i.i.i302.i = load float, ptr %353, align 4
  %_157.i.i.i305.i = load float, ptr %354, align 4
  %_160.i.i.i308.i = load float, ptr %355, align 4
  %_165.i.i.i313.i = load float, ptr %row21.i.i.i311.i, align 4
  %_168.i.i.i316.i = load float, ptr %356, align 4
  %_171.i.i.i319.i = load float, ptr %357, align 4
  %_174.i.i.i322.i = load float, ptr %358, align 4
  br label %bb5.i139.i, !dbg !38069

bb5.i139.i:                                       ; preds = %bb5.i139.i.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997
  %iter.sroa.0.0.i137.i7579 = phi i64 [ 0, %bb5.i139.i.lr.ph ], [ %394, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997 ]
  %history.i135.i.sroa.35.07578 = phi float [ %history.i135.i.sroa.35.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.32.07577, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997 ]
  %history.i135.i.sroa.32.07577 = phi float [ %history.i135.i.sroa.32.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.29.07576, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997 ]
  %history.i135.i.sroa.29.07576 = phi float [ %history.i135.i.sroa.29.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.26.07575, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997 ]
  %history.i135.i.sroa.26.07575 = phi float [ %history.i135.i.sroa.26.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.22.07574, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997 ]
  %history.i135.i.sroa.22.07574 = phi float [ %history.i135.i.sroa.22.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.19.07573, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997 ]
  %history.i135.i.sroa.19.07573 = phi float [ %history.i135.i.sroa.19.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.16.07572, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997 ]
  %history.i135.i.sroa.16.07572 = phi float [ %history.i135.i.sroa.16.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.13.07571, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997 ]
  %history.i135.i.sroa.13.07571 = phi float [ %history.i135.i.sroa.13.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.10.07570, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997 ]
  %history.i135.i.sroa.10.07570 = phi float [ %history.i135.i.sroa.10.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.7.07569, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997 ]
  %history.i135.i.sroa.7.07569 = phi float [ %history.i135.i.sroa.7.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.0.07568, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997 ]
  %history.i135.i.sroa.0.07568 = phi float [ %history.i135.i.sroa.0.0.copyload, %bb5.i139.i.lr.ph ], [ %_0.i2995, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997 ]
  %394 = add nuw nsw i64 %iter.sroa.0.0.i137.i7579, 1, !dbg !38070
  %_11.i140.i = add nuw nsw i64 %iter.sroa.0.0.i137.i7579, %iter.sroa.0.0.i7790, !dbg !38073
  %_24.i141.i = icmp ugt i64 %_11.i140.i, %left_io.1, !dbg !38074
  br i1 %_24.i141.i, label %bb7.i338.i, label %bb8.i142.i, !dbg !38074, !prof !639

bb8.i142.i:                                       ; preds = %bb5.i139.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38077), !dbg !38080
  %_3.not.i2993 = icmp eq i64 %left_io.1, %_11.i140.i, !dbg !38081
  br i1 %_3.not.i2993, label %panic.i2996, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997, !dbg !38081

panic.i2996:                                      ; preds = %bb8.i142.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !38081, !noalias !38083
  unreachable, !dbg !38081

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997: ; preds = %bb8.i142.i
  %_31.i144.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %_11.i140.i, !dbg !38087
  %_0.i2995 = load float, ptr %_31.i144.i, align 4, !dbg !38081, !alias.scope !38077, !noalias !38089, !noundef !12
  %395 = tail call noundef float @llvm.fabs.f32(float %history.i135.i.sroa.19.07573), !dbg !38090
  %_0.i2601 = fmul float %_0.i2995, %_11.i.i.i159.i, !dbg !38093
  %_0.i2173 = fadd float %_0.i2601, 0.000000e+00, !dbg !38096
  %_0.i2600 = fmul float %_0.i2995, %_14.i.i.i162.i, !dbg !38098
  %_0.i2172 = fadd float %_0.i2600, 0.000000e+00, !dbg !38100
  %_0.i2599 = fmul float %_0.i2995, %_17.i.i.i165.i, !dbg !38102
  %_0.i2171 = fadd float %_0.i2599, 0.000000e+00, !dbg !38104
  %_0.i2598 = fmul float %_0.i2995, %_20.i.i.i168.i, !dbg !38106
  %_0.i2170 = fadd float %_0.i2598, 0.000000e+00, !dbg !38108
  %_0.i2597 = fmul float %history.i135.i.sroa.0.07568, %_25.i.i.i173.i, !dbg !38110
  %_0.i2169 = fadd float %_0.i2173, %_0.i2597, !dbg !38112
  %_0.i2596 = fmul float %history.i135.i.sroa.0.07568, %_28.i.i.i176.i, !dbg !38114
  %_0.i2168 = fadd float %_0.i2172, %_0.i2596, !dbg !38116
  %_0.i2595 = fmul float %history.i135.i.sroa.0.07568, %_31.i.i.i179.i, !dbg !38118
  %_0.i2167 = fadd float %_0.i2171, %_0.i2595, !dbg !38120
  %_0.i2594 = fmul float %history.i135.i.sroa.0.07568, %_34.i.i.i182.i, !dbg !38122
  %_0.i2166 = fadd float %_0.i2170, %_0.i2594, !dbg !38124
  %_0.i2593 = fmul float %history.i135.i.sroa.7.07569, %_39.i.i.i187.i, !dbg !38126
  %_0.i2165 = fadd float %_0.i2169, %_0.i2593, !dbg !38128
  %_0.i2592 = fmul float %history.i135.i.sroa.7.07569, %_42.i.i.i190.i, !dbg !38130
  %_0.i2164 = fadd float %_0.i2168, %_0.i2592, !dbg !38132
  %_0.i2591 = fmul float %history.i135.i.sroa.7.07569, %_45.i.i.i193.i, !dbg !38134
  %_0.i2163 = fadd float %_0.i2167, %_0.i2591, !dbg !38136
  %_0.i2590 = fmul float %history.i135.i.sroa.7.07569, %_48.i.i.i196.i, !dbg !38138
  %_0.i2162 = fadd float %_0.i2166, %_0.i2590, !dbg !38140
  %_0.i2589 = fmul float %history.i135.i.sroa.10.07570, %_53.i.i.i201.i, !dbg !38142
  %_0.i2161 = fadd float %_0.i2165, %_0.i2589, !dbg !38144
  %_0.i2588 = fmul float %history.i135.i.sroa.10.07570, %_56.i.i.i204.i, !dbg !38146
  %_0.i2160 = fadd float %_0.i2164, %_0.i2588, !dbg !38148
  %_0.i2587 = fmul float %history.i135.i.sroa.10.07570, %_59.i.i.i207.i, !dbg !38150
  %_0.i2159 = fadd float %_0.i2163, %_0.i2587, !dbg !38152
  %_0.i2586 = fmul float %history.i135.i.sroa.10.07570, %_62.i.i.i210.i, !dbg !38154
  %_0.i2158 = fadd float %_0.i2162, %_0.i2586, !dbg !38156
  %_0.i2585 = fmul float %history.i135.i.sroa.13.07571, %_67.i.i.i215.i, !dbg !38158
  %_0.i2157 = fadd float %_0.i2161, %_0.i2585, !dbg !38160
  %_0.i2584 = fmul float %history.i135.i.sroa.13.07571, %_70.i.i.i218.i, !dbg !38162
  %_0.i2156 = fadd float %_0.i2160, %_0.i2584, !dbg !38164
  %_0.i2583 = fmul float %history.i135.i.sroa.13.07571, %_73.i.i.i221.i, !dbg !38166
  %_0.i2155 = fadd float %_0.i2159, %_0.i2583, !dbg !38168
  %_0.i2582 = fmul float %history.i135.i.sroa.13.07571, %_76.i.i.i224.i, !dbg !38170
  %_0.i2154 = fadd float %_0.i2158, %_0.i2582, !dbg !38172
  %_0.i2581 = fmul float %history.i135.i.sroa.16.07572, %_81.i.i.i229.i, !dbg !38174
  %_0.i2153 = fadd float %_0.i2157, %_0.i2581, !dbg !38176
  %_0.i2580 = fmul float %history.i135.i.sroa.16.07572, %_84.i.i.i232.i, !dbg !38178
  %_0.i2152 = fadd float %_0.i2156, %_0.i2580, !dbg !38180
  %_0.i2579 = fmul float %history.i135.i.sroa.16.07572, %_87.i.i.i235.i, !dbg !38182
  %_0.i2151 = fadd float %_0.i2155, %_0.i2579, !dbg !38184
  %_0.i2578 = fmul float %history.i135.i.sroa.16.07572, %_90.i.i.i238.i, !dbg !38186
  %_0.i2150 = fadd float %_0.i2154, %_0.i2578, !dbg !38188
  %_0.i2577 = fmul float %history.i135.i.sroa.19.07573, %_95.i.i.i243.i, !dbg !38190
  %_0.i2149 = fadd float %_0.i2153, %_0.i2577, !dbg !38192
  %_0.i2576 = fmul float %history.i135.i.sroa.19.07573, %_98.i.i.i246.i, !dbg !38194
  %_0.i2148 = fadd float %_0.i2152, %_0.i2576, !dbg !38196
  %_0.i2575 = fmul float %history.i135.i.sroa.19.07573, %_101.i.i.i249.i, !dbg !38198
  %_0.i2147 = fadd float %_0.i2151, %_0.i2575, !dbg !38200
  %_0.i2574 = fmul float %history.i135.i.sroa.19.07573, %_104.i.i.i252.i, !dbg !38202
  %_0.i2146 = fadd float %_0.i2150, %_0.i2574, !dbg !38204
  %_0.i2573 = fmul float %history.i135.i.sroa.22.07574, %_109.i.i.i257.i, !dbg !38206
  %_0.i2145 = fadd float %_0.i2149, %_0.i2573, !dbg !38208
  %_0.i2572 = fmul float %history.i135.i.sroa.22.07574, %_112.i.i.i260.i, !dbg !38210
  %_0.i2144 = fadd float %_0.i2148, %_0.i2572, !dbg !38212
  %_0.i2571 = fmul float %history.i135.i.sroa.22.07574, %_115.i.i.i263.i, !dbg !38214
  %_0.i2143 = fadd float %_0.i2147, %_0.i2571, !dbg !38216
  %_0.i2570 = fmul float %history.i135.i.sroa.22.07574, %_118.i.i.i266.i, !dbg !38218
  %_0.i2142 = fadd float %_0.i2146, %_0.i2570, !dbg !38220
  %_0.i2569 = fmul float %history.i135.i.sroa.26.07575, %_123.i.i.i271.i, !dbg !38222
  %_0.i2141 = fadd float %_0.i2145, %_0.i2569, !dbg !38224
  %_0.i2568 = fmul float %history.i135.i.sroa.26.07575, %_126.i.i.i274.i, !dbg !38226
  %_0.i2140 = fadd float %_0.i2144, %_0.i2568, !dbg !38228
  %_0.i2567 = fmul float %history.i135.i.sroa.26.07575, %_129.i.i.i277.i, !dbg !38230
  %_0.i2139 = fadd float %_0.i2143, %_0.i2567, !dbg !38232
  %_0.i2566 = fmul float %history.i135.i.sroa.26.07575, %_132.i.i.i280.i, !dbg !38234
  %_0.i2138 = fadd float %_0.i2142, %_0.i2566, !dbg !38236
  %_0.i2565 = fmul float %history.i135.i.sroa.29.07576, %_137.i.i.i285.i, !dbg !38238
  %_0.i2137 = fadd float %_0.i2141, %_0.i2565, !dbg !38240
  %_0.i2564 = fmul float %history.i135.i.sroa.29.07576, %_140.i.i.i288.i, !dbg !38242
  %_0.i2136 = fadd float %_0.i2140, %_0.i2564, !dbg !38244
  %_0.i2563 = fmul float %history.i135.i.sroa.29.07576, %_143.i.i.i291.i, !dbg !38246
  %_0.i2135 = fadd float %_0.i2139, %_0.i2563, !dbg !38248
  %_0.i2562 = fmul float %history.i135.i.sroa.29.07576, %_146.i.i.i294.i, !dbg !38250
  %_0.i2134 = fadd float %_0.i2138, %_0.i2562, !dbg !38252
  %_0.i2561 = fmul float %history.i135.i.sroa.32.07577, %_151.i.i.i299.i, !dbg !38254
  %_0.i2133 = fadd float %_0.i2137, %_0.i2561, !dbg !38256
  %_0.i2560 = fmul float %history.i135.i.sroa.32.07577, %_154.i.i.i302.i, !dbg !38258
  %_0.i2132 = fadd float %_0.i2136, %_0.i2560, !dbg !38260
  %_0.i2559 = fmul float %history.i135.i.sroa.32.07577, %_157.i.i.i305.i, !dbg !38262
  %_0.i2131 = fadd float %_0.i2135, %_0.i2559, !dbg !38264
  %_0.i2558 = fmul float %history.i135.i.sroa.32.07577, %_160.i.i.i308.i, !dbg !38266
  %_0.i2130 = fadd float %_0.i2134, %_0.i2558, !dbg !38268
  %_0.i2557 = fmul float %history.i135.i.sroa.35.07578, %_165.i.i.i313.i, !dbg !38270
  %_0.i2129 = fadd float %_0.i2133, %_0.i2557, !dbg !38272
  %_0.i2556 = fmul float %history.i135.i.sroa.35.07578, %_168.i.i.i316.i, !dbg !38274
  %_0.i2128 = fadd float %_0.i2132, %_0.i2556, !dbg !38276
  %_0.i2555 = fmul float %history.i135.i.sroa.35.07578, %_171.i.i.i319.i, !dbg !38278
  %_0.i2127 = fadd float %_0.i2131, %_0.i2555, !dbg !38280
  %_0.i2554 = fmul float %history.i135.i.sroa.35.07578, %_174.i.i.i322.i, !dbg !38282
  %_0.i2126 = fadd float %_0.i2130, %_0.i2554, !dbg !38284
  %396 = tail call noundef float @llvm.fabs.f32(float %_0.i2129), !dbg !38286
  %_3.i.i3630.inv = fcmp ogt float %395, %396, !dbg !38288
  %_4.i.i3637.v = select i1 %_3.i.i3630.inv, float %395, float %396, !dbg !38288
  %397 = tail call noundef float @llvm.fabs.f32(float %_0.i2128), !dbg !38286
  %_3.i.i3630.inv.1 = fcmp ogt float %_4.i.i3637.v, %397, !dbg !38288
  %_4.i.i3637.v.1 = select i1 %_3.i.i3630.inv.1, float %_4.i.i3637.v, float %397, !dbg !38288
  %398 = tail call noundef float @llvm.fabs.f32(float %_0.i2127), !dbg !38286
  %_3.i.i3630.inv.2 = fcmp ogt float %_4.i.i3637.v.1, %398, !dbg !38288
  %_4.i.i3637.v.2 = select i1 %_3.i.i3630.inv.2, float %_4.i.i3637.v.1, float %398, !dbg !38288
  %399 = tail call noundef float @llvm.fabs.f32(float %_0.i2126), !dbg !38286
  %_3.i.i3630.inv.3 = fcmp ogt float %_4.i.i3637.v.2, %399, !dbg !38288
  %_4.i.i3637.v.3 = select i1 %_3.i.i3630.inv.3, float %_4.i.i3637.v.2, float %399, !dbg !38288
  %_39.i333.i = getelementptr inbounds nuw float, ptr %peaks_left.i430, i64 %iter.sroa.0.0.i137.i7579, !dbg !38291
  store float %_4.i.i3637.v.3, ptr %_39.i333.i, align 4, !dbg !38296, !alias.scope !38298, !noalias !38089
  %exitcond11103.not = icmp eq i64 %394, %umax11120, !dbg !38065
  br i1 %exitcond11103.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit339.i, label %bb5.i139.i, !dbg !38069

bb7.i338.i:                                       ; preds = %bb5.i139.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_11.i140.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_283c8d9f84e10f29a75d8e6c3a347bea) #30, !dbg !38301, !noalias !38089
  unreachable, !dbg !38301

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit339.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997, %bb37.i
  %history.i135.i.sroa.0.0.lcssa = phi float [ %history.i135.i.sroa.0.0.copyload, %bb37.i ], [ %_0.i2995, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997 ], !dbg !38302
  %history.i135.i.sroa.7.0.lcssa = phi float [ %history.i135.i.sroa.7.0.copyload, %bb37.i ], [ %history.i135.i.sroa.0.07568, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997 ], !dbg !38302
  %history.i135.i.sroa.10.0.lcssa = phi float [ %history.i135.i.sroa.10.0.copyload, %bb37.i ], [ %history.i135.i.sroa.7.07569, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997 ], !dbg !38302
  %history.i135.i.sroa.13.0.lcssa = phi float [ %history.i135.i.sroa.13.0.copyload, %bb37.i ], [ %history.i135.i.sroa.10.07570, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997 ], !dbg !38302
  %history.i135.i.sroa.16.0.lcssa = phi float [ %history.i135.i.sroa.16.0.copyload, %bb37.i ], [ %history.i135.i.sroa.13.07571, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997 ], !dbg !38302
  %history.i135.i.sroa.19.0.lcssa = phi float [ %history.i135.i.sroa.19.0.copyload, %bb37.i ], [ %history.i135.i.sroa.16.07572, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997 ], !dbg !38302
  %history.i135.i.sroa.22.0.lcssa = phi float [ %history.i135.i.sroa.22.0.copyload, %bb37.i ], [ %history.i135.i.sroa.19.07573, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997 ], !dbg !38302
  %history.i135.i.sroa.26.0.lcssa = phi float [ %history.i135.i.sroa.26.0.copyload, %bb37.i ], [ %history.i135.i.sroa.22.07574, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997 ], !dbg !38302
  %history.i135.i.sroa.29.0.lcssa = phi float [ %history.i135.i.sroa.29.0.copyload, %bb37.i ], [ %history.i135.i.sroa.26.07575, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997 ], !dbg !38302
  %history.i135.i.sroa.32.0.lcssa = phi float [ %history.i135.i.sroa.32.0.copyload, %bb37.i ], [ %history.i135.i.sroa.29.07576, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997 ], !dbg !38302
  %history.i135.i.sroa.35.0.lcssa = phi float [ %history.i135.i.sroa.35.0.copyload, %bb37.i ], [ %history.i135.i.sroa.32.07577, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997 ], !dbg !38302
  %history.i135.i.sroa.38.0.lcssa = phi float [ %history.i135.i.sroa.38.0.copyload, %bb37.i ], [ %history.i135.i.sroa.35.07578, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit2997 ], !dbg !38302
  store float %history.i135.i.sroa.0.0.lcssa, ptr %hot_left.i432, align 4, !dbg !38303
  store float %history.i135.i.sroa.7.0.lcssa, ptr %history.i135.i.sroa.7.0.hot_left.i432.sroa_idx, align 4, !dbg !38303
  store float %history.i135.i.sroa.10.0.lcssa, ptr %history.i135.i.sroa.10.0.hot_left.i432.sroa_idx, align 4, !dbg !38303
  store float %history.i135.i.sroa.13.0.lcssa, ptr %history.i135.i.sroa.13.0.hot_left.i432.sroa_idx, align 4, !dbg !38303
  store float %history.i135.i.sroa.16.0.lcssa, ptr %history.i135.i.sroa.16.0.hot_left.i432.sroa_idx, align 4, !dbg !38303
  store float %history.i135.i.sroa.19.0.lcssa, ptr %history.i135.i.sroa.19.0.hot_left.i432.sroa_idx, align 4, !dbg !38303
  store float %history.i135.i.sroa.22.0.lcssa, ptr %history.i135.i.sroa.22.0.hot_left.i432.sroa_idx, align 4, !dbg !38303
  store float %history.i135.i.sroa.26.0.lcssa, ptr %history.i135.i.sroa.26.0.hot_left.i432.sroa_idx, align 4, !dbg !38303
  store float %history.i135.i.sroa.29.0.lcssa, ptr %history.i135.i.sroa.29.0.hot_left.i432.sroa_idx, align 4, !dbg !38303
  store float %history.i135.i.sroa.32.0.lcssa, ptr %history.i135.i.sroa.32.0.hot_left.i432.sroa_idx, align 4, !dbg !38303
  store float %history.i135.i.sroa.35.0.lcssa, ptr %history.i135.i.sroa.35.0.hot_left.i432.sroa_idx, align 4, !dbg !38303
  store float %history.i135.i.sroa.38.0.lcssa, ptr %history.i135.i.sroa.38.0.hot_left.i432.sroa_idx, align 4, !dbg !38303
  %history.i.i428.sroa.0.0.copyload = load float, ptr %hot_right.i431, align 4, !dbg !38304
  %history.i.i428.sroa.7.0.copyload = load float, ptr %history.i.i428.sroa.7.0.hot_right.i431.sroa_idx, align 4, !dbg !38304
  %history.i.i428.sroa.10.0.copyload = load float, ptr %history.i.i428.sroa.10.0.hot_right.i431.sroa_idx, align 4, !dbg !38304
  %history.i.i428.sroa.13.0.copyload = load float, ptr %history.i.i428.sroa.13.0.hot_right.i431.sroa_idx, align 4, !dbg !38304
  %history.i.i428.sroa.16.0.copyload = load float, ptr %history.i.i428.sroa.16.0.hot_right.i431.sroa_idx, align 4, !dbg !38304
  %history.i.i428.sroa.19.0.copyload = load float, ptr %history.i.i428.sroa.19.0.hot_right.i431.sroa_idx, align 4, !dbg !38304
  %history.i.i428.sroa.22.0.copyload = load float, ptr %history.i.i428.sroa.22.0.hot_right.i431.sroa_idx, align 4, !dbg !38304
  %history.i.i428.sroa.26.0.copyload = load float, ptr %history.i.i428.sroa.26.0.hot_right.i431.sroa_idx, align 4, !dbg !38304
  %history.i.i428.sroa.29.0.copyload = load float, ptr %history.i.i428.sroa.29.0.hot_right.i431.sroa_idx, align 4, !dbg !38304
  %history.i.i428.sroa.32.0.copyload = load float, ptr %history.i.i428.sroa.32.0.hot_right.i431.sroa_idx, align 4, !dbg !38304
  %history.i.i428.sroa.35.0.copyload = load float, ptr %history.i.i428.sroa.35.0.hot_right.i431.sroa_idx, align 4, !dbg !38304
  %history.i.i428.sroa.38.0.copyload = load float, ptr %history.i.i428.sroa.38.0.hot_right.i431.sroa_idx, align 4, !dbg !38304
  br i1 %_20.i138.i7567.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i453, label %bb5.i.i483.lr.ph, !dbg !38306

bb5.i.i483.lr.ph:                                 ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit339.i
  %_11.i.i.i.i502 = load float, ptr %_31, align 4
  %_14.i.i.i.i505 = load float, ptr %323, align 4
  %_17.i.i.i.i508 = load float, ptr %324, align 4
  %_20.i.i.i.i511 = load float, ptr %325, align 4
  %_25.i.i.i.i516 = load float, ptr %row1.i.i.i171.i, align 4
  %_28.i.i.i.i519 = load float, ptr %326, align 4
  %_31.i.i.i.i522 = load float, ptr %327, align 4
  %_34.i.i.i.i525 = load float, ptr %328, align 4
  %_39.i.i.i.i530 = load float, ptr %row3.i.i.i185.i, align 4
  %_42.i.i.i.i533 = load float, ptr %329, align 4
  %_45.i.i.i.i536 = load float, ptr %330, align 4
  %_48.i.i.i.i539 = load float, ptr %331, align 4
  %_53.i.i.i.i544 = load float, ptr %row5.i.i.i199.i, align 4
  %_56.i.i.i.i547 = load float, ptr %332, align 4
  %_59.i.i.i.i550 = load float, ptr %333, align 4
  %_62.i.i.i.i553 = load float, ptr %334, align 4
  %_67.i.i.i.i558 = load float, ptr %row7.i.i.i213.i, align 4
  %_70.i.i.i.i561 = load float, ptr %335, align 4
  %_73.i.i.i.i564 = load float, ptr %336, align 4
  %_76.i.i.i.i567 = load float, ptr %337, align 4
  %_81.i.i.i.i572 = load float, ptr %row9.i.i.i227.i, align 4
  %_84.i.i.i.i575 = load float, ptr %338, align 4
  %_87.i.i.i.i578 = load float, ptr %339, align 4
  %_90.i.i.i.i581 = load float, ptr %340, align 4
  %_95.i.i.i.i586 = load float, ptr %row11.i.i.i241.i, align 4
  %_98.i.i.i.i589 = load float, ptr %341, align 4
  %_101.i.i.i.i592 = load float, ptr %342, align 4
  %_104.i.i.i.i595 = load float, ptr %343, align 4
  %_109.i.i.i.i600 = load float, ptr %row13.i.i.i255.i, align 4
  %_112.i.i.i.i603 = load float, ptr %344, align 4
  %_115.i.i.i.i606 = load float, ptr %345, align 4
  %_118.i.i.i.i609 = load float, ptr %346, align 4
  %_123.i.i.i.i614 = load float, ptr %row15.i.i.i269.i, align 4
  %_126.i.i.i.i617 = load float, ptr %347, align 4
  %_129.i.i.i.i620 = load float, ptr %348, align 4
  %_132.i.i.i.i623 = load float, ptr %349, align 4
  %_137.i.i.i.i628 = load float, ptr %row17.i.i.i283.i, align 4
  %_140.i.i.i.i631 = load float, ptr %350, align 4
  %_143.i.i.i.i634 = load float, ptr %351, align 4
  %_146.i.i.i.i637 = load float, ptr %352, align 4
  %_151.i.i.i.i642 = load float, ptr %row19.i.i.i297.i, align 4
  %_154.i.i.i.i645 = load float, ptr %353, align 4
  %_157.i.i.i.i648 = load float, ptr %354, align 4
  %_160.i.i.i.i651 = load float, ptr %355, align 4
  %_165.i.i.i.i656 = load float, ptr %row21.i.i.i311.i, align 4
  %_168.i.i.i.i659 = load float, ptr %356, align 4
  %_171.i.i.i.i662 = load float, ptr %357, align 4
  %_174.i.i.i.i665 = load float, ptr %358, align 4
  br label %bb5.i.i483, !dbg !38306

bb5.i.i483:                                       ; preds = %bb5.i.i483.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002
  %iter.sroa.0.0.i.i4517606 = phi i64 [ 0, %bb5.i.i483.lr.ph ], [ %400, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002 ]
  %history.i.i428.sroa.35.07605 = phi float [ %history.i.i428.sroa.35.0.copyload, %bb5.i.i483.lr.ph ], [ %history.i.i428.sroa.32.07604, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002 ]
  %history.i.i428.sroa.32.07604 = phi float [ %history.i.i428.sroa.32.0.copyload, %bb5.i.i483.lr.ph ], [ %history.i.i428.sroa.29.07603, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002 ]
  %history.i.i428.sroa.29.07603 = phi float [ %history.i.i428.sroa.29.0.copyload, %bb5.i.i483.lr.ph ], [ %history.i.i428.sroa.26.07602, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002 ]
  %history.i.i428.sroa.26.07602 = phi float [ %history.i.i428.sroa.26.0.copyload, %bb5.i.i483.lr.ph ], [ %history.i.i428.sroa.22.07601, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002 ]
  %history.i.i428.sroa.22.07601 = phi float [ %history.i.i428.sroa.22.0.copyload, %bb5.i.i483.lr.ph ], [ %history.i.i428.sroa.19.07600, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002 ]
  %history.i.i428.sroa.19.07600 = phi float [ %history.i.i428.sroa.19.0.copyload, %bb5.i.i483.lr.ph ], [ %history.i.i428.sroa.16.07599, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002 ]
  %history.i.i428.sroa.16.07599 = phi float [ %history.i.i428.sroa.16.0.copyload, %bb5.i.i483.lr.ph ], [ %history.i.i428.sroa.13.07598, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002 ]
  %history.i.i428.sroa.13.07598 = phi float [ %history.i.i428.sroa.13.0.copyload, %bb5.i.i483.lr.ph ], [ %history.i.i428.sroa.10.07597, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002 ]
  %history.i.i428.sroa.10.07597 = phi float [ %history.i.i428.sroa.10.0.copyload, %bb5.i.i483.lr.ph ], [ %history.i.i428.sroa.7.07596, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002 ]
  %history.i.i428.sroa.7.07596 = phi float [ %history.i.i428.sroa.7.0.copyload, %bb5.i.i483.lr.ph ], [ %history.i.i428.sroa.0.07595, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002 ]
  %history.i.i428.sroa.0.07595 = phi float [ %history.i.i428.sroa.0.0.copyload, %bb5.i.i483.lr.ph ], [ %_0.i3000, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002 ]
  %400 = add nuw nsw i64 %iter.sroa.0.0.i.i4517606, 1, !dbg !38309
  %_11.i.i484 = add nuw nsw i64 %iter.sroa.0.0.i.i4517606, %iter.sroa.0.0.i7790, !dbg !38312
  %_24.i.i485 = icmp ugt i64 %_11.i.i484, %right_io.1, !dbg !38313
  br i1 %_24.i.i485, label %bb7.i.i681, label %bb8.i.i486, !dbg !38313, !prof !639

bb8.i.i486:                                       ; preds = %bb5.i.i483
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38316), !dbg !38319
  %_3.not.i2998 = icmp eq i64 %right_io.1, %_11.i.i484, !dbg !38320
  br i1 %_3.not.i2998, label %panic.i3001, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002, !dbg !38320

panic.i3001:                                      ; preds = %bb8.i.i486
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !38320, !noalias !38322
  unreachable, !dbg !38320

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002: ; preds = %bb8.i.i486
  %_31.i126.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %_11.i.i484, !dbg !38326
  %_0.i3000 = load float, ptr %_31.i126.i, align 4, !dbg !38320, !alias.scope !38316, !noalias !38328, !noundef !12
  %401 = tail call noundef float @llvm.fabs.f32(float %history.i.i428.sroa.19.07600), !dbg !38329
  %_0.i2649 = fmul float %_0.i3000, %_11.i.i.i.i502, !dbg !38332
  %_0.i2221 = fadd float %_0.i2649, 0.000000e+00, !dbg !38335
  %_0.i2648 = fmul float %_0.i3000, %_14.i.i.i.i505, !dbg !38337
  %_0.i2220 = fadd float %_0.i2648, 0.000000e+00, !dbg !38339
  %_0.i2647 = fmul float %_0.i3000, %_17.i.i.i.i508, !dbg !38341
  %_0.i2219 = fadd float %_0.i2647, 0.000000e+00, !dbg !38343
  %_0.i2646 = fmul float %_0.i3000, %_20.i.i.i.i511, !dbg !38345
  %_0.i2218 = fadd float %_0.i2646, 0.000000e+00, !dbg !38347
  %_0.i2645 = fmul float %history.i.i428.sroa.0.07595, %_25.i.i.i.i516, !dbg !38349
  %_0.i2217 = fadd float %_0.i2221, %_0.i2645, !dbg !38351
  %_0.i2644 = fmul float %history.i.i428.sroa.0.07595, %_28.i.i.i.i519, !dbg !38353
  %_0.i2216 = fadd float %_0.i2220, %_0.i2644, !dbg !38355
  %_0.i2643 = fmul float %history.i.i428.sroa.0.07595, %_31.i.i.i.i522, !dbg !38357
  %_0.i2215 = fadd float %_0.i2219, %_0.i2643, !dbg !38359
  %_0.i2642 = fmul float %history.i.i428.sroa.0.07595, %_34.i.i.i.i525, !dbg !38361
  %_0.i2214 = fadd float %_0.i2218, %_0.i2642, !dbg !38363
  %_0.i2641 = fmul float %history.i.i428.sroa.7.07596, %_39.i.i.i.i530, !dbg !38365
  %_0.i2213 = fadd float %_0.i2217, %_0.i2641, !dbg !38367
  %_0.i2640 = fmul float %history.i.i428.sroa.7.07596, %_42.i.i.i.i533, !dbg !38369
  %_0.i2212 = fadd float %_0.i2216, %_0.i2640, !dbg !38371
  %_0.i2639 = fmul float %history.i.i428.sroa.7.07596, %_45.i.i.i.i536, !dbg !38373
  %_0.i2211 = fadd float %_0.i2215, %_0.i2639, !dbg !38375
  %_0.i2638 = fmul float %history.i.i428.sroa.7.07596, %_48.i.i.i.i539, !dbg !38377
  %_0.i2210 = fadd float %_0.i2214, %_0.i2638, !dbg !38379
  %_0.i2637 = fmul float %history.i.i428.sroa.10.07597, %_53.i.i.i.i544, !dbg !38381
  %_0.i2209 = fadd float %_0.i2213, %_0.i2637, !dbg !38383
  %_0.i2636 = fmul float %history.i.i428.sroa.10.07597, %_56.i.i.i.i547, !dbg !38385
  %_0.i2208 = fadd float %_0.i2212, %_0.i2636, !dbg !38387
  %_0.i2635 = fmul float %history.i.i428.sroa.10.07597, %_59.i.i.i.i550, !dbg !38389
  %_0.i2207 = fadd float %_0.i2211, %_0.i2635, !dbg !38391
  %_0.i2634 = fmul float %history.i.i428.sroa.10.07597, %_62.i.i.i.i553, !dbg !38393
  %_0.i2206 = fadd float %_0.i2210, %_0.i2634, !dbg !38395
  %_0.i2633 = fmul float %history.i.i428.sroa.13.07598, %_67.i.i.i.i558, !dbg !38397
  %_0.i2205 = fadd float %_0.i2209, %_0.i2633, !dbg !38399
  %_0.i2632 = fmul float %history.i.i428.sroa.13.07598, %_70.i.i.i.i561, !dbg !38401
  %_0.i2204 = fadd float %_0.i2208, %_0.i2632, !dbg !38403
  %_0.i2631 = fmul float %history.i.i428.sroa.13.07598, %_73.i.i.i.i564, !dbg !38405
  %_0.i2203 = fadd float %_0.i2207, %_0.i2631, !dbg !38407
  %_0.i2630 = fmul float %history.i.i428.sroa.13.07598, %_76.i.i.i.i567, !dbg !38409
  %_0.i2202 = fadd float %_0.i2206, %_0.i2630, !dbg !38411
  %_0.i2629 = fmul float %history.i.i428.sroa.16.07599, %_81.i.i.i.i572, !dbg !38413
  %_0.i2201 = fadd float %_0.i2205, %_0.i2629, !dbg !38415
  %_0.i2628 = fmul float %history.i.i428.sroa.16.07599, %_84.i.i.i.i575, !dbg !38417
  %_0.i2200 = fadd float %_0.i2204, %_0.i2628, !dbg !38419
  %_0.i2627 = fmul float %history.i.i428.sroa.16.07599, %_87.i.i.i.i578, !dbg !38421
  %_0.i2199 = fadd float %_0.i2203, %_0.i2627, !dbg !38423
  %_0.i2626 = fmul float %history.i.i428.sroa.16.07599, %_90.i.i.i.i581, !dbg !38425
  %_0.i2198 = fadd float %_0.i2202, %_0.i2626, !dbg !38427
  %_0.i2625 = fmul float %history.i.i428.sroa.19.07600, %_95.i.i.i.i586, !dbg !38429
  %_0.i2197 = fadd float %_0.i2201, %_0.i2625, !dbg !38431
  %_0.i2624 = fmul float %history.i.i428.sroa.19.07600, %_98.i.i.i.i589, !dbg !38433
  %_0.i2196 = fadd float %_0.i2200, %_0.i2624, !dbg !38435
  %_0.i2623 = fmul float %history.i.i428.sroa.19.07600, %_101.i.i.i.i592, !dbg !38437
  %_0.i2195 = fadd float %_0.i2199, %_0.i2623, !dbg !38439
  %_0.i2622 = fmul float %history.i.i428.sroa.19.07600, %_104.i.i.i.i595, !dbg !38441
  %_0.i2194 = fadd float %_0.i2198, %_0.i2622, !dbg !38443
  %_0.i2621 = fmul float %history.i.i428.sroa.22.07601, %_109.i.i.i.i600, !dbg !38445
  %_0.i2193 = fadd float %_0.i2197, %_0.i2621, !dbg !38447
  %_0.i2620 = fmul float %history.i.i428.sroa.22.07601, %_112.i.i.i.i603, !dbg !38449
  %_0.i2192 = fadd float %_0.i2196, %_0.i2620, !dbg !38451
  %_0.i2619 = fmul float %history.i.i428.sroa.22.07601, %_115.i.i.i.i606, !dbg !38453
  %_0.i2191 = fadd float %_0.i2195, %_0.i2619, !dbg !38455
  %_0.i2618 = fmul float %history.i.i428.sroa.22.07601, %_118.i.i.i.i609, !dbg !38457
  %_0.i2190 = fadd float %_0.i2194, %_0.i2618, !dbg !38459
  %_0.i2617 = fmul float %history.i.i428.sroa.26.07602, %_123.i.i.i.i614, !dbg !38461
  %_0.i2189 = fadd float %_0.i2193, %_0.i2617, !dbg !38463
  %_0.i2616 = fmul float %history.i.i428.sroa.26.07602, %_126.i.i.i.i617, !dbg !38465
  %_0.i2188 = fadd float %_0.i2192, %_0.i2616, !dbg !38467
  %_0.i2615 = fmul float %history.i.i428.sroa.26.07602, %_129.i.i.i.i620, !dbg !38469
  %_0.i2187 = fadd float %_0.i2191, %_0.i2615, !dbg !38471
  %_0.i2614 = fmul float %history.i.i428.sroa.26.07602, %_132.i.i.i.i623, !dbg !38473
  %_0.i2186 = fadd float %_0.i2190, %_0.i2614, !dbg !38475
  %_0.i2613 = fmul float %history.i.i428.sroa.29.07603, %_137.i.i.i.i628, !dbg !38477
  %_0.i2185 = fadd float %_0.i2189, %_0.i2613, !dbg !38479
  %_0.i2612 = fmul float %history.i.i428.sroa.29.07603, %_140.i.i.i.i631, !dbg !38481
  %_0.i2184 = fadd float %_0.i2188, %_0.i2612, !dbg !38483
  %_0.i2611 = fmul float %history.i.i428.sroa.29.07603, %_143.i.i.i.i634, !dbg !38485
  %_0.i2183 = fadd float %_0.i2187, %_0.i2611, !dbg !38487
  %_0.i2610 = fmul float %history.i.i428.sroa.29.07603, %_146.i.i.i.i637, !dbg !38489
  %_0.i2182 = fadd float %_0.i2186, %_0.i2610, !dbg !38491
  %_0.i2609 = fmul float %history.i.i428.sroa.32.07604, %_151.i.i.i.i642, !dbg !38493
  %_0.i2181 = fadd float %_0.i2185, %_0.i2609, !dbg !38495
  %_0.i2608 = fmul float %history.i.i428.sroa.32.07604, %_154.i.i.i.i645, !dbg !38497
  %_0.i2180 = fadd float %_0.i2184, %_0.i2608, !dbg !38499
  %_0.i2607 = fmul float %history.i.i428.sroa.32.07604, %_157.i.i.i.i648, !dbg !38501
  %_0.i2179 = fadd float %_0.i2183, %_0.i2607, !dbg !38503
  %_0.i2606 = fmul float %history.i.i428.sroa.32.07604, %_160.i.i.i.i651, !dbg !38505
  %_0.i2178 = fadd float %_0.i2182, %_0.i2606, !dbg !38507
  %_0.i2605 = fmul float %history.i.i428.sroa.35.07605, %_165.i.i.i.i656, !dbg !38509
  %_0.i2177 = fadd float %_0.i2181, %_0.i2605, !dbg !38511
  %_0.i2604 = fmul float %history.i.i428.sroa.35.07605, %_168.i.i.i.i659, !dbg !38513
  %_0.i2176 = fadd float %_0.i2180, %_0.i2604, !dbg !38515
  %_0.i2603 = fmul float %history.i.i428.sroa.35.07605, %_171.i.i.i.i662, !dbg !38517
  %_0.i2175 = fadd float %_0.i2179, %_0.i2603, !dbg !38519
  %_0.i2602 = fmul float %history.i.i428.sroa.35.07605, %_174.i.i.i.i665, !dbg !38521
  %_0.i2174 = fadd float %_0.i2178, %_0.i2602, !dbg !38523
  %402 = tail call noundef float @llvm.fabs.f32(float %_0.i2177), !dbg !38525
  %_3.i.i3639.inv = fcmp ogt float %401, %402, !dbg !38527
  %_4.i.i3646.v = select i1 %_3.i.i3639.inv, float %401, float %402, !dbg !38527
  %403 = tail call noundef float @llvm.fabs.f32(float %_0.i2176), !dbg !38525
  %_3.i.i3639.inv.1 = fcmp ogt float %_4.i.i3646.v, %403, !dbg !38527
  %_4.i.i3646.v.1 = select i1 %_3.i.i3639.inv.1, float %_4.i.i3646.v, float %403, !dbg !38527
  %404 = tail call noundef float @llvm.fabs.f32(float %_0.i2175), !dbg !38525
  %_3.i.i3639.inv.2 = fcmp ogt float %_4.i.i3646.v.1, %404, !dbg !38527
  %_4.i.i3646.v.2 = select i1 %_3.i.i3639.inv.2, float %_4.i.i3646.v.1, float %404, !dbg !38527
  %405 = tail call noundef float @llvm.fabs.f32(float %_0.i2174), !dbg !38525
  %_3.i.i3639.inv.3 = fcmp ogt float %_4.i.i3646.v.2, %405, !dbg !38527
  %_4.i.i3646.v.3 = select i1 %_3.i.i3639.inv.3, float %_4.i.i3646.v.2, float %405, !dbg !38527
  %_39.i.i676 = getelementptr inbounds nuw float, ptr %peaks_right.i429, i64 %iter.sroa.0.0.i.i4517606, !dbg !38530
  store float %_4.i.i3646.v.3, ptr %_39.i.i676, align 4, !dbg !38535, !alias.scope !38537, !noalias !38328
  %exitcond11106.not = icmp eq i64 %400, %umax11120, !dbg !38540
  br i1 %exitcond11106.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i453, label %bb5.i.i483, !dbg !38306

bb7.i.i681:                                       ; preds = %bb5.i.i483
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_11.i.i484, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_283c8d9f84e10f29a75d8e6c3a347bea) #30, !dbg !38542, !noalias !38328
  unreachable, !dbg !38542

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i453: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit339.i
  %history.i.i428.sroa.0.0.lcssa = phi float [ %history.i.i428.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %_0.i3000, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002 ], !dbg !38543
  %history.i.i428.sroa.7.0.lcssa = phi float [ %history.i.i428.sroa.7.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i428.sroa.0.07595, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002 ], !dbg !38543
  %history.i.i428.sroa.10.0.lcssa = phi float [ %history.i.i428.sroa.10.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i428.sroa.7.07596, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002 ], !dbg !38543
  %history.i.i428.sroa.13.0.lcssa = phi float [ %history.i.i428.sroa.13.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i428.sroa.10.07597, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002 ], !dbg !38543
  %history.i.i428.sroa.16.0.lcssa = phi float [ %history.i.i428.sroa.16.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i428.sroa.13.07598, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002 ], !dbg !38543
  %history.i.i428.sroa.19.0.lcssa = phi float [ %history.i.i428.sroa.19.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i428.sroa.16.07599, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002 ], !dbg !38543
  %history.i.i428.sroa.22.0.lcssa = phi float [ %history.i.i428.sroa.22.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i428.sroa.19.07600, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002 ], !dbg !38543
  %history.i.i428.sroa.26.0.lcssa = phi float [ %history.i.i428.sroa.26.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i428.sroa.22.07601, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002 ], !dbg !38543
  %history.i.i428.sroa.29.0.lcssa = phi float [ %history.i.i428.sroa.29.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i428.sroa.26.07602, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002 ], !dbg !38543
  %history.i.i428.sroa.32.0.lcssa = phi float [ %history.i.i428.sroa.32.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i428.sroa.29.07603, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002 ], !dbg !38543
  %history.i.i428.sroa.35.0.lcssa = phi float [ %history.i.i428.sroa.35.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i428.sroa.32.07604, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002 ], !dbg !38543
  %history.i.i428.sroa.38.0.lcssa = phi float [ %history.i.i428.sroa.38.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i428.sroa.35.07605, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3002 ], !dbg !38543
  store float %history.i.i428.sroa.0.0.lcssa, ptr %hot_right.i431, align 4, !dbg !38544
  store float %history.i.i428.sroa.7.0.lcssa, ptr %history.i.i428.sroa.7.0.hot_right.i431.sroa_idx, align 4, !dbg !38544
  store float %history.i.i428.sroa.10.0.lcssa, ptr %history.i.i428.sroa.10.0.hot_right.i431.sroa_idx, align 4, !dbg !38544
  store float %history.i.i428.sroa.13.0.lcssa, ptr %history.i.i428.sroa.13.0.hot_right.i431.sroa_idx, align 4, !dbg !38544
  store float %history.i.i428.sroa.16.0.lcssa, ptr %history.i.i428.sroa.16.0.hot_right.i431.sroa_idx, align 4, !dbg !38544
  store float %history.i.i428.sroa.19.0.lcssa, ptr %history.i.i428.sroa.19.0.hot_right.i431.sroa_idx, align 4, !dbg !38544
  store float %history.i.i428.sroa.22.0.lcssa, ptr %history.i.i428.sroa.22.0.hot_right.i431.sroa_idx, align 4, !dbg !38544
  store float %history.i.i428.sroa.26.0.lcssa, ptr %history.i.i428.sroa.26.0.hot_right.i431.sroa_idx, align 4, !dbg !38544
  store float %history.i.i428.sroa.29.0.lcssa, ptr %history.i.i428.sroa.29.0.hot_right.i431.sroa_idx, align 4, !dbg !38544
  store float %history.i.i428.sroa.32.0.lcssa, ptr %history.i.i428.sroa.32.0.hot_right.i431.sroa_idx, align 4, !dbg !38544
  store float %history.i.i428.sroa.35.0.lcssa, ptr %history.i.i428.sroa.35.0.hot_right.i431.sroa_idx, align 4, !dbg !38544
  store float %history.i.i428.sroa.38.0.lcssa, ptr %history.i.i428.sroa.38.0.hot_right.i431.sroa_idx, align 4, !dbg !38544
  br i1 %_20.i138.i7567.not, label %bb13.i.loopexit, label %bb43.i.lr.ph, !dbg !38050

bb43.i.lr.ph:                                     ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i453
  %_8.i30.i = load float, ptr %_63.i457, align 4, !noundef !12
  %_9.i31.i = load float, ptr %_64.i, align 4, !noundef !12
  %_8.i.i459 = load float, ptr %_68.i, align 4, !noundef !12
  %_9.i.i460 = load float, ptr %_69.i458, align 4, !noundef !12
  %_86.i = load i64, ptr %359, align 8
  %_64.i89.i = load float, ptr %370, align 4
  %_64.i.i = load float, ptr %386, align 4
  %_101.i = load i64, ptr %390, align 8
  %.promoted7647 = load float, ptr %369, align 4
  %.promoted7716 = load float, ptr %371, align 4
  %.promoted7718 = load float, ptr %385, align 4
  %.promoted7787 = load float, ptr %387, align 4
  br label %bb43.i, !dbg !38050

bb43.i:                                           ; preds = %bb43.i.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3154
  %_0.i32537788 = phi float [ %.promoted7787, %bb43.i.lr.ph ], [ %_0.i3253, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3154 ]
  %_0.i28797719 = phi float [ %.promoted7718, %bb43.i.lr.ph ], [ %_0.i2879, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3154 ]
  %_0.i32577717 = phi float [ %.promoted7716, %bb43.i.lr.ph ], [ %_0.i3257, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3154 ]
  %_0.i28837648 = phi float [ %.promoted7647, %bb43.i.lr.ph ], [ %_0.i2883, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3154 ]
  %main_cursor.sroa.0.1.i4567644 = phi i64 [ %main_cursor.sroa.0.0.i4477793, %bb43.i.lr.ph ], [ %spec.store.select.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3154 ]
  %ring_cursor.sroa.0.1.i4557643 = phi i64 [ %ring_cursor.sroa.0.0.i4467792, %bb43.i.lr.ph ], [ %spec.store.select13.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3154 ]
  %iter1.sroa.0.0.i4547642 = phi i64 [ 0, %bb43.i.lr.ph ], [ %406, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3154 ]
  %406 = add nuw nsw i64 %iter1.sroa.0.0.i4547642, 1, !dbg !38545
  %_59.i = add nuw nsw i64 %iter1.sroa.0.0.i4547642, %iter.sroa.0.0.i7790, !dbg !38551
  %_128.i = getelementptr inbounds nuw float, ptr %peaks_left.i430, i64 %iter1.sroa.0.0.i4547642, !dbg !38552
  %_0.i3038 = load float, ptr %_128.i, align 4, !dbg !38563, !alias.scope !38565, !noalias !38568, !noundef !12
  %_133.i = getelementptr inbounds nuw float, ptr %peaks_right.i429, i64 %iter1.sroa.0.0.i4547642, !dbg !38569
  %_0.i3033 = load float, ptr %_133.i, align 4, !dbg !38579, !alias.scope !38581, !noalias !38568, !noundef !12
  %_3.i.i3666 = fcmp ule float %_0.i3033, %_0.i3038, !dbg !38584
  %_6.i.i3668 = bitcast float %_0.i3033 to i32, !dbg !38587
  %_8.i.i3670 = bitcast float %_0.i3038 to i32, !dbg !38590
  %_4.i.i3673 = select i1 %_3.i.i3666, i32 %_8.i.i3670, i32 %_6.i.i3668, !dbg !38592
  %_5.i3449 = and i32 %_4.i.i3673, %.none.i437, !dbg !38593
  %_7.i3452 = and i32 %_9.i3451, %_8.i.i3670, !dbg !38595
  %_4.i3453 = or disjoint i32 %_5.i3449, %_7.i3452, !dbg !38593
  %_0.i3454 = bitcast i32 %_4.i3453 to float, !dbg !38596
  %_7.i3445 = and i32 %_9.i3451, %_6.i.i3668, !dbg !38598
  %_4.i3446 = or disjoint i32 %_5.i3449, %_7.i3445, !dbg !38600
  %_0.i3447 = bitcast i32 %_4.i3446 to float, !dbg !38601
  %_134.i = icmp ugt i64 %_59.i, %left_io.1, !dbg !38603
  br i1 %_134.i, label %bb44.i, label %bb45.i, !dbg !38603, !prof !639

bb45.i:                                           ; preds = %bb43.i
  %_141.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %_59.i, !dbg !38607
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38612), !dbg !38615
  %_3.not.i3026 = icmp eq i64 %left_io.1, %_59.i, !dbg !38616
  br i1 %_3.not.i3026, label %panic.i3029, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3030, !dbg !38616

panic.i3029:                                      ; preds = %bb45.i
  store float %_0.i32577717, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !38616, !noalias !38618
  unreachable, !dbg !38616

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3030: ; preds = %bb45.i
  %_0.i3028 = load float, ptr %_141.i, align 4, !dbg !38616, !alias.scope !38612, !noalias !38568, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38619), !dbg !38622
  %width.i33.i = load i64, ptr %360, align 8, !dbg !38623, !alias.scope !38624, !noalias !38625, !noundef !12
  %_3.i1994 = fcmp uge float %_8.i30.i, %_0.i3454, !dbg !38629
  %_0.i2427 = fdiv float %_8.i30.i, %_0.i3454, !dbg !38631
  %_0.i3440 = select i1 %_3.i1994, float 1.000000e+00, float %_0.i2427, !dbg !38633
  %_144.1.i38.i = load i64, ptr %361, align 8, !dbg !38635, !alias.scope !38624, !noalias !38625, !noundef !12
  %_22.i39.i = mul i64 %width.i33.i, %ring_cursor.sroa.0.1.i4557643, !dbg !38636
  %_92.i40.i = icmp ugt i64 %_22.i39.i, %_144.1.i38.i, !dbg !38637
  br i1 %_92.i40.i, label %bb37.i124.i, label %bb38.i41.i, !dbg !38637, !prof !639

bb38.i41.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3030
  %_144.0.i42.i = load ptr, ptr %362, align 8, !dbg !38635, !alias.scope !38624, !noalias !38625, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38640), !dbg !38643
  %_4.not.i3179 = icmp eq i64 %_144.1.i38.i, %_22.i39.i, !dbg !38644
  br i1 %_4.not.i3179, label %panic.i3181, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3182, !dbg !38644

panic.i3181:                                      ; preds = %bb38.i41.i
  store float %_0.i32577717, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !38644, !noalias !38646
  unreachable, !dbg !38644

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3182: ; preds = %bb38.i41.i
  %_99.i44.i = getelementptr inbounds nuw float, ptr %_144.0.i42.i, i64 %_22.i39.i, !dbg !38647
  store float %_0.i3440, ptr %_99.i44.i, align 4, !dbg !38644, !alias.scope !38640, !noalias !38649
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38650), !dbg !38653
  %width.i1712 = load i64, ptr %360, align 8, !dbg !38654, !alias.scope !38650, !noalias !38656, !noundef !12
  %407 = icmp eq i64 %width.i1712, 0, !dbg !38658
  br i1 %407, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1820, label %bb32.i1719.lr.ph, !dbg !38658

bb32.i1719.lr.ph:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3182
  %_112.1.i1722 = load i64, ptr %62, align 8, !alias.scope !38650, !noalias !38656, !noundef !12
  %_112.0.i1726 = load ptr, ptr %61, align 8, !nonnull !12
  %408 = add i64 %ring_cursor.sroa.0.1.i4557643, 1
  %_23.not.i1733 = icmp ult i64 %408, %_86.i
  %409 = select i1 %_23.not.i1733, i64 0, i64 %_86.i
  %start1.sroa.0.0.i1734 = sub nuw i64 %408, %409
  %_114.1.i1737 = load i64, ptr %361, align 8
  %_114.0.i1741 = load ptr, ptr %362, align 8, !nonnull !12
  %_116.1.i1742 = load i64, ptr %363, align 8
  %_116.0.i1746 = load ptr, ptr %364, align 8, !nonnull !12
  %_118.1.i1750 = load i64, ptr %365, align 8
  %_118.0.i1754 = load ptr, ptr %366, align 8, !nonnull !12
  %_45.i1767 = mul i64 %width.i1712, %start1.sroa.0.0.i1734
  br label %bb32.i1719, !dbg !38658

bb32.i1719:                                       ; preds = %bb32.i1719.lr.ph, %bb31.i1782
  %iter.i1711.sroa.10.07624 = phi i64 [ %width.i1712, %bb32.i1719.lr.ph ], [ %410, %bb31.i1782 ]
  %iter.i1711.sroa.7.07623 = phi i64 [ 0, %bb32.i1719.lr.ph ], [ %_9.0.i4126, %bb31.i1782 ]
  %iter.i1711.sroa.0.0.idx7622 = phi i64 [ 0, %bb32.i1719.lr.ph ], [ %iter.i1711.sroa.0.0.add, %bb31.i1782 ]
  %iter.i1711.sroa.0.0.ptr7625 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 %iter.i1711.sroa.0.0.idx7622, !dbg !38660
  %410 = add i64 %iter.i1711.sroa.10.07624, -1, !dbg !38660
  %_7.i.i4122 = icmp eq i64 %iter.i1711.sroa.0.0.idx7622, 32, !dbg !38661
  br i1 %_7.i.i4122, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1820, label %bb3.i1721, !dbg !38665

bb3.i1721:                                        ; preds = %bb32.i1719
  %iter.i1711.sroa.0.0.add = add nuw nsw i64 %iter.i1711.sroa.0.0.idx7622, 4, !dbg !38666
  %_9.0.i4126 = add nuw nsw i64 %iter.i1711.sroa.7.07623, 1, !dbg !38668
  %exitcond11109.not = icmp eq i64 %iter.i1711.sroa.7.07623, %_112.1.i1722, !dbg !38669
  br i1 %exitcond11109.not, label %panic.i1724, label %bb5.i1725, !dbg !38669

bb5.i1725:                                        ; preds = %bb3.i1721
  %411 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i1726, i64 %iter.i1711.sroa.7.07623, !dbg !38669
  %shape.i1727 = load i32, ptr %411, align 4, !dbg !38669, !noalias !38670, !noundef !12
  %412 = getelementptr inbounds nuw i8, ptr %411, i64 4, !dbg !38669
  %shape3.i1728 = load i32, ptr %412, align 4, !dbg !38669, !noalias !38670, !noundef !12
  %window.i1729 = zext i32 %shape.i1727 to i64, !dbg !38671
  %_19.i1730 = zext i32 %shape3.i1728 to i64, !dbg !38672
  %413 = add i64 %ring_cursor.sroa.0.1.i4557643, %_19.i1730, !dbg !38673
  %_20.not.i1731 = icmp ult i64 %413, %_86.i, !dbg !38674
  %414 = select i1 %_20.not.i1731, i64 0, i64 %_86.i, !dbg !38674
  %spec.select.i1732 = sub nuw i64 %413, %414, !dbg !38674
  %_27.i1735 = mul i64 %spec.select.i1732, %width.i1712, !dbg !38675
  %_26.i1736 = add i64 %_27.i1735, %iter.i1711.sroa.7.07623, !dbg !38675
  %_30.i1738 = icmp ult i64 %_26.i1736, %_114.1.i1737, !dbg !38676
  br i1 %_30.i1738, label %bb12.i1740, label %panic5.i1739, !dbg !38676

panic.i1724:                                      ; preds = %bb3.i1721
  store float %_0.i32577717, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i1722, i64 noundef %_112.1.i1722, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4a8785a681d008a9bfd0cd82628ea9cb) #30, !dbg !38669, !noalias !38670
  unreachable, !dbg !38669

bb12.i1740:                                       ; preds = %bb5.i1725
  %415 = getelementptr inbounds nuw float, ptr %_114.0.i1741, i64 %_26.i1736, !dbg !38676
  %416 = load float, ptr %415, align 4, !dbg !38676, !noalias !38670, !noundef !12
  %exitcond11110.not = icmp eq i64 %iter.i1711.sroa.7.07623, %_116.1.i1742, !dbg !38677
  br i1 %exitcond11110.not, label %panic6.i1744, label %bb13.i1745, !dbg !38677

panic5.i1739:                                     ; preds = %bb5.i1725
  store float %_0.i32577717, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i1736, i64 noundef %_114.1.i1737, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cbce7773ac40979e4ba2385da3aec116) #30, !dbg !38676, !noalias !38670
  unreachable, !dbg !38676

bb13.i1745:                                       ; preds = %bb12.i1740
  %417 = getelementptr inbounds nuw i32, ptr %_116.0.i1746, i64 %iter.i1711.sroa.7.07623, !dbg !38677
  %_32.i1747 = load i32, ptr %417, align 4, !dbg !38677, !noalias !38670, !noundef !12
  %position.i1748 = zext i32 %_32.i1747 to i64, !dbg !38677
  %418 = icmp eq i32 %_32.i1747, 0, !dbg !38678
  br i1 %418, label %bb17.i1757, label %bb15.i1749, !dbg !38678

panic6.i1744:                                     ; preds = %bb12.i1740
  store float %_0.i32577717, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i1742, i64 noundef %_116.1.i1742, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ec0d48f73ebfc2755df5cedaa60b5c0a) #30, !dbg !38677, !noalias !38670
  unreachable, !dbg !38677

bb15.i1749:                                       ; preds = %bb13.i1745
  %_37.i1751 = icmp ult i64 %iter.i1711.sroa.7.07623, %_118.1.i1750, !dbg !38679
  br i1 %_37.i1751, label %bb16.i1753, label %panic7.i1752, !dbg !38679

bb17.i1757:                                       ; preds = %bb35.i1818, %bb16.i1753, %bb13.i1745
  %newest.sroa.0.0.i1758 = phi float [ %416, %bb13.i1745 ], [ %_35.i1755, %bb35.i1818 ], [ %416, %bb16.i1753 ], !dbg !38680
  %exitcond11111.not = icmp eq i64 %iter.i1711.sroa.7.07623, %_118.1.i1750, !dbg !38681
  br i1 %exitcond11111.not, label %panic8.i1761, label %bb18.i1762, !dbg !38681

bb16.i1753:                                       ; preds = %bb15.i1749
  %419 = getelementptr inbounds nuw float, ptr %_118.0.i1754, i64 %iter.i1711.sroa.7.07623, !dbg !38679
  %_35.i1755 = load float, ptr %419, align 4, !dbg !38679, !noalias !38670, !noundef !12
  %_102.i1756 = fcmp olt float %_35.i1755, %416, !dbg !38682
  br i1 %_102.i1756, label %bb35.i1818, label %bb17.i1757, !dbg !38682

panic7.i1752:                                     ; preds = %bb15.i1749
  store float %_0.i32577717, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i1711.sroa.7.07623, i64 noundef %_118.1.i1750, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2c461872bb652d4796cdcf89c28c82c8) #30, !dbg !38679, !noalias !38670
  unreachable, !dbg !38679

bb35.i1818:                                       ; preds = %bb16.i1753
  br label %bb17.i1757, !dbg !38684

bb18.i1762:                                       ; preds = %bb17.i1757
  %420 = getelementptr inbounds nuw float, ptr %_118.0.i1754, i64 %iter.i1711.sroa.7.07623, !dbg !38681
  store float %newest.sroa.0.0.i1758, ptr %420, align 4, !dbg !38681, !noalias !38670
  %_42.i1764 = add nuw nsw i64 %position.i1748, 1, !dbg !38685
  %complete.i1765 = icmp eq i64 %_42.i1764, %window.i1729, !dbg !38685
  br i1 %complete.i1765, label %bb22.i1787, label %bb20.i1766, !dbg !38686

panic8.i1761:                                     ; preds = %bb17.i1757
  store float %_0.i32577717, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i1750, i64 noundef %_118.1.i1750, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2b690e2c7763f11809942906fc2ca813) #30, !dbg !38681, !noalias !38670
  unreachable, !dbg !38681

bb20.i1766:                                       ; preds = %bb18.i1762
  %_44.i1768 = add i64 %iter.i1711.sroa.7.07623, %_45.i1767, !dbg !38687
  %_47.i1770 = icmp ult i64 %_44.i1768, %_114.1.i1737, !dbg !38688
  br i1 %_47.i1770, label %bb30.i1780, label %panic9.i1771, !dbg !38688

panic9.i1771:                                     ; preds = %bb20.i1766
  store float %_0.i32577717, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i1768, i64 noundef %_114.1.i1737, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9fa421ae81817f58fcfc4a3243223891) #30, !dbg !38688, !noalias !38670
  unreachable, !dbg !38688

bb30.i1780:                                       ; preds = %bb20.i1766
  %421 = getelementptr inbounds nuw float, ptr %_114.0.i1741, i64 %_44.i1768, !dbg !38688
  %_43.i1774 = load float, ptr %421, align 4, !dbg !38688, !noalias !38670, !noundef !12
  %_103.i1775 = fcmp olt float %_43.i1774, %newest.sroa.0.0.i1758, !dbg !38689
  %newest.sroa.0.1.i1776 = select i1 %_103.i1775, float %_43.i1774, float %newest.sroa.0.0.i1758, !dbg !38689
  store float %newest.sroa.0.1.i1776, ptr %iter.i1711.sroa.0.0.ptr7625, align 4, !dbg !38691, !noalias !38670
  %422 = trunc i64 %_42.i1764 to i32, !dbg !38692
  br label %bb31.i1782, !dbg !38693

bb31.i1782:                                       ; preds = %bb25.i1815, %bb30.i1780
  %storemerge4773 = phi i32 [ %422, %bb30.i1780 ], [ 0, %bb25.i1815 ], !dbg !38694
  store i32 %storemerge4773, ptr %417, align 4, !dbg !38694, !noalias !38670
  %423 = icmp eq i64 %410, 0, !dbg !38658
  br i1 %423, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1820, label %bb32.i1719, !dbg !38658

bb22.i1787:                                       ; preds = %bb18.i1762
  store float %newest.sroa.0.0.i1758, ptr %iter.i1711.sroa.0.0.ptr7625, align 4, !dbg !38691, !noalias !38670
  %424 = load float, ptr %415, align 4, !dbg !38695, !noalias !38670, !noundef !12
  br label %bb41.i1800, !dbg !38696

bb41.i1800:                                       ; preds = %bb22.i1787, %bb25.i1815
  %iter2.sroa.0.0.i17927621 = phi i64 [ 0, %bb22.i1787 ], [ %_105.i1801, %bb25.i1815 ]
  %suffix.sroa.0.0.i17917620 = phi float [ %424, %bb22.i1787 ], [ %suffix.sroa.0.1.i1811, %bb25.i1815 ]
  %end.sroa.0.1.i17907619 = phi i64 [ %spec.select.i1732, %bb22.i1787 ], [ %427, %bb25.i1815 ]
  %_56.i1802 = mul i64 %end.sroa.0.1.i17907619, %width.i1712, !dbg !38699
  %_55.i1803 = add i64 %_56.i1802, %iter.i1711.sroa.7.07623, !dbg !38699
  %_59.i1805 = icmp ult i64 %_55.i1803, %_114.1.i1737, !dbg !38700
  br i1 %_59.i1805, label %bb25.i1815, label %panic13.i1806, !dbg !38700

panic13.i1806:                                    ; preds = %bb41.i1800
  store float %_0.i32577717, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i1803, i64 noundef %_114.1.i1737, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_91a4c6b9b17ebf4d863f9a70b6dc929a) #30, !dbg !38700, !noalias !38670
  unreachable, !dbg !38700

bb25.i1815:                                       ; preds = %bb41.i1800
  %_105.i1801 = add nuw nsw i64 %iter2.sroa.0.0.i17927621, 1, !dbg !38701
  %425 = getelementptr inbounds nuw float, ptr %_114.0.i1741, i64 %_55.i1803, !dbg !38700
  %_54.i1809 = load float, ptr %425, align 4, !dbg !38700, !noalias !38670, !noundef !12
  %_107.i1810 = fcmp olt float %suffix.sroa.0.0.i17917620, %_54.i1809, !dbg !38704
  %suffix.sroa.0.1.i1811 = select i1 %_107.i1810, float %suffix.sroa.0.0.i17917620, float %_54.i1809, !dbg !38704
  store float %suffix.sroa.0.1.i1811, ptr %425, align 4, !dbg !38706, !noalias !38670
  %426 = icmp eq i64 %end.sroa.0.1.i17907619, 0, !dbg !38707
  %spec.store.select.i1817 = select i1 %426, i64 %_86.i, i64 %end.sroa.0.1.i17907619, !dbg !38707
  %427 = add i64 %spec.store.select.i1817, -1, !dbg !38708
  %exitcond11108.not = icmp eq i64 %_105.i1801, %window.i1729, !dbg !38709
  br i1 %exitcond11108.not, label %bb31.i1782, label %bb41.i1800, !dbg !38696

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1820: ; preds = %bb31.i1782, %bb32.i1719, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3182
  %_0.i3025 = load float, ptr %scratch.i, align 4, !dbg !38711, !alias.scope !38713, !noalias !38716, !noundef !12
  %_0.i2655 = fmul float %_0.i3025, 1.638400e+04, !dbg !38717
  %428 = tail call noundef float @llvm.floor.f32(float %_0.i2655), !dbg !38719
  %_0.i2654 = fmul float %428, 0x3F10000000000000, !dbg !38723
  %429 = icmp eq i64 %width.i33.i, 0, !dbg !38725
  %_149.1.i82.i.pre = load i64, ptr %367, align 8, !dbg !38727, !alias.scope !38624, !noalias !38625
  br i1 %429, label %bb16.i77.i, label %bb39.i57.i.lr.ph, !dbg !38725

bb39.i57.i.lr.ph:                                 ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1820
  %_145.1.i60.i = load i64, ptr %62, align 8, !alias.scope !38624, !noalias !38625, !noundef !12
  %_145.0.i64.i = load ptr, ptr %61, align 8, !nonnull !12
  %_147.0.i75.i = load ptr, ptr %368, align 8, !nonnull !12
  %exitcond11112.not = icmp eq i64 %_145.1.i60.i, 0, !dbg !38728
  br i1 %exitcond11112.not, label %panic.i62.i, label %bb17.i63.i, !dbg !38728

bb37.i124.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3030
  store float %_0.i32577717, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i39.i, i64 noundef %_144.1.i38.i, i64 noundef %_144.1.i38.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_56df7c041d29359441bca272bf4e38e3) #30, !dbg !38729, !noalias !38649
  unreachable, !dbg !38729

bb16.i77.i:                                       ; preds = %bb21.i74.i.7, %bb21.i74.i, %bb21.i74.i.1, %bb21.i74.i.2, %bb21.i74.i.3, %bb21.i74.i.4, %bb21.i74.i.5, %bb21.i74.i.6, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1820
  %_0.i3023 = phi float [ %_0.i3025, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1820 ], [ %_49.i76.i, %bb21.i74.i ], [ %_49.i76.i, %bb21.i74.i.7 ], [ %_49.i76.i, %bb21.i74.i.6 ], [ %_49.i76.i, %bb21.i74.i.5 ], [ %_49.i76.i, %bb21.i74.i.4 ], [ %_49.i76.i, %bb21.i74.i.3 ], [ %_49.i76.i, %bb21.i74.i.2 ], [ %_49.i76.i, %bb21.i74.i.1 ], !dbg !38730
  %_0.i2223 = fadd float %_0.i2654, %_0.i28837648, !dbg !38732
  %_0.i2883 = fsub float %_0.i2223, %_0.i3023, !dbg !38734
  %_109.i83.i = icmp ugt i64 %_22.i39.i, %_149.1.i82.i.pre, !dbg !38736
  br i1 %_109.i83.i, label %bb42.i123.i, label %bb43.i84.i, !dbg !38736, !prof !639

bb43.i84.i:                                       ; preds = %bb16.i77.i
  %_149.0.i85.i = load ptr, ptr %368, align 8, !dbg !38727, !alias.scope !38624, !noalias !38625, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38739), !dbg !38742
  %_4.not.i3175 = icmp eq i64 %_149.1.i82.i.pre, %_22.i39.i, !dbg !38743
  br i1 %_4.not.i3175, label %panic.i3177, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3178, !dbg !38743

panic.i3177:                                      ; preds = %bb43.i84.i
  store float %_0.i32577717, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !38743, !noalias !38745
  unreachable, !dbg !38743

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3178: ; preds = %bb43.i84.i
  %_116.i87.i = getelementptr inbounds nuw float, ptr %_149.0.i85.i, i64 %_22.i39.i, !dbg !38746
  store float %_0.i2654, ptr %_116.i87.i, align 4, !dbg !38743, !alias.scope !38739, !noalias !38716
  %_0.i2426 = fdiv float %_0.i2883, %_64.i89.i, !dbg !38748
  %_0.i2882 = fsub float 1.000000e+00, %_0.i2426, !dbg !38750
  %_0.i2881 = fsub float %_0.i2882, %_0.i32577717, !dbg !38752
  %_4.i2442 = fmul float %_9.i31.i, %_0.i2881, !dbg !38754
  %_0.i2443 = fadd float %_0.i32577717, %_4.i2442, !dbg !38754
  %_3.i.i3657.inv = fcmp ogt float %_0.i2882, %_0.i2443, !dbg !38756
  %_4.i.i3664.v = select i1 %_3.i.i3657.inv, float %_0.i2882, float %_0.i2443, !dbg !38756
  %430 = tail call noundef float @llvm.fabs.f32(float %_4.i.i3664.v), !dbg !38759
  %431 = fcmp uge float %430, 0x3BC79CA100000000, !dbg !38762
  %_0.i3257 = select i1 %431, float %_4.i.i3664.v, float 0.000000e+00, !dbg !38764
  %_0.i2880 = fsub float 1.000000e+00, %_0.i3257, !dbg !38765
  %_150.1.i101.i = load i64, ptr %372, align 8, !dbg !38767, !alias.scope !38624, !noalias !38625, !noundef !12
  %_76.i102.i = mul i64 %width.i33.i, %main_cursor.sroa.0.1.i4567644, !dbg !38768
  %_120.i103.i = icmp ugt i64 %_76.i102.i, %_150.1.i101.i, !dbg !38769
  br i1 %_120.i103.i, label %bb48.i122.i, label %bb49.i104.i, !dbg !38769, !prof !639

bb42.i123.i:                                      ; preds = %bb16.i77.i
  store float %_0.i32577717, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i39.i, i64 noundef %_149.1.i82.i.pre, i64 noundef %_149.1.i82.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_90498045d73339daaf9e4f537508f58b) #30, !dbg !38772, !noalias !38716
  unreachable, !dbg !38772

bb49.i104.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3178
  %_150.0.i105.i = load ptr, ptr %373, align 8, !dbg !38767, !alias.scope !38624, !noalias !38625, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38773), !dbg !38776
  %_3.not.i3017 = icmp eq i64 %_150.1.i101.i, %_76.i102.i, !dbg !38777
  br i1 %_3.not.i3017, label %panic.i3020, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3170, !dbg !38777

panic.i3020:                                      ; preds = %bb49.i104.i
  store float %_0.i3257, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !38777, !noalias !38779
  unreachable, !dbg !38777

bb48.i122.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3178
  store float %_0.i3257, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i102.i, i64 noundef %_150.1.i101.i, i64 noundef %_150.1.i101.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_da8af254b6d507a8e2ca31e544bfd21d) #30, !dbg !38780, !noalias !38716
  unreachable, !dbg !38780

bb17.i63.i:                                       ; preds = %bb39.i57.i.lr.ph
  %432 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i, i64 8, !dbg !38728
  %_44.i65.i = load i32, ptr %432, align 4, !dbg !38728, !noalias !38716, !noundef !12
  %_43.i66.i = zext i32 %_44.i65.i to i64, !dbg !38728
  %433 = add i64 %ring_cursor.sroa.0.1.i4557643, %_43.i66.i, !dbg !38781
  %_47.not.i67.i = icmp ult i64 %433, %_86.i, !dbg !38782
  %434 = select i1 %_47.not.i67.i, i64 0, i64 %_86.i, !dbg !38782
  %spec.select.i68.i = sub nuw i64 %433, %434, !dbg !38782
  %_51.i69.i = mul i64 %spec.select.i68.i, %width.i33.i, !dbg !38783
  %_53.i72.i = icmp ult i64 %_51.i69.i, %_149.1.i82.i.pre, !dbg !38784
  br i1 %_53.i72.i, label %bb21.i74.i, label %panic1.i73.i, !dbg !38784

panic.i62.i:                                      ; preds = %bb39.i57.i.7, %bb39.i57.i.6, %bb39.i57.i.5, %bb39.i57.i.4, %bb39.i57.i.3, %bb39.i57.i.2, %bb39.i57.i.1, %bb39.i57.i.lr.ph
  %_145.1.i60.i.lcssa.ph = phi i64 [ 7, %bb39.i57.i.7 ], [ 6, %bb39.i57.i.6 ], [ 5, %bb39.i57.i.5 ], [ 4, %bb39.i57.i.4 ], [ 3, %bb39.i57.i.3 ], [ 2, %bb39.i57.i.2 ], [ 1, %bb39.i57.i.1 ], [ 0, %bb39.i57.i.lr.ph ]
  store float %_0.i32577717, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i60.i.lcssa.ph, i64 noundef %_145.1.i60.i.lcssa.ph, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a6feb40b34112df5f214f84424dd2c7c) #30, !dbg !38728, !noalias !38716
  unreachable, !dbg !38728

bb21.i74.i:                                       ; preds = %bb17.i63.i
  %435 = getelementptr inbounds nuw float, ptr %_147.0.i75.i, i64 %_51.i69.i, !dbg !38784
  %_49.i76.i = load float, ptr %435, align 4, !dbg !38784, !noalias !38716, !noundef !12
  store float %_49.i76.i, ptr %scratch.i, align 4, !dbg !38785, !noalias !38716
  %436 = icmp eq i64 %width.i33.i, 1, !dbg !38725
  br i1 %436, label %bb16.i77.i, label %bb39.i57.i.1, !dbg !38725

bb39.i57.i.1:                                     ; preds = %bb21.i74.i
  %exitcond11112.1.not = icmp eq i64 %_145.1.i60.i, 1, !dbg !38728
  br i1 %exitcond11112.1.not, label %panic.i62.i, label %bb17.i63.i.1, !dbg !38728

bb17.i63.i.1:                                     ; preds = %bb39.i57.i.1
  %437 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i, i64 20, !dbg !38728
  %_44.i65.i.1 = load i32, ptr %437, align 4, !dbg !38728, !noalias !38716, !noundef !12
  %_43.i66.i.1 = zext i32 %_44.i65.i.1 to i64, !dbg !38728
  %438 = add i64 %ring_cursor.sroa.0.1.i4557643, %_43.i66.i.1, !dbg !38781
  %_47.not.i67.i.1 = icmp ult i64 %438, %_86.i, !dbg !38782
  %439 = select i1 %_47.not.i67.i.1, i64 0, i64 %_86.i, !dbg !38782
  %spec.select.i68.i.1 = sub nuw i64 %438, %439, !dbg !38782
  %_51.i69.i.1 = mul i64 %spec.select.i68.i.1, %width.i33.i, !dbg !38783
  %_50.i70.i.1 = add i64 %_51.i69.i.1, 1, !dbg !38783
  %_53.i72.i.1 = icmp ult i64 %_50.i70.i.1, %_149.1.i82.i.pre, !dbg !38784
  br i1 %_53.i72.i.1, label %bb21.i74.i.1, label %panic1.i73.i, !dbg !38784

bb21.i74.i.1:                                     ; preds = %bb17.i63.i.1
  %440 = getelementptr inbounds nuw float, ptr %_147.0.i75.i, i64 %_50.i70.i.1, !dbg !38784
  %_49.i76.i.1 = load float, ptr %440, align 4, !dbg !38784, !noalias !38716, !noundef !12
  store float %_49.i76.i.1, ptr %iter.i32.i.sroa.0.0.ptr7629.1, align 4, !dbg !38785, !noalias !38716
  %441 = icmp eq i64 %width.i33.i, 2, !dbg !38725
  br i1 %441, label %bb16.i77.i, label %bb39.i57.i.2, !dbg !38725

bb39.i57.i.2:                                     ; preds = %bb21.i74.i.1
  %exitcond11112.2.not = icmp eq i64 %_145.1.i60.i, 2, !dbg !38728
  br i1 %exitcond11112.2.not, label %panic.i62.i, label %bb17.i63.i.2, !dbg !38728

bb17.i63.i.2:                                     ; preds = %bb39.i57.i.2
  %442 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i, i64 32, !dbg !38728
  %_44.i65.i.2 = load i32, ptr %442, align 4, !dbg !38728, !noalias !38716, !noundef !12
  %_43.i66.i.2 = zext i32 %_44.i65.i.2 to i64, !dbg !38728
  %443 = add i64 %ring_cursor.sroa.0.1.i4557643, %_43.i66.i.2, !dbg !38781
  %_47.not.i67.i.2 = icmp ult i64 %443, %_86.i, !dbg !38782
  %444 = select i1 %_47.not.i67.i.2, i64 0, i64 %_86.i, !dbg !38782
  %spec.select.i68.i.2 = sub nuw i64 %443, %444, !dbg !38782
  %_51.i69.i.2 = mul i64 %spec.select.i68.i.2, %width.i33.i, !dbg !38783
  %_50.i70.i.2 = add i64 %_51.i69.i.2, 2, !dbg !38783
  %_53.i72.i.2 = icmp ult i64 %_50.i70.i.2, %_149.1.i82.i.pre, !dbg !38784
  br i1 %_53.i72.i.2, label %bb21.i74.i.2, label %panic1.i73.i, !dbg !38784

bb21.i74.i.2:                                     ; preds = %bb17.i63.i.2
  %445 = getelementptr inbounds nuw float, ptr %_147.0.i75.i, i64 %_50.i70.i.2, !dbg !38784
  %_49.i76.i.2 = load float, ptr %445, align 4, !dbg !38784, !noalias !38716, !noundef !12
  store float %_49.i76.i.2, ptr %iter.i32.i.sroa.0.0.ptr7629.2, align 4, !dbg !38785, !noalias !38716
  %446 = icmp eq i64 %width.i33.i, 3, !dbg !38725
  br i1 %446, label %bb16.i77.i, label %bb39.i57.i.3, !dbg !38725

bb39.i57.i.3:                                     ; preds = %bb21.i74.i.2
  %exitcond11112.3.not = icmp eq i64 %_145.1.i60.i, 3, !dbg !38728
  br i1 %exitcond11112.3.not, label %panic.i62.i, label %bb17.i63.i.3, !dbg !38728

bb17.i63.i.3:                                     ; preds = %bb39.i57.i.3
  %447 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i, i64 44, !dbg !38728
  %_44.i65.i.3 = load i32, ptr %447, align 4, !dbg !38728, !noalias !38716, !noundef !12
  %_43.i66.i.3 = zext i32 %_44.i65.i.3 to i64, !dbg !38728
  %448 = add i64 %ring_cursor.sroa.0.1.i4557643, %_43.i66.i.3, !dbg !38781
  %_47.not.i67.i.3 = icmp ult i64 %448, %_86.i, !dbg !38782
  %449 = select i1 %_47.not.i67.i.3, i64 0, i64 %_86.i, !dbg !38782
  %spec.select.i68.i.3 = sub nuw i64 %448, %449, !dbg !38782
  %_51.i69.i.3 = mul i64 %spec.select.i68.i.3, %width.i33.i, !dbg !38783
  %_50.i70.i.3 = add i64 %_51.i69.i.3, 3, !dbg !38783
  %_53.i72.i.3 = icmp ult i64 %_50.i70.i.3, %_149.1.i82.i.pre, !dbg !38784
  br i1 %_53.i72.i.3, label %bb21.i74.i.3, label %panic1.i73.i, !dbg !38784

bb21.i74.i.3:                                     ; preds = %bb17.i63.i.3
  %450 = getelementptr inbounds nuw float, ptr %_147.0.i75.i, i64 %_50.i70.i.3, !dbg !38784
  %_49.i76.i.3 = load float, ptr %450, align 4, !dbg !38784, !noalias !38716, !noundef !12
  store float %_49.i76.i.3, ptr %iter.i32.i.sroa.0.0.ptr7629.3, align 4, !dbg !38785, !noalias !38716
  %451 = icmp eq i64 %width.i33.i, 4, !dbg !38725
  br i1 %451, label %bb16.i77.i, label %bb39.i57.i.4, !dbg !38725

bb39.i57.i.4:                                     ; preds = %bb21.i74.i.3
  %exitcond11112.4.not = icmp eq i64 %_145.1.i60.i, 4, !dbg !38728
  br i1 %exitcond11112.4.not, label %panic.i62.i, label %bb17.i63.i.4, !dbg !38728

bb17.i63.i.4:                                     ; preds = %bb39.i57.i.4
  %452 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i, i64 56, !dbg !38728
  %_44.i65.i.4 = load i32, ptr %452, align 4, !dbg !38728, !noalias !38716, !noundef !12
  %_43.i66.i.4 = zext i32 %_44.i65.i.4 to i64, !dbg !38728
  %453 = add i64 %ring_cursor.sroa.0.1.i4557643, %_43.i66.i.4, !dbg !38781
  %_47.not.i67.i.4 = icmp ult i64 %453, %_86.i, !dbg !38782
  %454 = select i1 %_47.not.i67.i.4, i64 0, i64 %_86.i, !dbg !38782
  %spec.select.i68.i.4 = sub nuw i64 %453, %454, !dbg !38782
  %_51.i69.i.4 = mul i64 %spec.select.i68.i.4, %width.i33.i, !dbg !38783
  %_50.i70.i.4 = add i64 %_51.i69.i.4, 4, !dbg !38783
  %_53.i72.i.4 = icmp ult i64 %_50.i70.i.4, %_149.1.i82.i.pre, !dbg !38784
  br i1 %_53.i72.i.4, label %bb21.i74.i.4, label %panic1.i73.i, !dbg !38784

bb21.i74.i.4:                                     ; preds = %bb17.i63.i.4
  %455 = getelementptr inbounds nuw float, ptr %_147.0.i75.i, i64 %_50.i70.i.4, !dbg !38784
  %_49.i76.i.4 = load float, ptr %455, align 4, !dbg !38784, !noalias !38716, !noundef !12
  store float %_49.i76.i.4, ptr %iter.i32.i.sroa.0.0.ptr7629.4, align 4, !dbg !38785, !noalias !38716
  %456 = icmp eq i64 %width.i33.i, 5, !dbg !38725
  br i1 %456, label %bb16.i77.i, label %bb39.i57.i.5, !dbg !38725

bb39.i57.i.5:                                     ; preds = %bb21.i74.i.4
  %exitcond11112.5.not = icmp eq i64 %_145.1.i60.i, 5, !dbg !38728
  br i1 %exitcond11112.5.not, label %panic.i62.i, label %bb17.i63.i.5, !dbg !38728

bb17.i63.i.5:                                     ; preds = %bb39.i57.i.5
  %457 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i, i64 68, !dbg !38728
  %_44.i65.i.5 = load i32, ptr %457, align 4, !dbg !38728, !noalias !38716, !noundef !12
  %_43.i66.i.5 = zext i32 %_44.i65.i.5 to i64, !dbg !38728
  %458 = add i64 %ring_cursor.sroa.0.1.i4557643, %_43.i66.i.5, !dbg !38781
  %_47.not.i67.i.5 = icmp ult i64 %458, %_86.i, !dbg !38782
  %459 = select i1 %_47.not.i67.i.5, i64 0, i64 %_86.i, !dbg !38782
  %spec.select.i68.i.5 = sub nuw i64 %458, %459, !dbg !38782
  %_51.i69.i.5 = mul i64 %spec.select.i68.i.5, %width.i33.i, !dbg !38783
  %_50.i70.i.5 = add i64 %_51.i69.i.5, 5, !dbg !38783
  %_53.i72.i.5 = icmp ult i64 %_50.i70.i.5, %_149.1.i82.i.pre, !dbg !38784
  br i1 %_53.i72.i.5, label %bb21.i74.i.5, label %panic1.i73.i, !dbg !38784

bb21.i74.i.5:                                     ; preds = %bb17.i63.i.5
  %460 = getelementptr inbounds nuw float, ptr %_147.0.i75.i, i64 %_50.i70.i.5, !dbg !38784
  %_49.i76.i.5 = load float, ptr %460, align 4, !dbg !38784, !noalias !38716, !noundef !12
  store float %_49.i76.i.5, ptr %iter.i32.i.sroa.0.0.ptr7629.5, align 4, !dbg !38785, !noalias !38716
  %461 = icmp eq i64 %width.i33.i, 6, !dbg !38725
  br i1 %461, label %bb16.i77.i, label %bb39.i57.i.6, !dbg !38725

bb39.i57.i.6:                                     ; preds = %bb21.i74.i.5
  %exitcond11112.6.not = icmp eq i64 %_145.1.i60.i, 6, !dbg !38728
  br i1 %exitcond11112.6.not, label %panic.i62.i, label %bb17.i63.i.6, !dbg !38728

bb17.i63.i.6:                                     ; preds = %bb39.i57.i.6
  %462 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i, i64 80, !dbg !38728
  %_44.i65.i.6 = load i32, ptr %462, align 4, !dbg !38728, !noalias !38716, !noundef !12
  %_43.i66.i.6 = zext i32 %_44.i65.i.6 to i64, !dbg !38728
  %463 = add i64 %ring_cursor.sroa.0.1.i4557643, %_43.i66.i.6, !dbg !38781
  %_47.not.i67.i.6 = icmp ult i64 %463, %_86.i, !dbg !38782
  %464 = select i1 %_47.not.i67.i.6, i64 0, i64 %_86.i, !dbg !38782
  %spec.select.i68.i.6 = sub nuw i64 %463, %464, !dbg !38782
  %_51.i69.i.6 = mul i64 %spec.select.i68.i.6, %width.i33.i, !dbg !38783
  %_50.i70.i.6 = add i64 %_51.i69.i.6, 6, !dbg !38783
  %_53.i72.i.6 = icmp ult i64 %_50.i70.i.6, %_149.1.i82.i.pre, !dbg !38784
  br i1 %_53.i72.i.6, label %bb21.i74.i.6, label %panic1.i73.i, !dbg !38784

bb21.i74.i.6:                                     ; preds = %bb17.i63.i.6
  %465 = getelementptr inbounds nuw float, ptr %_147.0.i75.i, i64 %_50.i70.i.6, !dbg !38784
  %_49.i76.i.6 = load float, ptr %465, align 4, !dbg !38784, !noalias !38716, !noundef !12
  store float %_49.i76.i.6, ptr %iter.i32.i.sroa.0.0.ptr7629.6, align 4, !dbg !38785, !noalias !38716
  %466 = icmp eq i64 %width.i33.i, 7, !dbg !38725
  br i1 %466, label %bb16.i77.i, label %bb39.i57.i.7, !dbg !38725

bb39.i57.i.7:                                     ; preds = %bb21.i74.i.6
  %exitcond11112.7.not = icmp eq i64 %_145.1.i60.i, 7, !dbg !38728
  br i1 %exitcond11112.7.not, label %panic.i62.i, label %bb17.i63.i.7, !dbg !38728

bb17.i63.i.7:                                     ; preds = %bb39.i57.i.7
  %467 = getelementptr inbounds nuw i8, ptr %_145.0.i64.i, i64 92, !dbg !38728
  %_44.i65.i.7 = load i32, ptr %467, align 4, !dbg !38728, !noalias !38716, !noundef !12
  %_43.i66.i.7 = zext i32 %_44.i65.i.7 to i64, !dbg !38728
  %468 = add i64 %ring_cursor.sroa.0.1.i4557643, %_43.i66.i.7, !dbg !38781
  %_47.not.i67.i.7 = icmp ult i64 %468, %_86.i, !dbg !38782
  %469 = select i1 %_47.not.i67.i.7, i64 0, i64 %_86.i, !dbg !38782
  %spec.select.i68.i.7 = sub nuw i64 %468, %469, !dbg !38782
  %_51.i69.i.7 = mul i64 %spec.select.i68.i.7, %width.i33.i, !dbg !38783
  %_50.i70.i.7 = add i64 %_51.i69.i.7, 7, !dbg !38783
  %_53.i72.i.7 = icmp ult i64 %_50.i70.i.7, %_149.1.i82.i.pre, !dbg !38784
  br i1 %_53.i72.i.7, label %bb21.i74.i.7, label %panic1.i73.i, !dbg !38784

bb21.i74.i.7:                                     ; preds = %bb17.i63.i.7
  %470 = getelementptr inbounds nuw float, ptr %_147.0.i75.i, i64 %_50.i70.i.7, !dbg !38784
  %_49.i76.i.7 = load float, ptr %470, align 4, !dbg !38784, !noalias !38716, !noundef !12
  store float %_49.i76.i.7, ptr %iter.i32.i.sroa.0.0.ptr7629.7, align 4, !dbg !38785, !noalias !38716
  br label %bb16.i77.i, !dbg !38725

panic1.i73.i:                                     ; preds = %bb17.i63.i.7, %bb17.i63.i.6, %bb17.i63.i.5, %bb17.i63.i.4, %bb17.i63.i.3, %bb17.i63.i.2, %bb17.i63.i.1, %bb17.i63.i
  %_50.i70.i.lcssa.ph = phi i64 [ %_50.i70.i.7, %bb17.i63.i.7 ], [ %_50.i70.i.6, %bb17.i63.i.6 ], [ %_50.i70.i.5, %bb17.i63.i.5 ], [ %_50.i70.i.4, %bb17.i63.i.4 ], [ %_50.i70.i.3, %bb17.i63.i.3 ], [ %_50.i70.i.2, %bb17.i63.i.2 ], [ %_50.i70.i.1, %bb17.i63.i.1 ], [ %_51.i69.i, %bb17.i63.i ]
  store float %_0.i32577717, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i70.i.lcssa.ph, i64 noundef %_149.1.i82.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b305c1483509cfb31fdec21ff8752674) #30, !dbg !38784, !noalias !38716
  unreachable, !dbg !38784

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3170: ; preds = %bb49.i104.i
  %_127.i107.i = getelementptr inbounds nuw float, ptr %_150.0.i105.i, i64 %_76.i102.i, !dbg !38786
  %_0.i3019 = load float, ptr %_127.i107.i, align 4, !dbg !38777, !alias.scope !38773, !noalias !38716, !noundef !12
  store float %_0.i3028, ptr %_127.i107.i, align 4, !dbg !38788, !alias.scope !38790, !noalias !38716
  %_0.i2653 = fmul float %_0.i2880, %_0.i3019, !dbg !38793
  %_6.i3428 = bitcast float %_0.i3019 to i32, !dbg !38795
  %_5.i3429 = and i32 %_6.i3428, %all.sroa.0.0.i439, !dbg !38798
  %_8.i3430 = bitcast float %_0.i2653 to i32, !dbg !38799
  %_7.i3432 = and i32 %_9.i3431, %_8.i3430, !dbg !38801
  %_4.i3433 = or disjoint i32 %_7.i3432, %_5.i3429, !dbg !38798
  store i32 %_4.i3433, ptr %_141.i, align 4, !dbg !38802, !alias.scope !38804, !noalias !38807
  %_142.i = icmp ugt i64 %_59.i, %right_io.1, !dbg !38808
  br i1 %_142.i, label %bb46.i, label %bb47.i, !dbg !38808, !prof !639

bb44.i:                                           ; preds = %bb43.i
  store float %_0.i32577717, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_59.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_0a0a1af21ea56de7dc8d858c82432bb4) #30, !dbg !38812, !noalias !38568
  unreachable, !dbg !38812

bb47.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3170
  %_149.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %_59.i, !dbg !38813
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38818), !dbg !38821
  %_3.not.i3012 = icmp eq i64 %right_io.1, %_59.i, !dbg !38822
  br i1 %_3.not.i3012, label %panic.i3015, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3016, !dbg !38822

panic.i3015:                                      ; preds = %bb47.i
  store float %_0.i3257, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !38822, !noalias !38824
  unreachable, !dbg !38822

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3016: ; preds = %bb47.i
  %_0.i3014 = load float, ptr %_149.i, align 4, !dbg !38822, !alias.scope !38818, !noalias !38568, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38825), !dbg !38828
  %width.i.i = load i64, ptr %374, align 8, !dbg !38829, !alias.scope !38830, !noalias !38831, !noundef !12
  %_3.i1992 = fcmp uge float %_8.i.i459, %_0.i3447, !dbg !38835
  %_0.i2425 = fdiv float %_8.i.i459, %_0.i3447, !dbg !38837
  %_0.i3427 = select i1 %_3.i1992, float 1.000000e+00, float %_0.i2425, !dbg !38839
  %_144.1.i.i = load i64, ptr %375, align 8, !dbg !38841, !alias.scope !38830, !noalias !38831, !noundef !12
  %_22.i.i468 = mul i64 %width.i.i, %ring_cursor.sroa.0.1.i4557643, !dbg !38842
  %_92.i.i = icmp ugt i64 %_22.i.i468, %_144.1.i.i, !dbg !38843
  br i1 %_92.i.i, label %bb37.i.i, label %bb38.i.i, !dbg !38843, !prof !639

bb38.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3016
  %_144.0.i.i = load ptr, ptr %376, align 8, !dbg !38841, !alias.scope !38830, !noalias !38831, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38846), !dbg !38849
  %_4.not.i3163 = icmp eq i64 %_144.1.i.i, %_22.i.i468, !dbg !38850
  br i1 %_4.not.i3163, label %panic.i3165, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3166, !dbg !38850

panic.i3165:                                      ; preds = %bb38.i.i
  store float %_0.i3257, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !38850, !noalias !38852
  unreachable, !dbg !38850

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3166: ; preds = %bb38.i.i
  %_99.i.i = getelementptr inbounds nuw float, ptr %_144.0.i.i, i64 %_22.i.i468, !dbg !38853
  store float %_0.i3427, ptr %_99.i.i, align 4, !dbg !38850, !alias.scope !38846, !noalias !38855
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38856), !dbg !38859
  %width.i1602 = load i64, ptr %374, align 8, !dbg !38860, !alias.scope !38856, !noalias !38862, !noundef !12
  %471 = icmp eq i64 %width.i1602, 0, !dbg !38864
  br i1 %471, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1710, label %bb32.i1609.lr.ph, !dbg !38864

bb32.i1609.lr.ph:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3166
  %_112.1.i1612 = load i64, ptr %377, align 8, !alias.scope !38856, !noalias !38862, !noundef !12
  %_112.0.i1616 = load ptr, ptr %378, align 8, !nonnull !12
  %472 = add i64 %ring_cursor.sroa.0.1.i4557643, 1
  %_23.not.i1623 = icmp ult i64 %472, %_86.i
  %473 = select i1 %_23.not.i1623, i64 0, i64 %_86.i
  %start1.sroa.0.0.i1624 = sub nuw i64 %472, %473
  %_114.1.i1627 = load i64, ptr %375, align 8
  %_114.0.i1631 = load ptr, ptr %376, align 8, !nonnull !12
  %_116.1.i1632 = load i64, ptr %379, align 8
  %_116.0.i1636 = load ptr, ptr %380, align 8, !nonnull !12
  %_118.1.i1640 = load i64, ptr %381, align 8
  %_118.0.i1644 = load ptr, ptr %382, align 8, !nonnull !12
  %_45.i1657 = mul i64 %width.i1602, %start1.sroa.0.0.i1624
  br label %bb32.i1609, !dbg !38864

bb32.i1609:                                       ; preds = %bb32.i1609.lr.ph, %bb31.i1672
  %iter.i1601.sroa.10.07635 = phi i64 [ %width.i1602, %bb32.i1609.lr.ph ], [ %474, %bb31.i1672 ]
  %iter.i1601.sroa.7.07634 = phi i64 [ 0, %bb32.i1609.lr.ph ], [ %_9.0.i4148, %bb31.i1672 ]
  %iter.i1601.sroa.0.0.idx7633 = phi i64 [ 0, %bb32.i1609.lr.ph ], [ %iter.i1601.sroa.0.0.add, %bb31.i1672 ]
  %iter.i1601.sroa.0.0.ptr7636 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 %iter.i1601.sroa.0.0.idx7633, !dbg !38866
  %474 = add i64 %iter.i1601.sroa.10.07635, -1, !dbg !38866
  %_7.i.i4144 = icmp eq i64 %iter.i1601.sroa.0.0.idx7633, 32, !dbg !38867
  br i1 %_7.i.i4144, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1710.loopexit, label %bb3.i1611, !dbg !38871

bb3.i1611:                                        ; preds = %bb32.i1609
  %iter.i1601.sroa.0.0.add = add nuw nsw i64 %iter.i1601.sroa.0.0.idx7633, 4, !dbg !38872
  %_9.0.i4148 = add nuw nsw i64 %iter.i1601.sroa.7.07634, 1, !dbg !38874
  %exitcond11115.not = icmp eq i64 %iter.i1601.sroa.7.07634, %_112.1.i1612, !dbg !38875
  br i1 %exitcond11115.not, label %panic.i1614, label %bb5.i1615, !dbg !38875

bb5.i1615:                                        ; preds = %bb3.i1611
  %475 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i1616, i64 %iter.i1601.sroa.7.07634, !dbg !38875
  %shape.i1617 = load i32, ptr %475, align 4, !dbg !38875, !noalias !38876, !noundef !12
  %476 = getelementptr inbounds nuw i8, ptr %475, i64 4, !dbg !38875
  %shape3.i1618 = load i32, ptr %476, align 4, !dbg !38875, !noalias !38876, !noundef !12
  %window.i1619 = zext i32 %shape.i1617 to i64, !dbg !38877
  %_19.i1620 = zext i32 %shape3.i1618 to i64, !dbg !38878
  %477 = add i64 %ring_cursor.sroa.0.1.i4557643, %_19.i1620, !dbg !38879
  %_20.not.i1621 = icmp ult i64 %477, %_86.i, !dbg !38880
  %478 = select i1 %_20.not.i1621, i64 0, i64 %_86.i, !dbg !38880
  %spec.select.i1622 = sub nuw i64 %477, %478, !dbg !38880
  %_27.i1625 = mul i64 %spec.select.i1622, %width.i1602, !dbg !38881
  %_26.i1626 = add i64 %_27.i1625, %iter.i1601.sroa.7.07634, !dbg !38881
  %_30.i1628 = icmp ult i64 %_26.i1626, %_114.1.i1627, !dbg !38882
  br i1 %_30.i1628, label %bb12.i1630, label %panic5.i1629, !dbg !38882

panic.i1614:                                      ; preds = %bb3.i1611
  store float %_0.i3257, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i1612, i64 noundef %_112.1.i1612, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4a8785a681d008a9bfd0cd82628ea9cb) #30, !dbg !38875, !noalias !38876
  unreachable, !dbg !38875

bb12.i1630:                                       ; preds = %bb5.i1615
  %479 = getelementptr inbounds nuw float, ptr %_114.0.i1631, i64 %_26.i1626, !dbg !38882
  %480 = load float, ptr %479, align 4, !dbg !38882, !noalias !38876, !noundef !12
  %exitcond11116.not = icmp eq i64 %iter.i1601.sroa.7.07634, %_116.1.i1632, !dbg !38883
  br i1 %exitcond11116.not, label %panic6.i1634, label %bb13.i1635, !dbg !38883

panic5.i1629:                                     ; preds = %bb5.i1615
  store float %_0.i3257, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i1626, i64 noundef %_114.1.i1627, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cbce7773ac40979e4ba2385da3aec116) #30, !dbg !38882, !noalias !38876
  unreachable, !dbg !38882

bb13.i1635:                                       ; preds = %bb12.i1630
  %481 = getelementptr inbounds nuw i32, ptr %_116.0.i1636, i64 %iter.i1601.sroa.7.07634, !dbg !38883
  %_32.i1637 = load i32, ptr %481, align 4, !dbg !38883, !noalias !38876, !noundef !12
  %position.i1638 = zext i32 %_32.i1637 to i64, !dbg !38883
  %482 = icmp eq i32 %_32.i1637, 0, !dbg !38884
  br i1 %482, label %bb17.i1647, label %bb15.i1639, !dbg !38884

panic6.i1634:                                     ; preds = %bb12.i1630
  store float %_0.i3257, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i1632, i64 noundef %_116.1.i1632, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ec0d48f73ebfc2755df5cedaa60b5c0a) #30, !dbg !38883, !noalias !38876
  unreachable, !dbg !38883

bb15.i1639:                                       ; preds = %bb13.i1635
  %_37.i1641 = icmp ult i64 %iter.i1601.sroa.7.07634, %_118.1.i1640, !dbg !38885
  br i1 %_37.i1641, label %bb16.i1643, label %panic7.i1642, !dbg !38885

bb17.i1647:                                       ; preds = %bb35.i1708, %bb16.i1643, %bb13.i1635
  %newest.sroa.0.0.i1648 = phi float [ %480, %bb13.i1635 ], [ %_35.i1645, %bb35.i1708 ], [ %480, %bb16.i1643 ], !dbg !38886
  %exitcond11117.not = icmp eq i64 %iter.i1601.sroa.7.07634, %_118.1.i1640, !dbg !38887
  br i1 %exitcond11117.not, label %panic8.i1651, label %bb18.i1652, !dbg !38887

bb16.i1643:                                       ; preds = %bb15.i1639
  %483 = getelementptr inbounds nuw float, ptr %_118.0.i1644, i64 %iter.i1601.sroa.7.07634, !dbg !38885
  %_35.i1645 = load float, ptr %483, align 4, !dbg !38885, !noalias !38876, !noundef !12
  %_102.i1646 = fcmp olt float %_35.i1645, %480, !dbg !38888
  br i1 %_102.i1646, label %bb35.i1708, label %bb17.i1647, !dbg !38888

panic7.i1642:                                     ; preds = %bb15.i1639
  store float %_0.i3257, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i1601.sroa.7.07634, i64 noundef %_118.1.i1640, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2c461872bb652d4796cdcf89c28c82c8) #30, !dbg !38885, !noalias !38876
  unreachable, !dbg !38885

bb35.i1708:                                       ; preds = %bb16.i1643
  br label %bb17.i1647, !dbg !38890

bb18.i1652:                                       ; preds = %bb17.i1647
  %484 = getelementptr inbounds nuw float, ptr %_118.0.i1644, i64 %iter.i1601.sroa.7.07634, !dbg !38887
  store float %newest.sroa.0.0.i1648, ptr %484, align 4, !dbg !38887, !noalias !38876
  %_42.i1654 = add nuw nsw i64 %position.i1638, 1, !dbg !38891
  %complete.i1655 = icmp eq i64 %_42.i1654, %window.i1619, !dbg !38891
  br i1 %complete.i1655, label %bb22.i1677, label %bb20.i1656, !dbg !38892

panic8.i1651:                                     ; preds = %bb17.i1647
  store float %_0.i3257, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i1640, i64 noundef %_118.1.i1640, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2b690e2c7763f11809942906fc2ca813) #30, !dbg !38887, !noalias !38876
  unreachable, !dbg !38887

bb20.i1656:                                       ; preds = %bb18.i1652
  %_44.i1658 = add i64 %iter.i1601.sroa.7.07634, %_45.i1657, !dbg !38893
  %_47.i1660 = icmp ult i64 %_44.i1658, %_114.1.i1627, !dbg !38894
  br i1 %_47.i1660, label %bb30.i1670, label %panic9.i1661, !dbg !38894

panic9.i1661:                                     ; preds = %bb20.i1656
  store float %_0.i3257, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i1658, i64 noundef %_114.1.i1627, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9fa421ae81817f58fcfc4a3243223891) #30, !dbg !38894, !noalias !38876
  unreachable, !dbg !38894

bb30.i1670:                                       ; preds = %bb20.i1656
  %485 = getelementptr inbounds nuw float, ptr %_114.0.i1631, i64 %_44.i1658, !dbg !38894
  %_43.i1664 = load float, ptr %485, align 4, !dbg !38894, !noalias !38876, !noundef !12
  %_103.i1665 = fcmp olt float %_43.i1664, %newest.sroa.0.0.i1648, !dbg !38895
  %newest.sroa.0.1.i1666 = select i1 %_103.i1665, float %_43.i1664, float %newest.sroa.0.0.i1648, !dbg !38895
  store float %newest.sroa.0.1.i1666, ptr %iter.i1601.sroa.0.0.ptr7636, align 4, !dbg !38897, !noalias !38876
  %486 = trunc i64 %_42.i1654 to i32, !dbg !38898
  br label %bb31.i1672, !dbg !38899

bb31.i1672:                                       ; preds = %bb25.i1705, %bb30.i1670
  %storemerge4776 = phi i32 [ %486, %bb30.i1670 ], [ 0, %bb25.i1705 ], !dbg !38900
  store i32 %storemerge4776, ptr %481, align 4, !dbg !38900, !noalias !38876
  %487 = icmp eq i64 %474, 0, !dbg !38864
  br i1 %487, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1710.loopexit, label %bb32.i1609, !dbg !38864

bb22.i1677:                                       ; preds = %bb18.i1652
  store float %newest.sroa.0.0.i1648, ptr %iter.i1601.sroa.0.0.ptr7636, align 4, !dbg !38897, !noalias !38876
  %488 = load float, ptr %479, align 4, !dbg !38901, !noalias !38876, !noundef !12
  br label %bb41.i1690, !dbg !38902

bb41.i1690:                                       ; preds = %bb22.i1677, %bb25.i1705
  %iter2.sroa.0.0.i16827632 = phi i64 [ 0, %bb22.i1677 ], [ %_105.i1691, %bb25.i1705 ]
  %suffix.sroa.0.0.i16817631 = phi float [ %488, %bb22.i1677 ], [ %suffix.sroa.0.1.i1701, %bb25.i1705 ]
  %end.sroa.0.1.i16807630 = phi i64 [ %spec.select.i1622, %bb22.i1677 ], [ %491, %bb25.i1705 ]
  %_56.i1692 = mul i64 %end.sroa.0.1.i16807630, %width.i1602, !dbg !38905
  %_55.i1693 = add i64 %_56.i1692, %iter.i1601.sroa.7.07634, !dbg !38905
  %_59.i1695 = icmp ult i64 %_55.i1693, %_114.1.i1627, !dbg !38906
  br i1 %_59.i1695, label %bb25.i1705, label %panic13.i1696, !dbg !38906

panic13.i1696:                                    ; preds = %bb41.i1690
  store float %_0.i3257, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i1693, i64 noundef %_114.1.i1627, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_91a4c6b9b17ebf4d863f9a70b6dc929a) #30, !dbg !38906, !noalias !38876
  unreachable, !dbg !38906

bb25.i1705:                                       ; preds = %bb41.i1690
  %_105.i1691 = add nuw nsw i64 %iter2.sroa.0.0.i16827632, 1, !dbg !38907
  %489 = getelementptr inbounds nuw float, ptr %_114.0.i1631, i64 %_55.i1693, !dbg !38906
  %_54.i1699 = load float, ptr %489, align 4, !dbg !38906, !noalias !38876, !noundef !12
  %_107.i1700 = fcmp olt float %suffix.sroa.0.0.i16817631, %_54.i1699, !dbg !38910
  %suffix.sroa.0.1.i1701 = select i1 %_107.i1700, float %suffix.sroa.0.0.i16817631, float %_54.i1699, !dbg !38910
  store float %suffix.sroa.0.1.i1701, ptr %489, align 4, !dbg !38912, !noalias !38876
  %490 = icmp eq i64 %end.sroa.0.1.i16807630, 0, !dbg !38913
  %spec.store.select.i1707 = select i1 %490, i64 %_86.i, i64 %end.sroa.0.1.i16807630, !dbg !38913
  %491 = add i64 %spec.store.select.i1707, -1, !dbg !38914
  %exitcond11114.not = icmp eq i64 %_105.i1691, %window.i1619, !dbg !38915
  br i1 %exitcond11114.not, label %bb31.i1672, label %bb41.i1690, !dbg !38902

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1710.loopexit: ; preds = %bb32.i1609, %bb31.i1672
  %_0.i3011.pre = load float, ptr %scratch.i, align 4, !dbg !38917, !alias.scope !38919, !noalias !38922
  br label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1710, !dbg !38917

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1710: ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1710.loopexit, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3166
  %_0.i3011 = phi float [ %_0.i3011.pre, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1710.loopexit ], [ %_0.i3023, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3166 ], !dbg !38917
  %_0.i2652 = fmul float %_0.i3011, 1.638400e+04, !dbg !38923
  %492 = tail call noundef float @llvm.floor.f32(float %_0.i2652), !dbg !38925
  %_0.i2651 = fmul float %492, 0x3F10000000000000, !dbg !38929
  %493 = icmp eq i64 %width.i.i, 0, !dbg !38931
  %_149.1.i.i.pre = load i64, ptr %383, align 8, !dbg !38933, !alias.scope !38830, !noalias !38831
  br i1 %493, label %bb16.i.i, label %bb39.i.i.lr.ph, !dbg !38931

bb39.i.i.lr.ph:                                   ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1710
  %_145.1.i.i = load i64, ptr %377, align 8, !alias.scope !38830, !noalias !38831, !noundef !12
  %_145.0.i.i = load ptr, ptr %378, align 8, !nonnull !12
  %_147.0.i.i = load ptr, ptr %384, align 8, !nonnull !12
  %exitcond11118.not = icmp eq i64 %_145.1.i.i, 0, !dbg !38934
  br i1 %exitcond11118.not, label %panic.i.i, label %bb17.i.i, !dbg !38934

bb37.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3016
  store float %_0.i3257, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i468, i64 noundef %_144.1.i.i, i64 noundef %_144.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_56df7c041d29359441bca272bf4e38e3) #30, !dbg !38935, !noalias !38855
  unreachable, !dbg !38935

bb16.i.i:                                         ; preds = %bb21.i.i.7, %bb21.i.i, %bb21.i.i.1, %bb21.i.i.2, %bb21.i.i.3, %bb21.i.i.4, %bb21.i.i.5, %bb21.i.i.6, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1710
  %_0.i3009 = phi float [ %_0.i3011, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1710 ], [ %_49.i.i476, %bb21.i.i ], [ %_49.i.i476, %bb21.i.i.7 ], [ %_49.i.i476, %bb21.i.i.6 ], [ %_49.i.i476, %bb21.i.i.5 ], [ %_49.i.i476, %bb21.i.i.4 ], [ %_49.i.i476, %bb21.i.i.3 ], [ %_49.i.i476, %bb21.i.i.2 ], [ %_49.i.i476, %bb21.i.i.1 ], !dbg !38936
  %_0.i2222 = fadd float %_0.i2651, %_0.i28797719, !dbg !38938
  %_0.i2879 = fsub float %_0.i2222, %_0.i3009, !dbg !38940
  %_109.i.i = icmp ugt i64 %_22.i.i468, %_149.1.i.i.pre, !dbg !38942
  br i1 %_109.i.i, label %bb42.i.i, label %bb43.i.i, !dbg !38942, !prof !639

bb43.i.i:                                         ; preds = %bb16.i.i
  %_149.0.i.i = load ptr, ptr %384, align 8, !dbg !38933, !alias.scope !38830, !noalias !38831, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38945), !dbg !38948
  %_4.not.i3159 = icmp eq i64 %_149.1.i.i.pre, %_22.i.i468, !dbg !38949
  br i1 %_4.not.i3159, label %panic.i3161, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3162, !dbg !38949

panic.i3161:                                      ; preds = %bb43.i.i
  store float %_0.i3257, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !38949, !noalias !38951
  unreachable, !dbg !38949

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3162: ; preds = %bb43.i.i
  %_116.i.i = getelementptr inbounds nuw float, ptr %_149.0.i.i, i64 %_22.i.i468, !dbg !38952
  store float %_0.i2651, ptr %_116.i.i, align 4, !dbg !38949, !alias.scope !38945, !noalias !38922
  %_0.i2424 = fdiv float %_0.i2879, %_64.i.i, !dbg !38954
  %_0.i2878 = fsub float 1.000000e+00, %_0.i2424, !dbg !38956
  %_0.i2877 = fsub float %_0.i2878, %_0.i32537788, !dbg !38958
  %_4.i2440 = fmul float %_9.i.i460, %_0.i2877, !dbg !38960
  %_0.i2441 = fadd float %_0.i32537788, %_4.i2440, !dbg !38960
  %_3.i.i3648.inv = fcmp ogt float %_0.i2878, %_0.i2441, !dbg !38962
  %_4.i.i3655.v = select i1 %_3.i.i3648.inv, float %_0.i2878, float %_0.i2441, !dbg !38962
  %494 = tail call noundef float @llvm.fabs.f32(float %_4.i.i3655.v), !dbg !38965
  %495 = fcmp uge float %494, 0x3BC79CA100000000, !dbg !38968
  %_0.i3253 = select i1 %495, float %_4.i.i3655.v, float 0.000000e+00, !dbg !38970
  %_0.i2876 = fsub float 1.000000e+00, %_0.i3253, !dbg !38971
  %_150.1.i.i = load i64, ptr %388, align 8, !dbg !38973, !alias.scope !38830, !noalias !38831, !noundef !12
  %_76.i.i = mul i64 %width.i.i, %main_cursor.sroa.0.1.i4567644, !dbg !38974
  %_120.i.i = icmp ugt i64 %_76.i.i, %_150.1.i.i, !dbg !38975
  br i1 %_120.i.i, label %bb48.i.i, label %bb49.i.i, !dbg !38975, !prof !639

bb42.i.i:                                         ; preds = %bb16.i.i
  store float %_0.i3257, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i468, i64 noundef %_149.1.i.i.pre, i64 noundef %_149.1.i.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_90498045d73339daaf9e4f537508f58b) #30, !dbg !38978, !noalias !38922
  unreachable, !dbg !38978

bb49.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3162
  %_150.0.i.i = load ptr, ptr %389, align 8, !dbg !38973, !alias.scope !38830, !noalias !38831, !nonnull !12, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38979), !dbg !38982
  %_3.not.i3003 = icmp eq i64 %_150.1.i.i, %_76.i.i, !dbg !38983
  br i1 %_3.not.i3003, label %panic.i3006, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3154, !dbg !38983

panic.i3006:                                      ; preds = %bb49.i.i
  store float %_0.i3257, ptr %371, align 1, !dbg !38032
  store float %_0.i3253, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !38983, !noalias !38985
  unreachable, !dbg !38983

bb48.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3162
  store float %_0.i3257, ptr %371, align 1, !dbg !38032
  store float %_0.i3253, ptr %387, align 1, !dbg !38046
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i.i, i64 noundef %_150.1.i.i, i64 noundef %_150.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_da8af254b6d507a8e2ca31e544bfd21d) #30, !dbg !38986, !noalias !38922
  unreachable, !dbg !38986

bb17.i.i:                                         ; preds = %bb39.i.i.lr.ph
  %496 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 8, !dbg !38934
  %_44.i.i473 = load i32, ptr %496, align 4, !dbg !38934, !noalias !38922, !noundef !12
  %_43.i.i474 = zext i32 %_44.i.i473 to i64, !dbg !38934
  %497 = add i64 %ring_cursor.sroa.0.1.i4557643, %_43.i.i474, !dbg !38987
  %_47.not.i.i = icmp ult i64 %497, %_86.i, !dbg !38988
  %498 = select i1 %_47.not.i.i, i64 0, i64 %_86.i, !dbg !38988
  %spec.select.i.i = sub nuw i64 %497, %498, !dbg !38988
  %_51.i.i = mul i64 %spec.select.i.i, %width.i.i, !dbg !38989
  %_53.i.i475 = icmp ult i64 %_51.i.i, %_149.1.i.i.pre, !dbg !38990
  br i1 %_53.i.i475, label %bb21.i.i, label %panic1.i.i, !dbg !38990

panic.i.i:                                        ; preds = %bb39.i.i.7, %bb39.i.i.6, %bb39.i.i.5, %bb39.i.i.4, %bb39.i.i.3, %bb39.i.i.2, %bb39.i.i.1, %bb39.i.i.lr.ph
  %_145.1.i.i.lcssa.ph = phi i64 [ 7, %bb39.i.i.7 ], [ 6, %bb39.i.i.6 ], [ 5, %bb39.i.i.5 ], [ 4, %bb39.i.i.4 ], [ 3, %bb39.i.i.3 ], [ 2, %bb39.i.i.2 ], [ 1, %bb39.i.i.1 ], [ 0, %bb39.i.i.lr.ph ]
  store float %_0.i3257, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i.i.lcssa.ph, i64 noundef %_145.1.i.i.lcssa.ph, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a6feb40b34112df5f214f84424dd2c7c) #30, !dbg !38934, !noalias !38922
  unreachable, !dbg !38934

bb21.i.i:                                         ; preds = %bb17.i.i
  %499 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_51.i.i, !dbg !38990
  %_49.i.i476 = load float, ptr %499, align 4, !dbg !38990, !noalias !38922, !noundef !12
  store float %_49.i.i476, ptr %scratch.i, align 4, !dbg !38991, !noalias !38922
  %500 = icmp eq i64 %width.i.i, 1, !dbg !38931
  br i1 %500, label %bb16.i.i, label %bb39.i.i.1, !dbg !38931

bb39.i.i.1:                                       ; preds = %bb21.i.i
  %exitcond11118.1.not = icmp eq i64 %_145.1.i.i, 1, !dbg !38934
  br i1 %exitcond11118.1.not, label %panic.i.i, label %bb17.i.i.1, !dbg !38934

bb17.i.i.1:                                       ; preds = %bb39.i.i.1
  %501 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 20, !dbg !38934
  %_44.i.i473.1 = load i32, ptr %501, align 4, !dbg !38934, !noalias !38922, !noundef !12
  %_43.i.i474.1 = zext i32 %_44.i.i473.1 to i64, !dbg !38934
  %502 = add i64 %ring_cursor.sroa.0.1.i4557643, %_43.i.i474.1, !dbg !38987
  %_47.not.i.i.1 = icmp ult i64 %502, %_86.i, !dbg !38988
  %503 = select i1 %_47.not.i.i.1, i64 0, i64 %_86.i, !dbg !38988
  %spec.select.i.i.1 = sub nuw i64 %502, %503, !dbg !38988
  %_51.i.i.1 = mul i64 %spec.select.i.i.1, %width.i.i, !dbg !38989
  %_50.i.i.1 = add i64 %_51.i.i.1, 1, !dbg !38989
  %_53.i.i475.1 = icmp ult i64 %_50.i.i.1, %_149.1.i.i.pre, !dbg !38990
  br i1 %_53.i.i475.1, label %bb21.i.i.1, label %panic1.i.i, !dbg !38990

bb21.i.i.1:                                       ; preds = %bb17.i.i.1
  %504 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.1, !dbg !38990
  %_49.i.i476.1 = load float, ptr %504, align 4, !dbg !38990, !noalias !38922, !noundef !12
  store float %_49.i.i476.1, ptr %iter.i.i.sroa.0.0.ptr7640.1, align 4, !dbg !38991, !noalias !38922
  %505 = icmp eq i64 %width.i.i, 2, !dbg !38931
  br i1 %505, label %bb16.i.i, label %bb39.i.i.2, !dbg !38931

bb39.i.i.2:                                       ; preds = %bb21.i.i.1
  %exitcond11118.2.not = icmp eq i64 %_145.1.i.i, 2, !dbg !38934
  br i1 %exitcond11118.2.not, label %panic.i.i, label %bb17.i.i.2, !dbg !38934

bb17.i.i.2:                                       ; preds = %bb39.i.i.2
  %506 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 32, !dbg !38934
  %_44.i.i473.2 = load i32, ptr %506, align 4, !dbg !38934, !noalias !38922, !noundef !12
  %_43.i.i474.2 = zext i32 %_44.i.i473.2 to i64, !dbg !38934
  %507 = add i64 %ring_cursor.sroa.0.1.i4557643, %_43.i.i474.2, !dbg !38987
  %_47.not.i.i.2 = icmp ult i64 %507, %_86.i, !dbg !38988
  %508 = select i1 %_47.not.i.i.2, i64 0, i64 %_86.i, !dbg !38988
  %spec.select.i.i.2 = sub nuw i64 %507, %508, !dbg !38988
  %_51.i.i.2 = mul i64 %spec.select.i.i.2, %width.i.i, !dbg !38989
  %_50.i.i.2 = add i64 %_51.i.i.2, 2, !dbg !38989
  %_53.i.i475.2 = icmp ult i64 %_50.i.i.2, %_149.1.i.i.pre, !dbg !38990
  br i1 %_53.i.i475.2, label %bb21.i.i.2, label %panic1.i.i, !dbg !38990

bb21.i.i.2:                                       ; preds = %bb17.i.i.2
  %509 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.2, !dbg !38990
  %_49.i.i476.2 = load float, ptr %509, align 4, !dbg !38990, !noalias !38922, !noundef !12
  store float %_49.i.i476.2, ptr %iter.i.i.sroa.0.0.ptr7640.2, align 4, !dbg !38991, !noalias !38922
  %510 = icmp eq i64 %width.i.i, 3, !dbg !38931
  br i1 %510, label %bb16.i.i, label %bb39.i.i.3, !dbg !38931

bb39.i.i.3:                                       ; preds = %bb21.i.i.2
  %exitcond11118.3.not = icmp eq i64 %_145.1.i.i, 3, !dbg !38934
  br i1 %exitcond11118.3.not, label %panic.i.i, label %bb17.i.i.3, !dbg !38934

bb17.i.i.3:                                       ; preds = %bb39.i.i.3
  %511 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 44, !dbg !38934
  %_44.i.i473.3 = load i32, ptr %511, align 4, !dbg !38934, !noalias !38922, !noundef !12
  %_43.i.i474.3 = zext i32 %_44.i.i473.3 to i64, !dbg !38934
  %512 = add i64 %ring_cursor.sroa.0.1.i4557643, %_43.i.i474.3, !dbg !38987
  %_47.not.i.i.3 = icmp ult i64 %512, %_86.i, !dbg !38988
  %513 = select i1 %_47.not.i.i.3, i64 0, i64 %_86.i, !dbg !38988
  %spec.select.i.i.3 = sub nuw i64 %512, %513, !dbg !38988
  %_51.i.i.3 = mul i64 %spec.select.i.i.3, %width.i.i, !dbg !38989
  %_50.i.i.3 = add i64 %_51.i.i.3, 3, !dbg !38989
  %_53.i.i475.3 = icmp ult i64 %_50.i.i.3, %_149.1.i.i.pre, !dbg !38990
  br i1 %_53.i.i475.3, label %bb21.i.i.3, label %panic1.i.i, !dbg !38990

bb21.i.i.3:                                       ; preds = %bb17.i.i.3
  %514 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.3, !dbg !38990
  %_49.i.i476.3 = load float, ptr %514, align 4, !dbg !38990, !noalias !38922, !noundef !12
  store float %_49.i.i476.3, ptr %iter.i.i.sroa.0.0.ptr7640.3, align 4, !dbg !38991, !noalias !38922
  %515 = icmp eq i64 %width.i.i, 4, !dbg !38931
  br i1 %515, label %bb16.i.i, label %bb39.i.i.4, !dbg !38931

bb39.i.i.4:                                       ; preds = %bb21.i.i.3
  %exitcond11118.4.not = icmp eq i64 %_145.1.i.i, 4, !dbg !38934
  br i1 %exitcond11118.4.not, label %panic.i.i, label %bb17.i.i.4, !dbg !38934

bb17.i.i.4:                                       ; preds = %bb39.i.i.4
  %516 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 56, !dbg !38934
  %_44.i.i473.4 = load i32, ptr %516, align 4, !dbg !38934, !noalias !38922, !noundef !12
  %_43.i.i474.4 = zext i32 %_44.i.i473.4 to i64, !dbg !38934
  %517 = add i64 %ring_cursor.sroa.0.1.i4557643, %_43.i.i474.4, !dbg !38987
  %_47.not.i.i.4 = icmp ult i64 %517, %_86.i, !dbg !38988
  %518 = select i1 %_47.not.i.i.4, i64 0, i64 %_86.i, !dbg !38988
  %spec.select.i.i.4 = sub nuw i64 %517, %518, !dbg !38988
  %_51.i.i.4 = mul i64 %spec.select.i.i.4, %width.i.i, !dbg !38989
  %_50.i.i.4 = add i64 %_51.i.i.4, 4, !dbg !38989
  %_53.i.i475.4 = icmp ult i64 %_50.i.i.4, %_149.1.i.i.pre, !dbg !38990
  br i1 %_53.i.i475.4, label %bb21.i.i.4, label %panic1.i.i, !dbg !38990

bb21.i.i.4:                                       ; preds = %bb17.i.i.4
  %519 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.4, !dbg !38990
  %_49.i.i476.4 = load float, ptr %519, align 4, !dbg !38990, !noalias !38922, !noundef !12
  store float %_49.i.i476.4, ptr %iter.i.i.sroa.0.0.ptr7640.4, align 4, !dbg !38991, !noalias !38922
  %520 = icmp eq i64 %width.i.i, 5, !dbg !38931
  br i1 %520, label %bb16.i.i, label %bb39.i.i.5, !dbg !38931

bb39.i.i.5:                                       ; preds = %bb21.i.i.4
  %exitcond11118.5.not = icmp eq i64 %_145.1.i.i, 5, !dbg !38934
  br i1 %exitcond11118.5.not, label %panic.i.i, label %bb17.i.i.5, !dbg !38934

bb17.i.i.5:                                       ; preds = %bb39.i.i.5
  %521 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 68, !dbg !38934
  %_44.i.i473.5 = load i32, ptr %521, align 4, !dbg !38934, !noalias !38922, !noundef !12
  %_43.i.i474.5 = zext i32 %_44.i.i473.5 to i64, !dbg !38934
  %522 = add i64 %ring_cursor.sroa.0.1.i4557643, %_43.i.i474.5, !dbg !38987
  %_47.not.i.i.5 = icmp ult i64 %522, %_86.i, !dbg !38988
  %523 = select i1 %_47.not.i.i.5, i64 0, i64 %_86.i, !dbg !38988
  %spec.select.i.i.5 = sub nuw i64 %522, %523, !dbg !38988
  %_51.i.i.5 = mul i64 %spec.select.i.i.5, %width.i.i, !dbg !38989
  %_50.i.i.5 = add i64 %_51.i.i.5, 5, !dbg !38989
  %_53.i.i475.5 = icmp ult i64 %_50.i.i.5, %_149.1.i.i.pre, !dbg !38990
  br i1 %_53.i.i475.5, label %bb21.i.i.5, label %panic1.i.i, !dbg !38990

bb21.i.i.5:                                       ; preds = %bb17.i.i.5
  %524 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.5, !dbg !38990
  %_49.i.i476.5 = load float, ptr %524, align 4, !dbg !38990, !noalias !38922, !noundef !12
  store float %_49.i.i476.5, ptr %iter.i.i.sroa.0.0.ptr7640.5, align 4, !dbg !38991, !noalias !38922
  %525 = icmp eq i64 %width.i.i, 6, !dbg !38931
  br i1 %525, label %bb16.i.i, label %bb39.i.i.6, !dbg !38931

bb39.i.i.6:                                       ; preds = %bb21.i.i.5
  %exitcond11118.6.not = icmp eq i64 %_145.1.i.i, 6, !dbg !38934
  br i1 %exitcond11118.6.not, label %panic.i.i, label %bb17.i.i.6, !dbg !38934

bb17.i.i.6:                                       ; preds = %bb39.i.i.6
  %526 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 80, !dbg !38934
  %_44.i.i473.6 = load i32, ptr %526, align 4, !dbg !38934, !noalias !38922, !noundef !12
  %_43.i.i474.6 = zext i32 %_44.i.i473.6 to i64, !dbg !38934
  %527 = add i64 %ring_cursor.sroa.0.1.i4557643, %_43.i.i474.6, !dbg !38987
  %_47.not.i.i.6 = icmp ult i64 %527, %_86.i, !dbg !38988
  %528 = select i1 %_47.not.i.i.6, i64 0, i64 %_86.i, !dbg !38988
  %spec.select.i.i.6 = sub nuw i64 %527, %528, !dbg !38988
  %_51.i.i.6 = mul i64 %spec.select.i.i.6, %width.i.i, !dbg !38989
  %_50.i.i.6 = add i64 %_51.i.i.6, 6, !dbg !38989
  %_53.i.i475.6 = icmp ult i64 %_50.i.i.6, %_149.1.i.i.pre, !dbg !38990
  br i1 %_53.i.i475.6, label %bb21.i.i.6, label %panic1.i.i, !dbg !38990

bb21.i.i.6:                                       ; preds = %bb17.i.i.6
  %529 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.6, !dbg !38990
  %_49.i.i476.6 = load float, ptr %529, align 4, !dbg !38990, !noalias !38922, !noundef !12
  store float %_49.i.i476.6, ptr %iter.i.i.sroa.0.0.ptr7640.6, align 4, !dbg !38991, !noalias !38922
  %530 = icmp eq i64 %width.i.i, 7, !dbg !38931
  br i1 %530, label %bb16.i.i, label %bb39.i.i.7, !dbg !38931

bb39.i.i.7:                                       ; preds = %bb21.i.i.6
  %exitcond11118.7.not = icmp eq i64 %_145.1.i.i, 7, !dbg !38934
  br i1 %exitcond11118.7.not, label %panic.i.i, label %bb17.i.i.7, !dbg !38934

bb17.i.i.7:                                       ; preds = %bb39.i.i.7
  %531 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 92, !dbg !38934
  %_44.i.i473.7 = load i32, ptr %531, align 4, !dbg !38934, !noalias !38922, !noundef !12
  %_43.i.i474.7 = zext i32 %_44.i.i473.7 to i64, !dbg !38934
  %532 = add i64 %ring_cursor.sroa.0.1.i4557643, %_43.i.i474.7, !dbg !38987
  %_47.not.i.i.7 = icmp ult i64 %532, %_86.i, !dbg !38988
  %533 = select i1 %_47.not.i.i.7, i64 0, i64 %_86.i, !dbg !38988
  %spec.select.i.i.7 = sub nuw i64 %532, %533, !dbg !38988
  %_51.i.i.7 = mul i64 %spec.select.i.i.7, %width.i.i, !dbg !38989
  %_50.i.i.7 = add i64 %_51.i.i.7, 7, !dbg !38989
  %_53.i.i475.7 = icmp ult i64 %_50.i.i.7, %_149.1.i.i.pre, !dbg !38990
  br i1 %_53.i.i475.7, label %bb21.i.i.7, label %panic1.i.i, !dbg !38990

bb21.i.i.7:                                       ; preds = %bb17.i.i.7
  %534 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.7, !dbg !38990
  %_49.i.i476.7 = load float, ptr %534, align 4, !dbg !38990, !noalias !38922, !noundef !12
  store float %_49.i.i476.7, ptr %iter.i.i.sroa.0.0.ptr7640.7, align 4, !dbg !38991, !noalias !38922
  br label %bb16.i.i, !dbg !38931

panic1.i.i:                                       ; preds = %bb17.i.i.7, %bb17.i.i.6, %bb17.i.i.5, %bb17.i.i.4, %bb17.i.i.3, %bb17.i.i.2, %bb17.i.i.1, %bb17.i.i
  %_50.i.i.lcssa.ph = phi i64 [ %_50.i.i.7, %bb17.i.i.7 ], [ %_50.i.i.6, %bb17.i.i.6 ], [ %_50.i.i.5, %bb17.i.i.5 ], [ %_50.i.i.4, %bb17.i.i.4 ], [ %_50.i.i.3, %bb17.i.i.3 ], [ %_50.i.i.2, %bb17.i.i.2 ], [ %_50.i.i.1, %bb17.i.i.1 ], [ %_51.i.i, %bb17.i.i ]
  store float %_0.i3257, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i.i.lcssa.ph, i64 noundef %_149.1.i.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b305c1483509cfb31fdec21ff8752674) #30, !dbg !38990, !noalias !38922
  unreachable, !dbg !38990

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3154: ; preds = %bb49.i.i
  %_127.i.i = getelementptr inbounds nuw float, ptr %_150.0.i.i, i64 %_76.i.i, !dbg !38992
  %_0.i3005 = load float, ptr %_127.i.i, align 4, !dbg !38983, !alias.scope !38979, !noalias !38922, !noundef !12
  store float %_0.i3014, ptr %_127.i.i, align 4, !dbg !38994, !alias.scope !38996, !noalias !38922
  %_0.i2650 = fmul float %_0.i2876, %_0.i3005, !dbg !38999
  %_6.i3415 = bitcast float %_0.i3005 to i32, !dbg !39001
  %_5.i3416 = and i32 %_6.i3415, %all.sroa.0.0.i439, !dbg !39004
  %_8.i3417 = bitcast float %_0.i2650 to i32, !dbg !39005
  %_7.i3419 = and i32 %_9.i3431, %_8.i3417, !dbg !39007
  %_4.i3420 = or disjoint i32 %_7.i3419, %_5.i3416, !dbg !39004
  store i32 %_4.i3420, ptr %_149.i, align 4, !dbg !39008, !alias.scope !39010, !noalias !39013
  %535 = add i64 %main_cursor.sroa.0.1.i4567644, 1, !dbg !39014
  %_99.i = icmp eq i64 %535, %_101.i, !dbg !39015
  %spec.store.select.i = select i1 %_99.i, i64 0, i64 %535, !dbg !39015
  %536 = add i64 %ring_cursor.sroa.0.1.i4557643, 1, !dbg !39016
  %_102.i = icmp eq i64 %536, %_86.i, !dbg !39017
  %spec.store.select13.i = select i1 %_102.i, i64 0, i64 %536, !dbg !39017
  %exitcond11121.not = icmp eq i64 %406, %umax11120, !dbg !39018
  br i1 %exitcond11121.not, label %bb16.i.bb13.i.loopexit_crit_edge, label %bb43.i, !dbg !38050

bb46.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3170
  store float %_0.i3257, ptr %371, align 1, !dbg !38032
  store float %_0.i32537788, ptr %387, align 1, !dbg !38046
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_59.i, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_af093af980f3691b22b0bed94901ed34) #30, !dbg !39021, !noalias !38568
  unreachable, !dbg !39021

_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit.loopexit: ; preds = %bb13.i.loopexit
  %537 = trunc i64 %main_cursor.sroa.0.1.i456.lcssa to i32, !dbg !39022
  %538 = trunc i64 %ring_cursor.sroa.0.1.i455.lcssa to i32, !dbg !39023
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit, !dbg !39024

_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit.loopexit, %bb11.i
  %ring_cursor.sroa.0.0.i446.lcssa = phi i32 [ %_36.i440, %bb11.i ], [ %538, %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit.loopexit ], !dbg !38004
  %main_cursor.sroa.0.0.i447.lcssa = phi i32 [ %_34.i, %bb11.i ], [ %537, %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit.loopexit ], !dbg !38001
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %hot_left.i432, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33) #31, !dbg !39025
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %hot_right.i431, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34) #31, !dbg !39026
  store i32 %main_cursor.sroa.0.0.i447.lcssa, ptr %_35, align 4, !dbg !39022, !alias.scope !37983, !noalias !38003
  store i32 %ring_cursor.sroa.0.0.i446.lcssa, ptr %319, align 4, !dbg !39023, !alias.scope !37983, !noalias !38003
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i429), !dbg !39027, !noalias !38008
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i430), !dbg !39028, !noalias !38008
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i), !dbg !39029, !noalias !38008
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockfEB2_.exit, !dbg !37978

bb7.i:                                            ; preds = %bb5.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39030), !dbg !39033
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39034), !dbg !39033
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39036), !dbg !39033
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39038), !dbg !39033
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39040), !dbg !39033
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i37), !dbg !39042, !noalias !39046
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_left.i37, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #31, !dbg !39049, !noalias !39050
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_right.i36), !dbg !39051, !noalias !39046
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_right.i36, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #31, !dbg !39053, !noalias !39054
  %539 = getelementptr inbounds nuw i8, ptr %self, i64 776, !dbg !39055
  %540 = load i8, ptr %539, align 4, !dbg !39055, !range !17, !alias.scope !39030, !noalias !39059, !noundef !12
  %541 = getelementptr inbounds nuw i8, ptr %self, i64 777, !dbg !39060
  %542 = load i8, ptr %541, align 1, !dbg !39060, !range !17, !alias.scope !39030, !noalias !39059, !noundef !12
  %543 = getelementptr inbounds nuw i8, ptr %self, i64 544, !dbg !39062
  %ring.i45 = load i64, ptr %543, align 8, !dbg !39062, !alias.scope !39034, !noalias !39064, !noundef !12
  %544 = getelementptr inbounds nuw i8, ptr %self, i64 552, !dbg !39065
  %main.i46 = load i64, ptr %544, align 8, !dbg !39065, !alias.scope !39034, !noalias !39064, !noundef !12
  %_35.i47 = load i32, ptr %_35, align 4, !dbg !39067, !alias.scope !39040, !noalias !39069, !noundef !12
  %545 = getelementptr inbounds nuw i8, ptr %self, i64 564, !dbg !39070
  %_36.i48 = load i32, ptr %545, align 4, !dbg !39070, !alias.scope !39040, !noalias !39069, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i35), !dbg !39072, !noalias !39046
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i35, i8 0, i64 1024, i1 false), !noalias !39046
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i34), !dbg !39074, !noalias !39046
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i34, i8 0, i64 1024, i1 false), !noalias !39046
  %_31.i41 = zext nneg i8 %540 to i32, !dbg !39055
  %.none.i42 = sub nsw i32 0, %_31.i41, !dbg !39076
  %_32.i43 = zext nneg i8 %542 to i32, !dbg !39060
  %all.sroa.0.0.i44 = sub nsw i32 0, %_32.i43, !dbg !39060
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i33), !dbg !39077, !noalias !39046
; call <true_peak_limiter::UniformHot<f32>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotfE3newB5_(ptr noalias noundef align 8 captures(none) dereferenceable(80) %uniform_left.i33, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33, i64 %ring.i45, i64 %main.i46) #31, !dbg !39079
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_right.i32), !dbg !39080, !noalias !39046
  %_32.val = load i64, ptr %543, align 8, !dbg !39082, !noundef !12
  %_32.val3874 = load i64, ptr %544, align 8, !dbg !39082, !noundef !12
; call <true_peak_limiter::UniformHot<f32>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotfE3newB5_(ptr noalias noundef align 8 captures(none) dereferenceable(80) %uniform_right.i32, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34, i64 %_32.val, i64 %_32.val3874) #31, !dbg !39082
  %_162.not.i598411 = icmp eq i64 %frames, 0, !dbg !39083
  br i1 %_162.not.i598411, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit, label %bb50.i60.lr.ph, !dbg !39083

bb50.i60.lr.ph:                                   ; preds = %bb7.i
  %546 = zext i32 %_36.i48 to i64, !dbg !39070
  %547 = zext i32 %_35.i47 to i64, !dbg !39067
  %d9.i.i4164 = lshr i64 %frames, 5, !dbg !39093
  %r2.i.i4165 = and i64 %frames, 31, !dbg !39099
  %_19.not.i.i4166 = icmp ne i64 %r2.i.i4165, 0, !dbg !39100
  %548 = zext i1 %_19.not.i.i4166 to i64, !dbg !39100
  %yield_count.sroa.0.0.i.i4167 = add nuw nsw i64 %d9.i.i4164, %548, !dbg !39100
  %history.i39.i.sroa.7.0.hot_left.i37.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i37, i64 4
  %history.i39.i.sroa.10.0.hot_left.i37.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i37, i64 8
  %history.i39.i.sroa.13.0.hot_left.i37.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i37, i64 12
  %history.i39.i.sroa.16.0.hot_left.i37.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i37, i64 16
  %history.i39.i.sroa.19.0.hot_left.i37.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i37, i64 20
  %history.i39.i.sroa.22.0.hot_left.i37.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i37, i64 24
  %history.i39.i.sroa.26.0.hot_left.i37.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i37, i64 28
  %history.i39.i.sroa.29.0.hot_left.i37.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i37, i64 32
  %history.i39.i.sroa.32.0.hot_left.i37.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i37, i64 36
  %history.i39.i.sroa.35.0.hot_left.i37.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i37, i64 40
  %history.i39.i.sroa.38.0.hot_left.i37.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i37, i64 44
  %549 = getelementptr inbounds nuw i8, ptr %self, i64 588
  %550 = getelementptr inbounds nuw i8, ptr %self, i64 592
  %551 = getelementptr inbounds nuw i8, ptr %self, i64 596
  %row1.i.i.i75.i = getelementptr inbounds nuw i8, ptr %self, i64 600
  %552 = getelementptr inbounds nuw i8, ptr %self, i64 604
  %553 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %554 = getelementptr inbounds nuw i8, ptr %self, i64 612
  %row3.i.i.i89.i = getelementptr inbounds nuw i8, ptr %self, i64 616
  %555 = getelementptr inbounds nuw i8, ptr %self, i64 620
  %556 = getelementptr inbounds nuw i8, ptr %self, i64 624
  %557 = getelementptr inbounds nuw i8, ptr %self, i64 628
  %row5.i.i.i103.i = getelementptr inbounds nuw i8, ptr %self, i64 632
  %558 = getelementptr inbounds nuw i8, ptr %self, i64 636
  %559 = getelementptr inbounds nuw i8, ptr %self, i64 640
  %560 = getelementptr inbounds nuw i8, ptr %self, i64 644
  %row7.i.i.i117.i = getelementptr inbounds nuw i8, ptr %self, i64 648
  %561 = getelementptr inbounds nuw i8, ptr %self, i64 652
  %562 = getelementptr inbounds nuw i8, ptr %self, i64 656
  %563 = getelementptr inbounds nuw i8, ptr %self, i64 660
  %row9.i.i.i131.i = getelementptr inbounds nuw i8, ptr %self, i64 664
  %564 = getelementptr inbounds nuw i8, ptr %self, i64 668
  %565 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %566 = getelementptr inbounds nuw i8, ptr %self, i64 676
  %row11.i.i.i145.i = getelementptr inbounds nuw i8, ptr %self, i64 680
  %567 = getelementptr inbounds nuw i8, ptr %self, i64 684
  %568 = getelementptr inbounds nuw i8, ptr %self, i64 688
  %569 = getelementptr inbounds nuw i8, ptr %self, i64 692
  %row13.i.i.i159.i = getelementptr inbounds nuw i8, ptr %self, i64 696
  %570 = getelementptr inbounds nuw i8, ptr %self, i64 700
  %571 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %572 = getelementptr inbounds nuw i8, ptr %self, i64 708
  %row15.i.i.i173.i = getelementptr inbounds nuw i8, ptr %self, i64 712
  %573 = getelementptr inbounds nuw i8, ptr %self, i64 716
  %574 = getelementptr inbounds nuw i8, ptr %self, i64 720
  %575 = getelementptr inbounds nuw i8, ptr %self, i64 724
  %row17.i.i.i187.i = getelementptr inbounds nuw i8, ptr %self, i64 728
  %576 = getelementptr inbounds nuw i8, ptr %self, i64 732
  %577 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %578 = getelementptr inbounds nuw i8, ptr %self, i64 740
  %row19.i.i.i201.i = getelementptr inbounds nuw i8, ptr %self, i64 744
  %579 = getelementptr inbounds nuw i8, ptr %self, i64 748
  %580 = getelementptr inbounds nuw i8, ptr %self, i64 752
  %581 = getelementptr inbounds nuw i8, ptr %self, i64 756
  %row21.i.i.i215.i = getelementptr inbounds nuw i8, ptr %self, i64 760
  %582 = getelementptr inbounds nuw i8, ptr %self, i64 764
  %583 = getelementptr inbounds nuw i8, ptr %self, i64 768
  %584 = getelementptr inbounds nuw i8, ptr %self, i64 772
  %history.i.i13.sroa.7.0.hot_right.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i36, i64 4
  %history.i.i13.sroa.10.0.hot_right.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i36, i64 8
  %history.i.i13.sroa.13.0.hot_right.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i36, i64 12
  %history.i.i13.sroa.16.0.hot_right.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i36, i64 16
  %history.i.i13.sroa.19.0.hot_right.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i36, i64 20
  %history.i.i13.sroa.22.0.hot_right.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i36, i64 24
  %history.i.i13.sroa.26.0.hot_right.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i36, i64 28
  %history.i.i13.sroa.29.0.hot_right.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i36, i64 32
  %history.i.i13.sroa.32.0.hot_right.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i36, i64 36
  %history.i.i13.sroa.35.0.hot_right.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i36, i64 40
  %history.i.i13.sroa.38.0.hot_right.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i36, i64 44
  %585 = getelementptr inbounds nuw i8, ptr %uniform_left.i33, i64 48
  %_65.i29.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i33, i64 56
  %_65.i29.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i33, i64 64
  %586 = getelementptr inbounds nuw i8, ptr %uniform_right.i32, i64 48
  %_66.i28.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i32, i64 56
  %_66.i28.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i32, i64 64
  %_109.i106 = getelementptr inbounds nuw i8, ptr %hot_left.i37, i64 48
  %_110.i107 = getelementptr inbounds nuw i8, ptr %hot_left.i37, i64 64
  %587 = getelementptr inbounds nuw i8, ptr %hot_left.i37, i64 60
  %588 = getelementptr inbounds nuw i8, ptr %hot_left.i37, i64 56
  %589 = getelementptr inbounds nuw i8, ptr %hot_left.i37, i64 52
  %590 = getelementptr inbounds nuw i8, ptr %hot_left.i37, i64 76
  %591 = getelementptr inbounds nuw i8, ptr %hot_left.i37, i64 72
  %592 = getelementptr inbounds nuw i8, ptr %hot_left.i37, i64 68
  %_114.i108 = getelementptr inbounds nuw i8, ptr %hot_right.i36, i64 48
  %_115.i109 = getelementptr inbounds nuw i8, ptr %hot_right.i36, i64 64
  %593 = getelementptr inbounds nuw i8, ptr %hot_right.i36, i64 60
  %594 = getelementptr inbounds nuw i8, ptr %hot_right.i36, i64 56
  %595 = getelementptr inbounds nuw i8, ptr %hot_right.i36, i64 52
  %596 = getelementptr inbounds nuw i8, ptr %hot_right.i36, i64 76
  %597 = getelementptr inbounds nuw i8, ptr %hot_right.i36, i64 72
  %598 = getelementptr inbounds nuw i8, ptr %hot_right.i36, i64 68
  %_9.i3491 = add nsw i32 %_31.i41, -1
  %599 = getelementptr inbounds nuw i8, ptr %uniform_left.i33, i64 8
  %_21.i270.i = getelementptr inbounds nuw i8, ptr %uniform_left.i33, i64 72
  %_22.i271.i = getelementptr inbounds nuw i8, ptr %uniform_left.i33, i64 76
  %600 = getelementptr inbounds nuw i8, ptr %uniform_left.i33, i64 16
  %601 = getelementptr inbounds nuw i8, ptr %uniform_left.i33, i64 24
  %602 = getelementptr inbounds nuw i8, ptr %hot_left.i37, i64 84
  %603 = getelementptr inbounds nuw i8, ptr %hot_left.i37, i64 88
  %604 = getelementptr inbounds nuw i8, ptr %hot_left.i37, i64 80
  %605 = getelementptr inbounds nuw i8, ptr %uniform_left.i33, i64 40
  %606 = getelementptr inbounds nuw i8, ptr %uniform_left.i33, i64 32
  %_9.i3471 = add nsw i32 %_32.i43, -1
  %607 = getelementptr inbounds nuw i8, ptr %uniform_right.i32, i64 8
  %_21.i.i154 = getelementptr inbounds nuw i8, ptr %uniform_right.i32, i64 72
  %_22.i.i155 = getelementptr inbounds nuw i8, ptr %uniform_right.i32, i64 76
  %608 = getelementptr inbounds nuw i8, ptr %uniform_right.i32, i64 16
  %609 = getelementptr inbounds nuw i8, ptr %uniform_right.i32, i64 24
  %610 = getelementptr inbounds nuw i8, ptr %hot_right.i36, i64 84
  %611 = getelementptr inbounds nuw i8, ptr %hot_right.i36, i64 88
  %612 = getelementptr inbounds nuw i8, ptr %hot_right.i36, i64 80
  %613 = getelementptr inbounds nuw i8, ptr %uniform_right.i32, i64 40
  %614 = getelementptr inbounds nuw i8, ptr %uniform_right.i32, i64 32
  br label %bb50.i60, !dbg !39083

bb19.i68.bb15.i54.loopexit_crit_edge:             ; preds = %bb32.i198
  store float %_0.i.i.lcssa1187913364, ptr %587, align 4
  store float %_0.i.i3550.lcssa1179513397, ptr %593, align 4
  store float %running.sroa.0.0.i1179.lcssa1189113441, ptr %_21.i270.i, align 1, !dbg !39101
  store float %running.sroa.0.0.i.lcssa1192813458, ptr %_21.i.i154, align 1, !dbg !39132
  store i32 %storemerge.i1184.lcssa82338334, ptr %_22.i271.i, align 4
  store i32 %storemerge.i.lcssa82918373, ptr %_22.i.i155, align 4
  br label %bb15.i54.loopexit, !dbg !39135

bb15.i54.loopexit:                                ; preds = %bb19.i68.bb15.i54.loopexit_crit_edge, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i67
  %ring_cursor.sroa.0.1.i69.lcssa = phi i64 [ %ring_cursor.sroa.0.2.i201, %bb19.i68.bb15.i54.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i558412, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i67 ], !dbg !39136
  %main_cursor.sroa.0.1.i70.lcssa = phi i64 [ %main_cursor.sroa.0.2.i204, %bb19.i68.bb15.i54.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i568413, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i67 ], !dbg !39137
  %_162.not.i59 = icmp eq i64 %616, 0, !dbg !39083
  %indvars.iv.next11123 = add i64 %indvars.iv11122, -32, !dbg !39083
  br i1 %_162.not.i59, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit.loopexit, label %bb50.i60, !dbg !39083

bb50.i60:                                         ; preds = %bb50.i60.lr.ph, %bb15.i54.loopexit
  %indvars.iv11122 = phi i64 [ %frames, %bb50.i60.lr.ph ], [ %indvars.iv.next11123, %bb15.i54.loopexit ]
  %iter2.sroa.0.0.i588415 = phi i64 [ %yield_count.sroa.0.0.i.i4167, %bb50.i60.lr.ph ], [ %616, %bb15.i54.loopexit ]
  %iter1.sroa.0.0.i578414 = phi i64 [ 0, %bb50.i60.lr.ph ], [ %615, %bb15.i54.loopexit ]
  %main_cursor.sroa.0.0.i568413 = phi i64 [ %547, %bb50.i60.lr.ph ], [ %main_cursor.sroa.0.1.i70.lcssa, %bb15.i54.loopexit ]
  %ring_cursor.sroa.0.0.i558412 = phi i64 [ %546, %bb50.i60.lr.ph ], [ %ring_cursor.sroa.0.1.i69.lcssa, %bb15.i54.loopexit ]
  %umin11142 = call i64 @llvm.umin.i64(i64 %indvars.iv11122, i64 32), !dbg !39138
  %umax11128 = call i64 @llvm.umax.i64(i64 %umin11142, i64 1), !dbg !39138
  %615 = add i64 %iter1.sroa.0.0.i578414, 32, !dbg !39138
  %616 = add i64 %iter2.sroa.0.0.i588415, -1, !dbg !39142
  %_46.i62 = sub i64 %frames, %iter1.sroa.0.0.i578414, !dbg !39143
  %..i4168 = tail call noundef i64 @llvm.umin.i64(i64 %_46.i62, i64 32), !dbg !39144
  %history.i39.i.sroa.0.0.copyload = load float, ptr %hot_left.i37, align 4, !dbg !39148, !noalias !39150
  %history.i39.i.sroa.7.0.copyload = load float, ptr %history.i39.i.sroa.7.0.hot_left.i37.sroa_idx, align 4, !dbg !39148, !noalias !39150
  %history.i39.i.sroa.10.0.copyload = load float, ptr %history.i39.i.sroa.10.0.hot_left.i37.sroa_idx, align 4, !dbg !39148, !noalias !39150
  %history.i39.i.sroa.13.0.copyload = load float, ptr %history.i39.i.sroa.13.0.hot_left.i37.sroa_idx, align 4, !dbg !39148, !noalias !39150
  %history.i39.i.sroa.16.0.copyload = load float, ptr %history.i39.i.sroa.16.0.hot_left.i37.sroa_idx, align 4, !dbg !39148, !noalias !39150
  %history.i39.i.sroa.19.0.copyload = load float, ptr %history.i39.i.sroa.19.0.hot_left.i37.sroa_idx, align 4, !dbg !39148, !noalias !39150
  %history.i39.i.sroa.22.0.copyload = load float, ptr %history.i39.i.sroa.22.0.hot_left.i37.sroa_idx, align 4, !dbg !39148, !noalias !39150
  %history.i39.i.sroa.26.0.copyload = load float, ptr %history.i39.i.sroa.26.0.hot_left.i37.sroa_idx, align 4, !dbg !39148, !noalias !39150
  %history.i39.i.sroa.29.0.copyload = load float, ptr %history.i39.i.sroa.29.0.hot_left.i37.sroa_idx, align 4, !dbg !39148, !noalias !39150
  %history.i39.i.sroa.32.0.copyload = load float, ptr %history.i39.i.sroa.32.0.hot_left.i37.sroa_idx, align 4, !dbg !39148, !noalias !39150
  %history.i39.i.sroa.35.0.copyload = load float, ptr %history.i39.i.sroa.35.0.hot_left.i37.sroa_idx, align 4, !dbg !39148, !noalias !39150
  %history.i39.i.sroa.38.0.copyload = load float, ptr %history.i39.i.sroa.38.0.hot_left.i37.sroa_idx, align 4, !dbg !39148, !noalias !39150
  %_20.i42.i7798.not = icmp eq i64 %frames, %iter1.sroa.0.0.i578414, !dbg !39155
  br i1 %_20.i42.i7798.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit243.i, label %bb5.i43.i.lr.ph, !dbg !39159

bb5.i43.i.lr.ph:                                  ; preds = %bb50.i60
  %_11.i.i.i63.i = load float, ptr %_31, align 4
  %_14.i.i.i66.i = load float, ptr %549, align 4
  %_17.i.i.i69.i = load float, ptr %550, align 4
  %_20.i.i.i72.i = load float, ptr %551, align 4
  %_25.i.i.i77.i = load float, ptr %row1.i.i.i75.i, align 4
  %_28.i.i.i80.i = load float, ptr %552, align 4
  %_31.i.i.i83.i = load float, ptr %553, align 4
  %_34.i.i.i86.i = load float, ptr %554, align 4
  %_39.i.i.i91.i = load float, ptr %row3.i.i.i89.i, align 4
  %_42.i.i.i94.i = load float, ptr %555, align 4
  %_45.i.i.i97.i = load float, ptr %556, align 4
  %_48.i.i.i100.i = load float, ptr %557, align 4
  %_53.i.i.i105.i = load float, ptr %row5.i.i.i103.i, align 4
  %_56.i.i.i108.i = load float, ptr %558, align 4
  %_59.i.i.i111.i = load float, ptr %559, align 4
  %_62.i.i.i114.i = load float, ptr %560, align 4
  %_67.i.i.i119.i = load float, ptr %row7.i.i.i117.i, align 4
  %_70.i.i.i122.i = load float, ptr %561, align 4
  %_73.i.i.i125.i = load float, ptr %562, align 4
  %_76.i.i.i128.i = load float, ptr %563, align 4
  %_81.i.i.i133.i = load float, ptr %row9.i.i.i131.i, align 4
  %_84.i.i.i136.i = load float, ptr %564, align 4
  %_87.i.i.i139.i = load float, ptr %565, align 4
  %_90.i.i.i142.i = load float, ptr %566, align 4
  %_95.i.i.i147.i = load float, ptr %row11.i.i.i145.i, align 4
  %_98.i.i.i150.i = load float, ptr %567, align 4
  %_101.i.i.i153.i = load float, ptr %568, align 4
  %_104.i.i.i156.i = load float, ptr %569, align 4
  %_109.i.i.i161.i = load float, ptr %row13.i.i.i159.i, align 4
  %_112.i.i.i164.i = load float, ptr %570, align 4
  %_115.i.i.i167.i = load float, ptr %571, align 4
  %_118.i.i.i170.i = load float, ptr %572, align 4
  %_123.i.i.i175.i = load float, ptr %row15.i.i.i173.i, align 4
  %_126.i.i.i178.i = load float, ptr %573, align 4
  %_129.i.i.i181.i = load float, ptr %574, align 4
  %_132.i.i.i184.i = load float, ptr %575, align 4
  %_137.i.i.i189.i = load float, ptr %row17.i.i.i187.i, align 4
  %_140.i.i.i192.i = load float, ptr %576, align 4
  %_143.i.i.i195.i = load float, ptr %577, align 4
  %_146.i.i.i198.i = load float, ptr %578, align 4
  %_151.i.i.i203.i = load float, ptr %row19.i.i.i201.i, align 4
  %_154.i.i.i206.i = load float, ptr %579, align 4
  %_157.i.i.i209.i = load float, ptr %580, align 4
  %_160.i.i.i212.i = load float, ptr %581, align 4
  %_165.i.i.i217.i = load float, ptr %row21.i.i.i215.i, align 4
  %_168.i.i.i220.i = load float, ptr %582, align 4
  %_171.i.i.i223.i = load float, ptr %583, align 4
  %_174.i.i.i226.i = load float, ptr %584, align 4
  br label %bb5.i43.i, !dbg !39159

bb5.i43.i:                                        ; preds = %bb5.i43.i.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045
  %iter.sroa.0.0.i41.i7810 = phi i64 [ 0, %bb5.i43.i.lr.ph ], [ %617, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045 ]
  %history.i39.i.sroa.35.07809 = phi float [ %history.i39.i.sroa.35.0.copyload, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.32.07808, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045 ]
  %history.i39.i.sroa.32.07808 = phi float [ %history.i39.i.sroa.32.0.copyload, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.29.07807, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045 ]
  %history.i39.i.sroa.29.07807 = phi float [ %history.i39.i.sroa.29.0.copyload, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.26.07806, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045 ]
  %history.i39.i.sroa.26.07806 = phi float [ %history.i39.i.sroa.26.0.copyload, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.22.07805, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045 ]
  %history.i39.i.sroa.22.07805 = phi float [ %history.i39.i.sroa.22.0.copyload, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.19.07804, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045 ]
  %history.i39.i.sroa.19.07804 = phi float [ %history.i39.i.sroa.19.0.copyload, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.16.07803, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045 ]
  %history.i39.i.sroa.16.07803 = phi float [ %history.i39.i.sroa.16.0.copyload, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.13.07802, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045 ]
  %history.i39.i.sroa.13.07802 = phi float [ %history.i39.i.sroa.13.0.copyload, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.10.07801, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045 ]
  %history.i39.i.sroa.10.07801 = phi float [ %history.i39.i.sroa.10.0.copyload, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.7.07800, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045 ]
  %history.i39.i.sroa.7.07800 = phi float [ %history.i39.i.sroa.7.0.copyload, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.0.07799, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045 ]
  %history.i39.i.sroa.0.07799 = phi float [ %history.i39.i.sroa.0.0.copyload, %bb5.i43.i.lr.ph ], [ %_0.i3043, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045 ]
  %617 = add nuw nsw i64 %iter.sroa.0.0.i41.i7810, 1, !dbg !39160
  %_11.i44.i = add nuw nsw i64 %iter.sroa.0.0.i41.i7810, %iter1.sroa.0.0.i578414, !dbg !39163
  %_24.i45.i = icmp ugt i64 %_11.i44.i, %left_io.1, !dbg !39164
  br i1 %_24.i45.i, label %bb7.i242.i, label %bb8.i46.i, !dbg !39164, !prof !639

bb8.i46.i:                                        ; preds = %bb5.i43.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39167), !dbg !39170
  %_3.not.i3041 = icmp eq i64 %left_io.1, %_11.i44.i, !dbg !39171
  br i1 %_3.not.i3041, label %panic.i3044, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045, !dbg !39171

panic.i3044:                                      ; preds = %bb8.i46.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !39171, !noalias !39173
  unreachable, !dbg !39171

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045: ; preds = %bb8.i46.i
  %_31.i48.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %_11.i44.i, !dbg !39175
  %_0.i3043 = load float, ptr %_31.i48.i, align 4, !dbg !39171, !alias.scope !39167, !noalias !39177, !noundef !12
  %618 = tail call noundef float @llvm.fabs.f32(float %history.i39.i.sroa.19.07804), !dbg !39178
  %_0.i2703 = fmul float %_0.i3043, %_11.i.i.i63.i, !dbg !39181
  %_0.i2271 = fadd float %_0.i2703, 0.000000e+00, !dbg !39184
  %_0.i2702 = fmul float %_0.i3043, %_14.i.i.i66.i, !dbg !39186
  %_0.i2270 = fadd float %_0.i2702, 0.000000e+00, !dbg !39188
  %_0.i2701 = fmul float %_0.i3043, %_17.i.i.i69.i, !dbg !39190
  %_0.i2269 = fadd float %_0.i2701, 0.000000e+00, !dbg !39192
  %_0.i2700 = fmul float %_0.i3043, %_20.i.i.i72.i, !dbg !39194
  %_0.i2268 = fadd float %_0.i2700, 0.000000e+00, !dbg !39196
  %_0.i2699 = fmul float %history.i39.i.sroa.0.07799, %_25.i.i.i77.i, !dbg !39198
  %_0.i2267 = fadd float %_0.i2271, %_0.i2699, !dbg !39200
  %_0.i2698 = fmul float %history.i39.i.sroa.0.07799, %_28.i.i.i80.i, !dbg !39202
  %_0.i2266 = fadd float %_0.i2270, %_0.i2698, !dbg !39204
  %_0.i2697 = fmul float %history.i39.i.sroa.0.07799, %_31.i.i.i83.i, !dbg !39206
  %_0.i2265 = fadd float %_0.i2269, %_0.i2697, !dbg !39208
  %_0.i2696 = fmul float %history.i39.i.sroa.0.07799, %_34.i.i.i86.i, !dbg !39210
  %_0.i2264 = fadd float %_0.i2268, %_0.i2696, !dbg !39212
  %_0.i2695 = fmul float %history.i39.i.sroa.7.07800, %_39.i.i.i91.i, !dbg !39214
  %_0.i2263 = fadd float %_0.i2267, %_0.i2695, !dbg !39216
  %_0.i2694 = fmul float %history.i39.i.sroa.7.07800, %_42.i.i.i94.i, !dbg !39218
  %_0.i2262 = fadd float %_0.i2266, %_0.i2694, !dbg !39220
  %_0.i2693 = fmul float %history.i39.i.sroa.7.07800, %_45.i.i.i97.i, !dbg !39222
  %_0.i2261 = fadd float %_0.i2265, %_0.i2693, !dbg !39224
  %_0.i2692 = fmul float %history.i39.i.sroa.7.07800, %_48.i.i.i100.i, !dbg !39226
  %_0.i2260 = fadd float %_0.i2264, %_0.i2692, !dbg !39228
  %_0.i2691 = fmul float %history.i39.i.sroa.10.07801, %_53.i.i.i105.i, !dbg !39230
  %_0.i2259 = fadd float %_0.i2263, %_0.i2691, !dbg !39232
  %_0.i2690 = fmul float %history.i39.i.sroa.10.07801, %_56.i.i.i108.i, !dbg !39234
  %_0.i2258 = fadd float %_0.i2262, %_0.i2690, !dbg !39236
  %_0.i2689 = fmul float %history.i39.i.sroa.10.07801, %_59.i.i.i111.i, !dbg !39238
  %_0.i2257 = fadd float %_0.i2261, %_0.i2689, !dbg !39240
  %_0.i2688 = fmul float %history.i39.i.sroa.10.07801, %_62.i.i.i114.i, !dbg !39242
  %_0.i2256 = fadd float %_0.i2260, %_0.i2688, !dbg !39244
  %_0.i2687 = fmul float %history.i39.i.sroa.13.07802, %_67.i.i.i119.i, !dbg !39246
  %_0.i2255 = fadd float %_0.i2259, %_0.i2687, !dbg !39248
  %_0.i2686 = fmul float %history.i39.i.sroa.13.07802, %_70.i.i.i122.i, !dbg !39250
  %_0.i2254 = fadd float %_0.i2258, %_0.i2686, !dbg !39252
  %_0.i2685 = fmul float %history.i39.i.sroa.13.07802, %_73.i.i.i125.i, !dbg !39254
  %_0.i2253 = fadd float %_0.i2257, %_0.i2685, !dbg !39256
  %_0.i2684 = fmul float %history.i39.i.sroa.13.07802, %_76.i.i.i128.i, !dbg !39258
  %_0.i2252 = fadd float %_0.i2256, %_0.i2684, !dbg !39260
  %_0.i2683 = fmul float %history.i39.i.sroa.16.07803, %_81.i.i.i133.i, !dbg !39262
  %_0.i2251 = fadd float %_0.i2255, %_0.i2683, !dbg !39264
  %_0.i2682 = fmul float %history.i39.i.sroa.16.07803, %_84.i.i.i136.i, !dbg !39266
  %_0.i2250 = fadd float %_0.i2254, %_0.i2682, !dbg !39268
  %_0.i2681 = fmul float %history.i39.i.sroa.16.07803, %_87.i.i.i139.i, !dbg !39270
  %_0.i2249 = fadd float %_0.i2253, %_0.i2681, !dbg !39272
  %_0.i2680 = fmul float %history.i39.i.sroa.16.07803, %_90.i.i.i142.i, !dbg !39274
  %_0.i2248 = fadd float %_0.i2252, %_0.i2680, !dbg !39276
  %_0.i2679 = fmul float %history.i39.i.sroa.19.07804, %_95.i.i.i147.i, !dbg !39278
  %_0.i2247 = fadd float %_0.i2251, %_0.i2679, !dbg !39280
  %_0.i2678 = fmul float %history.i39.i.sroa.19.07804, %_98.i.i.i150.i, !dbg !39282
  %_0.i2246 = fadd float %_0.i2250, %_0.i2678, !dbg !39284
  %_0.i2677 = fmul float %history.i39.i.sroa.19.07804, %_101.i.i.i153.i, !dbg !39286
  %_0.i2245 = fadd float %_0.i2249, %_0.i2677, !dbg !39288
  %_0.i2676 = fmul float %history.i39.i.sroa.19.07804, %_104.i.i.i156.i, !dbg !39290
  %_0.i2244 = fadd float %_0.i2248, %_0.i2676, !dbg !39292
  %_0.i2675 = fmul float %history.i39.i.sroa.22.07805, %_109.i.i.i161.i, !dbg !39294
  %_0.i2243 = fadd float %_0.i2247, %_0.i2675, !dbg !39296
  %_0.i2674 = fmul float %history.i39.i.sroa.22.07805, %_112.i.i.i164.i, !dbg !39298
  %_0.i2242 = fadd float %_0.i2246, %_0.i2674, !dbg !39300
  %_0.i2673 = fmul float %history.i39.i.sroa.22.07805, %_115.i.i.i167.i, !dbg !39302
  %_0.i2241 = fadd float %_0.i2245, %_0.i2673, !dbg !39304
  %_0.i2672 = fmul float %history.i39.i.sroa.22.07805, %_118.i.i.i170.i, !dbg !39306
  %_0.i2240 = fadd float %_0.i2244, %_0.i2672, !dbg !39308
  %_0.i2671 = fmul float %history.i39.i.sroa.26.07806, %_123.i.i.i175.i, !dbg !39310
  %_0.i2239 = fadd float %_0.i2243, %_0.i2671, !dbg !39312
  %_0.i2670 = fmul float %history.i39.i.sroa.26.07806, %_126.i.i.i178.i, !dbg !39314
  %_0.i2238 = fadd float %_0.i2242, %_0.i2670, !dbg !39316
  %_0.i2669 = fmul float %history.i39.i.sroa.26.07806, %_129.i.i.i181.i, !dbg !39318
  %_0.i2237 = fadd float %_0.i2241, %_0.i2669, !dbg !39320
  %_0.i2668 = fmul float %history.i39.i.sroa.26.07806, %_132.i.i.i184.i, !dbg !39322
  %_0.i2236 = fadd float %_0.i2240, %_0.i2668, !dbg !39324
  %_0.i2667 = fmul float %history.i39.i.sroa.29.07807, %_137.i.i.i189.i, !dbg !39326
  %_0.i2235 = fadd float %_0.i2239, %_0.i2667, !dbg !39328
  %_0.i2666 = fmul float %history.i39.i.sroa.29.07807, %_140.i.i.i192.i, !dbg !39330
  %_0.i2234 = fadd float %_0.i2238, %_0.i2666, !dbg !39332
  %_0.i2665 = fmul float %history.i39.i.sroa.29.07807, %_143.i.i.i195.i, !dbg !39334
  %_0.i2233 = fadd float %_0.i2237, %_0.i2665, !dbg !39336
  %_0.i2664 = fmul float %history.i39.i.sroa.29.07807, %_146.i.i.i198.i, !dbg !39338
  %_0.i2232 = fadd float %_0.i2236, %_0.i2664, !dbg !39340
  %_0.i2663 = fmul float %history.i39.i.sroa.32.07808, %_151.i.i.i203.i, !dbg !39342
  %_0.i2231 = fadd float %_0.i2235, %_0.i2663, !dbg !39344
  %_0.i2662 = fmul float %history.i39.i.sroa.32.07808, %_154.i.i.i206.i, !dbg !39346
  %_0.i2230 = fadd float %_0.i2234, %_0.i2662, !dbg !39348
  %_0.i2661 = fmul float %history.i39.i.sroa.32.07808, %_157.i.i.i209.i, !dbg !39350
  %_0.i2229 = fadd float %_0.i2233, %_0.i2661, !dbg !39352
  %_0.i2660 = fmul float %history.i39.i.sroa.32.07808, %_160.i.i.i212.i, !dbg !39354
  %_0.i2228 = fadd float %_0.i2232, %_0.i2660, !dbg !39356
  %_0.i2659 = fmul float %history.i39.i.sroa.35.07809, %_165.i.i.i217.i, !dbg !39358
  %_0.i2227 = fadd float %_0.i2231, %_0.i2659, !dbg !39360
  %_0.i2658 = fmul float %history.i39.i.sroa.35.07809, %_168.i.i.i220.i, !dbg !39362
  %_0.i2226 = fadd float %_0.i2230, %_0.i2658, !dbg !39364
  %_0.i2657 = fmul float %history.i39.i.sroa.35.07809, %_171.i.i.i223.i, !dbg !39366
  %_0.i2225 = fadd float %_0.i2229, %_0.i2657, !dbg !39368
  %_0.i2656 = fmul float %history.i39.i.sroa.35.07809, %_174.i.i.i226.i, !dbg !39370
  %_0.i2224 = fadd float %_0.i2228, %_0.i2656, !dbg !39372
  %619 = tail call noundef float @llvm.fabs.f32(float %_0.i2227), !dbg !39374
  %_3.i.i3675.inv = fcmp ogt float %618, %619, !dbg !39376
  %_4.i.i3682.v = select i1 %_3.i.i3675.inv, float %618, float %619, !dbg !39376
  %620 = tail call noundef float @llvm.fabs.f32(float %_0.i2226), !dbg !39374
  %_3.i.i3675.inv.1 = fcmp ogt float %_4.i.i3682.v, %620, !dbg !39376
  %_4.i.i3682.v.1 = select i1 %_3.i.i3675.inv.1, float %_4.i.i3682.v, float %620, !dbg !39376
  %621 = tail call noundef float @llvm.fabs.f32(float %_0.i2225), !dbg !39374
  %_3.i.i3675.inv.2 = fcmp ogt float %_4.i.i3682.v.1, %621, !dbg !39376
  %_4.i.i3682.v.2 = select i1 %_3.i.i3675.inv.2, float %_4.i.i3682.v.1, float %621, !dbg !39376
  %622 = tail call noundef float @llvm.fabs.f32(float %_0.i2224), !dbg !39374
  %_3.i.i3675.inv.3 = fcmp ogt float %_4.i.i3682.v.2, %622, !dbg !39376
  %_4.i.i3682.v.3 = select i1 %_3.i.i3675.inv.3, float %_4.i.i3682.v.2, float %622, !dbg !39376
  %_39.i237.i = getelementptr inbounds nuw float, ptr %peaks_left.i35, i64 %iter.sroa.0.0.i41.i7810, !dbg !39379
  store float %_4.i.i3682.v.3, ptr %_39.i237.i, align 4, !dbg !39384, !alias.scope !39386, !noalias !39177
  %exitcond11126.not = icmp eq i64 %617, %umax11128, !dbg !39155
  br i1 %exitcond11126.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit243.i, label %bb5.i43.i, !dbg !39159

bb7.i242.i:                                       ; preds = %bb5.i43.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_11.i44.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_283c8d9f84e10f29a75d8e6c3a347bea) #30, !dbg !39389, !noalias !39177
  unreachable, !dbg !39389

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit243.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045, %bb50.i60
  %history.i39.i.sroa.0.0.lcssa = phi float [ %history.i39.i.sroa.0.0.copyload, %bb50.i60 ], [ %_0.i3043, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045 ], !dbg !39390
  %history.i39.i.sroa.7.0.lcssa = phi float [ %history.i39.i.sroa.7.0.copyload, %bb50.i60 ], [ %history.i39.i.sroa.0.07799, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045 ], !dbg !39390
  %history.i39.i.sroa.10.0.lcssa = phi float [ %history.i39.i.sroa.10.0.copyload, %bb50.i60 ], [ %history.i39.i.sroa.7.07800, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045 ], !dbg !39390
  %history.i39.i.sroa.13.0.lcssa = phi float [ %history.i39.i.sroa.13.0.copyload, %bb50.i60 ], [ %history.i39.i.sroa.10.07801, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045 ], !dbg !39390
  %history.i39.i.sroa.16.0.lcssa = phi float [ %history.i39.i.sroa.16.0.copyload, %bb50.i60 ], [ %history.i39.i.sroa.13.07802, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045 ], !dbg !39390
  %history.i39.i.sroa.19.0.lcssa = phi float [ %history.i39.i.sroa.19.0.copyload, %bb50.i60 ], [ %history.i39.i.sroa.16.07803, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045 ], !dbg !39390
  %history.i39.i.sroa.22.0.lcssa = phi float [ %history.i39.i.sroa.22.0.copyload, %bb50.i60 ], [ %history.i39.i.sroa.19.07804, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045 ], !dbg !39390
  %history.i39.i.sroa.26.0.lcssa = phi float [ %history.i39.i.sroa.26.0.copyload, %bb50.i60 ], [ %history.i39.i.sroa.22.07805, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045 ], !dbg !39390
  %history.i39.i.sroa.29.0.lcssa = phi float [ %history.i39.i.sroa.29.0.copyload, %bb50.i60 ], [ %history.i39.i.sroa.26.07806, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045 ], !dbg !39390
  %history.i39.i.sroa.32.0.lcssa = phi float [ %history.i39.i.sroa.32.0.copyload, %bb50.i60 ], [ %history.i39.i.sroa.29.07807, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045 ], !dbg !39390
  %history.i39.i.sroa.35.0.lcssa = phi float [ %history.i39.i.sroa.35.0.copyload, %bb50.i60 ], [ %history.i39.i.sroa.32.07808, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045 ], !dbg !39390
  %history.i39.i.sroa.38.0.lcssa = phi float [ %history.i39.i.sroa.38.0.copyload, %bb50.i60 ], [ %history.i39.i.sroa.35.07809, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3045 ], !dbg !39390
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i37, align 4, !dbg !39391, !noalias !39150
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i37.sroa_idx, align 4, !dbg !39391, !noalias !39150
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i37.sroa_idx, align 4, !dbg !39391, !noalias !39150
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i37.sroa_idx, align 4, !dbg !39391, !noalias !39150
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i37.sroa_idx, align 4, !dbg !39391, !noalias !39150
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i37.sroa_idx, align 4, !dbg !39391, !noalias !39150
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i37.sroa_idx, align 4, !dbg !39391, !noalias !39150
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i37.sroa_idx, align 4, !dbg !39391, !noalias !39150
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i37.sroa_idx, align 4, !dbg !39391, !noalias !39150
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i37.sroa_idx, align 4, !dbg !39391, !noalias !39150
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i37.sroa_idx, align 4, !dbg !39391, !noalias !39150
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i37.sroa_idx, align 4, !dbg !39391, !noalias !39150
  %history.i.i13.sroa.0.0.copyload = load float, ptr %hot_right.i36, align 4, !dbg !39392, !noalias !39394
  %history.i.i13.sroa.7.0.copyload = load float, ptr %history.i.i13.sroa.7.0.hot_right.i36.sroa_idx, align 4, !dbg !39392, !noalias !39394
  %history.i.i13.sroa.10.0.copyload = load float, ptr %history.i.i13.sroa.10.0.hot_right.i36.sroa_idx, align 4, !dbg !39392, !noalias !39394
  %history.i.i13.sroa.13.0.copyload = load float, ptr %history.i.i13.sroa.13.0.hot_right.i36.sroa_idx, align 4, !dbg !39392, !noalias !39394
  %history.i.i13.sroa.16.0.copyload = load float, ptr %history.i.i13.sroa.16.0.hot_right.i36.sroa_idx, align 4, !dbg !39392, !noalias !39394
  %history.i.i13.sroa.19.0.copyload = load float, ptr %history.i.i13.sroa.19.0.hot_right.i36.sroa_idx, align 4, !dbg !39392, !noalias !39394
  %history.i.i13.sroa.22.0.copyload = load float, ptr %history.i.i13.sroa.22.0.hot_right.i36.sroa_idx, align 4, !dbg !39392, !noalias !39394
  %history.i.i13.sroa.26.0.copyload = load float, ptr %history.i.i13.sroa.26.0.hot_right.i36.sroa_idx, align 4, !dbg !39392, !noalias !39394
  %history.i.i13.sroa.29.0.copyload = load float, ptr %history.i.i13.sroa.29.0.hot_right.i36.sroa_idx, align 4, !dbg !39392, !noalias !39394
  %history.i.i13.sroa.32.0.copyload = load float, ptr %history.i.i13.sroa.32.0.hot_right.i36.sroa_idx, align 4, !dbg !39392, !noalias !39394
  %history.i.i13.sroa.35.0.copyload = load float, ptr %history.i.i13.sroa.35.0.hot_right.i36.sroa_idx, align 4, !dbg !39392, !noalias !39394
  %history.i.i13.sroa.38.0.copyload = load float, ptr %history.i.i13.sroa.38.0.hot_right.i36.sroa_idx, align 4, !dbg !39392, !noalias !39394
  br i1 %_20.i42.i7798.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i67, label %bb5.i.i207.lr.ph, !dbg !39399

bb5.i.i207.lr.ph:                                 ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit243.i
  %_11.i.i.i.i226 = load float, ptr %_31, align 4
  %_14.i.i.i.i229 = load float, ptr %549, align 4
  %_17.i.i.i.i232 = load float, ptr %550, align 4
  %_20.i.i.i.i235 = load float, ptr %551, align 4
  %_25.i.i.i.i240 = load float, ptr %row1.i.i.i75.i, align 4
  %_28.i.i.i.i243 = load float, ptr %552, align 4
  %_31.i.i.i.i246 = load float, ptr %553, align 4
  %_34.i.i.i.i249 = load float, ptr %554, align 4
  %_39.i.i.i.i254 = load float, ptr %row3.i.i.i89.i, align 4
  %_42.i.i.i.i257 = load float, ptr %555, align 4
  %_45.i.i.i.i260 = load float, ptr %556, align 4
  %_48.i.i.i.i263 = load float, ptr %557, align 4
  %_53.i.i.i.i268 = load float, ptr %row5.i.i.i103.i, align 4
  %_56.i.i.i.i271 = load float, ptr %558, align 4
  %_59.i.i.i.i274 = load float, ptr %559, align 4
  %_62.i.i.i.i277 = load float, ptr %560, align 4
  %_67.i.i.i.i282 = load float, ptr %row7.i.i.i117.i, align 4
  %_70.i.i.i.i285 = load float, ptr %561, align 4
  %_73.i.i.i.i288 = load float, ptr %562, align 4
  %_76.i.i.i.i291 = load float, ptr %563, align 4
  %_81.i.i.i.i296 = load float, ptr %row9.i.i.i131.i, align 4
  %_84.i.i.i.i299 = load float, ptr %564, align 4
  %_87.i.i.i.i302 = load float, ptr %565, align 4
  %_90.i.i.i.i305 = load float, ptr %566, align 4
  %_95.i.i.i.i310 = load float, ptr %row11.i.i.i145.i, align 4
  %_98.i.i.i.i313 = load float, ptr %567, align 4
  %_101.i.i.i.i316 = load float, ptr %568, align 4
  %_104.i.i.i.i319 = load float, ptr %569, align 4
  %_109.i.i.i.i324 = load float, ptr %row13.i.i.i159.i, align 4
  %_112.i.i.i.i327 = load float, ptr %570, align 4
  %_115.i.i.i.i330 = load float, ptr %571, align 4
  %_118.i.i.i.i333 = load float, ptr %572, align 4
  %_123.i.i.i.i338 = load float, ptr %row15.i.i.i173.i, align 4
  %_126.i.i.i.i341 = load float, ptr %573, align 4
  %_129.i.i.i.i344 = load float, ptr %574, align 4
  %_132.i.i.i.i347 = load float, ptr %575, align 4
  %_137.i.i.i.i352 = load float, ptr %row17.i.i.i187.i, align 4
  %_140.i.i.i.i355 = load float, ptr %576, align 4
  %_143.i.i.i.i358 = load float, ptr %577, align 4
  %_146.i.i.i.i361 = load float, ptr %578, align 4
  %_151.i.i.i.i366 = load float, ptr %row19.i.i.i201.i, align 4
  %_154.i.i.i.i369 = load float, ptr %579, align 4
  %_157.i.i.i.i372 = load float, ptr %580, align 4
  %_160.i.i.i.i375 = load float, ptr %581, align 4
  %_165.i.i.i.i380 = load float, ptr %row21.i.i.i215.i, align 4
  %_168.i.i.i.i383 = load float, ptr %582, align 4
  %_171.i.i.i.i386 = load float, ptr %583, align 4
  %_174.i.i.i.i389 = load float, ptr %584, align 4
  br label %bb5.i.i207, !dbg !39399

bb5.i.i207:                                       ; preds = %bb5.i.i207.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050
  %iter.sroa.0.0.i.i657837 = phi i64 [ 0, %bb5.i.i207.lr.ph ], [ %623, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050 ]
  %history.i.i13.sroa.35.07836 = phi float [ %history.i.i13.sroa.35.0.copyload, %bb5.i.i207.lr.ph ], [ %history.i.i13.sroa.32.07835, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050 ]
  %history.i.i13.sroa.32.07835 = phi float [ %history.i.i13.sroa.32.0.copyload, %bb5.i.i207.lr.ph ], [ %history.i.i13.sroa.29.07834, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050 ]
  %history.i.i13.sroa.29.07834 = phi float [ %history.i.i13.sroa.29.0.copyload, %bb5.i.i207.lr.ph ], [ %history.i.i13.sroa.26.07833, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050 ]
  %history.i.i13.sroa.26.07833 = phi float [ %history.i.i13.sroa.26.0.copyload, %bb5.i.i207.lr.ph ], [ %history.i.i13.sroa.22.07832, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050 ]
  %history.i.i13.sroa.22.07832 = phi float [ %history.i.i13.sroa.22.0.copyload, %bb5.i.i207.lr.ph ], [ %history.i.i13.sroa.19.07831, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050 ]
  %history.i.i13.sroa.19.07831 = phi float [ %history.i.i13.sroa.19.0.copyload, %bb5.i.i207.lr.ph ], [ %history.i.i13.sroa.16.07830, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050 ]
  %history.i.i13.sroa.16.07830 = phi float [ %history.i.i13.sroa.16.0.copyload, %bb5.i.i207.lr.ph ], [ %history.i.i13.sroa.13.07829, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050 ]
  %history.i.i13.sroa.13.07829 = phi float [ %history.i.i13.sroa.13.0.copyload, %bb5.i.i207.lr.ph ], [ %history.i.i13.sroa.10.07828, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050 ]
  %history.i.i13.sroa.10.07828 = phi float [ %history.i.i13.sroa.10.0.copyload, %bb5.i.i207.lr.ph ], [ %history.i.i13.sroa.7.07827, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050 ]
  %history.i.i13.sroa.7.07827 = phi float [ %history.i.i13.sroa.7.0.copyload, %bb5.i.i207.lr.ph ], [ %history.i.i13.sroa.0.07826, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050 ]
  %history.i.i13.sroa.0.07826 = phi float [ %history.i.i13.sroa.0.0.copyload, %bb5.i.i207.lr.ph ], [ %_0.i3048, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050 ]
  %623 = add nuw nsw i64 %iter.sroa.0.0.i.i657837, 1, !dbg !39402
  %_11.i30.i = add nuw nsw i64 %iter.sroa.0.0.i.i657837, %iter1.sroa.0.0.i578414, !dbg !39405
  %_24.i.i208 = icmp ugt i64 %_11.i30.i, %right_io.1, !dbg !39406
  br i1 %_24.i.i208, label %bb7.i.i405, label %bb8.i.i209, !dbg !39406, !prof !639

bb8.i.i209:                                       ; preds = %bb5.i.i207
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39409), !dbg !39412
  %_3.not.i3046 = icmp eq i64 %right_io.1, %_11.i30.i, !dbg !39413
  br i1 %_3.not.i3046, label %panic.i3049, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050, !dbg !39413

panic.i3049:                                      ; preds = %bb8.i.i209
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !39413, !noalias !39415
  unreachable, !dbg !39413

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050: ; preds = %bb8.i.i209
  %_31.i.i211 = getelementptr inbounds nuw float, ptr %right_io.0, i64 %_11.i30.i, !dbg !39417
  %_0.i3048 = load float, ptr %_31.i.i211, align 4, !dbg !39413, !alias.scope !39409, !noalias !39419, !noundef !12
  %624 = tail call noundef float @llvm.fabs.f32(float %history.i.i13.sroa.19.07831), !dbg !39420
  %_0.i2751 = fmul float %_0.i3048, %_11.i.i.i.i226, !dbg !39423
  %_0.i2319 = fadd float %_0.i2751, 0.000000e+00, !dbg !39426
  %_0.i2750 = fmul float %_0.i3048, %_14.i.i.i.i229, !dbg !39428
  %_0.i2318 = fadd float %_0.i2750, 0.000000e+00, !dbg !39430
  %_0.i2749 = fmul float %_0.i3048, %_17.i.i.i.i232, !dbg !39432
  %_0.i2317 = fadd float %_0.i2749, 0.000000e+00, !dbg !39434
  %_0.i2748 = fmul float %_0.i3048, %_20.i.i.i.i235, !dbg !39436
  %_0.i2316 = fadd float %_0.i2748, 0.000000e+00, !dbg !39438
  %_0.i2747 = fmul float %history.i.i13.sroa.0.07826, %_25.i.i.i.i240, !dbg !39440
  %_0.i2315 = fadd float %_0.i2319, %_0.i2747, !dbg !39442
  %_0.i2746 = fmul float %history.i.i13.sroa.0.07826, %_28.i.i.i.i243, !dbg !39444
  %_0.i2314 = fadd float %_0.i2318, %_0.i2746, !dbg !39446
  %_0.i2745 = fmul float %history.i.i13.sroa.0.07826, %_31.i.i.i.i246, !dbg !39448
  %_0.i2313 = fadd float %_0.i2317, %_0.i2745, !dbg !39450
  %_0.i2744 = fmul float %history.i.i13.sroa.0.07826, %_34.i.i.i.i249, !dbg !39452
  %_0.i2312 = fadd float %_0.i2316, %_0.i2744, !dbg !39454
  %_0.i2743 = fmul float %history.i.i13.sroa.7.07827, %_39.i.i.i.i254, !dbg !39456
  %_0.i2311 = fadd float %_0.i2315, %_0.i2743, !dbg !39458
  %_0.i2742 = fmul float %history.i.i13.sroa.7.07827, %_42.i.i.i.i257, !dbg !39460
  %_0.i2310 = fadd float %_0.i2314, %_0.i2742, !dbg !39462
  %_0.i2741 = fmul float %history.i.i13.sroa.7.07827, %_45.i.i.i.i260, !dbg !39464
  %_0.i2309 = fadd float %_0.i2313, %_0.i2741, !dbg !39466
  %_0.i2740 = fmul float %history.i.i13.sroa.7.07827, %_48.i.i.i.i263, !dbg !39468
  %_0.i2308 = fadd float %_0.i2312, %_0.i2740, !dbg !39470
  %_0.i2739 = fmul float %history.i.i13.sroa.10.07828, %_53.i.i.i.i268, !dbg !39472
  %_0.i2307 = fadd float %_0.i2311, %_0.i2739, !dbg !39474
  %_0.i2738 = fmul float %history.i.i13.sroa.10.07828, %_56.i.i.i.i271, !dbg !39476
  %_0.i2306 = fadd float %_0.i2310, %_0.i2738, !dbg !39478
  %_0.i2737 = fmul float %history.i.i13.sroa.10.07828, %_59.i.i.i.i274, !dbg !39480
  %_0.i2305 = fadd float %_0.i2309, %_0.i2737, !dbg !39482
  %_0.i2736 = fmul float %history.i.i13.sroa.10.07828, %_62.i.i.i.i277, !dbg !39484
  %_0.i2304 = fadd float %_0.i2308, %_0.i2736, !dbg !39486
  %_0.i2735 = fmul float %history.i.i13.sroa.13.07829, %_67.i.i.i.i282, !dbg !39488
  %_0.i2303 = fadd float %_0.i2307, %_0.i2735, !dbg !39490
  %_0.i2734 = fmul float %history.i.i13.sroa.13.07829, %_70.i.i.i.i285, !dbg !39492
  %_0.i2302 = fadd float %_0.i2306, %_0.i2734, !dbg !39494
  %_0.i2733 = fmul float %history.i.i13.sroa.13.07829, %_73.i.i.i.i288, !dbg !39496
  %_0.i2301 = fadd float %_0.i2305, %_0.i2733, !dbg !39498
  %_0.i2732 = fmul float %history.i.i13.sroa.13.07829, %_76.i.i.i.i291, !dbg !39500
  %_0.i2300 = fadd float %_0.i2304, %_0.i2732, !dbg !39502
  %_0.i2731 = fmul float %history.i.i13.sroa.16.07830, %_81.i.i.i.i296, !dbg !39504
  %_0.i2299 = fadd float %_0.i2303, %_0.i2731, !dbg !39506
  %_0.i2730 = fmul float %history.i.i13.sroa.16.07830, %_84.i.i.i.i299, !dbg !39508
  %_0.i2298 = fadd float %_0.i2302, %_0.i2730, !dbg !39510
  %_0.i2729 = fmul float %history.i.i13.sroa.16.07830, %_87.i.i.i.i302, !dbg !39512
  %_0.i2297 = fadd float %_0.i2301, %_0.i2729, !dbg !39514
  %_0.i2728 = fmul float %history.i.i13.sroa.16.07830, %_90.i.i.i.i305, !dbg !39516
  %_0.i2296 = fadd float %_0.i2300, %_0.i2728, !dbg !39518
  %_0.i2727 = fmul float %history.i.i13.sroa.19.07831, %_95.i.i.i.i310, !dbg !39520
  %_0.i2295 = fadd float %_0.i2299, %_0.i2727, !dbg !39522
  %_0.i2726 = fmul float %history.i.i13.sroa.19.07831, %_98.i.i.i.i313, !dbg !39524
  %_0.i2294 = fadd float %_0.i2298, %_0.i2726, !dbg !39526
  %_0.i2725 = fmul float %history.i.i13.sroa.19.07831, %_101.i.i.i.i316, !dbg !39528
  %_0.i2293 = fadd float %_0.i2297, %_0.i2725, !dbg !39530
  %_0.i2724 = fmul float %history.i.i13.sroa.19.07831, %_104.i.i.i.i319, !dbg !39532
  %_0.i2292 = fadd float %_0.i2296, %_0.i2724, !dbg !39534
  %_0.i2723 = fmul float %history.i.i13.sroa.22.07832, %_109.i.i.i.i324, !dbg !39536
  %_0.i2291 = fadd float %_0.i2295, %_0.i2723, !dbg !39538
  %_0.i2722 = fmul float %history.i.i13.sroa.22.07832, %_112.i.i.i.i327, !dbg !39540
  %_0.i2290 = fadd float %_0.i2294, %_0.i2722, !dbg !39542
  %_0.i2721 = fmul float %history.i.i13.sroa.22.07832, %_115.i.i.i.i330, !dbg !39544
  %_0.i2289 = fadd float %_0.i2293, %_0.i2721, !dbg !39546
  %_0.i2720 = fmul float %history.i.i13.sroa.22.07832, %_118.i.i.i.i333, !dbg !39548
  %_0.i2288 = fadd float %_0.i2292, %_0.i2720, !dbg !39550
  %_0.i2719 = fmul float %history.i.i13.sroa.26.07833, %_123.i.i.i.i338, !dbg !39552
  %_0.i2287 = fadd float %_0.i2291, %_0.i2719, !dbg !39554
  %_0.i2718 = fmul float %history.i.i13.sroa.26.07833, %_126.i.i.i.i341, !dbg !39556
  %_0.i2286 = fadd float %_0.i2290, %_0.i2718, !dbg !39558
  %_0.i2717 = fmul float %history.i.i13.sroa.26.07833, %_129.i.i.i.i344, !dbg !39560
  %_0.i2285 = fadd float %_0.i2289, %_0.i2717, !dbg !39562
  %_0.i2716 = fmul float %history.i.i13.sroa.26.07833, %_132.i.i.i.i347, !dbg !39564
  %_0.i2284 = fadd float %_0.i2288, %_0.i2716, !dbg !39566
  %_0.i2715 = fmul float %history.i.i13.sroa.29.07834, %_137.i.i.i.i352, !dbg !39568
  %_0.i2283 = fadd float %_0.i2287, %_0.i2715, !dbg !39570
  %_0.i2714 = fmul float %history.i.i13.sroa.29.07834, %_140.i.i.i.i355, !dbg !39572
  %_0.i2282 = fadd float %_0.i2286, %_0.i2714, !dbg !39574
  %_0.i2713 = fmul float %history.i.i13.sroa.29.07834, %_143.i.i.i.i358, !dbg !39576
  %_0.i2281 = fadd float %_0.i2285, %_0.i2713, !dbg !39578
  %_0.i2712 = fmul float %history.i.i13.sroa.29.07834, %_146.i.i.i.i361, !dbg !39580
  %_0.i2280 = fadd float %_0.i2284, %_0.i2712, !dbg !39582
  %_0.i2711 = fmul float %history.i.i13.sroa.32.07835, %_151.i.i.i.i366, !dbg !39584
  %_0.i2279 = fadd float %_0.i2283, %_0.i2711, !dbg !39586
  %_0.i2710 = fmul float %history.i.i13.sroa.32.07835, %_154.i.i.i.i369, !dbg !39588
  %_0.i2278 = fadd float %_0.i2282, %_0.i2710, !dbg !39590
  %_0.i2709 = fmul float %history.i.i13.sroa.32.07835, %_157.i.i.i.i372, !dbg !39592
  %_0.i2277 = fadd float %_0.i2281, %_0.i2709, !dbg !39594
  %_0.i2708 = fmul float %history.i.i13.sroa.32.07835, %_160.i.i.i.i375, !dbg !39596
  %_0.i2276 = fadd float %_0.i2280, %_0.i2708, !dbg !39598
  %_0.i2707 = fmul float %history.i.i13.sroa.35.07836, %_165.i.i.i.i380, !dbg !39600
  %_0.i2275 = fadd float %_0.i2279, %_0.i2707, !dbg !39602
  %_0.i2706 = fmul float %history.i.i13.sroa.35.07836, %_168.i.i.i.i383, !dbg !39604
  %_0.i2274 = fadd float %_0.i2278, %_0.i2706, !dbg !39606
  %_0.i2705 = fmul float %history.i.i13.sroa.35.07836, %_171.i.i.i.i386, !dbg !39608
  %_0.i2273 = fadd float %_0.i2277, %_0.i2705, !dbg !39610
  %_0.i2704 = fmul float %history.i.i13.sroa.35.07836, %_174.i.i.i.i389, !dbg !39612
  %_0.i2272 = fadd float %_0.i2276, %_0.i2704, !dbg !39614
  %625 = tail call noundef float @llvm.fabs.f32(float %_0.i2275), !dbg !39616
  %_3.i.i3684.inv = fcmp ogt float %624, %625, !dbg !39618
  %_4.i.i3691.v = select i1 %_3.i.i3684.inv, float %624, float %625, !dbg !39618
  %626 = tail call noundef float @llvm.fabs.f32(float %_0.i2274), !dbg !39616
  %_3.i.i3684.inv.1 = fcmp ogt float %_4.i.i3691.v, %626, !dbg !39618
  %_4.i.i3691.v.1 = select i1 %_3.i.i3684.inv.1, float %_4.i.i3691.v, float %626, !dbg !39618
  %627 = tail call noundef float @llvm.fabs.f32(float %_0.i2273), !dbg !39616
  %_3.i.i3684.inv.2 = fcmp ogt float %_4.i.i3691.v.1, %627, !dbg !39618
  %_4.i.i3691.v.2 = select i1 %_3.i.i3684.inv.2, float %_4.i.i3691.v.1, float %627, !dbg !39618
  %628 = tail call noundef float @llvm.fabs.f32(float %_0.i2272), !dbg !39616
  %_3.i.i3684.inv.3 = fcmp ogt float %_4.i.i3691.v.2, %628, !dbg !39618
  %_4.i.i3691.v.3 = select i1 %_3.i.i3684.inv.3, float %_4.i.i3691.v.2, float %628, !dbg !39618
  %_39.i.i400 = getelementptr inbounds nuw float, ptr %peaks_right.i34, i64 %iter.sroa.0.0.i.i657837, !dbg !39621
  store float %_4.i.i3691.v.3, ptr %_39.i.i400, align 4, !dbg !39626, !alias.scope !39628, !noalias !39419
  %exitcond11129.not = icmp eq i64 %623, %umax11128, !dbg !39631
  br i1 %exitcond11129.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i67, label %bb5.i.i207, !dbg !39399

bb7.i.i405:                                       ; preds = %bb5.i.i207
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_11.i30.i, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_283c8d9f84e10f29a75d8e6c3a347bea) #30, !dbg !39633, !noalias !39419
  unreachable, !dbg !39633

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i67: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit243.i
  %history.i.i13.sroa.0.0.lcssa = phi float [ %history.i.i13.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %_0.i3048, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050 ], !dbg !39634
  %history.i.i13.sroa.7.0.lcssa = phi float [ %history.i.i13.sroa.7.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i13.sroa.0.07826, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050 ], !dbg !39634
  %history.i.i13.sroa.10.0.lcssa = phi float [ %history.i.i13.sroa.10.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i13.sroa.7.07827, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050 ], !dbg !39634
  %history.i.i13.sroa.13.0.lcssa = phi float [ %history.i.i13.sroa.13.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i13.sroa.10.07828, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050 ], !dbg !39634
  %history.i.i13.sroa.16.0.lcssa = phi float [ %history.i.i13.sroa.16.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i13.sroa.13.07829, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050 ], !dbg !39634
  %history.i.i13.sroa.19.0.lcssa = phi float [ %history.i.i13.sroa.19.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i13.sroa.16.07830, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050 ], !dbg !39634
  %history.i.i13.sroa.22.0.lcssa = phi float [ %history.i.i13.sroa.22.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i13.sroa.19.07831, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050 ], !dbg !39634
  %history.i.i13.sroa.26.0.lcssa = phi float [ %history.i.i13.sroa.26.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i13.sroa.22.07832, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050 ], !dbg !39634
  %history.i.i13.sroa.29.0.lcssa = phi float [ %history.i.i13.sroa.29.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i13.sroa.26.07833, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050 ], !dbg !39634
  %history.i.i13.sroa.32.0.lcssa = phi float [ %history.i.i13.sroa.32.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i13.sroa.29.07834, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050 ], !dbg !39634
  %history.i.i13.sroa.35.0.lcssa = phi float [ %history.i.i13.sroa.35.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i13.sroa.32.07835, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050 ], !dbg !39634
  %history.i.i13.sroa.38.0.lcssa = phi float [ %history.i.i13.sroa.38.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i13.sroa.35.07836, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3050 ], !dbg !39634
  store float %history.i.i13.sroa.0.0.lcssa, ptr %hot_right.i36, align 4, !dbg !39635, !noalias !39394
  store float %history.i.i13.sroa.7.0.lcssa, ptr %history.i.i13.sroa.7.0.hot_right.i36.sroa_idx, align 4, !dbg !39635, !noalias !39394
  store float %history.i.i13.sroa.10.0.lcssa, ptr %history.i.i13.sroa.10.0.hot_right.i36.sroa_idx, align 4, !dbg !39635, !noalias !39394
  store float %history.i.i13.sroa.13.0.lcssa, ptr %history.i.i13.sroa.13.0.hot_right.i36.sroa_idx, align 4, !dbg !39635, !noalias !39394
  store float %history.i.i13.sroa.16.0.lcssa, ptr %history.i.i13.sroa.16.0.hot_right.i36.sroa_idx, align 4, !dbg !39635, !noalias !39394
  store float %history.i.i13.sroa.19.0.lcssa, ptr %history.i.i13.sroa.19.0.hot_right.i36.sroa_idx, align 4, !dbg !39635, !noalias !39394
  store float %history.i.i13.sroa.22.0.lcssa, ptr %history.i.i13.sroa.22.0.hot_right.i36.sroa_idx, align 4, !dbg !39635, !noalias !39394
  store float %history.i.i13.sroa.26.0.lcssa, ptr %history.i.i13.sroa.26.0.hot_right.i36.sroa_idx, align 4, !dbg !39635, !noalias !39394
  store float %history.i.i13.sroa.29.0.lcssa, ptr %history.i.i13.sroa.29.0.hot_right.i36.sroa_idx, align 4, !dbg !39635, !noalias !39394
  store float %history.i.i13.sroa.32.0.lcssa, ptr %history.i.i13.sroa.32.0.hot_right.i36.sroa_idx, align 4, !dbg !39635, !noalias !39394
  store float %history.i.i13.sroa.35.0.lcssa, ptr %history.i.i13.sroa.35.0.hot_right.i36.sroa_idx, align 4, !dbg !39635, !noalias !39394
  store float %history.i.i13.sroa.38.0.lcssa, ptr %history.i.i13.sroa.38.0.hot_right.i36.sroa_idx, align 4, !dbg !39635, !noalias !39394
  br i1 %_20.i42.i7798.not, label %bb15.i54.loopexit, label %bb20.i73.lr.ph, !dbg !39135

bb20.i73.lr.ph:                                   ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i67
  %_65.i29.sroa.3.0.copyload = load i64, ptr %_65.i29.sroa.3.0..sroa_idx, align 8, !noalias !39046
  %_65.i29.sroa.4.0.copyload = load i64, ptr %_65.i29.sroa.4.0..sroa_idx, align 8, !noalias !39046
  %_66.i28.sroa.3.0.copyload = load i64, ptr %_66.i28.sroa.3.0..sroa_idx, align 8, !noalias !39046
  %_66.i28.sroa.4.0.copyload = load i64, ptr %_66.i28.sroa.4.0..sroa_idx, align 8, !noalias !39046
  %_54.0.i256.i = load ptr, ptr %uniform_left.i33, align 8, !nonnull !12, !align !24
  %_54.1.i257.i = load i64, ptr %599, align 8
  %_18.i267.i = load i64, ptr %585, align 8
  %_56.0.i278.i = load ptr, ptr %600, align 8, !nonnull !12, !align !24
  %_56.1.i279.i = load i64, ptr %601, align 8
  %_58.1.i304.i = load i64, ptr %605, align 8
  %_58.0.i303.i = load ptr, ptr %606, align 8, !nonnull !12, !align !24
  %_54.0.i.i142 = load ptr, ptr %uniform_right.i32, align 8, !nonnull !12, !align !24
  %_54.1.i.i143 = load i64, ptr %607, align 8
  %_18.i.i153 = load i64, ptr %586, align 8
  %_56.0.i.i160 = load ptr, ptr %608, align 8, !nonnull !12, !align !24
  %_56.1.i.i161 = load i64, ptr %609, align 8
  %_58.1.i.i184 = load i64, ptr %613, align 8
  %_58.0.i.i183 = load ptr, ptr %614, align 8, !nonnull !12, !align !24
  %_22.i271.i.promoted8333 = load i32, ptr %_22.i271.i, align 4
  %_21.i270.i.promoted8369 = load float, ptr %_21.i270.i, align 4
  %_22.i.i155.promoted8372 = load i32, ptr %_22.i.i155, align 4
  %_21.i.i154.promoted8408 = load float, ptr %_21.i.i154, align 4
  %umax11130 = call i64 @llvm.umax.i64(i64 %_18.i267.i, i64 1), !dbg !39135
  %umax11132 = call i64 @llvm.umax.i64(i64 %_18.i.i153, i64 1), !dbg !39135
  %_13.i197047804782 = load float, ptr %589, align 4
  %_13.i195847834785 = load float, ptr %592, align 4
  %_13.i194647864788 = load float, ptr %595, align 4
  %_13.i193447894791 = load float, ptr %598, align 4
  %_37.i291.i = load float, ptr %603, align 4
  %_37.i.i171 = load float, ptr %611, align 4
  %.promoted13363 = load float, ptr %587, align 4
  %_109.i106.promoted13381 = load float, ptr %_109.i106, align 4
  %.promoted13384 = load float, ptr %588, align 4
  %.promoted13387 = load float, ptr %590, align 4
  %_110.i107.promoted13390 = load float, ptr %_110.i107, align 4
  %.promoted13393 = load float, ptr %591, align 4
  %.promoted13396 = load float, ptr %593, align 4
  %_114.i108.promoted13414 = load float, ptr %_114.i108, align 4
  %.promoted13417 = load float, ptr %594, align 4
  %.promoted13420 = load float, ptr %596, align 4
  %_115.i109.promoted13423 = load float, ptr %_115.i109, align 4
  %.promoted13426 = load float, ptr %597, align 4
  %.promoted13429 = load float, ptr %602, align 4
  %.promoted13432 = load float, ptr %604, align 4
  %.promoted13435 = load float, ptr %610, align 4
  %.promoted13438 = load float, ptr %612, align 4
  %_21.i270.i.promoted = load float, ptr %_21.i270.i, align 1
  %_21.i.i154.promoted = load float, ptr %_21.i.i154, align 1
  br label %bb20.i73, !dbg !39135

bb20.i73:                                         ; preds = %bb20.i73.lr.ph, %bb32.i198
  %running.sroa.0.0.i.lcssa1192813459 = phi float [ %_21.i.i154.promoted, %bb20.i73.lr.ph ], [ %running.sroa.0.0.i.lcssa1192813458, %bb32.i198 ]
  %running.sroa.0.0.i1179.lcssa1189113442 = phi float [ %_21.i270.i.promoted, %bb20.i73.lr.ph ], [ %running.sroa.0.0.i1179.lcssa1189113441, %bb32.i198 ]
  %_0.i3261.lcssa1336213440 = phi float [ %.promoted13438, %bb20.i73.lr.ph ], [ %_0.i3261.lcssa1336213439, %bb32.i198 ]
  %_0.i2887.lcssa1334713437 = phi float [ %.promoted13435, %bb20.i73.lr.ph ], [ %_0.i2887.lcssa1334713436, %bb32.i198 ]
  %_0.i3265.lcssa1333213434 = phi float [ %.promoted13432, %bb20.i73.lr.ph ], [ %_0.i3265.lcssa1333213433, %bb32.i198 ]
  %_0.i2891.lcssa1331713431 = phi float [ %.promoted13429, %bb20.i73.lr.ph ], [ %_0.i2891.lcssa1331713430, %bb32.i198 ]
  %_0.i3316.lcssa1172513428 = phi float [ %.promoted13426, %bb20.i73.lr.ph ], [ %_0.i3316.lcssa1172513427, %bb32.i198 ]
  %_0.i3323.lcssa1173913425 = phi float [ %_115.i109.promoted13423, %bb20.i73.lr.ph ], [ %_0.i3323.lcssa1173913424, %bb32.i198 ]
  %_0.i.i3557.lcssa1175313422 = phi float [ %.promoted13420, %bb20.i73.lr.ph ], [ %_0.i.i3557.lcssa1175313421, %bb32.i198 ]
  %_0.i3303.lcssa1176713419 = phi float [ %.promoted13417, %bb20.i73.lr.ph ], [ %_0.i3303.lcssa1176713418, %bb32.i198 ]
  %_0.i3310.lcssa1178113416 = phi float [ %_114.i108.promoted13414, %bb20.i73.lr.ph ], [ %_0.i3310.lcssa1178113415, %bb32.i198 ]
  %_0.i.i3550.lcssa1179513398 = phi float [ %.promoted13396, %bb20.i73.lr.ph ], [ %_0.i.i3550.lcssa1179513397, %bb32.i198 ]
  %_0.i3290.lcssa1180913395 = phi float [ %.promoted13393, %bb20.i73.lr.ph ], [ %_0.i3290.lcssa1180913394, %bb32.i198 ]
  %_0.i3297.lcssa1182313392 = phi float [ %_110.i107.promoted13390, %bb20.i73.lr.ph ], [ %_0.i3297.lcssa1182313391, %bb32.i198 ]
  %_0.i.i3543.lcssa1183713389 = phi float [ %.promoted13387, %bb20.i73.lr.ph ], [ %_0.i.i3543.lcssa1183713388, %bb32.i198 ]
  %_0.i3278.lcssa1185113386 = phi float [ %.promoted13384, %bb20.i73.lr.ph ], [ %_0.i3278.lcssa1185113385, %bb32.i198 ]
  %_0.i3284.lcssa1186513383 = phi float [ %_109.i106.promoted13381, %bb20.i73.lr.ph ], [ %_0.i3284.lcssa1186513382, %bb32.i198 ]
  %_0.i.i.lcssa1187913365 = phi float [ %.promoted13363, %bb20.i73.lr.ph ], [ %_0.i.i.lcssa1187913364, %bb32.i198 ]
  %running.sroa.0.0.i.lcssa83188410 = phi float [ %_21.i.i154.promoted8408, %bb20.i73.lr.ph ], [ %running.sroa.0.0.i.lcssa83188409, %bb32.i198 ]
  %storemerge.i.lcssa82918374 = phi i32 [ %_22.i.i155.promoted8372, %bb20.i73.lr.ph ], [ %storemerge.i.lcssa82918373, %bb32.i198 ]
  %running.sroa.0.0.i1179.lcssa82608371 = phi float [ %_21.i270.i.promoted8369, %bb20.i73.lr.ph ], [ %running.sroa.0.0.i1179.lcssa82608370, %bb32.i198 ]
  %storemerge.i1184.lcssa82338335 = phi i32 [ %_22.i271.i.promoted8333, %bb20.i73.lr.ph ], [ %storemerge.i1184.lcssa82338334, %bb32.i198 ]
  %frame.sroa.0.0.i718330 = phi i64 [ 0, %bb20.i73.lr.ph ], [ %_80.i86, %bb32.i198 ]
  %main_cursor.sroa.0.1.i708329 = phi i64 [ %main_cursor.sroa.0.0.i568413, %bb20.i73.lr.ph ], [ %main_cursor.sroa.0.2.i204, %bb32.i198 ]
  %ring_cursor.sroa.0.1.i698328 = phi i64 [ %ring_cursor.sroa.0.0.i558412, %bb20.i73.lr.ph ], [ %ring_cursor.sroa.0.2.i201, %bb32.i198 ]
  %_63.i74 = sub nuw nsw i64 %..i4168, %frame.sroa.0.0.i718330, !dbg !39636
  %ring.i1821 = load i64, ptr %543, align 8, !dbg !39637, !alias.scope !39639, !noalias !39642, !noundef !12
  %main.i1822 = load i64, ptr %544, align 8, !dbg !39646, !alias.scope !39639, !noalias !39642, !noundef !12
  %_10.i = add i64 %ring_cursor.sroa.0.1.i698328, 1, !dbg !39647
  %_45.not.i = icmp ult i64 %_10.i, %ring.i1821, !dbg !39648
  %629 = select i1 %_45.not.i, i64 0, i64 %ring.i1821, !dbg !39648
  %start1.sroa.0.0.i1823 = sub nuw i64 %_10.i, %629, !dbg !39648
  %_12.i1824 = add i64 %_65.i29.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i698328, !dbg !39650
  %_46.not.i = icmp ult i64 %_12.i1824, %ring.i1821, !dbg !39651
  %630 = select i1 %_46.not.i, i64 0, i64 %ring.i1821, !dbg !39651
  %left_end.sroa.0.0.i = sub nuw i64 %_12.i1824, %630, !dbg !39651
  %_15.i1826 = add i64 %_66.i28.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i698328, !dbg !39653
  %_47.not.i = icmp ult i64 %_15.i1826, %ring.i1821, !dbg !39654
  %631 = select i1 %_47.not.i, i64 0, i64 %ring.i1821, !dbg !39654
  %right_end.sroa.0.0.i = sub nuw i64 %_15.i1826, %631, !dbg !39654
  %_18.i = add i64 %_65.i29.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i698328, !dbg !39656
  %_48.not.i = icmp ult i64 %_18.i, %ring.i1821, !dbg !39657
  %632 = select i1 %_48.not.i, i64 0, i64 %ring.i1821, !dbg !39657
  %left_expiring.sroa.0.0.i = sub nuw i64 %_18.i, %632, !dbg !39657
  %_21.i = add i64 %_66.i28.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i698328, !dbg !39659
  %_49.not.i = icmp ult i64 %_21.i, %ring.i1821, !dbg !39660
  %633 = select i1 %_49.not.i, i64 0, i64 %ring.i1821, !dbg !39660
  %right_expiring.sroa.0.0.i = sub nuw i64 %_21.i, %633, !dbg !39660
  %_30.i1828 = sub i64 %ring.i1821, %ring_cursor.sroa.0.1.i698328, !dbg !39662
  %..i4193 = tail call noundef i64 @llvm.umin.i64(i64 %_30.i1828, i64 %_63.i74), !dbg !39663
  %_31.i1830 = sub i64 %main.i1822, %main_cursor.sroa.0.1.i708329, !dbg !39665
  %..i4194 = tail call noundef i64 @llvm.umin.i64(i64 %_31.i1830, i64 %..i4193), !dbg !39666
  %_32.i1832 = sub i64 %ring.i1821, %start1.sroa.0.0.i1823, !dbg !39668
  %..i4195 = tail call noundef i64 @llvm.umin.i64(i64 %_32.i1832, i64 %..i4194), !dbg !39669
  %_34.i1834 = sub i64 %ring.i1821, %left_end.sroa.0.0.i, !dbg !39671
  %..i4196 = tail call noundef i64 @llvm.umin.i64(i64 %_34.i1834, i64 %..i4195), !dbg !39672
  %_36.i1836 = sub i64 %ring.i1821, %right_end.sroa.0.0.i, !dbg !39674
  %..i4197 = tail call noundef i64 @llvm.umin.i64(i64 %_36.i1836, i64 %..i4196), !dbg !39675
  %_38.i = sub i64 %ring.i1821, %left_expiring.sroa.0.0.i, !dbg !39677
  %..i4198 = tail call noundef i64 @llvm.umin.i64(i64 %_38.i, i64 %..i4197), !dbg !39678
  %_40.i1837 = sub i64 %ring.i1821, %right_expiring.sroa.0.0.i, !dbg !39680
  %..i4199 = tail call noundef i64 @llvm.umin.i64(i64 %_40.i1837, i64 %..i4198), !dbg !39681
  %_69.i76 = add i64 %frame.sroa.0.0.i718330, %iter1.sroa.0.0.i578414, !dbg !39683
  %_73.i77 = add i64 %..i4199, %_69.i76, !dbg !39684
  %_174.i78 = icmp ult i64 %_73.i77, %_69.i76, !dbg !39685
  %_168.not.i79 = icmp ugt i64 %_73.i77, %left_io.1
  %or.cond.i80 = or i1 %_174.i78, %_168.not.i79, !dbg !39685
  br i1 %or.cond.i80, label %bb55.i206, label %bb53.i81, !dbg !39685, !prof !165

bb55.i206:                                        ; preds = %bb20.i73
  store float %_0.i.i.lcssa1187913365, ptr %587, align 4
  store float %_0.i.i3550.lcssa1179513398, ptr %593, align 4
  store float %running.sroa.0.0.i1179.lcssa1189113442, ptr %_21.i270.i, align 1, !dbg !39101
  store float %running.sroa.0.0.i.lcssa1192813459, ptr %_21.i.i154, align 1, !dbg !39132
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_69.i76, i64 noundef %_73.i77, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_efa26be4ae64153d8eb817c4a24fadb5) #30, !dbg !39692, !noalias !39040
  unreachable, !dbg !39692

bb53.i81:                                         ; preds = %bb20.i73
  %_177.i82 = getelementptr inbounds nuw float, ptr %left_io.0, i64 %_69.i76, !dbg !39693
  %_178.not.i83 = icmp ugt i64 %_73.i77, %right_io.1, !dbg !39697
  br i1 %_178.not.i83, label %bb58.i205, label %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit, !dbg !39697, !prof !639

bb58.i205:                                        ; preds = %bb53.i81
  store float %_0.i.i.lcssa1187913365, ptr %587, align 4
  store float %_0.i.i3550.lcssa1179513398, ptr %593, align 4
  store float %running.sroa.0.0.i1179.lcssa1189113442, ptr %_21.i270.i, align 1, !dbg !39101
  store float %running.sroa.0.0.i.lcssa1192813459, ptr %_21.i.i154, align 1, !dbg !39132
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_69.i76, i64 noundef %_73.i77, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_fcc53ff99bd917af3a52e0ac8a8cbd22) #30, !dbg !39701, !noalias !39040
  unreachable, !dbg !39701

_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit: ; preds = %bb53.i81
  %_185.i85 = getelementptr inbounds nuw float, ptr %right_io.0, i64 %_69.i76, !dbg !39702
  %_80.i86 = add nuw nsw i64 %..i4199, %frame.sroa.0.0.i718330, !dbg !39706
  %_194.i92 = getelementptr inbounds nuw float, ptr %peaks_left.i35, i64 %frame.sroa.0.0.i718330, !dbg !39707
  %_203.i93 = getelementptr inbounds nuw float, ptr %peaks_right.i34, i64 %frame.sroa.0.0.i718330, !dbg !39717
  %_2.i.i.i7856.not = icmp eq i64 %..i4199, 0, !dbg !39726
  br i1 %_2.i.i.i7856.not, label %bb32.i198, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055.lr.ph, !dbg !39726

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055.lr.ph: ; preds = %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit
  %umin11136 = call i64 @llvm.umin.i64(i64 %_34.i1834, i64 %_36.i1836), !dbg !39726
  %umin11137 = call i64 @llvm.umin.i64(i64 %umin11136, i64 %_38.i), !dbg !39726
  %umin11138 = call i64 @llvm.umin.i64(i64 %umin11137, i64 %_40.i1837), !dbg !39726
  %umin11139 = call i64 @llvm.umin.i64(i64 %umin11138, i64 %_32.i1832), !dbg !39726
  %umin11140 = call i64 @llvm.umin.i64(i64 %umin11139, i64 %_30.i1828), !dbg !39726
  %umin11141 = call i64 @llvm.umin.i64(i64 %umin11140, i64 %_31.i1830), !dbg !39726
  %634 = sub nsw i64 %umin11142, %frame.sroa.0.0.i718330, !dbg !39726
  %umin11143 = call i64 @llvm.umin.i64(i64 %umin11141, i64 %634), !dbg !39726
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055, !dbg !39726

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202
  %_0.i326113349 = phi float [ %_0.i3261.lcssa1336213440, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055.lr.ph ], [ %_0.i3261, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202 ]
  %_0.i288713334 = phi float [ %_0.i2887.lcssa1334713437, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055.lr.ph ], [ %_0.i2887, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202 ]
  %_0.i326513319 = phi float [ %_0.i3265.lcssa1333213434, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055.lr.ph ], [ %_0.i3265, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202 ]
  %_0.i289113304 = phi float [ %_0.i2891.lcssa1331713431, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055.lr.ph ], [ %_0.i2891, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202 ]
  %running.sroa.0.0.i8296 = phi float [ %running.sroa.0.0.i.lcssa83188410, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055.lr.ph ], [ %running.sroa.0.0.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202 ]
  %storemerge.i8269 = phi i32 [ %storemerge.i.lcssa82918374, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055.lr.ph ], [ %storemerge.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202 ]
  %running.sroa.0.0.i11798238 = phi float [ %running.sroa.0.0.i1179.lcssa82608371, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055.lr.ph ], [ %running.sroa.0.0.i1179, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202 ]
  %storemerge.i11848211 = phi i32 [ %storemerge.i1184.lcssa82338335, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055.lr.ph ], [ %storemerge.i1184, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202 ]
  %_12.i19328182 = phi float [ %_0.i3316.lcssa1172513428, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055.lr.ph ], [ %_0.i3316, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202 ], !dbg !39730
  %_0.i33238153 = phi float [ %_0.i3323.lcssa1173913425, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055.lr.ph ], [ %_0.i3323, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202 ], !dbg !39730
  %_5.i19268124 = phi float [ %_0.i.i3557.lcssa1175313422, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055.lr.ph ], [ %_0.i.i3557, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202 ], !dbg !39730
  %_12.i19448094 = phi float [ %_0.i3303.lcssa1176713419, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055.lr.ph ], [ %_0.i3303, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202 ], !dbg !39730
  %_0.i33108065 = phi float [ %_0.i3310.lcssa1178113416, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055.lr.ph ], [ %_0.i3310, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202 ], !dbg !39730
  %_5.i19388036 = phi float [ %_0.i.i3550.lcssa1179513398, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055.lr.ph ], [ %_0.i.i3550, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202 ], !dbg !39730
  %_12.i19568006 = phi float [ %_0.i3290.lcssa1180913395, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055.lr.ph ], [ %_0.i3290, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202 ], !dbg !39730
  %_0.i32977977 = phi float [ %_0.i3297.lcssa1182313392, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055.lr.ph ], [ %_0.i3297, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202 ], !dbg !39730
  %_5.i19507948 = phi float [ %_0.i.i3543.lcssa1183713389, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055.lr.ph ], [ %_0.i.i3543, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202 ], !dbg !39730
  %_12.i19687918 = phi float [ %_0.i3278.lcssa1185113386, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055.lr.ph ], [ %_0.i3278, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202 ], !dbg !39730
  %_0.i32847889 = phi float [ %_0.i3284.lcssa1186513383, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055.lr.ph ], [ %_0.i3284, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202 ], !dbg !39730
  %_5.i19627860 = phi float [ %_0.i.i.lcssa1187913365, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055.lr.ph ], [ %_0.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202 ], !dbg !39730
  %iter.i19.sroa.41.07858 = phi i64 [ 0, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055.lr.ph ], [ %_9.0.i4247, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202 ]
  %_9.0.i4247 = add nuw i64 %iter.i19.sroa.41.07858, 1, !dbg !39731
  %data.i.i.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_177.i82, i64 %iter.i19.sroa.41.07858, !dbg !39732
  %data.i.i.i.i4245 = getelementptr inbounds nuw float, ptr %_203.i93, i64 %iter.i19.sroa.41.07858, !dbg !39739
  %data.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_194.i92, i64 %iter.i19.sroa.41.07858, !dbg !39742
  %data.i5.i.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_185.i85, i64 %iter.i19.sroa.41.07858, !dbg !39745
  %_0.i2860 = fadd float %_5.i19627860, -1.000000e+00, !dbg !39748
  %_3.i.i = fcmp ogt float %_0.i2860, 0.000000e+00, !dbg !39752
  %_0.i.i = select i1 %_3.i.i, float %_0.i2860, float 0.000000e+00, !dbg !39755
  %_0.i2020 = fadd float %_0.i32847889, %_12.i19687918, !dbg !39757
  %_0.i3284 = select i1 %_3.i.i, float %_0.i2020, float %_13.i197047804782, !dbg !39759
  %_0.i3278 = select i1 %_3.i.i, float %_12.i19687918, float 0.000000e+00, !dbg !39761
  %_0.i2861 = fadd float %_5.i19507948, -1.000000e+00, !dbg !39763
  %_3.i.i3537 = fcmp ogt float %_0.i2861, 0.000000e+00, !dbg !39766
  %_0.i.i3543 = select i1 %_3.i.i3537, float %_0.i2861, float 0.000000e+00, !dbg !39769
  %_0.i2021 = fadd float %_0.i32977977, %_12.i19568006, !dbg !39771
  %_0.i3297 = select i1 %_3.i.i3537, float %_0.i2021, float %_13.i195847834785, !dbg !39773
  %_0.i3290 = select i1 %_3.i.i3537, float %_12.i19568006, float 0.000000e+00, !dbg !39775
  %_0.i2862 = fadd float %_5.i19388036, -1.000000e+00, !dbg !39777
  %_3.i.i3544 = fcmp ogt float %_0.i2862, 0.000000e+00, !dbg !39781
  %_0.i.i3550 = select i1 %_3.i.i3544, float %_0.i2862, float 0.000000e+00, !dbg !39784
  %_0.i2022 = fadd float %_0.i33108065, %_12.i19448094, !dbg !39786
  %_0.i3310 = select i1 %_3.i.i3544, float %_0.i2022, float %_13.i194647864788, !dbg !39788
  %_0.i3303 = select i1 %_3.i.i3544, float %_12.i19448094, float 0.000000e+00, !dbg !39790
  %_0.i2863 = fadd float %_5.i19268124, -1.000000e+00, !dbg !39792
  %_3.i.i3551 = fcmp ogt float %_0.i2863, 0.000000e+00, !dbg !39795
  %_0.i.i3557 = select i1 %_3.i.i3551, float %_0.i2863, float 0.000000e+00, !dbg !39798
  %_0.i2023 = fadd float %_0.i33238153, %_12.i19328182, !dbg !39800
  %_0.i3323 = select i1 %_3.i.i3551, float %_0.i2023, float %_13.i193447894791, !dbg !39802
  %_0.i3316 = select i1 %_3.i.i3551, float %_12.i19328182, float 0.000000e+00, !dbg !39804
  %_0.i3068 = load float, ptr %data.i.i.i.i.i.i, align 4, !dbg !39806, !alias.scope !39808, !noalias !39040, !noundef !12
  %_0.i3063 = load float, ptr %data.i.i.i.i4245, align 4, !dbg !39811, !alias.scope !39813, !noalias !39040, !noundef !12
  %_3.i.i3711 = fcmp ule float %_0.i3063, %_0.i3068, !dbg !39816
  %_6.i.i3713 = bitcast float %_0.i3063 to i32, !dbg !39819
  %_8.i.i3715 = bitcast float %_0.i3068 to i32, !dbg !39822
  %_4.i.i3718 = select i1 %_3.i.i3711, i32 %_8.i.i3715, i32 %_6.i.i3713, !dbg !39824
  %_5.i3489 = and i32 %_4.i.i3718, %.none.i42, !dbg !39825
  %_7.i3485 = and i32 %_9.i3491, %_6.i.i3713, !dbg !39827
  %_4.i3486 = or disjoint i32 %_5.i3489, %_7.i3485, !dbg !39829
  %_0.i3487 = bitcast i32 %_4.i3486 to float, !dbg !39830
  %_0.i3058 = load float, ptr %data.i.i.i.i.i.i.i.i, align 4, !dbg !39832, !alias.scope !39834, !noalias !39040, !noundef !12
  %_0.i3053 = load float, ptr %data.i5.i.i.i.i.i.i.i, align 4, !dbg !39837, !alias.scope !39839, !noalias !39040, !noundef !12
  %_205.i123 = add nuw i64 %iter.i19.sroa.41.07858, %ring_cursor.sroa.0.1.i698328, !dbg !39842
  %_206.i124 = add nuw i64 %iter.i19.sroa.41.07858, %main_cursor.sroa.0.1.i708329, !dbg !39845
  %_207.i125 = add nuw i64 %iter.i19.sroa.41.07858, %left_end.sroa.0.0.i, !dbg !39846
  %_208.i126 = add i64 %iter.i19.sroa.41.07858, %start1.sroa.0.0.i1823, !dbg !39847
  %_209.i127 = add nuw i64 %iter.i19.sroa.41.07858, %left_expiring.sroa.0.0.i, !dbg !39848
  %_7.i8.i259.i = add i64 %_205.i123, 1, !dbg !39849
  %or.cond.i11.i262.i.not = icmp ult i64 %_205.i123, %_54.1.i257.i, !dbg !39853
  br i1 %or.cond.i11.i262.i.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i263.i, label %bb4.i13.i318.i, !dbg !39853, !prof !2723

bb4.i13.i318.i:                                   ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055
  store float %_0.i.i.lcssa1187913365, ptr %587, align 4
  store float %_0.i.i3550.lcssa1179513398, ptr %593, align 4
  store float %running.sroa.0.0.i1179.lcssa1189113442, ptr %_21.i270.i, align 1, !dbg !39101
  store float %running.sroa.0.0.i.lcssa1192813459, ptr %_21.i.i154, align 1, !dbg !39132
  store float %_0.i289113304, ptr %602, align 1, !dbg !39860
  store float %_0.i326513319, ptr %604, align 1, !dbg !39864
  store float %_0.i288713334, ptr %610, align 1, !dbg !39868
  store float %_0.i326113349, ptr %612, align 1, !dbg !39869
  %umax11134 = call i64 @llvm.umax.i64(i64 %ring_cursor.sroa.0.1.i698328, i64 %_54.1.i257.i), !dbg !39726
  %635 = add i64 %umax11134, 1, !dbg !39726
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_205.i123, i64 noundef %635, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i257.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !39870, !noalias !39871
  unreachable, !dbg !39870

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i263.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055
  %_7.i3492 = and i32 %_9.i3491, %_8.i.i3715, !dbg !39879
  %_4.i3493 = or disjoint i32 %_5.i3489, %_7.i3492, !dbg !39825
  %_0.i3494 = bitcast i32 %_4.i3493 to float, !dbg !39880
  %_0.i2431 = fdiv float %_0.i3284, %_0.i3494, !dbg !39882
  %_3.i1998 = fcmp uge float %_0.i3284, %_0.i3494, !dbg !39884
  %_0.i3480 = select i1 %_3.i1998, float 1.000000e+00, float %_0.i2431, !dbg !39886
  %_17.i12.i264.i = getelementptr inbounds nuw float, ptr %_54.0.i256.i, i64 %_205.i123, !dbg !39888
  store float %_0.i3480, ptr %_17.i12.i264.i, align 4, !dbg !39892, !alias.scope !39894, !noalias !39897
  %or.cond.i1352.not = icmp ult i64 %_207.i125, %_54.1.i257.i, !dbg !39898
  br i1 %or.cond.i1352.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1356, label %bb4.i1355, !dbg !39898, !prof !2723

bb4.i1355:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i263.i
  store float %_0.i.i.lcssa1187913365, ptr %587, align 4
  store float %_0.i.i3550.lcssa1179513398, ptr %593, align 4
  store float %running.sroa.0.0.i1179.lcssa1189113442, ptr %_21.i270.i, align 1, !dbg !39101
  store float %running.sroa.0.0.i.lcssa1192813459, ptr %_21.i.i154, align 1, !dbg !39132
  store float %_0.i289113304, ptr %602, align 1, !dbg !39860
  store float %_0.i326513319, ptr %604, align 1, !dbg !39864
  store float %_0.i288713334, ptr %610, align 1, !dbg !39868
  store float %_0.i326113349, ptr %612, align 1, !dbg !39869
  %_5.i1349 = add i64 %_207.i125, 1, !dbg !39908
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_207.i125, i64 noundef %_5.i1349, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i257.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !39909, !noalias !39910
  unreachable, !dbg !39909

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1356: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i263.i
  %_15.i1353 = getelementptr inbounds nuw float, ptr %_54.0.i256.i, i64 %_207.i125, !dbg !39916
  %_0.i2925 = load float, ptr %_15.i1353, align 4, !dbg !39920, !alias.scope !39922, !noalias !39925, !noundef !12
  %position.i1175 = zext i32 %storemerge.i11848211 to i64, !dbg !39926
  %636 = icmp eq i32 %storemerge.i11848211, 0, !dbg !39927
  %_3.i.i3837.inv = fcmp olt float %running.sroa.0.0.i11798238, %_0.i2925, !dbg !39927
  %_4.i.i3844.v = select i1 %_3.i.i3837.inv, float %running.sroa.0.0.i11798238, float %_0.i2925, !dbg !39927
  %running.sroa.0.0.i1179 = select i1 %636, float %_0.i2925, float %_4.i.i3844.v, !dbg !39927
  %_15.i1180 = add nuw nsw i64 %position.i1175, 1, !dbg !39928
  %complete.i1181 = icmp eq i64 %_15.i1180, %_18.i267.i, !dbg !39928
  br i1 %complete.i1181, label %bb19.i1192, label %bb7.i1182, !dbg !39930

bb7.i1182:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1356
  %or.cond.i1344.not = icmp ult i64 %_208.i126, %_54.1.i257.i, !dbg !39932
  br i1 %or.cond.i1344.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1348, label %bb4.i1347, !dbg !39932, !prof !2723

bb4.i1347:                                        ; preds = %bb7.i1182
  store float %_0.i.i.lcssa1187913365, ptr %587, align 4
  store float %_0.i.i3550.lcssa1179513398, ptr %593, align 4
  store float %running.sroa.0.0.i1179.lcssa1189113442, ptr %_21.i270.i, align 1, !dbg !39101
  store float %running.sroa.0.0.i.lcssa1192813459, ptr %_21.i.i154, align 1, !dbg !39132
  store float %_0.i289113304, ptr %602, align 1, !dbg !39860
  store float %_0.i326513319, ptr %604, align 1, !dbg !39864
  store float %_0.i288713334, ptr %610, align 1, !dbg !39868
  store float %_0.i326113349, ptr %612, align 1, !dbg !39869
  %_5.i1341 = add i64 %_208.i126, 1, !dbg !39937
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_208.i126, i64 noundef %_5.i1341, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i257.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !39938, !noalias !39939
  unreachable, !dbg !39938

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1348: ; preds = %bb7.i1182
  %_15.i1345 = getelementptr inbounds nuw float, ptr %_54.0.i256.i, i64 %_208.i126, !dbg !39942
  %_0.i2927 = load float, ptr %_15.i1345, align 4, !dbg !39944, !alias.scope !39946, !noalias !39925, !noundef !12
  %_3.i.i3828.inv = fcmp olt float %_0.i2927, %running.sroa.0.0.i1179, !dbg !39949
  %_4.i.i3835.v = select i1 %_3.i.i3828.inv, float %_0.i2927, float %running.sroa.0.0.i1179, !dbg !39949
  %637 = trunc i64 %_15.i1180 to i32, !dbg !39953
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1203, !dbg !39955

bb19.i1192:                                       ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1356, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1199
  %end.sroa.0.0.i11907852 = phi i64 [ %639, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1199 ], [ %_207.i125, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1356 ]
  %suffix.sroa.0.0.i11897851 = phi float [ %_4.i.i3826.v, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1199 ], [ %_0.i2925, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1356 ]
  %iter.sroa.0.0.i11887850 = phi i64 [ %_30.i1193, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1199 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1356 ]
  %or.cond.i1328.not = icmp ult i64 %end.sroa.0.0.i11907852, %_54.1.i257.i, !dbg !39956
  br i1 %or.cond.i1328.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1199, label %bb4.i1331, !dbg !39956, !prof !2723

bb4.i1331:                                        ; preds = %bb19.i1192
  store float %_0.i.i.lcssa1187913365, ptr %587, align 4
  store float %_0.i.i3550.lcssa1179513398, ptr %593, align 4
  store float %running.sroa.0.0.i1179.lcssa1189113442, ptr %_21.i270.i, align 1, !dbg !39101
  store float %running.sroa.0.0.i.lcssa1192813459, ptr %_21.i.i154, align 1, !dbg !39132
  store float %_0.i289113304, ptr %602, align 1, !dbg !39860
  store float %_0.i326513319, ptr %604, align 1, !dbg !39864
  store float %_0.i288713334, ptr %610, align 1, !dbg !39868
  store float %_0.i326113349, ptr %612, align 1, !dbg !39869
  %_5.i1325 = add i64 %end.sroa.0.0.i11907852, 1, !dbg !39964
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %end.sroa.0.0.i11907852, i64 noundef %_5.i1325, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i257.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !39965, !noalias !39966
  unreachable, !dbg !39965

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1199: ; preds = %bb19.i1192
  %_30.i1193 = add nuw i64 %iter.sroa.0.0.i11887850, 1, !dbg !39969
  %_15.i1329 = getelementptr inbounds nuw float, ptr %_54.0.i256.i, i64 %end.sroa.0.0.i11907852, !dbg !39980
  %_0.i2931 = load float, ptr %_15.i1329, align 4, !dbg !39982, !alias.scope !39984, !noalias !39925, !noundef !12
  %_3.i.i3819.inv = fcmp olt float %suffix.sroa.0.0.i11897851, %_0.i2931, !dbg !39987
  %_4.i.i3826.v = select i1 %_3.i.i3819.inv, float %suffix.sroa.0.0.i11897851, float %_0.i2931, !dbg !39987
  store float %_4.i.i3826.v, ptr %_15.i1329, align 4, !dbg !39990, !alias.scope !39993, !noalias !39925
  %638 = icmp eq i64 %end.sroa.0.0.i11907852, 0, !dbg !39996
  %spec.store.select.i1201 = select i1 %638, i64 %ring.i45, i64 %end.sroa.0.0.i11907852, !dbg !39996
  %639 = add i64 %spec.store.select.i1201, -1, !dbg !39997
  %exitcond11131.not = icmp eq i64 %_30.i1193, %umax11130, !dbg !39998
  br i1 %exitcond11131.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1203, label %bb19.i1192, !dbg !40001

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1203: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1199, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1348
  %storemerge.i1184 = phi i32 [ %637, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1348 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1199 ], !dbg !40002
  %running.sroa.0.1.i1185 = phi float [ %_4.i.i3835.v, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1348 ], [ %running.sroa.0.0.i1179, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1199 ], !dbg !39101
  %_0.i2757 = fmul float %running.sroa.0.1.i1185, 1.638400e+04, !dbg !40003
  %640 = tail call noundef float @llvm.floor.f32(float %_0.i2757), !dbg !40005
  %_0.i2756 = fmul float %640, 0x3F10000000000000, !dbg !40009
  %or.cond.i1416.not = icmp ult i64 %_209.i127, %_56.1.i279.i, !dbg !40011
  br i1 %or.cond.i1416.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1420, label %bb4.i1419, !dbg !40011, !prof !2723

bb4.i1419:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1203
  store float %_0.i.i.lcssa1187913365, ptr %587, align 4
  store float %_0.i.i3550.lcssa1179513398, ptr %593, align 4
  store float %running.sroa.0.0.i1179.lcssa1189113442, ptr %_21.i270.i, align 1, !dbg !39101
  store float %running.sroa.0.0.i.lcssa1192813459, ptr %_21.i.i154, align 1, !dbg !39132
  store float %_0.i289113304, ptr %602, align 1, !dbg !39860
  store float %_0.i326513319, ptr %604, align 1, !dbg !39864
  store float %_0.i288713334, ptr %610, align 1, !dbg !39868
  store float %_0.i326113349, ptr %612, align 1, !dbg !39869
  %_5.i1413 = add i64 %_209.i127, 1, !dbg !40016
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_209.i127, i64 noundef %_5.i1413, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i279.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !40017, !noalias !40018
  unreachable, !dbg !40017

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1420: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1203
  %_15.i1417 = getelementptr inbounds nuw float, ptr %_56.0.i278.i, i64 %_209.i127, !dbg !40021
  %_0.i2909 = load float, ptr %_15.i1417, align 4, !dbg !40023, !alias.scope !40025, !noalias !40028, !noundef !12
  %_0.i2321 = fadd float %_0.i2756, %_0.i289113304, !dbg !40029
  %_0.i2891 = fsub float %_0.i2321, %_0.i2909, !dbg !40031
  %_8.not.i3.i288.i = icmp ugt i64 %_7.i8.i259.i, %_56.1.i279.i
  br i1 %_8.not.i3.i288.i, label %bb4.i6.i317.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i289.i, !dbg !40033, !prof !165

bb4.i6.i317.i:                                    ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1420
  store float %_0.i.i.lcssa1187913365, ptr %587, align 4
  store float %_0.i.i3550.lcssa1179513398, ptr %593, align 4
  store float %running.sroa.0.0.i1179.lcssa1189113442, ptr %_21.i270.i, align 1, !dbg !39101
  store float %running.sroa.0.0.i.lcssa1192813459, ptr %_21.i.i154, align 1, !dbg !39132
  store float %_0.i2891, ptr %602, align 1, !dbg !39860
  store float %_0.i326513319, ptr %604, align 1, !dbg !39864
  store float %_0.i288713334, ptr %610, align 1, !dbg !39868
  store float %_0.i326113349, ptr %612, align 1, !dbg !39869
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_205.i123, i64 noundef %_7.i8.i259.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i279.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !40038, !noalias !40039
  unreachable, !dbg !40038

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i289.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1420
  %_17.i5.i290.i = getelementptr inbounds nuw float, ptr %_56.0.i278.i, i64 %_205.i123, !dbg !40042
  store float %_0.i2756, ptr %_17.i5.i290.i, align 4, !dbg !40044, !alias.scope !40046, !noalias !40028
  %_0.i2430 = fdiv float %_0.i2891, %_37.i291.i, !dbg !40049
  %_0.i2890 = fsub float 1.000000e+00, %_0.i2430, !dbg !40051
  %_0.i2889 = fsub float %_0.i2890, %_0.i326513319, !dbg !40053
  %_4.i2446 = fmul float %_0.i3297, %_0.i2889, !dbg !40055
  %_0.i2447 = fadd float %_0.i326513319, %_4.i2446, !dbg !40055
  %_3.i.i3702.inv = fcmp ogt float %_0.i2890, %_0.i2447, !dbg !40057
  %_4.i.i3709.v = select i1 %_3.i.i3702.inv, float %_0.i2890, float %_0.i2447, !dbg !40057
  %641 = tail call noundef float @llvm.fabs.f32(float %_4.i.i3709.v), !dbg !40060
  %642 = fcmp uge float %641, 0x3BC79CA100000000, !dbg !40064
  %_0.i3265 = select i1 %642, float %_4.i.i3709.v, float 0.000000e+00, !dbg !40066
  %_5.i1405 = add i64 %_206.i124, 1, !dbg !40067
  %or.cond.i1408.not = icmp ult i64 %_206.i124, %_58.1.i304.i, !dbg !40070
  br i1 %or.cond.i1408.not, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3209, label %bb4.i1411, !dbg !40070, !prof !2723

bb4.i1411:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i289.i
  store float %_0.i.i.lcssa1187913365, ptr %587, align 4
  store float %_0.i.i3550.lcssa1179513398, ptr %593, align 4
  store float %running.sroa.0.0.i1179.lcssa1189113442, ptr %_21.i270.i, align 1, !dbg !39101
  store float %running.sroa.0.0.i.lcssa1192813459, ptr %_21.i.i154, align 1, !dbg !39132
  store float %_0.i2891, ptr %602, align 1, !dbg !39860
  store float %_0.i3265, ptr %604, align 1, !dbg !39864
  store float %_0.i288713334, ptr %610, align 1, !dbg !39868
  store float %_0.i326113349, ptr %612, align 1, !dbg !39869
  %umax11135 = call i64 @llvm.umax.i64(i64 %main_cursor.sroa.0.1.i708329, i64 %_58.1.i304.i), !dbg !39726
  %643 = add i64 %umax11135, 1, !dbg !39726
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_206.i124, i64 noundef %643, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i304.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !40074, !noalias !40075
  unreachable, !dbg !40074

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3209: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i289.i
  %_0.i2888 = fsub float 1.000000e+00, %_0.i3265, !dbg !40078
  %_15.i1409 = getelementptr inbounds nuw float, ptr %_58.0.i303.i, i64 %_206.i124, !dbg !40080
  %_0.i2911 = load float, ptr %_15.i1409, align 4, !dbg !40082, !alias.scope !40084, !noalias !40028, !noundef !12
  store float %_0.i3058, ptr %_15.i1409, align 4, !dbg !40087, !alias.scope !40091, !noalias !40028
  %_0.i2755 = fmul float %_0.i2888, %_0.i2911, !dbg !40094
  %_6.i3468 = bitcast float %_0.i2911 to i32, !dbg !40096
  %_5.i3469 = and i32 %_6.i3468, %all.sroa.0.0.i44, !dbg !40099
  %_8.i3470 = bitcast float %_0.i2755 to i32, !dbg !40100
  %_7.i3472 = and i32 %_9.i3471, %_8.i3470, !dbg !40102
  %_4.i3473 = or disjoint i32 %_7.i3472, %_5.i3469, !dbg !40099
  store i32 %_4.i3473, ptr %data.i.i.i.i.i.i.i.i, align 4, !dbg !40103, !alias.scope !40105, !noalias !40108
  %_212.i135 = add nuw i64 %iter.i19.sroa.41.07858, %right_end.sroa.0.0.i, !dbg !40109
  %_214.i137 = add nuw i64 %iter.i19.sroa.41.07858, %right_expiring.sroa.0.0.i, !dbg !40111
  %_8.not.i10.i.i147 = icmp ugt i64 %_7.i8.i259.i, %_54.1.i.i143
  br i1 %_8.not.i10.i.i147, label %bb4.i13.i.i197, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i149, !dbg !40112, !prof !165

bb4.i13.i.i197:                                   ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3209
  store float %_0.i.i.lcssa1187913365, ptr %587, align 4
  store float %_0.i.i3550.lcssa1179513398, ptr %593, align 4
  store float %running.sroa.0.0.i1179.lcssa1189113442, ptr %_21.i270.i, align 1, !dbg !39101
  store float %running.sroa.0.0.i.lcssa1192813459, ptr %_21.i.i154, align 1, !dbg !39132
  store float %_0.i2891, ptr %602, align 1, !dbg !39860
  store float %_0.i3265, ptr %604, align 1, !dbg !39864
  store float %_0.i288713334, ptr %610, align 1, !dbg !39868
  store float %_0.i326113349, ptr %612, align 1, !dbg !39869
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_205.i123, i64 noundef %_7.i8.i259.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i143, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !40117, !noalias !40118
  unreachable, !dbg !40117

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i149: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3209
  %_0.i2429 = fdiv float %_0.i3310, %_0.i3487, !dbg !40126
  %_3.i1996 = fcmp uge float %_0.i3310, %_0.i3487, !dbg !40128
  %_0.i3467 = select i1 %_3.i1996, float 1.000000e+00, float %_0.i2429, !dbg !40130
  %_17.i12.i.i150 = getelementptr inbounds nuw float, ptr %_54.0.i.i142, i64 %_205.i123, !dbg !40132
  store float %_0.i3467, ptr %_17.i12.i.i150, align 4, !dbg !40134, !alias.scope !40136, !noalias !40139
  %or.cond.i1384.not = icmp ult i64 %_212.i135, %_54.1.i.i143, !dbg !40140
  br i1 %or.cond.i1384.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1388, label %bb4.i1387, !dbg !40140, !prof !2723

bb4.i1387:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i149
  store float %_0.i.i.lcssa1187913365, ptr %587, align 4
  store float %_0.i.i3550.lcssa1179513398, ptr %593, align 4
  store float %running.sroa.0.0.i1179.lcssa1189113442, ptr %_21.i270.i, align 1, !dbg !39101
  store float %running.sroa.0.0.i.lcssa1192813459, ptr %_21.i.i154, align 1, !dbg !39132
  store float %_0.i2891, ptr %602, align 1, !dbg !39860
  store float %_0.i3265, ptr %604, align 1, !dbg !39864
  store float %_0.i288713334, ptr %610, align 1, !dbg !39868
  store float %_0.i326113349, ptr %612, align 1, !dbg !39869
  %_5.i1381 = add i64 %_212.i135, 1, !dbg !40145
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_212.i135, i64 noundef %_5.i1381, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i143, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !40146, !noalias !40147
  unreachable, !dbg !40146

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1388: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i149
  %_15.i1385 = getelementptr inbounds nuw float, ptr %_54.0.i.i142, i64 %_212.i135, !dbg !40153
  %_0.i2917 = load float, ptr %_15.i1385, align 4, !dbg !40155, !alias.scope !40157, !noalias !40160, !noundef !12
  %position.i = zext i32 %storemerge.i8269 to i64, !dbg !40161
  %644 = icmp eq i32 %storemerge.i8269, 0, !dbg !40162
  %_3.i.i3864.inv = fcmp olt float %running.sroa.0.0.i8296, %_0.i2917, !dbg !40162
  %_4.i.i3871.v = select i1 %_3.i.i3864.inv, float %running.sroa.0.0.i8296, float %_0.i2917, !dbg !40162
  %running.sroa.0.0.i = select i1 %644, float %_0.i2917, float %_4.i.i3871.v, !dbg !40162
  %_15.i = add nuw nsw i64 %position.i, 1, !dbg !40163
  %complete.i = icmp eq i64 %_15.i, %_18.i.i153, !dbg !40163
  br i1 %complete.i, label %bb19.i1169, label %bb7.i1165, !dbg !40164

bb7.i1165:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1388
  %or.cond.i1376.not = icmp ult i64 %_208.i126, %_54.1.i.i143, !dbg !40165
  br i1 %or.cond.i1376.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1380, label %bb4.i1379, !dbg !40165, !prof !2723

bb4.i1379:                                        ; preds = %bb7.i1165
  store float %_0.i.i.lcssa1187913365, ptr %587, align 4
  store float %_0.i.i3550.lcssa1179513398, ptr %593, align 4
  store float %running.sroa.0.0.i1179.lcssa1189113442, ptr %_21.i270.i, align 1, !dbg !39101
  store float %running.sroa.0.0.i.lcssa1192813459, ptr %_21.i.i154, align 1, !dbg !39132
  store float %_0.i2891, ptr %602, align 1, !dbg !39860
  store float %_0.i3265, ptr %604, align 1, !dbg !39864
  store float %_0.i288713334, ptr %610, align 1, !dbg !39868
  store float %_0.i326113349, ptr %612, align 1, !dbg !39869
  %_5.i1373 = add i64 %_208.i126, 1, !dbg !40170
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_208.i126, i64 noundef %_5.i1373, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i143, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !40171, !noalias !40172
  unreachable, !dbg !40171

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1380: ; preds = %bb7.i1165
  %_15.i1377 = getelementptr inbounds nuw float, ptr %_54.0.i.i142, i64 %_208.i126, !dbg !40175
  %_0.i2919 = load float, ptr %_15.i1377, align 4, !dbg !40177, !alias.scope !40179, !noalias !40160, !noundef !12
  %_3.i.i3855.inv = fcmp olt float %_0.i2919, %running.sroa.0.0.i, !dbg !40182
  %_4.i.i3862.v = select i1 %_3.i.i3855.inv, float %_0.i2919, float %running.sroa.0.0.i, !dbg !40182
  %645 = trunc i64 %_15.i to i32, !dbg !40185
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit, !dbg !40186

bb19.i1169:                                       ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1388, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i
  %end.sroa.0.0.i7855 = phi i64 [ %647, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], [ %_212.i135, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1388 ]
  %suffix.sroa.0.0.i7854 = phi float [ %_4.i.i3853.v, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], [ %_0.i2917, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1388 ]
  %iter.sroa.0.0.i11687853 = phi i64 [ %_30.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1388 ]
  %or.cond.i1360.not = icmp ult i64 %end.sroa.0.0.i7855, %_54.1.i.i143, !dbg !40187
  br i1 %or.cond.i1360.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i, label %bb4.i1363, !dbg !40187, !prof !2723

bb4.i1363:                                        ; preds = %bb19.i1169
  store float %_0.i.i.lcssa1187913365, ptr %587, align 4
  store float %_0.i.i3550.lcssa1179513398, ptr %593, align 4
  store float %running.sroa.0.0.i1179.lcssa1189113442, ptr %_21.i270.i, align 1, !dbg !39101
  store float %running.sroa.0.0.i.lcssa1192813459, ptr %_21.i.i154, align 1, !dbg !39132
  store float %_0.i2891, ptr %602, align 1, !dbg !39860
  store float %_0.i3265, ptr %604, align 1, !dbg !39864
  store float %_0.i288713334, ptr %610, align 1, !dbg !39868
  store float %_0.i326113349, ptr %612, align 1, !dbg !39869
  %_5.i1357 = add i64 %end.sroa.0.0.i7855, 1, !dbg !40192
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %end.sroa.0.0.i7855, i64 noundef %_5.i1357, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i143, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !40193, !noalias !40194
  unreachable, !dbg !40193

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i: ; preds = %bb19.i1169
  %_30.i = add nuw i64 %iter.sroa.0.0.i11687853, 1, !dbg !40197
  %_15.i1361 = getelementptr inbounds nuw float, ptr %_54.0.i.i142, i64 %end.sroa.0.0.i7855, !dbg !40202
  %_0.i2923 = load float, ptr %_15.i1361, align 4, !dbg !40204, !alias.scope !40206, !noalias !40160, !noundef !12
  %_3.i.i3846.inv = fcmp olt float %suffix.sroa.0.0.i7854, %_0.i2923, !dbg !40209
  %_4.i.i3853.v = select i1 %_3.i.i3846.inv, float %suffix.sroa.0.0.i7854, float %_0.i2923, !dbg !40209
  store float %_4.i.i3853.v, ptr %_15.i1361, align 4, !dbg !40212, !alias.scope !40215, !noalias !40160
  %646 = icmp eq i64 %end.sroa.0.0.i7855, 0, !dbg !40218
  %spec.store.select.i1172 = select i1 %646, i64 %ring.i45, i64 %end.sroa.0.0.i7855, !dbg !40218
  %647 = add i64 %spec.store.select.i1172, -1, !dbg !40219
  %exitcond11133.not = icmp eq i64 %_30.i, %umax11132, !dbg !40220
  br i1 %exitcond11133.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit, label %bb19.i1169, !dbg !40222

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1380
  %storemerge.i = phi i32 [ %645, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1380 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], !dbg !40223
  %running.sroa.0.1.i = phi float [ %_4.i.i3862.v, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1380 ], [ %running.sroa.0.0.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], !dbg !39132
  %_0.i2754 = fmul float %running.sroa.0.1.i, 1.638400e+04, !dbg !40224
  %648 = tail call noundef float @llvm.floor.f32(float %_0.i2754), !dbg !40226
  %_0.i2753 = fmul float %648, 0x3F10000000000000, !dbg !40230
  %or.cond.i1400.not = icmp ult i64 %_214.i137, %_56.1.i.i161, !dbg !40232
  br i1 %or.cond.i1400.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1404, label %bb4.i1403, !dbg !40232, !prof !2723

bb4.i1403:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit
  store float %_0.i.i.lcssa1187913365, ptr %587, align 4
  store float %_0.i.i3550.lcssa1179513398, ptr %593, align 4
  store float %running.sroa.0.0.i1179.lcssa1189113442, ptr %_21.i270.i, align 1, !dbg !39101
  store float %running.sroa.0.0.i.lcssa1192813459, ptr %_21.i.i154, align 1, !dbg !39132
  store float %_0.i2891, ptr %602, align 1, !dbg !39860
  store float %_0.i3265, ptr %604, align 1, !dbg !39864
  store float %_0.i288713334, ptr %610, align 1, !dbg !39868
  store float %_0.i326113349, ptr %612, align 1, !dbg !39869
  %_5.i1397 = add i64 %_214.i137, 1, !dbg !40237
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_214.i137, i64 noundef %_5.i1397, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i161, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !40238, !noalias !40239
  unreachable, !dbg !40238

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1404: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit
  %_15.i1401 = getelementptr inbounds nuw float, ptr %_56.0.i.i160, i64 %_214.i137, !dbg !40242
  %_0.i2913 = load float, ptr %_15.i1401, align 4, !dbg !40244, !alias.scope !40246, !noalias !40249, !noundef !12
  %_0.i2320 = fadd float %_0.i2753, %_0.i288713334, !dbg !40250
  %_0.i2887 = fsub float %_0.i2320, %_0.i2913, !dbg !40252
  %_8.not.i3.i.i168 = icmp ugt i64 %_7.i8.i259.i, %_56.1.i.i161
  br i1 %_8.not.i3.i.i168, label %bb4.i6.i.i196, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i169, !dbg !40254, !prof !165

bb4.i6.i.i196:                                    ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1404
  store float %_0.i.i.lcssa1187913365, ptr %587, align 4
  store float %_0.i.i3550.lcssa1179513398, ptr %593, align 4
  store float %running.sroa.0.0.i1179.lcssa1189113442, ptr %_21.i270.i, align 1, !dbg !39101
  store float %running.sroa.0.0.i.lcssa1192813459, ptr %_21.i.i154, align 1, !dbg !39132
  store float %_0.i2891, ptr %602, align 1, !dbg !39860
  store float %_0.i3265, ptr %604, align 1, !dbg !39864
  store float %_0.i2887, ptr %610, align 1, !dbg !39868
  store float %_0.i326113349, ptr %612, align 1, !dbg !39869
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_205.i123, i64 noundef %_7.i8.i259.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i161, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !40259, !noalias !40260
  unreachable, !dbg !40259

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i169: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1404
  %_17.i5.i.i170 = getelementptr inbounds nuw float, ptr %_56.0.i.i160, i64 %_205.i123, !dbg !40263
  store float %_0.i2753, ptr %_17.i5.i.i170, align 4, !dbg !40265, !alias.scope !40267, !noalias !40249
  %_0.i2428 = fdiv float %_0.i2887, %_37.i.i171, !dbg !40270
  %_0.i2886 = fsub float 1.000000e+00, %_0.i2428, !dbg !40272
  %_0.i2885 = fsub float %_0.i2886, %_0.i326113349, !dbg !40274
  %_4.i2444 = fmul float %_0.i3323, %_0.i2885, !dbg !40276
  %_0.i2445 = fadd float %_0.i326113349, %_4.i2444, !dbg !40276
  %_3.i.i3693.inv = fcmp ogt float %_0.i2886, %_0.i2445, !dbg !40278
  %_4.i.i3700.v = select i1 %_3.i.i3693.inv, float %_0.i2886, float %_0.i2445, !dbg !40278
  %649 = tail call noundef float @llvm.fabs.f32(float %_4.i.i3700.v), !dbg !40281
  %650 = fcmp uge float %649, 0x3BC79CA100000000, !dbg !40284
  %_0.i3261 = select i1 %650, float %_4.i.i3700.v, float 0.000000e+00, !dbg !40286
  %_6.not.i1391 = icmp ugt i64 %_5.i1405, %_58.1.i.i184
  br i1 %_6.not.i1391, label %bb4.i1395, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202, !dbg !40287, !prof !165

bb4.i1395:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i169
  store float %_0.i.i.lcssa1187913365, ptr %587, align 4
  store float %_0.i.i3550.lcssa1179513398, ptr %593, align 4
  store float %running.sroa.0.0.i1179.lcssa1189113442, ptr %_21.i270.i, align 1, !dbg !39101
  store float %running.sroa.0.0.i.lcssa1192813459, ptr %_21.i.i154, align 1, !dbg !39132
  store float %_0.i2891, ptr %602, align 1, !dbg !39860
  store float %_0.i3265, ptr %604, align 1, !dbg !39864
  store float %_0.i2887, ptr %610, align 1, !dbg !39868
  store float %_0.i3261, ptr %612, align 1, !dbg !39869
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_206.i124, i64 noundef %_5.i1405, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i.i184, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !40292, !noalias !40293
  unreachable, !dbg !40292

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i169
  %_0.i2884 = fsub float 1.000000e+00, %_0.i3261, !dbg !40296
  %_15.i1393 = getelementptr inbounds nuw float, ptr %_58.0.i.i183, i64 %_206.i124, !dbg !40298
  %_0.i2915 = load float, ptr %_15.i1393, align 4, !dbg !40300, !alias.scope !40302, !noalias !40249, !noundef !12
  store float %_0.i3053, ptr %_15.i1393, align 4, !dbg !40305, !alias.scope !40308, !noalias !40249
  %_0.i2752 = fmul float %_0.i2884, %_0.i2915, !dbg !40311
  %_6.i3455 = bitcast float %_0.i2915 to i32, !dbg !40313
  %_5.i3456 = and i32 %_6.i3455, %all.sroa.0.0.i44, !dbg !40316
  %_8.i3457 = bitcast float %_0.i2752 to i32, !dbg !40317
  %_7.i3459 = and i32 %_9.i3471, %_8.i3457, !dbg !40319
  %_4.i3460 = or disjoint i32 %_7.i3459, %_5.i3456, !dbg !40316
  store i32 %_4.i3460, ptr %data.i5.i.i.i.i.i.i.i, align 4, !dbg !40320, !alias.scope !40322, !noalias !40325
  %exitcond11144.not = icmp eq i64 %_9.0.i4247, %umin11143, !dbg !39726
  br i1 %exitcond11144.not, label %bb29.i95.bb32.i198_crit_edge, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3055, !dbg !39726

bb29.i95.bb32.i198_crit_edge:                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3202
  store float %_0.i2891, ptr %602, align 1, !dbg !39860
  store float %_0.i3265, ptr %604, align 1, !dbg !39864
  store float %_0.i2887, ptr %610, align 1, !dbg !39868
  store float %_0.i3261, ptr %612, align 1, !dbg !39869
  store float %_0.i3284, ptr %_109.i106, align 4, !dbg !40326, !alias.scope !40327, !noalias !40330
  store float %_0.i3278, ptr %588, align 4, !dbg !40333, !alias.scope !40327, !noalias !40330
  store float %_0.i.i3543, ptr %590, align 4, !dbg !40334, !alias.scope !40335, !noalias !39040
  store float %_0.i3297, ptr %_110.i107, align 4, !dbg !40338, !alias.scope !40335, !noalias !39040
  store float %_0.i3290, ptr %591, align 4, !dbg !40339, !alias.scope !40335, !noalias !39040
  store float %_0.i3310, ptr %_114.i108, align 4, !dbg !40340, !alias.scope !40341, !noalias !40344
  store float %_0.i3303, ptr %594, align 4, !dbg !40347, !alias.scope !40341, !noalias !40344
  store float %_0.i.i3557, ptr %596, align 4, !dbg !40348, !alias.scope !40349, !noalias !39040
  store float %_0.i3323, ptr %_115.i109, align 4, !dbg !40352, !alias.scope !40349, !noalias !39040
  store float %_0.i3316, ptr %597, align 4, !dbg !40353, !alias.scope !40349, !noalias !39040
  br label %bb32.i198, !dbg !39726

bb32.i198:                                        ; preds = %bb29.i95.bb32.i198_crit_edge, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit
  %running.sroa.0.0.i.lcssa1192813458 = phi float [ %running.sroa.0.0.i, %bb29.i95.bb32.i198_crit_edge ], [ %running.sroa.0.0.i.lcssa1192813459, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %running.sroa.0.0.i1179.lcssa1189113441 = phi float [ %running.sroa.0.0.i1179, %bb29.i95.bb32.i198_crit_edge ], [ %running.sroa.0.0.i1179.lcssa1189113442, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i3261.lcssa1336213439 = phi float [ %_0.i3261, %bb29.i95.bb32.i198_crit_edge ], [ %_0.i3261.lcssa1336213440, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i2887.lcssa1334713436 = phi float [ %_0.i2887, %bb29.i95.bb32.i198_crit_edge ], [ %_0.i2887.lcssa1334713437, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i3265.lcssa1333213433 = phi float [ %_0.i3265, %bb29.i95.bb32.i198_crit_edge ], [ %_0.i3265.lcssa1333213434, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i2891.lcssa1331713430 = phi float [ %_0.i2891, %bb29.i95.bb32.i198_crit_edge ], [ %_0.i2891.lcssa1331713431, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i3316.lcssa1172513427 = phi float [ %_0.i3316, %bb29.i95.bb32.i198_crit_edge ], [ %_0.i3316.lcssa1172513428, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i3323.lcssa1173913424 = phi float [ %_0.i3323, %bb29.i95.bb32.i198_crit_edge ], [ %_0.i3323.lcssa1173913425, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i.i3557.lcssa1175313421 = phi float [ %_0.i.i3557, %bb29.i95.bb32.i198_crit_edge ], [ %_0.i.i3557.lcssa1175313422, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i3303.lcssa1176713418 = phi float [ %_0.i3303, %bb29.i95.bb32.i198_crit_edge ], [ %_0.i3303.lcssa1176713419, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i3310.lcssa1178113415 = phi float [ %_0.i3310, %bb29.i95.bb32.i198_crit_edge ], [ %_0.i3310.lcssa1178113416, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i.i3550.lcssa1179513397 = phi float [ %_0.i.i3550, %bb29.i95.bb32.i198_crit_edge ], [ %_0.i.i3550.lcssa1179513398, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i3290.lcssa1180913394 = phi float [ %_0.i3290, %bb29.i95.bb32.i198_crit_edge ], [ %_0.i3290.lcssa1180913395, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i3297.lcssa1182313391 = phi float [ %_0.i3297, %bb29.i95.bb32.i198_crit_edge ], [ %_0.i3297.lcssa1182313392, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i.i3543.lcssa1183713388 = phi float [ %_0.i.i3543, %bb29.i95.bb32.i198_crit_edge ], [ %_0.i.i3543.lcssa1183713389, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i3278.lcssa1185113385 = phi float [ %_0.i3278, %bb29.i95.bb32.i198_crit_edge ], [ %_0.i3278.lcssa1185113386, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i3284.lcssa1186513382 = phi float [ %_0.i3284, %bb29.i95.bb32.i198_crit_edge ], [ %_0.i3284.lcssa1186513383, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_0.i.i.lcssa1187913364 = phi float [ %_0.i.i, %bb29.i95.bb32.i198_crit_edge ], [ %_0.i.i.lcssa1187913365, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %running.sroa.0.0.i.lcssa83188409 = phi float [ %running.sroa.0.0.i, %bb29.i95.bb32.i198_crit_edge ], [ %running.sroa.0.0.i.lcssa83188410, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %storemerge.i.lcssa82918373 = phi i32 [ %storemerge.i, %bb29.i95.bb32.i198_crit_edge ], [ %storemerge.i.lcssa82918374, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %running.sroa.0.0.i1179.lcssa82608370 = phi float [ %running.sroa.0.0.i1179, %bb29.i95.bb32.i198_crit_edge ], [ %running.sroa.0.0.i1179.lcssa82608371, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %storemerge.i1184.lcssa82338334 = phi i32 [ %storemerge.i1184, %bb29.i95.bb32.i198_crit_edge ], [ %storemerge.i1184.lcssa82338335, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_138.i199 = add i64 %..i4199, %ring_cursor.sroa.0.1.i698328, !dbg !40354
  %_204.not.i200 = icmp ult i64 %_138.i199, %ring.i45, !dbg !40355
  %651 = select i1 %_204.not.i200, i64 0, i64 %ring.i45, !dbg !40355
  %ring_cursor.sroa.0.2.i201 = sub nuw i64 %_138.i199, %651, !dbg !40355
  %_140.i202 = add i64 %..i4199, %main_cursor.sroa.0.1.i708329, !dbg !40358
  %_215.not.i203 = icmp ult i64 %_140.i202, %main.i46, !dbg !40359
  %652 = select i1 %_215.not.i203, i64 0, i64 %main.i46, !dbg !40359
  %main_cursor.sroa.0.2.i204 = sub nuw i64 %_140.i202, %652, !dbg !40359
  %_58.i72 = icmp ult i64 %_80.i86, %..i4168, !dbg !39135
  br i1 %_58.i72, label %bb20.i73, label %bb19.i68.bb15.i54.loopexit_crit_edge, !dbg !39135

_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit.loopexit: ; preds = %bb15.i54.loopexit
  %653 = trunc i64 %main_cursor.sroa.0.1.i70.lcssa to i32, !dbg !40361
  %654 = trunc i64 %ring_cursor.sroa.0.1.i69.lcssa to i32, !dbg !40363
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit, !dbg !40364

_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit.loopexit, %bb7.i
  %ring_cursor.sroa.0.0.i55.lcssa = phi i32 [ %_36.i48, %bb7.i ], [ %654, %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit.loopexit ], !dbg !39070
  %main_cursor.sroa.0.0.i56.lcssa = phi i32 [ %_35.i47, %bb7.i ], [ %653, %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit.loopexit ], !dbg !39067
  %655 = getelementptr inbounds nuw i8, ptr %uniform_left.i33, i64 72, !dbg !40364
  %left_prefix.i406 = load float, ptr %655, align 8, !dbg !40364, !noalias !39046, !noundef !12
  %656 = getelementptr inbounds nuw i8, ptr %uniform_left.i33, i64 76, !dbg !40365
  %left_phase.i407 = load i32, ptr %656, align 4, !dbg !40365, !noalias !39046, !noundef !12
  %657 = getelementptr inbounds nuw i8, ptr %uniform_right.i32, i64 72, !dbg !40366
  %right_prefix.i408 = load float, ptr %657, align 8, !dbg !40366, !noalias !39046, !noundef !12
  %658 = getelementptr inbounds nuw i8, ptr %uniform_right.i32, i64 76, !dbg !40367
  %right_phase.i409 = load i32, ptr %658, align 4, !dbg !40367, !noalias !39046, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_right.i32), !dbg !40368, !noalias !39046
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i33), !dbg !40369, !noalias !39046
  %659 = getelementptr inbounds nuw i8, ptr %self, i64 216, !dbg !40370
  %_230.0.i410 = load ptr, ptr %659, align 8, !dbg !40370, !alias.scope !39036, !noalias !40371, !nonnull !12, !noundef !12
  %660 = getelementptr inbounds nuw i8, ptr %self, i64 224, !dbg !40370
  %_230.1.i411 = load i64, ptr %660, align 8, !dbg !40370, !alias.scope !39036, !noalias !40371, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !40372), !dbg !40375
  %_4.not.i3187 = icmp eq i64 %_230.1.i411, 0, !dbg !40376
  br i1 %_4.not.i3187, label %panic.i3189, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3190, !dbg !40376

panic.i3189:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !40376, !noalias !40378
  unreachable, !dbg !40376

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3190: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit
  store float %left_prefix.i406, ptr %_230.0.i410, align 4, !dbg !40376, !alias.scope !40372, !noalias !39040
  %_231.0.i412 = load ptr, ptr %68, align 8, !dbg !40379, !alias.scope !39036, !noalias !40371, !nonnull !12, !noundef !12
  %_231.1.i413 = load i64, ptr %69, align 8, !dbg !40379, !alias.scope !39036, !noalias !40371, !noundef !12
  %661 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i407), !dbg !40380
  br i1 %661, label %bb2.i4253, label %bb6.i4248, !dbg !40380

bb6.i4248:                                        ; preds = %bb2.i4253, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3190
  %end_or_len.idx.i = shl nuw nsw i64 %_231.1.i413, 2, !dbg !40384
  %end_or_len.i = getelementptr inbounds nuw i8, ptr %_231.0.i412, i64 %end_or_len.idx.i, !dbg !40384
  %_293.i = icmp eq i64 %_231.1.i413, 0, !dbg !40388
  br i1 %_293.i, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i4249, !dbg !40391

bb2.i4253:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3190
  %bytes1.sroa.0.0.zext.i = and i32 %left_phase.i407, 255, !dbg !40392
  %bytes1.sroa.0.0.isplat.i = mul nuw i32 %bytes1.sroa.0.0.zext.i, 16843009, !dbg !40392
  %_5.i4254 = icmp eq i32 %left_phase.i407, %bytes1.sroa.0.0.isplat.i, !dbg !40393
  br i1 %_5.i4254, label %bb3.i4255, label %bb6.i4248, !dbg !40393

bb3.i4255:                                        ; preds = %bb2.i4253
  %bytes.sroa.0.0.extract.trunc.i = trunc i32 %left_phase.i407 to i8, !dbg !40394
  %662 = shl nuw nsw i64 %_231.1.i413, 2, !dbg !40396
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_231.0.i412, i8 %bytes.sroa.0.0.extract.trunc.i, i64 %662, i1 false), !dbg !40396, !alias.scope !40397, !noalias !39040
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, !dbg !40400

bb10.i4249:                                       ; preds = %bb6.i4248, %bb10.i4249
  %iter.sroa.0.04.i = phi ptr [ %_38.i4250, %bb10.i4249 ], [ %_231.0.i412, %bb6.i4248 ]
  %_38.i4250 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i, i64 4, !dbg !40401
  store i32 %left_phase.i407, ptr %iter.sroa.0.04.i, align 4, !dbg !40403, !alias.scope !40397, !noalias !39040
  %_29.i4251 = icmp eq ptr %_38.i4250, %end_or_len.i, !dbg !40388
  br i1 %_29.i4251, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i4249, !dbg !40391

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit: ; preds = %bb10.i4249, %bb6.i4248, %bb3.i4255
  %663 = getelementptr inbounds nuw i8, ptr %self, i64 416, !dbg !40404
  %_232.0.i414 = load ptr, ptr %663, align 8, !dbg !40404, !alias.scope !39038, !noalias !40405, !nonnull !12, !noundef !12
  %664 = getelementptr inbounds nuw i8, ptr %self, i64 424, !dbg !40404
  %_232.1.i415 = load i64, ptr %664, align 8, !dbg !40404, !alias.scope !39038, !noalias !40405, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !40406), !dbg !40409
  %_4.not.i3183 = icmp eq i64 %_232.1.i415, 0, !dbg !40410
  br i1 %_4.not.i3183, label %panic.i3185, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3186, !dbg !40410

panic.i3185:                                      ; preds = %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !40410, !noalias !40412
  unreachable, !dbg !40410

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3186: ; preds = %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit
  store float %right_prefix.i408, ptr %_232.0.i414, align 4, !dbg !40410, !alias.scope !40406, !noalias !39040
  %_233.0.i416 = load ptr, ptr %77, align 8, !dbg !40413, !alias.scope !39038, !noalias !40405, !nonnull !12, !noundef !12
  %_233.1.i417 = load i64, ptr %78, align 8, !dbg !40413, !alias.scope !39038, !noalias !40405, !noundef !12
  %665 = tail call i1 @llvm.is.constant.i32(i32 %right_phase.i409), !dbg !40414
  br i1 %665, label %bb2.i4265, label %bb6.i4256, !dbg !40414

bb6.i4256:                                        ; preds = %bb2.i4265, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3186
  %end_or_len.idx.i4257 = shl nuw nsw i64 %_233.1.i417, 2, !dbg !40417
  %end_or_len.i4258 = getelementptr inbounds nuw i8, ptr %_233.0.i416, i64 %end_or_len.idx.i4257, !dbg !40417
  %_293.i4259 = icmp eq i64 %_233.1.i417, 0, !dbg !40421
  br i1 %_293.i4259, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4271, label %bb10.i4260, !dbg !40424

bb2.i4265:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3186
  %bytes1.sroa.0.0.zext.i4266 = and i32 %right_phase.i409, 255, !dbg !40425
  %bytes1.sroa.0.0.isplat.i4267 = mul nuw i32 %bytes1.sroa.0.0.zext.i4266, 16843009, !dbg !40425
  %_5.i4268 = icmp eq i32 %right_phase.i409, %bytes1.sroa.0.0.isplat.i4267, !dbg !40426
  br i1 %_5.i4268, label %bb3.i4269, label %bb6.i4256, !dbg !40426

bb3.i4269:                                        ; preds = %bb2.i4265
  %bytes.sroa.0.0.extract.trunc.i4270 = trunc i32 %right_phase.i409 to i8, !dbg !40427
  %666 = shl nuw nsw i64 %_233.1.i417, 2, !dbg !40429
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_233.0.i416, i8 %bytes.sroa.0.0.extract.trunc.i4270, i64 %666, i1 false), !dbg !40429, !alias.scope !40430, !noalias !39040
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4271, !dbg !40433

bb10.i4260:                                       ; preds = %bb6.i4256, %bb10.i4260
  %iter.sroa.0.04.i4261 = phi ptr [ %_38.i4262, %bb10.i4260 ], [ %_233.0.i416, %bb6.i4256 ]
  %_38.i4262 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i4261, i64 4, !dbg !40434
  store i32 %right_phase.i409, ptr %iter.sroa.0.04.i4261, align 4, !dbg !40436, !alias.scope !40430, !noalias !39040
  %_29.i4263 = icmp eq ptr %_38.i4262, %end_or_len.i4258, !dbg !40421
  br i1 %_29.i4263, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4271, label %bb10.i4260, !dbg !40424

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4271: ; preds = %bb10.i4260, %bb6.i4256, %bb3.i4269
  call void @llvm.lifetime.start.p0(ptr nonnull %_152.i15), !dbg !40437, !noalias !39046
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(92) %_152.i15, ptr noundef nonnull align 4 dereferenceable(92) %hot_left.i37, i64 92, i1 false), !dbg !40437, !noalias !39046
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_152.i15, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33) #31, !dbg !40438, !noalias !39040
  call void @llvm.lifetime.end.p0(ptr nonnull %_152.i15), !dbg !40439, !noalias !39046
  call void @llvm.lifetime.start.p0(ptr nonnull %_154.i14), !dbg !40440, !noalias !39046
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(92) %_154.i14, ptr noundef nonnull align 4 dereferenceable(92) %hot_right.i36, i64 92, i1 false), !dbg !40440, !noalias !39046
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_154.i14, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34) #31, !dbg !40441, !noalias !39040
  call void @llvm.lifetime.end.p0(ptr nonnull %_154.i14), !dbg !40442, !noalias !39046
  store i32 %main_cursor.sroa.0.0.i56.lcssa, ptr %_35, align 4, !dbg !40361, !alias.scope !39040, !noalias !39069
  store i32 %ring_cursor.sroa.0.0.i55.lcssa, ptr %545, align 4, !dbg !40363, !alias.scope !39040, !noalias !39069
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i34), !dbg !40443, !noalias !39046
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i35), !dbg !40444, !noalias !39046
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_right.i36), !dbg !40445, !noalias !39046
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i37), !dbg !40446, !noalias !39046
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockfEB2_.exit, !dbg !39033

bb6.i:                                            ; preds = %bb5.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !40447), !dbg !40450
  tail call void @llvm.experimental.noalias.scope.decl(metadata !40451), !dbg !40450
  tail call void @llvm.experimental.noalias.scope.decl(metadata !40453), !dbg !40450
  tail call void @llvm.experimental.noalias.scope.decl(metadata !40455), !dbg !40450
  tail call void @llvm.experimental.noalias.scope.decl(metadata !40457), !dbg !40450
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_left.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #31, !dbg !40459
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_right.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #31, !dbg !40463
  %667 = getelementptr inbounds nuw i8, ptr %self, i64 776, !dbg !40465
  %668 = load i8, ptr %667, align 4, !dbg !40465, !range !17, !alias.scope !40447, !noalias !40469, !noundef !12
  %669 = getelementptr inbounds nuw i8, ptr %self, i64 777, !dbg !40472
  %670 = load i8, ptr %669, align 1, !dbg !40472, !range !17, !alias.scope !40447, !noalias !40469, !noundef !12
  %671 = getelementptr inbounds nuw i8, ptr %self, i64 544, !dbg !40474
  %ring.i = load i64, ptr %671, align 8, !dbg !40474, !alias.scope !40451, !noalias !40476, !noundef !12
  %672 = getelementptr inbounds nuw i8, ptr %self, i64 552, !dbg !40477
  %main.i = load i64, ptr %672, align 8, !dbg !40477, !alias.scope !40451, !noalias !40476, !noundef !12
  %_35.i = load i32, ptr %_35, align 4, !dbg !40479, !alias.scope !40457, !noalias !40481, !noundef !12
  %673 = getelementptr inbounds nuw i8, ptr %self, i64 564, !dbg !40482
  %_36.i = load i32, ptr %673, align 4, !dbg !40482, !alias.scope !40457, !noalias !40481, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i), !dbg !40484, !noalias !40486
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i, i8 0, i64 1024, i1 false), !noalias !40486
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i), !dbg !40487, !noalias !40486
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i, i8 0, i64 1024, i1 false), !noalias !40486
  %_31.i = zext nneg i8 %668 to i32, !dbg !40465
  %.none.i = sub nsw i32 0, %_31.i, !dbg !40489
  %_32.i = zext nneg i8 %670 to i32, !dbg !40472
  %all.sroa.0.0.i = sub nsw i32 0, %_32.i, !dbg !40472
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i), !dbg !40490, !noalias !40486
; call <true_peak_limiter::UniformHot<f32>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotfE3newB5_(ptr noalias noundef align 8 captures(none) dereferenceable(80) %uniform_left.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33, i64 %ring.i, i64 %main.i) #31, !dbg !40492
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_right.i), !dbg !40493, !noalias !40486
  %_32.val3877 = load i64, ptr %671, align 8, !dbg !40495, !noundef !12
  %_32.val3878 = load i64, ptr %672, align 8, !dbg !40495, !noundef !12
; call <true_peak_limiter::UniformHot<f32>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotfE3newB5_(ptr noalias noundef align 8 captures(none) dereferenceable(80) %uniform_right.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34, i64 %_32.val3877, i64 %_32.val3878) #31, !dbg !40495
  %_162.not.i8789 = icmp eq i64 %frames, 0, !dbg !40496
  br i1 %_162.not.i8789, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit, label %bb50.i.lr.ph, !dbg !40496

bb50.i.lr.ph:                                     ; preds = %bb6.i
  %674 = zext i32 %_36.i to i64, !dbg !40482
  %675 = zext i32 %_35.i to i64, !dbg !40479
  %d9.i.i4272 = lshr i64 %frames, 5, !dbg !40506
  %r2.i.i4273 = and i64 %frames, 31, !dbg !40512
  %_19.not.i.i4274 = icmp ne i64 %r2.i.i4273, 0, !dbg !40513
  %676 = zext i1 %_19.not.i.i4274 to i64, !dbg !40513
  %yield_count.sroa.0.0.i.i4275 = add nuw nsw i64 %d9.i.i4272, %676, !dbg !40513
  %history.i38.i.sroa.7.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 4
  %history.i38.i.sroa.10.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 8
  %history.i38.i.sroa.13.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 12
  %history.i38.i.sroa.16.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 16
  %history.i38.i.sroa.19.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 20
  %history.i38.i.sroa.22.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 24
  %history.i38.i.sroa.26.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 28
  %history.i38.i.sroa.29.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 32
  %history.i38.i.sroa.32.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 36
  %history.i38.i.sroa.35.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 40
  %history.i38.i.sroa.38.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 44
  %677 = getelementptr inbounds nuw i8, ptr %self, i64 588
  %678 = getelementptr inbounds nuw i8, ptr %self, i64 592
  %679 = getelementptr inbounds nuw i8, ptr %self, i64 596
  %row1.i.i.i74.i = getelementptr inbounds nuw i8, ptr %self, i64 600
  %680 = getelementptr inbounds nuw i8, ptr %self, i64 604
  %681 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %682 = getelementptr inbounds nuw i8, ptr %self, i64 612
  %row3.i.i.i88.i = getelementptr inbounds nuw i8, ptr %self, i64 616
  %683 = getelementptr inbounds nuw i8, ptr %self, i64 620
  %684 = getelementptr inbounds nuw i8, ptr %self, i64 624
  %685 = getelementptr inbounds nuw i8, ptr %self, i64 628
  %row5.i.i.i102.i = getelementptr inbounds nuw i8, ptr %self, i64 632
  %686 = getelementptr inbounds nuw i8, ptr %self, i64 636
  %687 = getelementptr inbounds nuw i8, ptr %self, i64 640
  %688 = getelementptr inbounds nuw i8, ptr %self, i64 644
  %row7.i.i.i116.i = getelementptr inbounds nuw i8, ptr %self, i64 648
  %689 = getelementptr inbounds nuw i8, ptr %self, i64 652
  %690 = getelementptr inbounds nuw i8, ptr %self, i64 656
  %691 = getelementptr inbounds nuw i8, ptr %self, i64 660
  %row9.i.i.i130.i = getelementptr inbounds nuw i8, ptr %self, i64 664
  %692 = getelementptr inbounds nuw i8, ptr %self, i64 668
  %693 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %694 = getelementptr inbounds nuw i8, ptr %self, i64 676
  %row11.i.i.i144.i = getelementptr inbounds nuw i8, ptr %self, i64 680
  %695 = getelementptr inbounds nuw i8, ptr %self, i64 684
  %696 = getelementptr inbounds nuw i8, ptr %self, i64 688
  %697 = getelementptr inbounds nuw i8, ptr %self, i64 692
  %row13.i.i.i158.i = getelementptr inbounds nuw i8, ptr %self, i64 696
  %698 = getelementptr inbounds nuw i8, ptr %self, i64 700
  %699 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %700 = getelementptr inbounds nuw i8, ptr %self, i64 708
  %row15.i.i.i172.i = getelementptr inbounds nuw i8, ptr %self, i64 712
  %701 = getelementptr inbounds nuw i8, ptr %self, i64 716
  %702 = getelementptr inbounds nuw i8, ptr %self, i64 720
  %703 = getelementptr inbounds nuw i8, ptr %self, i64 724
  %row17.i.i.i186.i = getelementptr inbounds nuw i8, ptr %self, i64 728
  %704 = getelementptr inbounds nuw i8, ptr %self, i64 732
  %705 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %706 = getelementptr inbounds nuw i8, ptr %self, i64 740
  %row19.i.i.i200.i = getelementptr inbounds nuw i8, ptr %self, i64 744
  %707 = getelementptr inbounds nuw i8, ptr %self, i64 748
  %708 = getelementptr inbounds nuw i8, ptr %self, i64 752
  %709 = getelementptr inbounds nuw i8, ptr %self, i64 756
  %row21.i.i.i214.i = getelementptr inbounds nuw i8, ptr %self, i64 760
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
  %_65.i.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 56
  %_65.i.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 64
  %714 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 48
  %_66.i.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 56
  %_66.i.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 64
  %_109.i = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 48
  %_110.i = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 64
  %_114.i = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 48
  %_115.i = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 64
  %_9.i3531 = add nsw i32 %_31.i, -1
  %715 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 8
  %_21.i269.i = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 72
  %_22.i270.i = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 76
  %716 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 16
  %717 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 24
  %718 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 84
  %719 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 88
  %720 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 80
  %721 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 40
  %722 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 32
  %_9.i3511 = add nsw i32 %_32.i, -1
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
  %history.i38.i.sroa.7.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i38.i.sroa.7.0.hot_left.i.sroa_idx, align 4
  %history.i38.i.sroa.10.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i38.i.sroa.10.0.hot_left.i.sroa_idx, align 4
  %history.i38.i.sroa.13.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i38.i.sroa.13.0.hot_left.i.sroa_idx, align 4
  %history.i38.i.sroa.16.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i38.i.sroa.16.0.hot_left.i.sroa_idx, align 4
  %history.i38.i.sroa.19.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i38.i.sroa.19.0.hot_left.i.sroa_idx, align 4
  %history.i38.i.sroa.22.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i38.i.sroa.22.0.hot_left.i.sroa_idx, align 4
  %history.i38.i.sroa.26.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i38.i.sroa.26.0.hot_left.i.sroa_idx, align 4
  %history.i38.i.sroa.29.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i38.i.sroa.29.0.hot_left.i.sroa_idx, align 4
  %history.i38.i.sroa.32.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i38.i.sroa.32.0.hot_left.i.sroa_idx, align 4
  %history.i38.i.sroa.35.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i38.i.sroa.35.0.hot_left.i.sroa_idx, align 4
  %history.i38.i.sroa.38.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i38.i.sroa.38.0.hot_left.i.sroa_idx, align 4
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
  %_22.i270.i.promoted = load i32, ptr %_22.i270.i, align 4
  %_21.i269.i.promoted13664 = load float, ptr %_21.i269.i, align 4
  %_22.i.i.promoted = load i32, ptr %_22.i.i, align 4
  %_21.i.i.promoted13707 = load float, ptr %_21.i.i, align 4
  %.promoted13729 = load float, ptr %718, align 4
  %.promoted13732 = load float, ptr %720, align 4
  %.promoted13735 = load float, ptr %726, align 4
  %.promoted13738 = load float, ptr %728, align 4
  br label %bb50.i, !dbg !40496

bb19.i.bb15.i.loopexit_crit_edge:                 ; preds = %bb32.i
  store float %_0.i2899.lcssa1138213476, ptr %718, align 4
  store float %_0.i3273.lcssa1139813494, ptr %720, align 4
  store float %_0.i2895.lcssa1142213512, ptr %726, align 4
  store float %_0.i3269.lcssa1142313530, ptr %728, align 4
  br label %bb15.i.loopexit, !dbg !40514

bb15.i.loopexit:                                  ; preds = %bb19.i.bb15.i.loopexit_crit_edge, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i
  %_0.i3269.lcssa1142313530.lcssa13739 = phi float [ %_0.i3269.lcssa1142313530, %bb19.i.bb15.i.loopexit_crit_edge ], [ %_0.i3269.lcssa1142313530.lcssa13740, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i ]
  %_0.i2895.lcssa1142213512.lcssa13736 = phi float [ %_0.i2895.lcssa1142213512, %bb19.i.bb15.i.loopexit_crit_edge ], [ %_0.i2895.lcssa1142213512.lcssa13737, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i ]
  %_0.i3273.lcssa1139813494.lcssa13733 = phi float [ %_0.i3273.lcssa1139813494, %bb19.i.bb15.i.loopexit_crit_edge ], [ %_0.i3273.lcssa1139813494.lcssa13734, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i ]
  %_0.i2899.lcssa1138213476.lcssa13730 = phi float [ %_0.i2899.lcssa1138213476, %bb19.i.bb15.i.loopexit_crit_edge ], [ %_0.i2899.lcssa1138213476.lcssa13731, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i ]
  %running.sroa.0.0.i1210.lcssa1141713564.lcssa13708 = phi float [ %running.sroa.0.0.i1210.lcssa1141713564, %bb19.i.bb15.i.loopexit_crit_edge ], [ %running.sroa.0.0.i1210.lcssa1141713564.lcssa13709, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i ]
  %storemerge.i1215.lcssa86158751.lcssa13686 = phi i32 [ %storemerge.i1215.lcssa86158751, %bb19.i.bb15.i.loopexit_crit_edge ], [ %storemerge.i1215.lcssa86158751.lcssa13687, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i ]
  %running.sroa.0.0.i1241.lcssa1136313547.lcssa13665 = phi float [ %running.sroa.0.0.i1241.lcssa1136313547, %bb19.i.bb15.i.loopexit_crit_edge ], [ %running.sroa.0.0.i1241.lcssa1136313547.lcssa13666, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i ]
  %storemerge.i1246.lcssa85038712.lcssa13643 = phi i32 [ %storemerge.i1246.lcssa85038712, %bb19.i.bb15.i.loopexit_crit_edge ], [ %storemerge.i1246.lcssa85038712.lcssa13644, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i ]
  %ring_cursor.sroa.0.1.i.lcssa = phi i64 [ %ring_cursor.sroa.0.2.i, %bb19.i.bb15.i.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i8790, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i ], !dbg !40518
  %main_cursor.sroa.0.1.i.lcssa = phi i64 [ %main_cursor.sroa.0.2.i, %bb19.i.bb15.i.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i8791, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i ], !dbg !40519
  %_162.not.i = icmp eq i64 %732, 0, !dbg !40496
  %indvars.iv.next11146 = add i64 %indvars.iv11145, -32, !dbg !40496
  br i1 %_162.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit, label %bb50.i, !dbg !40496

bb50.i:                                           ; preds = %bb50.i.lr.ph, %bb15.i.loopexit
  %_0.i3269.lcssa1142313530.lcssa13740 = phi float [ %.promoted13738, %bb50.i.lr.ph ], [ %_0.i3269.lcssa1142313530.lcssa13739, %bb15.i.loopexit ]
  %_0.i2895.lcssa1142213512.lcssa13737 = phi float [ %.promoted13735, %bb50.i.lr.ph ], [ %_0.i2895.lcssa1142213512.lcssa13736, %bb15.i.loopexit ]
  %_0.i3273.lcssa1139813494.lcssa13734 = phi float [ %.promoted13732, %bb50.i.lr.ph ], [ %_0.i3273.lcssa1139813494.lcssa13733, %bb15.i.loopexit ]
  %_0.i2899.lcssa1138213476.lcssa13731 = phi float [ %.promoted13729, %bb50.i.lr.ph ], [ %_0.i2899.lcssa1138213476.lcssa13730, %bb15.i.loopexit ]
  %running.sroa.0.0.i1210.lcssa1141713564.lcssa13709 = phi float [ %_21.i.i.promoted13707, %bb50.i.lr.ph ], [ %running.sroa.0.0.i1210.lcssa1141713564.lcssa13708, %bb15.i.loopexit ]
  %storemerge.i1215.lcssa86158751.lcssa13687 = phi i32 [ %_22.i.i.promoted, %bb50.i.lr.ph ], [ %storemerge.i1215.lcssa86158751.lcssa13686, %bb15.i.loopexit ]
  %running.sroa.0.0.i1241.lcssa1136313547.lcssa13666 = phi float [ %_21.i269.i.promoted13664, %bb50.i.lr.ph ], [ %running.sroa.0.0.i1241.lcssa1136313547.lcssa13665, %bb15.i.loopexit ]
  %storemerge.i1246.lcssa85038712.lcssa13644 = phi i32 [ %_22.i270.i.promoted, %bb50.i.lr.ph ], [ %storemerge.i1246.lcssa85038712.lcssa13643, %bb15.i.loopexit ]
  %history.i.i.sroa.38.0.lcssa13642 = phi float [ %history.i.i.sroa.38.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.38.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.35.0.lcssa13641 = phi float [ %history.i.i.sroa.35.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.35.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.32.0.lcssa13640 = phi float [ %history.i.i.sroa.32.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.32.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.29.0.lcssa13639 = phi float [ %history.i.i.sroa.29.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.29.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.26.0.lcssa13638 = phi float [ %history.i.i.sroa.26.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.26.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.22.0.lcssa13637 = phi float [ %history.i.i.sroa.22.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.22.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.19.0.lcssa13636 = phi float [ %history.i.i.sroa.19.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.19.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.16.0.lcssa13635 = phi float [ %history.i.i.sroa.16.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.16.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.13.0.lcssa13634 = phi float [ %history.i.i.sroa.13.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.13.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.10.0.lcssa13633 = phi float [ %history.i.i.sroa.10.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.10.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.7.0.lcssa13632 = phi float [ %history.i.i.sroa.7.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.7.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.0.0.lcssa13612 = phi float [ %hot_right.i.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i38.i.sroa.38.0.lcssa13611 = phi float [ %history.i38.i.sroa.38.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i38.i.sroa.38.0.lcssa, %bb15.i.loopexit ]
  %history.i38.i.sroa.35.0.lcssa13610 = phi float [ %history.i38.i.sroa.35.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i38.i.sroa.35.0.lcssa, %bb15.i.loopexit ]
  %history.i38.i.sroa.32.0.lcssa13609 = phi float [ %history.i38.i.sroa.32.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i38.i.sroa.32.0.lcssa, %bb15.i.loopexit ]
  %history.i38.i.sroa.29.0.lcssa13608 = phi float [ %history.i38.i.sroa.29.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i38.i.sroa.29.0.lcssa, %bb15.i.loopexit ]
  %history.i38.i.sroa.26.0.lcssa13607 = phi float [ %history.i38.i.sroa.26.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i38.i.sroa.26.0.lcssa, %bb15.i.loopexit ]
  %history.i38.i.sroa.22.0.lcssa13606 = phi float [ %history.i38.i.sroa.22.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i38.i.sroa.22.0.lcssa, %bb15.i.loopexit ]
  %history.i38.i.sroa.19.0.lcssa13605 = phi float [ %history.i38.i.sroa.19.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i38.i.sroa.19.0.lcssa, %bb15.i.loopexit ]
  %history.i38.i.sroa.16.0.lcssa13604 = phi float [ %history.i38.i.sroa.16.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i38.i.sroa.16.0.lcssa, %bb15.i.loopexit ]
  %history.i38.i.sroa.13.0.lcssa13603 = phi float [ %history.i38.i.sroa.13.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i38.i.sroa.13.0.lcssa, %bb15.i.loopexit ]
  %history.i38.i.sroa.10.0.lcssa13602 = phi float [ %history.i38.i.sroa.10.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i38.i.sroa.10.0.lcssa, %bb15.i.loopexit ]
  %history.i38.i.sroa.7.0.lcssa13601 = phi float [ %history.i38.i.sroa.7.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i38.i.sroa.7.0.lcssa, %bb15.i.loopexit ]
  %history.i38.i.sroa.0.0.lcssa13581 = phi float [ %hot_left.i.promoted, %bb50.i.lr.ph ], [ %history.i38.i.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %indvars.iv11145 = phi i64 [ %frames, %bb50.i.lr.ph ], [ %indvars.iv.next11146, %bb15.i.loopexit ]
  %iter2.sroa.0.0.i8793 = phi i64 [ %yield_count.sroa.0.0.i.i4275, %bb50.i.lr.ph ], [ %732, %bb15.i.loopexit ]
  %iter1.sroa.0.0.i8792 = phi i64 [ 0, %bb50.i.lr.ph ], [ %731, %bb15.i.loopexit ]
  %main_cursor.sroa.0.0.i8791 = phi i64 [ %675, %bb50.i.lr.ph ], [ %main_cursor.sroa.0.1.i.lcssa, %bb15.i.loopexit ]
  %ring_cursor.sroa.0.0.i8790 = phi i64 [ %674, %bb50.i.lr.ph ], [ %ring_cursor.sroa.0.1.i.lcssa, %bb15.i.loopexit ]
  %umin11165 = call i64 @llvm.umin.i64(i64 %indvars.iv11145, i64 32), !dbg !40520
  %umax11151 = call i64 @llvm.umax.i64(i64 %umin11165, i64 1), !dbg !40520
  %731 = add i64 %iter1.sroa.0.0.i8792, 32, !dbg !40520
  %732 = add i64 %iter2.sroa.0.0.i8793, -1, !dbg !40524
  %_46.i = sub i64 %frames, %iter1.sroa.0.0.i8792, !dbg !40525
  %..i4276 = tail call noundef i64 @llvm.umin.i64(i64 %_46.i, i64 32), !dbg !40526
  %_20.i41.i8420.not = icmp eq i64 %frames, %iter1.sroa.0.0.i8792, !dbg !40530
  br i1 %_20.i41.i8420.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit242.i, label %bb5.i42.i.lr.ph, !dbg !40535

bb5.i42.i.lr.ph:                                  ; preds = %bb50.i
  %_11.i.i.i62.i = load float, ptr %_31, align 4
  %_14.i.i.i65.i = load float, ptr %677, align 4
  %_17.i.i.i68.i = load float, ptr %678, align 4
  %_20.i.i.i71.i = load float, ptr %679, align 4
  %_25.i.i.i76.i = load float, ptr %row1.i.i.i74.i, align 4
  %_28.i.i.i79.i = load float, ptr %680, align 4
  %_31.i.i.i82.i = load float, ptr %681, align 4
  %_34.i.i.i85.i = load float, ptr %682, align 4
  %_39.i.i.i90.i = load float, ptr %row3.i.i.i88.i, align 4
  %_42.i.i.i93.i = load float, ptr %683, align 4
  %_45.i.i.i96.i = load float, ptr %684, align 4
  %_48.i.i.i99.i = load float, ptr %685, align 4
  %_53.i.i.i104.i = load float, ptr %row5.i.i.i102.i, align 4
  %_56.i.i.i107.i = load float, ptr %686, align 4
  %_59.i.i.i110.i = load float, ptr %687, align 4
  %_62.i.i.i113.i = load float, ptr %688, align 4
  %_67.i.i.i118.i = load float, ptr %row7.i.i.i116.i, align 4
  %_70.i.i.i121.i = load float, ptr %689, align 4
  %_73.i.i.i124.i = load float, ptr %690, align 4
  %_76.i.i.i127.i = load float, ptr %691, align 4
  %_81.i.i.i132.i = load float, ptr %row9.i.i.i130.i, align 4
  %_84.i.i.i135.i = load float, ptr %692, align 4
  %_87.i.i.i138.i = load float, ptr %693, align 4
  %_90.i.i.i141.i = load float, ptr %694, align 4
  %_95.i.i.i146.i = load float, ptr %row11.i.i.i144.i, align 4
  %_98.i.i.i149.i = load float, ptr %695, align 4
  %_101.i.i.i152.i = load float, ptr %696, align 4
  %_104.i.i.i155.i = load float, ptr %697, align 4
  %_109.i.i.i160.i = load float, ptr %row13.i.i.i158.i, align 4
  %_112.i.i.i163.i = load float, ptr %698, align 4
  %_115.i.i.i166.i = load float, ptr %699, align 4
  %_118.i.i.i169.i = load float, ptr %700, align 4
  %_123.i.i.i174.i = load float, ptr %row15.i.i.i172.i, align 4
  %_126.i.i.i177.i = load float, ptr %701, align 4
  %_129.i.i.i180.i = load float, ptr %702, align 4
  %_132.i.i.i183.i = load float, ptr %703, align 4
  %_137.i.i.i188.i = load float, ptr %row17.i.i.i186.i, align 4
  %_140.i.i.i191.i = load float, ptr %704, align 4
  %_143.i.i.i194.i = load float, ptr %705, align 4
  %_146.i.i.i197.i = load float, ptr %706, align 4
  %_151.i.i.i202.i = load float, ptr %row19.i.i.i200.i, align 4
  %_154.i.i.i205.i = load float, ptr %707, align 4
  %_157.i.i.i208.i = load float, ptr %708, align 4
  %_160.i.i.i211.i = load float, ptr %709, align 4
  %_165.i.i.i216.i = load float, ptr %row21.i.i.i214.i, align 4
  %_168.i.i.i219.i = load float, ptr %710, align 4
  %_171.i.i.i222.i = load float, ptr %711, align 4
  %_174.i.i.i225.i = load float, ptr %712, align 4
  br label %bb5.i42.i, !dbg !40535

bb5.i42.i:                                        ; preds = %bb5.i42.i.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075
  %iter.sroa.0.0.i40.i8432 = phi i64 [ 0, %bb5.i42.i.lr.ph ], [ %733, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075 ]
  %history.i38.i.sroa.35.08431 = phi float [ %history.i38.i.sroa.35.0.lcssa13610, %bb5.i42.i.lr.ph ], [ %history.i38.i.sroa.32.08430, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075 ]
  %history.i38.i.sroa.32.08430 = phi float [ %history.i38.i.sroa.32.0.lcssa13609, %bb5.i42.i.lr.ph ], [ %history.i38.i.sroa.29.08429, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075 ]
  %history.i38.i.sroa.29.08429 = phi float [ %history.i38.i.sroa.29.0.lcssa13608, %bb5.i42.i.lr.ph ], [ %history.i38.i.sroa.26.08428, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075 ]
  %history.i38.i.sroa.26.08428 = phi float [ %history.i38.i.sroa.26.0.lcssa13607, %bb5.i42.i.lr.ph ], [ %history.i38.i.sroa.22.08427, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075 ]
  %history.i38.i.sroa.22.08427 = phi float [ %history.i38.i.sroa.22.0.lcssa13606, %bb5.i42.i.lr.ph ], [ %history.i38.i.sroa.19.08426, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075 ]
  %history.i38.i.sroa.19.08426 = phi float [ %history.i38.i.sroa.19.0.lcssa13605, %bb5.i42.i.lr.ph ], [ %history.i38.i.sroa.16.08425, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075 ]
  %history.i38.i.sroa.16.08425 = phi float [ %history.i38.i.sroa.16.0.lcssa13604, %bb5.i42.i.lr.ph ], [ %history.i38.i.sroa.13.08424, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075 ]
  %history.i38.i.sroa.13.08424 = phi float [ %history.i38.i.sroa.13.0.lcssa13603, %bb5.i42.i.lr.ph ], [ %history.i38.i.sroa.10.08423, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075 ]
  %history.i38.i.sroa.10.08423 = phi float [ %history.i38.i.sroa.10.0.lcssa13602, %bb5.i42.i.lr.ph ], [ %history.i38.i.sroa.7.08422, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075 ]
  %history.i38.i.sroa.7.08422 = phi float [ %history.i38.i.sroa.7.0.lcssa13601, %bb5.i42.i.lr.ph ], [ %history.i38.i.sroa.0.08421, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075 ]
  %history.i38.i.sroa.0.08421 = phi float [ %history.i38.i.sroa.0.0.lcssa13581, %bb5.i42.i.lr.ph ], [ %_0.i3073, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075 ]
  %733 = add nuw nsw i64 %iter.sroa.0.0.i40.i8432, 1, !dbg !40536
  %_11.i43.i = add nuw nsw i64 %iter.sroa.0.0.i40.i8432, %iter1.sroa.0.0.i8792, !dbg !40539
  %_24.i44.i = icmp ugt i64 %_11.i43.i, %left_io.1, !dbg !40540
  br i1 %_24.i44.i, label %bb7.i241.i, label %bb8.i45.i, !dbg !40540, !prof !639

bb8.i45.i:                                        ; preds = %bb5.i42.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !40543), !dbg !40546
  %_3.not.i3071 = icmp eq i64 %left_io.1, %_11.i43.i, !dbg !40547
  br i1 %_3.not.i3071, label %panic.i3074, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075, !dbg !40547

panic.i3074:                                      ; preds = %bb8.i45.i
  store float %history.i38.i.sroa.0.0.lcssa13581, ptr %hot_left.i, align 4, !dbg !40549
  store float %history.i.i.sroa.0.0.lcssa13612, ptr %hot_right.i, align 4, !dbg !40550
  store i32 %storemerge.i1246.lcssa85038712.lcssa13644, ptr %_22.i270.i, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313547.lcssa13666, ptr %_21.i269.i, align 4
  store i32 %storemerge.i1215.lcssa86158751.lcssa13687, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa1141713564.lcssa13709, ptr %_21.i.i, align 4
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !40547, !noalias !40552
  unreachable, !dbg !40547

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075: ; preds = %bb8.i45.i
  %_31.i47.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %_11.i43.i, !dbg !40556
  %_0.i3073 = load float, ptr %_31.i47.i, align 4, !dbg !40547, !alias.scope !40543, !noalias !40558, !noundef !12
  %734 = tail call noundef float @llvm.fabs.f32(float %history.i38.i.sroa.19.08426), !dbg !40559
  %_0.i2805 = fmul float %_0.i3073, %_11.i.i.i62.i, !dbg !40562
  %_0.i2369 = fadd float %_0.i2805, 0.000000e+00, !dbg !40565
  %_0.i2804 = fmul float %_0.i3073, %_14.i.i.i65.i, !dbg !40567
  %_0.i2368 = fadd float %_0.i2804, 0.000000e+00, !dbg !40569
  %_0.i2803 = fmul float %_0.i3073, %_17.i.i.i68.i, !dbg !40571
  %_0.i2367 = fadd float %_0.i2803, 0.000000e+00, !dbg !40573
  %_0.i2802 = fmul float %_0.i3073, %_20.i.i.i71.i, !dbg !40575
  %_0.i2366 = fadd float %_0.i2802, 0.000000e+00, !dbg !40577
  %_0.i2801 = fmul float %history.i38.i.sroa.0.08421, %_25.i.i.i76.i, !dbg !40579
  %_0.i2365 = fadd float %_0.i2369, %_0.i2801, !dbg !40581
  %_0.i2800 = fmul float %history.i38.i.sroa.0.08421, %_28.i.i.i79.i, !dbg !40583
  %_0.i2364 = fadd float %_0.i2368, %_0.i2800, !dbg !40585
  %_0.i2799 = fmul float %history.i38.i.sroa.0.08421, %_31.i.i.i82.i, !dbg !40587
  %_0.i2363 = fadd float %_0.i2367, %_0.i2799, !dbg !40589
  %_0.i2798 = fmul float %history.i38.i.sroa.0.08421, %_34.i.i.i85.i, !dbg !40591
  %_0.i2362 = fadd float %_0.i2366, %_0.i2798, !dbg !40593
  %_0.i2797 = fmul float %history.i38.i.sroa.7.08422, %_39.i.i.i90.i, !dbg !40595
  %_0.i2361 = fadd float %_0.i2365, %_0.i2797, !dbg !40597
  %_0.i2796 = fmul float %history.i38.i.sroa.7.08422, %_42.i.i.i93.i, !dbg !40599
  %_0.i2360 = fadd float %_0.i2364, %_0.i2796, !dbg !40601
  %_0.i2795 = fmul float %history.i38.i.sroa.7.08422, %_45.i.i.i96.i, !dbg !40603
  %_0.i2359 = fadd float %_0.i2363, %_0.i2795, !dbg !40605
  %_0.i2794 = fmul float %history.i38.i.sroa.7.08422, %_48.i.i.i99.i, !dbg !40607
  %_0.i2358 = fadd float %_0.i2362, %_0.i2794, !dbg !40609
  %_0.i2793 = fmul float %history.i38.i.sroa.10.08423, %_53.i.i.i104.i, !dbg !40611
  %_0.i2357 = fadd float %_0.i2361, %_0.i2793, !dbg !40613
  %_0.i2792 = fmul float %history.i38.i.sroa.10.08423, %_56.i.i.i107.i, !dbg !40615
  %_0.i2356 = fadd float %_0.i2360, %_0.i2792, !dbg !40617
  %_0.i2791 = fmul float %history.i38.i.sroa.10.08423, %_59.i.i.i110.i, !dbg !40619
  %_0.i2355 = fadd float %_0.i2359, %_0.i2791, !dbg !40621
  %_0.i2790 = fmul float %history.i38.i.sroa.10.08423, %_62.i.i.i113.i, !dbg !40623
  %_0.i2354 = fadd float %_0.i2358, %_0.i2790, !dbg !40625
  %_0.i2789 = fmul float %history.i38.i.sroa.13.08424, %_67.i.i.i118.i, !dbg !40627
  %_0.i2353 = fadd float %_0.i2357, %_0.i2789, !dbg !40629
  %_0.i2788 = fmul float %history.i38.i.sroa.13.08424, %_70.i.i.i121.i, !dbg !40631
  %_0.i2352 = fadd float %_0.i2356, %_0.i2788, !dbg !40633
  %_0.i2787 = fmul float %history.i38.i.sroa.13.08424, %_73.i.i.i124.i, !dbg !40635
  %_0.i2351 = fadd float %_0.i2355, %_0.i2787, !dbg !40637
  %_0.i2786 = fmul float %history.i38.i.sroa.13.08424, %_76.i.i.i127.i, !dbg !40639
  %_0.i2350 = fadd float %_0.i2354, %_0.i2786, !dbg !40641
  %_0.i2785 = fmul float %history.i38.i.sroa.16.08425, %_81.i.i.i132.i, !dbg !40643
  %_0.i2349 = fadd float %_0.i2353, %_0.i2785, !dbg !40645
  %_0.i2784 = fmul float %history.i38.i.sroa.16.08425, %_84.i.i.i135.i, !dbg !40647
  %_0.i2348 = fadd float %_0.i2352, %_0.i2784, !dbg !40649
  %_0.i2783 = fmul float %history.i38.i.sroa.16.08425, %_87.i.i.i138.i, !dbg !40651
  %_0.i2347 = fadd float %_0.i2351, %_0.i2783, !dbg !40653
  %_0.i2782 = fmul float %history.i38.i.sroa.16.08425, %_90.i.i.i141.i, !dbg !40655
  %_0.i2346 = fadd float %_0.i2350, %_0.i2782, !dbg !40657
  %_0.i2781 = fmul float %history.i38.i.sroa.19.08426, %_95.i.i.i146.i, !dbg !40659
  %_0.i2345 = fadd float %_0.i2349, %_0.i2781, !dbg !40661
  %_0.i2780 = fmul float %history.i38.i.sroa.19.08426, %_98.i.i.i149.i, !dbg !40663
  %_0.i2344 = fadd float %_0.i2348, %_0.i2780, !dbg !40665
  %_0.i2779 = fmul float %history.i38.i.sroa.19.08426, %_101.i.i.i152.i, !dbg !40667
  %_0.i2343 = fadd float %_0.i2347, %_0.i2779, !dbg !40669
  %_0.i2778 = fmul float %history.i38.i.sroa.19.08426, %_104.i.i.i155.i, !dbg !40671
  %_0.i2342 = fadd float %_0.i2346, %_0.i2778, !dbg !40673
  %_0.i2777 = fmul float %history.i38.i.sroa.22.08427, %_109.i.i.i160.i, !dbg !40675
  %_0.i2341 = fadd float %_0.i2345, %_0.i2777, !dbg !40677
  %_0.i2776 = fmul float %history.i38.i.sroa.22.08427, %_112.i.i.i163.i, !dbg !40679
  %_0.i2340 = fadd float %_0.i2344, %_0.i2776, !dbg !40681
  %_0.i2775 = fmul float %history.i38.i.sroa.22.08427, %_115.i.i.i166.i, !dbg !40683
  %_0.i2339 = fadd float %_0.i2343, %_0.i2775, !dbg !40685
  %_0.i2774 = fmul float %history.i38.i.sroa.22.08427, %_118.i.i.i169.i, !dbg !40687
  %_0.i2338 = fadd float %_0.i2342, %_0.i2774, !dbg !40689
  %_0.i2773 = fmul float %history.i38.i.sroa.26.08428, %_123.i.i.i174.i, !dbg !40691
  %_0.i2337 = fadd float %_0.i2341, %_0.i2773, !dbg !40693
  %_0.i2772 = fmul float %history.i38.i.sroa.26.08428, %_126.i.i.i177.i, !dbg !40695
  %_0.i2336 = fadd float %_0.i2340, %_0.i2772, !dbg !40697
  %_0.i2771 = fmul float %history.i38.i.sroa.26.08428, %_129.i.i.i180.i, !dbg !40699
  %_0.i2335 = fadd float %_0.i2339, %_0.i2771, !dbg !40701
  %_0.i2770 = fmul float %history.i38.i.sroa.26.08428, %_132.i.i.i183.i, !dbg !40703
  %_0.i2334 = fadd float %_0.i2338, %_0.i2770, !dbg !40705
  %_0.i2769 = fmul float %history.i38.i.sroa.29.08429, %_137.i.i.i188.i, !dbg !40707
  %_0.i2333 = fadd float %_0.i2337, %_0.i2769, !dbg !40709
  %_0.i2768 = fmul float %history.i38.i.sroa.29.08429, %_140.i.i.i191.i, !dbg !40711
  %_0.i2332 = fadd float %_0.i2336, %_0.i2768, !dbg !40713
  %_0.i2767 = fmul float %history.i38.i.sroa.29.08429, %_143.i.i.i194.i, !dbg !40715
  %_0.i2331 = fadd float %_0.i2335, %_0.i2767, !dbg !40717
  %_0.i2766 = fmul float %history.i38.i.sroa.29.08429, %_146.i.i.i197.i, !dbg !40719
  %_0.i2330 = fadd float %_0.i2334, %_0.i2766, !dbg !40721
  %_0.i2765 = fmul float %history.i38.i.sroa.32.08430, %_151.i.i.i202.i, !dbg !40723
  %_0.i2329 = fadd float %_0.i2333, %_0.i2765, !dbg !40725
  %_0.i2764 = fmul float %history.i38.i.sroa.32.08430, %_154.i.i.i205.i, !dbg !40727
  %_0.i2328 = fadd float %_0.i2332, %_0.i2764, !dbg !40729
  %_0.i2763 = fmul float %history.i38.i.sroa.32.08430, %_157.i.i.i208.i, !dbg !40731
  %_0.i2327 = fadd float %_0.i2331, %_0.i2763, !dbg !40733
  %_0.i2762 = fmul float %history.i38.i.sroa.32.08430, %_160.i.i.i211.i, !dbg !40735
  %_0.i2326 = fadd float %_0.i2330, %_0.i2762, !dbg !40737
  %_0.i2761 = fmul float %history.i38.i.sroa.35.08431, %_165.i.i.i216.i, !dbg !40739
  %_0.i2325 = fadd float %_0.i2329, %_0.i2761, !dbg !40741
  %_0.i2760 = fmul float %history.i38.i.sroa.35.08431, %_168.i.i.i219.i, !dbg !40743
  %_0.i2324 = fadd float %_0.i2328, %_0.i2760, !dbg !40745
  %_0.i2759 = fmul float %history.i38.i.sroa.35.08431, %_171.i.i.i222.i, !dbg !40747
  %_0.i2323 = fadd float %_0.i2327, %_0.i2759, !dbg !40749
  %_0.i2758 = fmul float %history.i38.i.sroa.35.08431, %_174.i.i.i225.i, !dbg !40751
  %_0.i2322 = fadd float %_0.i2326, %_0.i2758, !dbg !40753
  %735 = tail call noundef float @llvm.fabs.f32(float %_0.i2325), !dbg !40755
  %_3.i.i3720.inv = fcmp ogt float %734, %735, !dbg !40757
  %_4.i.i3727.v = select i1 %_3.i.i3720.inv, float %734, float %735, !dbg !40757
  %736 = tail call noundef float @llvm.fabs.f32(float %_0.i2324), !dbg !40755
  %_3.i.i3720.inv.1 = fcmp ogt float %_4.i.i3727.v, %736, !dbg !40757
  %_4.i.i3727.v.1 = select i1 %_3.i.i3720.inv.1, float %_4.i.i3727.v, float %736, !dbg !40757
  %737 = tail call noundef float @llvm.fabs.f32(float %_0.i2323), !dbg !40755
  %_3.i.i3720.inv.2 = fcmp ogt float %_4.i.i3727.v.1, %737, !dbg !40757
  %_4.i.i3727.v.2 = select i1 %_3.i.i3720.inv.2, float %_4.i.i3727.v.1, float %737, !dbg !40757
  %738 = tail call noundef float @llvm.fabs.f32(float %_0.i2322), !dbg !40755
  %_3.i.i3720.inv.3 = fcmp ogt float %_4.i.i3727.v.2, %738, !dbg !40757
  %_4.i.i3727.v.3 = select i1 %_3.i.i3720.inv.3, float %_4.i.i3727.v.2, float %738, !dbg !40757
  %_39.i236.i = getelementptr inbounds nuw float, ptr %peaks_left.i, i64 %iter.sroa.0.0.i40.i8432, !dbg !40760
  store float %_4.i.i3727.v.3, ptr %_39.i236.i, align 4, !dbg !40765, !alias.scope !40767, !noalias !40558
  %exitcond11149.not = icmp eq i64 %733, %umax11151, !dbg !40530
  br i1 %exitcond11149.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit242.i, label %bb5.i42.i, !dbg !40535

bb7.i241.i:                                       ; preds = %bb5.i42.i
  store float %history.i38.i.sroa.0.0.lcssa13581, ptr %hot_left.i, align 4, !dbg !40549
  store float %history.i.i.sroa.0.0.lcssa13612, ptr %hot_right.i, align 4, !dbg !40550
  store i32 %storemerge.i1246.lcssa85038712.lcssa13644, ptr %_22.i270.i, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313547.lcssa13666, ptr %_21.i269.i, align 4
  store i32 %storemerge.i1215.lcssa86158751.lcssa13687, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa1141713564.lcssa13709, ptr %_21.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_11.i43.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_283c8d9f84e10f29a75d8e6c3a347bea) #30, !dbg !40770, !noalias !40558
  unreachable, !dbg !40770

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit242.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075, %bb50.i
  %history.i38.i.sroa.0.0.lcssa = phi float [ %history.i38.i.sroa.0.0.lcssa13581, %bb50.i ], [ %_0.i3073, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075 ], !dbg !40549
  %history.i38.i.sroa.7.0.lcssa = phi float [ %history.i38.i.sroa.7.0.lcssa13601, %bb50.i ], [ %history.i38.i.sroa.0.08421, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075 ], !dbg !40549
  %history.i38.i.sroa.10.0.lcssa = phi float [ %history.i38.i.sroa.10.0.lcssa13602, %bb50.i ], [ %history.i38.i.sroa.7.08422, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075 ], !dbg !40549
  %history.i38.i.sroa.13.0.lcssa = phi float [ %history.i38.i.sroa.13.0.lcssa13603, %bb50.i ], [ %history.i38.i.sroa.10.08423, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075 ], !dbg !40549
  %history.i38.i.sroa.16.0.lcssa = phi float [ %history.i38.i.sroa.16.0.lcssa13604, %bb50.i ], [ %history.i38.i.sroa.13.08424, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075 ], !dbg !40549
  %history.i38.i.sroa.19.0.lcssa = phi float [ %history.i38.i.sroa.19.0.lcssa13605, %bb50.i ], [ %history.i38.i.sroa.16.08425, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075 ], !dbg !40549
  %history.i38.i.sroa.22.0.lcssa = phi float [ %history.i38.i.sroa.22.0.lcssa13606, %bb50.i ], [ %history.i38.i.sroa.19.08426, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075 ], !dbg !40549
  %history.i38.i.sroa.26.0.lcssa = phi float [ %history.i38.i.sroa.26.0.lcssa13607, %bb50.i ], [ %history.i38.i.sroa.22.08427, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075 ], !dbg !40549
  %history.i38.i.sroa.29.0.lcssa = phi float [ %history.i38.i.sroa.29.0.lcssa13608, %bb50.i ], [ %history.i38.i.sroa.26.08428, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075 ], !dbg !40549
  %history.i38.i.sroa.32.0.lcssa = phi float [ %history.i38.i.sroa.32.0.lcssa13609, %bb50.i ], [ %history.i38.i.sroa.29.08429, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075 ], !dbg !40549
  %history.i38.i.sroa.35.0.lcssa = phi float [ %history.i38.i.sroa.35.0.lcssa13610, %bb50.i ], [ %history.i38.i.sroa.32.08430, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075 ], !dbg !40549
  %history.i38.i.sroa.38.0.lcssa = phi float [ %history.i38.i.sroa.38.0.lcssa13611, %bb50.i ], [ %history.i38.i.sroa.35.08431, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3075 ], !dbg !40549
  store float %history.i38.i.sroa.7.0.lcssa, ptr %history.i38.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !40771
  store float %history.i38.i.sroa.10.0.lcssa, ptr %history.i38.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !40771
  store float %history.i38.i.sroa.13.0.lcssa, ptr %history.i38.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !40771
  store float %history.i38.i.sroa.16.0.lcssa, ptr %history.i38.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !40771
  store float %history.i38.i.sroa.19.0.lcssa, ptr %history.i38.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !40771
  store float %history.i38.i.sroa.22.0.lcssa, ptr %history.i38.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !40771
  store float %history.i38.i.sroa.26.0.lcssa, ptr %history.i38.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !40771
  store float %history.i38.i.sroa.29.0.lcssa, ptr %history.i38.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !40771
  store float %history.i38.i.sroa.32.0.lcssa, ptr %history.i38.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !40771
  store float %history.i38.i.sroa.35.0.lcssa, ptr %history.i38.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !40771
  store float %history.i38.i.sroa.38.0.lcssa, ptr %history.i38.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !40771
  br i1 %_20.i41.i8420.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i, label %bb5.i.i.lr.ph, !dbg !40772

bb5.i.i.lr.ph:                                    ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit242.i
  %_11.i.i.i.i = load float, ptr %_31, align 4
  %_14.i.i.i.i = load float, ptr %677, align 4
  %_17.i.i.i.i = load float, ptr %678, align 4
  %_20.i.i.i.i = load float, ptr %679, align 4
  %_25.i.i.i.i = load float, ptr %row1.i.i.i74.i, align 4
  %_28.i.i.i.i = load float, ptr %680, align 4
  %_31.i.i.i.i = load float, ptr %681, align 4
  %_34.i.i.i.i = load float, ptr %682, align 4
  %_39.i.i.i.i = load float, ptr %row3.i.i.i88.i, align 4
  %_42.i.i.i.i = load float, ptr %683, align 4
  %_45.i.i.i.i = load float, ptr %684, align 4
  %_48.i.i.i.i = load float, ptr %685, align 4
  %_53.i.i.i.i = load float, ptr %row5.i.i.i102.i, align 4
  %_56.i.i.i.i = load float, ptr %686, align 4
  %_59.i.i.i.i = load float, ptr %687, align 4
  %_62.i.i.i.i = load float, ptr %688, align 4
  %_67.i.i.i.i = load float, ptr %row7.i.i.i116.i, align 4
  %_70.i.i.i.i = load float, ptr %689, align 4
  %_73.i.i.i.i = load float, ptr %690, align 4
  %_76.i.i.i.i = load float, ptr %691, align 4
  %_81.i.i.i.i = load float, ptr %row9.i.i.i130.i, align 4
  %_84.i.i.i.i = load float, ptr %692, align 4
  %_87.i.i.i.i = load float, ptr %693, align 4
  %_90.i.i.i.i = load float, ptr %694, align 4
  %_95.i.i.i.i = load float, ptr %row11.i.i.i144.i, align 4
  %_98.i.i.i.i = load float, ptr %695, align 4
  %_101.i.i.i.i = load float, ptr %696, align 4
  %_104.i.i.i.i = load float, ptr %697, align 4
  %_109.i.i.i.i = load float, ptr %row13.i.i.i158.i, align 4
  %_112.i.i.i.i = load float, ptr %698, align 4
  %_115.i.i.i.i = load float, ptr %699, align 4
  %_118.i.i.i.i = load float, ptr %700, align 4
  %_123.i.i.i.i = load float, ptr %row15.i.i.i172.i, align 4
  %_126.i.i.i.i = load float, ptr %701, align 4
  %_129.i.i.i.i = load float, ptr %702, align 4
  %_132.i.i.i.i = load float, ptr %703, align 4
  %_137.i.i.i.i = load float, ptr %row17.i.i.i186.i, align 4
  %_140.i.i.i.i = load float, ptr %704, align 4
  %_143.i.i.i.i = load float, ptr %705, align 4
  %_146.i.i.i.i = load float, ptr %706, align 4
  %_151.i.i.i.i = load float, ptr %row19.i.i.i200.i, align 4
  %_154.i.i.i.i = load float, ptr %707, align 4
  %_157.i.i.i.i = load float, ptr %708, align 4
  %_160.i.i.i.i = load float, ptr %709, align 4
  %_165.i.i.i.i = load float, ptr %row21.i.i.i214.i, align 4
  %_168.i.i.i.i = load float, ptr %710, align 4
  %_171.i.i.i.i = load float, ptr %711, align 4
  %_174.i.i.i.i = load float, ptr %712, align 4
  br label %bb5.i.i, !dbg !40772

bb5.i.i:                                          ; preds = %bb5.i.i.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080
  %iter.sroa.0.0.i.i8459 = phi i64 [ 0, %bb5.i.i.lr.ph ], [ %739, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080 ]
  %history.i.i.sroa.35.08458 = phi float [ %history.i.i.sroa.35.0.lcssa13641, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.32.08457, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080 ]
  %history.i.i.sroa.32.08457 = phi float [ %history.i.i.sroa.32.0.lcssa13640, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.29.08456, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080 ]
  %history.i.i.sroa.29.08456 = phi float [ %history.i.i.sroa.29.0.lcssa13639, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.26.08455, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080 ]
  %history.i.i.sroa.26.08455 = phi float [ %history.i.i.sroa.26.0.lcssa13638, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.22.08454, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080 ]
  %history.i.i.sroa.22.08454 = phi float [ %history.i.i.sroa.22.0.lcssa13637, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.19.08453, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080 ]
  %history.i.i.sroa.19.08453 = phi float [ %history.i.i.sroa.19.0.lcssa13636, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.16.08452, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080 ]
  %history.i.i.sroa.16.08452 = phi float [ %history.i.i.sroa.16.0.lcssa13635, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.13.08451, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080 ]
  %history.i.i.sroa.13.08451 = phi float [ %history.i.i.sroa.13.0.lcssa13634, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.10.08450, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080 ]
  %history.i.i.sroa.10.08450 = phi float [ %history.i.i.sroa.10.0.lcssa13633, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.7.08449, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080 ]
  %history.i.i.sroa.7.08449 = phi float [ %history.i.i.sroa.7.0.lcssa13632, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.0.08448, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080 ]
  %history.i.i.sroa.0.08448 = phi float [ %history.i.i.sroa.0.0.lcssa13612, %bb5.i.i.lr.ph ], [ %_0.i3078, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080 ]
  %739 = add nuw nsw i64 %iter.sroa.0.0.i.i8459, 1, !dbg !40775
  %_11.i.i = add nuw nsw i64 %iter.sroa.0.0.i.i8459, %iter1.sroa.0.0.i8792, !dbg !40778
  %_24.i.i = icmp ugt i64 %_11.i.i, %right_io.1, !dbg !40779
  br i1 %_24.i.i, label %bb7.i.i, label %bb8.i.i, !dbg !40779, !prof !639

bb8.i.i:                                          ; preds = %bb5.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !40782), !dbg !40785
  %_3.not.i3076 = icmp eq i64 %right_io.1, %_11.i.i, !dbg !40786
  br i1 %_3.not.i3076, label %panic.i3079, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080, !dbg !40786

panic.i3079:                                      ; preds = %bb8.i.i
  store float %history.i38.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40549
  store float %history.i.i.sroa.0.0.lcssa13612, ptr %hot_right.i, align 4, !dbg !40550
  store i32 %storemerge.i1246.lcssa85038712.lcssa13644, ptr %_22.i270.i, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313547.lcssa13666, ptr %_21.i269.i, align 4
  store i32 %storemerge.i1215.lcssa86158751.lcssa13687, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa1141713564.lcssa13709, ptr %_21.i.i, align 4
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !40786, !noalias !40788
  unreachable, !dbg !40786

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080: ; preds = %bb8.i.i
  %_31.i.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %_11.i.i, !dbg !40792
  %_0.i3078 = load float, ptr %_31.i.i, align 4, !dbg !40786, !alias.scope !40782, !noalias !40794, !noundef !12
  %740 = tail call noundef float @llvm.fabs.f32(float %history.i.i.sroa.19.08453), !dbg !40795
  %_0.i2853 = fmul float %_0.i3078, %_11.i.i.i.i, !dbg !40798
  %_0.i2417 = fadd float %_0.i2853, 0.000000e+00, !dbg !40801
  %_0.i2852 = fmul float %_0.i3078, %_14.i.i.i.i, !dbg !40803
  %_0.i2416 = fadd float %_0.i2852, 0.000000e+00, !dbg !40805
  %_0.i2851 = fmul float %_0.i3078, %_17.i.i.i.i, !dbg !40807
  %_0.i2415 = fadd float %_0.i2851, 0.000000e+00, !dbg !40809
  %_0.i2850 = fmul float %_0.i3078, %_20.i.i.i.i, !dbg !40811
  %_0.i2414 = fadd float %_0.i2850, 0.000000e+00, !dbg !40813
  %_0.i2849 = fmul float %history.i.i.sroa.0.08448, %_25.i.i.i.i, !dbg !40815
  %_0.i2413 = fadd float %_0.i2417, %_0.i2849, !dbg !40817
  %_0.i2848 = fmul float %history.i.i.sroa.0.08448, %_28.i.i.i.i, !dbg !40819
  %_0.i2412 = fadd float %_0.i2416, %_0.i2848, !dbg !40821
  %_0.i2847 = fmul float %history.i.i.sroa.0.08448, %_31.i.i.i.i, !dbg !40823
  %_0.i2411 = fadd float %_0.i2415, %_0.i2847, !dbg !40825
  %_0.i2846 = fmul float %history.i.i.sroa.0.08448, %_34.i.i.i.i, !dbg !40827
  %_0.i2410 = fadd float %_0.i2414, %_0.i2846, !dbg !40829
  %_0.i2845 = fmul float %history.i.i.sroa.7.08449, %_39.i.i.i.i, !dbg !40831
  %_0.i2409 = fadd float %_0.i2413, %_0.i2845, !dbg !40833
  %_0.i2844 = fmul float %history.i.i.sroa.7.08449, %_42.i.i.i.i, !dbg !40835
  %_0.i2408 = fadd float %_0.i2412, %_0.i2844, !dbg !40837
  %_0.i2843 = fmul float %history.i.i.sroa.7.08449, %_45.i.i.i.i, !dbg !40839
  %_0.i2407 = fadd float %_0.i2411, %_0.i2843, !dbg !40841
  %_0.i2842 = fmul float %history.i.i.sroa.7.08449, %_48.i.i.i.i, !dbg !40843
  %_0.i2406 = fadd float %_0.i2410, %_0.i2842, !dbg !40845
  %_0.i2841 = fmul float %history.i.i.sroa.10.08450, %_53.i.i.i.i, !dbg !40847
  %_0.i2405 = fadd float %_0.i2409, %_0.i2841, !dbg !40849
  %_0.i2840 = fmul float %history.i.i.sroa.10.08450, %_56.i.i.i.i, !dbg !40851
  %_0.i2404 = fadd float %_0.i2408, %_0.i2840, !dbg !40853
  %_0.i2839 = fmul float %history.i.i.sroa.10.08450, %_59.i.i.i.i, !dbg !40855
  %_0.i2403 = fadd float %_0.i2407, %_0.i2839, !dbg !40857
  %_0.i2838 = fmul float %history.i.i.sroa.10.08450, %_62.i.i.i.i, !dbg !40859
  %_0.i2402 = fadd float %_0.i2406, %_0.i2838, !dbg !40861
  %_0.i2837 = fmul float %history.i.i.sroa.13.08451, %_67.i.i.i.i, !dbg !40863
  %_0.i2401 = fadd float %_0.i2405, %_0.i2837, !dbg !40865
  %_0.i2836 = fmul float %history.i.i.sroa.13.08451, %_70.i.i.i.i, !dbg !40867
  %_0.i2400 = fadd float %_0.i2404, %_0.i2836, !dbg !40869
  %_0.i2835 = fmul float %history.i.i.sroa.13.08451, %_73.i.i.i.i, !dbg !40871
  %_0.i2399 = fadd float %_0.i2403, %_0.i2835, !dbg !40873
  %_0.i2834 = fmul float %history.i.i.sroa.13.08451, %_76.i.i.i.i, !dbg !40875
  %_0.i2398 = fadd float %_0.i2402, %_0.i2834, !dbg !40877
  %_0.i2833 = fmul float %history.i.i.sroa.16.08452, %_81.i.i.i.i, !dbg !40879
  %_0.i2397 = fadd float %_0.i2401, %_0.i2833, !dbg !40881
  %_0.i2832 = fmul float %history.i.i.sroa.16.08452, %_84.i.i.i.i, !dbg !40883
  %_0.i2396 = fadd float %_0.i2400, %_0.i2832, !dbg !40885
  %_0.i2831 = fmul float %history.i.i.sroa.16.08452, %_87.i.i.i.i, !dbg !40887
  %_0.i2395 = fadd float %_0.i2399, %_0.i2831, !dbg !40889
  %_0.i2830 = fmul float %history.i.i.sroa.16.08452, %_90.i.i.i.i, !dbg !40891
  %_0.i2394 = fadd float %_0.i2398, %_0.i2830, !dbg !40893
  %_0.i2829 = fmul float %history.i.i.sroa.19.08453, %_95.i.i.i.i, !dbg !40895
  %_0.i2393 = fadd float %_0.i2397, %_0.i2829, !dbg !40897
  %_0.i2828 = fmul float %history.i.i.sroa.19.08453, %_98.i.i.i.i, !dbg !40899
  %_0.i2392 = fadd float %_0.i2396, %_0.i2828, !dbg !40901
  %_0.i2827 = fmul float %history.i.i.sroa.19.08453, %_101.i.i.i.i, !dbg !40903
  %_0.i2391 = fadd float %_0.i2395, %_0.i2827, !dbg !40905
  %_0.i2826 = fmul float %history.i.i.sroa.19.08453, %_104.i.i.i.i, !dbg !40907
  %_0.i2390 = fadd float %_0.i2394, %_0.i2826, !dbg !40909
  %_0.i2825 = fmul float %history.i.i.sroa.22.08454, %_109.i.i.i.i, !dbg !40911
  %_0.i2389 = fadd float %_0.i2393, %_0.i2825, !dbg !40913
  %_0.i2824 = fmul float %history.i.i.sroa.22.08454, %_112.i.i.i.i, !dbg !40915
  %_0.i2388 = fadd float %_0.i2392, %_0.i2824, !dbg !40917
  %_0.i2823 = fmul float %history.i.i.sroa.22.08454, %_115.i.i.i.i, !dbg !40919
  %_0.i2387 = fadd float %_0.i2391, %_0.i2823, !dbg !40921
  %_0.i2822 = fmul float %history.i.i.sroa.22.08454, %_118.i.i.i.i, !dbg !40923
  %_0.i2386 = fadd float %_0.i2390, %_0.i2822, !dbg !40925
  %_0.i2821 = fmul float %history.i.i.sroa.26.08455, %_123.i.i.i.i, !dbg !40927
  %_0.i2385 = fadd float %_0.i2389, %_0.i2821, !dbg !40929
  %_0.i2820 = fmul float %history.i.i.sroa.26.08455, %_126.i.i.i.i, !dbg !40931
  %_0.i2384 = fadd float %_0.i2388, %_0.i2820, !dbg !40933
  %_0.i2819 = fmul float %history.i.i.sroa.26.08455, %_129.i.i.i.i, !dbg !40935
  %_0.i2383 = fadd float %_0.i2387, %_0.i2819, !dbg !40937
  %_0.i2818 = fmul float %history.i.i.sroa.26.08455, %_132.i.i.i.i, !dbg !40939
  %_0.i2382 = fadd float %_0.i2386, %_0.i2818, !dbg !40941
  %_0.i2817 = fmul float %history.i.i.sroa.29.08456, %_137.i.i.i.i, !dbg !40943
  %_0.i2381 = fadd float %_0.i2385, %_0.i2817, !dbg !40945
  %_0.i2816 = fmul float %history.i.i.sroa.29.08456, %_140.i.i.i.i, !dbg !40947
  %_0.i2380 = fadd float %_0.i2384, %_0.i2816, !dbg !40949
  %_0.i2815 = fmul float %history.i.i.sroa.29.08456, %_143.i.i.i.i, !dbg !40951
  %_0.i2379 = fadd float %_0.i2383, %_0.i2815, !dbg !40953
  %_0.i2814 = fmul float %history.i.i.sroa.29.08456, %_146.i.i.i.i, !dbg !40955
  %_0.i2378 = fadd float %_0.i2382, %_0.i2814, !dbg !40957
  %_0.i2813 = fmul float %history.i.i.sroa.32.08457, %_151.i.i.i.i, !dbg !40959
  %_0.i2377 = fadd float %_0.i2381, %_0.i2813, !dbg !40961
  %_0.i2812 = fmul float %history.i.i.sroa.32.08457, %_154.i.i.i.i, !dbg !40963
  %_0.i2376 = fadd float %_0.i2380, %_0.i2812, !dbg !40965
  %_0.i2811 = fmul float %history.i.i.sroa.32.08457, %_157.i.i.i.i, !dbg !40967
  %_0.i2375 = fadd float %_0.i2379, %_0.i2811, !dbg !40969
  %_0.i2810 = fmul float %history.i.i.sroa.32.08457, %_160.i.i.i.i, !dbg !40971
  %_0.i2374 = fadd float %_0.i2378, %_0.i2810, !dbg !40973
  %_0.i2809 = fmul float %history.i.i.sroa.35.08458, %_165.i.i.i.i, !dbg !40975
  %_0.i2373 = fadd float %_0.i2377, %_0.i2809, !dbg !40977
  %_0.i2808 = fmul float %history.i.i.sroa.35.08458, %_168.i.i.i.i, !dbg !40979
  %_0.i2372 = fadd float %_0.i2376, %_0.i2808, !dbg !40981
  %_0.i2807 = fmul float %history.i.i.sroa.35.08458, %_171.i.i.i.i, !dbg !40983
  %_0.i2371 = fadd float %_0.i2375, %_0.i2807, !dbg !40985
  %_0.i2806 = fmul float %history.i.i.sroa.35.08458, %_174.i.i.i.i, !dbg !40987
  %_0.i2370 = fadd float %_0.i2374, %_0.i2806, !dbg !40989
  %741 = tail call noundef float @llvm.fabs.f32(float %_0.i2373), !dbg !40991
  %_3.i.i3729.inv = fcmp ogt float %740, %741, !dbg !40993
  %_4.i.i3736.v = select i1 %_3.i.i3729.inv, float %740, float %741, !dbg !40993
  %742 = tail call noundef float @llvm.fabs.f32(float %_0.i2372), !dbg !40991
  %_3.i.i3729.inv.1 = fcmp ogt float %_4.i.i3736.v, %742, !dbg !40993
  %_4.i.i3736.v.1 = select i1 %_3.i.i3729.inv.1, float %_4.i.i3736.v, float %742, !dbg !40993
  %743 = tail call noundef float @llvm.fabs.f32(float %_0.i2371), !dbg !40991
  %_3.i.i3729.inv.2 = fcmp ogt float %_4.i.i3736.v.1, %743, !dbg !40993
  %_4.i.i3736.v.2 = select i1 %_3.i.i3729.inv.2, float %_4.i.i3736.v.1, float %743, !dbg !40993
  %744 = tail call noundef float @llvm.fabs.f32(float %_0.i2370), !dbg !40991
  %_3.i.i3729.inv.3 = fcmp ogt float %_4.i.i3736.v.2, %744, !dbg !40993
  %_4.i.i3736.v.3 = select i1 %_3.i.i3729.inv.3, float %_4.i.i3736.v.2, float %744, !dbg !40993
  %_39.i.i = getelementptr inbounds nuw float, ptr %peaks_right.i, i64 %iter.sroa.0.0.i.i8459, !dbg !40996
  store float %_4.i.i3736.v.3, ptr %_39.i.i, align 4, !dbg !41001, !alias.scope !41003, !noalias !40794
  %exitcond11152.not = icmp eq i64 %739, %umax11151, !dbg !41006
  br i1 %exitcond11152.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i, label %bb5.i.i, !dbg !40772

bb7.i.i:                                          ; preds = %bb5.i.i
  store float %history.i38.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40549
  store float %history.i.i.sroa.0.0.lcssa13612, ptr %hot_right.i, align 4, !dbg !40550
  store i32 %storemerge.i1246.lcssa85038712.lcssa13644, ptr %_22.i270.i, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313547.lcssa13666, ptr %_21.i269.i, align 4
  store i32 %storemerge.i1215.lcssa86158751.lcssa13687, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa1141713564.lcssa13709, ptr %_21.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_11.i.i, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_283c8d9f84e10f29a75d8e6c3a347bea) #30, !dbg !41008, !noalias !40794
  unreachable, !dbg !41008

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit242.i
  %history.i.i.sroa.0.0.lcssa = phi float [ %history.i.i.sroa.0.0.lcssa13612, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit242.i ], [ %_0.i3078, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080 ], !dbg !40550
  %history.i.i.sroa.7.0.lcssa = phi float [ %history.i.i.sroa.7.0.lcssa13632, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit242.i ], [ %history.i.i.sroa.0.08448, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080 ], !dbg !40550
  %history.i.i.sroa.10.0.lcssa = phi float [ %history.i.i.sroa.10.0.lcssa13633, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit242.i ], [ %history.i.i.sroa.7.08449, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080 ], !dbg !40550
  %history.i.i.sroa.13.0.lcssa = phi float [ %history.i.i.sroa.13.0.lcssa13634, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit242.i ], [ %history.i.i.sroa.10.08450, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080 ], !dbg !40550
  %history.i.i.sroa.16.0.lcssa = phi float [ %history.i.i.sroa.16.0.lcssa13635, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit242.i ], [ %history.i.i.sroa.13.08451, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080 ], !dbg !40550
  %history.i.i.sroa.19.0.lcssa = phi float [ %history.i.i.sroa.19.0.lcssa13636, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit242.i ], [ %history.i.i.sroa.16.08452, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080 ], !dbg !40550
  %history.i.i.sroa.22.0.lcssa = phi float [ %history.i.i.sroa.22.0.lcssa13637, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit242.i ], [ %history.i.i.sroa.19.08453, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080 ], !dbg !40550
  %history.i.i.sroa.26.0.lcssa = phi float [ %history.i.i.sroa.26.0.lcssa13638, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit242.i ], [ %history.i.i.sroa.22.08454, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080 ], !dbg !40550
  %history.i.i.sroa.29.0.lcssa = phi float [ %history.i.i.sroa.29.0.lcssa13639, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit242.i ], [ %history.i.i.sroa.26.08455, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080 ], !dbg !40550
  %history.i.i.sroa.32.0.lcssa = phi float [ %history.i.i.sroa.32.0.lcssa13640, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit242.i ], [ %history.i.i.sroa.29.08456, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080 ], !dbg !40550
  %history.i.i.sroa.35.0.lcssa = phi float [ %history.i.i.sroa.35.0.lcssa13641, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit242.i ], [ %history.i.i.sroa.32.08457, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080 ], !dbg !40550
  %history.i.i.sroa.38.0.lcssa = phi float [ %history.i.i.sroa.38.0.lcssa13642, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit242.i ], [ %history.i.i.sroa.35.08458, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3080 ], !dbg !40550
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !41009
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !41009
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !41009
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !41009
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !41009
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !41009
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !41009
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !41009
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !41009
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !41009
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !41009
  br i1 %_20.i41.i8420.not, label %bb15.i.loopexit, label %bb20.i.lr.ph, !dbg !40514

bb20.i.lr.ph:                                     ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkfEB2_.exit.i
  %_65.i.sroa.3.0.copyload = load i64, ptr %_65.i.sroa.3.0..sroa_idx, align 8, !noalias !40486
  %_65.i.sroa.4.0.copyload = load i64, ptr %_65.i.sroa.4.0..sroa_idx, align 8, !noalias !40486
  %_66.i.sroa.3.0.copyload = load i64, ptr %_66.i.sroa.3.0..sroa_idx, align 8, !noalias !40486
  %_66.i.sroa.4.0.copyload = load i64, ptr %_66.i.sroa.4.0..sroa_idx, align 8, !noalias !40486
  %_54.0.i255.i = load ptr, ptr %uniform_left.i, align 8, !nonnull !12, !align !24
  %_54.1.i256.i = load i64, ptr %715, align 8
  %_18.i266.i = load i64, ptr %713, align 8
  %_56.0.i277.i = load ptr, ptr %716, align 8, !nonnull !12, !align !24
  %_56.1.i278.i = load i64, ptr %717, align 8
  %_58.1.i303.i = load i64, ptr %721, align 8
  %_58.0.i302.i = load ptr, ptr %722, align 8, !nonnull !12, !align !24
  %_54.0.i.i = load ptr, ptr %uniform_right.i, align 8, !nonnull !12, !align !24
  %_54.1.i.i = load i64, ptr %723, align 8
  %_18.i.i = load i64, ptr %714, align 8
  %_56.0.i.i = load ptr, ptr %724, align 8, !nonnull !12, !align !24
  %_56.1.i.i = load i64, ptr %725, align 8
  %_58.1.i.i = load i64, ptr %729, align 8
  %_58.0.i.i = load ptr, ptr %730, align 8, !nonnull !12, !align !24
  %umax11153 = call i64 @llvm.umax.i64(i64 %_18.i266.i, i64 1), !dbg !40514
  %umax11155 = call i64 @llvm.umax.i64(i64 %_18.i.i, i64 1), !dbg !40514
  %_8.i28.i = load float, ptr %_109.i, align 4
  %_9.i29.i = load float, ptr %_110.i, align 4
  %_8.i.i = load float, ptr %_114.i, align 4
  %_9.i.i = load float, ptr %_115.i, align 4
  %_37.i290.i = load float, ptr %719, align 4
  %_37.i.i = load float, ptr %727, align 4
  br label %bb20.i, !dbg !40514

bb20.i:                                           ; preds = %bb20.i.lr.ph, %bb32.i
  %running.sroa.0.0.i1210.lcssa1141713565 = phi float [ %running.sroa.0.0.i1210.lcssa1141713564.lcssa13709, %bb20.i.lr.ph ], [ %running.sroa.0.0.i1210.lcssa1141713564, %bb32.i ]
  %running.sroa.0.0.i1241.lcssa1136313548 = phi float [ %running.sroa.0.0.i1241.lcssa1136313547.lcssa13666, %bb20.i.lr.ph ], [ %running.sroa.0.0.i1241.lcssa1136313547, %bb32.i ]
  %_0.i3269.lcssa1142313531 = phi float [ %_0.i3269.lcssa1142313530.lcssa13740, %bb20.i.lr.ph ], [ %_0.i3269.lcssa1142313530, %bb32.i ]
  %_0.i2895.lcssa1142213513 = phi float [ %_0.i2895.lcssa1142213512.lcssa13737, %bb20.i.lr.ph ], [ %_0.i2895.lcssa1142213512, %bb32.i ]
  %_0.i3273.lcssa1139813495 = phi float [ %_0.i3273.lcssa1139813494.lcssa13734, %bb20.i.lr.ph ], [ %_0.i3273.lcssa1139813494, %bb32.i ]
  %_0.i2899.lcssa1138213477 = phi float [ %_0.i2899.lcssa1138213476.lcssa13731, %bb20.i.lr.ph ], [ %_0.i2899.lcssa1138213476, %bb32.i ]
  %running.sroa.0.0.i1210.lcssa86428788 = phi float [ %running.sroa.0.0.i1210.lcssa1141713564.lcssa13709, %bb20.i.lr.ph ], [ %running.sroa.0.0.i1210.lcssa86428787, %bb32.i ]
  %storemerge.i1215.lcssa86158752 = phi i32 [ %storemerge.i1215.lcssa86158751.lcssa13687, %bb20.i.lr.ph ], [ %storemerge.i1215.lcssa86158751, %bb32.i ]
  %running.sroa.0.0.i1241.lcssa85308749 = phi float [ %running.sroa.0.0.i1241.lcssa1136313547.lcssa13666, %bb20.i.lr.ph ], [ %running.sroa.0.0.i1241.lcssa85308748, %bb32.i ]
  %storemerge.i1246.lcssa85038713 = phi i32 [ %storemerge.i1246.lcssa85038712.lcssa13644, %bb20.i.lr.ph ], [ %storemerge.i1246.lcssa85038712, %bb32.i ]
  %frame.sroa.0.0.i8708 = phi i64 [ 0, %bb20.i.lr.ph ], [ %_80.i, %bb32.i ]
  %main_cursor.sroa.0.1.i8707 = phi i64 [ %main_cursor.sroa.0.0.i8791, %bb20.i.lr.ph ], [ %main_cursor.sroa.0.2.i, %bb32.i ]
  %ring_cursor.sroa.0.1.i8706 = phi i64 [ %ring_cursor.sroa.0.0.i8790, %bb20.i.lr.ph ], [ %ring_cursor.sroa.0.2.i, %bb32.i ]
  %_63.i = sub nuw nsw i64 %..i4276, %frame.sroa.0.0.i8708, !dbg !41010
  %ring.i1839 = load i64, ptr %671, align 8, !dbg !41011, !alias.scope !41013, !noalias !41016, !noundef !12
  %main.i1840 = load i64, ptr %672, align 8, !dbg !41020, !alias.scope !41013, !noalias !41016, !noundef !12
  %_10.i1841 = add i64 %ring_cursor.sroa.0.1.i8706, 1, !dbg !41021
  %_45.not.i1842 = icmp ult i64 %_10.i1841, %ring.i1839, !dbg !41022
  %745 = select i1 %_45.not.i1842, i64 0, i64 %ring.i1839, !dbg !41022
  %start1.sroa.0.0.i1843 = sub nuw i64 %_10.i1841, %745, !dbg !41022
  %_12.i1845 = add i64 %_65.i.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i8706, !dbg !41024
  %_46.not.i1846 = icmp ult i64 %_12.i1845, %ring.i1839, !dbg !41025
  %746 = select i1 %_46.not.i1846, i64 0, i64 %ring.i1839, !dbg !41025
  %left_end.sroa.0.0.i1847 = sub nuw i64 %_12.i1845, %746, !dbg !41025
  %_15.i1849 = add i64 %_66.i.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i8706, !dbg !41027
  %_47.not.i1850 = icmp ult i64 %_15.i1849, %ring.i1839, !dbg !41028
  %747 = select i1 %_47.not.i1850, i64 0, i64 %ring.i1839, !dbg !41028
  %right_end.sroa.0.0.i1851 = sub nuw i64 %_15.i1849, %747, !dbg !41028
  %_18.i1853 = add i64 %_65.i.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i8706, !dbg !41030
  %_48.not.i1854 = icmp ult i64 %_18.i1853, %ring.i1839, !dbg !41031
  %748 = select i1 %_48.not.i1854, i64 0, i64 %ring.i1839, !dbg !41031
  %left_expiring.sroa.0.0.i1855 = sub nuw i64 %_18.i1853, %748, !dbg !41031
  %_21.i1857 = add i64 %_66.i.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i8706, !dbg !41033
  %_49.not.i1858 = icmp ult i64 %_21.i1857, %ring.i1839, !dbg !41034
  %749 = select i1 %_49.not.i1858, i64 0, i64 %ring.i1839, !dbg !41034
  %right_expiring.sroa.0.0.i1859 = sub nuw i64 %_21.i1857, %749, !dbg !41034
  %_30.i1860 = sub i64 %ring.i1839, %ring_cursor.sroa.0.1.i8706, !dbg !41036
  %..i4301 = tail call noundef i64 @llvm.umin.i64(i64 %_30.i1860, i64 %_63.i), !dbg !41037
  %_31.i1862 = sub i64 %main.i1840, %main_cursor.sroa.0.1.i8707, !dbg !41039
  %..i4302 = tail call noundef i64 @llvm.umin.i64(i64 %_31.i1862, i64 %..i4301), !dbg !41040
  %_32.i1864 = sub i64 %ring.i1839, %start1.sroa.0.0.i1843, !dbg !41042
  %..i4303 = tail call noundef i64 @llvm.umin.i64(i64 %_32.i1864, i64 %..i4302), !dbg !41043
  %_34.i1866 = sub i64 %ring.i1839, %left_end.sroa.0.0.i1847, !dbg !41045
  %..i4304 = tail call noundef i64 @llvm.umin.i64(i64 %_34.i1866, i64 %..i4303), !dbg !41046
  %_36.i1868 = sub i64 %ring.i1839, %right_end.sroa.0.0.i1851, !dbg !41048
  %..i4305 = tail call noundef i64 @llvm.umin.i64(i64 %_36.i1868, i64 %..i4304), !dbg !41049
  %_38.i1870 = sub i64 %ring.i1839, %left_expiring.sroa.0.0.i1855, !dbg !41051
  %..i4306 = tail call noundef i64 @llvm.umin.i64(i64 %_38.i1870, i64 %..i4305), !dbg !41052
  %_40.i1872 = sub i64 %ring.i1839, %right_expiring.sroa.0.0.i1859, !dbg !41054
  %..i4307 = tail call noundef i64 @llvm.umin.i64(i64 %_40.i1872, i64 %..i4306), !dbg !41055
  %_69.i = add i64 %frame.sroa.0.0.i8708, %iter1.sroa.0.0.i8792, !dbg !41057
  %_73.i = add i64 %..i4307, %_69.i, !dbg !41060
  %_174.i = icmp ult i64 %_73.i, %_69.i, !dbg !41063
  %_168.not.i = icmp ugt i64 %_73.i, %left_io.1
  %or.cond.i = or i1 %_174.i, %_168.not.i, !dbg !41063
  br i1 %or.cond.i, label %bb55.i, label %bb53.i, !dbg !41063, !prof !165

bb55.i:                                           ; preds = %bb20.i
  store float %history.i38.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40549
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40550
  store i32 %storemerge.i1246.lcssa85038712.lcssa13644, ptr %_22.i270.i, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313547.lcssa13666, ptr %_21.i269.i, align 4
  store i32 %storemerge.i1215.lcssa86158751.lcssa13687, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa1141713564.lcssa13709, ptr %_21.i.i, align 4
  store float %_0.i2899.lcssa1138213477, ptr %718, align 4
  store float %_0.i3273.lcssa1139813495, ptr %720, align 4
  store float %_0.i2895.lcssa1142213513, ptr %726, align 4
  store float %_0.i3269.lcssa1142313531, ptr %728, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313548, ptr %_21.i269.i, align 1, !dbg !41070
  store float %running.sroa.0.0.i1210.lcssa1141713565, ptr %_21.i.i, align 1, !dbg !41088
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_69.i, i64 noundef %_73.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_efa26be4ae64153d8eb817c4a24fadb5) #30, !dbg !41091, !noalias !40457
  unreachable, !dbg !41091

bb53.i:                                           ; preds = %bb20.i
  %_177.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %_69.i, !dbg !41092
  %_178.not.i = icmp ugt i64 %_73.i, %right_io.1, !dbg !41096
  br i1 %_178.not.i, label %bb58.i, label %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit4374, !dbg !41096, !prof !639

bb58.i:                                           ; preds = %bb53.i
  store float %history.i38.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40549
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40550
  store i32 %storemerge.i1246.lcssa85038712.lcssa13644, ptr %_22.i270.i, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313547.lcssa13666, ptr %_21.i269.i, align 4
  store i32 %storemerge.i1215.lcssa86158751.lcssa13687, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa1141713564.lcssa13709, ptr %_21.i.i, align 4
  store float %_0.i2899.lcssa1138213477, ptr %718, align 4
  store float %_0.i3273.lcssa1139813495, ptr %720, align 4
  store float %_0.i2895.lcssa1142213513, ptr %726, align 4
  store float %_0.i3269.lcssa1142313531, ptr %728, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313548, ptr %_21.i269.i, align 1, !dbg !41070
  store float %running.sroa.0.0.i1210.lcssa1141713565, ptr %_21.i.i, align 1, !dbg !41088
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_69.i, i64 noundef %_73.i, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_fcc53ff99bd917af3a52e0ac8a8cbd22) #30, !dbg !41100, !noalias !40457
  unreachable, !dbg !41100

_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit4374: ; preds = %bb53.i
  %_185.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %_69.i, !dbg !41101
  %_80.i = add nuw nsw i64 %..i4307, %frame.sroa.0.0.i8708, !dbg !41105
  %_194.i = getelementptr inbounds nuw float, ptr %peaks_left.i, i64 %frame.sroa.0.0.i8708, !dbg !41106
  %_203.i = getelementptr inbounds nuw float, ptr %peaks_right.i, i64 %frame.sroa.0.0.i8708, !dbg !41116
  %_2.i.i.i43778478.not = icmp eq i64 %..i4307, 0, !dbg !41125
  br i1 %_2.i.i.i43778478.not, label %bb32.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3085.lr.ph, !dbg !41125

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3085.lr.ph: ; preds = %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit4374
  %umin11159 = call i64 @llvm.umin.i64(i64 %_34.i1866, i64 %_36.i1868), !dbg !41125
  %umin11160 = call i64 @llvm.umin.i64(i64 %umin11159, i64 %_38.i1870), !dbg !41125
  %umin11161 = call i64 @llvm.umin.i64(i64 %umin11160, i64 %_40.i1872), !dbg !41125
  %umin11162 = call i64 @llvm.umin.i64(i64 %umin11161, i64 %_32.i1864), !dbg !41125
  %umin11163 = call i64 @llvm.umin.i64(i64 %umin11162, i64 %_30.i1860), !dbg !41125
  %umin11164 = call i64 @llvm.umin.i64(i64 %umin11163, i64 %_31.i1862), !dbg !41125
  %750 = sub nsw i64 %umin11165, %frame.sroa.0.0.i8708, !dbg !41125
  %umin11166 = call i64 @llvm.umin.i64(i64 %umin11164, i64 %750), !dbg !41125
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3085, !dbg !41125

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3085: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3085.lr.ph, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3232
  %_0.i32698677 = phi float [ %_0.i3269.lcssa1142313531, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3085.lr.ph ], [ %_0.i3269, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3232 ]
  %_0.i28958648 = phi float [ %_0.i2895.lcssa1142213513, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3085.lr.ph ], [ %_0.i2895, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3232 ]
  %running.sroa.0.0.i12108620 = phi float [ %running.sroa.0.0.i1210.lcssa86428788, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3085.lr.ph ], [ %running.sroa.0.0.i1210, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3232 ]
  %storemerge.i12158593 = phi i32 [ %storemerge.i1215.lcssa86158752, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3085.lr.ph ], [ %storemerge.i1215, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3232 ]
  %_0.i32738565 = phi float [ %_0.i3273.lcssa1139813495, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3085.lr.ph ], [ %_0.i3273, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3232 ]
  %_0.i28998536 = phi float [ %_0.i2899.lcssa1138213477, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3085.lr.ph ], [ %_0.i2899, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3232 ]
  %running.sroa.0.0.i12418508 = phi float [ %running.sroa.0.0.i1241.lcssa85308749, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3085.lr.ph ], [ %running.sroa.0.0.i1241, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3232 ]
  %storemerge.i12468481 = phi i32 [ %storemerge.i1246.lcssa85038713, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3085.lr.ph ], [ %storemerge.i1246, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3232 ]
  %iter.i.sroa.41.08480 = phi i64 [ 0, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3085.lr.ph ], [ %_9.0.i4406, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3232 ]
  %_9.0.i4406 = add nuw i64 %iter.i.sroa.41.08480, 1, !dbg !41129
  %data.i.i.i.i.i.i.i.i4390 = getelementptr inbounds nuw float, ptr %_177.i, i64 %iter.i.sroa.41.08480, !dbg !41130
  %data.i.i.i.i4395 = getelementptr inbounds nuw float, ptr %_203.i, i64 %iter.i.sroa.41.08480, !dbg !41137
  %data.i.i.i.i.i.i4400 = getelementptr inbounds nuw float, ptr %_194.i, i64 %iter.i.sroa.41.08480, !dbg !41140
  %data.i5.i.i.i.i.i.i.i4404 = getelementptr inbounds nuw float, ptr %_185.i, i64 %iter.i.sroa.41.08480, !dbg !41143
  %_0.i3098 = load float, ptr %data.i.i.i.i.i.i4400, align 4, !dbg !41146, !alias.scope !41148, !noalias !40457, !noundef !12
  %_0.i3093 = load float, ptr %data.i.i.i.i4395, align 4, !dbg !41151, !alias.scope !41153, !noalias !40457, !noundef !12
  %_3.i.i3756 = fcmp ule float %_0.i3093, %_0.i3098, !dbg !41156
  %_6.i.i3758 = bitcast float %_0.i3093 to i32, !dbg !41159
  %_8.i.i3760 = bitcast float %_0.i3098 to i32, !dbg !41162
  %_4.i.i3763 = select i1 %_3.i.i3756, i32 %_8.i.i3760, i32 %_6.i.i3758, !dbg !41164
  %_5.i3529 = and i32 %_4.i.i3763, %.none.i, !dbg !41165
  %_7.i3525 = and i32 %_9.i3531, %_6.i.i3758, !dbg !41167
  %_4.i3526 = or disjoint i32 %_5.i3529, %_7.i3525, !dbg !41169
  %_0.i3527 = bitcast i32 %_4.i3526 to float, !dbg !41170
  %_0.i3088 = load float, ptr %data.i.i.i.i.i.i.i.i4390, align 4, !dbg !41172, !alias.scope !41174, !noalias !40457, !noundef !12
  %_0.i3083 = load float, ptr %data.i5.i.i.i.i.i.i.i4404, align 4, !dbg !41177, !alias.scope !41179, !noalias !40457, !noundef !12
  %_205.i = add nuw i64 %iter.i.sroa.41.08480, %ring_cursor.sroa.0.1.i8706, !dbg !41182
  %_206.i = add nuw i64 %iter.i.sroa.41.08480, %main_cursor.sroa.0.1.i8707, !dbg !41185
  %_207.i = add nuw i64 %iter.i.sroa.41.08480, %left_end.sroa.0.0.i1847, !dbg !41186
  %_208.i = add i64 %iter.i.sroa.41.08480, %start1.sroa.0.0.i1843, !dbg !41187
  %_209.i = add nuw i64 %iter.i.sroa.41.08480, %left_expiring.sroa.0.0.i1855, !dbg !41188
  %_7.i8.i258.i = add i64 %_205.i, 1, !dbg !41189
  %or.cond.i11.i261.i.not = icmp ult i64 %_205.i, %_54.1.i256.i, !dbg !41191
  br i1 %or.cond.i11.i261.i.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i262.i, label %bb4.i13.i317.i, !dbg !41191, !prof !2723

bb4.i13.i317.i:                                   ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3085
  store float %history.i38.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40549
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40550
  store i32 %storemerge.i1246.lcssa85038712.lcssa13644, ptr %_22.i270.i, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313547.lcssa13666, ptr %_21.i269.i, align 4
  store i32 %storemerge.i1215.lcssa86158751.lcssa13687, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa1141713564.lcssa13709, ptr %_21.i.i, align 4
  store float %_0.i2899.lcssa1138213477, ptr %718, align 4
  store float %_0.i3273.lcssa1139813495, ptr %720, align 4
  store float %_0.i2895.lcssa1142213513, ptr %726, align 4
  store float %_0.i3269.lcssa1142313531, ptr %728, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313548, ptr %_21.i269.i, align 1, !dbg !41070
  store float %running.sroa.0.0.i1210.lcssa1141713565, ptr %_21.i.i, align 1, !dbg !41088
  %umax11157 = call i64 @llvm.umax.i64(i64 %ring_cursor.sroa.0.1.i8706, i64 %_54.1.i256.i), !dbg !41125
  %751 = add i64 %umax11157, 1, !dbg !41125
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_205.i, i64 noundef %751, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i256.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !41195, !noalias !41196
  unreachable, !dbg !41195

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i262.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3085
  %_7.i3532 = and i32 %_9.i3531, %_8.i.i3760, !dbg !41204
  %_4.i3533 = or disjoint i32 %_5.i3529, %_7.i3532, !dbg !41165
  %_0.i3534 = bitcast i32 %_4.i3533 to float, !dbg !41205
  %_0.i2435 = fdiv float %_8.i28.i, %_0.i3534, !dbg !41207
  %_3.i2002 = fcmp uge float %_8.i28.i, %_0.i3534, !dbg !41209
  %_0.i3520 = select i1 %_3.i2002, float 1.000000e+00, float %_0.i2435, !dbg !41211
  %_17.i12.i263.i = getelementptr inbounds nuw float, ptr %_54.0.i255.i, i64 %_205.i, !dbg !41213
  store float %_0.i3520, ptr %_17.i12.i263.i, align 4, !dbg !41215, !alias.scope !41217, !noalias !41220
  %or.cond.i1288.not = icmp ult i64 %_207.i, %_54.1.i256.i, !dbg !41221
  br i1 %or.cond.i1288.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1292, label %bb4.i1291, !dbg !41221, !prof !2723

bb4.i1291:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i262.i
  store float %history.i38.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40549
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40550
  store i32 %storemerge.i1246.lcssa85038712.lcssa13644, ptr %_22.i270.i, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313547.lcssa13666, ptr %_21.i269.i, align 4
  store i32 %storemerge.i1215.lcssa86158751.lcssa13687, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa1141713564.lcssa13709, ptr %_21.i.i, align 4
  store float %_0.i2899.lcssa1138213477, ptr %718, align 4
  store float %_0.i3273.lcssa1139813495, ptr %720, align 4
  store float %_0.i2895.lcssa1142213513, ptr %726, align 4
  store float %_0.i3269.lcssa1142313531, ptr %728, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313548, ptr %_21.i269.i, align 1, !dbg !41070
  store float %running.sroa.0.0.i1210.lcssa1141713565, ptr %_21.i.i, align 1, !dbg !41088
  %_5.i1285 = add i64 %_207.i, 1, !dbg !41226
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_207.i, i64 noundef %_5.i1285, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i256.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !41227, !noalias !41228
  unreachable, !dbg !41227

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1292: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i262.i
  %_15.i1289 = getelementptr inbounds nuw float, ptr %_54.0.i255.i, i64 %_207.i, !dbg !41234
  %_0.i2941 = load float, ptr %_15.i1289, align 4, !dbg !41236, !alias.scope !41238, !noalias !41241, !noundef !12
  %position.i1237 = zext i32 %storemerge.i12468481 to i64, !dbg !41242
  %752 = icmp eq i32 %storemerge.i12468481, 0, !dbg !41243
  %_3.i.i3783.inv = fcmp olt float %running.sroa.0.0.i12418508, %_0.i2941, !dbg !41243
  %_4.i.i3790.v = select i1 %_3.i.i3783.inv, float %running.sroa.0.0.i12418508, float %_0.i2941, !dbg !41243
  %running.sroa.0.0.i1241 = select i1 %752, float %_0.i2941, float %_4.i.i3790.v, !dbg !41243
  %_15.i1242 = add nuw nsw i64 %position.i1237, 1, !dbg !41244
  %complete.i1243 = icmp eq i64 %_15.i1242, %_18.i266.i, !dbg !41244
  br i1 %complete.i1243, label %bb19.i1254, label %bb7.i1244, !dbg !41245

bb7.i1244:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1292
  %or.cond.i1280.not = icmp ult i64 %_208.i, %_54.1.i256.i, !dbg !41246
  br i1 %or.cond.i1280.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1284, label %bb4.i1283, !dbg !41246, !prof !2723

bb4.i1283:                                        ; preds = %bb7.i1244
  store float %history.i38.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40549
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40550
  store i32 %storemerge.i1246.lcssa85038712.lcssa13644, ptr %_22.i270.i, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313547.lcssa13666, ptr %_21.i269.i, align 4
  store i32 %storemerge.i1215.lcssa86158751.lcssa13687, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa1141713564.lcssa13709, ptr %_21.i.i, align 4
  store float %_0.i2899.lcssa1138213477, ptr %718, align 4
  store float %_0.i3273.lcssa1139813495, ptr %720, align 4
  store float %_0.i2895.lcssa1142213513, ptr %726, align 4
  store float %_0.i3269.lcssa1142313531, ptr %728, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313548, ptr %_21.i269.i, align 1, !dbg !41070
  store float %running.sroa.0.0.i1210.lcssa1141713565, ptr %_21.i.i, align 1, !dbg !41088
  %_5.i1277 = add i64 %_208.i, 1, !dbg !41251
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_208.i, i64 noundef %_5.i1277, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i256.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !41252, !noalias !41253
  unreachable, !dbg !41252

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1284: ; preds = %bb7.i1244
  %_15.i1281 = getelementptr inbounds nuw float, ptr %_54.0.i255.i, i64 %_208.i, !dbg !41256
  %_0.i2943 = load float, ptr %_15.i1281, align 4, !dbg !41258, !alias.scope !41260, !noalias !41241, !noundef !12
  %_3.i.i3774.inv = fcmp olt float %_0.i2943, %running.sroa.0.0.i1241, !dbg !41263
  %_4.i.i3781.v = select i1 %_3.i.i3774.inv, float %_0.i2943, float %running.sroa.0.0.i1241, !dbg !41263
  %753 = trunc i64 %_15.i1242 to i32, !dbg !41266
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1265, !dbg !41267

bb19.i1254:                                       ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1292, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1261
  %end.sroa.0.0.i12528474 = phi i64 [ %755, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1261 ], [ %_207.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1292 ]
  %suffix.sroa.0.0.i12518473 = phi float [ %_4.i.i3772.v, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1261 ], [ %_0.i2941, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1292 ]
  %iter.sroa.0.0.i12508472 = phi i64 [ %_30.i1255, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1261 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1292 ]
  %or.cond.i1267.not = icmp ult i64 %end.sroa.0.0.i12528474, %_54.1.i256.i, !dbg !41268
  br i1 %or.cond.i1267.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1261, label %bb4.i, !dbg !41268, !prof !2723

bb4.i:                                            ; preds = %bb19.i1254
  store float %history.i38.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40549
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40550
  store i32 %storemerge.i1246.lcssa85038712.lcssa13644, ptr %_22.i270.i, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313547.lcssa13666, ptr %_21.i269.i, align 4
  store i32 %storemerge.i1215.lcssa86158751.lcssa13687, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa1141713564.lcssa13709, ptr %_21.i.i, align 4
  store float %_0.i2899.lcssa1138213477, ptr %718, align 4
  store float %_0.i3273.lcssa1139813495, ptr %720, align 4
  store float %_0.i2895.lcssa1142213513, ptr %726, align 4
  store float %_0.i3269.lcssa1142313531, ptr %728, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313548, ptr %_21.i269.i, align 1, !dbg !41070
  store float %running.sroa.0.0.i1210.lcssa1141713565, ptr %_21.i.i, align 1, !dbg !41088
  %_5.i = add i64 %end.sroa.0.0.i12528474, 1, !dbg !41273
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %end.sroa.0.0.i12528474, i64 noundef %_5.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i256.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !41274, !noalias !41275
  unreachable, !dbg !41274

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1261: ; preds = %bb19.i1254
  %_30.i1255 = add nuw i64 %iter.sroa.0.0.i12508472, 1, !dbg !41278
  %_15.i1268 = getelementptr inbounds nuw float, ptr %_54.0.i255.i, i64 %end.sroa.0.0.i12528474, !dbg !41283
  %_0.i2947 = load float, ptr %_15.i1268, align 4, !dbg !41285, !alias.scope !41287, !noalias !41241, !noundef !12
  %_3.i.i3765.inv = fcmp olt float %suffix.sroa.0.0.i12518473, %_0.i2947, !dbg !41290
  %_4.i.i3772.v = select i1 %_3.i.i3765.inv, float %suffix.sroa.0.0.i12518473, float %_0.i2947, !dbg !41290
  store float %_4.i.i3772.v, ptr %_15.i1268, align 4, !dbg !41293, !alias.scope !41296, !noalias !41241
  %754 = icmp eq i64 %end.sroa.0.0.i12528474, 0, !dbg !41299
  %spec.store.select.i1263 = select i1 %754, i64 %ring.i, i64 %end.sroa.0.0.i12528474, !dbg !41299
  %755 = add i64 %spec.store.select.i1263, -1, !dbg !41300
  %exitcond11154.not = icmp eq i64 %_30.i1255, %umax11153, !dbg !41301
  br i1 %exitcond11154.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1265, label %bb19.i1254, !dbg !41303

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1265: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1261, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1284
  %storemerge.i1246 = phi i32 [ %753, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1284 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1261 ], !dbg !41304
  %running.sroa.0.1.i1247 = phi float [ %_4.i.i3781.v, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1284 ], [ %running.sroa.0.0.i1241, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1261 ], !dbg !41070
  %_0.i2859 = fmul float %running.sroa.0.1.i1247, 1.638400e+04, !dbg !41305
  %756 = tail call noundef float @llvm.floor.f32(float %_0.i2859), !dbg !41307
  %_0.i2858 = fmul float %756, 0x3F10000000000000, !dbg !41311
  %or.cond.i1448.not = icmp ult i64 %_209.i, %_56.1.i278.i, !dbg !41313
  br i1 %or.cond.i1448.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1452, label %bb4.i1451, !dbg !41313, !prof !2723

bb4.i1451:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1265
  store float %history.i38.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40549
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40550
  store i32 %storemerge.i1246.lcssa85038712.lcssa13644, ptr %_22.i270.i, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313547.lcssa13666, ptr %_21.i269.i, align 4
  store i32 %storemerge.i1215.lcssa86158751.lcssa13687, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa1141713564.lcssa13709, ptr %_21.i.i, align 4
  store float %_0.i2899.lcssa1138213477, ptr %718, align 4
  store float %_0.i3273.lcssa1139813495, ptr %720, align 4
  store float %_0.i2895.lcssa1142213513, ptr %726, align 4
  store float %_0.i3269.lcssa1142313531, ptr %728, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313548, ptr %_21.i269.i, align 1, !dbg !41070
  store float %running.sroa.0.0.i1210.lcssa1141713565, ptr %_21.i.i, align 1, !dbg !41088
  %_5.i1445 = add i64 %_209.i, 1, !dbg !41318
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_209.i, i64 noundef %_5.i1445, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i278.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !41319, !noalias !41320
  unreachable, !dbg !41319

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1452: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1265
  %_15.i1449 = getelementptr inbounds nuw float, ptr %_56.0.i277.i, i64 %_209.i, !dbg !41323
  %_0.i2901 = load float, ptr %_15.i1449, align 4, !dbg !41325, !alias.scope !41327, !noalias !41330, !noundef !12
  %_0.i2419 = fadd float %_0.i2858, %_0.i28998536, !dbg !41331
  %_0.i2899 = fsub float %_0.i2419, %_0.i2901, !dbg !41333
  %_8.not.i3.i287.i = icmp ugt i64 %_7.i8.i258.i, %_56.1.i278.i
  br i1 %_8.not.i3.i287.i, label %bb4.i6.i316.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i288.i, !dbg !41335, !prof !165

bb4.i6.i316.i:                                    ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1452
  store float %history.i38.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40549
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40550
  store i32 %storemerge.i1246.lcssa85038712.lcssa13644, ptr %_22.i270.i, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313547.lcssa13666, ptr %_21.i269.i, align 4
  store i32 %storemerge.i1215.lcssa86158751.lcssa13687, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa1141713564.lcssa13709, ptr %_21.i.i, align 4
  store float %_0.i2899.lcssa1138213477, ptr %718, align 4
  store float %_0.i3273.lcssa1139813495, ptr %720, align 4
  store float %_0.i2895.lcssa1142213513, ptr %726, align 4
  store float %_0.i3269.lcssa1142313531, ptr %728, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313548, ptr %_21.i269.i, align 1, !dbg !41070
  store float %running.sroa.0.0.i1210.lcssa1141713565, ptr %_21.i.i, align 1, !dbg !41088
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_205.i, i64 noundef %_7.i8.i258.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i278.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !41340, !noalias !41341
  unreachable, !dbg !41340

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i288.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1452
  %_17.i5.i289.i = getelementptr inbounds nuw float, ptr %_56.0.i277.i, i64 %_205.i, !dbg !41344
  store float %_0.i2858, ptr %_17.i5.i289.i, align 4, !dbg !41346, !alias.scope !41348, !noalias !41330
  %_0.i2434 = fdiv float %_0.i2899, %_37.i290.i, !dbg !41351
  %_0.i2898 = fsub float 1.000000e+00, %_0.i2434, !dbg !41353
  %_0.i2897 = fsub float %_0.i2898, %_0.i32738565, !dbg !41355
  %_4.i2450 = fmul float %_9.i29.i, %_0.i2897, !dbg !41357
  %_0.i2451 = fadd float %_0.i32738565, %_4.i2450, !dbg !41357
  %_3.i.i3747.inv = fcmp ogt float %_0.i2898, %_0.i2451, !dbg !41359
  %_4.i.i3754.v = select i1 %_3.i.i3747.inv, float %_0.i2898, float %_0.i2451, !dbg !41359
  %757 = tail call noundef float @llvm.fabs.f32(float %_4.i.i3754.v), !dbg !41362
  %758 = fcmp uge float %757, 0x3BC79CA100000000, !dbg !41365
  %_0.i3273 = select i1 %758, float %_4.i.i3754.v, float 0.000000e+00, !dbg !41367
  %_5.i1437 = add i64 %_206.i, 1, !dbg !41368
  %or.cond.i1440.not = icmp ult i64 %_206.i, %_58.1.i303.i, !dbg !41370
  br i1 %or.cond.i1440.not, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3239, label %bb4.i1443, !dbg !41370, !prof !2723

bb4.i1443:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i288.i
  store float %history.i38.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40549
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40550
  store i32 %storemerge.i1246.lcssa85038712.lcssa13644, ptr %_22.i270.i, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313547.lcssa13666, ptr %_21.i269.i, align 4
  store i32 %storemerge.i1215.lcssa86158751.lcssa13687, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa1141713564.lcssa13709, ptr %_21.i.i, align 4
  store float %_0.i2899.lcssa1138213477, ptr %718, align 4
  store float %_0.i3273.lcssa1139813495, ptr %720, align 4
  store float %_0.i2895.lcssa1142213513, ptr %726, align 4
  store float %_0.i3269.lcssa1142313531, ptr %728, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313548, ptr %_21.i269.i, align 1, !dbg !41070
  store float %running.sroa.0.0.i1210.lcssa1141713565, ptr %_21.i.i, align 1, !dbg !41088
  %umax11158 = call i64 @llvm.umax.i64(i64 %main_cursor.sroa.0.1.i8707, i64 %_58.1.i303.i), !dbg !41125
  %759 = add i64 %umax11158, 1, !dbg !41125
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_206.i, i64 noundef %759, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i303.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !41374, !noalias !41375
  unreachable, !dbg !41374

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3239: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i288.i
  %_0.i2896 = fsub float 1.000000e+00, %_0.i3273, !dbg !41378
  %_15.i1441 = getelementptr inbounds nuw float, ptr %_58.0.i302.i, i64 %_206.i, !dbg !41380
  %_0.i2903 = load float, ptr %_15.i1441, align 4, !dbg !41382, !alias.scope !41384, !noalias !41330, !noundef !12
  store float %_0.i3088, ptr %_15.i1441, align 4, !dbg !41387, !alias.scope !41390, !noalias !41330
  %_0.i2857 = fmul float %_0.i2896, %_0.i2903, !dbg !41393
  %_6.i3508 = bitcast float %_0.i2903 to i32, !dbg !41395
  %_5.i3509 = and i32 %_6.i3508, %all.sroa.0.0.i, !dbg !41398
  %_8.i3510 = bitcast float %_0.i2857 to i32, !dbg !41399
  %_7.i3512 = and i32 %_9.i3511, %_8.i3510, !dbg !41401
  %_4.i3513 = or disjoint i32 %_7.i3512, %_5.i3509, !dbg !41398
  store i32 %_4.i3513, ptr %data.i.i.i.i.i.i.i.i4390, align 4, !dbg !41402, !alias.scope !41404, !noalias !41407
  %_212.i = add nuw i64 %iter.i.sroa.41.08480, %right_end.sroa.0.0.i1851, !dbg !41408
  %_214.i = add nuw i64 %iter.i.sroa.41.08480, %right_expiring.sroa.0.0.i1859, !dbg !41410
  %_8.not.i10.i.i = icmp ugt i64 %_7.i8.i258.i, %_54.1.i.i
  br i1 %_8.not.i10.i.i, label %bb4.i13.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i, !dbg !41411, !prof !165

bb4.i13.i.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3239
  store float %history.i38.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40549
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40550
  store i32 %storemerge.i1246.lcssa85038712.lcssa13644, ptr %_22.i270.i, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313547.lcssa13666, ptr %_21.i269.i, align 4
  store i32 %storemerge.i1215.lcssa86158751.lcssa13687, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa1141713564.lcssa13709, ptr %_21.i.i, align 4
  store float %_0.i2899.lcssa1138213477, ptr %718, align 4
  store float %_0.i3273.lcssa1139813495, ptr %720, align 4
  store float %_0.i2895.lcssa1142213513, ptr %726, align 4
  store float %_0.i3269.lcssa1142313531, ptr %728, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313548, ptr %_21.i269.i, align 1, !dbg !41070
  store float %running.sroa.0.0.i1210.lcssa1141713565, ptr %_21.i.i, align 1, !dbg !41088
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_205.i, i64 noundef %_7.i8.i258.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !41416, !noalias !41417
  unreachable, !dbg !41416

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3239
  %_0.i2433 = fdiv float %_8.i.i, %_0.i3527, !dbg !41425
  %_3.i2000 = fcmp uge float %_8.i.i, %_0.i3527, !dbg !41427
  %_0.i3507 = select i1 %_3.i2000, float 1.000000e+00, float %_0.i2433, !dbg !41429
  %_17.i12.i.i = getelementptr inbounds nuw float, ptr %_54.0.i.i, i64 %_205.i, !dbg !41431
  store float %_0.i3507, ptr %_17.i12.i.i, align 4, !dbg !41433, !alias.scope !41435, !noalias !41438
  %or.cond.i1320.not = icmp ult i64 %_212.i, %_54.1.i.i, !dbg !41439
  br i1 %or.cond.i1320.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1324, label %bb4.i1323, !dbg !41439, !prof !2723

bb4.i1323:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i
  store float %history.i38.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40549
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40550
  store i32 %storemerge.i1246.lcssa85038712.lcssa13644, ptr %_22.i270.i, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313547.lcssa13666, ptr %_21.i269.i, align 4
  store i32 %storemerge.i1215.lcssa86158751.lcssa13687, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa1141713564.lcssa13709, ptr %_21.i.i, align 4
  store float %_0.i2899.lcssa1138213477, ptr %718, align 4
  store float %_0.i3273.lcssa1139813495, ptr %720, align 4
  store float %_0.i2895.lcssa1142213513, ptr %726, align 4
  store float %_0.i3269.lcssa1142313531, ptr %728, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313548, ptr %_21.i269.i, align 1, !dbg !41070
  store float %running.sroa.0.0.i1210.lcssa1141713565, ptr %_21.i.i, align 1, !dbg !41088
  %_5.i1317 = add i64 %_212.i, 1, !dbg !41444
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_212.i, i64 noundef %_5.i1317, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !41445, !noalias !41446
  unreachable, !dbg !41445

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1324: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i
  %_15.i1321 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i64 %_212.i, !dbg !41452
  %_0.i2933 = load float, ptr %_15.i1321, align 4, !dbg !41454, !alias.scope !41456, !noalias !41459, !noundef !12
  %position.i1206 = zext i32 %storemerge.i12158593 to i64, !dbg !41460
  %760 = icmp eq i32 %storemerge.i12158593, 0, !dbg !41461
  %_3.i.i3810.inv = fcmp olt float %running.sroa.0.0.i12108620, %_0.i2933, !dbg !41461
  %_4.i.i3817.v = select i1 %_3.i.i3810.inv, float %running.sroa.0.0.i12108620, float %_0.i2933, !dbg !41461
  %running.sroa.0.0.i1210 = select i1 %760, float %_0.i2933, float %_4.i.i3817.v, !dbg !41461
  %_15.i1211 = add nuw nsw i64 %position.i1206, 1, !dbg !41462
  %complete.i1212 = icmp eq i64 %_15.i1211, %_18.i.i, !dbg !41462
  br i1 %complete.i1212, label %bb19.i1223, label %bb7.i1213, !dbg !41463

bb7.i1213:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1324
  %or.cond.i1312.not = icmp ult i64 %_208.i, %_54.1.i.i, !dbg !41464
  br i1 %or.cond.i1312.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1316, label %bb4.i1315, !dbg !41464, !prof !2723

bb4.i1315:                                        ; preds = %bb7.i1213
  store float %history.i38.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40549
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40550
  store i32 %storemerge.i1246.lcssa85038712.lcssa13644, ptr %_22.i270.i, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313547.lcssa13666, ptr %_21.i269.i, align 4
  store i32 %storemerge.i1215.lcssa86158751.lcssa13687, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa1141713564.lcssa13709, ptr %_21.i.i, align 4
  store float %_0.i2899.lcssa1138213477, ptr %718, align 4
  store float %_0.i3273.lcssa1139813495, ptr %720, align 4
  store float %_0.i2895.lcssa1142213513, ptr %726, align 4
  store float %_0.i3269.lcssa1142313531, ptr %728, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313548, ptr %_21.i269.i, align 1, !dbg !41070
  store float %running.sroa.0.0.i1210.lcssa1141713565, ptr %_21.i.i, align 1, !dbg !41088
  %_5.i1309 = add i64 %_208.i, 1, !dbg !41469
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_208.i, i64 noundef %_5.i1309, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !41470, !noalias !41471
  unreachable, !dbg !41470

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1316: ; preds = %bb7.i1213
  %_15.i1313 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i64 %_208.i, !dbg !41474
  %_0.i2935 = load float, ptr %_15.i1313, align 4, !dbg !41476, !alias.scope !41478, !noalias !41459, !noundef !12
  %_3.i.i3801.inv = fcmp olt float %_0.i2935, %running.sroa.0.0.i1210, !dbg !41481
  %_4.i.i3808.v = select i1 %_3.i.i3801.inv, float %_0.i2935, float %running.sroa.0.0.i1210, !dbg !41481
  %761 = trunc i64 %_15.i1211 to i32, !dbg !41484
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1234, !dbg !41485

bb19.i1223:                                       ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1324, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230
  %end.sroa.0.0.i12218477 = phi i64 [ %763, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230 ], [ %_212.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1324 ]
  %suffix.sroa.0.0.i12208476 = phi float [ %_4.i.i3799.v, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230 ], [ %_0.i2933, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1324 ]
  %iter.sroa.0.0.i12198475 = phi i64 [ %_30.i1224, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1324 ]
  %or.cond.i1296.not = icmp ult i64 %end.sroa.0.0.i12218477, %_54.1.i.i, !dbg !41486
  br i1 %or.cond.i1296.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230, label %bb4.i1299, !dbg !41486, !prof !2723

bb4.i1299:                                        ; preds = %bb19.i1223
  store float %history.i38.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40549
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40550
  store i32 %storemerge.i1246.lcssa85038712.lcssa13644, ptr %_22.i270.i, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313547.lcssa13666, ptr %_21.i269.i, align 4
  store i32 %storemerge.i1215.lcssa86158751.lcssa13687, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa1141713564.lcssa13709, ptr %_21.i.i, align 4
  store float %_0.i2899.lcssa1138213477, ptr %718, align 4
  store float %_0.i3273.lcssa1139813495, ptr %720, align 4
  store float %_0.i2895.lcssa1142213513, ptr %726, align 4
  store float %_0.i3269.lcssa1142313531, ptr %728, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313548, ptr %_21.i269.i, align 1, !dbg !41070
  store float %running.sroa.0.0.i1210.lcssa1141713565, ptr %_21.i.i, align 1, !dbg !41088
  %_5.i1293 = add i64 %end.sroa.0.0.i12218477, 1, !dbg !41491
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %end.sroa.0.0.i12218477, i64 noundef %_5.i1293, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !41492, !noalias !41493
  unreachable, !dbg !41492

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230: ; preds = %bb19.i1223
  %_30.i1224 = add nuw i64 %iter.sroa.0.0.i12198475, 1, !dbg !41496
  %_15.i1297 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i64 %end.sroa.0.0.i12218477, !dbg !41501
  %_0.i2939 = load float, ptr %_15.i1297, align 4, !dbg !41503, !alias.scope !41505, !noalias !41459, !noundef !12
  %_3.i.i3792.inv = fcmp olt float %suffix.sroa.0.0.i12208476, %_0.i2939, !dbg !41508
  %_4.i.i3799.v = select i1 %_3.i.i3792.inv, float %suffix.sroa.0.0.i12208476, float %_0.i2939, !dbg !41508
  store float %_4.i.i3799.v, ptr %_15.i1297, align 4, !dbg !41511, !alias.scope !41514, !noalias !41459
  %762 = icmp eq i64 %end.sroa.0.0.i12218477, 0, !dbg !41517
  %spec.store.select.i1232 = select i1 %762, i64 %ring.i, i64 %end.sroa.0.0.i12218477, !dbg !41517
  %763 = add i64 %spec.store.select.i1232, -1, !dbg !41518
  %exitcond11156.not = icmp eq i64 %_30.i1224, %umax11155, !dbg !41519
  br i1 %exitcond11156.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1234, label %bb19.i1223, !dbg !41521

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1234: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1316
  %storemerge.i1215 = phi i32 [ %761, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1316 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230 ], !dbg !41522
  %running.sroa.0.1.i1216 = phi float [ %_4.i.i3808.v, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1316 ], [ %running.sroa.0.0.i1210, %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230 ], !dbg !41088
  %_0.i2856 = fmul float %running.sroa.0.1.i1216, 1.638400e+04, !dbg !41523
  %764 = tail call noundef float @llvm.floor.f32(float %_0.i2856), !dbg !41525
  %_0.i2855 = fmul float %764, 0x3F10000000000000, !dbg !41529
  %or.cond.i1432.not = icmp ult i64 %_214.i, %_56.1.i.i, !dbg !41531
  br i1 %or.cond.i1432.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1436, label %bb4.i1435, !dbg !41531, !prof !2723

bb4.i1435:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1234
  store float %history.i38.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40549
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40550
  store i32 %storemerge.i1246.lcssa85038712.lcssa13644, ptr %_22.i270.i, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313547.lcssa13666, ptr %_21.i269.i, align 4
  store i32 %storemerge.i1215.lcssa86158751.lcssa13687, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa1141713564.lcssa13709, ptr %_21.i.i, align 4
  store float %_0.i2899.lcssa1138213477, ptr %718, align 4
  store float %_0.i3273.lcssa1139813495, ptr %720, align 4
  store float %_0.i2895.lcssa1142213513, ptr %726, align 4
  store float %_0.i3269.lcssa1142313531, ptr %728, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313548, ptr %_21.i269.i, align 1, !dbg !41070
  store float %running.sroa.0.0.i1210.lcssa1141713565, ptr %_21.i.i, align 1, !dbg !41088
  %_5.i1429 = add i64 %_214.i, 1, !dbg !41536
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_214.i, i64 noundef %_5.i1429, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !41537, !noalias !41538
  unreachable, !dbg !41537

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1436: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1234
  %_15.i1433 = getelementptr inbounds nuw float, ptr %_56.0.i.i, i64 %_214.i, !dbg !41541
  %_0.i2905 = load float, ptr %_15.i1433, align 4, !dbg !41543, !alias.scope !41545, !noalias !41548, !noundef !12
  %_0.i2418 = fadd float %_0.i2855, %_0.i28958648, !dbg !41549
  %_0.i2895 = fsub float %_0.i2418, %_0.i2905, !dbg !41551
  %_8.not.i3.i.i = icmp ugt i64 %_7.i8.i258.i, %_56.1.i.i
  br i1 %_8.not.i3.i.i, label %bb4.i6.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i, !dbg !41553, !prof !165

bb4.i6.i.i:                                       ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1436
  store float %history.i38.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40549
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40550
  store i32 %storemerge.i1246.lcssa85038712.lcssa13644, ptr %_22.i270.i, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313547.lcssa13666, ptr %_21.i269.i, align 4
  store i32 %storemerge.i1215.lcssa86158751.lcssa13687, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa1141713564.lcssa13709, ptr %_21.i.i, align 4
  store float %_0.i2899.lcssa1138213477, ptr %718, align 4
  store float %_0.i3273.lcssa1139813495, ptr %720, align 4
  store float %_0.i2895.lcssa1142213513, ptr %726, align 4
  store float %_0.i3269.lcssa1142313531, ptr %728, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313548, ptr %_21.i269.i, align 1, !dbg !41070
  store float %running.sroa.0.0.i1210.lcssa1141713565, ptr %_21.i.i, align 1, !dbg !41088
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_205.i, i64 noundef %_7.i8.i258.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !41558, !noalias !41559
  unreachable, !dbg !41558

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_lanefEB2_.exit1436
  %_17.i5.i.i = getelementptr inbounds nuw float, ptr %_56.0.i.i, i64 %_205.i, !dbg !41562
  store float %_0.i2855, ptr %_17.i5.i.i, align 4, !dbg !41564, !alias.scope !41566, !noalias !41548
  %_6.not.i1423 = icmp ugt i64 %_5.i1437, %_58.1.i.i
  br i1 %_6.not.i1423, label %bb4.i1427, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3232, !dbg !41569, !prof !165

bb4.i1427:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i
  store float %history.i38.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40549
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40550
  store i32 %storemerge.i1246.lcssa85038712.lcssa13644, ptr %_22.i270.i, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313547.lcssa13666, ptr %_21.i269.i, align 4
  store i32 %storemerge.i1215.lcssa86158751.lcssa13687, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa1141713564.lcssa13709, ptr %_21.i.i, align 4
  store float %_0.i2899.lcssa1138213477, ptr %718, align 4
  store float %_0.i3273.lcssa1139813495, ptr %720, align 4
  store float %_0.i2895.lcssa1142213513, ptr %726, align 4
  store float %_0.i3269.lcssa1142313531, ptr %728, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313548, ptr %_21.i269.i, align 1, !dbg !41070
  store float %running.sroa.0.0.i1210.lcssa1141713565, ptr %_21.i.i, align 1, !dbg !41088
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_206.i, i64 noundef %_5.i1437, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !41574, !noalias !41575
  unreachable, !dbg !41574

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3232: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i
  %_0.i2432 = fdiv float %_0.i2895, %_37.i.i, !dbg !41578
  %_0.i2894 = fsub float 1.000000e+00, %_0.i2432, !dbg !41580
  %_0.i2893 = fsub float %_0.i2894, %_0.i32698677, !dbg !41582
  %_4.i2448 = fmul float %_9.i.i, %_0.i2893, !dbg !41584
  %_0.i2449 = fadd float %_0.i32698677, %_4.i2448, !dbg !41584
  %_3.i.i3738.inv = fcmp ogt float %_0.i2894, %_0.i2449, !dbg !41586
  %_4.i.i3745.v = select i1 %_3.i.i3738.inv, float %_0.i2894, float %_0.i2449, !dbg !41586
  %765 = tail call noundef float @llvm.fabs.f32(float %_4.i.i3745.v), !dbg !41589
  %766 = fcmp uge float %765, 0x3BC79CA100000000, !dbg !41592
  %_0.i3269 = select i1 %766, float %_4.i.i3745.v, float 0.000000e+00, !dbg !41594
  %_0.i2892 = fsub float 1.000000e+00, %_0.i3269, !dbg !41595
  %_15.i1425 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %_206.i, !dbg !41597
  %_0.i2907 = load float, ptr %_15.i1425, align 4, !dbg !41599, !alias.scope !41601, !noalias !41548, !noundef !12
  store float %_0.i3083, ptr %_15.i1425, align 4, !dbg !41604, !alias.scope !41607, !noalias !41548
  %_0.i2854 = fmul float %_0.i2892, %_0.i2907, !dbg !41610
  %_6.i3495 = bitcast float %_0.i2907 to i32, !dbg !41612
  %_5.i3496 = and i32 %_6.i3495, %all.sroa.0.0.i, !dbg !41615
  %_8.i3497 = bitcast float %_0.i2854 to i32, !dbg !41616
  %_7.i3499 = and i32 %_9.i3511, %_8.i3497, !dbg !41618
  %_4.i3500 = or disjoint i32 %_7.i3499, %_5.i3496, !dbg !41615
  store i32 %_4.i3500, ptr %data.i5.i.i.i.i.i.i.i4404, align 4, !dbg !41619, !alias.scope !41621, !noalias !41624
  %exitcond11167.not = icmp eq i64 %_9.0.i4406, %umin11166, !dbg !41125
  br i1 %exitcond11167.not, label %bb32.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3085, !dbg !41125

bb32.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3232, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit4374
  %running.sroa.0.0.i1210.lcssa1141713564 = phi float [ %running.sroa.0.0.i1210.lcssa1141713565, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit4374 ], [ %running.sroa.0.0.i1210, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3232 ]
  %running.sroa.0.0.i1241.lcssa1136313547 = phi float [ %running.sroa.0.0.i1241.lcssa1136313548, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit4374 ], [ %running.sroa.0.0.i1241, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3232 ]
  %_0.i3269.lcssa1142313530 = phi float [ %_0.i3269.lcssa1142313531, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit4374 ], [ %_0.i3269, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3232 ]
  %_0.i2895.lcssa1142213512 = phi float [ %_0.i2895.lcssa1142213513, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit4374 ], [ %_0.i2895, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3232 ]
  %_0.i3273.lcssa1139813494 = phi float [ %_0.i3273.lcssa1139813495, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit4374 ], [ %_0.i3273, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3232 ]
  %_0.i2899.lcssa1138213476 = phi float [ %_0.i2899.lcssa1138213477, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit4374 ], [ %_0.i2899, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3232 ]
  %running.sroa.0.0.i1210.lcssa86428787 = phi float [ %running.sroa.0.0.i1210.lcssa86428788, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit4374 ], [ %running.sroa.0.0.i1210, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3232 ]
  %storemerge.i1215.lcssa86158751 = phi i32 [ %storemerge.i1215.lcssa86158752, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit4374 ], [ %storemerge.i1215, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3232 ]
  %running.sroa.0.0.i1241.lcssa85308748 = phi float [ %running.sroa.0.0.i1241.lcssa85308749, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit4374 ], [ %running.sroa.0.0.i1241, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3232 ]
  %storemerge.i1246.lcssa85038712 = phi i32 [ %storemerge.i1246.lcssa85038713, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit4374 ], [ %storemerge.i1246, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3232 ]
  %_138.i = add i64 %..i4307, %ring_cursor.sroa.0.1.i8706, !dbg !41625
  %_204.not.i = icmp ult i64 %_138.i, %ring.i, !dbg !41626
  %767 = select i1 %_204.not.i, i64 0, i64 %ring.i, !dbg !41626
  %ring_cursor.sroa.0.2.i = sub nuw i64 %_138.i, %767, !dbg !41626
  %_140.i = add i64 %..i4307, %main_cursor.sroa.0.1.i8707, !dbg !41629
  %_215.not.i = icmp ult i64 %_140.i, %main.i, !dbg !41630
  %768 = select i1 %_215.not.i, i64 0, i64 %main.i, !dbg !41630
  %main_cursor.sroa.0.2.i = sub nuw i64 %_140.i, %768, !dbg !41630
  %_58.i = icmp ult i64 %_80.i, %..i4276, !dbg !40514
  br i1 %_58.i, label %bb20.i, label %bb19.i.bb15.i.loopexit_crit_edge, !dbg !40514

_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit: ; preds = %bb15.i.loopexit
  store float %history.i38.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !40549
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !40550
  store i32 %storemerge.i1246.lcssa85038712.lcssa13643, ptr %_22.i270.i, align 4
  store float %running.sroa.0.0.i1241.lcssa1136313547.lcssa13665, ptr %_21.i269.i, align 4
  store i32 %storemerge.i1215.lcssa86158751.lcssa13686, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa1141713564.lcssa13708, ptr %_21.i.i, align 4
  %769 = trunc i64 %main_cursor.sroa.0.1.i.lcssa to i32, !dbg !41632
  %770 = trunc i64 %ring_cursor.sroa.0.1.i.lcssa to i32, !dbg !41634
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit, !dbg !41635

_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit, %bb6.i
  %ring_cursor.sroa.0.0.i.lcssa = phi i32 [ %_36.i, %bb6.i ], [ %770, %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit ], !dbg !40482
  %main_cursor.sroa.0.0.i.lcssa = phi i32 [ %_35.i, %bb6.i ], [ %769, %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit ], !dbg !40479
  %771 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 72, !dbg !41635
  %left_prefix.i = load float, ptr %771, align 8, !dbg !41635, !noalias !40486, !noundef !12
  %772 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 76, !dbg !41636
  %left_phase.i = load i32, ptr %772, align 4, !dbg !41636, !noalias !40486, !noundef !12
  %773 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 72, !dbg !41637
  %right_prefix.i = load float, ptr %773, align 8, !dbg !41637, !noalias !40486, !noundef !12
  %774 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 76, !dbg !41638
  %right_phase.i = load i32, ptr %774, align 4, !dbg !41638, !noalias !40486, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_right.i), !dbg !41639, !noalias !40486
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i), !dbg !41640, !noalias !40486
  %775 = getelementptr inbounds nuw i8, ptr %self, i64 216, !dbg !41641
  %_230.0.i = load ptr, ptr %775, align 8, !dbg !41641, !alias.scope !40453, !noalias !41642, !nonnull !12, !noundef !12
  %776 = getelementptr inbounds nuw i8, ptr %self, i64 224, !dbg !41641
  %_230.1.i = load i64, ptr %776, align 8, !dbg !41641, !alias.scope !40453, !noalias !41642, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !41643), !dbg !41646
  %_4.not.i3217 = icmp eq i64 %_230.1.i, 0, !dbg !41647
  br i1 %_4.not.i3217, label %panic.i3219, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3220, !dbg !41647

panic.i3219:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !41647, !noalias !41649
  unreachable, !dbg !41647

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3220: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit
  store float %left_prefix.i, ptr %_230.0.i, align 4, !dbg !41647, !alias.scope !41643, !noalias !40457
  %_231.0.i = load ptr, ptr %68, align 8, !dbg !41650, !alias.scope !40453, !noalias !41642, !nonnull !12, !noundef !12
  %_231.1.i = load i64, ptr %69, align 8, !dbg !41650, !alias.scope !40453, !noalias !41642, !noundef !12
  %777 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i), !dbg !41651
  br i1 %777, label %bb2.i4425, label %bb6.i4416, !dbg !41651

bb6.i4416:                                        ; preds = %bb2.i4425, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3220
  %end_or_len.idx.i4417 = shl nuw nsw i64 %_231.1.i, 2, !dbg !41655
  %end_or_len.i4418 = getelementptr inbounds nuw i8, ptr %_231.0.i, i64 %end_or_len.idx.i4417, !dbg !41655
  %_293.i4419 = icmp eq i64 %_231.1.i, 0, !dbg !41659
  br i1 %_293.i4419, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4431, label %bb10.i4420, !dbg !41662

bb2.i4425:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3220
  %bytes1.sroa.0.0.zext.i4426 = and i32 %left_phase.i, 255, !dbg !41663
  %bytes1.sroa.0.0.isplat.i4427 = mul nuw i32 %bytes1.sroa.0.0.zext.i4426, 16843009, !dbg !41663
  %_5.i4428 = icmp eq i32 %left_phase.i, %bytes1.sroa.0.0.isplat.i4427, !dbg !41664
  br i1 %_5.i4428, label %bb3.i4429, label %bb6.i4416, !dbg !41664

bb3.i4429:                                        ; preds = %bb2.i4425
  %bytes.sroa.0.0.extract.trunc.i4430 = trunc i32 %left_phase.i to i8, !dbg !41665
  %778 = shl nuw nsw i64 %_231.1.i, 2, !dbg !41667
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_231.0.i, i8 %bytes.sroa.0.0.extract.trunc.i4430, i64 %778, i1 false), !dbg !41667, !alias.scope !41668, !noalias !40457
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4431, !dbg !41671

bb10.i4420:                                       ; preds = %bb6.i4416, %bb10.i4420
  %iter.sroa.0.04.i4421 = phi ptr [ %_38.i4422, %bb10.i4420 ], [ %_231.0.i, %bb6.i4416 ]
  %_38.i4422 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i4421, i64 4, !dbg !41672
  store i32 %left_phase.i, ptr %iter.sroa.0.04.i4421, align 4, !dbg !41674, !alias.scope !41668, !noalias !40457
  %_29.i4423 = icmp eq ptr %_38.i4422, %end_or_len.i4418, !dbg !41659
  br i1 %_29.i4423, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4431, label %bb10.i4420, !dbg !41662

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4431: ; preds = %bb10.i4420, %bb6.i4416, %bb3.i4429
  %779 = getelementptr inbounds nuw i8, ptr %self, i64 416, !dbg !41675
  %_232.0.i = load ptr, ptr %779, align 8, !dbg !41675, !alias.scope !40455, !noalias !41676, !nonnull !12, !noundef !12
  %780 = getelementptr inbounds nuw i8, ptr %self, i64 424, !dbg !41675
  %_232.1.i = load i64, ptr %780, align 8, !dbg !41675, !alias.scope !40455, !noalias !41676, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !41677), !dbg !41680
  %_4.not.i3213 = icmp eq i64 %_232.1.i, 0, !dbg !41681
  br i1 %_4.not.i3213, label %panic.i3215, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3216, !dbg !41681

panic.i3215:                                      ; preds = %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4431
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !41681, !noalias !41683
  unreachable, !dbg !41681

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3216: ; preds = %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4431
  store float %right_prefix.i, ptr %_232.0.i, align 4, !dbg !41681, !alias.scope !41677, !noalias !40457
  %_233.0.i = load ptr, ptr %77, align 8, !dbg !41684, !alias.scope !40455, !noalias !41676, !nonnull !12, !noundef !12
  %_233.1.i = load i64, ptr %78, align 8, !dbg !41684, !alias.scope !40455, !noalias !41676, !noundef !12
  %781 = tail call i1 @llvm.is.constant.i32(i32 %right_phase.i), !dbg !41685
  br i1 %781, label %bb2.i4441, label %bb6.i4432, !dbg !41685

bb6.i4432:                                        ; preds = %bb2.i4441, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3216
  %end_or_len.idx.i4433 = shl nuw nsw i64 %_233.1.i, 2, !dbg !41688
  %end_or_len.i4434 = getelementptr inbounds nuw i8, ptr %_233.0.i, i64 %end_or_len.idx.i4433, !dbg !41688
  %_293.i4435 = icmp eq i64 %_233.1.i, 0, !dbg !41692
  br i1 %_293.i4435, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4447, label %bb10.i4436, !dbg !41695

bb2.i4441:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3216
  %bytes1.sroa.0.0.zext.i4442 = and i32 %right_phase.i, 255, !dbg !41696
  %bytes1.sroa.0.0.isplat.i4443 = mul nuw i32 %bytes1.sroa.0.0.zext.i4442, 16843009, !dbg !41696
  %_5.i4444 = icmp eq i32 %right_phase.i, %bytes1.sroa.0.0.isplat.i4443, !dbg !41697
  br i1 %_5.i4444, label %bb3.i4445, label %bb6.i4432, !dbg !41697

bb3.i4445:                                        ; preds = %bb2.i4441
  %bytes.sroa.0.0.extract.trunc.i4446 = trunc i32 %right_phase.i to i8, !dbg !41698
  %782 = shl nuw nsw i64 %_233.1.i, 2, !dbg !41700
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_233.0.i, i8 %bytes.sroa.0.0.extract.trunc.i4446, i64 %782, i1 false), !dbg !41700, !alias.scope !41701, !noalias !40457
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4447, !dbg !41704

bb10.i4436:                                       ; preds = %bb6.i4432, %bb10.i4436
  %iter.sroa.0.04.i4437 = phi ptr [ %_38.i4438, %bb10.i4436 ], [ %_233.0.i, %bb6.i4432 ]
  %_38.i4438 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i4437, i64 4, !dbg !41705
  store i32 %right_phase.i, ptr %iter.sroa.0.04.i4437, align 4, !dbg !41707, !alias.scope !41701, !noalias !40457
  %_29.i4439 = icmp eq ptr %_38.i4438, %end_or_len.i4434, !dbg !41692
  br i1 %_29.i4439, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4447, label %bb10.i4436, !dbg !41695

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4447: ; preds = %bb10.i4436, %bb6.i4432, %bb3.i4445
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %hot_left.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33) #31, !dbg !41708
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %hot_right.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34) #31, !dbg !41709
  store i32 %main_cursor.sroa.0.0.i.lcssa, ptr %_35, align 4, !dbg !41632, !alias.scope !40457, !noalias !40481
  store i32 %ring_cursor.sroa.0.0.i.lcssa, ptr %673, align 4, !dbg !41634, !alias.scope !40457, !noalias !40481
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i), !dbg !41710, !noalias !40486
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i), !dbg !41711, !noalias !40486
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockfEB2_.exit, !dbg !40450

_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockfEB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit, %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit, %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4271, %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4447
  br i1 %quiet.sroa.0.04641, label %bb28, label %bb40, !dbg !41712

bb22:                                             ; preds = %bb20
  tail call void @llvm.experimental.noalias.scope.decl(metadata !41713), !dbg !41716
  %783 = getelementptr inbounds nuw i8, ptr %self, i64 248, !dbg !41717
  %_40.0.i = load ptr, ptr %783, align 8, !dbg !41717, !alias.scope !41713, !nonnull !12, !noundef !12
  %784 = getelementptr inbounds nuw i8, ptr %self, i64 256, !dbg !41717
  %_40.1.i = load i64, ptr %784, align 8, !dbg !41717, !alias.scope !41713, !noundef !12
  %785 = getelementptr inbounds nuw i8, ptr %self, i64 312, !dbg !41719
  %_41.0.i = load ptr, ptr %785, align 8, !dbg !41719, !alias.scope !41713, !nonnull !12, !noundef !12
  %786 = getelementptr inbounds nuw i8, ptr %self, i64 320, !dbg !41719
  %_41.1.i = load i64, ptr %786, align 8, !dbg !41719, !alias.scope !41713, !noundef !12
  %..i.i.i.i = tail call noundef i64 @llvm.umin.i64(i64 %_41.1.i, i64 %_40.1.i), !dbg !41720
  %_2.i6.not.i = icmp eq i64 %..i.i.i.i, 0, !dbg !41726
  br i1 %_2.i6.not.i, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb4.i4448, !dbg !41726

bb4.i4448:                                        ; preds = %bb22, %bb6.i4450
  %iter.sroa.8.07.i = phi i64 [ %787, %bb6.i4450 ], [ 0, %bb22 ]
  %_3.i1.i.i = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i, i64 %iter.sroa.8.07.i, !dbg !41729
  %_14.i4449 = load i32, ptr %_3.i1.i.i, align 4, !dbg !41732, !noalias !41713, !noundef !12
  %_20.i = icmp eq i32 %_14.i4449, 0, !dbg !41733
  br i1 %_20.i, label %panic.i4459, label %bb6.i4450, !dbg !41733

bb6.i4450:                                        ; preds = %bb4.i4448
  %_3.i.i.i4451 = getelementptr inbounds nuw i32, ptr %_40.0.i, i64 %iter.sroa.8.07.i, !dbg !41734
  %787 = add nuw i64 %iter.sroa.8.07.i, 1, !dbg !41737
  %window.i4452 = zext i32 %_14.i4449 to i64, !dbg !41732
  %_18.i4453 = load i32, ptr %_3.i.i.i4451, align 4, !dbg !41738, !noalias !41713, !noundef !12
  %_17.i4454 = zext i32 %_18.i4453 to i64, !dbg !41738
  %_19.i4455 = urem i64 %frames, %window.i4452, !dbg !41733
  %_16.i4456 = add nuw nsw i64 %_19.i4455, %_17.i4454, !dbg !41739
  %_15.i4457 = urem i64 %_16.i4456, %window.i4452, !dbg !41740
  %788 = trunc nuw i64 %_15.i4457 to i32, !dbg !41741
  store i32 %788, ptr %_3.i.i.i4451, align 4, !dbg !41741, !noalias !41713
  %exitcond.not.i = icmp eq i64 %787, %..i.i.i.i, !dbg !41726
  br i1 %exitcond.not.i, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb4.i4448, !dbg !41726

panic.i4459:                                      ; preds = %bb4.i4448
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bac57976a2bdbfad4a3a85d5d1c7648c) #30, !dbg !41733, !noalias !41713
  unreachable, !dbg !41733

_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit: ; preds = %bb6.i4450, %bb22
  tail call void @llvm.experimental.noalias.scope.decl(metadata !41742), !dbg !41745
  %789 = getelementptr inbounds nuw i8, ptr %self, i64 448, !dbg !41746
  %_40.0.i4460 = load ptr, ptr %789, align 8, !dbg !41746, !alias.scope !41742, !nonnull !12, !noundef !12
  %790 = getelementptr inbounds nuw i8, ptr %self, i64 456, !dbg !41746
  %_40.1.i4461 = load i64, ptr %790, align 8, !dbg !41746, !alias.scope !41742, !noundef !12
  %791 = getelementptr inbounds nuw i8, ptr %self, i64 512, !dbg !41748
  %_41.0.i4462 = load ptr, ptr %791, align 8, !dbg !41748, !alias.scope !41742, !nonnull !12, !noundef !12
  %792 = getelementptr inbounds nuw i8, ptr %self, i64 520, !dbg !41748
  %_41.1.i4463 = load i64, ptr %792, align 8, !dbg !41748, !alias.scope !41742, !noundef !12
  %..i.i.i.i4464 = tail call noundef i64 @llvm.umin.i64(i64 %_41.1.i4463, i64 %_40.1.i4461), !dbg !41749
  %_2.i6.not.i4465 = icmp eq i64 %..i.i.i.i4464, 0, !dbg !41755
  br i1 %_2.i6.not.i4465, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit4482, label %bb4.i4466, !dbg !41755

bb4.i4466:                                        ; preds = %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, %bb6.i4471
  %iter.sroa.8.07.i4467 = phi i64 [ %793, %bb6.i4471 ], [ 0, %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit ]
  %_3.i1.i.i4468 = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i4462, i64 %iter.sroa.8.07.i4467, !dbg !41758
  %_14.i4469 = load i32, ptr %_3.i1.i.i4468, align 4, !dbg !41761, !noalias !41742, !noundef !12
  %_20.i4470 = icmp eq i32 %_14.i4469, 0, !dbg !41762
  br i1 %_20.i4470, label %panic.i4481, label %bb6.i4471, !dbg !41762

bb6.i4471:                                        ; preds = %bb4.i4466
  %_3.i.i.i4472 = getelementptr inbounds nuw i32, ptr %_40.0.i4460, i64 %iter.sroa.8.07.i4467, !dbg !41763
  %793 = add nuw i64 %iter.sroa.8.07.i4467, 1, !dbg !41766
  %window.i4473 = zext i32 %_14.i4469 to i64, !dbg !41761
  %_18.i4474 = load i32, ptr %_3.i.i.i4472, align 4, !dbg !41767, !noalias !41742, !noundef !12
  %_17.i4475 = zext i32 %_18.i4474 to i64, !dbg !41767
  %_19.i4476 = urem i64 %frames, %window.i4473, !dbg !41762
  %_16.i4477 = add nuw nsw i64 %_19.i4476, %_17.i4475, !dbg !41768
  %_15.i4478 = urem i64 %_16.i4477, %window.i4473, !dbg !41769
  %794 = trunc nuw i64 %_15.i4478 to i32, !dbg !41770
  store i32 %794, ptr %_3.i.i.i4472, align 4, !dbg !41770, !noalias !41742
  %exitcond.not.i4479 = icmp eq i64 %793, %..i.i.i.i4464, !dbg !41755
  br i1 %exitcond.not.i4479, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit4482, label %bb4.i4466, !dbg !41755

panic.i4481:                                      ; preds = %bb4.i4466
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bac57976a2bdbfad4a3a85d5d1c7648c) #30, !dbg !41762, !noalias !41742
  unreachable, !dbg !41762

_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit4482: ; preds = %bb6.i4471, %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit
  %795 = getelementptr inbounds nuw i8, ptr %self, i64 544, !dbg !41771
  %_29.val = load i64, ptr %795, align 8, !dbg !41771
  %796 = getelementptr inbounds nuw i8, ptr %self, i64 552, !dbg !41771
  %_29.val3873 = load i64, ptr %796, align 8, !dbg !41771, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !41772), !dbg !41771
  %_10.i4483 = icmp eq i64 %_29.val3873, 0, !dbg !41775
  br i1 %_10.i4483, label %panic.i4495, label %bb1.i4484, !dbg !41775

bb1.i4484:                                        ; preds = %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit4482
  %_28 = getelementptr inbounds nuw i8, ptr %self, i64 560, !dbg !41777
  %_7.i = load i32, ptr %_28, align 4, !dbg !41778, !alias.scope !41772, !noundef !12
  %_6.i4485 = zext i32 %_7.i to i64, !dbg !41778
  %_8.i4486 = urem i64 %frames, %_29.val3873, !dbg !41775
  %_5.i4487 = add nuw nsw i64 %_8.i4486, %_6.i4485, !dbg !41779
  %_4.i4488 = urem i64 %_5.i4487, %_29.val3873, !dbg !41780
  %797 = trunc i64 %_4.i4488 to i32, !dbg !41781
  store i32 %797, ptr %_28, align 4, !dbg !41781, !alias.scope !41772
  %_17.i4489 = icmp eq i64 %_29.val, 0, !dbg !41782
  br i1 %_17.i4489, label %panic2.i, label %_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit, !dbg !41782

panic.i4495:                                      ; preds = %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit4482
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f0ee36f67d9a332211aa5518dd2ebfd5) #30, !dbg !41775, !noalias !41772
  unreachable, !dbg !41775

panic2.i:                                         ; preds = %bb1.i4484
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_33d4d33e0a850133578789055882dcf9) #30, !dbg !41782, !noalias !41772
  unreachable, !dbg !41782

_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit: ; preds = %bb1.i4484
  %798 = getelementptr inbounds nuw i8, ptr %self, i64 564, !dbg !41783
  %_14.i4491 = load i32, ptr %798, align 4, !dbg !41783, !alias.scope !41772, !noundef !12
  %_13.i4492 = zext i32 %_14.i4491 to i64, !dbg !41783
  %_15.i4493 = urem i64 %frames, %_29.val, !dbg !41782
  %_12.i = add nuw nsw i64 %_15.i4493, %_13.i4492, !dbg !41784
  %_11.i4494 = urem i64 %_12.i, %_29.val, !dbg !41785
  %799 = trunc i64 %_11.i4494 to i32, !dbg !41786
  store i32 %799, ptr %798, align 4, !dbg !41786, !alias.scope !41772
  br label %bb42, !dbg !41787

bb28:                                             ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockfEB2_.exit
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_37 = tail call noundef zeroext i1 @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #31, !dbg !41788
  br i1 %_37, label %bb30, label %bb40, !dbg !41789

bb30:                                             ; preds = %bb28
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_39 = tail call noundef zeroext i1 @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #31, !dbg !41790
  br i1 %_39, label %bb32, label %bb40, !dbg !41791

bb32:                                             ; preds = %bb30
  %_80.not = icmp ugt i64 %frames, %left_io.1
  br i1 %_80.not, label %bb51, label %bb1.i4496, !dbg !41792, !prof !165

bb51:                                             ; preds = %bb32
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %frames, i64 noundef %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bf99c8ff01c64f6a0b34983a37a16939) #30, !dbg !41800
  unreachable, !dbg !41800

bb1.i4496:                                        ; preds = %bb32, %bb10.i4510
  %iter.sroa.6.0.i4497 = phi i64 [ %len.i.i.i.i4502, %bb10.i4510 ], [ %frames, %bb32 ], !dbg !41801
  %iter.sroa.0.0.i4498 = phi ptr [ %data.i.i.i.i4501, %bb10.i4510 ], [ %left_io.0, %bb32 ], !dbg !41801
  %800 = icmp eq i64 %iter.sroa.6.0.i4497, 0, !dbg !41803
  br i1 %800, label %bb34, label %bb11.preheader.i4499, !dbg !41803

bb11.preheader.i4499:                             ; preds = %bb1.i4496
  %..i.i.i4500 = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i4497, i64 32), !dbg !41805
  %_18.idx.i4503 = shl nuw nsw i64 %..i.i.i4500, 2, !dbg !41808
  %_18.i4504 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i4498, i64 %_18.idx.i4503, !dbg !41808
  br label %bb11.i4505, !dbg !41813

bb11.i4505:                                       ; preds = %bb11.i4505, %bb11.preheader.i4499
  %iter1.sroa.0.014.i4506 = phi ptr [ %_31.i4508, %bb11.i4505 ], [ %iter.sroa.0.0.i4498, %bb11.preheader.i4499 ]
  %bits.sroa.0.013.i4507 = phi i32 [ %801, %bb11.i4505 ], [ 0, %bb11.preheader.i4499 ]
  %_31.i4508 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i4506, i64 4, !dbg !41815
  %_134.i4509 = load i32, ptr %iter1.sroa.0.014.i4506, align 4, !dbg !41817, !alias.scope !41818, !noundef !12
  %801 = or i32 %_134.i4509, %bits.sroa.0.013.i4507, !dbg !41821
  %_25.i = icmp eq ptr %_31.i4508, %_18.i4504, !dbg !41822
  br i1 %_25.i, label %bb10.i4510, label %bb11.i4505, !dbg !41813

bb10.i4510:                                       ; preds = %bb11.i4505
  %data.i.i.i.i4501 = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i4498, i64 %..i.i.i4500, !dbg !41824
  %len.i.i.i.i4502 = sub nuw nsw i64 %iter.sroa.6.0.i4497, %..i.i.i4500, !dbg !41829
  %802 = icmp eq i32 %801, 0, !dbg !41830
  br i1 %802, label %bb1.i4496, label %bb40, !dbg !41830

bb34:                                             ; preds = %bb1.i4496
  %_88.not = icmp ugt i64 %frames, %right_io.1, !dbg !41831
  br i1 %_88.not, label %bb54, label %bb1.i4525, !dbg !41831, !prof !639

bb40:                                             ; preds = %bb10.i4510, %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockfEB2_.exit, %bb28, %bb30, %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit4541
  %_36.sroa.0.0 = phi i8 [ %831, %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit4541 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockfEB2_.exit ], [ 0, %bb30 ], [ 0, %bb28 ], [ 0, %bb10.i4510 ], !dbg !41837
  store i8 %_36.sroa.0.0, ptr %38, align 4, !dbg !41838
  %803 = load i8, ptr %2, align 8, !dbg !41839, !range !17, !noundef !12
  store i8 %803, ptr %0, align 1, !dbg !41840
  call void @llvm.lifetime.start.p0(ptr nonnull %shape), !dbg !41841
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %shape, ptr noundef nonnull align 8 dereferenceable(24) %_32, i64 24, i1 false), !dbg !41842
  %804 = getelementptr inbounds nuw i8, ptr %self, i64 72, !dbg !41843
  %805 = load i32, ptr %804, align 8, !dbg !41843, !noundef !12
  %_53 = getelementptr inbounds nuw i8, ptr %self, i64 568, !dbg !41845
  %806 = getelementptr inbounds nuw i8, ptr %self, i64 104, !dbg !41852
  %_99.0 = load ptr, ptr %806, align 8, !dbg !41852, !nonnull !12, !noundef !12
  %807 = getelementptr inbounds nuw i8, ptr %self, i64 112, !dbg !41852
  %_99.1 = load i64, ptr %807, align 8, !dbg !41852, !noundef !12
  %808 = getelementptr inbounds nuw i8, ptr %self, i64 120, !dbg !41852
  %_100.0 = load ptr, ptr %808, align 8, !dbg !41852, !nonnull !12, !noundef !12
  %809 = getelementptr inbounds nuw i8, ptr %self, i64 128, !dbg !41852
  %_100.1 = load i64, ptr %809, align 8, !dbg !41852, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !41853), !dbg !41856
  tail call void @llvm.experimental.noalias.scope.decl(metadata !41857), !dbg !41856
  tail call void @llvm.experimental.noalias.scope.decl(metadata !41859), !dbg !41856
  %_22.not.i1463.i = icmp eq i64 %left_io.1, 0, !dbg !41861
  br i1 %_22.not.i1463.i, label %bb6.i.preheader.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i, !dbg !41861

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i: ; preds = %bb40, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i
  %ok.sroa.0.0.i1366.i = phi i32 [ %_0.i33.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i ], [ -1, %bb40 ]
  %iter.sroa.0.0.i1265.i = phi ptr [ %_27.i16.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i ], [ %left_io.0, %bb40 ]
  %iter.sroa.5.0.i1164.i = phi i64 [ %_28.i17.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i ], [ %left_io.1, %bb40 ]
  %_27.i16.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i1265.i, i64 4, !dbg !41874
  %_28.i17.i = add nsw i64 %iter.sroa.5.0.i1164.i, -1, !dbg !41881
  %_0.i28.i = load float, ptr %iter.sroa.0.0.i1265.i, align 4, !dbg !41882, !alias.scope !41885, !noalias !41888, !noundef !12
  %810 = tail call noundef float @llvm.fabs.f32(float %_0.i28.i), !dbg !41890
  %_3.i.i4512 = fcmp olt float %810, 0x46293E5940000000, !dbg !41893
  %_0.i33.i = select i1 %_3.i.i4512, i32 %ok.sroa.0.0.i1366.i, i32 0, !dbg !41896
  %_22.not.i14.i = icmp eq i64 %_28.i17.i, 0, !dbg !41861
  br i1 %_22.not.i14.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdvPQf9CMsz3_17true_peak_limiter.exit25.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i, !dbg !41861

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdvPQf9CMsz3_17true_peak_limiter.exit25.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i
  %811 = icmp eq i32 %_0.i33.i, -1, !dbg !41899
  br i1 %811, label %bb6.i.preheader.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i, !dbg !41902

bb6.i.preheader.i:                                ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdvPQf9CMsz3_17true_peak_limiter.exit25.i, %bb40
  %_22.not.i67.i = icmp eq i64 %right_io.1, 0, !dbg !41903
  br i1 %_22.not.i67.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockfNCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i, !dbg !41903

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i: ; preds = %bb6.i.preheader.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i
  %ok.sroa.0.0.i70.i = phi i32 [ %_0.i34.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i ], [ -1, %bb6.i.preheader.i ]
  %iter.sroa.0.0.i69.i = phi ptr [ %_27.i.i4523, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i ], [ %right_io.0, %bb6.i.preheader.i ]
  %iter.sroa.5.0.i68.i = phi i64 [ %_28.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i ], [ %right_io.1, %bb6.i.preheader.i ]
  %_27.i.i4523 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i69.i, i64 4, !dbg !41907
  %_28.i.i = add nsw i64 %iter.sroa.5.0.i68.i, -1, !dbg !41910
  %_0.i30.i = load float, ptr %iter.sroa.0.0.i69.i, align 4, !dbg !41911, !alias.scope !41913, !noalias !41916, !noundef !12
  %812 = tail call noundef float @llvm.fabs.f32(float %_0.i30.i), !dbg !41917
  %_3.i26.i = fcmp olt float %812, 0x46293E5940000000, !dbg !41919
  %_0.i34.i = select i1 %_3.i26.i, i32 %ok.sroa.0.0.i70.i, i32 0, !dbg !41921
  %_22.not.i.i = icmp eq i64 %_28.i.i, 0, !dbg !41903
  br i1 %_22.not.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i, !dbg !41903

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdvPQf9CMsz3_17true_peak_limiter.exit.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i
  %813 = icmp eq i32 %_0.i34.i, -1, !dbg !41923
  br i1 %813, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockfNCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit, label %bb7.i4524, !dbg !41925

bb7.i4524:                                        ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdvPQf9CMsz3_17true_peak_limiter.exit.i
  br i1 %_22.not.i1463.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i, !dbg !41926

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i: ; preds = %bb7.i4524, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdvPQf9CMsz3_17true_peak_limiter.exit25.i
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i, !dbg !41926

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i
  %ok.sroa.0.017.i.i = phi i32 [ %_0.i7.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ -1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i ]
  %iter.sroa.0.016.i.i = phi ptr [ %_45.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ %left_io.0, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i ]
  %iter.sroa.5.015.i.i = phi i64 [ %_46.i.i4513, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ %left_io.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i ]
  %_45.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.016.i.i, i64 4, !dbg !41937
  %_46.i.i4513 = add nsw i64 %iter.sroa.5.015.i.i, -1, !dbg !41944
  %_0.i.i.i4514 = load float, ptr %iter.sroa.0.016.i.i, align 4, !dbg !41945, !alias.scope !41948, !noalias !41888, !noundef !12
  %814 = tail call noundef float @llvm.fabs.f32(float %_0.i.i.i4514), !dbg !41953
  %_3.i.i.i4515 = fcmp olt float %814, 0x46293E5940000000, !dbg !41956
  %_0.i7.i.i = select i1 %_3.i.i.i4515, i32 %ok.sroa.0.017.i.i, i32 0, !dbg !41958
  %_40.not.i.i = icmp eq i64 %_46.i.i4513, 0, !dbg !41926
  br i1 %_40.not.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i, !dbg !41926

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i
  %815 = and i32 %_0.i7.i.i, 1065353216, !dbg !41960
  %816 = icmp ne i32 %815, 1065353216, !dbg !41963
  %817 = zext i1 %816 to i32, !dbg !41963
  %_40.not14.i40.i = icmp eq i64 %right_io.1, 0, !dbg !41967
  br i1 %_40.not14.i40.i, label %bb14.i.preheader.thread.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i, !dbg !41967

bb14.i.preheader.thread.i:                        ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit.i
  %818 = getelementptr inbounds nuw i8, ptr %self, i64 576, !dbg !41971
  store i32 %817, ptr %818, align 8, !dbg !41971, !alias.scope !41859, !noalias !41972
  %_1481.i = load i64, ptr %_53, align 8, !dbg !41973, !alias.scope !41859, !noalias !41972, !noundef !12
  %819 = tail call i64 @llvm.uadd.sat.i64(i64 %_1481.i, i64 1), !dbg !41974
  store i64 %819, ptr %_53, align 8, !dbg !41977, !alias.scope !41859, !noalias !41972
  br label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit60.i, !dbg !41978

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i: ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit.i, %bb7.i4524
  %ok.sroa.0.0.lcssa.i76.i = phi i32 [ %817, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit.i ], [ 0, %bb7.i4524 ]
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.i, !dbg !41967

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i
  %ok.sroa.0.017.i42.i = phi i32 [ %_0.i7.i49.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.i ], [ -1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i ]
  %iter.sroa.0.016.i43.i = phi ptr [ %_45.i45.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.i ], [ %right_io.0, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i ]
  %iter.sroa.5.015.i44.i = phi i64 [ %_46.i46.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.i ], [ %right_io.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i ]
  %_45.i45.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.016.i43.i, i64 4, !dbg !41983
  %_46.i46.i = add nsw i64 %iter.sroa.5.015.i44.i, -1, !dbg !41986
  %_0.i.i47.i = load float, ptr %iter.sroa.0.016.i43.i, align 4, !dbg !41987, !alias.scope !41989, !noalias !41916, !noundef !12
  %820 = tail call noundef float @llvm.fabs.f32(float %_0.i.i47.i), !dbg !41994
  %_3.i.i48.i = fcmp olt float %820, 0x46293E5940000000, !dbg !41996
  %_0.i7.i49.i = select i1 %_3.i.i48.i, i32 %ok.sroa.0.017.i42.i, i32 0, !dbg !41998
  %_40.not.i50.i = icmp eq i64 %_46.i46.i, 0, !dbg !41967
  br i1 %_40.not.i50.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit53.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.i, !dbg !41967

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit53.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i41.i
  %821 = and i32 %_0.i7.i49.i, 1065353216, !dbg !42000
  %822 = icmp ne i32 %821, 1065353216, !dbg !42002
  %823 = zext i1 %822 to i32, !dbg !42002
  %824 = or i32 %ok.sroa.0.0.lcssa.i76.i, %823, !dbg !41971
  %825 = getelementptr inbounds nuw i8, ptr %self, i64 576, !dbg !41971
  store i32 %824, ptr %825, align 8, !dbg !41971, !alias.scope !41859, !noalias !41972
  %_14.i4516 = load i64, ptr %_53, align 8, !dbg !41973, !alias.scope !41859, !noalias !41972, !noundef !12
  %826 = tail call i64 @llvm.uadd.sat.i64(i64 %_14.i4516, i64 1), !dbg !41974
  store i64 %826, ptr %_53, align 8, !dbg !41977, !alias.scope !41859, !noalias !41972
  br i1 %_22.not.i1463.i, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit60.i, label %bb14.i.preheader.i, !dbg !42003

bb14.i.preheader.i:                               ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit53.i
  %.idx.i.i = shl nuw nsw i64 %left_io.1, 2, !dbg !42007
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %left_io.0, i8 0, i64 %.idx.i.i, i1 false), !dbg !42011, !alias.scope !42012, !noalias !41888
  br label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit60.i, !dbg !41978

_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit60.i: ; preds = %bb14.i.preheader.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit53.i, %bb14.i.preheader.thread.i
  %left.1.sink.i = phi i64 [ %left_io.1, %bb14.i.preheader.thread.i ], [ %right_io.1, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit53.i ], [ %right_io.1, %bb14.i.preheader.i ]
  %left.0.sink.i = phi ptr [ %left_io.0, %bb14.i.preheader.thread.i ], [ %right_io.0, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdvPQf9CMsz3_17true_peak_limiter.exit53.i ], [ %right_io.0, %bb14.i.preheader.i ]
  %.idx.i85.i = shl nuw nsw i64 %left.1.sink.i, 2, !dbg !42015
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %left.0.sink.i, i8 0, i64 %.idx.i85.i, i1 false), !dbg !42021, !alias.scope !42022, !noalias !42023
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 8 dereferenceable(200) %_33, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_99.0, i64 noundef %_99.1, i32 noundef %805) #31, !dbg !42024, !noalias !42027
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 8 dereferenceable(200) %_34, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_100.0, i64 noundef %_100.1, i32 noundef %805) #31, !dbg !42030, !noalias !42027
  store i32 0, ptr %_35, align 4, !dbg !42031, !noalias !42027
  %827 = getelementptr inbounds nuw i8, ptr %self, i64 564, !dbg !42031
  store i32 0, ptr %827, align 4, !dbg !42031, !noalias !42027
  br label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockfNCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit, !dbg !42032

_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockfNCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit: ; preds = %bb6.i.preheader.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdvPQf9CMsz3_17true_peak_limiter.exit.i, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit60.i
  call void @llvm.lifetime.end.p0(ptr nonnull %shape), !dbg !42033
  br label %bb42, !dbg !41787

bb54:                                             ; preds = %bb34
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %frames, i64 noundef %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_15903c4e5f99181283027046d243e508) #30, !dbg !42034
  unreachable, !dbg !42034

bb1.i4525:                                        ; preds = %bb34, %bb10.i4540
  %iter.sroa.6.0.i4526 = phi i64 [ %len.i.i.i.i4531, %bb10.i4540 ], [ %frames, %bb34 ], !dbg !42035
  %iter.sroa.0.0.i4527 = phi ptr [ %data.i.i.i.i4530, %bb10.i4540 ], [ %right_io.0, %bb34 ], !dbg !42035
  %828 = icmp eq i64 %iter.sroa.6.0.i4526, 0, !dbg !42037
  br i1 %828, label %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit4541, label %bb11.preheader.i4528, !dbg !42037

bb11.preheader.i4528:                             ; preds = %bb1.i4525
  %..i.i.i4529 = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i4526, i64 32), !dbg !42039
  %_18.idx.i4532 = shl nuw nsw i64 %..i.i.i4529, 2, !dbg !42042
  %_18.i4533 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i4527, i64 %_18.idx.i4532, !dbg !42042
  br label %bb11.i4534, !dbg !42047

bb11.i4534:                                       ; preds = %bb11.i4534, %bb11.preheader.i4528
  %iter1.sroa.0.014.i4535 = phi ptr [ %_31.i4537, %bb11.i4534 ], [ %iter.sroa.0.0.i4527, %bb11.preheader.i4528 ]
  %bits.sroa.0.013.i4536 = phi i32 [ %829, %bb11.i4534 ], [ 0, %bb11.preheader.i4528 ]
  %_31.i4537 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i4535, i64 4, !dbg !42049
  %_134.i4538 = load i32, ptr %iter1.sroa.0.014.i4535, align 4, !dbg !42051, !alias.scope !42052, !noundef !12
  %829 = or i32 %_134.i4538, %bits.sroa.0.013.i4536, !dbg !42055
  %_25.i4539 = icmp eq ptr %_31.i4537, %_18.i4533, !dbg !42056
  br i1 %_25.i4539, label %bb10.i4540, label %bb11.i4534, !dbg !42047

bb10.i4540:                                       ; preds = %bb11.i4534
  %data.i.i.i.i4530 = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i4527, i64 %..i.i.i4529, !dbg !42058
  %len.i.i.i.i4531 = sub nuw nsw i64 %iter.sroa.6.0.i4526, %..i.i.i4529, !dbg !42063
  %830 = icmp eq i32 %829, 0, !dbg !42064
  br i1 %830, label %bb1.i4525, label %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit4541, !dbg !42064

_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit4541: ; preds = %bb1.i4525, %bb10.i4540
  %831 = zext i1 %828 to i8, !dbg !41838
  br label %bb40, !dbg !41712

bb42:                                             ; preds = %_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockfNCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit
  ret void, !dbg !41787
}
