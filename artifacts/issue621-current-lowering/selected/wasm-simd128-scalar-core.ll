define internal fastcc void @_RNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB5_11LimiterCorefE13process_blockB5_(ptr noalias noundef nonnull align 8 dereferenceable(544) %self, ptr noalias noundef nonnull align 4 captures(address) %left_io.0, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef nonnull align 4 captures(address) %right_io.0, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef %frames) unnamed_addr #1 !dbg !18694 {
start:
  %_108.i702 = alloca [92 x i8], align 4
  %_106.i703 = alloca [92 x i8], align 4
  %peaks_right.i704 = alloca [1024 x i8], align 4
  %peaks_left.i705 = alloca [1024 x i8], align 4
  %scratch.i706 = alloca [32 x i8], align 4
  %hot_right.i707 = alloca [92 x i8], align 4
  %hot_left.i708 = alloca [92 x i8], align 4
  %_108.i = alloca [92 x i8], align 4
  %_106.i = alloca [92 x i8], align 4
  %peaks_right.i434 = alloca [1024 x i8], align 4
  %peaks_left.i435 = alloca [1024 x i8], align 4
  %scratch.i = alloca [32 x i8], align 4
  %hot_right.i436 = alloca [92 x i8], align 4
  %hot_left.i437 = alloca [92 x i8], align 4
  %_155.i15 = alloca [92 x i8], align 4
  %_153.i16 = alloca [92 x i8], align 4
  %uniform_right.i31 = alloca [44 x i8], align 4
  %uniform_left.i32 = alloca [44 x i8], align 4
  %peaks_right.i33 = alloca [1024 x i8], align 4
  %peaks_left.i34 = alloca [1024 x i8], align 4
  %hot_right.i35 = alloca [92 x i8], align 4
  %hot_left.i36 = alloca [92 x i8], align 4
  %uniform_right.i = alloca [44 x i8], align 4
  %uniform_left.i = alloca [44 x i8], align 4
  %peaks_right.i = alloca [1024 x i8], align 4
  %peaks_left.i = alloca [1024 x i8], align 4
  %hot_right.i = alloca [92 x i8], align 4
  %hot_left.i = alloca [92 x i8], align 4
  %shape = alloca [12 x i8], align 4
  %0 = getelementptr inbounds nuw i8, ptr %self, i32 537, !dbg !18695
  %1 = load i8, ptr %0, align 1, !dbg !18695, !range !4765, !noundef !10
  %2 = getelementptr inbounds nuw i8, ptr %self, i32 80, !dbg !18697
  %3 = load i8, ptr %2, align 8, !dbg !18697, !range !4765, !noundef !10
  %_7 = icmp eq i8 %1, %3, !dbg !18695
  %4 = getelementptr inbounds nuw i8, ptr %self, i32 388
  %_95.0 = load ptr, ptr %4, align 4, !dbg !18698
  %5 = getelementptr inbounds nuw i8, ptr %self, i32 392
  %_95.1 = load i32, ptr %5, align 4, !dbg !18698
  br i1 %_7, label %bb1, label %bb20.thread, !dbg !18695

bb1:                                              ; preds = %start
  %_8.i3844 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_95.0, i32 %_95.1, !dbg !18699
  br label %bb1.i.i3845, !dbg !18704

bb1.i.i3845:                                      ; preds = %bb11.i.i, %bb1
  %_221.i.i = phi ptr [ %_22.i.i3846, %bb11.i.i ], [ %_95.0, %bb1 ]
  %_12.i.i = icmp eq ptr %_221.i.i, %_8.i3844, !dbg !18706
  br i1 %_12.i.i, label %bb3, label %bb11.i.i, !dbg !18709

bb11.i.i:                                         ; preds = %bb1.i.i3845
  %_22.i.i3846 = getelementptr inbounds nuw i8, ptr %_221.i.i, i32 16, !dbg !18710
  %6 = getelementptr inbounds nuw i8, ptr %_221.i.i, i32 12, !dbg !18712
  %_3.i.i.i = load i32, ptr %6, align 4, !dbg !18712, !alias.scope !18714, !noalias !18719, !noundef !10
  %7 = icmp eq i32 %_3.i.i.i, 0, !dbg !18712
  %_51.i.i.i = load i32, ptr %_221.i.i, align 4, !dbg !18712, !alias.scope !18714, !noalias !18719
  %8 = getelementptr inbounds nuw i8, ptr %_221.i.i, i32 4, !dbg !18712
  %_72.i.i.i = load i32, ptr %8, align 4, !dbg !18712, !alias.scope !18714, !noalias !18719
  %9 = icmp eq i32 %_51.i.i.i, %_72.i.i.i, !dbg !18712
  %_0.sroa.0.0.off0.i.i.i = select i1 %7, i1 %9, i1 false, !dbg !18712
  br i1 %_0.sroa.0.0.off0.i.i.i, label %bb1.i.i3845, label %bb20.thread, !dbg !18722

bb3:                                              ; preds = %bb1.i.i3845
  %10 = getelementptr inbounds nuw i8, ptr %self, i32 396, !dbg !18723
  %_96.0 = load ptr, ptr %10, align 4, !dbg !18723, !nonnull !10, !noundef !10
  %11 = getelementptr inbounds nuw i8, ptr %self, i32 400, !dbg !18723
  %_96.1 = load i32, ptr %11, align 4, !dbg !18723, !noundef !10
  %_8.i3847 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_96.0, i32 %_96.1, !dbg !18724
  br label %bb1.i.i3848, !dbg !18729

bb1.i.i3848:                                      ; preds = %bb11.i.i3851, %bb3
  %_221.i.i3849 = phi ptr [ %_22.i.i3852, %bb11.i.i3851 ], [ %_96.0, %bb3 ]
  %_12.i.i3850 = icmp eq ptr %_221.i.i3849, %_8.i3847, !dbg !18731
  br i1 %_12.i.i3850, label %bb5, label %bb11.i.i3851, !dbg !18734

bb11.i.i3851:                                     ; preds = %bb1.i.i3848
  %_22.i.i3852 = getelementptr inbounds nuw i8, ptr %_221.i.i3849, i32 16, !dbg !18735
  %12 = getelementptr inbounds nuw i8, ptr %_221.i.i3849, i32 12, !dbg !18737
  %_3.i.i.i3853 = load i32, ptr %12, align 4, !dbg !18737, !alias.scope !18739, !noalias !18744, !noundef !10
  %13 = icmp eq i32 %_3.i.i.i3853, 0, !dbg !18737
  %_51.i.i.i3854 = load i32, ptr %_221.i.i3849, align 4, !dbg !18737, !alias.scope !18739, !noalias !18744
  %14 = getelementptr inbounds nuw i8, ptr %_221.i.i3849, i32 4, !dbg !18737
  %_72.i.i.i3855 = load i32, ptr %14, align 4, !dbg !18737, !alias.scope !18739, !noalias !18744
  %15 = icmp eq i32 %_51.i.i.i3854, %_72.i.i.i3855, !dbg !18737
  %_0.sroa.0.0.off0.i.i.i3856 = select i1 %13, i1 %15, i1 false, !dbg !18737
  br i1 %_0.sroa.0.0.off0.i.i.i3856, label %bb1.i.i3848, label %bb20.thread, !dbg !18747

bb5:                                              ; preds = %bb1.i.i3848
  %16 = getelementptr inbounds nuw i8, ptr %self, i32 488, !dbg !18748
  %_97.0 = load ptr, ptr %16, align 8, !dbg !18748, !nonnull !10, !noundef !10
  %17 = getelementptr inbounds nuw i8, ptr %self, i32 492, !dbg !18748
  %_97.1 = load i32, ptr %17, align 4, !dbg !18748, !noundef !10
  %_8.i3858 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_97.0, i32 %_97.1, !dbg !18749
  br label %bb1.i.i3859, !dbg !18754

bb1.i.i3859:                                      ; preds = %bb11.i.i3862, %bb5
  %_221.i.i3860 = phi ptr [ %_22.i.i3863, %bb11.i.i3862 ], [ %_97.0, %bb5 ]
  %_12.i.i3861 = icmp eq ptr %_221.i.i3860, %_8.i3858, !dbg !18756
  br i1 %_12.i.i3861, label %bb7, label %bb11.i.i3862, !dbg !18759

bb11.i.i3862:                                     ; preds = %bb1.i.i3859
  %_22.i.i3863 = getelementptr inbounds nuw i8, ptr %_221.i.i3860, i32 16, !dbg !18760
  %18 = getelementptr inbounds nuw i8, ptr %_221.i.i3860, i32 12, !dbg !18762
  %_3.i.i.i3864 = load i32, ptr %18, align 4, !dbg !18762, !alias.scope !18764, !noalias !18769, !noundef !10
  %19 = icmp eq i32 %_3.i.i.i3864, 0, !dbg !18762
  %_51.i.i.i3865 = load i32, ptr %_221.i.i3860, align 4, !dbg !18762, !alias.scope !18764, !noalias !18769
  %20 = getelementptr inbounds nuw i8, ptr %_221.i.i3860, i32 4, !dbg !18762
  %_72.i.i.i3866 = load i32, ptr %20, align 4, !dbg !18762, !alias.scope !18764, !noalias !18769
  %21 = icmp eq i32 %_51.i.i.i3865, %_72.i.i.i3866, !dbg !18762
  %_0.sroa.0.0.off0.i.i.i3867 = select i1 %19, i1 %21, i1 false, !dbg !18762
  br i1 %_0.sroa.0.0.off0.i.i.i3867, label %bb1.i.i3859, label %bb20.thread, !dbg !18772

bb7:                                              ; preds = %bb1.i.i3859
  %22 = getelementptr inbounds nuw i8, ptr %self, i32 496, !dbg !18773
  %_98.0 = load ptr, ptr %22, align 8, !dbg !18773, !nonnull !10, !noundef !10
  %23 = getelementptr inbounds nuw i8, ptr %self, i32 500, !dbg !18773
  %_98.1 = load i32, ptr %23, align 4, !dbg !18773, !noundef !10
  %_8.i3869 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_98.0, i32 %_98.1, !dbg !18774
  br label %bb1.i.i3870, !dbg !18779

bb1.i.i3870:                                      ; preds = %bb11.i.i3873, %bb7
  %_221.i.i3871 = phi ptr [ %_22.i.i3874, %bb11.i.i3873 ], [ %_98.0, %bb7 ]
  %_12.i.i3872 = icmp eq ptr %_221.i.i3871, %_8.i3869, !dbg !18781
  br i1 %_12.i.i3872, label %bb9, label %bb11.i.i3873, !dbg !18784

bb11.i.i3873:                                     ; preds = %bb1.i.i3870
  %_22.i.i3874 = getelementptr inbounds nuw i8, ptr %_221.i.i3871, i32 16, !dbg !18785
  %24 = getelementptr inbounds nuw i8, ptr %_221.i.i3871, i32 12, !dbg !18787
  %_3.i.i.i3875 = load i32, ptr %24, align 4, !dbg !18787, !alias.scope !18789, !noalias !18794, !noundef !10
  %25 = icmp eq i32 %_3.i.i.i3875, 0, !dbg !18787
  %_51.i.i.i3876 = load i32, ptr %_221.i.i3871, align 4, !dbg !18787, !alias.scope !18789, !noalias !18794
  %26 = getelementptr inbounds nuw i8, ptr %_221.i.i3871, i32 4, !dbg !18787
  %_72.i.i.i3877 = load i32, ptr %26, align 4, !dbg !18787, !alias.scope !18789, !noalias !18794
  %27 = icmp eq i32 %_51.i.i.i3876, %_72.i.i.i3877, !dbg !18787
  %_0.sroa.0.0.off0.i.i.i3878 = select i1 %25, i1 %27, i1 false, !dbg !18787
  br i1 %_0.sroa.0.0.off0.i.i.i3878, label %bb1.i.i3870, label %bb20.thread, !dbg !18797

bb9:                                              ; preds = %bb1.i.i3870
  %_65.not = icmp ugt i32 %frames, %left_io.1
  br i1 %_65.not, label %bb45, label %bb1.i3880, !dbg !18798, !prof !4694

bb45:                                             ; preds = %bb9
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef %frames, i32 noundef %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ea4a5c70c3b6737d27e6f8140269468e) #33, !dbg !18807
  unreachable, !dbg !18807

bb1.i3880:                                        ; preds = %bb9, %bb12.i3885
  %io.sroa.5.0.i = phi i32 [ %len.i.i.i, %bb12.i3885 ], [ %frames, %bb9 ]
  %io.sroa.0.0.i = phi ptr [ %data.i.i.i, %bb12.i3885 ], [ %left_io.0, %bb9 ]
  %28 = icmp eq i32 %io.sroa.5.0.i, 0, !dbg !18808
  br i1 %28, label %bb11, label %bb13.preheader.i, !dbg !18808

bb13.preheader.i:                                 ; preds = %bb1.i3880
  %spec.store.select.i3881 = tail call i32 @llvm.umin.i32(i32 %io.sroa.5.0.i, i32 32), !dbg !18811
  %data.i.i.idx.i = shl nuw nsw i32 %spec.store.select.i3881, 2, !dbg !18814
  %data.i.i.i = getelementptr inbounds nuw i8, ptr %io.sroa.0.0.i, i32 %data.i.i.idx.i, !dbg !18814
  br label %bb13.i3882, !dbg !18819

bb13.i3882:                                       ; preds = %bb13.i3882, %bb13.preheader.i
  %iter.sroa.0.08.i = phi ptr [ %_35.i3883, %bb13.i3882 ], [ %io.sroa.0.0.i, %bb13.preheader.i ]
  %bits.sroa.0.07.i = phi i32 [ %29, %bb13.i3882 ], [ 0, %bb13.preheader.i ]
  %_35.i3883 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i, i32 4, !dbg !18821
  %_95.i3884 = load i32, ptr %iter.sroa.0.08.i, align 4, !dbg !18823, !alias.scope !18824, !noundef !10
  %29 = or i32 %_95.i3884, %bits.sroa.0.07.i, !dbg !18827
  %_29.i = icmp eq ptr %_35.i3883, %data.i.i.i, !dbg !18828
  br i1 %_29.i, label %bb12.i3885, label %bb13.i3882, !dbg !18819

bb12.i3885:                                       ; preds = %bb13.i3882
  %len.i.i.i = sub nuw nsw i32 %io.sroa.5.0.i, %spec.store.select.i3881, !dbg !18830
  %30 = icmp eq i32 %29, 0, !dbg !18831
  br i1 %30, label %bb1.i3880, label %bb20.thread, !dbg !18831

bb11:                                             ; preds = %bb1.i3880
  %_73.not = icmp ugt i32 %frames, %right_io.1, !dbg !18832
  br i1 %_73.not, label %bb48, label %bb1.i3887, !dbg !18832, !prof !902

bb20.thread:                                      ; preds = %bb11.i.i, %bb11.i.i3851, %bb11.i.i3862, %bb11.i.i3873, %bb12.i3885, %start
  %31 = getelementptr inbounds nuw i8, ptr %self, i32 536
  br label %bb26, !dbg !18838

bb20:                                             ; preds = %bb1.i3887
  %32 = getelementptr inbounds nuw i8, ptr %self, i32 536
  %33 = load i8, ptr %32, align 8, !range !4765
  %_22 = trunc nuw i8 %33 to i1
  br i1 %_22, label %bb22, label %bb26, !dbg !18838

bb48:                                             ; preds = %bb11
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef %frames, i32 noundef %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_786d3728118d574b1dded47beaf5441a) #33, !dbg !18840
  unreachable, !dbg !18840

bb1.i3887:                                        ; preds = %bb11, %bb12.i3901
  %io.sroa.5.0.i3888 = phi i32 [ %len.i.i.i3894, %bb12.i3901 ], [ %frames, %bb11 ]
  %io.sroa.0.0.i3889 = phi ptr [ %data.i.i.i3893, %bb12.i3901 ], [ %right_io.0, %bb11 ]
  %34 = icmp eq i32 %io.sroa.5.0.i3888, 0, !dbg !18841
  br i1 %34, label %bb20, label %bb13.preheader.i3890, !dbg !18841

bb13.preheader.i3890:                             ; preds = %bb1.i3887
  %spec.store.select.i3891 = tail call i32 @llvm.umin.i32(i32 %io.sroa.5.0.i3888, i32 32), !dbg !18844
  %data.i.i.idx.i3892 = shl nuw nsw i32 %spec.store.select.i3891, 2, !dbg !18847
  %data.i.i.i3893 = getelementptr inbounds nuw i8, ptr %io.sroa.0.0.i3889, i32 %data.i.i.idx.i3892, !dbg !18847
  br label %bb13.i3895, !dbg !18852

bb13.i3895:                                       ; preds = %bb13.i3895, %bb13.preheader.i3890
  %iter.sroa.0.08.i3896 = phi ptr [ %_35.i3898, %bb13.i3895 ], [ %io.sroa.0.0.i3889, %bb13.preheader.i3890 ]
  %bits.sroa.0.07.i3897 = phi i32 [ %35, %bb13.i3895 ], [ 0, %bb13.preheader.i3890 ]
  %_35.i3898 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i3896, i32 4, !dbg !18854
  %_95.i3899 = load i32, ptr %iter.sroa.0.08.i3896, align 4, !dbg !18856, !alias.scope !18857, !noundef !10
  %35 = or i32 %_95.i3899, %bits.sroa.0.07.i3897, !dbg !18860
  %_29.i3900 = icmp eq ptr %_35.i3898, %data.i.i.i3893, !dbg !18861
  br i1 %_29.i3900, label %bb12.i3901, label %bb13.i3895, !dbg !18852

bb12.i3901:                                       ; preds = %bb13.i3895
  %len.i.i.i3894 = sub nuw nsw i32 %io.sroa.5.0.i3888, %spec.store.select.i3891, !dbg !18863
  %36 = icmp eq i32 %35, 0, !dbg !18864
  br i1 %36, label %bb1.i3887, label %bb20.thread4439, !dbg !18864

bb20.thread4439:                                  ; preds = %bb12.i3901
  %37 = getelementptr inbounds nuw i8, ptr %self, i32 536
  br label %bb26, !dbg !18838

bb26:                                             ; preds = %bb20.thread4439, %bb20.thread, %bb20
  %38 = phi ptr [ %31, %bb20.thread ], [ %32, %bb20 ], [ %37, %bb20.thread4439 ]
  %quiet.sroa.0.0.off04438 = phi i1 [ false, %bb20.thread ], [ true, %bb20 ], [ false, %bb20.thread4439 ]
  %_31 = getelementptr inbounds nuw i8, ptr %self, i32 128, !dbg !18865
  %_32 = getelementptr inbounds nuw i8, ptr %self, i32 524, !dbg !18866
  %_33 = getelementptr inbounds nuw i8, ptr %self, i32 324, !dbg !18867
  %_34 = getelementptr inbounds nuw i8, ptr %self, i32 424, !dbg !18868
  %_35 = getelementptr inbounds nuw i8, ptr %self, i32 104, !dbg !18869
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18870), !dbg !18873
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18876), !dbg !18873
  %_8.i3904 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_95.0, i32 %_95.1, !dbg !18878
  br label %bb1.i.i3905, !dbg !18884

bb1.i.i3905:                                      ; preds = %bb11.i.i3908, %bb26
  %_221.i.i3906 = phi ptr [ %_22.i.i3909, %bb11.i.i3908 ], [ %_95.0, %bb26 ]
  %_12.i.i3907 = icmp eq ptr %_221.i.i3906, %_8.i3904, !dbg !18886
  br i1 %_12.i.i3907, label %bb2.i, label %bb11.i.i3908, !dbg !18889

bb11.i.i3908:                                     ; preds = %bb1.i.i3905
  %_22.i.i3909 = getelementptr inbounds nuw i8, ptr %_221.i.i3906, i32 16, !dbg !18890
  %39 = getelementptr inbounds nuw i8, ptr %_221.i.i3906, i32 12, !dbg !18892
  %_3.i.i.i3910 = load i32, ptr %39, align 4, !dbg !18892, !alias.scope !18894, !noalias !18899, !noundef !10
  %40 = icmp eq i32 %_3.i.i.i3910, 0, !dbg !18892
  %_51.i.i.i3911 = load i32, ptr %_221.i.i3906, align 4, !dbg !18892, !alias.scope !18894, !noalias !18899
  %41 = getelementptr inbounds nuw i8, ptr %_221.i.i3906, i32 4, !dbg !18892
  %_72.i.i.i3912 = load i32, ptr %41, align 4, !dbg !18892, !alias.scope !18894, !noalias !18899
  %42 = icmp eq i32 %_51.i.i.i3911, %_72.i.i.i3912, !dbg !18892
  %_0.sroa.0.0.off0.i.i.i3913 = select i1 %40, i1 %42, i1 false, !dbg !18892
  br i1 %_0.sroa.0.0.off0.i.i.i3913, label %bb1.i.i3905, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, !dbg !18908

bb2.i:                                            ; preds = %bb1.i.i3905
  %43 = getelementptr inbounds nuw i8, ptr %self, i32 396, !dbg !18909
  %_15.0.i = load ptr, ptr %43, align 4, !dbg !18909, !alias.scope !18870, !noalias !18910, !nonnull !10, !noundef !10
  %44 = getelementptr inbounds nuw i8, ptr %self, i32 400, !dbg !18909
  %_15.1.i = load i32, ptr %44, align 4, !dbg !18909, !alias.scope !18870, !noalias !18910, !noundef !10
  %_8.i3915 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_15.0.i, i32 %_15.1.i, !dbg !18911
  br label %bb1.i.i3916, !dbg !18916

bb1.i.i3916:                                      ; preds = %bb11.i.i3919, %bb2.i
  %_221.i.i3917 = phi ptr [ %_22.i.i3920, %bb11.i.i3919 ], [ %_15.0.i, %bb2.i ]
  %_12.i.i3918 = icmp eq ptr %_221.i.i3917, %_8.i3915, !dbg !18918
  br i1 %_12.i.i3918, label %bb4.i1454, label %bb11.i.i3919, !dbg !18921

bb11.i.i3919:                                     ; preds = %bb1.i.i3916
  %_22.i.i3920 = getelementptr inbounds nuw i8, ptr %_221.i.i3917, i32 16, !dbg !18922
  %45 = getelementptr inbounds nuw i8, ptr %_221.i.i3917, i32 12, !dbg !18924
  %_3.i.i.i3921 = load i32, ptr %45, align 4, !dbg !18924, !alias.scope !18926, !noalias !18931, !noundef !10
  %46 = icmp eq i32 %_3.i.i.i3921, 0, !dbg !18924
  %_51.i.i.i3922 = load i32, ptr %_221.i.i3917, align 4, !dbg !18924, !alias.scope !18926, !noalias !18931
  %47 = getelementptr inbounds nuw i8, ptr %_221.i.i3917, i32 4, !dbg !18924
  %_72.i.i.i3923 = load i32, ptr %47, align 4, !dbg !18924, !alias.scope !18926, !noalias !18931
  %48 = icmp eq i32 %_51.i.i.i3922, %_72.i.i.i3923, !dbg !18924
  %_0.sroa.0.0.off0.i.i.i3924 = select i1 %46, i1 %48, i1 false, !dbg !18924
  br i1 %_0.sroa.0.0.off0.i.i.i3924, label %bb1.i.i3916, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, !dbg !18934

bb4.i1454:                                        ; preds = %bb1.i.i3916
  %49 = getelementptr inbounds nuw i8, ptr %self, i32 488, !dbg !18935
  %_16.0.i = load ptr, ptr %49, align 4, !dbg !18935, !alias.scope !18876, !noalias !18936, !nonnull !10, !noundef !10
  %50 = getelementptr inbounds nuw i8, ptr %self, i32 492, !dbg !18935
  %_16.1.i = load i32, ptr %50, align 4, !dbg !18935, !alias.scope !18876, !noalias !18936, !noundef !10
  %_8.i3926 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_16.0.i, i32 %_16.1.i, !dbg !18937
  br label %bb1.i.i3927, !dbg !18942

bb1.i.i3927:                                      ; preds = %bb11.i.i3930, %bb4.i1454
  %_221.i.i3928 = phi ptr [ %_22.i.i3931, %bb11.i.i3930 ], [ %_16.0.i, %bb4.i1454 ]
  %_12.i.i3929 = icmp eq ptr %_221.i.i3928, %_8.i3926, !dbg !18944
  br i1 %_12.i.i3929, label %bb6.i1455, label %bb11.i.i3930, !dbg !18947

bb11.i.i3930:                                     ; preds = %bb1.i.i3927
  %_22.i.i3931 = getelementptr inbounds nuw i8, ptr %_221.i.i3928, i32 16, !dbg !18948
  %51 = getelementptr inbounds nuw i8, ptr %_221.i.i3928, i32 12, !dbg !18950
  %_3.i.i.i3932 = load i32, ptr %51, align 4, !dbg !18950, !alias.scope !18952, !noalias !18957, !noundef !10
  %52 = icmp eq i32 %_3.i.i.i3932, 0, !dbg !18950
  %_51.i.i.i3933 = load i32, ptr %_221.i.i3928, align 4, !dbg !18950, !alias.scope !18952, !noalias !18957
  %53 = getelementptr inbounds nuw i8, ptr %_221.i.i3928, i32 4, !dbg !18950
  %_72.i.i.i3934 = load i32, ptr %53, align 4, !dbg !18950, !alias.scope !18952, !noalias !18957
  %54 = icmp eq i32 %_51.i.i.i3933, %_72.i.i.i3934, !dbg !18950
  %_0.sroa.0.0.off0.i.i.i3935 = select i1 %52, i1 %54, i1 false, !dbg !18950
  br i1 %_0.sroa.0.0.off0.i.i.i3935, label %bb1.i.i3927, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, !dbg !18960

bb6.i1455:                                        ; preds = %bb1.i.i3927
  %55 = getelementptr inbounds nuw i8, ptr %self, i32 496, !dbg !18961
  %_17.0.i = load ptr, ptr %55, align 4, !dbg !18961, !alias.scope !18876, !noalias !18936, !nonnull !10, !noundef !10
  %56 = getelementptr inbounds nuw i8, ptr %self, i32 500, !dbg !18961
  %_17.1.i = load i32, ptr %56, align 4, !dbg !18961, !alias.scope !18876, !noalias !18936, !noundef !10
  %_8.i3937 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_17.0.i, i32 %_17.1.i, !dbg !18962
  br label %bb1.i.i3938, !dbg !18967

bb1.i.i3938:                                      ; preds = %bb11.i.i3941, %bb6.i1455
  %_221.i.i3939 = phi ptr [ %_22.i.i3942, %bb11.i.i3941 ], [ %_17.0.i, %bb6.i1455 ]
  %_12.i.i3940 = icmp eq ptr %_221.i.i3939, %_8.i3937, !dbg !18969
  br i1 %_12.i.i3940, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, label %bb11.i.i3941, !dbg !18972

bb11.i.i3941:                                     ; preds = %bb1.i.i3938
  %_22.i.i3942 = getelementptr inbounds nuw i8, ptr %_221.i.i3939, i32 16, !dbg !18973
  %57 = getelementptr inbounds nuw i8, ptr %_221.i.i3939, i32 12, !dbg !18975
  %_3.i.i.i3943 = load i32, ptr %57, align 4, !dbg !18975, !alias.scope !18977, !noalias !18982, !noundef !10
  %58 = icmp eq i32 %_3.i.i.i3943, 0, !dbg !18975
  %_51.i.i.i3944 = load i32, ptr %_221.i.i3939, align 4, !dbg !18975, !alias.scope !18977, !noalias !18982
  %59 = getelementptr inbounds nuw i8, ptr %_221.i.i3939, i32 4, !dbg !18975
  %_72.i.i.i3945 = load i32, ptr %59, align 4, !dbg !18975, !alias.scope !18977, !noalias !18982
  %60 = icmp eq i32 %_51.i.i.i3944, %_72.i.i.i3945, !dbg !18975
  %_0.sroa.0.0.off0.i.i.i3946 = select i1 %58, i1 %60, i1 false, !dbg !18975
  br i1 %_0.sroa.0.0.off0.i.i.i3946, label %bb1.i.i3938, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, !dbg !18985

_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit: ; preds = %bb11.i.i3908, %bb11.i.i3919, %bb11.i.i3930, %bb11.i.i3941, %bb1.i.i3938
  %_0.sroa.0.0.off0.i = phi i1 [ false, %bb11.i.i3930 ], [ false, %bb11.i.i3919 ], [ false, %bb11.i.i3941 ], [ true, %bb1.i.i3938 ], [ false, %bb11.i.i3908 ]
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18986), !dbg !18989
  %61 = getelementptr inbounds nuw i8, ptr %self, i32 412, !dbg !18991
  %_31.0.i = load ptr, ptr %61, align 4, !dbg !18991, !alias.scope !18986, !noalias !18993, !nonnull !10, !noundef !10
  %62 = getelementptr inbounds nuw i8, ptr %self, i32 416, !dbg !18991
  %_31.1.i = load i32, ptr %62, align 4, !dbg !18991, !alias.scope !18986, !noalias !18993, !noundef !10
  %_17.idx.i = mul nuw nsw i32 %_31.1.i, 12, !dbg !18994
  %_17.i = getelementptr inbounds nuw i8, ptr %_31.0.i, i32 %_17.idx.i, !dbg !18994
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18998), !dbg !19001, !noalias !18993
  %_5.not.i.i.i = icmp eq i32 %_31.1.i, 0
  %63 = getelementptr inbounds nuw i8, ptr %_31.0.i, i32 4
  %64 = getelementptr inbounds nuw i8, ptr %_31.0.i, i32 8
  br i1 %_5.not.i.i.i, label %bb2.i3956, label %bb1.i.i3948

bb1.i.i3948:                                      ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i
  %_224.i.i = phi ptr [ %_22.i.i3951, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i ], [ %_31.0.i, %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit ]
  %_12.i.i3949 = icmp eq ptr %_224.i.i, %_17.i, !dbg !19002
  br i1 %_12.i.i3949, label %bb2.i3956, label %bb11.i.i3950, !dbg !19006

bb11.i.i3950:                                     ; preds = %bb1.i.i3948
  %_22.i.i3951 = getelementptr inbounds nuw i8, ptr %_224.i.i, i32 12, !dbg !19007
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19009), !dbg !19012, !noalias !18993
  %_9.i.i.i = load i32, ptr %_224.i.i, align 4, !dbg !19013, !alias.scope !19009, !noalias !19016, !noundef !10
  %_10.i.i.i = load i32, ptr %_31.0.i, align 4, !dbg !19013, !alias.scope !18998, !noalias !19018, !noundef !10
  %_8.i.i.i = icmp eq i32 %_9.i.i.i, %_10.i.i.i, !dbg !19013
  br i1 %_8.i.i.i, label %bb2.i.i.i, label %bb10.i, !dbg !19013

bb2.i.i.i:                                        ; preds = %bb11.i.i3950
  %65 = getelementptr inbounds nuw i8, ptr %_224.i.i, i32 4, !dbg !19013
  %_12.i.i.i = load i32, ptr %65, align 4, !dbg !19013, !alias.scope !19009, !noalias !19016, !noundef !10
  %_13.i.i.i = load i32, ptr %63, align 4, !dbg !19013, !alias.scope !18998, !noalias !19018, !noundef !10
  %_11.i.i.i = icmp eq i32 %_12.i.i.i, %_13.i.i.i, !dbg !19013
  br i1 %_11.i.i.i, label %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, label %bb10.i, !dbg !19013

_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i: ; preds = %bb2.i.i.i
  %66 = getelementptr inbounds nuw i8, ptr %_224.i.i, i32 8, !dbg !19013
  %_14.i.i.i3954 = load i32, ptr %66, align 4, !dbg !19013, !alias.scope !19009, !noalias !19016, !noundef !10
  %_15.i.i.i3955 = load i32, ptr %64, align 4, !dbg !19013, !alias.scope !18998, !noalias !19018, !noundef !10
  %67 = icmp eq i32 %_14.i.i.i3954, %_15.i.i.i3955, !dbg !19013
  br i1 %67, label %bb1.i.i3948, label %bb10.i, !dbg !19012

bb2.i3956:                                        ; preds = %bb1.i.i3948, %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit
  %68 = getelementptr inbounds nuw i8, ptr %self, i32 380, !dbg !19019
  %_32.0.i = load ptr, ptr %68, align 4, !dbg !19019, !alias.scope !18986, !noalias !18993, !nonnull !10, !noundef !10
  %69 = getelementptr inbounds nuw i8, ptr %self, i32 384, !dbg !19019
  %_32.1.i = load i32, ptr %69, align 4, !dbg !19019, !alias.scope !18986, !noalias !18993, !noundef !10
  %_26.idx.i = shl nuw nsw i32 %_32.1.i, 2, !dbg !19020
  %_26.i = getelementptr inbounds nuw i8, ptr %_32.0.i, i32 %_26.idx.i, !dbg !19020
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19024), !dbg !19027, !noalias !18993
  %_6.not.i.i.i = icmp eq i32 %_32.1.i, 0
  br i1 %_6.not.i.i.i, label %bb3.i, label %bb1.i3.i

bb1.i3.i:                                         ; preds = %bb2.i3956, %bb11.i5.i
  %_223.i.i = phi ptr [ %_22.i6.i, %bb11.i5.i ], [ %_32.0.i, %bb2.i3956 ]
  %_12.i4.i = icmp eq ptr %_223.i.i, %_26.i, !dbg !19028
  br i1 %_12.i4.i, label %bb3.i, label %bb11.i5.i, !dbg !19032

bb11.i5.i:                                        ; preds = %bb1.i3.i
  %_22.i6.i = getelementptr inbounds nuw i8, ptr %_223.i.i, i32 4, !dbg !19033
  %ptr.val.i.i = load i32, ptr %_223.i.i, align 4, !dbg !19035, !noalias !19036
  %_4.i.i.i3957 = load i32, ptr %_32.0.i, align 4, !dbg !19038, !alias.scope !19024, !noalias !19040, !noundef !10
  %_0.i.i.i = icmp eq i32 %ptr.val.i.i, %_4.i.i.i3957, !dbg !19041
  br i1 %_0.i.i.i, label %bb1.i3.i, label %bb10.i, !dbg !19035

bb3.i:                                            ; preds = %bb1.i3.i, %bb2.i3956
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19042), !dbg !19045
  %70 = getelementptr inbounds nuw i8, ptr %self, i32 512, !dbg !19046
  %_31.0.i3958 = load ptr, ptr %70, align 4, !dbg !19046, !alias.scope !19042, !noalias !18993, !nonnull !10, !noundef !10
  %71 = getelementptr inbounds nuw i8, ptr %self, i32 516, !dbg !19046
  %_31.1.i3959 = load i32, ptr %71, align 4, !dbg !19046, !alias.scope !19042, !noalias !18993, !noundef !10
  %_17.idx.i3960 = mul nuw nsw i32 %_31.1.i3959, 12, !dbg !19048
  %_17.i3961 = getelementptr inbounds nuw i8, ptr %_31.0.i3958, i32 %_17.idx.i3960, !dbg !19048
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19052), !dbg !19055, !noalias !18993
  %_5.not.i.i.i3962 = icmp eq i32 %_31.1.i3959, 0
  %72 = getelementptr inbounds nuw i8, ptr %_31.0.i3958, i32 4
  %73 = getelementptr inbounds nuw i8, ptr %_31.0.i3958, i32 8
  br i1 %_5.not.i.i.i3962, label %bb2.i3980, label %bb1.i.i3963

bb1.i.i3963:                                      ; preds = %bb3.i, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3977
  %_224.i.i3964 = phi ptr [ %_22.i.i3967, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3977 ], [ %_31.0.i3958, %bb3.i ]
  %_12.i.i3965 = icmp eq ptr %_224.i.i3964, %_17.i3961, !dbg !19056
  br i1 %_12.i.i3965, label %bb2.i3980, label %bb11.i.i3966, !dbg !19060

bb11.i.i3966:                                     ; preds = %bb1.i.i3963
  %_22.i.i3967 = getelementptr inbounds nuw i8, ptr %_224.i.i3964, i32 12, !dbg !19061
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19063), !dbg !19066, !noalias !18993
  %_9.i.i.i3968 = load i32, ptr %_224.i.i3964, align 4, !dbg !19067, !alias.scope !19063, !noalias !19070, !noundef !10
  %_10.i.i.i3969 = load i32, ptr %_31.0.i3958, align 4, !dbg !19067, !alias.scope !19052, !noalias !19072, !noundef !10
  %_8.i.i.i3970 = icmp eq i32 %_9.i.i.i3968, %_10.i.i.i3969, !dbg !19067
  br i1 %_8.i.i.i3970, label %bb2.i.i.i3973, label %bb10.i, !dbg !19067

bb2.i.i.i3973:                                    ; preds = %bb11.i.i3966
  %74 = getelementptr inbounds nuw i8, ptr %_224.i.i3964, i32 4, !dbg !19067
  %_12.i.i.i3974 = load i32, ptr %74, align 4, !dbg !19067, !alias.scope !19063, !noalias !19070, !noundef !10
  %_13.i.i.i3975 = load i32, ptr %72, align 4, !dbg !19067, !alias.scope !19052, !noalias !19072, !noundef !10
  %_11.i.i.i3976 = icmp eq i32 %_12.i.i.i3974, %_13.i.i.i3975, !dbg !19067
  br i1 %_11.i.i.i3976, label %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3977, label %bb10.i, !dbg !19067

_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3977: ; preds = %bb2.i.i.i3973
  %75 = getelementptr inbounds nuw i8, ptr %_224.i.i3964, i32 8, !dbg !19067
  %_14.i.i.i3978 = load i32, ptr %75, align 4, !dbg !19067, !alias.scope !19063, !noalias !19070, !noundef !10
  %_15.i.i.i3979 = load i32, ptr %73, align 4, !dbg !19067, !alias.scope !19052, !noalias !19072, !noundef !10
  %76 = icmp eq i32 %_14.i.i.i3978, %_15.i.i.i3979, !dbg !19067
  br i1 %76, label %bb1.i.i3963, label %bb10.i, !dbg !19066

bb2.i3980:                                        ; preds = %bb1.i.i3963, %bb3.i
  %77 = getelementptr inbounds nuw i8, ptr %self, i32 480, !dbg !19073
  %_32.0.i3981 = load ptr, ptr %77, align 4, !dbg !19073, !alias.scope !19042, !noalias !18993, !nonnull !10, !noundef !10
  %78 = getelementptr inbounds nuw i8, ptr %self, i32 484, !dbg !19073
  %_32.1.i3982 = load i32, ptr %78, align 4, !dbg !19073, !alias.scope !19042, !noalias !18993, !noundef !10
  %_26.idx.i3983 = shl nuw nsw i32 %_32.1.i3982, 2, !dbg !19074
  %_26.i3984 = getelementptr inbounds nuw i8, ptr %_32.0.i3981, i32 %_26.idx.i3983, !dbg !19074
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19078), !dbg !19081, !noalias !18993
  %_6.not.i.i.i3985 = icmp eq i32 %_32.1.i3982, 0
  br i1 %_6.not.i.i.i3985, label %bb5.i, label %bb1.i3.i3986

bb1.i3.i3986:                                     ; preds = %bb2.i3980, %bb11.i5.i3989
  %_223.i.i3987 = phi ptr [ %_22.i6.i3990, %bb11.i5.i3989 ], [ %_32.0.i3981, %bb2.i3980 ]
  %_12.i4.i3988 = icmp eq ptr %_223.i.i3987, %_26.i3984, !dbg !19082
  br i1 %_12.i4.i3988, label %bb5.i, label %bb11.i5.i3989, !dbg !19086

bb11.i5.i3989:                                    ; preds = %bb1.i3.i3986
  %_22.i6.i3990 = getelementptr inbounds nuw i8, ptr %_223.i.i3987, i32 4, !dbg !19087
  %ptr.val.i.i3991 = load i32, ptr %_223.i.i3987, align 4, !dbg !19089, !noalias !19090
  %_4.i.i.i3992 = load i32, ptr %_32.0.i3981, align 4, !dbg !19092, !alias.scope !19078, !noalias !19094, !noundef !10
  %_0.i.i.i3993 = icmp eq i32 %ptr.val.i.i3991, %_4.i.i.i3992, !dbg !19095
  br i1 %_0.i.i.i3993, label %bb1.i3.i3986, label %bb10.i, !dbg !19089

bb10.i:                                           ; preds = %bb11.i.i3950, %bb2.i.i.i, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, %bb11.i5.i, %bb11.i.i3966, %bb2.i.i.i3973, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3977, %bb11.i5.i3989
  %79 = getelementptr inbounds nuw i8, ptr %self, i32 320, !dbg !19096
  %80 = getelementptr inbounds nuw i8, ptr %self, i32 321, !dbg !19096
  %81 = getelementptr inbounds nuw i8, ptr %self, i32 108, !dbg !19096
  br i1 %_0.sroa.0.0.off0.i, label %bb11.i, label %bb12.i, !dbg !19097

bb5.i:                                            ; preds = %bb1.i3.i3986, %bb2.i3980
  br i1 %_0.sroa.0.0.off0.i, label %bb6.i, label %bb7.i, !dbg !19098

bb12.i:                                           ; preds = %bb10.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19099), !dbg !19102
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19103), !dbg !19102
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19105), !dbg !19102
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19107), !dbg !19102
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i708), !dbg !19109, !noalias !19113
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_left.i708, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #32, !dbg !19117, !noalias !19118
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_right.i707), !dbg !19119, !noalias !19113
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_right.i707, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #32, !dbg !19121, !noalias !19122
  %82 = load i8, ptr %79, align 4, !dbg !19123, !range !4765, !alias.scope !19099, !noalias !19127, !noundef !10
  %83 = load i8, ptr %80, align 1, !dbg !19128, !range !4765, !alias.scope !19099, !noalias !19127, !noundef !10
  %_35.i716 = load i32, ptr %_35, align 4, !dbg !19130, !alias.scope !19107, !noalias !19132, !noundef !10
  %_37.i717 = load i32, ptr %81, align 4, !dbg !19133, !alias.scope !19107, !noalias !19132, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i706), !dbg !19135, !noalias !19113
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i706, i8 0, i32 32, i1 false), !noalias !19113
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i705), !dbg !19137, !noalias !19113
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i705, i8 0, i32 1024, i1 false), !noalias !19113
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i704), !dbg !19139, !noalias !19113
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i704, i8 0, i32 1024, i1 false), !noalias !19113
  %_32.i712 = zext nneg i8 %82 to i32, !dbg !19123
  %.none.i713 = sub nsw i32 0, %_32.i712, !dbg !19141
  %_33.i714 = zext nneg i8 %83 to i32, !dbg !19128
  %all.sroa.0.0.i715 = sub nsw i32 0, %_33.i714, !dbg !19128
  %_111.not.i7307579 = icmp eq i32 %frames, 0, !dbg !19142
  br i1 %_111.not.i7307579, label %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit, label %bb37.i731.lr.ph, !dbg !19142

bb37.i731.lr.ph:                                  ; preds = %bb12.i
  %d9.i = lshr i32 %frames, 5, !dbg !19152
  %r2.i = and i32 %frames, 31, !dbg !19159
  %_19.not.i = icmp ne i32 %r2.i, 0, !dbg !19160
  %84 = zext i1 %_19.not.i to i32, !dbg !19160
  %yield_count.sroa.0.0.i = add nuw nsw i32 %d9.i, %84, !dbg !19160
  %history.i136.i.sroa.7.0.hot_left.i708.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i708, i32 4
  %history.i136.i.sroa.10.0.hot_left.i708.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i708, i32 8
  %history.i136.i.sroa.13.0.hot_left.i708.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i708, i32 12
  %history.i136.i.sroa.16.0.hot_left.i708.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i708, i32 16
  %history.i136.i.sroa.19.0.hot_left.i708.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i708, i32 20
  %history.i136.i.sroa.22.0.hot_left.i708.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i708, i32 24
  %history.i136.i.sroa.26.0.hot_left.i708.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i708, i32 28
  %history.i136.i.sroa.29.0.hot_left.i708.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i708, i32 32
  %history.i136.i.sroa.32.0.hot_left.i708.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i708, i32 36
  %history.i136.i.sroa.35.0.hot_left.i708.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i708, i32 40
  %history.i136.i.sroa.38.0.hot_left.i708.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i708, i32 44
  %85 = getelementptr inbounds nuw i8, ptr %self, i32 132
  %86 = getelementptr inbounds nuw i8, ptr %self, i32 136
  %87 = getelementptr inbounds nuw i8, ptr %self, i32 140
  %row1.i.i.i172.i = getelementptr inbounds nuw i8, ptr %self, i32 144
  %88 = getelementptr inbounds nuw i8, ptr %self, i32 148
  %89 = getelementptr inbounds nuw i8, ptr %self, i32 152
  %90 = getelementptr inbounds nuw i8, ptr %self, i32 156
  %row3.i.i.i186.i = getelementptr inbounds nuw i8, ptr %self, i32 160
  %91 = getelementptr inbounds nuw i8, ptr %self, i32 164
  %92 = getelementptr inbounds nuw i8, ptr %self, i32 168
  %93 = getelementptr inbounds nuw i8, ptr %self, i32 172
  %row5.i.i.i200.i = getelementptr inbounds nuw i8, ptr %self, i32 176
  %94 = getelementptr inbounds nuw i8, ptr %self, i32 180
  %95 = getelementptr inbounds nuw i8, ptr %self, i32 184
  %96 = getelementptr inbounds nuw i8, ptr %self, i32 188
  %row7.i.i.i214.i = getelementptr inbounds nuw i8, ptr %self, i32 192
  %97 = getelementptr inbounds nuw i8, ptr %self, i32 196
  %98 = getelementptr inbounds nuw i8, ptr %self, i32 200
  %99 = getelementptr inbounds nuw i8, ptr %self, i32 204
  %row9.i.i.i228.i = getelementptr inbounds nuw i8, ptr %self, i32 208
  %100 = getelementptr inbounds nuw i8, ptr %self, i32 212
  %101 = getelementptr inbounds nuw i8, ptr %self, i32 216
  %102 = getelementptr inbounds nuw i8, ptr %self, i32 220
  %row11.i.i.i242.i = getelementptr inbounds nuw i8, ptr %self, i32 224
  %103 = getelementptr inbounds nuw i8, ptr %self, i32 228
  %104 = getelementptr inbounds nuw i8, ptr %self, i32 232
  %105 = getelementptr inbounds nuw i8, ptr %self, i32 236
  %row13.i.i.i256.i = getelementptr inbounds nuw i8, ptr %self, i32 240
  %106 = getelementptr inbounds nuw i8, ptr %self, i32 244
  %107 = getelementptr inbounds nuw i8, ptr %self, i32 248
  %108 = getelementptr inbounds nuw i8, ptr %self, i32 252
  %row15.i.i.i270.i = getelementptr inbounds nuw i8, ptr %self, i32 256
  %109 = getelementptr inbounds nuw i8, ptr %self, i32 260
  %110 = getelementptr inbounds nuw i8, ptr %self, i32 264
  %111 = getelementptr inbounds nuw i8, ptr %self, i32 268
  %row17.i.i.i284.i = getelementptr inbounds nuw i8, ptr %self, i32 272
  %112 = getelementptr inbounds nuw i8, ptr %self, i32 276
  %113 = getelementptr inbounds nuw i8, ptr %self, i32 280
  %114 = getelementptr inbounds nuw i8, ptr %self, i32 284
  %row19.i.i.i298.i = getelementptr inbounds nuw i8, ptr %self, i32 288
  %115 = getelementptr inbounds nuw i8, ptr %self, i32 292
  %116 = getelementptr inbounds nuw i8, ptr %self, i32 296
  %117 = getelementptr inbounds nuw i8, ptr %self, i32 300
  %row21.i.i.i312.i = getelementptr inbounds nuw i8, ptr %self, i32 304
  %118 = getelementptr inbounds nuw i8, ptr %self, i32 308
  %119 = getelementptr inbounds nuw i8, ptr %self, i32 312
  %120 = getelementptr inbounds nuw i8, ptr %self, i32 316
  %history.i.i701.sroa.7.0.hot_right.i707.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i707, i32 4
  %history.i.i701.sroa.10.0.hot_right.i707.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i707, i32 8
  %history.i.i701.sroa.13.0.hot_right.i707.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i707, i32 12
  %history.i.i701.sroa.16.0.hot_right.i707.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i707, i32 16
  %history.i.i701.sroa.19.0.hot_right.i707.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i707, i32 20
  %history.i.i701.sroa.22.0.hot_right.i707.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i707, i32 24
  %history.i.i701.sroa.26.0.hot_right.i707.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i707, i32 28
  %history.i.i701.sroa.29.0.hot_right.i707.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i707, i32 32
  %history.i.i701.sroa.32.0.hot_right.i707.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i707, i32 36
  %history.i.i701.sroa.35.0.hot_right.i707.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i707, i32 40
  %history.i.i701.sroa.38.0.hot_right.i707.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i707, i32 44
  %_64.i744 = getelementptr inbounds nuw i8, ptr %hot_left.i708, i32 48
  %_65.i745 = getelementptr inbounds nuw i8, ptr %hot_left.i708, i32 64
  %121 = getelementptr inbounds nuw i8, ptr %hot_left.i708, i32 60
  %122 = getelementptr inbounds nuw i8, ptr %hot_left.i708, i32 56
  %123 = getelementptr inbounds nuw i8, ptr %hot_left.i708, i32 52
  %124 = getelementptr inbounds nuw i8, ptr %hot_left.i708, i32 76
  %125 = getelementptr inbounds nuw i8, ptr %hot_left.i708, i32 72
  %126 = getelementptr inbounds nuw i8, ptr %hot_left.i708, i32 68
  %_69.i746 = getelementptr inbounds nuw i8, ptr %hot_right.i707, i32 48
  %_70.i747 = getelementptr inbounds nuw i8, ptr %hot_right.i707, i32 64
  %127 = getelementptr inbounds nuw i8, ptr %hot_right.i707, i32 60
  %128 = getelementptr inbounds nuw i8, ptr %hot_right.i707, i32 56
  %129 = getelementptr inbounds nuw i8, ptr %hot_right.i707, i32 52
  %130 = getelementptr inbounds nuw i8, ptr %hot_right.i707, i32 76
  %131 = getelementptr inbounds nuw i8, ptr %hot_right.i707, i32 72
  %132 = getelementptr inbounds nuw i8, ptr %hot_right.i707, i32 68
  %_9.i3374 = add nsw i32 %_32.i712, -1
  %133 = getelementptr inbounds nuw i8, ptr %self, i32 528
  %134 = getelementptr inbounds nuw i8, ptr %self, i32 420
  %135 = getelementptr inbounds nuw i8, ptr %self, i32 344
  %136 = getelementptr inbounds nuw i8, ptr %self, i32 340
  %137 = getelementptr inbounds nuw i8, ptr %self, i32 384
  %138 = getelementptr inbounds nuw i8, ptr %self, i32 380
  %139 = getelementptr inbounds nuw i8, ptr %self, i32 368
  %140 = getelementptr inbounds nuw i8, ptr %self, i32 364
  %141 = getelementptr inbounds nuw i8, ptr %self, i32 352
  %142 = getelementptr inbounds nuw i8, ptr %self, i32 348
  %143 = getelementptr inbounds nuw i8, ptr %hot_left.i708, i32 84
  %144 = getelementptr inbounds nuw i8, ptr %hot_left.i708, i32 88
  %145 = getelementptr inbounds nuw i8, ptr %hot_left.i708, i32 80
  %146 = getelementptr inbounds nuw i8, ptr %self, i32 336
  %147 = getelementptr inbounds nuw i8, ptr %self, i32 332
  %_9.i3354 = add nsw i32 %_33.i714, -1
  %148 = getelementptr inbounds nuw i8, ptr %self, i32 520
  %149 = getelementptr inbounds nuw i8, ptr %self, i32 444
  %150 = getelementptr inbounds nuw i8, ptr %self, i32 440
  %151 = getelementptr inbounds nuw i8, ptr %self, i32 516
  %152 = getelementptr inbounds nuw i8, ptr %self, i32 512
  %153 = getelementptr inbounds nuw i8, ptr %self, i32 484
  %154 = getelementptr inbounds nuw i8, ptr %self, i32 480
  %155 = getelementptr inbounds nuw i8, ptr %self, i32 468
  %156 = getelementptr inbounds nuw i8, ptr %self, i32 464
  %157 = getelementptr inbounds nuw i8, ptr %self, i32 452
  %158 = getelementptr inbounds nuw i8, ptr %self, i32 448
  %159 = getelementptr inbounds nuw i8, ptr %hot_right.i707, i32 84
  %160 = getelementptr inbounds nuw i8, ptr %hot_right.i707, i32 88
  %161 = getelementptr inbounds nuw i8, ptr %hot_right.i707, i32 80
  %162 = getelementptr inbounds nuw i8, ptr %self, i32 436
  %163 = getelementptr inbounds nuw i8, ptr %self, i32 432
  %164 = getelementptr inbounds nuw i8, ptr %self, i32 532
  %iter.sroa.0.0.ptr.i55.i7866453.1 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 4
  %iter.sroa.0.0.ptr.i55.i7866453.2 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 8
  %iter.sroa.0.0.ptr.i55.i7866453.3 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 12
  %iter.sroa.0.0.ptr.i55.i7866453.4 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 16
  %iter.sroa.0.0.ptr.i55.i7866453.5 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 20
  %iter.sroa.0.0.ptr.i55.i7866453.6 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 24
  %iter.sroa.0.0.ptr.i55.i7866453.7 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 28
  %iter.sroa.0.0.ptr.i.i8796465.1 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 4
  %iter.sroa.0.0.ptr.i.i8796465.2 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 8
  %iter.sroa.0.0.ptr.i.i8796465.3 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 12
  %iter.sroa.0.0.ptr.i.i8796465.4 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 16
  %iter.sroa.0.0.ptr.i.i8796465.5 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 20
  %iter.sroa.0.0.ptr.i.i8796465.6 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 24
  %iter.sroa.0.0.ptr.i.i8796465.7 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 28
  br label %bb37.i731, !dbg !19142

bb16.i737.bb13.i725.loopexit_crit_edge:           ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077
  store float %_0.i.i3527, ptr %121, align 4, !dbg !19161, !alias.scope !19172, !noalias !19175
  store float %_0.i3299, ptr %_64.i744, align 4, !dbg !19178, !alias.scope !19172, !noalias !19175
  store float %_0.i3292, ptr %122, align 4, !dbg !19180, !alias.scope !19172, !noalias !19175
  store float %_0.i.i3534, ptr %124, align 4, !dbg !19181, !alias.scope !19183, !noalias !19186
  store float %_0.i3312, ptr %_65.i745, align 4, !dbg !19187, !alias.scope !19183, !noalias !19186
  store float %_0.i3305, ptr %125, align 4, !dbg !19188, !alias.scope !19183, !noalias !19186
  store float %_0.i.i3541, ptr %127, align 4, !dbg !19189, !alias.scope !19193, !noalias !19196
  store float %_0.i3325, ptr %_69.i746, align 4, !dbg !19199, !alias.scope !19193, !noalias !19196
  store float %_0.i3318, ptr %128, align 4, !dbg !19200, !alias.scope !19193, !noalias !19196
  store float %_0.i.i3548, ptr %130, align 4, !dbg !19201, !alias.scope !19203, !noalias !19186
  store float %_0.i3338, ptr %_70.i747, align 4, !dbg !19206, !alias.scope !19203, !noalias !19186
  store float %_0.i3331, ptr %131, align 4, !dbg !19207, !alias.scope !19203, !noalias !19186
  store float %_0.i2838, ptr %143, align 4, !dbg !19208
  store float %_0.i3212, ptr %145, align 4, !dbg !19223
  store float %_0.i2834, ptr %159, align 4, !dbg !19226
  store float %_0.i3208, ptr %161, align 4, !dbg !19228
  br label %bb13.i725.loopexit, !dbg !19229

bb13.i725.loopexit:                               ; preds = %bb16.i737.bb13.i725.loopexit_crit_edge, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i736
  %ring_cursor.sroa.0.1.i739.lcssa = phi i32 [ %spec.store.select12.i949, %bb16.i737.bb13.i725.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i7287582, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i736 ], !dbg !19235
  %main_cursor.sroa.0.1.i740.lcssa = phi i32 [ %spec.store.select11.i947, %bb16.i737.bb13.i725.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i7297583, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i736 ], !dbg !19236
  %_111.not.i730 = icmp eq i32 %167, 0, !dbg !19142
  %indvars.iv.next = add i32 %indvars.iv, -32, !dbg !19142
  br i1 %_111.not.i730, label %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit, label %bb37.i731, !dbg !19142

bb37.i731:                                        ; preds = %bb37.i731.lr.ph, %bb13.i725.loopexit
  %indvars.iv = phi i32 [ %frames, %bb37.i731.lr.ph ], [ %indvars.iv.next, %bb13.i725.loopexit ]
  %main_cursor.sroa.0.0.i7297583 = phi i32 [ %_35.i716, %bb37.i731.lr.ph ], [ %main_cursor.sroa.0.1.i740.lcssa, %bb13.i725.loopexit ]
  %ring_cursor.sroa.0.0.i7287582 = phi i32 [ %_37.i717, %bb37.i731.lr.ph ], [ %ring_cursor.sroa.0.1.i739.lcssa, %bb13.i725.loopexit ]
  %iter2.sroa.0.0.i7277581 = phi i32 [ %yield_count.sroa.0.0.i, %bb37.i731.lr.ph ], [ %167, %bb13.i725.loopexit ]
  %iter.sroa.0.0.i7267580 = phi i32 [ 0, %bb37.i731.lr.ph ], [ %166, %bb13.i725.loopexit ]
  %165 = call i32 @llvm.umax.i32(i32 %indvars.iv, i32 1), !dbg !19237
  %umax11911 = call i32 @llvm.umin.i32(i32 %165, i32 32), !dbg !19237
  %166 = add i32 %iter.sroa.0.0.i7267580, 32, !dbg !19237
  %167 = add nsw i32 %iter2.sroa.0.0.i7277581, -1, !dbg !19241
  %history.i136.i.sroa.0.0.copyload = load float, ptr %hot_left.i708, align 4, !dbg !19242, !noalias !19246
  %history.i136.i.sroa.7.0.copyload = load float, ptr %history.i136.i.sroa.7.0.hot_left.i708.sroa_idx, align 4, !dbg !19242, !noalias !19246
  %history.i136.i.sroa.10.0.copyload = load float, ptr %history.i136.i.sroa.10.0.hot_left.i708.sroa_idx, align 4, !dbg !19242, !noalias !19246
  %history.i136.i.sroa.13.0.copyload = load float, ptr %history.i136.i.sroa.13.0.hot_left.i708.sroa_idx, align 4, !dbg !19242, !noalias !19246
  %history.i136.i.sroa.16.0.copyload = load float, ptr %history.i136.i.sroa.16.0.hot_left.i708.sroa_idx, align 4, !dbg !19242, !noalias !19246
  %history.i136.i.sroa.19.0.copyload = load float, ptr %history.i136.i.sroa.19.0.hot_left.i708.sroa_idx, align 4, !dbg !19242, !noalias !19246
  %history.i136.i.sroa.22.0.copyload = load float, ptr %history.i136.i.sroa.22.0.hot_left.i708.sroa_idx, align 4, !dbg !19242, !noalias !19246
  %history.i136.i.sroa.26.0.copyload = load float, ptr %history.i136.i.sroa.26.0.hot_left.i708.sroa_idx, align 4, !dbg !19242, !noalias !19246
  %history.i136.i.sroa.29.0.copyload = load float, ptr %history.i136.i.sroa.29.0.hot_left.i708.sroa_idx, align 4, !dbg !19242, !noalias !19246
  %history.i136.i.sroa.32.0.copyload = load float, ptr %history.i136.i.sroa.32.0.hot_left.i708.sroa_idx, align 4, !dbg !19242, !noalias !19246
  %history.i136.i.sroa.35.0.copyload = load float, ptr %history.i136.i.sroa.35.0.hot_left.i708.sroa_idx, align 4, !dbg !19242, !noalias !19246
  %history.i136.i.sroa.38.0.copyload = load float, ptr %history.i136.i.sroa.38.0.hot_left.i708.sroa_idx, align 4, !dbg !19242, !noalias !19246
  %_20.i139.i6391.not = icmp eq i32 %frames, %iter.sroa.0.0.i7267580, !dbg !19251
  br i1 %_20.i139.i6391.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i, label %bb5.i140.i.lr.ph, !dbg !19261

bb5.i140.i.lr.ph:                                 ; preds = %bb37.i731
  %_11.i.i.i160.i = load float, ptr %_31, align 4
  %_14.i.i.i163.i = load float, ptr %85, align 4
  %_17.i.i.i166.i = load float, ptr %86, align 4
  %_20.i.i.i169.i = load float, ptr %87, align 4
  %_25.i.i.i174.i = load float, ptr %row1.i.i.i172.i, align 4
  %_28.i.i.i177.i = load float, ptr %88, align 4
  %_31.i.i.i180.i = load float, ptr %89, align 4
  %_34.i.i.i183.i = load float, ptr %90, align 4
  %_39.i.i.i188.i = load float, ptr %row3.i.i.i186.i, align 4
  %_42.i.i.i191.i = load float, ptr %91, align 4
  %_45.i.i.i194.i = load float, ptr %92, align 4
  %_48.i.i.i197.i = load float, ptr %93, align 4
  %_53.i.i.i202.i = load float, ptr %row5.i.i.i200.i, align 4
  %_56.i.i.i205.i = load float, ptr %94, align 4
  %_59.i.i.i208.i = load float, ptr %95, align 4
  %_62.i.i.i211.i = load float, ptr %96, align 4
  %_67.i.i.i216.i = load float, ptr %row7.i.i.i214.i, align 4
  %_70.i.i.i219.i = load float, ptr %97, align 4
  %_73.i.i.i222.i = load float, ptr %98, align 4
  %_76.i.i.i225.i = load float, ptr %99, align 4
  %_81.i.i.i230.i = load float, ptr %row9.i.i.i228.i, align 4
  %_84.i.i.i233.i = load float, ptr %100, align 4
  %_87.i.i.i236.i = load float, ptr %101, align 4
  %_90.i.i.i239.i = load float, ptr %102, align 4
  %_95.i.i.i244.i = load float, ptr %row11.i.i.i242.i, align 4
  %_98.i.i.i247.i = load float, ptr %103, align 4
  %_101.i.i.i250.i = load float, ptr %104, align 4
  %_104.i.i.i253.i = load float, ptr %105, align 4
  %_109.i.i.i258.i = load float, ptr %row13.i.i.i256.i, align 4
  %_112.i.i.i261.i = load float, ptr %106, align 4
  %_115.i.i.i264.i = load float, ptr %107, align 4
  %_118.i.i.i267.i = load float, ptr %108, align 4
  %_123.i.i.i272.i = load float, ptr %row15.i.i.i270.i, align 4
  %_126.i.i.i275.i = load float, ptr %109, align 4
  %_129.i.i.i278.i = load float, ptr %110, align 4
  %_132.i.i.i281.i = load float, ptr %111, align 4
  %_137.i.i.i286.i = load float, ptr %row17.i.i.i284.i, align 4
  %_140.i.i.i289.i = load float, ptr %112, align 4
  %_143.i.i.i292.i = load float, ptr %113, align 4
  %_146.i.i.i295.i = load float, ptr %114, align 4
  %_151.i.i.i300.i = load float, ptr %row19.i.i.i298.i, align 4
  %_154.i.i.i303.i = load float, ptr %115, align 4
  %_157.i.i.i306.i = load float, ptr %116, align 4
  %_160.i.i.i309.i = load float, ptr %117, align 4
  %_165.i.i.i314.i = load float, ptr %row21.i.i.i312.i, align 4
  %_168.i.i.i317.i = load float, ptr %118, align 4
  %_171.i.i.i320.i = load float, ptr %119, align 4
  %_174.i.i.i323.i = load float, ptr %120, align 4
  br label %bb5.i140.i, !dbg !19261

bb5.i140.i:                                       ; preds = %bb5.i140.i.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit
  %iter.sroa.0.0.i138.i6403 = phi i32 [ 0, %bb5.i140.i.lr.ph ], [ %168, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.35.06402 = phi float [ %history.i136.i.sroa.35.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.32.06401, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.32.06401 = phi float [ %history.i136.i.sroa.32.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.29.06400, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.29.06400 = phi float [ %history.i136.i.sroa.29.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.26.06399, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.26.06399 = phi float [ %history.i136.i.sroa.26.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.22.06398, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.22.06398 = phi float [ %history.i136.i.sroa.22.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.19.06397, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.19.06397 = phi float [ %history.i136.i.sroa.19.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.16.06396, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.16.06396 = phi float [ %history.i136.i.sroa.16.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.13.06395, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.13.06395 = phi float [ %history.i136.i.sroa.13.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.10.06394, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.10.06394 = phi float [ %history.i136.i.sroa.10.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.7.06393, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.7.06393 = phi float [ %history.i136.i.sroa.7.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.0.06392, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.0.06392 = phi float [ %history.i136.i.sroa.0.0.copyload, %bb5.i140.i.lr.ph ], [ %_0.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ]
  %168 = add nuw nsw i32 %iter.sroa.0.0.i138.i6403, 1, !dbg !19262
  %_11.i141.i = add nuw nsw i32 %iter.sroa.0.0.i138.i6403, %iter.sroa.0.0.i7267580, !dbg !19268
  %_24.i142.i = icmp ugt i32 %_11.i141.i, %left_io.1, !dbg !19270
  br i1 %_24.i142.i, label %bb7.i339.i, label %bb8.i143.i, !dbg !19270, !prof !902

bb8.i143.i:                                       ; preds = %bb5.i140.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19276), !dbg !19279
  %_3.not.i = icmp eq i32 %left_io.1, %_11.i141.i, !dbg !19280
  br i1 %_3.not.i, label %panic.i2912, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit, !dbg !19280

panic.i2912:                                      ; preds = %bb8.i143.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #33, !dbg !19280, !noalias !19282
  unreachable, !dbg !19280

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit: ; preds = %bb8.i143.i
  %_31.i145.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %_11.i141.i, !dbg !19284
  %_0.i = load float, ptr %_31.i145.i, align 4, !dbg !19280, !alias.scope !19276, !noalias !19289, !noundef !10
  %169 = tail call noundef float @llvm.fabs.f32(float %history.i136.i.sroa.19.06397), !dbg !19290
  %_0.i2462 = fmul float %_0.i, %_11.i.i.i160.i, !dbg !19296
  %_0.i2038 = fadd float %_0.i2462, 0.000000e+00, !dbg !19308
  %_0.i2461 = fmul float %_0.i, %_14.i.i.i163.i, !dbg !19311
  %_0.i2037 = fadd float %_0.i2461, 0.000000e+00, !dbg !19313
  %_0.i2460 = fmul float %_0.i, %_17.i.i.i166.i, !dbg !19315
  %_0.i2036 = fadd float %_0.i2460, 0.000000e+00, !dbg !19317
  %_0.i2459 = fmul float %_0.i, %_20.i.i.i169.i, !dbg !19319
  %_0.i2035 = fadd float %_0.i2459, 0.000000e+00, !dbg !19321
  %_0.i2458 = fmul float %history.i136.i.sroa.0.06392, %_25.i.i.i174.i, !dbg !19323
  %_0.i2034 = fadd float %_0.i2038, %_0.i2458, !dbg !19327
  %_0.i2457 = fmul float %history.i136.i.sroa.0.06392, %_28.i.i.i177.i, !dbg !19329
  %_0.i2033 = fadd float %_0.i2037, %_0.i2457, !dbg !19331
  %_0.i2456 = fmul float %history.i136.i.sroa.0.06392, %_31.i.i.i180.i, !dbg !19333
  %_0.i2032 = fadd float %_0.i2036, %_0.i2456, !dbg !19335
  %_0.i2455 = fmul float %history.i136.i.sroa.0.06392, %_34.i.i.i183.i, !dbg !19337
  %_0.i2031 = fadd float %_0.i2035, %_0.i2455, !dbg !19339
  %_0.i2454 = fmul float %history.i136.i.sroa.7.06393, %_39.i.i.i188.i, !dbg !19341
  %_0.i2030 = fadd float %_0.i2034, %_0.i2454, !dbg !19345
  %_0.i2453 = fmul float %history.i136.i.sroa.7.06393, %_42.i.i.i191.i, !dbg !19347
  %_0.i2029 = fadd float %_0.i2033, %_0.i2453, !dbg !19349
  %_0.i2452 = fmul float %history.i136.i.sroa.7.06393, %_45.i.i.i194.i, !dbg !19351
  %_0.i2028 = fadd float %_0.i2032, %_0.i2452, !dbg !19353
  %_0.i2451 = fmul float %history.i136.i.sroa.7.06393, %_48.i.i.i197.i, !dbg !19355
  %_0.i2027 = fadd float %_0.i2031, %_0.i2451, !dbg !19357
  %_0.i2450 = fmul float %history.i136.i.sroa.10.06394, %_53.i.i.i202.i, !dbg !19359
  %_0.i2026 = fadd float %_0.i2030, %_0.i2450, !dbg !19363
  %_0.i2449 = fmul float %history.i136.i.sroa.10.06394, %_56.i.i.i205.i, !dbg !19365
  %_0.i2025 = fadd float %_0.i2029, %_0.i2449, !dbg !19367
  %_0.i2448 = fmul float %history.i136.i.sroa.10.06394, %_59.i.i.i208.i, !dbg !19369
  %_0.i2024 = fadd float %_0.i2028, %_0.i2448, !dbg !19371
  %_0.i2447 = fmul float %history.i136.i.sroa.10.06394, %_62.i.i.i211.i, !dbg !19373
  %_0.i2023 = fadd float %_0.i2027, %_0.i2447, !dbg !19375
  %_0.i2446 = fmul float %history.i136.i.sroa.13.06395, %_67.i.i.i216.i, !dbg !19377
  %_0.i2022 = fadd float %_0.i2026, %_0.i2446, !dbg !19381
  %_0.i2445 = fmul float %history.i136.i.sroa.13.06395, %_70.i.i.i219.i, !dbg !19383
  %_0.i2021 = fadd float %_0.i2025, %_0.i2445, !dbg !19385
  %_0.i2444 = fmul float %history.i136.i.sroa.13.06395, %_73.i.i.i222.i, !dbg !19387
  %_0.i2020 = fadd float %_0.i2024, %_0.i2444, !dbg !19389
  %_0.i2443 = fmul float %history.i136.i.sroa.13.06395, %_76.i.i.i225.i, !dbg !19391
  %_0.i2019 = fadd float %_0.i2023, %_0.i2443, !dbg !19393
  %_0.i2442 = fmul float %history.i136.i.sroa.16.06396, %_81.i.i.i230.i, !dbg !19395
  %_0.i2018 = fadd float %_0.i2022, %_0.i2442, !dbg !19399
  %_0.i2441 = fmul float %history.i136.i.sroa.16.06396, %_84.i.i.i233.i, !dbg !19401
  %_0.i2017 = fadd float %_0.i2021, %_0.i2441, !dbg !19403
  %_0.i2440 = fmul float %history.i136.i.sroa.16.06396, %_87.i.i.i236.i, !dbg !19405
  %_0.i2016 = fadd float %_0.i2020, %_0.i2440, !dbg !19407
  %_0.i2439 = fmul float %history.i136.i.sroa.16.06396, %_90.i.i.i239.i, !dbg !19409
  %_0.i2015 = fadd float %_0.i2019, %_0.i2439, !dbg !19411
  %_0.i2438 = fmul float %history.i136.i.sroa.19.06397, %_95.i.i.i244.i, !dbg !19413
  %_0.i2014 = fadd float %_0.i2018, %_0.i2438, !dbg !19417
  %_0.i2437 = fmul float %history.i136.i.sroa.19.06397, %_98.i.i.i247.i, !dbg !19419
  %_0.i2013 = fadd float %_0.i2017, %_0.i2437, !dbg !19421
  %_0.i2436 = fmul float %history.i136.i.sroa.19.06397, %_101.i.i.i250.i, !dbg !19423
  %_0.i2012 = fadd float %_0.i2016, %_0.i2436, !dbg !19425
  %_0.i2435 = fmul float %history.i136.i.sroa.19.06397, %_104.i.i.i253.i, !dbg !19427
  %_0.i2011 = fadd float %_0.i2015, %_0.i2435, !dbg !19429
  %_0.i2434 = fmul float %history.i136.i.sroa.22.06398, %_109.i.i.i258.i, !dbg !19431
  %_0.i2010 = fadd float %_0.i2014, %_0.i2434, !dbg !19435
  %_0.i2433 = fmul float %history.i136.i.sroa.22.06398, %_112.i.i.i261.i, !dbg !19437
  %_0.i2009 = fadd float %_0.i2013, %_0.i2433, !dbg !19439
  %_0.i2432 = fmul float %history.i136.i.sroa.22.06398, %_115.i.i.i264.i, !dbg !19441
  %_0.i2008 = fadd float %_0.i2012, %_0.i2432, !dbg !19443
  %_0.i2431 = fmul float %history.i136.i.sroa.22.06398, %_118.i.i.i267.i, !dbg !19445
  %_0.i2007 = fadd float %_0.i2011, %_0.i2431, !dbg !19447
  %_0.i2430 = fmul float %history.i136.i.sroa.26.06399, %_123.i.i.i272.i, !dbg !19449
  %_0.i2006 = fadd float %_0.i2010, %_0.i2430, !dbg !19453
  %_0.i2429 = fmul float %history.i136.i.sroa.26.06399, %_126.i.i.i275.i, !dbg !19455
  %_0.i2005 = fadd float %_0.i2009, %_0.i2429, !dbg !19457
  %_0.i2428 = fmul float %history.i136.i.sroa.26.06399, %_129.i.i.i278.i, !dbg !19459
  %_0.i2004 = fadd float %_0.i2008, %_0.i2428, !dbg !19461
  %_0.i2427 = fmul float %history.i136.i.sroa.26.06399, %_132.i.i.i281.i, !dbg !19463
  %_0.i2003 = fadd float %_0.i2007, %_0.i2427, !dbg !19465
  %_0.i2426 = fmul float %history.i136.i.sroa.29.06400, %_137.i.i.i286.i, !dbg !19467
  %_0.i2002 = fadd float %_0.i2006, %_0.i2426, !dbg !19471
  %_0.i2425 = fmul float %history.i136.i.sroa.29.06400, %_140.i.i.i289.i, !dbg !19473
  %_0.i2001 = fadd float %_0.i2005, %_0.i2425, !dbg !19475
  %_0.i2424 = fmul float %history.i136.i.sroa.29.06400, %_143.i.i.i292.i, !dbg !19477
  %_0.i2000 = fadd float %_0.i2004, %_0.i2424, !dbg !19479
  %_0.i2423 = fmul float %history.i136.i.sroa.29.06400, %_146.i.i.i295.i, !dbg !19481
  %_0.i1999 = fadd float %_0.i2003, %_0.i2423, !dbg !19483
  %_0.i2422 = fmul float %history.i136.i.sroa.32.06401, %_151.i.i.i300.i, !dbg !19485
  %_0.i1998 = fadd float %_0.i2002, %_0.i2422, !dbg !19489
  %_0.i2421 = fmul float %history.i136.i.sroa.32.06401, %_154.i.i.i303.i, !dbg !19491
  %_0.i1997 = fadd float %_0.i2001, %_0.i2421, !dbg !19493
  %_0.i2420 = fmul float %history.i136.i.sroa.32.06401, %_157.i.i.i306.i, !dbg !19495
  %_0.i1996 = fadd float %_0.i2000, %_0.i2420, !dbg !19497
  %_0.i2419 = fmul float %history.i136.i.sroa.32.06401, %_160.i.i.i309.i, !dbg !19499
  %_0.i1995 = fadd float %_0.i1999, %_0.i2419, !dbg !19501
  %_0.i2418 = fmul float %history.i136.i.sroa.35.06402, %_165.i.i.i314.i, !dbg !19503
  %_0.i1994 = fadd float %_0.i1998, %_0.i2418, !dbg !19507
  %_0.i2417 = fmul float %history.i136.i.sroa.35.06402, %_168.i.i.i317.i, !dbg !19509
  %_0.i1993 = fadd float %_0.i1997, %_0.i2417, !dbg !19511
  %_0.i2416 = fmul float %history.i136.i.sroa.35.06402, %_171.i.i.i320.i, !dbg !19513
  %_0.i1992 = fadd float %_0.i1996, %_0.i2416, !dbg !19515
  %_0.i2415 = fmul float %history.i136.i.sroa.35.06402, %_174.i.i.i323.i, !dbg !19517
  %_0.i1991 = fadd float %_0.i1995, %_0.i2415, !dbg !19519
  %170 = tail call noundef float @llvm.fabs.f32(float %_0.i1994), !dbg !19521
  %_3.i.i3549.inv = fcmp ogt float %169, %170, !dbg !19525
  %_4.i.i.v = select i1 %_3.i.i3549.inv, float %169, float %170, !dbg !19525
  %171 = tail call noundef float @llvm.fabs.f32(float %_0.i1993), !dbg !19521
  %_3.i.i3549.inv.1 = fcmp ogt float %_4.i.i.v, %171, !dbg !19525
  %_4.i.i.v.1 = select i1 %_3.i.i3549.inv.1, float %_4.i.i.v, float %171, !dbg !19525
  %172 = tail call noundef float @llvm.fabs.f32(float %_0.i1992), !dbg !19521
  %_3.i.i3549.inv.2 = fcmp ogt float %_4.i.i.v.1, %172, !dbg !19525
  %_4.i.i.v.2 = select i1 %_3.i.i3549.inv.2, float %_4.i.i.v.1, float %172, !dbg !19525
  %173 = tail call noundef float @llvm.fabs.f32(float %_0.i1991), !dbg !19521
  %_3.i.i3549.inv.3 = fcmp ogt float %_4.i.i.v.2, %173, !dbg !19525
  %_4.i.i.v.3 = select i1 %_3.i.i3549.inv.3, float %_4.i.i.v.2, float %173, !dbg !19525
  %_39.i334.i = getelementptr inbounds nuw float, ptr %peaks_left.i705, i32 %iter.sroa.0.0.i138.i6403, !dbg !19531
  store float %_4.i.i.v.3, ptr %_39.i334.i, align 4, !dbg !19542, !alias.scope !19544, !noalias !19289
  %exitcond.not = icmp eq i32 %168, %umax11911, !dbg !19251
  br i1 %exitcond.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i, label %bb5.i140.i, !dbg !19261

bb7.i339.i:                                       ; preds = %bb5.i140.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_11.i141.i, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #33, !dbg !19547, !noalias !19289
  unreachable, !dbg !19547

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit, %bb37.i731
  %history.i136.i.sroa.0.0.lcssa = phi float [ %history.i136.i.sroa.0.0.copyload, %bb37.i731 ], [ %_0.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !19548
  %history.i136.i.sroa.7.0.lcssa = phi float [ %history.i136.i.sroa.7.0.copyload, %bb37.i731 ], [ %history.i136.i.sroa.0.06392, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !19548
  %history.i136.i.sroa.10.0.lcssa = phi float [ %history.i136.i.sroa.10.0.copyload, %bb37.i731 ], [ %history.i136.i.sroa.7.06393, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !19548
  %history.i136.i.sroa.13.0.lcssa = phi float [ %history.i136.i.sroa.13.0.copyload, %bb37.i731 ], [ %history.i136.i.sroa.10.06394, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !19548
  %history.i136.i.sroa.16.0.lcssa = phi float [ %history.i136.i.sroa.16.0.copyload, %bb37.i731 ], [ %history.i136.i.sroa.13.06395, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !19548
  %history.i136.i.sroa.19.0.lcssa = phi float [ %history.i136.i.sroa.19.0.copyload, %bb37.i731 ], [ %history.i136.i.sroa.16.06396, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !19548
  %history.i136.i.sroa.22.0.lcssa = phi float [ %history.i136.i.sroa.22.0.copyload, %bb37.i731 ], [ %history.i136.i.sroa.19.06397, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !19548
  %history.i136.i.sroa.26.0.lcssa = phi float [ %history.i136.i.sroa.26.0.copyload, %bb37.i731 ], [ %history.i136.i.sroa.22.06398, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !19548
  %history.i136.i.sroa.29.0.lcssa = phi float [ %history.i136.i.sroa.29.0.copyload, %bb37.i731 ], [ %history.i136.i.sroa.26.06399, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !19548
  %history.i136.i.sroa.32.0.lcssa = phi float [ %history.i136.i.sroa.32.0.copyload, %bb37.i731 ], [ %history.i136.i.sroa.29.06400, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !19548
  %history.i136.i.sroa.35.0.lcssa = phi float [ %history.i136.i.sroa.35.0.copyload, %bb37.i731 ], [ %history.i136.i.sroa.32.06401, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !19548
  %history.i136.i.sroa.38.0.lcssa = phi float [ %history.i136.i.sroa.38.0.copyload, %bb37.i731 ], [ %history.i136.i.sroa.35.06402, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !19548
  store float %history.i136.i.sroa.0.0.lcssa, ptr %hot_left.i708, align 4, !dbg !19549, !noalias !19246
  store float %history.i136.i.sroa.7.0.lcssa, ptr %history.i136.i.sroa.7.0.hot_left.i708.sroa_idx, align 4, !dbg !19549, !noalias !19246
  store float %history.i136.i.sroa.10.0.lcssa, ptr %history.i136.i.sroa.10.0.hot_left.i708.sroa_idx, align 4, !dbg !19549, !noalias !19246
  store float %history.i136.i.sroa.13.0.lcssa, ptr %history.i136.i.sroa.13.0.hot_left.i708.sroa_idx, align 4, !dbg !19549, !noalias !19246
  store float %history.i136.i.sroa.16.0.lcssa, ptr %history.i136.i.sroa.16.0.hot_left.i708.sroa_idx, align 4, !dbg !19549, !noalias !19246
  store float %history.i136.i.sroa.19.0.lcssa, ptr %history.i136.i.sroa.19.0.hot_left.i708.sroa_idx, align 4, !dbg !19549, !noalias !19246
  store float %history.i136.i.sroa.22.0.lcssa, ptr %history.i136.i.sroa.22.0.hot_left.i708.sroa_idx, align 4, !dbg !19549, !noalias !19246
  store float %history.i136.i.sroa.26.0.lcssa, ptr %history.i136.i.sroa.26.0.hot_left.i708.sroa_idx, align 4, !dbg !19549, !noalias !19246
  store float %history.i136.i.sroa.29.0.lcssa, ptr %history.i136.i.sroa.29.0.hot_left.i708.sroa_idx, align 4, !dbg !19549, !noalias !19246
  store float %history.i136.i.sroa.32.0.lcssa, ptr %history.i136.i.sroa.32.0.hot_left.i708.sroa_idx, align 4, !dbg !19549, !noalias !19246
  store float %history.i136.i.sroa.35.0.lcssa, ptr %history.i136.i.sroa.35.0.hot_left.i708.sroa_idx, align 4, !dbg !19549, !noalias !19246
  store float %history.i136.i.sroa.38.0.lcssa, ptr %history.i136.i.sroa.38.0.hot_left.i708.sroa_idx, align 4, !dbg !19549, !noalias !19246
  %history.i.i701.sroa.0.0.copyload = load float, ptr %hot_right.i707, align 4, !dbg !19550, !noalias !19552
  %history.i.i701.sroa.7.0.copyload = load float, ptr %history.i.i701.sroa.7.0.hot_right.i707.sroa_idx, align 4, !dbg !19550, !noalias !19552
  %history.i.i701.sroa.10.0.copyload = load float, ptr %history.i.i701.sroa.10.0.hot_right.i707.sroa_idx, align 4, !dbg !19550, !noalias !19552
  %history.i.i701.sroa.13.0.copyload = load float, ptr %history.i.i701.sroa.13.0.hot_right.i707.sroa_idx, align 4, !dbg !19550, !noalias !19552
  %history.i.i701.sroa.16.0.copyload = load float, ptr %history.i.i701.sroa.16.0.hot_right.i707.sroa_idx, align 4, !dbg !19550, !noalias !19552
  %history.i.i701.sroa.19.0.copyload = load float, ptr %history.i.i701.sroa.19.0.hot_right.i707.sroa_idx, align 4, !dbg !19550, !noalias !19552
  %history.i.i701.sroa.22.0.copyload = load float, ptr %history.i.i701.sroa.22.0.hot_right.i707.sroa_idx, align 4, !dbg !19550, !noalias !19552
  %history.i.i701.sroa.26.0.copyload = load float, ptr %history.i.i701.sroa.26.0.hot_right.i707.sroa_idx, align 4, !dbg !19550, !noalias !19552
  %history.i.i701.sroa.29.0.copyload = load float, ptr %history.i.i701.sroa.29.0.hot_right.i707.sroa_idx, align 4, !dbg !19550, !noalias !19552
  %history.i.i701.sroa.32.0.copyload = load float, ptr %history.i.i701.sroa.32.0.hot_right.i707.sroa_idx, align 4, !dbg !19550, !noalias !19552
  %history.i.i701.sroa.35.0.copyload = load float, ptr %history.i.i701.sroa.35.0.hot_right.i707.sroa_idx, align 4, !dbg !19550, !noalias !19552
  %history.i.i701.sroa.38.0.copyload = load float, ptr %history.i.i701.sroa.38.0.hot_right.i707.sroa_idx, align 4, !dbg !19550, !noalias !19552
  br i1 %_20.i139.i6391.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i736, label %bb5.i.i962.lr.ph, !dbg !19557

bb5.i.i962.lr.ph:                                 ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i
  %_11.i.i.i.i980 = load float, ptr %_31, align 4
  %_14.i.i.i.i983 = load float, ptr %85, align 4
  %_17.i.i.i.i986 = load float, ptr %86, align 4
  %_20.i.i.i.i989 = load float, ptr %87, align 4
  %_25.i.i.i.i994 = load float, ptr %row1.i.i.i172.i, align 4
  %_28.i.i.i.i997 = load float, ptr %88, align 4
  %_31.i.i.i.i1000 = load float, ptr %89, align 4
  %_34.i.i.i.i1003 = load float, ptr %90, align 4
  %_39.i.i.i.i1008 = load float, ptr %row3.i.i.i186.i, align 4
  %_42.i.i.i.i1011 = load float, ptr %91, align 4
  %_45.i.i.i.i1014 = load float, ptr %92, align 4
  %_48.i.i.i.i1017 = load float, ptr %93, align 4
  %_53.i.i.i.i1022 = load float, ptr %row5.i.i.i200.i, align 4
  %_56.i.i.i.i1025 = load float, ptr %94, align 4
  %_59.i.i.i.i1028 = load float, ptr %95, align 4
  %_62.i.i.i.i1031 = load float, ptr %96, align 4
  %_67.i.i.i.i1036 = load float, ptr %row7.i.i.i214.i, align 4
  %_70.i.i.i.i1039 = load float, ptr %97, align 4
  %_73.i.i.i.i1042 = load float, ptr %98, align 4
  %_76.i.i.i.i1045 = load float, ptr %99, align 4
  %_81.i.i.i.i1050 = load float, ptr %row9.i.i.i228.i, align 4
  %_84.i.i.i.i1053 = load float, ptr %100, align 4
  %_87.i.i.i.i1056 = load float, ptr %101, align 4
  %_90.i.i.i.i1059 = load float, ptr %102, align 4
  %_95.i.i.i.i1064 = load float, ptr %row11.i.i.i242.i, align 4
  %_98.i.i.i.i1067 = load float, ptr %103, align 4
  %_101.i.i.i.i1070 = load float, ptr %104, align 4
  %_104.i.i.i.i1073 = load float, ptr %105, align 4
  %_109.i.i.i.i1078 = load float, ptr %row13.i.i.i256.i, align 4
  %_112.i.i.i.i1081 = load float, ptr %106, align 4
  %_115.i.i.i.i1084 = load float, ptr %107, align 4
  %_118.i.i.i.i1087 = load float, ptr %108, align 4
  %_123.i.i.i.i1092 = load float, ptr %row15.i.i.i270.i, align 4
  %_126.i.i.i.i1095 = load float, ptr %109, align 4
  %_129.i.i.i.i1098 = load float, ptr %110, align 4
  %_132.i.i.i.i1101 = load float, ptr %111, align 4
  %_137.i.i.i.i1106 = load float, ptr %row17.i.i.i284.i, align 4
  %_140.i.i.i.i1109 = load float, ptr %112, align 4
  %_143.i.i.i.i1112 = load float, ptr %113, align 4
  %_146.i.i.i.i1115 = load float, ptr %114, align 4
  %_151.i.i.i.i1120 = load float, ptr %row19.i.i.i298.i, align 4
  %_154.i.i.i.i1123 = load float, ptr %115, align 4
  %_157.i.i.i.i1126 = load float, ptr %116, align 4
  %_160.i.i.i.i1129 = load float, ptr %117, align 4
  %_165.i.i.i.i1134 = load float, ptr %row21.i.i.i312.i, align 4
  %_168.i.i.i.i1137 = load float, ptr %118, align 4
  %_171.i.i.i.i1140 = load float, ptr %119, align 4
  %_174.i.i.i.i1143 = load float, ptr %120, align 4
  br label %bb5.i.i962, !dbg !19557

bb5.i.i962:                                       ; preds = %bb5.i.i962.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917
  %iter.sroa.0.0.i.i7346429 = phi i32 [ 0, %bb5.i.i962.lr.ph ], [ %174, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ]
  %history.i.i701.sroa.35.06428 = phi float [ %history.i.i701.sroa.35.0.copyload, %bb5.i.i962.lr.ph ], [ %history.i.i701.sroa.32.06427, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ]
  %history.i.i701.sroa.32.06427 = phi float [ %history.i.i701.sroa.32.0.copyload, %bb5.i.i962.lr.ph ], [ %history.i.i701.sroa.29.06426, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ]
  %history.i.i701.sroa.29.06426 = phi float [ %history.i.i701.sroa.29.0.copyload, %bb5.i.i962.lr.ph ], [ %history.i.i701.sroa.26.06425, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ]
  %history.i.i701.sroa.26.06425 = phi float [ %history.i.i701.sroa.26.0.copyload, %bb5.i.i962.lr.ph ], [ %history.i.i701.sroa.22.06424, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ]
  %history.i.i701.sroa.22.06424 = phi float [ %history.i.i701.sroa.22.0.copyload, %bb5.i.i962.lr.ph ], [ %history.i.i701.sroa.19.06423, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ]
  %history.i.i701.sroa.19.06423 = phi float [ %history.i.i701.sroa.19.0.copyload, %bb5.i.i962.lr.ph ], [ %history.i.i701.sroa.16.06422, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ]
  %history.i.i701.sroa.16.06422 = phi float [ %history.i.i701.sroa.16.0.copyload, %bb5.i.i962.lr.ph ], [ %history.i.i701.sroa.13.06421, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ]
  %history.i.i701.sroa.13.06421 = phi float [ %history.i.i701.sroa.13.0.copyload, %bb5.i.i962.lr.ph ], [ %history.i.i701.sroa.10.06420, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ]
  %history.i.i701.sroa.10.06420 = phi float [ %history.i.i701.sroa.10.0.copyload, %bb5.i.i962.lr.ph ], [ %history.i.i701.sroa.7.06419, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ]
  %history.i.i701.sroa.7.06419 = phi float [ %history.i.i701.sroa.7.0.copyload, %bb5.i.i962.lr.ph ], [ %history.i.i701.sroa.0.06418, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ]
  %history.i.i701.sroa.0.06418 = phi float [ %history.i.i701.sroa.0.0.copyload, %bb5.i.i962.lr.ph ], [ %_0.i2915, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ]
  %174 = add nuw nsw i32 %iter.sroa.0.0.i.i7346429, 1, !dbg !19560
  %_11.i126.i = add nuw nsw i32 %iter.sroa.0.0.i.i7346429, %iter.sroa.0.0.i7267580, !dbg !19563
  %_24.i.i963 = icmp ugt i32 %_11.i126.i, %right_io.1, !dbg !19564
  br i1 %_24.i.i963, label %bb7.i.i1159, label %bb8.i.i964, !dbg !19564, !prof !902

bb8.i.i964:                                       ; preds = %bb5.i.i962
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19567), !dbg !19570
  %_3.not.i2913 = icmp eq i32 %right_io.1, %_11.i126.i, !dbg !19571
  br i1 %_3.not.i2913, label %panic.i2916, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917, !dbg !19571

panic.i2916:                                      ; preds = %bb8.i.i964
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #33, !dbg !19571, !noalias !19573
  unreachable, !dbg !19571

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917: ; preds = %bb8.i.i964
  %_31.i127.i = getelementptr inbounds nuw float, ptr %right_io.0, i32 %_11.i126.i, !dbg !19575
  %_0.i2915 = load float, ptr %_31.i127.i, align 4, !dbg !19571, !alias.scope !19567, !noalias !19577, !noundef !10
  %175 = tail call noundef float @llvm.fabs.f32(float %history.i.i701.sroa.19.06423), !dbg !19578
  %_0.i2510 = fmul float %_0.i2915, %_11.i.i.i.i980, !dbg !19581
  %_0.i2086 = fadd float %_0.i2510, 0.000000e+00, !dbg !19584
  %_0.i2509 = fmul float %_0.i2915, %_14.i.i.i.i983, !dbg !19586
  %_0.i2085 = fadd float %_0.i2509, 0.000000e+00, !dbg !19588
  %_0.i2508 = fmul float %_0.i2915, %_17.i.i.i.i986, !dbg !19590
  %_0.i2084 = fadd float %_0.i2508, 0.000000e+00, !dbg !19592
  %_0.i2507 = fmul float %_0.i2915, %_20.i.i.i.i989, !dbg !19594
  %_0.i2083 = fadd float %_0.i2507, 0.000000e+00, !dbg !19596
  %_0.i2506 = fmul float %history.i.i701.sroa.0.06418, %_25.i.i.i.i994, !dbg !19598
  %_0.i2082 = fadd float %_0.i2086, %_0.i2506, !dbg !19600
  %_0.i2505 = fmul float %history.i.i701.sroa.0.06418, %_28.i.i.i.i997, !dbg !19602
  %_0.i2081 = fadd float %_0.i2085, %_0.i2505, !dbg !19604
  %_0.i2504 = fmul float %history.i.i701.sroa.0.06418, %_31.i.i.i.i1000, !dbg !19606
  %_0.i2080 = fadd float %_0.i2084, %_0.i2504, !dbg !19608
  %_0.i2503 = fmul float %history.i.i701.sroa.0.06418, %_34.i.i.i.i1003, !dbg !19610
  %_0.i2079 = fadd float %_0.i2083, %_0.i2503, !dbg !19612
  %_0.i2502 = fmul float %history.i.i701.sroa.7.06419, %_39.i.i.i.i1008, !dbg !19614
  %_0.i2078 = fadd float %_0.i2082, %_0.i2502, !dbg !19616
  %_0.i2501 = fmul float %history.i.i701.sroa.7.06419, %_42.i.i.i.i1011, !dbg !19618
  %_0.i2077 = fadd float %_0.i2081, %_0.i2501, !dbg !19620
  %_0.i2500 = fmul float %history.i.i701.sroa.7.06419, %_45.i.i.i.i1014, !dbg !19622
  %_0.i2076 = fadd float %_0.i2080, %_0.i2500, !dbg !19624
  %_0.i2499 = fmul float %history.i.i701.sroa.7.06419, %_48.i.i.i.i1017, !dbg !19626
  %_0.i2075 = fadd float %_0.i2079, %_0.i2499, !dbg !19628
  %_0.i2498 = fmul float %history.i.i701.sroa.10.06420, %_53.i.i.i.i1022, !dbg !19630
  %_0.i2074 = fadd float %_0.i2078, %_0.i2498, !dbg !19632
  %_0.i2497 = fmul float %history.i.i701.sroa.10.06420, %_56.i.i.i.i1025, !dbg !19634
  %_0.i2073 = fadd float %_0.i2077, %_0.i2497, !dbg !19636
  %_0.i2496 = fmul float %history.i.i701.sroa.10.06420, %_59.i.i.i.i1028, !dbg !19638
  %_0.i2072 = fadd float %_0.i2076, %_0.i2496, !dbg !19640
  %_0.i2495 = fmul float %history.i.i701.sroa.10.06420, %_62.i.i.i.i1031, !dbg !19642
  %_0.i2071 = fadd float %_0.i2075, %_0.i2495, !dbg !19644
  %_0.i2494 = fmul float %history.i.i701.sroa.13.06421, %_67.i.i.i.i1036, !dbg !19646
  %_0.i2070 = fadd float %_0.i2074, %_0.i2494, !dbg !19648
  %_0.i2493 = fmul float %history.i.i701.sroa.13.06421, %_70.i.i.i.i1039, !dbg !19650
  %_0.i2069 = fadd float %_0.i2073, %_0.i2493, !dbg !19652
  %_0.i2492 = fmul float %history.i.i701.sroa.13.06421, %_73.i.i.i.i1042, !dbg !19654
  %_0.i2068 = fadd float %_0.i2072, %_0.i2492, !dbg !19656
  %_0.i2491 = fmul float %history.i.i701.sroa.13.06421, %_76.i.i.i.i1045, !dbg !19658
  %_0.i2067 = fadd float %_0.i2071, %_0.i2491, !dbg !19660
  %_0.i2490 = fmul float %history.i.i701.sroa.16.06422, %_81.i.i.i.i1050, !dbg !19662
  %_0.i2066 = fadd float %_0.i2070, %_0.i2490, !dbg !19664
  %_0.i2489 = fmul float %history.i.i701.sroa.16.06422, %_84.i.i.i.i1053, !dbg !19666
  %_0.i2065 = fadd float %_0.i2069, %_0.i2489, !dbg !19668
  %_0.i2488 = fmul float %history.i.i701.sroa.16.06422, %_87.i.i.i.i1056, !dbg !19670
  %_0.i2064 = fadd float %_0.i2068, %_0.i2488, !dbg !19672
  %_0.i2487 = fmul float %history.i.i701.sroa.16.06422, %_90.i.i.i.i1059, !dbg !19674
  %_0.i2063 = fadd float %_0.i2067, %_0.i2487, !dbg !19676
  %_0.i2486 = fmul float %history.i.i701.sroa.19.06423, %_95.i.i.i.i1064, !dbg !19678
  %_0.i2062 = fadd float %_0.i2066, %_0.i2486, !dbg !19680
  %_0.i2485 = fmul float %history.i.i701.sroa.19.06423, %_98.i.i.i.i1067, !dbg !19682
  %_0.i2061 = fadd float %_0.i2065, %_0.i2485, !dbg !19684
  %_0.i2484 = fmul float %history.i.i701.sroa.19.06423, %_101.i.i.i.i1070, !dbg !19686
  %_0.i2060 = fadd float %_0.i2064, %_0.i2484, !dbg !19688
  %_0.i2483 = fmul float %history.i.i701.sroa.19.06423, %_104.i.i.i.i1073, !dbg !19690
  %_0.i2059 = fadd float %_0.i2063, %_0.i2483, !dbg !19692
  %_0.i2482 = fmul float %history.i.i701.sroa.22.06424, %_109.i.i.i.i1078, !dbg !19694
  %_0.i2058 = fadd float %_0.i2062, %_0.i2482, !dbg !19696
  %_0.i2481 = fmul float %history.i.i701.sroa.22.06424, %_112.i.i.i.i1081, !dbg !19698
  %_0.i2057 = fadd float %_0.i2061, %_0.i2481, !dbg !19700
  %_0.i2480 = fmul float %history.i.i701.sroa.22.06424, %_115.i.i.i.i1084, !dbg !19702
  %_0.i2056 = fadd float %_0.i2060, %_0.i2480, !dbg !19704
  %_0.i2479 = fmul float %history.i.i701.sroa.22.06424, %_118.i.i.i.i1087, !dbg !19706
  %_0.i2055 = fadd float %_0.i2059, %_0.i2479, !dbg !19708
  %_0.i2478 = fmul float %history.i.i701.sroa.26.06425, %_123.i.i.i.i1092, !dbg !19710
  %_0.i2054 = fadd float %_0.i2058, %_0.i2478, !dbg !19712
  %_0.i2477 = fmul float %history.i.i701.sroa.26.06425, %_126.i.i.i.i1095, !dbg !19714
  %_0.i2053 = fadd float %_0.i2057, %_0.i2477, !dbg !19716
  %_0.i2476 = fmul float %history.i.i701.sroa.26.06425, %_129.i.i.i.i1098, !dbg !19718
  %_0.i2052 = fadd float %_0.i2056, %_0.i2476, !dbg !19720
  %_0.i2475 = fmul float %history.i.i701.sroa.26.06425, %_132.i.i.i.i1101, !dbg !19722
  %_0.i2051 = fadd float %_0.i2055, %_0.i2475, !dbg !19724
  %_0.i2474 = fmul float %history.i.i701.sroa.29.06426, %_137.i.i.i.i1106, !dbg !19726
  %_0.i2050 = fadd float %_0.i2054, %_0.i2474, !dbg !19728
  %_0.i2473 = fmul float %history.i.i701.sroa.29.06426, %_140.i.i.i.i1109, !dbg !19730
  %_0.i2049 = fadd float %_0.i2053, %_0.i2473, !dbg !19732
  %_0.i2472 = fmul float %history.i.i701.sroa.29.06426, %_143.i.i.i.i1112, !dbg !19734
  %_0.i2048 = fadd float %_0.i2052, %_0.i2472, !dbg !19736
  %_0.i2471 = fmul float %history.i.i701.sroa.29.06426, %_146.i.i.i.i1115, !dbg !19738
  %_0.i2047 = fadd float %_0.i2051, %_0.i2471, !dbg !19740
  %_0.i2470 = fmul float %history.i.i701.sroa.32.06427, %_151.i.i.i.i1120, !dbg !19742
  %_0.i2046 = fadd float %_0.i2050, %_0.i2470, !dbg !19744
  %_0.i2469 = fmul float %history.i.i701.sroa.32.06427, %_154.i.i.i.i1123, !dbg !19746
  %_0.i2045 = fadd float %_0.i2049, %_0.i2469, !dbg !19748
  %_0.i2468 = fmul float %history.i.i701.sroa.32.06427, %_157.i.i.i.i1126, !dbg !19750
  %_0.i2044 = fadd float %_0.i2048, %_0.i2468, !dbg !19752
  %_0.i2467 = fmul float %history.i.i701.sroa.32.06427, %_160.i.i.i.i1129, !dbg !19754
  %_0.i2043 = fadd float %_0.i2047, %_0.i2467, !dbg !19756
  %_0.i2466 = fmul float %history.i.i701.sroa.35.06428, %_165.i.i.i.i1134, !dbg !19758
  %_0.i2042 = fadd float %_0.i2046, %_0.i2466, !dbg !19760
  %_0.i2465 = fmul float %history.i.i701.sroa.35.06428, %_168.i.i.i.i1137, !dbg !19762
  %_0.i2041 = fadd float %_0.i2045, %_0.i2465, !dbg !19764
  %_0.i2464 = fmul float %history.i.i701.sroa.35.06428, %_171.i.i.i.i1140, !dbg !19766
  %_0.i2040 = fadd float %_0.i2044, %_0.i2464, !dbg !19768
  %_0.i2463 = fmul float %history.i.i701.sroa.35.06428, %_174.i.i.i.i1143, !dbg !19770
  %_0.i2039 = fadd float %_0.i2043, %_0.i2463, !dbg !19772
  %176 = tail call noundef float @llvm.fabs.f32(float %_0.i2042), !dbg !19774
  %_3.i.i3557.inv = fcmp ogt float %175, %176, !dbg !19776
  %_4.i.i3564.v = select i1 %_3.i.i3557.inv, float %175, float %176, !dbg !19776
  %177 = tail call noundef float @llvm.fabs.f32(float %_0.i2041), !dbg !19774
  %_3.i.i3557.inv.1 = fcmp ogt float %_4.i.i3564.v, %177, !dbg !19776
  %_4.i.i3564.v.1 = select i1 %_3.i.i3557.inv.1, float %_4.i.i3564.v, float %177, !dbg !19776
  %178 = tail call noundef float @llvm.fabs.f32(float %_0.i2040), !dbg !19774
  %_3.i.i3557.inv.2 = fcmp ogt float %_4.i.i3564.v.1, %178, !dbg !19776
  %_4.i.i3564.v.2 = select i1 %_3.i.i3557.inv.2, float %_4.i.i3564.v.1, float %178, !dbg !19776
  %179 = tail call noundef float @llvm.fabs.f32(float %_0.i2039), !dbg !19774
  %_3.i.i3557.inv.3 = fcmp ogt float %_4.i.i3564.v.2, %179, !dbg !19776
  %_4.i.i3564.v.3 = select i1 %_3.i.i3557.inv.3, float %_4.i.i3564.v.2, float %179, !dbg !19776
  %_39.i.i1154 = getelementptr inbounds nuw float, ptr %peaks_right.i704, i32 %iter.sroa.0.0.i.i7346429, !dbg !19779
  store float %_4.i.i3564.v.3, ptr %_39.i.i1154, align 4, !dbg !19784, !alias.scope !19786, !noalias !19577
  %exitcond11899.not = icmp eq i32 %174, %umax11911, !dbg !19789
  br i1 %exitcond11899.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i736, label %bb5.i.i962, !dbg !19557

bb7.i.i1159:                                      ; preds = %bb5.i.i962
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_11.i126.i, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #33, !dbg !19791, !noalias !19577
  unreachable, !dbg !19791

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i736: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i
  %history.i.i701.sroa.0.0.lcssa = phi float [ %history.i.i701.sroa.0.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %_0.i2915, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ], !dbg !19792
  %history.i.i701.sroa.7.0.lcssa = phi float [ %history.i.i701.sroa.7.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i701.sroa.0.06418, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ], !dbg !19792
  %history.i.i701.sroa.10.0.lcssa = phi float [ %history.i.i701.sroa.10.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i701.sroa.7.06419, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ], !dbg !19792
  %history.i.i701.sroa.13.0.lcssa = phi float [ %history.i.i701.sroa.13.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i701.sroa.10.06420, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ], !dbg !19792
  %history.i.i701.sroa.16.0.lcssa = phi float [ %history.i.i701.sroa.16.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i701.sroa.13.06421, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ], !dbg !19792
  %history.i.i701.sroa.19.0.lcssa = phi float [ %history.i.i701.sroa.19.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i701.sroa.16.06422, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ], !dbg !19792
  %history.i.i701.sroa.22.0.lcssa = phi float [ %history.i.i701.sroa.22.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i701.sroa.19.06423, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ], !dbg !19792
  %history.i.i701.sroa.26.0.lcssa = phi float [ %history.i.i701.sroa.26.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i701.sroa.22.06424, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ], !dbg !19792
  %history.i.i701.sroa.29.0.lcssa = phi float [ %history.i.i701.sroa.29.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i701.sroa.26.06425, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ], !dbg !19792
  %history.i.i701.sroa.32.0.lcssa = phi float [ %history.i.i701.sroa.32.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i701.sroa.29.06426, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ], !dbg !19792
  %history.i.i701.sroa.35.0.lcssa = phi float [ %history.i.i701.sroa.35.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i701.sroa.32.06427, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ], !dbg !19792
  %history.i.i701.sroa.38.0.lcssa = phi float [ %history.i.i701.sroa.38.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i701.sroa.35.06428, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ], !dbg !19792
  store float %history.i.i701.sroa.0.0.lcssa, ptr %hot_right.i707, align 4, !dbg !19793, !noalias !19552
  store float %history.i.i701.sroa.7.0.lcssa, ptr %history.i.i701.sroa.7.0.hot_right.i707.sroa_idx, align 4, !dbg !19793, !noalias !19552
  store float %history.i.i701.sroa.10.0.lcssa, ptr %history.i.i701.sroa.10.0.hot_right.i707.sroa_idx, align 4, !dbg !19793, !noalias !19552
  store float %history.i.i701.sroa.13.0.lcssa, ptr %history.i.i701.sroa.13.0.hot_right.i707.sroa_idx, align 4, !dbg !19793, !noalias !19552
  store float %history.i.i701.sroa.16.0.lcssa, ptr %history.i.i701.sroa.16.0.hot_right.i707.sroa_idx, align 4, !dbg !19793, !noalias !19552
  store float %history.i.i701.sroa.19.0.lcssa, ptr %history.i.i701.sroa.19.0.hot_right.i707.sroa_idx, align 4, !dbg !19793, !noalias !19552
  store float %history.i.i701.sroa.22.0.lcssa, ptr %history.i.i701.sroa.22.0.hot_right.i707.sroa_idx, align 4, !dbg !19793, !noalias !19552
  store float %history.i.i701.sroa.26.0.lcssa, ptr %history.i.i701.sroa.26.0.hot_right.i707.sroa_idx, align 4, !dbg !19793, !noalias !19552
  store float %history.i.i701.sroa.29.0.lcssa, ptr %history.i.i701.sroa.29.0.hot_right.i707.sroa_idx, align 4, !dbg !19793, !noalias !19552
  store float %history.i.i701.sroa.32.0.lcssa, ptr %history.i.i701.sroa.32.0.hot_right.i707.sroa_idx, align 4, !dbg !19793, !noalias !19552
  store float %history.i.i701.sroa.35.0.lcssa, ptr %history.i.i701.sroa.35.0.hot_right.i707.sroa_idx, align 4, !dbg !19793, !noalias !19552
  store float %history.i.i701.sroa.38.0.lcssa, ptr %history.i.i701.sroa.38.0.hot_right.i707.sroa_idx, align 4, !dbg !19793, !noalias !19552
  br i1 %_20.i139.i6391.not, label %bb13.i725.loopexit, label %bb42.i742.lr.ph, !dbg !19229

bb42.i742.lr.ph:                                  ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i736
  %_13.i188545024504 = load float, ptr %123, align 4, !alias.scope !19172, !noalias !19175, !noundef !10
  %_13.i187345054507 = load float, ptr %126, align 4, !alias.scope !19183, !noalias !19186, !noundef !10
  %_13.i186145084510 = load float, ptr %129, align 4, !alias.scope !19193, !noalias !19196, !noundef !10
  %_13.i185045114513 = load float, ptr %132, align 4, !alias.scope !19203, !noalias !19186, !noundef !10
  %_87.i763 = load i32, ptr %133, align 4
  %_62.i89.i820 = load float, ptr %144, align 4
  %_62.i.i913 = load float, ptr %160, align 4
  %_102.i945 = load i32, ptr %164, align 4
  %.promoted = load float, ptr %121, align 4, !alias.scope !19172, !noalias !19175
  %_64.i744.promoted = load float, ptr %_64.i744, align 4, !alias.scope !19172, !noalias !19175
  %.promoted6609 = load float, ptr %122, align 4, !alias.scope !19172, !noalias !19175
  %.promoted6679 = load float, ptr %124, align 4, !alias.scope !19183, !noalias !19186
  %_65.i745.promoted = load float, ptr %_65.i745, align 4, !alias.scope !19183, !noalias !19186
  %.promoted6817 = load float, ptr %125, align 4, !alias.scope !19183, !noalias !19186
  %.promoted6887 = load float, ptr %127, align 4, !alias.scope !19193, !noalias !19196
  %_69.i746.promoted = load float, ptr %_69.i746, align 4, !alias.scope !19193, !noalias !19196
  %.promoted7025 = load float, ptr %128, align 4, !alias.scope !19193, !noalias !19196
  %.promoted7095 = load float, ptr %130, align 4, !alias.scope !19203, !noalias !19186
  %_70.i747.promoted = load float, ptr %_70.i747, align 4, !alias.scope !19203, !noalias !19186
  %.promoted7233 = load float, ptr %131, align 4, !alias.scope !19203, !noalias !19186
  %.promoted7303 = load float, ptr %143, align 4
  %.promoted7372 = load float, ptr %145, align 4
  %.promoted7441 = load float, ptr %159, align 4
  %.promoted7510 = load float, ptr %161, align 4
  br label %bb42.i742, !dbg !19229

bb42.i742:                                        ; preds = %bb42.i742.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077
  %_0.i32087511 = phi float [ %.promoted7510, %bb42.i742.lr.ph ], [ %_0.i3208, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ]
  %_0.i28347442 = phi float [ %.promoted7441, %bb42.i742.lr.ph ], [ %_0.i2834, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ]
  %_0.i32127373 = phi float [ %.promoted7372, %bb42.i742.lr.ph ], [ %_0.i3212, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ]
  %_0.i28387304 = phi float [ %.promoted7303, %bb42.i742.lr.ph ], [ %_0.i2838, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ]
  %_12.i18487234 = phi float [ %.promoted7233, %bb42.i742.lr.ph ], [ %_0.i3331, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ], !dbg !19794
  %_0.i33387165 = phi float [ %_70.i747.promoted, %bb42.i742.lr.ph ], [ %_0.i3338, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ], !dbg !19794
  %_5.i18457096 = phi float [ %.promoted7095, %bb42.i742.lr.ph ], [ %_0.i.i3548, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ], !dbg !19794
  %_12.i18597026 = phi float [ %.promoted7025, %bb42.i742.lr.ph ], [ %_0.i3318, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ], !dbg !19794
  %_0.i33256957 = phi float [ %_69.i746.promoted, %bb42.i742.lr.ph ], [ %_0.i3325, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ], !dbg !19794
  %_5.i18536888 = phi float [ %.promoted6887, %bb42.i742.lr.ph ], [ %_0.i.i3541, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ], !dbg !19794
  %_12.i18716818 = phi float [ %.promoted6817, %bb42.i742.lr.ph ], [ %_0.i3305, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ], !dbg !19794
  %_0.i33126749 = phi float [ %_65.i745.promoted, %bb42.i742.lr.ph ], [ %_0.i3312, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ], !dbg !19794
  %_5.i18656680 = phi float [ %.promoted6679, %bb42.i742.lr.ph ], [ %_0.i.i3534, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ], !dbg !19794
  %_12.i18836610 = phi float [ %.promoted6609, %bb42.i742.lr.ph ], [ %_0.i3292, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ], !dbg !19794
  %_0.i32996541 = phi float [ %_64.i744.promoted, %bb42.i742.lr.ph ], [ %_0.i3299, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ], !dbg !19794
  %_5.i18776472 = phi float [ %.promoted, %bb42.i742.lr.ph ], [ %_0.i.i3527, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ], !dbg !19794
  %main_cursor.sroa.0.1.i7406469 = phi i32 [ %main_cursor.sroa.0.0.i7297583, %bb42.i742.lr.ph ], [ %spec.store.select11.i947, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ]
  %ring_cursor.sroa.0.1.i7396468 = phi i32 [ %ring_cursor.sroa.0.0.i7287582, %bb42.i742.lr.ph ], [ %spec.store.select12.i949, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ]
  %iter1.sroa.0.0.i7386467 = phi i32 [ 0, %bb42.i742.lr.ph ], [ %180, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ]
  %180 = add nuw nsw i32 %iter1.sroa.0.0.i7386467, 1, !dbg !19794
  %_60.i743 = add nuw nsw i32 %iter1.sroa.0.0.i7386467, %iter.sroa.0.0.i7267580, !dbg !19800
  %_0.i2827 = fadd float %_5.i18776472, -1.000000e+00, !dbg !19801
  %_3.i.i3521 = fcmp ogt float %_0.i2827, 0.000000e+00, !dbg !19804
  %_0.i.i3527 = select i1 %_3.i.i3521, float %_0.i2827, float 0.000000e+00, !dbg !19808
  %_0.i1987 = fadd float %_0.i32996541, %_12.i18836610, !dbg !19810
  %_0.i3299 = select i1 %_3.i.i3521, float %_0.i1987, float %_13.i188545024504, !dbg !19812
  %_0.i3292 = select i1 %_3.i.i3521, float %_12.i18836610, float 0.000000e+00, !dbg !19814
  %_0.i2828 = fadd float %_5.i18656680, -1.000000e+00, !dbg !19816
  %_3.i.i3528 = fcmp ogt float %_0.i2828, 0.000000e+00, !dbg !19818
  %_0.i.i3534 = select i1 %_3.i.i3528, float %_0.i2828, float 0.000000e+00, !dbg !19821
  %_0.i1988 = fadd float %_0.i33126749, %_12.i18716818, !dbg !19823
  %_0.i3312 = select i1 %_3.i.i3528, float %_0.i1988, float %_13.i187345054507, !dbg !19825
  %_0.i3305 = select i1 %_3.i.i3528, float %_12.i18716818, float 0.000000e+00, !dbg !19827
  %_0.i2829 = fadd float %_5.i18536888, -1.000000e+00, !dbg !19829
  %_3.i.i3535 = fcmp ogt float %_0.i2829, 0.000000e+00, !dbg !19831
  %_0.i.i3541 = select i1 %_3.i.i3535, float %_0.i2829, float 0.000000e+00, !dbg !19834
  %_0.i1989 = fadd float %_0.i33256957, %_12.i18597026, !dbg !19836
  %_0.i3325 = select i1 %_3.i.i3535, float %_0.i1989, float %_13.i186145084510, !dbg !19838
  %_0.i3318 = select i1 %_3.i.i3535, float %_12.i18597026, float 0.000000e+00, !dbg !19840
  %_0.i2830 = fadd float %_5.i18457096, -1.000000e+00, !dbg !19842
  %_3.i.i3542 = fcmp ogt float %_0.i2830, 0.000000e+00, !dbg !19844
  %_0.i.i3548 = select i1 %_3.i.i3542, float %_0.i2830, float 0.000000e+00, !dbg !19847
  %_0.i1990 = fadd float %_0.i33387165, %_12.i18487234, !dbg !19849
  %_0.i3338 = select i1 %_3.i.i3542, float %_0.i1990, float %_13.i185045114513, !dbg !19851
  %_0.i3331 = select i1 %_3.i.i3542, float %_12.i18487234, float 0.000000e+00, !dbg !19853
  %_126.i751 = getelementptr inbounds nuw float, ptr %peaks_left.i705, i32 %iter1.sroa.0.0.i7386467, !dbg !19855
  %_0.i2953 = load float, ptr %_126.i751, align 4, !dbg !19866, !alias.scope !19868, !noalias !19186, !noundef !10
  %_131.i753 = getelementptr inbounds nuw float, ptr %peaks_right.i704, i32 %iter1.sroa.0.0.i7386467, !dbg !19871
  %_0.i2948 = load float, ptr %_131.i753, align 4, !dbg !19881, !alias.scope !19883, !noalias !19186, !noundef !10
  %_3.i.i3584 = fcmp ule float %_0.i2948, %_0.i2953, !dbg !19886
  %_6.i.i3586 = bitcast float %_0.i2948 to i32, !dbg !19889
  %_8.i.i3588 = bitcast float %_0.i2953 to i32, !dbg !19893
  %_4.i.i3591 = select i1 %_3.i.i3584, i32 %_8.i.i3588, i32 %_6.i.i3586, !dbg !19895
  %_5.i3372 = and i32 %_4.i.i3591, %.none.i713, !dbg !19896
  %_7.i3375 = and i32 %_9.i3374, %_8.i.i3588, !dbg !19898
  %_4.i3376 = or disjoint i32 %_5.i3372, %_7.i3375, !dbg !19896
  %_0.i3377 = bitcast i32 %_4.i3376 to float, !dbg !19899
  %_7.i3368 = and i32 %_9.i3374, %_6.i.i3586, !dbg !19902
  %_4.i3369 = or disjoint i32 %_5.i3372, %_7.i3368, !dbg !19904
  %_0.i3370 = bitcast i32 %_4.i3369 to float, !dbg !19905
  %_132.i758 = icmp ugt i32 %_60.i743, %left_io.1, !dbg !19907
  br i1 %_132.i758, label %bb46.i961, label %bb47.i759, !dbg !19907, !prof !902

bb47.i759:                                        ; preds = %bb42.i742
  %_139.i761 = getelementptr inbounds nuw float, ptr %left_io.0, i32 %_60.i743, !dbg !19911
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19916), !dbg !19919
  %_3.not.i2941 = icmp eq i32 %left_io.1, %_60.i743, !dbg !19920
  br i1 %_3.not.i2941, label %panic.i2944, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2945, !dbg !19920

panic.i2944:                                      ; preds = %bb47.i759
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #33, !dbg !19920, !noalias !19922
  unreachable, !dbg !19920

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2945: ; preds = %bb47.i759
  %_0.i2943 = load float, ptr %_139.i761, align 4, !dbg !19920, !alias.scope !19916, !noalias !19186, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19923), !dbg !19926
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19927), !dbg !19926
  %width.i33.i764 = load i32, ptr %134, align 4, !dbg !19929, !alias.scope !19930, !noalias !19931, !noundef !10
  %_3.i1953 = fcmp uge float %_0.i3299, %_0.i3377, !dbg !19934
  %_0.i2386 = fdiv float %_0.i3299, %_0.i3377, !dbg !19936
  %_0.i3363 = select i1 %_3.i1953, float 1.000000e+00, float %_0.i2386, !dbg !19939
  %_158.1.i38.i769 = load i32, ptr %135, align 4, !dbg !19941, !alias.scope !19930, !noalias !19931, !noundef !10
  %_22.i39.i770 = mul i32 %width.i33.i764, %ring_cursor.sroa.0.1.i7396468, !dbg !19942
  %_90.i40.i771 = icmp ugt i32 %_22.i39.i770, %_158.1.i38.i769, !dbg !19943
  br i1 %_90.i40.i771, label %bb34.i124.i960, label %bb35.i41.i772, !dbg !19943, !prof !902

bb35.i41.i772:                                    ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2945
  %_158.0.i42.i773 = load ptr, ptr %136, align 4, !dbg !19941, !alias.scope !19930, !noalias !19931, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19948), !dbg !19951
  %_4.not.i3102 = icmp eq i32 %_158.1.i38.i769, %_22.i39.i770, !dbg !19952
  br i1 %_4.not.i3102, label %panic.i3104, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3105, !dbg !19952

panic.i3104:                                      ; preds = %bb35.i41.i772
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #33, !dbg !19952, !noalias !19954
  unreachable, !dbg !19952

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3105: ; preds = %bb35.i41.i772
  %_97.i44.i775 = getelementptr inbounds nuw float, ptr %_158.0.i42.i773, i32 %_22.i39.i770, !dbg !19955
  store float %_0.i3363, ptr %_97.i44.i775, align 4, !dbg !19952, !alias.scope !19948, !noalias !19960
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19961), !dbg !19964
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19965), !dbg !19964
  %width.i1477 = load i32, ptr %134, align 4, !dbg !19967, !alias.scope !19961, !noalias !19969, !noundef !10
  %181 = icmp eq i32 %width.i1477, 0, !dbg !19970
  br i1 %181, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1582, label %bb29.i1483.lr.ph, !dbg !19970

bb29.i1483.lr.ph:                                 ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3105
  %_126.1.i1488 = load i32, ptr %62, align 4, !alias.scope !19961, !noalias !19969, !noundef !10
  %_126.0.i1492 = load ptr, ptr %61, align 4, !nonnull !10
  %182 = add i32 %ring_cursor.sroa.0.1.i7396468, 1
  %_21.not.i1497 = icmp ult i32 %182, %_87.i763
  %183 = select i1 %_21.not.i1497, i32 0, i32 %_87.i763
  %start1.sroa.0.0.i1498 = sub nuw i32 %182, %183
  %_128.1.i1501 = load i32, ptr %135, align 4
  %_128.0.i1505 = load ptr, ptr %136, align 4, !nonnull !10
  %_130.1.i1506 = load i32, ptr %137, align 4
  %_130.0.i1510 = load ptr, ptr %138, align 4, !nonnull !10
  %_132.1.i1513 = load i32, ptr %139, align 4
  %_132.0.i1517 = load ptr, ptr %140, align 4, !nonnull !10
  %_43.i1530 = mul i32 %width.i1477, %start1.sroa.0.0.i1498
  br label %bb29.i1483, !dbg !19970

bb29.i1483:                                       ; preds = %bb29.i1483.lr.ph, %bb28.i1545
  %iter.sroa.0.0.idx.i14816448 = phi i32 [ 0, %bb29.i1483.lr.ph ], [ %iter.sroa.0.0.add.i1486, %bb28.i1545 ]
  %iter.sroa.4.0.i14806447 = phi i32 [ 0, %bb29.i1483.lr.ph ], [ %_102.0.i1487, %bb28.i1545 ]
  %iter.sroa.7.0.i14796446 = phi i32 [ %width.i1477, %bb29.i1483.lr.ph ], [ %184, %bb28.i1545 ]
  %iter.sroa.0.0.ptr.i14826449 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 %iter.sroa.0.0.idx.i14816448, !dbg !19972
  %184 = add i32 %iter.sroa.7.0.i14796446, -1, !dbg !19972
  %_109.i1484 = icmp eq i32 %iter.sroa.0.0.idx.i14816448, 32, !dbg !19973
  br i1 %_109.i1484, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1582, label %bb33.i1485, !dbg !19977

bb33.i1485:                                       ; preds = %bb29.i1483
  %iter.sroa.0.0.add.i1486 = add nuw nsw i32 %iter.sroa.0.0.idx.i14816448, 4, !dbg !19978
  %_102.0.i1487 = add nuw nsw i32 %iter.sroa.4.0.i14806447, 1, !dbg !19980
  %exitcond11901.not = icmp eq i32 %iter.sroa.4.0.i14806447, %_126.1.i1488, !dbg !19981
  br i1 %exitcond11901.not, label %panic.i1490, label %bb2.i1491, !dbg !19981

bb2.i1491:                                        ; preds = %bb33.i1485
  %185 = getelementptr inbounds nuw %LaneShape, ptr %_126.0.i1492, i32 %iter.sroa.4.0.i14806447, !dbg !19981
  %shape.i1493 = load i32, ptr %185, align 4, !dbg !19981, !noalias !19982, !noundef !10
  %186 = getelementptr inbounds nuw i8, ptr %185, i32 4, !dbg !19981
  %shape3.i1494 = load i32, ptr %186, align 4, !dbg !19981, !noalias !19982, !noundef !10
  %187 = add i32 %shape3.i1494, %ring_cursor.sroa.0.1.i7396468, !dbg !19983
  %_18.not.i1495 = icmp ult i32 %187, %_87.i763, !dbg !19984
  %188 = select i1 %_18.not.i1495, i32 0, i32 %_87.i763, !dbg !19984
  %spec.select.i1496 = sub nuw i32 %187, %188, !dbg !19984
  %_25.i1499 = mul i32 %spec.select.i1496, %width.i1477, !dbg !19985
  %_24.i1500 = add i32 %_25.i1499, %iter.sroa.4.0.i14806447, !dbg !19985
  %_28.i1502 = icmp ult i32 %_24.i1500, %_128.1.i1501, !dbg !19986
  br i1 %_28.i1502, label %bb9.i1504, label %panic5.i1503, !dbg !19986

panic.i1490:                                      ; preds = %bb33.i1485
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_126.1.i1488, i32 noundef %_126.1.i1488, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_12856b5f9033764dbf23e04eb77c5c46) #33, !dbg !19981, !noalias !19982
  unreachable, !dbg !19981

bb9.i1504:                                        ; preds = %bb2.i1491
  %189 = getelementptr inbounds nuw float, ptr %_128.0.i1505, i32 %_24.i1500, !dbg !19986
  %190 = load float, ptr %189, align 4, !dbg !19986, !noalias !19982, !noundef !10
  %exitcond11902.not = icmp eq i32 %iter.sroa.4.0.i14806447, %_130.1.i1506, !dbg !19987
  br i1 %exitcond11902.not, label %panic6.i1508, label %bb10.i1509, !dbg !19987

panic5.i1503:                                     ; preds = %bb2.i1491
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_24.i1500, i32 noundef %_128.1.i1501, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70ebf0cf1c90c6161926611ccf249a32) #33, !dbg !19986, !noalias !19982
  unreachable, !dbg !19986

bb10.i1509:                                       ; preds = %bb9.i1504
  %191 = getelementptr inbounds nuw i32, ptr %_130.0.i1510, i32 %iter.sroa.4.0.i14806447, !dbg !19987
  %_30.i1511 = load i32, ptr %191, align 4, !dbg !19987, !noalias !19982, !noundef !10
  %192 = icmp eq i32 %_30.i1511, 0, !dbg !19988
  br i1 %192, label %bb14.i1520, label %bb12.i1512, !dbg !19988

panic6.i1508:                                     ; preds = %bb9.i1504
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_130.1.i1506, i32 noundef %_130.1.i1506, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_023d591aaad7f5cf482ac80a9401eca1) #33, !dbg !19987, !noalias !19982
  unreachable, !dbg !19987

bb12.i1512:                                       ; preds = %bb10.i1509
  %_35.i1514 = icmp ult i32 %iter.sroa.4.0.i14806447, %_132.1.i1513, !dbg !19989
  br i1 %_35.i1514, label %bb13.i1516, label %panic7.i1515, !dbg !19989

bb14.i1520:                                       ; preds = %bb34.i1581, %bb13.i1516, %bb10.i1509
  %newest.sroa.0.0.i1521 = phi float [ %190, %bb10.i1509 ], [ %_33.i1518, %bb34.i1581 ], [ %190, %bb13.i1516 ], !dbg !19990
  %exitcond11903.not = icmp eq i32 %iter.sroa.4.0.i14806447, %_132.1.i1513, !dbg !19991
  br i1 %exitcond11903.not, label %panic8.i1524, label %bb15.i1525, !dbg !19991

bb13.i1516:                                       ; preds = %bb12.i1512
  %193 = getelementptr inbounds nuw float, ptr %_132.0.i1517, i32 %iter.sroa.4.0.i14806447, !dbg !19989
  %_33.i1518 = load float, ptr %193, align 4, !dbg !19989, !noalias !19982, !noundef !10
  %_116.i1519 = fcmp olt float %_33.i1518, %190, !dbg !19992
  br i1 %_116.i1519, label %bb34.i1581, label %bb14.i1520, !dbg !19992

panic7.i1515:                                     ; preds = %bb12.i1512
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %iter.sroa.4.0.i14806447, i32 noundef %_132.1.i1513, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a228cac3279aae3a34886f59f51fbe9e) #33, !dbg !19989, !noalias !19982
  unreachable, !dbg !19989

bb34.i1581:                                       ; preds = %bb13.i1516
  br label %bb14.i1520, !dbg !19994

bb15.i1525:                                       ; preds = %bb14.i1520
  %194 = getelementptr inbounds nuw float, ptr %_132.0.i1517, i32 %iter.sroa.4.0.i14806447, !dbg !19991
  store float %newest.sroa.0.0.i1521, ptr %194, align 4, !dbg !19991, !noalias !19982
  %_40.i1527 = add i32 %_30.i1511, 1, !dbg !19995
  %complete.i1528 = icmp eq i32 %_40.i1527, %shape.i1493, !dbg !19995
  br i1 %complete.i1528, label %bb19.i1550, label %bb17.i1529, !dbg !19996

panic8.i1524:                                     ; preds = %bb14.i1520
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_132.1.i1513, i32 noundef %_132.1.i1513, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1fa5d88c1a2cc292a2767c48606a38fe) #33, !dbg !19991, !noalias !19982
  unreachable, !dbg !19991

bb17.i1529:                                       ; preds = %bb15.i1525
  %_42.i1531 = add i32 %iter.sroa.4.0.i14806447, %_43.i1530, !dbg !19997
  %_45.i1533 = icmp ult i32 %_42.i1531, %_128.1.i1501, !dbg !19998
  br i1 %_45.i1533, label %bb27.i1543, label %panic9.i1534, !dbg !19998

panic9.i1534:                                     ; preds = %bb17.i1529
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_42.i1531, i32 noundef %_128.1.i1501, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70d40ae2302f3ea19fa0c4a65fe82786) #33, !dbg !19998, !noalias !19982
  unreachable, !dbg !19998

bb27.i1543:                                       ; preds = %bb17.i1529
  %195 = getelementptr inbounds nuw float, ptr %_128.0.i1505, i32 %_42.i1531, !dbg !19998
  %_41.i1537 = load float, ptr %195, align 4, !dbg !19998, !noalias !19982, !noundef !10
  %_117.i1538 = fcmp olt float %_41.i1537, %newest.sroa.0.0.i1521, !dbg !19999
  %newest.sroa.0.1.i1539 = select i1 %_117.i1538, float %_41.i1537, float %newest.sroa.0.0.i1521, !dbg !19999
  store float %newest.sroa.0.1.i1539, ptr %iter.sroa.0.0.ptr.i14826449, align 4, !dbg !20001, !alias.scope !19965, !noalias !20002
  br label %bb28.i1545, !dbg !20003

bb28.i1545:                                       ; preds = %bb22.i1578, %bb19.i1550, %bb27.i1543
  %storemerge = phi i32 [ %_40.i1527, %bb27.i1543 ], [ 0, %bb19.i1550 ], [ 0, %bb22.i1578 ], !dbg !20004
  store i32 %storemerge, ptr %191, align 4, !dbg !20004, !noalias !19982
  %196 = icmp eq i32 %184, 0, !dbg !19970
  br i1 %196, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1582, label %bb29.i1483, !dbg !19970

bb19.i1550:                                       ; preds = %bb15.i1525
  store float %newest.sroa.0.0.i1521, ptr %iter.sroa.0.0.ptr.i14826449, align 4, !dbg !20001, !alias.scope !19965, !noalias !20002
  %_118.i15566442.not = icmp eq i32 %shape.i1493, 0, !dbg !20005
  br i1 %_118.i15566442.not, label %bb28.i1545, label %bb40.i1563.preheader, !dbg !20009

bb40.i1563.preheader:                             ; preds = %bb19.i1550
  %197 = load float, ptr %189, align 4, !dbg !20010, !noalias !19982, !noundef !10
  br label %bb40.i1563, !dbg !20011

bb40.i1563:                                       ; preds = %bb40.i1563.preheader, %bb22.i1578
  %iter2.sroa.0.0.i15556445 = phi i32 [ %_119.i1564, %bb22.i1578 ], [ 0, %bb40.i1563.preheader ]
  %suffix.sroa.0.0.i15546444 = phi float [ %suffix.sroa.0.1.i1574, %bb22.i1578 ], [ %197, %bb40.i1563.preheader ]
  %end.sroa.0.1.i15536443 = phi i32 [ %200, %bb22.i1578 ], [ %spec.select.i1496, %bb40.i1563.preheader ]
  %_54.i1565 = mul i32 %end.sroa.0.1.i15536443, %width.i1477, !dbg !20012
  %_53.i1566 = add i32 %_54.i1565, %iter.sroa.4.0.i14806447, !dbg !20012
  %_57.i1568 = icmp ult i32 %_53.i1566, %_128.1.i1501, !dbg !20011
  br i1 %_57.i1568, label %bb22.i1578, label %panic13.i1569, !dbg !20011

panic13.i1569:                                    ; preds = %bb40.i1563
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_53.i1566, i32 noundef %_128.1.i1501, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a271bbb7fd83c3605ea58a06b7065fa4) #33, !dbg !20011, !noalias !19982
  unreachable, !dbg !20011

bb22.i1578:                                       ; preds = %bb40.i1563
  %_119.i1564 = add nuw i32 %iter2.sroa.0.0.i15556445, 1, !dbg !20013
  %198 = getelementptr inbounds nuw float, ptr %_128.0.i1505, i32 %_53.i1566, !dbg !20011
  %_52.i1572 = load float, ptr %198, align 4, !dbg !20011, !noalias !19982, !noundef !10
  %_121.i1573 = fcmp olt float %suffix.sroa.0.0.i15546444, %_52.i1572, !dbg !20016
  %suffix.sroa.0.1.i1574 = select i1 %_121.i1573, float %suffix.sroa.0.0.i15546444, float %_52.i1572, !dbg !20016
  store float %suffix.sroa.0.1.i1574, ptr %198, align 4, !dbg !20018, !noalias !19982
  %199 = icmp eq i32 %end.sroa.0.1.i15536443, 0, !dbg !20019
  %spec.store.select.i1580 = select i1 %199, i32 %_87.i763, i32 %end.sroa.0.1.i15536443, !dbg !20019
  %200 = add i32 %spec.store.select.i1580, -1, !dbg !20020
  %exitcond11900.not = icmp eq i32 %_119.i1564, %shape.i1493, !dbg !20005
  br i1 %exitcond11900.not, label %bb28.i1545, label %bb40.i1563, !dbg !20009

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1582: ; preds = %bb29.i1483, %bb28.i1545, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3105
  %_0.i2940 = load float, ptr %scratch.i706, align 4, !dbg !20021, !alias.scope !20023, !noalias !20026, !noundef !10
  %_0.i2516 = fmul float %_0.i2940, 1.638400e+04, !dbg !20027
  %201 = tail call noundef float @llvm.floor.f32(float %_0.i2516), !dbg !20029
  %_0.i2515 = fmul float %201, 0x3F10000000000000, !dbg !20036
  %202 = icmp eq i32 %width.i33.i764, 0, !dbg !20038
  %_163.1.i82.i813.pre = load i32, ptr %141, align 4, !dbg !20043, !alias.scope !19930, !noalias !19931
  br i1 %202, label %bb53.i77.i808, label %bb36.i56.i787.lr.ph, !dbg !20038

bb36.i56.i787.lr.ph:                              ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1582
  %_159.1.i61.i792 = load i32, ptr %62, align 4, !alias.scope !19930, !noalias !19931, !noundef !10
  %_159.0.i65.i796 = load ptr, ptr %61, align 4, !nonnull !10
  %_161.0.i75.i806 = load ptr, ptr %142, align 4, !nonnull !10
  %exitcond11904.not = icmp eq i32 %_159.1.i61.i792, 0, !dbg !20044
  br i1 %exitcond11904.not, label %panic.i63.i794, label %bb14.i64.i795, !dbg !20044

bb34.i124.i960:                                   ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2945
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i39.i770, i32 noundef %_158.1.i38.i769, i32 noundef %_158.1.i38.i769, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_db7c42041d7aaaba4839906f37294547) #33, !dbg !20046, !noalias !19960
  unreachable, !dbg !20046

bb53.i77.i808:                                    ; preds = %bb18.i74.i805.7, %bb18.i74.i805, %bb18.i74.i805.1, %bb18.i74.i805.2, %bb18.i74.i805.3, %bb18.i74.i805.4, %bb18.i74.i805.5, %bb18.i74.i805.6, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1582
  %_0.i2938 = phi float [ %_0.i2940, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1582 ], [ %_47.i76.i807, %bb18.i74.i805 ], [ %_47.i76.i807, %bb18.i74.i805.7 ], [ %_47.i76.i807, %bb18.i74.i805.6 ], [ %_47.i76.i807, %bb18.i74.i805.5 ], [ %_47.i76.i807, %bb18.i74.i805.4 ], [ %_47.i76.i807, %bb18.i74.i805.3 ], [ %_47.i76.i807, %bb18.i74.i805.2 ], [ %_47.i76.i807, %bb18.i74.i805.1 ], !dbg !20047
  %_0.i2088 = fadd float %_0.i2515, %_0.i28387304, !dbg !20049
  %_0.i2838 = fsub float %_0.i2088, %_0.i2938, !dbg !20051
  %_123.i83.i814 = icmp ugt i32 %_22.i39.i770, %_163.1.i82.i813.pre, !dbg !20053
  br i1 %_123.i83.i814, label %bb41.i123.i959, label %bb42.i84.i815, !dbg !20053, !prof !902

bb14.i64.i795:                                    ; preds = %bb36.i56.i787.lr.ph
  %203 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i796, i32 8, !dbg !20044
  %_42.i66.i797 = load i32, ptr %203, align 4, !dbg !20044, !noalias !20026, !noundef !10
  %204 = add i32 %_42.i66.i797, %ring_cursor.sroa.0.1.i7396468, !dbg !20057
  %_45.not.i67.i798 = icmp ult i32 %204, %_87.i763, !dbg !20058
  %205 = select i1 %_45.not.i67.i798, i32 0, i32 %_87.i763, !dbg !20058
  %spec.select.i68.i799 = sub nuw i32 %204, %205, !dbg !20058
  %_49.i69.i800 = mul i32 %spec.select.i68.i799, %width.i33.i764, !dbg !20060
  %_51.i72.i803 = icmp ult i32 %_49.i69.i800, %_163.1.i82.i813.pre, !dbg !20061
  br i1 %_51.i72.i803, label %bb18.i74.i805, label %panic1.i73.i804, !dbg !20061

panic.i63.i794:                                   ; preds = %bb36.i56.i787.7, %bb36.i56.i787.6, %bb36.i56.i787.5, %bb36.i56.i787.4, %bb36.i56.i787.3, %bb36.i56.i787.2, %bb36.i56.i787.1, %bb36.i56.i787.lr.ph
  %_159.1.i61.i792.lcssa.ph = phi i32 [ 7, %bb36.i56.i787.7 ], [ 6, %bb36.i56.i787.6 ], [ 5, %bb36.i56.i787.5 ], [ 4, %bb36.i56.i787.4 ], [ 3, %bb36.i56.i787.3 ], [ 2, %bb36.i56.i787.2 ], [ 1, %bb36.i56.i787.1 ], [ 0, %bb36.i56.i787.lr.ph ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_159.1.i61.i792.lcssa.ph, i32 noundef %_159.1.i61.i792.lcssa.ph, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_33746731a929bad63709ddedbca82f5f) #33, !dbg !20044, !noalias !20026
  unreachable, !dbg !20044

bb18.i74.i805:                                    ; preds = %bb14.i64.i795
  %206 = getelementptr inbounds nuw float, ptr %_161.0.i75.i806, i32 %_49.i69.i800, !dbg !20061
  %_47.i76.i807 = load float, ptr %206, align 4, !dbg !20061, !noalias !20026, !noundef !10
  store float %_47.i76.i807, ptr %scratch.i706, align 4, !dbg !20062, !alias.scope !19927, !noalias !20063
  %207 = icmp eq i32 %width.i33.i764, 1, !dbg !20038
  br i1 %207, label %bb53.i77.i808, label %bb36.i56.i787.1, !dbg !20038

bb36.i56.i787.1:                                  ; preds = %bb18.i74.i805
  %exitcond11904.1.not = icmp eq i32 %_159.1.i61.i792, 1, !dbg !20044
  br i1 %exitcond11904.1.not, label %panic.i63.i794, label %bb14.i64.i795.1, !dbg !20044

bb14.i64.i795.1:                                  ; preds = %bb36.i56.i787.1
  %208 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i796, i32 20, !dbg !20044
  %_42.i66.i797.1 = load i32, ptr %208, align 4, !dbg !20044, !noalias !20026, !noundef !10
  %209 = add i32 %_42.i66.i797.1, %ring_cursor.sroa.0.1.i7396468, !dbg !20057
  %_45.not.i67.i798.1 = icmp ult i32 %209, %_87.i763, !dbg !20058
  %210 = select i1 %_45.not.i67.i798.1, i32 0, i32 %_87.i763, !dbg !20058
  %spec.select.i68.i799.1 = sub nuw i32 %209, %210, !dbg !20058
  %_49.i69.i800.1 = mul i32 %spec.select.i68.i799.1, %width.i33.i764, !dbg !20060
  %_48.i70.i801.1 = add i32 %_49.i69.i800.1, 1, !dbg !20060
  %_51.i72.i803.1 = icmp ult i32 %_48.i70.i801.1, %_163.1.i82.i813.pre, !dbg !20061
  br i1 %_51.i72.i803.1, label %bb18.i74.i805.1, label %panic1.i73.i804, !dbg !20061

bb18.i74.i805.1:                                  ; preds = %bb14.i64.i795.1
  %211 = getelementptr inbounds nuw float, ptr %_161.0.i75.i806, i32 %_48.i70.i801.1, !dbg !20061
  %_47.i76.i807.1 = load float, ptr %211, align 4, !dbg !20061, !noalias !20026, !noundef !10
  store float %_47.i76.i807.1, ptr %iter.sroa.0.0.ptr.i55.i7866453.1, align 4, !dbg !20062, !alias.scope !19927, !noalias !20063
  %212 = icmp eq i32 %width.i33.i764, 2, !dbg !20038
  br i1 %212, label %bb53.i77.i808, label %bb36.i56.i787.2, !dbg !20038

bb36.i56.i787.2:                                  ; preds = %bb18.i74.i805.1
  %exitcond11904.2.not = icmp eq i32 %_159.1.i61.i792, 2, !dbg !20044
  br i1 %exitcond11904.2.not, label %panic.i63.i794, label %bb14.i64.i795.2, !dbg !20044

bb14.i64.i795.2:                                  ; preds = %bb36.i56.i787.2
  %213 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i796, i32 32, !dbg !20044
  %_42.i66.i797.2 = load i32, ptr %213, align 4, !dbg !20044, !noalias !20026, !noundef !10
  %214 = add i32 %_42.i66.i797.2, %ring_cursor.sroa.0.1.i7396468, !dbg !20057
  %_45.not.i67.i798.2 = icmp ult i32 %214, %_87.i763, !dbg !20058
  %215 = select i1 %_45.not.i67.i798.2, i32 0, i32 %_87.i763, !dbg !20058
  %spec.select.i68.i799.2 = sub nuw i32 %214, %215, !dbg !20058
  %_49.i69.i800.2 = mul i32 %spec.select.i68.i799.2, %width.i33.i764, !dbg !20060
  %_48.i70.i801.2 = add i32 %_49.i69.i800.2, 2, !dbg !20060
  %_51.i72.i803.2 = icmp ult i32 %_48.i70.i801.2, %_163.1.i82.i813.pre, !dbg !20061
  br i1 %_51.i72.i803.2, label %bb18.i74.i805.2, label %panic1.i73.i804, !dbg !20061

bb18.i74.i805.2:                                  ; preds = %bb14.i64.i795.2
  %216 = getelementptr inbounds nuw float, ptr %_161.0.i75.i806, i32 %_48.i70.i801.2, !dbg !20061
  %_47.i76.i807.2 = load float, ptr %216, align 4, !dbg !20061, !noalias !20026, !noundef !10
  store float %_47.i76.i807.2, ptr %iter.sroa.0.0.ptr.i55.i7866453.2, align 4, !dbg !20062, !alias.scope !19927, !noalias !20063
  %217 = icmp eq i32 %width.i33.i764, 3, !dbg !20038
  br i1 %217, label %bb53.i77.i808, label %bb36.i56.i787.3, !dbg !20038

bb36.i56.i787.3:                                  ; preds = %bb18.i74.i805.2
  %exitcond11904.3.not = icmp eq i32 %_159.1.i61.i792, 3, !dbg !20044
  br i1 %exitcond11904.3.not, label %panic.i63.i794, label %bb14.i64.i795.3, !dbg !20044

bb14.i64.i795.3:                                  ; preds = %bb36.i56.i787.3
  %218 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i796, i32 44, !dbg !20044
  %_42.i66.i797.3 = load i32, ptr %218, align 4, !dbg !20044, !noalias !20026, !noundef !10
  %219 = add i32 %_42.i66.i797.3, %ring_cursor.sroa.0.1.i7396468, !dbg !20057
  %_45.not.i67.i798.3 = icmp ult i32 %219, %_87.i763, !dbg !20058
  %220 = select i1 %_45.not.i67.i798.3, i32 0, i32 %_87.i763, !dbg !20058
  %spec.select.i68.i799.3 = sub nuw i32 %219, %220, !dbg !20058
  %_49.i69.i800.3 = mul i32 %spec.select.i68.i799.3, %width.i33.i764, !dbg !20060
  %_48.i70.i801.3 = add i32 %_49.i69.i800.3, 3, !dbg !20060
  %_51.i72.i803.3 = icmp ult i32 %_48.i70.i801.3, %_163.1.i82.i813.pre, !dbg !20061
  br i1 %_51.i72.i803.3, label %bb18.i74.i805.3, label %panic1.i73.i804, !dbg !20061

bb18.i74.i805.3:                                  ; preds = %bb14.i64.i795.3
  %221 = getelementptr inbounds nuw float, ptr %_161.0.i75.i806, i32 %_48.i70.i801.3, !dbg !20061
  %_47.i76.i807.3 = load float, ptr %221, align 4, !dbg !20061, !noalias !20026, !noundef !10
  store float %_47.i76.i807.3, ptr %iter.sroa.0.0.ptr.i55.i7866453.3, align 4, !dbg !20062, !alias.scope !19927, !noalias !20063
  %222 = icmp eq i32 %width.i33.i764, 4, !dbg !20038
  br i1 %222, label %bb53.i77.i808, label %bb36.i56.i787.4, !dbg !20038

bb36.i56.i787.4:                                  ; preds = %bb18.i74.i805.3
  %exitcond11904.4.not = icmp eq i32 %_159.1.i61.i792, 4, !dbg !20044
  br i1 %exitcond11904.4.not, label %panic.i63.i794, label %bb14.i64.i795.4, !dbg !20044

bb14.i64.i795.4:                                  ; preds = %bb36.i56.i787.4
  %223 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i796, i32 56, !dbg !20044
  %_42.i66.i797.4 = load i32, ptr %223, align 4, !dbg !20044, !noalias !20026, !noundef !10
  %224 = add i32 %_42.i66.i797.4, %ring_cursor.sroa.0.1.i7396468, !dbg !20057
  %_45.not.i67.i798.4 = icmp ult i32 %224, %_87.i763, !dbg !20058
  %225 = select i1 %_45.not.i67.i798.4, i32 0, i32 %_87.i763, !dbg !20058
  %spec.select.i68.i799.4 = sub nuw i32 %224, %225, !dbg !20058
  %_49.i69.i800.4 = mul i32 %spec.select.i68.i799.4, %width.i33.i764, !dbg !20060
  %_48.i70.i801.4 = add i32 %_49.i69.i800.4, 4, !dbg !20060
  %_51.i72.i803.4 = icmp ult i32 %_48.i70.i801.4, %_163.1.i82.i813.pre, !dbg !20061
  br i1 %_51.i72.i803.4, label %bb18.i74.i805.4, label %panic1.i73.i804, !dbg !20061

bb18.i74.i805.4:                                  ; preds = %bb14.i64.i795.4
  %226 = getelementptr inbounds nuw float, ptr %_161.0.i75.i806, i32 %_48.i70.i801.4, !dbg !20061
  %_47.i76.i807.4 = load float, ptr %226, align 4, !dbg !20061, !noalias !20026, !noundef !10
  store float %_47.i76.i807.4, ptr %iter.sroa.0.0.ptr.i55.i7866453.4, align 4, !dbg !20062, !alias.scope !19927, !noalias !20063
  %227 = icmp eq i32 %width.i33.i764, 5, !dbg !20038
  br i1 %227, label %bb53.i77.i808, label %bb36.i56.i787.5, !dbg !20038

bb36.i56.i787.5:                                  ; preds = %bb18.i74.i805.4
  %exitcond11904.5.not = icmp eq i32 %_159.1.i61.i792, 5, !dbg !20044
  br i1 %exitcond11904.5.not, label %panic.i63.i794, label %bb14.i64.i795.5, !dbg !20044

bb14.i64.i795.5:                                  ; preds = %bb36.i56.i787.5
  %228 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i796, i32 68, !dbg !20044
  %_42.i66.i797.5 = load i32, ptr %228, align 4, !dbg !20044, !noalias !20026, !noundef !10
  %229 = add i32 %_42.i66.i797.5, %ring_cursor.sroa.0.1.i7396468, !dbg !20057
  %_45.not.i67.i798.5 = icmp ult i32 %229, %_87.i763, !dbg !20058
  %230 = select i1 %_45.not.i67.i798.5, i32 0, i32 %_87.i763, !dbg !20058
  %spec.select.i68.i799.5 = sub nuw i32 %229, %230, !dbg !20058
  %_49.i69.i800.5 = mul i32 %spec.select.i68.i799.5, %width.i33.i764, !dbg !20060
  %_48.i70.i801.5 = add i32 %_49.i69.i800.5, 5, !dbg !20060
  %_51.i72.i803.5 = icmp ult i32 %_48.i70.i801.5, %_163.1.i82.i813.pre, !dbg !20061
  br i1 %_51.i72.i803.5, label %bb18.i74.i805.5, label %panic1.i73.i804, !dbg !20061

bb18.i74.i805.5:                                  ; preds = %bb14.i64.i795.5
  %231 = getelementptr inbounds nuw float, ptr %_161.0.i75.i806, i32 %_48.i70.i801.5, !dbg !20061
  %_47.i76.i807.5 = load float, ptr %231, align 4, !dbg !20061, !noalias !20026, !noundef !10
  store float %_47.i76.i807.5, ptr %iter.sroa.0.0.ptr.i55.i7866453.5, align 4, !dbg !20062, !alias.scope !19927, !noalias !20063
  %232 = icmp eq i32 %width.i33.i764, 6, !dbg !20038
  br i1 %232, label %bb53.i77.i808, label %bb36.i56.i787.6, !dbg !20038

bb36.i56.i787.6:                                  ; preds = %bb18.i74.i805.5
  %exitcond11904.6.not = icmp eq i32 %_159.1.i61.i792, 6, !dbg !20044
  br i1 %exitcond11904.6.not, label %panic.i63.i794, label %bb14.i64.i795.6, !dbg !20044

bb14.i64.i795.6:                                  ; preds = %bb36.i56.i787.6
  %233 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i796, i32 80, !dbg !20044
  %_42.i66.i797.6 = load i32, ptr %233, align 4, !dbg !20044, !noalias !20026, !noundef !10
  %234 = add i32 %_42.i66.i797.6, %ring_cursor.sroa.0.1.i7396468, !dbg !20057
  %_45.not.i67.i798.6 = icmp ult i32 %234, %_87.i763, !dbg !20058
  %235 = select i1 %_45.not.i67.i798.6, i32 0, i32 %_87.i763, !dbg !20058
  %spec.select.i68.i799.6 = sub nuw i32 %234, %235, !dbg !20058
  %_49.i69.i800.6 = mul i32 %spec.select.i68.i799.6, %width.i33.i764, !dbg !20060
  %_48.i70.i801.6 = add i32 %_49.i69.i800.6, 6, !dbg !20060
  %_51.i72.i803.6 = icmp ult i32 %_48.i70.i801.6, %_163.1.i82.i813.pre, !dbg !20061
  br i1 %_51.i72.i803.6, label %bb18.i74.i805.6, label %panic1.i73.i804, !dbg !20061

bb18.i74.i805.6:                                  ; preds = %bb14.i64.i795.6
  %236 = getelementptr inbounds nuw float, ptr %_161.0.i75.i806, i32 %_48.i70.i801.6, !dbg !20061
  %_47.i76.i807.6 = load float, ptr %236, align 4, !dbg !20061, !noalias !20026, !noundef !10
  store float %_47.i76.i807.6, ptr %iter.sroa.0.0.ptr.i55.i7866453.6, align 4, !dbg !20062, !alias.scope !19927, !noalias !20063
  %237 = icmp eq i32 %width.i33.i764, 7, !dbg !20038
  br i1 %237, label %bb53.i77.i808, label %bb36.i56.i787.7, !dbg !20038

bb36.i56.i787.7:                                  ; preds = %bb18.i74.i805.6
  %exitcond11904.7.not = icmp eq i32 %_159.1.i61.i792, 7, !dbg !20044
  br i1 %exitcond11904.7.not, label %panic.i63.i794, label %bb14.i64.i795.7, !dbg !20044

bb14.i64.i795.7:                                  ; preds = %bb36.i56.i787.7
  %238 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i796, i32 92, !dbg !20044
  %_42.i66.i797.7 = load i32, ptr %238, align 4, !dbg !20044, !noalias !20026, !noundef !10
  %239 = add i32 %_42.i66.i797.7, %ring_cursor.sroa.0.1.i7396468, !dbg !20057
  %_45.not.i67.i798.7 = icmp ult i32 %239, %_87.i763, !dbg !20058
  %240 = select i1 %_45.not.i67.i798.7, i32 0, i32 %_87.i763, !dbg !20058
  %spec.select.i68.i799.7 = sub nuw i32 %239, %240, !dbg !20058
  %_49.i69.i800.7 = mul i32 %spec.select.i68.i799.7, %width.i33.i764, !dbg !20060
  %_48.i70.i801.7 = add i32 %_49.i69.i800.7, 7, !dbg !20060
  %_51.i72.i803.7 = icmp ult i32 %_48.i70.i801.7, %_163.1.i82.i813.pre, !dbg !20061
  br i1 %_51.i72.i803.7, label %bb18.i74.i805.7, label %panic1.i73.i804, !dbg !20061

bb18.i74.i805.7:                                  ; preds = %bb14.i64.i795.7
  %241 = getelementptr inbounds nuw float, ptr %_161.0.i75.i806, i32 %_48.i70.i801.7, !dbg !20061
  %_47.i76.i807.7 = load float, ptr %241, align 4, !dbg !20061, !noalias !20026, !noundef !10
  store float %_47.i76.i807.7, ptr %iter.sroa.0.0.ptr.i55.i7866453.7, align 4, !dbg !20062, !alias.scope !19927, !noalias !20063
  br label %bb53.i77.i808, !dbg !20038

panic1.i73.i804:                                  ; preds = %bb14.i64.i795.7, %bb14.i64.i795.6, %bb14.i64.i795.5, %bb14.i64.i795.4, %bb14.i64.i795.3, %bb14.i64.i795.2, %bb14.i64.i795.1, %bb14.i64.i795
  %_48.i70.i801.lcssa.ph = phi i32 [ %_48.i70.i801.7, %bb14.i64.i795.7 ], [ %_48.i70.i801.6, %bb14.i64.i795.6 ], [ %_48.i70.i801.5, %bb14.i64.i795.5 ], [ %_48.i70.i801.4, %bb14.i64.i795.4 ], [ %_48.i70.i801.3, %bb14.i64.i795.3 ], [ %_48.i70.i801.2, %bb14.i64.i795.2 ], [ %_48.i70.i801.1, %bb14.i64.i795.1 ], [ %_49.i69.i800, %bb14.i64.i795 ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_48.i70.i801.lcssa.ph, i32 noundef %_163.1.i82.i813.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_0e76a26fbb751dfb8cf8a069b5b0d39a) #33, !dbg !20061, !noalias !20026
  unreachable, !dbg !20061

bb42.i84.i815:                                    ; preds = %bb53.i77.i808
  %_163.0.i85.i816 = load ptr, ptr %142, align 4, !dbg !20043, !alias.scope !19930, !noalias !19931, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20064), !dbg !20067
  %_4.not.i3098 = icmp eq i32 %_163.1.i82.i813.pre, %_22.i39.i770, !dbg !20068
  br i1 %_4.not.i3098, label %panic.i3100, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3101, !dbg !20068

panic.i3100:                                      ; preds = %bb42.i84.i815
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #33, !dbg !20068, !noalias !20070
  unreachable, !dbg !20068

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3101: ; preds = %bb42.i84.i815
  %_130.i87.i818 = getelementptr inbounds nuw float, ptr %_163.0.i85.i816, i32 %_22.i39.i770, !dbg !20071
  store float %_0.i2515, ptr %_130.i87.i818, align 4, !dbg !20068, !alias.scope !20064, !noalias !20026
  %_0.i2385 = fdiv float %_0.i2838, %_62.i89.i820, !dbg !20076
  %_0.i2837 = fsub float 1.000000e+00, %_0.i2385, !dbg !20078
  %_0.i2836 = fsub float %_0.i2837, %_0.i32127373, !dbg !20080
  %_4.i2401 = fmul float %_0.i3312, %_0.i2836, !dbg !20082
  %_0.i2402 = fadd float %_0.i32127373, %_4.i2401, !dbg !20082
  %_3.i.i3575.inv = fcmp ogt float %_0.i2837, %_0.i2402, !dbg !20085
  %_4.i.i3582.v = select i1 %_3.i.i3575.inv, float %_0.i2837, float %_0.i2402, !dbg !20085
  %242 = tail call noundef float @llvm.fabs.f32(float %_4.i.i3582.v), !dbg !20089
  %243 = fcmp uge float %242, 0x3BC79CA100000000, !dbg !20093
  %_0.i3212 = select i1 %243, float %_4.i.i3582.v, float 0.000000e+00, !dbg !20096
  %_0.i2835 = fsub float 1.000000e+00, %_0.i3212, !dbg !20097
  %_164.1.i101.i832 = load i32, ptr %146, align 4, !dbg !20099, !alias.scope !19930, !noalias !19931, !noundef !10
  %_74.i102.i833 = mul i32 %width.i33.i764, %main_cursor.sroa.0.1.i7406469, !dbg !20101
  %_134.i103.i834 = icmp ugt i32 %_74.i102.i833, %_164.1.i101.i832, !dbg !20102
  br i1 %_134.i103.i834, label %bb47.i122.i958, label %bb48.i104.i835, !dbg !20102, !prof !902

bb41.i123.i959:                                   ; preds = %bb53.i77.i808
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i39.i770, i32 noundef %_163.1.i82.i813.pre, i32 noundef %_163.1.i82.i813.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c2286ff41a33b8c11aa270fe215441de) #33, !dbg !20107, !noalias !20026
  unreachable, !dbg !20107

bb48.i104.i835:                                   ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3101
  %_164.0.i105.i836 = load ptr, ptr %147, align 4, !dbg !20099, !alias.scope !19930, !noalias !19931, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20108), !dbg !20111
  %_3.not.i2932 = icmp eq i32 %_164.1.i101.i832, %_74.i102.i833, !dbg !20112
  br i1 %_3.not.i2932, label %panic.i2935, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3093, !dbg !20112

panic.i2935:                                      ; preds = %bb48.i104.i835
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #33, !dbg !20112, !noalias !20114
  unreachable, !dbg !20112

bb47.i122.i958:                                   ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3101
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_74.i102.i833, i32 noundef %_164.1.i101.i832, i32 noundef %_164.1.i101.i832, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a3a21efd18b403ec25db545d522c479) #33, !dbg !20115, !noalias !20026
  unreachable, !dbg !20115

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3093: ; preds = %bb48.i104.i835
  %_141.i107.i838 = getelementptr inbounds nuw float, ptr %_164.0.i105.i836, i32 %_74.i102.i833, !dbg !20116
  %_0.i2934 = load float, ptr %_141.i107.i838, align 4, !dbg !20112, !alias.scope !20108, !noalias !20026, !noundef !10
  store float %_0.i2943, ptr %_141.i107.i838, align 4, !dbg !20121, !alias.scope !20124, !noalias !20026
  %_0.i2514 = fmul float %_0.i2835, %_0.i2934, !dbg !20127
  %_6.i3351 = bitcast float %_0.i2934 to i32, !dbg !20129
  %_5.i3352 = and i32 %_6.i3351, %all.sroa.0.0.i715, !dbg !20132
  %_8.i3353 = bitcast float %_0.i2514 to i32, !dbg !20133
  %_7.i3355 = and i32 %_9.i3354, %_8.i3353, !dbg !20135
  %_4.i3356 = or disjoint i32 %_7.i3355, %_5.i3352, !dbg !20132
  store i32 %_4.i3356, ptr %_139.i761, align 4, !dbg !20136, !alias.scope !20138, !noalias !20141
  %_140.i852 = icmp ugt i32 %_60.i743, %right_io.1, !dbg !20142
  br i1 %_140.i852, label %bb48.i955, label %bb49.i853, !dbg !20142, !prof !902

bb46.i961:                                        ; preds = %bb42.i742
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_60.i743, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_f462f4e49ffb40af04504eaf795aa606) #33, !dbg !20146, !noalias !19186
  unreachable, !dbg !20146

bb49.i853:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3093
  %_147.i855 = getelementptr inbounds nuw float, ptr %right_io.0, i32 %_60.i743, !dbg !20147
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20152), !dbg !20155
  %_3.not.i2927 = icmp eq i32 %right_io.1, %_60.i743, !dbg !20156
  br i1 %_3.not.i2927, label %panic.i2930, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2931, !dbg !20156

panic.i2930:                                      ; preds = %bb49.i853
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #33, !dbg !20156, !noalias !20158
  unreachable, !dbg !20156

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2931: ; preds = %bb49.i853
  %_0.i2929 = load float, ptr %_147.i855, align 4, !dbg !20156, !alias.scope !20152, !noalias !19186, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20159), !dbg !20162
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20163), !dbg !20162
  %width.i.i857 = load i32, ptr %148, align 4, !dbg !20165, !alias.scope !20166, !noalias !20167, !noundef !10
  %_3.i1951 = fcmp uge float %_0.i3325, %_0.i3370, !dbg !20170
  %_0.i2384 = fdiv float %_0.i3325, %_0.i3370, !dbg !20172
  %_0.i3350 = select i1 %_3.i1951, float 1.000000e+00, float %_0.i2384, !dbg !20174
  %_158.1.i.i862 = load i32, ptr %149, align 4, !dbg !20176, !alias.scope !20166, !noalias !20167, !noundef !10
  %_22.i.i863 = mul i32 %width.i.i857, %ring_cursor.sroa.0.1.i7396468, !dbg !20177
  %_90.i.i864 = icmp ugt i32 %_22.i.i863, %_158.1.i.i862, !dbg !20178
  br i1 %_90.i.i864, label %bb34.i.i954, label %bb35.i.i865, !dbg !20178, !prof !902

bb35.i.i865:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2931
  %_158.0.i.i866 = load ptr, ptr %150, align 4, !dbg !20176, !alias.scope !20166, !noalias !20167, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20181), !dbg !20184
  %_4.not.i3086 = icmp eq i32 %_158.1.i.i862, %_22.i.i863, !dbg !20185
  br i1 %_4.not.i3086, label %panic.i3088, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3089, !dbg !20185

panic.i3088:                                      ; preds = %bb35.i.i865
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #33, !dbg !20185, !noalias !20187
  unreachable, !dbg !20185

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3089: ; preds = %bb35.i.i865
  %_97.i.i868 = getelementptr inbounds nuw float, ptr %_158.0.i.i866, i32 %_22.i.i863, !dbg !20188
  store float %_0.i3350, ptr %_97.i.i868, align 4, !dbg !20185, !alias.scope !20181, !noalias !20190
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20191), !dbg !20194
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20195), !dbg !20194
  %width.i = load i32, ptr %148, align 4, !dbg !20197, !alias.scope !20191, !noalias !20199, !noundef !10
  %244 = icmp eq i32 %width.i, 0, !dbg !20200
  br i1 %244, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit, label %bb29.i.lr.ph, !dbg !20200

bb29.i.lr.ph:                                     ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3089
  %_126.1.i = load i32, ptr %151, align 4, !alias.scope !20191, !noalias !20199, !noundef !10
  %_126.0.i = load ptr, ptr %152, align 4, !nonnull !10
  %245 = add i32 %ring_cursor.sroa.0.1.i7396468, 1
  %_21.not.i = icmp ult i32 %245, %_87.i763
  %246 = select i1 %_21.not.i, i32 0, i32 %_87.i763
  %start1.sroa.0.0.i = sub nuw i32 %245, %246
  %_128.1.i = load i32, ptr %149, align 4
  %_128.0.i = load ptr, ptr %150, align 4, !nonnull !10
  %_130.1.i = load i32, ptr %153, align 4
  %_130.0.i = load ptr, ptr %154, align 4, !nonnull !10
  %_132.1.i = load i32, ptr %155, align 4
  %_132.0.i = load ptr, ptr %156, align 4, !nonnull !10
  %_43.i = mul i32 %width.i, %start1.sroa.0.0.i
  br label %bb29.i, !dbg !20200

bb29.i:                                           ; preds = %bb29.i.lr.ph, %bb28.i
  %iter.sroa.0.0.idx.i6460 = phi i32 [ 0, %bb29.i.lr.ph ], [ %iter.sroa.0.0.add.i, %bb28.i ]
  %iter.sroa.4.0.i6459 = phi i32 [ 0, %bb29.i.lr.ph ], [ %_102.0.i, %bb28.i ]
  %iter.sroa.7.0.i6458 = phi i32 [ %width.i, %bb29.i.lr.ph ], [ %247, %bb28.i ]
  %iter.sroa.0.0.ptr.i6461 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 %iter.sroa.0.0.idx.i6460, !dbg !20202
  %247 = add i32 %iter.sroa.7.0.i6458, -1, !dbg !20202
  %_109.i = icmp eq i32 %iter.sroa.0.0.idx.i6460, 32, !dbg !20203
  br i1 %_109.i, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit, label %bb33.i, !dbg !20207

bb33.i:                                           ; preds = %bb29.i
  %iter.sroa.0.0.add.i = add nuw nsw i32 %iter.sroa.0.0.idx.i6460, 4, !dbg !20208
  %_102.0.i = add nuw nsw i32 %iter.sroa.4.0.i6459, 1, !dbg !20210
  %exitcond11906.not = icmp eq i32 %iter.sroa.4.0.i6459, %_126.1.i, !dbg !20211
  br i1 %exitcond11906.not, label %panic.i, label %bb2.i1457, !dbg !20211

bb2.i1457:                                        ; preds = %bb33.i
  %248 = getelementptr inbounds nuw %LaneShape, ptr %_126.0.i, i32 %iter.sroa.4.0.i6459, !dbg !20211
  %shape.i = load i32, ptr %248, align 4, !dbg !20211, !noalias !20212, !noundef !10
  %249 = getelementptr inbounds nuw i8, ptr %248, i32 4, !dbg !20211
  %shape3.i = load i32, ptr %249, align 4, !dbg !20211, !noalias !20212, !noundef !10
  %250 = add i32 %shape3.i, %ring_cursor.sroa.0.1.i7396468, !dbg !20213
  %_18.not.i = icmp ult i32 %250, %_87.i763, !dbg !20214
  %251 = select i1 %_18.not.i, i32 0, i32 %_87.i763, !dbg !20214
  %spec.select.i = sub nuw i32 %250, %251, !dbg !20214
  %_25.i = mul i32 %spec.select.i, %width.i, !dbg !20215
  %_24.i = add i32 %_25.i, %iter.sroa.4.0.i6459, !dbg !20215
  %_28.i1458 = icmp ult i32 %_24.i, %_128.1.i, !dbg !20216
  br i1 %_28.i1458, label %bb9.i, label %panic5.i, !dbg !20216

panic.i:                                          ; preds = %bb33.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_126.1.i, i32 noundef %_126.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_12856b5f9033764dbf23e04eb77c5c46) #33, !dbg !20211, !noalias !20212
  unreachable, !dbg !20211

bb9.i:                                            ; preds = %bb2.i1457
  %252 = getelementptr inbounds nuw float, ptr %_128.0.i, i32 %_24.i, !dbg !20216
  %253 = load float, ptr %252, align 4, !dbg !20216, !noalias !20212, !noundef !10
  %exitcond11907.not = icmp eq i32 %iter.sroa.4.0.i6459, %_130.1.i, !dbg !20217
  br i1 %exitcond11907.not, label %panic6.i, label %bb10.i1460, !dbg !20217

panic5.i:                                         ; preds = %bb2.i1457
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_24.i, i32 noundef %_128.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70ebf0cf1c90c6161926611ccf249a32) #33, !dbg !20216, !noalias !20212
  unreachable, !dbg !20216

bb10.i1460:                                       ; preds = %bb9.i
  %254 = getelementptr inbounds nuw i32, ptr %_130.0.i, i32 %iter.sroa.4.0.i6459, !dbg !20217
  %_30.i1461 = load i32, ptr %254, align 4, !dbg !20217, !noalias !20212, !noundef !10
  %255 = icmp eq i32 %_30.i1461, 0, !dbg !20218
  br i1 %255, label %bb14.i, label %bb12.i1462, !dbg !20218

panic6.i:                                         ; preds = %bb9.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_130.1.i, i32 noundef %_130.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_023d591aaad7f5cf482ac80a9401eca1) #33, !dbg !20217, !noalias !20212
  unreachable, !dbg !20217

bb12.i1462:                                       ; preds = %bb10.i1460
  %_35.i1463 = icmp ult i32 %iter.sroa.4.0.i6459, %_132.1.i, !dbg !20219
  br i1 %_35.i1463, label %bb13.i1464, label %panic7.i, !dbg !20219

bb14.i:                                           ; preds = %bb34.i, %bb13.i1464, %bb10.i1460
  %newest.sroa.0.0.i = phi float [ %253, %bb10.i1460 ], [ %_33.i1465, %bb34.i ], [ %253, %bb13.i1464 ], !dbg !20220
  %exitcond11908.not = icmp eq i32 %iter.sroa.4.0.i6459, %_132.1.i, !dbg !20221
  br i1 %exitcond11908.not, label %panic8.i, label %bb15.i1467, !dbg !20221

bb13.i1464:                                       ; preds = %bb12.i1462
  %256 = getelementptr inbounds nuw float, ptr %_132.0.i, i32 %iter.sroa.4.0.i6459, !dbg !20219
  %_33.i1465 = load float, ptr %256, align 4, !dbg !20219, !noalias !20212, !noundef !10
  %_116.i1466 = fcmp olt float %_33.i1465, %253, !dbg !20222
  br i1 %_116.i1466, label %bb34.i, label %bb14.i, !dbg !20222

panic7.i:                                         ; preds = %bb12.i1462
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %iter.sroa.4.0.i6459, i32 noundef %_132.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a228cac3279aae3a34886f59f51fbe9e) #33, !dbg !20219, !noalias !20212
  unreachable, !dbg !20219

bb34.i:                                           ; preds = %bb13.i1464
  br label %bb14.i, !dbg !20224

bb15.i1467:                                       ; preds = %bb14.i
  %257 = getelementptr inbounds nuw float, ptr %_132.0.i, i32 %iter.sroa.4.0.i6459, !dbg !20221
  store float %newest.sroa.0.0.i, ptr %257, align 4, !dbg !20221, !noalias !20212
  %_40.i = add i32 %_30.i1461, 1, !dbg !20225
  %complete.i1468 = icmp eq i32 %_40.i, %shape.i, !dbg !20225
  br i1 %complete.i1468, label %bb19.i1471, label %bb17.i, !dbg !20226

panic8.i:                                         ; preds = %bb14.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_132.1.i, i32 noundef %_132.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1fa5d88c1a2cc292a2767c48606a38fe) #33, !dbg !20221, !noalias !20212
  unreachable, !dbg !20221

bb17.i:                                           ; preds = %bb15.i1467
  %_42.i = add i32 %iter.sroa.4.0.i6459, %_43.i, !dbg !20227
  %_45.i = icmp ult i32 %_42.i, %_128.1.i, !dbg !20228
  br i1 %_45.i, label %bb27.i, label %panic9.i, !dbg !20228

panic9.i:                                         ; preds = %bb17.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_42.i, i32 noundef %_128.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70d40ae2302f3ea19fa0c4a65fe82786) #33, !dbg !20228, !noalias !20212
  unreachable, !dbg !20228

bb27.i:                                           ; preds = %bb17.i
  %258 = getelementptr inbounds nuw float, ptr %_128.0.i, i32 %_42.i, !dbg !20228
  %_41.i = load float, ptr %258, align 4, !dbg !20228, !noalias !20212, !noundef !10
  %_117.i = fcmp olt float %_41.i, %newest.sroa.0.0.i, !dbg !20229
  %newest.sroa.0.1.i = select i1 %_117.i, float %_41.i, float %newest.sroa.0.0.i, !dbg !20229
  store float %newest.sroa.0.1.i, ptr %iter.sroa.0.0.ptr.i6461, align 4, !dbg !20231, !alias.scope !20195, !noalias !20232
  br label %bb28.i, !dbg !20233

bb28.i:                                           ; preds = %bb22.i, %bb19.i1471, %bb27.i
  %storemerge4518 = phi i32 [ %_40.i, %bb27.i ], [ 0, %bb19.i1471 ], [ 0, %bb22.i ], !dbg !20234
  store i32 %storemerge4518, ptr %254, align 4, !dbg !20234, !noalias !20212
  %259 = icmp eq i32 %247, 0, !dbg !20200
  br i1 %259, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit, label %bb29.i, !dbg !20200

bb19.i1471:                                       ; preds = %bb15.i1467
  store float %newest.sroa.0.0.i, ptr %iter.sroa.0.0.ptr.i6461, align 4, !dbg !20231, !alias.scope !20195, !noalias !20232
  %_118.i6454.not = icmp eq i32 %shape.i, 0, !dbg !20235
  br i1 %_118.i6454.not, label %bb28.i, label %bb40.i.preheader, !dbg !20239

bb40.i.preheader:                                 ; preds = %bb19.i1471
  %260 = load float, ptr %252, align 4, !dbg !20240, !noalias !20212, !noundef !10
  br label %bb40.i, !dbg !20241

bb40.i:                                           ; preds = %bb40.i.preheader, %bb22.i
  %iter2.sroa.0.0.i14746457 = phi i32 [ %_119.i, %bb22.i ], [ 0, %bb40.i.preheader ]
  %suffix.sroa.0.0.i14736456 = phi float [ %suffix.sroa.0.1.i, %bb22.i ], [ %260, %bb40.i.preheader ]
  %end.sroa.0.1.i6455 = phi i32 [ %263, %bb22.i ], [ %spec.select.i, %bb40.i.preheader ]
  %_54.i = mul i32 %end.sroa.0.1.i6455, %width.i, !dbg !20242
  %_53.i = add i32 %_54.i, %iter.sroa.4.0.i6459, !dbg !20242
  %_57.i = icmp ult i32 %_53.i, %_128.1.i, !dbg !20241
  br i1 %_57.i, label %bb22.i, label %panic13.i, !dbg !20241

panic13.i:                                        ; preds = %bb40.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_53.i, i32 noundef %_128.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a271bbb7fd83c3605ea58a06b7065fa4) #33, !dbg !20241, !noalias !20212
  unreachable, !dbg !20241

bb22.i:                                           ; preds = %bb40.i
  %_119.i = add nuw i32 %iter2.sroa.0.0.i14746457, 1, !dbg !20243
  %261 = getelementptr inbounds nuw float, ptr %_128.0.i, i32 %_53.i, !dbg !20241
  %_52.i = load float, ptr %261, align 4, !dbg !20241, !noalias !20212, !noundef !10
  %_121.i = fcmp olt float %suffix.sroa.0.0.i14736456, %_52.i, !dbg !20246
  %suffix.sroa.0.1.i = select i1 %_121.i, float %suffix.sroa.0.0.i14736456, float %_52.i, !dbg !20246
  store float %suffix.sroa.0.1.i, ptr %261, align 4, !dbg !20248, !noalias !20212
  %262 = icmp eq i32 %end.sroa.0.1.i6455, 0, !dbg !20249
  %spec.store.select.i1476 = select i1 %262, i32 %_87.i763, i32 %end.sroa.0.1.i6455, !dbg !20249
  %263 = add i32 %spec.store.select.i1476, -1, !dbg !20250
  %exitcond11905.not = icmp eq i32 %_119.i, %shape.i, !dbg !20235
  br i1 %exitcond11905.not, label %bb28.i, label %bb40.i, !dbg !20239

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit: ; preds = %bb28.i, %bb29.i
  %_0.i2926.pre = load float, ptr %scratch.i706, align 4, !dbg !20251, !alias.scope !20253, !noalias !20256
  br label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit, !dbg !20251

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit: ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3089
  %_0.i2926 = phi float [ %_0.i2926.pre, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit ], [ %_0.i2938, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3089 ], !dbg !20251
  %_0.i2513 = fmul float %_0.i2926, 1.638400e+04, !dbg !20257
  %264 = tail call noundef float @llvm.floor.f32(float %_0.i2513), !dbg !20259
  %_0.i2512 = fmul float %264, 0x3F10000000000000, !dbg !20263
  %265 = icmp eq i32 %width.i.i857, 0, !dbg !20265
  %_163.1.i.i906.pre = load i32, ptr %157, align 4, !dbg !20267, !alias.scope !20166, !noalias !20167
  br i1 %265, label %bb53.i.i901, label %bb36.i.i880.lr.ph, !dbg !20265

bb36.i.i880.lr.ph:                                ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit
  %_159.1.i.i885 = load i32, ptr %151, align 4, !alias.scope !20166, !noalias !20167, !noundef !10
  %_159.0.i.i889 = load ptr, ptr %152, align 4, !nonnull !10
  %_161.0.i.i899 = load ptr, ptr %158, align 4, !nonnull !10
  %exitcond11909.not = icmp eq i32 %_159.1.i.i885, 0, !dbg !20268
  br i1 %exitcond11909.not, label %panic.i.i887, label %bb14.i.i888, !dbg !20268

bb34.i.i954:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2931
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i863, i32 noundef %_158.1.i.i862, i32 noundef %_158.1.i.i862, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_db7c42041d7aaaba4839906f37294547) #33, !dbg !20269, !noalias !20190
  unreachable, !dbg !20269

bb53.i.i901:                                      ; preds = %bb18.i.i898.7, %bb18.i.i898, %bb18.i.i898.1, %bb18.i.i898.2, %bb18.i.i898.3, %bb18.i.i898.4, %bb18.i.i898.5, %bb18.i.i898.6, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit
  %_0.i2924 = phi float [ %_0.i2926, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit ], [ %_47.i.i900, %bb18.i.i898 ], [ %_47.i.i900, %bb18.i.i898.7 ], [ %_47.i.i900, %bb18.i.i898.6 ], [ %_47.i.i900, %bb18.i.i898.5 ], [ %_47.i.i900, %bb18.i.i898.4 ], [ %_47.i.i900, %bb18.i.i898.3 ], [ %_47.i.i900, %bb18.i.i898.2 ], [ %_47.i.i900, %bb18.i.i898.1 ], !dbg !20270
  %_0.i2087 = fadd float %_0.i2512, %_0.i28347442, !dbg !20272
  %_0.i2834 = fsub float %_0.i2087, %_0.i2924, !dbg !20274
  %_123.i.i907 = icmp ugt i32 %_22.i.i863, %_163.1.i.i906.pre, !dbg !20276
  br i1 %_123.i.i907, label %bb41.i.i953, label %bb42.i.i908, !dbg !20276, !prof !902

bb14.i.i888:                                      ; preds = %bb36.i.i880.lr.ph
  %266 = getelementptr inbounds nuw i8, ptr %_159.0.i.i889, i32 8, !dbg !20268
  %_42.i.i890 = load i32, ptr %266, align 4, !dbg !20268, !noalias !20256, !noundef !10
  %267 = add i32 %_42.i.i890, %ring_cursor.sroa.0.1.i7396468, !dbg !20279
  %_45.not.i.i891 = icmp ult i32 %267, %_87.i763, !dbg !20280
  %268 = select i1 %_45.not.i.i891, i32 0, i32 %_87.i763, !dbg !20280
  %spec.select.i.i892 = sub nuw i32 %267, %268, !dbg !20280
  %_49.i.i893 = mul i32 %spec.select.i.i892, %width.i.i857, !dbg !20281
  %_51.i.i896 = icmp ult i32 %_49.i.i893, %_163.1.i.i906.pre, !dbg !20282
  br i1 %_51.i.i896, label %bb18.i.i898, label %panic1.i.i897, !dbg !20282

panic.i.i887:                                     ; preds = %bb36.i.i880.7, %bb36.i.i880.6, %bb36.i.i880.5, %bb36.i.i880.4, %bb36.i.i880.3, %bb36.i.i880.2, %bb36.i.i880.1, %bb36.i.i880.lr.ph
  %_159.1.i.i885.lcssa.ph = phi i32 [ 7, %bb36.i.i880.7 ], [ 6, %bb36.i.i880.6 ], [ 5, %bb36.i.i880.5 ], [ 4, %bb36.i.i880.4 ], [ 3, %bb36.i.i880.3 ], [ 2, %bb36.i.i880.2 ], [ 1, %bb36.i.i880.1 ], [ 0, %bb36.i.i880.lr.ph ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_159.1.i.i885.lcssa.ph, i32 noundef %_159.1.i.i885.lcssa.ph, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_33746731a929bad63709ddedbca82f5f) #33, !dbg !20268, !noalias !20256
  unreachable, !dbg !20268

bb18.i.i898:                                      ; preds = %bb14.i.i888
  %269 = getelementptr inbounds nuw float, ptr %_161.0.i.i899, i32 %_49.i.i893, !dbg !20282
  %_47.i.i900 = load float, ptr %269, align 4, !dbg !20282, !noalias !20256, !noundef !10
  store float %_47.i.i900, ptr %scratch.i706, align 4, !dbg !20283, !alias.scope !20163, !noalias !20284
  %270 = icmp eq i32 %width.i.i857, 1, !dbg !20265
  br i1 %270, label %bb53.i.i901, label %bb36.i.i880.1, !dbg !20265

bb36.i.i880.1:                                    ; preds = %bb18.i.i898
  %exitcond11909.1.not = icmp eq i32 %_159.1.i.i885, 1, !dbg !20268
  br i1 %exitcond11909.1.not, label %panic.i.i887, label %bb14.i.i888.1, !dbg !20268

bb14.i.i888.1:                                    ; preds = %bb36.i.i880.1
  %271 = getelementptr inbounds nuw i8, ptr %_159.0.i.i889, i32 20, !dbg !20268
  %_42.i.i890.1 = load i32, ptr %271, align 4, !dbg !20268, !noalias !20256, !noundef !10
  %272 = add i32 %_42.i.i890.1, %ring_cursor.sroa.0.1.i7396468, !dbg !20279
  %_45.not.i.i891.1 = icmp ult i32 %272, %_87.i763, !dbg !20280
  %273 = select i1 %_45.not.i.i891.1, i32 0, i32 %_87.i763, !dbg !20280
  %spec.select.i.i892.1 = sub nuw i32 %272, %273, !dbg !20280
  %_49.i.i893.1 = mul i32 %spec.select.i.i892.1, %width.i.i857, !dbg !20281
  %_48.i.i894.1 = add i32 %_49.i.i893.1, 1, !dbg !20281
  %_51.i.i896.1 = icmp ult i32 %_48.i.i894.1, %_163.1.i.i906.pre, !dbg !20282
  br i1 %_51.i.i896.1, label %bb18.i.i898.1, label %panic1.i.i897, !dbg !20282

bb18.i.i898.1:                                    ; preds = %bb14.i.i888.1
  %274 = getelementptr inbounds nuw float, ptr %_161.0.i.i899, i32 %_48.i.i894.1, !dbg !20282
  %_47.i.i900.1 = load float, ptr %274, align 4, !dbg !20282, !noalias !20256, !noundef !10
  store float %_47.i.i900.1, ptr %iter.sroa.0.0.ptr.i.i8796465.1, align 4, !dbg !20283, !alias.scope !20163, !noalias !20284
  %275 = icmp eq i32 %width.i.i857, 2, !dbg !20265
  br i1 %275, label %bb53.i.i901, label %bb36.i.i880.2, !dbg !20265

bb36.i.i880.2:                                    ; preds = %bb18.i.i898.1
  %exitcond11909.2.not = icmp eq i32 %_159.1.i.i885, 2, !dbg !20268
  br i1 %exitcond11909.2.not, label %panic.i.i887, label %bb14.i.i888.2, !dbg !20268

bb14.i.i888.2:                                    ; preds = %bb36.i.i880.2
  %276 = getelementptr inbounds nuw i8, ptr %_159.0.i.i889, i32 32, !dbg !20268
  %_42.i.i890.2 = load i32, ptr %276, align 4, !dbg !20268, !noalias !20256, !noundef !10
  %277 = add i32 %_42.i.i890.2, %ring_cursor.sroa.0.1.i7396468, !dbg !20279
  %_45.not.i.i891.2 = icmp ult i32 %277, %_87.i763, !dbg !20280
  %278 = select i1 %_45.not.i.i891.2, i32 0, i32 %_87.i763, !dbg !20280
  %spec.select.i.i892.2 = sub nuw i32 %277, %278, !dbg !20280
  %_49.i.i893.2 = mul i32 %spec.select.i.i892.2, %width.i.i857, !dbg !20281
  %_48.i.i894.2 = add i32 %_49.i.i893.2, 2, !dbg !20281
  %_51.i.i896.2 = icmp ult i32 %_48.i.i894.2, %_163.1.i.i906.pre, !dbg !20282
  br i1 %_51.i.i896.2, label %bb18.i.i898.2, label %panic1.i.i897, !dbg !20282

bb18.i.i898.2:                                    ; preds = %bb14.i.i888.2
  %279 = getelementptr inbounds nuw float, ptr %_161.0.i.i899, i32 %_48.i.i894.2, !dbg !20282
  %_47.i.i900.2 = load float, ptr %279, align 4, !dbg !20282, !noalias !20256, !noundef !10
  store float %_47.i.i900.2, ptr %iter.sroa.0.0.ptr.i.i8796465.2, align 4, !dbg !20283, !alias.scope !20163, !noalias !20284
  %280 = icmp eq i32 %width.i.i857, 3, !dbg !20265
  br i1 %280, label %bb53.i.i901, label %bb36.i.i880.3, !dbg !20265

bb36.i.i880.3:                                    ; preds = %bb18.i.i898.2
  %exitcond11909.3.not = icmp eq i32 %_159.1.i.i885, 3, !dbg !20268
  br i1 %exitcond11909.3.not, label %panic.i.i887, label %bb14.i.i888.3, !dbg !20268

bb14.i.i888.3:                                    ; preds = %bb36.i.i880.3
  %281 = getelementptr inbounds nuw i8, ptr %_159.0.i.i889, i32 44, !dbg !20268
  %_42.i.i890.3 = load i32, ptr %281, align 4, !dbg !20268, !noalias !20256, !noundef !10
  %282 = add i32 %_42.i.i890.3, %ring_cursor.sroa.0.1.i7396468, !dbg !20279
  %_45.not.i.i891.3 = icmp ult i32 %282, %_87.i763, !dbg !20280
  %283 = select i1 %_45.not.i.i891.3, i32 0, i32 %_87.i763, !dbg !20280
  %spec.select.i.i892.3 = sub nuw i32 %282, %283, !dbg !20280
  %_49.i.i893.3 = mul i32 %spec.select.i.i892.3, %width.i.i857, !dbg !20281
  %_48.i.i894.3 = add i32 %_49.i.i893.3, 3, !dbg !20281
  %_51.i.i896.3 = icmp ult i32 %_48.i.i894.3, %_163.1.i.i906.pre, !dbg !20282
  br i1 %_51.i.i896.3, label %bb18.i.i898.3, label %panic1.i.i897, !dbg !20282

bb18.i.i898.3:                                    ; preds = %bb14.i.i888.3
  %284 = getelementptr inbounds nuw float, ptr %_161.0.i.i899, i32 %_48.i.i894.3, !dbg !20282
  %_47.i.i900.3 = load float, ptr %284, align 4, !dbg !20282, !noalias !20256, !noundef !10
  store float %_47.i.i900.3, ptr %iter.sroa.0.0.ptr.i.i8796465.3, align 4, !dbg !20283, !alias.scope !20163, !noalias !20284
  %285 = icmp eq i32 %width.i.i857, 4, !dbg !20265
  br i1 %285, label %bb53.i.i901, label %bb36.i.i880.4, !dbg !20265

bb36.i.i880.4:                                    ; preds = %bb18.i.i898.3
  %exitcond11909.4.not = icmp eq i32 %_159.1.i.i885, 4, !dbg !20268
  br i1 %exitcond11909.4.not, label %panic.i.i887, label %bb14.i.i888.4, !dbg !20268

bb14.i.i888.4:                                    ; preds = %bb36.i.i880.4
  %286 = getelementptr inbounds nuw i8, ptr %_159.0.i.i889, i32 56, !dbg !20268
  %_42.i.i890.4 = load i32, ptr %286, align 4, !dbg !20268, !noalias !20256, !noundef !10
  %287 = add i32 %_42.i.i890.4, %ring_cursor.sroa.0.1.i7396468, !dbg !20279
  %_45.not.i.i891.4 = icmp ult i32 %287, %_87.i763, !dbg !20280
  %288 = select i1 %_45.not.i.i891.4, i32 0, i32 %_87.i763, !dbg !20280
  %spec.select.i.i892.4 = sub nuw i32 %287, %288, !dbg !20280
  %_49.i.i893.4 = mul i32 %spec.select.i.i892.4, %width.i.i857, !dbg !20281
  %_48.i.i894.4 = add i32 %_49.i.i893.4, 4, !dbg !20281
  %_51.i.i896.4 = icmp ult i32 %_48.i.i894.4, %_163.1.i.i906.pre, !dbg !20282
  br i1 %_51.i.i896.4, label %bb18.i.i898.4, label %panic1.i.i897, !dbg !20282

bb18.i.i898.4:                                    ; preds = %bb14.i.i888.4
  %289 = getelementptr inbounds nuw float, ptr %_161.0.i.i899, i32 %_48.i.i894.4, !dbg !20282
  %_47.i.i900.4 = load float, ptr %289, align 4, !dbg !20282, !noalias !20256, !noundef !10
  store float %_47.i.i900.4, ptr %iter.sroa.0.0.ptr.i.i8796465.4, align 4, !dbg !20283, !alias.scope !20163, !noalias !20284
  %290 = icmp eq i32 %width.i.i857, 5, !dbg !20265
  br i1 %290, label %bb53.i.i901, label %bb36.i.i880.5, !dbg !20265

bb36.i.i880.5:                                    ; preds = %bb18.i.i898.4
  %exitcond11909.5.not = icmp eq i32 %_159.1.i.i885, 5, !dbg !20268
  br i1 %exitcond11909.5.not, label %panic.i.i887, label %bb14.i.i888.5, !dbg !20268

bb14.i.i888.5:                                    ; preds = %bb36.i.i880.5
  %291 = getelementptr inbounds nuw i8, ptr %_159.0.i.i889, i32 68, !dbg !20268
  %_42.i.i890.5 = load i32, ptr %291, align 4, !dbg !20268, !noalias !20256, !noundef !10
  %292 = add i32 %_42.i.i890.5, %ring_cursor.sroa.0.1.i7396468, !dbg !20279
  %_45.not.i.i891.5 = icmp ult i32 %292, %_87.i763, !dbg !20280
  %293 = select i1 %_45.not.i.i891.5, i32 0, i32 %_87.i763, !dbg !20280
  %spec.select.i.i892.5 = sub nuw i32 %292, %293, !dbg !20280
  %_49.i.i893.5 = mul i32 %spec.select.i.i892.5, %width.i.i857, !dbg !20281
  %_48.i.i894.5 = add i32 %_49.i.i893.5, 5, !dbg !20281
  %_51.i.i896.5 = icmp ult i32 %_48.i.i894.5, %_163.1.i.i906.pre, !dbg !20282
  br i1 %_51.i.i896.5, label %bb18.i.i898.5, label %panic1.i.i897, !dbg !20282

bb18.i.i898.5:                                    ; preds = %bb14.i.i888.5
  %294 = getelementptr inbounds nuw float, ptr %_161.0.i.i899, i32 %_48.i.i894.5, !dbg !20282
  %_47.i.i900.5 = load float, ptr %294, align 4, !dbg !20282, !noalias !20256, !noundef !10
  store float %_47.i.i900.5, ptr %iter.sroa.0.0.ptr.i.i8796465.5, align 4, !dbg !20283, !alias.scope !20163, !noalias !20284
  %295 = icmp eq i32 %width.i.i857, 6, !dbg !20265
  br i1 %295, label %bb53.i.i901, label %bb36.i.i880.6, !dbg !20265

bb36.i.i880.6:                                    ; preds = %bb18.i.i898.5
  %exitcond11909.6.not = icmp eq i32 %_159.1.i.i885, 6, !dbg !20268
  br i1 %exitcond11909.6.not, label %panic.i.i887, label %bb14.i.i888.6, !dbg !20268

bb14.i.i888.6:                                    ; preds = %bb36.i.i880.6
  %296 = getelementptr inbounds nuw i8, ptr %_159.0.i.i889, i32 80, !dbg !20268
  %_42.i.i890.6 = load i32, ptr %296, align 4, !dbg !20268, !noalias !20256, !noundef !10
  %297 = add i32 %_42.i.i890.6, %ring_cursor.sroa.0.1.i7396468, !dbg !20279
  %_45.not.i.i891.6 = icmp ult i32 %297, %_87.i763, !dbg !20280
  %298 = select i1 %_45.not.i.i891.6, i32 0, i32 %_87.i763, !dbg !20280
  %spec.select.i.i892.6 = sub nuw i32 %297, %298, !dbg !20280
  %_49.i.i893.6 = mul i32 %spec.select.i.i892.6, %width.i.i857, !dbg !20281
  %_48.i.i894.6 = add i32 %_49.i.i893.6, 6, !dbg !20281
  %_51.i.i896.6 = icmp ult i32 %_48.i.i894.6, %_163.1.i.i906.pre, !dbg !20282
  br i1 %_51.i.i896.6, label %bb18.i.i898.6, label %panic1.i.i897, !dbg !20282

bb18.i.i898.6:                                    ; preds = %bb14.i.i888.6
  %299 = getelementptr inbounds nuw float, ptr %_161.0.i.i899, i32 %_48.i.i894.6, !dbg !20282
  %_47.i.i900.6 = load float, ptr %299, align 4, !dbg !20282, !noalias !20256, !noundef !10
  store float %_47.i.i900.6, ptr %iter.sroa.0.0.ptr.i.i8796465.6, align 4, !dbg !20283, !alias.scope !20163, !noalias !20284
  %300 = icmp eq i32 %width.i.i857, 7, !dbg !20265
  br i1 %300, label %bb53.i.i901, label %bb36.i.i880.7, !dbg !20265

bb36.i.i880.7:                                    ; preds = %bb18.i.i898.6
  %exitcond11909.7.not = icmp eq i32 %_159.1.i.i885, 7, !dbg !20268
  br i1 %exitcond11909.7.not, label %panic.i.i887, label %bb14.i.i888.7, !dbg !20268

bb14.i.i888.7:                                    ; preds = %bb36.i.i880.7
  %301 = getelementptr inbounds nuw i8, ptr %_159.0.i.i889, i32 92, !dbg !20268
  %_42.i.i890.7 = load i32, ptr %301, align 4, !dbg !20268, !noalias !20256, !noundef !10
  %302 = add i32 %_42.i.i890.7, %ring_cursor.sroa.0.1.i7396468, !dbg !20279
  %_45.not.i.i891.7 = icmp ult i32 %302, %_87.i763, !dbg !20280
  %303 = select i1 %_45.not.i.i891.7, i32 0, i32 %_87.i763, !dbg !20280
  %spec.select.i.i892.7 = sub nuw i32 %302, %303, !dbg !20280
  %_49.i.i893.7 = mul i32 %spec.select.i.i892.7, %width.i.i857, !dbg !20281
  %_48.i.i894.7 = add i32 %_49.i.i893.7, 7, !dbg !20281
  %_51.i.i896.7 = icmp ult i32 %_48.i.i894.7, %_163.1.i.i906.pre, !dbg !20282
  br i1 %_51.i.i896.7, label %bb18.i.i898.7, label %panic1.i.i897, !dbg !20282

bb18.i.i898.7:                                    ; preds = %bb14.i.i888.7
  %304 = getelementptr inbounds nuw float, ptr %_161.0.i.i899, i32 %_48.i.i894.7, !dbg !20282
  %_47.i.i900.7 = load float, ptr %304, align 4, !dbg !20282, !noalias !20256, !noundef !10
  store float %_47.i.i900.7, ptr %iter.sroa.0.0.ptr.i.i8796465.7, align 4, !dbg !20283, !alias.scope !20163, !noalias !20284
  br label %bb53.i.i901, !dbg !20265

panic1.i.i897:                                    ; preds = %bb14.i.i888.7, %bb14.i.i888.6, %bb14.i.i888.5, %bb14.i.i888.4, %bb14.i.i888.3, %bb14.i.i888.2, %bb14.i.i888.1, %bb14.i.i888
  %_48.i.i894.lcssa.ph = phi i32 [ %_48.i.i894.7, %bb14.i.i888.7 ], [ %_48.i.i894.6, %bb14.i.i888.6 ], [ %_48.i.i894.5, %bb14.i.i888.5 ], [ %_48.i.i894.4, %bb14.i.i888.4 ], [ %_48.i.i894.3, %bb14.i.i888.3 ], [ %_48.i.i894.2, %bb14.i.i888.2 ], [ %_48.i.i894.1, %bb14.i.i888.1 ], [ %_49.i.i893, %bb14.i.i888 ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_48.i.i894.lcssa.ph, i32 noundef %_163.1.i.i906.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_0e76a26fbb751dfb8cf8a069b5b0d39a) #33, !dbg !20282, !noalias !20256
  unreachable, !dbg !20282

bb42.i.i908:                                      ; preds = %bb53.i.i901
  %_163.0.i.i909 = load ptr, ptr %158, align 4, !dbg !20267, !alias.scope !20166, !noalias !20167, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20285), !dbg !20288
  %_4.not.i3082 = icmp eq i32 %_163.1.i.i906.pre, %_22.i.i863, !dbg !20289
  br i1 %_4.not.i3082, label %panic.i3084, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3085, !dbg !20289

panic.i3084:                                      ; preds = %bb42.i.i908
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #33, !dbg !20289, !noalias !20291
  unreachable, !dbg !20289

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3085: ; preds = %bb42.i.i908
  %_130.i.i911 = getelementptr inbounds nuw float, ptr %_163.0.i.i909, i32 %_22.i.i863, !dbg !20292
  store float %_0.i2512, ptr %_130.i.i911, align 4, !dbg !20289, !alias.scope !20285, !noalias !20256
  %_0.i2383 = fdiv float %_0.i2834, %_62.i.i913, !dbg !20294
  %_0.i2833 = fsub float 1.000000e+00, %_0.i2383, !dbg !20296
  %_0.i2832 = fsub float %_0.i2833, %_0.i32087511, !dbg !20298
  %_4.i2399 = fmul float %_0.i3338, %_0.i2832, !dbg !20300
  %_0.i2400 = fadd float %_0.i32087511, %_4.i2399, !dbg !20300
  %_3.i.i3566.inv = fcmp ogt float %_0.i2833, %_0.i2400, !dbg !20302
  %_4.i.i3573.v = select i1 %_3.i.i3566.inv, float %_0.i2833, float %_0.i2400, !dbg !20302
  %305 = tail call noundef float @llvm.fabs.f32(float %_4.i.i3573.v), !dbg !20305
  %306 = fcmp uge float %305, 0x3BC79CA100000000, !dbg !20308
  %_0.i3208 = select i1 %306, float %_4.i.i3573.v, float 0.000000e+00, !dbg !20310
  %_0.i2831 = fsub float 1.000000e+00, %_0.i3208, !dbg !20311
  %_164.1.i.i925 = load i32, ptr %162, align 4, !dbg !20313, !alias.scope !20166, !noalias !20167, !noundef !10
  %_74.i.i926 = mul i32 %width.i.i857, %main_cursor.sroa.0.1.i7406469, !dbg !20314
  %_134.i.i927 = icmp ugt i32 %_74.i.i926, %_164.1.i.i925, !dbg !20315
  br i1 %_134.i.i927, label %bb47.i.i952, label %bb48.i.i928, !dbg !20315, !prof !902

bb41.i.i953:                                      ; preds = %bb53.i.i901
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i863, i32 noundef %_163.1.i.i906.pre, i32 noundef %_163.1.i.i906.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c2286ff41a33b8c11aa270fe215441de) #33, !dbg !20318, !noalias !20256
  unreachable, !dbg !20318

bb48.i.i928:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3085
  %_164.0.i.i929 = load ptr, ptr %163, align 4, !dbg !20313, !alias.scope !20166, !noalias !20167, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20319), !dbg !20322
  %_3.not.i2918 = icmp eq i32 %_164.1.i.i925, %_74.i.i926, !dbg !20323
  br i1 %_3.not.i2918, label %panic.i2921, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077, !dbg !20323

panic.i2921:                                      ; preds = %bb48.i.i928
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #33, !dbg !20323, !noalias !20325
  unreachable, !dbg !20323

bb47.i.i952:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3085
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_74.i.i926, i32 noundef %_164.1.i.i925, i32 noundef %_164.1.i.i925, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a3a21efd18b403ec25db545d522c479) #33, !dbg !20326, !noalias !20256
  unreachable, !dbg !20326

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077: ; preds = %bb48.i.i928
  %_141.i.i931 = getelementptr inbounds nuw float, ptr %_164.0.i.i929, i32 %_74.i.i926, !dbg !20327
  %_0.i2920 = load float, ptr %_141.i.i931, align 4, !dbg !20323, !alias.scope !20319, !noalias !20256, !noundef !10
  store float %_0.i2929, ptr %_141.i.i931, align 4, !dbg !20329, !alias.scope !20331, !noalias !20256
  %_0.i2511 = fmul float %_0.i2831, %_0.i2920, !dbg !20334
  %_6.i3339 = bitcast float %_0.i2920 to i32, !dbg !20336
  %_5.i3340 = and i32 %_6.i3339, %all.sroa.0.0.i715, !dbg !20339
  %_8.i3341 = bitcast float %_0.i2511 to i32, !dbg !20340
  %_7.i3342 = and i32 %_9.i3354, %_8.i3341, !dbg !20342
  %_4.i3343 = or disjoint i32 %_7.i3342, %_5.i3340, !dbg !20339
  store i32 %_4.i3343, ptr %_147.i855, align 4, !dbg !20343, !alias.scope !20345, !noalias !20348
  %307 = add i32 %main_cursor.sroa.0.1.i7406469, 1, !dbg !20349
  %_100.i946 = icmp eq i32 %307, %_102.i945, !dbg !20350
  %spec.store.select11.i947 = select i1 %_100.i946, i32 0, i32 %307, !dbg !20350
  %308 = add i32 %ring_cursor.sroa.0.1.i7396468, 1, !dbg !20351
  %_103.i948 = icmp eq i32 %308, %_87.i763, !dbg !20352
  %spec.store.select12.i949 = select i1 %_103.i948, i32 0, i32 %308, !dbg !20352
  %exitcond11912.not = icmp eq i32 %180, %umax11911, !dbg !20353
  br i1 %exitcond11912.not, label %bb16.i737.bb13.i725.loopexit_crit_edge, label %bb42.i742, !dbg !19229

bb48.i955:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3093
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_60.i743, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_3ec0ee57975400dd5dbd052e150c5392) #33, !dbg !20356, !noalias !19186
  unreachable, !dbg !20356

_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit: ; preds = %bb13.i725.loopexit, %bb12.i
  %ring_cursor.sroa.0.0.i728.lcssa = phi i32 [ %_37.i717, %bb12.i ], [ %ring_cursor.sroa.0.1.i739.lcssa, %bb13.i725.loopexit ], !dbg !19133
  %main_cursor.sroa.0.0.i729.lcssa = phi i32 [ %_35.i716, %bb12.i ], [ %main_cursor.sroa.0.1.i740.lcssa, %bb13.i725.loopexit ], !dbg !19130
  call void @llvm.lifetime.start.p0(ptr nonnull %_106.i703), !dbg !20357, !noalias !19113
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(92) %_106.i703, ptr noundef nonnull align 4 dereferenceable(92) %hot_left.i708, i32 92, i1 false), !dbg !20357, !noalias !19113
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_106.i703, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33) #32, !dbg !20358, !noalias !19186
  call void @llvm.lifetime.end.p0(ptr nonnull %_106.i703), !dbg !20359, !noalias !19113
  call void @llvm.lifetime.start.p0(ptr nonnull %_108.i702), !dbg !20360, !noalias !19113
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(92) %_108.i702, ptr noundef nonnull align 4 dereferenceable(92) %hot_right.i707, i32 92, i1 false), !dbg !20360, !noalias !19113
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_108.i702, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34) #32, !dbg !20361, !noalias !19186
  call void @llvm.lifetime.end.p0(ptr nonnull %_108.i702), !dbg !20362, !noalias !19113
  store i32 %main_cursor.sroa.0.0.i729.lcssa, ptr %_35, align 4, !dbg !20363, !alias.scope !19107, !noalias !19132
  store i32 %ring_cursor.sroa.0.0.i728.lcssa, ptr %81, align 4, !dbg !20364, !alias.scope !19107, !noalias !19132
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i704), !dbg !20365, !noalias !19113
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i705), !dbg !20366, !noalias !19113
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i706), !dbg !20367, !noalias !19113
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_right.i707), !dbg !20368, !noalias !19113
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i708), !dbg !20369, !noalias !19113
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockfEB2_.exit, !dbg !19102

bb11.i:                                           ; preds = %bb10.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20370), !dbg !20373
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20374), !dbg !20373
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20376), !dbg !20373
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20378), !dbg !20373
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i437), !dbg !20380, !noalias !20384
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_left.i437, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #32, !dbg !20388, !noalias !20389
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_right.i436), !dbg !20390, !noalias !20384
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_right.i436, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #32, !dbg !20392, !noalias !20393
  %309 = load i8, ptr %79, align 4, !dbg !20394, !range !4765, !alias.scope !20370, !noalias !20398, !noundef !10
  %310 = load i8, ptr %80, align 1, !dbg !20399, !range !4765, !alias.scope !20370, !noalias !20398, !noundef !10
  %_35.i = load i32, ptr %_35, align 4, !dbg !20401, !alias.scope !20378, !noalias !20403, !noundef !10
  %_37.i445 = load i32, ptr %81, align 4, !dbg !20404, !alias.scope !20378, !noalias !20403, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i), !dbg !20406, !noalias !20384
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i, i8 0, i32 32, i1 false), !noalias !20384
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i435), !dbg !20408, !noalias !20384
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i435, i8 0, i32 1024, i1 false), !noalias !20384
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i434), !dbg !20410, !noalias !20384
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i434, i8 0, i32 1024, i1 false), !noalias !20384
  %_32.i441 = zext nneg i8 %309 to i32, !dbg !20394
  %.none.i442 = sub nsw i32 0, %_32.i441, !dbg !20412
  %_33.i443 = zext nneg i8 %310 to i32, !dbg !20399
  %all.sroa.0.0.i444 = sub nsw i32 0, %_33.i443, !dbg !20399
  %_111.not.i7946 = icmp eq i32 %frames, 0, !dbg !20413
  br i1 %_111.not.i7946, label %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit, label %bb37.i.lr.ph, !dbg !20413

bb37.i.lr.ph:                                     ; preds = %bb11.i
  %d9.i4013 = lshr i32 %frames, 5, !dbg !20423
  %r2.i4014 = and i32 %frames, 31, !dbg !20430
  %_19.not.i4015 = icmp ne i32 %r2.i4014, 0, !dbg !20431
  %311 = zext i1 %_19.not.i4015 to i32, !dbg !20431
  %yield_count.sroa.0.0.i4016 = add nuw nsw i32 %d9.i4013, %311, !dbg !20431
  %history.i135.i.sroa.7.0.hot_left.i437.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i437, i32 4
  %history.i135.i.sroa.10.0.hot_left.i437.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i437, i32 8
  %history.i135.i.sroa.13.0.hot_left.i437.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i437, i32 12
  %history.i135.i.sroa.16.0.hot_left.i437.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i437, i32 16
  %history.i135.i.sroa.19.0.hot_left.i437.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i437, i32 20
  %history.i135.i.sroa.22.0.hot_left.i437.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i437, i32 24
  %history.i135.i.sroa.26.0.hot_left.i437.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i437, i32 28
  %history.i135.i.sroa.29.0.hot_left.i437.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i437, i32 32
  %history.i135.i.sroa.32.0.hot_left.i437.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i437, i32 36
  %history.i135.i.sroa.35.0.hot_left.i437.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i437, i32 40
  %history.i135.i.sroa.38.0.hot_left.i437.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i437, i32 44
  %312 = getelementptr inbounds nuw i8, ptr %self, i32 132
  %313 = getelementptr inbounds nuw i8, ptr %self, i32 136
  %314 = getelementptr inbounds nuw i8, ptr %self, i32 140
  %row1.i.i.i171.i = getelementptr inbounds nuw i8, ptr %self, i32 144
  %315 = getelementptr inbounds nuw i8, ptr %self, i32 148
  %316 = getelementptr inbounds nuw i8, ptr %self, i32 152
  %317 = getelementptr inbounds nuw i8, ptr %self, i32 156
  %row3.i.i.i185.i = getelementptr inbounds nuw i8, ptr %self, i32 160
  %318 = getelementptr inbounds nuw i8, ptr %self, i32 164
  %319 = getelementptr inbounds nuw i8, ptr %self, i32 168
  %320 = getelementptr inbounds nuw i8, ptr %self, i32 172
  %row5.i.i.i199.i = getelementptr inbounds nuw i8, ptr %self, i32 176
  %321 = getelementptr inbounds nuw i8, ptr %self, i32 180
  %322 = getelementptr inbounds nuw i8, ptr %self, i32 184
  %323 = getelementptr inbounds nuw i8, ptr %self, i32 188
  %row7.i.i.i213.i = getelementptr inbounds nuw i8, ptr %self, i32 192
  %324 = getelementptr inbounds nuw i8, ptr %self, i32 196
  %325 = getelementptr inbounds nuw i8, ptr %self, i32 200
  %326 = getelementptr inbounds nuw i8, ptr %self, i32 204
  %row9.i.i.i227.i = getelementptr inbounds nuw i8, ptr %self, i32 208
  %327 = getelementptr inbounds nuw i8, ptr %self, i32 212
  %328 = getelementptr inbounds nuw i8, ptr %self, i32 216
  %329 = getelementptr inbounds nuw i8, ptr %self, i32 220
  %row11.i.i.i241.i = getelementptr inbounds nuw i8, ptr %self, i32 224
  %330 = getelementptr inbounds nuw i8, ptr %self, i32 228
  %331 = getelementptr inbounds nuw i8, ptr %self, i32 232
  %332 = getelementptr inbounds nuw i8, ptr %self, i32 236
  %row13.i.i.i255.i = getelementptr inbounds nuw i8, ptr %self, i32 240
  %333 = getelementptr inbounds nuw i8, ptr %self, i32 244
  %334 = getelementptr inbounds nuw i8, ptr %self, i32 248
  %335 = getelementptr inbounds nuw i8, ptr %self, i32 252
  %row15.i.i.i269.i = getelementptr inbounds nuw i8, ptr %self, i32 256
  %336 = getelementptr inbounds nuw i8, ptr %self, i32 260
  %337 = getelementptr inbounds nuw i8, ptr %self, i32 264
  %338 = getelementptr inbounds nuw i8, ptr %self, i32 268
  %row17.i.i.i283.i = getelementptr inbounds nuw i8, ptr %self, i32 272
  %339 = getelementptr inbounds nuw i8, ptr %self, i32 276
  %340 = getelementptr inbounds nuw i8, ptr %self, i32 280
  %341 = getelementptr inbounds nuw i8, ptr %self, i32 284
  %row19.i.i.i297.i = getelementptr inbounds nuw i8, ptr %self, i32 288
  %342 = getelementptr inbounds nuw i8, ptr %self, i32 292
  %343 = getelementptr inbounds nuw i8, ptr %self, i32 296
  %344 = getelementptr inbounds nuw i8, ptr %self, i32 300
  %row21.i.i.i311.i = getelementptr inbounds nuw i8, ptr %self, i32 304
  %345 = getelementptr inbounds nuw i8, ptr %self, i32 308
  %346 = getelementptr inbounds nuw i8, ptr %self, i32 312
  %347 = getelementptr inbounds nuw i8, ptr %self, i32 316
  %history.i.i433.sroa.7.0.hot_right.i436.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i436, i32 4
  %history.i.i433.sroa.10.0.hot_right.i436.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i436, i32 8
  %history.i.i433.sroa.13.0.hot_right.i436.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i436, i32 12
  %history.i.i433.sroa.16.0.hot_right.i436.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i436, i32 16
  %history.i.i433.sroa.19.0.hot_right.i436.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i436, i32 20
  %history.i.i433.sroa.22.0.hot_right.i436.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i436, i32 24
  %history.i.i433.sroa.26.0.hot_right.i436.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i436, i32 28
  %history.i.i433.sroa.29.0.hot_right.i436.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i436, i32 32
  %history.i.i433.sroa.32.0.hot_right.i436.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i436, i32 36
  %history.i.i433.sroa.35.0.hot_right.i436.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i436, i32 40
  %history.i.i433.sroa.38.0.hot_right.i436.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i436, i32 44
  %_64.i = getelementptr inbounds nuw i8, ptr %hot_left.i437, i32 48
  %_65.i462 = getelementptr inbounds nuw i8, ptr %hot_left.i437, i32 64
  %_69.i463 = getelementptr inbounds nuw i8, ptr %hot_right.i436, i32 48
  %_70.i = getelementptr inbounds nuw i8, ptr %hot_right.i436, i32 64
  %_9.i3414 = add nsw i32 %_32.i441, -1
  %348 = getelementptr inbounds nuw i8, ptr %self, i32 528
  %349 = getelementptr inbounds nuw i8, ptr %self, i32 420
  %350 = getelementptr inbounds nuw i8, ptr %self, i32 344
  %351 = getelementptr inbounds nuw i8, ptr %self, i32 340
  %352 = getelementptr inbounds nuw i8, ptr %self, i32 384
  %353 = getelementptr inbounds nuw i8, ptr %self, i32 380
  %354 = getelementptr inbounds nuw i8, ptr %self, i32 368
  %355 = getelementptr inbounds nuw i8, ptr %self, i32 364
  %356 = getelementptr inbounds nuw i8, ptr %self, i32 352
  %357 = getelementptr inbounds nuw i8, ptr %self, i32 348
  %358 = getelementptr inbounds nuw i8, ptr %hot_left.i437, i32 84
  %359 = getelementptr inbounds nuw i8, ptr %hot_left.i437, i32 88
  %360 = getelementptr inbounds nuw i8, ptr %hot_left.i437, i32 80
  %361 = getelementptr inbounds nuw i8, ptr %self, i32 336
  %362 = getelementptr inbounds nuw i8, ptr %self, i32 332
  %_9.i3394 = add nsw i32 %_33.i443, -1
  %363 = getelementptr inbounds nuw i8, ptr %self, i32 520
  %364 = getelementptr inbounds nuw i8, ptr %self, i32 444
  %365 = getelementptr inbounds nuw i8, ptr %self, i32 440
  %366 = getelementptr inbounds nuw i8, ptr %self, i32 516
  %367 = getelementptr inbounds nuw i8, ptr %self, i32 512
  %368 = getelementptr inbounds nuw i8, ptr %self, i32 484
  %369 = getelementptr inbounds nuw i8, ptr %self, i32 480
  %370 = getelementptr inbounds nuw i8, ptr %self, i32 468
  %371 = getelementptr inbounds nuw i8, ptr %self, i32 464
  %372 = getelementptr inbounds nuw i8, ptr %self, i32 452
  %373 = getelementptr inbounds nuw i8, ptr %self, i32 448
  %374 = getelementptr inbounds nuw i8, ptr %hot_right.i436, i32 84
  %375 = getelementptr inbounds nuw i8, ptr %hot_right.i436, i32 88
  %376 = getelementptr inbounds nuw i8, ptr %hot_right.i436, i32 80
  %377 = getelementptr inbounds nuw i8, ptr %self, i32 436
  %378 = getelementptr inbounds nuw i8, ptr %self, i32 432
  %379 = getelementptr inbounds nuw i8, ptr %self, i32 532
  %iter.sroa.0.0.ptr.i55.i7651.1 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 4
  %iter.sroa.0.0.ptr.i55.i7651.2 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 8
  %iter.sroa.0.0.ptr.i55.i7651.3 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 12
  %iter.sroa.0.0.ptr.i55.i7651.4 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 16
  %iter.sroa.0.0.ptr.i55.i7651.5 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 20
  %iter.sroa.0.0.ptr.i55.i7651.6 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 24
  %iter.sroa.0.0.ptr.i55.i7651.7 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 28
  %iter.sroa.0.0.ptr.i.i7663.1 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 4
  %iter.sroa.0.0.ptr.i.i7663.2 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 8
  %iter.sroa.0.0.ptr.i.i7663.3 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 12
  %iter.sroa.0.0.ptr.i.i7663.4 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 16
  %iter.sroa.0.0.ptr.i.i7663.5 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 20
  %iter.sroa.0.0.ptr.i.i7663.6 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 24
  %iter.sroa.0.0.ptr.i.i7663.7 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 28
  br label %bb37.i, !dbg !20413

bb16.i.bb13.i.loopexit_crit_edge:                 ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3117
  store float %_0.i2846, ptr %358, align 4, !dbg !20432
  store float %_0.i3220, ptr %360, align 4, !dbg !20446
  store float %_0.i2842, ptr %374, align 4, !dbg !20447
  store float %_0.i3216, ptr %376, align 4, !dbg !20449
  br label %bb13.i.loopexit, !dbg !20450

bb13.i.loopexit:                                  ; preds = %bb16.i.bb13.i.loopexit_crit_edge, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i457
  %ring_cursor.sroa.0.1.i459.lcssa = phi i32 [ %spec.store.select12.i, %bb16.i.bb13.i.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i4517949, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i457 ], !dbg !20456
  %main_cursor.sroa.0.1.i460.lcssa = phi i32 [ %spec.store.select11.i, %bb16.i.bb13.i.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i4527950, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i457 ], !dbg !20457
  %_111.not.i = icmp eq i32 %382, 0, !dbg !20413
  %indvars.iv.next11914 = add i32 %indvars.iv11913, -32, !dbg !20413
  br i1 %_111.not.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit, label %bb37.i, !dbg !20413

bb37.i:                                           ; preds = %bb37.i.lr.ph, %bb13.i.loopexit
  %indvars.iv11913 = phi i32 [ %frames, %bb37.i.lr.ph ], [ %indvars.iv.next11914, %bb13.i.loopexit ]
  %main_cursor.sroa.0.0.i4527950 = phi i32 [ %_35.i, %bb37.i.lr.ph ], [ %main_cursor.sroa.0.1.i460.lcssa, %bb13.i.loopexit ]
  %ring_cursor.sroa.0.0.i4517949 = phi i32 [ %_37.i445, %bb37.i.lr.ph ], [ %ring_cursor.sroa.0.1.i459.lcssa, %bb13.i.loopexit ]
  %iter2.sroa.0.0.i4507948 = phi i32 [ %yield_count.sroa.0.0.i4016, %bb37.i.lr.ph ], [ %382, %bb13.i.loopexit ]
  %iter.sroa.0.0.i7947 = phi i32 [ 0, %bb37.i.lr.ph ], [ %381, %bb13.i.loopexit ]
  %380 = call i32 @llvm.umax.i32(i32 %indvars.iv11913, i32 1), !dbg !20458
  %umax11932 = call i32 @llvm.umin.i32(i32 %380, i32 32), !dbg !20458
  %381 = add i32 %iter.sroa.0.0.i7947, 32, !dbg !20458
  %382 = add nsw i32 %iter2.sroa.0.0.i4507948, -1, !dbg !20462
  %history.i135.i.sroa.0.0.copyload = load float, ptr %hot_left.i437, align 4, !dbg !20463, !noalias !20465
  %history.i135.i.sroa.7.0.copyload = load float, ptr %history.i135.i.sroa.7.0.hot_left.i437.sroa_idx, align 4, !dbg !20463, !noalias !20465
  %history.i135.i.sroa.10.0.copyload = load float, ptr %history.i135.i.sroa.10.0.hot_left.i437.sroa_idx, align 4, !dbg !20463, !noalias !20465
  %history.i135.i.sroa.13.0.copyload = load float, ptr %history.i135.i.sroa.13.0.hot_left.i437.sroa_idx, align 4, !dbg !20463, !noalias !20465
  %history.i135.i.sroa.16.0.copyload = load float, ptr %history.i135.i.sroa.16.0.hot_left.i437.sroa_idx, align 4, !dbg !20463, !noalias !20465
  %history.i135.i.sroa.19.0.copyload = load float, ptr %history.i135.i.sroa.19.0.hot_left.i437.sroa_idx, align 4, !dbg !20463, !noalias !20465
  %history.i135.i.sroa.22.0.copyload = load float, ptr %history.i135.i.sroa.22.0.hot_left.i437.sroa_idx, align 4, !dbg !20463, !noalias !20465
  %history.i135.i.sroa.26.0.copyload = load float, ptr %history.i135.i.sroa.26.0.hot_left.i437.sroa_idx, align 4, !dbg !20463, !noalias !20465
  %history.i135.i.sroa.29.0.copyload = load float, ptr %history.i135.i.sroa.29.0.hot_left.i437.sroa_idx, align 4, !dbg !20463, !noalias !20465
  %history.i135.i.sroa.32.0.copyload = load float, ptr %history.i135.i.sroa.32.0.hot_left.i437.sroa_idx, align 4, !dbg !20463, !noalias !20465
  %history.i135.i.sroa.35.0.copyload = load float, ptr %history.i135.i.sroa.35.0.hot_left.i437.sroa_idx, align 4, !dbg !20463, !noalias !20465
  %history.i135.i.sroa.38.0.copyload = load float, ptr %history.i135.i.sroa.38.0.hot_left.i437.sroa_idx, align 4, !dbg !20463, !noalias !20465
  %_20.i138.i7588.not = icmp eq i32 %frames, %iter.sroa.0.0.i7947, !dbg !20470
  br i1 %_20.i138.i7588.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i, label %bb5.i139.i.lr.ph, !dbg !20474

bb5.i139.i.lr.ph:                                 ; preds = %bb37.i
  %_11.i.i.i159.i = load float, ptr %_31, align 4
  %_14.i.i.i162.i = load float, ptr %312, align 4
  %_17.i.i.i165.i = load float, ptr %313, align 4
  %_20.i.i.i168.i = load float, ptr %314, align 4
  %_25.i.i.i173.i = load float, ptr %row1.i.i.i171.i, align 4
  %_28.i.i.i176.i = load float, ptr %315, align 4
  %_31.i.i.i179.i = load float, ptr %316, align 4
  %_34.i.i.i182.i = load float, ptr %317, align 4
  %_39.i.i.i187.i = load float, ptr %row3.i.i.i185.i, align 4
  %_42.i.i.i190.i = load float, ptr %318, align 4
  %_45.i.i.i193.i = load float, ptr %319, align 4
  %_48.i.i.i196.i = load float, ptr %320, align 4
  %_53.i.i.i201.i = load float, ptr %row5.i.i.i199.i, align 4
  %_56.i.i.i204.i = load float, ptr %321, align 4
  %_59.i.i.i207.i = load float, ptr %322, align 4
  %_62.i.i.i210.i = load float, ptr %323, align 4
  %_67.i.i.i215.i = load float, ptr %row7.i.i.i213.i, align 4
  %_70.i.i.i218.i = load float, ptr %324, align 4
  %_73.i.i.i221.i = load float, ptr %325, align 4
  %_76.i.i.i224.i = load float, ptr %326, align 4
  %_81.i.i.i229.i = load float, ptr %row9.i.i.i227.i, align 4
  %_84.i.i.i232.i = load float, ptr %327, align 4
  %_87.i.i.i235.i = load float, ptr %328, align 4
  %_90.i.i.i238.i = load float, ptr %329, align 4
  %_95.i.i.i243.i = load float, ptr %row11.i.i.i241.i, align 4
  %_98.i.i.i246.i = load float, ptr %330, align 4
  %_101.i.i.i249.i = load float, ptr %331, align 4
  %_104.i.i.i252.i = load float, ptr %332, align 4
  %_109.i.i.i257.i = load float, ptr %row13.i.i.i255.i, align 4
  %_112.i.i.i260.i = load float, ptr %333, align 4
  %_115.i.i.i263.i = load float, ptr %334, align 4
  %_118.i.i.i266.i = load float, ptr %335, align 4
  %_123.i.i.i271.i = load float, ptr %row15.i.i.i269.i, align 4
  %_126.i.i.i274.i = load float, ptr %336, align 4
  %_129.i.i.i277.i = load float, ptr %337, align 4
  %_132.i.i.i280.i = load float, ptr %338, align 4
  %_137.i.i.i285.i = load float, ptr %row17.i.i.i283.i, align 4
  %_140.i.i.i288.i = load float, ptr %339, align 4
  %_143.i.i.i291.i = load float, ptr %340, align 4
  %_146.i.i.i294.i = load float, ptr %341, align 4
  %_151.i.i.i299.i = load float, ptr %row19.i.i.i297.i, align 4
  %_154.i.i.i302.i = load float, ptr %342, align 4
  %_157.i.i.i305.i = load float, ptr %343, align 4
  %_160.i.i.i308.i = load float, ptr %344, align 4
  %_165.i.i.i313.i = load float, ptr %row21.i.i.i311.i, align 4
  %_168.i.i.i316.i = load float, ptr %345, align 4
  %_171.i.i.i319.i = load float, ptr %346, align 4
  %_174.i.i.i322.i = load float, ptr %347, align 4
  br label %bb5.i139.i, !dbg !20474

bb5.i139.i:                                       ; preds = %bb5.i139.i.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960
  %iter.sroa.0.0.i137.i7600 = phi i32 [ 0, %bb5.i139.i.lr.ph ], [ %383, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ]
  %history.i135.i.sroa.35.07599 = phi float [ %history.i135.i.sroa.35.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.32.07598, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ]
  %history.i135.i.sroa.32.07598 = phi float [ %history.i135.i.sroa.32.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.29.07597, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ]
  %history.i135.i.sroa.29.07597 = phi float [ %history.i135.i.sroa.29.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.26.07596, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ]
  %history.i135.i.sroa.26.07596 = phi float [ %history.i135.i.sroa.26.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.22.07595, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ]
  %history.i135.i.sroa.22.07595 = phi float [ %history.i135.i.sroa.22.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.19.07594, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ]
  %history.i135.i.sroa.19.07594 = phi float [ %history.i135.i.sroa.19.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.16.07593, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ]
  %history.i135.i.sroa.16.07593 = phi float [ %history.i135.i.sroa.16.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.13.07592, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ]
  %history.i135.i.sroa.13.07592 = phi float [ %history.i135.i.sroa.13.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.10.07591, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ]
  %history.i135.i.sroa.10.07591 = phi float [ %history.i135.i.sroa.10.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.7.07590, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ]
  %history.i135.i.sroa.7.07590 = phi float [ %history.i135.i.sroa.7.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.0.07589, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ]
  %history.i135.i.sroa.0.07589 = phi float [ %history.i135.i.sroa.0.0.copyload, %bb5.i139.i.lr.ph ], [ %_0.i2958, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ]
  %383 = add nuw nsw i32 %iter.sroa.0.0.i137.i7600, 1, !dbg !20475
  %_11.i140.i = add nuw nsw i32 %iter.sroa.0.0.i137.i7600, %iter.sroa.0.0.i7947, !dbg !20478
  %_24.i141.i = icmp ugt i32 %_11.i140.i, %left_io.1, !dbg !20479
  br i1 %_24.i141.i, label %bb7.i338.i, label %bb8.i142.i, !dbg !20479, !prof !902

bb8.i142.i:                                       ; preds = %bb5.i139.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20482), !dbg !20485
  %_3.not.i2956 = icmp eq i32 %left_io.1, %_11.i140.i, !dbg !20486
  br i1 %_3.not.i2956, label %panic.i2959, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960, !dbg !20486

panic.i2959:                                      ; preds = %bb8.i142.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #33, !dbg !20486, !noalias !20488
  unreachable, !dbg !20486

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960: ; preds = %bb8.i142.i
  %_31.i144.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %_11.i140.i, !dbg !20490
  %_0.i2958 = load float, ptr %_31.i144.i, align 4, !dbg !20486, !alias.scope !20482, !noalias !20492, !noundef !10
  %384 = tail call noundef float @llvm.fabs.f32(float %history.i135.i.sroa.19.07594), !dbg !20493
  %_0.i2564 = fmul float %_0.i2958, %_11.i.i.i159.i, !dbg !20496
  %_0.i2136 = fadd float %_0.i2564, 0.000000e+00, !dbg !20499
  %_0.i2563 = fmul float %_0.i2958, %_14.i.i.i162.i, !dbg !20501
  %_0.i2135 = fadd float %_0.i2563, 0.000000e+00, !dbg !20503
  %_0.i2562 = fmul float %_0.i2958, %_17.i.i.i165.i, !dbg !20505
  %_0.i2134 = fadd float %_0.i2562, 0.000000e+00, !dbg !20507
  %_0.i2561 = fmul float %_0.i2958, %_20.i.i.i168.i, !dbg !20509
  %_0.i2133 = fadd float %_0.i2561, 0.000000e+00, !dbg !20511
  %_0.i2560 = fmul float %history.i135.i.sroa.0.07589, %_25.i.i.i173.i, !dbg !20513
  %_0.i2132 = fadd float %_0.i2136, %_0.i2560, !dbg !20515
  %_0.i2559 = fmul float %history.i135.i.sroa.0.07589, %_28.i.i.i176.i, !dbg !20517
  %_0.i2131 = fadd float %_0.i2135, %_0.i2559, !dbg !20519
  %_0.i2558 = fmul float %history.i135.i.sroa.0.07589, %_31.i.i.i179.i, !dbg !20521
  %_0.i2130 = fadd float %_0.i2134, %_0.i2558, !dbg !20523
  %_0.i2557 = fmul float %history.i135.i.sroa.0.07589, %_34.i.i.i182.i, !dbg !20525
  %_0.i2129 = fadd float %_0.i2133, %_0.i2557, !dbg !20527
  %_0.i2556 = fmul float %history.i135.i.sroa.7.07590, %_39.i.i.i187.i, !dbg !20529
  %_0.i2128 = fadd float %_0.i2132, %_0.i2556, !dbg !20531
  %_0.i2555 = fmul float %history.i135.i.sroa.7.07590, %_42.i.i.i190.i, !dbg !20533
  %_0.i2127 = fadd float %_0.i2131, %_0.i2555, !dbg !20535
  %_0.i2554 = fmul float %history.i135.i.sroa.7.07590, %_45.i.i.i193.i, !dbg !20537
  %_0.i2126 = fadd float %_0.i2130, %_0.i2554, !dbg !20539
  %_0.i2553 = fmul float %history.i135.i.sroa.7.07590, %_48.i.i.i196.i, !dbg !20541
  %_0.i2125 = fadd float %_0.i2129, %_0.i2553, !dbg !20543
  %_0.i2552 = fmul float %history.i135.i.sroa.10.07591, %_53.i.i.i201.i, !dbg !20545
  %_0.i2124 = fadd float %_0.i2128, %_0.i2552, !dbg !20547
  %_0.i2551 = fmul float %history.i135.i.sroa.10.07591, %_56.i.i.i204.i, !dbg !20549
  %_0.i2123 = fadd float %_0.i2127, %_0.i2551, !dbg !20551
  %_0.i2550 = fmul float %history.i135.i.sroa.10.07591, %_59.i.i.i207.i, !dbg !20553
  %_0.i2122 = fadd float %_0.i2126, %_0.i2550, !dbg !20555
  %_0.i2549 = fmul float %history.i135.i.sroa.10.07591, %_62.i.i.i210.i, !dbg !20557
  %_0.i2121 = fadd float %_0.i2125, %_0.i2549, !dbg !20559
  %_0.i2548 = fmul float %history.i135.i.sroa.13.07592, %_67.i.i.i215.i, !dbg !20561
  %_0.i2120 = fadd float %_0.i2124, %_0.i2548, !dbg !20563
  %_0.i2547 = fmul float %history.i135.i.sroa.13.07592, %_70.i.i.i218.i, !dbg !20565
  %_0.i2119 = fadd float %_0.i2123, %_0.i2547, !dbg !20567
  %_0.i2546 = fmul float %history.i135.i.sroa.13.07592, %_73.i.i.i221.i, !dbg !20569
  %_0.i2118 = fadd float %_0.i2122, %_0.i2546, !dbg !20571
  %_0.i2545 = fmul float %history.i135.i.sroa.13.07592, %_76.i.i.i224.i, !dbg !20573
  %_0.i2117 = fadd float %_0.i2121, %_0.i2545, !dbg !20575
  %_0.i2544 = fmul float %history.i135.i.sroa.16.07593, %_81.i.i.i229.i, !dbg !20577
  %_0.i2116 = fadd float %_0.i2120, %_0.i2544, !dbg !20579
  %_0.i2543 = fmul float %history.i135.i.sroa.16.07593, %_84.i.i.i232.i, !dbg !20581
  %_0.i2115 = fadd float %_0.i2119, %_0.i2543, !dbg !20583
  %_0.i2542 = fmul float %history.i135.i.sroa.16.07593, %_87.i.i.i235.i, !dbg !20585
  %_0.i2114 = fadd float %_0.i2118, %_0.i2542, !dbg !20587
  %_0.i2541 = fmul float %history.i135.i.sroa.16.07593, %_90.i.i.i238.i, !dbg !20589
  %_0.i2113 = fadd float %_0.i2117, %_0.i2541, !dbg !20591
  %_0.i2540 = fmul float %history.i135.i.sroa.19.07594, %_95.i.i.i243.i, !dbg !20593
  %_0.i2112 = fadd float %_0.i2116, %_0.i2540, !dbg !20595
  %_0.i2539 = fmul float %history.i135.i.sroa.19.07594, %_98.i.i.i246.i, !dbg !20597
  %_0.i2111 = fadd float %_0.i2115, %_0.i2539, !dbg !20599
  %_0.i2538 = fmul float %history.i135.i.sroa.19.07594, %_101.i.i.i249.i, !dbg !20601
  %_0.i2110 = fadd float %_0.i2114, %_0.i2538, !dbg !20603
  %_0.i2537 = fmul float %history.i135.i.sroa.19.07594, %_104.i.i.i252.i, !dbg !20605
  %_0.i2109 = fadd float %_0.i2113, %_0.i2537, !dbg !20607
  %_0.i2536 = fmul float %history.i135.i.sroa.22.07595, %_109.i.i.i257.i, !dbg !20609
  %_0.i2108 = fadd float %_0.i2112, %_0.i2536, !dbg !20611
  %_0.i2535 = fmul float %history.i135.i.sroa.22.07595, %_112.i.i.i260.i, !dbg !20613
  %_0.i2107 = fadd float %_0.i2111, %_0.i2535, !dbg !20615
  %_0.i2534 = fmul float %history.i135.i.sroa.22.07595, %_115.i.i.i263.i, !dbg !20617
  %_0.i2106 = fadd float %_0.i2110, %_0.i2534, !dbg !20619
  %_0.i2533 = fmul float %history.i135.i.sroa.22.07595, %_118.i.i.i266.i, !dbg !20621
  %_0.i2105 = fadd float %_0.i2109, %_0.i2533, !dbg !20623
  %_0.i2532 = fmul float %history.i135.i.sroa.26.07596, %_123.i.i.i271.i, !dbg !20625
  %_0.i2104 = fadd float %_0.i2108, %_0.i2532, !dbg !20627
  %_0.i2531 = fmul float %history.i135.i.sroa.26.07596, %_126.i.i.i274.i, !dbg !20629
  %_0.i2103 = fadd float %_0.i2107, %_0.i2531, !dbg !20631
  %_0.i2530 = fmul float %history.i135.i.sroa.26.07596, %_129.i.i.i277.i, !dbg !20633
  %_0.i2102 = fadd float %_0.i2106, %_0.i2530, !dbg !20635
  %_0.i2529 = fmul float %history.i135.i.sroa.26.07596, %_132.i.i.i280.i, !dbg !20637
  %_0.i2101 = fadd float %_0.i2105, %_0.i2529, !dbg !20639
  %_0.i2528 = fmul float %history.i135.i.sroa.29.07597, %_137.i.i.i285.i, !dbg !20641
  %_0.i2100 = fadd float %_0.i2104, %_0.i2528, !dbg !20643
  %_0.i2527 = fmul float %history.i135.i.sroa.29.07597, %_140.i.i.i288.i, !dbg !20645
  %_0.i2099 = fadd float %_0.i2103, %_0.i2527, !dbg !20647
  %_0.i2526 = fmul float %history.i135.i.sroa.29.07597, %_143.i.i.i291.i, !dbg !20649
  %_0.i2098 = fadd float %_0.i2102, %_0.i2526, !dbg !20651
  %_0.i2525 = fmul float %history.i135.i.sroa.29.07597, %_146.i.i.i294.i, !dbg !20653
  %_0.i2097 = fadd float %_0.i2101, %_0.i2525, !dbg !20655
  %_0.i2524 = fmul float %history.i135.i.sroa.32.07598, %_151.i.i.i299.i, !dbg !20657
  %_0.i2096 = fadd float %_0.i2100, %_0.i2524, !dbg !20659
  %_0.i2523 = fmul float %history.i135.i.sroa.32.07598, %_154.i.i.i302.i, !dbg !20661
  %_0.i2095 = fadd float %_0.i2099, %_0.i2523, !dbg !20663
  %_0.i2522 = fmul float %history.i135.i.sroa.32.07598, %_157.i.i.i305.i, !dbg !20665
  %_0.i2094 = fadd float %_0.i2098, %_0.i2522, !dbg !20667
  %_0.i2521 = fmul float %history.i135.i.sroa.32.07598, %_160.i.i.i308.i, !dbg !20669
  %_0.i2093 = fadd float %_0.i2097, %_0.i2521, !dbg !20671
  %_0.i2520 = fmul float %history.i135.i.sroa.35.07599, %_165.i.i.i313.i, !dbg !20673
  %_0.i2092 = fadd float %_0.i2096, %_0.i2520, !dbg !20675
  %_0.i2519 = fmul float %history.i135.i.sroa.35.07599, %_168.i.i.i316.i, !dbg !20677
  %_0.i2091 = fadd float %_0.i2095, %_0.i2519, !dbg !20679
  %_0.i2518 = fmul float %history.i135.i.sroa.35.07599, %_171.i.i.i319.i, !dbg !20681
  %_0.i2090 = fadd float %_0.i2094, %_0.i2518, !dbg !20683
  %_0.i2517 = fmul float %history.i135.i.sroa.35.07599, %_174.i.i.i322.i, !dbg !20685
  %_0.i2089 = fadd float %_0.i2093, %_0.i2517, !dbg !20687
  %385 = tail call noundef float @llvm.fabs.f32(float %_0.i2092), !dbg !20689
  %_3.i.i3593.inv = fcmp ogt float %384, %385, !dbg !20691
  %_4.i.i3600.v = select i1 %_3.i.i3593.inv, float %384, float %385, !dbg !20691
  %386 = tail call noundef float @llvm.fabs.f32(float %_0.i2091), !dbg !20689
  %_3.i.i3593.inv.1 = fcmp ogt float %_4.i.i3600.v, %386, !dbg !20691
  %_4.i.i3600.v.1 = select i1 %_3.i.i3593.inv.1, float %_4.i.i3600.v, float %386, !dbg !20691
  %387 = tail call noundef float @llvm.fabs.f32(float %_0.i2090), !dbg !20689
  %_3.i.i3593.inv.2 = fcmp ogt float %_4.i.i3600.v.1, %387, !dbg !20691
  %_4.i.i3600.v.2 = select i1 %_3.i.i3593.inv.2, float %_4.i.i3600.v.1, float %387, !dbg !20691
  %388 = tail call noundef float @llvm.fabs.f32(float %_0.i2089), !dbg !20689
  %_3.i.i3593.inv.3 = fcmp ogt float %_4.i.i3600.v.2, %388, !dbg !20691
  %_4.i.i3600.v.3 = select i1 %_3.i.i3593.inv.3, float %_4.i.i3600.v.2, float %388, !dbg !20691
  %_39.i333.i = getelementptr inbounds nuw float, ptr %peaks_left.i435, i32 %iter.sroa.0.0.i137.i7600, !dbg !20694
  store float %_4.i.i3600.v.3, ptr %_39.i333.i, align 4, !dbg !20699, !alias.scope !20701, !noalias !20492
  %exitcond11917.not = icmp eq i32 %383, %umax11932, !dbg !20470
  br i1 %exitcond11917.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i, label %bb5.i139.i, !dbg !20474

bb7.i338.i:                                       ; preds = %bb5.i139.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_11.i140.i, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #33, !dbg !20704, !noalias !20492
  unreachable, !dbg !20704

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960, %bb37.i
  %history.i135.i.sroa.0.0.lcssa = phi float [ %history.i135.i.sroa.0.0.copyload, %bb37.i ], [ %_0.i2958, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ], !dbg !20705
  %history.i135.i.sroa.7.0.lcssa = phi float [ %history.i135.i.sroa.7.0.copyload, %bb37.i ], [ %history.i135.i.sroa.0.07589, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ], !dbg !20705
  %history.i135.i.sroa.10.0.lcssa = phi float [ %history.i135.i.sroa.10.0.copyload, %bb37.i ], [ %history.i135.i.sroa.7.07590, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ], !dbg !20705
  %history.i135.i.sroa.13.0.lcssa = phi float [ %history.i135.i.sroa.13.0.copyload, %bb37.i ], [ %history.i135.i.sroa.10.07591, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ], !dbg !20705
  %history.i135.i.sroa.16.0.lcssa = phi float [ %history.i135.i.sroa.16.0.copyload, %bb37.i ], [ %history.i135.i.sroa.13.07592, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ], !dbg !20705
  %history.i135.i.sroa.19.0.lcssa = phi float [ %history.i135.i.sroa.19.0.copyload, %bb37.i ], [ %history.i135.i.sroa.16.07593, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ], !dbg !20705
  %history.i135.i.sroa.22.0.lcssa = phi float [ %history.i135.i.sroa.22.0.copyload, %bb37.i ], [ %history.i135.i.sroa.19.07594, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ], !dbg !20705
  %history.i135.i.sroa.26.0.lcssa = phi float [ %history.i135.i.sroa.26.0.copyload, %bb37.i ], [ %history.i135.i.sroa.22.07595, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ], !dbg !20705
  %history.i135.i.sroa.29.0.lcssa = phi float [ %history.i135.i.sroa.29.0.copyload, %bb37.i ], [ %history.i135.i.sroa.26.07596, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ], !dbg !20705
  %history.i135.i.sroa.32.0.lcssa = phi float [ %history.i135.i.sroa.32.0.copyload, %bb37.i ], [ %history.i135.i.sroa.29.07597, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ], !dbg !20705
  %history.i135.i.sroa.35.0.lcssa = phi float [ %history.i135.i.sroa.35.0.copyload, %bb37.i ], [ %history.i135.i.sroa.32.07598, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ], !dbg !20705
  %history.i135.i.sroa.38.0.lcssa = phi float [ %history.i135.i.sroa.38.0.copyload, %bb37.i ], [ %history.i135.i.sroa.35.07599, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ], !dbg !20705
  store float %history.i135.i.sroa.0.0.lcssa, ptr %hot_left.i437, align 4, !dbg !20706, !noalias !20465
  store float %history.i135.i.sroa.7.0.lcssa, ptr %history.i135.i.sroa.7.0.hot_left.i437.sroa_idx, align 4, !dbg !20706, !noalias !20465
  store float %history.i135.i.sroa.10.0.lcssa, ptr %history.i135.i.sroa.10.0.hot_left.i437.sroa_idx, align 4, !dbg !20706, !noalias !20465
  store float %history.i135.i.sroa.13.0.lcssa, ptr %history.i135.i.sroa.13.0.hot_left.i437.sroa_idx, align 4, !dbg !20706, !noalias !20465
  store float %history.i135.i.sroa.16.0.lcssa, ptr %history.i135.i.sroa.16.0.hot_left.i437.sroa_idx, align 4, !dbg !20706, !noalias !20465
  store float %history.i135.i.sroa.19.0.lcssa, ptr %history.i135.i.sroa.19.0.hot_left.i437.sroa_idx, align 4, !dbg !20706, !noalias !20465
  store float %history.i135.i.sroa.22.0.lcssa, ptr %history.i135.i.sroa.22.0.hot_left.i437.sroa_idx, align 4, !dbg !20706, !noalias !20465
  store float %history.i135.i.sroa.26.0.lcssa, ptr %history.i135.i.sroa.26.0.hot_left.i437.sroa_idx, align 4, !dbg !20706, !noalias !20465
  store float %history.i135.i.sroa.29.0.lcssa, ptr %history.i135.i.sroa.29.0.hot_left.i437.sroa_idx, align 4, !dbg !20706, !noalias !20465
  store float %history.i135.i.sroa.32.0.lcssa, ptr %history.i135.i.sroa.32.0.hot_left.i437.sroa_idx, align 4, !dbg !20706, !noalias !20465
  store float %history.i135.i.sroa.35.0.lcssa, ptr %history.i135.i.sroa.35.0.hot_left.i437.sroa_idx, align 4, !dbg !20706, !noalias !20465
  store float %history.i135.i.sroa.38.0.lcssa, ptr %history.i135.i.sroa.38.0.hot_left.i437.sroa_idx, align 4, !dbg !20706, !noalias !20465
  %history.i.i433.sroa.0.0.copyload = load float, ptr %hot_right.i436, align 4, !dbg !20707, !noalias !20709
  %history.i.i433.sroa.7.0.copyload = load float, ptr %history.i.i433.sroa.7.0.hot_right.i436.sroa_idx, align 4, !dbg !20707, !noalias !20709
  %history.i.i433.sroa.10.0.copyload = load float, ptr %history.i.i433.sroa.10.0.hot_right.i436.sroa_idx, align 4, !dbg !20707, !noalias !20709
  %history.i.i433.sroa.13.0.copyload = load float, ptr %history.i.i433.sroa.13.0.hot_right.i436.sroa_idx, align 4, !dbg !20707, !noalias !20709
  %history.i.i433.sroa.16.0.copyload = load float, ptr %history.i.i433.sroa.16.0.hot_right.i436.sroa_idx, align 4, !dbg !20707, !noalias !20709
  %history.i.i433.sroa.19.0.copyload = load float, ptr %history.i.i433.sroa.19.0.hot_right.i436.sroa_idx, align 4, !dbg !20707, !noalias !20709
  %history.i.i433.sroa.22.0.copyload = load float, ptr %history.i.i433.sroa.22.0.hot_right.i436.sroa_idx, align 4, !dbg !20707, !noalias !20709
  %history.i.i433.sroa.26.0.copyload = load float, ptr %history.i.i433.sroa.26.0.hot_right.i436.sroa_idx, align 4, !dbg !20707, !noalias !20709
  %history.i.i433.sroa.29.0.copyload = load float, ptr %history.i.i433.sroa.29.0.hot_right.i436.sroa_idx, align 4, !dbg !20707, !noalias !20709
  %history.i.i433.sroa.32.0.copyload = load float, ptr %history.i.i433.sroa.32.0.hot_right.i436.sroa_idx, align 4, !dbg !20707, !noalias !20709
  %history.i.i433.sroa.35.0.copyload = load float, ptr %history.i.i433.sroa.35.0.hot_right.i436.sroa_idx, align 4, !dbg !20707, !noalias !20709
  %history.i.i433.sroa.38.0.copyload = load float, ptr %history.i.i433.sroa.38.0.hot_right.i436.sroa_idx, align 4, !dbg !20707, !noalias !20709
  br i1 %_20.i138.i7588.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i457, label %bb5.i.i492.lr.ph, !dbg !20714

bb5.i.i492.lr.ph:                                 ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i
  %_11.i.i.i.i511 = load float, ptr %_31, align 4
  %_14.i.i.i.i514 = load float, ptr %312, align 4
  %_17.i.i.i.i517 = load float, ptr %313, align 4
  %_20.i.i.i.i520 = load float, ptr %314, align 4
  %_25.i.i.i.i525 = load float, ptr %row1.i.i.i171.i, align 4
  %_28.i.i.i.i528 = load float, ptr %315, align 4
  %_31.i.i.i.i531 = load float, ptr %316, align 4
  %_34.i.i.i.i534 = load float, ptr %317, align 4
  %_39.i.i.i.i539 = load float, ptr %row3.i.i.i185.i, align 4
  %_42.i.i.i.i542 = load float, ptr %318, align 4
  %_45.i.i.i.i545 = load float, ptr %319, align 4
  %_48.i.i.i.i548 = load float, ptr %320, align 4
  %_53.i.i.i.i553 = load float, ptr %row5.i.i.i199.i, align 4
  %_56.i.i.i.i556 = load float, ptr %321, align 4
  %_59.i.i.i.i559 = load float, ptr %322, align 4
  %_62.i.i.i.i562 = load float, ptr %323, align 4
  %_67.i.i.i.i567 = load float, ptr %row7.i.i.i213.i, align 4
  %_70.i.i.i.i570 = load float, ptr %324, align 4
  %_73.i.i.i.i573 = load float, ptr %325, align 4
  %_76.i.i.i.i576 = load float, ptr %326, align 4
  %_81.i.i.i.i581 = load float, ptr %row9.i.i.i227.i, align 4
  %_84.i.i.i.i584 = load float, ptr %327, align 4
  %_87.i.i.i.i587 = load float, ptr %328, align 4
  %_90.i.i.i.i590 = load float, ptr %329, align 4
  %_95.i.i.i.i595 = load float, ptr %row11.i.i.i241.i, align 4
  %_98.i.i.i.i598 = load float, ptr %330, align 4
  %_101.i.i.i.i601 = load float, ptr %331, align 4
  %_104.i.i.i.i604 = load float, ptr %332, align 4
  %_109.i.i.i.i609 = load float, ptr %row13.i.i.i255.i, align 4
  %_112.i.i.i.i612 = load float, ptr %333, align 4
  %_115.i.i.i.i615 = load float, ptr %334, align 4
  %_118.i.i.i.i618 = load float, ptr %335, align 4
  %_123.i.i.i.i623 = load float, ptr %row15.i.i.i269.i, align 4
  %_126.i.i.i.i626 = load float, ptr %336, align 4
  %_129.i.i.i.i629 = load float, ptr %337, align 4
  %_132.i.i.i.i632 = load float, ptr %338, align 4
  %_137.i.i.i.i637 = load float, ptr %row17.i.i.i283.i, align 4
  %_140.i.i.i.i640 = load float, ptr %339, align 4
  %_143.i.i.i.i643 = load float, ptr %340, align 4
  %_146.i.i.i.i646 = load float, ptr %341, align 4
  %_151.i.i.i.i651 = load float, ptr %row19.i.i.i297.i, align 4
  %_154.i.i.i.i654 = load float, ptr %342, align 4
  %_157.i.i.i.i657 = load float, ptr %343, align 4
  %_160.i.i.i.i660 = load float, ptr %344, align 4
  %_165.i.i.i.i665 = load float, ptr %row21.i.i.i311.i, align 4
  %_168.i.i.i.i668 = load float, ptr %345, align 4
  %_171.i.i.i.i671 = load float, ptr %346, align 4
  %_174.i.i.i.i674 = load float, ptr %347, align 4
  br label %bb5.i.i492, !dbg !20714

bb5.i.i492:                                       ; preds = %bb5.i.i492.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965
  %iter.sroa.0.0.i.i4557627 = phi i32 [ 0, %bb5.i.i492.lr.ph ], [ %389, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ]
  %history.i.i433.sroa.35.07626 = phi float [ %history.i.i433.sroa.35.0.copyload, %bb5.i.i492.lr.ph ], [ %history.i.i433.sroa.32.07625, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ]
  %history.i.i433.sroa.32.07625 = phi float [ %history.i.i433.sroa.32.0.copyload, %bb5.i.i492.lr.ph ], [ %history.i.i433.sroa.29.07624, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ]
  %history.i.i433.sroa.29.07624 = phi float [ %history.i.i433.sroa.29.0.copyload, %bb5.i.i492.lr.ph ], [ %history.i.i433.sroa.26.07623, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ]
  %history.i.i433.sroa.26.07623 = phi float [ %history.i.i433.sroa.26.0.copyload, %bb5.i.i492.lr.ph ], [ %history.i.i433.sroa.22.07622, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ]
  %history.i.i433.sroa.22.07622 = phi float [ %history.i.i433.sroa.22.0.copyload, %bb5.i.i492.lr.ph ], [ %history.i.i433.sroa.19.07621, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ]
  %history.i.i433.sroa.19.07621 = phi float [ %history.i.i433.sroa.19.0.copyload, %bb5.i.i492.lr.ph ], [ %history.i.i433.sroa.16.07620, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ]
  %history.i.i433.sroa.16.07620 = phi float [ %history.i.i433.sroa.16.0.copyload, %bb5.i.i492.lr.ph ], [ %history.i.i433.sroa.13.07619, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ]
  %history.i.i433.sroa.13.07619 = phi float [ %history.i.i433.sroa.13.0.copyload, %bb5.i.i492.lr.ph ], [ %history.i.i433.sroa.10.07618, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ]
  %history.i.i433.sroa.10.07618 = phi float [ %history.i.i433.sroa.10.0.copyload, %bb5.i.i492.lr.ph ], [ %history.i.i433.sroa.7.07617, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ]
  %history.i.i433.sroa.7.07617 = phi float [ %history.i.i433.sroa.7.0.copyload, %bb5.i.i492.lr.ph ], [ %history.i.i433.sroa.0.07616, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ]
  %history.i.i433.sroa.0.07616 = phi float [ %history.i.i433.sroa.0.0.copyload, %bb5.i.i492.lr.ph ], [ %_0.i2963, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ]
  %389 = add nuw nsw i32 %iter.sroa.0.0.i.i4557627, 1, !dbg !20717
  %_11.i.i493 = add nuw nsw i32 %iter.sroa.0.0.i.i4557627, %iter.sroa.0.0.i7947, !dbg !20720
  %_24.i.i494 = icmp ugt i32 %_11.i.i493, %right_io.1, !dbg !20721
  br i1 %_24.i.i494, label %bb7.i.i690, label %bb8.i.i495, !dbg !20721, !prof !902

bb8.i.i495:                                       ; preds = %bb5.i.i492
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20724), !dbg !20727
  %_3.not.i2961 = icmp eq i32 %right_io.1, %_11.i.i493, !dbg !20728
  br i1 %_3.not.i2961, label %panic.i2964, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965, !dbg !20728

panic.i2964:                                      ; preds = %bb8.i.i495
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #33, !dbg !20728, !noalias !20730
  unreachable, !dbg !20728

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965: ; preds = %bb8.i.i495
  %_31.i126.i = getelementptr inbounds nuw float, ptr %right_io.0, i32 %_11.i.i493, !dbg !20732
  %_0.i2963 = load float, ptr %_31.i126.i, align 4, !dbg !20728, !alias.scope !20724, !noalias !20734, !noundef !10
  %390 = tail call noundef float @llvm.fabs.f32(float %history.i.i433.sroa.19.07621), !dbg !20735
  %_0.i2612 = fmul float %_0.i2963, %_11.i.i.i.i511, !dbg !20738
  %_0.i2184 = fadd float %_0.i2612, 0.000000e+00, !dbg !20741
  %_0.i2611 = fmul float %_0.i2963, %_14.i.i.i.i514, !dbg !20743
  %_0.i2183 = fadd float %_0.i2611, 0.000000e+00, !dbg !20745
  %_0.i2610 = fmul float %_0.i2963, %_17.i.i.i.i517, !dbg !20747
  %_0.i2182 = fadd float %_0.i2610, 0.000000e+00, !dbg !20749
  %_0.i2609 = fmul float %_0.i2963, %_20.i.i.i.i520, !dbg !20751
  %_0.i2181 = fadd float %_0.i2609, 0.000000e+00, !dbg !20753
  %_0.i2608 = fmul float %history.i.i433.sroa.0.07616, %_25.i.i.i.i525, !dbg !20755
  %_0.i2180 = fadd float %_0.i2184, %_0.i2608, !dbg !20757
  %_0.i2607 = fmul float %history.i.i433.sroa.0.07616, %_28.i.i.i.i528, !dbg !20759
  %_0.i2179 = fadd float %_0.i2183, %_0.i2607, !dbg !20761
  %_0.i2606 = fmul float %history.i.i433.sroa.0.07616, %_31.i.i.i.i531, !dbg !20763
  %_0.i2178 = fadd float %_0.i2182, %_0.i2606, !dbg !20765
  %_0.i2605 = fmul float %history.i.i433.sroa.0.07616, %_34.i.i.i.i534, !dbg !20767
  %_0.i2177 = fadd float %_0.i2181, %_0.i2605, !dbg !20769
  %_0.i2604 = fmul float %history.i.i433.sroa.7.07617, %_39.i.i.i.i539, !dbg !20771
  %_0.i2176 = fadd float %_0.i2180, %_0.i2604, !dbg !20773
  %_0.i2603 = fmul float %history.i.i433.sroa.7.07617, %_42.i.i.i.i542, !dbg !20775
  %_0.i2175 = fadd float %_0.i2179, %_0.i2603, !dbg !20777
  %_0.i2602 = fmul float %history.i.i433.sroa.7.07617, %_45.i.i.i.i545, !dbg !20779
  %_0.i2174 = fadd float %_0.i2178, %_0.i2602, !dbg !20781
  %_0.i2601 = fmul float %history.i.i433.sroa.7.07617, %_48.i.i.i.i548, !dbg !20783
  %_0.i2173 = fadd float %_0.i2177, %_0.i2601, !dbg !20785
  %_0.i2600 = fmul float %history.i.i433.sroa.10.07618, %_53.i.i.i.i553, !dbg !20787
  %_0.i2172 = fadd float %_0.i2176, %_0.i2600, !dbg !20789
  %_0.i2599 = fmul float %history.i.i433.sroa.10.07618, %_56.i.i.i.i556, !dbg !20791
  %_0.i2171 = fadd float %_0.i2175, %_0.i2599, !dbg !20793
  %_0.i2598 = fmul float %history.i.i433.sroa.10.07618, %_59.i.i.i.i559, !dbg !20795
  %_0.i2170 = fadd float %_0.i2174, %_0.i2598, !dbg !20797
  %_0.i2597 = fmul float %history.i.i433.sroa.10.07618, %_62.i.i.i.i562, !dbg !20799
  %_0.i2169 = fadd float %_0.i2173, %_0.i2597, !dbg !20801
  %_0.i2596 = fmul float %history.i.i433.sroa.13.07619, %_67.i.i.i.i567, !dbg !20803
  %_0.i2168 = fadd float %_0.i2172, %_0.i2596, !dbg !20805
  %_0.i2595 = fmul float %history.i.i433.sroa.13.07619, %_70.i.i.i.i570, !dbg !20807
  %_0.i2167 = fadd float %_0.i2171, %_0.i2595, !dbg !20809
  %_0.i2594 = fmul float %history.i.i433.sroa.13.07619, %_73.i.i.i.i573, !dbg !20811
  %_0.i2166 = fadd float %_0.i2170, %_0.i2594, !dbg !20813
  %_0.i2593 = fmul float %history.i.i433.sroa.13.07619, %_76.i.i.i.i576, !dbg !20815
  %_0.i2165 = fadd float %_0.i2169, %_0.i2593, !dbg !20817
  %_0.i2592 = fmul float %history.i.i433.sroa.16.07620, %_81.i.i.i.i581, !dbg !20819
  %_0.i2164 = fadd float %_0.i2168, %_0.i2592, !dbg !20821
  %_0.i2591 = fmul float %history.i.i433.sroa.16.07620, %_84.i.i.i.i584, !dbg !20823
  %_0.i2163 = fadd float %_0.i2167, %_0.i2591, !dbg !20825
  %_0.i2590 = fmul float %history.i.i433.sroa.16.07620, %_87.i.i.i.i587, !dbg !20827
  %_0.i2162 = fadd float %_0.i2166, %_0.i2590, !dbg !20829
  %_0.i2589 = fmul float %history.i.i433.sroa.16.07620, %_90.i.i.i.i590, !dbg !20831
  %_0.i2161 = fadd float %_0.i2165, %_0.i2589, !dbg !20833
  %_0.i2588 = fmul float %history.i.i433.sroa.19.07621, %_95.i.i.i.i595, !dbg !20835
  %_0.i2160 = fadd float %_0.i2164, %_0.i2588, !dbg !20837
  %_0.i2587 = fmul float %history.i.i433.sroa.19.07621, %_98.i.i.i.i598, !dbg !20839
  %_0.i2159 = fadd float %_0.i2163, %_0.i2587, !dbg !20841
  %_0.i2586 = fmul float %history.i.i433.sroa.19.07621, %_101.i.i.i.i601, !dbg !20843
  %_0.i2158 = fadd float %_0.i2162, %_0.i2586, !dbg !20845
  %_0.i2585 = fmul float %history.i.i433.sroa.19.07621, %_104.i.i.i.i604, !dbg !20847
  %_0.i2157 = fadd float %_0.i2161, %_0.i2585, !dbg !20849
  %_0.i2584 = fmul float %history.i.i433.sroa.22.07622, %_109.i.i.i.i609, !dbg !20851
  %_0.i2156 = fadd float %_0.i2160, %_0.i2584, !dbg !20853
  %_0.i2583 = fmul float %history.i.i433.sroa.22.07622, %_112.i.i.i.i612, !dbg !20855
  %_0.i2155 = fadd float %_0.i2159, %_0.i2583, !dbg !20857
  %_0.i2582 = fmul float %history.i.i433.sroa.22.07622, %_115.i.i.i.i615, !dbg !20859
  %_0.i2154 = fadd float %_0.i2158, %_0.i2582, !dbg !20861
  %_0.i2581 = fmul float %history.i.i433.sroa.22.07622, %_118.i.i.i.i618, !dbg !20863
  %_0.i2153 = fadd float %_0.i2157, %_0.i2581, !dbg !20865
  %_0.i2580 = fmul float %history.i.i433.sroa.26.07623, %_123.i.i.i.i623, !dbg !20867
  %_0.i2152 = fadd float %_0.i2156, %_0.i2580, !dbg !20869
  %_0.i2579 = fmul float %history.i.i433.sroa.26.07623, %_126.i.i.i.i626, !dbg !20871
  %_0.i2151 = fadd float %_0.i2155, %_0.i2579, !dbg !20873
  %_0.i2578 = fmul float %history.i.i433.sroa.26.07623, %_129.i.i.i.i629, !dbg !20875
  %_0.i2150 = fadd float %_0.i2154, %_0.i2578, !dbg !20877
  %_0.i2577 = fmul float %history.i.i433.sroa.26.07623, %_132.i.i.i.i632, !dbg !20879
  %_0.i2149 = fadd float %_0.i2153, %_0.i2577, !dbg !20881
  %_0.i2576 = fmul float %history.i.i433.sroa.29.07624, %_137.i.i.i.i637, !dbg !20883
  %_0.i2148 = fadd float %_0.i2152, %_0.i2576, !dbg !20885
  %_0.i2575 = fmul float %history.i.i433.sroa.29.07624, %_140.i.i.i.i640, !dbg !20887
  %_0.i2147 = fadd float %_0.i2151, %_0.i2575, !dbg !20889
  %_0.i2574 = fmul float %history.i.i433.sroa.29.07624, %_143.i.i.i.i643, !dbg !20891
  %_0.i2146 = fadd float %_0.i2150, %_0.i2574, !dbg !20893
  %_0.i2573 = fmul float %history.i.i433.sroa.29.07624, %_146.i.i.i.i646, !dbg !20895
  %_0.i2145 = fadd float %_0.i2149, %_0.i2573, !dbg !20897
  %_0.i2572 = fmul float %history.i.i433.sroa.32.07625, %_151.i.i.i.i651, !dbg !20899
  %_0.i2144 = fadd float %_0.i2148, %_0.i2572, !dbg !20901
  %_0.i2571 = fmul float %history.i.i433.sroa.32.07625, %_154.i.i.i.i654, !dbg !20903
  %_0.i2143 = fadd float %_0.i2147, %_0.i2571, !dbg !20905
  %_0.i2570 = fmul float %history.i.i433.sroa.32.07625, %_157.i.i.i.i657, !dbg !20907
  %_0.i2142 = fadd float %_0.i2146, %_0.i2570, !dbg !20909
  %_0.i2569 = fmul float %history.i.i433.sroa.32.07625, %_160.i.i.i.i660, !dbg !20911
  %_0.i2141 = fadd float %_0.i2145, %_0.i2569, !dbg !20913
  %_0.i2568 = fmul float %history.i.i433.sroa.35.07626, %_165.i.i.i.i665, !dbg !20915
  %_0.i2140 = fadd float %_0.i2144, %_0.i2568, !dbg !20917
  %_0.i2567 = fmul float %history.i.i433.sroa.35.07626, %_168.i.i.i.i668, !dbg !20919
  %_0.i2139 = fadd float %_0.i2143, %_0.i2567, !dbg !20921
  %_0.i2566 = fmul float %history.i.i433.sroa.35.07626, %_171.i.i.i.i671, !dbg !20923
  %_0.i2138 = fadd float %_0.i2142, %_0.i2566, !dbg !20925
  %_0.i2565 = fmul float %history.i.i433.sroa.35.07626, %_174.i.i.i.i674, !dbg !20927
  %_0.i2137 = fadd float %_0.i2141, %_0.i2565, !dbg !20929
  %391 = tail call noundef float @llvm.fabs.f32(float %_0.i2140), !dbg !20931
  %_3.i.i3602.inv = fcmp ogt float %390, %391, !dbg !20933
  %_4.i.i3609.v = select i1 %_3.i.i3602.inv, float %390, float %391, !dbg !20933
  %392 = tail call noundef float @llvm.fabs.f32(float %_0.i2139), !dbg !20931
  %_3.i.i3602.inv.1 = fcmp ogt float %_4.i.i3609.v, %392, !dbg !20933
  %_4.i.i3609.v.1 = select i1 %_3.i.i3602.inv.1, float %_4.i.i3609.v, float %392, !dbg !20933
  %393 = tail call noundef float @llvm.fabs.f32(float %_0.i2138), !dbg !20931
  %_3.i.i3602.inv.2 = fcmp ogt float %_4.i.i3609.v.1, %393, !dbg !20933
  %_4.i.i3609.v.2 = select i1 %_3.i.i3602.inv.2, float %_4.i.i3609.v.1, float %393, !dbg !20933
  %394 = tail call noundef float @llvm.fabs.f32(float %_0.i2137), !dbg !20931
  %_3.i.i3602.inv.3 = fcmp ogt float %_4.i.i3609.v.2, %394, !dbg !20933
  %_4.i.i3609.v.3 = select i1 %_3.i.i3602.inv.3, float %_4.i.i3609.v.2, float %394, !dbg !20933
  %_39.i.i685 = getelementptr inbounds nuw float, ptr %peaks_right.i434, i32 %iter.sroa.0.0.i.i4557627, !dbg !20936
  store float %_4.i.i3609.v.3, ptr %_39.i.i685, align 4, !dbg !20941, !alias.scope !20943, !noalias !20734
  %exitcond11920.not = icmp eq i32 %389, %umax11932, !dbg !20946
  br i1 %exitcond11920.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i457, label %bb5.i.i492, !dbg !20714

bb7.i.i690:                                       ; preds = %bb5.i.i492
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_11.i.i493, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #33, !dbg !20948, !noalias !20734
  unreachable, !dbg !20948

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i457: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i
  %history.i.i433.sroa.0.0.lcssa = phi float [ %history.i.i433.sroa.0.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %_0.i2963, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ], !dbg !20949
  %history.i.i433.sroa.7.0.lcssa = phi float [ %history.i.i433.sroa.7.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i433.sroa.0.07616, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ], !dbg !20949
  %history.i.i433.sroa.10.0.lcssa = phi float [ %history.i.i433.sroa.10.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i433.sroa.7.07617, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ], !dbg !20949
  %history.i.i433.sroa.13.0.lcssa = phi float [ %history.i.i433.sroa.13.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i433.sroa.10.07618, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ], !dbg !20949
  %history.i.i433.sroa.16.0.lcssa = phi float [ %history.i.i433.sroa.16.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i433.sroa.13.07619, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ], !dbg !20949
  %history.i.i433.sroa.19.0.lcssa = phi float [ %history.i.i433.sroa.19.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i433.sroa.16.07620, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ], !dbg !20949
  %history.i.i433.sroa.22.0.lcssa = phi float [ %history.i.i433.sroa.22.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i433.sroa.19.07621, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ], !dbg !20949
  %history.i.i433.sroa.26.0.lcssa = phi float [ %history.i.i433.sroa.26.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i433.sroa.22.07622, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ], !dbg !20949
  %history.i.i433.sroa.29.0.lcssa = phi float [ %history.i.i433.sroa.29.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i433.sroa.26.07623, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ], !dbg !20949
  %history.i.i433.sroa.32.0.lcssa = phi float [ %history.i.i433.sroa.32.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i433.sroa.29.07624, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ], !dbg !20949
  %history.i.i433.sroa.35.0.lcssa = phi float [ %history.i.i433.sroa.35.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i433.sroa.32.07625, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ], !dbg !20949
  %history.i.i433.sroa.38.0.lcssa = phi float [ %history.i.i433.sroa.38.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i433.sroa.35.07626, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ], !dbg !20949
  store float %history.i.i433.sroa.0.0.lcssa, ptr %hot_right.i436, align 4, !dbg !20950, !noalias !20709
  store float %history.i.i433.sroa.7.0.lcssa, ptr %history.i.i433.sroa.7.0.hot_right.i436.sroa_idx, align 4, !dbg !20950, !noalias !20709
  store float %history.i.i433.sroa.10.0.lcssa, ptr %history.i.i433.sroa.10.0.hot_right.i436.sroa_idx, align 4, !dbg !20950, !noalias !20709
  store float %history.i.i433.sroa.13.0.lcssa, ptr %history.i.i433.sroa.13.0.hot_right.i436.sroa_idx, align 4, !dbg !20950, !noalias !20709
  store float %history.i.i433.sroa.16.0.lcssa, ptr %history.i.i433.sroa.16.0.hot_right.i436.sroa_idx, align 4, !dbg !20950, !noalias !20709
  store float %history.i.i433.sroa.19.0.lcssa, ptr %history.i.i433.sroa.19.0.hot_right.i436.sroa_idx, align 4, !dbg !20950, !noalias !20709
  store float %history.i.i433.sroa.22.0.lcssa, ptr %history.i.i433.sroa.22.0.hot_right.i436.sroa_idx, align 4, !dbg !20950, !noalias !20709
  store float %history.i.i433.sroa.26.0.lcssa, ptr %history.i.i433.sroa.26.0.hot_right.i436.sroa_idx, align 4, !dbg !20950, !noalias !20709
  store float %history.i.i433.sroa.29.0.lcssa, ptr %history.i.i433.sroa.29.0.hot_right.i436.sroa_idx, align 4, !dbg !20950, !noalias !20709
  store float %history.i.i433.sroa.32.0.lcssa, ptr %history.i.i433.sroa.32.0.hot_right.i436.sroa_idx, align 4, !dbg !20950, !noalias !20709
  store float %history.i.i433.sroa.35.0.lcssa, ptr %history.i.i433.sroa.35.0.hot_right.i436.sroa_idx, align 4, !dbg !20950, !noalias !20709
  store float %history.i.i433.sroa.38.0.lcssa, ptr %history.i.i433.sroa.38.0.hot_right.i436.sroa_idx, align 4, !dbg !20950, !noalias !20709
  br i1 %_20.i138.i7588.not, label %bb13.i.loopexit, label %bb42.i.lr.ph, !dbg !20450

bb42.i.lr.ph:                                     ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i457
  %_8.i31.i = load float, ptr %_64.i, align 4, !alias.scope !20951, !noalias !20954, !noundef !10
  %_9.i32.i = load float, ptr %_65.i462, align 4, !alias.scope !20956, !noalias !20957, !noundef !10
  %_8.i.i464 = load float, ptr %_69.i463, align 4, !alias.scope !20958, !noalias !20961, !noundef !10
  %_9.i.i465 = load float, ptr %_70.i, align 4, !alias.scope !20963, !noalias !20964, !noundef !10
  %_87.i = load i32, ptr %348, align 4
  %_62.i89.i = load float, ptr %359, align 4
  %_62.i.i486 = load float, ptr %375, align 4
  %_102.i = load i32, ptr %379, align 4
  %.promoted7670 = load float, ptr %358, align 4
  %.promoted7739 = load float, ptr %360, align 4
  %.promoted7808 = load float, ptr %374, align 4
  %.promoted7877 = load float, ptr %376, align 4
  br label %bb42.i, !dbg !20450

bb42.i:                                           ; preds = %bb42.i.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3117
  %_0.i32167878 = phi float [ %.promoted7877, %bb42.i.lr.ph ], [ %_0.i3216, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3117 ]
  %_0.i28427809 = phi float [ %.promoted7808, %bb42.i.lr.ph ], [ %_0.i2842, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3117 ]
  %_0.i32207740 = phi float [ %.promoted7739, %bb42.i.lr.ph ], [ %_0.i3220, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3117 ]
  %_0.i28467671 = phi float [ %.promoted7670, %bb42.i.lr.ph ], [ %_0.i2846, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3117 ]
  %main_cursor.sroa.0.1.i4607667 = phi i32 [ %main_cursor.sroa.0.0.i4527950, %bb42.i.lr.ph ], [ %spec.store.select11.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3117 ]
  %ring_cursor.sroa.0.1.i4597666 = phi i32 [ %ring_cursor.sroa.0.0.i4517949, %bb42.i.lr.ph ], [ %spec.store.select12.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3117 ]
  %iter1.sroa.0.0.i4587665 = phi i32 [ 0, %bb42.i.lr.ph ], [ %395, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3117 ]
  %395 = add nuw nsw i32 %iter1.sroa.0.0.i4587665, 1, !dbg !20965
  %_60.i = add nuw nsw i32 %iter1.sroa.0.0.i4587665, %iter.sroa.0.0.i7947, !dbg !20971
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20951), !dbg !20972
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20956), !dbg !20972
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20958), !dbg !20973
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20963), !dbg !20973
  %_126.i = getelementptr inbounds nuw float, ptr %peaks_left.i435, i32 %iter1.sroa.0.0.i4587665, !dbg !20974
  %_0.i3001 = load float, ptr %_126.i, align 4, !dbg !20985, !alias.scope !20987, !noalias !20990, !noundef !10
  %_131.i467 = getelementptr inbounds nuw float, ptr %peaks_right.i434, i32 %iter1.sroa.0.0.i4587665, !dbg !20991
  %_0.i2996 = load float, ptr %_131.i467, align 4, !dbg !21001, !alias.scope !21003, !noalias !20990, !noundef !10
  %_3.i.i3629 = fcmp ule float %_0.i2996, %_0.i3001, !dbg !21006
  %_6.i.i3631 = bitcast float %_0.i2996 to i32, !dbg !21009
  %_8.i.i3633 = bitcast float %_0.i3001 to i32, !dbg !21012
  %_4.i.i3636 = select i1 %_3.i.i3629, i32 %_8.i.i3633, i32 %_6.i.i3631, !dbg !21014
  %_5.i3412 = and i32 %_4.i.i3636, %.none.i442, !dbg !21015
  %_7.i3415 = and i32 %_9.i3414, %_8.i.i3633, !dbg !21017
  %_4.i3416 = or disjoint i32 %_5.i3412, %_7.i3415, !dbg !21015
  %_0.i3417 = bitcast i32 %_4.i3416 to float, !dbg !21018
  %_7.i3408 = and i32 %_9.i3414, %_6.i.i3631, !dbg !21020
  %_4.i3409 = or disjoint i32 %_5.i3412, %_7.i3408, !dbg !21022
  %_0.i3410 = bitcast i32 %_4.i3409 to float, !dbg !21023
  %_132.i = icmp ugt i32 %_60.i, %left_io.1, !dbg !21025
  br i1 %_132.i, label %bb46.i, label %bb47.i, !dbg !21025, !prof !902

bb47.i:                                           ; preds = %bb42.i
  %_139.i470 = getelementptr inbounds nuw float, ptr %left_io.0, i32 %_60.i, !dbg !21029
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21034), !dbg !21037
  %_3.not.i2989 = icmp eq i32 %left_io.1, %_60.i, !dbg !21038
  br i1 %_3.not.i2989, label %panic.i2992, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2993, !dbg !21038

panic.i2992:                                      ; preds = %bb47.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #33, !dbg !21038, !noalias !21040
  unreachable, !dbg !21038

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2993: ; preds = %bb47.i
  %_0.i2991 = load float, ptr %_139.i470, align 4, !dbg !21038, !alias.scope !21034, !noalias !20990, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21041), !dbg !21044
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21045), !dbg !21044
  %width.i33.i = load i32, ptr %349, align 4, !dbg !21047, !alias.scope !21048, !noalias !21049, !noundef !10
  %_3.i1957 = fcmp uge float %_8.i31.i, %_0.i3417, !dbg !21052
  %_0.i2390 = fdiv float %_8.i31.i, %_0.i3417, !dbg !21054
  %_0.i3403 = select i1 %_3.i1957, float 1.000000e+00, float %_0.i2390, !dbg !21056
  %_158.1.i38.i = load i32, ptr %350, align 4, !dbg !21058, !alias.scope !21048, !noalias !21049, !noundef !10
  %_22.i39.i = mul i32 %width.i33.i, %ring_cursor.sroa.0.1.i4597666, !dbg !21059
  %_90.i40.i = icmp ugt i32 %_22.i39.i, %_158.1.i38.i, !dbg !21060
  br i1 %_90.i40.i, label %bb34.i124.i, label %bb35.i41.i, !dbg !21060, !prof !902

bb35.i41.i:                                       ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2993
  %_158.0.i42.i = load ptr, ptr %351, align 4, !dbg !21058, !alias.scope !21048, !noalias !21049, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21063), !dbg !21066
  %_4.not.i3142 = icmp eq i32 %_158.1.i38.i, %_22.i39.i, !dbg !21067
  br i1 %_4.not.i3142, label %panic.i3144, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3145, !dbg !21067

panic.i3144:                                      ; preds = %bb35.i41.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #33, !dbg !21067, !noalias !21069
  unreachable, !dbg !21067

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3145: ; preds = %bb35.i41.i
  %_97.i44.i = getelementptr inbounds nuw float, ptr %_158.0.i42.i, i32 %_22.i39.i, !dbg !21070
  store float %_0.i3403, ptr %_97.i44.i, align 4, !dbg !21067, !alias.scope !21063, !noalias !21072
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21073), !dbg !21076
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21077), !dbg !21076
  %width.i1689 = load i32, ptr %349, align 4, !dbg !21079, !alias.scope !21073, !noalias !21081, !noundef !10
  %396 = icmp eq i32 %width.i1689, 0, !dbg !21082
  br i1 %396, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1794, label %bb29.i1695.lr.ph, !dbg !21082

bb29.i1695.lr.ph:                                 ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3145
  %_126.1.i1700 = load i32, ptr %62, align 4, !alias.scope !21073, !noalias !21081, !noundef !10
  %_126.0.i1704 = load ptr, ptr %61, align 4, !nonnull !10
  %397 = add i32 %ring_cursor.sroa.0.1.i4597666, 1
  %_21.not.i1709 = icmp ult i32 %397, %_87.i
  %398 = select i1 %_21.not.i1709, i32 0, i32 %_87.i
  %start1.sroa.0.0.i1710 = sub nuw i32 %397, %398
  %_128.1.i1713 = load i32, ptr %350, align 4
  %_128.0.i1717 = load ptr, ptr %351, align 4, !nonnull !10
  %_130.1.i1718 = load i32, ptr %352, align 4
  %_130.0.i1722 = load ptr, ptr %353, align 4, !nonnull !10
  %_132.1.i1725 = load i32, ptr %354, align 4
  %_132.0.i1729 = load ptr, ptr %355, align 4, !nonnull !10
  %_43.i1742 = mul i32 %width.i1689, %start1.sroa.0.0.i1710
  br label %bb29.i1695, !dbg !21082

bb29.i1695:                                       ; preds = %bb29.i1695.lr.ph, %bb28.i1757
  %iter.sroa.0.0.idx.i16937646 = phi i32 [ 0, %bb29.i1695.lr.ph ], [ %iter.sroa.0.0.add.i1698, %bb28.i1757 ]
  %iter.sroa.4.0.i16927645 = phi i32 [ 0, %bb29.i1695.lr.ph ], [ %_102.0.i1699, %bb28.i1757 ]
  %iter.sroa.7.0.i16917644 = phi i32 [ %width.i1689, %bb29.i1695.lr.ph ], [ %399, %bb28.i1757 ]
  %iter.sroa.0.0.ptr.i16947647 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 %iter.sroa.0.0.idx.i16937646, !dbg !21084
  %399 = add i32 %iter.sroa.7.0.i16917644, -1, !dbg !21084
  %_109.i1696 = icmp eq i32 %iter.sroa.0.0.idx.i16937646, 32, !dbg !21085
  br i1 %_109.i1696, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1794, label %bb33.i1697, !dbg !21089

bb33.i1697:                                       ; preds = %bb29.i1695
  %iter.sroa.0.0.add.i1698 = add nuw nsw i32 %iter.sroa.0.0.idx.i16937646, 4, !dbg !21090
  %_102.0.i1699 = add nuw nsw i32 %iter.sroa.4.0.i16927645, 1, !dbg !21092
  %exitcond11922.not = icmp eq i32 %iter.sroa.4.0.i16927645, %_126.1.i1700, !dbg !21093
  br i1 %exitcond11922.not, label %panic.i1702, label %bb2.i1703, !dbg !21093

bb2.i1703:                                        ; preds = %bb33.i1697
  %400 = getelementptr inbounds nuw %LaneShape, ptr %_126.0.i1704, i32 %iter.sroa.4.0.i16927645, !dbg !21093
  %shape.i1705 = load i32, ptr %400, align 4, !dbg !21093, !noalias !21094, !noundef !10
  %401 = getelementptr inbounds nuw i8, ptr %400, i32 4, !dbg !21093
  %shape3.i1706 = load i32, ptr %401, align 4, !dbg !21093, !noalias !21094, !noundef !10
  %402 = add i32 %shape3.i1706, %ring_cursor.sroa.0.1.i4597666, !dbg !21095
  %_18.not.i1707 = icmp ult i32 %402, %_87.i, !dbg !21096
  %403 = select i1 %_18.not.i1707, i32 0, i32 %_87.i, !dbg !21096
  %spec.select.i1708 = sub nuw i32 %402, %403, !dbg !21096
  %_25.i1711 = mul i32 %spec.select.i1708, %width.i1689, !dbg !21097
  %_24.i1712 = add i32 %_25.i1711, %iter.sroa.4.0.i16927645, !dbg !21097
  %_28.i1714 = icmp ult i32 %_24.i1712, %_128.1.i1713, !dbg !21098
  br i1 %_28.i1714, label %bb9.i1716, label %panic5.i1715, !dbg !21098

panic.i1702:                                      ; preds = %bb33.i1697
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_126.1.i1700, i32 noundef %_126.1.i1700, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_12856b5f9033764dbf23e04eb77c5c46) #33, !dbg !21093, !noalias !21094
  unreachable, !dbg !21093

bb9.i1716:                                        ; preds = %bb2.i1703
  %404 = getelementptr inbounds nuw float, ptr %_128.0.i1717, i32 %_24.i1712, !dbg !21098
  %405 = load float, ptr %404, align 4, !dbg !21098, !noalias !21094, !noundef !10
  %exitcond11923.not = icmp eq i32 %iter.sroa.4.0.i16927645, %_130.1.i1718, !dbg !21099
  br i1 %exitcond11923.not, label %panic6.i1720, label %bb10.i1721, !dbg !21099

panic5.i1715:                                     ; preds = %bb2.i1703
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_24.i1712, i32 noundef %_128.1.i1713, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70ebf0cf1c90c6161926611ccf249a32) #33, !dbg !21098, !noalias !21094
  unreachable, !dbg !21098

bb10.i1721:                                       ; preds = %bb9.i1716
  %406 = getelementptr inbounds nuw i32, ptr %_130.0.i1722, i32 %iter.sroa.4.0.i16927645, !dbg !21099
  %_30.i1723 = load i32, ptr %406, align 4, !dbg !21099, !noalias !21094, !noundef !10
  %407 = icmp eq i32 %_30.i1723, 0, !dbg !21100
  br i1 %407, label %bb14.i1732, label %bb12.i1724, !dbg !21100

panic6.i1720:                                     ; preds = %bb9.i1716
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_130.1.i1718, i32 noundef %_130.1.i1718, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_023d591aaad7f5cf482ac80a9401eca1) #33, !dbg !21099, !noalias !21094
  unreachable, !dbg !21099

bb12.i1724:                                       ; preds = %bb10.i1721
  %_35.i1726 = icmp ult i32 %iter.sroa.4.0.i16927645, %_132.1.i1725, !dbg !21101
  br i1 %_35.i1726, label %bb13.i1728, label %panic7.i1727, !dbg !21101

bb14.i1732:                                       ; preds = %bb34.i1793, %bb13.i1728, %bb10.i1721
  %newest.sroa.0.0.i1733 = phi float [ %405, %bb10.i1721 ], [ %_33.i1730, %bb34.i1793 ], [ %405, %bb13.i1728 ], !dbg !21102
  %exitcond11924.not = icmp eq i32 %iter.sroa.4.0.i16927645, %_132.1.i1725, !dbg !21103
  br i1 %exitcond11924.not, label %panic8.i1736, label %bb15.i1737, !dbg !21103

bb13.i1728:                                       ; preds = %bb12.i1724
  %408 = getelementptr inbounds nuw float, ptr %_132.0.i1729, i32 %iter.sroa.4.0.i16927645, !dbg !21101
  %_33.i1730 = load float, ptr %408, align 4, !dbg !21101, !noalias !21094, !noundef !10
  %_116.i1731 = fcmp olt float %_33.i1730, %405, !dbg !21104
  br i1 %_116.i1731, label %bb34.i1793, label %bb14.i1732, !dbg !21104

panic7.i1727:                                     ; preds = %bb12.i1724
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %iter.sroa.4.0.i16927645, i32 noundef %_132.1.i1725, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a228cac3279aae3a34886f59f51fbe9e) #33, !dbg !21101, !noalias !21094
  unreachable, !dbg !21101

bb34.i1793:                                       ; preds = %bb13.i1728
  br label %bb14.i1732, !dbg !21106

bb15.i1737:                                       ; preds = %bb14.i1732
  %409 = getelementptr inbounds nuw float, ptr %_132.0.i1729, i32 %iter.sroa.4.0.i16927645, !dbg !21103
  store float %newest.sroa.0.0.i1733, ptr %409, align 4, !dbg !21103, !noalias !21094
  %_40.i1739 = add i32 %_30.i1723, 1, !dbg !21107
  %complete.i1740 = icmp eq i32 %_40.i1739, %shape.i1705, !dbg !21107
  br i1 %complete.i1740, label %bb19.i1762, label %bb17.i1741, !dbg !21108

panic8.i1736:                                     ; preds = %bb14.i1732
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_132.1.i1725, i32 noundef %_132.1.i1725, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1fa5d88c1a2cc292a2767c48606a38fe) #33, !dbg !21103, !noalias !21094
  unreachable, !dbg !21103

bb17.i1741:                                       ; preds = %bb15.i1737
  %_42.i1743 = add i32 %iter.sroa.4.0.i16927645, %_43.i1742, !dbg !21109
  %_45.i1745 = icmp ult i32 %_42.i1743, %_128.1.i1713, !dbg !21110
  br i1 %_45.i1745, label %bb27.i1755, label %panic9.i1746, !dbg !21110

panic9.i1746:                                     ; preds = %bb17.i1741
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_42.i1743, i32 noundef %_128.1.i1713, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70d40ae2302f3ea19fa0c4a65fe82786) #33, !dbg !21110, !noalias !21094
  unreachable, !dbg !21110

bb27.i1755:                                       ; preds = %bb17.i1741
  %410 = getelementptr inbounds nuw float, ptr %_128.0.i1717, i32 %_42.i1743, !dbg !21110
  %_41.i1749 = load float, ptr %410, align 4, !dbg !21110, !noalias !21094, !noundef !10
  %_117.i1750 = fcmp olt float %_41.i1749, %newest.sroa.0.0.i1733, !dbg !21111
  %newest.sroa.0.1.i1751 = select i1 %_117.i1750, float %_41.i1749, float %newest.sroa.0.0.i1733, !dbg !21111
  store float %newest.sroa.0.1.i1751, ptr %iter.sroa.0.0.ptr.i16947647, align 4, !dbg !21113, !alias.scope !21077, !noalias !21114
  br label %bb28.i1757, !dbg !21115

bb28.i1757:                                       ; preds = %bb22.i1790, %bb19.i1762, %bb27.i1755
  %storemerge4524 = phi i32 [ %_40.i1739, %bb27.i1755 ], [ 0, %bb19.i1762 ], [ 0, %bb22.i1790 ], !dbg !21116
  store i32 %storemerge4524, ptr %406, align 4, !dbg !21116, !noalias !21094
  %411 = icmp eq i32 %399, 0, !dbg !21082
  br i1 %411, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1794, label %bb29.i1695, !dbg !21082

bb19.i1762:                                       ; preds = %bb15.i1737
  store float %newest.sroa.0.0.i1733, ptr %iter.sroa.0.0.ptr.i16947647, align 4, !dbg !21113, !alias.scope !21077, !noalias !21114
  %_118.i17687640.not = icmp eq i32 %shape.i1705, 0, !dbg !21117
  br i1 %_118.i17687640.not, label %bb28.i1757, label %bb40.i1775.preheader, !dbg !21121

bb40.i1775.preheader:                             ; preds = %bb19.i1762
  %412 = load float, ptr %404, align 4, !dbg !21122, !noalias !21094, !noundef !10
  br label %bb40.i1775, !dbg !21123

bb40.i1775:                                       ; preds = %bb40.i1775.preheader, %bb22.i1790
  %iter2.sroa.0.0.i17677643 = phi i32 [ %_119.i1776, %bb22.i1790 ], [ 0, %bb40.i1775.preheader ]
  %suffix.sroa.0.0.i17667642 = phi float [ %suffix.sroa.0.1.i1786, %bb22.i1790 ], [ %412, %bb40.i1775.preheader ]
  %end.sroa.0.1.i17657641 = phi i32 [ %415, %bb22.i1790 ], [ %spec.select.i1708, %bb40.i1775.preheader ]
  %_54.i1777 = mul i32 %end.sroa.0.1.i17657641, %width.i1689, !dbg !21124
  %_53.i1778 = add i32 %_54.i1777, %iter.sroa.4.0.i16927645, !dbg !21124
  %_57.i1780 = icmp ult i32 %_53.i1778, %_128.1.i1713, !dbg !21123
  br i1 %_57.i1780, label %bb22.i1790, label %panic13.i1781, !dbg !21123

panic13.i1781:                                    ; preds = %bb40.i1775
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_53.i1778, i32 noundef %_128.1.i1713, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a271bbb7fd83c3605ea58a06b7065fa4) #33, !dbg !21123, !noalias !21094
  unreachable, !dbg !21123

bb22.i1790:                                       ; preds = %bb40.i1775
  %_119.i1776 = add nuw i32 %iter2.sroa.0.0.i17677643, 1, !dbg !21125
  %413 = getelementptr inbounds nuw float, ptr %_128.0.i1717, i32 %_53.i1778, !dbg !21123
  %_52.i1784 = load float, ptr %413, align 4, !dbg !21123, !noalias !21094, !noundef !10
  %_121.i1785 = fcmp olt float %suffix.sroa.0.0.i17667642, %_52.i1784, !dbg !21128
  %suffix.sroa.0.1.i1786 = select i1 %_121.i1785, float %suffix.sroa.0.0.i17667642, float %_52.i1784, !dbg !21128
  store float %suffix.sroa.0.1.i1786, ptr %413, align 4, !dbg !21130, !noalias !21094
  %414 = icmp eq i32 %end.sroa.0.1.i17657641, 0, !dbg !21131
  %spec.store.select.i1792 = select i1 %414, i32 %_87.i, i32 %end.sroa.0.1.i17657641, !dbg !21131
  %415 = add i32 %spec.store.select.i1792, -1, !dbg !21132
  %exitcond11921.not = icmp eq i32 %_119.i1776, %shape.i1705, !dbg !21117
  br i1 %exitcond11921.not, label %bb28.i1757, label %bb40.i1775, !dbg !21121

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1794: ; preds = %bb29.i1695, %bb28.i1757, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3145
  %_0.i2988 = load float, ptr %scratch.i, align 4, !dbg !21133, !alias.scope !21135, !noalias !21138, !noundef !10
  %_0.i2618 = fmul float %_0.i2988, 1.638400e+04, !dbg !21139
  %416 = tail call noundef float @llvm.floor.f32(float %_0.i2618), !dbg !21141
  %_0.i2617 = fmul float %416, 0x3F10000000000000, !dbg !21145
  %417 = icmp eq i32 %width.i33.i, 0, !dbg !21147
  %_163.1.i82.i.pre = load i32, ptr %356, align 4, !dbg !21149, !alias.scope !21048, !noalias !21049
  br i1 %417, label %bb53.i77.i, label %bb36.i56.i.lr.ph, !dbg !21147

bb36.i56.i.lr.ph:                                 ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1794
  %_159.1.i61.i = load i32, ptr %62, align 4, !alias.scope !21048, !noalias !21049, !noundef !10
  %_159.0.i65.i = load ptr, ptr %61, align 4, !nonnull !10
  %_161.0.i75.i = load ptr, ptr %357, align 4, !nonnull !10
  %exitcond11925.not = icmp eq i32 %_159.1.i61.i, 0, !dbg !21150
  br i1 %exitcond11925.not, label %panic.i63.i, label %bb14.i64.i, !dbg !21150

bb34.i124.i:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2993
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i39.i, i32 noundef %_158.1.i38.i, i32 noundef %_158.1.i38.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_db7c42041d7aaaba4839906f37294547) #33, !dbg !21151, !noalias !21072
  unreachable, !dbg !21151

bb53.i77.i:                                       ; preds = %bb18.i74.i.7, %bb18.i74.i, %bb18.i74.i.1, %bb18.i74.i.2, %bb18.i74.i.3, %bb18.i74.i.4, %bb18.i74.i.5, %bb18.i74.i.6, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1794
  %_0.i2986 = phi float [ %_0.i2988, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1794 ], [ %_47.i76.i, %bb18.i74.i ], [ %_47.i76.i, %bb18.i74.i.7 ], [ %_47.i76.i, %bb18.i74.i.6 ], [ %_47.i76.i, %bb18.i74.i.5 ], [ %_47.i76.i, %bb18.i74.i.4 ], [ %_47.i76.i, %bb18.i74.i.3 ], [ %_47.i76.i, %bb18.i74.i.2 ], [ %_47.i76.i, %bb18.i74.i.1 ], !dbg !21152
  %_0.i2186 = fadd float %_0.i2617, %_0.i28467671, !dbg !21154
  %_0.i2846 = fsub float %_0.i2186, %_0.i2986, !dbg !21156
  %_123.i83.i = icmp ugt i32 %_22.i39.i, %_163.1.i82.i.pre, !dbg !21158
  br i1 %_123.i83.i, label %bb41.i123.i, label %bb42.i84.i, !dbg !21158, !prof !902

bb14.i64.i:                                       ; preds = %bb36.i56.i.lr.ph
  %418 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i, i32 8, !dbg !21150
  %_42.i66.i = load i32, ptr %418, align 4, !dbg !21150, !noalias !21138, !noundef !10
  %419 = add i32 %_42.i66.i, %ring_cursor.sroa.0.1.i4597666, !dbg !21161
  %_45.not.i67.i = icmp ult i32 %419, %_87.i, !dbg !21162
  %420 = select i1 %_45.not.i67.i, i32 0, i32 %_87.i, !dbg !21162
  %spec.select.i68.i = sub nuw i32 %419, %420, !dbg !21162
  %_49.i69.i = mul i32 %spec.select.i68.i, %width.i33.i, !dbg !21163
  %_51.i72.i = icmp ult i32 %_49.i69.i, %_163.1.i82.i.pre, !dbg !21164
  br i1 %_51.i72.i, label %bb18.i74.i, label %panic1.i73.i, !dbg !21164

panic.i63.i:                                      ; preds = %bb36.i56.i.7, %bb36.i56.i.6, %bb36.i56.i.5, %bb36.i56.i.4, %bb36.i56.i.3, %bb36.i56.i.2, %bb36.i56.i.1, %bb36.i56.i.lr.ph
  %_159.1.i61.i.lcssa.ph = phi i32 [ 7, %bb36.i56.i.7 ], [ 6, %bb36.i56.i.6 ], [ 5, %bb36.i56.i.5 ], [ 4, %bb36.i56.i.4 ], [ 3, %bb36.i56.i.3 ], [ 2, %bb36.i56.i.2 ], [ 1, %bb36.i56.i.1 ], [ 0, %bb36.i56.i.lr.ph ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_159.1.i61.i.lcssa.ph, i32 noundef %_159.1.i61.i.lcssa.ph, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_33746731a929bad63709ddedbca82f5f) #33, !dbg !21150, !noalias !21138
  unreachable, !dbg !21150

bb18.i74.i:                                       ; preds = %bb14.i64.i
  %421 = getelementptr inbounds nuw float, ptr %_161.0.i75.i, i32 %_49.i69.i, !dbg !21164
  %_47.i76.i = load float, ptr %421, align 4, !dbg !21164, !noalias !21138, !noundef !10
  store float %_47.i76.i, ptr %scratch.i, align 4, !dbg !21165, !alias.scope !21045, !noalias !21166
  %422 = icmp eq i32 %width.i33.i, 1, !dbg !21147
  br i1 %422, label %bb53.i77.i, label %bb36.i56.i.1, !dbg !21147

bb36.i56.i.1:                                     ; preds = %bb18.i74.i
  %exitcond11925.1.not = icmp eq i32 %_159.1.i61.i, 1, !dbg !21150
  br i1 %exitcond11925.1.not, label %panic.i63.i, label %bb14.i64.i.1, !dbg !21150

bb14.i64.i.1:                                     ; preds = %bb36.i56.i.1
  %423 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i, i32 20, !dbg !21150
  %_42.i66.i.1 = load i32, ptr %423, align 4, !dbg !21150, !noalias !21138, !noundef !10
  %424 = add i32 %_42.i66.i.1, %ring_cursor.sroa.0.1.i4597666, !dbg !21161
  %_45.not.i67.i.1 = icmp ult i32 %424, %_87.i, !dbg !21162
  %425 = select i1 %_45.not.i67.i.1, i32 0, i32 %_87.i, !dbg !21162
  %spec.select.i68.i.1 = sub nuw i32 %424, %425, !dbg !21162
  %_49.i69.i.1 = mul i32 %spec.select.i68.i.1, %width.i33.i, !dbg !21163
  %_48.i70.i.1 = add i32 %_49.i69.i.1, 1, !dbg !21163
  %_51.i72.i.1 = icmp ult i32 %_48.i70.i.1, %_163.1.i82.i.pre, !dbg !21164
  br i1 %_51.i72.i.1, label %bb18.i74.i.1, label %panic1.i73.i, !dbg !21164

bb18.i74.i.1:                                     ; preds = %bb14.i64.i.1
  %426 = getelementptr inbounds nuw float, ptr %_161.0.i75.i, i32 %_48.i70.i.1, !dbg !21164
  %_47.i76.i.1 = load float, ptr %426, align 4, !dbg !21164, !noalias !21138, !noundef !10
  store float %_47.i76.i.1, ptr %iter.sroa.0.0.ptr.i55.i7651.1, align 4, !dbg !21165, !alias.scope !21045, !noalias !21166
  %427 = icmp eq i32 %width.i33.i, 2, !dbg !21147
  br i1 %427, label %bb53.i77.i, label %bb36.i56.i.2, !dbg !21147

bb36.i56.i.2:                                     ; preds = %bb18.i74.i.1
  %exitcond11925.2.not = icmp eq i32 %_159.1.i61.i, 2, !dbg !21150
  br i1 %exitcond11925.2.not, label %panic.i63.i, label %bb14.i64.i.2, !dbg !21150

bb14.i64.i.2:                                     ; preds = %bb36.i56.i.2
  %428 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i, i32 32, !dbg !21150
  %_42.i66.i.2 = load i32, ptr %428, align 4, !dbg !21150, !noalias !21138, !noundef !10
  %429 = add i32 %_42.i66.i.2, %ring_cursor.sroa.0.1.i4597666, !dbg !21161
  %_45.not.i67.i.2 = icmp ult i32 %429, %_87.i, !dbg !21162
  %430 = select i1 %_45.not.i67.i.2, i32 0, i32 %_87.i, !dbg !21162
  %spec.select.i68.i.2 = sub nuw i32 %429, %430, !dbg !21162
  %_49.i69.i.2 = mul i32 %spec.select.i68.i.2, %width.i33.i, !dbg !21163
  %_48.i70.i.2 = add i32 %_49.i69.i.2, 2, !dbg !21163
  %_51.i72.i.2 = icmp ult i32 %_48.i70.i.2, %_163.1.i82.i.pre, !dbg !21164
  br i1 %_51.i72.i.2, label %bb18.i74.i.2, label %panic1.i73.i, !dbg !21164

bb18.i74.i.2:                                     ; preds = %bb14.i64.i.2
  %431 = getelementptr inbounds nuw float, ptr %_161.0.i75.i, i32 %_48.i70.i.2, !dbg !21164
  %_47.i76.i.2 = load float, ptr %431, align 4, !dbg !21164, !noalias !21138, !noundef !10
  store float %_47.i76.i.2, ptr %iter.sroa.0.0.ptr.i55.i7651.2, align 4, !dbg !21165, !alias.scope !21045, !noalias !21166
  %432 = icmp eq i32 %width.i33.i, 3, !dbg !21147
  br i1 %432, label %bb53.i77.i, label %bb36.i56.i.3, !dbg !21147

bb36.i56.i.3:                                     ; preds = %bb18.i74.i.2
  %exitcond11925.3.not = icmp eq i32 %_159.1.i61.i, 3, !dbg !21150
  br i1 %exitcond11925.3.not, label %panic.i63.i, label %bb14.i64.i.3, !dbg !21150

bb14.i64.i.3:                                     ; preds = %bb36.i56.i.3
  %433 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i, i32 44, !dbg !21150
  %_42.i66.i.3 = load i32, ptr %433, align 4, !dbg !21150, !noalias !21138, !noundef !10
  %434 = add i32 %_42.i66.i.3, %ring_cursor.sroa.0.1.i4597666, !dbg !21161
  %_45.not.i67.i.3 = icmp ult i32 %434, %_87.i, !dbg !21162
  %435 = select i1 %_45.not.i67.i.3, i32 0, i32 %_87.i, !dbg !21162
  %spec.select.i68.i.3 = sub nuw i32 %434, %435, !dbg !21162
  %_49.i69.i.3 = mul i32 %spec.select.i68.i.3, %width.i33.i, !dbg !21163
  %_48.i70.i.3 = add i32 %_49.i69.i.3, 3, !dbg !21163
  %_51.i72.i.3 = icmp ult i32 %_48.i70.i.3, %_163.1.i82.i.pre, !dbg !21164
  br i1 %_51.i72.i.3, label %bb18.i74.i.3, label %panic1.i73.i, !dbg !21164

bb18.i74.i.3:                                     ; preds = %bb14.i64.i.3
  %436 = getelementptr inbounds nuw float, ptr %_161.0.i75.i, i32 %_48.i70.i.3, !dbg !21164
  %_47.i76.i.3 = load float, ptr %436, align 4, !dbg !21164, !noalias !21138, !noundef !10
  store float %_47.i76.i.3, ptr %iter.sroa.0.0.ptr.i55.i7651.3, align 4, !dbg !21165, !alias.scope !21045, !noalias !21166
  %437 = icmp eq i32 %width.i33.i, 4, !dbg !21147
  br i1 %437, label %bb53.i77.i, label %bb36.i56.i.4, !dbg !21147

bb36.i56.i.4:                                     ; preds = %bb18.i74.i.3
  %exitcond11925.4.not = icmp eq i32 %_159.1.i61.i, 4, !dbg !21150
  br i1 %exitcond11925.4.not, label %panic.i63.i, label %bb14.i64.i.4, !dbg !21150

bb14.i64.i.4:                                     ; preds = %bb36.i56.i.4
  %438 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i, i32 56, !dbg !21150
  %_42.i66.i.4 = load i32, ptr %438, align 4, !dbg !21150, !noalias !21138, !noundef !10
  %439 = add i32 %_42.i66.i.4, %ring_cursor.sroa.0.1.i4597666, !dbg !21161
  %_45.not.i67.i.4 = icmp ult i32 %439, %_87.i, !dbg !21162
  %440 = select i1 %_45.not.i67.i.4, i32 0, i32 %_87.i, !dbg !21162
  %spec.select.i68.i.4 = sub nuw i32 %439, %440, !dbg !21162
  %_49.i69.i.4 = mul i32 %spec.select.i68.i.4, %width.i33.i, !dbg !21163
  %_48.i70.i.4 = add i32 %_49.i69.i.4, 4, !dbg !21163
  %_51.i72.i.4 = icmp ult i32 %_48.i70.i.4, %_163.1.i82.i.pre, !dbg !21164
  br i1 %_51.i72.i.4, label %bb18.i74.i.4, label %panic1.i73.i, !dbg !21164

bb18.i74.i.4:                                     ; preds = %bb14.i64.i.4
  %441 = getelementptr inbounds nuw float, ptr %_161.0.i75.i, i32 %_48.i70.i.4, !dbg !21164
  %_47.i76.i.4 = load float, ptr %441, align 4, !dbg !21164, !noalias !21138, !noundef !10
  store float %_47.i76.i.4, ptr %iter.sroa.0.0.ptr.i55.i7651.4, align 4, !dbg !21165, !alias.scope !21045, !noalias !21166
  %442 = icmp eq i32 %width.i33.i, 5, !dbg !21147
  br i1 %442, label %bb53.i77.i, label %bb36.i56.i.5, !dbg !21147

bb36.i56.i.5:                                     ; preds = %bb18.i74.i.4
  %exitcond11925.5.not = icmp eq i32 %_159.1.i61.i, 5, !dbg !21150
  br i1 %exitcond11925.5.not, label %panic.i63.i, label %bb14.i64.i.5, !dbg !21150

bb14.i64.i.5:                                     ; preds = %bb36.i56.i.5
  %443 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i, i32 68, !dbg !21150
  %_42.i66.i.5 = load i32, ptr %443, align 4, !dbg !21150, !noalias !21138, !noundef !10
  %444 = add i32 %_42.i66.i.5, %ring_cursor.sroa.0.1.i4597666, !dbg !21161
  %_45.not.i67.i.5 = icmp ult i32 %444, %_87.i, !dbg !21162
  %445 = select i1 %_45.not.i67.i.5, i32 0, i32 %_87.i, !dbg !21162
  %spec.select.i68.i.5 = sub nuw i32 %444, %445, !dbg !21162
  %_49.i69.i.5 = mul i32 %spec.select.i68.i.5, %width.i33.i, !dbg !21163
  %_48.i70.i.5 = add i32 %_49.i69.i.5, 5, !dbg !21163
  %_51.i72.i.5 = icmp ult i32 %_48.i70.i.5, %_163.1.i82.i.pre, !dbg !21164
  br i1 %_51.i72.i.5, label %bb18.i74.i.5, label %panic1.i73.i, !dbg !21164

bb18.i74.i.5:                                     ; preds = %bb14.i64.i.5
  %446 = getelementptr inbounds nuw float, ptr %_161.0.i75.i, i32 %_48.i70.i.5, !dbg !21164
  %_47.i76.i.5 = load float, ptr %446, align 4, !dbg !21164, !noalias !21138, !noundef !10
  store float %_47.i76.i.5, ptr %iter.sroa.0.0.ptr.i55.i7651.5, align 4, !dbg !21165, !alias.scope !21045, !noalias !21166
  %447 = icmp eq i32 %width.i33.i, 6, !dbg !21147
  br i1 %447, label %bb53.i77.i, label %bb36.i56.i.6, !dbg !21147

bb36.i56.i.6:                                     ; preds = %bb18.i74.i.5
  %exitcond11925.6.not = icmp eq i32 %_159.1.i61.i, 6, !dbg !21150
  br i1 %exitcond11925.6.not, label %panic.i63.i, label %bb14.i64.i.6, !dbg !21150

bb14.i64.i.6:                                     ; preds = %bb36.i56.i.6
  %448 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i, i32 80, !dbg !21150
  %_42.i66.i.6 = load i32, ptr %448, align 4, !dbg !21150, !noalias !21138, !noundef !10
  %449 = add i32 %_42.i66.i.6, %ring_cursor.sroa.0.1.i4597666, !dbg !21161
  %_45.not.i67.i.6 = icmp ult i32 %449, %_87.i, !dbg !21162
  %450 = select i1 %_45.not.i67.i.6, i32 0, i32 %_87.i, !dbg !21162
  %spec.select.i68.i.6 = sub nuw i32 %449, %450, !dbg !21162
  %_49.i69.i.6 = mul i32 %spec.select.i68.i.6, %width.i33.i, !dbg !21163
  %_48.i70.i.6 = add i32 %_49.i69.i.6, 6, !dbg !21163
  %_51.i72.i.6 = icmp ult i32 %_48.i70.i.6, %_163.1.i82.i.pre, !dbg !21164
  br i1 %_51.i72.i.6, label %bb18.i74.i.6, label %panic1.i73.i, !dbg !21164

bb18.i74.i.6:                                     ; preds = %bb14.i64.i.6
  %451 = getelementptr inbounds nuw float, ptr %_161.0.i75.i, i32 %_48.i70.i.6, !dbg !21164
  %_47.i76.i.6 = load float, ptr %451, align 4, !dbg !21164, !noalias !21138, !noundef !10
  store float %_47.i76.i.6, ptr %iter.sroa.0.0.ptr.i55.i7651.6, align 4, !dbg !21165, !alias.scope !21045, !noalias !21166
  %452 = icmp eq i32 %width.i33.i, 7, !dbg !21147
  br i1 %452, label %bb53.i77.i, label %bb36.i56.i.7, !dbg !21147

bb36.i56.i.7:                                     ; preds = %bb18.i74.i.6
  %exitcond11925.7.not = icmp eq i32 %_159.1.i61.i, 7, !dbg !21150
  br i1 %exitcond11925.7.not, label %panic.i63.i, label %bb14.i64.i.7, !dbg !21150

bb14.i64.i.7:                                     ; preds = %bb36.i56.i.7
  %453 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i, i32 92, !dbg !21150
  %_42.i66.i.7 = load i32, ptr %453, align 4, !dbg !21150, !noalias !21138, !noundef !10
  %454 = add i32 %_42.i66.i.7, %ring_cursor.sroa.0.1.i4597666, !dbg !21161
  %_45.not.i67.i.7 = icmp ult i32 %454, %_87.i, !dbg !21162
  %455 = select i1 %_45.not.i67.i.7, i32 0, i32 %_87.i, !dbg !21162
  %spec.select.i68.i.7 = sub nuw i32 %454, %455, !dbg !21162
  %_49.i69.i.7 = mul i32 %spec.select.i68.i.7, %width.i33.i, !dbg !21163
  %_48.i70.i.7 = add i32 %_49.i69.i.7, 7, !dbg !21163
  %_51.i72.i.7 = icmp ult i32 %_48.i70.i.7, %_163.1.i82.i.pre, !dbg !21164
  br i1 %_51.i72.i.7, label %bb18.i74.i.7, label %panic1.i73.i, !dbg !21164

bb18.i74.i.7:                                     ; preds = %bb14.i64.i.7
  %456 = getelementptr inbounds nuw float, ptr %_161.0.i75.i, i32 %_48.i70.i.7, !dbg !21164
  %_47.i76.i.7 = load float, ptr %456, align 4, !dbg !21164, !noalias !21138, !noundef !10
  store float %_47.i76.i.7, ptr %iter.sroa.0.0.ptr.i55.i7651.7, align 4, !dbg !21165, !alias.scope !21045, !noalias !21166
  br label %bb53.i77.i, !dbg !21147

panic1.i73.i:                                     ; preds = %bb14.i64.i.7, %bb14.i64.i.6, %bb14.i64.i.5, %bb14.i64.i.4, %bb14.i64.i.3, %bb14.i64.i.2, %bb14.i64.i.1, %bb14.i64.i
  %_48.i70.i.lcssa.ph = phi i32 [ %_48.i70.i.7, %bb14.i64.i.7 ], [ %_48.i70.i.6, %bb14.i64.i.6 ], [ %_48.i70.i.5, %bb14.i64.i.5 ], [ %_48.i70.i.4, %bb14.i64.i.4 ], [ %_48.i70.i.3, %bb14.i64.i.3 ], [ %_48.i70.i.2, %bb14.i64.i.2 ], [ %_48.i70.i.1, %bb14.i64.i.1 ], [ %_49.i69.i, %bb14.i64.i ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_48.i70.i.lcssa.ph, i32 noundef %_163.1.i82.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_0e76a26fbb751dfb8cf8a069b5b0d39a) #33, !dbg !21164, !noalias !21138
  unreachable, !dbg !21164

bb42.i84.i:                                       ; preds = %bb53.i77.i
  %_163.0.i85.i = load ptr, ptr %357, align 4, !dbg !21149, !alias.scope !21048, !noalias !21049, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21167), !dbg !21170
  %_4.not.i3138 = icmp eq i32 %_163.1.i82.i.pre, %_22.i39.i, !dbg !21171
  br i1 %_4.not.i3138, label %panic.i3140, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3141, !dbg !21171

panic.i3140:                                      ; preds = %bb42.i84.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #33, !dbg !21171, !noalias !21173
  unreachable, !dbg !21171

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3141: ; preds = %bb42.i84.i
  %_130.i87.i = getelementptr inbounds nuw float, ptr %_163.0.i85.i, i32 %_22.i39.i, !dbg !21174
  store float %_0.i2617, ptr %_130.i87.i, align 4, !dbg !21171, !alias.scope !21167, !noalias !21138
  %_0.i2389 = fdiv float %_0.i2846, %_62.i89.i, !dbg !21176
  %_0.i2845 = fsub float 1.000000e+00, %_0.i2389, !dbg !21178
  %_0.i2844 = fsub float %_0.i2845, %_0.i32207740, !dbg !21180
  %_4.i2405 = fmul float %_9.i32.i, %_0.i2844, !dbg !21182
  %_0.i2406 = fadd float %_0.i32207740, %_4.i2405, !dbg !21182
  %_3.i.i3620.inv = fcmp ogt float %_0.i2845, %_0.i2406, !dbg !21184
  %_4.i.i3627.v = select i1 %_3.i.i3620.inv, float %_0.i2845, float %_0.i2406, !dbg !21184
  %457 = tail call noundef float @llvm.fabs.f32(float %_4.i.i3627.v), !dbg !21187
  %458 = fcmp uge float %457, 0x3BC79CA100000000, !dbg !21190
  %_0.i3220 = select i1 %458, float %_4.i.i3627.v, float 0.000000e+00, !dbg !21192
  %_0.i2843 = fsub float 1.000000e+00, %_0.i3220, !dbg !21193
  %_164.1.i101.i = load i32, ptr %361, align 4, !dbg !21195, !alias.scope !21048, !noalias !21049, !noundef !10
  %_74.i102.i = mul i32 %width.i33.i, %main_cursor.sroa.0.1.i4607667, !dbg !21196
  %_134.i103.i = icmp ugt i32 %_74.i102.i, %_164.1.i101.i, !dbg !21197
  br i1 %_134.i103.i, label %bb47.i122.i, label %bb48.i104.i, !dbg !21197, !prof !902

bb41.i123.i:                                      ; preds = %bb53.i77.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i39.i, i32 noundef %_163.1.i82.i.pre, i32 noundef %_163.1.i82.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c2286ff41a33b8c11aa270fe215441de) #33, !dbg !21200, !noalias !21138
  unreachable, !dbg !21200

bb48.i104.i:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3141
  %_164.0.i105.i = load ptr, ptr %362, align 4, !dbg !21195, !alias.scope !21048, !noalias !21049, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21201), !dbg !21204
  %_3.not.i2980 = icmp eq i32 %_164.1.i101.i, %_74.i102.i, !dbg !21205
  br i1 %_3.not.i2980, label %panic.i2983, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3133, !dbg !21205

panic.i2983:                                      ; preds = %bb48.i104.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #33, !dbg !21205, !noalias !21207
  unreachable, !dbg !21205

bb47.i122.i:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3141
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_74.i102.i, i32 noundef %_164.1.i101.i, i32 noundef %_164.1.i101.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a3a21efd18b403ec25db545d522c479) #33, !dbg !21208, !noalias !21138
  unreachable, !dbg !21208

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3133: ; preds = %bb48.i104.i
  %_141.i107.i = getelementptr inbounds nuw float, ptr %_164.0.i105.i, i32 %_74.i102.i, !dbg !21209
  %_0.i2982 = load float, ptr %_141.i107.i, align 4, !dbg !21205, !alias.scope !21201, !noalias !21138, !noundef !10
  store float %_0.i2991, ptr %_141.i107.i, align 4, !dbg !21211, !alias.scope !21213, !noalias !21138
  %_0.i2616 = fmul float %_0.i2843, %_0.i2982, !dbg !21216
  %_6.i3391 = bitcast float %_0.i2982 to i32, !dbg !21218
  %_5.i3392 = and i32 %_6.i3391, %all.sroa.0.0.i444, !dbg !21221
  %_8.i3393 = bitcast float %_0.i2616 to i32, !dbg !21222
  %_7.i3395 = and i32 %_9.i3394, %_8.i3393, !dbg !21224
  %_4.i3396 = or disjoint i32 %_7.i3395, %_5.i3392, !dbg !21221
  store i32 %_4.i3396, ptr %_139.i470, align 4, !dbg !21225, !alias.scope !21227, !noalias !21230
  %_140.i = icmp ugt i32 %_60.i, %right_io.1, !dbg !21231
  br i1 %_140.i, label %bb48.i, label %bb49.i472, !dbg !21231, !prof !902

bb46.i:                                           ; preds = %bb42.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_60.i, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_f462f4e49ffb40af04504eaf795aa606) #33, !dbg !21235, !noalias !20990
  unreachable, !dbg !21235

bb49.i472:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3133
  %_147.i = getelementptr inbounds nuw float, ptr %right_io.0, i32 %_60.i, !dbg !21236
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21241), !dbg !21244
  %_3.not.i2975 = icmp eq i32 %right_io.1, %_60.i, !dbg !21245
  br i1 %_3.not.i2975, label %panic.i2978, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2979, !dbg !21245

panic.i2978:                                      ; preds = %bb49.i472
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #33, !dbg !21245, !noalias !21247
  unreachable, !dbg !21245

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2979: ; preds = %bb49.i472
  %_0.i2977 = load float, ptr %_147.i, align 4, !dbg !21245, !alias.scope !21241, !noalias !20990, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21248), !dbg !21251
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21252), !dbg !21251
  %width.i.i = load i32, ptr %363, align 4, !dbg !21254, !alias.scope !21255, !noalias !21256, !noundef !10
  %_3.i1955 = fcmp uge float %_8.i.i464, %_0.i3410, !dbg !21259
  %_0.i2388 = fdiv float %_8.i.i464, %_0.i3410, !dbg !21261
  %_0.i3390 = select i1 %_3.i1955, float 1.000000e+00, float %_0.i2388, !dbg !21263
  %_158.1.i.i = load i32, ptr %364, align 4, !dbg !21265, !alias.scope !21255, !noalias !21256, !noundef !10
  %_22.i.i477 = mul i32 %width.i.i, %ring_cursor.sroa.0.1.i4597666, !dbg !21266
  %_90.i.i = icmp ugt i32 %_22.i.i477, %_158.1.i.i, !dbg !21267
  br i1 %_90.i.i, label %bb34.i.i, label %bb35.i.i, !dbg !21267, !prof !902

bb35.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2979
  %_158.0.i.i = load ptr, ptr %365, align 4, !dbg !21265, !alias.scope !21255, !noalias !21256, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21270), !dbg !21273
  %_4.not.i3126 = icmp eq i32 %_158.1.i.i, %_22.i.i477, !dbg !21274
  br i1 %_4.not.i3126, label %panic.i3128, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3129, !dbg !21274

panic.i3128:                                      ; preds = %bb35.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #33, !dbg !21274, !noalias !21276
  unreachable, !dbg !21274

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3129: ; preds = %bb35.i.i
  %_97.i.i = getelementptr inbounds nuw float, ptr %_158.0.i.i, i32 %_22.i.i477, !dbg !21277
  store float %_0.i3390, ptr %_97.i.i, align 4, !dbg !21274, !alias.scope !21270, !noalias !21279
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21280), !dbg !21283
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21284), !dbg !21283
  %width.i1583 = load i32, ptr %363, align 4, !dbg !21286, !alias.scope !21280, !noalias !21288, !noundef !10
  %459 = icmp eq i32 %width.i1583, 0, !dbg !21289
  br i1 %459, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1688, label %bb29.i1589.lr.ph, !dbg !21289

bb29.i1589.lr.ph:                                 ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3129
  %_126.1.i1594 = load i32, ptr %366, align 4, !alias.scope !21280, !noalias !21288, !noundef !10
  %_126.0.i1598 = load ptr, ptr %367, align 4, !nonnull !10
  %460 = add i32 %ring_cursor.sroa.0.1.i4597666, 1
  %_21.not.i1603 = icmp ult i32 %460, %_87.i
  %461 = select i1 %_21.not.i1603, i32 0, i32 %_87.i
  %start1.sroa.0.0.i1604 = sub nuw i32 %460, %461
  %_128.1.i1607 = load i32, ptr %364, align 4
  %_128.0.i1611 = load ptr, ptr %365, align 4, !nonnull !10
  %_130.1.i1612 = load i32, ptr %368, align 4
  %_130.0.i1616 = load ptr, ptr %369, align 4, !nonnull !10
  %_132.1.i1619 = load i32, ptr %370, align 4
  %_132.0.i1623 = load ptr, ptr %371, align 4, !nonnull !10
  %_43.i1636 = mul i32 %width.i1583, %start1.sroa.0.0.i1604
  br label %bb29.i1589, !dbg !21289

bb29.i1589:                                       ; preds = %bb29.i1589.lr.ph, %bb28.i1651
  %iter.sroa.0.0.idx.i15877658 = phi i32 [ 0, %bb29.i1589.lr.ph ], [ %iter.sroa.0.0.add.i1592, %bb28.i1651 ]
  %iter.sroa.4.0.i15867657 = phi i32 [ 0, %bb29.i1589.lr.ph ], [ %_102.0.i1593, %bb28.i1651 ]
  %iter.sroa.7.0.i15857656 = phi i32 [ %width.i1583, %bb29.i1589.lr.ph ], [ %462, %bb28.i1651 ]
  %iter.sroa.0.0.ptr.i15887659 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 %iter.sroa.0.0.idx.i15877658, !dbg !21291
  %462 = add i32 %iter.sroa.7.0.i15857656, -1, !dbg !21291
  %_109.i1590 = icmp eq i32 %iter.sroa.0.0.idx.i15877658, 32, !dbg !21292
  br i1 %_109.i1590, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1688.loopexit, label %bb33.i1591, !dbg !21296

bb33.i1591:                                       ; preds = %bb29.i1589
  %iter.sroa.0.0.add.i1592 = add nuw nsw i32 %iter.sroa.0.0.idx.i15877658, 4, !dbg !21297
  %_102.0.i1593 = add nuw nsw i32 %iter.sroa.4.0.i15867657, 1, !dbg !21299
  %exitcond11927.not = icmp eq i32 %iter.sroa.4.0.i15867657, %_126.1.i1594, !dbg !21300
  br i1 %exitcond11927.not, label %panic.i1596, label %bb2.i1597, !dbg !21300

bb2.i1597:                                        ; preds = %bb33.i1591
  %463 = getelementptr inbounds nuw %LaneShape, ptr %_126.0.i1598, i32 %iter.sroa.4.0.i15867657, !dbg !21300
  %shape.i1599 = load i32, ptr %463, align 4, !dbg !21300, !noalias !21301, !noundef !10
  %464 = getelementptr inbounds nuw i8, ptr %463, i32 4, !dbg !21300
  %shape3.i1600 = load i32, ptr %464, align 4, !dbg !21300, !noalias !21301, !noundef !10
  %465 = add i32 %shape3.i1600, %ring_cursor.sroa.0.1.i4597666, !dbg !21302
  %_18.not.i1601 = icmp ult i32 %465, %_87.i, !dbg !21303
  %466 = select i1 %_18.not.i1601, i32 0, i32 %_87.i, !dbg !21303
  %spec.select.i1602 = sub nuw i32 %465, %466, !dbg !21303
  %_25.i1605 = mul i32 %spec.select.i1602, %width.i1583, !dbg !21304
  %_24.i1606 = add i32 %_25.i1605, %iter.sroa.4.0.i15867657, !dbg !21304
  %_28.i1608 = icmp ult i32 %_24.i1606, %_128.1.i1607, !dbg !21305
  br i1 %_28.i1608, label %bb9.i1610, label %panic5.i1609, !dbg !21305

panic.i1596:                                      ; preds = %bb33.i1591
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_126.1.i1594, i32 noundef %_126.1.i1594, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_12856b5f9033764dbf23e04eb77c5c46) #33, !dbg !21300, !noalias !21301
  unreachable, !dbg !21300

bb9.i1610:                                        ; preds = %bb2.i1597
  %467 = getelementptr inbounds nuw float, ptr %_128.0.i1611, i32 %_24.i1606, !dbg !21305
  %468 = load float, ptr %467, align 4, !dbg !21305, !noalias !21301, !noundef !10
  %exitcond11928.not = icmp eq i32 %iter.sroa.4.0.i15867657, %_130.1.i1612, !dbg !21306
  br i1 %exitcond11928.not, label %panic6.i1614, label %bb10.i1615, !dbg !21306

panic5.i1609:                                     ; preds = %bb2.i1597
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_24.i1606, i32 noundef %_128.1.i1607, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70ebf0cf1c90c6161926611ccf249a32) #33, !dbg !21305, !noalias !21301
  unreachable, !dbg !21305

bb10.i1615:                                       ; preds = %bb9.i1610
  %469 = getelementptr inbounds nuw i32, ptr %_130.0.i1616, i32 %iter.sroa.4.0.i15867657, !dbg !21306
  %_30.i1617 = load i32, ptr %469, align 4, !dbg !21306, !noalias !21301, !noundef !10
  %470 = icmp eq i32 %_30.i1617, 0, !dbg !21307
  br i1 %470, label %bb14.i1626, label %bb12.i1618, !dbg !21307

panic6.i1614:                                     ; preds = %bb9.i1610
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_130.1.i1612, i32 noundef %_130.1.i1612, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_023d591aaad7f5cf482ac80a9401eca1) #33, !dbg !21306, !noalias !21301
  unreachable, !dbg !21306

bb12.i1618:                                       ; preds = %bb10.i1615
  %_35.i1620 = icmp ult i32 %iter.sroa.4.0.i15867657, %_132.1.i1619, !dbg !21308
  br i1 %_35.i1620, label %bb13.i1622, label %panic7.i1621, !dbg !21308

bb14.i1626:                                       ; preds = %bb34.i1687, %bb13.i1622, %bb10.i1615
  %newest.sroa.0.0.i1627 = phi float [ %468, %bb10.i1615 ], [ %_33.i1624, %bb34.i1687 ], [ %468, %bb13.i1622 ], !dbg !21309
  %exitcond11929.not = icmp eq i32 %iter.sroa.4.0.i15867657, %_132.1.i1619, !dbg !21310
  br i1 %exitcond11929.not, label %panic8.i1630, label %bb15.i1631, !dbg !21310

bb13.i1622:                                       ; preds = %bb12.i1618
  %471 = getelementptr inbounds nuw float, ptr %_132.0.i1623, i32 %iter.sroa.4.0.i15867657, !dbg !21308
  %_33.i1624 = load float, ptr %471, align 4, !dbg !21308, !noalias !21301, !noundef !10
  %_116.i1625 = fcmp olt float %_33.i1624, %468, !dbg !21311
  br i1 %_116.i1625, label %bb34.i1687, label %bb14.i1626, !dbg !21311

panic7.i1621:                                     ; preds = %bb12.i1618
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %iter.sroa.4.0.i15867657, i32 noundef %_132.1.i1619, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a228cac3279aae3a34886f59f51fbe9e) #33, !dbg !21308, !noalias !21301
  unreachable, !dbg !21308

bb34.i1687:                                       ; preds = %bb13.i1622
  br label %bb14.i1626, !dbg !21313

bb15.i1631:                                       ; preds = %bb14.i1626
  %472 = getelementptr inbounds nuw float, ptr %_132.0.i1623, i32 %iter.sroa.4.0.i15867657, !dbg !21310
  store float %newest.sroa.0.0.i1627, ptr %472, align 4, !dbg !21310, !noalias !21301
  %_40.i1633 = add i32 %_30.i1617, 1, !dbg !21314
  %complete.i1634 = icmp eq i32 %_40.i1633, %shape.i1599, !dbg !21314
  br i1 %complete.i1634, label %bb19.i1656, label %bb17.i1635, !dbg !21315

panic8.i1630:                                     ; preds = %bb14.i1626
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_132.1.i1619, i32 noundef %_132.1.i1619, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1fa5d88c1a2cc292a2767c48606a38fe) #33, !dbg !21310, !noalias !21301
  unreachable, !dbg !21310

bb17.i1635:                                       ; preds = %bb15.i1631
  %_42.i1637 = add i32 %iter.sroa.4.0.i15867657, %_43.i1636, !dbg !21316
  %_45.i1639 = icmp ult i32 %_42.i1637, %_128.1.i1607, !dbg !21317
  br i1 %_45.i1639, label %bb27.i1649, label %panic9.i1640, !dbg !21317

panic9.i1640:                                     ; preds = %bb17.i1635
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_42.i1637, i32 noundef %_128.1.i1607, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70d40ae2302f3ea19fa0c4a65fe82786) #33, !dbg !21317, !noalias !21301
  unreachable, !dbg !21317

bb27.i1649:                                       ; preds = %bb17.i1635
  %473 = getelementptr inbounds nuw float, ptr %_128.0.i1611, i32 %_42.i1637, !dbg !21317
  %_41.i1643 = load float, ptr %473, align 4, !dbg !21317, !noalias !21301, !noundef !10
  %_117.i1644 = fcmp olt float %_41.i1643, %newest.sroa.0.0.i1627, !dbg !21318
  %newest.sroa.0.1.i1645 = select i1 %_117.i1644, float %_41.i1643, float %newest.sroa.0.0.i1627, !dbg !21318
  store float %newest.sroa.0.1.i1645, ptr %iter.sroa.0.0.ptr.i15887659, align 4, !dbg !21320, !alias.scope !21284, !noalias !21321
  br label %bb28.i1651, !dbg !21322

bb28.i1651:                                       ; preds = %bb22.i1684, %bb19.i1656, %bb27.i1649
  %storemerge4527 = phi i32 [ %_40.i1633, %bb27.i1649 ], [ 0, %bb19.i1656 ], [ 0, %bb22.i1684 ], !dbg !21323
  store i32 %storemerge4527, ptr %469, align 4, !dbg !21323, !noalias !21301
  %474 = icmp eq i32 %462, 0, !dbg !21289
  br i1 %474, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1688.loopexit, label %bb29.i1589, !dbg !21289

bb19.i1656:                                       ; preds = %bb15.i1631
  store float %newest.sroa.0.0.i1627, ptr %iter.sroa.0.0.ptr.i15887659, align 4, !dbg !21320, !alias.scope !21284, !noalias !21321
  %_118.i16627652.not = icmp eq i32 %shape.i1599, 0, !dbg !21324
  br i1 %_118.i16627652.not, label %bb28.i1651, label %bb40.i1669.preheader, !dbg !21328

bb40.i1669.preheader:                             ; preds = %bb19.i1656
  %475 = load float, ptr %467, align 4, !dbg !21329, !noalias !21301, !noundef !10
  br label %bb40.i1669, !dbg !21330

bb40.i1669:                                       ; preds = %bb40.i1669.preheader, %bb22.i1684
  %iter2.sroa.0.0.i16617655 = phi i32 [ %_119.i1670, %bb22.i1684 ], [ 0, %bb40.i1669.preheader ]
  %suffix.sroa.0.0.i16607654 = phi float [ %suffix.sroa.0.1.i1680, %bb22.i1684 ], [ %475, %bb40.i1669.preheader ]
  %end.sroa.0.1.i16597653 = phi i32 [ %478, %bb22.i1684 ], [ %spec.select.i1602, %bb40.i1669.preheader ]
  %_54.i1671 = mul i32 %end.sroa.0.1.i16597653, %width.i1583, !dbg !21331
  %_53.i1672 = add i32 %_54.i1671, %iter.sroa.4.0.i15867657, !dbg !21331
  %_57.i1674 = icmp ult i32 %_53.i1672, %_128.1.i1607, !dbg !21330
  br i1 %_57.i1674, label %bb22.i1684, label %panic13.i1675, !dbg !21330

panic13.i1675:                                    ; preds = %bb40.i1669
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_53.i1672, i32 noundef %_128.1.i1607, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a271bbb7fd83c3605ea58a06b7065fa4) #33, !dbg !21330, !noalias !21301
  unreachable, !dbg !21330

bb22.i1684:                                       ; preds = %bb40.i1669
  %_119.i1670 = add nuw i32 %iter2.sroa.0.0.i16617655, 1, !dbg !21332
  %476 = getelementptr inbounds nuw float, ptr %_128.0.i1611, i32 %_53.i1672, !dbg !21330
  %_52.i1678 = load float, ptr %476, align 4, !dbg !21330, !noalias !21301, !noundef !10
  %_121.i1679 = fcmp olt float %suffix.sroa.0.0.i16607654, %_52.i1678, !dbg !21335
  %suffix.sroa.0.1.i1680 = select i1 %_121.i1679, float %suffix.sroa.0.0.i16607654, float %_52.i1678, !dbg !21335
  store float %suffix.sroa.0.1.i1680, ptr %476, align 4, !dbg !21337, !noalias !21301
  %477 = icmp eq i32 %end.sroa.0.1.i16597653, 0, !dbg !21338
  %spec.store.select.i1686 = select i1 %477, i32 %_87.i, i32 %end.sroa.0.1.i16597653, !dbg !21338
  %478 = add i32 %spec.store.select.i1686, -1, !dbg !21339
  %exitcond11926.not = icmp eq i32 %_119.i1670, %shape.i1599, !dbg !21324
  br i1 %exitcond11926.not, label %bb28.i1651, label %bb40.i1669, !dbg !21328

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1688.loopexit: ; preds = %bb28.i1651, %bb29.i1589
  %_0.i2974.pre = load float, ptr %scratch.i, align 4, !dbg !21340, !alias.scope !21342, !noalias !21345
  br label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1688, !dbg !21340

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1688: ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1688.loopexit, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3129
  %_0.i2974 = phi float [ %_0.i2974.pre, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1688.loopexit ], [ %_0.i2986, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3129 ], !dbg !21340
  %_0.i2615 = fmul float %_0.i2974, 1.638400e+04, !dbg !21346
  %479 = tail call noundef float @llvm.floor.f32(float %_0.i2615), !dbg !21348
  %_0.i2614 = fmul float %479, 0x3F10000000000000, !dbg !21352
  %480 = icmp eq i32 %width.i.i, 0, !dbg !21354
  %_163.1.i.i.pre = load i32, ptr %372, align 4, !dbg !21356, !alias.scope !21255, !noalias !21256
  br i1 %480, label %bb53.i.i, label %bb36.i.i.lr.ph, !dbg !21354

bb36.i.i.lr.ph:                                   ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1688
  %_159.1.i.i = load i32, ptr %366, align 4, !alias.scope !21255, !noalias !21256, !noundef !10
  %_159.0.i.i = load ptr, ptr %367, align 4, !nonnull !10
  %_161.0.i.i = load ptr, ptr %373, align 4, !nonnull !10
  %exitcond11930.not = icmp eq i32 %_159.1.i.i, 0, !dbg !21357
  br i1 %exitcond11930.not, label %panic.i.i, label %bb14.i.i, !dbg !21357

bb34.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2979
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i477, i32 noundef %_158.1.i.i, i32 noundef %_158.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_db7c42041d7aaaba4839906f37294547) #33, !dbg !21358, !noalias !21279
  unreachable, !dbg !21358

bb53.i.i:                                         ; preds = %bb18.i.i.7, %bb18.i.i, %bb18.i.i.1, %bb18.i.i.2, %bb18.i.i.3, %bb18.i.i.4, %bb18.i.i.5, %bb18.i.i.6, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1688
  %_0.i2972 = phi float [ %_0.i2974, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1688 ], [ %_47.i.i, %bb18.i.i ], [ %_47.i.i, %bb18.i.i.7 ], [ %_47.i.i, %bb18.i.i.6 ], [ %_47.i.i, %bb18.i.i.5 ], [ %_47.i.i, %bb18.i.i.4 ], [ %_47.i.i, %bb18.i.i.3 ], [ %_47.i.i, %bb18.i.i.2 ], [ %_47.i.i, %bb18.i.i.1 ], !dbg !21359
  %_0.i2185 = fadd float %_0.i2614, %_0.i28427809, !dbg !21361
  %_0.i2842 = fsub float %_0.i2185, %_0.i2972, !dbg !21363
  %_123.i.i = icmp ugt i32 %_22.i.i477, %_163.1.i.i.pre, !dbg !21365
  br i1 %_123.i.i, label %bb41.i.i, label %bb42.i.i, !dbg !21365, !prof !902

bb14.i.i:                                         ; preds = %bb36.i.i.lr.ph
  %481 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 8, !dbg !21357
  %_42.i.i = load i32, ptr %481, align 4, !dbg !21357, !noalias !21345, !noundef !10
  %482 = add i32 %_42.i.i, %ring_cursor.sroa.0.1.i4597666, !dbg !21368
  %_45.not.i.i = icmp ult i32 %482, %_87.i, !dbg !21369
  %483 = select i1 %_45.not.i.i, i32 0, i32 %_87.i, !dbg !21369
  %spec.select.i.i = sub nuw i32 %482, %483, !dbg !21369
  %_49.i.i483 = mul i32 %spec.select.i.i, %width.i.i, !dbg !21370
  %_51.i.i = icmp ult i32 %_49.i.i483, %_163.1.i.i.pre, !dbg !21371
  br i1 %_51.i.i, label %bb18.i.i, label %panic1.i.i, !dbg !21371

panic.i.i:                                        ; preds = %bb36.i.i.7, %bb36.i.i.6, %bb36.i.i.5, %bb36.i.i.4, %bb36.i.i.3, %bb36.i.i.2, %bb36.i.i.1, %bb36.i.i.lr.ph
  %_159.1.i.i.lcssa.ph = phi i32 [ 7, %bb36.i.i.7 ], [ 6, %bb36.i.i.6 ], [ 5, %bb36.i.i.5 ], [ 4, %bb36.i.i.4 ], [ 3, %bb36.i.i.3 ], [ 2, %bb36.i.i.2 ], [ 1, %bb36.i.i.1 ], [ 0, %bb36.i.i.lr.ph ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_159.1.i.i.lcssa.ph, i32 noundef %_159.1.i.i.lcssa.ph, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_33746731a929bad63709ddedbca82f5f) #33, !dbg !21357, !noalias !21345
  unreachable, !dbg !21357

bb18.i.i:                                         ; preds = %bb14.i.i
  %484 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_49.i.i483, !dbg !21371
  %_47.i.i = load float, ptr %484, align 4, !dbg !21371, !noalias !21345, !noundef !10
  store float %_47.i.i, ptr %scratch.i, align 4, !dbg !21372, !alias.scope !21252, !noalias !21373
  %485 = icmp eq i32 %width.i.i, 1, !dbg !21354
  br i1 %485, label %bb53.i.i, label %bb36.i.i.1, !dbg !21354

bb36.i.i.1:                                       ; preds = %bb18.i.i
  %exitcond11930.1.not = icmp eq i32 %_159.1.i.i, 1, !dbg !21357
  br i1 %exitcond11930.1.not, label %panic.i.i, label %bb14.i.i.1, !dbg !21357

bb14.i.i.1:                                       ; preds = %bb36.i.i.1
  %486 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 20, !dbg !21357
  %_42.i.i.1 = load i32, ptr %486, align 4, !dbg !21357, !noalias !21345, !noundef !10
  %487 = add i32 %_42.i.i.1, %ring_cursor.sroa.0.1.i4597666, !dbg !21368
  %_45.not.i.i.1 = icmp ult i32 %487, %_87.i, !dbg !21369
  %488 = select i1 %_45.not.i.i.1, i32 0, i32 %_87.i, !dbg !21369
  %spec.select.i.i.1 = sub nuw i32 %487, %488, !dbg !21369
  %_49.i.i483.1 = mul i32 %spec.select.i.i.1, %width.i.i, !dbg !21370
  %_48.i.i.1 = add i32 %_49.i.i483.1, 1, !dbg !21370
  %_51.i.i.1 = icmp ult i32 %_48.i.i.1, %_163.1.i.i.pre, !dbg !21371
  br i1 %_51.i.i.1, label %bb18.i.i.1, label %panic1.i.i, !dbg !21371

bb18.i.i.1:                                       ; preds = %bb14.i.i.1
  %489 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.1, !dbg !21371
  %_47.i.i.1 = load float, ptr %489, align 4, !dbg !21371, !noalias !21345, !noundef !10
  store float %_47.i.i.1, ptr %iter.sroa.0.0.ptr.i.i7663.1, align 4, !dbg !21372, !alias.scope !21252, !noalias !21373
  %490 = icmp eq i32 %width.i.i, 2, !dbg !21354
  br i1 %490, label %bb53.i.i, label %bb36.i.i.2, !dbg !21354

bb36.i.i.2:                                       ; preds = %bb18.i.i.1
  %exitcond11930.2.not = icmp eq i32 %_159.1.i.i, 2, !dbg !21357
  br i1 %exitcond11930.2.not, label %panic.i.i, label %bb14.i.i.2, !dbg !21357

bb14.i.i.2:                                       ; preds = %bb36.i.i.2
  %491 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 32, !dbg !21357
  %_42.i.i.2 = load i32, ptr %491, align 4, !dbg !21357, !noalias !21345, !noundef !10
  %492 = add i32 %_42.i.i.2, %ring_cursor.sroa.0.1.i4597666, !dbg !21368
  %_45.not.i.i.2 = icmp ult i32 %492, %_87.i, !dbg !21369
  %493 = select i1 %_45.not.i.i.2, i32 0, i32 %_87.i, !dbg !21369
  %spec.select.i.i.2 = sub nuw i32 %492, %493, !dbg !21369
  %_49.i.i483.2 = mul i32 %spec.select.i.i.2, %width.i.i, !dbg !21370
  %_48.i.i.2 = add i32 %_49.i.i483.2, 2, !dbg !21370
  %_51.i.i.2 = icmp ult i32 %_48.i.i.2, %_163.1.i.i.pre, !dbg !21371
  br i1 %_51.i.i.2, label %bb18.i.i.2, label %panic1.i.i, !dbg !21371

bb18.i.i.2:                                       ; preds = %bb14.i.i.2
  %494 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.2, !dbg !21371
  %_47.i.i.2 = load float, ptr %494, align 4, !dbg !21371, !noalias !21345, !noundef !10
  store float %_47.i.i.2, ptr %iter.sroa.0.0.ptr.i.i7663.2, align 4, !dbg !21372, !alias.scope !21252, !noalias !21373
  %495 = icmp eq i32 %width.i.i, 3, !dbg !21354
  br i1 %495, label %bb53.i.i, label %bb36.i.i.3, !dbg !21354

bb36.i.i.3:                                       ; preds = %bb18.i.i.2
  %exitcond11930.3.not = icmp eq i32 %_159.1.i.i, 3, !dbg !21357
  br i1 %exitcond11930.3.not, label %panic.i.i, label %bb14.i.i.3, !dbg !21357

bb14.i.i.3:                                       ; preds = %bb36.i.i.3
  %496 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 44, !dbg !21357
  %_42.i.i.3 = load i32, ptr %496, align 4, !dbg !21357, !noalias !21345, !noundef !10
  %497 = add i32 %_42.i.i.3, %ring_cursor.sroa.0.1.i4597666, !dbg !21368
  %_45.not.i.i.3 = icmp ult i32 %497, %_87.i, !dbg !21369
  %498 = select i1 %_45.not.i.i.3, i32 0, i32 %_87.i, !dbg !21369
  %spec.select.i.i.3 = sub nuw i32 %497, %498, !dbg !21369
  %_49.i.i483.3 = mul i32 %spec.select.i.i.3, %width.i.i, !dbg !21370
  %_48.i.i.3 = add i32 %_49.i.i483.3, 3, !dbg !21370
  %_51.i.i.3 = icmp ult i32 %_48.i.i.3, %_163.1.i.i.pre, !dbg !21371
  br i1 %_51.i.i.3, label %bb18.i.i.3, label %panic1.i.i, !dbg !21371

bb18.i.i.3:                                       ; preds = %bb14.i.i.3
  %499 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.3, !dbg !21371
  %_47.i.i.3 = load float, ptr %499, align 4, !dbg !21371, !noalias !21345, !noundef !10
  store float %_47.i.i.3, ptr %iter.sroa.0.0.ptr.i.i7663.3, align 4, !dbg !21372, !alias.scope !21252, !noalias !21373
  %500 = icmp eq i32 %width.i.i, 4, !dbg !21354
  br i1 %500, label %bb53.i.i, label %bb36.i.i.4, !dbg !21354

bb36.i.i.4:                                       ; preds = %bb18.i.i.3
  %exitcond11930.4.not = icmp eq i32 %_159.1.i.i, 4, !dbg !21357
  br i1 %exitcond11930.4.not, label %panic.i.i, label %bb14.i.i.4, !dbg !21357

bb14.i.i.4:                                       ; preds = %bb36.i.i.4
  %501 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 56, !dbg !21357
  %_42.i.i.4 = load i32, ptr %501, align 4, !dbg !21357, !noalias !21345, !noundef !10
  %502 = add i32 %_42.i.i.4, %ring_cursor.sroa.0.1.i4597666, !dbg !21368
  %_45.not.i.i.4 = icmp ult i32 %502, %_87.i, !dbg !21369
  %503 = select i1 %_45.not.i.i.4, i32 0, i32 %_87.i, !dbg !21369
  %spec.select.i.i.4 = sub nuw i32 %502, %503, !dbg !21369
  %_49.i.i483.4 = mul i32 %spec.select.i.i.4, %width.i.i, !dbg !21370
  %_48.i.i.4 = add i32 %_49.i.i483.4, 4, !dbg !21370
  %_51.i.i.4 = icmp ult i32 %_48.i.i.4, %_163.1.i.i.pre, !dbg !21371
  br i1 %_51.i.i.4, label %bb18.i.i.4, label %panic1.i.i, !dbg !21371

bb18.i.i.4:                                       ; preds = %bb14.i.i.4
  %504 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.4, !dbg !21371
  %_47.i.i.4 = load float, ptr %504, align 4, !dbg !21371, !noalias !21345, !noundef !10
  store float %_47.i.i.4, ptr %iter.sroa.0.0.ptr.i.i7663.4, align 4, !dbg !21372, !alias.scope !21252, !noalias !21373
  %505 = icmp eq i32 %width.i.i, 5, !dbg !21354
  br i1 %505, label %bb53.i.i, label %bb36.i.i.5, !dbg !21354

bb36.i.i.5:                                       ; preds = %bb18.i.i.4
  %exitcond11930.5.not = icmp eq i32 %_159.1.i.i, 5, !dbg !21357
  br i1 %exitcond11930.5.not, label %panic.i.i, label %bb14.i.i.5, !dbg !21357

bb14.i.i.5:                                       ; preds = %bb36.i.i.5
  %506 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 68, !dbg !21357
  %_42.i.i.5 = load i32, ptr %506, align 4, !dbg !21357, !noalias !21345, !noundef !10
  %507 = add i32 %_42.i.i.5, %ring_cursor.sroa.0.1.i4597666, !dbg !21368
  %_45.not.i.i.5 = icmp ult i32 %507, %_87.i, !dbg !21369
  %508 = select i1 %_45.not.i.i.5, i32 0, i32 %_87.i, !dbg !21369
  %spec.select.i.i.5 = sub nuw i32 %507, %508, !dbg !21369
  %_49.i.i483.5 = mul i32 %spec.select.i.i.5, %width.i.i, !dbg !21370
  %_48.i.i.5 = add i32 %_49.i.i483.5, 5, !dbg !21370
  %_51.i.i.5 = icmp ult i32 %_48.i.i.5, %_163.1.i.i.pre, !dbg !21371
  br i1 %_51.i.i.5, label %bb18.i.i.5, label %panic1.i.i, !dbg !21371

bb18.i.i.5:                                       ; preds = %bb14.i.i.5
  %509 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.5, !dbg !21371
  %_47.i.i.5 = load float, ptr %509, align 4, !dbg !21371, !noalias !21345, !noundef !10
  store float %_47.i.i.5, ptr %iter.sroa.0.0.ptr.i.i7663.5, align 4, !dbg !21372, !alias.scope !21252, !noalias !21373
  %510 = icmp eq i32 %width.i.i, 6, !dbg !21354
  br i1 %510, label %bb53.i.i, label %bb36.i.i.6, !dbg !21354

bb36.i.i.6:                                       ; preds = %bb18.i.i.5
  %exitcond11930.6.not = icmp eq i32 %_159.1.i.i, 6, !dbg !21357
  br i1 %exitcond11930.6.not, label %panic.i.i, label %bb14.i.i.6, !dbg !21357

bb14.i.i.6:                                       ; preds = %bb36.i.i.6
  %511 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 80, !dbg !21357
  %_42.i.i.6 = load i32, ptr %511, align 4, !dbg !21357, !noalias !21345, !noundef !10
  %512 = add i32 %_42.i.i.6, %ring_cursor.sroa.0.1.i4597666, !dbg !21368
  %_45.not.i.i.6 = icmp ult i32 %512, %_87.i, !dbg !21369
  %513 = select i1 %_45.not.i.i.6, i32 0, i32 %_87.i, !dbg !21369
  %spec.select.i.i.6 = sub nuw i32 %512, %513, !dbg !21369
  %_49.i.i483.6 = mul i32 %spec.select.i.i.6, %width.i.i, !dbg !21370
  %_48.i.i.6 = add i32 %_49.i.i483.6, 6, !dbg !21370
  %_51.i.i.6 = icmp ult i32 %_48.i.i.6, %_163.1.i.i.pre, !dbg !21371
  br i1 %_51.i.i.6, label %bb18.i.i.6, label %panic1.i.i, !dbg !21371

bb18.i.i.6:                                       ; preds = %bb14.i.i.6
  %514 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.6, !dbg !21371
  %_47.i.i.6 = load float, ptr %514, align 4, !dbg !21371, !noalias !21345, !noundef !10
  store float %_47.i.i.6, ptr %iter.sroa.0.0.ptr.i.i7663.6, align 4, !dbg !21372, !alias.scope !21252, !noalias !21373
  %515 = icmp eq i32 %width.i.i, 7, !dbg !21354
  br i1 %515, label %bb53.i.i, label %bb36.i.i.7, !dbg !21354

bb36.i.i.7:                                       ; preds = %bb18.i.i.6
  %exitcond11930.7.not = icmp eq i32 %_159.1.i.i, 7, !dbg !21357
  br i1 %exitcond11930.7.not, label %panic.i.i, label %bb14.i.i.7, !dbg !21357

bb14.i.i.7:                                       ; preds = %bb36.i.i.7
  %516 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 92, !dbg !21357
  %_42.i.i.7 = load i32, ptr %516, align 4, !dbg !21357, !noalias !21345, !noundef !10
  %517 = add i32 %_42.i.i.7, %ring_cursor.sroa.0.1.i4597666, !dbg !21368
  %_45.not.i.i.7 = icmp ult i32 %517, %_87.i, !dbg !21369
  %518 = select i1 %_45.not.i.i.7, i32 0, i32 %_87.i, !dbg !21369
  %spec.select.i.i.7 = sub nuw i32 %517, %518, !dbg !21369
  %_49.i.i483.7 = mul i32 %spec.select.i.i.7, %width.i.i, !dbg !21370
  %_48.i.i.7 = add i32 %_49.i.i483.7, 7, !dbg !21370
  %_51.i.i.7 = icmp ult i32 %_48.i.i.7, %_163.1.i.i.pre, !dbg !21371
  br i1 %_51.i.i.7, label %bb18.i.i.7, label %panic1.i.i, !dbg !21371

bb18.i.i.7:                                       ; preds = %bb14.i.i.7
  %519 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.7, !dbg !21371
  %_47.i.i.7 = load float, ptr %519, align 4, !dbg !21371, !noalias !21345, !noundef !10
  store float %_47.i.i.7, ptr %iter.sroa.0.0.ptr.i.i7663.7, align 4, !dbg !21372, !alias.scope !21252, !noalias !21373
  br label %bb53.i.i, !dbg !21354

panic1.i.i:                                       ; preds = %bb14.i.i.7, %bb14.i.i.6, %bb14.i.i.5, %bb14.i.i.4, %bb14.i.i.3, %bb14.i.i.2, %bb14.i.i.1, %bb14.i.i
  %_48.i.i.lcssa.ph = phi i32 [ %_48.i.i.7, %bb14.i.i.7 ], [ %_48.i.i.6, %bb14.i.i.6 ], [ %_48.i.i.5, %bb14.i.i.5 ], [ %_48.i.i.4, %bb14.i.i.4 ], [ %_48.i.i.3, %bb14.i.i.3 ], [ %_48.i.i.2, %bb14.i.i.2 ], [ %_48.i.i.1, %bb14.i.i.1 ], [ %_49.i.i483, %bb14.i.i ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_48.i.i.lcssa.ph, i32 noundef %_163.1.i.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_0e76a26fbb751dfb8cf8a069b5b0d39a) #33, !dbg !21371, !noalias !21345
  unreachable, !dbg !21371

bb42.i.i:                                         ; preds = %bb53.i.i
  %_163.0.i.i = load ptr, ptr %373, align 4, !dbg !21356, !alias.scope !21255, !noalias !21256, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21374), !dbg !21377
  %_4.not.i3122 = icmp eq i32 %_163.1.i.i.pre, %_22.i.i477, !dbg !21378
  br i1 %_4.not.i3122, label %panic.i3124, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3125, !dbg !21378

panic.i3124:                                      ; preds = %bb42.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #33, !dbg !21378, !noalias !21380
  unreachable, !dbg !21378

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3125: ; preds = %bb42.i.i
  %_130.i.i = getelementptr inbounds nuw float, ptr %_163.0.i.i, i32 %_22.i.i477, !dbg !21381
  store float %_0.i2614, ptr %_130.i.i, align 4, !dbg !21378, !alias.scope !21374, !noalias !21345
  %_0.i2387 = fdiv float %_0.i2842, %_62.i.i486, !dbg !21383
  %_0.i2841 = fsub float 1.000000e+00, %_0.i2387, !dbg !21385
  %_0.i2840 = fsub float %_0.i2841, %_0.i32167878, !dbg !21387
  %_4.i2403 = fmul float %_9.i.i465, %_0.i2840, !dbg !21389
  %_0.i2404 = fadd float %_0.i32167878, %_4.i2403, !dbg !21389
  %_3.i.i3611.inv = fcmp ogt float %_0.i2841, %_0.i2404, !dbg !21391
  %_4.i.i3618.v = select i1 %_3.i.i3611.inv, float %_0.i2841, float %_0.i2404, !dbg !21391
  %520 = tail call noundef float @llvm.fabs.f32(float %_4.i.i3618.v), !dbg !21394
  %521 = fcmp uge float %520, 0x3BC79CA100000000, !dbg !21397
  %_0.i3216 = select i1 %521, float %_4.i.i3618.v, float 0.000000e+00, !dbg !21399
  %_0.i2839 = fsub float 1.000000e+00, %_0.i3216, !dbg !21400
  %_164.1.i.i = load i32, ptr %377, align 4, !dbg !21402, !alias.scope !21255, !noalias !21256, !noundef !10
  %_74.i.i = mul i32 %width.i.i, %main_cursor.sroa.0.1.i4607667, !dbg !21403
  %_134.i.i = icmp ugt i32 %_74.i.i, %_164.1.i.i, !dbg !21404
  br i1 %_134.i.i, label %bb47.i.i, label %bb48.i.i, !dbg !21404, !prof !902

bb41.i.i:                                         ; preds = %bb53.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i477, i32 noundef %_163.1.i.i.pre, i32 noundef %_163.1.i.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c2286ff41a33b8c11aa270fe215441de) #33, !dbg !21407, !noalias !21345
  unreachable, !dbg !21407

bb48.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3125
  %_164.0.i.i = load ptr, ptr %378, align 4, !dbg !21402, !alias.scope !21255, !noalias !21256, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21408), !dbg !21411
  %_3.not.i2966 = icmp eq i32 %_164.1.i.i, %_74.i.i, !dbg !21412
  br i1 %_3.not.i2966, label %panic.i2969, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3117, !dbg !21412

panic.i2969:                                      ; preds = %bb48.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #33, !dbg !21412, !noalias !21414
  unreachable, !dbg !21412

bb47.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3125
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_74.i.i, i32 noundef %_164.1.i.i, i32 noundef %_164.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a3a21efd18b403ec25db545d522c479) #33, !dbg !21415, !noalias !21345
  unreachable, !dbg !21415

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3117: ; preds = %bb48.i.i
  %_141.i.i = getelementptr inbounds nuw float, ptr %_164.0.i.i, i32 %_74.i.i, !dbg !21416
  %_0.i2968 = load float, ptr %_141.i.i, align 4, !dbg !21412, !alias.scope !21408, !noalias !21345, !noundef !10
  store float %_0.i2977, ptr %_141.i.i, align 4, !dbg !21418, !alias.scope !21420, !noalias !21345
  %_0.i2613 = fmul float %_0.i2839, %_0.i2968, !dbg !21423
  %_6.i3378 = bitcast float %_0.i2968 to i32, !dbg !21425
  %_5.i3379 = and i32 %_6.i3378, %all.sroa.0.0.i444, !dbg !21428
  %_8.i3380 = bitcast float %_0.i2613 to i32, !dbg !21429
  %_7.i3382 = and i32 %_9.i3394, %_8.i3380, !dbg !21431
  %_4.i3383 = or disjoint i32 %_7.i3382, %_5.i3379, !dbg !21428
  store i32 %_4.i3383, ptr %_147.i, align 4, !dbg !21432, !alias.scope !21434, !noalias !21437
  %522 = add i32 %main_cursor.sroa.0.1.i4607667, 1, !dbg !21438
  %_100.i = icmp eq i32 %522, %_102.i, !dbg !21439
  %spec.store.select11.i = select i1 %_100.i, i32 0, i32 %522, !dbg !21439
  %523 = add i32 %ring_cursor.sroa.0.1.i4597666, 1, !dbg !21440
  %_103.i = icmp eq i32 %523, %_87.i, !dbg !21441
  %spec.store.select12.i = select i1 %_103.i, i32 0, i32 %523, !dbg !21441
  %exitcond11933.not = icmp eq i32 %395, %umax11932, !dbg !21442
  br i1 %exitcond11933.not, label %bb16.i.bb13.i.loopexit_crit_edge, label %bb42.i, !dbg !20450

bb48.i:                                           ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3133
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_60.i, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_3ec0ee57975400dd5dbd052e150c5392) #33, !dbg !21445, !noalias !20990
  unreachable, !dbg !21445

_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit: ; preds = %bb13.i.loopexit, %bb11.i
  %ring_cursor.sroa.0.0.i451.lcssa = phi i32 [ %_37.i445, %bb11.i ], [ %ring_cursor.sroa.0.1.i459.lcssa, %bb13.i.loopexit ], !dbg !20404
  %main_cursor.sroa.0.0.i452.lcssa = phi i32 [ %_35.i, %bb11.i ], [ %main_cursor.sroa.0.1.i460.lcssa, %bb13.i.loopexit ], !dbg !20401
  call void @llvm.lifetime.start.p0(ptr nonnull %_106.i), !dbg !21446, !noalias !20384
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(92) %_106.i, ptr noundef nonnull align 4 dereferenceable(92) %hot_left.i437, i32 92, i1 false), !dbg !21446, !noalias !20384
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_106.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33) #32, !dbg !21447, !noalias !20990
  call void @llvm.lifetime.end.p0(ptr nonnull %_106.i), !dbg !21448, !noalias !20384
  call void @llvm.lifetime.start.p0(ptr nonnull %_108.i), !dbg !21449, !noalias !20384
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(92) %_108.i, ptr noundef nonnull align 4 dereferenceable(92) %hot_right.i436, i32 92, i1 false), !dbg !21449, !noalias !20384
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_108.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34) #32, !dbg !21450, !noalias !20990
  call void @llvm.lifetime.end.p0(ptr nonnull %_108.i), !dbg !21451, !noalias !20384
  store i32 %main_cursor.sroa.0.0.i452.lcssa, ptr %_35, align 4, !dbg !21452, !alias.scope !20378, !noalias !20403
  store i32 %ring_cursor.sroa.0.0.i451.lcssa, ptr %81, align 4, !dbg !21453, !alias.scope !20378, !noalias !20403
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i434), !dbg !21454, !noalias !20384
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i435), !dbg !21455, !noalias !20384
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i), !dbg !21456, !noalias !20384
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_right.i436), !dbg !21457, !noalias !20384
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i437), !dbg !21458, !noalias !20384
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockfEB2_.exit, !dbg !20373

bb7.i:                                            ; preds = %bb5.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21459), !dbg !21462
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21463), !dbg !21462
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21465), !dbg !21462
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21467), !dbg !21462
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21469), !dbg !21462
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i36), !dbg !21471, !noalias !21475
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_left.i36, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #32, !dbg !21478, !noalias !21479
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_right.i35), !dbg !21480, !noalias !21475
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_right.i35, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #32, !dbg !21482, !noalias !21483
  %524 = getelementptr inbounds nuw i8, ptr %self, i32 320, !dbg !21484
  %525 = load i8, ptr %524, align 4, !dbg !21484, !range !4765, !alias.scope !21459, !noalias !21488, !noundef !10
  %526 = getelementptr inbounds nuw i8, ptr %self, i32 321, !dbg !21489
  %527 = load i8, ptr %526, align 1, !dbg !21489, !range !4765, !alias.scope !21459, !noalias !21488, !noundef !10
  %528 = getelementptr inbounds nuw i8, ptr %self, i32 528, !dbg !21491
  %ring.i44 = load i32, ptr %528, align 4, !dbg !21491, !alias.scope !21463, !noalias !21493, !noundef !10
  %529 = getelementptr inbounds nuw i8, ptr %self, i32 532, !dbg !21494
  %main.i45 = load i32, ptr %529, align 4, !dbg !21494, !alias.scope !21463, !noalias !21493, !noundef !10
  %_36.i46 = load i32, ptr %_35, align 4, !dbg !21496, !alias.scope !21469, !noalias !21498, !noundef !10
  %530 = getelementptr inbounds nuw i8, ptr %self, i32 108, !dbg !21499
  %_37.i47 = load i32, ptr %530, align 4, !dbg !21499, !alias.scope !21469, !noalias !21498, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i34), !dbg !21501, !noalias !21475
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i34, i8 0, i32 1024, i1 false), !noalias !21475
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i33), !dbg !21503, !noalias !21475
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i33, i8 0, i32 1024, i1 false), !noalias !21475
  %_32.i40 = zext nneg i8 %525 to i32, !dbg !21484
  %.none.i41 = sub nsw i32 0, %_32.i40, !dbg !21505
  %_33.i42 = zext nneg i8 %527 to i32, !dbg !21489
  %all.sroa.0.0.i43 = sub nsw i32 0, %_33.i42, !dbg !21489
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i32), !dbg !21506, !noalias !21475
; call <true_peak_limiter::UniformHot<f32>>::new
  call fastcc void @_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotfE3newB5_(ptr noalias noundef align 4 captures(none) dereferenceable(44) %uniform_left.i32, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33, i32 %ring.i44, i32 %main.i45) #32, !dbg !21508
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_right.i31), !dbg !21509, !noalias !21475
  %_32.val = load i32, ptr %528, align 4, !dbg !21511, !noundef !10
  %_32.val3837 = load i32, ptr %529, align 4, !dbg !21511, !noundef !10
; call <true_peak_limiter::UniformHot<f32>>::new
  call fastcc void @_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotfE3newB5_(ptr noalias noundef align 4 captures(none) dereferenceable(44) %uniform_right.i31, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34, i32 %_32.val, i32 %_32.val3837) #32, !dbg !21511
  %_162.not.i588744 = icmp eq i32 %frames, 0, !dbg !21512
  br i1 %_162.not.i588744, label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit, label %bb44.i59.lr.ph, !dbg !21512

bb44.i59.lr.ph:                                   ; preds = %bb7.i
  %d9.i4041 = lshr i32 %frames, 5, !dbg !21522
  %r2.i4042 = and i32 %frames, 31, !dbg !21529
  %_19.not.i4043 = icmp ne i32 %r2.i4042, 0, !dbg !21530
  %531 = zext i1 %_19.not.i4043 to i32, !dbg !21530
  %yield_count.sroa.0.0.i4044 = add nuw nsw i32 %d9.i4041, %531, !dbg !21530
  %history.i40.i.sroa.7.0.hot_left.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 4
  %history.i40.i.sroa.10.0.hot_left.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 8
  %history.i40.i.sroa.13.0.hot_left.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 12
  %history.i40.i.sroa.16.0.hot_left.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 16
  %history.i40.i.sroa.19.0.hot_left.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 20
  %history.i40.i.sroa.22.0.hot_left.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 24
  %history.i40.i.sroa.26.0.hot_left.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 28
  %history.i40.i.sroa.29.0.hot_left.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 32
  %history.i40.i.sroa.32.0.hot_left.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 36
  %history.i40.i.sroa.35.0.hot_left.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 40
  %history.i40.i.sroa.38.0.hot_left.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 44
  %532 = getelementptr inbounds nuw i8, ptr %self, i32 132
  %533 = getelementptr inbounds nuw i8, ptr %self, i32 136
  %534 = getelementptr inbounds nuw i8, ptr %self, i32 140
  %row1.i.i.i76.i = getelementptr inbounds nuw i8, ptr %self, i32 144
  %535 = getelementptr inbounds nuw i8, ptr %self, i32 148
  %536 = getelementptr inbounds nuw i8, ptr %self, i32 152
  %537 = getelementptr inbounds nuw i8, ptr %self, i32 156
  %row3.i.i.i90.i = getelementptr inbounds nuw i8, ptr %self, i32 160
  %538 = getelementptr inbounds nuw i8, ptr %self, i32 164
  %539 = getelementptr inbounds nuw i8, ptr %self, i32 168
  %540 = getelementptr inbounds nuw i8, ptr %self, i32 172
  %row5.i.i.i104.i = getelementptr inbounds nuw i8, ptr %self, i32 176
  %541 = getelementptr inbounds nuw i8, ptr %self, i32 180
  %542 = getelementptr inbounds nuw i8, ptr %self, i32 184
  %543 = getelementptr inbounds nuw i8, ptr %self, i32 188
  %row7.i.i.i118.i = getelementptr inbounds nuw i8, ptr %self, i32 192
  %544 = getelementptr inbounds nuw i8, ptr %self, i32 196
  %545 = getelementptr inbounds nuw i8, ptr %self, i32 200
  %546 = getelementptr inbounds nuw i8, ptr %self, i32 204
  %row9.i.i.i132.i = getelementptr inbounds nuw i8, ptr %self, i32 208
  %547 = getelementptr inbounds nuw i8, ptr %self, i32 212
  %548 = getelementptr inbounds nuw i8, ptr %self, i32 216
  %549 = getelementptr inbounds nuw i8, ptr %self, i32 220
  %row11.i.i.i146.i = getelementptr inbounds nuw i8, ptr %self, i32 224
  %550 = getelementptr inbounds nuw i8, ptr %self, i32 228
  %551 = getelementptr inbounds nuw i8, ptr %self, i32 232
  %552 = getelementptr inbounds nuw i8, ptr %self, i32 236
  %row13.i.i.i160.i = getelementptr inbounds nuw i8, ptr %self, i32 240
  %553 = getelementptr inbounds nuw i8, ptr %self, i32 244
  %554 = getelementptr inbounds nuw i8, ptr %self, i32 248
  %555 = getelementptr inbounds nuw i8, ptr %self, i32 252
  %row15.i.i.i174.i = getelementptr inbounds nuw i8, ptr %self, i32 256
  %556 = getelementptr inbounds nuw i8, ptr %self, i32 260
  %557 = getelementptr inbounds nuw i8, ptr %self, i32 264
  %558 = getelementptr inbounds nuw i8, ptr %self, i32 268
  %row17.i.i.i188.i = getelementptr inbounds nuw i8, ptr %self, i32 272
  %559 = getelementptr inbounds nuw i8, ptr %self, i32 276
  %560 = getelementptr inbounds nuw i8, ptr %self, i32 280
  %561 = getelementptr inbounds nuw i8, ptr %self, i32 284
  %row19.i.i.i202.i = getelementptr inbounds nuw i8, ptr %self, i32 288
  %562 = getelementptr inbounds nuw i8, ptr %self, i32 292
  %563 = getelementptr inbounds nuw i8, ptr %self, i32 296
  %564 = getelementptr inbounds nuw i8, ptr %self, i32 300
  %row21.i.i.i216.i = getelementptr inbounds nuw i8, ptr %self, i32 304
  %565 = getelementptr inbounds nuw i8, ptr %self, i32 308
  %566 = getelementptr inbounds nuw i8, ptr %self, i32 312
  %567 = getelementptr inbounds nuw i8, ptr %self, i32 316
  %history.i.i13.sroa.7.0.hot_right.i35.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i35, i32 4
  %history.i.i13.sroa.10.0.hot_right.i35.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i35, i32 8
  %history.i.i13.sroa.13.0.hot_right.i35.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i35, i32 12
  %history.i.i13.sroa.16.0.hot_right.i35.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i35, i32 16
  %history.i.i13.sroa.19.0.hot_right.i35.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i35, i32 20
  %history.i.i13.sroa.22.0.hot_right.i35.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i35, i32 24
  %history.i.i13.sroa.26.0.hot_right.i35.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i35, i32 28
  %history.i.i13.sroa.29.0.hot_right.i35.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i35, i32 32
  %history.i.i13.sroa.32.0.hot_right.i35.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i35, i32 36
  %history.i.i13.sroa.35.0.hot_right.i35.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i35, i32 40
  %history.i.i13.sroa.38.0.hot_right.i35.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i35, i32 44
  %568 = getelementptr inbounds nuw i8, ptr %uniform_left.i32, i32 32
  %_68.i29.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i32, i32 36
  %_68.i29.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i32, i32 40
  %569 = getelementptr inbounds nuw i8, ptr %uniform_right.i31, i32 32
  %_69.i28.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i31, i32 36
  %_69.i28.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i31, i32 40
  %_110.i111 = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 48
  %_111.i112 = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 64
  %570 = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 60
  %571 = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 56
  %572 = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 52
  %573 = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 76
  %574 = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 72
  %575 = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 68
  %_115.i113 = getelementptr inbounds nuw i8, ptr %hot_right.i35, i32 48
  %_116.i114 = getelementptr inbounds nuw i8, ptr %hot_right.i35, i32 64
  %576 = getelementptr inbounds nuw i8, ptr %hot_right.i35, i32 60
  %577 = getelementptr inbounds nuw i8, ptr %hot_right.i35, i32 56
  %578 = getelementptr inbounds nuw i8, ptr %hot_right.i35, i32 52
  %579 = getelementptr inbounds nuw i8, ptr %hot_right.i35, i32 76
  %580 = getelementptr inbounds nuw i8, ptr %hot_right.i35, i32 72
  %581 = getelementptr inbounds nuw i8, ptr %hot_right.i35, i32 68
  %_9.i3454 = add nsw i32 %_32.i40, -1
  %582 = getelementptr inbounds nuw i8, ptr %uniform_left.i32, i32 4
  %_21.i271.i = getelementptr inbounds nuw i8, ptr %uniform_left.i32, i32 24
  %_22.i272.i = getelementptr inbounds nuw i8, ptr %uniform_left.i32, i32 28
  %583 = getelementptr inbounds nuw i8, ptr %uniform_left.i32, i32 8
  %584 = getelementptr inbounds nuw i8, ptr %uniform_left.i32, i32 12
  %585 = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 84
  %586 = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 88
  %587 = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 80
  %588 = getelementptr inbounds nuw i8, ptr %uniform_left.i32, i32 20
  %589 = getelementptr inbounds nuw i8, ptr %uniform_left.i32, i32 16
  %_9.i3434 = add nsw i32 %_33.i42, -1
  %590 = getelementptr inbounds nuw i8, ptr %uniform_right.i31, i32 4
  %_21.i.i159 = getelementptr inbounds nuw i8, ptr %uniform_right.i31, i32 24
  %_22.i.i160 = getelementptr inbounds nuw i8, ptr %uniform_right.i31, i32 28
  %591 = getelementptr inbounds nuw i8, ptr %uniform_right.i31, i32 8
  %592 = getelementptr inbounds nuw i8, ptr %uniform_right.i31, i32 12
  %593 = getelementptr inbounds nuw i8, ptr %hot_right.i35, i32 84
  %594 = getelementptr inbounds nuw i8, ptr %hot_right.i35, i32 88
  %595 = getelementptr inbounds nuw i8, ptr %hot_right.i35, i32 80
  %596 = getelementptr inbounds nuw i8, ptr %uniform_right.i31, i32 20
  %597 = getelementptr inbounds nuw i8, ptr %uniform_right.i31, i32 16
  br label %bb44.i59, !dbg !21512

bb19.i65.bb15.i53.loopexit_crit_edge:             ; preds = %bb67.i203
  store float %_0.i.i.lcssa1268514051, ptr %570, align 4
  store float %_0.i3247.lcssa1267114069, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265714087, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264314105, ptr %573, align 4
  store float %_0.i3260.lcssa1262914123, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261514141, ptr %574, align 4
  store float %_0.i.i3513.lcssa1260114159, ptr %576, align 4
  store float %_0.i3273.lcssa1258714177, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257314195, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255914213, ptr %579, align 4
  store float %_0.i3286.lcssa1254514231, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1253114249, ptr %580, align 4
  store float %_0.i2854.lcssa1271614267, ptr %585, align 4
  store float %_0.i3228.lcssa1273214285, ptr %587, align 4
  store float %_0.i2850.lcssa1275614303, ptr %593, align 4
  store float %_0.i3224.lcssa1275714321, ptr %595, align 4
  store i32 %storemerge.i1185.lcssa83928601, ptr %_22.i272.i, align 4
  store float %running.sroa.0.0.i1180.lcssa84198637, ptr %_21.i271.i, align 4
  store i32 %storemerge.i.lcssa85048673, ptr %_22.i.i160, align 4
  store float %running.sroa.0.0.i.lcssa85318709, ptr %_21.i.i159, align 4
  br label %bb15.i53.loopexit, !dbg !21531

bb15.i53.loopexit:                                ; preds = %bb19.i65.bb15.i53.loopexit_crit_edge, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i64
  %ring_cursor.sroa.0.1.i66.lcssa = phi i32 [ %ring_cursor.sroa.0.2.i206, %bb19.i65.bb15.i53.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i548745, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i64 ], !dbg !21535
  %main_cursor.sroa.0.1.i67.lcssa = phi i32 [ %main_cursor.sroa.0.2.i209, %bb19.i65.bb15.i53.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i558746, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i64 ], !dbg !21536
  %_162.not.i58 = icmp eq i32 %599, 0, !dbg !21512
  %indvars.iv.next11935 = add i32 %indvars.iv11934, -32, !dbg !21512
  br i1 %_162.not.i58, label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit, label %bb44.i59, !dbg !21512

bb44.i59:                                         ; preds = %bb44.i59.lr.ph, %bb15.i53.loopexit
  %indvars.iv11934 = phi i32 [ %frames, %bb44.i59.lr.ph ], [ %indvars.iv.next11935, %bb15.i53.loopexit ]
  %iter2.sroa.0.0.i578748 = phi i32 [ %yield_count.sroa.0.0.i4044, %bb44.i59.lr.ph ], [ %599, %bb15.i53.loopexit ]
  %iter1.sroa.0.0.i568747 = phi i32 [ 0, %bb44.i59.lr.ph ], [ %598, %bb15.i53.loopexit ]
  %main_cursor.sroa.0.0.i558746 = phi i32 [ %_36.i46, %bb44.i59.lr.ph ], [ %main_cursor.sroa.0.1.i67.lcssa, %bb15.i53.loopexit ]
  %ring_cursor.sroa.0.0.i548745 = phi i32 [ %_37.i47, %bb44.i59.lr.ph ], [ %ring_cursor.sroa.0.1.i66.lcssa, %bb15.i53.loopexit ]
  %umin11954 = call i32 @llvm.umin.i32(i32 %indvars.iv11934, i32 32), !dbg !21537
  %umax11940 = call i32 @llvm.umax.i32(i32 %umin11954, i32 1), !dbg !21537
  %598 = add i32 %iter1.sroa.0.0.i568747, 32, !dbg !21537
  %599 = add nsw i32 %iter2.sroa.0.0.i578748, -1, !dbg !21541
  %600 = sub i32 %frames, %iter1.sroa.0.0.i568747, !dbg !21542
  %spec.store.select.i60 = tail call i32 @llvm.umin.i32(i32 %600, i32 32), !dbg !21543
  %history.i40.i.sroa.0.0.copyload = load float, ptr %hot_left.i36, align 4, !dbg !21548, !noalias !21550
  %history.i40.i.sroa.7.0.copyload = load float, ptr %history.i40.i.sroa.7.0.hot_left.i36.sroa_idx, align 4, !dbg !21548, !noalias !21550
  %history.i40.i.sroa.10.0.copyload = load float, ptr %history.i40.i.sroa.10.0.hot_left.i36.sroa_idx, align 4, !dbg !21548, !noalias !21550
  %history.i40.i.sroa.13.0.copyload = load float, ptr %history.i40.i.sroa.13.0.hot_left.i36.sroa_idx, align 4, !dbg !21548, !noalias !21550
  %history.i40.i.sroa.16.0.copyload = load float, ptr %history.i40.i.sroa.16.0.hot_left.i36.sroa_idx, align 4, !dbg !21548, !noalias !21550
  %history.i40.i.sroa.19.0.copyload = load float, ptr %history.i40.i.sroa.19.0.hot_left.i36.sroa_idx, align 4, !dbg !21548, !noalias !21550
  %history.i40.i.sroa.22.0.copyload = load float, ptr %history.i40.i.sroa.22.0.hot_left.i36.sroa_idx, align 4, !dbg !21548, !noalias !21550
  %history.i40.i.sroa.26.0.copyload = load float, ptr %history.i40.i.sroa.26.0.hot_left.i36.sroa_idx, align 4, !dbg !21548, !noalias !21550
  %history.i40.i.sroa.29.0.copyload = load float, ptr %history.i40.i.sroa.29.0.hot_left.i36.sroa_idx, align 4, !dbg !21548, !noalias !21550
  %history.i40.i.sroa.32.0.copyload = load float, ptr %history.i40.i.sroa.32.0.hot_left.i36.sroa_idx, align 4, !dbg !21548, !noalias !21550
  %history.i40.i.sroa.35.0.copyload = load float, ptr %history.i40.i.sroa.35.0.hot_left.i36.sroa_idx, align 4, !dbg !21548, !noalias !21550
  %history.i40.i.sroa.38.0.copyload = load float, ptr %history.i40.i.sroa.38.0.hot_left.i36.sroa_idx, align 4, !dbg !21548, !noalias !21550
  %_20.i43.i7955.not = icmp eq i32 %frames, %iter1.sroa.0.0.i568747, !dbg !21555
  br i1 %_20.i43.i7955.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i, label %bb5.i44.i.lr.ph, !dbg !21559

bb5.i44.i.lr.ph:                                  ; preds = %bb44.i59
  %_11.i.i.i64.i = load float, ptr %_31, align 4
  %_14.i.i.i67.i = load float, ptr %532, align 4
  %_17.i.i.i70.i = load float, ptr %533, align 4
  %_20.i.i.i73.i = load float, ptr %534, align 4
  %_25.i.i.i78.i = load float, ptr %row1.i.i.i76.i, align 4
  %_28.i.i.i81.i = load float, ptr %535, align 4
  %_31.i.i.i84.i = load float, ptr %536, align 4
  %_34.i.i.i87.i = load float, ptr %537, align 4
  %_39.i.i.i92.i = load float, ptr %row3.i.i.i90.i, align 4
  %_42.i.i.i95.i = load float, ptr %538, align 4
  %_45.i.i.i98.i = load float, ptr %539, align 4
  %_48.i.i.i101.i = load float, ptr %540, align 4
  %_53.i.i.i106.i = load float, ptr %row5.i.i.i104.i, align 4
  %_56.i.i.i109.i = load float, ptr %541, align 4
  %_59.i.i.i112.i = load float, ptr %542, align 4
  %_62.i.i.i115.i = load float, ptr %543, align 4
  %_67.i.i.i120.i = load float, ptr %row7.i.i.i118.i, align 4
  %_70.i.i.i123.i = load float, ptr %544, align 4
  %_73.i.i.i126.i = load float, ptr %545, align 4
  %_76.i.i.i129.i = load float, ptr %546, align 4
  %_81.i.i.i134.i = load float, ptr %row9.i.i.i132.i, align 4
  %_84.i.i.i137.i = load float, ptr %547, align 4
  %_87.i.i.i140.i = load float, ptr %548, align 4
  %_90.i.i.i143.i = load float, ptr %549, align 4
  %_95.i.i.i148.i = load float, ptr %row11.i.i.i146.i, align 4
  %_98.i.i.i151.i = load float, ptr %550, align 4
  %_101.i.i.i154.i = load float, ptr %551, align 4
  %_104.i.i.i157.i = load float, ptr %552, align 4
  %_109.i.i.i162.i = load float, ptr %row13.i.i.i160.i, align 4
  %_112.i.i.i165.i = load float, ptr %553, align 4
  %_115.i.i.i168.i = load float, ptr %554, align 4
  %_118.i.i.i171.i = load float, ptr %555, align 4
  %_123.i.i.i176.i = load float, ptr %row15.i.i.i174.i, align 4
  %_126.i.i.i179.i = load float, ptr %556, align 4
  %_129.i.i.i182.i = load float, ptr %557, align 4
  %_132.i.i.i185.i = load float, ptr %558, align 4
  %_137.i.i.i190.i = load float, ptr %row17.i.i.i188.i, align 4
  %_140.i.i.i193.i = load float, ptr %559, align 4
  %_143.i.i.i196.i = load float, ptr %560, align 4
  %_146.i.i.i199.i = load float, ptr %561, align 4
  %_151.i.i.i204.i = load float, ptr %row19.i.i.i202.i, align 4
  %_154.i.i.i207.i = load float, ptr %562, align 4
  %_157.i.i.i210.i = load float, ptr %563, align 4
  %_160.i.i.i213.i = load float, ptr %564, align 4
  %_165.i.i.i218.i = load float, ptr %row21.i.i.i216.i, align 4
  %_168.i.i.i221.i = load float, ptr %565, align 4
  %_171.i.i.i224.i = load float, ptr %566, align 4
  %_174.i.i.i227.i = load float, ptr %567, align 4
  br label %bb5.i44.i, !dbg !21559

bb5.i44.i:                                        ; preds = %bb5.i44.i.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008
  %iter.sroa.0.0.i42.i7967 = phi i32 [ 0, %bb5.i44.i.lr.ph ], [ %601, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ]
  %history.i40.i.sroa.35.07966 = phi float [ %history.i40.i.sroa.35.0.copyload, %bb5.i44.i.lr.ph ], [ %history.i40.i.sroa.32.07965, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ]
  %history.i40.i.sroa.32.07965 = phi float [ %history.i40.i.sroa.32.0.copyload, %bb5.i44.i.lr.ph ], [ %history.i40.i.sroa.29.07964, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ]
  %history.i40.i.sroa.29.07964 = phi float [ %history.i40.i.sroa.29.0.copyload, %bb5.i44.i.lr.ph ], [ %history.i40.i.sroa.26.07963, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ]
  %history.i40.i.sroa.26.07963 = phi float [ %history.i40.i.sroa.26.0.copyload, %bb5.i44.i.lr.ph ], [ %history.i40.i.sroa.22.07962, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ]
  %history.i40.i.sroa.22.07962 = phi float [ %history.i40.i.sroa.22.0.copyload, %bb5.i44.i.lr.ph ], [ %history.i40.i.sroa.19.07961, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ]
  %history.i40.i.sroa.19.07961 = phi float [ %history.i40.i.sroa.19.0.copyload, %bb5.i44.i.lr.ph ], [ %history.i40.i.sroa.16.07960, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ]
  %history.i40.i.sroa.16.07960 = phi float [ %history.i40.i.sroa.16.0.copyload, %bb5.i44.i.lr.ph ], [ %history.i40.i.sroa.13.07959, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ]
  %history.i40.i.sroa.13.07959 = phi float [ %history.i40.i.sroa.13.0.copyload, %bb5.i44.i.lr.ph ], [ %history.i40.i.sroa.10.07958, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ]
  %history.i40.i.sroa.10.07958 = phi float [ %history.i40.i.sroa.10.0.copyload, %bb5.i44.i.lr.ph ], [ %history.i40.i.sroa.7.07957, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ]
  %history.i40.i.sroa.7.07957 = phi float [ %history.i40.i.sroa.7.0.copyload, %bb5.i44.i.lr.ph ], [ %history.i40.i.sroa.0.07956, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ]
  %history.i40.i.sroa.0.07956 = phi float [ %history.i40.i.sroa.0.0.copyload, %bb5.i44.i.lr.ph ], [ %_0.i3006, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ]
  %601 = add nuw nsw i32 %iter.sroa.0.0.i42.i7967, 1, !dbg !21560
  %_11.i45.i = add nuw nsw i32 %iter.sroa.0.0.i42.i7967, %iter1.sroa.0.0.i568747, !dbg !21563
  %_24.i46.i = icmp ugt i32 %_11.i45.i, %left_io.1, !dbg !21564
  br i1 %_24.i46.i, label %bb7.i243.i, label %bb8.i47.i, !dbg !21564, !prof !902

bb8.i47.i:                                        ; preds = %bb5.i44.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21567), !dbg !21570
  %_3.not.i3004 = icmp eq i32 %left_io.1, %_11.i45.i, !dbg !21571
  br i1 %_3.not.i3004, label %panic.i3007, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008, !dbg !21571

panic.i3007:                                      ; preds = %bb8.i47.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #33, !dbg !21571, !noalias !21573
  unreachable, !dbg !21571

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008: ; preds = %bb8.i47.i
  %_31.i49.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %_11.i45.i, !dbg !21575
  %_0.i3006 = load float, ptr %_31.i49.i, align 4, !dbg !21571, !alias.scope !21567, !noalias !21577, !noundef !10
  %602 = tail call noundef float @llvm.fabs.f32(float %history.i40.i.sroa.19.07961), !dbg !21578
  %_0.i2666 = fmul float %_0.i3006, %_11.i.i.i64.i, !dbg !21581
  %_0.i2234 = fadd float %_0.i2666, 0.000000e+00, !dbg !21584
  %_0.i2665 = fmul float %_0.i3006, %_14.i.i.i67.i, !dbg !21586
  %_0.i2233 = fadd float %_0.i2665, 0.000000e+00, !dbg !21588
  %_0.i2664 = fmul float %_0.i3006, %_17.i.i.i70.i, !dbg !21590
  %_0.i2232 = fadd float %_0.i2664, 0.000000e+00, !dbg !21592
  %_0.i2663 = fmul float %_0.i3006, %_20.i.i.i73.i, !dbg !21594
  %_0.i2231 = fadd float %_0.i2663, 0.000000e+00, !dbg !21596
  %_0.i2662 = fmul float %history.i40.i.sroa.0.07956, %_25.i.i.i78.i, !dbg !21598
  %_0.i2230 = fadd float %_0.i2234, %_0.i2662, !dbg !21600
  %_0.i2661 = fmul float %history.i40.i.sroa.0.07956, %_28.i.i.i81.i, !dbg !21602
  %_0.i2229 = fadd float %_0.i2233, %_0.i2661, !dbg !21604
  %_0.i2660 = fmul float %history.i40.i.sroa.0.07956, %_31.i.i.i84.i, !dbg !21606
  %_0.i2228 = fadd float %_0.i2232, %_0.i2660, !dbg !21608
  %_0.i2659 = fmul float %history.i40.i.sroa.0.07956, %_34.i.i.i87.i, !dbg !21610
  %_0.i2227 = fadd float %_0.i2231, %_0.i2659, !dbg !21612
  %_0.i2658 = fmul float %history.i40.i.sroa.7.07957, %_39.i.i.i92.i, !dbg !21614
  %_0.i2226 = fadd float %_0.i2230, %_0.i2658, !dbg !21616
  %_0.i2657 = fmul float %history.i40.i.sroa.7.07957, %_42.i.i.i95.i, !dbg !21618
  %_0.i2225 = fadd float %_0.i2229, %_0.i2657, !dbg !21620
  %_0.i2656 = fmul float %history.i40.i.sroa.7.07957, %_45.i.i.i98.i, !dbg !21622
  %_0.i2224 = fadd float %_0.i2228, %_0.i2656, !dbg !21624
  %_0.i2655 = fmul float %history.i40.i.sroa.7.07957, %_48.i.i.i101.i, !dbg !21626
  %_0.i2223 = fadd float %_0.i2227, %_0.i2655, !dbg !21628
  %_0.i2654 = fmul float %history.i40.i.sroa.10.07958, %_53.i.i.i106.i, !dbg !21630
  %_0.i2222 = fadd float %_0.i2226, %_0.i2654, !dbg !21632
  %_0.i2653 = fmul float %history.i40.i.sroa.10.07958, %_56.i.i.i109.i, !dbg !21634
  %_0.i2221 = fadd float %_0.i2225, %_0.i2653, !dbg !21636
  %_0.i2652 = fmul float %history.i40.i.sroa.10.07958, %_59.i.i.i112.i, !dbg !21638
  %_0.i2220 = fadd float %_0.i2224, %_0.i2652, !dbg !21640
  %_0.i2651 = fmul float %history.i40.i.sroa.10.07958, %_62.i.i.i115.i, !dbg !21642
  %_0.i2219 = fadd float %_0.i2223, %_0.i2651, !dbg !21644
  %_0.i2650 = fmul float %history.i40.i.sroa.13.07959, %_67.i.i.i120.i, !dbg !21646
  %_0.i2218 = fadd float %_0.i2222, %_0.i2650, !dbg !21648
  %_0.i2649 = fmul float %history.i40.i.sroa.13.07959, %_70.i.i.i123.i, !dbg !21650
  %_0.i2217 = fadd float %_0.i2221, %_0.i2649, !dbg !21652
  %_0.i2648 = fmul float %history.i40.i.sroa.13.07959, %_73.i.i.i126.i, !dbg !21654
  %_0.i2216 = fadd float %_0.i2220, %_0.i2648, !dbg !21656
  %_0.i2647 = fmul float %history.i40.i.sroa.13.07959, %_76.i.i.i129.i, !dbg !21658
  %_0.i2215 = fadd float %_0.i2219, %_0.i2647, !dbg !21660
  %_0.i2646 = fmul float %history.i40.i.sroa.16.07960, %_81.i.i.i134.i, !dbg !21662
  %_0.i2214 = fadd float %_0.i2218, %_0.i2646, !dbg !21664
  %_0.i2645 = fmul float %history.i40.i.sroa.16.07960, %_84.i.i.i137.i, !dbg !21666
  %_0.i2213 = fadd float %_0.i2217, %_0.i2645, !dbg !21668
  %_0.i2644 = fmul float %history.i40.i.sroa.16.07960, %_87.i.i.i140.i, !dbg !21670
  %_0.i2212 = fadd float %_0.i2216, %_0.i2644, !dbg !21672
  %_0.i2643 = fmul float %history.i40.i.sroa.16.07960, %_90.i.i.i143.i, !dbg !21674
  %_0.i2211 = fadd float %_0.i2215, %_0.i2643, !dbg !21676
  %_0.i2642 = fmul float %history.i40.i.sroa.19.07961, %_95.i.i.i148.i, !dbg !21678
  %_0.i2210 = fadd float %_0.i2214, %_0.i2642, !dbg !21680
  %_0.i2641 = fmul float %history.i40.i.sroa.19.07961, %_98.i.i.i151.i, !dbg !21682
  %_0.i2209 = fadd float %_0.i2213, %_0.i2641, !dbg !21684
  %_0.i2640 = fmul float %history.i40.i.sroa.19.07961, %_101.i.i.i154.i, !dbg !21686
  %_0.i2208 = fadd float %_0.i2212, %_0.i2640, !dbg !21688
  %_0.i2639 = fmul float %history.i40.i.sroa.19.07961, %_104.i.i.i157.i, !dbg !21690
  %_0.i2207 = fadd float %_0.i2211, %_0.i2639, !dbg !21692
  %_0.i2638 = fmul float %history.i40.i.sroa.22.07962, %_109.i.i.i162.i, !dbg !21694
  %_0.i2206 = fadd float %_0.i2210, %_0.i2638, !dbg !21696
  %_0.i2637 = fmul float %history.i40.i.sroa.22.07962, %_112.i.i.i165.i, !dbg !21698
  %_0.i2205 = fadd float %_0.i2209, %_0.i2637, !dbg !21700
  %_0.i2636 = fmul float %history.i40.i.sroa.22.07962, %_115.i.i.i168.i, !dbg !21702
  %_0.i2204 = fadd float %_0.i2208, %_0.i2636, !dbg !21704
  %_0.i2635 = fmul float %history.i40.i.sroa.22.07962, %_118.i.i.i171.i, !dbg !21706
  %_0.i2203 = fadd float %_0.i2207, %_0.i2635, !dbg !21708
  %_0.i2634 = fmul float %history.i40.i.sroa.26.07963, %_123.i.i.i176.i, !dbg !21710
  %_0.i2202 = fadd float %_0.i2206, %_0.i2634, !dbg !21712
  %_0.i2633 = fmul float %history.i40.i.sroa.26.07963, %_126.i.i.i179.i, !dbg !21714
  %_0.i2201 = fadd float %_0.i2205, %_0.i2633, !dbg !21716
  %_0.i2632 = fmul float %history.i40.i.sroa.26.07963, %_129.i.i.i182.i, !dbg !21718
  %_0.i2200 = fadd float %_0.i2204, %_0.i2632, !dbg !21720
  %_0.i2631 = fmul float %history.i40.i.sroa.26.07963, %_132.i.i.i185.i, !dbg !21722
  %_0.i2199 = fadd float %_0.i2203, %_0.i2631, !dbg !21724
  %_0.i2630 = fmul float %history.i40.i.sroa.29.07964, %_137.i.i.i190.i, !dbg !21726
  %_0.i2198 = fadd float %_0.i2202, %_0.i2630, !dbg !21728
  %_0.i2629 = fmul float %history.i40.i.sroa.29.07964, %_140.i.i.i193.i, !dbg !21730
  %_0.i2197 = fadd float %_0.i2201, %_0.i2629, !dbg !21732
  %_0.i2628 = fmul float %history.i40.i.sroa.29.07964, %_143.i.i.i196.i, !dbg !21734
  %_0.i2196 = fadd float %_0.i2200, %_0.i2628, !dbg !21736
  %_0.i2627 = fmul float %history.i40.i.sroa.29.07964, %_146.i.i.i199.i, !dbg !21738
  %_0.i2195 = fadd float %_0.i2199, %_0.i2627, !dbg !21740
  %_0.i2626 = fmul float %history.i40.i.sroa.32.07965, %_151.i.i.i204.i, !dbg !21742
  %_0.i2194 = fadd float %_0.i2198, %_0.i2626, !dbg !21744
  %_0.i2625 = fmul float %history.i40.i.sroa.32.07965, %_154.i.i.i207.i, !dbg !21746
  %_0.i2193 = fadd float %_0.i2197, %_0.i2625, !dbg !21748
  %_0.i2624 = fmul float %history.i40.i.sroa.32.07965, %_157.i.i.i210.i, !dbg !21750
  %_0.i2192 = fadd float %_0.i2196, %_0.i2624, !dbg !21752
  %_0.i2623 = fmul float %history.i40.i.sroa.32.07965, %_160.i.i.i213.i, !dbg !21754
  %_0.i2191 = fadd float %_0.i2195, %_0.i2623, !dbg !21756
  %_0.i2622 = fmul float %history.i40.i.sroa.35.07966, %_165.i.i.i218.i, !dbg !21758
  %_0.i2190 = fadd float %_0.i2194, %_0.i2622, !dbg !21760
  %_0.i2621 = fmul float %history.i40.i.sroa.35.07966, %_168.i.i.i221.i, !dbg !21762
  %_0.i2189 = fadd float %_0.i2193, %_0.i2621, !dbg !21764
  %_0.i2620 = fmul float %history.i40.i.sroa.35.07966, %_171.i.i.i224.i, !dbg !21766
  %_0.i2188 = fadd float %_0.i2192, %_0.i2620, !dbg !21768
  %_0.i2619 = fmul float %history.i40.i.sroa.35.07966, %_174.i.i.i227.i, !dbg !21770
  %_0.i2187 = fadd float %_0.i2191, %_0.i2619, !dbg !21772
  %603 = tail call noundef float @llvm.fabs.f32(float %_0.i2190), !dbg !21774
  %_3.i.i3638.inv = fcmp ogt float %602, %603, !dbg !21776
  %_4.i.i3645.v = select i1 %_3.i.i3638.inv, float %602, float %603, !dbg !21776
  %604 = tail call noundef float @llvm.fabs.f32(float %_0.i2189), !dbg !21774
  %_3.i.i3638.inv.1 = fcmp ogt float %_4.i.i3645.v, %604, !dbg !21776
  %_4.i.i3645.v.1 = select i1 %_3.i.i3638.inv.1, float %_4.i.i3645.v, float %604, !dbg !21776
  %605 = tail call noundef float @llvm.fabs.f32(float %_0.i2188), !dbg !21774
  %_3.i.i3638.inv.2 = fcmp ogt float %_4.i.i3645.v.1, %605, !dbg !21776
  %_4.i.i3645.v.2 = select i1 %_3.i.i3638.inv.2, float %_4.i.i3645.v.1, float %605, !dbg !21776
  %606 = tail call noundef float @llvm.fabs.f32(float %_0.i2187), !dbg !21774
  %_3.i.i3638.inv.3 = fcmp ogt float %_4.i.i3645.v.2, %606, !dbg !21776
  %_4.i.i3645.v.3 = select i1 %_3.i.i3638.inv.3, float %_4.i.i3645.v.2, float %606, !dbg !21776
  %_39.i238.i = getelementptr inbounds nuw float, ptr %peaks_left.i34, i32 %iter.sroa.0.0.i42.i7967, !dbg !21779
  store float %_4.i.i3645.v.3, ptr %_39.i238.i, align 4, !dbg !21784, !alias.scope !21786, !noalias !21577
  %exitcond11938.not = icmp eq i32 %601, %umax11940, !dbg !21555
  br i1 %exitcond11938.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i, label %bb5.i44.i, !dbg !21559

bb7.i243.i:                                       ; preds = %bb5.i44.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_11.i45.i, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #33, !dbg !21789, !noalias !21577
  unreachable, !dbg !21789

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008, %bb44.i59
  %history.i40.i.sroa.0.0.lcssa = phi float [ %history.i40.i.sroa.0.0.copyload, %bb44.i59 ], [ %_0.i3006, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ], !dbg !21790
  %history.i40.i.sroa.7.0.lcssa = phi float [ %history.i40.i.sroa.7.0.copyload, %bb44.i59 ], [ %history.i40.i.sroa.0.07956, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ], !dbg !21790
  %history.i40.i.sroa.10.0.lcssa = phi float [ %history.i40.i.sroa.10.0.copyload, %bb44.i59 ], [ %history.i40.i.sroa.7.07957, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ], !dbg !21790
  %history.i40.i.sroa.13.0.lcssa = phi float [ %history.i40.i.sroa.13.0.copyload, %bb44.i59 ], [ %history.i40.i.sroa.10.07958, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ], !dbg !21790
  %history.i40.i.sroa.16.0.lcssa = phi float [ %history.i40.i.sroa.16.0.copyload, %bb44.i59 ], [ %history.i40.i.sroa.13.07959, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ], !dbg !21790
  %history.i40.i.sroa.19.0.lcssa = phi float [ %history.i40.i.sroa.19.0.copyload, %bb44.i59 ], [ %history.i40.i.sroa.16.07960, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ], !dbg !21790
  %history.i40.i.sroa.22.0.lcssa = phi float [ %history.i40.i.sroa.22.0.copyload, %bb44.i59 ], [ %history.i40.i.sroa.19.07961, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ], !dbg !21790
  %history.i40.i.sroa.26.0.lcssa = phi float [ %history.i40.i.sroa.26.0.copyload, %bb44.i59 ], [ %history.i40.i.sroa.22.07962, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ], !dbg !21790
  %history.i40.i.sroa.29.0.lcssa = phi float [ %history.i40.i.sroa.29.0.copyload, %bb44.i59 ], [ %history.i40.i.sroa.26.07963, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ], !dbg !21790
  %history.i40.i.sroa.32.0.lcssa = phi float [ %history.i40.i.sroa.32.0.copyload, %bb44.i59 ], [ %history.i40.i.sroa.29.07964, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ], !dbg !21790
  %history.i40.i.sroa.35.0.lcssa = phi float [ %history.i40.i.sroa.35.0.copyload, %bb44.i59 ], [ %history.i40.i.sroa.32.07965, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ], !dbg !21790
  %history.i40.i.sroa.38.0.lcssa = phi float [ %history.i40.i.sroa.38.0.copyload, %bb44.i59 ], [ %history.i40.i.sroa.35.07966, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ], !dbg !21790
  store float %history.i40.i.sroa.0.0.lcssa, ptr %hot_left.i36, align 4, !dbg !21791, !noalias !21550
  store float %history.i40.i.sroa.7.0.lcssa, ptr %history.i40.i.sroa.7.0.hot_left.i36.sroa_idx, align 4, !dbg !21791, !noalias !21550
  store float %history.i40.i.sroa.10.0.lcssa, ptr %history.i40.i.sroa.10.0.hot_left.i36.sroa_idx, align 4, !dbg !21791, !noalias !21550
  store float %history.i40.i.sroa.13.0.lcssa, ptr %history.i40.i.sroa.13.0.hot_left.i36.sroa_idx, align 4, !dbg !21791, !noalias !21550
  store float %history.i40.i.sroa.16.0.lcssa, ptr %history.i40.i.sroa.16.0.hot_left.i36.sroa_idx, align 4, !dbg !21791, !noalias !21550
  store float %history.i40.i.sroa.19.0.lcssa, ptr %history.i40.i.sroa.19.0.hot_left.i36.sroa_idx, align 4, !dbg !21791, !noalias !21550
  store float %history.i40.i.sroa.22.0.lcssa, ptr %history.i40.i.sroa.22.0.hot_left.i36.sroa_idx, align 4, !dbg !21791, !noalias !21550
  store float %history.i40.i.sroa.26.0.lcssa, ptr %history.i40.i.sroa.26.0.hot_left.i36.sroa_idx, align 4, !dbg !21791, !noalias !21550
  store float %history.i40.i.sroa.29.0.lcssa, ptr %history.i40.i.sroa.29.0.hot_left.i36.sroa_idx, align 4, !dbg !21791, !noalias !21550
  store float %history.i40.i.sroa.32.0.lcssa, ptr %history.i40.i.sroa.32.0.hot_left.i36.sroa_idx, align 4, !dbg !21791, !noalias !21550
  store float %history.i40.i.sroa.35.0.lcssa, ptr %history.i40.i.sroa.35.0.hot_left.i36.sroa_idx, align 4, !dbg !21791, !noalias !21550
  store float %history.i40.i.sroa.38.0.lcssa, ptr %history.i40.i.sroa.38.0.hot_left.i36.sroa_idx, align 4, !dbg !21791, !noalias !21550
  %history.i.i13.sroa.0.0.copyload = load float, ptr %hot_right.i35, align 4, !dbg !21792, !noalias !21794
  %history.i.i13.sroa.7.0.copyload = load float, ptr %history.i.i13.sroa.7.0.hot_right.i35.sroa_idx, align 4, !dbg !21792, !noalias !21794
  %history.i.i13.sroa.10.0.copyload = load float, ptr %history.i.i13.sroa.10.0.hot_right.i35.sroa_idx, align 4, !dbg !21792, !noalias !21794
  %history.i.i13.sroa.13.0.copyload = load float, ptr %history.i.i13.sroa.13.0.hot_right.i35.sroa_idx, align 4, !dbg !21792, !noalias !21794
  %history.i.i13.sroa.16.0.copyload = load float, ptr %history.i.i13.sroa.16.0.hot_right.i35.sroa_idx, align 4, !dbg !21792, !noalias !21794
  %history.i.i13.sroa.19.0.copyload = load float, ptr %history.i.i13.sroa.19.0.hot_right.i35.sroa_idx, align 4, !dbg !21792, !noalias !21794
  %history.i.i13.sroa.22.0.copyload = load float, ptr %history.i.i13.sroa.22.0.hot_right.i35.sroa_idx, align 4, !dbg !21792, !noalias !21794
  %history.i.i13.sroa.26.0.copyload = load float, ptr %history.i.i13.sroa.26.0.hot_right.i35.sroa_idx, align 4, !dbg !21792, !noalias !21794
  %history.i.i13.sroa.29.0.copyload = load float, ptr %history.i.i13.sroa.29.0.hot_right.i35.sroa_idx, align 4, !dbg !21792, !noalias !21794
  %history.i.i13.sroa.32.0.copyload = load float, ptr %history.i.i13.sroa.32.0.hot_right.i35.sroa_idx, align 4, !dbg !21792, !noalias !21794
  %history.i.i13.sroa.35.0.copyload = load float, ptr %history.i.i13.sroa.35.0.hot_right.i35.sroa_idx, align 4, !dbg !21792, !noalias !21794
  %history.i.i13.sroa.38.0.copyload = load float, ptr %history.i.i13.sroa.38.0.hot_right.i35.sroa_idx, align 4, !dbg !21792, !noalias !21794
  br i1 %_20.i43.i7955.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i64, label %bb5.i.i212.lr.ph, !dbg !21799

bb5.i.i212.lr.ph:                                 ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i
  %_11.i.i.i.i231 = load float, ptr %_31, align 4
  %_14.i.i.i.i234 = load float, ptr %532, align 4
  %_17.i.i.i.i237 = load float, ptr %533, align 4
  %_20.i.i.i.i240 = load float, ptr %534, align 4
  %_25.i.i.i.i245 = load float, ptr %row1.i.i.i76.i, align 4
  %_28.i.i.i.i248 = load float, ptr %535, align 4
  %_31.i.i.i.i251 = load float, ptr %536, align 4
  %_34.i.i.i.i254 = load float, ptr %537, align 4
  %_39.i.i.i.i259 = load float, ptr %row3.i.i.i90.i, align 4
  %_42.i.i.i.i262 = load float, ptr %538, align 4
  %_45.i.i.i.i265 = load float, ptr %539, align 4
  %_48.i.i.i.i268 = load float, ptr %540, align 4
  %_53.i.i.i.i273 = load float, ptr %row5.i.i.i104.i, align 4
  %_56.i.i.i.i276 = load float, ptr %541, align 4
  %_59.i.i.i.i279 = load float, ptr %542, align 4
  %_62.i.i.i.i282 = load float, ptr %543, align 4
  %_67.i.i.i.i287 = load float, ptr %row7.i.i.i118.i, align 4
  %_70.i.i.i.i290 = load float, ptr %544, align 4
  %_73.i.i.i.i293 = load float, ptr %545, align 4
  %_76.i.i.i.i296 = load float, ptr %546, align 4
  %_81.i.i.i.i301 = load float, ptr %row9.i.i.i132.i, align 4
  %_84.i.i.i.i304 = load float, ptr %547, align 4
  %_87.i.i.i.i307 = load float, ptr %548, align 4
  %_90.i.i.i.i310 = load float, ptr %549, align 4
  %_95.i.i.i.i315 = load float, ptr %row11.i.i.i146.i, align 4
  %_98.i.i.i.i318 = load float, ptr %550, align 4
  %_101.i.i.i.i321 = load float, ptr %551, align 4
  %_104.i.i.i.i324 = load float, ptr %552, align 4
  %_109.i.i.i.i329 = load float, ptr %row13.i.i.i160.i, align 4
  %_112.i.i.i.i332 = load float, ptr %553, align 4
  %_115.i.i.i.i335 = load float, ptr %554, align 4
  %_118.i.i.i.i338 = load float, ptr %555, align 4
  %_123.i.i.i.i343 = load float, ptr %row15.i.i.i174.i, align 4
  %_126.i.i.i.i346 = load float, ptr %556, align 4
  %_129.i.i.i.i349 = load float, ptr %557, align 4
  %_132.i.i.i.i352 = load float, ptr %558, align 4
  %_137.i.i.i.i357 = load float, ptr %row17.i.i.i188.i, align 4
  %_140.i.i.i.i360 = load float, ptr %559, align 4
  %_143.i.i.i.i363 = load float, ptr %560, align 4
  %_146.i.i.i.i366 = load float, ptr %561, align 4
  %_151.i.i.i.i371 = load float, ptr %row19.i.i.i202.i, align 4
  %_154.i.i.i.i374 = load float, ptr %562, align 4
  %_157.i.i.i.i377 = load float, ptr %563, align 4
  %_160.i.i.i.i380 = load float, ptr %564, align 4
  %_165.i.i.i.i385 = load float, ptr %row21.i.i.i216.i, align 4
  %_168.i.i.i.i388 = load float, ptr %565, align 4
  %_171.i.i.i.i391 = load float, ptr %566, align 4
  %_174.i.i.i.i394 = load float, ptr %567, align 4
  br label %bb5.i.i212, !dbg !21799

bb5.i.i212:                                       ; preds = %bb5.i.i212.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013
  %iter.sroa.0.0.i.i627994 = phi i32 [ 0, %bb5.i.i212.lr.ph ], [ %607, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ]
  %history.i.i13.sroa.35.07993 = phi float [ %history.i.i13.sroa.35.0.copyload, %bb5.i.i212.lr.ph ], [ %history.i.i13.sroa.32.07992, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ]
  %history.i.i13.sroa.32.07992 = phi float [ %history.i.i13.sroa.32.0.copyload, %bb5.i.i212.lr.ph ], [ %history.i.i13.sroa.29.07991, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ]
  %history.i.i13.sroa.29.07991 = phi float [ %history.i.i13.sroa.29.0.copyload, %bb5.i.i212.lr.ph ], [ %history.i.i13.sroa.26.07990, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ]
  %history.i.i13.sroa.26.07990 = phi float [ %history.i.i13.sroa.26.0.copyload, %bb5.i.i212.lr.ph ], [ %history.i.i13.sroa.22.07989, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ]
  %history.i.i13.sroa.22.07989 = phi float [ %history.i.i13.sroa.22.0.copyload, %bb5.i.i212.lr.ph ], [ %history.i.i13.sroa.19.07988, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ]
  %history.i.i13.sroa.19.07988 = phi float [ %history.i.i13.sroa.19.0.copyload, %bb5.i.i212.lr.ph ], [ %history.i.i13.sroa.16.07987, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ]
  %history.i.i13.sroa.16.07987 = phi float [ %history.i.i13.sroa.16.0.copyload, %bb5.i.i212.lr.ph ], [ %history.i.i13.sroa.13.07986, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ]
  %history.i.i13.sroa.13.07986 = phi float [ %history.i.i13.sroa.13.0.copyload, %bb5.i.i212.lr.ph ], [ %history.i.i13.sroa.10.07985, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ]
  %history.i.i13.sroa.10.07985 = phi float [ %history.i.i13.sroa.10.0.copyload, %bb5.i.i212.lr.ph ], [ %history.i.i13.sroa.7.07984, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ]
  %history.i.i13.sroa.7.07984 = phi float [ %history.i.i13.sroa.7.0.copyload, %bb5.i.i212.lr.ph ], [ %history.i.i13.sroa.0.07983, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ]
  %history.i.i13.sroa.0.07983 = phi float [ %history.i.i13.sroa.0.0.copyload, %bb5.i.i212.lr.ph ], [ %_0.i3011, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ]
  %607 = add nuw nsw i32 %iter.sroa.0.0.i.i627994, 1, !dbg !21802
  %_11.i31.i = add nuw nsw i32 %iter.sroa.0.0.i.i627994, %iter1.sroa.0.0.i568747, !dbg !21805
  %_24.i.i213 = icmp ugt i32 %_11.i31.i, %right_io.1, !dbg !21806
  br i1 %_24.i.i213, label %bb7.i.i410, label %bb8.i.i214, !dbg !21806, !prof !902

bb8.i.i214:                                       ; preds = %bb5.i.i212
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21809), !dbg !21812
  %_3.not.i3009 = icmp eq i32 %right_io.1, %_11.i31.i, !dbg !21813
  br i1 %_3.not.i3009, label %panic.i3012, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013, !dbg !21813

panic.i3012:                                      ; preds = %bb8.i.i214
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #33, !dbg !21813, !noalias !21815
  unreachable, !dbg !21813

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013: ; preds = %bb8.i.i214
  %_31.i.i216 = getelementptr inbounds nuw float, ptr %right_io.0, i32 %_11.i31.i, !dbg !21817
  %_0.i3011 = load float, ptr %_31.i.i216, align 4, !dbg !21813, !alias.scope !21809, !noalias !21819, !noundef !10
  %608 = tail call noundef float @llvm.fabs.f32(float %history.i.i13.sroa.19.07988), !dbg !21820
  %_0.i2714 = fmul float %_0.i3011, %_11.i.i.i.i231, !dbg !21823
  %_0.i2282 = fadd float %_0.i2714, 0.000000e+00, !dbg !21826
  %_0.i2713 = fmul float %_0.i3011, %_14.i.i.i.i234, !dbg !21828
  %_0.i2281 = fadd float %_0.i2713, 0.000000e+00, !dbg !21830
  %_0.i2712 = fmul float %_0.i3011, %_17.i.i.i.i237, !dbg !21832
  %_0.i2280 = fadd float %_0.i2712, 0.000000e+00, !dbg !21834
  %_0.i2711 = fmul float %_0.i3011, %_20.i.i.i.i240, !dbg !21836
  %_0.i2279 = fadd float %_0.i2711, 0.000000e+00, !dbg !21838
  %_0.i2710 = fmul float %history.i.i13.sroa.0.07983, %_25.i.i.i.i245, !dbg !21840
  %_0.i2278 = fadd float %_0.i2282, %_0.i2710, !dbg !21842
  %_0.i2709 = fmul float %history.i.i13.sroa.0.07983, %_28.i.i.i.i248, !dbg !21844
  %_0.i2277 = fadd float %_0.i2281, %_0.i2709, !dbg !21846
  %_0.i2708 = fmul float %history.i.i13.sroa.0.07983, %_31.i.i.i.i251, !dbg !21848
  %_0.i2276 = fadd float %_0.i2280, %_0.i2708, !dbg !21850
  %_0.i2707 = fmul float %history.i.i13.sroa.0.07983, %_34.i.i.i.i254, !dbg !21852
  %_0.i2275 = fadd float %_0.i2279, %_0.i2707, !dbg !21854
  %_0.i2706 = fmul float %history.i.i13.sroa.7.07984, %_39.i.i.i.i259, !dbg !21856
  %_0.i2274 = fadd float %_0.i2278, %_0.i2706, !dbg !21858
  %_0.i2705 = fmul float %history.i.i13.sroa.7.07984, %_42.i.i.i.i262, !dbg !21860
  %_0.i2273 = fadd float %_0.i2277, %_0.i2705, !dbg !21862
  %_0.i2704 = fmul float %history.i.i13.sroa.7.07984, %_45.i.i.i.i265, !dbg !21864
  %_0.i2272 = fadd float %_0.i2276, %_0.i2704, !dbg !21866
  %_0.i2703 = fmul float %history.i.i13.sroa.7.07984, %_48.i.i.i.i268, !dbg !21868
  %_0.i2271 = fadd float %_0.i2275, %_0.i2703, !dbg !21870
  %_0.i2702 = fmul float %history.i.i13.sroa.10.07985, %_53.i.i.i.i273, !dbg !21872
  %_0.i2270 = fadd float %_0.i2274, %_0.i2702, !dbg !21874
  %_0.i2701 = fmul float %history.i.i13.sroa.10.07985, %_56.i.i.i.i276, !dbg !21876
  %_0.i2269 = fadd float %_0.i2273, %_0.i2701, !dbg !21878
  %_0.i2700 = fmul float %history.i.i13.sroa.10.07985, %_59.i.i.i.i279, !dbg !21880
  %_0.i2268 = fadd float %_0.i2272, %_0.i2700, !dbg !21882
  %_0.i2699 = fmul float %history.i.i13.sroa.10.07985, %_62.i.i.i.i282, !dbg !21884
  %_0.i2267 = fadd float %_0.i2271, %_0.i2699, !dbg !21886
  %_0.i2698 = fmul float %history.i.i13.sroa.13.07986, %_67.i.i.i.i287, !dbg !21888
  %_0.i2266 = fadd float %_0.i2270, %_0.i2698, !dbg !21890
  %_0.i2697 = fmul float %history.i.i13.sroa.13.07986, %_70.i.i.i.i290, !dbg !21892
  %_0.i2265 = fadd float %_0.i2269, %_0.i2697, !dbg !21894
  %_0.i2696 = fmul float %history.i.i13.sroa.13.07986, %_73.i.i.i.i293, !dbg !21896
  %_0.i2264 = fadd float %_0.i2268, %_0.i2696, !dbg !21898
  %_0.i2695 = fmul float %history.i.i13.sroa.13.07986, %_76.i.i.i.i296, !dbg !21900
  %_0.i2263 = fadd float %_0.i2267, %_0.i2695, !dbg !21902
  %_0.i2694 = fmul float %history.i.i13.sroa.16.07987, %_81.i.i.i.i301, !dbg !21904
  %_0.i2262 = fadd float %_0.i2266, %_0.i2694, !dbg !21906
  %_0.i2693 = fmul float %history.i.i13.sroa.16.07987, %_84.i.i.i.i304, !dbg !21908
  %_0.i2261 = fadd float %_0.i2265, %_0.i2693, !dbg !21910
  %_0.i2692 = fmul float %history.i.i13.sroa.16.07987, %_87.i.i.i.i307, !dbg !21912
  %_0.i2260 = fadd float %_0.i2264, %_0.i2692, !dbg !21914
  %_0.i2691 = fmul float %history.i.i13.sroa.16.07987, %_90.i.i.i.i310, !dbg !21916
  %_0.i2259 = fadd float %_0.i2263, %_0.i2691, !dbg !21918
  %_0.i2690 = fmul float %history.i.i13.sroa.19.07988, %_95.i.i.i.i315, !dbg !21920
  %_0.i2258 = fadd float %_0.i2262, %_0.i2690, !dbg !21922
  %_0.i2689 = fmul float %history.i.i13.sroa.19.07988, %_98.i.i.i.i318, !dbg !21924
  %_0.i2257 = fadd float %_0.i2261, %_0.i2689, !dbg !21926
  %_0.i2688 = fmul float %history.i.i13.sroa.19.07988, %_101.i.i.i.i321, !dbg !21928
  %_0.i2256 = fadd float %_0.i2260, %_0.i2688, !dbg !21930
  %_0.i2687 = fmul float %history.i.i13.sroa.19.07988, %_104.i.i.i.i324, !dbg !21932
  %_0.i2255 = fadd float %_0.i2259, %_0.i2687, !dbg !21934
  %_0.i2686 = fmul float %history.i.i13.sroa.22.07989, %_109.i.i.i.i329, !dbg !21936
  %_0.i2254 = fadd float %_0.i2258, %_0.i2686, !dbg !21938
  %_0.i2685 = fmul float %history.i.i13.sroa.22.07989, %_112.i.i.i.i332, !dbg !21940
  %_0.i2253 = fadd float %_0.i2257, %_0.i2685, !dbg !21942
  %_0.i2684 = fmul float %history.i.i13.sroa.22.07989, %_115.i.i.i.i335, !dbg !21944
  %_0.i2252 = fadd float %_0.i2256, %_0.i2684, !dbg !21946
  %_0.i2683 = fmul float %history.i.i13.sroa.22.07989, %_118.i.i.i.i338, !dbg !21948
  %_0.i2251 = fadd float %_0.i2255, %_0.i2683, !dbg !21950
  %_0.i2682 = fmul float %history.i.i13.sroa.26.07990, %_123.i.i.i.i343, !dbg !21952
  %_0.i2250 = fadd float %_0.i2254, %_0.i2682, !dbg !21954
  %_0.i2681 = fmul float %history.i.i13.sroa.26.07990, %_126.i.i.i.i346, !dbg !21956
  %_0.i2249 = fadd float %_0.i2253, %_0.i2681, !dbg !21958
  %_0.i2680 = fmul float %history.i.i13.sroa.26.07990, %_129.i.i.i.i349, !dbg !21960
  %_0.i2248 = fadd float %_0.i2252, %_0.i2680, !dbg !21962
  %_0.i2679 = fmul float %history.i.i13.sroa.26.07990, %_132.i.i.i.i352, !dbg !21964
  %_0.i2247 = fadd float %_0.i2251, %_0.i2679, !dbg !21966
  %_0.i2678 = fmul float %history.i.i13.sroa.29.07991, %_137.i.i.i.i357, !dbg !21968
  %_0.i2246 = fadd float %_0.i2250, %_0.i2678, !dbg !21970
  %_0.i2677 = fmul float %history.i.i13.sroa.29.07991, %_140.i.i.i.i360, !dbg !21972
  %_0.i2245 = fadd float %_0.i2249, %_0.i2677, !dbg !21974
  %_0.i2676 = fmul float %history.i.i13.sroa.29.07991, %_143.i.i.i.i363, !dbg !21976
  %_0.i2244 = fadd float %_0.i2248, %_0.i2676, !dbg !21978
  %_0.i2675 = fmul float %history.i.i13.sroa.29.07991, %_146.i.i.i.i366, !dbg !21980
  %_0.i2243 = fadd float %_0.i2247, %_0.i2675, !dbg !21982
  %_0.i2674 = fmul float %history.i.i13.sroa.32.07992, %_151.i.i.i.i371, !dbg !21984
  %_0.i2242 = fadd float %_0.i2246, %_0.i2674, !dbg !21986
  %_0.i2673 = fmul float %history.i.i13.sroa.32.07992, %_154.i.i.i.i374, !dbg !21988
  %_0.i2241 = fadd float %_0.i2245, %_0.i2673, !dbg !21990
  %_0.i2672 = fmul float %history.i.i13.sroa.32.07992, %_157.i.i.i.i377, !dbg !21992
  %_0.i2240 = fadd float %_0.i2244, %_0.i2672, !dbg !21994
  %_0.i2671 = fmul float %history.i.i13.sroa.32.07992, %_160.i.i.i.i380, !dbg !21996
  %_0.i2239 = fadd float %_0.i2243, %_0.i2671, !dbg !21998
  %_0.i2670 = fmul float %history.i.i13.sroa.35.07993, %_165.i.i.i.i385, !dbg !22000
  %_0.i2238 = fadd float %_0.i2242, %_0.i2670, !dbg !22002
  %_0.i2669 = fmul float %history.i.i13.sroa.35.07993, %_168.i.i.i.i388, !dbg !22004
  %_0.i2237 = fadd float %_0.i2241, %_0.i2669, !dbg !22006
  %_0.i2668 = fmul float %history.i.i13.sroa.35.07993, %_171.i.i.i.i391, !dbg !22008
  %_0.i2236 = fadd float %_0.i2240, %_0.i2668, !dbg !22010
  %_0.i2667 = fmul float %history.i.i13.sroa.35.07993, %_174.i.i.i.i394, !dbg !22012
  %_0.i2235 = fadd float %_0.i2239, %_0.i2667, !dbg !22014
  %609 = tail call noundef float @llvm.fabs.f32(float %_0.i2238), !dbg !22016
  %_3.i.i3647.inv = fcmp ogt float %608, %609, !dbg !22018
  %_4.i.i3654.v = select i1 %_3.i.i3647.inv, float %608, float %609, !dbg !22018
  %610 = tail call noundef float @llvm.fabs.f32(float %_0.i2237), !dbg !22016
  %_3.i.i3647.inv.1 = fcmp ogt float %_4.i.i3654.v, %610, !dbg !22018
  %_4.i.i3654.v.1 = select i1 %_3.i.i3647.inv.1, float %_4.i.i3654.v, float %610, !dbg !22018
  %611 = tail call noundef float @llvm.fabs.f32(float %_0.i2236), !dbg !22016
  %_3.i.i3647.inv.2 = fcmp ogt float %_4.i.i3654.v.1, %611, !dbg !22018
  %_4.i.i3654.v.2 = select i1 %_3.i.i3647.inv.2, float %_4.i.i3654.v.1, float %611, !dbg !22018
  %612 = tail call noundef float @llvm.fabs.f32(float %_0.i2235), !dbg !22016
  %_3.i.i3647.inv.3 = fcmp ogt float %_4.i.i3654.v.2, %612, !dbg !22018
  %_4.i.i3654.v.3 = select i1 %_3.i.i3647.inv.3, float %_4.i.i3654.v.2, float %612, !dbg !22018
  %_39.i.i405 = getelementptr inbounds nuw float, ptr %peaks_right.i33, i32 %iter.sroa.0.0.i.i627994, !dbg !22021
  store float %_4.i.i3654.v.3, ptr %_39.i.i405, align 4, !dbg !22026, !alias.scope !22028, !noalias !21819
  %exitcond11941.not = icmp eq i32 %607, %umax11940, !dbg !22031
  br i1 %exitcond11941.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i64, label %bb5.i.i212, !dbg !21799

bb7.i.i410:                                       ; preds = %bb5.i.i212
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_11.i31.i, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #33, !dbg !22033, !noalias !21819
  unreachable, !dbg !22033

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i64: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i
  %history.i.i13.sroa.0.0.lcssa = phi float [ %history.i.i13.sroa.0.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i ], [ %_0.i3011, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ], !dbg !22034
  %history.i.i13.sroa.7.0.lcssa = phi float [ %history.i.i13.sroa.7.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i ], [ %history.i.i13.sroa.0.07983, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ], !dbg !22034
  %history.i.i13.sroa.10.0.lcssa = phi float [ %history.i.i13.sroa.10.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i ], [ %history.i.i13.sroa.7.07984, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ], !dbg !22034
  %history.i.i13.sroa.13.0.lcssa = phi float [ %history.i.i13.sroa.13.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i ], [ %history.i.i13.sroa.10.07985, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ], !dbg !22034
  %history.i.i13.sroa.16.0.lcssa = phi float [ %history.i.i13.sroa.16.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i ], [ %history.i.i13.sroa.13.07986, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ], !dbg !22034
  %history.i.i13.sroa.19.0.lcssa = phi float [ %history.i.i13.sroa.19.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i ], [ %history.i.i13.sroa.16.07987, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ], !dbg !22034
  %history.i.i13.sroa.22.0.lcssa = phi float [ %history.i.i13.sroa.22.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i ], [ %history.i.i13.sroa.19.07988, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ], !dbg !22034
  %history.i.i13.sroa.26.0.lcssa = phi float [ %history.i.i13.sroa.26.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i ], [ %history.i.i13.sroa.22.07989, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ], !dbg !22034
  %history.i.i13.sroa.29.0.lcssa = phi float [ %history.i.i13.sroa.29.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i ], [ %history.i.i13.sroa.26.07990, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ], !dbg !22034
  %history.i.i13.sroa.32.0.lcssa = phi float [ %history.i.i13.sroa.32.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i ], [ %history.i.i13.sroa.29.07991, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ], !dbg !22034
  %history.i.i13.sroa.35.0.lcssa = phi float [ %history.i.i13.sroa.35.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i ], [ %history.i.i13.sroa.32.07992, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ], !dbg !22034
  %history.i.i13.sroa.38.0.lcssa = phi float [ %history.i.i13.sroa.38.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i ], [ %history.i.i13.sroa.35.07993, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ], !dbg !22034
  store float %history.i.i13.sroa.0.0.lcssa, ptr %hot_right.i35, align 4, !dbg !22035, !noalias !21794
  store float %history.i.i13.sroa.7.0.lcssa, ptr %history.i.i13.sroa.7.0.hot_right.i35.sroa_idx, align 4, !dbg !22035, !noalias !21794
  store float %history.i.i13.sroa.10.0.lcssa, ptr %history.i.i13.sroa.10.0.hot_right.i35.sroa_idx, align 4, !dbg !22035, !noalias !21794
  store float %history.i.i13.sroa.13.0.lcssa, ptr %history.i.i13.sroa.13.0.hot_right.i35.sroa_idx, align 4, !dbg !22035, !noalias !21794
  store float %history.i.i13.sroa.16.0.lcssa, ptr %history.i.i13.sroa.16.0.hot_right.i35.sroa_idx, align 4, !dbg !22035, !noalias !21794
  store float %history.i.i13.sroa.19.0.lcssa, ptr %history.i.i13.sroa.19.0.hot_right.i35.sroa_idx, align 4, !dbg !22035, !noalias !21794
  store float %history.i.i13.sroa.22.0.lcssa, ptr %history.i.i13.sroa.22.0.hot_right.i35.sroa_idx, align 4, !dbg !22035, !noalias !21794
  store float %history.i.i13.sroa.26.0.lcssa, ptr %history.i.i13.sroa.26.0.hot_right.i35.sroa_idx, align 4, !dbg !22035, !noalias !21794
  store float %history.i.i13.sroa.29.0.lcssa, ptr %history.i.i13.sroa.29.0.hot_right.i35.sroa_idx, align 4, !dbg !22035, !noalias !21794
  store float %history.i.i13.sroa.32.0.lcssa, ptr %history.i.i13.sroa.32.0.hot_right.i35.sroa_idx, align 4, !dbg !22035, !noalias !21794
  store float %history.i.i13.sroa.35.0.lcssa, ptr %history.i.i13.sroa.35.0.hot_right.i35.sroa_idx, align 4, !dbg !22035, !noalias !21794
  store float %history.i.i13.sroa.38.0.lcssa, ptr %history.i.i13.sroa.38.0.hot_right.i35.sroa_idx, align 4, !dbg !22035, !noalias !21794
  br i1 %_20.i43.i7955.not, label %bb15.i53.loopexit, label %bb20.i70.lr.ph, !dbg !21531

bb20.i70.lr.ph:                                   ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i64
  %_68.i29.sroa.3.0.copyload = load i32, ptr %_68.i29.sroa.3.0..sroa_idx, align 4, !noalias !21475
  %_68.i29.sroa.4.0.copyload = load i32, ptr %_68.i29.sroa.4.0..sroa_idx, align 4, !noalias !21475
  %_69.i28.sroa.3.0.copyload = load i32, ptr %_69.i28.sroa.3.0..sroa_idx, align 4, !noalias !21475
  %_69.i28.sroa.4.0.copyload = load i32, ptr %_69.i28.sroa.4.0..sroa_idx, align 4, !noalias !21475
  %_54.0.i257.i = load ptr, ptr %uniform_left.i32, align 4, !nonnull !10, !align !10189
  %_54.1.i258.i = load i32, ptr %582, align 4
  %_18.i268.i = load i32, ptr %568, align 4
  %_29.i11928007.not = icmp eq i32 %_18.i268.i, 0
  %_56.0.i279.i = load ptr, ptr %583, align 4, !nonnull !10, !align !10189
  %_56.1.i280.i = load i32, ptr %584, align 4
  %_58.1.i305.i = load i32, ptr %588, align 4
  %_58.0.i304.i = load ptr, ptr %589, align 4, !nonnull !10, !align !10189
  %_54.0.i.i147 = load ptr, ptr %uniform_right.i31, align 4, !nonnull !10, !align !10189
  %_54.1.i.i148 = load i32, ptr %590, align 4
  %_18.i.i158 = load i32, ptr %569, align 4
  %_29.i11708011.not = icmp eq i32 %_18.i.i158, 0
  %_56.0.i.i165 = load ptr, ptr %591, align 4, !nonnull !10, !align !10189
  %_56.1.i.i166 = load i32, ptr %592, align 4
  %_58.1.i.i189 = load i32, ptr %596, align 4
  %_58.0.i.i188 = load ptr, ptr %597, align 4, !nonnull !10, !align !10189
  %_22.i272.i.promoted8600 = load i32, ptr %_22.i272.i, align 4
  %_21.i271.i.promoted8636 = load float, ptr %_21.i271.i, align 4
  %_22.i.i160.promoted8672 = load i32, ptr %_22.i.i160, align 4
  %_21.i.i159.promoted8708 = load float, ptr %_21.i.i159, align 4
  %_13.i193345314533 = load float, ptr %572, align 4
  %_13.i192145344536 = load float, ptr %575, align 4
  %_13.i190945374539 = load float, ptr %578, align 4
  %_13.i189745404542 = load float, ptr %581, align 4
  %_37.i292.i = load float, ptr %586, align 4
  %_37.i.i176 = load float, ptr %594, align 4
  %.promoted14050 = load float, ptr %570, align 4
  %_110.i111.promoted14068 = load float, ptr %_110.i111, align 4
  %.promoted14086 = load float, ptr %571, align 4
  %.promoted14104 = load float, ptr %573, align 4
  %_111.i112.promoted14122 = load float, ptr %_111.i112, align 4
  %.promoted14140 = load float, ptr %574, align 4
  %.promoted14158 = load float, ptr %576, align 4
  %_115.i113.promoted14176 = load float, ptr %_115.i113, align 4
  %.promoted14194 = load float, ptr %577, align 4
  %.promoted14212 = load float, ptr %579, align 4
  %_116.i114.promoted14230 = load float, ptr %_116.i114, align 4
  %.promoted14248 = load float, ptr %580, align 4
  %.promoted14266 = load float, ptr %585, align 4
  %.promoted14284 = load float, ptr %587, align 4
  %.promoted14302 = load float, ptr %593, align 4
  %.promoted14320 = load float, ptr %595, align 4
  br label %bb20.i70, !dbg !21531

bb20.i70:                                         ; preds = %bb20.i70.lr.ph, %bb67.i203
  %_0.i3224.lcssa1275714322 = phi float [ %.promoted14320, %bb20.i70.lr.ph ], [ %_0.i3224.lcssa1275714321, %bb67.i203 ]
  %_0.i2850.lcssa1275614304 = phi float [ %.promoted14302, %bb20.i70.lr.ph ], [ %_0.i2850.lcssa1275614303, %bb67.i203 ]
  %_0.i3228.lcssa1273214286 = phi float [ %.promoted14284, %bb20.i70.lr.ph ], [ %_0.i3228.lcssa1273214285, %bb67.i203 ]
  %_0.i2854.lcssa1271614268 = phi float [ %.promoted14266, %bb20.i70.lr.ph ], [ %_0.i2854.lcssa1271614267, %bb67.i203 ]
  %_0.i3279.lcssa1253114250 = phi float [ %.promoted14248, %bb20.i70.lr.ph ], [ %_0.i3279.lcssa1253114249, %bb67.i203 ]
  %_0.i3286.lcssa1254514232 = phi float [ %_116.i114.promoted14230, %bb20.i70.lr.ph ], [ %_0.i3286.lcssa1254514231, %bb67.i203 ]
  %_0.i.i3520.lcssa1255914214 = phi float [ %.promoted14212, %bb20.i70.lr.ph ], [ %_0.i.i3520.lcssa1255914213, %bb67.i203 ]
  %_0.i3266.lcssa1257314196 = phi float [ %.promoted14194, %bb20.i70.lr.ph ], [ %_0.i3266.lcssa1257314195, %bb67.i203 ]
  %_0.i3273.lcssa1258714178 = phi float [ %_115.i113.promoted14176, %bb20.i70.lr.ph ], [ %_0.i3273.lcssa1258714177, %bb67.i203 ]
  %_0.i.i3513.lcssa1260114160 = phi float [ %.promoted14158, %bb20.i70.lr.ph ], [ %_0.i.i3513.lcssa1260114159, %bb67.i203 ]
  %_0.i3253.lcssa1261514142 = phi float [ %.promoted14140, %bb20.i70.lr.ph ], [ %_0.i3253.lcssa1261514141, %bb67.i203 ]
  %_0.i3260.lcssa1262914124 = phi float [ %_111.i112.promoted14122, %bb20.i70.lr.ph ], [ %_0.i3260.lcssa1262914123, %bb67.i203 ]
  %_0.i.i3506.lcssa1264314106 = phi float [ %.promoted14104, %bb20.i70.lr.ph ], [ %_0.i.i3506.lcssa1264314105, %bb67.i203 ]
  %_0.i3241.lcssa1265714088 = phi float [ %.promoted14086, %bb20.i70.lr.ph ], [ %_0.i3241.lcssa1265714087, %bb67.i203 ]
  %_0.i3247.lcssa1267114070 = phi float [ %_110.i111.promoted14068, %bb20.i70.lr.ph ], [ %_0.i3247.lcssa1267114069, %bb67.i203 ]
  %_0.i.i.lcssa1268514052 = phi float [ %.promoted14050, %bb20.i70.lr.ph ], [ %_0.i.i.lcssa1268514051, %bb67.i203 ]
  %running.sroa.0.0.i.lcssa85318710 = phi float [ %_21.i.i159.promoted8708, %bb20.i70.lr.ph ], [ %running.sroa.0.0.i.lcssa85318709, %bb67.i203 ]
  %storemerge.i.lcssa85048674 = phi i32 [ %_22.i.i160.promoted8672, %bb20.i70.lr.ph ], [ %storemerge.i.lcssa85048673, %bb67.i203 ]
  %running.sroa.0.0.i1180.lcssa84198638 = phi float [ %_21.i271.i.promoted8636, %bb20.i70.lr.ph ], [ %running.sroa.0.0.i1180.lcssa84198637, %bb67.i203 ]
  %storemerge.i1185.lcssa83928602 = phi i32 [ %_22.i272.i.promoted8600, %bb20.i70.lr.ph ], [ %storemerge.i1185.lcssa83928601, %bb67.i203 ]
  %frame.sroa.0.0.i688597 = phi i32 [ 0, %bb20.i70.lr.ph ], [ %_83.i83, %bb67.i203 ]
  %main_cursor.sroa.0.1.i678596 = phi i32 [ %main_cursor.sroa.0.0.i558746, %bb20.i70.lr.ph ], [ %main_cursor.sroa.0.2.i209, %bb67.i203 ]
  %ring_cursor.sroa.0.1.i668595 = phi i32 [ %ring_cursor.sroa.0.0.i548745, %bb20.i70.lr.ph ], [ %ring_cursor.sroa.0.2.i206, %bb67.i203 ]
  %_65.i71 = sub nuw nsw i32 %spec.store.select.i60, %frame.sroa.0.0.i688597, !dbg !22036
  %ring.i1795 = load i32, ptr %528, align 4, !dbg !22037, !alias.scope !22039, !noalias !22042, !noundef !10
  %main.i1796 = load i32, ptr %529, align 4, !dbg !22046, !alias.scope !22039, !noalias !22042, !noundef !10
  %_10.i = add i32 %ring_cursor.sroa.0.1.i668595, 1, !dbg !22047
  %_38.not.i = icmp ult i32 %_10.i, %ring.i1795, !dbg !22048
  %613 = select i1 %_38.not.i, i32 0, i32 %ring.i1795, !dbg !22048
  %start1.sroa.0.0.i1797 = sub nuw i32 %_10.i, %613, !dbg !22048
  %_12.i1798 = add i32 %_68.i29.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i668595, !dbg !22050
  %_39.not.i = icmp ult i32 %_12.i1798, %ring.i1795, !dbg !22051
  %614 = select i1 %_39.not.i, i32 0, i32 %ring.i1795, !dbg !22051
  %left_end.sroa.0.0.i = sub nuw i32 %_12.i1798, %614, !dbg !22051
  %_15.i1799 = add i32 %_69.i28.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i668595, !dbg !22053
  %_40.not.i = icmp ult i32 %_15.i1799, %ring.i1795, !dbg !22054
  %615 = select i1 %_40.not.i, i32 0, i32 %ring.i1795, !dbg !22054
  %right_end.sroa.0.0.i = sub nuw i32 %_15.i1799, %615, !dbg !22054
  %_18.i = add i32 %_68.i29.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i668595, !dbg !22056
  %_41.not.i = icmp ult i32 %_18.i, %ring.i1795, !dbg !22057
  %616 = select i1 %_41.not.i, i32 0, i32 %ring.i1795, !dbg !22057
  %left_expiring.sroa.0.0.i = sub nuw i32 %_18.i, %616, !dbg !22057
  %_21.i = add i32 %_69.i28.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i668595, !dbg !22059
  %_42.not.i = icmp ult i32 %_21.i, %ring.i1795, !dbg !22060
  %617 = select i1 %_42.not.i, i32 0, i32 %ring.i1795, !dbg !22060
  %right_expiring.sroa.0.0.i = sub nuw i32 %_21.i, %617, !dbg !22060
  %618 = sub i32 %ring.i1795, %ring_cursor.sroa.0.1.i668595, !dbg !22062
  %spec.store.select.i1800 = tail call i32 @llvm.umin.i32(i32 %618, i32 %_65.i71), !dbg !22063
  %619 = sub i32 %main.i1796, %main_cursor.sroa.0.1.i678596, !dbg !22065
  %_24.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %619, i32 %spec.store.select.i1800), !dbg !22066
  %620 = sub i32 %ring.i1795, %start1.sroa.0.0.i1797, !dbg !22068
  %_25.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %620, i32 %_24.sroa.0.0.i), !dbg !22069
  %621 = sub i32 %ring.i1795, %left_end.sroa.0.0.i, !dbg !22071
  %_27.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %621, i32 %_25.sroa.0.0.i), !dbg !22072
  %622 = sub i32 %ring.i1795, %right_end.sroa.0.0.i, !dbg !22074
  %_29.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %622, i32 %_27.sroa.0.0.i), !dbg !22075
  %623 = sub i32 %ring.i1795, %left_expiring.sroa.0.0.i, !dbg !22077
  %_31.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %623, i32 %_29.sroa.0.0.i), !dbg !22078
  %624 = sub i32 %ring.i1795, %right_expiring.sroa.0.0.i, !dbg !22080
  %run.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %624, i32 %_31.sroa.0.0.i), !dbg !22081
  %_72.i73 = add i32 %frame.sroa.0.0.i688597, %iter1.sroa.0.0.i568747, !dbg !22083
  %_76.i74 = add i32 %run.sroa.0.0.i, %_72.i73, !dbg !22086
  %_172.i75 = icmp ult i32 %_76.i74, %_72.i73, !dbg !22089
  %_166.not.i76 = icmp ugt i32 %_76.i74, %left_io.1
  %or.cond.i77 = or i1 %_172.i75, %_166.not.i76, !dbg !22089
  br i1 %or.cond.i77, label %bb51.i211, label %bb49.i78, !dbg !22089, !prof !4694

bb51.i211:                                        ; preds = %bb20.i70
  store float %_0.i.i.lcssa1268514052, ptr %570, align 4
  store float %_0.i3247.lcssa1267114070, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265714088, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264314106, ptr %573, align 4
  store float %_0.i3260.lcssa1262914124, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261514142, ptr %574, align 4
  store float %_0.i.i3513.lcssa1260114160, ptr %576, align 4
  store float %_0.i3273.lcssa1258714178, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257314196, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255914214, ptr %579, align 4
  store float %_0.i3286.lcssa1254514232, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1253114250, ptr %580, align 4
  store float %_0.i2854.lcssa1271614268, ptr %585, align 4
  store float %_0.i3228.lcssa1273214286, ptr %587, align 4
  store float %_0.i2850.lcssa1275614304, ptr %593, align 4
  store float %_0.i3224.lcssa1275714322, ptr %595, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_72.i73, i32 noundef %_76.i74, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_bd3c2aadcc2cdd6816f02be0ceab2cc1) #33, !dbg !22096, !noalias !21469
  unreachable, !dbg !22096

bb49.i78:                                         ; preds = %bb20.i70
  %_175.i79 = getelementptr inbounds nuw float, ptr %left_io.0, i32 %_72.i73, !dbg !22097
  %_176.not.i80 = icmp ugt i32 %_76.i74, %right_io.1, !dbg !22101
  br i1 %_176.not.i80, label %bb54.i210, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit, !dbg !22101, !prof !902

bb54.i210:                                        ; preds = %bb49.i78
  store float %_0.i.i.lcssa1268514052, ptr %570, align 4
  store float %_0.i3247.lcssa1267114070, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265714088, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264314106, ptr %573, align 4
  store float %_0.i3260.lcssa1262914124, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261514142, ptr %574, align 4
  store float %_0.i.i3513.lcssa1260114160, ptr %576, align 4
  store float %_0.i3273.lcssa1258714178, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257314196, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255914214, ptr %579, align 4
  store float %_0.i3286.lcssa1254514232, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1253114250, ptr %580, align 4
  store float %_0.i2854.lcssa1271614268, ptr %585, align 4
  store float %_0.i3228.lcssa1273214286, ptr %587, align 4
  store float %_0.i2850.lcssa1275614304, ptr %593, align 4
  store float %_0.i3224.lcssa1275714322, ptr %595, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_72.i73, i32 noundef %_76.i74, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c1156ffa98f252967759477badafe965) #33, !dbg !22106, !noalias !21469
  unreachable, !dbg !22106

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit: ; preds = %bb49.i78
  %_183.i82 = getelementptr inbounds nuw float, ptr %right_io.0, i32 %_72.i73, !dbg !22107
  %_83.i83 = add nuw nsw i32 %run.sroa.0.0.i, %frame.sroa.0.0.i688597, !dbg !22111
  %_192.i89 = getelementptr inbounds nuw float, ptr %peaks_left.i34, i32 %frame.sroa.0.0.i688597, !dbg !22113
  %_201.i90 = getelementptr inbounds nuw float, ptr %peaks_right.i33, i32 %frame.sroa.0.0.i688597, !dbg !22123
  %_2.i8015.not = icmp eq i32 %run.sroa.0.0.i, 0, !dbg !22133
  br i1 %_2.i8015.not, label %bb67.i203, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph, !dbg !22133

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph: ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit
  %umax11944 = call i32 @llvm.umax.i32(i32 %ring_cursor.sroa.0.1.i668595, i32 %_54.1.i258.i), !dbg !22133
  %umax11945 = call i32 @llvm.umax.i32(i32 %main_cursor.sroa.0.1.i678596, i32 %_58.1.i305.i), !dbg !22133
  %625 = sub i32 %umax11944, %ring_cursor.sroa.0.1.i668595, !dbg !22133
  %626 = sub i32 %umax11945, %main_cursor.sroa.0.1.i678596, !dbg !22133
  %umin11948 = call i32 @llvm.umin.i32(i32 %621, i32 %622), !dbg !22133
  %umin11949 = call i32 @llvm.umin.i32(i32 %umin11948, i32 %623), !dbg !22133
  %umin11950 = call i32 @llvm.umin.i32(i32 %umin11949, i32 %624), !dbg !22133
  %umin11951 = call i32 @llvm.umin.i32(i32 %umin11950, i32 %620), !dbg !22133
  %umin11952 = call i32 @llvm.umin.i32(i32 %umin11951, i32 %618), !dbg !22133
  %umin11953 = call i32 @llvm.umin.i32(i32 %umin11952, i32 %619), !dbg !22133
  %627 = sub nsw i32 %umin11954, %frame.sroa.0.0.i688597, !dbg !22133
  %umin11955 = call i32 @llvm.umin.i32(i32 %umin11953, i32 %627), !dbg !22133
  br label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018, !dbg !22133

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165
  %_0.i32248566 = phi float [ %_0.i3224.lcssa1275714322, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i3224, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i28508537 = phi float [ %_0.i2850.lcssa1275614304, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i2850, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %running.sroa.0.0.i8509 = phi float [ %running.sroa.0.0.i.lcssa85318710, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %running.sroa.0.0.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %storemerge.i8482 = phi i32 [ %storemerge.i.lcssa85048674, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %storemerge.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i32288454 = phi float [ %_0.i3228.lcssa1273214286, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i3228, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i28548425 = phi float [ %_0.i2854.lcssa1271614268, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i2854, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %running.sroa.0.0.i11808397 = phi float [ %running.sroa.0.0.i1180.lcssa84198638, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %running.sroa.0.0.i1180, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %storemerge.i11858370 = phi i32 [ %storemerge.i1185.lcssa83928602, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %storemerge.i1185, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_12.i18958341 = phi float [ %_0.i3279.lcssa1253114250, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i3279, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ], !dbg !22142
  %_0.i32868312 = phi float [ %_0.i3286.lcssa1254514232, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i3286, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ], !dbg !22142
  %_5.i18898283 = phi float [ %_0.i.i3520.lcssa1255914214, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i.i3520, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ], !dbg !22142
  %_12.i19078253 = phi float [ %_0.i3266.lcssa1257314196, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i3266, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ], !dbg !22142
  %_0.i32738224 = phi float [ %_0.i3273.lcssa1258714178, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i3273, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ], !dbg !22142
  %_5.i19018195 = phi float [ %_0.i.i3513.lcssa1260114160, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i.i3513, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ], !dbg !22142
  %_12.i19198165 = phi float [ %_0.i3253.lcssa1261514142, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i3253, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ], !dbg !22142
  %_0.i32608136 = phi float [ %_0.i3260.lcssa1262914124, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i3260, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ], !dbg !22142
  %_5.i19138107 = phi float [ %_0.i.i3506.lcssa1264314106, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i.i3506, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ], !dbg !22142
  %_12.i19318077 = phi float [ %_0.i3241.lcssa1265714088, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i3241, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ], !dbg !22142
  %_0.i32478048 = phi float [ %_0.i3247.lcssa1267114070, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i3247, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ], !dbg !22142
  %_5.i19258019 = phi float [ %_0.i.i.lcssa1268514052, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ], !dbg !22142
  %iter.i19.sroa.41.08017 = phi i32 [ 0, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_206.0.i110, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_206.0.i110 = add nuw i32 %iter.i19.sroa.41.08017, 1, !dbg !22143
  %data.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_175.i79, i32 %iter.i19.sroa.41.08017, !dbg !22146
  %data.i5.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_183.i82, i32 %iter.i19.sroa.41.08017, !dbg !22153
  %data.i.i.i.i = getelementptr inbounds nuw float, ptr %_192.i89, i32 %iter.i19.sroa.41.08017, !dbg !22156
  %data.i.i4096 = getelementptr inbounds nuw float, ptr %_201.i90, i32 %iter.i19.sroa.41.08017, !dbg !22159
  %_0.i2823 = fadd float %_5.i19258019, -1.000000e+00, !dbg !22162
  %_3.i.i = fcmp ogt float %_0.i2823, 0.000000e+00, !dbg !22167
  %_0.i.i = select i1 %_3.i.i, float %_0.i2823, float 0.000000e+00, !dbg !22170
  %_0.i1983 = fadd float %_0.i32478048, %_12.i19318077, !dbg !22172
  %_0.i3247 = select i1 %_3.i.i, float %_0.i1983, float %_13.i193345314533, !dbg !22174
  %_0.i3241 = select i1 %_3.i.i, float %_12.i19318077, float 0.000000e+00, !dbg !22176
  %_0.i2824 = fadd float %_5.i19138107, -1.000000e+00, !dbg !22178
  %_3.i.i3500 = fcmp ogt float %_0.i2824, 0.000000e+00, !dbg !22181
  %_0.i.i3506 = select i1 %_3.i.i3500, float %_0.i2824, float 0.000000e+00, !dbg !22184
  %_0.i1984 = fadd float %_0.i32608136, %_12.i19198165, !dbg !22186
  %_0.i3260 = select i1 %_3.i.i3500, float %_0.i1984, float %_13.i192145344536, !dbg !22188
  %_0.i3253 = select i1 %_3.i.i3500, float %_12.i19198165, float 0.000000e+00, !dbg !22190
  %_0.i2825 = fadd float %_5.i19018195, -1.000000e+00, !dbg !22192
  %_3.i.i3507 = fcmp ogt float %_0.i2825, 0.000000e+00, !dbg !22197
  %_0.i.i3513 = select i1 %_3.i.i3507, float %_0.i2825, float 0.000000e+00, !dbg !22200
  %_0.i1985 = fadd float %_0.i32738224, %_12.i19078253, !dbg !22202
  %_0.i3273 = select i1 %_3.i.i3507, float %_0.i1985, float %_13.i190945374539, !dbg !22204
  %_0.i3266 = select i1 %_3.i.i3507, float %_12.i19078253, float 0.000000e+00, !dbg !22206
  %_0.i2826 = fadd float %_5.i18898283, -1.000000e+00, !dbg !22208
  %_3.i.i3514 = fcmp ogt float %_0.i2826, 0.000000e+00, !dbg !22211
  %_0.i.i3520 = select i1 %_3.i.i3514, float %_0.i2826, float 0.000000e+00, !dbg !22214
  %_0.i1986 = fadd float %_0.i32868312, %_12.i18958341, !dbg !22216
  %_0.i3286 = select i1 %_3.i.i3514, float %_0.i1986, float %_13.i189745404542, !dbg !22218
  %_0.i3279 = select i1 %_3.i.i3514, float %_12.i18958341, float 0.000000e+00, !dbg !22220
  %_0.i3031 = load float, ptr %data.i.i.i.i, align 4, !dbg !22222, !alias.scope !22225, !noalias !21469, !noundef !10
  %_0.i3026 = load float, ptr %data.i.i4096, align 4, !dbg !22228, !alias.scope !22231, !noalias !21469, !noundef !10
  %_3.i.i3674 = fcmp ule float %_0.i3026, %_0.i3031, !dbg !22234
  %_6.i.i3676 = bitcast float %_0.i3026 to i32, !dbg !22238
  %_8.i.i3678 = bitcast float %_0.i3031 to i32, !dbg !22241
  %_4.i.i3681 = select i1 %_3.i.i3674, i32 %_8.i.i3678, i32 %_6.i.i3676, !dbg !22243
  %_5.i3452 = and i32 %_4.i.i3681, %.none.i41, !dbg !22244
  %_7.i3448 = and i32 %_9.i3454, %_6.i.i3676, !dbg !22247
  %_4.i3449 = or disjoint i32 %_5.i3452, %_7.i3448, !dbg !22250
  %_0.i3450 = bitcast i32 %_4.i3449 to float, !dbg !22251
  %_0.i3021 = load float, ptr %data.i.i.i.i.i.i, align 4, !dbg !22253, !alias.scope !22256, !noalias !21469, !noundef !10
  %_0.i3016 = load float, ptr %data.i5.i.i.i.i.i, align 4, !dbg !22259, !alias.scope !22262, !noalias !21469, !noundef !10
  %_213.i128 = add nuw i32 %iter.i19.sroa.41.08017, %ring_cursor.sroa.0.1.i668595, !dbg !22265
  %_214.i129 = add nuw i32 %iter.i19.sroa.41.08017, %main_cursor.sroa.0.1.i678596, !dbg !22269
  %_215.i130 = add nuw i32 %iter.i19.sroa.41.08017, %left_end.sroa.0.0.i, !dbg !22270
  %_216.i131 = add i32 %iter.i19.sroa.41.08017, %start1.sroa.0.0.i1797, !dbg !22271
  %_217.i132 = add nuw i32 %iter.i19.sroa.41.08017, %left_expiring.sroa.0.0.i, !dbg !22272
  %_7.i8.i260.i = add i32 %_213.i128, 1, !dbg !22273
  %exitcond11946.not = icmp eq i32 %iter.i19.sroa.41.08017, %625, !dbg !22281
  br i1 %exitcond11946.not, label %bb4.i13.i319.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i264.i, !dbg !22281, !prof !4694

bb4.i13.i319.i:                                   ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018
  store float %_0.i.i.lcssa1268514052, ptr %570, align 4
  store float %_0.i3247.lcssa1267114070, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265714088, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264314106, ptr %573, align 4
  store float %_0.i3260.lcssa1262914124, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261514142, ptr %574, align 4
  store float %_0.i.i3513.lcssa1260114160, ptr %576, align 4
  store float %_0.i3273.lcssa1258714178, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257314196, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255914214, ptr %579, align 4
  store float %_0.i3286.lcssa1254514232, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1253114250, ptr %580, align 4
  store float %_0.i2854.lcssa1271614268, ptr %585, align 4
  store float %_0.i3228.lcssa1273214286, ptr %587, align 4
  store float %_0.i2850.lcssa1275614304, ptr %593, align 4
  store float %_0.i3224.lcssa1275714322, ptr %595, align 4
  %628 = add i32 %umax11944, 1, !dbg !22133
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_213.i128, i32 noundef %628, i32 noundef range(i32 0, 536870912) %_54.1.i258.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #33, !dbg !22288, !noalias !22289
  unreachable, !dbg !22288

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i264.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018
  %_7.i3455 = and i32 %_9.i3454, %_8.i.i3678, !dbg !22297
  %_4.i3456 = or disjoint i32 %_5.i3452, %_7.i3455, !dbg !22244
  %_0.i3457 = bitcast i32 %_4.i3456 to float, !dbg !22298
  %_0.i2394 = fdiv float %_0.i3247, %_0.i3457, !dbg !22300
  %_3.i1961 = fcmp uge float %_0.i3247, %_0.i3457, !dbg !22302
  %_0.i3443 = select i1 %_3.i1961, float 1.000000e+00, float %_0.i2394, !dbg !22304
  %_17.i12.i265.i = getelementptr inbounds nuw float, ptr %_54.0.i257.i, i32 %_213.i128, !dbg !22306
  store float %_0.i3443, ptr %_17.i12.i265.i, align 4, !dbg !22310, !alias.scope !22312, !noalias !22315
  %or.cond.i1351.not = icmp ult i32 %_215.i130, %_54.1.i258.i, !dbg !22316
  br i1 %or.cond.i1351.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1355, label %bb4.i1354, !dbg !22316, !prof !10587

bb4.i1354:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i264.i
  store float %_0.i.i.lcssa1268514052, ptr %570, align 4
  store float %_0.i3247.lcssa1267114070, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265714088, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264314106, ptr %573, align 4
  store float %_0.i3260.lcssa1262914124, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261514142, ptr %574, align 4
  store float %_0.i.i3513.lcssa1260114160, ptr %576, align 4
  store float %_0.i3273.lcssa1258714178, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257314196, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255914214, ptr %579, align 4
  store float %_0.i3286.lcssa1254514232, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1253114250, ptr %580, align 4
  store float %_0.i2854.lcssa1271614268, ptr %585, align 4
  store float %_0.i3228.lcssa1273214286, ptr %587, align 4
  store float %_0.i2850.lcssa1275614304, ptr %593, align 4
  store float %_0.i3224.lcssa1275714322, ptr %595, align 4
  %_5.i1348 = add i32 %_215.i130, 1, !dbg !22328
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_215.i130, i32 noundef %_5.i1348, i32 noundef range(i32 0, 536870912) %_54.1.i258.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !22329, !noalias !22330
  unreachable, !dbg !22329

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1355: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i264.i
  %_15.i1352 = getelementptr inbounds nuw float, ptr %_54.0.i257.i, i32 %_215.i130, !dbg !22336
  %_0.i2888 = load float, ptr %_15.i1352, align 4, !dbg !22340, !alias.scope !22342, !noalias !22345, !noundef !10
  %629 = icmp eq i32 %storemerge.i11858370, 0, !dbg !22346
  %_3.i.i3800.inv = fcmp olt float %running.sroa.0.0.i11808397, %_0.i2888, !dbg !22346
  %_4.i.i3807.v = select i1 %_3.i.i3800.inv, float %running.sroa.0.0.i11808397, float %_0.i2888, !dbg !22346
  %running.sroa.0.0.i1180 = select i1 %629, float %_0.i2888, float %_4.i.i3807.v, !dbg !22346
  %_15.i1181 = add i32 %storemerge.i11858370, 1, !dbg !22349
  %complete.i1182 = icmp eq i32 %_15.i1181, %_18.i268.i, !dbg !22349
  br i1 %complete.i1182, label %bb11.i1188.preheader, label %bb7.i1183, !dbg !22351

bb11.i1188.preheader:                             ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1355
  br i1 %_29.i11928007.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1204, label %bb19.i1193, !dbg !22353

bb7.i1183:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1355
  %or.cond.i1343.not = icmp ult i32 %_216.i131, %_54.1.i258.i, !dbg !22363
  br i1 %or.cond.i1343.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1347, label %bb4.i1346, !dbg !22363, !prof !10587

bb4.i1346:                                        ; preds = %bb7.i1183
  store float %_0.i.i.lcssa1268514052, ptr %570, align 4
  store float %_0.i3247.lcssa1267114070, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265714088, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264314106, ptr %573, align 4
  store float %_0.i3260.lcssa1262914124, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261514142, ptr %574, align 4
  store float %_0.i.i3513.lcssa1260114160, ptr %576, align 4
  store float %_0.i3273.lcssa1258714178, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257314196, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255914214, ptr %579, align 4
  store float %_0.i3286.lcssa1254514232, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1253114250, ptr %580, align 4
  store float %_0.i2854.lcssa1271614268, ptr %585, align 4
  store float %_0.i3228.lcssa1273214286, ptr %587, align 4
  store float %_0.i2850.lcssa1275614304, ptr %593, align 4
  store float %_0.i3224.lcssa1275714322, ptr %595, align 4
  %_5.i1340 = add i32 %_216.i131, 1, !dbg !22368
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_216.i131, i32 noundef %_5.i1340, i32 noundef range(i32 0, 536870912) %_54.1.i258.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !22369, !noalias !22370
  unreachable, !dbg !22369

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1347: ; preds = %bb7.i1183
  %_15.i1344 = getelementptr inbounds nuw float, ptr %_54.0.i257.i, i32 %_216.i131, !dbg !22373
  %_0.i2890 = load float, ptr %_15.i1344, align 4, !dbg !22375, !alias.scope !22377, !noalias !22345, !noundef !10
  %_3.i.i3791.inv = fcmp olt float %_0.i2890, %running.sroa.0.0.i1180, !dbg !22380
  %_4.i.i3798.v = select i1 %_3.i.i3791.inv, float %_0.i2890, float %running.sroa.0.0.i1180, !dbg !22380
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1204, !dbg !22384

bb19.i1193:                                       ; preds = %bb11.i1188.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1200
  %end.sroa.0.0.i11918010 = phi i32 [ %631, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1200 ], [ %_215.i130, %bb11.i1188.preheader ]
  %suffix.sroa.0.0.i11908009 = phi float [ %_4.i.i3789.v, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1200 ], [ %_0.i2888, %bb11.i1188.preheader ]
  %iter.sroa.0.0.i11898008 = phi i32 [ %_30.i1194, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1200 ], [ 0, %bb11.i1188.preheader ]
  %or.cond.i1327.not = icmp ult i32 %end.sroa.0.0.i11918010, %_54.1.i258.i, !dbg !22385
  br i1 %or.cond.i1327.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1200, label %bb4.i1330, !dbg !22385, !prof !10587

bb4.i1330:                                        ; preds = %bb19.i1193
  store float %_0.i.i.lcssa1268514052, ptr %570, align 4
  store float %_0.i3247.lcssa1267114070, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265714088, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264314106, ptr %573, align 4
  store float %_0.i3260.lcssa1262914124, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261514142, ptr %574, align 4
  store float %_0.i.i3513.lcssa1260114160, ptr %576, align 4
  store float %_0.i3273.lcssa1258714178, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257314196, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255914214, ptr %579, align 4
  store float %_0.i3286.lcssa1254514232, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1253114250, ptr %580, align 4
  store float %_0.i2854.lcssa1271614268, ptr %585, align 4
  store float %_0.i3228.lcssa1273214286, ptr %587, align 4
  store float %_0.i2850.lcssa1275614304, ptr %593, align 4
  store float %_0.i3224.lcssa1275714322, ptr %595, align 4
  %_5.i1324 = add i32 %end.sroa.0.0.i11918010, 1, !dbg !22390
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %end.sroa.0.0.i11918010, i32 noundef %_5.i1324, i32 noundef range(i32 0, 536870912) %_54.1.i258.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !22391, !noalias !22392
  unreachable, !dbg !22391

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1200: ; preds = %bb19.i1193
  %_30.i1194 = add nuw i32 %iter.sroa.0.0.i11898008, 1, !dbg !22395
  %_15.i1328 = getelementptr inbounds nuw float, ptr %_54.0.i257.i, i32 %end.sroa.0.0.i11918010, !dbg !22401
  %_0.i2894 = load float, ptr %_15.i1328, align 4, !dbg !22403, !alias.scope !22405, !noalias !22345, !noundef !10
  %_3.i.i3782.inv = fcmp olt float %suffix.sroa.0.0.i11908009, %_0.i2894, !dbg !22408
  %_4.i.i3789.v = select i1 %_3.i.i3782.inv, float %suffix.sroa.0.0.i11908009, float %_0.i2894, !dbg !22408
  store float %_4.i.i3789.v, ptr %_15.i1328, align 4, !dbg !22411, !alias.scope !22414, !noalias !22345
  %630 = icmp eq i32 %end.sroa.0.0.i11918010, 0, !dbg !22417
  %spec.store.select.i1202 = select i1 %630, i32 %ring.i44, i32 %end.sroa.0.0.i11918010, !dbg !22417
  %631 = add i32 %spec.store.select.i1202, -1, !dbg !22418
  %exitcond11942.not = icmp eq i32 %_30.i1194, %_18.i268.i, !dbg !22419
  br i1 %exitcond11942.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1204, label %bb19.i1193, !dbg !22353

_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1204: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1200, %bb11.i1188.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1347
  %storemerge.i1185 = phi i32 [ %_15.i1181, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1347 ], [ 0, %bb11.i1188.preheader ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1200 ], !dbg !22422
  %running.sroa.0.1.i1186 = phi float [ %_4.i.i3798.v, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1347 ], [ %running.sroa.0.0.i1180, %bb11.i1188.preheader ], [ %running.sroa.0.0.i1180, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1200 ], !dbg !22423
  %_0.i2720 = fmul float %running.sroa.0.1.i1186, 1.638400e+04, !dbg !22424
  %632 = tail call noundef float @llvm.floor.f32(float %_0.i2720), !dbg !22427
  %_0.i2719 = fmul float %632, 0x3F10000000000000, !dbg !22431
  %or.cond.i1415.not = icmp ult i32 %_217.i132, %_56.1.i280.i, !dbg !22433
  br i1 %or.cond.i1415.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1419, label %bb4.i1418, !dbg !22433, !prof !10587

bb4.i1418:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1204
  store float %_0.i.i.lcssa1268514052, ptr %570, align 4
  store float %_0.i3247.lcssa1267114070, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265714088, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264314106, ptr %573, align 4
  store float %_0.i3260.lcssa1262914124, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261514142, ptr %574, align 4
  store float %_0.i.i3513.lcssa1260114160, ptr %576, align 4
  store float %_0.i3273.lcssa1258714178, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257314196, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255914214, ptr %579, align 4
  store float %_0.i3286.lcssa1254514232, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1253114250, ptr %580, align 4
  store float %_0.i2854.lcssa1271614268, ptr %585, align 4
  store float %_0.i3228.lcssa1273214286, ptr %587, align 4
  store float %_0.i2850.lcssa1275614304, ptr %593, align 4
  store float %_0.i3224.lcssa1275714322, ptr %595, align 4
  %_5.i1412 = add i32 %_217.i132, 1, !dbg !22439
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_217.i132, i32 noundef %_5.i1412, i32 noundef range(i32 0, 536870912) %_56.1.i280.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !22440, !noalias !22441
  unreachable, !dbg !22440

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1419: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1204
  %_15.i1416 = getelementptr inbounds nuw float, ptr %_56.0.i279.i, i32 %_217.i132, !dbg !22444
  %_0.i2872 = load float, ptr %_15.i1416, align 4, !dbg !22446, !alias.scope !22448, !noalias !22451, !noundef !10
  %_0.i2284 = fadd float %_0.i2719, %_0.i28548425, !dbg !22452
  %_0.i2854 = fsub float %_0.i2284, %_0.i2872, !dbg !22455
  %_8.not.i3.i289.i = icmp ugt i32 %_7.i8.i260.i, %_56.1.i280.i
  br i1 %_8.not.i3.i289.i, label %bb4.i6.i318.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i290.i, !dbg !22457, !prof !4694

bb4.i6.i318.i:                                    ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1419
  store float %_0.i.i.lcssa1268514052, ptr %570, align 4
  store float %_0.i3247.lcssa1267114070, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265714088, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264314106, ptr %573, align 4
  store float %_0.i3260.lcssa1262914124, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261514142, ptr %574, align 4
  store float %_0.i.i3513.lcssa1260114160, ptr %576, align 4
  store float %_0.i3273.lcssa1258714178, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257314196, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255914214, ptr %579, align 4
  store float %_0.i3286.lcssa1254514232, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1253114250, ptr %580, align 4
  store float %_0.i2854.lcssa1271614268, ptr %585, align 4
  store float %_0.i3228.lcssa1273214286, ptr %587, align 4
  store float %_0.i2850.lcssa1275614304, ptr %593, align 4
  store float %_0.i3224.lcssa1275714322, ptr %595, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_213.i128, i32 noundef %_7.i8.i260.i, i32 noundef range(i32 0, 536870912) %_56.1.i280.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #33, !dbg !22462, !noalias !22463
  unreachable, !dbg !22462

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i290.i: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1419
  %_17.i5.i291.i = getelementptr inbounds nuw float, ptr %_56.0.i279.i, i32 %_213.i128, !dbg !22466
  store float %_0.i2719, ptr %_17.i5.i291.i, align 4, !dbg !22468, !alias.scope !22470, !noalias !22451
  %_0.i2393 = fdiv float %_0.i2854, %_37.i292.i, !dbg !22473
  %_0.i2853 = fsub float 1.000000e+00, %_0.i2393, !dbg !22475
  %_0.i2852 = fsub float %_0.i2853, %_0.i32288454, !dbg !22478
  %_4.i2409 = fmul float %_0.i3260, %_0.i2852, !dbg !22481
  %_0.i2410 = fadd float %_0.i32288454, %_4.i2409, !dbg !22481
  %_3.i.i3665.inv = fcmp ogt float %_0.i2853, %_0.i2410, !dbg !22483
  %_4.i.i3672.v = select i1 %_3.i.i3665.inv, float %_0.i2853, float %_0.i2410, !dbg !22483
  %633 = tail call noundef float @llvm.fabs.f32(float %_4.i.i3672.v), !dbg !22487
  %634 = fcmp uge float %633, 0x3BC79CA100000000, !dbg !22491
  %_0.i3228 = select i1 %634, float %_4.i.i3672.v, float 0.000000e+00, !dbg !22493
  %_5.i1404 = add i32 %_214.i129, 1, !dbg !22494
  %exitcond11947.not = icmp eq i32 %iter.i19.sroa.41.08017, %626, !dbg !22497
  br i1 %exitcond11947.not, label %bb4.i1410, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3172, !dbg !22497, !prof !4694

bb4.i1410:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i290.i
  store float %_0.i.i.lcssa1268514052, ptr %570, align 4
  store float %_0.i3247.lcssa1267114070, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265714088, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264314106, ptr %573, align 4
  store float %_0.i3260.lcssa1262914124, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261514142, ptr %574, align 4
  store float %_0.i.i3513.lcssa1260114160, ptr %576, align 4
  store float %_0.i3273.lcssa1258714178, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257314196, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255914214, ptr %579, align 4
  store float %_0.i3286.lcssa1254514232, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1253114250, ptr %580, align 4
  store float %_0.i2854.lcssa1271614268, ptr %585, align 4
  store float %_0.i3228.lcssa1273214286, ptr %587, align 4
  store float %_0.i2850.lcssa1275614304, ptr %593, align 4
  store float %_0.i3224.lcssa1275714322, ptr %595, align 4
  %635 = add i32 %umax11945, 1, !dbg !22133
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_214.i129, i32 noundef %635, i32 noundef range(i32 0, 536870912) %_58.1.i305.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !22501, !noalias !22502
  unreachable, !dbg !22501

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3172: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i290.i
  %_0.i2851 = fsub float 1.000000e+00, %_0.i3228, !dbg !22505
  %_15.i1408 = getelementptr inbounds nuw float, ptr %_58.0.i304.i, i32 %_214.i129, !dbg !22507
  %_0.i2874 = load float, ptr %_15.i1408, align 4, !dbg !22509, !alias.scope !22511, !noalias !22451, !noundef !10
  store float %_0.i3021, ptr %_15.i1408, align 4, !dbg !22514, !alias.scope !22518, !noalias !22451
  %_0.i2718 = fmul float %_0.i2851, %_0.i2874, !dbg !22521
  %_6.i3431 = bitcast float %_0.i2874 to i32, !dbg !22523
  %_5.i3432 = and i32 %_6.i3431, %all.sroa.0.0.i43, !dbg !22526
  %_8.i3433 = bitcast float %_0.i2718 to i32, !dbg !22527
  %_7.i3435 = and i32 %_9.i3434, %_8.i3433, !dbg !22529
  %_4.i3436 = or disjoint i32 %_7.i3435, %_5.i3432, !dbg !22526
  store i32 %_4.i3436, ptr %data.i.i.i.i.i.i, align 4, !dbg !22530, !alias.scope !22532, !noalias !22535
  %_220.i140 = add nuw i32 %iter.i19.sroa.41.08017, %right_end.sroa.0.0.i, !dbg !22536
  %_222.i142 = add nuw i32 %iter.i19.sroa.41.08017, %right_expiring.sroa.0.0.i, !dbg !22538
  %_8.not.i10.i.i152 = icmp ugt i32 %_7.i8.i260.i, %_54.1.i.i148
  br i1 %_8.not.i10.i.i152, label %bb4.i13.i.i202, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i154, !dbg !22539, !prof !4694

bb4.i13.i.i202:                                   ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3172
  store float %_0.i.i.lcssa1268514052, ptr %570, align 4
  store float %_0.i3247.lcssa1267114070, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265714088, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264314106, ptr %573, align 4
  store float %_0.i3260.lcssa1262914124, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261514142, ptr %574, align 4
  store float %_0.i.i3513.lcssa1260114160, ptr %576, align 4
  store float %_0.i3273.lcssa1258714178, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257314196, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255914214, ptr %579, align 4
  store float %_0.i3286.lcssa1254514232, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1253114250, ptr %580, align 4
  store float %_0.i2854.lcssa1271614268, ptr %585, align 4
  store float %_0.i3228.lcssa1273214286, ptr %587, align 4
  store float %_0.i2850.lcssa1275614304, ptr %593, align 4
  store float %_0.i3224.lcssa1275714322, ptr %595, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_213.i128, i32 noundef %_7.i8.i260.i, i32 noundef range(i32 0, 536870912) %_54.1.i.i148, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #33, !dbg !22545, !noalias !22546
  unreachable, !dbg !22545

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i154: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3172
  %_0.i2392 = fdiv float %_0.i3273, %_0.i3450, !dbg !22554
  %_3.i1959 = fcmp uge float %_0.i3273, %_0.i3450, !dbg !22556
  %_0.i3430 = select i1 %_3.i1959, float 1.000000e+00, float %_0.i2392, !dbg !22558
  %_17.i12.i.i155 = getelementptr inbounds nuw float, ptr %_54.0.i.i147, i32 %_213.i128, !dbg !22560
  store float %_0.i3430, ptr %_17.i12.i.i155, align 4, !dbg !22562, !alias.scope !22564, !noalias !22567
  %or.cond.i1383.not = icmp ult i32 %_220.i140, %_54.1.i.i148, !dbg !22568
  br i1 %or.cond.i1383.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1387, label %bb4.i1386, !dbg !22568, !prof !10587

bb4.i1386:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i154
  store float %_0.i.i.lcssa1268514052, ptr %570, align 4
  store float %_0.i3247.lcssa1267114070, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265714088, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264314106, ptr %573, align 4
  store float %_0.i3260.lcssa1262914124, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261514142, ptr %574, align 4
  store float %_0.i.i3513.lcssa1260114160, ptr %576, align 4
  store float %_0.i3273.lcssa1258714178, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257314196, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255914214, ptr %579, align 4
  store float %_0.i3286.lcssa1254514232, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1253114250, ptr %580, align 4
  store float %_0.i2854.lcssa1271614268, ptr %585, align 4
  store float %_0.i3228.lcssa1273214286, ptr %587, align 4
  store float %_0.i2850.lcssa1275614304, ptr %593, align 4
  store float %_0.i3224.lcssa1275714322, ptr %595, align 4
  %_5.i1380 = add i32 %_220.i140, 1, !dbg !22574
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_220.i140, i32 noundef %_5.i1380, i32 noundef range(i32 0, 536870912) %_54.1.i.i148, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !22575, !noalias !22576
  unreachable, !dbg !22575

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1387: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i154
  %_15.i1384 = getelementptr inbounds nuw float, ptr %_54.0.i.i147, i32 %_220.i140, !dbg !22582
  %_0.i2880 = load float, ptr %_15.i1384, align 4, !dbg !22584, !alias.scope !22586, !noalias !22589, !noundef !10
  %636 = icmp eq i32 %storemerge.i8482, 0, !dbg !22590
  %_3.i.i3827.inv = fcmp olt float %running.sroa.0.0.i8509, %_0.i2880, !dbg !22590
  %_4.i.i3834.v = select i1 %_3.i.i3827.inv, float %running.sroa.0.0.i8509, float %_0.i2880, !dbg !22590
  %running.sroa.0.0.i = select i1 %636, float %_0.i2880, float %_4.i.i3834.v, !dbg !22590
  %_15.i = add i32 %storemerge.i8482, 1, !dbg !22591
  %complete.i = icmp eq i32 %_15.i, %_18.i.i158, !dbg !22591
  br i1 %complete.i, label %bb11.i1168.preheader, label %bb7.i1166, !dbg !22592

bb11.i1168.preheader:                             ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1387
  br i1 %_29.i11708011.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit, label %bb19.i1171, !dbg !22593

bb7.i1166:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1387
  %or.cond.i1375.not = icmp ult i32 %_216.i131, %_54.1.i.i148, !dbg !22596
  br i1 %or.cond.i1375.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1379, label %bb4.i1378, !dbg !22596, !prof !10587

bb4.i1378:                                        ; preds = %bb7.i1166
  store float %_0.i.i.lcssa1268514052, ptr %570, align 4
  store float %_0.i3247.lcssa1267114070, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265714088, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264314106, ptr %573, align 4
  store float %_0.i3260.lcssa1262914124, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261514142, ptr %574, align 4
  store float %_0.i.i3513.lcssa1260114160, ptr %576, align 4
  store float %_0.i3273.lcssa1258714178, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257314196, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255914214, ptr %579, align 4
  store float %_0.i3286.lcssa1254514232, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1253114250, ptr %580, align 4
  store float %_0.i2854.lcssa1271614268, ptr %585, align 4
  store float %_0.i3228.lcssa1273214286, ptr %587, align 4
  store float %_0.i2850.lcssa1275614304, ptr %593, align 4
  store float %_0.i3224.lcssa1275714322, ptr %595, align 4
  %_5.i1372 = add i32 %_216.i131, 1, !dbg !22601
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_216.i131, i32 noundef %_5.i1372, i32 noundef range(i32 0, 536870912) %_54.1.i.i148, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !22602, !noalias !22603
  unreachable, !dbg !22602

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1379: ; preds = %bb7.i1166
  %_15.i1376 = getelementptr inbounds nuw float, ptr %_54.0.i.i147, i32 %_216.i131, !dbg !22606
  %_0.i2882 = load float, ptr %_15.i1376, align 4, !dbg !22608, !alias.scope !22610, !noalias !22589, !noundef !10
  %_3.i.i3818.inv = fcmp olt float %_0.i2882, %running.sroa.0.0.i, !dbg !22613
  %_4.i.i3825.v = select i1 %_3.i.i3818.inv, float %_0.i2882, float %running.sroa.0.0.i, !dbg !22613
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit, !dbg !22616

bb19.i1171:                                       ; preds = %bb11.i1168.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i
  %end.sroa.0.0.i8014 = phi i32 [ %638, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], [ %_220.i140, %bb11.i1168.preheader ]
  %suffix.sroa.0.0.i8013 = phi float [ %_4.i.i3816.v, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], [ %_0.i2880, %bb11.i1168.preheader ]
  %iter.sroa.0.0.i11698012 = phi i32 [ %_30.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], [ 0, %bb11.i1168.preheader ]
  %or.cond.i1359.not = icmp ult i32 %end.sroa.0.0.i8014, %_54.1.i.i148, !dbg !22617
  br i1 %or.cond.i1359.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i, label %bb4.i1362, !dbg !22617, !prof !10587

bb4.i1362:                                        ; preds = %bb19.i1171
  store float %_0.i.i.lcssa1268514052, ptr %570, align 4
  store float %_0.i3247.lcssa1267114070, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265714088, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264314106, ptr %573, align 4
  store float %_0.i3260.lcssa1262914124, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261514142, ptr %574, align 4
  store float %_0.i.i3513.lcssa1260114160, ptr %576, align 4
  store float %_0.i3273.lcssa1258714178, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257314196, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255914214, ptr %579, align 4
  store float %_0.i3286.lcssa1254514232, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1253114250, ptr %580, align 4
  store float %_0.i2854.lcssa1271614268, ptr %585, align 4
  store float %_0.i3228.lcssa1273214286, ptr %587, align 4
  store float %_0.i2850.lcssa1275614304, ptr %593, align 4
  store float %_0.i3224.lcssa1275714322, ptr %595, align 4
  %_5.i1356 = add i32 %end.sroa.0.0.i8014, 1, !dbg !22622
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %end.sroa.0.0.i8014, i32 noundef %_5.i1356, i32 noundef range(i32 0, 536870912) %_54.1.i.i148, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !22623, !noalias !22624
  unreachable, !dbg !22623

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i: ; preds = %bb19.i1171
  %_30.i = add nuw i32 %iter.sroa.0.0.i11698012, 1, !dbg !22627
  %_15.i1360 = getelementptr inbounds nuw float, ptr %_54.0.i.i147, i32 %end.sroa.0.0.i8014, !dbg !22630
  %_0.i2886 = load float, ptr %_15.i1360, align 4, !dbg !22632, !alias.scope !22634, !noalias !22589, !noundef !10
  %_3.i.i3809.inv = fcmp olt float %suffix.sroa.0.0.i8013, %_0.i2886, !dbg !22637
  %_4.i.i3816.v = select i1 %_3.i.i3809.inv, float %suffix.sroa.0.0.i8013, float %_0.i2886, !dbg !22637
  store float %_4.i.i3816.v, ptr %_15.i1360, align 4, !dbg !22640, !alias.scope !22643, !noalias !22589
  %637 = icmp eq i32 %end.sroa.0.0.i8014, 0, !dbg !22646
  %spec.store.select.i1174 = select i1 %637, i32 %ring.i44, i32 %end.sroa.0.0.i8014, !dbg !22646
  %638 = add i32 %spec.store.select.i1174, -1, !dbg !22647
  %exitcond11943.not = icmp eq i32 %_30.i, %_18.i.i158, !dbg !22648
  br i1 %exitcond11943.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit, label %bb19.i1171, !dbg !22593

_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i, %bb11.i1168.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1379
  %storemerge.i = phi i32 [ %_15.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1379 ], [ 0, %bb11.i1168.preheader ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], !dbg !22650
  %running.sroa.0.1.i = phi float [ %_4.i.i3825.v, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1379 ], [ %running.sroa.0.0.i, %bb11.i1168.preheader ], [ %running.sroa.0.0.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], !dbg !22651
  %_0.i2717 = fmul float %running.sroa.0.1.i, 1.638400e+04, !dbg !22652
  %639 = tail call noundef float @llvm.floor.f32(float %_0.i2717), !dbg !22654
  %_0.i2716 = fmul float %639, 0x3F10000000000000, !dbg !22658
  %or.cond.i1399.not = icmp ult i32 %_222.i142, %_56.1.i.i166, !dbg !22660
  br i1 %or.cond.i1399.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1403, label %bb4.i1402, !dbg !22660, !prof !10587

bb4.i1402:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit
  store float %_0.i.i.lcssa1268514052, ptr %570, align 4
  store float %_0.i3247.lcssa1267114070, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265714088, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264314106, ptr %573, align 4
  store float %_0.i3260.lcssa1262914124, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261514142, ptr %574, align 4
  store float %_0.i.i3513.lcssa1260114160, ptr %576, align 4
  store float %_0.i3273.lcssa1258714178, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257314196, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255914214, ptr %579, align 4
  store float %_0.i3286.lcssa1254514232, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1253114250, ptr %580, align 4
  store float %_0.i2854.lcssa1271614268, ptr %585, align 4
  store float %_0.i3228.lcssa1273214286, ptr %587, align 4
  store float %_0.i2850.lcssa1275614304, ptr %593, align 4
  store float %_0.i3224.lcssa1275714322, ptr %595, align 4
  %_5.i1396 = add i32 %_222.i142, 1, !dbg !22665
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_222.i142, i32 noundef %_5.i1396, i32 noundef range(i32 0, 536870912) %_56.1.i.i166, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !22666, !noalias !22667
  unreachable, !dbg !22666

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1403: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit
  %_15.i1400 = getelementptr inbounds nuw float, ptr %_56.0.i.i165, i32 %_222.i142, !dbg !22670
  %_0.i2876 = load float, ptr %_15.i1400, align 4, !dbg !22672, !alias.scope !22674, !noalias !22677, !noundef !10
  %_0.i2283 = fadd float %_0.i2716, %_0.i28508537, !dbg !22678
  %_0.i2850 = fsub float %_0.i2283, %_0.i2876, !dbg !22680
  %_8.not.i3.i.i173 = icmp ugt i32 %_7.i8.i260.i, %_56.1.i.i166
  br i1 %_8.not.i3.i.i173, label %bb4.i6.i.i201, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i174, !dbg !22682, !prof !4694

bb4.i6.i.i201:                                    ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1403
  store float %_0.i.i.lcssa1268514052, ptr %570, align 4
  store float %_0.i3247.lcssa1267114070, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265714088, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264314106, ptr %573, align 4
  store float %_0.i3260.lcssa1262914124, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261514142, ptr %574, align 4
  store float %_0.i.i3513.lcssa1260114160, ptr %576, align 4
  store float %_0.i3273.lcssa1258714178, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257314196, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255914214, ptr %579, align 4
  store float %_0.i3286.lcssa1254514232, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1253114250, ptr %580, align 4
  store float %_0.i2854.lcssa1271614268, ptr %585, align 4
  store float %_0.i3228.lcssa1273214286, ptr %587, align 4
  store float %_0.i2850.lcssa1275614304, ptr %593, align 4
  store float %_0.i3224.lcssa1275714322, ptr %595, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_213.i128, i32 noundef %_7.i8.i260.i, i32 noundef range(i32 0, 536870912) %_56.1.i.i166, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #33, !dbg !22687, !noalias !22688
  unreachable, !dbg !22687

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i174: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1403
  %_17.i5.i.i175 = getelementptr inbounds nuw float, ptr %_56.0.i.i165, i32 %_213.i128, !dbg !22691
  store float %_0.i2716, ptr %_17.i5.i.i175, align 4, !dbg !22693, !alias.scope !22695, !noalias !22677
  %_6.not.i1390 = icmp ugt i32 %_5.i1404, %_58.1.i.i189
  br i1 %_6.not.i1390, label %bb4.i1394, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165, !dbg !22698, !prof !4694

bb4.i1394:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i174
  store float %_0.i.i.lcssa1268514052, ptr %570, align 4
  store float %_0.i3247.lcssa1267114070, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265714088, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264314106, ptr %573, align 4
  store float %_0.i3260.lcssa1262914124, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261514142, ptr %574, align 4
  store float %_0.i.i3513.lcssa1260114160, ptr %576, align 4
  store float %_0.i3273.lcssa1258714178, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257314196, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255914214, ptr %579, align 4
  store float %_0.i3286.lcssa1254514232, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1253114250, ptr %580, align 4
  store float %_0.i2854.lcssa1271614268, ptr %585, align 4
  store float %_0.i3228.lcssa1273214286, ptr %587, align 4
  store float %_0.i2850.lcssa1275614304, ptr %593, align 4
  store float %_0.i3224.lcssa1275714322, ptr %595, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_214.i129, i32 noundef %_5.i1404, i32 noundef range(i32 0, 536870912) %_58.1.i.i189, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !22703, !noalias !22704
  unreachable, !dbg !22703

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i174
  %_0.i2391 = fdiv float %_0.i2850, %_37.i.i176, !dbg !22707
  %_0.i2849 = fsub float 1.000000e+00, %_0.i2391, !dbg !22709
  %_0.i2848 = fsub float %_0.i2849, %_0.i32248566, !dbg !22711
  %_4.i2407 = fmul float %_0.i3286, %_0.i2848, !dbg !22713
  %_0.i2408 = fadd float %_0.i32248566, %_4.i2407, !dbg !22713
  %_3.i.i3656.inv = fcmp ogt float %_0.i2849, %_0.i2408, !dbg !22715
  %_4.i.i3663.v = select i1 %_3.i.i3656.inv, float %_0.i2849, float %_0.i2408, !dbg !22715
  %640 = tail call noundef float @llvm.fabs.f32(float %_4.i.i3663.v), !dbg !22718
  %641 = fcmp uge float %640, 0x3BC79CA100000000, !dbg !22721
  %_0.i3224 = select i1 %641, float %_4.i.i3663.v, float 0.000000e+00, !dbg !22723
  %_0.i2847 = fsub float 1.000000e+00, %_0.i3224, !dbg !22724
  %_15.i1392 = getelementptr inbounds nuw float, ptr %_58.0.i.i188, i32 %_214.i129, !dbg !22726
  %_0.i2878 = load float, ptr %_15.i1392, align 4, !dbg !22728, !alias.scope !22730, !noalias !22677, !noundef !10
  store float %_0.i3016, ptr %_15.i1392, align 4, !dbg !22733, !alias.scope !22736, !noalias !22677
  %_0.i2715 = fmul float %_0.i2847, %_0.i2878, !dbg !22739
  %_6.i3418 = bitcast float %_0.i2878 to i32, !dbg !22741
  %_5.i3419 = and i32 %_6.i3418, %all.sroa.0.0.i43, !dbg !22744
  %_8.i3420 = bitcast float %_0.i2715 to i32, !dbg !22745
  %_7.i3422 = and i32 %_9.i3434, %_8.i3420, !dbg !22747
  %_4.i3423 = or disjoint i32 %_7.i3422, %_5.i3419, !dbg !22744
  store i32 %_4.i3423, ptr %data.i5.i.i.i.i.i, align 4, !dbg !22748, !alias.scope !22750, !noalias !22753
  %exitcond11956.not = icmp eq i32 %_206.0.i110, %umin11955, !dbg !22133
  br i1 %exitcond11956.not, label %bb67.i203, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018, !dbg !22133

bb67.i203:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit
  %_0.i3224.lcssa1275714321 = phi float [ %_0.i3224.lcssa1275714322, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3224, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i2850.lcssa1275614303 = phi float [ %_0.i2850.lcssa1275614304, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i2850, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i3228.lcssa1273214285 = phi float [ %_0.i3228.lcssa1273214286, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3228, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i2854.lcssa1271614267 = phi float [ %_0.i2854.lcssa1271614268, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i2854, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i3279.lcssa1253114249 = phi float [ %_0.i3279.lcssa1253114250, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3279, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i3286.lcssa1254514231 = phi float [ %_0.i3286.lcssa1254514232, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3286, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i.i3520.lcssa1255914213 = phi float [ %_0.i.i3520.lcssa1255914214, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i.i3520, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i3266.lcssa1257314195 = phi float [ %_0.i3266.lcssa1257314196, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3266, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i3273.lcssa1258714177 = phi float [ %_0.i3273.lcssa1258714178, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3273, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i.i3513.lcssa1260114159 = phi float [ %_0.i.i3513.lcssa1260114160, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i.i3513, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i3253.lcssa1261514141 = phi float [ %_0.i3253.lcssa1261514142, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3253, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i3260.lcssa1262914123 = phi float [ %_0.i3260.lcssa1262914124, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3260, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i.i3506.lcssa1264314105 = phi float [ %_0.i.i3506.lcssa1264314106, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i.i3506, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i3241.lcssa1265714087 = phi float [ %_0.i3241.lcssa1265714088, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3241, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i3247.lcssa1267114069 = phi float [ %_0.i3247.lcssa1267114070, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3247, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i.i.lcssa1268514051 = phi float [ %_0.i.i.lcssa1268514052, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %running.sroa.0.0.i.lcssa85318709 = phi float [ %running.sroa.0.0.i.lcssa85318710, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %running.sroa.0.0.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %storemerge.i.lcssa85048673 = phi i32 [ %storemerge.i.lcssa85048674, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %storemerge.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %running.sroa.0.0.i1180.lcssa84198637 = phi float [ %running.sroa.0.0.i1180.lcssa84198638, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %running.sroa.0.0.i1180, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %storemerge.i1185.lcssa83928601 = phi i32 [ %storemerge.i1185.lcssa83928602, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %storemerge.i1185, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_139.i204 = add i32 %run.sroa.0.0.i, %ring_cursor.sroa.0.1.i668595, !dbg !22754
  %_212.not.i205 = icmp ult i32 %_139.i204, %ring.i44, !dbg !22755
  %642 = select i1 %_212.not.i205, i32 0, i32 %ring.i44, !dbg !22755
  %ring_cursor.sroa.0.2.i206 = sub nuw i32 %_139.i204, %642, !dbg !22755
  %_141.i207 = add i32 %run.sroa.0.0.i, %main_cursor.sroa.0.1.i678596, !dbg !22758
  %_223.not.i208 = icmp ult i32 %_141.i207, %main.i45, !dbg !22759
  %643 = select i1 %_223.not.i208, i32 0, i32 %main.i45, !dbg !22759
  %main_cursor.sroa.0.2.i209 = sub nuw i32 %_141.i207, %643, !dbg !22759
  %_59.i69 = icmp ult i32 %_83.i83, %spec.store.select.i60, !dbg !21531
  br i1 %_59.i69, label %bb20.i70, label %bb19.i65.bb15.i53.loopexit_crit_edge, !dbg !21531

_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit: ; preds = %bb15.i53.loopexit, %bb7.i
  %ring_cursor.sroa.0.0.i54.lcssa = phi i32 [ %_37.i47, %bb7.i ], [ %ring_cursor.sroa.0.1.i66.lcssa, %bb15.i53.loopexit ], !dbg !21499
  %main_cursor.sroa.0.0.i55.lcssa = phi i32 [ %_36.i46, %bb7.i ], [ %main_cursor.sroa.0.1.i67.lcssa, %bb15.i53.loopexit ], !dbg !21496
  %644 = getelementptr inbounds nuw i8, ptr %uniform_left.i32, i32 24, !dbg !22761
  %left_prefix.i411 = load float, ptr %644, align 4, !dbg !22761, !noalias !21475, !noundef !10
  %645 = getelementptr inbounds nuw i8, ptr %uniform_left.i32, i32 28, !dbg !22762
  %left_phase.i412 = load i32, ptr %645, align 4, !dbg !22762, !noalias !21475, !noundef !10
  %646 = getelementptr inbounds nuw i8, ptr %uniform_right.i31, i32 24, !dbg !22763
  %right_prefix.i413 = load float, ptr %646, align 4, !dbg !22763, !noalias !21475, !noundef !10
  %647 = getelementptr inbounds nuw i8, ptr %uniform_right.i31, i32 28, !dbg !22764
  %right_phase.i414 = load i32, ptr %647, align 4, !dbg !22764, !noalias !21475, !noundef !10
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_right.i31), !dbg !22765, !noalias !21475
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i32), !dbg !22766, !noalias !21475
  %648 = getelementptr inbounds nuw i8, ptr %self, i32 364, !dbg !22767
  %_236.0.i415 = load ptr, ptr %648, align 4, !dbg !22767, !alias.scope !21465, !noalias !22769, !nonnull !10, !noundef !10
  %649 = getelementptr inbounds nuw i8, ptr %self, i32 368, !dbg !22767
  %_236.1.i416 = load i32, ptr %649, align 4, !dbg !22767, !alias.scope !21465, !noalias !22769, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !22770), !dbg !22773
  %_4.not.i3150 = icmp eq i32 %_236.1.i416, 0, !dbg !22774
  br i1 %_4.not.i3150, label %panic.i3152, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3153, !dbg !22774

panic.i3152:                                      ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #33, !dbg !22774, !noalias !22776
  unreachable, !dbg !22774

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3153: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit
  store float %left_prefix.i411, ptr %_236.0.i415, align 4, !dbg !22774, !alias.scope !22770, !noalias !21469
  %_237.0.i417 = load ptr, ptr %68, align 4, !dbg !22777, !alias.scope !21465, !noalias !22769, !nonnull !10, !noundef !10
  %_237.1.i418 = load i32, ptr %69, align 4, !dbg !22777, !alias.scope !21465, !noalias !22769, !noundef !10
  %650 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i412), !dbg !22778
  br i1 %650, label %bb2.i4101, label %bb6.i4097, !dbg !22778

bb6.i4097:                                        ; preds = %bb2.i4101, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3153
  %end_or_len.idx.i = shl nuw nsw i32 %_237.1.i418, 2, !dbg !22782
  %end_or_len.i = getelementptr inbounds nuw i8, ptr %_237.0.i417, i32 %end_or_len.idx.i, !dbg !22782
  %_293.i = icmp eq i32 %_237.1.i418, 0, !dbg !22786
  br i1 %_293.i, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i4098, !dbg !22789

bb2.i4101:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3153
  %bytes1.sroa.0.0.zext.i = and i32 %left_phase.i412, 255, !dbg !22790
  %bytes1.sroa.0.0.isplat.i = mul nuw i32 %bytes1.sroa.0.0.zext.i, 16843009, !dbg !22790
  %_5.i4102 = icmp eq i32 %left_phase.i412, %bytes1.sroa.0.0.isplat.i, !dbg !22791
  br i1 %_5.i4102, label %bb3.i4103, label %bb6.i4097, !dbg !22791

bb3.i4103:                                        ; preds = %bb2.i4101
  %bytes.sroa.0.0.extract.trunc.i = trunc i32 %left_phase.i412 to i8, !dbg !22792
  %651 = shl nuw nsw i32 %_237.1.i418, 2, !dbg !22794
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %_237.0.i417, i8 %bytes.sroa.0.0.extract.trunc.i, i32 %651, i1 false), !dbg !22794, !alias.scope !22795, !noalias !21469
  br label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, !dbg !22798

bb10.i4098:                                       ; preds = %bb6.i4097, %bb10.i4098
  %iter.sroa.0.04.i = phi ptr [ %_38.i4099, %bb10.i4098 ], [ %_237.0.i417, %bb6.i4097 ]
  %_38.i4099 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i, i32 4, !dbg !22799
  store i32 %left_phase.i412, ptr %iter.sroa.0.04.i, align 4, !dbg !22801, !alias.scope !22795, !noalias !21469
  %_29.i4100 = icmp eq ptr %_38.i4099, %end_or_len.i, !dbg !22786
  br i1 %_29.i4100, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i4098, !dbg !22789

_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit: ; preds = %bb10.i4098, %bb6.i4097, %bb3.i4103
  %652 = getelementptr inbounds nuw i8, ptr %self, i32 464, !dbg !22802
  %_238.0.i419 = load ptr, ptr %652, align 4, !dbg !22802, !alias.scope !21467, !noalias !22803, !nonnull !10, !noundef !10
  %653 = getelementptr inbounds nuw i8, ptr %self, i32 468, !dbg !22802
  %_238.1.i420 = load i32, ptr %653, align 4, !dbg !22802, !alias.scope !21467, !noalias !22803, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !22804), !dbg !22807
  %_4.not.i3146 = icmp eq i32 %_238.1.i420, 0, !dbg !22808
  br i1 %_4.not.i3146, label %panic.i3148, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3149, !dbg !22808

panic.i3148:                                      ; preds = %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #33, !dbg !22808, !noalias !22810
  unreachable, !dbg !22808

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3149: ; preds = %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit
  store float %right_prefix.i413, ptr %_238.0.i419, align 4, !dbg !22808, !alias.scope !22804, !noalias !21469
  %_239.0.i421 = load ptr, ptr %77, align 4, !dbg !22811, !alias.scope !21467, !noalias !22803, !nonnull !10, !noundef !10
  %_239.1.i422 = load i32, ptr %78, align 4, !dbg !22811, !alias.scope !21467, !noalias !22803, !noundef !10
  %654 = tail call i1 @llvm.is.constant.i32(i32 %right_phase.i414), !dbg !22812
  br i1 %654, label %bb2.i4112, label %bb6.i4104, !dbg !22812

bb6.i4104:                                        ; preds = %bb2.i4112, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3149
  %end_or_len.idx.i4105 = shl nuw nsw i32 %_239.1.i422, 2, !dbg !22815
  %end_or_len.i4106 = getelementptr inbounds nuw i8, ptr %_239.0.i421, i32 %end_or_len.idx.i4105, !dbg !22815
  %_293.i4107 = icmp eq i32 %_239.1.i422, 0, !dbg !22819
  br i1 %_293.i4107, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4118, label %bb10.i4108, !dbg !22822

bb2.i4112:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3149
  %bytes1.sroa.0.0.zext.i4113 = and i32 %right_phase.i414, 255, !dbg !22823
  %bytes1.sroa.0.0.isplat.i4114 = mul nuw i32 %bytes1.sroa.0.0.zext.i4113, 16843009, !dbg !22823
  %_5.i4115 = icmp eq i32 %right_phase.i414, %bytes1.sroa.0.0.isplat.i4114, !dbg !22824
  br i1 %_5.i4115, label %bb3.i4116, label %bb6.i4104, !dbg !22824

bb3.i4116:                                        ; preds = %bb2.i4112
  %bytes.sroa.0.0.extract.trunc.i4117 = trunc i32 %right_phase.i414 to i8, !dbg !22825
  %655 = shl nuw nsw i32 %_239.1.i422, 2, !dbg !22827
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %_239.0.i421, i8 %bytes.sroa.0.0.extract.trunc.i4117, i32 %655, i1 false), !dbg !22827, !alias.scope !22828, !noalias !21469
  br label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4118, !dbg !22831

bb10.i4108:                                       ; preds = %bb6.i4104, %bb10.i4108
  %iter.sroa.0.04.i4109 = phi ptr [ %_38.i4110, %bb10.i4108 ], [ %_239.0.i421, %bb6.i4104 ]
  %_38.i4110 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i4109, i32 4, !dbg !22832
  store i32 %right_phase.i414, ptr %iter.sroa.0.04.i4109, align 4, !dbg !22834, !alias.scope !22828, !noalias !21469
  %_29.i4111 = icmp eq ptr %_38.i4110, %end_or_len.i4106, !dbg !22819
  br i1 %_29.i4111, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4118, label %bb10.i4108, !dbg !22822

_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4118: ; preds = %bb10.i4108, %bb6.i4104, %bb3.i4116
  call void @llvm.lifetime.start.p0(ptr nonnull %_153.i16), !dbg !22835, !noalias !21475
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(92) %_153.i16, ptr noundef nonnull align 4 dereferenceable(92) %hot_left.i36, i32 92, i1 false), !dbg !22835, !noalias !21475
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_153.i16, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33) #32, !dbg !22836, !noalias !21469
  call void @llvm.lifetime.end.p0(ptr nonnull %_153.i16), !dbg !22837, !noalias !21475
  call void @llvm.lifetime.start.p0(ptr nonnull %_155.i15), !dbg !22838, !noalias !21475
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(92) %_155.i15, ptr noundef nonnull align 4 dereferenceable(92) %hot_right.i35, i32 92, i1 false), !dbg !22838, !noalias !21475
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_155.i15, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34) #32, !dbg !22839, !noalias !21469
  call void @llvm.lifetime.end.p0(ptr nonnull %_155.i15), !dbg !22840, !noalias !21475
  store i32 %main_cursor.sroa.0.0.i55.lcssa, ptr %_35, align 4, !dbg !22841, !alias.scope !21469, !noalias !21498
  store i32 %ring_cursor.sroa.0.0.i54.lcssa, ptr %530, align 4, !dbg !22842, !alias.scope !21469, !noalias !21498
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i33), !dbg !22843, !noalias !21475
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i34), !dbg !22844, !noalias !21475
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_right.i35), !dbg !22845, !noalias !21475
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i36), !dbg !22846, !noalias !21475
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockfEB2_.exit, !dbg !21462

bb6.i:                                            ; preds = %bb5.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !22847), !dbg !22850
  tail call void @llvm.experimental.noalias.scope.decl(metadata !22851), !dbg !22850
  tail call void @llvm.experimental.noalias.scope.decl(metadata !22853), !dbg !22850
  tail call void @llvm.experimental.noalias.scope.decl(metadata !22855), !dbg !22850
  tail call void @llvm.experimental.noalias.scope.decl(metadata !22857), !dbg !22850
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_left.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #32, !dbg !22859
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_right.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #32, !dbg !22863
  %656 = getelementptr inbounds nuw i8, ptr %self, i32 320, !dbg !22865
  %657 = load i8, ptr %656, align 4, !dbg !22865, !range !4765, !alias.scope !22847, !noalias !22869, !noundef !10
  %658 = getelementptr inbounds nuw i8, ptr %self, i32 321, !dbg !22872
  %659 = load i8, ptr %658, align 1, !dbg !22872, !range !4765, !alias.scope !22847, !noalias !22869, !noundef !10
  %660 = getelementptr inbounds nuw i8, ptr %self, i32 528, !dbg !22874
  %ring.i = load i32, ptr %660, align 4, !dbg !22874, !alias.scope !22851, !noalias !22876, !noundef !10
  %661 = getelementptr inbounds nuw i8, ptr %self, i32 532, !dbg !22877
  %main.i = load i32, ptr %661, align 4, !dbg !22877, !alias.scope !22851, !noalias !22876, !noundef !10
  %_36.i = load i32, ptr %_35, align 4, !dbg !22879, !alias.scope !22857, !noalias !22881, !noundef !10
  %662 = getelementptr inbounds nuw i8, ptr %self, i32 108, !dbg !22882
  %_37.i = load i32, ptr %662, align 4, !dbg !22882, !alias.scope !22857, !noalias !22881, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i), !dbg !22884, !noalias !22886
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i, i8 0, i32 1024, i1 false), !noalias !22886
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i), !dbg !22887, !noalias !22886
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i, i8 0, i32 1024, i1 false), !noalias !22886
  %_32.i = zext nneg i8 %657 to i32, !dbg !22865
  %.none.i = sub nsw i32 0, %_32.i, !dbg !22889
  %_33.i = zext nneg i8 %659 to i32, !dbg !22872
  %all.sroa.0.0.i = sub nsw i32 0, %_33.i, !dbg !22872
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i), !dbg !22890, !noalias !22886
; call <true_peak_limiter::UniformHot<f32>>::new
  call fastcc void @_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotfE3newB5_(ptr noalias noundef align 4 captures(none) dereferenceable(44) %uniform_left.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33, i32 %ring.i, i32 %main.i) #32, !dbg !22892
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_right.i), !dbg !22893, !noalias !22886
  %_32.val3840 = load i32, ptr %660, align 4, !dbg !22895, !noundef !10
  %_32.val3841 = load i32, ptr %661, align 4, !dbg !22895, !noundef !10
; call <true_peak_limiter::UniformHot<f32>>::new
  call fastcc void @_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotfE3newB5_(ptr noalias noundef align 4 captures(none) dereferenceable(44) %uniform_right.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34, i32 %_32.val3840, i32 %_32.val3841) #32, !dbg !22895
  %_162.not.i9190 = icmp eq i32 %frames, 0, !dbg !22896
  br i1 %_162.not.i9190, label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit, label %bb44.i.lr.ph, !dbg !22896

bb44.i.lr.ph:                                     ; preds = %bb6.i
  %d9.i4119 = lshr i32 %frames, 5, !dbg !22906
  %r2.i4120 = and i32 %frames, 31, !dbg !22913
  %_19.not.i4121 = icmp ne i32 %r2.i4120, 0, !dbg !22914
  %663 = zext i1 %_19.not.i4121 to i32, !dbg !22914
  %yield_count.sroa.0.0.i4122 = add nuw nsw i32 %d9.i4119, %663, !dbg !22914
  %history.i39.i.sroa.7.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 4
  %history.i39.i.sroa.10.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 8
  %history.i39.i.sroa.13.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 12
  %history.i39.i.sroa.16.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 16
  %history.i39.i.sroa.19.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 20
  %history.i39.i.sroa.22.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 24
  %history.i39.i.sroa.26.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 28
  %history.i39.i.sroa.29.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 32
  %history.i39.i.sroa.32.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 36
  %history.i39.i.sroa.35.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 40
  %history.i39.i.sroa.38.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 44
  %664 = getelementptr inbounds nuw i8, ptr %self, i32 132
  %665 = getelementptr inbounds nuw i8, ptr %self, i32 136
  %666 = getelementptr inbounds nuw i8, ptr %self, i32 140
  %row1.i.i.i75.i = getelementptr inbounds nuw i8, ptr %self, i32 144
  %667 = getelementptr inbounds nuw i8, ptr %self, i32 148
  %668 = getelementptr inbounds nuw i8, ptr %self, i32 152
  %669 = getelementptr inbounds nuw i8, ptr %self, i32 156
  %row3.i.i.i89.i = getelementptr inbounds nuw i8, ptr %self, i32 160
  %670 = getelementptr inbounds nuw i8, ptr %self, i32 164
  %671 = getelementptr inbounds nuw i8, ptr %self, i32 168
  %672 = getelementptr inbounds nuw i8, ptr %self, i32 172
  %row5.i.i.i103.i = getelementptr inbounds nuw i8, ptr %self, i32 176
  %673 = getelementptr inbounds nuw i8, ptr %self, i32 180
  %674 = getelementptr inbounds nuw i8, ptr %self, i32 184
  %675 = getelementptr inbounds nuw i8, ptr %self, i32 188
  %row7.i.i.i117.i = getelementptr inbounds nuw i8, ptr %self, i32 192
  %676 = getelementptr inbounds nuw i8, ptr %self, i32 196
  %677 = getelementptr inbounds nuw i8, ptr %self, i32 200
  %678 = getelementptr inbounds nuw i8, ptr %self, i32 204
  %row9.i.i.i131.i = getelementptr inbounds nuw i8, ptr %self, i32 208
  %679 = getelementptr inbounds nuw i8, ptr %self, i32 212
  %680 = getelementptr inbounds nuw i8, ptr %self, i32 216
  %681 = getelementptr inbounds nuw i8, ptr %self, i32 220
  %row11.i.i.i145.i = getelementptr inbounds nuw i8, ptr %self, i32 224
  %682 = getelementptr inbounds nuw i8, ptr %self, i32 228
  %683 = getelementptr inbounds nuw i8, ptr %self, i32 232
  %684 = getelementptr inbounds nuw i8, ptr %self, i32 236
  %row13.i.i.i159.i = getelementptr inbounds nuw i8, ptr %self, i32 240
  %685 = getelementptr inbounds nuw i8, ptr %self, i32 244
  %686 = getelementptr inbounds nuw i8, ptr %self, i32 248
  %687 = getelementptr inbounds nuw i8, ptr %self, i32 252
  %row15.i.i.i173.i = getelementptr inbounds nuw i8, ptr %self, i32 256
  %688 = getelementptr inbounds nuw i8, ptr %self, i32 260
  %689 = getelementptr inbounds nuw i8, ptr %self, i32 264
  %690 = getelementptr inbounds nuw i8, ptr %self, i32 268
  %row17.i.i.i187.i = getelementptr inbounds nuw i8, ptr %self, i32 272
  %691 = getelementptr inbounds nuw i8, ptr %self, i32 276
  %692 = getelementptr inbounds nuw i8, ptr %self, i32 280
  %693 = getelementptr inbounds nuw i8, ptr %self, i32 284
  %row19.i.i.i201.i = getelementptr inbounds nuw i8, ptr %self, i32 288
  %694 = getelementptr inbounds nuw i8, ptr %self, i32 292
  %695 = getelementptr inbounds nuw i8, ptr %self, i32 296
  %696 = getelementptr inbounds nuw i8, ptr %self, i32 300
  %row21.i.i.i215.i = getelementptr inbounds nuw i8, ptr %self, i32 304
  %697 = getelementptr inbounds nuw i8, ptr %self, i32 308
  %698 = getelementptr inbounds nuw i8, ptr %self, i32 312
  %699 = getelementptr inbounds nuw i8, ptr %self, i32 316
  %history.i.i.sroa.7.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 4
  %history.i.i.sroa.10.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 8
  %history.i.i.sroa.13.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 12
  %history.i.i.sroa.16.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 16
  %history.i.i.sroa.19.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 20
  %history.i.i.sroa.22.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 24
  %history.i.i.sroa.26.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 28
  %history.i.i.sroa.29.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 32
  %history.i.i.sroa.32.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 36
  %history.i.i.sroa.35.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 40
  %history.i.i.sroa.38.0.hot_right.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 44
  %700 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 32
  %_68.i.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 36
  %_68.i.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 40
  %701 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 32
  %_69.i.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 36
  %_69.i.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 40
  %_110.i = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 48
  %_111.i = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 64
  %_115.i = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 48
  %_116.i = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 64
  %_9.i3494 = add nsw i32 %_32.i, -1
  %702 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 4
  %_21.i270.i = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 24
  %_22.i271.i = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 28
  %703 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 8
  %704 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 12
  %705 = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 84
  %706 = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 88
  %707 = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 80
  %708 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 20
  %709 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 16
  %_9.i3474 = add nsw i32 %_33.i, -1
  %710 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 4
  %_21.i.i = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 24
  %_22.i.i = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 28
  %711 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 8
  %712 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 12
  %713 = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 84
  %714 = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 88
  %715 = getelementptr inbounds nuw i8, ptr %hot_right.i, i32 80
  %716 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 20
  %717 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 16
  %hot_left.i.promoted = load float, ptr %hot_left.i, align 4
  %history.i39.i.sroa.7.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4
  %history.i39.i.sroa.10.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4
  %history.i39.i.sroa.13.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4
  %history.i39.i.sroa.16.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4
  %history.i39.i.sroa.19.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4
  %history.i39.i.sroa.22.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4
  %history.i39.i.sroa.26.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4
  %history.i39.i.sroa.29.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4
  %history.i39.i.sroa.32.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4
  %history.i39.i.sroa.35.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4
  %history.i39.i.sroa.38.0.hot_left.i.sroa_idx.promoted = load float, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4
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
  %_22.i271.i.promoted = load i32, ptr %_22.i271.i, align 4
  %_21.i270.i.promoted = load float, ptr %_21.i270.i, align 4
  %_22.i.i.promoted = load i32, ptr %_22.i.i, align 4
  %_21.i.i.promoted = load float, ptr %_21.i.i, align 4
  %.promoted14974 = load float, ptr %705, align 4
  %.promoted14996 = load float, ptr %707, align 4
  %.promoted15018 = load float, ptr %713, align 4
  %.promoted15040 = load float, ptr %715, align 4
  br label %bb44.i, !dbg !22896

bb15.i.loopexit:                                  ; preds = %bb67.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i
  %_0.i3232.lcssa1223514393.lcssa15041 = phi float [ %_0.i3232.lcssa1223514393.lcssa15042, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %_0.i3232.lcssa1223514393, %bb67.i ]
  %_0.i2858.lcssa1223414375.lcssa15019 = phi float [ %_0.i2858.lcssa1223414375.lcssa15020, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %_0.i2858.lcssa1223414375, %bb67.i ]
  %_0.i3236.lcssa1221014357.lcssa14997 = phi float [ %_0.i3236.lcssa1221014357.lcssa14998, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %_0.i3236.lcssa1221014357, %bb67.i ]
  %_0.i2862.lcssa1219414339.lcssa14975 = phi float [ %_0.i2862.lcssa1219414339.lcssa14976, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %_0.i2862.lcssa1219414339, %bb67.i ]
  %running.sroa.0.0.i1210.lcssa89779155.lcssa14953 = phi float [ %running.sroa.0.0.i1210.lcssa89779155.lcssa14954, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %running.sroa.0.0.i1210.lcssa89779155, %bb67.i ]
  %storemerge.i1215.lcssa89509119.lcssa14932 = phi i32 [ %storemerge.i1215.lcssa89509119.lcssa14933, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %storemerge.i1215.lcssa89509119, %bb67.i ]
  %running.sroa.0.0.i1240.lcssa88659083.lcssa14911 = phi float [ %running.sroa.0.0.i1240.lcssa88659083.lcssa14912, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %running.sroa.0.0.i1240.lcssa88659083, %bb67.i ]
  %storemerge.i1245.lcssa88389047.lcssa14890 = phi i32 [ %storemerge.i1245.lcssa88389047.lcssa14891, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %storemerge.i1245.lcssa88389047, %bb67.i ]
  %ring_cursor.sroa.0.1.i.lcssa = phi i32 [ %ring_cursor.sroa.0.0.i9191, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %ring_cursor.sroa.0.2.i, %bb67.i ], !dbg !22915
  %main_cursor.sroa.0.1.i.lcssa = phi i32 [ %main_cursor.sroa.0.0.i9192, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %main_cursor.sroa.0.2.i, %bb67.i ], !dbg !22916
  %_162.not.i = icmp eq i32 %719, 0, !dbg !22896
  %indvars.iv.next11958 = add i32 %indvars.iv11957, -32, !dbg !22896
  br i1 %_162.not.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit, label %bb44.i, !dbg !22896

bb44.i:                                           ; preds = %bb44.i.lr.ph, %bb15.i.loopexit
  %_0.i3232.lcssa1223514393.lcssa15042 = phi float [ %.promoted15040, %bb44.i.lr.ph ], [ %_0.i3232.lcssa1223514393.lcssa15041, %bb15.i.loopexit ]
  %_0.i2858.lcssa1223414375.lcssa15020 = phi float [ %.promoted15018, %bb44.i.lr.ph ], [ %_0.i2858.lcssa1223414375.lcssa15019, %bb15.i.loopexit ]
  %_0.i3236.lcssa1221014357.lcssa14998 = phi float [ %.promoted14996, %bb44.i.lr.ph ], [ %_0.i3236.lcssa1221014357.lcssa14997, %bb15.i.loopexit ]
  %_0.i2862.lcssa1219414339.lcssa14976 = phi float [ %.promoted14974, %bb44.i.lr.ph ], [ %_0.i2862.lcssa1219414339.lcssa14975, %bb15.i.loopexit ]
  %running.sroa.0.0.i1210.lcssa89779155.lcssa14954 = phi float [ %_21.i.i.promoted, %bb44.i.lr.ph ], [ %running.sroa.0.0.i1210.lcssa89779155.lcssa14953, %bb15.i.loopexit ]
  %storemerge.i1215.lcssa89509119.lcssa14933 = phi i32 [ %_22.i.i.promoted, %bb44.i.lr.ph ], [ %storemerge.i1215.lcssa89509119.lcssa14932, %bb15.i.loopexit ]
  %running.sroa.0.0.i1240.lcssa88659083.lcssa14912 = phi float [ %_21.i270.i.promoted, %bb44.i.lr.ph ], [ %running.sroa.0.0.i1240.lcssa88659083.lcssa14911, %bb15.i.loopexit ]
  %storemerge.i1245.lcssa88389047.lcssa14891 = phi i32 [ %_22.i271.i.promoted, %bb44.i.lr.ph ], [ %storemerge.i1245.lcssa88389047.lcssa14890, %bb15.i.loopexit ]
  %history.i.i.sroa.38.0.lcssa14870 = phi float [ %history.i.i.sroa.38.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.38.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.35.0.lcssa14850 = phi float [ %history.i.i.sroa.35.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.35.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.32.0.lcssa14830 = phi float [ %history.i.i.sroa.32.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.32.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.29.0.lcssa14810 = phi float [ %history.i.i.sroa.29.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.29.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.26.0.lcssa14790 = phi float [ %history.i.i.sroa.26.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.26.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.22.0.lcssa14770 = phi float [ %history.i.i.sroa.22.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.22.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.19.0.lcssa14750 = phi float [ %history.i.i.sroa.19.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.19.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.16.0.lcssa14730 = phi float [ %history.i.i.sroa.16.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.16.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.13.0.lcssa14710 = phi float [ %history.i.i.sroa.13.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.13.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.10.0.lcssa14690 = phi float [ %history.i.i.sroa.10.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.10.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.7.0.lcssa14670 = phi float [ %history.i.i.sroa.7.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.7.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.0.0.lcssa14650 = phi float [ %hot_right.i.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i39.i.sroa.38.0.lcssa14630 = phi float [ %history.i39.i.sroa.38.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i39.i.sroa.38.0.lcssa, %bb15.i.loopexit ]
  %history.i39.i.sroa.35.0.lcssa14610 = phi float [ %history.i39.i.sroa.35.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i39.i.sroa.35.0.lcssa, %bb15.i.loopexit ]
  %history.i39.i.sroa.32.0.lcssa14590 = phi float [ %history.i39.i.sroa.32.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i39.i.sroa.32.0.lcssa, %bb15.i.loopexit ]
  %history.i39.i.sroa.29.0.lcssa14570 = phi float [ %history.i39.i.sroa.29.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i39.i.sroa.29.0.lcssa, %bb15.i.loopexit ]
  %history.i39.i.sroa.26.0.lcssa14550 = phi float [ %history.i39.i.sroa.26.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i39.i.sroa.26.0.lcssa, %bb15.i.loopexit ]
  %history.i39.i.sroa.22.0.lcssa14530 = phi float [ %history.i39.i.sroa.22.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i39.i.sroa.22.0.lcssa, %bb15.i.loopexit ]
  %history.i39.i.sroa.19.0.lcssa14510 = phi float [ %history.i39.i.sroa.19.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i39.i.sroa.19.0.lcssa, %bb15.i.loopexit ]
  %history.i39.i.sroa.16.0.lcssa14490 = phi float [ %history.i39.i.sroa.16.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i39.i.sroa.16.0.lcssa, %bb15.i.loopexit ]
  %history.i39.i.sroa.13.0.lcssa14470 = phi float [ %history.i39.i.sroa.13.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i39.i.sroa.13.0.lcssa, %bb15.i.loopexit ]
  %history.i39.i.sroa.10.0.lcssa14450 = phi float [ %history.i39.i.sroa.10.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i39.i.sroa.10.0.lcssa, %bb15.i.loopexit ]
  %history.i39.i.sroa.7.0.lcssa14430 = phi float [ %history.i39.i.sroa.7.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i39.i.sroa.7.0.lcssa, %bb15.i.loopexit ]
  %history.i39.i.sroa.0.0.lcssa14410 = phi float [ %hot_left.i.promoted, %bb44.i.lr.ph ], [ %history.i39.i.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %indvars.iv11957 = phi i32 [ %frames, %bb44.i.lr.ph ], [ %indvars.iv.next11958, %bb15.i.loopexit ]
  %iter2.sroa.0.0.i9194 = phi i32 [ %yield_count.sroa.0.0.i4122, %bb44.i.lr.ph ], [ %719, %bb15.i.loopexit ]
  %iter1.sroa.0.0.i9193 = phi i32 [ 0, %bb44.i.lr.ph ], [ %718, %bb15.i.loopexit ]
  %main_cursor.sroa.0.0.i9192 = phi i32 [ %_36.i, %bb44.i.lr.ph ], [ %main_cursor.sroa.0.1.i.lcssa, %bb15.i.loopexit ]
  %ring_cursor.sroa.0.0.i9191 = phi i32 [ %_37.i, %bb44.i.lr.ph ], [ %ring_cursor.sroa.0.1.i.lcssa, %bb15.i.loopexit ]
  %umin11977 = call i32 @llvm.umin.i32(i32 %indvars.iv11957, i32 32), !dbg !22917
  %umax11963 = call i32 @llvm.umax.i32(i32 %umin11977, i32 1), !dbg !22917
  %718 = add i32 %iter1.sroa.0.0.i9193, 32, !dbg !22917
  %719 = add nsw i32 %iter2.sroa.0.0.i9194, -1, !dbg !22921
  %720 = sub i32 %frames, %iter1.sroa.0.0.i9193, !dbg !22922
  %spec.store.select.i = tail call i32 @llvm.umin.i32(i32 %720, i32 32), !dbg !22924
  %_20.i42.i8753.not = icmp eq i32 %frames, %iter1.sroa.0.0.i9193, !dbg !22929
  br i1 %_20.i42.i8753.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i, label %bb5.i43.i.lr.ph, !dbg !22935

bb5.i43.i.lr.ph:                                  ; preds = %bb44.i
  %_11.i.i.i63.i = load float, ptr %_31, align 4
  %_14.i.i.i66.i = load float, ptr %664, align 4
  %_17.i.i.i69.i = load float, ptr %665, align 4
  %_20.i.i.i72.i = load float, ptr %666, align 4
  %_25.i.i.i77.i = load float, ptr %row1.i.i.i75.i, align 4
  %_28.i.i.i80.i = load float, ptr %667, align 4
  %_31.i.i.i83.i = load float, ptr %668, align 4
  %_34.i.i.i86.i = load float, ptr %669, align 4
  %_39.i.i.i91.i = load float, ptr %row3.i.i.i89.i, align 4
  %_42.i.i.i94.i = load float, ptr %670, align 4
  %_45.i.i.i97.i = load float, ptr %671, align 4
  %_48.i.i.i100.i = load float, ptr %672, align 4
  %_53.i.i.i105.i = load float, ptr %row5.i.i.i103.i, align 4
  %_56.i.i.i108.i = load float, ptr %673, align 4
  %_59.i.i.i111.i = load float, ptr %674, align 4
  %_62.i.i.i114.i = load float, ptr %675, align 4
  %_67.i.i.i119.i = load float, ptr %row7.i.i.i117.i, align 4
  %_70.i.i.i122.i = load float, ptr %676, align 4
  %_73.i.i.i125.i = load float, ptr %677, align 4
  %_76.i.i.i128.i = load float, ptr %678, align 4
  %_81.i.i.i133.i = load float, ptr %row9.i.i.i131.i, align 4
  %_84.i.i.i136.i = load float, ptr %679, align 4
  %_87.i.i.i139.i = load float, ptr %680, align 4
  %_90.i.i.i142.i = load float, ptr %681, align 4
  %_95.i.i.i147.i = load float, ptr %row11.i.i.i145.i, align 4
  %_98.i.i.i150.i = load float, ptr %682, align 4
  %_101.i.i.i153.i = load float, ptr %683, align 4
  %_104.i.i.i156.i = load float, ptr %684, align 4
  %_109.i.i.i161.i = load float, ptr %row13.i.i.i159.i, align 4
  %_112.i.i.i164.i = load float, ptr %685, align 4
  %_115.i.i.i167.i = load float, ptr %686, align 4
  %_118.i.i.i170.i = load float, ptr %687, align 4
  %_123.i.i.i175.i = load float, ptr %row15.i.i.i173.i, align 4
  %_126.i.i.i178.i = load float, ptr %688, align 4
  %_129.i.i.i181.i = load float, ptr %689, align 4
  %_132.i.i.i184.i = load float, ptr %690, align 4
  %_137.i.i.i189.i = load float, ptr %row17.i.i.i187.i, align 4
  %_140.i.i.i192.i = load float, ptr %691, align 4
  %_143.i.i.i195.i = load float, ptr %692, align 4
  %_146.i.i.i198.i = load float, ptr %693, align 4
  %_151.i.i.i203.i = load float, ptr %row19.i.i.i201.i, align 4
  %_154.i.i.i206.i = load float, ptr %694, align 4
  %_157.i.i.i209.i = load float, ptr %695, align 4
  %_160.i.i.i212.i = load float, ptr %696, align 4
  %_165.i.i.i217.i = load float, ptr %row21.i.i.i215.i, align 4
  %_168.i.i.i220.i = load float, ptr %697, align 4
  %_171.i.i.i223.i = load float, ptr %698, align 4
  %_174.i.i.i226.i = load float, ptr %699, align 4
  br label %bb5.i43.i, !dbg !22935

bb5.i43.i:                                        ; preds = %bb5.i43.i.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038
  %iter.sroa.0.0.i41.i8765 = phi i32 [ 0, %bb5.i43.i.lr.ph ], [ %721, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ]
  %history.i39.i.sroa.35.08764 = phi float [ %history.i39.i.sroa.35.0.lcssa14610, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.32.08763, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ]
  %history.i39.i.sroa.32.08763 = phi float [ %history.i39.i.sroa.32.0.lcssa14590, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.29.08762, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ]
  %history.i39.i.sroa.29.08762 = phi float [ %history.i39.i.sroa.29.0.lcssa14570, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.26.08761, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ]
  %history.i39.i.sroa.26.08761 = phi float [ %history.i39.i.sroa.26.0.lcssa14550, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.22.08760, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ]
  %history.i39.i.sroa.22.08760 = phi float [ %history.i39.i.sroa.22.0.lcssa14530, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.19.08759, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ]
  %history.i39.i.sroa.19.08759 = phi float [ %history.i39.i.sroa.19.0.lcssa14510, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.16.08758, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ]
  %history.i39.i.sroa.16.08758 = phi float [ %history.i39.i.sroa.16.0.lcssa14490, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.13.08757, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ]
  %history.i39.i.sroa.13.08757 = phi float [ %history.i39.i.sroa.13.0.lcssa14470, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.10.08756, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ]
  %history.i39.i.sroa.10.08756 = phi float [ %history.i39.i.sroa.10.0.lcssa14450, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.7.08755, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ]
  %history.i39.i.sroa.7.08755 = phi float [ %history.i39.i.sroa.7.0.lcssa14430, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.0.08754, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ]
  %history.i39.i.sroa.0.08754 = phi float [ %history.i39.i.sroa.0.0.lcssa14410, %bb5.i43.i.lr.ph ], [ %_0.i3036, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ]
  %721 = add nuw nsw i32 %iter.sroa.0.0.i41.i8765, 1, !dbg !22936
  %_11.i44.i = add nuw nsw i32 %iter.sroa.0.0.i41.i8765, %iter1.sroa.0.0.i9193, !dbg !22939
  %_24.i45.i = icmp ugt i32 %_11.i44.i, %left_io.1, !dbg !22940
  br i1 %_24.i45.i, label %bb7.i242.i, label %bb8.i46.i, !dbg !22940, !prof !902

bb8.i46.i:                                        ; preds = %bb5.i43.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !22943), !dbg !22946
  %_3.not.i3034 = icmp eq i32 %left_io.1, %_11.i44.i, !dbg !22947
  br i1 %_3.not.i3034, label %panic.i3037, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038, !dbg !22947

panic.i3037:                                      ; preds = %bb8.i46.i
  store float %history.i39.i.sroa.0.0.lcssa14410, ptr %hot_left.i, align 4, !dbg !22949
  store float %history.i39.i.sroa.7.0.lcssa14430, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.10.0.lcssa14450, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.13.0.lcssa14470, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.16.0.lcssa14490, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.19.0.lcssa14510, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.22.0.lcssa14530, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.26.0.lcssa14550, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.29.0.lcssa14570, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.32.0.lcssa14590, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.35.0.lcssa14610, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.38.0.lcssa14630, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i.i.sroa.0.0.lcssa14650, ptr %hot_right.i, align 4, !dbg !22950
  store float %history.i.i.sroa.7.0.lcssa14670, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.10.0.lcssa14690, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.13.0.lcssa14710, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.16.0.lcssa14730, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.19.0.lcssa14750, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.22.0.lcssa14770, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.26.0.lcssa14790, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.29.0.lcssa14810, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.32.0.lcssa14830, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.35.0.lcssa14850, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.38.0.lcssa14870, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store i32 %storemerge.i1245.lcssa88389047.lcssa14891, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88659083.lcssa14912, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89509119.lcssa14933, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89779155.lcssa14954, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219414339.lcssa14976, ptr %705, align 4
  store float %_0.i3236.lcssa1221014357.lcssa14998, ptr %707, align 4
  store float %_0.i2858.lcssa1223414375.lcssa15020, ptr %713, align 4
  store float %_0.i3232.lcssa1223514393.lcssa15042, ptr %715, align 4
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #33, !dbg !22947, !noalias !22952
  unreachable, !dbg !22947

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038: ; preds = %bb8.i46.i
  %_31.i48.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %_11.i44.i, !dbg !22956
  %_0.i3036 = load float, ptr %_31.i48.i, align 4, !dbg !22947, !alias.scope !22943, !noalias !22958, !noundef !10
  %722 = tail call noundef float @llvm.fabs.f32(float %history.i39.i.sroa.19.08759), !dbg !22959
  %_0.i2768 = fmul float %_0.i3036, %_11.i.i.i63.i, !dbg !22962
  %_0.i2332 = fadd float %_0.i2768, 0.000000e+00, !dbg !22965
  %_0.i2767 = fmul float %_0.i3036, %_14.i.i.i66.i, !dbg !22967
  %_0.i2331 = fadd float %_0.i2767, 0.000000e+00, !dbg !22969
  %_0.i2766 = fmul float %_0.i3036, %_17.i.i.i69.i, !dbg !22971
  %_0.i2330 = fadd float %_0.i2766, 0.000000e+00, !dbg !22973
  %_0.i2765 = fmul float %_0.i3036, %_20.i.i.i72.i, !dbg !22975
  %_0.i2329 = fadd float %_0.i2765, 0.000000e+00, !dbg !22977
  %_0.i2764 = fmul float %history.i39.i.sroa.0.08754, %_25.i.i.i77.i, !dbg !22979
  %_0.i2328 = fadd float %_0.i2332, %_0.i2764, !dbg !22981
  %_0.i2763 = fmul float %history.i39.i.sroa.0.08754, %_28.i.i.i80.i, !dbg !22983
  %_0.i2327 = fadd float %_0.i2331, %_0.i2763, !dbg !22985
  %_0.i2762 = fmul float %history.i39.i.sroa.0.08754, %_31.i.i.i83.i, !dbg !22987
  %_0.i2326 = fadd float %_0.i2330, %_0.i2762, !dbg !22989
  %_0.i2761 = fmul float %history.i39.i.sroa.0.08754, %_34.i.i.i86.i, !dbg !22991
  %_0.i2325 = fadd float %_0.i2329, %_0.i2761, !dbg !22993
  %_0.i2760 = fmul float %history.i39.i.sroa.7.08755, %_39.i.i.i91.i, !dbg !22995
  %_0.i2324 = fadd float %_0.i2328, %_0.i2760, !dbg !22997
  %_0.i2759 = fmul float %history.i39.i.sroa.7.08755, %_42.i.i.i94.i, !dbg !22999
  %_0.i2323 = fadd float %_0.i2327, %_0.i2759, !dbg !23001
  %_0.i2758 = fmul float %history.i39.i.sroa.7.08755, %_45.i.i.i97.i, !dbg !23003
  %_0.i2322 = fadd float %_0.i2326, %_0.i2758, !dbg !23005
  %_0.i2757 = fmul float %history.i39.i.sroa.7.08755, %_48.i.i.i100.i, !dbg !23007
  %_0.i2321 = fadd float %_0.i2325, %_0.i2757, !dbg !23009
  %_0.i2756 = fmul float %history.i39.i.sroa.10.08756, %_53.i.i.i105.i, !dbg !23011
  %_0.i2320 = fadd float %_0.i2324, %_0.i2756, !dbg !23013
  %_0.i2755 = fmul float %history.i39.i.sroa.10.08756, %_56.i.i.i108.i, !dbg !23015
  %_0.i2319 = fadd float %_0.i2323, %_0.i2755, !dbg !23017
  %_0.i2754 = fmul float %history.i39.i.sroa.10.08756, %_59.i.i.i111.i, !dbg !23019
  %_0.i2318 = fadd float %_0.i2322, %_0.i2754, !dbg !23021
  %_0.i2753 = fmul float %history.i39.i.sroa.10.08756, %_62.i.i.i114.i, !dbg !23023
  %_0.i2317 = fadd float %_0.i2321, %_0.i2753, !dbg !23025
  %_0.i2752 = fmul float %history.i39.i.sroa.13.08757, %_67.i.i.i119.i, !dbg !23027
  %_0.i2316 = fadd float %_0.i2320, %_0.i2752, !dbg !23029
  %_0.i2751 = fmul float %history.i39.i.sroa.13.08757, %_70.i.i.i122.i, !dbg !23031
  %_0.i2315 = fadd float %_0.i2319, %_0.i2751, !dbg !23033
  %_0.i2750 = fmul float %history.i39.i.sroa.13.08757, %_73.i.i.i125.i, !dbg !23035
  %_0.i2314 = fadd float %_0.i2318, %_0.i2750, !dbg !23037
  %_0.i2749 = fmul float %history.i39.i.sroa.13.08757, %_76.i.i.i128.i, !dbg !23039
  %_0.i2313 = fadd float %_0.i2317, %_0.i2749, !dbg !23041
  %_0.i2748 = fmul float %history.i39.i.sroa.16.08758, %_81.i.i.i133.i, !dbg !23043
  %_0.i2312 = fadd float %_0.i2316, %_0.i2748, !dbg !23045
  %_0.i2747 = fmul float %history.i39.i.sroa.16.08758, %_84.i.i.i136.i, !dbg !23047
  %_0.i2311 = fadd float %_0.i2315, %_0.i2747, !dbg !23049
  %_0.i2746 = fmul float %history.i39.i.sroa.16.08758, %_87.i.i.i139.i, !dbg !23051
  %_0.i2310 = fadd float %_0.i2314, %_0.i2746, !dbg !23053
  %_0.i2745 = fmul float %history.i39.i.sroa.16.08758, %_90.i.i.i142.i, !dbg !23055
  %_0.i2309 = fadd float %_0.i2313, %_0.i2745, !dbg !23057
  %_0.i2744 = fmul float %history.i39.i.sroa.19.08759, %_95.i.i.i147.i, !dbg !23059
  %_0.i2308 = fadd float %_0.i2312, %_0.i2744, !dbg !23061
  %_0.i2743 = fmul float %history.i39.i.sroa.19.08759, %_98.i.i.i150.i, !dbg !23063
  %_0.i2307 = fadd float %_0.i2311, %_0.i2743, !dbg !23065
  %_0.i2742 = fmul float %history.i39.i.sroa.19.08759, %_101.i.i.i153.i, !dbg !23067
  %_0.i2306 = fadd float %_0.i2310, %_0.i2742, !dbg !23069
  %_0.i2741 = fmul float %history.i39.i.sroa.19.08759, %_104.i.i.i156.i, !dbg !23071
  %_0.i2305 = fadd float %_0.i2309, %_0.i2741, !dbg !23073
  %_0.i2740 = fmul float %history.i39.i.sroa.22.08760, %_109.i.i.i161.i, !dbg !23075
  %_0.i2304 = fadd float %_0.i2308, %_0.i2740, !dbg !23077
  %_0.i2739 = fmul float %history.i39.i.sroa.22.08760, %_112.i.i.i164.i, !dbg !23079
  %_0.i2303 = fadd float %_0.i2307, %_0.i2739, !dbg !23081
  %_0.i2738 = fmul float %history.i39.i.sroa.22.08760, %_115.i.i.i167.i, !dbg !23083
  %_0.i2302 = fadd float %_0.i2306, %_0.i2738, !dbg !23085
  %_0.i2737 = fmul float %history.i39.i.sroa.22.08760, %_118.i.i.i170.i, !dbg !23087
  %_0.i2301 = fadd float %_0.i2305, %_0.i2737, !dbg !23089
  %_0.i2736 = fmul float %history.i39.i.sroa.26.08761, %_123.i.i.i175.i, !dbg !23091
  %_0.i2300 = fadd float %_0.i2304, %_0.i2736, !dbg !23093
  %_0.i2735 = fmul float %history.i39.i.sroa.26.08761, %_126.i.i.i178.i, !dbg !23095
  %_0.i2299 = fadd float %_0.i2303, %_0.i2735, !dbg !23097
  %_0.i2734 = fmul float %history.i39.i.sroa.26.08761, %_129.i.i.i181.i, !dbg !23099
  %_0.i2298 = fadd float %_0.i2302, %_0.i2734, !dbg !23101
  %_0.i2733 = fmul float %history.i39.i.sroa.26.08761, %_132.i.i.i184.i, !dbg !23103
  %_0.i2297 = fadd float %_0.i2301, %_0.i2733, !dbg !23105
  %_0.i2732 = fmul float %history.i39.i.sroa.29.08762, %_137.i.i.i189.i, !dbg !23107
  %_0.i2296 = fadd float %_0.i2300, %_0.i2732, !dbg !23109
  %_0.i2731 = fmul float %history.i39.i.sroa.29.08762, %_140.i.i.i192.i, !dbg !23111
  %_0.i2295 = fadd float %_0.i2299, %_0.i2731, !dbg !23113
  %_0.i2730 = fmul float %history.i39.i.sroa.29.08762, %_143.i.i.i195.i, !dbg !23115
  %_0.i2294 = fadd float %_0.i2298, %_0.i2730, !dbg !23117
  %_0.i2729 = fmul float %history.i39.i.sroa.29.08762, %_146.i.i.i198.i, !dbg !23119
  %_0.i2293 = fadd float %_0.i2297, %_0.i2729, !dbg !23121
  %_0.i2728 = fmul float %history.i39.i.sroa.32.08763, %_151.i.i.i203.i, !dbg !23123
  %_0.i2292 = fadd float %_0.i2296, %_0.i2728, !dbg !23125
  %_0.i2727 = fmul float %history.i39.i.sroa.32.08763, %_154.i.i.i206.i, !dbg !23127
  %_0.i2291 = fadd float %_0.i2295, %_0.i2727, !dbg !23129
  %_0.i2726 = fmul float %history.i39.i.sroa.32.08763, %_157.i.i.i209.i, !dbg !23131
  %_0.i2290 = fadd float %_0.i2294, %_0.i2726, !dbg !23133
  %_0.i2725 = fmul float %history.i39.i.sroa.32.08763, %_160.i.i.i212.i, !dbg !23135
  %_0.i2289 = fadd float %_0.i2293, %_0.i2725, !dbg !23137
  %_0.i2724 = fmul float %history.i39.i.sroa.35.08764, %_165.i.i.i217.i, !dbg !23139
  %_0.i2288 = fadd float %_0.i2292, %_0.i2724, !dbg !23141
  %_0.i2723 = fmul float %history.i39.i.sroa.35.08764, %_168.i.i.i220.i, !dbg !23143
  %_0.i2287 = fadd float %_0.i2291, %_0.i2723, !dbg !23145
  %_0.i2722 = fmul float %history.i39.i.sroa.35.08764, %_171.i.i.i223.i, !dbg !23147
  %_0.i2286 = fadd float %_0.i2290, %_0.i2722, !dbg !23149
  %_0.i2721 = fmul float %history.i39.i.sroa.35.08764, %_174.i.i.i226.i, !dbg !23151
  %_0.i2285 = fadd float %_0.i2289, %_0.i2721, !dbg !23153
  %723 = tail call noundef float @llvm.fabs.f32(float %_0.i2288), !dbg !23155
  %_3.i.i3683.inv = fcmp ogt float %722, %723, !dbg !23157
  %_4.i.i3690.v = select i1 %_3.i.i3683.inv, float %722, float %723, !dbg !23157
  %724 = tail call noundef float @llvm.fabs.f32(float %_0.i2287), !dbg !23155
  %_3.i.i3683.inv.1 = fcmp ogt float %_4.i.i3690.v, %724, !dbg !23157
  %_4.i.i3690.v.1 = select i1 %_3.i.i3683.inv.1, float %_4.i.i3690.v, float %724, !dbg !23157
  %725 = tail call noundef float @llvm.fabs.f32(float %_0.i2286), !dbg !23155
  %_3.i.i3683.inv.2 = fcmp ogt float %_4.i.i3690.v.1, %725, !dbg !23157
  %_4.i.i3690.v.2 = select i1 %_3.i.i3683.inv.2, float %_4.i.i3690.v.1, float %725, !dbg !23157
  %726 = tail call noundef float @llvm.fabs.f32(float %_0.i2285), !dbg !23155
  %_3.i.i3683.inv.3 = fcmp ogt float %_4.i.i3690.v.2, %726, !dbg !23157
  %_4.i.i3690.v.3 = select i1 %_3.i.i3683.inv.3, float %_4.i.i3690.v.2, float %726, !dbg !23157
  %_39.i237.i = getelementptr inbounds nuw float, ptr %peaks_left.i, i32 %iter.sroa.0.0.i41.i8765, !dbg !23160
  store float %_4.i.i3690.v.3, ptr %_39.i237.i, align 4, !dbg !23165, !alias.scope !23167, !noalias !22958
  %exitcond11961.not = icmp eq i32 %721, %umax11963, !dbg !22929
  br i1 %exitcond11961.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i, label %bb5.i43.i, !dbg !22935

bb7.i242.i:                                       ; preds = %bb5.i43.i
  store float %history.i39.i.sroa.0.0.lcssa14410, ptr %hot_left.i, align 4, !dbg !22949
  store float %history.i39.i.sroa.7.0.lcssa14430, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.10.0.lcssa14450, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.13.0.lcssa14470, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.16.0.lcssa14490, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.19.0.lcssa14510, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.22.0.lcssa14530, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.26.0.lcssa14550, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.29.0.lcssa14570, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.32.0.lcssa14590, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.35.0.lcssa14610, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.38.0.lcssa14630, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i.i.sroa.0.0.lcssa14650, ptr %hot_right.i, align 4, !dbg !22950
  store float %history.i.i.sroa.7.0.lcssa14670, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.10.0.lcssa14690, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.13.0.lcssa14710, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.16.0.lcssa14730, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.19.0.lcssa14750, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.22.0.lcssa14770, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.26.0.lcssa14790, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.29.0.lcssa14810, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.32.0.lcssa14830, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.35.0.lcssa14850, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.38.0.lcssa14870, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store i32 %storemerge.i1245.lcssa88389047.lcssa14891, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88659083.lcssa14912, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89509119.lcssa14933, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89779155.lcssa14954, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219414339.lcssa14976, ptr %705, align 4
  store float %_0.i3236.lcssa1221014357.lcssa14998, ptr %707, align 4
  store float %_0.i2858.lcssa1223414375.lcssa15020, ptr %713, align 4
  store float %_0.i3232.lcssa1223514393.lcssa15042, ptr %715, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_11.i44.i, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #33, !dbg !23170, !noalias !22958
  unreachable, !dbg !23170

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038, %bb44.i
  %history.i39.i.sroa.0.0.lcssa = phi float [ %history.i39.i.sroa.0.0.lcssa14410, %bb44.i ], [ %_0.i3036, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ], !dbg !22949
  %history.i39.i.sroa.7.0.lcssa = phi float [ %history.i39.i.sroa.7.0.lcssa14430, %bb44.i ], [ %history.i39.i.sroa.0.08754, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ], !dbg !22949
  %history.i39.i.sroa.10.0.lcssa = phi float [ %history.i39.i.sroa.10.0.lcssa14450, %bb44.i ], [ %history.i39.i.sroa.7.08755, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ], !dbg !22949
  %history.i39.i.sroa.13.0.lcssa = phi float [ %history.i39.i.sroa.13.0.lcssa14470, %bb44.i ], [ %history.i39.i.sroa.10.08756, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ], !dbg !22949
  %history.i39.i.sroa.16.0.lcssa = phi float [ %history.i39.i.sroa.16.0.lcssa14490, %bb44.i ], [ %history.i39.i.sroa.13.08757, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ], !dbg !22949
  %history.i39.i.sroa.19.0.lcssa = phi float [ %history.i39.i.sroa.19.0.lcssa14510, %bb44.i ], [ %history.i39.i.sroa.16.08758, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ], !dbg !22949
  %history.i39.i.sroa.22.0.lcssa = phi float [ %history.i39.i.sroa.22.0.lcssa14530, %bb44.i ], [ %history.i39.i.sroa.19.08759, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ], !dbg !22949
  %history.i39.i.sroa.26.0.lcssa = phi float [ %history.i39.i.sroa.26.0.lcssa14550, %bb44.i ], [ %history.i39.i.sroa.22.08760, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ], !dbg !22949
  %history.i39.i.sroa.29.0.lcssa = phi float [ %history.i39.i.sroa.29.0.lcssa14570, %bb44.i ], [ %history.i39.i.sroa.26.08761, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ], !dbg !22949
  %history.i39.i.sroa.32.0.lcssa = phi float [ %history.i39.i.sroa.32.0.lcssa14590, %bb44.i ], [ %history.i39.i.sroa.29.08762, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ], !dbg !22949
  %history.i39.i.sroa.35.0.lcssa = phi float [ %history.i39.i.sroa.35.0.lcssa14610, %bb44.i ], [ %history.i39.i.sroa.32.08763, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ], !dbg !22949
  %history.i39.i.sroa.38.0.lcssa = phi float [ %history.i39.i.sroa.38.0.lcssa14630, %bb44.i ], [ %history.i39.i.sroa.35.08764, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ], !dbg !22949
  br i1 %_20.i42.i8753.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i, label %bb5.i.i.lr.ph, !dbg !23171

bb5.i.i.lr.ph:                                    ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i
  %_11.i.i.i.i = load float, ptr %_31, align 4
  %_14.i.i.i.i = load float, ptr %664, align 4
  %_17.i.i.i.i = load float, ptr %665, align 4
  %_20.i.i.i.i = load float, ptr %666, align 4
  %_25.i.i.i.i = load float, ptr %row1.i.i.i75.i, align 4
  %_28.i.i.i.i = load float, ptr %667, align 4
  %_31.i.i.i.i = load float, ptr %668, align 4
  %_34.i.i.i.i = load float, ptr %669, align 4
  %_39.i.i.i.i = load float, ptr %row3.i.i.i89.i, align 4
  %_42.i.i.i.i = load float, ptr %670, align 4
  %_45.i.i.i.i = load float, ptr %671, align 4
  %_48.i.i.i.i = load float, ptr %672, align 4
  %_53.i.i.i.i = load float, ptr %row5.i.i.i103.i, align 4
  %_56.i.i.i.i = load float, ptr %673, align 4
  %_59.i.i.i.i = load float, ptr %674, align 4
  %_62.i.i.i.i = load float, ptr %675, align 4
  %_67.i.i.i.i = load float, ptr %row7.i.i.i117.i, align 4
  %_70.i.i.i.i = load float, ptr %676, align 4
  %_73.i.i.i.i = load float, ptr %677, align 4
  %_76.i.i.i.i = load float, ptr %678, align 4
  %_81.i.i.i.i = load float, ptr %row9.i.i.i131.i, align 4
  %_84.i.i.i.i = load float, ptr %679, align 4
  %_87.i.i.i.i = load float, ptr %680, align 4
  %_90.i.i.i.i = load float, ptr %681, align 4
  %_95.i.i.i.i = load float, ptr %row11.i.i.i145.i, align 4
  %_98.i.i.i.i = load float, ptr %682, align 4
  %_101.i.i.i.i = load float, ptr %683, align 4
  %_104.i.i.i.i = load float, ptr %684, align 4
  %_109.i.i.i.i = load float, ptr %row13.i.i.i159.i, align 4
  %_112.i.i.i.i = load float, ptr %685, align 4
  %_115.i.i.i.i = load float, ptr %686, align 4
  %_118.i.i.i.i = load float, ptr %687, align 4
  %_123.i.i.i.i = load float, ptr %row15.i.i.i173.i, align 4
  %_126.i.i.i.i = load float, ptr %688, align 4
  %_129.i.i.i.i = load float, ptr %689, align 4
  %_132.i.i.i.i = load float, ptr %690, align 4
  %_137.i.i.i.i = load float, ptr %row17.i.i.i187.i, align 4
  %_140.i.i.i.i = load float, ptr %691, align 4
  %_143.i.i.i.i = load float, ptr %692, align 4
  %_146.i.i.i.i = load float, ptr %693, align 4
  %_151.i.i.i.i = load float, ptr %row19.i.i.i201.i, align 4
  %_154.i.i.i.i = load float, ptr %694, align 4
  %_157.i.i.i.i = load float, ptr %695, align 4
  %_160.i.i.i.i = load float, ptr %696, align 4
  %_165.i.i.i.i = load float, ptr %row21.i.i.i215.i, align 4
  %_168.i.i.i.i = load float, ptr %697, align 4
  %_171.i.i.i.i = load float, ptr %698, align 4
  %_174.i.i.i.i = load float, ptr %699, align 4
  br label %bb5.i.i, !dbg !23171

bb5.i.i:                                          ; preds = %bb5.i.i.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043
  %iter.sroa.0.0.i.i8792 = phi i32 [ 0, %bb5.i.i.lr.ph ], [ %727, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ]
  %history.i.i.sroa.35.08791 = phi float [ %history.i.i.sroa.35.0.lcssa14850, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.32.08790, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ]
  %history.i.i.sroa.32.08790 = phi float [ %history.i.i.sroa.32.0.lcssa14830, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.29.08789, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ]
  %history.i.i.sroa.29.08789 = phi float [ %history.i.i.sroa.29.0.lcssa14810, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.26.08788, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ]
  %history.i.i.sroa.26.08788 = phi float [ %history.i.i.sroa.26.0.lcssa14790, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.22.08787, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ]
  %history.i.i.sroa.22.08787 = phi float [ %history.i.i.sroa.22.0.lcssa14770, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.19.08786, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ]
  %history.i.i.sroa.19.08786 = phi float [ %history.i.i.sroa.19.0.lcssa14750, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.16.08785, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ]
  %history.i.i.sroa.16.08785 = phi float [ %history.i.i.sroa.16.0.lcssa14730, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.13.08784, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ]
  %history.i.i.sroa.13.08784 = phi float [ %history.i.i.sroa.13.0.lcssa14710, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.10.08783, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ]
  %history.i.i.sroa.10.08783 = phi float [ %history.i.i.sroa.10.0.lcssa14690, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.7.08782, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ]
  %history.i.i.sroa.7.08782 = phi float [ %history.i.i.sroa.7.0.lcssa14670, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.0.08781, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ]
  %history.i.i.sroa.0.08781 = phi float [ %history.i.i.sroa.0.0.lcssa14650, %bb5.i.i.lr.ph ], [ %_0.i3041, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ]
  %727 = add nuw nsw i32 %iter.sroa.0.0.i.i8792, 1, !dbg !23174
  %_11.i.i = add nuw nsw i32 %iter.sroa.0.0.i.i8792, %iter1.sroa.0.0.i9193, !dbg !23177
  %_24.i.i = icmp ugt i32 %_11.i.i, %right_io.1, !dbg !23178
  br i1 %_24.i.i, label %bb7.i.i, label %bb8.i.i, !dbg !23178, !prof !902

bb8.i.i:                                          ; preds = %bb5.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !23181), !dbg !23184
  %_3.not.i3039 = icmp eq i32 %right_io.1, %_11.i.i, !dbg !23185
  br i1 %_3.not.i3039, label %panic.i3042, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043, !dbg !23185

panic.i3042:                                      ; preds = %bb8.i.i
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !22949
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i.i.sroa.0.0.lcssa14650, ptr %hot_right.i, align 4, !dbg !22950
  store float %history.i.i.sroa.7.0.lcssa14670, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.10.0.lcssa14690, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.13.0.lcssa14710, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.16.0.lcssa14730, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.19.0.lcssa14750, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.22.0.lcssa14770, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.26.0.lcssa14790, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.29.0.lcssa14810, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.32.0.lcssa14830, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.35.0.lcssa14850, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.38.0.lcssa14870, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store i32 %storemerge.i1245.lcssa88389047.lcssa14891, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88659083.lcssa14912, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89509119.lcssa14933, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89779155.lcssa14954, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219414339.lcssa14976, ptr %705, align 4
  store float %_0.i3236.lcssa1221014357.lcssa14998, ptr %707, align 4
  store float %_0.i2858.lcssa1223414375.lcssa15020, ptr %713, align 4
  store float %_0.i3232.lcssa1223514393.lcssa15042, ptr %715, align 4
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #33, !dbg !23185, !noalias !23187
  unreachable, !dbg !23185

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043: ; preds = %bb8.i.i
  %_31.i.i = getelementptr inbounds nuw float, ptr %right_io.0, i32 %_11.i.i, !dbg !23191
  %_0.i3041 = load float, ptr %_31.i.i, align 4, !dbg !23185, !alias.scope !23181, !noalias !23193, !noundef !10
  %728 = tail call noundef float @llvm.fabs.f32(float %history.i.i.sroa.19.08786), !dbg !23194
  %_0.i2816 = fmul float %_0.i3041, %_11.i.i.i.i, !dbg !23197
  %_0.i2380 = fadd float %_0.i2816, 0.000000e+00, !dbg !23200
  %_0.i2815 = fmul float %_0.i3041, %_14.i.i.i.i, !dbg !23202
  %_0.i2379 = fadd float %_0.i2815, 0.000000e+00, !dbg !23204
  %_0.i2814 = fmul float %_0.i3041, %_17.i.i.i.i, !dbg !23206
  %_0.i2378 = fadd float %_0.i2814, 0.000000e+00, !dbg !23208
  %_0.i2813 = fmul float %_0.i3041, %_20.i.i.i.i, !dbg !23210
  %_0.i2377 = fadd float %_0.i2813, 0.000000e+00, !dbg !23212
  %_0.i2812 = fmul float %history.i.i.sroa.0.08781, %_25.i.i.i.i, !dbg !23214
  %_0.i2376 = fadd float %_0.i2380, %_0.i2812, !dbg !23216
  %_0.i2811 = fmul float %history.i.i.sroa.0.08781, %_28.i.i.i.i, !dbg !23218
  %_0.i2375 = fadd float %_0.i2379, %_0.i2811, !dbg !23220
  %_0.i2810 = fmul float %history.i.i.sroa.0.08781, %_31.i.i.i.i, !dbg !23222
  %_0.i2374 = fadd float %_0.i2378, %_0.i2810, !dbg !23224
  %_0.i2809 = fmul float %history.i.i.sroa.0.08781, %_34.i.i.i.i, !dbg !23226
  %_0.i2373 = fadd float %_0.i2377, %_0.i2809, !dbg !23228
  %_0.i2808 = fmul float %history.i.i.sroa.7.08782, %_39.i.i.i.i, !dbg !23230
  %_0.i2372 = fadd float %_0.i2376, %_0.i2808, !dbg !23232
  %_0.i2807 = fmul float %history.i.i.sroa.7.08782, %_42.i.i.i.i, !dbg !23234
  %_0.i2371 = fadd float %_0.i2375, %_0.i2807, !dbg !23236
  %_0.i2806 = fmul float %history.i.i.sroa.7.08782, %_45.i.i.i.i, !dbg !23238
  %_0.i2370 = fadd float %_0.i2374, %_0.i2806, !dbg !23240
  %_0.i2805 = fmul float %history.i.i.sroa.7.08782, %_48.i.i.i.i, !dbg !23242
  %_0.i2369 = fadd float %_0.i2373, %_0.i2805, !dbg !23244
  %_0.i2804 = fmul float %history.i.i.sroa.10.08783, %_53.i.i.i.i, !dbg !23246
  %_0.i2368 = fadd float %_0.i2372, %_0.i2804, !dbg !23248
  %_0.i2803 = fmul float %history.i.i.sroa.10.08783, %_56.i.i.i.i, !dbg !23250
  %_0.i2367 = fadd float %_0.i2371, %_0.i2803, !dbg !23252
  %_0.i2802 = fmul float %history.i.i.sroa.10.08783, %_59.i.i.i.i, !dbg !23254
  %_0.i2366 = fadd float %_0.i2370, %_0.i2802, !dbg !23256
  %_0.i2801 = fmul float %history.i.i.sroa.10.08783, %_62.i.i.i.i, !dbg !23258
  %_0.i2365 = fadd float %_0.i2369, %_0.i2801, !dbg !23260
  %_0.i2800 = fmul float %history.i.i.sroa.13.08784, %_67.i.i.i.i, !dbg !23262
  %_0.i2364 = fadd float %_0.i2368, %_0.i2800, !dbg !23264
  %_0.i2799 = fmul float %history.i.i.sroa.13.08784, %_70.i.i.i.i, !dbg !23266
  %_0.i2363 = fadd float %_0.i2367, %_0.i2799, !dbg !23268
  %_0.i2798 = fmul float %history.i.i.sroa.13.08784, %_73.i.i.i.i, !dbg !23270
  %_0.i2362 = fadd float %_0.i2366, %_0.i2798, !dbg !23272
  %_0.i2797 = fmul float %history.i.i.sroa.13.08784, %_76.i.i.i.i, !dbg !23274
  %_0.i2361 = fadd float %_0.i2365, %_0.i2797, !dbg !23276
  %_0.i2796 = fmul float %history.i.i.sroa.16.08785, %_81.i.i.i.i, !dbg !23278
  %_0.i2360 = fadd float %_0.i2364, %_0.i2796, !dbg !23280
  %_0.i2795 = fmul float %history.i.i.sroa.16.08785, %_84.i.i.i.i, !dbg !23282
  %_0.i2359 = fadd float %_0.i2363, %_0.i2795, !dbg !23284
  %_0.i2794 = fmul float %history.i.i.sroa.16.08785, %_87.i.i.i.i, !dbg !23286
  %_0.i2358 = fadd float %_0.i2362, %_0.i2794, !dbg !23288
  %_0.i2793 = fmul float %history.i.i.sroa.16.08785, %_90.i.i.i.i, !dbg !23290
  %_0.i2357 = fadd float %_0.i2361, %_0.i2793, !dbg !23292
  %_0.i2792 = fmul float %history.i.i.sroa.19.08786, %_95.i.i.i.i, !dbg !23294
  %_0.i2356 = fadd float %_0.i2360, %_0.i2792, !dbg !23296
  %_0.i2791 = fmul float %history.i.i.sroa.19.08786, %_98.i.i.i.i, !dbg !23298
  %_0.i2355 = fadd float %_0.i2359, %_0.i2791, !dbg !23300
  %_0.i2790 = fmul float %history.i.i.sroa.19.08786, %_101.i.i.i.i, !dbg !23302
  %_0.i2354 = fadd float %_0.i2358, %_0.i2790, !dbg !23304
  %_0.i2789 = fmul float %history.i.i.sroa.19.08786, %_104.i.i.i.i, !dbg !23306
  %_0.i2353 = fadd float %_0.i2357, %_0.i2789, !dbg !23308
  %_0.i2788 = fmul float %history.i.i.sroa.22.08787, %_109.i.i.i.i, !dbg !23310
  %_0.i2352 = fadd float %_0.i2356, %_0.i2788, !dbg !23312
  %_0.i2787 = fmul float %history.i.i.sroa.22.08787, %_112.i.i.i.i, !dbg !23314
  %_0.i2351 = fadd float %_0.i2355, %_0.i2787, !dbg !23316
  %_0.i2786 = fmul float %history.i.i.sroa.22.08787, %_115.i.i.i.i, !dbg !23318
  %_0.i2350 = fadd float %_0.i2354, %_0.i2786, !dbg !23320
  %_0.i2785 = fmul float %history.i.i.sroa.22.08787, %_118.i.i.i.i, !dbg !23322
  %_0.i2349 = fadd float %_0.i2353, %_0.i2785, !dbg !23324
  %_0.i2784 = fmul float %history.i.i.sroa.26.08788, %_123.i.i.i.i, !dbg !23326
  %_0.i2348 = fadd float %_0.i2352, %_0.i2784, !dbg !23328
  %_0.i2783 = fmul float %history.i.i.sroa.26.08788, %_126.i.i.i.i, !dbg !23330
  %_0.i2347 = fadd float %_0.i2351, %_0.i2783, !dbg !23332
  %_0.i2782 = fmul float %history.i.i.sroa.26.08788, %_129.i.i.i.i, !dbg !23334
  %_0.i2346 = fadd float %_0.i2350, %_0.i2782, !dbg !23336
  %_0.i2781 = fmul float %history.i.i.sroa.26.08788, %_132.i.i.i.i, !dbg !23338
  %_0.i2345 = fadd float %_0.i2349, %_0.i2781, !dbg !23340
  %_0.i2780 = fmul float %history.i.i.sroa.29.08789, %_137.i.i.i.i, !dbg !23342
  %_0.i2344 = fadd float %_0.i2348, %_0.i2780, !dbg !23344
  %_0.i2779 = fmul float %history.i.i.sroa.29.08789, %_140.i.i.i.i, !dbg !23346
  %_0.i2343 = fadd float %_0.i2347, %_0.i2779, !dbg !23348
  %_0.i2778 = fmul float %history.i.i.sroa.29.08789, %_143.i.i.i.i, !dbg !23350
  %_0.i2342 = fadd float %_0.i2346, %_0.i2778, !dbg !23352
  %_0.i2777 = fmul float %history.i.i.sroa.29.08789, %_146.i.i.i.i, !dbg !23354
  %_0.i2341 = fadd float %_0.i2345, %_0.i2777, !dbg !23356
  %_0.i2776 = fmul float %history.i.i.sroa.32.08790, %_151.i.i.i.i, !dbg !23358
  %_0.i2340 = fadd float %_0.i2344, %_0.i2776, !dbg !23360
  %_0.i2775 = fmul float %history.i.i.sroa.32.08790, %_154.i.i.i.i, !dbg !23362
  %_0.i2339 = fadd float %_0.i2343, %_0.i2775, !dbg !23364
  %_0.i2774 = fmul float %history.i.i.sroa.32.08790, %_157.i.i.i.i, !dbg !23366
  %_0.i2338 = fadd float %_0.i2342, %_0.i2774, !dbg !23368
  %_0.i2773 = fmul float %history.i.i.sroa.32.08790, %_160.i.i.i.i, !dbg !23370
  %_0.i2337 = fadd float %_0.i2341, %_0.i2773, !dbg !23372
  %_0.i2772 = fmul float %history.i.i.sroa.35.08791, %_165.i.i.i.i, !dbg !23374
  %_0.i2336 = fadd float %_0.i2340, %_0.i2772, !dbg !23376
  %_0.i2771 = fmul float %history.i.i.sroa.35.08791, %_168.i.i.i.i, !dbg !23378
  %_0.i2335 = fadd float %_0.i2339, %_0.i2771, !dbg !23380
  %_0.i2770 = fmul float %history.i.i.sroa.35.08791, %_171.i.i.i.i, !dbg !23382
  %_0.i2334 = fadd float %_0.i2338, %_0.i2770, !dbg !23384
  %_0.i2769 = fmul float %history.i.i.sroa.35.08791, %_174.i.i.i.i, !dbg !23386
  %_0.i2333 = fadd float %_0.i2337, %_0.i2769, !dbg !23388
  %729 = tail call noundef float @llvm.fabs.f32(float %_0.i2336), !dbg !23390
  %_3.i.i3692.inv = fcmp ogt float %728, %729, !dbg !23392
  %_4.i.i3699.v = select i1 %_3.i.i3692.inv, float %728, float %729, !dbg !23392
  %730 = tail call noundef float @llvm.fabs.f32(float %_0.i2335), !dbg !23390
  %_3.i.i3692.inv.1 = fcmp ogt float %_4.i.i3699.v, %730, !dbg !23392
  %_4.i.i3699.v.1 = select i1 %_3.i.i3692.inv.1, float %_4.i.i3699.v, float %730, !dbg !23392
  %731 = tail call noundef float @llvm.fabs.f32(float %_0.i2334), !dbg !23390
  %_3.i.i3692.inv.2 = fcmp ogt float %_4.i.i3699.v.1, %731, !dbg !23392
  %_4.i.i3699.v.2 = select i1 %_3.i.i3692.inv.2, float %_4.i.i3699.v.1, float %731, !dbg !23392
  %732 = tail call noundef float @llvm.fabs.f32(float %_0.i2333), !dbg !23390
  %_3.i.i3692.inv.3 = fcmp ogt float %_4.i.i3699.v.2, %732, !dbg !23392
  %_4.i.i3699.v.3 = select i1 %_3.i.i3692.inv.3, float %_4.i.i3699.v.2, float %732, !dbg !23392
  %_39.i.i = getelementptr inbounds nuw float, ptr %peaks_right.i, i32 %iter.sroa.0.0.i.i8792, !dbg !23395
  store float %_4.i.i3699.v.3, ptr %_39.i.i, align 4, !dbg !23400, !alias.scope !23402, !noalias !23193
  %exitcond11964.not = icmp eq i32 %727, %umax11963, !dbg !23405
  br i1 %exitcond11964.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i, label %bb5.i.i, !dbg !23171

bb7.i.i:                                          ; preds = %bb5.i.i
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !22949
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i.i.sroa.0.0.lcssa14650, ptr %hot_right.i, align 4, !dbg !22950
  store float %history.i.i.sroa.7.0.lcssa14670, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.10.0.lcssa14690, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.13.0.lcssa14710, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.16.0.lcssa14730, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.19.0.lcssa14750, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.22.0.lcssa14770, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.26.0.lcssa14790, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.29.0.lcssa14810, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.32.0.lcssa14830, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.35.0.lcssa14850, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.38.0.lcssa14870, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store i32 %storemerge.i1245.lcssa88389047.lcssa14891, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88659083.lcssa14912, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89509119.lcssa14933, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89779155.lcssa14954, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219414339.lcssa14976, ptr %705, align 4
  store float %_0.i3236.lcssa1221014357.lcssa14998, ptr %707, align 4
  store float %_0.i2858.lcssa1223414375.lcssa15020, ptr %713, align 4
  store float %_0.i3232.lcssa1223514393.lcssa15042, ptr %715, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_11.i.i, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #33, !dbg !23407, !noalias !23193
  unreachable, !dbg !23407

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i
  %history.i.i.sroa.0.0.lcssa = phi float [ %history.i.i.sroa.0.0.lcssa14650, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %_0.i3041, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ], !dbg !22950
  %history.i.i.sroa.7.0.lcssa = phi float [ %history.i.i.sroa.7.0.lcssa14670, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i.sroa.0.08781, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ], !dbg !22950
  %history.i.i.sroa.10.0.lcssa = phi float [ %history.i.i.sroa.10.0.lcssa14690, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i.sroa.7.08782, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ], !dbg !22950
  %history.i.i.sroa.13.0.lcssa = phi float [ %history.i.i.sroa.13.0.lcssa14710, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i.sroa.10.08783, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ], !dbg !22950
  %history.i.i.sroa.16.0.lcssa = phi float [ %history.i.i.sroa.16.0.lcssa14730, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i.sroa.13.08784, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ], !dbg !22950
  %history.i.i.sroa.19.0.lcssa = phi float [ %history.i.i.sroa.19.0.lcssa14750, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i.sroa.16.08785, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ], !dbg !22950
  %history.i.i.sroa.22.0.lcssa = phi float [ %history.i.i.sroa.22.0.lcssa14770, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i.sroa.19.08786, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ], !dbg !22950
  %history.i.i.sroa.26.0.lcssa = phi float [ %history.i.i.sroa.26.0.lcssa14790, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i.sroa.22.08787, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ], !dbg !22950
  %history.i.i.sroa.29.0.lcssa = phi float [ %history.i.i.sroa.29.0.lcssa14810, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i.sroa.26.08788, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ], !dbg !22950
  %history.i.i.sroa.32.0.lcssa = phi float [ %history.i.i.sroa.32.0.lcssa14830, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i.sroa.29.08789, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ], !dbg !22950
  %history.i.i.sroa.35.0.lcssa = phi float [ %history.i.i.sroa.35.0.lcssa14850, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i.sroa.32.08790, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ], !dbg !22950
  %history.i.i.sroa.38.0.lcssa = phi float [ %history.i.i.sroa.38.0.lcssa14870, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i.sroa.35.08791, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ], !dbg !22950
  br i1 %_20.i42.i8753.not, label %bb15.i.loopexit, label %bb20.i.lr.ph, !dbg !23408

bb20.i.lr.ph:                                     ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i
  %_68.i.sroa.3.0.copyload = load i32, ptr %_68.i.sroa.3.0..sroa_idx, align 4, !noalias !22886
  %_68.i.sroa.4.0.copyload = load i32, ptr %_68.i.sroa.4.0..sroa_idx, align 4, !noalias !22886
  %_69.i.sroa.3.0.copyload = load i32, ptr %_69.i.sroa.3.0..sroa_idx, align 4, !noalias !22886
  %_69.i.sroa.4.0.copyload = load i32, ptr %_69.i.sroa.4.0..sroa_idx, align 4, !noalias !22886
  %_54.0.i256.i = load ptr, ptr %uniform_left.i, align 4, !nonnull !10, !align !10189
  %_54.1.i257.i = load i32, ptr %702, align 4
  %_18.i267.i = load i32, ptr %700, align 4
  %_29.i12528805.not = icmp eq i32 %_18.i267.i, 0
  %_56.0.i278.i = load ptr, ptr %703, align 4, !nonnull !10, !align !10189
  %_56.1.i279.i = load i32, ptr %704, align 4
  %_58.1.i304.i = load i32, ptr %708, align 4
  %_58.0.i303.i = load ptr, ptr %709, align 4, !nonnull !10, !align !10189
  %_54.0.i.i = load ptr, ptr %uniform_right.i, align 4, !nonnull !10, !align !10189
  %_54.1.i.i = load i32, ptr %710, align 4
  %_18.i.i = load i32, ptr %701, align 4
  %_29.i12228809.not = icmp eq i32 %_18.i.i, 0
  %_56.0.i.i = load ptr, ptr %711, align 4, !nonnull !10, !align !10189
  %_56.1.i.i = load i32, ptr %712, align 4
  %_58.1.i.i = load i32, ptr %716, align 4
  %_58.0.i.i = load ptr, ptr %717, align 4, !nonnull !10, !align !10189
  %_8.i29.i = load float, ptr %_110.i, align 4
  %_9.i30.i = load float, ptr %_111.i, align 4
  %_8.i.i = load float, ptr %_115.i, align 4
  %_9.i.i = load float, ptr %_116.i, align 4
  %_37.i291.i = load float, ptr %706, align 4
  %_37.i.i = load float, ptr %714, align 4
  br label %bb20.i, !dbg !23408

bb20.i:                                           ; preds = %bb20.i.lr.ph, %bb67.i
  %_0.i3232.lcssa1223514394 = phi float [ %_0.i3232.lcssa1223514393.lcssa15042, %bb20.i.lr.ph ], [ %_0.i3232.lcssa1223514393, %bb67.i ]
  %_0.i2858.lcssa1223414376 = phi float [ %_0.i2858.lcssa1223414375.lcssa15020, %bb20.i.lr.ph ], [ %_0.i2858.lcssa1223414375, %bb67.i ]
  %_0.i3236.lcssa1221014358 = phi float [ %_0.i3236.lcssa1221014357.lcssa14998, %bb20.i.lr.ph ], [ %_0.i3236.lcssa1221014357, %bb67.i ]
  %_0.i2862.lcssa1219414340 = phi float [ %_0.i2862.lcssa1219414339.lcssa14976, %bb20.i.lr.ph ], [ %_0.i2862.lcssa1219414339, %bb67.i ]
  %running.sroa.0.0.i1210.lcssa89779156 = phi float [ %running.sroa.0.0.i1210.lcssa89779155.lcssa14954, %bb20.i.lr.ph ], [ %running.sroa.0.0.i1210.lcssa89779155, %bb67.i ]
  %storemerge.i1215.lcssa89509120 = phi i32 [ %storemerge.i1215.lcssa89509119.lcssa14933, %bb20.i.lr.ph ], [ %storemerge.i1215.lcssa89509119, %bb67.i ]
  %running.sroa.0.0.i1240.lcssa88659084 = phi float [ %running.sroa.0.0.i1240.lcssa88659083.lcssa14912, %bb20.i.lr.ph ], [ %running.sroa.0.0.i1240.lcssa88659083, %bb67.i ]
  %storemerge.i1245.lcssa88389048 = phi i32 [ %storemerge.i1245.lcssa88389047.lcssa14891, %bb20.i.lr.ph ], [ %storemerge.i1245.lcssa88389047, %bb67.i ]
  %frame.sroa.0.0.i9043 = phi i32 [ 0, %bb20.i.lr.ph ], [ %_83.i, %bb67.i ]
  %main_cursor.sroa.0.1.i9042 = phi i32 [ %main_cursor.sroa.0.0.i9192, %bb20.i.lr.ph ], [ %main_cursor.sroa.0.2.i, %bb67.i ]
  %ring_cursor.sroa.0.1.i9041 = phi i32 [ %ring_cursor.sroa.0.0.i9191, %bb20.i.lr.ph ], [ %ring_cursor.sroa.0.2.i, %bb67.i ]
  %_65.i = sub nuw nsw i32 %spec.store.select.i, %frame.sroa.0.0.i9043, !dbg !23410
  %ring.i1803 = load i32, ptr %660, align 4, !dbg !23411, !alias.scope !23413, !noalias !23416, !noundef !10
  %main.i1804 = load i32, ptr %661, align 4, !dbg !23420, !alias.scope !23413, !noalias !23416, !noundef !10
  %_10.i1805 = add i32 %ring_cursor.sroa.0.1.i9041, 1, !dbg !23421
  %_38.not.i1806 = icmp ult i32 %_10.i1805, %ring.i1803, !dbg !23422
  %733 = select i1 %_38.not.i1806, i32 0, i32 %ring.i1803, !dbg !23422
  %start1.sroa.0.0.i1807 = sub nuw i32 %_10.i1805, %733, !dbg !23422
  %_12.i1809 = add i32 %_68.i.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i9041, !dbg !23424
  %_39.not.i1810 = icmp ult i32 %_12.i1809, %ring.i1803, !dbg !23425
  %734 = select i1 %_39.not.i1810, i32 0, i32 %ring.i1803, !dbg !23425
  %left_end.sroa.0.0.i1811 = sub nuw i32 %_12.i1809, %734, !dbg !23425
  %_15.i1813 = add i32 %_69.i.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i9041, !dbg !23427
  %_40.not.i1814 = icmp ult i32 %_15.i1813, %ring.i1803, !dbg !23428
  %735 = select i1 %_40.not.i1814, i32 0, i32 %ring.i1803, !dbg !23428
  %right_end.sroa.0.0.i1815 = sub nuw i32 %_15.i1813, %735, !dbg !23428
  %_18.i1817 = add i32 %_68.i.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i9041, !dbg !23430
  %_41.not.i1818 = icmp ult i32 %_18.i1817, %ring.i1803, !dbg !23431
  %736 = select i1 %_41.not.i1818, i32 0, i32 %ring.i1803, !dbg !23431
  %left_expiring.sroa.0.0.i1819 = sub nuw i32 %_18.i1817, %736, !dbg !23431
  %_21.i1821 = add i32 %_69.i.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i9041, !dbg !23433
  %_42.not.i1822 = icmp ult i32 %_21.i1821, %ring.i1803, !dbg !23434
  %737 = select i1 %_42.not.i1822, i32 0, i32 %ring.i1803, !dbg !23434
  %right_expiring.sroa.0.0.i1823 = sub nuw i32 %_21.i1821, %737, !dbg !23434
  %738 = sub i32 %ring.i1803, %ring_cursor.sroa.0.1.i9041, !dbg !23436
  %spec.store.select.i1824 = tail call i32 @llvm.umin.i32(i32 %738, i32 %_65.i), !dbg !23437
  %739 = sub i32 %main.i1804, %main_cursor.sroa.0.1.i9042, !dbg !23439
  %_24.sroa.0.0.i1826 = tail call i32 @llvm.umin.i32(i32 %739, i32 %spec.store.select.i1824), !dbg !23440
  %740 = sub i32 %ring.i1803, %start1.sroa.0.0.i1807, !dbg !23442
  %_25.sroa.0.0.i1828 = tail call i32 @llvm.umin.i32(i32 %740, i32 %_24.sroa.0.0.i1826), !dbg !23443
  %741 = sub i32 %ring.i1803, %left_end.sroa.0.0.i1811, !dbg !23445
  %_27.sroa.0.0.i1830 = tail call i32 @llvm.umin.i32(i32 %741, i32 %_25.sroa.0.0.i1828), !dbg !23446
  %742 = sub i32 %ring.i1803, %right_end.sroa.0.0.i1815, !dbg !23448
  %_29.sroa.0.0.i1832 = tail call i32 @llvm.umin.i32(i32 %742, i32 %_27.sroa.0.0.i1830), !dbg !23449
  %743 = sub i32 %ring.i1803, %left_expiring.sroa.0.0.i1819, !dbg !23451
  %_31.sroa.0.0.i1834 = tail call i32 @llvm.umin.i32(i32 %743, i32 %_29.sroa.0.0.i1832), !dbg !23452
  %744 = sub i32 %ring.i1803, %right_expiring.sroa.0.0.i1823, !dbg !23454
  %run.sroa.0.0.i1836 = tail call i32 @llvm.umin.i32(i32 %744, i32 %_31.sroa.0.0.i1834), !dbg !23455
  %_72.i = add i32 %frame.sroa.0.0.i9043, %iter1.sroa.0.0.i9193, !dbg !23457
  %_76.i = add i32 %run.sroa.0.0.i1836, %_72.i, !dbg !23460
  %_172.i = icmp ult i32 %_76.i, %_72.i, !dbg !23463
  %_166.not.i = icmp ugt i32 %_76.i, %left_io.1
  %or.cond.i = or i1 %_172.i, %_166.not.i, !dbg !23463
  br i1 %or.cond.i, label %bb51.i, label %bb49.i, !dbg !23463, !prof !4694

bb51.i:                                           ; preds = %bb20.i
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !22949
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !22950
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store i32 %storemerge.i1245.lcssa88389047.lcssa14891, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88659083.lcssa14912, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89509119.lcssa14933, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89779155.lcssa14954, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219414339.lcssa14976, ptr %705, align 4
  store float %_0.i3236.lcssa1221014357.lcssa14998, ptr %707, align 4
  store float %_0.i2858.lcssa1223414375.lcssa15020, ptr %713, align 4
  store float %_0.i3232.lcssa1223514393.lcssa15042, ptr %715, align 4
  store float %_0.i2862.lcssa1219414340, ptr %705, align 4
  store float %_0.i3236.lcssa1221014358, ptr %707, align 4
  store float %_0.i2858.lcssa1223414376, ptr %713, align 4
  store float %_0.i3232.lcssa1223514394, ptr %715, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_72.i, i32 noundef %_76.i, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_bd3c2aadcc2cdd6816f02be0ceab2cc1) #33, !dbg !23470, !noalias !22857
  unreachable, !dbg !23470

bb49.i:                                           ; preds = %bb20.i
  %_175.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %_72.i, !dbg !23471
  %_176.not.i = icmp ugt i32 %_76.i, %right_io.1, !dbg !23475
  br i1 %_176.not.i, label %bb54.i, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4183, !dbg !23475, !prof !902

bb54.i:                                           ; preds = %bb49.i
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !22949
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !22950
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store i32 %storemerge.i1245.lcssa88389047.lcssa14891, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88659083.lcssa14912, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89509119.lcssa14933, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89779155.lcssa14954, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219414339.lcssa14976, ptr %705, align 4
  store float %_0.i3236.lcssa1221014357.lcssa14998, ptr %707, align 4
  store float %_0.i2858.lcssa1223414375.lcssa15020, ptr %713, align 4
  store float %_0.i3232.lcssa1223514393.lcssa15042, ptr %715, align 4
  store float %_0.i2862.lcssa1219414340, ptr %705, align 4
  store float %_0.i3236.lcssa1221014358, ptr %707, align 4
  store float %_0.i2858.lcssa1223414376, ptr %713, align 4
  store float %_0.i3232.lcssa1223514394, ptr %715, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_72.i, i32 noundef %_76.i, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c1156ffa98f252967759477badafe965) #33, !dbg !23480, !noalias !22857
  unreachable, !dbg !23480

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4183: ; preds = %bb49.i
  %_183.i = getelementptr inbounds nuw float, ptr %right_io.0, i32 %_72.i, !dbg !23481
  %_83.i = add nuw nsw i32 %run.sroa.0.0.i1836, %frame.sroa.0.0.i9043, !dbg !23485
  %_192.i = getelementptr inbounds nuw float, ptr %peaks_left.i, i32 %frame.sroa.0.0.i9043, !dbg !23487
  %_201.i = getelementptr inbounds nuw float, ptr %peaks_right.i, i32 %frame.sroa.0.0.i9043, !dbg !23497
  %_2.i41868813.not = icmp eq i32 %run.sroa.0.0.i1836, 0, !dbg !23507
  br i1 %_2.i41868813.not, label %bb67.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048.lr.ph, !dbg !23507

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048.lr.ph: ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4183
  %umax11967 = call i32 @llvm.umax.i32(i32 %ring_cursor.sroa.0.1.i9041, i32 %_54.1.i257.i), !dbg !23507
  %umax11968 = call i32 @llvm.umax.i32(i32 %main_cursor.sroa.0.1.i9042, i32 %_58.1.i304.i), !dbg !23507
  %745 = sub i32 %umax11967, %ring_cursor.sroa.0.1.i9041, !dbg !23507
  %746 = sub i32 %umax11968, %main_cursor.sroa.0.1.i9042, !dbg !23507
  %umin11971 = call i32 @llvm.umin.i32(i32 %741, i32 %742), !dbg !23507
  %umin11972 = call i32 @llvm.umin.i32(i32 %umin11971, i32 %743), !dbg !23507
  %umin11973 = call i32 @llvm.umin.i32(i32 %umin11972, i32 %744), !dbg !23507
  %umin11974 = call i32 @llvm.umin.i32(i32 %umin11973, i32 %740), !dbg !23507
  %umin11975 = call i32 @llvm.umin.i32(i32 %umin11974, i32 %738), !dbg !23507
  %umin11976 = call i32 @llvm.umin.i32(i32 %umin11975, i32 %739), !dbg !23507
  %747 = sub nsw i32 %umin11977, %frame.sroa.0.0.i9043, !dbg !23507
  %umin11978 = call i32 @llvm.umin.i32(i32 %umin11976, i32 %747), !dbg !23507
  br label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048, !dbg !23507

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195
  %_0.i32329012 = phi float [ %_0.i3232.lcssa1223514394, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048.lr.ph ], [ %_0.i3232, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %_0.i28588983 = phi float [ %_0.i2858.lcssa1223414376, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048.lr.ph ], [ %_0.i2858, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %running.sroa.0.0.i12108955 = phi float [ %running.sroa.0.0.i1210.lcssa89779156, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048.lr.ph ], [ %running.sroa.0.0.i1210, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %storemerge.i12158928 = phi i32 [ %storemerge.i1215.lcssa89509120, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048.lr.ph ], [ %storemerge.i1215, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %_0.i32368900 = phi float [ %_0.i3236.lcssa1221014358, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048.lr.ph ], [ %_0.i3236, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %_0.i28628871 = phi float [ %_0.i2862.lcssa1219414340, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048.lr.ph ], [ %_0.i2862, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %running.sroa.0.0.i12408843 = phi float [ %running.sroa.0.0.i1240.lcssa88659084, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048.lr.ph ], [ %running.sroa.0.0.i1240, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %storemerge.i12458816 = phi i32 [ %storemerge.i1245.lcssa88389048, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048.lr.ph ], [ %storemerge.i1245, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %iter.i.sroa.41.08815 = phi i32 [ 0, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048.lr.ph ], [ %_206.0.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %_206.0.i = add nuw i32 %iter.i.sroa.41.08815, 1, !dbg !23516
  %data.i.i.i.i.i.i4197 = getelementptr inbounds nuw float, ptr %_175.i, i32 %iter.i.sroa.41.08815, !dbg !23519
  %data.i5.i.i.i.i.i4201 = getelementptr inbounds nuw float, ptr %_183.i, i32 %iter.i.sroa.41.08815, !dbg !23526
  %data.i.i.i.i4206 = getelementptr inbounds nuw float, ptr %_192.i, i32 %iter.i.sroa.41.08815, !dbg !23529
  %data.i.i4211 = getelementptr inbounds nuw float, ptr %_201.i, i32 %iter.i.sroa.41.08815, !dbg !23532
  %_0.i3061 = load float, ptr %data.i.i.i.i4206, align 4, !dbg !23535, !alias.scope !23540, !noalias !22857, !noundef !10
  %_0.i3056 = load float, ptr %data.i.i4211, align 4, !dbg !23543, !alias.scope !23546, !noalias !22857, !noundef !10
  %_3.i.i3719 = fcmp ule float %_0.i3056, %_0.i3061, !dbg !23549
  %_6.i.i3721 = bitcast float %_0.i3056 to i32, !dbg !23553
  %_8.i.i3723 = bitcast float %_0.i3061 to i32, !dbg !23556
  %_4.i.i3726 = select i1 %_3.i.i3719, i32 %_8.i.i3723, i32 %_6.i.i3721, !dbg !23558
  %_5.i3492 = and i32 %_4.i.i3726, %.none.i, !dbg !23559
  %_7.i3488 = and i32 %_9.i3494, %_6.i.i3721, !dbg !23562
  %_4.i3489 = or disjoint i32 %_5.i3492, %_7.i3488, !dbg !23565
  %_0.i3490 = bitcast i32 %_4.i3489 to float, !dbg !23566
  %_0.i3051 = load float, ptr %data.i.i.i.i.i.i4197, align 4, !dbg !23568, !alias.scope !23571, !noalias !22857, !noundef !10
  %_0.i3046 = load float, ptr %data.i5.i.i.i.i.i4201, align 4, !dbg !23574, !alias.scope !23577, !noalias !22857, !noundef !10
  %_213.i = add nuw i32 %iter.i.sroa.41.08815, %ring_cursor.sroa.0.1.i9041, !dbg !23580
  %_214.i = add nuw i32 %iter.i.sroa.41.08815, %main_cursor.sroa.0.1.i9042, !dbg !23584
  %_215.i = add nuw i32 %iter.i.sroa.41.08815, %left_end.sroa.0.0.i1811, !dbg !23585
  %_216.i = add i32 %iter.i.sroa.41.08815, %start1.sroa.0.0.i1807, !dbg !23586
  %_217.i = add nuw i32 %iter.i.sroa.41.08815, %left_expiring.sroa.0.0.i1819, !dbg !23587
  %_7.i8.i259.i = add i32 %_213.i, 1, !dbg !23588
  %exitcond11969.not = icmp eq i32 %iter.i.sroa.41.08815, %745, !dbg !23591
  br i1 %exitcond11969.not, label %bb4.i13.i318.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i263.i, !dbg !23591, !prof !4694

bb4.i13.i318.i:                                   ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !22949
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !22950
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store i32 %storemerge.i1245.lcssa88389047.lcssa14891, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88659083.lcssa14912, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89509119.lcssa14933, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89779155.lcssa14954, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219414339.lcssa14976, ptr %705, align 4
  store float %_0.i3236.lcssa1221014357.lcssa14998, ptr %707, align 4
  store float %_0.i2858.lcssa1223414375.lcssa15020, ptr %713, align 4
  store float %_0.i3232.lcssa1223514393.lcssa15042, ptr %715, align 4
  store float %_0.i2862.lcssa1219414340, ptr %705, align 4
  store float %_0.i3236.lcssa1221014358, ptr %707, align 4
  store float %_0.i2858.lcssa1223414376, ptr %713, align 4
  store float %_0.i3232.lcssa1223514394, ptr %715, align 4
  %748 = add i32 %umax11967, 1, !dbg !23507
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_213.i, i32 noundef %748, i32 noundef range(i32 0, 536870912) %_54.1.i257.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #33, !dbg !23595, !noalias !23596
  unreachable, !dbg !23595

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i263.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048
  %_7.i3495 = and i32 %_9.i3494, %_8.i.i3723, !dbg !23604
  %_4.i3496 = or disjoint i32 %_5.i3492, %_7.i3495, !dbg !23559
  %_0.i3497 = bitcast i32 %_4.i3496 to float, !dbg !23605
  %_0.i2398 = fdiv float %_8.i29.i, %_0.i3497, !dbg !23607
  %_3.i1965 = fcmp uge float %_8.i29.i, %_0.i3497, !dbg !23609
  %_0.i3483 = select i1 %_3.i1965, float 1.000000e+00, float %_0.i2398, !dbg !23611
  %_17.i12.i264.i = getelementptr inbounds nuw float, ptr %_54.0.i256.i, i32 %_213.i, !dbg !23613
  store float %_0.i3483, ptr %_17.i12.i264.i, align 4, !dbg !23615, !alias.scope !23617, !noalias !23620
  %or.cond.i1287.not = icmp ult i32 %_215.i, %_54.1.i257.i, !dbg !23621
  br i1 %or.cond.i1287.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1291, label %bb4.i1290, !dbg !23621, !prof !10587

bb4.i1290:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i263.i
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !22949
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !22950
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store i32 %storemerge.i1245.lcssa88389047.lcssa14891, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88659083.lcssa14912, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89509119.lcssa14933, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89779155.lcssa14954, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219414339.lcssa14976, ptr %705, align 4
  store float %_0.i3236.lcssa1221014357.lcssa14998, ptr %707, align 4
  store float %_0.i2858.lcssa1223414375.lcssa15020, ptr %713, align 4
  store float %_0.i3232.lcssa1223514393.lcssa15042, ptr %715, align 4
  store float %_0.i2862.lcssa1219414340, ptr %705, align 4
  store float %_0.i3236.lcssa1221014358, ptr %707, align 4
  store float %_0.i2858.lcssa1223414376, ptr %713, align 4
  store float %_0.i3232.lcssa1223514394, ptr %715, align 4
  %_5.i1284 = add i32 %_215.i, 1, !dbg !23627
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_215.i, i32 noundef %_5.i1284, i32 noundef range(i32 0, 536870912) %_54.1.i257.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !23628, !noalias !23629
  unreachable, !dbg !23628

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1291: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i263.i
  %_15.i1288 = getelementptr inbounds nuw float, ptr %_54.0.i256.i, i32 %_215.i, !dbg !23635
  %_0.i2904 = load float, ptr %_15.i1288, align 4, !dbg !23637, !alias.scope !23639, !noalias !23642, !noundef !10
  %749 = icmp eq i32 %storemerge.i12458816, 0, !dbg !23643
  %_3.i.i3746.inv = fcmp olt float %running.sroa.0.0.i12408843, %_0.i2904, !dbg !23643
  %_4.i.i3753.v = select i1 %_3.i.i3746.inv, float %running.sroa.0.0.i12408843, float %_0.i2904, !dbg !23643
  %running.sroa.0.0.i1240 = select i1 %749, float %_0.i2904, float %_4.i.i3753.v, !dbg !23643
  %_15.i1241 = add i32 %storemerge.i12458816, 1, !dbg !23644
  %complete.i1242 = icmp eq i32 %_15.i1241, %_18.i267.i, !dbg !23644
  br i1 %complete.i1242, label %bb11.i1248.preheader, label %bb7.i1243, !dbg !23645

bb11.i1248.preheader:                             ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1291
  br i1 %_29.i12528805.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1264, label %bb19.i1253, !dbg !23646

bb7.i1243:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1291
  %or.cond.i1279.not = icmp ult i32 %_216.i, %_54.1.i257.i, !dbg !23649
  br i1 %or.cond.i1279.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1283, label %bb4.i1282, !dbg !23649, !prof !10587

bb4.i1282:                                        ; preds = %bb7.i1243
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !22949
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !22950
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store i32 %storemerge.i1245.lcssa88389047.lcssa14891, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88659083.lcssa14912, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89509119.lcssa14933, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89779155.lcssa14954, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219414339.lcssa14976, ptr %705, align 4
  store float %_0.i3236.lcssa1221014357.lcssa14998, ptr %707, align 4
  store float %_0.i2858.lcssa1223414375.lcssa15020, ptr %713, align 4
  store float %_0.i3232.lcssa1223514393.lcssa15042, ptr %715, align 4
  store float %_0.i2862.lcssa1219414340, ptr %705, align 4
  store float %_0.i3236.lcssa1221014358, ptr %707, align 4
  store float %_0.i2858.lcssa1223414376, ptr %713, align 4
  store float %_0.i3232.lcssa1223514394, ptr %715, align 4
  %_5.i1276 = add i32 %_216.i, 1, !dbg !23654
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_216.i, i32 noundef %_5.i1276, i32 noundef range(i32 0, 536870912) %_54.1.i257.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !23655, !noalias !23656
  unreachable, !dbg !23655

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1283: ; preds = %bb7.i1243
  %_15.i1280 = getelementptr inbounds nuw float, ptr %_54.0.i256.i, i32 %_216.i, !dbg !23659
  %_0.i2906 = load float, ptr %_15.i1280, align 4, !dbg !23661, !alias.scope !23663, !noalias !23642, !noundef !10
  %_3.i.i3737.inv = fcmp olt float %_0.i2906, %running.sroa.0.0.i1240, !dbg !23666
  %_4.i.i3744.v = select i1 %_3.i.i3737.inv, float %_0.i2906, float %running.sroa.0.0.i1240, !dbg !23666
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1264, !dbg !23669

bb19.i1253:                                       ; preds = %bb11.i1248.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1260
  %end.sroa.0.0.i12518808 = phi i32 [ %751, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1260 ], [ %_215.i, %bb11.i1248.preheader ]
  %suffix.sroa.0.0.i12508807 = phi float [ %_4.i.i3735.v, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1260 ], [ %_0.i2904, %bb11.i1248.preheader ]
  %iter.sroa.0.0.i12498806 = phi i32 [ %_30.i1254, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1260 ], [ 0, %bb11.i1248.preheader ]
  %or.cond.i1266.not = icmp ult i32 %end.sroa.0.0.i12518808, %_54.1.i257.i, !dbg !23670
  br i1 %or.cond.i1266.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1260, label %bb4.i, !dbg !23670, !prof !10587

bb4.i:                                            ; preds = %bb19.i1253
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !22949
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !22950
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store i32 %storemerge.i1245.lcssa88389047.lcssa14891, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88659083.lcssa14912, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89509119.lcssa14933, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89779155.lcssa14954, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219414339.lcssa14976, ptr %705, align 4
  store float %_0.i3236.lcssa1221014357.lcssa14998, ptr %707, align 4
  store float %_0.i2858.lcssa1223414375.lcssa15020, ptr %713, align 4
  store float %_0.i3232.lcssa1223514393.lcssa15042, ptr %715, align 4
  store float %_0.i2862.lcssa1219414340, ptr %705, align 4
  store float %_0.i3236.lcssa1221014358, ptr %707, align 4
  store float %_0.i2858.lcssa1223414376, ptr %713, align 4
  store float %_0.i3232.lcssa1223514394, ptr %715, align 4
  %_5.i = add i32 %end.sroa.0.0.i12518808, 1, !dbg !23675
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %end.sroa.0.0.i12518808, i32 noundef %_5.i, i32 noundef range(i32 0, 536870912) %_54.1.i257.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !23676, !noalias !23677
  unreachable, !dbg !23676

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1260: ; preds = %bb19.i1253
  %_30.i1254 = add nuw i32 %iter.sroa.0.0.i12498806, 1, !dbg !23680
  %_15.i1267 = getelementptr inbounds nuw float, ptr %_54.0.i256.i, i32 %end.sroa.0.0.i12518808, !dbg !23683
  %_0.i2910 = load float, ptr %_15.i1267, align 4, !dbg !23685, !alias.scope !23687, !noalias !23642, !noundef !10
  %_3.i.i3728.inv = fcmp olt float %suffix.sroa.0.0.i12508807, %_0.i2910, !dbg !23690
  %_4.i.i3735.v = select i1 %_3.i.i3728.inv, float %suffix.sroa.0.0.i12508807, float %_0.i2910, !dbg !23690
  store float %_4.i.i3735.v, ptr %_15.i1267, align 4, !dbg !23693, !alias.scope !23696, !noalias !23642
  %750 = icmp eq i32 %end.sroa.0.0.i12518808, 0, !dbg !23699
  %spec.store.select.i1262 = select i1 %750, i32 %ring.i, i32 %end.sroa.0.0.i12518808, !dbg !23699
  %751 = add i32 %spec.store.select.i1262, -1, !dbg !23700
  %exitcond11965.not = icmp eq i32 %_30.i1254, %_18.i267.i, !dbg !23701
  br i1 %exitcond11965.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1264, label %bb19.i1253, !dbg !23646

_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1264: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1260, %bb11.i1248.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1283
  %storemerge.i1245 = phi i32 [ %_15.i1241, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1283 ], [ 0, %bb11.i1248.preheader ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1260 ], !dbg !23703
  %running.sroa.0.1.i1246 = phi float [ %_4.i.i3744.v, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1283 ], [ %running.sroa.0.0.i1240, %bb11.i1248.preheader ], [ %running.sroa.0.0.i1240, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1260 ], !dbg !23704
  %_0.i2822 = fmul float %running.sroa.0.1.i1246, 1.638400e+04, !dbg !23705
  %752 = tail call noundef float @llvm.floor.f32(float %_0.i2822), !dbg !23707
  %_0.i2821 = fmul float %752, 0x3F10000000000000, !dbg !23711
  %or.cond.i1447.not = icmp ult i32 %_217.i, %_56.1.i279.i, !dbg !23713
  br i1 %or.cond.i1447.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1451, label %bb4.i1450, !dbg !23713, !prof !10587

bb4.i1450:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1264
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !22949
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !22950
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store i32 %storemerge.i1245.lcssa88389047.lcssa14891, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88659083.lcssa14912, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89509119.lcssa14933, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89779155.lcssa14954, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219414339.lcssa14976, ptr %705, align 4
  store float %_0.i3236.lcssa1221014357.lcssa14998, ptr %707, align 4
  store float %_0.i2858.lcssa1223414375.lcssa15020, ptr %713, align 4
  store float %_0.i3232.lcssa1223514393.lcssa15042, ptr %715, align 4
  store float %_0.i2862.lcssa1219414340, ptr %705, align 4
  store float %_0.i3236.lcssa1221014358, ptr %707, align 4
  store float %_0.i2858.lcssa1223414376, ptr %713, align 4
  store float %_0.i3232.lcssa1223514394, ptr %715, align 4
  %_5.i1444 = add i32 %_217.i, 1, !dbg !23718
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_217.i, i32 noundef %_5.i1444, i32 noundef range(i32 0, 536870912) %_56.1.i279.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !23719, !noalias !23720
  unreachable, !dbg !23719

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1451: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1264
  %_15.i1448 = getelementptr inbounds nuw float, ptr %_56.0.i278.i, i32 %_217.i, !dbg !23723
  %_0.i2864 = load float, ptr %_15.i1448, align 4, !dbg !23725, !alias.scope !23727, !noalias !23730, !noundef !10
  %_0.i2382 = fadd float %_0.i2821, %_0.i28628871, !dbg !23731
  %_0.i2862 = fsub float %_0.i2382, %_0.i2864, !dbg !23733
  %_8.not.i3.i288.i = icmp ugt i32 %_7.i8.i259.i, %_56.1.i279.i
  br i1 %_8.not.i3.i288.i, label %bb4.i6.i317.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i289.i, !dbg !23735, !prof !4694

bb4.i6.i317.i:                                    ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1451
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !22949
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !22950
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store i32 %storemerge.i1245.lcssa88389047.lcssa14891, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88659083.lcssa14912, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89509119.lcssa14933, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89779155.lcssa14954, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219414339.lcssa14976, ptr %705, align 4
  store float %_0.i3236.lcssa1221014357.lcssa14998, ptr %707, align 4
  store float %_0.i2858.lcssa1223414375.lcssa15020, ptr %713, align 4
  store float %_0.i3232.lcssa1223514393.lcssa15042, ptr %715, align 4
  store float %_0.i2862.lcssa1219414340, ptr %705, align 4
  store float %_0.i3236.lcssa1221014358, ptr %707, align 4
  store float %_0.i2858.lcssa1223414376, ptr %713, align 4
  store float %_0.i3232.lcssa1223514394, ptr %715, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_213.i, i32 noundef %_7.i8.i259.i, i32 noundef range(i32 0, 536870912) %_56.1.i279.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #33, !dbg !23740, !noalias !23741
  unreachable, !dbg !23740

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i289.i: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1451
  %_17.i5.i290.i = getelementptr inbounds nuw float, ptr %_56.0.i278.i, i32 %_213.i, !dbg !23744
  store float %_0.i2821, ptr %_17.i5.i290.i, align 4, !dbg !23746, !alias.scope !23748, !noalias !23730
  %_0.i2397 = fdiv float %_0.i2862, %_37.i291.i, !dbg !23751
  %_0.i2861 = fsub float 1.000000e+00, %_0.i2397, !dbg !23753
  %_0.i2860 = fsub float %_0.i2861, %_0.i32368900, !dbg !23755
  %_4.i2413 = fmul float %_9.i30.i, %_0.i2860, !dbg !23757
  %_0.i2414 = fadd float %_0.i32368900, %_4.i2413, !dbg !23757
  %_3.i.i3710.inv = fcmp ogt float %_0.i2861, %_0.i2414, !dbg !23759
  %_4.i.i3717.v = select i1 %_3.i.i3710.inv, float %_0.i2861, float %_0.i2414, !dbg !23759
  %753 = tail call noundef float @llvm.fabs.f32(float %_4.i.i3717.v), !dbg !23762
  %754 = fcmp uge float %753, 0x3BC79CA100000000, !dbg !23765
  %_0.i3236 = select i1 %754, float %_4.i.i3717.v, float 0.000000e+00, !dbg !23767
  %_5.i1436 = add i32 %_214.i, 1, !dbg !23768
  %exitcond11970.not = icmp eq i32 %iter.i.sroa.41.08815, %746, !dbg !23770
  br i1 %exitcond11970.not, label %bb4.i1442, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3202, !dbg !23770, !prof !4694

bb4.i1442:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i289.i
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !22949
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !22950
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store i32 %storemerge.i1245.lcssa88389047.lcssa14891, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88659083.lcssa14912, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89509119.lcssa14933, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89779155.lcssa14954, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219414339.lcssa14976, ptr %705, align 4
  store float %_0.i3236.lcssa1221014357.lcssa14998, ptr %707, align 4
  store float %_0.i2858.lcssa1223414375.lcssa15020, ptr %713, align 4
  store float %_0.i3232.lcssa1223514393.lcssa15042, ptr %715, align 4
  store float %_0.i2862.lcssa1219414340, ptr %705, align 4
  store float %_0.i3236.lcssa1221014358, ptr %707, align 4
  store float %_0.i2858.lcssa1223414376, ptr %713, align 4
  store float %_0.i3232.lcssa1223514394, ptr %715, align 4
  %755 = add i32 %umax11968, 1, !dbg !23507
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_214.i, i32 noundef %755, i32 noundef range(i32 0, 536870912) %_58.1.i304.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !23774, !noalias !23775
  unreachable, !dbg !23774

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3202: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i289.i
  %_0.i2859 = fsub float 1.000000e+00, %_0.i3236, !dbg !23778
  %_15.i1440 = getelementptr inbounds nuw float, ptr %_58.0.i303.i, i32 %_214.i, !dbg !23780
  %_0.i2866 = load float, ptr %_15.i1440, align 4, !dbg !23782, !alias.scope !23784, !noalias !23730, !noundef !10
  store float %_0.i3051, ptr %_15.i1440, align 4, !dbg !23787, !alias.scope !23790, !noalias !23730
  %_0.i2820 = fmul float %_0.i2859, %_0.i2866, !dbg !23793
  %_6.i3471 = bitcast float %_0.i2866 to i32, !dbg !23795
  %_5.i3472 = and i32 %_6.i3471, %all.sroa.0.0.i, !dbg !23798
  %_8.i3473 = bitcast float %_0.i2820 to i32, !dbg !23799
  %_7.i3475 = and i32 %_9.i3474, %_8.i3473, !dbg !23801
  %_4.i3476 = or disjoint i32 %_7.i3475, %_5.i3472, !dbg !23798
  store i32 %_4.i3476, ptr %data.i.i.i.i.i.i4197, align 4, !dbg !23802, !alias.scope !23804, !noalias !23807
  %_220.i = add nuw i32 %iter.i.sroa.41.08815, %right_end.sroa.0.0.i1815, !dbg !23808
  %_222.i = add nuw i32 %iter.i.sroa.41.08815, %right_expiring.sroa.0.0.i1823, !dbg !23810
  %_8.not.i10.i.i = icmp ugt i32 %_7.i8.i259.i, %_54.1.i.i
  br i1 %_8.not.i10.i.i, label %bb4.i13.i.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i, !dbg !23811, !prof !4694

bb4.i13.i.i:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3202
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !22949
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !22950
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store i32 %storemerge.i1245.lcssa88389047.lcssa14891, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88659083.lcssa14912, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89509119.lcssa14933, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89779155.lcssa14954, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219414339.lcssa14976, ptr %705, align 4
  store float %_0.i3236.lcssa1221014357.lcssa14998, ptr %707, align 4
  store float %_0.i2858.lcssa1223414375.lcssa15020, ptr %713, align 4
  store float %_0.i3232.lcssa1223514393.lcssa15042, ptr %715, align 4
  store float %_0.i2862.lcssa1219414340, ptr %705, align 4
  store float %_0.i3236.lcssa1221014358, ptr %707, align 4
  store float %_0.i2858.lcssa1223414376, ptr %713, align 4
  store float %_0.i3232.lcssa1223514394, ptr %715, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_213.i, i32 noundef %_7.i8.i259.i, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #33, !dbg !23817, !noalias !23818
  unreachable, !dbg !23817

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3202
  %_0.i2396 = fdiv float %_8.i.i, %_0.i3490, !dbg !23826
  %_3.i1963 = fcmp uge float %_8.i.i, %_0.i3490, !dbg !23828
  %_0.i3470 = select i1 %_3.i1963, float 1.000000e+00, float %_0.i2396, !dbg !23830
  %_17.i12.i.i = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %_213.i, !dbg !23832
  store float %_0.i3470, ptr %_17.i12.i.i, align 4, !dbg !23834, !alias.scope !23836, !noalias !23839
  %or.cond.i1319.not = icmp ult i32 %_220.i, %_54.1.i.i, !dbg !23840
  br i1 %or.cond.i1319.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1323, label %bb4.i1322, !dbg !23840, !prof !10587

bb4.i1322:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !22949
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !22950
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store i32 %storemerge.i1245.lcssa88389047.lcssa14891, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88659083.lcssa14912, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89509119.lcssa14933, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89779155.lcssa14954, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219414339.lcssa14976, ptr %705, align 4
  store float %_0.i3236.lcssa1221014357.lcssa14998, ptr %707, align 4
  store float %_0.i2858.lcssa1223414375.lcssa15020, ptr %713, align 4
  store float %_0.i3232.lcssa1223514393.lcssa15042, ptr %715, align 4
  store float %_0.i2862.lcssa1219414340, ptr %705, align 4
  store float %_0.i3236.lcssa1221014358, ptr %707, align 4
  store float %_0.i2858.lcssa1223414376, ptr %713, align 4
  store float %_0.i3232.lcssa1223514394, ptr %715, align 4
  %_5.i1316 = add i32 %_220.i, 1, !dbg !23846
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_220.i, i32 noundef %_5.i1316, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !23847, !noalias !23848
  unreachable, !dbg !23847

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1323: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i
  %_15.i1320 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %_220.i, !dbg !23854
  %_0.i2896 = load float, ptr %_15.i1320, align 4, !dbg !23856, !alias.scope !23858, !noalias !23861, !noundef !10
  %756 = icmp eq i32 %storemerge.i12158928, 0, !dbg !23862
  %_3.i.i3773.inv = fcmp olt float %running.sroa.0.0.i12108955, %_0.i2896, !dbg !23862
  %_4.i.i3780.v = select i1 %_3.i.i3773.inv, float %running.sroa.0.0.i12108955, float %_0.i2896, !dbg !23862
  %running.sroa.0.0.i1210 = select i1 %756, float %_0.i2896, float %_4.i.i3780.v, !dbg !23862
  %_15.i1211 = add i32 %storemerge.i12158928, 1, !dbg !23863
  %complete.i1212 = icmp eq i32 %_15.i1211, %_18.i.i, !dbg !23863
  br i1 %complete.i1212, label %bb11.i1218.preheader, label %bb7.i1213, !dbg !23864

bb11.i1218.preheader:                             ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1323
  br i1 %_29.i12228809.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1234, label %bb19.i1223, !dbg !23865

bb7.i1213:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1323
  %or.cond.i1311.not = icmp ult i32 %_216.i, %_54.1.i.i, !dbg !23868
  br i1 %or.cond.i1311.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1315, label %bb4.i1314, !dbg !23868, !prof !10587

bb4.i1314:                                        ; preds = %bb7.i1213
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !22949
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !22950
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store i32 %storemerge.i1245.lcssa88389047.lcssa14891, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88659083.lcssa14912, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89509119.lcssa14933, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89779155.lcssa14954, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219414339.lcssa14976, ptr %705, align 4
  store float %_0.i3236.lcssa1221014357.lcssa14998, ptr %707, align 4
  store float %_0.i2858.lcssa1223414375.lcssa15020, ptr %713, align 4
  store float %_0.i3232.lcssa1223514393.lcssa15042, ptr %715, align 4
  store float %_0.i2862.lcssa1219414340, ptr %705, align 4
  store float %_0.i3236.lcssa1221014358, ptr %707, align 4
  store float %_0.i2858.lcssa1223414376, ptr %713, align 4
  store float %_0.i3232.lcssa1223514394, ptr %715, align 4
  %_5.i1308 = add i32 %_216.i, 1, !dbg !23873
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_216.i, i32 noundef %_5.i1308, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !23874, !noalias !23875
  unreachable, !dbg !23874

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1315: ; preds = %bb7.i1213
  %_15.i1312 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %_216.i, !dbg !23878
  %_0.i2898 = load float, ptr %_15.i1312, align 4, !dbg !23880, !alias.scope !23882, !noalias !23861, !noundef !10
  %_3.i.i3764.inv = fcmp olt float %_0.i2898, %running.sroa.0.0.i1210, !dbg !23885
  %_4.i.i3771.v = select i1 %_3.i.i3764.inv, float %_0.i2898, float %running.sroa.0.0.i1210, !dbg !23885
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1234, !dbg !23888

bb19.i1223:                                       ; preds = %bb11.i1218.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230
  %end.sroa.0.0.i12218812 = phi i32 [ %758, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230 ], [ %_220.i, %bb11.i1218.preheader ]
  %suffix.sroa.0.0.i12208811 = phi float [ %_4.i.i3762.v, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230 ], [ %_0.i2896, %bb11.i1218.preheader ]
  %iter.sroa.0.0.i12198810 = phi i32 [ %_30.i1224, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230 ], [ 0, %bb11.i1218.preheader ]
  %or.cond.i1295.not = icmp ult i32 %end.sroa.0.0.i12218812, %_54.1.i.i, !dbg !23889
  br i1 %or.cond.i1295.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230, label %bb4.i1298, !dbg !23889, !prof !10587

bb4.i1298:                                        ; preds = %bb19.i1223
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !22949
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !22950
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store i32 %storemerge.i1245.lcssa88389047.lcssa14891, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88659083.lcssa14912, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89509119.lcssa14933, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89779155.lcssa14954, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219414339.lcssa14976, ptr %705, align 4
  store float %_0.i3236.lcssa1221014357.lcssa14998, ptr %707, align 4
  store float %_0.i2858.lcssa1223414375.lcssa15020, ptr %713, align 4
  store float %_0.i3232.lcssa1223514393.lcssa15042, ptr %715, align 4
  store float %_0.i2862.lcssa1219414340, ptr %705, align 4
  store float %_0.i3236.lcssa1221014358, ptr %707, align 4
  store float %_0.i2858.lcssa1223414376, ptr %713, align 4
  store float %_0.i3232.lcssa1223514394, ptr %715, align 4
  %_5.i1292 = add i32 %end.sroa.0.0.i12218812, 1, !dbg !23894
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %end.sroa.0.0.i12218812, i32 noundef %_5.i1292, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !23895, !noalias !23896
  unreachable, !dbg !23895

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230: ; preds = %bb19.i1223
  %_30.i1224 = add nuw i32 %iter.sroa.0.0.i12198810, 1, !dbg !23899
  %_15.i1296 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %end.sroa.0.0.i12218812, !dbg !23902
  %_0.i2902 = load float, ptr %_15.i1296, align 4, !dbg !23904, !alias.scope !23906, !noalias !23861, !noundef !10
  %_3.i.i3755.inv = fcmp olt float %suffix.sroa.0.0.i12208811, %_0.i2902, !dbg !23909
  %_4.i.i3762.v = select i1 %_3.i.i3755.inv, float %suffix.sroa.0.0.i12208811, float %_0.i2902, !dbg !23909
  store float %_4.i.i3762.v, ptr %_15.i1296, align 4, !dbg !23912, !alias.scope !23915, !noalias !23861
  %757 = icmp eq i32 %end.sroa.0.0.i12218812, 0, !dbg !23918
  %spec.store.select.i1232 = select i1 %757, i32 %ring.i, i32 %end.sroa.0.0.i12218812, !dbg !23918
  %758 = add i32 %spec.store.select.i1232, -1, !dbg !23919
  %exitcond11966.not = icmp eq i32 %_30.i1224, %_18.i.i, !dbg !23920
  br i1 %exitcond11966.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1234, label %bb19.i1223, !dbg !23865

_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1234: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230, %bb11.i1218.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1315
  %storemerge.i1215 = phi i32 [ %_15.i1211, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1315 ], [ 0, %bb11.i1218.preheader ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230 ], !dbg !23922
  %running.sroa.0.1.i1216 = phi float [ %_4.i.i3771.v, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1315 ], [ %running.sroa.0.0.i1210, %bb11.i1218.preheader ], [ %running.sroa.0.0.i1210, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230 ], !dbg !23923
  %_0.i2819 = fmul float %running.sroa.0.1.i1216, 1.638400e+04, !dbg !23924
  %759 = tail call noundef float @llvm.floor.f32(float %_0.i2819), !dbg !23926
  %_0.i2818 = fmul float %759, 0x3F10000000000000, !dbg !23930
  %or.cond.i1431.not = icmp ult i32 %_222.i, %_56.1.i.i, !dbg !23932
  br i1 %or.cond.i1431.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1435, label %bb4.i1434, !dbg !23932, !prof !10587

bb4.i1434:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1234
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !22949
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !22950
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store i32 %storemerge.i1245.lcssa88389047.lcssa14891, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88659083.lcssa14912, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89509119.lcssa14933, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89779155.lcssa14954, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219414339.lcssa14976, ptr %705, align 4
  store float %_0.i3236.lcssa1221014357.lcssa14998, ptr %707, align 4
  store float %_0.i2858.lcssa1223414375.lcssa15020, ptr %713, align 4
  store float %_0.i3232.lcssa1223514393.lcssa15042, ptr %715, align 4
  store float %_0.i2862.lcssa1219414340, ptr %705, align 4
  store float %_0.i3236.lcssa1221014358, ptr %707, align 4
  store float %_0.i2858.lcssa1223414376, ptr %713, align 4
  store float %_0.i3232.lcssa1223514394, ptr %715, align 4
  %_5.i1428 = add i32 %_222.i, 1, !dbg !23937
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_222.i, i32 noundef %_5.i1428, i32 noundef range(i32 0, 536870912) %_56.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !23938, !noalias !23939
  unreachable, !dbg !23938

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1435: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1234
  %_15.i1432 = getelementptr inbounds nuw float, ptr %_56.0.i.i, i32 %_222.i, !dbg !23942
  %_0.i2868 = load float, ptr %_15.i1432, align 4, !dbg !23944, !alias.scope !23946, !noalias !23949, !noundef !10
  %_0.i2381 = fadd float %_0.i2818, %_0.i28588983, !dbg !23950
  %_0.i2858 = fsub float %_0.i2381, %_0.i2868, !dbg !23952
  %_8.not.i3.i.i = icmp ugt i32 %_7.i8.i259.i, %_56.1.i.i
  br i1 %_8.not.i3.i.i, label %bb4.i6.i.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i, !dbg !23954, !prof !4694

bb4.i6.i.i:                                       ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1435
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !22949
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !22950
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store i32 %storemerge.i1245.lcssa88389047.lcssa14891, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88659083.lcssa14912, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89509119.lcssa14933, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89779155.lcssa14954, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219414339.lcssa14976, ptr %705, align 4
  store float %_0.i3236.lcssa1221014357.lcssa14998, ptr %707, align 4
  store float %_0.i2858.lcssa1223414375.lcssa15020, ptr %713, align 4
  store float %_0.i3232.lcssa1223514393.lcssa15042, ptr %715, align 4
  store float %_0.i2862.lcssa1219414340, ptr %705, align 4
  store float %_0.i3236.lcssa1221014358, ptr %707, align 4
  store float %_0.i2858.lcssa1223414376, ptr %713, align 4
  store float %_0.i3232.lcssa1223514394, ptr %715, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_213.i, i32 noundef %_7.i8.i259.i, i32 noundef range(i32 0, 536870912) %_56.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #33, !dbg !23959, !noalias !23960
  unreachable, !dbg !23959

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1435
  %_17.i5.i.i = getelementptr inbounds nuw float, ptr %_56.0.i.i, i32 %_213.i, !dbg !23963
  store float %_0.i2818, ptr %_17.i5.i.i, align 4, !dbg !23965, !alias.scope !23967, !noalias !23949
  %_6.not.i1422 = icmp ugt i32 %_5.i1436, %_58.1.i.i
  br i1 %_6.not.i1422, label %bb4.i1426, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195, !dbg !23970, !prof !4694

bb4.i1426:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !22949
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !22950
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store i32 %storemerge.i1245.lcssa88389047.lcssa14891, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88659083.lcssa14912, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89509119.lcssa14933, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89779155.lcssa14954, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219414339.lcssa14976, ptr %705, align 4
  store float %_0.i3236.lcssa1221014357.lcssa14998, ptr %707, align 4
  store float %_0.i2858.lcssa1223414375.lcssa15020, ptr %713, align 4
  store float %_0.i3232.lcssa1223514393.lcssa15042, ptr %715, align 4
  store float %_0.i2862.lcssa1219414340, ptr %705, align 4
  store float %_0.i3236.lcssa1221014358, ptr %707, align 4
  store float %_0.i2858.lcssa1223414376, ptr %713, align 4
  store float %_0.i3232.lcssa1223514394, ptr %715, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_214.i, i32 noundef %_5.i1436, i32 noundef range(i32 0, 536870912) %_58.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !23975, !noalias !23976
  unreachable, !dbg !23975

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i
  %_0.i2395 = fdiv float %_0.i2858, %_37.i.i, !dbg !23979
  %_0.i2857 = fsub float 1.000000e+00, %_0.i2395, !dbg !23981
  %_0.i2856 = fsub float %_0.i2857, %_0.i32329012, !dbg !23983
  %_4.i2411 = fmul float %_9.i.i, %_0.i2856, !dbg !23985
  %_0.i2412 = fadd float %_0.i32329012, %_4.i2411, !dbg !23985
  %_3.i.i3701.inv = fcmp ogt float %_0.i2857, %_0.i2412, !dbg !23987
  %_4.i.i3708.v = select i1 %_3.i.i3701.inv, float %_0.i2857, float %_0.i2412, !dbg !23987
  %760 = tail call noundef float @llvm.fabs.f32(float %_4.i.i3708.v), !dbg !23990
  %761 = fcmp uge float %760, 0x3BC79CA100000000, !dbg !23993
  %_0.i3232 = select i1 %761, float %_4.i.i3708.v, float 0.000000e+00, !dbg !23995
  %_0.i2855 = fsub float 1.000000e+00, %_0.i3232, !dbg !23996
  %_15.i1424 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i32 %_214.i, !dbg !23998
  %_0.i2870 = load float, ptr %_15.i1424, align 4, !dbg !24000, !alias.scope !24002, !noalias !23949, !noundef !10
  store float %_0.i3046, ptr %_15.i1424, align 4, !dbg !24005, !alias.scope !24008, !noalias !23949
  %_0.i2817 = fmul float %_0.i2855, %_0.i2870, !dbg !24011
  %_6.i3458 = bitcast float %_0.i2870 to i32, !dbg !24013
  %_5.i3459 = and i32 %_6.i3458, %all.sroa.0.0.i, !dbg !24016
  %_8.i3460 = bitcast float %_0.i2817 to i32, !dbg !24017
  %_7.i3462 = and i32 %_9.i3474, %_8.i3460, !dbg !24019
  %_4.i3463 = or disjoint i32 %_7.i3462, %_5.i3459, !dbg !24016
  store i32 %_4.i3463, ptr %data.i5.i.i.i.i.i4201, align 4, !dbg !24020, !alias.scope !24022, !noalias !24025
  %exitcond11979.not = icmp eq i32 %_206.0.i, %umin11978, !dbg !23507
  br i1 %exitcond11979.not, label %bb67.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048, !dbg !23507

bb67.i:                                           ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4183
  %_0.i3232.lcssa1223514393 = phi float [ %_0.i3232.lcssa1223514394, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4183 ], [ %_0.i3232, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %_0.i2858.lcssa1223414375 = phi float [ %_0.i2858.lcssa1223414376, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4183 ], [ %_0.i2858, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %_0.i3236.lcssa1221014357 = phi float [ %_0.i3236.lcssa1221014358, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4183 ], [ %_0.i3236, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %_0.i2862.lcssa1219414339 = phi float [ %_0.i2862.lcssa1219414340, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4183 ], [ %_0.i2862, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %running.sroa.0.0.i1210.lcssa89779155 = phi float [ %running.sroa.0.0.i1210.lcssa89779156, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4183 ], [ %running.sroa.0.0.i1210, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %storemerge.i1215.lcssa89509119 = phi i32 [ %storemerge.i1215.lcssa89509120, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4183 ], [ %storemerge.i1215, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %running.sroa.0.0.i1240.lcssa88659083 = phi float [ %running.sroa.0.0.i1240.lcssa88659084, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4183 ], [ %running.sroa.0.0.i1240, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %storemerge.i1245.lcssa88389047 = phi i32 [ %storemerge.i1245.lcssa88389048, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4183 ], [ %storemerge.i1245, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %_139.i = add i32 %run.sroa.0.0.i1836, %ring_cursor.sroa.0.1.i9041, !dbg !24026
  %_212.not.i = icmp ult i32 %_139.i, %ring.i, !dbg !24027
  %762 = select i1 %_212.not.i, i32 0, i32 %ring.i, !dbg !24027
  %ring_cursor.sroa.0.2.i = sub nuw i32 %_139.i, %762, !dbg !24027
  %_141.i = add i32 %run.sroa.0.0.i1836, %main_cursor.sroa.0.1.i9042, !dbg !24030
  %_223.not.i = icmp ult i32 %_141.i, %main.i, !dbg !24031
  %763 = select i1 %_223.not.i, i32 0, i32 %main.i, !dbg !24031
  %main_cursor.sroa.0.2.i = sub nuw i32 %_141.i, %763, !dbg !24031
  %_59.i = icmp ult i32 %_83.i, %spec.store.select.i, !dbg !23408
  br i1 %_59.i, label %bb20.i, label %bb15.i.loopexit, !dbg !23408

_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit: ; preds = %bb15.i.loopexit
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !22949
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !22949
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !22950
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !22950
  store i32 %storemerge.i1245.lcssa88389047.lcssa14890, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88659083.lcssa14911, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89509119.lcssa14932, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89779155.lcssa14953, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219414339.lcssa14975, ptr %705, align 4
  store float %_0.i3236.lcssa1221014357.lcssa14997, ptr %707, align 4
  store float %_0.i2858.lcssa1223414375.lcssa15019, ptr %713, align 4
  store float %_0.i3232.lcssa1223514393.lcssa15041, ptr %715, align 4
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit, !dbg !24033

_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit, %bb6.i
  %ring_cursor.sroa.0.0.i.lcssa = phi i32 [ %_37.i, %bb6.i ], [ %ring_cursor.sroa.0.1.i.lcssa, %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit ], !dbg !22882
  %main_cursor.sroa.0.0.i.lcssa = phi i32 [ %_36.i, %bb6.i ], [ %main_cursor.sroa.0.1.i.lcssa, %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit ], !dbg !22879
  %764 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 24, !dbg !24033
  %left_prefix.i = load float, ptr %764, align 4, !dbg !24033, !noalias !22886, !noundef !10
  %765 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 28, !dbg !24034
  %left_phase.i = load i32, ptr %765, align 4, !dbg !24034, !noalias !22886, !noundef !10
  %766 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 24, !dbg !24035
  %right_prefix.i = load float, ptr %766, align 4, !dbg !24035, !noalias !22886, !noundef !10
  %767 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 28, !dbg !24036
  %right_phase.i = load i32, ptr %767, align 4, !dbg !24036, !noalias !22886, !noundef !10
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_right.i), !dbg !24037, !noalias !22886
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i), !dbg !24038, !noalias !22886
  %768 = getelementptr inbounds nuw i8, ptr %self, i32 364, !dbg !24039
  %_236.0.i = load ptr, ptr %768, align 4, !dbg !24039, !alias.scope !22853, !noalias !24041, !nonnull !10, !noundef !10
  %769 = getelementptr inbounds nuw i8, ptr %self, i32 368, !dbg !24039
  %_236.1.i = load i32, ptr %769, align 4, !dbg !24039, !alias.scope !22853, !noalias !24041, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !24042), !dbg !24045
  %_4.not.i3180 = icmp eq i32 %_236.1.i, 0, !dbg !24046
  br i1 %_4.not.i3180, label %panic.i3182, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3183, !dbg !24046

panic.i3182:                                      ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #33, !dbg !24046, !noalias !24048
  unreachable, !dbg !24046

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3183: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit
  store float %left_prefix.i, ptr %_236.0.i, align 4, !dbg !24046, !alias.scope !24042, !noalias !22857
  %_237.0.i = load ptr, ptr %68, align 4, !dbg !24049, !alias.scope !22853, !noalias !24041, !nonnull !10, !noundef !10
  %_237.1.i = load i32, ptr %69, align 4, !dbg !24049, !alias.scope !22853, !noalias !24041, !noundef !10
  %770 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i), !dbg !24050
  br i1 %770, label %bb2.i4228, label %bb6.i4220, !dbg !24050

bb6.i4220:                                        ; preds = %bb2.i4228, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3183
  %end_or_len.idx.i4221 = shl nuw nsw i32 %_237.1.i, 2, !dbg !24054
  %end_or_len.i4222 = getelementptr inbounds nuw i8, ptr %_237.0.i, i32 %end_or_len.idx.i4221, !dbg !24054
  %_293.i4223 = icmp eq i32 %_237.1.i, 0, !dbg !24058
  br i1 %_293.i4223, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4234, label %bb10.i4224, !dbg !24061

bb2.i4228:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3183
  %bytes1.sroa.0.0.zext.i4229 = and i32 %left_phase.i, 255, !dbg !24062
  %bytes1.sroa.0.0.isplat.i4230 = mul nuw i32 %bytes1.sroa.0.0.zext.i4229, 16843009, !dbg !24062
  %_5.i4231 = icmp eq i32 %left_phase.i, %bytes1.sroa.0.0.isplat.i4230, !dbg !24063
  br i1 %_5.i4231, label %bb3.i4232, label %bb6.i4220, !dbg !24063

bb3.i4232:                                        ; preds = %bb2.i4228
  %bytes.sroa.0.0.extract.trunc.i4233 = trunc i32 %left_phase.i to i8, !dbg !24064
  %771 = shl nuw nsw i32 %_237.1.i, 2, !dbg !24066
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %_237.0.i, i8 %bytes.sroa.0.0.extract.trunc.i4233, i32 %771, i1 false), !dbg !24066, !alias.scope !24067, !noalias !22857
  br label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4234, !dbg !24070

bb10.i4224:                                       ; preds = %bb6.i4220, %bb10.i4224
  %iter.sroa.0.04.i4225 = phi ptr [ %_38.i4226, %bb10.i4224 ], [ %_237.0.i, %bb6.i4220 ]
  %_38.i4226 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i4225, i32 4, !dbg !24071
  store i32 %left_phase.i, ptr %iter.sroa.0.04.i4225, align 4, !dbg !24073, !alias.scope !24067, !noalias !22857
  %_29.i4227 = icmp eq ptr %_38.i4226, %end_or_len.i4222, !dbg !24058
  br i1 %_29.i4227, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4234, label %bb10.i4224, !dbg !24061

_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4234: ; preds = %bb10.i4224, %bb6.i4220, %bb3.i4232
  %772 = getelementptr inbounds nuw i8, ptr %self, i32 464, !dbg !24074
  %_238.0.i = load ptr, ptr %772, align 4, !dbg !24074, !alias.scope !22855, !noalias !24075, !nonnull !10, !noundef !10
  %773 = getelementptr inbounds nuw i8, ptr %self, i32 468, !dbg !24074
  %_238.1.i = load i32, ptr %773, align 4, !dbg !24074, !alias.scope !22855, !noalias !24075, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !24076), !dbg !24079
  %_4.not.i3176 = icmp eq i32 %_238.1.i, 0, !dbg !24080
  br i1 %_4.not.i3176, label %panic.i3178, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3179, !dbg !24080

panic.i3178:                                      ; preds = %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4234
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #33, !dbg !24080, !noalias !24082
  unreachable, !dbg !24080

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3179: ; preds = %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4234
  store float %right_prefix.i, ptr %_238.0.i, align 4, !dbg !24080, !alias.scope !24076, !noalias !22857
  %_239.0.i = load ptr, ptr %77, align 4, !dbg !24083, !alias.scope !22855, !noalias !24075, !nonnull !10, !noundef !10
  %_239.1.i = load i32, ptr %78, align 4, !dbg !24083, !alias.scope !22855, !noalias !24075, !noundef !10
  %774 = tail call i1 @llvm.is.constant.i32(i32 %right_phase.i), !dbg !24084
  br i1 %774, label %bb2.i4243, label %bb6.i4235, !dbg !24084

bb6.i4235:                                        ; preds = %bb2.i4243, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3179
  %end_or_len.idx.i4236 = shl nuw nsw i32 %_239.1.i, 2, !dbg !24087
  %end_or_len.i4237 = getelementptr inbounds nuw i8, ptr %_239.0.i, i32 %end_or_len.idx.i4236, !dbg !24087
  %_293.i4238 = icmp eq i32 %_239.1.i, 0, !dbg !24091
  br i1 %_293.i4238, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4249, label %bb10.i4239, !dbg !24094

bb2.i4243:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3179
  %bytes1.sroa.0.0.zext.i4244 = and i32 %right_phase.i, 255, !dbg !24095
  %bytes1.sroa.0.0.isplat.i4245 = mul nuw i32 %bytes1.sroa.0.0.zext.i4244, 16843009, !dbg !24095
  %_5.i4246 = icmp eq i32 %right_phase.i, %bytes1.sroa.0.0.isplat.i4245, !dbg !24096
  br i1 %_5.i4246, label %bb3.i4247, label %bb6.i4235, !dbg !24096

bb3.i4247:                                        ; preds = %bb2.i4243
  %bytes.sroa.0.0.extract.trunc.i4248 = trunc i32 %right_phase.i to i8, !dbg !24097
  %775 = shl nuw nsw i32 %_239.1.i, 2, !dbg !24099
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %_239.0.i, i8 %bytes.sroa.0.0.extract.trunc.i4248, i32 %775, i1 false), !dbg !24099, !alias.scope !24100, !noalias !22857
  br label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4249, !dbg !24103

bb10.i4239:                                       ; preds = %bb6.i4235, %bb10.i4239
  %iter.sroa.0.04.i4240 = phi ptr [ %_38.i4241, %bb10.i4239 ], [ %_239.0.i, %bb6.i4235 ]
  %_38.i4241 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i4240, i32 4, !dbg !24104
  store i32 %right_phase.i, ptr %iter.sroa.0.04.i4240, align 4, !dbg !24106, !alias.scope !24100, !noalias !22857
  %_29.i4242 = icmp eq ptr %_38.i4241, %end_or_len.i4237, !dbg !24091
  br i1 %_29.i4242, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4249, label %bb10.i4239, !dbg !24094

_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4249: ; preds = %bb10.i4239, %bb6.i4235, %bb3.i4247
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %hot_left.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33) #32, !dbg !24107
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %hot_right.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34) #32, !dbg !24108
  store i32 %main_cursor.sroa.0.0.i.lcssa, ptr %_35, align 4, !dbg !24109, !alias.scope !22857, !noalias !22881
  store i32 %ring_cursor.sroa.0.0.i.lcssa, ptr %662, align 4, !dbg !24110, !alias.scope !22857, !noalias !22881
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i), !dbg !24111, !noalias !22886
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i), !dbg !24112, !noalias !22886
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockfEB2_.exit, !dbg !22850

_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockfEB2_.exit: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit, %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit, %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4118, %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4249
  br i1 %quiet.sroa.0.0.off04438, label %bb28, label %bb40, !dbg !24113

bb22:                                             ; preds = %bb20
  tail call void @llvm.experimental.noalias.scope.decl(metadata !24114), !dbg !24117
  %776 = getelementptr inbounds nuw i8, ptr %self, i32 380, !dbg !24118
  %_40.0.i = load ptr, ptr %776, align 4, !dbg !24118, !alias.scope !24114, !nonnull !10, !noundef !10
  %777 = getelementptr inbounds nuw i8, ptr %self, i32 384, !dbg !24118
  %_40.1.i = load i32, ptr %777, align 4, !dbg !24118, !alias.scope !24114, !noundef !10
  %778 = getelementptr inbounds nuw i8, ptr %self, i32 412, !dbg !24120
  %_41.0.i = load ptr, ptr %778, align 4, !dbg !24120, !alias.scope !24114, !nonnull !10, !noundef !10
  %779 = getelementptr inbounds nuw i8, ptr %self, i32 416, !dbg !24120
  %_41.1.i = load i32, ptr %779, align 4, !dbg !24120, !alias.scope !24114, !noundef !10
  %spec.store.select.i.i = tail call i32 @llvm.umin.i32(i32 %_41.1.i, i32 %_40.1.i), !dbg !24121
  %_2.i6.not.i = icmp eq i32 %spec.store.select.i.i, 0, !dbg !24127
  br i1 %_2.i6.not.i, label %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb3.i4250, !dbg !24127

bb3.i4250:                                        ; preds = %bb22, %bb5.i4252
  %iter.sroa.8.07.i = phi i32 [ %780, %bb5.i4252 ], [ 0, %bb22 ]
  %_3.i1.i.i = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i, i32 %iter.sroa.8.07.i, !dbg !24130
  %_14.i4251 = load i32, ptr %_3.i1.i.i, align 4, !dbg !24133, !noalias !24114, !noundef !10
  %_20.i = icmp eq i32 %_14.i4251, 0, !dbg !24134
  br i1 %_20.i, label %panic.i4259, label %bb5.i4252, !dbg !24134

bb5.i4252:                                        ; preds = %bb3.i4250
  %_3.i.i.i4253 = getelementptr inbounds nuw i32, ptr %_40.0.i, i32 %iter.sroa.8.07.i, !dbg !24135
  %780 = add nuw i32 %iter.sroa.8.07.i, 1, !dbg !24138
  %_18.i4254 = load i32, ptr %_3.i.i.i4253, align 4, !dbg !24139, !noalias !24114, !noundef !10
  %_19.i4255 = urem i32 %frames, %_14.i4251, !dbg !24134
  %_16.i4256 = add i32 %_19.i4255, %_18.i4254, !dbg !24140
  %_15.i4257 = urem i32 %_16.i4256, %_14.i4251, !dbg !24141
  store i32 %_15.i4257, ptr %_3.i.i.i4253, align 4, !dbg !24142, !noalias !24114
  %exitcond.not.i = icmp eq i32 %780, %spec.store.select.i.i, !dbg !24127
  br i1 %exitcond.not.i, label %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb3.i4250, !dbg !24127

panic.i4259:                                      ; preds = %bb3.i4250
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_f5b0427df9b659e554a697ca46ce8b5a) #33, !dbg !24134, !noalias !24114
  unreachable, !dbg !24134

_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit: ; preds = %bb5.i4252, %bb22
  tail call void @llvm.experimental.noalias.scope.decl(metadata !24143), !dbg !24146
  %781 = getelementptr inbounds nuw i8, ptr %self, i32 480, !dbg !24147
  %_40.0.i4260 = load ptr, ptr %781, align 4, !dbg !24147, !alias.scope !24143, !nonnull !10, !noundef !10
  %782 = getelementptr inbounds nuw i8, ptr %self, i32 484, !dbg !24147
  %_40.1.i4261 = load i32, ptr %782, align 4, !dbg !24147, !alias.scope !24143, !noundef !10
  %783 = getelementptr inbounds nuw i8, ptr %self, i32 512, !dbg !24149
  %_41.0.i4262 = load ptr, ptr %783, align 4, !dbg !24149, !alias.scope !24143, !nonnull !10, !noundef !10
  %784 = getelementptr inbounds nuw i8, ptr %self, i32 516, !dbg !24149
  %_41.1.i4263 = load i32, ptr %784, align 4, !dbg !24149, !alias.scope !24143, !noundef !10
  %spec.store.select.i.i4264 = tail call i32 @llvm.umin.i32(i32 %_41.1.i4263, i32 %_40.1.i4261), !dbg !24150
  %_2.i6.not.i4265 = icmp eq i32 %spec.store.select.i.i4264, 0, !dbg !24156
  br i1 %_2.i6.not.i4265, label %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit4280, label %bb3.i4266, !dbg !24156

bb3.i4266:                                        ; preds = %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, %bb5.i4271
  %iter.sroa.8.07.i4267 = phi i32 [ %785, %bb5.i4271 ], [ 0, %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit ]
  %_3.i1.i.i4268 = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i4262, i32 %iter.sroa.8.07.i4267, !dbg !24159
  %_14.i4269 = load i32, ptr %_3.i1.i.i4268, align 4, !dbg !24162, !noalias !24143, !noundef !10
  %_20.i4270 = icmp eq i32 %_14.i4269, 0, !dbg !24163
  br i1 %_20.i4270, label %panic.i4279, label %bb5.i4271, !dbg !24163

bb5.i4271:                                        ; preds = %bb3.i4266
  %_3.i.i.i4272 = getelementptr inbounds nuw i32, ptr %_40.0.i4260, i32 %iter.sroa.8.07.i4267, !dbg !24164
  %785 = add nuw i32 %iter.sroa.8.07.i4267, 1, !dbg !24167
  %_18.i4273 = load i32, ptr %_3.i.i.i4272, align 4, !dbg !24168, !noalias !24143, !noundef !10
  %_19.i4274 = urem i32 %frames, %_14.i4269, !dbg !24163
  %_16.i4275 = add i32 %_19.i4274, %_18.i4273, !dbg !24169
  %_15.i4276 = urem i32 %_16.i4275, %_14.i4269, !dbg !24170
  store i32 %_15.i4276, ptr %_3.i.i.i4272, align 4, !dbg !24171, !noalias !24143
  %exitcond.not.i4277 = icmp eq i32 %785, %spec.store.select.i.i4264, !dbg !24156
  br i1 %exitcond.not.i4277, label %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit4280, label %bb3.i4266, !dbg !24156

panic.i4279:                                      ; preds = %bb3.i4266
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_f5b0427df9b659e554a697ca46ce8b5a) #33, !dbg !24163, !noalias !24143
  unreachable, !dbg !24163

_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit4280: ; preds = %bb5.i4271, %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit
  %786 = getelementptr inbounds nuw i8, ptr %self, i32 528, !dbg !24172
  %_29.val = load i32, ptr %786, align 4, !dbg !24172
  %787 = getelementptr inbounds nuw i8, ptr %self, i32 532, !dbg !24172
  %_29.val3836 = load i32, ptr %787, align 4, !dbg !24172, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !24173), !dbg !24172
  %_10.i4281 = icmp eq i32 %_29.val3836, 0, !dbg !24176
  br i1 %_10.i4281, label %panic.i4291, label %bb1.i4282, !dbg !24176

bb1.i4282:                                        ; preds = %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit4280
  %_28 = getelementptr inbounds nuw i8, ptr %self, i32 104, !dbg !24178
  %_7.i = load i32, ptr %_28, align 4, !dbg !24179, !alias.scope !24173, !noundef !10
  %_8.i4283 = urem i32 %frames, %_29.val3836, !dbg !24176
  %_5.i4284 = add i32 %_8.i4283, %_7.i, !dbg !24180
  %_4.i4285 = urem i32 %_5.i4284, %_29.val3836, !dbg !24181
  store i32 %_4.i4285, ptr %_28, align 4, !dbg !24182, !alias.scope !24173
  %_17.i4286 = icmp eq i32 %_29.val, 0, !dbg !24183
  br i1 %_17.i4286, label %panic2.i, label %_RNvMs1_CsjLJhryqjeDL_17true_peak_limiterNtB5_7Cursors7advance.exit, !dbg !24183

panic.i4291:                                      ; preds = %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit4280
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c7e64e9e8489bc8e648659d0098d136c) #33, !dbg !24176, !noalias !24173
  unreachable, !dbg !24176

panic2.i:                                         ; preds = %bb1.i4282
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_34b23597fad9d85cb3bee6d64ebf77d5) #33, !dbg !24183, !noalias !24173
  unreachable, !dbg !24183

_RNvMs1_CsjLJhryqjeDL_17true_peak_limiterNtB5_7Cursors7advance.exit: ; preds = %bb1.i4282
  %788 = getelementptr inbounds nuw i8, ptr %self, i32 108, !dbg !24184
  %_14.i4288 = load i32, ptr %788, align 4, !dbg !24184, !alias.scope !24173, !noundef !10
  %_15.i4289 = urem i32 %frames, %_29.val, !dbg !24183
  %_12.i = add i32 %_15.i4289, %_14.i4288, !dbg !24185
  %_11.i4290 = urem i32 %_12.i, %_29.val, !dbg !24186
  store i32 %_11.i4290, ptr %788, align 4, !dbg !24187, !alias.scope !24173
  br label %bb42, !dbg !24188

bb28:                                             ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockfEB2_.exit
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_37 = tail call noundef zeroext i1 @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #32, !dbg !24189
  br i1 %_37, label %bb30, label %bb40, !dbg !24190

bb30:                                             ; preds = %bb28
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_39 = tail call noundef zeroext i1 @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #32, !dbg !24191
  br i1 %_39, label %bb32, label %bb40, !dbg !24192

bb32:                                             ; preds = %bb30
  %_80.not = icmp ugt i32 %frames, %left_io.1
  br i1 %_80.not, label %bb51, label %bb1.i4292, !dbg !24193, !prof !4694

bb51:                                             ; preds = %bb32
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef %frames, i32 noundef %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_8457596f73b1d71f2ddde42261d136f3) #33, !dbg !24201
  unreachable, !dbg !24201

bb1.i4292:                                        ; preds = %bb32, %bb12.i4306
  %io.sroa.5.0.i4293 = phi i32 [ %len.i.i.i4299, %bb12.i4306 ], [ %frames, %bb32 ]
  %io.sroa.0.0.i4294 = phi ptr [ %data.i.i.i4298, %bb12.i4306 ], [ %left_io.0, %bb32 ]
  %789 = icmp eq i32 %io.sroa.5.0.i4293, 0, !dbg !24202
  br i1 %789, label %bb34, label %bb13.preheader.i4295, !dbg !24202

bb13.preheader.i4295:                             ; preds = %bb1.i4292
  %spec.store.select.i4296 = tail call i32 @llvm.umin.i32(i32 %io.sroa.5.0.i4293, i32 32), !dbg !24205
  %data.i.i.idx.i4297 = shl nuw nsw i32 %spec.store.select.i4296, 2, !dbg !24208
  %data.i.i.i4298 = getelementptr inbounds nuw i8, ptr %io.sroa.0.0.i4294, i32 %data.i.i.idx.i4297, !dbg !24208
  br label %bb13.i4300, !dbg !24213

bb13.i4300:                                       ; preds = %bb13.i4300, %bb13.preheader.i4295
  %iter.sroa.0.08.i4301 = phi ptr [ %_35.i4303, %bb13.i4300 ], [ %io.sroa.0.0.i4294, %bb13.preheader.i4295 ]
  %bits.sroa.0.07.i4302 = phi i32 [ %790, %bb13.i4300 ], [ 0, %bb13.preheader.i4295 ]
  %_35.i4303 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i4301, i32 4, !dbg !24215
  %_95.i4304 = load i32, ptr %iter.sroa.0.08.i4301, align 4, !dbg !24217, !alias.scope !24218, !noundef !10
  %790 = or i32 %_95.i4304, %bits.sroa.0.07.i4302, !dbg !24221
  %_29.i4305 = icmp eq ptr %_35.i4303, %data.i.i.i4298, !dbg !24222
  br i1 %_29.i4305, label %bb12.i4306, label %bb13.i4300, !dbg !24213

bb12.i4306:                                       ; preds = %bb13.i4300
  %len.i.i.i4299 = sub nuw nsw i32 %io.sroa.5.0.i4293, %spec.store.select.i4296, !dbg !24224
  %791 = icmp eq i32 %790, 0, !dbg !24225
  br i1 %791, label %bb1.i4292, label %bb40, !dbg !24225

bb34:                                             ; preds = %bb1.i4292
  %_88.not = icmp ugt i32 %frames, %right_io.1, !dbg !24226
  br i1 %_88.not, label %bb54, label %bb1.i4322, !dbg !24226, !prof !902

bb40:                                             ; preds = %bb12.i4306, %bb12.i4336, %bb1.i4322, %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockfEB2_.exit, %bb28, %bb30
  %_36.sroa.0.0.off0 = phi i1 [ false, %bb12.i4336 ], [ false, %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockfEB2_.exit ], [ false, %bb30 ], [ false, %bb28 ], [ true, %bb1.i4322 ], [ false, %bb12.i4306 ]
  %792 = zext i1 %_36.sroa.0.0.off0 to i8, !dbg !24232
  store i8 %792, ptr %38, align 8, !dbg !24232
  %793 = load i8, ptr %2, align 8, !dbg !24233, !range !4765, !noundef !10
  store i8 %793, ptr %0, align 1, !dbg !24234
  call void @llvm.lifetime.start.p0(ptr nonnull %shape), !dbg !24235
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(12) %shape, ptr noundef nonnull align 4 dereferenceable(12) %_32, i32 12, i1 false), !dbg !24236
  %794 = getelementptr inbounds nuw i8, ptr %self, i32 56, !dbg !24237
  %795 = load i32, ptr %794, align 8, !dbg !24237, !noundef !10
  %_53 = getelementptr inbounds nuw i8, ptr %self, i32 112, !dbg !24239
  %796 = getelementptr inbounds nuw i8, ptr %self, i32 88, !dbg !24246
  %_99.0 = load ptr, ptr %796, align 8, !dbg !24246, !nonnull !10, !noundef !10
  %797 = getelementptr inbounds nuw i8, ptr %self, i32 92, !dbg !24246
  %_99.1 = load i32, ptr %797, align 4, !dbg !24246, !noundef !10
  %798 = getelementptr inbounds nuw i8, ptr %self, i32 96, !dbg !24246
  %_100.0 = load ptr, ptr %798, align 8, !dbg !24246, !nonnull !10, !noundef !10
  %799 = getelementptr inbounds nuw i8, ptr %self, i32 100, !dbg !24246
  %_100.1 = load i32, ptr %799, align 4, !dbg !24246, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !24247), !dbg !24250
  tail call void @llvm.experimental.noalias.scope.decl(metadata !24251), !dbg !24250
  tail call void @llvm.experimental.noalias.scope.decl(metadata !24253), !dbg !24250
  %_22.not.i1463.i = icmp eq i32 %left_io.1, 0, !dbg !24255
  br i1 %_22.not.i1463.i, label %bb6.i.preheader.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i, !dbg !24255

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i: ; preds = %bb40, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i
  %ok.sroa.0.0.i1366.i = phi i32 [ %_0.i33.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i ], [ -1, %bb40 ]
  %iter.sroa.0.0.i1265.i = phi ptr [ %_27.i16.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i ], [ %left_io.0, %bb40 ]
  %iter.sroa.5.0.i1164.i = phi i32 [ %_28.i17.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i ], [ %left_io.1, %bb40 ]
  %_27.i16.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i1265.i, i32 4, !dbg !24268
  %_28.i17.i = add nsw i32 %iter.sroa.5.0.i1164.i, -1, !dbg !24275
  %_0.i28.i = load float, ptr %iter.sroa.0.0.i1265.i, align 4, !dbg !24276, !alias.scope !24279, !noalias !24282, !noundef !10
  %800 = tail call noundef float @llvm.fabs.f32(float %_0.i28.i), !dbg !24284
  %_3.i.i4309 = fcmp olt float %800, 0x46293E5940000000, !dbg !24287
  %_0.i33.i = select i1 %_3.i.i4309, i32 %ok.sroa.0.0.i1366.i, i32 0, !dbg !24290
  %_22.not.i14.i = icmp eq i32 %_28.i17.i, 0, !dbg !24255
  br i1 %_22.not.i14.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockfECsjLJhryqjeDL_17true_peak_limiter.exit25.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i, !dbg !24255

_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockfECsjLJhryqjeDL_17true_peak_limiter.exit25.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i
  %801 = icmp eq i32 %_0.i33.i, -1, !dbg !24293
  br i1 %801, label %bb6.i.preheader.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i, !dbg !24296

bb6.i.preheader.i:                                ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockfECsjLJhryqjeDL_17true_peak_limiter.exit25.i, %bb40
  %_22.not.i67.i = icmp eq i32 %right_io.1, 0, !dbg !24297
  br i1 %_22.not.i67.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockfNCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit32.i, !dbg !24297

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit32.i: ; preds = %bb6.i.preheader.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit32.i
  %ok.sroa.0.0.i70.i = phi i32 [ %_0.i34.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit32.i ], [ -1, %bb6.i.preheader.i ]
  %iter.sroa.0.0.i69.i = phi ptr [ %_27.i.i4320, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit32.i ], [ %right_io.0, %bb6.i.preheader.i ]
  %iter.sroa.5.0.i68.i = phi i32 [ %_28.i.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit32.i ], [ %right_io.1, %bb6.i.preheader.i ]
  %_27.i.i4320 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i69.i, i32 4, !dbg !24301
  %_28.i.i = add nsw i32 %iter.sroa.5.0.i68.i, -1, !dbg !24304
  %_0.i30.i = load float, ptr %iter.sroa.0.0.i69.i, align 4, !dbg !24305, !alias.scope !24307, !noalias !24310, !noundef !10
  %802 = tail call noundef float @llvm.fabs.f32(float %_0.i30.i), !dbg !24311
  %_3.i26.i = fcmp olt float %802, 0x46293E5940000000, !dbg !24313
  %_0.i34.i = select i1 %_3.i26.i, i32 %ok.sroa.0.0.i70.i, i32 0, !dbg !24315
  %_22.not.i.i = icmp eq i32 %_28.i.i, 0, !dbg !24297
  br i1 %_22.not.i.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockfECsjLJhryqjeDL_17true_peak_limiter.exit.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit32.i, !dbg !24297

_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockfECsjLJhryqjeDL_17true_peak_limiter.exit.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit32.i
  %803 = icmp eq i32 %_0.i34.i, -1, !dbg !24317
  br i1 %803, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockfNCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit, label %bb7.i4321, !dbg !24319

bb7.i4321:                                        ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockfECsjLJhryqjeDL_17true_peak_limiter.exit.i
  br i1 %_22.not.i1463.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i, !dbg !24320

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i: ; preds = %bb7.i4321, %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockfECsjLJhryqjeDL_17true_peak_limiter.exit25.i
  br label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.i, !dbg !24320

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i
  %ok.sroa.0.012.i.i = phi i32 [ %_0.i8.i.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ -1, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i ]
  %iter.sroa.0.011.i.i = phi ptr [ %_42.i.i4310, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ %left_io.0, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i ]
  %iter.sroa.5.010.i.i = phi i32 [ %_43.i.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ %left_io.1, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i ]
  %_42.i.i4310 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.011.i.i, i32 4, !dbg !24331
  %_43.i.i = add nsw i32 %iter.sroa.5.010.i.i, -1, !dbg !24338
  %_0.i.i.i4311 = load float, ptr %iter.sroa.0.011.i.i, align 4, !dbg !24339, !alias.scope !24342, !noalias !24282, !noundef !10
  %804 = tail call noundef float @llvm.fabs.f32(float %_0.i.i.i4311), !dbg !24347
  %_3.i.i.i4312 = fcmp olt float %804, 0x46293E5940000000, !dbg !24350
  %_0.i8.i.i = select i1 %_3.i.i.i4312, i32 %ok.sroa.0.012.i.i, i32 0, !dbg !24352
  %_37.not.i.i = icmp eq i32 %_43.i.i, 0, !dbg !24320
  br i1 %_37.not.i.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.i, !dbg !24320

_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.i
  %805 = and i32 %_0.i8.i.i, 1065353216, !dbg !24354
  %806 = icmp ne i32 %805, 1065353216, !dbg !24357
  %807 = zext i1 %806 to i32, !dbg !24357
  %_37.not9.i40.i = icmp eq i32 %right_io.1, 0, !dbg !24361
  br i1 %_37.not9.i40.i, label %bb12.i.preheader.thread.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i, !dbg !24361

bb12.i.preheader.thread.i:                        ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit.i
  %808 = getelementptr inbounds nuw i8, ptr %self, i32 120, !dbg !24365
  store i32 %807, ptr %808, align 8, !dbg !24365, !alias.scope !24253, !noalias !24366
  %_1481.i = load i64, ptr %_53, align 8, !dbg !24367, !alias.scope !24253, !noalias !24366, !noundef !10
  %809 = tail call i64 @llvm.uadd.sat.i64(i64 %_1481.i, i64 1), !dbg !24368
  store i64 %809, ptr %_53, align 8, !dbg !24371, !alias.scope !24253, !noalias !24366
  br label %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit60.i, !dbg !24372

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i: ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit.i, %bb7.i4321
  %ok.sroa.0.0.lcssa.i76.i = phi i32 [ %807, %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit.i ], [ 0, %bb7.i4321 ]
  br label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.i, !dbg !24361

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i
  %ok.sroa.0.012.i42.i = phi i32 [ %_0.i8.i49.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.i ], [ -1, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i ]
  %iter.sroa.0.011.i43.i = phi ptr [ %_42.i45.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.i ], [ %right_io.0, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i ]
  %iter.sroa.5.010.i44.i = phi i32 [ %_43.i46.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.i ], [ %right_io.1, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i ]
  %_42.i45.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.011.i43.i, i32 4, !dbg !24377
  %_43.i46.i = add nsw i32 %iter.sroa.5.010.i44.i, -1, !dbg !24380
  %_0.i.i47.i = load float, ptr %iter.sroa.0.011.i43.i, align 4, !dbg !24381, !alias.scope !24383, !noalias !24310, !noundef !10
  %810 = tail call noundef float @llvm.fabs.f32(float %_0.i.i47.i), !dbg !24388
  %_3.i.i48.i = fcmp olt float %810, 0x46293E5940000000, !dbg !24390
  %_0.i8.i49.i = select i1 %_3.i.i48.i, i32 %ok.sroa.0.012.i42.i, i32 0, !dbg !24392
  %_37.not.i50.i = icmp eq i32 %_43.i46.i, 0, !dbg !24361
  br i1 %_37.not.i50.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit53.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.i, !dbg !24361

_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit53.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.i
  %811 = and i32 %_0.i8.i49.i, 1065353216, !dbg !24394
  %812 = icmp ne i32 %811, 1065353216, !dbg !24396
  %813 = zext i1 %812 to i32, !dbg !24396
  %814 = or i32 %ok.sroa.0.0.lcssa.i76.i, %813, !dbg !24365
  %815 = getelementptr inbounds nuw i8, ptr %self, i32 120, !dbg !24365
  store i32 %814, ptr %815, align 8, !dbg !24365, !alias.scope !24253, !noalias !24366
  %_14.i4313 = load i64, ptr %_53, align 8, !dbg !24367, !alias.scope !24253, !noalias !24366, !noundef !10
  %816 = tail call i64 @llvm.uadd.sat.i64(i64 %_14.i4313, i64 1), !dbg !24368
  store i64 %816, ptr %_53, align 8, !dbg !24371, !alias.scope !24253, !noalias !24366
  br i1 %_22.not.i1463.i, label %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit60.i, label %bb12.i.preheader.i, !dbg !24397

bb12.i.preheader.i:                               ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit53.i
  %.idx.i.i = shl nuw nsw i32 %left_io.1, 2, !dbg !24401
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %left_io.0, i8 0, i32 %.idx.i.i, i1 false), !dbg !24405, !alias.scope !24406, !noalias !24282
  br label %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit60.i, !dbg !24372

_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit60.i: ; preds = %bb12.i.preheader.i, %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit53.i, %bb12.i.preheader.thread.i
  %left.1.sink.i = phi i32 [ %left_io.1, %bb12.i.preheader.thread.i ], [ %right_io.1, %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit53.i ], [ %right_io.1, %bb12.i.preheader.i ]
  %left.0.sink.i = phi ptr [ %left_io.0, %bb12.i.preheader.thread.i ], [ %right_io.0, %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit53.i ], [ %right_io.0, %bb12.i.preheader.i ]
  %.idx.i85.i = shl nuw nsw i32 %left.1.sink.i, 2, !dbg !24409
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %left.0.sink.i, i8 0, i32 %.idx.i85.i, i1 false), !dbg !24415, !alias.scope !24416, !noalias !24417
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 4 dereferenceable(100) %_33, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(12) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_99.0, i32 noundef %_99.1, i32 noundef %795) #32, !dbg !24418, !noalias !24421
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 4 dereferenceable(100) %_34, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(12) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_100.0, i32 noundef %_100.1, i32 noundef %795) #32, !dbg !24424, !noalias !24421
  store i32 0, ptr %_35, align 4, !dbg !24425, !noalias !24421
  %817 = getelementptr inbounds nuw i8, ptr %self, i32 108, !dbg !24425
  store i32 0, ptr %817, align 4, !dbg !24425, !noalias !24421
  br label %_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockfNCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit, !dbg !24426

_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockfNCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit: ; preds = %bb6.i.preheader.i, %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockfECsjLJhryqjeDL_17true_peak_limiter.exit.i, %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit60.i
  call void @llvm.lifetime.end.p0(ptr nonnull %shape), !dbg !24427
  br label %bb42, !dbg !24188

bb54:                                             ; preds = %bb34
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef %frames, i32 noundef %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_8b6b4fff29b57e2077f91d7df0eea7e7) #33, !dbg !24428
  unreachable, !dbg !24428

bb1.i4322:                                        ; preds = %bb34, %bb12.i4336
  %io.sroa.5.0.i4323 = phi i32 [ %len.i.i.i4329, %bb12.i4336 ], [ %frames, %bb34 ]
  %io.sroa.0.0.i4324 = phi ptr [ %data.i.i.i4328, %bb12.i4336 ], [ %right_io.0, %bb34 ]
  %818 = icmp eq i32 %io.sroa.5.0.i4323, 0, !dbg !24429
  br i1 %818, label %bb40, label %bb13.preheader.i4325, !dbg !24429

bb13.preheader.i4325:                             ; preds = %bb1.i4322
  %spec.store.select.i4326 = tail call i32 @llvm.umin.i32(i32 %io.sroa.5.0.i4323, i32 32), !dbg !24432
  %data.i.i.idx.i4327 = shl nuw nsw i32 %spec.store.select.i4326, 2, !dbg !24435
  %data.i.i.i4328 = getelementptr inbounds nuw i8, ptr %io.sroa.0.0.i4324, i32 %data.i.i.idx.i4327, !dbg !24435
  br label %bb13.i4330, !dbg !24440

bb13.i4330:                                       ; preds = %bb13.i4330, %bb13.preheader.i4325
  %iter.sroa.0.08.i4331 = phi ptr [ %_35.i4333, %bb13.i4330 ], [ %io.sroa.0.0.i4324, %bb13.preheader.i4325 ]
  %bits.sroa.0.07.i4332 = phi i32 [ %819, %bb13.i4330 ], [ 0, %bb13.preheader.i4325 ]
  %_35.i4333 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i4331, i32 4, !dbg !24442
  %_95.i4334 = load i32, ptr %iter.sroa.0.08.i4331, align 4, !dbg !24444, !alias.scope !24445, !noundef !10
  %819 = or i32 %_95.i4334, %bits.sroa.0.07.i4332, !dbg !24448
  %_29.i4335 = icmp eq ptr %_35.i4333, %data.i.i.i4328, !dbg !24449
  br i1 %_29.i4335, label %bb12.i4336, label %bb13.i4330, !dbg !24440

bb12.i4336:                                       ; preds = %bb13.i4330
  %len.i.i.i4329 = sub nuw nsw i32 %io.sroa.5.0.i4323, %spec.store.select.i4326, !dbg !24451
  %820 = icmp eq i32 %819, 0, !dbg !24452
  br i1 %820, label %bb1.i4322, label %bb40, !dbg !24452

bb42:                                             ; preds = %_RNvMs1_CsjLJhryqjeDL_17true_peak_limiterNtB5_7Cursors7advance.exit, %_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockfNCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit
  ret void, !dbg !24188
}
