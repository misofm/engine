define internal fastcc void @_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13process_blockB5_(ptr noalias noundef nonnull align 32 dereferenceable(2176) %self, ptr noalias noundef nonnull align 4 captures(address) %left_io.0, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef nonnull align 4 captures(address) %right_io.0, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 4294967296) %frames) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !26618 {
start:
  %peaks_right.i1520 = alloca [1024 x i8], align 4
  %peaks_left.i1521 = alloca [1024 x i8], align 4
  %scratch.i1522 = alloca [32 x i8], align 4
  %hot_right.i1528 = alloca [736 x i8], align 32
  %hot_left.i1529 = alloca [736 x i8], align 32
  %peaks_right.i980 = alloca [1024 x i8], align 4
  %peaks_left.i981 = alloca [1024 x i8], align 4
  %scratch.i = alloca [32 x i8], align 4
  %hot_right.i987 = alloca [736 x i8], align 32
  %hot_left.i988 = alloca [736 x i8], align 32
  %right_prefix.i454 = alloca [32 x i8], align 32
  %left_prefix.i455 = alloca [32 x i8], align 32
  %uniform_right.i489 = alloca [128 x i8], align 32
  %uniform_left.i490 = alloca [128 x i8], align 32
  %peaks_right.i491 = alloca [1024 x i8], align 4
  %peaks_left.i492 = alloca [1024 x i8], align 4
  %hot_right.i498 = alloca [736 x i8], align 32
  %hot_left.i499 = alloca [736 x i8], align 32
  %right_prefix.i = alloca [32 x i8], align 32
  %left_prefix.i = alloca [32 x i8], align 32
  %uniform_right.i = alloca [128 x i8], align 32
  %uniform_left.i = alloca [128 x i8], align 32
  %peaks_right.i = alloca [1024 x i8], align 4
  %peaks_left.i = alloca [1024 x i8], align 4
  %hot_right.i = alloca [736 x i8], align 32
  %hot_left.i = alloca [736 x i8], align 32
  %shape = alloca [24 x i8], align 8
  %words = shl nuw nsw i64 %frames, 3, !dbg !26619
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 2153, !dbg !26620
  %1 = load i8, ptr %0, align 1, !dbg !26620, !range !17, !noundef !12
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 2144, !dbg !26622
  %3 = load i8, ptr %2, align 32, !dbg !26622, !range !17, !noundef !12
  %_7 = icmp eq i8 %1, %3, !dbg !26620
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 1776
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 1784
  %_95.1 = load i64, ptr %5, align 8, !dbg !26623
  br i1 %_7, label %bb1, label %start.bb20.thread_crit_edge, !dbg !26620

start.bb20.thread_crit_edge:                      ; preds = %start
  %_14.0.i.pre.pre = load ptr, ptr %4, align 8, !dbg !26624, !alias.scope !26629, !noalias !26632
  br label %bb20.thread, !dbg !26620

bb1:                                              ; preds = %start
  %_95.0 = load ptr, ptr %4, align 16, !dbg !26640, !nonnull !12, !noundef !12
  %_8.i7024 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_95.0, i64 %_95.1, !dbg !26641
  br label %bb1.i.i, !dbg !26646

bb1.i.i:                                          ; preds = %bb13.i.i7026, %bb1
  %_221.i.i = phi ptr [ %_22.i.i7027, %bb13.i.i7026 ], [ %_95.0, %bb1 ]
  %_12.i.i7025 = icmp eq ptr %_221.i.i, %_8.i7024, !dbg !26648
  br i1 %_12.i.i7025, label %bb3, label %bb13.i.i7026, !dbg !26651

bb13.i.i7026:                                     ; preds = %bb1.i.i
  %_22.i.i7027 = getelementptr inbounds nuw i8, ptr %_221.i.i, i64 16, !dbg !26652
  %6 = getelementptr inbounds nuw i8, ptr %_221.i.i, i64 12, !dbg !26654
  %_3.i.i.i = load i32, ptr %6, align 4, !dbg !26654, !alias.scope !26656, !noalias !26661, !noundef !12
  %7 = icmp eq i32 %_3.i.i.i, 0, !dbg !26654
  %_51.i.i.i = load i32, ptr %_221.i.i, align 4, !dbg !26654, !alias.scope !26656, !noalias !26661
  %8 = getelementptr inbounds nuw i8, ptr %_221.i.i, i64 4, !dbg !26654
  %_72.i.i.i = load i32, ptr %8, align 4, !dbg !26654, !alias.scope !26656, !noalias !26661
  %9 = icmp eq i32 %_51.i.i.i, %_72.i.i.i, !dbg !26654
  %_0.sroa.0.0.i.i.i = select i1 %7, i1 %9, i1 false, !dbg !26654
  br i1 %_0.sroa.0.0.i.i.i, label %bb1.i.i, label %bb20.thread, !dbg !26664

bb3:                                              ; preds = %bb1.i.i
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 1792, !dbg !26665
  %_96.0 = load ptr, ptr %10, align 16, !dbg !26665, !nonnull !12, !noundef !12
  %11 = getelementptr inbounds nuw i8, ptr %self, i64 1800, !dbg !26665
  %_96.1 = load i64, ptr %11, align 8, !dbg !26665, !noundef !12
  %_8.i7028 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_96.0, i64 %_96.1, !dbg !26666
  br label %bb1.i.i7029, !dbg !26671

bb1.i.i7029:                                      ; preds = %bb13.i.i7032, %bb3
  %_221.i.i7030 = phi ptr [ %_22.i.i7033, %bb13.i.i7032 ], [ %_96.0, %bb3 ]
  %_12.i.i7031 = icmp eq ptr %_221.i.i7030, %_8.i7028, !dbg !26673
  br i1 %_12.i.i7031, label %bb5, label %bb13.i.i7032, !dbg !26676

bb13.i.i7032:                                     ; preds = %bb1.i.i7029
  %_22.i.i7033 = getelementptr inbounds nuw i8, ptr %_221.i.i7030, i64 16, !dbg !26677
  %12 = getelementptr inbounds nuw i8, ptr %_221.i.i7030, i64 12, !dbg !26679
  %_3.i.i.i7034 = load i32, ptr %12, align 4, !dbg !26679, !alias.scope !26681, !noalias !26686, !noundef !12
  %13 = icmp eq i32 %_3.i.i.i7034, 0, !dbg !26679
  %_51.i.i.i7035 = load i32, ptr %_221.i.i7030, align 4, !dbg !26679, !alias.scope !26681, !noalias !26686
  %14 = getelementptr inbounds nuw i8, ptr %_221.i.i7030, i64 4, !dbg !26679
  %_72.i.i.i7036 = load i32, ptr %14, align 4, !dbg !26679, !alias.scope !26681, !noalias !26686
  %15 = icmp eq i32 %_51.i.i.i7035, %_72.i.i.i7036, !dbg !26679
  %_0.sroa.0.0.i.i.i7037 = select i1 %13, i1 %15, i1 false, !dbg !26679
  br i1 %_0.sroa.0.0.i.i.i7037, label %bb1.i.i7029, label %bb20.thread, !dbg !26689

bb5:                                              ; preds = %bb1.i.i7029
  %16 = getelementptr inbounds nuw i8, ptr %self, i64 1976, !dbg !26690
  %_97.0 = load ptr, ptr %16, align 8, !dbg !26690, !nonnull !12, !noundef !12
  %17 = getelementptr inbounds nuw i8, ptr %self, i64 1984, !dbg !26690
  %_97.1 = load i64, ptr %17, align 8, !dbg !26690, !noundef !12
  %_8.i7039 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_97.0, i64 %_97.1, !dbg !26691
  br label %bb1.i.i7040, !dbg !26696

bb1.i.i7040:                                      ; preds = %bb13.i.i7043, %bb5
  %_221.i.i7041 = phi ptr [ %_22.i.i7044, %bb13.i.i7043 ], [ %_97.0, %bb5 ]
  %_12.i.i7042 = icmp eq ptr %_221.i.i7041, %_8.i7039, !dbg !26698
  br i1 %_12.i.i7042, label %bb7, label %bb13.i.i7043, !dbg !26701

bb13.i.i7043:                                     ; preds = %bb1.i.i7040
  %_22.i.i7044 = getelementptr inbounds nuw i8, ptr %_221.i.i7041, i64 16, !dbg !26702
  %18 = getelementptr inbounds nuw i8, ptr %_221.i.i7041, i64 12, !dbg !26704
  %_3.i.i.i7045 = load i32, ptr %18, align 4, !dbg !26704, !alias.scope !26706, !noalias !26711, !noundef !12
  %19 = icmp eq i32 %_3.i.i.i7045, 0, !dbg !26704
  %_51.i.i.i7046 = load i32, ptr %_221.i.i7041, align 4, !dbg !26704, !alias.scope !26706, !noalias !26711
  %20 = getelementptr inbounds nuw i8, ptr %_221.i.i7041, i64 4, !dbg !26704
  %_72.i.i.i7047 = load i32, ptr %20, align 4, !dbg !26704, !alias.scope !26706, !noalias !26711
  %21 = icmp eq i32 %_51.i.i.i7046, %_72.i.i.i7047, !dbg !26704
  %_0.sroa.0.0.i.i.i7048 = select i1 %19, i1 %21, i1 false, !dbg !26704
  br i1 %_0.sroa.0.0.i.i.i7048, label %bb1.i.i7040, label %bb20.thread, !dbg !26714

bb7:                                              ; preds = %bb1.i.i7040
  %22 = getelementptr inbounds nuw i8, ptr %self, i64 1992, !dbg !26715
  %_98.0 = load ptr, ptr %22, align 8, !dbg !26715, !nonnull !12, !noundef !12
  %23 = getelementptr inbounds nuw i8, ptr %self, i64 2000, !dbg !26715
  %_98.1 = load i64, ptr %23, align 8, !dbg !26715, !noundef !12
  %_8.i7050 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_98.0, i64 %_98.1, !dbg !26716
  br label %bb1.i.i7051, !dbg !26721

bb1.i.i7051:                                      ; preds = %bb13.i.i7054, %bb7
  %_221.i.i7052 = phi ptr [ %_22.i.i7055, %bb13.i.i7054 ], [ %_98.0, %bb7 ]
  %_12.i.i7053 = icmp eq ptr %_221.i.i7052, %_8.i7050, !dbg !26723
  br i1 %_12.i.i7053, label %bb9, label %bb13.i.i7054, !dbg !26726

bb13.i.i7054:                                     ; preds = %bb1.i.i7051
  %_22.i.i7055 = getelementptr inbounds nuw i8, ptr %_221.i.i7052, i64 16, !dbg !26727
  %24 = getelementptr inbounds nuw i8, ptr %_221.i.i7052, i64 12, !dbg !26729
  %_3.i.i.i7056 = load i32, ptr %24, align 4, !dbg !26729, !alias.scope !26731, !noalias !26736, !noundef !12
  %25 = icmp eq i32 %_3.i.i.i7056, 0, !dbg !26729
  %_51.i.i.i7057 = load i32, ptr %_221.i.i7052, align 4, !dbg !26729, !alias.scope !26731, !noalias !26736
  %26 = getelementptr inbounds nuw i8, ptr %_221.i.i7052, i64 4, !dbg !26729
  %_72.i.i.i7058 = load i32, ptr %26, align 4, !dbg !26729, !alias.scope !26731, !noalias !26736
  %27 = icmp eq i32 %_51.i.i.i7057, %_72.i.i.i7058, !dbg !26729
  %_0.sroa.0.0.i.i.i7059 = select i1 %25, i1 %27, i1 false, !dbg !26729
  br i1 %_0.sroa.0.0.i.i.i7059, label %bb1.i.i7051, label %bb20.thread, !dbg !26739

bb9:                                              ; preds = %bb1.i.i7051
  %_65.not = icmp samesign ugt i64 %words, %left_io.1
  br i1 %_65.not, label %bb45, label %bb1.i7061, !dbg !26740, !prof !165

bb45:                                             ; preds = %bb9
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %words, i64 noundef %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_34e4746889305f9c657ea46721b345d0) #30, !dbg !26749
  unreachable, !dbg !26749

bb1.i7061:                                        ; preds = %bb9, %bb10.i7067
  %iter.sroa.6.0.i = phi i64 [ %len.i.i.i.i, %bb10.i7067 ], [ %words, %bb9 ], !dbg !26750
  %iter.sroa.0.0.i7062 = phi ptr [ %data.i.i.i.i, %bb10.i7067 ], [ %left_io.0, %bb9 ], !dbg !26750
  %28 = icmp eq i64 %iter.sroa.6.0.i, 0, !dbg !26752
  br i1 %28, label %bb11, label %bb11.preheader.i, !dbg !26752

bb11.preheader.i:                                 ; preds = %bb1.i7061
  %..i.i.i = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i, i64 32), !dbg !26754
  %_18.idx.i = shl nuw nsw i64 %..i.i.i, 2, !dbg !26757
  %_18.i7063 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i7062, i64 %_18.idx.i, !dbg !26757
  br label %bb11.i7064, !dbg !26762

bb11.i7064:                                       ; preds = %bb11.i7064, %bb11.preheader.i
  %iter1.sroa.0.014.i = phi ptr [ %_31.i7065, %bb11.i7064 ], [ %iter.sroa.0.0.i7062, %bb11.preheader.i ]
  %bits.sroa.0.013.i = phi i32 [ %29, %bb11.i7064 ], [ 0, %bb11.preheader.i ]
  %_31.i7065 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i, i64 4, !dbg !26764
  %_134.i = load i32, ptr %iter1.sroa.0.014.i, align 4, !dbg !26766, !alias.scope !26767, !noundef !12
  %29 = or i32 %_134.i, %bits.sroa.0.013.i, !dbg !26770
  %_25.i7066 = icmp eq ptr %_31.i7065, %_18.i7063, !dbg !26771
  br i1 %_25.i7066, label %bb10.i7067, label %bb11.i7064, !dbg !26762

bb10.i7067:                                       ; preds = %bb11.i7064
  %data.i.i.i.i = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i7062, i64 %..i.i.i, !dbg !26773
  %len.i.i.i.i = sub nuw nsw i64 %iter.sroa.6.0.i, %..i.i.i, !dbg !26778
  %30 = icmp eq i32 %29, 0, !dbg !26779
  br i1 %30, label %bb1.i7061, label %bb20.thread, !dbg !26779

bb11:                                             ; preds = %bb1.i7061
  %_73.not = icmp samesign ugt i64 %words, %right_io.1, !dbg !26780
  br i1 %_73.not, label %bb48, label %bb1.i7069, !dbg !26780, !prof !1406

bb20.thread:                                      ; preds = %bb13.i.i7026, %bb13.i.i7032, %bb13.i.i7043, %bb13.i.i7054, %bb10.i7067, %start.bb20.thread_crit_edge
  %_14.0.i.pre = phi ptr [ %_14.0.i.pre.pre, %start.bb20.thread_crit_edge ], [ %_95.0, %bb13.i.i7054 ], [ %_95.0, %bb10.i7067 ], [ %_95.0, %bb13.i.i7032 ], [ %_95.0, %bb13.i.i7043 ], [ %_95.0, %bb13.i.i7026 ], !dbg !26624
  %31 = getelementptr inbounds nuw i8, ptr %self, i64 2152
  br label %bb26, !dbg !26786

bb20:                                             ; preds = %bb1.i7069
  %32 = getelementptr inbounds nuw i8, ptr %self, i64 2152
  %33 = load i8, ptr %32, align 8, !range !17
  %_22 = trunc nuw i8 %33 to i1
  br i1 %_22, label %bb22, label %bb26, !dbg !26786

bb48:                                             ; preds = %bb11
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %words, i64 noundef %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_22a7c5212c1e2b1f20d68ba89a9d6a79) #30, !dbg !26787
  unreachable, !dbg !26787

bb1.i7069:                                        ; preds = %bb11, %bb10.i7084
  %iter.sroa.6.0.i7070 = phi i64 [ %len.i.i.i.i7075, %bb10.i7084 ], [ %words, %bb11 ], !dbg !26788
  %iter.sroa.0.0.i7071 = phi ptr [ %data.i.i.i.i7074, %bb10.i7084 ], [ %right_io.0, %bb11 ], !dbg !26788
  %34 = icmp eq i64 %iter.sroa.6.0.i7070, 0, !dbg !26790
  br i1 %34, label %bb20, label %bb11.preheader.i7072, !dbg !26790

bb11.preheader.i7072:                             ; preds = %bb1.i7069
  %..i.i.i7073 = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i7070, i64 32), !dbg !26792
  %_18.idx.i7076 = shl nuw nsw i64 %..i.i.i7073, 2, !dbg !26795
  %_18.i7077 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i7071, i64 %_18.idx.i7076, !dbg !26795
  br label %bb11.i7078, !dbg !26800

bb11.i7078:                                       ; preds = %bb11.i7078, %bb11.preheader.i7072
  %iter1.sroa.0.014.i7079 = phi ptr [ %_31.i7081, %bb11.i7078 ], [ %iter.sroa.0.0.i7071, %bb11.preheader.i7072 ]
  %bits.sroa.0.013.i7080 = phi i32 [ %35, %bb11.i7078 ], [ 0, %bb11.preheader.i7072 ]
  %_31.i7081 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i7079, i64 4, !dbg !26802
  %_134.i7082 = load i32, ptr %iter1.sroa.0.014.i7079, align 4, !dbg !26804, !alias.scope !26805, !noundef !12
  %35 = or i32 %_134.i7082, %bits.sroa.0.013.i7080, !dbg !26808
  %_25.i7083 = icmp eq ptr %_31.i7081, %_18.i7077, !dbg !26809
  br i1 %_25.i7083, label %bb10.i7084, label %bb11.i7078, !dbg !26800

bb10.i7084:                                       ; preds = %bb11.i7078
  %data.i.i.i.i7074 = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i7071, i64 %..i.i.i7073, !dbg !26811
  %len.i.i.i.i7075 = sub nuw nsw i64 %iter.sroa.6.0.i7070, %..i.i.i7073, !dbg !26816
  %36 = icmp eq i32 %35, 0, !dbg !26817
  br i1 %36, label %bb1.i7069, label %bb20.thread13230, !dbg !26817

bb20.thread13230:                                 ; preds = %bb10.i7084
  %37 = getelementptr inbounds nuw i8, ptr %self, i64 2152
  br label %bb26, !dbg !26786

bb26:                                             ; preds = %bb20.thread13230, %bb20.thread, %bb20
  %_14.0.i = phi ptr [ %_14.0.i.pre, %bb20.thread ], [ %_95.0, %bb20 ], [ %_95.0, %bb20.thread13230 ], !dbg !26624
  %38 = phi ptr [ %31, %bb20.thread ], [ %32, %bb20 ], [ %37, %bb20.thread13230 ]
  %quiet.sroa.0.013229 = phi i1 [ false, %bb20.thread ], [ true, %bb20 ], [ false, %bb20.thread13230 ]
  %_32 = getelementptr inbounds nuw i8, ptr %self, i64 1616, !dbg !26818
  %_33 = getelementptr inbounds nuw i8, ptr %self, i64 1648, !dbg !26819
  %_34 = getelementptr inbounds nuw i8, ptr %self, i64 1848, !dbg !26820
  %_35 = getelementptr inbounds nuw i8, ptr %self, i64 1640, !dbg !26821
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26629), !dbg !26822
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26823), !dbg !26822
  %_8.i7087 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_14.0.i, i64 %_95.1, !dbg !26824
  br label %bb1.i.i7088, !dbg !26829

bb1.i.i7088:                                      ; preds = %bb13.i.i7091, %bb26
  %_221.i.i7089 = phi ptr [ %_22.i.i7092, %bb13.i.i7091 ], [ %_14.0.i, %bb26 ]
  %_12.i.i7090 = icmp eq ptr %_221.i.i7089, %_8.i7087, !dbg !26831
  br i1 %_12.i.i7090, label %bb2.i2115, label %bb13.i.i7091, !dbg !26834

bb13.i.i7091:                                     ; preds = %bb1.i.i7088
  %_22.i.i7092 = getelementptr inbounds nuw i8, ptr %_221.i.i7089, i64 16, !dbg !26835
  %39 = getelementptr inbounds nuw i8, ptr %_221.i.i7089, i64 12, !dbg !26837
  %_3.i.i.i7093 = load i32, ptr %39, align 4, !dbg !26837, !alias.scope !26839, !noalias !26844, !noundef !12
  %40 = icmp eq i32 %_3.i.i.i7093, 0, !dbg !26837
  %_51.i.i.i7094 = load i32, ptr %_221.i.i7089, align 4, !dbg !26837, !alias.scope !26839, !noalias !26844
  %41 = getelementptr inbounds nuw i8, ptr %_221.i.i7089, i64 4, !dbg !26837
  %_72.i.i.i7095 = load i32, ptr %41, align 4, !dbg !26837, !alias.scope !26839, !noalias !26844
  %42 = icmp eq i32 %_51.i.i.i7094, %_72.i.i.i7095, !dbg !26837
  %_0.sroa.0.0.i.i.i7096 = select i1 %40, i1 %42, i1 false, !dbg !26837
  br i1 %_0.sroa.0.0.i.i.i7096, label %bb1.i.i7088, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit, !dbg !26847

bb2.i2115:                                        ; preds = %bb1.i.i7088
  %43 = getelementptr inbounds nuw i8, ptr %self, i64 1792, !dbg !26848
  %_15.0.i = load ptr, ptr %43, align 8, !dbg !26848, !alias.scope !26629, !noalias !26632, !nonnull !12, !noundef !12
  %44 = getelementptr inbounds nuw i8, ptr %self, i64 1800, !dbg !26848
  %_15.1.i = load i64, ptr %44, align 8, !dbg !26848, !alias.scope !26629, !noalias !26632, !noundef !12
  %_8.i7098 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_15.0.i, i64 %_15.1.i, !dbg !26849
  br label %bb1.i.i7099, !dbg !26854

bb1.i.i7099:                                      ; preds = %bb13.i.i7102, %bb2.i2115
  %_221.i.i7100 = phi ptr [ %_22.i.i7103, %bb13.i.i7102 ], [ %_15.0.i, %bb2.i2115 ]
  %_12.i.i7101 = icmp eq ptr %_221.i.i7100, %_8.i7098, !dbg !26856
  br i1 %_12.i.i7101, label %bb4.i2117, label %bb13.i.i7102, !dbg !26859

bb13.i.i7102:                                     ; preds = %bb1.i.i7099
  %_22.i.i7103 = getelementptr inbounds nuw i8, ptr %_221.i.i7100, i64 16, !dbg !26860
  %45 = getelementptr inbounds nuw i8, ptr %_221.i.i7100, i64 12, !dbg !26862
  %_3.i.i.i7104 = load i32, ptr %45, align 4, !dbg !26862, !alias.scope !26864, !noalias !26869, !noundef !12
  %46 = icmp eq i32 %_3.i.i.i7104, 0, !dbg !26862
  %_51.i.i.i7105 = load i32, ptr %_221.i.i7100, align 4, !dbg !26862, !alias.scope !26864, !noalias !26869
  %47 = getelementptr inbounds nuw i8, ptr %_221.i.i7100, i64 4, !dbg !26862
  %_72.i.i.i7106 = load i32, ptr %47, align 4, !dbg !26862, !alias.scope !26864, !noalias !26869
  %48 = icmp eq i32 %_51.i.i.i7105, %_72.i.i.i7106, !dbg !26862
  %_0.sroa.0.0.i.i.i7107 = select i1 %46, i1 %48, i1 false, !dbg !26862
  br i1 %_0.sroa.0.0.i.i.i7107, label %bb1.i.i7099, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit, !dbg !26872

bb4.i2117:                                        ; preds = %bb1.i.i7099
  %49 = getelementptr inbounds nuw i8, ptr %self, i64 1976, !dbg !26873
  %_16.0.i = load ptr, ptr %49, align 8, !dbg !26873, !alias.scope !26823, !noalias !26874, !nonnull !12, !noundef !12
  %50 = getelementptr inbounds nuw i8, ptr %self, i64 1984, !dbg !26873
  %_16.1.i = load i64, ptr %50, align 8, !dbg !26873, !alias.scope !26823, !noalias !26874, !noundef !12
  %_8.i7109 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_16.0.i, i64 %_16.1.i, !dbg !26875
  br label %bb1.i.i7110, !dbg !26880

bb1.i.i7110:                                      ; preds = %bb13.i.i7113, %bb4.i2117
  %_221.i.i7111 = phi ptr [ %_22.i.i7114, %bb13.i.i7113 ], [ %_16.0.i, %bb4.i2117 ]
  %_12.i.i7112 = icmp eq ptr %_221.i.i7111, %_8.i7109, !dbg !26882
  br i1 %_12.i.i7112, label %bb6.i2118, label %bb13.i.i7113, !dbg !26885

bb13.i.i7113:                                     ; preds = %bb1.i.i7110
  %_22.i.i7114 = getelementptr inbounds nuw i8, ptr %_221.i.i7111, i64 16, !dbg !26886
  %51 = getelementptr inbounds nuw i8, ptr %_221.i.i7111, i64 12, !dbg !26888
  %_3.i.i.i7115 = load i32, ptr %51, align 4, !dbg !26888, !alias.scope !26890, !noalias !26895, !noundef !12
  %52 = icmp eq i32 %_3.i.i.i7115, 0, !dbg !26888
  %_51.i.i.i7116 = load i32, ptr %_221.i.i7111, align 4, !dbg !26888, !alias.scope !26890, !noalias !26895
  %53 = getelementptr inbounds nuw i8, ptr %_221.i.i7111, i64 4, !dbg !26888
  %_72.i.i.i7117 = load i32, ptr %53, align 4, !dbg !26888, !alias.scope !26890, !noalias !26895
  %54 = icmp eq i32 %_51.i.i.i7116, %_72.i.i.i7117, !dbg !26888
  %_0.sroa.0.0.i.i.i7118 = select i1 %52, i1 %54, i1 false, !dbg !26888
  br i1 %_0.sroa.0.0.i.i.i7118, label %bb1.i.i7110, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit, !dbg !26898

bb6.i2118:                                        ; preds = %bb1.i.i7110
  %55 = getelementptr inbounds nuw i8, ptr %self, i64 1992, !dbg !26899
  %_17.0.i = load ptr, ptr %55, align 8, !dbg !26899, !alias.scope !26823, !noalias !26874, !nonnull !12, !noundef !12
  %56 = getelementptr inbounds nuw i8, ptr %self, i64 2000, !dbg !26899
  %_17.1.i = load i64, ptr %56, align 8, !dbg !26899, !alias.scope !26823, !noalias !26874, !noundef !12
  %_8.i7120 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_17.0.i, i64 %_17.1.i, !dbg !26900
  br label %bb1.i.i7121, !dbg !26905

bb1.i.i7121:                                      ; preds = %bb13.i.i7124, %bb6.i2118
  %_221.i.i7122 = phi ptr [ %_22.i.i7125, %bb13.i.i7124 ], [ %_17.0.i, %bb6.i2118 ]
  %_12.i.i7123 = icmp eq ptr %_221.i.i7122, %_8.i7120, !dbg !26907
  br i1 %_12.i.i7123, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit, label %bb13.i.i7124, !dbg !26910

bb13.i.i7124:                                     ; preds = %bb1.i.i7121
  %_22.i.i7125 = getelementptr inbounds nuw i8, ptr %_221.i.i7122, i64 16, !dbg !26911
  %57 = getelementptr inbounds nuw i8, ptr %_221.i.i7122, i64 12, !dbg !26913
  %_3.i.i.i7126 = load i32, ptr %57, align 4, !dbg !26913, !alias.scope !26915, !noalias !26920, !noundef !12
  %58 = icmp eq i32 %_3.i.i.i7126, 0, !dbg !26913
  %_51.i.i.i7127 = load i32, ptr %_221.i.i7122, align 4, !dbg !26913, !alias.scope !26915, !noalias !26920
  %59 = getelementptr inbounds nuw i8, ptr %_221.i.i7122, i64 4, !dbg !26913
  %_72.i.i.i7128 = load i32, ptr %59, align 4, !dbg !26913, !alias.scope !26915, !noalias !26920
  %60 = icmp eq i32 %_51.i.i.i7127, %_72.i.i.i7128, !dbg !26913
  %_0.sroa.0.0.i.i.i7129 = select i1 %58, i1 %60, i1 false, !dbg !26913
  br i1 %_0.sroa.0.0.i.i.i7129, label %bb1.i.i7121, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit, !dbg !26923

_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit: ; preds = %bb13.i.i7091, %bb13.i.i7102, %bb13.i.i7113, %bb13.i.i7124, %bb1.i.i7121
  %_0.sroa.0.0.i = phi i1 [ false, %bb13.i.i7113 ], [ false, %bb13.i.i7102 ], [ false, %bb13.i.i7124 ], [ true, %bb1.i.i7121 ], [ false, %bb13.i.i7091 ], !dbg !26924
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26925), !dbg !26928
  %61 = getelementptr inbounds nuw i8, ptr %self, i64 1824, !dbg !26930
  %_31.0.i = load ptr, ptr %61, align 8, !dbg !26930, !alias.scope !26925, !noalias !26932, !nonnull !12, !noundef !12
  %62 = getelementptr inbounds nuw i8, ptr %self, i64 1832, !dbg !26930
  %_31.1.i = load i64, ptr %62, align 8, !dbg !26930, !alias.scope !26925, !noalias !26932, !noundef !12
  %_17.idx.i = mul nuw nsw i64 %_31.1.i, 12, !dbg !26933
  %_17.i = getelementptr inbounds nuw i8, ptr %_31.0.i, i64 %_17.idx.i, !dbg !26933
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26937), !dbg !26940, !noalias !26932
  %_5.not.i.i.i = icmp eq i64 %_31.1.i, 0
  %63 = getelementptr inbounds nuw i8, ptr %_31.0.i, i64 4
  %64 = getelementptr inbounds nuw i8, ptr %_31.0.i, i64 8
  br i1 %_5.not.i.i.i, label %bb2.i7142, label %bb1.i.i7131

bb1.i.i7131:                                      ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i
  %_224.i.i = phi ptr [ %_22.i.i7134, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i ], [ %_31.0.i, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit ]
  %_12.i.i7132 = icmp eq ptr %_224.i.i, %_17.i, !dbg !26941
  br i1 %_12.i.i7132, label %bb2.i7142, label %bb13.i.i7133, !dbg !26945

bb13.i.i7133:                                     ; preds = %bb1.i.i7131
  %_22.i.i7134 = getelementptr inbounds nuw i8, ptr %_224.i.i, i64 12, !dbg !26946
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26948), !dbg !26951, !noalias !26932
  %_9.i.i.i7135 = load i32, ptr %_224.i.i, align 4, !dbg !26952, !alias.scope !26948, !noalias !26955, !noundef !12
  %_10.i.i.i = load i32, ptr %_31.0.i, align 4, !dbg !26952, !alias.scope !26937, !noalias !26957, !noundef !12
  %_8.i.i.i = icmp eq i32 %_9.i.i.i7135, %_10.i.i.i, !dbg !26952
  br i1 %_8.i.i.i, label %bb2.i.i.i, label %bb10.i, !dbg !26952

bb2.i.i.i:                                        ; preds = %bb13.i.i7133
  %65 = getelementptr inbounds nuw i8, ptr %_224.i.i, i64 4, !dbg !26952
  %_12.i.i.i7138 = load i32, ptr %65, align 4, !dbg !26952, !alias.scope !26948, !noalias !26955, !noundef !12
  %_13.i.i.i7139 = load i32, ptr %63, align 4, !dbg !26952, !alias.scope !26937, !noalias !26957, !noundef !12
  %_11.i.i.i = icmp eq i32 %_12.i.i.i7138, %_13.i.i.i7139, !dbg !26952
  br i1 %_11.i.i.i, label %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, label %bb10.i, !dbg !26952

_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i: ; preds = %bb2.i.i.i
  %66 = getelementptr inbounds nuw i8, ptr %_224.i.i, i64 8, !dbg !26952
  %_14.i.i.i7140 = load i32, ptr %66, align 4, !dbg !26952, !alias.scope !26948, !noalias !26955, !noundef !12
  %_15.i.i.i7141 = load i32, ptr %64, align 4, !dbg !26952, !alias.scope !26937, !noalias !26957, !noundef !12
  %67 = icmp eq i32 %_14.i.i.i7140, %_15.i.i.i7141, !dbg !26952
  br i1 %67, label %bb1.i.i7131, label %bb10.i, !dbg !26951

bb2.i7142:                                        ; preds = %bb1.i.i7131, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15dual_stationary.exit
  %68 = getelementptr inbounds nuw i8, ptr %self, i64 1760, !dbg !26958
  %_32.0.i = load ptr, ptr %68, align 8, !dbg !26958, !alias.scope !26925, !noalias !26932, !nonnull !12, !noundef !12
  %69 = getelementptr inbounds nuw i8, ptr %self, i64 1768, !dbg !26958
  %_32.1.i = load i64, ptr %69, align 8, !dbg !26958, !alias.scope !26925, !noalias !26932, !noundef !12
  %_26.idx.i = shl nuw nsw i64 %_32.1.i, 2, !dbg !26959
  %_26.i7143 = getelementptr inbounds nuw i8, ptr %_32.0.i, i64 %_26.idx.i, !dbg !26959
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26963), !dbg !26966, !noalias !26932
  %_6.not.i.i.i = icmp eq i64 %_32.1.i, 0
  br i1 %_6.not.i.i.i, label %bb3.i, label %bb1.i3.i

bb1.i3.i:                                         ; preds = %bb2.i7142, %bb13.i5.i
  %_223.i.i = phi ptr [ %_22.i6.i, %bb13.i5.i ], [ %_32.0.i, %bb2.i7142 ]
  %_12.i4.i = icmp eq ptr %_223.i.i, %_26.i7143, !dbg !26967
  br i1 %_12.i4.i, label %bb3.i, label %bb13.i5.i, !dbg !26971

bb13.i5.i:                                        ; preds = %bb1.i3.i
  %_22.i6.i = getelementptr inbounds nuw i8, ptr %_223.i.i, i64 4, !dbg !26972
  %ptr.val.i.i = load i32, ptr %_223.i.i, align 4, !dbg !26974, !noalias !26975
  %_4.i.i.i7144 = load i32, ptr %_32.0.i, align 4, !dbg !26977, !alias.scope !26963, !noalias !26979, !noundef !12
  %_0.i.i.i = icmp eq i32 %ptr.val.i.i, %_4.i.i.i7144, !dbg !26980
  br i1 %_0.i.i.i, label %bb1.i3.i, label %bb10.i, !dbg !26974

bb3.i:                                            ; preds = %bb1.i3.i, %bb2.i7142
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26981), !dbg !26984
  %70 = getelementptr inbounds nuw i8, ptr %self, i64 2024, !dbg !26985
  %_31.0.i7145 = load ptr, ptr %70, align 8, !dbg !26985, !alias.scope !26981, !noalias !26932, !nonnull !12, !noundef !12
  %71 = getelementptr inbounds nuw i8, ptr %self, i64 2032, !dbg !26985
  %_31.1.i7146 = load i64, ptr %71, align 8, !dbg !26985, !alias.scope !26981, !noalias !26932, !noundef !12
  %_17.idx.i7147 = mul nuw nsw i64 %_31.1.i7146, 12, !dbg !26987
  %_17.i7148 = getelementptr inbounds nuw i8, ptr %_31.0.i7145, i64 %_17.idx.i7147, !dbg !26987
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26991), !dbg !26994, !noalias !26932
  %_5.not.i.i.i7149 = icmp eq i64 %_31.1.i7146, 0
  %72 = getelementptr inbounds nuw i8, ptr %_31.0.i7145, i64 4
  %73 = getelementptr inbounds nuw i8, ptr %_31.0.i7145, i64 8
  br i1 %_5.not.i.i.i7149, label %bb2.i7167, label %bb1.i.i7150

bb1.i.i7150:                                      ; preds = %bb3.i, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i7164
  %_224.i.i7151 = phi ptr [ %_22.i.i7154, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i7164 ], [ %_31.0.i7145, %bb3.i ]
  %_12.i.i7152 = icmp eq ptr %_224.i.i7151, %_17.i7148, !dbg !26995
  br i1 %_12.i.i7152, label %bb2.i7167, label %bb13.i.i7153, !dbg !26999

bb13.i.i7153:                                     ; preds = %bb1.i.i7150
  %_22.i.i7154 = getelementptr inbounds nuw i8, ptr %_224.i.i7151, i64 12, !dbg !27000
  tail call void @llvm.experimental.noalias.scope.decl(metadata !27002), !dbg !27005, !noalias !26932
  %_9.i.i.i7155 = load i32, ptr %_224.i.i7151, align 4, !dbg !27006, !alias.scope !27002, !noalias !27009, !noundef !12
  %_10.i.i.i7156 = load i32, ptr %_31.0.i7145, align 4, !dbg !27006, !alias.scope !26991, !noalias !27011, !noundef !12
  %_8.i.i.i7157 = icmp eq i32 %_9.i.i.i7155, %_10.i.i.i7156, !dbg !27006
  br i1 %_8.i.i.i7157, label %bb2.i.i.i7160, label %bb10.i, !dbg !27006

bb2.i.i.i7160:                                    ; preds = %bb13.i.i7153
  %74 = getelementptr inbounds nuw i8, ptr %_224.i.i7151, i64 4, !dbg !27006
  %_12.i.i.i7161 = load i32, ptr %74, align 4, !dbg !27006, !alias.scope !27002, !noalias !27009, !noundef !12
  %_13.i.i.i7162 = load i32, ptr %72, align 4, !dbg !27006, !alias.scope !26991, !noalias !27011, !noundef !12
  %_11.i.i.i7163 = icmp eq i32 %_12.i.i.i7161, %_13.i.i.i7162, !dbg !27006
  br i1 %_11.i.i.i7163, label %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i7164, label %bb10.i, !dbg !27006

_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i7164: ; preds = %bb2.i.i.i7160
  %75 = getelementptr inbounds nuw i8, ptr %_224.i.i7151, i64 8, !dbg !27006
  %_14.i.i.i7165 = load i32, ptr %75, align 4, !dbg !27006, !alias.scope !27002, !noalias !27009, !noundef !12
  %_15.i.i.i7166 = load i32, ptr %73, align 4, !dbg !27006, !alias.scope !26991, !noalias !27011, !noundef !12
  %76 = icmp eq i32 %_14.i.i.i7165, %_15.i.i.i7166, !dbg !27006
  br i1 %76, label %bb1.i.i7150, label %bb10.i, !dbg !27005

bb2.i7167:                                        ; preds = %bb1.i.i7150, %bb3.i
  %77 = getelementptr inbounds nuw i8, ptr %self, i64 1960, !dbg !27012
  %_32.0.i7168 = load ptr, ptr %77, align 8, !dbg !27012, !alias.scope !26981, !noalias !26932, !nonnull !12, !noundef !12
  %78 = getelementptr inbounds nuw i8, ptr %self, i64 1968, !dbg !27012
  %_32.1.i7169 = load i64, ptr %78, align 8, !dbg !27012, !alias.scope !26981, !noalias !26932, !noundef !12
  %_26.idx.i7170 = shl nuw nsw i64 %_32.1.i7169, 2, !dbg !27013
  %_26.i7171 = getelementptr inbounds nuw i8, ptr %_32.0.i7168, i64 %_26.idx.i7170, !dbg !27013
  tail call void @llvm.experimental.noalias.scope.decl(metadata !27017), !dbg !27020, !noalias !26932
  %_6.not.i.i.i7172 = icmp eq i64 %_32.1.i7169, 0
  br i1 %_6.not.i.i.i7172, label %bb5.i, label %bb1.i3.i7173

bb1.i3.i7173:                                     ; preds = %bb2.i7167, %bb13.i5.i7176
  %_223.i.i7174 = phi ptr [ %_22.i6.i7177, %bb13.i5.i7176 ], [ %_32.0.i7168, %bb2.i7167 ]
  %_12.i4.i7175 = icmp eq ptr %_223.i.i7174, %_26.i7171, !dbg !27021
  br i1 %_12.i4.i7175, label %bb5.i, label %bb13.i5.i7176, !dbg !27025

bb13.i5.i7176:                                    ; preds = %bb1.i3.i7173
  %_22.i6.i7177 = getelementptr inbounds nuw i8, ptr %_223.i.i7174, i64 4, !dbg !27026
  %ptr.val.i.i7178 = load i32, ptr %_223.i.i7174, align 4, !dbg !27028, !noalias !27029
  %_4.i.i.i7179 = load i32, ptr %_32.0.i7168, align 4, !dbg !27031, !alias.scope !27017, !noalias !27033, !noundef !12
  %_0.i.i.i7180 = icmp eq i32 %ptr.val.i.i7178, %_4.i.i.i7179, !dbg !27034
  br i1 %_0.i.i.i7180, label %bb1.i3.i7173, label %bb10.i, !dbg !27028

bb10.i:                                           ; preds = %bb13.i.i7133, %bb2.i.i.i, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, %bb13.i5.i, %bb13.i.i7153, %bb2.i.i.i7160, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i7164, %bb13.i5.i7176
  %79 = getelementptr inbounds nuw i8, ptr %self, i64 1536, !dbg !27035
  %80 = getelementptr inbounds nuw i8, ptr %self, i64 1537, !dbg !27035
  %81 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !27035
  %82 = add nuw nsw i64 %frames, 31, !dbg !27035
  %yield_count.sroa.0.0.i.i7319 = lshr i64 %82, 5, !dbg !27035
  %_116.not.i15609 = icmp eq i64 %yield_count.sroa.0.0.i.i7319, 0, !dbg !27035
  br i1 %_0.sroa.0.0.i, label %bb11.i, label %bb12.i, !dbg !27036

bb5.i:                                            ; preds = %bb1.i3.i7173, %bb2.i7167
  %83 = getelementptr inbounds nuw i8, ptr %self, i64 1536, !dbg !27035
  %84 = getelementptr inbounds nuw i8, ptr %self, i64 1537, !dbg !27035
  %85 = getelementptr inbounds nuw i8, ptr %self, i64 1624, !dbg !27035
  %86 = getelementptr inbounds nuw i8, ptr %self, i64 1632, !dbg !27035
  %87 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !27035
  br i1 %_0.sroa.0.0.i, label %bb6.i, label %bb7.i, !dbg !27037

bb12.i:                                           ; preds = %bb10.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !27038), !dbg !27041
  tail call void @llvm.experimental.noalias.scope.decl(metadata !27042), !dbg !27041
  tail call void @llvm.experimental.noalias.scope.decl(metadata !27044), !dbg !27041
  tail call void @llvm.experimental.noalias.scope.decl(metadata !27046), !dbg !27041
  tail call void @llvm.experimental.noalias.scope.decl(metadata !27048), !dbg !27041
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_left.i1529, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #31, !dbg !27050
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_right.i1528, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #31, !dbg !27054
  %88 = load i8, ptr %79, align 32, !dbg !27056, !range !17, !alias.scope !27038, !noalias !27060, !noundef !12
  %89 = load i8, ptr %80, align 1, !dbg !27063, !range !17, !alias.scope !27038, !noalias !27060, !noundef !12
  %_34.i1536 = load i32, ptr %_35, align 4, !dbg !27065, !alias.scope !27048, !noalias !27067, !noundef !12
  %_36.i1537 = load i32, ptr %81, align 4, !dbg !27068, !alias.scope !27048, !noalias !27067, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i1522), !dbg !27070, !noalias !27072
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i1522, i8 0, i64 32, i1 false), !noalias !27072
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i1521), !dbg !27073, !noalias !27072
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i1521, i8 0, i64 1024, i1 false), !noalias !27072
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i1520), !dbg !27075, !noalias !27072
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i1520, i8 0, i64 1024, i1 false), !noalias !27072
  br i1 %_116.not.i15609, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, label %bb37.i1551.lr.ph, !dbg !27077

bb37.i1551.lr.ph:                                 ; preds = %bb12.i
  %90 = zext i32 %_36.i1537 to i64, !dbg !27068
  %91 = zext i32 %_34.i1536 to i64, !dbg !27065
  %_32.i1533 = trunc nuw i8 %89 to i1, !dbg !27063
  %_31.i1530 = trunc nuw i8 %88 to i1, !dbg !27056
  %92 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !27087
  %93 = bitcast <8 x float> %92 to <8 x i32>, !dbg !27093
  %94 = xor <8 x i32> %93, splat (i32 -1), !dbg !27099
  %history.i315.i1241.sroa.10.0.hot_left.i1529.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1529, i64 32
  %history.i315.i1241.sroa.13.0.hot_left.i1529.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1529, i64 64
  %history.i315.i1241.sroa.16.0.hot_left.i1529.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1529, i64 96
  %history.i315.i1241.sroa.19.0.hot_left.i1529.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1529, i64 128
  %history.i315.i1241.sroa.22.0.hot_left.i1529.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1529, i64 160
  %history.i315.i1241.sroa.25.0.hot_left.i1529.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1529, i64 192
  %history.i315.i1241.sroa.29.0.hot_left.i1529.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1529, i64 224
  %history.i315.i1241.sroa.32.0.hot_left.i1529.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1529, i64 256
  %history.i315.i1241.sroa.35.0.hot_left.i1529.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1529, i64 288
  %history.i315.i1241.sroa.38.0.hot_left.i1529.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1529, i64 320
  %history.i315.i1241.sroa.41.0.hot_left.i1529.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1529, i64 352
  %95 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %96 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %97 = getelementptr inbounds nuw i8, ptr %self, i64 96
  %row12.i.i.i322.i1572 = getelementptr inbounds nuw i8, ptr %self, i64 128
  %98 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %99 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %100 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %row13.i.i.i323.i1573 = getelementptr inbounds nuw i8, ptr %self, i64 256
  %101 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %102 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %103 = getelementptr inbounds nuw i8, ptr %self, i64 352
  %row14.i.i.i324.i1574 = getelementptr inbounds nuw i8, ptr %self, i64 384
  %104 = getelementptr inbounds nuw i8, ptr %self, i64 416
  %105 = getelementptr inbounds nuw i8, ptr %self, i64 448
  %106 = getelementptr inbounds nuw i8, ptr %self, i64 480
  %row15.i.i.i325.i1575 = getelementptr inbounds nuw i8, ptr %self, i64 512
  %107 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %108 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %109 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %row16.i.i.i326.i1576 = getelementptr inbounds nuw i8, ptr %self, i64 640
  %110 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %111 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %112 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %row17.i.i.i327.i1577 = getelementptr inbounds nuw i8, ptr %self, i64 768
  %113 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %114 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %115 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %row18.i.i.i328.i1578 = getelementptr inbounds nuw i8, ptr %self, i64 896
  %116 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %117 = getelementptr inbounds nuw i8, ptr %self, i64 960
  %118 = getelementptr inbounds nuw i8, ptr %self, i64 992
  %row19.i.i.i329.i1579 = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %119 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %120 = getelementptr inbounds nuw i8, ptr %self, i64 1088
  %121 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %row20.i.i.i330.i1580 = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %122 = getelementptr inbounds nuw i8, ptr %self, i64 1184
  %123 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %124 = getelementptr inbounds nuw i8, ptr %self, i64 1248
  %row21.i.i.i331.i1581 = getelementptr inbounds nuw i8, ptr %self, i64 1280
  %125 = getelementptr inbounds nuw i8, ptr %self, i64 1312
  %126 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %127 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %row22.i.i.i332.i1582 = getelementptr inbounds nuw i8, ptr %self, i64 1408
  %128 = getelementptr inbounds nuw i8, ptr %self, i64 1440
  %129 = getelementptr inbounds nuw i8, ptr %self, i64 1472
  %130 = getelementptr inbounds nuw i8, ptr %self, i64 1504
  %history.i.i1431.sroa.10.0.hot_right.i1528.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1528, i64 32
  %history.i.i1431.sroa.13.0.hot_right.i1528.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1528, i64 64
  %history.i.i1431.sroa.16.0.hot_right.i1528.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1528, i64 96
  %history.i.i1431.sroa.19.0.hot_right.i1528.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1528, i64 128
  %history.i.i1431.sroa.22.0.hot_right.i1528.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1528, i64 160
  %history.i.i1431.sroa.25.0.hot_right.i1528.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1528, i64 192
  %history.i.i1431.sroa.29.0.hot_right.i1528.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1528, i64 224
  %history.i.i1431.sroa.32.0.hot_right.i1528.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1528, i64 256
  %history.i.i1431.sroa.35.0.hot_right.i1528.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1528, i64 288
  %history.i.i1431.sroa.38.0.hot_right.i1528.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1528, i64 320
  %history.i.i1431.sroa.41.0.hot_right.i1528.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1528, i64 352
  %_68.i1625 = getelementptr inbounds nuw i8, ptr %hot_left.i1529, i64 384
  %_69.i1626 = getelementptr inbounds nuw i8, ptr %hot_left.i1529, i64 512
  %131 = getelementptr inbounds nuw i8, ptr %hot_left.i1529, i64 480
  %132 = getelementptr inbounds nuw i8, ptr %hot_left.i1529, i64 448
  %133 = getelementptr inbounds nuw i8, ptr %hot_left.i1529, i64 416
  %134 = getelementptr inbounds nuw i8, ptr %hot_left.i1529, i64 608
  %135 = getelementptr inbounds nuw i8, ptr %hot_left.i1529, i64 576
  %136 = getelementptr inbounds nuw i8, ptr %hot_left.i1529, i64 544
  %_73.i1627 = getelementptr inbounds nuw i8, ptr %hot_right.i1528, i64 384
  %_74.i1628 = getelementptr inbounds nuw i8, ptr %hot_right.i1528, i64 512
  %137 = getelementptr inbounds nuw i8, ptr %hot_right.i1528, i64 480
  %138 = getelementptr inbounds nuw i8, ptr %hot_right.i1528, i64 448
  %139 = getelementptr inbounds nuw i8, ptr %hot_right.i1528, i64 416
  %140 = getelementptr inbounds nuw i8, ptr %hot_right.i1528, i64 608
  %141 = getelementptr inbounds nuw i8, ptr %hot_right.i1528, i64 576
  %142 = getelementptr inbounds nuw i8, ptr %hot_right.i1528, i64 544
  %143 = select i1 %_31.i1530, <8 x i32> %93, <8 x i32> %94
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
  %155 = getelementptr inbounds nuw i8, ptr %hot_left.i1529, i64 672
  %156 = getelementptr inbounds nuw i8, ptr %hot_left.i1529, i64 704
  %157 = getelementptr inbounds nuw i8, ptr %hot_left.i1529, i64 640
  %158 = getelementptr inbounds nuw i8, ptr %self, i64 1672
  %159 = getelementptr inbounds nuw i8, ptr %self, i64 1664
  %160 = select i1 %_32.i1533, <8 x i32> %93, <8 x i32> %94
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
  %173 = getelementptr inbounds nuw i8, ptr %hot_right.i1528, i64 672
  %174 = getelementptr inbounds nuw i8, ptr %hot_right.i1528, i64 704
  %175 = getelementptr inbounds nuw i8, ptr %hot_right.i1528, i64 640
  %176 = getelementptr inbounds nuw i8, ptr %self, i64 1872
  %177 = getelementptr inbounds nuw i8, ptr %self, i64 1864
  %178 = getelementptr inbounds nuw i8, ptr %self, i64 1632
  %iter.i49.i1453.sroa.0.0.ptr15355.1 = getelementptr inbounds nuw i8, ptr %scratch.i1522, i64 4
  %iter.i49.i1453.sroa.0.0.ptr15355.2 = getelementptr inbounds nuw i8, ptr %scratch.i1522, i64 8
  %iter.i49.i1453.sroa.0.0.ptr15355.3 = getelementptr inbounds nuw i8, ptr %scratch.i1522, i64 12
  %iter.i49.i1453.sroa.0.0.ptr15355.4 = getelementptr inbounds nuw i8, ptr %scratch.i1522, i64 16
  %iter.i49.i1453.sroa.0.0.ptr15355.5 = getelementptr inbounds nuw i8, ptr %scratch.i1522, i64 20
  %iter.i49.i1453.sroa.0.0.ptr15355.6 = getelementptr inbounds nuw i8, ptr %scratch.i1522, i64 24
  %iter.i49.i1453.sroa.0.0.ptr15355.7 = getelementptr inbounds nuw i8, ptr %scratch.i1522, i64 28
  %iter.i.i1486.sroa.0.0.ptr15366.1 = getelementptr inbounds nuw i8, ptr %scratch.i1522, i64 4
  %iter.i.i1486.sroa.0.0.ptr15366.2 = getelementptr inbounds nuw i8, ptr %scratch.i1522, i64 8
  %iter.i.i1486.sroa.0.0.ptr15366.3 = getelementptr inbounds nuw i8, ptr %scratch.i1522, i64 12
  %iter.i.i1486.sroa.0.0.ptr15366.4 = getelementptr inbounds nuw i8, ptr %scratch.i1522, i64 16
  %iter.i.i1486.sroa.0.0.ptr15366.5 = getelementptr inbounds nuw i8, ptr %scratch.i1522, i64 20
  %iter.i.i1486.sroa.0.0.ptr15366.6 = getelementptr inbounds nuw i8, ptr %scratch.i1522, i64 24
  %iter.i.i1486.sroa.0.0.ptr15366.7 = getelementptr inbounds nuw i8, ptr %scratch.i1522, i64 28
  br label %bb37.i1551, !dbg !27077

bb13.i1545.loopexit.loopexit:                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
  br label %bb13.i1545.loopexit, !dbg !27077

bb13.i1545.loopexit:                              ; preds = %bb13.i1545.loopexit.loopexit, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1616
  %ring_cursor.sroa.0.1.i1619.lcssa = phi i64 [ %ring_cursor.sroa.0.0.i154815376, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1616 ], [ %spec.store.select13.i1764, %bb13.i1545.loopexit.loopexit ], !dbg !27127
  %main_cursor.sroa.0.1.i1620.lcssa = phi i64 [ %main_cursor.sroa.0.0.i154915377, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1616 ], [ %spec.store.select.i1762, %bb13.i1545.loopexit.loopexit ], !dbg !27128
  %_116.not.i1550 = icmp eq i64 %181, 0, !dbg !27077
  %indvars.iv.next = add nsw i64 %indvars.iv, -32, !dbg !27077
  br i1 %_116.not.i1550, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit, label %bb37.i1551, !dbg !27077

bb37.i1551:                                       ; preds = %bb37.i1551.lr.ph, %bb13.i1545.loopexit
  %indvars.iv = phi i64 [ %frames, %bb37.i1551.lr.ph ], [ %indvars.iv.next, %bb13.i1545.loopexit ]
  %main_cursor.sroa.0.0.i154915377 = phi i64 [ %91, %bb37.i1551.lr.ph ], [ %main_cursor.sroa.0.1.i1620.lcssa, %bb13.i1545.loopexit ]
  %ring_cursor.sroa.0.0.i154815376 = phi i64 [ %90, %bb37.i1551.lr.ph ], [ %ring_cursor.sroa.0.1.i1619.lcssa, %bb13.i1545.loopexit ]
  %iter4.sroa.0.0.i154715375 = phi i64 [ %yield_count.sroa.0.0.i.i7319, %bb37.i1551.lr.ph ], [ %181, %bb13.i1545.loopexit ]
  %iter.sroa.0.0.i154615374 = phi i64 [ 0, %bb37.i1551.lr.ph ], [ %180, %bb13.i1545.loopexit ]
  %179 = call i64 @llvm.umax.i64(i64 %indvars.iv, i64 1), !dbg !27129
  %umax17991 = call i64 @llvm.umin.i64(i64 %179, i64 32), !dbg !27129
  %180 = add nuw nsw i64 %iter.sroa.0.0.i154615374, 32, !dbg !27129
  %181 = add nsw i64 %iter4.sroa.0.0.i154715375, -1, !dbg !27133
  %_45.i1553 = sub nsw i64 %frames, %iter.sroa.0.0.i154615374, !dbg !27134
  %..i = tail call noundef i64 @llvm.umin.i64(i64 %_45.i1553, i64 32), !dbg !27135
  %active_base.i1555 = shl i64 %iter.sroa.0.0.i154615374, 3, !dbg !27139
  %active_base.i155513412 = add nuw i64 %..i, %iter.sroa.0.0.i154615374, !dbg !27140
  %_51.i1557 = shl i64 %active_base.i155513412, 3, !dbg !27140
  %_128.i1558 = icmp samesign ult i64 %_51.i1557, %active_base.i1555, !dbg !27141
  %_122.not.i1559 = icmp ugt i64 %_51.i1557, %left_io.1
  %or.cond.i1560 = or i1 %_128.i1558, %_122.not.i1559, !dbg !27141
  br i1 %or.cond.i1560, label %bb41.i1779, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit, !dbg !27141, !prof !165

bb41.i1779:                                       ; preds = %bb37.i1551
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %active_base.i1555, i64 noundef %_51.i1557, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5cbcacccdbb7b907c55dbc42154fcf08) #30, !dbg !27148, !noalias !27149
  unreachable, !dbg !27148

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit: ; preds = %bb37.i1551
  %_131.i1565 = getelementptr inbounds nuw float, ptr %left_io.0, i64 %active_base.i1555, !dbg !27150
  %history.i315.i1241.sroa.0.0.copyload = load <8 x float>, ptr %hot_left.i1529, align 32, !dbg !27154
  %history.i315.i1241.sroa.10.sroa.0.0.copyload = load <8 x float>, ptr %history.i315.i1241.sroa.10.0.hot_left.i1529.sroa_idx, align 32, !dbg !27154
  %history.i315.i1241.sroa.13.sroa.0.0.copyload = load <8 x float>, ptr %history.i315.i1241.sroa.13.0.hot_left.i1529.sroa_idx, align 32, !dbg !27154
  %history.i315.i1241.sroa.16.sroa.0.0.copyload = load <8 x float>, ptr %history.i315.i1241.sroa.16.0.hot_left.i1529.sroa_idx, align 32, !dbg !27154
  %history.i315.i1241.sroa.19.sroa.0.0.copyload = load <8 x float>, ptr %history.i315.i1241.sroa.19.0.hot_left.i1529.sroa_idx, align 32, !dbg !27154
  %history.i315.i1241.sroa.22.sroa.0.0.copyload = load <8 x float>, ptr %history.i315.i1241.sroa.22.0.hot_left.i1529.sroa_idx, align 32, !dbg !27154
  %history.i315.i1241.sroa.25.sroa.0.0.copyload = load <8 x float>, ptr %history.i315.i1241.sroa.25.0.hot_left.i1529.sroa_idx, align 32, !dbg !27154
  %history.i315.i1241.sroa.29.sroa.0.0.copyload = load <8 x float>, ptr %history.i315.i1241.sroa.29.0.hot_left.i1529.sroa_idx, align 32, !dbg !27154
  %history.i315.i1241.sroa.32.sroa.0.0.copyload = load <8 x float>, ptr %history.i315.i1241.sroa.32.0.hot_left.i1529.sroa_idx, align 32, !dbg !27154
  %history.i315.i1241.sroa.35.sroa.0.0.copyload = load <8 x float>, ptr %history.i315.i1241.sroa.35.0.hot_left.i1529.sroa_idx, align 32, !dbg !27154
  %history.i315.i1241.sroa.38.sroa.0.0.copyload = load <8 x float>, ptr %history.i315.i1241.sroa.38.0.hot_left.i1529.sroa_idx, align 32, !dbg !27154
  %history.i315.i1241.sroa.41.sroa.0.0.copyload = load <8 x float>, ptr %history.i315.i1241.sroa.41.0.hot_left.i1529.sroa_idx, align 32, !dbg !27154
  %_2.i718715294.not = icmp eq i64 %frames, %iter.sroa.0.0.i154615374, !dbg !27156
  br i1 %_2.i718715294.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit339.i1589, label %bb6.i318.i1568.lr.ph, !dbg !27156

bb6.i318.i1568.lr.ph:                             ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit
  %_5.i4639 = load <8 x float>, ptr %self, align 32, !alias.scope !27159, !noalias !27162
  %_14.i.i.i275.i1201.sroa.0.0.copyload = load <8 x float>, ptr %95, align 32, !noalias !27174
  %_17.i.i.i272.i1198.sroa.0.0.copyload = load <8 x float>, ptr %96, align 32, !noalias !27174
  %_20.i.i.i269.i1195.sroa.0.0.copyload = load <8 x float>, ptr %97, align 32, !noalias !27174
  %_25.i.i.i265.i1191.sroa.0.0.copyload = load <8 x float>, ptr %row12.i.i.i322.i1572, align 32, !noalias !27174
  %_28.i.i.i262.i1188.sroa.0.0.copyload = load <8 x float>, ptr %98, align 32, !noalias !27174
  %_31.i.i.i259.i1185.sroa.0.0.copyload = load <8 x float>, ptr %99, align 32, !noalias !27174
  %_34.i.i.i256.i1182.sroa.0.0.copyload = load <8 x float>, ptr %100, align 32, !noalias !27174
  %_39.i.i.i252.i1178.sroa.0.0.copyload = load <8 x float>, ptr %row13.i.i.i323.i1573, align 32, !noalias !27174
  %_42.i.i.i249.i1175.sroa.0.0.copyload = load <8 x float>, ptr %101, align 32, !noalias !27174
  %_45.i.i.i246.i1172.sroa.0.0.copyload = load <8 x float>, ptr %102, align 32, !noalias !27174
  %_48.i.i.i243.i1169.sroa.0.0.copyload = load <8 x float>, ptr %103, align 32, !noalias !27174
  %_53.i.i.i239.i1165.sroa.0.0.copyload = load <8 x float>, ptr %row14.i.i.i324.i1574, align 32, !noalias !27174
  %_56.i.i.i236.i1162.sroa.0.0.copyload = load <8 x float>, ptr %104, align 32, !noalias !27174
  %_59.i.i.i233.i1159.sroa.0.0.copyload = load <8 x float>, ptr %105, align 32, !noalias !27174
  %_62.i.i.i230.i1156.sroa.0.0.copyload = load <8 x float>, ptr %106, align 32, !noalias !27174
  %_67.i.i.i226.i1152.sroa.0.0.copyload = load <8 x float>, ptr %row15.i.i.i325.i1575, align 32, !noalias !27174
  %_70.i.i.i223.i1149.sroa.0.0.copyload = load <8 x float>, ptr %107, align 32, !noalias !27174
  %_73.i.i.i220.i1146.sroa.0.0.copyload = load <8 x float>, ptr %108, align 32, !noalias !27174
  %_76.i.i.i217.i1143.sroa.0.0.copyload = load <8 x float>, ptr %109, align 32, !noalias !27174
  %_81.i.i.i213.i1139.sroa.0.0.copyload = load <8 x float>, ptr %row16.i.i.i326.i1576, align 32, !noalias !27174
  %_84.i.i.i210.i1136.sroa.0.0.copyload = load <8 x float>, ptr %110, align 32, !noalias !27174
  %_87.i.i.i207.i1133.sroa.0.0.copyload = load <8 x float>, ptr %111, align 32, !noalias !27174
  %_90.i.i.i204.i1130.sroa.0.0.copyload = load <8 x float>, ptr %112, align 32, !noalias !27174
  %_95.i.i.i200.i1126.sroa.0.0.copyload = load <8 x float>, ptr %row17.i.i.i327.i1577, align 32, !noalias !27174
  %_98.i.i.i197.i1123.sroa.0.0.copyload = load <8 x float>, ptr %113, align 32, !noalias !27174
  %_101.i.i.i194.i1120.sroa.0.0.copyload = load <8 x float>, ptr %114, align 32, !noalias !27174
  %_104.i.i.i191.i1117.sroa.0.0.copyload = load <8 x float>, ptr %115, align 32, !noalias !27174
  %_109.i.i.i187.i1113.sroa.0.0.copyload = load <8 x float>, ptr %row18.i.i.i328.i1578, align 32, !noalias !27174
  %_112.i.i.i184.i1110.sroa.0.0.copyload = load <8 x float>, ptr %116, align 32, !noalias !27174
  %_115.i.i.i181.i1107.sroa.0.0.copyload = load <8 x float>, ptr %117, align 32, !noalias !27174
  %_118.i.i.i178.i1104.sroa.0.0.copyload = load <8 x float>, ptr %118, align 32, !noalias !27174
  %_123.i.i.i174.i1100.sroa.0.0.copyload = load <8 x float>, ptr %row19.i.i.i329.i1579, align 32, !noalias !27174
  %_126.i.i.i171.i1097.sroa.0.0.copyload = load <8 x float>, ptr %119, align 32, !noalias !27174
  %_129.i.i.i168.i1094.sroa.0.0.copyload = load <8 x float>, ptr %120, align 32, !noalias !27174
  %_132.i.i.i165.i1091.sroa.0.0.copyload = load <8 x float>, ptr %121, align 32, !noalias !27174
  %_137.i.i.i161.i1087.sroa.0.0.copyload = load <8 x float>, ptr %row20.i.i.i330.i1580, align 32, !noalias !27174
  %_140.i.i.i158.i1084.sroa.0.0.copyload = load <8 x float>, ptr %122, align 32, !noalias !27174
  %_143.i.i.i155.i1081.sroa.0.0.copyload = load <8 x float>, ptr %123, align 32, !noalias !27174
  %_146.i.i.i152.i1078.sroa.0.0.copyload = load <8 x float>, ptr %124, align 32, !noalias !27174
  %_151.i.i.i148.i1074.sroa.0.0.copyload = load <8 x float>, ptr %row21.i.i.i331.i1581, align 32, !noalias !27174
  %_154.i.i.i145.i1071.sroa.0.0.copyload = load <8 x float>, ptr %125, align 32, !noalias !27174
  %_157.i.i.i142.i1068.sroa.0.0.copyload = load <8 x float>, ptr %126, align 32, !noalias !27174
  %_160.i.i.i139.i1065.sroa.0.0.copyload = load <8 x float>, ptr %127, align 32, !noalias !27174
  %_165.i.i.i135.i1061.sroa.0.0.copyload = load <8 x float>, ptr %row22.i.i.i332.i1582, align 32, !noalias !27174
  %_168.i.i.i132.i1058.sroa.0.0.copyload = load <8 x float>, ptr %128, align 32, !noalias !27174
  %_171.i.i.i129.i1055.sroa.0.0.copyload = load <8 x float>, ptr %129, align 32, !noalias !27174
  %_174.i.i.i126.i1052.sroa.0.0.copyload = load <8 x float>, ptr %130, align 32, !noalias !27174
  br label %bb6.i318.i1568, !dbg !27156

bb6.i318.i1568:                                   ; preds = %bb6.i318.i1568.lr.ph, %bb6.i318.i1568
  %history.i315.i1241.sroa.10.sroa.0.015306 = phi <8 x float> [ %history.i315.i1241.sroa.10.sroa.0.0.copyload, %bb6.i318.i1568.lr.ph ], [ %history.i315.i1241.sroa.0.015295, %bb6.i318.i1568 ]
  %history.i315.i1241.sroa.13.sroa.0.015305 = phi <8 x float> [ %history.i315.i1241.sroa.13.sroa.0.0.copyload, %bb6.i318.i1568.lr.ph ], [ %history.i315.i1241.sroa.10.sroa.0.015306, %bb6.i318.i1568 ]
  %history.i315.i1241.sroa.16.sroa.0.015304 = phi <8 x float> [ %history.i315.i1241.sroa.16.sroa.0.0.copyload, %bb6.i318.i1568.lr.ph ], [ %history.i315.i1241.sroa.13.sroa.0.015305, %bb6.i318.i1568 ]
  %history.i315.i1241.sroa.19.sroa.0.015303 = phi <8 x float> [ %history.i315.i1241.sroa.19.sroa.0.0.copyload, %bb6.i318.i1568.lr.ph ], [ %history.i315.i1241.sroa.16.sroa.0.015304, %bb6.i318.i1568 ]
  %history.i315.i1241.sroa.22.sroa.0.015302 = phi <8 x float> [ %history.i315.i1241.sroa.22.sroa.0.0.copyload, %bb6.i318.i1568.lr.ph ], [ %history.i315.i1241.sroa.19.sroa.0.015303, %bb6.i318.i1568 ]
  %history.i315.i1241.sroa.38.sroa.0.015301 = phi <8 x float> [ %history.i315.i1241.sroa.38.sroa.0.0.copyload, %bb6.i318.i1568.lr.ph ], [ %history.i315.i1241.sroa.35.sroa.0.015300, %bb6.i318.i1568 ]
  %history.i315.i1241.sroa.35.sroa.0.015300 = phi <8 x float> [ %history.i315.i1241.sroa.35.sroa.0.0.copyload, %bb6.i318.i1568.lr.ph ], [ %history.i315.i1241.sroa.32.sroa.0.015299, %bb6.i318.i1568 ]
  %history.i315.i1241.sroa.32.sroa.0.015299 = phi <8 x float> [ %history.i315.i1241.sroa.32.sroa.0.0.copyload, %bb6.i318.i1568.lr.ph ], [ %history.i315.i1241.sroa.29.sroa.0.015298, %bb6.i318.i1568 ]
  %history.i315.i1241.sroa.29.sroa.0.015298 = phi <8 x float> [ %history.i315.i1241.sroa.29.sroa.0.0.copyload, %bb6.i318.i1568.lr.ph ], [ %history.i315.i1241.sroa.25.sroa.0.015297, %bb6.i318.i1568 ]
  %history.i315.i1241.sroa.25.sroa.0.015297 = phi <8 x float> [ %history.i315.i1241.sroa.25.sroa.0.0.copyload, %bb6.i318.i1568.lr.ph ], [ %history.i315.i1241.sroa.22.sroa.0.015302, %bb6.i318.i1568 ]
  %iter.i311.i1237.sroa.16.015296 = phi i64 [ 0, %bb6.i318.i1568.lr.ph ], [ %287, %bb6.i318.i1568 ]
  %history.i315.i1241.sroa.0.015295 = phi <8 x float> [ %history.i315.i1241.sroa.0.0.copyload, %bb6.i318.i1568.lr.ph ], [ %lanes.i5938.sroa.0.0.copyload, %bb6.i318.i1568 ]
  %start1.i.i = shl i64 %iter.i311.i1237.sroa.16.015296, 3, !dbg !27177
  %data.i.i7190 = getelementptr inbounds nuw float, ptr %_131.i1565, i64 %start1.i.i, !dbg !27179
  %lanes.i5938.sroa.0.0.copyload = load <8 x float>, ptr %data.i.i7190, align 4, !dbg !27181, !alias.scope !27186, !noalias !27190
  %182 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i315.i1241.sroa.22.sroa.0.015302), !dbg !27195
  %183 = fmul <8 x float> %lanes.i5938.sroa.0.0.copyload, %_5.i4639, !dbg !27202
  %184 = fadd <8 x float> %183, zeroinitializer, !dbg !27208
  %185 = fmul <8 x float> %lanes.i5938.sroa.0.0.copyload, %_14.i.i.i275.i1201.sroa.0.0.copyload, !dbg !27213
  %186 = fadd <8 x float> %185, zeroinitializer, !dbg !27218
  %187 = fmul <8 x float> %lanes.i5938.sroa.0.0.copyload, %_17.i.i.i272.i1198.sroa.0.0.copyload, !dbg !27223
  %188 = fadd <8 x float> %187, zeroinitializer, !dbg !27228
  %189 = fmul <8 x float> %lanes.i5938.sroa.0.0.copyload, %_20.i.i.i269.i1195.sroa.0.0.copyload, !dbg !27233
  %190 = fadd <8 x float> %189, zeroinitializer, !dbg !27238
  %191 = fmul <8 x float> %history.i315.i1241.sroa.0.015295, %_25.i.i.i265.i1191.sroa.0.0.copyload, !dbg !27243
  %192 = fadd <8 x float> %184, %191, !dbg !27248
  %193 = fmul <8 x float> %history.i315.i1241.sroa.0.015295, %_28.i.i.i262.i1188.sroa.0.0.copyload, !dbg !27253
  %194 = fadd <8 x float> %186, %193, !dbg !27258
  %195 = fmul <8 x float> %history.i315.i1241.sroa.0.015295, %_31.i.i.i259.i1185.sroa.0.0.copyload, !dbg !27263
  %196 = fadd <8 x float> %188, %195, !dbg !27268
  %197 = fmul <8 x float> %history.i315.i1241.sroa.0.015295, %_34.i.i.i256.i1182.sroa.0.0.copyload, !dbg !27273
  %198 = fadd <8 x float> %190, %197, !dbg !27278
  %199 = fmul <8 x float> %history.i315.i1241.sroa.10.sroa.0.015306, %_39.i.i.i252.i1178.sroa.0.0.copyload, !dbg !27283
  %200 = fadd <8 x float> %192, %199, !dbg !27288
  %201 = fmul <8 x float> %history.i315.i1241.sroa.10.sroa.0.015306, %_42.i.i.i249.i1175.sroa.0.0.copyload, !dbg !27293
  %202 = fadd <8 x float> %194, %201, !dbg !27298
  %203 = fmul <8 x float> %history.i315.i1241.sroa.10.sroa.0.015306, %_45.i.i.i246.i1172.sroa.0.0.copyload, !dbg !27303
  %204 = fadd <8 x float> %196, %203, !dbg !27308
  %205 = fmul <8 x float> %history.i315.i1241.sroa.10.sroa.0.015306, %_48.i.i.i243.i1169.sroa.0.0.copyload, !dbg !27313
  %206 = fadd <8 x float> %198, %205, !dbg !27318
  %207 = fmul <8 x float> %history.i315.i1241.sroa.13.sroa.0.015305, %_53.i.i.i239.i1165.sroa.0.0.copyload, !dbg !27323
  %208 = fadd <8 x float> %200, %207, !dbg !27328
  %209 = fmul <8 x float> %history.i315.i1241.sroa.13.sroa.0.015305, %_56.i.i.i236.i1162.sroa.0.0.copyload, !dbg !27333
  %210 = fadd <8 x float> %202, %209, !dbg !27338
  %211 = fmul <8 x float> %history.i315.i1241.sroa.13.sroa.0.015305, %_59.i.i.i233.i1159.sroa.0.0.copyload, !dbg !27343
  %212 = fadd <8 x float> %204, %211, !dbg !27348
  %213 = fmul <8 x float> %history.i315.i1241.sroa.13.sroa.0.015305, %_62.i.i.i230.i1156.sroa.0.0.copyload, !dbg !27353
  %214 = fadd <8 x float> %206, %213, !dbg !27358
  %215 = fmul <8 x float> %history.i315.i1241.sroa.16.sroa.0.015304, %_67.i.i.i226.i1152.sroa.0.0.copyload, !dbg !27363
  %216 = fadd <8 x float> %208, %215, !dbg !27368
  %217 = fmul <8 x float> %history.i315.i1241.sroa.16.sroa.0.015304, %_70.i.i.i223.i1149.sroa.0.0.copyload, !dbg !27373
  %218 = fadd <8 x float> %210, %217, !dbg !27378
  %219 = fmul <8 x float> %history.i315.i1241.sroa.16.sroa.0.015304, %_73.i.i.i220.i1146.sroa.0.0.copyload, !dbg !27383
  %220 = fadd <8 x float> %212, %219, !dbg !27388
  %221 = fmul <8 x float> %history.i315.i1241.sroa.16.sroa.0.015304, %_76.i.i.i217.i1143.sroa.0.0.copyload, !dbg !27393
  %222 = fadd <8 x float> %214, %221, !dbg !27398
  %223 = fmul <8 x float> %history.i315.i1241.sroa.19.sroa.0.015303, %_81.i.i.i213.i1139.sroa.0.0.copyload, !dbg !27403
  %224 = fadd <8 x float> %216, %223, !dbg !27408
  %225 = fmul <8 x float> %history.i315.i1241.sroa.19.sroa.0.015303, %_84.i.i.i210.i1136.sroa.0.0.copyload, !dbg !27413
  %226 = fadd <8 x float> %218, %225, !dbg !27418
  %227 = fmul <8 x float> %history.i315.i1241.sroa.19.sroa.0.015303, %_87.i.i.i207.i1133.sroa.0.0.copyload, !dbg !27423
  %228 = fadd <8 x float> %220, %227, !dbg !27428
  %229 = fmul <8 x float> %history.i315.i1241.sroa.19.sroa.0.015303, %_90.i.i.i204.i1130.sroa.0.0.copyload, !dbg !27433
  %230 = fadd <8 x float> %222, %229, !dbg !27438
  %231 = fmul <8 x float> %history.i315.i1241.sroa.22.sroa.0.015302, %_95.i.i.i200.i1126.sroa.0.0.copyload, !dbg !27443
  %232 = fadd <8 x float> %224, %231, !dbg !27448
  %233 = fmul <8 x float> %history.i315.i1241.sroa.22.sroa.0.015302, %_98.i.i.i197.i1123.sroa.0.0.copyload, !dbg !27453
  %234 = fadd <8 x float> %226, %233, !dbg !27458
  %235 = fmul <8 x float> %history.i315.i1241.sroa.22.sroa.0.015302, %_101.i.i.i194.i1120.sroa.0.0.copyload, !dbg !27463
  %236 = fadd <8 x float> %228, %235, !dbg !27468
  %237 = fmul <8 x float> %history.i315.i1241.sroa.22.sroa.0.015302, %_104.i.i.i191.i1117.sroa.0.0.copyload, !dbg !27473
  %238 = fadd <8 x float> %230, %237, !dbg !27478
  %239 = fmul <8 x float> %history.i315.i1241.sroa.25.sroa.0.015297, %_109.i.i.i187.i1113.sroa.0.0.copyload, !dbg !27483
  %240 = fadd <8 x float> %232, %239, !dbg !27488
  %241 = fmul <8 x float> %history.i315.i1241.sroa.25.sroa.0.015297, %_112.i.i.i184.i1110.sroa.0.0.copyload, !dbg !27493
  %242 = fadd <8 x float> %234, %241, !dbg !27498
  %243 = fmul <8 x float> %history.i315.i1241.sroa.25.sroa.0.015297, %_115.i.i.i181.i1107.sroa.0.0.copyload, !dbg !27503
  %244 = fadd <8 x float> %236, %243, !dbg !27508
  %245 = fmul <8 x float> %history.i315.i1241.sroa.25.sroa.0.015297, %_118.i.i.i178.i1104.sroa.0.0.copyload, !dbg !27513
  %246 = fadd <8 x float> %238, %245, !dbg !27518
  %247 = fmul <8 x float> %history.i315.i1241.sroa.29.sroa.0.015298, %_123.i.i.i174.i1100.sroa.0.0.copyload, !dbg !27523
  %248 = fadd <8 x float> %240, %247, !dbg !27528
  %249 = fmul <8 x float> %history.i315.i1241.sroa.29.sroa.0.015298, %_126.i.i.i171.i1097.sroa.0.0.copyload, !dbg !27533
  %250 = fadd <8 x float> %242, %249, !dbg !27538
  %251 = fmul <8 x float> %history.i315.i1241.sroa.29.sroa.0.015298, %_129.i.i.i168.i1094.sroa.0.0.copyload, !dbg !27543
  %252 = fadd <8 x float> %244, %251, !dbg !27548
  %253 = fmul <8 x float> %history.i315.i1241.sroa.29.sroa.0.015298, %_132.i.i.i165.i1091.sroa.0.0.copyload, !dbg !27553
  %254 = fadd <8 x float> %246, %253, !dbg !27558
  %255 = fmul <8 x float> %history.i315.i1241.sroa.32.sroa.0.015299, %_137.i.i.i161.i1087.sroa.0.0.copyload, !dbg !27563
  %256 = fadd <8 x float> %248, %255, !dbg !27568
  %257 = fmul <8 x float> %history.i315.i1241.sroa.32.sroa.0.015299, %_140.i.i.i158.i1084.sroa.0.0.copyload, !dbg !27573
  %258 = fadd <8 x float> %250, %257, !dbg !27578
  %259 = fmul <8 x float> %history.i315.i1241.sroa.32.sroa.0.015299, %_143.i.i.i155.i1081.sroa.0.0.copyload, !dbg !27583
  %260 = fadd <8 x float> %252, %259, !dbg !27588
  %261 = fmul <8 x float> %history.i315.i1241.sroa.32.sroa.0.015299, %_146.i.i.i152.i1078.sroa.0.0.copyload, !dbg !27593
  %262 = fadd <8 x float> %254, %261, !dbg !27598
  %263 = fmul <8 x float> %history.i315.i1241.sroa.35.sroa.0.015300, %_151.i.i.i148.i1074.sroa.0.0.copyload, !dbg !27603
  %264 = fadd <8 x float> %256, %263, !dbg !27608
  %265 = fmul <8 x float> %history.i315.i1241.sroa.35.sroa.0.015300, %_154.i.i.i145.i1071.sroa.0.0.copyload, !dbg !27613
  %266 = fadd <8 x float> %258, %265, !dbg !27618
  %267 = fmul <8 x float> %history.i315.i1241.sroa.35.sroa.0.015300, %_157.i.i.i142.i1068.sroa.0.0.copyload, !dbg !27623
  %268 = fadd <8 x float> %260, %267, !dbg !27628
  %269 = fmul <8 x float> %history.i315.i1241.sroa.35.sroa.0.015300, %_160.i.i.i139.i1065.sroa.0.0.copyload, !dbg !27633
  %270 = fadd <8 x float> %262, %269, !dbg !27638
  %271 = fmul <8 x float> %history.i315.i1241.sroa.38.sroa.0.015301, %_165.i.i.i135.i1061.sroa.0.0.copyload, !dbg !27643
  %272 = fadd <8 x float> %264, %271, !dbg !27648
  %273 = fmul <8 x float> %history.i315.i1241.sroa.38.sroa.0.015301, %_168.i.i.i132.i1058.sroa.0.0.copyload, !dbg !27653
  %274 = fadd <8 x float> %266, %273, !dbg !27658
  %275 = fmul <8 x float> %history.i315.i1241.sroa.38.sroa.0.015301, %_171.i.i.i129.i1055.sroa.0.0.copyload, !dbg !27663
  %276 = fadd <8 x float> %268, %275, !dbg !27668
  %277 = fmul <8 x float> %history.i315.i1241.sroa.38.sroa.0.015301, %_174.i.i.i126.i1052.sroa.0.0.copyload, !dbg !27673
  %278 = fadd <8 x float> %270, %277, !dbg !27678
  %279 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %272), !dbg !27683
  %280 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %182, <8 x float> %279), !dbg !27689
  %281 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %274), !dbg !27683
  %282 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %280, <8 x float> %281), !dbg !27689
  %283 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %276), !dbg !27683
  %284 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %282, <8 x float> %283), !dbg !27689
  %285 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %278), !dbg !27683
  %286 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %284, <8 x float> %285), !dbg !27689
  %287 = add nuw nsw i64 %iter.i311.i1237.sroa.16.015296, 1, !dbg !27694
  %data.i4.i = getelementptr inbounds nuw float, ptr %peaks_left.i1521, i64 %start1.i.i, !dbg !27695
  store <8 x float> %286, ptr %data.i4.i, align 4, !dbg !27698, !alias.scope !27703, !noalias !27707
  %exitcond.not = icmp eq i64 %287, %umax17991, !dbg !27156
  br i1 %exitcond.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit339.i1589, label %bb6.i318.i1568, !dbg !27156

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit339.i1589: ; preds = %bb6.i318.i1568, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit
  %history.i315.i1241.sroa.0.0.lcssa = phi <8 x float> [ %history.i315.i1241.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit ], [ %lanes.i5938.sroa.0.0.copyload, %bb6.i318.i1568 ], !dbg !27711
  %history.i315.i1241.sroa.25.sroa.0.0.lcssa = phi <8 x float> [ %history.i315.i1241.sroa.25.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit ], [ %history.i315.i1241.sroa.22.sroa.0.015302, %bb6.i318.i1568 ], !dbg !27711
  %history.i315.i1241.sroa.29.sroa.0.0.lcssa = phi <8 x float> [ %history.i315.i1241.sroa.29.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit ], [ %history.i315.i1241.sroa.25.sroa.0.015297, %bb6.i318.i1568 ], !dbg !27711
  %history.i315.i1241.sroa.32.sroa.0.0.lcssa = phi <8 x float> [ %history.i315.i1241.sroa.32.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit ], [ %history.i315.i1241.sroa.29.sroa.0.015298, %bb6.i318.i1568 ], !dbg !27711
  %history.i315.i1241.sroa.35.sroa.0.0.lcssa = phi <8 x float> [ %history.i315.i1241.sroa.35.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit ], [ %history.i315.i1241.sroa.32.sroa.0.015299, %bb6.i318.i1568 ], !dbg !27711
  %history.i315.i1241.sroa.38.sroa.0.0.lcssa = phi <8 x float> [ %history.i315.i1241.sroa.38.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit ], [ %history.i315.i1241.sroa.35.sroa.0.015300, %bb6.i318.i1568 ], !dbg !27711
  %history.i315.i1241.sroa.41.sroa.0.0.lcssa = phi <8 x float> [ %history.i315.i1241.sroa.41.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit ], [ %history.i315.i1241.sroa.38.sroa.0.015301, %bb6.i318.i1568 ], !dbg !27711
  %history.i315.i1241.sroa.22.sroa.0.0.lcssa = phi <8 x float> [ %history.i315.i1241.sroa.22.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit ], [ %history.i315.i1241.sroa.19.sroa.0.015303, %bb6.i318.i1568 ], !dbg !27711
  %history.i315.i1241.sroa.19.sroa.0.0.lcssa = phi <8 x float> [ %history.i315.i1241.sroa.19.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit ], [ %history.i315.i1241.sroa.16.sroa.0.015304, %bb6.i318.i1568 ], !dbg !27711
  %history.i315.i1241.sroa.16.sroa.0.0.lcssa = phi <8 x float> [ %history.i315.i1241.sroa.16.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit ], [ %history.i315.i1241.sroa.13.sroa.0.015305, %bb6.i318.i1568 ], !dbg !27711
  %history.i315.i1241.sroa.13.sroa.0.0.lcssa = phi <8 x float> [ %history.i315.i1241.sroa.13.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit ], [ %history.i315.i1241.sroa.10.sroa.0.015306, %bb6.i318.i1568 ], !dbg !27711
  %history.i315.i1241.sroa.10.sroa.0.0.lcssa = phi <8 x float> [ %history.i315.i1241.sroa.10.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit ], [ %history.i315.i1241.sroa.0.015295, %bb6.i318.i1568 ], !dbg !27711
  store <8 x float> %history.i315.i1241.sroa.0.0.lcssa, ptr %hot_left.i1529, align 32, !dbg !27712
  store <8 x float> %history.i315.i1241.sroa.10.sroa.0.0.lcssa, ptr %history.i315.i1241.sroa.10.0.hot_left.i1529.sroa_idx, align 32, !dbg !27712
  store <8 x float> %history.i315.i1241.sroa.13.sroa.0.0.lcssa, ptr %history.i315.i1241.sroa.13.0.hot_left.i1529.sroa_idx, align 32, !dbg !27712
  store <8 x float> %history.i315.i1241.sroa.16.sroa.0.0.lcssa, ptr %history.i315.i1241.sroa.16.0.hot_left.i1529.sroa_idx, align 32, !dbg !27712
  store <8 x float> %history.i315.i1241.sroa.19.sroa.0.0.lcssa, ptr %history.i315.i1241.sroa.19.0.hot_left.i1529.sroa_idx, align 32, !dbg !27712
  store <8 x float> %history.i315.i1241.sroa.22.sroa.0.0.lcssa, ptr %history.i315.i1241.sroa.22.0.hot_left.i1529.sroa_idx, align 32, !dbg !27712
  store <8 x float> %history.i315.i1241.sroa.25.sroa.0.0.lcssa, ptr %history.i315.i1241.sroa.25.0.hot_left.i1529.sroa_idx, align 32, !dbg !27712
  store <8 x float> %history.i315.i1241.sroa.29.sroa.0.0.lcssa, ptr %history.i315.i1241.sroa.29.0.hot_left.i1529.sroa_idx, align 32, !dbg !27712
  store <8 x float> %history.i315.i1241.sroa.32.sroa.0.0.lcssa, ptr %history.i315.i1241.sroa.32.0.hot_left.i1529.sroa_idx, align 32, !dbg !27712
  store <8 x float> %history.i315.i1241.sroa.35.sroa.0.0.lcssa, ptr %history.i315.i1241.sroa.35.0.hot_left.i1529.sroa_idx, align 32, !dbg !27712
  store <8 x float> %history.i315.i1241.sroa.38.sroa.0.0.lcssa, ptr %history.i315.i1241.sroa.38.0.hot_left.i1529.sroa_idx, align 32, !dbg !27712
  store <8 x float> %history.i315.i1241.sroa.41.sroa.0.0.lcssa, ptr %history.i315.i1241.sroa.41.0.hot_left.i1529.sroa_idx, align 32, !dbg !27712
  %_139.not.i1590 = icmp ugt i64 %_51.i1557, %right_io.1, !dbg !27713
  br i1 %_139.not.i1590, label %bb47.i1778, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7229, !dbg !27713, !prof !1406

bb47.i1778:                                       ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit339.i1589
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %active_base.i1555, i64 noundef %_51.i1557, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f6bbc99b95dcf27c100d71299ef7abde) #30, !dbg !27717, !noalias !27149
  unreachable, !dbg !27717

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7229: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit339.i1589
  %_146.i1592 = getelementptr inbounds nuw float, ptr %right_io.0, i64 %active_base.i1555, !dbg !27718
  %history.i.i1431.sroa.0.0.copyload = load <8 x float>, ptr %hot_right.i1528, align 32, !dbg !27722
  %history.i.i1431.sroa.10.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i1431.sroa.10.0.hot_right.i1528.sroa_idx, align 32, !dbg !27722
  %history.i.i1431.sroa.13.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i1431.sroa.13.0.hot_right.i1528.sroa_idx, align 32, !dbg !27722
  %history.i.i1431.sroa.16.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i1431.sroa.16.0.hot_right.i1528.sroa_idx, align 32, !dbg !27722
  %history.i.i1431.sroa.19.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i1431.sroa.19.0.hot_right.i1528.sroa_idx, align 32, !dbg !27722
  %history.i.i1431.sroa.22.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i1431.sroa.22.0.hot_right.i1528.sroa_idx, align 32, !dbg !27722
  %history.i.i1431.sroa.25.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i1431.sroa.25.0.hot_right.i1528.sroa_idx, align 32, !dbg !27722
  %history.i.i1431.sroa.29.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i1431.sroa.29.0.hot_right.i1528.sroa_idx, align 32, !dbg !27722
  %history.i.i1431.sroa.32.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i1431.sroa.32.0.hot_right.i1528.sroa_idx, align 32, !dbg !27722
  %history.i.i1431.sroa.35.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i1431.sroa.35.0.hot_right.i1528.sroa_idx, align 32, !dbg !27722
  %history.i.i1431.sroa.38.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i1431.sroa.38.0.hot_right.i1528.sroa_idx, align 32, !dbg !27722
  %history.i.i1431.sroa.41.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i1431.sroa.41.0.hot_right.i1528.sroa_idx, align 32, !dbg !27722
  br i1 %_2.i718715294.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1616, label %bb6.i.i1595.lr.ph, !dbg !27724

bb6.i.i1595.lr.ph:                                ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7229
  %_5.i4495 = load <8 x float>, ptr %self, align 32, !alias.scope !27727, !noalias !27730
  %_14.i.i.i.i1391.sroa.0.0.copyload = load <8 x float>, ptr %95, align 32, !noalias !27742
  %_17.i.i.i.i1388.sroa.0.0.copyload = load <8 x float>, ptr %96, align 32, !noalias !27742
  %_20.i.i.i.i1385.sroa.0.0.copyload = load <8 x float>, ptr %97, align 32, !noalias !27742
  %_25.i.i.i.i1381.sroa.0.0.copyload = load <8 x float>, ptr %row12.i.i.i322.i1572, align 32, !noalias !27742
  %_28.i.i.i.i1378.sroa.0.0.copyload = load <8 x float>, ptr %98, align 32, !noalias !27742
  %_31.i.i.i.i1375.sroa.0.0.copyload = load <8 x float>, ptr %99, align 32, !noalias !27742
  %_34.i.i.i.i1372.sroa.0.0.copyload = load <8 x float>, ptr %100, align 32, !noalias !27742
  %_39.i.i.i.i1368.sroa.0.0.copyload = load <8 x float>, ptr %row13.i.i.i323.i1573, align 32, !noalias !27742
  %_42.i.i.i.i1365.sroa.0.0.copyload = load <8 x float>, ptr %101, align 32, !noalias !27742
  %_45.i.i.i.i1362.sroa.0.0.copyload = load <8 x float>, ptr %102, align 32, !noalias !27742
  %_48.i.i.i.i1359.sroa.0.0.copyload = load <8 x float>, ptr %103, align 32, !noalias !27742
  %_53.i.i.i.i1355.sroa.0.0.copyload = load <8 x float>, ptr %row14.i.i.i324.i1574, align 32, !noalias !27742
  %_56.i.i.i.i1352.sroa.0.0.copyload = load <8 x float>, ptr %104, align 32, !noalias !27742
  %_59.i.i.i.i1349.sroa.0.0.copyload = load <8 x float>, ptr %105, align 32, !noalias !27742
  %_62.i.i.i.i1346.sroa.0.0.copyload = load <8 x float>, ptr %106, align 32, !noalias !27742
  %_67.i.i.i.i1342.sroa.0.0.copyload = load <8 x float>, ptr %row15.i.i.i325.i1575, align 32, !noalias !27742
  %_70.i.i.i.i1339.sroa.0.0.copyload = load <8 x float>, ptr %107, align 32, !noalias !27742
  %_73.i.i.i.i1336.sroa.0.0.copyload = load <8 x float>, ptr %108, align 32, !noalias !27742
  %_76.i.i.i.i1333.sroa.0.0.copyload = load <8 x float>, ptr %109, align 32, !noalias !27742
  %_81.i.i.i.i1329.sroa.0.0.copyload = load <8 x float>, ptr %row16.i.i.i326.i1576, align 32, !noalias !27742
  %_84.i.i.i.i1326.sroa.0.0.copyload = load <8 x float>, ptr %110, align 32, !noalias !27742
  %_87.i.i.i.i1323.sroa.0.0.copyload = load <8 x float>, ptr %111, align 32, !noalias !27742
  %_90.i.i.i.i1320.sroa.0.0.copyload = load <8 x float>, ptr %112, align 32, !noalias !27742
  %_95.i.i.i.i1316.sroa.0.0.copyload = load <8 x float>, ptr %row17.i.i.i327.i1577, align 32, !noalias !27742
  %_98.i.i.i.i1313.sroa.0.0.copyload = load <8 x float>, ptr %113, align 32, !noalias !27742
  %_101.i.i.i.i1310.sroa.0.0.copyload = load <8 x float>, ptr %114, align 32, !noalias !27742
  %_104.i.i.i.i1307.sroa.0.0.copyload = load <8 x float>, ptr %115, align 32, !noalias !27742
  %_109.i.i.i.i1303.sroa.0.0.copyload = load <8 x float>, ptr %row18.i.i.i328.i1578, align 32, !noalias !27742
  %_112.i.i.i.i1300.sroa.0.0.copyload = load <8 x float>, ptr %116, align 32, !noalias !27742
  %_115.i.i.i.i1297.sroa.0.0.copyload = load <8 x float>, ptr %117, align 32, !noalias !27742
  %_118.i.i.i.i1294.sroa.0.0.copyload = load <8 x float>, ptr %118, align 32, !noalias !27742
  %_123.i.i.i.i1290.sroa.0.0.copyload = load <8 x float>, ptr %row19.i.i.i329.i1579, align 32, !noalias !27742
  %_126.i.i.i.i1287.sroa.0.0.copyload = load <8 x float>, ptr %119, align 32, !noalias !27742
  %_129.i.i.i.i1284.sroa.0.0.copyload = load <8 x float>, ptr %120, align 32, !noalias !27742
  %_132.i.i.i.i1281.sroa.0.0.copyload = load <8 x float>, ptr %121, align 32, !noalias !27742
  %_137.i.i.i.i1277.sroa.0.0.copyload = load <8 x float>, ptr %row20.i.i.i330.i1580, align 32, !noalias !27742
  %_140.i.i.i.i1274.sroa.0.0.copyload = load <8 x float>, ptr %122, align 32, !noalias !27742
  %_143.i.i.i.i1271.sroa.0.0.copyload = load <8 x float>, ptr %123, align 32, !noalias !27742
  %_146.i.i.i.i1268.sroa.0.0.copyload = load <8 x float>, ptr %124, align 32, !noalias !27742
  %_151.i.i.i.i1264.sroa.0.0.copyload = load <8 x float>, ptr %row21.i.i.i331.i1581, align 32, !noalias !27742
  %_154.i.i.i.i1261.sroa.0.0.copyload = load <8 x float>, ptr %125, align 32, !noalias !27742
  %_157.i.i.i.i1258.sroa.0.0.copyload = load <8 x float>, ptr %126, align 32, !noalias !27742
  %_160.i.i.i.i1255.sroa.0.0.copyload = load <8 x float>, ptr %127, align 32, !noalias !27742
  %_165.i.i.i.i1251.sroa.0.0.copyload = load <8 x float>, ptr %row22.i.i.i332.i1582, align 32, !noalias !27742
  %_168.i.i.i.i1248.sroa.0.0.copyload = load <8 x float>, ptr %128, align 32, !noalias !27742
  %_171.i.i.i.i1245.sroa.0.0.copyload = load <8 x float>, ptr %129, align 32, !noalias !27742
  %_174.i.i.i.i1242.sroa.0.0.copyload = load <8 x float>, ptr %130, align 32, !noalias !27742
  br label %bb6.i.i1595, !dbg !27724

bb6.i.i1595:                                      ; preds = %bb6.i.i1595.lr.ph, %bb6.i.i1595
  %history.i.i1431.sroa.10.sroa.0.015332 = phi <8 x float> [ %history.i.i1431.sroa.10.sroa.0.0.copyload, %bb6.i.i1595.lr.ph ], [ %history.i.i1431.sroa.0.015321, %bb6.i.i1595 ]
  %history.i.i1431.sroa.13.sroa.0.015331 = phi <8 x float> [ %history.i.i1431.sroa.13.sroa.0.0.copyload, %bb6.i.i1595.lr.ph ], [ %history.i.i1431.sroa.10.sroa.0.015332, %bb6.i.i1595 ]
  %history.i.i1431.sroa.16.sroa.0.015330 = phi <8 x float> [ %history.i.i1431.sroa.16.sroa.0.0.copyload, %bb6.i.i1595.lr.ph ], [ %history.i.i1431.sroa.13.sroa.0.015331, %bb6.i.i1595 ]
  %history.i.i1431.sroa.19.sroa.0.015329 = phi <8 x float> [ %history.i.i1431.sroa.19.sroa.0.0.copyload, %bb6.i.i1595.lr.ph ], [ %history.i.i1431.sroa.16.sroa.0.015330, %bb6.i.i1595 ]
  %history.i.i1431.sroa.22.sroa.0.015328 = phi <8 x float> [ %history.i.i1431.sroa.22.sroa.0.0.copyload, %bb6.i.i1595.lr.ph ], [ %history.i.i1431.sroa.19.sroa.0.015329, %bb6.i.i1595 ]
  %history.i.i1431.sroa.38.sroa.0.015327 = phi <8 x float> [ %history.i.i1431.sroa.38.sroa.0.0.copyload, %bb6.i.i1595.lr.ph ], [ %history.i.i1431.sroa.35.sroa.0.015326, %bb6.i.i1595 ]
  %history.i.i1431.sroa.35.sroa.0.015326 = phi <8 x float> [ %history.i.i1431.sroa.35.sroa.0.0.copyload, %bb6.i.i1595.lr.ph ], [ %history.i.i1431.sroa.32.sroa.0.015325, %bb6.i.i1595 ]
  %history.i.i1431.sroa.32.sroa.0.015325 = phi <8 x float> [ %history.i.i1431.sroa.32.sroa.0.0.copyload, %bb6.i.i1595.lr.ph ], [ %history.i.i1431.sroa.29.sroa.0.015324, %bb6.i.i1595 ]
  %history.i.i1431.sroa.29.sroa.0.015324 = phi <8 x float> [ %history.i.i1431.sroa.29.sroa.0.0.copyload, %bb6.i.i1595.lr.ph ], [ %history.i.i1431.sroa.25.sroa.0.015323, %bb6.i.i1595 ]
  %history.i.i1431.sroa.25.sroa.0.015323 = phi <8 x float> [ %history.i.i1431.sroa.25.sroa.0.0.copyload, %bb6.i.i1595.lr.ph ], [ %history.i.i1431.sroa.22.sroa.0.015328, %bb6.i.i1595 ]
  %iter.i124.i1427.sroa.16.015322 = phi i64 [ 0, %bb6.i.i1595.lr.ph ], [ %393, %bb6.i.i1595 ]
  %history.i.i1431.sroa.0.015321 = phi <8 x float> [ %history.i.i1431.sroa.0.0.copyload, %bb6.i.i1595.lr.ph ], [ %lanes.i5929.sroa.0.0.copyload, %bb6.i.i1595 ]
  %start1.i.i7238 = shl i64 %iter.i124.i1427.sroa.16.015322, 3, !dbg !27745
  %data.i.i7239 = getelementptr inbounds nuw float, ptr %_146.i1592, i64 %start1.i.i7238, !dbg !27747
  %lanes.i5929.sroa.0.0.copyload = load <8 x float>, ptr %data.i.i7239, align 4, !dbg !27749, !alias.scope !27754, !noalias !27758
  %288 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i.i1431.sroa.22.sroa.0.015328), !dbg !27763
  %289 = fmul <8 x float> %lanes.i5929.sroa.0.0.copyload, %_5.i4495, !dbg !27770
  %290 = fadd <8 x float> %289, zeroinitializer, !dbg !27776
  %291 = fmul <8 x float> %lanes.i5929.sroa.0.0.copyload, %_14.i.i.i.i1391.sroa.0.0.copyload, !dbg !27781
  %292 = fadd <8 x float> %291, zeroinitializer, !dbg !27786
  %293 = fmul <8 x float> %lanes.i5929.sroa.0.0.copyload, %_17.i.i.i.i1388.sroa.0.0.copyload, !dbg !27791
  %294 = fadd <8 x float> %293, zeroinitializer, !dbg !27796
  %295 = fmul <8 x float> %lanes.i5929.sroa.0.0.copyload, %_20.i.i.i.i1385.sroa.0.0.copyload, !dbg !27801
  %296 = fadd <8 x float> %295, zeroinitializer, !dbg !27806
  %297 = fmul <8 x float> %history.i.i1431.sroa.0.015321, %_25.i.i.i.i1381.sroa.0.0.copyload, !dbg !27811
  %298 = fadd <8 x float> %290, %297, !dbg !27816
  %299 = fmul <8 x float> %history.i.i1431.sroa.0.015321, %_28.i.i.i.i1378.sroa.0.0.copyload, !dbg !27821
  %300 = fadd <8 x float> %292, %299, !dbg !27826
  %301 = fmul <8 x float> %history.i.i1431.sroa.0.015321, %_31.i.i.i.i1375.sroa.0.0.copyload, !dbg !27831
  %302 = fadd <8 x float> %294, %301, !dbg !27836
  %303 = fmul <8 x float> %history.i.i1431.sroa.0.015321, %_34.i.i.i.i1372.sroa.0.0.copyload, !dbg !27841
  %304 = fadd <8 x float> %296, %303, !dbg !27846
  %305 = fmul <8 x float> %history.i.i1431.sroa.10.sroa.0.015332, %_39.i.i.i.i1368.sroa.0.0.copyload, !dbg !27851
  %306 = fadd <8 x float> %298, %305, !dbg !27856
  %307 = fmul <8 x float> %history.i.i1431.sroa.10.sroa.0.015332, %_42.i.i.i.i1365.sroa.0.0.copyload, !dbg !27861
  %308 = fadd <8 x float> %300, %307, !dbg !27866
  %309 = fmul <8 x float> %history.i.i1431.sroa.10.sroa.0.015332, %_45.i.i.i.i1362.sroa.0.0.copyload, !dbg !27871
  %310 = fadd <8 x float> %302, %309, !dbg !27876
  %311 = fmul <8 x float> %history.i.i1431.sroa.10.sroa.0.015332, %_48.i.i.i.i1359.sroa.0.0.copyload, !dbg !27881
  %312 = fadd <8 x float> %304, %311, !dbg !27886
  %313 = fmul <8 x float> %history.i.i1431.sroa.13.sroa.0.015331, %_53.i.i.i.i1355.sroa.0.0.copyload, !dbg !27891
  %314 = fadd <8 x float> %306, %313, !dbg !27896
  %315 = fmul <8 x float> %history.i.i1431.sroa.13.sroa.0.015331, %_56.i.i.i.i1352.sroa.0.0.copyload, !dbg !27901
  %316 = fadd <8 x float> %308, %315, !dbg !27906
  %317 = fmul <8 x float> %history.i.i1431.sroa.13.sroa.0.015331, %_59.i.i.i.i1349.sroa.0.0.copyload, !dbg !27911
  %318 = fadd <8 x float> %310, %317, !dbg !27916
  %319 = fmul <8 x float> %history.i.i1431.sroa.13.sroa.0.015331, %_62.i.i.i.i1346.sroa.0.0.copyload, !dbg !27921
  %320 = fadd <8 x float> %312, %319, !dbg !27926
  %321 = fmul <8 x float> %history.i.i1431.sroa.16.sroa.0.015330, %_67.i.i.i.i1342.sroa.0.0.copyload, !dbg !27931
  %322 = fadd <8 x float> %314, %321, !dbg !27936
  %323 = fmul <8 x float> %history.i.i1431.sroa.16.sroa.0.015330, %_70.i.i.i.i1339.sroa.0.0.copyload, !dbg !27941
  %324 = fadd <8 x float> %316, %323, !dbg !27946
  %325 = fmul <8 x float> %history.i.i1431.sroa.16.sroa.0.015330, %_73.i.i.i.i1336.sroa.0.0.copyload, !dbg !27951
  %326 = fadd <8 x float> %318, %325, !dbg !27956
  %327 = fmul <8 x float> %history.i.i1431.sroa.16.sroa.0.015330, %_76.i.i.i.i1333.sroa.0.0.copyload, !dbg !27961
  %328 = fadd <8 x float> %320, %327, !dbg !27966
  %329 = fmul <8 x float> %history.i.i1431.sroa.19.sroa.0.015329, %_81.i.i.i.i1329.sroa.0.0.copyload, !dbg !27971
  %330 = fadd <8 x float> %322, %329, !dbg !27976
  %331 = fmul <8 x float> %history.i.i1431.sroa.19.sroa.0.015329, %_84.i.i.i.i1326.sroa.0.0.copyload, !dbg !27981
  %332 = fadd <8 x float> %324, %331, !dbg !27986
  %333 = fmul <8 x float> %history.i.i1431.sroa.19.sroa.0.015329, %_87.i.i.i.i1323.sroa.0.0.copyload, !dbg !27991
  %334 = fadd <8 x float> %326, %333, !dbg !27996
  %335 = fmul <8 x float> %history.i.i1431.sroa.19.sroa.0.015329, %_90.i.i.i.i1320.sroa.0.0.copyload, !dbg !28001
  %336 = fadd <8 x float> %328, %335, !dbg !28006
  %337 = fmul <8 x float> %history.i.i1431.sroa.22.sroa.0.015328, %_95.i.i.i.i1316.sroa.0.0.copyload, !dbg !28011
  %338 = fadd <8 x float> %330, %337, !dbg !28016
  %339 = fmul <8 x float> %history.i.i1431.sroa.22.sroa.0.015328, %_98.i.i.i.i1313.sroa.0.0.copyload, !dbg !28021
  %340 = fadd <8 x float> %332, %339, !dbg !28026
  %341 = fmul <8 x float> %history.i.i1431.sroa.22.sroa.0.015328, %_101.i.i.i.i1310.sroa.0.0.copyload, !dbg !28031
  %342 = fadd <8 x float> %334, %341, !dbg !28036
  %343 = fmul <8 x float> %history.i.i1431.sroa.22.sroa.0.015328, %_104.i.i.i.i1307.sroa.0.0.copyload, !dbg !28041
  %344 = fadd <8 x float> %336, %343, !dbg !28046
  %345 = fmul <8 x float> %history.i.i1431.sroa.25.sroa.0.015323, %_109.i.i.i.i1303.sroa.0.0.copyload, !dbg !28051
  %346 = fadd <8 x float> %338, %345, !dbg !28056
  %347 = fmul <8 x float> %history.i.i1431.sroa.25.sroa.0.015323, %_112.i.i.i.i1300.sroa.0.0.copyload, !dbg !28061
  %348 = fadd <8 x float> %340, %347, !dbg !28066
  %349 = fmul <8 x float> %history.i.i1431.sroa.25.sroa.0.015323, %_115.i.i.i.i1297.sroa.0.0.copyload, !dbg !28071
  %350 = fadd <8 x float> %342, %349, !dbg !28076
  %351 = fmul <8 x float> %history.i.i1431.sroa.25.sroa.0.015323, %_118.i.i.i.i1294.sroa.0.0.copyload, !dbg !28081
  %352 = fadd <8 x float> %344, %351, !dbg !28086
  %353 = fmul <8 x float> %history.i.i1431.sroa.29.sroa.0.015324, %_123.i.i.i.i1290.sroa.0.0.copyload, !dbg !28091
  %354 = fadd <8 x float> %346, %353, !dbg !28096
  %355 = fmul <8 x float> %history.i.i1431.sroa.29.sroa.0.015324, %_126.i.i.i.i1287.sroa.0.0.copyload, !dbg !28101
  %356 = fadd <8 x float> %348, %355, !dbg !28106
  %357 = fmul <8 x float> %history.i.i1431.sroa.29.sroa.0.015324, %_129.i.i.i.i1284.sroa.0.0.copyload, !dbg !28111
  %358 = fadd <8 x float> %350, %357, !dbg !28116
  %359 = fmul <8 x float> %history.i.i1431.sroa.29.sroa.0.015324, %_132.i.i.i.i1281.sroa.0.0.copyload, !dbg !28121
  %360 = fadd <8 x float> %352, %359, !dbg !28126
  %361 = fmul <8 x float> %history.i.i1431.sroa.32.sroa.0.015325, %_137.i.i.i.i1277.sroa.0.0.copyload, !dbg !28131
  %362 = fadd <8 x float> %354, %361, !dbg !28136
  %363 = fmul <8 x float> %history.i.i1431.sroa.32.sroa.0.015325, %_140.i.i.i.i1274.sroa.0.0.copyload, !dbg !28141
  %364 = fadd <8 x float> %356, %363, !dbg !28146
  %365 = fmul <8 x float> %history.i.i1431.sroa.32.sroa.0.015325, %_143.i.i.i.i1271.sroa.0.0.copyload, !dbg !28151
  %366 = fadd <8 x float> %358, %365, !dbg !28156
  %367 = fmul <8 x float> %history.i.i1431.sroa.32.sroa.0.015325, %_146.i.i.i.i1268.sroa.0.0.copyload, !dbg !28161
  %368 = fadd <8 x float> %360, %367, !dbg !28166
  %369 = fmul <8 x float> %history.i.i1431.sroa.35.sroa.0.015326, %_151.i.i.i.i1264.sroa.0.0.copyload, !dbg !28171
  %370 = fadd <8 x float> %362, %369, !dbg !28176
  %371 = fmul <8 x float> %history.i.i1431.sroa.35.sroa.0.015326, %_154.i.i.i.i1261.sroa.0.0.copyload, !dbg !28181
  %372 = fadd <8 x float> %364, %371, !dbg !28186
  %373 = fmul <8 x float> %history.i.i1431.sroa.35.sroa.0.015326, %_157.i.i.i.i1258.sroa.0.0.copyload, !dbg !28191
  %374 = fadd <8 x float> %366, %373, !dbg !28196
  %375 = fmul <8 x float> %history.i.i1431.sroa.35.sroa.0.015326, %_160.i.i.i.i1255.sroa.0.0.copyload, !dbg !28201
  %376 = fadd <8 x float> %368, %375, !dbg !28206
  %377 = fmul <8 x float> %history.i.i1431.sroa.38.sroa.0.015327, %_165.i.i.i.i1251.sroa.0.0.copyload, !dbg !28211
  %378 = fadd <8 x float> %370, %377, !dbg !28216
  %379 = fmul <8 x float> %history.i.i1431.sroa.38.sroa.0.015327, %_168.i.i.i.i1248.sroa.0.0.copyload, !dbg !28221
  %380 = fadd <8 x float> %372, %379, !dbg !28226
  %381 = fmul <8 x float> %history.i.i1431.sroa.38.sroa.0.015327, %_171.i.i.i.i1245.sroa.0.0.copyload, !dbg !28231
  %382 = fadd <8 x float> %374, %381, !dbg !28236
  %383 = fmul <8 x float> %history.i.i1431.sroa.38.sroa.0.015327, %_174.i.i.i.i1242.sroa.0.0.copyload, !dbg !28241
  %384 = fadd <8 x float> %376, %383, !dbg !28246
  %385 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %378), !dbg !28251
  %386 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %288, <8 x float> %385), !dbg !28257
  %387 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %380), !dbg !28251
  %388 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %386, <8 x float> %387), !dbg !28257
  %389 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %382), !dbg !28251
  %390 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %388, <8 x float> %389), !dbg !28257
  %391 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %384), !dbg !28251
  %392 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %390, <8 x float> %391), !dbg !28257
  %393 = add nuw nsw i64 %iter.i124.i1427.sroa.16.015322, 1, !dbg !28262
  %data.i4.i7243 = getelementptr inbounds nuw float, ptr %peaks_right.i1520, i64 %start1.i.i7238, !dbg !28263
  store <8 x float> %392, ptr %data.i4.i7243, align 4, !dbg !28266, !alias.scope !28271, !noalias !28275
  %exitcond17963.not = icmp eq i64 %393, %umax17991, !dbg !27724
  br i1 %exitcond17963.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1616, label %bb6.i.i1595, !dbg !27724

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1616: ; preds = %bb6.i.i1595, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7229
  %history.i.i1431.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i1431.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7229 ], [ %lanes.i5929.sroa.0.0.copyload, %bb6.i.i1595 ], !dbg !28279
  %history.i.i1431.sroa.25.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i1431.sroa.25.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7229 ], [ %history.i.i1431.sroa.22.sroa.0.015328, %bb6.i.i1595 ], !dbg !28279
  %history.i.i1431.sroa.29.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i1431.sroa.29.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7229 ], [ %history.i.i1431.sroa.25.sroa.0.015323, %bb6.i.i1595 ], !dbg !28279
  %history.i.i1431.sroa.32.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i1431.sroa.32.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7229 ], [ %history.i.i1431.sroa.29.sroa.0.015324, %bb6.i.i1595 ], !dbg !28279
  %history.i.i1431.sroa.35.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i1431.sroa.35.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7229 ], [ %history.i.i1431.sroa.32.sroa.0.015325, %bb6.i.i1595 ], !dbg !28279
  %history.i.i1431.sroa.38.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i1431.sroa.38.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7229 ], [ %history.i.i1431.sroa.35.sroa.0.015326, %bb6.i.i1595 ], !dbg !28279
  %history.i.i1431.sroa.41.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i1431.sroa.41.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7229 ], [ %history.i.i1431.sroa.38.sroa.0.015327, %bb6.i.i1595 ], !dbg !28279
  %history.i.i1431.sroa.22.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i1431.sroa.22.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7229 ], [ %history.i.i1431.sroa.19.sroa.0.015329, %bb6.i.i1595 ], !dbg !28279
  %history.i.i1431.sroa.19.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i1431.sroa.19.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7229 ], [ %history.i.i1431.sroa.16.sroa.0.015330, %bb6.i.i1595 ], !dbg !28279
  %history.i.i1431.sroa.16.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i1431.sroa.16.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7229 ], [ %history.i.i1431.sroa.13.sroa.0.015331, %bb6.i.i1595 ], !dbg !28279
  %history.i.i1431.sroa.13.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i1431.sroa.13.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7229 ], [ %history.i.i1431.sroa.10.sroa.0.015332, %bb6.i.i1595 ], !dbg !28279
  %history.i.i1431.sroa.10.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i1431.sroa.10.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7229 ], [ %history.i.i1431.sroa.0.015321, %bb6.i.i1595 ], !dbg !28279
  store <8 x float> %history.i.i1431.sroa.0.0.lcssa, ptr %hot_right.i1528, align 32, !dbg !28280
  store <8 x float> %history.i.i1431.sroa.10.sroa.0.0.lcssa, ptr %history.i.i1431.sroa.10.0.hot_right.i1528.sroa_idx, align 32, !dbg !28280
  store <8 x float> %history.i.i1431.sroa.13.sroa.0.0.lcssa, ptr %history.i.i1431.sroa.13.0.hot_right.i1528.sroa_idx, align 32, !dbg !28280
  store <8 x float> %history.i.i1431.sroa.16.sroa.0.0.lcssa, ptr %history.i.i1431.sroa.16.0.hot_right.i1528.sroa_idx, align 32, !dbg !28280
  store <8 x float> %history.i.i1431.sroa.19.sroa.0.0.lcssa, ptr %history.i.i1431.sroa.19.0.hot_right.i1528.sroa_idx, align 32, !dbg !28280
  store <8 x float> %history.i.i1431.sroa.22.sroa.0.0.lcssa, ptr %history.i.i1431.sroa.22.0.hot_right.i1528.sroa_idx, align 32, !dbg !28280
  store <8 x float> %history.i.i1431.sroa.25.sroa.0.0.lcssa, ptr %history.i.i1431.sroa.25.0.hot_right.i1528.sroa_idx, align 32, !dbg !28280
  store <8 x float> %history.i.i1431.sroa.29.sroa.0.0.lcssa, ptr %history.i.i1431.sroa.29.0.hot_right.i1528.sroa_idx, align 32, !dbg !28280
  store <8 x float> %history.i.i1431.sroa.32.sroa.0.0.lcssa, ptr %history.i.i1431.sroa.32.0.hot_right.i1528.sroa_idx, align 32, !dbg !28280
  store <8 x float> %history.i.i1431.sroa.35.sroa.0.0.lcssa, ptr %history.i.i1431.sroa.35.0.hot_right.i1528.sroa_idx, align 32, !dbg !28280
  store <8 x float> %history.i.i1431.sroa.38.sroa.0.0.lcssa, ptr %history.i.i1431.sroa.38.0.hot_right.i1528.sroa_idx, align 32, !dbg !28280
  store <8 x float> %history.i.i1431.sroa.41.sroa.0.0.lcssa, ptr %history.i.i1431.sroa.41.0.hot_right.i1528.sroa_idx, align 32, !dbg !28280
  br i1 %_2.i718715294.not, label %bb13.i1545.loopexit, label %bb48.i1622.preheader, !dbg !28281

bb48.i1622.preheader:                             ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1616
  %_5.i2594.sroa.0.0.copyload.pre = load <8 x float>, ptr %131, align 32, !dbg !28287
  %_11.i2588.sroa.0.0.copyload.pre = load <8 x float>, ptr %_68.i1625, align 32, !dbg !28288
  %_12.i2587.sroa.0.0.copyload.pre = load <8 x float>, ptr %132, align 32, !dbg !28289
  %_13.i2586.sroa.0.0.copyload.pre = load <8 x float>, ptr %133, align 32, !dbg !28290
  %_5.i2580.sroa.0.0.copyload.pre = load <8 x float>, ptr %134, align 32, !dbg !28291
  %_11.i2574.sroa.0.0.copyload.pre = load <8 x float>, ptr %_69.i1626, align 32, !dbg !28292
  %_12.i2573.sroa.0.0.copyload.pre = load <8 x float>, ptr %135, align 32, !dbg !28293
  %_13.i2572.sroa.0.0.copyload.pre = load <8 x float>, ptr %136, align 32, !dbg !28294
  %_5.i2566.sroa.0.0.copyload.pre = load <8 x float>, ptr %137, align 32, !dbg !28295
  %_11.i2560.sroa.0.0.copyload.pre = load <8 x float>, ptr %_73.i1627, align 32, !dbg !28296
  %_12.i2559.sroa.0.0.copyload.pre = load <8 x float>, ptr %138, align 32, !dbg !28297
  %_13.i2558.sroa.0.0.copyload.pre = load <8 x float>, ptr %139, align 32, !dbg !28298
  %_5.i2554.sroa.0.0.copyload.pre = load <8 x float>, ptr %140, align 32, !dbg !28299
  %_74.i1628.promoted = load <8 x float>, ptr %_74.i1628, align 32
  %.promoted20540 = load <8 x float>, ptr %141, align 32
  %.promoted20577 = load <8 x float>, ptr %155, align 32
  %.promoted20579 = load <8 x float>, ptr %173, align 32
  br label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5927

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5927: ; preds = %bb48.i1622.preheader, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit
  %_58.i.i1483.sroa.0.0.copyload20580 = phi <8 x float> [ %.promoted20579, %bb48.i1622.preheader ], [ %548, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %_58.i46.i1450.sroa.0.0.copyload20578 = phi <8 x float> [ %.promoted20577, %bb48.i1622.preheader ], [ %461, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %_12.i2551.sroa.0.0.copyload20541 = phi <8 x float> [ %.promoted20540, %bb48.i1622.preheader ], [ %426, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !28300
  %_11.i2552.sroa.0.0.copyload20504 = phi <8 x float> [ %_74.i1628.promoted, %bb48.i1622.preheader ], [ %425, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !28300
  %_5.i2554.sroa.0.0.copyload = phi <8 x float> [ %_5.i2554.sroa.0.0.copyload.pre, %bb48.i1622.preheader ], [ %420, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !28299
  %_12.i2559.sroa.0.0.copyload = phi <8 x float> [ %_12.i2559.sroa.0.0.copyload.pre, %bb48.i1622.preheader ], [ %417, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !28297
  %_11.i2560.sroa.0.0.copyload = phi <8 x float> [ %_11.i2560.sroa.0.0.copyload.pre, %bb48.i1622.preheader ], [ %416, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !28296
  %_5.i2566.sroa.0.0.copyload = phi <8 x float> [ %_5.i2566.sroa.0.0.copyload.pre, %bb48.i1622.preheader ], [ %411, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !28295
  %_12.i2573.sroa.0.0.copyload = phi <8 x float> [ %_12.i2573.sroa.0.0.copyload.pre, %bb48.i1622.preheader ], [ %409, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !28293
  %_11.i2574.sroa.0.0.copyload = phi <8 x float> [ %_11.i2574.sroa.0.0.copyload.pre, %bb48.i1622.preheader ], [ %408, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !28292
  %_5.i2580.sroa.0.0.copyload = phi <8 x float> [ %_5.i2580.sroa.0.0.copyload.pre, %bb48.i1622.preheader ], [ %403, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !28291
  %_12.i2587.sroa.0.0.copyload = phi <8 x float> [ %_12.i2587.sroa.0.0.copyload.pre, %bb48.i1622.preheader ], [ %401, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !28289
  %_11.i2588.sroa.0.0.copyload = phi <8 x float> [ %_11.i2588.sroa.0.0.copyload.pre, %bb48.i1622.preheader ], [ %400, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !28288
  %_5.i2594.sroa.0.0.copyload = phi <8 x float> [ %_5.i2594.sroa.0.0.copyload.pre, %bb48.i1622.preheader ], [ %395, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ], !dbg !28287
  %main_cursor.sroa.0.1.i162015370 = phi i64 [ %main_cursor.sroa.0.0.i154915377, %bb48.i1622.preheader ], [ %spec.store.select.i1762, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %ring_cursor.sroa.0.1.i161915369 = phi i64 [ %ring_cursor.sroa.0.0.i154815376, %bb48.i1622.preheader ], [ %spec.store.select13.i1764, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %iter3.sroa.0.0.i161815368 = phi i64 [ 0, %bb48.i1622.preheader ], [ %418, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit ]
  %394 = fadd <8 x float> %_5.i2594.sroa.0.0.copyload, splat (float -1.000000e+00), !dbg !28300
  %395 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %394, <8 x float> zeroinitializer), !dbg !28305
  %396 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %395, <8 x float> zeroinitializer, i8 30), !dbg !28310
  %397 = fadd <8 x float> %_11.i2588.sroa.0.0.copyload, %_12.i2587.sroa.0.0.copyload, !dbg !28316
  %398 = bitcast <8 x float> %396 to <8 x i32>, !dbg !28321
  %399 = icmp slt <8 x i32> %398, zeroinitializer, !dbg !28325
  %400 = select <8 x i1> %399, <8 x float> %397, <8 x float> %_13.i2586.sroa.0.0.copyload.pre, !dbg !28325
  %401 = select <8 x i1> %399, <8 x float> %_12.i2587.sroa.0.0.copyload, <8 x float> zeroinitializer, !dbg !28327
  %402 = fadd <8 x float> %_5.i2580.sroa.0.0.copyload, splat (float -1.000000e+00), !dbg !28332
  %403 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %402, <8 x float> zeroinitializer), !dbg !28337
  %404 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %403, <8 x float> zeroinitializer, i8 30), !dbg !28342
  %405 = fadd <8 x float> %_11.i2574.sroa.0.0.copyload, %_12.i2573.sroa.0.0.copyload, !dbg !28348
  %406 = bitcast <8 x float> %404 to <8 x i32>, !dbg !28353
  %407 = icmp slt <8 x i32> %406, zeroinitializer, !dbg !28357
  %408 = select <8 x i1> %407, <8 x float> %405, <8 x float> %_13.i2572.sroa.0.0.copyload.pre, !dbg !28357
  %409 = select <8 x i1> %407, <8 x float> %_12.i2573.sroa.0.0.copyload, <8 x float> zeroinitializer, !dbg !28359
  %410 = fadd <8 x float> %_5.i2566.sroa.0.0.copyload, splat (float -1.000000e+00), !dbg !28364
  %411 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %410, <8 x float> zeroinitializer), !dbg !28369
  %412 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %411, <8 x float> zeroinitializer, i8 30), !dbg !28374
  %413 = fadd <8 x float> %_11.i2560.sroa.0.0.copyload, %_12.i2559.sroa.0.0.copyload, !dbg !28380
  %414 = bitcast <8 x float> %412 to <8 x i32>, !dbg !28385
  %415 = icmp slt <8 x i32> %414, zeroinitializer, !dbg !28389
  %416 = select <8 x i1> %415, <8 x float> %413, <8 x float> %_13.i2558.sroa.0.0.copyload.pre, !dbg !28389
  %417 = select <8 x i1> %415, <8 x float> %_12.i2559.sroa.0.0.copyload, <8 x float> zeroinitializer, !dbg !28391
  %418 = add nuw nsw i64 %iter3.sroa.0.0.i161815368, 1, !dbg !28396
  %_64.i1623 = add nuw nsw i64 %iter3.sroa.0.0.i161815368, %iter.sroa.0.0.i154615374, !dbg !28402
  %base.i1624 = shl i64 %_64.i1623, 3, !dbg !28402
  %419 = fadd <8 x float> %_5.i2554.sroa.0.0.copyload, splat (float -1.000000e+00), !dbg !28403
  %420 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %419, <8 x float> zeroinitializer), !dbg !28408
  %421 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %420, <8 x float> zeroinitializer, i8 30), !dbg !28413
  %422 = fadd <8 x float> %_11.i2552.sroa.0.0.copyload20504, %_12.i2551.sroa.0.0.copyload20541, !dbg !28419
  %_13.i2550.sroa.0.0.copyload = load <8 x float>, ptr %142, align 32, !dbg !28424
  %423 = bitcast <8 x float> %421 to <8 x i32>, !dbg !28425
  %424 = icmp slt <8 x i32> %423, zeroinitializer, !dbg !28429
  %425 = select <8 x i1> %424, <8 x float> %422, <8 x float> %_13.i2550.sroa.0.0.copyload, !dbg !28429
  %426 = select <8 x i1> %424, <8 x float> %_12.i2551.sroa.0.0.copyload20541, <8 x float> zeroinitializer, !dbg !28431
  %_78.i1629 = shl i64 %iter3.sroa.0.0.i161815368, 3, !dbg !28436
  %_162.i1633 = getelementptr inbounds nuw float, ptr %peaks_left.i1521, i64 %_78.i1629, !dbg !28438
  %lanes.i5920.sroa.0.0.copyload = load <8 x float>, ptr %_162.i1633, align 4, !dbg !28449, !alias.scope !28454, !noalias !28458
  %_167.i1634 = getelementptr inbounds nuw float, ptr %peaks_right.i1520, i64 %_78.i1629, !dbg !28462
  %lanes.i5911.sroa.0.0.copyload = load <8 x float>, ptr %_167.i1634, align 4, !dbg !28473, !alias.scope !28478, !noalias !28482
  %427 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i5911.sroa.0.0.copyload, <8 x float> %lanes.i5920.sroa.0.0.copyload), !dbg !28486
  %428 = select <8 x i1> %144, <8 x float> %427, <8 x float> %lanes.i5920.sroa.0.0.copyload, !dbg !28492
  %429 = select <8 x i1> %144, <8 x float> %427, <8 x float> %lanes.i5911.sroa.0.0.copyload, !dbg !28498
  %_168.i1635 = icmp samesign ugt i64 %base.i1624, %left_io.1, !dbg !28504
  br i1 %_168.i1635, label %bb52.i1776, label %bb53.i1636, !dbg !28504, !prof !1406

bb53.i1636:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5927
  %_170.i1637 = sub nuw nsw i64 %left_io.1, %base.i1624, !dbg !28509
  %_174.i1638 = getelementptr inbounds nuw float, ptr %left_io.0, i64 %base.i1624, !dbg !28510
  %_8.i5905 = icmp samesign ugt i64 %_170.i1637, 7, !dbg !28515
  br i1 %_8.i5905, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5909, label %bb2.i5906, !dbg !28515, !prof !1421

bb2.i5906:                                        ; preds = %bb53.i1636
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_170.i1637, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !28520, !noalias !28521
  unreachable, !dbg !28520

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5909: ; preds = %bb53.i1636
  %_91.i1639 = load i64, ptr %145, align 8, !dbg !28525, !alias.scope !27042, !noalias !28526, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !28527), !dbg !28530
  %width.i61.i1640 = load i64, ptr %146, align 8, !dbg !28531, !alias.scope !28533, !noalias !28534, !noundef !12
  %430 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %428, <8 x float> %400, i8 30), !dbg !28543
  %431 = fdiv <8 x float> %400, %428, !dbg !28549
  %432 = bitcast <8 x float> %430 to <8 x i32>, !dbg !28554
  %433 = icmp slt <8 x i32> %432, zeroinitializer, !dbg !28558
  %434 = select <8 x i1> %433, <8 x float> %431, <8 x float> splat (float 1.000000e+00), !dbg !28558
  %_144.1.i62.i1641 = load i64, ptr %147, align 8, !dbg !28560, !alias.scope !28533, !noalias !28534, !noundef !12
  %_22.i63.i1642 = mul i64 %width.i61.i1640, %ring_cursor.sroa.0.1.i161915369, !dbg !28561
  %_92.i64.i1643 = icmp ugt i64 %_22.i63.i1642, %_144.1.i62.i1641, !dbg !28562
  br i1 %_92.i64.i1643, label %bb37.i122.i1775, label %bb38.i65.i1644, !dbg !28562, !prof !1406

bb38.i65.i1644:                                   ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5909
  %_95.i67.i1646 = sub nuw i64 %_144.1.i62.i1641, %_22.i63.i1642, !dbg !28565
  %_8.i6649 = icmp samesign ugt i64 %_95.i67.i1646, 7, !dbg !28566
  br i1 %_8.i6649, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6652, label %bb2.i6650, !dbg !28566, !prof !1421

bb2.i6650:                                        ; preds = %bb38.i65.i1644
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_95.i67.i1646, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !28571, !noalias !28572
  unreachable, !dbg !28571

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6652: ; preds = %bb38.i65.i1644
  %_144.0.i66.i1645 = load ptr, ptr %148, align 8, !dbg !28560, !alias.scope !28533, !noalias !28534, !nonnull !12, !noundef !12
  %_99.i68.i1647 = getelementptr inbounds nuw float, ptr %_144.0.i66.i1645, i64 %_22.i63.i1642, !dbg !28576
  store <8 x float> %434, ptr %_99.i68.i1647, align 4, !dbg !28578, !alias.scope !28582, !noalias !28586
  tail call void @llvm.experimental.noalias.scope.decl(metadata !28588), !dbg !28591
  %width.i2154 = load i64, ptr %146, align 8, !dbg !28592, !alias.scope !28588, !noalias !28594, !noundef !12
  %435 = icmp eq i64 %width.i2154, 0, !dbg !28596
  br i1 %435, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2262, label %bb32.i2161.lr.ph, !dbg !28596

bb32.i2161.lr.ph:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6652
  %_112.1.i2164 = load i64, ptr %62, align 8, !alias.scope !28588, !noalias !28594, !noundef !12
  %_112.0.i2168 = load ptr, ptr %61, align 8, !nonnull !12
  %436 = add i64 %ring_cursor.sroa.0.1.i161915369, 1
  %_23.not.i2175 = icmp ult i64 %436, %_91.i1639
  %437 = select i1 %_23.not.i2175, i64 0, i64 %_91.i1639
  %start1.sroa.0.0.i2176 = sub nuw i64 %436, %437
  %_114.1.i2179 = load i64, ptr %147, align 8
  %_114.0.i2183 = load ptr, ptr %148, align 8, !nonnull !12
  %_116.1.i2184 = load i64, ptr %149, align 8
  %_116.0.i2188 = load ptr, ptr %150, align 8, !nonnull !12
  %_118.1.i2192 = load i64, ptr %151, align 8
  %_118.0.i2196 = load ptr, ptr %152, align 8, !nonnull !12
  %_45.i2209 = mul i64 %width.i2154, %start1.sroa.0.0.i2176
  br label %bb32.i2161, !dbg !28596

bb32.i2161:                                       ; preds = %bb32.i2161.lr.ph, %bb31.i2224
  %iter.i2153.sroa.10.015350 = phi i64 [ %width.i2154, %bb32.i2161.lr.ph ], [ %438, %bb31.i2224 ]
  %iter.i2153.sroa.7.015349 = phi i64 [ 0, %bb32.i2161.lr.ph ], [ %_9.0.i, %bb31.i2224 ]
  %iter.i2153.sroa.0.0.idx15348 = phi i64 [ 0, %bb32.i2161.lr.ph ], [ %iter.i2153.sroa.0.0.add, %bb31.i2224 ]
  %iter.i2153.sroa.0.0.ptr15351 = getelementptr inbounds nuw i8, ptr %scratch.i1522, i64 %iter.i2153.sroa.0.0.idx15348, !dbg !28598
  %438 = add i64 %iter.i2153.sroa.10.015350, -1, !dbg !28598
  %_7.i.i7264 = icmp eq i64 %iter.i2153.sroa.0.0.idx15348, 32, !dbg !28599
  br i1 %_7.i.i7264, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2262, label %bb3.i2163, !dbg !28603

bb3.i2163:                                        ; preds = %bb32.i2161
  %iter.i2153.sroa.0.0.add = add nuw nsw i64 %iter.i2153.sroa.0.0.idx15348, 4, !dbg !28604
  %_9.0.i = add nuw nsw i64 %iter.i2153.sroa.7.015349, 1, !dbg !28606
  %exitcond17971.not = icmp eq i64 %iter.i2153.sroa.7.015349, %_112.1.i2164, !dbg !28607
  br i1 %exitcond17971.not, label %panic.i2166, label %bb5.i2167, !dbg !28607

bb5.i2167:                                        ; preds = %bb3.i2163
  %439 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i2168, i64 %iter.i2153.sroa.7.015349, !dbg !28607
  %shape.i2169 = load i32, ptr %439, align 4, !dbg !28607, !noalias !28608, !noundef !12
  %440 = getelementptr inbounds nuw i8, ptr %439, i64 4, !dbg !28607
  %shape3.i2170 = load i32, ptr %440, align 4, !dbg !28607, !noalias !28608, !noundef !12
  %window.i2171 = zext i32 %shape.i2169 to i64, !dbg !28609
  %_19.i2172 = zext i32 %shape3.i2170 to i64, !dbg !28610
  %441 = add i64 %ring_cursor.sroa.0.1.i161915369, %_19.i2172, !dbg !28611
  %_20.not.i2173 = icmp ult i64 %441, %_91.i1639, !dbg !28612
  %442 = select i1 %_20.not.i2173, i64 0, i64 %_91.i1639, !dbg !28612
  %spec.select.i2174 = sub nuw i64 %441, %442, !dbg !28612
  %_27.i2177 = mul i64 %spec.select.i2174, %width.i2154, !dbg !28613
  %_26.i2178 = add i64 %_27.i2177, %iter.i2153.sroa.7.015349, !dbg !28613
  %_30.i2180 = icmp ult i64 %_26.i2178, %_114.1.i2179, !dbg !28614
  br i1 %_30.i2180, label %bb12.i2182, label %panic5.i2181, !dbg !28614

panic.i2166:                                      ; preds = %bb3.i2163
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i2164, i64 noundef %_112.1.i2164, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4a8785a681d008a9bfd0cd82628ea9cb) #30, !dbg !28607, !noalias !28608
  unreachable, !dbg !28607

bb12.i2182:                                       ; preds = %bb5.i2167
  %443 = getelementptr inbounds nuw float, ptr %_114.0.i2183, i64 %_26.i2178, !dbg !28614
  %444 = load float, ptr %443, align 4, !dbg !28614, !noalias !28608, !noundef !12
  %exitcond17972.not = icmp eq i64 %iter.i2153.sroa.7.015349, %_116.1.i2184, !dbg !28615
  br i1 %exitcond17972.not, label %panic6.i2186, label %bb13.i2187, !dbg !28615

panic5.i2181:                                     ; preds = %bb5.i2167
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i2178, i64 noundef %_114.1.i2179, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cbce7773ac40979e4ba2385da3aec116) #30, !dbg !28614, !noalias !28608
  unreachable, !dbg !28614

bb13.i2187:                                       ; preds = %bb12.i2182
  %445 = getelementptr inbounds nuw i32, ptr %_116.0.i2188, i64 %iter.i2153.sroa.7.015349, !dbg !28615
  %_32.i2189 = load i32, ptr %445, align 4, !dbg !28615, !noalias !28608, !noundef !12
  %position.i2190 = zext i32 %_32.i2189 to i64, !dbg !28615
  %446 = icmp eq i32 %_32.i2189, 0, !dbg !28616
  br i1 %446, label %bb17.i2199, label %bb15.i2191, !dbg !28616

panic6.i2186:                                     ; preds = %bb12.i2182
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i2184, i64 noundef %_116.1.i2184, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ec0d48f73ebfc2755df5cedaa60b5c0a) #30, !dbg !28615, !noalias !28608
  unreachable, !dbg !28615

bb15.i2191:                                       ; preds = %bb13.i2187
  %_37.i2193 = icmp ult i64 %iter.i2153.sroa.7.015349, %_118.1.i2192, !dbg !28617
  br i1 %_37.i2193, label %bb16.i2195, label %panic7.i2194, !dbg !28617

bb17.i2199:                                       ; preds = %bb35.i2260, %bb16.i2195, %bb13.i2187
  %newest.sroa.0.0.i2200 = phi float [ %444, %bb13.i2187 ], [ %_35.i2197, %bb35.i2260 ], [ %444, %bb16.i2195 ], !dbg !28618
  %exitcond17973.not = icmp eq i64 %iter.i2153.sroa.7.015349, %_118.1.i2192, !dbg !28619
  br i1 %exitcond17973.not, label %panic8.i2203, label %bb18.i2204, !dbg !28619

bb16.i2195:                                       ; preds = %bb15.i2191
  %447 = getelementptr inbounds nuw float, ptr %_118.0.i2196, i64 %iter.i2153.sroa.7.015349, !dbg !28617
  %_35.i2197 = load float, ptr %447, align 4, !dbg !28617, !noalias !28608, !noundef !12
  %_102.i2198 = fcmp olt float %_35.i2197, %444, !dbg !28620
  br i1 %_102.i2198, label %bb35.i2260, label %bb17.i2199, !dbg !28620

panic7.i2194:                                     ; preds = %bb15.i2191
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i2153.sroa.7.015349, i64 noundef %_118.1.i2192, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2c461872bb652d4796cdcf89c28c82c8) #30, !dbg !28617, !noalias !28608
  unreachable, !dbg !28617

bb35.i2260:                                       ; preds = %bb16.i2195
  br label %bb17.i2199, !dbg !28622

bb18.i2204:                                       ; preds = %bb17.i2199
  %448 = getelementptr inbounds nuw float, ptr %_118.0.i2196, i64 %iter.i2153.sroa.7.015349, !dbg !28619
  store float %newest.sroa.0.0.i2200, ptr %448, align 4, !dbg !28619, !noalias !28608
  %_42.i2206 = add nuw nsw i64 %position.i2190, 1, !dbg !28623
  %complete.i2207 = icmp eq i64 %_42.i2206, %window.i2171, !dbg !28623
  br i1 %complete.i2207, label %bb22.i2229, label %bb20.i2208, !dbg !28624

panic8.i2203:                                     ; preds = %bb17.i2199
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i2192, i64 noundef %_118.1.i2192, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2b690e2c7763f11809942906fc2ca813) #30, !dbg !28619, !noalias !28608
  unreachable, !dbg !28619

bb20.i2208:                                       ; preds = %bb18.i2204
  %_44.i2210 = add i64 %iter.i2153.sroa.7.015349, %_45.i2209, !dbg !28625
  %_47.i2212 = icmp ult i64 %_44.i2210, %_114.1.i2179, !dbg !28626
  br i1 %_47.i2212, label %bb30.i2222, label %panic9.i2213, !dbg !28626

panic9.i2213:                                     ; preds = %bb20.i2208
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i2210, i64 noundef %_114.1.i2179, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9fa421ae81817f58fcfc4a3243223891) #30, !dbg !28626, !noalias !28608
  unreachable, !dbg !28626

bb30.i2222:                                       ; preds = %bb20.i2208
  %449 = getelementptr inbounds nuw float, ptr %_114.0.i2183, i64 %_44.i2210, !dbg !28626
  %_43.i2216 = load float, ptr %449, align 4, !dbg !28626, !noalias !28608, !noundef !12
  %_103.i2217 = fcmp olt float %_43.i2216, %newest.sroa.0.0.i2200, !dbg !28627
  %newest.sroa.0.1.i2218 = select i1 %_103.i2217, float %_43.i2216, float %newest.sroa.0.0.i2200, !dbg !28627
  store float %newest.sroa.0.1.i2218, ptr %iter.i2153.sroa.0.0.ptr15351, align 4, !dbg !28629, !noalias !28608
  %450 = trunc i64 %_42.i2206 to i32, !dbg !28630
  br label %bb31.i2224, !dbg !28631

bb31.i2224:                                       ; preds = %bb25.i2257, %bb30.i2222
  %storemerge = phi i32 [ %450, %bb30.i2222 ], [ 0, %bb25.i2257 ], !dbg !28632
  store i32 %storemerge, ptr %445, align 4, !dbg !28632, !noalias !28608
  %451 = icmp eq i64 %438, 0, !dbg !28596
  br i1 %451, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2262, label %bb32.i2161, !dbg !28596

bb22.i2229:                                       ; preds = %bb18.i2204
  store float %newest.sroa.0.0.i2200, ptr %iter.i2153.sroa.0.0.ptr15351, align 4, !dbg !28629, !noalias !28608
  %452 = load float, ptr %443, align 4, !dbg !28633, !noalias !28608, !noundef !12
  br label %bb41.i2242, !dbg !28634

bb41.i2242:                                       ; preds = %bb22.i2229, %bb25.i2257
  %iter2.sroa.0.0.i223415347 = phi i64 [ 0, %bb22.i2229 ], [ %_105.i2243, %bb25.i2257 ]
  %suffix.sroa.0.0.i223315346 = phi float [ %452, %bb22.i2229 ], [ %suffix.sroa.0.1.i2253, %bb25.i2257 ]
  %end.sroa.0.1.i223215345 = phi i64 [ %spec.select.i2174, %bb22.i2229 ], [ %455, %bb25.i2257 ]
  %_56.i2244 = mul i64 %end.sroa.0.1.i223215345, %width.i2154, !dbg !28637
  %_55.i2245 = add i64 %_56.i2244, %iter.i2153.sroa.7.015349, !dbg !28637
  %_59.i2247 = icmp ult i64 %_55.i2245, %_114.1.i2179, !dbg !28638
  br i1 %_59.i2247, label %bb25.i2257, label %panic13.i2248, !dbg !28638

panic13.i2248:                                    ; preds = %bb41.i2242
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i2245, i64 noundef %_114.1.i2179, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_91a4c6b9b17ebf4d863f9a70b6dc929a) #30, !dbg !28638, !noalias !28608
  unreachable, !dbg !28638

bb25.i2257:                                       ; preds = %bb41.i2242
  %_105.i2243 = add nuw nsw i64 %iter2.sroa.0.0.i223415347, 1, !dbg !28639
  %453 = getelementptr inbounds nuw float, ptr %_114.0.i2183, i64 %_55.i2245, !dbg !28638
  %_54.i2251 = load float, ptr %453, align 4, !dbg !28638, !noalias !28608, !noundef !12
  %_107.i2252 = fcmp olt float %suffix.sroa.0.0.i223315346, %_54.i2251, !dbg !28642
  %suffix.sroa.0.1.i2253 = select i1 %_107.i2252, float %suffix.sroa.0.0.i223315346, float %_54.i2251, !dbg !28642
  store float %suffix.sroa.0.1.i2253, ptr %453, align 4, !dbg !28644, !noalias !28608
  %454 = icmp eq i64 %end.sroa.0.1.i223215345, 0, !dbg !28645
  %spec.store.select.i2259 = select i1 %454, i64 %_91.i1639, i64 %end.sroa.0.1.i223215345, !dbg !28645
  %455 = add i64 %spec.store.select.i2259, -1, !dbg !28646
  %exitcond17970.not = icmp eq i64 %_105.i2243, %window.i2171, !dbg !28647
  br i1 %exitcond17970.not, label %bb31.i2224, label %bb41.i2242, !dbg !28634

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2262: ; preds = %bb31.i2224, %bb32.i2161, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6652
  %lanes.i5895.sroa.0.0.copyload = load <8 x float>, ptr %scratch.i1522, align 4, !dbg !28649, !alias.scope !28654, !noalias !28658
  %456 = fmul <8 x float> %lanes.i5895.sroa.0.0.copyload, splat (float 1.638400e+04), !dbg !28662
  %457 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %456), !dbg !28667
  %458 = fmul <8 x float> %457, splat (float 0x3F10000000000000), !dbg !28672
  %459 = icmp eq i64 %width.i61.i1640, 0, !dbg !28677
  %_149.1.i96.i1675.pre = load i64, ptr %153, align 8, !dbg !28679, !alias.scope !28533, !noalias !28534
  br i1 %459, label %bb16.i95.i1674, label %bb39.i75.i1654.lr.ph, !dbg !28677

bb39.i75.i1654.lr.ph:                             ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2262
  %_145.1.i78.i1657 = load i64, ptr %62, align 8, !alias.scope !28533, !noalias !28534, !noundef !12
  %_145.0.i82.i1661 = load ptr, ptr %61, align 8, !nonnull !12
  %_147.0.i93.i1672 = load ptr, ptr %154, align 8, !nonnull !12
  %exitcond17976.not = icmp eq i64 %_145.1.i78.i1657, 0, !dbg !28680
  br i1 %exitcond17976.not, label %panic.i80.i1659, label %bb17.i81.i1660, !dbg !28680

bb37.i122.i1775:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5909
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i63.i1642, i64 noundef %_144.1.i62.i1641, i64 noundef %_144.1.i62.i1641, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_56df7c041d29359441bca272bf4e38e3) #30, !dbg !28681, !noalias !28682
  unreachable, !dbg !28681

bb16.i95.i1674.loopexit:                          ; preds = %bb21.i92.i1671.7, %bb21.i92.i1671.6, %bb21.i92.i1671.5, %bb21.i92.i1671.4, %bb21.i92.i1671.3, %bb21.i92.i1671.2, %bb21.i92.i1671.1, %bb21.i92.i1671
  %lanes.i5888.sroa.0.0.copyload.pre = load <8 x float>, ptr %scratch.i1522, align 4, !dbg !28683, !alias.scope !28688, !noalias !28692
  br label %bb16.i95.i1674, !dbg !28696

bb16.i95.i1674:                                   ; preds = %bb16.i95.i1674.loopexit, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2262
  %lanes.i5888.sroa.0.0.copyload = phi <8 x float> [ %lanes.i5888.sroa.0.0.copyload.pre, %bb16.i95.i1674.loopexit ], [ %lanes.i5895.sroa.0.0.copyload, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2262 ], !dbg !28683
  %460 = fadd <8 x float> %458, %_58.i46.i1450.sroa.0.0.copyload20578, !dbg !28697
  %461 = fsub <8 x float> %460, %lanes.i5888.sroa.0.0.copyload, !dbg !28702
  store <8 x float> %461, ptr %155, align 32, !dbg !28707
  %_109.i97.i1676 = icmp ugt i64 %_22.i63.i1642, %_149.1.i96.i1675.pre, !dbg !28708
  br i1 %_109.i97.i1676, label %bb42.i121.i1774, label %bb43.i98.i1677, !dbg !28708, !prof !1406

bb43.i98.i1677:                                   ; preds = %bb16.i95.i1674
  %_112.i100.i1679 = sub nuw i64 %_149.1.i96.i1675.pre, %_22.i63.i1642, !dbg !28711
  %_8.i6644 = icmp samesign ugt i64 %_112.i100.i1679, 7, !dbg !28712
  br i1 %_8.i6644, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6647, label %bb2.i6645, !dbg !28712, !prof !1421

bb2.i6645:                                        ; preds = %bb43.i98.i1677
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_112.i100.i1679, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !28717, !noalias !28718
  unreachable, !dbg !28717

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6647: ; preds = %bb43.i98.i1677
  %_149.0.i99.i1678 = load ptr, ptr %154, align 8, !dbg !28679, !alias.scope !28533, !noalias !28534, !nonnull !12, !noundef !12
  %_116.i101.i1680 = getelementptr inbounds nuw float, ptr %_149.0.i99.i1678, i64 %_22.i63.i1642, !dbg !28722
  store <8 x float> %458, ptr %_116.i101.i1680, align 4, !dbg !28724, !alias.scope !28728, !noalias !28732
  %_64.i43.i1447.sroa.0.0.copyload = load <8 x float>, ptr %156, align 32, !dbg !28734
  %_68.i39.i1443.sroa.0.0.copyload = load <8 x float>, ptr %157, align 32, !dbg !28735
  %462 = fdiv <8 x float> %461, %_64.i43.i1447.sroa.0.0.copyload, !dbg !28736
  %463 = fsub <8 x float> splat (float 1.000000e+00), %462, !dbg !28741
  %464 = fsub <8 x float> %463, %_68.i39.i1443.sroa.0.0.copyload, !dbg !28746
  %465 = fmul <8 x float> %408, %464, !dbg !28751
  %466 = fadd <8 x float> %_68.i39.i1443.sroa.0.0.copyload, %465, !dbg !28756
  %467 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %463, <8 x float> %466), !dbg !28760
  %468 = bitcast <8 x float> %467 to <8 x i32>, !dbg !28765
  %469 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %467), !dbg !28771
  %470 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %469, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !28773
  %471 = bitcast <8 x float> %470 to <8 x i32>, !dbg !28779
  %472 = xor <8 x i32> %471, splat (i32 -1), !dbg !28785
  %473 = and <8 x i32> %472, %468, !dbg !28787
  %474 = bitcast <8 x i32> %473 to <8 x float>, !dbg !28791
  store <8 x i32> %473, ptr %157, align 32, !dbg !28792
  %475 = fsub <8 x float> splat (float 1.000000e+00), %474, !dbg !28793
  %_150.1.i102.i1681 = load i64, ptr %158, align 8, !dbg !28798, !alias.scope !28533, !noalias !28534, !noundef !12
  %_76.i103.i1682 = mul i64 %width.i61.i1640, %main_cursor.sroa.0.1.i162015370, !dbg !28799
  %_120.i104.i1683 = icmp ugt i64 %_76.i103.i1682, %_150.1.i102.i1681, !dbg !28800
  br i1 %_120.i104.i1683, label %bb48.i120.i1773, label %bb49.i105.i1684, !dbg !28800, !prof !1406

bb42.i121.i1774:                                  ; preds = %bb16.i95.i1674
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i63.i1642, i64 noundef %_149.1.i96.i1675.pre, i64 noundef %_149.1.i96.i1675.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_90498045d73339daaf9e4f537508f58b) #30, !dbg !28803, !noalias !28804
  unreachable, !dbg !28803

bb49.i105.i1684:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6647
  %_123.i107.i1686 = sub nuw i64 %_150.1.i102.i1681, %_76.i103.i1682, !dbg !28805
  %_8.i5882 = icmp samesign ugt i64 %_123.i107.i1686, 7, !dbg !28806
  br i1 %_8.i5882, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6637, label %bb2.i5883, !dbg !28806, !prof !1421

bb2.i5883:                                        ; preds = %bb49.i105.i1684
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_123.i107.i1686, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !28811, !noalias !28812
  unreachable, !dbg !28811

bb48.i120.i1773:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6647
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i103.i1682, i64 noundef %_150.1.i102.i1681, i64 noundef %_150.1.i102.i1681, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_da8af254b6d507a8e2ca31e544bfd21d) #30, !dbg !28816, !noalias !28804
  unreachable, !dbg !28816

bb17.i81.i1660:                                   ; preds = %bb39.i75.i1654.lr.ph
  %476 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i1661, i64 8, !dbg !28680
  %_44.i83.i1662 = load i32, ptr %476, align 4, !dbg !28680, !noalias !28804, !noundef !12
  %_43.i84.i1663 = zext i32 %_44.i83.i1662 to i64, !dbg !28680
  %477 = add i64 %ring_cursor.sroa.0.1.i161915369, %_43.i84.i1663, !dbg !28817
  %_47.not.i85.i1664 = icmp ult i64 %477, %_91.i1639, !dbg !28818
  %478 = select i1 %_47.not.i85.i1664, i64 0, i64 %_91.i1639, !dbg !28818
  %spec.select.i86.i1665 = sub nuw i64 %477, %478, !dbg !28818
  %_51.i87.i1666 = mul i64 %spec.select.i86.i1665, %width.i61.i1640, !dbg !28819
  %_53.i90.i1669 = icmp ult i64 %_51.i87.i1666, %_149.1.i96.i1675.pre, !dbg !28820
  br i1 %_53.i90.i1669, label %bb21.i92.i1671, label %panic1.i91.i1670, !dbg !28820

panic.i80.i1659:                                  ; preds = %bb39.i75.i1654.7, %bb39.i75.i1654.6, %bb39.i75.i1654.5, %bb39.i75.i1654.4, %bb39.i75.i1654.3, %bb39.i75.i1654.2, %bb39.i75.i1654.1, %bb39.i75.i1654.lr.ph
  %_145.1.i78.i1657.lcssa.ph = phi i64 [ 7, %bb39.i75.i1654.7 ], [ 6, %bb39.i75.i1654.6 ], [ 5, %bb39.i75.i1654.5 ], [ 4, %bb39.i75.i1654.4 ], [ 3, %bb39.i75.i1654.3 ], [ 2, %bb39.i75.i1654.2 ], [ 1, %bb39.i75.i1654.1 ], [ 0, %bb39.i75.i1654.lr.ph ]
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i78.i1657.lcssa.ph, i64 noundef %_145.1.i78.i1657.lcssa.ph, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a6feb40b34112df5f214f84424dd2c7c) #30, !dbg !28680, !noalias !28804
  unreachable, !dbg !28680

bb21.i92.i1671:                                   ; preds = %bb17.i81.i1660
  %479 = getelementptr inbounds nuw float, ptr %_147.0.i93.i1672, i64 %_51.i87.i1666, !dbg !28820
  %_49.i94.i1673 = load float, ptr %479, align 4, !dbg !28820, !noalias !28804, !noundef !12
  store float %_49.i94.i1673, ptr %scratch.i1522, align 4, !dbg !28821, !noalias !28804
  %480 = icmp eq i64 %width.i61.i1640, 1, !dbg !28677
  br i1 %480, label %bb16.i95.i1674.loopexit, label %bb39.i75.i1654.1, !dbg !28677

bb39.i75.i1654.1:                                 ; preds = %bb21.i92.i1671
  %exitcond17976.1.not = icmp eq i64 %_145.1.i78.i1657, 1, !dbg !28680
  br i1 %exitcond17976.1.not, label %panic.i80.i1659, label %bb17.i81.i1660.1, !dbg !28680

bb17.i81.i1660.1:                                 ; preds = %bb39.i75.i1654.1
  %481 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i1661, i64 20, !dbg !28680
  %_44.i83.i1662.1 = load i32, ptr %481, align 4, !dbg !28680, !noalias !28804, !noundef !12
  %_43.i84.i1663.1 = zext i32 %_44.i83.i1662.1 to i64, !dbg !28680
  %482 = add i64 %ring_cursor.sroa.0.1.i161915369, %_43.i84.i1663.1, !dbg !28817
  %_47.not.i85.i1664.1 = icmp ult i64 %482, %_91.i1639, !dbg !28818
  %483 = select i1 %_47.not.i85.i1664.1, i64 0, i64 %_91.i1639, !dbg !28818
  %spec.select.i86.i1665.1 = sub nuw i64 %482, %483, !dbg !28818
  %_51.i87.i1666.1 = mul i64 %spec.select.i86.i1665.1, %width.i61.i1640, !dbg !28819
  %_50.i88.i1667.1 = add i64 %_51.i87.i1666.1, 1, !dbg !28819
  %_53.i90.i1669.1 = icmp ult i64 %_50.i88.i1667.1, %_149.1.i96.i1675.pre, !dbg !28820
  br i1 %_53.i90.i1669.1, label %bb21.i92.i1671.1, label %panic1.i91.i1670, !dbg !28820

bb21.i92.i1671.1:                                 ; preds = %bb17.i81.i1660.1
  %484 = getelementptr inbounds nuw float, ptr %_147.0.i93.i1672, i64 %_50.i88.i1667.1, !dbg !28820
  %_49.i94.i1673.1 = load float, ptr %484, align 4, !dbg !28820, !noalias !28804, !noundef !12
  store float %_49.i94.i1673.1, ptr %iter.i49.i1453.sroa.0.0.ptr15355.1, align 4, !dbg !28821, !noalias !28804
  %485 = icmp eq i64 %width.i61.i1640, 2, !dbg !28677
  br i1 %485, label %bb16.i95.i1674.loopexit, label %bb39.i75.i1654.2, !dbg !28677

bb39.i75.i1654.2:                                 ; preds = %bb21.i92.i1671.1
  %exitcond17976.2.not = icmp eq i64 %_145.1.i78.i1657, 2, !dbg !28680
  br i1 %exitcond17976.2.not, label %panic.i80.i1659, label %bb17.i81.i1660.2, !dbg !28680

bb17.i81.i1660.2:                                 ; preds = %bb39.i75.i1654.2
  %486 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i1661, i64 32, !dbg !28680
  %_44.i83.i1662.2 = load i32, ptr %486, align 4, !dbg !28680, !noalias !28804, !noundef !12
  %_43.i84.i1663.2 = zext i32 %_44.i83.i1662.2 to i64, !dbg !28680
  %487 = add i64 %ring_cursor.sroa.0.1.i161915369, %_43.i84.i1663.2, !dbg !28817
  %_47.not.i85.i1664.2 = icmp ult i64 %487, %_91.i1639, !dbg !28818
  %488 = select i1 %_47.not.i85.i1664.2, i64 0, i64 %_91.i1639, !dbg !28818
  %spec.select.i86.i1665.2 = sub nuw i64 %487, %488, !dbg !28818
  %_51.i87.i1666.2 = mul i64 %spec.select.i86.i1665.2, %width.i61.i1640, !dbg !28819
  %_50.i88.i1667.2 = add i64 %_51.i87.i1666.2, 2, !dbg !28819
  %_53.i90.i1669.2 = icmp ult i64 %_50.i88.i1667.2, %_149.1.i96.i1675.pre, !dbg !28820
  br i1 %_53.i90.i1669.2, label %bb21.i92.i1671.2, label %panic1.i91.i1670, !dbg !28820

bb21.i92.i1671.2:                                 ; preds = %bb17.i81.i1660.2
  %489 = getelementptr inbounds nuw float, ptr %_147.0.i93.i1672, i64 %_50.i88.i1667.2, !dbg !28820
  %_49.i94.i1673.2 = load float, ptr %489, align 4, !dbg !28820, !noalias !28804, !noundef !12
  store float %_49.i94.i1673.2, ptr %iter.i49.i1453.sroa.0.0.ptr15355.2, align 4, !dbg !28821, !noalias !28804
  %490 = icmp eq i64 %width.i61.i1640, 3, !dbg !28677
  br i1 %490, label %bb16.i95.i1674.loopexit, label %bb39.i75.i1654.3, !dbg !28677

bb39.i75.i1654.3:                                 ; preds = %bb21.i92.i1671.2
  %exitcond17976.3.not = icmp eq i64 %_145.1.i78.i1657, 3, !dbg !28680
  br i1 %exitcond17976.3.not, label %panic.i80.i1659, label %bb17.i81.i1660.3, !dbg !28680

bb17.i81.i1660.3:                                 ; preds = %bb39.i75.i1654.3
  %491 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i1661, i64 44, !dbg !28680
  %_44.i83.i1662.3 = load i32, ptr %491, align 4, !dbg !28680, !noalias !28804, !noundef !12
  %_43.i84.i1663.3 = zext i32 %_44.i83.i1662.3 to i64, !dbg !28680
  %492 = add i64 %ring_cursor.sroa.0.1.i161915369, %_43.i84.i1663.3, !dbg !28817
  %_47.not.i85.i1664.3 = icmp ult i64 %492, %_91.i1639, !dbg !28818
  %493 = select i1 %_47.not.i85.i1664.3, i64 0, i64 %_91.i1639, !dbg !28818
  %spec.select.i86.i1665.3 = sub nuw i64 %492, %493, !dbg !28818
  %_51.i87.i1666.3 = mul i64 %spec.select.i86.i1665.3, %width.i61.i1640, !dbg !28819
  %_50.i88.i1667.3 = add i64 %_51.i87.i1666.3, 3, !dbg !28819
  %_53.i90.i1669.3 = icmp ult i64 %_50.i88.i1667.3, %_149.1.i96.i1675.pre, !dbg !28820
  br i1 %_53.i90.i1669.3, label %bb21.i92.i1671.3, label %panic1.i91.i1670, !dbg !28820

bb21.i92.i1671.3:                                 ; preds = %bb17.i81.i1660.3
  %494 = getelementptr inbounds nuw float, ptr %_147.0.i93.i1672, i64 %_50.i88.i1667.3, !dbg !28820
  %_49.i94.i1673.3 = load float, ptr %494, align 4, !dbg !28820, !noalias !28804, !noundef !12
  store float %_49.i94.i1673.3, ptr %iter.i49.i1453.sroa.0.0.ptr15355.3, align 4, !dbg !28821, !noalias !28804
  %495 = icmp eq i64 %width.i61.i1640, 4, !dbg !28677
  br i1 %495, label %bb16.i95.i1674.loopexit, label %bb39.i75.i1654.4, !dbg !28677

bb39.i75.i1654.4:                                 ; preds = %bb21.i92.i1671.3
  %exitcond17976.4.not = icmp eq i64 %_145.1.i78.i1657, 4, !dbg !28680
  br i1 %exitcond17976.4.not, label %panic.i80.i1659, label %bb17.i81.i1660.4, !dbg !28680

bb17.i81.i1660.4:                                 ; preds = %bb39.i75.i1654.4
  %496 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i1661, i64 56, !dbg !28680
  %_44.i83.i1662.4 = load i32, ptr %496, align 4, !dbg !28680, !noalias !28804, !noundef !12
  %_43.i84.i1663.4 = zext i32 %_44.i83.i1662.4 to i64, !dbg !28680
  %497 = add i64 %ring_cursor.sroa.0.1.i161915369, %_43.i84.i1663.4, !dbg !28817
  %_47.not.i85.i1664.4 = icmp ult i64 %497, %_91.i1639, !dbg !28818
  %498 = select i1 %_47.not.i85.i1664.4, i64 0, i64 %_91.i1639, !dbg !28818
  %spec.select.i86.i1665.4 = sub nuw i64 %497, %498, !dbg !28818
  %_51.i87.i1666.4 = mul i64 %spec.select.i86.i1665.4, %width.i61.i1640, !dbg !28819
  %_50.i88.i1667.4 = add i64 %_51.i87.i1666.4, 4, !dbg !28819
  %_53.i90.i1669.4 = icmp ult i64 %_50.i88.i1667.4, %_149.1.i96.i1675.pre, !dbg !28820
  br i1 %_53.i90.i1669.4, label %bb21.i92.i1671.4, label %panic1.i91.i1670, !dbg !28820

bb21.i92.i1671.4:                                 ; preds = %bb17.i81.i1660.4
  %499 = getelementptr inbounds nuw float, ptr %_147.0.i93.i1672, i64 %_50.i88.i1667.4, !dbg !28820
  %_49.i94.i1673.4 = load float, ptr %499, align 4, !dbg !28820, !noalias !28804, !noundef !12
  store float %_49.i94.i1673.4, ptr %iter.i49.i1453.sroa.0.0.ptr15355.4, align 4, !dbg !28821, !noalias !28804
  %500 = icmp eq i64 %width.i61.i1640, 5, !dbg !28677
  br i1 %500, label %bb16.i95.i1674.loopexit, label %bb39.i75.i1654.5, !dbg !28677

bb39.i75.i1654.5:                                 ; preds = %bb21.i92.i1671.4
  %exitcond17976.5.not = icmp eq i64 %_145.1.i78.i1657, 5, !dbg !28680
  br i1 %exitcond17976.5.not, label %panic.i80.i1659, label %bb17.i81.i1660.5, !dbg !28680

bb17.i81.i1660.5:                                 ; preds = %bb39.i75.i1654.5
  %501 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i1661, i64 68, !dbg !28680
  %_44.i83.i1662.5 = load i32, ptr %501, align 4, !dbg !28680, !noalias !28804, !noundef !12
  %_43.i84.i1663.5 = zext i32 %_44.i83.i1662.5 to i64, !dbg !28680
  %502 = add i64 %ring_cursor.sroa.0.1.i161915369, %_43.i84.i1663.5, !dbg !28817
  %_47.not.i85.i1664.5 = icmp ult i64 %502, %_91.i1639, !dbg !28818
  %503 = select i1 %_47.not.i85.i1664.5, i64 0, i64 %_91.i1639, !dbg !28818
  %spec.select.i86.i1665.5 = sub nuw i64 %502, %503, !dbg !28818
  %_51.i87.i1666.5 = mul i64 %spec.select.i86.i1665.5, %width.i61.i1640, !dbg !28819
  %_50.i88.i1667.5 = add i64 %_51.i87.i1666.5, 5, !dbg !28819
  %_53.i90.i1669.5 = icmp ult i64 %_50.i88.i1667.5, %_149.1.i96.i1675.pre, !dbg !28820
  br i1 %_53.i90.i1669.5, label %bb21.i92.i1671.5, label %panic1.i91.i1670, !dbg !28820

bb21.i92.i1671.5:                                 ; preds = %bb17.i81.i1660.5
  %504 = getelementptr inbounds nuw float, ptr %_147.0.i93.i1672, i64 %_50.i88.i1667.5, !dbg !28820
  %_49.i94.i1673.5 = load float, ptr %504, align 4, !dbg !28820, !noalias !28804, !noundef !12
  store float %_49.i94.i1673.5, ptr %iter.i49.i1453.sroa.0.0.ptr15355.5, align 4, !dbg !28821, !noalias !28804
  %505 = icmp eq i64 %width.i61.i1640, 6, !dbg !28677
  br i1 %505, label %bb16.i95.i1674.loopexit, label %bb39.i75.i1654.6, !dbg !28677

bb39.i75.i1654.6:                                 ; preds = %bb21.i92.i1671.5
  %exitcond17976.6.not = icmp eq i64 %_145.1.i78.i1657, 6, !dbg !28680
  br i1 %exitcond17976.6.not, label %panic.i80.i1659, label %bb17.i81.i1660.6, !dbg !28680

bb17.i81.i1660.6:                                 ; preds = %bb39.i75.i1654.6
  %506 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i1661, i64 80, !dbg !28680
  %_44.i83.i1662.6 = load i32, ptr %506, align 4, !dbg !28680, !noalias !28804, !noundef !12
  %_43.i84.i1663.6 = zext i32 %_44.i83.i1662.6 to i64, !dbg !28680
  %507 = add i64 %ring_cursor.sroa.0.1.i161915369, %_43.i84.i1663.6, !dbg !28817
  %_47.not.i85.i1664.6 = icmp ult i64 %507, %_91.i1639, !dbg !28818
  %508 = select i1 %_47.not.i85.i1664.6, i64 0, i64 %_91.i1639, !dbg !28818
  %spec.select.i86.i1665.6 = sub nuw i64 %507, %508, !dbg !28818
  %_51.i87.i1666.6 = mul i64 %spec.select.i86.i1665.6, %width.i61.i1640, !dbg !28819
  %_50.i88.i1667.6 = add i64 %_51.i87.i1666.6, 6, !dbg !28819
  %_53.i90.i1669.6 = icmp ult i64 %_50.i88.i1667.6, %_149.1.i96.i1675.pre, !dbg !28820
  br i1 %_53.i90.i1669.6, label %bb21.i92.i1671.6, label %panic1.i91.i1670, !dbg !28820

bb21.i92.i1671.6:                                 ; preds = %bb17.i81.i1660.6
  %509 = getelementptr inbounds nuw float, ptr %_147.0.i93.i1672, i64 %_50.i88.i1667.6, !dbg !28820
  %_49.i94.i1673.6 = load float, ptr %509, align 4, !dbg !28820, !noalias !28804, !noundef !12
  store float %_49.i94.i1673.6, ptr %iter.i49.i1453.sroa.0.0.ptr15355.6, align 4, !dbg !28821, !noalias !28804
  %510 = icmp eq i64 %width.i61.i1640, 7, !dbg !28677
  br i1 %510, label %bb16.i95.i1674.loopexit, label %bb39.i75.i1654.7, !dbg !28677

bb39.i75.i1654.7:                                 ; preds = %bb21.i92.i1671.6
  %exitcond17976.7.not = icmp eq i64 %_145.1.i78.i1657, 7, !dbg !28680
  br i1 %exitcond17976.7.not, label %panic.i80.i1659, label %bb17.i81.i1660.7, !dbg !28680

bb17.i81.i1660.7:                                 ; preds = %bb39.i75.i1654.7
  %511 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i1661, i64 92, !dbg !28680
  %_44.i83.i1662.7 = load i32, ptr %511, align 4, !dbg !28680, !noalias !28804, !noundef !12
  %_43.i84.i1663.7 = zext i32 %_44.i83.i1662.7 to i64, !dbg !28680
  %512 = add i64 %ring_cursor.sroa.0.1.i161915369, %_43.i84.i1663.7, !dbg !28817
  %_47.not.i85.i1664.7 = icmp ult i64 %512, %_91.i1639, !dbg !28818
  %513 = select i1 %_47.not.i85.i1664.7, i64 0, i64 %_91.i1639, !dbg !28818
  %spec.select.i86.i1665.7 = sub nuw i64 %512, %513, !dbg !28818
  %_51.i87.i1666.7 = mul i64 %spec.select.i86.i1665.7, %width.i61.i1640, !dbg !28819
  %_50.i88.i1667.7 = add i64 %_51.i87.i1666.7, 7, !dbg !28819
  %_53.i90.i1669.7 = icmp ult i64 %_50.i88.i1667.7, %_149.1.i96.i1675.pre, !dbg !28820
  br i1 %_53.i90.i1669.7, label %bb21.i92.i1671.7, label %panic1.i91.i1670, !dbg !28820

bb21.i92.i1671.7:                                 ; preds = %bb17.i81.i1660.7
  %514 = getelementptr inbounds nuw float, ptr %_147.0.i93.i1672, i64 %_50.i88.i1667.7, !dbg !28820
  %_49.i94.i1673.7 = load float, ptr %514, align 4, !dbg !28820, !noalias !28804, !noundef !12
  store float %_49.i94.i1673.7, ptr %iter.i49.i1453.sroa.0.0.ptr15355.7, align 4, !dbg !28821, !noalias !28804
  br label %bb16.i95.i1674.loopexit, !dbg !28677

panic1.i91.i1670:                                 ; preds = %bb17.i81.i1660.7, %bb17.i81.i1660.6, %bb17.i81.i1660.5, %bb17.i81.i1660.4, %bb17.i81.i1660.3, %bb17.i81.i1660.2, %bb17.i81.i1660.1, %bb17.i81.i1660
  %_50.i88.i1667.lcssa.ph = phi i64 [ %_50.i88.i1667.7, %bb17.i81.i1660.7 ], [ %_50.i88.i1667.6, %bb17.i81.i1660.6 ], [ %_50.i88.i1667.5, %bb17.i81.i1660.5 ], [ %_50.i88.i1667.4, %bb17.i81.i1660.4 ], [ %_50.i88.i1667.3, %bb17.i81.i1660.3 ], [ %_50.i88.i1667.2, %bb17.i81.i1660.2 ], [ %_50.i88.i1667.1, %bb17.i81.i1660.1 ], [ %_51.i87.i1666, %bb17.i81.i1660 ]
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i88.i1667.lcssa.ph, i64 noundef %_149.1.i96.i1675.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b305c1483509cfb31fdec21ff8752674) #30, !dbg !28820, !noalias !28804
  unreachable, !dbg !28820

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6637: ; preds = %bb49.i105.i1684
  %_150.0.i106.i1685 = load ptr, ptr %159, align 8, !dbg !28798, !alias.scope !28533, !noalias !28534, !nonnull !12, !noundef !12
  %_127.i108.i1687 = getelementptr inbounds nuw float, ptr %_150.0.i106.i1685, i64 %_76.i103.i1682, !dbg !28822
  %lanes.i5879.sroa.0.0.copyload = load <8 x float>, ptr %_127.i108.i1687, align 4, !dbg !28824, !alias.scope !28828, !noalias !28832
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_127.i108.i1687, ptr noundef nonnull align 4 dereferenceable(32) %_174.i1638, i64 32, i1 false), !dbg !28834
  %515 = fmul <8 x float> %475, %lanes.i5879.sroa.0.0.copyload, !dbg !28839
  %516 = select <8 x i1> %161, <8 x float> %lanes.i5879.sroa.0.0.copyload, <8 x float> %515, !dbg !28844
  store <8 x float> %516, ptr %_174.i1638, align 4, !dbg !28849, !alias.scope !28854, !noalias !28858
  %_175.i1698 = icmp samesign ugt i64 %base.i1624, %right_io.1, !dbg !28862
  br i1 %_175.i1698, label %bb54.i1770, label %bb55.i1699, !dbg !28862, !prof !1406

bb52.i1776:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5927
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1624, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_422064f3ca430d31d9007f55b436c6ca) #30, !dbg !28866, !noalias !27149
  unreachable, !dbg !28866

bb55.i1699:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6637
  %_177.i1700 = sub nuw nsw i64 %right_io.1, %base.i1624, !dbg !28867
  %_181.i1701 = getelementptr inbounds nuw float, ptr %right_io.0, i64 %base.i1624, !dbg !28868
  %_8.i5873 = icmp samesign ugt i64 %_177.i1700, 7, !dbg !28873
  br i1 %_8.i5873, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5877, label %bb2.i5874, !dbg !28873, !prof !1421

bb2.i5874:                                        ; preds = %bb55.i1699
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_177.i1700, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !28878, !noalias !28879
  unreachable, !dbg !28878

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5877: ; preds = %bb55.i1699
  tail call void @llvm.experimental.noalias.scope.decl(metadata !28883), !dbg !28886
  %width.i.i1702 = load i64, ptr %162, align 8, !dbg !28887, !alias.scope !28889, !noalias !28890, !noundef !12
  %517 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %429, <8 x float> %416, i8 30), !dbg !28899
  %518 = fdiv <8 x float> %416, %429, !dbg !28905
  %519 = bitcast <8 x float> %517 to <8 x i32>, !dbg !28910
  %520 = icmp slt <8 x i32> %519, zeroinitializer, !dbg !28914
  %521 = select <8 x i1> %520, <8 x float> %518, <8 x float> splat (float 1.000000e+00), !dbg !28914
  %_144.1.i.i1703 = load i64, ptr %163, align 8, !dbg !28916, !alias.scope !28889, !noalias !28890, !noundef !12
  %_22.i.i1704 = mul i64 %width.i.i1702, %ring_cursor.sroa.0.1.i161915369, !dbg !28917
  %_92.i.i1705 = icmp ugt i64 %_22.i.i1704, %_144.1.i.i1703, !dbg !28918
  br i1 %_92.i.i1705, label %bb37.i.i1769, label %bb38.i.i1706, !dbg !28918, !prof !1406

bb38.i.i1706:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5877
  %_95.i.i1708 = sub nuw i64 %_144.1.i.i1703, %_22.i.i1704, !dbg !28921
  %_8.i6629 = icmp samesign ugt i64 %_95.i.i1708, 7, !dbg !28922
  br i1 %_8.i6629, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6632, label %bb2.i6630, !dbg !28922, !prof !1421

bb2.i6630:                                        ; preds = %bb38.i.i1706
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_95.i.i1708, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !28927, !noalias !28928
  unreachable, !dbg !28927

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6632: ; preds = %bb38.i.i1706
  %_144.0.i.i1707 = load ptr, ptr %164, align 8, !dbg !28916, !alias.scope !28889, !noalias !28890, !nonnull !12, !noundef !12
  %_99.i.i1709 = getelementptr inbounds nuw float, ptr %_144.0.i.i1707, i64 %_22.i.i1704, !dbg !28932
  store <8 x float> %521, ptr %_99.i.i1709, align 4, !dbg !28934, !alias.scope !28938, !noalias !28942
  tail call void @llvm.experimental.noalias.scope.decl(metadata !28944), !dbg !28947
  %width.i = load i64, ptr %162, align 8, !dbg !28948, !alias.scope !28944, !noalias !28950, !noundef !12
  %522 = icmp eq i64 %width.i, 0, !dbg !28952
  br i1 %522, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit, label %bb32.i2122.lr.ph, !dbg !28952

bb32.i2122.lr.ph:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6632
  %_112.1.i = load i64, ptr %165, align 8, !alias.scope !28944, !noalias !28950, !noundef !12
  %_112.0.i = load ptr, ptr %166, align 8, !nonnull !12
  %523 = add i64 %ring_cursor.sroa.0.1.i161915369, 1
  %_23.not.i = icmp ult i64 %523, %_91.i1639
  %524 = select i1 %_23.not.i, i64 0, i64 %_91.i1639
  %start1.sroa.0.0.i = sub nuw i64 %523, %524
  %_114.1.i = load i64, ptr %163, align 8
  %_114.0.i = load ptr, ptr %164, align 8, !nonnull !12
  %_116.1.i = load i64, ptr %167, align 8
  %_116.0.i = load ptr, ptr %168, align 8, !nonnull !12
  %_118.1.i = load i64, ptr %169, align 8
  %_118.0.i = load ptr, ptr %170, align 8, !nonnull !12
  %_45.i2141 = mul i64 %width.i, %start1.sroa.0.0.i
  br label %bb32.i2122, !dbg !28952

bb32.i2122:                                       ; preds = %bb32.i2122.lr.ph, %bb31.i2144
  %iter.i2119.sroa.10.015361 = phi i64 [ %width.i, %bb32.i2122.lr.ph ], [ %525, %bb31.i2144 ]
  %iter.i2119.sroa.7.015360 = phi i64 [ 0, %bb32.i2122.lr.ph ], [ %_9.0.i7294, %bb31.i2144 ]
  %iter.i2119.sroa.0.0.idx15359 = phi i64 [ 0, %bb32.i2122.lr.ph ], [ %iter.i2119.sroa.0.0.add, %bb31.i2144 ]
  %iter.i2119.sroa.0.0.ptr15362 = getelementptr inbounds nuw i8, ptr %scratch.i1522, i64 %iter.i2119.sroa.0.0.idx15359, !dbg !28954
  %525 = add i64 %iter.i2119.sroa.10.015361, -1, !dbg !28954
  %_7.i.i7290 = icmp eq i64 %iter.i2119.sroa.0.0.idx15359, 32, !dbg !28955
  br i1 %_7.i.i7290, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit, label %bb3.i2124, !dbg !28959

bb3.i2124:                                        ; preds = %bb32.i2122
  %iter.i2119.sroa.0.0.add = add nuw nsw i64 %iter.i2119.sroa.0.0.idx15359, 4, !dbg !28960
  %_9.0.i7294 = add nuw nsw i64 %iter.i2119.sroa.7.015360, 1, !dbg !28962
  %exitcond17982.not = icmp eq i64 %iter.i2119.sroa.7.015360, %_112.1.i, !dbg !28963
  br i1 %exitcond17982.not, label %panic.i, label %bb5.i2126, !dbg !28963

bb5.i2126:                                        ; preds = %bb3.i2124
  %526 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i, i64 %iter.i2119.sroa.7.015360, !dbg !28963
  %shape.i = load i32, ptr %526, align 4, !dbg !28963, !noalias !28964, !noundef !12
  %527 = getelementptr inbounds nuw i8, ptr %526, i64 4, !dbg !28963
  %shape3.i = load i32, ptr %527, align 4, !dbg !28963, !noalias !28964, !noundef !12
  %window.i = zext i32 %shape.i to i64, !dbg !28965
  %_19.i = zext i32 %shape3.i to i64, !dbg !28966
  %528 = add i64 %ring_cursor.sroa.0.1.i161915369, %_19.i, !dbg !28967
  %_20.not.i = icmp ult i64 %528, %_91.i1639, !dbg !28968
  %529 = select i1 %_20.not.i, i64 0, i64 %_91.i1639, !dbg !28968
  %spec.select.i = sub nuw i64 %528, %529, !dbg !28968
  %_27.i2127 = mul i64 %spec.select.i, %width.i, !dbg !28969
  %_26.i = add i64 %_27.i2127, %iter.i2119.sroa.7.015360, !dbg !28969
  %_30.i2128 = icmp ult i64 %_26.i, %_114.1.i, !dbg !28970
  br i1 %_30.i2128, label %bb12.i2129, label %panic5.i, !dbg !28970

panic.i:                                          ; preds = %bb3.i2124
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i, i64 noundef %_112.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4a8785a681d008a9bfd0cd82628ea9cb) #30, !dbg !28963, !noalias !28964
  unreachable, !dbg !28963

bb12.i2129:                                       ; preds = %bb5.i2126
  %530 = getelementptr inbounds nuw float, ptr %_114.0.i, i64 %_26.i, !dbg !28970
  %531 = load float, ptr %530, align 4, !dbg !28970, !noalias !28964, !noundef !12
  %exitcond17983.not = icmp eq i64 %iter.i2119.sroa.7.015360, %_116.1.i, !dbg !28971
  br i1 %exitcond17983.not, label %panic6.i, label %bb13.i2131, !dbg !28971

panic5.i:                                         ; preds = %bb5.i2126
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i, i64 noundef %_114.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cbce7773ac40979e4ba2385da3aec116) #30, !dbg !28970, !noalias !28964
  unreachable, !dbg !28970

bb13.i2131:                                       ; preds = %bb12.i2129
  %532 = getelementptr inbounds nuw i32, ptr %_116.0.i, i64 %iter.i2119.sroa.7.015360, !dbg !28971
  %_32.i2132 = load i32, ptr %532, align 4, !dbg !28971, !noalias !28964, !noundef !12
  %position.i2133 = zext i32 %_32.i2132 to i64, !dbg !28971
  %533 = icmp eq i32 %_32.i2132, 0, !dbg !28972
  br i1 %533, label %bb17.i, label %bb15.i2134, !dbg !28972

panic6.i:                                         ; preds = %bb12.i2129
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i, i64 noundef %_116.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ec0d48f73ebfc2755df5cedaa60b5c0a) #30, !dbg !28971, !noalias !28964
  unreachable, !dbg !28971

bb15.i2134:                                       ; preds = %bb13.i2131
  %_37.i = icmp ult i64 %iter.i2119.sroa.7.015360, %_118.1.i, !dbg !28973
  br i1 %_37.i, label %bb16.i2135, label %panic7.i, !dbg !28973

bb17.i:                                           ; preds = %bb35.i, %bb16.i2135, %bb13.i2131
  %newest.sroa.0.0.i = phi float [ %531, %bb13.i2131 ], [ %_35.i2136, %bb35.i ], [ %531, %bb16.i2135 ], !dbg !28974
  %exitcond17984.not = icmp eq i64 %iter.i2119.sroa.7.015360, %_118.1.i, !dbg !28975
  br i1 %exitcond17984.not, label %panic8.i, label %bb18.i, !dbg !28975

bb16.i2135:                                       ; preds = %bb15.i2134
  %534 = getelementptr inbounds nuw float, ptr %_118.0.i, i64 %iter.i2119.sroa.7.015360, !dbg !28973
  %_35.i2136 = load float, ptr %534, align 4, !dbg !28973, !noalias !28964, !noundef !12
  %_102.i2137 = fcmp olt float %_35.i2136, %531, !dbg !28976
  br i1 %_102.i2137, label %bb35.i, label %bb17.i, !dbg !28976

panic7.i:                                         ; preds = %bb15.i2134
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i2119.sroa.7.015360, i64 noundef %_118.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2c461872bb652d4796cdcf89c28c82c8) #30, !dbg !28973, !noalias !28964
  unreachable, !dbg !28973

bb35.i:                                           ; preds = %bb16.i2135
  br label %bb17.i, !dbg !28978

bb18.i:                                           ; preds = %bb17.i
  %535 = getelementptr inbounds nuw float, ptr %_118.0.i, i64 %iter.i2119.sroa.7.015360, !dbg !28975
  store float %newest.sroa.0.0.i, ptr %535, align 4, !dbg !28975, !noalias !28964
  %_42.i = add nuw nsw i64 %position.i2133, 1, !dbg !28979
  %complete.i2139 = icmp eq i64 %_42.i, %window.i, !dbg !28979
  br i1 %complete.i2139, label %bb22.i, label %bb20.i2140, !dbg !28980

panic8.i:                                         ; preds = %bb17.i
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i, i64 noundef %_118.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2b690e2c7763f11809942906fc2ca813) #30, !dbg !28975, !noalias !28964
  unreachable, !dbg !28975

bb20.i2140:                                       ; preds = %bb18.i
  %_44.i = add i64 %iter.i2119.sroa.7.015360, %_45.i2141, !dbg !28981
  %_47.i = icmp ult i64 %_44.i, %_114.1.i, !dbg !28982
  br i1 %_47.i, label %bb30.i, label %panic9.i, !dbg !28982

panic9.i:                                         ; preds = %bb20.i2140
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i, i64 noundef %_114.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9fa421ae81817f58fcfc4a3243223891) #30, !dbg !28982, !noalias !28964
  unreachable, !dbg !28982

bb30.i:                                           ; preds = %bb20.i2140
  %536 = getelementptr inbounds nuw float, ptr %_114.0.i, i64 %_44.i, !dbg !28982
  %_43.i = load float, ptr %536, align 4, !dbg !28982, !noalias !28964, !noundef !12
  %_103.i2142 = fcmp olt float %_43.i, %newest.sroa.0.0.i, !dbg !28983
  %newest.sroa.0.1.i = select i1 %_103.i2142, float %_43.i, float %newest.sroa.0.0.i, !dbg !28983
  store float %newest.sroa.0.1.i, ptr %iter.i2119.sroa.0.0.ptr15362, align 4, !dbg !28985, !noalias !28964
  %537 = trunc i64 %_42.i to i32, !dbg !28986
  br label %bb31.i2144, !dbg !28987

bb31.i2144:                                       ; preds = %bb25.i, %bb30.i
  %storemerge13414 = phi i32 [ %537, %bb30.i ], [ 0, %bb25.i ], !dbg !28988
  store i32 %storemerge13414, ptr %532, align 4, !dbg !28988, !noalias !28964
  %538 = icmp eq i64 %525, 0, !dbg !28952
  br i1 %538, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit, label %bb32.i2122, !dbg !28952

bb22.i:                                           ; preds = %bb18.i
  store float %newest.sroa.0.0.i, ptr %iter.i2119.sroa.0.0.ptr15362, align 4, !dbg !28985, !noalias !28964
  %539 = load float, ptr %530, align 4, !dbg !28989, !noalias !28964, !noundef !12
  br label %bb41.i2149, !dbg !28990

bb41.i2149:                                       ; preds = %bb22.i, %bb25.i
  %iter2.sroa.0.0.i15358 = phi i64 [ 0, %bb22.i ], [ %_105.i, %bb25.i ]
  %suffix.sroa.0.0.i15357 = phi float [ %539, %bb22.i ], [ %suffix.sroa.0.1.i, %bb25.i ]
  %end.sroa.0.1.i15356 = phi i64 [ %spec.select.i, %bb22.i ], [ %542, %bb25.i ]
  %_56.i = mul i64 %end.sroa.0.1.i15356, %width.i, !dbg !28993
  %_55.i = add i64 %_56.i, %iter.i2119.sroa.7.015360, !dbg !28993
  %_59.i = icmp ult i64 %_55.i, %_114.1.i, !dbg !28994
  br i1 %_59.i, label %bb25.i, label %panic13.i, !dbg !28994

panic13.i:                                        ; preds = %bb41.i2149
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i, i64 noundef %_114.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_91a4c6b9b17ebf4d863f9a70b6dc929a) #30, !dbg !28994, !noalias !28964
  unreachable, !dbg !28994

bb25.i:                                           ; preds = %bb41.i2149
  %_105.i = add nuw nsw i64 %iter2.sroa.0.0.i15358, 1, !dbg !28995
  %540 = getelementptr inbounds nuw float, ptr %_114.0.i, i64 %_55.i, !dbg !28994
  %_54.i = load float, ptr %540, align 4, !dbg !28994, !noalias !28964, !noundef !12
  %_107.i2150 = fcmp olt float %suffix.sroa.0.0.i15357, %_54.i, !dbg !28998
  %suffix.sroa.0.1.i = select i1 %_107.i2150, float %suffix.sroa.0.0.i15357, float %_54.i, !dbg !28998
  store float %suffix.sroa.0.1.i, ptr %540, align 4, !dbg !29000, !noalias !28964
  %541 = icmp eq i64 %end.sroa.0.1.i15356, 0, !dbg !29001
  %spec.store.select.i2151 = select i1 %541, i64 %_91.i1639, i64 %end.sroa.0.1.i15356, !dbg !29001
  %542 = add i64 %spec.store.select.i2151, -1, !dbg !29002
  %exitcond17981.not = icmp eq i64 %_105.i, %window.i, !dbg !29003
  br i1 %exitcond17981.not, label %bb31.i2144, label %bb41.i2149, !dbg !28990

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit: ; preds = %bb32.i2122, %bb31.i2144
  %lanes.i5863.sroa.0.0.copyload.pre = load <8 x float>, ptr %scratch.i1522, align 4, !dbg !29005, !alias.scope !29010, !noalias !29014
  br label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit, !dbg !29018

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit: ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6632
  %lanes.i5863.sroa.0.0.copyload = phi <8 x float> [ %lanes.i5863.sroa.0.0.copyload.pre, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.loopexit ], [ %lanes.i5888.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6632 ], !dbg !29005
  %543 = fmul <8 x float> %lanes.i5863.sroa.0.0.copyload, splat (float 1.638400e+04), !dbg !29019
  %544 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %543), !dbg !29024
  %545 = fmul <8 x float> %544, splat (float 0x3F10000000000000), !dbg !29029
  %546 = icmp eq i64 %width.i.i1702, 0, !dbg !29034
  %_149.1.i.i1737.pre = load i64, ptr %171, align 8, !dbg !29036, !alias.scope !28889, !noalias !28890
  br i1 %546, label %bb16.i.i1736, label %bb39.i.i1716.lr.ph, !dbg !29034

bb39.i.i1716.lr.ph:                               ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit
  %_145.1.i.i1719 = load i64, ptr %165, align 8, !alias.scope !28889, !noalias !28890, !noundef !12
  %_145.0.i.i1723 = load ptr, ptr %166, align 8, !nonnull !12
  %_147.0.i.i1734 = load ptr, ptr %172, align 8, !nonnull !12
  %exitcond17987.not = icmp eq i64 %_145.1.i.i1719, 0, !dbg !29037
  br i1 %exitcond17987.not, label %panic.i.i1721, label %bb17.i.i1722, !dbg !29037

bb37.i.i1769:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5877
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i1704, i64 noundef %_144.1.i.i1703, i64 noundef %_144.1.i.i1703, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_56df7c041d29359441bca272bf4e38e3) #30, !dbg !29038, !noalias !29039
  unreachable, !dbg !29038

bb16.i.i1736.loopexit:                            ; preds = %bb21.i.i1733.7, %bb21.i.i1733.6, %bb21.i.i1733.5, %bb21.i.i1733.4, %bb21.i.i1733.3, %bb21.i.i1733.2, %bb21.i.i1733.1, %bb21.i.i1733
  %lanes.i5856.sroa.0.0.copyload.pre = load <8 x float>, ptr %scratch.i1522, align 4, !dbg !29040, !alias.scope !29045, !noalias !29049
  br label %bb16.i.i1736, !dbg !29053

bb16.i.i1736:                                     ; preds = %bb16.i.i1736.loopexit, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit
  %lanes.i5856.sroa.0.0.copyload = phi <8 x float> [ %lanes.i5856.sroa.0.0.copyload.pre, %bb16.i.i1736.loopexit ], [ %lanes.i5863.sroa.0.0.copyload, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit ], !dbg !29040
  %547 = fadd <8 x float> %545, %_58.i.i1483.sroa.0.0.copyload20580, !dbg !29054
  %548 = fsub <8 x float> %547, %lanes.i5856.sroa.0.0.copyload, !dbg !29059
  store <8 x float> %548, ptr %173, align 32, !dbg !29064
  %_109.i.i1738 = icmp ugt i64 %_22.i.i1704, %_149.1.i.i1737.pre, !dbg !29065
  br i1 %_109.i.i1738, label %bb42.i.i1768, label %bb43.i.i1739, !dbg !29065, !prof !1406

bb43.i.i1739:                                     ; preds = %bb16.i.i1736
  %_112.i.i1741 = sub nuw i64 %_149.1.i.i1737.pre, %_22.i.i1704, !dbg !29068
  %_8.i6624 = icmp samesign ugt i64 %_112.i.i1741, 7, !dbg !29069
  br i1 %_8.i6624, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6627, label %bb2.i6625, !dbg !29069, !prof !1421

bb2.i6625:                                        ; preds = %bb43.i.i1739
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_112.i.i1741, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !29074, !noalias !29075
  unreachable, !dbg !29074

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6627: ; preds = %bb43.i.i1739
  %_149.0.i.i1740 = load ptr, ptr %172, align 8, !dbg !29036, !alias.scope !28889, !noalias !28890, !nonnull !12, !noundef !12
  %_116.i.i1742 = getelementptr inbounds nuw float, ptr %_149.0.i.i1740, i64 %_22.i.i1704, !dbg !29079
  store <8 x float> %545, ptr %_116.i.i1742, align 4, !dbg !29081, !alias.scope !29085, !noalias !29089
  %_64.i.i1480.sroa.0.0.copyload = load <8 x float>, ptr %174, align 32, !dbg !29091
  %_68.i.i1476.sroa.0.0.copyload = load <8 x float>, ptr %175, align 32, !dbg !29092
  %549 = fdiv <8 x float> %548, %_64.i.i1480.sroa.0.0.copyload, !dbg !29093
  %550 = fsub <8 x float> splat (float 1.000000e+00), %549, !dbg !29098
  %551 = fsub <8 x float> %550, %_68.i.i1476.sroa.0.0.copyload, !dbg !29103
  %552 = fmul <8 x float> %425, %551, !dbg !29108
  %553 = fadd <8 x float> %_68.i.i1476.sroa.0.0.copyload, %552, !dbg !29113
  %554 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %550, <8 x float> %553), !dbg !29117
  %555 = bitcast <8 x float> %554 to <8 x i32>, !dbg !29122
  %556 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %554), !dbg !29128
  %557 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %556, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !29130
  %558 = bitcast <8 x float> %557 to <8 x i32>, !dbg !29136
  %559 = xor <8 x i32> %558, splat (i32 -1), !dbg !29142
  %560 = and <8 x i32> %559, %555, !dbg !29144
  %561 = bitcast <8 x i32> %560 to <8 x float>, !dbg !29148
  store <8 x i32> %560, ptr %175, align 32, !dbg !29149
  %562 = fsub <8 x float> splat (float 1.000000e+00), %561, !dbg !29150
  %_150.1.i.i1743 = load i64, ptr %176, align 8, !dbg !29155, !alias.scope !28889, !noalias !28890, !noundef !12
  %_76.i.i1744 = mul i64 %width.i.i1702, %main_cursor.sroa.0.1.i162015370, !dbg !29156
  %_120.i.i1745 = icmp ugt i64 %_76.i.i1744, %_150.1.i.i1743, !dbg !29157
  br i1 %_120.i.i1745, label %bb48.i.i1767, label %bb49.i.i1746, !dbg !29157, !prof !1406

bb42.i.i1768:                                     ; preds = %bb16.i.i1736
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i1704, i64 noundef %_149.1.i.i1737.pre, i64 noundef %_149.1.i.i1737.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_90498045d73339daaf9e4f537508f58b) #30, !dbg !29160, !noalias !29161
  unreachable, !dbg !29160

bb49.i.i1746:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6627
  %_123.i.i1748 = sub nuw i64 %_150.1.i.i1743, %_76.i.i1744, !dbg !29162
  %_8.i5850 = icmp samesign ugt i64 %_123.i.i1748, 7, !dbg !29163
  br i1 %_8.i5850, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit, label %bb2.i5851, !dbg !29163, !prof !1421

bb2.i5851:                                        ; preds = %bb49.i.i1746
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_123.i.i1748, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !29168, !noalias !29169
  unreachable, !dbg !29168

bb48.i.i1767:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6627
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i.i1744, i64 noundef %_150.1.i.i1743, i64 noundef %_150.1.i.i1743, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_da8af254b6d507a8e2ca31e544bfd21d) #30, !dbg !29173, !noalias !29161
  unreachable, !dbg !29173

bb17.i.i1722:                                     ; preds = %bb39.i.i1716.lr.ph
  %563 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1723, i64 8, !dbg !29037
  %_44.i.i1724 = load i32, ptr %563, align 4, !dbg !29037, !noalias !29161, !noundef !12
  %_43.i.i1725 = zext i32 %_44.i.i1724 to i64, !dbg !29037
  %564 = add i64 %ring_cursor.sroa.0.1.i161915369, %_43.i.i1725, !dbg !29174
  %_47.not.i.i1726 = icmp ult i64 %564, %_91.i1639, !dbg !29175
  %565 = select i1 %_47.not.i.i1726, i64 0, i64 %_91.i1639, !dbg !29175
  %spec.select.i.i1727 = sub nuw i64 %564, %565, !dbg !29175
  %_51.i.i1728 = mul i64 %spec.select.i.i1727, %width.i.i1702, !dbg !29176
  %_53.i.i1731 = icmp ult i64 %_51.i.i1728, %_149.1.i.i1737.pre, !dbg !29177
  br i1 %_53.i.i1731, label %bb21.i.i1733, label %panic1.i.i1732, !dbg !29177

panic.i.i1721:                                    ; preds = %bb39.i.i1716.7, %bb39.i.i1716.6, %bb39.i.i1716.5, %bb39.i.i1716.4, %bb39.i.i1716.3, %bb39.i.i1716.2, %bb39.i.i1716.1, %bb39.i.i1716.lr.ph
  %_145.1.i.i1719.lcssa.ph = phi i64 [ 7, %bb39.i.i1716.7 ], [ 6, %bb39.i.i1716.6 ], [ 5, %bb39.i.i1716.5 ], [ 4, %bb39.i.i1716.4 ], [ 3, %bb39.i.i1716.3 ], [ 2, %bb39.i.i1716.2 ], [ 1, %bb39.i.i1716.1 ], [ 0, %bb39.i.i1716.lr.ph ]
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i.i1719.lcssa.ph, i64 noundef %_145.1.i.i1719.lcssa.ph, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a6feb40b34112df5f214f84424dd2c7c) #30, !dbg !29037, !noalias !29161
  unreachable, !dbg !29037

bb21.i.i1733:                                     ; preds = %bb17.i.i1722
  %566 = getelementptr inbounds nuw float, ptr %_147.0.i.i1734, i64 %_51.i.i1728, !dbg !29177
  %_49.i.i1735 = load float, ptr %566, align 4, !dbg !29177, !noalias !29161, !noundef !12
  store float %_49.i.i1735, ptr %scratch.i1522, align 4, !dbg !29178, !noalias !29161
  %567 = icmp eq i64 %width.i.i1702, 1, !dbg !29034
  br i1 %567, label %bb16.i.i1736.loopexit, label %bb39.i.i1716.1, !dbg !29034

bb39.i.i1716.1:                                   ; preds = %bb21.i.i1733
  %exitcond17987.1.not = icmp eq i64 %_145.1.i.i1719, 1, !dbg !29037
  br i1 %exitcond17987.1.not, label %panic.i.i1721, label %bb17.i.i1722.1, !dbg !29037

bb17.i.i1722.1:                                   ; preds = %bb39.i.i1716.1
  %568 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1723, i64 20, !dbg !29037
  %_44.i.i1724.1 = load i32, ptr %568, align 4, !dbg !29037, !noalias !29161, !noundef !12
  %_43.i.i1725.1 = zext i32 %_44.i.i1724.1 to i64, !dbg !29037
  %569 = add i64 %ring_cursor.sroa.0.1.i161915369, %_43.i.i1725.1, !dbg !29174
  %_47.not.i.i1726.1 = icmp ult i64 %569, %_91.i1639, !dbg !29175
  %570 = select i1 %_47.not.i.i1726.1, i64 0, i64 %_91.i1639, !dbg !29175
  %spec.select.i.i1727.1 = sub nuw i64 %569, %570, !dbg !29175
  %_51.i.i1728.1 = mul i64 %spec.select.i.i1727.1, %width.i.i1702, !dbg !29176
  %_50.i.i1729.1 = add i64 %_51.i.i1728.1, 1, !dbg !29176
  %_53.i.i1731.1 = icmp ult i64 %_50.i.i1729.1, %_149.1.i.i1737.pre, !dbg !29177
  br i1 %_53.i.i1731.1, label %bb21.i.i1733.1, label %panic1.i.i1732, !dbg !29177

bb21.i.i1733.1:                                   ; preds = %bb17.i.i1722.1
  %571 = getelementptr inbounds nuw float, ptr %_147.0.i.i1734, i64 %_50.i.i1729.1, !dbg !29177
  %_49.i.i1735.1 = load float, ptr %571, align 4, !dbg !29177, !noalias !29161, !noundef !12
  store float %_49.i.i1735.1, ptr %iter.i.i1486.sroa.0.0.ptr15366.1, align 4, !dbg !29178, !noalias !29161
  %572 = icmp eq i64 %width.i.i1702, 2, !dbg !29034
  br i1 %572, label %bb16.i.i1736.loopexit, label %bb39.i.i1716.2, !dbg !29034

bb39.i.i1716.2:                                   ; preds = %bb21.i.i1733.1
  %exitcond17987.2.not = icmp eq i64 %_145.1.i.i1719, 2, !dbg !29037
  br i1 %exitcond17987.2.not, label %panic.i.i1721, label %bb17.i.i1722.2, !dbg !29037

bb17.i.i1722.2:                                   ; preds = %bb39.i.i1716.2
  %573 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1723, i64 32, !dbg !29037
  %_44.i.i1724.2 = load i32, ptr %573, align 4, !dbg !29037, !noalias !29161, !noundef !12
  %_43.i.i1725.2 = zext i32 %_44.i.i1724.2 to i64, !dbg !29037
  %574 = add i64 %ring_cursor.sroa.0.1.i161915369, %_43.i.i1725.2, !dbg !29174
  %_47.not.i.i1726.2 = icmp ult i64 %574, %_91.i1639, !dbg !29175
  %575 = select i1 %_47.not.i.i1726.2, i64 0, i64 %_91.i1639, !dbg !29175
  %spec.select.i.i1727.2 = sub nuw i64 %574, %575, !dbg !29175
  %_51.i.i1728.2 = mul i64 %spec.select.i.i1727.2, %width.i.i1702, !dbg !29176
  %_50.i.i1729.2 = add i64 %_51.i.i1728.2, 2, !dbg !29176
  %_53.i.i1731.2 = icmp ult i64 %_50.i.i1729.2, %_149.1.i.i1737.pre, !dbg !29177
  br i1 %_53.i.i1731.2, label %bb21.i.i1733.2, label %panic1.i.i1732, !dbg !29177

bb21.i.i1733.2:                                   ; preds = %bb17.i.i1722.2
  %576 = getelementptr inbounds nuw float, ptr %_147.0.i.i1734, i64 %_50.i.i1729.2, !dbg !29177
  %_49.i.i1735.2 = load float, ptr %576, align 4, !dbg !29177, !noalias !29161, !noundef !12
  store float %_49.i.i1735.2, ptr %iter.i.i1486.sroa.0.0.ptr15366.2, align 4, !dbg !29178, !noalias !29161
  %577 = icmp eq i64 %width.i.i1702, 3, !dbg !29034
  br i1 %577, label %bb16.i.i1736.loopexit, label %bb39.i.i1716.3, !dbg !29034

bb39.i.i1716.3:                                   ; preds = %bb21.i.i1733.2
  %exitcond17987.3.not = icmp eq i64 %_145.1.i.i1719, 3, !dbg !29037
  br i1 %exitcond17987.3.not, label %panic.i.i1721, label %bb17.i.i1722.3, !dbg !29037

bb17.i.i1722.3:                                   ; preds = %bb39.i.i1716.3
  %578 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1723, i64 44, !dbg !29037
  %_44.i.i1724.3 = load i32, ptr %578, align 4, !dbg !29037, !noalias !29161, !noundef !12
  %_43.i.i1725.3 = zext i32 %_44.i.i1724.3 to i64, !dbg !29037
  %579 = add i64 %ring_cursor.sroa.0.1.i161915369, %_43.i.i1725.3, !dbg !29174
  %_47.not.i.i1726.3 = icmp ult i64 %579, %_91.i1639, !dbg !29175
  %580 = select i1 %_47.not.i.i1726.3, i64 0, i64 %_91.i1639, !dbg !29175
  %spec.select.i.i1727.3 = sub nuw i64 %579, %580, !dbg !29175
  %_51.i.i1728.3 = mul i64 %spec.select.i.i1727.3, %width.i.i1702, !dbg !29176
  %_50.i.i1729.3 = add i64 %_51.i.i1728.3, 3, !dbg !29176
  %_53.i.i1731.3 = icmp ult i64 %_50.i.i1729.3, %_149.1.i.i1737.pre, !dbg !29177
  br i1 %_53.i.i1731.3, label %bb21.i.i1733.3, label %panic1.i.i1732, !dbg !29177

bb21.i.i1733.3:                                   ; preds = %bb17.i.i1722.3
  %581 = getelementptr inbounds nuw float, ptr %_147.0.i.i1734, i64 %_50.i.i1729.3, !dbg !29177
  %_49.i.i1735.3 = load float, ptr %581, align 4, !dbg !29177, !noalias !29161, !noundef !12
  store float %_49.i.i1735.3, ptr %iter.i.i1486.sroa.0.0.ptr15366.3, align 4, !dbg !29178, !noalias !29161
  %582 = icmp eq i64 %width.i.i1702, 4, !dbg !29034
  br i1 %582, label %bb16.i.i1736.loopexit, label %bb39.i.i1716.4, !dbg !29034

bb39.i.i1716.4:                                   ; preds = %bb21.i.i1733.3
  %exitcond17987.4.not = icmp eq i64 %_145.1.i.i1719, 4, !dbg !29037
  br i1 %exitcond17987.4.not, label %panic.i.i1721, label %bb17.i.i1722.4, !dbg !29037

bb17.i.i1722.4:                                   ; preds = %bb39.i.i1716.4
  %583 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1723, i64 56, !dbg !29037
  %_44.i.i1724.4 = load i32, ptr %583, align 4, !dbg !29037, !noalias !29161, !noundef !12
  %_43.i.i1725.4 = zext i32 %_44.i.i1724.4 to i64, !dbg !29037
  %584 = add i64 %ring_cursor.sroa.0.1.i161915369, %_43.i.i1725.4, !dbg !29174
  %_47.not.i.i1726.4 = icmp ult i64 %584, %_91.i1639, !dbg !29175
  %585 = select i1 %_47.not.i.i1726.4, i64 0, i64 %_91.i1639, !dbg !29175
  %spec.select.i.i1727.4 = sub nuw i64 %584, %585, !dbg !29175
  %_51.i.i1728.4 = mul i64 %spec.select.i.i1727.4, %width.i.i1702, !dbg !29176
  %_50.i.i1729.4 = add i64 %_51.i.i1728.4, 4, !dbg !29176
  %_53.i.i1731.4 = icmp ult i64 %_50.i.i1729.4, %_149.1.i.i1737.pre, !dbg !29177
  br i1 %_53.i.i1731.4, label %bb21.i.i1733.4, label %panic1.i.i1732, !dbg !29177

bb21.i.i1733.4:                                   ; preds = %bb17.i.i1722.4
  %586 = getelementptr inbounds nuw float, ptr %_147.0.i.i1734, i64 %_50.i.i1729.4, !dbg !29177
  %_49.i.i1735.4 = load float, ptr %586, align 4, !dbg !29177, !noalias !29161, !noundef !12
  store float %_49.i.i1735.4, ptr %iter.i.i1486.sroa.0.0.ptr15366.4, align 4, !dbg !29178, !noalias !29161
  %587 = icmp eq i64 %width.i.i1702, 5, !dbg !29034
  br i1 %587, label %bb16.i.i1736.loopexit, label %bb39.i.i1716.5, !dbg !29034

bb39.i.i1716.5:                                   ; preds = %bb21.i.i1733.4
  %exitcond17987.5.not = icmp eq i64 %_145.1.i.i1719, 5, !dbg !29037
  br i1 %exitcond17987.5.not, label %panic.i.i1721, label %bb17.i.i1722.5, !dbg !29037

bb17.i.i1722.5:                                   ; preds = %bb39.i.i1716.5
  %588 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1723, i64 68, !dbg !29037
  %_44.i.i1724.5 = load i32, ptr %588, align 4, !dbg !29037, !noalias !29161, !noundef !12
  %_43.i.i1725.5 = zext i32 %_44.i.i1724.5 to i64, !dbg !29037
  %589 = add i64 %ring_cursor.sroa.0.1.i161915369, %_43.i.i1725.5, !dbg !29174
  %_47.not.i.i1726.5 = icmp ult i64 %589, %_91.i1639, !dbg !29175
  %590 = select i1 %_47.not.i.i1726.5, i64 0, i64 %_91.i1639, !dbg !29175
  %spec.select.i.i1727.5 = sub nuw i64 %589, %590, !dbg !29175
  %_51.i.i1728.5 = mul i64 %spec.select.i.i1727.5, %width.i.i1702, !dbg !29176
  %_50.i.i1729.5 = add i64 %_51.i.i1728.5, 5, !dbg !29176
  %_53.i.i1731.5 = icmp ult i64 %_50.i.i1729.5, %_149.1.i.i1737.pre, !dbg !29177
  br i1 %_53.i.i1731.5, label %bb21.i.i1733.5, label %panic1.i.i1732, !dbg !29177

bb21.i.i1733.5:                                   ; preds = %bb17.i.i1722.5
  %591 = getelementptr inbounds nuw float, ptr %_147.0.i.i1734, i64 %_50.i.i1729.5, !dbg !29177
  %_49.i.i1735.5 = load float, ptr %591, align 4, !dbg !29177, !noalias !29161, !noundef !12
  store float %_49.i.i1735.5, ptr %iter.i.i1486.sroa.0.0.ptr15366.5, align 4, !dbg !29178, !noalias !29161
  %592 = icmp eq i64 %width.i.i1702, 6, !dbg !29034
  br i1 %592, label %bb16.i.i1736.loopexit, label %bb39.i.i1716.6, !dbg !29034

bb39.i.i1716.6:                                   ; preds = %bb21.i.i1733.5
  %exitcond17987.6.not = icmp eq i64 %_145.1.i.i1719, 6, !dbg !29037
  br i1 %exitcond17987.6.not, label %panic.i.i1721, label %bb17.i.i1722.6, !dbg !29037

bb17.i.i1722.6:                                   ; preds = %bb39.i.i1716.6
  %593 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1723, i64 80, !dbg !29037
  %_44.i.i1724.6 = load i32, ptr %593, align 4, !dbg !29037, !noalias !29161, !noundef !12
  %_43.i.i1725.6 = zext i32 %_44.i.i1724.6 to i64, !dbg !29037
  %594 = add i64 %ring_cursor.sroa.0.1.i161915369, %_43.i.i1725.6, !dbg !29174
  %_47.not.i.i1726.6 = icmp ult i64 %594, %_91.i1639, !dbg !29175
  %595 = select i1 %_47.not.i.i1726.6, i64 0, i64 %_91.i1639, !dbg !29175
  %spec.select.i.i1727.6 = sub nuw i64 %594, %595, !dbg !29175
  %_51.i.i1728.6 = mul i64 %spec.select.i.i1727.6, %width.i.i1702, !dbg !29176
  %_50.i.i1729.6 = add i64 %_51.i.i1728.6, 6, !dbg !29176
  %_53.i.i1731.6 = icmp ult i64 %_50.i.i1729.6, %_149.1.i.i1737.pre, !dbg !29177
  br i1 %_53.i.i1731.6, label %bb21.i.i1733.6, label %panic1.i.i1732, !dbg !29177

bb21.i.i1733.6:                                   ; preds = %bb17.i.i1722.6
  %596 = getelementptr inbounds nuw float, ptr %_147.0.i.i1734, i64 %_50.i.i1729.6, !dbg !29177
  %_49.i.i1735.6 = load float, ptr %596, align 4, !dbg !29177, !noalias !29161, !noundef !12
  store float %_49.i.i1735.6, ptr %iter.i.i1486.sroa.0.0.ptr15366.6, align 4, !dbg !29178, !noalias !29161
  %597 = icmp eq i64 %width.i.i1702, 7, !dbg !29034
  br i1 %597, label %bb16.i.i1736.loopexit, label %bb39.i.i1716.7, !dbg !29034

bb39.i.i1716.7:                                   ; preds = %bb21.i.i1733.6
  %exitcond17987.7.not = icmp eq i64 %_145.1.i.i1719, 7, !dbg !29037
  br i1 %exitcond17987.7.not, label %panic.i.i1721, label %bb17.i.i1722.7, !dbg !29037

bb17.i.i1722.7:                                   ; preds = %bb39.i.i1716.7
  %598 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1723, i64 92, !dbg !29037
  %_44.i.i1724.7 = load i32, ptr %598, align 4, !dbg !29037, !noalias !29161, !noundef !12
  %_43.i.i1725.7 = zext i32 %_44.i.i1724.7 to i64, !dbg !29037
  %599 = add i64 %ring_cursor.sroa.0.1.i161915369, %_43.i.i1725.7, !dbg !29174
  %_47.not.i.i1726.7 = icmp ult i64 %599, %_91.i1639, !dbg !29175
  %600 = select i1 %_47.not.i.i1726.7, i64 0, i64 %_91.i1639, !dbg !29175
  %spec.select.i.i1727.7 = sub nuw i64 %599, %600, !dbg !29175
  %_51.i.i1728.7 = mul i64 %spec.select.i.i1727.7, %width.i.i1702, !dbg !29176
  %_50.i.i1729.7 = add i64 %_51.i.i1728.7, 7, !dbg !29176
  %_53.i.i1731.7 = icmp ult i64 %_50.i.i1729.7, %_149.1.i.i1737.pre, !dbg !29177
  br i1 %_53.i.i1731.7, label %bb21.i.i1733.7, label %panic1.i.i1732, !dbg !29177

bb21.i.i1733.7:                                   ; preds = %bb17.i.i1722.7
  %601 = getelementptr inbounds nuw float, ptr %_147.0.i.i1734, i64 %_50.i.i1729.7, !dbg !29177
  %_49.i.i1735.7 = load float, ptr %601, align 4, !dbg !29177, !noalias !29161, !noundef !12
  store float %_49.i.i1735.7, ptr %iter.i.i1486.sroa.0.0.ptr15366.7, align 4, !dbg !29178, !noalias !29161
  br label %bb16.i.i1736.loopexit, !dbg !29034

panic1.i.i1732:                                   ; preds = %bb17.i.i1722.7, %bb17.i.i1722.6, %bb17.i.i1722.5, %bb17.i.i1722.4, %bb17.i.i1722.3, %bb17.i.i1722.2, %bb17.i.i1722.1, %bb17.i.i1722
  %_50.i.i1729.lcssa.ph = phi i64 [ %_50.i.i1729.7, %bb17.i.i1722.7 ], [ %_50.i.i1729.6, %bb17.i.i1722.6 ], [ %_50.i.i1729.5, %bb17.i.i1722.5 ], [ %_50.i.i1729.4, %bb17.i.i1722.4 ], [ %_50.i.i1729.3, %bb17.i.i1722.3 ], [ %_50.i.i1729.2, %bb17.i.i1722.2 ], [ %_50.i.i1729.1, %bb17.i.i1722.1 ], [ %_51.i.i1728, %bb17.i.i1722 ]
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i.i1729.lcssa.ph, i64 noundef %_149.1.i.i1737.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b305c1483509cfb31fdec21ff8752674) #30, !dbg !29177, !noalias !29161
  unreachable, !dbg !29177

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit: ; preds = %bb49.i.i1746
  %_150.0.i.i1747 = load ptr, ptr %177, align 8, !dbg !29155, !alias.scope !28889, !noalias !28890, !nonnull !12, !noundef !12
  %_127.i.i1749 = getelementptr inbounds nuw float, ptr %_150.0.i.i1747, i64 %_76.i.i1744, !dbg !29179
  %lanes.i5847.sroa.0.0.copyload = load <8 x float>, ptr %_127.i.i1749, align 4, !dbg !29181, !alias.scope !29185, !noalias !29189
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_127.i.i1749, ptr noundef nonnull align 4 dereferenceable(32) %_181.i1701, i64 32, i1 false), !dbg !29191
  %602 = fmul <8 x float> %562, %lanes.i5847.sroa.0.0.copyload, !dbg !29196
  %603 = select <8 x i1> %161, <8 x float> %lanes.i5847.sroa.0.0.copyload, <8 x float> %602, !dbg !29201
  store <8 x float> %603, ptr %_181.i1701, align 4, !dbg !29206, !alias.scope !29211, !noalias !29215
  %604 = add i64 %main_cursor.sroa.0.1.i162015370, 1, !dbg !29219
  %_106.i1760 = load i64, ptr %178, align 8, !dbg !29220, !alias.scope !27042, !noalias !28526, !noundef !12
  %_104.i1761 = icmp eq i64 %604, %_106.i1760, !dbg !29221
  %spec.store.select.i1762 = select i1 %_104.i1761, i64 0, i64 %604, !dbg !29221
  %605 = add i64 %ring_cursor.sroa.0.1.i161915369, 1, !dbg !29222
  %_107.i1763 = icmp eq i64 %605, %_91.i1639, !dbg !29223
  %spec.store.select13.i1764 = select i1 %_107.i1763, i64 0, i64 %605, !dbg !29223
  %exitcond17992.not = icmp eq i64 %418, %umax17991, !dbg !29224
  br i1 %exitcond17992.not, label %bb13.i1545.loopexit.loopexit, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5927, !dbg !28281

bb54.i1770:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6637
  store <8 x float> %395, ptr %131, align 32, !dbg !27101
  store <8 x float> %400, ptr %_68.i1625, align 32, !dbg !27111
  store <8 x float> %401, ptr %132, align 32, !dbg !27112
  store <8 x float> %403, ptr %134, align 32, !dbg !27113
  store <8 x float> %408, ptr %_69.i1626, align 32, !dbg !27115
  store <8 x float> %409, ptr %135, align 32, !dbg !27116
  store <8 x float> %411, ptr %137, align 32, !dbg !27117
  store <8 x float> %416, ptr %_73.i1627, align 32, !dbg !27121
  store <8 x float> %417, ptr %138, align 32, !dbg !27122
  store <8 x float> %420, ptr %140, align 32, !dbg !27123
  store <8 x float> %425, ptr %_74.i1628, align 32, !dbg !27125
  store <8 x float> %426, ptr %141, align 32, !dbg !27126
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1624, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_54f0ebc64763f3f022885a8e201aab88) #30, !dbg !29227, !noalias !27149
  unreachable, !dbg !29227

_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit: ; preds = %bb13.i1545.loopexit
  %606 = trunc i64 %main_cursor.sroa.0.1.i1620.lcssa to i32, !dbg !29228
  %607 = trunc i64 %ring_cursor.sroa.0.1.i1619.lcssa to i32, !dbg !29229
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, !dbg !29230

_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit, %bb12.i
  %ring_cursor.sroa.0.0.i1548.lcssa = phi i32 [ %_36.i1537, %bb12.i ], [ %607, %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit ], !dbg !27068
  %main_cursor.sroa.0.0.i1549.lcssa = phi i32 [ %_34.i1536, %bb12.i ], [ %606, %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit ], !dbg !27065
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_left.i1529, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33) #31, !dbg !29231
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_right.i1528, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34) #31, !dbg !29232
  store i32 %main_cursor.sroa.0.0.i1549.lcssa, ptr %_35, align 4, !dbg !29228, !alias.scope !27048, !noalias !27067
  store i32 %ring_cursor.sroa.0.0.i1548.lcssa, ptr %81, align 4, !dbg !29229, !alias.scope !27048, !noalias !27067
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i1520), !dbg !29233, !noalias !27072
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i1521), !dbg !29234, !noalias !27072
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i1522), !dbg !29235, !noalias !27072
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, !dbg !27041

bb11.i:                                           ; preds = %bb10.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !29236), !dbg !29239
  tail call void @llvm.experimental.noalias.scope.decl(metadata !29240), !dbg !29239
  tail call void @llvm.experimental.noalias.scope.decl(metadata !29242), !dbg !29239
  tail call void @llvm.experimental.noalias.scope.decl(metadata !29244), !dbg !29239
  tail call void @llvm.experimental.noalias.scope.decl(metadata !29246), !dbg !29239
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_left.i988, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #31, !dbg !29248
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_right.i987, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #31, !dbg !29252
  %608 = load i8, ptr %79, align 32, !dbg !29254, !range !17, !alias.scope !29236, !noalias !29258, !noundef !12
  %609 = load i8, ptr %80, align 1, !dbg !29261, !range !17, !alias.scope !29236, !noalias !29258, !noundef !12
  %_34.i = load i32, ptr %_35, align 4, !dbg !29263, !alias.scope !29246, !noalias !29265, !noundef !12
  %_36.i995 = load i32, ptr %81, align 4, !dbg !29266, !alias.scope !29246, !noalias !29265, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i), !dbg !29268, !noalias !29270
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i, i8 0, i64 32, i1 false), !noalias !29270
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i981), !dbg !29271, !noalias !29270
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i981, i8 0, i64 1024, i1 false), !noalias !29270
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i980), !dbg !29273, !noalias !29270
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i980, i8 0, i64 1024, i1 false), !noalias !29270
  br i1 %_116.not.i15609, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, label %bb37.i.lr.ph, !dbg !29275

bb37.i.lr.ph:                                     ; preds = %bb11.i
  %610 = zext i32 %_36.i995 to i64, !dbg !29266
  %611 = zext i32 %_34.i to i64, !dbg !29263
  %_32.i992 = trunc nuw i8 %609 to i1, !dbg !29261
  %_31.i989 = trunc nuw i8 %608 to i1, !dbg !29254
  %612 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !29285
  %613 = bitcast <8 x float> %612 to <8 x i32>, !dbg !29291
  %614 = xor <8 x i32> %613, splat (i32 -1), !dbg !29297
  %history.i315.i.sroa.10.0.hot_left.i988.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i988, i64 32
  %history.i315.i.sroa.13.0.hot_left.i988.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i988, i64 64
  %history.i315.i.sroa.16.0.hot_left.i988.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i988, i64 96
  %history.i315.i.sroa.19.0.hot_left.i988.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i988, i64 128
  %history.i315.i.sroa.22.0.hot_left.i988.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i988, i64 160
  %history.i315.i.sroa.25.0.hot_left.i988.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i988, i64 192
  %history.i315.i.sroa.29.0.hot_left.i988.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i988, i64 224
  %history.i315.i.sroa.32.0.hot_left.i988.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i988, i64 256
  %history.i315.i.sroa.35.0.hot_left.i988.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i988, i64 288
  %history.i315.i.sroa.38.0.hot_left.i988.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i988, i64 320
  %history.i315.i.sroa.41.0.hot_left.i988.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i988, i64 352
  %615 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %616 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %617 = getelementptr inbounds nuw i8, ptr %self, i64 96
  %row12.i.i.i322.i = getelementptr inbounds nuw i8, ptr %self, i64 128
  %618 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %619 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %620 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %row13.i.i.i323.i = getelementptr inbounds nuw i8, ptr %self, i64 256
  %621 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %622 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %623 = getelementptr inbounds nuw i8, ptr %self, i64 352
  %row14.i.i.i324.i = getelementptr inbounds nuw i8, ptr %self, i64 384
  %624 = getelementptr inbounds nuw i8, ptr %self, i64 416
  %625 = getelementptr inbounds nuw i8, ptr %self, i64 448
  %626 = getelementptr inbounds nuw i8, ptr %self, i64 480
  %row15.i.i.i325.i = getelementptr inbounds nuw i8, ptr %self, i64 512
  %627 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %628 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %629 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %row16.i.i.i326.i = getelementptr inbounds nuw i8, ptr %self, i64 640
  %630 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %631 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %632 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %row17.i.i.i327.i = getelementptr inbounds nuw i8, ptr %self, i64 768
  %633 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %634 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %635 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %row18.i.i.i328.i = getelementptr inbounds nuw i8, ptr %self, i64 896
  %636 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %637 = getelementptr inbounds nuw i8, ptr %self, i64 960
  %638 = getelementptr inbounds nuw i8, ptr %self, i64 992
  %row19.i.i.i329.i = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %639 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %640 = getelementptr inbounds nuw i8, ptr %self, i64 1088
  %641 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %row20.i.i.i330.i = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %642 = getelementptr inbounds nuw i8, ptr %self, i64 1184
  %643 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %644 = getelementptr inbounds nuw i8, ptr %self, i64 1248
  %row21.i.i.i331.i = getelementptr inbounds nuw i8, ptr %self, i64 1280
  %645 = getelementptr inbounds nuw i8, ptr %self, i64 1312
  %646 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %647 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %row22.i.i.i332.i = getelementptr inbounds nuw i8, ptr %self, i64 1408
  %648 = getelementptr inbounds nuw i8, ptr %self, i64 1440
  %649 = getelementptr inbounds nuw i8, ptr %self, i64 1472
  %650 = getelementptr inbounds nuw i8, ptr %self, i64 1504
  %history.i.i952.sroa.10.0.hot_right.i987.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i987, i64 32
  %history.i.i952.sroa.13.0.hot_right.i987.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i987, i64 64
  %history.i.i952.sroa.16.0.hot_right.i987.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i987, i64 96
  %history.i.i952.sroa.19.0.hot_right.i987.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i987, i64 128
  %history.i.i952.sroa.22.0.hot_right.i987.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i987, i64 160
  %history.i.i952.sroa.25.0.hot_right.i987.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i987, i64 192
  %history.i.i952.sroa.29.0.hot_right.i987.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i987, i64 224
  %history.i.i952.sroa.32.0.hot_right.i987.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i987, i64 256
  %history.i.i952.sroa.35.0.hot_right.i987.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i987, i64 288
  %history.i.i952.sroa.38.0.hot_right.i987.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i987, i64 320
  %history.i.i952.sroa.41.0.hot_right.i987.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i987, i64 352
  %_68.i1035 = getelementptr inbounds nuw i8, ptr %hot_left.i988, i64 384
  %_69.i = getelementptr inbounds nuw i8, ptr %hot_left.i988, i64 512
  %_73.i = getelementptr inbounds nuw i8, ptr %hot_right.i987, i64 384
  %_74.i1036 = getelementptr inbounds nuw i8, ptr %hot_right.i987, i64 512
  %651 = select i1 %_31.i989, <8 x i32> %613, <8 x i32> %614
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
  %663 = getelementptr inbounds nuw i8, ptr %hot_left.i988, i64 672
  %664 = getelementptr inbounds nuw i8, ptr %hot_left.i988, i64 704
  %665 = getelementptr inbounds nuw i8, ptr %hot_left.i988, i64 640
  %666 = getelementptr inbounds nuw i8, ptr %self, i64 1672
  %667 = getelementptr inbounds nuw i8, ptr %self, i64 1664
  %668 = select i1 %_32.i992, <8 x i32> %613, <8 x i32> %614
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
  %681 = getelementptr inbounds nuw i8, ptr %hot_right.i987, i64 672
  %682 = getelementptr inbounds nuw i8, ptr %hot_right.i987, i64 704
  %683 = getelementptr inbounds nuw i8, ptr %hot_right.i987, i64 640
  %684 = getelementptr inbounds nuw i8, ptr %self, i64 1872
  %685 = getelementptr inbounds nuw i8, ptr %self, i64 1864
  %686 = getelementptr inbounds nuw i8, ptr %self, i64 1632
  %iter.i49.i.sroa.0.0.ptr15444.1 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 4
  %iter.i49.i.sroa.0.0.ptr15444.2 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 8
  %iter.i49.i.sroa.0.0.ptr15444.3 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 12
  %iter.i49.i.sroa.0.0.ptr15444.4 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 16
  %iter.i49.i.sroa.0.0.ptr15444.5 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 20
  %iter.i49.i.sroa.0.0.ptr15444.6 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 24
  %iter.i49.i.sroa.0.0.ptr15444.7 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 28
  %iter.i.i958.sroa.0.0.ptr15455.1 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 4
  %iter.i.i958.sroa.0.0.ptr15455.2 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 8
  %iter.i.i958.sroa.0.0.ptr15455.3 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 12
  %iter.i.i958.sroa.0.0.ptr15455.4 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 16
  %iter.i.i958.sroa.0.0.ptr15455.5 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 20
  %iter.i.i958.sroa.0.0.ptr15455.6 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 24
  %iter.i.i958.sroa.0.0.ptr15455.7 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 28
  br label %bb37.i, !dbg !29275

bb16.i.bb13.i.loopexit_crit_edge:                 ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6667
  store <8 x float> %937, ptr %663, align 32, !dbg !29299
  store <8 x float> %1024, ptr %681, align 32, !dbg !29315
  br label %bb13.i.loopexit, !dbg !29317

bb13.i.loopexit:                                  ; preds = %bb16.i.bb13.i.loopexit_crit_edge, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1030
  %ring_cursor.sroa.0.1.i1032.lcssa = phi i64 [ %spec.store.select13.i, %bb16.i.bb13.i.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i100115612, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1030 ], !dbg !29323
  %main_cursor.sroa.0.1.i1033.lcssa = phi i64 [ %spec.store.select.i, %bb16.i.bb13.i.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i100215613, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1030 ], !dbg !29324
  %_116.not.i = icmp eq i64 %689, 0, !dbg !29275
  %indvars.iv.next17996 = add nsw i64 %indvars.iv17995, -32, !dbg !29275
  br i1 %_116.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit, label %bb37.i, !dbg !29275

bb37.i:                                           ; preds = %bb37.i.lr.ph, %bb13.i.loopexit
  %indvars.iv17995 = phi i64 [ %frames, %bb37.i.lr.ph ], [ %indvars.iv.next17996, %bb13.i.loopexit ]
  %main_cursor.sroa.0.0.i100215613 = phi i64 [ %611, %bb37.i.lr.ph ], [ %main_cursor.sroa.0.1.i1033.lcssa, %bb13.i.loopexit ]
  %ring_cursor.sroa.0.0.i100115612 = phi i64 [ %610, %bb37.i.lr.ph ], [ %ring_cursor.sroa.0.1.i1032.lcssa, %bb13.i.loopexit ]
  %iter4.sroa.0.0.i100015611 = phi i64 [ %yield_count.sroa.0.0.i.i7319, %bb37.i.lr.ph ], [ %689, %bb13.i.loopexit ]
  %iter.sroa.0.0.i15610 = phi i64 [ 0, %bb37.i.lr.ph ], [ %688, %bb13.i.loopexit ]
  %687 = call i64 @llvm.umax.i64(i64 %indvars.iv17995, i64 1), !dbg !29325
  %umax18028 = call i64 @llvm.umin.i64(i64 %687, i64 32), !dbg !29325
  %688 = add nuw nsw i64 %iter.sroa.0.0.i15610, 32, !dbg !29325
  %689 = add nsw i64 %iter4.sroa.0.0.i100015611, -1, !dbg !29329
  %_45.i = sub nsw i64 %frames, %iter.sroa.0.0.i15610, !dbg !29330
  %..i7320 = tail call noundef i64 @llvm.umin.i64(i64 %_45.i, i64 32), !dbg !29331
  %active_base.i1005 = shl i64 %iter.sroa.0.0.i15610, 3, !dbg !29335
  %active_base.i100513423 = add nuw i64 %..i7320, %iter.sroa.0.0.i15610, !dbg !29336
  %_51.i = shl i64 %active_base.i100513423, 3, !dbg !29336
  %_128.i = icmp samesign ult i64 %_51.i, %active_base.i1005, !dbg !29337
  %_122.not.i = icmp ugt i64 %_51.i, %left_io.1
  %or.cond.i1007 = or i1 %_128.i, %_122.not.i, !dbg !29337
  br i1 %or.cond.i1007, label %bb41.i, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7350, !dbg !29337, !prof !165

bb41.i:                                           ; preds = %bb37.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %active_base.i1005, i64 noundef %_51.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5cbcacccdbb7b907c55dbc42154fcf08) #30, !dbg !29344, !noalias !29345
  unreachable, !dbg !29344

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7350: ; preds = %bb37.i
  %_131.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %active_base.i1005, !dbg !29346
  %history.i315.i.sroa.0.0.copyload = load <8 x float>, ptr %hot_left.i988, align 32, !dbg !29350
  %history.i315.i.sroa.10.sroa.0.0.copyload = load <8 x float>, ptr %history.i315.i.sroa.10.0.hot_left.i988.sroa_idx, align 32, !dbg !29350
  %history.i315.i.sroa.13.sroa.0.0.copyload = load <8 x float>, ptr %history.i315.i.sroa.13.0.hot_left.i988.sroa_idx, align 32, !dbg !29350
  %history.i315.i.sroa.16.sroa.0.0.copyload = load <8 x float>, ptr %history.i315.i.sroa.16.0.hot_left.i988.sroa_idx, align 32, !dbg !29350
  %history.i315.i.sroa.19.sroa.0.0.copyload = load <8 x float>, ptr %history.i315.i.sroa.19.0.hot_left.i988.sroa_idx, align 32, !dbg !29350
  %history.i315.i.sroa.22.sroa.0.0.copyload = load <8 x float>, ptr %history.i315.i.sroa.22.0.hot_left.i988.sroa_idx, align 32, !dbg !29350
  %history.i315.i.sroa.25.sroa.0.0.copyload = load <8 x float>, ptr %history.i315.i.sroa.25.0.hot_left.i988.sroa_idx, align 32, !dbg !29350
  %history.i315.i.sroa.29.sroa.0.0.copyload = load <8 x float>, ptr %history.i315.i.sroa.29.0.hot_left.i988.sroa_idx, align 32, !dbg !29350
  %history.i315.i.sroa.32.sroa.0.0.copyload = load <8 x float>, ptr %history.i315.i.sroa.32.0.hot_left.i988.sroa_idx, align 32, !dbg !29350
  %history.i315.i.sroa.35.sroa.0.0.copyload = load <8 x float>, ptr %history.i315.i.sroa.35.0.hot_left.i988.sroa_idx, align 32, !dbg !29350
  %history.i315.i.sroa.38.sroa.0.0.copyload = load <8 x float>, ptr %history.i315.i.sroa.38.0.hot_left.i988.sroa_idx, align 32, !dbg !29350
  %history.i315.i.sroa.41.sroa.0.0.copyload = load <8 x float>, ptr %history.i315.i.sroa.41.0.hot_left.i988.sroa_idx, align 32, !dbg !29350
  %_2.i735315382.not = icmp eq i64 %frames, %iter.sroa.0.0.i15610, !dbg !29352
  br i1 %_2.i735315382.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit339.i, label %bb6.i318.i.lr.ph, !dbg !29352

bb6.i318.i.lr.ph:                                 ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7350
  %_5.i4945 = load <8 x float>, ptr %self, align 32, !alias.scope !29355, !noalias !29358
  %_14.i.i.i275.i.sroa.0.0.copyload = load <8 x float>, ptr %615, align 32, !noalias !29370
  %_17.i.i.i272.i.sroa.0.0.copyload = load <8 x float>, ptr %616, align 32, !noalias !29370
  %_20.i.i.i269.i.sroa.0.0.copyload = load <8 x float>, ptr %617, align 32, !noalias !29370
  %_25.i.i.i265.i.sroa.0.0.copyload = load <8 x float>, ptr %row12.i.i.i322.i, align 32, !noalias !29370
  %_28.i.i.i262.i.sroa.0.0.copyload = load <8 x float>, ptr %618, align 32, !noalias !29370
  %_31.i.i.i259.i.sroa.0.0.copyload = load <8 x float>, ptr %619, align 32, !noalias !29370
  %_34.i.i.i256.i.sroa.0.0.copyload = load <8 x float>, ptr %620, align 32, !noalias !29370
  %_39.i.i.i252.i.sroa.0.0.copyload = load <8 x float>, ptr %row13.i.i.i323.i, align 32, !noalias !29370
  %_42.i.i.i249.i.sroa.0.0.copyload = load <8 x float>, ptr %621, align 32, !noalias !29370
  %_45.i.i.i246.i.sroa.0.0.copyload = load <8 x float>, ptr %622, align 32, !noalias !29370
  %_48.i.i.i243.i.sroa.0.0.copyload = load <8 x float>, ptr %623, align 32, !noalias !29370
  %_53.i.i.i239.i.sroa.0.0.copyload = load <8 x float>, ptr %row14.i.i.i324.i, align 32, !noalias !29370
  %_56.i.i.i236.i.sroa.0.0.copyload = load <8 x float>, ptr %624, align 32, !noalias !29370
  %_59.i.i.i233.i.sroa.0.0.copyload = load <8 x float>, ptr %625, align 32, !noalias !29370
  %_62.i.i.i230.i.sroa.0.0.copyload = load <8 x float>, ptr %626, align 32, !noalias !29370
  %_67.i.i.i226.i.sroa.0.0.copyload = load <8 x float>, ptr %row15.i.i.i325.i, align 32, !noalias !29370
  %_70.i.i.i223.i.sroa.0.0.copyload = load <8 x float>, ptr %627, align 32, !noalias !29370
  %_73.i.i.i220.i.sroa.0.0.copyload = load <8 x float>, ptr %628, align 32, !noalias !29370
  %_76.i.i.i217.i.sroa.0.0.copyload = load <8 x float>, ptr %629, align 32, !noalias !29370
  %_81.i.i.i213.i.sroa.0.0.copyload = load <8 x float>, ptr %row16.i.i.i326.i, align 32, !noalias !29370
  %_84.i.i.i210.i.sroa.0.0.copyload = load <8 x float>, ptr %630, align 32, !noalias !29370
  %_87.i.i.i207.i.sroa.0.0.copyload = load <8 x float>, ptr %631, align 32, !noalias !29370
  %_90.i.i.i204.i.sroa.0.0.copyload = load <8 x float>, ptr %632, align 32, !noalias !29370
  %_95.i.i.i200.i.sroa.0.0.copyload = load <8 x float>, ptr %row17.i.i.i327.i, align 32, !noalias !29370
  %_98.i.i.i197.i.sroa.0.0.copyload = load <8 x float>, ptr %633, align 32, !noalias !29370
  %_101.i.i.i194.i.sroa.0.0.copyload = load <8 x float>, ptr %634, align 32, !noalias !29370
  %_104.i.i.i191.i.sroa.0.0.copyload = load <8 x float>, ptr %635, align 32, !noalias !29370
  %_109.i.i.i187.i.sroa.0.0.copyload = load <8 x float>, ptr %row18.i.i.i328.i, align 32, !noalias !29370
  %_112.i.i.i184.i.sroa.0.0.copyload = load <8 x float>, ptr %636, align 32, !noalias !29370
  %_115.i.i.i181.i.sroa.0.0.copyload = load <8 x float>, ptr %637, align 32, !noalias !29370
  %_118.i.i.i178.i.sroa.0.0.copyload = load <8 x float>, ptr %638, align 32, !noalias !29370
  %_123.i.i.i174.i.sroa.0.0.copyload = load <8 x float>, ptr %row19.i.i.i329.i, align 32, !noalias !29370
  %_126.i.i.i171.i.sroa.0.0.copyload = load <8 x float>, ptr %639, align 32, !noalias !29370
  %_129.i.i.i168.i.sroa.0.0.copyload = load <8 x float>, ptr %640, align 32, !noalias !29370
  %_132.i.i.i165.i.sroa.0.0.copyload = load <8 x float>, ptr %641, align 32, !noalias !29370
  %_137.i.i.i161.i.sroa.0.0.copyload = load <8 x float>, ptr %row20.i.i.i330.i, align 32, !noalias !29370
  %_140.i.i.i158.i.sroa.0.0.copyload = load <8 x float>, ptr %642, align 32, !noalias !29370
  %_143.i.i.i155.i.sroa.0.0.copyload = load <8 x float>, ptr %643, align 32, !noalias !29370
  %_146.i.i.i152.i.sroa.0.0.copyload = load <8 x float>, ptr %644, align 32, !noalias !29370
  %_151.i.i.i148.i.sroa.0.0.copyload = load <8 x float>, ptr %row21.i.i.i331.i, align 32, !noalias !29370
  %_154.i.i.i145.i.sroa.0.0.copyload = load <8 x float>, ptr %645, align 32, !noalias !29370
  %_157.i.i.i142.i.sroa.0.0.copyload = load <8 x float>, ptr %646, align 32, !noalias !29370
  %_160.i.i.i139.i.sroa.0.0.copyload = load <8 x float>, ptr %647, align 32, !noalias !29370
  %_165.i.i.i135.i.sroa.0.0.copyload = load <8 x float>, ptr %row22.i.i.i332.i, align 32, !noalias !29370
  %_168.i.i.i132.i.sroa.0.0.copyload = load <8 x float>, ptr %648, align 32, !noalias !29370
  %_171.i.i.i129.i.sroa.0.0.copyload = load <8 x float>, ptr %649, align 32, !noalias !29370
  %_174.i.i.i126.i.sroa.0.0.copyload = load <8 x float>, ptr %650, align 32, !noalias !29370
  br label %bb6.i318.i, !dbg !29352

bb6.i318.i:                                       ; preds = %bb6.i318.i.lr.ph, %bb6.i318.i
  %history.i315.i.sroa.10.sroa.0.015394 = phi <8 x float> [ %history.i315.i.sroa.10.sroa.0.0.copyload, %bb6.i318.i.lr.ph ], [ %history.i315.i.sroa.0.015383, %bb6.i318.i ]
  %history.i315.i.sroa.13.sroa.0.015393 = phi <8 x float> [ %history.i315.i.sroa.13.sroa.0.0.copyload, %bb6.i318.i.lr.ph ], [ %history.i315.i.sroa.10.sroa.0.015394, %bb6.i318.i ]
  %history.i315.i.sroa.16.sroa.0.015392 = phi <8 x float> [ %history.i315.i.sroa.16.sroa.0.0.copyload, %bb6.i318.i.lr.ph ], [ %history.i315.i.sroa.13.sroa.0.015393, %bb6.i318.i ]
  %history.i315.i.sroa.19.sroa.0.015391 = phi <8 x float> [ %history.i315.i.sroa.19.sroa.0.0.copyload, %bb6.i318.i.lr.ph ], [ %history.i315.i.sroa.16.sroa.0.015392, %bb6.i318.i ]
  %history.i315.i.sroa.22.sroa.0.015390 = phi <8 x float> [ %history.i315.i.sroa.22.sroa.0.0.copyload, %bb6.i318.i.lr.ph ], [ %history.i315.i.sroa.19.sroa.0.015391, %bb6.i318.i ]
  %history.i315.i.sroa.38.sroa.0.015389 = phi <8 x float> [ %history.i315.i.sroa.38.sroa.0.0.copyload, %bb6.i318.i.lr.ph ], [ %history.i315.i.sroa.35.sroa.0.015388, %bb6.i318.i ]
  %history.i315.i.sroa.35.sroa.0.015388 = phi <8 x float> [ %history.i315.i.sroa.35.sroa.0.0.copyload, %bb6.i318.i.lr.ph ], [ %history.i315.i.sroa.32.sroa.0.015387, %bb6.i318.i ]
  %history.i315.i.sroa.32.sroa.0.015387 = phi <8 x float> [ %history.i315.i.sroa.32.sroa.0.0.copyload, %bb6.i318.i.lr.ph ], [ %history.i315.i.sroa.29.sroa.0.015386, %bb6.i318.i ]
  %history.i315.i.sroa.29.sroa.0.015386 = phi <8 x float> [ %history.i315.i.sroa.29.sroa.0.0.copyload, %bb6.i318.i.lr.ph ], [ %history.i315.i.sroa.25.sroa.0.015385, %bb6.i318.i ]
  %history.i315.i.sroa.25.sroa.0.015385 = phi <8 x float> [ %history.i315.i.sroa.25.sroa.0.0.copyload, %bb6.i318.i.lr.ph ], [ %history.i315.i.sroa.22.sroa.0.015390, %bb6.i318.i ]
  %iter.i311.i.sroa.16.015384 = phi i64 [ 0, %bb6.i318.i.lr.ph ], [ %795, %bb6.i318.i ]
  %history.i315.i.sroa.0.015383 = phi <8 x float> [ %history.i315.i.sroa.0.0.copyload, %bb6.i318.i.lr.ph ], [ %lanes.i6038.sroa.0.0.copyload, %bb6.i318.i ]
  %start1.i.i7359 = shl i64 %iter.i311.i.sroa.16.015384, 3, !dbg !29373
  %data.i.i7360 = getelementptr inbounds nuw float, ptr %_131.i, i64 %start1.i.i7359, !dbg !29375
  %lanes.i6038.sroa.0.0.copyload = load <8 x float>, ptr %data.i.i7360, align 4, !dbg !29377, !alias.scope !29382, !noalias !29386
  %690 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i315.i.sroa.22.sroa.0.015390), !dbg !29391
  %691 = fmul <8 x float> %lanes.i6038.sroa.0.0.copyload, %_5.i4945, !dbg !29398
  %692 = fadd <8 x float> %691, zeroinitializer, !dbg !29404
  %693 = fmul <8 x float> %lanes.i6038.sroa.0.0.copyload, %_14.i.i.i275.i.sroa.0.0.copyload, !dbg !29409
  %694 = fadd <8 x float> %693, zeroinitializer, !dbg !29414
  %695 = fmul <8 x float> %lanes.i6038.sroa.0.0.copyload, %_17.i.i.i272.i.sroa.0.0.copyload, !dbg !29419
  %696 = fadd <8 x float> %695, zeroinitializer, !dbg !29424
  %697 = fmul <8 x float> %lanes.i6038.sroa.0.0.copyload, %_20.i.i.i269.i.sroa.0.0.copyload, !dbg !29429
  %698 = fadd <8 x float> %697, zeroinitializer, !dbg !29434
  %699 = fmul <8 x float> %history.i315.i.sroa.0.015383, %_25.i.i.i265.i.sroa.0.0.copyload, !dbg !29439
  %700 = fadd <8 x float> %692, %699, !dbg !29444
  %701 = fmul <8 x float> %history.i315.i.sroa.0.015383, %_28.i.i.i262.i.sroa.0.0.copyload, !dbg !29449
  %702 = fadd <8 x float> %694, %701, !dbg !29454
  %703 = fmul <8 x float> %history.i315.i.sroa.0.015383, %_31.i.i.i259.i.sroa.0.0.copyload, !dbg !29459
  %704 = fadd <8 x float> %696, %703, !dbg !29464
  %705 = fmul <8 x float> %history.i315.i.sroa.0.015383, %_34.i.i.i256.i.sroa.0.0.copyload, !dbg !29469
  %706 = fadd <8 x float> %698, %705, !dbg !29474
  %707 = fmul <8 x float> %history.i315.i.sroa.10.sroa.0.015394, %_39.i.i.i252.i.sroa.0.0.copyload, !dbg !29479
  %708 = fadd <8 x float> %700, %707, !dbg !29484
  %709 = fmul <8 x float> %history.i315.i.sroa.10.sroa.0.015394, %_42.i.i.i249.i.sroa.0.0.copyload, !dbg !29489
  %710 = fadd <8 x float> %702, %709, !dbg !29494
  %711 = fmul <8 x float> %history.i315.i.sroa.10.sroa.0.015394, %_45.i.i.i246.i.sroa.0.0.copyload, !dbg !29499
  %712 = fadd <8 x float> %704, %711, !dbg !29504
  %713 = fmul <8 x float> %history.i315.i.sroa.10.sroa.0.015394, %_48.i.i.i243.i.sroa.0.0.copyload, !dbg !29509
  %714 = fadd <8 x float> %706, %713, !dbg !29514
  %715 = fmul <8 x float> %history.i315.i.sroa.13.sroa.0.015393, %_53.i.i.i239.i.sroa.0.0.copyload, !dbg !29519
  %716 = fadd <8 x float> %708, %715, !dbg !29524
  %717 = fmul <8 x float> %history.i315.i.sroa.13.sroa.0.015393, %_56.i.i.i236.i.sroa.0.0.copyload, !dbg !29529
  %718 = fadd <8 x float> %710, %717, !dbg !29534
  %719 = fmul <8 x float> %history.i315.i.sroa.13.sroa.0.015393, %_59.i.i.i233.i.sroa.0.0.copyload, !dbg !29539
  %720 = fadd <8 x float> %712, %719, !dbg !29544
  %721 = fmul <8 x float> %history.i315.i.sroa.13.sroa.0.015393, %_62.i.i.i230.i.sroa.0.0.copyload, !dbg !29549
  %722 = fadd <8 x float> %714, %721, !dbg !29554
  %723 = fmul <8 x float> %history.i315.i.sroa.16.sroa.0.015392, %_67.i.i.i226.i.sroa.0.0.copyload, !dbg !29559
  %724 = fadd <8 x float> %716, %723, !dbg !29564
  %725 = fmul <8 x float> %history.i315.i.sroa.16.sroa.0.015392, %_70.i.i.i223.i.sroa.0.0.copyload, !dbg !29569
  %726 = fadd <8 x float> %718, %725, !dbg !29574
  %727 = fmul <8 x float> %history.i315.i.sroa.16.sroa.0.015392, %_73.i.i.i220.i.sroa.0.0.copyload, !dbg !29579
  %728 = fadd <8 x float> %720, %727, !dbg !29584
  %729 = fmul <8 x float> %history.i315.i.sroa.16.sroa.0.015392, %_76.i.i.i217.i.sroa.0.0.copyload, !dbg !29589
  %730 = fadd <8 x float> %722, %729, !dbg !29594
  %731 = fmul <8 x float> %history.i315.i.sroa.19.sroa.0.015391, %_81.i.i.i213.i.sroa.0.0.copyload, !dbg !29599
  %732 = fadd <8 x float> %724, %731, !dbg !29604
  %733 = fmul <8 x float> %history.i315.i.sroa.19.sroa.0.015391, %_84.i.i.i210.i.sroa.0.0.copyload, !dbg !29609
  %734 = fadd <8 x float> %726, %733, !dbg !29614
  %735 = fmul <8 x float> %history.i315.i.sroa.19.sroa.0.015391, %_87.i.i.i207.i.sroa.0.0.copyload, !dbg !29619
  %736 = fadd <8 x float> %728, %735, !dbg !29624
  %737 = fmul <8 x float> %history.i315.i.sroa.19.sroa.0.015391, %_90.i.i.i204.i.sroa.0.0.copyload, !dbg !29629
  %738 = fadd <8 x float> %730, %737, !dbg !29634
  %739 = fmul <8 x float> %history.i315.i.sroa.22.sroa.0.015390, %_95.i.i.i200.i.sroa.0.0.copyload, !dbg !29639
  %740 = fadd <8 x float> %732, %739, !dbg !29644
  %741 = fmul <8 x float> %history.i315.i.sroa.22.sroa.0.015390, %_98.i.i.i197.i.sroa.0.0.copyload, !dbg !29649
  %742 = fadd <8 x float> %734, %741, !dbg !29654
  %743 = fmul <8 x float> %history.i315.i.sroa.22.sroa.0.015390, %_101.i.i.i194.i.sroa.0.0.copyload, !dbg !29659
  %744 = fadd <8 x float> %736, %743, !dbg !29664
  %745 = fmul <8 x float> %history.i315.i.sroa.22.sroa.0.015390, %_104.i.i.i191.i.sroa.0.0.copyload, !dbg !29669
  %746 = fadd <8 x float> %738, %745, !dbg !29674
  %747 = fmul <8 x float> %history.i315.i.sroa.25.sroa.0.015385, %_109.i.i.i187.i.sroa.0.0.copyload, !dbg !29679
  %748 = fadd <8 x float> %740, %747, !dbg !29684
  %749 = fmul <8 x float> %history.i315.i.sroa.25.sroa.0.015385, %_112.i.i.i184.i.sroa.0.0.copyload, !dbg !29689
  %750 = fadd <8 x float> %742, %749, !dbg !29694
  %751 = fmul <8 x float> %history.i315.i.sroa.25.sroa.0.015385, %_115.i.i.i181.i.sroa.0.0.copyload, !dbg !29699
  %752 = fadd <8 x float> %744, %751, !dbg !29704
  %753 = fmul <8 x float> %history.i315.i.sroa.25.sroa.0.015385, %_118.i.i.i178.i.sroa.0.0.copyload, !dbg !29709
  %754 = fadd <8 x float> %746, %753, !dbg !29714
  %755 = fmul <8 x float> %history.i315.i.sroa.29.sroa.0.015386, %_123.i.i.i174.i.sroa.0.0.copyload, !dbg !29719
  %756 = fadd <8 x float> %748, %755, !dbg !29724
  %757 = fmul <8 x float> %history.i315.i.sroa.29.sroa.0.015386, %_126.i.i.i171.i.sroa.0.0.copyload, !dbg !29729
  %758 = fadd <8 x float> %750, %757, !dbg !29734
  %759 = fmul <8 x float> %history.i315.i.sroa.29.sroa.0.015386, %_129.i.i.i168.i.sroa.0.0.copyload, !dbg !29739
  %760 = fadd <8 x float> %752, %759, !dbg !29744
  %761 = fmul <8 x float> %history.i315.i.sroa.29.sroa.0.015386, %_132.i.i.i165.i.sroa.0.0.copyload, !dbg !29749
  %762 = fadd <8 x float> %754, %761, !dbg !29754
  %763 = fmul <8 x float> %history.i315.i.sroa.32.sroa.0.015387, %_137.i.i.i161.i.sroa.0.0.copyload, !dbg !29759
  %764 = fadd <8 x float> %756, %763, !dbg !29764
  %765 = fmul <8 x float> %history.i315.i.sroa.32.sroa.0.015387, %_140.i.i.i158.i.sroa.0.0.copyload, !dbg !29769
  %766 = fadd <8 x float> %758, %765, !dbg !29774
  %767 = fmul <8 x float> %history.i315.i.sroa.32.sroa.0.015387, %_143.i.i.i155.i.sroa.0.0.copyload, !dbg !29779
  %768 = fadd <8 x float> %760, %767, !dbg !29784
  %769 = fmul <8 x float> %history.i315.i.sroa.32.sroa.0.015387, %_146.i.i.i152.i.sroa.0.0.copyload, !dbg !29789
  %770 = fadd <8 x float> %762, %769, !dbg !29794
  %771 = fmul <8 x float> %history.i315.i.sroa.35.sroa.0.015388, %_151.i.i.i148.i.sroa.0.0.copyload, !dbg !29799
  %772 = fadd <8 x float> %764, %771, !dbg !29804
  %773 = fmul <8 x float> %history.i315.i.sroa.35.sroa.0.015388, %_154.i.i.i145.i.sroa.0.0.copyload, !dbg !29809
  %774 = fadd <8 x float> %766, %773, !dbg !29814
  %775 = fmul <8 x float> %history.i315.i.sroa.35.sroa.0.015388, %_157.i.i.i142.i.sroa.0.0.copyload, !dbg !29819
  %776 = fadd <8 x float> %768, %775, !dbg !29824
  %777 = fmul <8 x float> %history.i315.i.sroa.35.sroa.0.015388, %_160.i.i.i139.i.sroa.0.0.copyload, !dbg !29829
  %778 = fadd <8 x float> %770, %777, !dbg !29834
  %779 = fmul <8 x float> %history.i315.i.sroa.38.sroa.0.015389, %_165.i.i.i135.i.sroa.0.0.copyload, !dbg !29839
  %780 = fadd <8 x float> %772, %779, !dbg !29844
  %781 = fmul <8 x float> %history.i315.i.sroa.38.sroa.0.015389, %_168.i.i.i132.i.sroa.0.0.copyload, !dbg !29849
  %782 = fadd <8 x float> %774, %781, !dbg !29854
  %783 = fmul <8 x float> %history.i315.i.sroa.38.sroa.0.015389, %_171.i.i.i129.i.sroa.0.0.copyload, !dbg !29859
  %784 = fadd <8 x float> %776, %783, !dbg !29864
  %785 = fmul <8 x float> %history.i315.i.sroa.38.sroa.0.015389, %_174.i.i.i126.i.sroa.0.0.copyload, !dbg !29869
  %786 = fadd <8 x float> %778, %785, !dbg !29874
  %787 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %780), !dbg !29879
  %788 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %690, <8 x float> %787), !dbg !29885
  %789 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %782), !dbg !29879
  %790 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %788, <8 x float> %789), !dbg !29885
  %791 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %784), !dbg !29879
  %792 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %790, <8 x float> %791), !dbg !29885
  %793 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %786), !dbg !29879
  %794 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %792, <8 x float> %793), !dbg !29885
  %795 = add nuw nsw i64 %iter.i311.i.sroa.16.015384, 1, !dbg !29890
  %data.i4.i7364 = getelementptr inbounds nuw float, ptr %peaks_left.i981, i64 %start1.i.i7359, !dbg !29891
  store <8 x float> %794, ptr %data.i4.i7364, align 4, !dbg !29894, !alias.scope !29899, !noalias !29903
  %exitcond17999.not = icmp eq i64 %795, %umax18028, !dbg !29352
  br i1 %exitcond17999.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit339.i, label %bb6.i318.i, !dbg !29352

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit339.i: ; preds = %bb6.i318.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7350
  %history.i315.i.sroa.0.0.lcssa = phi <8 x float> [ %history.i315.i.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7350 ], [ %lanes.i6038.sroa.0.0.copyload, %bb6.i318.i ], !dbg !29907
  %history.i315.i.sroa.25.sroa.0.0.lcssa = phi <8 x float> [ %history.i315.i.sroa.25.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7350 ], [ %history.i315.i.sroa.22.sroa.0.015390, %bb6.i318.i ], !dbg !29907
  %history.i315.i.sroa.29.sroa.0.0.lcssa = phi <8 x float> [ %history.i315.i.sroa.29.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7350 ], [ %history.i315.i.sroa.25.sroa.0.015385, %bb6.i318.i ], !dbg !29907
  %history.i315.i.sroa.32.sroa.0.0.lcssa = phi <8 x float> [ %history.i315.i.sroa.32.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7350 ], [ %history.i315.i.sroa.29.sroa.0.015386, %bb6.i318.i ], !dbg !29907
  %history.i315.i.sroa.35.sroa.0.0.lcssa = phi <8 x float> [ %history.i315.i.sroa.35.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7350 ], [ %history.i315.i.sroa.32.sroa.0.015387, %bb6.i318.i ], !dbg !29907
  %history.i315.i.sroa.38.sroa.0.0.lcssa = phi <8 x float> [ %history.i315.i.sroa.38.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7350 ], [ %history.i315.i.sroa.35.sroa.0.015388, %bb6.i318.i ], !dbg !29907
  %history.i315.i.sroa.41.sroa.0.0.lcssa = phi <8 x float> [ %history.i315.i.sroa.41.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7350 ], [ %history.i315.i.sroa.38.sroa.0.015389, %bb6.i318.i ], !dbg !29907
  %history.i315.i.sroa.22.sroa.0.0.lcssa = phi <8 x float> [ %history.i315.i.sroa.22.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7350 ], [ %history.i315.i.sroa.19.sroa.0.015391, %bb6.i318.i ], !dbg !29907
  %history.i315.i.sroa.19.sroa.0.0.lcssa = phi <8 x float> [ %history.i315.i.sroa.19.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7350 ], [ %history.i315.i.sroa.16.sroa.0.015392, %bb6.i318.i ], !dbg !29907
  %history.i315.i.sroa.16.sroa.0.0.lcssa = phi <8 x float> [ %history.i315.i.sroa.16.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7350 ], [ %history.i315.i.sroa.13.sroa.0.015393, %bb6.i318.i ], !dbg !29907
  %history.i315.i.sroa.13.sroa.0.0.lcssa = phi <8 x float> [ %history.i315.i.sroa.13.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7350 ], [ %history.i315.i.sroa.10.sroa.0.015394, %bb6.i318.i ], !dbg !29907
  %history.i315.i.sroa.10.sroa.0.0.lcssa = phi <8 x float> [ %history.i315.i.sroa.10.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7350 ], [ %history.i315.i.sroa.0.015383, %bb6.i318.i ], !dbg !29907
  store <8 x float> %history.i315.i.sroa.0.0.lcssa, ptr %hot_left.i988, align 32, !dbg !29908
  store <8 x float> %history.i315.i.sroa.10.sroa.0.0.lcssa, ptr %history.i315.i.sroa.10.0.hot_left.i988.sroa_idx, align 32, !dbg !29908
  store <8 x float> %history.i315.i.sroa.13.sroa.0.0.lcssa, ptr %history.i315.i.sroa.13.0.hot_left.i988.sroa_idx, align 32, !dbg !29908
  store <8 x float> %history.i315.i.sroa.16.sroa.0.0.lcssa, ptr %history.i315.i.sroa.16.0.hot_left.i988.sroa_idx, align 32, !dbg !29908
  store <8 x float> %history.i315.i.sroa.19.sroa.0.0.lcssa, ptr %history.i315.i.sroa.19.0.hot_left.i988.sroa_idx, align 32, !dbg !29908
  store <8 x float> %history.i315.i.sroa.22.sroa.0.0.lcssa, ptr %history.i315.i.sroa.22.0.hot_left.i988.sroa_idx, align 32, !dbg !29908
  store <8 x float> %history.i315.i.sroa.25.sroa.0.0.lcssa, ptr %history.i315.i.sroa.25.0.hot_left.i988.sroa_idx, align 32, !dbg !29908
  store <8 x float> %history.i315.i.sroa.29.sroa.0.0.lcssa, ptr %history.i315.i.sroa.29.0.hot_left.i988.sroa_idx, align 32, !dbg !29908
  store <8 x float> %history.i315.i.sroa.32.sroa.0.0.lcssa, ptr %history.i315.i.sroa.32.0.hot_left.i988.sroa_idx, align 32, !dbg !29908
  store <8 x float> %history.i315.i.sroa.35.sroa.0.0.lcssa, ptr %history.i315.i.sroa.35.0.hot_left.i988.sroa_idx, align 32, !dbg !29908
  store <8 x float> %history.i315.i.sroa.38.sroa.0.0.lcssa, ptr %history.i315.i.sroa.38.0.hot_left.i988.sroa_idx, align 32, !dbg !29908
  store <8 x float> %history.i315.i.sroa.41.sroa.0.0.lcssa, ptr %history.i315.i.sroa.41.0.hot_left.i988.sroa_idx, align 32, !dbg !29908
  %_139.not.i = icmp ugt i64 %_51.i, %right_io.1, !dbg !29909
  br i1 %_139.not.i, label %bb47.i, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7410, !dbg !29909, !prof !1406

bb47.i:                                           ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit339.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %active_base.i1005, i64 noundef %_51.i, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f6bbc99b95dcf27c100d71299ef7abde) #30, !dbg !29913, !noalias !29345
  unreachable, !dbg !29913

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7410: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit339.i
  %_146.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %active_base.i1005, !dbg !29914
  %history.i.i952.sroa.0.0.copyload = load <8 x float>, ptr %hot_right.i987, align 32, !dbg !29918
  %history.i.i952.sroa.10.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i952.sroa.10.0.hot_right.i987.sroa_idx, align 32, !dbg !29918
  %history.i.i952.sroa.13.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i952.sroa.13.0.hot_right.i987.sroa_idx, align 32, !dbg !29918
  %history.i.i952.sroa.16.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i952.sroa.16.0.hot_right.i987.sroa_idx, align 32, !dbg !29918
  %history.i.i952.sroa.19.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i952.sroa.19.0.hot_right.i987.sroa_idx, align 32, !dbg !29918
  %history.i.i952.sroa.22.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i952.sroa.22.0.hot_right.i987.sroa_idx, align 32, !dbg !29918
  %history.i.i952.sroa.25.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i952.sroa.25.0.hot_right.i987.sroa_idx, align 32, !dbg !29918
  %history.i.i952.sroa.29.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i952.sroa.29.0.hot_right.i987.sroa_idx, align 32, !dbg !29918
  %history.i.i952.sroa.32.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i952.sroa.32.0.hot_right.i987.sroa_idx, align 32, !dbg !29918
  %history.i.i952.sroa.35.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i952.sroa.35.0.hot_right.i987.sroa_idx, align 32, !dbg !29918
  %history.i.i952.sroa.38.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i952.sroa.38.0.hot_right.i987.sroa_idx, align 32, !dbg !29918
  %history.i.i952.sroa.41.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i952.sroa.41.0.hot_right.i987.sroa_idx, align 32, !dbg !29918
  br i1 %_2.i735315382.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1030, label %bb6.i.i1009.lr.ph, !dbg !29920

bb6.i.i1009.lr.ph:                                ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7410
  %_5.i4801 = load <8 x float>, ptr %self, align 32, !alias.scope !29923, !noalias !29926
  %_14.i.i.i.i913.sroa.0.0.copyload = load <8 x float>, ptr %615, align 32, !noalias !29938
  %_17.i.i.i.i910.sroa.0.0.copyload = load <8 x float>, ptr %616, align 32, !noalias !29938
  %_20.i.i.i.i907.sroa.0.0.copyload = load <8 x float>, ptr %617, align 32, !noalias !29938
  %_25.i.i.i.i903.sroa.0.0.copyload = load <8 x float>, ptr %row12.i.i.i322.i, align 32, !noalias !29938
  %_28.i.i.i.i900.sroa.0.0.copyload = load <8 x float>, ptr %618, align 32, !noalias !29938
  %_31.i.i.i.i897.sroa.0.0.copyload = load <8 x float>, ptr %619, align 32, !noalias !29938
  %_34.i.i.i.i894.sroa.0.0.copyload = load <8 x float>, ptr %620, align 32, !noalias !29938
  %_39.i.i.i.i890.sroa.0.0.copyload = load <8 x float>, ptr %row13.i.i.i323.i, align 32, !noalias !29938
  %_42.i.i.i.i887.sroa.0.0.copyload = load <8 x float>, ptr %621, align 32, !noalias !29938
  %_45.i.i.i.i884.sroa.0.0.copyload = load <8 x float>, ptr %622, align 32, !noalias !29938
  %_48.i.i.i.i881.sroa.0.0.copyload = load <8 x float>, ptr %623, align 32, !noalias !29938
  %_53.i.i.i.i877.sroa.0.0.copyload = load <8 x float>, ptr %row14.i.i.i324.i, align 32, !noalias !29938
  %_56.i.i.i.i874.sroa.0.0.copyload = load <8 x float>, ptr %624, align 32, !noalias !29938
  %_59.i.i.i.i871.sroa.0.0.copyload = load <8 x float>, ptr %625, align 32, !noalias !29938
  %_62.i.i.i.i868.sroa.0.0.copyload = load <8 x float>, ptr %626, align 32, !noalias !29938
  %_67.i.i.i.i864.sroa.0.0.copyload = load <8 x float>, ptr %row15.i.i.i325.i, align 32, !noalias !29938
  %_70.i.i.i.i861.sroa.0.0.copyload = load <8 x float>, ptr %627, align 32, !noalias !29938
  %_73.i.i.i.i858.sroa.0.0.copyload = load <8 x float>, ptr %628, align 32, !noalias !29938
  %_76.i.i.i.i855.sroa.0.0.copyload = load <8 x float>, ptr %629, align 32, !noalias !29938
  %_81.i.i.i.i851.sroa.0.0.copyload = load <8 x float>, ptr %row16.i.i.i326.i, align 32, !noalias !29938
  %_84.i.i.i.i848.sroa.0.0.copyload = load <8 x float>, ptr %630, align 32, !noalias !29938
  %_87.i.i.i.i845.sroa.0.0.copyload = load <8 x float>, ptr %631, align 32, !noalias !29938
  %_90.i.i.i.i842.sroa.0.0.copyload = load <8 x float>, ptr %632, align 32, !noalias !29938
  %_95.i.i.i.i838.sroa.0.0.copyload = load <8 x float>, ptr %row17.i.i.i327.i, align 32, !noalias !29938
  %_98.i.i.i.i835.sroa.0.0.copyload = load <8 x float>, ptr %633, align 32, !noalias !29938
  %_101.i.i.i.i832.sroa.0.0.copyload = load <8 x float>, ptr %634, align 32, !noalias !29938
  %_104.i.i.i.i829.sroa.0.0.copyload = load <8 x float>, ptr %635, align 32, !noalias !29938
  %_109.i.i.i.i825.sroa.0.0.copyload = load <8 x float>, ptr %row18.i.i.i328.i, align 32, !noalias !29938
  %_112.i.i.i.i822.sroa.0.0.copyload = load <8 x float>, ptr %636, align 32, !noalias !29938
  %_115.i.i.i.i819.sroa.0.0.copyload = load <8 x float>, ptr %637, align 32, !noalias !29938
  %_118.i.i.i.i816.sroa.0.0.copyload = load <8 x float>, ptr %638, align 32, !noalias !29938
  %_123.i.i.i.i812.sroa.0.0.copyload = load <8 x float>, ptr %row19.i.i.i329.i, align 32, !noalias !29938
  %_126.i.i.i.i809.sroa.0.0.copyload = load <8 x float>, ptr %639, align 32, !noalias !29938
  %_129.i.i.i.i806.sroa.0.0.copyload = load <8 x float>, ptr %640, align 32, !noalias !29938
  %_132.i.i.i.i803.sroa.0.0.copyload = load <8 x float>, ptr %641, align 32, !noalias !29938
  %_137.i.i.i.i799.sroa.0.0.copyload = load <8 x float>, ptr %row20.i.i.i330.i, align 32, !noalias !29938
  %_140.i.i.i.i796.sroa.0.0.copyload = load <8 x float>, ptr %642, align 32, !noalias !29938
  %_143.i.i.i.i793.sroa.0.0.copyload = load <8 x float>, ptr %643, align 32, !noalias !29938
  %_146.i.i.i.i790.sroa.0.0.copyload = load <8 x float>, ptr %644, align 32, !noalias !29938
  %_151.i.i.i.i786.sroa.0.0.copyload = load <8 x float>, ptr %row21.i.i.i331.i, align 32, !noalias !29938
  %_154.i.i.i.i783.sroa.0.0.copyload = load <8 x float>, ptr %645, align 32, !noalias !29938
  %_157.i.i.i.i780.sroa.0.0.copyload = load <8 x float>, ptr %646, align 32, !noalias !29938
  %_160.i.i.i.i777.sroa.0.0.copyload = load <8 x float>, ptr %647, align 32, !noalias !29938
  %_165.i.i.i.i773.sroa.0.0.copyload = load <8 x float>, ptr %row22.i.i.i332.i, align 32, !noalias !29938
  %_168.i.i.i.i770.sroa.0.0.copyload = load <8 x float>, ptr %648, align 32, !noalias !29938
  %_171.i.i.i.i767.sroa.0.0.copyload = load <8 x float>, ptr %649, align 32, !noalias !29938
  %_174.i.i.i.i764.sroa.0.0.copyload = load <8 x float>, ptr %650, align 32, !noalias !29938
  br label %bb6.i.i1009, !dbg !29920

bb6.i.i1009:                                      ; preds = %bb6.i.i1009.lr.ph, %bb6.i.i1009
  %history.i.i952.sroa.10.sroa.0.015421 = phi <8 x float> [ %history.i.i952.sroa.10.sroa.0.0.copyload, %bb6.i.i1009.lr.ph ], [ %history.i.i952.sroa.0.015410, %bb6.i.i1009 ]
  %history.i.i952.sroa.13.sroa.0.015420 = phi <8 x float> [ %history.i.i952.sroa.13.sroa.0.0.copyload, %bb6.i.i1009.lr.ph ], [ %history.i.i952.sroa.10.sroa.0.015421, %bb6.i.i1009 ]
  %history.i.i952.sroa.16.sroa.0.015419 = phi <8 x float> [ %history.i.i952.sroa.16.sroa.0.0.copyload, %bb6.i.i1009.lr.ph ], [ %history.i.i952.sroa.13.sroa.0.015420, %bb6.i.i1009 ]
  %history.i.i952.sroa.19.sroa.0.015418 = phi <8 x float> [ %history.i.i952.sroa.19.sroa.0.0.copyload, %bb6.i.i1009.lr.ph ], [ %history.i.i952.sroa.16.sroa.0.015419, %bb6.i.i1009 ]
  %history.i.i952.sroa.22.sroa.0.015417 = phi <8 x float> [ %history.i.i952.sroa.22.sroa.0.0.copyload, %bb6.i.i1009.lr.ph ], [ %history.i.i952.sroa.19.sroa.0.015418, %bb6.i.i1009 ]
  %history.i.i952.sroa.38.sroa.0.015416 = phi <8 x float> [ %history.i.i952.sroa.38.sroa.0.0.copyload, %bb6.i.i1009.lr.ph ], [ %history.i.i952.sroa.35.sroa.0.015415, %bb6.i.i1009 ]
  %history.i.i952.sroa.35.sroa.0.015415 = phi <8 x float> [ %history.i.i952.sroa.35.sroa.0.0.copyload, %bb6.i.i1009.lr.ph ], [ %history.i.i952.sroa.32.sroa.0.015414, %bb6.i.i1009 ]
  %history.i.i952.sroa.32.sroa.0.015414 = phi <8 x float> [ %history.i.i952.sroa.32.sroa.0.0.copyload, %bb6.i.i1009.lr.ph ], [ %history.i.i952.sroa.29.sroa.0.015413, %bb6.i.i1009 ]
  %history.i.i952.sroa.29.sroa.0.015413 = phi <8 x float> [ %history.i.i952.sroa.29.sroa.0.0.copyload, %bb6.i.i1009.lr.ph ], [ %history.i.i952.sroa.25.sroa.0.015412, %bb6.i.i1009 ]
  %history.i.i952.sroa.25.sroa.0.015412 = phi <8 x float> [ %history.i.i952.sroa.25.sroa.0.0.copyload, %bb6.i.i1009.lr.ph ], [ %history.i.i952.sroa.22.sroa.0.015417, %bb6.i.i1009 ]
  %iter.i124.i.sroa.16.015411 = phi i64 [ 0, %bb6.i.i1009.lr.ph ], [ %901, %bb6.i.i1009 ]
  %history.i.i952.sroa.0.015410 = phi <8 x float> [ %history.i.i952.sroa.0.0.copyload, %bb6.i.i1009.lr.ph ], [ %lanes.i6029.sroa.0.0.copyload, %bb6.i.i1009 ]
  %start1.i.i7419 = shl i64 %iter.i124.i.sroa.16.015411, 3, !dbg !29941
  %data.i.i7420 = getelementptr inbounds nuw float, ptr %_146.i, i64 %start1.i.i7419, !dbg !29943
  %lanes.i6029.sroa.0.0.copyload = load <8 x float>, ptr %data.i.i7420, align 4, !dbg !29945, !alias.scope !29950, !noalias !29954
  %796 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i.i952.sroa.22.sroa.0.015417), !dbg !29959
  %797 = fmul <8 x float> %lanes.i6029.sroa.0.0.copyload, %_5.i4801, !dbg !29966
  %798 = fadd <8 x float> %797, zeroinitializer, !dbg !29972
  %799 = fmul <8 x float> %lanes.i6029.sroa.0.0.copyload, %_14.i.i.i.i913.sroa.0.0.copyload, !dbg !29977
  %800 = fadd <8 x float> %799, zeroinitializer, !dbg !29982
  %801 = fmul <8 x float> %lanes.i6029.sroa.0.0.copyload, %_17.i.i.i.i910.sroa.0.0.copyload, !dbg !29987
  %802 = fadd <8 x float> %801, zeroinitializer, !dbg !29992
  %803 = fmul <8 x float> %lanes.i6029.sroa.0.0.copyload, %_20.i.i.i.i907.sroa.0.0.copyload, !dbg !29997
  %804 = fadd <8 x float> %803, zeroinitializer, !dbg !30002
  %805 = fmul <8 x float> %history.i.i952.sroa.0.015410, %_25.i.i.i.i903.sroa.0.0.copyload, !dbg !30007
  %806 = fadd <8 x float> %798, %805, !dbg !30012
  %807 = fmul <8 x float> %history.i.i952.sroa.0.015410, %_28.i.i.i.i900.sroa.0.0.copyload, !dbg !30017
  %808 = fadd <8 x float> %800, %807, !dbg !30022
  %809 = fmul <8 x float> %history.i.i952.sroa.0.015410, %_31.i.i.i.i897.sroa.0.0.copyload, !dbg !30027
  %810 = fadd <8 x float> %802, %809, !dbg !30032
  %811 = fmul <8 x float> %history.i.i952.sroa.0.015410, %_34.i.i.i.i894.sroa.0.0.copyload, !dbg !30037
  %812 = fadd <8 x float> %804, %811, !dbg !30042
  %813 = fmul <8 x float> %history.i.i952.sroa.10.sroa.0.015421, %_39.i.i.i.i890.sroa.0.0.copyload, !dbg !30047
  %814 = fadd <8 x float> %806, %813, !dbg !30052
  %815 = fmul <8 x float> %history.i.i952.sroa.10.sroa.0.015421, %_42.i.i.i.i887.sroa.0.0.copyload, !dbg !30057
  %816 = fadd <8 x float> %808, %815, !dbg !30062
  %817 = fmul <8 x float> %history.i.i952.sroa.10.sroa.0.015421, %_45.i.i.i.i884.sroa.0.0.copyload, !dbg !30067
  %818 = fadd <8 x float> %810, %817, !dbg !30072
  %819 = fmul <8 x float> %history.i.i952.sroa.10.sroa.0.015421, %_48.i.i.i.i881.sroa.0.0.copyload, !dbg !30077
  %820 = fadd <8 x float> %812, %819, !dbg !30082
  %821 = fmul <8 x float> %history.i.i952.sroa.13.sroa.0.015420, %_53.i.i.i.i877.sroa.0.0.copyload, !dbg !30087
  %822 = fadd <8 x float> %814, %821, !dbg !30092
  %823 = fmul <8 x float> %history.i.i952.sroa.13.sroa.0.015420, %_56.i.i.i.i874.sroa.0.0.copyload, !dbg !30097
  %824 = fadd <8 x float> %816, %823, !dbg !30102
  %825 = fmul <8 x float> %history.i.i952.sroa.13.sroa.0.015420, %_59.i.i.i.i871.sroa.0.0.copyload, !dbg !30107
  %826 = fadd <8 x float> %818, %825, !dbg !30112
  %827 = fmul <8 x float> %history.i.i952.sroa.13.sroa.0.015420, %_62.i.i.i.i868.sroa.0.0.copyload, !dbg !30117
  %828 = fadd <8 x float> %820, %827, !dbg !30122
  %829 = fmul <8 x float> %history.i.i952.sroa.16.sroa.0.015419, %_67.i.i.i.i864.sroa.0.0.copyload, !dbg !30127
  %830 = fadd <8 x float> %822, %829, !dbg !30132
  %831 = fmul <8 x float> %history.i.i952.sroa.16.sroa.0.015419, %_70.i.i.i.i861.sroa.0.0.copyload, !dbg !30137
  %832 = fadd <8 x float> %824, %831, !dbg !30142
  %833 = fmul <8 x float> %history.i.i952.sroa.16.sroa.0.015419, %_73.i.i.i.i858.sroa.0.0.copyload, !dbg !30147
  %834 = fadd <8 x float> %826, %833, !dbg !30152
  %835 = fmul <8 x float> %history.i.i952.sroa.16.sroa.0.015419, %_76.i.i.i.i855.sroa.0.0.copyload, !dbg !30157
  %836 = fadd <8 x float> %828, %835, !dbg !30162
  %837 = fmul <8 x float> %history.i.i952.sroa.19.sroa.0.015418, %_81.i.i.i.i851.sroa.0.0.copyload, !dbg !30167
  %838 = fadd <8 x float> %830, %837, !dbg !30172
  %839 = fmul <8 x float> %history.i.i952.sroa.19.sroa.0.015418, %_84.i.i.i.i848.sroa.0.0.copyload, !dbg !30177
  %840 = fadd <8 x float> %832, %839, !dbg !30182
  %841 = fmul <8 x float> %history.i.i952.sroa.19.sroa.0.015418, %_87.i.i.i.i845.sroa.0.0.copyload, !dbg !30187
  %842 = fadd <8 x float> %834, %841, !dbg !30192
  %843 = fmul <8 x float> %history.i.i952.sroa.19.sroa.0.015418, %_90.i.i.i.i842.sroa.0.0.copyload, !dbg !30197
  %844 = fadd <8 x float> %836, %843, !dbg !30202
  %845 = fmul <8 x float> %history.i.i952.sroa.22.sroa.0.015417, %_95.i.i.i.i838.sroa.0.0.copyload, !dbg !30207
  %846 = fadd <8 x float> %838, %845, !dbg !30212
  %847 = fmul <8 x float> %history.i.i952.sroa.22.sroa.0.015417, %_98.i.i.i.i835.sroa.0.0.copyload, !dbg !30217
  %848 = fadd <8 x float> %840, %847, !dbg !30222
  %849 = fmul <8 x float> %history.i.i952.sroa.22.sroa.0.015417, %_101.i.i.i.i832.sroa.0.0.copyload, !dbg !30227
  %850 = fadd <8 x float> %842, %849, !dbg !30232
  %851 = fmul <8 x float> %history.i.i952.sroa.22.sroa.0.015417, %_104.i.i.i.i829.sroa.0.0.copyload, !dbg !30237
  %852 = fadd <8 x float> %844, %851, !dbg !30242
  %853 = fmul <8 x float> %history.i.i952.sroa.25.sroa.0.015412, %_109.i.i.i.i825.sroa.0.0.copyload, !dbg !30247
  %854 = fadd <8 x float> %846, %853, !dbg !30252
  %855 = fmul <8 x float> %history.i.i952.sroa.25.sroa.0.015412, %_112.i.i.i.i822.sroa.0.0.copyload, !dbg !30257
  %856 = fadd <8 x float> %848, %855, !dbg !30262
  %857 = fmul <8 x float> %history.i.i952.sroa.25.sroa.0.015412, %_115.i.i.i.i819.sroa.0.0.copyload, !dbg !30267
  %858 = fadd <8 x float> %850, %857, !dbg !30272
  %859 = fmul <8 x float> %history.i.i952.sroa.25.sroa.0.015412, %_118.i.i.i.i816.sroa.0.0.copyload, !dbg !30277
  %860 = fadd <8 x float> %852, %859, !dbg !30282
  %861 = fmul <8 x float> %history.i.i952.sroa.29.sroa.0.015413, %_123.i.i.i.i812.sroa.0.0.copyload, !dbg !30287
  %862 = fadd <8 x float> %854, %861, !dbg !30292
  %863 = fmul <8 x float> %history.i.i952.sroa.29.sroa.0.015413, %_126.i.i.i.i809.sroa.0.0.copyload, !dbg !30297
  %864 = fadd <8 x float> %856, %863, !dbg !30302
  %865 = fmul <8 x float> %history.i.i952.sroa.29.sroa.0.015413, %_129.i.i.i.i806.sroa.0.0.copyload, !dbg !30307
  %866 = fadd <8 x float> %858, %865, !dbg !30312
  %867 = fmul <8 x float> %history.i.i952.sroa.29.sroa.0.015413, %_132.i.i.i.i803.sroa.0.0.copyload, !dbg !30317
  %868 = fadd <8 x float> %860, %867, !dbg !30322
  %869 = fmul <8 x float> %history.i.i952.sroa.32.sroa.0.015414, %_137.i.i.i.i799.sroa.0.0.copyload, !dbg !30327
  %870 = fadd <8 x float> %862, %869, !dbg !30332
  %871 = fmul <8 x float> %history.i.i952.sroa.32.sroa.0.015414, %_140.i.i.i.i796.sroa.0.0.copyload, !dbg !30337
  %872 = fadd <8 x float> %864, %871, !dbg !30342
  %873 = fmul <8 x float> %history.i.i952.sroa.32.sroa.0.015414, %_143.i.i.i.i793.sroa.0.0.copyload, !dbg !30347
  %874 = fadd <8 x float> %866, %873, !dbg !30352
  %875 = fmul <8 x float> %history.i.i952.sroa.32.sroa.0.015414, %_146.i.i.i.i790.sroa.0.0.copyload, !dbg !30357
  %876 = fadd <8 x float> %868, %875, !dbg !30362
  %877 = fmul <8 x float> %history.i.i952.sroa.35.sroa.0.015415, %_151.i.i.i.i786.sroa.0.0.copyload, !dbg !30367
  %878 = fadd <8 x float> %870, %877, !dbg !30372
  %879 = fmul <8 x float> %history.i.i952.sroa.35.sroa.0.015415, %_154.i.i.i.i783.sroa.0.0.copyload, !dbg !30377
  %880 = fadd <8 x float> %872, %879, !dbg !30382
  %881 = fmul <8 x float> %history.i.i952.sroa.35.sroa.0.015415, %_157.i.i.i.i780.sroa.0.0.copyload, !dbg !30387
  %882 = fadd <8 x float> %874, %881, !dbg !30392
  %883 = fmul <8 x float> %history.i.i952.sroa.35.sroa.0.015415, %_160.i.i.i.i777.sroa.0.0.copyload, !dbg !30397
  %884 = fadd <8 x float> %876, %883, !dbg !30402
  %885 = fmul <8 x float> %history.i.i952.sroa.38.sroa.0.015416, %_165.i.i.i.i773.sroa.0.0.copyload, !dbg !30407
  %886 = fadd <8 x float> %878, %885, !dbg !30412
  %887 = fmul <8 x float> %history.i.i952.sroa.38.sroa.0.015416, %_168.i.i.i.i770.sroa.0.0.copyload, !dbg !30417
  %888 = fadd <8 x float> %880, %887, !dbg !30422
  %889 = fmul <8 x float> %history.i.i952.sroa.38.sroa.0.015416, %_171.i.i.i.i767.sroa.0.0.copyload, !dbg !30427
  %890 = fadd <8 x float> %882, %889, !dbg !30432
  %891 = fmul <8 x float> %history.i.i952.sroa.38.sroa.0.015416, %_174.i.i.i.i764.sroa.0.0.copyload, !dbg !30437
  %892 = fadd <8 x float> %884, %891, !dbg !30442
  %893 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %886), !dbg !30447
  %894 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %796, <8 x float> %893), !dbg !30453
  %895 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %888), !dbg !30447
  %896 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %894, <8 x float> %895), !dbg !30453
  %897 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %890), !dbg !30447
  %898 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %896, <8 x float> %897), !dbg !30453
  %899 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %892), !dbg !30447
  %900 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %898, <8 x float> %899), !dbg !30453
  %901 = add nuw nsw i64 %iter.i124.i.sroa.16.015411, 1, !dbg !30458
  %data.i4.i7424 = getelementptr inbounds nuw float, ptr %peaks_right.i980, i64 %start1.i.i7419, !dbg !30459
  store <8 x float> %900, ptr %data.i4.i7424, align 4, !dbg !30462, !alias.scope !30467, !noalias !30471
  %exitcond18004.not = icmp eq i64 %901, %umax18028, !dbg !29920
  br i1 %exitcond18004.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1030, label %bb6.i.i1009, !dbg !29920

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1030: ; preds = %bb6.i.i1009, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7410
  %history.i.i952.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i952.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7410 ], [ %lanes.i6029.sroa.0.0.copyload, %bb6.i.i1009 ], !dbg !30475
  %history.i.i952.sroa.25.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i952.sroa.25.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7410 ], [ %history.i.i952.sroa.22.sroa.0.015417, %bb6.i.i1009 ], !dbg !30475
  %history.i.i952.sroa.29.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i952.sroa.29.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7410 ], [ %history.i.i952.sroa.25.sroa.0.015412, %bb6.i.i1009 ], !dbg !30475
  %history.i.i952.sroa.32.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i952.sroa.32.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7410 ], [ %history.i.i952.sroa.29.sroa.0.015413, %bb6.i.i1009 ], !dbg !30475
  %history.i.i952.sroa.35.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i952.sroa.35.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7410 ], [ %history.i.i952.sroa.32.sroa.0.015414, %bb6.i.i1009 ], !dbg !30475
  %history.i.i952.sroa.38.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i952.sroa.38.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7410 ], [ %history.i.i952.sroa.35.sroa.0.015415, %bb6.i.i1009 ], !dbg !30475
  %history.i.i952.sroa.41.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i952.sroa.41.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7410 ], [ %history.i.i952.sroa.38.sroa.0.015416, %bb6.i.i1009 ], !dbg !30475
  %history.i.i952.sroa.22.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i952.sroa.22.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7410 ], [ %history.i.i952.sroa.19.sroa.0.015418, %bb6.i.i1009 ], !dbg !30475
  %history.i.i952.sroa.19.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i952.sroa.19.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7410 ], [ %history.i.i952.sroa.16.sroa.0.015419, %bb6.i.i1009 ], !dbg !30475
  %history.i.i952.sroa.16.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i952.sroa.16.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7410 ], [ %history.i.i952.sroa.13.sroa.0.015420, %bb6.i.i1009 ], !dbg !30475
  %history.i.i952.sroa.13.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i952.sroa.13.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7410 ], [ %history.i.i952.sroa.10.sroa.0.015421, %bb6.i.i1009 ], !dbg !30475
  %history.i.i952.sroa.10.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i952.sroa.10.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7410 ], [ %history.i.i952.sroa.0.015410, %bb6.i.i1009 ], !dbg !30475
  store <8 x float> %history.i.i952.sroa.0.0.lcssa, ptr %hot_right.i987, align 32, !dbg !30476
  store <8 x float> %history.i.i952.sroa.10.sroa.0.0.lcssa, ptr %history.i.i952.sroa.10.0.hot_right.i987.sroa_idx, align 32, !dbg !30476
  store <8 x float> %history.i.i952.sroa.13.sroa.0.0.lcssa, ptr %history.i.i952.sroa.13.0.hot_right.i987.sroa_idx, align 32, !dbg !30476
  store <8 x float> %history.i.i952.sroa.16.sroa.0.0.lcssa, ptr %history.i.i952.sroa.16.0.hot_right.i987.sroa_idx, align 32, !dbg !30476
  store <8 x float> %history.i.i952.sroa.19.sroa.0.0.lcssa, ptr %history.i.i952.sroa.19.0.hot_right.i987.sroa_idx, align 32, !dbg !30476
  store <8 x float> %history.i.i952.sroa.22.sroa.0.0.lcssa, ptr %history.i.i952.sroa.22.0.hot_right.i987.sroa_idx, align 32, !dbg !30476
  store <8 x float> %history.i.i952.sroa.25.sroa.0.0.lcssa, ptr %history.i.i952.sroa.25.0.hot_right.i987.sroa_idx, align 32, !dbg !30476
  store <8 x float> %history.i.i952.sroa.29.sroa.0.0.lcssa, ptr %history.i.i952.sroa.29.0.hot_right.i987.sroa_idx, align 32, !dbg !30476
  store <8 x float> %history.i.i952.sroa.32.sroa.0.0.lcssa, ptr %history.i.i952.sroa.32.0.hot_right.i987.sroa_idx, align 32, !dbg !30476
  store <8 x float> %history.i.i952.sroa.35.sroa.0.0.lcssa, ptr %history.i.i952.sroa.35.0.hot_right.i987.sroa_idx, align 32, !dbg !30476
  store <8 x float> %history.i.i952.sroa.38.sroa.0.0.lcssa, ptr %history.i.i952.sroa.38.0.hot_right.i987.sroa_idx, align 32, !dbg !30476
  store <8 x float> %history.i.i952.sroa.41.sroa.0.0.lcssa, ptr %history.i.i952.sroa.41.0.hot_right.i987.sroa_idx, align 32, !dbg !30476
  br i1 %_2.i735315382.not, label %bb13.i.loopexit, label %bb48.i.lr.ph, !dbg !29317

bb48.i.lr.ph:                                     ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i1030
  %.promoted = load <8 x float>, ptr %663, align 32
  %.promoted15535 = load <8 x float>, ptr %681, align 32
  %_8.i27.i.sroa.0.0.copyload.pre = load <8 x float>, ptr %_68.i1035, align 32, !dbg !30477
  %_9.i26.i.sroa.0.0.copyload.pre = load <8 x float>, ptr %_69.i, align 32, !dbg !30482
  %_8.i.i968.sroa.0.0.copyload.pre = load <8 x float>, ptr %_73.i, align 32, !dbg !30484
  %_9.i.i967.sroa.0.0.copyload.pre = load <8 x float>, ptr %_74.i1036, align 32, !dbg !30487
  %_64.i43.i.sroa.0.0.copyload = load <8 x float>, ptr %664, align 32
  %_64.i.i.sroa.0.0.copyload = load <8 x float>, ptr %682, align 32
  br label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit6027, !dbg !29317

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit6027: ; preds = %bb48.i.lr.ph, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6667
  %_58.i.i.sroa.0.0.copyload15536 = phi <8 x float> [ %.promoted15535, %bb48.i.lr.ph ], [ %1024, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6667 ]
  %_58.i46.i.sroa.0.0.copyload15462 = phi <8 x float> [ %.promoted, %bb48.i.lr.ph ], [ %937, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6667 ]
  %main_cursor.sroa.0.1.i103315459 = phi i64 [ %main_cursor.sroa.0.0.i100215613, %bb48.i.lr.ph ], [ %spec.store.select.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6667 ]
  %ring_cursor.sroa.0.1.i103215458 = phi i64 [ %ring_cursor.sroa.0.0.i100115612, %bb48.i.lr.ph ], [ %spec.store.select13.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6667 ]
  %iter3.sroa.0.0.i103115457 = phi i64 [ 0, %bb48.i.lr.ph ], [ %902, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6667 ]
  %902 = add nuw nsw i64 %iter3.sroa.0.0.i103115457, 1, !dbg !30489
  %_64.i = add nuw nsw i64 %iter3.sroa.0.0.i103115457, %iter.sroa.0.0.i15610, !dbg !30495
  %base.i1034 = shl i64 %_64.i, 3, !dbg !30495
  %_78.i1037 = shl i64 %iter3.sroa.0.0.i103115457, 3, !dbg !30496
  %_162.i = getelementptr inbounds nuw float, ptr %peaks_left.i981, i64 %_78.i1037, !dbg !30497
  %lanes.i6020.sroa.0.0.copyload = load <8 x float>, ptr %_162.i, align 4, !dbg !30508, !alias.scope !30513, !noalias !30517
  %_167.i = getelementptr inbounds nuw float, ptr %peaks_right.i980, i64 %_78.i1037, !dbg !30521
  %lanes.i6011.sroa.0.0.copyload = load <8 x float>, ptr %_167.i, align 4, !dbg !30531, !alias.scope !30536, !noalias !30540
  %903 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i6011.sroa.0.0.copyload, <8 x float> %lanes.i6020.sroa.0.0.copyload), !dbg !30544
  %904 = select <8 x i1> %652, <8 x float> %903, <8 x float> %lanes.i6020.sroa.0.0.copyload, !dbg !30549
  %905 = select <8 x i1> %652, <8 x float> %903, <8 x float> %lanes.i6011.sroa.0.0.copyload, !dbg !30554
  %_168.i = icmp samesign ugt i64 %base.i1034, %left_io.1, !dbg !30559
  br i1 %_168.i, label %bb52.i, label %bb53.i1038, !dbg !30559, !prof !1406

bb53.i1038:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit6027
  %_170.i1039 = sub nuw nsw i64 %left_io.1, %base.i1034, !dbg !30563
  %_174.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %base.i1034, !dbg !30564
  %_8.i6005 = icmp samesign ugt i64 %_170.i1039, 7, !dbg !30569
  br i1 %_8.i6005, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit6009, label %bb2.i6006, !dbg !30569, !prof !1421

bb2.i6006:                                        ; preds = %bb53.i1038
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_170.i1039, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !30574, !noalias !30575
  unreachable, !dbg !30574

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit6009: ; preds = %bb53.i1038
  %_91.i = load i64, ptr %653, align 8, !dbg !30579, !alias.scope !29240, !noalias !30580, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !30581), !dbg !30584
  %width.i61.i = load i64, ptr %654, align 8, !dbg !30585, !alias.scope !30586, !noalias !30587, !noundef !12
  %906 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %904, <8 x float> %_8.i27.i.sroa.0.0.copyload.pre, i8 30), !dbg !30596
  %907 = fdiv <8 x float> %_8.i27.i.sroa.0.0.copyload.pre, %904, !dbg !30602
  %908 = bitcast <8 x float> %906 to <8 x i32>, !dbg !30607
  %909 = icmp slt <8 x i32> %908, zeroinitializer, !dbg !30611
  %910 = select <8 x i1> %909, <8 x float> %907, <8 x float> splat (float 1.000000e+00), !dbg !30611
  %_144.1.i62.i = load i64, ptr %655, align 8, !dbg !30613, !alias.scope !30586, !noalias !30587, !noundef !12
  %_22.i63.i = mul i64 %width.i61.i, %ring_cursor.sroa.0.1.i103215458, !dbg !30614
  %_92.i64.i = icmp ugt i64 %_22.i63.i, %_144.1.i62.i, !dbg !30615
  br i1 %_92.i64.i, label %bb37.i122.i, label %bb38.i65.i, !dbg !30615, !prof !1406

bb38.i65.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit6009
  %_95.i67.i = sub nuw i64 %_144.1.i62.i, %_22.i63.i, !dbg !30618
  %_8.i6699 = icmp samesign ugt i64 %_95.i67.i, 7, !dbg !30619
  br i1 %_8.i6699, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6702, label %bb2.i6700, !dbg !30619, !prof !1421

bb2.i6700:                                        ; preds = %bb38.i65.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_95.i67.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !30624, !noalias !30625
  unreachable, !dbg !30624

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6702: ; preds = %bb38.i65.i
  %_144.0.i66.i = load ptr, ptr %656, align 8, !dbg !30613, !alias.scope !30586, !noalias !30587, !nonnull !12, !noundef !12
  %_99.i68.i = getelementptr inbounds nuw float, ptr %_144.0.i66.i, i64 %_22.i63.i, !dbg !30629
  store <8 x float> %910, ptr %_99.i68.i, align 4, !dbg !30631, !alias.scope !30635, !noalias !30639
  tail call void @llvm.experimental.noalias.scope.decl(metadata !30641), !dbg !30644
  %width.i2374 = load i64, ptr %654, align 8, !dbg !30645, !alias.scope !30641, !noalias !30647, !noundef !12
  %911 = icmp eq i64 %width.i2374, 0, !dbg !30649
  br i1 %911, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2482, label %bb32.i2381.lr.ph, !dbg !30649

bb32.i2381.lr.ph:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6702
  %_112.1.i2384 = load i64, ptr %62, align 8, !alias.scope !30641, !noalias !30647, !noundef !12
  %_112.0.i2388 = load ptr, ptr %61, align 8, !nonnull !12
  %912 = add i64 %ring_cursor.sroa.0.1.i103215458, 1
  %_23.not.i2395 = icmp ult i64 %912, %_91.i
  %913 = select i1 %_23.not.i2395, i64 0, i64 %_91.i
  %start1.sroa.0.0.i2396 = sub nuw i64 %912, %913
  %_114.1.i2399 = load i64, ptr %655, align 8
  %_114.0.i2403 = load ptr, ptr %656, align 8, !nonnull !12
  %_116.1.i2404 = load i64, ptr %657, align 8
  %_116.0.i2408 = load ptr, ptr %658, align 8, !nonnull !12
  %_118.1.i2412 = load i64, ptr %659, align 8
  %_118.0.i2416 = load ptr, ptr %660, align 8, !nonnull !12
  %_45.i2429 = mul i64 %width.i2374, %start1.sroa.0.0.i2396
  br label %bb32.i2381, !dbg !30649

bb32.i2381:                                       ; preds = %bb32.i2381.lr.ph, %bb31.i2444
  %iter.i2373.sroa.10.015439 = phi i64 [ %width.i2374, %bb32.i2381.lr.ph ], [ %914, %bb31.i2444 ]
  %iter.i2373.sroa.7.015438 = phi i64 [ 0, %bb32.i2381.lr.ph ], [ %_9.0.i7451, %bb31.i2444 ]
  %iter.i2373.sroa.0.0.idx15437 = phi i64 [ 0, %bb32.i2381.lr.ph ], [ %iter.i2373.sroa.0.0.add, %bb31.i2444 ]
  %iter.i2373.sroa.0.0.ptr15440 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 %iter.i2373.sroa.0.0.idx15437, !dbg !30651
  %914 = add i64 %iter.i2373.sroa.10.015439, -1, !dbg !30651
  %_7.i.i7447 = icmp eq i64 %iter.i2373.sroa.0.0.idx15437, 32, !dbg !30652
  br i1 %_7.i.i7447, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2482, label %bb3.i2383, !dbg !30656

bb3.i2383:                                        ; preds = %bb32.i2381
  %iter.i2373.sroa.0.0.add = add nuw nsw i64 %iter.i2373.sroa.0.0.idx15437, 4, !dbg !30657
  %_9.0.i7451 = add nuw nsw i64 %iter.i2373.sroa.7.015438, 1, !dbg !30659
  %exitcond18008.not = icmp eq i64 %iter.i2373.sroa.7.015438, %_112.1.i2384, !dbg !30660
  br i1 %exitcond18008.not, label %panic.i2386, label %bb5.i2387, !dbg !30660

bb5.i2387:                                        ; preds = %bb3.i2383
  %915 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i2388, i64 %iter.i2373.sroa.7.015438, !dbg !30660
  %shape.i2389 = load i32, ptr %915, align 4, !dbg !30660, !noalias !30661, !noundef !12
  %916 = getelementptr inbounds nuw i8, ptr %915, i64 4, !dbg !30660
  %shape3.i2390 = load i32, ptr %916, align 4, !dbg !30660, !noalias !30661, !noundef !12
  %window.i2391 = zext i32 %shape.i2389 to i64, !dbg !30662
  %_19.i2392 = zext i32 %shape3.i2390 to i64, !dbg !30663
  %917 = add i64 %ring_cursor.sroa.0.1.i103215458, %_19.i2392, !dbg !30664
  %_20.not.i2393 = icmp ult i64 %917, %_91.i, !dbg !30665
  %918 = select i1 %_20.not.i2393, i64 0, i64 %_91.i, !dbg !30665
  %spec.select.i2394 = sub nuw i64 %917, %918, !dbg !30665
  %_27.i2397 = mul i64 %spec.select.i2394, %width.i2374, !dbg !30666
  %_26.i2398 = add i64 %_27.i2397, %iter.i2373.sroa.7.015438, !dbg !30666
  %_30.i2400 = icmp ult i64 %_26.i2398, %_114.1.i2399, !dbg !30667
  br i1 %_30.i2400, label %bb12.i2402, label %panic5.i2401, !dbg !30667

panic.i2386:                                      ; preds = %bb3.i2383
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i2384, i64 noundef %_112.1.i2384, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4a8785a681d008a9bfd0cd82628ea9cb) #30, !dbg !30660, !noalias !30661
  unreachable, !dbg !30660

bb12.i2402:                                       ; preds = %bb5.i2387
  %919 = getelementptr inbounds nuw float, ptr %_114.0.i2403, i64 %_26.i2398, !dbg !30667
  %920 = load float, ptr %919, align 4, !dbg !30667, !noalias !30661, !noundef !12
  %exitcond18009.not = icmp eq i64 %iter.i2373.sroa.7.015438, %_116.1.i2404, !dbg !30668
  br i1 %exitcond18009.not, label %panic6.i2406, label %bb13.i2407, !dbg !30668

panic5.i2401:                                     ; preds = %bb5.i2387
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i2398, i64 noundef %_114.1.i2399, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cbce7773ac40979e4ba2385da3aec116) #30, !dbg !30667, !noalias !30661
  unreachable, !dbg !30667

bb13.i2407:                                       ; preds = %bb12.i2402
  %921 = getelementptr inbounds nuw i32, ptr %_116.0.i2408, i64 %iter.i2373.sroa.7.015438, !dbg !30668
  %_32.i2409 = load i32, ptr %921, align 4, !dbg !30668, !noalias !30661, !noundef !12
  %position.i2410 = zext i32 %_32.i2409 to i64, !dbg !30668
  %922 = icmp eq i32 %_32.i2409, 0, !dbg !30669
  br i1 %922, label %bb17.i2419, label %bb15.i2411, !dbg !30669

panic6.i2406:                                     ; preds = %bb12.i2402
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i2404, i64 noundef %_116.1.i2404, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ec0d48f73ebfc2755df5cedaa60b5c0a) #30, !dbg !30668, !noalias !30661
  unreachable, !dbg !30668

bb15.i2411:                                       ; preds = %bb13.i2407
  %_37.i2413 = icmp ult i64 %iter.i2373.sroa.7.015438, %_118.1.i2412, !dbg !30670
  br i1 %_37.i2413, label %bb16.i2415, label %panic7.i2414, !dbg !30670

bb17.i2419:                                       ; preds = %bb35.i2480, %bb16.i2415, %bb13.i2407
  %newest.sroa.0.0.i2420 = phi float [ %920, %bb13.i2407 ], [ %_35.i2417, %bb35.i2480 ], [ %920, %bb16.i2415 ], !dbg !30671
  %exitcond18010.not = icmp eq i64 %iter.i2373.sroa.7.015438, %_118.1.i2412, !dbg !30672
  br i1 %exitcond18010.not, label %panic8.i2423, label %bb18.i2424, !dbg !30672

bb16.i2415:                                       ; preds = %bb15.i2411
  %923 = getelementptr inbounds nuw float, ptr %_118.0.i2416, i64 %iter.i2373.sroa.7.015438, !dbg !30670
  %_35.i2417 = load float, ptr %923, align 4, !dbg !30670, !noalias !30661, !noundef !12
  %_102.i2418 = fcmp olt float %_35.i2417, %920, !dbg !30673
  br i1 %_102.i2418, label %bb35.i2480, label %bb17.i2419, !dbg !30673

panic7.i2414:                                     ; preds = %bb15.i2411
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i2373.sroa.7.015438, i64 noundef %_118.1.i2412, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2c461872bb652d4796cdcf89c28c82c8) #30, !dbg !30670, !noalias !30661
  unreachable, !dbg !30670

bb35.i2480:                                       ; preds = %bb16.i2415
  br label %bb17.i2419, !dbg !30675

bb18.i2424:                                       ; preds = %bb17.i2419
  %924 = getelementptr inbounds nuw float, ptr %_118.0.i2416, i64 %iter.i2373.sroa.7.015438, !dbg !30672
  store float %newest.sroa.0.0.i2420, ptr %924, align 4, !dbg !30672, !noalias !30661
  %_42.i2426 = add nuw nsw i64 %position.i2410, 1, !dbg !30676
  %complete.i2427 = icmp eq i64 %_42.i2426, %window.i2391, !dbg !30676
  br i1 %complete.i2427, label %bb22.i2449, label %bb20.i2428, !dbg !30677

panic8.i2423:                                     ; preds = %bb17.i2419
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i2412, i64 noundef %_118.1.i2412, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2b690e2c7763f11809942906fc2ca813) #30, !dbg !30672, !noalias !30661
  unreachable, !dbg !30672

bb20.i2428:                                       ; preds = %bb18.i2424
  %_44.i2430 = add i64 %iter.i2373.sroa.7.015438, %_45.i2429, !dbg !30678
  %_47.i2432 = icmp ult i64 %_44.i2430, %_114.1.i2399, !dbg !30679
  br i1 %_47.i2432, label %bb30.i2442, label %panic9.i2433, !dbg !30679

panic9.i2433:                                     ; preds = %bb20.i2428
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i2430, i64 noundef %_114.1.i2399, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9fa421ae81817f58fcfc4a3243223891) #30, !dbg !30679, !noalias !30661
  unreachable, !dbg !30679

bb30.i2442:                                       ; preds = %bb20.i2428
  %925 = getelementptr inbounds nuw float, ptr %_114.0.i2403, i64 %_44.i2430, !dbg !30679
  %_43.i2436 = load float, ptr %925, align 4, !dbg !30679, !noalias !30661, !noundef !12
  %_103.i2437 = fcmp olt float %_43.i2436, %newest.sroa.0.0.i2420, !dbg !30680
  %newest.sroa.0.1.i2438 = select i1 %_103.i2437, float %_43.i2436, float %newest.sroa.0.0.i2420, !dbg !30680
  store float %newest.sroa.0.1.i2438, ptr %iter.i2373.sroa.0.0.ptr15440, align 4, !dbg !30682, !noalias !30661
  %926 = trunc i64 %_42.i2426 to i32, !dbg !30683
  br label %bb31.i2444, !dbg !30684

bb31.i2444:                                       ; preds = %bb25.i2477, %bb30.i2442
  %storemerge13424 = phi i32 [ %926, %bb30.i2442 ], [ 0, %bb25.i2477 ], !dbg !30685
  store i32 %storemerge13424, ptr %921, align 4, !dbg !30685, !noalias !30661
  %927 = icmp eq i64 %914, 0, !dbg !30649
  br i1 %927, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2482, label %bb32.i2381, !dbg !30649

bb22.i2449:                                       ; preds = %bb18.i2424
  store float %newest.sroa.0.0.i2420, ptr %iter.i2373.sroa.0.0.ptr15440, align 4, !dbg !30682, !noalias !30661
  %928 = load float, ptr %919, align 4, !dbg !30686, !noalias !30661, !noundef !12
  br label %bb41.i2462, !dbg !30687

bb41.i2462:                                       ; preds = %bb22.i2449, %bb25.i2477
  %iter2.sroa.0.0.i245415436 = phi i64 [ 0, %bb22.i2449 ], [ %_105.i2463, %bb25.i2477 ]
  %suffix.sroa.0.0.i245315435 = phi float [ %928, %bb22.i2449 ], [ %suffix.sroa.0.1.i2473, %bb25.i2477 ]
  %end.sroa.0.1.i245215434 = phi i64 [ %spec.select.i2394, %bb22.i2449 ], [ %931, %bb25.i2477 ]
  %_56.i2464 = mul i64 %end.sroa.0.1.i245215434, %width.i2374, !dbg !30690
  %_55.i2465 = add i64 %_56.i2464, %iter.i2373.sroa.7.015438, !dbg !30690
  %_59.i2467 = icmp ult i64 %_55.i2465, %_114.1.i2399, !dbg !30691
  br i1 %_59.i2467, label %bb25.i2477, label %panic13.i2468, !dbg !30691

panic13.i2468:                                    ; preds = %bb41.i2462
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i2465, i64 noundef %_114.1.i2399, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_91a4c6b9b17ebf4d863f9a70b6dc929a) #30, !dbg !30691, !noalias !30661
  unreachable, !dbg !30691

bb25.i2477:                                       ; preds = %bb41.i2462
  %_105.i2463 = add nuw nsw i64 %iter2.sroa.0.0.i245415436, 1, !dbg !30692
  %929 = getelementptr inbounds nuw float, ptr %_114.0.i2403, i64 %_55.i2465, !dbg !30691
  %_54.i2471 = load float, ptr %929, align 4, !dbg !30691, !noalias !30661, !noundef !12
  %_107.i2472 = fcmp olt float %suffix.sroa.0.0.i245315435, %_54.i2471, !dbg !30695
  %suffix.sroa.0.1.i2473 = select i1 %_107.i2472, float %suffix.sroa.0.0.i245315435, float %_54.i2471, !dbg !30695
  store float %suffix.sroa.0.1.i2473, ptr %929, align 4, !dbg !30697, !noalias !30661
  %930 = icmp eq i64 %end.sroa.0.1.i245215434, 0, !dbg !30698
  %spec.store.select.i2479 = select i1 %930, i64 %_91.i, i64 %end.sroa.0.1.i245215434, !dbg !30698
  %931 = add i64 %spec.store.select.i2479, -1, !dbg !30699
  %exitcond18007.not = icmp eq i64 %_105.i2463, %window.i2391, !dbg !30700
  br i1 %exitcond18007.not, label %bb31.i2444, label %bb41.i2462, !dbg !30687

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2482: ; preds = %bb31.i2444, %bb32.i2381, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6702
  %lanes.i5995.sroa.0.0.copyload = load <8 x float>, ptr %scratch.i, align 4, !dbg !30702, !alias.scope !30707, !noalias !30711
  %932 = fmul <8 x float> %lanes.i5995.sroa.0.0.copyload, splat (float 1.638400e+04), !dbg !30715
  %933 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %932), !dbg !30720
  %934 = fmul <8 x float> %933, splat (float 0x3F10000000000000), !dbg !30725
  %935 = icmp eq i64 %width.i61.i, 0, !dbg !30730
  %_149.1.i96.i.pre = load i64, ptr %661, align 8, !dbg !30732, !alias.scope !30586, !noalias !30587
  br i1 %935, label %bb16.i95.i, label %bb39.i75.i.lr.ph, !dbg !30730

bb39.i75.i.lr.ph:                                 ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2482
  %_145.1.i78.i = load i64, ptr %62, align 8, !alias.scope !30586, !noalias !30587, !noundef !12
  %_145.0.i82.i = load ptr, ptr %61, align 8, !nonnull !12
  %_147.0.i93.i = load ptr, ptr %662, align 8, !nonnull !12
  %exitcond18013.not = icmp eq i64 %_145.1.i78.i, 0, !dbg !30733
  br i1 %exitcond18013.not, label %panic.i80.i, label %bb17.i81.i, !dbg !30733

bb37.i122.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit6009
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i63.i, i64 noundef %_144.1.i62.i, i64 noundef %_144.1.i62.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_56df7c041d29359441bca272bf4e38e3) #30, !dbg !30734, !noalias !30735
  unreachable, !dbg !30734

bb16.i95.i.loopexit:                              ; preds = %bb21.i92.i.7, %bb21.i92.i.6, %bb21.i92.i.5, %bb21.i92.i.4, %bb21.i92.i.3, %bb21.i92.i.2, %bb21.i92.i.1, %bb21.i92.i
  %lanes.i5988.sroa.0.0.copyload.pre = load <8 x float>, ptr %scratch.i, align 4, !dbg !30736, !alias.scope !30741, !noalias !30745
  br label %bb16.i95.i, !dbg !30749

bb16.i95.i:                                       ; preds = %bb16.i95.i.loopexit, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2482
  %lanes.i5988.sroa.0.0.copyload = phi <8 x float> [ %lanes.i5988.sroa.0.0.copyload.pre, %bb16.i95.i.loopexit ], [ %lanes.i5995.sroa.0.0.copyload, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2482 ], !dbg !30736
  %936 = fadd <8 x float> %934, %_58.i46.i.sroa.0.0.copyload15462, !dbg !30750
  %937 = fsub <8 x float> %936, %lanes.i5988.sroa.0.0.copyload, !dbg !30755
  %_109.i97.i = icmp ugt i64 %_22.i63.i, %_149.1.i96.i.pre, !dbg !30760
  br i1 %_109.i97.i, label %bb42.i121.i, label %bb43.i98.i, !dbg !30760, !prof !1406

bb43.i98.i:                                       ; preds = %bb16.i95.i
  %_112.i100.i = sub nuw i64 %_149.1.i96.i.pre, %_22.i63.i, !dbg !30763
  %_8.i6694 = icmp samesign ugt i64 %_112.i100.i, 7, !dbg !30764
  br i1 %_8.i6694, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6697, label %bb2.i6695, !dbg !30764, !prof !1421

bb2.i6695:                                        ; preds = %bb43.i98.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_112.i100.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !30769, !noalias !30770
  unreachable, !dbg !30769

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6697: ; preds = %bb43.i98.i
  %_149.0.i99.i = load ptr, ptr %662, align 8, !dbg !30732, !alias.scope !30586, !noalias !30587, !nonnull !12, !noundef !12
  %_116.i101.i = getelementptr inbounds nuw float, ptr %_149.0.i99.i, i64 %_22.i63.i, !dbg !30774
  store <8 x float> %934, ptr %_116.i101.i, align 4, !dbg !30776, !alias.scope !30780, !noalias !30784
  %_68.i39.i.sroa.0.0.copyload = load <8 x float>, ptr %665, align 32, !dbg !30786
  %938 = fdiv <8 x float> %937, %_64.i43.i.sroa.0.0.copyload, !dbg !30787
  %939 = fsub <8 x float> splat (float 1.000000e+00), %938, !dbg !30792
  %940 = fsub <8 x float> %939, %_68.i39.i.sroa.0.0.copyload, !dbg !30797
  %941 = fmul <8 x float> %_9.i26.i.sroa.0.0.copyload.pre, %940, !dbg !30802
  %942 = fadd <8 x float> %_68.i39.i.sroa.0.0.copyload, %941, !dbg !30807
  %943 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %939, <8 x float> %942), !dbg !30811
  %944 = bitcast <8 x float> %943 to <8 x i32>, !dbg !30816
  %945 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %943), !dbg !30822
  %946 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %945, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !30824
  %947 = bitcast <8 x float> %946 to <8 x i32>, !dbg !30830
  %948 = xor <8 x i32> %947, splat (i32 -1), !dbg !30836
  %949 = and <8 x i32> %948, %944, !dbg !30838
  %950 = bitcast <8 x i32> %949 to <8 x float>, !dbg !30842
  store <8 x i32> %949, ptr %665, align 32, !dbg !30843
  %951 = fsub <8 x float> splat (float 1.000000e+00), %950, !dbg !30844
  %_150.1.i102.i = load i64, ptr %666, align 8, !dbg !30849, !alias.scope !30586, !noalias !30587, !noundef !12
  %_76.i103.i = mul i64 %width.i61.i, %main_cursor.sroa.0.1.i103315459, !dbg !30850
  %_120.i104.i = icmp ugt i64 %_76.i103.i, %_150.1.i102.i, !dbg !30851
  br i1 %_120.i104.i, label %bb48.i120.i, label %bb49.i105.i, !dbg !30851, !prof !1406

bb42.i121.i:                                      ; preds = %bb16.i95.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i63.i, i64 noundef %_149.1.i96.i.pre, i64 noundef %_149.1.i96.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_90498045d73339daaf9e4f537508f58b) #30, !dbg !30854, !noalias !30855
  unreachable, !dbg !30854

bb49.i105.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6697
  %_123.i107.i = sub nuw i64 %_150.1.i102.i, %_76.i103.i, !dbg !30856
  %_8.i5982 = icmp samesign ugt i64 %_123.i107.i, 7, !dbg !30857
  br i1 %_8.i5982, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6687, label %bb2.i5983, !dbg !30857, !prof !1421

bb2.i5983:                                        ; preds = %bb49.i105.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_123.i107.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !30862, !noalias !30863
  unreachable, !dbg !30862

bb48.i120.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6697
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i103.i, i64 noundef %_150.1.i102.i, i64 noundef %_150.1.i102.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_da8af254b6d507a8e2ca31e544bfd21d) #30, !dbg !30867, !noalias !30855
  unreachable, !dbg !30867

bb17.i81.i:                                       ; preds = %bb39.i75.i.lr.ph
  %952 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i, i64 8, !dbg !30733
  %_44.i83.i = load i32, ptr %952, align 4, !dbg !30733, !noalias !30855, !noundef !12
  %_43.i84.i = zext i32 %_44.i83.i to i64, !dbg !30733
  %953 = add i64 %ring_cursor.sroa.0.1.i103215458, %_43.i84.i, !dbg !30868
  %_47.not.i85.i = icmp ult i64 %953, %_91.i, !dbg !30869
  %954 = select i1 %_47.not.i85.i, i64 0, i64 %_91.i, !dbg !30869
  %spec.select.i86.i = sub nuw i64 %953, %954, !dbg !30869
  %_51.i87.i = mul i64 %spec.select.i86.i, %width.i61.i, !dbg !30870
  %_53.i90.i = icmp ult i64 %_51.i87.i, %_149.1.i96.i.pre, !dbg !30871
  br i1 %_53.i90.i, label %bb21.i92.i, label %panic1.i91.i, !dbg !30871

panic.i80.i:                                      ; preds = %bb39.i75.i.7, %bb39.i75.i.6, %bb39.i75.i.5, %bb39.i75.i.4, %bb39.i75.i.3, %bb39.i75.i.2, %bb39.i75.i.1, %bb39.i75.i.lr.ph
  %_145.1.i78.i.lcssa.ph = phi i64 [ 7, %bb39.i75.i.7 ], [ 6, %bb39.i75.i.6 ], [ 5, %bb39.i75.i.5 ], [ 4, %bb39.i75.i.4 ], [ 3, %bb39.i75.i.3 ], [ 2, %bb39.i75.i.2 ], [ 1, %bb39.i75.i.1 ], [ 0, %bb39.i75.i.lr.ph ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i78.i.lcssa.ph, i64 noundef %_145.1.i78.i.lcssa.ph, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a6feb40b34112df5f214f84424dd2c7c) #30, !dbg !30733, !noalias !30855
  unreachable, !dbg !30733

bb21.i92.i:                                       ; preds = %bb17.i81.i
  %955 = getelementptr inbounds nuw float, ptr %_147.0.i93.i, i64 %_51.i87.i, !dbg !30871
  %_49.i94.i = load float, ptr %955, align 4, !dbg !30871, !noalias !30855, !noundef !12
  store float %_49.i94.i, ptr %scratch.i, align 4, !dbg !30872, !noalias !30855
  %956 = icmp eq i64 %width.i61.i, 1, !dbg !30730
  br i1 %956, label %bb16.i95.i.loopexit, label %bb39.i75.i.1, !dbg !30730

bb39.i75.i.1:                                     ; preds = %bb21.i92.i
  %exitcond18013.1.not = icmp eq i64 %_145.1.i78.i, 1, !dbg !30733
  br i1 %exitcond18013.1.not, label %panic.i80.i, label %bb17.i81.i.1, !dbg !30733

bb17.i81.i.1:                                     ; preds = %bb39.i75.i.1
  %957 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i, i64 20, !dbg !30733
  %_44.i83.i.1 = load i32, ptr %957, align 4, !dbg !30733, !noalias !30855, !noundef !12
  %_43.i84.i.1 = zext i32 %_44.i83.i.1 to i64, !dbg !30733
  %958 = add i64 %ring_cursor.sroa.0.1.i103215458, %_43.i84.i.1, !dbg !30868
  %_47.not.i85.i.1 = icmp ult i64 %958, %_91.i, !dbg !30869
  %959 = select i1 %_47.not.i85.i.1, i64 0, i64 %_91.i, !dbg !30869
  %spec.select.i86.i.1 = sub nuw i64 %958, %959, !dbg !30869
  %_51.i87.i.1 = mul i64 %spec.select.i86.i.1, %width.i61.i, !dbg !30870
  %_50.i88.i.1 = add i64 %_51.i87.i.1, 1, !dbg !30870
  %_53.i90.i.1 = icmp ult i64 %_50.i88.i.1, %_149.1.i96.i.pre, !dbg !30871
  br i1 %_53.i90.i.1, label %bb21.i92.i.1, label %panic1.i91.i, !dbg !30871

bb21.i92.i.1:                                     ; preds = %bb17.i81.i.1
  %960 = getelementptr inbounds nuw float, ptr %_147.0.i93.i, i64 %_50.i88.i.1, !dbg !30871
  %_49.i94.i.1 = load float, ptr %960, align 4, !dbg !30871, !noalias !30855, !noundef !12
  store float %_49.i94.i.1, ptr %iter.i49.i.sroa.0.0.ptr15444.1, align 4, !dbg !30872, !noalias !30855
  %961 = icmp eq i64 %width.i61.i, 2, !dbg !30730
  br i1 %961, label %bb16.i95.i.loopexit, label %bb39.i75.i.2, !dbg !30730

bb39.i75.i.2:                                     ; preds = %bb21.i92.i.1
  %exitcond18013.2.not = icmp eq i64 %_145.1.i78.i, 2, !dbg !30733
  br i1 %exitcond18013.2.not, label %panic.i80.i, label %bb17.i81.i.2, !dbg !30733

bb17.i81.i.2:                                     ; preds = %bb39.i75.i.2
  %962 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i, i64 32, !dbg !30733
  %_44.i83.i.2 = load i32, ptr %962, align 4, !dbg !30733, !noalias !30855, !noundef !12
  %_43.i84.i.2 = zext i32 %_44.i83.i.2 to i64, !dbg !30733
  %963 = add i64 %ring_cursor.sroa.0.1.i103215458, %_43.i84.i.2, !dbg !30868
  %_47.not.i85.i.2 = icmp ult i64 %963, %_91.i, !dbg !30869
  %964 = select i1 %_47.not.i85.i.2, i64 0, i64 %_91.i, !dbg !30869
  %spec.select.i86.i.2 = sub nuw i64 %963, %964, !dbg !30869
  %_51.i87.i.2 = mul i64 %spec.select.i86.i.2, %width.i61.i, !dbg !30870
  %_50.i88.i.2 = add i64 %_51.i87.i.2, 2, !dbg !30870
  %_53.i90.i.2 = icmp ult i64 %_50.i88.i.2, %_149.1.i96.i.pre, !dbg !30871
  br i1 %_53.i90.i.2, label %bb21.i92.i.2, label %panic1.i91.i, !dbg !30871

bb21.i92.i.2:                                     ; preds = %bb17.i81.i.2
  %965 = getelementptr inbounds nuw float, ptr %_147.0.i93.i, i64 %_50.i88.i.2, !dbg !30871
  %_49.i94.i.2 = load float, ptr %965, align 4, !dbg !30871, !noalias !30855, !noundef !12
  store float %_49.i94.i.2, ptr %iter.i49.i.sroa.0.0.ptr15444.2, align 4, !dbg !30872, !noalias !30855
  %966 = icmp eq i64 %width.i61.i, 3, !dbg !30730
  br i1 %966, label %bb16.i95.i.loopexit, label %bb39.i75.i.3, !dbg !30730

bb39.i75.i.3:                                     ; preds = %bb21.i92.i.2
  %exitcond18013.3.not = icmp eq i64 %_145.1.i78.i, 3, !dbg !30733
  br i1 %exitcond18013.3.not, label %panic.i80.i, label %bb17.i81.i.3, !dbg !30733

bb17.i81.i.3:                                     ; preds = %bb39.i75.i.3
  %967 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i, i64 44, !dbg !30733
  %_44.i83.i.3 = load i32, ptr %967, align 4, !dbg !30733, !noalias !30855, !noundef !12
  %_43.i84.i.3 = zext i32 %_44.i83.i.3 to i64, !dbg !30733
  %968 = add i64 %ring_cursor.sroa.0.1.i103215458, %_43.i84.i.3, !dbg !30868
  %_47.not.i85.i.3 = icmp ult i64 %968, %_91.i, !dbg !30869
  %969 = select i1 %_47.not.i85.i.3, i64 0, i64 %_91.i, !dbg !30869
  %spec.select.i86.i.3 = sub nuw i64 %968, %969, !dbg !30869
  %_51.i87.i.3 = mul i64 %spec.select.i86.i.3, %width.i61.i, !dbg !30870
  %_50.i88.i.3 = add i64 %_51.i87.i.3, 3, !dbg !30870
  %_53.i90.i.3 = icmp ult i64 %_50.i88.i.3, %_149.1.i96.i.pre, !dbg !30871
  br i1 %_53.i90.i.3, label %bb21.i92.i.3, label %panic1.i91.i, !dbg !30871

bb21.i92.i.3:                                     ; preds = %bb17.i81.i.3
  %970 = getelementptr inbounds nuw float, ptr %_147.0.i93.i, i64 %_50.i88.i.3, !dbg !30871
  %_49.i94.i.3 = load float, ptr %970, align 4, !dbg !30871, !noalias !30855, !noundef !12
  store float %_49.i94.i.3, ptr %iter.i49.i.sroa.0.0.ptr15444.3, align 4, !dbg !30872, !noalias !30855
  %971 = icmp eq i64 %width.i61.i, 4, !dbg !30730
  br i1 %971, label %bb16.i95.i.loopexit, label %bb39.i75.i.4, !dbg !30730

bb39.i75.i.4:                                     ; preds = %bb21.i92.i.3
  %exitcond18013.4.not = icmp eq i64 %_145.1.i78.i, 4, !dbg !30733
  br i1 %exitcond18013.4.not, label %panic.i80.i, label %bb17.i81.i.4, !dbg !30733

bb17.i81.i.4:                                     ; preds = %bb39.i75.i.4
  %972 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i, i64 56, !dbg !30733
  %_44.i83.i.4 = load i32, ptr %972, align 4, !dbg !30733, !noalias !30855, !noundef !12
  %_43.i84.i.4 = zext i32 %_44.i83.i.4 to i64, !dbg !30733
  %973 = add i64 %ring_cursor.sroa.0.1.i103215458, %_43.i84.i.4, !dbg !30868
  %_47.not.i85.i.4 = icmp ult i64 %973, %_91.i, !dbg !30869
  %974 = select i1 %_47.not.i85.i.4, i64 0, i64 %_91.i, !dbg !30869
  %spec.select.i86.i.4 = sub nuw i64 %973, %974, !dbg !30869
  %_51.i87.i.4 = mul i64 %spec.select.i86.i.4, %width.i61.i, !dbg !30870
  %_50.i88.i.4 = add i64 %_51.i87.i.4, 4, !dbg !30870
  %_53.i90.i.4 = icmp ult i64 %_50.i88.i.4, %_149.1.i96.i.pre, !dbg !30871
  br i1 %_53.i90.i.4, label %bb21.i92.i.4, label %panic1.i91.i, !dbg !30871

bb21.i92.i.4:                                     ; preds = %bb17.i81.i.4
  %975 = getelementptr inbounds nuw float, ptr %_147.0.i93.i, i64 %_50.i88.i.4, !dbg !30871
  %_49.i94.i.4 = load float, ptr %975, align 4, !dbg !30871, !noalias !30855, !noundef !12
  store float %_49.i94.i.4, ptr %iter.i49.i.sroa.0.0.ptr15444.4, align 4, !dbg !30872, !noalias !30855
  %976 = icmp eq i64 %width.i61.i, 5, !dbg !30730
  br i1 %976, label %bb16.i95.i.loopexit, label %bb39.i75.i.5, !dbg !30730

bb39.i75.i.5:                                     ; preds = %bb21.i92.i.4
  %exitcond18013.5.not = icmp eq i64 %_145.1.i78.i, 5, !dbg !30733
  br i1 %exitcond18013.5.not, label %panic.i80.i, label %bb17.i81.i.5, !dbg !30733

bb17.i81.i.5:                                     ; preds = %bb39.i75.i.5
  %977 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i, i64 68, !dbg !30733
  %_44.i83.i.5 = load i32, ptr %977, align 4, !dbg !30733, !noalias !30855, !noundef !12
  %_43.i84.i.5 = zext i32 %_44.i83.i.5 to i64, !dbg !30733
  %978 = add i64 %ring_cursor.sroa.0.1.i103215458, %_43.i84.i.5, !dbg !30868
  %_47.not.i85.i.5 = icmp ult i64 %978, %_91.i, !dbg !30869
  %979 = select i1 %_47.not.i85.i.5, i64 0, i64 %_91.i, !dbg !30869
  %spec.select.i86.i.5 = sub nuw i64 %978, %979, !dbg !30869
  %_51.i87.i.5 = mul i64 %spec.select.i86.i.5, %width.i61.i, !dbg !30870
  %_50.i88.i.5 = add i64 %_51.i87.i.5, 5, !dbg !30870
  %_53.i90.i.5 = icmp ult i64 %_50.i88.i.5, %_149.1.i96.i.pre, !dbg !30871
  br i1 %_53.i90.i.5, label %bb21.i92.i.5, label %panic1.i91.i, !dbg !30871

bb21.i92.i.5:                                     ; preds = %bb17.i81.i.5
  %980 = getelementptr inbounds nuw float, ptr %_147.0.i93.i, i64 %_50.i88.i.5, !dbg !30871
  %_49.i94.i.5 = load float, ptr %980, align 4, !dbg !30871, !noalias !30855, !noundef !12
  store float %_49.i94.i.5, ptr %iter.i49.i.sroa.0.0.ptr15444.5, align 4, !dbg !30872, !noalias !30855
  %981 = icmp eq i64 %width.i61.i, 6, !dbg !30730
  br i1 %981, label %bb16.i95.i.loopexit, label %bb39.i75.i.6, !dbg !30730

bb39.i75.i.6:                                     ; preds = %bb21.i92.i.5
  %exitcond18013.6.not = icmp eq i64 %_145.1.i78.i, 6, !dbg !30733
  br i1 %exitcond18013.6.not, label %panic.i80.i, label %bb17.i81.i.6, !dbg !30733

bb17.i81.i.6:                                     ; preds = %bb39.i75.i.6
  %982 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i, i64 80, !dbg !30733
  %_44.i83.i.6 = load i32, ptr %982, align 4, !dbg !30733, !noalias !30855, !noundef !12
  %_43.i84.i.6 = zext i32 %_44.i83.i.6 to i64, !dbg !30733
  %983 = add i64 %ring_cursor.sroa.0.1.i103215458, %_43.i84.i.6, !dbg !30868
  %_47.not.i85.i.6 = icmp ult i64 %983, %_91.i, !dbg !30869
  %984 = select i1 %_47.not.i85.i.6, i64 0, i64 %_91.i, !dbg !30869
  %spec.select.i86.i.6 = sub nuw i64 %983, %984, !dbg !30869
  %_51.i87.i.6 = mul i64 %spec.select.i86.i.6, %width.i61.i, !dbg !30870
  %_50.i88.i.6 = add i64 %_51.i87.i.6, 6, !dbg !30870
  %_53.i90.i.6 = icmp ult i64 %_50.i88.i.6, %_149.1.i96.i.pre, !dbg !30871
  br i1 %_53.i90.i.6, label %bb21.i92.i.6, label %panic1.i91.i, !dbg !30871

bb21.i92.i.6:                                     ; preds = %bb17.i81.i.6
  %985 = getelementptr inbounds nuw float, ptr %_147.0.i93.i, i64 %_50.i88.i.6, !dbg !30871
  %_49.i94.i.6 = load float, ptr %985, align 4, !dbg !30871, !noalias !30855, !noundef !12
  store float %_49.i94.i.6, ptr %iter.i49.i.sroa.0.0.ptr15444.6, align 4, !dbg !30872, !noalias !30855
  %986 = icmp eq i64 %width.i61.i, 7, !dbg !30730
  br i1 %986, label %bb16.i95.i.loopexit, label %bb39.i75.i.7, !dbg !30730

bb39.i75.i.7:                                     ; preds = %bb21.i92.i.6
  %exitcond18013.7.not = icmp eq i64 %_145.1.i78.i, 7, !dbg !30733
  br i1 %exitcond18013.7.not, label %panic.i80.i, label %bb17.i81.i.7, !dbg !30733

bb17.i81.i.7:                                     ; preds = %bb39.i75.i.7
  %987 = getelementptr inbounds nuw i8, ptr %_145.0.i82.i, i64 92, !dbg !30733
  %_44.i83.i.7 = load i32, ptr %987, align 4, !dbg !30733, !noalias !30855, !noundef !12
  %_43.i84.i.7 = zext i32 %_44.i83.i.7 to i64, !dbg !30733
  %988 = add i64 %ring_cursor.sroa.0.1.i103215458, %_43.i84.i.7, !dbg !30868
  %_47.not.i85.i.7 = icmp ult i64 %988, %_91.i, !dbg !30869
  %989 = select i1 %_47.not.i85.i.7, i64 0, i64 %_91.i, !dbg !30869
  %spec.select.i86.i.7 = sub nuw i64 %988, %989, !dbg !30869
  %_51.i87.i.7 = mul i64 %spec.select.i86.i.7, %width.i61.i, !dbg !30870
  %_50.i88.i.7 = add i64 %_51.i87.i.7, 7, !dbg !30870
  %_53.i90.i.7 = icmp ult i64 %_50.i88.i.7, %_149.1.i96.i.pre, !dbg !30871
  br i1 %_53.i90.i.7, label %bb21.i92.i.7, label %panic1.i91.i, !dbg !30871

bb21.i92.i.7:                                     ; preds = %bb17.i81.i.7
  %990 = getelementptr inbounds nuw float, ptr %_147.0.i93.i, i64 %_50.i88.i.7, !dbg !30871
  %_49.i94.i.7 = load float, ptr %990, align 4, !dbg !30871, !noalias !30855, !noundef !12
  store float %_49.i94.i.7, ptr %iter.i49.i.sroa.0.0.ptr15444.7, align 4, !dbg !30872, !noalias !30855
  br label %bb16.i95.i.loopexit, !dbg !30730

panic1.i91.i:                                     ; preds = %bb17.i81.i.7, %bb17.i81.i.6, %bb17.i81.i.5, %bb17.i81.i.4, %bb17.i81.i.3, %bb17.i81.i.2, %bb17.i81.i.1, %bb17.i81.i
  %_50.i88.i.lcssa.ph = phi i64 [ %_50.i88.i.7, %bb17.i81.i.7 ], [ %_50.i88.i.6, %bb17.i81.i.6 ], [ %_50.i88.i.5, %bb17.i81.i.5 ], [ %_50.i88.i.4, %bb17.i81.i.4 ], [ %_50.i88.i.3, %bb17.i81.i.3 ], [ %_50.i88.i.2, %bb17.i81.i.2 ], [ %_50.i88.i.1, %bb17.i81.i.1 ], [ %_51.i87.i, %bb17.i81.i ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i88.i.lcssa.ph, i64 noundef %_149.1.i96.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b305c1483509cfb31fdec21ff8752674) #30, !dbg !30871, !noalias !30855
  unreachable, !dbg !30871

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6687: ; preds = %bb49.i105.i
  %_150.0.i106.i = load ptr, ptr %667, align 8, !dbg !30849, !alias.scope !30586, !noalias !30587, !nonnull !12, !noundef !12
  %_127.i108.i = getelementptr inbounds nuw float, ptr %_150.0.i106.i, i64 %_76.i103.i, !dbg !30873
  %lanes.i5979.sroa.0.0.copyload = load <8 x float>, ptr %_127.i108.i, align 4, !dbg !30875, !alias.scope !30879, !noalias !30883
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_127.i108.i, ptr noundef nonnull align 4 dereferenceable(32) %_174.i, i64 32, i1 false), !dbg !30885
  %991 = fmul <8 x float> %951, %lanes.i5979.sroa.0.0.copyload, !dbg !30890
  %992 = select <8 x i1> %669, <8 x float> %lanes.i5979.sroa.0.0.copyload, <8 x float> %991, !dbg !30895
  store <8 x float> %992, ptr %_174.i, align 4, !dbg !30900, !alias.scope !30905, !noalias !30909
  %_175.i = icmp samesign ugt i64 %base.i1034, %right_io.1, !dbg !30913
  br i1 %_175.i, label %bb54.i1047, label %bb55.i, !dbg !30913, !prof !1406

bb52.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit6027
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1034, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_422064f3ca430d31d9007f55b436c6ca) #30, !dbg !30917, !noalias !29345
  unreachable, !dbg !30917

bb55.i:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6687
  %_177.i = sub nuw nsw i64 %right_io.1, %base.i1034, !dbg !30918
  %_181.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %base.i1034, !dbg !30919
  %_8.i5973 = icmp samesign ugt i64 %_177.i, 7, !dbg !30924
  br i1 %_8.i5973, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5977, label %bb2.i5974, !dbg !30924, !prof !1421

bb2.i5974:                                        ; preds = %bb55.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_177.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !30929, !noalias !30930
  unreachable, !dbg !30929

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5977: ; preds = %bb55.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !30934), !dbg !30937
  %width.i.i = load i64, ptr %670, align 8, !dbg !30938, !alias.scope !30939, !noalias !30940, !noundef !12
  %993 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %905, <8 x float> %_8.i.i968.sroa.0.0.copyload.pre, i8 30), !dbg !30949
  %994 = fdiv <8 x float> %_8.i.i968.sroa.0.0.copyload.pre, %905, !dbg !30955
  %995 = bitcast <8 x float> %993 to <8 x i32>, !dbg !30960
  %996 = icmp slt <8 x i32> %995, zeroinitializer, !dbg !30964
  %997 = select <8 x i1> %996, <8 x float> %994, <8 x float> splat (float 1.000000e+00), !dbg !30964
  %_144.1.i.i = load i64, ptr %671, align 8, !dbg !30966, !alias.scope !30939, !noalias !30940, !noundef !12
  %_22.i.i1040 = mul i64 %width.i.i, %ring_cursor.sroa.0.1.i103215458, !dbg !30967
  %_92.i.i = icmp ugt i64 %_22.i.i1040, %_144.1.i.i, !dbg !30968
  br i1 %_92.i.i, label %bb37.i.i, label %bb38.i.i, !dbg !30968, !prof !1406

bb38.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5977
  %_95.i.i = sub nuw i64 %_144.1.i.i, %_22.i.i1040, !dbg !30971
  %_8.i6679 = icmp samesign ugt i64 %_95.i.i, 7, !dbg !30972
  br i1 %_8.i6679, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6682, label %bb2.i6680, !dbg !30972, !prof !1421

bb2.i6680:                                        ; preds = %bb38.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_95.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !30977, !noalias !30978
  unreachable, !dbg !30977

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6682: ; preds = %bb38.i.i
  %_144.0.i.i = load ptr, ptr %672, align 8, !dbg !30966, !alias.scope !30939, !noalias !30940, !nonnull !12, !noundef !12
  %_99.i.i = getelementptr inbounds nuw float, ptr %_144.0.i.i, i64 %_22.i.i1040, !dbg !30982
  store <8 x float> %997, ptr %_99.i.i, align 4, !dbg !30984, !alias.scope !30988, !noalias !30992
  tail call void @llvm.experimental.noalias.scope.decl(metadata !30994), !dbg !30997
  %width.i2264 = load i64, ptr %670, align 8, !dbg !30998, !alias.scope !30994, !noalias !31000, !noundef !12
  %998 = icmp eq i64 %width.i2264, 0, !dbg !31002
  br i1 %998, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2372, label %bb32.i2271.lr.ph, !dbg !31002

bb32.i2271.lr.ph:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6682
  %_112.1.i2274 = load i64, ptr %673, align 8, !alias.scope !30994, !noalias !31000, !noundef !12
  %_112.0.i2278 = load ptr, ptr %674, align 8, !nonnull !12
  %999 = add i64 %ring_cursor.sroa.0.1.i103215458, 1
  %_23.not.i2285 = icmp ult i64 %999, %_91.i
  %1000 = select i1 %_23.not.i2285, i64 0, i64 %_91.i
  %start1.sroa.0.0.i2286 = sub nuw i64 %999, %1000
  %_114.1.i2289 = load i64, ptr %671, align 8
  %_114.0.i2293 = load ptr, ptr %672, align 8, !nonnull !12
  %_116.1.i2294 = load i64, ptr %675, align 8
  %_116.0.i2298 = load ptr, ptr %676, align 8, !nonnull !12
  %_118.1.i2302 = load i64, ptr %677, align 8
  %_118.0.i2306 = load ptr, ptr %678, align 8, !nonnull !12
  %_45.i2319 = mul i64 %width.i2264, %start1.sroa.0.0.i2286
  br label %bb32.i2271, !dbg !31002

bb32.i2271:                                       ; preds = %bb32.i2271.lr.ph, %bb31.i2334
  %iter.i2263.sroa.10.015450 = phi i64 [ %width.i2264, %bb32.i2271.lr.ph ], [ %1001, %bb31.i2334 ]
  %iter.i2263.sroa.7.015449 = phi i64 [ 0, %bb32.i2271.lr.ph ], [ %_9.0.i7481, %bb31.i2334 ]
  %iter.i2263.sroa.0.0.idx15448 = phi i64 [ 0, %bb32.i2271.lr.ph ], [ %iter.i2263.sroa.0.0.add, %bb31.i2334 ]
  %iter.i2263.sroa.0.0.ptr15451 = getelementptr inbounds nuw i8, ptr %scratch.i, i64 %iter.i2263.sroa.0.0.idx15448, !dbg !31004
  %1001 = add i64 %iter.i2263.sroa.10.015450, -1, !dbg !31004
  %_7.i.i7477 = icmp eq i64 %iter.i2263.sroa.0.0.idx15448, 32, !dbg !31005
  br i1 %_7.i.i7477, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2372.loopexit, label %bb3.i2273, !dbg !31009

bb3.i2273:                                        ; preds = %bb32.i2271
  %iter.i2263.sroa.0.0.add = add nuw nsw i64 %iter.i2263.sroa.0.0.idx15448, 4, !dbg !31010
  %_9.0.i7481 = add nuw nsw i64 %iter.i2263.sroa.7.015449, 1, !dbg !31012
  %exitcond18019.not = icmp eq i64 %iter.i2263.sroa.7.015449, %_112.1.i2274, !dbg !31013
  br i1 %exitcond18019.not, label %panic.i2276, label %bb5.i2277, !dbg !31013

bb5.i2277:                                        ; preds = %bb3.i2273
  %1002 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i2278, i64 %iter.i2263.sroa.7.015449, !dbg !31013
  %shape.i2279 = load i32, ptr %1002, align 4, !dbg !31013, !noalias !31014, !noundef !12
  %1003 = getelementptr inbounds nuw i8, ptr %1002, i64 4, !dbg !31013
  %shape3.i2280 = load i32, ptr %1003, align 4, !dbg !31013, !noalias !31014, !noundef !12
  %window.i2281 = zext i32 %shape.i2279 to i64, !dbg !31015
  %_19.i2282 = zext i32 %shape3.i2280 to i64, !dbg !31016
  %1004 = add i64 %ring_cursor.sroa.0.1.i103215458, %_19.i2282, !dbg !31017
  %_20.not.i2283 = icmp ult i64 %1004, %_91.i, !dbg !31018
  %1005 = select i1 %_20.not.i2283, i64 0, i64 %_91.i, !dbg !31018
  %spec.select.i2284 = sub nuw i64 %1004, %1005, !dbg !31018
  %_27.i2287 = mul i64 %spec.select.i2284, %width.i2264, !dbg !31019
  %_26.i2288 = add i64 %_27.i2287, %iter.i2263.sroa.7.015449, !dbg !31019
  %_30.i2290 = icmp ult i64 %_26.i2288, %_114.1.i2289, !dbg !31020
  br i1 %_30.i2290, label %bb12.i2292, label %panic5.i2291, !dbg !31020

panic.i2276:                                      ; preds = %bb3.i2273
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i2274, i64 noundef %_112.1.i2274, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4a8785a681d008a9bfd0cd82628ea9cb) #30, !dbg !31013, !noalias !31014
  unreachable, !dbg !31013

bb12.i2292:                                       ; preds = %bb5.i2277
  %1006 = getelementptr inbounds nuw float, ptr %_114.0.i2293, i64 %_26.i2288, !dbg !31020
  %1007 = load float, ptr %1006, align 4, !dbg !31020, !noalias !31014, !noundef !12
  %exitcond18020.not = icmp eq i64 %iter.i2263.sroa.7.015449, %_116.1.i2294, !dbg !31021
  br i1 %exitcond18020.not, label %panic6.i2296, label %bb13.i2297, !dbg !31021

panic5.i2291:                                     ; preds = %bb5.i2277
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i2288, i64 noundef %_114.1.i2289, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cbce7773ac40979e4ba2385da3aec116) #30, !dbg !31020, !noalias !31014
  unreachable, !dbg !31020

bb13.i2297:                                       ; preds = %bb12.i2292
  %1008 = getelementptr inbounds nuw i32, ptr %_116.0.i2298, i64 %iter.i2263.sroa.7.015449, !dbg !31021
  %_32.i2299 = load i32, ptr %1008, align 4, !dbg !31021, !noalias !31014, !noundef !12
  %position.i2300 = zext i32 %_32.i2299 to i64, !dbg !31021
  %1009 = icmp eq i32 %_32.i2299, 0, !dbg !31022
  br i1 %1009, label %bb17.i2309, label %bb15.i2301, !dbg !31022

panic6.i2296:                                     ; preds = %bb12.i2292
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i2294, i64 noundef %_116.1.i2294, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ec0d48f73ebfc2755df5cedaa60b5c0a) #30, !dbg !31021, !noalias !31014
  unreachable, !dbg !31021

bb15.i2301:                                       ; preds = %bb13.i2297
  %_37.i2303 = icmp ult i64 %iter.i2263.sroa.7.015449, %_118.1.i2302, !dbg !31023
  br i1 %_37.i2303, label %bb16.i2305, label %panic7.i2304, !dbg !31023

bb17.i2309:                                       ; preds = %bb35.i2370, %bb16.i2305, %bb13.i2297
  %newest.sroa.0.0.i2310 = phi float [ %1007, %bb13.i2297 ], [ %_35.i2307, %bb35.i2370 ], [ %1007, %bb16.i2305 ], !dbg !31024
  %exitcond18021.not = icmp eq i64 %iter.i2263.sroa.7.015449, %_118.1.i2302, !dbg !31025
  br i1 %exitcond18021.not, label %panic8.i2313, label %bb18.i2314, !dbg !31025

bb16.i2305:                                       ; preds = %bb15.i2301
  %1010 = getelementptr inbounds nuw float, ptr %_118.0.i2306, i64 %iter.i2263.sroa.7.015449, !dbg !31023
  %_35.i2307 = load float, ptr %1010, align 4, !dbg !31023, !noalias !31014, !noundef !12
  %_102.i2308 = fcmp olt float %_35.i2307, %1007, !dbg !31026
  br i1 %_102.i2308, label %bb35.i2370, label %bb17.i2309, !dbg !31026

panic7.i2304:                                     ; preds = %bb15.i2301
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i2263.sroa.7.015449, i64 noundef %_118.1.i2302, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2c461872bb652d4796cdcf89c28c82c8) #30, !dbg !31023, !noalias !31014
  unreachable, !dbg !31023

bb35.i2370:                                       ; preds = %bb16.i2305
  br label %bb17.i2309, !dbg !31028

bb18.i2314:                                       ; preds = %bb17.i2309
  %1011 = getelementptr inbounds nuw float, ptr %_118.0.i2306, i64 %iter.i2263.sroa.7.015449, !dbg !31025
  store float %newest.sroa.0.0.i2310, ptr %1011, align 4, !dbg !31025, !noalias !31014
  %_42.i2316 = add nuw nsw i64 %position.i2300, 1, !dbg !31029
  %complete.i2317 = icmp eq i64 %_42.i2316, %window.i2281, !dbg !31029
  br i1 %complete.i2317, label %bb22.i2339, label %bb20.i2318, !dbg !31030

panic8.i2313:                                     ; preds = %bb17.i2309
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i2302, i64 noundef %_118.1.i2302, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2b690e2c7763f11809942906fc2ca813) #30, !dbg !31025, !noalias !31014
  unreachable, !dbg !31025

bb20.i2318:                                       ; preds = %bb18.i2314
  %_44.i2320 = add i64 %iter.i2263.sroa.7.015449, %_45.i2319, !dbg !31031
  %_47.i2322 = icmp ult i64 %_44.i2320, %_114.1.i2289, !dbg !31032
  br i1 %_47.i2322, label %bb30.i2332, label %panic9.i2323, !dbg !31032

panic9.i2323:                                     ; preds = %bb20.i2318
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i2320, i64 noundef %_114.1.i2289, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9fa421ae81817f58fcfc4a3243223891) #30, !dbg !31032, !noalias !31014
  unreachable, !dbg !31032

bb30.i2332:                                       ; preds = %bb20.i2318
  %1012 = getelementptr inbounds nuw float, ptr %_114.0.i2293, i64 %_44.i2320, !dbg !31032
  %_43.i2326 = load float, ptr %1012, align 4, !dbg !31032, !noalias !31014, !noundef !12
  %_103.i2327 = fcmp olt float %_43.i2326, %newest.sroa.0.0.i2310, !dbg !31033
  %newest.sroa.0.1.i2328 = select i1 %_103.i2327, float %_43.i2326, float %newest.sroa.0.0.i2310, !dbg !31033
  store float %newest.sroa.0.1.i2328, ptr %iter.i2263.sroa.0.0.ptr15451, align 4, !dbg !31035, !noalias !31014
  %1013 = trunc i64 %_42.i2316 to i32, !dbg !31036
  br label %bb31.i2334, !dbg !31037

bb31.i2334:                                       ; preds = %bb25.i2367, %bb30.i2332
  %storemerge13426 = phi i32 [ %1013, %bb30.i2332 ], [ 0, %bb25.i2367 ], !dbg !31038
  store i32 %storemerge13426, ptr %1008, align 4, !dbg !31038, !noalias !31014
  %1014 = icmp eq i64 %1001, 0, !dbg !31002
  br i1 %1014, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2372.loopexit, label %bb32.i2271, !dbg !31002

bb22.i2339:                                       ; preds = %bb18.i2314
  store float %newest.sroa.0.0.i2310, ptr %iter.i2263.sroa.0.0.ptr15451, align 4, !dbg !31035, !noalias !31014
  %1015 = load float, ptr %1006, align 4, !dbg !31039, !noalias !31014, !noundef !12
  br label %bb41.i2352, !dbg !31040

bb41.i2352:                                       ; preds = %bb22.i2339, %bb25.i2367
  %iter2.sroa.0.0.i234415447 = phi i64 [ 0, %bb22.i2339 ], [ %_105.i2353, %bb25.i2367 ]
  %suffix.sroa.0.0.i234315446 = phi float [ %1015, %bb22.i2339 ], [ %suffix.sroa.0.1.i2363, %bb25.i2367 ]
  %end.sroa.0.1.i234215445 = phi i64 [ %spec.select.i2284, %bb22.i2339 ], [ %1018, %bb25.i2367 ]
  %_56.i2354 = mul i64 %end.sroa.0.1.i234215445, %width.i2264, !dbg !31043
  %_55.i2355 = add i64 %_56.i2354, %iter.i2263.sroa.7.015449, !dbg !31043
  %_59.i2357 = icmp ult i64 %_55.i2355, %_114.1.i2289, !dbg !31044
  br i1 %_59.i2357, label %bb25.i2367, label %panic13.i2358, !dbg !31044

panic13.i2358:                                    ; preds = %bb41.i2352
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i2355, i64 noundef %_114.1.i2289, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_91a4c6b9b17ebf4d863f9a70b6dc929a) #30, !dbg !31044, !noalias !31014
  unreachable, !dbg !31044

bb25.i2367:                                       ; preds = %bb41.i2352
  %_105.i2353 = add nuw nsw i64 %iter2.sroa.0.0.i234415447, 1, !dbg !31045
  %1016 = getelementptr inbounds nuw float, ptr %_114.0.i2293, i64 %_55.i2355, !dbg !31044
  %_54.i2361 = load float, ptr %1016, align 4, !dbg !31044, !noalias !31014, !noundef !12
  %_107.i2362 = fcmp olt float %suffix.sroa.0.0.i234315446, %_54.i2361, !dbg !31048
  %suffix.sroa.0.1.i2363 = select i1 %_107.i2362, float %suffix.sroa.0.0.i234315446, float %_54.i2361, !dbg !31048
  store float %suffix.sroa.0.1.i2363, ptr %1016, align 4, !dbg !31050, !noalias !31014
  %1017 = icmp eq i64 %end.sroa.0.1.i234215445, 0, !dbg !31051
  %spec.store.select.i2369 = select i1 %1017, i64 %_91.i, i64 %end.sroa.0.1.i234215445, !dbg !31051
  %1018 = add i64 %spec.store.select.i2369, -1, !dbg !31052
  %exitcond18018.not = icmp eq i64 %_105.i2353, %window.i2281, !dbg !31053
  br i1 %exitcond18018.not, label %bb31.i2334, label %bb41.i2352, !dbg !31040

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2372.loopexit: ; preds = %bb32.i2271, %bb31.i2334
  %lanes.i5963.sroa.0.0.copyload.pre = load <8 x float>, ptr %scratch.i, align 4, !dbg !31055, !alias.scope !31060, !noalias !31064
  br label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2372, !dbg !31068

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2372: ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2372.loopexit, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6682
  %lanes.i5963.sroa.0.0.copyload = phi <8 x float> [ %lanes.i5963.sroa.0.0.copyload.pre, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2372.loopexit ], [ %lanes.i5988.sroa.0.0.copyload, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6682 ], !dbg !31055
  %1019 = fmul <8 x float> %lanes.i5963.sroa.0.0.copyload, splat (float 1.638400e+04), !dbg !31069
  %1020 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %1019), !dbg !31074
  %1021 = fmul <8 x float> %1020, splat (float 0x3F10000000000000), !dbg !31079
  %1022 = icmp eq i64 %width.i.i, 0, !dbg !31084
  %_149.1.i.i.pre = load i64, ptr %679, align 8, !dbg !31086, !alias.scope !30939, !noalias !30940
  br i1 %1022, label %bb16.i.i, label %bb39.i.i.lr.ph, !dbg !31084

bb39.i.i.lr.ph:                                   ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2372
  %_145.1.i.i = load i64, ptr %673, align 8, !alias.scope !30939, !noalias !30940, !noundef !12
  %_145.0.i.i = load ptr, ptr %674, align 8, !nonnull !12
  %_147.0.i.i = load ptr, ptr %680, align 8, !nonnull !12
  %exitcond18024.not = icmp eq i64 %_145.1.i.i, 0, !dbg !31087
  br i1 %exitcond18024.not, label %panic.i.i, label %bb17.i.i, !dbg !31087

bb37.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit5977
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i1040, i64 noundef %_144.1.i.i, i64 noundef %_144.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_56df7c041d29359441bca272bf4e38e3) #30, !dbg !31088, !noalias !31089
  unreachable, !dbg !31088

bb16.i.i.loopexit:                                ; preds = %bb21.i.i.7, %bb21.i.i.6, %bb21.i.i.5, %bb21.i.i.4, %bb21.i.i.3, %bb21.i.i.2, %bb21.i.i.1, %bb21.i.i
  %lanes.i5956.sroa.0.0.copyload.pre = load <8 x float>, ptr %scratch.i, align 4, !dbg !31090, !alias.scope !31095, !noalias !31099
  br label %bb16.i.i, !dbg !31103

bb16.i.i:                                         ; preds = %bb16.i.i.loopexit, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2372
  %lanes.i5956.sroa.0.0.copyload = phi <8 x float> [ %lanes.i5956.sroa.0.0.copyload.pre, %bb16.i.i.loopexit ], [ %lanes.i5963.sroa.0.0.copyload, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit2372 ], !dbg !31090
  %1023 = fadd <8 x float> %1021, %_58.i.i.sroa.0.0.copyload15536, !dbg !31104
  %1024 = fsub <8 x float> %1023, %lanes.i5956.sroa.0.0.copyload, !dbg !31109
  %_109.i.i = icmp ugt i64 %_22.i.i1040, %_149.1.i.i.pre, !dbg !31114
  br i1 %_109.i.i, label %bb42.i.i, label %bb43.i.i, !dbg !31114, !prof !1406

bb43.i.i:                                         ; preds = %bb16.i.i
  %_112.i.i = sub nuw i64 %_149.1.i.i.pre, %_22.i.i1040, !dbg !31117
  %_8.i6674 = icmp samesign ugt i64 %_112.i.i, 7, !dbg !31118
  br i1 %_8.i6674, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6677, label %bb2.i6675, !dbg !31118, !prof !1421

bb2.i6675:                                        ; preds = %bb43.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_112.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !31123, !noalias !31124
  unreachable, !dbg !31123

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6677: ; preds = %bb43.i.i
  %_149.0.i.i = load ptr, ptr %680, align 8, !dbg !31086, !alias.scope !30939, !noalias !30940, !nonnull !12, !noundef !12
  %_116.i.i = getelementptr inbounds nuw float, ptr %_149.0.i.i, i64 %_22.i.i1040, !dbg !31128
  store <8 x float> %1021, ptr %_116.i.i, align 4, !dbg !31130, !alias.scope !31134, !noalias !31138
  %_68.i.i.sroa.0.0.copyload = load <8 x float>, ptr %683, align 32, !dbg !31140
  %1025 = fdiv <8 x float> %1024, %_64.i.i.sroa.0.0.copyload, !dbg !31141
  %1026 = fsub <8 x float> splat (float 1.000000e+00), %1025, !dbg !31146
  %1027 = fsub <8 x float> %1026, %_68.i.i.sroa.0.0.copyload, !dbg !31151
  %1028 = fmul <8 x float> %_9.i.i967.sroa.0.0.copyload.pre, %1027, !dbg !31156
  %1029 = fadd <8 x float> %_68.i.i.sroa.0.0.copyload, %1028, !dbg !31161
  %1030 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1026, <8 x float> %1029), !dbg !31165
  %1031 = bitcast <8 x float> %1030 to <8 x i32>, !dbg !31170
  %1032 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1030), !dbg !31176
  %1033 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1032, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !31178
  %1034 = bitcast <8 x float> %1033 to <8 x i32>, !dbg !31184
  %1035 = xor <8 x i32> %1034, splat (i32 -1), !dbg !31190
  %1036 = and <8 x i32> %1035, %1031, !dbg !31192
  %1037 = bitcast <8 x i32> %1036 to <8 x float>, !dbg !31196
  store <8 x i32> %1036, ptr %683, align 32, !dbg !31197
  %1038 = fsub <8 x float> splat (float 1.000000e+00), %1037, !dbg !31198
  %_150.1.i.i = load i64, ptr %684, align 8, !dbg !31203, !alias.scope !30939, !noalias !30940, !noundef !12
  %_76.i.i = mul i64 %width.i.i, %main_cursor.sroa.0.1.i103315459, !dbg !31204
  %_120.i.i = icmp ugt i64 %_76.i.i, %_150.1.i.i, !dbg !31205
  br i1 %_120.i.i, label %bb48.i.i, label %bb49.i.i, !dbg !31205, !prof !1406

bb42.i.i:                                         ; preds = %bb16.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i1040, i64 noundef %_149.1.i.i.pre, i64 noundef %_149.1.i.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_90498045d73339daaf9e4f537508f58b) #30, !dbg !31208, !noalias !31209
  unreachable, !dbg !31208

bb49.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6677
  %_123.i.i = sub nuw i64 %_150.1.i.i, %_76.i.i, !dbg !31210
  %_8.i5950 = icmp samesign ugt i64 %_123.i.i, 7, !dbg !31211
  br i1 %_8.i5950, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6667, label %bb2.i5951, !dbg !31211, !prof !1421

bb2.i5951:                                        ; preds = %bb49.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_123.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !31216, !noalias !31217
  unreachable, !dbg !31216

bb48.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6677
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i.i, i64 noundef %_150.1.i.i, i64 noundef %_150.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_da8af254b6d507a8e2ca31e544bfd21d) #30, !dbg !31221, !noalias !31209
  unreachable, !dbg !31221

bb17.i.i:                                         ; preds = %bb39.i.i.lr.ph
  %1039 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 8, !dbg !31087
  %_44.i.i1043 = load i32, ptr %1039, align 4, !dbg !31087, !noalias !31209, !noundef !12
  %_43.i.i1044 = zext i32 %_44.i.i1043 to i64, !dbg !31087
  %1040 = add i64 %ring_cursor.sroa.0.1.i103215458, %_43.i.i1044, !dbg !31222
  %_47.not.i.i = icmp ult i64 %1040, %_91.i, !dbg !31223
  %1041 = select i1 %_47.not.i.i, i64 0, i64 %_91.i, !dbg !31223
  %spec.select.i.i = sub nuw i64 %1040, %1041, !dbg !31223
  %_51.i.i = mul i64 %spec.select.i.i, %width.i.i, !dbg !31224
  %_53.i.i1045 = icmp ult i64 %_51.i.i, %_149.1.i.i.pre, !dbg !31225
  br i1 %_53.i.i1045, label %bb21.i.i, label %panic1.i.i, !dbg !31225

panic.i.i:                                        ; preds = %bb39.i.i.7, %bb39.i.i.6, %bb39.i.i.5, %bb39.i.i.4, %bb39.i.i.3, %bb39.i.i.2, %bb39.i.i.1, %bb39.i.i.lr.ph
  %_145.1.i.i.lcssa.ph = phi i64 [ 7, %bb39.i.i.7 ], [ 6, %bb39.i.i.6 ], [ 5, %bb39.i.i.5 ], [ 4, %bb39.i.i.4 ], [ 3, %bb39.i.i.3 ], [ 2, %bb39.i.i.2 ], [ 1, %bb39.i.i.1 ], [ 0, %bb39.i.i.lr.ph ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i.i.lcssa.ph, i64 noundef %_145.1.i.i.lcssa.ph, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a6feb40b34112df5f214f84424dd2c7c) #30, !dbg !31087, !noalias !31209
  unreachable, !dbg !31087

bb21.i.i:                                         ; preds = %bb17.i.i
  %1042 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_51.i.i, !dbg !31225
  %_49.i.i1046 = load float, ptr %1042, align 4, !dbg !31225, !noalias !31209, !noundef !12
  store float %_49.i.i1046, ptr %scratch.i, align 4, !dbg !31226, !noalias !31209
  %1043 = icmp eq i64 %width.i.i, 1, !dbg !31084
  br i1 %1043, label %bb16.i.i.loopexit, label %bb39.i.i.1, !dbg !31084

bb39.i.i.1:                                       ; preds = %bb21.i.i
  %exitcond18024.1.not = icmp eq i64 %_145.1.i.i, 1, !dbg !31087
  br i1 %exitcond18024.1.not, label %panic.i.i, label %bb17.i.i.1, !dbg !31087

bb17.i.i.1:                                       ; preds = %bb39.i.i.1
  %1044 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 20, !dbg !31087
  %_44.i.i1043.1 = load i32, ptr %1044, align 4, !dbg !31087, !noalias !31209, !noundef !12
  %_43.i.i1044.1 = zext i32 %_44.i.i1043.1 to i64, !dbg !31087
  %1045 = add i64 %ring_cursor.sroa.0.1.i103215458, %_43.i.i1044.1, !dbg !31222
  %_47.not.i.i.1 = icmp ult i64 %1045, %_91.i, !dbg !31223
  %1046 = select i1 %_47.not.i.i.1, i64 0, i64 %_91.i, !dbg !31223
  %spec.select.i.i.1 = sub nuw i64 %1045, %1046, !dbg !31223
  %_51.i.i.1 = mul i64 %spec.select.i.i.1, %width.i.i, !dbg !31224
  %_50.i.i.1 = add i64 %_51.i.i.1, 1, !dbg !31224
  %_53.i.i1045.1 = icmp ult i64 %_50.i.i.1, %_149.1.i.i.pre, !dbg !31225
  br i1 %_53.i.i1045.1, label %bb21.i.i.1, label %panic1.i.i, !dbg !31225

bb21.i.i.1:                                       ; preds = %bb17.i.i.1
  %1047 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.1, !dbg !31225
  %_49.i.i1046.1 = load float, ptr %1047, align 4, !dbg !31225, !noalias !31209, !noundef !12
  store float %_49.i.i1046.1, ptr %iter.i.i958.sroa.0.0.ptr15455.1, align 4, !dbg !31226, !noalias !31209
  %1048 = icmp eq i64 %width.i.i, 2, !dbg !31084
  br i1 %1048, label %bb16.i.i.loopexit, label %bb39.i.i.2, !dbg !31084

bb39.i.i.2:                                       ; preds = %bb21.i.i.1
  %exitcond18024.2.not = icmp eq i64 %_145.1.i.i, 2, !dbg !31087
  br i1 %exitcond18024.2.not, label %panic.i.i, label %bb17.i.i.2, !dbg !31087

bb17.i.i.2:                                       ; preds = %bb39.i.i.2
  %1049 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 32, !dbg !31087
  %_44.i.i1043.2 = load i32, ptr %1049, align 4, !dbg !31087, !noalias !31209, !noundef !12
  %_43.i.i1044.2 = zext i32 %_44.i.i1043.2 to i64, !dbg !31087
  %1050 = add i64 %ring_cursor.sroa.0.1.i103215458, %_43.i.i1044.2, !dbg !31222
  %_47.not.i.i.2 = icmp ult i64 %1050, %_91.i, !dbg !31223
  %1051 = select i1 %_47.not.i.i.2, i64 0, i64 %_91.i, !dbg !31223
  %spec.select.i.i.2 = sub nuw i64 %1050, %1051, !dbg !31223
  %_51.i.i.2 = mul i64 %spec.select.i.i.2, %width.i.i, !dbg !31224
  %_50.i.i.2 = add i64 %_51.i.i.2, 2, !dbg !31224
  %_53.i.i1045.2 = icmp ult i64 %_50.i.i.2, %_149.1.i.i.pre, !dbg !31225
  br i1 %_53.i.i1045.2, label %bb21.i.i.2, label %panic1.i.i, !dbg !31225

bb21.i.i.2:                                       ; preds = %bb17.i.i.2
  %1052 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.2, !dbg !31225
  %_49.i.i1046.2 = load float, ptr %1052, align 4, !dbg !31225, !noalias !31209, !noundef !12
  store float %_49.i.i1046.2, ptr %iter.i.i958.sroa.0.0.ptr15455.2, align 4, !dbg !31226, !noalias !31209
  %1053 = icmp eq i64 %width.i.i, 3, !dbg !31084
  br i1 %1053, label %bb16.i.i.loopexit, label %bb39.i.i.3, !dbg !31084

bb39.i.i.3:                                       ; preds = %bb21.i.i.2
  %exitcond18024.3.not = icmp eq i64 %_145.1.i.i, 3, !dbg !31087
  br i1 %exitcond18024.3.not, label %panic.i.i, label %bb17.i.i.3, !dbg !31087

bb17.i.i.3:                                       ; preds = %bb39.i.i.3
  %1054 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 44, !dbg !31087
  %_44.i.i1043.3 = load i32, ptr %1054, align 4, !dbg !31087, !noalias !31209, !noundef !12
  %_43.i.i1044.3 = zext i32 %_44.i.i1043.3 to i64, !dbg !31087
  %1055 = add i64 %ring_cursor.sroa.0.1.i103215458, %_43.i.i1044.3, !dbg !31222
  %_47.not.i.i.3 = icmp ult i64 %1055, %_91.i, !dbg !31223
  %1056 = select i1 %_47.not.i.i.3, i64 0, i64 %_91.i, !dbg !31223
  %spec.select.i.i.3 = sub nuw i64 %1055, %1056, !dbg !31223
  %_51.i.i.3 = mul i64 %spec.select.i.i.3, %width.i.i, !dbg !31224
  %_50.i.i.3 = add i64 %_51.i.i.3, 3, !dbg !31224
  %_53.i.i1045.3 = icmp ult i64 %_50.i.i.3, %_149.1.i.i.pre, !dbg !31225
  br i1 %_53.i.i1045.3, label %bb21.i.i.3, label %panic1.i.i, !dbg !31225

bb21.i.i.3:                                       ; preds = %bb17.i.i.3
  %1057 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.3, !dbg !31225
  %_49.i.i1046.3 = load float, ptr %1057, align 4, !dbg !31225, !noalias !31209, !noundef !12
  store float %_49.i.i1046.3, ptr %iter.i.i958.sroa.0.0.ptr15455.3, align 4, !dbg !31226, !noalias !31209
  %1058 = icmp eq i64 %width.i.i, 4, !dbg !31084
  br i1 %1058, label %bb16.i.i.loopexit, label %bb39.i.i.4, !dbg !31084

bb39.i.i.4:                                       ; preds = %bb21.i.i.3
  %exitcond18024.4.not = icmp eq i64 %_145.1.i.i, 4, !dbg !31087
  br i1 %exitcond18024.4.not, label %panic.i.i, label %bb17.i.i.4, !dbg !31087

bb17.i.i.4:                                       ; preds = %bb39.i.i.4
  %1059 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 56, !dbg !31087
  %_44.i.i1043.4 = load i32, ptr %1059, align 4, !dbg !31087, !noalias !31209, !noundef !12
  %_43.i.i1044.4 = zext i32 %_44.i.i1043.4 to i64, !dbg !31087
  %1060 = add i64 %ring_cursor.sroa.0.1.i103215458, %_43.i.i1044.4, !dbg !31222
  %_47.not.i.i.4 = icmp ult i64 %1060, %_91.i, !dbg !31223
  %1061 = select i1 %_47.not.i.i.4, i64 0, i64 %_91.i, !dbg !31223
  %spec.select.i.i.4 = sub nuw i64 %1060, %1061, !dbg !31223
  %_51.i.i.4 = mul i64 %spec.select.i.i.4, %width.i.i, !dbg !31224
  %_50.i.i.4 = add i64 %_51.i.i.4, 4, !dbg !31224
  %_53.i.i1045.4 = icmp ult i64 %_50.i.i.4, %_149.1.i.i.pre, !dbg !31225
  br i1 %_53.i.i1045.4, label %bb21.i.i.4, label %panic1.i.i, !dbg !31225

bb21.i.i.4:                                       ; preds = %bb17.i.i.4
  %1062 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.4, !dbg !31225
  %_49.i.i1046.4 = load float, ptr %1062, align 4, !dbg !31225, !noalias !31209, !noundef !12
  store float %_49.i.i1046.4, ptr %iter.i.i958.sroa.0.0.ptr15455.4, align 4, !dbg !31226, !noalias !31209
  %1063 = icmp eq i64 %width.i.i, 5, !dbg !31084
  br i1 %1063, label %bb16.i.i.loopexit, label %bb39.i.i.5, !dbg !31084

bb39.i.i.5:                                       ; preds = %bb21.i.i.4
  %exitcond18024.5.not = icmp eq i64 %_145.1.i.i, 5, !dbg !31087
  br i1 %exitcond18024.5.not, label %panic.i.i, label %bb17.i.i.5, !dbg !31087

bb17.i.i.5:                                       ; preds = %bb39.i.i.5
  %1064 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 68, !dbg !31087
  %_44.i.i1043.5 = load i32, ptr %1064, align 4, !dbg !31087, !noalias !31209, !noundef !12
  %_43.i.i1044.5 = zext i32 %_44.i.i1043.5 to i64, !dbg !31087
  %1065 = add i64 %ring_cursor.sroa.0.1.i103215458, %_43.i.i1044.5, !dbg !31222
  %_47.not.i.i.5 = icmp ult i64 %1065, %_91.i, !dbg !31223
  %1066 = select i1 %_47.not.i.i.5, i64 0, i64 %_91.i, !dbg !31223
  %spec.select.i.i.5 = sub nuw i64 %1065, %1066, !dbg !31223
  %_51.i.i.5 = mul i64 %spec.select.i.i.5, %width.i.i, !dbg !31224
  %_50.i.i.5 = add i64 %_51.i.i.5, 5, !dbg !31224
  %_53.i.i1045.5 = icmp ult i64 %_50.i.i.5, %_149.1.i.i.pre, !dbg !31225
  br i1 %_53.i.i1045.5, label %bb21.i.i.5, label %panic1.i.i, !dbg !31225

bb21.i.i.5:                                       ; preds = %bb17.i.i.5
  %1067 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.5, !dbg !31225
  %_49.i.i1046.5 = load float, ptr %1067, align 4, !dbg !31225, !noalias !31209, !noundef !12
  store float %_49.i.i1046.5, ptr %iter.i.i958.sroa.0.0.ptr15455.5, align 4, !dbg !31226, !noalias !31209
  %1068 = icmp eq i64 %width.i.i, 6, !dbg !31084
  br i1 %1068, label %bb16.i.i.loopexit, label %bb39.i.i.6, !dbg !31084

bb39.i.i.6:                                       ; preds = %bb21.i.i.5
  %exitcond18024.6.not = icmp eq i64 %_145.1.i.i, 6, !dbg !31087
  br i1 %exitcond18024.6.not, label %panic.i.i, label %bb17.i.i.6, !dbg !31087

bb17.i.i.6:                                       ; preds = %bb39.i.i.6
  %1069 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 80, !dbg !31087
  %_44.i.i1043.6 = load i32, ptr %1069, align 4, !dbg !31087, !noalias !31209, !noundef !12
  %_43.i.i1044.6 = zext i32 %_44.i.i1043.6 to i64, !dbg !31087
  %1070 = add i64 %ring_cursor.sroa.0.1.i103215458, %_43.i.i1044.6, !dbg !31222
  %_47.not.i.i.6 = icmp ult i64 %1070, %_91.i, !dbg !31223
  %1071 = select i1 %_47.not.i.i.6, i64 0, i64 %_91.i, !dbg !31223
  %spec.select.i.i.6 = sub nuw i64 %1070, %1071, !dbg !31223
  %_51.i.i.6 = mul i64 %spec.select.i.i.6, %width.i.i, !dbg !31224
  %_50.i.i.6 = add i64 %_51.i.i.6, 6, !dbg !31224
  %_53.i.i1045.6 = icmp ult i64 %_50.i.i.6, %_149.1.i.i.pre, !dbg !31225
  br i1 %_53.i.i1045.6, label %bb21.i.i.6, label %panic1.i.i, !dbg !31225

bb21.i.i.6:                                       ; preds = %bb17.i.i.6
  %1072 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.6, !dbg !31225
  %_49.i.i1046.6 = load float, ptr %1072, align 4, !dbg !31225, !noalias !31209, !noundef !12
  store float %_49.i.i1046.6, ptr %iter.i.i958.sroa.0.0.ptr15455.6, align 4, !dbg !31226, !noalias !31209
  %1073 = icmp eq i64 %width.i.i, 7, !dbg !31084
  br i1 %1073, label %bb16.i.i.loopexit, label %bb39.i.i.7, !dbg !31084

bb39.i.i.7:                                       ; preds = %bb21.i.i.6
  %exitcond18024.7.not = icmp eq i64 %_145.1.i.i, 7, !dbg !31087
  br i1 %exitcond18024.7.not, label %panic.i.i, label %bb17.i.i.7, !dbg !31087

bb17.i.i.7:                                       ; preds = %bb39.i.i.7
  %1074 = getelementptr inbounds nuw i8, ptr %_145.0.i.i, i64 92, !dbg !31087
  %_44.i.i1043.7 = load i32, ptr %1074, align 4, !dbg !31087, !noalias !31209, !noundef !12
  %_43.i.i1044.7 = zext i32 %_44.i.i1043.7 to i64, !dbg !31087
  %1075 = add i64 %ring_cursor.sroa.0.1.i103215458, %_43.i.i1044.7, !dbg !31222
  %_47.not.i.i.7 = icmp ult i64 %1075, %_91.i, !dbg !31223
  %1076 = select i1 %_47.not.i.i.7, i64 0, i64 %_91.i, !dbg !31223
  %spec.select.i.i.7 = sub nuw i64 %1075, %1076, !dbg !31223
  %_51.i.i.7 = mul i64 %spec.select.i.i.7, %width.i.i, !dbg !31224
  %_50.i.i.7 = add i64 %_51.i.i.7, 7, !dbg !31224
  %_53.i.i1045.7 = icmp ult i64 %_50.i.i.7, %_149.1.i.i.pre, !dbg !31225
  br i1 %_53.i.i1045.7, label %bb21.i.i.7, label %panic1.i.i, !dbg !31225

bb21.i.i.7:                                       ; preds = %bb17.i.i.7
  %1077 = getelementptr inbounds nuw float, ptr %_147.0.i.i, i64 %_50.i.i.7, !dbg !31225
  %_49.i.i1046.7 = load float, ptr %1077, align 4, !dbg !31225, !noalias !31209, !noundef !12
  store float %_49.i.i1046.7, ptr %iter.i.i958.sroa.0.0.ptr15455.7, align 4, !dbg !31226, !noalias !31209
  br label %bb16.i.i.loopexit, !dbg !31084

panic1.i.i:                                       ; preds = %bb17.i.i.7, %bb17.i.i.6, %bb17.i.i.5, %bb17.i.i.4, %bb17.i.i.3, %bb17.i.i.2, %bb17.i.i.1, %bb17.i.i
  %_50.i.i.lcssa.ph = phi i64 [ %_50.i.i.7, %bb17.i.i.7 ], [ %_50.i.i.6, %bb17.i.i.6 ], [ %_50.i.i.5, %bb17.i.i.5 ], [ %_50.i.i.4, %bb17.i.i.4 ], [ %_50.i.i.3, %bb17.i.i.3 ], [ %_50.i.i.2, %bb17.i.i.2 ], [ %_50.i.i.1, %bb17.i.i.1 ], [ %_51.i.i, %bb17.i.i ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i.i.lcssa.ph, i64 noundef %_149.1.i.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b305c1483509cfb31fdec21ff8752674) #30, !dbg !31225, !noalias !31209
  unreachable, !dbg !31225

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6667: ; preds = %bb49.i.i
  %_150.0.i.i = load ptr, ptr %685, align 8, !dbg !31203, !alias.scope !30939, !noalias !30940, !nonnull !12, !noundef !12
  %_127.i.i = getelementptr inbounds nuw float, ptr %_150.0.i.i, i64 %_76.i.i, !dbg !31227
  %lanes.i5947.sroa.0.0.copyload = load <8 x float>, ptr %_127.i.i, align 4, !dbg !31229, !alias.scope !31233, !noalias !31237
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_127.i.i, ptr noundef nonnull align 4 dereferenceable(32) %_181.i, i64 32, i1 false), !dbg !31239
  %1078 = fmul <8 x float> %1038, %lanes.i5947.sroa.0.0.copyload, !dbg !31244
  %1079 = select <8 x i1> %669, <8 x float> %lanes.i5947.sroa.0.0.copyload, <8 x float> %1078, !dbg !31249
  store <8 x float> %1079, ptr %_181.i, align 4, !dbg !31254, !alias.scope !31259, !noalias !31263
  %1080 = add i64 %main_cursor.sroa.0.1.i103315459, 1, !dbg !31267
  %_106.i = load i64, ptr %686, align 8, !dbg !31268, !alias.scope !29240, !noalias !30580, !noundef !12
  %_104.i = icmp eq i64 %1080, %_106.i, !dbg !31269
  %spec.store.select.i = select i1 %_104.i, i64 0, i64 %1080, !dbg !31269
  %1081 = add i64 %ring_cursor.sroa.0.1.i103215458, 1, !dbg !31270
  %_107.i = icmp eq i64 %1081, %_91.i, !dbg !31271
  %spec.store.select13.i = select i1 %_107.i, i64 0, i64 %1081, !dbg !31271
  %exitcond18029.not = icmp eq i64 %902, %umax18028, !dbg !31272
  br i1 %exitcond18029.not, label %bb16.i.bb13.i.loopexit_crit_edge, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit6027, !dbg !29317

bb54.i1047:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6687
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1034, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_54f0ebc64763f3f022885a8e201aab88) #30, !dbg !31275, !noalias !29345
  unreachable, !dbg !31275

_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit: ; preds = %bb13.i.loopexit
  %1082 = trunc i64 %main_cursor.sroa.0.1.i1033.lcssa to i32, !dbg !31276
  %1083 = trunc i64 %ring_cursor.sroa.0.1.i1032.lcssa to i32, !dbg !31277
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, !dbg !31278

_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit, %bb11.i
  %ring_cursor.sroa.0.0.i1001.lcssa = phi i32 [ %_36.i995, %bb11.i ], [ %1083, %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit ], !dbg !29266
  %main_cursor.sroa.0.0.i1002.lcssa = phi i32 [ %_34.i, %bb11.i ], [ %1082, %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit ], !dbg !29263
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_left.i988, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33) #31, !dbg !31279
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_right.i987, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34) #31, !dbg !31280
  store i32 %main_cursor.sroa.0.0.i1002.lcssa, ptr %_35, align 4, !dbg !31276, !alias.scope !29246, !noalias !29265
  store i32 %ring_cursor.sroa.0.0.i1001.lcssa, ptr %81, align 4, !dbg !31277, !alias.scope !29246, !noalias !29265
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i980), !dbg !31281, !noalias !29270
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i981), !dbg !31282, !noalias !29270
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i), !dbg !31283, !noalias !29270
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, !dbg !29239

bb7.i:                                            ; preds = %bb5.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !31284), !dbg !31287
  tail call void @llvm.experimental.noalias.scope.decl(metadata !31288), !dbg !31287
  tail call void @llvm.experimental.noalias.scope.decl(metadata !31290), !dbg !31287
  tail call void @llvm.experimental.noalias.scope.decl(metadata !31292), !dbg !31287
  tail call void @llvm.experimental.noalias.scope.decl(metadata !31294), !dbg !31287
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_left.i499, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #31, !dbg !31296
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_right.i498, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #31, !dbg !31300
  %1084 = load i8, ptr %83, align 32, !dbg !31302, !range !17, !alias.scope !31284, !noalias !31306, !noundef !12
  %1085 = load i8, ptr %84, align 1, !dbg !31309, !range !17, !alias.scope !31284, !noalias !31306, !noundef !12
  %ring.i506 = load i64, ptr %85, align 8, !dbg !31311, !alias.scope !31288, !noalias !31313, !noundef !12
  %main.i507 = load i64, ptr %86, align 8, !dbg !31314, !alias.scope !31288, !noalias !31313, !noundef !12
  %_35.i508 = load i32, ptr %_35, align 4, !dbg !31316, !alias.scope !31294, !noalias !31318, !noundef !12
  %_36.i509 = load i32, ptr %87, align 4, !dbg !31319, !alias.scope !31294, !noalias !31318, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i492), !dbg !31321, !noalias !31323
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i492, i8 0, i64 1024, i1 false), !noalias !31323
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i491), !dbg !31324, !noalias !31323
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i491, i8 0, i64 1024, i1 false), !noalias !31323
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i490), !dbg !31326, !noalias !31323
; call <true_peak_limiter::UniformHot<wide::f32x8_::f32x8>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_(ptr noalias noundef align 32 captures(none) dereferenceable(128) %uniform_left.i490, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33, i64 %ring.i506, i64 %main.i507) #31, !dbg !31328
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_right.i489), !dbg !31329, !noalias !31323
  %_32.val = load i64, ptr %85, align 8, !dbg !31331, !noundef !12
  %_32.val7017 = load i64, ptr %86, align 8, !dbg !31331, !noundef !12
; call <true_peak_limiter::UniformHot<wide::f32x8_::f32x8>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_(ptr noalias noundef align 32 captures(none) dereferenceable(128) %uniform_right.i489, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34, i64 %_32.val, i64 %_32.val7017) #31, !dbg !31331
  %1086 = add nuw nsw i64 %frames, 31, !dbg !31332
  %yield_count.sroa.0.0.i.i7506 = lshr i64 %1086, 5, !dbg !31332
  %_167.not.i52015687 = icmp eq i64 %yield_count.sroa.0.0.i.i7506, 0, !dbg !31339
  br i1 %_167.not.i52015687, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, label %bb50.i521.lr.ph, !dbg !31339

bb50.i521.lr.ph:                                  ; preds = %bb7.i
  %1087 = zext i32 %_36.i509 to i64, !dbg !31319
  %1088 = zext i32 %_35.i508 to i64, !dbg !31316
  %_32.i503 = trunc nuw i8 %1085 to i1, !dbg !31309
  %_31.i500 = trunc nuw i8 %1084 to i1, !dbg !31302
  %1089 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !31348
  %1090 = bitcast <8 x float> %1089 to <8 x i32>, !dbg !31354
  %1091 = xor <8 x i32> %1090, splat (i32 -1), !dbg !31360
  %history.i215.i261.sroa.10.0.hot_left.i499.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i499, i64 32
  %history.i215.i261.sroa.13.0.hot_left.i499.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i499, i64 64
  %history.i215.i261.sroa.16.0.hot_left.i499.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i499, i64 96
  %history.i215.i261.sroa.19.0.hot_left.i499.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i499, i64 128
  %history.i215.i261.sroa.22.0.hot_left.i499.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i499, i64 160
  %history.i215.i261.sroa.25.0.hot_left.i499.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i499, i64 192
  %history.i215.i261.sroa.29.0.hot_left.i499.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i499, i64 224
  %history.i215.i261.sroa.32.0.hot_left.i499.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i499, i64 256
  %history.i215.i261.sroa.35.0.hot_left.i499.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i499, i64 288
  %history.i215.i261.sroa.38.0.hot_left.i499.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i499, i64 320
  %history.i215.i261.sroa.41.0.hot_left.i499.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i499, i64 352
  %1092 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %1093 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %1094 = getelementptr inbounds nuw i8, ptr %self, i64 96
  %row12.i.i.i222.i542 = getelementptr inbounds nuw i8, ptr %self, i64 128
  %1095 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %1096 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %1097 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %row13.i.i.i223.i543 = getelementptr inbounds nuw i8, ptr %self, i64 256
  %1098 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %1099 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %1100 = getelementptr inbounds nuw i8, ptr %self, i64 352
  %row14.i.i.i224.i544 = getelementptr inbounds nuw i8, ptr %self, i64 384
  %1101 = getelementptr inbounds nuw i8, ptr %self, i64 416
  %1102 = getelementptr inbounds nuw i8, ptr %self, i64 448
  %1103 = getelementptr inbounds nuw i8, ptr %self, i64 480
  %row15.i.i.i225.i545 = getelementptr inbounds nuw i8, ptr %self, i64 512
  %1104 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %1105 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %1106 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %row16.i.i.i226.i546 = getelementptr inbounds nuw i8, ptr %self, i64 640
  %1107 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %1108 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %1109 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %row17.i.i.i227.i547 = getelementptr inbounds nuw i8, ptr %self, i64 768
  %1110 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %1111 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %1112 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %row18.i.i.i228.i548 = getelementptr inbounds nuw i8, ptr %self, i64 896
  %1113 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %1114 = getelementptr inbounds nuw i8, ptr %self, i64 960
  %1115 = getelementptr inbounds nuw i8, ptr %self, i64 992
  %row19.i.i.i229.i549 = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %1116 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %1117 = getelementptr inbounds nuw i8, ptr %self, i64 1088
  %1118 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %row20.i.i.i230.i550 = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %1119 = getelementptr inbounds nuw i8, ptr %self, i64 1184
  %1120 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %1121 = getelementptr inbounds nuw i8, ptr %self, i64 1248
  %row21.i.i.i231.i551 = getelementptr inbounds nuw i8, ptr %self, i64 1280
  %1122 = getelementptr inbounds nuw i8, ptr %self, i64 1312
  %1123 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %1124 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %row22.i.i.i232.i552 = getelementptr inbounds nuw i8, ptr %self, i64 1408
  %1125 = getelementptr inbounds nuw i8, ptr %self, i64 1440
  %1126 = getelementptr inbounds nuw i8, ptr %self, i64 1472
  %1127 = getelementptr inbounds nuw i8, ptr %self, i64 1504
  %history.i.i451.sroa.10.0.hot_right.i498.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i498, i64 32
  %history.i.i451.sroa.13.0.hot_right.i498.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i498, i64 64
  %history.i.i451.sroa.16.0.hot_right.i498.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i498, i64 96
  %history.i.i451.sroa.19.0.hot_right.i498.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i498, i64 128
  %history.i.i451.sroa.22.0.hot_right.i498.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i498, i64 160
  %history.i.i451.sroa.25.0.hot_right.i498.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i498, i64 192
  %history.i.i451.sroa.29.0.hot_right.i498.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i498, i64 224
  %history.i.i451.sroa.32.0.hot_right.i498.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i498, i64 256
  %history.i.i451.sroa.35.0.hot_right.i498.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i498, i64 288
  %history.i.i451.sroa.38.0.hot_right.i498.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i498, i64 320
  %history.i.i451.sroa.41.0.hot_right.i498.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i498, i64 352
  %1128 = getelementptr inbounds nuw i8, ptr %uniform_left.i490, i64 80
  %_70.i486.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i490, i64 88
  %_70.i486.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i490, i64 96
  %1129 = getelementptr inbounds nuw i8, ptr %uniform_right.i489, i64 80
  %_71.i485.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i489, i64 88
  %_71.i485.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i489, i64 96
  %_114.i630 = getelementptr inbounds nuw i8, ptr %hot_left.i499, i64 384
  %_115.i631 = getelementptr inbounds nuw i8, ptr %hot_left.i499, i64 512
  %1130 = getelementptr inbounds nuw i8, ptr %hot_left.i499, i64 480
  %1131 = getelementptr inbounds nuw i8, ptr %hot_left.i499, i64 448
  %1132 = getelementptr inbounds nuw i8, ptr %hot_left.i499, i64 416
  %1133 = getelementptr inbounds nuw i8, ptr %hot_left.i499, i64 608
  %1134 = getelementptr inbounds nuw i8, ptr %hot_left.i499, i64 576
  %1135 = getelementptr inbounds nuw i8, ptr %hot_left.i499, i64 544
  %_119.i632 = getelementptr inbounds nuw i8, ptr %hot_right.i498, i64 384
  %_120.i633 = getelementptr inbounds nuw i8, ptr %hot_right.i498, i64 512
  %1136 = getelementptr inbounds nuw i8, ptr %hot_right.i498, i64 480
  %1137 = getelementptr inbounds nuw i8, ptr %hot_right.i498, i64 448
  %1138 = getelementptr inbounds nuw i8, ptr %hot_right.i498, i64 416
  %1139 = getelementptr inbounds nuw i8, ptr %hot_right.i498, i64 608
  %1140 = getelementptr inbounds nuw i8, ptr %hot_right.i498, i64 576
  %1141 = getelementptr inbounds nuw i8, ptr %hot_right.i498, i64 544
  %1142 = select i1 %_31.i500, <8 x i32> %1090, <8 x i32> %1091
  %1143 = icmp slt <8 x i32> %1142, zeroinitializer
  %1144 = getelementptr inbounds nuw i8, ptr %uniform_left.i490, i64 32
  %1145 = getelementptr inbounds nuw i8, ptr %uniform_left.i490, i64 40
  %_22.i293.i659 = getelementptr inbounds nuw i8, ptr %uniform_left.i490, i64 104
  %1146 = getelementptr inbounds nuw i8, ptr %uniform_left.i490, i64 48
  %1147 = getelementptr inbounds nuw i8, ptr %uniform_left.i490, i64 56
  %1148 = getelementptr inbounds nuw i8, ptr %hot_left.i499, i64 672
  %1149 = getelementptr inbounds nuw i8, ptr %hot_left.i499, i64 704
  %1150 = getelementptr inbounds nuw i8, ptr %hot_left.i499, i64 640
  %1151 = getelementptr inbounds nuw i8, ptr %uniform_left.i490, i64 72
  %1152 = getelementptr inbounds nuw i8, ptr %uniform_left.i490, i64 64
  %1153 = select i1 %_32.i503, <8 x i32> %1090, <8 x i32> %1091
  %1154 = icmp slt <8 x i32> %1153, zeroinitializer
  %1155 = getelementptr inbounds nuw i8, ptr %uniform_right.i489, i64 32
  %1156 = getelementptr inbounds nuw i8, ptr %uniform_right.i489, i64 40
  %_22.i.i709 = getelementptr inbounds nuw i8, ptr %uniform_right.i489, i64 104
  %1157 = getelementptr inbounds nuw i8, ptr %uniform_right.i489, i64 48
  %1158 = getelementptr inbounds nuw i8, ptr %uniform_right.i489, i64 56
  %1159 = getelementptr inbounds nuw i8, ptr %hot_right.i498, i64 672
  %1160 = getelementptr inbounds nuw i8, ptr %hot_right.i498, i64 704
  %1161 = getelementptr inbounds nuw i8, ptr %hot_right.i498, i64 640
  %1162 = getelementptr inbounds nuw i8, ptr %uniform_right.i489, i64 72
  %1163 = getelementptr inbounds nuw i8, ptr %uniform_right.i489, i64 64
  br label %bb50.i521, !dbg !31339

bb15.i515.loopexit.loopexit:                      ; preds = %bb32.i740
  store i32 %storemerge.i1813.lcssa2078220830, ptr %_22.i293.i659, align 4
  store <8 x float> %minimum.i273.i35.sroa.0.0.lcssa2079620849, ptr %uniform_left.i490, align 32
  store i32 %storemerge.i.lcssa2081220868, ptr %_22.i.i709, align 4
  store <8 x float> %minimum.i.i67.sroa.0.0.lcssa2082620887, ptr %uniform_right.i489, align 32
  br label %bb15.i515.loopexit, !dbg !31339

bb15.i515.loopexit:                               ; preds = %bb15.i515.loopexit.loopexit, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i586
  %ring_cursor.sroa.0.1.i588.lcssa = phi i64 [ %ring_cursor.sroa.0.0.i51615688, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i586 ], [ %ring_cursor.sroa.0.2.i743, %bb15.i515.loopexit.loopexit ], !dbg !31362
  %main_cursor.sroa.0.1.i589.lcssa = phi i64 [ %main_cursor.sroa.0.0.i51715689, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i586 ], [ %main_cursor.sroa.0.2.i746, %bb15.i515.loopexit.loopexit ], !dbg !31363
  %_167.not.i520 = icmp eq i64 %1165, 0, !dbg !31339
  %indvars.iv.next18033 = add nsw i64 %indvars.iv18032, -32, !dbg !31339
  br i1 %_167.not.i520, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit, label %bb50.i521, !dbg !31339

bb50.i521:                                        ; preds = %bb50.i521.lr.ph, %bb15.i515.loopexit
  %indvars.iv18032 = phi i64 [ %frames, %bb50.i521.lr.ph ], [ %indvars.iv.next18033, %bb15.i515.loopexit ]
  %iter4.sroa.0.0.i51915691 = phi i64 [ %yield_count.sroa.0.0.i.i7506, %bb50.i521.lr.ph ], [ %1165, %bb15.i515.loopexit ]
  %iter3.sroa.0.0.i51815690 = phi i64 [ 0, %bb50.i521.lr.ph ], [ %1164, %bb15.i515.loopexit ]
  %main_cursor.sroa.0.0.i51715689 = phi i64 [ %1088, %bb50.i521.lr.ph ], [ %main_cursor.sroa.0.1.i589.lcssa, %bb15.i515.loopexit ]
  %ring_cursor.sroa.0.0.i51615688 = phi i64 [ %1087, %bb50.i521.lr.ph ], [ %ring_cursor.sroa.0.1.i588.lcssa, %bb15.i515.loopexit ]
  %umin18064 = call i64 @llvm.umin.i64(i64 %indvars.iv18032, i64 32), !dbg !31364
  %umax18040 = call i64 @llvm.umax.i64(i64 %umin18064, i64 1), !dbg !31364
  %1164 = add nuw nsw i64 %iter3.sroa.0.0.i51815690, 32, !dbg !31364
  %1165 = add nsw i64 %iter4.sroa.0.0.i51915691, -1, !dbg !31368
  %_46.i523 = sub nsw i64 %frames, %iter3.sroa.0.0.i51815690, !dbg !31369
  %..i7507 = tail call noundef i64 @llvm.umin.i64(i64 %_46.i523, i64 32), !dbg !31371
  %active_base.i525 = shl i64 %iter3.sroa.0.0.i51815690, 3, !dbg !31375
  %active_base.i52513435 = add nuw i64 %..i7507, %iter3.sroa.0.0.i51815690, !dbg !31377
  %_52.i527 = shl i64 %active_base.i52513435, 3, !dbg !31377
  %_179.i528 = icmp samesign ult i64 %_52.i527, %active_base.i525, !dbg !31380
  %_173.not.i529 = icmp ugt i64 %_52.i527, %left_io.1
  %or.cond.i530 = or i1 %_179.i528, %_173.not.i529, !dbg !31380
  br i1 %or.cond.i530, label %bb54.i750, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7537, !dbg !31380, !prof !165

bb54.i750:                                        ; preds = %bb50.i521
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %active_base.i525, i64 noundef %_52.i527, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e7f134ea71d3d762bf72c0ef5353d5ff) #30, !dbg !31387, !noalias !31294
  unreachable, !dbg !31387

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7537: ; preds = %bb50.i521
  %_182.i535 = getelementptr inbounds nuw float, ptr %left_io.0, i64 %active_base.i525, !dbg !31388
  %history.i215.i261.sroa.0.0.copyload = load <8 x float>, ptr %hot_left.i499, align 32, !dbg !31392
  %history.i215.i261.sroa.10.sroa.0.0.copyload = load <8 x float>, ptr %history.i215.i261.sroa.10.0.hot_left.i499.sroa_idx, align 32, !dbg !31392
  %history.i215.i261.sroa.13.sroa.0.0.copyload = load <8 x float>, ptr %history.i215.i261.sroa.13.0.hot_left.i499.sroa_idx, align 32, !dbg !31392
  %history.i215.i261.sroa.16.sroa.0.0.copyload = load <8 x float>, ptr %history.i215.i261.sroa.16.0.hot_left.i499.sroa_idx, align 32, !dbg !31392
  %history.i215.i261.sroa.19.sroa.0.0.copyload = load <8 x float>, ptr %history.i215.i261.sroa.19.0.hot_left.i499.sroa_idx, align 32, !dbg !31392
  %history.i215.i261.sroa.22.sroa.0.0.copyload = load <8 x float>, ptr %history.i215.i261.sroa.22.0.hot_left.i499.sroa_idx, align 32, !dbg !31392
  %history.i215.i261.sroa.25.sroa.0.0.copyload = load <8 x float>, ptr %history.i215.i261.sroa.25.0.hot_left.i499.sroa_idx, align 32, !dbg !31392
  %history.i215.i261.sroa.29.sroa.0.0.copyload = load <8 x float>, ptr %history.i215.i261.sroa.29.0.hot_left.i499.sroa_idx, align 32, !dbg !31392
  %history.i215.i261.sroa.32.sroa.0.0.copyload = load <8 x float>, ptr %history.i215.i261.sroa.32.0.hot_left.i499.sroa_idx, align 32, !dbg !31392
  %history.i215.i261.sroa.35.sroa.0.0.copyload = load <8 x float>, ptr %history.i215.i261.sroa.35.0.hot_left.i499.sroa_idx, align 32, !dbg !31392
  %history.i215.i261.sroa.38.sroa.0.0.copyload = load <8 x float>, ptr %history.i215.i261.sroa.38.0.hot_left.i499.sroa_idx, align 32, !dbg !31392
  %history.i215.i261.sroa.41.sroa.0.0.copyload = load <8 x float>, ptr %history.i215.i261.sroa.41.0.hot_left.i499.sroa_idx, align 32, !dbg !31392
  %_2.i754015618.not = icmp eq i64 %frames, %iter3.sroa.0.0.i51815690, !dbg !31394
  br i1 %_2.i754015618.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit239.i559, label %bb6.i218.i538.lr.ph, !dbg !31394

bb6.i218.i538.lr.ph:                              ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7537
  %_5.i5251 = load <8 x float>, ptr %self, align 32, !alias.scope !31397, !noalias !31400
  %_14.i.i.i175.i221.sroa.0.0.copyload = load <8 x float>, ptr %1092, align 32, !noalias !31412
  %_17.i.i.i172.i218.sroa.0.0.copyload = load <8 x float>, ptr %1093, align 32, !noalias !31412
  %_20.i.i.i169.i215.sroa.0.0.copyload = load <8 x float>, ptr %1094, align 32, !noalias !31412
  %_25.i.i.i165.i211.sroa.0.0.copyload = load <8 x float>, ptr %row12.i.i.i222.i542, align 32, !noalias !31412
  %_28.i.i.i162.i208.sroa.0.0.copyload = load <8 x float>, ptr %1095, align 32, !noalias !31412
  %_31.i.i.i159.i205.sroa.0.0.copyload = load <8 x float>, ptr %1096, align 32, !noalias !31412
  %_34.i.i.i156.i202.sroa.0.0.copyload = load <8 x float>, ptr %1097, align 32, !noalias !31412
  %_39.i.i.i152.i198.sroa.0.0.copyload = load <8 x float>, ptr %row13.i.i.i223.i543, align 32, !noalias !31412
  %_42.i.i.i149.i195.sroa.0.0.copyload = load <8 x float>, ptr %1098, align 32, !noalias !31412
  %_45.i.i.i146.i192.sroa.0.0.copyload = load <8 x float>, ptr %1099, align 32, !noalias !31412
  %_48.i.i.i143.i189.sroa.0.0.copyload = load <8 x float>, ptr %1100, align 32, !noalias !31412
  %_53.i.i.i139.i185.sroa.0.0.copyload = load <8 x float>, ptr %row14.i.i.i224.i544, align 32, !noalias !31412
  %_56.i.i.i136.i182.sroa.0.0.copyload = load <8 x float>, ptr %1101, align 32, !noalias !31412
  %_59.i.i.i133.i179.sroa.0.0.copyload = load <8 x float>, ptr %1102, align 32, !noalias !31412
  %_62.i.i.i130.i176.sroa.0.0.copyload = load <8 x float>, ptr %1103, align 32, !noalias !31412
  %_67.i.i.i126.i172.sroa.0.0.copyload = load <8 x float>, ptr %row15.i.i.i225.i545, align 32, !noalias !31412
  %_70.i.i.i123.i169.sroa.0.0.copyload = load <8 x float>, ptr %1104, align 32, !noalias !31412
  %_73.i.i.i120.i166.sroa.0.0.copyload = load <8 x float>, ptr %1105, align 32, !noalias !31412
  %_76.i.i.i117.i163.sroa.0.0.copyload = load <8 x float>, ptr %1106, align 32, !noalias !31412
  %_81.i.i.i113.i159.sroa.0.0.copyload = load <8 x float>, ptr %row16.i.i.i226.i546, align 32, !noalias !31412
  %_84.i.i.i110.i156.sroa.0.0.copyload = load <8 x float>, ptr %1107, align 32, !noalias !31412
  %_87.i.i.i107.i153.sroa.0.0.copyload = load <8 x float>, ptr %1108, align 32, !noalias !31412
  %_90.i.i.i104.i150.sroa.0.0.copyload = load <8 x float>, ptr %1109, align 32, !noalias !31412
  %_95.i.i.i100.i146.sroa.0.0.copyload = load <8 x float>, ptr %row17.i.i.i227.i547, align 32, !noalias !31412
  %_98.i.i.i97.i143.sroa.0.0.copyload = load <8 x float>, ptr %1110, align 32, !noalias !31412
  %_101.i.i.i94.i140.sroa.0.0.copyload = load <8 x float>, ptr %1111, align 32, !noalias !31412
  %_104.i.i.i91.i137.sroa.0.0.copyload = load <8 x float>, ptr %1112, align 32, !noalias !31412
  %_109.i.i.i87.i133.sroa.0.0.copyload = load <8 x float>, ptr %row18.i.i.i228.i548, align 32, !noalias !31412
  %_112.i.i.i84.i130.sroa.0.0.copyload = load <8 x float>, ptr %1113, align 32, !noalias !31412
  %_115.i.i.i81.i127.sroa.0.0.copyload = load <8 x float>, ptr %1114, align 32, !noalias !31412
  %_118.i.i.i78.i124.sroa.0.0.copyload = load <8 x float>, ptr %1115, align 32, !noalias !31412
  %_123.i.i.i74.i120.sroa.0.0.copyload = load <8 x float>, ptr %row19.i.i.i229.i549, align 32, !noalias !31412
  %_126.i.i.i71.i117.sroa.0.0.copyload = load <8 x float>, ptr %1116, align 32, !noalias !31412
  %_129.i.i.i68.i114.sroa.0.0.copyload = load <8 x float>, ptr %1117, align 32, !noalias !31412
  %_132.i.i.i65.i111.sroa.0.0.copyload = load <8 x float>, ptr %1118, align 32, !noalias !31412
  %_137.i.i.i61.i107.sroa.0.0.copyload = load <8 x float>, ptr %row20.i.i.i230.i550, align 32, !noalias !31412
  %_140.i.i.i58.i104.sroa.0.0.copyload = load <8 x float>, ptr %1119, align 32, !noalias !31412
  %_143.i.i.i55.i101.sroa.0.0.copyload = load <8 x float>, ptr %1120, align 32, !noalias !31412
  %_146.i.i.i52.i98.sroa.0.0.copyload = load <8 x float>, ptr %1121, align 32, !noalias !31412
  %_151.i.i.i48.i94.sroa.0.0.copyload = load <8 x float>, ptr %row21.i.i.i231.i551, align 32, !noalias !31412
  %_154.i.i.i45.i91.sroa.0.0.copyload = load <8 x float>, ptr %1122, align 32, !noalias !31412
  %_157.i.i.i42.i88.sroa.0.0.copyload = load <8 x float>, ptr %1123, align 32, !noalias !31412
  %_160.i.i.i39.i85.sroa.0.0.copyload = load <8 x float>, ptr %1124, align 32, !noalias !31412
  %_165.i.i.i35.i81.sroa.0.0.copyload = load <8 x float>, ptr %row22.i.i.i232.i552, align 32, !noalias !31412
  %_168.i.i.i32.i78.sroa.0.0.copyload = load <8 x float>, ptr %1125, align 32, !noalias !31412
  %_171.i.i.i29.i75.sroa.0.0.copyload = load <8 x float>, ptr %1126, align 32, !noalias !31412
  %_174.i.i.i26.i72.sroa.0.0.copyload = load <8 x float>, ptr %1127, align 32, !noalias !31412
  br label %bb6.i218.i538, !dbg !31394

bb6.i218.i538:                                    ; preds = %bb6.i218.i538.lr.ph, %bb6.i218.i538
  %history.i215.i261.sroa.10.sroa.0.015630 = phi <8 x float> [ %history.i215.i261.sroa.10.sroa.0.0.copyload, %bb6.i218.i538.lr.ph ], [ %history.i215.i261.sroa.0.015619, %bb6.i218.i538 ]
  %history.i215.i261.sroa.13.sroa.0.015629 = phi <8 x float> [ %history.i215.i261.sroa.13.sroa.0.0.copyload, %bb6.i218.i538.lr.ph ], [ %history.i215.i261.sroa.10.sroa.0.015630, %bb6.i218.i538 ]
  %history.i215.i261.sroa.16.sroa.0.015628 = phi <8 x float> [ %history.i215.i261.sroa.16.sroa.0.0.copyload, %bb6.i218.i538.lr.ph ], [ %history.i215.i261.sroa.13.sroa.0.015629, %bb6.i218.i538 ]
  %history.i215.i261.sroa.19.sroa.0.015627 = phi <8 x float> [ %history.i215.i261.sroa.19.sroa.0.0.copyload, %bb6.i218.i538.lr.ph ], [ %history.i215.i261.sroa.16.sroa.0.015628, %bb6.i218.i538 ]
  %history.i215.i261.sroa.22.sroa.0.015626 = phi <8 x float> [ %history.i215.i261.sroa.22.sroa.0.0.copyload, %bb6.i218.i538.lr.ph ], [ %history.i215.i261.sroa.19.sroa.0.015627, %bb6.i218.i538 ]
  %history.i215.i261.sroa.38.sroa.0.015625 = phi <8 x float> [ %history.i215.i261.sroa.38.sroa.0.0.copyload, %bb6.i218.i538.lr.ph ], [ %history.i215.i261.sroa.35.sroa.0.015624, %bb6.i218.i538 ]
  %history.i215.i261.sroa.35.sroa.0.015624 = phi <8 x float> [ %history.i215.i261.sroa.35.sroa.0.0.copyload, %bb6.i218.i538.lr.ph ], [ %history.i215.i261.sroa.32.sroa.0.015623, %bb6.i218.i538 ]
  %history.i215.i261.sroa.32.sroa.0.015623 = phi <8 x float> [ %history.i215.i261.sroa.32.sroa.0.0.copyload, %bb6.i218.i538.lr.ph ], [ %history.i215.i261.sroa.29.sroa.0.015622, %bb6.i218.i538 ]
  %history.i215.i261.sroa.29.sroa.0.015622 = phi <8 x float> [ %history.i215.i261.sroa.29.sroa.0.0.copyload, %bb6.i218.i538.lr.ph ], [ %history.i215.i261.sroa.25.sroa.0.015621, %bb6.i218.i538 ]
  %history.i215.i261.sroa.25.sroa.0.015621 = phi <8 x float> [ %history.i215.i261.sroa.25.sroa.0.0.copyload, %bb6.i218.i538.lr.ph ], [ %history.i215.i261.sroa.22.sroa.0.015626, %bb6.i218.i538 ]
  %iter.i211.i257.sroa.16.015620 = phi i64 [ 0, %bb6.i218.i538.lr.ph ], [ %1271, %bb6.i218.i538 ]
  %history.i215.i261.sroa.0.015619 = phi <8 x float> [ %history.i215.i261.sroa.0.0.copyload, %bb6.i218.i538.lr.ph ], [ %lanes.i6092.sroa.0.0.copyload, %bb6.i218.i538 ]
  %start1.i.i7546 = shl i64 %iter.i211.i257.sroa.16.015620, 3, !dbg !31415
  %data.i.i7547 = getelementptr inbounds nuw float, ptr %_182.i535, i64 %start1.i.i7546, !dbg !31417
  %lanes.i6092.sroa.0.0.copyload = load <8 x float>, ptr %data.i.i7547, align 4, !dbg !31419, !alias.scope !31424, !noalias !31428
  %1166 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i215.i261.sroa.22.sroa.0.015626), !dbg !31433
  %1167 = fmul <8 x float> %lanes.i6092.sroa.0.0.copyload, %_5.i5251, !dbg !31440
  %1168 = fadd <8 x float> %1167, zeroinitializer, !dbg !31446
  %1169 = fmul <8 x float> %lanes.i6092.sroa.0.0.copyload, %_14.i.i.i175.i221.sroa.0.0.copyload, !dbg !31451
  %1170 = fadd <8 x float> %1169, zeroinitializer, !dbg !31456
  %1171 = fmul <8 x float> %lanes.i6092.sroa.0.0.copyload, %_17.i.i.i172.i218.sroa.0.0.copyload, !dbg !31461
  %1172 = fadd <8 x float> %1171, zeroinitializer, !dbg !31466
  %1173 = fmul <8 x float> %lanes.i6092.sroa.0.0.copyload, %_20.i.i.i169.i215.sroa.0.0.copyload, !dbg !31471
  %1174 = fadd <8 x float> %1173, zeroinitializer, !dbg !31476
  %1175 = fmul <8 x float> %history.i215.i261.sroa.0.015619, %_25.i.i.i165.i211.sroa.0.0.copyload, !dbg !31481
  %1176 = fadd <8 x float> %1168, %1175, !dbg !31486
  %1177 = fmul <8 x float> %history.i215.i261.sroa.0.015619, %_28.i.i.i162.i208.sroa.0.0.copyload, !dbg !31491
  %1178 = fadd <8 x float> %1170, %1177, !dbg !31496
  %1179 = fmul <8 x float> %history.i215.i261.sroa.0.015619, %_31.i.i.i159.i205.sroa.0.0.copyload, !dbg !31501
  %1180 = fadd <8 x float> %1172, %1179, !dbg !31506
  %1181 = fmul <8 x float> %history.i215.i261.sroa.0.015619, %_34.i.i.i156.i202.sroa.0.0.copyload, !dbg !31511
  %1182 = fadd <8 x float> %1174, %1181, !dbg !31516
  %1183 = fmul <8 x float> %history.i215.i261.sroa.10.sroa.0.015630, %_39.i.i.i152.i198.sroa.0.0.copyload, !dbg !31521
  %1184 = fadd <8 x float> %1176, %1183, !dbg !31526
  %1185 = fmul <8 x float> %history.i215.i261.sroa.10.sroa.0.015630, %_42.i.i.i149.i195.sroa.0.0.copyload, !dbg !31531
  %1186 = fadd <8 x float> %1178, %1185, !dbg !31536
  %1187 = fmul <8 x float> %history.i215.i261.sroa.10.sroa.0.015630, %_45.i.i.i146.i192.sroa.0.0.copyload, !dbg !31541
  %1188 = fadd <8 x float> %1180, %1187, !dbg !31546
  %1189 = fmul <8 x float> %history.i215.i261.sroa.10.sroa.0.015630, %_48.i.i.i143.i189.sroa.0.0.copyload, !dbg !31551
  %1190 = fadd <8 x float> %1182, %1189, !dbg !31556
  %1191 = fmul <8 x float> %history.i215.i261.sroa.13.sroa.0.015629, %_53.i.i.i139.i185.sroa.0.0.copyload, !dbg !31561
  %1192 = fadd <8 x float> %1184, %1191, !dbg !31566
  %1193 = fmul <8 x float> %history.i215.i261.sroa.13.sroa.0.015629, %_56.i.i.i136.i182.sroa.0.0.copyload, !dbg !31571
  %1194 = fadd <8 x float> %1186, %1193, !dbg !31576
  %1195 = fmul <8 x float> %history.i215.i261.sroa.13.sroa.0.015629, %_59.i.i.i133.i179.sroa.0.0.copyload, !dbg !31581
  %1196 = fadd <8 x float> %1188, %1195, !dbg !31586
  %1197 = fmul <8 x float> %history.i215.i261.sroa.13.sroa.0.015629, %_62.i.i.i130.i176.sroa.0.0.copyload, !dbg !31591
  %1198 = fadd <8 x float> %1190, %1197, !dbg !31596
  %1199 = fmul <8 x float> %history.i215.i261.sroa.16.sroa.0.015628, %_67.i.i.i126.i172.sroa.0.0.copyload, !dbg !31601
  %1200 = fadd <8 x float> %1192, %1199, !dbg !31606
  %1201 = fmul <8 x float> %history.i215.i261.sroa.16.sroa.0.015628, %_70.i.i.i123.i169.sroa.0.0.copyload, !dbg !31611
  %1202 = fadd <8 x float> %1194, %1201, !dbg !31616
  %1203 = fmul <8 x float> %history.i215.i261.sroa.16.sroa.0.015628, %_73.i.i.i120.i166.sroa.0.0.copyload, !dbg !31621
  %1204 = fadd <8 x float> %1196, %1203, !dbg !31626
  %1205 = fmul <8 x float> %history.i215.i261.sroa.16.sroa.0.015628, %_76.i.i.i117.i163.sroa.0.0.copyload, !dbg !31631
  %1206 = fadd <8 x float> %1198, %1205, !dbg !31636
  %1207 = fmul <8 x float> %history.i215.i261.sroa.19.sroa.0.015627, %_81.i.i.i113.i159.sroa.0.0.copyload, !dbg !31641
  %1208 = fadd <8 x float> %1200, %1207, !dbg !31646
  %1209 = fmul <8 x float> %history.i215.i261.sroa.19.sroa.0.015627, %_84.i.i.i110.i156.sroa.0.0.copyload, !dbg !31651
  %1210 = fadd <8 x float> %1202, %1209, !dbg !31656
  %1211 = fmul <8 x float> %history.i215.i261.sroa.19.sroa.0.015627, %_87.i.i.i107.i153.sroa.0.0.copyload, !dbg !31661
  %1212 = fadd <8 x float> %1204, %1211, !dbg !31666
  %1213 = fmul <8 x float> %history.i215.i261.sroa.19.sroa.0.015627, %_90.i.i.i104.i150.sroa.0.0.copyload, !dbg !31671
  %1214 = fadd <8 x float> %1206, %1213, !dbg !31676
  %1215 = fmul <8 x float> %history.i215.i261.sroa.22.sroa.0.015626, %_95.i.i.i100.i146.sroa.0.0.copyload, !dbg !31681
  %1216 = fadd <8 x float> %1208, %1215, !dbg !31686
  %1217 = fmul <8 x float> %history.i215.i261.sroa.22.sroa.0.015626, %_98.i.i.i97.i143.sroa.0.0.copyload, !dbg !31691
  %1218 = fadd <8 x float> %1210, %1217, !dbg !31696
  %1219 = fmul <8 x float> %history.i215.i261.sroa.22.sroa.0.015626, %_101.i.i.i94.i140.sroa.0.0.copyload, !dbg !31701
  %1220 = fadd <8 x float> %1212, %1219, !dbg !31706
  %1221 = fmul <8 x float> %history.i215.i261.sroa.22.sroa.0.015626, %_104.i.i.i91.i137.sroa.0.0.copyload, !dbg !31711
  %1222 = fadd <8 x float> %1214, %1221, !dbg !31716
  %1223 = fmul <8 x float> %history.i215.i261.sroa.25.sroa.0.015621, %_109.i.i.i87.i133.sroa.0.0.copyload, !dbg !31721
  %1224 = fadd <8 x float> %1216, %1223, !dbg !31726
  %1225 = fmul <8 x float> %history.i215.i261.sroa.25.sroa.0.015621, %_112.i.i.i84.i130.sroa.0.0.copyload, !dbg !31731
  %1226 = fadd <8 x float> %1218, %1225, !dbg !31736
  %1227 = fmul <8 x float> %history.i215.i261.sroa.25.sroa.0.015621, %_115.i.i.i81.i127.sroa.0.0.copyload, !dbg !31741
  %1228 = fadd <8 x float> %1220, %1227, !dbg !31746
  %1229 = fmul <8 x float> %history.i215.i261.sroa.25.sroa.0.015621, %_118.i.i.i78.i124.sroa.0.0.copyload, !dbg !31751
  %1230 = fadd <8 x float> %1222, %1229, !dbg !31756
  %1231 = fmul <8 x float> %history.i215.i261.sroa.29.sroa.0.015622, %_123.i.i.i74.i120.sroa.0.0.copyload, !dbg !31761
  %1232 = fadd <8 x float> %1224, %1231, !dbg !31766
  %1233 = fmul <8 x float> %history.i215.i261.sroa.29.sroa.0.015622, %_126.i.i.i71.i117.sroa.0.0.copyload, !dbg !31771
  %1234 = fadd <8 x float> %1226, %1233, !dbg !31776
  %1235 = fmul <8 x float> %history.i215.i261.sroa.29.sroa.0.015622, %_129.i.i.i68.i114.sroa.0.0.copyload, !dbg !31781
  %1236 = fadd <8 x float> %1228, %1235, !dbg !31786
  %1237 = fmul <8 x float> %history.i215.i261.sroa.29.sroa.0.015622, %_132.i.i.i65.i111.sroa.0.0.copyload, !dbg !31791
  %1238 = fadd <8 x float> %1230, %1237, !dbg !31796
  %1239 = fmul <8 x float> %history.i215.i261.sroa.32.sroa.0.015623, %_137.i.i.i61.i107.sroa.0.0.copyload, !dbg !31801
  %1240 = fadd <8 x float> %1232, %1239, !dbg !31806
  %1241 = fmul <8 x float> %history.i215.i261.sroa.32.sroa.0.015623, %_140.i.i.i58.i104.sroa.0.0.copyload, !dbg !31811
  %1242 = fadd <8 x float> %1234, %1241, !dbg !31816
  %1243 = fmul <8 x float> %history.i215.i261.sroa.32.sroa.0.015623, %_143.i.i.i55.i101.sroa.0.0.copyload, !dbg !31821
  %1244 = fadd <8 x float> %1236, %1243, !dbg !31826
  %1245 = fmul <8 x float> %history.i215.i261.sroa.32.sroa.0.015623, %_146.i.i.i52.i98.sroa.0.0.copyload, !dbg !31831
  %1246 = fadd <8 x float> %1238, %1245, !dbg !31836
  %1247 = fmul <8 x float> %history.i215.i261.sroa.35.sroa.0.015624, %_151.i.i.i48.i94.sroa.0.0.copyload, !dbg !31841
  %1248 = fadd <8 x float> %1240, %1247, !dbg !31846
  %1249 = fmul <8 x float> %history.i215.i261.sroa.35.sroa.0.015624, %_154.i.i.i45.i91.sroa.0.0.copyload, !dbg !31851
  %1250 = fadd <8 x float> %1242, %1249, !dbg !31856
  %1251 = fmul <8 x float> %history.i215.i261.sroa.35.sroa.0.015624, %_157.i.i.i42.i88.sroa.0.0.copyload, !dbg !31861
  %1252 = fadd <8 x float> %1244, %1251, !dbg !31866
  %1253 = fmul <8 x float> %history.i215.i261.sroa.35.sroa.0.015624, %_160.i.i.i39.i85.sroa.0.0.copyload, !dbg !31871
  %1254 = fadd <8 x float> %1246, %1253, !dbg !31876
  %1255 = fmul <8 x float> %history.i215.i261.sroa.38.sroa.0.015625, %_165.i.i.i35.i81.sroa.0.0.copyload, !dbg !31881
  %1256 = fadd <8 x float> %1248, %1255, !dbg !31886
  %1257 = fmul <8 x float> %history.i215.i261.sroa.38.sroa.0.015625, %_168.i.i.i32.i78.sroa.0.0.copyload, !dbg !31891
  %1258 = fadd <8 x float> %1250, %1257, !dbg !31896
  %1259 = fmul <8 x float> %history.i215.i261.sroa.38.sroa.0.015625, %_171.i.i.i29.i75.sroa.0.0.copyload, !dbg !31901
  %1260 = fadd <8 x float> %1252, %1259, !dbg !31906
  %1261 = fmul <8 x float> %history.i215.i261.sroa.38.sroa.0.015625, %_174.i.i.i26.i72.sroa.0.0.copyload, !dbg !31911
  %1262 = fadd <8 x float> %1254, %1261, !dbg !31916
  %1263 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1256), !dbg !31921
  %1264 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1166, <8 x float> %1263), !dbg !31927
  %1265 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1258), !dbg !31921
  %1266 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1264, <8 x float> %1265), !dbg !31927
  %1267 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1260), !dbg !31921
  %1268 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1266, <8 x float> %1267), !dbg !31927
  %1269 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1262), !dbg !31921
  %1270 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1268, <8 x float> %1269), !dbg !31927
  %1271 = add nuw nsw i64 %iter.i211.i257.sroa.16.015620, 1, !dbg !31932
  %data.i4.i7551 = getelementptr inbounds nuw float, ptr %peaks_left.i492, i64 %start1.i.i7546, !dbg !31933
  store <8 x float> %1270, ptr %data.i4.i7551, align 4, !dbg !31936, !alias.scope !31941, !noalias !31945
  %exitcond18036.not = icmp eq i64 %1271, %umax18040, !dbg !31394
  br i1 %exitcond18036.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit239.i559, label %bb6.i218.i538, !dbg !31394

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit239.i559: ; preds = %bb6.i218.i538, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7537
  %history.i215.i261.sroa.0.0.lcssa = phi <8 x float> [ %history.i215.i261.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7537 ], [ %lanes.i6092.sroa.0.0.copyload, %bb6.i218.i538 ], !dbg !31949
  %history.i215.i261.sroa.25.sroa.0.0.lcssa = phi <8 x float> [ %history.i215.i261.sroa.25.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7537 ], [ %history.i215.i261.sroa.22.sroa.0.015626, %bb6.i218.i538 ], !dbg !31949
  %history.i215.i261.sroa.29.sroa.0.0.lcssa = phi <8 x float> [ %history.i215.i261.sroa.29.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7537 ], [ %history.i215.i261.sroa.25.sroa.0.015621, %bb6.i218.i538 ], !dbg !31949
  %history.i215.i261.sroa.32.sroa.0.0.lcssa = phi <8 x float> [ %history.i215.i261.sroa.32.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7537 ], [ %history.i215.i261.sroa.29.sroa.0.015622, %bb6.i218.i538 ], !dbg !31949
  %history.i215.i261.sroa.35.sroa.0.0.lcssa = phi <8 x float> [ %history.i215.i261.sroa.35.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7537 ], [ %history.i215.i261.sroa.32.sroa.0.015623, %bb6.i218.i538 ], !dbg !31949
  %history.i215.i261.sroa.38.sroa.0.0.lcssa = phi <8 x float> [ %history.i215.i261.sroa.38.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7537 ], [ %history.i215.i261.sroa.35.sroa.0.015624, %bb6.i218.i538 ], !dbg !31949
  %history.i215.i261.sroa.41.sroa.0.0.lcssa = phi <8 x float> [ %history.i215.i261.sroa.41.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7537 ], [ %history.i215.i261.sroa.38.sroa.0.015625, %bb6.i218.i538 ], !dbg !31949
  %history.i215.i261.sroa.22.sroa.0.0.lcssa = phi <8 x float> [ %history.i215.i261.sroa.22.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7537 ], [ %history.i215.i261.sroa.19.sroa.0.015627, %bb6.i218.i538 ], !dbg !31949
  %history.i215.i261.sroa.19.sroa.0.0.lcssa = phi <8 x float> [ %history.i215.i261.sroa.19.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7537 ], [ %history.i215.i261.sroa.16.sroa.0.015628, %bb6.i218.i538 ], !dbg !31949
  %history.i215.i261.sroa.16.sroa.0.0.lcssa = phi <8 x float> [ %history.i215.i261.sroa.16.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7537 ], [ %history.i215.i261.sroa.13.sroa.0.015629, %bb6.i218.i538 ], !dbg !31949
  %history.i215.i261.sroa.13.sroa.0.0.lcssa = phi <8 x float> [ %history.i215.i261.sroa.13.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7537 ], [ %history.i215.i261.sroa.10.sroa.0.015630, %bb6.i218.i538 ], !dbg !31949
  %history.i215.i261.sroa.10.sroa.0.0.lcssa = phi <8 x float> [ %history.i215.i261.sroa.10.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7537 ], [ %history.i215.i261.sroa.0.015619, %bb6.i218.i538 ], !dbg !31949
  store <8 x float> %history.i215.i261.sroa.0.0.lcssa, ptr %hot_left.i499, align 32, !dbg !31950
  store <8 x float> %history.i215.i261.sroa.10.sroa.0.0.lcssa, ptr %history.i215.i261.sroa.10.0.hot_left.i499.sroa_idx, align 32, !dbg !31950
  store <8 x float> %history.i215.i261.sroa.13.sroa.0.0.lcssa, ptr %history.i215.i261.sroa.13.0.hot_left.i499.sroa_idx, align 32, !dbg !31950
  store <8 x float> %history.i215.i261.sroa.16.sroa.0.0.lcssa, ptr %history.i215.i261.sroa.16.0.hot_left.i499.sroa_idx, align 32, !dbg !31950
  store <8 x float> %history.i215.i261.sroa.19.sroa.0.0.lcssa, ptr %history.i215.i261.sroa.19.0.hot_left.i499.sroa_idx, align 32, !dbg !31950
  store <8 x float> %history.i215.i261.sroa.22.sroa.0.0.lcssa, ptr %history.i215.i261.sroa.22.0.hot_left.i499.sroa_idx, align 32, !dbg !31950
  store <8 x float> %history.i215.i261.sroa.25.sroa.0.0.lcssa, ptr %history.i215.i261.sroa.25.0.hot_left.i499.sroa_idx, align 32, !dbg !31950
  store <8 x float> %history.i215.i261.sroa.29.sroa.0.0.lcssa, ptr %history.i215.i261.sroa.29.0.hot_left.i499.sroa_idx, align 32, !dbg !31950
  store <8 x float> %history.i215.i261.sroa.32.sroa.0.0.lcssa, ptr %history.i215.i261.sroa.32.0.hot_left.i499.sroa_idx, align 32, !dbg !31950
  store <8 x float> %history.i215.i261.sroa.35.sroa.0.0.lcssa, ptr %history.i215.i261.sroa.35.0.hot_left.i499.sroa_idx, align 32, !dbg !31950
  store <8 x float> %history.i215.i261.sroa.38.sroa.0.0.lcssa, ptr %history.i215.i261.sroa.38.0.hot_left.i499.sroa_idx, align 32, !dbg !31950
  store <8 x float> %history.i215.i261.sroa.41.sroa.0.0.lcssa, ptr %history.i215.i261.sroa.41.0.hot_left.i499.sroa_idx, align 32, !dbg !31950
  %_190.not.i560 = icmp ugt i64 %_52.i527, %right_io.1, !dbg !31951
  br i1 %_190.not.i560, label %bb60.i749, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7597, !dbg !31951, !prof !1406

bb60.i749:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit239.i559
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %active_base.i525, i64 noundef %_52.i527, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_480b8302a9d65cc7541e43747df38ed7) #30, !dbg !31955, !noalias !31294
  unreachable, !dbg !31955

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7597: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit239.i559
  %_197.i562 = getelementptr inbounds nuw float, ptr %right_io.0, i64 %active_base.i525, !dbg !31956
  %history.i.i451.sroa.0.0.copyload = load <8 x float>, ptr %hot_right.i498, align 32, !dbg !31960
  %history.i.i451.sroa.10.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i451.sroa.10.0.hot_right.i498.sroa_idx, align 32, !dbg !31960
  %history.i.i451.sroa.13.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i451.sroa.13.0.hot_right.i498.sroa_idx, align 32, !dbg !31960
  %history.i.i451.sroa.16.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i451.sroa.16.0.hot_right.i498.sroa_idx, align 32, !dbg !31960
  %history.i.i451.sroa.19.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i451.sroa.19.0.hot_right.i498.sroa_idx, align 32, !dbg !31960
  %history.i.i451.sroa.22.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i451.sroa.22.0.hot_right.i498.sroa_idx, align 32, !dbg !31960
  %history.i.i451.sroa.25.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i451.sroa.25.0.hot_right.i498.sroa_idx, align 32, !dbg !31960
  %history.i.i451.sroa.29.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i451.sroa.29.0.hot_right.i498.sroa_idx, align 32, !dbg !31960
  %history.i.i451.sroa.32.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i451.sroa.32.0.hot_right.i498.sroa_idx, align 32, !dbg !31960
  %history.i.i451.sroa.35.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i451.sroa.35.0.hot_right.i498.sroa_idx, align 32, !dbg !31960
  %history.i.i451.sroa.38.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i451.sroa.38.0.hot_right.i498.sroa_idx, align 32, !dbg !31960
  %history.i.i451.sroa.41.sroa.0.0.copyload = load <8 x float>, ptr %history.i.i451.sroa.41.0.hot_right.i498.sroa_idx, align 32, !dbg !31960
  br i1 %_2.i754015618.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i586, label %bb6.i.i565.lr.ph, !dbg !31962

bb6.i.i565.lr.ph:                                 ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7597
  %_5.i5107 = load <8 x float>, ptr %self, align 32, !alias.scope !31965, !noalias !31968
  %_14.i.i.i.i411.sroa.0.0.copyload = load <8 x float>, ptr %1092, align 32, !noalias !31980
  %_17.i.i.i.i408.sroa.0.0.copyload = load <8 x float>, ptr %1093, align 32, !noalias !31980
  %_20.i.i.i.i405.sroa.0.0.copyload = load <8 x float>, ptr %1094, align 32, !noalias !31980
  %_25.i.i.i.i401.sroa.0.0.copyload = load <8 x float>, ptr %row12.i.i.i222.i542, align 32, !noalias !31980
  %_28.i.i.i.i398.sroa.0.0.copyload = load <8 x float>, ptr %1095, align 32, !noalias !31980
  %_31.i.i.i.i395.sroa.0.0.copyload = load <8 x float>, ptr %1096, align 32, !noalias !31980
  %_34.i.i.i.i392.sroa.0.0.copyload = load <8 x float>, ptr %1097, align 32, !noalias !31980
  %_39.i.i.i.i388.sroa.0.0.copyload = load <8 x float>, ptr %row13.i.i.i223.i543, align 32, !noalias !31980
  %_42.i.i.i.i385.sroa.0.0.copyload = load <8 x float>, ptr %1098, align 32, !noalias !31980
  %_45.i.i.i.i382.sroa.0.0.copyload = load <8 x float>, ptr %1099, align 32, !noalias !31980
  %_48.i.i.i.i379.sroa.0.0.copyload = load <8 x float>, ptr %1100, align 32, !noalias !31980
  %_53.i.i.i.i375.sroa.0.0.copyload = load <8 x float>, ptr %row14.i.i.i224.i544, align 32, !noalias !31980
  %_56.i.i.i.i372.sroa.0.0.copyload = load <8 x float>, ptr %1101, align 32, !noalias !31980
  %_59.i.i.i.i369.sroa.0.0.copyload = load <8 x float>, ptr %1102, align 32, !noalias !31980
  %_62.i.i.i.i366.sroa.0.0.copyload = load <8 x float>, ptr %1103, align 32, !noalias !31980
  %_67.i.i.i.i362.sroa.0.0.copyload = load <8 x float>, ptr %row15.i.i.i225.i545, align 32, !noalias !31980
  %_70.i.i.i.i359.sroa.0.0.copyload = load <8 x float>, ptr %1104, align 32, !noalias !31980
  %_73.i.i.i.i356.sroa.0.0.copyload = load <8 x float>, ptr %1105, align 32, !noalias !31980
  %_76.i.i.i.i353.sroa.0.0.copyload = load <8 x float>, ptr %1106, align 32, !noalias !31980
  %_81.i.i.i.i349.sroa.0.0.copyload = load <8 x float>, ptr %row16.i.i.i226.i546, align 32, !noalias !31980
  %_84.i.i.i.i346.sroa.0.0.copyload = load <8 x float>, ptr %1107, align 32, !noalias !31980
  %_87.i.i.i.i343.sroa.0.0.copyload = load <8 x float>, ptr %1108, align 32, !noalias !31980
  %_90.i.i.i.i340.sroa.0.0.copyload = load <8 x float>, ptr %1109, align 32, !noalias !31980
  %_95.i.i.i.i336.sroa.0.0.copyload = load <8 x float>, ptr %row17.i.i.i227.i547, align 32, !noalias !31980
  %_98.i.i.i.i333.sroa.0.0.copyload = load <8 x float>, ptr %1110, align 32, !noalias !31980
  %_101.i.i.i.i330.sroa.0.0.copyload = load <8 x float>, ptr %1111, align 32, !noalias !31980
  %_104.i.i.i.i327.sroa.0.0.copyload = load <8 x float>, ptr %1112, align 32, !noalias !31980
  %_109.i.i.i.i323.sroa.0.0.copyload = load <8 x float>, ptr %row18.i.i.i228.i548, align 32, !noalias !31980
  %_112.i.i.i.i320.sroa.0.0.copyload = load <8 x float>, ptr %1113, align 32, !noalias !31980
  %_115.i.i.i.i317.sroa.0.0.copyload = load <8 x float>, ptr %1114, align 32, !noalias !31980
  %_118.i.i.i.i314.sroa.0.0.copyload = load <8 x float>, ptr %1115, align 32, !noalias !31980
  %_123.i.i.i.i310.sroa.0.0.copyload = load <8 x float>, ptr %row19.i.i.i229.i549, align 32, !noalias !31980
  %_126.i.i.i.i307.sroa.0.0.copyload = load <8 x float>, ptr %1116, align 32, !noalias !31980
  %_129.i.i.i.i304.sroa.0.0.copyload = load <8 x float>, ptr %1117, align 32, !noalias !31980
  %_132.i.i.i.i301.sroa.0.0.copyload = load <8 x float>, ptr %1118, align 32, !noalias !31980
  %_137.i.i.i.i297.sroa.0.0.copyload = load <8 x float>, ptr %row20.i.i.i230.i550, align 32, !noalias !31980
  %_140.i.i.i.i294.sroa.0.0.copyload = load <8 x float>, ptr %1119, align 32, !noalias !31980
  %_143.i.i.i.i291.sroa.0.0.copyload = load <8 x float>, ptr %1120, align 32, !noalias !31980
  %_146.i.i.i.i288.sroa.0.0.copyload = load <8 x float>, ptr %1121, align 32, !noalias !31980
  %_151.i.i.i.i284.sroa.0.0.copyload = load <8 x float>, ptr %row21.i.i.i231.i551, align 32, !noalias !31980
  %_154.i.i.i.i281.sroa.0.0.copyload = load <8 x float>, ptr %1122, align 32, !noalias !31980
  %_157.i.i.i.i278.sroa.0.0.copyload = load <8 x float>, ptr %1123, align 32, !noalias !31980
  %_160.i.i.i.i275.sroa.0.0.copyload = load <8 x float>, ptr %1124, align 32, !noalias !31980
  %_165.i.i.i.i271.sroa.0.0.copyload = load <8 x float>, ptr %row22.i.i.i232.i552, align 32, !noalias !31980
  %_168.i.i.i.i268.sroa.0.0.copyload = load <8 x float>, ptr %1125, align 32, !noalias !31980
  %_171.i.i.i.i265.sroa.0.0.copyload = load <8 x float>, ptr %1126, align 32, !noalias !31980
  %_174.i.i.i.i262.sroa.0.0.copyload = load <8 x float>, ptr %1127, align 32, !noalias !31980
  br label %bb6.i.i565, !dbg !31962

bb6.i.i565:                                       ; preds = %bb6.i.i565.lr.ph, %bb6.i.i565
  %history.i.i451.sroa.10.sroa.0.015657 = phi <8 x float> [ %history.i.i451.sroa.10.sroa.0.0.copyload, %bb6.i.i565.lr.ph ], [ %history.i.i451.sroa.0.015646, %bb6.i.i565 ]
  %history.i.i451.sroa.13.sroa.0.015656 = phi <8 x float> [ %history.i.i451.sroa.13.sroa.0.0.copyload, %bb6.i.i565.lr.ph ], [ %history.i.i451.sroa.10.sroa.0.015657, %bb6.i.i565 ]
  %history.i.i451.sroa.16.sroa.0.015655 = phi <8 x float> [ %history.i.i451.sroa.16.sroa.0.0.copyload, %bb6.i.i565.lr.ph ], [ %history.i.i451.sroa.13.sroa.0.015656, %bb6.i.i565 ]
  %history.i.i451.sroa.19.sroa.0.015654 = phi <8 x float> [ %history.i.i451.sroa.19.sroa.0.0.copyload, %bb6.i.i565.lr.ph ], [ %history.i.i451.sroa.16.sroa.0.015655, %bb6.i.i565 ]
  %history.i.i451.sroa.22.sroa.0.015653 = phi <8 x float> [ %history.i.i451.sroa.22.sroa.0.0.copyload, %bb6.i.i565.lr.ph ], [ %history.i.i451.sroa.19.sroa.0.015654, %bb6.i.i565 ]
  %history.i.i451.sroa.38.sroa.0.015652 = phi <8 x float> [ %history.i.i451.sroa.38.sroa.0.0.copyload, %bb6.i.i565.lr.ph ], [ %history.i.i451.sroa.35.sroa.0.015651, %bb6.i.i565 ]
  %history.i.i451.sroa.35.sroa.0.015651 = phi <8 x float> [ %history.i.i451.sroa.35.sroa.0.0.copyload, %bb6.i.i565.lr.ph ], [ %history.i.i451.sroa.32.sroa.0.015650, %bb6.i.i565 ]
  %history.i.i451.sroa.32.sroa.0.015650 = phi <8 x float> [ %history.i.i451.sroa.32.sroa.0.0.copyload, %bb6.i.i565.lr.ph ], [ %history.i.i451.sroa.29.sroa.0.015649, %bb6.i.i565 ]
  %history.i.i451.sroa.29.sroa.0.015649 = phi <8 x float> [ %history.i.i451.sroa.29.sroa.0.0.copyload, %bb6.i.i565.lr.ph ], [ %history.i.i451.sroa.25.sroa.0.015648, %bb6.i.i565 ]
  %history.i.i451.sroa.25.sroa.0.015648 = phi <8 x float> [ %history.i.i451.sroa.25.sroa.0.0.copyload, %bb6.i.i565.lr.ph ], [ %history.i.i451.sroa.22.sroa.0.015653, %bb6.i.i565 ]
  %iter.i.i447.sroa.16.015647 = phi i64 [ 0, %bb6.i.i565.lr.ph ], [ %1377, %bb6.i.i565 ]
  %history.i.i451.sroa.0.015646 = phi <8 x float> [ %history.i.i451.sroa.0.0.copyload, %bb6.i.i565.lr.ph ], [ %lanes.i6083.sroa.0.0.copyload, %bb6.i.i565 ]
  %start1.i.i7606 = shl i64 %iter.i.i447.sroa.16.015647, 3, !dbg !31983
  %data.i.i7607 = getelementptr inbounds nuw float, ptr %_197.i562, i64 %start1.i.i7606, !dbg !31985
  %lanes.i6083.sroa.0.0.copyload = load <8 x float>, ptr %data.i.i7607, align 4, !dbg !31987, !alias.scope !31992, !noalias !31996
  %1272 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i.i451.sroa.22.sroa.0.015653), !dbg !32001
  %1273 = fmul <8 x float> %lanes.i6083.sroa.0.0.copyload, %_5.i5107, !dbg !32008
  %1274 = fadd <8 x float> %1273, zeroinitializer, !dbg !32014
  %1275 = fmul <8 x float> %lanes.i6083.sroa.0.0.copyload, %_14.i.i.i.i411.sroa.0.0.copyload, !dbg !32019
  %1276 = fadd <8 x float> %1275, zeroinitializer, !dbg !32024
  %1277 = fmul <8 x float> %lanes.i6083.sroa.0.0.copyload, %_17.i.i.i.i408.sroa.0.0.copyload, !dbg !32029
  %1278 = fadd <8 x float> %1277, zeroinitializer, !dbg !32034
  %1279 = fmul <8 x float> %lanes.i6083.sroa.0.0.copyload, %_20.i.i.i.i405.sroa.0.0.copyload, !dbg !32039
  %1280 = fadd <8 x float> %1279, zeroinitializer, !dbg !32044
  %1281 = fmul <8 x float> %history.i.i451.sroa.0.015646, %_25.i.i.i.i401.sroa.0.0.copyload, !dbg !32049
  %1282 = fadd <8 x float> %1274, %1281, !dbg !32054
  %1283 = fmul <8 x float> %history.i.i451.sroa.0.015646, %_28.i.i.i.i398.sroa.0.0.copyload, !dbg !32059
  %1284 = fadd <8 x float> %1276, %1283, !dbg !32064
  %1285 = fmul <8 x float> %history.i.i451.sroa.0.015646, %_31.i.i.i.i395.sroa.0.0.copyload, !dbg !32069
  %1286 = fadd <8 x float> %1278, %1285, !dbg !32074
  %1287 = fmul <8 x float> %history.i.i451.sroa.0.015646, %_34.i.i.i.i392.sroa.0.0.copyload, !dbg !32079
  %1288 = fadd <8 x float> %1280, %1287, !dbg !32084
  %1289 = fmul <8 x float> %history.i.i451.sroa.10.sroa.0.015657, %_39.i.i.i.i388.sroa.0.0.copyload, !dbg !32089
  %1290 = fadd <8 x float> %1282, %1289, !dbg !32094
  %1291 = fmul <8 x float> %history.i.i451.sroa.10.sroa.0.015657, %_42.i.i.i.i385.sroa.0.0.copyload, !dbg !32099
  %1292 = fadd <8 x float> %1284, %1291, !dbg !32104
  %1293 = fmul <8 x float> %history.i.i451.sroa.10.sroa.0.015657, %_45.i.i.i.i382.sroa.0.0.copyload, !dbg !32109
  %1294 = fadd <8 x float> %1286, %1293, !dbg !32114
  %1295 = fmul <8 x float> %history.i.i451.sroa.10.sroa.0.015657, %_48.i.i.i.i379.sroa.0.0.copyload, !dbg !32119
  %1296 = fadd <8 x float> %1288, %1295, !dbg !32124
  %1297 = fmul <8 x float> %history.i.i451.sroa.13.sroa.0.015656, %_53.i.i.i.i375.sroa.0.0.copyload, !dbg !32129
  %1298 = fadd <8 x float> %1290, %1297, !dbg !32134
  %1299 = fmul <8 x float> %history.i.i451.sroa.13.sroa.0.015656, %_56.i.i.i.i372.sroa.0.0.copyload, !dbg !32139
  %1300 = fadd <8 x float> %1292, %1299, !dbg !32144
  %1301 = fmul <8 x float> %history.i.i451.sroa.13.sroa.0.015656, %_59.i.i.i.i369.sroa.0.0.copyload, !dbg !32149
  %1302 = fadd <8 x float> %1294, %1301, !dbg !32154
  %1303 = fmul <8 x float> %history.i.i451.sroa.13.sroa.0.015656, %_62.i.i.i.i366.sroa.0.0.copyload, !dbg !32159
  %1304 = fadd <8 x float> %1296, %1303, !dbg !32164
  %1305 = fmul <8 x float> %history.i.i451.sroa.16.sroa.0.015655, %_67.i.i.i.i362.sroa.0.0.copyload, !dbg !32169
  %1306 = fadd <8 x float> %1298, %1305, !dbg !32174
  %1307 = fmul <8 x float> %history.i.i451.sroa.16.sroa.0.015655, %_70.i.i.i.i359.sroa.0.0.copyload, !dbg !32179
  %1308 = fadd <8 x float> %1300, %1307, !dbg !32184
  %1309 = fmul <8 x float> %history.i.i451.sroa.16.sroa.0.015655, %_73.i.i.i.i356.sroa.0.0.copyload, !dbg !32189
  %1310 = fadd <8 x float> %1302, %1309, !dbg !32194
  %1311 = fmul <8 x float> %history.i.i451.sroa.16.sroa.0.015655, %_76.i.i.i.i353.sroa.0.0.copyload, !dbg !32199
  %1312 = fadd <8 x float> %1304, %1311, !dbg !32204
  %1313 = fmul <8 x float> %history.i.i451.sroa.19.sroa.0.015654, %_81.i.i.i.i349.sroa.0.0.copyload, !dbg !32209
  %1314 = fadd <8 x float> %1306, %1313, !dbg !32214
  %1315 = fmul <8 x float> %history.i.i451.sroa.19.sroa.0.015654, %_84.i.i.i.i346.sroa.0.0.copyload, !dbg !32219
  %1316 = fadd <8 x float> %1308, %1315, !dbg !32224
  %1317 = fmul <8 x float> %history.i.i451.sroa.19.sroa.0.015654, %_87.i.i.i.i343.sroa.0.0.copyload, !dbg !32229
  %1318 = fadd <8 x float> %1310, %1317, !dbg !32234
  %1319 = fmul <8 x float> %history.i.i451.sroa.19.sroa.0.015654, %_90.i.i.i.i340.sroa.0.0.copyload, !dbg !32239
  %1320 = fadd <8 x float> %1312, %1319, !dbg !32244
  %1321 = fmul <8 x float> %history.i.i451.sroa.22.sroa.0.015653, %_95.i.i.i.i336.sroa.0.0.copyload, !dbg !32249
  %1322 = fadd <8 x float> %1314, %1321, !dbg !32254
  %1323 = fmul <8 x float> %history.i.i451.sroa.22.sroa.0.015653, %_98.i.i.i.i333.sroa.0.0.copyload, !dbg !32259
  %1324 = fadd <8 x float> %1316, %1323, !dbg !32264
  %1325 = fmul <8 x float> %history.i.i451.sroa.22.sroa.0.015653, %_101.i.i.i.i330.sroa.0.0.copyload, !dbg !32269
  %1326 = fadd <8 x float> %1318, %1325, !dbg !32274
  %1327 = fmul <8 x float> %history.i.i451.sroa.22.sroa.0.015653, %_104.i.i.i.i327.sroa.0.0.copyload, !dbg !32279
  %1328 = fadd <8 x float> %1320, %1327, !dbg !32284
  %1329 = fmul <8 x float> %history.i.i451.sroa.25.sroa.0.015648, %_109.i.i.i.i323.sroa.0.0.copyload, !dbg !32289
  %1330 = fadd <8 x float> %1322, %1329, !dbg !32294
  %1331 = fmul <8 x float> %history.i.i451.sroa.25.sroa.0.015648, %_112.i.i.i.i320.sroa.0.0.copyload, !dbg !32299
  %1332 = fadd <8 x float> %1324, %1331, !dbg !32304
  %1333 = fmul <8 x float> %history.i.i451.sroa.25.sroa.0.015648, %_115.i.i.i.i317.sroa.0.0.copyload, !dbg !32309
  %1334 = fadd <8 x float> %1326, %1333, !dbg !32314
  %1335 = fmul <8 x float> %history.i.i451.sroa.25.sroa.0.015648, %_118.i.i.i.i314.sroa.0.0.copyload, !dbg !32319
  %1336 = fadd <8 x float> %1328, %1335, !dbg !32324
  %1337 = fmul <8 x float> %history.i.i451.sroa.29.sroa.0.015649, %_123.i.i.i.i310.sroa.0.0.copyload, !dbg !32329
  %1338 = fadd <8 x float> %1330, %1337, !dbg !32334
  %1339 = fmul <8 x float> %history.i.i451.sroa.29.sroa.0.015649, %_126.i.i.i.i307.sroa.0.0.copyload, !dbg !32339
  %1340 = fadd <8 x float> %1332, %1339, !dbg !32344
  %1341 = fmul <8 x float> %history.i.i451.sroa.29.sroa.0.015649, %_129.i.i.i.i304.sroa.0.0.copyload, !dbg !32349
  %1342 = fadd <8 x float> %1334, %1341, !dbg !32354
  %1343 = fmul <8 x float> %history.i.i451.sroa.29.sroa.0.015649, %_132.i.i.i.i301.sroa.0.0.copyload, !dbg !32359
  %1344 = fadd <8 x float> %1336, %1343, !dbg !32364
  %1345 = fmul <8 x float> %history.i.i451.sroa.32.sroa.0.015650, %_137.i.i.i.i297.sroa.0.0.copyload, !dbg !32369
  %1346 = fadd <8 x float> %1338, %1345, !dbg !32374
  %1347 = fmul <8 x float> %history.i.i451.sroa.32.sroa.0.015650, %_140.i.i.i.i294.sroa.0.0.copyload, !dbg !32379
  %1348 = fadd <8 x float> %1340, %1347, !dbg !32384
  %1349 = fmul <8 x float> %history.i.i451.sroa.32.sroa.0.015650, %_143.i.i.i.i291.sroa.0.0.copyload, !dbg !32389
  %1350 = fadd <8 x float> %1342, %1349, !dbg !32394
  %1351 = fmul <8 x float> %history.i.i451.sroa.32.sroa.0.015650, %_146.i.i.i.i288.sroa.0.0.copyload, !dbg !32399
  %1352 = fadd <8 x float> %1344, %1351, !dbg !32404
  %1353 = fmul <8 x float> %history.i.i451.sroa.35.sroa.0.015651, %_151.i.i.i.i284.sroa.0.0.copyload, !dbg !32409
  %1354 = fadd <8 x float> %1346, %1353, !dbg !32414
  %1355 = fmul <8 x float> %history.i.i451.sroa.35.sroa.0.015651, %_154.i.i.i.i281.sroa.0.0.copyload, !dbg !32419
  %1356 = fadd <8 x float> %1348, %1355, !dbg !32424
  %1357 = fmul <8 x float> %history.i.i451.sroa.35.sroa.0.015651, %_157.i.i.i.i278.sroa.0.0.copyload, !dbg !32429
  %1358 = fadd <8 x float> %1350, %1357, !dbg !32434
  %1359 = fmul <8 x float> %history.i.i451.sroa.35.sroa.0.015651, %_160.i.i.i.i275.sroa.0.0.copyload, !dbg !32439
  %1360 = fadd <8 x float> %1352, %1359, !dbg !32444
  %1361 = fmul <8 x float> %history.i.i451.sroa.38.sroa.0.015652, %_165.i.i.i.i271.sroa.0.0.copyload, !dbg !32449
  %1362 = fadd <8 x float> %1354, %1361, !dbg !32454
  %1363 = fmul <8 x float> %history.i.i451.sroa.38.sroa.0.015652, %_168.i.i.i.i268.sroa.0.0.copyload, !dbg !32459
  %1364 = fadd <8 x float> %1356, %1363, !dbg !32464
  %1365 = fmul <8 x float> %history.i.i451.sroa.38.sroa.0.015652, %_171.i.i.i.i265.sroa.0.0.copyload, !dbg !32469
  %1366 = fadd <8 x float> %1358, %1365, !dbg !32474
  %1367 = fmul <8 x float> %history.i.i451.sroa.38.sroa.0.015652, %_174.i.i.i.i262.sroa.0.0.copyload, !dbg !32479
  %1368 = fadd <8 x float> %1360, %1367, !dbg !32484
  %1369 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1362), !dbg !32489
  %1370 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1272, <8 x float> %1369), !dbg !32495
  %1371 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1364), !dbg !32489
  %1372 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1370, <8 x float> %1371), !dbg !32495
  %1373 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1366), !dbg !32489
  %1374 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1372, <8 x float> %1373), !dbg !32495
  %1375 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1368), !dbg !32489
  %1376 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1374, <8 x float> %1375), !dbg !32495
  %1377 = add nuw nsw i64 %iter.i.i447.sroa.16.015647, 1, !dbg !32500
  %data.i4.i7611 = getelementptr inbounds nuw float, ptr %peaks_right.i491, i64 %start1.i.i7606, !dbg !32501
  store <8 x float> %1376, ptr %data.i4.i7611, align 4, !dbg !32504, !alias.scope !32509, !noalias !32513
  %exitcond18041.not = icmp eq i64 %1377, %umax18040, !dbg !31962
  br i1 %exitcond18041.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i586, label %bb6.i.i565, !dbg !31962

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i586: ; preds = %bb6.i.i565, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7597
  %history.i.i451.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i451.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7597 ], [ %lanes.i6083.sroa.0.0.copyload, %bb6.i.i565 ], !dbg !32517
  %history.i.i451.sroa.25.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i451.sroa.25.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7597 ], [ %history.i.i451.sroa.22.sroa.0.015653, %bb6.i.i565 ], !dbg !32517
  %history.i.i451.sroa.29.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i451.sroa.29.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7597 ], [ %history.i.i451.sroa.25.sroa.0.015648, %bb6.i.i565 ], !dbg !32517
  %history.i.i451.sroa.32.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i451.sroa.32.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7597 ], [ %history.i.i451.sroa.29.sroa.0.015649, %bb6.i.i565 ], !dbg !32517
  %history.i.i451.sroa.35.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i451.sroa.35.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7597 ], [ %history.i.i451.sroa.32.sroa.0.015650, %bb6.i.i565 ], !dbg !32517
  %history.i.i451.sroa.38.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i451.sroa.38.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7597 ], [ %history.i.i451.sroa.35.sroa.0.015651, %bb6.i.i565 ], !dbg !32517
  %history.i.i451.sroa.41.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i451.sroa.41.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7597 ], [ %history.i.i451.sroa.38.sroa.0.015652, %bb6.i.i565 ], !dbg !32517
  %history.i.i451.sroa.22.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i451.sroa.22.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7597 ], [ %history.i.i451.sroa.19.sroa.0.015654, %bb6.i.i565 ], !dbg !32517
  %history.i.i451.sroa.19.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i451.sroa.19.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7597 ], [ %history.i.i451.sroa.16.sroa.0.015655, %bb6.i.i565 ], !dbg !32517
  %history.i.i451.sroa.16.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i451.sroa.16.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7597 ], [ %history.i.i451.sroa.13.sroa.0.015656, %bb6.i.i565 ], !dbg !32517
  %history.i.i451.sroa.13.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i451.sroa.13.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7597 ], [ %history.i.i451.sroa.10.sroa.0.015657, %bb6.i.i565 ], !dbg !32517
  %history.i.i451.sroa.10.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i451.sroa.10.sroa.0.0.copyload, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7597 ], [ %history.i.i451.sroa.0.015646, %bb6.i.i565 ], !dbg !32517
  store <8 x float> %history.i.i451.sroa.0.0.lcssa, ptr %hot_right.i498, align 32, !dbg !32518
  store <8 x float> %history.i.i451.sroa.10.sroa.0.0.lcssa, ptr %history.i.i451.sroa.10.0.hot_right.i498.sroa_idx, align 32, !dbg !32518
  store <8 x float> %history.i.i451.sroa.13.sroa.0.0.lcssa, ptr %history.i.i451.sroa.13.0.hot_right.i498.sroa_idx, align 32, !dbg !32518
  store <8 x float> %history.i.i451.sroa.16.sroa.0.0.lcssa, ptr %history.i.i451.sroa.16.0.hot_right.i498.sroa_idx, align 32, !dbg !32518
  store <8 x float> %history.i.i451.sroa.19.sroa.0.0.lcssa, ptr %history.i.i451.sroa.19.0.hot_right.i498.sroa_idx, align 32, !dbg !32518
  store <8 x float> %history.i.i451.sroa.22.sroa.0.0.lcssa, ptr %history.i.i451.sroa.22.0.hot_right.i498.sroa_idx, align 32, !dbg !32518
  store <8 x float> %history.i.i451.sroa.25.sroa.0.0.lcssa, ptr %history.i.i451.sroa.25.0.hot_right.i498.sroa_idx, align 32, !dbg !32518
  store <8 x float> %history.i.i451.sroa.29.sroa.0.0.lcssa, ptr %history.i.i451.sroa.29.0.hot_right.i498.sroa_idx, align 32, !dbg !32518
  store <8 x float> %history.i.i451.sroa.32.sroa.0.0.lcssa, ptr %history.i.i451.sroa.32.0.hot_right.i498.sroa_idx, align 32, !dbg !32518
  store <8 x float> %history.i.i451.sroa.35.sroa.0.0.lcssa, ptr %history.i.i451.sroa.35.0.hot_right.i498.sroa_idx, align 32, !dbg !32518
  store <8 x float> %history.i.i451.sroa.38.sroa.0.0.lcssa, ptr %history.i.i451.sroa.38.0.hot_right.i498.sroa_idx, align 32, !dbg !32518
  store <8 x float> %history.i.i451.sroa.41.sroa.0.0.lcssa, ptr %history.i.i451.sroa.41.0.hot_right.i498.sroa_idx, align 32, !dbg !32518
  br i1 %_2.i754015618.not, label %bb15.i515.loopexit, label %bb20.i592.preheader, !dbg !32519

bb20.i592.preheader:                              ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i586
  %_70.i486.sroa.3.0.copyload = load i64, ptr %_70.i486.sroa.3.0..sroa_idx, align 8, !noalias !31323
  %_70.i486.sroa.4.0.copyload = load i64, ptr %_70.i486.sroa.4.0..sroa_idx, align 16, !noalias !31323
  %_71.i485.sroa.3.0.copyload = load i64, ptr %_71.i485.sroa.3.0..sroa_idx, align 8, !noalias !31323
  %_71.i485.sroa.4.0.copyload = load i64, ptr %_71.i485.sroa.4.0..sroa_idx, align 16, !noalias !31323
  %_54.0.i278.i644 = load ptr, ptr %1144, align 32, !nonnull !12, !align !24
  %_54.1.i279.i645 = load i64, ptr %1145, align 8
  %_18.i290.i656 = load i64, ptr %1128, align 16
  %_56.0.i294.i660 = load ptr, ptr %1146, align 16, !nonnull !12, !align !24
  %_56.1.i295.i661 = load i64, ptr %1147, align 8
  %_58.1.i307.i673 = load i64, ptr %1151, align 8
  %_58.0.i306.i672 = load ptr, ptr %1152, align 32, !nonnull !12, !align !24
  %_54.0.i.i694 = load ptr, ptr %1155, align 32, !nonnull !12, !align !24
  %_54.1.i.i695 = load i64, ptr %1156, align 8
  %_18.i.i706 = load i64, ptr %1129, align 16
  %_56.0.i.i710 = load ptr, ptr %1157, align 16, !nonnull !12, !align !24
  %_56.1.i.i711 = load i64, ptr %1158, align 8
  %_58.1.i.i723 = load i64, ptr %1162, align 8
  %_58.0.i.i722 = load ptr, ptr %1163, align 32, !nonnull !12, !align !24
  %_22.i293.i659.promoted20829 = load i32, ptr %_22.i293.i659, align 4
  %uniform_left.i490.promoted20848 = load <8 x float>, ptr %uniform_left.i490, align 32
  %_22.i.i709.promoted20867 = load i32, ptr %_22.i.i709, align 4
  %uniform_right.i489.promoted20886 = load <8 x float>, ptr %uniform_right.i489, align 32
  br label %bb20.i592, !dbg !32521

bb20.i592:                                        ; preds = %bb20.i592.preheader, %bb32.i740
  %minimum.i.i67.sroa.0.0.lcssa2082620888 = phi <8 x float> [ %uniform_right.i489.promoted20886, %bb20.i592.preheader ], [ %minimum.i.i67.sroa.0.0.lcssa2082620887, %bb32.i740 ]
  %storemerge.i.lcssa2081220869 = phi i32 [ %_22.i.i709.promoted20867, %bb20.i592.preheader ], [ %storemerge.i.lcssa2081220868, %bb32.i740 ]
  %minimum.i273.i35.sroa.0.0.lcssa2079620850 = phi <8 x float> [ %uniform_left.i490.promoted20848, %bb20.i592.preheader ], [ %minimum.i273.i35.sroa.0.0.lcssa2079620849, %bb32.i740 ]
  %storemerge.i1813.lcssa2078220831 = phi i32 [ %_22.i293.i659.promoted20829, %bb20.i592.preheader ], [ %storemerge.i1813.lcssa2078220830, %bb32.i740 ]
  %frame.sroa.0.0.i59015684 = phi i64 [ 0, %bb20.i592.preheader ], [ %_85.i608, %bb32.i740 ]
  %main_cursor.sroa.0.1.i58915683 = phi i64 [ %main_cursor.sroa.0.0.i51715689, %bb20.i592.preheader ], [ %main_cursor.sroa.0.2.i746, %bb32.i740 ]
  %ring_cursor.sroa.0.1.i58815682 = phi i64 [ %ring_cursor.sroa.0.0.i51615688, %bb20.i592.preheader ], [ %ring_cursor.sroa.0.2.i743, %bb32.i740 ]
  %_68.i593 = sub nuw nsw i64 %..i7507, %frame.sroa.0.0.i59015684, !dbg !32533
  %ring.i2483 = load i64, ptr %85, align 8, !dbg !32534, !alias.scope !32536, !noalias !32539, !noundef !12
  %main.i2484 = load i64, ptr %86, align 8, !dbg !32543, !alias.scope !32536, !noalias !32539, !noundef !12
  %_10.i = add i64 %ring_cursor.sroa.0.1.i58815682, 1, !dbg !32544
  %_45.not.i = icmp ult i64 %_10.i, %ring.i2483, !dbg !32545
  %1378 = select i1 %_45.not.i, i64 0, i64 %ring.i2483, !dbg !32545
  %start1.sroa.0.0.i2485 = sub nuw i64 %_10.i, %1378, !dbg !32545
  %_12.i2487 = add i64 %_70.i486.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i58815682, !dbg !32547
  %_46.not.i = icmp ult i64 %_12.i2487, %ring.i2483, !dbg !32548
  %1379 = select i1 %_46.not.i, i64 0, i64 %ring.i2483, !dbg !32548
  %left_end.sroa.0.0.i = sub nuw i64 %_12.i2487, %1379, !dbg !32548
  %_15.i2489 = add i64 %_71.i485.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i58815682, !dbg !32550
  %_47.not.i = icmp ult i64 %_15.i2489, %ring.i2483, !dbg !32551
  %1380 = select i1 %_47.not.i, i64 0, i64 %ring.i2483, !dbg !32551
  %right_end.sroa.0.0.i = sub nuw i64 %_15.i2489, %1380, !dbg !32551
  %_18.i2491 = add i64 %_70.i486.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i58815682, !dbg !32553
  %_48.not.i = icmp ult i64 %_18.i2491, %ring.i2483, !dbg !32554
  %1381 = select i1 %_48.not.i, i64 0, i64 %ring.i2483, !dbg !32554
  %left_expiring.sroa.0.0.i = sub nuw i64 %_18.i2491, %1381, !dbg !32554
  %_21.i2493 = add i64 %_71.i485.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i58815682, !dbg !32556
  %_49.not.i = icmp ult i64 %_21.i2493, %ring.i2483, !dbg !32557
  %1382 = select i1 %_49.not.i, i64 0, i64 %ring.i2483, !dbg !32557
  %right_expiring.sroa.0.0.i = sub nuw i64 %_21.i2493, %1382, !dbg !32557
  %_30.i2494 = sub i64 %ring.i2483, %ring_cursor.sroa.0.1.i58815682, !dbg !32559
  %..i7628 = tail call noundef i64 @llvm.umin.i64(i64 %_30.i2494, i64 %_68.i593), !dbg !32560
  %_31.i2496 = sub i64 %main.i2484, %main_cursor.sroa.0.1.i58915683, !dbg !32562
  %..i7629 = tail call noundef i64 @llvm.umin.i64(i64 %_31.i2496, i64 %..i7628), !dbg !32563
  %_32.i2498 = sub i64 %ring.i2483, %start1.sroa.0.0.i2485, !dbg !32565
  %..i7630 = tail call noundef i64 @llvm.umin.i64(i64 %_32.i2498, i64 %..i7629), !dbg !32566
  %_34.i2500 = sub i64 %ring.i2483, %left_end.sroa.0.0.i, !dbg !32568
  %..i7631 = tail call noundef i64 @llvm.umin.i64(i64 %_34.i2500, i64 %..i7630), !dbg !32569
  %_36.i2502 = sub i64 %ring.i2483, %right_end.sroa.0.0.i, !dbg !32571
  %..i7632 = tail call noundef i64 @llvm.umin.i64(i64 %_36.i2502, i64 %..i7631), !dbg !32572
  %_38.i = sub i64 %ring.i2483, %left_expiring.sroa.0.0.i, !dbg !32574
  %..i7633 = tail call noundef i64 @llvm.umin.i64(i64 %_38.i, i64 %..i7632), !dbg !32575
  %_40.i2503 = sub i64 %ring.i2483, %right_expiring.sroa.0.0.i, !dbg !32577
  %..i7634 = tail call noundef i64 @llvm.umin.i64(i64 %_40.i2503, i64 %..i7633), !dbg !32578
  %_74.i595 = add i64 %frame.sroa.0.0.i59015684, %iter3.sroa.0.0.i51815690, !dbg !32580
  %base.i596 = shl i64 %_74.i595, 3, !dbg !32580
  %base.i59613437 = add i64 %..i7634, %_74.i595, !dbg !32581
  %_78.i598 = shl i64 %base.i59613437, 3, !dbg !32581
  %_206.i599 = icmp ult i64 %_78.i598, %base.i596, !dbg !32521
  %_202.not.i600 = icmp ugt i64 %_78.i598, %left_io.1
  %or.cond23.i601 = or i1 %_206.i599, %_202.not.i600, !dbg !32521
  br i1 %or.cond23.i601, label %bb62.i748, label %bb61.i602, !dbg !32521, !prof !165

bb62.i748:                                        ; preds = %bb20.i592
  store i32 %storemerge.i1813.lcssa2078220831, ptr %_22.i293.i659, align 4
  store <8 x float> %minimum.i273.i35.sroa.0.0.lcssa2079620850, ptr %uniform_left.i490, align 32
  store i32 %storemerge.i.lcssa2081220869, ptr %_22.i.i709, align 4
  store <8 x float> %minimum.i.i67.sroa.0.0.lcssa2082620888, ptr %uniform_right.i489, align 32
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i596, i64 noundef %_78.i598, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_debdb702bca99cd8ca93ec127a5c320a) #30, !dbg !32582, !noalias !31294
  unreachable, !dbg !32582

bb61.i602:                                        ; preds = %bb20.i592
  %_209.i603 = getelementptr inbounds nuw float, ptr %left_io.0, i64 %base.i596, !dbg !32583
  %_210.not.i604 = icmp ugt i64 %_78.i598, %right_io.1, !dbg !32587
  br i1 %_210.not.i604, label %bb65.i747, label %bb64.i605, !dbg !32587, !prof !1406

bb65.i747:                                        ; preds = %bb61.i602
  store i32 %storemerge.i1813.lcssa2078220831, ptr %_22.i293.i659, align 4
  store <8 x float> %minimum.i273.i35.sroa.0.0.lcssa2079620850, ptr %uniform_left.i490, align 32
  store i32 %storemerge.i.lcssa2081220869, ptr %_22.i.i709, align 4
  store <8 x float> %minimum.i.i67.sroa.0.0.lcssa2082620888, ptr %uniform_right.i489, align 32
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i596, i64 noundef %_78.i598, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_01912ef845d3ed33a157086defa4d008) #30, !dbg !32592, !noalias !31294
  unreachable, !dbg !32592

bb64.i605:                                        ; preds = %bb61.i602
  %_215.i606 = getelementptr inbounds nuw float, ptr %right_io.0, i64 %base.i596, !dbg !32593
  %_82.i607 = shl nuw nsw i64 %frame.sroa.0.0.i59015684, 3, !dbg !32597
  %_85.i608 = add nuw nsw i64 %..i7634, %frame.sroa.0.0.i59015684, !dbg !32599
  %_217.i613 = icmp ult i64 %_85.i608, 33
  br i1 %_217.i613, label %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit, label %bb67.i614, !dbg !32600, !prof !2740

bb67.i614:                                        ; preds = %bb64.i605
  store i32 %storemerge.i1813.lcssa2078220831, ptr %_22.i293.i659, align 4
  store <8 x float> %minimum.i273.i35.sroa.0.0.lcssa2079620850, ptr %uniform_left.i490, align 32
  store i32 %storemerge.i.lcssa2081220869, ptr %_22.i.i709, align 4
  store <8 x float> %minimum.i.i67.sroa.0.0.lcssa2082620888, ptr %uniform_right.i489, align 32
  %_84.i609 = shl nuw nsw i64 %_85.i608, 3, !dbg !32599
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_82.i607, i64 noundef %_84.i609, i64 noundef 256, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_0ad51c1dc139f477f06f604c0b5d4a59) #30, !dbg !32608, !noalias !31294
  unreachable, !dbg !32608

_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit: ; preds = %bb64.i605
  %_224.i616 = getelementptr inbounds nuw float, ptr %peaks_left.i492, i64 %_82.i607, !dbg !32609
  %_233.i617 = getelementptr inbounds nuw float, ptr %peaks_right.i491, i64 %_82.i607, !dbg !32612
  %_2.i.i.i15678.not = icmp eq i64 %..i7634, 0, !dbg !32622
  br i1 %_2.i.i.i15678.not, label %bb32.i740, label %bb31.i621.preheader, !dbg !32622

bb31.i621.preheader:                              ; preds = %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit
  %umin18058 = call i64 @llvm.umin.i64(i64 %_34.i2500, i64 %_36.i2502)
  %umin18059 = call i64 @llvm.umin.i64(i64 %umin18058, i64 %_38.i)
  %umin18060 = call i64 @llvm.umin.i64(i64 %umin18059, i64 %_40.i2503)
  %umin18061 = call i64 @llvm.umin.i64(i64 %umin18060, i64 %_32.i2498)
  %umin18062 = call i64 @llvm.umin.i64(i64 %umin18061, i64 %_30.i2494)
  %umin18063 = call i64 @llvm.umin.i64(i64 %umin18062, i64 %_31.i2496)
  %1383 = sub nsw i64 %umin18064, %frame.sroa.0.0.i59015684
  %umin18065 = call i64 @llvm.umin.i64(i64 %umin18063, i64 %1383)
  %1384 = and i64 %umin18065, 2305843009213693951
  %_11.i2630.sroa.0.0.copyload.pre = load <8 x float>, ptr %_115.i631, align 32, !dbg !32628
  %_12.i2629.sroa.0.0.copyload.pre = load <8 x float>, ptr %1134, align 32, !dbg !32632
  %_5.i2622.sroa.0.0.copyload.pre = load <8 x float>, ptr %1136, align 32, !dbg !32633
  %_11.i2616.sroa.0.0.copyload.pre = load <8 x float>, ptr %_119.i632, align 32, !dbg !32637
  %_12.i2615.sroa.0.0.copyload.pre = load <8 x float>, ptr %1137, align 32, !dbg !32638
  %_13.i2614.sroa.0.0.copyload.pre = load <8 x float>, ptr %1138, align 32, !dbg !32639
  %_5.i2608.sroa.0.0.copyload.pre = load <8 x float>, ptr %1139, align 32, !dbg !32640
  %_13.i2642.sroa.0.0.copyload = load <8 x float>, ptr %1132, align 32
  %_13.i2628.sroa.0.0.copyload = load <8 x float>, ptr %1135, align 32
  %_13.i2600.sroa.0.0.copyload = load <8 x float>, ptr %1141, align 32
  %_37.i261.i23.sroa.0.0.copyload = load <8 x float>, ptr %1149, align 32
  %_37.i.i55.sroa.0.0.copyload = load <8 x float>, ptr %1160, align 32
  %.promoted20581 = load <8 x float>, ptr %1130, align 32
  %_114.i630.promoted = load <8 x float>, ptr %_114.i630, align 32
  %.promoted20613 = load <8 x float>, ptr %1131, align 32
  %.promoted20630 = load <8 x float>, ptr %1133, align 32
  %_120.i633.promoted = load <8 x float>, ptr %_120.i633, align 32
  %.promoted20752 = load <8 x float>, ptr %1140, align 32
  %.promoted20797 = load <8 x float>, ptr %1148, align 32
  %.promoted20827 = load <8 x float>, ptr %1159, align 32
  br label %bb31.i621

bb31.i621:                                        ; preds = %bb31.i621.preheader, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2050
  %_33.i.i58.sroa.0.0.copyload20828 = phi <8 x float> [ %.promoted20827, %bb31.i621.preheader ], [ %1481, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2050 ]
  %minimum.i.i67.sroa.0.020813 = phi <8 x float> [ %minimum.i.i67.sroa.0.0.lcssa2082620888, %bb31.i621.preheader ], [ %minimum.i.i67.sroa.0.0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2050 ]
  %storemerge.i20799 = phi i32 [ %storemerge.i.lcssa2081220869, %bb31.i621.preheader ], [ %storemerge.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2050 ]
  %_33.i264.i26.sroa.0.0.copyload20798 = phi <8 x float> [ %.promoted20797, %bb31.i621.preheader ], [ %1443, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2050 ]
  %minimum.i273.i35.sroa.0.020783 = phi <8 x float> [ %minimum.i273.i35.sroa.0.0.lcssa2079620850, %bb31.i621.preheader ], [ %minimum.i273.i35.sroa.0.0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2050 ]
  %storemerge.i181320769 = phi i32 [ %storemerge.i1813.lcssa2078220831, %bb31.i621.preheader ], [ %storemerge.i1813, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2050 ]
  %_12.i2601.sroa.0.0.copyload20753 = phi <8 x float> [ %.promoted20752, %bb31.i621.preheader ], [ %1418, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2050 ], !dbg !32642
  %_11.i2602.sroa.0.0.copyload20736 = phi <8 x float> [ %_120.i633.promoted, %bb31.i621.preheader ], [ %1417, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2050 ], !dbg !32642
  %1385 = phi <8 x float> [ %.promoted20630, %bb31.i621.preheader ], [ %1396, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2050 ], !dbg !32642
  %_12.i2643.sroa.0.0.copyload20614 = phi <8 x float> [ %.promoted20613, %bb31.i621.preheader ], [ %1394, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2050 ], !dbg !32642
  %_11.i2644.sroa.0.0.copyload20597 = phi <8 x float> [ %_114.i630.promoted, %bb31.i621.preheader ], [ %1393, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2050 ], !dbg !32642
  %1386 = phi <8 x float> [ %.promoted20581, %bb31.i621.preheader ], [ %1388, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2050 ], !dbg !32648
  %_5.i2608.sroa.0.0.copyload = phi <8 x float> [ %_5.i2608.sroa.0.0.copyload.pre, %bb31.i621.preheader ], [ %1412, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2050 ], !dbg !32640
  %_12.i2615.sroa.0.0.copyload = phi <8 x float> [ %_12.i2615.sroa.0.0.copyload.pre, %bb31.i621.preheader ], [ %1410, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2050 ], !dbg !32638
  %_11.i2616.sroa.0.0.copyload = phi <8 x float> [ %_11.i2616.sroa.0.0.copyload.pre, %bb31.i621.preheader ], [ %1409, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2050 ], !dbg !32637
  %_5.i2622.sroa.0.0.copyload = phi <8 x float> [ %_5.i2622.sroa.0.0.copyload.pre, %bb31.i621.preheader ], [ %1404, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2050 ], !dbg !32633
  %_12.i2629.sroa.0.0.copyload = phi <8 x float> [ %_12.i2629.sroa.0.0.copyload.pre, %bb31.i621.preheader ], [ %1402, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2050 ], !dbg !32632
  %_11.i2630.sroa.0.0.copyload = phi <8 x float> [ %_11.i2630.sroa.0.0.copyload.pre, %bb31.i621.preheader ], [ %1401, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2050 ], !dbg !32628
  %iter.i476.sroa.41.015680 = phi i64 [ 0, %bb31.i621.preheader ], [ %_9.0.i7706, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2050 ]
  %1387 = fadd <8 x float> %1386, splat (float -1.000000e+00), !dbg !32642
  %1388 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1387, <8 x float> zeroinitializer), !dbg !32649
  %1389 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1388, <8 x float> zeroinitializer, i8 30), !dbg !32654
  %1390 = fadd <8 x float> %_11.i2644.sroa.0.0.copyload20597, %_12.i2643.sroa.0.0.copyload20614, !dbg !32660
  %1391 = bitcast <8 x float> %1389 to <8 x i32>, !dbg !32665
  %1392 = icmp slt <8 x i32> %1391, zeroinitializer, !dbg !32669
  %1393 = select <8 x i1> %1392, <8 x float> %1390, <8 x float> %_13.i2642.sroa.0.0.copyload, !dbg !32669
  %1394 = select <8 x i1> %1392, <8 x float> %_12.i2643.sroa.0.0.copyload20614, <8 x float> zeroinitializer, !dbg !32671
  %1395 = fadd <8 x float> %1385, splat (float -1.000000e+00), !dbg !32676
  %1396 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1395, <8 x float> zeroinitializer), !dbg !32681
  %1397 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1396, <8 x float> zeroinitializer, i8 30), !dbg !32686
  %1398 = fadd <8 x float> %_11.i2630.sroa.0.0.copyload, %_12.i2629.sroa.0.0.copyload, !dbg !32692
  %1399 = bitcast <8 x float> %1397 to <8 x i32>, !dbg !32697
  %1400 = icmp slt <8 x i32> %1399, zeroinitializer, !dbg !32701
  %1401 = select <8 x i1> %1400, <8 x float> %1398, <8 x float> %_13.i2628.sroa.0.0.copyload, !dbg !32701
  %1402 = select <8 x i1> %1400, <8 x float> %_12.i2629.sroa.0.0.copyload, <8 x float> zeroinitializer, !dbg !32703
  %1403 = fadd <8 x float> %_5.i2622.sroa.0.0.copyload, splat (float -1.000000e+00), !dbg !32708
  %1404 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1403, <8 x float> zeroinitializer), !dbg !32713
  %1405 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1404, <8 x float> zeroinitializer, i8 30), !dbg !32718
  %1406 = fadd <8 x float> %_11.i2616.sroa.0.0.copyload, %_12.i2615.sroa.0.0.copyload, !dbg !32724
  %1407 = bitcast <8 x float> %1405 to <8 x i32>, !dbg !32729
  %1408 = icmp slt <8 x i32> %1407, zeroinitializer, !dbg !32733
  %1409 = select <8 x i1> %1408, <8 x float> %1406, <8 x float> %_13.i2614.sroa.0.0.copyload.pre, !dbg !32733
  %1410 = select <8 x i1> %1408, <8 x float> %_12.i2615.sroa.0.0.copyload, <8 x float> zeroinitializer, !dbg !32735
  %1411 = fadd <8 x float> %_5.i2608.sroa.0.0.copyload, splat (float -1.000000e+00), !dbg !32740
  %1412 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1411, <8 x float> zeroinitializer), !dbg !32745
  %1413 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1412, <8 x float> zeroinitializer, i8 30), !dbg !32750
  %1414 = fadd <8 x float> %_11.i2602.sroa.0.0.copyload20736, %_12.i2601.sroa.0.0.copyload20753, !dbg !32756
  %1415 = bitcast <8 x float> %1413 to <8 x i32>, !dbg !32761
  %1416 = icmp slt <8 x i32> %1415, zeroinitializer, !dbg !32765
  %1417 = select <8 x i1> %1416, <8 x float> %1414, <8 x float> %_13.i2600.sroa.0.0.copyload, !dbg !32765
  %1418 = select <8 x i1> %1416, <8 x float> %_12.i2601.sroa.0.0.copyload20753, <8 x float> zeroinitializer, !dbg !32767
  %start1.i.i.i.i.i.i.i.i = shl i64 %iter.i476.sroa.41.015680, 3, !dbg !32772
  %data.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_224.i616, i64 %start1.i.i.i.i.i.i.i.i, !dbg !32778
  %lanes.i6074.sroa.0.0.copyload = load <8 x float>, ptr %data.i.i.i.i.i.i, align 4, !dbg !32781, !alias.scope !32787, !noalias !32791
  %data.i.i.i.i7704 = getelementptr inbounds nuw float, ptr %_233.i617, i64 %start1.i.i.i.i.i.i.i.i, !dbg !32795
  %lanes.i6065.sroa.0.0.copyload = load <8 x float>, ptr %data.i.i.i.i7704, align 4, !dbg !32798, !alias.scope !32804, !noalias !32808
  %data.i.i.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_209.i603, i64 %start1.i.i.i.i.i.i.i.i, !dbg !32812
  %data.i5.i.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_215.i606, i64 %start1.i.i.i.i.i.i.i.i, !dbg !32814
  %_9.0.i7706 = add nuw nsw i64 %iter.i476.sroa.41.015680, 1, !dbg !32817
  %1419 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i6065.sroa.0.0.copyload, <8 x float> %lanes.i6074.sroa.0.0.copyload), !dbg !32818
  %1420 = select <8 x i1> %1143, <8 x float> %1419, <8 x float> %lanes.i6065.sroa.0.0.copyload, !dbg !32824
  %_235.i639 = add i64 %iter.i476.sroa.41.015680, %ring_cursor.sroa.0.1.i58815682, !dbg !32831
  %_236.i640 = add i64 %iter.i476.sroa.41.015680, %main_cursor.sroa.0.1.i58915683, !dbg !32837
  %_237.i641 = add i64 %iter.i476.sroa.41.015680, %left_end.sroa.0.0.i, !dbg !32838
  %_238.i642 = add i64 %iter.i476.sroa.41.015680, %start1.sroa.0.0.i2485, !dbg !32839
  %_239.i643 = add i64 %iter.i476.sroa.41.015680, %left_expiring.sroa.0.0.i, !dbg !32840
  %base.i9.i281.i647 = shl i64 %_235.i639, 3, !dbg !32841
  %_7.i10.i282.i648 = add i64 %base.i9.i281.i647, 8, !dbg !32844
  %1421 = or disjoint i64 %base.i9.i281.i647, 7, !dbg !32845
  %or.cond.i13.i285.i651.not = icmp ult i64 %1421, %_54.1.i279.i645, !dbg !32845
  br i1 %or.cond.i13.i285.i651.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i286.i652, label %bb4.i15.i319.i739, !dbg !32845, !prof !2740

bb4.i15.i319.i739:                                ; preds = %bb31.i621
  store i32 %storemerge.i1813.lcssa2078220831, ptr %_22.i293.i659, align 4
  store <8 x float> %minimum.i273.i35.sroa.0.0.lcssa2079620850, ptr %uniform_left.i490, align 32
  store i32 %storemerge.i.lcssa2081220869, ptr %_22.i.i709, align 4
  store <8 x float> %minimum.i.i67.sroa.0.0.lcssa2082620888, ptr %uniform_right.i489, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32849
  store <8 x float> %1393, ptr %_114.i630, align 32, !dbg !32850
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32851
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32852
  store <8 x float> %1401, ptr %_115.i631, align 32, !dbg !32853
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32854
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32855
  store <8 x float> %1409, ptr %_119.i632, align 32, !dbg !32856
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32857
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32858
  store <8 x float> %1417, ptr %_120.i633, align 32, !dbg !32859
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32860
  store i32 %storemerge.i181320769, ptr %_22.i293.i659, align 4, !dbg !32861
  store <8 x float> %minimum.i273.i35.sroa.0.020783, ptr %uniform_left.i490, align 32, !dbg !32863
  store i32 %storemerge.i20799, ptr %_22.i.i709, align 4, !dbg !32864
  store <8 x float> %minimum.i.i67.sroa.0.020813, ptr %uniform_right.i489, align 32, !dbg !32867
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i281.i647, i64 noundef %_7.i10.i282.i648, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i279.i645, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !32868, !noalias !32869
  unreachable, !dbg !32868

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i286.i652: ; preds = %bb31.i621
  %1422 = select <8 x i1> %1143, <8 x float> %1419, <8 x float> %lanes.i6074.sroa.0.0.copyload, !dbg !32883
  %1423 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1422, <8 x float> %1393, i8 30), !dbg !32888
  %1424 = bitcast <8 x float> %1423 to <8 x i32>, !dbg !32894
  %1425 = icmp slt <8 x i32> %1424, zeroinitializer, !dbg !32898
  %1426 = fdiv <8 x float> %1393, %1422, !dbg !32900
  %1427 = select <8 x i1> %1425, <8 x float> %1426, <8 x float> splat (float 1.000000e+00), !dbg !32898
  %_17.i14.i287.i653 = getelementptr inbounds nuw float, ptr %_54.0.i278.i644, i64 %base.i9.i281.i647, !dbg !32905
  store <8 x float> %1427, ptr %_17.i14.i287.i653, align 4, !dbg !32907, !alias.scope !32912, !noalias !32916
  %base.i1997 = shl i64 %_237.i641, 3, !dbg !32920
  %1428 = or disjoint i64 %base.i1997, 7, !dbg !32922
  %or.cond.i2001.not = icmp ult i64 %1428, %_54.1.i279.i645, !dbg !32922
  br i1 %or.cond.i2001.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2005, label %bb4.i2004, !dbg !32922, !prof !2740

bb4.i2004:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i286.i652
  store i32 %storemerge.i1813.lcssa2078220831, ptr %_22.i293.i659, align 4
  store <8 x float> %minimum.i273.i35.sroa.0.0.lcssa2079620850, ptr %uniform_left.i490, align 32
  store i32 %storemerge.i.lcssa2081220869, ptr %_22.i.i709, align 4
  store <8 x float> %minimum.i.i67.sroa.0.0.lcssa2082620888, ptr %uniform_right.i489, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32849
  store <8 x float> %1393, ptr %_114.i630, align 32, !dbg !32850
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32851
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32852
  store <8 x float> %1401, ptr %_115.i631, align 32, !dbg !32853
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32854
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32855
  store <8 x float> %1409, ptr %_119.i632, align 32, !dbg !32856
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32857
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32858
  store <8 x float> %1417, ptr %_120.i633, align 32, !dbg !32859
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32860
  store i32 %storemerge.i181320769, ptr %_22.i293.i659, align 4, !dbg !32861
  store <8 x float> %minimum.i273.i35.sroa.0.020783, ptr %uniform_left.i490, align 32, !dbg !32863
  store i32 %storemerge.i20799, ptr %_22.i.i709, align 4, !dbg !32864
  store <8 x float> %minimum.i.i67.sroa.0.020813, ptr %uniform_right.i489, align 32, !dbg !32867
  %_5.i1998 = add i64 %base.i1997, 8, !dbg !32926
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1997, i64 noundef %_5.i1998, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i279.i645, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !32927, !noalias !32928
  unreachable, !dbg !32927

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2005: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i286.i652
  %_15.i2003 = getelementptr inbounds nuw float, ptr %_54.0.i278.i644, i64 %base.i1997, !dbg !32936
  %lanes.i5763.sroa.0.0.copyload = load <8 x float>, ptr %_15.i2003, align 4, !dbg !32938, !alias.scope !32943, !noalias !32947
  %position.i1807 = zext i32 %storemerge.i181320769 to i64, !dbg !32951
  %1429 = icmp eq i32 %storemerge.i181320769, 0, !dbg !32952
  br i1 %1429, label %bb5.i1809, label %bb3.i1808, !dbg !32952

bb3.i1808:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2005
  %1430 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %minimum.i273.i35.sroa.0.020783, <8 x float> %lanes.i5763.sroa.0.0.copyload), !dbg !32953
  br label %bb5.i1809, !dbg !32958

bb5.i1809:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2005, %bb3.i1808
  %minimum.i273.i35.sroa.0.0 = phi <8 x float> [ %1430, %bb3.i1808 ], [ %lanes.i5763.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2005 ], !dbg !32863
  %_15.i1810 = add nuw nsw i64 %position.i1807, 1, !dbg !32959
  %complete.i1811 = icmp eq i64 %_15.i1810, %_18.i290.i656, !dbg !32959
  br i1 %complete.i1811, label %bb19.i1819, label %bb7.i1812, !dbg !32960

bb7.i1812:                                        ; preds = %bb5.i1809
  %base.i1988 = shl i64 %_238.i642, 3, !dbg !32961
  %1431 = or disjoint i64 %base.i1988, 7, !dbg !32963
  %or.cond.i1992.not = icmp ult i64 %1431, %_54.1.i279.i645, !dbg !32963
  br i1 %or.cond.i1992.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1996, label %bb4.i1995, !dbg !32963, !prof !2740

bb4.i1995:                                        ; preds = %bb7.i1812
  store i32 %storemerge.i1813.lcssa2078220831, ptr %_22.i293.i659, align 4
  store <8 x float> %minimum.i273.i35.sroa.0.0.lcssa2079620850, ptr %uniform_left.i490, align 32
  store i32 %storemerge.i.lcssa2081220869, ptr %_22.i.i709, align 4
  store <8 x float> %minimum.i.i67.sroa.0.0.lcssa2082620888, ptr %uniform_right.i489, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32849
  store <8 x float> %1393, ptr %_114.i630, align 32, !dbg !32850
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32851
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32852
  store <8 x float> %1401, ptr %_115.i631, align 32, !dbg !32853
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32854
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32855
  store <8 x float> %1409, ptr %_119.i632, align 32, !dbg !32856
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32857
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32858
  store <8 x float> %1417, ptr %_120.i633, align 32, !dbg !32859
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32860
  store i32 %storemerge.i181320769, ptr %_22.i293.i659, align 4, !dbg !32861
  store <8 x float> %minimum.i273.i35.sroa.0.0, ptr %uniform_left.i490, align 32, !dbg !32863
  store i32 %storemerge.i20799, ptr %_22.i.i709, align 4, !dbg !32864
  store <8 x float> %minimum.i.i67.sroa.0.020813, ptr %uniform_right.i489, align 32, !dbg !32867
  %_5.i1989 = add i64 %base.i1988, 8, !dbg !32967
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1988, i64 noundef %_5.i1989, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i279.i645, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !32968, !noalias !32969
  unreachable, !dbg !32968

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1996: ; preds = %bb7.i1812
  %_15.i1994 = getelementptr inbounds nuw float, ptr %_54.0.i278.i644, i64 %base.i1988, !dbg !32973
  %lanes.i5770.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1994, align 4, !dbg !32975, !alias.scope !32980, !noalias !32984
  %1432 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %lanes.i5770.sroa.0.0.copyload, <8 x float> %minimum.i273.i35.sroa.0.0), !dbg !32988
  %1433 = trunc i64 %_15.i1810 to i32, !dbg !32993
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1831, !dbg !32994

bb19.i1819:                                       ; preds = %bb5.i1809, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1978
  %end.sroa.0.0.i181715673 = phi i64 [ %1437, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1978 ], [ %_237.i641, %bb5.i1809 ]
  %iter.sroa.0.0.i181615672 = phi i64 [ %_30.i1820, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1978 ], [ 0, %bb5.i1809 ]
  %suffix.i1800.sroa.0.015671 = phi <8 x float> [ %1435, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1978 ], [ %lanes.i5763.sroa.0.0.copyload, %bb5.i1809 ]
  %base.i1970 = shl i64 %end.sroa.0.0.i181715673, 3, !dbg !32995
  %1434 = or disjoint i64 %base.i1970, 7, !dbg !32997
  %or.cond.i1974.not = icmp ult i64 %1434, %_54.1.i279.i645, !dbg !32997
  br i1 %or.cond.i1974.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1978, label %bb4.i1977, !dbg !32997, !prof !2740

bb4.i1977:                                        ; preds = %bb19.i1819
  store i32 %storemerge.i1813.lcssa2078220831, ptr %_22.i293.i659, align 4
  store <8 x float> %minimum.i273.i35.sroa.0.0.lcssa2079620850, ptr %uniform_left.i490, align 32
  store i32 %storemerge.i.lcssa2081220869, ptr %_22.i.i709, align 4
  store <8 x float> %minimum.i.i67.sroa.0.0.lcssa2082620888, ptr %uniform_right.i489, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32849
  store <8 x float> %1393, ptr %_114.i630, align 32, !dbg !32850
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32851
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32852
  store <8 x float> %1401, ptr %_115.i631, align 32, !dbg !32853
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32854
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32855
  store <8 x float> %1409, ptr %_119.i632, align 32, !dbg !32856
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32857
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32858
  store <8 x float> %1417, ptr %_120.i633, align 32, !dbg !32859
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32860
  store i32 %storemerge.i181320769, ptr %_22.i293.i659, align 4, !dbg !32861
  store <8 x float> %minimum.i273.i35.sroa.0.0, ptr %uniform_left.i490, align 32, !dbg !32863
  store i32 %storemerge.i20799, ptr %_22.i.i709, align 4, !dbg !32864
  store <8 x float> %minimum.i.i67.sroa.0.020813, ptr %uniform_right.i489, align 32, !dbg !32867
  %_5.i1971 = add i64 %base.i1970, 8, !dbg !33001
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1970, i64 noundef %_5.i1971, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i279.i645, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !33002, !noalias !33003
  unreachable, !dbg !33002

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1978: ; preds = %bb19.i1819
  %_30.i1820 = add nuw i64 %iter.sroa.0.0.i181615672, 1, !dbg !33007
  %_15.i1976 = getelementptr inbounds nuw float, ptr %_54.0.i278.i644, i64 %base.i1970, !dbg !33012
  %lanes.i5784.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1976, align 4, !dbg !33014, !alias.scope !33019, !noalias !33023
  %1435 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %suffix.i1800.sroa.0.015671, <8 x float> %lanes.i5784.sroa.0.0.copyload), !dbg !33027
  store <8 x float> %1435, ptr %_15.i1976, align 4, !dbg !33032, !alias.scope !33038, !noalias !33042
  %1436 = icmp eq i64 %end.sroa.0.0.i181715673, 0, !dbg !33046
  %spec.store.select.i1828 = select i1 %1436, i64 %ring.i506, i64 %end.sroa.0.0.i181715673, !dbg !33046
  %1437 = add i64 %spec.store.select.i1828, -1, !dbg !33047
  %exitcond18047.not = icmp eq i64 %_30.i1820, %_18.i290.i656, !dbg !33048
  br i1 %exitcond18047.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1831, label %bb19.i1819, !dbg !33050

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1831: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1978, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1996
  %minimum.i273.i35.sroa.0.1 = phi <8 x float> [ %1432, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1996 ], [ %minimum.i273.i35.sroa.0.0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1978 ], !dbg !32863
  %storemerge.i1813 = phi i32 [ %1433, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1996 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1978 ], !dbg !33051
  %1438 = fmul <8 x float> %minimum.i273.i35.sroa.0.1, splat (float 1.638400e+04), !dbg !33052
  %1439 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %1438), !dbg !33057
  %1440 = fmul <8 x float> %1439, splat (float 0x3F10000000000000), !dbg !33062
  %base.i2069 = shl i64 %_239.i643, 3, !dbg !33067
  %1441 = or disjoint i64 %base.i2069, 7, !dbg !33069
  %or.cond.i2073.not = icmp ult i64 %1441, %_56.1.i295.i661, !dbg !33069
  br i1 %or.cond.i2073.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2077, label %bb4.i2076, !dbg !33069, !prof !2740

bb4.i2076:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1831
  store i32 %storemerge.i1813.lcssa2078220831, ptr %_22.i293.i659, align 4
  store <8 x float> %minimum.i273.i35.sroa.0.0.lcssa2079620850, ptr %uniform_left.i490, align 32
  store i32 %storemerge.i.lcssa2081220869, ptr %_22.i.i709, align 4
  store <8 x float> %minimum.i.i67.sroa.0.0.lcssa2082620888, ptr %uniform_right.i489, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32849
  store <8 x float> %1393, ptr %_114.i630, align 32, !dbg !32850
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32851
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32852
  store <8 x float> %1401, ptr %_115.i631, align 32, !dbg !32853
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32854
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32855
  store <8 x float> %1409, ptr %_119.i632, align 32, !dbg !32856
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32857
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32858
  store <8 x float> %1417, ptr %_120.i633, align 32, !dbg !32859
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32860
  store i32 %storemerge.i1813, ptr %_22.i293.i659, align 4, !dbg !32861
  store <8 x float> %minimum.i273.i35.sroa.0.0, ptr %uniform_left.i490, align 32, !dbg !32863
  store i32 %storemerge.i20799, ptr %_22.i.i709, align 4, !dbg !32864
  store <8 x float> %minimum.i.i67.sroa.0.020813, ptr %uniform_right.i489, align 32, !dbg !32867
  %_5.i2070 = add i64 %base.i2069, 8, !dbg !33073
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i2069, i64 noundef %_5.i2070, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i295.i661, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !33074, !noalias !33075
  unreachable, !dbg !33074

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2077: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1831
  %_15.i2075 = getelementptr inbounds nuw float, ptr %_56.0.i294.i660, i64 %base.i2069, !dbg !33079
  %lanes.i5707.sroa.0.0.copyload = load <8 x float>, ptr %_15.i2075, align 4, !dbg !33081, !alias.scope !33086, !noalias !33090
  %1442 = fadd <8 x float> %1440, %_33.i264.i26.sroa.0.0.copyload20798, !dbg !33094
  %1443 = fsub <8 x float> %1442, %lanes.i5707.sroa.0.0.copyload, !dbg !33099
  store <8 x float> %1443, ptr %1148, align 32, !dbg !33104
  %_8.not.i4.i302.i668 = icmp ugt i64 %_7.i10.i282.i648, %_56.1.i295.i661
  br i1 %_8.not.i4.i302.i668, label %bb4.i7.i318.i738, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i304.i670, !dbg !33105, !prof !165

bb4.i7.i318.i738:                                 ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2077
  store i32 %storemerge.i1813.lcssa2078220831, ptr %_22.i293.i659, align 4
  store <8 x float> %minimum.i273.i35.sroa.0.0.lcssa2079620850, ptr %uniform_left.i490, align 32
  store i32 %storemerge.i.lcssa2081220869, ptr %_22.i.i709, align 4
  store <8 x float> %minimum.i.i67.sroa.0.0.lcssa2082620888, ptr %uniform_right.i489, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32849
  store <8 x float> %1393, ptr %_114.i630, align 32, !dbg !32850
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32851
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32852
  store <8 x float> %1401, ptr %_115.i631, align 32, !dbg !32853
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32854
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32855
  store <8 x float> %1409, ptr %_119.i632, align 32, !dbg !32856
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32857
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32858
  store <8 x float> %1417, ptr %_120.i633, align 32, !dbg !32859
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32860
  store i32 %storemerge.i1813, ptr %_22.i293.i659, align 4, !dbg !32861
  store <8 x float> %minimum.i273.i35.sroa.0.0, ptr %uniform_left.i490, align 32, !dbg !32863
  store i32 %storemerge.i20799, ptr %_22.i.i709, align 4, !dbg !32864
  store <8 x float> %minimum.i.i67.sroa.0.020813, ptr %uniform_right.i489, align 32, !dbg !32867
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i281.i647, i64 noundef %_7.i10.i282.i648, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i295.i661, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !33110, !noalias !33111
  unreachable, !dbg !33110

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i304.i670: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2077
  %_17.i6.i305.i671 = getelementptr inbounds nuw float, ptr %_56.0.i294.i660, i64 %base.i9.i281.i647, !dbg !33115
  store <8 x float> %1440, ptr %_17.i6.i305.i671, align 4, !dbg !33117, !alias.scope !33122, !noalias !33126
  %_41.i257.i19.sroa.0.0.copyload = load <8 x float>, ptr %1150, align 32, !dbg !33130
  %1444 = fdiv <8 x float> %1443, %_37.i261.i23.sroa.0.0.copyload, !dbg !33131
  %1445 = fsub <8 x float> splat (float 1.000000e+00), %1444, !dbg !33136
  %1446 = fsub <8 x float> %1445, %_41.i257.i19.sroa.0.0.copyload, !dbg !33141
  %1447 = fmul <8 x float> %1401, %1446, !dbg !33146
  %1448 = fadd <8 x float> %_41.i257.i19.sroa.0.0.copyload, %1447, !dbg !33151
  %1449 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1445, <8 x float> %1448), !dbg !33155
  %1450 = bitcast <8 x float> %1449 to <8 x i32>, !dbg !33160
  %1451 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1449), !dbg !33166
  %1452 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1451, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !33168
  %1453 = bitcast <8 x float> %1452 to <8 x i32>, !dbg !33174
  %1454 = xor <8 x i32> %1453, splat (i32 -1), !dbg !33180
  %1455 = and <8 x i32> %1454, %1450, !dbg !33182
  store <8 x i32> %1455, ptr %1150, align 32, !dbg !33186
  %base.i2060 = shl i64 %_236.i640, 3, !dbg !33187
  %_5.i2061 = add i64 %base.i2060, 8, !dbg !33189
  %1456 = or disjoint i64 %base.i2060, 7, !dbg !33190
  %or.cond.i2064.not = icmp ult i64 %1456, %_58.1.i307.i673, !dbg !33190
  br i1 %or.cond.i2064.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2068, label %bb4.i2067, !dbg !33190, !prof !2740

bb4.i2067:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i304.i670
  store i32 %storemerge.i1813.lcssa2078220831, ptr %_22.i293.i659, align 4
  store <8 x float> %minimum.i273.i35.sroa.0.0.lcssa2079620850, ptr %uniform_left.i490, align 32
  store i32 %storemerge.i.lcssa2081220869, ptr %_22.i.i709, align 4
  store <8 x float> %minimum.i.i67.sroa.0.0.lcssa2082620888, ptr %uniform_right.i489, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32849
  store <8 x float> %1393, ptr %_114.i630, align 32, !dbg !32850
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32851
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32852
  store <8 x float> %1401, ptr %_115.i631, align 32, !dbg !32853
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32854
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32855
  store <8 x float> %1409, ptr %_119.i632, align 32, !dbg !32856
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32857
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32858
  store <8 x float> %1417, ptr %_120.i633, align 32, !dbg !32859
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32860
  store i32 %storemerge.i1813, ptr %_22.i293.i659, align 4, !dbg !32861
  store <8 x float> %minimum.i273.i35.sroa.0.0, ptr %uniform_left.i490, align 32, !dbg !32863
  store i32 %storemerge.i20799, ptr %_22.i.i709, align 4, !dbg !32864
  store <8 x float> %minimum.i.i67.sroa.0.020813, ptr %uniform_right.i489, align 32, !dbg !32867
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i2060, i64 noundef %_5.i2061, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i307.i673, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !33194, !noalias !33195
  unreachable, !dbg !33194

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2068: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i304.i670
  %1457 = bitcast <8 x i32> %1455 to <8 x float>, !dbg !33199
  %1458 = fsub <8 x float> splat (float 1.000000e+00), %1457, !dbg !33200
  %_15.i2066 = getelementptr inbounds nuw float, ptr %_58.0.i306.i672, i64 %base.i2060, !dbg !33205
  %lanes.i5714.sroa.0.0.copyload = load <8 x float>, ptr %_15.i2066, align 4, !dbg !33207, !alias.scope !33212, !noalias !33216
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_15.i2066, ptr noundef nonnull align 4 dereferenceable(32) %data.i.i.i.i.i.i.i.i, i64 32, i1 false), !dbg !33220
  %1459 = fmul <8 x float> %1458, %lanes.i5714.sroa.0.0.copyload, !dbg !33226
  %1460 = select <8 x i1> %1154, <8 x float> %lanes.i5714.sroa.0.0.copyload, <8 x float> %1459, !dbg !33231
  store <8 x float> %1460, ptr %data.i.i.i.i.i.i.i.i, align 4, !dbg !33236, !alias.scope !33241, !noalias !33245
  %_242.i691 = add i64 %iter.i476.sroa.41.015680, %right_end.sroa.0.0.i, !dbg !33249
  %_244.i693 = add i64 %iter.i476.sroa.41.015680, %right_expiring.sroa.0.0.i, !dbg !33251
  %_8.not.i12.i.i700 = icmp ugt i64 %_7.i10.i282.i648, %_54.1.i.i695
  br i1 %_8.not.i12.i.i700, label %bb4.i15.i.i736, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i702, !dbg !33252, !prof !165

bb4.i15.i.i736:                                   ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2068
  store i32 %storemerge.i1813.lcssa2078220831, ptr %_22.i293.i659, align 4
  store <8 x float> %minimum.i273.i35.sroa.0.0.lcssa2079620850, ptr %uniform_left.i490, align 32
  store i32 %storemerge.i.lcssa2081220869, ptr %_22.i.i709, align 4
  store <8 x float> %minimum.i.i67.sroa.0.0.lcssa2082620888, ptr %uniform_right.i489, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32849
  store <8 x float> %1393, ptr %_114.i630, align 32, !dbg !32850
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32851
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32852
  store <8 x float> %1401, ptr %_115.i631, align 32, !dbg !32853
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32854
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32855
  store <8 x float> %1409, ptr %_119.i632, align 32, !dbg !32856
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32857
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32858
  store <8 x float> %1417, ptr %_120.i633, align 32, !dbg !32859
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32860
  store i32 %storemerge.i1813, ptr %_22.i293.i659, align 4, !dbg !32861
  store <8 x float> %minimum.i273.i35.sroa.0.0, ptr %uniform_left.i490, align 32, !dbg !32863
  store i32 %storemerge.i20799, ptr %_22.i.i709, align 4, !dbg !32864
  store <8 x float> %minimum.i.i67.sroa.0.020813, ptr %uniform_right.i489, align 32, !dbg !32867
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i281.i647, i64 noundef %_7.i10.i282.i648, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i695, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !33257, !noalias !33258
  unreachable, !dbg !33257

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i702: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2068
  %1461 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1420, <8 x float> %1409, i8 30), !dbg !33272
  %1462 = bitcast <8 x float> %1461 to <8 x i32>, !dbg !33278
  %1463 = icmp slt <8 x i32> %1462, zeroinitializer, !dbg !33282
  %1464 = fdiv <8 x float> %1409, %1420, !dbg !33284
  %1465 = select <8 x i1> %1463, <8 x float> %1464, <8 x float> splat (float 1.000000e+00), !dbg !33282
  %_17.i14.i.i703 = getelementptr inbounds nuw float, ptr %_54.0.i.i694, i64 %base.i9.i281.i647, !dbg !33289
  store <8 x float> %1465, ptr %_17.i14.i.i703, align 4, !dbg !33291, !alias.scope !33296, !noalias !33300
  %base.i2033 = shl i64 %_242.i691, 3, !dbg !33304
  %1466 = or disjoint i64 %base.i2033, 7, !dbg !33306
  %or.cond.i2037.not = icmp ult i64 %1466, %_54.1.i.i695, !dbg !33306
  br i1 %or.cond.i2037.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2041, label %bb4.i2040, !dbg !33306, !prof !2740

bb4.i2040:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i702
  store i32 %storemerge.i1813.lcssa2078220831, ptr %_22.i293.i659, align 4
  store <8 x float> %minimum.i273.i35.sroa.0.0.lcssa2079620850, ptr %uniform_left.i490, align 32
  store i32 %storemerge.i.lcssa2081220869, ptr %_22.i.i709, align 4
  store <8 x float> %minimum.i.i67.sroa.0.0.lcssa2082620888, ptr %uniform_right.i489, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32849
  store <8 x float> %1393, ptr %_114.i630, align 32, !dbg !32850
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32851
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32852
  store <8 x float> %1401, ptr %_115.i631, align 32, !dbg !32853
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32854
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32855
  store <8 x float> %1409, ptr %_119.i632, align 32, !dbg !32856
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32857
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32858
  store <8 x float> %1417, ptr %_120.i633, align 32, !dbg !32859
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32860
  store i32 %storemerge.i1813, ptr %_22.i293.i659, align 4, !dbg !32861
  store <8 x float> %minimum.i273.i35.sroa.0.0, ptr %uniform_left.i490, align 32, !dbg !32863
  store i32 %storemerge.i20799, ptr %_22.i.i709, align 4, !dbg !32864
  store <8 x float> %minimum.i.i67.sroa.0.020813, ptr %uniform_right.i489, align 32, !dbg !32867
  %_5.i2034 = add i64 %base.i2033, 8, !dbg !33310
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i2033, i64 noundef %_5.i2034, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i695, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !33311, !noalias !33312
  unreachable, !dbg !33311

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2041: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i702
  %_15.i2039 = getelementptr inbounds nuw float, ptr %_54.0.i.i694, i64 %base.i2033, !dbg !33320
  %lanes.i5735.sroa.0.0.copyload = load <8 x float>, ptr %_15.i2039, align 4, !dbg !33322, !alias.scope !33327, !noalias !33331
  %position.i = zext i32 %storemerge.i20799 to i64, !dbg !33335
  %1467 = icmp eq i32 %storemerge.i20799, 0, !dbg !33336
  br i1 %1467, label %bb5.i1787, label %bb3.i1786, !dbg !33336

bb3.i1786:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2041
  %1468 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %minimum.i.i67.sroa.0.020813, <8 x float> %lanes.i5735.sroa.0.0.copyload), !dbg !33337
  br label %bb5.i1787, !dbg !33342

bb5.i1787:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2041, %bb3.i1786
  %minimum.i.i67.sroa.0.0 = phi <8 x float> [ %1468, %bb3.i1786 ], [ %lanes.i5735.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2041 ], !dbg !32867
  %_15.i = add nuw nsw i64 %position.i, 1, !dbg !33343
  %complete.i = icmp eq i64 %_15.i, %_18.i.i706, !dbg !33343
  br i1 %complete.i, label %bb19.i1792, label %bb7.i1788, !dbg !33344

bb7.i1788:                                        ; preds = %bb5.i1787
  %base.i2024 = shl i64 %_238.i642, 3, !dbg !33345
  %1469 = or disjoint i64 %base.i2024, 7, !dbg !33347
  %or.cond.i2028.not = icmp ult i64 %1469, %_54.1.i.i695, !dbg !33347
  br i1 %or.cond.i2028.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2032, label %bb4.i2031, !dbg !33347, !prof !2740

bb4.i2031:                                        ; preds = %bb7.i1788
  store i32 %storemerge.i1813.lcssa2078220831, ptr %_22.i293.i659, align 4
  store <8 x float> %minimum.i273.i35.sroa.0.0.lcssa2079620850, ptr %uniform_left.i490, align 32
  store i32 %storemerge.i.lcssa2081220869, ptr %_22.i.i709, align 4
  store <8 x float> %minimum.i.i67.sroa.0.0.lcssa2082620888, ptr %uniform_right.i489, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32849
  store <8 x float> %1393, ptr %_114.i630, align 32, !dbg !32850
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32851
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32852
  store <8 x float> %1401, ptr %_115.i631, align 32, !dbg !32853
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32854
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32855
  store <8 x float> %1409, ptr %_119.i632, align 32, !dbg !32856
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32857
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32858
  store <8 x float> %1417, ptr %_120.i633, align 32, !dbg !32859
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32860
  store i32 %storemerge.i1813, ptr %_22.i293.i659, align 4, !dbg !32861
  store <8 x float> %minimum.i273.i35.sroa.0.0, ptr %uniform_left.i490, align 32, !dbg !32863
  store i32 %storemerge.i20799, ptr %_22.i.i709, align 4, !dbg !32864
  store <8 x float> %minimum.i.i67.sroa.0.0, ptr %uniform_right.i489, align 32, !dbg !32867
  %_5.i2025 = add i64 %base.i2024, 8, !dbg !33351
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i2024, i64 noundef %_5.i2025, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i695, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !33352, !noalias !33353
  unreachable, !dbg !33352

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2032: ; preds = %bb7.i1788
  %_15.i2030 = getelementptr inbounds nuw float, ptr %_54.0.i.i694, i64 %base.i2024, !dbg !33357
  %lanes.i5742.sroa.0.0.copyload = load <8 x float>, ptr %_15.i2030, align 4, !dbg !33359, !alias.scope !33364, !noalias !33368
  %1470 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %lanes.i5742.sroa.0.0.copyload, <8 x float> %minimum.i.i67.sroa.0.0), !dbg !33372
  %1471 = trunc i64 %_15.i to i32, !dbg !33377
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, !dbg !33378

bb19.i1792:                                       ; preds = %bb5.i1787, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2014
  %end.sroa.0.0.i15677 = phi i64 [ %1475, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2014 ], [ %_242.i691, %bb5.i1787 ]
  %iter.sroa.0.0.i179115676 = phi i64 [ %_30.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2014 ], [ 0, %bb5.i1787 ]
  %suffix.i.sroa.0.015675 = phi <8 x float> [ %1473, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2014 ], [ %lanes.i5735.sroa.0.0.copyload, %bb5.i1787 ]
  %base.i2006 = shl i64 %end.sroa.0.0.i15677, 3, !dbg !33379
  %1472 = or disjoint i64 %base.i2006, 7, !dbg !33381
  %or.cond.i2010.not = icmp ult i64 %1472, %_54.1.i.i695, !dbg !33381
  br i1 %or.cond.i2010.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2014, label %bb4.i2013, !dbg !33381, !prof !2740

bb4.i2013:                                        ; preds = %bb19.i1792
  store i32 %storemerge.i1813.lcssa2078220831, ptr %_22.i293.i659, align 4
  store <8 x float> %minimum.i273.i35.sroa.0.0.lcssa2079620850, ptr %uniform_left.i490, align 32
  store i32 %storemerge.i.lcssa2081220869, ptr %_22.i.i709, align 4
  store <8 x float> %minimum.i.i67.sroa.0.0.lcssa2082620888, ptr %uniform_right.i489, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32849
  store <8 x float> %1393, ptr %_114.i630, align 32, !dbg !32850
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32851
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32852
  store <8 x float> %1401, ptr %_115.i631, align 32, !dbg !32853
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32854
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32855
  store <8 x float> %1409, ptr %_119.i632, align 32, !dbg !32856
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32857
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32858
  store <8 x float> %1417, ptr %_120.i633, align 32, !dbg !32859
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32860
  store i32 %storemerge.i1813, ptr %_22.i293.i659, align 4, !dbg !32861
  store <8 x float> %minimum.i273.i35.sroa.0.0, ptr %uniform_left.i490, align 32, !dbg !32863
  store i32 %storemerge.i20799, ptr %_22.i.i709, align 4, !dbg !32864
  store <8 x float> %minimum.i.i67.sroa.0.0, ptr %uniform_right.i489, align 32, !dbg !32867
  %_5.i2007 = add i64 %base.i2006, 8, !dbg !33385
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i2006, i64 noundef %_5.i2007, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i695, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !33386, !noalias !33387
  unreachable, !dbg !33386

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2014: ; preds = %bb19.i1792
  %_30.i = add nuw i64 %iter.sroa.0.0.i179115676, 1, !dbg !33391
  %_15.i2012 = getelementptr inbounds nuw float, ptr %_54.0.i.i694, i64 %base.i2006, !dbg !33396
  %lanes.i5756.sroa.0.0.copyload = load <8 x float>, ptr %_15.i2012, align 4, !dbg !33398, !alias.scope !33403, !noalias !33407
  %1473 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %suffix.i.sroa.0.015675, <8 x float> %lanes.i5756.sroa.0.0.copyload), !dbg !33411
  store <8 x float> %1473, ptr %_15.i2012, align 4, !dbg !33416, !alias.scope !33422, !noalias !33426
  %1474 = icmp eq i64 %end.sroa.0.0.i15677, 0, !dbg !33430
  %spec.store.select.i1795 = select i1 %1474, i64 %ring.i506, i64 %end.sroa.0.0.i15677, !dbg !33430
  %1475 = add i64 %spec.store.select.i1795, -1, !dbg !33431
  %exitcond18053.not = icmp eq i64 %_30.i, %_18.i.i706, !dbg !33432
  br i1 %exitcond18053.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, label %bb19.i1792, !dbg !33434

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2014, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2032
  %minimum.i.i67.sroa.0.1 = phi <8 x float> [ %1470, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2032 ], [ %minimum.i.i67.sroa.0.0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2014 ], !dbg !32867
  %storemerge.i = phi i32 [ %1471, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2032 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2014 ], !dbg !33435
  %1476 = fmul <8 x float> %minimum.i.i67.sroa.0.1, splat (float 1.638400e+04), !dbg !33436
  %1477 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %1476), !dbg !33441
  %1478 = fmul <8 x float> %1477, splat (float 0x3F10000000000000), !dbg !33446
  %base.i2051 = shl i64 %_244.i693, 3, !dbg !33451
  %1479 = or disjoint i64 %base.i2051, 7, !dbg !33453
  %or.cond.i2055.not = icmp ult i64 %1479, %_56.1.i.i711, !dbg !33453
  br i1 %or.cond.i2055.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2059, label %bb4.i2058, !dbg !33453, !prof !2740

bb4.i2058:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit
  store i32 %storemerge.i1813.lcssa2078220831, ptr %_22.i293.i659, align 4
  store <8 x float> %minimum.i273.i35.sroa.0.0.lcssa2079620850, ptr %uniform_left.i490, align 32
  store i32 %storemerge.i.lcssa2081220869, ptr %_22.i.i709, align 4
  store <8 x float> %minimum.i.i67.sroa.0.0.lcssa2082620888, ptr %uniform_right.i489, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32849
  store <8 x float> %1393, ptr %_114.i630, align 32, !dbg !32850
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32851
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32852
  store <8 x float> %1401, ptr %_115.i631, align 32, !dbg !32853
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32854
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32855
  store <8 x float> %1409, ptr %_119.i632, align 32, !dbg !32856
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32857
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32858
  store <8 x float> %1417, ptr %_120.i633, align 32, !dbg !32859
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32860
  store i32 %storemerge.i1813, ptr %_22.i293.i659, align 4, !dbg !32861
  store <8 x float> %minimum.i273.i35.sroa.0.0, ptr %uniform_left.i490, align 32, !dbg !32863
  store i32 %storemerge.i, ptr %_22.i.i709, align 4, !dbg !32864
  store <8 x float> %minimum.i.i67.sroa.0.0, ptr %uniform_right.i489, align 32, !dbg !32867
  %_5.i2052 = add i64 %base.i2051, 8, !dbg !33457
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i2051, i64 noundef %_5.i2052, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i711, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !33458, !noalias !33459
  unreachable, !dbg !33458

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2059: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit
  %_15.i2057 = getelementptr inbounds nuw float, ptr %_56.0.i.i710, i64 %base.i2051, !dbg !33463
  %lanes.i5721.sroa.0.0.copyload = load <8 x float>, ptr %_15.i2057, align 4, !dbg !33465, !alias.scope !33470, !noalias !33474
  %1480 = fadd <8 x float> %1478, %_33.i.i58.sroa.0.0.copyload20828, !dbg !33478
  %1481 = fsub <8 x float> %1480, %lanes.i5721.sroa.0.0.copyload, !dbg !33483
  store <8 x float> %1481, ptr %1159, align 32, !dbg !33488
  %_8.not.i4.i.i718 = icmp ugt i64 %_7.i10.i282.i648, %_56.1.i.i711
  br i1 %_8.not.i4.i.i718, label %bb4.i7.i.i735, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i720, !dbg !33489, !prof !165

bb4.i7.i.i735:                                    ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2059
  store i32 %storemerge.i1813.lcssa2078220831, ptr %_22.i293.i659, align 4
  store <8 x float> %minimum.i273.i35.sroa.0.0.lcssa2079620850, ptr %uniform_left.i490, align 32
  store i32 %storemerge.i.lcssa2081220869, ptr %_22.i.i709, align 4
  store <8 x float> %minimum.i.i67.sroa.0.0.lcssa2082620888, ptr %uniform_right.i489, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32849
  store <8 x float> %1393, ptr %_114.i630, align 32, !dbg !32850
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32851
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32852
  store <8 x float> %1401, ptr %_115.i631, align 32, !dbg !32853
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32854
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32855
  store <8 x float> %1409, ptr %_119.i632, align 32, !dbg !32856
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32857
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32858
  store <8 x float> %1417, ptr %_120.i633, align 32, !dbg !32859
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32860
  store i32 %storemerge.i1813, ptr %_22.i293.i659, align 4, !dbg !32861
  store <8 x float> %minimum.i273.i35.sroa.0.0, ptr %uniform_left.i490, align 32, !dbg !32863
  store i32 %storemerge.i, ptr %_22.i.i709, align 4, !dbg !32864
  store <8 x float> %minimum.i.i67.sroa.0.0, ptr %uniform_right.i489, align 32, !dbg !32867
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i281.i647, i64 noundef %_7.i10.i282.i648, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i711, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !33494, !noalias !33495
  unreachable, !dbg !33494

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i720: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2059
  %_17.i6.i.i721 = getelementptr inbounds nuw float, ptr %_56.0.i.i710, i64 %base.i9.i281.i647, !dbg !33499
  store <8 x float> %1478, ptr %_17.i6.i.i721, align 4, !dbg !33501, !alias.scope !33506, !noalias !33510
  %_41.i.i51.sroa.0.0.copyload = load <8 x float>, ptr %1161, align 32, !dbg !33514
  %1482 = fdiv <8 x float> %1481, %_37.i.i55.sroa.0.0.copyload, !dbg !33515
  %1483 = fsub <8 x float> splat (float 1.000000e+00), %1482, !dbg !33520
  %1484 = fsub <8 x float> %1483, %_41.i.i51.sroa.0.0.copyload, !dbg !33525
  %1485 = fmul <8 x float> %1417, %1484, !dbg !33530
  %1486 = fadd <8 x float> %_41.i.i51.sroa.0.0.copyload, %1485, !dbg !33535
  %1487 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1483, <8 x float> %1486), !dbg !33539
  %1488 = bitcast <8 x float> %1487 to <8 x i32>, !dbg !33544
  %1489 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1487), !dbg !33550
  %1490 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1489, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !33552
  %1491 = bitcast <8 x float> %1490 to <8 x i32>, !dbg !33558
  %1492 = xor <8 x i32> %1491, splat (i32 -1), !dbg !33564
  %1493 = and <8 x i32> %1492, %1488, !dbg !33566
  store <8 x i32> %1493, ptr %1161, align 32, !dbg !33570
  %_6.not.i2045 = icmp ugt i64 %_5.i2061, %_58.1.i.i723
  br i1 %_6.not.i2045, label %bb4.i2049, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2050, !dbg !33571, !prof !165

bb4.i2049:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i720
  store i32 %storemerge.i1813.lcssa2078220831, ptr %_22.i293.i659, align 4
  store <8 x float> %minimum.i273.i35.sroa.0.0.lcssa2079620850, ptr %uniform_left.i490, align 32
  store i32 %storemerge.i.lcssa2081220869, ptr %_22.i.i709, align 4
  store <8 x float> %minimum.i.i67.sroa.0.0.lcssa2082620888, ptr %uniform_right.i489, align 32
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32849
  store <8 x float> %1393, ptr %_114.i630, align 32, !dbg !32850
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32851
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32852
  store <8 x float> %1401, ptr %_115.i631, align 32, !dbg !32853
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32854
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32855
  store <8 x float> %1409, ptr %_119.i632, align 32, !dbg !32856
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32857
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32858
  store <8 x float> %1417, ptr %_120.i633, align 32, !dbg !32859
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32860
  store i32 %storemerge.i1813, ptr %_22.i293.i659, align 4, !dbg !32861
  store <8 x float> %minimum.i273.i35.sroa.0.0, ptr %uniform_left.i490, align 32, !dbg !32863
  store i32 %storemerge.i, ptr %_22.i.i709, align 4, !dbg !32864
  store <8 x float> %minimum.i.i67.sroa.0.0, ptr %uniform_right.i489, align 32, !dbg !32867
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i2060, i64 noundef %_5.i2061, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i.i723, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !33576, !noalias !33577
  unreachable, !dbg !33576

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2050: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i720
  %1494 = bitcast <8 x i32> %1493 to <8 x float>, !dbg !33581
  %1495 = fsub <8 x float> splat (float 1.000000e+00), %1494, !dbg !33582
  %_15.i2048 = getelementptr inbounds nuw float, ptr %_58.0.i.i722, i64 %base.i2060, !dbg !33587
  %lanes.i5728.sroa.0.0.copyload = load <8 x float>, ptr %_15.i2048, align 4, !dbg !33589, !alias.scope !33594, !noalias !33598
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_15.i2048, ptr noundef nonnull align 4 dereferenceable(32) %data.i5.i.i.i.i.i.i.i, i64 32, i1 false), !dbg !33602
  %1496 = fmul <8 x float> %1495, %lanes.i5728.sroa.0.0.copyload, !dbg !33608
  %1497 = select <8 x i1> %1154, <8 x float> %lanes.i5728.sroa.0.0.copyload, <8 x float> %1496, !dbg !33613
  store <8 x float> %1497, ptr %data.i5.i.i.i.i.i.i.i, align 4, !dbg !33618, !alias.scope !33623, !noalias !33627
  %exitcond18066.not = icmp eq i64 %_9.0.i7706, %1384, !dbg !32622
  br i1 %exitcond18066.not, label %bb32.i740.loopexit, label %bb31.i621, !dbg !32622

bb32.i740.loopexit:                               ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2050
  store <8 x float> %1388, ptr %1130, align 32, !dbg !32849
  store <8 x float> %1393, ptr %_114.i630, align 32, !dbg !32850
  store <8 x float> %1394, ptr %1131, align 32, !dbg !32851
  store <8 x float> %1396, ptr %1133, align 32, !dbg !32852
  store <8 x float> %1401, ptr %_115.i631, align 32, !dbg !32853
  store <8 x float> %1402, ptr %1134, align 32, !dbg !32854
  store <8 x float> %1404, ptr %1136, align 32, !dbg !32855
  store <8 x float> %1409, ptr %_119.i632, align 32, !dbg !32856
  store <8 x float> %1410, ptr %1137, align 32, !dbg !32857
  store <8 x float> %1412, ptr %1139, align 32, !dbg !32858
  store <8 x float> %1417, ptr %_120.i633, align 32, !dbg !32859
  store <8 x float> %1418, ptr %1140, align 32, !dbg !32860
  br label %bb32.i740, !dbg !33631

bb32.i740:                                        ; preds = %bb32.i740.loopexit, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit
  %minimum.i.i67.sroa.0.0.lcssa2082620887 = phi <8 x float> [ %minimum.i.i67.sroa.0.0, %bb32.i740.loopexit ], [ %minimum.i.i67.sroa.0.0.lcssa2082620888, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %storemerge.i.lcssa2081220868 = phi i32 [ %storemerge.i, %bb32.i740.loopexit ], [ %storemerge.i.lcssa2081220869, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %minimum.i273.i35.sroa.0.0.lcssa2079620849 = phi <8 x float> [ %minimum.i273.i35.sroa.0.0, %bb32.i740.loopexit ], [ %minimum.i273.i35.sroa.0.0.lcssa2079620850, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %storemerge.i1813.lcssa2078220830 = phi i32 [ %storemerge.i1813, %bb32.i740.loopexit ], [ %storemerge.i1813.lcssa2078220831, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit ]
  %_143.i741 = add i64 %..i7634, %ring_cursor.sroa.0.1.i58815682, !dbg !33631
  %_234.not.i742 = icmp ult i64 %_143.i741, %ring.i506, !dbg !33632
  %1498 = select i1 %_234.not.i742, i64 0, i64 %ring.i506, !dbg !33632
  %ring_cursor.sroa.0.2.i743 = sub nuw i64 %_143.i741, %1498, !dbg !33632
  %_145.i744 = add i64 %..i7634, %main_cursor.sroa.0.1.i58915683, !dbg !33635
  %_245.not.i745 = icmp ult i64 %_145.i744, %main.i507, !dbg !33636
  %1499 = select i1 %_245.not.i745, i64 0, i64 %main.i507, !dbg !33636
  %main_cursor.sroa.0.2.i746 = sub nuw i64 %_145.i744, %1499, !dbg !33636
  %_63.i591 = icmp ult i64 %_85.i608, %..i7507, !dbg !32519
  br i1 %_63.i591, label %bb20.i592, label %bb15.i515.loopexit.loopexit, !dbg !32519

_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit: ; preds = %bb15.i515.loopexit
  %1500 = trunc i64 %main_cursor.sroa.0.1.i589.lcssa to i32, !dbg !33638
  %1501 = trunc i64 %ring_cursor.sroa.0.1.i588.lcssa to i32, !dbg !33640
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, !dbg !33641

_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit, %bb7.i
  %ring_cursor.sroa.0.0.i516.lcssa = phi i32 [ %_36.i509, %bb7.i ], [ %1501, %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit ], !dbg !31319
  %main_cursor.sroa.0.0.i517.lcssa = phi i32 [ %_35.i508, %bb7.i ], [ %1500, %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit ], !dbg !31316
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %left_prefix.i455, ptr noundef nonnull align 32 dereferenceable(32) %uniform_left.i490, i64 32, i1 false), !dbg !33641
  %1502 = getelementptr inbounds nuw i8, ptr %uniform_left.i490, i64 104, !dbg !33642
  %left_phase.i751 = load i32, ptr %1502, align 8, !dbg !33642, !noalias !31323, !noundef !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %right_prefix.i454, ptr noundef nonnull align 32 dereferenceable(32) %uniform_right.i489, i64 32, i1 false), !dbg !33643
  %1503 = getelementptr inbounds nuw i8, ptr %uniform_right.i489, i64 104, !dbg !33644
  %right_phase.i752 = load i32, ptr %1503, align 8, !dbg !33644, !noalias !31323, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_right.i489), !dbg !33645, !noalias !31323
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i490), !dbg !33646, !noalias !31323
  %1504 = getelementptr inbounds nuw i8, ptr %self, i64 1736, !dbg !33647
  %_260.1.i754 = load i64, ptr %1504, align 8, !dbg !33647, !alias.scope !31290, !noalias !33648, !noundef !12
  %_8.i6719 = icmp samesign ugt i64 %_260.1.i754, 7, !dbg !33649
  br i1 %_8.i6719, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6722, label %bb2.i6720, !dbg !33649, !prof !1421

bb2.i6720:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_260.1.i754, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !33654, !noalias !33655
  unreachable, !dbg !33654

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6722: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit
  %1505 = getelementptr inbounds nuw i8, ptr %self, i64 1728, !dbg !33647
  %_260.0.i753 = load ptr, ptr %1505, align 8, !dbg !33647, !alias.scope !31290, !noalias !33648, !nonnull !12, !noundef !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_260.0.i753, ptr noundef nonnull align 32 dereferenceable(32) %left_prefix.i455, i64 32, i1 false), !dbg !33659
  %_261.0.i755 = load ptr, ptr %68, align 8, !dbg !33663, !alias.scope !31290, !noalias !33648, !nonnull !12, !noundef !12
  %_261.1.i756 = load i64, ptr %69, align 8, !dbg !33663, !alias.scope !31290, !noalias !33648, !noundef !12
  %1506 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i751), !dbg !33664
  br i1 %1506, label %bb2.i7739, label %bb6.i7734, !dbg !33664

bb6.i7734:                                        ; preds = %bb2.i7739, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6722
  %end_or_len.idx.i = shl nuw nsw i64 %_261.1.i756, 2, !dbg !33668
  %end_or_len.i = getelementptr inbounds nuw i8, ptr %_261.0.i755, i64 %end_or_len.idx.i, !dbg !33668
  %_293.i = icmp eq i64 %_261.1.i756, 0, !dbg !33672
  br i1 %_293.i, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i7735, !dbg !33675

bb2.i7739:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6722
  %bytes1.sroa.0.0.zext.i = and i32 %left_phase.i751, 255, !dbg !33676
  %bytes1.sroa.0.0.isplat.i = mul nuw i32 %bytes1.sroa.0.0.zext.i, 16843009, !dbg !33676
  %_5.i7740 = icmp eq i32 %left_phase.i751, %bytes1.sroa.0.0.isplat.i, !dbg !33677
  br i1 %_5.i7740, label %bb3.i7741, label %bb6.i7734, !dbg !33677

bb3.i7741:                                        ; preds = %bb2.i7739
  %bytes.sroa.0.0.extract.trunc.i = trunc i32 %left_phase.i751 to i8, !dbg !33678
  %1507 = shl nuw nsw i64 %_261.1.i756, 2, !dbg !33680
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_261.0.i755, i8 %bytes.sroa.0.0.extract.trunc.i, i64 %1507, i1 false), !dbg !33680, !alias.scope !33681, !noalias !31294
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, !dbg !33684

bb10.i7735:                                       ; preds = %bb6.i7734, %bb10.i7735
  %iter.sroa.0.04.i = phi ptr [ %_38.i7736, %bb10.i7735 ], [ %_261.0.i755, %bb6.i7734 ]
  %_38.i7736 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i, i64 4, !dbg !33685
  store i32 %left_phase.i751, ptr %iter.sroa.0.04.i, align 4, !dbg !33687, !alias.scope !33681, !noalias !31294
  %_29.i7737 = icmp eq ptr %_38.i7736, %end_or_len.i, !dbg !33672
  br i1 %_29.i7737, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i7735, !dbg !33675

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit: ; preds = %bb10.i7735, %bb6.i7734, %bb3.i7741
  %1508 = getelementptr inbounds nuw i8, ptr %self, i64 1936, !dbg !33688
  %_262.1.i758 = load i64, ptr %1508, align 8, !dbg !33688, !alias.scope !31292, !noalias !33689, !noundef !12
  %_8.i6714 = icmp samesign ugt i64 %_262.1.i758, 7, !dbg !33690
  br i1 %_8.i6714, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6717, label %bb2.i6715, !dbg !33690, !prof !1421

bb2.i6715:                                        ; preds = %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_262.1.i758, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !33695, !noalias !33696
  unreachable, !dbg !33695

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6717: ; preds = %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit
  %1509 = getelementptr inbounds nuw i8, ptr %self, i64 1928, !dbg !33688
  %_262.0.i757 = load ptr, ptr %1509, align 8, !dbg !33688, !alias.scope !31292, !noalias !33689, !nonnull !12, !noundef !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_262.0.i757, ptr noundef nonnull align 32 dereferenceable(32) %right_prefix.i454, i64 32, i1 false), !dbg !33700
  %_263.0.i759 = load ptr, ptr %77, align 8, !dbg !33704, !alias.scope !31292, !noalias !33689, !nonnull !12, !noundef !12
  %_263.1.i760 = load i64, ptr %78, align 8, !dbg !33704, !alias.scope !31292, !noalias !33689, !noundef !12
  %1510 = tail call i1 @llvm.is.constant.i32(i32 %right_phase.i752), !dbg !33705
  br i1 %1510, label %bb2.i7752, label %bb6.i7743, !dbg !33705

bb6.i7743:                                        ; preds = %bb2.i7752, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6717
  %end_or_len.idx.i7744 = shl nuw nsw i64 %_263.1.i760, 2, !dbg !33708
  %end_or_len.i7745 = getelementptr inbounds nuw i8, ptr %_263.0.i759, i64 %end_or_len.idx.i7744, !dbg !33708
  %_293.i7746 = icmp eq i64 %_263.1.i760, 0, !dbg !33712
  br i1 %_293.i7746, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7758, label %bb10.i7747, !dbg !33715

bb2.i7752:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6717
  %bytes1.sroa.0.0.zext.i7753 = and i32 %right_phase.i752, 255, !dbg !33716
  %bytes1.sroa.0.0.isplat.i7754 = mul nuw i32 %bytes1.sroa.0.0.zext.i7753, 16843009, !dbg !33716
  %_5.i7755 = icmp eq i32 %right_phase.i752, %bytes1.sroa.0.0.isplat.i7754, !dbg !33717
  br i1 %_5.i7755, label %bb3.i7756, label %bb6.i7743, !dbg !33717

bb3.i7756:                                        ; preds = %bb2.i7752
  %bytes.sroa.0.0.extract.trunc.i7757 = trunc i32 %right_phase.i752 to i8, !dbg !33718
  %1511 = shl nuw nsw i64 %_263.1.i760, 2, !dbg !33720
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_263.0.i759, i8 %bytes.sroa.0.0.extract.trunc.i7757, i64 %1511, i1 false), !dbg !33720, !alias.scope !33721, !noalias !31294
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7758, !dbg !33724

bb10.i7747:                                       ; preds = %bb6.i7743, %bb10.i7747
  %iter.sroa.0.04.i7748 = phi ptr [ %_38.i7749, %bb10.i7747 ], [ %_263.0.i759, %bb6.i7743 ]
  %_38.i7749 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i7748, i64 4, !dbg !33725
  store i32 %right_phase.i752, ptr %iter.sroa.0.04.i7748, align 4, !dbg !33727, !alias.scope !33721, !noalias !31294
  %_29.i7750 = icmp eq ptr %_38.i7749, %end_or_len.i7745, !dbg !33712
  br i1 %_29.i7750, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7758, label %bb10.i7747, !dbg !33715

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7758: ; preds = %bb10.i7747, %bb6.i7743, %bb3.i7756
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_left.i499, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33) #31, !dbg !33728
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_right.i498, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34) #31, !dbg !33729
  store i32 %main_cursor.sroa.0.0.i517.lcssa, ptr %_35, align 4, !dbg !33638, !alias.scope !31294, !noalias !31318
  store i32 %ring_cursor.sroa.0.0.i516.lcssa, ptr %87, align 4, !dbg !33640, !alias.scope !31294, !noalias !31318
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i491), !dbg !33730, !noalias !31323
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i492), !dbg !33731, !noalias !31323
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, !dbg !31287

bb6.i:                                            ; preds = %bb5.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !33732), !dbg !33735
  tail call void @llvm.experimental.noalias.scope.decl(metadata !33736), !dbg !33735
  tail call void @llvm.experimental.noalias.scope.decl(metadata !33738), !dbg !33735
  tail call void @llvm.experimental.noalias.scope.decl(metadata !33740), !dbg !33735
  tail call void @llvm.experimental.noalias.scope.decl(metadata !33742), !dbg !33735
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_left.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #31, !dbg !33744
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_right.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #31, !dbg !33748
  %1512 = load i8, ptr %83, align 32, !dbg !33750, !range !17, !alias.scope !33732, !noalias !33754, !noundef !12
  %1513 = load i8, ptr %84, align 1, !dbg !33757, !range !17, !alias.scope !33732, !noalias !33754, !noundef !12
  %ring.i = load i64, ptr %85, align 8, !dbg !33759, !alias.scope !33736, !noalias !33761, !noundef !12
  %main.i = load i64, ptr %86, align 8, !dbg !33762, !alias.scope !33736, !noalias !33761, !noundef !12
  %_35.i = load i32, ptr %_35, align 4, !dbg !33764, !alias.scope !33742, !noalias !33766, !noundef !12
  %_36.i = load i32, ptr %87, align 4, !dbg !33767, !alias.scope !33742, !noalias !33766, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i), !dbg !33769, !noalias !33771
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i, i8 0, i64 1024, i1 false), !noalias !33771
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i), !dbg !33772, !noalias !33771
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i, i8 0, i64 1024, i1 false), !noalias !33771
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i), !dbg !33774, !noalias !33771
; call <true_peak_limiter::UniformHot<wide::f32x8_::f32x8>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_(ptr noalias noundef align 32 captures(none) dereferenceable(128) %uniform_left.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33, i64 %ring.i, i64 %main.i) #31, !dbg !33776
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_right.i), !dbg !33777, !noalias !33771
  %_32.val7020 = load i64, ptr %85, align 8, !dbg !33779, !noundef !12
  %_32.val7021 = load i64, ptr %86, align 8, !dbg !33779, !noundef !12
; call <true_peak_limiter::UniformHot<wide::f32x8_::f32x8>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_(ptr noalias noundef align 32 captures(none) dereferenceable(128) %uniform_right.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34, i64 %_32.val7020, i64 %_32.val7021) #31, !dbg !33779
  %1514 = add nuw nsw i64 %frames, 31, !dbg !33780
  %yield_count.sroa.0.0.i.i7762 = lshr i64 %1514, 5, !dbg !33780
  %_167.not.i16098 = icmp eq i64 %yield_count.sroa.0.0.i.i7762, 0, !dbg !33787
  br i1 %_167.not.i16098, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, label %bb50.i.lr.ph, !dbg !33787

bb50.i.lr.ph:                                     ; preds = %bb6.i
  %1515 = zext i32 %_36.i to i64, !dbg !33767
  %1516 = zext i32 %_35.i to i64, !dbg !33764
  %_32.i = trunc nuw i8 %1513 to i1, !dbg !33757
  %_31.i = trunc nuw i8 %1512 to i1, !dbg !33750
  %1517 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !33796
  %1518 = bitcast <8 x float> %1517 to <8 x i32>, !dbg !33802
  %1519 = xor <8 x i32> %1518, splat (i32 -1), !dbg !33808
  %history.i215.i.sroa.10.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 32
  %history.i215.i.sroa.13.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 64
  %history.i215.i.sroa.16.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 96
  %history.i215.i.sroa.19.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 128
  %history.i215.i.sroa.22.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 160
  %history.i215.i.sroa.25.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 192
  %history.i215.i.sroa.29.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 224
  %history.i215.i.sroa.32.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 256
  %history.i215.i.sroa.35.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 288
  %history.i215.i.sroa.38.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 320
  %history.i215.i.sroa.41.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 352
  %1520 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %1521 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %1522 = getelementptr inbounds nuw i8, ptr %self, i64 96
  %row12.i.i.i222.i = getelementptr inbounds nuw i8, ptr %self, i64 128
  %1523 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %1524 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %1525 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %row13.i.i.i223.i = getelementptr inbounds nuw i8, ptr %self, i64 256
  %1526 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %1527 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %1528 = getelementptr inbounds nuw i8, ptr %self, i64 352
  %row14.i.i.i224.i = getelementptr inbounds nuw i8, ptr %self, i64 384
  %1529 = getelementptr inbounds nuw i8, ptr %self, i64 416
  %1530 = getelementptr inbounds nuw i8, ptr %self, i64 448
  %1531 = getelementptr inbounds nuw i8, ptr %self, i64 480
  %row15.i.i.i225.i = getelementptr inbounds nuw i8, ptr %self, i64 512
  %1532 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %1533 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %1534 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %row16.i.i.i226.i = getelementptr inbounds nuw i8, ptr %self, i64 640
  %1535 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %1536 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %1537 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %row17.i.i.i227.i = getelementptr inbounds nuw i8, ptr %self, i64 768
  %1538 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %1539 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %1540 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %row18.i.i.i228.i = getelementptr inbounds nuw i8, ptr %self, i64 896
  %1541 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %1542 = getelementptr inbounds nuw i8, ptr %self, i64 960
  %1543 = getelementptr inbounds nuw i8, ptr %self, i64 992
  %row19.i.i.i229.i = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %1544 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %1545 = getelementptr inbounds nuw i8, ptr %self, i64 1088
  %1546 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %row20.i.i.i230.i = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %1547 = getelementptr inbounds nuw i8, ptr %self, i64 1184
  %1548 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %1549 = getelementptr inbounds nuw i8, ptr %self, i64 1248
  %row21.i.i.i231.i = getelementptr inbounds nuw i8, ptr %self, i64 1280
  %1550 = getelementptr inbounds nuw i8, ptr %self, i64 1312
  %1551 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %1552 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %row22.i.i.i232.i = getelementptr inbounds nuw i8, ptr %self, i64 1408
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
  %_70.i.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 88
  %_70.i.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 96
  %1557 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 80
  %_71.i.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 88
  %_71.i.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 96
  %_114.i = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 384
  %_115.i = getelementptr inbounds nuw i8, ptr %hot_left.i, i64 512
  %_119.i = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 384
  %_120.i = getelementptr inbounds nuw i8, ptr %hot_right.i, i64 512
  %1558 = select i1 %_31.i, <8 x i32> %1518, <8 x i32> %1519
  %1559 = icmp slt <8 x i32> %1558, zeroinitializer
  %1560 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 32
  %1561 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 40
  %_22.i293.i = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 104
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
  %history.i215.i.sroa.10.0.hot_left.i.sroa_idx.promoted = load <8 x float>, ptr %history.i215.i.sroa.10.0.hot_left.i.sroa_idx, align 32
  %history.i215.i.sroa.13.0.hot_left.i.sroa_idx.promoted = load <8 x float>, ptr %history.i215.i.sroa.13.0.hot_left.i.sroa_idx, align 32
  %history.i215.i.sroa.16.0.hot_left.i.sroa_idx.promoted = load <8 x float>, ptr %history.i215.i.sroa.16.0.hot_left.i.sroa_idx, align 32
  %history.i215.i.sroa.19.0.hot_left.i.sroa_idx.promoted = load <8 x float>, ptr %history.i215.i.sroa.19.0.hot_left.i.sroa_idx, align 32
  %history.i215.i.sroa.22.0.hot_left.i.sroa_idx.promoted = load <8 x float>, ptr %history.i215.i.sroa.22.0.hot_left.i.sroa_idx, align 32
  %history.i215.i.sroa.25.0.hot_left.i.sroa_idx.promoted = load <8 x float>, ptr %history.i215.i.sroa.25.0.hot_left.i.sroa_idx, align 32
  %history.i215.i.sroa.29.0.hot_left.i.sroa_idx.promoted = load <8 x float>, ptr %history.i215.i.sroa.29.0.hot_left.i.sroa_idx, align 32
  %history.i215.i.sroa.32.0.hot_left.i.sroa_idx.promoted = load <8 x float>, ptr %history.i215.i.sroa.32.0.hot_left.i.sroa_idx, align 32
  %history.i215.i.sroa.35.0.hot_left.i.sroa_idx.promoted = load <8 x float>, ptr %history.i215.i.sroa.35.0.hot_left.i.sroa_idx, align 32
  %history.i215.i.sroa.38.0.hot_left.i.sroa_idx.promoted = load <8 x float>, ptr %history.i215.i.sroa.38.0.hot_left.i.sroa_idx, align 32
  %history.i215.i.sroa.41.0.hot_left.i.sroa_idx.promoted = load <8 x float>, ptr %history.i215.i.sroa.41.0.hot_left.i.sroa_idx, align 32
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
  %_22.i293.i.promoted = load i32, ptr %_22.i293.i, align 4
  %.promoted21095 = load <8 x float>, ptr %1564, align 32
  %_22.i.i.promoted = load i32, ptr %_22.i.i, align 4
  %.promoted21118 = load <8 x float>, ptr %1575, align 32
  br label %bb50.i, !dbg !33787

bb19.i.bb15.i.loopexit_crit_edge:                 ; preds = %bb32.i
  store <8 x float> %.lcssa1584415991, ptr %1564, align 32
  store <8 x float> %.lcssa1590116063, ptr %1575, align 32
  br label %bb15.i.loopexit, !dbg !33810

bb15.i.loopexit:                                  ; preds = %bb19.i.bb15.i.loopexit_crit_edge, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i
  %.lcssa1590116063.lcssa21119 = phi <8 x float> [ %.lcssa1590116063, %bb19.i.bb15.i.loopexit_crit_edge ], [ %.lcssa1590116063.lcssa21120, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ]
  %storemerge.i1848.lcssa1587116027.lcssa21098 = phi i32 [ %storemerge.i1848.lcssa1587116027, %bb19.i.bb15.i.loopexit_crit_edge ], [ %storemerge.i1848.lcssa1587116027.lcssa21099, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ]
  %.lcssa1584415991.lcssa21096 = phi <8 x float> [ %.lcssa1584415991, %bb19.i.bb15.i.loopexit_crit_edge ], [ %.lcssa1584415991.lcssa21097, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ]
  %storemerge.i1883.lcssa1581415955.lcssa21075 = phi i32 [ %storemerge.i1883.lcssa1581415955, %bb19.i.bb15.i.loopexit_crit_edge ], [ %storemerge.i1883.lcssa1581415955.lcssa21076, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ]
  %minimum.i.i.sroa.0.015770.lcssa15926.lcssa = phi <8 x float> [ %minimum.i.i.sroa.0.015770.lcssa, %bb19.i.bb15.i.loopexit_crit_edge ], [ %minimum.i.i.sroa.0.015770.lcssa15926.lcssa21056, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ]
  %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa = phi <8 x float> [ %minimum.i273.i.sroa.0.015756.lcssa, %bb19.i.bb15.i.loopexit_crit_edge ], [ %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa21037, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ]
  %ring_cursor.sroa.0.1.i.lcssa = phi i64 [ %ring_cursor.sroa.0.2.i, %bb19.i.bb15.i.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i16099, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ], !dbg !33816
  %main_cursor.sroa.0.1.i.lcssa = phi i64 [ %main_cursor.sroa.0.2.i, %bb19.i.bb15.i.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i16100, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ], !dbg !33817
  %_167.not.i = icmp eq i64 %1581, 0, !dbg !33787
  %indvars.iv.next18070 = add nsw i64 %indvars.iv18069, -32, !dbg !33787
  br i1 %_167.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit, label %bb50.i, !dbg !33787

bb50.i:                                           ; preds = %bb50.i.lr.ph, %bb15.i.loopexit
  %.lcssa1590116063.lcssa21120 = phi <8 x float> [ %.promoted21118, %bb50.i.lr.ph ], [ %.lcssa1590116063.lcssa21119, %bb15.i.loopexit ]
  %storemerge.i1848.lcssa1587116027.lcssa21099 = phi i32 [ %_22.i.i.promoted, %bb50.i.lr.ph ], [ %storemerge.i1848.lcssa1587116027.lcssa21098, %bb15.i.loopexit ]
  %.lcssa1584415991.lcssa21097 = phi <8 x float> [ %.promoted21095, %bb50.i.lr.ph ], [ %.lcssa1584415991.lcssa21096, %bb15.i.loopexit ]
  %storemerge.i1883.lcssa1581415955.lcssa21076 = phi i32 [ %_22.i293.i.promoted, %bb50.i.lr.ph ], [ %storemerge.i1883.lcssa1581415955.lcssa21075, %bb15.i.loopexit ]
  %minimum.i.i.sroa.0.015770.lcssa15926.lcssa21056 = phi <8 x float> [ %uniform_right.i.promoted, %bb50.i.lr.ph ], [ %minimum.i.i.sroa.0.015770.lcssa15926.lcssa, %bb15.i.loopexit ]
  %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa21037 = phi <8 x float> [ %uniform_left.i.promoted, %bb50.i.lr.ph ], [ %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.41.sroa.0.0.lcssa21036 = phi <8 x float> [ %history.i.i.sroa.41.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.41.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.38.sroa.0.0.lcssa21035 = phi <8 x float> [ %history.i.i.sroa.38.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.38.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.35.sroa.0.0.lcssa21034 = phi <8 x float> [ %history.i.i.sroa.35.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.35.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.32.sroa.0.0.lcssa21033 = phi <8 x float> [ %history.i.i.sroa.32.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.32.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.29.sroa.0.0.lcssa21032 = phi <8 x float> [ %history.i.i.sroa.29.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.29.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.25.sroa.0.0.lcssa21031 = phi <8 x float> [ %history.i.i.sroa.25.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.25.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.22.sroa.0.0.lcssa21030 = phi <8 x float> [ %history.i.i.sroa.22.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.22.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.19.sroa.0.0.lcssa21029 = phi <8 x float> [ %history.i.i.sroa.19.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.19.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.16.sroa.0.0.lcssa21028 = phi <8 x float> [ %history.i.i.sroa.16.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.16.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.13.sroa.0.0.lcssa21009 = phi <8 x float> [ %history.i.i.sroa.13.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.13.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.10.sroa.0.0.lcssa20990 = phi <8 x float> [ %history.i.i.sroa.10.0.hot_right.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.10.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.0.0.lcssa20971 = phi <8 x float> [ %hot_right.i.promoted, %bb50.i.lr.ph ], [ %history.i.i.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i215.i.sroa.41.sroa.0.0.lcssa20970 = phi <8 x float> [ %history.i215.i.sroa.41.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i215.i.sroa.41.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i215.i.sroa.38.sroa.0.0.lcssa20969 = phi <8 x float> [ %history.i215.i.sroa.38.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i215.i.sroa.38.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i215.i.sroa.35.sroa.0.0.lcssa20968 = phi <8 x float> [ %history.i215.i.sroa.35.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i215.i.sroa.35.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i215.i.sroa.32.sroa.0.0.lcssa20967 = phi <8 x float> [ %history.i215.i.sroa.32.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i215.i.sroa.32.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i215.i.sroa.29.sroa.0.0.lcssa20966 = phi <8 x float> [ %history.i215.i.sroa.29.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i215.i.sroa.29.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i215.i.sroa.25.sroa.0.0.lcssa20965 = phi <8 x float> [ %history.i215.i.sroa.25.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i215.i.sroa.25.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i215.i.sroa.22.sroa.0.0.lcssa20964 = phi <8 x float> [ %history.i215.i.sroa.22.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i215.i.sroa.22.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i215.i.sroa.19.sroa.0.0.lcssa20963 = phi <8 x float> [ %history.i215.i.sroa.19.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i215.i.sroa.19.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i215.i.sroa.16.sroa.0.0.lcssa20962 = phi <8 x float> [ %history.i215.i.sroa.16.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i215.i.sroa.16.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i215.i.sroa.13.sroa.0.0.lcssa20943 = phi <8 x float> [ %history.i215.i.sroa.13.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i215.i.sroa.13.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i215.i.sroa.10.sroa.0.0.lcssa20924 = phi <8 x float> [ %history.i215.i.sroa.10.0.hot_left.i.sroa_idx.promoted, %bb50.i.lr.ph ], [ %history.i215.i.sroa.10.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i215.i.sroa.0.0.lcssa20905 = phi <8 x float> [ %hot_left.i.promoted, %bb50.i.lr.ph ], [ %history.i215.i.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %indvars.iv18069 = phi i64 [ %frames, %bb50.i.lr.ph ], [ %indvars.iv.next18070, %bb15.i.loopexit ]
  %iter4.sroa.0.0.i16102 = phi i64 [ %yield_count.sroa.0.0.i.i7762, %bb50.i.lr.ph ], [ %1581, %bb15.i.loopexit ]
  %iter3.sroa.0.0.i16101 = phi i64 [ 0, %bb50.i.lr.ph ], [ %1580, %bb15.i.loopexit ]
  %main_cursor.sroa.0.0.i16100 = phi i64 [ %1516, %bb50.i.lr.ph ], [ %main_cursor.sroa.0.1.i.lcssa, %bb15.i.loopexit ]
  %ring_cursor.sroa.0.0.i16099 = phi i64 [ %1515, %bb50.i.lr.ph ], [ %ring_cursor.sroa.0.1.i.lcssa, %bb15.i.loopexit ]
  %umin18097 = call i64 @llvm.umin.i64(i64 %indvars.iv18069, i64 32), !dbg !33818
  %umax18077 = call i64 @llvm.umax.i64(i64 %umin18097, i64 1), !dbg !33818
  %1580 = add nuw nsw i64 %iter3.sroa.0.0.i16101, 32, !dbg !33818
  %1581 = add nsw i64 %iter4.sroa.0.0.i16102, -1, !dbg !33822
  %_46.i = sub nsw i64 %frames, %iter3.sroa.0.0.i16101, !dbg !33823
  %..i7763 = tail call noundef i64 @llvm.umin.i64(i64 %_46.i, i64 32), !dbg !33824
  %active_base.i = shl i64 %iter3.sroa.0.0.i16101, 3, !dbg !33828
  %active_base.i13447 = add nuw i64 %..i7763, %iter3.sroa.0.0.i16101, !dbg !33829
  %_52.i = shl i64 %active_base.i13447, 3, !dbg !33829
  %_179.i = icmp samesign ult i64 %_52.i, %active_base.i, !dbg !33830
  %_173.not.i = icmp ugt i64 %_52.i, %left_io.1
  %or.cond.i = or i1 %_179.i, %_173.not.i, !dbg !33830
  br i1 %or.cond.i, label %bb54.i, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7793, !dbg !33830, !prof !165

bb54.i:                                           ; preds = %bb50.i
  store <8 x float> %history.i215.i.sroa.0.0.lcssa20905, ptr %hot_left.i, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.10.sroa.0.0.lcssa20924, ptr %history.i215.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.13.sroa.0.0.lcssa20943, ptr %history.i215.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i.i.sroa.0.0.lcssa20971, ptr %hot_right.i, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa20990, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa21009, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa21037, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015770.lcssa15926.lcssa21056, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1883.lcssa1581415955.lcssa21076, ptr %_22.i293.i, align 4
  store i32 %storemerge.i1848.lcssa1587116027.lcssa21099, ptr %_22.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %active_base.i, i64 noundef %_52.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e7f134ea71d3d762bf72c0ef5353d5ff) #30, !dbg !33841, !noalias !33742
  unreachable, !dbg !33841

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7793: ; preds = %bb50.i
  %_182.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %active_base.i, !dbg !33842
  %_2.i779615696.not = icmp eq i64 %frames, %iter3.sroa.0.0.i16101, !dbg !33846
  br i1 %_2.i779615696.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit239.i, label %bb6.i218.i.lr.ph, !dbg !33846

bb6.i218.i.lr.ph:                                 ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7793
  %_5.i5557 = load <8 x float>, ptr %self, align 32, !alias.scope !33849, !noalias !33852
  %_14.i.i.i175.i.sroa.0.0.copyload = load <8 x float>, ptr %1520, align 32, !noalias !33864
  %_17.i.i.i172.i.sroa.0.0.copyload = load <8 x float>, ptr %1521, align 32, !noalias !33864
  %_20.i.i.i169.i.sroa.0.0.copyload = load <8 x float>, ptr %1522, align 32, !noalias !33864
  %_25.i.i.i165.i.sroa.0.0.copyload = load <8 x float>, ptr %row12.i.i.i222.i, align 32, !noalias !33864
  %_28.i.i.i162.i.sroa.0.0.copyload = load <8 x float>, ptr %1523, align 32, !noalias !33864
  %_31.i.i.i159.i.sroa.0.0.copyload = load <8 x float>, ptr %1524, align 32, !noalias !33864
  %_34.i.i.i156.i.sroa.0.0.copyload = load <8 x float>, ptr %1525, align 32, !noalias !33864
  %_39.i.i.i152.i.sroa.0.0.copyload = load <8 x float>, ptr %row13.i.i.i223.i, align 32, !noalias !33864
  %_42.i.i.i149.i.sroa.0.0.copyload = load <8 x float>, ptr %1526, align 32, !noalias !33864
  %_45.i.i.i146.i.sroa.0.0.copyload = load <8 x float>, ptr %1527, align 32, !noalias !33864
  %_48.i.i.i143.i.sroa.0.0.copyload = load <8 x float>, ptr %1528, align 32, !noalias !33864
  %_53.i.i.i139.i.sroa.0.0.copyload = load <8 x float>, ptr %row14.i.i.i224.i, align 32, !noalias !33864
  %_56.i.i.i136.i.sroa.0.0.copyload = load <8 x float>, ptr %1529, align 32, !noalias !33864
  %_59.i.i.i133.i.sroa.0.0.copyload = load <8 x float>, ptr %1530, align 32, !noalias !33864
  %_62.i.i.i130.i.sroa.0.0.copyload = load <8 x float>, ptr %1531, align 32, !noalias !33864
  %_67.i.i.i126.i.sroa.0.0.copyload = load <8 x float>, ptr %row15.i.i.i225.i, align 32, !noalias !33864
  %_70.i.i.i123.i.sroa.0.0.copyload = load <8 x float>, ptr %1532, align 32, !noalias !33864
  %_73.i.i.i120.i.sroa.0.0.copyload = load <8 x float>, ptr %1533, align 32, !noalias !33864
  %_76.i.i.i117.i.sroa.0.0.copyload = load <8 x float>, ptr %1534, align 32, !noalias !33864
  %_81.i.i.i113.i.sroa.0.0.copyload = load <8 x float>, ptr %row16.i.i.i226.i, align 32, !noalias !33864
  %_84.i.i.i110.i.sroa.0.0.copyload = load <8 x float>, ptr %1535, align 32, !noalias !33864
  %_87.i.i.i107.i.sroa.0.0.copyload = load <8 x float>, ptr %1536, align 32, !noalias !33864
  %_90.i.i.i104.i.sroa.0.0.copyload = load <8 x float>, ptr %1537, align 32, !noalias !33864
  %_95.i.i.i100.i.sroa.0.0.copyload = load <8 x float>, ptr %row17.i.i.i227.i, align 32, !noalias !33864
  %_98.i.i.i97.i.sroa.0.0.copyload = load <8 x float>, ptr %1538, align 32, !noalias !33864
  %_101.i.i.i94.i.sroa.0.0.copyload = load <8 x float>, ptr %1539, align 32, !noalias !33864
  %_104.i.i.i91.i.sroa.0.0.copyload = load <8 x float>, ptr %1540, align 32, !noalias !33864
  %_109.i.i.i87.i.sroa.0.0.copyload = load <8 x float>, ptr %row18.i.i.i228.i, align 32, !noalias !33864
  %_112.i.i.i84.i.sroa.0.0.copyload = load <8 x float>, ptr %1541, align 32, !noalias !33864
  %_115.i.i.i81.i.sroa.0.0.copyload = load <8 x float>, ptr %1542, align 32, !noalias !33864
  %_118.i.i.i78.i.sroa.0.0.copyload = load <8 x float>, ptr %1543, align 32, !noalias !33864
  %_123.i.i.i74.i.sroa.0.0.copyload = load <8 x float>, ptr %row19.i.i.i229.i, align 32, !noalias !33864
  %_126.i.i.i71.i.sroa.0.0.copyload = load <8 x float>, ptr %1544, align 32, !noalias !33864
  %_129.i.i.i68.i.sroa.0.0.copyload = load <8 x float>, ptr %1545, align 32, !noalias !33864
  %_132.i.i.i65.i.sroa.0.0.copyload = load <8 x float>, ptr %1546, align 32, !noalias !33864
  %_137.i.i.i61.i.sroa.0.0.copyload = load <8 x float>, ptr %row20.i.i.i230.i, align 32, !noalias !33864
  %_140.i.i.i58.i.sroa.0.0.copyload = load <8 x float>, ptr %1547, align 32, !noalias !33864
  %_143.i.i.i55.i.sroa.0.0.copyload = load <8 x float>, ptr %1548, align 32, !noalias !33864
  %_146.i.i.i52.i.sroa.0.0.copyload = load <8 x float>, ptr %1549, align 32, !noalias !33864
  %_151.i.i.i48.i.sroa.0.0.copyload = load <8 x float>, ptr %row21.i.i.i231.i, align 32, !noalias !33864
  %_154.i.i.i45.i.sroa.0.0.copyload = load <8 x float>, ptr %1550, align 32, !noalias !33864
  %_157.i.i.i42.i.sroa.0.0.copyload = load <8 x float>, ptr %1551, align 32, !noalias !33864
  %_160.i.i.i39.i.sroa.0.0.copyload = load <8 x float>, ptr %1552, align 32, !noalias !33864
  %_165.i.i.i35.i.sroa.0.0.copyload = load <8 x float>, ptr %row22.i.i.i232.i, align 32, !noalias !33864
  %_168.i.i.i32.i.sroa.0.0.copyload = load <8 x float>, ptr %1553, align 32, !noalias !33864
  %_171.i.i.i29.i.sroa.0.0.copyload = load <8 x float>, ptr %1554, align 32, !noalias !33864
  %_174.i.i.i26.i.sroa.0.0.copyload = load <8 x float>, ptr %1555, align 32, !noalias !33864
  br label %bb6.i218.i, !dbg !33846

bb6.i218.i:                                       ; preds = %bb6.i218.i.lr.ph, %bb6.i218.i
  %history.i215.i.sroa.10.sroa.0.015708 = phi <8 x float> [ %history.i215.i.sroa.10.sroa.0.0.lcssa20924, %bb6.i218.i.lr.ph ], [ %history.i215.i.sroa.0.015697, %bb6.i218.i ]
  %history.i215.i.sroa.13.sroa.0.015707 = phi <8 x float> [ %history.i215.i.sroa.13.sroa.0.0.lcssa20943, %bb6.i218.i.lr.ph ], [ %history.i215.i.sroa.10.sroa.0.015708, %bb6.i218.i ]
  %history.i215.i.sroa.16.sroa.0.015706 = phi <8 x float> [ %history.i215.i.sroa.16.sroa.0.0.lcssa20962, %bb6.i218.i.lr.ph ], [ %history.i215.i.sroa.13.sroa.0.015707, %bb6.i218.i ]
  %history.i215.i.sroa.19.sroa.0.015705 = phi <8 x float> [ %history.i215.i.sroa.19.sroa.0.0.lcssa20963, %bb6.i218.i.lr.ph ], [ %history.i215.i.sroa.16.sroa.0.015706, %bb6.i218.i ]
  %history.i215.i.sroa.22.sroa.0.015704 = phi <8 x float> [ %history.i215.i.sroa.22.sroa.0.0.lcssa20964, %bb6.i218.i.lr.ph ], [ %history.i215.i.sroa.19.sroa.0.015705, %bb6.i218.i ]
  %history.i215.i.sroa.38.sroa.0.015703 = phi <8 x float> [ %history.i215.i.sroa.38.sroa.0.0.lcssa20969, %bb6.i218.i.lr.ph ], [ %history.i215.i.sroa.35.sroa.0.015702, %bb6.i218.i ]
  %history.i215.i.sroa.35.sroa.0.015702 = phi <8 x float> [ %history.i215.i.sroa.35.sroa.0.0.lcssa20968, %bb6.i218.i.lr.ph ], [ %history.i215.i.sroa.32.sroa.0.015701, %bb6.i218.i ]
  %history.i215.i.sroa.32.sroa.0.015701 = phi <8 x float> [ %history.i215.i.sroa.32.sroa.0.0.lcssa20967, %bb6.i218.i.lr.ph ], [ %history.i215.i.sroa.29.sroa.0.015700, %bb6.i218.i ]
  %history.i215.i.sroa.29.sroa.0.015700 = phi <8 x float> [ %history.i215.i.sroa.29.sroa.0.0.lcssa20966, %bb6.i218.i.lr.ph ], [ %history.i215.i.sroa.25.sroa.0.015699, %bb6.i218.i ]
  %history.i215.i.sroa.25.sroa.0.015699 = phi <8 x float> [ %history.i215.i.sroa.25.sroa.0.0.lcssa20965, %bb6.i218.i.lr.ph ], [ %history.i215.i.sroa.22.sroa.0.015704, %bb6.i218.i ]
  %iter.i211.i.sroa.16.015698 = phi i64 [ 0, %bb6.i218.i.lr.ph ], [ %1687, %bb6.i218.i ]
  %history.i215.i.sroa.0.015697 = phi <8 x float> [ %history.i215.i.sroa.0.0.lcssa20905, %bb6.i218.i.lr.ph ], [ %lanes.i6146.sroa.0.0.copyload, %bb6.i218.i ]
  %start1.i.i7802 = shl i64 %iter.i211.i.sroa.16.015698, 3, !dbg !33867
  %data.i.i7803 = getelementptr inbounds nuw float, ptr %_182.i, i64 %start1.i.i7802, !dbg !33869
  %lanes.i6146.sroa.0.0.copyload = load <8 x float>, ptr %data.i.i7803, align 4, !dbg !33871, !alias.scope !33876, !noalias !33880
  %1582 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i215.i.sroa.22.sroa.0.015704), !dbg !33885
  %1583 = fmul <8 x float> %lanes.i6146.sroa.0.0.copyload, %_5.i5557, !dbg !33892
  %1584 = fadd <8 x float> %1583, zeroinitializer, !dbg !33898
  %1585 = fmul <8 x float> %lanes.i6146.sroa.0.0.copyload, %_14.i.i.i175.i.sroa.0.0.copyload, !dbg !33903
  %1586 = fadd <8 x float> %1585, zeroinitializer, !dbg !33908
  %1587 = fmul <8 x float> %lanes.i6146.sroa.0.0.copyload, %_17.i.i.i172.i.sroa.0.0.copyload, !dbg !33913
  %1588 = fadd <8 x float> %1587, zeroinitializer, !dbg !33918
  %1589 = fmul <8 x float> %lanes.i6146.sroa.0.0.copyload, %_20.i.i.i169.i.sroa.0.0.copyload, !dbg !33923
  %1590 = fadd <8 x float> %1589, zeroinitializer, !dbg !33928
  %1591 = fmul <8 x float> %history.i215.i.sroa.0.015697, %_25.i.i.i165.i.sroa.0.0.copyload, !dbg !33933
  %1592 = fadd <8 x float> %1584, %1591, !dbg !33938
  %1593 = fmul <8 x float> %history.i215.i.sroa.0.015697, %_28.i.i.i162.i.sroa.0.0.copyload, !dbg !33943
  %1594 = fadd <8 x float> %1586, %1593, !dbg !33948
  %1595 = fmul <8 x float> %history.i215.i.sroa.0.015697, %_31.i.i.i159.i.sroa.0.0.copyload, !dbg !33953
  %1596 = fadd <8 x float> %1588, %1595, !dbg !33958
  %1597 = fmul <8 x float> %history.i215.i.sroa.0.015697, %_34.i.i.i156.i.sroa.0.0.copyload, !dbg !33963
  %1598 = fadd <8 x float> %1590, %1597, !dbg !33968
  %1599 = fmul <8 x float> %history.i215.i.sroa.10.sroa.0.015708, %_39.i.i.i152.i.sroa.0.0.copyload, !dbg !33973
  %1600 = fadd <8 x float> %1592, %1599, !dbg !33978
  %1601 = fmul <8 x float> %history.i215.i.sroa.10.sroa.0.015708, %_42.i.i.i149.i.sroa.0.0.copyload, !dbg !33983
  %1602 = fadd <8 x float> %1594, %1601, !dbg !33988
  %1603 = fmul <8 x float> %history.i215.i.sroa.10.sroa.0.015708, %_45.i.i.i146.i.sroa.0.0.copyload, !dbg !33993
  %1604 = fadd <8 x float> %1596, %1603, !dbg !33998
  %1605 = fmul <8 x float> %history.i215.i.sroa.10.sroa.0.015708, %_48.i.i.i143.i.sroa.0.0.copyload, !dbg !34003
  %1606 = fadd <8 x float> %1598, %1605, !dbg !34008
  %1607 = fmul <8 x float> %history.i215.i.sroa.13.sroa.0.015707, %_53.i.i.i139.i.sroa.0.0.copyload, !dbg !34013
  %1608 = fadd <8 x float> %1600, %1607, !dbg !34018
  %1609 = fmul <8 x float> %history.i215.i.sroa.13.sroa.0.015707, %_56.i.i.i136.i.sroa.0.0.copyload, !dbg !34023
  %1610 = fadd <8 x float> %1602, %1609, !dbg !34028
  %1611 = fmul <8 x float> %history.i215.i.sroa.13.sroa.0.015707, %_59.i.i.i133.i.sroa.0.0.copyload, !dbg !34033
  %1612 = fadd <8 x float> %1604, %1611, !dbg !34038
  %1613 = fmul <8 x float> %history.i215.i.sroa.13.sroa.0.015707, %_62.i.i.i130.i.sroa.0.0.copyload, !dbg !34043
  %1614 = fadd <8 x float> %1606, %1613, !dbg !34048
  %1615 = fmul <8 x float> %history.i215.i.sroa.16.sroa.0.015706, %_67.i.i.i126.i.sroa.0.0.copyload, !dbg !34053
  %1616 = fadd <8 x float> %1608, %1615, !dbg !34058
  %1617 = fmul <8 x float> %history.i215.i.sroa.16.sroa.0.015706, %_70.i.i.i123.i.sroa.0.0.copyload, !dbg !34063
  %1618 = fadd <8 x float> %1610, %1617, !dbg !34068
  %1619 = fmul <8 x float> %history.i215.i.sroa.16.sroa.0.015706, %_73.i.i.i120.i.sroa.0.0.copyload, !dbg !34073
  %1620 = fadd <8 x float> %1612, %1619, !dbg !34078
  %1621 = fmul <8 x float> %history.i215.i.sroa.16.sroa.0.015706, %_76.i.i.i117.i.sroa.0.0.copyload, !dbg !34083
  %1622 = fadd <8 x float> %1614, %1621, !dbg !34088
  %1623 = fmul <8 x float> %history.i215.i.sroa.19.sroa.0.015705, %_81.i.i.i113.i.sroa.0.0.copyload, !dbg !34093
  %1624 = fadd <8 x float> %1616, %1623, !dbg !34098
  %1625 = fmul <8 x float> %history.i215.i.sroa.19.sroa.0.015705, %_84.i.i.i110.i.sroa.0.0.copyload, !dbg !34103
  %1626 = fadd <8 x float> %1618, %1625, !dbg !34108
  %1627 = fmul <8 x float> %history.i215.i.sroa.19.sroa.0.015705, %_87.i.i.i107.i.sroa.0.0.copyload, !dbg !34113
  %1628 = fadd <8 x float> %1620, %1627, !dbg !34118
  %1629 = fmul <8 x float> %history.i215.i.sroa.19.sroa.0.015705, %_90.i.i.i104.i.sroa.0.0.copyload, !dbg !34123
  %1630 = fadd <8 x float> %1622, %1629, !dbg !34128
  %1631 = fmul <8 x float> %history.i215.i.sroa.22.sroa.0.015704, %_95.i.i.i100.i.sroa.0.0.copyload, !dbg !34133
  %1632 = fadd <8 x float> %1624, %1631, !dbg !34138
  %1633 = fmul <8 x float> %history.i215.i.sroa.22.sroa.0.015704, %_98.i.i.i97.i.sroa.0.0.copyload, !dbg !34143
  %1634 = fadd <8 x float> %1626, %1633, !dbg !34148
  %1635 = fmul <8 x float> %history.i215.i.sroa.22.sroa.0.015704, %_101.i.i.i94.i.sroa.0.0.copyload, !dbg !34153
  %1636 = fadd <8 x float> %1628, %1635, !dbg !34158
  %1637 = fmul <8 x float> %history.i215.i.sroa.22.sroa.0.015704, %_104.i.i.i91.i.sroa.0.0.copyload, !dbg !34163
  %1638 = fadd <8 x float> %1630, %1637, !dbg !34168
  %1639 = fmul <8 x float> %history.i215.i.sroa.25.sroa.0.015699, %_109.i.i.i87.i.sroa.0.0.copyload, !dbg !34173
  %1640 = fadd <8 x float> %1632, %1639, !dbg !34178
  %1641 = fmul <8 x float> %history.i215.i.sroa.25.sroa.0.015699, %_112.i.i.i84.i.sroa.0.0.copyload, !dbg !34183
  %1642 = fadd <8 x float> %1634, %1641, !dbg !34188
  %1643 = fmul <8 x float> %history.i215.i.sroa.25.sroa.0.015699, %_115.i.i.i81.i.sroa.0.0.copyload, !dbg !34193
  %1644 = fadd <8 x float> %1636, %1643, !dbg !34198
  %1645 = fmul <8 x float> %history.i215.i.sroa.25.sroa.0.015699, %_118.i.i.i78.i.sroa.0.0.copyload, !dbg !34203
  %1646 = fadd <8 x float> %1638, %1645, !dbg !34208
  %1647 = fmul <8 x float> %history.i215.i.sroa.29.sroa.0.015700, %_123.i.i.i74.i.sroa.0.0.copyload, !dbg !34213
  %1648 = fadd <8 x float> %1640, %1647, !dbg !34218
  %1649 = fmul <8 x float> %history.i215.i.sroa.29.sroa.0.015700, %_126.i.i.i71.i.sroa.0.0.copyload, !dbg !34223
  %1650 = fadd <8 x float> %1642, %1649, !dbg !34228
  %1651 = fmul <8 x float> %history.i215.i.sroa.29.sroa.0.015700, %_129.i.i.i68.i.sroa.0.0.copyload, !dbg !34233
  %1652 = fadd <8 x float> %1644, %1651, !dbg !34238
  %1653 = fmul <8 x float> %history.i215.i.sroa.29.sroa.0.015700, %_132.i.i.i65.i.sroa.0.0.copyload, !dbg !34243
  %1654 = fadd <8 x float> %1646, %1653, !dbg !34248
  %1655 = fmul <8 x float> %history.i215.i.sroa.32.sroa.0.015701, %_137.i.i.i61.i.sroa.0.0.copyload, !dbg !34253
  %1656 = fadd <8 x float> %1648, %1655, !dbg !34258
  %1657 = fmul <8 x float> %history.i215.i.sroa.32.sroa.0.015701, %_140.i.i.i58.i.sroa.0.0.copyload, !dbg !34263
  %1658 = fadd <8 x float> %1650, %1657, !dbg !34268
  %1659 = fmul <8 x float> %history.i215.i.sroa.32.sroa.0.015701, %_143.i.i.i55.i.sroa.0.0.copyload, !dbg !34273
  %1660 = fadd <8 x float> %1652, %1659, !dbg !34278
  %1661 = fmul <8 x float> %history.i215.i.sroa.32.sroa.0.015701, %_146.i.i.i52.i.sroa.0.0.copyload, !dbg !34283
  %1662 = fadd <8 x float> %1654, %1661, !dbg !34288
  %1663 = fmul <8 x float> %history.i215.i.sroa.35.sroa.0.015702, %_151.i.i.i48.i.sroa.0.0.copyload, !dbg !34293
  %1664 = fadd <8 x float> %1656, %1663, !dbg !34298
  %1665 = fmul <8 x float> %history.i215.i.sroa.35.sroa.0.015702, %_154.i.i.i45.i.sroa.0.0.copyload, !dbg !34303
  %1666 = fadd <8 x float> %1658, %1665, !dbg !34308
  %1667 = fmul <8 x float> %history.i215.i.sroa.35.sroa.0.015702, %_157.i.i.i42.i.sroa.0.0.copyload, !dbg !34313
  %1668 = fadd <8 x float> %1660, %1667, !dbg !34318
  %1669 = fmul <8 x float> %history.i215.i.sroa.35.sroa.0.015702, %_160.i.i.i39.i.sroa.0.0.copyload, !dbg !34323
  %1670 = fadd <8 x float> %1662, %1669, !dbg !34328
  %1671 = fmul <8 x float> %history.i215.i.sroa.38.sroa.0.015703, %_165.i.i.i35.i.sroa.0.0.copyload, !dbg !34333
  %1672 = fadd <8 x float> %1664, %1671, !dbg !34338
  %1673 = fmul <8 x float> %history.i215.i.sroa.38.sroa.0.015703, %_168.i.i.i32.i.sroa.0.0.copyload, !dbg !34343
  %1674 = fadd <8 x float> %1666, %1673, !dbg !34348
  %1675 = fmul <8 x float> %history.i215.i.sroa.38.sroa.0.015703, %_171.i.i.i29.i.sroa.0.0.copyload, !dbg !34353
  %1676 = fadd <8 x float> %1668, %1675, !dbg !34358
  %1677 = fmul <8 x float> %history.i215.i.sroa.38.sroa.0.015703, %_174.i.i.i26.i.sroa.0.0.copyload, !dbg !34363
  %1678 = fadd <8 x float> %1670, %1677, !dbg !34368
  %1679 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1672), !dbg !34373
  %1680 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1582, <8 x float> %1679), !dbg !34379
  %1681 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1674), !dbg !34373
  %1682 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1680, <8 x float> %1681), !dbg !34379
  %1683 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1676), !dbg !34373
  %1684 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1682, <8 x float> %1683), !dbg !34379
  %1685 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1678), !dbg !34373
  %1686 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1684, <8 x float> %1685), !dbg !34379
  %1687 = add nuw nsw i64 %iter.i211.i.sroa.16.015698, 1, !dbg !34384
  %data.i4.i7807 = getelementptr inbounds nuw float, ptr %peaks_left.i, i64 %start1.i.i7802, !dbg !34385
  store <8 x float> %1686, ptr %data.i4.i7807, align 4, !dbg !34388, !alias.scope !34393, !noalias !34397
  %exitcond18073.not = icmp eq i64 %1687, %umax18077, !dbg !33846
  br i1 %exitcond18073.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit239.i, label %bb6.i218.i, !dbg !33846

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit239.i: ; preds = %bb6.i218.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7793
  %history.i215.i.sroa.0.0.lcssa = phi <8 x float> [ %history.i215.i.sroa.0.0.lcssa20905, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7793 ], [ %lanes.i6146.sroa.0.0.copyload, %bb6.i218.i ], !dbg !33837
  %history.i215.i.sroa.25.sroa.0.0.lcssa = phi <8 x float> [ %history.i215.i.sroa.25.sroa.0.0.lcssa20965, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7793 ], [ %history.i215.i.sroa.22.sroa.0.015704, %bb6.i218.i ], !dbg !33837
  %history.i215.i.sroa.29.sroa.0.0.lcssa = phi <8 x float> [ %history.i215.i.sroa.29.sroa.0.0.lcssa20966, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7793 ], [ %history.i215.i.sroa.25.sroa.0.015699, %bb6.i218.i ], !dbg !33837
  %history.i215.i.sroa.32.sroa.0.0.lcssa = phi <8 x float> [ %history.i215.i.sroa.32.sroa.0.0.lcssa20967, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7793 ], [ %history.i215.i.sroa.29.sroa.0.015700, %bb6.i218.i ], !dbg !33837
  %history.i215.i.sroa.35.sroa.0.0.lcssa = phi <8 x float> [ %history.i215.i.sroa.35.sroa.0.0.lcssa20968, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7793 ], [ %history.i215.i.sroa.32.sroa.0.015701, %bb6.i218.i ], !dbg !33837
  %history.i215.i.sroa.38.sroa.0.0.lcssa = phi <8 x float> [ %history.i215.i.sroa.38.sroa.0.0.lcssa20969, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7793 ], [ %history.i215.i.sroa.35.sroa.0.015702, %bb6.i218.i ], !dbg !33837
  %history.i215.i.sroa.41.sroa.0.0.lcssa = phi <8 x float> [ %history.i215.i.sroa.41.sroa.0.0.lcssa20970, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7793 ], [ %history.i215.i.sroa.38.sroa.0.015703, %bb6.i218.i ], !dbg !33837
  %history.i215.i.sroa.22.sroa.0.0.lcssa = phi <8 x float> [ %history.i215.i.sroa.22.sroa.0.0.lcssa20964, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7793 ], [ %history.i215.i.sroa.19.sroa.0.015705, %bb6.i218.i ], !dbg !33837
  %history.i215.i.sroa.19.sroa.0.0.lcssa = phi <8 x float> [ %history.i215.i.sroa.19.sroa.0.0.lcssa20963, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7793 ], [ %history.i215.i.sroa.16.sroa.0.015706, %bb6.i218.i ], !dbg !33837
  %history.i215.i.sroa.16.sroa.0.0.lcssa = phi <8 x float> [ %history.i215.i.sroa.16.sroa.0.0.lcssa20962, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7793 ], [ %history.i215.i.sroa.13.sroa.0.015707, %bb6.i218.i ], !dbg !33837
  %history.i215.i.sroa.13.sroa.0.0.lcssa = phi <8 x float> [ %history.i215.i.sroa.13.sroa.0.0.lcssa20943, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7793 ], [ %history.i215.i.sroa.10.sroa.0.015708, %bb6.i218.i ], !dbg !33837
  %history.i215.i.sroa.10.sroa.0.0.lcssa = phi <8 x float> [ %history.i215.i.sroa.10.sroa.0.0.lcssa20924, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7793 ], [ %history.i215.i.sroa.0.015697, %bb6.i218.i ], !dbg !33837
  store <8 x float> %history.i215.i.sroa.16.sroa.0.0.lcssa, ptr %history.i215.i.sroa.16.0.hot_left.i.sroa_idx, align 32, !dbg !34401
  store <8 x float> %history.i215.i.sroa.19.sroa.0.0.lcssa, ptr %history.i215.i.sroa.19.0.hot_left.i.sroa_idx, align 32, !dbg !34401
  store <8 x float> %history.i215.i.sroa.22.sroa.0.0.lcssa, ptr %history.i215.i.sroa.22.0.hot_left.i.sroa_idx, align 32, !dbg !34401
  store <8 x float> %history.i215.i.sroa.25.sroa.0.0.lcssa, ptr %history.i215.i.sroa.25.0.hot_left.i.sroa_idx, align 32, !dbg !34401
  store <8 x float> %history.i215.i.sroa.29.sroa.0.0.lcssa, ptr %history.i215.i.sroa.29.0.hot_left.i.sroa_idx, align 32, !dbg !34401
  store <8 x float> %history.i215.i.sroa.32.sroa.0.0.lcssa, ptr %history.i215.i.sroa.32.0.hot_left.i.sroa_idx, align 32, !dbg !34401
  store <8 x float> %history.i215.i.sroa.35.sroa.0.0.lcssa, ptr %history.i215.i.sroa.35.0.hot_left.i.sroa_idx, align 32, !dbg !34401
  store <8 x float> %history.i215.i.sroa.38.sroa.0.0.lcssa, ptr %history.i215.i.sroa.38.0.hot_left.i.sroa_idx, align 32, !dbg !34401
  store <8 x float> %history.i215.i.sroa.41.sroa.0.0.lcssa, ptr %history.i215.i.sroa.41.0.hot_left.i.sroa_idx, align 32, !dbg !34401
  %_190.not.i = icmp ugt i64 %_52.i, %right_io.1, !dbg !34402
  br i1 %_190.not.i, label %bb60.i, label %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7853, !dbg !34402, !prof !1406

bb60.i:                                           ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit239.i
  store <8 x float> %history.i215.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.10.sroa.0.0.lcssa, ptr %history.i215.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.13.sroa.0.0.lcssa, ptr %history.i215.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i.i.sroa.0.0.lcssa20971, ptr %hot_right.i, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa20990, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa21009, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa21037, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015770.lcssa15926.lcssa21056, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1883.lcssa1581415955.lcssa21076, ptr %_22.i293.i, align 4
  store i32 %storemerge.i1848.lcssa1587116027.lcssa21099, ptr %_22.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %active_base.i, i64 noundef %_52.i, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_480b8302a9d65cc7541e43747df38ed7) #30, !dbg !34406, !noalias !33742
  unreachable, !dbg !34406

_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7853: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit239.i
  %_197.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %active_base.i, !dbg !34407
  br i1 %_2.i779615696.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, label %bb6.i.i.lr.ph, !dbg !34411

bb6.i.i.lr.ph:                                    ; preds = %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7853
  %_5.i5413 = load <8 x float>, ptr %self, align 32, !alias.scope !34414, !noalias !34417
  %_14.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1520, align 32, !noalias !34429
  %_17.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1521, align 32, !noalias !34429
  %_20.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1522, align 32, !noalias !34429
  %_25.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row12.i.i.i222.i, align 32, !noalias !34429
  %_28.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1523, align 32, !noalias !34429
  %_31.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1524, align 32, !noalias !34429
  %_34.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1525, align 32, !noalias !34429
  %_39.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row13.i.i.i223.i, align 32, !noalias !34429
  %_42.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1526, align 32, !noalias !34429
  %_45.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1527, align 32, !noalias !34429
  %_48.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1528, align 32, !noalias !34429
  %_53.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row14.i.i.i224.i, align 32, !noalias !34429
  %_56.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1529, align 32, !noalias !34429
  %_59.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1530, align 32, !noalias !34429
  %_62.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1531, align 32, !noalias !34429
  %_67.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row15.i.i.i225.i, align 32, !noalias !34429
  %_70.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1532, align 32, !noalias !34429
  %_73.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1533, align 32, !noalias !34429
  %_76.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1534, align 32, !noalias !34429
  %_81.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row16.i.i.i226.i, align 32, !noalias !34429
  %_84.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1535, align 32, !noalias !34429
  %_87.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1536, align 32, !noalias !34429
  %_90.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1537, align 32, !noalias !34429
  %_95.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row17.i.i.i227.i, align 32, !noalias !34429
  %_98.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1538, align 32, !noalias !34429
  %_101.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1539, align 32, !noalias !34429
  %_104.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1540, align 32, !noalias !34429
  %_109.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row18.i.i.i228.i, align 32, !noalias !34429
  %_112.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1541, align 32, !noalias !34429
  %_115.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1542, align 32, !noalias !34429
  %_118.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1543, align 32, !noalias !34429
  %_123.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row19.i.i.i229.i, align 32, !noalias !34429
  %_126.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1544, align 32, !noalias !34429
  %_129.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1545, align 32, !noalias !34429
  %_132.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1546, align 32, !noalias !34429
  %_137.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row20.i.i.i230.i, align 32, !noalias !34429
  %_140.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1547, align 32, !noalias !34429
  %_143.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1548, align 32, !noalias !34429
  %_146.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1549, align 32, !noalias !34429
  %_151.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row21.i.i.i231.i, align 32, !noalias !34429
  %_154.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1550, align 32, !noalias !34429
  %_157.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1551, align 32, !noalias !34429
  %_160.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1552, align 32, !noalias !34429
  %_165.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %row22.i.i.i232.i, align 32, !noalias !34429
  %_168.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1553, align 32, !noalias !34429
  %_171.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1554, align 32, !noalias !34429
  %_174.i.i.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1555, align 32, !noalias !34429
  br label %bb6.i.i, !dbg !34411

bb6.i.i:                                          ; preds = %bb6.i.i.lr.ph, %bb6.i.i
  %history.i.i.sroa.0.015735 = phi <8 x float> [ %history.i.i.sroa.0.0.lcssa20971, %bb6.i.i.lr.ph ], [ %lanes.i6137.sroa.0.0.copyload, %bb6.i.i ]
  %iter.i.i.sroa.16.015734 = phi i64 [ 0, %bb6.i.i.lr.ph ], [ %1793, %bb6.i.i ]
  %history.i.i.sroa.25.sroa.0.015733 = phi <8 x float> [ %history.i.i.sroa.25.sroa.0.0.lcssa21031, %bb6.i.i.lr.ph ], [ %history.i.i.sroa.22.sroa.0.015728, %bb6.i.i ]
  %history.i.i.sroa.29.sroa.0.015732 = phi <8 x float> [ %history.i.i.sroa.29.sroa.0.0.lcssa21032, %bb6.i.i.lr.ph ], [ %history.i.i.sroa.25.sroa.0.015733, %bb6.i.i ]
  %history.i.i.sroa.32.sroa.0.015731 = phi <8 x float> [ %history.i.i.sroa.32.sroa.0.0.lcssa21033, %bb6.i.i.lr.ph ], [ %history.i.i.sroa.29.sroa.0.015732, %bb6.i.i ]
  %history.i.i.sroa.35.sroa.0.015730 = phi <8 x float> [ %history.i.i.sroa.35.sroa.0.0.lcssa21034, %bb6.i.i.lr.ph ], [ %history.i.i.sroa.32.sroa.0.015731, %bb6.i.i ]
  %history.i.i.sroa.38.sroa.0.015729 = phi <8 x float> [ %history.i.i.sroa.38.sroa.0.0.lcssa21035, %bb6.i.i.lr.ph ], [ %history.i.i.sroa.35.sroa.0.015730, %bb6.i.i ]
  %history.i.i.sroa.22.sroa.0.015728 = phi <8 x float> [ %history.i.i.sroa.22.sroa.0.0.lcssa21030, %bb6.i.i.lr.ph ], [ %history.i.i.sroa.19.sroa.0.015727, %bb6.i.i ]
  %history.i.i.sroa.19.sroa.0.015727 = phi <8 x float> [ %history.i.i.sroa.19.sroa.0.0.lcssa21029, %bb6.i.i.lr.ph ], [ %history.i.i.sroa.16.sroa.0.015726, %bb6.i.i ]
  %history.i.i.sroa.16.sroa.0.015726 = phi <8 x float> [ %history.i.i.sroa.16.sroa.0.0.lcssa21028, %bb6.i.i.lr.ph ], [ %history.i.i.sroa.13.sroa.0.015725, %bb6.i.i ]
  %history.i.i.sroa.13.sroa.0.015725 = phi <8 x float> [ %history.i.i.sroa.13.sroa.0.0.lcssa21009, %bb6.i.i.lr.ph ], [ %history.i.i.sroa.10.sroa.0.015724, %bb6.i.i ]
  %history.i.i.sroa.10.sroa.0.015724 = phi <8 x float> [ %history.i.i.sroa.10.sroa.0.0.lcssa20990, %bb6.i.i.lr.ph ], [ %history.i.i.sroa.0.015735, %bb6.i.i ]
  %start1.i.i7862 = shl i64 %iter.i.i.sroa.16.015734, 3, !dbg !34432
  %data.i.i7863 = getelementptr inbounds nuw float, ptr %_197.i, i64 %start1.i.i7862, !dbg !34434
  %lanes.i6137.sroa.0.0.copyload = load <8 x float>, ptr %data.i.i7863, align 4, !dbg !34436, !alias.scope !34441, !noalias !34445
  %1688 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i.i.sroa.22.sroa.0.015728), !dbg !34450
  %1689 = fmul <8 x float> %lanes.i6137.sroa.0.0.copyload, %_5.i5413, !dbg !34457
  %1690 = fadd <8 x float> %1689, zeroinitializer, !dbg !34463
  %1691 = fmul <8 x float> %lanes.i6137.sroa.0.0.copyload, %_14.i.i.i.i.sroa.0.0.copyload, !dbg !34468
  %1692 = fadd <8 x float> %1691, zeroinitializer, !dbg !34473
  %1693 = fmul <8 x float> %lanes.i6137.sroa.0.0.copyload, %_17.i.i.i.i.sroa.0.0.copyload, !dbg !34478
  %1694 = fadd <8 x float> %1693, zeroinitializer, !dbg !34483
  %1695 = fmul <8 x float> %lanes.i6137.sroa.0.0.copyload, %_20.i.i.i.i.sroa.0.0.copyload, !dbg !34488
  %1696 = fadd <8 x float> %1695, zeroinitializer, !dbg !34493
  %1697 = fmul <8 x float> %history.i.i.sroa.0.015735, %_25.i.i.i.i.sroa.0.0.copyload, !dbg !34498
  %1698 = fadd <8 x float> %1690, %1697, !dbg !34503
  %1699 = fmul <8 x float> %history.i.i.sroa.0.015735, %_28.i.i.i.i.sroa.0.0.copyload, !dbg !34508
  %1700 = fadd <8 x float> %1692, %1699, !dbg !34513
  %1701 = fmul <8 x float> %history.i.i.sroa.0.015735, %_31.i.i.i.i.sroa.0.0.copyload, !dbg !34518
  %1702 = fadd <8 x float> %1694, %1701, !dbg !34523
  %1703 = fmul <8 x float> %history.i.i.sroa.0.015735, %_34.i.i.i.i.sroa.0.0.copyload, !dbg !34528
  %1704 = fadd <8 x float> %1696, %1703, !dbg !34533
  %1705 = fmul <8 x float> %history.i.i.sroa.10.sroa.0.015724, %_39.i.i.i.i.sroa.0.0.copyload, !dbg !34538
  %1706 = fadd <8 x float> %1698, %1705, !dbg !34543
  %1707 = fmul <8 x float> %history.i.i.sroa.10.sroa.0.015724, %_42.i.i.i.i.sroa.0.0.copyload, !dbg !34548
  %1708 = fadd <8 x float> %1700, %1707, !dbg !34553
  %1709 = fmul <8 x float> %history.i.i.sroa.10.sroa.0.015724, %_45.i.i.i.i.sroa.0.0.copyload, !dbg !34558
  %1710 = fadd <8 x float> %1702, %1709, !dbg !34563
  %1711 = fmul <8 x float> %history.i.i.sroa.10.sroa.0.015724, %_48.i.i.i.i.sroa.0.0.copyload, !dbg !34568
  %1712 = fadd <8 x float> %1704, %1711, !dbg !34573
  %1713 = fmul <8 x float> %history.i.i.sroa.13.sroa.0.015725, %_53.i.i.i.i.sroa.0.0.copyload, !dbg !34578
  %1714 = fadd <8 x float> %1706, %1713, !dbg !34583
  %1715 = fmul <8 x float> %history.i.i.sroa.13.sroa.0.015725, %_56.i.i.i.i.sroa.0.0.copyload, !dbg !34588
  %1716 = fadd <8 x float> %1708, %1715, !dbg !34593
  %1717 = fmul <8 x float> %history.i.i.sroa.13.sroa.0.015725, %_59.i.i.i.i.sroa.0.0.copyload, !dbg !34598
  %1718 = fadd <8 x float> %1710, %1717, !dbg !34603
  %1719 = fmul <8 x float> %history.i.i.sroa.13.sroa.0.015725, %_62.i.i.i.i.sroa.0.0.copyload, !dbg !34608
  %1720 = fadd <8 x float> %1712, %1719, !dbg !34613
  %1721 = fmul <8 x float> %history.i.i.sroa.16.sroa.0.015726, %_67.i.i.i.i.sroa.0.0.copyload, !dbg !34618
  %1722 = fadd <8 x float> %1714, %1721, !dbg !34623
  %1723 = fmul <8 x float> %history.i.i.sroa.16.sroa.0.015726, %_70.i.i.i.i.sroa.0.0.copyload, !dbg !34628
  %1724 = fadd <8 x float> %1716, %1723, !dbg !34633
  %1725 = fmul <8 x float> %history.i.i.sroa.16.sroa.0.015726, %_73.i.i.i.i.sroa.0.0.copyload, !dbg !34638
  %1726 = fadd <8 x float> %1718, %1725, !dbg !34643
  %1727 = fmul <8 x float> %history.i.i.sroa.16.sroa.0.015726, %_76.i.i.i.i.sroa.0.0.copyload, !dbg !34648
  %1728 = fadd <8 x float> %1720, %1727, !dbg !34653
  %1729 = fmul <8 x float> %history.i.i.sroa.19.sroa.0.015727, %_81.i.i.i.i.sroa.0.0.copyload, !dbg !34658
  %1730 = fadd <8 x float> %1722, %1729, !dbg !34663
  %1731 = fmul <8 x float> %history.i.i.sroa.19.sroa.0.015727, %_84.i.i.i.i.sroa.0.0.copyload, !dbg !34668
  %1732 = fadd <8 x float> %1724, %1731, !dbg !34673
  %1733 = fmul <8 x float> %history.i.i.sroa.19.sroa.0.015727, %_87.i.i.i.i.sroa.0.0.copyload, !dbg !34678
  %1734 = fadd <8 x float> %1726, %1733, !dbg !34683
  %1735 = fmul <8 x float> %history.i.i.sroa.19.sroa.0.015727, %_90.i.i.i.i.sroa.0.0.copyload, !dbg !34688
  %1736 = fadd <8 x float> %1728, %1735, !dbg !34693
  %1737 = fmul <8 x float> %history.i.i.sroa.22.sroa.0.015728, %_95.i.i.i.i.sroa.0.0.copyload, !dbg !34698
  %1738 = fadd <8 x float> %1730, %1737, !dbg !34703
  %1739 = fmul <8 x float> %history.i.i.sroa.22.sroa.0.015728, %_98.i.i.i.i.sroa.0.0.copyload, !dbg !34708
  %1740 = fadd <8 x float> %1732, %1739, !dbg !34713
  %1741 = fmul <8 x float> %history.i.i.sroa.22.sroa.0.015728, %_101.i.i.i.i.sroa.0.0.copyload, !dbg !34718
  %1742 = fadd <8 x float> %1734, %1741, !dbg !34723
  %1743 = fmul <8 x float> %history.i.i.sroa.22.sroa.0.015728, %_104.i.i.i.i.sroa.0.0.copyload, !dbg !34728
  %1744 = fadd <8 x float> %1736, %1743, !dbg !34733
  %1745 = fmul <8 x float> %history.i.i.sroa.25.sroa.0.015733, %_109.i.i.i.i.sroa.0.0.copyload, !dbg !34738
  %1746 = fadd <8 x float> %1738, %1745, !dbg !34743
  %1747 = fmul <8 x float> %history.i.i.sroa.25.sroa.0.015733, %_112.i.i.i.i.sroa.0.0.copyload, !dbg !34748
  %1748 = fadd <8 x float> %1740, %1747, !dbg !34753
  %1749 = fmul <8 x float> %history.i.i.sroa.25.sroa.0.015733, %_115.i.i.i.i.sroa.0.0.copyload, !dbg !34758
  %1750 = fadd <8 x float> %1742, %1749, !dbg !34763
  %1751 = fmul <8 x float> %history.i.i.sroa.25.sroa.0.015733, %_118.i.i.i.i.sroa.0.0.copyload, !dbg !34768
  %1752 = fadd <8 x float> %1744, %1751, !dbg !34773
  %1753 = fmul <8 x float> %history.i.i.sroa.29.sroa.0.015732, %_123.i.i.i.i.sroa.0.0.copyload, !dbg !34778
  %1754 = fadd <8 x float> %1746, %1753, !dbg !34783
  %1755 = fmul <8 x float> %history.i.i.sroa.29.sroa.0.015732, %_126.i.i.i.i.sroa.0.0.copyload, !dbg !34788
  %1756 = fadd <8 x float> %1748, %1755, !dbg !34793
  %1757 = fmul <8 x float> %history.i.i.sroa.29.sroa.0.015732, %_129.i.i.i.i.sroa.0.0.copyload, !dbg !34798
  %1758 = fadd <8 x float> %1750, %1757, !dbg !34803
  %1759 = fmul <8 x float> %history.i.i.sroa.29.sroa.0.015732, %_132.i.i.i.i.sroa.0.0.copyload, !dbg !34808
  %1760 = fadd <8 x float> %1752, %1759, !dbg !34813
  %1761 = fmul <8 x float> %history.i.i.sroa.32.sroa.0.015731, %_137.i.i.i.i.sroa.0.0.copyload, !dbg !34818
  %1762 = fadd <8 x float> %1754, %1761, !dbg !34823
  %1763 = fmul <8 x float> %history.i.i.sroa.32.sroa.0.015731, %_140.i.i.i.i.sroa.0.0.copyload, !dbg !34828
  %1764 = fadd <8 x float> %1756, %1763, !dbg !34833
  %1765 = fmul <8 x float> %history.i.i.sroa.32.sroa.0.015731, %_143.i.i.i.i.sroa.0.0.copyload, !dbg !34838
  %1766 = fadd <8 x float> %1758, %1765, !dbg !34843
  %1767 = fmul <8 x float> %history.i.i.sroa.32.sroa.0.015731, %_146.i.i.i.i.sroa.0.0.copyload, !dbg !34848
  %1768 = fadd <8 x float> %1760, %1767, !dbg !34853
  %1769 = fmul <8 x float> %history.i.i.sroa.35.sroa.0.015730, %_151.i.i.i.i.sroa.0.0.copyload, !dbg !34858
  %1770 = fadd <8 x float> %1762, %1769, !dbg !34863
  %1771 = fmul <8 x float> %history.i.i.sroa.35.sroa.0.015730, %_154.i.i.i.i.sroa.0.0.copyload, !dbg !34868
  %1772 = fadd <8 x float> %1764, %1771, !dbg !34873
  %1773 = fmul <8 x float> %history.i.i.sroa.35.sroa.0.015730, %_157.i.i.i.i.sroa.0.0.copyload, !dbg !34878
  %1774 = fadd <8 x float> %1766, %1773, !dbg !34883
  %1775 = fmul <8 x float> %history.i.i.sroa.35.sroa.0.015730, %_160.i.i.i.i.sroa.0.0.copyload, !dbg !34888
  %1776 = fadd <8 x float> %1768, %1775, !dbg !34893
  %1777 = fmul <8 x float> %history.i.i.sroa.38.sroa.0.015729, %_165.i.i.i.i.sroa.0.0.copyload, !dbg !34898
  %1778 = fadd <8 x float> %1770, %1777, !dbg !34903
  %1779 = fmul <8 x float> %history.i.i.sroa.38.sroa.0.015729, %_168.i.i.i.i.sroa.0.0.copyload, !dbg !34908
  %1780 = fadd <8 x float> %1772, %1779, !dbg !34913
  %1781 = fmul <8 x float> %history.i.i.sroa.38.sroa.0.015729, %_171.i.i.i.i.sroa.0.0.copyload, !dbg !34918
  %1782 = fadd <8 x float> %1774, %1781, !dbg !34923
  %1783 = fmul <8 x float> %history.i.i.sroa.38.sroa.0.015729, %_174.i.i.i.i.sroa.0.0.copyload, !dbg !34928
  %1784 = fadd <8 x float> %1776, %1783, !dbg !34933
  %1785 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1778), !dbg !34938
  %1786 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1688, <8 x float> %1785), !dbg !34944
  %1787 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1780), !dbg !34938
  %1788 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1786, <8 x float> %1787), !dbg !34944
  %1789 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1782), !dbg !34938
  %1790 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1788, <8 x float> %1789), !dbg !34944
  %1791 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1784), !dbg !34938
  %1792 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1790, <8 x float> %1791), !dbg !34944
  %1793 = add nuw nsw i64 %iter.i.i.sroa.16.015734, 1, !dbg !34949
  %data.i4.i7867 = getelementptr inbounds nuw float, ptr %peaks_right.i, i64 %start1.i.i7862, !dbg !34950
  store <8 x float> %1792, ptr %data.i4.i7867, align 4, !dbg !34953, !alias.scope !34958, !noalias !34962
  %exitcond18078.not = icmp eq i64 %1793, %umax18077, !dbg !34411
  br i1 %exitcond18078.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, label %bb6.i.i, !dbg !34411

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i: ; preds = %bb6.i.i, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7853
  %history.i.i.sroa.10.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.10.sroa.0.0.lcssa20990, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7853 ], [ %history.i.i.sroa.0.015735, %bb6.i.i ], !dbg !33839
  %history.i.i.sroa.13.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.13.sroa.0.0.lcssa21009, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7853 ], [ %history.i.i.sroa.10.sroa.0.015724, %bb6.i.i ], !dbg !33839
  %history.i.i.sroa.16.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.16.sroa.0.0.lcssa21028, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7853 ], [ %history.i.i.sroa.13.sroa.0.015725, %bb6.i.i ], !dbg !33839
  %history.i.i.sroa.19.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.19.sroa.0.0.lcssa21029, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7853 ], [ %history.i.i.sroa.16.sroa.0.015726, %bb6.i.i ], !dbg !33839
  %history.i.i.sroa.22.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.22.sroa.0.0.lcssa21030, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7853 ], [ %history.i.i.sroa.19.sroa.0.015727, %bb6.i.i ], !dbg !33839
  %history.i.i.sroa.41.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.41.sroa.0.0.lcssa21036, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7853 ], [ %history.i.i.sroa.38.sroa.0.015729, %bb6.i.i ], !dbg !33839
  %history.i.i.sroa.38.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.38.sroa.0.0.lcssa21035, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7853 ], [ %history.i.i.sroa.35.sroa.0.015730, %bb6.i.i ], !dbg !33839
  %history.i.i.sroa.35.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.35.sroa.0.0.lcssa21034, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7853 ], [ %history.i.i.sroa.32.sroa.0.015731, %bb6.i.i ], !dbg !33839
  %history.i.i.sroa.32.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.32.sroa.0.0.lcssa21033, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7853 ], [ %history.i.i.sroa.29.sroa.0.015732, %bb6.i.i ], !dbg !33839
  %history.i.i.sroa.29.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.29.sroa.0.0.lcssa21032, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7853 ], [ %history.i.i.sroa.25.sroa.0.015733, %bb6.i.i ], !dbg !33839
  %history.i.i.sroa.25.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.25.sroa.0.0.lcssa21031, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7853 ], [ %history.i.i.sroa.22.sroa.0.015728, %bb6.i.i ], !dbg !33839
  %history.i.i.sroa.0.0.lcssa = phi <8 x float> [ %history.i.i.sroa.0.0.lcssa20971, %_RINvYINtNtNtCs4NRVxsYgnAr_4core5slice4iter11ChunksExactfENtNtNtNtBa_4iter6traits8iterator8Iterator3zipINtB6_14ChunksExactMutfEECsdvPQf9CMsz3_17true_peak_limiter.exit7853 ], [ %lanes.i6137.sroa.0.0.copyload, %bb6.i.i ], !dbg !33839
  store <8 x float> %history.i.i.sroa.16.sroa.0.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 32, !dbg !34966
  store <8 x float> %history.i.i.sroa.19.sroa.0.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 32, !dbg !34966
  store <8 x float> %history.i.i.sroa.22.sroa.0.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 32, !dbg !34966
  store <8 x float> %history.i.i.sroa.25.sroa.0.0.lcssa, ptr %history.i.i.sroa.25.0.hot_right.i.sroa_idx, align 32, !dbg !34966
  store <8 x float> %history.i.i.sroa.29.sroa.0.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 32, !dbg !34966
  store <8 x float> %history.i.i.sroa.32.sroa.0.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 32, !dbg !34966
  store <8 x float> %history.i.i.sroa.35.sroa.0.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 32, !dbg !34966
  store <8 x float> %history.i.i.sroa.38.sroa.0.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 32, !dbg !34966
  store <8 x float> %history.i.i.sroa.41.sroa.0.0.lcssa, ptr %history.i.i.sroa.41.0.hot_right.i.sroa_idx, align 32, !dbg !34966
  br i1 %_2.i779615696.not, label %bb15.i.loopexit, label %bb20.i.lr.ph, !dbg !33810

bb20.i.lr.ph:                                     ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i
  %_70.i.sroa.3.0.copyload.pre = load i64, ptr %_70.i.sroa.3.0..sroa_idx, align 8, !dbg !34967, !noalias !33771
  %_70.i.sroa.4.0.copyload.pre = load i64, ptr %_70.i.sroa.4.0..sroa_idx, align 16, !dbg !34967, !noalias !33771
  %_71.i.sroa.3.0.copyload.pre = load i64, ptr %_71.i.sroa.3.0..sroa_idx, align 8, !dbg !34968, !noalias !33771
  %_71.i.sroa.4.0.copyload.pre = load i64, ptr %_71.i.sroa.4.0..sroa_idx, align 16, !dbg !34968, !noalias !33771
  %_8.i25.i.sroa.0.0.copyload.pre = load <8 x float>, ptr %_114.i, align 32
  %_9.i24.i.sroa.0.0.copyload.pre = load <8 x float>, ptr %_115.i, align 32
  %_8.i.i.sroa.0.0.copyload.pre = load <8 x float>, ptr %_119.i, align 32
  %_9.i.i.sroa.0.0.copyload.pre = load <8 x float>, ptr %_120.i, align 32
  %_54.0.i278.i.pre = load ptr, ptr %1560, align 32
  %_54.1.i279.i.pre = load i64, ptr %1561, align 8
  %_18.i290.i = load i64, ptr %1556, align 16
  %_56.0.i294.i = load ptr, ptr %1562, align 16, !nonnull !12, !align !24
  %_56.1.i295.i = load i64, ptr %1563, align 8
  %_37.i261.i.sroa.0.0.copyload = load <8 x float>, ptr %1565, align 32
  %_58.1.i307.i = load i64, ptr %1567, align 8
  %_58.0.i306.i = load ptr, ptr %1568, align 32, !nonnull !12, !align !24
  %_54.0.i.i = load ptr, ptr %1571, align 32, !nonnull !12, !align !24
  %_54.1.i.i = load i64, ptr %1572, align 8
  %_18.i.i = load i64, ptr %1557, align 16
  %_56.0.i.i = load ptr, ptr %1573, align 16, !nonnull !12, !align !24
  %_56.1.i.i = load i64, ptr %1574, align 8
  %_37.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1576, align 32
  %_58.1.i.i = load i64, ptr %1578, align 8
  %_58.0.i.i = load ptr, ptr %1579, align 32, !nonnull !12, !align !24
  br label %bb20.i, !dbg !33810

bb20.i:                                           ; preds = %bb20.i.lr.ph, %bb32.i
  %.lcssa1590116064 = phi <8 x float> [ %.lcssa1590116063.lcssa21120, %bb20.i.lr.ph ], [ %.lcssa1590116063, %bb32.i ]
  %storemerge.i1848.lcssa1587116028 = phi i32 [ %storemerge.i1848.lcssa1587116027.lcssa21099, %bb20.i.lr.ph ], [ %storemerge.i1848.lcssa1587116027, %bb32.i ]
  %.lcssa1584415992 = phi <8 x float> [ %.lcssa1584415991.lcssa21097, %bb20.i.lr.ph ], [ %.lcssa1584415991, %bb32.i ]
  %storemerge.i1883.lcssa1581415956 = phi i32 [ %storemerge.i1883.lcssa1581415955.lcssa21076, %bb20.i.lr.ph ], [ %storemerge.i1883.lcssa1581415955, %bb32.i ]
  %frame.sroa.0.0.i15949 = phi i64 [ 0, %bb20.i.lr.ph ], [ %_85.i, %bb32.i ]
  %main_cursor.sroa.0.1.i15948 = phi i64 [ %main_cursor.sroa.0.0.i16100, %bb20.i.lr.ph ], [ %main_cursor.sroa.0.2.i, %bb32.i ]
  %ring_cursor.sroa.0.1.i15947 = phi i64 [ %ring_cursor.sroa.0.0.i16099, %bb20.i.lr.ph ], [ %ring_cursor.sroa.0.2.i, %bb32.i ]
  %minimum.i273.i.sroa.0.015756.lcssa1590715946 = phi <8 x float> [ %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa21037, %bb20.i.lr.ph ], [ %minimum.i273.i.sroa.0.015756.lcssa, %bb32.i ]
  %minimum.i.i.sroa.0.015770.lcssa1592615945 = phi <8 x float> [ %minimum.i.i.sroa.0.015770.lcssa15926.lcssa21056, %bb20.i.lr.ph ], [ %minimum.i.i.sroa.0.015770.lcssa, %bb32.i ]
  %_68.i = sub nuw nsw i64 %..i7763, %frame.sroa.0.0.i15949, !dbg !34969
  %ring.i2505 = load i64, ptr %85, align 8, !dbg !34970, !alias.scope !34972, !noalias !34975, !noundef !12
  %main.i2506 = load i64, ptr %86, align 8, !dbg !34979, !alias.scope !34972, !noalias !34975, !noundef !12
  %_10.i2507 = add i64 %ring_cursor.sroa.0.1.i15947, 1, !dbg !34980
  %_45.not.i2508 = icmp ult i64 %_10.i2507, %ring.i2505, !dbg !34981
  %1794 = select i1 %_45.not.i2508, i64 0, i64 %ring.i2505, !dbg !34981
  %start1.sroa.0.0.i2509 = sub nuw i64 %_10.i2507, %1794, !dbg !34981
  %_12.i2511 = add i64 %_70.i.sroa.3.0.copyload.pre, %ring_cursor.sroa.0.1.i15947, !dbg !34983
  %_46.not.i2512 = icmp ult i64 %_12.i2511, %ring.i2505, !dbg !34984
  %1795 = select i1 %_46.not.i2512, i64 0, i64 %ring.i2505, !dbg !34984
  %left_end.sroa.0.0.i2513 = sub nuw i64 %_12.i2511, %1795, !dbg !34984
  %_15.i2515 = add i64 %_71.i.sroa.3.0.copyload.pre, %ring_cursor.sroa.0.1.i15947, !dbg !34986
  %_47.not.i2516 = icmp ult i64 %_15.i2515, %ring.i2505, !dbg !34987
  %1796 = select i1 %_47.not.i2516, i64 0, i64 %ring.i2505, !dbg !34987
  %right_end.sroa.0.0.i2517 = sub nuw i64 %_15.i2515, %1796, !dbg !34987
  %_18.i2519 = add i64 %_70.i.sroa.4.0.copyload.pre, %ring_cursor.sroa.0.1.i15947, !dbg !34989
  %_48.not.i2520 = icmp ult i64 %_18.i2519, %ring.i2505, !dbg !34990
  %1797 = select i1 %_48.not.i2520, i64 0, i64 %ring.i2505, !dbg !34990
  %left_expiring.sroa.0.0.i2521 = sub nuw i64 %_18.i2519, %1797, !dbg !34990
  %_21.i2523 = add i64 %_71.i.sroa.4.0.copyload.pre, %ring_cursor.sroa.0.1.i15947, !dbg !34992
  %_49.not.i2524 = icmp ult i64 %_21.i2523, %ring.i2505, !dbg !34993
  %1798 = select i1 %_49.not.i2524, i64 0, i64 %ring.i2505, !dbg !34993
  %right_expiring.sroa.0.0.i2525 = sub nuw i64 %_21.i2523, %1798, !dbg !34993
  %_30.i2526 = sub i64 %ring.i2505, %ring_cursor.sroa.0.1.i15947, !dbg !34995
  %..i7884 = tail call noundef i64 @llvm.umin.i64(i64 %_30.i2526, i64 %_68.i), !dbg !34996
  %_31.i2528 = sub i64 %main.i2506, %main_cursor.sroa.0.1.i15948, !dbg !34998
  %..i7885 = tail call noundef i64 @llvm.umin.i64(i64 %_31.i2528, i64 %..i7884), !dbg !34999
  %_32.i2530 = sub i64 %ring.i2505, %start1.sroa.0.0.i2509, !dbg !35001
  %..i7886 = tail call noundef i64 @llvm.umin.i64(i64 %_32.i2530, i64 %..i7885), !dbg !35002
  %_34.i2532 = sub i64 %ring.i2505, %left_end.sroa.0.0.i2513, !dbg !35004
  %..i7887 = tail call noundef i64 @llvm.umin.i64(i64 %_34.i2532, i64 %..i7886), !dbg !35005
  %_36.i2534 = sub i64 %ring.i2505, %right_end.sroa.0.0.i2517, !dbg !35007
  %..i7888 = tail call noundef i64 @llvm.umin.i64(i64 %_36.i2534, i64 %..i7887), !dbg !35008
  %_38.i2536 = sub i64 %ring.i2505, %left_expiring.sroa.0.0.i2521, !dbg !35010
  %..i7889 = tail call noundef i64 @llvm.umin.i64(i64 %_38.i2536, i64 %..i7888), !dbg !35011
  %_40.i2538 = sub i64 %ring.i2505, %right_expiring.sroa.0.0.i2525, !dbg !35013
  %..i7890 = tail call noundef i64 @llvm.umin.i64(i64 %_40.i2538, i64 %..i7889), !dbg !35014
  %_74.i = add i64 %frame.sroa.0.0.i15949, %iter3.sroa.0.0.i16101, !dbg !35016
  %base.i = shl i64 %_74.i, 3, !dbg !35016
  %base.i13449 = add i64 %..i7890, %_74.i, !dbg !35019
  %_78.i = shl i64 %base.i13449, 3, !dbg !35019
  %_206.i = icmp ult i64 %_78.i, %base.i, !dbg !35022
  %_202.not.i = icmp ugt i64 %_78.i, %left_io.1
  %or.cond23.i = or i1 %_206.i, %_202.not.i, !dbg !35022
  br i1 %or.cond23.i, label %bb62.i, label %bb61.i, !dbg !35022, !prof !165

bb62.i:                                           ; preds = %bb20.i
  store <8 x float> %history.i215.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.10.sroa.0.0.lcssa, ptr %history.i215.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.13.sroa.0.0.lcssa, ptr %history.i215.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa21037, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015770.lcssa15926.lcssa21056, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1883.lcssa1581415955.lcssa21076, ptr %_22.i293.i, align 4
  store i32 %storemerge.i1848.lcssa1587116027.lcssa21099, ptr %_22.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i, i64 noundef %_78.i, i64 noundef range(i64 0, 2305843009213693952) %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_debdb702bca99cd8ca93ec127a5c320a) #30, !dbg !35030, !noalias !33742
  unreachable, !dbg !35030

bb61.i:                                           ; preds = %bb20.i
  %_209.i = getelementptr inbounds nuw float, ptr %left_io.0, i64 %base.i, !dbg !35031
  %_210.not.i = icmp ugt i64 %_78.i, %right_io.1, !dbg !35035
  br i1 %_210.not.i, label %bb65.i, label %bb64.i, !dbg !35035, !prof !1406

bb65.i:                                           ; preds = %bb61.i
  store <8 x float> %history.i215.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.10.sroa.0.0.lcssa, ptr %history.i215.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.13.sroa.0.0.lcssa, ptr %history.i215.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa21037, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015770.lcssa15926.lcssa21056, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1883.lcssa1581415955.lcssa21076, ptr %_22.i293.i, align 4
  store i32 %storemerge.i1848.lcssa1587116027.lcssa21099, ptr %_22.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i, i64 noundef %_78.i, i64 noundef range(i64 0, 2305843009213693952) %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_01912ef845d3ed33a157086defa4d008) #30, !dbg !35040, !noalias !33742
  unreachable, !dbg !35040

bb64.i:                                           ; preds = %bb61.i
  %_215.i = getelementptr inbounds nuw float, ptr %right_io.0, i64 %base.i, !dbg !35041
  %_82.i = shl nuw nsw i64 %frame.sroa.0.0.i15949, 3, !dbg !35045
  %_85.i = add nuw nsw i64 %..i7890, %frame.sroa.0.0.i15949, !dbg !35047
  %_217.i = icmp ult i64 %_85.i, 33
  br i1 %_217.i, label %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit7965, label %bb67.i, !dbg !35048, !prof !2740

bb67.i:                                           ; preds = %bb64.i
  store <8 x float> %history.i215.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.10.sroa.0.0.lcssa, ptr %history.i215.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.13.sroa.0.0.lcssa, ptr %history.i215.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa21037, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015770.lcssa15926.lcssa21056, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1883.lcssa1581415955.lcssa21076, ptr %_22.i293.i, align 4
  store i32 %storemerge.i1848.lcssa1587116027.lcssa21099, ptr %_22.i.i, align 4
  %_84.i = shl nuw nsw i64 %_85.i, 3, !dbg !35047
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_82.i, i64 noundef %_84.i, i64 noundef 256, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_0ad51c1dc139f477f06f604c0b5d4a59) #30, !dbg !35056, !noalias !33742
  unreachable, !dbg !35056

_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit7965: ; preds = %bb64.i
  %_224.i = getelementptr inbounds nuw float, ptr %peaks_left.i, i64 %_82.i, !dbg !35057
  %_233.i = getelementptr inbounds nuw float, ptr %peaks_right.i, i64 %_82.i, !dbg !35060
  %_2.i.i.i796815784.not = icmp eq i64 %..i7890, 0, !dbg !35070
  br i1 %_2.i.i.i796815784.not, label %bb32.i, label %bb31.i.preheader, !dbg !35070

bb31.i.preheader:                                 ; preds = %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit7965
  %umin18091 = call i64 @llvm.umin.i64(i64 %_34.i2532, i64 %_36.i2534)
  %umin18092 = call i64 @llvm.umin.i64(i64 %umin18091, i64 %_38.i2536)
  %umin18093 = call i64 @llvm.umin.i64(i64 %umin18092, i64 %_40.i2538)
  %umin18094 = call i64 @llvm.umin.i64(i64 %umin18093, i64 %_32.i2530)
  %umin18095 = call i64 @llvm.umin.i64(i64 %umin18094, i64 %_30.i2526)
  %umin18096 = call i64 @llvm.umin.i64(i64 %umin18095, i64 %_31.i2528)
  %1799 = sub nsw i64 %umin18097, %frame.sroa.0.0.i15949
  %umin18098 = call i64 @llvm.umin.i64(i64 %umin18096, i64 %1799)
  %1800 = and i64 %umin18098, 2305843009213693951
  br label %bb31.i

bb31.i:                                           ; preds = %bb31.i.preheader, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2086
  %_33.i.i.sroa.0.0.copyload15877 = phi <8 x float> [ %.lcssa1590116064, %bb31.i.preheader ], [ %1864, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2086 ]
  %storemerge.i184815849 = phi i32 [ %storemerge.i1848.lcssa1587116028, %bb31.i.preheader ], [ %storemerge.i1848, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2086 ]
  %_33.i264.i.sroa.0.0.copyload15820 = phi <8 x float> [ %.lcssa1584415992, %bb31.i.preheader ], [ %1826, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2086 ]
  %storemerge.i188315792 = phi i32 [ %storemerge.i1883.lcssa1581415956, %bb31.i.preheader ], [ %storemerge.i1883, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2086 ]
  %iter.i.sroa.36.015789 = phi i64 [ 0, %bb31.i.preheader ], [ %1801, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2086 ]
  %minimum.i273.i.sroa.0.01575615787 = phi <8 x float> [ %minimum.i273.i.sroa.0.015756.lcssa1590715946, %bb31.i.preheader ], [ %minimum.i273.i.sroa.0.0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2086 ]
  %minimum.i.i.sroa.0.01577015785 = phi <8 x float> [ %minimum.i.i.sroa.0.015770.lcssa1592615945, %bb31.i.preheader ], [ %minimum.i.i.sroa.0.0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2086 ]
  %start1.i.i.i.i.i.i.i.i7980 = shl i64 %iter.i.sroa.36.015789, 3, !dbg !35076
  %data.i.i.i.i.i.i7991 = getelementptr inbounds nuw float, ptr %_224.i, i64 %start1.i.i.i.i.i.i.i.i7980, !dbg !35082
  %lanes.i6128.sroa.0.0.copyload = load <8 x float>, ptr %data.i.i.i.i.i.i7991, align 4, !dbg !35085, !alias.scope !35093, !noalias !35097
  %data.i.i.i.i7986 = getelementptr inbounds nuw float, ptr %_233.i, i64 %start1.i.i.i.i.i.i.i.i7980, !dbg !35101
  %lanes.i6119.sroa.0.0.copyload = load <8 x float>, ptr %data.i.i.i.i7986, align 4, !dbg !35104, !alias.scope !35110, !noalias !35114
  %data.i.i.i.i.i.i.i.i7981 = getelementptr inbounds nuw float, ptr %_209.i, i64 %start1.i.i.i.i.i.i.i.i7980, !dbg !35118
  %data.i5.i.i.i.i.i.i.i7995 = getelementptr inbounds nuw float, ptr %_215.i, i64 %start1.i.i.i.i.i.i.i.i7980, !dbg !35120
  %1801 = add nuw nsw i64 %iter.i.sroa.36.015789, 1, !dbg !35123
  %1802 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i6119.sroa.0.0.copyload, <8 x float> %lanes.i6128.sroa.0.0.copyload), !dbg !35124
  %1803 = select <8 x i1> %1559, <8 x float> %1802, <8 x float> %lanes.i6119.sroa.0.0.copyload, !dbg !35130
  %_235.i = add i64 %iter.i.sroa.36.015789, %ring_cursor.sroa.0.1.i15947, !dbg !35137
  %_236.i = add i64 %iter.i.sroa.36.015789, %main_cursor.sroa.0.1.i15948, !dbg !35143
  %_237.i = add i64 %iter.i.sroa.36.015789, %left_end.sroa.0.0.i2513, !dbg !35144
  %_238.i = add i64 %iter.i.sroa.36.015789, %start1.sroa.0.0.i2509, !dbg !35145
  %_239.i = add i64 %iter.i.sroa.36.015789, %left_expiring.sroa.0.0.i2521, !dbg !35146
  %base.i9.i281.i = shl i64 %_235.i, 3, !dbg !35147
  %_7.i10.i282.i = add i64 %base.i9.i281.i, 8, !dbg !35150
  %1804 = or disjoint i64 %base.i9.i281.i, 7, !dbg !35151
  %or.cond.i13.i285.i.not = icmp ult i64 %1804, %_54.1.i279.i.pre, !dbg !35151
  br i1 %or.cond.i13.i285.i.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i286.i, label %bb4.i15.i319.i, !dbg !35151, !prof !2740

bb4.i15.i319.i:                                   ; preds = %bb31.i
  store <8 x float> %history.i215.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.10.sroa.0.0.lcssa, ptr %history.i215.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.13.sroa.0.0.lcssa, ptr %history.i215.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa21037, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015770.lcssa15926.lcssa21056, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1883.lcssa1581415955.lcssa21076, ptr %_22.i293.i, align 4
  store i32 %storemerge.i1848.lcssa1587116027.lcssa21099, ptr %_22.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i281.i, i64 noundef %_7.i10.i282.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i279.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !35155, !noalias !35156
  unreachable, !dbg !35155

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i286.i: ; preds = %bb31.i
  %1805 = select <8 x i1> %1559, <8 x float> %1802, <8 x float> %lanes.i6128.sroa.0.0.copyload, !dbg !35170
  %1806 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1805, <8 x float> %_8.i25.i.sroa.0.0.copyload.pre, i8 30), !dbg !35175
  %1807 = bitcast <8 x float> %1806 to <8 x i32>, !dbg !35181
  %1808 = icmp slt <8 x i32> %1807, zeroinitializer, !dbg !35185
  %1809 = fdiv <8 x float> %_8.i25.i.sroa.0.0.copyload.pre, %1805, !dbg !35187
  %1810 = select <8 x i1> %1808, <8 x float> %1809, <8 x float> splat (float 1.000000e+00), !dbg !35185
  %_17.i14.i287.i = getelementptr inbounds nuw float, ptr %_54.0.i278.i.pre, i64 %base.i9.i281.i, !dbg !35192
  store <8 x float> %1810, ptr %_17.i14.i287.i, align 4, !dbg !35194, !alias.scope !35199, !noalias !35203
  %base.i1925 = shl i64 %_237.i, 3, !dbg !35207
  %1811 = or disjoint i64 %base.i1925, 7, !dbg !35210
  %or.cond.i1929.not = icmp ult i64 %1811, %_54.1.i279.i.pre, !dbg !35210
  br i1 %or.cond.i1929.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1933, label %bb4.i1932, !dbg !35210, !prof !2740

bb4.i1932:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i286.i
  store <8 x float> %history.i215.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.10.sroa.0.0.lcssa, ptr %history.i215.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.13.sroa.0.0.lcssa, ptr %history.i215.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa21037, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015770.lcssa15926.lcssa21056, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1883.lcssa1581415955.lcssa21076, ptr %_22.i293.i, align 4
  store i32 %storemerge.i1848.lcssa1587116027.lcssa21099, ptr %_22.i.i, align 4
  %_5.i1926 = add i64 %base.i1925, 8, !dbg !35214
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1925, i64 noundef %_5.i1926, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i279.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !35215, !noalias !35216
  unreachable, !dbg !35215

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1933: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i286.i
  %_15.i1931 = getelementptr inbounds nuw float, ptr %_54.0.i278.i.pre, i64 %base.i1925, !dbg !35224
  %lanes.i5819.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1931, align 4, !dbg !35226, !alias.scope !35231, !noalias !35235
  %position.i1877 = zext i32 %storemerge.i188315792 to i64, !dbg !35239
  %1812 = icmp eq i32 %storemerge.i188315792, 0, !dbg !35240
  br i1 %1812, label %bb5.i1879, label %bb3.i1878, !dbg !35240

bb3.i1878:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1933
  %1813 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %minimum.i273.i.sroa.0.01575615787, <8 x float> %lanes.i5819.sroa.0.0.copyload), !dbg !35241
  br label %bb5.i1879, !dbg !35246

bb5.i1879:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1933, %bb3.i1878
  %minimum.i273.i.sroa.0.0 = phi <8 x float> [ %1813, %bb3.i1878 ], [ %lanes.i5819.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1933 ], !dbg !35247
  %_15.i1880 = add nuw nsw i64 %position.i1877, 1, !dbg !35248
  %complete.i1881 = icmp eq i64 %_15.i1880, %_18.i290.i, !dbg !35248
  br i1 %complete.i1881, label %bb19.i1889, label %bb7.i1882, !dbg !35249

bb7.i1882:                                        ; preds = %bb5.i1879
  %base.i1916 = shl i64 %_238.i, 3, !dbg !35250
  %1814 = or disjoint i64 %base.i1916, 7, !dbg !35252
  %or.cond.i1920.not = icmp ult i64 %1814, %_54.1.i279.i.pre, !dbg !35252
  br i1 %or.cond.i1920.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1924, label %bb4.i1923, !dbg !35252, !prof !2740

bb4.i1923:                                        ; preds = %bb7.i1882
  store <8 x float> %history.i215.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.10.sroa.0.0.lcssa, ptr %history.i215.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.13.sroa.0.0.lcssa, ptr %history.i215.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa21037, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015770.lcssa15926.lcssa21056, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1883.lcssa1581415955.lcssa21076, ptr %_22.i293.i, align 4
  store i32 %storemerge.i1848.lcssa1587116027.lcssa21099, ptr %_22.i.i, align 4
  %_5.i1917 = add i64 %base.i1916, 8, !dbg !35256
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1916, i64 noundef %_5.i1917, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i279.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !35257, !noalias !35258
  unreachable, !dbg !35257

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1924: ; preds = %bb7.i1882
  %_15.i1922 = getelementptr inbounds nuw float, ptr %_54.0.i278.i.pre, i64 %base.i1916, !dbg !35262
  %lanes.i5826.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1922, align 4, !dbg !35264, !alias.scope !35269, !noalias !35273
  %1815 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %lanes.i5826.sroa.0.0.copyload, <8 x float> %minimum.i273.i.sroa.0.0), !dbg !35277
  %1816 = trunc i64 %_15.i1880 to i32, !dbg !35282
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1901, !dbg !35283

bb19.i1889:                                       ; preds = %bb5.i1879, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit
  %end.sroa.0.0.i188715751 = phi i64 [ %1820, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit ], [ %_237.i, %bb5.i1879 ]
  %iter.sroa.0.0.i188615750 = phi i64 [ %_30.i1890, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit ], [ 0, %bb5.i1879 ]
  %suffix.i1870.sroa.0.015749 = phi <8 x float> [ %1818, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit ], [ %lanes.i5819.sroa.0.0.copyload, %bb5.i1879 ]
  %base.i1902 = shl i64 %end.sroa.0.0.i188715751, 3, !dbg !35284
  %1817 = or disjoint i64 %base.i1902, 7, !dbg !35286
  %or.cond.i1904.not = icmp ult i64 %1817, %_54.1.i279.i.pre, !dbg !35286
  br i1 %or.cond.i1904.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, label %bb4.i, !dbg !35286, !prof !2740

bb4.i:                                            ; preds = %bb19.i1889
  store <8 x float> %history.i215.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.10.sroa.0.0.lcssa, ptr %history.i215.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.13.sroa.0.0.lcssa, ptr %history.i215.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa21037, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015770.lcssa15926.lcssa21056, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1883.lcssa1581415955.lcssa21076, ptr %_22.i293.i, align 4
  store i32 %storemerge.i1848.lcssa1587116027.lcssa21099, ptr %_22.i.i, align 4
  %_5.i = add i64 %base.i1902, 8, !dbg !35290
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1902, i64 noundef %_5.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i279.i.pre, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !35291, !noalias !35292
  unreachable, !dbg !35291

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit: ; preds = %bb19.i1889
  %_30.i1890 = add nuw i64 %iter.sroa.0.0.i188615750, 1, !dbg !35296
  %_15.i1906 = getelementptr inbounds nuw float, ptr %_54.0.i278.i.pre, i64 %base.i1902, !dbg !35301
  %lanes.i5840.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1906, align 4, !dbg !35303, !alias.scope !35308, !noalias !35312
  %1818 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %suffix.i1870.sroa.0.015749, <8 x float> %lanes.i5840.sroa.0.0.copyload), !dbg !35316
  store <8 x float> %1818, ptr %_15.i1906, align 4, !dbg !35321, !alias.scope !35327, !noalias !35331
  %1819 = icmp eq i64 %end.sroa.0.0.i188715751, 0, !dbg !35335
  %spec.store.select.i1898 = select i1 %1819, i64 %ring.i, i64 %end.sroa.0.0.i188715751, !dbg !35335
  %1820 = add i64 %spec.store.select.i1898, -1, !dbg !35336
  %exitcond18080.not = icmp eq i64 %_30.i1890, %_18.i290.i, !dbg !35337
  br i1 %exitcond18080.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1901, label %bb19.i1889, !dbg !35339

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1901: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1924
  %minimum.i273.i.sroa.0.1 = phi <8 x float> [ %1815, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1924 ], [ %minimum.i273.i.sroa.0.0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit ], !dbg !35247
  %storemerge.i1883 = phi i32 [ %1816, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1924 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit ], !dbg !35340
  %1821 = fmul <8 x float> %minimum.i273.i.sroa.0.1, splat (float 1.638400e+04), !dbg !35341
  %1822 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %1821), !dbg !35346
  %1823 = fmul <8 x float> %1822, splat (float 0x3F10000000000000), !dbg !35351
  %base.i2105 = shl i64 %_239.i, 3, !dbg !35356
  %1824 = or disjoint i64 %base.i2105, 7, !dbg !35358
  %or.cond.i2109.not = icmp ult i64 %1824, %_56.1.i295.i, !dbg !35358
  br i1 %or.cond.i2109.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2113, label %bb4.i2112, !dbg !35358, !prof !2740

bb4.i2112:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1901
  store <8 x float> %history.i215.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.10.sroa.0.0.lcssa, ptr %history.i215.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.13.sroa.0.0.lcssa, ptr %history.i215.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa21037, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015770.lcssa15926.lcssa21056, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1883.lcssa1581415955.lcssa21076, ptr %_22.i293.i, align 4
  store i32 %storemerge.i1848.lcssa1587116027.lcssa21099, ptr %_22.i.i, align 4
  %_5.i2106 = add i64 %base.i2105, 8, !dbg !35362
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i2105, i64 noundef %_5.i2106, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i295.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !35363, !noalias !35364
  unreachable, !dbg !35363

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2113: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1901
  %_15.i2111 = getelementptr inbounds nuw float, ptr %_56.0.i294.i, i64 %base.i2105, !dbg !35368
  %lanes.i.sroa.0.0.copyload = load <8 x float>, ptr %_15.i2111, align 4, !dbg !35370, !alias.scope !35375, !noalias !35379
  %1825 = fadd <8 x float> %1823, %_33.i264.i.sroa.0.0.copyload15820, !dbg !35383
  %1826 = fsub <8 x float> %1825, %lanes.i.sroa.0.0.copyload, !dbg !35388
  %_8.not.i4.i302.i = icmp ugt i64 %_7.i10.i282.i, %_56.1.i295.i
  br i1 %_8.not.i4.i302.i, label %bb4.i7.i318.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i304.i, !dbg !35393, !prof !165

bb4.i7.i318.i:                                    ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2113
  store <8 x float> %history.i215.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.10.sroa.0.0.lcssa, ptr %history.i215.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.13.sroa.0.0.lcssa, ptr %history.i215.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa21037, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015770.lcssa15926.lcssa21056, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1883.lcssa1581415955.lcssa21076, ptr %_22.i293.i, align 4
  store i32 %storemerge.i1848.lcssa1587116027.lcssa21099, ptr %_22.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i281.i, i64 noundef %_7.i10.i282.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i295.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !35398, !noalias !35399
  unreachable, !dbg !35398

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i304.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2113
  %_17.i6.i305.i = getelementptr inbounds nuw float, ptr %_56.0.i294.i, i64 %base.i9.i281.i, !dbg !35403
  store <8 x float> %1823, ptr %_17.i6.i305.i, align 4, !dbg !35405, !alias.scope !35410, !noalias !35414
  %_41.i257.i.sroa.0.0.copyload = load <8 x float>, ptr %1566, align 32, !dbg !35418
  %1827 = fdiv <8 x float> %1826, %_37.i261.i.sroa.0.0.copyload, !dbg !35419
  %1828 = fsub <8 x float> splat (float 1.000000e+00), %1827, !dbg !35424
  %1829 = fsub <8 x float> %1828, %_41.i257.i.sroa.0.0.copyload, !dbg !35429
  %1830 = fmul <8 x float> %_9.i24.i.sroa.0.0.copyload.pre, %1829, !dbg !35434
  %1831 = fadd <8 x float> %_41.i257.i.sroa.0.0.copyload, %1830, !dbg !35439
  %1832 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1828, <8 x float> %1831), !dbg !35443
  %1833 = bitcast <8 x float> %1832 to <8 x i32>, !dbg !35448
  %1834 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1832), !dbg !35454
  %1835 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1834, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !35456
  %1836 = bitcast <8 x float> %1835 to <8 x i32>, !dbg !35462
  %1837 = xor <8 x i32> %1836, splat (i32 -1), !dbg !35468
  %1838 = and <8 x i32> %1837, %1833, !dbg !35470
  store <8 x i32> %1838, ptr %1566, align 32, !dbg !35474
  %base.i2096 = shl i64 %_236.i, 3, !dbg !35475
  %_5.i2097 = add i64 %base.i2096, 8, !dbg !35477
  %1839 = or disjoint i64 %base.i2096, 7, !dbg !35478
  %or.cond.i2100.not = icmp ult i64 %1839, %_58.1.i307.i, !dbg !35478
  br i1 %or.cond.i2100.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2104, label %bb4.i2103, !dbg !35478, !prof !2740

bb4.i2103:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i304.i
  store <8 x float> %history.i215.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.10.sroa.0.0.lcssa, ptr %history.i215.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.13.sroa.0.0.lcssa, ptr %history.i215.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa21037, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015770.lcssa15926.lcssa21056, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1883.lcssa1581415955.lcssa21076, ptr %_22.i293.i, align 4
  store i32 %storemerge.i1848.lcssa1587116027.lcssa21099, ptr %_22.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i2096, i64 noundef %_5.i2097, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i307.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !35482, !noalias !35483
  unreachable, !dbg !35482

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2104: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i304.i
  %1840 = bitcast <8 x i32> %1838 to <8 x float>, !dbg !35487
  %1841 = fsub <8 x float> splat (float 1.000000e+00), %1840, !dbg !35488
  %_15.i2102 = getelementptr inbounds nuw float, ptr %_58.0.i306.i, i64 %base.i2096, !dbg !35493
  %lanes.i5686.sroa.0.0.copyload = load <8 x float>, ptr %_15.i2102, align 4, !dbg !35495, !alias.scope !35500, !noalias !35504
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_15.i2102, ptr noundef nonnull align 4 dereferenceable(32) %data.i.i.i.i.i.i.i.i7981, i64 32, i1 false), !dbg !35508
  %1842 = fmul <8 x float> %1841, %lanes.i5686.sroa.0.0.copyload, !dbg !35514
  %1843 = select <8 x i1> %1570, <8 x float> %lanes.i5686.sroa.0.0.copyload, <8 x float> %1842, !dbg !35519
  store <8 x float> %1843, ptr %data.i.i.i.i.i.i.i.i7981, align 4, !dbg !35524, !alias.scope !35529, !noalias !35533
  %_242.i = add i64 %iter.i.sroa.36.015789, %right_end.sroa.0.0.i2517, !dbg !35537
  %_244.i = add i64 %iter.i.sroa.36.015789, %right_expiring.sroa.0.0.i2525, !dbg !35539
  %_8.not.i12.i.i = icmp ugt i64 %_7.i10.i282.i, %_54.1.i.i
  br i1 %_8.not.i12.i.i, label %bb4.i15.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i, !dbg !35540, !prof !165

bb4.i15.i.i:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2104
  store <8 x float> %history.i215.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.10.sroa.0.0.lcssa, ptr %history.i215.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.13.sroa.0.0.lcssa, ptr %history.i215.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa21037, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015770.lcssa15926.lcssa21056, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1883.lcssa1581415955.lcssa21076, ptr %_22.i293.i, align 4
  store i32 %storemerge.i1848.lcssa1587116027.lcssa21099, ptr %_22.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i281.i, i64 noundef %_7.i10.i282.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !35546, !noalias !35547
  unreachable, !dbg !35546

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2104
  %1844 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1803, <8 x float> %_8.i.i.sroa.0.0.copyload.pre, i8 30), !dbg !35561
  %1845 = bitcast <8 x float> %1844 to <8 x i32>, !dbg !35567
  %1846 = icmp slt <8 x i32> %1845, zeroinitializer, !dbg !35571
  %1847 = fdiv <8 x float> %_8.i.i.sroa.0.0.copyload.pre, %1803, !dbg !35573
  %1848 = select <8 x i1> %1846, <8 x float> %1847, <8 x float> splat (float 1.000000e+00), !dbg !35571
  %_17.i14.i.i = getelementptr inbounds nuw float, ptr %_54.0.i.i, i64 %base.i9.i281.i, !dbg !35578
  store <8 x float> %1848, ptr %_17.i14.i.i, align 4, !dbg !35580, !alias.scope !35585, !noalias !35589
  %base.i1961 = shl i64 %_242.i, 3, !dbg !35593
  %1849 = or disjoint i64 %base.i1961, 7, !dbg !35596
  %or.cond.i1965.not = icmp ult i64 %1849, %_54.1.i.i, !dbg !35596
  br i1 %or.cond.i1965.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1969, label %bb4.i1968, !dbg !35596, !prof !2740

bb4.i1968:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i
  store <8 x float> %history.i215.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.10.sroa.0.0.lcssa, ptr %history.i215.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.13.sroa.0.0.lcssa, ptr %history.i215.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa21037, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015770.lcssa15926.lcssa21056, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1883.lcssa1581415955.lcssa21076, ptr %_22.i293.i, align 4
  store i32 %storemerge.i1848.lcssa1587116027.lcssa21099, ptr %_22.i.i, align 4
  %_5.i1962 = add i64 %base.i1961, 8, !dbg !35600
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1961, i64 noundef %_5.i1962, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !35601, !noalias !35602
  unreachable, !dbg !35601

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1969: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i
  %_15.i1967 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i64 %base.i1961, !dbg !35610
  %lanes.i5791.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1967, align 4, !dbg !35612, !alias.scope !35617, !noalias !35621
  %position.i1842 = zext i32 %storemerge.i184815849 to i64, !dbg !35625
  %1850 = icmp eq i32 %storemerge.i184815849, 0, !dbg !35626
  br i1 %1850, label %bb5.i1844, label %bb3.i1843, !dbg !35626

bb3.i1843:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1969
  %1851 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %minimum.i.i.sroa.0.01577015785, <8 x float> %lanes.i5791.sroa.0.0.copyload), !dbg !35627
  br label %bb5.i1844, !dbg !35632

bb5.i1844:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1969, %bb3.i1843
  %minimum.i.i.sroa.0.0 = phi <8 x float> [ %1851, %bb3.i1843 ], [ %lanes.i5791.sroa.0.0.copyload, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1969 ], !dbg !35633
  %_15.i1845 = add nuw nsw i64 %position.i1842, 1, !dbg !35634
  %complete.i1846 = icmp eq i64 %_15.i1845, %_18.i.i, !dbg !35634
  br i1 %complete.i1846, label %bb19.i1854, label %bb7.i1847, !dbg !35635

bb7.i1847:                                        ; preds = %bb5.i1844
  %base.i1952 = shl i64 %_238.i, 3, !dbg !35636
  %1852 = or disjoint i64 %base.i1952, 7, !dbg !35638
  %or.cond.i1956.not = icmp ult i64 %1852, %_54.1.i.i, !dbg !35638
  br i1 %or.cond.i1956.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1960, label %bb4.i1959, !dbg !35638, !prof !2740

bb4.i1959:                                        ; preds = %bb7.i1847
  store <8 x float> %history.i215.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.10.sroa.0.0.lcssa, ptr %history.i215.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.13.sroa.0.0.lcssa, ptr %history.i215.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa21037, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015770.lcssa15926.lcssa21056, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1883.lcssa1581415955.lcssa21076, ptr %_22.i293.i, align 4
  store i32 %storemerge.i1848.lcssa1587116027.lcssa21099, ptr %_22.i.i, align 4
  %_5.i1953 = add i64 %base.i1952, 8, !dbg !35642
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1952, i64 noundef %_5.i1953, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !35643, !noalias !35644
  unreachable, !dbg !35643

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1960: ; preds = %bb7.i1847
  %_15.i1958 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i64 %base.i1952, !dbg !35648
  %lanes.i5798.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1958, align 4, !dbg !35650, !alias.scope !35655, !noalias !35659
  %1853 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %lanes.i5798.sroa.0.0.copyload, <8 x float> %minimum.i.i.sroa.0.0), !dbg !35663
  %1854 = trunc i64 %_15.i1845 to i32, !dbg !35668
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1866, !dbg !35669

bb19.i1854:                                       ; preds = %bb5.i1844, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1942
  %end.sroa.0.0.i185215755 = phi i64 [ %1858, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1942 ], [ %_242.i, %bb5.i1844 ]
  %iter.sroa.0.0.i185115754 = phi i64 [ %_30.i1855, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1942 ], [ 0, %bb5.i1844 ]
  %suffix.i1835.sroa.0.015753 = phi <8 x float> [ %1856, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1942 ], [ %lanes.i5791.sroa.0.0.copyload, %bb5.i1844 ]
  %base.i1934 = shl i64 %end.sroa.0.0.i185215755, 3, !dbg !35670
  %1855 = or disjoint i64 %base.i1934, 7, !dbg !35672
  %or.cond.i1938.not = icmp ult i64 %1855, %_54.1.i.i, !dbg !35672
  br i1 %or.cond.i1938.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1942, label %bb4.i1941, !dbg !35672, !prof !2740

bb4.i1941:                                        ; preds = %bb19.i1854
  store <8 x float> %history.i215.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.10.sroa.0.0.lcssa, ptr %history.i215.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.13.sroa.0.0.lcssa, ptr %history.i215.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa21037, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015770.lcssa15926.lcssa21056, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1883.lcssa1581415955.lcssa21076, ptr %_22.i293.i, align 4
  store i32 %storemerge.i1848.lcssa1587116027.lcssa21099, ptr %_22.i.i, align 4
  %_5.i1935 = add i64 %base.i1934, 8, !dbg !35676
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1934, i64 noundef %_5.i1935, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !35677, !noalias !35678
  unreachable, !dbg !35677

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1942: ; preds = %bb19.i1854
  %_30.i1855 = add nuw i64 %iter.sroa.0.0.i185115754, 1, !dbg !35682
  %_15.i1940 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i64 %base.i1934, !dbg !35687
  %lanes.i5812.sroa.0.0.copyload = load <8 x float>, ptr %_15.i1940, align 4, !dbg !35689, !alias.scope !35694, !noalias !35698
  %1856 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %suffix.i1835.sroa.0.015753, <8 x float> %lanes.i5812.sroa.0.0.copyload), !dbg !35702
  store <8 x float> %1856, ptr %_15.i1940, align 4, !dbg !35707, !alias.scope !35713, !noalias !35717
  %1857 = icmp eq i64 %end.sroa.0.0.i185215755, 0, !dbg !35721
  %spec.store.select.i1863 = select i1 %1857, i64 %ring.i, i64 %end.sroa.0.0.i185215755, !dbg !35721
  %1858 = add i64 %spec.store.select.i1863, -1, !dbg !35722
  %exitcond18086.not = icmp eq i64 %_30.i1855, %_18.i.i, !dbg !35723
  br i1 %exitcond18086.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1866, label %bb19.i1854, !dbg !35725

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1866: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1942, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1960
  %minimum.i.i.sroa.0.1 = phi <8 x float> [ %1853, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1960 ], [ %minimum.i.i.sroa.0.0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1942 ], !dbg !35633
  %storemerge.i1848 = phi i32 [ %1854, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1960 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1942 ], !dbg !35726
  %1859 = fmul <8 x float> %minimum.i.i.sroa.0.1, splat (float 1.638400e+04), !dbg !35727
  %1860 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %1859), !dbg !35732
  %1861 = fmul <8 x float> %1860, splat (float 0x3F10000000000000), !dbg !35737
  %base.i2087 = shl i64 %_244.i, 3, !dbg !35742
  %1862 = or disjoint i64 %base.i2087, 7, !dbg !35744
  %or.cond.i2091.not = icmp ult i64 %1862, %_56.1.i.i, !dbg !35744
  br i1 %or.cond.i2091.not, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2095, label %bb4.i2094, !dbg !35744, !prof !2740

bb4.i2094:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1866
  store <8 x float> %history.i215.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.10.sroa.0.0.lcssa, ptr %history.i215.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.13.sroa.0.0.lcssa, ptr %history.i215.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa21037, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015770.lcssa15926.lcssa21056, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1883.lcssa1581415955.lcssa21076, ptr %_22.i293.i, align 4
  store i32 %storemerge.i1848.lcssa1587116027.lcssa21099, ptr %_22.i.i, align 4
  %_5.i2088 = add i64 %base.i2087, 8, !dbg !35748
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i2087, i64 noundef %_5.i2088, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !35749, !noalias !35750
  unreachable, !dbg !35749

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2095: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1866
  %_15.i2093 = getelementptr inbounds nuw float, ptr %_56.0.i.i, i64 %base.i2087, !dbg !35754
  %lanes.i5693.sroa.0.0.copyload = load <8 x float>, ptr %_15.i2093, align 4, !dbg !35756, !alias.scope !35761, !noalias !35765
  %1863 = fadd <8 x float> %1861, %_33.i.i.sroa.0.0.copyload15877, !dbg !35769
  %1864 = fsub <8 x float> %1863, %lanes.i5693.sroa.0.0.copyload, !dbg !35774
  %_8.not.i4.i.i = icmp ugt i64 %_7.i10.i282.i, %_56.1.i.i
  br i1 %_8.not.i4.i.i, label %bb4.i7.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i, !dbg !35779, !prof !165

bb4.i7.i.i:                                       ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2095
  store <8 x float> %history.i215.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.10.sroa.0.0.lcssa, ptr %history.i215.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.13.sroa.0.0.lcssa, ptr %history.i215.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa21037, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015770.lcssa15926.lcssa21056, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1883.lcssa1581415955.lcssa21076, ptr %_22.i293.i, align 4
  store i32 %storemerge.i1848.lcssa1587116027.lcssa21099, ptr %_22.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i281.i, i64 noundef %_7.i10.i282.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aac110d217bd6a4c15b81b08a4ef3ef7) #30, !dbg !35784, !noalias !35785
  unreachable, !dbg !35784

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2095
  %_17.i6.i.i = getelementptr inbounds nuw float, ptr %_56.0.i.i, i64 %base.i9.i281.i, !dbg !35789
  store <8 x float> %1861, ptr %_17.i6.i.i, align 4, !dbg !35791, !alias.scope !35796, !noalias !35800
  %_41.i.i.sroa.0.0.copyload = load <8 x float>, ptr %1577, align 32, !dbg !35804
  %1865 = fdiv <8 x float> %1864, %_37.i.i.sroa.0.0.copyload, !dbg !35805
  %1866 = fsub <8 x float> splat (float 1.000000e+00), %1865, !dbg !35810
  %1867 = fsub <8 x float> %1866, %_41.i.i.sroa.0.0.copyload, !dbg !35815
  %1868 = fmul <8 x float> %_9.i.i.sroa.0.0.copyload.pre, %1867, !dbg !35820
  %1869 = fadd <8 x float> %_41.i.i.sroa.0.0.copyload, %1868, !dbg !35825
  %1870 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1866, <8 x float> %1869), !dbg !35829
  %1871 = bitcast <8 x float> %1870 to <8 x i32>, !dbg !35834
  %1872 = call <8 x float> @llvm.fabs.v8f32(<8 x float> %1870), !dbg !35840
  %1873 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1872, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !35842
  %1874 = bitcast <8 x float> %1873 to <8 x i32>, !dbg !35848
  %1875 = xor <8 x i32> %1874, splat (i32 -1), !dbg !35854
  %1876 = and <8 x i32> %1875, %1871, !dbg !35856
  store <8 x i32> %1876, ptr %1577, align 32, !dbg !35860
  %_6.not.i2081 = icmp ugt i64 %_5.i2097, %_58.1.i.i
  br i1 %_6.not.i2081, label %bb4.i2085, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2086, !dbg !35861, !prof !165

bb4.i2085:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i
  store <8 x float> %history.i215.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.10.sroa.0.0.lcssa, ptr %history.i215.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.13.sroa.0.0.lcssa, ptr %history.i215.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa21037, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015770.lcssa15926.lcssa21056, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1883.lcssa1581415955.lcssa21076, ptr %_22.i293.i, align 4
  store i32 %storemerge.i1848.lcssa1587116027.lcssa21099, ptr %_22.i.i, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i2096, i64 noundef %_5.i2097, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f20c129b493c5309be2c04d6b670f551) #30, !dbg !35866, !noalias !35867
  unreachable, !dbg !35866

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2086: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i
  %1877 = bitcast <8 x i32> %1876 to <8 x float>, !dbg !35871
  %1878 = fsub <8 x float> splat (float 1.000000e+00), %1877, !dbg !35872
  %_15.i2084 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %base.i2096, !dbg !35877
  %lanes.i5700.sroa.0.0.copyload = load <8 x float>, ptr %_15.i2084, align 4, !dbg !35879, !alias.scope !35884, !noalias !35888
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_15.i2084, ptr noundef nonnull align 4 dereferenceable(32) %data.i5.i.i.i.i.i.i.i7995, i64 32, i1 false), !dbg !35892
  %1879 = fmul <8 x float> %1878, %lanes.i5700.sroa.0.0.copyload, !dbg !35898
  %1880 = select <8 x i1> %1570, <8 x float> %lanes.i5700.sroa.0.0.copyload, <8 x float> %1879, !dbg !35903
  store <8 x float> %1880, ptr %data.i5.i.i.i.i.i.i.i7995, align 4, !dbg !35908, !alias.scope !35913, !noalias !35917
  %exitcond18099.not = icmp eq i64 %1801, %1800, !dbg !35070
  br i1 %exitcond18099.not, label %bb32.i, label %bb31.i, !dbg !35070

bb32.i:                                           ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2086, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit7965
  %.lcssa1590116063 = phi <8 x float> [ %.lcssa1590116064, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit7965 ], [ %1864, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2086 ]
  %storemerge.i1848.lcssa1587116027 = phi i32 [ %storemerge.i1848.lcssa1587116028, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit7965 ], [ %storemerge.i1848, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2086 ]
  %.lcssa1584415991 = phi <8 x float> [ %.lcssa1584415992, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit7965 ], [ %1826, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2086 ]
  %storemerge.i1883.lcssa1581415955 = phi i32 [ %storemerge.i1883.lcssa1581415956, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit7965 ], [ %storemerge.i1883, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2086 ]
  %minimum.i.i.sroa.0.015770.lcssa = phi <8 x float> [ %minimum.i.i.sroa.0.015770.lcssa1592615945, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit7965 ], [ %minimum.i.i.sroa.0.0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2086 ]
  %minimum.i273.i.sroa.0.015756.lcssa = phi <8 x float> [ %minimum.i273.i.sroa.0.015756.lcssa1590715946, %_RINvYINtNtNtNtCs4NRVxsYgnAr_4core4iter8adapters3zip3ZipIB4_INtNtNtBc_5slice4iter14ChunksExactMutfEBV_EINtBY_11ChunksExactfEENtNtNtBa_6traits8iterator8Iterator3zipB1C_ECsdvPQf9CMsz3_17true_peak_limiter.exit7965 ], [ %minimum.i273.i.sroa.0.0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit2086 ]
  %_143.i = add i64 %..i7890, %ring_cursor.sroa.0.1.i15947, !dbg !35921
  %_234.not.i = icmp ult i64 %_143.i, %ring.i, !dbg !35922
  %1881 = select i1 %_234.not.i, i64 0, i64 %ring.i, !dbg !35922
  %ring_cursor.sroa.0.2.i = sub nuw i64 %_143.i, %1881, !dbg !35922
  %_145.i = add i64 %..i7890, %main_cursor.sroa.0.1.i15948, !dbg !35925
  %_245.not.i = icmp ult i64 %_145.i, %main.i, !dbg !35926
  %1882 = select i1 %_245.not.i, i64 0, i64 %main.i, !dbg !35926
  %main_cursor.sroa.0.2.i = sub nuw i64 %_145.i, %1882, !dbg !35926
  %_63.i = icmp ult i64 %_85.i, %..i7763, !dbg !33810
  br i1 %_63.i, label %bb20.i, label %bb19.i.bb15.i.loopexit_crit_edge, !dbg !33810

_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit: ; preds = %bb15.i.loopexit
  store <8 x float> %history.i215.i.sroa.0.0.lcssa, ptr %hot_left.i, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.10.sroa.0.0.lcssa, ptr %history.i215.i.sroa.10.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i215.i.sroa.13.sroa.0.0.lcssa, ptr %history.i215.i.sroa.13.0.hot_left.i.sroa_idx, align 32, !dbg !33837
  store <8 x float> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 32, !dbg !33839
  store <8 x float> %minimum.i273.i.sroa.0.015756.lcssa15907.lcssa, ptr %uniform_left.i, align 1
  store <8 x float> %minimum.i.i.sroa.0.015770.lcssa15926.lcssa, ptr %uniform_right.i, align 1
  store i32 %storemerge.i1883.lcssa1581415955.lcssa21075, ptr %_22.i293.i, align 4
  store i32 %storemerge.i1848.lcssa1587116027.lcssa21098, ptr %_22.i.i, align 4
  %1883 = trunc i64 %main_cursor.sroa.0.1.i.lcssa to i32, !dbg !35928
  %1884 = trunc i64 %ring_cursor.sroa.0.1.i.lcssa to i32, !dbg !35930
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, !dbg !35931

_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit, %bb6.i
  %ring_cursor.sroa.0.0.i.lcssa = phi i32 [ %_36.i, %bb6.i ], [ %1884, %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit ], !dbg !33767
  %main_cursor.sroa.0.0.i.lcssa = phi i32 [ %_35.i, %bb6.i ], [ %1883, %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit ], !dbg !33764
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %left_prefix.i, ptr noundef nonnull align 32 dereferenceable(32) %uniform_left.i, i64 32, i1 false), !dbg !35931
  %1885 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i64 104, !dbg !35932
  %left_phase.i = load i32, ptr %1885, align 8, !dbg !35932, !noalias !33771, !noundef !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %right_prefix.i, ptr noundef nonnull align 32 dereferenceable(32) %uniform_right.i, i64 32, i1 false), !dbg !35933
  %1886 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i64 104, !dbg !35934
  %right_phase.i = load i32, ptr %1886, align 8, !dbg !35934, !noalias !33771, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_right.i), !dbg !35935, !noalias !33771
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i), !dbg !35936, !noalias !33771
  %1887 = getelementptr inbounds nuw i8, ptr %self, i64 1736, !dbg !35937
  %_260.1.i = load i64, ptr %1887, align 8, !dbg !35937, !alias.scope !33738, !noalias !35938, !noundef !12
  %_8.i6761 = icmp samesign ugt i64 %_260.1.i, 7, !dbg !35939
  br i1 %_8.i6761, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6764, label %bb2.i6762, !dbg !35939, !prof !1421

bb2.i6762:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_260.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !35944, !noalias !35945
  unreachable, !dbg !35944

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6764: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit
  %1888 = getelementptr inbounds nuw i8, ptr %self, i64 1728, !dbg !35937
  %_260.0.i = load ptr, ptr %1888, align 8, !dbg !35937, !alias.scope !33738, !noalias !35938, !nonnull !12, !noundef !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_260.0.i, ptr noundef nonnull align 32 dereferenceable(32) %left_prefix.i, i64 32, i1 false), !dbg !35949
  %_261.0.i = load ptr, ptr %68, align 8, !dbg !35953, !alias.scope !33738, !noalias !35938, !nonnull !12, !noundef !12
  %_261.1.i = load i64, ptr %69, align 8, !dbg !35953, !alias.scope !33738, !noalias !35938, !noundef !12
  %1889 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i), !dbg !35954
  br i1 %1889, label %bb2.i8043, label %bb6.i8034, !dbg !35954

bb6.i8034:                                        ; preds = %bb2.i8043, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6764
  %end_or_len.idx.i8035 = shl nuw nsw i64 %_261.1.i, 2, !dbg !35958
  %end_or_len.i8036 = getelementptr inbounds nuw i8, ptr %_261.0.i, i64 %end_or_len.idx.i8035, !dbg !35958
  %_293.i8037 = icmp eq i64 %_261.1.i, 0, !dbg !35962
  br i1 %_293.i8037, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit8049, label %bb10.i8038, !dbg !35965

bb2.i8043:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6764
  %bytes1.sroa.0.0.zext.i8044 = and i32 %left_phase.i, 255, !dbg !35966
  %bytes1.sroa.0.0.isplat.i8045 = mul nuw i32 %bytes1.sroa.0.0.zext.i8044, 16843009, !dbg !35966
  %_5.i8046 = icmp eq i32 %left_phase.i, %bytes1.sroa.0.0.isplat.i8045, !dbg !35967
  br i1 %_5.i8046, label %bb3.i8047, label %bb6.i8034, !dbg !35967

bb3.i8047:                                        ; preds = %bb2.i8043
  %bytes.sroa.0.0.extract.trunc.i8048 = trunc i32 %left_phase.i to i8, !dbg !35968
  %1890 = shl nuw nsw i64 %_261.1.i, 2, !dbg !35970
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_261.0.i, i8 %bytes.sroa.0.0.extract.trunc.i8048, i64 %1890, i1 false), !dbg !35970, !alias.scope !35971, !noalias !33742
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit8049, !dbg !35974

bb10.i8038:                                       ; preds = %bb6.i8034, %bb10.i8038
  %iter.sroa.0.04.i8039 = phi ptr [ %_38.i8040, %bb10.i8038 ], [ %_261.0.i, %bb6.i8034 ]
  %_38.i8040 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i8039, i64 4, !dbg !35975
  store i32 %left_phase.i, ptr %iter.sroa.0.04.i8039, align 4, !dbg !35977, !alias.scope !35971, !noalias !33742
  %_29.i8041 = icmp eq ptr %_38.i8040, %end_or_len.i8036, !dbg !35962
  br i1 %_29.i8041, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit8049, label %bb10.i8038, !dbg !35965

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit8049: ; preds = %bb10.i8038, %bb6.i8034, %bb3.i8047
  %1891 = getelementptr inbounds nuw i8, ptr %self, i64 1936, !dbg !35978
  %_262.1.i = load i64, ptr %1891, align 8, !dbg !35978, !alias.scope !33740, !noalias !35979, !noundef !12
  %_8.i6756 = icmp samesign ugt i64 %_262.1.i, 7, !dbg !35980
  br i1 %_8.i6756, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6759, label %bb2.i6757, !dbg !35980, !prof !1421

bb2.i6757:                                        ; preds = %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit8049
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_262.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !35985, !noalias !35986
  unreachable, !dbg !35985

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6759: ; preds = %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit8049
  %1892 = getelementptr inbounds nuw i8, ptr %self, i64 1928, !dbg !35978
  %_262.0.i = load ptr, ptr %1892, align 8, !dbg !35978, !alias.scope !33740, !noalias !35979, !nonnull !12, !noundef !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_262.0.i, ptr noundef nonnull align 32 dereferenceable(32) %right_prefix.i, i64 32, i1 false), !dbg !35990
  %_263.0.i = load ptr, ptr %77, align 8, !dbg !35994, !alias.scope !33740, !noalias !35979, !nonnull !12, !noundef !12
  %_263.1.i = load i64, ptr %78, align 8, !dbg !35994, !alias.scope !33740, !noalias !35979, !noundef !12
  %1893 = tail call i1 @llvm.is.constant.i32(i32 %right_phase.i), !dbg !35995
  br i1 %1893, label %bb2.i8060, label %bb6.i8051, !dbg !35995

bb6.i8051:                                        ; preds = %bb2.i8060, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6759
  %end_or_len.idx.i8052 = shl nuw nsw i64 %_263.1.i, 2, !dbg !35998
  %end_or_len.i8053 = getelementptr inbounds nuw i8, ptr %_263.0.i, i64 %end_or_len.idx.i8052, !dbg !35998
  %_293.i8054 = icmp eq i64 %_263.1.i, 0, !dbg !36002
  br i1 %_293.i8054, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit8066, label %bb10.i8055, !dbg !36005

bb2.i8060:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit6759
  %bytes1.sroa.0.0.zext.i8061 = and i32 %right_phase.i, 255, !dbg !36006
  %bytes1.sroa.0.0.isplat.i8062 = mul nuw i32 %bytes1.sroa.0.0.zext.i8061, 16843009, !dbg !36006
  %_5.i8063 = icmp eq i32 %right_phase.i, %bytes1.sroa.0.0.isplat.i8062, !dbg !36007
  br i1 %_5.i8063, label %bb3.i8064, label %bb6.i8051, !dbg !36007

bb3.i8064:                                        ; preds = %bb2.i8060
  %bytes.sroa.0.0.extract.trunc.i8065 = trunc i32 %right_phase.i to i8, !dbg !36008
  %1894 = shl nuw nsw i64 %_263.1.i, 2, !dbg !36010
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_263.0.i, i8 %bytes.sroa.0.0.extract.trunc.i8065, i64 %1894, i1 false), !dbg !36010, !alias.scope !36011, !noalias !33742
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit8066, !dbg !36014

bb10.i8055:                                       ; preds = %bb6.i8051, %bb10.i8055
  %iter.sroa.0.04.i8056 = phi ptr [ %_38.i8057, %bb10.i8055 ], [ %_263.0.i, %bb6.i8051 ]
  %_38.i8057 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i8056, i64 4, !dbg !36015
  store i32 %right_phase.i, ptr %iter.sroa.0.04.i8056, align 4, !dbg !36017, !alias.scope !36011, !noalias !33742
  %_29.i8058 = icmp eq ptr %_38.i8057, %end_or_len.i8053, !dbg !36002
  br i1 %_29.i8058, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit8066, label %bb10.i8055, !dbg !36005

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit8066: ; preds = %bb10.i8055, %bb6.i8051, %bb3.i8064
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_left.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_33) #31, !dbg !36018
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_right.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_34) #31, !dbg !36019
  store i32 %main_cursor.sroa.0.0.i.lcssa, ptr %_35, align 4, !dbg !35928, !alias.scope !33742, !noalias !33766
  store i32 %ring_cursor.sroa.0.0.i.lcssa, ptr %87, align 4, !dbg !35930, !alias.scope !33742, !noalias !33766
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i), !dbg !36020, !noalias !33771
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i), !dbg !36021, !noalias !33771
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, !dbg !33735

_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, %_RINvCsdvPQf9CMsz3_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7758, %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit8066
  br i1 %quiet.sroa.0.013229, label %bb28, label %bb40, !dbg !36022

bb22:                                             ; preds = %bb20
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36023), !dbg !36026
  %1895 = getelementptr inbounds nuw i8, ptr %self, i64 1760, !dbg !36027
  %_40.0.i = load ptr, ptr %1895, align 8, !dbg !36027, !alias.scope !36023, !nonnull !12, !noundef !12
  %1896 = getelementptr inbounds nuw i8, ptr %self, i64 1768, !dbg !36027
  %_40.1.i = load i64, ptr %1896, align 8, !dbg !36027, !alias.scope !36023, !noundef !12
  %1897 = getelementptr inbounds nuw i8, ptr %self, i64 1824, !dbg !36029
  %_41.0.i = load ptr, ptr %1897, align 8, !dbg !36029, !alias.scope !36023, !nonnull !12, !noundef !12
  %1898 = getelementptr inbounds nuw i8, ptr %self, i64 1832, !dbg !36029
  %_41.1.i = load i64, ptr %1898, align 8, !dbg !36029, !alias.scope !36023, !noundef !12
  %..i.i.i.i = tail call noundef i64 @llvm.umin.i64(i64 %_41.1.i, i64 %_40.1.i), !dbg !36030
  %_2.i6.not.i = icmp eq i64 %..i.i.i.i, 0, !dbg !36036
  br i1 %_2.i6.not.i, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb4.i8067, !dbg !36036

bb4.i8067:                                        ; preds = %bb22, %bb6.i8068
  %iter.sroa.8.07.i = phi i64 [ %1899, %bb6.i8068 ], [ 0, %bb22 ]
  %_3.i1.i.i = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i, i64 %iter.sroa.8.07.i, !dbg !36039
  %_14.i = load i32, ptr %_3.i1.i.i, align 4, !dbg !36042, !noalias !36023, !noundef !12
  %_20.i = icmp eq i32 %_14.i, 0, !dbg !36043
  br i1 %_20.i, label %panic.i8077, label %bb6.i8068, !dbg !36043

bb6.i8068:                                        ; preds = %bb4.i8067
  %_3.i.i.i8069 = getelementptr inbounds nuw i32, ptr %_40.0.i, i64 %iter.sroa.8.07.i, !dbg !36044
  %1899 = add nuw i64 %iter.sroa.8.07.i, 1, !dbg !36047
  %window.i8070 = zext i32 %_14.i to i64, !dbg !36042
  %_18.i8071 = load i32, ptr %_3.i.i.i8069, align 4, !dbg !36048, !noalias !36023, !noundef !12
  %_17.i8072 = zext i32 %_18.i8071 to i64, !dbg !36048
  %_19.i8073 = urem i64 %frames, %window.i8070, !dbg !36043
  %_16.i8074 = add nuw nsw i64 %_19.i8073, %_17.i8072, !dbg !36049
  %_15.i8075 = urem i64 %_16.i8074, %window.i8070, !dbg !36050
  %1900 = trunc nuw i64 %_15.i8075 to i32, !dbg !36051
  store i32 %1900, ptr %_3.i.i.i8069, align 4, !dbg !36051, !noalias !36023
  %exitcond.not.i = icmp eq i64 %1899, %..i.i.i.i, !dbg !36036
  br i1 %exitcond.not.i, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb4.i8067, !dbg !36036

panic.i8077:                                      ; preds = %bb4.i8067
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bac57976a2bdbfad4a3a85d5d1c7648c) #30, !dbg !36043, !noalias !36023
  unreachable, !dbg !36043

_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit: ; preds = %bb6.i8068, %bb22
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36052), !dbg !36055
  %1901 = getelementptr inbounds nuw i8, ptr %self, i64 1960, !dbg !36056
  %_40.0.i8078 = load ptr, ptr %1901, align 8, !dbg !36056, !alias.scope !36052, !nonnull !12, !noundef !12
  %1902 = getelementptr inbounds nuw i8, ptr %self, i64 1968, !dbg !36056
  %_40.1.i8079 = load i64, ptr %1902, align 8, !dbg !36056, !alias.scope !36052, !noundef !12
  %1903 = getelementptr inbounds nuw i8, ptr %self, i64 2024, !dbg !36058
  %_41.0.i8080 = load ptr, ptr %1903, align 8, !dbg !36058, !alias.scope !36052, !nonnull !12, !noundef !12
  %1904 = getelementptr inbounds nuw i8, ptr %self, i64 2032, !dbg !36058
  %_41.1.i8081 = load i64, ptr %1904, align 8, !dbg !36058, !alias.scope !36052, !noundef !12
  %..i.i.i.i8082 = tail call noundef i64 @llvm.umin.i64(i64 %_41.1.i8081, i64 %_40.1.i8079), !dbg !36059
  %_2.i6.not.i8083 = icmp eq i64 %..i.i.i.i8082, 0, !dbg !36065
  br i1 %_2.i6.not.i8083, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit8100, label %bb4.i8084, !dbg !36065

bb4.i8084:                                        ; preds = %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, %bb6.i8089
  %iter.sroa.8.07.i8085 = phi i64 [ %1905, %bb6.i8089 ], [ 0, %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit ]
  %_3.i1.i.i8086 = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i8080, i64 %iter.sroa.8.07.i8085, !dbg !36068
  %_14.i8087 = load i32, ptr %_3.i1.i.i8086, align 4, !dbg !36071, !noalias !36052, !noundef !12
  %_20.i8088 = icmp eq i32 %_14.i8087, 0, !dbg !36072
  br i1 %_20.i8088, label %panic.i8099, label %bb6.i8089, !dbg !36072

bb6.i8089:                                        ; preds = %bb4.i8084
  %_3.i.i.i8090 = getelementptr inbounds nuw i32, ptr %_40.0.i8078, i64 %iter.sroa.8.07.i8085, !dbg !36073
  %1905 = add nuw i64 %iter.sroa.8.07.i8085, 1, !dbg !36076
  %window.i8091 = zext i32 %_14.i8087 to i64, !dbg !36071
  %_18.i8092 = load i32, ptr %_3.i.i.i8090, align 4, !dbg !36077, !noalias !36052, !noundef !12
  %_17.i8093 = zext i32 %_18.i8092 to i64, !dbg !36077
  %_19.i8094 = urem i64 %frames, %window.i8091, !dbg !36072
  %_16.i8095 = add nuw nsw i64 %_19.i8094, %_17.i8093, !dbg !36078
  %_15.i8096 = urem i64 %_16.i8095, %window.i8091, !dbg !36079
  %1906 = trunc nuw i64 %_15.i8096 to i32, !dbg !36080
  store i32 %1906, ptr %_3.i.i.i8090, align 4, !dbg !36080, !noalias !36052
  %exitcond.not.i8097 = icmp eq i64 %1905, %..i.i.i.i8082, !dbg !36065
  br i1 %exitcond.not.i8097, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit8100, label %bb4.i8084, !dbg !36065

panic.i8099:                                      ; preds = %bb4.i8084
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bac57976a2bdbfad4a3a85d5d1c7648c) #30, !dbg !36072, !noalias !36052
  unreachable, !dbg !36072

_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit8100: ; preds = %bb6.i8089, %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit
  %1907 = getelementptr inbounds nuw i8, ptr %self, i64 1624, !dbg !36081
  %_29.val = load i64, ptr %1907, align 8, !dbg !36081
  %1908 = getelementptr inbounds nuw i8, ptr %self, i64 1632, !dbg !36081
  %_29.val7016 = load i64, ptr %1908, align 8, !dbg !36081, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36082), !dbg !36081
  %_10.i8101 = icmp eq i64 %_29.val7016, 0, !dbg !36085
  br i1 %_10.i8101, label %panic.i8113, label %bb1.i8102, !dbg !36085

bb1.i8102:                                        ; preds = %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit8100
  %_28 = getelementptr inbounds nuw i8, ptr %self, i64 1640, !dbg !36087
  %_7.i = load i32, ptr %_28, align 4, !dbg !36088, !alias.scope !36082, !noundef !12
  %_6.i8103 = zext i32 %_7.i to i64, !dbg !36088
  %_8.i8104 = urem i64 %frames, %_29.val7016, !dbg !36085
  %_5.i8105 = add nuw nsw i64 %_8.i8104, %_6.i8103, !dbg !36089
  %_4.i8106 = urem i64 %_5.i8105, %_29.val7016, !dbg !36090
  %1909 = trunc i64 %_4.i8106 to i32, !dbg !36091
  store i32 %1909, ptr %_28, align 4, !dbg !36091, !alias.scope !36082
  %_17.i8107 = icmp eq i64 %_29.val, 0, !dbg !36092
  br i1 %_17.i8107, label %panic2.i, label %_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit, !dbg !36092

panic.i8113:                                      ; preds = %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit8100
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f0ee36f67d9a332211aa5518dd2ebfd5) #30, !dbg !36085, !noalias !36082
  unreachable, !dbg !36085

panic2.i:                                         ; preds = %bb1.i8102
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_33d4d33e0a850133578789055882dcf9) #30, !dbg !36092, !noalias !36082
  unreachable, !dbg !36092

_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit: ; preds = %bb1.i8102
  %1910 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !36093
  %_14.i8109 = load i32, ptr %1910, align 4, !dbg !36093, !alias.scope !36082, !noundef !12
  %_13.i8110 = zext i32 %_14.i8109 to i64, !dbg !36093
  %_15.i8111 = urem i64 %frames, %_29.val, !dbg !36092
  %_12.i = add nuw nsw i64 %_15.i8111, %_13.i8110, !dbg !36094
  %_11.i8112 = urem i64 %_12.i, %_29.val, !dbg !36095
  %1911 = trunc i64 %_11.i8112 to i32, !dbg !36096
  store i32 %1911, ptr %1910, align 4, !dbg !36096, !alias.scope !36082
  br label %bb42, !dbg !36097

bb28:                                             ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_37 = tail call noundef zeroext i1 @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_33) #31, !dbg !36098
  br i1 %_37, label %bb30, label %bb40, !dbg !36099

bb30:                                             ; preds = %bb28
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_39 = tail call noundef zeroext i1 @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_34) #31, !dbg !36100
  br i1 %_39, label %bb32, label %bb40, !dbg !36101

bb32:                                             ; preds = %bb30
  %_80.not = icmp samesign ugt i64 %words, %left_io.1
  br i1 %_80.not, label %bb51, label %bb1.i8114, !dbg !36102, !prof !165

bb51:                                             ; preds = %bb32
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %words, i64 noundef %left_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5792a3affd2091ba045d8adc49b05663) #30, !dbg !36110
  unreachable, !dbg !36110

bb1.i8114:                                        ; preds = %bb32, %bb10.i8128
  %iter.sroa.6.0.i8115 = phi i64 [ %len.i.i.i.i8120, %bb10.i8128 ], [ %words, %bb32 ], !dbg !36111
  %iter.sroa.0.0.i8116 = phi ptr [ %data.i.i.i.i8119, %bb10.i8128 ], [ %left_io.0, %bb32 ], !dbg !36111
  %1912 = icmp eq i64 %iter.sroa.6.0.i8115, 0, !dbg !36113
  br i1 %1912, label %bb34, label %bb11.preheader.i8117, !dbg !36113

bb11.preheader.i8117:                             ; preds = %bb1.i8114
  %..i.i.i8118 = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i8115, i64 32), !dbg !36115
  %_18.idx.i8121 = shl nuw nsw i64 %..i.i.i8118, 2, !dbg !36118
  %_18.i8122 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i8116, i64 %_18.idx.i8121, !dbg !36118
  br label %bb11.i8123, !dbg !36123

bb11.i8123:                                       ; preds = %bb11.i8123, %bb11.preheader.i8117
  %iter1.sroa.0.014.i8124 = phi ptr [ %_31.i8126, %bb11.i8123 ], [ %iter.sroa.0.0.i8116, %bb11.preheader.i8117 ]
  %bits.sroa.0.013.i8125 = phi i32 [ %1913, %bb11.i8123 ], [ 0, %bb11.preheader.i8117 ]
  %_31.i8126 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i8124, i64 4, !dbg !36125
  %_134.i8127 = load i32, ptr %iter1.sroa.0.014.i8124, align 4, !dbg !36127, !alias.scope !36128, !noundef !12
  %1913 = or i32 %_134.i8127, %bits.sroa.0.013.i8125, !dbg !36131
  %_25.i = icmp eq ptr %_31.i8126, %_18.i8122, !dbg !36132
  br i1 %_25.i, label %bb10.i8128, label %bb11.i8123, !dbg !36123

bb10.i8128:                                       ; preds = %bb11.i8123
  %data.i.i.i.i8119 = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i8116, i64 %..i.i.i8118, !dbg !36134
  %len.i.i.i.i8120 = sub nuw nsw i64 %iter.sroa.6.0.i8115, %..i.i.i8118, !dbg !36139
  %1914 = icmp eq i32 %1913, 0, !dbg !36140
  br i1 %1914, label %bb1.i8114, label %bb40, !dbg !36140

bb34:                                             ; preds = %bb1.i8114
  %_88.not = icmp samesign ugt i64 %words, %right_io.1, !dbg !36141
  br i1 %_88.not, label %bb54, label %bb1.i8141, !dbg !36141, !prof !1406

bb40:                                             ; preds = %bb10.i8128, %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit, %bb28, %bb30, %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit8158
  %_36.sroa.0.0 = phi i8 [ %2005, %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit8158 ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter13limiter_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit ], [ 0, %bb30 ], [ 0, %bb28 ], [ 0, %bb10.i8128 ], !dbg !36147
  store i8 %_36.sroa.0.0, ptr %38, align 8, !dbg !36148
  %1915 = load i8, ptr %2, align 32, !dbg !36149, !range !17, !noundef !12
  store i8 %1915, ptr %0, align 1, !dbg !36150
  call void @llvm.lifetime.start.p0(ptr nonnull %shape), !dbg !36151
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %shape, ptr noundef nonnull align 16 dereferenceable(24) %_32, i64 24, i1 false), !dbg !36152
  %1916 = getelementptr inbounds nuw i8, ptr %self, i64 2120, !dbg !36153
  %1917 = load i32, ptr %1916, align 8, !dbg !36153, !noundef !12
  %_53 = getelementptr inbounds nuw i8, ptr %self, i64 1568, !dbg !36155
  %1918 = getelementptr inbounds nuw i8, ptr %self, i64 1584, !dbg !36162
  %_99.0 = load ptr, ptr %1918, align 16, !dbg !36162, !nonnull !12, !noundef !12
  %1919 = getelementptr inbounds nuw i8, ptr %self, i64 1592, !dbg !36162
  %_99.1 = load i64, ptr %1919, align 8, !dbg !36162, !noundef !12
  %1920 = getelementptr inbounds nuw i8, ptr %self, i64 1600, !dbg !36162
  %_100.0 = load ptr, ptr %1920, align 32, !dbg !36162, !nonnull !12, !noundef !12
  %1921 = getelementptr inbounds nuw i8, ptr %self, i64 1608, !dbg !36162
  %_100.1 = load i64, ptr %1921, align 8, !dbg !36162, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36163), !dbg !36166
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36167), !dbg !36166
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36169), !dbg !36166
  %1922 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !36171
  %fst_len.i.i = and i64 %left_io.1, 2305843009213693944, !dbg !36180
  %1923 = bitcast <8 x float> %1922 to <8 x i32>, !dbg !36183
  %_22.not.i19207.i = icmp eq i64 %fst_len.i.i, 0, !dbg !36187
  br i1 %_22.not.i19207.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit25.i, label %bb13.i20.i, !dbg !36187

bb13.i20.i:                                       ; preds = %bb40, %bb13.i20.i
  %iter.sroa.0.0.i18210.i = phi ptr [ %_27.i21.i, %bb13.i20.i ], [ %left_io.0, %bb40 ]
  %iter.sroa.5.0.i17209.i = phi i64 [ %_28.i22.i, %bb13.i20.i ], [ %fst_len.i.i, %bb40 ]
  %ok.i9.sroa.0.0208.i = phi <8 x i32> [ %1928, %bb13.i20.i ], [ %1923, %bb40 ]
  %lanes.i.sroa.0.0.copyload.i = load <8 x i32>, ptr %iter.sroa.0.0.i18210.i, align 4, !dbg !36190, !alias.scope !36195, !noalias !36199
  %_27.i21.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i18210.i, i64 32, !dbg !36204
  %_28.i22.i = add i64 %iter.sroa.5.0.i17209.i, -8, !dbg !36207
  %1924 = and <8 x i32> %lanes.i.sroa.0.0.copyload.i, splat (i32 2147483647), !dbg !36208
  %1925 = bitcast <8 x i32> %1924 to <8 x float>, !dbg !36214
  %1926 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1925, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !36215
  %1927 = bitcast <8 x float> %1926 to <8 x i32>, !dbg !36183
  %1928 = and <8 x i32> %ok.i9.sroa.0.0208.i, %1927, !dbg !36221
  %_22.not.i19.i = icmp eq i64 %_28.i22.i, 0, !dbg !36187
  br i1 %_22.not.i19.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit25.i, label %bb13.i20.i, !dbg !36187

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit25.i: ; preds = %bb13.i20.i, %bb40
  %ok.i9.sroa.0.0.lcssa.i = phi <8 x i32> [ %1923, %bb40 ], [ %1928, %bb13.i20.i ], !dbg !36223
  %1929 = icmp sgt <8 x i32> %ok.i9.sroa.0.0.lcssa.i, splat (i32 -1), !dbg !36224
  %1930 = bitcast <8 x i1> %1929 to i8, !dbg !36224
  %_0.i90.not.i = icmp eq i8 %1930, 0, !dbg !36229
  br i1 %_0.i90.not.i, label %bb2.i8137, label %bb7.i8131, !dbg !36230

bb2.i8137:                                        ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit25.i
  %fst_len.i105.i = and i64 %right_io.1, 2305843009213693944, !dbg !36231
  %_22.not.i211.i = icmp eq i64 %fst_len.i105.i, 0, !dbg !36235
  br i1 %_22.not.i211.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %bb13.i.i8138, !dbg !36235

bb13.i.i8138:                                     ; preds = %bb2.i8137, %bb13.i.i8138
  %iter.sroa.0.0.i214.i = phi ptr [ %_27.i.i8139, %bb13.i.i8138 ], [ %right_io.0, %bb2.i8137 ]
  %iter.sroa.5.0.i213.i = phi i64 [ %_28.i.i8140, %bb13.i.i8138 ], [ %fst_len.i105.i, %bb2.i8137 ]
  %ok.i.sroa.0.0212.i = phi <8 x i32> [ %1935, %bb13.i.i8138 ], [ %1923, %bb2.i8137 ]
  %lanes.i49.sroa.0.0.copyload.i = load <8 x i32>, ptr %iter.sroa.0.0.i214.i, align 4, !dbg !36238, !alias.scope !36243, !noalias !36247
  %_27.i.i8139 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i214.i, i64 32, !dbg !36251
  %_28.i.i8140 = add i64 %iter.sroa.5.0.i213.i, -8, !dbg !36254
  %1931 = and <8 x i32> %lanes.i49.sroa.0.0.copyload.i, splat (i32 2147483647), !dbg !36255
  %1932 = bitcast <8 x i32> %1931 to <8 x float>, !dbg !36261
  %1933 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1932, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !36262
  %1934 = bitcast <8 x float> %1933 to <8 x i32>, !dbg !36268
  %1935 = and <8 x i32> %ok.i.sroa.0.0212.i, %1934, !dbg !36272
  %_22.not.i.i = icmp eq i64 %_28.i.i8140, 0, !dbg !36235
  br i1 %_22.not.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %bb13.i.i8138, !dbg !36235

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i: ; preds = %bb13.i.i8138, %bb2.i8137
  %ok.i.sroa.0.0.lcssa.i = phi <8 x i32> [ %1923, %bb2.i8137 ], [ %1935, %bb13.i.i8138 ], !dbg !36274
  %1936 = icmp sgt <8 x i32> %ok.i.sroa.0.0.lcssa.i, splat (i32 -1), !dbg !36275
  %1937 = bitcast <8 x i1> %1936 to i8, !dbg !36275
  %_0.i93.not.i = icmp eq i8 %1937, 0, !dbg !36280
  br i1 %_0.i93.not.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB1z_11LimiterCoreBR_E13process_block0EB1z_.exit, label %bb7.i8131, !dbg !36281

bb7.i8131:                                        ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit25.i
  br i1 %_22.not.i19207.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %bb23.i.i, !dbg !36282

bb23.i.i:                                         ; preds = %bb7.i8131, %bb23.i.i
  %iter.sroa.0.074.i.i = phi ptr [ %_45.i.i, %bb23.i.i ], [ %left_io.0, %bb7.i8131 ]
  %iter.sroa.5.073.i.i = phi i64 [ %_46.i.i8132, %bb23.i.i ], [ %fst_len.i.i, %bb7.i8131 ]
  %ok.sroa.0.072.i.i = phi <8 x i32> [ %1942, %bb23.i.i ], [ %1923, %bb7.i8131 ]
  %lanes.i.sroa.0.0.copyload.i.i = load <8 x i32>, ptr %iter.sroa.0.074.i.i, align 4, !dbg !36286, !alias.scope !36291, !noalias !36297
  %_45.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.074.i.i, i64 32, !dbg !36301
  %_46.i.i8132 = add i64 %iter.sroa.5.073.i.i, -8, !dbg !36304
  %1938 = and <8 x i32> %lanes.i.sroa.0.0.copyload.i.i, splat (i32 2147483647), !dbg !36305
  %1939 = bitcast <8 x i32> %1938 to <8 x float>, !dbg !36311
  %1940 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1939, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !36312
  %1941 = bitcast <8 x float> %1940 to <8 x i32>, !dbg !36318
  %1942 = and <8 x i32> %ok.sroa.0.072.i.i, %1941, !dbg !36322
  %_40.not.i.i = icmp eq i64 %_46.i.i8132, 0, !dbg !36282
  br i1 %_40.not.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %bb23.i.i, !dbg !36282

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i: ; preds = %bb23.i.i, %bb7.i8131
  %ok.sroa.0.0.lcssa.i.i = phi <8 x i32> [ %1923, %bb7.i8131 ], [ %1942, %bb23.i.i ], !dbg !36324
  %1943 = icmp slt <8 x i32> %ok.sroa.0.0.lcssa.i.i, zeroinitializer, !dbg !36325
  %bc.i.i = select <8 x i1> %1943, <8 x i32> zeroinitializer, <8 x i32> splat (i32 1065353216), !dbg !36325
  %1944 = extractelement <8 x i32> %bc.i.i, i64 0, !dbg !36330
  %1945 = icmp ne i32 %1944, 0, !dbg !36330
  %1946 = zext i1 %1945 to i32, !dbg !36330
  %1947 = extractelement <8 x i32> %bc.i.i, i64 1, !dbg !36330
  %1948 = icmp eq i32 %1947, 0, !dbg !36330
  %1949 = select i1 %1948, i32 0, i32 2, !dbg !36330
  %1950 = extractelement <8 x i32> %bc.i.i, i64 2, !dbg !36330
  %1951 = icmp eq i32 %1950, 0, !dbg !36330
  %1952 = select i1 %1951, i32 0, i32 4, !dbg !36330
  %1953 = extractelement <8 x i32> %bc.i.i, i64 3, !dbg !36330
  %1954 = icmp eq i32 %1953, 0, !dbg !36330
  %1955 = select i1 %1954, i32 0, i32 8, !dbg !36330
  %1956 = extractelement <8 x i32> %bc.i.i, i64 4, !dbg !36330
  %1957 = icmp eq i32 %1956, 0, !dbg !36330
  %1958 = select i1 %1957, i32 0, i32 16, !dbg !36330
  %1959 = extractelement <8 x i32> %bc.i.i, i64 5, !dbg !36330
  %1960 = icmp eq i32 %1959, 0, !dbg !36330
  %1961 = select i1 %1960, i32 0, i32 32, !dbg !36330
  %1962 = extractelement <8 x i32> %bc.i.i, i64 6, !dbg !36330
  %1963 = icmp eq i32 %1962, 0, !dbg !36330
  %1964 = select i1 %1963, i32 0, i32 64, !dbg !36330
  %1965 = extractelement <8 x i32> %bc.i.i, i64 7, !dbg !36330
  %1966 = icmp eq i32 %1965, 0, !dbg !36330
  %1967 = select i1 %1966, i32 0, i32 128, !dbg !36330
  %fst_len.i.i108.i = and i64 %right_io.1, 2305843009213693944, !dbg !36331
  %_40.not71.i109.i = icmp eq i64 %fst_len.i.i108.i, 0, !dbg !36335
  br i1 %_40.not71.i109.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit134.i, label %bb23.i110.i, !dbg !36335

bb23.i110.i:                                      ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, %bb23.i110.i
  %iter.sroa.0.074.i111.i = phi ptr [ %_45.i115.i, %bb23.i110.i ], [ %right_io.0, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i ]
  %iter.sroa.5.073.i112.i = phi i64 [ %_46.i116.i, %bb23.i110.i ], [ %fst_len.i.i108.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i ]
  %ok.sroa.0.072.i113.i = phi <8 x i32> [ %1972, %bb23.i110.i ], [ %1923, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i ]
  %lanes.i.sroa.0.0.copyload.i114.i = load <8 x i32>, ptr %iter.sroa.0.074.i111.i, align 4, !dbg !36338, !alias.scope !36343, !noalias !36349
  %_45.i115.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.074.i111.i, i64 32, !dbg !36353
  %_46.i116.i = add i64 %iter.sroa.5.073.i112.i, -8, !dbg !36356
  %1968 = and <8 x i32> %lanes.i.sroa.0.0.copyload.i114.i, splat (i32 2147483647), !dbg !36357
  %1969 = bitcast <8 x i32> %1968 to <8 x float>, !dbg !36363
  %1970 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1969, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !36364
  %1971 = bitcast <8 x float> %1970 to <8 x i32>, !dbg !36370
  %1972 = and <8 x i32> %ok.sroa.0.072.i113.i, %1971, !dbg !36374
  %_40.not.i117.i = icmp eq i64 %_46.i116.i, 0, !dbg !36335
  br i1 %_40.not.i117.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit134.i, label %bb23.i110.i, !dbg !36335

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit134.i: ; preds = %bb23.i110.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i
  %ok.sroa.0.0.lcssa.i118.i = phi <8 x i32> [ %1923, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i ], [ %1972, %bb23.i110.i ], !dbg !36376
  %1973 = icmp slt <8 x i32> %ok.sroa.0.0.lcssa.i118.i, zeroinitializer, !dbg !36377
  %bc.i119.i = select <8 x i1> %1973, <8 x i32> zeroinitializer, <8 x i32> splat (i32 1065353216), !dbg !36377
  %1974 = extractelement <8 x i32> %bc.i119.i, i64 0, !dbg !36382
  %1975 = icmp ne i32 %1974, 0, !dbg !36382
  %1976 = zext i1 %1975 to i32, !dbg !36382
  %1977 = extractelement <8 x i32> %bc.i119.i, i64 1, !dbg !36382
  %1978 = icmp eq i32 %1977, 0, !dbg !36382
  %1979 = select i1 %1978, i32 0, i32 2, !dbg !36382
  %1980 = extractelement <8 x i32> %bc.i119.i, i64 2, !dbg !36382
  %1981 = icmp eq i32 %1980, 0, !dbg !36382
  %1982 = select i1 %1981, i32 0, i32 4, !dbg !36382
  %1983 = extractelement <8 x i32> %bc.i119.i, i64 3, !dbg !36382
  %1984 = icmp eq i32 %1983, 0, !dbg !36382
  %1985 = select i1 %1984, i32 0, i32 8, !dbg !36382
  %1986 = extractelement <8 x i32> %bc.i119.i, i64 4, !dbg !36382
  %1987 = icmp eq i32 %1986, 0, !dbg !36382
  %1988 = select i1 %1987, i32 0, i32 16, !dbg !36382
  %1989 = extractelement <8 x i32> %bc.i119.i, i64 5, !dbg !36382
  %1990 = icmp eq i32 %1989, 0, !dbg !36382
  %1991 = select i1 %1990, i32 0, i32 32, !dbg !36382
  %1992 = extractelement <8 x i32> %bc.i119.i, i64 6, !dbg !36382
  %1993 = icmp eq i32 %1992, 0, !dbg !36382
  %1994 = select i1 %1993, i32 0, i32 64, !dbg !36382
  %1995 = extractelement <8 x i32> %bc.i119.i, i64 7, !dbg !36382
  %1996 = icmp eq i32 %1995, 0, !dbg !36382
  %1997 = select i1 %1996, i32 0, i32 128, !dbg !36382
  %1998 = getelementptr inbounds nuw i8, ptr %self, i64 1576, !dbg !36383
  %mask.sroa.0.1.1.i121.i = or disjoint i32 %1949, %1946, !dbg !36382
  %mask.sroa.0.1.2.i123.i = or disjoint i32 %mask.sroa.0.1.1.i121.i, %1952, !dbg !36382
  %mask.sroa.0.1.3.i125.i = or disjoint i32 %mask.sroa.0.1.2.i123.i, %1955, !dbg !36382
  %mask.sroa.0.1.4.i127.i = or disjoint i32 %mask.sroa.0.1.3.i125.i, %1958, !dbg !36382
  %mask.sroa.0.1.5.i129.i = or disjoint i32 %mask.sroa.0.1.4.i127.i, %1961, !dbg !36382
  %mask.sroa.0.1.6.i131.i = or i32 %mask.sroa.0.1.5.i129.i, %1964, !dbg !36382
  %mask.sroa.0.1.7.i133.i = or i32 %mask.sroa.0.1.6.i131.i, %1967, !dbg !36382
  %mask.sroa.0.1.1.i.i = or i32 %mask.sroa.0.1.7.i133.i, %1976, !dbg !36330
  %mask.sroa.0.1.2.i.i = or i32 %mask.sroa.0.1.1.i.i, %1979, !dbg !36330
  %mask.sroa.0.1.3.i.i = or i32 %mask.sroa.0.1.2.i.i, %1982, !dbg !36330
  %mask.sroa.0.1.4.i.i = or i32 %mask.sroa.0.1.3.i.i, %1985, !dbg !36330
  %mask.sroa.0.1.5.i.i = or i32 %mask.sroa.0.1.4.i.i, %1988, !dbg !36330
  %mask.sroa.0.1.6.i.i = or i32 %mask.sroa.0.1.5.i.i, %1991, !dbg !36330
  %mask.sroa.0.1.7.i.i = or i32 %mask.sroa.0.1.6.i.i, %1994, !dbg !36330
  %1999 = or i32 %mask.sroa.0.1.7.i.i, %1997, !dbg !36383
  store i32 %1999, ptr %1998, align 8, !dbg !36383, !alias.scope !36169, !noalias !36384
  %_14.i8133 = load i64, ptr %_53, align 8, !dbg !36385, !alias.scope !36169, !noalias !36384, !noundef !12
  %2000 = tail call i64 @llvm.uadd.sat.i64(i64 %_14.i8133, i64 1), !dbg !36386
  store i64 %2000, ptr %_53, align 8, !dbg !36389, !alias.scope !36169, !noalias !36384
  %_222.i.i = icmp eq i64 %left_io.1, 0, !dbg !36390
  br i1 %_222.i.i, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %bb14.i.preheader.i, !dbg !36396

bb14.i.preheader.i:                               ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit134.i
  %.idx.i.i = shl nuw nsw i64 %left_io.1, 2, !dbg !36397
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %left_io.0, i8 0, i64 %.idx.i.i, i1 false), !dbg !36401, !alias.scope !36402, !noalias !36405
  br label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i, !dbg !36406

_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i: ; preds = %bb14.i.preheader.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit134.i
  %_222.i136.i = icmp eq i64 %right_io.1, 0, !dbg !36412
  br i1 %_222.i136.i, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit141.i, label %bb14.i137.preheader.i, !dbg !36415

bb14.i137.preheader.i:                            ; preds = %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i
  %.idx.i135.i = shl nuw nsw i64 %right_io.1, 2, !dbg !36406
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %right_io.0, i8 0, i64 %.idx.i135.i, i1 false), !dbg !36416, !alias.scope !36417, !noalias !36420
  br label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit141.i, !dbg !36421

_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit141.i: ; preds = %bb14.i137.preheader.i, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 8 dereferenceable(200) %_33, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_99.0, i64 noundef %_99.1, i32 noundef %1917) #31, !dbg !36422, !noalias !36425
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 8 dereferenceable(200) %_34, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_100.0, i64 noundef %_100.1, i32 noundef %1917) #31, !dbg !36428, !noalias !36425
  store i32 0, ptr %_35, align 4, !dbg !36429, !noalias !36425
  %2001 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !36429
  store i32 0, ptr %2001, align 4, !dbg !36429, !noalias !36425
  br label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB1z_11LimiterCoreBR_E13process_block0EB1z_.exit, !dbg !36430

_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB1z_11LimiterCoreBR_E13process_block0EB1z_.exit: ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit141.i
  call void @llvm.lifetime.end.p0(ptr nonnull %shape), !dbg !36431
  br label %bb42, !dbg !36097

bb54:                                             ; preds = %bb34
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %words, i64 noundef %right_io.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e8e5016f8f28771294ff88b89dfac103) #30, !dbg !36432
  unreachable, !dbg !36432

bb1.i8141:                                        ; preds = %bb34, %bb10.i8156
  %iter.sroa.6.0.i8142 = phi i64 [ %len.i.i.i.i8147, %bb10.i8156 ], [ %words, %bb34 ], !dbg !36433
  %iter.sroa.0.0.i8143 = phi ptr [ %data.i.i.i.i8146, %bb10.i8156 ], [ %right_io.0, %bb34 ], !dbg !36433
  %2002 = icmp eq i64 %iter.sroa.6.0.i8142, 0, !dbg !36435
  br i1 %2002, label %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit8158, label %bb11.preheader.i8144, !dbg !36435

bb11.preheader.i8144:                             ; preds = %bb1.i8141
  %..i.i.i8145 = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i8142, i64 32), !dbg !36437
  %_18.idx.i8148 = shl nuw nsw i64 %..i.i.i8145, 2, !dbg !36440
  %_18.i8149 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i8143, i64 %_18.idx.i8148, !dbg !36440
  br label %bb11.i8150, !dbg !36445

bb11.i8150:                                       ; preds = %bb11.i8150, %bb11.preheader.i8144
  %iter1.sroa.0.014.i8151 = phi ptr [ %_31.i8153, %bb11.i8150 ], [ %iter.sroa.0.0.i8143, %bb11.preheader.i8144 ]
  %bits.sroa.0.013.i8152 = phi i32 [ %2003, %bb11.i8150 ], [ 0, %bb11.preheader.i8144 ]
  %_31.i8153 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i8151, i64 4, !dbg !36447
  %_134.i8154 = load i32, ptr %iter1.sroa.0.014.i8151, align 4, !dbg !36449, !alias.scope !36450, !noundef !12
  %2003 = or i32 %_134.i8154, %bits.sroa.0.013.i8152, !dbg !36453
  %_25.i8155 = icmp eq ptr %_31.i8153, %_18.i8149, !dbg !36454
  br i1 %_25.i8155, label %bb10.i8156, label %bb11.i8150, !dbg !36445

bb10.i8156:                                       ; preds = %bb11.i8150
  %data.i.i.i.i8146 = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i8143, i64 %..i.i.i8145, !dbg !36456
  %len.i.i.i.i8147 = sub nuw nsw i64 %iter.sroa.6.0.i8142, %..i.i.i8145, !dbg !36461
  %2004 = icmp eq i32 %2003, 0, !dbg !36462
  br i1 %2004, label %bb1.i8141, label %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit8158, !dbg !36462

_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit8158: ; preds = %bb1.i8141, %bb10.i8156
  %2005 = zext i1 %2002 to i8, !dbg !36148
  br label %bb40, !dbg !36022

bb42:                                             ; preds = %_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank12finish_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NCNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB1z_11LimiterCoreBR_E13process_block0EB1z_.exit
  ret void, !dbg !36097
}
