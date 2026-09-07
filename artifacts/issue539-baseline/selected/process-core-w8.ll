define internal fastcc void @_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13process_blockB5_(ptr noalias noundef nonnull align 32 dereferenceable(2176) %self, ptr noalias noundef nonnull align 4 captures(address) %left_io.0, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef nonnull align 4 captures(address) %right_io.0, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 4294967296) %frames) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !11398 {
start:
  %peaks_right.i216 = alloca [1024 x i8], align 4
  %peaks_left.i217 = alloca [1024 x i8], align 4
  %scratch.i = alloca [32 x i8], align 4
  %hot_right.i223 = alloca [736 x i8], align 32
  %hot_left.i224 = alloca [736 x i8], align 32
  %right_prefix.i = alloca [32 x i8], align 32
  %left_prefix.i = alloca [32 x i8], align 32
  %uniform_right.i = alloca [128 x i8], align 32
  %uniform_left.i = alloca [128 x i8], align 32
  %peaks_right.i = alloca [1024 x i8], align 4
  %peaks_left.i = alloca [1024 x i8], align 4
  %hot_right.i = alloca [736 x i8], align 32
  %hot_left.i = alloca [736 x i8], align 32
  %shape = alloca [24 x i8], align 8
  %words = shl nuw nsw i64 %frames, 3, !dbg !11399
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 2153, !dbg !11400
  %1 = load i8, ptr %0, align 1, !dbg !11400, !range !5399, !noundef !12
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 2144, !dbg !11402
  %3 = load i8, ptr %2, align 32, !dbg !11402, !range !5399, !noundef !12
  %_7 = icmp eq i8 %1, %3, !dbg !11400
  br i1 %_7, label %bb1, label %bb20.thread, !dbg !11400

bb1:                                              ; preds = %start
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 1776, !dbg !11403
  %_95.0 = load ptr, ptr %4, align 16, !dbg !11403, !nonnull !12, !noundef !12
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 1784, !dbg !11403
  %_95.1 = load i64, ptr %5, align 8, !dbg !11403, !noundef !12
  %_8.i3021 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_95.0, i64 %_95.1, !dbg !11404
  br label %bb1.i.i3022, !dbg !11409

bb1.i.i3022:                                      ; preds = %bb13.i.i3023, %bb1
  %_221.i.i = phi ptr [ %_22.i.i3024, %bb13.i.i3023 ], [ %_95.0, %bb1 ]
  %_12.i.i = icmp eq ptr %_221.i.i, %_8.i3021, !dbg !11411
  br i1 %_12.i.i, label %bb3, label %bb13.i.i3023, !dbg !11414

bb13.i.i3023:                                     ; preds = %bb1.i.i3022
  %_22.i.i3024 = getelementptr inbounds nuw i8, ptr %_221.i.i, i64 16, !dbg !11415
  %6 = getelementptr inbounds nuw i8, ptr %_221.i.i, i64 12, !dbg !11417
  %_3.i.i.i = load i32, ptr %6, align 4, !dbg !11417, !alias.scope !11419, !noalias !11424, !noundef !12
  %7 = icmp eq i32 %_3.i.i.i, 0, !dbg !11417
  %_51.i.i.i = load i32, ptr %_221.i.i, align 4, !dbg !11417, !alias.scope !11419, !noalias !11424
  %8 = getelementptr inbounds nuw i8, ptr %_221.i.i, i64 4, !dbg !11417
  %_72.i.i.i = load i32, ptr %8, align 4, !dbg !11417, !alias.scope !11419, !noalias !11424
  %9 = icmp eq i32 %_51.i.i.i, %_72.i.i.i, !dbg !11417
  %_0.sroa.0.0.i.i.i = select i1 %7, i1 %9, i1 false, !dbg !11417
  br i1 %_0.sroa.0.0.i.i.i, label %bb1.i.i3022, label %bb20.thread, !dbg !11427

bb3:                                              ; preds = %bb1.i.i3022
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 1792, !dbg !11428
  %_96.0 = load ptr, ptr %10, align 16, !dbg !11428, !nonnull !12, !noundef !12
  %11 = getelementptr inbounds nuw i8, ptr %self, i64 1800, !dbg !11428
  %_96.1 = load i64, ptr %11, align 8, !dbg !11428, !noundef !12
  %_8.i3025 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_96.0, i64 %_96.1, !dbg !11429
  br label %bb1.i.i3026, !dbg !11434

bb1.i.i3026:                                      ; preds = %bb13.i.i3029, %bb3
  %_221.i.i3027 = phi ptr [ %_22.i.i3030, %bb13.i.i3029 ], [ %_96.0, %bb3 ]
  %_12.i.i3028 = icmp eq ptr %_221.i.i3027, %_8.i3025, !dbg !11436
  br i1 %_12.i.i3028, label %bb5, label %bb13.i.i3029, !dbg !11439

bb13.i.i3029:                                     ; preds = %bb1.i.i3026
  %_22.i.i3030 = getelementptr inbounds nuw i8, ptr %_221.i.i3027, i64 16, !dbg !11440
  %12 = getelementptr inbounds nuw i8, ptr %_221.i.i3027, i64 12, !dbg !11442
  %_3.i.i.i3031 = load i32, ptr %12, align 4, !dbg !11442, !alias.scope !11444, !noalias !11449, !noundef !12
  %13 = icmp eq i32 %_3.i.i.i3031, 0, !dbg !11442
  %_51.i.i.i3032 = load i32, ptr %_221.i.i3027, align 4, !dbg !11442, !alias.scope !11444, !noalias !11449
  %14 = getelementptr inbounds nuw i8, ptr %_221.i.i3027, i64 4, !dbg !11442
  %_72.i.i.i3033 = load i32, ptr %14, align 4, !dbg !11442, !alias.scope !11444, !noalias !11449
  %15 = icmp eq i32 %_51.i.i.i3032, %_72.i.i.i3033, !dbg !11442
  %_0.sroa.0.0.i.i.i3034 = select i1 %13, i1 %15, i1 false, !dbg !11442
  br i1 %_0.sroa.0.0.i.i.i3034, label %bb1.i.i3026, label %bb20.thread, !dbg !11452

bb5:                                              ; preds = %bb1.i.i3026
  %16 = getelementptr inbounds nuw i8, ptr %self, i64 1976, !dbg !11453
  %_97.0 = load ptr, ptr %16, align 8, !dbg !11453, !nonnull !12, !noundef !12
  %17 = getelementptr inbounds nuw i8, ptr %self, i64 1984, !dbg !11453
  %_97.1 = load i64, ptr %17, align 8, !dbg !11453, !noundef !12
  %_8.i3036 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_97.0, i64 %_97.1, !dbg !11454
  br label %bb1.i.i3037, !dbg !11459

bb1.i.i3037:                                      ; preds = %bb13.i.i3040, %bb5
  %_221.i.i3038 = phi ptr [ %_22.i.i3041, %bb13.i.i3040 ], [ %_97.0, %bb5 ]
  %_12.i.i3039 = icmp eq ptr %_221.i.i3038, %_8.i3036, !dbg !11461
  br i1 %_12.i.i3039, label %bb7, label %bb13.i.i3040, !dbg !11464

bb13.i.i3040:                                     ; preds = %bb1.i.i3037
  %_22.i.i3041 = getelementptr inbounds nuw i8, ptr %_221.i.i3038, i64 16, !dbg !11465
  %18 = getelementptr inbounds nuw i8, ptr %_221.i.i3038, i64 12, !dbg !11467
  %_3.i.i.i3042 = load i32, ptr %18, align 4, !dbg !11467, !alias.scope !11469, !noalias !11474, !noundef !12
  %19 = icmp eq i32 %_3.i.i.i3042, 0, !dbg !11467
  %_51.i.i.i3043 = load i32, ptr %_221.i.i3038, align 4, !dbg !11467, !alias.scope !11469, !noalias !11474
  %20 = getelementptr inbounds nuw i8, ptr %_221.i.i3038, i64 4, !dbg !11467
  %_72.i.i.i3044 = load i32, ptr %20, align 4, !dbg !11467, !alias.scope !11469, !noalias !11474
  %21 = icmp eq i32 %_51.i.i.i3043, %_72.i.i.i3044, !dbg !11467
  %_0.sroa.0.0.i.i.i3045 = select i1 %19, i1 %21, i1 false, !dbg !11467
  br i1 %_0.sroa.0.0.i.i.i3045, label %bb1.i.i3037, label %bb20.thread, !dbg !11477

bb7:                                              ; preds = %bb1.i.i3037
  %22 = getelementptr inbounds nuw i8, ptr %self, i64 1992, !dbg !11478
  %_98.0 = load ptr, ptr %22, align 8, !dbg !11478, !nonnull !12, !noundef !12
  %23 = getelementptr inbounds nuw i8, ptr %self, i64 2000, !dbg !11478
  %_98.1 = load i64, ptr %23, align 8, !dbg !11478, !noundef !12
  %_8.i3047 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_98.0, i64 %_98.1, !dbg !11479
  br label %bb1.i.i3048, !dbg !11484

bb1.i.i3048:                                      ; preds = %bb13.i.i3051, %bb7
  %_221.i.i3049 = phi ptr [ %_22.i.i3052, %bb13.i.i3051 ], [ %_98.0, %bb7 ]
  %_12.i.i3050 = icmp eq ptr %_221.i.i3049, %_8.i3047, !dbg !11486
  br i1 %_12.i.i3050, label %bb9, label %bb13.i.i3051, !dbg !11489

bb13.i.i3051:                                     ; preds = %bb1.i.i3048
  %_22.i.i3052 = getelementptr inbounds nuw i8, ptr %_221.i.i3049, i64 16, !dbg !11490
  %24 = getelementptr inbounds nuw i8, ptr %_221.i.i3049, i64 12, !dbg !11492
  %_3.i.i.i3053 = load i32, ptr %24, align 4, !dbg !11492, !alias.scope !11494, !noalias !11499, !noundef !12
  %25 = icmp eq i32 %_3.i.i.i3053, 0, !dbg !11492
  %_51.i.i.i3054 = load i32, ptr %_221.i.i3049, align 4, !dbg !11492, !alias.scope !11494, !noalias !11499
  %26 = getelementptr inbounds nuw i8, ptr %_221.i.i3049, i64 4, !dbg !11492
  %_72.i.i.i3055 = load i32, ptr %26, align 4, !dbg !11492, !alias.scope !11494, !noalias !11499
  %27 = icmp eq i32 %_51.i.i.i3054, %_72.i.i.i3055, !dbg !11492
  %_0.sroa.0.0.i.i.i3056 = select i1 %25, i1 %27, i1 false, !dbg !11492
  br i1 %_0.sroa.0.0.i.i.i3056, label %bb1.i.i3048, label %bb20.thread, !dbg !11502

bb9:                                              ; preds = %bb1.i.i3048
  %_65.not = icmp samesign ugt i64 %words, %left_io.1
  br i1 %_65.not, label %bb45, label %bb1.i3058, !dbg !11503, !prof !5262

bb45:                                             ; preds = %bb9
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %words, i64 noundef %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_449c9992b9e28dda1c528a8de6026d01) #31, !dbg !11512
  unreachable, !dbg !11512

bb1.i3058:                                        ; preds = %bb9, %bb10.i
  %iter.sroa.6.0.i = phi i64 [ %len.i.i.i.i, %bb10.i ], [ %words, %bb9 ], !dbg !11513
  %iter.sroa.0.0.i3059 = phi ptr [ %data.i.i.i.i, %bb10.i ], [ %left_io.0, %bb9 ], !dbg !11513
  %28 = icmp eq i64 %iter.sroa.6.0.i, 0, !dbg !11515
  br i1 %28, label %bb11, label %bb11.preheader.i, !dbg !11515

bb11.preheader.i:                                 ; preds = %bb1.i3058
  %..i.i.i = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i, i64 32), !dbg !11517
  %_18.idx.i = shl nuw nsw i64 %..i.i.i, 2, !dbg !11520
  %_18.i3060 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i3059, i64 %_18.idx.i, !dbg !11520
  br label %bb11.i3061, !dbg !11525

bb11.i3061:                                       ; preds = %bb11.i3061, %bb11.preheader.i
  %iter1.sroa.0.014.i = phi ptr [ %_31.i3062, %bb11.i3061 ], [ %iter.sroa.0.0.i3059, %bb11.preheader.i ]
  %bits.sroa.0.013.i = phi i32 [ %29, %bb11.i3061 ], [ 0, %bb11.preheader.i ]
  %_31.i3062 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i, i64 4, !dbg !11527
  %_134.i3063 = load i32, ptr %iter1.sroa.0.014.i, align 4, !dbg !11529, !alias.scope !11530, !noundef !12
  %29 = or i32 %_134.i3063, %bits.sroa.0.013.i, !dbg !11533
  %_25.i3064 = icmp eq ptr %_31.i3062, %_18.i3060, !dbg !11534
  br i1 %_25.i3064, label %bb10.i, label %bb11.i3061, !dbg !11525

bb10.i:                                           ; preds = %bb11.i3061
  %data.i.i.i.i = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i3059, i64 %..i.i.i, !dbg !11536
  %len.i.i.i.i = sub nuw nsw i64 %iter.sroa.6.0.i, %..i.i.i, !dbg !11541
  %30 = icmp eq i32 %29, 0, !dbg !11542
  br i1 %30, label %bb1.i3058, label %bb20.thread, !dbg !11542

bb11:                                             ; preds = %bb1.i3058
  %_73.not = icmp samesign ugt i64 %words, %right_io.1, !dbg !11543
  br i1 %_73.not, label %bb48, label %bb1.i3065, !dbg !11543, !prof !905

bb20.thread:                                      ; preds = %bb13.i.i3023, %bb13.i.i3029, %bb13.i.i3040, %bb13.i.i3051, %bb10.i, %start
  %31 = getelementptr inbounds nuw i8, ptr %self, i64 2152
  br label %bb26, !dbg !11549

bb20:                                             ; preds = %bb1.i3065
  %32 = getelementptr inbounds nuw i8, ptr %self, i64 2152
  %33 = load i8, ptr %32, align 8, !range !5399
  %_22 = trunc nuw i8 %33 to i1
  br i1 %_22, label %bb22, label %bb26, !dbg !11549

bb48:                                             ; preds = %bb11
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %words, i64 noundef %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_69177cbbe364d953a90a8930cefca3fe) #31, !dbg !11551
  unreachable, !dbg !11551

bb1.i3065:                                        ; preds = %bb11, %bb10.i3080
  %iter.sroa.6.0.i3066 = phi i64 [ %len.i.i.i.i3071, %bb10.i3080 ], [ %words, %bb11 ], !dbg !11552
  %iter.sroa.0.0.i3067 = phi ptr [ %data.i.i.i.i3070, %bb10.i3080 ], [ %right_io.0, %bb11 ], !dbg !11552
  %34 = icmp eq i64 %iter.sroa.6.0.i3066, 0, !dbg !11554
  br i1 %34, label %bb20, label %bb11.preheader.i3068, !dbg !11554

bb11.preheader.i3068:                             ; preds = %bb1.i3065
  %..i.i.i3069 = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i3066, i64 32), !dbg !11556
  %_18.idx.i3072 = shl nuw nsw i64 %..i.i.i3069, 2, !dbg !11559
  %_18.i3073 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i3067, i64 %_18.idx.i3072, !dbg !11559
  br label %bb11.i3074, !dbg !11564

bb11.i3074:                                       ; preds = %bb11.i3074, %bb11.preheader.i3068
  %iter1.sroa.0.014.i3075 = phi ptr [ %_31.i3077, %bb11.i3074 ], [ %iter.sroa.0.0.i3067, %bb11.preheader.i3068 ]
  %bits.sroa.0.013.i3076 = phi i32 [ %35, %bb11.i3074 ], [ 0, %bb11.preheader.i3068 ]
  %_31.i3077 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i3075, i64 4, !dbg !11566
  %_134.i3078 = load i32, ptr %iter1.sroa.0.014.i3075, align 4, !dbg !11568, !alias.scope !11569, !noundef !12
  %35 = or i32 %_134.i3078, %bits.sroa.0.013.i3076, !dbg !11572
  %_25.i3079 = icmp eq ptr %_31.i3077, %_18.i3073, !dbg !11573
  br i1 %_25.i3079, label %bb10.i3080, label %bb11.i3074, !dbg !11564

bb10.i3080:                                       ; preds = %bb11.i3074
  %data.i.i.i.i3070 = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i3067, i64 %..i.i.i3069, !dbg !11575
  %len.i.i.i.i3071 = sub nuw nsw i64 %iter.sroa.6.0.i3066, %..i.i.i3069, !dbg !11580
  %36 = icmp eq i32 %35, 0, !dbg !11581
  br i1 %36, label %bb1.i3065, label %bb20.thread6182, !dbg !11581

bb20.thread6182:                                  ; preds = %bb10.i3080
  %37 = getelementptr inbounds nuw i8, ptr %self, i64 2152
  br label %bb26, !dbg !11549

bb26:                                             ; preds = %bb20.thread6182, %bb20.thread, %bb20
  %38 = phi ptr [ %31, %bb20.thread ], [ %32, %bb20 ], [ %37, %bb20.thread6182 ]
  %quiet.sroa.0.06181 = phi i1 [ false, %bb20.thread ], [ true, %bb20 ], [ false, %bb20.thread6182 ]
  %_32 = getelementptr inbounds nuw i8, ptr %self, i64 1616, !dbg !11582
  %_33 = getelementptr inbounds nuw i8, ptr %self, i64 1648, !dbg !11583
  %_34 = getelementptr inbounds nuw i8, ptr %self, i64 1848, !dbg !11584
  %_35 = getelementptr inbounds nuw i8, ptr %self, i64 1640, !dbg !11585
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11586), !dbg !11589
  %39 = getelementptr inbounds nuw i8, ptr %self, i64 1824, !dbg !11592
  %_31.0.i = load ptr, ptr %39, align 8, !dbg !11592, !alias.scope !11586, !noalias !11594, !nonnull !12, !noundef !12
  %40 = getelementptr inbounds nuw i8, ptr %self, i64 1832, !dbg !11592
  %_31.1.i = load i64, ptr %40, align 8, !dbg !11592, !alias.scope !11586, !noalias !11594, !noundef !12
  %_17.idx.i = mul nuw nsw i64 %_31.1.i, 12, !dbg !11602
  %_17.i = getelementptr inbounds nuw i8, ptr %_31.0.i, i64 %_17.idx.i, !dbg !11602
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11606), !dbg !11609, !noalias !11594
  %_5.not.i.i.i = icmp eq i64 %_31.1.i, 0
  %41 = getelementptr inbounds nuw i8, ptr %_31.0.i, i64 4
  %42 = getelementptr inbounds nuw i8, ptr %_31.0.i, i64 8
  br i1 %_5.not.i.i.i, label %bb2.i3092, label %bb1.i.i3082

bb1.i.i3082:                                      ; preds = %bb26, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i
  %_224.i.i = phi ptr [ %_22.i.i3085, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i ], [ %_31.0.i, %bb26 ]
  %_12.i.i3083 = icmp eq ptr %_224.i.i, %_17.i, !dbg !11610
  br i1 %_12.i.i3083, label %bb2.i3092, label %bb13.i.i3084, !dbg !11614

bb13.i.i3084:                                     ; preds = %bb1.i.i3082
  %_22.i.i3085 = getelementptr inbounds nuw i8, ptr %_224.i.i, i64 12, !dbg !11615
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11617), !dbg !11620, !noalias !11594
  %_9.i.i.i3086 = load i32, ptr %_224.i.i, align 4, !dbg !11621, !alias.scope !11617, !noalias !11624, !noundef !12
  %_10.i.i.i = load i32, ptr %_31.0.i, align 4, !dbg !11621, !alias.scope !11606, !noalias !11626, !noundef !12
  %_8.i.i.i = icmp eq i32 %_9.i.i.i3086, %_10.i.i.i, !dbg !11621
  br i1 %_8.i.i.i, label %bb2.i.i.i, label %bb7.i, !dbg !11621

bb2.i.i.i:                                        ; preds = %bb13.i.i3084
  %43 = getelementptr inbounds nuw i8, ptr %_224.i.i, i64 4, !dbg !11621
  %_12.i.i.i3088 = load i32, ptr %43, align 4, !dbg !11621, !alias.scope !11617, !noalias !11624, !noundef !12
  %_13.i.i.i3089 = load i32, ptr %41, align 4, !dbg !11621, !alias.scope !11606, !noalias !11626, !noundef !12
  %_11.i.i.i = icmp eq i32 %_12.i.i.i3088, %_13.i.i.i3089, !dbg !11621
  br i1 %_11.i.i.i, label %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, label %bb7.i, !dbg !11621

_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i: ; preds = %bb2.i.i.i
  %44 = getelementptr inbounds nuw i8, ptr %_224.i.i, i64 8, !dbg !11621
  %_14.i.i.i3090 = load i32, ptr %44, align 4, !dbg !11621, !alias.scope !11617, !noalias !11624, !noundef !12
  %_15.i.i.i3091 = load i32, ptr %42, align 4, !dbg !11621, !alias.scope !11606, !noalias !11626, !noundef !12
  %45 = icmp eq i32 %_14.i.i.i3090, %_15.i.i.i3091, !dbg !11621
  br i1 %45, label %bb1.i.i3082, label %bb7.i, !dbg !11620

bb2.i3092:                                        ; preds = %bb1.i.i3082, %bb26
  %46 = getelementptr inbounds nuw i8, ptr %self, i64 1760, !dbg !11627
  %_32.0.i = load ptr, ptr %46, align 8, !dbg !11627, !alias.scope !11586, !noalias !11594, !nonnull !12, !noundef !12
  %47 = getelementptr inbounds nuw i8, ptr %self, i64 1768, !dbg !11627
  %_32.1.i = load i64, ptr %47, align 8, !dbg !11627, !alias.scope !11586, !noalias !11594, !noundef !12
  %_26.idx.i = shl nuw nsw i64 %_32.1.i, 2, !dbg !11628
  %_26.i3093 = getelementptr inbounds nuw i8, ptr %_32.0.i, i64 %_26.idx.i, !dbg !11628
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11632), !dbg !11635, !noalias !11594
  %_6.not.i.i.i = icmp eq i64 %_32.1.i, 0
  br i1 %_6.not.i.i.i, label %bb2.i, label %bb1.i3.i

bb1.i3.i:                                         ; preds = %bb2.i3092, %bb13.i5.i
  %_223.i.i = phi ptr [ %_22.i6.i, %bb13.i5.i ], [ %_32.0.i, %bb2.i3092 ]
  %_12.i4.i = icmp eq ptr %_223.i.i, %_26.i3093, !dbg !11636
  br i1 %_12.i4.i, label %bb2.i, label %bb13.i5.i, !dbg !11640

bb13.i5.i:                                        ; preds = %bb1.i3.i
  %_22.i6.i = getelementptr inbounds nuw i8, ptr %_223.i.i, i64 4, !dbg !11641
  %ptr.val.i.i = load i32, ptr %_223.i.i, align 4, !dbg !11643, !noalias !11644
  %_4.i.i.i3094 = load i32, ptr %_32.0.i, align 4, !dbg !11646, !alias.scope !11632, !noalias !11648, !noundef !12
  %_0.i.i.i = icmp eq i32 %ptr.val.i.i, %_4.i.i.i3094, !dbg !11649
  br i1 %_0.i.i.i, label %bb1.i3.i, label %bb7.i, !dbg !11643

bb2.i:                                            ; preds = %bb1.i3.i, %bb2.i3092
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11650), !dbg !11653
  %48 = getelementptr inbounds nuw i8, ptr %self, i64 2024, !dbg !11654
  %_31.0.i3095 = load ptr, ptr %48, align 8, !dbg !11654, !alias.scope !11650, !noalias !11656, !nonnull !12, !noundef !12
  %49 = getelementptr inbounds nuw i8, ptr %self, i64 2032, !dbg !11654
  %_31.1.i3096 = load i64, ptr %49, align 8, !dbg !11654, !alias.scope !11650, !noalias !11656, !noundef !12
  %_17.idx.i3097 = mul nuw nsw i64 %_31.1.i3096, 12, !dbg !11657
  %_17.i3098 = getelementptr inbounds nuw i8, ptr %_31.0.i3095, i64 %_17.idx.i3097, !dbg !11657
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11661), !dbg !11664, !noalias !11656
  %_5.not.i.i.i3099 = icmp eq i64 %_31.1.i3096, 0
  %50 = getelementptr inbounds nuw i8, ptr %_31.0.i3095, i64 4
  %51 = getelementptr inbounds nuw i8, ptr %_31.0.i3095, i64 8
  br i1 %_5.not.i.i.i3099, label %bb2.i3117, label %bb1.i.i3100

bb1.i.i3100:                                      ; preds = %bb2.i, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3114
  %_224.i.i3101 = phi ptr [ %_22.i.i3104, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3114 ], [ %_31.0.i3095, %bb2.i ]
  %_12.i.i3102 = icmp eq ptr %_224.i.i3101, %_17.i3098, !dbg !11665
  br i1 %_12.i.i3102, label %bb2.i3117, label %bb13.i.i3103, !dbg !11669

bb13.i.i3103:                                     ; preds = %bb1.i.i3100
  %_22.i.i3104 = getelementptr inbounds nuw i8, ptr %_224.i.i3101, i64 12, !dbg !11670
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11672), !dbg !11675, !noalias !11656
  %_9.i.i.i3105 = load i32, ptr %_224.i.i3101, align 4, !dbg !11676, !alias.scope !11672, !noalias !11679, !noundef !12
  %_10.i.i.i3106 = load i32, ptr %_31.0.i3095, align 4, !dbg !11676, !alias.scope !11661, !noalias !11681, !noundef !12
  %_8.i.i.i3107 = icmp eq i32 %_9.i.i.i3105, %_10.i.i.i3106, !dbg !11676
  br i1 %_8.i.i.i3107, label %bb2.i.i.i3110, label %bb7.i, !dbg !11676

bb2.i.i.i3110:                                    ; preds = %bb13.i.i3103
  %52 = getelementptr inbounds nuw i8, ptr %_224.i.i3101, i64 4, !dbg !11676
  %_12.i.i.i3111 = load i32, ptr %52, align 4, !dbg !11676, !alias.scope !11672, !noalias !11679, !noundef !12
  %_13.i.i.i3112 = load i32, ptr %50, align 4, !dbg !11676, !alias.scope !11661, !noalias !11681, !noundef !12
  %_11.i.i.i3113 = icmp eq i32 %_12.i.i.i3111, %_13.i.i.i3112, !dbg !11676
  br i1 %_11.i.i.i3113, label %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3114, label %bb7.i, !dbg !11676

_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3114: ; preds = %bb2.i.i.i3110
  %53 = getelementptr inbounds nuw i8, ptr %_224.i.i3101, i64 8, !dbg !11676
  %_14.i.i.i3115 = load i32, ptr %53, align 4, !dbg !11676, !alias.scope !11672, !noalias !11679, !noundef !12
  %_15.i.i.i3116 = load i32, ptr %51, align 4, !dbg !11676, !alias.scope !11661, !noalias !11681, !noundef !12
  %54 = icmp eq i32 %_14.i.i.i3115, %_15.i.i.i3116, !dbg !11676
  br i1 %54, label %bb1.i.i3100, label %bb7.i, !dbg !11675

bb2.i3117:                                        ; preds = %bb1.i.i3100, %bb2.i
  %55 = getelementptr inbounds nuw i8, ptr %self, i64 1960, !dbg !11682
  %_32.0.i3118 = load ptr, ptr %55, align 8, !dbg !11682, !alias.scope !11650, !noalias !11656, !nonnull !12, !noundef !12
  %56 = getelementptr inbounds nuw i8, ptr %self, i64 1968, !dbg !11682
  %_32.1.i3119 = load i64, ptr %56, align 8, !dbg !11682, !alias.scope !11650, !noalias !11656, !noundef !12
  %_26.idx.i3120 = shl nuw nsw i64 %_32.1.i3119, 2, !dbg !11683
  %_26.i3121 = getelementptr inbounds nuw i8, ptr %_32.0.i3118, i64 %_26.idx.i3120, !dbg !11683
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11687), !dbg !11690, !noalias !11656
  %_6.not.i.i.i3122 = icmp eq i64 %_32.1.i3119, 0
  br i1 %_6.not.i.i.i3122, label %bb4.i, label %bb1.i3.i3123

bb1.i3.i3123:                                     ; preds = %bb2.i3117, %bb13.i5.i3126
  %_223.i.i3124 = phi ptr [ %_22.i6.i3127, %bb13.i5.i3126 ], [ %_32.0.i3118, %bb2.i3117 ]
  %_12.i4.i3125 = icmp eq ptr %_223.i.i3124, %_26.i3121, !dbg !11691
  br i1 %_12.i4.i3125, label %bb4.i, label %bb13.i5.i3126, !dbg !11695

bb13.i5.i3126:                                    ; preds = %bb1.i3.i3123
  %_22.i6.i3127 = getelementptr inbounds nuw i8, ptr %_223.i.i3124, i64 4, !dbg !11696
  %ptr.val.i.i3128 = load i32, ptr %_223.i.i3124, align 4, !dbg !11698, !noalias !11699
  %_4.i.i.i3129 = load i32, ptr %_32.0.i3118, align 4, !dbg !11701, !alias.scope !11687, !noalias !11703, !noundef !12
  %_0.i.i.i3130 = icmp eq i32 %ptr.val.i.i3128, %_4.i.i.i3129, !dbg !11704
  br i1 %_0.i.i.i3130, label %bb1.i3.i3123, label %bb7.i, !dbg !11698

bb7.i:                                            ; preds = %bb13.i.i3084, %bb2.i.i.i, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, %bb13.i5.i, %bb13.i.i3103, %bb2.i.i.i3110, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3114, %bb13.i5.i3126
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11705), !dbg !11708
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11709), !dbg !11708
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11711), !dbg !11708
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11713), !dbg !11708
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11715), !dbg !11708
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_left.i224, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #30, !dbg !11717
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_right.i223, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #30, !dbg !11721
  %57 = getelementptr inbounds nuw i8, ptr %self, i64 1776, !dbg !11723
  %_162.0.i = load ptr, ptr %57, align 8, !dbg !11723, !alias.scope !11711, !noalias !11725, !nonnull !12, !noundef !12
  %58 = getelementptr inbounds nuw i8, ptr %self, i64 1784, !dbg !11723
  %_162.1.i = load i64, ptr %58, align 8, !dbg !11723, !alias.scope !11711, !noalias !11725, !noundef !12
  %_8.i3132 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_162.0.i, i64 %_162.1.i, !dbg !11728
  br label %bb1.i.i3133, !dbg !11733

bb1.i.i3133:                                      ; preds = %bb13.i.i3136, %bb7.i
  %_221.i.i3134 = phi ptr [ %_22.i.i3137, %bb13.i.i3136 ], [ %_162.0.i, %bb7.i ]
  %_12.i.i3135 = icmp eq ptr %_221.i.i3134, %_8.i3132, !dbg !11735
  br i1 %_12.i.i3135, label %bb4.i292, label %bb13.i.i3136, !dbg !11738

bb13.i.i3136:                                     ; preds = %bb1.i.i3133
  %_22.i.i3137 = getelementptr inbounds nuw i8, ptr %_221.i.i3134, i64 16, !dbg !11739
  %59 = getelementptr inbounds nuw i8, ptr %_221.i.i3134, i64 12, !dbg !11741
  %_3.i.i.i3138 = load i32, ptr %59, align 4, !dbg !11741, !alias.scope !11743, !noalias !11748, !noundef !12
  %60 = icmp eq i32 %_3.i.i.i3138, 0, !dbg !11741
  %_51.i.i.i3139 = load i32, ptr %_221.i.i3134, align 4, !dbg !11741, !alias.scope !11743, !noalias !11748
  %61 = getelementptr inbounds nuw i8, ptr %_221.i.i3134, i64 4, !dbg !11741
  %_72.i.i.i3140 = load i32, ptr %61, align 4, !dbg !11741, !alias.scope !11743, !noalias !11748
  %62 = icmp eq i32 %_51.i.i.i3139, %_72.i.i.i3140, !dbg !11741
  %_0.sroa.0.0.i.i.i3141 = select i1 %60, i1 %62, i1 false, !dbg !11741
  br i1 %_0.sroa.0.0.i.i.i3141, label %bb1.i.i3133, label %bb14.i226, !dbg !11751

bb4.i292:                                         ; preds = %bb1.i.i3133
  %63 = getelementptr inbounds nuw i8, ptr %self, i64 1792, !dbg !11752
  %_163.0.i = load ptr, ptr %63, align 8, !dbg !11752, !alias.scope !11711, !noalias !11725, !nonnull !12, !noundef !12
  %64 = getelementptr inbounds nuw i8, ptr %self, i64 1800, !dbg !11752
  %_163.1.i = load i64, ptr %64, align 8, !dbg !11752, !alias.scope !11711, !noalias !11725, !noundef !12
  %_8.i3143 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_163.0.i, i64 %_163.1.i, !dbg !11753
  br label %bb1.i.i3144, !dbg !11758

bb1.i.i3144:                                      ; preds = %bb13.i.i3147, %bb4.i292
  %_221.i.i3145 = phi ptr [ %_22.i.i3148, %bb13.i.i3147 ], [ %_163.0.i, %bb4.i292 ]
  %_12.i.i3146 = icmp eq ptr %_221.i.i3145, %_8.i3143, !dbg !11760
  br i1 %_12.i.i3146, label %bb6.i294, label %bb13.i.i3147, !dbg !11763

bb13.i.i3147:                                     ; preds = %bb1.i.i3144
  %_22.i.i3148 = getelementptr inbounds nuw i8, ptr %_221.i.i3145, i64 16, !dbg !11764
  %65 = getelementptr inbounds nuw i8, ptr %_221.i.i3145, i64 12, !dbg !11766
  %_3.i.i.i3149 = load i32, ptr %65, align 4, !dbg !11766, !alias.scope !11768, !noalias !11773, !noundef !12
  %66 = icmp eq i32 %_3.i.i.i3149, 0, !dbg !11766
  %_51.i.i.i3150 = load i32, ptr %_221.i.i3145, align 4, !dbg !11766, !alias.scope !11768, !noalias !11773
  %67 = getelementptr inbounds nuw i8, ptr %_221.i.i3145, i64 4, !dbg !11766
  %_72.i.i.i3151 = load i32, ptr %67, align 4, !dbg !11766, !alias.scope !11768, !noalias !11773
  %68 = icmp eq i32 %_51.i.i.i3150, %_72.i.i.i3151, !dbg !11766
  %_0.sroa.0.0.i.i.i3152 = select i1 %66, i1 %68, i1 false, !dbg !11766
  br i1 %_0.sroa.0.0.i.i.i3152, label %bb1.i.i3144, label %bb14.i226, !dbg !11776

bb6.i294:                                         ; preds = %bb1.i.i3144
  %69 = getelementptr inbounds nuw i8, ptr %self, i64 1976, !dbg !11777
  %_164.0.i = load ptr, ptr %69, align 8, !dbg !11777, !alias.scope !11713, !noalias !11778, !nonnull !12, !noundef !12
  %70 = getelementptr inbounds nuw i8, ptr %self, i64 1984, !dbg !11777
  %_164.1.i = load i64, ptr %70, align 8, !dbg !11777, !alias.scope !11713, !noalias !11778, !noundef !12
  %_8.i3154 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_164.0.i, i64 %_164.1.i, !dbg !11779
  br label %bb1.i.i3155, !dbg !11784

bb1.i.i3155:                                      ; preds = %bb13.i.i3158, %bb6.i294
  %_221.i.i3156 = phi ptr [ %_22.i.i3159, %bb13.i.i3158 ], [ %_164.0.i, %bb6.i294 ]
  %_12.i.i3157 = icmp eq ptr %_221.i.i3156, %_8.i3154, !dbg !11786
  br i1 %_12.i.i3157, label %bb8.i296, label %bb13.i.i3158, !dbg !11789

bb13.i.i3158:                                     ; preds = %bb1.i.i3155
  %_22.i.i3159 = getelementptr inbounds nuw i8, ptr %_221.i.i3156, i64 16, !dbg !11790
  %71 = getelementptr inbounds nuw i8, ptr %_221.i.i3156, i64 12, !dbg !11792
  %_3.i.i.i3160 = load i32, ptr %71, align 4, !dbg !11792, !alias.scope !11794, !noalias !11799, !noundef !12
  %72 = icmp eq i32 %_3.i.i.i3160, 0, !dbg !11792
  %_51.i.i.i3161 = load i32, ptr %_221.i.i3156, align 4, !dbg !11792, !alias.scope !11794, !noalias !11799
  %73 = getelementptr inbounds nuw i8, ptr %_221.i.i3156, i64 4, !dbg !11792
  %_72.i.i.i3162 = load i32, ptr %73, align 4, !dbg !11792, !alias.scope !11794, !noalias !11799
  %74 = icmp eq i32 %_51.i.i.i3161, %_72.i.i.i3162, !dbg !11792
  %_0.sroa.0.0.i.i.i3163 = select i1 %72, i1 %74, i1 false, !dbg !11792
  br i1 %_0.sroa.0.0.i.i.i3163, label %bb1.i.i3155, label %bb14.i226, !dbg !11802

bb8.i296:                                         ; preds = %bb1.i.i3155
  %75 = getelementptr inbounds nuw i8, ptr %self, i64 1992, !dbg !11803
  %_165.0.i = load ptr, ptr %75, align 8, !dbg !11803, !alias.scope !11713, !noalias !11778, !nonnull !12, !noundef !12
  %76 = getelementptr inbounds nuw i8, ptr %self, i64 2000, !dbg !11803
  %_165.1.i = load i64, ptr %76, align 8, !dbg !11803, !alias.scope !11713, !noalias !11778, !noundef !12
  %_8.i3165 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_165.0.i, i64 %_165.1.i, !dbg !11804
  br label %bb1.i.i3166, !dbg !11809

bb1.i.i3166:                                      ; preds = %bb13.i.i3169, %bb8.i296
  %_221.i.i3167 = phi ptr [ %_22.i.i3170, %bb13.i.i3169 ], [ %_165.0.i, %bb8.i296 ]
  %_12.i.i3168 = icmp eq ptr %_221.i.i3167, %_8.i3165, !dbg !11811
  br i1 %_12.i.i3168, label %bb14.i226, label %bb13.i.i3169, !dbg !11814

bb13.i.i3169:                                     ; preds = %bb1.i.i3166
  %_22.i.i3170 = getelementptr inbounds nuw i8, ptr %_221.i.i3167, i64 16, !dbg !11815
  %77 = getelementptr inbounds nuw i8, ptr %_221.i.i3167, i64 12, !dbg !11817
  %_3.i.i.i3171 = load i32, ptr %77, align 4, !dbg !11817, !alias.scope !11819, !noalias !11824, !noundef !12
  %78 = icmp eq i32 %_3.i.i.i3171, 0, !dbg !11817
  %_51.i.i.i3172 = load i32, ptr %_221.i.i3167, align 4, !dbg !11817, !alias.scope !11819, !noalias !11824
  %79 = getelementptr inbounds nuw i8, ptr %_221.i.i3167, i64 4, !dbg !11817
  %_72.i.i.i3173 = load i32, ptr %79, align 4, !dbg !11817, !alias.scope !11819, !noalias !11824
  %80 = icmp eq i32 %_51.i.i.i3172, %_72.i.i.i3173, !dbg !11817
  %_0.sroa.0.0.i.i.i3174 = select i1 %78, i1 %80, i1 false, !dbg !11817
  br i1 %_0.sroa.0.0.i.i.i3174, label %bb1.i.i3166, label %bb14.i226, !dbg !11827

bb14.i226:                                        ; preds = %bb13.i.i3136, %bb13.i.i3147, %bb13.i.i3158, %bb13.i.i3169, %bb1.i.i3166
  %stationary.sroa.0.0.i227 = phi i1 [ false, %bb13.i.i3158 ], [ false, %bb13.i.i3147 ], [ false, %bb13.i.i3169 ], [ true, %bb1.i.i3166 ], [ false, %bb13.i.i3136 ], !dbg !11828
  %81 = getelementptr inbounds nuw i8, ptr %self, i64 1536, !dbg !11829
  %82 = load i8, ptr %81, align 32, !dbg !11829, !range !5399, !alias.scope !11705, !noalias !11833, !noundef !12
  %83 = getelementptr inbounds nuw i8, ptr %self, i64 1537, !dbg !11834
  %84 = load i8, ptr %83, align 1, !dbg !11834, !range !5399, !alias.scope !11705, !noalias !11833, !noundef !12
  %_41.i = load i32, ptr %_35, align 4, !dbg !11836, !alias.scope !11715, !noalias !11838, !noundef !12
  %85 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !11839
  %_43.i234 = load i32, ptr %85, align 4, !dbg !11839, !alias.scope !11715, !noalias !11838, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i), !dbg !11841, !noalias !11843
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i, i8 0, i64 32, i1 false), !noalias !11843
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i217), !dbg !11844, !noalias !11843
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i217, i8 0, i64 1024, i1 false), !noalias !11843
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i216), !dbg !11846, !noalias !11843
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i216, i8 0, i64 1024, i1 false), !noalias !11843
  %86 = add nuw nsw i64 %frames, 31, !dbg !11848
  %yield_count.sroa.0.0.i.i = lshr i64 %86, 5, !dbg !11848
  %_121.not.i7391 = icmp eq i64 %yield_count.sroa.0.0.i.i, 0, !dbg !11855
  br i1 %_121.not.i7391, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, label %bb54.i.lr.ph, !dbg !11855

bb54.i.lr.ph:                                     ; preds = %bb14.i226
  %87 = zext i32 %_43.i234 to i64, !dbg !11839
  %88 = zext i32 %_41.i to i64, !dbg !11836
  %_39.i231 = trunc nuw i8 %84 to i1, !dbg !11834
  %_38.i228 = trunc nuw i8 %82 to i1, !dbg !11829
  %89 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !11864
  %90 = bitcast <8 x float> %89 to <8 x i32>, !dbg !11883
  %91 = xor <8 x i32> %90, splat (i32 -1), !dbg !11898
  %history.i308.i.sroa.10.0.hot_left.i224.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i224, i64 32
  %history.i308.i.sroa.13.0.hot_left.i224.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i224, i64 64
  %history.i308.i.sroa.16.0.hot_left.i224.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i224, i64 96
  %history.i308.i.sroa.19.0.hot_left.i224.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i224, i64 128
  %history.i308.i.sroa.22.0.hot_left.i224.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i224, i64 160
  %history.i308.i.sroa.25.0.hot_left.i224.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i224, i64 192
  %history.i308.i.sroa.29.0.hot_left.i224.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i224, i64 224
  %history.i308.i.sroa.32.0.hot_left.i224.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i224, i64 256
  %history.i308.i.sroa.35.0.hot_left.i224.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i224, i64 288
  %history.i308.i.sroa.38.0.hot_left.i224.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i224, i64 320
  %history.i308.i.sroa.41.0.hot_left.i224.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i224, i64 352
  %92 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %93 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %94 = getelementptr inbounds nuw i8, ptr %self, i64 96
  %row12.i.i.i319.i = getelementptr inbounds nuw i8, ptr %self, i64 128
  %95 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %96 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %97 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %row13.i.i.i320.i = getelementptr inbounds nuw i8, ptr %self, i64 256
  %98 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %99 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %100 = getelementptr inbounds nuw i8, ptr %self, i64 352
  %row14.i.i.i321.i = getelementptr inbounds nuw i8, ptr %self, i64 384
  %101 = getelementptr inbounds nuw i8, ptr %self, i64 416
  %102 = getelementptr inbounds nuw i8, ptr %self, i64 448
  %103 = getelementptr inbounds nuw i8, ptr %self, i64 480
  %row15.i.i.i322.i = getelementptr inbounds nuw i8, ptr %self, i64 512
  %104 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %105 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %106 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %row16.i.i.i323.i = getelementptr inbounds nuw i8, ptr %self, i64 640
  %107 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %108 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %109 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %row17.i.i.i324.i = getelementptr inbounds nuw i8, ptr %self, i64 768
  %110 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %111 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %112 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %row18.i.i.i325.i = getelementptr inbounds nuw i8, ptr %self, i64 896
  %113 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %114 = getelementptr inbounds nuw i8, ptr %self, i64 960
  %115 = getelementptr inbounds nuw i8, ptr %self, i64 992
  %row19.i.i.i326.i = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %116 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %117 = getelementptr inbounds nuw i8, ptr %self, i64 1088
  %118 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %row20.i.i.i327.i = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %119 = getelementptr inbounds nuw i8, ptr %self, i64 1184
  %120 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %121 = getelementptr inbounds nuw i8, ptr %self, i64 1248
  %row21.i.i.i328.i = getelementptr inbounds nuw i8, ptr %self, i64 1280
  %122 = getelementptr inbounds nuw i8, ptr %self, i64 1312
  %123 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %124 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %row22.i.i.i329.i = getelementptr inbounds nuw i8, ptr %self, i64 1408
  %125 = getelementptr inbounds nuw i8, ptr %self, i64 1440
  %126 = getelementptr inbounds nuw i8, ptr %self, i64 1472
  %127 = getelementptr inbounds nuw i8, ptr %self, i64 1504
  %history.i.i190.sroa.10.0.hot_right.i223.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i223, i64 32
  %history.i.i190.sroa.13.0.hot_right.i223.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i223, i64 64
  %history.i.i190.sroa.16.0.hot_right.i223.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i223, i64 96
  %history.i.i190.sroa.19.0.hot_right.i223.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i223, i64 128
  %history.i.i190.sroa.22.0.hot_right.i223.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i223, i64 160
  %history.i.i190.sroa.25.0.hot_right.i223.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i223, i64 192
  %history.i.i190.sroa.29.0.hot_right.i223.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i223, i64 224
  %history.i.i190.sroa.32.0.hot_right.i223.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i223, i64 256
  %history.i.i190.sroa.35.0.hot_right.i223.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i223, i64 288
  %history.i.i190.sroa.38.0.hot_right.i223.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i223, i64 320
  %history.i.i190.sroa.41.0.hot_right.i223.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i223, i64 352
  %_69.i = getelementptr inbounds nuw i8, ptr %hot_left.i224, i64 384
  %128 = getelementptr inbounds nuw i8, ptr %hot_left.i224, i64 480
  %129 = getelementptr inbounds nuw i8, ptr %hot_left.i224, i64 448
  %130 = getelementptr inbounds nuw i8, ptr %hot_left.i224, i64 416
  %_71.i = getelementptr inbounds nuw i8, ptr %hot_left.i224, i64 512
  %131 = getelementptr inbounds nuw i8, ptr %hot_left.i224, i64 608
  %132 = getelementptr inbounds nuw i8, ptr %hot_left.i224, i64 576
  %133 = getelementptr inbounds nuw i8, ptr %hot_left.i224, i64 544
  %_73.i251 = getelementptr inbounds nuw i8, ptr %hot_right.i223, i64 384
  %134 = getelementptr inbounds nuw i8, ptr %hot_right.i223, i64 480
  %135 = getelementptr inbounds nuw i8, ptr %hot_right.i223, i64 448
  %136 = getelementptr inbounds nuw i8, ptr %hot_right.i223, i64 416
  %_75.i = getelementptr inbounds nuw i8, ptr %hot_right.i223, i64 512
  %137 = getelementptr inbounds nuw i8, ptr %hot_right.i223, i64 608
  %138 = getelementptr inbounds nuw i8, ptr %hot_right.i223, i64 576
  %139 = getelementptr inbounds nuw i8, ptr %hot_right.i223, i64 544
  %140 = select i1 %_38.i228, <8 x i32> %90, <8 x i32> %91
  %141 = icmp slt <8 x i32> %140, zeroinitializer
  %142 = getelementptr inbounds nuw i8, ptr %self, i64 1624
  %143 = getelementptr inbounds nuw i8, ptr %self, i64 1840
  %144 = getelementptr inbounds nuw i8, ptr %self, i64 1688
  %145 = getelementptr inbounds nuw i8, ptr %self, i64 1680
  %146 = getelementptr inbounds nuw i8, ptr %self, i64 1768
  %147 = getelementptr inbounds nuw i8, ptr %self, i64 1760
  %148 = getelementptr inbounds nuw i8, ptr %self, i64 1736
  %149 = getelementptr inbounds nuw i8, ptr %self, i64 1728
  %150 = getelementptr inbounds nuw i8, ptr %self, i64 1704
  %151 = getelementptr inbounds nuw i8, ptr %self, i64 1696
  %152 = getelementptr inbounds nuw i8, ptr %hot_left.i224, i64 672
  %153 = getelementptr inbounds nuw i8, ptr %hot_left.i224, i64 704
  %154 = getelementptr inbounds nuw i8, ptr %hot_left.i224, i64 640
  %155 = getelementptr inbounds nuw i8, ptr %self, i64 1672
  %156 = getelementptr inbounds nuw i8, ptr %self, i64 1664
  %157 = select i1 %_39.i231, <8 x i32> %90, <8 x i32> %91
  %158 = icmp slt <8 x i32> %157, zeroinitializer
  %159 = getelementptr inbounds nuw i8, ptr %self, i64 2040
  %160 = getelementptr inbounds nuw i8, ptr %self, i64 1888
  %161 = getelementptr inbounds nuw i8, ptr %self, i64 1880
  %162 = getelementptr inbounds nuw i8, ptr %self, i64 2032
  %163 = getelementptr inbounds nuw i8, ptr %self, i64 2024
  %164 = getelementptr inbounds nuw i8, ptr %self, i64 1968
  %165 = getelementptr inbounds nuw i8, ptr %self, i64 1960
  %166 = getelementptr inbounds nuw i8, ptr %self, i64 1936
  %167 = getelementptr inbounds nuw i8, ptr %self, i64 1928
  %168 = getelementptr inbounds nuw i8, ptr %self, i64 1904
  %169 = getelementptr inbounds nuw i8, ptr %self, i64 1896
  %170 = getelementptr inbounds nuw i8, ptr %hot_right.i223, i64 672
  %171 = getelementptr inbounds nuw i8, ptr %hot_right.i223, i64 704
  %172 = getelementptr inbounds nuw i8, ptr %hot_right.i223, i64 640
  %173 = getelementptr inbounds nuw i8, ptr %self, i64 1872
  %174 = getelementptr inbounds nuw i8, ptr %self, i64 1864
  %175 = getelementptr inbounds nuw i8, ptr %self, i64 1632
  %iter.i47.i.sroa.0.0.ptr7373.1 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 4
  %iter.i47.i.sroa.0.0.ptr7373.2 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 8
  %iter.i47.i.sroa.0.0.ptr7373.3 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 12
  %iter.i47.i.sroa.0.0.ptr7373.4 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 16
  %iter.i47.i.sroa.0.0.ptr7373.5 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 20
  %iter.i47.i.sroa.0.0.ptr7373.6 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 24
  %iter.i47.i.sroa.0.0.ptr7373.7 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 28
  %iter.i.i.sroa.0.0.ptr7384.1 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 4
  %iter.i.i.sroa.0.0.ptr7384.2 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 8
  %iter.i.i.sroa.0.0.ptr7384.3 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 12
  %iter.i.i.sroa.0.0.ptr7384.4 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 16
  %iter.i.i.sroa.0.0.ptr7384.5 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 20
  %iter.i.i.sroa.0.0.ptr7384.6 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 24
  %iter.i.i.sroa.0.0.ptr7384.7 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 28
  br label %bb54.i, !dbg !11855

bb25.i.loopexit.loopexit:                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2798
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
  br label %bb25.i.loopexit, !dbg !11855

bb25.i.loopexit:                                  ; preds = %bb25.i.loopexit.loopexit, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i246
  %ring_cursor.sroa.0.1.i248.lcssa = phi i64 [ %ring_cursor.sroa.0.0.i2407394, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i246 ], [ %spec.store.select13.i, %bb25.i.loopexit.loopexit ], !dbg !11914
  %main_cursor.sroa.0.1.i249.lcssa = phi i64 [ %main_cursor.sroa.0.0.i2417395, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i246 ], [ %spec.store.select.i, %bb25.i.loopexit.loopexit ], !dbg !11915
  %_121.not.i = icmp eq i64 %178, 0, !dbg !11855
  %indvars.iv.next = add nsw i64 %indvars.iv, -32, !dbg !11855
  br i1 %_121.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit, label %bb54.i, !dbg !11855

bb54.i:                                           ; preds = %bb54.i.lr.ph, %bb25.i.loopexit
  %indvars.iv = phi i64 [ %frames, %bb54.i.lr.ph ], [ %indvars.iv.next, %bb25.i.loopexit ]
  %main_cursor.sroa.0.0.i2417395 = phi i64 [ %88, %bb54.i.lr.ph ], [ %main_cursor.sroa.0.1.i249.lcssa, %bb25.i.loopexit ]
  %ring_cursor.sroa.0.0.i2407394 = phi i64 [ %87, %bb54.i.lr.ph ], [ %ring_cursor.sroa.0.1.i248.lcssa, %bb25.i.loopexit ]
  %iter4.sroa.0.0.i2397393 = phi i64 [ %yield_count.sroa.0.0.i.i, %bb54.i.lr.ph ], [ %178, %bb25.i.loopexit ]
  %iter.sroa.0.0.i7392 = phi i64 [ 0, %bb54.i.lr.ph ], [ %177, %bb25.i.loopexit ]
  %176 = call i64 @llvm.umax.i64(i64 %indvars.iv, i64 1), !dbg !11916
  %umax8389 = call i64 @llvm.umin.i64(i64 %176, i64 32), !dbg !11916
  %177 = add nuw nsw i64 %iter.sroa.0.0.i7392, 32, !dbg !11916
  %178 = add nsw i64 %iter4.sroa.0.0.i2397393, -1, !dbg !11920
  %history.i308.i.sroa.0.0.copyload = load <8 x float>, ptr %hot_left.i224, align 32, !dbg !11921
  %history.i308.i.sroa.10.sroa.0.0.copyload = load <8 x float>, ptr %history.i308.i.sroa.10.0.hot_left.i224.sroa_idx, align 32, !dbg !11921
  %history.i308.i.sroa.13.sroa.0.0.copyload = load <8 x float>, ptr %history.i308.i.sroa.13.0.hot_left.i224.sroa_idx, align 32, !dbg !11921
  %history.i308.i.sroa.16.sroa.0.0.copyload = load <8 x float>, ptr %history.i308.i.sroa.16.0.hot_left.i224.sroa_idx, align 32, !dbg !11921
  %history.i308.i.sroa.19.sroa.0.0.copyload = load <8 x float>, ptr %history.i308.i.sroa.19.0.hot_left.i224.sroa_idx, align 32, !dbg !11921
  %history.i308.i.sroa.22.sroa.0.0.copyload = load <8 x float>, ptr %history.i308.i.sroa.22.0.hot_left.i224.sroa_idx, align 32, !dbg !11921
  %history.i308.i.sroa.25.sroa.0.0.copyload = load <8 x float>, ptr %history.i308.i.sroa.25.0.hot_left.i224.sroa_idx, align 32, !dbg !11921
  %history.i308.i.sroa.29.sroa.0.0.copyload = load <8 x float>, ptr %history.i308.i.sroa.29.0.hot_left.i224.sroa_idx, align 32, !dbg !11921
  %history.i308.i.sroa.32.sroa.0.0.copyload = load <8 x float>, ptr %history.i308.i.sroa.32.0.hot_left.i224.sroa_idx, align 32, !dbg !11921
  %history.i308.i.sroa.35.sroa.0.0.copyload = load <8 x float>, ptr %history.i308.i.sroa.35.0.hot_left.i224.sroa_idx, align 32, !dbg !11921
  %history.i308.i.sroa.38.sroa.0.0.copyload = load <8 x float>, ptr %history.i308.i.sroa.38.0.hot_left.i224.sroa_idx, align 32, !dbg !11921
  %history.i308.i.sroa.41.sroa.0.0.copyload = load <8 x float>, ptr %history.i308.i.sroa.41.0.hot_left.i224.sroa_idx, align 32, !dbg !11921
  %_20.i311.i7312.not = icmp eq i64 %frames, %iter.sroa.0.0.i7392, !dbg !11925
  br i1 %_20.i311.i7312.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit343.i, label %bb5.i312.i.lr.ph, !dbg !11935

bb5.i312.i.lr.ph:                                 ; preds = %bb54.i
  %_5.i1738 = load <8 x float>, ptr %self, align 32
  %_14.i.i.i273.i.sroa.0.0.copyload = load <8 x float>, ptr %92, align 32
  %_17.i.i.i270.i.sroa.0.0.copyload = load <8 x float>, ptr %93, align 32
  %_20.i.i.i267.i.sroa.0.0.copyload = load <8 x float>, ptr %94, align 32
  %_25.i.i.i263.i.sroa.0.0.copyload = load <8 x float>, ptr %row12.i.i.i319.i, align 32
  %_28.i.i.i260.i.sroa.0.0.copyload = load <8 x float>, ptr %95, align 32
  %_31.i.i.i257.i.sroa.0.0.copyload = load <8 x float>, ptr %96, align 32
  %_34.i.i.i254.i.sroa.0.0.copyload = load <8 x float>, ptr %97, align 32
  %_39.i.i.i250.i.sroa.0.0.copyload = load <8 x float>, ptr %row13.i.i.i320.i, align 32
  %_42.i.i.i247.i.sroa.0.0.copyload = load <8 x float>, ptr %98, align 32
  %_45.i.i.i244.i.sroa.0.0.copyload = load <8 x float>, ptr %99, align 32
  %_48.i.i.i241.i.sroa.0.0.copyload = load <8 x float>, ptr %100, align 32
  %_53.i.i.i237.i.sroa.0.0.copyload = load <8 x float>, ptr %row14.i.i.i321.i, align 32
  %_56.i.i.i234.i.sroa.0.0.copyload = load <8 x float>, ptr %101, align 32
  %_59.i.i.i231.i.sroa.0.0.copyload = load <8 x float>, ptr %102, align 32
  %_62.i.i.i228.i.sroa.0.0.copyload = load <8 x float>, ptr %103, align 32
  %_67.i.i.i224.i.sroa.0.0.copyload = load <8 x float>, ptr %row15.i.i.i322.i, align 32
  %_70.i.i.i221.i.sroa.0.0.copyload = load <8 x float>, ptr %104, align 32
  %_73.i.i.i218.i.sroa.0.0.copyload = load <8 x float>, ptr %105, align 32
  %_76.i.i.i215.i.sroa.0.0.copyload = load <8 x float>, ptr %106, align 32
  %_81.i.i.i211.i.sroa.0.0.copyload = load <8 x float>, ptr %row16.i.i.i323.i, align 32
  %_84.i.i.i208.i.sroa.0.0.copyload = load <8 x float>, ptr %107, align 32
  %_87.i.i.i205.i.sroa.0.0.copyload = load <8 x float>, ptr %108, align 32
  %_90.i.i.i202.i.sroa.0.0.copyload = load <8 x float>, ptr %109, align 32
  %_95.i.i.i198.i.sroa.0.0.copyload = load <8 x float>, ptr %row17.i.i.i324.i, align 32
  %_98.i.i.i195.i.sroa.0.0.copyload = load <8 x float>, ptr %110, align 32
  %_101.i.i.i192.i.sroa.0.0.copyload = load <8 x float>, ptr %111, align 32
  %_104.i.i.i189.i.sroa.0.0.copyload = load <8 x float>, ptr %112, align 32
  %_109.i.i.i185.i.sroa.0.0.copyload = load <8 x float>, ptr %row18.i.i.i325.i, align 32
  %_112.i.i.i182.i.sroa.0.0.copyload = load <8 x float>, ptr %113, align 32
  %_115.i.i.i179.i.sroa.0.0.copyload = load <8 x float>, ptr %114, align 32
  %_118.i.i.i176.i.sroa.0.0.copyload = load <8 x float>, ptr %115, align 32
  %_123.i.i.i172.i.sroa.0.0.copyload = load <8 x float>, ptr %row19.i.i.i326.i, align 32
  %_126.i.i.i169.i.sroa.0.0.copyload = load <8 x float>, ptr %116, align 32
  %_129.i.i.i166.i.sroa.0.0.copyload = load <8 x float>, ptr %117, align 32
  %_132.i.i.i163.i.sroa.0.0.copyload = load <8 x float>, ptr %118, align 32
  %_137.i.i.i159.i.sroa.0.0.copyload = load <8 x float>, ptr %row20.i.i.i327.i, align 32
  %_140.i.i.i156.i.sroa.0.0.copyload = load <8 x float>, ptr %119, align 32
  %_143.i.i.i153.i.sroa.0.0.copyload = load <8 x float>, ptr %120, align 32
  %_146.i.i.i150.i.sroa.0.0.copyload = load <8 x float>, ptr %121, align 32
  %_151.i.i.i146.i.sroa.0.0.copyload = load <8 x float>, ptr %row21.i.i.i328.i, align 32
  %_154.i.i.i143.i.sroa.0.0.copyload = load <8 x float>, ptr %122, align 32
  %_157.i.i.i140.i.sroa.0.0.copyload = load <8 x float>, ptr %123, align 32
  %_160.i.i.i137.i.sroa.0.0.copyload = load <8 x float>, ptr %124, align 32
  %_165.i.i.i133.i.sroa.0.0.copyload = load <8 x float>, ptr %row22.i.i.i329.i, align 32
  %_168.i.i.i130.i.sroa.0.0.copyload = load <8 x float>, ptr %125, align 32
  %_171.i.i.i127.i.sroa.0.0.copyload = load <8 x float>, ptr %126, align 32
  %_174.i.i.i124.i.sroa.0.0.copyload = load <8 x float>, ptr %127, align 32
  br label %bb5.i312.i, !dbg !11935

bb5.i312.i:                                       ; preds = %bb5.i312.i.lr.ph, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit
  %iter.sroa.0.0.i310.i7324 = phi i64 [ 0, %bb5.i312.i.lr.ph ], [ %179, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %history.i308.i.sroa.10.sroa.0.07323 = phi <8 x float> [ %history.i308.i.sroa.10.sroa.0.0.copyload, %bb5.i312.i.lr.ph ], [ %history.i308.i.sroa.0.07313, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %history.i308.i.sroa.13.sroa.0.07322 = phi <8 x float> [ %history.i308.i.sroa.13.sroa.0.0.copyload, %bb5.i312.i.lr.ph ], [ %history.i308.i.sroa.10.sroa.0.07323, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %history.i308.i.sroa.16.sroa.0.07321 = phi <8 x float> [ %history.i308.i.sroa.16.sroa.0.0.copyload, %bb5.i312.i.lr.ph ], [ %history.i308.i.sroa.13.sroa.0.07322, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %history.i308.i.sroa.19.sroa.0.07320 = phi <8 x float> [ %history.i308.i.sroa.19.sroa.0.0.copyload, %bb5.i312.i.lr.ph ], [ %history.i308.i.sroa.16.sroa.0.07321, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %history.i308.i.sroa.22.sroa.0.07319 = phi <8 x float> [ %history.i308.i.sroa.22.sroa.0.0.copyload, %bb5.i312.i.lr.ph ], [ %history.i308.i.sroa.19.sroa.0.07320, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %history.i308.i.sroa.38.sroa.0.07318 = phi <8 x float> [ %history.i308.i.sroa.38.sroa.0.0.copyload, %bb5.i312.i.lr.ph ], [ %history.i308.i.sroa.35.sroa.0.07317, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %history.i308.i.sroa.35.sroa.0.07317 = phi <8 x float> [ %history.i308.i.sroa.35.sroa.0.0.copyload, %bb5.i312.i.lr.ph ], [ %history.i308.i.sroa.32.sroa.0.07316, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %history.i308.i.sroa.32.sroa.0.07316 = phi <8 x float> [ %history.i308.i.sroa.32.sroa.0.0.copyload, %bb5.i312.i.lr.ph ], [ %history.i308.i.sroa.29.sroa.0.07315, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %history.i308.i.sroa.29.sroa.0.07315 = phi <8 x float> [ %history.i308.i.sroa.29.sroa.0.0.copyload, %bb5.i312.i.lr.ph ], [ %history.i308.i.sroa.25.sroa.0.07314, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %history.i308.i.sroa.25.sroa.0.07314 = phi <8 x float> [ %history.i308.i.sroa.25.sroa.0.0.copyload, %bb5.i312.i.lr.ph ], [ %history.i308.i.sroa.22.sroa.0.07319, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %history.i308.i.sroa.0.07313 = phi <8 x float> [ %history.i308.i.sroa.0.0.copyload, %bb5.i312.i.lr.ph ], [ %lanes.i2364.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %179 = add nuw nsw i64 %iter.sroa.0.0.i310.i7324, 1, !dbg !11936
  %_11.i313.i = add nuw nsw i64 %iter.sroa.0.0.i310.i7324, %iter.sroa.0.0.i7392, !dbg !11942
  %base.i314.i = shl i64 %_11.i313.i, 3, !dbg !11942
  %_24.i315.i = icmp samesign ugt i64 %base.i314.i, %left_io.1, !dbg !11944
  br i1 %_24.i315.i, label %bb7.i342.i, label %bb8.i316.i, !dbg !11944, !prof !905

bb8.i316.i:                                       ; preds = %bb5.i312.i
  %_27.i317.i = sub nuw nsw i64 %left_io.1, %base.i314.i, !dbg !11950
  %_8.i2367 = icmp samesign ugt i64 %_27.i317.i, 7, !dbg !11951
  br i1 %_8.i2367, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit, label %bb2.i2368, !dbg !11951, !prof !1076

bb2.i2368:                                        ; preds = %bb8.i316.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_27.i317.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #31, !dbg !11956, !noalias !11957
  unreachable, !dbg !11956

bb7.i342.i:                                       ; preds = %bb5.i312.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i314.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e0592aef22128a0ac53753b9632a8183) #31, !dbg !11964, !noalias !11965
  unreachable, !dbg !11964

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit: ; preds = %bb8.i316.i
  %_31.i318.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %base.i314.i, !dbg !11966
  %lanes.i2364.sroa.0.0.copyload = load <8 x float>, ptr %_31.i318.i, align 4, !dbg !11971, !alias.scope !11976, !noalias !11980
  %180 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i308.i.sroa.22.sroa.0.07319), !dbg !11982
  %181 = fmul <8 x float> %lanes.i2364.sroa.0.0.copyload, %_5.i1738, !dbg !12000
  %182 = fadd <8 x float> %181, zeroinitializer, !dbg !12019
  %183 = fmul <8 x float> %lanes.i2364.sroa.0.0.copyload, %_14.i.i.i273.i.sroa.0.0.copyload, !dbg !12029
  %184 = fadd <8 x float> %183, zeroinitializer, !dbg !12034
  %185 = fmul <8 x float> %lanes.i2364.sroa.0.0.copyload, %_17.i.i.i270.i.sroa.0.0.copyload, !dbg !12039
  %186 = fadd <8 x float> %185, zeroinitializer, !dbg !12044
  %187 = fmul <8 x float> %lanes.i2364.sroa.0.0.copyload, %_20.i.i.i267.i.sroa.0.0.copyload, !dbg !12049
  %188 = fadd <8 x float> %187, zeroinitializer, !dbg !12054
  %189 = fmul <8 x float> %history.i308.i.sroa.0.07313, %_25.i.i.i263.i.sroa.0.0.copyload, !dbg !12059
  %190 = fadd <8 x float> %182, %189, !dbg !12066
  %191 = fmul <8 x float> %history.i308.i.sroa.0.07313, %_28.i.i.i260.i.sroa.0.0.copyload, !dbg !12071
  %192 = fadd <8 x float> %184, %191, !dbg !12076
  %193 = fmul <8 x float> %history.i308.i.sroa.0.07313, %_31.i.i.i257.i.sroa.0.0.copyload, !dbg !12081
  %194 = fadd <8 x float> %186, %193, !dbg !12086
  %195 = fmul <8 x float> %history.i308.i.sroa.0.07313, %_34.i.i.i254.i.sroa.0.0.copyload, !dbg !12091
  %196 = fadd <8 x float> %188, %195, !dbg !12096
  %197 = fmul <8 x float> %history.i308.i.sroa.10.sroa.0.07323, %_39.i.i.i250.i.sroa.0.0.copyload, !dbg !12101
  %198 = fadd <8 x float> %190, %197, !dbg !12108
  %199 = fmul <8 x float> %history.i308.i.sroa.10.sroa.0.07323, %_42.i.i.i247.i.sroa.0.0.copyload, !dbg !12113
  %200 = fadd <8 x float> %192, %199, !dbg !12118
  %201 = fmul <8 x float> %history.i308.i.sroa.10.sroa.0.07323, %_45.i.i.i244.i.sroa.0.0.copyload, !dbg !12123
  %202 = fadd <8 x float> %194, %201, !dbg !12128
  %203 = fmul <8 x float> %history.i308.i.sroa.10.sroa.0.07323, %_48.i.i.i241.i.sroa.0.0.copyload, !dbg !12133
  %204 = fadd <8 x float> %196, %203, !dbg !12138
  %205 = fmul <8 x float> %history.i308.i.sroa.13.sroa.0.07322, %_53.i.i.i237.i.sroa.0.0.copyload, !dbg !12143
  %206 = fadd <8 x float> %198, %205, !dbg !12150
  %207 = fmul <8 x float> %history.i308.i.sroa.13.sroa.0.07322, %_56.i.i.i234.i.sroa.0.0.copyload, !dbg !12155
  %208 = fadd <8 x float> %200, %207, !dbg !12160
  %209 = fmul <8 x float> %history.i308.i.sroa.13.sroa.0.07322, %_59.i.i.i231.i.sroa.0.0.copyload, !dbg !12165
  %210 = fadd <8 x float> %202, %209, !dbg !12170
  %211 = fmul <8 x float> %history.i308.i.sroa.13.sroa.0.07322, %_62.i.i.i228.i.sroa.0.0.copyload, !dbg !12175
  %212 = fadd <8 x float> %204, %211, !dbg !12180
  %213 = fmul <8 x float> %history.i308.i.sroa.16.sroa.0.07321, %_67.i.i.i224.i.sroa.0.0.copyload, !dbg !12185
  %214 = fadd <8 x float> %206, %213, !dbg !12192
  %215 = fmul <8 x float> %history.i308.i.sroa.16.sroa.0.07321, %_70.i.i.i221.i.sroa.0.0.copyload, !dbg !12197
  %216 = fadd <8 x float> %208, %215, !dbg !12202
  %217 = fmul <8 x float> %history.i308.i.sroa.16.sroa.0.07321, %_73.i.i.i218.i.sroa.0.0.copyload, !dbg !12207
  %218 = fadd <8 x float> %210, %217, !dbg !12212
  %219 = fmul <8 x float> %history.i308.i.sroa.16.sroa.0.07321, %_76.i.i.i215.i.sroa.0.0.copyload, !dbg !12217
  %220 = fadd <8 x float> %212, %219, !dbg !12222
  %221 = fmul <8 x float> %history.i308.i.sroa.19.sroa.0.07320, %_81.i.i.i211.i.sroa.0.0.copyload, !dbg !12227
  %222 = fadd <8 x float> %214, %221, !dbg !12234
  %223 = fmul <8 x float> %history.i308.i.sroa.19.sroa.0.07320, %_84.i.i.i208.i.sroa.0.0.copyload, !dbg !12239
  %224 = fadd <8 x float> %216, %223, !dbg !12244
  %225 = fmul <8 x float> %history.i308.i.sroa.19.sroa.0.07320, %_87.i.i.i205.i.sroa.0.0.copyload, !dbg !12249
  %226 = fadd <8 x float> %218, %225, !dbg !12254
  %227 = fmul <8 x float> %history.i308.i.sroa.19.sroa.0.07320, %_90.i.i.i202.i.sroa.0.0.copyload, !dbg !12259
  %228 = fadd <8 x float> %220, %227, !dbg !12264
  %229 = fmul <8 x float> %history.i308.i.sroa.22.sroa.0.07319, %_95.i.i.i198.i.sroa.0.0.copyload, !dbg !12269
  %230 = fadd <8 x float> %222, %229, !dbg !12276
  %231 = fmul <8 x float> %history.i308.i.sroa.22.sroa.0.07319, %_98.i.i.i195.i.sroa.0.0.copyload, !dbg !12281
  %232 = fadd <8 x float> %224, %231, !dbg !12286
  %233 = fmul <8 x float> %history.i308.i.sroa.22.sroa.0.07319, %_101.i.i.i192.i.sroa.0.0.copyload, !dbg !12291
  %234 = fadd <8 x float> %226, %233, !dbg !12296
  %235 = fmul <8 x float> %history.i308.i.sroa.22.sroa.0.07319, %_104.i.i.i189.i.sroa.0.0.copyload, !dbg !12301
  %236 = fadd <8 x float> %228, %235, !dbg !12306
  %237 = fmul <8 x float> %history.i308.i.sroa.25.sroa.0.07314, %_109.i.i.i185.i.sroa.0.0.copyload, !dbg !12311
  %238 = fadd <8 x float> %230, %237, !dbg !12318
  %239 = fmul <8 x float> %history.i308.i.sroa.25.sroa.0.07314, %_112.i.i.i182.i.sroa.0.0.copyload, !dbg !12323
  %240 = fadd <8 x float> %232, %239, !dbg !12328
  %241 = fmul <8 x float> %history.i308.i.sroa.25.sroa.0.07314, %_115.i.i.i179.i.sroa.0.0.copyload, !dbg !12333
  %242 = fadd <8 x float> %234, %241, !dbg !12338
  %243 = fmul <8 x float> %history.i308.i.sroa.25.sroa.0.07314, %_118.i.i.i176.i.sroa.0.0.copyload, !dbg !12343
  %244 = fadd <8 x float> %236, %243, !dbg !12348
  %245 = fmul <8 x float> %history.i308.i.sroa.29.sroa.0.07315, %_123.i.i.i172.i.sroa.0.0.copyload, !dbg !12353
  %246 = fadd <8 x float> %238, %245, !dbg !12360
  %247 = fmul <8 x float> %history.i308.i.sroa.29.sroa.0.07315, %_126.i.i.i169.i.sroa.0.0.copyload, !dbg !12365
  %248 = fadd <8 x float> %240, %247, !dbg !12370
  %249 = fmul <8 x float> %history.i308.i.sroa.29.sroa.0.07315, %_129.i.i.i166.i.sroa.0.0.copyload, !dbg !12375
  %250 = fadd <8 x float> %242, %249, !dbg !12380
  %251 = fmul <8 x float> %history.i308.i.sroa.29.sroa.0.07315, %_132.i.i.i163.i.sroa.0.0.copyload, !dbg !12385
  %252 = fadd <8 x float> %244, %251, !dbg !12390
  %253 = fmul <8 x float> %history.i308.i.sroa.32.sroa.0.07316, %_137.i.i.i159.i.sroa.0.0.copyload, !dbg !12395
  %254 = fadd <8 x float> %246, %253, !dbg !12402
  %255 = fmul <8 x float> %history.i308.i.sroa.32.sroa.0.07316, %_140.i.i.i156.i.sroa.0.0.copyload, !dbg !12407
  %256 = fadd <8 x float> %248, %255, !dbg !12412
  %257 = fmul <8 x float> %history.i308.i.sroa.32.sroa.0.07316, %_143.i.i.i153.i.sroa.0.0.copyload, !dbg !12417
  %258 = fadd <8 x float> %250, %257, !dbg !12422
  %259 = fmul <8 x float> %history.i308.i.sroa.32.sroa.0.07316, %_146.i.i.i150.i.sroa.0.0.copyload, !dbg !12427
  %260 = fadd <8 x float> %252, %259, !dbg !12432
  %261 = fmul <8 x float> %history.i308.i.sroa.35.sroa.0.07317, %_151.i.i.i146.i.sroa.0.0.copyload, !dbg !12437
  %262 = fadd <8 x float> %254, %261, !dbg !12444
  %263 = fmul <8 x float> %history.i308.i.sroa.35.sroa.0.07317, %_154.i.i.i143.i.sroa.0.0.copyload, !dbg !12449
  %264 = fadd <8 x float> %256, %263, !dbg !12454
  %265 = fmul <8 x float> %history.i308.i.sroa.35.sroa.0.07317, %_157.i.i.i140.i.sroa.0.0.copyload, !dbg !12459
  %266 = fadd <8 x float> %258, %265, !dbg !12464
  %267 = fmul <8 x float> %history.i308.i.sroa.35.sroa.0.07317, %_160.i.i.i137.i.sroa.0.0.copyload, !dbg !12469
  %268 = fadd <8 x float> %260, %267, !dbg !12474
  %269 = fmul <8 x float> %history.i308.i.sroa.38.sroa.0.07318, %_165.i.i.i133.i.sroa.0.0.copyload, !dbg !12479
  %270 = fadd <8 x float> %262, %269, !dbg !12486
  %271 = fmul <8 x float> %history.i308.i.sroa.38.sroa.0.07318, %_168.i.i.i130.i.sroa.0.0.copyload, !dbg !12491
  %272 = fadd <8 x float> %264, %271, !dbg !12496
  %273 = fmul <8 x float> %history.i308.i.sroa.38.sroa.0.07318, %_171.i.i.i127.i.sroa.0.0.copyload, !dbg !12501
  %274 = fadd <8 x float> %266, %273, !dbg !12506
  %275 = fmul <8 x float> %history.i308.i.sroa.38.sroa.0.07318, %_174.i.i.i124.i.sroa.0.0.copyload, !dbg !12511
  %276 = fadd <8 x float> %268, %275, !dbg !12516
  %277 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %270), !dbg !12521
  %278 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %180, <8 x float> %277), !dbg !12529
  %279 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %272), !dbg !12521
  %280 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %278, <8 x float> %279), !dbg !12529
  %281 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %274), !dbg !12521
  %282 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %280, <8 x float> %281), !dbg !12529
  %283 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %276), !dbg !12521
  %284 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %282, <8 x float> %283), !dbg !12529
  %_39.i339.i.idx = shl i64 %iter.sroa.0.0.i310.i7324, 5, !dbg !12538
  %_39.i339.i = getelementptr inbounds nuw i8, ptr %peaks_left.i217, i64 %_39.i339.i.idx, !dbg !12538
  store <8 x float> %284, ptr %_39.i339.i, align 4, !dbg !12549, !alias.scope !12554, !noalias !12558
  %exitcond.not = icmp eq i64 %179, %umax8389, !dbg !11925
  br i1 %exitcond.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit343.i, label %bb5.i312.i, !dbg !11935

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit343.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit, %bb54.i
  %history.i308.i.sroa.0.0.lcssa = phi <8 x float> [ %history.i308.i.sroa.0.0.copyload, %bb54.i ], [ %lanes.i2364.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !12562
  %history.i308.i.sroa.25.sroa.0.0.lcssa = phi <8 x float> [ %history.i308.i.sroa.25.sroa.0.0.copyload, %bb54.i ], [ %history.i308.i.sroa.22.sroa.0.07319, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !12562
  %history.i308.i.sroa.29.sroa.0.0.lcssa = phi <8 x float> [ %history.i308.i.sroa.29.sroa.0.0.copyload, %bb54.i ], [ %history.i308.i.sroa.25.sroa.0.07314, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !12562
  %history.i308.i.sroa.32.sroa.0.0.lcssa = phi <8 x float> [ %history.i308.i.sroa.32.sroa.0.0.copyload, %bb54.i ], [ %history.i308.i.sroa.29.sroa.0.07315, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !12562
  %history.i308.i.sroa.35.sroa.0.0.lcssa = phi <8 x float> [ %history.i308.i.sroa.35.sroa.0.0.copyload, %bb54.i ], [ %history.i308.i.sroa.32.sroa.0.07316, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !12562
  %history.i308.i.sroa.38.sroa.0.0.lcssa = phi <8 x float> [ %history.i308.i.sroa.38.sroa.0.0.copyload, %bb54.i ], [ %history.i308.i.sroa.35.sroa.0.07317, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !12562
  %history.i308.i.sroa.41.sroa.0.0.lcssa = phi <8 x float> [ %history.i308.i.sroa.41.sroa.0.0.copyload, %bb54.i ], [ %history.i308.i.sroa.38.sroa.0.07318, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !12562
  %history.i308.i.sroa.22.sroa.0.0.lcssa = phi <8 x float> [ %history.i308.i.sroa.22.sroa.0.0.copyload, %bb54.i ], [ %history.i308.i.sroa.19.sroa.0.07320, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !12562
  %history.i308.i.sroa.19.sroa.0.0.lcssa = phi <8 x float> [ %history.i308.i.sroa.19.sroa.0.0.copyload, %bb54.i ], [ %history.i308.i.sroa.16.sroa.0.07321, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !12562
  %history.i308.i.sroa.16.sroa.0.0.lcssa = phi <8 x float> [ %history.i308.i.sroa.16.sroa.0.0.copyload, %bb54.i ], [ %history.i308.i.sroa.13.sroa.0.07322, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !12562
  %history.i308.i.sroa.13.sroa.0.0.lcssa = phi <8 x float> [ %history.i308.i.sroa.13.sroa.0.0.copyload, %bb54.i ], [ %history.i308.i.sroa.10.sroa.0.07323, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !12562
  %history.i308.i.sroa.10.sroa.0.0.lcssa = phi <8 x float> [ %history.i308.i.sroa.10.sroa.0.0.copyload, %bb54.i ], [ %history.i308.i.sroa.0.07313, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !12562
  store <8 x float> %history.i308.i.sroa.0.0.lcssa, ptr %hot_left.i224, align 32, !dbg !12563
  store <8 x float> %history.i308.i.sroa.10.sroa.0.0.lcssa, ptr %history.i308.i.sroa.10.0.hot_left.i224.sroa_idx, align 32, !dbg !12563
  store <8 x float> %history.i308.i.sroa.13.sroa.0.0.lcssa, ptr %history.i308.i.sroa.13.0.hot_left.i224.sroa_idx, align 32, !dbg !12563
  store <8 x float> %history.i308.i.sroa.16.sroa.0.0.lcssa, ptr %history.i308.i.sroa.16.0.hot_left.i224.sroa_idx, align 32, !dbg !12563
  store <8 x float> %history.i308.i.sroa.19.sroa.0.0.lcssa, ptr %history.i308.i.sroa.19.0.hot_left.i224.sroa_idx, align 32, !dbg !12563
  store <8 x float> %history.i308.i.sroa.22.sroa.0.0.lcssa, ptr %history.i308.i.sroa.22.0.hot_left.i224.sroa_idx, align 32, !dbg !12563
  store <8 x float> %history.i308.i.sroa.25.sroa.0.0.lcssa, ptr %history.i308.i.sroa.25.0.hot_left.i224.sroa_idx, align 32, !dbg !12563
  store <8 x float> %history.i308.i.sroa.29.sroa.0.0.lcssa, ptr %history.i308.i.sroa.29.0.hot_left.i224.sroa_idx, align 32, !dbg !12563
  store <8 x float> %history.i308.i.sroa.32.sroa.0.0.lcssa, ptr %history.i308.i.sroa.32.0.hot_left.i224.sroa_idx, align 32, !dbg !12563
  store <8 x float> %history.i308.i.sroa.35.sroa.0.0.lcssa, ptr %history.i308.i.sroa.35.0.hot_left.i224.sroa_idx, align 32, !dbg !12563
  store <8 x float> %history.i308.i.sroa.38.sroa.0.0.lcssa, ptr %history.i308.i.sroa.38.0.hot_left.i224.sroa_idx, align 32, !dbg !12563
  store <8 x float> %history.i308.i.sroa.41.sroa.0.0.lcssa, ptr %history.i308.i.sroa.41.0.hot_left.i224.sroa_idx, align 32, !dbg !12563
  %history.i.i190.sroa.0.0.copyload = load <8 x float>, ptr %hot_right.i223, align 32, !dbg !12564
  %history.i.i190.sroa.10.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i190.sroa.10.0.hot_right.i223.sroa_idx, align 32, !dbg !12564
  %history.i.i190.sroa.13.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i190.sroa.13.0.hot_right.i223.sroa_idx, align 32, !dbg !12564
  %history.i.i190.sroa.16.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i190.sroa.16.0.hot_right.i223.sroa_idx, align 32, !dbg !12564
  %history.i.i190.sroa.19.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i190.sroa.19.0.hot_right.i223.sroa_idx, align 32, !dbg !12564
  %history.i.i190.sroa.22.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i190.sroa.22.0.hot_right.i223.sroa_idx, align 32, !dbg !12564
  %history.i.i190.sroa.25.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i190.sroa.25.0.hot_right.i223.sroa_idx, align 32, !dbg !12564
  %history.i.i190.sroa.29.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i190.sroa.29.0.hot_right.i223.sroa_idx, align 32, !dbg !12564
  %history.i.i190.sroa.32.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i190.sroa.32.0.hot_right.i223.sroa_idx, align 32, !dbg !12564
  %history.i.i190.sroa.35.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i190.sroa.35.0.hot_right.i223.sroa_idx, align 32, !dbg !12564
  %history.i.i190.sroa.38.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i190.sroa.38.0.hot_right.i223.sroa_idx, align 32, !dbg !12564
  %history.i.i190.sroa.41.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i190.sroa.41.0.hot_right.i223.sroa_idx, align 32, !dbg !12564
  br i1 %_20.i311.i7312.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i246, label %bb5.i.i260.lr.ph, !dbg !12566

bb5.i.i260.lr.ph:                                 ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit343.i
  %_5.i1882 = load <8 x float>, ptr %self, align 32
  %_14.i.i.i.i155.sroa.0.0.copyload = load <8 x float>, ptr %92, align 32
  %_17.i.i.i.i152.sroa.0.0.copyload = load <8 x float>, ptr %93, align 32
  %_20.i.i.i.i149.sroa.0.0.copyload = load <8 x float>, ptr %94, align 32
  %_25.i.i.i.i145.sroa.0.0.copyload = load <8 x float>, ptr %row12.i.i.i319.i, align 32
  %_28.i.i.i.i142.sroa.0.0.copyload = load <8 x float>, ptr %95, align 32
  %_31.i.i.i.i139.sroa.0.0.copyload = load <8 x float>, ptr %96, align 32
  %_34.i.i.i.i136.sroa.0.0.copyload = load <8 x float>, ptr %97, align 32
  %_39.i.i.i.i132.sroa.0.0.copyload = load <8 x float>, ptr %row13.i.i.i320.i, align 32
  %_42.i.i.i.i129.sroa.0.0.copyload = load <8 x float>, ptr %98, align 32
  %_45.i.i.i.i126.sroa.0.0.copyload = load <8 x float>, ptr %99, align 32
  %_48.i.i.i.i123.sroa.0.0.copyload = load <8 x float>, ptr %100, align 32
  %_53.i.i.i.i119.sroa.0.0.copyload = load <8 x float>, ptr %row14.i.i.i321.i, align 32
  %_56.i.i.i.i116.sroa.0.0.copyload = load <8 x float>, ptr %101, align 32
  %_59.i.i.i.i113.sroa.0.0.copyload = load <8 x float>, ptr %102, align 32
  %_62.i.i.i.i110.sroa.0.0.copyload = load <8 x float>, ptr %103, align 32
  %_67.i.i.i.i106.sroa.0.0.copyload = load <8 x float>, ptr %row15.i.i.i322.i, align 32
  %_70.i.i.i.i103.sroa.0.0.copyload = load <8 x float>, ptr %104, align 32
  %_73.i.i.i.i100.sroa.0.0.copyload = load <8 x float>, ptr %105, align 32
  %_76.i.i.i.i97.sroa.0.0.copyload = load <8 x float>, ptr %106, align 32
  %_81.i.i.i.i93.sroa.0.0.copyload = load <8 x float>, ptr %row16.i.i.i323.i, align 32
  %_84.i.i.i.i90.sroa.0.0.copyload = load <8 x float>, ptr %107, align 32
  %_87.i.i.i.i87.sroa.0.0.copyload = load <8 x float>, ptr %108, align 32
  %_90.i.i.i.i84.sroa.0.0.copyload = load <8 x float>, ptr %109, align 32
  %_95.i.i.i.i80.sroa.0.0.copyload = load <8 x float>, ptr %row17.i.i.i324.i, align 32
  %_98.i.i.i.i77.sroa.0.0.copyload = load <8 x float>, ptr %110, align 32
  %_101.i.i.i.i74.sroa.0.0.copyload = load <8 x float>, ptr %111, align 32
  %_104.i.i.i.i71.sroa.0.0.copyload = load <8 x float>, ptr %112, align 32
  %_109.i.i.i.i67.sroa.0.0.copyload = load <8 x float>, ptr %row18.i.i.i325.i, align 32
  %_112.i.i.i.i64.sroa.0.0.copyload = load <8 x float>, ptr %113, align 32
  %_115.i.i.i.i61.sroa.0.0.copyload = load <8 x float>, ptr %114, align 32
  %_118.i.i.i.i58.sroa.0.0.copyload = load <8 x float>, ptr %115, align 32
  %_123.i.i.i.i54.sroa.0.0.copyload = load <8 x float>, ptr %row19.i.i.i326.i, align 32
  %_126.i.i.i.i51.sroa.0.0.copyload = load <8 x float>, ptr %116, align 32
  %_129.i.i.i.i48.sroa.0.0.copyload = load <8 x float>, ptr %117, align 32
  %_132.i.i.i.i45.sroa.0.0.copyload = load <8 x float>, ptr %118, align 32
  %_137.i.i.i.i41.sroa.0.0.copyload = load <8 x float>, ptr %row20.i.i.i327.i, align 32
  %_140.i.i.i.i38.sroa.0.0.copyload = load <8 x float>, ptr %119, align 32
  %_143.i.i.i.i35.sroa.0.0.copyload = load <8 x float>, ptr %120, align 32
  %_146.i.i.i.i32.sroa.0.0.copyload = load <8 x float>, ptr %121, align 32
  %_151.i.i.i.i28.sroa.0.0.copyload = load <8 x float>, ptr %row21.i.i.i328.i, align 32
  %_154.i.i.i.i25.sroa.0.0.copyload = load <8 x float>, ptr %122, align 32
  %_157.i.i.i.i22.sroa.0.0.copyload = load <8 x float>, ptr %123, align 32
  %_160.i.i.i.i19.sroa.0.0.copyload = load <8 x float>, ptr %124, align 32
  %_165.i.i.i.i15.sroa.0.0.copyload = load <8 x float>, ptr %row22.i.i.i329.i, align 32
  %_168.i.i.i.i12.sroa.0.0.copyload = load <8 x float>, ptr %125, align 32
  %_171.i.i.i.i9.sroa.0.0.copyload = load <8 x float>, ptr %126, align 32
  %_174.i.i.i.i6.sroa.0.0.copyload = load <8 x float>, ptr %127, align 32
  br label %bb5.i.i260, !dbg !12566

bb5.i.i260:                                       ; preds = %bb5.i.i260.lr.ph, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793
  %iter.sroa.0.0.i.i2447350 = phi i64 [ 0, %bb5.i.i260.lr.ph ], [ %285, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793 ]
  %history.i.i190.sroa.10.sroa.0.07349 = phi <8 x float> [ %history.i.i190.sroa.10.sroa.0.0.copyload, %bb5.i.i260.lr.ph ], [ %history.i.i190.sroa.0.07339, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793 ]
  %history.i.i190.sroa.13.sroa.0.07348 = phi <8 x float> [ %history.i.i190.sroa.13.sroa.0.0.copyload, %bb5.i.i260.lr.ph ], [ %history.i.i190.sroa.10.sroa.0.07349, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793 ]
  %history.i.i190.sroa.16.sroa.0.07347 = phi <8 x float> [ %history.i.i190.sroa.16.sroa.0.0.copyload, %bb5.i.i260.lr.ph ], [ %history.i.i190.sroa.13.sroa.0.07348, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793 ]
  %history.i.i190.sroa.19.sroa.0.07346 = phi <8 x float> [ %history.i.i190.sroa.19.sroa.0.0.copyload, %bb5.i.i260.lr.ph ], [ %history.i.i190.sroa.16.sroa.0.07347, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793 ]
  %history.i.i190.sroa.22.sroa.0.07345 = phi <8 x float> [ %history.i.i190.sroa.22.sroa.0.0.copyload, %bb5.i.i260.lr.ph ], [ %history.i.i190.sroa.19.sroa.0.07346, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793 ]
  %history.i.i190.sroa.38.sroa.0.07344 = phi <8 x float> [ %history.i.i190.sroa.38.sroa.0.0.copyload, %bb5.i.i260.lr.ph ], [ %history.i.i190.sroa.35.sroa.0.07343, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793 ]
  %history.i.i190.sroa.35.sroa.0.07343 = phi <8 x float> [ %history.i.i190.sroa.35.sroa.0.0.copyload, %bb5.i.i260.lr.ph ], [ %history.i.i190.sroa.32.sroa.0.07342, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793 ]
  %history.i.i190.sroa.32.sroa.0.07342 = phi <8 x float> [ %history.i.i190.sroa.32.sroa.0.0.copyload, %bb5.i.i260.lr.ph ], [ %history.i.i190.sroa.29.sroa.0.07341, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793 ]
  %history.i.i190.sroa.29.sroa.0.07341 = phi <8 x float> [ %history.i.i190.sroa.29.sroa.0.0.copyload, %bb5.i.i260.lr.ph ], [ %history.i.i190.sroa.25.sroa.0.07340, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793 ]
  %history.i.i190.sroa.25.sroa.0.07340 = phi <8 x float> [ %history.i.i190.sroa.25.sroa.0.0.copyload, %bb5.i.i260.lr.ph ], [ %history.i.i190.sroa.22.sroa.0.07345, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793 ]
  %history.i.i190.sroa.0.07339 = phi <8 x float> [ %history.i.i190.sroa.0.0.copyload, %bb5.i.i260.lr.ph ], [ %lanes.i2373.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793 ]
  %285 = add nuw nsw i64 %iter.sroa.0.0.i.i2447350, 1, !dbg !12569
  %_11.i.i261 = add nuw nsw i64 %iter.sroa.0.0.i.i2447350, %iter.sroa.0.0.i7392, !dbg !12572
  %base.i.i262 = shl i64 %_11.i.i261, 3, !dbg !12572
  %_24.i.i263 = icmp samesign ugt i64 %base.i.i262, %right_io.1, !dbg !12573
  br i1 %_24.i.i263, label %bb7.i.i288, label %bb8.i.i264, !dbg !12573, !prof !905

bb8.i.i264:                                       ; preds = %bb5.i.i260
  %_27.i.i265 = sub nuw nsw i64 %right_io.1, %base.i.i262, !dbg !12576
  %_8.i2376 = icmp samesign ugt i64 %_27.i.i265, 7, !dbg !12577
  br i1 %_8.i2376, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793, label %bb2.i2377, !dbg !12577, !prof !1076

bb2.i2377:                                        ; preds = %bb8.i.i264
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_27.i.i265, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #31, !dbg !12582, !noalias !12583
  unreachable, !dbg !12582

bb7.i.i288:                                       ; preds = %bb5.i.i260
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i.i262, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e0592aef22128a0ac53753b9632a8183) #31, !dbg !12590, !noalias !12591
  unreachable, !dbg !12590

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793: ; preds = %bb8.i.i264
  %_31.i122.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %base.i.i262, !dbg !12592
  %lanes.i2373.sroa.0.0.copyload = load <8 x float>, ptr %_31.i122.i, align 4, !dbg !12594, !alias.scope !12598, !noalias !12602
  %286 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i.i190.sroa.22.sroa.0.07345), !dbg !12604
  %287 = fmul <8 x float> %lanes.i2373.sroa.0.0.copyload, %_5.i1882, !dbg !12611
  %288 = fadd <8 x float> %287, zeroinitializer, !dbg !12617
  %289 = fmul <8 x float> %lanes.i2373.sroa.0.0.copyload, %_14.i.i.i.i155.sroa.0.0.copyload, !dbg !12622
  %290 = fadd <8 x float> %289, zeroinitializer, !dbg !12627
  %291 = fmul <8 x float> %lanes.i2373.sroa.0.0.copyload, %_17.i.i.i.i152.sroa.0.0.copyload, !dbg !12632
  %292 = fadd <8 x float> %291, zeroinitializer, !dbg !12637
  %293 = fmul <8 x float> %lanes.i2373.sroa.0.0.copyload, %_20.i.i.i.i149.sroa.0.0.copyload, !dbg !12642
  %294 = fadd <8 x float> %293, zeroinitializer, !dbg !12647
  %295 = fmul <8 x float> %history.i.i190.sroa.0.07339, %_25.i.i.i.i145.sroa.0.0.copyload, !dbg !12652
  %296 = fadd <8 x float> %288, %295, !dbg !12657
  %297 = fmul <8 x float> %history.i.i190.sroa.0.07339, %_28.i.i.i.i142.sroa.0.0.copyload, !dbg !12662
  %298 = fadd <8 x float> %290, %297, !dbg !12667
  %299 = fmul <8 x float> %history.i.i190.sroa.0.07339, %_31.i.i.i.i139.sroa.0.0.copyload, !dbg !12672
  %300 = fadd <8 x float> %292, %299, !dbg !12677
  %301 = fmul <8 x float> %history.i.i190.sroa.0.07339, %_34.i.i.i.i136.sroa.0.0.copyload, !dbg !12682
  %302 = fadd <8 x float> %294, %301, !dbg !12687
  %303 = fmul <8 x float> %history.i.i190.sroa.10.sroa.0.07349, %_39.i.i.i.i132.sroa.0.0.copyload, !dbg !12692
  %304 = fadd <8 x float> %296, %303, !dbg !12697
  %305 = fmul <8 x float> %history.i.i190.sroa.10.sroa.0.07349, %_42.i.i.i.i129.sroa.0.0.copyload, !dbg !12702
  %306 = fadd <8 x float> %298, %305, !dbg !12707
  %307 = fmul <8 x float> %history.i.i190.sroa.10.sroa.0.07349, %_45.i.i.i.i126.sroa.0.0.copyload, !dbg !12712
  %308 = fadd <8 x float> %300, %307, !dbg !12717
  %309 = fmul <8 x float> %history.i.i190.sroa.10.sroa.0.07349, %_48.i.i.i.i123.sroa.0.0.copyload, !dbg !12722
  %310 = fadd <8 x float> %302, %309, !dbg !12727
  %311 = fmul <8 x float> %history.i.i190.sroa.13.sroa.0.07348, %_53.i.i.i.i119.sroa.0.0.copyload, !dbg !12732
  %312 = fadd <8 x float> %304, %311, !dbg !12737
  %313 = fmul <8 x float> %history.i.i190.sroa.13.sroa.0.07348, %_56.i.i.i.i116.sroa.0.0.copyload, !dbg !12742
  %314 = fadd <8 x float> %306, %313, !dbg !12747
  %315 = fmul <8 x float> %history.i.i190.sroa.13.sroa.0.07348, %_59.i.i.i.i113.sroa.0.0.copyload, !dbg !12752
  %316 = fadd <8 x float> %308, %315, !dbg !12757
  %317 = fmul <8 x float> %history.i.i190.sroa.13.sroa.0.07348, %_62.i.i.i.i110.sroa.0.0.copyload, !dbg !12762
  %318 = fadd <8 x float> %310, %317, !dbg !12767
  %319 = fmul <8 x float> %history.i.i190.sroa.16.sroa.0.07347, %_67.i.i.i.i106.sroa.0.0.copyload, !dbg !12772
  %320 = fadd <8 x float> %312, %319, !dbg !12777
  %321 = fmul <8 x float> %history.i.i190.sroa.16.sroa.0.07347, %_70.i.i.i.i103.sroa.0.0.copyload, !dbg !12782
  %322 = fadd <8 x float> %314, %321, !dbg !12787
  %323 = fmul <8 x float> %history.i.i190.sroa.16.sroa.0.07347, %_73.i.i.i.i100.sroa.0.0.copyload, !dbg !12792
  %324 = fadd <8 x float> %316, %323, !dbg !12797
  %325 = fmul <8 x float> %history.i.i190.sroa.16.sroa.0.07347, %_76.i.i.i.i97.sroa.0.0.copyload, !dbg !12802
  %326 = fadd <8 x float> %318, %325, !dbg !12807
  %327 = fmul <8 x float> %history.i.i190.sroa.19.sroa.0.07346, %_81.i.i.i.i93.sroa.0.0.copyload, !dbg !12812
  %328 = fadd <8 x float> %320, %327, !dbg !12817
  %329 = fmul <8 x float> %history.i.i190.sroa.19.sroa.0.07346, %_84.i.i.i.i90.sroa.0.0.copyload, !dbg !12822
  %330 = fadd <8 x float> %322, %329, !dbg !12827
  %331 = fmul <8 x float> %history.i.i190.sroa.19.sroa.0.07346, %_87.i.i.i.i87.sroa.0.0.copyload, !dbg !12832
  %332 = fadd <8 x float> %324, %331, !dbg !12837
  %333 = fmul <8 x float> %history.i.i190.sroa.19.sroa.0.07346, %_90.i.i.i.i84.sroa.0.0.copyload, !dbg !12842
  %334 = fadd <8 x float> %326, %333, !dbg !12847
  %335 = fmul <8 x float> %history.i.i190.sroa.22.sroa.0.07345, %_95.i.i.i.i80.sroa.0.0.copyload, !dbg !12852
  %336 = fadd <8 x float> %328, %335, !dbg !12857
  %337 = fmul <8 x float> %history.i.i190.sroa.22.sroa.0.07345, %_98.i.i.i.i77.sroa.0.0.copyload, !dbg !12862
  %338 = fadd <8 x float> %330, %337, !dbg !12867
  %339 = fmul <8 x float> %history.i.i190.sroa.22.sroa.0.07345, %_101.i.i.i.i74.sroa.0.0.copyload, !dbg !12872
  %340 = fadd <8 x float> %332, %339, !dbg !12877
  %341 = fmul <8 x float> %history.i.i190.sroa.22.sroa.0.07345, %_104.i.i.i.i71.sroa.0.0.copyload, !dbg !12882
  %342 = fadd <8 x float> %334, %341, !dbg !12887
  %343 = fmul <8 x float> %history.i.i190.sroa.25.sroa.0.07340, %_109.i.i.i.i67.sroa.0.0.copyload, !dbg !12892
  %344 = fadd <8 x float> %336, %343, !dbg !12897
  %345 = fmul <8 x float> %history.i.i190.sroa.25.sroa.0.07340, %_112.i.i.i.i64.sroa.0.0.copyload, !dbg !12902
  %346 = fadd <8 x float> %338, %345, !dbg !12907
  %347 = fmul <8 x float> %history.i.i190.sroa.25.sroa.0.07340, %_115.i.i.i.i61.sroa.0.0.copyload, !dbg !12912
  %348 = fadd <8 x float> %340, %347, !dbg !12917
  %349 = fmul <8 x float> %history.i.i190.sroa.25.sroa.0.07340, %_118.i.i.i.i58.sroa.0.0.copyload, !dbg !12922
  %350 = fadd <8 x float> %342, %349, !dbg !12927
  %351 = fmul <8 x float> %history.i.i190.sroa.29.sroa.0.07341, %_123.i.i.i.i54.sroa.0.0.copyload, !dbg !12932
  %352 = fadd <8 x float> %344, %351, !dbg !12937
  %353 = fmul <8 x float> %history.i.i190.sroa.29.sroa.0.07341, %_126.i.i.i.i51.sroa.0.0.copyload, !dbg !12942
  %354 = fadd <8 x float> %346, %353, !dbg !12947
  %355 = fmul <8 x float> %history.i.i190.sroa.29.sroa.0.07341, %_129.i.i.i.i48.sroa.0.0.copyload, !dbg !12952
  %356 = fadd <8 x float> %348, %355, !dbg !12957
  %357 = fmul <8 x float> %history.i.i190.sroa.29.sroa.0.07341, %_132.i.i.i.i45.sroa.0.0.copyload, !dbg !12962
  %358 = fadd <8 x float> %350, %357, !dbg !12967
  %359 = fmul <8 x float> %history.i.i190.sroa.32.sroa.0.07342, %_137.i.i.i.i41.sroa.0.0.copyload, !dbg !12972
  %360 = fadd <8 x float> %352, %359, !dbg !12977
  %361 = fmul <8 x float> %history.i.i190.sroa.32.sroa.0.07342, %_140.i.i.i.i38.sroa.0.0.copyload, !dbg !12982
  %362 = fadd <8 x float> %354, %361, !dbg !12987
  %363 = fmul <8 x float> %history.i.i190.sroa.32.sroa.0.07342, %_143.i.i.i.i35.sroa.0.0.copyload, !dbg !12992
  %364 = fadd <8 x float> %356, %363, !dbg !12997
  %365 = fmul <8 x float> %history.i.i190.sroa.32.sroa.0.07342, %_146.i.i.i.i32.sroa.0.0.copyload, !dbg !13002
  %366 = fadd <8 x float> %358, %365, !dbg !13007
  %367 = fmul <8 x float> %history.i.i190.sroa.35.sroa.0.07343, %_151.i.i.i.i28.sroa.0.0.copyload, !dbg !13012
  %368 = fadd <8 x float> %360, %367, !dbg !13017
  %369 = fmul <8 x float> %history.i.i190.sroa.35.sroa.0.07343, %_154.i.i.i.i25.sroa.0.0.copyload, !dbg !13022
  %370 = fadd <8 x float> %362, %369, !dbg !13027
  %371 = fmul <8 x float> %history.i.i190.sroa.35.sroa.0.07343, %_157.i.i.i.i22.sroa.0.0.copyload, !dbg !13032
  %372 = fadd <8 x float> %364, %371, !dbg !13037
  %373 = fmul <8 x float> %history.i.i190.sroa.35.sroa.0.07343, %_160.i.i.i.i19.sroa.0.0.copyload, !dbg !13042
  %374 = fadd <8 x float> %366, %373, !dbg !13047
  %375 = fmul <8 x float> %history.i.i190.sroa.38.sroa.0.07344, %_165.i.i.i.i15.sroa.0.0.copyload, !dbg !13052
  %376 = fadd <8 x float> %368, %375, !dbg !13057
  %377 = fmul <8 x float> %history.i.i190.sroa.38.sroa.0.07344, %_168.i.i.i.i12.sroa.0.0.copyload, !dbg !13062
  %378 = fadd <8 x float> %370, %377, !dbg !13067
  %379 = fmul <8 x float> %history.i.i190.sroa.38.sroa.0.07344, %_171.i.i.i.i9.sroa.0.0.copyload, !dbg !13072
  %380 = fadd <8 x float> %372, %379, !dbg !13077
  %381 = fmul <8 x float> %history.i.i190.sroa.38.sroa.0.07344, %_174.i.i.i.i6.sroa.0.0.copyload, !dbg !13082
  %382 = fadd <8 x float> %374, %381, !dbg !13087
  %383 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %376), !dbg !13092
  %384 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %286, <8 x float> %383), !dbg !13098
  %385 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %378), !dbg !13092
  %386 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %384, <8 x float> %385), !dbg !13098
  %387 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %380), !dbg !13092
  %388 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %386, <8 x float> %387), !dbg !13098
  %389 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %382), !dbg !13092
  %390 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %388, <8 x float> %389), !dbg !13098
  %_39.i.i285.idx = shl i64 %iter.sroa.0.0.i.i2447350, 5, !dbg !13103
  %_39.i.i285 = getelementptr inbounds nuw i8, ptr %peaks_right.i216, i64 %_39.i.i285.idx, !dbg !13103
  store <8 x float> %390, ptr %_39.i.i285, align 4, !dbg !13108, !alias.scope !13113, !noalias !13117
  %exitcond8361.not = icmp eq i64 %285, %umax8389, !dbg !13121
  br i1 %exitcond8361.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i246, label %bb5.i.i260, !dbg !12566

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i246: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit343.i
  %history.i.i190.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i190.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit343.i ], [ %lanes.i2373.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793 ], !dbg !13123
  %history.i.i190.sroa.25.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i190.sroa.25.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit343.i ], [ %history.i.i190.sroa.22.sroa.0.07345, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793 ], !dbg !13123
  %history.i.i190.sroa.29.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i190.sroa.29.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit343.i ], [ %history.i.i190.sroa.25.sroa.0.07340, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793 ], !dbg !13123
  %history.i.i190.sroa.32.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i190.sroa.32.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit343.i ], [ %history.i.i190.sroa.29.sroa.0.07341, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793 ], !dbg !13123
  %history.i.i190.sroa.35.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i190.sroa.35.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit343.i ], [ %history.i.i190.sroa.32.sroa.0.07342, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793 ], !dbg !13123
  %history.i.i190.sroa.38.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i190.sroa.38.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit343.i ], [ %history.i.i190.sroa.35.sroa.0.07343, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793 ], !dbg !13123
  %history.i.i190.sroa.41.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i190.sroa.41.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit343.i ], [ %history.i.i190.sroa.38.sroa.0.07344, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793 ], !dbg !13123
  %history.i.i190.sroa.22.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i190.sroa.22.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit343.i ], [ %history.i.i190.sroa.19.sroa.0.07346, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793 ], !dbg !13123
  %history.i.i190.sroa.19.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i190.sroa.19.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit343.i ], [ %history.i.i190.sroa.16.sroa.0.07347, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793 ], !dbg !13123
  %history.i.i190.sroa.16.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i190.sroa.16.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit343.i ], [ %history.i.i190.sroa.13.sroa.0.07348, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793 ], !dbg !13123
  %history.i.i190.sroa.13.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i190.sroa.13.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit343.i ], [ %history.i.i190.sroa.10.sroa.0.07349, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793 ], !dbg !13123
  %history.i.i190.sroa.10.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i190.sroa.10.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit343.i ], [ %history.i.i190.sroa.0.07339, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2793 ], !dbg !13123
  store <8 x float> %history.i.i190.sroa.0.0.lcssa, ptr %hot_right.i223, align 32, !dbg !13124
  store <8 x float> %history.i.i190.sroa.10.sroa.0.0.lcssa, ptr %history.i.i190.sroa.10.0.hot_right.i223.sroa_idx, align 32, !dbg !13124
  store <8 x float> %history.i.i190.sroa.13.sroa.0.0.lcssa, ptr %history.i.i190.sroa.13.0.hot_right.i223.sroa_idx, align 32, !dbg !13124
  store <8 x float> %history.i.i190.sroa.16.sroa.0.0.lcssa, ptr %history.i.i190.sroa.16.0.hot_right.i223.sroa_idx, align 32, !dbg !13124
  store <8 x float> %history.i.i190.sroa.19.sroa.0.0.lcssa, ptr %history.i.i190.sroa.19.0.hot_right.i223.sroa_idx, align 32, !dbg !13124
  store <8 x float> %history.i.i190.sroa.22.sroa.0.0.lcssa, ptr %history.i.i190.sroa.22.0.hot_right.i223.sroa_idx, align 32, !dbg !13124
  store <8 x float> %history.i.i190.sroa.25.sroa.0.0.lcssa, ptr %history.i.i190.sroa.25.0.hot_right.i223.sroa_idx, align 32, !dbg !13124
  store <8 x float> %history.i.i190.sroa.29.sroa.0.0.lcssa, ptr %history.i.i190.sroa.29.0.hot_right.i223.sroa_idx, align 32, !dbg !13124
  store <8 x float> %history.i.i190.sroa.32.sroa.0.0.lcssa, ptr %history.i.i190.sroa.32.0.hot_right.i223.sroa_idx, align 32, !dbg !13124
  store <8 x float> %history.i.i190.sroa.35.sroa.0.0.lcssa, ptr %history.i.i190.sroa.35.0.hot_right.i223.sroa_idx, align 32, !dbg !13124
  store <8 x float> %history.i.i190.sroa.38.sroa.0.0.lcssa, ptr %history.i.i190.sroa.38.0.hot_right.i223.sroa_idx, align 32, !dbg !13124
  store <8 x float> %history.i.i190.sroa.41.sroa.0.0.lcssa, ptr %history.i.i190.sroa.41.0.hot_right.i223.sroa_idx, align 32, !dbg !13124
  br i1 %_20.i311.i7312.not, label %bb25.i.loopexit, label %bb57.i.preheader, !dbg !13125

bb57.i.preheader:                                 ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i246
  %.promoted = load <8 x float>, ptr %128, align 32
  %_69.i.promoted = load <8 x float>, ptr %_69.i, align 32
  %.promoted9458 = load <8 x float>, ptr %129, align 32
  %.promoted9461 = load <8 x float>, ptr %131, align 32
  %_71.i.promoted = load <8 x float>, ptr %_71.i, align 32
  %.promoted9464 = load <8 x float>, ptr %132, align 32
  %.promoted9467 = load <8 x float>, ptr %134, align 32
  %_73.i251.promoted = load <8 x float>, ptr %_73.i251, align 32
  %.promoted9505 = load <8 x float>, ptr %135, align 32
  %.promoted9508 = load <8 x float>, ptr %137, align 32
  %_75.i.promoted = load <8 x float>, ptr %_75.i, align 32
  %.promoted9511 = load <8 x float>, ptr %138, align 32
  %.promoted9514 = load <8 x float>, ptr %152, align 32
  %.promoted9516 = load <8 x float>, ptr %170, align 32
  br label %bb57.i, !dbg !13131

bb57.i:                                           ; preds = %bb57.i.preheader, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2798
  %_58.i.i.sroa.0.0.copyload9517 = phi <8 x float> [ %.promoted9516, %bb57.i.preheader ], [ %553, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2798 ]
  %_58.i44.i.sroa.0.0.copyload9515 = phi <8 x float> [ %.promoted9514, %bb57.i.preheader ], [ %466, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2798 ]
  %_12.i619.sroa.0.0.copyload9513 = phi <8 x float> [ %.promoted9511, %bb57.i.preheader ], [ %_12.i619.sroa.0.0.copyload9512, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2798 ]
  %release_right.i212.sroa.0.0.copyload9510 = phi <8 x float> [ %_75.i.promoted, %bb57.i.preheader ], [ %release_right.i212.sroa.0.0.copyload9509, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2798 ]
  %391 = phi <8 x float> [ %.promoted9508, %bb57.i.preheader ], [ %428, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2798 ]
  %_12.i627.sroa.0.0.copyload9507 = phi <8 x float> [ %.promoted9505, %bb57.i.preheader ], [ %_12.i627.sroa.0.0.copyload9506, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2798 ]
  %limit_right.i213.sroa.0.0.copyload39719504 = phi <8 x float> [ %_73.i251.promoted, %bb57.i.preheader ], [ %limit_right.i213.sroa.0.0.copyload39719503, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2798 ]
  %392 = phi <8 x float> [ %.promoted9467, %bb57.i.preheader ], [ %429, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2798 ]
  %_12.i641.sroa.0.0.copyload9466 = phi <8 x float> [ %.promoted9464, %bb57.i.preheader ], [ %_12.i641.sroa.0.0.copyload9465, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2798 ]
  %release_left.i214.sroa.0.0.copyload9463 = phi <8 x float> [ %_71.i.promoted, %bb57.i.preheader ], [ %release_left.i214.sroa.0.0.copyload9462, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2798 ]
  %393 = phi <8 x float> [ %.promoted9461, %bb57.i.preheader ], [ %430, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2798 ]
  %_12.i655.sroa.0.0.copyload9460 = phi <8 x float> [ %.promoted9458, %bb57.i.preheader ], [ %_12.i655.sroa.0.0.copyload9459, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2798 ]
  %limit_left.i215.sroa.0.0.copyload39689457 = phi <8 x float> [ %_69.i.promoted, %bb57.i.preheader ], [ %limit_left.i215.sroa.0.0.copyload39689456, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2798 ]
  %394 = phi <8 x float> [ %.promoted, %bb57.i.preheader ], [ %431, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2798 ]
  %main_cursor.sroa.0.1.i2497388 = phi i64 [ %main_cursor.sroa.0.0.i2417395, %bb57.i.preheader ], [ %spec.store.select.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2798 ]
  %ring_cursor.sroa.0.1.i2487387 = phi i64 [ %ring_cursor.sroa.0.0.i2407394, %bb57.i.preheader ], [ %spec.store.select13.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2798 ]
  %iter3.sroa.0.0.i2477386 = phi i64 [ 0, %bb57.i.preheader ], [ %395, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2798 ]
  %395 = add nuw nsw i64 %iter3.sroa.0.0.i2477386, 1, !dbg !13132
  %_66.i = add nuw nsw i64 %iter3.sroa.0.0.i2477386, %iter.sroa.0.0.i7392, !dbg !13138
  %base.i250 = shl i64 %_66.i, 3, !dbg !13138
  br i1 %stationary.sroa.0.0.i227, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit2462, label %bb30.i, !dbg !13131

bb30.i:                                           ; preds = %bb57.i
  %396 = fadd <8 x float> %394, splat (float -1.000000e+00), !dbg !13139
  %397 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %396, <8 x float> zeroinitializer), !dbg !13149
  %398 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %397, <8 x float> zeroinitializer, i8 30), !dbg !13154
  %399 = fadd <8 x float> %limit_left.i215.sroa.0.0.copyload39689457, %_12.i655.sroa.0.0.copyload9460, !dbg !13166
  %_13.i654.sroa.0.0.copyload = load <8 x float>, ptr %130, align 32, !dbg !13172
  %400 = bitcast <8 x float> %398 to <8 x i32>, !dbg !13173
  %401 = icmp slt <8 x i32> %400, zeroinitializer, !dbg !13180
  %402 = select <8 x i1> %401, <8 x float> %399, <8 x float> %_13.i654.sroa.0.0.copyload, !dbg !13180
  store <8 x float> %402, ptr %_69.i, align 32, !dbg !13184
  %403 = select <8 x i1> %401, <8 x float> %_12.i655.sroa.0.0.copyload9460, <8 x float> zeroinitializer, !dbg !13185
  store <8 x float> %403, ptr %129, align 32, !dbg !13190
  %404 = fadd <8 x float> %393, splat (float -1.000000e+00), !dbg !13191
  %405 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %404, <8 x float> zeroinitializer), !dbg !13197
  store <8 x float> %405, ptr %131, align 32, !dbg !13202
  %406 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %405, <8 x float> zeroinitializer, i8 30), !dbg !13203
  %407 = fadd <8 x float> %release_left.i214.sroa.0.0.copyload9463, %_12.i641.sroa.0.0.copyload9466, !dbg !13209
  %_13.i640.sroa.0.0.copyload = load <8 x float>, ptr %133, align 32, !dbg !13214
  %408 = bitcast <8 x float> %406 to <8 x i32>, !dbg !13215
  %409 = icmp slt <8 x i32> %408, zeroinitializer, !dbg !13219
  %410 = select <8 x i1> %409, <8 x float> %407, <8 x float> %_13.i640.sroa.0.0.copyload, !dbg !13219
  store <8 x float> %410, ptr %_71.i, align 32, !dbg !13221
  %411 = select <8 x i1> %409, <8 x float> %_12.i641.sroa.0.0.copyload9466, <8 x float> zeroinitializer, !dbg !13222
  store <8 x float> %411, ptr %132, align 32, !dbg !13227
  %412 = fadd <8 x float> %392, splat (float -1.000000e+00), !dbg !13228
  %413 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %412, <8 x float> zeroinitializer), !dbg !13233
  %414 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %413, <8 x float> zeroinitializer, i8 30), !dbg !13238
  %415 = fadd <8 x float> %limit_right.i213.sroa.0.0.copyload39719504, %_12.i627.sroa.0.0.copyload9507, !dbg !13244
  %_13.i626.sroa.0.0.copyload = load <8 x float>, ptr %136, align 32, !dbg !13249
  %416 = bitcast <8 x float> %414 to <8 x i32>, !dbg !13250
  %417 = icmp slt <8 x i32> %416, zeroinitializer, !dbg !13254
  %418 = select <8 x i1> %417, <8 x float> %415, <8 x float> %_13.i626.sroa.0.0.copyload, !dbg !13254
  store <8 x float> %418, ptr %_73.i251, align 32, !dbg !13256
  %419 = select <8 x i1> %417, <8 x float> %_12.i627.sroa.0.0.copyload9507, <8 x float> zeroinitializer, !dbg !13257
  store <8 x float> %419, ptr %135, align 32, !dbg !13262
  %420 = fadd <8 x float> %391, splat (float -1.000000e+00), !dbg !13263
  %421 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %420, <8 x float> zeroinitializer), !dbg !13269
  store <8 x float> %421, ptr %137, align 32, !dbg !13274
  %422 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %421, <8 x float> zeroinitializer, i8 30), !dbg !13275
  %423 = fadd <8 x float> %release_right.i212.sroa.0.0.copyload9510, %_12.i619.sroa.0.0.copyload9513, !dbg !13281
  %_13.i618.sroa.0.0.copyload = load <8 x float>, ptr %139, align 32, !dbg !13286
  %424 = bitcast <8 x float> %422 to <8 x i32>, !dbg !13287
  %425 = icmp slt <8 x i32> %424, zeroinitializer, !dbg !13291
  %426 = select <8 x i1> %425, <8 x float> %423, <8 x float> %_13.i618.sroa.0.0.copyload, !dbg !13291
  store <8 x float> %426, ptr %_75.i, align 32, !dbg !13293
  %427 = select <8 x i1> %425, <8 x float> %_12.i619.sroa.0.0.copyload9513, <8 x float> zeroinitializer, !dbg !13294
  store <8 x float> %427, ptr %138, align 32, !dbg !13299
  br label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit2462, !dbg !13300

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit2462: ; preds = %bb57.i, %bb30.i
  %_12.i619.sroa.0.0.copyload9512 = phi <8 x float> [ %427, %bb30.i ], [ %_12.i619.sroa.0.0.copyload9513, %bb57.i ]
  %release_right.i212.sroa.0.0.copyload9509 = phi <8 x float> [ %426, %bb30.i ], [ %release_right.i212.sroa.0.0.copyload9510, %bb57.i ]
  %428 = phi <8 x float> [ %421, %bb30.i ], [ %391, %bb57.i ]
  %_12.i627.sroa.0.0.copyload9506 = phi <8 x float> [ %419, %bb30.i ], [ %_12.i627.sroa.0.0.copyload9507, %bb57.i ]
  %limit_right.i213.sroa.0.0.copyload39719503 = phi <8 x float> [ %418, %bb30.i ], [ %limit_right.i213.sroa.0.0.copyload39719504, %bb57.i ]
  %429 = phi <8 x float> [ %413, %bb30.i ], [ %392, %bb57.i ]
  %_12.i641.sroa.0.0.copyload9465 = phi <8 x float> [ %411, %bb30.i ], [ %_12.i641.sroa.0.0.copyload9466, %bb57.i ]
  %release_left.i214.sroa.0.0.copyload9462 = phi <8 x float> [ %410, %bb30.i ], [ %release_left.i214.sroa.0.0.copyload9463, %bb57.i ]
  %430 = phi <8 x float> [ %405, %bb30.i ], [ %393, %bb57.i ]
  %_12.i655.sroa.0.0.copyload9459 = phi <8 x float> [ %403, %bb30.i ], [ %_12.i655.sroa.0.0.copyload9460, %bb57.i ]
  %limit_left.i215.sroa.0.0.copyload39689456 = phi <8 x float> [ %402, %bb30.i ], [ %limit_left.i215.sroa.0.0.copyload39689457, %bb57.i ]
  %431 = phi <8 x float> [ %397, %bb30.i ], [ %394, %bb57.i ]
  %_79.i = shl i64 %iter3.sroa.0.0.i2477386, 3, !dbg !13301
  %_138.i = getelementptr inbounds nuw float, ptr %peaks_left.i217, i64 %_79.i, !dbg !13303
  %lanes.i2455.sroa.0.0.copyload = load <8 x float>, ptr %_138.i, align 4, !dbg !13314, !alias.scope !13319, !noalias !13323
  %_143.i = getelementptr inbounds nuw float, ptr %peaks_right.i216, i64 %_79.i, !dbg !13327
  %lanes.i2446.sroa.0.0.copyload = load <8 x float>, ptr %_143.i, align 4, !dbg !13338, !alias.scope !13343, !noalias !13347
  %432 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i2446.sroa.0.0.copyload, <8 x float> %lanes.i2455.sroa.0.0.copyload), !dbg !13351
  %433 = select <8 x i1> %141, <8 x float> %432, <8 x float> %lanes.i2455.sroa.0.0.copyload, !dbg !13357
  %434 = select <8 x i1> %141, <8 x float> %432, <8 x float> %lanes.i2446.sroa.0.0.copyload, !dbg !13363
  %_144.i252 = icmp samesign ugt i64 %base.i250, %left_io.1, !dbg !13369
  br i1 %_144.i252, label %bb61.i, label %bb62.i, !dbg !13369, !prof !905

bb62.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit2462
  %_147.i = sub nuw nsw i64 %left_io.1, %base.i250, !dbg !13374
  %_151.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %base.i250, !dbg !13375
  %_8.i2440 = icmp samesign ugt i64 %_147.i, 7, !dbg !13380
  br i1 %_8.i2440, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit2444, label %bb2.i2441, !dbg !13380, !prof !1076

bb2.i2441:                                        ; preds = %bb62.i
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_147.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #31, !dbg !13385, !noalias !13386
  unreachable, !dbg !13385

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit2444: ; preds = %bb62.i
  %_92.i = load i64, ptr %142, align 8, !dbg !13390, !alias.scope !11709, !noalias !13391, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13392), !dbg !13395
  %width.i59.i = load i64, ptr %143, align 8, !dbg !13396, !alias.scope !13399, !noalias !13400, !noundef !12
  %435 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %433, <8 x float> %limit_left.i215.sroa.0.0.copyload39689456, i8 30), !dbg !13409
  %436 = fdiv <8 x float> %limit_left.i215.sroa.0.0.copyload39689456, %433, !dbg !13417
  %437 = bitcast <8 x float> %435 to <8 x i32>, !dbg !13427
  %438 = icmp slt <8 x i32> %437, zeroinitializer, !dbg !13431
  %439 = select <8 x i1> %438, <8 x float> %436, <8 x float> splat (float 1.000000e+00), !dbg !13431
  %_144.1.i60.i = load i64, ptr %144, align 8, !dbg !13433, !alias.scope !13399, !noalias !13400, !noundef !12
  %_22.i61.i = mul i64 %width.i59.i, %ring_cursor.sroa.0.1.i2487387, !dbg !13435
  %_92.i62.i = icmp ugt i64 %_22.i61.i, %_144.1.i60.i, !dbg !13436
  br i1 %_92.i62.i, label %bb37.i120.i, label %bb38.i63.i, !dbg !13436, !prof !905

bb38.i63.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit2444
  %_95.i65.i = sub nuw i64 %_144.1.i60.i, %_22.i61.i, !dbg !13441
  %_8.i2830 = icmp samesign ugt i64 %_95.i65.i, 7, !dbg !13442
  br i1 %_8.i2830, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2833, label %bb2.i2831, !dbg !13442, !prof !1076

bb2.i2831:                                        ; preds = %bb38.i63.i
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_95.i65.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #31, !dbg !13447, !noalias !13448
  unreachable, !dbg !13447

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2833: ; preds = %bb38.i63.i
  %_144.0.i64.i = load ptr, ptr %145, align 8, !dbg !13433, !alias.scope !13399, !noalias !13400, !nonnull !12, !noundef !12
  %_99.i66.i = getelementptr inbounds nuw float, ptr %_144.0.i64.i, i64 %_22.i61.i, !dbg !13452
  store <8 x float> %439, ptr %_99.i66.i, align 4, !dbg !13457, !alias.scope !13461, !noalias !13465
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13467), !dbg !13470
  %width.i487 = load i64, ptr %143, align 8, !dbg !13471, !alias.scope !13467, !noalias !13473, !noundef !12
  %440 = icmp eq i64 %width.i487, 0, !dbg !13475
  br i1 %440, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit595, label %bb32.i494.lr.ph, !dbg !13475

bb32.i494.lr.ph:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2833
  %_112.1.i497 = load i64, ptr %40, align 8, !alias.scope !13467, !noalias !13473, !noundef !12
  %_112.0.i501 = load ptr, ptr %39, align 8, !nonnull !12
  %441 = add i64 %ring_cursor.sroa.0.1.i2487387, 1
  %_23.not.i508 = icmp ult i64 %441, %_92.i
  %442 = select i1 %_23.not.i508, i64 0, i64 %_92.i
  %start1.sroa.0.0.i509 = sub nuw i64 %441, %442
  %_114.1.i512 = load i64, ptr %144, align 8
  %_114.0.i516 = load ptr, ptr %145, align 8, !nonnull !12
  %_116.1.i517 = load i64, ptr %146, align 8
  %_116.0.i521 = load ptr, ptr %147, align 8, !nonnull !12
  %_118.1.i525 = load i64, ptr %148, align 8
  %_118.0.i529 = load ptr, ptr %149, align 8, !nonnull !12
  %_45.i542 = mul i64 %width.i487, %start1.sroa.0.0.i509
  br label %bb32.i494, !dbg !13475

bb32.i494:                                        ; preds = %bb32.i494.lr.ph, %bb31.i557
  %iter.i486.sroa.10.07368 = phi i64 [ %width.i487, %bb32.i494.lr.ph ], [ %443, %bb31.i557 ]
  %iter.i486.sroa.7.07367 = phi i64 [ 0, %bb32.i494.lr.ph ], [ %_9.0.i, %bb31.i557 ]
  %iter.i486.sroa.0.0.idx7366 = phi i64 [ 0, %bb32.i494.lr.ph ], [ %iter.i486.sroa.0.0.add, %bb31.i557 ]
  %iter.i486.sroa.0.0.ptr7369 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 %iter.i486.sroa.0.0.idx7366, !dbg !13477
  %443 = add i64 %iter.i486.sroa.10.07368, -1, !dbg !13477
  %_7.i.i3198 = icmp eq i64 %iter.i486.sroa.0.0.idx7366, 32, !dbg !13478
  br i1 %_7.i.i3198, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit595, label %bb3.i496, !dbg !13482

bb3.i496:                                         ; preds = %bb32.i494
  %iter.i486.sroa.0.0.add = add nuw nsw i64 %iter.i486.sroa.0.0.idx7366, 4, !dbg !13483
  %_9.0.i = add nuw nsw i64 %iter.i486.sroa.7.07367, 1, !dbg !13485
  %exitcond8369.not = icmp eq i64 %iter.i486.sroa.7.07367, %_112.1.i497, !dbg !13486
  br i1 %exitcond8369.not, label %panic.i499, label %bb5.i500, !dbg !13486

bb5.i500:                                         ; preds = %bb3.i496
  %444 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i501, i64 %iter.i486.sroa.7.07367, !dbg !13486
  %shape.i502 = load i32, ptr %444, align 4, !dbg !13486, !noalias !13487, !noundef !12
  %445 = getelementptr inbounds nuw i8, ptr %444, i64 4, !dbg !13486
  %shape3.i503 = load i32, ptr %445, align 4, !dbg !13486, !noalias !13487, !noundef !12
  %window.i504 = zext i32 %shape.i502 to i64, !dbg !13488
  %_19.i505 = zext i32 %shape3.i503 to i64, !dbg !13489
  %446 = add i64 %ring_cursor.sroa.0.1.i2487387, %_19.i505, !dbg !13490
  %_20.not.i506 = icmp ult i64 %446, %_92.i, !dbg !13491
  %447 = select i1 %_20.not.i506, i64 0, i64 %_92.i, !dbg !13491
  %spec.select.i507 = sub nuw i64 %446, %447, !dbg !13491
  %_27.i510 = mul i64 %spec.select.i507, %width.i487, !dbg !13492
  %_26.i511 = add i64 %_27.i510, %iter.i486.sroa.7.07367, !dbg !13492
  %_30.i513 = icmp ult i64 %_26.i511, %_114.1.i512, !dbg !13493
  br i1 %_30.i513, label %bb12.i515, label %panic5.i514, !dbg !13493

panic.i499:                                       ; preds = %bb3.i496
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i497, i64 noundef %_112.1.i497, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_077370d5cece7380867993336836eb69) #31, !dbg !13486, !noalias !13487
  unreachable, !dbg !13486

bb12.i515:                                        ; preds = %bb5.i500
  %448 = getelementptr inbounds nuw float, ptr %_114.0.i516, i64 %_26.i511, !dbg !13493
  %449 = load float, ptr %448, align 4, !dbg !13493, !noalias !13487, !noundef !12
  %exitcond8370.not = icmp eq i64 %iter.i486.sroa.7.07367, %_116.1.i517, !dbg !13494
  br i1 %exitcond8370.not, label %panic6.i519, label %bb13.i520, !dbg !13494

panic5.i514:                                      ; preds = %bb5.i500
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i511, i64 noundef %_114.1.i512, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5cae9ea88ed6362f62dbad10291edbd4) #31, !dbg !13493, !noalias !13487
  unreachable, !dbg !13493

bb13.i520:                                        ; preds = %bb12.i515
  %450 = getelementptr inbounds nuw i32, ptr %_116.0.i521, i64 %iter.i486.sroa.7.07367, !dbg !13494
  %_32.i522 = load i32, ptr %450, align 4, !dbg !13494, !noalias !13487, !noundef !12
  %position.i523 = zext i32 %_32.i522 to i64, !dbg !13494
  %451 = icmp eq i32 %_32.i522, 0, !dbg !13495
  br i1 %451, label %bb17.i532, label %bb15.i524, !dbg !13495

panic6.i519:                                      ; preds = %bb12.i515
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i517, i64 noundef %_116.1.i517, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9fdd00ed7abe4ccfd73f0d145a7b741b) #31, !dbg !13494, !noalias !13487
  unreachable, !dbg !13494

bb15.i524:                                        ; preds = %bb13.i520
  %_37.i526 = icmp ult i64 %iter.i486.sroa.7.07367, %_118.1.i525, !dbg !13496
  br i1 %_37.i526, label %bb16.i528, label %panic7.i527, !dbg !13496

bb17.i532:                                        ; preds = %bb35.i593, %bb16.i528, %bb13.i520
  %newest.sroa.0.0.i533 = phi float [ %449, %bb13.i520 ], [ %_35.i530, %bb35.i593 ], [ %449, %bb16.i528 ], !dbg !13497
  %exitcond8371.not = icmp eq i64 %iter.i486.sroa.7.07367, %_118.1.i525, !dbg !13498
  br i1 %exitcond8371.not, label %panic8.i536, label %bb18.i537, !dbg !13498

bb16.i528:                                        ; preds = %bb15.i524
  %452 = getelementptr inbounds nuw float, ptr %_118.0.i529, i64 %iter.i486.sroa.7.07367, !dbg !13496
  %_35.i530 = load float, ptr %452, align 4, !dbg !13496, !noalias !13487, !noundef !12
  %_102.i531 = fcmp olt float %_35.i530, %449, !dbg !13499
  br i1 %_102.i531, label %bb35.i593, label %bb17.i532, !dbg !13499

panic7.i527:                                      ; preds = %bb15.i524
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i486.sroa.7.07367, i64 noundef %_118.1.i525, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_697dc8945f5e5b040d7e9a0b92cb249f) #31, !dbg !13496, !noalias !13487
  unreachable, !dbg !13496

bb35.i593:                                        ; preds = %bb16.i528
  br label %bb17.i532, !dbg !13501

bb18.i537:                                        ; preds = %bb17.i532
  %453 = getelementptr inbounds nuw float, ptr %_118.0.i529, i64 %iter.i486.sroa.7.07367, !dbg !13498
  store float %newest.sroa.0.0.i533, ptr %453, align 4, !dbg !13498, !noalias !13487
  %_42.i539 = add nuw nsw i64 %position.i523, 1, !dbg !13502
  %complete.i540 = icmp eq i64 %_42.i539, %window.i504, !dbg !13502
  br i1 %complete.i540, label %bb22.i562, label %bb20.i541, !dbg !13503

panic8.i536:                                      ; preds = %bb17.i532
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i525, i64 noundef %_118.1.i525, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_067dce244605df4a33956fbd4d726027) #31, !dbg !13498, !noalias !13487
  unreachable, !dbg !13498

bb20.i541:                                        ; preds = %bb18.i537
  %_44.i543 = add i64 %iter.i486.sroa.7.07367, %_45.i542, !dbg !13504
  %_47.i545 = icmp ult i64 %_44.i543, %_114.1.i512, !dbg !13505
  br i1 %_47.i545, label %bb30.i555, label %panic9.i546, !dbg !13505

panic9.i546:                                      ; preds = %bb20.i541
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i543, i64 noundef %_114.1.i512, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d5ea38731a1cb6311efef2f919234d06) #31, !dbg !13505, !noalias !13487
  unreachable, !dbg !13505

bb30.i555:                                        ; preds = %bb20.i541
  %454 = getelementptr inbounds nuw float, ptr %_114.0.i516, i64 %_44.i543, !dbg !13505
  %_43.i549 = load float, ptr %454, align 4, !dbg !13505, !noalias !13487, !noundef !12
  %_103.i550 = fcmp olt float %_43.i549, %newest.sroa.0.0.i533, !dbg !13506
  %newest.sroa.0.1.i551 = select i1 %_103.i550, float %_43.i549, float %newest.sroa.0.0.i533, !dbg !13506
  store float %newest.sroa.0.1.i551, ptr %iter.i486.sroa.0.0.ptr7369, align 4, !dbg !13508, !noalias !13487
  %455 = trunc i64 %_42.i539 to i32, !dbg !13509
  br label %bb31.i557, !dbg !13510

bb31.i557:                                        ; preds = %bb25.i590, %bb30.i555
  %storemerge = phi i32 [ %455, %bb30.i555 ], [ 0, %bb25.i590 ], !dbg !13511
  store i32 %storemerge, ptr %450, align 4, !dbg !13511, !noalias !13487
  %456 = icmp eq i64 %443, 0, !dbg !13475
  br i1 %456, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit595, label %bb32.i494, !dbg !13475

bb22.i562:                                        ; preds = %bb18.i537
  store float %newest.sroa.0.0.i533, ptr %iter.i486.sroa.0.0.ptr7369, align 4, !dbg !13508, !noalias !13487
  %457 = load float, ptr %448, align 4, !dbg !13512, !noalias !13487, !noundef !12
  br label %bb41.i575, !dbg !13513

bb41.i575:                                        ; preds = %bb22.i562, %bb25.i590
  %iter2.sroa.0.0.i5677365 = phi i64 [ 0, %bb22.i562 ], [ %_105.i576, %bb25.i590 ]
  %suffix.sroa.0.0.i5667364 = phi float [ %457, %bb22.i562 ], [ %suffix.sroa.0.1.i586, %bb25.i590 ]
  %end.sroa.0.1.i5657363 = phi i64 [ %spec.select.i507, %bb22.i562 ], [ %460, %bb25.i590 ]
  %_56.i577 = mul i64 %end.sroa.0.1.i5657363, %width.i487, !dbg !13516
  %_55.i578 = add i64 %_56.i577, %iter.i486.sroa.7.07367, !dbg !13516
  %_59.i580 = icmp ult i64 %_55.i578, %_114.1.i512, !dbg !13517
  br i1 %_59.i580, label %bb25.i590, label %panic13.i581, !dbg !13517

panic13.i581:                                     ; preds = %bb41.i575
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i578, i64 noundef %_114.1.i512, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a19fa7d9077791c9eadbe74fafea8de7) #31, !dbg !13517, !noalias !13487
  unreachable, !dbg !13517

bb25.i590:                                        ; preds = %bb41.i575
  %_105.i576 = add nuw nsw i64 %iter2.sroa.0.0.i5677365, 1, !dbg !13518
  %458 = getelementptr inbounds nuw float, ptr %_114.0.i516, i64 %_55.i578, !dbg !13517
  %_54.i584 = load float, ptr %458, align 4, !dbg !13517, !noalias !13487, !noundef !12
  %_107.i585 = fcmp olt float %suffix.sroa.0.0.i5667364, %_54.i584, !dbg !13521
  %suffix.sroa.0.1.i586 = select i1 %_107.i585, float %suffix.sroa.0.0.i5667364, float %_54.i584, !dbg !13521
  store float %suffix.sroa.0.1.i586, ptr %458, align 4, !dbg !13523, !noalias !13487
  %459 = icmp eq i64 %end.sroa.0.1.i5657363, 0, !dbg !13524
  %spec.store.select.i592 = select i1 %459, i64 %_92.i, i64 %end.sroa.0.1.i5657363, !dbg !13524
  %460 = add i64 %spec.store.select.i592, -1, !dbg !13525
  %exitcond8368.not = icmp eq i64 %_105.i576, %window.i504, !dbg !13526
  br i1 %exitcond8368.not, label %bb31.i557, label %bb41.i575, !dbg !13513

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit595: ; preds = %bb31.i557, %bb32.i494, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2833
  %lanes.i2430.sroa.0.0.copyload = load <8 x float>, ptr %scratch.i, align 4, !dbg !13528, !alias.scope !13533, !noalias !13537
  %461 = fmul <8 x float> %lanes.i2430.sroa.0.0.copyload, splat (float 1.638400e+04), !dbg !13541
  %462 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %461), !dbg !13547
  %463 = fmul <8 x float> %462, splat (float 0x3F10000000000000), !dbg !13556
  %464 = icmp eq i64 %width.i59.i, 0, !dbg !13561
  %_149.1.i94.i.pre = load i64, ptr %150, align 8, !dbg !13567, !alias.scope !13399, !noalias !13400
  br i1 %464, label %bb16.i93.i, label %bb39.i73.i.lr.ph, !dbg !13561

bb39.i73.i.lr.ph:                                 ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit595
  %_145.1.i76.i = load i64, ptr %40, align 8, !alias.scope !13399, !noalias !13400, !noundef !12
  %_145.0.i80.i = load ptr, ptr %39, align 8, !nonnull !12
  %_147.0.i91.i = load ptr, ptr %151, align 8, !nonnull !12
  %exitcond8374.not = icmp eq i64 %_145.1.i76.i, 0, !dbg !13569
  br i1 %exitcond8374.not, label %panic.i78.i, label %bb17.i79.i, !dbg !13569

bb37.i120.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit2444
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i61.i, i64 noundef %_144.1.i60.i, i64 noundef %_144.1.i60.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9d2713d1692431af37d60082290049ed) #31, !dbg !13571, !noalias !13572
  unreachable, !dbg !13571

bb16.i93.i.loopexit:                              ; preds = %bb21.i90.i.7, %bb21.i90.i.6, %bb21.i90.i.5, %bb21.i90.i.4, %bb21.i90.i.3, %bb21.i90.i.2, %bb21.i90.i.1, %bb21.i90.i
  %lanes.i2423.sroa.0.0.copyload.pre = load <8 x float>, ptr %scratch.i, align 4, !dbg !13573, !alias.scope !13578, !noalias !13582
  br label %bb16.i93.i, !dbg !13586

bb16.i93.i:                                       ; preds = %bb16.i93.i.loopexit, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit595
  %lanes.i2423.sroa.0.0.copyload = phi <8 x float> [ %lanes.i2423.sroa.0.0.copyload.pre, %bb16.i93.i.loopexit ], [ %lanes.i2430.sroa.0.0.copyload, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit595 ], !dbg !13573
  %465 = fadd <8 x float> %463, %_58.i44.i.sroa.0.0.copyload9515, !dbg !13587
  %466 = fsub <8 x float> %465, %lanes.i2423.sroa.0.0.copyload, !dbg !13592
  store <8 x float> %466, ptr %152, align 32, !dbg !13597
  %_109.i95.i = icmp ugt i64 %_22.i61.i, %_149.1.i94.i.pre, !dbg !13598
  br i1 %_109.i95.i, label %bb42.i119.i, label %bb43.i96.i, !dbg !13598, !prof !905

bb43.i96.i:                                       ; preds = %bb16.i93.i
  %_112.i98.i = sub nuw i64 %_149.1.i94.i.pre, %_22.i61.i, !dbg !13602
  %_8.i2825 = icmp samesign ugt i64 %_112.i98.i, 7, !dbg !13603
  br i1 %_8.i2825, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2828, label %bb2.i2826, !dbg !13603, !prof !1076

bb2.i2826:                                        ; preds = %bb43.i96.i
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_112.i98.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #31, !dbg !13608, !noalias !13609
  unreachable, !dbg !13608

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2828: ; preds = %bb43.i96.i
  %_149.0.i97.i = load ptr, ptr %151, align 8, !dbg !13567, !alias.scope !13399, !noalias !13400, !nonnull !12, !noundef !12
  %_116.i99.i = getelementptr inbounds nuw float, ptr %_149.0.i97.i, i64 %_22.i61.i, !dbg !13613
  store <8 x float> %463, ptr %_116.i99.i, align 4, !dbg !13618, !alias.scope !13622, !noalias !13626
  %_64.i41.i.sroa.0.0.copyload = load <8 x float>, ptr %153, align 32, !dbg !13628
  %_68.i37.i.sroa.0.0.copyload = load <8 x float>, ptr %154, align 32, !dbg !13629
  %467 = fdiv <8 x float> %466, %_64.i41.i.sroa.0.0.copyload, !dbg !13632
  %468 = fsub <8 x float> splat (float 1.000000e+00), %467, !dbg !13637
  %469 = fsub <8 x float> %468, %_68.i37.i.sroa.0.0.copyload, !dbg !13642
  %470 = fmul <8 x float> %release_left.i214.sroa.0.0.copyload9462, %469, !dbg !13647
  %471 = fadd <8 x float> %_68.i37.i.sroa.0.0.copyload, %470, !dbg !13655
  %472 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %468, <8 x float> %471), !dbg !13662
  %473 = bitcast <8 x float> %472 to <8 x i32>, !dbg !13668
  %474 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %472), !dbg !13675
  %475 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %474, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !13677
  %476 = bitcast <8 x float> %475 to <8 x i32>, !dbg !13689
  %477 = xor <8 x i32> %476, splat (i32 -1), !dbg !13701
  %478 = and <8 x i32> %477, %473, !dbg !13703
  %479 = bitcast <8 x i32> %478 to <8 x float>, !dbg !13709
  store <8 x i32> %478, ptr %154, align 32, !dbg !13710
  %480 = fsub <8 x float> splat (float 1.000000e+00), %479, !dbg !13711
  %_150.1.i100.i = load i64, ptr %155, align 8, !dbg !13716, !alias.scope !13399, !noalias !13400, !noundef !12
  %_76.i101.i = mul i64 %width.i59.i, %main_cursor.sroa.0.1.i2497388, !dbg !13718
  %_120.i102.i = icmp ugt i64 %_76.i101.i, %_150.1.i100.i, !dbg !13719
  br i1 %_120.i102.i, label %bb48.i118.i, label %bb49.i103.i, !dbg !13719, !prof !905

bb42.i119.i:                                      ; preds = %bb16.i93.i
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i61.i, i64 noundef %_149.1.i94.i.pre, i64 noundef %_149.1.i94.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8002ed69501742f3ea2ea25eb68cd581) #31, !dbg !13724, !noalias !13725
  unreachable, !dbg !13724

bb49.i103.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2828
  %_123.i105.i = sub nuw i64 %_150.1.i100.i, %_76.i101.i, !dbg !13726
  %_8.i2417 = icmp samesign ugt i64 %_123.i105.i, 7, !dbg !13727
  br i1 %_8.i2417, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2818, label %bb2.i2418, !dbg !13727, !prof !1076

bb2.i2418:                                        ; preds = %bb49.i103.i
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_123.i105.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #31, !dbg !13732, !noalias !13733
  unreachable, !dbg !13732

bb48.i118.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2828
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i101.i, i64 noundef %_150.1.i100.i, i64 noundef %_150.1.i100.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a07d19b424a92543209d516ece5bdf2a) #31, !dbg !13737, !noalias !13725
  unreachable, !dbg !13737

bb17.i79.i:                                       ; preds = %bb39.i73.i.lr.ph
  %481 = getelementptr inbounds nuw i8, ptr %_145.0.i80.i, i64 8, !dbg !13569
  %_44.i81.i = load i32, ptr %481, align 4, !dbg !13569, !noalias !13725, !noundef !12
  %_43.i82.i = zext i32 %_44.i81.i to i64, !dbg !13569
  %482 = add i64 %ring_cursor.sroa.0.1.i2487387, %_43.i82.i, !dbg !13738
  %_47.not.i83.i = icmp ult i64 %482, %_92.i, !dbg !13739
  %483 = select i1 %_47.not.i83.i, i64 0, i64 %_92.i, !dbg !13739
  %spec.select.i84.i = sub nuw i64 %482, %483, !dbg !13739
  %_51.i85.i = mul i64 %spec.select.i84.i, %width.i59.i, !dbg !13741
  %_53.i88.i = icmp ult i64 %_51.i85.i, %_149.1.i94.i.pre, !dbg !13742
  br i1 %_53.i88.i, label %bb21.i90.i, label %panic1.i89.i, !dbg !13742

panic.i78.i:                                      ; preds = %bb39.i73.i.7, %bb39.i73.i.6, %bb39.i73.i.5, %bb39.i73.i.4, %bb39.i73.i.3, %bb39.i73.i.2, %bb39.i73.i.1, %bb39.i73.i.lr.ph
  %_145.1.i76.i.lcssa.ph = phi i64 [ 7, %bb39.i73.i.7 ], [ 6, %bb39.i73.i.6 ], [ 5, %bb39.i73.i.5 ], [ 4, %bb39.i73.i.4 ], [ 3, %bb39.i73.i.3 ], [ 2, %bb39.i73.i.2 ], [ 1, %bb39.i73.i.1 ], [ 0, %bb39.i73.i.lr.ph ]
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i76.i.lcssa.ph, i64 noundef %_145.1.i76.i.lcssa.ph, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_891dd683d7367e0a1ec5a22f9c2a2aa8) #31, !dbg !13569, !noalias !13725
  unreachable, !dbg !13569

bb21.i90.i:                                       ; preds = %bb17.i79.i
  %484 = getelementptr inbounds nuw float, ptr %_147.0.i91.i, i64 %_51.i85.i, !dbg !13742
  %_49.i92.i = load float, ptr %484, align 4, !dbg !13742, !noalias !13725, !noundef !12
  store float %_49.i92.i, ptr %scratch.i, align 4, !dbg !13743, !noalias !13725
  %485 = icmp eq i64 %width.i59.i, 1, !dbg !13561
  br i1 %485, label %bb16.i93.i.loopexit, label %bb39.i73.i.1, !dbg !13561

bb39.i73.i.1:                                     ; preds = %bb21.i90.i
  %exitcond8374.1.not = icmp eq i64 %_145.1.i76.i, 1, !dbg !13569
  br i1 %exitcond8374.1.not, label %panic.i78.i, label %bb17.i79.i.1, !dbg !13569

bb17.i79.i.1:                                     ; preds = %bb39.i73.i.1
  %486 = getelementptr inbounds nuw i8, ptr %_145.0.i80.i, i64 20, !dbg !13569
  %_44.i81.i.1 = load i32, ptr %486, align 4, !dbg !13569, !noalias !13725, !noundef !12
  %_43.i82.i.1 = zext i32 %_44.i81.i.1 to i64, !dbg !13569
  %487 = add i64 %ring_cursor.sroa.0.1.i2487387, %_43.i82.i.1, !dbg !13738
  %_47.not.i83.i.1 = icmp ult i64 %487, %_92.i, !dbg !13739
  %488 = select i1 %_47.not.i83.i.1, i64 0, i64 %_92.i, !dbg !13739
  %spec.select.i84.i.1 = sub nuw i64 %487, %488, !dbg !13739
  %_51.i85.i.1 = mul i64 %spec.select.i84.i.1, %width.i59.i, !dbg !13741
  %_50.i86.i.1 = add i64 %_51.i85.i.1, 1, !dbg !13741
  %_53.i88.i.1 = icmp ult i64 %_50.i86.i.1, %_149.1.i94.i.pre, !dbg !13742
  br i1 %_53.i88.i.1, label %bb21.i90.i.1, label %panic1.i89.i, !dbg !13742

bb21.i90.i.1:                                     ; preds = %bb17.i79.i.1
  %489 = getelementptr inbounds nuw float, ptr %_147.0.i91.i, i64 %_50.i86.i.1, !dbg !13742
  %_49.i92.i.1 = load float, ptr %489, align 4, !dbg !13742, !noalias !13725, !noundef !12
  store float %_49.i92.i.1, ptr %iter.i47.i.sroa.0.0.ptr7373.1, align 4, !dbg !13743, !noalias !13725
  %490 = icmp eq i64 %width.i59.i, 2, !dbg !13561
  br i1 %490, label %bb16.i93.i.loopexit, label %bb39.i73.i.2, !dbg !13561

bb39.i73.i.2:                                     ; preds = %bb21.i90.i.1
  %exitcond8374.2.not = icmp eq i64 %_145.1.i76.i, 2, !dbg !13569
  br i1 %exitcond8374.2.not, label %panic.i78.i, label %bb17.i79.i.2, !dbg !13569

bb17.i79.i.2:                                     ; preds = %bb39.i73.i.2
  %491 = getelementptr inbounds nuw i8, ptr %_145.0.i80.i, i64 32, !dbg !13569
  %_44.i81.i.2 = load i32, ptr %491, align 4, !dbg !13569, !noalias !13725, !noundef !12
  %_43.i82.i.2 = zext i32 %_44.i81.i.2 to i64, !dbg !13569
  %492 = add i64 %ring_cursor.sroa.0.1.i2487387, %_43.i82.i.2, !dbg !13738
  %_47.not.i83.i.2 = icmp ult i64 %492, %_92.i, !dbg !13739
  %493 = select i1 %_47.not.i83.i.2, i64 0, i64 %_92.i, !dbg !13739
  %spec.select.i84.i.2 = sub nuw i64 %492, %493, !dbg !13739
  %_51.i85.i.2 = mul i64 %spec.select.i84.i.2, %width.i59.i, !dbg !13741
  %_50.i86.i.2 = add i64 %_51.i85.i.2, 2, !dbg !13741
  %_53.i88.i.2 = icmp ult i64 %_50.i86.i.2, %_149.1.i94.i.pre, !dbg !13742
  br i1 %_53.i88.i.2, label %bb21.i90.i.2, label %panic1.i89.i, !dbg !13742

bb21.i90.i.2:                                     ; preds = %bb17.i79.i.2
  %494 = getelementptr inbounds nuw float, ptr %_147.0.i91.i, i64 %_50.i86.i.2, !dbg !13742
  %_49.i92.i.2 = load float, ptr %494, align 4, !dbg !13742, !noalias !13725, !noundef !12
  store float %_49.i92.i.2, ptr %iter.i47.i.sroa.0.0.ptr7373.2, align 4, !dbg !13743, !noalias !13725
  %495 = icmp eq i64 %width.i59.i, 3, !dbg !13561
  br i1 %495, label %bb16.i93.i.loopexit, label %bb39.i73.i.3, !dbg !13561

bb39.i73.i.3:                                     ; preds = %bb21.i90.i.2
  %exitcond8374.3.not = icmp eq i64 %_145.1.i76.i, 3, !dbg !13569
  br i1 %exitcond8374.3.not, label %panic.i78.i, label %bb17.i79.i.3, !dbg !13569

bb17.i79.i.3:                                     ; preds = %bb39.i73.i.3
  %496 = getelementptr inbounds nuw i8, ptr %_145.0.i80.i, i64 44, !dbg !13569
  %_44.i81.i.3 = load i32, ptr %496, align 4, !dbg !13569, !noalias !13725, !noundef !12
  %_43.i82.i.3 = zext i32 %_44.i81.i.3 to i64, !dbg !13569
  %497 = add i64 %ring_cursor.sroa.0.1.i2487387, %_43.i82.i.3, !dbg !13738
  %_47.not.i83.i.3 = icmp ult i64 %497, %_92.i, !dbg !13739
  %498 = select i1 %_47.not.i83.i.3, i64 0, i64 %_92.i, !dbg !13739
  %spec.select.i84.i.3 = sub nuw i64 %497, %498, !dbg !13739
  %_51.i85.i.3 = mul i64 %spec.select.i84.i.3, %width.i59.i, !dbg !13741
  %_50.i86.i.3 = add i64 %_51.i85.i.3, 3, !dbg !13741
  %_53.i88.i.3 = icmp ult i64 %_50.i86.i.3, %_149.1.i94.i.pre, !dbg !13742
  br i1 %_53.i88.i.3, label %bb21.i90.i.3, label %panic1.i89.i, !dbg !13742

bb21.i90.i.3:                                     ; preds = %bb17.i79.i.3
  %499 = getelementptr inbounds nuw float, ptr %_147.0.i91.i, i64 %_50.i86.i.3, !dbg !13742
  %_49.i92.i.3 = load float, ptr %499, align 4, !dbg !13742, !noalias !13725, !noundef !12
  store float %_49.i92.i.3, ptr %iter.i47.i.sroa.0.0.ptr7373.3, align 4, !dbg !13743, !noalias !13725
  %500 = icmp eq i64 %width.i59.i, 4, !dbg !13561
  br i1 %500, label %bb16.i93.i.loopexit, label %bb39.i73.i.4, !dbg !13561

bb39.i73.i.4:                                     ; preds = %bb21.i90.i.3
  %exitcond8374.4.not = icmp eq i64 %_145.1.i76.i, 4, !dbg !13569
  br i1 %exitcond8374.4.not, label %panic.i78.i, label %bb17.i79.i.4, !dbg !13569

bb17.i79.i.4:                                     ; preds = %bb39.i73.i.4
  %501 = getelementptr inbounds nuw i8, ptr %_145.0.i80.i, i64 56, !dbg !13569
  %_44.i81.i.4 = load i32, ptr %501, align 4, !dbg !13569, !noalias !13725, !noundef !12
  %_43.i82.i.4 = zext i32 %_44.i81.i.4 to i64, !dbg !13569
  %502 = add i64 %ring_cursor.sroa.0.1.i2487387, %_43.i82.i.4, !dbg !13738
  %_47.not.i83.i.4 = icmp ult i64 %502, %_92.i, !dbg !13739
  %503 = select i1 %_47.not.i83.i.4, i64 0, i64 %_92.i, !dbg !13739
  %spec.select.i84.i.4 = sub nuw i64 %502, %503, !dbg !13739
  %_51.i85.i.4 = mul i64 %spec.select.i84.i.4, %width.i59.i, !dbg !13741
  %_50.i86.i.4 = add i64 %_51.i85.i.4, 4, !dbg !13741
  %_53.i88.i.4 = icmp ult i64 %_50.i86.i.4, %_149.1.i94.i.pre, !dbg !13742
  br i1 %_53.i88.i.4, label %bb21.i90.i.4, label %panic1.i89.i, !dbg !13742

bb21.i90.i.4:                                     ; preds = %bb17.i79.i.4
  %504 = getelementptr inbounds nuw float, ptr %_147.0.i91.i, i64 %_50.i86.i.4, !dbg !13742
  %_49.i92.i.4 = load float, ptr %504, align 4, !dbg !13742, !noalias !13725, !noundef !12
  store float %_49.i92.i.4, ptr %iter.i47.i.sroa.0.0.ptr7373.4, align 4, !dbg !13743, !noalias !13725
  %505 = icmp eq i64 %width.i59.i, 5, !dbg !13561
  br i1 %505, label %bb16.i93.i.loopexit, label %bb39.i73.i.5, !dbg !13561

bb39.i73.i.5:                                     ; preds = %bb21.i90.i.4
  %exitcond8374.5.not = icmp eq i64 %_145.1.i76.i, 5, !dbg !13569
  br i1 %exitcond8374.5.not, label %panic.i78.i, label %bb17.i79.i.5, !dbg !13569

bb17.i79.i.5:                                     ; preds = %bb39.i73.i.5
  %506 = getelementptr inbounds nuw i8, ptr %_145.0.i80.i, i64 68, !dbg !13569
  %_44.i81.i.5 = load i32, ptr %506, align 4, !dbg !13569, !noalias !13725, !noundef !12
  %_43.i82.i.5 = zext i32 %_44.i81.i.5 to i64, !dbg !13569
  %507 = add i64 %ring_cursor.sroa.0.1.i2487387, %_43.i82.i.5, !dbg !13738
  %_47.not.i83.i.5 = icmp ult i64 %507, %_92.i, !dbg !13739
  %508 = select i1 %_47.not.i83.i.5, i64 0, i64 %_92.i, !dbg !13739
  %spec.select.i84.i.5 = sub nuw i64 %507, %508, !dbg !13739
  %_51.i85.i.5 = mul i64 %spec.select.i84.i.5, %width.i59.i, !dbg !13741
  %_50.i86.i.5 = add i64 %_51.i85.i.5, 5, !dbg !13741
  %_53.i88.i.5 = icmp ult i64 %_50.i86.i.5, %_149.1.i94.i.pre, !dbg !13742
  br i1 %_53.i88.i.5, label %bb21.i90.i.5, label %panic1.i89.i, !dbg !13742

bb21.i90.i.5:                                     ; preds = %bb17.i79.i.5
  %509 = getelementptr inbounds nuw float, ptr %_147.0.i91.i, i64 %_50.i86.i.5, !dbg !13742
  %_49.i92.i.5 = load float, ptr %509, align 4, !dbg !13742, !noalias !13725, !noundef !12
  store float %_49.i92.i.5, ptr %iter.i47.i.sroa.0.0.ptr7373.5, align 4, !dbg !13743, !noalias !13725
  %510 = icmp eq i64 %width.i59.i, 6, !dbg !13561
  br i1 %510, label %bb16.i93.i.loopexit, label %bb39.i73.i.6, !dbg !13561

bb39.i73.i.6:                                     ; preds = %bb21.i90.i.5
  %exitcond8374.6.not = icmp eq i64 %_145.1.i76.i, 6, !dbg !13569
  br i1 %exitcond8374.6.not, label %panic.i78.i, label %bb17.i79.i.6, !dbg !13569

bb17.i79.i.6:                                     ; preds = %bb39.i73.i.6
  %511 = getelementptr inbounds nuw i8, ptr %_145.0.i80.i, i64 80, !dbg !13569
  %_44.i81.i.6 = load i32, ptr %511, align 4, !dbg !13569, !noalias !13725, !noundef !12
  %_43.i82.i.6 = zext i32 %_44.i81.i.6 to i64, !dbg !13569
  %512 = add i64 %ring_cursor.sroa.0.1.i2487387, %_43.i82.i.6, !dbg !13738
  %_47.not.i83.i.6 = icmp ult i64 %512, %_92.i, !dbg !13739
  %513 = select i1 %_47.not.i83.i.6, i64 0, i64 %_92.i, !dbg !13739
  %spec.select.i84.i.6 = sub nuw i64 %512, %513, !dbg !13739
  %_51.i85.i.6 = mul i64 %spec.select.i84.i.6, %width.i59.i, !dbg !13741
  %_50.i86.i.6 = add i64 %_51.i85.i.6, 6, !dbg !13741
  %_53.i88.i.6 = icmp ult i64 %_50.i86.i.6, %_149.1.i94.i.pre, !dbg !13742
  br i1 %_53.i88.i.6, label %bb21.i90.i.6, label %panic1.i89.i, !dbg !13742

bb21.i90.i.6:                                     ; preds = %bb17.i79.i.6
  %514 = getelementptr inbounds nuw float, ptr %_147.0.i91.i, i64 %_50.i86.i.6, !dbg !13742
  %_49.i92.i.6 = load float, ptr %514, align 4, !dbg !13742, !noalias !13725, !noundef !12
  store float %_49.i92.i.6, ptr %iter.i47.i.sroa.0.0.ptr7373.6, align 4, !dbg !13743, !noalias !13725
  %515 = icmp eq i64 %width.i59.i, 7, !dbg !13561
  br i1 %515, label %bb16.i93.i.loopexit, label %bb39.i73.i.7, !dbg !13561

bb39.i73.i.7:                                     ; preds = %bb21.i90.i.6
  %exitcond8374.7.not = icmp eq i64 %_145.1.i76.i, 7, !dbg !13569
  br i1 %exitcond8374.7.not, label %panic.i78.i, label %bb17.i79.i.7, !dbg !13569

bb17.i79.i.7:                                     ; preds = %bb39.i73.i.7
  %516 = getelementptr inbounds nuw i8, ptr %_145.0.i80.i, i64 92, !dbg !13569
  %_44.i81.i.7 = load i32, ptr %516, align 4, !dbg !13569, !noalias !13725, !noundef !12
  %_43.i82.i.7 = zext i32 %_44.i81.i.7 to i64, !dbg !13569
  %517 = add i64 %ring_cursor.sroa.0.1.i2487387, %_43.i82.i.7, !dbg !13738
  %_47.not.i83.i.7 = icmp ult i64 %517, %_92.i, !dbg !13739
  %518 = select i1 %_47.not.i83.i.7, i64 0, i64 %_92.i, !dbg !13739
  %spec.select.i84.i.7 = sub nuw i64 %517, %518, !dbg !13739
  %_51.i85.i.7 = mul i64 %spec.select.i84.i.7, %width.i59.i, !dbg !13741
  %_50.i86.i.7 = add i64 %_51.i85.i.7, 7, !dbg !13741
  %_53.i88.i.7 = icmp ult i64 %_50.i86.i.7, %_149.1.i94.i.pre, !dbg !13742
  br i1 %_53.i88.i.7, label %bb21.i90.i.7, label %panic1.i89.i, !dbg !13742

bb21.i90.i.7:                                     ; preds = %bb17.i79.i.7
  %519 = getelementptr inbounds nuw float, ptr %_147.0.i91.i, i64 %_50.i86.i.7, !dbg !13742
  %_49.i92.i.7 = load float, ptr %519, align 4, !dbg !13742, !noalias !13725, !noundef !12
  store float %_49.i92.i.7, ptr %iter.i47.i.sroa.0.0.ptr7373.7, align 4, !dbg !13743, !noalias !13725
  br label %bb16.i93.i.loopexit, !dbg !13561

panic1.i89.i:                                     ; preds = %bb17.i79.i.7, %bb17.i79.i.6, %bb17.i79.i.5, %bb17.i79.i.4, %bb17.i79.i.3, %bb17.i79.i.2, %bb17.i79.i.1, %bb17.i79.i
  %_50.i86.i.lcssa.ph = phi i64 [ %_50.i86.i.7, %bb17.i79.i.7 ], [ %_50.i86.i.6, %bb17.i79.i.6 ], [ %_50.i86.i.5, %bb17.i79.i.5 ], [ %_50.i86.i.4, %bb17.i79.i.4 ], [ %_50.i86.i.3, %bb17.i79.i.3 ], [ %_50.i86.i.2, %bb17.i79.i.2 ], [ %_50.i86.i.1, %bb17.i79.i.1 ], [ %_51.i85.i, %bb17.i79.i ]
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i86.i.lcssa.ph, i64 noundef %_149.1.i94.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aca25255cb99dc5ba482e692667f5878) #31, !dbg !13742, !noalias !13725
  unreachable, !dbg !13742

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2818: ; preds = %bb49.i103.i
  %_150.0.i104.i = load ptr, ptr %156, align 8, !dbg !13716, !alias.scope !13399, !noalias !13400, !nonnull !12, !noundef !12
  %_127.i106.i = getelementptr inbounds nuw float, ptr %_150.0.i104.i, i64 %_76.i101.i, !dbg !13744
  %lanes.i2414.sroa.0.0.copyload = load <8 x float>, ptr %_127.i106.i, align 4, !dbg !13749, !alias.scope !13753, !noalias !13757
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_127.i106.i, ptr noundef nonnull align 4 dereferenceable(32) %_151.i, i64 32, i1 false), !dbg !13759
  %520 = fmul <8 x float> %480, %lanes.i2414.sroa.0.0.copyload, !dbg !13765
  %521 = select <8 x i1> %158, <8 x float> %lanes.i2414.sroa.0.0.copyload, <8 x float> %520, !dbg !13770
  store <8 x float> %521, ptr %_151.i, align 4, !dbg !13775, !alias.scope !13780, !noalias !13784
  %_152.i = icmp samesign ugt i64 %base.i250, %right_io.1, !dbg !13788
  br i1 %_152.i, label %bb63.i, label %bb64.i, !dbg !13788, !prof !905

bb61.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit2462
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i250, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f473a90cd9861be1576d1010d795a4d4) #31, !dbg !13792, !noalias !13793
  unreachable, !dbg !13792

bb64.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2818
  %_155.i = sub nuw nsw i64 %right_io.1, %base.i250, !dbg !13794
  %_159.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %base.i250, !dbg !13795
  %_8.i2408 = icmp samesign ugt i64 %_155.i, 7, !dbg !13800
  br i1 %_8.i2408, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit2412, label %bb2.i2409, !dbg !13800, !prof !1076

bb2.i2409:                                        ; preds = %bb64.i
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_155.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #31, !dbg !13805, !noalias !13806
  unreachable, !dbg !13805

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit2412: ; preds = %bb64.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13810), !dbg !13813
  %width.i.i = load i64, ptr %159, align 8, !dbg !13814, !alias.scope !13816, !noalias !13817, !noundef !12
  %522 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %434, <8 x float> %limit_right.i213.sroa.0.0.copyload39719503, i8 30), !dbg !13826
  %523 = fdiv <8 x float> %limit_right.i213.sroa.0.0.copyload39719503, %434, !dbg !13832
  %524 = bitcast <8 x float> %522 to <8 x i32>, !dbg !13837
  %525 = icmp slt <8 x i32> %524, zeroinitializer, !dbg !13841
  %526 = select <8 x i1> %525, <8 x float> %523, <8 x float> splat (float 1.000000e+00), !dbg !13841
  %_144.1.i.i = load i64, ptr %160, align 8, !dbg !13843, !alias.scope !13816, !noalias !13817, !noundef !12
  %_22.i.i253 = mul i64 %width.i.i, %ring_cursor.sroa.0.1.i2487387, !dbg !13844
  %_92.i.i = icmp ugt i64 %_22.i.i253, %_144.1.i.i, !dbg !13845
  br i1 %_92.i.i, label %bb37.i.i, label %bb38.i.i, !dbg !13845, !prof !905

bb38.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit2412
  %_95.i.i = sub nuw i64 %_144.1.i.i, %_22.i.i253, !dbg !13848
  %_8.i2810 = icmp samesign ugt i64 %_95.i.i, 7, !dbg !13849
  br i1 %_8.i2810, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2813, label %bb2.i2811, !dbg !13849, !prof !1076

bb2.i2811:                                        ; preds = %bb38.i.i
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_95.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #31, !dbg !13854, !noalias !13855
  unreachable, !dbg !13854

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2813: ; preds = %bb38.i.i
  %_144.0.i.i = load ptr, ptr %161, align 8, !dbg !13843, !alias.scope !13816, !noalias !13817, !nonnull !12, !noundef !12
  %_99.i.i = getelementptr inbounds nuw float, ptr %_144.0.i.i, i64 %_22.i.i253, !dbg !13859
  store <8 x float> %526, ptr %_99.i.i, align 4, !dbg !13861, !alias.scope !13865, !noalias !13869
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13871), !dbg !13874
  %width.i = load i64, ptr %159, align 8, !dbg !13875, !alias.scope !13871, !noalias !13877, !noundef !12
  %527 = icmp eq i64 %width.i, 0, !dbg !13879
  br i1 %527, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit, label %bb32.i451.lr.ph, !dbg !13879

bb32.i451.lr.ph:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2813
  %_112.1.i = load i64, ptr %162, align 8, !alias.scope !13871, !noalias !13877, !noundef !12
  %_112.0.i = load ptr, ptr %163, align 8, !nonnull !12
  %528 = add i64 %ring_cursor.sroa.0.1.i2487387, 1
  %_23.not.i = icmp ult i64 %528, %_92.i
  %529 = select i1 %_23.not.i, i64 0, i64 %_92.i
  %start1.sroa.0.0.i = sub nuw i64 %528, %529
  %_114.1.i = load i64, ptr %160, align 8
  %_114.0.i = load ptr, ptr %161, align 8, !nonnull !12
  %_116.1.i = load i64, ptr %164, align 8
  %_116.0.i = load ptr, ptr %165, align 8, !nonnull !12
  %_118.1.i = load i64, ptr %166, align 8
  %_118.0.i = load ptr, ptr %167, align 8, !nonnull !12
  %_45.i = mul i64 %width.i, %start1.sroa.0.0.i
  br label %bb32.i451, !dbg !13879

bb32.i451:                                        ; preds = %bb32.i451.lr.ph, %bb31.i472
  %iter.i448.sroa.10.07379 = phi i64 [ %width.i, %bb32.i451.lr.ph ], [ %530, %bb31.i472 ]
  %iter.i448.sroa.7.07378 = phi i64 [ 0, %bb32.i451.lr.ph ], [ %_9.0.i3228, %bb31.i472 ]
  %iter.i448.sroa.0.0.idx7377 = phi i64 [ 0, %bb32.i451.lr.ph ], [ %iter.i448.sroa.0.0.add, %bb31.i472 ]
  %iter.i448.sroa.0.0.ptr7380 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 %iter.i448.sroa.0.0.idx7377, !dbg !13881
  %530 = add i64 %iter.i448.sroa.10.07379, -1, !dbg !13881
  %_7.i.i3224 = icmp eq i64 %iter.i448.sroa.0.0.idx7377, 32, !dbg !13882
  br i1 %_7.i.i3224, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit, label %bb3.i453, !dbg !13886

bb3.i453:                                         ; preds = %bb32.i451
  %iter.i448.sroa.0.0.add = add nuw nsw i64 %iter.i448.sroa.0.0.idx7377, 4, !dbg !13887
  %_9.0.i3228 = add nuw nsw i64 %iter.i448.sroa.7.07378, 1, !dbg !13889
  %exitcond8380.not = icmp eq i64 %iter.i448.sroa.7.07378, %_112.1.i, !dbg !13890
  br i1 %exitcond8380.not, label %panic.i, label %bb5.i455, !dbg !13890

bb5.i455:                                         ; preds = %bb3.i453
  %531 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i, i64 %iter.i448.sroa.7.07378, !dbg !13890
  %shape.i = load i32, ptr %531, align 4, !dbg !13890, !noalias !13891, !noundef !12
  %532 = getelementptr inbounds nuw i8, ptr %531, i64 4, !dbg !13890
  %shape3.i = load i32, ptr %532, align 4, !dbg !13890, !noalias !13891, !noundef !12
  %window.i = zext i32 %shape.i to i64, !dbg !13892
  %_19.i = zext i32 %shape3.i to i64, !dbg !13893
  %533 = add i64 %ring_cursor.sroa.0.1.i2487387, %_19.i, !dbg !13894
  %_20.not.i = icmp ult i64 %533, %_92.i, !dbg !13895
  %534 = select i1 %_20.not.i, i64 0, i64 %_92.i, !dbg !13895
  %spec.select.i = sub nuw i64 %533, %534, !dbg !13895
  %_27.i456 = mul i64 %spec.select.i, %width.i, !dbg !13896
  %_26.i457 = add i64 %_27.i456, %iter.i448.sroa.7.07378, !dbg !13896
  %_30.i458 = icmp ult i64 %_26.i457, %_114.1.i, !dbg !13897
  br i1 %_30.i458, label %bb12.i, label %panic5.i, !dbg !13897

panic.i:                                          ; preds = %bb3.i453
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i, i64 noundef %_112.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_077370d5cece7380867993336836eb69) #31, !dbg !13890, !noalias !13891
  unreachable, !dbg !13890

bb12.i:                                           ; preds = %bb5.i455
  %535 = getelementptr inbounds nuw float, ptr %_114.0.i, i64 %_26.i457, !dbg !13897
  %536 = load float, ptr %535, align 4, !dbg !13897, !noalias !13891, !noundef !12
  %exitcond8381.not = icmp eq i64 %iter.i448.sroa.7.07378, %_116.1.i, !dbg !13898
  br i1 %exitcond8381.not, label %panic6.i, label %bb13.i, !dbg !13898

panic5.i:                                         ; preds = %bb5.i455
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i457, i64 noundef %_114.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5cae9ea88ed6362f62dbad10291edbd4) #31, !dbg !13897, !noalias !13891
  unreachable, !dbg !13897

bb13.i:                                           ; preds = %bb12.i
  %537 = getelementptr inbounds nuw i32, ptr %_116.0.i, i64 %iter.i448.sroa.7.07378, !dbg !13898
  %_32.i = load i32, ptr %537, align 4, !dbg !13898, !noalias !13891, !noundef !12
  %position.i460 = zext i32 %_32.i to i64, !dbg !13898
  %538 = icmp eq i32 %_32.i, 0, !dbg !13899
  br i1 %538, label %bb17.i, label %bb15.i, !dbg !13899

panic6.i:                                         ; preds = %bb12.i
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i, i64 noundef %_116.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9fdd00ed7abe4ccfd73f0d145a7b741b) #31, !dbg !13898, !noalias !13891
  unreachable, !dbg !13898

bb15.i:                                           ; preds = %bb13.i
  %_37.i = icmp ult i64 %iter.i448.sroa.7.07378, %_118.1.i, !dbg !13900
  br i1 %_37.i, label %bb16.i, label %panic7.i, !dbg !13900

bb17.i:                                           ; preds = %bb35.i484, %bb16.i, %bb13.i
  %newest.sroa.0.0.i = phi float [ %536, %bb13.i ], [ %_35.i461, %bb35.i484 ], [ %536, %bb16.i ], !dbg !13901
  %exitcond8382.not = icmp eq i64 %iter.i448.sroa.7.07378, %_118.1.i, !dbg !13902
  br i1 %exitcond8382.not, label %panic8.i, label %bb18.i, !dbg !13902

bb16.i:                                           ; preds = %bb15.i
  %539 = getelementptr inbounds nuw float, ptr %_118.0.i, i64 %iter.i448.sroa.7.07378, !dbg !13900
  %_35.i461 = load float, ptr %539, align 4, !dbg !13900, !noalias !13891, !noundef !12
  %_102.i462 = fcmp olt float %_35.i461, %536, !dbg !13903
  br i1 %_102.i462, label %bb35.i484, label %bb17.i, !dbg !13903

panic7.i:                                         ; preds = %bb15.i
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i448.sroa.7.07378, i64 noundef %_118.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_697dc8945f5e5b040d7e9a0b92cb249f) #31, !dbg !13900, !noalias !13891
  unreachable, !dbg !13900

bb35.i484:                                        ; preds = %bb16.i
  br label %bb17.i, !dbg !13905

bb18.i:                                           ; preds = %bb17.i
  %540 = getelementptr inbounds nuw float, ptr %_118.0.i, i64 %iter.i448.sroa.7.07378, !dbg !13902
  store float %newest.sroa.0.0.i, ptr %540, align 4, !dbg !13902, !noalias !13891
  %_42.i463 = add nuw nsw i64 %position.i460, 1, !dbg !13906
  %complete.i464 = icmp eq i64 %_42.i463, %window.i, !dbg !13906
  br i1 %complete.i464, label %bb22.i, label %bb20.i465, !dbg !13907

panic8.i:                                         ; preds = %bb17.i
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i, i64 noundef %_118.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_067dce244605df4a33956fbd4d726027) #31, !dbg !13902, !noalias !13891
  unreachable, !dbg !13902

bb20.i465:                                        ; preds = %bb18.i
  %_44.i = add i64 %iter.i448.sroa.7.07378, %_45.i, !dbg !13908
  %_47.i466 = icmp ult i64 %_44.i, %_114.1.i, !dbg !13909
  br i1 %_47.i466, label %bb30.i471, label %panic9.i, !dbg !13909

panic9.i:                                         ; preds = %bb20.i465
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i, i64 noundef %_114.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d5ea38731a1cb6311efef2f919234d06) #31, !dbg !13909, !noalias !13891
  unreachable, !dbg !13909

bb30.i471:                                        ; preds = %bb20.i465
  %541 = getelementptr inbounds nuw float, ptr %_114.0.i, i64 %_44.i, !dbg !13909
  %_43.i468 = load float, ptr %541, align 4, !dbg !13909, !noalias !13891, !noundef !12
  %_103.i469 = fcmp olt float %_43.i468, %newest.sroa.0.0.i, !dbg !13910
  %newest.sroa.0.1.i = select i1 %_103.i469, float %_43.i468, float %newest.sroa.0.0.i, !dbg !13910
  store float %newest.sroa.0.1.i, ptr %iter.i448.sroa.0.0.ptr7380, align 4, !dbg !13912, !noalias !13891
  %542 = trunc i64 %_42.i463 to i32, !dbg !13913
  br label %bb31.i472, !dbg !13914

bb31.i472:                                        ; preds = %bb25.i482, %bb30.i471
  %storemerge6233 = phi i32 [ %542, %bb30.i471 ], [ 0, %bb25.i482 ], !dbg !13915
  store i32 %storemerge6233, ptr %537, align 4, !dbg !13915, !noalias !13891
  %543 = icmp eq i64 %530, 0, !dbg !13879
  br i1 %543, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit, label %bb32.i451, !dbg !13879

bb22.i:                                           ; preds = %bb18.i
  store float %newest.sroa.0.0.i, ptr %iter.i448.sroa.0.0.ptr7380, align 4, !dbg !13912, !noalias !13891
  %544 = load float, ptr %535, align 4, !dbg !13916, !noalias !13891, !noundef !12
  br label %bb41.i477, !dbg !13917

bb41.i477:                                        ; preds = %bb22.i, %bb25.i482
  %iter2.sroa.0.0.i7376 = phi i64 [ 0, %bb22.i ], [ %_105.i478, %bb25.i482 ]
  %suffix.sroa.0.0.i7375 = phi float [ %544, %bb22.i ], [ %suffix.sroa.0.1.i, %bb25.i482 ]
  %end.sroa.0.1.i7374 = phi i64 [ %spec.select.i, %bb22.i ], [ %547, %bb25.i482 ]
  %_56.i = mul i64 %end.sroa.0.1.i7374, %width.i, !dbg !13920
  %_55.i = add i64 %_56.i, %iter.i448.sroa.7.07378, !dbg !13920
  %_59.i = icmp ult i64 %_55.i, %_114.1.i, !dbg !13921
  br i1 %_59.i, label %bb25.i482, label %panic13.i, !dbg !13921

panic13.i:                                        ; preds = %bb41.i477
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i, i64 noundef %_114.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a19fa7d9077791c9eadbe74fafea8de7) #31, !dbg !13921, !noalias !13891
  unreachable, !dbg !13921

bb25.i482:                                        ; preds = %bb41.i477
  %_105.i478 = add nuw nsw i64 %iter2.sroa.0.0.i7376, 1, !dbg !13922
  %545 = getelementptr inbounds nuw float, ptr %_114.0.i, i64 %_55.i, !dbg !13921
  %_54.i = load float, ptr %545, align 4, !dbg !13921, !noalias !13891, !noundef !12
  %_107.i480 = fcmp olt float %suffix.sroa.0.0.i7375, %_54.i, !dbg !13925
  %suffix.sroa.0.1.i = select i1 %_107.i480, float %suffix.sroa.0.0.i7375, float %_54.i, !dbg !13925
  store float %suffix.sroa.0.1.i, ptr %545, align 4, !dbg !13927, !noalias !13891
  %546 = icmp eq i64 %end.sroa.0.1.i7374, 0, !dbg !13928
  %spec.store.select.i483 = select i1 %546, i64 %_92.i, i64 %end.sroa.0.1.i7374, !dbg !13928
  %547 = add i64 %spec.store.select.i483, -1, !dbg !13929
  %exitcond8379.not = icmp eq i64 %_105.i478, %window.i, !dbg !13930
  br i1 %exitcond8379.not, label %bb31.i472, label %bb41.i477, !dbg !13917

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit: ; preds = %bb32.i451, %bb31.i472
  %lanes.i2398.sroa.0.0.copyload.pre = load <8 x float>, ptr %scratch.i, align 4, !dbg !13932, !alias.scope !13937, !noalias !13941
  br label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit, !dbg !13945

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit: ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2813
  %lanes.i2398.sroa.0.0.copyload = phi <8 x float> [ %lanes.i2398.sroa.0.0.copyload.pre, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit ], [ %lanes.i2423.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2813 ], !dbg !13932
  %548 = fmul <8 x float> %lanes.i2398.sroa.0.0.copyload, splat (float 1.638400e+04), !dbg !13946
  %549 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %548), !dbg !13951
  %550 = fmul <8 x float> %549, splat (float 0x3F10000000000000), !dbg !13956
  %551 = icmp eq i64 %width.i.i, 0, !dbg !13961
  %_149.1.i.i.pre = load i64, ptr %168, align 8, !dbg !13963, !alias.scope !13816, !noalias !13817
  br i1 %551, label %bb16.i.i, label %bb39.i.i.lr.ph, !dbg !13961

bb39.i.i.lr.ph:                                   ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit
  %_145.1.i.i = load i64, ptr %162, align 8, !alias.scope !13816, !noalias !13817, !noundef !12
  %_145.0.i.i = load ptr, ptr %163, align 8, !nonnull !12
  %_147.0.i.i = load ptr, ptr %169, align 8, !nonnull !12
  %exitcond8385.not = icmp eq i64 %_145.1.i.i, 0, !dbg !13964
  br i1 %exitcond8385.not, label %panic.i.i, label %bb17.i.i, !dbg !13964

bb37.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit2412
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i253, i64 noundef %_144.1.i.i, i64 noundef %_144.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9d2713d1692431af37d60082290049ed) #31, !dbg !13965, !noalias !13966
  unreachable, !dbg !13965

bb16.i.i.loopexit:                                ; preds = %bb21.i.i.7, %bb21.i.i.6, %bb21.i.i.5, %bb21.i.i.4, %bb21.i.i.3, %bb21.i.i.2, %bb21.i.i.1, %bb21.i.i
  %lanes.i2391.sroa.0.0.copyload.pre = load <8 x float>, ptr %scratch.i, align 4, !dbg !13967, !alias.scope !13972, !noalias !13976
  br label %bb16.i.i, !dbg !13980

bb16.i.i:                                         ; preds = %bb16.i.i.loopexit, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit
  %lanes.i2391.sroa.0.0.copyload = phi <8 x float> [ %lanes.i2391.sroa.0.0.copyload.pre, %bb16.i.i.loopexit ], [ %lanes.i2398.sroa.0.0.copyload, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit ], !dbg !13967
  %552 = fadd <8 x float> %550, %_58.i.i.sroa.0.0.copyload9517, !dbg !13981
  %553 = fsub <8 x float> %552, %lanes.i2391.sroa.0.0.copyload, !dbg !13986
  store <8 x float> %553, ptr %170, align 32, !dbg !13991
  %_109.i.i = icmp ugt i64 %_22.i.i253, %_149.1.i.i.pre, !dbg !13992
  br i1 %_109.i.i, label %bb42.i.i, label %bb43.i.i, !dbg !13992, !prof !905

bb43.i.i:                                         ; preds = %bb16.i.i
  %_112.i.i = sub nuw i64 %_149.1.i.i.pre, %_22.i.i253, !dbg !13995
  %_8.i2805 = icmp samesign ugt i64 %_112.i.i, 7, !dbg !13996
  br i1 %_8.i2805, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2808, label %bb2.i2806, !dbg !13996, !prof !1076

bb2.i2806:                                        ; preds = %bb43.i.i
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_112.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #31, !dbg !14001, !noalias !14002
  unreachable, !dbg !14001

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2808: ; preds = %bb43.i.i
  %_149.0.i.i = load ptr, ptr %169, align 8, !dbg !13963, !alias.scope !13816, !noalias !13817, !nonnull !12, !noundef !12
  %_116.i.i = getelementptr inbounds nuw float, ptr %_149.0.i.i, i64 %_22.i.i253, !dbg !14006
  store <8 x float> %550, ptr %_116.i.i, align 4, !dbg !14008, !alias.scope !14012, !noalias !14016
  %_64.i.i.sroa.0.0.copyload = load <8 x float>, ptr %171, align 32, !dbg !14018
  %_68.i.i.sroa.0.0.copyload = load <8 x float>, ptr %172, align 32, !dbg !14019
  %554 = fdiv <8 x float> %553, %_64.i.i.sroa.0.0.copyload, !dbg !14020
  %555 = fsub <8 x float> splat (float 1.000000e+00), %554, !dbg !14025
  %556 = fsub <8 x float> %555, %_68.i.i.sroa.0.0.copyload, !dbg !14030
  %557 = fmul <8 x float> %release_right.i212.sroa.0.0.copyload9509, %556, !dbg !14035
  %558 = fadd <8 x float> %_68.i.i.sroa.0.0.copyload, %557, !dbg !14040
  %559 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %555, <8 x float> %558), !dbg !14044
  %560 = bitcast <8 x float> %559 to <8 x i32>, !dbg !14049
  %561 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %559), !dbg !14055
  %562 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %561, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !14057
  %563 = bitcast <8 x float> %562 to <8 x i32>, !dbg !14063
  %564 = xor <8 x i32> %563, splat (i32 -1), !dbg !14069
  %565 = and <8 x i32> %564, %560, !dbg !14071
  %566 = bitcast <8 x i32> %565 to <8 x float>, !dbg !14075
  store <8 x i32> %565, ptr %172, align 32, !dbg !14076
  %567 = fsub <8 x float> splat (float 1.000000e+00), %566, !dbg !14077
  %_150.1.i.i = load i64, ptr %173, align 8, !dbg !14082, !alias.scope !13816, !noalias !13817, !noundef !12
  %_76.i.i = mul i64 %width.i.i, %main_cursor.sroa.0.1.i2497388, !dbg !14083
  %_120.i.i = icmp ugt i64 %_76.i.i, %_150.1.i.i, !dbg !14084
  br i1 %_120.i.i, label %bb48.i.i, label %bb49.i.i, !dbg !14084, !prof !905

bb42.i.i:                                         ; preds = %bb16.i.i
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i253, i64 noundef %_149.1.i.i.pre, i64 noundef %_149.1.i.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8002ed69501742f3ea2ea25eb68cd581) #31, !dbg !14087, !noalias !14088
  unreachable, !dbg !14087

bb49.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2808
  %_123.i.i = sub nuw i64 %_150.1.i.i, %_76.i.i, !dbg !14089
  %_8.i2385 = icmp samesign ugt i64 %_123.i.i, 7, !dbg !14090
  br i1 %_8.i2385, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2798, label %bb2.i2386, !dbg !14090, !prof !1076

bb2.i2386:                                        ; preds = %bb49.i.i
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_123.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #31, !dbg !14095, !noalias !14096
  unreachable, !dbg !14095

bb48.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2808
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i.i, i64 noundef %_150.1.i.i, i64 noundef %_150.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a07d19b424a92543209d516ece5bdf2a) #31, !dbg !14100, !noalias !14088
  unreachable, !dbg !14100

bb17.i.i:                                         ; preds = %bb39.i.i.lr.ph
  %568 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 8, !dbg !13964
  %_44.i.i255 = load i32, ptr %568, align 4, !dbg !13964, !noalias !14088, !noundef !12
  %_43.i.i256 = zext i32 %_44.i.i255 to i64, !dbg !13964
  %569 = add i64 %ring_cursor.sroa.0.1.i2487387, %_43.i.i256, !dbg !14101
  %_47.not.i.i = icmp ult i64 %569, %_92.i, !dbg !14102
  %570 = select i1 %_47.not.i.i, i64 0, i64 %_92.i, !dbg !14102
  %spec.select.i.i = sub nuw i64 %569, %570, !dbg !14102
  %_51.i.i = mul i64 %spec.select.i.i, %width.i.i, !dbg !14103
  %_53.i.i257 = icmp ult i64 %_51.i.i, %_149.1.i.i.pre, !dbg !14104
  br i1 %_53.i.i257, label %bb21.i.i, label %panic1.i.i, !dbg !14104

panic.i.i:                                        ; preds = %bb39.i.i.7, %bb39.i.i.6, %bb39.i.i.5, %bb39.i.i.4, %bb39.i.i.3, %bb39.i.i.2, %bb39.i.i.1, %bb39.i.i.lr.ph
  %_145.1.i.i.lcssa.ph = phi i64 [ 7, %bb39.i.i.7 ], [ 6, %bb39.i.i.6 ], [ 5, %bb39.i.i.5 ], [ 4, %bb39.i.i.4 ], [ 3, %bb39.i.i.3 ], [ 2, %bb39.i.i.2 ], [ 1, %bb39.i.i.1 ], [ 0, %bb39.i.i.lr.ph ]
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i.i.lcssa.ph, i64 noundef %_145.1.i.i.lcssa.ph, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_891dd683d7367e0a1ec5a22f9c2a2aa8) #31, !dbg !13964, !noalias !14088
  unreachable, !dbg !13964

bb21.i.i:                                         ; preds = %bb17.i.i
  %571 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_51.i.i, !dbg !14104
  %_49.i.i258 = load float, ptr %571, align 4, !dbg !14104, !noalias !14088, !noundef !12
  store float %_49.i.i258, ptr %scratch.i, align 4, !dbg !14105, !noalias !14088
  %572 = icmp eq i64 %width.i.i, 1, !dbg !13961
  br i1 %572, label %bb16.i.i.loopexit, label %bb39.i.i.1, !dbg !13961

bb39.i.i.1:                                       ; preds = %bb21.i.i
  %exitcond8385.1.not = icmp eq i64 %_145.1.i.i, 1, !dbg !13964
  br i1 %exitcond8385.1.not, label %panic.i.i, label %bb17.i.i.1, !dbg !13964

bb17.i.i.1:                                       ; preds = %bb39.i.i.1
  %573 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 20, !dbg !13964
  %_44.i.i255.1 = load i32, ptr %573, align 4, !dbg !13964, !noalias !14088, !noundef !12
  %_43.i.i256.1 = zext i32 %_44.i.i255.1 to i64, !dbg !13964
  %574 = add i64 %ring_cursor.sroa.0.1.i2487387, %_43.i.i256.1, !dbg !14101
  %_47.not.i.i.1 = icmp ult i64 %574, %_92.i, !dbg !14102
  %575 = select i1 %_47.not.i.i.1, i64 0, i64 %_92.i, !dbg !14102
  %spec.select.i.i.1 = sub nuw i64 %574, %575, !dbg !14102
  %_51.i.i.1 = mul i64 %spec.select.i.i.1, %width.i.i, !dbg !14103
  %_50.i.i.1 = add i64 %_51.i.i.1, 1, !dbg !14103
  %_53.i.i257.1 = icmp ult i64 %_50.i.i.1, %_149.1.i.i.pre, !dbg !14104
  br i1 %_53.i.i257.1, label %bb21.i.i.1, label %panic1.i.i, !dbg !14104

bb21.i.i.1:                                       ; preds = %bb17.i.i.1
  %576 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.1, !dbg !14104
  %_49.i.i258.1 = load float, ptr %576, align 4, !dbg !14104, !noalias !14088, !noundef !12
  store float %_49.i.i258.1, ptr %iter.i.i.sroa.0.0.ptr7384.1, align 4, !dbg !14105, !noalias !14088
  %577 = icmp eq i64 %width.i.i, 2, !dbg !13961
  br i1 %577, label %bb16.i.i.loopexit, label %bb39.i.i.2, !dbg !13961

bb39.i.i.2:                                       ; preds = %bb21.i.i.1
  %exitcond8385.2.not = icmp eq i64 %_145.1.i.i, 2, !dbg !13964
  br i1 %exitcond8385.2.not, label %panic.i.i, label %bb17.i.i.2, !dbg !13964

bb17.i.i.2:                                       ; preds = %bb39.i.i.2
  %578 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 32, !dbg !13964
  %_44.i.i255.2 = load i32, ptr %578, align 4, !dbg !13964, !noalias !14088, !noundef !12
  %_43.i.i256.2 = zext i32 %_44.i.i255.2 to i64, !dbg !13964
  %579 = add i64 %ring_cursor.sroa.0.1.i2487387, %_43.i.i256.2, !dbg !14101
  %_47.not.i.i.2 = icmp ult i64 %579, %_92.i, !dbg !14102
  %580 = select i1 %_47.not.i.i.2, i64 0, i64 %_92.i, !dbg !14102
  %spec.select.i.i.2 = sub nuw i64 %579, %580, !dbg !14102
  %_51.i.i.2 = mul i64 %spec.select.i.i.2, %width.i.i, !dbg !14103
  %_50.i.i.2 = add i64 %_51.i.i.2, 2, !dbg !14103
  %_53.i.i257.2 = icmp ult i64 %_50.i.i.2, %_149.1.i.i.pre, !dbg !14104
  br i1 %_53.i.i257.2, label %bb21.i.i.2, label %panic1.i.i, !dbg !14104

bb21.i.i.2:                                       ; preds = %bb17.i.i.2
  %581 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.2, !dbg !14104
  %_49.i.i258.2 = load float, ptr %581, align 4, !dbg !14104, !noalias !14088, !noundef !12
  store float %_49.i.i258.2, ptr %iter.i.i.sroa.0.0.ptr7384.2, align 4, !dbg !14105, !noalias !14088
  %582 = icmp eq i64 %width.i.i, 3, !dbg !13961
  br i1 %582, label %bb16.i.i.loopexit, label %bb39.i.i.3, !dbg !13961

bb39.i.i.3:                                       ; preds = %bb21.i.i.2
  %exitcond8385.3.not = icmp eq i64 %_145.1.i.i, 3, !dbg !13964
  br i1 %exitcond8385.3.not, label %panic.i.i, label %bb17.i.i.3, !dbg !13964

bb17.i.i.3:                                       ; preds = %bb39.i.i.3
  %583 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 44, !dbg !13964
  %_44.i.i255.3 = load i32, ptr %583, align 4, !dbg !13964, !noalias !14088, !noundef !12
  %_43.i.i256.3 = zext i32 %_44.i.i255.3 to i64, !dbg !13964
  %584 = add i64 %ring_cursor.sroa.0.1.i2487387, %_43.i.i256.3, !dbg !14101
  %_47.not.i.i.3 = icmp ult i64 %584, %_92.i, !dbg !14102
  %585 = select i1 %_47.not.i.i.3, i64 0, i64 %_92.i, !dbg !14102
  %spec.select.i.i.3 = sub nuw i64 %584, %585, !dbg !14102
  %_51.i.i.3 = mul i64 %spec.select.i.i.3, %width.i.i, !dbg !14103
  %_50.i.i.3 = add i64 %_51.i.i.3, 3, !dbg !14103
  %_53.i.i257.3 = icmp ult i64 %_50.i.i.3, %_149.1.i.i.pre, !dbg !14104
  br i1 %_53.i.i257.3, label %bb21.i.i.3, label %panic1.i.i, !dbg !14104

bb21.i.i.3:                                       ; preds = %bb17.i.i.3
  %586 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.3, !dbg !14104
  %_49.i.i258.3 = load float, ptr %586, align 4, !dbg !14104, !noalias !14088, !noundef !12
  store float %_49.i.i258.3, ptr %iter.i.i.sroa.0.0.ptr7384.3, align 4, !dbg !14105, !noalias !14088
  %587 = icmp eq i64 %width.i.i, 4, !dbg !13961
  br i1 %587, label %bb16.i.i.loopexit, label %bb39.i.i.4, !dbg !13961

bb39.i.i.4:                                       ; preds = %bb21.i.i.3
  %exitcond8385.4.not = icmp eq i64 %_145.1.i.i, 4, !dbg !13964
  br i1 %exitcond8385.4.not, label %panic.i.i, label %bb17.i.i.4, !dbg !13964

bb17.i.i.4:                                       ; preds = %bb39.i.i.4
  %588 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 56, !dbg !13964
  %_44.i.i255.4 = load i32, ptr %588, align 4, !dbg !13964, !noalias !14088, !noundef !12
  %_43.i.i256.4 = zext i32 %_44.i.i255.4 to i64, !dbg !13964
  %589 = add i64 %ring_cursor.sroa.0.1.i2487387, %_43.i.i256.4, !dbg !14101
  %_47.not.i.i.4 = icmp ult i64 %589, %_92.i, !dbg !14102
  %590 = select i1 %_47.not.i.i.4, i64 0, i64 %_92.i, !dbg !14102
  %spec.select.i.i.4 = sub nuw i64 %589, %590, !dbg !14102
  %_51.i.i.4 = mul i64 %spec.select.i.i.4, %width.i.i, !dbg !14103
  %_50.i.i.4 = add i64 %_51.i.i.4, 4, !dbg !14103
  %_53.i.i257.4 = icmp ult i64 %_50.i.i.4, %_149.1.i.i.pre, !dbg !14104
  br i1 %_53.i.i257.4, label %bb21.i.i.4, label %panic1.i.i, !dbg !14104

bb21.i.i.4:                                       ; preds = %bb17.i.i.4
  %591 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.4, !dbg !14104
  %_49.i.i258.4 = load float, ptr %591, align 4, !dbg !14104, !noalias !14088, !noundef !12
  store float %_49.i.i258.4, ptr %iter.i.i.sroa.0.0.ptr7384.4, align 4, !dbg !14105, !noalias !14088
  %592 = icmp eq i64 %width.i.i, 5, !dbg !13961
  br i1 %592, label %bb16.i.i.loopexit, label %bb39.i.i.5, !dbg !13961

bb39.i.i.5:                                       ; preds = %bb21.i.i.4
  %exitcond8385.5.not = icmp eq i64 %_145.1.i.i, 5, !dbg !13964
  br i1 %exitcond8385.5.not, label %panic.i.i, label %bb17.i.i.5, !dbg !13964

bb17.i.i.5:                                       ; preds = %bb39.i.i.5
  %593 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 68, !dbg !13964
  %_44.i.i255.5 = load i32, ptr %593, align 4, !dbg !13964, !noalias !14088, !noundef !12
  %_43.i.i256.5 = zext i32 %_44.i.i255.5 to i64, !dbg !13964
  %594 = add i64 %ring_cursor.sroa.0.1.i2487387, %_43.i.i256.5, !dbg !14101
  %_47.not.i.i.5 = icmp ult i64 %594, %_92.i, !dbg !14102
  %595 = select i1 %_47.not.i.i.5, i64 0, i64 %_92.i, !dbg !14102
  %spec.select.i.i.5 = sub nuw i64 %594, %595, !dbg !14102
  %_51.i.i.5 = mul i64 %spec.select.i.i.5, %width.i.i, !dbg !14103
  %_50.i.i.5 = add i64 %_51.i.i.5, 5, !dbg !14103
  %_53.i.i257.5 = icmp ult i64 %_50.i.i.5, %_149.1.i.i.pre, !dbg !14104
  br i1 %_53.i.i257.5, label %bb21.i.i.5, label %panic1.i.i, !dbg !14104

bb21.i.i.5:                                       ; preds = %bb17.i.i.5
  %596 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.5, !dbg !14104
  %_49.i.i258.5 = load float, ptr %596, align 4, !dbg !14104, !noalias !14088, !noundef !12
  store float %_49.i.i258.5, ptr %iter.i.i.sroa.0.0.ptr7384.5, align 4, !dbg !14105, !noalias !14088
  %597 = icmp eq i64 %width.i.i, 6, !dbg !13961
  br i1 %597, label %bb16.i.i.loopexit, label %bb39.i.i.6, !dbg !13961

bb39.i.i.6:                                       ; preds = %bb21.i.i.5
  %exitcond8385.6.not = icmp eq i64 %_145.1.i.i, 6, !dbg !13964
  br i1 %exitcond8385.6.not, label %panic.i.i, label %bb17.i.i.6, !dbg !13964

bb17.i.i.6:                                       ; preds = %bb39.i.i.6
  %598 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 80, !dbg !13964
  %_44.i.i255.6 = load i32, ptr %598, align 4, !dbg !13964, !noalias !14088, !noundef !12
  %_43.i.i256.6 = zext i32 %_44.i.i255.6 to i64, !dbg !13964
  %599 = add i64 %ring_cursor.sroa.0.1.i2487387, %_43.i.i256.6, !dbg !14101
  %_47.not.i.i.6 = icmp ult i64 %599, %_92.i, !dbg !14102
  %600 = select i1 %_47.not.i.i.6, i64 0, i64 %_92.i, !dbg !14102
  %spec.select.i.i.6 = sub nuw i64 %599, %600, !dbg !14102
  %_51.i.i.6 = mul i64 %spec.select.i.i.6, %width.i.i, !dbg !14103
  %_50.i.i.6 = add i64 %_51.i.i.6, 6, !dbg !14103
  %_53.i.i257.6 = icmp ult i64 %_50.i.i.6, %_149.1.i.i.pre, !dbg !14104
  br i1 %_53.i.i257.6, label %bb21.i.i.6, label %panic1.i.i, !dbg !14104

bb21.i.i.6:                                       ; preds = %bb17.i.i.6
  %601 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.6, !dbg !14104
  %_49.i.i258.6 = load float, ptr %601, align 4, !dbg !14104, !noalias !14088, !noundef !12
  store float %_49.i.i258.6, ptr %iter.i.i.sroa.0.0.ptr7384.6, align 4, !dbg !14105, !noalias !14088
  %602 = icmp eq i64 %width.i.i, 7, !dbg !13961
  br i1 %602, label %bb16.i.i.loopexit, label %bb39.i.i.7, !dbg !13961

bb39.i.i.7:                                       ; preds = %bb21.i.i.6
  %exitcond8385.7.not = icmp eq i64 %_145.1.i.i, 7, !dbg !13964
  br i1 %exitcond8385.7.not, label %panic.i.i, label %bb17.i.i.7, !dbg !13964

bb17.i.i.7:                                       ; preds = %bb39.i.i.7
  %603 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 92, !dbg !13964
  %_44.i.i255.7 = load i32, ptr %603, align 4, !dbg !13964, !noalias !14088, !noundef !12
  %_43.i.i256.7 = zext i32 %_44.i.i255.7 to i64, !dbg !13964
  %604 = add i64 %ring_cursor.sroa.0.1.i2487387, %_43.i.i256.7, !dbg !14101
  %_47.not.i.i.7 = icmp ult i64 %604, %_92.i, !dbg !14102
  %605 = select i1 %_47.not.i.i.7, i64 0, i64 %_92.i, !dbg !14102
  %spec.select.i.i.7 = sub nuw i64 %604, %605, !dbg !14102
  %_51.i.i.7 = mul i64 %spec.select.i.i.7, %width.i.i, !dbg !14103
  %_50.i.i.7 = add i64 %_51.i.i.7, 7, !dbg !14103
  %_53.i.i257.7 = icmp ult i64 %_50.i.i.7, %_149.1.i.i.pre, !dbg !14104
  br i1 %_53.i.i257.7, label %bb21.i.i.7, label %panic1.i.i, !dbg !14104

bb21.i.i.7:                                       ; preds = %bb17.i.i.7
  %606 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.7, !dbg !14104
  %_49.i.i258.7 = load float, ptr %606, align 4, !dbg !14104, !noalias !14088, !noundef !12
  store float %_49.i.i258.7, ptr %iter.i.i.sroa.0.0.ptr7384.7, align 4, !dbg !14105, !noalias !14088
  br label %bb16.i.i.loopexit, !dbg !13961

panic1.i.i:                                       ; preds = %bb17.i.i.7, %bb17.i.i.6, %bb17.i.i.5, %bb17.i.i.4, %bb17.i.i.3, %bb17.i.i.2, %bb17.i.i.1, %bb17.i.i
  %_50.i.i.lcssa.ph = phi i64 [ %_50.i.i.7, %bb17.i.i.7 ], [ %_50.i.i.6, %bb17.i.i.6 ], [ %_50.i.i.5, %bb17.i.i.5 ], [ %_50.i.i.4, %bb17.i.i.4 ], [ %_50.i.i.3, %bb17.i.i.3 ], [ %_50.i.i.2, %bb17.i.i.2 ], [ %_50.i.i.1, %bb17.i.i.1 ], [ %_51.i.i, %bb17.i.i ]
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i.i.lcssa.ph, i64 noundef %_149.1.i.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aca25255cb99dc5ba482e692667f5878) #31, !dbg !14104, !noalias !14088
  unreachable, !dbg !14104

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2798: ; preds = %bb49.i.i
  %_150.0.i.i = load ptr, ptr %174, align 8, !dbg !14082, !alias.scope !13816, !noalias !13817, !nonnull !12, !noundef !12
  %_127.i.i = getelementptr inbounds nuw float, ptr %_150.0.i.i, i64 %_76.i.i, !dbg !14106
  %lanes.i2382.sroa.0.0.copyload = load <8 x float>, ptr %_127.i.i, align 4, !dbg !14108, !alias.scope !14112, !noalias !14116
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_127.i.i, ptr noundef nonnull align 4 dereferenceable(32) %_159.i, i64 32, i1 false), !dbg !14118
  %607 = fmul <8 x float> %567, %lanes.i2382.sroa.0.0.copyload, !dbg !14123
  %608 = select <8 x i1> %158, <8 x float> %lanes.i2382.sroa.0.0.copyload, <8 x float> %607, !dbg !14128
  store <8 x float> %608, ptr %_159.i, align 4, !dbg !14133, !alias.scope !14138, !noalias !14142
  %609 = add i64 %main_cursor.sroa.0.1.i2497388, 1, !dbg !14146
  %_107.i = load i64, ptr %175, align 8, !dbg !14147, !alias.scope !11709, !noalias !13391, !noundef !12
  %_105.i259 = icmp eq i64 %609, %_107.i, !dbg !14148
  %spec.store.select.i = select i1 %_105.i259, i64 0, i64 %609, !dbg !14148
  %610 = add i64 %ring_cursor.sroa.0.1.i2487387, 1, !dbg !14149
  %_108.i = icmp eq i64 %610, %_92.i, !dbg !14150
  %spec.store.select13.i = select i1 %_108.i, i64 0, i64 %610, !dbg !14150
  %exitcond8390.not = icmp eq i64 %395, %umax8389, !dbg !14151
  br i1 %exitcond8390.not, label %bb25.i.loopexit.loopexit, label %bb57.i, !dbg !13125

bb63.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2818
  store <8 x float> %431, ptr %128, align 32, !dbg !11903
  store <8 x float> %429, ptr %134, align 32, !dbg !11912
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i250, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_59d870179e725b921b323beec09137a0) #31, !dbg !14154, !noalias !13793
  unreachable, !dbg !14154

_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit: ; preds = %bb25.i.loopexit
  %611 = trunc i64 %main_cursor.sroa.0.1.i249.lcssa to i32, !dbg !14155
  %612 = trunc i64 %ring_cursor.sroa.0.1.i248.lcssa to i32, !dbg !14156
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, !dbg !14157

_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit, %bb14.i226
  %ring_cursor.sroa.0.0.i240.lcssa = phi i32 [ %_43.i234, %bb14.i226 ], [ %612, %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit ], !dbg !11839
  %main_cursor.sroa.0.0.i241.lcssa = phi i32 [ %_41.i, %bb14.i226 ], [ %611, %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit ], !dbg !11836
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_left.i224, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33) #30, !dbg !14158
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_right.i223, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34) #30, !dbg !14159
  store i32 %main_cursor.sroa.0.0.i241.lcssa, ptr %_35, align 4, !dbg !14155, !alias.scope !11715, !noalias !11838
  store i32 %ring_cursor.sroa.0.0.i240.lcssa, ptr %85, align 4, !dbg !14156, !alias.scope !11715, !noalias !11838
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i216), !dbg !14160, !noalias !11843
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i217), !dbg !14161, !noalias !11843
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i), !dbg !14162, !noalias !11843
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, !dbg !11708

bb4.i:                                            ; preds = %bb1.i3.i3123, %bb2.i3117
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14163), !dbg !14166
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14167), !dbg !14166
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14169), !dbg !14166
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14171), !dbg !14166
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14173), !dbg !14166
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_left.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #30, !dbg !14175
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_right.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #30, !dbg !14179
  %613 = getelementptr inbounds nuw i8, ptr %self, i64 1776, !dbg !14181
  %_240.0.i = load ptr, ptr %613, align 8, !dbg !14181, !alias.scope !14169, !noalias !14183, !nonnull !12, !noundef !12
  %614 = getelementptr inbounds nuw i8, ptr %self, i64 1784, !dbg !14181
  %_240.1.i = load i64, ptr %614, align 8, !dbg !14181, !alias.scope !14169, !noalias !14183, !noundef !12
  %_8.i3250 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_240.0.i, i64 %_240.1.i, !dbg !14186
  br label %bb1.i.i3251, !dbg !14191

bb1.i.i3251:                                      ; preds = %bb13.i.i3254, %bb4.i
  %_221.i.i3252 = phi ptr [ %_22.i.i3255, %bb13.i.i3254 ], [ %_240.0.i, %bb4.i ]
  %_12.i.i3253 = icmp eq ptr %_221.i.i3252, %_8.i3250, !dbg !14193
  br i1 %_12.i.i3253, label %bb4.i5, label %bb13.i.i3254, !dbg !14196

bb13.i.i3254:                                     ; preds = %bb1.i.i3251
  %_22.i.i3255 = getelementptr inbounds nuw i8, ptr %_221.i.i3252, i64 16, !dbg !14197
  %615 = getelementptr inbounds nuw i8, ptr %_221.i.i3252, i64 12, !dbg !14199
  %_3.i.i.i3256 = load i32, ptr %615, align 4, !dbg !14199, !alias.scope !14201, !noalias !14206, !noundef !12
  %616 = icmp eq i32 %_3.i.i.i3256, 0, !dbg !14199
  %_51.i.i.i3257 = load i32, ptr %_221.i.i3252, align 4, !dbg !14199, !alias.scope !14201, !noalias !14206
  %617 = getelementptr inbounds nuw i8, ptr %_221.i.i3252, i64 4, !dbg !14199
  %_72.i.i.i3258 = load i32, ptr %617, align 4, !dbg !14199, !alias.scope !14201, !noalias !14206
  %618 = icmp eq i32 %_51.i.i.i3257, %_72.i.i.i3258, !dbg !14199
  %_0.sroa.0.0.i.i.i3259 = select i1 %616, i1 %618, i1 false, !dbg !14199
  br i1 %_0.sroa.0.0.i.i.i3259, label %bb1.i.i3251, label %bb14.i, !dbg !14209

bb4.i5:                                           ; preds = %bb1.i.i3251
  %619 = getelementptr inbounds nuw i8, ptr %self, i64 1792, !dbg !14210
  %_241.0.i = load ptr, ptr %619, align 8, !dbg !14210, !alias.scope !14169, !noalias !14183, !nonnull !12, !noundef !12
  %620 = getelementptr inbounds nuw i8, ptr %self, i64 1800, !dbg !14210
  %_241.1.i = load i64, ptr %620, align 8, !dbg !14210, !alias.scope !14169, !noalias !14183, !noundef !12
  %_8.i3261 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_241.0.i, i64 %_241.1.i, !dbg !14211
  br label %bb1.i.i3262, !dbg !14216

bb1.i.i3262:                                      ; preds = %bb13.i.i3265, %bb4.i5
  %_221.i.i3263 = phi ptr [ %_22.i.i3266, %bb13.i.i3265 ], [ %_241.0.i, %bb4.i5 ]
  %_12.i.i3264 = icmp eq ptr %_221.i.i3263, %_8.i3261, !dbg !14218
  br i1 %_12.i.i3264, label %bb6.i, label %bb13.i.i3265, !dbg !14221

bb13.i.i3265:                                     ; preds = %bb1.i.i3262
  %_22.i.i3266 = getelementptr inbounds nuw i8, ptr %_221.i.i3263, i64 16, !dbg !14222
  %621 = getelementptr inbounds nuw i8, ptr %_221.i.i3263, i64 12, !dbg !14224
  %_3.i.i.i3267 = load i32, ptr %621, align 4, !dbg !14224, !alias.scope !14226, !noalias !14231, !noundef !12
  %622 = icmp eq i32 %_3.i.i.i3267, 0, !dbg !14224
  %_51.i.i.i3268 = load i32, ptr %_221.i.i3263, align 4, !dbg !14224, !alias.scope !14226, !noalias !14231
  %623 = getelementptr inbounds nuw i8, ptr %_221.i.i3263, i64 4, !dbg !14224
  %_72.i.i.i3269 = load i32, ptr %623, align 4, !dbg !14224, !alias.scope !14226, !noalias !14231
  %624 = icmp eq i32 %_51.i.i.i3268, %_72.i.i.i3269, !dbg !14224
  %_0.sroa.0.0.i.i.i3270 = select i1 %622, i1 %624, i1 false, !dbg !14224
  br i1 %_0.sroa.0.0.i.i.i3270, label %bb1.i.i3262, label %bb14.i, !dbg !14234

bb6.i:                                            ; preds = %bb1.i.i3262
  %625 = getelementptr inbounds nuw i8, ptr %self, i64 1976, !dbg !14235
  %_242.0.i = load ptr, ptr %625, align 8, !dbg !14235, !alias.scope !14171, !noalias !14236, !nonnull !12, !noundef !12
  %626 = getelementptr inbounds nuw i8, ptr %self, i64 1984, !dbg !14235
  %_242.1.i = load i64, ptr %626, align 8, !dbg !14235, !alias.scope !14171, !noalias !14236, !noundef !12
  %_8.i3272 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_242.0.i, i64 %_242.1.i, !dbg !14237
  br label %bb1.i.i3273, !dbg !14242

bb1.i.i3273:                                      ; preds = %bb13.i.i3276, %bb6.i
  %_221.i.i3274 = phi ptr [ %_22.i.i3277, %bb13.i.i3276 ], [ %_242.0.i, %bb6.i ]
  %_12.i.i3275 = icmp eq ptr %_221.i.i3274, %_8.i3272, !dbg !14244
  br i1 %_12.i.i3275, label %bb8.i, label %bb13.i.i3276, !dbg !14247

bb13.i.i3276:                                     ; preds = %bb1.i.i3273
  %_22.i.i3277 = getelementptr inbounds nuw i8, ptr %_221.i.i3274, i64 16, !dbg !14248
  %627 = getelementptr inbounds nuw i8, ptr %_221.i.i3274, i64 12, !dbg !14250
  %_3.i.i.i3278 = load i32, ptr %627, align 4, !dbg !14250, !alias.scope !14252, !noalias !14257, !noundef !12
  %628 = icmp eq i32 %_3.i.i.i3278, 0, !dbg !14250
  %_51.i.i.i3279 = load i32, ptr %_221.i.i3274, align 4, !dbg !14250, !alias.scope !14252, !noalias !14257
  %629 = getelementptr inbounds nuw i8, ptr %_221.i.i3274, i64 4, !dbg !14250
  %_72.i.i.i3280 = load i32, ptr %629, align 4, !dbg !14250, !alias.scope !14252, !noalias !14257
  %630 = icmp eq i32 %_51.i.i.i3279, %_72.i.i.i3280, !dbg !14250
  %_0.sroa.0.0.i.i.i3281 = select i1 %628, i1 %630, i1 false, !dbg !14250
  br i1 %_0.sroa.0.0.i.i.i3281, label %bb1.i.i3273, label %bb14.i, !dbg !14260

bb8.i:                                            ; preds = %bb1.i.i3273
  %631 = getelementptr inbounds nuw i8, ptr %self, i64 1992, !dbg !14261
  %_243.0.i = load ptr, ptr %631, align 8, !dbg !14261, !alias.scope !14171, !noalias !14236, !nonnull !12, !noundef !12
  %632 = getelementptr inbounds nuw i8, ptr %self, i64 2000, !dbg !14261
  %_243.1.i = load i64, ptr %632, align 8, !dbg !14261, !alias.scope !14171, !noalias !14236, !noundef !12
  %_8.i3283 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_243.0.i, i64 %_243.1.i, !dbg !14262
  br label %bb1.i.i3284, !dbg !14267

bb1.i.i3284:                                      ; preds = %bb13.i.i3287, %bb8.i
  %_221.i.i3285 = phi ptr [ %_22.i.i3288, %bb13.i.i3287 ], [ %_243.0.i, %bb8.i ]
  %_12.i.i3286 = icmp eq ptr %_221.i.i3285, %_8.i3283, !dbg !14269
  br i1 %_12.i.i3286, label %bb14.i, label %bb13.i.i3287, !dbg !14272

bb13.i.i3287:                                     ; preds = %bb1.i.i3284
  %_22.i.i3288 = getelementptr inbounds nuw i8, ptr %_221.i.i3285, i64 16, !dbg !14273
  %633 = getelementptr inbounds nuw i8, ptr %_221.i.i3285, i64 12, !dbg !14275
  %_3.i.i.i3289 = load i32, ptr %633, align 4, !dbg !14275, !alias.scope !14277, !noalias !14282, !noundef !12
  %634 = icmp eq i32 %_3.i.i.i3289, 0, !dbg !14275
  %_51.i.i.i3290 = load i32, ptr %_221.i.i3285, align 4, !dbg !14275, !alias.scope !14277, !noalias !14282
  %635 = getelementptr inbounds nuw i8, ptr %_221.i.i3285, i64 4, !dbg !14275
  %_72.i.i.i3291 = load i32, ptr %635, align 4, !dbg !14275, !alias.scope !14277, !noalias !14282
  %636 = icmp eq i32 %_51.i.i.i3290, %_72.i.i.i3291, !dbg !14275
  %_0.sroa.0.0.i.i.i3292 = select i1 %634, i1 %636, i1 false, !dbg !14275
  br i1 %_0.sroa.0.0.i.i.i3292, label %bb1.i.i3284, label %bb14.i, !dbg !14285

bb14.i:                                           ; preds = %bb13.i.i3254, %bb13.i.i3265, %bb13.i.i3276, %bb13.i.i3287, %bb1.i.i3284
  %stationary.sroa.0.0.i = phi i1 [ false, %bb13.i.i3276 ], [ false, %bb13.i.i3265 ], [ false, %bb13.i.i3287 ], [ true, %bb1.i.i3284 ], [ false, %bb13.i.i3254 ], !dbg !14286
  %637 = getelementptr inbounds nuw i8, ptr %self, i64 1536, !dbg !14287
  %638 = load i8, ptr %637, align 32, !dbg !14287, !range !5399, !alias.scope !14163, !noalias !14291, !noundef !12
  %639 = getelementptr inbounds nuw i8, ptr %self, i64 1537, !dbg !14292
  %640 = load i8, ptr %639, align 1, !dbg !14292, !range !5399, !alias.scope !14163, !noalias !14291, !noundef !12
  %641 = getelementptr inbounds nuw i8, ptr %self, i64 1624, !dbg !14294
  %ring.i = load i64, ptr %641, align 8, !dbg !14294, !alias.scope !14167, !noalias !14296, !noundef !12
  %642 = getelementptr inbounds nuw i8, ptr %self, i64 1632, !dbg !14297
  %main.i = load i64, ptr %642, align 8, !dbg !14297, !alias.scope !14167, !noalias !14296, !noundef !12
  %_42.i = load i32, ptr %_35, align 4, !dbg !14299, !alias.scope !14173, !noalias !14301, !noundef !12
  %643 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !14302
  %_43.i = load i32, ptr %643, align 4, !dbg !14302, !alias.scope !14173, !noalias !14301, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i), !dbg !14304, !noalias !14306
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i, i8 0, i64 1024, i1 false), !noalias !14306
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i), !dbg !14307, !noalias !14306
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i, i8 0, i64 1024, i1 false), !noalias !14306
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i), !dbg !14309, !noalias !14306
; call <true_peak_limiter::UniformHot<wide::f32x8_::f32x8>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_(ptr noalias noundef align 32 captures(none) dereferenceable(128) %uniform_left.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33, i64 %ring.i, i64 %main.i) #30, !dbg !14311
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_right.i), !dbg !14312, !noalias !14306
  %_32.val = load i64, ptr %641, align 8, !dbg !14314, !noundef !12
  %_32.val3018 = load i64, ptr %642, align 8, !dbg !14314, !noundef !12
; call <true_peak_limiter::UniformHot<wide::f32x8_::f32x8>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_(ptr noalias noundef align 32 captures(none) dereferenceable(128) %uniform_right.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34, i64 %_32.val, i64 %_32.val3018) #30, !dbg !14314
  %644 = add nuw nsw i64 %frames, 31, !dbg !14315
  %yield_count.sroa.0.0.i.i3297 = lshr i64 %644, 5, !dbg !14315
  %_172.not.i7469 = icmp eq i64 %yield_count.sroa.0.0.i.i3297, 0, !dbg !14322
  br i1 %_172.not.i7469, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, label %bb67.i.lr.ph, !dbg !14322

bb67.i.lr.ph:                                     ; preds = %bb14.i
  %645 = zext i32 %_43.i to i64, !dbg !14302
  %646 = zext i32 %_42.i to i64, !dbg !14299
  %_39.i = trunc nuw i8 %640 to i1, !dbg !14292
  %_38.i = trunc nuw i8 %638 to i1, !dbg !14287
  %647 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !14331
  %648 = bitcast <8 x float> %647 to <8 x i32>, !dbg !14337
  %649 = xor <8 x i32> %648, splat (i32 -1), !dbg !14343
  %history.i207.i.sroa.10.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 32
  %history.i207.i.sroa.13.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 64
  %history.i207.i.sroa.16.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 96
  %history.i207.i.sroa.19.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 128
  %history.i207.i.sroa.22.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 160
  %history.i207.i.sroa.25.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 192
  %history.i207.i.sroa.29.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 224
  %history.i207.i.sroa.32.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 256
  %history.i207.i.sroa.35.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 288
  %history.i207.i.sroa.38.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 320
  %history.i207.i.sroa.41.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 352
  %650 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %651 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %652 = getelementptr inbounds nuw i8, ptr %self, i64 96
  %row12.i.i.i218.i = getelementptr inbounds nuw i8, ptr %self, i64 128
  %653 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %654 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %655 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %row13.i.i.i219.i = getelementptr inbounds nuw i8, ptr %self, i64 256
  %656 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %657 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %658 = getelementptr inbounds nuw i8, ptr %self, i64 352
  %row14.i.i.i220.i = getelementptr inbounds nuw i8, ptr %self, i64 384
  %659 = getelementptr inbounds nuw i8, ptr %self, i64 416
  %660 = getelementptr inbounds nuw i8, ptr %self, i64 448
  %661 = getelementptr inbounds nuw i8, ptr %self, i64 480
  %row15.i.i.i221.i = getelementptr inbounds nuw i8, ptr %self, i64 512
  %662 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %663 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %664 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %row16.i.i.i222.i = getelementptr inbounds nuw i8, ptr %self, i64 640
  %665 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %666 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %667 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %row17.i.i.i223.i = getelementptr inbounds nuw i8, ptr %self, i64 768
  %668 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %669 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %670 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %row18.i.i.i224.i = getelementptr inbounds nuw i8, ptr %self, i64 896
  %671 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %672 = getelementptr inbounds nuw i8, ptr %self, i64 960
  %673 = getelementptr inbounds nuw i8, ptr %self, i64 992
  %row19.i.i.i225.i = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %674 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %675 = getelementptr inbounds nuw i8, ptr %self, i64 1088
  %676 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %row20.i.i.i226.i = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %677 = getelementptr inbounds nuw i8, ptr %self, i64 1184
  %678 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %679 = getelementptr inbounds nuw i8, ptr %self, i64 1248
  %row21.i.i.i227.i = getelementptr inbounds nuw i8, ptr %self, i64 1280
  %680 = getelementptr inbounds nuw i8, ptr %self, i64 1312
  %681 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %682 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %row22.i.i.i228.i = getelementptr inbounds nuw i8, ptr %self, i64 1408
  %683 = getelementptr inbounds nuw i8, ptr %self, i64 1440
  %684 = getelementptr inbounds nuw i8, ptr %self, i64 1472
  %685 = getelementptr inbounds nuw i8, ptr %self, i64 1504
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
  %686 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 80
  %_72.i.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 88
  %_72.i.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 96
  %687 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 80
  %_73.i.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 88
  %_73.i.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 96
  %_115.i = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 384
  %688 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 480
  %689 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 448
  %690 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 416
  %_117.i = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 512
  %691 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 608
  %692 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 576
  %693 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 544
  %_119.i = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 384
  %694 = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 480
  %695 = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 448
  %696 = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 416
  %_121.i = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 512
  %697 = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 608
  %698 = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 576
  %699 = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 544
  %700 = select i1 %_38.i, <8 x i32> %648, <8 x i32> %649
  %701 = icmp slt <8 x i32> %700, zeroinitializer
  %702 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 32
  %703 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 40
  %_22.i299.i = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 104
  %704 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 48
  %705 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 56
  %706 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 672
  %707 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 704
  %708 = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 640
  %709 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 72
  %710 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 64
  %711 = select i1 %_39.i, <8 x i32> %648, <8 x i32> %649
  %712 = icmp slt <8 x i32> %711, zeroinitializer
  %713 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 32
  %714 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 40
  %_22.i.i = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 104
  %715 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 48
  %716 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 56
  %717 = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 672
  %718 = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 704
  %719 = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 640
  %720 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 72
  %721 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 64
  br label %bb67.i, !dbg !14322

bb27.i.loopexit.loopexit:                         ; preds = %bb44.i
  store i32 %storemerge.i324.lcssa96419689, ptr %_22.i299.i, align 4
  store <8 x float> %minimum.i279.i.sroa.0.0.lcssa96559708, ptr %uniform_left.i, align 32
  store i32 %storemerge.i.lcssa96719727, ptr %_22.i.i, align 4
  store <8 x float> %minimum.i.i.sroa.0.0.lcssa96859746, ptr %uniform_right.i, align 32
  br label %bb27.i.loopexit, !dbg !14322

bb27.i.loopexit:                                  ; preds = %bb27.i.loopexit.loopexit, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i
  %ring_cursor.sroa.0.1.i.lcssa = phi i64 [ %ring_cursor.sroa.0.0.i7470, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ], [ %ring_cursor.sroa.0.2.i, %bb27.i.loopexit.loopexit ], !dbg !14345
  %main_cursor.sroa.0.1.i.lcssa = phi i64 [ %main_cursor.sroa.0.0.i7471, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ], [ %main_cursor.sroa.0.2.i, %bb27.i.loopexit.loopexit ], !dbg !14346
  %_172.not.i = icmp eq i64 %723, 0, !dbg !14322
  %indvars.iv.next8394 = add nsw i64 %indvars.iv8393, -32, !dbg !14322
  br i1 %_172.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit, label %bb67.i, !dbg !14322

bb67.i:                                           ; preds = %bb67.i.lr.ph, %bb27.i.loopexit
  %indvars.iv8393 = phi i64 [ %frames, %bb67.i.lr.ph ], [ %indvars.iv.next8394, %bb27.i.loopexit ]
  %iter4.sroa.0.0.i7473 = phi i64 [ %yield_count.sroa.0.0.i.i3297, %bb67.i.lr.ph ], [ %723, %bb27.i.loopexit ]
  %iter3.sroa.0.0.i7472 = phi i64 [ 0, %bb67.i.lr.ph ], [ %722, %bb27.i.loopexit ]
  %main_cursor.sroa.0.0.i7471 = phi i64 [ %646, %bb67.i.lr.ph ], [ %main_cursor.sroa.0.1.i.lcssa, %bb27.i.loopexit ]
  %ring_cursor.sroa.0.0.i7470 = phi i64 [ %645, %bb67.i.lr.ph ], [ %ring_cursor.sroa.0.1.i.lcssa, %bb27.i.loopexit ]
  %umin8425 = call i64 @llvm.umin.i64(i64 %indvars.iv8393, i64 32), !dbg !14347
  %umax8401 = call i64 @llvm.umax.i64(i64 %umin8425, i64 1), !dbg !14347
  %722 = add nuw nsw i64 %iter3.sroa.0.0.i7472, 32, !dbg !14347
  %723 = add nsw i64 %iter4.sroa.0.0.i7473, -1, !dbg !14351
  %_53.i = sub nsw i64 %frames, %iter3.sroa.0.0.i7472, !dbg !14352
  %..i3298 = tail call noundef i64 @llvm.umin.i64(i64 %_53.i, i64 32), !dbg !14354
  %history.i207.i.sroa.0.0.copyload = load <8 x float>, ptr %hot_left.i, align 32, !dbg !14358
  %history.i207.i.sroa.10.sroa.0.0.copyload = load <8 x float>, ptr %history.i207.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !14358
  %history.i207.i.sroa.13.sroa.0.0.copyload = load <8 x float>, ptr %history.i207.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !14358
  %history.i207.i.sroa.16.sroa.0.0.copyload = load <8 x float>, ptr %history.i207.i.sroa.16.0.hot_left.i.sroa_idx, align 32, !dbg !14358
  %history.i207.i.sroa.19.sroa.0.0.copyload = load <8 x float>, ptr %history.i207.i.sroa.19.0.hot_left.i.sroa_idx, align 32, !dbg !14358
  %history.i207.i.sroa.22.sroa.0.0.copyload = load <8 x float>, ptr %history.i207.i.sroa.22.0.hot_left.i.sroa_idx, align 32, !dbg !14358
  %history.i207.i.sroa.25.sroa.0.0.copyload = load <8 x float>, ptr %history.i207.i.sroa.25.0.hot_left.i.sroa_idx, align 32, !dbg !14358
  %history.i207.i.sroa.29.sroa.0.0.copyload = load <8 x float>, ptr %history.i207.i.sroa.29.0.hot_left.i.sroa_idx, align 32, !dbg !14358
  %history.i207.i.sroa.32.sroa.0.0.copyload = load <8 x float>, ptr %history.i207.i.sroa.32.0.hot_left.i.sroa_idx, align 32, !dbg !14358
  %history.i207.i.sroa.35.sroa.0.0.copyload = load <8 x float>, ptr %history.i207.i.sroa.35.0.hot_left.i.sroa_idx, align 32, !dbg !14358
  %history.i207.i.sroa.38.sroa.0.0.copyload = load <8 x float>, ptr %history.i207.i.sroa.38.0.hot_left.i.sroa_idx, align 32, !dbg !14358
  %history.i207.i.sroa.41.sroa.0.0.copyload = load <8 x float>, ptr %history.i207.i.sroa.41.0.hot_left.i.sroa_idx, align 32, !dbg !14358
  %_20.i210.i7400.not = icmp eq i64 %frames, %iter3.sroa.0.0.i7472, !dbg !14361
  br i1 %_20.i210.i7400.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit242.i, label %bb5.i211.i.lr.ph, !dbg !14365

bb5.i211.i.lr.ph:                                 ; preds = %bb67.i
  %_5.i2044 = load <8 x float>, ptr %self, align 32
  %_14.i.i.i172.i.sroa.0.0.copyload = load <8 x float>, ptr %650, align 32
  %_17.i.i.i169.i.sroa.0.0.copyload = load <8 x float>, ptr %651, align 32
  %_20.i.i.i166.i.sroa.0.0.copyload = load <8 x float>, ptr %652, align 32
  %_25.i.i.i162.i.sroa.0.0.copyload = load <8 x float>, ptr %row12.i.i.i218.i, align 32
  %_28.i.i.i159.i.sroa.0.0.copyload = load <8 x float>, ptr %653, align 32
  %_31.i.i.i156.i.sroa.0.0.copyload = load <8 x float>, ptr %654, align 32
  %_34.i.i.i153.i.sroa.0.0.copyload = load <8 x float>, ptr %655, align 32
  %_39.i.i.i149.i.sroa.0.0.copyload = load <8 x float>, ptr %row13.i.i.i219.i, align 32
  %_42.i.i.i146.i.sroa.0.0.copyload = load <8 x float>, ptr %656, align 32
  %_45.i.i.i143.i.sroa.0.0.copyload = load <8 x float>, ptr %657, align 32
  %_48.i.i.i140.i.sroa.0.0.copyload = load <8 x float>, ptr %658, align 32
  %_53.i.i.i136.i.sroa.0.0.copyload = load <8 x float>, ptr %row14.i.i.i220.i, align 32
  %_56.i.i.i133.i.sroa.0.0.copyload = load <8 x float>, ptr %659, align 32
  %_59.i.i.i130.i.sroa.0.0.copyload = load <8 x float>, ptr %660, align 32
  %_62.i.i.i127.i.sroa.0.0.copyload = load <8 x float>, ptr %661, align 32
  %_67.i.i.i123.i.sroa.0.0.copyload = load <8 x float>, ptr %row15.i.i.i221.i, align 32
  %_70.i.i.i120.i.sroa.0.0.copyload = load <8 x float>, ptr %662, align 32
  %_73.i.i.i117.i.sroa.0.0.copyload = load <8 x float>, ptr %663, align 32
  %_76.i.i.i114.i.sroa.0.0.copyload = load <8 x float>, ptr %664, align 32
  %_81.i.i.i110.i.sroa.0.0.copyload = load <8 x float>, ptr %row16.i.i.i222.i, align 32
  %_84.i.i.i107.i.sroa.0.0.copyload = load <8 x float>, ptr %665, align 32
  %_87.i.i.i104.i.sroa.0.0.copyload = load <8 x float>, ptr %666, align 32
  %_90.i.i.i101.i.sroa.0.0.copyload = load <8 x float>, ptr %667, align 32
  %_95.i.i.i97.i.sroa.0.0.copyload = load <8 x float>, ptr %row17.i.i.i223.i, align 32
  %_98.i.i.i94.i.sroa.0.0.copyload = load <8 x float>, ptr %668, align 32
  %_101.i.i.i91.i.sroa.0.0.copyload = load <8 x float>, ptr %669, align 32
  %_104.i.i.i88.i.sroa.0.0.copyload = load <8 x float>, ptr %670, align 32
  %_109.i.i.i84.i.sroa.0.0.copyload = load <8 x float>, ptr %row18.i.i.i224.i, align 32
  %_112.i.i.i81.i.sroa.0.0.copyload = load <8 x float>, ptr %671, align 32
  %_115.i.i.i78.i.sroa.0.0.copyload = load <8 x float>, ptr %672, align 32
  %_118.i.i.i75.i.sroa.0.0.copyload = load <8 x float>, ptr %673, align 32
  %_123.i.i.i71.i.sroa.0.0.copyload = load <8 x float>, ptr %row19.i.i.i225.i, align 32
  %_126.i.i.i68.i.sroa.0.0.copyload = load <8 x float>, ptr %674, align 32
  %_129.i.i.i65.i.sroa.0.0.copyload = load <8 x float>, ptr %675, align 32
  %_132.i.i.i62.i.sroa.0.0.copyload = load <8 x float>, ptr %676, align 32
  %_137.i.i.i58.i.sroa.0.0.copyload = load <8 x float>, ptr %row20.i.i.i226.i, align 32
  %_140.i.i.i55.i.sroa.0.0.copyload = load <8 x float>, ptr %677, align 32
  %_143.i.i.i52.i.sroa.0.0.copyload = load <8 x float>, ptr %678, align 32
  %_146.i.i.i49.i.sroa.0.0.copyload = load <8 x float>, ptr %679, align 32
  %_151.i.i.i45.i.sroa.0.0.copyload = load <8 x float>, ptr %row21.i.i.i227.i, align 32
  %_154.i.i.i42.i.sroa.0.0.copyload = load <8 x float>, ptr %680, align 32
  %_157.i.i.i39.i.sroa.0.0.copyload = load <8 x float>, ptr %681, align 32
  %_160.i.i.i36.i.sroa.0.0.copyload = load <8 x float>, ptr %682, align 32
  %_165.i.i.i32.i.sroa.0.0.copyload = load <8 x float>, ptr %row22.i.i.i228.i, align 32
  %_168.i.i.i29.i.sroa.0.0.copyload = load <8 x float>, ptr %683, align 32
  %_171.i.i.i26.i.sroa.0.0.copyload = load <8 x float>, ptr %684, align 32
  %_174.i.i.i23.i.sroa.0.0.copyload = load <8 x float>, ptr %685, align 32
  br label %bb5.i211.i, !dbg !14365

bb5.i211.i:                                       ; preds = %bb5.i211.i.lr.ph, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848
  %iter.sroa.0.0.i209.i7412 = phi i64 [ 0, %bb5.i211.i.lr.ph ], [ %724, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848 ]
  %history.i207.i.sroa.10.sroa.0.07411 = phi <8 x float> [ %history.i207.i.sroa.10.sroa.0.0.copyload, %bb5.i211.i.lr.ph ], [ %history.i207.i.sroa.0.07401, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848 ]
  %history.i207.i.sroa.13.sroa.0.07410 = phi <8 x float> [ %history.i207.i.sroa.13.sroa.0.0.copyload, %bb5.i211.i.lr.ph ], [ %history.i207.i.sroa.10.sroa.0.07411, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848 ]
  %history.i207.i.sroa.16.sroa.0.07409 = phi <8 x float> [ %history.i207.i.sroa.16.sroa.0.0.copyload, %bb5.i211.i.lr.ph ], [ %history.i207.i.sroa.13.sroa.0.07410, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848 ]
  %history.i207.i.sroa.19.sroa.0.07408 = phi <8 x float> [ %history.i207.i.sroa.19.sroa.0.0.copyload, %bb5.i211.i.lr.ph ], [ %history.i207.i.sroa.16.sroa.0.07409, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848 ]
  %history.i207.i.sroa.22.sroa.0.07407 = phi <8 x float> [ %history.i207.i.sroa.22.sroa.0.0.copyload, %bb5.i211.i.lr.ph ], [ %history.i207.i.sroa.19.sroa.0.07408, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848 ]
  %history.i207.i.sroa.38.sroa.0.07406 = phi <8 x float> [ %history.i207.i.sroa.38.sroa.0.0.copyload, %bb5.i211.i.lr.ph ], [ %history.i207.i.sroa.35.sroa.0.07405, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848 ]
  %history.i207.i.sroa.35.sroa.0.07405 = phi <8 x float> [ %history.i207.i.sroa.35.sroa.0.0.copyload, %bb5.i211.i.lr.ph ], [ %history.i207.i.sroa.32.sroa.0.07404, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848 ]
  %history.i207.i.sroa.32.sroa.0.07404 = phi <8 x float> [ %history.i207.i.sroa.32.sroa.0.0.copyload, %bb5.i211.i.lr.ph ], [ %history.i207.i.sroa.29.sroa.0.07403, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848 ]
  %history.i207.i.sroa.29.sroa.0.07403 = phi <8 x float> [ %history.i207.i.sroa.29.sroa.0.0.copyload, %bb5.i211.i.lr.ph ], [ %history.i207.i.sroa.25.sroa.0.07402, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848 ]
  %history.i207.i.sroa.25.sroa.0.07402 = phi <8 x float> [ %history.i207.i.sroa.25.sroa.0.0.copyload, %bb5.i211.i.lr.ph ], [ %history.i207.i.sroa.22.sroa.0.07407, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848 ]
  %history.i207.i.sroa.0.07401 = phi <8 x float> [ %history.i207.i.sroa.0.0.copyload, %bb5.i211.i.lr.ph ], [ %lanes.i2464.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848 ]
  %724 = add nuw nsw i64 %iter.sroa.0.0.i209.i7412, 1, !dbg !14366
  %_11.i212.i = add nuw nsw i64 %iter.sroa.0.0.i209.i7412, %iter3.sroa.0.0.i7472, !dbg !14369
  %base.i213.i = shl i64 %_11.i212.i, 3, !dbg !14369
  %_24.i214.i = icmp samesign ugt i64 %base.i213.i, %left_io.1, !dbg !14370
  br i1 %_24.i214.i, label %bb7.i241.i, label %bb8.i215.i, !dbg !14370, !prof !905

bb8.i215.i:                                       ; preds = %bb5.i211.i
  %_27.i216.i = sub nuw nsw i64 %left_io.1, %base.i213.i, !dbg !14373
  %_8.i2467 = icmp samesign ugt i64 %_27.i216.i, 7, !dbg !14374
  br i1 %_8.i2467, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848, label %bb2.i2468, !dbg !14374, !prof !1076

bb2.i2468:                                        ; preds = %bb8.i215.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_27.i216.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #31, !dbg !14379, !noalias !14380
  unreachable, !dbg !14379

bb7.i241.i:                                       ; preds = %bb5.i211.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i213.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e0592aef22128a0ac53753b9632a8183) #31, !dbg !14387, !noalias !14388
  unreachable, !dbg !14387

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848: ; preds = %bb8.i215.i
  %_31.i217.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %base.i213.i, !dbg !14389
  %lanes.i2464.sroa.0.0.copyload = load <8 x float>, ptr %_31.i217.i, align 4, !dbg !14391, !alias.scope !14395, !noalias !14399
  %725 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i207.i.sroa.22.sroa.0.07407), !dbg !14401
  %726 = fmul <8 x float> %lanes.i2464.sroa.0.0.copyload, %_5.i2044, !dbg !14408
  %727 = fadd <8 x float> %726, zeroinitializer, !dbg !14414
  %728 = fmul <8 x float> %lanes.i2464.sroa.0.0.copyload, %_14.i.i.i172.i.sroa.0.0.copyload, !dbg !14419
  %729 = fadd <8 x float> %728, zeroinitializer, !dbg !14424
  %730 = fmul <8 x float> %lanes.i2464.sroa.0.0.copyload, %_17.i.i.i169.i.sroa.0.0.copyload, !dbg !14429
  %731 = fadd <8 x float> %730, zeroinitializer, !dbg !14434
  %732 = fmul <8 x float> %lanes.i2464.sroa.0.0.copyload, %_20.i.i.i166.i.sroa.0.0.copyload, !dbg !14439
  %733 = fadd <8 x float> %732, zeroinitializer, !dbg !14444
  %734 = fmul <8 x float> %history.i207.i.sroa.0.07401, %_25.i.i.i162.i.sroa.0.0.copyload, !dbg !14449
  %735 = fadd <8 x float> %727, %734, !dbg !14454
  %736 = fmul <8 x float> %history.i207.i.sroa.0.07401, %_28.i.i.i159.i.sroa.0.0.copyload, !dbg !14459
  %737 = fadd <8 x float> %729, %736, !dbg !14464
  %738 = fmul <8 x float> %history.i207.i.sroa.0.07401, %_31.i.i.i156.i.sroa.0.0.copyload, !dbg !14469
  %739 = fadd <8 x float> %731, %738, !dbg !14474
  %740 = fmul <8 x float> %history.i207.i.sroa.0.07401, %_34.i.i.i153.i.sroa.0.0.copyload, !dbg !14479
  %741 = fadd <8 x float> %733, %740, !dbg !14484
  %742 = fmul <8 x float> %history.i207.i.sroa.10.sroa.0.07411, %_39.i.i.i149.i.sroa.0.0.copyload, !dbg !14489
  %743 = fadd <8 x float> %735, %742, !dbg !14494
  %744 = fmul <8 x float> %history.i207.i.sroa.10.sroa.0.07411, %_42.i.i.i146.i.sroa.0.0.copyload, !dbg !14499
  %745 = fadd <8 x float> %737, %744, !dbg !14504
  %746 = fmul <8 x float> %history.i207.i.sroa.10.sroa.0.07411, %_45.i.i.i143.i.sroa.0.0.copyload, !dbg !14509
  %747 = fadd <8 x float> %739, %746, !dbg !14514
  %748 = fmul <8 x float> %history.i207.i.sroa.10.sroa.0.07411, %_48.i.i.i140.i.sroa.0.0.copyload, !dbg !14519
  %749 = fadd <8 x float> %741, %748, !dbg !14524
  %750 = fmul <8 x float> %history.i207.i.sroa.13.sroa.0.07410, %_53.i.i.i136.i.sroa.0.0.copyload, !dbg !14529
  %751 = fadd <8 x float> %743, %750, !dbg !14534
  %752 = fmul <8 x float> %history.i207.i.sroa.13.sroa.0.07410, %_56.i.i.i133.i.sroa.0.0.copyload, !dbg !14539
  %753 = fadd <8 x float> %745, %752, !dbg !14544
  %754 = fmul <8 x float> %history.i207.i.sroa.13.sroa.0.07410, %_59.i.i.i130.i.sroa.0.0.copyload, !dbg !14549
  %755 = fadd <8 x float> %747, %754, !dbg !14554
  %756 = fmul <8 x float> %history.i207.i.sroa.13.sroa.0.07410, %_62.i.i.i127.i.sroa.0.0.copyload, !dbg !14559
  %757 = fadd <8 x float> %749, %756, !dbg !14564
  %758 = fmul <8 x float> %history.i207.i.sroa.16.sroa.0.07409, %_67.i.i.i123.i.sroa.0.0.copyload, !dbg !14569
  %759 = fadd <8 x float> %751, %758, !dbg !14574
  %760 = fmul <8 x float> %history.i207.i.sroa.16.sroa.0.07409, %_70.i.i.i120.i.sroa.0.0.copyload, !dbg !14579
  %761 = fadd <8 x float> %753, %760, !dbg !14584
  %762 = fmul <8 x float> %history.i207.i.sroa.16.sroa.0.07409, %_73.i.i.i117.i.sroa.0.0.copyload, !dbg !14589
  %763 = fadd <8 x float> %755, %762, !dbg !14594
  %764 = fmul <8 x float> %history.i207.i.sroa.16.sroa.0.07409, %_76.i.i.i114.i.sroa.0.0.copyload, !dbg !14599
  %765 = fadd <8 x float> %757, %764, !dbg !14604
  %766 = fmul <8 x float> %history.i207.i.sroa.19.sroa.0.07408, %_81.i.i.i110.i.sroa.0.0.copyload, !dbg !14609
  %767 = fadd <8 x float> %759, %766, !dbg !14614
  %768 = fmul <8 x float> %history.i207.i.sroa.19.sroa.0.07408, %_84.i.i.i107.i.sroa.0.0.copyload, !dbg !14619
  %769 = fadd <8 x float> %761, %768, !dbg !14624
  %770 = fmul <8 x float> %history.i207.i.sroa.19.sroa.0.07408, %_87.i.i.i104.i.sroa.0.0.copyload, !dbg !14629
  %771 = fadd <8 x float> %763, %770, !dbg !14634
  %772 = fmul <8 x float> %history.i207.i.sroa.19.sroa.0.07408, %_90.i.i.i101.i.sroa.0.0.copyload, !dbg !14639
  %773 = fadd <8 x float> %765, %772, !dbg !14644
  %774 = fmul <8 x float> %history.i207.i.sroa.22.sroa.0.07407, %_95.i.i.i97.i.sroa.0.0.copyload, !dbg !14649
  %775 = fadd <8 x float> %767, %774, !dbg !14654
  %776 = fmul <8 x float> %history.i207.i.sroa.22.sroa.0.07407, %_98.i.i.i94.i.sroa.0.0.copyload, !dbg !14659
  %777 = fadd <8 x float> %769, %776, !dbg !14664
  %778 = fmul <8 x float> %history.i207.i.sroa.22.sroa.0.07407, %_101.i.i.i91.i.sroa.0.0.copyload, !dbg !14669
  %779 = fadd <8 x float> %771, %778, !dbg !14674
  %780 = fmul <8 x float> %history.i207.i.sroa.22.sroa.0.07407, %_104.i.i.i88.i.sroa.0.0.copyload, !dbg !14679
  %781 = fadd <8 x float> %773, %780, !dbg !14684
  %782 = fmul <8 x float> %history.i207.i.sroa.25.sroa.0.07402, %_109.i.i.i84.i.sroa.0.0.copyload, !dbg !14689
  %783 = fadd <8 x float> %775, %782, !dbg !14694
  %784 = fmul <8 x float> %history.i207.i.sroa.25.sroa.0.07402, %_112.i.i.i81.i.sroa.0.0.copyload, !dbg !14699
  %785 = fadd <8 x float> %777, %784, !dbg !14704
  %786 = fmul <8 x float> %history.i207.i.sroa.25.sroa.0.07402, %_115.i.i.i78.i.sroa.0.0.copyload, !dbg !14709
  %787 = fadd <8 x float> %779, %786, !dbg !14714
  %788 = fmul <8 x float> %history.i207.i.sroa.25.sroa.0.07402, %_118.i.i.i75.i.sroa.0.0.copyload, !dbg !14719
  %789 = fadd <8 x float> %781, %788, !dbg !14724
  %790 = fmul <8 x float> %history.i207.i.sroa.29.sroa.0.07403, %_123.i.i.i71.i.sroa.0.0.copyload, !dbg !14729
  %791 = fadd <8 x float> %783, %790, !dbg !14734
  %792 = fmul <8 x float> %history.i207.i.sroa.29.sroa.0.07403, %_126.i.i.i68.i.sroa.0.0.copyload, !dbg !14739
  %793 = fadd <8 x float> %785, %792, !dbg !14744
  %794 = fmul <8 x float> %history.i207.i.sroa.29.sroa.0.07403, %_129.i.i.i65.i.sroa.0.0.copyload, !dbg !14749
  %795 = fadd <8 x float> %787, %794, !dbg !14754
  %796 = fmul <8 x float> %history.i207.i.sroa.29.sroa.0.07403, %_132.i.i.i62.i.sroa.0.0.copyload, !dbg !14759
  %797 = fadd <8 x float> %789, %796, !dbg !14764
  %798 = fmul <8 x float> %history.i207.i.sroa.32.sroa.0.07404, %_137.i.i.i58.i.sroa.0.0.copyload, !dbg !14769
  %799 = fadd <8 x float> %791, %798, !dbg !14774
  %800 = fmul <8 x float> %history.i207.i.sroa.32.sroa.0.07404, %_140.i.i.i55.i.sroa.0.0.copyload, !dbg !14779
  %801 = fadd <8 x float> %793, %800, !dbg !14784
  %802 = fmul <8 x float> %history.i207.i.sroa.32.sroa.0.07404, %_143.i.i.i52.i.sroa.0.0.copyload, !dbg !14789
  %803 = fadd <8 x float> %795, %802, !dbg !14794
  %804 = fmul <8 x float> %history.i207.i.sroa.32.sroa.0.07404, %_146.i.i.i49.i.sroa.0.0.copyload, !dbg !14799
  %805 = fadd <8 x float> %797, %804, !dbg !14804
  %806 = fmul <8 x float> %history.i207.i.sroa.35.sroa.0.07405, %_151.i.i.i45.i.sroa.0.0.copyload, !dbg !14809
  %807 = fadd <8 x float> %799, %806, !dbg !14814
  %808 = fmul <8 x float> %history.i207.i.sroa.35.sroa.0.07405, %_154.i.i.i42.i.sroa.0.0.copyload, !dbg !14819
  %809 = fadd <8 x float> %801, %808, !dbg !14824
  %810 = fmul <8 x float> %history.i207.i.sroa.35.sroa.0.07405, %_157.i.i.i39.i.sroa.0.0.copyload, !dbg !14829
  %811 = fadd <8 x float> %803, %810, !dbg !14834
  %812 = fmul <8 x float> %history.i207.i.sroa.35.sroa.0.07405, %_160.i.i.i36.i.sroa.0.0.copyload, !dbg !14839
  %813 = fadd <8 x float> %805, %812, !dbg !14844
  %814 = fmul <8 x float> %history.i207.i.sroa.38.sroa.0.07406, %_165.i.i.i32.i.sroa.0.0.copyload, !dbg !14849
  %815 = fadd <8 x float> %807, %814, !dbg !14854
  %816 = fmul <8 x float> %history.i207.i.sroa.38.sroa.0.07406, %_168.i.i.i29.i.sroa.0.0.copyload, !dbg !14859
  %817 = fadd <8 x float> %809, %816, !dbg !14864
  %818 = fmul <8 x float> %history.i207.i.sroa.38.sroa.0.07406, %_171.i.i.i26.i.sroa.0.0.copyload, !dbg !14869
  %819 = fadd <8 x float> %811, %818, !dbg !14874
  %820 = fmul <8 x float> %history.i207.i.sroa.38.sroa.0.07406, %_174.i.i.i23.i.sroa.0.0.copyload, !dbg !14879
  %821 = fadd <8 x float> %813, %820, !dbg !14884
  %822 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %815), !dbg !14889
  %823 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %725, <8 x float> %822), !dbg !14895
  %824 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %817), !dbg !14889
  %825 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %823, <8 x float> %824), !dbg !14895
  %826 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %819), !dbg !14889
  %827 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %825, <8 x float> %826), !dbg !14895
  %828 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %821), !dbg !14889
  %829 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %827, <8 x float> %828), !dbg !14895
  %_39.i238.i.idx = shl i64 %iter.sroa.0.0.i209.i7412, 5, !dbg !14900
  %_39.i238.i = getelementptr inbounds nuw i8, ptr %peaks_left.i, i64 %_39.i238.i.idx, !dbg !14900
  store <8 x float> %829, ptr %_39.i238.i, align 4, !dbg !14905, !alias.scope !14910, !noalias !14914
  %exitcond8397.not = icmp eq i64 %724, %umax8401, !dbg !14361
  br i1 %exitcond8397.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit242.i, label %bb5.i211.i, !dbg !14365

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit242.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848, %bb67.i
  %history.i207.i.sroa.0.0.lcssa = phi <8 x float> [ %history.i207.i.sroa.0.0.copyload, %bb67.i ], [ %lanes.i2464.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848 ], !dbg !14918
  %history.i207.i.sroa.25.sroa.0.0.lcssa = phi <8 x float> [ %history.i207.i.sroa.25.sroa.0.0.copyload, %bb67.i ], [ %history.i207.i.sroa.22.sroa.0.07407, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848 ], !dbg !14918
  %history.i207.i.sroa.29.sroa.0.0.lcssa = phi <8 x float> [ %history.i207.i.sroa.29.sroa.0.0.copyload, %bb67.i ], [ %history.i207.i.sroa.25.sroa.0.07402, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848 ], !dbg !14918
  %history.i207.i.sroa.32.sroa.0.0.lcssa = phi <8 x float> [ %history.i207.i.sroa.32.sroa.0.0.copyload, %bb67.i ], [ %history.i207.i.sroa.29.sroa.0.07403, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848 ], !dbg !14918
  %history.i207.i.sroa.35.sroa.0.0.lcssa = phi <8 x float> [ %history.i207.i.sroa.35.sroa.0.0.copyload, %bb67.i ], [ %history.i207.i.sroa.32.sroa.0.07404, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848 ], !dbg !14918
  %history.i207.i.sroa.38.sroa.0.0.lcssa = phi <8 x float> [ %history.i207.i.sroa.38.sroa.0.0.copyload, %bb67.i ], [ %history.i207.i.sroa.35.sroa.0.07405, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848 ], !dbg !14918
  %history.i207.i.sroa.41.sroa.0.0.lcssa = phi <8 x float> [ %history.i207.i.sroa.41.sroa.0.0.copyload, %bb67.i ], [ %history.i207.i.sroa.38.sroa.0.07406, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848 ], !dbg !14918
  %history.i207.i.sroa.22.sroa.0.0.lcssa = phi <8 x float> [ %history.i207.i.sroa.22.sroa.0.0.copyload, %bb67.i ], [ %history.i207.i.sroa.19.sroa.0.07408, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848 ], !dbg !14918
  %history.i207.i.sroa.19.sroa.0.0.lcssa = phi <8 x float> [ %history.i207.i.sroa.19.sroa.0.0.copyload, %bb67.i ], [ %history.i207.i.sroa.16.sroa.0.07409, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848 ], !dbg !14918
  %history.i207.i.sroa.16.sroa.0.0.lcssa = phi <8 x float> [ %history.i207.i.sroa.16.sroa.0.0.copyload, %bb67.i ], [ %history.i207.i.sroa.13.sroa.0.07410, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848 ], !dbg !14918
  %history.i207.i.sroa.13.sroa.0.0.lcssa = phi <8 x float> [ %history.i207.i.sroa.13.sroa.0.0.copyload, %bb67.i ], [ %history.i207.i.sroa.10.sroa.0.07411, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848 ], !dbg !14918
  %history.i207.i.sroa.10.sroa.0.0.lcssa = phi <8 x float> [ %history.i207.i.sroa.10.sroa.0.0.copyload, %bb67.i ], [ %history.i207.i.sroa.0.07401, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2848 ], !dbg !14918
  store <8 x float> %history.i207.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !14919
  store <8 x float> %history.i207.i.sroa.10.sroa.0.0.lcssa, ptr %history.i207.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !14919
  store <8 x float> %history.i207.i.sroa.13.sroa.0.0.lcssa, ptr %history.i207.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !14919
  store <8 x float> %history.i207.i.sroa.16.sroa.0.0.lcssa, ptr %history.i207.i.sroa.16.0.hot_left.i.sroa_idx, align 32, !dbg !14919
  store <8 x float> %history.i207.i.sroa.19.sroa.0.0.lcssa, ptr %history.i207.i.sroa.19.0.hot_left.i.sroa_idx, align 32, !dbg !14919
  store <8 x float> %history.i207.i.sroa.22.sroa.0.0.lcssa, ptr %history.i207.i.sroa.22.0.hot_left.i.sroa_idx, align 32, !dbg !14919
  store <8 x float> %history.i207.i.sroa.25.sroa.0.0.lcssa, ptr %history.i207.i.sroa.25.0.hot_left.i.sroa_idx, align 32, !dbg !14919
  store <8 x float> %history.i207.i.sroa.29.sroa.0.0.lcssa, ptr %history.i207.i.sroa.29.0.hot_left.i.sroa_idx, align 32, !dbg !14919
  store <8 x float> %history.i207.i.sroa.32.sroa.0.0.lcssa, ptr %history.i207.i.sroa.32.0.hot_left.i.sroa_idx, align 32, !dbg !14919
  store <8 x float> %history.i207.i.sroa.35.sroa.0.0.lcssa, ptr %history.i207.i.sroa.35.0.hot_left.i.sroa_idx, align 32, !dbg !14919
  store <8 x float> %history.i207.i.sroa.38.sroa.0.0.lcssa, ptr %history.i207.i.sroa.38.0.hot_left.i.sroa_idx, align 32, !dbg !14919
  store <8 x float> %history.i207.i.sroa.41.sroa.0.0.lcssa, ptr %history.i207.i.sroa.41.0.hot_left.i.sroa_idx, align 32, !dbg !14919
  %history.i.i.sroa.0.0.copyload = load <8 x float>, ptr %hot_right.i, align 32, !dbg !14920
  %history.i.i.sroa.10.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !14920
  %history.i.i.sroa.13.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !14920
  %history.i.i.sroa.16.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 32, !dbg !14920
  %history.i.i.sroa.19.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 32, !dbg !14920
  %history.i.i.sroa.22.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 32, !dbg !14920
  %history.i.i.sroa.25.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i.sroa.25.0.hot_right.i.sroa_idx, align 32, !dbg !14920
  %history.i.i.sroa.29.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 32, !dbg !14920
  %history.i.i.sroa.32.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 32, !dbg !14920
  %history.i.i.sroa.35.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 32, !dbg !14920
  %history.i.i.sroa.38.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 32, !dbg !14920
  %history.i.i.sroa.41.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i.sroa.41.0.hot_right.i.sroa_idx, align 32, !dbg !14920
  br i1 %_20.i210.i7400.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, label %bb5.i.i.lr.ph, !dbg !14922

bb5.i.i.lr.ph:                                    ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit242.i
  %_5.i2188 = load <8 x float>, ptr %self, align 32
  %_14.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %650, align 32
  %_17.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %651, align 32
  %_20.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %652, align 32
  %_25.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row12.i.i.i218.i, align 32
  %_28.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %653, align 32
  %_31.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %654, align 32
  %_34.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %655, align 32
  %_39.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row13.i.i.i219.i, align 32
  %_42.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %656, align 32
  %_45.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %657, align 32
  %_48.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %658, align 32
  %_53.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row14.i.i.i220.i, align 32
  %_56.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %659, align 32
  %_59.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %660, align 32
  %_62.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %661, align 32
  %_67.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row15.i.i.i221.i, align 32
  %_70.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %662, align 32
  %_73.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %663, align 32
  %_76.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %664, align 32
  %_81.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row16.i.i.i222.i, align 32
  %_84.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %665, align 32
  %_87.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %666, align 32
  %_90.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %667, align 32
  %_95.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row17.i.i.i223.i, align 32
  %_98.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %668, align 32
  %_101.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %669, align 32
  %_104.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %670, align 32
  %_109.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row18.i.i.i224.i, align 32
  %_112.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %671, align 32
  %_115.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %672, align 32
  %_118.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %673, align 32
  %_123.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row19.i.i.i225.i, align 32
  %_126.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %674, align 32
  %_129.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %675, align 32
  %_132.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %676, align 32
  %_137.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row20.i.i.i226.i, align 32
  %_140.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %677, align 32
  %_143.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %678, align 32
  %_146.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %679, align 32
  %_151.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row21.i.i.i227.i, align 32
  %_154.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %680, align 32
  %_157.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %681, align 32
  %_160.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %682, align 32
  %_165.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row22.i.i.i228.i, align 32
  %_168.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %683, align 32
  %_171.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %684, align 32
  %_174.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %685, align 32
  br label %bb5.i.i, !dbg !14922

bb5.i.i:                                          ; preds = %bb5.i.i.lr.ph, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853
  %iter.sroa.0.0.i.i7439 = phi i64 [ 0, %bb5.i.i.lr.ph ], [ %830, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853 ]
  %history.i.i.sroa.10.sroa.0.07438 = phi <8 x float> [ %history.i.i.sroa.10.sroa.0.0.copyload, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.0.07428, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853 ]
  %history.i.i.sroa.13.sroa.0.07437 = phi <8 x float> [ %history.i.i.sroa.13.sroa.0.0.copyload, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.10.sroa.0.07438, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853 ]
  %history.i.i.sroa.16.sroa.0.07436 = phi <8 x float> [ %history.i.i.sroa.16.sroa.0.0.copyload, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.13.sroa.0.07437, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853 ]
  %history.i.i.sroa.19.sroa.0.07435 = phi <8 x float> [ %history.i.i.sroa.19.sroa.0.0.copyload, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.16.sroa.0.07436, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853 ]
  %history.i.i.sroa.22.sroa.0.07434 = phi <8 x float> [ %history.i.i.sroa.22.sroa.0.0.copyload, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.19.sroa.0.07435, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853 ]
  %history.i.i.sroa.38.sroa.0.07433 = phi <8 x float> [ %history.i.i.sroa.38.sroa.0.0.copyload, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.35.sroa.0.07432, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853 ]
  %history.i.i.sroa.35.sroa.0.07432 = phi <8 x float> [ %history.i.i.sroa.35.sroa.0.0.copyload, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.32.sroa.0.07431, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853 ]
  %history.i.i.sroa.32.sroa.0.07431 = phi <8 x float> [ %history.i.i.sroa.32.sroa.0.0.copyload, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.29.sroa.0.07430, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853 ]
  %history.i.i.sroa.29.sroa.0.07430 = phi <8 x float> [ %history.i.i.sroa.29.sroa.0.0.copyload, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.25.sroa.0.07429, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853 ]
  %history.i.i.sroa.25.sroa.0.07429 = phi <8 x float> [ %history.i.i.sroa.25.sroa.0.0.copyload, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.22.sroa.0.07434, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853 ]
  %history.i.i.sroa.0.07428 = phi <8 x float> [ %history.i.i.sroa.0.0.copyload, %bb5.i.i.lr.ph ], [ %lanes.i2473.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853 ]
  %830 = add nuw nsw i64 %iter.sroa.0.0.i.i7439, 1, !dbg !14925
  %_11.i.i = add nuw nsw i64 %iter.sroa.0.0.i.i7439, %iter3.sroa.0.0.i7472, !dbg !14928
  %base.i.i = shl i64 %_11.i.i, 3, !dbg !14928
  %_24.i.i = icmp samesign ugt i64 %base.i.i, %right_io.1, !dbg !14929
  br i1 %_24.i.i, label %bb7.i.i, label %bb8.i.i, !dbg !14929, !prof !905

bb8.i.i:                                          ; preds = %bb5.i.i
  %_27.i.i = sub nuw nsw i64 %right_io.1, %base.i.i, !dbg !14932
  %_8.i2476 = icmp samesign ugt i64 %_27.i.i, 7, !dbg !14933
  br i1 %_8.i2476, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853, label %bb2.i2477, !dbg !14933, !prof !1076

bb2.i2477:                                        ; preds = %bb8.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_27.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #31, !dbg !14938, !noalias !14939
  unreachable, !dbg !14938

bb7.i.i:                                          ; preds = %bb5.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i.i, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e0592aef22128a0ac53753b9632a8183) #31, !dbg !14946, !noalias !14947
  unreachable, !dbg !14946

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853: ; preds = %bb8.i.i
  %_31.i.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %base.i.i, !dbg !14948
  %lanes.i2473.sroa.0.0.copyload = load <8 x float>, ptr %_31.i.i, align 4, !dbg !14950, !alias.scope !14954, !noalias !14958
  %831 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i.i.sroa.22.sroa.0.07434), !dbg !14960
  %832 = fmul <8 x float> %lanes.i2473.sroa.0.0.copyload, %_5.i2188, !dbg !14967
  %833 = fadd <8 x float> %832, zeroinitializer, !dbg !14973
  %834 = fmul <8 x float> %lanes.i2473.sroa.0.0.copyload, %_14.i.i.i.i.sroa.0.0.copyload, !dbg !14978
  %835 = fadd <8 x float> %834, zeroinitializer, !dbg !14983
  %836 = fmul <8 x float> %lanes.i2473.sroa.0.0.copyload, %_17.i.i.i.i.sroa.0.0.copyload, !dbg !14988
  %837 = fadd <8 x float> %836, zeroinitializer, !dbg !14993
  %838 = fmul <8 x float> %lanes.i2473.sroa.0.0.copyload, %_20.i.i.i.i.sroa.0.0.copyload, !dbg !14998
  %839 = fadd <8 x float> %838, zeroinitializer, !dbg !15003
  %840 = fmul <8 x float> %history.i.i.sroa.0.07428, %_25.i.i.i.i.sroa.0.0.copyload, !dbg !15008
  %841 = fadd <8 x float> %833, %840, !dbg !15013
  %842 = fmul <8 x float> %history.i.i.sroa.0.07428, %_28.i.i.i.i.sroa.0.0.copyload, !dbg !15018
  %843 = fadd <8 x float> %835, %842, !dbg !15023
  %844 = fmul <8 x float> %history.i.i.sroa.0.07428, %_31.i.i.i.i.sroa.0.0.copyload, !dbg !15028
  %845 = fadd <8 x float> %837, %844, !dbg !15033
  %846 = fmul <8 x float> %history.i.i.sroa.0.07428, %_34.i.i.i.i.sroa.0.0.copyload, !dbg !15038
  %847 = fadd <8 x float> %839, %846, !dbg !15043
  %848 = fmul <8 x float> %history.i.i.sroa.10.sroa.0.07438, %_39.i.i.i.i.sroa.0.0.copyload, !dbg !15048
  %849 = fadd <8 x float> %841, %848, !dbg !15053
  %850 = fmul <8 x float> %history.i.i.sroa.10.sroa.0.07438, %_42.i.i.i.i.sroa.0.0.copyload, !dbg !15058
  %851 = fadd <8 x float> %843, %850, !dbg !15063
  %852 = fmul <8 x float> %history.i.i.sroa.10.sroa.0.07438, %_45.i.i.i.i.sroa.0.0.copyload, !dbg !15068
  %853 = fadd <8 x float> %845, %852, !dbg !15073
  %854 = fmul <8 x float> %history.i.i.sroa.10.sroa.0.07438, %_48.i.i.i.i.sroa.0.0.copyload, !dbg !15078
  %855 = fadd <8 x float> %847, %854, !dbg !15083
  %856 = fmul <8 x float> %history.i.i.sroa.13.sroa.0.07437, %_53.i.i.i.i.sroa.0.0.copyload, !dbg !15088
  %857 = fadd <8 x float> %849, %856, !dbg !15093
  %858 = fmul <8 x float> %history.i.i.sroa.13.sroa.0.07437, %_56.i.i.i.i.sroa.0.0.copyload, !dbg !15098
  %859 = fadd <8 x float> %851, %858, !dbg !15103
  %860 = fmul <8 x float> %history.i.i.sroa.13.sroa.0.07437, %_59.i.i.i.i.sroa.0.0.copyload, !dbg !15108
  %861 = fadd <8 x float> %853, %860, !dbg !15113
  %862 = fmul <8 x float> %history.i.i.sroa.13.sroa.0.07437, %_62.i.i.i.i.sroa.0.0.copyload, !dbg !15118
  %863 = fadd <8 x float> %855, %862, !dbg !15123
  %864 = fmul <8 x float> %history.i.i.sroa.16.sroa.0.07436, %_67.i.i.i.i.sroa.0.0.copyload, !dbg !15128
  %865 = fadd <8 x float> %857, %864, !dbg !15133
  %866 = fmul <8 x float> %history.i.i.sroa.16.sroa.0.07436, %_70.i.i.i.i.sroa.0.0.copyload, !dbg !15138
  %867 = fadd <8 x float> %859, %866, !dbg !15143
  %868 = fmul <8 x float> %history.i.i.sroa.16.sroa.0.07436, %_73.i.i.i.i.sroa.0.0.copyload, !dbg !15148
  %869 = fadd <8 x float> %861, %868, !dbg !15153
  %870 = fmul <8 x float> %history.i.i.sroa.16.sroa.0.07436, %_76.i.i.i.i.sroa.0.0.copyload, !dbg !15158
  %871 = fadd <8 x float> %863, %870, !dbg !15163
  %872 = fmul <8 x float> %history.i.i.sroa.19.sroa.0.07435, %_81.i.i.i.i.sroa.0.0.copyload, !dbg !15168
  %873 = fadd <8 x float> %865, %872, !dbg !15173
  %874 = fmul <8 x float> %history.i.i.sroa.19.sroa.0.07435, %_84.i.i.i.i.sroa.0.0.copyload, !dbg !15178
  %875 = fadd <8 x float> %867, %874, !dbg !15183
  %876 = fmul <8 x float> %history.i.i.sroa.19.sroa.0.07435, %_87.i.i.i.i.sroa.0.0.copyload, !dbg !15188
  %877 = fadd <8 x float> %869, %876, !dbg !15193
  %878 = fmul <8 x float> %history.i.i.sroa.19.sroa.0.07435, %_90.i.i.i.i.sroa.0.0.copyload, !dbg !15198
  %879 = fadd <8 x float> %871, %878, !dbg !15203
  %880 = fmul <8 x float> %history.i.i.sroa.22.sroa.0.07434, %_95.i.i.i.i.sroa.0.0.copyload, !dbg !15208
  %881 = fadd <8 x float> %873, %880, !dbg !15213
  %882 = fmul <8 x float> %history.i.i.sroa.22.sroa.0.07434, %_98.i.i.i.i.sroa.0.0.copyload, !dbg !15218
  %883 = fadd <8 x float> %875, %882, !dbg !15223
  %884 = fmul <8 x float> %history.i.i.sroa.22.sroa.0.07434, %_101.i.i.i.i.sroa.0.0.copyload, !dbg !15228
  %885 = fadd <8 x float> %877, %884, !dbg !15233
  %886 = fmul <8 x float> %history.i.i.sroa.22.sroa.0.07434, %_104.i.i.i.i.sroa.0.0.copyload, !dbg !15238
  %887 = fadd <8 x float> %879, %886, !dbg !15243
  %888 = fmul <8 x float> %history.i.i.sroa.25.sroa.0.07429, %_109.i.i.i.i.sroa.0.0.copyload, !dbg !15248
  %889 = fadd <8 x float> %881, %888, !dbg !15253
  %890 = fmul <8 x float> %history.i.i.sroa.25.sroa.0.07429, %_112.i.i.i.i.sroa.0.0.copyload, !dbg !15258
  %891 = fadd <8 x float> %883, %890, !dbg !15263
  %892 = fmul <8 x float> %history.i.i.sroa.25.sroa.0.07429, %_115.i.i.i.i.sroa.0.0.copyload, !dbg !15268
  %893 = fadd <8 x float> %885, %892, !dbg !15273
  %894 = fmul <8 x float> %history.i.i.sroa.25.sroa.0.07429, %_118.i.i.i.i.sroa.0.0.copyload, !dbg !15278
  %895 = fadd <8 x float> %887, %894, !dbg !15283
  %896 = fmul <8 x float> %history.i.i.sroa.29.sroa.0.07430, %_123.i.i.i.i.sroa.0.0.copyload, !dbg !15288
  %897 = fadd <8 x float> %889, %896, !dbg !15293
  %898 = fmul <8 x float> %history.i.i.sroa.29.sroa.0.07430, %_126.i.i.i.i.sroa.0.0.copyload, !dbg !15298
  %899 = fadd <8 x float> %891, %898, !dbg !15303
  %900 = fmul <8 x float> %history.i.i.sroa.29.sroa.0.07430, %_129.i.i.i.i.sroa.0.0.copyload, !dbg !15308
  %901 = fadd <8 x float> %893, %900, !dbg !15313
  %902 = fmul <8 x float> %history.i.i.sroa.29.sroa.0.07430, %_132.i.i.i.i.sroa.0.0.copyload, !dbg !15318
  %903 = fadd <8 x float> %895, %902, !dbg !15323
  %904 = fmul <8 x float> %history.i.i.sroa.32.sroa.0.07431, %_137.i.i.i.i.sroa.0.0.copyload, !dbg !15328
  %905 = fadd <8 x float> %897, %904, !dbg !15333
  %906 = fmul <8 x float> %history.i.i.sroa.32.sroa.0.07431, %_140.i.i.i.i.sroa.0.0.copyload, !dbg !15338
  %907 = fadd <8 x float> %899, %906, !dbg !15343
  %908 = fmul <8 x float> %history.i.i.sroa.32.sroa.0.07431, %_143.i.i.i.i.sroa.0.0.copyload, !dbg !15348
  %909 = fadd <8 x float> %901, %908, !dbg !15353
  %910 = fmul <8 x float> %history.i.i.sroa.32.sroa.0.07431, %_146.i.i.i.i.sroa.0.0.copyload, !dbg !15358
  %911 = fadd <8 x float> %903, %910, !dbg !15363
  %912 = fmul <8 x float> %history.i.i.sroa.35.sroa.0.07432, %_151.i.i.i.i.sroa.0.0.copyload, !dbg !15368
  %913 = fadd <8 x float> %905, %912, !dbg !15373
  %914 = fmul <8 x float> %history.i.i.sroa.35.sroa.0.07432, %_154.i.i.i.i.sroa.0.0.copyload, !dbg !15378
  %915 = fadd <8 x float> %907, %914, !dbg !15383
  %916 = fmul <8 x float> %history.i.i.sroa.35.sroa.0.07432, %_157.i.i.i.i.sroa.0.0.copyload, !dbg !15388
  %917 = fadd <8 x float> %909, %916, !dbg !15393
  %918 = fmul <8 x float> %history.i.i.sroa.35.sroa.0.07432, %_160.i.i.i.i.sroa.0.0.copyload, !dbg !15398
  %919 = fadd <8 x float> %911, %918, !dbg !15403
  %920 = fmul <8 x float> %history.i.i.sroa.38.sroa.0.07433, %_165.i.i.i.i.sroa.0.0.copyload, !dbg !15408
  %921 = fadd <8 x float> %913, %920, !dbg !15413
  %922 = fmul <8 x float> %history.i.i.sroa.38.sroa.0.07433, %_168.i.i.i.i.sroa.0.0.copyload, !dbg !15418
  %923 = fadd <8 x float> %915, %922, !dbg !15423
  %924 = fmul <8 x float> %history.i.i.sroa.38.sroa.0.07433, %_171.i.i.i.i.sroa.0.0.copyload, !dbg !15428
  %925 = fadd <8 x float> %917, %924, !dbg !15433
  %926 = fmul <8 x float> %history.i.i.sroa.38.sroa.0.07433, %_174.i.i.i.i.sroa.0.0.copyload, !dbg !15438
  %927 = fadd <8 x float> %919, %926, !dbg !15443
  %928 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %921), !dbg !15448
  %929 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %831, <8 x float> %928), !dbg !15454
  %930 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %923), !dbg !15448
  %931 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %929, <8 x float> %930), !dbg !15454
  %932 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %925), !dbg !15448
  %933 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %931, <8 x float> %932), !dbg !15454
  %934 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %927), !dbg !15448
  %935 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %933, <8 x float> %934), !dbg !15454
  %_39.i.i.idx = shl i64 %iter.sroa.0.0.i.i7439, 5, !dbg !15459
  %_39.i.i = getelementptr inbounds nuw i8, ptr %peaks_right.i, i64 %_39.i.i.idx, !dbg !15459
  store <8 x float> %935, ptr %_39.i.i, align 4, !dbg !15464, !alias.scope !15469, !noalias !15473
  %exitcond8402.not = icmp eq i64 %830, %umax8401, !dbg !15477
  br i1 %exitcond8402.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, label %bb5.i.i, !dbg !14922

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit242.i
  %history.i.i.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit242.i ], [ %lanes.i2473.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853 ], !dbg !15479
  %history.i.i.sroa.25.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.25.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit242.i ], [ %history.i.i.sroa.22.sroa.0.07434, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853 ], !dbg !15479
  %history.i.i.sroa.29.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.29.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit242.i ], [ %history.i.i.sroa.25.sroa.0.07429, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853 ], !dbg !15479
  %history.i.i.sroa.32.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.32.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit242.i ], [ %history.i.i.sroa.29.sroa.0.07430, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853 ], !dbg !15479
  %history.i.i.sroa.35.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.35.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit242.i ], [ %history.i.i.sroa.32.sroa.0.07431, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853 ], !dbg !15479
  %history.i.i.sroa.38.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.38.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit242.i ], [ %history.i.i.sroa.35.sroa.0.07432, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853 ], !dbg !15479
  %history.i.i.sroa.41.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.41.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit242.i ], [ %history.i.i.sroa.38.sroa.0.07433, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853 ], !dbg !15479
  %history.i.i.sroa.22.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.22.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit242.i ], [ %history.i.i.sroa.19.sroa.0.07435, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853 ], !dbg !15479
  %history.i.i.sroa.19.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.19.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit242.i ], [ %history.i.i.sroa.16.sroa.0.07436, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853 ], !dbg !15479
  %history.i.i.sroa.16.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.16.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit242.i ], [ %history.i.i.sroa.13.sroa.0.07437, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853 ], !dbg !15479
  %history.i.i.sroa.13.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.13.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit242.i ], [ %history.i.i.sroa.10.sroa.0.07438, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853 ], !dbg !15479
  %history.i.i.sroa.10.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.10.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit242.i ], [ %history.i.i.sroa.0.07428, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2853 ], !dbg !15479
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !15480
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !15480
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !15480
  store <8 x float> %history.i.i.sroa.16.sroa.0.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 32, !dbg !15480
  store <8 x float> %history.i.i.sroa.19.sroa.0.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 32, !dbg !15480
  store <8 x float> %history.i.i.sroa.22.sroa.0.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 32, !dbg !15480
  store <8 x float> %history.i.i.sroa.25.sroa.0.0.lcssa, ptr %history.i.i.sroa.25.0.hot_right.i.sroa_idx, align 32, !dbg !15480
  store <8 x float> %history.i.i.sroa.29.sroa.0.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 32, !dbg !15480
  store <8 x float> %history.i.i.sroa.32.sroa.0.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 32, !dbg !15480
  store <8 x float> %history.i.i.sroa.35.sroa.0.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 32, !dbg !15480
  store <8 x float> %history.i.i.sroa.38.sroa.0.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 32, !dbg !15480
  store <8 x float> %history.i.i.sroa.41.sroa.0.0.lcssa, ptr %history.i.i.sroa.41.0.hot_right.i.sroa_idx, align 32, !dbg !15480
  br i1 %_20.i210.i7400.not, label %bb27.i.loopexit, label %bb32.i.preheader, !dbg !15481

bb32.i.preheader:                                 ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i
  %_72.i.sroa.3.0.copyload.pre = load i64, ptr %_72.i.sroa.3.0..sroa_idx, align 8, !dbg !15483, !noalias !14306
  %_72.i.sroa.4.0.copyload.pre = load i64, ptr %_72.i.sroa.4.0..sroa_idx, align 16, !dbg !15483, !noalias !14306
  %_73.i.sroa.3.0.copyload.pre = load i64, ptr %_73.i.sroa.3.0..sroa_idx, align 8, !dbg !15484, !noalias !14306
  %_73.i.sroa.4.0.copyload.pre = load i64, ptr %_73.i.sroa.4.0..sroa_idx, align 16, !dbg !15484, !noalias !14306
  %_54.0.i284.i = load ptr, ptr %702, align 32, !nonnull !12, !align !9542
  %_54.1.i285.i = load i64, ptr %703, align 8
  %_18.i296.i = load i64, ptr %686, align 16
  %_56.0.i300.i = load ptr, ptr %704, align 16, !nonnull !12, !align !9542
  %_56.1.i301.i = load i64, ptr %705, align 8
  %_58.1.i313.i = load i64, ptr %709, align 8
  %_58.0.i312.i = load ptr, ptr %710, align 32, !nonnull !12, !align !9542
  %_54.0.i.i = load ptr, ptr %713, align 32, !nonnull !12, !align !9542
  %_54.1.i.i = load i64, ptr %714, align 8
  %_18.i247.i = load i64, ptr %687, align 16
  %_56.0.i.i = load ptr, ptr %715, align 16, !nonnull !12, !align !9542
  %_56.1.i.i = load i64, ptr %716, align 8
  %_58.1.i.i = load i64, ptr %720, align 8
  %_58.0.i.i = load ptr, ptr %721, align 32, !nonnull !12, !align !9542
  %_22.i299.i.promoted9688 = load i32, ptr %_22.i299.i, align 4
  %uniform_left.i.promoted9707 = load <8 x float>, ptr %uniform_left.i, align 32
  %_22.i.i.promoted9726 = load i32, ptr %_22.i.i, align 4
  %uniform_right.i.promoted9745 = load <8 x float>, ptr %uniform_right.i, align 32
  br label %bb32.i, !dbg !15485

bb32.i:                                           ; preds = %bb32.i.preheader, %bb44.i
  %minimum.i.i.sroa.0.0.lcssa96859747 = phi <8 x float> [ %uniform_right.i.promoted9745, %bb32.i.preheader ], [ %minimum.i.i.sroa.0.0.lcssa96859746, %bb44.i ]
  %storemerge.i.lcssa96719728 = phi i32 [ %_22.i.i.promoted9726, %bb32.i.preheader ], [ %storemerge.i.lcssa96719727, %bb44.i ]
  %minimum.i279.i.sroa.0.0.lcssa96559709 = phi <8 x float> [ %uniform_left.i.promoted9707, %bb32.i.preheader ], [ %minimum.i279.i.sroa.0.0.lcssa96559708, %bb44.i ]
  %storemerge.i324.lcssa96419690 = phi i32 [ %_22.i299.i.promoted9688, %bb32.i.preheader ], [ %storemerge.i324.lcssa96419689, %bb44.i ]
  %frame.sroa.0.0.i7466 = phi i64 [ 0, %bb32.i.preheader ], [ %_87.i, %bb44.i ]
  %main_cursor.sroa.0.1.i7465 = phi i64 [ %main_cursor.sroa.0.0.i7471, %bb32.i.preheader ], [ %main_cursor.sroa.0.2.i, %bb44.i ]
  %ring_cursor.sroa.0.1.i7464 = phi i64 [ %ring_cursor.sroa.0.0.i7470, %bb32.i.preheader ], [ %ring_cursor.sroa.0.2.i, %bb44.i ]
  %_70.i = sub nuw nsw i64 %..i3298, %frame.sroa.0.0.i7466, !dbg !15496
  %ring.i596 = load i64, ptr %641, align 8, !dbg !15497, !alias.scope !15499, !noalias !15502, !noundef !12
  %main.i597 = load i64, ptr %642, align 8, !dbg !15506, !alias.scope !15499, !noalias !15502, !noundef !12
  %_10.i = add i64 %ring_cursor.sroa.0.1.i7464, 1, !dbg !15507
  %_45.not.i = icmp ult i64 %_10.i, %ring.i596, !dbg !15508
  %936 = select i1 %_45.not.i, i64 0, i64 %ring.i596, !dbg !15508
  %start1.sroa.0.0.i598 = sub nuw i64 %_10.i, %936, !dbg !15508
  %_12.i600 = add i64 %_72.i.sroa.3.0.copyload.pre, %ring_cursor.sroa.0.1.i7464, !dbg !15510
  %_46.not.i = icmp ult i64 %_12.i600, %ring.i596, !dbg !15511
  %937 = select i1 %_46.not.i, i64 0, i64 %ring.i596, !dbg !15511
  %left_end.sroa.0.0.i = sub nuw i64 %_12.i600, %937, !dbg !15511
  %_15.i602 = add i64 %_73.i.sroa.3.0.copyload.pre, %ring_cursor.sroa.0.1.i7464, !dbg !15513
  %_47.not.i = icmp ult i64 %_15.i602, %ring.i596, !dbg !15514
  %938 = select i1 %_47.not.i, i64 0, i64 %ring.i596, !dbg !15514
  %right_end.sroa.0.0.i = sub nuw i64 %_15.i602, %938, !dbg !15514
  %_18.i604 = add i64 %_72.i.sroa.4.0.copyload.pre, %ring_cursor.sroa.0.1.i7464, !dbg !15516
  %_48.not.i = icmp ult i64 %_18.i604, %ring.i596, !dbg !15517
  %939 = select i1 %_48.not.i, i64 0, i64 %ring.i596, !dbg !15517
  %left_expiring.sroa.0.0.i = sub nuw i64 %_18.i604, %939, !dbg !15517
  %_21.i606 = add i64 %_73.i.sroa.4.0.copyload.pre, %ring_cursor.sroa.0.1.i7464, !dbg !15519
  %_49.not.i = icmp ult i64 %_21.i606, %ring.i596, !dbg !15520
  %940 = select i1 %_49.not.i, i64 0, i64 %ring.i596, !dbg !15520
  %right_expiring.sroa.0.0.i = sub nuw i64 %_21.i606, %940, !dbg !15520
  %_30.i607 = sub i64 %ring.i596, %ring_cursor.sroa.0.1.i7464, !dbg !15522
  %..i3323 = tail call noundef i64 @llvm.umin.i64(i64 %_30.i607, i64 %_70.i), !dbg !15523
  %_31.i = sub i64 %main.i597, %main_cursor.sroa.0.1.i7465, !dbg !15525
  %..i3324 = tail call noundef i64 @llvm.umin.i64(i64 %_31.i, i64 %..i3323), !dbg !15526
  %_32.i610 = sub i64 %ring.i596, %start1.sroa.0.0.i598, !dbg !15528
  %..i3325 = tail call noundef i64 @llvm.umin.i64(i64 %_32.i610, i64 %..i3324), !dbg !15529
  %_34.i612 = sub i64 %ring.i596, %left_end.sroa.0.0.i, !dbg !15531
  %..i3326 = tail call noundef i64 @llvm.umin.i64(i64 %_34.i612, i64 %..i3325), !dbg !15532
  %_36.i = sub i64 %ring.i596, %right_end.sroa.0.0.i, !dbg !15534
  %..i3327 = tail call noundef i64 @llvm.umin.i64(i64 %_36.i, i64 %..i3326), !dbg !15535
  %_38.i614 = sub i64 %ring.i596, %left_expiring.sroa.0.0.i, !dbg !15537
  %..i3328 = tail call noundef i64 @llvm.umin.i64(i64 %_38.i614, i64 %..i3327), !dbg !15538
  %_40.i615 = sub i64 %ring.i596, %right_expiring.sroa.0.0.i, !dbg !15540
  %..i3329 = tail call noundef i64 @llvm.umin.i64(i64 %_40.i615, i64 %..i3328), !dbg !15541
  %_76.i = add i64 %frame.sroa.0.0.i7466, %iter3.sroa.0.0.i7472, !dbg !15543
  %base.i = shl i64 %_76.i, 3, !dbg !15543
  %base.i6242 = add i64 %..i3329, %_76.i, !dbg !15544
  %_80.i = shl i64 %base.i6242, 3, !dbg !15544
  %_184.i = icmp ult i64 %_80.i, %base.i, !dbg !15485
  %_178.not.i = icmp ugt i64 %_80.i, %left_io.1
  %or.cond.i = or i1 %_184.i, %_178.not.i, !dbg !15485
  br i1 %or.cond.i, label %bb72.i, label %bb70.i, !dbg !15485, !prof !5262

bb72.i:                                           ; preds = %bb32.i
  store i32 %storemerge.i324.lcssa96419690, ptr %_22.i299.i, align 4
  store <8 x float> %minimum.i279.i.sroa.0.0.lcssa96559709, ptr %uniform_left.i, align 32
  store i32 %storemerge.i.lcssa96719728, ptr %_22.i.i, align 4
  store <8 x float> %minimum.i.i.sroa.0.0.lcssa96859747, ptr %uniform_right.i, align 32
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i, i64 noundef %_80.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_14e3d3ba493695bccf4dcde9be821bfc) #31, !dbg !15545, !noalias !14173
  unreachable, !dbg !15545

bb70.i:                                           ; preds = %bb32.i
  %_187.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %base.i, !dbg !15546
  %_188.not.i = icmp ugt i64 %_80.i, %right_io.1, !dbg !15550
  br i1 %_188.not.i, label %bb75.i, label %bb74.i, !dbg !15550, !prof !905

bb75.i:                                           ; preds = %bb70.i
  store i32 %storemerge.i324.lcssa96419690, ptr %_22.i299.i, align 4
  store <8 x float> %minimum.i279.i.sroa.0.0.lcssa96559709, ptr %uniform_left.i, align 32
  store i32 %storemerge.i.lcssa96719728, ptr %_22.i.i, align 4
  store <8 x float> %minimum.i.i.sroa.0.0.lcssa96859747, ptr %uniform_right.i, align 32
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i, i64 noundef %_80.i, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8706d23ef0096dbeb31c5991bb003668) #31, !dbg !15555, !noalias !14173
  unreachable, !dbg !15555

bb74.i:                                           ; preds = %bb70.i
  %_195.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %base.i, !dbg !15556
  %_84.i = shl nuw nsw i64 %frame.sroa.0.0.i7466, 3, !dbg !15560
  %_87.i = add nuw nsw i64 %..i3329, %frame.sroa.0.0.i7466, !dbg !15562
  %_197.i = icmp ult i64 %_87.i, 33
  br i1 %_197.i, label %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit, label %bb77.i, !dbg !15563, !prof !9914

bb77.i:                                           ; preds = %bb74.i
  store i32 %storemerge.i324.lcssa96419690, ptr %_22.i299.i, align 4
  store <8 x float> %minimum.i279.i.sroa.0.0.lcssa96559709, ptr %uniform_left.i, align 32
  store i32 %storemerge.i.lcssa96719728, ptr %_22.i.i, align 4
  store <8 x float> %minimum.i.i.sroa.0.0.lcssa96859747, ptr %uniform_right.i, align 32
  %_86.i = shl nuw nsw i64 %_87.i, 3, !dbg !15562
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_84.i, i64 noundef %_86.i, i64 noundef 256, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f81d29d0a485451b3eceb8bcdce402aa) #31, !dbg !15571, !noalias !14173
  unreachable, !dbg !15571

_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit: ; preds = %bb74.i
  %_204.i = getelementptr inbounds nuw float, ptr %peaks_left.i, i64 %_84.i, !dbg !15572
  %_213.i = getelementptr inbounds nuw float, ptr %peaks_right.i, i64 %_84.i, !dbg !15576
  %_2.i.i.i7460.not = icmp eq i64 %..i3329, 0, !dbg !15586
  br i1 %_2.i.i.i7460.not, label %bb44.i, label %bb43.i.preheader, !dbg !15586

bb43.i.preheader:                                 ; preds = %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit
  %umin8419 = call i64 @llvm.umin.i64(i64 %_34.i612, i64 %_36.i), !dbg !15592
  %umin8420 = call i64 @llvm.umin.i64(i64 %umin8419, i64 %_38.i614), !dbg !15592
  %umin8421 = call i64 @llvm.umin.i64(i64 %umin8420, i64 %_40.i615), !dbg !15592
  %umin8422 = call i64 @llvm.umin.i64(i64 %umin8421, i64 %_32.i610), !dbg !15592
  %umin8423 = call i64 @llvm.umin.i64(i64 %umin8422, i64 %_30.i607), !dbg !15592
  %umin8424 = call i64 @llvm.umin.i64(i64 %umin8423, i64 %_31.i), !dbg !15592
  %941 = sub nsw i64 %umin8425, %frame.sroa.0.0.i7466, !dbg !15592
  %umin8426 = call i64 @llvm.umin.i64(i64 %umin8424, i64 %941), !dbg !15592
  %942 = and i64 %umin8426, 2305843009213693951, !dbg !15592
  %_13.i710.sroa.0.0.copyload = load <8 x float>, ptr %690, align 32
  %_13.i696.sroa.0.0.copyload = load <8 x float>, ptr %693, align 32
  %_13.i682.sroa.0.0.copyload = load <8 x float>, ptr %696, align 32
  %_13.i668.sroa.0.0.copyload = load <8 x float>, ptr %699, align 32
  %_37.i267.i.sroa.0.0.copyload = load <8 x float>, ptr %707, align 32
  %_37.i.i.sroa.0.0.copyload = load <8 x float>, ptr %718, align 32
  %.promoted9518 = load <8 x float>, ptr %688, align 32
  %_115.i.promoted = load <8 x float>, ptr %_115.i, align 32
  %.promoted9550 = load <8 x float>, ptr %689, align 32
  %.promoted9567 = load <8 x float>, ptr %691, align 32
  %_117.i.promoted = load <8 x float>, ptr %_117.i, align 32
  %.promoted9570 = load <8 x float>, ptr %692, align 32
  %.promoted9573 = load <8 x float>, ptr %694, align 32
  %_119.i.promoted = load <8 x float>, ptr %_119.i, align 32
  %.promoted9605 = load <8 x float>, ptr %695, align 32
  %.promoted9622 = load <8 x float>, ptr %697, align 32
  %_121.i.promoted = load <8 x float>, ptr %_121.i, align 32
  %.promoted9625 = load <8 x float>, ptr %698, align 32
  %.promoted9656 = load <8 x float>, ptr %706, align 32
  %.promoted9686 = load <8 x float>, ptr %717, align 32
  br label %bb43.i, !dbg !15592

bb43.i:                                           ; preds = %bb43.i.preheader, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit420
  %_33.i243.i.sroa.0.0.copyload9687 = phi <8 x float> [ %.promoted9686, %bb43.i.preheader ], [ %1048, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit420 ]
  %minimum.i.i.sroa.0.09672 = phi <8 x float> [ %minimum.i.i.sroa.0.0.lcssa96859747, %bb43.i.preheader ], [ %minimum.i.i.sroa.0.0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit420 ]
  %storemerge.i9658 = phi i32 [ %storemerge.i.lcssa96719728, %bb43.i.preheader ], [ %storemerge.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit420 ]
  %_33.i270.i.sroa.0.0.copyload9657 = phi <8 x float> [ %.promoted9656, %bb43.i.preheader ], [ %1010, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit420 ]
  %minimum.i279.i.sroa.0.09642 = phi <8 x float> [ %minimum.i279.i.sroa.0.0.lcssa96559709, %bb43.i.preheader ], [ %minimum.i279.i.sroa.0.0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit420 ]
  %storemerge.i3249628 = phi i32 [ %storemerge.i324.lcssa96419690, %bb43.i.preheader ], [ %storemerge.i324, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit420 ]
  %_12.i669.sroa.0.0.copyload9627 = phi <8 x float> [ %.promoted9625, %bb43.i.preheader ], [ %_12.i669.sroa.0.0.copyload9626, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit420 ]
  %release_right.i.sroa.0.0.copyload9624 = phi <8 x float> [ %_121.i.promoted, %bb43.i.preheader ], [ %release_right.i.sroa.0.0.copyload9623, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit420 ]
  %943 = phi <8 x float> [ %.promoted9622, %bb43.i.preheader ], [ %982, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit420 ]
  %_12.i683.sroa.0.0.copyload9607 = phi <8 x float> [ %.promoted9605, %bb43.i.preheader ], [ %_12.i683.sroa.0.0.copyload9606, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit420 ]
  %limit_right.i.sroa.0.0.copyload35379590 = phi <8 x float> [ %_119.i.promoted, %bb43.i.preheader ], [ %limit_right.i.sroa.0.0.copyload35379589, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit420 ]
  %944 = phi <8 x float> [ %.promoted9573, %bb43.i.preheader ], [ %983, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit420 ]
  %_12.i697.sroa.0.0.copyload9572 = phi <8 x float> [ %.promoted9570, %bb43.i.preheader ], [ %_12.i697.sroa.0.0.copyload9571, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit420 ]
  %release_left.i.sroa.0.0.copyload9569 = phi <8 x float> [ %_117.i.promoted, %bb43.i.preheader ], [ %release_left.i.sroa.0.0.copyload9568, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit420 ]
  %945 = phi <8 x float> [ %.promoted9567, %bb43.i.preheader ], [ %984, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit420 ]
  %_12.i711.sroa.0.0.copyload9552 = phi <8 x float> [ %.promoted9550, %bb43.i.preheader ], [ %_12.i711.sroa.0.0.copyload9551, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit420 ]
  %limit_left.i.sroa.0.0.copyload35349535 = phi <8 x float> [ %_115.i.promoted, %bb43.i.preheader ], [ %limit_left.i.sroa.0.0.copyload35349534, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit420 ]
  %946 = phi <8 x float> [ %.promoted9518, %bb43.i.preheader ], [ %985, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit420 ]
  %iter.i.sroa.36.07462 = phi i64 [ 0, %bb43.i.preheader ], [ %947, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit420 ]
  %947 = add nuw nsw i64 %iter.i.sroa.36.07462, 1, !dbg !15594
  %start1.i.i.i.i.i.i.i.i = shl i64 %iter.i.sroa.36.07462, 3, !dbg !15595
  %data.i.i.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_187.i, i64 %start1.i.i.i.i.i.i.i.i, !dbg !15601
  %data.i.i.i.i3381 = getelementptr inbounds nuw float, ptr %_213.i, i64 %start1.i.i.i.i.i.i.i.i, !dbg !15603
  %data.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_204.i, i64 %start1.i.i.i.i.i.i.i.i, !dbg !15606
  %data.i5.i.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_195.i, i64 %start1.i.i.i.i.i.i.i.i, !dbg !15609
  br i1 %stationary.sroa.0.0.i, label %bb51.i, label %bb46.i, !dbg !15592

bb44.i.loopexit:                                  ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit420
  store <8 x float> %985, ptr %688, align 32, !dbg !15612
  store <8 x float> %limit_left.i.sroa.0.0.copyload35349534, ptr %_115.i, align 32, !dbg !15614
  store <8 x float> %_12.i711.sroa.0.0.copyload9551, ptr %689, align 32, !dbg !15615
  store <8 x float> %983, ptr %694, align 32, !dbg !15616
  store <8 x float> %limit_right.i.sroa.0.0.copyload35379589, ptr %_119.i, align 32, !dbg !15614
  store <8 x float> %_12.i683.sroa.0.0.copyload9606, ptr %695, align 32, !dbg !15618
  br label %bb44.i, !dbg !15619

bb44.i:                                           ; preds = %bb44.i.loopexit, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit
  %minimum.i.i.sroa.0.0.lcssa96859746 = phi <8 x float> [ %minimum.i.i.sroa.0.0, %bb44.i.loopexit ], [ %minimum.i.i.sroa.0.0.lcssa96859747, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %storemerge.i.lcssa96719727 = phi i32 [ %storemerge.i, %bb44.i.loopexit ], [ %storemerge.i.lcssa96719728, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %minimum.i279.i.sroa.0.0.lcssa96559708 = phi <8 x float> [ %minimum.i279.i.sroa.0.0, %bb44.i.loopexit ], [ %minimum.i279.i.sroa.0.0.lcssa96559709, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %storemerge.i324.lcssa96419689 = phi i32 [ %storemerge.i324, %bb44.i.loopexit ], [ %storemerge.i324.lcssa96419690, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_144.i = add i64 %..i3329, %ring_cursor.sroa.0.1.i7464, !dbg !15619
  %_214.not.i = icmp ult i64 %_144.i, %ring.i, !dbg !15620
  %948 = select i1 %_214.not.i, i64 0, i64 %ring.i, !dbg !15620
  %ring_cursor.sroa.0.2.i = sub nuw i64 %_144.i, %948, !dbg !15620
  %_146.i = add i64 %..i3329, %main_cursor.sroa.0.1.i7465, !dbg !15623
  %_225.not.i = icmp ult i64 %_146.i, %main.i, !dbg !15624
  %949 = select i1 %_225.not.i, i64 0, i64 %main.i, !dbg !15624
  %main_cursor.sroa.0.2.i = sub nuw i64 %_146.i, %949, !dbg !15624
  %_65.i = icmp ult i64 %_87.i, %..i3298, !dbg !15481
  br i1 %_65.i, label %bb32.i, label %bb27.i.loopexit.loopexit, !dbg !15481

bb46.i:                                           ; preds = %bb43.i
  %950 = fadd <8 x float> %946, splat (float -1.000000e+00), !dbg !15626
  %951 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %950, <8 x float> zeroinitializer), !dbg !15631
  %952 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %951, <8 x float> zeroinitializer, i8 30), !dbg !15636
  %953 = fadd <8 x float> %limit_left.i.sroa.0.0.copyload35349535, %_12.i711.sroa.0.0.copyload9552, !dbg !15642
  %954 = bitcast <8 x float> %952 to <8 x i32>, !dbg !15647
  %955 = icmp slt <8 x i32> %954, zeroinitializer, !dbg !15651
  %956 = select <8 x i1> %955, <8 x float> %953, <8 x float> %_13.i710.sroa.0.0.copyload, !dbg !15651
  %957 = select <8 x i1> %955, <8 x float> %_12.i711.sroa.0.0.copyload9552, <8 x float> zeroinitializer, !dbg !15653
  %958 = fadd <8 x float> %945, splat (float -1.000000e+00), !dbg !15658
  %959 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %958, <8 x float> zeroinitializer), !dbg !15664
  store <8 x float> %959, ptr %691, align 32, !dbg !15669
  %960 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %959, <8 x float> zeroinitializer, i8 30), !dbg !15670
  %961 = fadd <8 x float> %release_left.i.sroa.0.0.copyload9569, %_12.i697.sroa.0.0.copyload9572, !dbg !15676
  %962 = bitcast <8 x float> %960 to <8 x i32>, !dbg !15681
  %963 = icmp slt <8 x i32> %962, zeroinitializer, !dbg !15685
  %964 = select <8 x i1> %963, <8 x float> %961, <8 x float> %_13.i696.sroa.0.0.copyload, !dbg !15685
  store <8 x float> %964, ptr %_117.i, align 32, !dbg !15687
  %965 = select <8 x i1> %963, <8 x float> %_12.i697.sroa.0.0.copyload9572, <8 x float> zeroinitializer, !dbg !15688
  store <8 x float> %965, ptr %692, align 32, !dbg !15693
  %966 = fadd <8 x float> %944, splat (float -1.000000e+00), !dbg !15694
  %967 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %966, <8 x float> zeroinitializer), !dbg !15699
  %968 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %967, <8 x float> zeroinitializer, i8 30), !dbg !15704
  %969 = fadd <8 x float> %limit_right.i.sroa.0.0.copyload35379590, %_12.i683.sroa.0.0.copyload9607, !dbg !15710
  %970 = bitcast <8 x float> %968 to <8 x i32>, !dbg !15715
  %971 = icmp slt <8 x i32> %970, zeroinitializer, !dbg !15719
  %972 = select <8 x i1> %971, <8 x float> %969, <8 x float> %_13.i682.sroa.0.0.copyload, !dbg !15719
  %973 = select <8 x i1> %971, <8 x float> %_12.i683.sroa.0.0.copyload9607, <8 x float> zeroinitializer, !dbg !15721
  %974 = fadd <8 x float> %943, splat (float -1.000000e+00), !dbg !15726
  %975 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %974, <8 x float> zeroinitializer), !dbg !15732
  store <8 x float> %975, ptr %697, align 32, !dbg !15737
  %976 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %975, <8 x float> zeroinitializer, i8 30), !dbg !15738
  %977 = fadd <8 x float> %release_right.i.sroa.0.0.copyload9624, %_12.i669.sroa.0.0.copyload9627, !dbg !15744
  %978 = bitcast <8 x float> %976 to <8 x i32>, !dbg !15749
  %979 = icmp slt <8 x i32> %978, zeroinitializer, !dbg !15753
  %980 = select <8 x i1> %979, <8 x float> %977, <8 x float> %_13.i668.sroa.0.0.copyload, !dbg !15753
  store <8 x float> %980, ptr %_121.i, align 32, !dbg !15755
  %981 = select <8 x i1> %979, <8 x float> %_12.i669.sroa.0.0.copyload9627, <8 x float> zeroinitializer, !dbg !15756
  store <8 x float> %981, ptr %698, align 32, !dbg !15761
  br label %bb51.i, !dbg !15762

bb51.i:                                           ; preds = %bb43.i, %bb46.i
  %_12.i669.sroa.0.0.copyload9626 = phi <8 x float> [ %981, %bb46.i ], [ %_12.i669.sroa.0.0.copyload9627, %bb43.i ]
  %release_right.i.sroa.0.0.copyload9623 = phi <8 x float> [ %980, %bb46.i ], [ %release_right.i.sroa.0.0.copyload9624, %bb43.i ]
  %982 = phi <8 x float> [ %975, %bb46.i ], [ %943, %bb43.i ]
  %_12.i683.sroa.0.0.copyload9606 = phi <8 x float> [ %973, %bb46.i ], [ %_12.i683.sroa.0.0.copyload9607, %bb43.i ]
  %limit_right.i.sroa.0.0.copyload35379589 = phi <8 x float> [ %972, %bb46.i ], [ %limit_right.i.sroa.0.0.copyload35379590, %bb43.i ]
  %983 = phi <8 x float> [ %967, %bb46.i ], [ %944, %bb43.i ]
  %_12.i697.sroa.0.0.copyload9571 = phi <8 x float> [ %965, %bb46.i ], [ %_12.i697.sroa.0.0.copyload9572, %bb43.i ]
  %release_left.i.sroa.0.0.copyload9568 = phi <8 x float> [ %964, %bb46.i ], [ %release_left.i.sroa.0.0.copyload9569, %bb43.i ]
  %984 = phi <8 x float> [ %959, %bb46.i ], [ %945, %bb43.i ]
  %_12.i711.sroa.0.0.copyload9551 = phi <8 x float> [ %957, %bb46.i ], [ %_12.i711.sroa.0.0.copyload9552, %bb43.i ]
  %limit_left.i.sroa.0.0.copyload35349534 = phi <8 x float> [ %956, %bb46.i ], [ %limit_left.i.sroa.0.0.copyload35349535, %bb43.i ]
  %985 = phi <8 x float> [ %951, %bb46.i ], [ %946, %bb43.i ]
  %lanes.i2509.sroa.0.0.copyload = load <8 x float>, ptr %data.i.i.i.i.i.i, align 4, !dbg !15763, !alias.scope !15769, !noalias !15773
  %lanes.i2500.sroa.0.0.copyload = load <8 x float>, ptr %data.i.i.i.i3381, align 4, !dbg !15777, !alias.scope !15783, !noalias !15787
  %986 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i2500.sroa.0.0.copyload, <8 x float> %lanes.i2509.sroa.0.0.copyload), !dbg !15791
  %987 = select <8 x i1> %701, <8 x float> %986, <8 x float> %lanes.i2500.sroa.0.0.copyload, !dbg !15797
  %_215.i = add i64 %iter.i.sroa.36.07462, %ring_cursor.sroa.0.1.i7464, !dbg !15804
  %_216.i = add i64 %iter.i.sroa.36.07462, %main_cursor.sroa.0.1.i7465, !dbg !15810
  %_217.i = add i64 %iter.i.sroa.36.07462, %left_end.sroa.0.0.i, !dbg !15811
  %_218.i = add i64 %iter.i.sroa.36.07462, %start1.sroa.0.0.i598, !dbg !15812
  %_219.i = add i64 %iter.i.sroa.36.07462, %left_expiring.sroa.0.0.i, !dbg !15813
  %base.i9.i287.i = shl i64 %_215.i, 3, !dbg !15814
  %_7.i10.i288.i = add i64 %base.i9.i287.i, 8, !dbg !15821
  %988 = or disjoint i64 %base.i9.i287.i, 7, !dbg !15823
  %or.cond.i13.i291.i.not = icmp ult i64 %988, %_54.1.i285.i, !dbg !15823
  br i1 %or.cond.i13.i291.i.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i292.i, label %bb4.i15.i325.i, !dbg !15823, !prof !9914

bb4.i15.i325.i:                                   ; preds = %bb51.i
  store i32 %storemerge.i324.lcssa96419690, ptr %_22.i299.i, align 4
  store <8 x float> %minimum.i279.i.sroa.0.0.lcssa96559709, ptr %uniform_left.i, align 32
  store i32 %storemerge.i.lcssa96719728, ptr %_22.i.i, align 4
  store <8 x float> %minimum.i.i.sroa.0.0.lcssa96859747, ptr %uniform_right.i, align 32
  store <8 x float> %985, ptr %688, align 32, !dbg !15612
  store <8 x float> %limit_left.i.sroa.0.0.copyload35349534, ptr %_115.i, align 32, !dbg !15614
  store <8 x float> %_12.i711.sroa.0.0.copyload9551, ptr %689, align 32, !dbg !15615
  store <8 x float> %983, ptr %694, align 32, !dbg !15616
  store <8 x float> %limit_right.i.sroa.0.0.copyload35379589, ptr %_119.i, align 32, !dbg !15614
  store <8 x float> %_12.i683.sroa.0.0.copyload9606, ptr %695, align 32, !dbg !15618
  store i32 %storemerge.i3249628, ptr %_22.i299.i, align 4, !dbg !15830
  store <8 x float> %minimum.i279.i.sroa.0.09642, ptr %uniform_left.i, align 32, !dbg !15834
  store i32 %storemerge.i9658, ptr %_22.i.i, align 4, !dbg !15836
  store <8 x float> %minimum.i.i.sroa.0.09672, ptr %uniform_right.i, align 32, !dbg !15839
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i287.i, i64 noundef %_7.i10.i288.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i285.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cd0e502fea74c9eb9984d521d7f3533e) #31, !dbg !15840, !noalias !15841
  unreachable, !dbg !15840

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i292.i: ; preds = %bb51.i
  %989 = select <8 x i1> %701, <8 x float> %986, <8 x float> %lanes.i2509.sroa.0.0.copyload, !dbg !15855
  %990 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %989, <8 x float> %limit_left.i.sroa.0.0.copyload35349534, i8 30), !dbg !15860
  %991 = bitcast <8 x float> %990 to <8 x i32>, !dbg !15866
  %992 = icmp slt <8 x i32> %991, zeroinitializer, !dbg !15870
  %993 = fdiv <8 x float> %limit_left.i.sroa.0.0.copyload35349534, %989, !dbg !15872
  %994 = select <8 x i1> %992, <8 x float> %993, <8 x float> splat (float 1.000000e+00), !dbg !15870
  %_17.i14.i293.i = getelementptr inbounds nuw float, ptr %_54.0.i284.i, i64 %base.i9.i287.i, !dbg !15877
  store <8 x float> %994, ptr %_17.i14.i293.i, align 4, !dbg !15881, !alias.scope !15886, !noalias !15890
  %base.i367 = shl i64 %_217.i, 3, !dbg !15894
  %995 = or disjoint i64 %base.i367, 7, !dbg !15897
  %or.cond.i371.not = icmp ult i64 %995, %_54.1.i285.i, !dbg !15897
  br i1 %or.cond.i371.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit375, label %bb4.i374, !dbg !15897, !prof !9914

bb4.i374:                                         ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i292.i
  store i32 %storemerge.i324.lcssa96419690, ptr %_22.i299.i, align 4
  store <8 x float> %minimum.i279.i.sroa.0.0.lcssa96559709, ptr %uniform_left.i, align 32
  store i32 %storemerge.i.lcssa96719728, ptr %_22.i.i, align 4
  store <8 x float> %minimum.i.i.sroa.0.0.lcssa96859747, ptr %uniform_right.i, align 32
  store <8 x float> %985, ptr %688, align 32, !dbg !15612
  store <8 x float> %limit_left.i.sroa.0.0.copyload35349534, ptr %_115.i, align 32, !dbg !15614
  store <8 x float> %_12.i711.sroa.0.0.copyload9551, ptr %689, align 32, !dbg !15615
  store <8 x float> %983, ptr %694, align 32, !dbg !15616
  store <8 x float> %limit_right.i.sroa.0.0.copyload35379589, ptr %_119.i, align 32, !dbg !15614
  store <8 x float> %_12.i683.sroa.0.0.copyload9606, ptr %695, align 32, !dbg !15618
  store i32 %storemerge.i3249628, ptr %_22.i299.i, align 4, !dbg !15830
  store <8 x float> %minimum.i279.i.sroa.0.09642, ptr %uniform_left.i, align 32, !dbg !15834
  store i32 %storemerge.i9658, ptr %_22.i.i, align 4, !dbg !15836
  store <8 x float> %minimum.i.i.sroa.0.09672, ptr %uniform_right.i, align 32, !dbg !15839
  %_5.i368 = add i64 %base.i367, 8, !dbg !15905
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i367, i64 noundef %_5.i368, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i285.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !15906, !noalias !15907
  unreachable, !dbg !15906

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit375: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i292.i
  %_15.i373 = getelementptr inbounds nuw float, ptr %_54.0.i284.i, i64 %base.i367, !dbg !15915
  %lanes.i2336.sroa.0.0.copyload = load <8 x float>, ptr %_15.i373, align 4, !dbg !15919, !alias.scope !15924, !noalias !15928
  %position.i318 = zext i32 %storemerge.i3249628 to i64, !dbg !15932
  %996 = icmp eq i32 %storemerge.i3249628, 0, !dbg !15933
  br i1 %996, label %bb5.i320, label %bb3.i319, !dbg !15933

bb3.i319:                                         ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit375
  %997 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %minimum.i279.i.sroa.0.09642, <8 x float> %lanes.i2336.sroa.0.0.copyload), !dbg !15934
  br label %bb5.i320, !dbg !15943

bb5.i320:                                         ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit375, %bb3.i319
  %minimum.i279.i.sroa.0.0 = phi <8 x float> [ %997, %bb3.i319 ], [ %lanes.i2336.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit375 ], !dbg !15834
  %_15.i321 = add nuw nsw i64 %position.i318, 1, !dbg !15944
  %complete.i322 = icmp eq i64 %_15.i321, %_18.i296.i, !dbg !15944
  br i1 %complete.i322, label %bb19.i330, label %bb7.i323, !dbg !15946

bb7.i323:                                         ; preds = %bb5.i320
  %base.i358 = shl i64 %_218.i, 3, !dbg !15948
  %998 = or disjoint i64 %base.i358, 7, !dbg !15950
  %or.cond.i362.not = icmp ult i64 %998, %_54.1.i285.i, !dbg !15950
  br i1 %or.cond.i362.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit366, label %bb4.i365, !dbg !15950, !prof !9914

bb4.i365:                                         ; preds = %bb7.i323
  store i32 %storemerge.i324.lcssa96419690, ptr %_22.i299.i, align 4
  store <8 x float> %minimum.i279.i.sroa.0.0.lcssa96559709, ptr %uniform_left.i, align 32
  store i32 %storemerge.i.lcssa96719728, ptr %_22.i.i, align 4
  store <8 x float> %minimum.i.i.sroa.0.0.lcssa96859747, ptr %uniform_right.i, align 32
  store <8 x float> %985, ptr %688, align 32, !dbg !15612
  store <8 x float> %limit_left.i.sroa.0.0.copyload35349534, ptr %_115.i, align 32, !dbg !15614
  store <8 x float> %_12.i711.sroa.0.0.copyload9551, ptr %689, align 32, !dbg !15615
  store <8 x float> %983, ptr %694, align 32, !dbg !15616
  store <8 x float> %limit_right.i.sroa.0.0.copyload35379589, ptr %_119.i, align 32, !dbg !15614
  store <8 x float> %_12.i683.sroa.0.0.copyload9606, ptr %695, align 32, !dbg !15618
  store i32 %storemerge.i3249628, ptr %_22.i299.i, align 4, !dbg !15830
  store <8 x float> %minimum.i279.i.sroa.0.0, ptr %uniform_left.i, align 32, !dbg !15834
  store i32 %storemerge.i9658, ptr %_22.i.i, align 4, !dbg !15836
  store <8 x float> %minimum.i.i.sroa.0.09672, ptr %uniform_right.i, align 32, !dbg !15839
  %_5.i359 = add i64 %base.i358, 8, !dbg !15954
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i358, i64 noundef %_5.i359, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i285.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !15955, !noalias !15956
  unreachable, !dbg !15955

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit366: ; preds = %bb7.i323
  %_15.i364 = getelementptr inbounds nuw float, ptr %_54.0.i284.i, i64 %base.i358, !dbg !15960
  %lanes.i2343.sroa.0.0.copyload = load <8 x float>, ptr %_15.i364, align 4, !dbg !15962, !alias.scope !15967, !noalias !15971
  %999 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %lanes.i2343.sroa.0.0.copyload, <8 x float> %minimum.i279.i.sroa.0.0), !dbg !15975
  %1000 = trunc i64 %_15.i321 to i32, !dbg !15980
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit342, !dbg !15982

bb19.i330:                                        ; preds = %bb5.i320, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit
  %end.sroa.0.0.i3287455 = phi i64 [ %1004, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit ], [ %_217.i, %bb5.i320 ]
  %iter.sroa.0.0.i3277454 = phi i64 [ %_30.i331, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit ], [ 0, %bb5.i320 ]
  %suffix.i311.sroa.0.07453 = phi <8 x float> [ %1002, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit ], [ %lanes.i2336.sroa.0.0.copyload, %bb5.i320 ]
  %base.i343 = shl i64 %end.sroa.0.0.i3287455, 3, !dbg !15983
  %1001 = or disjoint i64 %base.i343, 7, !dbg !15988
  %or.cond.i345.not = icmp ult i64 %1001, %_54.1.i285.i, !dbg !15988
  br i1 %or.cond.i345.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, label %bb4.i348, !dbg !15988, !prof !9914

bb4.i348:                                         ; preds = %bb19.i330
  store i32 %storemerge.i324.lcssa96419690, ptr %_22.i299.i, align 4
  store <8 x float> %minimum.i279.i.sroa.0.0.lcssa96559709, ptr %uniform_left.i, align 32
  store i32 %storemerge.i.lcssa96719728, ptr %_22.i.i, align 4
  store <8 x float> %minimum.i.i.sroa.0.0.lcssa96859747, ptr %uniform_right.i, align 32
  store <8 x float> %985, ptr %688, align 32, !dbg !15612
  store <8 x float> %limit_left.i.sroa.0.0.copyload35349534, ptr %_115.i, align 32, !dbg !15614
  store <8 x float> %_12.i711.sroa.0.0.copyload9551, ptr %689, align 32, !dbg !15615
  store <8 x float> %983, ptr %694, align 32, !dbg !15616
  store <8 x float> %limit_right.i.sroa.0.0.copyload35379589, ptr %_119.i, align 32, !dbg !15614
  store <8 x float> %_12.i683.sroa.0.0.copyload9606, ptr %695, align 32, !dbg !15618
  store i32 %storemerge.i3249628, ptr %_22.i299.i, align 4, !dbg !15830
  store <8 x float> %minimum.i279.i.sroa.0.0, ptr %uniform_left.i, align 32, !dbg !15834
  store i32 %storemerge.i9658, ptr %_22.i.i, align 4, !dbg !15836
  store <8 x float> %minimum.i.i.sroa.0.09672, ptr %uniform_right.i, align 32, !dbg !15839
  %_5.i = add i64 %base.i343, 8, !dbg !15992
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i343, i64 noundef %_5.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i285.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !15993, !noalias !15994
  unreachable, !dbg !15993

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit: ; preds = %bb19.i330
  %_30.i331 = add nuw i64 %iter.sroa.0.0.i3277454, 1, !dbg !15998
  %_15.i347 = getelementptr inbounds nuw float, ptr %_54.0.i284.i, i64 %base.i343, !dbg !16009
  %lanes.i2357.sroa.0.0.copyload = load <8 x float>, ptr %_15.i347, align 4, !dbg !16011, !alias.scope !16016, !noalias !16020
  %1002 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %suffix.i311.sroa.0.07453, <8 x float> %lanes.i2357.sroa.0.0.copyload), !dbg !16024
  store <8 x float> %1002, ptr %_15.i347, align 4, !dbg !16029, !alias.scope !16035, !noalias !16039
  %1003 = icmp eq i64 %end.sroa.0.0.i3287455, 0, !dbg !16043
  %spec.store.select.i339 = select i1 %1003, i64 %ring.i, i64 %end.sroa.0.0.i3287455, !dbg !16043
  %1004 = add i64 %spec.store.select.i339, -1, !dbg !16044
  %exitcond8408.not = icmp eq i64 %_30.i331, %_18.i296.i, !dbg !16045
  br i1 %exitcond8408.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit342, label %bb19.i330, !dbg !16048

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit342: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit366
  %minimum.i279.i.sroa.0.1 = phi <8 x float> [ %999, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit366 ], [ %minimum.i279.i.sroa.0.0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit ], !dbg !15834
  %storemerge.i324 = phi i32 [ %1000, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit366 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit ], !dbg !16049
  %1005 = fmul <8 x float> %minimum.i279.i.sroa.0.1, splat (float 1.638400e+04), !dbg !16050
  %1006 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %1005), !dbg !16056
  %1007 = fmul <8 x float> %1006, splat (float 0x3F10000000000000), !dbg !16061
  %base.i439 = shl i64 %_219.i, 3, !dbg !16066
  %1008 = or disjoint i64 %base.i439, 7, !dbg !16069
  %or.cond.i443.not = icmp ult i64 %1008, %_56.1.i301.i, !dbg !16069
  br i1 %or.cond.i443.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit447, label %bb4.i446, !dbg !16069, !prof !9914

bb4.i446:                                         ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit342
  store i32 %storemerge.i324.lcssa96419690, ptr %_22.i299.i, align 4
  store <8 x float> %minimum.i279.i.sroa.0.0.lcssa96559709, ptr %uniform_left.i, align 32
  store i32 %storemerge.i.lcssa96719728, ptr %_22.i.i, align 4
  store <8 x float> %minimum.i.i.sroa.0.0.lcssa96859747, ptr %uniform_right.i, align 32
  store <8 x float> %985, ptr %688, align 32, !dbg !15612
  store <8 x float> %limit_left.i.sroa.0.0.copyload35349534, ptr %_115.i, align 32, !dbg !15614
  store <8 x float> %_12.i711.sroa.0.0.copyload9551, ptr %689, align 32, !dbg !15615
  store <8 x float> %983, ptr %694, align 32, !dbg !15616
  store <8 x float> %limit_right.i.sroa.0.0.copyload35379589, ptr %_119.i, align 32, !dbg !15614
  store <8 x float> %_12.i683.sroa.0.0.copyload9606, ptr %695, align 32, !dbg !15618
  store i32 %storemerge.i324, ptr %_22.i299.i, align 4, !dbg !15830
  store <8 x float> %minimum.i279.i.sroa.0.0, ptr %uniform_left.i, align 32, !dbg !15834
  store i32 %storemerge.i9658, ptr %_22.i.i, align 4, !dbg !15836
  store <8 x float> %minimum.i.i.sroa.0.09672, ptr %uniform_right.i, align 32, !dbg !15839
  %_5.i440 = add i64 %base.i439, 8, !dbg !16073
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i439, i64 noundef %_5.i440, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i301.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !16074, !noalias !16075
  unreachable, !dbg !16074

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit447: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit342
  %_15.i445 = getelementptr inbounds nuw float, ptr %_56.0.i300.i, i64 %base.i439, !dbg !16079
  %lanes.i.sroa.0.0.copyload = load <8 x float>, ptr %_15.i445, align 4, !dbg !16081, !alias.scope !16086, !noalias !16090
  %1009 = fadd <8 x float> %1007, %_33.i270.i.sroa.0.0.copyload9657, !dbg !16094
  %1010 = fsub <8 x float> %1009, %lanes.i.sroa.0.0.copyload, !dbg !16100
  store <8 x float> %1010, ptr %706, align 32, !dbg !16105
  %_8.not.i4.i308.i = icmp ugt i64 %_7.i10.i288.i, %_56.1.i301.i
  br i1 %_8.not.i4.i308.i, label %bb4.i7.i324.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i310.i, !dbg !16106, !prof !5262

bb4.i7.i324.i:                                    ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit447
  store i32 %storemerge.i324.lcssa96419690, ptr %_22.i299.i, align 4
  store <8 x float> %minimum.i279.i.sroa.0.0.lcssa96559709, ptr %uniform_left.i, align 32
  store i32 %storemerge.i.lcssa96719728, ptr %_22.i.i, align 4
  store <8 x float> %minimum.i.i.sroa.0.0.lcssa96859747, ptr %uniform_right.i, align 32
  store <8 x float> %985, ptr %688, align 32, !dbg !15612
  store <8 x float> %limit_left.i.sroa.0.0.copyload35349534, ptr %_115.i, align 32, !dbg !15614
  store <8 x float> %_12.i711.sroa.0.0.copyload9551, ptr %689, align 32, !dbg !15615
  store <8 x float> %983, ptr %694, align 32, !dbg !15616
  store <8 x float> %limit_right.i.sroa.0.0.copyload35379589, ptr %_119.i, align 32, !dbg !15614
  store <8 x float> %_12.i683.sroa.0.0.copyload9606, ptr %695, align 32, !dbg !15618
  store i32 %storemerge.i324, ptr %_22.i299.i, align 4, !dbg !15830
  store <8 x float> %minimum.i279.i.sroa.0.0, ptr %uniform_left.i, align 32, !dbg !15834
  store i32 %storemerge.i9658, ptr %_22.i.i, align 4, !dbg !15836
  store <8 x float> %minimum.i.i.sroa.0.09672, ptr %uniform_right.i, align 32, !dbg !15839
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i287.i, i64 noundef %_7.i10.i288.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i301.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cd0e502fea74c9eb9984d521d7f3533e) #31, !dbg !16111, !noalias !16112
  unreachable, !dbg !16111

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i310.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit447
  %_17.i6.i311.i = getelementptr inbounds nuw float, ptr %_56.0.i300.i, i64 %base.i9.i287.i, !dbg !16116
  store <8 x float> %1007, ptr %_17.i6.i311.i, align 4, !dbg !16118, !alias.scope !16123, !noalias !16127
  %_41.i263.i.sroa.0.0.copyload = load <8 x float>, ptr %708, align 32, !dbg !16131
  %1011 = fdiv <8 x float> %1010, %_37.i267.i.sroa.0.0.copyload, !dbg !16134
  %1012 = fsub <8 x float> splat (float 1.000000e+00), %1011, !dbg !16139
  %1013 = fsub <8 x float> %1012, %_41.i263.i.sroa.0.0.copyload, !dbg !16144
  %1014 = fmul <8 x float> %release_left.i.sroa.0.0.copyload9568, %1013, !dbg !16149
  %1015 = fadd <8 x float> %_41.i263.i.sroa.0.0.copyload, %1014, !dbg !16154
  %1016 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1012, <8 x float> %1015), !dbg !16158
  %1017 = bitcast <8 x float> %1016 to <8 x i32>, !dbg !16164
  %1018 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1016), !dbg !16171
  %1019 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1018, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !16173
  %1020 = bitcast <8 x float> %1019 to <8 x i32>, !dbg !16179
  %1021 = xor <8 x i32> %1020, splat (i32 -1), !dbg !16185
  %1022 = and <8 x i32> %1021, %1017, !dbg !16187
  store <8 x i32> %1022, ptr %708, align 32, !dbg !16191
  %base.i430 = shl i64 %_216.i, 3, !dbg !16192
  %_5.i431 = add i64 %base.i430, 8, !dbg !16195
  %1023 = or disjoint i64 %base.i430, 7, !dbg !16196
  %or.cond.i434.not = icmp ult i64 %1023, %_58.1.i313.i, !dbg !16196
  br i1 %or.cond.i434.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit438, label %bb4.i437, !dbg !16196, !prof !9914

bb4.i437:                                         ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i310.i
  store i32 %storemerge.i324.lcssa96419690, ptr %_22.i299.i, align 4
  store <8 x float> %minimum.i279.i.sroa.0.0.lcssa96559709, ptr %uniform_left.i, align 32
  store i32 %storemerge.i.lcssa96719728, ptr %_22.i.i, align 4
  store <8 x float> %minimum.i.i.sroa.0.0.lcssa96859747, ptr %uniform_right.i, align 32
  store <8 x float> %985, ptr %688, align 32, !dbg !15612
  store <8 x float> %limit_left.i.sroa.0.0.copyload35349534, ptr %_115.i, align 32, !dbg !15614
  store <8 x float> %_12.i711.sroa.0.0.copyload9551, ptr %689, align 32, !dbg !15615
  store <8 x float> %983, ptr %694, align 32, !dbg !15616
  store <8 x float> %limit_right.i.sroa.0.0.copyload35379589, ptr %_119.i, align 32, !dbg !15614
  store <8 x float> %_12.i683.sroa.0.0.copyload9606, ptr %695, align 32, !dbg !15618
  store i32 %storemerge.i324, ptr %_22.i299.i, align 4, !dbg !15830
  store <8 x float> %minimum.i279.i.sroa.0.0, ptr %uniform_left.i, align 32, !dbg !15834
  store i32 %storemerge.i9658, ptr %_22.i.i, align 4, !dbg !15836
  store <8 x float> %minimum.i.i.sroa.0.09672, ptr %uniform_right.i, align 32, !dbg !15839
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i430, i64 noundef %_5.i431, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i313.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !16200, !noalias !16201
  unreachable, !dbg !16200

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit438: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i310.i
  %1024 = bitcast <8 x i32> %1022 to <8 x float>, !dbg !16205
  %1025 = fsub <8 x float> splat (float 1.000000e+00), %1024, !dbg !16206
  %_15.i436 = getelementptr inbounds nuw float, ptr %_58.0.i312.i, i64 %base.i430, !dbg !16211
  %lanes.i2287.sroa.0.0.copyload = load <8 x float>, ptr %_15.i436, align 4, !dbg !16213, !alias.scope !16218, !noalias !16222
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_15.i436, ptr noundef nonnull align 4 dereferenceable(32) %data.i.i.i.i.i.i.i.i, i64 32, i1 false), !dbg !16226
  %1026 = fmul <8 x float> %1025, %lanes.i2287.sroa.0.0.copyload, !dbg !16233
  %1027 = select <8 x i1> %712, <8 x float> %lanes.i2287.sroa.0.0.copyload, <8 x float> %1026, !dbg !16238
  store <8 x float> %1027, ptr %data.i.i.i.i.i.i.i.i, align 4, !dbg !16243, !alias.scope !16248, !noalias !16252
  %_222.i = add i64 %iter.i.sroa.36.07462, %right_end.sroa.0.0.i, !dbg !16256
  %_224.i = add i64 %iter.i.sroa.36.07462, %right_expiring.sroa.0.0.i, !dbg !16258
  %_8.not.i12.i.i = icmp ugt i64 %_7.i10.i288.i, %_54.1.i.i
  br i1 %_8.not.i12.i.i, label %bb4.i15.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i, !dbg !16259, !prof !5262

bb4.i15.i.i:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit438
  store i32 %storemerge.i324.lcssa96419690, ptr %_22.i299.i, align 4
  store <8 x float> %minimum.i279.i.sroa.0.0.lcssa96559709, ptr %uniform_left.i, align 32
  store i32 %storemerge.i.lcssa96719728, ptr %_22.i.i, align 4
  store <8 x float> %minimum.i.i.sroa.0.0.lcssa96859747, ptr %uniform_right.i, align 32
  store <8 x float> %985, ptr %688, align 32, !dbg !15612
  store <8 x float> %limit_left.i.sroa.0.0.copyload35349534, ptr %_115.i, align 32, !dbg !15614
  store <8 x float> %_12.i711.sroa.0.0.copyload9551, ptr %689, align 32, !dbg !15615
  store <8 x float> %983, ptr %694, align 32, !dbg !15616
  store <8 x float> %limit_right.i.sroa.0.0.copyload35379589, ptr %_119.i, align 32, !dbg !15614
  store <8 x float> %_12.i683.sroa.0.0.copyload9606, ptr %695, align 32, !dbg !15618
  store i32 %storemerge.i324, ptr %_22.i299.i, align 4, !dbg !15830
  store <8 x float> %minimum.i279.i.sroa.0.0, ptr %uniform_left.i, align 32, !dbg !15834
  store i32 %storemerge.i9658, ptr %_22.i.i, align 4, !dbg !15836
  store <8 x float> %minimum.i.i.sroa.0.09672, ptr %uniform_right.i, align 32, !dbg !15839
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i287.i, i64 noundef %_7.i10.i288.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cd0e502fea74c9eb9984d521d7f3533e) #31, !dbg !16264, !noalias !16265
  unreachable, !dbg !16264

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit438
  %1028 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %987, <8 x float> %limit_right.i.sroa.0.0.copyload35379589, i8 30), !dbg !16279
  %1029 = bitcast <8 x float> %1028 to <8 x i32>, !dbg !16285
  %1030 = icmp slt <8 x i32> %1029, zeroinitializer, !dbg !16289
  %1031 = fdiv <8 x float> %limit_right.i.sroa.0.0.copyload35379589, %987, !dbg !16291
  %1032 = select <8 x i1> %1030, <8 x float> %1031, <8 x float> splat (float 1.000000e+00), !dbg !16289
  %_17.i14.i.i = getelementptr inbounds nuw float, ptr %_54.0.i.i, i64 %base.i9.i287.i, !dbg !16296
  store <8 x float> %1032, ptr %_17.i14.i.i, align 4, !dbg !16298, !alias.scope !16303, !noalias !16307
  %base.i403 = shl i64 %_222.i, 3, !dbg !16311
  %1033 = or disjoint i64 %base.i403, 7, !dbg !16313
  %or.cond.i407.not = icmp ult i64 %1033, %_54.1.i.i, !dbg !16313
  br i1 %or.cond.i407.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit411, label %bb4.i410, !dbg !16313, !prof !9914

bb4.i410:                                         ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i
  store i32 %storemerge.i324.lcssa96419690, ptr %_22.i299.i, align 4
  store <8 x float> %minimum.i279.i.sroa.0.0.lcssa96559709, ptr %uniform_left.i, align 32
  store i32 %storemerge.i.lcssa96719728, ptr %_22.i.i, align 4
  store <8 x float> %minimum.i.i.sroa.0.0.lcssa96859747, ptr %uniform_right.i, align 32
  store <8 x float> %985, ptr %688, align 32, !dbg !15612
  store <8 x float> %limit_left.i.sroa.0.0.copyload35349534, ptr %_115.i, align 32, !dbg !15614
  store <8 x float> %_12.i711.sroa.0.0.copyload9551, ptr %689, align 32, !dbg !15615
  store <8 x float> %983, ptr %694, align 32, !dbg !15616
  store <8 x float> %limit_right.i.sroa.0.0.copyload35379589, ptr %_119.i, align 32, !dbg !15614
  store <8 x float> %_12.i683.sroa.0.0.copyload9606, ptr %695, align 32, !dbg !15618
  store i32 %storemerge.i324, ptr %_22.i299.i, align 4, !dbg !15830
  store <8 x float> %minimum.i279.i.sroa.0.0, ptr %uniform_left.i, align 32, !dbg !15834
  store i32 %storemerge.i9658, ptr %_22.i.i, align 4, !dbg !15836
  store <8 x float> %minimum.i.i.sroa.0.09672, ptr %uniform_right.i, align 32, !dbg !15839
  %_5.i404 = add i64 %base.i403, 8, !dbg !16317
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i403, i64 noundef %_5.i404, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !16318, !noalias !16319
  unreachable, !dbg !16318

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit411: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i
  %_15.i409 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i64 %base.i403, !dbg !16327
  %lanes.i2308.sroa.0.0.copyload = load <8 x float>, ptr %_15.i409, align 4, !dbg !16329, !alias.scope !16334, !noalias !16338
  %position.i = zext i32 %storemerge.i9658 to i64, !dbg !16342
  %1034 = icmp eq i32 %storemerge.i9658, 0, !dbg !16343
  br i1 %1034, label %bb5.i, label %bb3.i, !dbg !16343

bb3.i:                                            ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit411
  %1035 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %minimum.i.i.sroa.0.09672, <8 x float> %lanes.i2308.sroa.0.0.copyload), !dbg !16344
  br label %bb5.i, !dbg !16349

bb5.i:                                            ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit411, %bb3.i
  %minimum.i.i.sroa.0.0 = phi <8 x float> [ %1035, %bb3.i ], [ %lanes.i2308.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit411 ], !dbg !15839
  %_15.i = add nuw nsw i64 %position.i, 1, !dbg !16350
  %complete.i = icmp eq i64 %_15.i, %_18.i247.i, !dbg !16350
  br i1 %complete.i, label %bb19.i301, label %bb7.i298, !dbg !16351

bb7.i298:                                         ; preds = %bb5.i
  %base.i394 = shl i64 %_218.i, 3, !dbg !16352
  %1036 = or disjoint i64 %base.i394, 7, !dbg !16354
  %or.cond.i398.not = icmp ult i64 %1036, %_54.1.i.i, !dbg !16354
  br i1 %or.cond.i398.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit402, label %bb4.i401, !dbg !16354, !prof !9914

bb4.i401:                                         ; preds = %bb7.i298
  store i32 %storemerge.i324.lcssa96419690, ptr %_22.i299.i, align 4
  store <8 x float> %minimum.i279.i.sroa.0.0.lcssa96559709, ptr %uniform_left.i, align 32
  store i32 %storemerge.i.lcssa96719728, ptr %_22.i.i, align 4
  store <8 x float> %minimum.i.i.sroa.0.0.lcssa96859747, ptr %uniform_right.i, align 32
  store <8 x float> %985, ptr %688, align 32, !dbg !15612
  store <8 x float> %limit_left.i.sroa.0.0.copyload35349534, ptr %_115.i, align 32, !dbg !15614
  store <8 x float> %_12.i711.sroa.0.0.copyload9551, ptr %689, align 32, !dbg !15615
  store <8 x float> %983, ptr %694, align 32, !dbg !15616
  store <8 x float> %limit_right.i.sroa.0.0.copyload35379589, ptr %_119.i, align 32, !dbg !15614
  store <8 x float> %_12.i683.sroa.0.0.copyload9606, ptr %695, align 32, !dbg !15618
  store i32 %storemerge.i324, ptr %_22.i299.i, align 4, !dbg !15830
  store <8 x float> %minimum.i279.i.sroa.0.0, ptr %uniform_left.i, align 32, !dbg !15834
  store i32 %storemerge.i9658, ptr %_22.i.i, align 4, !dbg !15836
  store <8 x float> %minimum.i.i.sroa.0.0, ptr %uniform_right.i, align 32, !dbg !15839
  %_5.i395 = add i64 %base.i394, 8, !dbg !16358
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i394, i64 noundef %_5.i395, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !16359, !noalias !16360
  unreachable, !dbg !16359

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit402: ; preds = %bb7.i298
  %_15.i400 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i64 %base.i394, !dbg !16364
  %lanes.i2315.sroa.0.0.copyload = load <8 x float>, ptr %_15.i400, align 4, !dbg !16366, !alias.scope !16371, !noalias !16375
  %1037 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %lanes.i2315.sroa.0.0.copyload, <8 x float> %minimum.i.i.sroa.0.0), !dbg !16379
  %1038 = trunc i64 %_15.i to i32, !dbg !16384
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, !dbg !16385

bb19.i301:                                        ; preds = %bb5.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit384
  %end.sroa.0.0.i7459 = phi i64 [ %1042, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit384 ], [ %_222.i, %bb5.i ]
  %iter.sroa.0.0.i3007458 = phi i64 [ %_30.i302, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit384 ], [ 0, %bb5.i ]
  %suffix.i.sroa.0.07457 = phi <8 x float> [ %1040, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit384 ], [ %lanes.i2308.sroa.0.0.copyload, %bb5.i ]
  %base.i376 = shl i64 %end.sroa.0.0.i7459, 3, !dbg !16386
  %1039 = or disjoint i64 %base.i376, 7, !dbg !16388
  %or.cond.i380.not = icmp ult i64 %1039, %_54.1.i.i, !dbg !16388
  br i1 %or.cond.i380.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit384, label %bb4.i383, !dbg !16388, !prof !9914

bb4.i383:                                         ; preds = %bb19.i301
  store i32 %storemerge.i324.lcssa96419690, ptr %_22.i299.i, align 4
  store <8 x float> %minimum.i279.i.sroa.0.0.lcssa96559709, ptr %uniform_left.i, align 32
  store i32 %storemerge.i.lcssa96719728, ptr %_22.i.i, align 4
  store <8 x float> %minimum.i.i.sroa.0.0.lcssa96859747, ptr %uniform_right.i, align 32
  store <8 x float> %985, ptr %688, align 32, !dbg !15612
  store <8 x float> %limit_left.i.sroa.0.0.copyload35349534, ptr %_115.i, align 32, !dbg !15614
  store <8 x float> %_12.i711.sroa.0.0.copyload9551, ptr %689, align 32, !dbg !15615
  store <8 x float> %983, ptr %694, align 32, !dbg !15616
  store <8 x float> %limit_right.i.sroa.0.0.copyload35379589, ptr %_119.i, align 32, !dbg !15614
  store <8 x float> %_12.i683.sroa.0.0.copyload9606, ptr %695, align 32, !dbg !15618
  store i32 %storemerge.i324, ptr %_22.i299.i, align 4, !dbg !15830
  store <8 x float> %minimum.i279.i.sroa.0.0, ptr %uniform_left.i, align 32, !dbg !15834
  store i32 %storemerge.i9658, ptr %_22.i.i, align 4, !dbg !15836
  store <8 x float> %minimum.i.i.sroa.0.0, ptr %uniform_right.i, align 32, !dbg !15839
  %_5.i377 = add i64 %base.i376, 8, !dbg !16392
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i376, i64 noundef %_5.i377, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !16393, !noalias !16394
  unreachable, !dbg !16393

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit384: ; preds = %bb19.i301
  %_30.i302 = add nuw i64 %iter.sroa.0.0.i3007458, 1, !dbg !16398
  %_15.i382 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i64 %base.i376, !dbg !16403
  %lanes.i2329.sroa.0.0.copyload = load <8 x float>, ptr %_15.i382, align 4, !dbg !16405, !alias.scope !16410, !noalias !16414
  %1040 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %suffix.i.sroa.0.07457, <8 x float> %lanes.i2329.sroa.0.0.copyload), !dbg !16418
  store <8 x float> %1040, ptr %_15.i382, align 4, !dbg !16423, !alias.scope !16429, !noalias !16433
  %1041 = icmp eq i64 %end.sroa.0.0.i7459, 0, !dbg !16437
  %spec.store.select.i306 = select i1 %1041, i64 %ring.i, i64 %end.sroa.0.0.i7459, !dbg !16437
  %1042 = add i64 %spec.store.select.i306, -1, !dbg !16438
  %exitcond8414.not = icmp eq i64 %_30.i302, %_18.i247.i, !dbg !16439
  br i1 %exitcond8414.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, label %bb19.i301, !dbg !16441

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit384, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit402
  %minimum.i.i.sroa.0.1 = phi <8 x float> [ %1037, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit402 ], [ %minimum.i.i.sroa.0.0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit384 ], !dbg !15839
  %storemerge.i = phi i32 [ %1038, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit402 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit384 ], !dbg !16442
  %1043 = fmul <8 x float> %minimum.i.i.sroa.0.1, splat (float 1.638400e+04), !dbg !16443
  %1044 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %1043), !dbg !16448
  %1045 = fmul <8 x float> %1044, splat (float 0x3F10000000000000), !dbg !16453
  %base.i421 = shl i64 %_224.i, 3, !dbg !16458
  %1046 = or disjoint i64 %base.i421, 7, !dbg !16460
  %or.cond.i425.not = icmp ult i64 %1046, %_56.1.i.i, !dbg !16460
  br i1 %or.cond.i425.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit429, label %bb4.i428, !dbg !16460, !prof !9914

bb4.i428:                                         ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit
  store i32 %storemerge.i324.lcssa96419690, ptr %_22.i299.i, align 4
  store <8 x float> %minimum.i279.i.sroa.0.0.lcssa96559709, ptr %uniform_left.i, align 32
  store i32 %storemerge.i.lcssa96719728, ptr %_22.i.i, align 4
  store <8 x float> %minimum.i.i.sroa.0.0.lcssa96859747, ptr %uniform_right.i, align 32
  store <8 x float> %985, ptr %688, align 32, !dbg !15612
  store <8 x float> %limit_left.i.sroa.0.0.copyload35349534, ptr %_115.i, align 32, !dbg !15614
  store <8 x float> %_12.i711.sroa.0.0.copyload9551, ptr %689, align 32, !dbg !15615
  store <8 x float> %983, ptr %694, align 32, !dbg !15616
  store <8 x float> %limit_right.i.sroa.0.0.copyload35379589, ptr %_119.i, align 32, !dbg !15614
  store <8 x float> %_12.i683.sroa.0.0.copyload9606, ptr %695, align 32, !dbg !15618
  store i32 %storemerge.i324, ptr %_22.i299.i, align 4, !dbg !15830
  store <8 x float> %minimum.i279.i.sroa.0.0, ptr %uniform_left.i, align 32, !dbg !15834
  store i32 %storemerge.i, ptr %_22.i.i, align 4, !dbg !15836
  store <8 x float> %minimum.i.i.sroa.0.0, ptr %uniform_right.i, align 32, !dbg !15839
  %_5.i422 = add i64 %base.i421, 8, !dbg !16464
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i421, i64 noundef %_5.i422, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !16465, !noalias !16466
  unreachable, !dbg !16465

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit429: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit
  %_15.i427 = getelementptr inbounds nuw float, ptr %_56.0.i.i, i64 %base.i421, !dbg !16470
  %lanes.i2294.sroa.0.0.copyload = load <8 x float>, ptr %_15.i427, align 4, !dbg !16472, !alias.scope !16477, !noalias !16481
  %1047 = fadd <8 x float> %1045, %_33.i243.i.sroa.0.0.copyload9687, !dbg !16485
  %1048 = fsub <8 x float> %1047, %lanes.i2294.sroa.0.0.copyload, !dbg !16490
  store <8 x float> %1048, ptr %717, align 32, !dbg !16495
  %_8.not.i4.i.i = icmp ugt i64 %_7.i10.i288.i, %_56.1.i.i
  br i1 %_8.not.i4.i.i, label %bb4.i7.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i, !dbg !16496, !prof !5262

bb4.i7.i.i:                                       ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit429
  store i32 %storemerge.i324.lcssa96419690, ptr %_22.i299.i, align 4
  store <8 x float> %minimum.i279.i.sroa.0.0.lcssa96559709, ptr %uniform_left.i, align 32
  store i32 %storemerge.i.lcssa96719728, ptr %_22.i.i, align 4
  store <8 x float> %minimum.i.i.sroa.0.0.lcssa96859747, ptr %uniform_right.i, align 32
  store <8 x float> %985, ptr %688, align 32, !dbg !15612
  store <8 x float> %limit_left.i.sroa.0.0.copyload35349534, ptr %_115.i, align 32, !dbg !15614
  store <8 x float> %_12.i711.sroa.0.0.copyload9551, ptr %689, align 32, !dbg !15615
  store <8 x float> %983, ptr %694, align 32, !dbg !15616
  store <8 x float> %limit_right.i.sroa.0.0.copyload35379589, ptr %_119.i, align 32, !dbg !15614
  store <8 x float> %_12.i683.sroa.0.0.copyload9606, ptr %695, align 32, !dbg !15618
  store i32 %storemerge.i324, ptr %_22.i299.i, align 4, !dbg !15830
  store <8 x float> %minimum.i279.i.sroa.0.0, ptr %uniform_left.i, align 32, !dbg !15834
  store i32 %storemerge.i, ptr %_22.i.i, align 4, !dbg !15836
  store <8 x float> %minimum.i.i.sroa.0.0, ptr %uniform_right.i, align 32, !dbg !15839
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i287.i, i64 noundef %_7.i10.i288.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cd0e502fea74c9eb9984d521d7f3533e) #31, !dbg !16501, !noalias !16502
  unreachable, !dbg !16501

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit429
  %_17.i6.i.i = getelementptr inbounds nuw float, ptr %_56.0.i.i, i64 %base.i9.i287.i, !dbg !16506
  store <8 x float> %1045, ptr %_17.i6.i.i, align 4, !dbg !16508, !alias.scope !16513, !noalias !16517
  %_41.i.i.sroa.0.0.copyload = load <8 x float>, ptr %719, align 32, !dbg !16521
  %1049 = fdiv <8 x float> %1048, %_37.i.i.sroa.0.0.copyload, !dbg !16522
  %1050 = fsub <8 x float> splat (float 1.000000e+00), %1049, !dbg !16527
  %1051 = fsub <8 x float> %1050, %_41.i.i.sroa.0.0.copyload, !dbg !16532
  %1052 = fmul <8 x float> %release_right.i.sroa.0.0.copyload9623, %1051, !dbg !16537
  %1053 = fadd <8 x float> %_41.i.i.sroa.0.0.copyload, %1052, !dbg !16542
  %1054 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1050, <8 x float> %1053), !dbg !16546
  %1055 = bitcast <8 x float> %1054 to <8 x i32>, !dbg !16551
  %1056 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1054), !dbg !16557
  %1057 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1056, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !16559
  %1058 = bitcast <8 x float> %1057 to <8 x i32>, !dbg !16565
  %1059 = xor <8 x i32> %1058, splat (i32 -1), !dbg !16571
  %1060 = and <8 x i32> %1059, %1055, !dbg !16573
  store <8 x i32> %1060, ptr %719, align 32, !dbg !16577
  %_6.not.i415 = icmp ugt i64 %_5.i431, %_58.1.i.i
  br i1 %_6.not.i415, label %bb4.i419, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit420, !dbg !16578, !prof !5262

bb4.i419:                                         ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i
  store i32 %storemerge.i324.lcssa96419690, ptr %_22.i299.i, align 4
  store <8 x float> %minimum.i279.i.sroa.0.0.lcssa96559709, ptr %uniform_left.i, align 32
  store i32 %storemerge.i.lcssa96719728, ptr %_22.i.i, align 4
  store <8 x float> %minimum.i.i.sroa.0.0.lcssa96859747, ptr %uniform_right.i, align 32
  store <8 x float> %985, ptr %688, align 32, !dbg !15612
  store <8 x float> %limit_left.i.sroa.0.0.copyload35349534, ptr %_115.i, align 32, !dbg !15614
  store <8 x float> %_12.i711.sroa.0.0.copyload9551, ptr %689, align 32, !dbg !15615
  store <8 x float> %983, ptr %694, align 32, !dbg !15616
  store <8 x float> %limit_right.i.sroa.0.0.copyload35379589, ptr %_119.i, align 32, !dbg !15614
  store <8 x float> %_12.i683.sroa.0.0.copyload9606, ptr %695, align 32, !dbg !15618
  store i32 %storemerge.i324, ptr %_22.i299.i, align 4, !dbg !15830
  store <8 x float> %minimum.i279.i.sroa.0.0, ptr %uniform_left.i, align 32, !dbg !15834
  store i32 %storemerge.i, ptr %_22.i.i, align 4, !dbg !15836
  store <8 x float> %minimum.i.i.sroa.0.0, ptr %uniform_right.i, align 32, !dbg !15839
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i430, i64 noundef %_5.i431, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_296a0a227063f9d4f193ea1ae436593b) #31, !dbg !16583, !noalias !16584
  unreachable, !dbg !16583

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit420: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i
  %1061 = bitcast <8 x i32> %1060 to <8 x float>, !dbg !16588
  %1062 = fsub <8 x float> splat (float 1.000000e+00), %1061, !dbg !16589
  %_15.i418 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %base.i430, !dbg !16594
  %lanes.i2301.sroa.0.0.copyload = load <8 x float>, ptr %_15.i418, align 4, !dbg !16596, !alias.scope !16601, !noalias !16605
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_15.i418, ptr noundef nonnull align 4 dereferenceable(32) %data.i5.i.i.i.i.i.i.i, i64 32, i1 false), !dbg !16609
  %1063 = fmul <8 x float> %1062, %lanes.i2301.sroa.0.0.copyload, !dbg !16615
  %1064 = select <8 x i1> %712, <8 x float> %lanes.i2301.sroa.0.0.copyload, <8 x float> %1063, !dbg !16620
  store <8 x float> %1064, ptr %data.i5.i.i.i.i.i.i.i, align 4, !dbg !16625, !alias.scope !16630, !noalias !16634
  %exitcond8427.not = icmp eq i64 %947, %942, !dbg !15586
  br i1 %exitcond8427.not, label %bb44.i.loopexit, label %bb43.i, !dbg !15586

_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit: ; preds = %bb27.i.loopexit
  %1065 = trunc i64 %main_cursor.sroa.0.1.i.lcssa to i32, !dbg !16638
  %1066 = trunc i64 %ring_cursor.sroa.0.1.i.lcssa to i32, !dbg !16640
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, !dbg !16641

_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit, %bb14.i
  %ring_cursor.sroa.0.0.i.lcssa = phi i32 [ %_43.i, %bb14.i ], [ %1066, %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit ], !dbg !14302
  %main_cursor.sroa.0.0.i.lcssa = phi i32 [ %_42.i, %bb14.i ], [ %1065, %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit ], !dbg !14299
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %left_prefix.i, ptr noundef nonnull align 32 dereferenceable(32) %uniform_left.i, i64 32, i1 false), !dbg !16641
  %1067 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 104, !dbg !16642
  %left_phase.i = load i32, ptr %1067, align 8, !dbg !16642, !noalias !14306, !noundef !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %right_prefix.i, ptr noundef nonnull align 32 dereferenceable(32) %uniform_right.i, i64 32, i1 false), !dbg !16643
  %1068 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 104, !dbg !16644
  %right_phase.i = load i32, ptr %1068, align 8, !dbg !16644, !noalias !14306, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_right.i), !dbg !16645, !noalias !14306
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i), !dbg !16646, !noalias !14306
  %1069 = getelementptr inbounds nuw i8, ptr %self, i64 1736, !dbg !16647
  %_244.1.i = load i64, ptr %1069, align 8, !dbg !16647, !alias.scope !14169, !noalias !14183, !noundef !12
  %_8.i2840 = icmp samesign ugt i64 %_244.1.i, 7, !dbg !16648
  br i1 %_8.i2840, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2843, label %bb2.i2841, !dbg !16648, !prof !1076

bb2.i2841:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_244.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #31, !dbg !16653, !noalias !16654
  unreachable, !dbg !16653

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2843: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit
  %1070 = getelementptr inbounds nuw i8, ptr %self, i64 1728, !dbg !16647
  %_244.0.i = load ptr, ptr %1070, align 8, !dbg !16647, !alias.scope !14169, !noalias !14183, !nonnull !12, !noundef !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_244.0.i, ptr noundef nonnull align 32 dereferenceable(32) %left_prefix.i, i64 32, i1 false), !dbg !16658
  %_245.0.i = load ptr, ptr %46, align 8, !dbg !16662, !alias.scope !14169, !noalias !14183, !nonnull !12, !noundef !12
  %_245.1.i = load i64, ptr %47, align 8, !dbg !16662, !alias.scope !14169, !noalias !14183, !noundef !12
  %1071 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i), !dbg !16663
  br i1 %1071, label %bb2.i3416, label %bb6.i3411, !dbg !16663

bb6.i3411:                                        ; preds = %bb2.i3416, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2843
  %end_or_len.idx.i = shl nuw nsw i64 %_245.1.i, 2, !dbg !16667
  %end_or_len.i = getelementptr inbounds nuw i8, ptr %_245.0.i, i64 %end_or_len.idx.i, !dbg !16667
  %_293.i = icmp eq i64 %_245.1.i, 0, !dbg !16671
  br i1 %_293.i, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i3412, !dbg !16674

bb2.i3416:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2843
  %bytes1.sroa.0.0.zext.i = and i32 %left_phase.i, 255, !dbg !16675
  %bytes1.sroa.0.0.isplat.i = mul nuw i32 %bytes1.sroa.0.0.zext.i, 16843009, !dbg !16675
  %_5.i3417 = icmp eq i32 %left_phase.i, %bytes1.sroa.0.0.isplat.i, !dbg !16676
  br i1 %_5.i3417, label %bb3.i3418, label %bb6.i3411, !dbg !16676

bb3.i3418:                                        ; preds = %bb2.i3416
  %bytes.sroa.0.0.extract.trunc.i = trunc i32 %left_phase.i to i8, !dbg !16677
  %1072 = shl nuw nsw i64 %_245.1.i, 2, !dbg !16679
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_245.0.i, i8 %bytes.sroa.0.0.extract.trunc.i, i64 %1072, i1 false), !dbg !16679, !alias.scope !16680, !noalias !14173
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, !dbg !16683

bb10.i3412:                                       ; preds = %bb6.i3411, %bb10.i3412
  %iter.sroa.0.04.i = phi ptr [ %_38.i3413, %bb10.i3412 ], [ %_245.0.i, %bb6.i3411 ]
  %_38.i3413 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i, i64 4, !dbg !16684
  store i32 %left_phase.i, ptr %iter.sroa.0.04.i, align 4, !dbg !16686, !alias.scope !16680, !noalias !14173
  %_29.i3414 = icmp eq ptr %_38.i3413, %end_or_len.i, !dbg !16671
  br i1 %_29.i3414, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i3412, !dbg !16674

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit: ; preds = %bb10.i3412, %bb6.i3411, %bb3.i3418
  %1073 = getelementptr inbounds nuw i8, ptr %self, i64 1936, !dbg !16687
  %_246.1.i = load i64, ptr %1073, align 8, !dbg !16687, !alias.scope !14171, !noalias !14236, !noundef !12
  %_8.i2835 = icmp samesign ugt i64 %_246.1.i, 7, !dbg !16688
  br i1 %_8.i2835, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2838, label %bb2.i2836, !dbg !16688, !prof !1076

bb2.i2836:                                        ; preds = %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_246.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #31, !dbg !16693, !noalias !16694
  unreachable, !dbg !16693

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2838: ; preds = %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit
  %1074 = getelementptr inbounds nuw i8, ptr %self, i64 1928, !dbg !16687
  %_246.0.i = load ptr, ptr %1074, align 8, !dbg !16687, !alias.scope !14171, !noalias !14236, !nonnull !12, !noundef !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_246.0.i, ptr noundef nonnull align 32 dereferenceable(32) %right_prefix.i, i64 32, i1 false), !dbg !16698
  %_247.0.i = load ptr, ptr %55, align 8, !dbg !16702, !alias.scope !14171, !noalias !14236, !nonnull !12, !noundef !12
  %_247.1.i = load i64, ptr %56, align 8, !dbg !16702, !alias.scope !14171, !noalias !14236, !noundef !12
  %1075 = tail call i1 @llvm.is.constant.i32(i32 %right_phase.i), !dbg !16703
  br i1 %1075, label %bb2.i3429, label %bb6.i3420, !dbg !16703

bb6.i3420:                                        ; preds = %bb2.i3429, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2838
  %end_or_len.idx.i3421 = shl nuw nsw i64 %_247.1.i, 2, !dbg !16706
  %end_or_len.i3422 = getelementptr inbounds nuw i8, ptr %_247.0.i, i64 %end_or_len.idx.i3421, !dbg !16706
  %_293.i3423 = icmp eq i64 %_247.1.i, 0, !dbg !16710
  br i1 %_293.i3423, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit3435, label %bb10.i3424, !dbg !16713

bb2.i3429:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit2838
  %bytes1.sroa.0.0.zext.i3430 = and i32 %right_phase.i, 255, !dbg !16714
  %bytes1.sroa.0.0.isplat.i3431 = mul nuw i32 %bytes1.sroa.0.0.zext.i3430, 16843009, !dbg !16714
  %_5.i3432 = icmp eq i32 %right_phase.i, %bytes1.sroa.0.0.isplat.i3431, !dbg !16715
  br i1 %_5.i3432, label %bb3.i3433, label %bb6.i3420, !dbg !16715

bb3.i3433:                                        ; preds = %bb2.i3429
  %bytes.sroa.0.0.extract.trunc.i3434 = trunc i32 %right_phase.i to i8, !dbg !16716
  %1076 = shl nuw nsw i64 %_247.1.i, 2, !dbg !16718
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_247.0.i, i8 %bytes.sroa.0.0.extract.trunc.i3434, i64 %1076, i1 false), !dbg !16718, !alias.scope !16719, !noalias !14173
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit3435, !dbg !16722

bb10.i3424:                                       ; preds = %bb6.i3420, %bb10.i3424
  %iter.sroa.0.04.i3425 = phi ptr [ %_38.i3426, %bb10.i3424 ], [ %_247.0.i, %bb6.i3420 ]
  %_38.i3426 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i3425, i64 4, !dbg !16723
  store i32 %right_phase.i, ptr %iter.sroa.0.04.i3425, align 4, !dbg !16725, !alias.scope !16719, !noalias !14173
  %_29.i3427 = icmp eq ptr %_38.i3426, %end_or_len.i3422, !dbg !16710
  br i1 %_29.i3427, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit3435, label %bb10.i3424, !dbg !16713

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit3435: ; preds = %bb10.i3424, %bb6.i3420, %bb3.i3433
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_left.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33) #30, !dbg !16726
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_right.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34) #30, !dbg !16727
  store i32 %main_cursor.sroa.0.0.i.lcssa, ptr %_35, align 4, !dbg !16638, !alias.scope !14173, !noalias !14301
  store i32 %ring_cursor.sroa.0.0.i.lcssa, ptr %643, align 4, !dbg !16640, !alias.scope !14173, !noalias !14301
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i), !dbg !16728, !noalias !14306
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i), !dbg !16729, !noalias !14306
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, !dbg !14166

_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit3435
  br i1 %quiet.sroa.0.06181, label %bb28, label %bb40, !dbg !16730

bb22:                                             ; preds = %bb20
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16731), !dbg !16734
  %1077 = getelementptr inbounds nuw i8, ptr %self, i64 1760, !dbg !16735
  %_40.0.i = load ptr, ptr %1077, align 8, !dbg !16735, !alias.scope !16731, !nonnull !12, !noundef !12
  %1078 = getelementptr inbounds nuw i8, ptr %self, i64 1768, !dbg !16735
  %_40.1.i = load i64, ptr %1078, align 8, !dbg !16735, !alias.scope !16731, !noundef !12
  %1079 = getelementptr inbounds nuw i8, ptr %self, i64 1824, !dbg !16737
  %_41.0.i = load ptr, ptr %1079, align 8, !dbg !16737, !alias.scope !16731, !nonnull !12, !noundef !12
  %1080 = getelementptr inbounds nuw i8, ptr %self, i64 1832, !dbg !16737
  %_41.1.i = load i64, ptr %1080, align 8, !dbg !16737, !alias.scope !16731, !noundef !12
  %..i.i.i.i = tail call noundef i64 @llvm.umin.i64(i64 %_41.1.i, i64 %_40.1.i), !dbg !16738
  %_2.i6.not.i = icmp eq i64 %..i.i.i.i, 0, !dbg !16744
  br i1 %_2.i6.not.i, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb4.i3436, !dbg !16744

bb4.i3436:                                        ; preds = %bb22, %bb6.i3438
  %iter.sroa.8.07.i = phi i64 [ %1081, %bb6.i3438 ], [ 0, %bb22 ]
  %_3.i1.i.i = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i, i64 %iter.sroa.8.07.i, !dbg !16747
  %_14.i3437 = load i32, ptr %_3.i1.i.i, align 4, !dbg !16750, !noalias !16731, !noundef !12
  %_20.i = icmp eq i32 %_14.i3437, 0, !dbg !16751
  br i1 %_20.i, label %panic.i3447, label %bb6.i3438, !dbg !16751

bb6.i3438:                                        ; preds = %bb4.i3436
  %_3.i.i.i3439 = getelementptr inbounds nuw i32, ptr %_40.0.i, i64 %iter.sroa.8.07.i, !dbg !16752
  %1081 = add nuw i64 %iter.sroa.8.07.i, 1, !dbg !16755
  %window.i3440 = zext i32 %_14.i3437 to i64, !dbg !16750
  %_18.i3441 = load i32, ptr %_3.i.i.i3439, align 4, !dbg !16756, !noalias !16731, !noundef !12
  %_17.i3442 = zext i32 %_18.i3441 to i64, !dbg !16756
  %_19.i3443 = urem i64 %frames, %window.i3440, !dbg !16751
  %_16.i3444 = add nuw nsw i64 %_19.i3443, %_17.i3442, !dbg !16757
  %_15.i3445 = urem i64 %_16.i3444, %window.i3440, !dbg !16758
  %1082 = trunc nuw i64 %_15.i3445 to i32, !dbg !16759
  store i32 %1082, ptr %_3.i.i.i3439, align 4, !dbg !16759, !noalias !16731
  %exitcond.not.i = icmp eq i64 %1081, %..i.i.i.i, !dbg !16744
  br i1 %exitcond.not.i, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb4.i3436, !dbg !16744

panic.i3447:                                      ; preds = %bb4.i3436
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1781ea1b97b96e9885c590e9b4440a45) #31, !dbg !16751, !noalias !16731
  unreachable, !dbg !16751

_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit: ; preds = %bb6.i3438, %bb22
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16760), !dbg !16763
  %1083 = getelementptr inbounds nuw i8, ptr %self, i64 1960, !dbg !16764
  %_40.0.i3448 = load ptr, ptr %1083, align 8, !dbg !16764, !alias.scope !16760, !nonnull !12, !noundef !12
  %1084 = getelementptr inbounds nuw i8, ptr %self, i64 1968, !dbg !16764
  %_40.1.i3449 = load i64, ptr %1084, align 8, !dbg !16764, !alias.scope !16760, !noundef !12
  %1085 = getelementptr inbounds nuw i8, ptr %self, i64 2024, !dbg !16766
  %_41.0.i3450 = load ptr, ptr %1085, align 8, !dbg !16766, !alias.scope !16760, !nonnull !12, !noundef !12
  %1086 = getelementptr inbounds nuw i8, ptr %self, i64 2032, !dbg !16766
  %_41.1.i3451 = load i64, ptr %1086, align 8, !dbg !16766, !alias.scope !16760, !noundef !12
  %..i.i.i.i3452 = tail call noundef i64 @llvm.umin.i64(i64 %_41.1.i3451, i64 %_40.1.i3449), !dbg !16767
  %_2.i6.not.i3453 = icmp eq i64 %..i.i.i.i3452, 0, !dbg !16773
  br i1 %_2.i6.not.i3453, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit3470, label %bb4.i3454, !dbg !16773

bb4.i3454:                                        ; preds = %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, %bb6.i3459
  %iter.sroa.8.07.i3455 = phi i64 [ %1087, %bb6.i3459 ], [ 0, %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit ]
  %_3.i1.i.i3456 = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i3450, i64 %iter.sroa.8.07.i3455, !dbg !16776
  %_14.i3457 = load i32, ptr %_3.i1.i.i3456, align 4, !dbg !16779, !noalias !16760, !noundef !12
  %_20.i3458 = icmp eq i32 %_14.i3457, 0, !dbg !16780
  br i1 %_20.i3458, label %panic.i3469, label %bb6.i3459, !dbg !16780

bb6.i3459:                                        ; preds = %bb4.i3454
  %_3.i.i.i3460 = getelementptr inbounds nuw i32, ptr %_40.0.i3448, i64 %iter.sroa.8.07.i3455, !dbg !16781
  %1087 = add nuw i64 %iter.sroa.8.07.i3455, 1, !dbg !16784
  %window.i3461 = zext i32 %_14.i3457 to i64, !dbg !16779
  %_18.i3462 = load i32, ptr %_3.i.i.i3460, align 4, !dbg !16785, !noalias !16760, !noundef !12
  %_17.i3463 = zext i32 %_18.i3462 to i64, !dbg !16785
  %_19.i3464 = urem i64 %frames, %window.i3461, !dbg !16780
  %_16.i3465 = add nuw nsw i64 %_19.i3464, %_17.i3463, !dbg !16786
  %_15.i3466 = urem i64 %_16.i3465, %window.i3461, !dbg !16787
  %1088 = trunc nuw i64 %_15.i3466 to i32, !dbg !16788
  store i32 %1088, ptr %_3.i.i.i3460, align 4, !dbg !16788, !noalias !16760
  %exitcond.not.i3467 = icmp eq i64 %1087, %..i.i.i.i3452, !dbg !16773
  br i1 %exitcond.not.i3467, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit3470, label %bb4.i3454, !dbg !16773

panic.i3469:                                      ; preds = %bb4.i3454
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1781ea1b97b96e9885c590e9b4440a45) #31, !dbg !16780, !noalias !16760
  unreachable, !dbg !16780

_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit3470: ; preds = %bb6.i3459, %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit
  %1089 = getelementptr inbounds nuw i8, ptr %self, i64 1624, !dbg !16789
  %_29.val = load i64, ptr %1089, align 8, !dbg !16789
  %1090 = getelementptr inbounds nuw i8, ptr %self, i64 1632, !dbg !16789
  %_29.val3017 = load i64, ptr %1090, align 8, !dbg !16789, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16790), !dbg !16789
  %_10.i3471 = icmp eq i64 %_29.val3017, 0, !dbg !16793
  br i1 %_10.i3471, label %panic.i3485, label %bb1.i3472, !dbg !16793

bb1.i3472:                                        ; preds = %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit3470
  %_28 = getelementptr inbounds nuw i8, ptr %self, i64 1640, !dbg !16795
  %_7.i3473 = load i32, ptr %_28, align 4, !dbg !16796, !alias.scope !16790, !noundef !12
  %_6.i3474 = zext i32 %_7.i3473 to i64, !dbg !16796
  %_8.i3475 = urem i64 %frames, %_29.val3017, !dbg !16793
  %_5.i3476 = add nuw nsw i64 %_8.i3475, %_6.i3474, !dbg !16797
  %_4.i3477 = urem i64 %_5.i3476, %_29.val3017, !dbg !16798
  %1091 = trunc i64 %_4.i3477 to i32, !dbg !16799
  store i32 %1091, ptr %_28, align 4, !dbg !16799, !alias.scope !16790
  %_17.i3478 = icmp eq i64 %_29.val, 0, !dbg !16800
  br i1 %_17.i3478, label %panic2.i, label %_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit, !dbg !16800

panic.i3485:                                      ; preds = %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit3470
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6b076e9a9e313bc2481504f4da644a5c) #31, !dbg !16793, !noalias !16790
  unreachable, !dbg !16793

panic2.i:                                         ; preds = %bb1.i3472
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f370b9a38141751c9788be81259bacff) #31, !dbg !16800, !noalias !16790
  unreachable, !dbg !16800

_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit: ; preds = %bb1.i3472
  %1092 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !16801
  %_14.i3480 = load i32, ptr %1092, align 4, !dbg !16801, !alias.scope !16790, !noundef !12
  %_13.i3481 = zext i32 %_14.i3480 to i64, !dbg !16801
  %_15.i3482 = urem i64 %frames, %_29.val, !dbg !16800
  %_12.i3483 = add nuw nsw i64 %_15.i3482, %_13.i3481, !dbg !16802
  %_11.i3484 = urem i64 %_12.i3483, %_29.val, !dbg !16803
  %1093 = trunc i64 %_11.i3484 to i32, !dbg !16804
  store i32 %1093, ptr %1092, align 4, !dbg !16804, !alias.scope !16790
  br label %bb42, !dbg !16805

bb28:                                             ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_37 = tail call noundef zeroext i1 @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #30, !dbg !16806
  br i1 %_37, label %bb30, label %bb40, !dbg !16807

bb30:                                             ; preds = %bb28
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_39 = tail call noundef zeroext i1 @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #30, !dbg !16808
  br i1 %_39, label %bb32, label %bb40, !dbg !16809

bb32:                                             ; preds = %bb30
  %_80.not = icmp samesign ugt i64 %words, %left_io.1
  br i1 %_80.not, label %bb51, label %bb1.i3486, !dbg !16810, !prof !5262

bb51:                                             ; preds = %bb32
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %words, i64 noundef %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_df34220a19b474a957e74caf6cc3e38d) #31, !dbg !16818
  unreachable, !dbg !16818

bb1.i3486:                                        ; preds = %bb32, %bb10.i3500
  %iter.sroa.6.0.i3487 = phi i64 [ %len.i.i.i.i3492, %bb10.i3500 ], [ %words, %bb32 ], !dbg !16819
  %iter.sroa.0.0.i3488 = phi ptr [ %data.i.i.i.i3491, %bb10.i3500 ], [ %left_io.0, %bb32 ], !dbg !16819
  %1094 = icmp eq i64 %iter.sroa.6.0.i3487, 0, !dbg !16821
  br i1 %1094, label %bb34, label %bb11.preheader.i3489, !dbg !16821

bb11.preheader.i3489:                             ; preds = %bb1.i3486
  %..i.i.i3490 = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i3487, i64 32), !dbg !16823
  %_18.idx.i3493 = shl nuw nsw i64 %..i.i.i3490, 2, !dbg !16826
  %_18.i3494 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i3488, i64 %_18.idx.i3493, !dbg !16826
  br label %bb11.i3495, !dbg !16831

bb11.i3495:                                       ; preds = %bb11.i3495, %bb11.preheader.i3489
  %iter1.sroa.0.014.i3496 = phi ptr [ %_31.i3498, %bb11.i3495 ], [ %iter.sroa.0.0.i3488, %bb11.preheader.i3489 ]
  %bits.sroa.0.013.i3497 = phi i32 [ %1095, %bb11.i3495 ], [ 0, %bb11.preheader.i3489 ]
  %_31.i3498 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i3496, i64 4, !dbg !16833
  %_134.i3499 = load i32, ptr %iter1.sroa.0.014.i3496, align 4, !dbg !16835, !alias.scope !16836, !noundef !12
  %1095 = or i32 %_134.i3499, %bits.sroa.0.013.i3497, !dbg !16839
  %_25.i = icmp eq ptr %_31.i3498, %_18.i3494, !dbg !16840
  br i1 %_25.i, label %bb10.i3500, label %bb11.i3495, !dbg !16831

bb10.i3500:                                       ; preds = %bb11.i3495
  %data.i.i.i.i3491 = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i3488, i64 %..i.i.i3490, !dbg !16842
  %len.i.i.i.i3492 = sub nuw nsw i64 %iter.sroa.6.0.i3487, %..i.i.i3490, !dbg !16847
  %1096 = icmp eq i32 %1095, 0, !dbg !16848
  br i1 %1096, label %bb1.i3486, label %bb40, !dbg !16848

bb34:                                             ; preds = %bb1.i3486
  %_88.not = icmp samesign ugt i64 %words, %right_io.1, !dbg !16849
  br i1 %_88.not, label %bb54, label %bb1.i3512, !dbg !16849, !prof !905

bb40:                                             ; preds = %bb10.i3500, %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, %bb28, %bb30, %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit3528
  %_36.sroa.0.0 = phi i8 [ %1187, %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit3528 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit ], [ 0, %bb30 ], [ 0, %bb28 ], [ 0, %bb10.i3500 ], !dbg !16855
  store i8 %_36.sroa.0.0, ptr %38, align 8, !dbg !16856
  %1097 = load i8, ptr %2, align 32, !dbg !16857, !range !5399, !noundef !12
  store i8 %1097, ptr %0, align 1, !dbg !16858
  call void @llvm.lifetime.start.p0(ptr nonnull %shape), !dbg !16859
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %shape, ptr noundef nonnull align 16 dereferenceable(24) %_32, i64 24, i1 false), !dbg !16860
  %1098 = getelementptr inbounds nuw i8, ptr %self, i64 2120, !dbg !16861
  %1099 = load i32, ptr %1098, align 8, !dbg !16861, !noundef !12
  %_53 = getelementptr inbounds nuw i8, ptr %self, i64 1568, !dbg !16863
  %1100 = getelementptr inbounds nuw i8, ptr %self, i64 1584, !dbg !16870
  %_99.0 = load ptr, ptr %1100, align 16, !dbg !16870, !nonnull !12, !noundef !12
  %1101 = getelementptr inbounds nuw i8, ptr %self, i64 1592, !dbg !16870
  %_99.1 = load i64, ptr %1101, align 8, !dbg !16870, !noundef !12
  %1102 = getelementptr inbounds nuw i8, ptr %self, i64 1600, !dbg !16870
  %_100.0 = load ptr, ptr %1102, align 32, !dbg !16870, !nonnull !12, !noundef !12
  %1103 = getelementptr inbounds nuw i8, ptr %self, i64 1608, !dbg !16870
  %_100.1 = load i64, ptr %1103, align 8, !dbg !16870, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16871), !dbg !16874
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16875), !dbg !16874
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16877), !dbg !16874
  %1104 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !16879
  %fst_len.i.i = and i64 %left_io.1, 2305843009213693944, !dbg !16890
  %1105 = bitcast <8 x float> %1104 to <8 x i32>, !dbg !16894
  %_22.not.i19207.i = icmp eq i64 %fst_len.i.i, 0, !dbg !16904
  br i1 %_22.not.i19207.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit25.i, label %bb13.i20.i, !dbg !16904

bb13.i20.i:                                       ; preds = %bb40, %bb13.i20.i
  %iter.sroa.0.0.i18210.i = phi ptr [ %_27.i21.i, %bb13.i20.i ], [ %left_io.0, %bb40 ]
  %iter.sroa.5.0.i17209.i = phi i64 [ %_28.i22.i, %bb13.i20.i ], [ %fst_len.i.i, %bb40 ]
  %ok.i9.sroa.0.0208.i = phi <8 x i32> [ %1110, %bb13.i20.i ], [ %1105, %bb40 ]
  %lanes.i.sroa.0.0.copyload.i = load <8 x i32>, ptr %iter.sroa.0.0.i18210.i, align 4, !dbg !16910, !alias.scope !16915, !noalias !16919
  %_27.i21.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i18210.i, i64 32, !dbg !16924
  %_28.i22.i = add i64 %iter.sroa.5.0.i17209.i, -8, !dbg !16931
  %1106 = and <8 x i32> %lanes.i.sroa.0.0.copyload.i, splat (i32 2147483647), !dbg !16932
  %1107 = bitcast <8 x i32> %1106 to <8 x float>, !dbg !16938
  %1108 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1107, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !16939
  %1109 = bitcast <8 x float> %1108 to <8 x i32>, !dbg !16894
  %1110 = and <8 x i32> %ok.i9.sroa.0.0208.i, %1109, !dbg !16945
  %_22.not.i19.i = icmp eq i64 %_28.i22.i, 0, !dbg !16904
  br i1 %_22.not.i19.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit25.i, label %bb13.i20.i, !dbg !16904

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit25.i: ; preds = %bb13.i20.i, %bb40
  %ok.i9.sroa.0.0.lcssa.i = phi <8 x i32> [ %1105, %bb40 ], [ %1110, %bb13.i20.i ], !dbg !16947
  %1111 = icmp sgt <8 x i32> %ok.i9.sroa.0.0.lcssa.i, splat (i32 -1), !dbg !16948
  %1112 = bitcast <8 x i1> %1111 to i8, !dbg !16948
  %_0.i90.not.i = icmp eq i8 %1112, 0, !dbg !16958
  br i1 %_0.i90.not.i, label %bb2.i3508, label %bb7.i3502, !dbg !16959

bb2.i3508:                                        ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit25.i
  %fst_len.i105.i = and i64 %right_io.1, 2305843009213693944, !dbg !16960
  %_22.not.i211.i = icmp eq i64 %fst_len.i105.i, 0, !dbg !16964
  br i1 %_22.not.i211.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %bb13.i.i3509, !dbg !16964

bb13.i.i3509:                                     ; preds = %bb2.i3508, %bb13.i.i3509
  %iter.sroa.0.0.i214.i = phi ptr [ %_27.i.i3510, %bb13.i.i3509 ], [ %right_io.0, %bb2.i3508 ]
  %iter.sroa.5.0.i213.i = phi i64 [ %_28.i.i3511, %bb13.i.i3509 ], [ %fst_len.i105.i, %bb2.i3508 ]
  %ok.i.sroa.0.0212.i = phi <8 x i32> [ %1117, %bb13.i.i3509 ], [ %1105, %bb2.i3508 ]
  %lanes.i49.sroa.0.0.copyload.i = load <8 x i32>, ptr %iter.sroa.0.0.i214.i, align 4, !dbg !16967, !alias.scope !16972, !noalias !16976
  %_27.i.i3510 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i214.i, i64 32, !dbg !16980
  %_28.i.i3511 = add i64 %iter.sroa.5.0.i213.i, -8, !dbg !16983
  %1113 = and <8 x i32> %lanes.i49.sroa.0.0.copyload.i, splat (i32 2147483647), !dbg !16984
  %1114 = bitcast <8 x i32> %1113 to <8 x float>, !dbg !16990
  %1115 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1114, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !16991
  %1116 = bitcast <8 x float> %1115 to <8 x i32>, !dbg !16997
  %1117 = and <8 x i32> %ok.i.sroa.0.0212.i, %1116, !dbg !17001
  %_22.not.i.i = icmp eq i64 %_28.i.i3511, 0, !dbg !16964
  br i1 %_22.not.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %bb13.i.i3509, !dbg !16964

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i: ; preds = %bb13.i.i3509, %bb2.i3508
  %ok.i.sroa.0.0.lcssa.i = phi <8 x i32> [ %1105, %bb2.i3508 ], [ %1117, %bb13.i.i3509 ], !dbg !17003
  %1118 = icmp sgt <8 x i32> %ok.i.sroa.0.0.lcssa.i, splat (i32 -1), !dbg !17004
  %1119 = bitcast <8 x i1> %1118 to i8, !dbg !17004
  %_0.i93.not.i = icmp eq i8 %1119, 0, !dbg !17009
  br i1 %_0.i93.not.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB1z_11LimiterCoreBR_E13process_block0EB1z_.exit, label %bb7.i3502, !dbg !17010

bb7.i3502:                                        ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit25.i
  br i1 %_22.not.i19207.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %bb23.i.i, !dbg !17011

bb23.i.i:                                         ; preds = %bb7.i3502, %bb23.i.i
  %iter.sroa.0.074.i.i = phi ptr [ %_45.i.i, %bb23.i.i ], [ %left_io.0, %bb7.i3502 ]
  %iter.sroa.5.073.i.i = phi i64 [ %_46.i.i3503, %bb23.i.i ], [ %fst_len.i.i, %bb7.i3502 ]
  %ok.sroa.0.072.i.i = phi <8 x i32> [ %1124, %bb23.i.i ], [ %1105, %bb7.i3502 ]
  %lanes.i.sroa.0.0.copyload.i.i = load <8 x i32>, ptr %iter.sroa.0.074.i.i, align 4, !dbg !17022, !alias.scope !17028, !noalias !17034
  %_45.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.074.i.i, i64 32, !dbg !17038
  %_46.i.i3503 = add i64 %iter.sroa.5.073.i.i, -8, !dbg !17045
  %1120 = and <8 x i32> %lanes.i.sroa.0.0.copyload.i.i, splat (i32 2147483647), !dbg !17046
  %1121 = bitcast <8 x i32> %1120 to <8 x float>, !dbg !17053
  %1122 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1121, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !17054
  %1123 = bitcast <8 x float> %1122 to <8 x i32>, !dbg !17060
  %1124 = and <8 x i32> %ok.sroa.0.072.i.i, %1123, !dbg !17064
  %_40.not.i.i = icmp eq i64 %_46.i.i3503, 0, !dbg !17011
  br i1 %_40.not.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %bb23.i.i, !dbg !17011

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i: ; preds = %bb23.i.i, %bb7.i3502
  %ok.sroa.0.0.lcssa.i.i = phi <8 x i32> [ %1105, %bb7.i3502 ], [ %1124, %bb23.i.i ], !dbg !17066
  %1125 = icmp slt <8 x i32> %ok.sroa.0.0.lcssa.i.i, zeroinitializer, !dbg !17067
  %bc.i.i = select <8 x i1> %1125, <8 x i32> zeroinitializer, <8 x i32> splat (i32 1065353216), !dbg !17067
  %1126 = extractelement <8 x i32> %bc.i.i, i64 0, !dbg !17073
  %1127 = icmp ne i32 %1126, 0, !dbg !17073
  %1128 = zext i1 %1127 to i32, !dbg !17073
  %1129 = extractelement <8 x i32> %bc.i.i, i64 1, !dbg !17073
  %1130 = icmp eq i32 %1129, 0, !dbg !17073
  %1131 = select i1 %1130, i32 0, i32 2, !dbg !17073
  %1132 = extractelement <8 x i32> %bc.i.i, i64 2, !dbg !17073
  %1133 = icmp eq i32 %1132, 0, !dbg !17073
  %1134 = select i1 %1133, i32 0, i32 4, !dbg !17073
  %1135 = extractelement <8 x i32> %bc.i.i, i64 3, !dbg !17073
  %1136 = icmp eq i32 %1135, 0, !dbg !17073
  %1137 = select i1 %1136, i32 0, i32 8, !dbg !17073
  %1138 = extractelement <8 x i32> %bc.i.i, i64 4, !dbg !17073
  %1139 = icmp eq i32 %1138, 0, !dbg !17073
  %1140 = select i1 %1139, i32 0, i32 16, !dbg !17073
  %1141 = extractelement <8 x i32> %bc.i.i, i64 5, !dbg !17073
  %1142 = icmp eq i32 %1141, 0, !dbg !17073
  %1143 = select i1 %1142, i32 0, i32 32, !dbg !17073
  %1144 = extractelement <8 x i32> %bc.i.i, i64 6, !dbg !17073
  %1145 = icmp eq i32 %1144, 0, !dbg !17073
  %1146 = select i1 %1145, i32 0, i32 64, !dbg !17073
  %1147 = extractelement <8 x i32> %bc.i.i, i64 7, !dbg !17073
  %1148 = icmp eq i32 %1147, 0, !dbg !17073
  %1149 = select i1 %1148, i32 0, i32 128, !dbg !17073
  %fst_len.i.i108.i = and i64 %right_io.1, 2305843009213693944, !dbg !17077
  %_40.not71.i109.i = icmp eq i64 %fst_len.i.i108.i, 0, !dbg !17081
  br i1 %_40.not71.i109.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit134.i, label %bb23.i110.i, !dbg !17081

bb23.i110.i:                                      ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, %bb23.i110.i
  %iter.sroa.0.074.i111.i = phi ptr [ %_45.i115.i, %bb23.i110.i ], [ %right_io.0, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i ]
  %iter.sroa.5.073.i112.i = phi i64 [ %_46.i116.i, %bb23.i110.i ], [ %fst_len.i.i108.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i ]
  %ok.sroa.0.072.i113.i = phi <8 x i32> [ %1154, %bb23.i110.i ], [ %1105, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i ]
  %lanes.i.sroa.0.0.copyload.i114.i = load <8 x i32>, ptr %iter.sroa.0.074.i111.i, align 4, !dbg !17084, !alias.scope !17089, !noalias !17095
  %_45.i115.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.074.i111.i, i64 32, !dbg !17099
  %_46.i116.i = add i64 %iter.sroa.5.073.i112.i, -8, !dbg !17102
  %1150 = and <8 x i32> %lanes.i.sroa.0.0.copyload.i114.i, splat (i32 2147483647), !dbg !17103
  %1151 = bitcast <8 x i32> %1150 to <8 x float>, !dbg !17109
  %1152 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1151, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !17110
  %1153 = bitcast <8 x float> %1152 to <8 x i32>, !dbg !17116
  %1154 = and <8 x i32> %ok.sroa.0.072.i113.i, %1153, !dbg !17120
  %_40.not.i117.i = icmp eq i64 %_46.i116.i, 0, !dbg !17081
  br i1 %_40.not.i117.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit134.i, label %bb23.i110.i, !dbg !17081

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit134.i: ; preds = %bb23.i110.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i
  %ok.sroa.0.0.lcssa.i118.i = phi <8 x i32> [ %1105, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i ], [ %1154, %bb23.i110.i ], !dbg !17122
  %1155 = icmp slt <8 x i32> %ok.sroa.0.0.lcssa.i118.i, zeroinitializer, !dbg !17123
  %bc.i119.i = select <8 x i1> %1155, <8 x i32> zeroinitializer, <8 x i32> splat (i32 1065353216), !dbg !17123
  %1156 = extractelement <8 x i32> %bc.i119.i, i64 0, !dbg !17128
  %1157 = icmp ne i32 %1156, 0, !dbg !17128
  %1158 = zext i1 %1157 to i32, !dbg !17128
  %1159 = extractelement <8 x i32> %bc.i119.i, i64 1, !dbg !17128
  %1160 = icmp eq i32 %1159, 0, !dbg !17128
  %1161 = select i1 %1160, i32 0, i32 2, !dbg !17128
  %1162 = extractelement <8 x i32> %bc.i119.i, i64 2, !dbg !17128
  %1163 = icmp eq i32 %1162, 0, !dbg !17128
  %1164 = select i1 %1163, i32 0, i32 4, !dbg !17128
  %1165 = extractelement <8 x i32> %bc.i119.i, i64 3, !dbg !17128
  %1166 = icmp eq i32 %1165, 0, !dbg !17128
  %1167 = select i1 %1166, i32 0, i32 8, !dbg !17128
  %1168 = extractelement <8 x i32> %bc.i119.i, i64 4, !dbg !17128
  %1169 = icmp eq i32 %1168, 0, !dbg !17128
  %1170 = select i1 %1169, i32 0, i32 16, !dbg !17128
  %1171 = extractelement <8 x i32> %bc.i119.i, i64 5, !dbg !17128
  %1172 = icmp eq i32 %1171, 0, !dbg !17128
  %1173 = select i1 %1172, i32 0, i32 32, !dbg !17128
  %1174 = extractelement <8 x i32> %bc.i119.i, i64 6, !dbg !17128
  %1175 = icmp eq i32 %1174, 0, !dbg !17128
  %1176 = select i1 %1175, i32 0, i32 64, !dbg !17128
  %1177 = extractelement <8 x i32> %bc.i119.i, i64 7, !dbg !17128
  %1178 = icmp eq i32 %1177, 0, !dbg !17128
  %1179 = select i1 %1178, i32 0, i32 128, !dbg !17128
  %1180 = getelementptr inbounds nuw i8, ptr %self, i64 1576, !dbg !17129
  %mask.sroa.0.1.1.i121.i = or disjoint i32 %1131, %1128, !dbg !17128
  %mask.sroa.0.1.2.i123.i = or disjoint i32 %mask.sroa.0.1.1.i121.i, %1134, !dbg !17128
  %mask.sroa.0.1.3.i125.i = or disjoint i32 %mask.sroa.0.1.2.i123.i, %1137, !dbg !17128
  %mask.sroa.0.1.4.i127.i = or disjoint i32 %mask.sroa.0.1.3.i125.i, %1140, !dbg !17128
  %mask.sroa.0.1.5.i129.i = or disjoint i32 %mask.sroa.0.1.4.i127.i, %1143, !dbg !17128
  %mask.sroa.0.1.6.i131.i = or i32 %mask.sroa.0.1.5.i129.i, %1146, !dbg !17128
  %mask.sroa.0.1.7.i133.i = or i32 %mask.sroa.0.1.6.i131.i, %1149, !dbg !17128
  %mask.sroa.0.1.1.i.i = or i32 %mask.sroa.0.1.7.i133.i, %1158, !dbg !17073
  %mask.sroa.0.1.2.i.i = or i32 %mask.sroa.0.1.1.i.i, %1161, !dbg !17073
  %mask.sroa.0.1.3.i.i = or i32 %mask.sroa.0.1.2.i.i, %1164, !dbg !17073
  %mask.sroa.0.1.4.i.i = or i32 %mask.sroa.0.1.3.i.i, %1167, !dbg !17073
  %mask.sroa.0.1.5.i.i = or i32 %mask.sroa.0.1.4.i.i, %1170, !dbg !17073
  %mask.sroa.0.1.6.i.i = or i32 %mask.sroa.0.1.5.i.i, %1173, !dbg !17073
  %mask.sroa.0.1.7.i.i = or i32 %mask.sroa.0.1.6.i.i, %1176, !dbg !17073
  %1181 = or i32 %mask.sroa.0.1.7.i.i, %1179, !dbg !17129
  store i32 %1181, ptr %1180, align 8, !dbg !17129, !alias.scope !16877, !noalias !17130
  %_14.i3504 = load i64, ptr %_53, align 8, !dbg !17131, !alias.scope !16877, !noalias !17130, !noundef !12
  %1182 = tail call i64 @llvm.uadd.sat.i64(i64 %_14.i3504, i64 1), !dbg !17132
  store i64 %1182, ptr %_53, align 8, !dbg !17135, !alias.scope !16877, !noalias !17130
  %_222.i.i = icmp eq i64 %left_io.1, 0, !dbg !17136
  br i1 %_222.i.i, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %bb14.i.preheader.i, !dbg !17142

bb14.i.preheader.i:                               ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit134.i
  %.idx.i.i = shl nuw nsw i64 %left_io.1, 2, !dbg !17143
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %left_io.0, i8 0, i64 %.idx.i.i, i1 false), !dbg !17147, !alias.scope !17148, !noalias !17151
  br label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i, !dbg !17152

_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i: ; preds = %bb14.i.preheader.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit134.i
  %_222.i136.i = icmp eq i64 %right_io.1, 0, !dbg !17158
  br i1 %_222.i136.i, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit141.i, label %bb14.i137.preheader.i, !dbg !17161

bb14.i137.preheader.i:                            ; preds = %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i
  %.idx.i135.i = shl nuw nsw i64 %right_io.1, 2, !dbg !17152
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %right_io.0, i8 0, i64 %.idx.i135.i, i1 false), !dbg !17162, !alias.scope !17163, !noalias !17166
  br label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit141.i, !dbg !17167

_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit141.i: ; preds = %bb14.i137.preheader.i, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 8 dereferenceable(200) %_33, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_99.0, i64 noundef %_99.1, i32 noundef %1099) #30, !dbg !17168, !noalias !17171
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 8 dereferenceable(200) %_34, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_100.0, i64 noundef %_100.1, i32 noundef %1099) #30, !dbg !17174, !noalias !17171
  store i32 0, ptr %_35, align 4, !dbg !17175, !noalias !17171
  %1183 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !17175
  store i32 0, ptr %1183, align 4, !dbg !17175, !noalias !17171
  br label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB1z_11LimiterCoreBR_E13process_block0EB1z_.exit, !dbg !17176

_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB1z_11LimiterCoreBR_E13process_block0EB1z_.exit: ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit141.i
  call void @llvm.lifetime.end.p0(ptr nonnull %shape), !dbg !17177
  br label %bb42, !dbg !16805

bb54:                                             ; preds = %bb34
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %words, i64 noundef %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_983c0eafc7ebd40f6695894c941b193b) #31, !dbg !17178
  unreachable, !dbg !17178

bb1.i3512:                                        ; preds = %bb34, %bb10.i3527
  %iter.sroa.6.0.i3513 = phi i64 [ %len.i.i.i.i3518, %bb10.i3527 ], [ %words, %bb34 ], !dbg !17179
  %iter.sroa.0.0.i3514 = phi ptr [ %data.i.i.i.i3517, %bb10.i3527 ], [ %right_io.0, %bb34 ], !dbg !17179
  %1184 = icmp eq i64 %iter.sroa.6.0.i3513, 0, !dbg !17181
  br i1 %1184, label %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit3528, label %bb11.preheader.i3515, !dbg !17181

bb11.preheader.i3515:                             ; preds = %bb1.i3512
  %..i.i.i3516 = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i3513, i64 32), !dbg !17183
  %_18.idx.i3519 = shl nuw nsw i64 %..i.i.i3516, 2, !dbg !17186
  %_18.i3520 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i3514, i64 %_18.idx.i3519, !dbg !17186
  br label %bb11.i3521, !dbg !17191

bb11.i3521:                                       ; preds = %bb11.i3521, %bb11.preheader.i3515
  %iter1.sroa.0.014.i3522 = phi ptr [ %_31.i3524, %bb11.i3521 ], [ %iter.sroa.0.0.i3514, %bb11.preheader.i3515 ]
  %bits.sroa.0.013.i3523 = phi i32 [ %1185, %bb11.i3521 ], [ 0, %bb11.preheader.i3515 ]
  %_31.i3524 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i3522, i64 4, !dbg !17193
  %_134.i3525 = load i32, ptr %iter1.sroa.0.014.i3522, align 4, !dbg !17195, !alias.scope !17196, !noundef !12
  %1185 = or i32 %_134.i3525, %bits.sroa.0.013.i3523, !dbg !17199
  %_25.i3526 = icmp eq ptr %_31.i3524, %_18.i3520, !dbg !17200
  br i1 %_25.i3526, label %bb10.i3527, label %bb11.i3521, !dbg !17191

bb10.i3527:                                       ; preds = %bb11.i3521
  %data.i.i.i.i3517 = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i3514, i64 %..i.i.i3516, !dbg !17202
  %len.i.i.i.i3518 = sub nuw nsw i64 %iter.sroa.6.0.i3513, %..i.i.i3516, !dbg !17207
  %1186 = icmp eq i32 %1185, 0, !dbg !17208
  br i1 %1186, label %bb1.i3512, label %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit3528, !dbg !17208

_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit3528: ; preds = %bb1.i3512, %bb10.i3527
  %1187 = zext i1 %1184 to i8, !dbg !16856
  br label %bb40, !dbg !16730

bb42:                                             ; preds = %_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB1z_11LimiterCoreBR_E13process_block0EB1z_.exit
  ret void, !dbg !16805
}
