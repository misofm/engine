define internal fastcc void @_RNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB5_11LimiterCoreNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E13process_blockB5_(ptr noalias noundef nonnull align 16 dereferenceable(1136) %self, ptr noalias noundef nonnull align 4 captures(address) %left_io.0, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef nonnull align 4 captures(address) %right_io.0, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef %frames) unnamed_addr #0 !dbg !4662 {
start:
  %peaks_right.i1086 = alloca [1024 x i8], align 4
  %peaks_left.i1087 = alloca [1024 x i8], align 4
  %scratch.i1088 = alloca [32 x i8], align 4
  %hot_right.i1090 = alloca [368 x i8], align 16
  %hot_left.i1091 = alloca [368 x i8], align 16
  %peaks_right.i779 = alloca [1024 x i8], align 4
  %peaks_left.i780 = alloca [1024 x i8], align 4
  %scratch.i = alloca [32 x i8], align 4
  %hot_right.i782 = alloca [368 x i8], align 16
  %hot_left.i783 = alloca [368 x i8], align 16
  %uniform_right.i62 = alloca [64 x i8], align 16
  %uniform_left.i63 = alloca [64 x i8], align 16
  %peaks_right.i64 = alloca [1024 x i8], align 4
  %peaks_left.i65 = alloca [1024 x i8], align 4
  %hot_right.i67 = alloca [368 x i8], align 16
  %hot_left.i68 = alloca [368 x i8], align 16
  %uniform_right.i = alloca [64 x i8], align 16
  %uniform_left.i = alloca [64 x i8], align 16
  %peaks_right.i = alloca [1024 x i8], align 4
  %peaks_left.i = alloca [1024 x i8], align 4
  %hot_right.i = alloca [368 x i8], align 16
  %hot_left.i = alloca [368 x i8], align 16
  %shape = alloca [12 x i8], align 4
  %words = shl i32 %frames, 2, !dbg !4664
  %0 = getelementptr inbounds nuw i8, ptr %self, i32 1125, !dbg !4665
  %1 = load i8, ptr %0, align 1, !dbg !4665, !range !4667, !noundef !10
  %2 = getelementptr inbounds nuw i8, ptr %self, i32 904, !dbg !4668
  %3 = load i8, ptr %2, align 8, !dbg !4668, !range !4667, !noundef !10
  %_7 = icmp eq i8 %1, %3, !dbg !4665
  %4 = getelementptr inbounds nuw i8, ptr %self, i32 988
  %_95.0 = load ptr, ptr %4, align 4, !dbg !4669
  %5 = getelementptr inbounds nuw i8, ptr %self, i32 992
  %_95.1 = load i32, ptr %5, align 4, !dbg !4669
  br i1 %_7, label %bb1, label %bb20.thread, !dbg !4665

bb1:                                              ; preds = %start
  %_8.i6814 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_95.0, i32 %_95.1, !dbg !4670
  br label %bb1.i.i, !dbg !4675

bb1.i.i:                                          ; preds = %bb11.i.i, %bb1
  %_221.i.i = phi ptr [ %_22.i.i6816, %bb11.i.i ], [ %_95.0, %bb1 ]
  %_12.i.i6815 = icmp eq ptr %_221.i.i, %_8.i6814, !dbg !4677
  br i1 %_12.i.i6815, label %bb3, label %bb11.i.i, !dbg !4680

bb11.i.i:                                         ; preds = %bb1.i.i
  %_22.i.i6816 = getelementptr inbounds nuw i8, ptr %_221.i.i, i32 16, !dbg !4681
  %6 = getelementptr inbounds nuw i8, ptr %_221.i.i, i32 12, !dbg !4683
  %_3.i.i.i = load i32, ptr %6, align 4, !dbg !4683, !alias.scope !4685, !noalias !4690, !noundef !10
  %7 = icmp eq i32 %_3.i.i.i, 0, !dbg !4683
  %_51.i.i.i = load i32, ptr %_221.i.i, align 4, !dbg !4683, !alias.scope !4685, !noalias !4690
  %8 = getelementptr inbounds nuw i8, ptr %_221.i.i, i32 4, !dbg !4683
  %_72.i.i.i = load i32, ptr %8, align 4, !dbg !4683, !alias.scope !4685, !noalias !4690
  %9 = icmp eq i32 %_51.i.i.i, %_72.i.i.i, !dbg !4683
  %_0.sroa.0.0.off0.i.i.i = select i1 %7, i1 %9, i1 false, !dbg !4683
  br i1 %_0.sroa.0.0.off0.i.i.i, label %bb1.i.i, label %bb20.thread, !dbg !4693

bb3:                                              ; preds = %bb1.i.i
  %10 = getelementptr inbounds nuw i8, ptr %self, i32 996, !dbg !4694
  %_96.0 = load ptr, ptr %10, align 4, !dbg !4694, !nonnull !10, !noundef !10
  %11 = getelementptr inbounds nuw i8, ptr %self, i32 1000, !dbg !4694
  %_96.1 = load i32, ptr %11, align 4, !dbg !4694, !noundef !10
  %_8.i6817 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_96.0, i32 %_96.1, !dbg !4695
  br label %bb1.i.i6818, !dbg !4700

bb1.i.i6818:                                      ; preds = %bb11.i.i6821, %bb3
  %_221.i.i6819 = phi ptr [ %_22.i.i6822, %bb11.i.i6821 ], [ %_96.0, %bb3 ]
  %_12.i.i6820 = icmp eq ptr %_221.i.i6819, %_8.i6817, !dbg !4702
  br i1 %_12.i.i6820, label %bb5, label %bb11.i.i6821, !dbg !4705

bb11.i.i6821:                                     ; preds = %bb1.i.i6818
  %_22.i.i6822 = getelementptr inbounds nuw i8, ptr %_221.i.i6819, i32 16, !dbg !4706
  %12 = getelementptr inbounds nuw i8, ptr %_221.i.i6819, i32 12, !dbg !4708
  %_3.i.i.i6823 = load i32, ptr %12, align 4, !dbg !4708, !alias.scope !4710, !noalias !4715, !noundef !10
  %13 = icmp eq i32 %_3.i.i.i6823, 0, !dbg !4708
  %_51.i.i.i6824 = load i32, ptr %_221.i.i6819, align 4, !dbg !4708, !alias.scope !4710, !noalias !4715
  %14 = getelementptr inbounds nuw i8, ptr %_221.i.i6819, i32 4, !dbg !4708
  %_72.i.i.i6825 = load i32, ptr %14, align 4, !dbg !4708, !alias.scope !4710, !noalias !4715
  %15 = icmp eq i32 %_51.i.i.i6824, %_72.i.i.i6825, !dbg !4708
  %_0.sroa.0.0.off0.i.i.i6826 = select i1 %13, i1 %15, i1 false, !dbg !4708
  br i1 %_0.sroa.0.0.off0.i.i.i6826, label %bb1.i.i6818, label %bb20.thread, !dbg !4718

bb5:                                              ; preds = %bb1.i.i6818
  %16 = getelementptr inbounds nuw i8, ptr %self, i32 1088, !dbg !4719
  %_97.0 = load ptr, ptr %16, align 16, !dbg !4719, !nonnull !10, !noundef !10
  %17 = getelementptr inbounds nuw i8, ptr %self, i32 1092, !dbg !4719
  %_97.1 = load i32, ptr %17, align 4, !dbg !4719, !noundef !10
  %_8.i6828 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_97.0, i32 %_97.1, !dbg !4720
  br label %bb1.i.i6829, !dbg !4725

bb1.i.i6829:                                      ; preds = %bb11.i.i6832, %bb5
  %_221.i.i6830 = phi ptr [ %_22.i.i6833, %bb11.i.i6832 ], [ %_97.0, %bb5 ]
  %_12.i.i6831 = icmp eq ptr %_221.i.i6830, %_8.i6828, !dbg !4727
  br i1 %_12.i.i6831, label %bb7, label %bb11.i.i6832, !dbg !4730

bb11.i.i6832:                                     ; preds = %bb1.i.i6829
  %_22.i.i6833 = getelementptr inbounds nuw i8, ptr %_221.i.i6830, i32 16, !dbg !4731
  %18 = getelementptr inbounds nuw i8, ptr %_221.i.i6830, i32 12, !dbg !4733
  %_3.i.i.i6834 = load i32, ptr %18, align 4, !dbg !4733, !alias.scope !4735, !noalias !4740, !noundef !10
  %19 = icmp eq i32 %_3.i.i.i6834, 0, !dbg !4733
  %_51.i.i.i6835 = load i32, ptr %_221.i.i6830, align 4, !dbg !4733, !alias.scope !4735, !noalias !4740
  %20 = getelementptr inbounds nuw i8, ptr %_221.i.i6830, i32 4, !dbg !4733
  %_72.i.i.i6836 = load i32, ptr %20, align 4, !dbg !4733, !alias.scope !4735, !noalias !4740
  %21 = icmp eq i32 %_51.i.i.i6835, %_72.i.i.i6836, !dbg !4733
  %_0.sroa.0.0.off0.i.i.i6837 = select i1 %19, i1 %21, i1 false, !dbg !4733
  br i1 %_0.sroa.0.0.off0.i.i.i6837, label %bb1.i.i6829, label %bb20.thread, !dbg !4743

bb7:                                              ; preds = %bb1.i.i6829
  %22 = getelementptr inbounds nuw i8, ptr %self, i32 1096, !dbg !4744
  %_98.0 = load ptr, ptr %22, align 8, !dbg !4744, !nonnull !10, !noundef !10
  %23 = getelementptr inbounds nuw i8, ptr %self, i32 1100, !dbg !4744
  %_98.1 = load i32, ptr %23, align 4, !dbg !4744, !noundef !10
  %_8.i6839 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_98.0, i32 %_98.1, !dbg !4745
  br label %bb1.i.i6840, !dbg !4750

bb1.i.i6840:                                      ; preds = %bb11.i.i6843, %bb7
  %_221.i.i6841 = phi ptr [ %_22.i.i6844, %bb11.i.i6843 ], [ %_98.0, %bb7 ]
  %_12.i.i6842 = icmp eq ptr %_221.i.i6841, %_8.i6839, !dbg !4752
  br i1 %_12.i.i6842, label %bb9, label %bb11.i.i6843, !dbg !4755

bb11.i.i6843:                                     ; preds = %bb1.i.i6840
  %_22.i.i6844 = getelementptr inbounds nuw i8, ptr %_221.i.i6841, i32 16, !dbg !4756
  %24 = getelementptr inbounds nuw i8, ptr %_221.i.i6841, i32 12, !dbg !4758
  %_3.i.i.i6845 = load i32, ptr %24, align 4, !dbg !4758, !alias.scope !4760, !noalias !4765, !noundef !10
  %25 = icmp eq i32 %_3.i.i.i6845, 0, !dbg !4758
  %_51.i.i.i6846 = load i32, ptr %_221.i.i6841, align 4, !dbg !4758, !alias.scope !4760, !noalias !4765
  %26 = getelementptr inbounds nuw i8, ptr %_221.i.i6841, i32 4, !dbg !4758
  %_72.i.i.i6847 = load i32, ptr %26, align 4, !dbg !4758, !alias.scope !4760, !noalias !4765
  %27 = icmp eq i32 %_51.i.i.i6846, %_72.i.i.i6847, !dbg !4758
  %_0.sroa.0.0.off0.i.i.i6848 = select i1 %25, i1 %27, i1 false, !dbg !4758
  br i1 %_0.sroa.0.0.off0.i.i.i6848, label %bb1.i.i6840, label %bb20.thread, !dbg !4768

bb9:                                              ; preds = %bb1.i.i6840
  %_65.not = icmp ugt i32 %words, %left_io.1
  br i1 %_65.not, label %bb45, label %bb1.i6850, !dbg !4769, !prof !4596

bb45:                                             ; preds = %bb9
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef %words, i32 noundef %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_9d0ae4ec703392629abc127e52af1d06) #32, !dbg !4778
  unreachable, !dbg !4778

bb1.i6850:                                        ; preds = %bb9, %bb12.i6855
  %io.sroa.5.0.i = phi i32 [ %len.i.i.i, %bb12.i6855 ], [ %words, %bb9 ]
  %io.sroa.0.0.i = phi ptr [ %data.i.i.i, %bb12.i6855 ], [ %left_io.0, %bb9 ]
  %28 = icmp eq i32 %io.sroa.5.0.i, 0, !dbg !4779
  br i1 %28, label %bb11, label %bb13.preheader.i, !dbg !4779

bb13.preheader.i:                                 ; preds = %bb1.i6850
  %spec.store.select.i6851 = tail call i32 @llvm.umin.i32(i32 %io.sroa.5.0.i, i32 32), !dbg !4782
  %data.i.i.idx.i = shl nuw nsw i32 %spec.store.select.i6851, 2, !dbg !4785
  %data.i.i.i = getelementptr inbounds nuw i8, ptr %io.sroa.0.0.i, i32 %data.i.i.idx.i, !dbg !4785
  br label %bb13.i6852, !dbg !4790

bb13.i6852:                                       ; preds = %bb13.i6852, %bb13.preheader.i
  %iter.sroa.0.08.i = phi ptr [ %_35.i6853, %bb13.i6852 ], [ %io.sroa.0.0.i, %bb13.preheader.i ]
  %bits.sroa.0.07.i = phi i32 [ %29, %bb13.i6852 ], [ 0, %bb13.preheader.i ]
  %_35.i6853 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i, i32 4, !dbg !4792
  %_95.i = load i32, ptr %iter.sroa.0.08.i, align 4, !dbg !4794, !alias.scope !4795, !noundef !10
  %29 = or i32 %_95.i, %bits.sroa.0.07.i, !dbg !4798
  %_29.i6854 = icmp eq ptr %_35.i6853, %data.i.i.i, !dbg !4799
  br i1 %_29.i6854, label %bb12.i6855, label %bb13.i6852, !dbg !4790

bb12.i6855:                                       ; preds = %bb13.i6852
  %len.i.i.i = sub nuw nsw i32 %io.sroa.5.0.i, %spec.store.select.i6851, !dbg !4801
  %30 = icmp eq i32 %29, 0, !dbg !4802
  br i1 %30, label %bb1.i6850, label %bb20.thread, !dbg !4802

bb11:                                             ; preds = %bb1.i6850
  %_73.not = icmp ugt i32 %words, %right_io.1, !dbg !4803
  br i1 %_73.not, label %bb48, label %bb1.i6857, !dbg !4803, !prof !787

bb20.thread:                                      ; preds = %bb11.i.i, %bb11.i.i6821, %bb11.i.i6832, %bb11.i.i6843, %bb12.i6855, %start
  %31 = getelementptr inbounds nuw i8, ptr %self, i32 1124
  br label %bb26, !dbg !4809

bb20:                                             ; preds = %bb1.i6857
  %32 = getelementptr inbounds nuw i8, ptr %self, i32 1124
  %33 = load i8, ptr %32, align 4, !range !4667
  %_22 = trunc nuw i8 %33 to i1
  br i1 %_22, label %bb22, label %bb26, !dbg !4809

bb48:                                             ; preds = %bb11
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef %words, i32 noundef %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_51ffbab136fc4cbad11477705c6179e2) #32, !dbg !4811
  unreachable, !dbg !4811

bb1.i6857:                                        ; preds = %bb11, %bb12.i6871
  %io.sroa.5.0.i6858 = phi i32 [ %len.i.i.i6864, %bb12.i6871 ], [ %words, %bb11 ]
  %io.sroa.0.0.i6859 = phi ptr [ %data.i.i.i6863, %bb12.i6871 ], [ %right_io.0, %bb11 ]
  %34 = icmp eq i32 %io.sroa.5.0.i6858, 0, !dbg !4812
  br i1 %34, label %bb20, label %bb13.preheader.i6860, !dbg !4812

bb13.preheader.i6860:                             ; preds = %bb1.i6857
  %spec.store.select.i6861 = tail call i32 @llvm.umin.i32(i32 %io.sroa.5.0.i6858, i32 32), !dbg !4815
  %data.i.i.idx.i6862 = shl nuw nsw i32 %spec.store.select.i6861, 2, !dbg !4818
  %data.i.i.i6863 = getelementptr inbounds nuw i8, ptr %io.sroa.0.0.i6859, i32 %data.i.i.idx.i6862, !dbg !4818
  br label %bb13.i6865, !dbg !4823

bb13.i6865:                                       ; preds = %bb13.i6865, %bb13.preheader.i6860
  %iter.sroa.0.08.i6866 = phi ptr [ %_35.i6868, %bb13.i6865 ], [ %io.sroa.0.0.i6859, %bb13.preheader.i6860 ]
  %bits.sroa.0.07.i6867 = phi i32 [ %35, %bb13.i6865 ], [ 0, %bb13.preheader.i6860 ]
  %_35.i6868 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i6866, i32 4, !dbg !4825
  %_95.i6869 = load i32, ptr %iter.sroa.0.08.i6866, align 4, !dbg !4827, !alias.scope !4828, !noundef !10
  %35 = or i32 %_95.i6869, %bits.sroa.0.07.i6867, !dbg !4831
  %_29.i6870 = icmp eq ptr %_35.i6868, %data.i.i.i6863, !dbg !4832
  br i1 %_29.i6870, label %bb12.i6871, label %bb13.i6865, !dbg !4823

bb12.i6871:                                       ; preds = %bb13.i6865
  %len.i.i.i6864 = sub nuw nsw i32 %io.sroa.5.0.i6858, %spec.store.select.i6861, !dbg !4834
  %36 = icmp eq i32 %35, 0, !dbg !4835
  br i1 %36, label %bb1.i6857, label %bb20.thread15847, !dbg !4835

bb20.thread15847:                                 ; preds = %bb12.i6871
  %37 = getelementptr inbounds nuw i8, ptr %self, i32 1124
  br label %bb26, !dbg !4809

bb26:                                             ; preds = %bb20.thread15847, %bb20.thread, %bb20
  %38 = phi ptr [ %31, %bb20.thread ], [ %32, %bb20 ], [ %37, %bb20.thread15847 ]
  %quiet.sroa.0.0.off015846 = phi i1 [ false, %bb20.thread ], [ true, %bb20 ], [ false, %bb20.thread15847 ]
  %_31 = getelementptr inbounds nuw i8, ptr %self, i32 16, !dbg !4836
  %_32 = getelementptr inbounds nuw i8, ptr %self, i32 912, !dbg !4837
  %_33 = getelementptr inbounds nuw i8, ptr %self, i32 924, !dbg !4838
  %_34 = getelementptr inbounds nuw i8, ptr %self, i32 1024, !dbg !4839
  %_35 = getelementptr inbounds nuw i8, ptr %self, i32 800, !dbg !4840
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4841), !dbg !4844
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4847), !dbg !4844
  %_8.i6874 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_95.0, i32 %_95.1, !dbg !4849
  br label %bb1.i.i6875, !dbg !4856

bb1.i.i6875:                                      ; preds = %bb11.i.i6878, %bb26
  %_221.i.i6876 = phi ptr [ %_22.i.i6879, %bb11.i.i6878 ], [ %_95.0, %bb26 ]
  %_12.i.i6877 = icmp eq ptr %_221.i.i6876, %_8.i6874, !dbg !4858
  br i1 %_12.i.i6877, label %bb2.i2091, label %bb11.i.i6878, !dbg !4861

bb11.i.i6878:                                     ; preds = %bb1.i.i6875
  %_22.i.i6879 = getelementptr inbounds nuw i8, ptr %_221.i.i6876, i32 16, !dbg !4862
  %39 = getelementptr inbounds nuw i8, ptr %_221.i.i6876, i32 12, !dbg !4864
  %_3.i.i.i6880 = load i32, ptr %39, align 4, !dbg !4864, !alias.scope !4866, !noalias !4871, !noundef !10
  %40 = icmp eq i32 %_3.i.i.i6880, 0, !dbg !4864
  %_51.i.i.i6881 = load i32, ptr %_221.i.i6876, align 4, !dbg !4864, !alias.scope !4866, !noalias !4871
  %41 = getelementptr inbounds nuw i8, ptr %_221.i.i6876, i32 4, !dbg !4864
  %_72.i.i.i6882 = load i32, ptr %41, align 4, !dbg !4864, !alias.scope !4866, !noalias !4871
  %42 = icmp eq i32 %_51.i.i.i6881, %_72.i.i.i6882, !dbg !4864
  %_0.sroa.0.0.off0.i.i.i6883 = select i1 %40, i1 %42, i1 false, !dbg !4864
  br i1 %_0.sroa.0.0.off0.i.i.i6883, label %bb1.i.i6875, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, !dbg !4880

bb2.i2091:                                        ; preds = %bb1.i.i6875
  %43 = getelementptr inbounds nuw i8, ptr %self, i32 996, !dbg !4881
  %_15.0.i = load ptr, ptr %43, align 4, !dbg !4881, !alias.scope !4841, !noalias !4882, !nonnull !10, !noundef !10
  %44 = getelementptr inbounds nuw i8, ptr %self, i32 1000, !dbg !4881
  %_15.1.i = load i32, ptr %44, align 4, !dbg !4881, !alias.scope !4841, !noalias !4882, !noundef !10
  %_8.i6885 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_15.0.i, i32 %_15.1.i, !dbg !4883
  br label %bb1.i.i6886, !dbg !4888

bb1.i.i6886:                                      ; preds = %bb11.i.i6889, %bb2.i2091
  %_221.i.i6887 = phi ptr [ %_22.i.i6890, %bb11.i.i6889 ], [ %_15.0.i, %bb2.i2091 ]
  %_12.i.i6888 = icmp eq ptr %_221.i.i6887, %_8.i6885, !dbg !4890
  br i1 %_12.i.i6888, label %bb4.i2093, label %bb11.i.i6889, !dbg !4893

bb11.i.i6889:                                     ; preds = %bb1.i.i6886
  %_22.i.i6890 = getelementptr inbounds nuw i8, ptr %_221.i.i6887, i32 16, !dbg !4894
  %45 = getelementptr inbounds nuw i8, ptr %_221.i.i6887, i32 12, !dbg !4896
  %_3.i.i.i6891 = load i32, ptr %45, align 4, !dbg !4896, !alias.scope !4898, !noalias !4903, !noundef !10
  %46 = icmp eq i32 %_3.i.i.i6891, 0, !dbg !4896
  %_51.i.i.i6892 = load i32, ptr %_221.i.i6887, align 4, !dbg !4896, !alias.scope !4898, !noalias !4903
  %47 = getelementptr inbounds nuw i8, ptr %_221.i.i6887, i32 4, !dbg !4896
  %_72.i.i.i6893 = load i32, ptr %47, align 4, !dbg !4896, !alias.scope !4898, !noalias !4903
  %48 = icmp eq i32 %_51.i.i.i6892, %_72.i.i.i6893, !dbg !4896
  %_0.sroa.0.0.off0.i.i.i6894 = select i1 %46, i1 %48, i1 false, !dbg !4896
  br i1 %_0.sroa.0.0.off0.i.i.i6894, label %bb1.i.i6886, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, !dbg !4906

bb4.i2093:                                        ; preds = %bb1.i.i6886
  %49 = getelementptr inbounds nuw i8, ptr %self, i32 1088, !dbg !4907
  %_16.0.i = load ptr, ptr %49, align 4, !dbg !4907, !alias.scope !4847, !noalias !4908, !nonnull !10, !noundef !10
  %50 = getelementptr inbounds nuw i8, ptr %self, i32 1092, !dbg !4907
  %_16.1.i = load i32, ptr %50, align 4, !dbg !4907, !alias.scope !4847, !noalias !4908, !noundef !10
  %_8.i6896 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_16.0.i, i32 %_16.1.i, !dbg !4909
  br label %bb1.i.i6897, !dbg !4914

bb1.i.i6897:                                      ; preds = %bb11.i.i6900, %bb4.i2093
  %_221.i.i6898 = phi ptr [ %_22.i.i6901, %bb11.i.i6900 ], [ %_16.0.i, %bb4.i2093 ]
  %_12.i.i6899 = icmp eq ptr %_221.i.i6898, %_8.i6896, !dbg !4916
  br i1 %_12.i.i6899, label %bb6.i2094, label %bb11.i.i6900, !dbg !4919

bb11.i.i6900:                                     ; preds = %bb1.i.i6897
  %_22.i.i6901 = getelementptr inbounds nuw i8, ptr %_221.i.i6898, i32 16, !dbg !4920
  %51 = getelementptr inbounds nuw i8, ptr %_221.i.i6898, i32 12, !dbg !4922
  %_3.i.i.i6902 = load i32, ptr %51, align 4, !dbg !4922, !alias.scope !4924, !noalias !4929, !noundef !10
  %52 = icmp eq i32 %_3.i.i.i6902, 0, !dbg !4922
  %_51.i.i.i6903 = load i32, ptr %_221.i.i6898, align 4, !dbg !4922, !alias.scope !4924, !noalias !4929
  %53 = getelementptr inbounds nuw i8, ptr %_221.i.i6898, i32 4, !dbg !4922
  %_72.i.i.i6904 = load i32, ptr %53, align 4, !dbg !4922, !alias.scope !4924, !noalias !4929
  %54 = icmp eq i32 %_51.i.i.i6903, %_72.i.i.i6904, !dbg !4922
  %_0.sroa.0.0.off0.i.i.i6905 = select i1 %52, i1 %54, i1 false, !dbg !4922
  br i1 %_0.sroa.0.0.off0.i.i.i6905, label %bb1.i.i6897, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, !dbg !4932

bb6.i2094:                                        ; preds = %bb1.i.i6897
  %55 = getelementptr inbounds nuw i8, ptr %self, i32 1096, !dbg !4933
  %_17.0.i = load ptr, ptr %55, align 4, !dbg !4933, !alias.scope !4847, !noalias !4908, !nonnull !10, !noundef !10
  %56 = getelementptr inbounds nuw i8, ptr %self, i32 1100, !dbg !4933
  %_17.1.i = load i32, ptr %56, align 4, !dbg !4933, !alias.scope !4847, !noalias !4908, !noundef !10
  %_8.i6907 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_17.0.i, i32 %_17.1.i, !dbg !4934
  br label %bb1.i.i6908, !dbg !4939

bb1.i.i6908:                                      ; preds = %bb11.i.i6911, %bb6.i2094
  %_221.i.i6909 = phi ptr [ %_22.i.i6912, %bb11.i.i6911 ], [ %_17.0.i, %bb6.i2094 ]
  %_12.i.i6910 = icmp eq ptr %_221.i.i6909, %_8.i6907, !dbg !4941
  br i1 %_12.i.i6910, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, label %bb11.i.i6911, !dbg !4944

bb11.i.i6911:                                     ; preds = %bb1.i.i6908
  %_22.i.i6912 = getelementptr inbounds nuw i8, ptr %_221.i.i6909, i32 16, !dbg !4945
  %57 = getelementptr inbounds nuw i8, ptr %_221.i.i6909, i32 12, !dbg !4947
  %_3.i.i.i6913 = load i32, ptr %57, align 4, !dbg !4947, !alias.scope !4949, !noalias !4954, !noundef !10
  %58 = icmp eq i32 %_3.i.i.i6913, 0, !dbg !4947
  %_51.i.i.i6914 = load i32, ptr %_221.i.i6909, align 4, !dbg !4947, !alias.scope !4949, !noalias !4954
  %59 = getelementptr inbounds nuw i8, ptr %_221.i.i6909, i32 4, !dbg !4947
  %_72.i.i.i6915 = load i32, ptr %59, align 4, !dbg !4947, !alias.scope !4949, !noalias !4954
  %60 = icmp eq i32 %_51.i.i.i6914, %_72.i.i.i6915, !dbg !4947
  %_0.sroa.0.0.off0.i.i.i6916 = select i1 %58, i1 %60, i1 false, !dbg !4947
  br i1 %_0.sroa.0.0.off0.i.i.i6916, label %bb1.i.i6908, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, !dbg !4957

_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit: ; preds = %bb11.i.i6878, %bb11.i.i6889, %bb11.i.i6900, %bb11.i.i6911, %bb1.i.i6908
  %_0.sroa.0.0.off0.i = phi i1 [ false, %bb11.i.i6900 ], [ false, %bb11.i.i6889 ], [ false, %bb11.i.i6911 ], [ true, %bb1.i.i6908 ], [ false, %bb11.i.i6878 ]
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4958), !dbg !4961
  %61 = getelementptr inbounds nuw i8, ptr %self, i32 1012, !dbg !4963
  %_31.0.i = load ptr, ptr %61, align 4, !dbg !4963, !alias.scope !4958, !noalias !4965, !nonnull !10, !noundef !10
  %62 = getelementptr inbounds nuw i8, ptr %self, i32 1016, !dbg !4963
  %_31.1.i = load i32, ptr %62, align 4, !dbg !4963, !alias.scope !4958, !noalias !4965, !noundef !10
  %_17.idx.i = mul nuw nsw i32 %_31.1.i, 12, !dbg !4966
  %_17.i = getelementptr inbounds nuw i8, ptr %_31.0.i, i32 %_17.idx.i, !dbg !4966
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4970), !dbg !4973, !noalias !4965
  %_5.not.i.i.i = icmp eq i32 %_31.1.i, 0
  %63 = getelementptr inbounds nuw i8, ptr %_31.0.i, i32 4
  %64 = getelementptr inbounds nuw i8, ptr %_31.0.i, i32 8
  br i1 %_5.not.i.i.i, label %bb2.i6929, label %bb1.i.i6918

bb1.i.i6918:                                      ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i
  %_224.i.i = phi ptr [ %_22.i.i6921, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i ], [ %_31.0.i, %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit ]
  %_12.i.i6919 = icmp eq ptr %_224.i.i, %_17.i, !dbg !4974
  br i1 %_12.i.i6919, label %bb2.i6929, label %bb11.i.i6920, !dbg !4978

bb11.i.i6920:                                     ; preds = %bb1.i.i6918
  %_22.i.i6921 = getelementptr inbounds nuw i8, ptr %_224.i.i, i32 12, !dbg !4979
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4981), !dbg !4984, !noalias !4965
  %_9.i.i.i6922 = load i32, ptr %_224.i.i, align 4, !dbg !4985, !alias.scope !4981, !noalias !4988, !noundef !10
  %_10.i.i.i = load i32, ptr %_31.0.i, align 4, !dbg !4985, !alias.scope !4970, !noalias !4990, !noundef !10
  %_8.i.i.i = icmp eq i32 %_9.i.i.i6922, %_10.i.i.i, !dbg !4985
  br i1 %_8.i.i.i, label %bb2.i.i.i, label %bb10.i, !dbg !4985

bb2.i.i.i:                                        ; preds = %bb11.i.i6920
  %65 = getelementptr inbounds nuw i8, ptr %_224.i.i, i32 4, !dbg !4985
  %_12.i.i.i6925 = load i32, ptr %65, align 4, !dbg !4985, !alias.scope !4981, !noalias !4988, !noundef !10
  %_13.i.i.i6926 = load i32, ptr %63, align 4, !dbg !4985, !alias.scope !4970, !noalias !4990, !noundef !10
  %_11.i.i.i = icmp eq i32 %_12.i.i.i6925, %_13.i.i.i6926, !dbg !4985
  br i1 %_11.i.i.i, label %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, label %bb10.i, !dbg !4985

_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i: ; preds = %bb2.i.i.i
  %66 = getelementptr inbounds nuw i8, ptr %_224.i.i, i32 8, !dbg !4985
  %_14.i.i.i6927 = load i32, ptr %66, align 4, !dbg !4985, !alias.scope !4981, !noalias !4988, !noundef !10
  %_15.i.i.i6928 = load i32, ptr %64, align 4, !dbg !4985, !alias.scope !4970, !noalias !4990, !noundef !10
  %67 = icmp eq i32 %_14.i.i.i6927, %_15.i.i.i6928, !dbg !4985
  br i1 %67, label %bb1.i.i6918, label %bb10.i, !dbg !4984

bb2.i6929:                                        ; preds = %bb1.i.i6918, %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit
  %68 = getelementptr inbounds nuw i8, ptr %self, i32 980, !dbg !4991
  %_32.0.i = load ptr, ptr %68, align 4, !dbg !4991, !alias.scope !4958, !noalias !4965, !nonnull !10, !noundef !10
  %69 = getelementptr inbounds nuw i8, ptr %self, i32 984, !dbg !4991
  %_32.1.i = load i32, ptr %69, align 4, !dbg !4991, !alias.scope !4958, !noalias !4965, !noundef !10
  %_26.idx.i = shl nuw nsw i32 %_32.1.i, 2, !dbg !4992
  %_26.i = getelementptr inbounds nuw i8, ptr %_32.0.i, i32 %_26.idx.i, !dbg !4992
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4996), !dbg !4999, !noalias !4965
  %_6.not.i.i.i = icmp eq i32 %_32.1.i, 0
  br i1 %_6.not.i.i.i, label %bb3.i, label %bb1.i3.i

bb1.i3.i:                                         ; preds = %bb2.i6929, %bb11.i5.i
  %_223.i.i = phi ptr [ %_22.i6.i, %bb11.i5.i ], [ %_32.0.i, %bb2.i6929 ]
  %_12.i4.i = icmp eq ptr %_223.i.i, %_26.i, !dbg !5000
  br i1 %_12.i4.i, label %bb3.i, label %bb11.i5.i, !dbg !5004

bb11.i5.i:                                        ; preds = %bb1.i3.i
  %_22.i6.i = getelementptr inbounds nuw i8, ptr %_223.i.i, i32 4, !dbg !5005
  %ptr.val.i.i = load i32, ptr %_223.i.i, align 4, !dbg !5007, !noalias !5008
  %_4.i.i.i6930 = load i32, ptr %_32.0.i, align 4, !dbg !5010, !alias.scope !4996, !noalias !5012, !noundef !10
  %_0.i.i.i = icmp eq i32 %ptr.val.i.i, %_4.i.i.i6930, !dbg !5013
  br i1 %_0.i.i.i, label %bb1.i3.i, label %bb10.i, !dbg !5007

bb3.i:                                            ; preds = %bb1.i3.i, %bb2.i6929
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5014), !dbg !5017
  %70 = getelementptr inbounds nuw i8, ptr %self, i32 1112, !dbg !5018
  %_31.0.i6931 = load ptr, ptr %70, align 4, !dbg !5018, !alias.scope !5014, !noalias !4965, !nonnull !10, !noundef !10
  %71 = getelementptr inbounds nuw i8, ptr %self, i32 1116, !dbg !5018
  %_31.1.i6932 = load i32, ptr %71, align 4, !dbg !5018, !alias.scope !5014, !noalias !4965, !noundef !10
  %_17.idx.i6933 = mul nuw nsw i32 %_31.1.i6932, 12, !dbg !5020
  %_17.i6934 = getelementptr inbounds nuw i8, ptr %_31.0.i6931, i32 %_17.idx.i6933, !dbg !5020
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5024), !dbg !5027, !noalias !4965
  %_5.not.i.i.i6935 = icmp eq i32 %_31.1.i6932, 0
  %72 = getelementptr inbounds nuw i8, ptr %_31.0.i6931, i32 4
  %73 = getelementptr inbounds nuw i8, ptr %_31.0.i6931, i32 8
  br i1 %_5.not.i.i.i6935, label %bb2.i6953, label %bb1.i.i6936

bb1.i.i6936:                                      ; preds = %bb3.i, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i6950
  %_224.i.i6937 = phi ptr [ %_22.i.i6940, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i6950 ], [ %_31.0.i6931, %bb3.i ]
  %_12.i.i6938 = icmp eq ptr %_224.i.i6937, %_17.i6934, !dbg !5028
  br i1 %_12.i.i6938, label %bb2.i6953, label %bb11.i.i6939, !dbg !5032

bb11.i.i6939:                                     ; preds = %bb1.i.i6936
  %_22.i.i6940 = getelementptr inbounds nuw i8, ptr %_224.i.i6937, i32 12, !dbg !5033
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5035), !dbg !5038, !noalias !4965
  %_9.i.i.i6941 = load i32, ptr %_224.i.i6937, align 4, !dbg !5039, !alias.scope !5035, !noalias !5042, !noundef !10
  %_10.i.i.i6942 = load i32, ptr %_31.0.i6931, align 4, !dbg !5039, !alias.scope !5024, !noalias !5044, !noundef !10
  %_8.i.i.i6943 = icmp eq i32 %_9.i.i.i6941, %_10.i.i.i6942, !dbg !5039
  br i1 %_8.i.i.i6943, label %bb2.i.i.i6946, label %bb10.i, !dbg !5039

bb2.i.i.i6946:                                    ; preds = %bb11.i.i6939
  %74 = getelementptr inbounds nuw i8, ptr %_224.i.i6937, i32 4, !dbg !5039
  %_12.i.i.i6947 = load i32, ptr %74, align 4, !dbg !5039, !alias.scope !5035, !noalias !5042, !noundef !10
  %_13.i.i.i6948 = load i32, ptr %72, align 4, !dbg !5039, !alias.scope !5024, !noalias !5044, !noundef !10
  %_11.i.i.i6949 = icmp eq i32 %_12.i.i.i6947, %_13.i.i.i6948, !dbg !5039
  br i1 %_11.i.i.i6949, label %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i6950, label %bb10.i, !dbg !5039

_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i6950: ; preds = %bb2.i.i.i6946
  %75 = getelementptr inbounds nuw i8, ptr %_224.i.i6937, i32 8, !dbg !5039
  %_14.i.i.i6951 = load i32, ptr %75, align 4, !dbg !5039, !alias.scope !5035, !noalias !5042, !noundef !10
  %_15.i.i.i6952 = load i32, ptr %73, align 4, !dbg !5039, !alias.scope !5024, !noalias !5044, !noundef !10
  %76 = icmp eq i32 %_14.i.i.i6951, %_15.i.i.i6952, !dbg !5039
  br i1 %76, label %bb1.i.i6936, label %bb10.i, !dbg !5038

bb2.i6953:                                        ; preds = %bb1.i.i6936, %bb3.i
  %77 = getelementptr inbounds nuw i8, ptr %self, i32 1080, !dbg !5045
  %_32.0.i6954 = load ptr, ptr %77, align 4, !dbg !5045, !alias.scope !5014, !noalias !4965, !nonnull !10, !noundef !10
  %78 = getelementptr inbounds nuw i8, ptr %self, i32 1084, !dbg !5045
  %_32.1.i6955 = load i32, ptr %78, align 4, !dbg !5045, !alias.scope !5014, !noalias !4965, !noundef !10
  %_26.idx.i6956 = shl nuw nsw i32 %_32.1.i6955, 2, !dbg !5046
  %_26.i6957 = getelementptr inbounds nuw i8, ptr %_32.0.i6954, i32 %_26.idx.i6956, !dbg !5046
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5050), !dbg !5053, !noalias !4965
  %_6.not.i.i.i6958 = icmp eq i32 %_32.1.i6955, 0
  br i1 %_6.not.i.i.i6958, label %bb5.i, label %bb1.i3.i6959

bb1.i3.i6959:                                     ; preds = %bb2.i6953, %bb11.i5.i6962
  %_223.i.i6960 = phi ptr [ %_22.i6.i6963, %bb11.i5.i6962 ], [ %_32.0.i6954, %bb2.i6953 ]
  %_12.i4.i6961 = icmp eq ptr %_223.i.i6960, %_26.i6957, !dbg !5054
  br i1 %_12.i4.i6961, label %bb5.i, label %bb11.i5.i6962, !dbg !5058

bb11.i5.i6962:                                    ; preds = %bb1.i3.i6959
  %_22.i6.i6963 = getelementptr inbounds nuw i8, ptr %_223.i.i6960, i32 4, !dbg !5059
  %ptr.val.i.i6964 = load i32, ptr %_223.i.i6960, align 4, !dbg !5061, !noalias !5062
  %_4.i.i.i6965 = load i32, ptr %_32.0.i6954, align 4, !dbg !5064, !alias.scope !5050, !noalias !5066, !noundef !10
  %_0.i.i.i6966 = icmp eq i32 %ptr.val.i.i6964, %_4.i.i.i6965, !dbg !5067
  br i1 %_0.i.i.i6966, label %bb1.i3.i6959, label %bb10.i, !dbg !5061

bb10.i:                                           ; preds = %bb11.i.i6920, %bb2.i.i.i, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, %bb11.i5.i, %bb11.i.i6939, %bb2.i.i.i6946, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i6950, %bb11.i5.i6962
  %79 = getelementptr inbounds nuw i8, ptr %self, i32 784, !dbg !5068
  %80 = getelementptr inbounds nuw i8, ptr %self, i32 785, !dbg !5068
  %81 = getelementptr inbounds nuw i8, ptr %self, i32 804, !dbg !5068
  %_115.not.i18696 = icmp eq i32 %frames, 0, !dbg !5068
  br i1 %_0.sroa.0.0.off0.i, label %bb11.i, label %bb12.i, !dbg !5069

bb5.i:                                            ; preds = %bb1.i3.i6959, %bb2.i6953
  %82 = getelementptr inbounds nuw i8, ptr %self, i32 784, !dbg !5068
  %83 = getelementptr inbounds nuw i8, ptr %self, i32 785, !dbg !5068
  %84 = getelementptr inbounds nuw i8, ptr %self, i32 916, !dbg !5068
  %85 = getelementptr inbounds nuw i8, ptr %self, i32 920, !dbg !5068
  %86 = getelementptr inbounds nuw i8, ptr %self, i32 804, !dbg !5068
  br i1 %_0.sroa.0.0.off0.i, label %bb6.i, label %bb7.i, !dbg !5070

bb12.i:                                           ; preds = %bb10.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5071), !dbg !5074
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5075), !dbg !5074
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5077), !dbg !5074
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5079), !dbg !5074
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5081), !dbg !5074
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_(ptr noalias noundef align 16 captures(none) dereferenceable(368) %hot_left.i1091, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #31, !dbg !5083
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_(ptr noalias noundef align 16 captures(none) dereferenceable(368) %hot_right.i1090, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #31, !dbg !5087
  %87 = load i8, ptr %79, align 16, !dbg !5089, !range !4667, !alias.scope !5071, !noalias !5093, !noundef !10
  %88 = load i8, ptr %80, align 1, !dbg !5096, !range !4667, !alias.scope !5071, !noalias !5093, !noundef !10
  %_35.i1099 = load i32, ptr %_35, align 4, !dbg !5098, !alias.scope !5081, !noalias !5100, !noundef !10
  %_37.i1100 = load i32, ptr %81, align 4, !dbg !5101, !alias.scope !5081, !noalias !5100, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i1088), !dbg !5103, !noalias !5105
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i1088, i8 0, i32 32, i1 false), !noalias !5105
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i1087), !dbg !5106, !noalias !5105
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i1087, i8 0, i32 1024, i1 false), !noalias !5105
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i1086), !dbg !5108, !noalias !5105
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i1086, i8 0, i32 1024, i1 false), !noalias !5105
  br i1 %_115.not.i18696, label %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb37.i1114.lr.ph, !dbg !5110

bb37.i1114.lr.ph:                                 ; preds = %bb12.i
  %_33.i1097 = trunc nuw i8 %88 to i1, !dbg !5096
  %spec.store.select27.i1098 = select i1 %_33.i1097, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !5096
  %_32.i1095 = trunc nuw i8 %87 to i1, !dbg !5089
  %link.sroa.0.0.i1096 = select i1 %_32.i1095, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !5089
  %d9.i = lshr i32 %frames, 5, !dbg !5124
  %r2.i = and i32 %frames, 31, !dbg !5136
  %_19.not.i = icmp ne i32 %r2.i, 0, !dbg !5138
  %89 = zext i1 %_19.not.i to i32, !dbg !5138
  %yield_count.sroa.0.0.i = add nuw nsw i32 %d9.i, %89, !dbg !5138
  %history.i142.i1062.sroa.7.0.hot_left.i1091.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1091, i32 16
  %history.i142.i1062.sroa.10.0.hot_left.i1091.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1091, i32 32
  %history.i142.i1062.sroa.13.0.hot_left.i1091.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1091, i32 48
  %history.i142.i1062.sroa.16.0.hot_left.i1091.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1091, i32 64
  %history.i142.i1062.sroa.19.0.hot_left.i1091.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1091, i32 80
  %history.i142.i1062.sroa.22.0.hot_left.i1091.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1091, i32 96
  %history.i142.i1062.sroa.26.0.hot_left.i1091.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1091, i32 112
  %history.i142.i1062.sroa.29.0.hot_left.i1091.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1091, i32 128
  %history.i142.i1062.sroa.32.0.hot_left.i1091.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1091, i32 144
  %history.i142.i1062.sroa.35.0.hot_left.i1091.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1091, i32 160
  %history.i142.i1062.sroa.38.0.hot_left.i1091.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i1091, i32 176
  %90 = getelementptr inbounds nuw i8, ptr %self, i32 32
  %91 = getelementptr inbounds nuw i8, ptr %self, i32 48
  %92 = getelementptr inbounds nuw i8, ptr %self, i32 64
  %row1.i.i.i175.i1156 = getelementptr inbounds nuw i8, ptr %self, i32 80
  %93 = getelementptr inbounds nuw i8, ptr %self, i32 96
  %94 = getelementptr inbounds nuw i8, ptr %self, i32 112
  %95 = getelementptr inbounds nuw i8, ptr %self, i32 128
  %row3.i.i.i189.i1170 = getelementptr inbounds nuw i8, ptr %self, i32 144
  %96 = getelementptr inbounds nuw i8, ptr %self, i32 160
  %97 = getelementptr inbounds nuw i8, ptr %self, i32 176
  %98 = getelementptr inbounds nuw i8, ptr %self, i32 192
  %row5.i.i.i203.i1184 = getelementptr inbounds nuw i8, ptr %self, i32 208
  %99 = getelementptr inbounds nuw i8, ptr %self, i32 224
  %100 = getelementptr inbounds nuw i8, ptr %self, i32 240
  %101 = getelementptr inbounds nuw i8, ptr %self, i32 256
  %row7.i.i.i217.i1198 = getelementptr inbounds nuw i8, ptr %self, i32 272
  %102 = getelementptr inbounds nuw i8, ptr %self, i32 288
  %103 = getelementptr inbounds nuw i8, ptr %self, i32 304
  %104 = getelementptr inbounds nuw i8, ptr %self, i32 320
  %row9.i.i.i231.i1212 = getelementptr inbounds nuw i8, ptr %self, i32 336
  %105 = getelementptr inbounds nuw i8, ptr %self, i32 352
  %106 = getelementptr inbounds nuw i8, ptr %self, i32 368
  %107 = getelementptr inbounds nuw i8, ptr %self, i32 384
  %row11.i.i.i245.i1226 = getelementptr inbounds nuw i8, ptr %self, i32 400
  %108 = getelementptr inbounds nuw i8, ptr %self, i32 416
  %109 = getelementptr inbounds nuw i8, ptr %self, i32 432
  %110 = getelementptr inbounds nuw i8, ptr %self, i32 448
  %row13.i.i.i259.i1240 = getelementptr inbounds nuw i8, ptr %self, i32 464
  %111 = getelementptr inbounds nuw i8, ptr %self, i32 480
  %112 = getelementptr inbounds nuw i8, ptr %self, i32 496
  %113 = getelementptr inbounds nuw i8, ptr %self, i32 512
  %row15.i.i.i273.i1254 = getelementptr inbounds nuw i8, ptr %self, i32 528
  %114 = getelementptr inbounds nuw i8, ptr %self, i32 544
  %115 = getelementptr inbounds nuw i8, ptr %self, i32 560
  %116 = getelementptr inbounds nuw i8, ptr %self, i32 576
  %row17.i.i.i287.i1268 = getelementptr inbounds nuw i8, ptr %self, i32 592
  %117 = getelementptr inbounds nuw i8, ptr %self, i32 608
  %118 = getelementptr inbounds nuw i8, ptr %self, i32 624
  %119 = getelementptr inbounds nuw i8, ptr %self, i32 640
  %row19.i.i.i301.i1282 = getelementptr inbounds nuw i8, ptr %self, i32 656
  %120 = getelementptr inbounds nuw i8, ptr %self, i32 672
  %121 = getelementptr inbounds nuw i8, ptr %self, i32 688
  %122 = getelementptr inbounds nuw i8, ptr %self, i32 704
  %row21.i.i.i315.i1296 = getelementptr inbounds nuw i8, ptr %self, i32 720
  %123 = getelementptr inbounds nuw i8, ptr %self, i32 736
  %124 = getelementptr inbounds nuw i8, ptr %self, i32 752
  %125 = getelementptr inbounds nuw i8, ptr %self, i32 768
  %history.i.i1081.sroa.7.0.hot_right.i1090.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1090, i32 16
  %history.i.i1081.sroa.10.0.hot_right.i1090.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1090, i32 32
  %history.i.i1081.sroa.13.0.hot_right.i1090.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1090, i32 48
  %history.i.i1081.sroa.16.0.hot_right.i1090.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1090, i32 64
  %history.i.i1081.sroa.19.0.hot_right.i1090.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1090, i32 80
  %history.i.i1081.sroa.22.0.hot_right.i1090.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1090, i32 96
  %history.i.i1081.sroa.26.0.hot_right.i1090.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1090, i32 112
  %history.i.i1081.sroa.29.0.hot_right.i1090.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1090, i32 128
  %history.i.i1081.sroa.32.0.hot_right.i1090.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1090, i32 144
  %history.i.i1081.sroa.35.0.hot_right.i1090.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1090, i32 160
  %history.i.i1081.sroa.38.0.hot_right.i1090.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i1090, i32 176
  %_68.i1529 = getelementptr inbounds nuw i8, ptr %hot_left.i1091, i32 192
  %_69.i1530 = getelementptr inbounds nuw i8, ptr %hot_left.i1091, i32 256
  %126 = getelementptr inbounds nuw i8, ptr %hot_left.i1091, i32 240
  %127 = getelementptr inbounds nuw i8, ptr %hot_left.i1091, i32 224
  %128 = getelementptr inbounds nuw i8, ptr %hot_left.i1091, i32 208
  %129 = getelementptr inbounds nuw i8, ptr %hot_left.i1091, i32 304
  %130 = getelementptr inbounds nuw i8, ptr %hot_left.i1091, i32 288
  %131 = getelementptr inbounds nuw i8, ptr %hot_left.i1091, i32 272
  %_73.i1533 = getelementptr inbounds nuw i8, ptr %hot_right.i1090, i32 192
  %_74.i1534 = getelementptr inbounds nuw i8, ptr %hot_right.i1090, i32 256
  %132 = getelementptr inbounds nuw i8, ptr %hot_right.i1090, i32 240
  %133 = getelementptr inbounds nuw i8, ptr %hot_right.i1090, i32 224
  %134 = getelementptr inbounds nuw i8, ptr %hot_right.i1090, i32 208
  %135 = getelementptr inbounds nuw i8, ptr %hot_right.i1090, i32 304
  %136 = getelementptr inbounds nuw i8, ptr %hot_right.i1090, i32 288
  %137 = getelementptr inbounds nuw i8, ptr %hot_right.i1090, i32 272
  %138 = bitcast <4 x i32> %link.sroa.0.0.i1096 to <16 x i8>
  %139 = getelementptr inbounds nuw i8, ptr %self, i32 916
  %140 = getelementptr inbounds nuw i8, ptr %self, i32 1020
  %141 = getelementptr inbounds nuw i8, ptr %self, i32 944
  %142 = getelementptr inbounds nuw i8, ptr %self, i32 940
  %143 = getelementptr inbounds nuw i8, ptr %self, i32 984
  %144 = getelementptr inbounds nuw i8, ptr %self, i32 980
  %145 = getelementptr inbounds nuw i8, ptr %self, i32 968
  %146 = getelementptr inbounds nuw i8, ptr %self, i32 964
  %147 = getelementptr inbounds nuw i8, ptr %self, i32 952
  %148 = getelementptr inbounds nuw i8, ptr %self, i32 948
  %149 = getelementptr inbounds nuw i8, ptr %hot_left.i1091, i32 336
  %150 = getelementptr inbounds nuw i8, ptr %hot_left.i1091, i32 352
  %151 = getelementptr inbounds nuw i8, ptr %hot_left.i1091, i32 320
  %152 = getelementptr inbounds nuw i8, ptr %self, i32 936
  %153 = getelementptr inbounds nuw i8, ptr %self, i32 932
  %154 = bitcast <4 x i32> %spec.store.select27.i1098 to <16 x i8>
  %155 = getelementptr inbounds nuw i8, ptr %self, i32 1120
  %156 = getelementptr inbounds nuw i8, ptr %self, i32 1044
  %157 = getelementptr inbounds nuw i8, ptr %self, i32 1040
  %158 = getelementptr inbounds nuw i8, ptr %self, i32 1116
  %159 = getelementptr inbounds nuw i8, ptr %self, i32 1112
  %160 = getelementptr inbounds nuw i8, ptr %self, i32 1084
  %161 = getelementptr inbounds nuw i8, ptr %self, i32 1080
  %162 = getelementptr inbounds nuw i8, ptr %self, i32 1068
  %163 = getelementptr inbounds nuw i8, ptr %self, i32 1064
  %164 = getelementptr inbounds nuw i8, ptr %self, i32 1052
  %165 = getelementptr inbounds nuw i8, ptr %self, i32 1048
  %166 = getelementptr inbounds nuw i8, ptr %hot_right.i1090, i32 336
  %167 = getelementptr inbounds nuw i8, ptr %hot_right.i1090, i32 352
  %168 = getelementptr inbounds nuw i8, ptr %hot_right.i1090, i32 320
  %169 = getelementptr inbounds nuw i8, ptr %self, i32 1036
  %170 = getelementptr inbounds nuw i8, ptr %self, i32 1032
  %171 = getelementptr inbounds nuw i8, ptr %self, i32 920
  %iter.sroa.0.0.ptr.i52.i157918439.1 = getelementptr inbounds nuw i8, ptr %scratch.i1088, i32 4
  %iter.sroa.0.0.ptr.i52.i157918439.2 = getelementptr inbounds nuw i8, ptr %scratch.i1088, i32 8
  %iter.sroa.0.0.ptr.i52.i157918439.3 = getelementptr inbounds nuw i8, ptr %scratch.i1088, i32 12
  %iter.sroa.0.0.ptr.i52.i157918439.4 = getelementptr inbounds nuw i8, ptr %scratch.i1088, i32 16
  %iter.sroa.0.0.ptr.i52.i157918439.5 = getelementptr inbounds nuw i8, ptr %scratch.i1088, i32 20
  %iter.sroa.0.0.ptr.i52.i157918439.6 = getelementptr inbounds nuw i8, ptr %scratch.i1088, i32 24
  %iter.sroa.0.0.ptr.i52.i157918439.7 = getelementptr inbounds nuw i8, ptr %scratch.i1088, i32 28
  %iter.sroa.0.0.ptr.i.i167418451.1 = getelementptr inbounds nuw i8, ptr %scratch.i1088, i32 4
  %iter.sroa.0.0.ptr.i.i167418451.2 = getelementptr inbounds nuw i8, ptr %scratch.i1088, i32 8
  %iter.sroa.0.0.ptr.i.i167418451.3 = getelementptr inbounds nuw i8, ptr %scratch.i1088, i32 12
  %iter.sroa.0.0.ptr.i.i167418451.4 = getelementptr inbounds nuw i8, ptr %scratch.i1088, i32 16
  %iter.sroa.0.0.ptr.i.i167418451.5 = getelementptr inbounds nuw i8, ptr %scratch.i1088, i32 20
  %iter.sroa.0.0.ptr.i.i167418451.6 = getelementptr inbounds nuw i8, ptr %scratch.i1088, i32 24
  %iter.sroa.0.0.ptr.i.i167418451.7 = getelementptr inbounds nuw i8, ptr %scratch.i1088, i32 28
  br label %bb37.i1114, !dbg !5110

bb13.i1108.loopexit.loopexit:                     ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %503, ptr %149, align 16, !dbg !5168
  store <4 x float> %593, ptr %166, align 16, !dbg !5183
  br label %bb13.i1108.loopexit, !dbg !5110

bb13.i1108.loopexit:                              ; preds = %bb13.i1108.loopexit.loopexit, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i1520
  %ring_cursor.sroa.0.1.i1523.lcssa = phi i32 [ %ring_cursor.sroa.0.0.i111118461, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i1520 ], [ %spec.store.select12.i1745, %bb13.i1108.loopexit.loopexit ], !dbg !5185
  %main_cursor.sroa.0.1.i1524.lcssa = phi i32 [ %main_cursor.sroa.0.0.i111218462, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i1520 ], [ %spec.store.select11.i1743, %bb13.i1108.loopexit.loopexit ], !dbg !5186
  %_115.not.i1113 = icmp eq i32 %174, 0, !dbg !5110
  %indvars.iv.next = add i32 %indvars.iv, -32, !dbg !5110
  br i1 %_115.not.i1113, label %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb37.i1114, !dbg !5110

bb37.i1114:                                       ; preds = %bb37.i1114.lr.ph, %bb13.i1108.loopexit
  %indvars.iv = phi i32 [ %frames, %bb37.i1114.lr.ph ], [ %indvars.iv.next, %bb13.i1108.loopexit ]
  %main_cursor.sroa.0.0.i111218462 = phi i32 [ %_35.i1099, %bb37.i1114.lr.ph ], [ %main_cursor.sroa.0.1.i1524.lcssa, %bb13.i1108.loopexit ]
  %ring_cursor.sroa.0.0.i111118461 = phi i32 [ %_37.i1100, %bb37.i1114.lr.ph ], [ %ring_cursor.sroa.0.1.i1523.lcssa, %bb13.i1108.loopexit ]
  %iter2.sroa.0.0.i111018460 = phi i32 [ %yield_count.sroa.0.0.i, %bb37.i1114.lr.ph ], [ %174, %bb13.i1108.loopexit ]
  %iter.sroa.0.0.i110918459 = phi i32 [ 0, %bb37.i1114.lr.ph ], [ %173, %bb13.i1108.loopexit ]
  %172 = call i32 @llvm.umax.i32(i32 %indvars.iv, i32 1), !dbg !5187
  %umax20918 = call i32 @llvm.umin.i32(i32 %172, i32 32), !dbg !5187
  %173 = add i32 %iter.sroa.0.0.i110918459, 32, !dbg !5187
  %174 = add nsw i32 %iter2.sroa.0.0.i111018460, -1, !dbg !5191
  %175 = sub i32 %frames, %iter.sroa.0.0.i110918459, !dbg !5192
  %spec.store.select.i1115 = tail call i32 @llvm.umin.i32(i32 %175, i32 32), !dbg !5193
  %active_base.i1116 = shl i32 %iter.sroa.0.0.i110918459, 2, !dbg !5198
  %active_base.i111615978 = add i32 %spec.store.select.i1115, %iter.sroa.0.0.i110918459, !dbg !5199
  %_51.i1118 = shl i32 %active_base.i111615978, 2, !dbg !5199
  %_125.i1119 = icmp ult i32 %_51.i1118, %active_base.i1116, !dbg !5200
  %_119.not.i1120 = icmp ugt i32 %_51.i1118, %left_io.1
  %or.cond.i1121 = or i1 %_125.i1119, %_119.not.i1120, !dbg !5200
  br i1 %or.cond.i1121, label %bb43.i1760, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit, !dbg !5200, !prof !4596

bb43.i1760:                                       ; preds = %bb37.i1114
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %active_base.i1116, i32 noundef %_51.i1118, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_56e6f2edba81ce75fd22cd243776a278) #32, !dbg !5207, !noalias !5208
  unreachable, !dbg !5207

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit: ; preds = %bb37.i1114
  %_128.i1123 = getelementptr inbounds nuw float, ptr %left_io.0, i32 %active_base.i1116, !dbg !5209
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5213), !dbg !5216
  %history.i142.i1062.sroa.0.0.copyload = load <4 x i32>, ptr %hot_left.i1091, align 16, !dbg !5217
  %history.i142.i1062.sroa.7.0.copyload = load <4 x i32>, ptr %history.i142.i1062.sroa.7.0.hot_left.i1091.sroa_idx, align 16, !dbg !5217
  %history.i142.i1062.sroa.10.0.copyload = load <4 x i32>, ptr %history.i142.i1062.sroa.10.0.hot_left.i1091.sroa_idx, align 16, !dbg !5217
  %history.i142.i1062.sroa.13.0.copyload = load <4 x i32>, ptr %history.i142.i1062.sroa.13.0.hot_left.i1091.sroa_idx, align 16, !dbg !5217
  %history.i142.i1062.sroa.16.0.copyload = load <4 x i32>, ptr %history.i142.i1062.sroa.16.0.hot_left.i1091.sroa_idx, align 16, !dbg !5217
  %history.i142.i1062.sroa.19.0.copyload = load <4 x i32>, ptr %history.i142.i1062.sroa.19.0.hot_left.i1091.sroa_idx, align 16, !dbg !5217
  %history.i142.i1062.sroa.22.0.copyload = load <4 x i32>, ptr %history.i142.i1062.sroa.22.0.hot_left.i1091.sroa_idx, align 16, !dbg !5217
  %history.i142.i1062.sroa.26.0.copyload = load <4 x i32>, ptr %history.i142.i1062.sroa.26.0.hot_left.i1091.sroa_idx, align 16, !dbg !5217
  %history.i142.i1062.sroa.29.0.copyload = load <4 x i32>, ptr %history.i142.i1062.sroa.29.0.hot_left.i1091.sroa_idx, align 16, !dbg !5217
  %history.i142.i1062.sroa.32.0.copyload = load <4 x i32>, ptr %history.i142.i1062.sroa.32.0.hot_left.i1091.sroa_idx, align 16, !dbg !5217
  %history.i142.i1062.sroa.35.0.copyload = load <4 x i32>, ptr %history.i142.i1062.sroa.35.0.hot_left.i1091.sroa_idx, align 16, !dbg !5217
  %history.i142.i1062.sroa.38.0.copyload = load <4 x i32>, ptr %history.i142.i1062.sroa.38.0.hot_left.i1091.sroa_idx, align 16, !dbg !5217
  %_2.i697418377.not = icmp eq i32 %frames, %iter.sroa.0.0.i110918459, !dbg !5221
  br i1 %_2.i697418377.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit339.i1320, label %bb5.i145.i1126.lr.ph, !dbg !5221

bb5.i145.i1126.lr.ph:                             ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit
  %_11.i.i.i163.i114416069 = load <4 x float>, ptr %_31, align 16, !alias.scope !5229, !noalias !5234
  %_14.i.i.i166.i114716070 = load <4 x float>, ptr %90, align 16, !alias.scope !5229, !noalias !5234
  %_17.i.i.i169.i115016071 = load <4 x float>, ptr %91, align 16, !alias.scope !5229, !noalias !5234
  %_20.i.i.i172.i115316072 = load <4 x float>, ptr %92, align 16, !alias.scope !5229, !noalias !5234
  %_25.i.i.i177.i115816073 = load <4 x float>, ptr %row1.i.i.i175.i1156, align 16, !alias.scope !5229, !noalias !5234
  %_28.i.i.i180.i116116074 = load <4 x float>, ptr %93, align 16, !alias.scope !5229, !noalias !5234
  %_31.i.i.i183.i116416075 = load <4 x float>, ptr %94, align 16, !alias.scope !5229, !noalias !5234
  %_34.i.i.i186.i116716076 = load <4 x float>, ptr %95, align 16, !alias.scope !5229, !noalias !5234
  %_39.i.i.i191.i117216077 = load <4 x float>, ptr %row3.i.i.i189.i1170, align 16, !alias.scope !5229, !noalias !5234
  %_42.i.i.i194.i117516078 = load <4 x float>, ptr %96, align 16, !alias.scope !5229, !noalias !5234
  %_45.i.i.i197.i117816079 = load <4 x float>, ptr %97, align 16, !alias.scope !5229, !noalias !5234
  %_48.i.i.i200.i118116080 = load <4 x float>, ptr %98, align 16, !alias.scope !5229, !noalias !5234
  %_53.i.i.i205.i118616081 = load <4 x float>, ptr %row5.i.i.i203.i1184, align 16, !alias.scope !5229, !noalias !5234
  %_56.i.i.i208.i118916082 = load <4 x float>, ptr %99, align 16, !alias.scope !5229, !noalias !5234
  %_59.i.i.i211.i119216083 = load <4 x float>, ptr %100, align 16, !alias.scope !5229, !noalias !5234
  %_62.i.i.i214.i119516084 = load <4 x float>, ptr %101, align 16, !alias.scope !5229, !noalias !5234
  %_67.i.i.i219.i120016085 = load <4 x float>, ptr %row7.i.i.i217.i1198, align 16, !alias.scope !5229, !noalias !5234
  %_70.i.i.i222.i120316086 = load <4 x float>, ptr %102, align 16, !alias.scope !5229, !noalias !5234
  %_73.i.i.i225.i120616087 = load <4 x float>, ptr %103, align 16, !alias.scope !5229, !noalias !5234
  %_76.i.i.i228.i120916088 = load <4 x float>, ptr %104, align 16, !alias.scope !5229, !noalias !5234
  %_81.i.i.i233.i121416089 = load <4 x float>, ptr %row9.i.i.i231.i1212, align 16, !alias.scope !5229, !noalias !5234
  %_84.i.i.i236.i121716090 = load <4 x float>, ptr %105, align 16, !alias.scope !5229, !noalias !5234
  %_87.i.i.i239.i122016091 = load <4 x float>, ptr %106, align 16, !alias.scope !5229, !noalias !5234
  %_90.i.i.i242.i122316092 = load <4 x float>, ptr %107, align 16, !alias.scope !5229, !noalias !5234
  %_95.i.i.i247.i122816093 = load <4 x float>, ptr %row11.i.i.i245.i1226, align 16, !alias.scope !5229, !noalias !5234
  %_98.i.i.i250.i123116094 = load <4 x float>, ptr %108, align 16, !alias.scope !5229, !noalias !5234
  %_101.i.i.i253.i123416095 = load <4 x float>, ptr %109, align 16, !alias.scope !5229, !noalias !5234
  %_104.i.i.i256.i123716096 = load <4 x float>, ptr %110, align 16, !alias.scope !5229, !noalias !5234
  %_109.i.i.i261.i124216097 = load <4 x float>, ptr %row13.i.i.i259.i1240, align 16, !alias.scope !5229, !noalias !5234
  %_112.i.i.i264.i124516098 = load <4 x float>, ptr %111, align 16, !alias.scope !5229, !noalias !5234
  %_115.i.i.i267.i124816099 = load <4 x float>, ptr %112, align 16, !alias.scope !5229, !noalias !5234
  %_118.i.i.i270.i125116100 = load <4 x float>, ptr %113, align 16, !alias.scope !5229, !noalias !5234
  %_123.i.i.i275.i125616101 = load <4 x float>, ptr %row15.i.i.i273.i1254, align 16, !alias.scope !5229, !noalias !5234
  %_126.i.i.i278.i125916102 = load <4 x float>, ptr %114, align 16, !alias.scope !5229, !noalias !5234
  %_129.i.i.i281.i126216103 = load <4 x float>, ptr %115, align 16, !alias.scope !5229, !noalias !5234
  %_132.i.i.i284.i126516104 = load <4 x float>, ptr %116, align 16, !alias.scope !5229, !noalias !5234
  %_137.i.i.i289.i127016105 = load <4 x float>, ptr %row17.i.i.i287.i1268, align 16, !alias.scope !5229, !noalias !5234
  %_140.i.i.i292.i127316106 = load <4 x float>, ptr %117, align 16, !alias.scope !5229, !noalias !5234
  %_143.i.i.i295.i127616107 = load <4 x float>, ptr %118, align 16, !alias.scope !5229, !noalias !5234
  %_146.i.i.i298.i127916108 = load <4 x float>, ptr %119, align 16, !alias.scope !5229, !noalias !5234
  %_151.i.i.i303.i128416109 = load <4 x float>, ptr %row19.i.i.i301.i1282, align 16, !alias.scope !5229, !noalias !5234
  %_154.i.i.i306.i128716110 = load <4 x float>, ptr %120, align 16, !alias.scope !5229, !noalias !5234
  %_157.i.i.i309.i129016111 = load <4 x float>, ptr %121, align 16, !alias.scope !5229, !noalias !5234
  %_160.i.i.i312.i129316112 = load <4 x float>, ptr %122, align 16, !alias.scope !5229, !noalias !5234
  %_165.i.i.i317.i129816113 = load <4 x float>, ptr %row21.i.i.i315.i1296, align 16, !alias.scope !5229, !noalias !5234
  %_168.i.i.i320.i130116114 = load <4 x float>, ptr %123, align 16, !alias.scope !5229, !noalias !5234
  %_171.i.i.i323.i130416115 = load <4 x float>, ptr %124, align 16, !alias.scope !5229, !noalias !5234
  %_174.i.i.i326.i130716116 = load <4 x float>, ptr %125, align 16, !alias.scope !5229, !noalias !5234
  br label %bb5.i145.i1126, !dbg !5221

bb5.i145.i1126:                                   ; preds = %bb5.i145.i1126.lr.ph, %bb5.i145.i1126
  %iter.i138.i1058.sroa.16.018389 = phi i32 [ 0, %bb5.i145.i1126.lr.ph ], [ %297, %bb5.i145.i1126 ]
  %history.i142.i1062.sroa.35.018388 = phi <4 x i32> [ %history.i142.i1062.sroa.35.0.copyload, %bb5.i145.i1126.lr.ph ], [ %history.i142.i1062.sroa.32.018387, %bb5.i145.i1126 ]
  %history.i142.i1062.sroa.32.018387 = phi <4 x i32> [ %history.i142.i1062.sroa.32.0.copyload, %bb5.i145.i1126.lr.ph ], [ %history.i142.i1062.sroa.29.018386, %bb5.i145.i1126 ]
  %history.i142.i1062.sroa.29.018386 = phi <4 x i32> [ %history.i142.i1062.sroa.29.0.copyload, %bb5.i145.i1126.lr.ph ], [ %history.i142.i1062.sroa.26.018385, %bb5.i145.i1126 ]
  %history.i142.i1062.sroa.26.018385 = phi <4 x i32> [ %history.i142.i1062.sroa.26.0.copyload, %bb5.i145.i1126.lr.ph ], [ %history.i142.i1062.sroa.22.018384, %bb5.i145.i1126 ]
  %history.i142.i1062.sroa.22.018384 = phi <4 x i32> [ %history.i142.i1062.sroa.22.0.copyload, %bb5.i145.i1126.lr.ph ], [ %history.i142.i1062.sroa.19.018383, %bb5.i145.i1126 ]
  %history.i142.i1062.sroa.19.018383 = phi <4 x i32> [ %history.i142.i1062.sroa.19.0.copyload, %bb5.i145.i1126.lr.ph ], [ %history.i142.i1062.sroa.16.018382, %bb5.i145.i1126 ]
  %history.i142.i1062.sroa.16.018382 = phi <4 x i32> [ %history.i142.i1062.sroa.16.0.copyload, %bb5.i145.i1126.lr.ph ], [ %history.i142.i1062.sroa.13.018381, %bb5.i145.i1126 ]
  %history.i142.i1062.sroa.13.018381 = phi <4 x i32> [ %history.i142.i1062.sroa.13.0.copyload, %bb5.i145.i1126.lr.ph ], [ %history.i142.i1062.sroa.10.018380, %bb5.i145.i1126 ]
  %history.i142.i1062.sroa.10.018380 = phi <4 x i32> [ %history.i142.i1062.sroa.10.0.copyload, %bb5.i145.i1126.lr.ph ], [ %history.i142.i1062.sroa.7.018379, %bb5.i145.i1126 ]
  %history.i142.i1062.sroa.7.018379 = phi <4 x i32> [ %history.i142.i1062.sroa.7.0.copyload, %bb5.i145.i1126.lr.ph ], [ %history.i142.i1062.sroa.0.018378, %bb5.i145.i1126 ]
  %history.i142.i1062.sroa.0.018378 = phi <4 x i32> [ %history.i142.i1062.sroa.0.0.copyload, %bb5.i145.i1126.lr.ph ], [ %lanes.i5735.sroa.0.0.copyload, %bb5.i145.i1126 ]
  %start1.i.i = shl i32 %iter.i138.i1058.sroa.16.018389, 2, !dbg !5243
  %data.i.i6977 = getelementptr inbounds nuw float, ptr %_128.i1123, i32 %start1.i.i, !dbg !5248
  %lanes.i5735.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i6977, align 4, !dbg !5252, !alias.scope !5258, !noalias !5262
  %176 = bitcast <4 x i32> %history.i142.i1062.sroa.19.018383 to <4 x float>, !dbg !5266
  %177 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %176), !dbg !5284
  %178 = bitcast <4 x i32> %lanes.i5735.sroa.0.0.copyload to <4 x float>, !dbg !5285
  %179 = fmul <4 x float> %_11.i.i.i163.i114416069, %178, !dbg !5302
  %180 = fadd <4 x float> %179, zeroinitializer, !dbg !5303
  %181 = fmul <4 x float> %_14.i.i.i166.i114716070, %178, !dbg !5311
  %182 = fadd <4 x float> %181, zeroinitializer, !dbg !5315
  %183 = fmul <4 x float> %_17.i.i.i169.i115016071, %178, !dbg !5319
  %184 = fadd <4 x float> %183, zeroinitializer, !dbg !5323
  %185 = fmul <4 x float> %_20.i.i.i172.i115316072, %178, !dbg !5327
  %186 = fadd <4 x float> %185, zeroinitializer, !dbg !5331
  %187 = bitcast <4 x i32> %history.i142.i1062.sroa.0.018378 to <4 x float>, !dbg !5335
  %188 = fmul <4 x float> %_25.i.i.i177.i115816073, %187, !dbg !5341
  %189 = fadd <4 x float> %180, %188, !dbg !5342
  %190 = fmul <4 x float> %_28.i.i.i180.i116116074, %187, !dbg !5346
  %191 = fadd <4 x float> %182, %190, !dbg !5350
  %192 = fmul <4 x float> %_31.i.i.i183.i116416075, %187, !dbg !5354
  %193 = fadd <4 x float> %184, %192, !dbg !5358
  %194 = fmul <4 x float> %_34.i.i.i186.i116716076, %187, !dbg !5362
  %195 = fadd <4 x float> %186, %194, !dbg !5366
  %196 = bitcast <4 x i32> %history.i142.i1062.sroa.7.018379 to <4 x float>, !dbg !5370
  %197 = fmul <4 x float> %_39.i.i.i191.i117216077, %196, !dbg !5376
  %198 = fadd <4 x float> %189, %197, !dbg !5377
  %199 = fmul <4 x float> %_42.i.i.i194.i117516078, %196, !dbg !5381
  %200 = fadd <4 x float> %191, %199, !dbg !5385
  %201 = fmul <4 x float> %_45.i.i.i197.i117816079, %196, !dbg !5389
  %202 = fadd <4 x float> %193, %201, !dbg !5393
  %203 = fmul <4 x float> %_48.i.i.i200.i118116080, %196, !dbg !5397
  %204 = fadd <4 x float> %195, %203, !dbg !5401
  %205 = bitcast <4 x i32> %history.i142.i1062.sroa.10.018380 to <4 x float>, !dbg !5405
  %206 = fmul <4 x float> %_53.i.i.i205.i118616081, %205, !dbg !5411
  %207 = fadd <4 x float> %198, %206, !dbg !5412
  %208 = fmul <4 x float> %_56.i.i.i208.i118916082, %205, !dbg !5416
  %209 = fadd <4 x float> %200, %208, !dbg !5420
  %210 = fmul <4 x float> %_59.i.i.i211.i119216083, %205, !dbg !5424
  %211 = fadd <4 x float> %202, %210, !dbg !5428
  %212 = fmul <4 x float> %_62.i.i.i214.i119516084, %205, !dbg !5432
  %213 = fadd <4 x float> %204, %212, !dbg !5436
  %214 = bitcast <4 x i32> %history.i142.i1062.sroa.13.018381 to <4 x float>, !dbg !5440
  %215 = fmul <4 x float> %_67.i.i.i219.i120016085, %214, !dbg !5446
  %216 = fadd <4 x float> %207, %215, !dbg !5447
  %217 = fmul <4 x float> %_70.i.i.i222.i120316086, %214, !dbg !5451
  %218 = fadd <4 x float> %209, %217, !dbg !5455
  %219 = fmul <4 x float> %_73.i.i.i225.i120616087, %214, !dbg !5459
  %220 = fadd <4 x float> %211, %219, !dbg !5463
  %221 = fmul <4 x float> %_76.i.i.i228.i120916088, %214, !dbg !5467
  %222 = fadd <4 x float> %213, %221, !dbg !5471
  %223 = bitcast <4 x i32> %history.i142.i1062.sroa.16.018382 to <4 x float>, !dbg !5475
  %224 = fmul <4 x float> %_81.i.i.i233.i121416089, %223, !dbg !5481
  %225 = fadd <4 x float> %216, %224, !dbg !5482
  %226 = fmul <4 x float> %_84.i.i.i236.i121716090, %223, !dbg !5486
  %227 = fadd <4 x float> %218, %226, !dbg !5490
  %228 = fmul <4 x float> %_87.i.i.i239.i122016091, %223, !dbg !5494
  %229 = fadd <4 x float> %220, %228, !dbg !5498
  %230 = fmul <4 x float> %_90.i.i.i242.i122316092, %223, !dbg !5502
  %231 = fadd <4 x float> %222, %230, !dbg !5506
  %232 = fmul <4 x float> %_95.i.i.i247.i122816093, %176, !dbg !5510
  %233 = fadd <4 x float> %225, %232, !dbg !5516
  %234 = fmul <4 x float> %_98.i.i.i250.i123116094, %176, !dbg !5520
  %235 = fadd <4 x float> %227, %234, !dbg !5524
  %236 = fmul <4 x float> %_101.i.i.i253.i123416095, %176, !dbg !5528
  %237 = fadd <4 x float> %229, %236, !dbg !5532
  %238 = fmul <4 x float> %_104.i.i.i256.i123716096, %176, !dbg !5536
  %239 = fadd <4 x float> %231, %238, !dbg !5540
  %240 = bitcast <4 x i32> %history.i142.i1062.sroa.22.018384 to <4 x float>, !dbg !5544
  %241 = fmul <4 x float> %_109.i.i.i261.i124216097, %240, !dbg !5550
  %242 = fadd <4 x float> %233, %241, !dbg !5551
  %243 = fmul <4 x float> %_112.i.i.i264.i124516098, %240, !dbg !5555
  %244 = fadd <4 x float> %235, %243, !dbg !5559
  %245 = fmul <4 x float> %_115.i.i.i267.i124816099, %240, !dbg !5563
  %246 = fadd <4 x float> %237, %245, !dbg !5567
  %247 = fmul <4 x float> %_118.i.i.i270.i125116100, %240, !dbg !5571
  %248 = fadd <4 x float> %239, %247, !dbg !5575
  %249 = bitcast <4 x i32> %history.i142.i1062.sroa.26.018385 to <4 x float>, !dbg !5579
  %250 = fmul <4 x float> %_123.i.i.i275.i125616101, %249, !dbg !5585
  %251 = fadd <4 x float> %242, %250, !dbg !5586
  %252 = fmul <4 x float> %_126.i.i.i278.i125916102, %249, !dbg !5590
  %253 = fadd <4 x float> %244, %252, !dbg !5594
  %254 = fmul <4 x float> %_129.i.i.i281.i126216103, %249, !dbg !5598
  %255 = fadd <4 x float> %246, %254, !dbg !5602
  %256 = fmul <4 x float> %_132.i.i.i284.i126516104, %249, !dbg !5606
  %257 = fadd <4 x float> %248, %256, !dbg !5610
  %258 = bitcast <4 x i32> %history.i142.i1062.sroa.29.018386 to <4 x float>, !dbg !5614
  %259 = fmul <4 x float> %_137.i.i.i289.i127016105, %258, !dbg !5620
  %260 = fadd <4 x float> %251, %259, !dbg !5621
  %261 = fmul <4 x float> %_140.i.i.i292.i127316106, %258, !dbg !5625
  %262 = fadd <4 x float> %253, %261, !dbg !5629
  %263 = fmul <4 x float> %_143.i.i.i295.i127616107, %258, !dbg !5633
  %264 = fadd <4 x float> %255, %263, !dbg !5637
  %265 = fmul <4 x float> %_146.i.i.i298.i127916108, %258, !dbg !5641
  %266 = fadd <4 x float> %257, %265, !dbg !5645
  %267 = bitcast <4 x i32> %history.i142.i1062.sroa.32.018387 to <4 x float>, !dbg !5649
  %268 = fmul <4 x float> %_151.i.i.i303.i128416109, %267, !dbg !5655
  %269 = fadd <4 x float> %260, %268, !dbg !5656
  %270 = fmul <4 x float> %_154.i.i.i306.i128716110, %267, !dbg !5660
  %271 = fadd <4 x float> %262, %270, !dbg !5664
  %272 = fmul <4 x float> %_157.i.i.i309.i129016111, %267, !dbg !5668
  %273 = fadd <4 x float> %264, %272, !dbg !5672
  %274 = fmul <4 x float> %_160.i.i.i312.i129316112, %267, !dbg !5676
  %275 = fadd <4 x float> %266, %274, !dbg !5680
  %276 = bitcast <4 x i32> %history.i142.i1062.sroa.35.018388 to <4 x float>, !dbg !5684
  %277 = fmul <4 x float> %_165.i.i.i317.i129816113, %276, !dbg !5690
  %278 = fadd <4 x float> %269, %277, !dbg !5691
  %279 = fmul <4 x float> %_168.i.i.i320.i130116114, %276, !dbg !5695
  %280 = fadd <4 x float> %271, %279, !dbg !5699
  %281 = fmul <4 x float> %_171.i.i.i323.i130416115, %276, !dbg !5703
  %282 = fadd <4 x float> %273, %281, !dbg !5707
  %283 = fmul <4 x float> %_174.i.i.i326.i130716116, %276, !dbg !5711
  %284 = fadd <4 x float> %275, %283, !dbg !5715
  %285 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %278), !dbg !5719
  %286 = fcmp olt <4 x float> %285, %177, !dbg !5725
  %287 = select <4 x i1> %286, <4 x float> %177, <4 x float> %285, !dbg !5732
  %288 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %280), !dbg !5719
  %289 = fcmp olt <4 x float> %288, %287, !dbg !5725
  %290 = select <4 x i1> %289, <4 x float> %287, <4 x float> %288, !dbg !5732
  %291 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %282), !dbg !5719
  %292 = fcmp olt <4 x float> %291, %290, !dbg !5725
  %293 = select <4 x i1> %292, <4 x float> %290, <4 x float> %291, !dbg !5732
  %294 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %284), !dbg !5719
  %295 = fcmp olt <4 x float> %294, %293, !dbg !5725
  %296 = select <4 x i1> %295, <4 x float> %293, <4 x float> %294, !dbg !5732
  %297 = add nuw nsw i32 %iter.i138.i1058.sroa.16.018389, 1, !dbg !5733
  %data.i4.i = getelementptr inbounds nuw float, ptr %peaks_left.i1087, i32 %start1.i.i, !dbg !5734
  store <4 x float> %296, ptr %data.i4.i, align 4, !dbg !5741, !alias.scope !5746, !noalias !5750
  %exitcond.not = icmp eq i32 %297, %umax20918, !dbg !5221
  br i1 %exitcond.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit339.i1320, label %bb5.i145.i1126, !dbg !5221

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit339.i1320: ; preds = %bb5.i145.i1126, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit
  %history.i142.i1062.sroa.0.0.lcssa = phi <4 x i32> [ %history.i142.i1062.sroa.0.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %lanes.i5735.sroa.0.0.copyload, %bb5.i145.i1126 ], !dbg !5754
  %history.i142.i1062.sroa.7.0.lcssa = phi <4 x i32> [ %history.i142.i1062.sroa.7.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i142.i1062.sroa.0.018378, %bb5.i145.i1126 ], !dbg !5754
  %history.i142.i1062.sroa.10.0.lcssa = phi <4 x i32> [ %history.i142.i1062.sroa.10.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i142.i1062.sroa.7.018379, %bb5.i145.i1126 ], !dbg !5754
  %history.i142.i1062.sroa.13.0.lcssa = phi <4 x i32> [ %history.i142.i1062.sroa.13.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i142.i1062.sroa.10.018380, %bb5.i145.i1126 ], !dbg !5754
  %history.i142.i1062.sroa.16.0.lcssa = phi <4 x i32> [ %history.i142.i1062.sroa.16.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i142.i1062.sroa.13.018381, %bb5.i145.i1126 ], !dbg !5754
  %history.i142.i1062.sroa.19.0.lcssa = phi <4 x i32> [ %history.i142.i1062.sroa.19.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i142.i1062.sroa.16.018382, %bb5.i145.i1126 ], !dbg !5754
  %history.i142.i1062.sroa.22.0.lcssa = phi <4 x i32> [ %history.i142.i1062.sroa.22.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i142.i1062.sroa.19.018383, %bb5.i145.i1126 ], !dbg !5754
  %history.i142.i1062.sroa.26.0.lcssa = phi <4 x i32> [ %history.i142.i1062.sroa.26.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i142.i1062.sroa.22.018384, %bb5.i145.i1126 ], !dbg !5754
  %history.i142.i1062.sroa.29.0.lcssa = phi <4 x i32> [ %history.i142.i1062.sroa.29.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i142.i1062.sroa.26.018385, %bb5.i145.i1126 ], !dbg !5754
  %history.i142.i1062.sroa.32.0.lcssa = phi <4 x i32> [ %history.i142.i1062.sroa.32.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i142.i1062.sroa.29.018386, %bb5.i145.i1126 ], !dbg !5754
  %history.i142.i1062.sroa.35.0.lcssa = phi <4 x i32> [ %history.i142.i1062.sroa.35.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i142.i1062.sroa.32.018387, %bb5.i145.i1126 ], !dbg !5754
  %history.i142.i1062.sroa.38.0.lcssa = phi <4 x i32> [ %history.i142.i1062.sroa.38.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i142.i1062.sroa.35.018388, %bb5.i145.i1126 ], !dbg !5754
  store <4 x i32> %history.i142.i1062.sroa.0.0.lcssa, ptr %hot_left.i1091, align 16, !dbg !5755
  store <4 x i32> %history.i142.i1062.sroa.7.0.lcssa, ptr %history.i142.i1062.sroa.7.0.hot_left.i1091.sroa_idx, align 16, !dbg !5755
  store <4 x i32> %history.i142.i1062.sroa.10.0.lcssa, ptr %history.i142.i1062.sroa.10.0.hot_left.i1091.sroa_idx, align 16, !dbg !5755
  store <4 x i32> %history.i142.i1062.sroa.13.0.lcssa, ptr %history.i142.i1062.sroa.13.0.hot_left.i1091.sroa_idx, align 16, !dbg !5755
  store <4 x i32> %history.i142.i1062.sroa.16.0.lcssa, ptr %history.i142.i1062.sroa.16.0.hot_left.i1091.sroa_idx, align 16, !dbg !5755
  store <4 x i32> %history.i142.i1062.sroa.19.0.lcssa, ptr %history.i142.i1062.sroa.19.0.hot_left.i1091.sroa_idx, align 16, !dbg !5755
  store <4 x i32> %history.i142.i1062.sroa.22.0.lcssa, ptr %history.i142.i1062.sroa.22.0.hot_left.i1091.sroa_idx, align 16, !dbg !5755
  store <4 x i32> %history.i142.i1062.sroa.26.0.lcssa, ptr %history.i142.i1062.sroa.26.0.hot_left.i1091.sroa_idx, align 16, !dbg !5755
  store <4 x i32> %history.i142.i1062.sroa.29.0.lcssa, ptr %history.i142.i1062.sroa.29.0.hot_left.i1091.sroa_idx, align 16, !dbg !5755
  store <4 x i32> %history.i142.i1062.sroa.32.0.lcssa, ptr %history.i142.i1062.sroa.32.0.hot_left.i1091.sroa_idx, align 16, !dbg !5755
  store <4 x i32> %history.i142.i1062.sroa.35.0.lcssa, ptr %history.i142.i1062.sroa.35.0.hot_left.i1091.sroa_idx, align 16, !dbg !5755
  store <4 x i32> %history.i142.i1062.sroa.38.0.lcssa, ptr %history.i142.i1062.sroa.38.0.hot_left.i1091.sroa_idx, align 16, !dbg !5755
  %_136.not.i1321 = icmp ugt i32 %_51.i1118, %right_io.1, !dbg !5756
  br i1 %_136.not.i1321, label %bb49.i1759, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7005, !dbg !5756, !prof !787

bb49.i1759:                                       ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit339.i1320
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %active_base.i1116, i32 noundef %_51.i1118, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_685aef6cd6b0f866813eafbee04e700c) #32, !dbg !5760, !noalias !5208
  unreachable, !dbg !5760

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7005: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit339.i1320
  %_143.i1323 = getelementptr inbounds nuw float, ptr %right_io.0, i32 %active_base.i1116, !dbg !5761
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5765), !dbg !5768
  %history.i.i1081.sroa.0.0.copyload = load <4 x i32>, ptr %hot_right.i1090, align 16, !dbg !5769
  %history.i.i1081.sroa.7.0.copyload = load <4 x i32>, ptr %history.i.i1081.sroa.7.0.hot_right.i1090.sroa_idx, align 16, !dbg !5769
  %history.i.i1081.sroa.10.0.copyload = load <4 x i32>, ptr %history.i.i1081.sroa.10.0.hot_right.i1090.sroa_idx, align 16, !dbg !5769
  %history.i.i1081.sroa.13.0.copyload = load <4 x i32>, ptr %history.i.i1081.sroa.13.0.hot_right.i1090.sroa_idx, align 16, !dbg !5769
  %history.i.i1081.sroa.16.0.copyload = load <4 x i32>, ptr %history.i.i1081.sroa.16.0.hot_right.i1090.sroa_idx, align 16, !dbg !5769
  %history.i.i1081.sroa.19.0.copyload = load <4 x i32>, ptr %history.i.i1081.sroa.19.0.hot_right.i1090.sroa_idx, align 16, !dbg !5769
  %history.i.i1081.sroa.22.0.copyload = load <4 x i32>, ptr %history.i.i1081.sroa.22.0.hot_right.i1090.sroa_idx, align 16, !dbg !5769
  %history.i.i1081.sroa.26.0.copyload = load <4 x i32>, ptr %history.i.i1081.sroa.26.0.hot_right.i1090.sroa_idx, align 16, !dbg !5769
  %history.i.i1081.sroa.29.0.copyload = load <4 x i32>, ptr %history.i.i1081.sroa.29.0.hot_right.i1090.sroa_idx, align 16, !dbg !5769
  %history.i.i1081.sroa.32.0.copyload = load <4 x i32>, ptr %history.i.i1081.sroa.32.0.hot_right.i1090.sroa_idx, align 16, !dbg !5769
  %history.i.i1081.sroa.35.0.copyload = load <4 x i32>, ptr %history.i.i1081.sroa.35.0.hot_right.i1090.sroa_idx, align 16, !dbg !5769
  %history.i.i1081.sroa.38.0.copyload = load <4 x i32>, ptr %history.i.i1081.sroa.38.0.hot_right.i1090.sroa_idx, align 16, !dbg !5769
  br i1 %_2.i697418377.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i1520, label %bb5.i.i1326.lr.ph, !dbg !5771

bb5.i.i1326.lr.ph:                                ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7005
  %_11.i.i.i.i134416016 = load <4 x float>, ptr %_31, align 16, !alias.scope !5774, !noalias !5779
  %_14.i.i.i.i134716017 = load <4 x float>, ptr %90, align 16, !alias.scope !5774, !noalias !5779
  %_17.i.i.i.i135016018 = load <4 x float>, ptr %91, align 16, !alias.scope !5774, !noalias !5779
  %_20.i.i.i.i135316019 = load <4 x float>, ptr %92, align 16, !alias.scope !5774, !noalias !5779
  %_25.i.i.i.i135816020 = load <4 x float>, ptr %row1.i.i.i175.i1156, align 16, !alias.scope !5774, !noalias !5779
  %_28.i.i.i.i136116021 = load <4 x float>, ptr %93, align 16, !alias.scope !5774, !noalias !5779
  %_31.i.i.i.i136416022 = load <4 x float>, ptr %94, align 16, !alias.scope !5774, !noalias !5779
  %_34.i.i.i.i136716023 = load <4 x float>, ptr %95, align 16, !alias.scope !5774, !noalias !5779
  %_39.i.i.i.i137216024 = load <4 x float>, ptr %row3.i.i.i189.i1170, align 16, !alias.scope !5774, !noalias !5779
  %_42.i.i.i.i137516025 = load <4 x float>, ptr %96, align 16, !alias.scope !5774, !noalias !5779
  %_45.i.i.i.i137816026 = load <4 x float>, ptr %97, align 16, !alias.scope !5774, !noalias !5779
  %_48.i.i.i.i138116027 = load <4 x float>, ptr %98, align 16, !alias.scope !5774, !noalias !5779
  %_53.i.i.i.i138616028 = load <4 x float>, ptr %row5.i.i.i203.i1184, align 16, !alias.scope !5774, !noalias !5779
  %_56.i.i.i.i138916029 = load <4 x float>, ptr %99, align 16, !alias.scope !5774, !noalias !5779
  %_59.i.i.i.i139216030 = load <4 x float>, ptr %100, align 16, !alias.scope !5774, !noalias !5779
  %_62.i.i.i.i139516031 = load <4 x float>, ptr %101, align 16, !alias.scope !5774, !noalias !5779
  %_67.i.i.i.i140016032 = load <4 x float>, ptr %row7.i.i.i217.i1198, align 16, !alias.scope !5774, !noalias !5779
  %_70.i.i.i.i140316033 = load <4 x float>, ptr %102, align 16, !alias.scope !5774, !noalias !5779
  %_73.i.i.i.i140616034 = load <4 x float>, ptr %103, align 16, !alias.scope !5774, !noalias !5779
  %_76.i.i.i.i140916035 = load <4 x float>, ptr %104, align 16, !alias.scope !5774, !noalias !5779
  %_81.i.i.i.i141416036 = load <4 x float>, ptr %row9.i.i.i231.i1212, align 16, !alias.scope !5774, !noalias !5779
  %_84.i.i.i.i141716037 = load <4 x float>, ptr %105, align 16, !alias.scope !5774, !noalias !5779
  %_87.i.i.i.i142016038 = load <4 x float>, ptr %106, align 16, !alias.scope !5774, !noalias !5779
  %_90.i.i.i.i142316039 = load <4 x float>, ptr %107, align 16, !alias.scope !5774, !noalias !5779
  %_95.i.i.i.i142816040 = load <4 x float>, ptr %row11.i.i.i245.i1226, align 16, !alias.scope !5774, !noalias !5779
  %_98.i.i.i.i143116041 = load <4 x float>, ptr %108, align 16, !alias.scope !5774, !noalias !5779
  %_101.i.i.i.i143416042 = load <4 x float>, ptr %109, align 16, !alias.scope !5774, !noalias !5779
  %_104.i.i.i.i143716043 = load <4 x float>, ptr %110, align 16, !alias.scope !5774, !noalias !5779
  %_109.i.i.i.i144216044 = load <4 x float>, ptr %row13.i.i.i259.i1240, align 16, !alias.scope !5774, !noalias !5779
  %_112.i.i.i.i144516045 = load <4 x float>, ptr %111, align 16, !alias.scope !5774, !noalias !5779
  %_115.i.i.i.i144816046 = load <4 x float>, ptr %112, align 16, !alias.scope !5774, !noalias !5779
  %_118.i.i.i.i145116047 = load <4 x float>, ptr %113, align 16, !alias.scope !5774, !noalias !5779
  %_123.i.i.i.i145616048 = load <4 x float>, ptr %row15.i.i.i273.i1254, align 16, !alias.scope !5774, !noalias !5779
  %_126.i.i.i.i145916049 = load <4 x float>, ptr %114, align 16, !alias.scope !5774, !noalias !5779
  %_129.i.i.i.i146216050 = load <4 x float>, ptr %115, align 16, !alias.scope !5774, !noalias !5779
  %_132.i.i.i.i146516051 = load <4 x float>, ptr %116, align 16, !alias.scope !5774, !noalias !5779
  %_137.i.i.i.i147016052 = load <4 x float>, ptr %row17.i.i.i287.i1268, align 16, !alias.scope !5774, !noalias !5779
  %_140.i.i.i.i147316053 = load <4 x float>, ptr %117, align 16, !alias.scope !5774, !noalias !5779
  %_143.i.i.i.i147616054 = load <4 x float>, ptr %118, align 16, !alias.scope !5774, !noalias !5779
  %_146.i.i.i.i147916055 = load <4 x float>, ptr %119, align 16, !alias.scope !5774, !noalias !5779
  %_151.i.i.i.i148416056 = load <4 x float>, ptr %row19.i.i.i301.i1282, align 16, !alias.scope !5774, !noalias !5779
  %_154.i.i.i.i148716057 = load <4 x float>, ptr %120, align 16, !alias.scope !5774, !noalias !5779
  %_157.i.i.i.i149016058 = load <4 x float>, ptr %121, align 16, !alias.scope !5774, !noalias !5779
  %_160.i.i.i.i149316059 = load <4 x float>, ptr %122, align 16, !alias.scope !5774, !noalias !5779
  %_165.i.i.i.i149816060 = load <4 x float>, ptr %row21.i.i.i315.i1296, align 16, !alias.scope !5774, !noalias !5779
  %_168.i.i.i.i150116061 = load <4 x float>, ptr %123, align 16, !alias.scope !5774, !noalias !5779
  %_171.i.i.i.i150416062 = load <4 x float>, ptr %124, align 16, !alias.scope !5774, !noalias !5779
  %_174.i.i.i.i150716063 = load <4 x float>, ptr %125, align 16, !alias.scope !5774, !noalias !5779
  br label %bb5.i.i1326, !dbg !5771

bb5.i.i1326:                                      ; preds = %bb5.i.i1326.lr.ph, %bb5.i.i1326
  %iter.i.i1077.sroa.16.018415 = phi i32 [ 0, %bb5.i.i1326.lr.ph ], [ %419, %bb5.i.i1326 ]
  %history.i.i1081.sroa.35.018414 = phi <4 x i32> [ %history.i.i1081.sroa.35.0.copyload, %bb5.i.i1326.lr.ph ], [ %history.i.i1081.sroa.32.018413, %bb5.i.i1326 ]
  %history.i.i1081.sroa.32.018413 = phi <4 x i32> [ %history.i.i1081.sroa.32.0.copyload, %bb5.i.i1326.lr.ph ], [ %history.i.i1081.sroa.29.018412, %bb5.i.i1326 ]
  %history.i.i1081.sroa.29.018412 = phi <4 x i32> [ %history.i.i1081.sroa.29.0.copyload, %bb5.i.i1326.lr.ph ], [ %history.i.i1081.sroa.26.018411, %bb5.i.i1326 ]
  %history.i.i1081.sroa.26.018411 = phi <4 x i32> [ %history.i.i1081.sroa.26.0.copyload, %bb5.i.i1326.lr.ph ], [ %history.i.i1081.sroa.22.018410, %bb5.i.i1326 ]
  %history.i.i1081.sroa.22.018410 = phi <4 x i32> [ %history.i.i1081.sroa.22.0.copyload, %bb5.i.i1326.lr.ph ], [ %history.i.i1081.sroa.19.018409, %bb5.i.i1326 ]
  %history.i.i1081.sroa.19.018409 = phi <4 x i32> [ %history.i.i1081.sroa.19.0.copyload, %bb5.i.i1326.lr.ph ], [ %history.i.i1081.sroa.16.018408, %bb5.i.i1326 ]
  %history.i.i1081.sroa.16.018408 = phi <4 x i32> [ %history.i.i1081.sroa.16.0.copyload, %bb5.i.i1326.lr.ph ], [ %history.i.i1081.sroa.13.018407, %bb5.i.i1326 ]
  %history.i.i1081.sroa.13.018407 = phi <4 x i32> [ %history.i.i1081.sroa.13.0.copyload, %bb5.i.i1326.lr.ph ], [ %history.i.i1081.sroa.10.018406, %bb5.i.i1326 ]
  %history.i.i1081.sroa.10.018406 = phi <4 x i32> [ %history.i.i1081.sroa.10.0.copyload, %bb5.i.i1326.lr.ph ], [ %history.i.i1081.sroa.7.018405, %bb5.i.i1326 ]
  %history.i.i1081.sroa.7.018405 = phi <4 x i32> [ %history.i.i1081.sroa.7.0.copyload, %bb5.i.i1326.lr.ph ], [ %history.i.i1081.sroa.0.018404, %bb5.i.i1326 ]
  %history.i.i1081.sroa.0.018404 = phi <4 x i32> [ %history.i.i1081.sroa.0.0.copyload, %bb5.i.i1326.lr.ph ], [ %lanes.i5726.sroa.0.0.copyload, %bb5.i.i1326 ]
  %start1.i.i7014 = shl i32 %iter.i.i1077.sroa.16.018415, 2, !dbg !5788
  %data.i.i7015 = getelementptr inbounds nuw float, ptr %_143.i1323, i32 %start1.i.i7014, !dbg !5790
  %lanes.i5726.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i7015, align 4, !dbg !5792, !alias.scope !5797, !noalias !5801
  %298 = bitcast <4 x i32> %history.i.i1081.sroa.19.018409 to <4 x float>, !dbg !5805
  %299 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %298), !dbg !5810
  %300 = bitcast <4 x i32> %lanes.i5726.sroa.0.0.copyload to <4 x float>, !dbg !5811
  %301 = fmul <4 x float> %_11.i.i.i.i134416016, %300, !dbg !5816
  %302 = fadd <4 x float> %301, zeroinitializer, !dbg !5817
  %303 = fmul <4 x float> %_14.i.i.i.i134716017, %300, !dbg !5821
  %304 = fadd <4 x float> %303, zeroinitializer, !dbg !5825
  %305 = fmul <4 x float> %_17.i.i.i.i135016018, %300, !dbg !5829
  %306 = fadd <4 x float> %305, zeroinitializer, !dbg !5833
  %307 = fmul <4 x float> %_20.i.i.i.i135316019, %300, !dbg !5837
  %308 = fadd <4 x float> %307, zeroinitializer, !dbg !5841
  %309 = bitcast <4 x i32> %history.i.i1081.sroa.0.018404 to <4 x float>, !dbg !5845
  %310 = fmul <4 x float> %_25.i.i.i.i135816020, %309, !dbg !5849
  %311 = fadd <4 x float> %302, %310, !dbg !5850
  %312 = fmul <4 x float> %_28.i.i.i.i136116021, %309, !dbg !5854
  %313 = fadd <4 x float> %304, %312, !dbg !5858
  %314 = fmul <4 x float> %_31.i.i.i.i136416022, %309, !dbg !5862
  %315 = fadd <4 x float> %306, %314, !dbg !5866
  %316 = fmul <4 x float> %_34.i.i.i.i136716023, %309, !dbg !5870
  %317 = fadd <4 x float> %308, %316, !dbg !5874
  %318 = bitcast <4 x i32> %history.i.i1081.sroa.7.018405 to <4 x float>, !dbg !5878
  %319 = fmul <4 x float> %_39.i.i.i.i137216024, %318, !dbg !5882
  %320 = fadd <4 x float> %311, %319, !dbg !5883
  %321 = fmul <4 x float> %_42.i.i.i.i137516025, %318, !dbg !5887
  %322 = fadd <4 x float> %313, %321, !dbg !5891
  %323 = fmul <4 x float> %_45.i.i.i.i137816026, %318, !dbg !5895
  %324 = fadd <4 x float> %315, %323, !dbg !5899
  %325 = fmul <4 x float> %_48.i.i.i.i138116027, %318, !dbg !5903
  %326 = fadd <4 x float> %317, %325, !dbg !5907
  %327 = bitcast <4 x i32> %history.i.i1081.sroa.10.018406 to <4 x float>, !dbg !5911
  %328 = fmul <4 x float> %_53.i.i.i.i138616028, %327, !dbg !5915
  %329 = fadd <4 x float> %320, %328, !dbg !5916
  %330 = fmul <4 x float> %_56.i.i.i.i138916029, %327, !dbg !5920
  %331 = fadd <4 x float> %322, %330, !dbg !5924
  %332 = fmul <4 x float> %_59.i.i.i.i139216030, %327, !dbg !5928
  %333 = fadd <4 x float> %324, %332, !dbg !5932
  %334 = fmul <4 x float> %_62.i.i.i.i139516031, %327, !dbg !5936
  %335 = fadd <4 x float> %326, %334, !dbg !5940
  %336 = bitcast <4 x i32> %history.i.i1081.sroa.13.018407 to <4 x float>, !dbg !5944
  %337 = fmul <4 x float> %_67.i.i.i.i140016032, %336, !dbg !5948
  %338 = fadd <4 x float> %329, %337, !dbg !5949
  %339 = fmul <4 x float> %_70.i.i.i.i140316033, %336, !dbg !5953
  %340 = fadd <4 x float> %331, %339, !dbg !5957
  %341 = fmul <4 x float> %_73.i.i.i.i140616034, %336, !dbg !5961
  %342 = fadd <4 x float> %333, %341, !dbg !5965
  %343 = fmul <4 x float> %_76.i.i.i.i140916035, %336, !dbg !5969
  %344 = fadd <4 x float> %335, %343, !dbg !5973
  %345 = bitcast <4 x i32> %history.i.i1081.sroa.16.018408 to <4 x float>, !dbg !5977
  %346 = fmul <4 x float> %_81.i.i.i.i141416036, %345, !dbg !5981
  %347 = fadd <4 x float> %338, %346, !dbg !5982
  %348 = fmul <4 x float> %_84.i.i.i.i141716037, %345, !dbg !5986
  %349 = fadd <4 x float> %340, %348, !dbg !5990
  %350 = fmul <4 x float> %_87.i.i.i.i142016038, %345, !dbg !5994
  %351 = fadd <4 x float> %342, %350, !dbg !5998
  %352 = fmul <4 x float> %_90.i.i.i.i142316039, %345, !dbg !6002
  %353 = fadd <4 x float> %344, %352, !dbg !6006
  %354 = fmul <4 x float> %_95.i.i.i.i142816040, %298, !dbg !6010
  %355 = fadd <4 x float> %347, %354, !dbg !6014
  %356 = fmul <4 x float> %_98.i.i.i.i143116041, %298, !dbg !6018
  %357 = fadd <4 x float> %349, %356, !dbg !6022
  %358 = fmul <4 x float> %_101.i.i.i.i143416042, %298, !dbg !6026
  %359 = fadd <4 x float> %351, %358, !dbg !6030
  %360 = fmul <4 x float> %_104.i.i.i.i143716043, %298, !dbg !6034
  %361 = fadd <4 x float> %353, %360, !dbg !6038
  %362 = bitcast <4 x i32> %history.i.i1081.sroa.22.018410 to <4 x float>, !dbg !6042
  %363 = fmul <4 x float> %_109.i.i.i.i144216044, %362, !dbg !6046
  %364 = fadd <4 x float> %355, %363, !dbg !6047
  %365 = fmul <4 x float> %_112.i.i.i.i144516045, %362, !dbg !6051
  %366 = fadd <4 x float> %357, %365, !dbg !6055
  %367 = fmul <4 x float> %_115.i.i.i.i144816046, %362, !dbg !6059
  %368 = fadd <4 x float> %359, %367, !dbg !6063
  %369 = fmul <4 x float> %_118.i.i.i.i145116047, %362, !dbg !6067
  %370 = fadd <4 x float> %361, %369, !dbg !6071
  %371 = bitcast <4 x i32> %history.i.i1081.sroa.26.018411 to <4 x float>, !dbg !6075
  %372 = fmul <4 x float> %_123.i.i.i.i145616048, %371, !dbg !6079
  %373 = fadd <4 x float> %364, %372, !dbg !6080
  %374 = fmul <4 x float> %_126.i.i.i.i145916049, %371, !dbg !6084
  %375 = fadd <4 x float> %366, %374, !dbg !6088
  %376 = fmul <4 x float> %_129.i.i.i.i146216050, %371, !dbg !6092
  %377 = fadd <4 x float> %368, %376, !dbg !6096
  %378 = fmul <4 x float> %_132.i.i.i.i146516051, %371, !dbg !6100
  %379 = fadd <4 x float> %370, %378, !dbg !6104
  %380 = bitcast <4 x i32> %history.i.i1081.sroa.29.018412 to <4 x float>, !dbg !6108
  %381 = fmul <4 x float> %_137.i.i.i.i147016052, %380, !dbg !6112
  %382 = fadd <4 x float> %373, %381, !dbg !6113
  %383 = fmul <4 x float> %_140.i.i.i.i147316053, %380, !dbg !6117
  %384 = fadd <4 x float> %375, %383, !dbg !6121
  %385 = fmul <4 x float> %_143.i.i.i.i147616054, %380, !dbg !6125
  %386 = fadd <4 x float> %377, %385, !dbg !6129
  %387 = fmul <4 x float> %_146.i.i.i.i147916055, %380, !dbg !6133
  %388 = fadd <4 x float> %379, %387, !dbg !6137
  %389 = bitcast <4 x i32> %history.i.i1081.sroa.32.018413 to <4 x float>, !dbg !6141
  %390 = fmul <4 x float> %_151.i.i.i.i148416056, %389, !dbg !6145
  %391 = fadd <4 x float> %382, %390, !dbg !6146
  %392 = fmul <4 x float> %_154.i.i.i.i148716057, %389, !dbg !6150
  %393 = fadd <4 x float> %384, %392, !dbg !6154
  %394 = fmul <4 x float> %_157.i.i.i.i149016058, %389, !dbg !6158
  %395 = fadd <4 x float> %386, %394, !dbg !6162
  %396 = fmul <4 x float> %_160.i.i.i.i149316059, %389, !dbg !6166
  %397 = fadd <4 x float> %388, %396, !dbg !6170
  %398 = bitcast <4 x i32> %history.i.i1081.sroa.35.018414 to <4 x float>, !dbg !6174
  %399 = fmul <4 x float> %_165.i.i.i.i149816060, %398, !dbg !6178
  %400 = fadd <4 x float> %391, %399, !dbg !6179
  %401 = fmul <4 x float> %_168.i.i.i.i150116061, %398, !dbg !6183
  %402 = fadd <4 x float> %393, %401, !dbg !6187
  %403 = fmul <4 x float> %_171.i.i.i.i150416062, %398, !dbg !6191
  %404 = fadd <4 x float> %395, %403, !dbg !6195
  %405 = fmul <4 x float> %_174.i.i.i.i150716063, %398, !dbg !6199
  %406 = fadd <4 x float> %397, %405, !dbg !6203
  %407 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %400), !dbg !6207
  %408 = fcmp olt <4 x float> %407, %299, !dbg !6211
  %409 = select <4 x i1> %408, <4 x float> %299, <4 x float> %407, !dbg !6215
  %410 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %402), !dbg !6207
  %411 = fcmp olt <4 x float> %410, %409, !dbg !6211
  %412 = select <4 x i1> %411, <4 x float> %409, <4 x float> %410, !dbg !6215
  %413 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %404), !dbg !6207
  %414 = fcmp olt <4 x float> %413, %412, !dbg !6211
  %415 = select <4 x i1> %414, <4 x float> %412, <4 x float> %413, !dbg !6215
  %416 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %406), !dbg !6207
  %417 = fcmp olt <4 x float> %416, %415, !dbg !6211
  %418 = select <4 x i1> %417, <4 x float> %415, <4 x float> %416, !dbg !6215
  %419 = add nuw nsw i32 %iter.i.i1077.sroa.16.018415, 1, !dbg !6216
  %data.i4.i7019 = getelementptr inbounds nuw float, ptr %peaks_right.i1086, i32 %start1.i.i7014, !dbg !6217
  store <4 x float> %418, ptr %data.i4.i7019, align 4, !dbg !6220, !alias.scope !6225, !noalias !6229
  %exitcond20894.not = icmp eq i32 %419, %umax20918, !dbg !5771
  br i1 %exitcond20894.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i1520, label %bb5.i.i1326, !dbg !5771

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i1520: ; preds = %bb5.i.i1326, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7005
  %history.i.i1081.sroa.0.0.lcssa = phi <4 x i32> [ %history.i.i1081.sroa.0.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7005 ], [ %lanes.i5726.sroa.0.0.copyload, %bb5.i.i1326 ], !dbg !6233
  %history.i.i1081.sroa.7.0.lcssa = phi <4 x i32> [ %history.i.i1081.sroa.7.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7005 ], [ %history.i.i1081.sroa.0.018404, %bb5.i.i1326 ], !dbg !6233
  %history.i.i1081.sroa.10.0.lcssa = phi <4 x i32> [ %history.i.i1081.sroa.10.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7005 ], [ %history.i.i1081.sroa.7.018405, %bb5.i.i1326 ], !dbg !6233
  %history.i.i1081.sroa.13.0.lcssa = phi <4 x i32> [ %history.i.i1081.sroa.13.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7005 ], [ %history.i.i1081.sroa.10.018406, %bb5.i.i1326 ], !dbg !6233
  %history.i.i1081.sroa.16.0.lcssa = phi <4 x i32> [ %history.i.i1081.sroa.16.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7005 ], [ %history.i.i1081.sroa.13.018407, %bb5.i.i1326 ], !dbg !6233
  %history.i.i1081.sroa.19.0.lcssa = phi <4 x i32> [ %history.i.i1081.sroa.19.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7005 ], [ %history.i.i1081.sroa.16.018408, %bb5.i.i1326 ], !dbg !6233
  %history.i.i1081.sroa.22.0.lcssa = phi <4 x i32> [ %history.i.i1081.sroa.22.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7005 ], [ %history.i.i1081.sroa.19.018409, %bb5.i.i1326 ], !dbg !6233
  %history.i.i1081.sroa.26.0.lcssa = phi <4 x i32> [ %history.i.i1081.sroa.26.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7005 ], [ %history.i.i1081.sroa.22.018410, %bb5.i.i1326 ], !dbg !6233
  %history.i.i1081.sroa.29.0.lcssa = phi <4 x i32> [ %history.i.i1081.sroa.29.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7005 ], [ %history.i.i1081.sroa.26.018411, %bb5.i.i1326 ], !dbg !6233
  %history.i.i1081.sroa.32.0.lcssa = phi <4 x i32> [ %history.i.i1081.sroa.32.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7005 ], [ %history.i.i1081.sroa.29.018412, %bb5.i.i1326 ], !dbg !6233
  %history.i.i1081.sroa.35.0.lcssa = phi <4 x i32> [ %history.i.i1081.sroa.35.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7005 ], [ %history.i.i1081.sroa.32.018413, %bb5.i.i1326 ], !dbg !6233
  %history.i.i1081.sroa.38.0.lcssa = phi <4 x i32> [ %history.i.i1081.sroa.38.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7005 ], [ %history.i.i1081.sroa.35.018414, %bb5.i.i1326 ], !dbg !6233
  store <4 x i32> %history.i.i1081.sroa.0.0.lcssa, ptr %hot_right.i1090, align 16, !dbg !6234
  store <4 x i32> %history.i.i1081.sroa.7.0.lcssa, ptr %history.i.i1081.sroa.7.0.hot_right.i1090.sroa_idx, align 16, !dbg !6234
  store <4 x i32> %history.i.i1081.sroa.10.0.lcssa, ptr %history.i.i1081.sroa.10.0.hot_right.i1090.sroa_idx, align 16, !dbg !6234
  store <4 x i32> %history.i.i1081.sroa.13.0.lcssa, ptr %history.i.i1081.sroa.13.0.hot_right.i1090.sroa_idx, align 16, !dbg !6234
  store <4 x i32> %history.i.i1081.sroa.16.0.lcssa, ptr %history.i.i1081.sroa.16.0.hot_right.i1090.sroa_idx, align 16, !dbg !6234
  store <4 x i32> %history.i.i1081.sroa.19.0.lcssa, ptr %history.i.i1081.sroa.19.0.hot_right.i1090.sroa_idx, align 16, !dbg !6234
  store <4 x i32> %history.i.i1081.sroa.22.0.lcssa, ptr %history.i.i1081.sroa.22.0.hot_right.i1090.sroa_idx, align 16, !dbg !6234
  store <4 x i32> %history.i.i1081.sroa.26.0.lcssa, ptr %history.i.i1081.sroa.26.0.hot_right.i1090.sroa_idx, align 16, !dbg !6234
  store <4 x i32> %history.i.i1081.sroa.29.0.lcssa, ptr %history.i.i1081.sroa.29.0.hot_right.i1090.sroa_idx, align 16, !dbg !6234
  store <4 x i32> %history.i.i1081.sroa.32.0.lcssa, ptr %history.i.i1081.sroa.32.0.hot_right.i1090.sroa_idx, align 16, !dbg !6234
  store <4 x i32> %history.i.i1081.sroa.35.0.lcssa, ptr %history.i.i1081.sroa.35.0.hot_right.i1090.sroa_idx, align 16, !dbg !6234
  store <4 x i32> %history.i.i1081.sroa.38.0.lcssa, ptr %history.i.i1081.sroa.38.0.hot_right.i1090.sroa_idx, align 16, !dbg !6234
  br i1 %_2.i697418377.not, label %bb13.i1108.loopexit, label %bb50.i1526.preheader, !dbg !6235

bb50.i1526.preheader:                             ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i1520
  %_5.i253115979.pre = load <4 x float>, ptr %126, align 16, !dbg !6241
  %_11.i253615981.pre = load <4 x float>, ptr %_68.i1529, align 16, !dbg !6242
  %_12.i2537.pre = load <4 x i32>, ptr %127, align 16, !dbg !6243
  %_13.i253915982.pre = load <16 x i8>, ptr %128, align 16, !dbg !6244
  %_5.i251815983.pre = load <4 x float>, ptr %129, align 16, !dbg !6245
  %_11.i252315985.pre = load <4 x float>, ptr %_69.i1530, align 16, !dbg !6246
  %_12.i2524.pre = load <4 x i32>, ptr %130, align 16, !dbg !6247
  %_13.i252615986.pre = load <16 x i8>, ptr %131, align 16, !dbg !6248
  %_5.i250515987.pre = load <4 x float>, ptr %132, align 16, !dbg !6249
  %_11.i251015989.pre = load <4 x float>, ptr %_73.i1533, align 16, !dbg !6250
  %_12.i2511.pre = load <4 x i32>, ptr %133, align 16, !dbg !6251
  %_13.i251315990.pre = load <16 x i8>, ptr %134, align 16, !dbg !6252
  %_5.i249615991.pre = load <4 x float>, ptr %135, align 16, !dbg !6253
  %.promoted22856 = load <4 x float>, ptr %149, align 16
  %.promoted22892 = load <4 x float>, ptr %166, align 16
  br label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5724

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5724: ; preds = %bb50.i1526.preheader, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit
  %420 = phi <4 x float> [ %.promoted22892, %bb50.i1526.preheader ], [ %593, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %421 = phi <4 x float> [ %.promoted22856, %bb50.i1526.preheader ], [ %503, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %_5.i249615991 = phi <4 x float> [ %_5.i249615991.pre, %bb50.i1526.preheader ], [ %452, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !6253
  %_12.i2511 = phi <4 x i32> [ %_12.i2511.pre, %bb50.i1526.preheader ], [ %464, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !6251
  %_11.i251015989 = phi <4 x float> [ %_11.i251015989.pre, %bb50.i1526.preheader ], [ %463, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !6250
  %_5.i250515987 = phi <4 x float> [ %_5.i250515987.pre, %bb50.i1526.preheader ], [ %442, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !6249
  %_12.i2524 = phi <4 x i32> [ %_12.i2524.pre, %bb50.i1526.preheader ], [ %462, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !6247
  %_11.i252315985 = phi <4 x float> [ %_11.i252315985.pre, %bb50.i1526.preheader ], [ %461, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !6246
  %_5.i251815983 = phi <4 x float> [ %_5.i251815983.pre, %bb50.i1526.preheader ], [ %433, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !6245
  %_12.i2537 = phi <4 x i32> [ %_12.i2537.pre, %bb50.i1526.preheader ], [ %460, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !6243
  %_11.i253615981 = phi <4 x float> [ %_11.i253615981.pre, %bb50.i1526.preheader ], [ %459, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !6242
  %_5.i253115979 = phi <4 x float> [ %_5.i253115979.pre, %bb50.i1526.preheader ], [ %424, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !6241
  %main_cursor.sroa.0.1.i152418455 = phi i32 [ %main_cursor.sroa.0.0.i111218462, %bb50.i1526.preheader ], [ %spec.store.select11.i1743, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %ring_cursor.sroa.0.1.i152318454 = phi i32 [ %ring_cursor.sroa.0.0.i111118461, %bb50.i1526.preheader ], [ %spec.store.select12.i1745, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %iter1.sroa.0.0.i152218453 = phi i32 [ 0, %bb50.i1526.preheader ], [ %449, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %422 = fadd <4 x float> %_5.i253115979, splat (float -1.000000e+00), !dbg !6254
  %423 = fcmp ogt <4 x float> %422, zeroinitializer, !dbg !6262
  %424 = select <4 x i1> %423, <4 x float> %422, <4 x float> zeroinitializer, !dbg !6266
  %425 = sext <4 x i1> %423 to <4 x i32>, !dbg !6267
  %426 = bitcast <4 x i32> %_12.i2537 to <4 x float>, !dbg !6278
  %427 = fadd <4 x float> %_11.i253615981, %426, !dbg !6282
  %428 = bitcast <4 x float> %427 to <16 x i8>, !dbg !6283
  %429 = bitcast <4 x i32> %425 to <16 x i8>, !dbg !6290
  %_4.i7037 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %428, <16 x i8> %_13.i253915982.pre, <16 x i8> %429), !dbg !6291
  %430 = bitcast <4 x i32> %_12.i2537 to <16 x i8>, !dbg !6292
  %_4.i7038 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %430, <16 x i8> zeroinitializer, <16 x i8> %429), !dbg !6296
  %431 = fadd <4 x float> %_5.i251815983, splat (float -1.000000e+00), !dbg !6297
  %432 = fcmp ogt <4 x float> %431, zeroinitializer, !dbg !6301
  %433 = select <4 x i1> %432, <4 x float> %431, <4 x float> zeroinitializer, !dbg !6305
  %434 = sext <4 x i1> %432 to <4 x i32>, !dbg !6306
  %435 = bitcast <4 x i32> %_12.i2524 to <4 x float>, !dbg !6311
  %436 = fadd <4 x float> %_11.i252315985, %435, !dbg !6315
  %437 = bitcast <4 x float> %436 to <16 x i8>, !dbg !6316
  %438 = bitcast <4 x i32> %434 to <16 x i8>, !dbg !6320
  %_4.i7039 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %437, <16 x i8> %_13.i252615986.pre, <16 x i8> %438), !dbg !6321
  %439 = bitcast <4 x i32> %_12.i2524 to <16 x i8>, !dbg !6322
  %_4.i7040 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %439, <16 x i8> zeroinitializer, <16 x i8> %438), !dbg !6326
  %440 = fadd <4 x float> %_5.i250515987, splat (float -1.000000e+00), !dbg !6327
  %441 = fcmp ogt <4 x float> %440, zeroinitializer, !dbg !6331
  %442 = select <4 x i1> %441, <4 x float> %440, <4 x float> zeroinitializer, !dbg !6335
  %443 = sext <4 x i1> %441 to <4 x i32>, !dbg !6336
  %444 = bitcast <4 x i32> %_12.i2511 to <4 x float>, !dbg !6341
  %445 = fadd <4 x float> %_11.i251015989, %444, !dbg !6345
  %446 = bitcast <4 x float> %445 to <16 x i8>, !dbg !6346
  %447 = bitcast <4 x i32> %443 to <16 x i8>, !dbg !6350
  %_4.i7041 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %446, <16 x i8> %_13.i251315990.pre, <16 x i8> %447), !dbg !6351
  %448 = bitcast <4 x i32> %_12.i2511 to <16 x i8>, !dbg !6352
  %_4.i7042 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %448, <16 x i8> zeroinitializer, <16 x i8> %447), !dbg !6356
  %449 = add nuw nsw i32 %iter1.sroa.0.0.i152218453, 1, !dbg !6357
  %_64.i1527 = add nuw nsw i32 %iter1.sroa.0.0.i152218453, %iter.sroa.0.0.i110918459, !dbg !6363
  %base.i1528 = shl i32 %_64.i1527, 2, !dbg !6363
  %450 = fadd <4 x float> %_5.i249615991, splat (float -1.000000e+00), !dbg !6364
  %451 = fcmp ogt <4 x float> %450, zeroinitializer, !dbg !6368
  %452 = select <4 x i1> %451, <4 x float> %450, <4 x float> zeroinitializer, !dbg !6372
  %453 = sext <4 x i1> %451 to <4 x i32>, !dbg !6373
  %_11.i249815993 = load <4 x float>, ptr %_74.i1534, align 16, !dbg !6378
  %_12.i2499 = load <4 x i32>, ptr %136, align 16, !dbg !6379
  %454 = bitcast <4 x i32> %_12.i2499 to <4 x float>, !dbg !6380
  %455 = fadd <4 x float> %_11.i249815993, %454, !dbg !6384
  %_13.i250115994 = load <16 x i8>, ptr %137, align 16, !dbg !6385
  %456 = bitcast <4 x float> %455 to <16 x i8>, !dbg !6386
  %457 = bitcast <4 x i32> %453 to <16 x i8>, !dbg !6390
  %_4.i7043 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %456, <16 x i8> %_13.i250115994, <16 x i8> %457), !dbg !6391
  store <16 x i8> %_4.i7043, ptr %_74.i1534, align 16, !dbg !6392
  %458 = bitcast <4 x i32> %_12.i2499 to <16 x i8>, !dbg !6393
  %_4.i7044 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %458, <16 x i8> zeroinitializer, <16 x i8> %457), !dbg !6397
  store <16 x i8> %_4.i7044, ptr %136, align 16, !dbg !6398
  %_78.i1539 = shl i32 %iter1.sroa.0.0.i152218453, 2, !dbg !6399
  %459 = bitcast <16 x i8> %_4.i7037 to <4 x float>, !dbg !6400
  %460 = bitcast <16 x i8> %_4.i7038 to <4 x i32>, !dbg !6400
  %461 = bitcast <16 x i8> %_4.i7039 to <4 x float>, !dbg !6400
  %462 = bitcast <16 x i8> %_4.i7040 to <4 x i32>, !dbg !6400
  %463 = bitcast <16 x i8> %_4.i7041 to <4 x float>, !dbg !6400
  %464 = bitcast <16 x i8> %_4.i7042 to <4 x i32>, !dbg !6400
  %_159.i1543 = getelementptr inbounds nuw float, ptr %peaks_left.i1087, i32 %_78.i1539, !dbg !6411
  %lanes.i5717.sroa.0.0.copyload = load <4 x i32>, ptr %_159.i1543, align 4, !dbg !6415, !alias.scope !6420, !noalias !6424
  %_164.i1545 = getelementptr inbounds nuw float, ptr %peaks_right.i1086, i32 %_78.i1539, !dbg !6428
  %lanes.i5708.sroa.0.0.copyload = load <4 x i32>, ptr %_164.i1545, align 4, !dbg !6438, !alias.scope !6443, !noalias !6447
  %465 = bitcast <4 x i32> %lanes.i5717.sroa.0.0.copyload to <4 x float>, !dbg !6451
  %466 = bitcast <4 x i32> %lanes.i5708.sroa.0.0.copyload to <4 x float>, !dbg !6455
  %467 = fcmp olt <4 x float> %465, %466, !dbg !6456
  %.v = select <4 x i1> %467, <4 x i32> %lanes.i5708.sroa.0.0.copyload, <4 x i32> %lanes.i5717.sroa.0.0.copyload, !dbg !6457
  %468 = bitcast <4 x i32> %.v to <16 x i8>, !dbg !6458
  %469 = bitcast <4 x i32> %lanes.i5717.sroa.0.0.copyload to <16 x i8>, !dbg !6462
  %_4.i7047 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %468, <16 x i8> %469, <16 x i8> %138), !dbg !6463
  %470 = bitcast <4 x i32> %lanes.i5708.sroa.0.0.copyload to <16 x i8>, !dbg !6464
  %_4.i7048 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %468, <16 x i8> %470, <16 x i8> %138), !dbg !6468
  %_165.i1550 = icmp ugt i32 %base.i1528, %left_io.1, !dbg !6469
  br i1 %_165.i1550, label %bb54.i1757, label %bb55.i1551, !dbg !6469, !prof !787

bb55.i1551:                                       ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5724
  %_167.i1552 = sub nuw nsw i32 %left_io.1, %base.i1528, !dbg !6473
  %_171.i1553 = getelementptr inbounds nuw float, ptr %left_io.0, i32 %base.i1528, !dbg !6474
  %_8.i5702 = icmp samesign ugt i32 %_167.i1552, 3, !dbg !6479
  br i1 %_8.i5702, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5706, label %bb2.i5703, !dbg !6479, !prof !1039

bb2.i5703:                                        ; preds = %bb55.i1551
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %421, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_167.i1552, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !6484, !noalias !6485
  unreachable, !dbg !6484

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5706: ; preds = %bb55.i1551
  %lanes.i5699.sroa.0.0.copyload = load <4 x i32>, ptr %_171.i1553, align 4, !dbg !6489, !alias.scope !6493, !noalias !6497
  %_91.i1555 = load i32, ptr %139, align 4, !dbg !6499, !alias.scope !5075, !noalias !6500, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6501), !dbg !6504
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6505), !dbg !6504
  %width.i30.i1557 = load i32, ptr %140, align 4, !dbg !6507, !alias.scope !6508, !noalias !6509, !noundef !10
  %471 = bitcast <16 x i8> %_4.i7047 to <4 x float>, !dbg !6517
  %472 = bitcast <16 x i8> %_4.i7037 to <4 x float>, !dbg !6522
  %473 = fcmp ogt <4 x float> %471, %472, !dbg !6523
  %474 = sext <4 x i1> %473 to <4 x i32>, !dbg !6523
  %475 = fdiv <4 x float> %472, %471, !dbg !6524
  %476 = bitcast <4 x float> %475 to <16 x i8>, !dbg !6532
  %477 = bitcast <4 x i32> %474 to <16 x i8>, !dbg !6536
  %_4.i7050 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %476, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %477), !dbg !6537
  %_158.1.i35.i1562 = load i32, ptr %141, align 4, !dbg !6538, !alias.scope !6508, !noalias !6509, !noundef !10
  %_22.i36.i1563 = mul i32 %width.i30.i1557, %ring_cursor.sroa.0.1.i152318454, !dbg !6539
  %_90.i37.i1564 = icmp ugt i32 %_22.i36.i1563, %_158.1.i35.i1562, !dbg !6540
  br i1 %_90.i37.i1564, label %bb34.i122.i1756, label %bb35.i38.i1565, !dbg !6540, !prof !787

bb35.i38.i1565:                                   ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5706
  %_93.i40.i1567 = sub nuw i32 %_158.1.i35.i1562, %_22.i36.i1563, !dbg !6545
  %_8.i6447 = icmp samesign ugt i32 %_93.i40.i1567, 3, !dbg !6546
  br i1 %_8.i6447, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6450, label %bb2.i6448, !dbg !6546, !prof !1039

bb2.i6448:                                        ; preds = %bb35.i38.i1565
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %421, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_93.i40.i1567, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !6551, !noalias !6552
  unreachable, !dbg !6551

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6450: ; preds = %bb35.i38.i1565
  %_158.0.i39.i1566 = load ptr, ptr %142, align 4, !dbg !6538, !alias.scope !6508, !noalias !6509, !nonnull !10, !noundef !10
  %_97.i41.i1568 = getelementptr inbounds nuw float, ptr %_158.0.i39.i1566, i32 %_22.i36.i1563, !dbg !6556
  store <16 x i8> %_4.i7050, ptr %_97.i41.i1568, align 4, !dbg !6561, !alias.scope !6565, !noalias !6569
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6571), !dbg !6574
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6575), !dbg !6574
  %width.i2120 = load i32, ptr %140, align 4, !dbg !6577, !alias.scope !6571, !noalias !6580, !noundef !10
  %478 = icmp eq i32 %width.i2120, 0, !dbg !6581
  br i1 %478, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2226, label %bb29.i2126.lr.ph, !dbg !6581

bb29.i2126.lr.ph:                                 ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6450
  %_126.1.i2131 = load i32, ptr %62, align 4, !alias.scope !6571, !noalias !6580, !noundef !10
  %_126.0.i2135 = load ptr, ptr %61, align 4, !nonnull !10
  %479 = add i32 %ring_cursor.sroa.0.1.i152318454, 1
  %_21.not.i2140 = icmp ult i32 %479, %_91.i1555
  %480 = select i1 %_21.not.i2140, i32 0, i32 %_91.i1555
  %start1.sroa.0.0.i2141 = sub nuw i32 %479, %480
  %_128.1.i2144 = load i32, ptr %141, align 4
  %_128.0.i2148 = load ptr, ptr %142, align 4, !nonnull !10
  %_130.1.i2149 = load i32, ptr %143, align 4
  %_130.0.i2153 = load ptr, ptr %144, align 4, !nonnull !10
  %_132.1.i2156 = load i32, ptr %145, align 4
  %_132.0.i2160 = load ptr, ptr %146, align 4, !nonnull !10
  %_43.i2173 = mul i32 %width.i2120, %start1.sroa.0.0.i2141
  br label %bb29.i2126, !dbg !6581

bb29.i2126:                                       ; preds = %bb29.i2126.lr.ph, %bb28.i2188
  %iter.sroa.0.0.idx.i212418434 = phi i32 [ 0, %bb29.i2126.lr.ph ], [ %iter.sroa.0.0.add.i2129, %bb28.i2188 ]
  %iter.sroa.4.0.i212318433 = phi i32 [ 0, %bb29.i2126.lr.ph ], [ %_102.0.i2130, %bb28.i2188 ]
  %iter.sroa.7.0.i212218432 = phi i32 [ %width.i2120, %bb29.i2126.lr.ph ], [ %481, %bb28.i2188 ]
  %iter.sroa.0.0.ptr.i212518435 = getelementptr inbounds nuw i8, ptr %scratch.i1088, i32 %iter.sroa.0.0.idx.i212418434, !dbg !6590
  %481 = add i32 %iter.sroa.7.0.i212218432, -1, !dbg !6590
  %_109.i2127 = icmp eq i32 %iter.sroa.0.0.idx.i212418434, 32, !dbg !6591
  br i1 %_109.i2127, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2226, label %bb33.i2128, !dbg !6600

bb33.i2128:                                       ; preds = %bb29.i2126
  %iter.sroa.0.0.add.i2129 = add nuw nsw i32 %iter.sroa.0.0.idx.i212418434, 4, !dbg !6601
  %_102.0.i2130 = add nuw nsw i32 %iter.sroa.4.0.i212318433, 1, !dbg !6604
  %exitcond20901.not = icmp eq i32 %iter.sroa.4.0.i212318433, %_126.1.i2131, !dbg !6607
  br i1 %exitcond20901.not, label %panic.i2133, label %bb2.i2134, !dbg !6607

bb2.i2134:                                        ; preds = %bb33.i2128
  %482 = getelementptr inbounds nuw %LaneShape, ptr %_126.0.i2135, i32 %iter.sroa.4.0.i212318433, !dbg !6607
  %shape.i2136 = load i32, ptr %482, align 4, !dbg !6607, !noalias !6609, !noundef !10
  %483 = getelementptr inbounds nuw i8, ptr %482, i32 4, !dbg !6607
  %shape3.i2137 = load i32, ptr %483, align 4, !dbg !6607, !noalias !6609, !noundef !10
  %484 = add i32 %shape3.i2137, %ring_cursor.sroa.0.1.i152318454, !dbg !6610
  %_18.not.i2138 = icmp ult i32 %484, %_91.i1555, !dbg !6613
  %485 = select i1 %_18.not.i2138, i32 0, i32 %_91.i1555, !dbg !6613
  %spec.select.i2139 = sub nuw i32 %484, %485, !dbg !6613
  %_25.i2142 = mul i32 %spec.select.i2139, %width.i2120, !dbg !6615
  %_24.i2143 = add i32 %_25.i2142, %iter.sroa.4.0.i212318433, !dbg !6615
  %_28.i2145 = icmp ult i32 %_24.i2143, %_128.1.i2144, !dbg !6617
  br i1 %_28.i2145, label %bb9.i2147, label %panic5.i2146, !dbg !6617

panic.i2133:                                      ; preds = %bb33.i2128
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %421, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_126.1.i2131, i32 noundef %_126.1.i2131, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_12856b5f9033764dbf23e04eb77c5c46) #32, !dbg !6607, !noalias !6609
  unreachable, !dbg !6607

bb9.i2147:                                        ; preds = %bb2.i2134
  %486 = getelementptr inbounds nuw float, ptr %_128.0.i2148, i32 %_24.i2143, !dbg !6617
  %487 = load float, ptr %486, align 4, !dbg !6617, !noalias !6609, !noundef !10
  %exitcond20902.not = icmp eq i32 %iter.sroa.4.0.i212318433, %_130.1.i2149, !dbg !6618
  br i1 %exitcond20902.not, label %panic6.i2151, label %bb10.i2152, !dbg !6618

panic5.i2146:                                     ; preds = %bb2.i2134
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %421, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_24.i2143, i32 noundef %_128.1.i2144, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70ebf0cf1c90c6161926611ccf249a32) #32, !dbg !6617, !noalias !6609
  unreachable, !dbg !6617

bb10.i2152:                                       ; preds = %bb9.i2147
  %488 = getelementptr inbounds nuw i32, ptr %_130.0.i2153, i32 %iter.sroa.4.0.i212318433, !dbg !6618
  %_30.i2154 = load i32, ptr %488, align 4, !dbg !6618, !noalias !6609, !noundef !10
  %489 = icmp eq i32 %_30.i2154, 0, !dbg !6620
  br i1 %489, label %bb14.i2163, label %bb12.i2155, !dbg !6620

panic6.i2151:                                     ; preds = %bb9.i2147
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %421, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_130.1.i2149, i32 noundef %_130.1.i2149, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_023d591aaad7f5cf482ac80a9401eca1) #32, !dbg !6618, !noalias !6609
  unreachable, !dbg !6618

bb12.i2155:                                       ; preds = %bb10.i2152
  %_35.i2157 = icmp ult i32 %iter.sroa.4.0.i212318433, %_132.1.i2156, !dbg !6622
  br i1 %_35.i2157, label %bb13.i2159, label %panic7.i2158, !dbg !6622

bb14.i2163:                                       ; preds = %bb34.i2224, %bb13.i2159, %bb10.i2152
  %newest.sroa.0.0.i2164 = phi float [ %487, %bb10.i2152 ], [ %_33.i2161, %bb34.i2224 ], [ %487, %bb13.i2159 ], !dbg !6623
  %exitcond20903.not = icmp eq i32 %iter.sroa.4.0.i212318433, %_132.1.i2156, !dbg !6624
  br i1 %exitcond20903.not, label %panic8.i2167, label %bb15.i2168, !dbg !6624

bb13.i2159:                                       ; preds = %bb12.i2155
  %490 = getelementptr inbounds nuw float, ptr %_132.0.i2160, i32 %iter.sroa.4.0.i212318433, !dbg !6622
  %_33.i2161 = load float, ptr %490, align 4, !dbg !6622, !noalias !6609, !noundef !10
  %_116.i2162 = fcmp olt float %_33.i2161, %487, !dbg !6626
  br i1 %_116.i2162, label %bb34.i2224, label %bb14.i2163, !dbg !6626

panic7.i2158:                                     ; preds = %bb12.i2155
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %421, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %iter.sroa.4.0.i212318433, i32 noundef %_132.1.i2156, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a228cac3279aae3a34886f59f51fbe9e) #32, !dbg !6622, !noalias !6609
  unreachable, !dbg !6622

bb34.i2224:                                       ; preds = %bb13.i2159
  br label %bb14.i2163, !dbg !6629

bb15.i2168:                                       ; preds = %bb14.i2163
  %491 = getelementptr inbounds nuw float, ptr %_132.0.i2160, i32 %iter.sroa.4.0.i212318433, !dbg !6624
  store float %newest.sroa.0.0.i2164, ptr %491, align 4, !dbg !6624, !noalias !6609
  %_40.i2170 = add i32 %_30.i2154, 1, !dbg !6630
  %complete.i2171 = icmp eq i32 %_40.i2170, %shape.i2136, !dbg !6630
  br i1 %complete.i2171, label %bb19.i2193, label %bb17.i2172, !dbg !6631

panic8.i2167:                                     ; preds = %bb14.i2163
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %421, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_132.1.i2156, i32 noundef %_132.1.i2156, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1fa5d88c1a2cc292a2767c48606a38fe) #32, !dbg !6624, !noalias !6609
  unreachable, !dbg !6624

bb17.i2172:                                       ; preds = %bb15.i2168
  %_42.i2174 = add i32 %iter.sroa.4.0.i212318433, %_43.i2173, !dbg !6633
  %_45.i2176 = icmp ult i32 %_42.i2174, %_128.1.i2144, !dbg !6634
  br i1 %_45.i2176, label %bb27.i2186, label %panic9.i2177, !dbg !6634

panic9.i2177:                                     ; preds = %bb17.i2172
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %421, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_42.i2174, i32 noundef %_128.1.i2144, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70d40ae2302f3ea19fa0c4a65fe82786) #32, !dbg !6634, !noalias !6609
  unreachable, !dbg !6634

bb27.i2186:                                       ; preds = %bb17.i2172
  %492 = getelementptr inbounds nuw float, ptr %_128.0.i2148, i32 %_42.i2174, !dbg !6634
  %_41.i2180 = load float, ptr %492, align 4, !dbg !6634, !noalias !6609, !noundef !10
  %_117.i2181 = fcmp olt float %_41.i2180, %newest.sroa.0.0.i2164, !dbg !6635
  %newest.sroa.0.1.i2182 = select i1 %_117.i2181, float %_41.i2180, float %newest.sroa.0.0.i2164, !dbg !6635
  store float %newest.sroa.0.1.i2182, ptr %iter.sroa.0.0.ptr.i212518435, align 4, !dbg !6637, !alias.scope !6575, !noalias !6638
  br label %bb28.i2188, !dbg !6639

bb28.i2188:                                       ; preds = %bb22.i2221, %bb19.i2193, %bb27.i2186
  %storemerge = phi i32 [ %_40.i2170, %bb27.i2186 ], [ 0, %bb19.i2193 ], [ 0, %bb22.i2221 ], !dbg !6640
  store i32 %storemerge, ptr %488, align 4, !dbg !6640, !noalias !6609
  %493 = icmp eq i32 %481, 0, !dbg !6581
  br i1 %493, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2226, label %bb29.i2126, !dbg !6581

bb19.i2193:                                       ; preds = %bb15.i2168
  store float %newest.sroa.0.0.i2164, ptr %iter.sroa.0.0.ptr.i212518435, align 4, !dbg !6637, !alias.scope !6575, !noalias !6638
  %_118.i219918428.not = icmp eq i32 %shape.i2136, 0, !dbg !6641
  br i1 %_118.i219918428.not, label %bb28.i2188, label %bb40.i2206.preheader, !dbg !6652

bb40.i2206.preheader:                             ; preds = %bb19.i2193
  %494 = load float, ptr %486, align 4, !dbg !6653, !noalias !6609, !noundef !10
  br label %bb40.i2206, !dbg !6654

bb40.i2206:                                       ; preds = %bb40.i2206.preheader, %bb22.i2221
  %iter2.sroa.0.0.i219818431 = phi i32 [ %_119.i2207, %bb22.i2221 ], [ 0, %bb40.i2206.preheader ]
  %suffix.sroa.0.0.i219718430 = phi float [ %suffix.sroa.0.1.i2217, %bb22.i2221 ], [ %494, %bb40.i2206.preheader ]
  %end.sroa.0.1.i219618429 = phi i32 [ %497, %bb22.i2221 ], [ %spec.select.i2139, %bb40.i2206.preheader ]
  %_54.i2208 = mul i32 %end.sroa.0.1.i219618429, %width.i2120, !dbg !6655
  %_53.i2209 = add i32 %_54.i2208, %iter.sroa.4.0.i212318433, !dbg !6655
  %_57.i2211 = icmp ult i32 %_53.i2209, %_128.1.i2144, !dbg !6654
  br i1 %_57.i2211, label %bb22.i2221, label %panic13.i2212, !dbg !6654

panic13.i2212:                                    ; preds = %bb40.i2206
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %421, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_53.i2209, i32 noundef %_128.1.i2144, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a271bbb7fd83c3605ea58a06b7065fa4) #32, !dbg !6654, !noalias !6609
  unreachable, !dbg !6654

bb22.i2221:                                       ; preds = %bb40.i2206
  %_119.i2207 = add nuw i32 %iter2.sroa.0.0.i219818431, 1, !dbg !6656
  %495 = getelementptr inbounds nuw float, ptr %_128.0.i2148, i32 %_53.i2209, !dbg !6654
  %_52.i2215 = load float, ptr %495, align 4, !dbg !6654, !noalias !6609, !noundef !10
  %_121.i2216 = fcmp olt float %suffix.sroa.0.0.i219718430, %_52.i2215, !dbg !6662
  %suffix.sroa.0.1.i2217 = select i1 %_121.i2216, float %suffix.sroa.0.0.i219718430, float %_52.i2215, !dbg !6662
  store float %suffix.sroa.0.1.i2217, ptr %495, align 4, !dbg !6664, !noalias !6609
  %496 = icmp eq i32 %end.sroa.0.1.i219618429, 0, !dbg !6665
  %spec.store.select.i2223 = select i1 %496, i32 %_91.i1555, i32 %end.sroa.0.1.i219618429, !dbg !6665
  %497 = add i32 %spec.store.select.i2223, -1, !dbg !6666
  %exitcond20900.not = icmp eq i32 %_119.i2207, %shape.i2136, !dbg !6641
  br i1 %exitcond20900.not, label %bb28.i2188, label %bb40.i2206, !dbg !6652

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2226: ; preds = %bb29.i2126, %bb28.i2188, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6450
  %lanes.i5692.sroa.0.0.copyload = load <4 x float>, ptr %scratch.i1088, align 4, !dbg !6667, !alias.scope !6672, !noalias !6676
  %498 = fmul <4 x float> %lanes.i5692.sroa.0.0.copyload, splat (float 1.638400e+04), !dbg !6680
  %499 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %498), !dbg !6684
  %500 = fmul <4 x float> %499, splat (float 0x3F10000000000000), !dbg !6691
  %501 = icmp eq i32 %width.i30.i1557, 0, !dbg !6695
  %_163.1.i79.i1606.pre = load i32, ptr %147, align 4, !dbg !6700, !alias.scope !6508, !noalias !6509
  br i1 %501, label %bb53.i74.i1601, label %bb36.i53.i1580.lr.ph, !dbg !6695

bb36.i53.i1580.lr.ph:                             ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2226
  %_159.1.i58.i1585 = load i32, ptr %62, align 4, !alias.scope !6508, !noalias !6509, !noundef !10
  %_159.0.i62.i1589 = load ptr, ptr %61, align 4, !nonnull !10
  %_161.0.i72.i1599 = load ptr, ptr %148, align 4, !nonnull !10
  %exitcond20906.not = icmp eq i32 %_159.1.i58.i1585, 0, !dbg !6701
  br i1 %exitcond20906.not, label %panic.i60.i1587, label %bb14.i61.i1588, !dbg !6701

bb34.i122.i1756:                                  ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5706
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %421, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i36.i1563, i32 noundef %_158.1.i35.i1562, i32 noundef %_158.1.i35.i1562, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_db7c42041d7aaaba4839906f37294547) #32, !dbg !6703, !noalias !6704
  unreachable, !dbg !6703

bb53.i74.i1601.loopexit:                          ; preds = %bb18.i71.i1598.7, %bb18.i71.i1598.6, %bb18.i71.i1598.5, %bb18.i71.i1598.4, %bb18.i71.i1598.3, %bb18.i71.i1598.2, %bb18.i71.i1598.1, %bb18.i71.i1598
  %lanes.i5685.sroa.0.0.copyload.pre = load <4 x float>, ptr %scratch.i1088, align 4, !dbg !6705, !alias.scope !6710, !noalias !6714
  br label %bb53.i74.i1601, !dbg !6718

bb53.i74.i1601:                                   ; preds = %bb53.i74.i1601.loopexit, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2226
  %lanes.i5685.sroa.0.0.copyload = phi <4 x float> [ %lanes.i5685.sroa.0.0.copyload.pre, %bb53.i74.i1601.loopexit ], [ %lanes.i5692.sroa.0.0.copyload, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2226 ], !dbg !6705
  %502 = fadd <4 x float> %500, %421, !dbg !6719
  %503 = fsub <4 x float> %502, %lanes.i5685.sroa.0.0.copyload, !dbg !6723
  %_123.i80.i1607 = icmp ugt i32 %_22.i36.i1563, %_163.1.i79.i1606.pre, !dbg !6727
  br i1 %_123.i80.i1607, label %bb41.i121.i1755, label %bb42.i81.i1608, !dbg !6727, !prof !787

bb14.i61.i1588:                                   ; preds = %bb36.i53.i1580.lr.ph
  %504 = getelementptr inbounds nuw i8, ptr %_159.0.i62.i1589, i32 8, !dbg !6701
  %_42.i63.i1590 = load i32, ptr %504, align 4, !dbg !6701, !noalias !6731, !noundef !10
  %505 = add i32 %_42.i63.i1590, %ring_cursor.sroa.0.1.i152318454, !dbg !6732
  %_45.not.i64.i1591 = icmp ult i32 %505, %_91.i1555, !dbg !6733
  %506 = select i1 %_45.not.i64.i1591, i32 0, i32 %_91.i1555, !dbg !6733
  %spec.select.i65.i1592 = sub nuw i32 %505, %506, !dbg !6733
  %_49.i66.i1593 = mul i32 %spec.select.i65.i1592, %width.i30.i1557, !dbg !6735
  %_51.i69.i1596 = icmp ult i32 %_49.i66.i1593, %_163.1.i79.i1606.pre, !dbg !6736
  br i1 %_51.i69.i1596, label %bb18.i71.i1598, label %panic1.i70.i1597, !dbg !6736

panic.i60.i1587:                                  ; preds = %bb36.i53.i1580.7, %bb36.i53.i1580.6, %bb36.i53.i1580.5, %bb36.i53.i1580.4, %bb36.i53.i1580.3, %bb36.i53.i1580.2, %bb36.i53.i1580.1, %bb36.i53.i1580.lr.ph
  %_159.1.i58.i1585.lcssa.ph = phi i32 [ 7, %bb36.i53.i1580.7 ], [ 6, %bb36.i53.i1580.6 ], [ 5, %bb36.i53.i1580.5 ], [ 4, %bb36.i53.i1580.4 ], [ 3, %bb36.i53.i1580.3 ], [ 2, %bb36.i53.i1580.2 ], [ 1, %bb36.i53.i1580.1 ], [ 0, %bb36.i53.i1580.lr.ph ]
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %421, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_159.1.i58.i1585.lcssa.ph, i32 noundef %_159.1.i58.i1585.lcssa.ph, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_33746731a929bad63709ddedbca82f5f) #32, !dbg !6701, !noalias !6731
  unreachable, !dbg !6701

bb18.i71.i1598:                                   ; preds = %bb14.i61.i1588
  %507 = getelementptr inbounds nuw float, ptr %_161.0.i72.i1599, i32 %_49.i66.i1593, !dbg !6736
  %_47.i73.i1600 = load float, ptr %507, align 4, !dbg !6736, !noalias !6731, !noundef !10
  store float %_47.i73.i1600, ptr %scratch.i1088, align 4, !dbg !6737, !alias.scope !6505, !noalias !6738
  %508 = icmp eq i32 %width.i30.i1557, 1, !dbg !6695
  br i1 %508, label %bb53.i74.i1601.loopexit, label %bb36.i53.i1580.1, !dbg !6695

bb36.i53.i1580.1:                                 ; preds = %bb18.i71.i1598
  %exitcond20906.1.not = icmp eq i32 %_159.1.i58.i1585, 1, !dbg !6701
  br i1 %exitcond20906.1.not, label %panic.i60.i1587, label %bb14.i61.i1588.1, !dbg !6701

bb14.i61.i1588.1:                                 ; preds = %bb36.i53.i1580.1
  %509 = getelementptr inbounds nuw i8, ptr %_159.0.i62.i1589, i32 20, !dbg !6701
  %_42.i63.i1590.1 = load i32, ptr %509, align 4, !dbg !6701, !noalias !6731, !noundef !10
  %510 = add i32 %_42.i63.i1590.1, %ring_cursor.sroa.0.1.i152318454, !dbg !6732
  %_45.not.i64.i1591.1 = icmp ult i32 %510, %_91.i1555, !dbg !6733
  %511 = select i1 %_45.not.i64.i1591.1, i32 0, i32 %_91.i1555, !dbg !6733
  %spec.select.i65.i1592.1 = sub nuw i32 %510, %511, !dbg !6733
  %_49.i66.i1593.1 = mul i32 %spec.select.i65.i1592.1, %width.i30.i1557, !dbg !6735
  %_48.i67.i1594.1 = add i32 %_49.i66.i1593.1, 1, !dbg !6735
  %_51.i69.i1596.1 = icmp ult i32 %_48.i67.i1594.1, %_163.1.i79.i1606.pre, !dbg !6736
  br i1 %_51.i69.i1596.1, label %bb18.i71.i1598.1, label %panic1.i70.i1597, !dbg !6736

bb18.i71.i1598.1:                                 ; preds = %bb14.i61.i1588.1
  %512 = getelementptr inbounds nuw float, ptr %_161.0.i72.i1599, i32 %_48.i67.i1594.1, !dbg !6736
  %_47.i73.i1600.1 = load float, ptr %512, align 4, !dbg !6736, !noalias !6731, !noundef !10
  store float %_47.i73.i1600.1, ptr %iter.sroa.0.0.ptr.i52.i157918439.1, align 4, !dbg !6737, !alias.scope !6505, !noalias !6738
  %513 = icmp eq i32 %width.i30.i1557, 2, !dbg !6695
  br i1 %513, label %bb53.i74.i1601.loopexit, label %bb36.i53.i1580.2, !dbg !6695

bb36.i53.i1580.2:                                 ; preds = %bb18.i71.i1598.1
  %exitcond20906.2.not = icmp eq i32 %_159.1.i58.i1585, 2, !dbg !6701
  br i1 %exitcond20906.2.not, label %panic.i60.i1587, label %bb14.i61.i1588.2, !dbg !6701

bb14.i61.i1588.2:                                 ; preds = %bb36.i53.i1580.2
  %514 = getelementptr inbounds nuw i8, ptr %_159.0.i62.i1589, i32 32, !dbg !6701
  %_42.i63.i1590.2 = load i32, ptr %514, align 4, !dbg !6701, !noalias !6731, !noundef !10
  %515 = add i32 %_42.i63.i1590.2, %ring_cursor.sroa.0.1.i152318454, !dbg !6732
  %_45.not.i64.i1591.2 = icmp ult i32 %515, %_91.i1555, !dbg !6733
  %516 = select i1 %_45.not.i64.i1591.2, i32 0, i32 %_91.i1555, !dbg !6733
  %spec.select.i65.i1592.2 = sub nuw i32 %515, %516, !dbg !6733
  %_49.i66.i1593.2 = mul i32 %spec.select.i65.i1592.2, %width.i30.i1557, !dbg !6735
  %_48.i67.i1594.2 = add i32 %_49.i66.i1593.2, 2, !dbg !6735
  %_51.i69.i1596.2 = icmp ult i32 %_48.i67.i1594.2, %_163.1.i79.i1606.pre, !dbg !6736
  br i1 %_51.i69.i1596.2, label %bb18.i71.i1598.2, label %panic1.i70.i1597, !dbg !6736

bb18.i71.i1598.2:                                 ; preds = %bb14.i61.i1588.2
  %517 = getelementptr inbounds nuw float, ptr %_161.0.i72.i1599, i32 %_48.i67.i1594.2, !dbg !6736
  %_47.i73.i1600.2 = load float, ptr %517, align 4, !dbg !6736, !noalias !6731, !noundef !10
  store float %_47.i73.i1600.2, ptr %iter.sroa.0.0.ptr.i52.i157918439.2, align 4, !dbg !6737, !alias.scope !6505, !noalias !6738
  %518 = icmp eq i32 %width.i30.i1557, 3, !dbg !6695
  br i1 %518, label %bb53.i74.i1601.loopexit, label %bb36.i53.i1580.3, !dbg !6695

bb36.i53.i1580.3:                                 ; preds = %bb18.i71.i1598.2
  %exitcond20906.3.not = icmp eq i32 %_159.1.i58.i1585, 3, !dbg !6701
  br i1 %exitcond20906.3.not, label %panic.i60.i1587, label %bb14.i61.i1588.3, !dbg !6701

bb14.i61.i1588.3:                                 ; preds = %bb36.i53.i1580.3
  %519 = getelementptr inbounds nuw i8, ptr %_159.0.i62.i1589, i32 44, !dbg !6701
  %_42.i63.i1590.3 = load i32, ptr %519, align 4, !dbg !6701, !noalias !6731, !noundef !10
  %520 = add i32 %_42.i63.i1590.3, %ring_cursor.sroa.0.1.i152318454, !dbg !6732
  %_45.not.i64.i1591.3 = icmp ult i32 %520, %_91.i1555, !dbg !6733
  %521 = select i1 %_45.not.i64.i1591.3, i32 0, i32 %_91.i1555, !dbg !6733
  %spec.select.i65.i1592.3 = sub nuw i32 %520, %521, !dbg !6733
  %_49.i66.i1593.3 = mul i32 %spec.select.i65.i1592.3, %width.i30.i1557, !dbg !6735
  %_48.i67.i1594.3 = add i32 %_49.i66.i1593.3, 3, !dbg !6735
  %_51.i69.i1596.3 = icmp ult i32 %_48.i67.i1594.3, %_163.1.i79.i1606.pre, !dbg !6736
  br i1 %_51.i69.i1596.3, label %bb18.i71.i1598.3, label %panic1.i70.i1597, !dbg !6736

bb18.i71.i1598.3:                                 ; preds = %bb14.i61.i1588.3
  %522 = getelementptr inbounds nuw float, ptr %_161.0.i72.i1599, i32 %_48.i67.i1594.3, !dbg !6736
  %_47.i73.i1600.3 = load float, ptr %522, align 4, !dbg !6736, !noalias !6731, !noundef !10
  store float %_47.i73.i1600.3, ptr %iter.sroa.0.0.ptr.i52.i157918439.3, align 4, !dbg !6737, !alias.scope !6505, !noalias !6738
  %523 = icmp eq i32 %width.i30.i1557, 4, !dbg !6695
  br i1 %523, label %bb53.i74.i1601.loopexit, label %bb36.i53.i1580.4, !dbg !6695

bb36.i53.i1580.4:                                 ; preds = %bb18.i71.i1598.3
  %exitcond20906.4.not = icmp eq i32 %_159.1.i58.i1585, 4, !dbg !6701
  br i1 %exitcond20906.4.not, label %panic.i60.i1587, label %bb14.i61.i1588.4, !dbg !6701

bb14.i61.i1588.4:                                 ; preds = %bb36.i53.i1580.4
  %524 = getelementptr inbounds nuw i8, ptr %_159.0.i62.i1589, i32 56, !dbg !6701
  %_42.i63.i1590.4 = load i32, ptr %524, align 4, !dbg !6701, !noalias !6731, !noundef !10
  %525 = add i32 %_42.i63.i1590.4, %ring_cursor.sroa.0.1.i152318454, !dbg !6732
  %_45.not.i64.i1591.4 = icmp ult i32 %525, %_91.i1555, !dbg !6733
  %526 = select i1 %_45.not.i64.i1591.4, i32 0, i32 %_91.i1555, !dbg !6733
  %spec.select.i65.i1592.4 = sub nuw i32 %525, %526, !dbg !6733
  %_49.i66.i1593.4 = mul i32 %spec.select.i65.i1592.4, %width.i30.i1557, !dbg !6735
  %_48.i67.i1594.4 = add i32 %_49.i66.i1593.4, 4, !dbg !6735
  %_51.i69.i1596.4 = icmp ult i32 %_48.i67.i1594.4, %_163.1.i79.i1606.pre, !dbg !6736
  br i1 %_51.i69.i1596.4, label %bb18.i71.i1598.4, label %panic1.i70.i1597, !dbg !6736

bb18.i71.i1598.4:                                 ; preds = %bb14.i61.i1588.4
  %527 = getelementptr inbounds nuw float, ptr %_161.0.i72.i1599, i32 %_48.i67.i1594.4, !dbg !6736
  %_47.i73.i1600.4 = load float, ptr %527, align 4, !dbg !6736, !noalias !6731, !noundef !10
  store float %_47.i73.i1600.4, ptr %iter.sroa.0.0.ptr.i52.i157918439.4, align 4, !dbg !6737, !alias.scope !6505, !noalias !6738
  %528 = icmp eq i32 %width.i30.i1557, 5, !dbg !6695
  br i1 %528, label %bb53.i74.i1601.loopexit, label %bb36.i53.i1580.5, !dbg !6695

bb36.i53.i1580.5:                                 ; preds = %bb18.i71.i1598.4
  %exitcond20906.5.not = icmp eq i32 %_159.1.i58.i1585, 5, !dbg !6701
  br i1 %exitcond20906.5.not, label %panic.i60.i1587, label %bb14.i61.i1588.5, !dbg !6701

bb14.i61.i1588.5:                                 ; preds = %bb36.i53.i1580.5
  %529 = getelementptr inbounds nuw i8, ptr %_159.0.i62.i1589, i32 68, !dbg !6701
  %_42.i63.i1590.5 = load i32, ptr %529, align 4, !dbg !6701, !noalias !6731, !noundef !10
  %530 = add i32 %_42.i63.i1590.5, %ring_cursor.sroa.0.1.i152318454, !dbg !6732
  %_45.not.i64.i1591.5 = icmp ult i32 %530, %_91.i1555, !dbg !6733
  %531 = select i1 %_45.not.i64.i1591.5, i32 0, i32 %_91.i1555, !dbg !6733
  %spec.select.i65.i1592.5 = sub nuw i32 %530, %531, !dbg !6733
  %_49.i66.i1593.5 = mul i32 %spec.select.i65.i1592.5, %width.i30.i1557, !dbg !6735
  %_48.i67.i1594.5 = add i32 %_49.i66.i1593.5, 5, !dbg !6735
  %_51.i69.i1596.5 = icmp ult i32 %_48.i67.i1594.5, %_163.1.i79.i1606.pre, !dbg !6736
  br i1 %_51.i69.i1596.5, label %bb18.i71.i1598.5, label %panic1.i70.i1597, !dbg !6736

bb18.i71.i1598.5:                                 ; preds = %bb14.i61.i1588.5
  %532 = getelementptr inbounds nuw float, ptr %_161.0.i72.i1599, i32 %_48.i67.i1594.5, !dbg !6736
  %_47.i73.i1600.5 = load float, ptr %532, align 4, !dbg !6736, !noalias !6731, !noundef !10
  store float %_47.i73.i1600.5, ptr %iter.sroa.0.0.ptr.i52.i157918439.5, align 4, !dbg !6737, !alias.scope !6505, !noalias !6738
  %533 = icmp eq i32 %width.i30.i1557, 6, !dbg !6695
  br i1 %533, label %bb53.i74.i1601.loopexit, label %bb36.i53.i1580.6, !dbg !6695

bb36.i53.i1580.6:                                 ; preds = %bb18.i71.i1598.5
  %exitcond20906.6.not = icmp eq i32 %_159.1.i58.i1585, 6, !dbg !6701
  br i1 %exitcond20906.6.not, label %panic.i60.i1587, label %bb14.i61.i1588.6, !dbg !6701

bb14.i61.i1588.6:                                 ; preds = %bb36.i53.i1580.6
  %534 = getelementptr inbounds nuw i8, ptr %_159.0.i62.i1589, i32 80, !dbg !6701
  %_42.i63.i1590.6 = load i32, ptr %534, align 4, !dbg !6701, !noalias !6731, !noundef !10
  %535 = add i32 %_42.i63.i1590.6, %ring_cursor.sroa.0.1.i152318454, !dbg !6732
  %_45.not.i64.i1591.6 = icmp ult i32 %535, %_91.i1555, !dbg !6733
  %536 = select i1 %_45.not.i64.i1591.6, i32 0, i32 %_91.i1555, !dbg !6733
  %spec.select.i65.i1592.6 = sub nuw i32 %535, %536, !dbg !6733
  %_49.i66.i1593.6 = mul i32 %spec.select.i65.i1592.6, %width.i30.i1557, !dbg !6735
  %_48.i67.i1594.6 = add i32 %_49.i66.i1593.6, 6, !dbg !6735
  %_51.i69.i1596.6 = icmp ult i32 %_48.i67.i1594.6, %_163.1.i79.i1606.pre, !dbg !6736
  br i1 %_51.i69.i1596.6, label %bb18.i71.i1598.6, label %panic1.i70.i1597, !dbg !6736

bb18.i71.i1598.6:                                 ; preds = %bb14.i61.i1588.6
  %537 = getelementptr inbounds nuw float, ptr %_161.0.i72.i1599, i32 %_48.i67.i1594.6, !dbg !6736
  %_47.i73.i1600.6 = load float, ptr %537, align 4, !dbg !6736, !noalias !6731, !noundef !10
  store float %_47.i73.i1600.6, ptr %iter.sroa.0.0.ptr.i52.i157918439.6, align 4, !dbg !6737, !alias.scope !6505, !noalias !6738
  %538 = icmp eq i32 %width.i30.i1557, 7, !dbg !6695
  br i1 %538, label %bb53.i74.i1601.loopexit, label %bb36.i53.i1580.7, !dbg !6695

bb36.i53.i1580.7:                                 ; preds = %bb18.i71.i1598.6
  %exitcond20906.7.not = icmp eq i32 %_159.1.i58.i1585, 7, !dbg !6701
  br i1 %exitcond20906.7.not, label %panic.i60.i1587, label %bb14.i61.i1588.7, !dbg !6701

bb14.i61.i1588.7:                                 ; preds = %bb36.i53.i1580.7
  %539 = getelementptr inbounds nuw i8, ptr %_159.0.i62.i1589, i32 92, !dbg !6701
  %_42.i63.i1590.7 = load i32, ptr %539, align 4, !dbg !6701, !noalias !6731, !noundef !10
  %540 = add i32 %_42.i63.i1590.7, %ring_cursor.sroa.0.1.i152318454, !dbg !6732
  %_45.not.i64.i1591.7 = icmp ult i32 %540, %_91.i1555, !dbg !6733
  %541 = select i1 %_45.not.i64.i1591.7, i32 0, i32 %_91.i1555, !dbg !6733
  %spec.select.i65.i1592.7 = sub nuw i32 %540, %541, !dbg !6733
  %_49.i66.i1593.7 = mul i32 %spec.select.i65.i1592.7, %width.i30.i1557, !dbg !6735
  %_48.i67.i1594.7 = add i32 %_49.i66.i1593.7, 7, !dbg !6735
  %_51.i69.i1596.7 = icmp ult i32 %_48.i67.i1594.7, %_163.1.i79.i1606.pre, !dbg !6736
  br i1 %_51.i69.i1596.7, label %bb18.i71.i1598.7, label %panic1.i70.i1597, !dbg !6736

bb18.i71.i1598.7:                                 ; preds = %bb14.i61.i1588.7
  %542 = getelementptr inbounds nuw float, ptr %_161.0.i72.i1599, i32 %_48.i67.i1594.7, !dbg !6736
  %_47.i73.i1600.7 = load float, ptr %542, align 4, !dbg !6736, !noalias !6731, !noundef !10
  store float %_47.i73.i1600.7, ptr %iter.sroa.0.0.ptr.i52.i157918439.7, align 4, !dbg !6737, !alias.scope !6505, !noalias !6738
  br label %bb53.i74.i1601.loopexit, !dbg !6695

panic1.i70.i1597:                                 ; preds = %bb14.i61.i1588.7, %bb14.i61.i1588.6, %bb14.i61.i1588.5, %bb14.i61.i1588.4, %bb14.i61.i1588.3, %bb14.i61.i1588.2, %bb14.i61.i1588.1, %bb14.i61.i1588
  %_48.i67.i1594.lcssa.ph = phi i32 [ %_48.i67.i1594.7, %bb14.i61.i1588.7 ], [ %_48.i67.i1594.6, %bb14.i61.i1588.6 ], [ %_48.i67.i1594.5, %bb14.i61.i1588.5 ], [ %_48.i67.i1594.4, %bb14.i61.i1588.4 ], [ %_48.i67.i1594.3, %bb14.i61.i1588.3 ], [ %_48.i67.i1594.2, %bb14.i61.i1588.2 ], [ %_48.i67.i1594.1, %bb14.i61.i1588.1 ], [ %_49.i66.i1593, %bb14.i61.i1588 ]
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %421, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_48.i67.i1594.lcssa.ph, i32 noundef %_163.1.i79.i1606.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_0e76a26fbb751dfb8cf8a069b5b0d39a) #32, !dbg !6736, !noalias !6731
  unreachable, !dbg !6736

bb42.i81.i1608:                                   ; preds = %bb53.i74.i1601
  %_126.i83.i1610 = sub nuw i32 %_163.1.i79.i1606.pre, %_22.i36.i1563, !dbg !6739
  %_8.i6442 = icmp samesign ugt i32 %_126.i83.i1610, 3, !dbg !6740
  br i1 %_8.i6442, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6445, label %bb2.i6443, !dbg !6740, !prof !1039

bb2.i6443:                                        ; preds = %bb42.i81.i1608
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %503, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_126.i83.i1610, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !6745, !noalias !6746
  unreachable, !dbg !6745

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6445: ; preds = %bb42.i81.i1608
  %_163.0.i82.i1609 = load ptr, ptr %148, align 4, !dbg !6700, !alias.scope !6508, !noalias !6509, !nonnull !10, !noundef !10
  %_130.i84.i1611 = getelementptr inbounds nuw float, ptr %_163.0.i82.i1609, i32 %_22.i36.i1563, !dbg !6750
  store <4 x float> %500, ptr %_130.i84.i1611, align 4, !dbg !6755, !alias.scope !6759, !noalias !6763
  %_62.i86.i161316000 = load <4 x float>, ptr %150, align 16, !dbg !6765
  %_66.i89.i161616001 = load <4 x float>, ptr %151, align 16, !dbg !6766
  %543 = fdiv <4 x float> %503, %_62.i86.i161316000, !dbg !6769
  %544 = fsub <4 x float> splat (float 1.000000e+00), %543, !dbg !6773
  %545 = fsub <4 x float> %544, %_66.i89.i161616001, !dbg !6777
  %546 = bitcast <16 x i8> %_4.i7039 to <4 x float>, !dbg !6781
  %547 = fmul <4 x float> %545, %546, !dbg !6787
  %548 = fadd <4 x float> %_66.i89.i161616001, %547, !dbg !6788
  %549 = fcmp olt <4 x float> %548, %544, !dbg !6793
  %550 = select <4 x i1> %549, <4 x float> %544, <4 x float> %548, !dbg !6798
  %551 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %550), !dbg !6799
  %552 = fcmp uge <4 x float> %551, splat (float 0x3BC79CA100000000), !dbg !6806
  %553 = bitcast <4 x float> %550 to <4 x i32>, !dbg !6816
  %554 = select <4 x i1> %552, <4 x i32> %553, <4 x i32> zeroinitializer, !dbg !6816
  store <4 x i32> %554, ptr %151, align 16, !dbg !6822
  %555 = bitcast <4 x i32> %554 to <4 x float>, !dbg !6823
  %556 = fsub <4 x float> splat (float 1.000000e+00), %555, !dbg !6827
  %_164.1.i99.i1626 = load i32, ptr %152, align 4, !dbg !6828, !alias.scope !6508, !noalias !6509, !noundef !10
  %_74.i100.i1627 = mul i32 %width.i30.i1557, %main_cursor.sroa.0.1.i152418455, !dbg !6830
  %_134.i101.i1628 = icmp ugt i32 %_74.i100.i1627, %_164.1.i99.i1626, !dbg !6831
  br i1 %_134.i101.i1628, label %bb47.i120.i1754, label %bb48.i102.i1629, !dbg !6831, !prof !787

bb41.i121.i1755:                                  ; preds = %bb53.i74.i1601
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %503, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i36.i1563, i32 noundef %_163.1.i79.i1606.pre, i32 noundef %_163.1.i79.i1606.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c2286ff41a33b8c11aa270fe215441de) #32, !dbg !6836, !noalias !6731
  unreachable, !dbg !6836

bb48.i102.i1629:                                  ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6445
  %_137.i104.i1631 = sub nuw i32 %_164.1.i99.i1626, %_74.i100.i1627, !dbg !6837
  %_8.i5679 = icmp samesign ugt i32 %_137.i104.i1631, 3, !dbg !6838
  br i1 %_8.i5679, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6435, label %bb2.i5680, !dbg !6838, !prof !1039

bb2.i5680:                                        ; preds = %bb48.i102.i1629
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %503, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_137.i104.i1631, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !6843, !noalias !6844
  unreachable, !dbg !6843

bb47.i120.i1754:                                  ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6445
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %503, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_74.i100.i1627, i32 noundef %_164.1.i99.i1626, i32 noundef %_164.1.i99.i1626, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a3a21efd18b403ec25db545d522c479) #32, !dbg !6848, !noalias !6731
  unreachable, !dbg !6848

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6435: ; preds = %bb48.i102.i1629
  %_164.0.i103.i1630 = load ptr, ptr %153, align 4, !dbg !6828, !alias.scope !6508, !noalias !6509, !nonnull !10, !noundef !10
  %_141.i105.i1632 = getelementptr inbounds nuw float, ptr %_164.0.i103.i1630, i32 %_74.i100.i1627, !dbg !6849
  %lanes.i5676.sroa.0.0.copyload = load <4 x i32>, ptr %_141.i105.i1632, align 4, !dbg !6854, !alias.scope !6858, !noalias !6862
  store <4 x i32> %lanes.i5699.sroa.0.0.copyload, ptr %_141.i105.i1632, align 4, !dbg !6864, !alias.scope !6870, !noalias !6874
  %557 = bitcast <4 x i32> %lanes.i5676.sroa.0.0.copyload to <4 x float>, !dbg !6878
  %558 = fmul <4 x float> %556, %557, !dbg !6882
  %559 = bitcast <4 x i32> %lanes.i5676.sroa.0.0.copyload to <16 x i8>, !dbg !6883
  %560 = bitcast <4 x float> %558 to <16 x i8>, !dbg !6887
  %_4.i7057 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %559, <16 x i8> %560, <16 x i8> %154), !dbg !6888
  store <16 x i8> %_4.i7057, ptr %_171.i1553, align 4, !dbg !6889, !alias.scope !6894, !noalias !6898
  %_172.i1646 = icmp ugt i32 %base.i1528, %right_io.1, !dbg !6902
  br i1 %_172.i1646, label %bb56.i1751, label %bb57.i1647, !dbg !6902, !prof !787

bb54.i1757:                                       ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5724
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %421, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1528, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_adfae95437a74b76017017a3a282b9f2) #32, !dbg !6906, !noalias !5208
  unreachable, !dbg !6906

bb57.i1647:                                       ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6435
  %_174.i1648 = sub nuw nsw i32 %right_io.1, %base.i1528, !dbg !6907
  %_178.i1649 = getelementptr inbounds nuw float, ptr %right_io.0, i32 %base.i1528, !dbg !6908
  %_8.i5670 = icmp samesign ugt i32 %_174.i1648, 3, !dbg !6913
  br i1 %_8.i5670, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5674, label %bb2.i5671, !dbg !6913, !prof !1039

bb2.i5671:                                        ; preds = %bb57.i1647
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %503, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_174.i1648, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !6918, !noalias !6919
  unreachable, !dbg !6918

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5674: ; preds = %bb57.i1647
  %lanes.i5667.sroa.0.0.copyload = load <4 x i32>, ptr %_178.i1649, align 4, !dbg !6923, !alias.scope !6927, !noalias !6931
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6933), !dbg !6936
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6937), !dbg !6936
  %width.i.i1652 = load i32, ptr %155, align 4, !dbg !6939, !alias.scope !6940, !noalias !6941, !noundef !10
  %561 = bitcast <16 x i8> %_4.i7048 to <4 x float>, !dbg !6949
  %562 = bitcast <16 x i8> %_4.i7041 to <4 x float>, !dbg !6954
  %563 = fcmp ogt <4 x float> %561, %562, !dbg !6955
  %564 = sext <4 x i1> %563 to <4 x i32>, !dbg !6955
  %565 = fdiv <4 x float> %562, %561, !dbg !6956
  %566 = bitcast <4 x float> %565 to <16 x i8>, !dbg !6960
  %567 = bitcast <4 x i32> %564 to <16 x i8>, !dbg !6964
  %_4.i7060 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %566, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %567), !dbg !6965
  %_158.1.i.i1657 = load i32, ptr %156, align 4, !dbg !6966, !alias.scope !6940, !noalias !6941, !noundef !10
  %_22.i.i1658 = mul i32 %width.i.i1652, %ring_cursor.sroa.0.1.i152318454, !dbg !6967
  %_90.i.i1659 = icmp ugt i32 %_22.i.i1658, %_158.1.i.i1657, !dbg !6968
  br i1 %_90.i.i1659, label %bb34.i.i1750, label %bb35.i.i1660, !dbg !6968, !prof !787

bb35.i.i1660:                                     ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5674
  %_93.i.i1662 = sub nuw i32 %_158.1.i.i1657, %_22.i.i1658, !dbg !6971
  %_8.i6427 = icmp samesign ugt i32 %_93.i.i1662, 3, !dbg !6972
  br i1 %_8.i6427, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6430, label %bb2.i6428, !dbg !6972, !prof !1039

bb2.i6428:                                        ; preds = %bb35.i.i1660
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %503, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_93.i.i1662, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !6977, !noalias !6978
  unreachable, !dbg !6977

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6430: ; preds = %bb35.i.i1660
  %_158.0.i.i1661 = load ptr, ptr %157, align 4, !dbg !6966, !alias.scope !6940, !noalias !6941, !nonnull !10, !noundef !10
  %_97.i.i1663 = getelementptr inbounds nuw float, ptr %_158.0.i.i1661, i32 %_22.i.i1658, !dbg !6982
  store <16 x i8> %_4.i7060, ptr %_97.i.i1663, align 4, !dbg !6984, !alias.scope !6988, !noalias !6992
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6994), !dbg !6997
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6998), !dbg !6997
  %width.i = load i32, ptr %155, align 4, !dbg !7000, !alias.scope !6994, !noalias !7002, !noundef !10
  %568 = icmp eq i32 %width.i, 0, !dbg !7003
  br i1 %568, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit, label %bb29.i.lr.ph, !dbg !7003

bb29.i.lr.ph:                                     ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6430
  %_126.1.i = load i32, ptr %158, align 4, !alias.scope !6994, !noalias !7002, !noundef !10
  %_126.0.i = load ptr, ptr %159, align 4, !nonnull !10
  %569 = add i32 %ring_cursor.sroa.0.1.i152318454, 1
  %_21.not.i = icmp ult i32 %569, %_91.i1555
  %570 = select i1 %_21.not.i, i32 0, i32 %_91.i1555
  %start1.sroa.0.0.i = sub nuw i32 %569, %570
  %_128.1.i = load i32, ptr %156, align 4
  %_128.0.i = load ptr, ptr %157, align 4, !nonnull !10
  %_130.1.i = load i32, ptr %160, align 4
  %_130.0.i = load ptr, ptr %161, align 4, !nonnull !10
  %_132.1.i = load i32, ptr %162, align 4
  %_132.0.i = load ptr, ptr %163, align 4, !nonnull !10
  %_43.i = mul i32 %width.i, %start1.sroa.0.0.i
  br label %bb29.i, !dbg !7003

bb29.i:                                           ; preds = %bb29.i.lr.ph, %bb28.i
  %iter.sroa.0.0.idx.i18446 = phi i32 [ 0, %bb29.i.lr.ph ], [ %iter.sroa.0.0.add.i, %bb28.i ]
  %iter.sroa.4.0.i18445 = phi i32 [ 0, %bb29.i.lr.ph ], [ %_102.0.i, %bb28.i ]
  %iter.sroa.7.0.i18444 = phi i32 [ %width.i, %bb29.i.lr.ph ], [ %571, %bb28.i ]
  %iter.sroa.0.0.ptr.i18447 = getelementptr inbounds nuw i8, ptr %scratch.i1088, i32 %iter.sroa.0.0.idx.i18446, !dbg !7005
  %571 = add i32 %iter.sroa.7.0.i18444, -1, !dbg !7005
  %_109.i = icmp eq i32 %iter.sroa.0.0.idx.i18446, 32, !dbg !7006
  br i1 %_109.i, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit, label %bb33.i, !dbg !7010

bb33.i:                                           ; preds = %bb29.i
  %iter.sroa.0.0.add.i = add nuw nsw i32 %iter.sroa.0.0.idx.i18446, 4, !dbg !7011
  %_102.0.i = add nuw nsw i32 %iter.sroa.4.0.i18445, 1, !dbg !7013
  %exitcond20910.not = icmp eq i32 %iter.sroa.4.0.i18445, %_126.1.i, !dbg !7014
  br i1 %exitcond20910.not, label %panic.i, label %bb2.i2096, !dbg !7014

bb2.i2096:                                        ; preds = %bb33.i
  %572 = getelementptr inbounds nuw %LaneShape, ptr %_126.0.i, i32 %iter.sroa.4.0.i18445, !dbg !7014
  %shape.i = load i32, ptr %572, align 4, !dbg !7014, !noalias !7015, !noundef !10
  %573 = getelementptr inbounds nuw i8, ptr %572, i32 4, !dbg !7014
  %shape3.i = load i32, ptr %573, align 4, !dbg !7014, !noalias !7015, !noundef !10
  %574 = add i32 %shape3.i, %ring_cursor.sroa.0.1.i152318454, !dbg !7016
  %_18.not.i = icmp ult i32 %574, %_91.i1555, !dbg !7017
  %575 = select i1 %_18.not.i, i32 0, i32 %_91.i1555, !dbg !7017
  %spec.select.i = sub nuw i32 %574, %575, !dbg !7017
  %_25.i = mul i32 %spec.select.i, %width.i, !dbg !7018
  %_24.i = add i32 %_25.i, %iter.sroa.4.0.i18445, !dbg !7018
  %_28.i2097 = icmp ult i32 %_24.i, %_128.1.i, !dbg !7019
  br i1 %_28.i2097, label %bb9.i, label %panic5.i, !dbg !7019

panic.i:                                          ; preds = %bb33.i
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %503, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_126.1.i, i32 noundef %_126.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_12856b5f9033764dbf23e04eb77c5c46) #32, !dbg !7014, !noalias !7015
  unreachable, !dbg !7014

bb9.i:                                            ; preds = %bb2.i2096
  %576 = getelementptr inbounds nuw float, ptr %_128.0.i, i32 %_24.i, !dbg !7019
  %577 = load float, ptr %576, align 4, !dbg !7019, !noalias !7015, !noundef !10
  %exitcond20911.not = icmp eq i32 %iter.sroa.4.0.i18445, %_130.1.i, !dbg !7020
  br i1 %exitcond20911.not, label %panic6.i, label %bb10.i2099, !dbg !7020

panic5.i:                                         ; preds = %bb2.i2096
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %503, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_24.i, i32 noundef %_128.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70ebf0cf1c90c6161926611ccf249a32) #32, !dbg !7019, !noalias !7015
  unreachable, !dbg !7019

bb10.i2099:                                       ; preds = %bb9.i
  %578 = getelementptr inbounds nuw i32, ptr %_130.0.i, i32 %iter.sroa.4.0.i18445, !dbg !7020
  %_30.i2100 = load i32, ptr %578, align 4, !dbg !7020, !noalias !7015, !noundef !10
  %579 = icmp eq i32 %_30.i2100, 0, !dbg !7021
  br i1 %579, label %bb14.i, label %bb12.i2101, !dbg !7021

panic6.i:                                         ; preds = %bb9.i
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %503, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_130.1.i, i32 noundef %_130.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_023d591aaad7f5cf482ac80a9401eca1) #32, !dbg !7020, !noalias !7015
  unreachable, !dbg !7020

bb12.i2101:                                       ; preds = %bb10.i2099
  %_35.i2102 = icmp ult i32 %iter.sroa.4.0.i18445, %_132.1.i, !dbg !7022
  br i1 %_35.i2102, label %bb13.i2103, label %panic7.i, !dbg !7022

bb14.i:                                           ; preds = %bb34.i, %bb13.i2103, %bb10.i2099
  %newest.sroa.0.0.i = phi float [ %577, %bb10.i2099 ], [ %_33.i2104, %bb34.i ], [ %577, %bb13.i2103 ], !dbg !7023
  %exitcond20912.not = icmp eq i32 %iter.sroa.4.0.i18445, %_132.1.i, !dbg !7024
  br i1 %exitcond20912.not, label %panic8.i, label %bb15.i2105, !dbg !7024

bb13.i2103:                                       ; preds = %bb12.i2101
  %580 = getelementptr inbounds nuw float, ptr %_132.0.i, i32 %iter.sroa.4.0.i18445, !dbg !7022
  %_33.i2104 = load float, ptr %580, align 4, !dbg !7022, !noalias !7015, !noundef !10
  %_116.i = fcmp olt float %_33.i2104, %577, !dbg !7025
  br i1 %_116.i, label %bb34.i, label %bb14.i, !dbg !7025

panic7.i:                                         ; preds = %bb12.i2101
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %503, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %iter.sroa.4.0.i18445, i32 noundef %_132.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a228cac3279aae3a34886f59f51fbe9e) #32, !dbg !7022, !noalias !7015
  unreachable, !dbg !7022

bb34.i:                                           ; preds = %bb13.i2103
  br label %bb14.i, !dbg !7027

bb15.i2105:                                       ; preds = %bb14.i
  %581 = getelementptr inbounds nuw float, ptr %_132.0.i, i32 %iter.sroa.4.0.i18445, !dbg !7024
  store float %newest.sroa.0.0.i, ptr %581, align 4, !dbg !7024, !noalias !7015
  %_40.i = add i32 %_30.i2100, 1, !dbg !7028
  %complete.i2106 = icmp eq i32 %_40.i, %shape.i, !dbg !7028
  br i1 %complete.i2106, label %bb19.i2110, label %bb17.i, !dbg !7029

panic8.i:                                         ; preds = %bb14.i
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %503, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_132.1.i, i32 noundef %_132.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1fa5d88c1a2cc292a2767c48606a38fe) #32, !dbg !7024, !noalias !7015
  unreachable, !dbg !7024

bb17.i:                                           ; preds = %bb15.i2105
  %_42.i = add i32 %iter.sroa.4.0.i18445, %_43.i, !dbg !7030
  %_45.i = icmp ult i32 %_42.i, %_128.1.i, !dbg !7031
  br i1 %_45.i, label %bb27.i, label %panic9.i, !dbg !7031

panic9.i:                                         ; preds = %bb17.i
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %503, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_42.i, i32 noundef %_128.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70d40ae2302f3ea19fa0c4a65fe82786) #32, !dbg !7031, !noalias !7015
  unreachable, !dbg !7031

bb27.i:                                           ; preds = %bb17.i
  %582 = getelementptr inbounds nuw float, ptr %_128.0.i, i32 %_42.i, !dbg !7031
  %_41.i = load float, ptr %582, align 4, !dbg !7031, !noalias !7015, !noundef !10
  %_117.i = fcmp olt float %_41.i, %newest.sroa.0.0.i, !dbg !7032
  %newest.sroa.0.1.i = select i1 %_117.i, float %_41.i, float %newest.sroa.0.0.i, !dbg !7032
  store float %newest.sroa.0.1.i, ptr %iter.sroa.0.0.ptr.i18447, align 4, !dbg !7034, !alias.scope !6998, !noalias !7035
  br label %bb28.i, !dbg !7036

bb28.i:                                           ; preds = %bb22.i, %bb19.i2110, %bb27.i
  %storemerge16003 = phi i32 [ %_40.i, %bb27.i ], [ 0, %bb19.i2110 ], [ 0, %bb22.i ], !dbg !7037
  store i32 %storemerge16003, ptr %578, align 4, !dbg !7037, !noalias !7015
  %583 = icmp eq i32 %571, 0, !dbg !7003
  br i1 %583, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit, label %bb29.i, !dbg !7003

bb19.i2110:                                       ; preds = %bb15.i2105
  store float %newest.sroa.0.0.i, ptr %iter.sroa.0.0.ptr.i18447, align 4, !dbg !7034, !alias.scope !6998, !noalias !7035
  %_118.i211318440.not = icmp eq i32 %shape.i, 0, !dbg !7038
  br i1 %_118.i211318440.not, label %bb28.i, label %bb40.i.preheader, !dbg !7042

bb40.i.preheader:                                 ; preds = %bb19.i2110
  %584 = load float, ptr %576, align 4, !dbg !7043, !noalias !7015, !noundef !10
  br label %bb40.i, !dbg !7044

bb40.i:                                           ; preds = %bb40.i.preheader, %bb22.i
  %iter2.sroa.0.0.i211218443 = phi i32 [ %_119.i2115, %bb22.i ], [ 0, %bb40.i.preheader ]
  %suffix.sroa.0.0.i18442 = phi float [ %suffix.sroa.0.1.i, %bb22.i ], [ %584, %bb40.i.preheader ]
  %end.sroa.0.1.i18441 = phi i32 [ %587, %bb22.i ], [ %spec.select.i, %bb40.i.preheader ]
  %_54.i = mul i32 %end.sroa.0.1.i18441, %width.i, !dbg !7045
  %_53.i = add i32 %_54.i, %iter.sroa.4.0.i18445, !dbg !7045
  %_57.i = icmp ult i32 %_53.i, %_128.1.i, !dbg !7044
  br i1 %_57.i, label %bb22.i, label %panic13.i, !dbg !7044

panic13.i:                                        ; preds = %bb40.i
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %503, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_53.i, i32 noundef %_128.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a271bbb7fd83c3605ea58a06b7065fa4) #32, !dbg !7044, !noalias !7015
  unreachable, !dbg !7044

bb22.i:                                           ; preds = %bb40.i
  %_119.i2115 = add nuw i32 %iter2.sroa.0.0.i211218443, 1, !dbg !7046
  %585 = getelementptr inbounds nuw float, ptr %_128.0.i, i32 %_53.i, !dbg !7044
  %_52.i2116 = load float, ptr %585, align 4, !dbg !7044, !noalias !7015, !noundef !10
  %_121.i = fcmp olt float %suffix.sroa.0.0.i18442, %_52.i2116, !dbg !7049
  %suffix.sroa.0.1.i = select i1 %_121.i, float %suffix.sroa.0.0.i18442, float %_52.i2116, !dbg !7049
  store float %suffix.sroa.0.1.i, ptr %585, align 4, !dbg !7051, !noalias !7015
  %586 = icmp eq i32 %end.sroa.0.1.i18441, 0, !dbg !7052
  %spec.store.select.i2118 = select i1 %586, i32 %_91.i1555, i32 %end.sroa.0.1.i18441, !dbg !7052
  %587 = add i32 %spec.store.select.i2118, -1, !dbg !7053
  %exitcond20909.not = icmp eq i32 %_119.i2115, %shape.i, !dbg !7038
  br i1 %exitcond20909.not, label %bb28.i, label %bb40.i, !dbg !7042

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit: ; preds = %bb28.i, %bb29.i
  %lanes.i5660.sroa.0.0.copyload.pre = load <4 x float>, ptr %scratch.i1088, align 4, !dbg !7054, !alias.scope !7059, !noalias !7063
  br label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit, !dbg !7067

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit: ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6430
  %lanes.i5660.sroa.0.0.copyload = phi <4 x float> [ %lanes.i5660.sroa.0.0.copyload.pre, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit ], [ %lanes.i5685.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6430 ], !dbg !7054
  %588 = fmul <4 x float> %lanes.i5660.sroa.0.0.copyload, splat (float 1.638400e+04), !dbg !7068
  %589 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %588), !dbg !7072
  %590 = fmul <4 x float> %589, splat (float 0x3F10000000000000), !dbg !7076
  %591 = icmp eq i32 %width.i.i1652, 0, !dbg !7080
  %_163.1.i.i1701.pre = load i32, ptr %164, align 4, !dbg !7082, !alias.scope !6940, !noalias !6941
  br i1 %591, label %bb53.i.i1696, label %bb36.i.i1675.lr.ph, !dbg !7080

bb36.i.i1675.lr.ph:                               ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit
  %_159.1.i.i1680 = load i32, ptr %158, align 4, !alias.scope !6940, !noalias !6941, !noundef !10
  %_159.0.i.i1684 = load ptr, ptr %159, align 4, !nonnull !10
  %_161.0.i.i1694 = load ptr, ptr %165, align 4, !nonnull !10
  %exitcond20915.not = icmp eq i32 %_159.1.i.i1680, 0, !dbg !7083
  br i1 %exitcond20915.not, label %panic.i.i1682, label %bb14.i.i1683, !dbg !7083

bb34.i.i1750:                                     ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5674
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %503, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i1658, i32 noundef %_158.1.i.i1657, i32 noundef %_158.1.i.i1657, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_db7c42041d7aaaba4839906f37294547) #32, !dbg !7084, !noalias !7085
  unreachable, !dbg !7084

bb53.i.i1696.loopexit:                            ; preds = %bb18.i.i1693.7, %bb18.i.i1693.6, %bb18.i.i1693.5, %bb18.i.i1693.4, %bb18.i.i1693.3, %bb18.i.i1693.2, %bb18.i.i1693.1, %bb18.i.i1693
  %lanes.i5653.sroa.0.0.copyload.pre = load <4 x float>, ptr %scratch.i1088, align 4, !dbg !7086, !alias.scope !7091, !noalias !7095
  br label %bb53.i.i1696, !dbg !7099

bb53.i.i1696:                                     ; preds = %bb53.i.i1696.loopexit, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit
  %lanes.i5653.sroa.0.0.copyload = phi <4 x float> [ %lanes.i5653.sroa.0.0.copyload.pre, %bb53.i.i1696.loopexit ], [ %lanes.i5660.sroa.0.0.copyload, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit ], !dbg !7086
  %592 = fadd <4 x float> %590, %420, !dbg !7100
  %593 = fsub <4 x float> %592, %lanes.i5653.sroa.0.0.copyload, !dbg !7104
  %_123.i.i1702 = icmp ugt i32 %_22.i.i1658, %_163.1.i.i1701.pre, !dbg !7108
  br i1 %_123.i.i1702, label %bb41.i.i1749, label %bb42.i.i1703, !dbg !7108, !prof !787

bb14.i.i1683:                                     ; preds = %bb36.i.i1675.lr.ph
  %594 = getelementptr inbounds nuw i8, ptr %_159.0.i.i1684, i32 8, !dbg !7083
  %_42.i.i1685 = load i32, ptr %594, align 4, !dbg !7083, !noalias !7111, !noundef !10
  %595 = add i32 %_42.i.i1685, %ring_cursor.sroa.0.1.i152318454, !dbg !7112
  %_45.not.i.i1686 = icmp ult i32 %595, %_91.i1555, !dbg !7113
  %596 = select i1 %_45.not.i.i1686, i32 0, i32 %_91.i1555, !dbg !7113
  %spec.select.i.i1687 = sub nuw i32 %595, %596, !dbg !7113
  %_49.i.i1688 = mul i32 %spec.select.i.i1687, %width.i.i1652, !dbg !7114
  %_51.i.i1691 = icmp ult i32 %_49.i.i1688, %_163.1.i.i1701.pre, !dbg !7115
  br i1 %_51.i.i1691, label %bb18.i.i1693, label %panic1.i.i1692, !dbg !7115

panic.i.i1682:                                    ; preds = %bb36.i.i1675.7, %bb36.i.i1675.6, %bb36.i.i1675.5, %bb36.i.i1675.4, %bb36.i.i1675.3, %bb36.i.i1675.2, %bb36.i.i1675.1, %bb36.i.i1675.lr.ph
  %_159.1.i.i1680.lcssa.ph = phi i32 [ 7, %bb36.i.i1675.7 ], [ 6, %bb36.i.i1675.6 ], [ 5, %bb36.i.i1675.5 ], [ 4, %bb36.i.i1675.4 ], [ 3, %bb36.i.i1675.3 ], [ 2, %bb36.i.i1675.2 ], [ 1, %bb36.i.i1675.1 ], [ 0, %bb36.i.i1675.lr.ph ]
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %503, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_159.1.i.i1680.lcssa.ph, i32 noundef %_159.1.i.i1680.lcssa.ph, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_33746731a929bad63709ddedbca82f5f) #32, !dbg !7083, !noalias !7111
  unreachable, !dbg !7083

bb18.i.i1693:                                     ; preds = %bb14.i.i1683
  %597 = getelementptr inbounds nuw float, ptr %_161.0.i.i1694, i32 %_49.i.i1688, !dbg !7115
  %_47.i.i1695 = load float, ptr %597, align 4, !dbg !7115, !noalias !7111, !noundef !10
  store float %_47.i.i1695, ptr %scratch.i1088, align 4, !dbg !7116, !alias.scope !6937, !noalias !7117
  %598 = icmp eq i32 %width.i.i1652, 1, !dbg !7080
  br i1 %598, label %bb53.i.i1696.loopexit, label %bb36.i.i1675.1, !dbg !7080

bb36.i.i1675.1:                                   ; preds = %bb18.i.i1693
  %exitcond20915.1.not = icmp eq i32 %_159.1.i.i1680, 1, !dbg !7083
  br i1 %exitcond20915.1.not, label %panic.i.i1682, label %bb14.i.i1683.1, !dbg !7083

bb14.i.i1683.1:                                   ; preds = %bb36.i.i1675.1
  %599 = getelementptr inbounds nuw i8, ptr %_159.0.i.i1684, i32 20, !dbg !7083
  %_42.i.i1685.1 = load i32, ptr %599, align 4, !dbg !7083, !noalias !7111, !noundef !10
  %600 = add i32 %_42.i.i1685.1, %ring_cursor.sroa.0.1.i152318454, !dbg !7112
  %_45.not.i.i1686.1 = icmp ult i32 %600, %_91.i1555, !dbg !7113
  %601 = select i1 %_45.not.i.i1686.1, i32 0, i32 %_91.i1555, !dbg !7113
  %spec.select.i.i1687.1 = sub nuw i32 %600, %601, !dbg !7113
  %_49.i.i1688.1 = mul i32 %spec.select.i.i1687.1, %width.i.i1652, !dbg !7114
  %_48.i.i1689.1 = add i32 %_49.i.i1688.1, 1, !dbg !7114
  %_51.i.i1691.1 = icmp ult i32 %_48.i.i1689.1, %_163.1.i.i1701.pre, !dbg !7115
  br i1 %_51.i.i1691.1, label %bb18.i.i1693.1, label %panic1.i.i1692, !dbg !7115

bb18.i.i1693.1:                                   ; preds = %bb14.i.i1683.1
  %602 = getelementptr inbounds nuw float, ptr %_161.0.i.i1694, i32 %_48.i.i1689.1, !dbg !7115
  %_47.i.i1695.1 = load float, ptr %602, align 4, !dbg !7115, !noalias !7111, !noundef !10
  store float %_47.i.i1695.1, ptr %iter.sroa.0.0.ptr.i.i167418451.1, align 4, !dbg !7116, !alias.scope !6937, !noalias !7117
  %603 = icmp eq i32 %width.i.i1652, 2, !dbg !7080
  br i1 %603, label %bb53.i.i1696.loopexit, label %bb36.i.i1675.2, !dbg !7080

bb36.i.i1675.2:                                   ; preds = %bb18.i.i1693.1
  %exitcond20915.2.not = icmp eq i32 %_159.1.i.i1680, 2, !dbg !7083
  br i1 %exitcond20915.2.not, label %panic.i.i1682, label %bb14.i.i1683.2, !dbg !7083

bb14.i.i1683.2:                                   ; preds = %bb36.i.i1675.2
  %604 = getelementptr inbounds nuw i8, ptr %_159.0.i.i1684, i32 32, !dbg !7083
  %_42.i.i1685.2 = load i32, ptr %604, align 4, !dbg !7083, !noalias !7111, !noundef !10
  %605 = add i32 %_42.i.i1685.2, %ring_cursor.sroa.0.1.i152318454, !dbg !7112
  %_45.not.i.i1686.2 = icmp ult i32 %605, %_91.i1555, !dbg !7113
  %606 = select i1 %_45.not.i.i1686.2, i32 0, i32 %_91.i1555, !dbg !7113
  %spec.select.i.i1687.2 = sub nuw i32 %605, %606, !dbg !7113
  %_49.i.i1688.2 = mul i32 %spec.select.i.i1687.2, %width.i.i1652, !dbg !7114
  %_48.i.i1689.2 = add i32 %_49.i.i1688.2, 2, !dbg !7114
  %_51.i.i1691.2 = icmp ult i32 %_48.i.i1689.2, %_163.1.i.i1701.pre, !dbg !7115
  br i1 %_51.i.i1691.2, label %bb18.i.i1693.2, label %panic1.i.i1692, !dbg !7115

bb18.i.i1693.2:                                   ; preds = %bb14.i.i1683.2
  %607 = getelementptr inbounds nuw float, ptr %_161.0.i.i1694, i32 %_48.i.i1689.2, !dbg !7115
  %_47.i.i1695.2 = load float, ptr %607, align 4, !dbg !7115, !noalias !7111, !noundef !10
  store float %_47.i.i1695.2, ptr %iter.sroa.0.0.ptr.i.i167418451.2, align 4, !dbg !7116, !alias.scope !6937, !noalias !7117
  %608 = icmp eq i32 %width.i.i1652, 3, !dbg !7080
  br i1 %608, label %bb53.i.i1696.loopexit, label %bb36.i.i1675.3, !dbg !7080

bb36.i.i1675.3:                                   ; preds = %bb18.i.i1693.2
  %exitcond20915.3.not = icmp eq i32 %_159.1.i.i1680, 3, !dbg !7083
  br i1 %exitcond20915.3.not, label %panic.i.i1682, label %bb14.i.i1683.3, !dbg !7083

bb14.i.i1683.3:                                   ; preds = %bb36.i.i1675.3
  %609 = getelementptr inbounds nuw i8, ptr %_159.0.i.i1684, i32 44, !dbg !7083
  %_42.i.i1685.3 = load i32, ptr %609, align 4, !dbg !7083, !noalias !7111, !noundef !10
  %610 = add i32 %_42.i.i1685.3, %ring_cursor.sroa.0.1.i152318454, !dbg !7112
  %_45.not.i.i1686.3 = icmp ult i32 %610, %_91.i1555, !dbg !7113
  %611 = select i1 %_45.not.i.i1686.3, i32 0, i32 %_91.i1555, !dbg !7113
  %spec.select.i.i1687.3 = sub nuw i32 %610, %611, !dbg !7113
  %_49.i.i1688.3 = mul i32 %spec.select.i.i1687.3, %width.i.i1652, !dbg !7114
  %_48.i.i1689.3 = add i32 %_49.i.i1688.3, 3, !dbg !7114
  %_51.i.i1691.3 = icmp ult i32 %_48.i.i1689.3, %_163.1.i.i1701.pre, !dbg !7115
  br i1 %_51.i.i1691.3, label %bb18.i.i1693.3, label %panic1.i.i1692, !dbg !7115

bb18.i.i1693.3:                                   ; preds = %bb14.i.i1683.3
  %612 = getelementptr inbounds nuw float, ptr %_161.0.i.i1694, i32 %_48.i.i1689.3, !dbg !7115
  %_47.i.i1695.3 = load float, ptr %612, align 4, !dbg !7115, !noalias !7111, !noundef !10
  store float %_47.i.i1695.3, ptr %iter.sroa.0.0.ptr.i.i167418451.3, align 4, !dbg !7116, !alias.scope !6937, !noalias !7117
  %613 = icmp eq i32 %width.i.i1652, 4, !dbg !7080
  br i1 %613, label %bb53.i.i1696.loopexit, label %bb36.i.i1675.4, !dbg !7080

bb36.i.i1675.4:                                   ; preds = %bb18.i.i1693.3
  %exitcond20915.4.not = icmp eq i32 %_159.1.i.i1680, 4, !dbg !7083
  br i1 %exitcond20915.4.not, label %panic.i.i1682, label %bb14.i.i1683.4, !dbg !7083

bb14.i.i1683.4:                                   ; preds = %bb36.i.i1675.4
  %614 = getelementptr inbounds nuw i8, ptr %_159.0.i.i1684, i32 56, !dbg !7083
  %_42.i.i1685.4 = load i32, ptr %614, align 4, !dbg !7083, !noalias !7111, !noundef !10
  %615 = add i32 %_42.i.i1685.4, %ring_cursor.sroa.0.1.i152318454, !dbg !7112
  %_45.not.i.i1686.4 = icmp ult i32 %615, %_91.i1555, !dbg !7113
  %616 = select i1 %_45.not.i.i1686.4, i32 0, i32 %_91.i1555, !dbg !7113
  %spec.select.i.i1687.4 = sub nuw i32 %615, %616, !dbg !7113
  %_49.i.i1688.4 = mul i32 %spec.select.i.i1687.4, %width.i.i1652, !dbg !7114
  %_48.i.i1689.4 = add i32 %_49.i.i1688.4, 4, !dbg !7114
  %_51.i.i1691.4 = icmp ult i32 %_48.i.i1689.4, %_163.1.i.i1701.pre, !dbg !7115
  br i1 %_51.i.i1691.4, label %bb18.i.i1693.4, label %panic1.i.i1692, !dbg !7115

bb18.i.i1693.4:                                   ; preds = %bb14.i.i1683.4
  %617 = getelementptr inbounds nuw float, ptr %_161.0.i.i1694, i32 %_48.i.i1689.4, !dbg !7115
  %_47.i.i1695.4 = load float, ptr %617, align 4, !dbg !7115, !noalias !7111, !noundef !10
  store float %_47.i.i1695.4, ptr %iter.sroa.0.0.ptr.i.i167418451.4, align 4, !dbg !7116, !alias.scope !6937, !noalias !7117
  %618 = icmp eq i32 %width.i.i1652, 5, !dbg !7080
  br i1 %618, label %bb53.i.i1696.loopexit, label %bb36.i.i1675.5, !dbg !7080

bb36.i.i1675.5:                                   ; preds = %bb18.i.i1693.4
  %exitcond20915.5.not = icmp eq i32 %_159.1.i.i1680, 5, !dbg !7083
  br i1 %exitcond20915.5.not, label %panic.i.i1682, label %bb14.i.i1683.5, !dbg !7083

bb14.i.i1683.5:                                   ; preds = %bb36.i.i1675.5
  %619 = getelementptr inbounds nuw i8, ptr %_159.0.i.i1684, i32 68, !dbg !7083
  %_42.i.i1685.5 = load i32, ptr %619, align 4, !dbg !7083, !noalias !7111, !noundef !10
  %620 = add i32 %_42.i.i1685.5, %ring_cursor.sroa.0.1.i152318454, !dbg !7112
  %_45.not.i.i1686.5 = icmp ult i32 %620, %_91.i1555, !dbg !7113
  %621 = select i1 %_45.not.i.i1686.5, i32 0, i32 %_91.i1555, !dbg !7113
  %spec.select.i.i1687.5 = sub nuw i32 %620, %621, !dbg !7113
  %_49.i.i1688.5 = mul i32 %spec.select.i.i1687.5, %width.i.i1652, !dbg !7114
  %_48.i.i1689.5 = add i32 %_49.i.i1688.5, 5, !dbg !7114
  %_51.i.i1691.5 = icmp ult i32 %_48.i.i1689.5, %_163.1.i.i1701.pre, !dbg !7115
  br i1 %_51.i.i1691.5, label %bb18.i.i1693.5, label %panic1.i.i1692, !dbg !7115

bb18.i.i1693.5:                                   ; preds = %bb14.i.i1683.5
  %622 = getelementptr inbounds nuw float, ptr %_161.0.i.i1694, i32 %_48.i.i1689.5, !dbg !7115
  %_47.i.i1695.5 = load float, ptr %622, align 4, !dbg !7115, !noalias !7111, !noundef !10
  store float %_47.i.i1695.5, ptr %iter.sroa.0.0.ptr.i.i167418451.5, align 4, !dbg !7116, !alias.scope !6937, !noalias !7117
  %623 = icmp eq i32 %width.i.i1652, 6, !dbg !7080
  br i1 %623, label %bb53.i.i1696.loopexit, label %bb36.i.i1675.6, !dbg !7080

bb36.i.i1675.6:                                   ; preds = %bb18.i.i1693.5
  %exitcond20915.6.not = icmp eq i32 %_159.1.i.i1680, 6, !dbg !7083
  br i1 %exitcond20915.6.not, label %panic.i.i1682, label %bb14.i.i1683.6, !dbg !7083

bb14.i.i1683.6:                                   ; preds = %bb36.i.i1675.6
  %624 = getelementptr inbounds nuw i8, ptr %_159.0.i.i1684, i32 80, !dbg !7083
  %_42.i.i1685.6 = load i32, ptr %624, align 4, !dbg !7083, !noalias !7111, !noundef !10
  %625 = add i32 %_42.i.i1685.6, %ring_cursor.sroa.0.1.i152318454, !dbg !7112
  %_45.not.i.i1686.6 = icmp ult i32 %625, %_91.i1555, !dbg !7113
  %626 = select i1 %_45.not.i.i1686.6, i32 0, i32 %_91.i1555, !dbg !7113
  %spec.select.i.i1687.6 = sub nuw i32 %625, %626, !dbg !7113
  %_49.i.i1688.6 = mul i32 %spec.select.i.i1687.6, %width.i.i1652, !dbg !7114
  %_48.i.i1689.6 = add i32 %_49.i.i1688.6, 6, !dbg !7114
  %_51.i.i1691.6 = icmp ult i32 %_48.i.i1689.6, %_163.1.i.i1701.pre, !dbg !7115
  br i1 %_51.i.i1691.6, label %bb18.i.i1693.6, label %panic1.i.i1692, !dbg !7115

bb18.i.i1693.6:                                   ; preds = %bb14.i.i1683.6
  %627 = getelementptr inbounds nuw float, ptr %_161.0.i.i1694, i32 %_48.i.i1689.6, !dbg !7115
  %_47.i.i1695.6 = load float, ptr %627, align 4, !dbg !7115, !noalias !7111, !noundef !10
  store float %_47.i.i1695.6, ptr %iter.sroa.0.0.ptr.i.i167418451.6, align 4, !dbg !7116, !alias.scope !6937, !noalias !7117
  %628 = icmp eq i32 %width.i.i1652, 7, !dbg !7080
  br i1 %628, label %bb53.i.i1696.loopexit, label %bb36.i.i1675.7, !dbg !7080

bb36.i.i1675.7:                                   ; preds = %bb18.i.i1693.6
  %exitcond20915.7.not = icmp eq i32 %_159.1.i.i1680, 7, !dbg !7083
  br i1 %exitcond20915.7.not, label %panic.i.i1682, label %bb14.i.i1683.7, !dbg !7083

bb14.i.i1683.7:                                   ; preds = %bb36.i.i1675.7
  %629 = getelementptr inbounds nuw i8, ptr %_159.0.i.i1684, i32 92, !dbg !7083
  %_42.i.i1685.7 = load i32, ptr %629, align 4, !dbg !7083, !noalias !7111, !noundef !10
  %630 = add i32 %_42.i.i1685.7, %ring_cursor.sroa.0.1.i152318454, !dbg !7112
  %_45.not.i.i1686.7 = icmp ult i32 %630, %_91.i1555, !dbg !7113
  %631 = select i1 %_45.not.i.i1686.7, i32 0, i32 %_91.i1555, !dbg !7113
  %spec.select.i.i1687.7 = sub nuw i32 %630, %631, !dbg !7113
  %_49.i.i1688.7 = mul i32 %spec.select.i.i1687.7, %width.i.i1652, !dbg !7114
  %_48.i.i1689.7 = add i32 %_49.i.i1688.7, 7, !dbg !7114
  %_51.i.i1691.7 = icmp ult i32 %_48.i.i1689.7, %_163.1.i.i1701.pre, !dbg !7115
  br i1 %_51.i.i1691.7, label %bb18.i.i1693.7, label %panic1.i.i1692, !dbg !7115

bb18.i.i1693.7:                                   ; preds = %bb14.i.i1683.7
  %632 = getelementptr inbounds nuw float, ptr %_161.0.i.i1694, i32 %_48.i.i1689.7, !dbg !7115
  %_47.i.i1695.7 = load float, ptr %632, align 4, !dbg !7115, !noalias !7111, !noundef !10
  store float %_47.i.i1695.7, ptr %iter.sroa.0.0.ptr.i.i167418451.7, align 4, !dbg !7116, !alias.scope !6937, !noalias !7117
  br label %bb53.i.i1696.loopexit, !dbg !7080

panic1.i.i1692:                                   ; preds = %bb14.i.i1683.7, %bb14.i.i1683.6, %bb14.i.i1683.5, %bb14.i.i1683.4, %bb14.i.i1683.3, %bb14.i.i1683.2, %bb14.i.i1683.1, %bb14.i.i1683
  %_48.i.i1689.lcssa.ph = phi i32 [ %_48.i.i1689.7, %bb14.i.i1683.7 ], [ %_48.i.i1689.6, %bb14.i.i1683.6 ], [ %_48.i.i1689.5, %bb14.i.i1683.5 ], [ %_48.i.i1689.4, %bb14.i.i1683.4 ], [ %_48.i.i1689.3, %bb14.i.i1683.3 ], [ %_48.i.i1689.2, %bb14.i.i1683.2 ], [ %_48.i.i1689.1, %bb14.i.i1683.1 ], [ %_49.i.i1688, %bb14.i.i1683 ]
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %503, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_48.i.i1689.lcssa.ph, i32 noundef %_163.1.i.i1701.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_0e76a26fbb751dfb8cf8a069b5b0d39a) #32, !dbg !7115, !noalias !7111
  unreachable, !dbg !7115

bb42.i.i1703:                                     ; preds = %bb53.i.i1696
  %_126.i.i1705 = sub nuw i32 %_163.1.i.i1701.pre, %_22.i.i1658, !dbg !7118
  %_8.i6422 = icmp samesign ugt i32 %_126.i.i1705, 3, !dbg !7119
  br i1 %_8.i6422, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6425, label %bb2.i6423, !dbg !7119, !prof !1039

bb2.i6423:                                        ; preds = %bb42.i.i1703
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %503, ptr %149, align 16, !dbg !5168
  store <4 x float> %593, ptr %166, align 16, !dbg !5183
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_126.i.i1705, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !7124, !noalias !7125
  unreachable, !dbg !7124

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6425: ; preds = %bb42.i.i1703
  %_163.0.i.i1704 = load ptr, ptr %165, align 4, !dbg !7082, !alias.scope !6940, !noalias !6941, !nonnull !10, !noundef !10
  %_130.i.i1706 = getelementptr inbounds nuw float, ptr %_163.0.i.i1704, i32 %_22.i.i1658, !dbg !7129
  store <4 x float> %590, ptr %_130.i.i1706, align 4, !dbg !7131, !alias.scope !7135, !noalias !7139
  %_62.i.i170816009 = load <4 x float>, ptr %167, align 16, !dbg !7141
  %_66.i.i171116010 = load <4 x float>, ptr %168, align 16, !dbg !7142
  %633 = fdiv <4 x float> %593, %_62.i.i170816009, !dbg !7143
  %634 = fsub <4 x float> splat (float 1.000000e+00), %633, !dbg !7147
  %635 = fsub <4 x float> %634, %_66.i.i171116010, !dbg !7151
  %636 = bitcast <16 x i8> %_4.i7043 to <4 x float>, !dbg !7155
  %637 = fmul <4 x float> %635, %636, !dbg !7159
  %638 = fadd <4 x float> %_66.i.i171116010, %637, !dbg !7160
  %639 = fcmp olt <4 x float> %638, %634, !dbg !7163
  %640 = select <4 x i1> %639, <4 x float> %634, <4 x float> %638, !dbg !7167
  %641 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %640), !dbg !7168
  %642 = fcmp uge <4 x float> %641, splat (float 0x3BC79CA100000000), !dbg !7173
  %643 = bitcast <4 x float> %640 to <4 x i32>, !dbg !7178
  %644 = select <4 x i1> %642, <4 x i32> %643, <4 x i32> zeroinitializer, !dbg !7178
  store <4 x i32> %644, ptr %168, align 16, !dbg !7181
  %645 = bitcast <4 x i32> %644 to <4 x float>, !dbg !7182
  %646 = fsub <4 x float> splat (float 1.000000e+00), %645, !dbg !7186
  %_164.1.i.i1721 = load i32, ptr %169, align 4, !dbg !7187, !alias.scope !6940, !noalias !6941, !noundef !10
  %_74.i.i1722 = mul i32 %width.i.i1652, %main_cursor.sroa.0.1.i152418455, !dbg !7188
  %_134.i.i1723 = icmp ugt i32 %_74.i.i1722, %_164.1.i.i1721, !dbg !7189
  br i1 %_134.i.i1723, label %bb47.i.i1748, label %bb48.i.i1724, !dbg !7189, !prof !787

bb41.i.i1749:                                     ; preds = %bb53.i.i1696
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %503, ptr %149, align 16, !dbg !5168
  store <4 x float> %593, ptr %166, align 16, !dbg !5183
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i1658, i32 noundef %_163.1.i.i1701.pre, i32 noundef %_163.1.i.i1701.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c2286ff41a33b8c11aa270fe215441de) #32, !dbg !7192, !noalias !7111
  unreachable, !dbg !7192

bb48.i.i1724:                                     ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6425
  %_137.i.i1726 = sub nuw i32 %_164.1.i.i1721, %_74.i.i1722, !dbg !7193
  %_8.i5647 = icmp samesign ugt i32 %_137.i.i1726, 3, !dbg !7194
  br i1 %_8.i5647, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit, label %bb2.i5648, !dbg !7194, !prof !1039

bb2.i5648:                                        ; preds = %bb48.i.i1724
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %503, ptr %149, align 16, !dbg !5168
  store <4 x float> %593, ptr %166, align 16, !dbg !5183
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_137.i.i1726, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !7199, !noalias !7200
  unreachable, !dbg !7199

bb47.i.i1748:                                     ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6425
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %503, ptr %149, align 16, !dbg !5168
  store <4 x float> %593, ptr %166, align 16, !dbg !5183
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_74.i.i1722, i32 noundef %_164.1.i.i1721, i32 noundef %_164.1.i.i1721, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a3a21efd18b403ec25db545d522c479) #32, !dbg !7204, !noalias !7111
  unreachable, !dbg !7204

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit: ; preds = %bb48.i.i1724
  %_164.0.i.i1725 = load ptr, ptr %170, align 4, !dbg !7187, !alias.scope !6940, !noalias !6941, !nonnull !10, !noundef !10
  %_141.i.i1727 = getelementptr inbounds nuw float, ptr %_164.0.i.i1725, i32 %_74.i.i1722, !dbg !7205
  %lanes.i5644.sroa.0.0.copyload = load <4 x i32>, ptr %_141.i.i1727, align 4, !dbg !7207, !alias.scope !7211, !noalias !7215
  store <4 x i32> %lanes.i5667.sroa.0.0.copyload, ptr %_141.i.i1727, align 4, !dbg !7217, !alias.scope !7222, !noalias !7226
  %647 = bitcast <4 x i32> %lanes.i5644.sroa.0.0.copyload to <4 x float>, !dbg !7230
  %648 = fmul <4 x float> %646, %647, !dbg !7234
  %649 = bitcast <4 x i32> %lanes.i5644.sroa.0.0.copyload to <16 x i8>, !dbg !7235
  %650 = bitcast <4 x float> %648 to <16 x i8>, !dbg !7239
  %_4.i7067 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %649, <16 x i8> %650, <16 x i8> %154), !dbg !7240
  store <16 x i8> %_4.i7067, ptr %_178.i1649, align 4, !dbg !7241, !alias.scope !7246, !noalias !7250
  %651 = add i32 %main_cursor.sroa.0.1.i152418455, 1, !dbg !7254
  %_106.i1741 = load i32, ptr %171, align 4, !dbg !7255, !alias.scope !5075, !noalias !6500, !noundef !10
  %_104.i1742 = icmp eq i32 %651, %_106.i1741, !dbg !7256
  %spec.store.select11.i1743 = select i1 %_104.i1742, i32 0, i32 %651, !dbg !7256
  %652 = add i32 %ring_cursor.sroa.0.1.i152318454, 1, !dbg !7257
  %_107.i1744 = icmp eq i32 %652, %_91.i1555, !dbg !7258
  %spec.store.select12.i1745 = select i1 %_107.i1744, i32 0, i32 %652, !dbg !7258
  %exitcond20919.not = icmp eq i32 %449, %umax20918, !dbg !7259
  br i1 %exitcond20919.not, label %bb13.i1108.loopexit.loopexit, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5724, !dbg !6235

bb56.i1751:                                       ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6435
  store <4 x float> %424, ptr %126, align 16, !dbg !5140
  store <16 x i8> %_4.i7037, ptr %_68.i1529, align 16, !dbg !5153
  store <16 x i8> %_4.i7038, ptr %127, align 16, !dbg !5155
  store <4 x float> %433, ptr %129, align 16, !dbg !5156
  store <16 x i8> %_4.i7039, ptr %_69.i1530, align 16, !dbg !5158
  store <16 x i8> %_4.i7040, ptr %130, align 16, !dbg !5159
  store <4 x float> %442, ptr %132, align 16, !dbg !5160
  store <16 x i8> %_4.i7041, ptr %_73.i1533, align 16, !dbg !5164
  store <16 x i8> %_4.i7042, ptr %133, align 16, !dbg !5165
  store <4 x float> %452, ptr %135, align 16, !dbg !5166
  store <4 x float> %503, ptr %149, align 16, !dbg !5168
  store <4 x float> %420, ptr %166, align 16, !dbg !5183
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1528, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_7e50e0c93e87b60da469fc303486c501) #32, !dbg !7262, !noalias !5208
  unreachable, !dbg !7262

_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %bb13.i1108.loopexit, %bb12.i
  %ring_cursor.sroa.0.0.i1111.lcssa = phi i32 [ %_37.i1100, %bb12.i ], [ %ring_cursor.sroa.0.1.i1523.lcssa, %bb13.i1108.loopexit ], !dbg !5101
  %main_cursor.sroa.0.0.i1112.lcssa = phi i32 [ %_35.i1099, %bb12.i ], [ %main_cursor.sroa.0.1.i1524.lcssa, %bb13.i1108.loopexit ], !dbg !5098
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_(ptr noalias noundef readonly align 16 captures(none) dereferenceable(368) %hot_left.i1091, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33) #31, !dbg !7263
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_(ptr noalias noundef readonly align 16 captures(none) dereferenceable(368) %hot_right.i1090, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34) #31, !dbg !7264
  store i32 %main_cursor.sroa.0.0.i1112.lcssa, ptr %_35, align 4, !dbg !7265, !alias.scope !5081, !noalias !5100
  store i32 %ring_cursor.sroa.0.0.i1111.lcssa, ptr %81, align 4, !dbg !7266, !alias.scope !5081, !noalias !5100
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i1086), !dbg !7267, !noalias !5105
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i1087), !dbg !7268, !noalias !5105
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i1088), !dbg !7269, !noalias !5105
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !5074

bb11.i:                                           ; preds = %bb10.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7270), !dbg !7273
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7274), !dbg !7273
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7276), !dbg !7273
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7278), !dbg !7273
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_(ptr noalias noundef align 16 captures(none) dereferenceable(368) %hot_left.i783, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #31, !dbg !7280
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_(ptr noalias noundef align 16 captures(none) dereferenceable(368) %hot_right.i782, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #31, !dbg !7284
  %653 = load i8, ptr %79, align 16, !dbg !7286, !range !4667, !alias.scope !7270, !noalias !7290, !noundef !10
  %654 = load i8, ptr %80, align 1, !dbg !7294, !range !4667, !alias.scope !7270, !noalias !7290, !noundef !10
  %_35.i = load i32, ptr %_35, align 4, !dbg !7296, !alias.scope !7278, !noalias !7298, !noundef !10
  %_37.i790 = load i32, ptr %81, align 4, !dbg !7299, !alias.scope !7278, !noalias !7298, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i), !dbg !7301, !noalias !7303
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i, i8 0, i32 32, i1 false), !noalias !7303
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i780), !dbg !7304, !noalias !7303
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i780, i8 0, i32 1024, i1 false), !noalias !7303
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i779), !dbg !7306, !noalias !7303
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i779, i8 0, i32 1024, i1 false), !noalias !7303
  br i1 %_115.not.i18696, label %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb37.i.lr.ph, !dbg !7308

bb37.i.lr.ph:                                     ; preds = %bb11.i
  %_33.i789 = trunc nuw i8 %654 to i1, !dbg !7294
  %spec.store.select27.i = select i1 %_33.i789, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !7294
  %_32.i787 = trunc nuw i8 %653 to i1, !dbg !7286
  %link.sroa.0.0.i788 = select i1 %_32.i787, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !7286
  %d9.i7069 = lshr i32 %frames, 5, !dbg !7318
  %r2.i7070 = and i32 %frames, 31, !dbg !7325
  %_19.not.i7071 = icmp ne i32 %r2.i7070, 0, !dbg !7326
  %655 = zext i1 %_19.not.i7071 to i32, !dbg !7326
  %yield_count.sroa.0.0.i7072 = add nuw nsw i32 %d9.i7069, %655, !dbg !7326
  %history.i142.i.sroa.7.0.hot_left.i783.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i783, i32 16
  %history.i142.i.sroa.10.0.hot_left.i783.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i783, i32 32
  %history.i142.i.sroa.13.0.hot_left.i783.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i783, i32 48
  %history.i142.i.sroa.16.0.hot_left.i783.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i783, i32 64
  %history.i142.i.sroa.19.0.hot_left.i783.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i783, i32 80
  %history.i142.i.sroa.22.0.hot_left.i783.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i783, i32 96
  %history.i142.i.sroa.26.0.hot_left.i783.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i783, i32 112
  %history.i142.i.sroa.29.0.hot_left.i783.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i783, i32 128
  %history.i142.i.sroa.32.0.hot_left.i783.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i783, i32 144
  %history.i142.i.sroa.35.0.hot_left.i783.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i783, i32 160
  %history.i142.i.sroa.38.0.hot_left.i783.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i783, i32 176
  %656 = getelementptr inbounds nuw i8, ptr %self, i32 32
  %657 = getelementptr inbounds nuw i8, ptr %self, i32 48
  %658 = getelementptr inbounds nuw i8, ptr %self, i32 64
  %row1.i.i.i175.i = getelementptr inbounds nuw i8, ptr %self, i32 80
  %659 = getelementptr inbounds nuw i8, ptr %self, i32 96
  %660 = getelementptr inbounds nuw i8, ptr %self, i32 112
  %661 = getelementptr inbounds nuw i8, ptr %self, i32 128
  %row3.i.i.i189.i = getelementptr inbounds nuw i8, ptr %self, i32 144
  %662 = getelementptr inbounds nuw i8, ptr %self, i32 160
  %663 = getelementptr inbounds nuw i8, ptr %self, i32 176
  %664 = getelementptr inbounds nuw i8, ptr %self, i32 192
  %row5.i.i.i203.i = getelementptr inbounds nuw i8, ptr %self, i32 208
  %665 = getelementptr inbounds nuw i8, ptr %self, i32 224
  %666 = getelementptr inbounds nuw i8, ptr %self, i32 240
  %667 = getelementptr inbounds nuw i8, ptr %self, i32 256
  %row7.i.i.i217.i = getelementptr inbounds nuw i8, ptr %self, i32 272
  %668 = getelementptr inbounds nuw i8, ptr %self, i32 288
  %669 = getelementptr inbounds nuw i8, ptr %self, i32 304
  %670 = getelementptr inbounds nuw i8, ptr %self, i32 320
  %row9.i.i.i231.i = getelementptr inbounds nuw i8, ptr %self, i32 336
  %671 = getelementptr inbounds nuw i8, ptr %self, i32 352
  %672 = getelementptr inbounds nuw i8, ptr %self, i32 368
  %673 = getelementptr inbounds nuw i8, ptr %self, i32 384
  %row11.i.i.i245.i = getelementptr inbounds nuw i8, ptr %self, i32 400
  %674 = getelementptr inbounds nuw i8, ptr %self, i32 416
  %675 = getelementptr inbounds nuw i8, ptr %self, i32 432
  %676 = getelementptr inbounds nuw i8, ptr %self, i32 448
  %row13.i.i.i259.i = getelementptr inbounds nuw i8, ptr %self, i32 464
  %677 = getelementptr inbounds nuw i8, ptr %self, i32 480
  %678 = getelementptr inbounds nuw i8, ptr %self, i32 496
  %679 = getelementptr inbounds nuw i8, ptr %self, i32 512
  %row15.i.i.i273.i = getelementptr inbounds nuw i8, ptr %self, i32 528
  %680 = getelementptr inbounds nuw i8, ptr %self, i32 544
  %681 = getelementptr inbounds nuw i8, ptr %self, i32 560
  %682 = getelementptr inbounds nuw i8, ptr %self, i32 576
  %row17.i.i.i287.i = getelementptr inbounds nuw i8, ptr %self, i32 592
  %683 = getelementptr inbounds nuw i8, ptr %self, i32 608
  %684 = getelementptr inbounds nuw i8, ptr %self, i32 624
  %685 = getelementptr inbounds nuw i8, ptr %self, i32 640
  %row19.i.i.i301.i = getelementptr inbounds nuw i8, ptr %self, i32 656
  %686 = getelementptr inbounds nuw i8, ptr %self, i32 672
  %687 = getelementptr inbounds nuw i8, ptr %self, i32 688
  %688 = getelementptr inbounds nuw i8, ptr %self, i32 704
  %row21.i.i.i315.i = getelementptr inbounds nuw i8, ptr %self, i32 720
  %689 = getelementptr inbounds nuw i8, ptr %self, i32 736
  %690 = getelementptr inbounds nuw i8, ptr %self, i32 752
  %691 = getelementptr inbounds nuw i8, ptr %self, i32 768
  %history.i.i777.sroa.7.0.hot_right.i782.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i782, i32 16
  %history.i.i777.sroa.10.0.hot_right.i782.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i782, i32 32
  %history.i.i777.sroa.13.0.hot_right.i782.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i782, i32 48
  %history.i.i777.sroa.16.0.hot_right.i782.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i782, i32 64
  %history.i.i777.sroa.19.0.hot_right.i782.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i782, i32 80
  %history.i.i777.sroa.22.0.hot_right.i782.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i782, i32 96
  %history.i.i777.sroa.26.0.hot_right.i782.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i782, i32 112
  %history.i.i777.sroa.29.0.hot_right.i782.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i782, i32 128
  %history.i.i777.sroa.32.0.hot_right.i782.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i782, i32 144
  %history.i.i777.sroa.35.0.hot_right.i782.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i782, i32 160
  %history.i.i777.sroa.38.0.hot_right.i782.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i782, i32 176
  %_68.i = getelementptr inbounds nuw i8, ptr %hot_left.i783, i32 192
  %_69.i1005 = getelementptr inbounds nuw i8, ptr %hot_left.i783, i32 256
  %_73.i1008 = getelementptr inbounds nuw i8, ptr %hot_right.i782, i32 192
  %_74.i = getelementptr inbounds nuw i8, ptr %hot_right.i782, i32 256
  %692 = bitcast <4 x i32> %link.sroa.0.0.i788 to <16 x i8>
  %693 = getelementptr inbounds nuw i8, ptr %self, i32 916
  %694 = getelementptr inbounds nuw i8, ptr %self, i32 1020
  %695 = getelementptr inbounds nuw i8, ptr %self, i32 944
  %696 = getelementptr inbounds nuw i8, ptr %self, i32 940
  %697 = getelementptr inbounds nuw i8, ptr %self, i32 984
  %698 = getelementptr inbounds nuw i8, ptr %self, i32 980
  %699 = getelementptr inbounds nuw i8, ptr %self, i32 968
  %700 = getelementptr inbounds nuw i8, ptr %self, i32 964
  %701 = getelementptr inbounds nuw i8, ptr %self, i32 952
  %702 = getelementptr inbounds nuw i8, ptr %self, i32 948
  %703 = getelementptr inbounds nuw i8, ptr %hot_left.i783, i32 336
  %704 = getelementptr inbounds nuw i8, ptr %hot_left.i783, i32 352
  %705 = getelementptr inbounds nuw i8, ptr %hot_left.i783, i32 320
  %706 = getelementptr inbounds nuw i8, ptr %self, i32 936
  %707 = getelementptr inbounds nuw i8, ptr %self, i32 932
  %708 = bitcast <4 x i32> %spec.store.select27.i to <16 x i8>
  %709 = getelementptr inbounds nuw i8, ptr %self, i32 1120
  %710 = getelementptr inbounds nuw i8, ptr %self, i32 1044
  %711 = getelementptr inbounds nuw i8, ptr %self, i32 1040
  %712 = getelementptr inbounds nuw i8, ptr %self, i32 1116
  %713 = getelementptr inbounds nuw i8, ptr %self, i32 1112
  %714 = getelementptr inbounds nuw i8, ptr %self, i32 1084
  %715 = getelementptr inbounds nuw i8, ptr %self, i32 1080
  %716 = getelementptr inbounds nuw i8, ptr %self, i32 1068
  %717 = getelementptr inbounds nuw i8, ptr %self, i32 1064
  %718 = getelementptr inbounds nuw i8, ptr %self, i32 1052
  %719 = getelementptr inbounds nuw i8, ptr %self, i32 1048
  %720 = getelementptr inbounds nuw i8, ptr %hot_right.i782, i32 336
  %721 = getelementptr inbounds nuw i8, ptr %hot_right.i782, i32 352
  %722 = getelementptr inbounds nuw i8, ptr %hot_right.i782, i32 320
  %723 = getelementptr inbounds nuw i8, ptr %self, i32 1036
  %724 = getelementptr inbounds nuw i8, ptr %self, i32 1032
  %725 = getelementptr inbounds nuw i8, ptr %self, i32 920
  %iter.sroa.0.0.ptr.i52.i18530.1 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 4
  %iter.sroa.0.0.ptr.i52.i18530.2 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 8
  %iter.sroa.0.0.ptr.i52.i18530.3 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 12
  %iter.sroa.0.0.ptr.i52.i18530.4 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 16
  %iter.sroa.0.0.ptr.i52.i18530.5 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 20
  %iter.sroa.0.0.ptr.i52.i18530.6 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 24
  %iter.sroa.0.0.ptr.i52.i18530.7 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 28
  %iter.sroa.0.0.ptr.i.i18542.1 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 4
  %iter.sroa.0.0.ptr.i.i18542.2 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 8
  %iter.sroa.0.0.ptr.i.i18542.3 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 12
  %iter.sroa.0.0.ptr.i.i18542.4 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 16
  %iter.sroa.0.0.ptr.i.i18542.5 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 20
  %iter.sroa.0.0.ptr.i.i18542.6 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 24
  %iter.sroa.0.0.ptr.i.i18542.7 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 28
  br label %bb37.i, !dbg !7308

bb16.i.bb13.i.loopexit_crit_edge:                 ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6465
  store <4 x float> %1014, ptr %703, align 16, !dbg !7327
  store <4 x float> %1102, ptr %720, align 16, !dbg !7343
  br label %bb13.i.loopexit, !dbg !7345

bb13.i.loopexit:                                  ; preds = %bb16.i.bb13.i.loopexit_crit_edge, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i999
  %ring_cursor.sroa.0.1.i1001.lcssa = phi i32 [ %spec.store.select12.i, %bb16.i.bb13.i.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i79618699, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i999 ], !dbg !7351
  %main_cursor.sroa.0.1.i1002.lcssa = phi i32 [ %spec.store.select11.i, %bb16.i.bb13.i.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i79718700, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i999 ], !dbg !7352
  %_115.not.i = icmp eq i32 %728, 0, !dbg !7308
  %indvars.iv.next20921 = add i32 %indvars.iv20920, -32, !dbg !7308
  br i1 %_115.not.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb37.i, !dbg !7308

bb37.i:                                           ; preds = %bb37.i.lr.ph, %bb13.i.loopexit
  %indvars.iv20920 = phi i32 [ %frames, %bb37.i.lr.ph ], [ %indvars.iv.next20921, %bb13.i.loopexit ]
  %main_cursor.sroa.0.0.i79718700 = phi i32 [ %_35.i, %bb37.i.lr.ph ], [ %main_cursor.sroa.0.1.i1002.lcssa, %bb13.i.loopexit ]
  %ring_cursor.sroa.0.0.i79618699 = phi i32 [ %_37.i790, %bb37.i.lr.ph ], [ %ring_cursor.sroa.0.1.i1001.lcssa, %bb13.i.loopexit ]
  %iter2.sroa.0.0.i79518698 = phi i32 [ %yield_count.sroa.0.0.i7072, %bb37.i.lr.ph ], [ %728, %bb13.i.loopexit ]
  %iter.sroa.0.0.i18697 = phi i32 [ 0, %bb37.i.lr.ph ], [ %727, %bb13.i.loopexit ]
  %726 = call i32 @llvm.umax.i32(i32 %indvars.iv20920, i32 1), !dbg !7353
  %umax20947 = call i32 @llvm.umin.i32(i32 %726, i32 32), !dbg !7353
  %727 = add i32 %iter.sroa.0.0.i18697, 32, !dbg !7353
  %728 = add nsw i32 %iter2.sroa.0.0.i79518698, -1, !dbg !7357
  %729 = sub i32 %frames, %iter.sroa.0.0.i18697, !dbg !7358
  %spec.store.select.i798 = tail call i32 @llvm.umin.i32(i32 %729, i32 32), !dbg !7359
  %active_base.i799 = shl i32 %iter.sroa.0.0.i18697, 2, !dbg !7364
  %active_base.i79916121 = add i32 %spec.store.select.i798, %iter.sroa.0.0.i18697, !dbg !7365
  %_51.i = shl i32 %active_base.i79916121, 2, !dbg !7365
  %_125.i = icmp ult i32 %_51.i, %active_base.i799, !dbg !7366
  %_119.not.i = icmp ugt i32 %_51.i, %left_io.1
  %or.cond.i801 = or i1 %_125.i, %_119.not.i, !dbg !7366
  br i1 %or.cond.i801, label %bb43.i, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7091, !dbg !7366, !prof !4596

bb43.i:                                           ; preds = %bb37.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %active_base.i799, i32 noundef %_51.i, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_56e6f2edba81ce75fd22cd243776a278) #32, !dbg !7373, !noalias !7374
  unreachable, !dbg !7373

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7091: ; preds = %bb37.i
  %_128.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %active_base.i799, !dbg !7375
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7379), !dbg !7382
  %history.i142.i.sroa.0.0.copyload = load <4 x i32>, ptr %hot_left.i783, align 16, !dbg !7383
  %history.i142.i.sroa.7.0.copyload = load <4 x i32>, ptr %history.i142.i.sroa.7.0.hot_left.i783.sroa_idx, align 16, !dbg !7383
  %history.i142.i.sroa.10.0.copyload = load <4 x i32>, ptr %history.i142.i.sroa.10.0.hot_left.i783.sroa_idx, align 16, !dbg !7383
  %history.i142.i.sroa.13.0.copyload = load <4 x i32>, ptr %history.i142.i.sroa.13.0.hot_left.i783.sroa_idx, align 16, !dbg !7383
  %history.i142.i.sroa.16.0.copyload = load <4 x i32>, ptr %history.i142.i.sroa.16.0.hot_left.i783.sroa_idx, align 16, !dbg !7383
  %history.i142.i.sroa.19.0.copyload = load <4 x i32>, ptr %history.i142.i.sroa.19.0.hot_left.i783.sroa_idx, align 16, !dbg !7383
  %history.i142.i.sroa.22.0.copyload = load <4 x i32>, ptr %history.i142.i.sroa.22.0.hot_left.i783.sroa_idx, align 16, !dbg !7383
  %history.i142.i.sroa.26.0.copyload = load <4 x i32>, ptr %history.i142.i.sroa.26.0.hot_left.i783.sroa_idx, align 16, !dbg !7383
  %history.i142.i.sroa.29.0.copyload = load <4 x i32>, ptr %history.i142.i.sroa.29.0.hot_left.i783.sroa_idx, align 16, !dbg !7383
  %history.i142.i.sroa.32.0.copyload = load <4 x i32>, ptr %history.i142.i.sroa.32.0.hot_left.i783.sroa_idx, align 16, !dbg !7383
  %history.i142.i.sroa.35.0.copyload = load <4 x i32>, ptr %history.i142.i.sroa.35.0.hot_left.i783.sroa_idx, align 16, !dbg !7383
  %history.i142.i.sroa.38.0.copyload = load <4 x i32>, ptr %history.i142.i.sroa.38.0.hot_left.i783.sroa_idx, align 16, !dbg !7383
  %_2.i709418467.not = icmp eq i32 %frames, %iter.sroa.0.0.i18697, !dbg !7385
  br i1 %_2.i709418467.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit339.i, label %bb5.i145.i.lr.ph, !dbg !7385

bb5.i145.i.lr.ph:                                 ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7091
  %_11.i.i.i163.i16202 = load <4 x float>, ptr %_31, align 16, !alias.scope !7388, !noalias !7393
  %_14.i.i.i166.i16203 = load <4 x float>, ptr %656, align 16, !alias.scope !7388, !noalias !7393
  %_17.i.i.i169.i16204 = load <4 x float>, ptr %657, align 16, !alias.scope !7388, !noalias !7393
  %_20.i.i.i172.i16205 = load <4 x float>, ptr %658, align 16, !alias.scope !7388, !noalias !7393
  %_25.i.i.i177.i16206 = load <4 x float>, ptr %row1.i.i.i175.i, align 16, !alias.scope !7388, !noalias !7393
  %_28.i.i.i180.i16207 = load <4 x float>, ptr %659, align 16, !alias.scope !7388, !noalias !7393
  %_31.i.i.i183.i16208 = load <4 x float>, ptr %660, align 16, !alias.scope !7388, !noalias !7393
  %_34.i.i.i186.i16209 = load <4 x float>, ptr %661, align 16, !alias.scope !7388, !noalias !7393
  %_39.i.i.i191.i16210 = load <4 x float>, ptr %row3.i.i.i189.i, align 16, !alias.scope !7388, !noalias !7393
  %_42.i.i.i194.i16211 = load <4 x float>, ptr %662, align 16, !alias.scope !7388, !noalias !7393
  %_45.i.i.i197.i16212 = load <4 x float>, ptr %663, align 16, !alias.scope !7388, !noalias !7393
  %_48.i.i.i200.i16213 = load <4 x float>, ptr %664, align 16, !alias.scope !7388, !noalias !7393
  %_53.i.i.i205.i16214 = load <4 x float>, ptr %row5.i.i.i203.i, align 16, !alias.scope !7388, !noalias !7393
  %_56.i.i.i208.i16215 = load <4 x float>, ptr %665, align 16, !alias.scope !7388, !noalias !7393
  %_59.i.i.i211.i16216 = load <4 x float>, ptr %666, align 16, !alias.scope !7388, !noalias !7393
  %_62.i.i.i214.i16217 = load <4 x float>, ptr %667, align 16, !alias.scope !7388, !noalias !7393
  %_67.i.i.i219.i16218 = load <4 x float>, ptr %row7.i.i.i217.i, align 16, !alias.scope !7388, !noalias !7393
  %_70.i.i.i222.i16219 = load <4 x float>, ptr %668, align 16, !alias.scope !7388, !noalias !7393
  %_73.i.i.i225.i16220 = load <4 x float>, ptr %669, align 16, !alias.scope !7388, !noalias !7393
  %_76.i.i.i228.i16221 = load <4 x float>, ptr %670, align 16, !alias.scope !7388, !noalias !7393
  %_81.i.i.i233.i16222 = load <4 x float>, ptr %row9.i.i.i231.i, align 16, !alias.scope !7388, !noalias !7393
  %_84.i.i.i236.i16223 = load <4 x float>, ptr %671, align 16, !alias.scope !7388, !noalias !7393
  %_87.i.i.i239.i16224 = load <4 x float>, ptr %672, align 16, !alias.scope !7388, !noalias !7393
  %_90.i.i.i242.i16225 = load <4 x float>, ptr %673, align 16, !alias.scope !7388, !noalias !7393
  %_95.i.i.i247.i16226 = load <4 x float>, ptr %row11.i.i.i245.i, align 16, !alias.scope !7388, !noalias !7393
  %_98.i.i.i250.i16227 = load <4 x float>, ptr %674, align 16, !alias.scope !7388, !noalias !7393
  %_101.i.i.i253.i16228 = load <4 x float>, ptr %675, align 16, !alias.scope !7388, !noalias !7393
  %_104.i.i.i256.i16229 = load <4 x float>, ptr %676, align 16, !alias.scope !7388, !noalias !7393
  %_109.i.i.i261.i16230 = load <4 x float>, ptr %row13.i.i.i259.i, align 16, !alias.scope !7388, !noalias !7393
  %_112.i.i.i264.i16231 = load <4 x float>, ptr %677, align 16, !alias.scope !7388, !noalias !7393
  %_115.i.i.i267.i16232 = load <4 x float>, ptr %678, align 16, !alias.scope !7388, !noalias !7393
  %_118.i.i.i270.i16233 = load <4 x float>, ptr %679, align 16, !alias.scope !7388, !noalias !7393
  %_123.i.i.i275.i16234 = load <4 x float>, ptr %row15.i.i.i273.i, align 16, !alias.scope !7388, !noalias !7393
  %_126.i.i.i278.i16235 = load <4 x float>, ptr %680, align 16, !alias.scope !7388, !noalias !7393
  %_129.i.i.i281.i16236 = load <4 x float>, ptr %681, align 16, !alias.scope !7388, !noalias !7393
  %_132.i.i.i284.i16237 = load <4 x float>, ptr %682, align 16, !alias.scope !7388, !noalias !7393
  %_137.i.i.i289.i16238 = load <4 x float>, ptr %row17.i.i.i287.i, align 16, !alias.scope !7388, !noalias !7393
  %_140.i.i.i292.i16239 = load <4 x float>, ptr %683, align 16, !alias.scope !7388, !noalias !7393
  %_143.i.i.i295.i16240 = load <4 x float>, ptr %684, align 16, !alias.scope !7388, !noalias !7393
  %_146.i.i.i298.i16241 = load <4 x float>, ptr %685, align 16, !alias.scope !7388, !noalias !7393
  %_151.i.i.i303.i16242 = load <4 x float>, ptr %row19.i.i.i301.i, align 16, !alias.scope !7388, !noalias !7393
  %_154.i.i.i306.i16243 = load <4 x float>, ptr %686, align 16, !alias.scope !7388, !noalias !7393
  %_157.i.i.i309.i16244 = load <4 x float>, ptr %687, align 16, !alias.scope !7388, !noalias !7393
  %_160.i.i.i312.i16245 = load <4 x float>, ptr %688, align 16, !alias.scope !7388, !noalias !7393
  %_165.i.i.i317.i16246 = load <4 x float>, ptr %row21.i.i.i315.i, align 16, !alias.scope !7388, !noalias !7393
  %_168.i.i.i320.i16247 = load <4 x float>, ptr %689, align 16, !alias.scope !7388, !noalias !7393
  %_171.i.i.i323.i16248 = load <4 x float>, ptr %690, align 16, !alias.scope !7388, !noalias !7393
  %_174.i.i.i326.i16249 = load <4 x float>, ptr %691, align 16, !alias.scope !7388, !noalias !7393
  br label %bb5.i145.i, !dbg !7385

bb5.i145.i:                                       ; preds = %bb5.i145.i.lr.ph, %bb5.i145.i
  %iter.i138.i.sroa.16.018479 = phi i32 [ 0, %bb5.i145.i.lr.ph ], [ %851, %bb5.i145.i ]
  %history.i142.i.sroa.35.018478 = phi <4 x i32> [ %history.i142.i.sroa.35.0.copyload, %bb5.i145.i.lr.ph ], [ %history.i142.i.sroa.32.018477, %bb5.i145.i ]
  %history.i142.i.sroa.32.018477 = phi <4 x i32> [ %history.i142.i.sroa.32.0.copyload, %bb5.i145.i.lr.ph ], [ %history.i142.i.sroa.29.018476, %bb5.i145.i ]
  %history.i142.i.sroa.29.018476 = phi <4 x i32> [ %history.i142.i.sroa.29.0.copyload, %bb5.i145.i.lr.ph ], [ %history.i142.i.sroa.26.018475, %bb5.i145.i ]
  %history.i142.i.sroa.26.018475 = phi <4 x i32> [ %history.i142.i.sroa.26.0.copyload, %bb5.i145.i.lr.ph ], [ %history.i142.i.sroa.22.018474, %bb5.i145.i ]
  %history.i142.i.sroa.22.018474 = phi <4 x i32> [ %history.i142.i.sroa.22.0.copyload, %bb5.i145.i.lr.ph ], [ %history.i142.i.sroa.19.018473, %bb5.i145.i ]
  %history.i142.i.sroa.19.018473 = phi <4 x i32> [ %history.i142.i.sroa.19.0.copyload, %bb5.i145.i.lr.ph ], [ %history.i142.i.sroa.16.018472, %bb5.i145.i ]
  %history.i142.i.sroa.16.018472 = phi <4 x i32> [ %history.i142.i.sroa.16.0.copyload, %bb5.i145.i.lr.ph ], [ %history.i142.i.sroa.13.018471, %bb5.i145.i ]
  %history.i142.i.sroa.13.018471 = phi <4 x i32> [ %history.i142.i.sroa.13.0.copyload, %bb5.i145.i.lr.ph ], [ %history.i142.i.sroa.10.018470, %bb5.i145.i ]
  %history.i142.i.sroa.10.018470 = phi <4 x i32> [ %history.i142.i.sroa.10.0.copyload, %bb5.i145.i.lr.ph ], [ %history.i142.i.sroa.7.018469, %bb5.i145.i ]
  %history.i142.i.sroa.7.018469 = phi <4 x i32> [ %history.i142.i.sroa.7.0.copyload, %bb5.i145.i.lr.ph ], [ %history.i142.i.sroa.0.018468, %bb5.i145.i ]
  %history.i142.i.sroa.0.018468 = phi <4 x i32> [ %history.i142.i.sroa.0.0.copyload, %bb5.i145.i.lr.ph ], [ %lanes.i5835.sroa.0.0.copyload, %bb5.i145.i ]
  %start1.i.i7100 = shl i32 %iter.i138.i.sroa.16.018479, 2, !dbg !7402
  %data.i.i7101 = getelementptr inbounds nuw float, ptr %_128.i, i32 %start1.i.i7100, !dbg !7404
  %lanes.i5835.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i7101, align 4, !dbg !7406, !alias.scope !7411, !noalias !7415
  %730 = bitcast <4 x i32> %history.i142.i.sroa.19.018473 to <4 x float>, !dbg !7419
  %731 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %730), !dbg !7424
  %732 = bitcast <4 x i32> %lanes.i5835.sroa.0.0.copyload to <4 x float>, !dbg !7425
  %733 = fmul <4 x float> %_11.i.i.i163.i16202, %732, !dbg !7430
  %734 = fadd <4 x float> %733, zeroinitializer, !dbg !7431
  %735 = fmul <4 x float> %_14.i.i.i166.i16203, %732, !dbg !7435
  %736 = fadd <4 x float> %735, zeroinitializer, !dbg !7439
  %737 = fmul <4 x float> %_17.i.i.i169.i16204, %732, !dbg !7443
  %738 = fadd <4 x float> %737, zeroinitializer, !dbg !7447
  %739 = fmul <4 x float> %_20.i.i.i172.i16205, %732, !dbg !7451
  %740 = fadd <4 x float> %739, zeroinitializer, !dbg !7455
  %741 = bitcast <4 x i32> %history.i142.i.sroa.0.018468 to <4 x float>, !dbg !7459
  %742 = fmul <4 x float> %_25.i.i.i177.i16206, %741, !dbg !7463
  %743 = fadd <4 x float> %734, %742, !dbg !7464
  %744 = fmul <4 x float> %_28.i.i.i180.i16207, %741, !dbg !7468
  %745 = fadd <4 x float> %736, %744, !dbg !7472
  %746 = fmul <4 x float> %_31.i.i.i183.i16208, %741, !dbg !7476
  %747 = fadd <4 x float> %738, %746, !dbg !7480
  %748 = fmul <4 x float> %_34.i.i.i186.i16209, %741, !dbg !7484
  %749 = fadd <4 x float> %740, %748, !dbg !7488
  %750 = bitcast <4 x i32> %history.i142.i.sroa.7.018469 to <4 x float>, !dbg !7492
  %751 = fmul <4 x float> %_39.i.i.i191.i16210, %750, !dbg !7496
  %752 = fadd <4 x float> %743, %751, !dbg !7497
  %753 = fmul <4 x float> %_42.i.i.i194.i16211, %750, !dbg !7501
  %754 = fadd <4 x float> %745, %753, !dbg !7505
  %755 = fmul <4 x float> %_45.i.i.i197.i16212, %750, !dbg !7509
  %756 = fadd <4 x float> %747, %755, !dbg !7513
  %757 = fmul <4 x float> %_48.i.i.i200.i16213, %750, !dbg !7517
  %758 = fadd <4 x float> %749, %757, !dbg !7521
  %759 = bitcast <4 x i32> %history.i142.i.sroa.10.018470 to <4 x float>, !dbg !7525
  %760 = fmul <4 x float> %_53.i.i.i205.i16214, %759, !dbg !7529
  %761 = fadd <4 x float> %752, %760, !dbg !7530
  %762 = fmul <4 x float> %_56.i.i.i208.i16215, %759, !dbg !7534
  %763 = fadd <4 x float> %754, %762, !dbg !7538
  %764 = fmul <4 x float> %_59.i.i.i211.i16216, %759, !dbg !7542
  %765 = fadd <4 x float> %756, %764, !dbg !7546
  %766 = fmul <4 x float> %_62.i.i.i214.i16217, %759, !dbg !7550
  %767 = fadd <4 x float> %758, %766, !dbg !7554
  %768 = bitcast <4 x i32> %history.i142.i.sroa.13.018471 to <4 x float>, !dbg !7558
  %769 = fmul <4 x float> %_67.i.i.i219.i16218, %768, !dbg !7562
  %770 = fadd <4 x float> %761, %769, !dbg !7563
  %771 = fmul <4 x float> %_70.i.i.i222.i16219, %768, !dbg !7567
  %772 = fadd <4 x float> %763, %771, !dbg !7571
  %773 = fmul <4 x float> %_73.i.i.i225.i16220, %768, !dbg !7575
  %774 = fadd <4 x float> %765, %773, !dbg !7579
  %775 = fmul <4 x float> %_76.i.i.i228.i16221, %768, !dbg !7583
  %776 = fadd <4 x float> %767, %775, !dbg !7587
  %777 = bitcast <4 x i32> %history.i142.i.sroa.16.018472 to <4 x float>, !dbg !7591
  %778 = fmul <4 x float> %_81.i.i.i233.i16222, %777, !dbg !7595
  %779 = fadd <4 x float> %770, %778, !dbg !7596
  %780 = fmul <4 x float> %_84.i.i.i236.i16223, %777, !dbg !7600
  %781 = fadd <4 x float> %772, %780, !dbg !7604
  %782 = fmul <4 x float> %_87.i.i.i239.i16224, %777, !dbg !7608
  %783 = fadd <4 x float> %774, %782, !dbg !7612
  %784 = fmul <4 x float> %_90.i.i.i242.i16225, %777, !dbg !7616
  %785 = fadd <4 x float> %776, %784, !dbg !7620
  %786 = fmul <4 x float> %_95.i.i.i247.i16226, %730, !dbg !7624
  %787 = fadd <4 x float> %779, %786, !dbg !7628
  %788 = fmul <4 x float> %_98.i.i.i250.i16227, %730, !dbg !7632
  %789 = fadd <4 x float> %781, %788, !dbg !7636
  %790 = fmul <4 x float> %_101.i.i.i253.i16228, %730, !dbg !7640
  %791 = fadd <4 x float> %783, %790, !dbg !7644
  %792 = fmul <4 x float> %_104.i.i.i256.i16229, %730, !dbg !7648
  %793 = fadd <4 x float> %785, %792, !dbg !7652
  %794 = bitcast <4 x i32> %history.i142.i.sroa.22.018474 to <4 x float>, !dbg !7656
  %795 = fmul <4 x float> %_109.i.i.i261.i16230, %794, !dbg !7660
  %796 = fadd <4 x float> %787, %795, !dbg !7661
  %797 = fmul <4 x float> %_112.i.i.i264.i16231, %794, !dbg !7665
  %798 = fadd <4 x float> %789, %797, !dbg !7669
  %799 = fmul <4 x float> %_115.i.i.i267.i16232, %794, !dbg !7673
  %800 = fadd <4 x float> %791, %799, !dbg !7677
  %801 = fmul <4 x float> %_118.i.i.i270.i16233, %794, !dbg !7681
  %802 = fadd <4 x float> %793, %801, !dbg !7685
  %803 = bitcast <4 x i32> %history.i142.i.sroa.26.018475 to <4 x float>, !dbg !7689
  %804 = fmul <4 x float> %_123.i.i.i275.i16234, %803, !dbg !7693
  %805 = fadd <4 x float> %796, %804, !dbg !7694
  %806 = fmul <4 x float> %_126.i.i.i278.i16235, %803, !dbg !7698
  %807 = fadd <4 x float> %798, %806, !dbg !7702
  %808 = fmul <4 x float> %_129.i.i.i281.i16236, %803, !dbg !7706
  %809 = fadd <4 x float> %800, %808, !dbg !7710
  %810 = fmul <4 x float> %_132.i.i.i284.i16237, %803, !dbg !7714
  %811 = fadd <4 x float> %802, %810, !dbg !7718
  %812 = bitcast <4 x i32> %history.i142.i.sroa.29.018476 to <4 x float>, !dbg !7722
  %813 = fmul <4 x float> %_137.i.i.i289.i16238, %812, !dbg !7726
  %814 = fadd <4 x float> %805, %813, !dbg !7727
  %815 = fmul <4 x float> %_140.i.i.i292.i16239, %812, !dbg !7731
  %816 = fadd <4 x float> %807, %815, !dbg !7735
  %817 = fmul <4 x float> %_143.i.i.i295.i16240, %812, !dbg !7739
  %818 = fadd <4 x float> %809, %817, !dbg !7743
  %819 = fmul <4 x float> %_146.i.i.i298.i16241, %812, !dbg !7747
  %820 = fadd <4 x float> %811, %819, !dbg !7751
  %821 = bitcast <4 x i32> %history.i142.i.sroa.32.018477 to <4 x float>, !dbg !7755
  %822 = fmul <4 x float> %_151.i.i.i303.i16242, %821, !dbg !7759
  %823 = fadd <4 x float> %814, %822, !dbg !7760
  %824 = fmul <4 x float> %_154.i.i.i306.i16243, %821, !dbg !7764
  %825 = fadd <4 x float> %816, %824, !dbg !7768
  %826 = fmul <4 x float> %_157.i.i.i309.i16244, %821, !dbg !7772
  %827 = fadd <4 x float> %818, %826, !dbg !7776
  %828 = fmul <4 x float> %_160.i.i.i312.i16245, %821, !dbg !7780
  %829 = fadd <4 x float> %820, %828, !dbg !7784
  %830 = bitcast <4 x i32> %history.i142.i.sroa.35.018478 to <4 x float>, !dbg !7788
  %831 = fmul <4 x float> %_165.i.i.i317.i16246, %830, !dbg !7792
  %832 = fadd <4 x float> %823, %831, !dbg !7793
  %833 = fmul <4 x float> %_168.i.i.i320.i16247, %830, !dbg !7797
  %834 = fadd <4 x float> %825, %833, !dbg !7801
  %835 = fmul <4 x float> %_171.i.i.i323.i16248, %830, !dbg !7805
  %836 = fadd <4 x float> %827, %835, !dbg !7809
  %837 = fmul <4 x float> %_174.i.i.i326.i16249, %830, !dbg !7813
  %838 = fadd <4 x float> %829, %837, !dbg !7817
  %839 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %832), !dbg !7821
  %840 = fcmp olt <4 x float> %839, %731, !dbg !7825
  %841 = select <4 x i1> %840, <4 x float> %731, <4 x float> %839, !dbg !7829
  %842 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %834), !dbg !7821
  %843 = fcmp olt <4 x float> %842, %841, !dbg !7825
  %844 = select <4 x i1> %843, <4 x float> %841, <4 x float> %842, !dbg !7829
  %845 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %836), !dbg !7821
  %846 = fcmp olt <4 x float> %845, %844, !dbg !7825
  %847 = select <4 x i1> %846, <4 x float> %844, <4 x float> %845, !dbg !7829
  %848 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %838), !dbg !7821
  %849 = fcmp olt <4 x float> %848, %847, !dbg !7825
  %850 = select <4 x i1> %849, <4 x float> %847, <4 x float> %848, !dbg !7829
  %851 = add nuw nsw i32 %iter.i138.i.sroa.16.018479, 1, !dbg !7830
  %data.i4.i7105 = getelementptr inbounds nuw float, ptr %peaks_left.i780, i32 %start1.i.i7100, !dbg !7831
  store <4 x float> %850, ptr %data.i4.i7105, align 4, !dbg !7834, !alias.scope !7839, !noalias !7843
  %exitcond20924.not = icmp eq i32 %851, %umax20947, !dbg !7385
  br i1 %exitcond20924.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit339.i, label %bb5.i145.i, !dbg !7385

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit339.i: ; preds = %bb5.i145.i, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7091
  %history.i142.i.sroa.0.0.lcssa = phi <4 x i32> [ %history.i142.i.sroa.0.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7091 ], [ %lanes.i5835.sroa.0.0.copyload, %bb5.i145.i ], !dbg !7847
  %history.i142.i.sroa.7.0.lcssa = phi <4 x i32> [ %history.i142.i.sroa.7.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7091 ], [ %history.i142.i.sroa.0.018468, %bb5.i145.i ], !dbg !7847
  %history.i142.i.sroa.10.0.lcssa = phi <4 x i32> [ %history.i142.i.sroa.10.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7091 ], [ %history.i142.i.sroa.7.018469, %bb5.i145.i ], !dbg !7847
  %history.i142.i.sroa.13.0.lcssa = phi <4 x i32> [ %history.i142.i.sroa.13.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7091 ], [ %history.i142.i.sroa.10.018470, %bb5.i145.i ], !dbg !7847
  %history.i142.i.sroa.16.0.lcssa = phi <4 x i32> [ %history.i142.i.sroa.16.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7091 ], [ %history.i142.i.sroa.13.018471, %bb5.i145.i ], !dbg !7847
  %history.i142.i.sroa.19.0.lcssa = phi <4 x i32> [ %history.i142.i.sroa.19.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7091 ], [ %history.i142.i.sroa.16.018472, %bb5.i145.i ], !dbg !7847
  %history.i142.i.sroa.22.0.lcssa = phi <4 x i32> [ %history.i142.i.sroa.22.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7091 ], [ %history.i142.i.sroa.19.018473, %bb5.i145.i ], !dbg !7847
  %history.i142.i.sroa.26.0.lcssa = phi <4 x i32> [ %history.i142.i.sroa.26.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7091 ], [ %history.i142.i.sroa.22.018474, %bb5.i145.i ], !dbg !7847
  %history.i142.i.sroa.29.0.lcssa = phi <4 x i32> [ %history.i142.i.sroa.29.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7091 ], [ %history.i142.i.sroa.26.018475, %bb5.i145.i ], !dbg !7847
  %history.i142.i.sroa.32.0.lcssa = phi <4 x i32> [ %history.i142.i.sroa.32.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7091 ], [ %history.i142.i.sroa.29.018476, %bb5.i145.i ], !dbg !7847
  %history.i142.i.sroa.35.0.lcssa = phi <4 x i32> [ %history.i142.i.sroa.35.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7091 ], [ %history.i142.i.sroa.32.018477, %bb5.i145.i ], !dbg !7847
  %history.i142.i.sroa.38.0.lcssa = phi <4 x i32> [ %history.i142.i.sroa.38.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7091 ], [ %history.i142.i.sroa.35.018478, %bb5.i145.i ], !dbg !7847
  store <4 x i32> %history.i142.i.sroa.0.0.lcssa, ptr %hot_left.i783, align 16, !dbg !7848
  store <4 x i32> %history.i142.i.sroa.7.0.lcssa, ptr %history.i142.i.sroa.7.0.hot_left.i783.sroa_idx, align 16, !dbg !7848
  store <4 x i32> %history.i142.i.sroa.10.0.lcssa, ptr %history.i142.i.sroa.10.0.hot_left.i783.sroa_idx, align 16, !dbg !7848
  store <4 x i32> %history.i142.i.sroa.13.0.lcssa, ptr %history.i142.i.sroa.13.0.hot_left.i783.sroa_idx, align 16, !dbg !7848
  store <4 x i32> %history.i142.i.sroa.16.0.lcssa, ptr %history.i142.i.sroa.16.0.hot_left.i783.sroa_idx, align 16, !dbg !7848
  store <4 x i32> %history.i142.i.sroa.19.0.lcssa, ptr %history.i142.i.sroa.19.0.hot_left.i783.sroa_idx, align 16, !dbg !7848
  store <4 x i32> %history.i142.i.sroa.22.0.lcssa, ptr %history.i142.i.sroa.22.0.hot_left.i783.sroa_idx, align 16, !dbg !7848
  store <4 x i32> %history.i142.i.sroa.26.0.lcssa, ptr %history.i142.i.sroa.26.0.hot_left.i783.sroa_idx, align 16, !dbg !7848
  store <4 x i32> %history.i142.i.sroa.29.0.lcssa, ptr %history.i142.i.sroa.29.0.hot_left.i783.sroa_idx, align 16, !dbg !7848
  store <4 x i32> %history.i142.i.sroa.32.0.lcssa, ptr %history.i142.i.sroa.32.0.hot_left.i783.sroa_idx, align 16, !dbg !7848
  store <4 x i32> %history.i142.i.sroa.35.0.lcssa, ptr %history.i142.i.sroa.35.0.hot_left.i783.sroa_idx, align 16, !dbg !7848
  store <4 x i32> %history.i142.i.sroa.38.0.lcssa, ptr %history.i142.i.sroa.38.0.hot_left.i783.sroa_idx, align 16, !dbg !7848
  %_136.not.i = icmp ugt i32 %_51.i, %right_io.1, !dbg !7849
  br i1 %_136.not.i, label %bb49.i, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7141, !dbg !7849, !prof !787

bb49.i:                                           ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit339.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %active_base.i799, i32 noundef %_51.i, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_685aef6cd6b0f866813eafbee04e700c) #32, !dbg !7853, !noalias !7374
  unreachable, !dbg !7853

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7141: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit339.i
  %_143.i802 = getelementptr inbounds nuw float, ptr %right_io.0, i32 %active_base.i799, !dbg !7854
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7858), !dbg !7861
  %history.i.i777.sroa.0.0.copyload = load <4 x i32>, ptr %hot_right.i782, align 16, !dbg !7862
  %history.i.i777.sroa.7.0.copyload = load <4 x i32>, ptr %history.i.i777.sroa.7.0.hot_right.i782.sroa_idx, align 16, !dbg !7862
  %history.i.i777.sroa.10.0.copyload = load <4 x i32>, ptr %history.i.i777.sroa.10.0.hot_right.i782.sroa_idx, align 16, !dbg !7862
  %history.i.i777.sroa.13.0.copyload = load <4 x i32>, ptr %history.i.i777.sroa.13.0.hot_right.i782.sroa_idx, align 16, !dbg !7862
  %history.i.i777.sroa.16.0.copyload = load <4 x i32>, ptr %history.i.i777.sroa.16.0.hot_right.i782.sroa_idx, align 16, !dbg !7862
  %history.i.i777.sroa.19.0.copyload = load <4 x i32>, ptr %history.i.i777.sroa.19.0.hot_right.i782.sroa_idx, align 16, !dbg !7862
  %history.i.i777.sroa.22.0.copyload = load <4 x i32>, ptr %history.i.i777.sroa.22.0.hot_right.i782.sroa_idx, align 16, !dbg !7862
  %history.i.i777.sroa.26.0.copyload = load <4 x i32>, ptr %history.i.i777.sroa.26.0.hot_right.i782.sroa_idx, align 16, !dbg !7862
  %history.i.i777.sroa.29.0.copyload = load <4 x i32>, ptr %history.i.i777.sroa.29.0.hot_right.i782.sroa_idx, align 16, !dbg !7862
  %history.i.i777.sroa.32.0.copyload = load <4 x i32>, ptr %history.i.i777.sroa.32.0.hot_right.i782.sroa_idx, align 16, !dbg !7862
  %history.i.i777.sroa.35.0.copyload = load <4 x i32>, ptr %history.i.i777.sroa.35.0.hot_right.i782.sroa_idx, align 16, !dbg !7862
  %history.i.i777.sroa.38.0.copyload = load <4 x i32>, ptr %history.i.i777.sroa.38.0.hot_right.i782.sroa_idx, align 16, !dbg !7862
  br i1 %_2.i709418467.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i999, label %bb5.i.i805.lr.ph, !dbg !7864

bb5.i.i805.lr.ph:                                 ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7141
  %_11.i.i.i.i82316149 = load <4 x float>, ptr %_31, align 16, !alias.scope !7867, !noalias !7872
  %_14.i.i.i.i82616150 = load <4 x float>, ptr %656, align 16, !alias.scope !7867, !noalias !7872
  %_17.i.i.i.i82916151 = load <4 x float>, ptr %657, align 16, !alias.scope !7867, !noalias !7872
  %_20.i.i.i.i83216152 = load <4 x float>, ptr %658, align 16, !alias.scope !7867, !noalias !7872
  %_25.i.i.i.i83716153 = load <4 x float>, ptr %row1.i.i.i175.i, align 16, !alias.scope !7867, !noalias !7872
  %_28.i.i.i.i84016154 = load <4 x float>, ptr %659, align 16, !alias.scope !7867, !noalias !7872
  %_31.i.i.i.i84316155 = load <4 x float>, ptr %660, align 16, !alias.scope !7867, !noalias !7872
  %_34.i.i.i.i84616156 = load <4 x float>, ptr %661, align 16, !alias.scope !7867, !noalias !7872
  %_39.i.i.i.i85116157 = load <4 x float>, ptr %row3.i.i.i189.i, align 16, !alias.scope !7867, !noalias !7872
  %_42.i.i.i.i85416158 = load <4 x float>, ptr %662, align 16, !alias.scope !7867, !noalias !7872
  %_45.i.i.i.i85716159 = load <4 x float>, ptr %663, align 16, !alias.scope !7867, !noalias !7872
  %_48.i.i.i.i86016160 = load <4 x float>, ptr %664, align 16, !alias.scope !7867, !noalias !7872
  %_53.i.i.i.i86516161 = load <4 x float>, ptr %row5.i.i.i203.i, align 16, !alias.scope !7867, !noalias !7872
  %_56.i.i.i.i86816162 = load <4 x float>, ptr %665, align 16, !alias.scope !7867, !noalias !7872
  %_59.i.i.i.i87116163 = load <4 x float>, ptr %666, align 16, !alias.scope !7867, !noalias !7872
  %_62.i.i.i.i87416164 = load <4 x float>, ptr %667, align 16, !alias.scope !7867, !noalias !7872
  %_67.i.i.i.i87916165 = load <4 x float>, ptr %row7.i.i.i217.i, align 16, !alias.scope !7867, !noalias !7872
  %_70.i.i.i.i88216166 = load <4 x float>, ptr %668, align 16, !alias.scope !7867, !noalias !7872
  %_73.i.i.i.i88516167 = load <4 x float>, ptr %669, align 16, !alias.scope !7867, !noalias !7872
  %_76.i.i.i.i88816168 = load <4 x float>, ptr %670, align 16, !alias.scope !7867, !noalias !7872
  %_81.i.i.i.i89316169 = load <4 x float>, ptr %row9.i.i.i231.i, align 16, !alias.scope !7867, !noalias !7872
  %_84.i.i.i.i89616170 = load <4 x float>, ptr %671, align 16, !alias.scope !7867, !noalias !7872
  %_87.i.i.i.i89916171 = load <4 x float>, ptr %672, align 16, !alias.scope !7867, !noalias !7872
  %_90.i.i.i.i90216172 = load <4 x float>, ptr %673, align 16, !alias.scope !7867, !noalias !7872
  %_95.i.i.i.i90716173 = load <4 x float>, ptr %row11.i.i.i245.i, align 16, !alias.scope !7867, !noalias !7872
  %_98.i.i.i.i91016174 = load <4 x float>, ptr %674, align 16, !alias.scope !7867, !noalias !7872
  %_101.i.i.i.i91316175 = load <4 x float>, ptr %675, align 16, !alias.scope !7867, !noalias !7872
  %_104.i.i.i.i91616176 = load <4 x float>, ptr %676, align 16, !alias.scope !7867, !noalias !7872
  %_109.i.i.i.i92116177 = load <4 x float>, ptr %row13.i.i.i259.i, align 16, !alias.scope !7867, !noalias !7872
  %_112.i.i.i.i92416178 = load <4 x float>, ptr %677, align 16, !alias.scope !7867, !noalias !7872
  %_115.i.i.i.i92716179 = load <4 x float>, ptr %678, align 16, !alias.scope !7867, !noalias !7872
  %_118.i.i.i.i93016180 = load <4 x float>, ptr %679, align 16, !alias.scope !7867, !noalias !7872
  %_123.i.i.i.i93516181 = load <4 x float>, ptr %row15.i.i.i273.i, align 16, !alias.scope !7867, !noalias !7872
  %_126.i.i.i.i93816182 = load <4 x float>, ptr %680, align 16, !alias.scope !7867, !noalias !7872
  %_129.i.i.i.i94116183 = load <4 x float>, ptr %681, align 16, !alias.scope !7867, !noalias !7872
  %_132.i.i.i.i94416184 = load <4 x float>, ptr %682, align 16, !alias.scope !7867, !noalias !7872
  %_137.i.i.i.i94916185 = load <4 x float>, ptr %row17.i.i.i287.i, align 16, !alias.scope !7867, !noalias !7872
  %_140.i.i.i.i95216186 = load <4 x float>, ptr %683, align 16, !alias.scope !7867, !noalias !7872
  %_143.i.i.i.i95516187 = load <4 x float>, ptr %684, align 16, !alias.scope !7867, !noalias !7872
  %_146.i.i.i.i95816188 = load <4 x float>, ptr %685, align 16, !alias.scope !7867, !noalias !7872
  %_151.i.i.i.i96316189 = load <4 x float>, ptr %row19.i.i.i301.i, align 16, !alias.scope !7867, !noalias !7872
  %_154.i.i.i.i96616190 = load <4 x float>, ptr %686, align 16, !alias.scope !7867, !noalias !7872
  %_157.i.i.i.i96916191 = load <4 x float>, ptr %687, align 16, !alias.scope !7867, !noalias !7872
  %_160.i.i.i.i97216192 = load <4 x float>, ptr %688, align 16, !alias.scope !7867, !noalias !7872
  %_165.i.i.i.i97716193 = load <4 x float>, ptr %row21.i.i.i315.i, align 16, !alias.scope !7867, !noalias !7872
  %_168.i.i.i.i98016194 = load <4 x float>, ptr %689, align 16, !alias.scope !7867, !noalias !7872
  %_171.i.i.i.i98316195 = load <4 x float>, ptr %690, align 16, !alias.scope !7867, !noalias !7872
  %_174.i.i.i.i98616196 = load <4 x float>, ptr %691, align 16, !alias.scope !7867, !noalias !7872
  br label %bb5.i.i805, !dbg !7864

bb5.i.i805:                                       ; preds = %bb5.i.i805.lr.ph, %bb5.i.i805
  %iter.i.i773.sroa.16.018506 = phi i32 [ 0, %bb5.i.i805.lr.ph ], [ %973, %bb5.i.i805 ]
  %history.i.i777.sroa.35.018505 = phi <4 x i32> [ %history.i.i777.sroa.35.0.copyload, %bb5.i.i805.lr.ph ], [ %history.i.i777.sroa.32.018504, %bb5.i.i805 ]
  %history.i.i777.sroa.32.018504 = phi <4 x i32> [ %history.i.i777.sroa.32.0.copyload, %bb5.i.i805.lr.ph ], [ %history.i.i777.sroa.29.018503, %bb5.i.i805 ]
  %history.i.i777.sroa.29.018503 = phi <4 x i32> [ %history.i.i777.sroa.29.0.copyload, %bb5.i.i805.lr.ph ], [ %history.i.i777.sroa.26.018502, %bb5.i.i805 ]
  %history.i.i777.sroa.26.018502 = phi <4 x i32> [ %history.i.i777.sroa.26.0.copyload, %bb5.i.i805.lr.ph ], [ %history.i.i777.sroa.22.018501, %bb5.i.i805 ]
  %history.i.i777.sroa.22.018501 = phi <4 x i32> [ %history.i.i777.sroa.22.0.copyload, %bb5.i.i805.lr.ph ], [ %history.i.i777.sroa.19.018500, %bb5.i.i805 ]
  %history.i.i777.sroa.19.018500 = phi <4 x i32> [ %history.i.i777.sroa.19.0.copyload, %bb5.i.i805.lr.ph ], [ %history.i.i777.sroa.16.018499, %bb5.i.i805 ]
  %history.i.i777.sroa.16.018499 = phi <4 x i32> [ %history.i.i777.sroa.16.0.copyload, %bb5.i.i805.lr.ph ], [ %history.i.i777.sroa.13.018498, %bb5.i.i805 ]
  %history.i.i777.sroa.13.018498 = phi <4 x i32> [ %history.i.i777.sroa.13.0.copyload, %bb5.i.i805.lr.ph ], [ %history.i.i777.sroa.10.018497, %bb5.i.i805 ]
  %history.i.i777.sroa.10.018497 = phi <4 x i32> [ %history.i.i777.sroa.10.0.copyload, %bb5.i.i805.lr.ph ], [ %history.i.i777.sroa.7.018496, %bb5.i.i805 ]
  %history.i.i777.sroa.7.018496 = phi <4 x i32> [ %history.i.i777.sroa.7.0.copyload, %bb5.i.i805.lr.ph ], [ %history.i.i777.sroa.0.018495, %bb5.i.i805 ]
  %history.i.i777.sroa.0.018495 = phi <4 x i32> [ %history.i.i777.sroa.0.0.copyload, %bb5.i.i805.lr.ph ], [ %lanes.i5826.sroa.0.0.copyload, %bb5.i.i805 ]
  %start1.i.i7150 = shl i32 %iter.i.i773.sroa.16.018506, 2, !dbg !7881
  %data.i.i7151 = getelementptr inbounds nuw float, ptr %_143.i802, i32 %start1.i.i7150, !dbg !7883
  %lanes.i5826.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i7151, align 4, !dbg !7885, !alias.scope !7890, !noalias !7894
  %852 = bitcast <4 x i32> %history.i.i777.sroa.19.018500 to <4 x float>, !dbg !7898
  %853 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %852), !dbg !7903
  %854 = bitcast <4 x i32> %lanes.i5826.sroa.0.0.copyload to <4 x float>, !dbg !7904
  %855 = fmul <4 x float> %_11.i.i.i.i82316149, %854, !dbg !7909
  %856 = fadd <4 x float> %855, zeroinitializer, !dbg !7910
  %857 = fmul <4 x float> %_14.i.i.i.i82616150, %854, !dbg !7914
  %858 = fadd <4 x float> %857, zeroinitializer, !dbg !7918
  %859 = fmul <4 x float> %_17.i.i.i.i82916151, %854, !dbg !7922
  %860 = fadd <4 x float> %859, zeroinitializer, !dbg !7926
  %861 = fmul <4 x float> %_20.i.i.i.i83216152, %854, !dbg !7930
  %862 = fadd <4 x float> %861, zeroinitializer, !dbg !7934
  %863 = bitcast <4 x i32> %history.i.i777.sroa.0.018495 to <4 x float>, !dbg !7938
  %864 = fmul <4 x float> %_25.i.i.i.i83716153, %863, !dbg !7942
  %865 = fadd <4 x float> %856, %864, !dbg !7943
  %866 = fmul <4 x float> %_28.i.i.i.i84016154, %863, !dbg !7947
  %867 = fadd <4 x float> %858, %866, !dbg !7951
  %868 = fmul <4 x float> %_31.i.i.i.i84316155, %863, !dbg !7955
  %869 = fadd <4 x float> %860, %868, !dbg !7959
  %870 = fmul <4 x float> %_34.i.i.i.i84616156, %863, !dbg !7963
  %871 = fadd <4 x float> %862, %870, !dbg !7967
  %872 = bitcast <4 x i32> %history.i.i777.sroa.7.018496 to <4 x float>, !dbg !7971
  %873 = fmul <4 x float> %_39.i.i.i.i85116157, %872, !dbg !7975
  %874 = fadd <4 x float> %865, %873, !dbg !7976
  %875 = fmul <4 x float> %_42.i.i.i.i85416158, %872, !dbg !7980
  %876 = fadd <4 x float> %867, %875, !dbg !7984
  %877 = fmul <4 x float> %_45.i.i.i.i85716159, %872, !dbg !7988
  %878 = fadd <4 x float> %869, %877, !dbg !7992
  %879 = fmul <4 x float> %_48.i.i.i.i86016160, %872, !dbg !7996
  %880 = fadd <4 x float> %871, %879, !dbg !8000
  %881 = bitcast <4 x i32> %history.i.i777.sroa.10.018497 to <4 x float>, !dbg !8004
  %882 = fmul <4 x float> %_53.i.i.i.i86516161, %881, !dbg !8008
  %883 = fadd <4 x float> %874, %882, !dbg !8009
  %884 = fmul <4 x float> %_56.i.i.i.i86816162, %881, !dbg !8013
  %885 = fadd <4 x float> %876, %884, !dbg !8017
  %886 = fmul <4 x float> %_59.i.i.i.i87116163, %881, !dbg !8021
  %887 = fadd <4 x float> %878, %886, !dbg !8025
  %888 = fmul <4 x float> %_62.i.i.i.i87416164, %881, !dbg !8029
  %889 = fadd <4 x float> %880, %888, !dbg !8033
  %890 = bitcast <4 x i32> %history.i.i777.sroa.13.018498 to <4 x float>, !dbg !8037
  %891 = fmul <4 x float> %_67.i.i.i.i87916165, %890, !dbg !8041
  %892 = fadd <4 x float> %883, %891, !dbg !8042
  %893 = fmul <4 x float> %_70.i.i.i.i88216166, %890, !dbg !8046
  %894 = fadd <4 x float> %885, %893, !dbg !8050
  %895 = fmul <4 x float> %_73.i.i.i.i88516167, %890, !dbg !8054
  %896 = fadd <4 x float> %887, %895, !dbg !8058
  %897 = fmul <4 x float> %_76.i.i.i.i88816168, %890, !dbg !8062
  %898 = fadd <4 x float> %889, %897, !dbg !8066
  %899 = bitcast <4 x i32> %history.i.i777.sroa.16.018499 to <4 x float>, !dbg !8070
  %900 = fmul <4 x float> %_81.i.i.i.i89316169, %899, !dbg !8074
  %901 = fadd <4 x float> %892, %900, !dbg !8075
  %902 = fmul <4 x float> %_84.i.i.i.i89616170, %899, !dbg !8079
  %903 = fadd <4 x float> %894, %902, !dbg !8083
  %904 = fmul <4 x float> %_87.i.i.i.i89916171, %899, !dbg !8087
  %905 = fadd <4 x float> %896, %904, !dbg !8091
  %906 = fmul <4 x float> %_90.i.i.i.i90216172, %899, !dbg !8095
  %907 = fadd <4 x float> %898, %906, !dbg !8099
  %908 = fmul <4 x float> %_95.i.i.i.i90716173, %852, !dbg !8103
  %909 = fadd <4 x float> %901, %908, !dbg !8107
  %910 = fmul <4 x float> %_98.i.i.i.i91016174, %852, !dbg !8111
  %911 = fadd <4 x float> %903, %910, !dbg !8115
  %912 = fmul <4 x float> %_101.i.i.i.i91316175, %852, !dbg !8119
  %913 = fadd <4 x float> %905, %912, !dbg !8123
  %914 = fmul <4 x float> %_104.i.i.i.i91616176, %852, !dbg !8127
  %915 = fadd <4 x float> %907, %914, !dbg !8131
  %916 = bitcast <4 x i32> %history.i.i777.sroa.22.018501 to <4 x float>, !dbg !8135
  %917 = fmul <4 x float> %_109.i.i.i.i92116177, %916, !dbg !8139
  %918 = fadd <4 x float> %909, %917, !dbg !8140
  %919 = fmul <4 x float> %_112.i.i.i.i92416178, %916, !dbg !8144
  %920 = fadd <4 x float> %911, %919, !dbg !8148
  %921 = fmul <4 x float> %_115.i.i.i.i92716179, %916, !dbg !8152
  %922 = fadd <4 x float> %913, %921, !dbg !8156
  %923 = fmul <4 x float> %_118.i.i.i.i93016180, %916, !dbg !8160
  %924 = fadd <4 x float> %915, %923, !dbg !8164
  %925 = bitcast <4 x i32> %history.i.i777.sroa.26.018502 to <4 x float>, !dbg !8168
  %926 = fmul <4 x float> %_123.i.i.i.i93516181, %925, !dbg !8172
  %927 = fadd <4 x float> %918, %926, !dbg !8173
  %928 = fmul <4 x float> %_126.i.i.i.i93816182, %925, !dbg !8177
  %929 = fadd <4 x float> %920, %928, !dbg !8181
  %930 = fmul <4 x float> %_129.i.i.i.i94116183, %925, !dbg !8185
  %931 = fadd <4 x float> %922, %930, !dbg !8189
  %932 = fmul <4 x float> %_132.i.i.i.i94416184, %925, !dbg !8193
  %933 = fadd <4 x float> %924, %932, !dbg !8197
  %934 = bitcast <4 x i32> %history.i.i777.sroa.29.018503 to <4 x float>, !dbg !8201
  %935 = fmul <4 x float> %_137.i.i.i.i94916185, %934, !dbg !8205
  %936 = fadd <4 x float> %927, %935, !dbg !8206
  %937 = fmul <4 x float> %_140.i.i.i.i95216186, %934, !dbg !8210
  %938 = fadd <4 x float> %929, %937, !dbg !8214
  %939 = fmul <4 x float> %_143.i.i.i.i95516187, %934, !dbg !8218
  %940 = fadd <4 x float> %931, %939, !dbg !8222
  %941 = fmul <4 x float> %_146.i.i.i.i95816188, %934, !dbg !8226
  %942 = fadd <4 x float> %933, %941, !dbg !8230
  %943 = bitcast <4 x i32> %history.i.i777.sroa.32.018504 to <4 x float>, !dbg !8234
  %944 = fmul <4 x float> %_151.i.i.i.i96316189, %943, !dbg !8238
  %945 = fadd <4 x float> %936, %944, !dbg !8239
  %946 = fmul <4 x float> %_154.i.i.i.i96616190, %943, !dbg !8243
  %947 = fadd <4 x float> %938, %946, !dbg !8247
  %948 = fmul <4 x float> %_157.i.i.i.i96916191, %943, !dbg !8251
  %949 = fadd <4 x float> %940, %948, !dbg !8255
  %950 = fmul <4 x float> %_160.i.i.i.i97216192, %943, !dbg !8259
  %951 = fadd <4 x float> %942, %950, !dbg !8263
  %952 = bitcast <4 x i32> %history.i.i777.sroa.35.018505 to <4 x float>, !dbg !8267
  %953 = fmul <4 x float> %_165.i.i.i.i97716193, %952, !dbg !8271
  %954 = fadd <4 x float> %945, %953, !dbg !8272
  %955 = fmul <4 x float> %_168.i.i.i.i98016194, %952, !dbg !8276
  %956 = fadd <4 x float> %947, %955, !dbg !8280
  %957 = fmul <4 x float> %_171.i.i.i.i98316195, %952, !dbg !8284
  %958 = fadd <4 x float> %949, %957, !dbg !8288
  %959 = fmul <4 x float> %_174.i.i.i.i98616196, %952, !dbg !8292
  %960 = fadd <4 x float> %951, %959, !dbg !8296
  %961 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %954), !dbg !8300
  %962 = fcmp olt <4 x float> %961, %853, !dbg !8304
  %963 = select <4 x i1> %962, <4 x float> %853, <4 x float> %961, !dbg !8308
  %964 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %956), !dbg !8300
  %965 = fcmp olt <4 x float> %964, %963, !dbg !8304
  %966 = select <4 x i1> %965, <4 x float> %963, <4 x float> %964, !dbg !8308
  %967 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %958), !dbg !8300
  %968 = fcmp olt <4 x float> %967, %966, !dbg !8304
  %969 = select <4 x i1> %968, <4 x float> %966, <4 x float> %967, !dbg !8308
  %970 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %960), !dbg !8300
  %971 = fcmp olt <4 x float> %970, %969, !dbg !8304
  %972 = select <4 x i1> %971, <4 x float> %969, <4 x float> %970, !dbg !8308
  %973 = add nuw nsw i32 %iter.i.i773.sroa.16.018506, 1, !dbg !8309
  %data.i4.i7155 = getelementptr inbounds nuw float, ptr %peaks_right.i779, i32 %start1.i.i7150, !dbg !8310
  store <4 x float> %972, ptr %data.i4.i7155, align 4, !dbg !8313, !alias.scope !8318, !noalias !8322
  %exitcond20927.not = icmp eq i32 %973, %umax20947, !dbg !7864
  br i1 %exitcond20927.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i999, label %bb5.i.i805, !dbg !7864

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i999: ; preds = %bb5.i.i805, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7141
  %history.i.i777.sroa.0.0.lcssa = phi <4 x i32> [ %history.i.i777.sroa.0.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7141 ], [ %lanes.i5826.sroa.0.0.copyload, %bb5.i.i805 ], !dbg !8326
  %history.i.i777.sroa.7.0.lcssa = phi <4 x i32> [ %history.i.i777.sroa.7.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7141 ], [ %history.i.i777.sroa.0.018495, %bb5.i.i805 ], !dbg !8326
  %history.i.i777.sroa.10.0.lcssa = phi <4 x i32> [ %history.i.i777.sroa.10.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7141 ], [ %history.i.i777.sroa.7.018496, %bb5.i.i805 ], !dbg !8326
  %history.i.i777.sroa.13.0.lcssa = phi <4 x i32> [ %history.i.i777.sroa.13.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7141 ], [ %history.i.i777.sroa.10.018497, %bb5.i.i805 ], !dbg !8326
  %history.i.i777.sroa.16.0.lcssa = phi <4 x i32> [ %history.i.i777.sroa.16.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7141 ], [ %history.i.i777.sroa.13.018498, %bb5.i.i805 ], !dbg !8326
  %history.i.i777.sroa.19.0.lcssa = phi <4 x i32> [ %history.i.i777.sroa.19.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7141 ], [ %history.i.i777.sroa.16.018499, %bb5.i.i805 ], !dbg !8326
  %history.i.i777.sroa.22.0.lcssa = phi <4 x i32> [ %history.i.i777.sroa.22.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7141 ], [ %history.i.i777.sroa.19.018500, %bb5.i.i805 ], !dbg !8326
  %history.i.i777.sroa.26.0.lcssa = phi <4 x i32> [ %history.i.i777.sroa.26.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7141 ], [ %history.i.i777.sroa.22.018501, %bb5.i.i805 ], !dbg !8326
  %history.i.i777.sroa.29.0.lcssa = phi <4 x i32> [ %history.i.i777.sroa.29.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7141 ], [ %history.i.i777.sroa.26.018502, %bb5.i.i805 ], !dbg !8326
  %history.i.i777.sroa.32.0.lcssa = phi <4 x i32> [ %history.i.i777.sroa.32.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7141 ], [ %history.i.i777.sroa.29.018503, %bb5.i.i805 ], !dbg !8326
  %history.i.i777.sroa.35.0.lcssa = phi <4 x i32> [ %history.i.i777.sroa.35.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7141 ], [ %history.i.i777.sroa.32.018504, %bb5.i.i805 ], !dbg !8326
  %history.i.i777.sroa.38.0.lcssa = phi <4 x i32> [ %history.i.i777.sroa.38.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7141 ], [ %history.i.i777.sroa.35.018505, %bb5.i.i805 ], !dbg !8326
  store <4 x i32> %history.i.i777.sroa.0.0.lcssa, ptr %hot_right.i782, align 16, !dbg !8327
  store <4 x i32> %history.i.i777.sroa.7.0.lcssa, ptr %history.i.i777.sroa.7.0.hot_right.i782.sroa_idx, align 16, !dbg !8327
  store <4 x i32> %history.i.i777.sroa.10.0.lcssa, ptr %history.i.i777.sroa.10.0.hot_right.i782.sroa_idx, align 16, !dbg !8327
  store <4 x i32> %history.i.i777.sroa.13.0.lcssa, ptr %history.i.i777.sroa.13.0.hot_right.i782.sroa_idx, align 16, !dbg !8327
  store <4 x i32> %history.i.i777.sroa.16.0.lcssa, ptr %history.i.i777.sroa.16.0.hot_right.i782.sroa_idx, align 16, !dbg !8327
  store <4 x i32> %history.i.i777.sroa.19.0.lcssa, ptr %history.i.i777.sroa.19.0.hot_right.i782.sroa_idx, align 16, !dbg !8327
  store <4 x i32> %history.i.i777.sroa.22.0.lcssa, ptr %history.i.i777.sroa.22.0.hot_right.i782.sroa_idx, align 16, !dbg !8327
  store <4 x i32> %history.i.i777.sroa.26.0.lcssa, ptr %history.i.i777.sroa.26.0.hot_right.i782.sroa_idx, align 16, !dbg !8327
  store <4 x i32> %history.i.i777.sroa.29.0.lcssa, ptr %history.i.i777.sroa.29.0.hot_right.i782.sroa_idx, align 16, !dbg !8327
  store <4 x i32> %history.i.i777.sroa.32.0.lcssa, ptr %history.i.i777.sroa.32.0.hot_right.i782.sroa_idx, align 16, !dbg !8327
  store <4 x i32> %history.i.i777.sroa.35.0.lcssa, ptr %history.i.i777.sroa.35.0.hot_right.i782.sroa_idx, align 16, !dbg !8327
  store <4 x i32> %history.i.i777.sroa.38.0.lcssa, ptr %history.i.i777.sroa.38.0.hot_right.i782.sroa_idx, align 16, !dbg !8327
  br i1 %_2.i709418467.not, label %bb13.i.loopexit, label %bb50.i1003.lr.ph, !dbg !7345

bb50.i1003.lr.ph:                                 ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i999
  %.promoted = load <4 x float>, ptr %703, align 16
  %.promoted18622 = load <4 x float>, ptr %720, align 16
  %_8.i28.i16122.pre = load <4 x float>, ptr %_68.i, align 16, !dbg !8328
  %_9.i29.i16123.pre = load <4 x float>, ptr %_69.i1005, align 16, !dbg !8333
  %_8.i.i100916124.pre = load <4 x float>, ptr %_73.i1008, align 16, !dbg !8335
  %_9.i.i101016125.pre = load <4 x float>, ptr %_74.i, align 16, !dbg !8338
  %_91.i = load i32, ptr %693, align 4
  %_62.i86.i16133 = load <4 x float>, ptr %704, align 16
  %_62.i.i103416142 = load <4 x float>, ptr %721, align 16
  %_106.i = load i32, ptr %725, align 4
  br label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5824, !dbg !7345

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5824: ; preds = %bb50.i1003.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6465
  %974 = phi <4 x float> [ %.promoted18622, %bb50.i1003.lr.ph ], [ %1102, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6465 ]
  %975 = phi <4 x float> [ %.promoted, %bb50.i1003.lr.ph ], [ %1014, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6465 ]
  %main_cursor.sroa.0.1.i100218546 = phi i32 [ %main_cursor.sroa.0.0.i79718700, %bb50.i1003.lr.ph ], [ %spec.store.select11.i, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6465 ]
  %ring_cursor.sroa.0.1.i100118545 = phi i32 [ %ring_cursor.sroa.0.0.i79618699, %bb50.i1003.lr.ph ], [ %spec.store.select12.i, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6465 ]
  %iter1.sroa.0.0.i100018544 = phi i32 [ 0, %bb50.i1003.lr.ph ], [ %976, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6465 ]
  %976 = add nuw nsw i32 %iter1.sroa.0.0.i100018544, 1, !dbg !8340
  %_64.i = add nuw nsw i32 %iter1.sroa.0.0.i100018544, %iter.sroa.0.0.i18697, !dbg !8346
  %base.i1004 = shl i32 %_64.i, 2, !dbg !8346
  %_78.i = shl i32 %iter1.sroa.0.0.i100018544, 2, !dbg !8347
  %_159.i1013 = getelementptr inbounds nuw float, ptr %peaks_left.i780, i32 %_78.i, !dbg !8348
  %lanes.i5817.sroa.0.0.copyload = load <4 x i32>, ptr %_159.i1013, align 4, !dbg !8359, !alias.scope !8364, !noalias !8368
  %_164.i = getelementptr inbounds nuw float, ptr %peaks_right.i779, i32 %_78.i, !dbg !8372
  %lanes.i5808.sroa.0.0.copyload = load <4 x i32>, ptr %_164.i, align 4, !dbg !8382, !alias.scope !8387, !noalias !8391
  %977 = bitcast <4 x i32> %lanes.i5817.sroa.0.0.copyload to <4 x float>, !dbg !8395
  %978 = bitcast <4 x i32> %lanes.i5808.sroa.0.0.copyload to <4 x float>, !dbg !8399
  %979 = fcmp olt <4 x float> %977, %978, !dbg !8400
  %.v16126 = select <4 x i1> %979, <4 x i32> %lanes.i5808.sroa.0.0.copyload, <4 x i32> %lanes.i5817.sroa.0.0.copyload, !dbg !8401
  %980 = bitcast <4 x i32> %.v16126 to <16 x i8>, !dbg !8402
  %981 = bitcast <4 x i32> %lanes.i5817.sroa.0.0.copyload to <16 x i8>, !dbg !8406
  %_4.i7175 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %980, <16 x i8> %981, <16 x i8> %692), !dbg !8407
  %982 = bitcast <4 x i32> %lanes.i5808.sroa.0.0.copyload to <16 x i8>, !dbg !8408
  %_4.i7176 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %980, <16 x i8> %982, <16 x i8> %692), !dbg !8412
  %_165.i = icmp ugt i32 %base.i1004, %left_io.1, !dbg !8413
  br i1 %_165.i, label %bb54.i, label %bb55.i1017, !dbg !8413, !prof !787

bb55.i1017:                                       ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5824
  %_167.i = sub nuw nsw i32 %left_io.1, %base.i1004, !dbg !8417
  %_171.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %base.i1004, !dbg !8418
  %_8.i5802 = icmp samesign ugt i32 %_167.i, 3, !dbg !8423
  br i1 %_8.i5802, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5806, label %bb2.i5803, !dbg !8423, !prof !1039

bb2.i5803:                                        ; preds = %bb55.i1017
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_167.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !8428, !noalias !8429
  unreachable, !dbg !8428

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5806: ; preds = %bb55.i1017
  %lanes.i5799.sroa.0.0.copyload = load <4 x i32>, ptr %_171.i, align 4, !dbg !8433, !alias.scope !8437, !noalias !8441
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8443), !dbg !8446
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8447), !dbg !8446
  %width.i30.i = load i32, ptr %694, align 4, !dbg !8449, !alias.scope !8450, !noalias !8451, !noundef !10
  %983 = bitcast <16 x i8> %_4.i7175 to <4 x float>, !dbg !8459
  %984 = fcmp olt <4 x float> %_8.i28.i16122.pre, %983, !dbg !8464
  %985 = sext <4 x i1> %984 to <4 x i32>, !dbg !8464
  %986 = fdiv <4 x float> %_8.i28.i16122.pre, %983, !dbg !8465
  %987 = bitcast <4 x float> %986 to <16 x i8>, !dbg !8469
  %988 = bitcast <4 x i32> %985 to <16 x i8>, !dbg !8473
  %_4.i7178 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %987, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %988), !dbg !8474
  %_158.1.i35.i = load i32, ptr %695, align 4, !dbg !8475, !alias.scope !8450, !noalias !8451, !noundef !10
  %_22.i36.i = mul i32 %width.i30.i, %ring_cursor.sroa.0.1.i100118545, !dbg !8476
  %_90.i37.i = icmp ugt i32 %_22.i36.i, %_158.1.i35.i, !dbg !8477
  br i1 %_90.i37.i, label %bb34.i122.i, label %bb35.i38.i, !dbg !8477, !prof !787

bb35.i38.i:                                       ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5806
  %_93.i40.i = sub nuw i32 %_158.1.i35.i, %_22.i36.i, !dbg !8480
  %_8.i6497 = icmp samesign ugt i32 %_93.i40.i, 3, !dbg !8481
  br i1 %_8.i6497, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6500, label %bb2.i6498, !dbg !8481, !prof !1039

bb2.i6498:                                        ; preds = %bb35.i38.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_93.i40.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !8486, !noalias !8487
  unreachable, !dbg !8486

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6500: ; preds = %bb35.i38.i
  %_158.0.i39.i = load ptr, ptr %696, align 4, !dbg !8475, !alias.scope !8450, !noalias !8451, !nonnull !10, !noundef !10
  %_97.i41.i = getelementptr inbounds nuw float, ptr %_158.0.i39.i, i32 %_22.i36.i, !dbg !8491
  store <16 x i8> %_4.i7178, ptr %_97.i41.i, align 4, !dbg !8493, !alias.scope !8497, !noalias !8501
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8503), !dbg !8506
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8507), !dbg !8506
  %width.i2334 = load i32, ptr %694, align 4, !dbg !8509, !alias.scope !8503, !noalias !8511, !noundef !10
  %989 = icmp eq i32 %width.i2334, 0, !dbg !8512
  br i1 %989, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2440, label %bb29.i2340.lr.ph, !dbg !8512

bb29.i2340.lr.ph:                                 ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6500
  %_126.1.i2345 = load i32, ptr %62, align 4, !alias.scope !8503, !noalias !8511, !noundef !10
  %_126.0.i2349 = load ptr, ptr %61, align 4, !nonnull !10
  %990 = add i32 %ring_cursor.sroa.0.1.i100118545, 1
  %_21.not.i2354 = icmp ult i32 %990, %_91.i
  %991 = select i1 %_21.not.i2354, i32 0, i32 %_91.i
  %start1.sroa.0.0.i2355 = sub nuw i32 %990, %991
  %_128.1.i2358 = load i32, ptr %695, align 4
  %_128.0.i2362 = load ptr, ptr %696, align 4, !nonnull !10
  %_130.1.i2363 = load i32, ptr %697, align 4
  %_130.0.i2367 = load ptr, ptr %698, align 4, !nonnull !10
  %_132.1.i2370 = load i32, ptr %699, align 4
  %_132.0.i2374 = load ptr, ptr %700, align 4, !nonnull !10
  %_43.i2387 = mul i32 %width.i2334, %start1.sroa.0.0.i2355
  br label %bb29.i2340, !dbg !8512

bb29.i2340:                                       ; preds = %bb29.i2340.lr.ph, %bb28.i2402
  %iter.sroa.0.0.idx.i233818525 = phi i32 [ 0, %bb29.i2340.lr.ph ], [ %iter.sroa.0.0.add.i2343, %bb28.i2402 ]
  %iter.sroa.4.0.i233718524 = phi i32 [ 0, %bb29.i2340.lr.ph ], [ %_102.0.i2344, %bb28.i2402 ]
  %iter.sroa.7.0.i233618523 = phi i32 [ %width.i2334, %bb29.i2340.lr.ph ], [ %992, %bb28.i2402 ]
  %iter.sroa.0.0.ptr.i233918526 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 %iter.sroa.0.0.idx.i233818525, !dbg !8514
  %992 = add i32 %iter.sroa.7.0.i233618523, -1, !dbg !8514
  %_109.i2341 = icmp eq i32 %iter.sroa.0.0.idx.i233818525, 32, !dbg !8515
  br i1 %_109.i2341, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2440, label %bb33.i2342, !dbg !8519

bb33.i2342:                                       ; preds = %bb29.i2340
  %iter.sroa.0.0.add.i2343 = add nuw nsw i32 %iter.sroa.0.0.idx.i233818525, 4, !dbg !8520
  %_102.0.i2344 = add nuw nsw i32 %iter.sroa.4.0.i233718524, 1, !dbg !8522
  %exitcond20930.not = icmp eq i32 %iter.sroa.4.0.i233718524, %_126.1.i2345, !dbg !8523
  br i1 %exitcond20930.not, label %panic.i2347, label %bb2.i2348, !dbg !8523

bb2.i2348:                                        ; preds = %bb33.i2342
  %993 = getelementptr inbounds nuw %LaneShape, ptr %_126.0.i2349, i32 %iter.sroa.4.0.i233718524, !dbg !8523
  %shape.i2350 = load i32, ptr %993, align 4, !dbg !8523, !noalias !8524, !noundef !10
  %994 = getelementptr inbounds nuw i8, ptr %993, i32 4, !dbg !8523
  %shape3.i2351 = load i32, ptr %994, align 4, !dbg !8523, !noalias !8524, !noundef !10
  %995 = add i32 %shape3.i2351, %ring_cursor.sroa.0.1.i100118545, !dbg !8525
  %_18.not.i2352 = icmp ult i32 %995, %_91.i, !dbg !8526
  %996 = select i1 %_18.not.i2352, i32 0, i32 %_91.i, !dbg !8526
  %spec.select.i2353 = sub nuw i32 %995, %996, !dbg !8526
  %_25.i2356 = mul i32 %spec.select.i2353, %width.i2334, !dbg !8527
  %_24.i2357 = add i32 %_25.i2356, %iter.sroa.4.0.i233718524, !dbg !8527
  %_28.i2359 = icmp ult i32 %_24.i2357, %_128.1.i2358, !dbg !8528
  br i1 %_28.i2359, label %bb9.i2361, label %panic5.i2360, !dbg !8528

panic.i2347:                                      ; preds = %bb33.i2342
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_126.1.i2345, i32 noundef %_126.1.i2345, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_12856b5f9033764dbf23e04eb77c5c46) #32, !dbg !8523, !noalias !8524
  unreachable, !dbg !8523

bb9.i2361:                                        ; preds = %bb2.i2348
  %997 = getelementptr inbounds nuw float, ptr %_128.0.i2362, i32 %_24.i2357, !dbg !8528
  %998 = load float, ptr %997, align 4, !dbg !8528, !noalias !8524, !noundef !10
  %exitcond20931.not = icmp eq i32 %iter.sroa.4.0.i233718524, %_130.1.i2363, !dbg !8529
  br i1 %exitcond20931.not, label %panic6.i2365, label %bb10.i2366, !dbg !8529

panic5.i2360:                                     ; preds = %bb2.i2348
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_24.i2357, i32 noundef %_128.1.i2358, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70ebf0cf1c90c6161926611ccf249a32) #32, !dbg !8528, !noalias !8524
  unreachable, !dbg !8528

bb10.i2366:                                       ; preds = %bb9.i2361
  %999 = getelementptr inbounds nuw i32, ptr %_130.0.i2367, i32 %iter.sroa.4.0.i233718524, !dbg !8529
  %_30.i2368 = load i32, ptr %999, align 4, !dbg !8529, !noalias !8524, !noundef !10
  %1000 = icmp eq i32 %_30.i2368, 0, !dbg !8530
  br i1 %1000, label %bb14.i2377, label %bb12.i2369, !dbg !8530

panic6.i2365:                                     ; preds = %bb9.i2361
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_130.1.i2363, i32 noundef %_130.1.i2363, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_023d591aaad7f5cf482ac80a9401eca1) #32, !dbg !8529, !noalias !8524
  unreachable, !dbg !8529

bb12.i2369:                                       ; preds = %bb10.i2366
  %_35.i2371 = icmp ult i32 %iter.sroa.4.0.i233718524, %_132.1.i2370, !dbg !8531
  br i1 %_35.i2371, label %bb13.i2373, label %panic7.i2372, !dbg !8531

bb14.i2377:                                       ; preds = %bb34.i2438, %bb13.i2373, %bb10.i2366
  %newest.sroa.0.0.i2378 = phi float [ %998, %bb10.i2366 ], [ %_33.i2375, %bb34.i2438 ], [ %998, %bb13.i2373 ], !dbg !8532
  %exitcond20932.not = icmp eq i32 %iter.sroa.4.0.i233718524, %_132.1.i2370, !dbg !8533
  br i1 %exitcond20932.not, label %panic8.i2381, label %bb15.i2382, !dbg !8533

bb13.i2373:                                       ; preds = %bb12.i2369
  %1001 = getelementptr inbounds nuw float, ptr %_132.0.i2374, i32 %iter.sroa.4.0.i233718524, !dbg !8531
  %_33.i2375 = load float, ptr %1001, align 4, !dbg !8531, !noalias !8524, !noundef !10
  %_116.i2376 = fcmp olt float %_33.i2375, %998, !dbg !8534
  br i1 %_116.i2376, label %bb34.i2438, label %bb14.i2377, !dbg !8534

panic7.i2372:                                     ; preds = %bb12.i2369
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %iter.sroa.4.0.i233718524, i32 noundef %_132.1.i2370, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a228cac3279aae3a34886f59f51fbe9e) #32, !dbg !8531, !noalias !8524
  unreachable, !dbg !8531

bb34.i2438:                                       ; preds = %bb13.i2373
  br label %bb14.i2377, !dbg !8536

bb15.i2382:                                       ; preds = %bb14.i2377
  %1002 = getelementptr inbounds nuw float, ptr %_132.0.i2374, i32 %iter.sroa.4.0.i233718524, !dbg !8533
  store float %newest.sroa.0.0.i2378, ptr %1002, align 4, !dbg !8533, !noalias !8524
  %_40.i2384 = add i32 %_30.i2368, 1, !dbg !8537
  %complete.i2385 = icmp eq i32 %_40.i2384, %shape.i2350, !dbg !8537
  br i1 %complete.i2385, label %bb19.i2407, label %bb17.i2386, !dbg !8538

panic8.i2381:                                     ; preds = %bb14.i2377
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_132.1.i2370, i32 noundef %_132.1.i2370, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1fa5d88c1a2cc292a2767c48606a38fe) #32, !dbg !8533, !noalias !8524
  unreachable, !dbg !8533

bb17.i2386:                                       ; preds = %bb15.i2382
  %_42.i2388 = add i32 %iter.sroa.4.0.i233718524, %_43.i2387, !dbg !8539
  %_45.i2390 = icmp ult i32 %_42.i2388, %_128.1.i2358, !dbg !8540
  br i1 %_45.i2390, label %bb27.i2400, label %panic9.i2391, !dbg !8540

panic9.i2391:                                     ; preds = %bb17.i2386
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_42.i2388, i32 noundef %_128.1.i2358, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70d40ae2302f3ea19fa0c4a65fe82786) #32, !dbg !8540, !noalias !8524
  unreachable, !dbg !8540

bb27.i2400:                                       ; preds = %bb17.i2386
  %1003 = getelementptr inbounds nuw float, ptr %_128.0.i2362, i32 %_42.i2388, !dbg !8540
  %_41.i2394 = load float, ptr %1003, align 4, !dbg !8540, !noalias !8524, !noundef !10
  %_117.i2395 = fcmp olt float %_41.i2394, %newest.sroa.0.0.i2378, !dbg !8541
  %newest.sroa.0.1.i2396 = select i1 %_117.i2395, float %_41.i2394, float %newest.sroa.0.0.i2378, !dbg !8541
  store float %newest.sroa.0.1.i2396, ptr %iter.sroa.0.0.ptr.i233918526, align 4, !dbg !8543, !alias.scope !8507, !noalias !8544
  br label %bb28.i2402, !dbg !8545

bb28.i2402:                                       ; preds = %bb22.i2435, %bb19.i2407, %bb27.i2400
  %storemerge16127 = phi i32 [ %_40.i2384, %bb27.i2400 ], [ 0, %bb19.i2407 ], [ 0, %bb22.i2435 ], !dbg !8546
  store i32 %storemerge16127, ptr %999, align 4, !dbg !8546, !noalias !8524
  %1004 = icmp eq i32 %992, 0, !dbg !8512
  br i1 %1004, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2440, label %bb29.i2340, !dbg !8512

bb19.i2407:                                       ; preds = %bb15.i2382
  store float %newest.sroa.0.0.i2378, ptr %iter.sroa.0.0.ptr.i233918526, align 4, !dbg !8543, !alias.scope !8507, !noalias !8544
  %_118.i241318519.not = icmp eq i32 %shape.i2350, 0, !dbg !8547
  br i1 %_118.i241318519.not, label %bb28.i2402, label %bb40.i2420.preheader, !dbg !8551

bb40.i2420.preheader:                             ; preds = %bb19.i2407
  %1005 = load float, ptr %997, align 4, !dbg !8552, !noalias !8524, !noundef !10
  br label %bb40.i2420, !dbg !8553

bb40.i2420:                                       ; preds = %bb40.i2420.preheader, %bb22.i2435
  %iter2.sroa.0.0.i241218522 = phi i32 [ %_119.i2421, %bb22.i2435 ], [ 0, %bb40.i2420.preheader ]
  %suffix.sroa.0.0.i241118521 = phi float [ %suffix.sroa.0.1.i2431, %bb22.i2435 ], [ %1005, %bb40.i2420.preheader ]
  %end.sroa.0.1.i241018520 = phi i32 [ %1008, %bb22.i2435 ], [ %spec.select.i2353, %bb40.i2420.preheader ]
  %_54.i2422 = mul i32 %end.sroa.0.1.i241018520, %width.i2334, !dbg !8554
  %_53.i2423 = add i32 %_54.i2422, %iter.sroa.4.0.i233718524, !dbg !8554
  %_57.i2425 = icmp ult i32 %_53.i2423, %_128.1.i2358, !dbg !8553
  br i1 %_57.i2425, label %bb22.i2435, label %panic13.i2426, !dbg !8553

panic13.i2426:                                    ; preds = %bb40.i2420
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_53.i2423, i32 noundef %_128.1.i2358, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a271bbb7fd83c3605ea58a06b7065fa4) #32, !dbg !8553, !noalias !8524
  unreachable, !dbg !8553

bb22.i2435:                                       ; preds = %bb40.i2420
  %_119.i2421 = add nuw i32 %iter2.sroa.0.0.i241218522, 1, !dbg !8555
  %1006 = getelementptr inbounds nuw float, ptr %_128.0.i2362, i32 %_53.i2423, !dbg !8553
  %_52.i2429 = load float, ptr %1006, align 4, !dbg !8553, !noalias !8524, !noundef !10
  %_121.i2430 = fcmp olt float %suffix.sroa.0.0.i241118521, %_52.i2429, !dbg !8558
  %suffix.sroa.0.1.i2431 = select i1 %_121.i2430, float %suffix.sroa.0.0.i241118521, float %_52.i2429, !dbg !8558
  store float %suffix.sroa.0.1.i2431, ptr %1006, align 4, !dbg !8560, !noalias !8524
  %1007 = icmp eq i32 %end.sroa.0.1.i241018520, 0, !dbg !8561
  %spec.store.select.i2437 = select i1 %1007, i32 %_91.i, i32 %end.sroa.0.1.i241018520, !dbg !8561
  %1008 = add i32 %spec.store.select.i2437, -1, !dbg !8562
  %exitcond20929.not = icmp eq i32 %_119.i2421, %shape.i2350, !dbg !8547
  br i1 %exitcond20929.not, label %bb28.i2402, label %bb40.i2420, !dbg !8551

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2440: ; preds = %bb29.i2340, %bb28.i2402, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6500
  %lanes.i5792.sroa.0.0.copyload = load <4 x float>, ptr %scratch.i, align 4, !dbg !8563, !alias.scope !8568, !noalias !8572
  %1009 = fmul <4 x float> %lanes.i5792.sroa.0.0.copyload, splat (float 1.638400e+04), !dbg !8576
  %1010 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %1009), !dbg !8580
  %1011 = fmul <4 x float> %1010, splat (float 0x3F10000000000000), !dbg !8584
  %1012 = icmp eq i32 %width.i30.i, 0, !dbg !8588
  %_163.1.i79.i.pre = load i32, ptr %701, align 4, !dbg !8590, !alias.scope !8450, !noalias !8451
  br i1 %1012, label %bb53.i74.i, label %bb36.i53.i.lr.ph, !dbg !8588

bb36.i53.i.lr.ph:                                 ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2440
  %_159.1.i58.i = load i32, ptr %62, align 4, !alias.scope !8450, !noalias !8451, !noundef !10
  %_159.0.i62.i = load ptr, ptr %61, align 4, !nonnull !10
  %_161.0.i72.i = load ptr, ptr %702, align 4, !nonnull !10
  %exitcond20935.not = icmp eq i32 %_159.1.i58.i, 0, !dbg !8591
  br i1 %exitcond20935.not, label %panic.i60.i, label %bb14.i61.i, !dbg !8591

bb34.i122.i:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5806
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i36.i, i32 noundef %_158.1.i35.i, i32 noundef %_158.1.i35.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_db7c42041d7aaaba4839906f37294547) #32, !dbg !8592, !noalias !8593
  unreachable, !dbg !8592

bb53.i74.i.loopexit:                              ; preds = %bb18.i71.i.7, %bb18.i71.i.6, %bb18.i71.i.5, %bb18.i71.i.4, %bb18.i71.i.3, %bb18.i71.i.2, %bb18.i71.i.1, %bb18.i71.i
  %lanes.i5785.sroa.0.0.copyload.pre = load <4 x float>, ptr %scratch.i, align 4, !dbg !8594, !alias.scope !8599, !noalias !8603
  br label %bb53.i74.i, !dbg !8607

bb53.i74.i:                                       ; preds = %bb53.i74.i.loopexit, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2440
  %lanes.i5785.sroa.0.0.copyload = phi <4 x float> [ %lanes.i5785.sroa.0.0.copyload.pre, %bb53.i74.i.loopexit ], [ %lanes.i5792.sroa.0.0.copyload, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2440 ], !dbg !8594
  %1013 = fadd <4 x float> %1011, %975, !dbg !8608
  %1014 = fsub <4 x float> %1013, %lanes.i5785.sroa.0.0.copyload, !dbg !8612
  %_123.i80.i = icmp ugt i32 %_22.i36.i, %_163.1.i79.i.pre, !dbg !8616
  br i1 %_123.i80.i, label %bb41.i121.i, label %bb42.i81.i, !dbg !8616, !prof !787

bb14.i61.i:                                       ; preds = %bb36.i53.i.lr.ph
  %1015 = getelementptr inbounds nuw i8, ptr %_159.0.i62.i, i32 8, !dbg !8591
  %_42.i63.i = load i32, ptr %1015, align 4, !dbg !8591, !noalias !8619, !noundef !10
  %1016 = add i32 %_42.i63.i, %ring_cursor.sroa.0.1.i100118545, !dbg !8620
  %_45.not.i64.i = icmp ult i32 %1016, %_91.i, !dbg !8621
  %1017 = select i1 %_45.not.i64.i, i32 0, i32 %_91.i, !dbg !8621
  %spec.select.i65.i = sub nuw i32 %1016, %1017, !dbg !8621
  %_49.i66.i = mul i32 %spec.select.i65.i, %width.i30.i, !dbg !8622
  %_51.i69.i = icmp ult i32 %_49.i66.i, %_163.1.i79.i.pre, !dbg !8623
  br i1 %_51.i69.i, label %bb18.i71.i, label %panic1.i70.i, !dbg !8623

panic.i60.i:                                      ; preds = %bb36.i53.i.7, %bb36.i53.i.6, %bb36.i53.i.5, %bb36.i53.i.4, %bb36.i53.i.3, %bb36.i53.i.2, %bb36.i53.i.1, %bb36.i53.i.lr.ph
  %_159.1.i58.i.lcssa.ph = phi i32 [ 7, %bb36.i53.i.7 ], [ 6, %bb36.i53.i.6 ], [ 5, %bb36.i53.i.5 ], [ 4, %bb36.i53.i.4 ], [ 3, %bb36.i53.i.3 ], [ 2, %bb36.i53.i.2 ], [ 1, %bb36.i53.i.1 ], [ 0, %bb36.i53.i.lr.ph ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_159.1.i58.i.lcssa.ph, i32 noundef %_159.1.i58.i.lcssa.ph, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_33746731a929bad63709ddedbca82f5f) #32, !dbg !8591, !noalias !8619
  unreachable, !dbg !8591

bb18.i71.i:                                       ; preds = %bb14.i61.i
  %1018 = getelementptr inbounds nuw float, ptr %_161.0.i72.i, i32 %_49.i66.i, !dbg !8623
  %_47.i73.i = load float, ptr %1018, align 4, !dbg !8623, !noalias !8619, !noundef !10
  store float %_47.i73.i, ptr %scratch.i, align 4, !dbg !8624, !alias.scope !8447, !noalias !8625
  %1019 = icmp eq i32 %width.i30.i, 1, !dbg !8588
  br i1 %1019, label %bb53.i74.i.loopexit, label %bb36.i53.i.1, !dbg !8588

bb36.i53.i.1:                                     ; preds = %bb18.i71.i
  %exitcond20935.1.not = icmp eq i32 %_159.1.i58.i, 1, !dbg !8591
  br i1 %exitcond20935.1.not, label %panic.i60.i, label %bb14.i61.i.1, !dbg !8591

bb14.i61.i.1:                                     ; preds = %bb36.i53.i.1
  %1020 = getelementptr inbounds nuw i8, ptr %_159.0.i62.i, i32 20, !dbg !8591
  %_42.i63.i.1 = load i32, ptr %1020, align 4, !dbg !8591, !noalias !8619, !noundef !10
  %1021 = add i32 %_42.i63.i.1, %ring_cursor.sroa.0.1.i100118545, !dbg !8620
  %_45.not.i64.i.1 = icmp ult i32 %1021, %_91.i, !dbg !8621
  %1022 = select i1 %_45.not.i64.i.1, i32 0, i32 %_91.i, !dbg !8621
  %spec.select.i65.i.1 = sub nuw i32 %1021, %1022, !dbg !8621
  %_49.i66.i.1 = mul i32 %spec.select.i65.i.1, %width.i30.i, !dbg !8622
  %_48.i67.i.1 = add i32 %_49.i66.i.1, 1, !dbg !8622
  %_51.i69.i.1 = icmp ult i32 %_48.i67.i.1, %_163.1.i79.i.pre, !dbg !8623
  br i1 %_51.i69.i.1, label %bb18.i71.i.1, label %panic1.i70.i, !dbg !8623

bb18.i71.i.1:                                     ; preds = %bb14.i61.i.1
  %1023 = getelementptr inbounds nuw float, ptr %_161.0.i72.i, i32 %_48.i67.i.1, !dbg !8623
  %_47.i73.i.1 = load float, ptr %1023, align 4, !dbg !8623, !noalias !8619, !noundef !10
  store float %_47.i73.i.1, ptr %iter.sroa.0.0.ptr.i52.i18530.1, align 4, !dbg !8624, !alias.scope !8447, !noalias !8625
  %1024 = icmp eq i32 %width.i30.i, 2, !dbg !8588
  br i1 %1024, label %bb53.i74.i.loopexit, label %bb36.i53.i.2, !dbg !8588

bb36.i53.i.2:                                     ; preds = %bb18.i71.i.1
  %exitcond20935.2.not = icmp eq i32 %_159.1.i58.i, 2, !dbg !8591
  br i1 %exitcond20935.2.not, label %panic.i60.i, label %bb14.i61.i.2, !dbg !8591

bb14.i61.i.2:                                     ; preds = %bb36.i53.i.2
  %1025 = getelementptr inbounds nuw i8, ptr %_159.0.i62.i, i32 32, !dbg !8591
  %_42.i63.i.2 = load i32, ptr %1025, align 4, !dbg !8591, !noalias !8619, !noundef !10
  %1026 = add i32 %_42.i63.i.2, %ring_cursor.sroa.0.1.i100118545, !dbg !8620
  %_45.not.i64.i.2 = icmp ult i32 %1026, %_91.i, !dbg !8621
  %1027 = select i1 %_45.not.i64.i.2, i32 0, i32 %_91.i, !dbg !8621
  %spec.select.i65.i.2 = sub nuw i32 %1026, %1027, !dbg !8621
  %_49.i66.i.2 = mul i32 %spec.select.i65.i.2, %width.i30.i, !dbg !8622
  %_48.i67.i.2 = add i32 %_49.i66.i.2, 2, !dbg !8622
  %_51.i69.i.2 = icmp ult i32 %_48.i67.i.2, %_163.1.i79.i.pre, !dbg !8623
  br i1 %_51.i69.i.2, label %bb18.i71.i.2, label %panic1.i70.i, !dbg !8623

bb18.i71.i.2:                                     ; preds = %bb14.i61.i.2
  %1028 = getelementptr inbounds nuw float, ptr %_161.0.i72.i, i32 %_48.i67.i.2, !dbg !8623
  %_47.i73.i.2 = load float, ptr %1028, align 4, !dbg !8623, !noalias !8619, !noundef !10
  store float %_47.i73.i.2, ptr %iter.sroa.0.0.ptr.i52.i18530.2, align 4, !dbg !8624, !alias.scope !8447, !noalias !8625
  %1029 = icmp eq i32 %width.i30.i, 3, !dbg !8588
  br i1 %1029, label %bb53.i74.i.loopexit, label %bb36.i53.i.3, !dbg !8588

bb36.i53.i.3:                                     ; preds = %bb18.i71.i.2
  %exitcond20935.3.not = icmp eq i32 %_159.1.i58.i, 3, !dbg !8591
  br i1 %exitcond20935.3.not, label %panic.i60.i, label %bb14.i61.i.3, !dbg !8591

bb14.i61.i.3:                                     ; preds = %bb36.i53.i.3
  %1030 = getelementptr inbounds nuw i8, ptr %_159.0.i62.i, i32 44, !dbg !8591
  %_42.i63.i.3 = load i32, ptr %1030, align 4, !dbg !8591, !noalias !8619, !noundef !10
  %1031 = add i32 %_42.i63.i.3, %ring_cursor.sroa.0.1.i100118545, !dbg !8620
  %_45.not.i64.i.3 = icmp ult i32 %1031, %_91.i, !dbg !8621
  %1032 = select i1 %_45.not.i64.i.3, i32 0, i32 %_91.i, !dbg !8621
  %spec.select.i65.i.3 = sub nuw i32 %1031, %1032, !dbg !8621
  %_49.i66.i.3 = mul i32 %spec.select.i65.i.3, %width.i30.i, !dbg !8622
  %_48.i67.i.3 = add i32 %_49.i66.i.3, 3, !dbg !8622
  %_51.i69.i.3 = icmp ult i32 %_48.i67.i.3, %_163.1.i79.i.pre, !dbg !8623
  br i1 %_51.i69.i.3, label %bb18.i71.i.3, label %panic1.i70.i, !dbg !8623

bb18.i71.i.3:                                     ; preds = %bb14.i61.i.3
  %1033 = getelementptr inbounds nuw float, ptr %_161.0.i72.i, i32 %_48.i67.i.3, !dbg !8623
  %_47.i73.i.3 = load float, ptr %1033, align 4, !dbg !8623, !noalias !8619, !noundef !10
  store float %_47.i73.i.3, ptr %iter.sroa.0.0.ptr.i52.i18530.3, align 4, !dbg !8624, !alias.scope !8447, !noalias !8625
  %1034 = icmp eq i32 %width.i30.i, 4, !dbg !8588
  br i1 %1034, label %bb53.i74.i.loopexit, label %bb36.i53.i.4, !dbg !8588

bb36.i53.i.4:                                     ; preds = %bb18.i71.i.3
  %exitcond20935.4.not = icmp eq i32 %_159.1.i58.i, 4, !dbg !8591
  br i1 %exitcond20935.4.not, label %panic.i60.i, label %bb14.i61.i.4, !dbg !8591

bb14.i61.i.4:                                     ; preds = %bb36.i53.i.4
  %1035 = getelementptr inbounds nuw i8, ptr %_159.0.i62.i, i32 56, !dbg !8591
  %_42.i63.i.4 = load i32, ptr %1035, align 4, !dbg !8591, !noalias !8619, !noundef !10
  %1036 = add i32 %_42.i63.i.4, %ring_cursor.sroa.0.1.i100118545, !dbg !8620
  %_45.not.i64.i.4 = icmp ult i32 %1036, %_91.i, !dbg !8621
  %1037 = select i1 %_45.not.i64.i.4, i32 0, i32 %_91.i, !dbg !8621
  %spec.select.i65.i.4 = sub nuw i32 %1036, %1037, !dbg !8621
  %_49.i66.i.4 = mul i32 %spec.select.i65.i.4, %width.i30.i, !dbg !8622
  %_48.i67.i.4 = add i32 %_49.i66.i.4, 4, !dbg !8622
  %_51.i69.i.4 = icmp ult i32 %_48.i67.i.4, %_163.1.i79.i.pre, !dbg !8623
  br i1 %_51.i69.i.4, label %bb18.i71.i.4, label %panic1.i70.i, !dbg !8623

bb18.i71.i.4:                                     ; preds = %bb14.i61.i.4
  %1038 = getelementptr inbounds nuw float, ptr %_161.0.i72.i, i32 %_48.i67.i.4, !dbg !8623
  %_47.i73.i.4 = load float, ptr %1038, align 4, !dbg !8623, !noalias !8619, !noundef !10
  store float %_47.i73.i.4, ptr %iter.sroa.0.0.ptr.i52.i18530.4, align 4, !dbg !8624, !alias.scope !8447, !noalias !8625
  %1039 = icmp eq i32 %width.i30.i, 5, !dbg !8588
  br i1 %1039, label %bb53.i74.i.loopexit, label %bb36.i53.i.5, !dbg !8588

bb36.i53.i.5:                                     ; preds = %bb18.i71.i.4
  %exitcond20935.5.not = icmp eq i32 %_159.1.i58.i, 5, !dbg !8591
  br i1 %exitcond20935.5.not, label %panic.i60.i, label %bb14.i61.i.5, !dbg !8591

bb14.i61.i.5:                                     ; preds = %bb36.i53.i.5
  %1040 = getelementptr inbounds nuw i8, ptr %_159.0.i62.i, i32 68, !dbg !8591
  %_42.i63.i.5 = load i32, ptr %1040, align 4, !dbg !8591, !noalias !8619, !noundef !10
  %1041 = add i32 %_42.i63.i.5, %ring_cursor.sroa.0.1.i100118545, !dbg !8620
  %_45.not.i64.i.5 = icmp ult i32 %1041, %_91.i, !dbg !8621
  %1042 = select i1 %_45.not.i64.i.5, i32 0, i32 %_91.i, !dbg !8621
  %spec.select.i65.i.5 = sub nuw i32 %1041, %1042, !dbg !8621
  %_49.i66.i.5 = mul i32 %spec.select.i65.i.5, %width.i30.i, !dbg !8622
  %_48.i67.i.5 = add i32 %_49.i66.i.5, 5, !dbg !8622
  %_51.i69.i.5 = icmp ult i32 %_48.i67.i.5, %_163.1.i79.i.pre, !dbg !8623
  br i1 %_51.i69.i.5, label %bb18.i71.i.5, label %panic1.i70.i, !dbg !8623

bb18.i71.i.5:                                     ; preds = %bb14.i61.i.5
  %1043 = getelementptr inbounds nuw float, ptr %_161.0.i72.i, i32 %_48.i67.i.5, !dbg !8623
  %_47.i73.i.5 = load float, ptr %1043, align 4, !dbg !8623, !noalias !8619, !noundef !10
  store float %_47.i73.i.5, ptr %iter.sroa.0.0.ptr.i52.i18530.5, align 4, !dbg !8624, !alias.scope !8447, !noalias !8625
  %1044 = icmp eq i32 %width.i30.i, 6, !dbg !8588
  br i1 %1044, label %bb53.i74.i.loopexit, label %bb36.i53.i.6, !dbg !8588

bb36.i53.i.6:                                     ; preds = %bb18.i71.i.5
  %exitcond20935.6.not = icmp eq i32 %_159.1.i58.i, 6, !dbg !8591
  br i1 %exitcond20935.6.not, label %panic.i60.i, label %bb14.i61.i.6, !dbg !8591

bb14.i61.i.6:                                     ; preds = %bb36.i53.i.6
  %1045 = getelementptr inbounds nuw i8, ptr %_159.0.i62.i, i32 80, !dbg !8591
  %_42.i63.i.6 = load i32, ptr %1045, align 4, !dbg !8591, !noalias !8619, !noundef !10
  %1046 = add i32 %_42.i63.i.6, %ring_cursor.sroa.0.1.i100118545, !dbg !8620
  %_45.not.i64.i.6 = icmp ult i32 %1046, %_91.i, !dbg !8621
  %1047 = select i1 %_45.not.i64.i.6, i32 0, i32 %_91.i, !dbg !8621
  %spec.select.i65.i.6 = sub nuw i32 %1046, %1047, !dbg !8621
  %_49.i66.i.6 = mul i32 %spec.select.i65.i.6, %width.i30.i, !dbg !8622
  %_48.i67.i.6 = add i32 %_49.i66.i.6, 6, !dbg !8622
  %_51.i69.i.6 = icmp ult i32 %_48.i67.i.6, %_163.1.i79.i.pre, !dbg !8623
  br i1 %_51.i69.i.6, label %bb18.i71.i.6, label %panic1.i70.i, !dbg !8623

bb18.i71.i.6:                                     ; preds = %bb14.i61.i.6
  %1048 = getelementptr inbounds nuw float, ptr %_161.0.i72.i, i32 %_48.i67.i.6, !dbg !8623
  %_47.i73.i.6 = load float, ptr %1048, align 4, !dbg !8623, !noalias !8619, !noundef !10
  store float %_47.i73.i.6, ptr %iter.sroa.0.0.ptr.i52.i18530.6, align 4, !dbg !8624, !alias.scope !8447, !noalias !8625
  %1049 = icmp eq i32 %width.i30.i, 7, !dbg !8588
  br i1 %1049, label %bb53.i74.i.loopexit, label %bb36.i53.i.7, !dbg !8588

bb36.i53.i.7:                                     ; preds = %bb18.i71.i.6
  %exitcond20935.7.not = icmp eq i32 %_159.1.i58.i, 7, !dbg !8591
  br i1 %exitcond20935.7.not, label %panic.i60.i, label %bb14.i61.i.7, !dbg !8591

bb14.i61.i.7:                                     ; preds = %bb36.i53.i.7
  %1050 = getelementptr inbounds nuw i8, ptr %_159.0.i62.i, i32 92, !dbg !8591
  %_42.i63.i.7 = load i32, ptr %1050, align 4, !dbg !8591, !noalias !8619, !noundef !10
  %1051 = add i32 %_42.i63.i.7, %ring_cursor.sroa.0.1.i100118545, !dbg !8620
  %_45.not.i64.i.7 = icmp ult i32 %1051, %_91.i, !dbg !8621
  %1052 = select i1 %_45.not.i64.i.7, i32 0, i32 %_91.i, !dbg !8621
  %spec.select.i65.i.7 = sub nuw i32 %1051, %1052, !dbg !8621
  %_49.i66.i.7 = mul i32 %spec.select.i65.i.7, %width.i30.i, !dbg !8622
  %_48.i67.i.7 = add i32 %_49.i66.i.7, 7, !dbg !8622
  %_51.i69.i.7 = icmp ult i32 %_48.i67.i.7, %_163.1.i79.i.pre, !dbg !8623
  br i1 %_51.i69.i.7, label %bb18.i71.i.7, label %panic1.i70.i, !dbg !8623

bb18.i71.i.7:                                     ; preds = %bb14.i61.i.7
  %1053 = getelementptr inbounds nuw float, ptr %_161.0.i72.i, i32 %_48.i67.i.7, !dbg !8623
  %_47.i73.i.7 = load float, ptr %1053, align 4, !dbg !8623, !noalias !8619, !noundef !10
  store float %_47.i73.i.7, ptr %iter.sroa.0.0.ptr.i52.i18530.7, align 4, !dbg !8624, !alias.scope !8447, !noalias !8625
  br label %bb53.i74.i.loopexit, !dbg !8588

panic1.i70.i:                                     ; preds = %bb14.i61.i.7, %bb14.i61.i.6, %bb14.i61.i.5, %bb14.i61.i.4, %bb14.i61.i.3, %bb14.i61.i.2, %bb14.i61.i.1, %bb14.i61.i
  %_48.i67.i.lcssa.ph = phi i32 [ %_48.i67.i.7, %bb14.i61.i.7 ], [ %_48.i67.i.6, %bb14.i61.i.6 ], [ %_48.i67.i.5, %bb14.i61.i.5 ], [ %_48.i67.i.4, %bb14.i61.i.4 ], [ %_48.i67.i.3, %bb14.i61.i.3 ], [ %_48.i67.i.2, %bb14.i61.i.2 ], [ %_48.i67.i.1, %bb14.i61.i.1 ], [ %_49.i66.i, %bb14.i61.i ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_48.i67.i.lcssa.ph, i32 noundef %_163.1.i79.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_0e76a26fbb751dfb8cf8a069b5b0d39a) #32, !dbg !8623, !noalias !8619
  unreachable, !dbg !8623

bb42.i81.i:                                       ; preds = %bb53.i74.i
  %_126.i83.i = sub nuw i32 %_163.1.i79.i.pre, %_22.i36.i, !dbg !8626
  %_8.i6492 = icmp samesign ugt i32 %_126.i83.i, 3, !dbg !8627
  br i1 %_8.i6492, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6495, label %bb2.i6493, !dbg !8627, !prof !1039

bb2.i6493:                                        ; preds = %bb42.i81.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_126.i83.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !8632, !noalias !8633
  unreachable, !dbg !8632

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6495: ; preds = %bb42.i81.i
  %_163.0.i82.i = load ptr, ptr %702, align 4, !dbg !8590, !alias.scope !8450, !noalias !8451, !nonnull !10, !noundef !10
  %_130.i84.i = getelementptr inbounds nuw float, ptr %_163.0.i82.i, i32 %_22.i36.i, !dbg !8637
  store <4 x float> %1011, ptr %_130.i84.i, align 4, !dbg !8639, !alias.scope !8643, !noalias !8647
  %_66.i89.i16134 = load <4 x float>, ptr %705, align 16, !dbg !8649
  %1054 = fdiv <4 x float> %1014, %_62.i86.i16133, !dbg !8650
  %1055 = fsub <4 x float> splat (float 1.000000e+00), %1054, !dbg !8654
  %1056 = fsub <4 x float> %1055, %_66.i89.i16134, !dbg !8658
  %1057 = fmul <4 x float> %_9.i29.i16123.pre, %1056, !dbg !8662
  %1058 = fadd <4 x float> %_66.i89.i16134, %1057, !dbg !8666
  %1059 = fcmp olt <4 x float> %1058, %1055, !dbg !8669
  %1060 = select <4 x i1> %1059, <4 x float> %1055, <4 x float> %1058, !dbg !8673
  %1061 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1060), !dbg !8674
  %1062 = fcmp uge <4 x float> %1061, splat (float 0x3BC79CA100000000), !dbg !8679
  %1063 = bitcast <4 x float> %1060 to <4 x i32>, !dbg !8684
  %1064 = select <4 x i1> %1062, <4 x i32> %1063, <4 x i32> zeroinitializer, !dbg !8684
  store <4 x i32> %1064, ptr %705, align 16, !dbg !8687
  %1065 = bitcast <4 x i32> %1064 to <4 x float>, !dbg !8688
  %1066 = fsub <4 x float> splat (float 1.000000e+00), %1065, !dbg !8692
  %_164.1.i99.i = load i32, ptr %706, align 4, !dbg !8693, !alias.scope !8450, !noalias !8451, !noundef !10
  %_74.i100.i = mul i32 %width.i30.i, %main_cursor.sroa.0.1.i100218546, !dbg !8694
  %_134.i101.i = icmp ugt i32 %_74.i100.i, %_164.1.i99.i, !dbg !8695
  br i1 %_134.i101.i, label %bb47.i120.i, label %bb48.i102.i, !dbg !8695, !prof !787

bb41.i121.i:                                      ; preds = %bb53.i74.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i36.i, i32 noundef %_163.1.i79.i.pre, i32 noundef %_163.1.i79.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c2286ff41a33b8c11aa270fe215441de) #32, !dbg !8698, !noalias !8619
  unreachable, !dbg !8698

bb48.i102.i:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6495
  %_137.i104.i = sub nuw i32 %_164.1.i99.i, %_74.i100.i, !dbg !8699
  %_8.i5779 = icmp samesign ugt i32 %_137.i104.i, 3, !dbg !8700
  br i1 %_8.i5779, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6485, label %bb2.i5780, !dbg !8700, !prof !1039

bb2.i5780:                                        ; preds = %bb48.i102.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_137.i104.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !8705, !noalias !8706
  unreachable, !dbg !8705

bb47.i120.i:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6495
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_74.i100.i, i32 noundef %_164.1.i99.i, i32 noundef %_164.1.i99.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a3a21efd18b403ec25db545d522c479) #32, !dbg !8710, !noalias !8619
  unreachable, !dbg !8710

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6485: ; preds = %bb48.i102.i
  %_164.0.i103.i = load ptr, ptr %707, align 4, !dbg !8693, !alias.scope !8450, !noalias !8451, !nonnull !10, !noundef !10
  %_141.i105.i = getelementptr inbounds nuw float, ptr %_164.0.i103.i, i32 %_74.i100.i, !dbg !8711
  %lanes.i5776.sroa.0.0.copyload = load <4 x i32>, ptr %_141.i105.i, align 4, !dbg !8713, !alias.scope !8717, !noalias !8721
  store <4 x i32> %lanes.i5799.sroa.0.0.copyload, ptr %_141.i105.i, align 4, !dbg !8723, !alias.scope !8728, !noalias !8732
  %1067 = bitcast <4 x i32> %lanes.i5776.sroa.0.0.copyload to <4 x float>, !dbg !8736
  %1068 = fmul <4 x float> %1066, %1067, !dbg !8740
  %1069 = bitcast <4 x i32> %lanes.i5776.sroa.0.0.copyload to <16 x i8>, !dbg !8741
  %1070 = bitcast <4 x float> %1068 to <16 x i8>, !dbg !8745
  %_4.i7185 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1069, <16 x i8> %1070, <16 x i8> %708), !dbg !8746
  store <16 x i8> %_4.i7185, ptr %_171.i, align 4, !dbg !8747, !alias.scope !8752, !noalias !8756
  %_172.i = icmp ugt i32 %base.i1004, %right_io.1, !dbg !8760
  br i1 %_172.i, label %bb56.i1040, label %bb57.i1018, !dbg !8760, !prof !787

bb54.i:                                           ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5824
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1004, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_adfae95437a74b76017017a3a282b9f2) #32, !dbg !8764, !noalias !7374
  unreachable, !dbg !8764

bb57.i1018:                                       ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6485
  %_174.i = sub nuw nsw i32 %right_io.1, %base.i1004, !dbg !8765
  %_178.i = getelementptr inbounds nuw float, ptr %right_io.0, i32 %base.i1004, !dbg !8766
  %_8.i5770 = icmp samesign ugt i32 %_174.i, 3, !dbg !8771
  br i1 %_8.i5770, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5774, label %bb2.i5771, !dbg !8771, !prof !1039

bb2.i5771:                                        ; preds = %bb57.i1018
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_174.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !8776, !noalias !8777
  unreachable, !dbg !8776

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5774: ; preds = %bb57.i1018
  %lanes.i5767.sroa.0.0.copyload = load <4 x i32>, ptr %_178.i, align 4, !dbg !8781, !alias.scope !8785, !noalias !8789
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8791), !dbg !8794
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8795), !dbg !8794
  %width.i.i = load i32, ptr %709, align 4, !dbg !8797, !alias.scope !8798, !noalias !8799, !noundef !10
  %1071 = bitcast <16 x i8> %_4.i7176 to <4 x float>, !dbg !8807
  %1072 = fcmp olt <4 x float> %_8.i.i100916124.pre, %1071, !dbg !8812
  %1073 = sext <4 x i1> %1072 to <4 x i32>, !dbg !8812
  %1074 = fdiv <4 x float> %_8.i.i100916124.pre, %1071, !dbg !8813
  %1075 = bitcast <4 x float> %1074 to <16 x i8>, !dbg !8817
  %1076 = bitcast <4 x i32> %1073 to <16 x i8>, !dbg !8821
  %_4.i7188 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1075, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %1076), !dbg !8822
  %_158.1.i.i = load i32, ptr %710, align 4, !dbg !8823, !alias.scope !8798, !noalias !8799, !noundef !10
  %_22.i.i1024 = mul i32 %width.i.i, %ring_cursor.sroa.0.1.i100118545, !dbg !8824
  %_90.i.i = icmp ugt i32 %_22.i.i1024, %_158.1.i.i, !dbg !8825
  br i1 %_90.i.i, label %bb34.i.i, label %bb35.i.i, !dbg !8825, !prof !787

bb35.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5774
  %_93.i.i = sub nuw i32 %_158.1.i.i, %_22.i.i1024, !dbg !8828
  %_8.i6477 = icmp samesign ugt i32 %_93.i.i, 3, !dbg !8829
  br i1 %_8.i6477, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6480, label %bb2.i6478, !dbg !8829, !prof !1039

bb2.i6478:                                        ; preds = %bb35.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_93.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !8834, !noalias !8835
  unreachable, !dbg !8834

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6480: ; preds = %bb35.i.i
  %_158.0.i.i = load ptr, ptr %711, align 4, !dbg !8823, !alias.scope !8798, !noalias !8799, !nonnull !10, !noundef !10
  %_97.i.i = getelementptr inbounds nuw float, ptr %_158.0.i.i, i32 %_22.i.i1024, !dbg !8839
  store <16 x i8> %_4.i7188, ptr %_97.i.i, align 4, !dbg !8841, !alias.scope !8845, !noalias !8849
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8851), !dbg !8854
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8855), !dbg !8854
  %width.i2227 = load i32, ptr %709, align 4, !dbg !8857, !alias.scope !8851, !noalias !8859, !noundef !10
  %1077 = icmp eq i32 %width.i2227, 0, !dbg !8860
  br i1 %1077, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2333, label %bb29.i2233.lr.ph, !dbg !8860

bb29.i2233.lr.ph:                                 ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6480
  %_126.1.i2238 = load i32, ptr %712, align 4, !alias.scope !8851, !noalias !8859, !noundef !10
  %_126.0.i2242 = load ptr, ptr %713, align 4, !nonnull !10
  %1078 = add i32 %ring_cursor.sroa.0.1.i100118545, 1
  %_21.not.i2247 = icmp ult i32 %1078, %_91.i
  %1079 = select i1 %_21.not.i2247, i32 0, i32 %_91.i
  %start1.sroa.0.0.i2248 = sub nuw i32 %1078, %1079
  %_128.1.i2251 = load i32, ptr %710, align 4
  %_128.0.i2255 = load ptr, ptr %711, align 4, !nonnull !10
  %_130.1.i2256 = load i32, ptr %714, align 4
  %_130.0.i2260 = load ptr, ptr %715, align 4, !nonnull !10
  %_132.1.i2263 = load i32, ptr %716, align 4
  %_132.0.i2267 = load ptr, ptr %717, align 4, !nonnull !10
  %_43.i2280 = mul i32 %width.i2227, %start1.sroa.0.0.i2248
  br label %bb29.i2233, !dbg !8860

bb29.i2233:                                       ; preds = %bb29.i2233.lr.ph, %bb28.i2295
  %iter.sroa.0.0.idx.i223118537 = phi i32 [ 0, %bb29.i2233.lr.ph ], [ %iter.sroa.0.0.add.i2236, %bb28.i2295 ]
  %iter.sroa.4.0.i223018536 = phi i32 [ 0, %bb29.i2233.lr.ph ], [ %_102.0.i2237, %bb28.i2295 ]
  %iter.sroa.7.0.i222918535 = phi i32 [ %width.i2227, %bb29.i2233.lr.ph ], [ %1080, %bb28.i2295 ]
  %iter.sroa.0.0.ptr.i223218538 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 %iter.sroa.0.0.idx.i223118537, !dbg !8862
  %1080 = add i32 %iter.sroa.7.0.i222918535, -1, !dbg !8862
  %_109.i2234 = icmp eq i32 %iter.sroa.0.0.idx.i223118537, 32, !dbg !8863
  br i1 %_109.i2234, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2333.loopexit, label %bb33.i2235, !dbg !8867

bb33.i2235:                                       ; preds = %bb29.i2233
  %iter.sroa.0.0.add.i2236 = add nuw nsw i32 %iter.sroa.0.0.idx.i223118537, 4, !dbg !8868
  %_102.0.i2237 = add nuw nsw i32 %iter.sroa.4.0.i223018536, 1, !dbg !8870
  %exitcond20939.not = icmp eq i32 %iter.sroa.4.0.i223018536, %_126.1.i2238, !dbg !8871
  br i1 %exitcond20939.not, label %panic.i2240, label %bb2.i2241, !dbg !8871

bb2.i2241:                                        ; preds = %bb33.i2235
  %1081 = getelementptr inbounds nuw %LaneShape, ptr %_126.0.i2242, i32 %iter.sroa.4.0.i223018536, !dbg !8871
  %shape.i2243 = load i32, ptr %1081, align 4, !dbg !8871, !noalias !8872, !noundef !10
  %1082 = getelementptr inbounds nuw i8, ptr %1081, i32 4, !dbg !8871
  %shape3.i2244 = load i32, ptr %1082, align 4, !dbg !8871, !noalias !8872, !noundef !10
  %1083 = add i32 %shape3.i2244, %ring_cursor.sroa.0.1.i100118545, !dbg !8873
  %_18.not.i2245 = icmp ult i32 %1083, %_91.i, !dbg !8874
  %1084 = select i1 %_18.not.i2245, i32 0, i32 %_91.i, !dbg !8874
  %spec.select.i2246 = sub nuw i32 %1083, %1084, !dbg !8874
  %_25.i2249 = mul i32 %spec.select.i2246, %width.i2227, !dbg !8875
  %_24.i2250 = add i32 %_25.i2249, %iter.sroa.4.0.i223018536, !dbg !8875
  %_28.i2252 = icmp ult i32 %_24.i2250, %_128.1.i2251, !dbg !8876
  br i1 %_28.i2252, label %bb9.i2254, label %panic5.i2253, !dbg !8876

panic.i2240:                                      ; preds = %bb33.i2235
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_126.1.i2238, i32 noundef %_126.1.i2238, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_12856b5f9033764dbf23e04eb77c5c46) #32, !dbg !8871, !noalias !8872
  unreachable, !dbg !8871

bb9.i2254:                                        ; preds = %bb2.i2241
  %1085 = getelementptr inbounds nuw float, ptr %_128.0.i2255, i32 %_24.i2250, !dbg !8876
  %1086 = load float, ptr %1085, align 4, !dbg !8876, !noalias !8872, !noundef !10
  %exitcond20940.not = icmp eq i32 %iter.sroa.4.0.i223018536, %_130.1.i2256, !dbg !8877
  br i1 %exitcond20940.not, label %panic6.i2258, label %bb10.i2259, !dbg !8877

panic5.i2253:                                     ; preds = %bb2.i2241
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_24.i2250, i32 noundef %_128.1.i2251, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70ebf0cf1c90c6161926611ccf249a32) #32, !dbg !8876, !noalias !8872
  unreachable, !dbg !8876

bb10.i2259:                                       ; preds = %bb9.i2254
  %1087 = getelementptr inbounds nuw i32, ptr %_130.0.i2260, i32 %iter.sroa.4.0.i223018536, !dbg !8877
  %_30.i2261 = load i32, ptr %1087, align 4, !dbg !8877, !noalias !8872, !noundef !10
  %1088 = icmp eq i32 %_30.i2261, 0, !dbg !8878
  br i1 %1088, label %bb14.i2270, label %bb12.i2262, !dbg !8878

panic6.i2258:                                     ; preds = %bb9.i2254
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_130.1.i2256, i32 noundef %_130.1.i2256, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_023d591aaad7f5cf482ac80a9401eca1) #32, !dbg !8877, !noalias !8872
  unreachable, !dbg !8877

bb12.i2262:                                       ; preds = %bb10.i2259
  %_35.i2264 = icmp ult i32 %iter.sroa.4.0.i223018536, %_132.1.i2263, !dbg !8879
  br i1 %_35.i2264, label %bb13.i2266, label %panic7.i2265, !dbg !8879

bb14.i2270:                                       ; preds = %bb34.i2331, %bb13.i2266, %bb10.i2259
  %newest.sroa.0.0.i2271 = phi float [ %1086, %bb10.i2259 ], [ %_33.i2268, %bb34.i2331 ], [ %1086, %bb13.i2266 ], !dbg !8880
  %exitcond20941.not = icmp eq i32 %iter.sroa.4.0.i223018536, %_132.1.i2263, !dbg !8881
  br i1 %exitcond20941.not, label %panic8.i2274, label %bb15.i2275, !dbg !8881

bb13.i2266:                                       ; preds = %bb12.i2262
  %1089 = getelementptr inbounds nuw float, ptr %_132.0.i2267, i32 %iter.sroa.4.0.i223018536, !dbg !8879
  %_33.i2268 = load float, ptr %1089, align 4, !dbg !8879, !noalias !8872, !noundef !10
  %_116.i2269 = fcmp olt float %_33.i2268, %1086, !dbg !8882
  br i1 %_116.i2269, label %bb34.i2331, label %bb14.i2270, !dbg !8882

panic7.i2265:                                     ; preds = %bb12.i2262
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %iter.sroa.4.0.i223018536, i32 noundef %_132.1.i2263, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a228cac3279aae3a34886f59f51fbe9e) #32, !dbg !8879, !noalias !8872
  unreachable, !dbg !8879

bb34.i2331:                                       ; preds = %bb13.i2266
  br label %bb14.i2270, !dbg !8884

bb15.i2275:                                       ; preds = %bb14.i2270
  %1090 = getelementptr inbounds nuw float, ptr %_132.0.i2267, i32 %iter.sroa.4.0.i223018536, !dbg !8881
  store float %newest.sroa.0.0.i2271, ptr %1090, align 4, !dbg !8881, !noalias !8872
  %_40.i2277 = add i32 %_30.i2261, 1, !dbg !8885
  %complete.i2278 = icmp eq i32 %_40.i2277, %shape.i2243, !dbg !8885
  br i1 %complete.i2278, label %bb19.i2300, label %bb17.i2279, !dbg !8886

panic8.i2274:                                     ; preds = %bb14.i2270
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_132.1.i2263, i32 noundef %_132.1.i2263, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1fa5d88c1a2cc292a2767c48606a38fe) #32, !dbg !8881, !noalias !8872
  unreachable, !dbg !8881

bb17.i2279:                                       ; preds = %bb15.i2275
  %_42.i2281 = add i32 %iter.sroa.4.0.i223018536, %_43.i2280, !dbg !8887
  %_45.i2283 = icmp ult i32 %_42.i2281, %_128.1.i2251, !dbg !8888
  br i1 %_45.i2283, label %bb27.i2293, label %panic9.i2284, !dbg !8888

panic9.i2284:                                     ; preds = %bb17.i2279
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_42.i2281, i32 noundef %_128.1.i2251, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70d40ae2302f3ea19fa0c4a65fe82786) #32, !dbg !8888, !noalias !8872
  unreachable, !dbg !8888

bb27.i2293:                                       ; preds = %bb17.i2279
  %1091 = getelementptr inbounds nuw float, ptr %_128.0.i2255, i32 %_42.i2281, !dbg !8888
  %_41.i2287 = load float, ptr %1091, align 4, !dbg !8888, !noalias !8872, !noundef !10
  %_117.i2288 = fcmp olt float %_41.i2287, %newest.sroa.0.0.i2271, !dbg !8889
  %newest.sroa.0.1.i2289 = select i1 %_117.i2288, float %_41.i2287, float %newest.sroa.0.0.i2271, !dbg !8889
  store float %newest.sroa.0.1.i2289, ptr %iter.sroa.0.0.ptr.i223218538, align 4, !dbg !8891, !alias.scope !8855, !noalias !8892
  br label %bb28.i2295, !dbg !8893

bb28.i2295:                                       ; preds = %bb22.i2328, %bb19.i2300, %bb27.i2293
  %storemerge16136 = phi i32 [ %_40.i2277, %bb27.i2293 ], [ 0, %bb19.i2300 ], [ 0, %bb22.i2328 ], !dbg !8894
  store i32 %storemerge16136, ptr %1087, align 4, !dbg !8894, !noalias !8872
  %1092 = icmp eq i32 %1080, 0, !dbg !8860
  br i1 %1092, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2333.loopexit, label %bb29.i2233, !dbg !8860

bb19.i2300:                                       ; preds = %bb15.i2275
  store float %newest.sroa.0.0.i2271, ptr %iter.sroa.0.0.ptr.i223218538, align 4, !dbg !8891, !alias.scope !8855, !noalias !8892
  %_118.i230618531.not = icmp eq i32 %shape.i2243, 0, !dbg !8895
  br i1 %_118.i230618531.not, label %bb28.i2295, label %bb40.i2313.preheader, !dbg !8899

bb40.i2313.preheader:                             ; preds = %bb19.i2300
  %1093 = load float, ptr %1085, align 4, !dbg !8900, !noalias !8872, !noundef !10
  br label %bb40.i2313, !dbg !8901

bb40.i2313:                                       ; preds = %bb40.i2313.preheader, %bb22.i2328
  %iter2.sroa.0.0.i230518534 = phi i32 [ %_119.i2314, %bb22.i2328 ], [ 0, %bb40.i2313.preheader ]
  %suffix.sroa.0.0.i230418533 = phi float [ %suffix.sroa.0.1.i2324, %bb22.i2328 ], [ %1093, %bb40.i2313.preheader ]
  %end.sroa.0.1.i230318532 = phi i32 [ %1096, %bb22.i2328 ], [ %spec.select.i2246, %bb40.i2313.preheader ]
  %_54.i2315 = mul i32 %end.sroa.0.1.i230318532, %width.i2227, !dbg !8902
  %_53.i2316 = add i32 %_54.i2315, %iter.sroa.4.0.i223018536, !dbg !8902
  %_57.i2318 = icmp ult i32 %_53.i2316, %_128.1.i2251, !dbg !8901
  br i1 %_57.i2318, label %bb22.i2328, label %panic13.i2319, !dbg !8901

panic13.i2319:                                    ; preds = %bb40.i2313
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_53.i2316, i32 noundef %_128.1.i2251, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a271bbb7fd83c3605ea58a06b7065fa4) #32, !dbg !8901, !noalias !8872
  unreachable, !dbg !8901

bb22.i2328:                                       ; preds = %bb40.i2313
  %_119.i2314 = add nuw i32 %iter2.sroa.0.0.i230518534, 1, !dbg !8903
  %1094 = getelementptr inbounds nuw float, ptr %_128.0.i2255, i32 %_53.i2316, !dbg !8901
  %_52.i2322 = load float, ptr %1094, align 4, !dbg !8901, !noalias !8872, !noundef !10
  %_121.i2323 = fcmp olt float %suffix.sroa.0.0.i230418533, %_52.i2322, !dbg !8906
  %suffix.sroa.0.1.i2324 = select i1 %_121.i2323, float %suffix.sroa.0.0.i230418533, float %_52.i2322, !dbg !8906
  store float %suffix.sroa.0.1.i2324, ptr %1094, align 4, !dbg !8908, !noalias !8872
  %1095 = icmp eq i32 %end.sroa.0.1.i230318532, 0, !dbg !8909
  %spec.store.select.i2330 = select i1 %1095, i32 %_91.i, i32 %end.sroa.0.1.i230318532, !dbg !8909
  %1096 = add i32 %spec.store.select.i2330, -1, !dbg !8910
  %exitcond20938.not = icmp eq i32 %_119.i2314, %shape.i2243, !dbg !8895
  br i1 %exitcond20938.not, label %bb28.i2295, label %bb40.i2313, !dbg !8899

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2333.loopexit: ; preds = %bb28.i2295, %bb29.i2233
  %lanes.i5760.sroa.0.0.copyload.pre = load <4 x float>, ptr %scratch.i, align 4, !dbg !8911, !alias.scope !8916, !noalias !8920
  br label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2333, !dbg !8924

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2333: ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2333.loopexit, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6480
  %lanes.i5760.sroa.0.0.copyload = phi <4 x float> [ %lanes.i5760.sroa.0.0.copyload.pre, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2333.loopexit ], [ %lanes.i5785.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6480 ], !dbg !8911
  %1097 = fmul <4 x float> %lanes.i5760.sroa.0.0.copyload, splat (float 1.638400e+04), !dbg !8925
  %1098 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %1097), !dbg !8929
  %1099 = fmul <4 x float> %1098, splat (float 0x3F10000000000000), !dbg !8933
  %1100 = icmp eq i32 %width.i.i, 0, !dbg !8937
  %_163.1.i.i.pre = load i32, ptr %718, align 4, !dbg !8939, !alias.scope !8798, !noalias !8799
  br i1 %1100, label %bb53.i.i, label %bb36.i.i.lr.ph, !dbg !8937

bb36.i.i.lr.ph:                                   ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2333
  %_159.1.i.i = load i32, ptr %712, align 4, !alias.scope !8798, !noalias !8799, !noundef !10
  %_159.0.i.i = load ptr, ptr %713, align 4, !nonnull !10
  %_161.0.i.i = load ptr, ptr %719, align 4, !nonnull !10
  %exitcond20944.not = icmp eq i32 %_159.1.i.i, 0, !dbg !8940
  br i1 %exitcond20944.not, label %panic.i.i, label %bb14.i.i, !dbg !8940

bb34.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5774
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i1024, i32 noundef %_158.1.i.i, i32 noundef %_158.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_db7c42041d7aaaba4839906f37294547) #32, !dbg !8941, !noalias !8942
  unreachable, !dbg !8941

bb53.i.i.loopexit:                                ; preds = %bb18.i.i.7, %bb18.i.i.6, %bb18.i.i.5, %bb18.i.i.4, %bb18.i.i.3, %bb18.i.i.2, %bb18.i.i.1, %bb18.i.i
  %lanes.i5753.sroa.0.0.copyload.pre = load <4 x float>, ptr %scratch.i, align 4, !dbg !8943, !alias.scope !8948, !noalias !8952
  br label %bb53.i.i, !dbg !8956

bb53.i.i:                                         ; preds = %bb53.i.i.loopexit, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2333
  %lanes.i5753.sroa.0.0.copyload = phi <4 x float> [ %lanes.i5753.sroa.0.0.copyload.pre, %bb53.i.i.loopexit ], [ %lanes.i5760.sroa.0.0.copyload, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit2333 ], !dbg !8943
  %1101 = fadd <4 x float> %1099, %974, !dbg !8957
  %1102 = fsub <4 x float> %1101, %lanes.i5753.sroa.0.0.copyload, !dbg !8961
  %_123.i.i = icmp ugt i32 %_22.i.i1024, %_163.1.i.i.pre, !dbg !8965
  br i1 %_123.i.i, label %bb41.i.i, label %bb42.i.i, !dbg !8965, !prof !787

bb14.i.i:                                         ; preds = %bb36.i.i.lr.ph
  %1103 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 8, !dbg !8940
  %_42.i.i1030 = load i32, ptr %1103, align 4, !dbg !8940, !noalias !8968, !noundef !10
  %1104 = add i32 %_42.i.i1030, %ring_cursor.sroa.0.1.i100118545, !dbg !8969
  %_45.not.i.i = icmp ult i32 %1104, %_91.i, !dbg !8970
  %1105 = select i1 %_45.not.i.i, i32 0, i32 %_91.i, !dbg !8970
  %spec.select.i.i = sub nuw i32 %1104, %1105, !dbg !8970
  %_49.i.i1031 = mul i32 %spec.select.i.i, %width.i.i, !dbg !8971
  %_51.i.i = icmp ult i32 %_49.i.i1031, %_163.1.i.i.pre, !dbg !8972
  br i1 %_51.i.i, label %bb18.i.i, label %panic1.i.i, !dbg !8972

panic.i.i:                                        ; preds = %bb36.i.i.7, %bb36.i.i.6, %bb36.i.i.5, %bb36.i.i.4, %bb36.i.i.3, %bb36.i.i.2, %bb36.i.i.1, %bb36.i.i.lr.ph
  %_159.1.i.i.lcssa.ph = phi i32 [ 7, %bb36.i.i.7 ], [ 6, %bb36.i.i.6 ], [ 5, %bb36.i.i.5 ], [ 4, %bb36.i.i.4 ], [ 3, %bb36.i.i.3 ], [ 2, %bb36.i.i.2 ], [ 1, %bb36.i.i.1 ], [ 0, %bb36.i.i.lr.ph ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_159.1.i.i.lcssa.ph, i32 noundef %_159.1.i.i.lcssa.ph, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_33746731a929bad63709ddedbca82f5f) #32, !dbg !8940, !noalias !8968
  unreachable, !dbg !8940

bb18.i.i:                                         ; preds = %bb14.i.i
  %1106 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_49.i.i1031, !dbg !8972
  %_47.i.i = load float, ptr %1106, align 4, !dbg !8972, !noalias !8968, !noundef !10
  store float %_47.i.i, ptr %scratch.i, align 4, !dbg !8973, !alias.scope !8795, !noalias !8974
  %1107 = icmp eq i32 %width.i.i, 1, !dbg !8937
  br i1 %1107, label %bb53.i.i.loopexit, label %bb36.i.i.1, !dbg !8937

bb36.i.i.1:                                       ; preds = %bb18.i.i
  %exitcond20944.1.not = icmp eq i32 %_159.1.i.i, 1, !dbg !8940
  br i1 %exitcond20944.1.not, label %panic.i.i, label %bb14.i.i.1, !dbg !8940

bb14.i.i.1:                                       ; preds = %bb36.i.i.1
  %1108 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 20, !dbg !8940
  %_42.i.i1030.1 = load i32, ptr %1108, align 4, !dbg !8940, !noalias !8968, !noundef !10
  %1109 = add i32 %_42.i.i1030.1, %ring_cursor.sroa.0.1.i100118545, !dbg !8969
  %_45.not.i.i.1 = icmp ult i32 %1109, %_91.i, !dbg !8970
  %1110 = select i1 %_45.not.i.i.1, i32 0, i32 %_91.i, !dbg !8970
  %spec.select.i.i.1 = sub nuw i32 %1109, %1110, !dbg !8970
  %_49.i.i1031.1 = mul i32 %spec.select.i.i.1, %width.i.i, !dbg !8971
  %_48.i.i.1 = add i32 %_49.i.i1031.1, 1, !dbg !8971
  %_51.i.i.1 = icmp ult i32 %_48.i.i.1, %_163.1.i.i.pre, !dbg !8972
  br i1 %_51.i.i.1, label %bb18.i.i.1, label %panic1.i.i, !dbg !8972

bb18.i.i.1:                                       ; preds = %bb14.i.i.1
  %1111 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.1, !dbg !8972
  %_47.i.i.1 = load float, ptr %1111, align 4, !dbg !8972, !noalias !8968, !noundef !10
  store float %_47.i.i.1, ptr %iter.sroa.0.0.ptr.i.i18542.1, align 4, !dbg !8973, !alias.scope !8795, !noalias !8974
  %1112 = icmp eq i32 %width.i.i, 2, !dbg !8937
  br i1 %1112, label %bb53.i.i.loopexit, label %bb36.i.i.2, !dbg !8937

bb36.i.i.2:                                       ; preds = %bb18.i.i.1
  %exitcond20944.2.not = icmp eq i32 %_159.1.i.i, 2, !dbg !8940
  br i1 %exitcond20944.2.not, label %panic.i.i, label %bb14.i.i.2, !dbg !8940

bb14.i.i.2:                                       ; preds = %bb36.i.i.2
  %1113 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 32, !dbg !8940
  %_42.i.i1030.2 = load i32, ptr %1113, align 4, !dbg !8940, !noalias !8968, !noundef !10
  %1114 = add i32 %_42.i.i1030.2, %ring_cursor.sroa.0.1.i100118545, !dbg !8969
  %_45.not.i.i.2 = icmp ult i32 %1114, %_91.i, !dbg !8970
  %1115 = select i1 %_45.not.i.i.2, i32 0, i32 %_91.i, !dbg !8970
  %spec.select.i.i.2 = sub nuw i32 %1114, %1115, !dbg !8970
  %_49.i.i1031.2 = mul i32 %spec.select.i.i.2, %width.i.i, !dbg !8971
  %_48.i.i.2 = add i32 %_49.i.i1031.2, 2, !dbg !8971
  %_51.i.i.2 = icmp ult i32 %_48.i.i.2, %_163.1.i.i.pre, !dbg !8972
  br i1 %_51.i.i.2, label %bb18.i.i.2, label %panic1.i.i, !dbg !8972

bb18.i.i.2:                                       ; preds = %bb14.i.i.2
  %1116 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.2, !dbg !8972
  %_47.i.i.2 = load float, ptr %1116, align 4, !dbg !8972, !noalias !8968, !noundef !10
  store float %_47.i.i.2, ptr %iter.sroa.0.0.ptr.i.i18542.2, align 4, !dbg !8973, !alias.scope !8795, !noalias !8974
  %1117 = icmp eq i32 %width.i.i, 3, !dbg !8937
  br i1 %1117, label %bb53.i.i.loopexit, label %bb36.i.i.3, !dbg !8937

bb36.i.i.3:                                       ; preds = %bb18.i.i.2
  %exitcond20944.3.not = icmp eq i32 %_159.1.i.i, 3, !dbg !8940
  br i1 %exitcond20944.3.not, label %panic.i.i, label %bb14.i.i.3, !dbg !8940

bb14.i.i.3:                                       ; preds = %bb36.i.i.3
  %1118 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 44, !dbg !8940
  %_42.i.i1030.3 = load i32, ptr %1118, align 4, !dbg !8940, !noalias !8968, !noundef !10
  %1119 = add i32 %_42.i.i1030.3, %ring_cursor.sroa.0.1.i100118545, !dbg !8969
  %_45.not.i.i.3 = icmp ult i32 %1119, %_91.i, !dbg !8970
  %1120 = select i1 %_45.not.i.i.3, i32 0, i32 %_91.i, !dbg !8970
  %spec.select.i.i.3 = sub nuw i32 %1119, %1120, !dbg !8970
  %_49.i.i1031.3 = mul i32 %spec.select.i.i.3, %width.i.i, !dbg !8971
  %_48.i.i.3 = add i32 %_49.i.i1031.3, 3, !dbg !8971
  %_51.i.i.3 = icmp ult i32 %_48.i.i.3, %_163.1.i.i.pre, !dbg !8972
  br i1 %_51.i.i.3, label %bb18.i.i.3, label %panic1.i.i, !dbg !8972

bb18.i.i.3:                                       ; preds = %bb14.i.i.3
  %1121 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.3, !dbg !8972
  %_47.i.i.3 = load float, ptr %1121, align 4, !dbg !8972, !noalias !8968, !noundef !10
  store float %_47.i.i.3, ptr %iter.sroa.0.0.ptr.i.i18542.3, align 4, !dbg !8973, !alias.scope !8795, !noalias !8974
  %1122 = icmp eq i32 %width.i.i, 4, !dbg !8937
  br i1 %1122, label %bb53.i.i.loopexit, label %bb36.i.i.4, !dbg !8937

bb36.i.i.4:                                       ; preds = %bb18.i.i.3
  %exitcond20944.4.not = icmp eq i32 %_159.1.i.i, 4, !dbg !8940
  br i1 %exitcond20944.4.not, label %panic.i.i, label %bb14.i.i.4, !dbg !8940

bb14.i.i.4:                                       ; preds = %bb36.i.i.4
  %1123 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 56, !dbg !8940
  %_42.i.i1030.4 = load i32, ptr %1123, align 4, !dbg !8940, !noalias !8968, !noundef !10
  %1124 = add i32 %_42.i.i1030.4, %ring_cursor.sroa.0.1.i100118545, !dbg !8969
  %_45.not.i.i.4 = icmp ult i32 %1124, %_91.i, !dbg !8970
  %1125 = select i1 %_45.not.i.i.4, i32 0, i32 %_91.i, !dbg !8970
  %spec.select.i.i.4 = sub nuw i32 %1124, %1125, !dbg !8970
  %_49.i.i1031.4 = mul i32 %spec.select.i.i.4, %width.i.i, !dbg !8971
  %_48.i.i.4 = add i32 %_49.i.i1031.4, 4, !dbg !8971
  %_51.i.i.4 = icmp ult i32 %_48.i.i.4, %_163.1.i.i.pre, !dbg !8972
  br i1 %_51.i.i.4, label %bb18.i.i.4, label %panic1.i.i, !dbg !8972

bb18.i.i.4:                                       ; preds = %bb14.i.i.4
  %1126 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.4, !dbg !8972
  %_47.i.i.4 = load float, ptr %1126, align 4, !dbg !8972, !noalias !8968, !noundef !10
  store float %_47.i.i.4, ptr %iter.sroa.0.0.ptr.i.i18542.4, align 4, !dbg !8973, !alias.scope !8795, !noalias !8974
  %1127 = icmp eq i32 %width.i.i, 5, !dbg !8937
  br i1 %1127, label %bb53.i.i.loopexit, label %bb36.i.i.5, !dbg !8937

bb36.i.i.5:                                       ; preds = %bb18.i.i.4
  %exitcond20944.5.not = icmp eq i32 %_159.1.i.i, 5, !dbg !8940
  br i1 %exitcond20944.5.not, label %panic.i.i, label %bb14.i.i.5, !dbg !8940

bb14.i.i.5:                                       ; preds = %bb36.i.i.5
  %1128 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 68, !dbg !8940
  %_42.i.i1030.5 = load i32, ptr %1128, align 4, !dbg !8940, !noalias !8968, !noundef !10
  %1129 = add i32 %_42.i.i1030.5, %ring_cursor.sroa.0.1.i100118545, !dbg !8969
  %_45.not.i.i.5 = icmp ult i32 %1129, %_91.i, !dbg !8970
  %1130 = select i1 %_45.not.i.i.5, i32 0, i32 %_91.i, !dbg !8970
  %spec.select.i.i.5 = sub nuw i32 %1129, %1130, !dbg !8970
  %_49.i.i1031.5 = mul i32 %spec.select.i.i.5, %width.i.i, !dbg !8971
  %_48.i.i.5 = add i32 %_49.i.i1031.5, 5, !dbg !8971
  %_51.i.i.5 = icmp ult i32 %_48.i.i.5, %_163.1.i.i.pre, !dbg !8972
  br i1 %_51.i.i.5, label %bb18.i.i.5, label %panic1.i.i, !dbg !8972

bb18.i.i.5:                                       ; preds = %bb14.i.i.5
  %1131 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.5, !dbg !8972
  %_47.i.i.5 = load float, ptr %1131, align 4, !dbg !8972, !noalias !8968, !noundef !10
  store float %_47.i.i.5, ptr %iter.sroa.0.0.ptr.i.i18542.5, align 4, !dbg !8973, !alias.scope !8795, !noalias !8974
  %1132 = icmp eq i32 %width.i.i, 6, !dbg !8937
  br i1 %1132, label %bb53.i.i.loopexit, label %bb36.i.i.6, !dbg !8937

bb36.i.i.6:                                       ; preds = %bb18.i.i.5
  %exitcond20944.6.not = icmp eq i32 %_159.1.i.i, 6, !dbg !8940
  br i1 %exitcond20944.6.not, label %panic.i.i, label %bb14.i.i.6, !dbg !8940

bb14.i.i.6:                                       ; preds = %bb36.i.i.6
  %1133 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 80, !dbg !8940
  %_42.i.i1030.6 = load i32, ptr %1133, align 4, !dbg !8940, !noalias !8968, !noundef !10
  %1134 = add i32 %_42.i.i1030.6, %ring_cursor.sroa.0.1.i100118545, !dbg !8969
  %_45.not.i.i.6 = icmp ult i32 %1134, %_91.i, !dbg !8970
  %1135 = select i1 %_45.not.i.i.6, i32 0, i32 %_91.i, !dbg !8970
  %spec.select.i.i.6 = sub nuw i32 %1134, %1135, !dbg !8970
  %_49.i.i1031.6 = mul i32 %spec.select.i.i.6, %width.i.i, !dbg !8971
  %_48.i.i.6 = add i32 %_49.i.i1031.6, 6, !dbg !8971
  %_51.i.i.6 = icmp ult i32 %_48.i.i.6, %_163.1.i.i.pre, !dbg !8972
  br i1 %_51.i.i.6, label %bb18.i.i.6, label %panic1.i.i, !dbg !8972

bb18.i.i.6:                                       ; preds = %bb14.i.i.6
  %1136 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.6, !dbg !8972
  %_47.i.i.6 = load float, ptr %1136, align 4, !dbg !8972, !noalias !8968, !noundef !10
  store float %_47.i.i.6, ptr %iter.sroa.0.0.ptr.i.i18542.6, align 4, !dbg !8973, !alias.scope !8795, !noalias !8974
  %1137 = icmp eq i32 %width.i.i, 7, !dbg !8937
  br i1 %1137, label %bb53.i.i.loopexit, label %bb36.i.i.7, !dbg !8937

bb36.i.i.7:                                       ; preds = %bb18.i.i.6
  %exitcond20944.7.not = icmp eq i32 %_159.1.i.i, 7, !dbg !8940
  br i1 %exitcond20944.7.not, label %panic.i.i, label %bb14.i.i.7, !dbg !8940

bb14.i.i.7:                                       ; preds = %bb36.i.i.7
  %1138 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 92, !dbg !8940
  %_42.i.i1030.7 = load i32, ptr %1138, align 4, !dbg !8940, !noalias !8968, !noundef !10
  %1139 = add i32 %_42.i.i1030.7, %ring_cursor.sroa.0.1.i100118545, !dbg !8969
  %_45.not.i.i.7 = icmp ult i32 %1139, %_91.i, !dbg !8970
  %1140 = select i1 %_45.not.i.i.7, i32 0, i32 %_91.i, !dbg !8970
  %spec.select.i.i.7 = sub nuw i32 %1139, %1140, !dbg !8970
  %_49.i.i1031.7 = mul i32 %spec.select.i.i.7, %width.i.i, !dbg !8971
  %_48.i.i.7 = add i32 %_49.i.i1031.7, 7, !dbg !8971
  %_51.i.i.7 = icmp ult i32 %_48.i.i.7, %_163.1.i.i.pre, !dbg !8972
  br i1 %_51.i.i.7, label %bb18.i.i.7, label %panic1.i.i, !dbg !8972

bb18.i.i.7:                                       ; preds = %bb14.i.i.7
  %1141 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.7, !dbg !8972
  %_47.i.i.7 = load float, ptr %1141, align 4, !dbg !8972, !noalias !8968, !noundef !10
  store float %_47.i.i.7, ptr %iter.sroa.0.0.ptr.i.i18542.7, align 4, !dbg !8973, !alias.scope !8795, !noalias !8974
  br label %bb53.i.i.loopexit, !dbg !8937

panic1.i.i:                                       ; preds = %bb14.i.i.7, %bb14.i.i.6, %bb14.i.i.5, %bb14.i.i.4, %bb14.i.i.3, %bb14.i.i.2, %bb14.i.i.1, %bb14.i.i
  %_48.i.i.lcssa.ph = phi i32 [ %_48.i.i.7, %bb14.i.i.7 ], [ %_48.i.i.6, %bb14.i.i.6 ], [ %_48.i.i.5, %bb14.i.i.5 ], [ %_48.i.i.4, %bb14.i.i.4 ], [ %_48.i.i.3, %bb14.i.i.3 ], [ %_48.i.i.2, %bb14.i.i.2 ], [ %_48.i.i.1, %bb14.i.i.1 ], [ %_49.i.i1031, %bb14.i.i ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_48.i.i.lcssa.ph, i32 noundef %_163.1.i.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_0e76a26fbb751dfb8cf8a069b5b0d39a) #32, !dbg !8972, !noalias !8968
  unreachable, !dbg !8972

bb42.i.i:                                         ; preds = %bb53.i.i
  %_126.i.i = sub nuw i32 %_163.1.i.i.pre, %_22.i.i1024, !dbg !8975
  %_8.i6472 = icmp samesign ugt i32 %_126.i.i, 3, !dbg !8976
  br i1 %_8.i6472, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6475, label %bb2.i6473, !dbg !8976, !prof !1039

bb2.i6473:                                        ; preds = %bb42.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_126.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !8981, !noalias !8982
  unreachable, !dbg !8981

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6475: ; preds = %bb42.i.i
  %_163.0.i.i = load ptr, ptr %719, align 4, !dbg !8939, !alias.scope !8798, !noalias !8799, !nonnull !10, !noundef !10
  %_130.i.i = getelementptr inbounds nuw float, ptr %_163.0.i.i, i32 %_22.i.i1024, !dbg !8986
  store <4 x float> %1099, ptr %_130.i.i, align 4, !dbg !8988, !alias.scope !8992, !noalias !8996
  %_66.i.i16143 = load <4 x float>, ptr %722, align 16, !dbg !8998
  %1142 = fdiv <4 x float> %1102, %_62.i.i103416142, !dbg !8999
  %1143 = fsub <4 x float> splat (float 1.000000e+00), %1142, !dbg !9003
  %1144 = fsub <4 x float> %1143, %_66.i.i16143, !dbg !9007
  %1145 = fmul <4 x float> %_9.i.i101016125.pre, %1144, !dbg !9011
  %1146 = fadd <4 x float> %_66.i.i16143, %1145, !dbg !9015
  %1147 = fcmp olt <4 x float> %1146, %1143, !dbg !9018
  %1148 = select <4 x i1> %1147, <4 x float> %1143, <4 x float> %1146, !dbg !9022
  %1149 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1148), !dbg !9023
  %1150 = fcmp uge <4 x float> %1149, splat (float 0x3BC79CA100000000), !dbg !9028
  %1151 = bitcast <4 x float> %1148 to <4 x i32>, !dbg !9033
  %1152 = select <4 x i1> %1150, <4 x i32> %1151, <4 x i32> zeroinitializer, !dbg !9033
  store <4 x i32> %1152, ptr %722, align 16, !dbg !9036
  %1153 = bitcast <4 x i32> %1152 to <4 x float>, !dbg !9037
  %1154 = fsub <4 x float> splat (float 1.000000e+00), %1153, !dbg !9041
  %_164.1.i.i = load i32, ptr %723, align 4, !dbg !9042, !alias.scope !8798, !noalias !8799, !noundef !10
  %_74.i.i = mul i32 %width.i.i, %main_cursor.sroa.0.1.i100218546, !dbg !9043
  %_134.i.i = icmp ugt i32 %_74.i.i, %_164.1.i.i, !dbg !9044
  br i1 %_134.i.i, label %bb47.i.i, label %bb48.i.i, !dbg !9044, !prof !787

bb41.i.i:                                         ; preds = %bb53.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i1024, i32 noundef %_163.1.i.i.pre, i32 noundef %_163.1.i.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c2286ff41a33b8c11aa270fe215441de) #32, !dbg !9047, !noalias !8968
  unreachable, !dbg !9047

bb48.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6475
  %_137.i.i = sub nuw i32 %_164.1.i.i, %_74.i.i, !dbg !9048
  %_8.i5747 = icmp samesign ugt i32 %_137.i.i, 3, !dbg !9049
  br i1 %_8.i5747, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6465, label %bb2.i5748, !dbg !9049, !prof !1039

bb2.i5748:                                        ; preds = %bb48.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_137.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !9054, !noalias !9055
  unreachable, !dbg !9054

bb47.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6475
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_74.i.i, i32 noundef %_164.1.i.i, i32 noundef %_164.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a3a21efd18b403ec25db545d522c479) #32, !dbg !9059, !noalias !8968
  unreachable, !dbg !9059

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6465: ; preds = %bb48.i.i
  %_164.0.i.i = load ptr, ptr %724, align 4, !dbg !9042, !alias.scope !8798, !noalias !8799, !nonnull !10, !noundef !10
  %_141.i.i = getelementptr inbounds nuw float, ptr %_164.0.i.i, i32 %_74.i.i, !dbg !9060
  %lanes.i5744.sroa.0.0.copyload = load <4 x i32>, ptr %_141.i.i, align 4, !dbg !9062, !alias.scope !9066, !noalias !9070
  store <4 x i32> %lanes.i5767.sroa.0.0.copyload, ptr %_141.i.i, align 4, !dbg !9072, !alias.scope !9077, !noalias !9081
  %1155 = bitcast <4 x i32> %lanes.i5744.sroa.0.0.copyload to <4 x float>, !dbg !9085
  %1156 = fmul <4 x float> %1154, %1155, !dbg !9089
  %1157 = bitcast <4 x i32> %lanes.i5744.sroa.0.0.copyload to <16 x i8>, !dbg !9090
  %1158 = bitcast <4 x float> %1156 to <16 x i8>, !dbg !9094
  %_4.i7195 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1157, <16 x i8> %1158, <16 x i8> %708), !dbg !9095
  store <16 x i8> %_4.i7195, ptr %_178.i, align 4, !dbg !9096, !alias.scope !9101, !noalias !9105
  %1159 = add i32 %main_cursor.sroa.0.1.i100218546, 1, !dbg !9109
  %_104.i = icmp eq i32 %1159, %_106.i, !dbg !9110
  %spec.store.select11.i = select i1 %_104.i, i32 0, i32 %1159, !dbg !9110
  %1160 = add i32 %ring_cursor.sroa.0.1.i100118545, 1, !dbg !9111
  %_107.i = icmp eq i32 %1160, %_91.i, !dbg !9112
  %spec.store.select12.i = select i1 %_107.i, i32 0, i32 %1160, !dbg !9112
  %exitcond20948.not = icmp eq i32 %976, %umax20947, !dbg !9113
  br i1 %exitcond20948.not, label %bb16.i.bb13.i.loopexit_crit_edge, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit5824, !dbg !7345

bb56.i1040:                                       ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6485
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1004, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_7e50e0c93e87b60da469fc303486c501) #32, !dbg !9116, !noalias !7374
  unreachable, !dbg !9116

_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %bb13.i.loopexit, %bb11.i
  %ring_cursor.sroa.0.0.i796.lcssa = phi i32 [ %_37.i790, %bb11.i ], [ %ring_cursor.sroa.0.1.i1001.lcssa, %bb13.i.loopexit ], !dbg !7299
  %main_cursor.sroa.0.0.i797.lcssa = phi i32 [ %_35.i, %bb11.i ], [ %main_cursor.sroa.0.1.i1002.lcssa, %bb13.i.loopexit ], !dbg !7296
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_(ptr noalias noundef readonly align 16 captures(none) dereferenceable(368) %hot_left.i783, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33) #31, !dbg !9117
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_(ptr noalias noundef readonly align 16 captures(none) dereferenceable(368) %hot_right.i782, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34) #31, !dbg !9118
  store i32 %main_cursor.sroa.0.0.i797.lcssa, ptr %_35, align 4, !dbg !9119, !alias.scope !7278, !noalias !7298
  store i32 %ring_cursor.sroa.0.0.i796.lcssa, ptr %81, align 4, !dbg !9120, !alias.scope !7278, !noalias !7298
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i779), !dbg !9121, !noalias !7303
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i780), !dbg !9122, !noalias !7303
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i), !dbg !9123, !noalias !7303
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !7273

bb7.i:                                            ; preds = %bb5.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9124), !dbg !9127
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9128), !dbg !9127
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9130), !dbg !9127
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9132), !dbg !9127
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9134), !dbg !9127
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_(ptr noalias noundef align 16 captures(none) dereferenceable(368) %hot_left.i68, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #31, !dbg !9136
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_(ptr noalias noundef align 16 captures(none) dereferenceable(368) %hot_right.i67, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #31, !dbg !9140
  %1161 = load i8, ptr %82, align 16, !dbg !9142, !range !4667, !alias.scope !9124, !noalias !9146, !noundef !10
  %1162 = load i8, ptr %83, align 1, !dbg !9149, !range !4667, !alias.scope !9124, !noalias !9146, !noundef !10
  %ring.i76 = load i32, ptr %84, align 4, !dbg !9151, !alias.scope !9128, !noalias !9153, !noundef !10
  %main.i77 = load i32, ptr %85, align 4, !dbg !9154, !alias.scope !9128, !noalias !9153, !noundef !10
  %_36.i78 = load i32, ptr %_35, align 4, !dbg !9156, !alias.scope !9134, !noalias !9158, !noundef !10
  %_37.i79 = load i32, ptr %86, align 4, !dbg !9159, !alias.scope !9134, !noalias !9158, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i65), !dbg !9161, !noalias !9163
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i65, i8 0, i32 1024, i1 false), !noalias !9163
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i64), !dbg !9164, !noalias !9163
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i64, i8 0, i32 1024, i1 false), !noalias !9163
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i63), !dbg !9166, !noalias !9163
; call <true_peak_limiter::UniformHot<wide::f32x4_::f32x4>>::new
  call fastcc void @_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E3newB5_(ptr noalias noundef align 16 captures(none) dereferenceable(64) %uniform_left.i63, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33, i32 %ring.i76, i32 %main.i77) #31, !dbg !9168
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_right.i62), !dbg !9169, !noalias !9163
  %_32.val = load i32, ptr %84, align 4, !dbg !9171, !noundef !10
  %_32.val6806 = load i32, ptr %85, align 4, !dbg !9171, !noundef !10
; call <true_peak_limiter::UniformHot<wide::f32x4_::f32x4>>::new
  call fastcc void @_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E3newB5_(ptr noalias noundef align 16 captures(none) dereferenceable(64) %uniform_right.i62, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34, i32 %_32.val, i32 %_32.val6806) #31, !dbg !9171
  %_166.not.i9018772 = icmp eq i32 %frames, 0, !dbg !9172
  br i1 %_166.not.i9018772, label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb44.i91.lr.ph, !dbg !9172

bb44.i91.lr.ph:                                   ; preds = %bb7.i
  %_33.i74 = trunc nuw i8 %1162 to i1, !dbg !9149
  %spec.store.select25.i75 = select i1 %_33.i74, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !9149
  %_32.i72 = trunc nuw i8 %1161 to i1, !dbg !9142
  %link.sroa.0.0.i73 = select i1 %_32.i72, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !9142
  %d9.i7197 = lshr i32 %frames, 5, !dbg !9182
  %r2.i7198 = and i32 %frames, 31, !dbg !9189
  %_19.not.i7199 = icmp ne i32 %r2.i7198, 0, !dbg !9190
  %1163 = zext i1 %_19.not.i7199 to i32, !dbg !9190
  %yield_count.sroa.0.0.i7200 = add nuw nsw i32 %d9.i7197, %1163, !dbg !9190
  %history.i46.i23.sroa.7.0.hot_left.i68.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i68, i32 16
  %history.i46.i23.sroa.10.0.hot_left.i68.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i68, i32 32
  %history.i46.i23.sroa.13.0.hot_left.i68.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i68, i32 48
  %history.i46.i23.sroa.16.0.hot_left.i68.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i68, i32 64
  %history.i46.i23.sroa.19.0.hot_left.i68.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i68, i32 80
  %history.i46.i23.sroa.22.0.hot_left.i68.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i68, i32 96
  %history.i46.i23.sroa.26.0.hot_left.i68.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i68, i32 112
  %history.i46.i23.sroa.29.0.hot_left.i68.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i68, i32 128
  %history.i46.i23.sroa.32.0.hot_left.i68.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i68, i32 144
  %history.i46.i23.sroa.35.0.hot_left.i68.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i68, i32 160
  %history.i46.i23.sroa.38.0.hot_left.i68.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i68, i32 176
  %1164 = getelementptr inbounds nuw i8, ptr %self, i32 32
  %1165 = getelementptr inbounds nuw i8, ptr %self, i32 48
  %1166 = getelementptr inbounds nuw i8, ptr %self, i32 64
  %row1.i.i.i79.i133 = getelementptr inbounds nuw i8, ptr %self, i32 80
  %1167 = getelementptr inbounds nuw i8, ptr %self, i32 96
  %1168 = getelementptr inbounds nuw i8, ptr %self, i32 112
  %1169 = getelementptr inbounds nuw i8, ptr %self, i32 128
  %row3.i.i.i93.i147 = getelementptr inbounds nuw i8, ptr %self, i32 144
  %1170 = getelementptr inbounds nuw i8, ptr %self, i32 160
  %1171 = getelementptr inbounds nuw i8, ptr %self, i32 176
  %1172 = getelementptr inbounds nuw i8, ptr %self, i32 192
  %row5.i.i.i107.i161 = getelementptr inbounds nuw i8, ptr %self, i32 208
  %1173 = getelementptr inbounds nuw i8, ptr %self, i32 224
  %1174 = getelementptr inbounds nuw i8, ptr %self, i32 240
  %1175 = getelementptr inbounds nuw i8, ptr %self, i32 256
  %row7.i.i.i121.i175 = getelementptr inbounds nuw i8, ptr %self, i32 272
  %1176 = getelementptr inbounds nuw i8, ptr %self, i32 288
  %1177 = getelementptr inbounds nuw i8, ptr %self, i32 304
  %1178 = getelementptr inbounds nuw i8, ptr %self, i32 320
  %row9.i.i.i135.i189 = getelementptr inbounds nuw i8, ptr %self, i32 336
  %1179 = getelementptr inbounds nuw i8, ptr %self, i32 352
  %1180 = getelementptr inbounds nuw i8, ptr %self, i32 368
  %1181 = getelementptr inbounds nuw i8, ptr %self, i32 384
  %row11.i.i.i149.i203 = getelementptr inbounds nuw i8, ptr %self, i32 400
  %1182 = getelementptr inbounds nuw i8, ptr %self, i32 416
  %1183 = getelementptr inbounds nuw i8, ptr %self, i32 432
  %1184 = getelementptr inbounds nuw i8, ptr %self, i32 448
  %row13.i.i.i163.i217 = getelementptr inbounds nuw i8, ptr %self, i32 464
  %1185 = getelementptr inbounds nuw i8, ptr %self, i32 480
  %1186 = getelementptr inbounds nuw i8, ptr %self, i32 496
  %1187 = getelementptr inbounds nuw i8, ptr %self, i32 512
  %row15.i.i.i177.i231 = getelementptr inbounds nuw i8, ptr %self, i32 528
  %1188 = getelementptr inbounds nuw i8, ptr %self, i32 544
  %1189 = getelementptr inbounds nuw i8, ptr %self, i32 560
  %1190 = getelementptr inbounds nuw i8, ptr %self, i32 576
  %row17.i.i.i191.i245 = getelementptr inbounds nuw i8, ptr %self, i32 592
  %1191 = getelementptr inbounds nuw i8, ptr %self, i32 608
  %1192 = getelementptr inbounds nuw i8, ptr %self, i32 624
  %1193 = getelementptr inbounds nuw i8, ptr %self, i32 640
  %row19.i.i.i205.i259 = getelementptr inbounds nuw i8, ptr %self, i32 656
  %1194 = getelementptr inbounds nuw i8, ptr %self, i32 672
  %1195 = getelementptr inbounds nuw i8, ptr %self, i32 688
  %1196 = getelementptr inbounds nuw i8, ptr %self, i32 704
  %row21.i.i.i219.i273 = getelementptr inbounds nuw i8, ptr %self, i32 720
  %1197 = getelementptr inbounds nuw i8, ptr %self, i32 736
  %1198 = getelementptr inbounds nuw i8, ptr %self, i32 752
  %1199 = getelementptr inbounds nuw i8, ptr %self, i32 768
  %history.i.i42.sroa.7.0.hot_right.i67.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i67, i32 16
  %history.i.i42.sroa.10.0.hot_right.i67.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i67, i32 32
  %history.i.i42.sroa.13.0.hot_right.i67.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i67, i32 48
  %history.i.i42.sroa.16.0.hot_right.i67.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i67, i32 64
  %history.i.i42.sroa.19.0.hot_right.i67.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i67, i32 80
  %history.i.i42.sroa.22.0.hot_right.i67.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i67, i32 96
  %history.i.i42.sroa.26.0.hot_right.i67.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i67, i32 112
  %history.i.i42.sroa.29.0.hot_right.i67.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i67, i32 128
  %history.i.i42.sroa.32.0.hot_right.i67.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i67, i32 144
  %history.i.i42.sroa.35.0.hot_right.i67.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i67, i32 160
  %history.i.i42.sroa.38.0.hot_right.i67.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i67, i32 176
  %1200 = getelementptr inbounds nuw i8, ptr %uniform_left.i63, i32 20
  %_72.i60.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i63, i32 24
  %_72.i60.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i63, i32 28
  %1201 = getelementptr inbounds nuw i8, ptr %uniform_right.i62, i32 20
  %_73.i59.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i62, i32 24
  %_73.i59.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i62, i32 28
  %_114.i549 = getelementptr inbounds nuw i8, ptr %hot_left.i68, i32 192
  %_115.i550 = getelementptr inbounds nuw i8, ptr %hot_left.i68, i32 256
  %1202 = getelementptr inbounds nuw i8, ptr %hot_left.i68, i32 240
  %1203 = getelementptr inbounds nuw i8, ptr %hot_left.i68, i32 224
  %1204 = getelementptr inbounds nuw i8, ptr %hot_left.i68, i32 208
  %1205 = getelementptr inbounds nuw i8, ptr %hot_left.i68, i32 304
  %1206 = getelementptr inbounds nuw i8, ptr %hot_left.i68, i32 288
  %1207 = getelementptr inbounds nuw i8, ptr %hot_left.i68, i32 272
  %_119.i553 = getelementptr inbounds nuw i8, ptr %hot_right.i67, i32 192
  %_120.i554 = getelementptr inbounds nuw i8, ptr %hot_right.i67, i32 256
  %1208 = getelementptr inbounds nuw i8, ptr %hot_right.i67, i32 240
  %1209 = getelementptr inbounds nuw i8, ptr %hot_right.i67, i32 224
  %1210 = getelementptr inbounds nuw i8, ptr %hot_right.i67, i32 208
  %1211 = getelementptr inbounds nuw i8, ptr %hot_right.i67, i32 304
  %1212 = getelementptr inbounds nuw i8, ptr %hot_right.i67, i32 288
  %1213 = getelementptr inbounds nuw i8, ptr %hot_right.i67, i32 272
  %1214 = bitcast <4 x i32> %link.sroa.0.0.i73 to <16 x i8>
  %1215 = getelementptr inbounds nuw i8, ptr %uniform_left.i63, i32 32
  %1216 = getelementptr inbounds nuw i8, ptr %uniform_left.i63, i32 36
  %_22.i269.i594 = getelementptr inbounds nuw i8, ptr %uniform_left.i63, i32 16
  %1217 = getelementptr inbounds nuw i8, ptr %uniform_left.i63, i32 40
  %1218 = getelementptr inbounds nuw i8, ptr %uniform_left.i63, i32 44
  %1219 = getelementptr inbounds nuw i8, ptr %hot_left.i68, i32 336
  %1220 = getelementptr inbounds nuw i8, ptr %hot_left.i68, i32 352
  %1221 = getelementptr inbounds nuw i8, ptr %hot_left.i68, i32 320
  %1222 = getelementptr inbounds nuw i8, ptr %uniform_left.i63, i32 52
  %1223 = getelementptr inbounds nuw i8, ptr %uniform_left.i63, i32 48
  %1224 = bitcast <4 x i32> %spec.store.select25.i75 to <16 x i8>
  %1225 = getelementptr inbounds nuw i8, ptr %uniform_right.i62, i32 32
  %1226 = getelementptr inbounds nuw i8, ptr %uniform_right.i62, i32 36
  %_22.i.i676 = getelementptr inbounds nuw i8, ptr %uniform_right.i62, i32 16
  %1227 = getelementptr inbounds nuw i8, ptr %uniform_right.i62, i32 40
  %1228 = getelementptr inbounds nuw i8, ptr %uniform_right.i62, i32 44
  %1229 = getelementptr inbounds nuw i8, ptr %hot_right.i67, i32 336
  %1230 = getelementptr inbounds nuw i8, ptr %hot_right.i67, i32 352
  %1231 = getelementptr inbounds nuw i8, ptr %hot_right.i67, i32 320
  %1232 = getelementptr inbounds nuw i8, ptr %uniform_right.i62, i32 52
  %1233 = getelementptr inbounds nuw i8, ptr %uniform_right.i62, i32 48
  br label %bb44.i91, !dbg !9172

bb15.i85.loopexit.loopexit:                       ; preds = %bb74.i734
  store i32 %storemerge.i1788.lcssa2305923107, ptr %_22.i269.i594, align 4
  store i32 %storemerge.i.lcssa2308923125, ptr %_22.i.i676, align 4
  br label %bb15.i85.loopexit, !dbg !9172

bb15.i85.loopexit:                                ; preds = %bb15.i85.loopexit.loopexit, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i497
  %ring_cursor.sroa.0.1.i499.lcssa = phi i32 [ %ring_cursor.sroa.0.0.i8618773, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i497 ], [ %ring_cursor.sroa.0.2.i737, %bb15.i85.loopexit.loopexit ], !dbg !9191
  %main_cursor.sroa.0.1.i500.lcssa = phi i32 [ %main_cursor.sroa.0.0.i8718774, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i497 ], [ %main_cursor.sroa.0.2.i740, %bb15.i85.loopexit.loopexit ], !dbg !9192
  %_166.not.i90 = icmp eq i32 %1235, 0, !dbg !9172
  %indvars.iv.next20950 = add i32 %indvars.iv20949, -32, !dbg !9172
  br i1 %_166.not.i90, label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb44.i91, !dbg !9172

bb44.i91:                                         ; preds = %bb44.i91.lr.ph, %bb15.i85.loopexit
  %indvars.iv20949 = phi i32 [ %frames, %bb44.i91.lr.ph ], [ %indvars.iv.next20950, %bb15.i85.loopexit ]
  %iter2.sroa.0.0.i8918776 = phi i32 [ %yield_count.sroa.0.0.i7200, %bb44.i91.lr.ph ], [ %1235, %bb15.i85.loopexit ]
  %iter1.sroa.0.0.i8818775 = phi i32 [ 0, %bb44.i91.lr.ph ], [ %1234, %bb15.i85.loopexit ]
  %main_cursor.sroa.0.0.i8718774 = phi i32 [ %_36.i78, %bb44.i91.lr.ph ], [ %main_cursor.sroa.0.1.i500.lcssa, %bb15.i85.loopexit ]
  %ring_cursor.sroa.0.0.i8618773 = phi i32 [ %_37.i79, %bb44.i91.lr.ph ], [ %ring_cursor.sroa.0.1.i499.lcssa, %bb15.i85.loopexit ]
  %umin20977 = call i32 @llvm.umin.i32(i32 %indvars.iv20949, i32 32), !dbg !9193
  %umax20955 = call i32 @llvm.umax.i32(i32 %umin20977, i32 1), !dbg !9193
  %1234 = add i32 %iter1.sroa.0.0.i8818775, 32, !dbg !9193
  %1235 = add nsw i32 %iter2.sroa.0.0.i8918776, -1, !dbg !9197
  %1236 = sub i32 %frames, %iter1.sroa.0.0.i8818775, !dbg !9198
  %spec.store.select.i92 = tail call i32 @llvm.umin.i32(i32 %1236, i32 32), !dbg !9200
  %active_base.i93 = shl i32 %iter1.sroa.0.0.i8818775, 2, !dbg !9205
  %active_base.i9316254 = add i32 %spec.store.select.i92, %iter1.sroa.0.0.i8818775, !dbg !9207
  %_52.i95 = shl i32 %active_base.i9316254, 2, !dbg !9207
  %_176.i96 = icmp ult i32 %_52.i95, %active_base.i93, !dbg !9210
  %_170.not.i97 = icmp ugt i32 %_52.i95, %left_io.1
  %or.cond.i98 = or i1 %_176.i96, %_170.not.i97, !dbg !9210
  br i1 %or.cond.i98, label %bb50.i744, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7219, !dbg !9210, !prof !4596

bb50.i744:                                        ; preds = %bb44.i91
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %active_base.i93, i32 noundef %_52.i95, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_b74a748963eaf51a410c7eb21835ee21) #32, !dbg !9217, !noalias !9134
  unreachable, !dbg !9217

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7219: ; preds = %bb44.i91
  %_179.i100 = getelementptr inbounds nuw float, ptr %left_io.0, i32 %active_base.i93, !dbg !9218
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9222), !dbg !9225
  %history.i46.i23.sroa.0.0.copyload = load <4 x i32>, ptr %hot_left.i68, align 16, !dbg !9226
  %history.i46.i23.sroa.7.0.copyload = load <4 x i32>, ptr %history.i46.i23.sroa.7.0.hot_left.i68.sroa_idx, align 16, !dbg !9226
  %history.i46.i23.sroa.10.0.copyload = load <4 x i32>, ptr %history.i46.i23.sroa.10.0.hot_left.i68.sroa_idx, align 16, !dbg !9226
  %history.i46.i23.sroa.13.0.copyload = load <4 x i32>, ptr %history.i46.i23.sroa.13.0.hot_left.i68.sroa_idx, align 16, !dbg !9226
  %history.i46.i23.sroa.16.0.copyload = load <4 x i32>, ptr %history.i46.i23.sroa.16.0.hot_left.i68.sroa_idx, align 16, !dbg !9226
  %history.i46.i23.sroa.19.0.copyload = load <4 x i32>, ptr %history.i46.i23.sroa.19.0.hot_left.i68.sroa_idx, align 16, !dbg !9226
  %history.i46.i23.sroa.22.0.copyload = load <4 x i32>, ptr %history.i46.i23.sroa.22.0.hot_left.i68.sroa_idx, align 16, !dbg !9226
  %history.i46.i23.sroa.26.0.copyload = load <4 x i32>, ptr %history.i46.i23.sroa.26.0.hot_left.i68.sroa_idx, align 16, !dbg !9226
  %history.i46.i23.sroa.29.0.copyload = load <4 x i32>, ptr %history.i46.i23.sroa.29.0.hot_left.i68.sroa_idx, align 16, !dbg !9226
  %history.i46.i23.sroa.32.0.copyload = load <4 x i32>, ptr %history.i46.i23.sroa.32.0.hot_left.i68.sroa_idx, align 16, !dbg !9226
  %history.i46.i23.sroa.35.0.copyload = load <4 x i32>, ptr %history.i46.i23.sroa.35.0.hot_left.i68.sroa_idx, align 16, !dbg !9226
  %history.i46.i23.sroa.38.0.copyload = load <4 x i32>, ptr %history.i46.i23.sroa.38.0.hot_left.i68.sroa_idx, align 16, !dbg !9226
  %_2.i722218705.not = icmp eq i32 %frames, %iter1.sroa.0.0.i8818775, !dbg !9228
  br i1 %_2.i722218705.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit243.i297, label %bb5.i49.i103.lr.ph, !dbg !9228

bb5.i49.i103.lr.ph:                               ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7219
  %_11.i.i.i67.i12116353 = load <4 x float>, ptr %_31, align 16, !alias.scope !9231, !noalias !9236
  %_14.i.i.i70.i12416354 = load <4 x float>, ptr %1164, align 16, !alias.scope !9231, !noalias !9236
  %_17.i.i.i73.i12716355 = load <4 x float>, ptr %1165, align 16, !alias.scope !9231, !noalias !9236
  %_20.i.i.i76.i13016356 = load <4 x float>, ptr %1166, align 16, !alias.scope !9231, !noalias !9236
  %_25.i.i.i81.i13516357 = load <4 x float>, ptr %row1.i.i.i79.i133, align 16, !alias.scope !9231, !noalias !9236
  %_28.i.i.i84.i13816358 = load <4 x float>, ptr %1167, align 16, !alias.scope !9231, !noalias !9236
  %_31.i.i.i87.i14116359 = load <4 x float>, ptr %1168, align 16, !alias.scope !9231, !noalias !9236
  %_34.i.i.i90.i14416360 = load <4 x float>, ptr %1169, align 16, !alias.scope !9231, !noalias !9236
  %_39.i.i.i95.i14916361 = load <4 x float>, ptr %row3.i.i.i93.i147, align 16, !alias.scope !9231, !noalias !9236
  %_42.i.i.i98.i15216362 = load <4 x float>, ptr %1170, align 16, !alias.scope !9231, !noalias !9236
  %_45.i.i.i101.i15516363 = load <4 x float>, ptr %1171, align 16, !alias.scope !9231, !noalias !9236
  %_48.i.i.i104.i15816364 = load <4 x float>, ptr %1172, align 16, !alias.scope !9231, !noalias !9236
  %_53.i.i.i109.i16316365 = load <4 x float>, ptr %row5.i.i.i107.i161, align 16, !alias.scope !9231, !noalias !9236
  %_56.i.i.i112.i16616366 = load <4 x float>, ptr %1173, align 16, !alias.scope !9231, !noalias !9236
  %_59.i.i.i115.i16916367 = load <4 x float>, ptr %1174, align 16, !alias.scope !9231, !noalias !9236
  %_62.i.i.i118.i17216368 = load <4 x float>, ptr %1175, align 16, !alias.scope !9231, !noalias !9236
  %_67.i.i.i123.i17716369 = load <4 x float>, ptr %row7.i.i.i121.i175, align 16, !alias.scope !9231, !noalias !9236
  %_70.i.i.i126.i18016370 = load <4 x float>, ptr %1176, align 16, !alias.scope !9231, !noalias !9236
  %_73.i.i.i129.i18316371 = load <4 x float>, ptr %1177, align 16, !alias.scope !9231, !noalias !9236
  %_76.i.i.i132.i18616372 = load <4 x float>, ptr %1178, align 16, !alias.scope !9231, !noalias !9236
  %_81.i.i.i137.i19116373 = load <4 x float>, ptr %row9.i.i.i135.i189, align 16, !alias.scope !9231, !noalias !9236
  %_84.i.i.i140.i19416374 = load <4 x float>, ptr %1179, align 16, !alias.scope !9231, !noalias !9236
  %_87.i.i.i143.i19716375 = load <4 x float>, ptr %1180, align 16, !alias.scope !9231, !noalias !9236
  %_90.i.i.i146.i20016376 = load <4 x float>, ptr %1181, align 16, !alias.scope !9231, !noalias !9236
  %_95.i.i.i151.i20516377 = load <4 x float>, ptr %row11.i.i.i149.i203, align 16, !alias.scope !9231, !noalias !9236
  %_98.i.i.i154.i20816378 = load <4 x float>, ptr %1182, align 16, !alias.scope !9231, !noalias !9236
  %_101.i.i.i157.i21116379 = load <4 x float>, ptr %1183, align 16, !alias.scope !9231, !noalias !9236
  %_104.i.i.i160.i21416380 = load <4 x float>, ptr %1184, align 16, !alias.scope !9231, !noalias !9236
  %_109.i.i.i165.i21916381 = load <4 x float>, ptr %row13.i.i.i163.i217, align 16, !alias.scope !9231, !noalias !9236
  %_112.i.i.i168.i22216382 = load <4 x float>, ptr %1185, align 16, !alias.scope !9231, !noalias !9236
  %_115.i.i.i171.i22516383 = load <4 x float>, ptr %1186, align 16, !alias.scope !9231, !noalias !9236
  %_118.i.i.i174.i22816384 = load <4 x float>, ptr %1187, align 16, !alias.scope !9231, !noalias !9236
  %_123.i.i.i179.i23316385 = load <4 x float>, ptr %row15.i.i.i177.i231, align 16, !alias.scope !9231, !noalias !9236
  %_126.i.i.i182.i23616386 = load <4 x float>, ptr %1188, align 16, !alias.scope !9231, !noalias !9236
  %_129.i.i.i185.i23916387 = load <4 x float>, ptr %1189, align 16, !alias.scope !9231, !noalias !9236
  %_132.i.i.i188.i24216388 = load <4 x float>, ptr %1190, align 16, !alias.scope !9231, !noalias !9236
  %_137.i.i.i193.i24716389 = load <4 x float>, ptr %row17.i.i.i191.i245, align 16, !alias.scope !9231, !noalias !9236
  %_140.i.i.i196.i25016390 = load <4 x float>, ptr %1191, align 16, !alias.scope !9231, !noalias !9236
  %_143.i.i.i199.i25316391 = load <4 x float>, ptr %1192, align 16, !alias.scope !9231, !noalias !9236
  %_146.i.i.i202.i25616392 = load <4 x float>, ptr %1193, align 16, !alias.scope !9231, !noalias !9236
  %_151.i.i.i207.i26116393 = load <4 x float>, ptr %row19.i.i.i205.i259, align 16, !alias.scope !9231, !noalias !9236
  %_154.i.i.i210.i26416394 = load <4 x float>, ptr %1194, align 16, !alias.scope !9231, !noalias !9236
  %_157.i.i.i213.i26716395 = load <4 x float>, ptr %1195, align 16, !alias.scope !9231, !noalias !9236
  %_160.i.i.i216.i27016396 = load <4 x float>, ptr %1196, align 16, !alias.scope !9231, !noalias !9236
  %_165.i.i.i221.i27516397 = load <4 x float>, ptr %row21.i.i.i219.i273, align 16, !alias.scope !9231, !noalias !9236
  %_168.i.i.i224.i27816398 = load <4 x float>, ptr %1197, align 16, !alias.scope !9231, !noalias !9236
  %_171.i.i.i227.i28116399 = load <4 x float>, ptr %1198, align 16, !alias.scope !9231, !noalias !9236
  %_174.i.i.i230.i28416400 = load <4 x float>, ptr %1199, align 16, !alias.scope !9231, !noalias !9236
  br label %bb5.i49.i103, !dbg !9228

bb5.i49.i103:                                     ; preds = %bb5.i49.i103.lr.ph, %bb5.i49.i103
  %iter.i42.i19.sroa.16.018717 = phi i32 [ 0, %bb5.i49.i103.lr.ph ], [ %1358, %bb5.i49.i103 ]
  %history.i46.i23.sroa.35.018716 = phi <4 x i32> [ %history.i46.i23.sroa.35.0.copyload, %bb5.i49.i103.lr.ph ], [ %history.i46.i23.sroa.32.018715, %bb5.i49.i103 ]
  %history.i46.i23.sroa.32.018715 = phi <4 x i32> [ %history.i46.i23.sroa.32.0.copyload, %bb5.i49.i103.lr.ph ], [ %history.i46.i23.sroa.29.018714, %bb5.i49.i103 ]
  %history.i46.i23.sroa.29.018714 = phi <4 x i32> [ %history.i46.i23.sroa.29.0.copyload, %bb5.i49.i103.lr.ph ], [ %history.i46.i23.sroa.26.018713, %bb5.i49.i103 ]
  %history.i46.i23.sroa.26.018713 = phi <4 x i32> [ %history.i46.i23.sroa.26.0.copyload, %bb5.i49.i103.lr.ph ], [ %history.i46.i23.sroa.22.018712, %bb5.i49.i103 ]
  %history.i46.i23.sroa.22.018712 = phi <4 x i32> [ %history.i46.i23.sroa.22.0.copyload, %bb5.i49.i103.lr.ph ], [ %history.i46.i23.sroa.19.018711, %bb5.i49.i103 ]
  %history.i46.i23.sroa.19.018711 = phi <4 x i32> [ %history.i46.i23.sroa.19.0.copyload, %bb5.i49.i103.lr.ph ], [ %history.i46.i23.sroa.16.018710, %bb5.i49.i103 ]
  %history.i46.i23.sroa.16.018710 = phi <4 x i32> [ %history.i46.i23.sroa.16.0.copyload, %bb5.i49.i103.lr.ph ], [ %history.i46.i23.sroa.13.018709, %bb5.i49.i103 ]
  %history.i46.i23.sroa.13.018709 = phi <4 x i32> [ %history.i46.i23.sroa.13.0.copyload, %bb5.i49.i103.lr.ph ], [ %history.i46.i23.sroa.10.018708, %bb5.i49.i103 ]
  %history.i46.i23.sroa.10.018708 = phi <4 x i32> [ %history.i46.i23.sroa.10.0.copyload, %bb5.i49.i103.lr.ph ], [ %history.i46.i23.sroa.7.018707, %bb5.i49.i103 ]
  %history.i46.i23.sroa.7.018707 = phi <4 x i32> [ %history.i46.i23.sroa.7.0.copyload, %bb5.i49.i103.lr.ph ], [ %history.i46.i23.sroa.0.018706, %bb5.i49.i103 ]
  %history.i46.i23.sroa.0.018706 = phi <4 x i32> [ %history.i46.i23.sroa.0.0.copyload, %bb5.i49.i103.lr.ph ], [ %lanes.i5889.sroa.0.0.copyload, %bb5.i49.i103 ]
  %start1.i.i7228 = shl i32 %iter.i42.i19.sroa.16.018717, 2, !dbg !9245
  %data.i.i7229 = getelementptr inbounds nuw float, ptr %_179.i100, i32 %start1.i.i7228, !dbg !9247
  %lanes.i5889.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i7229, align 4, !dbg !9249, !alias.scope !9254, !noalias !9258
  %1237 = bitcast <4 x i32> %history.i46.i23.sroa.19.018711 to <4 x float>, !dbg !9262
  %1238 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1237), !dbg !9267
  %1239 = bitcast <4 x i32> %lanes.i5889.sroa.0.0.copyload to <4 x float>, !dbg !9268
  %1240 = fmul <4 x float> %_11.i.i.i67.i12116353, %1239, !dbg !9273
  %1241 = fadd <4 x float> %1240, zeroinitializer, !dbg !9274
  %1242 = fmul <4 x float> %_14.i.i.i70.i12416354, %1239, !dbg !9278
  %1243 = fadd <4 x float> %1242, zeroinitializer, !dbg !9282
  %1244 = fmul <4 x float> %_17.i.i.i73.i12716355, %1239, !dbg !9286
  %1245 = fadd <4 x float> %1244, zeroinitializer, !dbg !9290
  %1246 = fmul <4 x float> %_20.i.i.i76.i13016356, %1239, !dbg !9294
  %1247 = fadd <4 x float> %1246, zeroinitializer, !dbg !9298
  %1248 = bitcast <4 x i32> %history.i46.i23.sroa.0.018706 to <4 x float>, !dbg !9302
  %1249 = fmul <4 x float> %_25.i.i.i81.i13516357, %1248, !dbg !9306
  %1250 = fadd <4 x float> %1241, %1249, !dbg !9307
  %1251 = fmul <4 x float> %_28.i.i.i84.i13816358, %1248, !dbg !9311
  %1252 = fadd <4 x float> %1243, %1251, !dbg !9315
  %1253 = fmul <4 x float> %_31.i.i.i87.i14116359, %1248, !dbg !9319
  %1254 = fadd <4 x float> %1245, %1253, !dbg !9323
  %1255 = fmul <4 x float> %_34.i.i.i90.i14416360, %1248, !dbg !9327
  %1256 = fadd <4 x float> %1247, %1255, !dbg !9331
  %1257 = bitcast <4 x i32> %history.i46.i23.sroa.7.018707 to <4 x float>, !dbg !9335
  %1258 = fmul <4 x float> %_39.i.i.i95.i14916361, %1257, !dbg !9339
  %1259 = fadd <4 x float> %1250, %1258, !dbg !9340
  %1260 = fmul <4 x float> %_42.i.i.i98.i15216362, %1257, !dbg !9344
  %1261 = fadd <4 x float> %1252, %1260, !dbg !9348
  %1262 = fmul <4 x float> %_45.i.i.i101.i15516363, %1257, !dbg !9352
  %1263 = fadd <4 x float> %1254, %1262, !dbg !9356
  %1264 = fmul <4 x float> %_48.i.i.i104.i15816364, %1257, !dbg !9360
  %1265 = fadd <4 x float> %1256, %1264, !dbg !9364
  %1266 = bitcast <4 x i32> %history.i46.i23.sroa.10.018708 to <4 x float>, !dbg !9368
  %1267 = fmul <4 x float> %_53.i.i.i109.i16316365, %1266, !dbg !9372
  %1268 = fadd <4 x float> %1259, %1267, !dbg !9373
  %1269 = fmul <4 x float> %_56.i.i.i112.i16616366, %1266, !dbg !9377
  %1270 = fadd <4 x float> %1261, %1269, !dbg !9381
  %1271 = fmul <4 x float> %_59.i.i.i115.i16916367, %1266, !dbg !9385
  %1272 = fadd <4 x float> %1263, %1271, !dbg !9389
  %1273 = fmul <4 x float> %_62.i.i.i118.i17216368, %1266, !dbg !9393
  %1274 = fadd <4 x float> %1265, %1273, !dbg !9397
  %1275 = bitcast <4 x i32> %history.i46.i23.sroa.13.018709 to <4 x float>, !dbg !9401
  %1276 = fmul <4 x float> %_67.i.i.i123.i17716369, %1275, !dbg !9405
  %1277 = fadd <4 x float> %1268, %1276, !dbg !9406
  %1278 = fmul <4 x float> %_70.i.i.i126.i18016370, %1275, !dbg !9410
  %1279 = fadd <4 x float> %1270, %1278, !dbg !9414
  %1280 = fmul <4 x float> %_73.i.i.i129.i18316371, %1275, !dbg !9418
  %1281 = fadd <4 x float> %1272, %1280, !dbg !9422
  %1282 = fmul <4 x float> %_76.i.i.i132.i18616372, %1275, !dbg !9426
  %1283 = fadd <4 x float> %1274, %1282, !dbg !9430
  %1284 = bitcast <4 x i32> %history.i46.i23.sroa.16.018710 to <4 x float>, !dbg !9434
  %1285 = fmul <4 x float> %_81.i.i.i137.i19116373, %1284, !dbg !9438
  %1286 = fadd <4 x float> %1277, %1285, !dbg !9439
  %1287 = fmul <4 x float> %_84.i.i.i140.i19416374, %1284, !dbg !9443
  %1288 = fadd <4 x float> %1279, %1287, !dbg !9447
  %1289 = fmul <4 x float> %_87.i.i.i143.i19716375, %1284, !dbg !9451
  %1290 = fadd <4 x float> %1281, %1289, !dbg !9455
  %1291 = fmul <4 x float> %_90.i.i.i146.i20016376, %1284, !dbg !9459
  %1292 = fadd <4 x float> %1283, %1291, !dbg !9463
  %1293 = fmul <4 x float> %_95.i.i.i151.i20516377, %1237, !dbg !9467
  %1294 = fadd <4 x float> %1286, %1293, !dbg !9471
  %1295 = fmul <4 x float> %_98.i.i.i154.i20816378, %1237, !dbg !9475
  %1296 = fadd <4 x float> %1288, %1295, !dbg !9479
  %1297 = fmul <4 x float> %_101.i.i.i157.i21116379, %1237, !dbg !9483
  %1298 = fadd <4 x float> %1290, %1297, !dbg !9487
  %1299 = fmul <4 x float> %_104.i.i.i160.i21416380, %1237, !dbg !9491
  %1300 = fadd <4 x float> %1292, %1299, !dbg !9495
  %1301 = bitcast <4 x i32> %history.i46.i23.sroa.22.018712 to <4 x float>, !dbg !9499
  %1302 = fmul <4 x float> %_109.i.i.i165.i21916381, %1301, !dbg !9503
  %1303 = fadd <4 x float> %1294, %1302, !dbg !9504
  %1304 = fmul <4 x float> %_112.i.i.i168.i22216382, %1301, !dbg !9508
  %1305 = fadd <4 x float> %1296, %1304, !dbg !9512
  %1306 = fmul <4 x float> %_115.i.i.i171.i22516383, %1301, !dbg !9516
  %1307 = fadd <4 x float> %1298, %1306, !dbg !9520
  %1308 = fmul <4 x float> %_118.i.i.i174.i22816384, %1301, !dbg !9524
  %1309 = fadd <4 x float> %1300, %1308, !dbg !9528
  %1310 = bitcast <4 x i32> %history.i46.i23.sroa.26.018713 to <4 x float>, !dbg !9532
  %1311 = fmul <4 x float> %_123.i.i.i179.i23316385, %1310, !dbg !9536
  %1312 = fadd <4 x float> %1303, %1311, !dbg !9537
  %1313 = fmul <4 x float> %_126.i.i.i182.i23616386, %1310, !dbg !9541
  %1314 = fadd <4 x float> %1305, %1313, !dbg !9545
  %1315 = fmul <4 x float> %_129.i.i.i185.i23916387, %1310, !dbg !9549
  %1316 = fadd <4 x float> %1307, %1315, !dbg !9553
  %1317 = fmul <4 x float> %_132.i.i.i188.i24216388, %1310, !dbg !9557
  %1318 = fadd <4 x float> %1309, %1317, !dbg !9561
  %1319 = bitcast <4 x i32> %history.i46.i23.sroa.29.018714 to <4 x float>, !dbg !9565
  %1320 = fmul <4 x float> %_137.i.i.i193.i24716389, %1319, !dbg !9569
  %1321 = fadd <4 x float> %1312, %1320, !dbg !9570
  %1322 = fmul <4 x float> %_140.i.i.i196.i25016390, %1319, !dbg !9574
  %1323 = fadd <4 x float> %1314, %1322, !dbg !9578
  %1324 = fmul <4 x float> %_143.i.i.i199.i25316391, %1319, !dbg !9582
  %1325 = fadd <4 x float> %1316, %1324, !dbg !9586
  %1326 = fmul <4 x float> %_146.i.i.i202.i25616392, %1319, !dbg !9590
  %1327 = fadd <4 x float> %1318, %1326, !dbg !9594
  %1328 = bitcast <4 x i32> %history.i46.i23.sroa.32.018715 to <4 x float>, !dbg !9598
  %1329 = fmul <4 x float> %_151.i.i.i207.i26116393, %1328, !dbg !9602
  %1330 = fadd <4 x float> %1321, %1329, !dbg !9603
  %1331 = fmul <4 x float> %_154.i.i.i210.i26416394, %1328, !dbg !9607
  %1332 = fadd <4 x float> %1323, %1331, !dbg !9611
  %1333 = fmul <4 x float> %_157.i.i.i213.i26716395, %1328, !dbg !9615
  %1334 = fadd <4 x float> %1325, %1333, !dbg !9619
  %1335 = fmul <4 x float> %_160.i.i.i216.i27016396, %1328, !dbg !9623
  %1336 = fadd <4 x float> %1327, %1335, !dbg !9627
  %1337 = bitcast <4 x i32> %history.i46.i23.sroa.35.018716 to <4 x float>, !dbg !9631
  %1338 = fmul <4 x float> %_165.i.i.i221.i27516397, %1337, !dbg !9635
  %1339 = fadd <4 x float> %1330, %1338, !dbg !9636
  %1340 = fmul <4 x float> %_168.i.i.i224.i27816398, %1337, !dbg !9640
  %1341 = fadd <4 x float> %1332, %1340, !dbg !9644
  %1342 = fmul <4 x float> %_171.i.i.i227.i28116399, %1337, !dbg !9648
  %1343 = fadd <4 x float> %1334, %1342, !dbg !9652
  %1344 = fmul <4 x float> %_174.i.i.i230.i28416400, %1337, !dbg !9656
  %1345 = fadd <4 x float> %1336, %1344, !dbg !9660
  %1346 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1339), !dbg !9664
  %1347 = fcmp olt <4 x float> %1346, %1238, !dbg !9668
  %1348 = select <4 x i1> %1347, <4 x float> %1238, <4 x float> %1346, !dbg !9672
  %1349 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1341), !dbg !9664
  %1350 = fcmp olt <4 x float> %1349, %1348, !dbg !9668
  %1351 = select <4 x i1> %1350, <4 x float> %1348, <4 x float> %1349, !dbg !9672
  %1352 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1343), !dbg !9664
  %1353 = fcmp olt <4 x float> %1352, %1351, !dbg !9668
  %1354 = select <4 x i1> %1353, <4 x float> %1351, <4 x float> %1352, !dbg !9672
  %1355 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1345), !dbg !9664
  %1356 = fcmp olt <4 x float> %1355, %1354, !dbg !9668
  %1357 = select <4 x i1> %1356, <4 x float> %1354, <4 x float> %1355, !dbg !9672
  %1358 = add nuw nsw i32 %iter.i42.i19.sroa.16.018717, 1, !dbg !9673
  %data.i4.i7233 = getelementptr inbounds nuw float, ptr %peaks_left.i65, i32 %start1.i.i7228, !dbg !9674
  store <4 x float> %1357, ptr %data.i4.i7233, align 4, !dbg !9677, !alias.scope !9682, !noalias !9686
  %exitcond20953.not = icmp eq i32 %1358, %umax20955, !dbg !9228
  br i1 %exitcond20953.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit243.i297, label %bb5.i49.i103, !dbg !9228

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit243.i297: ; preds = %bb5.i49.i103, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7219
  %history.i46.i23.sroa.0.0.lcssa = phi <4 x i32> [ %history.i46.i23.sroa.0.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7219 ], [ %lanes.i5889.sroa.0.0.copyload, %bb5.i49.i103 ], !dbg !9690
  %history.i46.i23.sroa.7.0.lcssa = phi <4 x i32> [ %history.i46.i23.sroa.7.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7219 ], [ %history.i46.i23.sroa.0.018706, %bb5.i49.i103 ], !dbg !9690
  %history.i46.i23.sroa.10.0.lcssa = phi <4 x i32> [ %history.i46.i23.sroa.10.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7219 ], [ %history.i46.i23.sroa.7.018707, %bb5.i49.i103 ], !dbg !9690
  %history.i46.i23.sroa.13.0.lcssa = phi <4 x i32> [ %history.i46.i23.sroa.13.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7219 ], [ %history.i46.i23.sroa.10.018708, %bb5.i49.i103 ], !dbg !9690
  %history.i46.i23.sroa.16.0.lcssa = phi <4 x i32> [ %history.i46.i23.sroa.16.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7219 ], [ %history.i46.i23.sroa.13.018709, %bb5.i49.i103 ], !dbg !9690
  %history.i46.i23.sroa.19.0.lcssa = phi <4 x i32> [ %history.i46.i23.sroa.19.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7219 ], [ %history.i46.i23.sroa.16.018710, %bb5.i49.i103 ], !dbg !9690
  %history.i46.i23.sroa.22.0.lcssa = phi <4 x i32> [ %history.i46.i23.sroa.22.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7219 ], [ %history.i46.i23.sroa.19.018711, %bb5.i49.i103 ], !dbg !9690
  %history.i46.i23.sroa.26.0.lcssa = phi <4 x i32> [ %history.i46.i23.sroa.26.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7219 ], [ %history.i46.i23.sroa.22.018712, %bb5.i49.i103 ], !dbg !9690
  %history.i46.i23.sroa.29.0.lcssa = phi <4 x i32> [ %history.i46.i23.sroa.29.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7219 ], [ %history.i46.i23.sroa.26.018713, %bb5.i49.i103 ], !dbg !9690
  %history.i46.i23.sroa.32.0.lcssa = phi <4 x i32> [ %history.i46.i23.sroa.32.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7219 ], [ %history.i46.i23.sroa.29.018714, %bb5.i49.i103 ], !dbg !9690
  %history.i46.i23.sroa.35.0.lcssa = phi <4 x i32> [ %history.i46.i23.sroa.35.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7219 ], [ %history.i46.i23.sroa.32.018715, %bb5.i49.i103 ], !dbg !9690
  %history.i46.i23.sroa.38.0.lcssa = phi <4 x i32> [ %history.i46.i23.sroa.38.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7219 ], [ %history.i46.i23.sroa.35.018716, %bb5.i49.i103 ], !dbg !9690
  store <4 x i32> %history.i46.i23.sroa.0.0.lcssa, ptr %hot_left.i68, align 16, !dbg !9691
  store <4 x i32> %history.i46.i23.sroa.7.0.lcssa, ptr %history.i46.i23.sroa.7.0.hot_left.i68.sroa_idx, align 16, !dbg !9691
  store <4 x i32> %history.i46.i23.sroa.10.0.lcssa, ptr %history.i46.i23.sroa.10.0.hot_left.i68.sroa_idx, align 16, !dbg !9691
  store <4 x i32> %history.i46.i23.sroa.13.0.lcssa, ptr %history.i46.i23.sroa.13.0.hot_left.i68.sroa_idx, align 16, !dbg !9691
  store <4 x i32> %history.i46.i23.sroa.16.0.lcssa, ptr %history.i46.i23.sroa.16.0.hot_left.i68.sroa_idx, align 16, !dbg !9691
  store <4 x i32> %history.i46.i23.sroa.19.0.lcssa, ptr %history.i46.i23.sroa.19.0.hot_left.i68.sroa_idx, align 16, !dbg !9691
  store <4 x i32> %history.i46.i23.sroa.22.0.lcssa, ptr %history.i46.i23.sroa.22.0.hot_left.i68.sroa_idx, align 16, !dbg !9691
  store <4 x i32> %history.i46.i23.sroa.26.0.lcssa, ptr %history.i46.i23.sroa.26.0.hot_left.i68.sroa_idx, align 16, !dbg !9691
  store <4 x i32> %history.i46.i23.sroa.29.0.lcssa, ptr %history.i46.i23.sroa.29.0.hot_left.i68.sroa_idx, align 16, !dbg !9691
  store <4 x i32> %history.i46.i23.sroa.32.0.lcssa, ptr %history.i46.i23.sroa.32.0.hot_left.i68.sroa_idx, align 16, !dbg !9691
  store <4 x i32> %history.i46.i23.sroa.35.0.lcssa, ptr %history.i46.i23.sroa.35.0.hot_left.i68.sroa_idx, align 16, !dbg !9691
  store <4 x i32> %history.i46.i23.sroa.38.0.lcssa, ptr %history.i46.i23.sroa.38.0.hot_left.i68.sroa_idx, align 16, !dbg !9691
  %_187.not.i298 = icmp ugt i32 %_52.i95, %right_io.1, !dbg !9692
  br i1 %_187.not.i298, label %bb56.i743, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7269, !dbg !9692, !prof !787

bb56.i743:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit243.i297
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %active_base.i93, i32 noundef %_52.i95, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_8715d54bb9ab90680506cd3587af9682) #32, !dbg !9696, !noalias !9134
  unreachable, !dbg !9696

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7269: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit243.i297
  %_194.i300 = getelementptr inbounds nuw float, ptr %right_io.0, i32 %active_base.i93, !dbg !9697
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9701), !dbg !9704
  %history.i.i42.sroa.0.0.copyload = load <4 x i32>, ptr %hot_right.i67, align 16, !dbg !9705
  %history.i.i42.sroa.7.0.copyload = load <4 x i32>, ptr %history.i.i42.sroa.7.0.hot_right.i67.sroa_idx, align 16, !dbg !9705
  %history.i.i42.sroa.10.0.copyload = load <4 x i32>, ptr %history.i.i42.sroa.10.0.hot_right.i67.sroa_idx, align 16, !dbg !9705
  %history.i.i42.sroa.13.0.copyload = load <4 x i32>, ptr %history.i.i42.sroa.13.0.hot_right.i67.sroa_idx, align 16, !dbg !9705
  %history.i.i42.sroa.16.0.copyload = load <4 x i32>, ptr %history.i.i42.sroa.16.0.hot_right.i67.sroa_idx, align 16, !dbg !9705
  %history.i.i42.sroa.19.0.copyload = load <4 x i32>, ptr %history.i.i42.sroa.19.0.hot_right.i67.sroa_idx, align 16, !dbg !9705
  %history.i.i42.sroa.22.0.copyload = load <4 x i32>, ptr %history.i.i42.sroa.22.0.hot_right.i67.sroa_idx, align 16, !dbg !9705
  %history.i.i42.sroa.26.0.copyload = load <4 x i32>, ptr %history.i.i42.sroa.26.0.hot_right.i67.sroa_idx, align 16, !dbg !9705
  %history.i.i42.sroa.29.0.copyload = load <4 x i32>, ptr %history.i.i42.sroa.29.0.hot_right.i67.sroa_idx, align 16, !dbg !9705
  %history.i.i42.sroa.32.0.copyload = load <4 x i32>, ptr %history.i.i42.sroa.32.0.hot_right.i67.sroa_idx, align 16, !dbg !9705
  %history.i.i42.sroa.35.0.copyload = load <4 x i32>, ptr %history.i.i42.sroa.35.0.hot_right.i67.sroa_idx, align 16, !dbg !9705
  %history.i.i42.sroa.38.0.copyload = load <4 x i32>, ptr %history.i.i42.sroa.38.0.hot_right.i67.sroa_idx, align 16, !dbg !9705
  br i1 %_2.i722218705.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i497, label %bb5.i.i303.lr.ph, !dbg !9707

bb5.i.i303.lr.ph:                                 ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7269
  %_11.i.i.i.i32116300 = load <4 x float>, ptr %_31, align 16, !alias.scope !9710, !noalias !9715
  %_14.i.i.i.i32416301 = load <4 x float>, ptr %1164, align 16, !alias.scope !9710, !noalias !9715
  %_17.i.i.i.i32716302 = load <4 x float>, ptr %1165, align 16, !alias.scope !9710, !noalias !9715
  %_20.i.i.i.i33016303 = load <4 x float>, ptr %1166, align 16, !alias.scope !9710, !noalias !9715
  %_25.i.i.i.i33516304 = load <4 x float>, ptr %row1.i.i.i79.i133, align 16, !alias.scope !9710, !noalias !9715
  %_28.i.i.i.i33816305 = load <4 x float>, ptr %1167, align 16, !alias.scope !9710, !noalias !9715
  %_31.i.i.i.i34116306 = load <4 x float>, ptr %1168, align 16, !alias.scope !9710, !noalias !9715
  %_34.i.i.i.i34416307 = load <4 x float>, ptr %1169, align 16, !alias.scope !9710, !noalias !9715
  %_39.i.i.i.i34916308 = load <4 x float>, ptr %row3.i.i.i93.i147, align 16, !alias.scope !9710, !noalias !9715
  %_42.i.i.i.i35216309 = load <4 x float>, ptr %1170, align 16, !alias.scope !9710, !noalias !9715
  %_45.i.i.i.i35516310 = load <4 x float>, ptr %1171, align 16, !alias.scope !9710, !noalias !9715
  %_48.i.i.i.i35816311 = load <4 x float>, ptr %1172, align 16, !alias.scope !9710, !noalias !9715
  %_53.i.i.i.i36316312 = load <4 x float>, ptr %row5.i.i.i107.i161, align 16, !alias.scope !9710, !noalias !9715
  %_56.i.i.i.i36616313 = load <4 x float>, ptr %1173, align 16, !alias.scope !9710, !noalias !9715
  %_59.i.i.i.i36916314 = load <4 x float>, ptr %1174, align 16, !alias.scope !9710, !noalias !9715
  %_62.i.i.i.i37216315 = load <4 x float>, ptr %1175, align 16, !alias.scope !9710, !noalias !9715
  %_67.i.i.i.i37716316 = load <4 x float>, ptr %row7.i.i.i121.i175, align 16, !alias.scope !9710, !noalias !9715
  %_70.i.i.i.i38016317 = load <4 x float>, ptr %1176, align 16, !alias.scope !9710, !noalias !9715
  %_73.i.i.i.i38316318 = load <4 x float>, ptr %1177, align 16, !alias.scope !9710, !noalias !9715
  %_76.i.i.i.i38616319 = load <4 x float>, ptr %1178, align 16, !alias.scope !9710, !noalias !9715
  %_81.i.i.i.i39116320 = load <4 x float>, ptr %row9.i.i.i135.i189, align 16, !alias.scope !9710, !noalias !9715
  %_84.i.i.i.i39416321 = load <4 x float>, ptr %1179, align 16, !alias.scope !9710, !noalias !9715
  %_87.i.i.i.i39716322 = load <4 x float>, ptr %1180, align 16, !alias.scope !9710, !noalias !9715
  %_90.i.i.i.i40016323 = load <4 x float>, ptr %1181, align 16, !alias.scope !9710, !noalias !9715
  %_95.i.i.i.i40516324 = load <4 x float>, ptr %row11.i.i.i149.i203, align 16, !alias.scope !9710, !noalias !9715
  %_98.i.i.i.i40816325 = load <4 x float>, ptr %1182, align 16, !alias.scope !9710, !noalias !9715
  %_101.i.i.i.i41116326 = load <4 x float>, ptr %1183, align 16, !alias.scope !9710, !noalias !9715
  %_104.i.i.i.i41416327 = load <4 x float>, ptr %1184, align 16, !alias.scope !9710, !noalias !9715
  %_109.i.i.i.i41916328 = load <4 x float>, ptr %row13.i.i.i163.i217, align 16, !alias.scope !9710, !noalias !9715
  %_112.i.i.i.i42216329 = load <4 x float>, ptr %1185, align 16, !alias.scope !9710, !noalias !9715
  %_115.i.i.i.i42516330 = load <4 x float>, ptr %1186, align 16, !alias.scope !9710, !noalias !9715
  %_118.i.i.i.i42816331 = load <4 x float>, ptr %1187, align 16, !alias.scope !9710, !noalias !9715
  %_123.i.i.i.i43316332 = load <4 x float>, ptr %row15.i.i.i177.i231, align 16, !alias.scope !9710, !noalias !9715
  %_126.i.i.i.i43616333 = load <4 x float>, ptr %1188, align 16, !alias.scope !9710, !noalias !9715
  %_129.i.i.i.i43916334 = load <4 x float>, ptr %1189, align 16, !alias.scope !9710, !noalias !9715
  %_132.i.i.i.i44216335 = load <4 x float>, ptr %1190, align 16, !alias.scope !9710, !noalias !9715
  %_137.i.i.i.i44716336 = load <4 x float>, ptr %row17.i.i.i191.i245, align 16, !alias.scope !9710, !noalias !9715
  %_140.i.i.i.i45016337 = load <4 x float>, ptr %1191, align 16, !alias.scope !9710, !noalias !9715
  %_143.i.i.i.i45316338 = load <4 x float>, ptr %1192, align 16, !alias.scope !9710, !noalias !9715
  %_146.i.i.i.i45616339 = load <4 x float>, ptr %1193, align 16, !alias.scope !9710, !noalias !9715
  %_151.i.i.i.i46116340 = load <4 x float>, ptr %row19.i.i.i205.i259, align 16, !alias.scope !9710, !noalias !9715
  %_154.i.i.i.i46416341 = load <4 x float>, ptr %1194, align 16, !alias.scope !9710, !noalias !9715
  %_157.i.i.i.i46716342 = load <4 x float>, ptr %1195, align 16, !alias.scope !9710, !noalias !9715
  %_160.i.i.i.i47016343 = load <4 x float>, ptr %1196, align 16, !alias.scope !9710, !noalias !9715
  %_165.i.i.i.i47516344 = load <4 x float>, ptr %row21.i.i.i219.i273, align 16, !alias.scope !9710, !noalias !9715
  %_168.i.i.i.i47816345 = load <4 x float>, ptr %1197, align 16, !alias.scope !9710, !noalias !9715
  %_171.i.i.i.i48116346 = load <4 x float>, ptr %1198, align 16, !alias.scope !9710, !noalias !9715
  %_174.i.i.i.i48416347 = load <4 x float>, ptr %1199, align 16, !alias.scope !9710, !noalias !9715
  br label %bb5.i.i303, !dbg !9707

bb5.i.i303:                                       ; preds = %bb5.i.i303.lr.ph, %bb5.i.i303
  %iter.i.i38.sroa.16.018744 = phi i32 [ 0, %bb5.i.i303.lr.ph ], [ %1480, %bb5.i.i303 ]
  %history.i.i42.sroa.35.018743 = phi <4 x i32> [ %history.i.i42.sroa.35.0.copyload, %bb5.i.i303.lr.ph ], [ %history.i.i42.sroa.32.018742, %bb5.i.i303 ]
  %history.i.i42.sroa.32.018742 = phi <4 x i32> [ %history.i.i42.sroa.32.0.copyload, %bb5.i.i303.lr.ph ], [ %history.i.i42.sroa.29.018741, %bb5.i.i303 ]
  %history.i.i42.sroa.29.018741 = phi <4 x i32> [ %history.i.i42.sroa.29.0.copyload, %bb5.i.i303.lr.ph ], [ %history.i.i42.sroa.26.018740, %bb5.i.i303 ]
  %history.i.i42.sroa.26.018740 = phi <4 x i32> [ %history.i.i42.sroa.26.0.copyload, %bb5.i.i303.lr.ph ], [ %history.i.i42.sroa.22.018739, %bb5.i.i303 ]
  %history.i.i42.sroa.22.018739 = phi <4 x i32> [ %history.i.i42.sroa.22.0.copyload, %bb5.i.i303.lr.ph ], [ %history.i.i42.sroa.19.018738, %bb5.i.i303 ]
  %history.i.i42.sroa.19.018738 = phi <4 x i32> [ %history.i.i42.sroa.19.0.copyload, %bb5.i.i303.lr.ph ], [ %history.i.i42.sroa.16.018737, %bb5.i.i303 ]
  %history.i.i42.sroa.16.018737 = phi <4 x i32> [ %history.i.i42.sroa.16.0.copyload, %bb5.i.i303.lr.ph ], [ %history.i.i42.sroa.13.018736, %bb5.i.i303 ]
  %history.i.i42.sroa.13.018736 = phi <4 x i32> [ %history.i.i42.sroa.13.0.copyload, %bb5.i.i303.lr.ph ], [ %history.i.i42.sroa.10.018735, %bb5.i.i303 ]
  %history.i.i42.sroa.10.018735 = phi <4 x i32> [ %history.i.i42.sroa.10.0.copyload, %bb5.i.i303.lr.ph ], [ %history.i.i42.sroa.7.018734, %bb5.i.i303 ]
  %history.i.i42.sroa.7.018734 = phi <4 x i32> [ %history.i.i42.sroa.7.0.copyload, %bb5.i.i303.lr.ph ], [ %history.i.i42.sroa.0.018733, %bb5.i.i303 ]
  %history.i.i42.sroa.0.018733 = phi <4 x i32> [ %history.i.i42.sroa.0.0.copyload, %bb5.i.i303.lr.ph ], [ %lanes.i5880.sroa.0.0.copyload, %bb5.i.i303 ]
  %start1.i.i7278 = shl i32 %iter.i.i38.sroa.16.018744, 2, !dbg !9724
  %data.i.i7279 = getelementptr inbounds nuw float, ptr %_194.i300, i32 %start1.i.i7278, !dbg !9726
  %lanes.i5880.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i7279, align 4, !dbg !9728, !alias.scope !9733, !noalias !9737
  %1359 = bitcast <4 x i32> %history.i.i42.sroa.19.018738 to <4 x float>, !dbg !9741
  %1360 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1359), !dbg !9746
  %1361 = bitcast <4 x i32> %lanes.i5880.sroa.0.0.copyload to <4 x float>, !dbg !9747
  %1362 = fmul <4 x float> %_11.i.i.i.i32116300, %1361, !dbg !9752
  %1363 = fadd <4 x float> %1362, zeroinitializer, !dbg !9753
  %1364 = fmul <4 x float> %_14.i.i.i.i32416301, %1361, !dbg !9757
  %1365 = fadd <4 x float> %1364, zeroinitializer, !dbg !9761
  %1366 = fmul <4 x float> %_17.i.i.i.i32716302, %1361, !dbg !9765
  %1367 = fadd <4 x float> %1366, zeroinitializer, !dbg !9769
  %1368 = fmul <4 x float> %_20.i.i.i.i33016303, %1361, !dbg !9773
  %1369 = fadd <4 x float> %1368, zeroinitializer, !dbg !9777
  %1370 = bitcast <4 x i32> %history.i.i42.sroa.0.018733 to <4 x float>, !dbg !9781
  %1371 = fmul <4 x float> %_25.i.i.i.i33516304, %1370, !dbg !9785
  %1372 = fadd <4 x float> %1363, %1371, !dbg !9786
  %1373 = fmul <4 x float> %_28.i.i.i.i33816305, %1370, !dbg !9790
  %1374 = fadd <4 x float> %1365, %1373, !dbg !9794
  %1375 = fmul <4 x float> %_31.i.i.i.i34116306, %1370, !dbg !9798
  %1376 = fadd <4 x float> %1367, %1375, !dbg !9802
  %1377 = fmul <4 x float> %_34.i.i.i.i34416307, %1370, !dbg !9806
  %1378 = fadd <4 x float> %1369, %1377, !dbg !9810
  %1379 = bitcast <4 x i32> %history.i.i42.sroa.7.018734 to <4 x float>, !dbg !9814
  %1380 = fmul <4 x float> %_39.i.i.i.i34916308, %1379, !dbg !9818
  %1381 = fadd <4 x float> %1372, %1380, !dbg !9819
  %1382 = fmul <4 x float> %_42.i.i.i.i35216309, %1379, !dbg !9823
  %1383 = fadd <4 x float> %1374, %1382, !dbg !9827
  %1384 = fmul <4 x float> %_45.i.i.i.i35516310, %1379, !dbg !9831
  %1385 = fadd <4 x float> %1376, %1384, !dbg !9835
  %1386 = fmul <4 x float> %_48.i.i.i.i35816311, %1379, !dbg !9839
  %1387 = fadd <4 x float> %1378, %1386, !dbg !9843
  %1388 = bitcast <4 x i32> %history.i.i42.sroa.10.018735 to <4 x float>, !dbg !9847
  %1389 = fmul <4 x float> %_53.i.i.i.i36316312, %1388, !dbg !9851
  %1390 = fadd <4 x float> %1381, %1389, !dbg !9852
  %1391 = fmul <4 x float> %_56.i.i.i.i36616313, %1388, !dbg !9856
  %1392 = fadd <4 x float> %1383, %1391, !dbg !9860
  %1393 = fmul <4 x float> %_59.i.i.i.i36916314, %1388, !dbg !9864
  %1394 = fadd <4 x float> %1385, %1393, !dbg !9868
  %1395 = fmul <4 x float> %_62.i.i.i.i37216315, %1388, !dbg !9872
  %1396 = fadd <4 x float> %1387, %1395, !dbg !9876
  %1397 = bitcast <4 x i32> %history.i.i42.sroa.13.018736 to <4 x float>, !dbg !9880
  %1398 = fmul <4 x float> %_67.i.i.i.i37716316, %1397, !dbg !9884
  %1399 = fadd <4 x float> %1390, %1398, !dbg !9885
  %1400 = fmul <4 x float> %_70.i.i.i.i38016317, %1397, !dbg !9889
  %1401 = fadd <4 x float> %1392, %1400, !dbg !9893
  %1402 = fmul <4 x float> %_73.i.i.i.i38316318, %1397, !dbg !9897
  %1403 = fadd <4 x float> %1394, %1402, !dbg !9901
  %1404 = fmul <4 x float> %_76.i.i.i.i38616319, %1397, !dbg !9905
  %1405 = fadd <4 x float> %1396, %1404, !dbg !9909
  %1406 = bitcast <4 x i32> %history.i.i42.sroa.16.018737 to <4 x float>, !dbg !9913
  %1407 = fmul <4 x float> %_81.i.i.i.i39116320, %1406, !dbg !9917
  %1408 = fadd <4 x float> %1399, %1407, !dbg !9918
  %1409 = fmul <4 x float> %_84.i.i.i.i39416321, %1406, !dbg !9922
  %1410 = fadd <4 x float> %1401, %1409, !dbg !9926
  %1411 = fmul <4 x float> %_87.i.i.i.i39716322, %1406, !dbg !9930
  %1412 = fadd <4 x float> %1403, %1411, !dbg !9934
  %1413 = fmul <4 x float> %_90.i.i.i.i40016323, %1406, !dbg !9938
  %1414 = fadd <4 x float> %1405, %1413, !dbg !9942
  %1415 = fmul <4 x float> %_95.i.i.i.i40516324, %1359, !dbg !9946
  %1416 = fadd <4 x float> %1408, %1415, !dbg !9950
  %1417 = fmul <4 x float> %_98.i.i.i.i40816325, %1359, !dbg !9954
  %1418 = fadd <4 x float> %1410, %1417, !dbg !9958
  %1419 = fmul <4 x float> %_101.i.i.i.i41116326, %1359, !dbg !9962
  %1420 = fadd <4 x float> %1412, %1419, !dbg !9966
  %1421 = fmul <4 x float> %_104.i.i.i.i41416327, %1359, !dbg !9970
  %1422 = fadd <4 x float> %1414, %1421, !dbg !9974
  %1423 = bitcast <4 x i32> %history.i.i42.sroa.22.018739 to <4 x float>, !dbg !9978
  %1424 = fmul <4 x float> %_109.i.i.i.i41916328, %1423, !dbg !9982
  %1425 = fadd <4 x float> %1416, %1424, !dbg !9983
  %1426 = fmul <4 x float> %_112.i.i.i.i42216329, %1423, !dbg !9987
  %1427 = fadd <4 x float> %1418, %1426, !dbg !9991
  %1428 = fmul <4 x float> %_115.i.i.i.i42516330, %1423, !dbg !9995
  %1429 = fadd <4 x float> %1420, %1428, !dbg !9999
  %1430 = fmul <4 x float> %_118.i.i.i.i42816331, %1423, !dbg !10003
  %1431 = fadd <4 x float> %1422, %1430, !dbg !10007
  %1432 = bitcast <4 x i32> %history.i.i42.sroa.26.018740 to <4 x float>, !dbg !10011
  %1433 = fmul <4 x float> %_123.i.i.i.i43316332, %1432, !dbg !10015
  %1434 = fadd <4 x float> %1425, %1433, !dbg !10016
  %1435 = fmul <4 x float> %_126.i.i.i.i43616333, %1432, !dbg !10020
  %1436 = fadd <4 x float> %1427, %1435, !dbg !10024
  %1437 = fmul <4 x float> %_129.i.i.i.i43916334, %1432, !dbg !10028
  %1438 = fadd <4 x float> %1429, %1437, !dbg !10032
  %1439 = fmul <4 x float> %_132.i.i.i.i44216335, %1432, !dbg !10036
  %1440 = fadd <4 x float> %1431, %1439, !dbg !10040
  %1441 = bitcast <4 x i32> %history.i.i42.sroa.29.018741 to <4 x float>, !dbg !10044
  %1442 = fmul <4 x float> %_137.i.i.i.i44716336, %1441, !dbg !10048
  %1443 = fadd <4 x float> %1434, %1442, !dbg !10049
  %1444 = fmul <4 x float> %_140.i.i.i.i45016337, %1441, !dbg !10053
  %1445 = fadd <4 x float> %1436, %1444, !dbg !10057
  %1446 = fmul <4 x float> %_143.i.i.i.i45316338, %1441, !dbg !10061
  %1447 = fadd <4 x float> %1438, %1446, !dbg !10065
  %1448 = fmul <4 x float> %_146.i.i.i.i45616339, %1441, !dbg !10069
  %1449 = fadd <4 x float> %1440, %1448, !dbg !10073
  %1450 = bitcast <4 x i32> %history.i.i42.sroa.32.018742 to <4 x float>, !dbg !10077
  %1451 = fmul <4 x float> %_151.i.i.i.i46116340, %1450, !dbg !10081
  %1452 = fadd <4 x float> %1443, %1451, !dbg !10082
  %1453 = fmul <4 x float> %_154.i.i.i.i46416341, %1450, !dbg !10086
  %1454 = fadd <4 x float> %1445, %1453, !dbg !10090
  %1455 = fmul <4 x float> %_157.i.i.i.i46716342, %1450, !dbg !10094
  %1456 = fadd <4 x float> %1447, %1455, !dbg !10098
  %1457 = fmul <4 x float> %_160.i.i.i.i47016343, %1450, !dbg !10102
  %1458 = fadd <4 x float> %1449, %1457, !dbg !10106
  %1459 = bitcast <4 x i32> %history.i.i42.sroa.35.018743 to <4 x float>, !dbg !10110
  %1460 = fmul <4 x float> %_165.i.i.i.i47516344, %1459, !dbg !10114
  %1461 = fadd <4 x float> %1452, %1460, !dbg !10115
  %1462 = fmul <4 x float> %_168.i.i.i.i47816345, %1459, !dbg !10119
  %1463 = fadd <4 x float> %1454, %1462, !dbg !10123
  %1464 = fmul <4 x float> %_171.i.i.i.i48116346, %1459, !dbg !10127
  %1465 = fadd <4 x float> %1456, %1464, !dbg !10131
  %1466 = fmul <4 x float> %_174.i.i.i.i48416347, %1459, !dbg !10135
  %1467 = fadd <4 x float> %1458, %1466, !dbg !10139
  %1468 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1461), !dbg !10143
  %1469 = fcmp olt <4 x float> %1468, %1360, !dbg !10147
  %1470 = select <4 x i1> %1469, <4 x float> %1360, <4 x float> %1468, !dbg !10151
  %1471 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1463), !dbg !10143
  %1472 = fcmp olt <4 x float> %1471, %1470, !dbg !10147
  %1473 = select <4 x i1> %1472, <4 x float> %1470, <4 x float> %1471, !dbg !10151
  %1474 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1465), !dbg !10143
  %1475 = fcmp olt <4 x float> %1474, %1473, !dbg !10147
  %1476 = select <4 x i1> %1475, <4 x float> %1473, <4 x float> %1474, !dbg !10151
  %1477 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1467), !dbg !10143
  %1478 = fcmp olt <4 x float> %1477, %1476, !dbg !10147
  %1479 = select <4 x i1> %1478, <4 x float> %1476, <4 x float> %1477, !dbg !10151
  %1480 = add nuw nsw i32 %iter.i.i38.sroa.16.018744, 1, !dbg !10152
  %data.i4.i7283 = getelementptr inbounds nuw float, ptr %peaks_right.i64, i32 %start1.i.i7278, !dbg !10153
  store <4 x float> %1479, ptr %data.i4.i7283, align 4, !dbg !10156, !alias.scope !10161, !noalias !10165
  %exitcond20956.not = icmp eq i32 %1480, %umax20955, !dbg !9707
  br i1 %exitcond20956.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i497, label %bb5.i.i303, !dbg !9707

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i497: ; preds = %bb5.i.i303, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7269
  %history.i.i42.sroa.0.0.lcssa = phi <4 x i32> [ %history.i.i42.sroa.0.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7269 ], [ %lanes.i5880.sroa.0.0.copyload, %bb5.i.i303 ], !dbg !10169
  %history.i.i42.sroa.7.0.lcssa = phi <4 x i32> [ %history.i.i42.sroa.7.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7269 ], [ %history.i.i42.sroa.0.018733, %bb5.i.i303 ], !dbg !10169
  %history.i.i42.sroa.10.0.lcssa = phi <4 x i32> [ %history.i.i42.sroa.10.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7269 ], [ %history.i.i42.sroa.7.018734, %bb5.i.i303 ], !dbg !10169
  %history.i.i42.sroa.13.0.lcssa = phi <4 x i32> [ %history.i.i42.sroa.13.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7269 ], [ %history.i.i42.sroa.10.018735, %bb5.i.i303 ], !dbg !10169
  %history.i.i42.sroa.16.0.lcssa = phi <4 x i32> [ %history.i.i42.sroa.16.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7269 ], [ %history.i.i42.sroa.13.018736, %bb5.i.i303 ], !dbg !10169
  %history.i.i42.sroa.19.0.lcssa = phi <4 x i32> [ %history.i.i42.sroa.19.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7269 ], [ %history.i.i42.sroa.16.018737, %bb5.i.i303 ], !dbg !10169
  %history.i.i42.sroa.22.0.lcssa = phi <4 x i32> [ %history.i.i42.sroa.22.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7269 ], [ %history.i.i42.sroa.19.018738, %bb5.i.i303 ], !dbg !10169
  %history.i.i42.sroa.26.0.lcssa = phi <4 x i32> [ %history.i.i42.sroa.26.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7269 ], [ %history.i.i42.sroa.22.018739, %bb5.i.i303 ], !dbg !10169
  %history.i.i42.sroa.29.0.lcssa = phi <4 x i32> [ %history.i.i42.sroa.29.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7269 ], [ %history.i.i42.sroa.26.018740, %bb5.i.i303 ], !dbg !10169
  %history.i.i42.sroa.32.0.lcssa = phi <4 x i32> [ %history.i.i42.sroa.32.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7269 ], [ %history.i.i42.sroa.29.018741, %bb5.i.i303 ], !dbg !10169
  %history.i.i42.sroa.35.0.lcssa = phi <4 x i32> [ %history.i.i42.sroa.35.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7269 ], [ %history.i.i42.sroa.32.018742, %bb5.i.i303 ], !dbg !10169
  %history.i.i42.sroa.38.0.lcssa = phi <4 x i32> [ %history.i.i42.sroa.38.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7269 ], [ %history.i.i42.sroa.35.018743, %bb5.i.i303 ], !dbg !10169
  store <4 x i32> %history.i.i42.sroa.0.0.lcssa, ptr %hot_right.i67, align 16, !dbg !10170
  store <4 x i32> %history.i.i42.sroa.7.0.lcssa, ptr %history.i.i42.sroa.7.0.hot_right.i67.sroa_idx, align 16, !dbg !10170
  store <4 x i32> %history.i.i42.sroa.10.0.lcssa, ptr %history.i.i42.sroa.10.0.hot_right.i67.sroa_idx, align 16, !dbg !10170
  store <4 x i32> %history.i.i42.sroa.13.0.lcssa, ptr %history.i.i42.sroa.13.0.hot_right.i67.sroa_idx, align 16, !dbg !10170
  store <4 x i32> %history.i.i42.sroa.16.0.lcssa, ptr %history.i.i42.sroa.16.0.hot_right.i67.sroa_idx, align 16, !dbg !10170
  store <4 x i32> %history.i.i42.sroa.19.0.lcssa, ptr %history.i.i42.sroa.19.0.hot_right.i67.sroa_idx, align 16, !dbg !10170
  store <4 x i32> %history.i.i42.sroa.22.0.lcssa, ptr %history.i.i42.sroa.22.0.hot_right.i67.sroa_idx, align 16, !dbg !10170
  store <4 x i32> %history.i.i42.sroa.26.0.lcssa, ptr %history.i.i42.sroa.26.0.hot_right.i67.sroa_idx, align 16, !dbg !10170
  store <4 x i32> %history.i.i42.sroa.29.0.lcssa, ptr %history.i.i42.sroa.29.0.hot_right.i67.sroa_idx, align 16, !dbg !10170
  store <4 x i32> %history.i.i42.sroa.32.0.lcssa, ptr %history.i.i42.sroa.32.0.hot_right.i67.sroa_idx, align 16, !dbg !10170
  store <4 x i32> %history.i.i42.sroa.35.0.lcssa, ptr %history.i.i42.sroa.35.0.hot_right.i67.sroa_idx, align 16, !dbg !10170
  store <4 x i32> %history.i.i42.sroa.38.0.lcssa, ptr %history.i.i42.sroa.38.0.hot_right.i67.sroa_idx, align 16, !dbg !10170
  br i1 %_2.i722218705.not, label %bb15.i85.loopexit, label %bb20.i503.preheader, !dbg !10171

bb20.i503.preheader:                              ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i497
  %_72.i60.sroa.3.0.copyload = load i32, ptr %_72.i60.sroa.3.0..sroa_idx, align 4, !noalias !9163
  %_72.i60.sroa.4.0.copyload = load i32, ptr %_72.i60.sroa.4.0..sroa_idx, align 4, !noalias !9163
  %_73.i59.sroa.3.0.copyload = load i32, ptr %_73.i59.sroa.3.0..sroa_idx, align 4, !noalias !9163
  %_73.i59.sroa.4.0.copyload = load i32, ptr %_73.i59.sroa.4.0..sroa_idx, align 4, !noalias !9163
  %_54.0.i254.i579 = load ptr, ptr %1215, align 16, !nonnull !10, !align !10173
  %_54.1.i255.i580 = load i32, ptr %1216, align 4
  %_18.i266.i591 = load i32, ptr %1200, align 4
  %_29.i179318757.not = icmp eq i32 %_18.i266.i591, 0
  %_56.0.i276.i601 = load ptr, ptr %1217, align 8, !nonnull !10, !align !10173
  %_56.1.i277.i602 = load i32, ptr %1218, align 4
  %_58.1.i307.i632 = load i32, ptr %1222, align 4
  %_58.0.i306.i631 = load ptr, ptr %1223, align 16, !nonnull !10, !align !10173
  %_54.0.i.i661 = load ptr, ptr %1225, align 16, !nonnull !10, !align !10173
  %_54.1.i.i662 = load i32, ptr %1226, align 4
  %_18.i.i673 = load i32, ptr %1201, align 4
  %_29.i177118760.not = icmp eq i32 %_18.i.i673, 0
  %_56.0.i.i683 = load ptr, ptr %1227, align 8, !nonnull !10, !align !10173
  %_56.1.i.i684 = load i32, ptr %1228, align 4
  %_58.1.i.i714 = load i32, ptr %1232, align 4
  %_58.0.i.i713 = load ptr, ptr %1233, align 16, !nonnull !10, !align !10173
  %_22.i269.i594.promoted23106 = load i32, ptr %_22.i269.i594, align 4
  %_22.i.i676.promoted23124 = load i32, ptr %_22.i.i676, align 4
  br label %bb20.i503, !dbg !10174

bb20.i503:                                        ; preds = %bb20.i503.preheader, %bb74.i734
  %storemerge.i.lcssa2308923126 = phi i32 [ %_22.i.i676.promoted23124, %bb20.i503.preheader ], [ %storemerge.i.lcssa2308923125, %bb74.i734 ]
  %storemerge.i1788.lcssa2305923108 = phi i32 [ %_22.i269.i594.promoted23106, %bb20.i503.preheader ], [ %storemerge.i1788.lcssa2305923107, %bb74.i734 ]
  %frame.sroa.0.0.i50118769 = phi i32 [ 0, %bb20.i503.preheader ], [ %_87.i519, %bb74.i734 ]
  %main_cursor.sroa.0.1.i50018768 = phi i32 [ %main_cursor.sroa.0.0.i8718774, %bb20.i503.preheader ], [ %main_cursor.sroa.0.2.i740, %bb74.i734 ]
  %ring_cursor.sroa.0.1.i49918767 = phi i32 [ %ring_cursor.sroa.0.0.i8618773, %bb20.i503.preheader ], [ %ring_cursor.sroa.0.2.i737, %bb74.i734 ]
  %_69.i504 = sub nuw nsw i32 %spec.store.select.i92, %frame.sroa.0.0.i50118769, !dbg !10186
  %ring.i2441 = load i32, ptr %84, align 4, !dbg !10187, !alias.scope !10190, !noalias !10193, !noundef !10
  %main.i2442 = load i32, ptr %85, align 4, !dbg !10197, !alias.scope !10190, !noalias !10193, !noundef !10
  %_10.i = add i32 %ring_cursor.sroa.0.1.i49918767, 1, !dbg !10199
  %_38.not.i = icmp ult i32 %_10.i, %ring.i2441, !dbg !10201
  %1481 = select i1 %_38.not.i, i32 0, i32 %ring.i2441, !dbg !10201
  %start1.sroa.0.0.i2443 = sub nuw i32 %_10.i, %1481, !dbg !10201
  %_12.i2445 = add i32 %_72.i60.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i49918767, !dbg !10204
  %_39.not.i = icmp ult i32 %_12.i2445, %ring.i2441, !dbg !10206
  %1482 = select i1 %_39.not.i, i32 0, i32 %ring.i2441, !dbg !10206
  %left_end.sroa.0.0.i = sub nuw i32 %_12.i2445, %1482, !dbg !10206
  %_15.i2447 = add i32 %_73.i59.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i49918767, !dbg !10208
  %_40.not.i = icmp ult i32 %_15.i2447, %ring.i2441, !dbg !10210
  %1483 = select i1 %_40.not.i, i32 0, i32 %ring.i2441, !dbg !10210
  %right_end.sroa.0.0.i = sub nuw i32 %_15.i2447, %1483, !dbg !10210
  %_18.i2448 = add i32 %_72.i60.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i49918767, !dbg !10212
  %_41.not.i = icmp ult i32 %_18.i2448, %ring.i2441, !dbg !10214
  %1484 = select i1 %_41.not.i, i32 0, i32 %ring.i2441, !dbg !10214
  %left_expiring.sroa.0.0.i = sub nuw i32 %_18.i2448, %1484, !dbg !10214
  %_21.i2450 = add i32 %_73.i59.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i49918767, !dbg !10216
  %_42.not.i = icmp ult i32 %_21.i2450, %ring.i2441, !dbg !10218
  %1485 = select i1 %_42.not.i, i32 0, i32 %ring.i2441, !dbg !10218
  %right_expiring.sroa.0.0.i = sub nuw i32 %_21.i2450, %1485, !dbg !10218
  %1486 = sub i32 %ring.i2441, %ring_cursor.sroa.0.1.i49918767, !dbg !10220
  %spec.store.select.i2451 = tail call i32 @llvm.umin.i32(i32 %1486, i32 %_69.i504), !dbg !10222
  %1487 = sub i32 %main.i2442, %main_cursor.sroa.0.1.i50018768, !dbg !10225
  %_24.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %1487, i32 %spec.store.select.i2451), !dbg !10226
  %1488 = sub i32 %ring.i2441, %start1.sroa.0.0.i2443, !dbg !10228
  %_25.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %1488, i32 %_24.sroa.0.0.i), !dbg !10229
  %1489 = sub i32 %ring.i2441, %left_end.sroa.0.0.i, !dbg !10231
  %_27.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %1489, i32 %_25.sroa.0.0.i), !dbg !10232
  %1490 = sub i32 %ring.i2441, %right_end.sroa.0.0.i, !dbg !10234
  %_29.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %1490, i32 %_27.sroa.0.0.i), !dbg !10235
  %1491 = sub i32 %ring.i2441, %left_expiring.sroa.0.0.i, !dbg !10237
  %_31.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %1491, i32 %_29.sroa.0.0.i), !dbg !10238
  %1492 = sub i32 %ring.i2441, %right_expiring.sroa.0.0.i, !dbg !10240
  %run.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %1492, i32 %_31.sroa.0.0.i), !dbg !10241
  %_76.i506 = add i32 %frame.sroa.0.0.i50118769, %iter1.sroa.0.0.i8818775, !dbg !10243
  %base.i507 = shl i32 %_76.i506, 2, !dbg !10243
  %base.i50716256 = add i32 %run.sroa.0.0.i, %_76.i506, !dbg !10244
  %_80.i509 = shl i32 %base.i50716256, 2, !dbg !10244
  %_203.i510 = icmp ult i32 %_80.i509, %base.i507, !dbg !10174
  %_199.not.i511 = icmp ugt i32 %_80.i509, %left_io.1
  %or.cond24.i512 = or i1 %_203.i510, %_199.not.i511, !dbg !10174
  br i1 %or.cond24.i512, label %bb58.i742, label %bb57.i513, !dbg !10174, !prof !4596

bb58.i742:                                        ; preds = %bb20.i503
  store i32 %storemerge.i1788.lcssa2305923108, ptr %_22.i269.i594, align 4
  store i32 %storemerge.i.lcssa2308923126, ptr %_22.i.i676, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i507, i32 noundef %_80.i509, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb67056a871aedf25dd2ba0a06d720f) #32, !dbg !10245, !noalias !9134
  unreachable, !dbg !10245

bb57.i513:                                        ; preds = %bb20.i503
  %_206.i514 = getelementptr inbounds nuw float, ptr %left_io.0, i32 %base.i507, !dbg !10246
  %_207.not.i515 = icmp ugt i32 %_80.i509, %right_io.1, !dbg !10250
  br i1 %_207.not.i515, label %bb61.i741, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit, !dbg !10250, !prof !787

bb61.i741:                                        ; preds = %bb57.i513
  store i32 %storemerge.i1788.lcssa2305923108, ptr %_22.i269.i594, align 4
  store i32 %storemerge.i.lcssa2308923126, ptr %_22.i.i676, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i507, i32 noundef %_80.i509, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1d0fce16c93a3aa07bd2f89733f587ef) #32, !dbg !10255, !noalias !9134
  unreachable, !dbg !10255

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit: ; preds = %bb57.i513
  %_212.i517 = getelementptr inbounds nuw float, ptr %right_io.0, i32 %base.i507, !dbg !10256
  %_87.i519 = add nuw nsw i32 %run.sroa.0.0.i, %frame.sroa.0.0.i50118769, !dbg !10260
  %_84.i518 = shl nuw nsw i32 %frame.sroa.0.0.i50118769, 2, !dbg !10262
  %_221.i527 = getelementptr inbounds nuw float, ptr %peaks_left.i65, i32 %_84.i518, !dbg !10263
  %_230.i528 = getelementptr inbounds nuw float, ptr %peaks_right.i64, i32 %_84.i518, !dbg !10272
  %_2.i734018763.not = icmp eq i32 %run.sroa.0.0.i, 0, !dbg !10282
  br i1 %_2.i734018763.not, label %bb74.i734, label %bb75.i532.preheader, !dbg !10282

bb75.i532.preheader:                              ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit
  %umin20971 = call i32 @llvm.umin.i32(i32 %1489, i32 %1490)
  %umin20972 = call i32 @llvm.umin.i32(i32 %umin20971, i32 %1491)
  %umin20973 = call i32 @llvm.umin.i32(i32 %umin20972, i32 %1492)
  %umin20974 = call i32 @llvm.umin.i32(i32 %umin20973, i32 %1488)
  %umin20975 = call i32 @llvm.umin.i32(i32 %umin20974, i32 %1486)
  %umin20976 = call i32 @llvm.umin.i32(i32 %umin20975, i32 %1487)
  %1493 = sub nsw i32 %umin20977, %frame.sroa.0.0.i50118769
  %umin20978 = call i32 @llvm.umin.i32(i32 %umin20976, i32 %1493)
  %1494 = and i32 %umin20978, 1073741823
  %_11.i257516263.pre = load <4 x float>, ptr %_115.i550, align 16, !dbg !10292
  %_12.i2576.pre = load <4 x i32>, ptr %1206, align 16, !dbg !10296
  %_5.i255716265.pre = load <4 x float>, ptr %1208, align 16, !dbg !10297
  %_11.i256216267.pre = load <4 x float>, ptr %_119.i553, align 16, !dbg !10301
  %_12.i2563.pre = load <4 x i32>, ptr %1209, align 16, !dbg !10302
  %_13.i256516268.pre = load <16 x i8>, ptr %1210, align 16, !dbg !10303
  %_5.i254416269.pre = load <4 x float>, ptr %1211, align 16, !dbg !10304
  %_13.i259116260 = load <16 x i8>, ptr %1204, align 16
  %_13.i257816264 = load <16 x i8>, ptr %1207, align 16
  %_13.i255216272 = load <16 x i8>, ptr %1213, align 16
  %_37.i293.i61816280 = load <4 x float>, ptr %1220, align 16
  %_37.i.i70016289 = load <4 x float>, ptr %1230, align 16
  %.promoted22928 = load <4 x float>, ptr %1202, align 16
  %.promoted22944 = load <4 x float>, ptr %1205, align 16
  %.promoted23060 = load <4 x float>, ptr %1219, align 16
  %.promoted23090 = load <4 x float>, ptr %1229, align 16
  br label %bb75.i532

bb75.i532:                                        ; preds = %bb75.i532.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2026
  %1495 = phi <4 x float> [ %.promoted23090, %bb75.i532.preheader ], [ %1628, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2026 ]
  %storemerge.i23076 = phi i32 [ %storemerge.i.lcssa2308923126, %bb75.i532.preheader ], [ %storemerge.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2026 ]
  %1496 = phi <4 x float> [ %.promoted23060, %bb75.i532.preheader ], [ %1577, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2026 ]
  %storemerge.i178823046 = phi i32 [ %storemerge.i1788.lcssa2305923108, %bb75.i532.preheader ], [ %storemerge.i1788, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2026 ]
  %1497 = phi <4 x float> [ %.promoted22944, %bb75.i532.preheader ], [ %1510, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2026 ], !dbg !10306
  %1498 = phi <4 x float> [ %.promoted22928, %bb75.i532.preheader ], [ %1501, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2026 ], !dbg !10311
  %_5.i254416269 = phi <4 x float> [ %_5.i254416269.pre, %bb75.i532.preheader ], [ %1528, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2026 ], !dbg !10304
  %_12.i2563 = phi <4 x i32> [ %_12.i2563.pre, %bb75.i532.preheader ], [ %1544, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2026 ], !dbg !10302
  %_11.i256216267 = phi <4 x float> [ %_11.i256216267.pre, %bb75.i532.preheader ], [ %1543, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2026 ], !dbg !10301
  %_5.i255716265 = phi <4 x float> [ %_5.i255716265.pre, %bb75.i532.preheader ], [ %1519, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2026 ], !dbg !10297
  %_12.i2576 = phi <4 x i32> [ %_12.i2576.pre, %bb75.i532.preheader ], [ %1542, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2026 ], !dbg !10296
  %_11.i257516263 = phi <4 x float> [ %_11.i257516263.pre, %bb75.i532.preheader ], [ %1541, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2026 ], !dbg !10292
  %iter.i50.sroa.41.018765 = phi i32 [ 0, %bb75.i532.preheader ], [ %_235.0.i548, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2026 ]
  %1499 = fadd <4 x float> %1498, splat (float -1.000000e+00), !dbg !10306
  %1500 = fcmp ogt <4 x float> %1499, zeroinitializer, !dbg !10312
  %1501 = select <4 x i1> %1500, <4 x float> %1499, <4 x float> zeroinitializer, !dbg !10316
  %1502 = sext <4 x i1> %1500 to <4 x i32>, !dbg !10317
  %_11.i258816259 = load <4 x float>, ptr %_114.i549, align 16, !dbg !10322
  %_12.i2589 = load <4 x i32>, ptr %1203, align 16, !dbg !10323
  %1503 = bitcast <4 x i32> %_12.i2589 to <4 x float>, !dbg !10324
  %1504 = fadd <4 x float> %_11.i258816259, %1503, !dbg !10328
  %1505 = bitcast <4 x float> %1504 to <16 x i8>, !dbg !10329
  %1506 = bitcast <4 x i32> %1502 to <16 x i8>, !dbg !10333
  %_4.i7352 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1505, <16 x i8> %_13.i259116260, <16 x i8> %1506), !dbg !10334
  store <16 x i8> %_4.i7352, ptr %_114.i549, align 16, !dbg !10335
  %1507 = bitcast <4 x i32> %_12.i2589 to <16 x i8>, !dbg !10336
  %_4.i7353 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1507, <16 x i8> zeroinitializer, <16 x i8> %1506), !dbg !10340
  store <16 x i8> %_4.i7353, ptr %1203, align 16, !dbg !10341
  %1508 = fadd <4 x float> %1497, splat (float -1.000000e+00), !dbg !10342
  %1509 = fcmp ogt <4 x float> %1508, zeroinitializer, !dbg !10346
  %1510 = select <4 x i1> %1509, <4 x float> %1508, <4 x float> zeroinitializer, !dbg !10350
  %1511 = sext <4 x i1> %1509 to <4 x i32>, !dbg !10351
  %1512 = bitcast <4 x i32> %_12.i2576 to <4 x float>, !dbg !10356
  %1513 = fadd <4 x float> %_11.i257516263, %1512, !dbg !10360
  %1514 = bitcast <4 x float> %1513 to <16 x i8>, !dbg !10361
  %1515 = bitcast <4 x i32> %1511 to <16 x i8>, !dbg !10365
  %_4.i7354 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1514, <16 x i8> %_13.i257816264, <16 x i8> %1515), !dbg !10366
  %1516 = bitcast <4 x i32> %_12.i2576 to <16 x i8>, !dbg !10367
  %_4.i7355 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1516, <16 x i8> zeroinitializer, <16 x i8> %1515), !dbg !10371
  %1517 = fadd <4 x float> %_5.i255716265, splat (float -1.000000e+00), !dbg !10372
  %1518 = fcmp ogt <4 x float> %1517, zeroinitializer, !dbg !10376
  %1519 = select <4 x i1> %1518, <4 x float> %1517, <4 x float> zeroinitializer, !dbg !10380
  %1520 = sext <4 x i1> %1518 to <4 x i32>, !dbg !10381
  %1521 = bitcast <4 x i32> %_12.i2563 to <4 x float>, !dbg !10386
  %1522 = fadd <4 x float> %_11.i256216267, %1521, !dbg !10390
  %1523 = bitcast <4 x float> %1522 to <16 x i8>, !dbg !10391
  %1524 = bitcast <4 x i32> %1520 to <16 x i8>, !dbg !10395
  %_4.i7356 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1523, <16 x i8> %_13.i256516268.pre, <16 x i8> %1524), !dbg !10396
  %1525 = bitcast <4 x i32> %_12.i2563 to <16 x i8>, !dbg !10397
  %_4.i7357 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1525, <16 x i8> zeroinitializer, <16 x i8> %1524), !dbg !10401
  %1526 = fadd <4 x float> %_5.i254416269, splat (float -1.000000e+00), !dbg !10402
  %1527 = fcmp ogt <4 x float> %1526, zeroinitializer, !dbg !10406
  %1528 = select <4 x i1> %1527, <4 x float> %1526, <4 x float> zeroinitializer, !dbg !10410
  %1529 = sext <4 x i1> %1527 to <4 x i32>, !dbg !10411
  %_11.i254916271 = load <4 x float>, ptr %_120.i554, align 16, !dbg !10416
  %_12.i2550 = load <4 x i32>, ptr %1212, align 16, !dbg !10417
  %1530 = bitcast <4 x i32> %_12.i2550 to <4 x float>, !dbg !10418
  %1531 = fadd <4 x float> %_11.i254916271, %1530, !dbg !10422
  %1532 = bitcast <4 x float> %1531 to <16 x i8>, !dbg !10423
  %1533 = bitcast <4 x i32> %1529 to <16 x i8>, !dbg !10427
  %_4.i7358 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1532, <16 x i8> %_13.i255216272, <16 x i8> %1533), !dbg !10428
  store <16 x i8> %_4.i7358, ptr %_120.i554, align 16, !dbg !10429
  %1534 = bitcast <4 x i32> %_12.i2550 to <16 x i8>, !dbg !10430
  %_4.i7359 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1534, <16 x i8> zeroinitializer, <16 x i8> %1533), !dbg !10434
  store <16 x i8> %_4.i7359, ptr %1212, align 16, !dbg !10435
  %start1.i.i.i.i.i.i = shl i32 %iter.i50.sroa.41.018765, 2, !dbg !10436
  %data.i.i.i.i = getelementptr inbounds nuw float, ptr %_221.i527, i32 %start1.i.i.i.i.i.i, !dbg !10450
  %lanes.i5871.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i.i.i, align 4, !dbg !10453, !alias.scope !10459, !noalias !10463
  %data.i.i7349 = getelementptr inbounds nuw float, ptr %_230.i528, i32 %start1.i.i.i.i.i.i, !dbg !10467
  %lanes.i5862.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i7349, align 4, !dbg !10470, !alias.scope !10476, !noalias !10480
  %data.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_206.i514, i32 %start1.i.i.i.i.i.i, !dbg !10484
  %lanes.i5853.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i.i.i.i.i, align 4, !dbg !10486, !alias.scope !10495, !noalias !10499
  %data.i5.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_212.i517, i32 %start1.i.i.i.i.i.i, !dbg !10503
  %lanes.i5844.sroa.0.0.copyload = load <4 x i32>, ptr %data.i5.i.i.i.i.i, align 4, !dbg !10506, !alias.scope !10512, !noalias !10516
  %_235.0.i548 = add nuw nsw i32 %iter.i50.sroa.41.018765, 1, !dbg !10520
  %1535 = bitcast <4 x i32> %lanes.i5871.sroa.0.0.copyload to <4 x float>, !dbg !10523
  %1536 = bitcast <4 x i32> %lanes.i5862.sroa.0.0.copyload to <4 x float>, !dbg !10527
  %1537 = fcmp olt <4 x float> %1535, %1536, !dbg !10528
  %.v16273 = select <4 x i1> %1537, <4 x i32> %lanes.i5862.sroa.0.0.copyload, <4 x i32> %lanes.i5871.sroa.0.0.copyload, !dbg !10529
  %1538 = bitcast <4 x i32> %.v16273 to <16 x i8>, !dbg !10530
  %1539 = bitcast <4 x i32> %lanes.i5862.sroa.0.0.copyload to <16 x i8>, !dbg !10534
  %_4.i7363 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1538, <16 x i8> %1539, <16 x i8> %1214), !dbg !10538
  %_242.i569 = add i32 %iter.i50.sroa.41.018765, %ring_cursor.sroa.0.1.i49918767, !dbg !10539
  %_243.i570 = add i32 %iter.i50.sroa.41.018765, %main_cursor.sroa.0.1.i50018768, !dbg !10544
  %_244.i571 = add i32 %iter.i50.sroa.41.018765, %left_end.sroa.0.0.i, !dbg !10545
  %_245.i572 = add i32 %iter.i50.sroa.41.018765, %start1.sroa.0.0.i2443, !dbg !10546
  %_246.i573 = add i32 %iter.i50.sroa.41.018765, %left_expiring.sroa.0.0.i, !dbg !10547
  %base.i9.i257.i582 = shl i32 %_242.i569, 2, !dbg !10548
  %_7.i10.i258.i583 = add i32 %base.i9.i257.i582, 4, !dbg !10555
  %1540 = or disjoint i32 %base.i9.i257.i582, 3, !dbg !10557
  %or.cond.i13.i261.i586.not = icmp ult i32 %1540, %_54.1.i255.i580, !dbg !10557
  %1541 = bitcast <16 x i8> %_4.i7354 to <4 x float>, !dbg !10557
  %1542 = bitcast <16 x i8> %_4.i7355 to <4 x i32>, !dbg !10557
  %1543 = bitcast <16 x i8> %_4.i7356 to <4 x float>, !dbg !10557
  %1544 = bitcast <16 x i8> %_4.i7357 to <4 x i32>, !dbg !10557
  br i1 %or.cond.i13.i261.i586.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i262.i587, label %bb4.i15.i322.i733, !dbg !10557, !prof !10564

bb4.i15.i322.i733:                                ; preds = %bb75.i532
  store i32 %storemerge.i1788.lcssa2305923108, ptr %_22.i269.i594, align 4
  store i32 %storemerge.i.lcssa2308923126, ptr %_22.i.i676, align 4
  store <4 x float> %1501, ptr %1202, align 16, !dbg !10565
  store <4 x float> %1510, ptr %1205, align 16, !dbg !10566
  store <16 x i8> %_4.i7354, ptr %_115.i550, align 16, !dbg !10567
  store <16 x i8> %_4.i7355, ptr %1206, align 16, !dbg !10568
  store <4 x float> %1519, ptr %1208, align 16, !dbg !10569
  store <16 x i8> %_4.i7356, ptr %_119.i553, align 16, !dbg !10570
  store <16 x i8> %_4.i7357, ptr %1209, align 16, !dbg !10571
  store <4 x float> %1528, ptr %1211, align 16, !dbg !10572
  store i32 %storemerge.i178823046, ptr %_22.i269.i594, align 4, !dbg !10573
  store <4 x float> %1496, ptr %1219, align 16, !dbg !10577
  store i32 %storemerge.i23076, ptr %_22.i.i676, align 4, !dbg !10581
  store <4 x float> %1495, ptr %1229, align 16, !dbg !10584
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i9.i257.i582, i32 noundef %_7.i10.i258.i583, i32 noundef range(i32 0, 536870912) %_54.1.i255.i580, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #32, !dbg !10585, !noalias !10586
  unreachable, !dbg !10585

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i262.i587: ; preds = %bb75.i532
  %1545 = bitcast <4 x i32> %lanes.i5871.sroa.0.0.copyload to <16 x i8>, !dbg !10600
  %_4.i7362 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1538, <16 x i8> %1545, <16 x i8> %1214), !dbg !10601
  %1546 = bitcast <16 x i8> %_4.i7352 to <4 x float>, !dbg !10602
  %1547 = bitcast <16 x i8> %_4.i7362 to <4 x float>, !dbg !10607
  %1548 = fdiv <4 x float> %1546, %1547, !dbg !10608
  %1549 = bitcast <4 x float> %1548 to <16 x i8>, !dbg !10612
  %1550 = fcmp ogt <4 x float> %1547, %1546, !dbg !10616
  %1551 = sext <4 x i1> %1550 to <4 x i32>, !dbg !10616
  %1552 = bitcast <4 x i32> %1551 to <16 x i8>, !dbg !10617
  %_4.i7366 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1549, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %1552), !dbg !10618
  %_17.i14.i263.i588 = getelementptr inbounds nuw float, ptr %_54.0.i254.i579, i32 %base.i9.i257.i582, !dbg !10619
  store <16 x i8> %_4.i7366, ptr %_17.i14.i263.i588, align 4, !dbg !10623, !alias.scope !10628, !noalias !10632
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10636), !dbg !10639
  %base.i1973 = shl i32 %_244.i571, 2, !dbg !10640
  %1553 = or disjoint i32 %base.i1973, 3, !dbg !10643
  %or.cond.i1977.not = icmp ult i32 %1553, %_54.1.i255.i580, !dbg !10643
  br i1 %or.cond.i1977.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1981, label %bb4.i1980, !dbg !10643, !prof !10564

bb4.i1980:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i262.i587
  store i32 %storemerge.i1788.lcssa2305923108, ptr %_22.i269.i594, align 4
  store i32 %storemerge.i.lcssa2308923126, ptr %_22.i.i676, align 4
  store <4 x float> %1501, ptr %1202, align 16, !dbg !10565
  store <4 x float> %1510, ptr %1205, align 16, !dbg !10566
  store <16 x i8> %_4.i7354, ptr %_115.i550, align 16, !dbg !10567
  store <16 x i8> %_4.i7355, ptr %1206, align 16, !dbg !10568
  store <4 x float> %1519, ptr %1208, align 16, !dbg !10569
  store <16 x i8> %_4.i7356, ptr %_119.i553, align 16, !dbg !10570
  store <16 x i8> %_4.i7357, ptr %1209, align 16, !dbg !10571
  store <4 x float> %1528, ptr %1211, align 16, !dbg !10572
  store i32 %storemerge.i178823046, ptr %_22.i269.i594, align 4, !dbg !10573
  store <4 x float> %1496, ptr %1219, align 16, !dbg !10577
  store i32 %storemerge.i23076, ptr %_22.i.i676, align 4, !dbg !10581
  store <4 x float> %1495, ptr %1229, align 16, !dbg !10584
  %_5.i1974 = add i32 %base.i1973, 4, !dbg !10651
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1973, i32 noundef %_5.i1974, i32 noundef range(i32 0, 536870912) %_54.1.i255.i580, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !10652, !noalias !10653
  unreachable, !dbg !10652

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1981: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i262.i587
  %_15.i1979 = getelementptr inbounds nuw float, ptr %_54.0.i254.i579, i32 %base.i1973, !dbg !10659
  %lanes.i5560.sroa.0.0.copyload = load <4 x i32>, ptr %_15.i1979, align 4, !dbg !10663
  %1554 = icmp eq i32 %storemerge.i178823046, 0, !dbg !10668
  %_12.i178016274 = load <4 x float>, ptr %uniform_left.i63, align 16, !dbg !10668
  %1555 = bitcast <4 x i32> %lanes.i5560.sroa.0.0.copyload to <4 x float>, !dbg !10668
  %1556 = fcmp olt <4 x float> %_12.i178016274, %1555, !dbg !10668
  %1557 = select <4 x i1> %1556, <4 x float> %_12.i178016274, <4 x float> %1555, !dbg !10668
  %1558 = bitcast <4 x float> %1557 to <4 x i32>, !dbg !10668
  %.sroa.09557.0 = select i1 %1554, <4 x i32> %lanes.i5560.sroa.0.0.copyload, <4 x i32> %1558, !dbg !10668
  store <4 x i32> %.sroa.09557.0, ptr %uniform_left.i63, align 16, !dbg !10670, !alias.scope !10636, !noalias !10672
  %_15.i1783 = add i32 %storemerge.i178823046, 1, !dbg !10674
  %complete.i1784 = icmp eq i32 %_15.i1783, %_18.i266.i591, !dbg !10674
  br i1 %complete.i1784, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1963, label %bb7.i1785, !dbg !10675

bb7.i1785:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1981
  %base.i1964 = shl i32 %_245.i572, 2, !dbg !10677
  %1559 = or disjoint i32 %base.i1964, 3, !dbg !10679
  %or.cond.i1968.not = icmp ult i32 %1559, %_54.1.i255.i580, !dbg !10679
  br i1 %or.cond.i1968.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1972, label %bb4.i1971, !dbg !10679, !prof !10564

bb4.i1971:                                        ; preds = %bb7.i1785
  store i32 %storemerge.i1788.lcssa2305923108, ptr %_22.i269.i594, align 4
  store i32 %storemerge.i.lcssa2308923126, ptr %_22.i.i676, align 4
  store <4 x float> %1501, ptr %1202, align 16, !dbg !10565
  store <4 x float> %1510, ptr %1205, align 16, !dbg !10566
  store <16 x i8> %_4.i7354, ptr %_115.i550, align 16, !dbg !10567
  store <16 x i8> %_4.i7355, ptr %1206, align 16, !dbg !10568
  store <4 x float> %1519, ptr %1208, align 16, !dbg !10569
  store <16 x i8> %_4.i7356, ptr %_119.i553, align 16, !dbg !10570
  store <16 x i8> %_4.i7357, ptr %1209, align 16, !dbg !10571
  store <4 x float> %1528, ptr %1211, align 16, !dbg !10572
  store i32 %storemerge.i178823046, ptr %_22.i269.i594, align 4, !dbg !10573
  store <4 x float> %1496, ptr %1219, align 16, !dbg !10577
  store i32 %storemerge.i23076, ptr %_22.i.i676, align 4, !dbg !10581
  store <4 x float> %1495, ptr %1229, align 16, !dbg !10584
  %_5.i1965 = add i32 %base.i1964, 4, !dbg !10683
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1964, i32 noundef %_5.i1965, i32 noundef range(i32 0, 536870912) %_54.1.i255.i580, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !10684, !noalias !10685
  unreachable, !dbg !10684

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1972: ; preds = %bb7.i1785
  %_15.i1970 = getelementptr inbounds nuw float, ptr %_54.0.i254.i579, i32 %base.i1964, !dbg !10689
  %lanes.i5567.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1970, align 4, !dbg !10691, !alias.scope !10696, !noalias !10700
  %1560 = bitcast <4 x i32> %.sroa.09557.0 to <4 x float>, !dbg !10704
  %1561 = fcmp olt <4 x float> %lanes.i5567.sroa.0.0.copyload, %1560, !dbg !10711
  %1562 = select <4 x i1> %1561, <4 x float> %lanes.i5567.sroa.0.0.copyload, <4 x float> %1560, !dbg !10712
  %1563 = bitcast <4 x float> %1562 to <4 x i32>, !dbg !10713
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1809, !dbg !10718

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1963: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1981
  %1564 = bitcast <4 x i32> %lanes.i5560.sroa.0.0.copyload to <4 x float>, !dbg !10675
  br i1 %_29.i179318757.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1809, label %bb19.i1794, !dbg !10720

bb19.i1794:                                       ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1963, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1954
  %end.sroa.0.0.i179218759 = phi i32 [ %1570, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1954 ], [ %_244.i571, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1963 ]
  %iter.sroa.0.0.i179118758 = phi i32 [ %_30.i1795, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1954 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1963 ]
  %1565 = phi <4 x float> [ %1568, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1954 ], [ %1564, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1963 ]
  %base.i1946 = shl i32 %end.sroa.0.0.i179218759, 2, !dbg !10729
  %1566 = or disjoint i32 %base.i1946, 3, !dbg !10731
  %or.cond.i1950.not = icmp ult i32 %1566, %_54.1.i255.i580, !dbg !10731
  br i1 %or.cond.i1950.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1954, label %bb4.i1953, !dbg !10731, !prof !10564

bb4.i1953:                                        ; preds = %bb19.i1794
  store i32 %storemerge.i1788.lcssa2305923108, ptr %_22.i269.i594, align 4
  store i32 %storemerge.i.lcssa2308923126, ptr %_22.i.i676, align 4
  store <4 x float> %1501, ptr %1202, align 16, !dbg !10565
  store <4 x float> %1510, ptr %1205, align 16, !dbg !10566
  store <16 x i8> %_4.i7354, ptr %_115.i550, align 16, !dbg !10567
  store <16 x i8> %_4.i7355, ptr %1206, align 16, !dbg !10568
  store <4 x float> %1519, ptr %1208, align 16, !dbg !10569
  store <16 x i8> %_4.i7356, ptr %_119.i553, align 16, !dbg !10570
  store <16 x i8> %_4.i7357, ptr %1209, align 16, !dbg !10571
  store <4 x float> %1528, ptr %1211, align 16, !dbg !10572
  store i32 %storemerge.i178823046, ptr %_22.i269.i594, align 4, !dbg !10573
  store <4 x float> %1496, ptr %1219, align 16, !dbg !10577
  store i32 %storemerge.i23076, ptr %_22.i.i676, align 4, !dbg !10581
  store <4 x float> %1495, ptr %1229, align 16, !dbg !10584
  %_5.i1947 = add i32 %base.i1946, 4, !dbg !10735
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1946, i32 noundef %_5.i1947, i32 noundef range(i32 0, 536870912) %_54.1.i255.i580, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !10736, !noalias !10737
  unreachable, !dbg !10736

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1954: ; preds = %bb19.i1794
  %_30.i1795 = add nuw i32 %iter.sroa.0.0.i179118758, 1, !dbg !10741
  %_15.i1952 = getelementptr inbounds nuw float, ptr %_54.0.i254.i579, i32 %base.i1946, !dbg !10747
  %lanes.i5581.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1952, align 4, !dbg !10749, !alias.scope !10754, !noalias !10758
  %1567 = fcmp olt <4 x float> %1565, %lanes.i5581.sroa.0.0.copyload, !dbg !10762
  %1568 = select <4 x i1> %1567, <4 x float> %1565, <4 x float> %lanes.i5581.sroa.0.0.copyload, !dbg !10766
  store <4 x float> %1568, ptr %_15.i1952, align 4, !dbg !10767, !alias.scope !10773, !noalias !10777
  %1569 = icmp eq i32 %end.sroa.0.0.i179218759, 0, !dbg !10783
  %spec.store.select.i1806 = select i1 %1569, i32 %ring.i76, i32 %end.sroa.0.0.i179218759, !dbg !10783
  %1570 = add i32 %spec.store.select.i1806, -1, !dbg !10784
  %exitcond20962.not = icmp eq i32 %_30.i1795, %_18.i266.i591, !dbg !10785
  br i1 %exitcond20962.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1809, label %bb19.i1794, !dbg !10720

_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1809: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1954, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1963, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1972
  %.sroa.09557.1 = phi <4 x i32> [ %1563, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1972 ], [ %.sroa.09557.0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1963 ], [ %.sroa.09557.0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1954 ], !dbg !10788
  %storemerge.i1788 = phi i32 [ %_15.i1783, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1972 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1963 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1954 ], !dbg !10789
  %1571 = bitcast <4 x i32> %.sroa.09557.1 to <4 x float>, !dbg !10790
  %1572 = fmul <4 x float> %1571, splat (float 1.638400e+04), !dbg !10794
  %1573 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %1572), !dbg !10795
  %1574 = fmul <4 x float> %1573, splat (float 0x3F10000000000000), !dbg !10799
  %base.i2045 = shl i32 %_246.i573, 2, !dbg !10803
  %1575 = or disjoint i32 %base.i2045, 3, !dbg !10805
  %or.cond.i2049.not = icmp ult i32 %1575, %_56.1.i277.i602, !dbg !10805
  br i1 %or.cond.i2049.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2053, label %bb4.i2052, !dbg !10805, !prof !10564

bb4.i2052:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1809
  store i32 %storemerge.i1788.lcssa2305923108, ptr %_22.i269.i594, align 4
  store i32 %storemerge.i.lcssa2308923126, ptr %_22.i.i676, align 4
  store <4 x float> %1501, ptr %1202, align 16, !dbg !10565
  store <4 x float> %1510, ptr %1205, align 16, !dbg !10566
  store <16 x i8> %_4.i7354, ptr %_115.i550, align 16, !dbg !10567
  store <16 x i8> %_4.i7355, ptr %1206, align 16, !dbg !10568
  store <4 x float> %1519, ptr %1208, align 16, !dbg !10569
  store <16 x i8> %_4.i7356, ptr %_119.i553, align 16, !dbg !10570
  store <16 x i8> %_4.i7357, ptr %1209, align 16, !dbg !10571
  store <4 x float> %1528, ptr %1211, align 16, !dbg !10572
  store i32 %storemerge.i1788, ptr %_22.i269.i594, align 4, !dbg !10573
  store <4 x float> %1496, ptr %1219, align 16, !dbg !10577
  store i32 %storemerge.i23076, ptr %_22.i.i676, align 4, !dbg !10581
  store <4 x float> %1495, ptr %1229, align 16, !dbg !10584
  %_5.i2046 = add i32 %base.i2045, 4, !dbg !10809
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i2045, i32 noundef %_5.i2046, i32 noundef range(i32 0, 536870912) %_56.1.i277.i602, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !10810, !noalias !10811
  unreachable, !dbg !10810

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2053: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1809
  %_15.i2051 = getelementptr inbounds nuw float, ptr %_56.0.i276.i601, i32 %base.i2045, !dbg !10815
  %lanes.i5504.sroa.0.0.copyload = load <4 x float>, ptr %_15.i2051, align 4, !dbg !10817, !alias.scope !10822, !noalias !10826
  %1576 = fadd <4 x float> %1574, %1496, !dbg !10830
  %1577 = fsub <4 x float> %1576, %lanes.i5504.sroa.0.0.copyload, !dbg !10834
  %_8.not.i4.i288.i613 = icmp ugt i32 %_7.i10.i258.i583, %_56.1.i277.i602
  br i1 %_8.not.i4.i288.i613, label %bb4.i7.i321.i732, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i290.i615, !dbg !10838, !prof !4596

bb4.i7.i321.i732:                                 ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2053
  store i32 %storemerge.i1788.lcssa2305923108, ptr %_22.i269.i594, align 4
  store i32 %storemerge.i.lcssa2308923126, ptr %_22.i.i676, align 4
  store <4 x float> %1501, ptr %1202, align 16, !dbg !10565
  store <4 x float> %1510, ptr %1205, align 16, !dbg !10566
  store <16 x i8> %_4.i7354, ptr %_115.i550, align 16, !dbg !10567
  store <16 x i8> %_4.i7355, ptr %1206, align 16, !dbg !10568
  store <4 x float> %1519, ptr %1208, align 16, !dbg !10569
  store <16 x i8> %_4.i7356, ptr %_119.i553, align 16, !dbg !10570
  store <16 x i8> %_4.i7357, ptr %1209, align 16, !dbg !10571
  store <4 x float> %1528, ptr %1211, align 16, !dbg !10572
  store i32 %storemerge.i1788, ptr %_22.i269.i594, align 4, !dbg !10573
  store <4 x float> %1577, ptr %1219, align 16, !dbg !10577
  store i32 %storemerge.i23076, ptr %_22.i.i676, align 4, !dbg !10581
  store <4 x float> %1495, ptr %1229, align 16, !dbg !10584
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i9.i257.i582, i32 noundef %_7.i10.i258.i583, i32 noundef range(i32 0, 536870912) %_56.1.i277.i602, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #32, !dbg !10843, !noalias !10844
  unreachable, !dbg !10843

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i290.i615: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2053
  %_17.i6.i291.i616 = getelementptr inbounds nuw float, ptr %_56.0.i276.i601, i32 %base.i9.i257.i582, !dbg !10848
  store <4 x float> %1574, ptr %_17.i6.i291.i616, align 4, !dbg !10850, !alias.scope !10855, !noalias !10859
  %_41.i296.i62116281 = load <4 x float>, ptr %1221, align 16, !dbg !10863
  %1578 = fdiv <4 x float> %1577, %_37.i293.i61816280, !dbg !10866
  %1579 = fsub <4 x float> splat (float 1.000000e+00), %1578, !dbg !10870
  %1580 = fsub <4 x float> %1579, %_41.i296.i62116281, !dbg !10874
  %1581 = bitcast <16 x i8> %_4.i7354 to <4 x float>, !dbg !10878
  %1582 = fmul <4 x float> %1580, %1581, !dbg !10882
  %1583 = fadd <4 x float> %_41.i296.i62116281, %1582, !dbg !10883
  %1584 = fcmp olt <4 x float> %1583, %1579, !dbg !10886
  %1585 = select <4 x i1> %1584, <4 x float> %1579, <4 x float> %1583, !dbg !10891
  %1586 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1585), !dbg !10892
  %1587 = fcmp uge <4 x float> %1586, splat (float 0x3BC79CA100000000), !dbg !10898
  %1588 = bitcast <4 x float> %1585 to <4 x i32>, !dbg !10903
  %1589 = select <4 x i1> %1587, <4 x i32> %1588, <4 x i32> zeroinitializer, !dbg !10903
  store <4 x i32> %1589, ptr %1221, align 16, !dbg !10906
  %base.i2036 = shl i32 %_243.i570, 2, !dbg !10907
  %_5.i2037 = add i32 %base.i2036, 4, !dbg !10910
  %1590 = or disjoint i32 %base.i2036, 3, !dbg !10911
  %or.cond.i2040.not = icmp ult i32 %1590, %_58.1.i307.i632, !dbg !10911
  br i1 %or.cond.i2040.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2044, label %bb4.i2043, !dbg !10911, !prof !10564

bb4.i2043:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i290.i615
  store i32 %storemerge.i1788.lcssa2305923108, ptr %_22.i269.i594, align 4
  store i32 %storemerge.i.lcssa2308923126, ptr %_22.i.i676, align 4
  store <4 x float> %1501, ptr %1202, align 16, !dbg !10565
  store <4 x float> %1510, ptr %1205, align 16, !dbg !10566
  store <16 x i8> %_4.i7354, ptr %_115.i550, align 16, !dbg !10567
  store <16 x i8> %_4.i7355, ptr %1206, align 16, !dbg !10568
  store <4 x float> %1519, ptr %1208, align 16, !dbg !10569
  store <16 x i8> %_4.i7356, ptr %_119.i553, align 16, !dbg !10570
  store <16 x i8> %_4.i7357, ptr %1209, align 16, !dbg !10571
  store <4 x float> %1528, ptr %1211, align 16, !dbg !10572
  store i32 %storemerge.i1788, ptr %_22.i269.i594, align 4, !dbg !10573
  store <4 x float> %1577, ptr %1219, align 16, !dbg !10577
  store i32 %storemerge.i23076, ptr %_22.i.i676, align 4, !dbg !10581
  store <4 x float> %1495, ptr %1229, align 16, !dbg !10584
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i2036, i32 noundef %_5.i2037, i32 noundef range(i32 0, 536870912) %_58.1.i307.i632, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !10915, !noalias !10916
  unreachable, !dbg !10915

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2044: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i290.i615
  %1591 = bitcast <4 x i32> %1589 to <4 x float>, !dbg !10920
  %1592 = fsub <4 x float> splat (float 1.000000e+00), %1591, !dbg !10924
  %_15.i2042 = getelementptr inbounds nuw float, ptr %_58.0.i306.i631, i32 %base.i2036, !dbg !10925
  %lanes.i5511.sroa.0.0.copyload = load <4 x i32>, ptr %_15.i2042, align 4, !dbg !10927, !alias.scope !10932, !noalias !10936
  store <4 x i32> %lanes.i5853.sroa.0.0.copyload, ptr %_15.i2042, align 4, !dbg !10940, !alias.scope !10947, !noalias !10951
  %1593 = bitcast <4 x i32> %lanes.i5511.sroa.0.0.copyload to <4 x float>, !dbg !10957
  %1594 = fmul <4 x float> %1592, %1593, !dbg !10961
  %1595 = bitcast <4 x i32> %lanes.i5511.sroa.0.0.copyload to <16 x i8>, !dbg !10962
  %1596 = bitcast <4 x float> %1594 to <16 x i8>, !dbg !10966
  %_4.i7377 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1595, <16 x i8> %1596, <16 x i8> %1224), !dbg !10967
  store <16 x i8> %_4.i7377, ptr %data.i.i.i.i.i.i, align 4, !dbg !10968, !alias.scope !10973, !noalias !10977
  %_249.i653 = add i32 %iter.i50.sroa.41.018765, %right_end.sroa.0.0.i, !dbg !10981
  %_251.i655 = add i32 %iter.i50.sroa.41.018765, %right_expiring.sroa.0.0.i, !dbg !10983
  %_8.not.i12.i.i667 = icmp ugt i32 %_7.i10.i258.i583, %_54.1.i.i662
  br i1 %_8.not.i12.i.i667, label %bb4.i15.i.i730, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i669, !dbg !10984, !prof !4596

bb4.i15.i.i730:                                   ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2044
  store i32 %storemerge.i1788.lcssa2305923108, ptr %_22.i269.i594, align 4
  store i32 %storemerge.i.lcssa2308923126, ptr %_22.i.i676, align 4
  store <4 x float> %1501, ptr %1202, align 16, !dbg !10565
  store <4 x float> %1510, ptr %1205, align 16, !dbg !10566
  store <16 x i8> %_4.i7354, ptr %_115.i550, align 16, !dbg !10567
  store <16 x i8> %_4.i7355, ptr %1206, align 16, !dbg !10568
  store <4 x float> %1519, ptr %1208, align 16, !dbg !10569
  store <16 x i8> %_4.i7356, ptr %_119.i553, align 16, !dbg !10570
  store <16 x i8> %_4.i7357, ptr %1209, align 16, !dbg !10571
  store <4 x float> %1528, ptr %1211, align 16, !dbg !10572
  store i32 %storemerge.i1788, ptr %_22.i269.i594, align 4, !dbg !10573
  store <4 x float> %1577, ptr %1219, align 16, !dbg !10577
  store i32 %storemerge.i23076, ptr %_22.i.i676, align 4, !dbg !10581
  store <4 x float> %1495, ptr %1229, align 16, !dbg !10584
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i9.i257.i582, i32 noundef %_7.i10.i258.i583, i32 noundef range(i32 0, 536870912) %_54.1.i.i662, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #32, !dbg !10989, !noalias !10990
  unreachable, !dbg !10989

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i669: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2044
  %1597 = bitcast <16 x i8> %_4.i7356 to <4 x float>, !dbg !11004
  %1598 = bitcast <16 x i8> %_4.i7363 to <4 x float>, !dbg !11009
  %1599 = fdiv <4 x float> %1597, %1598, !dbg !11010
  %1600 = bitcast <4 x float> %1599 to <16 x i8>, !dbg !11014
  %1601 = fcmp ogt <4 x float> %1598, %1597, !dbg !11018
  %1602 = sext <4 x i1> %1601 to <4 x i32>, !dbg !11018
  %1603 = bitcast <4 x i32> %1602 to <16 x i8>, !dbg !11019
  %_4.i7379 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1600, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %1603), !dbg !11020
  %_17.i14.i.i670 = getelementptr inbounds nuw float, ptr %_54.0.i.i661, i32 %base.i9.i257.i582, !dbg !11021
  store <16 x i8> %_4.i7379, ptr %_17.i14.i.i670, align 4, !dbg !11023, !alias.scope !11028, !noalias !11032
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11036), !dbg !11039
  %base.i2009 = shl i32 %_249.i653, 2, !dbg !11040
  %1604 = or disjoint i32 %base.i2009, 3, !dbg !11042
  %or.cond.i2013.not = icmp ult i32 %1604, %_54.1.i.i662, !dbg !11042
  br i1 %or.cond.i2013.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2017, label %bb4.i2016, !dbg !11042, !prof !10564

bb4.i2016:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i669
  store i32 %storemerge.i1788.lcssa2305923108, ptr %_22.i269.i594, align 4
  store i32 %storemerge.i.lcssa2308923126, ptr %_22.i.i676, align 4
  store <4 x float> %1501, ptr %1202, align 16, !dbg !10565
  store <4 x float> %1510, ptr %1205, align 16, !dbg !10566
  store <16 x i8> %_4.i7354, ptr %_115.i550, align 16, !dbg !10567
  store <16 x i8> %_4.i7355, ptr %1206, align 16, !dbg !10568
  store <4 x float> %1519, ptr %1208, align 16, !dbg !10569
  store <16 x i8> %_4.i7356, ptr %_119.i553, align 16, !dbg !10570
  store <16 x i8> %_4.i7357, ptr %1209, align 16, !dbg !10571
  store <4 x float> %1528, ptr %1211, align 16, !dbg !10572
  store i32 %storemerge.i1788, ptr %_22.i269.i594, align 4, !dbg !10573
  store <4 x float> %1577, ptr %1219, align 16, !dbg !10577
  store i32 %storemerge.i23076, ptr %_22.i.i676, align 4, !dbg !10581
  store <4 x float> %1495, ptr %1229, align 16, !dbg !10584
  %_5.i2010 = add i32 %base.i2009, 4, !dbg !11046
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i2009, i32 noundef %_5.i2010, i32 noundef range(i32 0, 536870912) %_54.1.i.i662, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !11047, !noalias !11048
  unreachable, !dbg !11047

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2017: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i669
  %_15.i2015 = getelementptr inbounds nuw float, ptr %_54.0.i.i661, i32 %base.i2009, !dbg !11054
  %lanes.i5532.sroa.0.0.copyload = load <4 x i32>, ptr %_15.i2015, align 4, !dbg !11056
  %1605 = icmp eq i32 %storemerge.i23076, 0, !dbg !11061
  %_12.i176516283 = load <4 x float>, ptr %uniform_right.i62, align 16, !dbg !11061
  %1606 = bitcast <4 x i32> %lanes.i5532.sroa.0.0.copyload to <4 x float>, !dbg !11061
  %1607 = fcmp olt <4 x float> %_12.i176516283, %1606, !dbg !11061
  %1608 = select <4 x i1> %1607, <4 x float> %_12.i176516283, <4 x float> %1606, !dbg !11061
  %1609 = bitcast <4 x float> %1608 to <4 x i32>, !dbg !11061
  %.sroa.09483.0 = select i1 %1605, <4 x i32> %lanes.i5532.sroa.0.0.copyload, <4 x i32> %1609, !dbg !11061
  store <4 x i32> %.sroa.09483.0, ptr %uniform_right.i62, align 16, !dbg !11062, !alias.scope !11036, !noalias !11063
  %_15.i = add i32 %storemerge.i23076, 1, !dbg !11065
  %complete.i = icmp eq i32 %_15.i, %_18.i.i673, !dbg !11065
  br i1 %complete.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1999, label %bb7.i1767, !dbg !11066

bb7.i1767:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2017
  %base.i2000 = shl i32 %_245.i572, 2, !dbg !11067
  %1610 = or disjoint i32 %base.i2000, 3, !dbg !11069
  %or.cond.i2004.not = icmp ult i32 %1610, %_54.1.i.i662, !dbg !11069
  br i1 %or.cond.i2004.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2008, label %bb4.i2007, !dbg !11069, !prof !10564

bb4.i2007:                                        ; preds = %bb7.i1767
  store i32 %storemerge.i1788.lcssa2305923108, ptr %_22.i269.i594, align 4
  store i32 %storemerge.i.lcssa2308923126, ptr %_22.i.i676, align 4
  store <4 x float> %1501, ptr %1202, align 16, !dbg !10565
  store <4 x float> %1510, ptr %1205, align 16, !dbg !10566
  store <16 x i8> %_4.i7354, ptr %_115.i550, align 16, !dbg !10567
  store <16 x i8> %_4.i7355, ptr %1206, align 16, !dbg !10568
  store <4 x float> %1519, ptr %1208, align 16, !dbg !10569
  store <16 x i8> %_4.i7356, ptr %_119.i553, align 16, !dbg !10570
  store <16 x i8> %_4.i7357, ptr %1209, align 16, !dbg !10571
  store <4 x float> %1528, ptr %1211, align 16, !dbg !10572
  store i32 %storemerge.i1788, ptr %_22.i269.i594, align 4, !dbg !10573
  store <4 x float> %1577, ptr %1219, align 16, !dbg !10577
  store i32 %storemerge.i23076, ptr %_22.i.i676, align 4, !dbg !10581
  store <4 x float> %1495, ptr %1229, align 16, !dbg !10584
  %_5.i2001 = add i32 %base.i2000, 4, !dbg !11073
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i2000, i32 noundef %_5.i2001, i32 noundef range(i32 0, 536870912) %_54.1.i.i662, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !11074, !noalias !11075
  unreachable, !dbg !11074

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2008: ; preds = %bb7.i1767
  %_15.i2006 = getelementptr inbounds nuw float, ptr %_54.0.i.i661, i32 %base.i2000, !dbg !11079
  %lanes.i5539.sroa.0.0.copyload = load <4 x float>, ptr %_15.i2006, align 4, !dbg !11081, !alias.scope !11086, !noalias !11090
  %1611 = bitcast <4 x i32> %.sroa.09483.0 to <4 x float>, !dbg !11094
  %1612 = fcmp olt <4 x float> %lanes.i5539.sroa.0.0.copyload, %1611, !dbg !11098
  %1613 = select <4 x i1> %1612, <4 x float> %lanes.i5539.sroa.0.0.copyload, <4 x float> %1611, !dbg !11099
  %1614 = bitcast <4 x float> %1613 to <4 x i32>, !dbg !11100
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !11102

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1999: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2017
  %1615 = bitcast <4 x i32> %lanes.i5532.sroa.0.0.copyload to <4 x float>, !dbg !11066
  br i1 %_29.i177118760.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb19.i1772, !dbg !11103

bb19.i1772:                                       ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1999, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1990
  %end.sroa.0.0.i18762 = phi i32 [ %1621, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1990 ], [ %_249.i653, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1999 ]
  %iter.sroa.0.0.i177018761 = phi i32 [ %_30.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1990 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1999 ]
  %1616 = phi <4 x float> [ %1619, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1990 ], [ %1615, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1999 ]
  %base.i1982 = shl i32 %end.sroa.0.0.i18762, 2, !dbg !11106
  %1617 = or disjoint i32 %base.i1982, 3, !dbg !11108
  %or.cond.i1986.not = icmp ult i32 %1617, %_54.1.i.i662, !dbg !11108
  br i1 %or.cond.i1986.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1990, label %bb4.i1989, !dbg !11108, !prof !10564

bb4.i1989:                                        ; preds = %bb19.i1772
  store i32 %storemerge.i1788.lcssa2305923108, ptr %_22.i269.i594, align 4
  store i32 %storemerge.i.lcssa2308923126, ptr %_22.i.i676, align 4
  store <4 x float> %1501, ptr %1202, align 16, !dbg !10565
  store <4 x float> %1510, ptr %1205, align 16, !dbg !10566
  store <16 x i8> %_4.i7354, ptr %_115.i550, align 16, !dbg !10567
  store <16 x i8> %_4.i7355, ptr %1206, align 16, !dbg !10568
  store <4 x float> %1519, ptr %1208, align 16, !dbg !10569
  store <16 x i8> %_4.i7356, ptr %_119.i553, align 16, !dbg !10570
  store <16 x i8> %_4.i7357, ptr %1209, align 16, !dbg !10571
  store <4 x float> %1528, ptr %1211, align 16, !dbg !10572
  store i32 %storemerge.i1788, ptr %_22.i269.i594, align 4, !dbg !10573
  store <4 x float> %1577, ptr %1219, align 16, !dbg !10577
  store i32 %storemerge.i23076, ptr %_22.i.i676, align 4, !dbg !10581
  store <4 x float> %1495, ptr %1229, align 16, !dbg !10584
  %_5.i1983 = add i32 %base.i1982, 4, !dbg !11112
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1982, i32 noundef %_5.i1983, i32 noundef range(i32 0, 536870912) %_54.1.i.i662, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !11113, !noalias !11114
  unreachable, !dbg !11113

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1990: ; preds = %bb19.i1772
  %_30.i = add nuw i32 %iter.sroa.0.0.i177018761, 1, !dbg !11118
  %_15.i1988 = getelementptr inbounds nuw float, ptr %_54.0.i.i661, i32 %base.i1982, !dbg !11121
  %lanes.i5553.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1988, align 4, !dbg !11123, !alias.scope !11128, !noalias !11132
  %1618 = fcmp olt <4 x float> %1616, %lanes.i5553.sroa.0.0.copyload, !dbg !11136
  %1619 = select <4 x i1> %1618, <4 x float> %1616, <4 x float> %lanes.i5553.sroa.0.0.copyload, !dbg !11140
  store <4 x float> %1619, ptr %_15.i1988, align 4, !dbg !11141, !alias.scope !11147, !noalias !11151
  %1620 = icmp eq i32 %end.sroa.0.0.i18762, 0, !dbg !11157
  %spec.store.select.i1775 = select i1 %1620, i32 %ring.i76, i32 %end.sroa.0.0.i18762, !dbg !11157
  %1621 = add i32 %spec.store.select.i1775, -1, !dbg !11158
  %exitcond20967.not = icmp eq i32 %_30.i, %_18.i.i673, !dbg !11159
  br i1 %exitcond20967.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb19.i1772, !dbg !11103

_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1990, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1999, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2008
  %.sroa.09483.1 = phi <4 x i32> [ %1614, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2008 ], [ %.sroa.09483.0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1999 ], [ %.sroa.09483.0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1990 ], !dbg !11161
  %storemerge.i = phi i32 [ %_15.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2008 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1999 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1990 ], !dbg !11162
  %1622 = bitcast <4 x i32> %.sroa.09483.1 to <4 x float>, !dbg !11163
  %1623 = fmul <4 x float> %1622, splat (float 1.638400e+04), !dbg !11167
  %1624 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %1623), !dbg !11168
  %1625 = fmul <4 x float> %1624, splat (float 0x3F10000000000000), !dbg !11172
  %base.i2027 = shl i32 %_251.i655, 2, !dbg !11176
  %1626 = or disjoint i32 %base.i2027, 3, !dbg !11178
  %or.cond.i2031.not = icmp ult i32 %1626, %_56.1.i.i684, !dbg !11178
  br i1 %or.cond.i2031.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2035, label %bb4.i2034, !dbg !11178, !prof !10564

bb4.i2034:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
  store i32 %storemerge.i1788.lcssa2305923108, ptr %_22.i269.i594, align 4
  store i32 %storemerge.i.lcssa2308923126, ptr %_22.i.i676, align 4
  store <4 x float> %1501, ptr %1202, align 16, !dbg !10565
  store <4 x float> %1510, ptr %1205, align 16, !dbg !10566
  store <16 x i8> %_4.i7354, ptr %_115.i550, align 16, !dbg !10567
  store <16 x i8> %_4.i7355, ptr %1206, align 16, !dbg !10568
  store <4 x float> %1519, ptr %1208, align 16, !dbg !10569
  store <16 x i8> %_4.i7356, ptr %_119.i553, align 16, !dbg !10570
  store <16 x i8> %_4.i7357, ptr %1209, align 16, !dbg !10571
  store <4 x float> %1528, ptr %1211, align 16, !dbg !10572
  store i32 %storemerge.i1788, ptr %_22.i269.i594, align 4, !dbg !10573
  store <4 x float> %1577, ptr %1219, align 16, !dbg !10577
  store i32 %storemerge.i, ptr %_22.i.i676, align 4, !dbg !10581
  store <4 x float> %1495, ptr %1229, align 16, !dbg !10584
  %_5.i2028 = add i32 %base.i2027, 4, !dbg !11182
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i2027, i32 noundef %_5.i2028, i32 noundef range(i32 0, 536870912) %_56.1.i.i684, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !11183, !noalias !11184
  unreachable, !dbg !11183

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2035: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
  %_15.i2033 = getelementptr inbounds nuw float, ptr %_56.0.i.i683, i32 %base.i2027, !dbg !11188
  %lanes.i5518.sroa.0.0.copyload = load <4 x float>, ptr %_15.i2033, align 4, !dbg !11190, !alias.scope !11195, !noalias !11199
  %1627 = fadd <4 x float> %1625, %1495, !dbg !11203
  %1628 = fsub <4 x float> %1627, %lanes.i5518.sroa.0.0.copyload, !dbg !11207
  %_8.not.i4.i.i695 = icmp ugt i32 %_7.i10.i258.i583, %_56.1.i.i684
  br i1 %_8.not.i4.i.i695, label %bb4.i7.i.i729, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i697, !dbg !11211, !prof !4596

bb4.i7.i.i729:                                    ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2035
  store i32 %storemerge.i1788.lcssa2305923108, ptr %_22.i269.i594, align 4
  store i32 %storemerge.i.lcssa2308923126, ptr %_22.i.i676, align 4
  store <4 x float> %1501, ptr %1202, align 16, !dbg !10565
  store <4 x float> %1510, ptr %1205, align 16, !dbg !10566
  store <16 x i8> %_4.i7354, ptr %_115.i550, align 16, !dbg !10567
  store <16 x i8> %_4.i7355, ptr %1206, align 16, !dbg !10568
  store <4 x float> %1519, ptr %1208, align 16, !dbg !10569
  store <16 x i8> %_4.i7356, ptr %_119.i553, align 16, !dbg !10570
  store <16 x i8> %_4.i7357, ptr %1209, align 16, !dbg !10571
  store <4 x float> %1528, ptr %1211, align 16, !dbg !10572
  store i32 %storemerge.i1788, ptr %_22.i269.i594, align 4, !dbg !10573
  store <4 x float> %1577, ptr %1219, align 16, !dbg !10577
  store i32 %storemerge.i, ptr %_22.i.i676, align 4, !dbg !10581
  store <4 x float> %1628, ptr %1229, align 16, !dbg !10584
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i9.i257.i582, i32 noundef %_7.i10.i258.i583, i32 noundef range(i32 0, 536870912) %_56.1.i.i684, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #32, !dbg !11216, !noalias !11217
  unreachable, !dbg !11216

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i697: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2035
  %_17.i6.i.i698 = getelementptr inbounds nuw float, ptr %_56.0.i.i683, i32 %base.i9.i257.i582, !dbg !11221
  store <4 x float> %1625, ptr %_17.i6.i.i698, align 4, !dbg !11223, !alias.scope !11228, !noalias !11232
  %_41.i.i70316290 = load <4 x float>, ptr %1231, align 16, !dbg !11236
  %1629 = fdiv <4 x float> %1628, %_37.i.i70016289, !dbg !11237
  %1630 = fsub <4 x float> splat (float 1.000000e+00), %1629, !dbg !11241
  %1631 = fsub <4 x float> %1630, %_41.i.i70316290, !dbg !11245
  %1632 = bitcast <16 x i8> %_4.i7358 to <4 x float>, !dbg !11249
  %1633 = fmul <4 x float> %1631, %1632, !dbg !11253
  %1634 = fadd <4 x float> %_41.i.i70316290, %1633, !dbg !11254
  %1635 = fcmp olt <4 x float> %1634, %1630, !dbg !11257
  %1636 = select <4 x i1> %1635, <4 x float> %1630, <4 x float> %1634, !dbg !11261
  %1637 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1636), !dbg !11262
  %1638 = fcmp uge <4 x float> %1637, splat (float 0x3BC79CA100000000), !dbg !11267
  %1639 = bitcast <4 x float> %1636 to <4 x i32>, !dbg !11272
  %1640 = select <4 x i1> %1638, <4 x i32> %1639, <4 x i32> zeroinitializer, !dbg !11272
  store <4 x i32> %1640, ptr %1231, align 16, !dbg !11275
  %_6.not.i2021 = icmp ugt i32 %_5.i2037, %_58.1.i.i714
  br i1 %_6.not.i2021, label %bb4.i2025, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2026, !dbg !11276, !prof !4596

bb4.i2025:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i697
  store i32 %storemerge.i1788.lcssa2305923108, ptr %_22.i269.i594, align 4
  store i32 %storemerge.i.lcssa2308923126, ptr %_22.i.i676, align 4
  store <4 x float> %1501, ptr %1202, align 16, !dbg !10565
  store <4 x float> %1510, ptr %1205, align 16, !dbg !10566
  store <16 x i8> %_4.i7354, ptr %_115.i550, align 16, !dbg !10567
  store <16 x i8> %_4.i7355, ptr %1206, align 16, !dbg !10568
  store <4 x float> %1519, ptr %1208, align 16, !dbg !10569
  store <16 x i8> %_4.i7356, ptr %_119.i553, align 16, !dbg !10570
  store <16 x i8> %_4.i7357, ptr %1209, align 16, !dbg !10571
  store <4 x float> %1528, ptr %1211, align 16, !dbg !10572
  store i32 %storemerge.i1788, ptr %_22.i269.i594, align 4, !dbg !10573
  store <4 x float> %1577, ptr %1219, align 16, !dbg !10577
  store i32 %storemerge.i, ptr %_22.i.i676, align 4, !dbg !10581
  store <4 x float> %1628, ptr %1229, align 16, !dbg !10584
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i2036, i32 noundef %_5.i2037, i32 noundef range(i32 0, 536870912) %_58.1.i.i714, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !11281, !noalias !11282
  unreachable, !dbg !11281

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2026: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i697
  %1641 = bitcast <4 x i32> %1640 to <4 x float>, !dbg !11286
  %1642 = fsub <4 x float> splat (float 1.000000e+00), %1641, !dbg !11290
  %_15.i2024 = getelementptr inbounds nuw float, ptr %_58.0.i.i713, i32 %base.i2036, !dbg !11291
  %lanes.i5525.sroa.0.0.copyload = load <4 x i32>, ptr %_15.i2024, align 4, !dbg !11293, !alias.scope !11298, !noalias !11302
  store <4 x i32> %lanes.i5844.sroa.0.0.copyload, ptr %_15.i2024, align 4, !dbg !11306, !alias.scope !11312, !noalias !11316
  %1643 = bitcast <4 x i32> %lanes.i5525.sroa.0.0.copyload to <4 x float>, !dbg !11322
  %1644 = fmul <4 x float> %1642, %1643, !dbg !11326
  %1645 = bitcast <4 x i32> %lanes.i5525.sroa.0.0.copyload to <16 x i8>, !dbg !11327
  %1646 = bitcast <4 x float> %1644 to <16 x i8>, !dbg !11331
  %_4.i7390 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1645, <16 x i8> %1646, <16 x i8> %1224), !dbg !11332
  store <16 x i8> %_4.i7390, ptr %data.i5.i.i.i.i.i, align 4, !dbg !11333, !alias.scope !11338, !noalias !11342
  %exitcond20979.not = icmp eq i32 %_235.0.i548, %1494, !dbg !10282
  br i1 %exitcond20979.not, label %bb74.i734.loopexit, label %bb75.i532, !dbg !10282

bb74.i734.loopexit:                               ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2026
  store <4 x float> %1501, ptr %1202, align 16, !dbg !10565
  store <4 x float> %1510, ptr %1205, align 16, !dbg !10566
  store <16 x i8> %_4.i7354, ptr %_115.i550, align 16, !dbg !10567
  store <16 x i8> %_4.i7355, ptr %1206, align 16, !dbg !10568
  store <4 x float> %1519, ptr %1208, align 16, !dbg !10569
  store <16 x i8> %_4.i7356, ptr %_119.i553, align 16, !dbg !10570
  store <16 x i8> %_4.i7357, ptr %1209, align 16, !dbg !10571
  store <4 x float> %1528, ptr %1211, align 16, !dbg !10572
  store <4 x float> %1577, ptr %1219, align 16, !dbg !10577
  store <4 x float> %1628, ptr %1229, align 16, !dbg !10584
  br label %bb74.i734, !dbg !11346

bb74.i734:                                        ; preds = %bb74.i734.loopexit, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit
  %storemerge.i.lcssa2308923125 = phi i32 [ %storemerge.i, %bb74.i734.loopexit ], [ %storemerge.i.lcssa2308923126, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ]
  %storemerge.i1788.lcssa2305923107 = phi i32 [ %storemerge.i1788, %bb74.i734.loopexit ], [ %storemerge.i1788.lcssa2305923108, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ]
  %_143.i735 = add i32 %run.sroa.0.0.i, %ring_cursor.sroa.0.1.i49918767, !dbg !11346
  %_241.not.i736 = icmp ult i32 %_143.i735, %ring.i76, !dbg !11347
  %1647 = select i1 %_241.not.i736, i32 0, i32 %ring.i76, !dbg !11347
  %ring_cursor.sroa.0.2.i737 = sub nuw i32 %_143.i735, %1647, !dbg !11347
  %_145.i738 = add i32 %run.sroa.0.0.i, %main_cursor.sroa.0.1.i50018768, !dbg !11350
  %_252.not.i739 = icmp ult i32 %_145.i738, %main.i77, !dbg !11351
  %1648 = select i1 %_252.not.i739, i32 0, i32 %main.i77, !dbg !11351
  %main_cursor.sroa.0.2.i740 = sub nuw i32 %_145.i738, %1648, !dbg !11351
  %_63.i502 = icmp ult i32 %_87.i519, %spec.store.select.i92, !dbg !10171
  br i1 %_63.i502, label %bb20.i503, label %bb15.i85.loopexit.loopexit, !dbg !10171

_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %bb15.i85.loopexit, %bb7.i
  %ring_cursor.sroa.0.0.i86.lcssa = phi i32 [ %_37.i79, %bb7.i ], [ %ring_cursor.sroa.0.1.i499.lcssa, %bb15.i85.loopexit ], !dbg !9159
  %main_cursor.sroa.0.0.i87.lcssa = phi i32 [ %_36.i78, %bb7.i ], [ %main_cursor.sroa.0.1.i500.lcssa, %bb15.i85.loopexit ], !dbg !9156
  %left_prefix.i745 = load <4 x i32>, ptr %uniform_left.i63, align 16, !dbg !11353, !noalias !9163
  %1649 = getelementptr inbounds nuw i8, ptr %uniform_left.i63, i32 16, !dbg !11354
  %left_phase.i746 = load i32, ptr %1649, align 16, !dbg !11354, !noalias !9163, !noundef !10
  %right_prefix.i747 = load <4 x i32>, ptr %uniform_right.i62, align 16, !dbg !11355, !noalias !9163
  %1650 = getelementptr inbounds nuw i8, ptr %uniform_right.i62, i32 16, !dbg !11356
  %right_phase.i748 = load i32, ptr %1650, align 16, !dbg !11356, !noalias !9163, !noundef !10
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_right.i62), !dbg !11357, !noalias !9163
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i63), !dbg !11358, !noalias !9163
  %1651 = getelementptr inbounds nuw i8, ptr %self, i32 968, !dbg !11359
  %_265.1.i750 = load i32, ptr %1651, align 4, !dbg !11359, !alias.scope !9130, !noalias !11361, !noundef !10
  %_8.i6517 = icmp samesign ugt i32 %_265.1.i750, 3, !dbg !11362
  br i1 %_8.i6517, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6520, label %bb2.i6518, !dbg !11362, !prof !1039

bb2.i6518:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_265.1.i750, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !11367, !noalias !11368
  unreachable, !dbg !11367

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6520: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
  %1652 = getelementptr inbounds nuw i8, ptr %self, i32 964, !dbg !11359
  %_265.0.i749 = load ptr, ptr %1652, align 4, !dbg !11359, !alias.scope !9130, !noalias !11361, !nonnull !10, !noundef !10
  store <4 x i32> %left_prefix.i745, ptr %_265.0.i749, align 4, !dbg !11372, !alias.scope !11376, !noalias !11380
  %_266.0.i751 = load ptr, ptr %68, align 4, !dbg !11382, !alias.scope !9130, !noalias !11361, !nonnull !10, !noundef !10
  %_266.1.i752 = load i32, ptr %69, align 4, !dbg !11382, !alias.scope !9130, !noalias !11361, !noundef !10
  %1653 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i746), !dbg !11383
  br i1 %1653, label %bb2.i7397, label %bb6.i7393, !dbg !11383

bb6.i7393:                                        ; preds = %bb2.i7397, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6520
  %end_or_len.idx.i = shl nuw nsw i32 %_266.1.i752, 2, !dbg !11387
  %end_or_len.i = getelementptr inbounds nuw i8, ptr %_266.0.i751, i32 %end_or_len.idx.i, !dbg !11387
  %_293.i = icmp eq i32 %_266.1.i752, 0, !dbg !11396
  br i1 %_293.i, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i7394, !dbg !11405

bb2.i7397:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6520
  %bytes1.sroa.0.0.zext.i = and i32 %left_phase.i746, 255, !dbg !11406
  %bytes1.sroa.0.0.isplat.i = mul nuw i32 %bytes1.sroa.0.0.zext.i, 16843009, !dbg !11406
  %_5.i7398 = icmp eq i32 %left_phase.i746, %bytes1.sroa.0.0.isplat.i, !dbg !11407
  br i1 %_5.i7398, label %bb3.i7399, label %bb6.i7393, !dbg !11407

bb3.i7399:                                        ; preds = %bb2.i7397
  %bytes.sroa.0.0.extract.trunc.i = trunc i32 %left_phase.i746 to i8, !dbg !11408
  %1654 = shl nuw nsw i32 %_266.1.i752, 2, !dbg !11411
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %_266.0.i751, i8 %bytes.sroa.0.0.extract.trunc.i, i32 %1654, i1 false), !dbg !11411, !alias.scope !11412, !noalias !9134
  br label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, !dbg !11415

bb10.i7394:                                       ; preds = %bb6.i7393, %bb10.i7394
  %iter.sroa.0.04.i = phi ptr [ %_38.i7395, %bb10.i7394 ], [ %_266.0.i751, %bb6.i7393 ]
  %_38.i7395 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i, i32 4, !dbg !11416
  store i32 %left_phase.i746, ptr %iter.sroa.0.04.i, align 4, !dbg !11419, !alias.scope !11412, !noalias !9134
  %_29.i7396 = icmp eq ptr %_38.i7395, %end_or_len.i, !dbg !11396
  br i1 %_29.i7396, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i7394, !dbg !11405

_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit: ; preds = %bb10.i7394, %bb6.i7393, %bb3.i7399
  %1655 = getelementptr inbounds nuw i8, ptr %self, i32 1068, !dbg !11421
  %_267.1.i754 = load i32, ptr %1655, align 4, !dbg !11421, !alias.scope !9132, !noalias !11422, !noundef !10
  %_8.i6512 = icmp samesign ugt i32 %_267.1.i754, 3, !dbg !11423
  br i1 %_8.i6512, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6515, label %bb2.i6513, !dbg !11423, !prof !1039

bb2.i6513:                                        ; preds = %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_267.1.i754, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !11428, !noalias !11429
  unreachable, !dbg !11428

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6515: ; preds = %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit
  %1656 = getelementptr inbounds nuw i8, ptr %self, i32 1064, !dbg !11421
  %_267.0.i753 = load ptr, ptr %1656, align 4, !dbg !11421, !alias.scope !9132, !noalias !11422, !nonnull !10, !noundef !10
  store <4 x i32> %right_prefix.i747, ptr %_267.0.i753, align 4, !dbg !11433, !alias.scope !11437, !noalias !11441
  %_268.0.i755 = load ptr, ptr %77, align 4, !dbg !11443, !alias.scope !9132, !noalias !11422, !nonnull !10, !noundef !10
  %_268.1.i756 = load i32, ptr %78, align 4, !dbg !11443, !alias.scope !9132, !noalias !11422, !noundef !10
  %1657 = tail call i1 @llvm.is.constant.i32(i32 %right_phase.i748), !dbg !11444
  br i1 %1657, label %bb2.i7409, label %bb6.i7401, !dbg !11444

bb6.i7401:                                        ; preds = %bb2.i7409, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6515
  %end_or_len.idx.i7402 = shl nuw nsw i32 %_268.1.i756, 2, !dbg !11447
  %end_or_len.i7403 = getelementptr inbounds nuw i8, ptr %_268.0.i755, i32 %end_or_len.idx.i7402, !dbg !11447
  %_293.i7404 = icmp eq i32 %_268.1.i756, 0, !dbg !11451
  br i1 %_293.i7404, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7415, label %bb10.i7405, !dbg !11454

bb2.i7409:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6515
  %bytes1.sroa.0.0.zext.i7410 = and i32 %right_phase.i748, 255, !dbg !11455
  %bytes1.sroa.0.0.isplat.i7411 = mul nuw i32 %bytes1.sroa.0.0.zext.i7410, 16843009, !dbg !11455
  %_5.i7412 = icmp eq i32 %right_phase.i748, %bytes1.sroa.0.0.isplat.i7411, !dbg !11456
  br i1 %_5.i7412, label %bb3.i7413, label %bb6.i7401, !dbg !11456

bb3.i7413:                                        ; preds = %bb2.i7409
  %bytes.sroa.0.0.extract.trunc.i7414 = trunc i32 %right_phase.i748 to i8, !dbg !11457
  %1658 = shl nuw nsw i32 %_268.1.i756, 2, !dbg !11459
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %_268.0.i755, i8 %bytes.sroa.0.0.extract.trunc.i7414, i32 %1658, i1 false), !dbg !11459, !alias.scope !11460, !noalias !9134
  br label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7415, !dbg !11463

bb10.i7405:                                       ; preds = %bb6.i7401, %bb10.i7405
  %iter.sroa.0.04.i7406 = phi ptr [ %_38.i7407, %bb10.i7405 ], [ %_268.0.i755, %bb6.i7401 ]
  %_38.i7407 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i7406, i32 4, !dbg !11464
  store i32 %right_phase.i748, ptr %iter.sroa.0.04.i7406, align 4, !dbg !11466, !alias.scope !11460, !noalias !9134
  %_29.i7408 = icmp eq ptr %_38.i7407, %end_or_len.i7403, !dbg !11451
  br i1 %_29.i7408, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7415, label %bb10.i7405, !dbg !11454

_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7415: ; preds = %bb10.i7405, %bb6.i7401, %bb3.i7413
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_(ptr noalias noundef readonly align 16 captures(none) dereferenceable(368) %hot_left.i68, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33) #31, !dbg !11467
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_(ptr noalias noundef readonly align 16 captures(none) dereferenceable(368) %hot_right.i67, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34) #31, !dbg !11468
  store i32 %main_cursor.sroa.0.0.i87.lcssa, ptr %_35, align 4, !dbg !11469, !alias.scope !9134, !noalias !9158
  store i32 %ring_cursor.sroa.0.0.i86.lcssa, ptr %86, align 4, !dbg !11470, !alias.scope !9134, !noalias !9158
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i64), !dbg !11471, !noalias !9163
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i65), !dbg !11472, !noalias !9163
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !9127

bb6.i:                                            ; preds = %bb5.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11473), !dbg !11476
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11477), !dbg !11476
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11479), !dbg !11476
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11481), !dbg !11476
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11483), !dbg !11476
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_(ptr noalias noundef align 16 captures(none) dereferenceable(368) %hot_left.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #31, !dbg !11485
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_(ptr noalias noundef align 16 captures(none) dereferenceable(368) %hot_right.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #31, !dbg !11489
  %1659 = load i8, ptr %82, align 16, !dbg !11491, !range !4667, !alias.scope !11473, !noalias !11495, !noundef !10
  %1660 = load i8, ptr %83, align 1, !dbg !11498, !range !4667, !alias.scope !11473, !noalias !11495, !noundef !10
  %ring.i = load i32, ptr %84, align 4, !dbg !11500, !alias.scope !11477, !noalias !11502, !noundef !10
  %main.i = load i32, ptr %85, align 4, !dbg !11503, !alias.scope !11477, !noalias !11502, !noundef !10
  %_36.i = load i32, ptr %_35, align 4, !dbg !11505, !alias.scope !11483, !noalias !11507, !noundef !10
  %_37.i = load i32, ptr %86, align 4, !dbg !11508, !alias.scope !11483, !noalias !11507, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i), !dbg !11510, !noalias !11512
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i, i8 0, i32 1024, i1 false), !noalias !11512
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i), !dbg !11513, !noalias !11512
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i, i8 0, i32 1024, i1 false), !noalias !11512
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i), !dbg !11515, !noalias !11512
; call <true_peak_limiter::UniformHot<wide::f32x4_::f32x4>>::new
  call fastcc void @_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E3newB5_(ptr noalias noundef align 16 captures(none) dereferenceable(64) %uniform_left.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33, i32 %ring.i, i32 %main.i) #31, !dbg !11517
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_right.i), !dbg !11518, !noalias !11512
  %_32.val6809 = load i32, ptr %84, align 4, !dbg !11520, !noundef !10
  %_32.val6810 = load i32, ptr %85, align 4, !dbg !11520, !noundef !10
; call <true_peak_limiter::UniformHot<wide::f32x4_::f32x4>>::new
  call fastcc void @_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E3newB5_(ptr noalias noundef align 16 captures(none) dereferenceable(64) %uniform_right.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34, i32 %_32.val6809, i32 %_32.val6810) #31, !dbg !11520
  %_166.not.i19106 = icmp eq i32 %frames, 0, !dbg !11521
  br i1 %_166.not.i19106, label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb44.i.lr.ph, !dbg !11521

bb44.i.lr.ph:                                     ; preds = %bb6.i
  %_33.i = trunc nuw i8 %1660 to i1, !dbg !11498
  %spec.store.select25.i = select i1 %_33.i, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !11498
  %_32.i = trunc nuw i8 %1659 to i1, !dbg !11491
  %link.sroa.0.0.i = select i1 %_32.i, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !11491
  %d9.i7416 = lshr i32 %frames, 5, !dbg !11531
  %r2.i7417 = and i32 %frames, 31, !dbg !11538
  %_19.not.i7418 = icmp ne i32 %r2.i7417, 0, !dbg !11539
  %1661 = zext i1 %_19.not.i7418 to i32, !dbg !11539
  %yield_count.sroa.0.0.i7419 = add nuw nsw i32 %d9.i7416, %1661, !dbg !11539
  %history.i46.i.sroa.7.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 16
  %history.i46.i.sroa.10.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 32
  %history.i46.i.sroa.13.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 48
  %history.i46.i.sroa.16.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 64
  %history.i46.i.sroa.19.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 80
  %history.i46.i.sroa.22.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 96
  %history.i46.i.sroa.26.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 112
  %history.i46.i.sroa.29.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 128
  %history.i46.i.sroa.32.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 144
  %history.i46.i.sroa.35.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 160
  %history.i46.i.sroa.38.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 176
  %1662 = getelementptr inbounds nuw i8, ptr %self, i32 32
  %1663 = getelementptr inbounds nuw i8, ptr %self, i32 48
  %1664 = getelementptr inbounds nuw i8, ptr %self, i32 64
  %row1.i.i.i79.i = getelementptr inbounds nuw i8, ptr %self, i32 80
  %1665 = getelementptr inbounds nuw i8, ptr %self, i32 96
  %1666 = getelementptr inbounds nuw i8, ptr %self, i32 112
  %1667 = getelementptr inbounds nuw i8, ptr %self, i32 128
  %row3.i.i.i93.i = getelementptr inbounds nuw i8, ptr %self, i32 144
  %1668 = getelementptr inbounds nuw i8, ptr %self, i32 160
  %1669 = getelementptr inbounds nuw i8, ptr %self, i32 176
  %1670 = getelementptr inbounds nuw i8, ptr %self, i32 192
  %row5.i.i.i107.i = getelementptr inbounds nuw i8, ptr %self, i32 208
  %1671 = getelementptr inbounds nuw i8, ptr %self, i32 224
  %1672 = getelementptr inbounds nuw i8, ptr %self, i32 240
  %1673 = getelementptr inbounds nuw i8, ptr %self, i32 256
  %row7.i.i.i121.i = getelementptr inbounds nuw i8, ptr %self, i32 272
  %1674 = getelementptr inbounds nuw i8, ptr %self, i32 288
  %1675 = getelementptr inbounds nuw i8, ptr %self, i32 304
  %1676 = getelementptr inbounds nuw i8, ptr %self, i32 320
  %row9.i.i.i135.i = getelementptr inbounds nuw i8, ptr %self, i32 336
  %1677 = getelementptr inbounds nuw i8, ptr %self, i32 352
  %1678 = getelementptr inbounds nuw i8, ptr %self, i32 368
  %1679 = getelementptr inbounds nuw i8, ptr %self, i32 384
  %row11.i.i.i149.i = getelementptr inbounds nuw i8, ptr %self, i32 400
  %1680 = getelementptr inbounds nuw i8, ptr %self, i32 416
  %1681 = getelementptr inbounds nuw i8, ptr %self, i32 432
  %1682 = getelementptr inbounds nuw i8, ptr %self, i32 448
  %row13.i.i.i163.i = getelementptr inbounds nuw i8, ptr %self, i32 464
  %1683 = getelementptr inbounds nuw i8, ptr %self, i32 480
  %1684 = getelementptr inbounds nuw i8, ptr %self, i32 496
  %1685 = getelementptr inbounds nuw i8, ptr %self, i32 512
  %row15.i.i.i177.i = getelementptr inbounds nuw i8, ptr %self, i32 528
  %1686 = getelementptr inbounds nuw i8, ptr %self, i32 544
  %1687 = getelementptr inbounds nuw i8, ptr %self, i32 560
  %1688 = getelementptr inbounds nuw i8, ptr %self, i32 576
  %row17.i.i.i191.i = getelementptr inbounds nuw i8, ptr %self, i32 592
  %1689 = getelementptr inbounds nuw i8, ptr %self, i32 608
  %1690 = getelementptr inbounds nuw i8, ptr %self, i32 624
  %1691 = getelementptr inbounds nuw i8, ptr %self, i32 640
  %row19.i.i.i205.i = getelementptr inbounds nuw i8, ptr %self, i32 656
  %1692 = getelementptr inbounds nuw i8, ptr %self, i32 672
  %1693 = getelementptr inbounds nuw i8, ptr %self, i32 688
  %1694 = getelementptr inbounds nuw i8, ptr %self, i32 704
  %row21.i.i.i219.i = getelementptr inbounds nuw i8, ptr %self, i32 720
  %1695 = getelementptr inbounds nuw i8, ptr %self, i32 736
  %1696 = getelementptr inbounds nuw i8, ptr %self, i32 752
  %1697 = getelementptr inbounds nuw i8, ptr %self, i32 768
  %history.i.i.sroa.7.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 16
  %history.i.i.sroa.10.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 32
  %history.i.i.sroa.13.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 48
  %history.i.i.sroa.16.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 64
  %history.i.i.sroa.19.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 80
  %history.i.i.sroa.22.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 96
  %history.i.i.sroa.26.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 112
  %history.i.i.sroa.29.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 128
  %history.i.i.sroa.32.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 144
  %history.i.i.sroa.35.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 160
  %history.i.i.sroa.38.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 176
  %1698 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 20
  %_72.i.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 24
  %_72.i.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 28
  %1699 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 20
  %_73.i.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 24
  %_73.i.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 28
  %_114.i = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 192
  %_115.i = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 256
  %_119.i = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 192
  %_120.i = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 256
  %1700 = bitcast <4 x i32> %link.sroa.0.0.i to <16 x i8>
  %1701 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 32
  %1702 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 36
  %_22.i269.i = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 16
  %1703 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 40
  %1704 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 44
  %1705 = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 336
  %1706 = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 352
  %1707 = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 320
  %1708 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 52
  %1709 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 48
  %1710 = bitcast <4 x i32> %spec.store.select25.i to <16 x i8>
  %1711 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 32
  %1712 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 36
  %_22.i.i = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 16
  %1713 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 40
  %1714 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 44
  %1715 = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 336
  %1716 = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 352
  %1717 = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 320
  %1718 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 52
  %1719 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 48
  %hot_left.i.promoted = load <4 x i32>, ptr %hot_left.i, align 16
  %history.i46.i.sroa.7.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i46.i.sroa.7.0.hot_left.i.sroa_idx, align 16
  %history.i46.i.sroa.10.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i46.i.sroa.10.0.hot_left.i.sroa_idx, align 16
  %history.i46.i.sroa.13.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i46.i.sroa.13.0.hot_left.i.sroa_idx, align 16
  %history.i46.i.sroa.16.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i46.i.sroa.16.0.hot_left.i.sroa_idx, align 16
  %history.i46.i.sroa.19.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i46.i.sroa.19.0.hot_left.i.sroa_idx, align 16
  %history.i46.i.sroa.22.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i46.i.sroa.22.0.hot_left.i.sroa_idx, align 16
  %history.i46.i.sroa.26.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i46.i.sroa.26.0.hot_left.i.sroa_idx, align 16
  %history.i46.i.sroa.29.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i46.i.sroa.29.0.hot_left.i.sroa_idx, align 16
  %history.i46.i.sroa.32.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i46.i.sroa.32.0.hot_left.i.sroa_idx, align 16
  %history.i46.i.sroa.35.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i46.i.sroa.35.0.hot_left.i.sroa_idx, align 16
  %history.i46.i.sroa.38.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i46.i.sroa.38.0.hot_left.i.sroa_idx, align 16
  %hot_right.i.promoted = load <4 x i32>, ptr %hot_right.i, align 16
  %history.i.i.sroa.7.0.hot_right.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16
  %history.i.i.sroa.10.0.hot_right.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16
  %history.i.i.sroa.13.0.hot_right.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16
  %history.i.i.sroa.16.0.hot_right.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16
  %history.i.i.sroa.19.0.hot_right.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16
  %history.i.i.sroa.22.0.hot_right.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16
  %history.i.i.sroa.26.0.hot_right.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16
  %history.i.i.sroa.29.0.hot_right.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16
  %history.i.i.sroa.32.0.hot_right.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16
  %history.i.i.sroa.35.0.hot_right.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16
  %history.i.i.sroa.38.0.hot_right.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16
  %_22.i269.i.promoted = load i32, ptr %_22.i269.i, align 4
  %.promoted23593 = load <4 x float>, ptr %1705, align 16
  %_22.i.i.promoted = load i32, ptr %_22.i.i, align 4
  %.promoted23632 = load <4 x float>, ptr %1715, align 16
  br label %bb44.i, !dbg !11521

bb15.i.loopexit:                                  ; preds = %bb74.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i
  %.lcssa1895119071.lcssa23633 = phi <4 x float> [ %.lcssa1895119071.lcssa23634, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i ], [ %.lcssa1895119071, %bb74.i ]
  %storemerge.i1822.lcssa1892119035.lcssa23613 = phi i32 [ %storemerge.i1822.lcssa1892119035.lcssa23614, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i ], [ %storemerge.i1822.lcssa1892119035, %bb74.i ]
  %.lcssa1889418999.lcssa23594 = phi <4 x float> [ %.lcssa1889418999.lcssa23595, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i ], [ %.lcssa1889418999, %bb74.i ]
  %storemerge.i1856.lcssa1886418963.lcssa23574 = phi i32 [ %storemerge.i1856.lcssa1886418963.lcssa23575, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i ], [ %storemerge.i1856.lcssa1886418963, %bb74.i ]
  %ring_cursor.sroa.0.1.i.lcssa = phi i32 [ %ring_cursor.sroa.0.0.i19107, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i ], [ %ring_cursor.sroa.0.2.i, %bb74.i ], !dbg !11540
  %main_cursor.sroa.0.1.i.lcssa = phi i32 [ %main_cursor.sroa.0.0.i19108, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i ], [ %main_cursor.sroa.0.2.i, %bb74.i ], !dbg !11541
  %_166.not.i = icmp eq i32 %1721, 0, !dbg !11521
  %indvars.iv.next20981 = add i32 %indvars.iv20980, -32, !dbg !11521
  br i1 %_166.not.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit, label %bb44.i, !dbg !11521

bb44.i:                                           ; preds = %bb44.i.lr.ph, %bb15.i.loopexit
  %.lcssa1895119071.lcssa23634 = phi <4 x float> [ %.promoted23632, %bb44.i.lr.ph ], [ %.lcssa1895119071.lcssa23633, %bb15.i.loopexit ]
  %storemerge.i1822.lcssa1892119035.lcssa23614 = phi i32 [ %_22.i.i.promoted, %bb44.i.lr.ph ], [ %storemerge.i1822.lcssa1892119035.lcssa23613, %bb15.i.loopexit ]
  %.lcssa1889418999.lcssa23595 = phi <4 x float> [ %.promoted23593, %bb44.i.lr.ph ], [ %.lcssa1889418999.lcssa23594, %bb15.i.loopexit ]
  %storemerge.i1856.lcssa1886418963.lcssa23575 = phi i32 [ %_22.i269.i.promoted, %bb44.i.lr.ph ], [ %storemerge.i1856.lcssa1886418963.lcssa23574, %bb15.i.loopexit ]
  %history.i.i.sroa.38.0.lcssa23556 = phi <4 x i32> [ %history.i.i.sroa.38.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.38.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.35.0.lcssa23538 = phi <4 x i32> [ %history.i.i.sroa.35.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.35.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.32.0.lcssa23520 = phi <4 x i32> [ %history.i.i.sroa.32.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.32.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.29.0.lcssa23502 = phi <4 x i32> [ %history.i.i.sroa.29.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.29.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.26.0.lcssa23484 = phi <4 x i32> [ %history.i.i.sroa.26.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.26.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.22.0.lcssa23466 = phi <4 x i32> [ %history.i.i.sroa.22.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.22.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.19.0.lcssa23448 = phi <4 x i32> [ %history.i.i.sroa.19.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.19.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.16.0.lcssa23430 = phi <4 x i32> [ %history.i.i.sroa.16.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.16.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.13.0.lcssa23412 = phi <4 x i32> [ %history.i.i.sroa.13.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.13.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.10.0.lcssa23394 = phi <4 x i32> [ %history.i.i.sroa.10.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.10.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.7.0.lcssa23376 = phi <4 x i32> [ %history.i.i.sroa.7.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.7.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.0.0.lcssa23358 = phi <4 x i32> [ %hot_right.i.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i46.i.sroa.38.0.lcssa23340 = phi <4 x i32> [ %history.i46.i.sroa.38.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i46.i.sroa.38.0.lcssa, %bb15.i.loopexit ]
  %history.i46.i.sroa.35.0.lcssa23322 = phi <4 x i32> [ %history.i46.i.sroa.35.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i46.i.sroa.35.0.lcssa, %bb15.i.loopexit ]
  %history.i46.i.sroa.32.0.lcssa23304 = phi <4 x i32> [ %history.i46.i.sroa.32.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i46.i.sroa.32.0.lcssa, %bb15.i.loopexit ]
  %history.i46.i.sroa.29.0.lcssa23286 = phi <4 x i32> [ %history.i46.i.sroa.29.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i46.i.sroa.29.0.lcssa, %bb15.i.loopexit ]
  %history.i46.i.sroa.26.0.lcssa23268 = phi <4 x i32> [ %history.i46.i.sroa.26.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i46.i.sroa.26.0.lcssa, %bb15.i.loopexit ]
  %history.i46.i.sroa.22.0.lcssa23250 = phi <4 x i32> [ %history.i46.i.sroa.22.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i46.i.sroa.22.0.lcssa, %bb15.i.loopexit ]
  %history.i46.i.sroa.19.0.lcssa23232 = phi <4 x i32> [ %history.i46.i.sroa.19.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i46.i.sroa.19.0.lcssa, %bb15.i.loopexit ]
  %history.i46.i.sroa.16.0.lcssa23214 = phi <4 x i32> [ %history.i46.i.sroa.16.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i46.i.sroa.16.0.lcssa, %bb15.i.loopexit ]
  %history.i46.i.sroa.13.0.lcssa23196 = phi <4 x i32> [ %history.i46.i.sroa.13.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i46.i.sroa.13.0.lcssa, %bb15.i.loopexit ]
  %history.i46.i.sroa.10.0.lcssa23178 = phi <4 x i32> [ %history.i46.i.sroa.10.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i46.i.sroa.10.0.lcssa, %bb15.i.loopexit ]
  %history.i46.i.sroa.7.0.lcssa23160 = phi <4 x i32> [ %history.i46.i.sroa.7.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i46.i.sroa.7.0.lcssa, %bb15.i.loopexit ]
  %history.i46.i.sroa.0.0.lcssa23142 = phi <4 x i32> [ %hot_left.i.promoted, %bb44.i.lr.ph ], [ %history.i46.i.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %indvars.iv20980 = phi i32 [ %frames, %bb44.i.lr.ph ], [ %indvars.iv.next20981, %bb15.i.loopexit ]
  %iter2.sroa.0.0.i19110 = phi i32 [ %yield_count.sroa.0.0.i7419, %bb44.i.lr.ph ], [ %1721, %bb15.i.loopexit ]
  %iter1.sroa.0.0.i19109 = phi i32 [ 0, %bb44.i.lr.ph ], [ %1720, %bb15.i.loopexit ]
  %main_cursor.sroa.0.0.i19108 = phi i32 [ %_36.i, %bb44.i.lr.ph ], [ %main_cursor.sroa.0.1.i.lcssa, %bb15.i.loopexit ]
  %ring_cursor.sroa.0.0.i19107 = phi i32 [ %_37.i, %bb44.i.lr.ph ], [ %ring_cursor.sroa.0.1.i.lcssa, %bb15.i.loopexit ]
  %umin21004 = call i32 @llvm.umin.i32(i32 %indvars.iv20980, i32 32), !dbg !11542
  %umax20986 = call i32 @llvm.umax.i32(i32 %umin21004, i32 1), !dbg !11542
  %1720 = add i32 %iter1.sroa.0.0.i19109, 32, !dbg !11542
  %1721 = add nsw i32 %iter2.sroa.0.0.i19110, -1, !dbg !11546
  %1722 = sub i32 %frames, %iter1.sroa.0.0.i19109, !dbg !11547
  %spec.store.select.i = tail call i32 @llvm.umin.i32(i32 %1722, i32 32), !dbg !11549
  %active_base.i = shl i32 %iter1.sroa.0.0.i19109, 2, !dbg !11554
  %active_base.i16405 = add i32 %spec.store.select.i, %iter1.sroa.0.0.i19109, !dbg !11556
  %_52.i = shl i32 %active_base.i16405, 2, !dbg !11556
  %_176.i = icmp ult i32 %_52.i, %active_base.i, !dbg !11559
  %_170.not.i = icmp ugt i32 %_52.i, %left_io.1
  %or.cond.i = or i1 %_176.i, %_170.not.i, !dbg !11559
  br i1 %or.cond.i, label %bb50.i, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7439, !dbg !11559, !prof !4596

bb50.i:                                           ; preds = %bb44.i
  store <4 x i32> %history.i46.i.sroa.0.0.lcssa23142, ptr %hot_left.i, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.7.0.lcssa23160, ptr %history.i46.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.10.0.lcssa23178, ptr %history.i46.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.13.0.lcssa23196, ptr %history.i46.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.16.0.lcssa23214, ptr %history.i46.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.19.0.lcssa23232, ptr %history.i46.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.22.0.lcssa23250, ptr %history.i46.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.26.0.lcssa23268, ptr %history.i46.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.29.0.lcssa23286, ptr %history.i46.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.32.0.lcssa23304, ptr %history.i46.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.35.0.lcssa23322, ptr %history.i46.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.38.0.lcssa23340, ptr %history.i46.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i.i.sroa.0.0.lcssa23358, ptr %hot_right.i, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.7.0.lcssa23376, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.10.0.lcssa23394, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.13.0.lcssa23412, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.16.0.lcssa23430, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.19.0.lcssa23448, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.22.0.lcssa23466, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.26.0.lcssa23484, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.29.0.lcssa23502, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.32.0.lcssa23520, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.35.0.lcssa23538, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.38.0.lcssa23556, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store i32 %storemerge.i1856.lcssa1886418963.lcssa23575, ptr %_22.i269.i, align 4
  store <4 x float> %.lcssa1889418999.lcssa23595, ptr %1705, align 16
  store i32 %storemerge.i1822.lcssa1892119035.lcssa23614, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1895119071.lcssa23634, ptr %1715, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %active_base.i, i32 noundef %_52.i, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_b74a748963eaf51a410c7eb21835ee21) #32, !dbg !11570, !noalias !11483
  unreachable, !dbg !11570

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7439: ; preds = %bb44.i
  %_179.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %active_base.i, !dbg !11571
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11575), !dbg !11578
  %_2.i744218781.not = icmp eq i32 %frames, %iter1.sroa.0.0.i19109, !dbg !11579
  br i1 %_2.i744218781.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit243.i, label %bb5.i49.i.lr.ph, !dbg !11579

bb5.i49.i.lr.ph:                                  ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7439
  %_11.i.i.i67.i16492 = load <4 x float>, ptr %_31, align 16, !alias.scope !11582, !noalias !11587
  %_14.i.i.i70.i16493 = load <4 x float>, ptr %1662, align 16, !alias.scope !11582, !noalias !11587
  %_17.i.i.i73.i16494 = load <4 x float>, ptr %1663, align 16, !alias.scope !11582, !noalias !11587
  %_20.i.i.i76.i16495 = load <4 x float>, ptr %1664, align 16, !alias.scope !11582, !noalias !11587
  %_25.i.i.i81.i16496 = load <4 x float>, ptr %row1.i.i.i79.i, align 16, !alias.scope !11582, !noalias !11587
  %_28.i.i.i84.i16497 = load <4 x float>, ptr %1665, align 16, !alias.scope !11582, !noalias !11587
  %_31.i.i.i87.i16498 = load <4 x float>, ptr %1666, align 16, !alias.scope !11582, !noalias !11587
  %_34.i.i.i90.i16499 = load <4 x float>, ptr %1667, align 16, !alias.scope !11582, !noalias !11587
  %_39.i.i.i95.i16500 = load <4 x float>, ptr %row3.i.i.i93.i, align 16, !alias.scope !11582, !noalias !11587
  %_42.i.i.i98.i16501 = load <4 x float>, ptr %1668, align 16, !alias.scope !11582, !noalias !11587
  %_45.i.i.i101.i16502 = load <4 x float>, ptr %1669, align 16, !alias.scope !11582, !noalias !11587
  %_48.i.i.i104.i16503 = load <4 x float>, ptr %1670, align 16, !alias.scope !11582, !noalias !11587
  %_53.i.i.i109.i16504 = load <4 x float>, ptr %row5.i.i.i107.i, align 16, !alias.scope !11582, !noalias !11587
  %_56.i.i.i112.i16505 = load <4 x float>, ptr %1671, align 16, !alias.scope !11582, !noalias !11587
  %_59.i.i.i115.i16506 = load <4 x float>, ptr %1672, align 16, !alias.scope !11582, !noalias !11587
  %_62.i.i.i118.i16507 = load <4 x float>, ptr %1673, align 16, !alias.scope !11582, !noalias !11587
  %_67.i.i.i123.i16508 = load <4 x float>, ptr %row7.i.i.i121.i, align 16, !alias.scope !11582, !noalias !11587
  %_70.i.i.i126.i16509 = load <4 x float>, ptr %1674, align 16, !alias.scope !11582, !noalias !11587
  %_73.i.i.i129.i16510 = load <4 x float>, ptr %1675, align 16, !alias.scope !11582, !noalias !11587
  %_76.i.i.i132.i16511 = load <4 x float>, ptr %1676, align 16, !alias.scope !11582, !noalias !11587
  %_81.i.i.i137.i16512 = load <4 x float>, ptr %row9.i.i.i135.i, align 16, !alias.scope !11582, !noalias !11587
  %_84.i.i.i140.i16513 = load <4 x float>, ptr %1677, align 16, !alias.scope !11582, !noalias !11587
  %_87.i.i.i143.i16514 = load <4 x float>, ptr %1678, align 16, !alias.scope !11582, !noalias !11587
  %_90.i.i.i146.i16515 = load <4 x float>, ptr %1679, align 16, !alias.scope !11582, !noalias !11587
  %_95.i.i.i151.i16516 = load <4 x float>, ptr %row11.i.i.i149.i, align 16, !alias.scope !11582, !noalias !11587
  %_98.i.i.i154.i16517 = load <4 x float>, ptr %1680, align 16, !alias.scope !11582, !noalias !11587
  %_101.i.i.i157.i16518 = load <4 x float>, ptr %1681, align 16, !alias.scope !11582, !noalias !11587
  %_104.i.i.i160.i16519 = load <4 x float>, ptr %1682, align 16, !alias.scope !11582, !noalias !11587
  %_109.i.i.i165.i16520 = load <4 x float>, ptr %row13.i.i.i163.i, align 16, !alias.scope !11582, !noalias !11587
  %_112.i.i.i168.i16521 = load <4 x float>, ptr %1683, align 16, !alias.scope !11582, !noalias !11587
  %_115.i.i.i171.i16522 = load <4 x float>, ptr %1684, align 16, !alias.scope !11582, !noalias !11587
  %_118.i.i.i174.i16523 = load <4 x float>, ptr %1685, align 16, !alias.scope !11582, !noalias !11587
  %_123.i.i.i179.i16524 = load <4 x float>, ptr %row15.i.i.i177.i, align 16, !alias.scope !11582, !noalias !11587
  %_126.i.i.i182.i16525 = load <4 x float>, ptr %1686, align 16, !alias.scope !11582, !noalias !11587
  %_129.i.i.i185.i16526 = load <4 x float>, ptr %1687, align 16, !alias.scope !11582, !noalias !11587
  %_132.i.i.i188.i16527 = load <4 x float>, ptr %1688, align 16, !alias.scope !11582, !noalias !11587
  %_137.i.i.i193.i16528 = load <4 x float>, ptr %row17.i.i.i191.i, align 16, !alias.scope !11582, !noalias !11587
  %_140.i.i.i196.i16529 = load <4 x float>, ptr %1689, align 16, !alias.scope !11582, !noalias !11587
  %_143.i.i.i199.i16530 = load <4 x float>, ptr %1690, align 16, !alias.scope !11582, !noalias !11587
  %_146.i.i.i202.i16531 = load <4 x float>, ptr %1691, align 16, !alias.scope !11582, !noalias !11587
  %_151.i.i.i207.i16532 = load <4 x float>, ptr %row19.i.i.i205.i, align 16, !alias.scope !11582, !noalias !11587
  %_154.i.i.i210.i16533 = load <4 x float>, ptr %1692, align 16, !alias.scope !11582, !noalias !11587
  %_157.i.i.i213.i16534 = load <4 x float>, ptr %1693, align 16, !alias.scope !11582, !noalias !11587
  %_160.i.i.i216.i16535 = load <4 x float>, ptr %1694, align 16, !alias.scope !11582, !noalias !11587
  %_165.i.i.i221.i16536 = load <4 x float>, ptr %row21.i.i.i219.i, align 16, !alias.scope !11582, !noalias !11587
  %_168.i.i.i224.i16537 = load <4 x float>, ptr %1695, align 16, !alias.scope !11582, !noalias !11587
  %_171.i.i.i227.i16538 = load <4 x float>, ptr %1696, align 16, !alias.scope !11582, !noalias !11587
  %_174.i.i.i230.i16539 = load <4 x float>, ptr %1697, align 16, !alias.scope !11582, !noalias !11587
  br label %bb5.i49.i, !dbg !11579

bb5.i49.i:                                        ; preds = %bb5.i49.i.lr.ph, %bb5.i49.i
  %history.i46.i.sroa.0.018793 = phi <4 x i32> [ %history.i46.i.sroa.0.0.lcssa23142, %bb5.i49.i.lr.ph ], [ %lanes.i5943.sroa.0.0.copyload, %bb5.i49.i ]
  %history.i46.i.sroa.7.018792 = phi <4 x i32> [ %history.i46.i.sroa.7.0.lcssa23160, %bb5.i49.i.lr.ph ], [ %history.i46.i.sroa.0.018793, %bb5.i49.i ]
  %history.i46.i.sroa.10.018791 = phi <4 x i32> [ %history.i46.i.sroa.10.0.lcssa23178, %bb5.i49.i.lr.ph ], [ %history.i46.i.sroa.7.018792, %bb5.i49.i ]
  %history.i46.i.sroa.13.018790 = phi <4 x i32> [ %history.i46.i.sroa.13.0.lcssa23196, %bb5.i49.i.lr.ph ], [ %history.i46.i.sroa.10.018791, %bb5.i49.i ]
  %history.i46.i.sroa.16.018789 = phi <4 x i32> [ %history.i46.i.sroa.16.0.lcssa23214, %bb5.i49.i.lr.ph ], [ %history.i46.i.sroa.13.018790, %bb5.i49.i ]
  %history.i46.i.sroa.19.018788 = phi <4 x i32> [ %history.i46.i.sroa.19.0.lcssa23232, %bb5.i49.i.lr.ph ], [ %history.i46.i.sroa.16.018789, %bb5.i49.i ]
  %history.i46.i.sroa.22.018787 = phi <4 x i32> [ %history.i46.i.sroa.22.0.lcssa23250, %bb5.i49.i.lr.ph ], [ %history.i46.i.sroa.19.018788, %bb5.i49.i ]
  %history.i46.i.sroa.26.018786 = phi <4 x i32> [ %history.i46.i.sroa.26.0.lcssa23268, %bb5.i49.i.lr.ph ], [ %history.i46.i.sroa.22.018787, %bb5.i49.i ]
  %history.i46.i.sroa.29.018785 = phi <4 x i32> [ %history.i46.i.sroa.29.0.lcssa23286, %bb5.i49.i.lr.ph ], [ %history.i46.i.sroa.26.018786, %bb5.i49.i ]
  %history.i46.i.sroa.32.018784 = phi <4 x i32> [ %history.i46.i.sroa.32.0.lcssa23304, %bb5.i49.i.lr.ph ], [ %history.i46.i.sroa.29.018785, %bb5.i49.i ]
  %history.i46.i.sroa.35.018783 = phi <4 x i32> [ %history.i46.i.sroa.35.0.lcssa23322, %bb5.i49.i.lr.ph ], [ %history.i46.i.sroa.32.018784, %bb5.i49.i ]
  %iter.i42.i.sroa.16.018782 = phi i32 [ 0, %bb5.i49.i.lr.ph ], [ %1844, %bb5.i49.i ]
  %start1.i.i7448 = shl i32 %iter.i42.i.sroa.16.018782, 2, !dbg !11596
  %data.i.i7449 = getelementptr inbounds nuw float, ptr %_179.i, i32 %start1.i.i7448, !dbg !11598
  %lanes.i5943.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i7449, align 4, !dbg !11600, !alias.scope !11605, !noalias !11609
  %1723 = bitcast <4 x i32> %history.i46.i.sroa.19.018788 to <4 x float>, !dbg !11613
  %1724 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1723), !dbg !11618
  %1725 = bitcast <4 x i32> %lanes.i5943.sroa.0.0.copyload to <4 x float>, !dbg !11619
  %1726 = fmul <4 x float> %_11.i.i.i67.i16492, %1725, !dbg !11624
  %1727 = fadd <4 x float> %1726, zeroinitializer, !dbg !11625
  %1728 = fmul <4 x float> %_14.i.i.i70.i16493, %1725, !dbg !11629
  %1729 = fadd <4 x float> %1728, zeroinitializer, !dbg !11633
  %1730 = fmul <4 x float> %_17.i.i.i73.i16494, %1725, !dbg !11637
  %1731 = fadd <4 x float> %1730, zeroinitializer, !dbg !11641
  %1732 = fmul <4 x float> %_20.i.i.i76.i16495, %1725, !dbg !11645
  %1733 = fadd <4 x float> %1732, zeroinitializer, !dbg !11649
  %1734 = bitcast <4 x i32> %history.i46.i.sroa.0.018793 to <4 x float>, !dbg !11653
  %1735 = fmul <4 x float> %_25.i.i.i81.i16496, %1734, !dbg !11657
  %1736 = fadd <4 x float> %1727, %1735, !dbg !11658
  %1737 = fmul <4 x float> %_28.i.i.i84.i16497, %1734, !dbg !11662
  %1738 = fadd <4 x float> %1729, %1737, !dbg !11666
  %1739 = fmul <4 x float> %_31.i.i.i87.i16498, %1734, !dbg !11670
  %1740 = fadd <4 x float> %1731, %1739, !dbg !11674
  %1741 = fmul <4 x float> %_34.i.i.i90.i16499, %1734, !dbg !11678
  %1742 = fadd <4 x float> %1733, %1741, !dbg !11682
  %1743 = bitcast <4 x i32> %history.i46.i.sroa.7.018792 to <4 x float>, !dbg !11686
  %1744 = fmul <4 x float> %_39.i.i.i95.i16500, %1743, !dbg !11690
  %1745 = fadd <4 x float> %1736, %1744, !dbg !11691
  %1746 = fmul <4 x float> %_42.i.i.i98.i16501, %1743, !dbg !11695
  %1747 = fadd <4 x float> %1738, %1746, !dbg !11699
  %1748 = fmul <4 x float> %_45.i.i.i101.i16502, %1743, !dbg !11703
  %1749 = fadd <4 x float> %1740, %1748, !dbg !11707
  %1750 = fmul <4 x float> %_48.i.i.i104.i16503, %1743, !dbg !11711
  %1751 = fadd <4 x float> %1742, %1750, !dbg !11715
  %1752 = bitcast <4 x i32> %history.i46.i.sroa.10.018791 to <4 x float>, !dbg !11719
  %1753 = fmul <4 x float> %_53.i.i.i109.i16504, %1752, !dbg !11723
  %1754 = fadd <4 x float> %1745, %1753, !dbg !11724
  %1755 = fmul <4 x float> %_56.i.i.i112.i16505, %1752, !dbg !11728
  %1756 = fadd <4 x float> %1747, %1755, !dbg !11732
  %1757 = fmul <4 x float> %_59.i.i.i115.i16506, %1752, !dbg !11736
  %1758 = fadd <4 x float> %1749, %1757, !dbg !11740
  %1759 = fmul <4 x float> %_62.i.i.i118.i16507, %1752, !dbg !11744
  %1760 = fadd <4 x float> %1751, %1759, !dbg !11748
  %1761 = bitcast <4 x i32> %history.i46.i.sroa.13.018790 to <4 x float>, !dbg !11752
  %1762 = fmul <4 x float> %_67.i.i.i123.i16508, %1761, !dbg !11756
  %1763 = fadd <4 x float> %1754, %1762, !dbg !11757
  %1764 = fmul <4 x float> %_70.i.i.i126.i16509, %1761, !dbg !11761
  %1765 = fadd <4 x float> %1756, %1764, !dbg !11765
  %1766 = fmul <4 x float> %_73.i.i.i129.i16510, %1761, !dbg !11769
  %1767 = fadd <4 x float> %1758, %1766, !dbg !11773
  %1768 = fmul <4 x float> %_76.i.i.i132.i16511, %1761, !dbg !11777
  %1769 = fadd <4 x float> %1760, %1768, !dbg !11781
  %1770 = bitcast <4 x i32> %history.i46.i.sroa.16.018789 to <4 x float>, !dbg !11785
  %1771 = fmul <4 x float> %_81.i.i.i137.i16512, %1770, !dbg !11789
  %1772 = fadd <4 x float> %1763, %1771, !dbg !11790
  %1773 = fmul <4 x float> %_84.i.i.i140.i16513, %1770, !dbg !11794
  %1774 = fadd <4 x float> %1765, %1773, !dbg !11798
  %1775 = fmul <4 x float> %_87.i.i.i143.i16514, %1770, !dbg !11802
  %1776 = fadd <4 x float> %1767, %1775, !dbg !11806
  %1777 = fmul <4 x float> %_90.i.i.i146.i16515, %1770, !dbg !11810
  %1778 = fadd <4 x float> %1769, %1777, !dbg !11814
  %1779 = fmul <4 x float> %_95.i.i.i151.i16516, %1723, !dbg !11818
  %1780 = fadd <4 x float> %1772, %1779, !dbg !11822
  %1781 = fmul <4 x float> %_98.i.i.i154.i16517, %1723, !dbg !11826
  %1782 = fadd <4 x float> %1774, %1781, !dbg !11830
  %1783 = fmul <4 x float> %_101.i.i.i157.i16518, %1723, !dbg !11834
  %1784 = fadd <4 x float> %1776, %1783, !dbg !11838
  %1785 = fmul <4 x float> %_104.i.i.i160.i16519, %1723, !dbg !11842
  %1786 = fadd <4 x float> %1778, %1785, !dbg !11846
  %1787 = bitcast <4 x i32> %history.i46.i.sroa.22.018787 to <4 x float>, !dbg !11850
  %1788 = fmul <4 x float> %_109.i.i.i165.i16520, %1787, !dbg !11854
  %1789 = fadd <4 x float> %1780, %1788, !dbg !11855
  %1790 = fmul <4 x float> %_112.i.i.i168.i16521, %1787, !dbg !11859
  %1791 = fadd <4 x float> %1782, %1790, !dbg !11863
  %1792 = fmul <4 x float> %_115.i.i.i171.i16522, %1787, !dbg !11867
  %1793 = fadd <4 x float> %1784, %1792, !dbg !11871
  %1794 = fmul <4 x float> %_118.i.i.i174.i16523, %1787, !dbg !11875
  %1795 = fadd <4 x float> %1786, %1794, !dbg !11879
  %1796 = bitcast <4 x i32> %history.i46.i.sroa.26.018786 to <4 x float>, !dbg !11883
  %1797 = fmul <4 x float> %_123.i.i.i179.i16524, %1796, !dbg !11887
  %1798 = fadd <4 x float> %1789, %1797, !dbg !11888
  %1799 = fmul <4 x float> %_126.i.i.i182.i16525, %1796, !dbg !11892
  %1800 = fadd <4 x float> %1791, %1799, !dbg !11896
  %1801 = fmul <4 x float> %_129.i.i.i185.i16526, %1796, !dbg !11900
  %1802 = fadd <4 x float> %1793, %1801, !dbg !11904
  %1803 = fmul <4 x float> %_132.i.i.i188.i16527, %1796, !dbg !11908
  %1804 = fadd <4 x float> %1795, %1803, !dbg !11912
  %1805 = bitcast <4 x i32> %history.i46.i.sroa.29.018785 to <4 x float>, !dbg !11916
  %1806 = fmul <4 x float> %_137.i.i.i193.i16528, %1805, !dbg !11920
  %1807 = fadd <4 x float> %1798, %1806, !dbg !11921
  %1808 = fmul <4 x float> %_140.i.i.i196.i16529, %1805, !dbg !11925
  %1809 = fadd <4 x float> %1800, %1808, !dbg !11929
  %1810 = fmul <4 x float> %_143.i.i.i199.i16530, %1805, !dbg !11933
  %1811 = fadd <4 x float> %1802, %1810, !dbg !11937
  %1812 = fmul <4 x float> %_146.i.i.i202.i16531, %1805, !dbg !11941
  %1813 = fadd <4 x float> %1804, %1812, !dbg !11945
  %1814 = bitcast <4 x i32> %history.i46.i.sroa.32.018784 to <4 x float>, !dbg !11949
  %1815 = fmul <4 x float> %_151.i.i.i207.i16532, %1814, !dbg !11953
  %1816 = fadd <4 x float> %1807, %1815, !dbg !11954
  %1817 = fmul <4 x float> %_154.i.i.i210.i16533, %1814, !dbg !11958
  %1818 = fadd <4 x float> %1809, %1817, !dbg !11962
  %1819 = fmul <4 x float> %_157.i.i.i213.i16534, %1814, !dbg !11966
  %1820 = fadd <4 x float> %1811, %1819, !dbg !11970
  %1821 = fmul <4 x float> %_160.i.i.i216.i16535, %1814, !dbg !11974
  %1822 = fadd <4 x float> %1813, %1821, !dbg !11978
  %1823 = bitcast <4 x i32> %history.i46.i.sroa.35.018783 to <4 x float>, !dbg !11982
  %1824 = fmul <4 x float> %_165.i.i.i221.i16536, %1823, !dbg !11986
  %1825 = fadd <4 x float> %1816, %1824, !dbg !11987
  %1826 = fmul <4 x float> %_168.i.i.i224.i16537, %1823, !dbg !11991
  %1827 = fadd <4 x float> %1818, %1826, !dbg !11995
  %1828 = fmul <4 x float> %_171.i.i.i227.i16538, %1823, !dbg !11999
  %1829 = fadd <4 x float> %1820, %1828, !dbg !12003
  %1830 = fmul <4 x float> %_174.i.i.i230.i16539, %1823, !dbg !12007
  %1831 = fadd <4 x float> %1822, %1830, !dbg !12011
  %1832 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1825), !dbg !12015
  %1833 = fcmp olt <4 x float> %1832, %1724, !dbg !12019
  %1834 = select <4 x i1> %1833, <4 x float> %1724, <4 x float> %1832, !dbg !12023
  %1835 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1827), !dbg !12015
  %1836 = fcmp olt <4 x float> %1835, %1834, !dbg !12019
  %1837 = select <4 x i1> %1836, <4 x float> %1834, <4 x float> %1835, !dbg !12023
  %1838 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1829), !dbg !12015
  %1839 = fcmp olt <4 x float> %1838, %1837, !dbg !12019
  %1840 = select <4 x i1> %1839, <4 x float> %1837, <4 x float> %1838, !dbg !12023
  %1841 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1831), !dbg !12015
  %1842 = fcmp olt <4 x float> %1841, %1840, !dbg !12019
  %1843 = select <4 x i1> %1842, <4 x float> %1840, <4 x float> %1841, !dbg !12023
  %1844 = add nuw nsw i32 %iter.i42.i.sroa.16.018782, 1, !dbg !12024
  %data.i4.i7453 = getelementptr inbounds nuw float, ptr %peaks_left.i, i32 %start1.i.i7448, !dbg !12025
  store <4 x float> %1843, ptr %data.i4.i7453, align 4, !dbg !12028, !alias.scope !12033, !noalias !12037
  %exitcond20984.not = icmp eq i32 %1844, %umax20986, !dbg !11579
  br i1 %exitcond20984.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit243.i, label %bb5.i49.i, !dbg !11579

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit243.i: ; preds = %bb5.i49.i, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7439
  %history.i46.i.sroa.38.0.lcssa = phi <4 x i32> [ %history.i46.i.sroa.38.0.lcssa23340, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7439 ], [ %history.i46.i.sroa.35.018783, %bb5.i49.i ], !dbg !11566
  %history.i46.i.sroa.35.0.lcssa = phi <4 x i32> [ %history.i46.i.sroa.35.0.lcssa23322, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7439 ], [ %history.i46.i.sroa.32.018784, %bb5.i49.i ], !dbg !11566
  %history.i46.i.sroa.32.0.lcssa = phi <4 x i32> [ %history.i46.i.sroa.32.0.lcssa23304, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7439 ], [ %history.i46.i.sroa.29.018785, %bb5.i49.i ], !dbg !11566
  %history.i46.i.sroa.29.0.lcssa = phi <4 x i32> [ %history.i46.i.sroa.29.0.lcssa23286, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7439 ], [ %history.i46.i.sroa.26.018786, %bb5.i49.i ], !dbg !11566
  %history.i46.i.sroa.26.0.lcssa = phi <4 x i32> [ %history.i46.i.sroa.26.0.lcssa23268, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7439 ], [ %history.i46.i.sroa.22.018787, %bb5.i49.i ], !dbg !11566
  %history.i46.i.sroa.22.0.lcssa = phi <4 x i32> [ %history.i46.i.sroa.22.0.lcssa23250, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7439 ], [ %history.i46.i.sroa.19.018788, %bb5.i49.i ], !dbg !11566
  %history.i46.i.sroa.19.0.lcssa = phi <4 x i32> [ %history.i46.i.sroa.19.0.lcssa23232, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7439 ], [ %history.i46.i.sroa.16.018789, %bb5.i49.i ], !dbg !11566
  %history.i46.i.sroa.16.0.lcssa = phi <4 x i32> [ %history.i46.i.sroa.16.0.lcssa23214, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7439 ], [ %history.i46.i.sroa.13.018790, %bb5.i49.i ], !dbg !11566
  %history.i46.i.sroa.13.0.lcssa = phi <4 x i32> [ %history.i46.i.sroa.13.0.lcssa23196, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7439 ], [ %history.i46.i.sroa.10.018791, %bb5.i49.i ], !dbg !11566
  %history.i46.i.sroa.10.0.lcssa = phi <4 x i32> [ %history.i46.i.sroa.10.0.lcssa23178, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7439 ], [ %history.i46.i.sroa.7.018792, %bb5.i49.i ], !dbg !11566
  %history.i46.i.sroa.7.0.lcssa = phi <4 x i32> [ %history.i46.i.sroa.7.0.lcssa23160, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7439 ], [ %history.i46.i.sroa.0.018793, %bb5.i49.i ], !dbg !11566
  %history.i46.i.sroa.0.0.lcssa = phi <4 x i32> [ %history.i46.i.sroa.0.0.lcssa23142, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7439 ], [ %lanes.i5943.sroa.0.0.copyload, %bb5.i49.i ], !dbg !11566
  %_187.not.i = icmp ugt i32 %_52.i, %right_io.1, !dbg !12041
  br i1 %_187.not.i, label %bb56.i, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7490, !dbg !12041, !prof !787

bb56.i:                                           ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit243.i
  store <4 x i32> %history.i46.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.7.0.lcssa, ptr %history.i46.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.10.0.lcssa, ptr %history.i46.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.13.0.lcssa, ptr %history.i46.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.16.0.lcssa, ptr %history.i46.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.19.0.lcssa, ptr %history.i46.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.22.0.lcssa, ptr %history.i46.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.26.0.lcssa, ptr %history.i46.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.29.0.lcssa, ptr %history.i46.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.32.0.lcssa, ptr %history.i46.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.35.0.lcssa, ptr %history.i46.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.38.0.lcssa, ptr %history.i46.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i.i.sroa.0.0.lcssa23358, ptr %hot_right.i, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.7.0.lcssa23376, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.10.0.lcssa23394, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.13.0.lcssa23412, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.16.0.lcssa23430, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.19.0.lcssa23448, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.22.0.lcssa23466, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.26.0.lcssa23484, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.29.0.lcssa23502, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.32.0.lcssa23520, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.35.0.lcssa23538, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.38.0.lcssa23556, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store i32 %storemerge.i1856.lcssa1886418963.lcssa23575, ptr %_22.i269.i, align 4
  store <4 x float> %.lcssa1889418999.lcssa23595, ptr %1705, align 16
  store i32 %storemerge.i1822.lcssa1892119035.lcssa23614, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1895119071.lcssa23634, ptr %1715, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %active_base.i, i32 noundef %_52.i, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_8715d54bb9ab90680506cd3587af9682) #32, !dbg !12045, !noalias !11483
  unreachable, !dbg !12045

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7490: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit243.i
  %_194.i = getelementptr inbounds nuw float, ptr %right_io.0, i32 %active_base.i, !dbg !12046
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12050), !dbg !12053
  br i1 %_2.i744218781.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i, label %bb5.i.i.lr.ph, !dbg !12054

bb5.i.i.lr.ph:                                    ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7490
  %_11.i.i.i.i16439 = load <4 x float>, ptr %_31, align 16, !alias.scope !12057, !noalias !12062
  %_14.i.i.i.i16440 = load <4 x float>, ptr %1662, align 16, !alias.scope !12057, !noalias !12062
  %_17.i.i.i.i16441 = load <4 x float>, ptr %1663, align 16, !alias.scope !12057, !noalias !12062
  %_20.i.i.i.i16442 = load <4 x float>, ptr %1664, align 16, !alias.scope !12057, !noalias !12062
  %_25.i.i.i.i16443 = load <4 x float>, ptr %row1.i.i.i79.i, align 16, !alias.scope !12057, !noalias !12062
  %_28.i.i.i.i16444 = load <4 x float>, ptr %1665, align 16, !alias.scope !12057, !noalias !12062
  %_31.i.i.i.i16445 = load <4 x float>, ptr %1666, align 16, !alias.scope !12057, !noalias !12062
  %_34.i.i.i.i16446 = load <4 x float>, ptr %1667, align 16, !alias.scope !12057, !noalias !12062
  %_39.i.i.i.i16447 = load <4 x float>, ptr %row3.i.i.i93.i, align 16, !alias.scope !12057, !noalias !12062
  %_42.i.i.i.i16448 = load <4 x float>, ptr %1668, align 16, !alias.scope !12057, !noalias !12062
  %_45.i.i.i.i16449 = load <4 x float>, ptr %1669, align 16, !alias.scope !12057, !noalias !12062
  %_48.i.i.i.i16450 = load <4 x float>, ptr %1670, align 16, !alias.scope !12057, !noalias !12062
  %_53.i.i.i.i16451 = load <4 x float>, ptr %row5.i.i.i107.i, align 16, !alias.scope !12057, !noalias !12062
  %_56.i.i.i.i16452 = load <4 x float>, ptr %1671, align 16, !alias.scope !12057, !noalias !12062
  %_59.i.i.i.i16453 = load <4 x float>, ptr %1672, align 16, !alias.scope !12057, !noalias !12062
  %_62.i.i.i.i16454 = load <4 x float>, ptr %1673, align 16, !alias.scope !12057, !noalias !12062
  %_67.i.i.i.i16455 = load <4 x float>, ptr %row7.i.i.i121.i, align 16, !alias.scope !12057, !noalias !12062
  %_70.i.i.i.i16456 = load <4 x float>, ptr %1674, align 16, !alias.scope !12057, !noalias !12062
  %_73.i.i.i.i16457 = load <4 x float>, ptr %1675, align 16, !alias.scope !12057, !noalias !12062
  %_76.i.i.i.i16458 = load <4 x float>, ptr %1676, align 16, !alias.scope !12057, !noalias !12062
  %_81.i.i.i.i16459 = load <4 x float>, ptr %row9.i.i.i135.i, align 16, !alias.scope !12057, !noalias !12062
  %_84.i.i.i.i16460 = load <4 x float>, ptr %1677, align 16, !alias.scope !12057, !noalias !12062
  %_87.i.i.i.i16461 = load <4 x float>, ptr %1678, align 16, !alias.scope !12057, !noalias !12062
  %_90.i.i.i.i16462 = load <4 x float>, ptr %1679, align 16, !alias.scope !12057, !noalias !12062
  %_95.i.i.i.i16463 = load <4 x float>, ptr %row11.i.i.i149.i, align 16, !alias.scope !12057, !noalias !12062
  %_98.i.i.i.i16464 = load <4 x float>, ptr %1680, align 16, !alias.scope !12057, !noalias !12062
  %_101.i.i.i.i16465 = load <4 x float>, ptr %1681, align 16, !alias.scope !12057, !noalias !12062
  %_104.i.i.i.i16466 = load <4 x float>, ptr %1682, align 16, !alias.scope !12057, !noalias !12062
  %_109.i.i.i.i16467 = load <4 x float>, ptr %row13.i.i.i163.i, align 16, !alias.scope !12057, !noalias !12062
  %_112.i.i.i.i16468 = load <4 x float>, ptr %1683, align 16, !alias.scope !12057, !noalias !12062
  %_115.i.i.i.i16469 = load <4 x float>, ptr %1684, align 16, !alias.scope !12057, !noalias !12062
  %_118.i.i.i.i16470 = load <4 x float>, ptr %1685, align 16, !alias.scope !12057, !noalias !12062
  %_123.i.i.i.i16471 = load <4 x float>, ptr %row15.i.i.i177.i, align 16, !alias.scope !12057, !noalias !12062
  %_126.i.i.i.i16472 = load <4 x float>, ptr %1686, align 16, !alias.scope !12057, !noalias !12062
  %_129.i.i.i.i16473 = load <4 x float>, ptr %1687, align 16, !alias.scope !12057, !noalias !12062
  %_132.i.i.i.i16474 = load <4 x float>, ptr %1688, align 16, !alias.scope !12057, !noalias !12062
  %_137.i.i.i.i16475 = load <4 x float>, ptr %row17.i.i.i191.i, align 16, !alias.scope !12057, !noalias !12062
  %_140.i.i.i.i16476 = load <4 x float>, ptr %1689, align 16, !alias.scope !12057, !noalias !12062
  %_143.i.i.i.i16477 = load <4 x float>, ptr %1690, align 16, !alias.scope !12057, !noalias !12062
  %_146.i.i.i.i16478 = load <4 x float>, ptr %1691, align 16, !alias.scope !12057, !noalias !12062
  %_151.i.i.i.i16479 = load <4 x float>, ptr %row19.i.i.i205.i, align 16, !alias.scope !12057, !noalias !12062
  %_154.i.i.i.i16480 = load <4 x float>, ptr %1692, align 16, !alias.scope !12057, !noalias !12062
  %_157.i.i.i.i16481 = load <4 x float>, ptr %1693, align 16, !alias.scope !12057, !noalias !12062
  %_160.i.i.i.i16482 = load <4 x float>, ptr %1694, align 16, !alias.scope !12057, !noalias !12062
  %_165.i.i.i.i16483 = load <4 x float>, ptr %row21.i.i.i219.i, align 16, !alias.scope !12057, !noalias !12062
  %_168.i.i.i.i16484 = load <4 x float>, ptr %1695, align 16, !alias.scope !12057, !noalias !12062
  %_171.i.i.i.i16485 = load <4 x float>, ptr %1696, align 16, !alias.scope !12057, !noalias !12062
  %_174.i.i.i.i16486 = load <4 x float>, ptr %1697, align 16, !alias.scope !12057, !noalias !12062
  br label %bb5.i.i, !dbg !12054

bb5.i.i:                                          ; preds = %bb5.i.i.lr.ph, %bb5.i.i
  %history.i.i.sroa.0.018820 = phi <4 x i32> [ %history.i.i.sroa.0.0.lcssa23358, %bb5.i.i.lr.ph ], [ %lanes.i5934.sroa.0.0.copyload, %bb5.i.i ]
  %history.i.i.sroa.7.018819 = phi <4 x i32> [ %history.i.i.sroa.7.0.lcssa23376, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.0.018820, %bb5.i.i ]
  %history.i.i.sroa.10.018818 = phi <4 x i32> [ %history.i.i.sroa.10.0.lcssa23394, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.7.018819, %bb5.i.i ]
  %history.i.i.sroa.13.018817 = phi <4 x i32> [ %history.i.i.sroa.13.0.lcssa23412, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.10.018818, %bb5.i.i ]
  %history.i.i.sroa.16.018816 = phi <4 x i32> [ %history.i.i.sroa.16.0.lcssa23430, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.13.018817, %bb5.i.i ]
  %history.i.i.sroa.19.018815 = phi <4 x i32> [ %history.i.i.sroa.19.0.lcssa23448, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.16.018816, %bb5.i.i ]
  %history.i.i.sroa.22.018814 = phi <4 x i32> [ %history.i.i.sroa.22.0.lcssa23466, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.19.018815, %bb5.i.i ]
  %history.i.i.sroa.26.018813 = phi <4 x i32> [ %history.i.i.sroa.26.0.lcssa23484, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.22.018814, %bb5.i.i ]
  %history.i.i.sroa.29.018812 = phi <4 x i32> [ %history.i.i.sroa.29.0.lcssa23502, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.26.018813, %bb5.i.i ]
  %history.i.i.sroa.32.018811 = phi <4 x i32> [ %history.i.i.sroa.32.0.lcssa23520, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.29.018812, %bb5.i.i ]
  %history.i.i.sroa.35.018810 = phi <4 x i32> [ %history.i.i.sroa.35.0.lcssa23538, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.32.018811, %bb5.i.i ]
  %iter.i.i.sroa.16.018809 = phi i32 [ 0, %bb5.i.i.lr.ph ], [ %1966, %bb5.i.i ]
  %start1.i.i7499 = shl i32 %iter.i.i.sroa.16.018809, 2, !dbg !12071
  %data.i.i7500 = getelementptr inbounds nuw float, ptr %_194.i, i32 %start1.i.i7499, !dbg !12073
  %lanes.i5934.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i7500, align 4, !dbg !12075, !alias.scope !12080, !noalias !12084
  %1845 = bitcast <4 x i32> %history.i.i.sroa.19.018815 to <4 x float>, !dbg !12088
  %1846 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1845), !dbg !12093
  %1847 = bitcast <4 x i32> %lanes.i5934.sroa.0.0.copyload to <4 x float>, !dbg !12094
  %1848 = fmul <4 x float> %_11.i.i.i.i16439, %1847, !dbg !12099
  %1849 = fadd <4 x float> %1848, zeroinitializer, !dbg !12100
  %1850 = fmul <4 x float> %_14.i.i.i.i16440, %1847, !dbg !12104
  %1851 = fadd <4 x float> %1850, zeroinitializer, !dbg !12108
  %1852 = fmul <4 x float> %_17.i.i.i.i16441, %1847, !dbg !12112
  %1853 = fadd <4 x float> %1852, zeroinitializer, !dbg !12116
  %1854 = fmul <4 x float> %_20.i.i.i.i16442, %1847, !dbg !12120
  %1855 = fadd <4 x float> %1854, zeroinitializer, !dbg !12124
  %1856 = bitcast <4 x i32> %history.i.i.sroa.0.018820 to <4 x float>, !dbg !12128
  %1857 = fmul <4 x float> %_25.i.i.i.i16443, %1856, !dbg !12132
  %1858 = fadd <4 x float> %1849, %1857, !dbg !12133
  %1859 = fmul <4 x float> %_28.i.i.i.i16444, %1856, !dbg !12137
  %1860 = fadd <4 x float> %1851, %1859, !dbg !12141
  %1861 = fmul <4 x float> %_31.i.i.i.i16445, %1856, !dbg !12145
  %1862 = fadd <4 x float> %1853, %1861, !dbg !12149
  %1863 = fmul <4 x float> %_34.i.i.i.i16446, %1856, !dbg !12153
  %1864 = fadd <4 x float> %1855, %1863, !dbg !12157
  %1865 = bitcast <4 x i32> %history.i.i.sroa.7.018819 to <4 x float>, !dbg !12161
  %1866 = fmul <4 x float> %_39.i.i.i.i16447, %1865, !dbg !12165
  %1867 = fadd <4 x float> %1858, %1866, !dbg !12166
  %1868 = fmul <4 x float> %_42.i.i.i.i16448, %1865, !dbg !12170
  %1869 = fadd <4 x float> %1860, %1868, !dbg !12174
  %1870 = fmul <4 x float> %_45.i.i.i.i16449, %1865, !dbg !12178
  %1871 = fadd <4 x float> %1862, %1870, !dbg !12182
  %1872 = fmul <4 x float> %_48.i.i.i.i16450, %1865, !dbg !12186
  %1873 = fadd <4 x float> %1864, %1872, !dbg !12190
  %1874 = bitcast <4 x i32> %history.i.i.sroa.10.018818 to <4 x float>, !dbg !12194
  %1875 = fmul <4 x float> %_53.i.i.i.i16451, %1874, !dbg !12198
  %1876 = fadd <4 x float> %1867, %1875, !dbg !12199
  %1877 = fmul <4 x float> %_56.i.i.i.i16452, %1874, !dbg !12203
  %1878 = fadd <4 x float> %1869, %1877, !dbg !12207
  %1879 = fmul <4 x float> %_59.i.i.i.i16453, %1874, !dbg !12211
  %1880 = fadd <4 x float> %1871, %1879, !dbg !12215
  %1881 = fmul <4 x float> %_62.i.i.i.i16454, %1874, !dbg !12219
  %1882 = fadd <4 x float> %1873, %1881, !dbg !12223
  %1883 = bitcast <4 x i32> %history.i.i.sroa.13.018817 to <4 x float>, !dbg !12227
  %1884 = fmul <4 x float> %_67.i.i.i.i16455, %1883, !dbg !12231
  %1885 = fadd <4 x float> %1876, %1884, !dbg !12232
  %1886 = fmul <4 x float> %_70.i.i.i.i16456, %1883, !dbg !12236
  %1887 = fadd <4 x float> %1878, %1886, !dbg !12240
  %1888 = fmul <4 x float> %_73.i.i.i.i16457, %1883, !dbg !12244
  %1889 = fadd <4 x float> %1880, %1888, !dbg !12248
  %1890 = fmul <4 x float> %_76.i.i.i.i16458, %1883, !dbg !12252
  %1891 = fadd <4 x float> %1882, %1890, !dbg !12256
  %1892 = bitcast <4 x i32> %history.i.i.sroa.16.018816 to <4 x float>, !dbg !12260
  %1893 = fmul <4 x float> %_81.i.i.i.i16459, %1892, !dbg !12264
  %1894 = fadd <4 x float> %1885, %1893, !dbg !12265
  %1895 = fmul <4 x float> %_84.i.i.i.i16460, %1892, !dbg !12269
  %1896 = fadd <4 x float> %1887, %1895, !dbg !12273
  %1897 = fmul <4 x float> %_87.i.i.i.i16461, %1892, !dbg !12277
  %1898 = fadd <4 x float> %1889, %1897, !dbg !12281
  %1899 = fmul <4 x float> %_90.i.i.i.i16462, %1892, !dbg !12285
  %1900 = fadd <4 x float> %1891, %1899, !dbg !12289
  %1901 = fmul <4 x float> %_95.i.i.i.i16463, %1845, !dbg !12293
  %1902 = fadd <4 x float> %1894, %1901, !dbg !12297
  %1903 = fmul <4 x float> %_98.i.i.i.i16464, %1845, !dbg !12301
  %1904 = fadd <4 x float> %1896, %1903, !dbg !12305
  %1905 = fmul <4 x float> %_101.i.i.i.i16465, %1845, !dbg !12309
  %1906 = fadd <4 x float> %1898, %1905, !dbg !12313
  %1907 = fmul <4 x float> %_104.i.i.i.i16466, %1845, !dbg !12317
  %1908 = fadd <4 x float> %1900, %1907, !dbg !12321
  %1909 = bitcast <4 x i32> %history.i.i.sroa.22.018814 to <4 x float>, !dbg !12325
  %1910 = fmul <4 x float> %_109.i.i.i.i16467, %1909, !dbg !12329
  %1911 = fadd <4 x float> %1902, %1910, !dbg !12330
  %1912 = fmul <4 x float> %_112.i.i.i.i16468, %1909, !dbg !12334
  %1913 = fadd <4 x float> %1904, %1912, !dbg !12338
  %1914 = fmul <4 x float> %_115.i.i.i.i16469, %1909, !dbg !12342
  %1915 = fadd <4 x float> %1906, %1914, !dbg !12346
  %1916 = fmul <4 x float> %_118.i.i.i.i16470, %1909, !dbg !12350
  %1917 = fadd <4 x float> %1908, %1916, !dbg !12354
  %1918 = bitcast <4 x i32> %history.i.i.sroa.26.018813 to <4 x float>, !dbg !12358
  %1919 = fmul <4 x float> %_123.i.i.i.i16471, %1918, !dbg !12362
  %1920 = fadd <4 x float> %1911, %1919, !dbg !12363
  %1921 = fmul <4 x float> %_126.i.i.i.i16472, %1918, !dbg !12367
  %1922 = fadd <4 x float> %1913, %1921, !dbg !12371
  %1923 = fmul <4 x float> %_129.i.i.i.i16473, %1918, !dbg !12375
  %1924 = fadd <4 x float> %1915, %1923, !dbg !12379
  %1925 = fmul <4 x float> %_132.i.i.i.i16474, %1918, !dbg !12383
  %1926 = fadd <4 x float> %1917, %1925, !dbg !12387
  %1927 = bitcast <4 x i32> %history.i.i.sroa.29.018812 to <4 x float>, !dbg !12391
  %1928 = fmul <4 x float> %_137.i.i.i.i16475, %1927, !dbg !12395
  %1929 = fadd <4 x float> %1920, %1928, !dbg !12396
  %1930 = fmul <4 x float> %_140.i.i.i.i16476, %1927, !dbg !12400
  %1931 = fadd <4 x float> %1922, %1930, !dbg !12404
  %1932 = fmul <4 x float> %_143.i.i.i.i16477, %1927, !dbg !12408
  %1933 = fadd <4 x float> %1924, %1932, !dbg !12412
  %1934 = fmul <4 x float> %_146.i.i.i.i16478, %1927, !dbg !12416
  %1935 = fadd <4 x float> %1926, %1934, !dbg !12420
  %1936 = bitcast <4 x i32> %history.i.i.sroa.32.018811 to <4 x float>, !dbg !12424
  %1937 = fmul <4 x float> %_151.i.i.i.i16479, %1936, !dbg !12428
  %1938 = fadd <4 x float> %1929, %1937, !dbg !12429
  %1939 = fmul <4 x float> %_154.i.i.i.i16480, %1936, !dbg !12433
  %1940 = fadd <4 x float> %1931, %1939, !dbg !12437
  %1941 = fmul <4 x float> %_157.i.i.i.i16481, %1936, !dbg !12441
  %1942 = fadd <4 x float> %1933, %1941, !dbg !12445
  %1943 = fmul <4 x float> %_160.i.i.i.i16482, %1936, !dbg !12449
  %1944 = fadd <4 x float> %1935, %1943, !dbg !12453
  %1945 = bitcast <4 x i32> %history.i.i.sroa.35.018810 to <4 x float>, !dbg !12457
  %1946 = fmul <4 x float> %_165.i.i.i.i16483, %1945, !dbg !12461
  %1947 = fadd <4 x float> %1938, %1946, !dbg !12462
  %1948 = fmul <4 x float> %_168.i.i.i.i16484, %1945, !dbg !12466
  %1949 = fadd <4 x float> %1940, %1948, !dbg !12470
  %1950 = fmul <4 x float> %_171.i.i.i.i16485, %1945, !dbg !12474
  %1951 = fadd <4 x float> %1942, %1950, !dbg !12478
  %1952 = fmul <4 x float> %_174.i.i.i.i16486, %1945, !dbg !12482
  %1953 = fadd <4 x float> %1944, %1952, !dbg !12486
  %1954 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1947), !dbg !12490
  %1955 = fcmp olt <4 x float> %1954, %1846, !dbg !12494
  %1956 = select <4 x i1> %1955, <4 x float> %1846, <4 x float> %1954, !dbg !12498
  %1957 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1949), !dbg !12490
  %1958 = fcmp olt <4 x float> %1957, %1956, !dbg !12494
  %1959 = select <4 x i1> %1958, <4 x float> %1956, <4 x float> %1957, !dbg !12498
  %1960 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1951), !dbg !12490
  %1961 = fcmp olt <4 x float> %1960, %1959, !dbg !12494
  %1962 = select <4 x i1> %1961, <4 x float> %1959, <4 x float> %1960, !dbg !12498
  %1963 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1953), !dbg !12490
  %1964 = fcmp olt <4 x float> %1963, %1962, !dbg !12494
  %1965 = select <4 x i1> %1964, <4 x float> %1962, <4 x float> %1963, !dbg !12498
  %1966 = add nuw nsw i32 %iter.i.i.sroa.16.018809, 1, !dbg !12499
  %data.i4.i7504 = getelementptr inbounds nuw float, ptr %peaks_right.i, i32 %start1.i.i7499, !dbg !12500
  store <4 x float> %1965, ptr %data.i4.i7504, align 4, !dbg !12503, !alias.scope !12508, !noalias !12512
  %exitcond20987.not = icmp eq i32 %1966, %umax20986, !dbg !12054
  br i1 %exitcond20987.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i, label %bb5.i.i, !dbg !12054

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i: ; preds = %bb5.i.i, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7490
  %history.i.i.sroa.38.0.lcssa = phi <4 x i32> [ %history.i.i.sroa.38.0.lcssa23556, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7490 ], [ %history.i.i.sroa.35.018810, %bb5.i.i ], !dbg !11568
  %history.i.i.sroa.35.0.lcssa = phi <4 x i32> [ %history.i.i.sroa.35.0.lcssa23538, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7490 ], [ %history.i.i.sroa.32.018811, %bb5.i.i ], !dbg !11568
  %history.i.i.sroa.32.0.lcssa = phi <4 x i32> [ %history.i.i.sroa.32.0.lcssa23520, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7490 ], [ %history.i.i.sroa.29.018812, %bb5.i.i ], !dbg !11568
  %history.i.i.sroa.29.0.lcssa = phi <4 x i32> [ %history.i.i.sroa.29.0.lcssa23502, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7490 ], [ %history.i.i.sroa.26.018813, %bb5.i.i ], !dbg !11568
  %history.i.i.sroa.26.0.lcssa = phi <4 x i32> [ %history.i.i.sroa.26.0.lcssa23484, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7490 ], [ %history.i.i.sroa.22.018814, %bb5.i.i ], !dbg !11568
  %history.i.i.sroa.22.0.lcssa = phi <4 x i32> [ %history.i.i.sroa.22.0.lcssa23466, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7490 ], [ %history.i.i.sroa.19.018815, %bb5.i.i ], !dbg !11568
  %history.i.i.sroa.19.0.lcssa = phi <4 x i32> [ %history.i.i.sroa.19.0.lcssa23448, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7490 ], [ %history.i.i.sroa.16.018816, %bb5.i.i ], !dbg !11568
  %history.i.i.sroa.16.0.lcssa = phi <4 x i32> [ %history.i.i.sroa.16.0.lcssa23430, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7490 ], [ %history.i.i.sroa.13.018817, %bb5.i.i ], !dbg !11568
  %history.i.i.sroa.13.0.lcssa = phi <4 x i32> [ %history.i.i.sroa.13.0.lcssa23412, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7490 ], [ %history.i.i.sroa.10.018818, %bb5.i.i ], !dbg !11568
  %history.i.i.sroa.10.0.lcssa = phi <4 x i32> [ %history.i.i.sroa.10.0.lcssa23394, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7490 ], [ %history.i.i.sroa.7.018819, %bb5.i.i ], !dbg !11568
  %history.i.i.sroa.7.0.lcssa = phi <4 x i32> [ %history.i.i.sroa.7.0.lcssa23376, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7490 ], [ %history.i.i.sroa.0.018820, %bb5.i.i ], !dbg !11568
  %history.i.i.sroa.0.0.lcssa = phi <4 x i32> [ %history.i.i.sroa.0.0.lcssa23358, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7490 ], [ %lanes.i5934.sroa.0.0.copyload, %bb5.i.i ], !dbg !11568
  br i1 %_2.i744218781.not, label %bb15.i.loopexit, label %bb20.i.lr.ph, !dbg !12516

bb20.i.lr.ph:                                     ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i
  %_72.i.sroa.3.0.copyload.pre = load i32, ptr %_72.i.sroa.3.0..sroa_idx, align 4, !dbg !12518, !noalias !11512
  %_72.i.sroa.4.0.copyload.pre = load i32, ptr %_72.i.sroa.4.0..sroa_idx, align 4, !dbg !12518, !noalias !11512
  %_73.i.sroa.3.0.copyload.pre = load i32, ptr %_73.i.sroa.3.0..sroa_idx, align 4, !dbg !12519, !noalias !11512
  %_73.i.sroa.4.0.copyload.pre = load i32, ptr %_73.i.sroa.4.0..sroa_idx, align 4, !dbg !12519, !noalias !11512
  %_8.i26.i16408.pre = load <4 x float>, ptr %_114.i, align 16
  %_9.i27.i16409.pre = load <4 x float>, ptr %_115.i, align 16
  %_8.i.i16410.pre = load <4 x float>, ptr %_119.i, align 16
  %_9.i.i16411.pre = load <4 x float>, ptr %_120.i, align 16
  %_54.0.i254.i.pre = load ptr, ptr %1701, align 16
  %_54.1.i255.i.pre = load i32, ptr %1702, align 4
  %_18.i266.i = load i32, ptr %1698, align 4
  %_29.i186118833.not = icmp eq i32 %_18.i266.i, 0
  %_56.0.i276.i = load ptr, ptr %1703, align 8, !nonnull !10, !align !10173
  %_56.1.i277.i = load i32, ptr %1704, align 4
  %_37.i293.i16419 = load <4 x float>, ptr %1706, align 16
  %_58.1.i307.i = load i32, ptr %1708, align 4
  %_58.0.i306.i = load ptr, ptr %1709, align 16, !nonnull !10, !align !10173
  %_54.0.i.i = load ptr, ptr %1711, align 16, !nonnull !10, !align !10173
  %_54.1.i.i = load i32, ptr %1712, align 4
  %_18.i.i = load i32, ptr %1699, align 4
  %_29.i182718836.not = icmp eq i32 %_18.i.i, 0
  %_56.0.i.i = load ptr, ptr %1713, align 8, !nonnull !10, !align !10173
  %_56.1.i.i = load i32, ptr %1714, align 4
  %_37.i.i16428 = load <4 x float>, ptr %1716, align 16
  %_58.1.i.i = load i32, ptr %1718, align 4
  %_58.0.i.i = load ptr, ptr %1719, align 16, !nonnull !10, !align !10173
  br label %bb20.i, !dbg !12516

bb20.i:                                           ; preds = %bb20.i.lr.ph, %bb74.i
  %.lcssa1895119072 = phi <4 x float> [ %.lcssa1895119071.lcssa23634, %bb20.i.lr.ph ], [ %.lcssa1895119071, %bb74.i ]
  %storemerge.i1822.lcssa1892119036 = phi i32 [ %storemerge.i1822.lcssa1892119035.lcssa23614, %bb20.i.lr.ph ], [ %storemerge.i1822.lcssa1892119035, %bb74.i ]
  %.lcssa1889419000 = phi <4 x float> [ %.lcssa1889418999.lcssa23595, %bb20.i.lr.ph ], [ %.lcssa1889418999, %bb74.i ]
  %storemerge.i1856.lcssa1886418964 = phi i32 [ %storemerge.i1856.lcssa1886418963.lcssa23575, %bb20.i.lr.ph ], [ %storemerge.i1856.lcssa1886418963, %bb74.i ]
  %frame.sroa.0.0.i18959 = phi i32 [ 0, %bb20.i.lr.ph ], [ %_87.i, %bb74.i ]
  %main_cursor.sroa.0.1.i18958 = phi i32 [ %main_cursor.sroa.0.0.i19108, %bb20.i.lr.ph ], [ %main_cursor.sroa.0.2.i, %bb74.i ]
  %ring_cursor.sroa.0.1.i18957 = phi i32 [ %ring_cursor.sroa.0.0.i19107, %bb20.i.lr.ph ], [ %ring_cursor.sroa.0.2.i, %bb74.i ]
  %_69.i = sub nuw nsw i32 %spec.store.select.i, %frame.sroa.0.0.i18959, !dbg !12520
  %ring.i2454 = load i32, ptr %84, align 4, !dbg !12521, !alias.scope !12523, !noalias !12526, !noundef !10
  %main.i2455 = load i32, ptr %85, align 4, !dbg !12530, !alias.scope !12523, !noalias !12526, !noundef !10
  %_10.i2456 = add i32 %ring_cursor.sroa.0.1.i18957, 1, !dbg !12531
  %_38.not.i2457 = icmp ult i32 %_10.i2456, %ring.i2454, !dbg !12532
  %1967 = select i1 %_38.not.i2457, i32 0, i32 %ring.i2454, !dbg !12532
  %start1.sroa.0.0.i2458 = sub nuw i32 %_10.i2456, %1967, !dbg !12532
  %_12.i2460 = add i32 %_72.i.sroa.3.0.copyload.pre, %ring_cursor.sroa.0.1.i18957, !dbg !12534
  %_39.not.i2461 = icmp ult i32 %_12.i2460, %ring.i2454, !dbg !12535
  %1968 = select i1 %_39.not.i2461, i32 0, i32 %ring.i2454, !dbg !12535
  %left_end.sroa.0.0.i2462 = sub nuw i32 %_12.i2460, %1968, !dbg !12535
  %_15.i2464 = add i32 %_73.i.sroa.3.0.copyload.pre, %ring_cursor.sroa.0.1.i18957, !dbg !12537
  %_40.not.i2465 = icmp ult i32 %_15.i2464, %ring.i2454, !dbg !12538
  %1969 = select i1 %_40.not.i2465, i32 0, i32 %ring.i2454, !dbg !12538
  %right_end.sroa.0.0.i2466 = sub nuw i32 %_15.i2464, %1969, !dbg !12538
  %_18.i2468 = add i32 %_72.i.sroa.4.0.copyload.pre, %ring_cursor.sroa.0.1.i18957, !dbg !12540
  %_41.not.i2469 = icmp ult i32 %_18.i2468, %ring.i2454, !dbg !12541
  %1970 = select i1 %_41.not.i2469, i32 0, i32 %ring.i2454, !dbg !12541
  %left_expiring.sroa.0.0.i2470 = sub nuw i32 %_18.i2468, %1970, !dbg !12541
  %_21.i2472 = add i32 %_73.i.sroa.4.0.copyload.pre, %ring_cursor.sroa.0.1.i18957, !dbg !12543
  %_42.not.i2473 = icmp ult i32 %_21.i2472, %ring.i2454, !dbg !12544
  %1971 = select i1 %_42.not.i2473, i32 0, i32 %ring.i2454, !dbg !12544
  %right_expiring.sroa.0.0.i2474 = sub nuw i32 %_21.i2472, %1971, !dbg !12544
  %1972 = sub i32 %ring.i2454, %ring_cursor.sroa.0.1.i18957, !dbg !12546
  %spec.store.select.i2475 = tail call i32 @llvm.umin.i32(i32 %1972, i32 %_69.i), !dbg !12547
  %1973 = sub i32 %main.i2455, %main_cursor.sroa.0.1.i18958, !dbg !12549
  %_24.sroa.0.0.i2477 = tail call i32 @llvm.umin.i32(i32 %1973, i32 %spec.store.select.i2475), !dbg !12550
  %1974 = sub i32 %ring.i2454, %start1.sroa.0.0.i2458, !dbg !12552
  %_25.sroa.0.0.i2479 = tail call i32 @llvm.umin.i32(i32 %1974, i32 %_24.sroa.0.0.i2477), !dbg !12553
  %1975 = sub i32 %ring.i2454, %left_end.sroa.0.0.i2462, !dbg !12555
  %_27.sroa.0.0.i2481 = tail call i32 @llvm.umin.i32(i32 %1975, i32 %_25.sroa.0.0.i2479), !dbg !12556
  %1976 = sub i32 %ring.i2454, %right_end.sroa.0.0.i2466, !dbg !12558
  %_29.sroa.0.0.i2483 = tail call i32 @llvm.umin.i32(i32 %1976, i32 %_27.sroa.0.0.i2481), !dbg !12559
  %1977 = sub i32 %ring.i2454, %left_expiring.sroa.0.0.i2470, !dbg !12561
  %_31.sroa.0.0.i2485 = tail call i32 @llvm.umin.i32(i32 %1977, i32 %_29.sroa.0.0.i2483), !dbg !12562
  %1978 = sub i32 %ring.i2454, %right_expiring.sroa.0.0.i2474, !dbg !12564
  %run.sroa.0.0.i2487 = tail call i32 @llvm.umin.i32(i32 %1978, i32 %_31.sroa.0.0.i2485), !dbg !12565
  %_76.i = add i32 %frame.sroa.0.0.i18959, %iter1.sroa.0.0.i19109, !dbg !12567
  %base.i = shl i32 %_76.i, 2, !dbg !12567
  %base.i16407 = add i32 %run.sroa.0.0.i2487, %_76.i, !dbg !12570
  %_80.i = shl i32 %base.i16407, 2, !dbg !12570
  %_203.i = icmp ult i32 %_80.i, %base.i, !dbg !12573
  %_199.not.i = icmp ugt i32 %_80.i, %left_io.1
  %or.cond24.i = or i1 %_203.i, %_199.not.i, !dbg !12573
  br i1 %or.cond24.i, label %bb58.i, label %bb57.i, !dbg !12573, !prof !4596

bb58.i:                                           ; preds = %bb20.i
  store <4 x i32> %history.i46.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.7.0.lcssa, ptr %history.i46.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.10.0.lcssa, ptr %history.i46.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.13.0.lcssa, ptr %history.i46.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.16.0.lcssa, ptr %history.i46.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.19.0.lcssa, ptr %history.i46.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.22.0.lcssa, ptr %history.i46.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.26.0.lcssa, ptr %history.i46.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.29.0.lcssa, ptr %history.i46.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.32.0.lcssa, ptr %history.i46.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.35.0.lcssa, ptr %history.i46.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.38.0.lcssa, ptr %history.i46.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store i32 %storemerge.i1856.lcssa1886418963.lcssa23575, ptr %_22.i269.i, align 4
  store <4 x float> %.lcssa1889418999.lcssa23595, ptr %1705, align 16
  store i32 %storemerge.i1822.lcssa1892119035.lcssa23614, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1895119071.lcssa23634, ptr %1715, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i, i32 noundef %_80.i, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb67056a871aedf25dd2ba0a06d720f) #32, !dbg !12581, !noalias !11483
  unreachable, !dbg !12581

bb57.i:                                           ; preds = %bb20.i
  %_206.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %base.i, !dbg !12582
  %_207.not.i = icmp ugt i32 %_80.i, %right_io.1, !dbg !12586
  br i1 %_207.not.i, label %bb61.i, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7568, !dbg !12586, !prof !787

bb61.i:                                           ; preds = %bb57.i
  store <4 x i32> %history.i46.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.7.0.lcssa, ptr %history.i46.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.10.0.lcssa, ptr %history.i46.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.13.0.lcssa, ptr %history.i46.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.16.0.lcssa, ptr %history.i46.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.19.0.lcssa, ptr %history.i46.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.22.0.lcssa, ptr %history.i46.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.26.0.lcssa, ptr %history.i46.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.29.0.lcssa, ptr %history.i46.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.32.0.lcssa, ptr %history.i46.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.35.0.lcssa, ptr %history.i46.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.38.0.lcssa, ptr %history.i46.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store i32 %storemerge.i1856.lcssa1886418963.lcssa23575, ptr %_22.i269.i, align 4
  store <4 x float> %.lcssa1889418999.lcssa23595, ptr %1705, align 16
  store i32 %storemerge.i1822.lcssa1892119035.lcssa23614, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1895119071.lcssa23634, ptr %1715, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i, i32 noundef %_80.i, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1d0fce16c93a3aa07bd2f89733f587ef) #32, !dbg !12591, !noalias !11483
  unreachable, !dbg !12591

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7568: ; preds = %bb57.i
  %_212.i = getelementptr inbounds nuw float, ptr %right_io.0, i32 %base.i, !dbg !12592
  %_87.i = add nuw nsw i32 %run.sroa.0.0.i2487, %frame.sroa.0.0.i18959, !dbg !12596
  %_84.i = shl nuw nsw i32 %frame.sroa.0.0.i18959, 2, !dbg !12598
  %_221.i = getelementptr inbounds nuw float, ptr %peaks_left.i, i32 %_84.i, !dbg !12599
  %_230.i = getelementptr inbounds nuw float, ptr %peaks_right.i, i32 %_84.i, !dbg !12608
  %_2.i757118839.not = icmp eq i32 %run.sroa.0.0.i2487, 0, !dbg !12618
  br i1 %_2.i757118839.not, label %bb74.i, label %bb75.i.preheader, !dbg !12618

bb75.i.preheader:                                 ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7568
  %umin20998 = call i32 @llvm.umin.i32(i32 %1975, i32 %1976)
  %umin20999 = call i32 @llvm.umin.i32(i32 %umin20998, i32 %1977)
  %umin21000 = call i32 @llvm.umin.i32(i32 %umin20999, i32 %1978)
  %umin21001 = call i32 @llvm.umin.i32(i32 %umin21000, i32 %1974)
  %umin21002 = call i32 @llvm.umin.i32(i32 %umin21001, i32 %1972)
  %umin21003 = call i32 @llvm.umin.i32(i32 %umin21002, i32 %1973)
  %1979 = sub nsw i32 %umin21004, %frame.sroa.0.0.i18959
  %umin21005 = call i32 @llvm.umin.i32(i32 %umin21003, i32 %1979)
  %1980 = and i32 %umin21005, 1073741823
  br label %bb75.i

bb75.i:                                           ; preds = %bb75.i.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2062
  %1981 = phi <4 x float> [ %.lcssa1895119072, %bb75.i.preheader ], [ %2070, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2062 ]
  %storemerge.i182218899 = phi i32 [ %storemerge.i1822.lcssa1892119036, %bb75.i.preheader ], [ %storemerge.i1822, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2062 ]
  %1982 = phi <4 x float> [ %.lcssa1889419000, %bb75.i.preheader ], [ %2021, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2062 ]
  %storemerge.i185618842 = phi i32 [ %storemerge.i1856.lcssa1886418964, %bb75.i.preheader ], [ %storemerge.i1856, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2062 ]
  %iter.i.sroa.36.018841 = phi i32 [ 0, %bb75.i.preheader ], [ %1983, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2062 ]
  %start1.i.i.i.i.i.i7581 = shl i32 %iter.i.sroa.36.018841, 2, !dbg !12627
  %data.i.i.i.i7591 = getelementptr inbounds nuw float, ptr %_221.i, i32 %start1.i.i.i.i.i.i7581, !dbg !12633
  %lanes.i5925.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i.i.i7591, align 4, !dbg !12636, !alias.scope !12644, !noalias !12648
  %data.i.i7596 = getelementptr inbounds nuw float, ptr %_230.i, i32 %start1.i.i.i.i.i.i7581, !dbg !12652
  %lanes.i5916.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i7596, align 4, !dbg !12655, !alias.scope !12661, !noalias !12665
  %data.i.i.i.i.i.i7582 = getelementptr inbounds nuw float, ptr %_206.i, i32 %start1.i.i.i.i.i.i7581, !dbg !12669
  %lanes.i5907.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i.i.i.i.i7582, align 4, !dbg !12671, !alias.scope !12680, !noalias !12684
  %data.i5.i.i.i.i.i7586 = getelementptr inbounds nuw float, ptr %_212.i, i32 %start1.i.i.i.i.i.i7581, !dbg !12688
  %lanes.i5898.sroa.0.0.copyload = load <4 x i32>, ptr %data.i5.i.i.i.i.i7586, align 4, !dbg !12691, !alias.scope !12697, !noalias !12701
  %1983 = add nuw nsw i32 %iter.i.sroa.36.018841, 1, !dbg !12705
  %1984 = bitcast <4 x i32> %lanes.i5925.sroa.0.0.copyload to <4 x float>, !dbg !12706
  %1985 = bitcast <4 x i32> %lanes.i5916.sroa.0.0.copyload to <4 x float>, !dbg !12710
  %1986 = fcmp olt <4 x float> %1984, %1985, !dbg !12711
  %.v16412 = select <4 x i1> %1986, <4 x i32> %lanes.i5916.sroa.0.0.copyload, <4 x i32> %lanes.i5925.sroa.0.0.copyload, !dbg !12712
  %1987 = bitcast <4 x i32> %.v16412 to <16 x i8>, !dbg !12713
  %1988 = bitcast <4 x i32> %lanes.i5916.sroa.0.0.copyload to <16 x i8>, !dbg !12717
  %_4.i7608 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1987, <16 x i8> %1988, <16 x i8> %1700), !dbg !12721
  %_242.i = add i32 %iter.i.sroa.36.018841, %ring_cursor.sroa.0.1.i18957, !dbg !12722
  %_243.i = add i32 %iter.i.sroa.36.018841, %main_cursor.sroa.0.1.i18958, !dbg !12726
  %_244.i = add i32 %iter.i.sroa.36.018841, %left_end.sroa.0.0.i2462, !dbg !12727
  %_245.i = add i32 %iter.i.sroa.36.018841, %start1.sroa.0.0.i2458, !dbg !12728
  %_246.i = add i32 %iter.i.sroa.36.018841, %left_expiring.sroa.0.0.i2470, !dbg !12729
  %base.i9.i257.i = shl i32 %_242.i, 2, !dbg !12730
  %_7.i10.i258.i = add i32 %base.i9.i257.i, 4, !dbg !12733
  %1989 = or disjoint i32 %base.i9.i257.i, 3, !dbg !12734
  %or.cond.i13.i261.i.not = icmp ult i32 %1989, %_54.1.i255.i.pre, !dbg !12734
  br i1 %or.cond.i13.i261.i.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i262.i, label %bb4.i15.i322.i, !dbg !12734, !prof !10564

bb4.i15.i322.i:                                   ; preds = %bb75.i
  store <4 x i32> %history.i46.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.7.0.lcssa, ptr %history.i46.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.10.0.lcssa, ptr %history.i46.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.13.0.lcssa, ptr %history.i46.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.16.0.lcssa, ptr %history.i46.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.19.0.lcssa, ptr %history.i46.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.22.0.lcssa, ptr %history.i46.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.26.0.lcssa, ptr %history.i46.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.29.0.lcssa, ptr %history.i46.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.32.0.lcssa, ptr %history.i46.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.35.0.lcssa, ptr %history.i46.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.38.0.lcssa, ptr %history.i46.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store i32 %storemerge.i1856.lcssa1886418963.lcssa23575, ptr %_22.i269.i, align 4
  store <4 x float> %.lcssa1889418999.lcssa23595, ptr %1705, align 16
  store i32 %storemerge.i1822.lcssa1892119035.lcssa23614, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1895119071.lcssa23634, ptr %1715, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i9.i257.i, i32 noundef %_7.i10.i258.i, i32 noundef range(i32 0, 536870912) %_54.1.i255.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #32, !dbg !12738, !noalias !12739
  unreachable, !dbg !12738

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i262.i: ; preds = %bb75.i
  %1990 = bitcast <4 x i32> %lanes.i5925.sroa.0.0.copyload to <16 x i8>, !dbg !12753
  %_4.i7607 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1987, <16 x i8> %1990, <16 x i8> %1700), !dbg !12754
  %1991 = bitcast <16 x i8> %_4.i7607 to <4 x float>, !dbg !12755
  %1992 = fdiv <4 x float> %_8.i26.i16408.pre, %1991, !dbg !12760
  %1993 = bitcast <4 x float> %1992 to <16 x i8>, !dbg !12764
  %1994 = fcmp olt <4 x float> %_8.i26.i16408.pre, %1991, !dbg !12768
  %1995 = sext <4 x i1> %1994 to <4 x i32>, !dbg !12768
  %1996 = bitcast <4 x i32> %1995 to <16 x i8>, !dbg !12769
  %_4.i7611 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1993, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %1996), !dbg !12770
  %_17.i14.i263.i = getelementptr inbounds nuw float, ptr %_54.0.i254.i.pre, i32 %base.i9.i257.i, !dbg !12771
  store <16 x i8> %_4.i7611, ptr %_17.i14.i263.i, align 4, !dbg !12773, !alias.scope !12778, !noalias !12782
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12786), !dbg !12789
  %base.i1901 = shl i32 %_244.i, 2, !dbg !12790
  %1997 = or disjoint i32 %base.i1901, 3, !dbg !12793
  %or.cond.i1905.not = icmp ult i32 %1997, %_54.1.i255.i.pre, !dbg !12793
  br i1 %or.cond.i1905.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1909, label %bb4.i1908, !dbg !12793, !prof !10564

bb4.i1908:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i262.i
  store <4 x i32> %history.i46.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.7.0.lcssa, ptr %history.i46.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.10.0.lcssa, ptr %history.i46.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.13.0.lcssa, ptr %history.i46.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.16.0.lcssa, ptr %history.i46.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.19.0.lcssa, ptr %history.i46.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.22.0.lcssa, ptr %history.i46.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.26.0.lcssa, ptr %history.i46.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.29.0.lcssa, ptr %history.i46.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.32.0.lcssa, ptr %history.i46.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.35.0.lcssa, ptr %history.i46.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.38.0.lcssa, ptr %history.i46.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store i32 %storemerge.i1856.lcssa1886418963.lcssa23575, ptr %_22.i269.i, align 4
  store <4 x float> %.lcssa1889418999.lcssa23595, ptr %1705, align 16
  store i32 %storemerge.i1822.lcssa1892119035.lcssa23614, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1895119071.lcssa23634, ptr %1715, align 16
  %_5.i1902 = add i32 %base.i1901, 4, !dbg !12797
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1901, i32 noundef %_5.i1902, i32 noundef range(i32 0, 536870912) %_54.1.i255.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !12798, !noalias !12799
  unreachable, !dbg !12798

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1909: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i262.i
  %_15.i1907 = getelementptr inbounds nuw float, ptr %_54.0.i254.i.pre, i32 %base.i1901, !dbg !12805
  %lanes.i5616.sroa.0.0.copyload = load <4 x i32>, ptr %_15.i1907, align 4, !dbg !12807
  %1998 = icmp eq i32 %storemerge.i185618842, 0, !dbg !12812
  %_12.i184816413 = load <4 x float>, ptr %uniform_left.i, align 16, !dbg !12812
  %1999 = bitcast <4 x i32> %lanes.i5616.sroa.0.0.copyload to <4 x float>, !dbg !12812
  %2000 = fcmp olt <4 x float> %_12.i184816413, %1999, !dbg !12812
  %2001 = select <4 x i1> %2000, <4 x float> %_12.i184816413, <4 x float> %1999, !dbg !12812
  %2002 = bitcast <4 x float> %2001 to <4 x i32>, !dbg !12812
  %.sroa.08623.0 = select i1 %1998, <4 x i32> %lanes.i5616.sroa.0.0.copyload, <4 x i32> %2002, !dbg !12812
  store <4 x i32> %.sroa.08623.0, ptr %uniform_left.i, align 16, !dbg !12813, !alias.scope !12786, !noalias !12814
  %_15.i1851 = add i32 %storemerge.i185618842, 1, !dbg !12816
  %complete.i1852 = icmp eq i32 %_15.i1851, %_18.i266.i, !dbg !12816
  br i1 %complete.i1852, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1891, label %bb7.i1853, !dbg !12817

bb7.i1853:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1909
  %base.i1892 = shl i32 %_245.i, 2, !dbg !12818
  %2003 = or disjoint i32 %base.i1892, 3, !dbg !12820
  %or.cond.i1896.not = icmp ult i32 %2003, %_54.1.i255.i.pre, !dbg !12820
  br i1 %or.cond.i1896.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1900, label %bb4.i1899, !dbg !12820, !prof !10564

bb4.i1899:                                        ; preds = %bb7.i1853
  store <4 x i32> %history.i46.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.7.0.lcssa, ptr %history.i46.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.10.0.lcssa, ptr %history.i46.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.13.0.lcssa, ptr %history.i46.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.16.0.lcssa, ptr %history.i46.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.19.0.lcssa, ptr %history.i46.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.22.0.lcssa, ptr %history.i46.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.26.0.lcssa, ptr %history.i46.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.29.0.lcssa, ptr %history.i46.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.32.0.lcssa, ptr %history.i46.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.35.0.lcssa, ptr %history.i46.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.38.0.lcssa, ptr %history.i46.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store i32 %storemerge.i1856.lcssa1886418963.lcssa23575, ptr %_22.i269.i, align 4
  store <4 x float> %.lcssa1889418999.lcssa23595, ptr %1705, align 16
  store i32 %storemerge.i1822.lcssa1892119035.lcssa23614, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1895119071.lcssa23634, ptr %1715, align 16
  %_5.i1893 = add i32 %base.i1892, 4, !dbg !12824
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1892, i32 noundef %_5.i1893, i32 noundef range(i32 0, 536870912) %_54.1.i255.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !12825, !noalias !12826
  unreachable, !dbg !12825

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1900: ; preds = %bb7.i1853
  %_15.i1898 = getelementptr inbounds nuw float, ptr %_54.0.i254.i.pre, i32 %base.i1892, !dbg !12830
  %lanes.i5623.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1898, align 4, !dbg !12832, !alias.scope !12837, !noalias !12841
  %2004 = bitcast <4 x i32> %.sroa.08623.0 to <4 x float>, !dbg !12845
  %2005 = fcmp olt <4 x float> %lanes.i5623.sroa.0.0.copyload, %2004, !dbg !12849
  %2006 = select <4 x i1> %2005, <4 x float> %lanes.i5623.sroa.0.0.copyload, <4 x float> %2004, !dbg !12850
  %2007 = bitcast <4 x float> %2006 to <4 x i32>, !dbg !12851
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1877, !dbg !12853

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1891: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1909
  %2008 = bitcast <4 x i32> %lanes.i5616.sroa.0.0.copyload to <4 x float>, !dbg !12817
  br i1 %_29.i186118833.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1877, label %bb19.i1862, !dbg !12854

bb19.i1862:                                       ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1891, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
  %end.sroa.0.0.i186018835 = phi i32 [ %2014, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit ], [ %_244.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1891 ]
  %iter.sroa.0.0.i185918834 = phi i32 [ %_30.i1863, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1891 ]
  %2009 = phi <4 x float> [ %2012, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit ], [ %2008, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1891 ]
  %base.i1878 = shl i32 %end.sroa.0.0.i186018835, 2, !dbg !12857
  %2010 = or disjoint i32 %base.i1878, 3, !dbg !12859
  %or.cond.i1880.not = icmp ult i32 %2010, %_54.1.i255.i.pre, !dbg !12859
  br i1 %or.cond.i1880.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb4.i, !dbg !12859, !prof !10564

bb4.i:                                            ; preds = %bb19.i1862
  store <4 x i32> %history.i46.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.7.0.lcssa, ptr %history.i46.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.10.0.lcssa, ptr %history.i46.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.13.0.lcssa, ptr %history.i46.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.16.0.lcssa, ptr %history.i46.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.19.0.lcssa, ptr %history.i46.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.22.0.lcssa, ptr %history.i46.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.26.0.lcssa, ptr %history.i46.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.29.0.lcssa, ptr %history.i46.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.32.0.lcssa, ptr %history.i46.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.35.0.lcssa, ptr %history.i46.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.38.0.lcssa, ptr %history.i46.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store i32 %storemerge.i1856.lcssa1886418963.lcssa23575, ptr %_22.i269.i, align 4
  store <4 x float> %.lcssa1889418999.lcssa23595, ptr %1705, align 16
  store i32 %storemerge.i1822.lcssa1892119035.lcssa23614, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1895119071.lcssa23634, ptr %1715, align 16
  %_5.i = add i32 %base.i1878, 4, !dbg !12863
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1878, i32 noundef %_5.i, i32 noundef range(i32 0, 536870912) %_54.1.i255.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !12864, !noalias !12865
  unreachable, !dbg !12864

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %bb19.i1862
  %_30.i1863 = add nuw i32 %iter.sroa.0.0.i185918834, 1, !dbg !12869
  %_15.i1882 = getelementptr inbounds nuw float, ptr %_54.0.i254.i.pre, i32 %base.i1878, !dbg !12872
  %lanes.i5637.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1882, align 4, !dbg !12874, !alias.scope !12879, !noalias !12883
  %2011 = fcmp olt <4 x float> %2009, %lanes.i5637.sroa.0.0.copyload, !dbg !12887
  %2012 = select <4 x i1> %2011, <4 x float> %2009, <4 x float> %lanes.i5637.sroa.0.0.copyload, !dbg !12891
  store <4 x float> %2012, ptr %_15.i1882, align 4, !dbg !12892, !alias.scope !12898, !noalias !12902
  %2013 = icmp eq i32 %end.sroa.0.0.i186018835, 0, !dbg !12908
  %spec.store.select.i1874 = select i1 %2013, i32 %ring.i, i32 %end.sroa.0.0.i186018835, !dbg !12908
  %2014 = add i32 %spec.store.select.i1874, -1, !dbg !12909
  %exitcond20989.not = icmp eq i32 %_30.i1863, %_18.i266.i, !dbg !12910
  br i1 %exitcond20989.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1877, label %bb19.i1862, !dbg !12854

_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1877: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1891, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1900
  %.sroa.08623.1 = phi <4 x i32> [ %2007, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1900 ], [ %.sroa.08623.0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1891 ], [ %.sroa.08623.0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit ], !dbg !12912
  %storemerge.i1856 = phi i32 [ %_15.i1851, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1900 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1891 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit ], !dbg !12913
  %2015 = bitcast <4 x i32> %.sroa.08623.1 to <4 x float>, !dbg !12914
  %2016 = fmul <4 x float> %2015, splat (float 1.638400e+04), !dbg !12918
  %2017 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %2016), !dbg !12919
  %2018 = fmul <4 x float> %2017, splat (float 0x3F10000000000000), !dbg !12923
  %base.i2081 = shl i32 %_246.i, 2, !dbg !12927
  %2019 = or disjoint i32 %base.i2081, 3, !dbg !12929
  %or.cond.i2085.not = icmp ult i32 %2019, %_56.1.i277.i, !dbg !12929
  br i1 %or.cond.i2085.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2089, label %bb4.i2088, !dbg !12929, !prof !10564

bb4.i2088:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1877
  store <4 x i32> %history.i46.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.7.0.lcssa, ptr %history.i46.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.10.0.lcssa, ptr %history.i46.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.13.0.lcssa, ptr %history.i46.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.16.0.lcssa, ptr %history.i46.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.19.0.lcssa, ptr %history.i46.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.22.0.lcssa, ptr %history.i46.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.26.0.lcssa, ptr %history.i46.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.29.0.lcssa, ptr %history.i46.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.32.0.lcssa, ptr %history.i46.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.35.0.lcssa, ptr %history.i46.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.38.0.lcssa, ptr %history.i46.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store i32 %storemerge.i1856.lcssa1886418963.lcssa23575, ptr %_22.i269.i, align 4
  store <4 x float> %.lcssa1889418999.lcssa23595, ptr %1705, align 16
  store i32 %storemerge.i1822.lcssa1892119035.lcssa23614, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1895119071.lcssa23634, ptr %1715, align 16
  %_5.i2082 = add i32 %base.i2081, 4, !dbg !12933
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i2081, i32 noundef %_5.i2082, i32 noundef range(i32 0, 536870912) %_56.1.i277.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !12934, !noalias !12935
  unreachable, !dbg !12934

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2089: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1877
  %_15.i2087 = getelementptr inbounds nuw float, ptr %_56.0.i276.i, i32 %base.i2081, !dbg !12939
  %lanes.i.sroa.0.0.copyload = load <4 x float>, ptr %_15.i2087, align 4, !dbg !12941, !alias.scope !12946, !noalias !12950
  %2020 = fadd <4 x float> %2018, %1982, !dbg !12954
  %2021 = fsub <4 x float> %2020, %lanes.i.sroa.0.0.copyload, !dbg !12958
  %_8.not.i4.i288.i = icmp ugt i32 %_7.i10.i258.i, %_56.1.i277.i
  br i1 %_8.not.i4.i288.i, label %bb4.i7.i321.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i290.i, !dbg !12962, !prof !4596

bb4.i7.i321.i:                                    ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2089
  store <4 x i32> %history.i46.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.7.0.lcssa, ptr %history.i46.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.10.0.lcssa, ptr %history.i46.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.13.0.lcssa, ptr %history.i46.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.16.0.lcssa, ptr %history.i46.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.19.0.lcssa, ptr %history.i46.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.22.0.lcssa, ptr %history.i46.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.26.0.lcssa, ptr %history.i46.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.29.0.lcssa, ptr %history.i46.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.32.0.lcssa, ptr %history.i46.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.35.0.lcssa, ptr %history.i46.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.38.0.lcssa, ptr %history.i46.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store i32 %storemerge.i1856.lcssa1886418963.lcssa23575, ptr %_22.i269.i, align 4
  store <4 x float> %.lcssa1889418999.lcssa23595, ptr %1705, align 16
  store i32 %storemerge.i1822.lcssa1892119035.lcssa23614, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1895119071.lcssa23634, ptr %1715, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i9.i257.i, i32 noundef %_7.i10.i258.i, i32 noundef range(i32 0, 536870912) %_56.1.i277.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #32, !dbg !12967, !noalias !12968
  unreachable, !dbg !12967

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i290.i: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2089
  %_17.i6.i291.i = getelementptr inbounds nuw float, ptr %_56.0.i276.i, i32 %base.i9.i257.i, !dbg !12972
  store <4 x float> %2018, ptr %_17.i6.i291.i, align 4, !dbg !12974, !alias.scope !12979, !noalias !12983
  %_41.i296.i16420 = load <4 x float>, ptr %1707, align 16, !dbg !12987
  %2022 = fdiv <4 x float> %2021, %_37.i293.i16419, !dbg !12988
  %2023 = fsub <4 x float> splat (float 1.000000e+00), %2022, !dbg !12992
  %2024 = fsub <4 x float> %2023, %_41.i296.i16420, !dbg !12996
  %2025 = fmul <4 x float> %_9.i27.i16409.pre, %2024, !dbg !13000
  %2026 = fadd <4 x float> %_41.i296.i16420, %2025, !dbg !13004
  %2027 = fcmp olt <4 x float> %2026, %2023, !dbg !13007
  %2028 = select <4 x i1> %2027, <4 x float> %2023, <4 x float> %2026, !dbg !13011
  %2029 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %2028), !dbg !13012
  %2030 = fcmp uge <4 x float> %2029, splat (float 0x3BC79CA100000000), !dbg !13017
  %2031 = bitcast <4 x float> %2028 to <4 x i32>, !dbg !13022
  %2032 = select <4 x i1> %2030, <4 x i32> %2031, <4 x i32> zeroinitializer, !dbg !13022
  store <4 x i32> %2032, ptr %1707, align 16, !dbg !13025
  %base.i2072 = shl i32 %_243.i, 2, !dbg !13026
  %_5.i2073 = add i32 %base.i2072, 4, !dbg !13028
  %2033 = or disjoint i32 %base.i2072, 3, !dbg !13029
  %or.cond.i2076.not = icmp ult i32 %2033, %_58.1.i307.i, !dbg !13029
  br i1 %or.cond.i2076.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2080, label %bb4.i2079, !dbg !13029, !prof !10564

bb4.i2079:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i290.i
  store <4 x i32> %history.i46.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.7.0.lcssa, ptr %history.i46.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.10.0.lcssa, ptr %history.i46.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.13.0.lcssa, ptr %history.i46.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.16.0.lcssa, ptr %history.i46.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.19.0.lcssa, ptr %history.i46.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.22.0.lcssa, ptr %history.i46.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.26.0.lcssa, ptr %history.i46.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.29.0.lcssa, ptr %history.i46.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.32.0.lcssa, ptr %history.i46.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.35.0.lcssa, ptr %history.i46.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.38.0.lcssa, ptr %history.i46.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store i32 %storemerge.i1856.lcssa1886418963.lcssa23575, ptr %_22.i269.i, align 4
  store <4 x float> %.lcssa1889418999.lcssa23595, ptr %1705, align 16
  store i32 %storemerge.i1822.lcssa1892119035.lcssa23614, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1895119071.lcssa23634, ptr %1715, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i2072, i32 noundef %_5.i2073, i32 noundef range(i32 0, 536870912) %_58.1.i307.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !13033, !noalias !13034
  unreachable, !dbg !13033

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2080: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i290.i
  %2034 = bitcast <4 x i32> %2032 to <4 x float>, !dbg !13038
  %2035 = fsub <4 x float> splat (float 1.000000e+00), %2034, !dbg !13042
  %_15.i2078 = getelementptr inbounds nuw float, ptr %_58.0.i306.i, i32 %base.i2072, !dbg !13043
  %lanes.i5483.sroa.0.0.copyload = load <4 x i32>, ptr %_15.i2078, align 4, !dbg !13045, !alias.scope !13050, !noalias !13054
  store <4 x i32> %lanes.i5907.sroa.0.0.copyload, ptr %_15.i2078, align 4, !dbg !13058, !alias.scope !13064, !noalias !13068
  %2036 = bitcast <4 x i32> %lanes.i5483.sroa.0.0.copyload to <4 x float>, !dbg !13074
  %2037 = fmul <4 x float> %2035, %2036, !dbg !13078
  %2038 = bitcast <4 x i32> %lanes.i5483.sroa.0.0.copyload to <16 x i8>, !dbg !13079
  %2039 = bitcast <4 x float> %2037 to <16 x i8>, !dbg !13083
  %_4.i7622 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %2038, <16 x i8> %2039, <16 x i8> %1710), !dbg !13084
  store <16 x i8> %_4.i7622, ptr %data.i.i.i.i.i.i7582, align 4, !dbg !13085, !alias.scope !13090, !noalias !13094
  %_249.i = add i32 %iter.i.sroa.36.018841, %right_end.sroa.0.0.i2466, !dbg !13098
  %_251.i = add i32 %iter.i.sroa.36.018841, %right_expiring.sroa.0.0.i2474, !dbg !13100
  %_8.not.i12.i.i = icmp ugt i32 %_7.i10.i258.i, %_54.1.i.i
  br i1 %_8.not.i12.i.i, label %bb4.i15.i.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i, !dbg !13101, !prof !4596

bb4.i15.i.i:                                      ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2080
  store <4 x i32> %history.i46.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.7.0.lcssa, ptr %history.i46.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.10.0.lcssa, ptr %history.i46.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.13.0.lcssa, ptr %history.i46.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.16.0.lcssa, ptr %history.i46.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.19.0.lcssa, ptr %history.i46.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.22.0.lcssa, ptr %history.i46.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.26.0.lcssa, ptr %history.i46.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.29.0.lcssa, ptr %history.i46.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.32.0.lcssa, ptr %history.i46.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.35.0.lcssa, ptr %history.i46.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.38.0.lcssa, ptr %history.i46.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store i32 %storemerge.i1856.lcssa1886418963.lcssa23575, ptr %_22.i269.i, align 4
  store <4 x float> %.lcssa1889418999.lcssa23595, ptr %1705, align 16
  store i32 %storemerge.i1822.lcssa1892119035.lcssa23614, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1895119071.lcssa23634, ptr %1715, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i9.i257.i, i32 noundef %_7.i10.i258.i, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #32, !dbg !13107, !noalias !13108
  unreachable, !dbg !13107

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2080
  %2040 = bitcast <16 x i8> %_4.i7608 to <4 x float>, !dbg !13122
  %2041 = fdiv <4 x float> %_8.i.i16410.pre, %2040, !dbg !13127
  %2042 = bitcast <4 x float> %2041 to <16 x i8>, !dbg !13131
  %2043 = fcmp olt <4 x float> %_8.i.i16410.pre, %2040, !dbg !13135
  %2044 = sext <4 x i1> %2043 to <4 x i32>, !dbg !13135
  %2045 = bitcast <4 x i32> %2044 to <16 x i8>, !dbg !13136
  %_4.i7624 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %2042, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %2045), !dbg !13137
  %_17.i14.i.i = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %base.i9.i257.i, !dbg !13138
  store <16 x i8> %_4.i7624, ptr %_17.i14.i.i, align 4, !dbg !13140, !alias.scope !13145, !noalias !13149
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13153), !dbg !13156
  %base.i1937 = shl i32 %_249.i, 2, !dbg !13157
  %2046 = or disjoint i32 %base.i1937, 3, !dbg !13160
  %or.cond.i1941.not = icmp ult i32 %2046, %_54.1.i.i, !dbg !13160
  br i1 %or.cond.i1941.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1945, label %bb4.i1944, !dbg !13160, !prof !10564

bb4.i1944:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i
  store <4 x i32> %history.i46.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.7.0.lcssa, ptr %history.i46.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.10.0.lcssa, ptr %history.i46.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.13.0.lcssa, ptr %history.i46.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.16.0.lcssa, ptr %history.i46.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.19.0.lcssa, ptr %history.i46.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.22.0.lcssa, ptr %history.i46.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.26.0.lcssa, ptr %history.i46.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.29.0.lcssa, ptr %history.i46.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.32.0.lcssa, ptr %history.i46.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.35.0.lcssa, ptr %history.i46.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.38.0.lcssa, ptr %history.i46.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store i32 %storemerge.i1856.lcssa1886418963.lcssa23575, ptr %_22.i269.i, align 4
  store <4 x float> %.lcssa1889418999.lcssa23595, ptr %1705, align 16
  store i32 %storemerge.i1822.lcssa1892119035.lcssa23614, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1895119071.lcssa23634, ptr %1715, align 16
  %_5.i1938 = add i32 %base.i1937, 4, !dbg !13164
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1937, i32 noundef %_5.i1938, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !13165, !noalias !13166
  unreachable, !dbg !13165

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1945: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i
  %_15.i1943 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %base.i1937, !dbg !13172
  %lanes.i5588.sroa.0.0.copyload = load <4 x i32>, ptr %_15.i1943, align 4, !dbg !13174
  %2047 = icmp eq i32 %storemerge.i182218899, 0, !dbg !13179
  %_12.i181416422 = load <4 x float>, ptr %uniform_right.i, align 16, !dbg !13179
  %2048 = bitcast <4 x i32> %lanes.i5588.sroa.0.0.copyload to <4 x float>, !dbg !13179
  %2049 = fcmp olt <4 x float> %_12.i181416422, %2048, !dbg !13179
  %2050 = select <4 x i1> %2049, <4 x float> %_12.i181416422, <4 x float> %2048, !dbg !13179
  %2051 = bitcast <4 x float> %2050 to <4 x i32>, !dbg !13179
  %.sroa.08549.0 = select i1 %2047, <4 x i32> %lanes.i5588.sroa.0.0.copyload, <4 x i32> %2051, !dbg !13179
  store <4 x i32> %.sroa.08549.0, ptr %uniform_right.i, align 16, !dbg !13180, !alias.scope !13153, !noalias !13181
  %_15.i1817 = add i32 %storemerge.i182218899, 1, !dbg !13183
  %complete.i1818 = icmp eq i32 %_15.i1817, %_18.i.i, !dbg !13183
  br i1 %complete.i1818, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1927, label %bb7.i1819, !dbg !13184

bb7.i1819:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1945
  %base.i1928 = shl i32 %_245.i, 2, !dbg !13185
  %2052 = or disjoint i32 %base.i1928, 3, !dbg !13187
  %or.cond.i1932.not = icmp ult i32 %2052, %_54.1.i.i, !dbg !13187
  br i1 %or.cond.i1932.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1936, label %bb4.i1935, !dbg !13187, !prof !10564

bb4.i1935:                                        ; preds = %bb7.i1819
  store <4 x i32> %history.i46.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.7.0.lcssa, ptr %history.i46.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.10.0.lcssa, ptr %history.i46.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.13.0.lcssa, ptr %history.i46.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.16.0.lcssa, ptr %history.i46.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.19.0.lcssa, ptr %history.i46.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.22.0.lcssa, ptr %history.i46.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.26.0.lcssa, ptr %history.i46.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.29.0.lcssa, ptr %history.i46.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.32.0.lcssa, ptr %history.i46.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.35.0.lcssa, ptr %history.i46.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.38.0.lcssa, ptr %history.i46.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store i32 %storemerge.i1856.lcssa1886418963.lcssa23575, ptr %_22.i269.i, align 4
  store <4 x float> %.lcssa1889418999.lcssa23595, ptr %1705, align 16
  store i32 %storemerge.i1822.lcssa1892119035.lcssa23614, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1895119071.lcssa23634, ptr %1715, align 16
  %_5.i1929 = add i32 %base.i1928, 4, !dbg !13191
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1928, i32 noundef %_5.i1929, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !13192, !noalias !13193
  unreachable, !dbg !13192

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1936: ; preds = %bb7.i1819
  %_15.i1934 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %base.i1928, !dbg !13197
  %lanes.i5595.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1934, align 4, !dbg !13199, !alias.scope !13204, !noalias !13208
  %2053 = bitcast <4 x i32> %.sroa.08549.0 to <4 x float>, !dbg !13212
  %2054 = fcmp olt <4 x float> %lanes.i5595.sroa.0.0.copyload, %2053, !dbg !13216
  %2055 = select <4 x i1> %2054, <4 x float> %lanes.i5595.sroa.0.0.copyload, <4 x float> %2053, !dbg !13217
  %2056 = bitcast <4 x float> %2055 to <4 x i32>, !dbg !13218
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1843, !dbg !13220

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1927: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1945
  %2057 = bitcast <4 x i32> %lanes.i5588.sroa.0.0.copyload to <4 x float>, !dbg !13184
  br i1 %_29.i182718836.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1843, label %bb19.i1828, !dbg !13221

bb19.i1828:                                       ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1927, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1918
  %end.sroa.0.0.i182618838 = phi i32 [ %2063, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1918 ], [ %_249.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1927 ]
  %iter.sroa.0.0.i182518837 = phi i32 [ %_30.i1829, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1918 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1927 ]
  %2058 = phi <4 x float> [ %2061, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1918 ], [ %2057, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1927 ]
  %base.i1910 = shl i32 %end.sroa.0.0.i182618838, 2, !dbg !13224
  %2059 = or disjoint i32 %base.i1910, 3, !dbg !13226
  %or.cond.i1914.not = icmp ult i32 %2059, %_54.1.i.i, !dbg !13226
  br i1 %or.cond.i1914.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1918, label %bb4.i1917, !dbg !13226, !prof !10564

bb4.i1917:                                        ; preds = %bb19.i1828
  store <4 x i32> %history.i46.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.7.0.lcssa, ptr %history.i46.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.10.0.lcssa, ptr %history.i46.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.13.0.lcssa, ptr %history.i46.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.16.0.lcssa, ptr %history.i46.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.19.0.lcssa, ptr %history.i46.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.22.0.lcssa, ptr %history.i46.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.26.0.lcssa, ptr %history.i46.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.29.0.lcssa, ptr %history.i46.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.32.0.lcssa, ptr %history.i46.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.35.0.lcssa, ptr %history.i46.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.38.0.lcssa, ptr %history.i46.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store i32 %storemerge.i1856.lcssa1886418963.lcssa23575, ptr %_22.i269.i, align 4
  store <4 x float> %.lcssa1889418999.lcssa23595, ptr %1705, align 16
  store i32 %storemerge.i1822.lcssa1892119035.lcssa23614, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1895119071.lcssa23634, ptr %1715, align 16
  %_5.i1911 = add i32 %base.i1910, 4, !dbg !13230
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1910, i32 noundef %_5.i1911, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !13231, !noalias !13232
  unreachable, !dbg !13231

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1918: ; preds = %bb19.i1828
  %_30.i1829 = add nuw i32 %iter.sroa.0.0.i182518837, 1, !dbg !13236
  %_15.i1916 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %base.i1910, !dbg !13239
  %lanes.i5609.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1916, align 4, !dbg !13241, !alias.scope !13246, !noalias !13250
  %2060 = fcmp olt <4 x float> %2058, %lanes.i5609.sroa.0.0.copyload, !dbg !13254
  %2061 = select <4 x i1> %2060, <4 x float> %2058, <4 x float> %lanes.i5609.sroa.0.0.copyload, !dbg !13258
  store <4 x float> %2061, ptr %_15.i1916, align 4, !dbg !13259, !alias.scope !13265, !noalias !13269
  %2062 = icmp eq i32 %end.sroa.0.0.i182618838, 0, !dbg !13275
  %spec.store.select.i1840 = select i1 %2062, i32 %ring.i, i32 %end.sroa.0.0.i182618838, !dbg !13275
  %2063 = add i32 %spec.store.select.i1840, -1, !dbg !13276
  %exitcond20994.not = icmp eq i32 %_30.i1829, %_18.i.i, !dbg !13277
  br i1 %exitcond20994.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1843, label %bb19.i1828, !dbg !13221

_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1843: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1918, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1927, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1936
  %.sroa.08549.1 = phi <4 x i32> [ %2056, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1936 ], [ %.sroa.08549.0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1927 ], [ %.sroa.08549.0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1918 ], !dbg !13279
  %storemerge.i1822 = phi i32 [ %_15.i1817, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1936 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1927 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1918 ], !dbg !13280
  %2064 = bitcast <4 x i32> %.sroa.08549.1 to <4 x float>, !dbg !13281
  %2065 = fmul <4 x float> %2064, splat (float 1.638400e+04), !dbg !13285
  %2066 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %2065), !dbg !13286
  %2067 = fmul <4 x float> %2066, splat (float 0x3F10000000000000), !dbg !13290
  %base.i2063 = shl i32 %_251.i, 2, !dbg !13294
  %2068 = or disjoint i32 %base.i2063, 3, !dbg !13296
  %or.cond.i2067.not = icmp ult i32 %2068, %_56.1.i.i, !dbg !13296
  br i1 %or.cond.i2067.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2071, label %bb4.i2070, !dbg !13296, !prof !10564

bb4.i2070:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1843
  store <4 x i32> %history.i46.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.7.0.lcssa, ptr %history.i46.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.10.0.lcssa, ptr %history.i46.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.13.0.lcssa, ptr %history.i46.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.16.0.lcssa, ptr %history.i46.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.19.0.lcssa, ptr %history.i46.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.22.0.lcssa, ptr %history.i46.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.26.0.lcssa, ptr %history.i46.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.29.0.lcssa, ptr %history.i46.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.32.0.lcssa, ptr %history.i46.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.35.0.lcssa, ptr %history.i46.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.38.0.lcssa, ptr %history.i46.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store i32 %storemerge.i1856.lcssa1886418963.lcssa23575, ptr %_22.i269.i, align 4
  store <4 x float> %.lcssa1889418999.lcssa23595, ptr %1705, align 16
  store i32 %storemerge.i1822.lcssa1892119035.lcssa23614, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1895119071.lcssa23634, ptr %1715, align 16
  %_5.i2064 = add i32 %base.i2063, 4, !dbg !13300
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i2063, i32 noundef %_5.i2064, i32 noundef range(i32 0, 536870912) %_56.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !13301, !noalias !13302
  unreachable, !dbg !13301

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2071: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1843
  %_15.i2069 = getelementptr inbounds nuw float, ptr %_56.0.i.i, i32 %base.i2063, !dbg !13306
  %lanes.i5490.sroa.0.0.copyload = load <4 x float>, ptr %_15.i2069, align 4, !dbg !13308, !alias.scope !13313, !noalias !13317
  %2069 = fadd <4 x float> %2067, %1981, !dbg !13321
  %2070 = fsub <4 x float> %2069, %lanes.i5490.sroa.0.0.copyload, !dbg !13325
  %_8.not.i4.i.i = icmp ugt i32 %_7.i10.i258.i, %_56.1.i.i
  br i1 %_8.not.i4.i.i, label %bb4.i7.i.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i, !dbg !13329, !prof !4596

bb4.i7.i.i:                                       ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2071
  store <4 x i32> %history.i46.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.7.0.lcssa, ptr %history.i46.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.10.0.lcssa, ptr %history.i46.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.13.0.lcssa, ptr %history.i46.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.16.0.lcssa, ptr %history.i46.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.19.0.lcssa, ptr %history.i46.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.22.0.lcssa, ptr %history.i46.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.26.0.lcssa, ptr %history.i46.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.29.0.lcssa, ptr %history.i46.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.32.0.lcssa, ptr %history.i46.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.35.0.lcssa, ptr %history.i46.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.38.0.lcssa, ptr %history.i46.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store i32 %storemerge.i1856.lcssa1886418963.lcssa23575, ptr %_22.i269.i, align 4
  store <4 x float> %.lcssa1889418999.lcssa23595, ptr %1705, align 16
  store i32 %storemerge.i1822.lcssa1892119035.lcssa23614, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1895119071.lcssa23634, ptr %1715, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i9.i257.i, i32 noundef %_7.i10.i258.i, i32 noundef range(i32 0, 536870912) %_56.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #32, !dbg !13334, !noalias !13335
  unreachable, !dbg !13334

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2071
  %_17.i6.i.i = getelementptr inbounds nuw float, ptr %_56.0.i.i, i32 %base.i9.i257.i, !dbg !13339
  store <4 x float> %2067, ptr %_17.i6.i.i, align 4, !dbg !13341, !alias.scope !13346, !noalias !13350
  %_41.i.i16429 = load <4 x float>, ptr %1717, align 16, !dbg !13354
  %2071 = fdiv <4 x float> %2070, %_37.i.i16428, !dbg !13355
  %2072 = fsub <4 x float> splat (float 1.000000e+00), %2071, !dbg !13359
  %2073 = fsub <4 x float> %2072, %_41.i.i16429, !dbg !13363
  %2074 = fmul <4 x float> %_9.i.i16411.pre, %2073, !dbg !13367
  %2075 = fadd <4 x float> %_41.i.i16429, %2074, !dbg !13371
  %2076 = fcmp olt <4 x float> %2075, %2072, !dbg !13374
  %2077 = select <4 x i1> %2076, <4 x float> %2072, <4 x float> %2075, !dbg !13378
  %2078 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %2077), !dbg !13379
  %2079 = fcmp uge <4 x float> %2078, splat (float 0x3BC79CA100000000), !dbg !13384
  %2080 = bitcast <4 x float> %2077 to <4 x i32>, !dbg !13389
  %2081 = select <4 x i1> %2079, <4 x i32> %2080, <4 x i32> zeroinitializer, !dbg !13389
  store <4 x i32> %2081, ptr %1717, align 16, !dbg !13392
  %_6.not.i2057 = icmp ugt i32 %_5.i2073, %_58.1.i.i
  br i1 %_6.not.i2057, label %bb4.i2061, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2062, !dbg !13393, !prof !4596

bb4.i2061:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i
  store <4 x i32> %history.i46.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.7.0.lcssa, ptr %history.i46.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.10.0.lcssa, ptr %history.i46.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.13.0.lcssa, ptr %history.i46.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.16.0.lcssa, ptr %history.i46.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.19.0.lcssa, ptr %history.i46.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.22.0.lcssa, ptr %history.i46.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.26.0.lcssa, ptr %history.i46.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.29.0.lcssa, ptr %history.i46.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.32.0.lcssa, ptr %history.i46.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.35.0.lcssa, ptr %history.i46.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.38.0.lcssa, ptr %history.i46.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store i32 %storemerge.i1856.lcssa1886418963.lcssa23575, ptr %_22.i269.i, align 4
  store <4 x float> %.lcssa1889418999.lcssa23595, ptr %1705, align 16
  store i32 %storemerge.i1822.lcssa1892119035.lcssa23614, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1895119071.lcssa23634, ptr %1715, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i2072, i32 noundef %_5.i2073, i32 noundef range(i32 0, 536870912) %_58.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !13398, !noalias !13399
  unreachable, !dbg !13398

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2062: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i
  %2082 = bitcast <4 x i32> %2081 to <4 x float>, !dbg !13403
  %2083 = fsub <4 x float> splat (float 1.000000e+00), %2082, !dbg !13407
  %_15.i2060 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i32 %base.i2072, !dbg !13408
  %lanes.i5497.sroa.0.0.copyload = load <4 x i32>, ptr %_15.i2060, align 4, !dbg !13410, !alias.scope !13415, !noalias !13419
  store <4 x i32> %lanes.i5898.sroa.0.0.copyload, ptr %_15.i2060, align 4, !dbg !13423, !alias.scope !13429, !noalias !13433
  %2084 = bitcast <4 x i32> %lanes.i5497.sroa.0.0.copyload to <4 x float>, !dbg !13439
  %2085 = fmul <4 x float> %2083, %2084, !dbg !13443
  %2086 = bitcast <4 x i32> %lanes.i5497.sroa.0.0.copyload to <16 x i8>, !dbg !13444
  %2087 = bitcast <4 x float> %2085 to <16 x i8>, !dbg !13448
  %_4.i7635 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %2086, <16 x i8> %2087, <16 x i8> %1710), !dbg !13449
  store <16 x i8> %_4.i7635, ptr %data.i5.i.i.i.i.i7586, align 4, !dbg !13450, !alias.scope !13455, !noalias !13459
  %exitcond21006.not = icmp eq i32 %1983, %1980, !dbg !12618
  br i1 %exitcond21006.not, label %bb74.i, label %bb75.i, !dbg !12618

bb74.i:                                           ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2062, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7568
  %.lcssa1895119071 = phi <4 x float> [ %.lcssa1895119072, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7568 ], [ %2070, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2062 ]
  %storemerge.i1822.lcssa1892119035 = phi i32 [ %storemerge.i1822.lcssa1892119036, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7568 ], [ %storemerge.i1822, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2062 ]
  %.lcssa1889418999 = phi <4 x float> [ %.lcssa1889419000, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7568 ], [ %2021, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2062 ]
  %storemerge.i1856.lcssa1886418963 = phi i32 [ %storemerge.i1856.lcssa1886418964, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit7568 ], [ %storemerge.i1856, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit2062 ]
  %_143.i = add i32 %run.sroa.0.0.i2487, %ring_cursor.sroa.0.1.i18957, !dbg !13463
  %_241.not.i = icmp ult i32 %_143.i, %ring.i, !dbg !13464
  %2088 = select i1 %_241.not.i, i32 0, i32 %ring.i, !dbg !13464
  %ring_cursor.sroa.0.2.i = sub nuw i32 %_143.i, %2088, !dbg !13464
  %_145.i = add i32 %run.sroa.0.0.i2487, %main_cursor.sroa.0.1.i18958, !dbg !13467
  %_252.not.i = icmp ult i32 %_145.i, %main.i, !dbg !13468
  %2089 = select i1 %_252.not.i, i32 0, i32 %main.i, !dbg !13468
  %main_cursor.sroa.0.2.i = sub nuw i32 %_145.i, %2089, !dbg !13468
  %_63.i = icmp ult i32 %_87.i, %spec.store.select.i, !dbg !12516
  br i1 %_63.i, label %bb20.i, label %bb15.i.loopexit, !dbg !12516

_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit: ; preds = %bb15.i.loopexit
  store <4 x i32> %history.i46.i.sroa.0.0.lcssa, ptr %hot_left.i, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.7.0.lcssa, ptr %history.i46.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.10.0.lcssa, ptr %history.i46.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.13.0.lcssa, ptr %history.i46.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.16.0.lcssa, ptr %history.i46.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.19.0.lcssa, ptr %history.i46.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.22.0.lcssa, ptr %history.i46.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.26.0.lcssa, ptr %history.i46.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.29.0.lcssa, ptr %history.i46.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.32.0.lcssa, ptr %history.i46.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.35.0.lcssa, ptr %history.i46.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i46.i.sroa.38.0.lcssa, ptr %history.i46.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !11566
  store <4 x i32> %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store <4 x i32> %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 16, !dbg !11568
  store i32 %storemerge.i1856.lcssa1886418963.lcssa23574, ptr %_22.i269.i, align 4
  store <4 x float> %.lcssa1889418999.lcssa23594, ptr %1705, align 16
  store i32 %storemerge.i1822.lcssa1892119035.lcssa23613, ptr %_22.i.i, align 4
  store <4 x float> %.lcssa1895119071.lcssa23633, ptr %1715, align 16
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !13470

_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit, %bb6.i
  %ring_cursor.sroa.0.0.i.lcssa = phi i32 [ %_37.i, %bb6.i ], [ %ring_cursor.sroa.0.1.i.lcssa, %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit ], !dbg !11508
  %main_cursor.sroa.0.0.i.lcssa = phi i32 [ %_36.i, %bb6.i ], [ %main_cursor.sroa.0.1.i.lcssa, %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit ], !dbg !11505
  %left_prefix.i = load <4 x i32>, ptr %uniform_left.i, align 16, !dbg !13470, !noalias !11512
  %2090 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 16, !dbg !13471
  %left_phase.i = load i32, ptr %2090, align 16, !dbg !13471, !noalias !11512, !noundef !10
  %right_prefix.i = load <4 x i32>, ptr %uniform_right.i, align 16, !dbg !13472, !noalias !11512
  %2091 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 16, !dbg !13473
  %right_phase.i = load i32, ptr %2091, align 16, !dbg !13473, !noalias !11512, !noundef !10
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_right.i), !dbg !13474, !noalias !11512
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i), !dbg !13475, !noalias !11512
  %2092 = getelementptr inbounds nuw i8, ptr %self, i32 968, !dbg !13476
  %_265.1.i = load i32, ptr %2092, align 4, !dbg !13476, !alias.scope !11479, !noalias !13478, !noundef !10
  %_8.i6559 = icmp samesign ugt i32 %_265.1.i, 3, !dbg !13479
  br i1 %_8.i6559, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6562, label %bb2.i6560, !dbg !13479, !prof !1039

bb2.i6560:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_265.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !13484, !noalias !13485
  unreachable, !dbg !13484

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6562: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
  %2093 = getelementptr inbounds nuw i8, ptr %self, i32 964, !dbg !13476
  %_265.0.i = load ptr, ptr %2093, align 4, !dbg !13476, !alias.scope !11479, !noalias !13478, !nonnull !10, !noundef !10
  store <4 x i32> %left_prefix.i, ptr %_265.0.i, align 4, !dbg !13489, !alias.scope !13493, !noalias !13497
  %_266.0.i = load ptr, ptr %68, align 4, !dbg !13499, !alias.scope !11479, !noalias !13478, !nonnull !10, !noundef !10
  %_266.1.i = load i32, ptr %69, align 4, !dbg !13499, !alias.scope !11479, !noalias !13478, !noundef !10
  %2094 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i), !dbg !13500
  br i1 %2094, label %bb2.i7646, label %bb6.i7638, !dbg !13500

bb6.i7638:                                        ; preds = %bb2.i7646, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6562
  %end_or_len.idx.i7639 = shl nuw nsw i32 %_266.1.i, 2, !dbg !13504
  %end_or_len.i7640 = getelementptr inbounds nuw i8, ptr %_266.0.i, i32 %end_or_len.idx.i7639, !dbg !13504
  %_293.i7641 = icmp eq i32 %_266.1.i, 0, !dbg !13508
  br i1 %_293.i7641, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7652, label %bb10.i7642, !dbg !13511

bb2.i7646:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6562
  %bytes1.sroa.0.0.zext.i7647 = and i32 %left_phase.i, 255, !dbg !13512
  %bytes1.sroa.0.0.isplat.i7648 = mul nuw i32 %bytes1.sroa.0.0.zext.i7647, 16843009, !dbg !13512
  %_5.i7649 = icmp eq i32 %left_phase.i, %bytes1.sroa.0.0.isplat.i7648, !dbg !13513
  br i1 %_5.i7649, label %bb3.i7650, label %bb6.i7638, !dbg !13513

bb3.i7650:                                        ; preds = %bb2.i7646
  %bytes.sroa.0.0.extract.trunc.i7651 = trunc i32 %left_phase.i to i8, !dbg !13514
  %2095 = shl nuw nsw i32 %_266.1.i, 2, !dbg !13516
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %_266.0.i, i8 %bytes.sroa.0.0.extract.trunc.i7651, i32 %2095, i1 false), !dbg !13516, !alias.scope !13517, !noalias !11483
  br label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7652, !dbg !13520

bb10.i7642:                                       ; preds = %bb6.i7638, %bb10.i7642
  %iter.sroa.0.04.i7643 = phi ptr [ %_38.i7644, %bb10.i7642 ], [ %_266.0.i, %bb6.i7638 ]
  %_38.i7644 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i7643, i32 4, !dbg !13521
  store i32 %left_phase.i, ptr %iter.sroa.0.04.i7643, align 4, !dbg !13523, !alias.scope !13517, !noalias !11483
  %_29.i7645 = icmp eq ptr %_38.i7644, %end_or_len.i7640, !dbg !13508
  br i1 %_29.i7645, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7652, label %bb10.i7642, !dbg !13511

_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7652: ; preds = %bb10.i7642, %bb6.i7638, %bb3.i7650
  %2096 = getelementptr inbounds nuw i8, ptr %self, i32 1068, !dbg !13524
  %_267.1.i = load i32, ptr %2096, align 4, !dbg !13524, !alias.scope !11481, !noalias !13525, !noundef !10
  %_8.i6554 = icmp samesign ugt i32 %_267.1.i, 3, !dbg !13526
  br i1 %_8.i6554, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6557, label %bb2.i6555, !dbg !13526, !prof !1039

bb2.i6555:                                        ; preds = %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7652
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_267.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !13531, !noalias !13532
  unreachable, !dbg !13531

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6557: ; preds = %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7652
  %2097 = getelementptr inbounds nuw i8, ptr %self, i32 1064, !dbg !13524
  %_267.0.i = load ptr, ptr %2097, align 4, !dbg !13524, !alias.scope !11481, !noalias !13525, !nonnull !10, !noundef !10
  store <4 x i32> %right_prefix.i, ptr %_267.0.i, align 4, !dbg !13536, !alias.scope !13540, !noalias !13544
  %_268.0.i = load ptr, ptr %77, align 4, !dbg !13546, !alias.scope !11481, !noalias !13525, !nonnull !10, !noundef !10
  %_268.1.i = load i32, ptr %78, align 4, !dbg !13546, !alias.scope !11481, !noalias !13525, !noundef !10
  %2098 = tail call i1 @llvm.is.constant.i32(i32 %right_phase.i), !dbg !13547
  br i1 %2098, label %bb2.i7662, label %bb6.i7654, !dbg !13547

bb6.i7654:                                        ; preds = %bb2.i7662, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6557
  %end_or_len.idx.i7655 = shl nuw nsw i32 %_268.1.i, 2, !dbg !13550
  %end_or_len.i7656 = getelementptr inbounds nuw i8, ptr %_268.0.i, i32 %end_or_len.idx.i7655, !dbg !13550
  %_293.i7657 = icmp eq i32 %_268.1.i, 0, !dbg !13554
  br i1 %_293.i7657, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7668, label %bb10.i7658, !dbg !13557

bb2.i7662:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit6557
  %bytes1.sroa.0.0.zext.i7663 = and i32 %right_phase.i, 255, !dbg !13558
  %bytes1.sroa.0.0.isplat.i7664 = mul nuw i32 %bytes1.sroa.0.0.zext.i7663, 16843009, !dbg !13558
  %_5.i7665 = icmp eq i32 %right_phase.i, %bytes1.sroa.0.0.isplat.i7664, !dbg !13559
  br i1 %_5.i7665, label %bb3.i7666, label %bb6.i7654, !dbg !13559

bb3.i7666:                                        ; preds = %bb2.i7662
  %bytes.sroa.0.0.extract.trunc.i7667 = trunc i32 %right_phase.i to i8, !dbg !13560
  %2099 = shl nuw nsw i32 %_268.1.i, 2, !dbg !13562
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %_268.0.i, i8 %bytes.sroa.0.0.extract.trunc.i7667, i32 %2099, i1 false), !dbg !13562, !alias.scope !13563, !noalias !11483
  br label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7668, !dbg !13566

bb10.i7658:                                       ; preds = %bb6.i7654, %bb10.i7658
  %iter.sroa.0.04.i7659 = phi ptr [ %_38.i7660, %bb10.i7658 ], [ %_268.0.i, %bb6.i7654 ]
  %_38.i7660 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i7659, i32 4, !dbg !13567
  store i32 %right_phase.i, ptr %iter.sroa.0.04.i7659, align 4, !dbg !13569, !alias.scope !13563, !noalias !11483
  %_29.i7661 = icmp eq ptr %_38.i7660, %end_or_len.i7656, !dbg !13554
  br i1 %_29.i7661, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7668, label %bb10.i7658, !dbg !13557

_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7668: ; preds = %bb10.i7658, %bb6.i7654, %bb3.i7666
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_(ptr noalias noundef readonly align 16 captures(none) dereferenceable(368) %hot_left.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33) #31, !dbg !13570
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_(ptr noalias noundef readonly align 16 captures(none) dereferenceable(368) %hot_right.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34) #31, !dbg !13571
  store i32 %main_cursor.sroa.0.0.i.lcssa, ptr %_35, align 4, !dbg !13572, !alias.scope !11483, !noalias !11507
  store i32 %ring_cursor.sroa.0.0.i.lcssa, ptr %86, align 4, !dbg !13573, !alias.scope !11483, !noalias !11507
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i), !dbg !13574, !noalias !11512
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i), !dbg !13575, !noalias !11512
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !11476

_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7415, %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit7668
  br i1 %quiet.sroa.0.0.off015846, label %bb28, label %bb40, !dbg !13576

bb22:                                             ; preds = %bb20
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13577), !dbg !13580
  %2100 = getelementptr inbounds nuw i8, ptr %self, i32 980, !dbg !13581
  %_40.0.i = load ptr, ptr %2100, align 4, !dbg !13581, !alias.scope !13577, !nonnull !10, !noundef !10
  %2101 = getelementptr inbounds nuw i8, ptr %self, i32 984, !dbg !13581
  %_40.1.i = load i32, ptr %2101, align 4, !dbg !13581, !alias.scope !13577, !noundef !10
  %2102 = getelementptr inbounds nuw i8, ptr %self, i32 1012, !dbg !13583
  %_41.0.i = load ptr, ptr %2102, align 4, !dbg !13583, !alias.scope !13577, !nonnull !10, !noundef !10
  %2103 = getelementptr inbounds nuw i8, ptr %self, i32 1016, !dbg !13583
  %_41.1.i = load i32, ptr %2103, align 4, !dbg !13583, !alias.scope !13577, !noundef !10
  %spec.store.select.i.i = tail call i32 @llvm.umin.i32(i32 %_41.1.i, i32 %_40.1.i), !dbg !13584
  %_2.i6.not.i = icmp eq i32 %spec.store.select.i.i, 0, !dbg !13590
  br i1 %_2.i6.not.i, label %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb3.i7669, !dbg !13590

bb3.i7669:                                        ; preds = %bb22, %bb5.i7671
  %iter.sroa.8.07.i = phi i32 [ %2104, %bb5.i7671 ], [ 0, %bb22 ]
  %_3.i1.i.i = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i, i32 %iter.sroa.8.07.i, !dbg !13593
  %_14.i7670 = load i32, ptr %_3.i1.i.i, align 4, !dbg !13596, !noalias !13577, !noundef !10
  %_20.i = icmp eq i32 %_14.i7670, 0, !dbg !13597
  br i1 %_20.i, label %panic.i7678, label %bb5.i7671, !dbg !13597

bb5.i7671:                                        ; preds = %bb3.i7669
  %_3.i.i.i7672 = getelementptr inbounds nuw i32, ptr %_40.0.i, i32 %iter.sroa.8.07.i, !dbg !13598
  %2104 = add nuw i32 %iter.sroa.8.07.i, 1, !dbg !13601
  %_18.i7673 = load i32, ptr %_3.i.i.i7672, align 4, !dbg !13602, !noalias !13577, !noundef !10
  %_19.i7674 = urem i32 %frames, %_14.i7670, !dbg !13597
  %_16.i7675 = add i32 %_19.i7674, %_18.i7673, !dbg !13603
  %_15.i7676 = urem i32 %_16.i7675, %_14.i7670, !dbg !13604
  store i32 %_15.i7676, ptr %_3.i.i.i7672, align 4, !dbg !13605, !noalias !13577
  %exitcond.not.i = icmp eq i32 %2104, %spec.store.select.i.i, !dbg !13590
  br i1 %exitcond.not.i, label %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb3.i7669, !dbg !13590

panic.i7678:                                      ; preds = %bb3.i7669
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_f5b0427df9b659e554a697ca46ce8b5a) #32, !dbg !13597, !noalias !13577
  unreachable, !dbg !13597

_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit: ; preds = %bb5.i7671, %bb22
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13606), !dbg !13609
  %2105 = getelementptr inbounds nuw i8, ptr %self, i32 1080, !dbg !13610
  %_40.0.i7679 = load ptr, ptr %2105, align 4, !dbg !13610, !alias.scope !13606, !nonnull !10, !noundef !10
  %2106 = getelementptr inbounds nuw i8, ptr %self, i32 1084, !dbg !13610
  %_40.1.i7680 = load i32, ptr %2106, align 4, !dbg !13610, !alias.scope !13606, !noundef !10
  %2107 = getelementptr inbounds nuw i8, ptr %self, i32 1112, !dbg !13612
  %_41.0.i7681 = load ptr, ptr %2107, align 4, !dbg !13612, !alias.scope !13606, !nonnull !10, !noundef !10
  %2108 = getelementptr inbounds nuw i8, ptr %self, i32 1116, !dbg !13612
  %_41.1.i7682 = load i32, ptr %2108, align 4, !dbg !13612, !alias.scope !13606, !noundef !10
  %spec.store.select.i.i7683 = tail call i32 @llvm.umin.i32(i32 %_41.1.i7682, i32 %_40.1.i7680), !dbg !13613
  %_2.i6.not.i7684 = icmp eq i32 %spec.store.select.i.i7683, 0, !dbg !13619
  br i1 %_2.i6.not.i7684, label %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit7699, label %bb3.i7685, !dbg !13619

bb3.i7685:                                        ; preds = %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, %bb5.i7690
  %iter.sroa.8.07.i7686 = phi i32 [ %2109, %bb5.i7690 ], [ 0, %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit ]
  %_3.i1.i.i7687 = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i7681, i32 %iter.sroa.8.07.i7686, !dbg !13622
  %_14.i7688 = load i32, ptr %_3.i1.i.i7687, align 4, !dbg !13625, !noalias !13606, !noundef !10
  %_20.i7689 = icmp eq i32 %_14.i7688, 0, !dbg !13626
  br i1 %_20.i7689, label %panic.i7698, label %bb5.i7690, !dbg !13626

bb5.i7690:                                        ; preds = %bb3.i7685
  %_3.i.i.i7691 = getelementptr inbounds nuw i32, ptr %_40.0.i7679, i32 %iter.sroa.8.07.i7686, !dbg !13627
  %2109 = add nuw i32 %iter.sroa.8.07.i7686, 1, !dbg !13630
  %_18.i7692 = load i32, ptr %_3.i.i.i7691, align 4, !dbg !13631, !noalias !13606, !noundef !10
  %_19.i7693 = urem i32 %frames, %_14.i7688, !dbg !13626
  %_16.i7694 = add i32 %_19.i7693, %_18.i7692, !dbg !13632
  %_15.i7695 = urem i32 %_16.i7694, %_14.i7688, !dbg !13633
  store i32 %_15.i7695, ptr %_3.i.i.i7691, align 4, !dbg !13634, !noalias !13606
  %exitcond.not.i7696 = icmp eq i32 %2109, %spec.store.select.i.i7683, !dbg !13619
  br i1 %exitcond.not.i7696, label %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit7699, label %bb3.i7685, !dbg !13619

panic.i7698:                                      ; preds = %bb3.i7685
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_f5b0427df9b659e554a697ca46ce8b5a) #32, !dbg !13626, !noalias !13606
  unreachable, !dbg !13626

_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit7699: ; preds = %bb5.i7690, %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit
  %2110 = getelementptr inbounds nuw i8, ptr %self, i32 916, !dbg !13635
  %_29.val = load i32, ptr %2110, align 4, !dbg !13635
  %2111 = getelementptr inbounds nuw i8, ptr %self, i32 920, !dbg !13635
  %_29.val6813 = load i32, ptr %2111, align 4, !dbg !13635, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13636), !dbg !13635
  %_10.i7700 = icmp eq i32 %_29.val6813, 0, !dbg !13639
  br i1 %_10.i7700, label %panic.i7710, label %bb1.i7701, !dbg !13639

bb1.i7701:                                        ; preds = %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit7699
  %_28 = getelementptr inbounds nuw i8, ptr %self, i32 800, !dbg !13643
  %_7.i = load i32, ptr %_28, align 4, !dbg !13644, !alias.scope !13636, !noundef !10
  %_8.i7702 = urem i32 %frames, %_29.val6813, !dbg !13639
  %_5.i7703 = add i32 %_8.i7702, %_7.i, !dbg !13645
  %_4.i7704 = urem i32 %_5.i7703, %_29.val6813, !dbg !13646
  store i32 %_4.i7704, ptr %_28, align 4, !dbg !13647, !alias.scope !13636
  %_17.i7705 = icmp eq i32 %_29.val, 0, !dbg !13648
  br i1 %_17.i7705, label %panic2.i, label %_RNvMs1_CsjLJhryqjeDL_17true_peak_limiterNtB5_7Cursors7advance.exit, !dbg !13648

panic.i7710:                                      ; preds = %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit7699
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c7e64e9e8489bc8e648659d0098d136c) #32, !dbg !13639, !noalias !13636
  unreachable, !dbg !13639

panic2.i:                                         ; preds = %bb1.i7701
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_34b23597fad9d85cb3bee6d64ebf77d5) #32, !dbg !13648, !noalias !13636
  unreachable, !dbg !13648

_RNvMs1_CsjLJhryqjeDL_17true_peak_limiterNtB5_7Cursors7advance.exit: ; preds = %bb1.i7701
  %2112 = getelementptr inbounds nuw i8, ptr %self, i32 804, !dbg !13649
  %_14.i7707 = load i32, ptr %2112, align 4, !dbg !13649, !alias.scope !13636, !noundef !10
  %_15.i7708 = urem i32 %frames, %_29.val, !dbg !13648
  %_12.i = add i32 %_15.i7708, %_14.i7707, !dbg !13650
  %_11.i7709 = urem i32 %_12.i, %_29.val, !dbg !13651
  store i32 %_11.i7709, ptr %2112, align 4, !dbg !13652, !alias.scope !13636
  br label %bb42, !dbg !13653

bb28:                                             ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_37 = tail call noundef zeroext i1 @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #31, !dbg !13654
  br i1 %_37, label %bb30, label %bb40, !dbg !13655

bb30:                                             ; preds = %bb28
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_39 = tail call noundef zeroext i1 @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #31, !dbg !13656
  br i1 %_39, label %bb32, label %bb40, !dbg !13657

bb32:                                             ; preds = %bb30
  %_80.not = icmp ugt i32 %words, %left_io.1
  br i1 %_80.not, label %bb51, label %bb1.i7711, !dbg !13658, !prof !4596

bb51:                                             ; preds = %bb32
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef %words, i32 noundef %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_fd9a647e86e53fac40c0d38dd38d80a3) #32, !dbg !13666
  unreachable, !dbg !13666

bb1.i7711:                                        ; preds = %bb32, %bb12.i7725
  %io.sroa.5.0.i7712 = phi i32 [ %len.i.i.i7718, %bb12.i7725 ], [ %words, %bb32 ]
  %io.sroa.0.0.i7713 = phi ptr [ %data.i.i.i7717, %bb12.i7725 ], [ %left_io.0, %bb32 ]
  %2113 = icmp eq i32 %io.sroa.5.0.i7712, 0, !dbg !13667
  br i1 %2113, label %bb34, label %bb13.preheader.i7714, !dbg !13667

bb13.preheader.i7714:                             ; preds = %bb1.i7711
  %spec.store.select.i7715 = tail call i32 @llvm.umin.i32(i32 %io.sroa.5.0.i7712, i32 32), !dbg !13670
  %data.i.i.idx.i7716 = shl nuw nsw i32 %spec.store.select.i7715, 2, !dbg !13673
  %data.i.i.i7717 = getelementptr inbounds nuw i8, ptr %io.sroa.0.0.i7713, i32 %data.i.i.idx.i7716, !dbg !13673
  br label %bb13.i7719, !dbg !13678

bb13.i7719:                                       ; preds = %bb13.i7719, %bb13.preheader.i7714
  %iter.sroa.0.08.i7720 = phi ptr [ %_35.i7722, %bb13.i7719 ], [ %io.sroa.0.0.i7713, %bb13.preheader.i7714 ]
  %bits.sroa.0.07.i7721 = phi i32 [ %2114, %bb13.i7719 ], [ 0, %bb13.preheader.i7714 ]
  %_35.i7722 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i7720, i32 4, !dbg !13680
  %_95.i7723 = load i32, ptr %iter.sroa.0.08.i7720, align 4, !dbg !13682, !alias.scope !13683, !noundef !10
  %2114 = or i32 %_95.i7723, %bits.sroa.0.07.i7721, !dbg !13686
  %_29.i7724 = icmp eq ptr %_35.i7722, %data.i.i.i7717, !dbg !13687
  br i1 %_29.i7724, label %bb12.i7725, label %bb13.i7719, !dbg !13678

bb12.i7725:                                       ; preds = %bb13.i7719
  %len.i.i.i7718 = sub nuw nsw i32 %io.sroa.5.0.i7712, %spec.store.select.i7715, !dbg !13689
  %2115 = icmp eq i32 %2114, 0, !dbg !13690
  br i1 %2115, label %bb1.i7711, label %bb40, !dbg !13690

bb34:                                             ; preds = %bb1.i7711
  %_88.not = icmp ugt i32 %words, %right_io.1, !dbg !13691
  br i1 %_88.not, label %bb54, label %bb1.i7739, !dbg !13691, !prof !787

bb40:                                             ; preds = %bb12.i7725, %bb12.i7753, %bb1.i7739, %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, %bb28, %bb30
  %_36.sroa.0.0.off0 = phi i1 [ false, %bb12.i7753 ], [ false, %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit ], [ false, %bb30 ], [ false, %bb28 ], [ true, %bb1.i7739 ], [ false, %bb12.i7725 ]
  %2116 = zext i1 %_36.sroa.0.0.off0 to i8, !dbg !13697
  store i8 %2116, ptr %38, align 4, !dbg !13697
  %2117 = load i8, ptr %2, align 8, !dbg !13698, !range !4667, !noundef !10
  store i8 %2117, ptr %0, align 1, !dbg !13699
  call void @llvm.lifetime.start.p0(ptr nonnull %shape), !dbg !13700
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(12) %shape, ptr noundef nonnull align 16 dereferenceable(12) %_32, i32 12, i1 false), !dbg !13701
  %2118 = getelementptr inbounds nuw i8, ptr %self, i32 880, !dbg !13702
  %2119 = load i32, ptr %2118, align 8, !dbg !13702, !noundef !10
  %2120 = getelementptr inbounds nuw i8, ptr %self, i32 808, !dbg !13704
  %_99.0 = load ptr, ptr %2120, align 8, !dbg !13704, !nonnull !10, !noundef !10
  %2121 = getelementptr inbounds nuw i8, ptr %self, i32 812, !dbg !13704
  %_99.1 = load i32, ptr %2121, align 4, !dbg !13704, !noundef !10
  %2122 = getelementptr inbounds nuw i8, ptr %self, i32 816, !dbg !13704
  %_100.0 = load ptr, ptr %2122, align 16, !dbg !13704, !nonnull !10, !noundef !10
  %2123 = getelementptr inbounds nuw i8, ptr %self, i32 820, !dbg !13704
  %_100.1 = load i32, ptr %2123, align 4, !dbg !13704, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13711), !dbg !13714
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13715), !dbg !13714
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13717), !dbg !13714
  %fst_len.i.i = and i32 %left_io.1, 536870908, !dbg !13719
  %_22.not.i14201.i = icmp eq i32 %fst_len.i.i, 0, !dbg !13732
  br i1 %_22.not.i14201.i, label %bb2.i7734, label %bb13.i15.i, !dbg !13732

bb13.i15.i:                                       ; preds = %bb40, %bb13.i15.i
  %iter.sroa.0.0.i13204.i = phi ptr [ %_27.i16.i, %bb13.i15.i ], [ %left_io.0, %bb40 ]
  %iter.sroa.5.0.i12203.i = phi i32 [ %_28.i17.i, %bb13.i15.i ], [ %fst_len.i.i, %bb40 ]
  %ok.i2.sroa.0.0202.i = phi <4 x i32> [ %2126, %bb13.i15.i ], [ splat (i32 -1), %bb40 ]
  %_27.i16.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i13204.i, i32 16, !dbg !13739
  %_28.i17.i = add i32 %iter.sroa.5.0.i12203.i, -4, !dbg !13746
  %lanes.i.sroa.0.0.copyload.i = load <4 x float>, ptr %iter.sroa.0.0.i13204.i, align 4, !dbg !13747, !alias.scope !13753, !noalias !13757
  %2124 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %lanes.i.sroa.0.0.copyload.i), !dbg !13762
  %2125 = fcmp olt <4 x float> %2124, splat (float 0x46293E5940000000), !dbg !13767
  %2126 = select <4 x i1> %2125, <4 x i32> %ok.i2.sroa.0.0202.i, <4 x i32> zeroinitializer, !dbg !13772
  %_22.not.i14.i = icmp eq i32 %_28.i17.i, 0, !dbg !13732
  br i1 %_22.not.i14.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit25.i, label %bb13.i15.i, !dbg !13732

_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit25.i: ; preds = %bb13.i15.i
  %2127 = bitcast <4 x i32> %2126 to <16 x i8>, !dbg !13779
  %2128 = xor <16 x i8> %2127, splat (i8 -1), !dbg !13779
  %2129 = tail call i32 @llvm.wasm.anytrue.v16i8(<16 x i8> %2128), !dbg !13786
  %2130 = icmp eq i32 %2129, 0, !dbg !13786
  br i1 %2130, label %bb2.i7734, label %bb19.i.preheader.i, !dbg !13787

bb2.i7734:                                        ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit25.i, %bb40
  %fst_len.i89.i = and i32 %right_io.1, 536870908, !dbg !13788
  %_22.not.i205.i = icmp eq i32 %fst_len.i89.i, 0, !dbg !13792
  br i1 %_22.not.i205.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB1z_11LimiterCoreBR_E13process_block0EB1z_.exit, label %bb13.i.i7735, !dbg !13792

bb13.i.i7735:                                     ; preds = %bb2.i7734, %bb13.i.i7735
  %iter.sroa.0.0.i208.i = phi ptr [ %_27.i.i7736, %bb13.i.i7735 ], [ %right_io.0, %bb2.i7734 ]
  %iter.sroa.5.0.i207.i = phi i32 [ %_28.i.i7737, %bb13.i.i7735 ], [ %fst_len.i89.i, %bb2.i7734 ]
  %ok.i.sroa.0.0206.i = phi <4 x i32> [ %2133, %bb13.i.i7735 ], [ splat (i32 -1), %bb2.i7734 ]
  %_27.i.i7736 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i208.i, i32 16, !dbg !13795
  %_28.i.i7737 = add i32 %iter.sroa.5.0.i207.i, -4, !dbg !13798
  %lanes.i41.sroa.0.0.copyload.i = load <4 x float>, ptr %iter.sroa.0.0.i208.i, align 4, !dbg !13799, !alias.scope !13804, !noalias !13808
  %2131 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %lanes.i41.sroa.0.0.copyload.i), !dbg !13812
  %2132 = fcmp olt <4 x float> %2131, splat (float 0x46293E5940000000), !dbg !13816
  %2133 = select <4 x i1> %2132, <4 x i32> %ok.i.sroa.0.0206.i, <4 x i32> zeroinitializer, !dbg !13821
  %_22.not.i.i = icmp eq i32 %_28.i.i7737, 0, !dbg !13792
  br i1 %_22.not.i.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i, label %bb13.i.i7735, !dbg !13792

_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i: ; preds = %bb13.i.i7735
  %2134 = bitcast <4 x i32> %2133 to <16 x i8>, !dbg !13825
  %2135 = xor <16 x i8> %2134, splat (i8 -1), !dbg !13825
  %2136 = tail call i32 @llvm.wasm.anytrue.v16i8(<16 x i8> %2135), !dbg !13829
  %2137 = icmp eq i32 %2136, 0, !dbg !13829
  br i1 %2137, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB1z_11LimiterCoreBR_E13process_block0EB1z_.exit, label %bb7.i7738, !dbg !13830

bb7.i7738:                                        ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i
  br i1 %_22.not.i14201.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i, label %bb19.i.preheader.i, !dbg !13831

bb19.i.preheader.i:                               ; preds = %bb7.i7738, %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit25.i
  br label %bb19.i.i, !dbg !13831

bb20.loopexit.i.i:                                ; preds = %bb19.i.i
  %2138 = bitcast <4 x i32> %2141 to <16 x i8>, !dbg !13842
  %.pre = and i32 %right_io.1, 536870908, !dbg !13847
  br label %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i, !dbg !13851

bb19.i.i:                                         ; preds = %bb19.i.i, %bb19.i.preheader.i
  %iter.sroa.0.089.i.i = phi ptr [ %_42.i.i7728, %bb19.i.i ], [ %left_io.0, %bb19.i.preheader.i ]
  %iter.sroa.5.088.i.i = phi i32 [ %_43.i.i7729, %bb19.i.i ], [ %fst_len.i.i, %bb19.i.preheader.i ]
  %ok.sroa.0.087.i.i = phi <4 x i32> [ %2141, %bb19.i.i ], [ splat (i32 -1), %bb19.i.preheader.i ]
  %_42.i.i7728 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.089.i.i, i32 16, !dbg !13852
  %_43.i.i7729 = add i32 %iter.sroa.5.088.i.i, -4, !dbg !13859
  %lanes.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %iter.sroa.0.089.i.i, align 4, !dbg !13860, !alias.scope !13866, !noalias !13872
  %2139 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %lanes.i.sroa.0.0.copyload.i.i), !dbg !13876
  %2140 = fcmp olt <4 x float> %2139, splat (float 0x46293E5940000000), !dbg !13881
  %2141 = select <4 x i1> %2140, <4 x i32> %ok.sroa.0.087.i.i, <4 x i32> zeroinitializer, !dbg !13886
  %_37.not.i.i = icmp eq i32 %_43.i.i7729, 0, !dbg !13831
  br i1 %_37.not.i.i, label %bb20.loopexit.i.i, label %bb19.i.i, !dbg !13831

_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i: ; preds = %bb20.loopexit.i.i, %bb7.i7738
  %fst_len.i.i93.i.pre-phi = phi i32 [ %.pre, %bb20.loopexit.i.i ], [ %fst_len.i89.i, %bb7.i7738 ], !dbg !13847
  %ok.sroa.0.0.lcssa.i.i = phi <16 x i8> [ %2138, %bb20.loopexit.i.i ], [ splat (i8 -1), %bb7.i7738 ], !dbg !13890
  %_4.i39.i.i = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> zeroinitializer, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %ok.sroa.0.0.lcssa.i.i), !dbg !13891
  %2142 = bitcast <16 x i8> %_4.i39.i.i to <4 x i32>, !dbg !13892
  %words.sroa.0.0.vec.extract.i.i = extractelement <4 x i32> %2142, i64 0, !dbg !13899
  %2143 = icmp ne i32 %words.sroa.0.0.vec.extract.i.i, 0, !dbg !13899
  %2144 = zext i1 %2143 to i32, !dbg !13899
  %words.sroa.0.4.vec.extract.i.i = extractelement <4 x i32> %2142, i64 1, !dbg !13899
  %2145 = icmp eq i32 %words.sroa.0.4.vec.extract.i.i, 0, !dbg !13899
  %2146 = select i1 %2145, i32 0, i32 2, !dbg !13899
  %words.sroa.0.8.vec.extract.i.i = extractelement <4 x i32> %2142, i64 2, !dbg !13899
  %2147 = icmp eq i32 %words.sroa.0.8.vec.extract.i.i, 0, !dbg !13899
  %2148 = select i1 %2147, i32 0, i32 4, !dbg !13899
  %words.sroa.0.12.vec.extract.i.i = extractelement <4 x i32> %2142, i64 3, !dbg !13899
  %2149 = icmp eq i32 %words.sroa.0.12.vec.extract.i.i, 0, !dbg !13899
  %2150 = select i1 %2149, i32 0, i32 8, !dbg !13899
  %_37.not86.i94.i = icmp eq i32 %fst_len.i.i93.i.pre-phi, 0, !dbg !13903
  br i1 %_37.not86.i94.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit113.i, label %bb19.i95.i, !dbg !13903

bb20.loopexit.i103.i:                             ; preds = %bb19.i95.i
  %2151 = bitcast <4 x i32> %2154 to <16 x i8>, !dbg !13906
  br label %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit113.i, !dbg !13910

bb19.i95.i:                                       ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i, %bb19.i95.i
  %iter.sroa.0.089.i96.i = phi ptr [ %_42.i99.i, %bb19.i95.i ], [ %right_io.0, %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i ]
  %iter.sroa.5.088.i97.i = phi i32 [ %_43.i100.i, %bb19.i95.i ], [ %fst_len.i.i93.i.pre-phi, %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i ]
  %ok.sroa.0.087.i98.i = phi <4 x i32> [ %2154, %bb19.i95.i ], [ splat (i32 -1), %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i ]
  %_42.i99.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.089.i96.i, i32 16, !dbg !13911
  %_43.i100.i = add i32 %iter.sroa.5.088.i97.i, -4, !dbg !13914
  %lanes.i.sroa.0.0.copyload.i101.i = load <4 x float>, ptr %iter.sroa.0.089.i96.i, align 4, !dbg !13915, !alias.scope !13920, !noalias !13926
  %2152 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %lanes.i.sroa.0.0.copyload.i101.i), !dbg !13930
  %2153 = fcmp olt <4 x float> %2152, splat (float 0x46293E5940000000), !dbg !13934
  %2154 = select <4 x i1> %2153, <4 x i32> %ok.sroa.0.087.i98.i, <4 x i32> zeroinitializer, !dbg !13939
  %_37.not.i102.i = icmp eq i32 %_43.i100.i, 0, !dbg !13903
  br i1 %_37.not.i102.i, label %bb20.loopexit.i103.i, label %bb19.i95.i, !dbg !13903

_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit113.i: ; preds = %bb20.loopexit.i103.i, %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i
  %ok.sroa.0.0.lcssa.i104.i = phi <16 x i8> [ splat (i8 -1), %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i ], [ %2151, %bb20.loopexit.i103.i ], !dbg !13943
  %_4.i39.i105.i = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> zeroinitializer, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %ok.sroa.0.0.lcssa.i104.i), !dbg !13944
  %2155 = bitcast <16 x i8> %_4.i39.i105.i to <4 x i32>, !dbg !13945
  %words.sroa.0.0.vec.extract.i106.i = extractelement <4 x i32> %2155, i64 0, !dbg !13950
  %2156 = icmp ne i32 %words.sroa.0.0.vec.extract.i106.i, 0, !dbg !13950
  %2157 = zext i1 %2156 to i32, !dbg !13950
  %words.sroa.0.4.vec.extract.i107.i = extractelement <4 x i32> %2155, i64 1, !dbg !13950
  %2158 = icmp eq i32 %words.sroa.0.4.vec.extract.i107.i, 0, !dbg !13950
  %2159 = select i1 %2158, i32 0, i32 2, !dbg !13950
  %words.sroa.0.8.vec.extract.i109.i = extractelement <4 x i32> %2155, i64 2, !dbg !13950
  %2160 = icmp eq i32 %words.sroa.0.8.vec.extract.i109.i, 0, !dbg !13950
  %2161 = select i1 %2160, i32 0, i32 4, !dbg !13950
  %words.sroa.0.12.vec.extract.i111.i = extractelement <4 x i32> %2155, i64 3, !dbg !13950
  %2162 = icmp eq i32 %words.sroa.0.12.vec.extract.i111.i, 0, !dbg !13950
  %2163 = select i1 %2162, i32 0, i32 8, !dbg !13950
  %2164 = getelementptr inbounds nuw i8, ptr %self, i32 8, !dbg !13951
  %mask.sroa.0.1.1.i108.i = or disjoint i32 %2146, %2144, !dbg !13950
  %mask.sroa.0.1.2.i110.i = or disjoint i32 %mask.sroa.0.1.1.i108.i, %2148, !dbg !13950
  %mask.sroa.0.1.3.i112.i = or disjoint i32 %mask.sroa.0.1.2.i110.i, %2150, !dbg !13950
  %mask.sroa.0.1.1.i.i = or i32 %mask.sroa.0.1.3.i112.i, %2157, !dbg !13899
  %mask.sroa.0.1.2.i.i = or i32 %mask.sroa.0.1.1.i.i, %2159, !dbg !13899
  %mask.sroa.0.1.3.i.i = or i32 %mask.sroa.0.1.2.i.i, %2161, !dbg !13899
  %2165 = or i32 %mask.sroa.0.1.3.i.i, %2163, !dbg !13951
  store i32 %2165, ptr %2164, align 8, !dbg !13951, !alias.scope !13717, !noalias !13952
  %_14.i7730 = load i64, ptr %self, align 8, !dbg !13953, !alias.scope !13717, !noalias !13952, !noundef !10
  %2166 = tail call i64 @llvm.uadd.sat.i64(i64 %_14.i7730, i64 1), !dbg !13954
  store i64 %2166, ptr %self, align 8, !dbg !13957, !alias.scope !13717, !noalias !13952
  %_222.i.i = icmp eq i32 %left_io.1, 0, !dbg !13958
  br i1 %_222.i.i, label %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit.i, label %bb12.i.preheader.i, !dbg !13964

bb12.i.preheader.i:                               ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit113.i
  %.idx.i.i = shl nuw nsw i32 %left_io.1, 2, !dbg !13965
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %left_io.0, i8 0, i32 %.idx.i.i, i1 false), !dbg !13969, !alias.scope !13970, !noalias !13973
  br label %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit.i, !dbg !13974

_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit.i: ; preds = %bb12.i.preheader.i, %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit113.i
  %_222.i115.i = icmp eq i32 %right_io.1, 0, !dbg !13980
  br i1 %_222.i115.i, label %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit120.i, label %bb12.i116.preheader.i, !dbg !13983

bb12.i116.preheader.i:                            ; preds = %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit.i
  %.idx.i114.i = shl nuw nsw i32 %right_io.1, 2, !dbg !13974
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %right_io.0, i8 0, i32 %.idx.i114.i, i1 false), !dbg !13984, !alias.scope !13985, !noalias !13988
  br label %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit120.i, !dbg !13989

_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit120.i: ; preds = %bb12.i116.preheader.i, %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit.i
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 4 dereferenceable(100) %_33, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(12) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_99.0, i32 noundef %_99.1, i32 noundef %2119) #31, !dbg !13990, !noalias !13995
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 4 dereferenceable(100) %_34, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(12) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_100.0, i32 noundef %_100.1, i32 noundef %2119) #31, !dbg !13998, !noalias !13995
  store i32 0, ptr %_35, align 4, !dbg !13999, !noalias !13995
  %2167 = getelementptr inbounds nuw i8, ptr %self, i32 804, !dbg !13999
  store i32 0, ptr %2167, align 4, !dbg !13999, !noalias !13995
  br label %_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB1z_11LimiterCoreBR_E13process_block0EB1z_.exit, !dbg !14000

_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB1z_11LimiterCoreBR_E13process_block0EB1z_.exit: ; preds = %bb2.i7734, %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit.i, %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit120.i
  call void @llvm.lifetime.end.p0(ptr nonnull %shape), !dbg !14001
  br label %bb42, !dbg !13653

bb54:                                             ; preds = %bb34
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef %words, i32 noundef %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_35f4c396f0d0e2784e6b286770694d2b) #32, !dbg !14002
  unreachable, !dbg !14002

bb1.i7739:                                        ; preds = %bb34, %bb12.i7753
  %io.sroa.5.0.i7740 = phi i32 [ %len.i.i.i7746, %bb12.i7753 ], [ %words, %bb34 ]
  %io.sroa.0.0.i7741 = phi ptr [ %data.i.i.i7745, %bb12.i7753 ], [ %right_io.0, %bb34 ]
  %2168 = icmp eq i32 %io.sroa.5.0.i7740, 0, !dbg !14003
  br i1 %2168, label %bb40, label %bb13.preheader.i7742, !dbg !14003

bb13.preheader.i7742:                             ; preds = %bb1.i7739
  %spec.store.select.i7743 = tail call i32 @llvm.umin.i32(i32 %io.sroa.5.0.i7740, i32 32), !dbg !14006
  %data.i.i.idx.i7744 = shl nuw nsw i32 %spec.store.select.i7743, 2, !dbg !14009
  %data.i.i.i7745 = getelementptr inbounds nuw i8, ptr %io.sroa.0.0.i7741, i32 %data.i.i.idx.i7744, !dbg !14009
  br label %bb13.i7747, !dbg !14014

bb13.i7747:                                       ; preds = %bb13.i7747, %bb13.preheader.i7742
  %iter.sroa.0.08.i7748 = phi ptr [ %_35.i7750, %bb13.i7747 ], [ %io.sroa.0.0.i7741, %bb13.preheader.i7742 ]
  %bits.sroa.0.07.i7749 = phi i32 [ %2169, %bb13.i7747 ], [ 0, %bb13.preheader.i7742 ]
  %_35.i7750 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i7748, i32 4, !dbg !14016
  %_95.i7751 = load i32, ptr %iter.sroa.0.08.i7748, align 4, !dbg !14018, !alias.scope !14019, !noundef !10
  %2169 = or i32 %_95.i7751, %bits.sroa.0.07.i7749, !dbg !14022
  %_29.i7752 = icmp eq ptr %_35.i7750, %data.i.i.i7745, !dbg !14023
  br i1 %_29.i7752, label %bb12.i7753, label %bb13.i7747, !dbg !14014

bb12.i7753:                                       ; preds = %bb13.i7747
  %len.i.i.i7746 = sub nuw nsw i32 %io.sroa.5.0.i7740, %spec.store.select.i7743, !dbg !14025
  %2170 = icmp eq i32 %2169, 0, !dbg !14026
  br i1 %2170, label %bb1.i7739, label %bb40, !dbg !14026

bb42:                                             ; preds = %_RNvMs1_CsjLJhryqjeDL_17true_peak_limiterNtB5_7Cursors7advance.exit, %_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB1z_11LimiterCoreBR_E13process_block0EB1z_.exit
  ret void, !dbg !13653
}
