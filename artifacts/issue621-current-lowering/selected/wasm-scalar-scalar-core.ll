define internal fastcc void @_RNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB5_11LimiterCorefE13process_blockB5_(ptr noalias noundef nonnull align 8 dereferenceable(544) %self, ptr noalias noundef nonnull align 4 captures(address) %left_io.0, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef nonnull align 4 captures(address) %right_io.0, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef %frames) unnamed_addr #0 !dbg !3566 {
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
  %0 = getelementptr inbounds nuw i8, ptr %self, i32 537, !dbg !3568
  %1 = load i8, ptr %0, align 1, !dbg !3568, !range !3570, !noundef !10
  %2 = getelementptr inbounds nuw i8, ptr %self, i32 80, !dbg !3571
  %3 = load i8, ptr %2, align 8, !dbg !3571, !range !3570, !noundef !10
  %_7 = icmp eq i8 %1, %3, !dbg !3568
  %4 = getelementptr inbounds nuw i8, ptr %self, i32 388
  %_95.0 = load ptr, ptr %4, align 4, !dbg !3572
  %5 = getelementptr inbounds nuw i8, ptr %self, i32 392
  %_95.1 = load i32, ptr %5, align 4, !dbg !3572
  br i1 %_7, label %bb1, label %bb20.thread, !dbg !3568

bb1:                                              ; preds = %start
  %_8.i3844 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_95.0, i32 %_95.1, !dbg !3573
  br label %bb1.i.i3845, !dbg !3578

bb1.i.i3845:                                      ; preds = %bb11.i.i, %bb1
  %_221.i.i = phi ptr [ %_22.i.i3846, %bb11.i.i ], [ %_95.0, %bb1 ]
  %_12.i.i = icmp eq ptr %_221.i.i, %_8.i3844, !dbg !3580
  br i1 %_12.i.i, label %bb3, label %bb11.i.i, !dbg !3583

bb11.i.i:                                         ; preds = %bb1.i.i3845
  %_22.i.i3846 = getelementptr inbounds nuw i8, ptr %_221.i.i, i32 16, !dbg !3584
  %6 = getelementptr inbounds nuw i8, ptr %_221.i.i, i32 12, !dbg !3586
  %_3.i.i.i = load i32, ptr %6, align 4, !dbg !3586, !alias.scope !3588, !noalias !3593, !noundef !10
  %7 = icmp eq i32 %_3.i.i.i, 0, !dbg !3586
  %_51.i.i.i = load i32, ptr %_221.i.i, align 4, !dbg !3586, !alias.scope !3588, !noalias !3593
  %8 = getelementptr inbounds nuw i8, ptr %_221.i.i, i32 4, !dbg !3586
  %_72.i.i.i = load i32, ptr %8, align 4, !dbg !3586, !alias.scope !3588, !noalias !3593
  %9 = icmp eq i32 %_51.i.i.i, %_72.i.i.i, !dbg !3586
  %_0.sroa.0.0.off0.i.i.i = select i1 %7, i1 %9, i1 false, !dbg !3586
  br i1 %_0.sroa.0.0.off0.i.i.i, label %bb1.i.i3845, label %bb20.thread, !dbg !3596

bb3:                                              ; preds = %bb1.i.i3845
  %10 = getelementptr inbounds nuw i8, ptr %self, i32 396, !dbg !3597
  %_96.0 = load ptr, ptr %10, align 4, !dbg !3597, !nonnull !10, !noundef !10
  %11 = getelementptr inbounds nuw i8, ptr %self, i32 400, !dbg !3597
  %_96.1 = load i32, ptr %11, align 4, !dbg !3597, !noundef !10
  %_8.i3847 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_96.0, i32 %_96.1, !dbg !3598
  br label %bb1.i.i3848, !dbg !3603

bb1.i.i3848:                                      ; preds = %bb11.i.i3851, %bb3
  %_221.i.i3849 = phi ptr [ %_22.i.i3852, %bb11.i.i3851 ], [ %_96.0, %bb3 ]
  %_12.i.i3850 = icmp eq ptr %_221.i.i3849, %_8.i3847, !dbg !3605
  br i1 %_12.i.i3850, label %bb5, label %bb11.i.i3851, !dbg !3608

bb11.i.i3851:                                     ; preds = %bb1.i.i3848
  %_22.i.i3852 = getelementptr inbounds nuw i8, ptr %_221.i.i3849, i32 16, !dbg !3609
  %12 = getelementptr inbounds nuw i8, ptr %_221.i.i3849, i32 12, !dbg !3611
  %_3.i.i.i3853 = load i32, ptr %12, align 4, !dbg !3611, !alias.scope !3613, !noalias !3618, !noundef !10
  %13 = icmp eq i32 %_3.i.i.i3853, 0, !dbg !3611
  %_51.i.i.i3854 = load i32, ptr %_221.i.i3849, align 4, !dbg !3611, !alias.scope !3613, !noalias !3618
  %14 = getelementptr inbounds nuw i8, ptr %_221.i.i3849, i32 4, !dbg !3611
  %_72.i.i.i3855 = load i32, ptr %14, align 4, !dbg !3611, !alias.scope !3613, !noalias !3618
  %15 = icmp eq i32 %_51.i.i.i3854, %_72.i.i.i3855, !dbg !3611
  %_0.sroa.0.0.off0.i.i.i3856 = select i1 %13, i1 %15, i1 false, !dbg !3611
  br i1 %_0.sroa.0.0.off0.i.i.i3856, label %bb1.i.i3848, label %bb20.thread, !dbg !3621

bb5:                                              ; preds = %bb1.i.i3848
  %16 = getelementptr inbounds nuw i8, ptr %self, i32 488, !dbg !3622
  %_97.0 = load ptr, ptr %16, align 8, !dbg !3622, !nonnull !10, !noundef !10
  %17 = getelementptr inbounds nuw i8, ptr %self, i32 492, !dbg !3622
  %_97.1 = load i32, ptr %17, align 4, !dbg !3622, !noundef !10
  %_8.i3858 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_97.0, i32 %_97.1, !dbg !3623
  br label %bb1.i.i3859, !dbg !3628

bb1.i.i3859:                                      ; preds = %bb11.i.i3862, %bb5
  %_221.i.i3860 = phi ptr [ %_22.i.i3863, %bb11.i.i3862 ], [ %_97.0, %bb5 ]
  %_12.i.i3861 = icmp eq ptr %_221.i.i3860, %_8.i3858, !dbg !3630
  br i1 %_12.i.i3861, label %bb7, label %bb11.i.i3862, !dbg !3633

bb11.i.i3862:                                     ; preds = %bb1.i.i3859
  %_22.i.i3863 = getelementptr inbounds nuw i8, ptr %_221.i.i3860, i32 16, !dbg !3634
  %18 = getelementptr inbounds nuw i8, ptr %_221.i.i3860, i32 12, !dbg !3636
  %_3.i.i.i3864 = load i32, ptr %18, align 4, !dbg !3636, !alias.scope !3638, !noalias !3643, !noundef !10
  %19 = icmp eq i32 %_3.i.i.i3864, 0, !dbg !3636
  %_51.i.i.i3865 = load i32, ptr %_221.i.i3860, align 4, !dbg !3636, !alias.scope !3638, !noalias !3643
  %20 = getelementptr inbounds nuw i8, ptr %_221.i.i3860, i32 4, !dbg !3636
  %_72.i.i.i3866 = load i32, ptr %20, align 4, !dbg !3636, !alias.scope !3638, !noalias !3643
  %21 = icmp eq i32 %_51.i.i.i3865, %_72.i.i.i3866, !dbg !3636
  %_0.sroa.0.0.off0.i.i.i3867 = select i1 %19, i1 %21, i1 false, !dbg !3636
  br i1 %_0.sroa.0.0.off0.i.i.i3867, label %bb1.i.i3859, label %bb20.thread, !dbg !3646

bb7:                                              ; preds = %bb1.i.i3859
  %22 = getelementptr inbounds nuw i8, ptr %self, i32 496, !dbg !3647
  %_98.0 = load ptr, ptr %22, align 8, !dbg !3647, !nonnull !10, !noundef !10
  %23 = getelementptr inbounds nuw i8, ptr %self, i32 500, !dbg !3647
  %_98.1 = load i32, ptr %23, align 4, !dbg !3647, !noundef !10
  %_8.i3869 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_98.0, i32 %_98.1, !dbg !3648
  br label %bb1.i.i3870, !dbg !3653

bb1.i.i3870:                                      ; preds = %bb11.i.i3873, %bb7
  %_221.i.i3871 = phi ptr [ %_22.i.i3874, %bb11.i.i3873 ], [ %_98.0, %bb7 ]
  %_12.i.i3872 = icmp eq ptr %_221.i.i3871, %_8.i3869, !dbg !3655
  br i1 %_12.i.i3872, label %bb9, label %bb11.i.i3873, !dbg !3658

bb11.i.i3873:                                     ; preds = %bb1.i.i3870
  %_22.i.i3874 = getelementptr inbounds nuw i8, ptr %_221.i.i3871, i32 16, !dbg !3659
  %24 = getelementptr inbounds nuw i8, ptr %_221.i.i3871, i32 12, !dbg !3661
  %_3.i.i.i3875 = load i32, ptr %24, align 4, !dbg !3661, !alias.scope !3663, !noalias !3668, !noundef !10
  %25 = icmp eq i32 %_3.i.i.i3875, 0, !dbg !3661
  %_51.i.i.i3876 = load i32, ptr %_221.i.i3871, align 4, !dbg !3661, !alias.scope !3663, !noalias !3668
  %26 = getelementptr inbounds nuw i8, ptr %_221.i.i3871, i32 4, !dbg !3661
  %_72.i.i.i3877 = load i32, ptr %26, align 4, !dbg !3661, !alias.scope !3663, !noalias !3668
  %27 = icmp eq i32 %_51.i.i.i3876, %_72.i.i.i3877, !dbg !3661
  %_0.sroa.0.0.off0.i.i.i3878 = select i1 %25, i1 %27, i1 false, !dbg !3661
  br i1 %_0.sroa.0.0.off0.i.i.i3878, label %bb1.i.i3870, label %bb20.thread, !dbg !3671

bb9:                                              ; preds = %bb1.i.i3870
  %_65.not = icmp ugt i32 %frames, %left_io.1
  br i1 %_65.not, label %bb45, label %bb1.i3880, !dbg !3672, !prof !3544

bb45:                                             ; preds = %bb9
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef %frames, i32 noundef %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ea4a5c70c3b6737d27e6f8140269468e) #28, !dbg !3681
  unreachable, !dbg !3681

bb1.i3880:                                        ; preds = %bb9, %bb12.i3885
  %io.sroa.5.0.i = phi i32 [ %len.i.i.i, %bb12.i3885 ], [ %frames, %bb9 ]
  %io.sroa.0.0.i = phi ptr [ %data.i.i.i, %bb12.i3885 ], [ %left_io.0, %bb9 ]
  %28 = icmp eq i32 %io.sroa.5.0.i, 0, !dbg !3682
  br i1 %28, label %bb11, label %bb13.preheader.i, !dbg !3682

bb13.preheader.i:                                 ; preds = %bb1.i3880
  %spec.store.select.i3881 = tail call i32 @llvm.umin.i32(i32 %io.sroa.5.0.i, i32 32), !dbg !3685
  %data.i.i.idx.i = shl nuw nsw i32 %spec.store.select.i3881, 2, !dbg !3688
  %data.i.i.i = getelementptr inbounds nuw i8, ptr %io.sroa.0.0.i, i32 %data.i.i.idx.i, !dbg !3688
  br label %bb13.i3882, !dbg !3693

bb13.i3882:                                       ; preds = %bb13.i3882, %bb13.preheader.i
  %iter.sroa.0.08.i = phi ptr [ %_35.i3883, %bb13.i3882 ], [ %io.sroa.0.0.i, %bb13.preheader.i ]
  %bits.sroa.0.07.i = phi i32 [ %29, %bb13.i3882 ], [ 0, %bb13.preheader.i ]
  %_35.i3883 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i, i32 4, !dbg !3695
  %_95.i3884 = load i32, ptr %iter.sroa.0.08.i, align 4, !dbg !3697, !alias.scope !3698, !noundef !10
  %29 = or i32 %_95.i3884, %bits.sroa.0.07.i, !dbg !3701
  %_29.i = icmp eq ptr %_35.i3883, %data.i.i.i, !dbg !3702
  br i1 %_29.i, label %bb12.i3885, label %bb13.i3882, !dbg !3693

bb12.i3885:                                       ; preds = %bb13.i3882
  %len.i.i.i = sub nuw nsw i32 %io.sroa.5.0.i, %spec.store.select.i3881, !dbg !3704
  %30 = icmp eq i32 %29, 0, !dbg !3705
  br i1 %30, label %bb1.i3880, label %bb20.thread, !dbg !3705

bb11:                                             ; preds = %bb1.i3880
  %_73.not = icmp ugt i32 %frames, %right_io.1, !dbg !3706
  br i1 %_73.not, label %bb48, label %bb1.i3887, !dbg !3706, !prof !755

bb20.thread:                                      ; preds = %bb11.i.i, %bb11.i.i3851, %bb11.i.i3862, %bb11.i.i3873, %bb12.i3885, %start
  %31 = getelementptr inbounds nuw i8, ptr %self, i32 536
  br label %bb26, !dbg !3712

bb20:                                             ; preds = %bb1.i3887
  %32 = getelementptr inbounds nuw i8, ptr %self, i32 536
  %33 = load i8, ptr %32, align 8, !range !3570
  %_22 = trunc nuw i8 %33 to i1
  br i1 %_22, label %bb22, label %bb26, !dbg !3712

bb48:                                             ; preds = %bb11
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef %frames, i32 noundef %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_786d3728118d574b1dded47beaf5441a) #28, !dbg !3714
  unreachable, !dbg !3714

bb1.i3887:                                        ; preds = %bb11, %bb12.i3901
  %io.sroa.5.0.i3888 = phi i32 [ %len.i.i.i3894, %bb12.i3901 ], [ %frames, %bb11 ]
  %io.sroa.0.0.i3889 = phi ptr [ %data.i.i.i3893, %bb12.i3901 ], [ %right_io.0, %bb11 ]
  %34 = icmp eq i32 %io.sroa.5.0.i3888, 0, !dbg !3715
  br i1 %34, label %bb20, label %bb13.preheader.i3890, !dbg !3715

bb13.preheader.i3890:                             ; preds = %bb1.i3887
  %spec.store.select.i3891 = tail call i32 @llvm.umin.i32(i32 %io.sroa.5.0.i3888, i32 32), !dbg !3718
  %data.i.i.idx.i3892 = shl nuw nsw i32 %spec.store.select.i3891, 2, !dbg !3721
  %data.i.i.i3893 = getelementptr inbounds nuw i8, ptr %io.sroa.0.0.i3889, i32 %data.i.i.idx.i3892, !dbg !3721
  br label %bb13.i3895, !dbg !3726

bb13.i3895:                                       ; preds = %bb13.i3895, %bb13.preheader.i3890
  %iter.sroa.0.08.i3896 = phi ptr [ %_35.i3898, %bb13.i3895 ], [ %io.sroa.0.0.i3889, %bb13.preheader.i3890 ]
  %bits.sroa.0.07.i3897 = phi i32 [ %35, %bb13.i3895 ], [ 0, %bb13.preheader.i3890 ]
  %_35.i3898 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i3896, i32 4, !dbg !3728
  %_95.i3899 = load i32, ptr %iter.sroa.0.08.i3896, align 4, !dbg !3730, !alias.scope !3731, !noundef !10
  %35 = or i32 %_95.i3899, %bits.sroa.0.07.i3897, !dbg !3734
  %_29.i3900 = icmp eq ptr %_35.i3898, %data.i.i.i3893, !dbg !3735
  br i1 %_29.i3900, label %bb12.i3901, label %bb13.i3895, !dbg !3726

bb12.i3901:                                       ; preds = %bb13.i3895
  %len.i.i.i3894 = sub nuw nsw i32 %io.sroa.5.0.i3888, %spec.store.select.i3891, !dbg !3737
  %36 = icmp eq i32 %35, 0, !dbg !3738
  br i1 %36, label %bb1.i3887, label %bb20.thread4437, !dbg !3738

bb20.thread4437:                                  ; preds = %bb12.i3901
  %37 = getelementptr inbounds nuw i8, ptr %self, i32 536
  br label %bb26, !dbg !3712

bb26:                                             ; preds = %bb20.thread4437, %bb20.thread, %bb20
  %38 = phi ptr [ %31, %bb20.thread ], [ %32, %bb20 ], [ %37, %bb20.thread4437 ]
  %quiet.sroa.0.0.off04436 = phi i1 [ false, %bb20.thread ], [ true, %bb20 ], [ false, %bb20.thread4437 ]
  %_31 = getelementptr inbounds nuw i8, ptr %self, i32 128, !dbg !3739
  %_32 = getelementptr inbounds nuw i8, ptr %self, i32 524, !dbg !3740
  %_33 = getelementptr inbounds nuw i8, ptr %self, i32 324, !dbg !3741
  %_34 = getelementptr inbounds nuw i8, ptr %self, i32 424, !dbg !3742
  %_35 = getelementptr inbounds nuw i8, ptr %self, i32 104, !dbg !3743
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3744), !dbg !3747
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3750), !dbg !3747
  %_8.i3904 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_95.0, i32 %_95.1, !dbg !3752
  br label %bb1.i.i3905, !dbg !3759

bb1.i.i3905:                                      ; preds = %bb11.i.i3908, %bb26
  %_221.i.i3906 = phi ptr [ %_22.i.i3909, %bb11.i.i3908 ], [ %_95.0, %bb26 ]
  %_12.i.i3907 = icmp eq ptr %_221.i.i3906, %_8.i3904, !dbg !3761
  br i1 %_12.i.i3907, label %bb2.i, label %bb11.i.i3908, !dbg !3764

bb11.i.i3908:                                     ; preds = %bb1.i.i3905
  %_22.i.i3909 = getelementptr inbounds nuw i8, ptr %_221.i.i3906, i32 16, !dbg !3765
  %39 = getelementptr inbounds nuw i8, ptr %_221.i.i3906, i32 12, !dbg !3767
  %_3.i.i.i3910 = load i32, ptr %39, align 4, !dbg !3767, !alias.scope !3769, !noalias !3774, !noundef !10
  %40 = icmp eq i32 %_3.i.i.i3910, 0, !dbg !3767
  %_51.i.i.i3911 = load i32, ptr %_221.i.i3906, align 4, !dbg !3767, !alias.scope !3769, !noalias !3774
  %41 = getelementptr inbounds nuw i8, ptr %_221.i.i3906, i32 4, !dbg !3767
  %_72.i.i.i3912 = load i32, ptr %41, align 4, !dbg !3767, !alias.scope !3769, !noalias !3774
  %42 = icmp eq i32 %_51.i.i.i3911, %_72.i.i.i3912, !dbg !3767
  %_0.sroa.0.0.off0.i.i.i3913 = select i1 %40, i1 %42, i1 false, !dbg !3767
  br i1 %_0.sroa.0.0.off0.i.i.i3913, label %bb1.i.i3905, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, !dbg !3783

bb2.i:                                            ; preds = %bb1.i.i3905
  %43 = getelementptr inbounds nuw i8, ptr %self, i32 396, !dbg !3784
  %_15.0.i = load ptr, ptr %43, align 4, !dbg !3784, !alias.scope !3744, !noalias !3785, !nonnull !10, !noundef !10
  %44 = getelementptr inbounds nuw i8, ptr %self, i32 400, !dbg !3784
  %_15.1.i = load i32, ptr %44, align 4, !dbg !3784, !alias.scope !3744, !noalias !3785, !noundef !10
  %_8.i3915 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_15.0.i, i32 %_15.1.i, !dbg !3786
  br label %bb1.i.i3916, !dbg !3791

bb1.i.i3916:                                      ; preds = %bb11.i.i3919, %bb2.i
  %_221.i.i3917 = phi ptr [ %_22.i.i3920, %bb11.i.i3919 ], [ %_15.0.i, %bb2.i ]
  %_12.i.i3918 = icmp eq ptr %_221.i.i3917, %_8.i3915, !dbg !3793
  br i1 %_12.i.i3918, label %bb4.i1454, label %bb11.i.i3919, !dbg !3796

bb11.i.i3919:                                     ; preds = %bb1.i.i3916
  %_22.i.i3920 = getelementptr inbounds nuw i8, ptr %_221.i.i3917, i32 16, !dbg !3797
  %45 = getelementptr inbounds nuw i8, ptr %_221.i.i3917, i32 12, !dbg !3799
  %_3.i.i.i3921 = load i32, ptr %45, align 4, !dbg !3799, !alias.scope !3801, !noalias !3806, !noundef !10
  %46 = icmp eq i32 %_3.i.i.i3921, 0, !dbg !3799
  %_51.i.i.i3922 = load i32, ptr %_221.i.i3917, align 4, !dbg !3799, !alias.scope !3801, !noalias !3806
  %47 = getelementptr inbounds nuw i8, ptr %_221.i.i3917, i32 4, !dbg !3799
  %_72.i.i.i3923 = load i32, ptr %47, align 4, !dbg !3799, !alias.scope !3801, !noalias !3806
  %48 = icmp eq i32 %_51.i.i.i3922, %_72.i.i.i3923, !dbg !3799
  %_0.sroa.0.0.off0.i.i.i3924 = select i1 %46, i1 %48, i1 false, !dbg !3799
  br i1 %_0.sroa.0.0.off0.i.i.i3924, label %bb1.i.i3916, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, !dbg !3809

bb4.i1454:                                        ; preds = %bb1.i.i3916
  %49 = getelementptr inbounds nuw i8, ptr %self, i32 488, !dbg !3810
  %_16.0.i = load ptr, ptr %49, align 4, !dbg !3810, !alias.scope !3750, !noalias !3811, !nonnull !10, !noundef !10
  %50 = getelementptr inbounds nuw i8, ptr %self, i32 492, !dbg !3810
  %_16.1.i = load i32, ptr %50, align 4, !dbg !3810, !alias.scope !3750, !noalias !3811, !noundef !10
  %_8.i3926 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_16.0.i, i32 %_16.1.i, !dbg !3812
  br label %bb1.i.i3927, !dbg !3817

bb1.i.i3927:                                      ; preds = %bb11.i.i3930, %bb4.i1454
  %_221.i.i3928 = phi ptr [ %_22.i.i3931, %bb11.i.i3930 ], [ %_16.0.i, %bb4.i1454 ]
  %_12.i.i3929 = icmp eq ptr %_221.i.i3928, %_8.i3926, !dbg !3819
  br i1 %_12.i.i3929, label %bb6.i1455, label %bb11.i.i3930, !dbg !3822

bb11.i.i3930:                                     ; preds = %bb1.i.i3927
  %_22.i.i3931 = getelementptr inbounds nuw i8, ptr %_221.i.i3928, i32 16, !dbg !3823
  %51 = getelementptr inbounds nuw i8, ptr %_221.i.i3928, i32 12, !dbg !3825
  %_3.i.i.i3932 = load i32, ptr %51, align 4, !dbg !3825, !alias.scope !3827, !noalias !3832, !noundef !10
  %52 = icmp eq i32 %_3.i.i.i3932, 0, !dbg !3825
  %_51.i.i.i3933 = load i32, ptr %_221.i.i3928, align 4, !dbg !3825, !alias.scope !3827, !noalias !3832
  %53 = getelementptr inbounds nuw i8, ptr %_221.i.i3928, i32 4, !dbg !3825
  %_72.i.i.i3934 = load i32, ptr %53, align 4, !dbg !3825, !alias.scope !3827, !noalias !3832
  %54 = icmp eq i32 %_51.i.i.i3933, %_72.i.i.i3934, !dbg !3825
  %_0.sroa.0.0.off0.i.i.i3935 = select i1 %52, i1 %54, i1 false, !dbg !3825
  br i1 %_0.sroa.0.0.off0.i.i.i3935, label %bb1.i.i3927, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, !dbg !3835

bb6.i1455:                                        ; preds = %bb1.i.i3927
  %55 = getelementptr inbounds nuw i8, ptr %self, i32 496, !dbg !3836
  %_17.0.i = load ptr, ptr %55, align 4, !dbg !3836, !alias.scope !3750, !noalias !3811, !nonnull !10, !noundef !10
  %56 = getelementptr inbounds nuw i8, ptr %self, i32 500, !dbg !3836
  %_17.1.i = load i32, ptr %56, align 4, !dbg !3836, !alias.scope !3750, !noalias !3811, !noundef !10
  %_8.i3937 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_17.0.i, i32 %_17.1.i, !dbg !3837
  br label %bb1.i.i3938, !dbg !3842

bb1.i.i3938:                                      ; preds = %bb11.i.i3941, %bb6.i1455
  %_221.i.i3939 = phi ptr [ %_22.i.i3942, %bb11.i.i3941 ], [ %_17.0.i, %bb6.i1455 ]
  %_12.i.i3940 = icmp eq ptr %_221.i.i3939, %_8.i3937, !dbg !3844
  br i1 %_12.i.i3940, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, label %bb11.i.i3941, !dbg !3847

bb11.i.i3941:                                     ; preds = %bb1.i.i3938
  %_22.i.i3942 = getelementptr inbounds nuw i8, ptr %_221.i.i3939, i32 16, !dbg !3848
  %57 = getelementptr inbounds nuw i8, ptr %_221.i.i3939, i32 12, !dbg !3850
  %_3.i.i.i3943 = load i32, ptr %57, align 4, !dbg !3850, !alias.scope !3852, !noalias !3857, !noundef !10
  %58 = icmp eq i32 %_3.i.i.i3943, 0, !dbg !3850
  %_51.i.i.i3944 = load i32, ptr %_221.i.i3939, align 4, !dbg !3850, !alias.scope !3852, !noalias !3857
  %59 = getelementptr inbounds nuw i8, ptr %_221.i.i3939, i32 4, !dbg !3850
  %_72.i.i.i3945 = load i32, ptr %59, align 4, !dbg !3850, !alias.scope !3852, !noalias !3857
  %60 = icmp eq i32 %_51.i.i.i3944, %_72.i.i.i3945, !dbg !3850
  %_0.sroa.0.0.off0.i.i.i3946 = select i1 %58, i1 %60, i1 false, !dbg !3850
  br i1 %_0.sroa.0.0.off0.i.i.i3946, label %bb1.i.i3938, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, !dbg !3860

_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit: ; preds = %bb11.i.i3908, %bb11.i.i3919, %bb11.i.i3930, %bb11.i.i3941, %bb1.i.i3938
  %_0.sroa.0.0.off0.i = phi i1 [ false, %bb11.i.i3930 ], [ false, %bb11.i.i3919 ], [ false, %bb11.i.i3941 ], [ true, %bb1.i.i3938 ], [ false, %bb11.i.i3908 ]
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3861), !dbg !3864
  %61 = getelementptr inbounds nuw i8, ptr %self, i32 412, !dbg !3866
  %_31.0.i = load ptr, ptr %61, align 4, !dbg !3866, !alias.scope !3861, !noalias !3868, !nonnull !10, !noundef !10
  %62 = getelementptr inbounds nuw i8, ptr %self, i32 416, !dbg !3866
  %_31.1.i = load i32, ptr %62, align 4, !dbg !3866, !alias.scope !3861, !noalias !3868, !noundef !10
  %_17.idx.i = mul nuw nsw i32 %_31.1.i, 12, !dbg !3869
  %_17.i = getelementptr inbounds nuw i8, ptr %_31.0.i, i32 %_17.idx.i, !dbg !3869
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3873), !dbg !3876, !noalias !3868
  %_5.not.i.i.i = icmp eq i32 %_31.1.i, 0
  %63 = getelementptr inbounds nuw i8, ptr %_31.0.i, i32 4
  %64 = getelementptr inbounds nuw i8, ptr %_31.0.i, i32 8
  br i1 %_5.not.i.i.i, label %bb2.i3956, label %bb1.i.i3948

bb1.i.i3948:                                      ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i
  %_224.i.i = phi ptr [ %_22.i.i3951, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i ], [ %_31.0.i, %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit ]
  %_12.i.i3949 = icmp eq ptr %_224.i.i, %_17.i, !dbg !3877
  br i1 %_12.i.i3949, label %bb2.i3956, label %bb11.i.i3950, !dbg !3881

bb11.i.i3950:                                     ; preds = %bb1.i.i3948
  %_22.i.i3951 = getelementptr inbounds nuw i8, ptr %_224.i.i, i32 12, !dbg !3882
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3884), !dbg !3887, !noalias !3868
  %_9.i.i.i = load i32, ptr %_224.i.i, align 4, !dbg !3888, !alias.scope !3884, !noalias !3891, !noundef !10
  %_10.i.i.i = load i32, ptr %_31.0.i, align 4, !dbg !3888, !alias.scope !3873, !noalias !3893, !noundef !10
  %_8.i.i.i = icmp eq i32 %_9.i.i.i, %_10.i.i.i, !dbg !3888
  br i1 %_8.i.i.i, label %bb2.i.i.i, label %bb10.i, !dbg !3888

bb2.i.i.i:                                        ; preds = %bb11.i.i3950
  %65 = getelementptr inbounds nuw i8, ptr %_224.i.i, i32 4, !dbg !3888
  %_12.i.i.i = load i32, ptr %65, align 4, !dbg !3888, !alias.scope !3884, !noalias !3891, !noundef !10
  %_13.i.i.i = load i32, ptr %63, align 4, !dbg !3888, !alias.scope !3873, !noalias !3893, !noundef !10
  %_11.i.i.i = icmp eq i32 %_12.i.i.i, %_13.i.i.i, !dbg !3888
  br i1 %_11.i.i.i, label %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, label %bb10.i, !dbg !3888

_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i: ; preds = %bb2.i.i.i
  %66 = getelementptr inbounds nuw i8, ptr %_224.i.i, i32 8, !dbg !3888
  %_14.i.i.i3954 = load i32, ptr %66, align 4, !dbg !3888, !alias.scope !3884, !noalias !3891, !noundef !10
  %_15.i.i.i3955 = load i32, ptr %64, align 4, !dbg !3888, !alias.scope !3873, !noalias !3893, !noundef !10
  %67 = icmp eq i32 %_14.i.i.i3954, %_15.i.i.i3955, !dbg !3888
  br i1 %67, label %bb1.i.i3948, label %bb10.i, !dbg !3887

bb2.i3956:                                        ; preds = %bb1.i.i3948, %_RNvCsjLJhryqjeDL_17true_peak_limiter15dual_stationary.exit
  %68 = getelementptr inbounds nuw i8, ptr %self, i32 380, !dbg !3894
  %_32.0.i = load ptr, ptr %68, align 4, !dbg !3894, !alias.scope !3861, !noalias !3868, !nonnull !10, !noundef !10
  %69 = getelementptr inbounds nuw i8, ptr %self, i32 384, !dbg !3894
  %_32.1.i = load i32, ptr %69, align 4, !dbg !3894, !alias.scope !3861, !noalias !3868, !noundef !10
  %_26.idx.i = shl nuw nsw i32 %_32.1.i, 2, !dbg !3895
  %_26.i = getelementptr inbounds nuw i8, ptr %_32.0.i, i32 %_26.idx.i, !dbg !3895
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3899), !dbg !3902, !noalias !3868
  %_6.not.i.i.i = icmp eq i32 %_32.1.i, 0
  br i1 %_6.not.i.i.i, label %bb3.i, label %bb1.i3.i

bb1.i3.i:                                         ; preds = %bb2.i3956, %bb11.i5.i
  %_223.i.i = phi ptr [ %_22.i6.i, %bb11.i5.i ], [ %_32.0.i, %bb2.i3956 ]
  %_12.i4.i = icmp eq ptr %_223.i.i, %_26.i, !dbg !3903
  br i1 %_12.i4.i, label %bb3.i, label %bb11.i5.i, !dbg !3907

bb11.i5.i:                                        ; preds = %bb1.i3.i
  %_22.i6.i = getelementptr inbounds nuw i8, ptr %_223.i.i, i32 4, !dbg !3908
  %ptr.val.i.i = load i32, ptr %_223.i.i, align 4, !dbg !3910, !noalias !3911
  %_4.i.i.i3957 = load i32, ptr %_32.0.i, align 4, !dbg !3913, !alias.scope !3899, !noalias !3915, !noundef !10
  %_0.i.i.i = icmp eq i32 %ptr.val.i.i, %_4.i.i.i3957, !dbg !3916
  br i1 %_0.i.i.i, label %bb1.i3.i, label %bb10.i, !dbg !3910

bb3.i:                                            ; preds = %bb1.i3.i, %bb2.i3956
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3917), !dbg !3920
  %70 = getelementptr inbounds nuw i8, ptr %self, i32 512, !dbg !3921
  %_31.0.i3958 = load ptr, ptr %70, align 4, !dbg !3921, !alias.scope !3917, !noalias !3868, !nonnull !10, !noundef !10
  %71 = getelementptr inbounds nuw i8, ptr %self, i32 516, !dbg !3921
  %_31.1.i3959 = load i32, ptr %71, align 4, !dbg !3921, !alias.scope !3917, !noalias !3868, !noundef !10
  %_17.idx.i3960 = mul nuw nsw i32 %_31.1.i3959, 12, !dbg !3923
  %_17.i3961 = getelementptr inbounds nuw i8, ptr %_31.0.i3958, i32 %_17.idx.i3960, !dbg !3923
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3927), !dbg !3930, !noalias !3868
  %_5.not.i.i.i3962 = icmp eq i32 %_31.1.i3959, 0
  %72 = getelementptr inbounds nuw i8, ptr %_31.0.i3958, i32 4
  %73 = getelementptr inbounds nuw i8, ptr %_31.0.i3958, i32 8
  br i1 %_5.not.i.i.i3962, label %bb2.i3980, label %bb1.i.i3963

bb1.i.i3963:                                      ; preds = %bb3.i, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3977
  %_224.i.i3964 = phi ptr [ %_22.i.i3967, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3977 ], [ %_31.0.i3958, %bb3.i ]
  %_12.i.i3965 = icmp eq ptr %_224.i.i3964, %_17.i3961, !dbg !3931
  br i1 %_12.i.i3965, label %bb2.i3980, label %bb11.i.i3966, !dbg !3935

bb11.i.i3966:                                     ; preds = %bb1.i.i3963
  %_22.i.i3967 = getelementptr inbounds nuw i8, ptr %_224.i.i3964, i32 12, !dbg !3936
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3938), !dbg !3941, !noalias !3868
  %_9.i.i.i3968 = load i32, ptr %_224.i.i3964, align 4, !dbg !3942, !alias.scope !3938, !noalias !3945, !noundef !10
  %_10.i.i.i3969 = load i32, ptr %_31.0.i3958, align 4, !dbg !3942, !alias.scope !3927, !noalias !3947, !noundef !10
  %_8.i.i.i3970 = icmp eq i32 %_9.i.i.i3968, %_10.i.i.i3969, !dbg !3942
  br i1 %_8.i.i.i3970, label %bb2.i.i.i3973, label %bb10.i, !dbg !3942

bb2.i.i.i3973:                                    ; preds = %bb11.i.i3966
  %74 = getelementptr inbounds nuw i8, ptr %_224.i.i3964, i32 4, !dbg !3942
  %_12.i.i.i3974 = load i32, ptr %74, align 4, !dbg !3942, !alias.scope !3938, !noalias !3945, !noundef !10
  %_13.i.i.i3975 = load i32, ptr %72, align 4, !dbg !3942, !alias.scope !3927, !noalias !3947, !noundef !10
  %_11.i.i.i3976 = icmp eq i32 %_12.i.i.i3974, %_13.i.i.i3975, !dbg !3942
  br i1 %_11.i.i.i3976, label %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3977, label %bb10.i, !dbg !3942

_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3977: ; preds = %bb2.i.i.i3973
  %75 = getelementptr inbounds nuw i8, ptr %_224.i.i3964, i32 8, !dbg !3942
  %_14.i.i.i3978 = load i32, ptr %75, align 4, !dbg !3942, !alias.scope !3938, !noalias !3945, !noundef !10
  %_15.i.i.i3979 = load i32, ptr %73, align 4, !dbg !3942, !alias.scope !3927, !noalias !3947, !noundef !10
  %76 = icmp eq i32 %_14.i.i.i3978, %_15.i.i.i3979, !dbg !3942
  br i1 %76, label %bb1.i.i3963, label %bb10.i, !dbg !3941

bb2.i3980:                                        ; preds = %bb1.i.i3963, %bb3.i
  %77 = getelementptr inbounds nuw i8, ptr %self, i32 480, !dbg !3948
  %_32.0.i3981 = load ptr, ptr %77, align 4, !dbg !3948, !alias.scope !3917, !noalias !3868, !nonnull !10, !noundef !10
  %78 = getelementptr inbounds nuw i8, ptr %self, i32 484, !dbg !3948
  %_32.1.i3982 = load i32, ptr %78, align 4, !dbg !3948, !alias.scope !3917, !noalias !3868, !noundef !10
  %_26.idx.i3983 = shl nuw nsw i32 %_32.1.i3982, 2, !dbg !3949
  %_26.i3984 = getelementptr inbounds nuw i8, ptr %_32.0.i3981, i32 %_26.idx.i3983, !dbg !3949
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3953), !dbg !3956, !noalias !3868
  %_6.not.i.i.i3985 = icmp eq i32 %_32.1.i3982, 0
  br i1 %_6.not.i.i.i3985, label %bb5.i, label %bb1.i3.i3986

bb1.i3.i3986:                                     ; preds = %bb2.i3980, %bb11.i5.i3989
  %_223.i.i3987 = phi ptr [ %_22.i6.i3990, %bb11.i5.i3989 ], [ %_32.0.i3981, %bb2.i3980 ]
  %_12.i4.i3988 = icmp eq ptr %_223.i.i3987, %_26.i3984, !dbg !3957
  br i1 %_12.i4.i3988, label %bb5.i, label %bb11.i5.i3989, !dbg !3961

bb11.i5.i3989:                                    ; preds = %bb1.i3.i3986
  %_22.i6.i3990 = getelementptr inbounds nuw i8, ptr %_223.i.i3987, i32 4, !dbg !3962
  %ptr.val.i.i3991 = load i32, ptr %_223.i.i3987, align 4, !dbg !3964, !noalias !3965
  %_4.i.i.i3992 = load i32, ptr %_32.0.i3981, align 4, !dbg !3967, !alias.scope !3953, !noalias !3969, !noundef !10
  %_0.i.i.i3993 = icmp eq i32 %ptr.val.i.i3991, %_4.i.i.i3992, !dbg !3970
  br i1 %_0.i.i.i3993, label %bb1.i3.i3986, label %bb10.i, !dbg !3964

bb10.i:                                           ; preds = %bb11.i.i3950, %bb2.i.i.i, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, %bb11.i5.i, %bb11.i.i3966, %bb2.i.i.i3973, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3977, %bb11.i5.i3989
  %79 = getelementptr inbounds nuw i8, ptr %self, i32 320, !dbg !3971
  %80 = getelementptr inbounds nuw i8, ptr %self, i32 321, !dbg !3971
  %81 = getelementptr inbounds nuw i8, ptr %self, i32 108, !dbg !3971
  br i1 %_0.sroa.0.0.off0.i, label %bb11.i, label %bb12.i, !dbg !3972

bb5.i:                                            ; preds = %bb1.i3.i3986, %bb2.i3980
  br i1 %_0.sroa.0.0.off0.i, label %bb6.i, label %bb7.i, !dbg !3973

bb12.i:                                           ; preds = %bb10.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3974), !dbg !3977
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3978), !dbg !3977
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3980), !dbg !3977
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3982), !dbg !3977
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i708), !dbg !3984, !noalias !3988
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_left.i708, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #27, !dbg !3992, !noalias !3993
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_right.i707), !dbg !3994, !noalias !3988
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_right.i707, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #27, !dbg !3996, !noalias !3997
  %82 = load i8, ptr %79, align 4, !dbg !3998, !range !3570, !alias.scope !3974, !noalias !4002, !noundef !10
  %83 = load i8, ptr %80, align 1, !dbg !4003, !range !3570, !alias.scope !3974, !noalias !4002, !noundef !10
  %_35.i716 = load i32, ptr %_35, align 4, !dbg !4005, !alias.scope !3982, !noalias !4007, !noundef !10
  %_37.i717 = load i32, ptr %81, align 4, !dbg !4008, !alias.scope !3982, !noalias !4007, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i706), !dbg !4010, !noalias !3988
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i706, i8 0, i32 32, i1 false), !noalias !3988
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i705), !dbg !4012, !noalias !3988
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i705, i8 0, i32 1024, i1 false), !noalias !3988
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i704), !dbg !4014, !noalias !3988
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i704, i8 0, i32 1024, i1 false), !noalias !3988
  %_32.i712 = zext nneg i8 %82 to i32, !dbg !3998
  %.none.i713 = sub nsw i32 0, %_32.i712, !dbg !4016
  %_33.i714 = zext nneg i8 %83 to i32, !dbg !4003
  %all.sroa.0.0.i715 = sub nsw i32 0, %_33.i714, !dbg !4003
  %_111.not.i7307577 = icmp eq i32 %frames, 0, !dbg !4017
  br i1 %_111.not.i7307577, label %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit, label %bb37.i731.lr.ph, !dbg !4017

bb37.i731.lr.ph:                                  ; preds = %bb12.i
  %d9.i = lshr i32 %frames, 5, !dbg !4031
  %r2.i = and i32 %frames, 31, !dbg !4043
  %_19.not.i = icmp ne i32 %r2.i, 0, !dbg !4045
  %84 = zext i1 %_19.not.i to i32, !dbg !4045
  %yield_count.sroa.0.0.i = add nuw nsw i32 %d9.i, %84, !dbg !4045
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
  %iter.sroa.0.0.ptr.i55.i7866451.1 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 4
  %iter.sroa.0.0.ptr.i55.i7866451.2 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 8
  %iter.sroa.0.0.ptr.i55.i7866451.3 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 12
  %iter.sroa.0.0.ptr.i55.i7866451.4 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 16
  %iter.sroa.0.0.ptr.i55.i7866451.5 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 20
  %iter.sroa.0.0.ptr.i55.i7866451.6 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 24
  %iter.sroa.0.0.ptr.i55.i7866451.7 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 28
  %iter.sroa.0.0.ptr.i.i8796463.1 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 4
  %iter.sroa.0.0.ptr.i.i8796463.2 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 8
  %iter.sroa.0.0.ptr.i.i8796463.3 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 12
  %iter.sroa.0.0.ptr.i.i8796463.4 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 16
  %iter.sroa.0.0.ptr.i.i8796463.5 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 20
  %iter.sroa.0.0.ptr.i.i8796463.6 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 24
  %iter.sroa.0.0.ptr.i.i8796463.7 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 28
  br label %bb37.i731, !dbg !4017

bb16.i737.bb13.i725.loopexit_crit_edge:           ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077
  store float %_0.i.i3527, ptr %121, align 4, !dbg !4047, !alias.scope !4058, !noalias !4061
  store float %_0.i3299, ptr %_64.i744, align 4, !dbg !4064, !alias.scope !4058, !noalias !4061
  store float %_0.i3292, ptr %122, align 4, !dbg !4066, !alias.scope !4058, !noalias !4061
  store float %_0.i.i3534, ptr %124, align 4, !dbg !4067, !alias.scope !4069, !noalias !4072
  store float %_0.i3312, ptr %_65.i745, align 4, !dbg !4073, !alias.scope !4069, !noalias !4072
  store float %_0.i3305, ptr %125, align 4, !dbg !4074, !alias.scope !4069, !noalias !4072
  store float %_0.i.i3541, ptr %127, align 4, !dbg !4075, !alias.scope !4079, !noalias !4082
  store float %_0.i3325, ptr %_69.i746, align 4, !dbg !4085, !alias.scope !4079, !noalias !4082
  store float %_0.i3318, ptr %128, align 4, !dbg !4086, !alias.scope !4079, !noalias !4082
  store float %_0.i.i3548, ptr %130, align 4, !dbg !4087, !alias.scope !4089, !noalias !4072
  store float %_0.i3338, ptr %_70.i747, align 4, !dbg !4092, !alias.scope !4089, !noalias !4072
  store float %_0.i3331, ptr %131, align 4, !dbg !4093, !alias.scope !4089, !noalias !4072
  store float %_0.i2838, ptr %143, align 4, !dbg !4094
  store float %_0.i3212, ptr %145, align 4, !dbg !4109
  store float %_0.i2834, ptr %159, align 4, !dbg !4112
  store float %_0.i3208, ptr %161, align 4, !dbg !4114
  br label %bb13.i725.loopexit, !dbg !4115

bb13.i725.loopexit:                               ; preds = %bb16.i737.bb13.i725.loopexit_crit_edge, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i736
  %ring_cursor.sroa.0.1.i739.lcssa = phi i32 [ %spec.store.select12.i949, %bb16.i737.bb13.i725.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i7287580, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i736 ], !dbg !4121
  %main_cursor.sroa.0.1.i740.lcssa = phi i32 [ %spec.store.select11.i947, %bb16.i737.bb13.i725.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i7297581, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i736 ], !dbg !4122
  %_111.not.i730 = icmp eq i32 %167, 0, !dbg !4017
  %indvars.iv.next = add i32 %indvars.iv, -32, !dbg !4017
  br i1 %_111.not.i730, label %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit, label %bb37.i731, !dbg !4017

bb37.i731:                                        ; preds = %bb37.i731.lr.ph, %bb13.i725.loopexit
  %indvars.iv = phi i32 [ %frames, %bb37.i731.lr.ph ], [ %indvars.iv.next, %bb13.i725.loopexit ]
  %main_cursor.sroa.0.0.i7297581 = phi i32 [ %_35.i716, %bb37.i731.lr.ph ], [ %main_cursor.sroa.0.1.i740.lcssa, %bb13.i725.loopexit ]
  %ring_cursor.sroa.0.0.i7287580 = phi i32 [ %_37.i717, %bb37.i731.lr.ph ], [ %ring_cursor.sroa.0.1.i739.lcssa, %bb13.i725.loopexit ]
  %iter2.sroa.0.0.i7277579 = phi i32 [ %yield_count.sroa.0.0.i, %bb37.i731.lr.ph ], [ %167, %bb13.i725.loopexit ]
  %iter.sroa.0.0.i7267578 = phi i32 [ 0, %bb37.i731.lr.ph ], [ %166, %bb13.i725.loopexit ]
  %165 = call i32 @llvm.umax.i32(i32 %indvars.iv, i32 1), !dbg !4123
  %umax11909 = call i32 @llvm.umin.i32(i32 %165, i32 32), !dbg !4123
  %166 = add i32 %iter.sroa.0.0.i7267578, 32, !dbg !4123
  %167 = add nsw i32 %iter2.sroa.0.0.i7277579, -1, !dbg !4127
  %history.i136.i.sroa.0.0.copyload = load float, ptr %hot_left.i708, align 4, !dbg !4128, !noalias !4132
  %history.i136.i.sroa.7.0.copyload = load float, ptr %history.i136.i.sroa.7.0.hot_left.i708.sroa_idx, align 4, !dbg !4128, !noalias !4132
  %history.i136.i.sroa.10.0.copyload = load float, ptr %history.i136.i.sroa.10.0.hot_left.i708.sroa_idx, align 4, !dbg !4128, !noalias !4132
  %history.i136.i.sroa.13.0.copyload = load float, ptr %history.i136.i.sroa.13.0.hot_left.i708.sroa_idx, align 4, !dbg !4128, !noalias !4132
  %history.i136.i.sroa.16.0.copyload = load float, ptr %history.i136.i.sroa.16.0.hot_left.i708.sroa_idx, align 4, !dbg !4128, !noalias !4132
  %history.i136.i.sroa.19.0.copyload = load float, ptr %history.i136.i.sroa.19.0.hot_left.i708.sroa_idx, align 4, !dbg !4128, !noalias !4132
  %history.i136.i.sroa.22.0.copyload = load float, ptr %history.i136.i.sroa.22.0.hot_left.i708.sroa_idx, align 4, !dbg !4128, !noalias !4132
  %history.i136.i.sroa.26.0.copyload = load float, ptr %history.i136.i.sroa.26.0.hot_left.i708.sroa_idx, align 4, !dbg !4128, !noalias !4132
  %history.i136.i.sroa.29.0.copyload = load float, ptr %history.i136.i.sroa.29.0.hot_left.i708.sroa_idx, align 4, !dbg !4128, !noalias !4132
  %history.i136.i.sroa.32.0.copyload = load float, ptr %history.i136.i.sroa.32.0.hot_left.i708.sroa_idx, align 4, !dbg !4128, !noalias !4132
  %history.i136.i.sroa.35.0.copyload = load float, ptr %history.i136.i.sroa.35.0.hot_left.i708.sroa_idx, align 4, !dbg !4128, !noalias !4132
  %history.i136.i.sroa.38.0.copyload = load float, ptr %history.i136.i.sroa.38.0.hot_left.i708.sroa_idx, align 4, !dbg !4128, !noalias !4132
  %_20.i139.i6389.not = icmp eq i32 %frames, %iter.sroa.0.0.i7267578, !dbg !4137
  br i1 %_20.i139.i6389.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i, label %bb5.i140.i.lr.ph, !dbg !4147

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
  br label %bb5.i140.i, !dbg !4147

bb5.i140.i:                                       ; preds = %bb5.i140.i.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit
  %iter.sroa.0.0.i138.i6401 = phi i32 [ 0, %bb5.i140.i.lr.ph ], [ %168, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.35.06400 = phi float [ %history.i136.i.sroa.35.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.32.06399, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.32.06399 = phi float [ %history.i136.i.sroa.32.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.29.06398, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.29.06398 = phi float [ %history.i136.i.sroa.29.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.26.06397, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.26.06397 = phi float [ %history.i136.i.sroa.26.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.22.06396, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.22.06396 = phi float [ %history.i136.i.sroa.22.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.19.06395, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.19.06395 = phi float [ %history.i136.i.sroa.19.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.16.06394, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.16.06394 = phi float [ %history.i136.i.sroa.16.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.13.06393, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.13.06393 = phi float [ %history.i136.i.sroa.13.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.10.06392, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.10.06392 = phi float [ %history.i136.i.sroa.10.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.7.06391, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.7.06391 = phi float [ %history.i136.i.sroa.7.0.copyload, %bb5.i140.i.lr.ph ], [ %history.i136.i.sroa.0.06390, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ]
  %history.i136.i.sroa.0.06390 = phi float [ %history.i136.i.sroa.0.0.copyload, %bb5.i140.i.lr.ph ], [ %_0.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ]
  %168 = add nuw nsw i32 %iter.sroa.0.0.i138.i6401, 1, !dbg !4148
  %_11.i141.i = add nuw nsw i32 %iter.sroa.0.0.i138.i6401, %iter.sroa.0.0.i7267578, !dbg !4154
  %_24.i142.i = icmp ugt i32 %_11.i141.i, %left_io.1, !dbg !4156
  br i1 %_24.i142.i, label %bb7.i339.i, label %bb8.i143.i, !dbg !4156, !prof !755

bb8.i143.i:                                       ; preds = %bb5.i140.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4162), !dbg !4165
  %_3.not.i = icmp eq i32 %left_io.1, %_11.i141.i, !dbg !4166
  br i1 %_3.not.i, label %panic.i2912, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit, !dbg !4166

panic.i2912:                                      ; preds = %bb8.i143.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #28, !dbg !4166, !noalias !4168
  unreachable, !dbg !4166

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit: ; preds = %bb8.i143.i
  %_31.i145.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %_11.i141.i, !dbg !4170
  %_0.i = load float, ptr %_31.i145.i, align 4, !dbg !4166, !alias.scope !4162, !noalias !4175, !noundef !10
  %169 = tail call noundef float @llvm.fabs.f32(float %history.i136.i.sroa.19.06395), !dbg !4176
  %_0.i2462 = fmul float %_0.i, %_11.i.i.i160.i, !dbg !4182
  %_0.i2038 = fadd float %_0.i2462, 0.000000e+00, !dbg !4194
  %_0.i2461 = fmul float %_0.i, %_14.i.i.i163.i, !dbg !4197
  %_0.i2037 = fadd float %_0.i2461, 0.000000e+00, !dbg !4199
  %_0.i2460 = fmul float %_0.i, %_17.i.i.i166.i, !dbg !4201
  %_0.i2036 = fadd float %_0.i2460, 0.000000e+00, !dbg !4203
  %_0.i2459 = fmul float %_0.i, %_20.i.i.i169.i, !dbg !4205
  %_0.i2035 = fadd float %_0.i2459, 0.000000e+00, !dbg !4207
  %_0.i2458 = fmul float %history.i136.i.sroa.0.06390, %_25.i.i.i174.i, !dbg !4209
  %_0.i2034 = fadd float %_0.i2038, %_0.i2458, !dbg !4213
  %_0.i2457 = fmul float %history.i136.i.sroa.0.06390, %_28.i.i.i177.i, !dbg !4215
  %_0.i2033 = fadd float %_0.i2037, %_0.i2457, !dbg !4217
  %_0.i2456 = fmul float %history.i136.i.sroa.0.06390, %_31.i.i.i180.i, !dbg !4219
  %_0.i2032 = fadd float %_0.i2036, %_0.i2456, !dbg !4221
  %_0.i2455 = fmul float %history.i136.i.sroa.0.06390, %_34.i.i.i183.i, !dbg !4223
  %_0.i2031 = fadd float %_0.i2035, %_0.i2455, !dbg !4225
  %_0.i2454 = fmul float %history.i136.i.sroa.7.06391, %_39.i.i.i188.i, !dbg !4227
  %_0.i2030 = fadd float %_0.i2034, %_0.i2454, !dbg !4231
  %_0.i2453 = fmul float %history.i136.i.sroa.7.06391, %_42.i.i.i191.i, !dbg !4233
  %_0.i2029 = fadd float %_0.i2033, %_0.i2453, !dbg !4235
  %_0.i2452 = fmul float %history.i136.i.sroa.7.06391, %_45.i.i.i194.i, !dbg !4237
  %_0.i2028 = fadd float %_0.i2032, %_0.i2452, !dbg !4239
  %_0.i2451 = fmul float %history.i136.i.sroa.7.06391, %_48.i.i.i197.i, !dbg !4241
  %_0.i2027 = fadd float %_0.i2031, %_0.i2451, !dbg !4243
  %_0.i2450 = fmul float %history.i136.i.sroa.10.06392, %_53.i.i.i202.i, !dbg !4245
  %_0.i2026 = fadd float %_0.i2030, %_0.i2450, !dbg !4249
  %_0.i2449 = fmul float %history.i136.i.sroa.10.06392, %_56.i.i.i205.i, !dbg !4251
  %_0.i2025 = fadd float %_0.i2029, %_0.i2449, !dbg !4253
  %_0.i2448 = fmul float %history.i136.i.sroa.10.06392, %_59.i.i.i208.i, !dbg !4255
  %_0.i2024 = fadd float %_0.i2028, %_0.i2448, !dbg !4257
  %_0.i2447 = fmul float %history.i136.i.sroa.10.06392, %_62.i.i.i211.i, !dbg !4259
  %_0.i2023 = fadd float %_0.i2027, %_0.i2447, !dbg !4261
  %_0.i2446 = fmul float %history.i136.i.sroa.13.06393, %_67.i.i.i216.i, !dbg !4263
  %_0.i2022 = fadd float %_0.i2026, %_0.i2446, !dbg !4267
  %_0.i2445 = fmul float %history.i136.i.sroa.13.06393, %_70.i.i.i219.i, !dbg !4269
  %_0.i2021 = fadd float %_0.i2025, %_0.i2445, !dbg !4271
  %_0.i2444 = fmul float %history.i136.i.sroa.13.06393, %_73.i.i.i222.i, !dbg !4273
  %_0.i2020 = fadd float %_0.i2024, %_0.i2444, !dbg !4275
  %_0.i2443 = fmul float %history.i136.i.sroa.13.06393, %_76.i.i.i225.i, !dbg !4277
  %_0.i2019 = fadd float %_0.i2023, %_0.i2443, !dbg !4279
  %_0.i2442 = fmul float %history.i136.i.sroa.16.06394, %_81.i.i.i230.i, !dbg !4281
  %_0.i2018 = fadd float %_0.i2022, %_0.i2442, !dbg !4285
  %_0.i2441 = fmul float %history.i136.i.sroa.16.06394, %_84.i.i.i233.i, !dbg !4287
  %_0.i2017 = fadd float %_0.i2021, %_0.i2441, !dbg !4289
  %_0.i2440 = fmul float %history.i136.i.sroa.16.06394, %_87.i.i.i236.i, !dbg !4291
  %_0.i2016 = fadd float %_0.i2020, %_0.i2440, !dbg !4293
  %_0.i2439 = fmul float %history.i136.i.sroa.16.06394, %_90.i.i.i239.i, !dbg !4295
  %_0.i2015 = fadd float %_0.i2019, %_0.i2439, !dbg !4297
  %_0.i2438 = fmul float %history.i136.i.sroa.19.06395, %_95.i.i.i244.i, !dbg !4299
  %_0.i2014 = fadd float %_0.i2018, %_0.i2438, !dbg !4303
  %_0.i2437 = fmul float %history.i136.i.sroa.19.06395, %_98.i.i.i247.i, !dbg !4305
  %_0.i2013 = fadd float %_0.i2017, %_0.i2437, !dbg !4307
  %_0.i2436 = fmul float %history.i136.i.sroa.19.06395, %_101.i.i.i250.i, !dbg !4309
  %_0.i2012 = fadd float %_0.i2016, %_0.i2436, !dbg !4311
  %_0.i2435 = fmul float %history.i136.i.sroa.19.06395, %_104.i.i.i253.i, !dbg !4313
  %_0.i2011 = fadd float %_0.i2015, %_0.i2435, !dbg !4315
  %_0.i2434 = fmul float %history.i136.i.sroa.22.06396, %_109.i.i.i258.i, !dbg !4317
  %_0.i2010 = fadd float %_0.i2014, %_0.i2434, !dbg !4321
  %_0.i2433 = fmul float %history.i136.i.sroa.22.06396, %_112.i.i.i261.i, !dbg !4323
  %_0.i2009 = fadd float %_0.i2013, %_0.i2433, !dbg !4325
  %_0.i2432 = fmul float %history.i136.i.sroa.22.06396, %_115.i.i.i264.i, !dbg !4327
  %_0.i2008 = fadd float %_0.i2012, %_0.i2432, !dbg !4329
  %_0.i2431 = fmul float %history.i136.i.sroa.22.06396, %_118.i.i.i267.i, !dbg !4331
  %_0.i2007 = fadd float %_0.i2011, %_0.i2431, !dbg !4333
  %_0.i2430 = fmul float %history.i136.i.sroa.26.06397, %_123.i.i.i272.i, !dbg !4335
  %_0.i2006 = fadd float %_0.i2010, %_0.i2430, !dbg !4339
  %_0.i2429 = fmul float %history.i136.i.sroa.26.06397, %_126.i.i.i275.i, !dbg !4341
  %_0.i2005 = fadd float %_0.i2009, %_0.i2429, !dbg !4343
  %_0.i2428 = fmul float %history.i136.i.sroa.26.06397, %_129.i.i.i278.i, !dbg !4345
  %_0.i2004 = fadd float %_0.i2008, %_0.i2428, !dbg !4347
  %_0.i2427 = fmul float %history.i136.i.sroa.26.06397, %_132.i.i.i281.i, !dbg !4349
  %_0.i2003 = fadd float %_0.i2007, %_0.i2427, !dbg !4351
  %_0.i2426 = fmul float %history.i136.i.sroa.29.06398, %_137.i.i.i286.i, !dbg !4353
  %_0.i2002 = fadd float %_0.i2006, %_0.i2426, !dbg !4357
  %_0.i2425 = fmul float %history.i136.i.sroa.29.06398, %_140.i.i.i289.i, !dbg !4359
  %_0.i2001 = fadd float %_0.i2005, %_0.i2425, !dbg !4361
  %_0.i2424 = fmul float %history.i136.i.sroa.29.06398, %_143.i.i.i292.i, !dbg !4363
  %_0.i2000 = fadd float %_0.i2004, %_0.i2424, !dbg !4365
  %_0.i2423 = fmul float %history.i136.i.sroa.29.06398, %_146.i.i.i295.i, !dbg !4367
  %_0.i1999 = fadd float %_0.i2003, %_0.i2423, !dbg !4369
  %_0.i2422 = fmul float %history.i136.i.sroa.32.06399, %_151.i.i.i300.i, !dbg !4371
  %_0.i1998 = fadd float %_0.i2002, %_0.i2422, !dbg !4375
  %_0.i2421 = fmul float %history.i136.i.sroa.32.06399, %_154.i.i.i303.i, !dbg !4377
  %_0.i1997 = fadd float %_0.i2001, %_0.i2421, !dbg !4379
  %_0.i2420 = fmul float %history.i136.i.sroa.32.06399, %_157.i.i.i306.i, !dbg !4381
  %_0.i1996 = fadd float %_0.i2000, %_0.i2420, !dbg !4383
  %_0.i2419 = fmul float %history.i136.i.sroa.32.06399, %_160.i.i.i309.i, !dbg !4385
  %_0.i1995 = fadd float %_0.i1999, %_0.i2419, !dbg !4387
  %_0.i2418 = fmul float %history.i136.i.sroa.35.06400, %_165.i.i.i314.i, !dbg !4389
  %_0.i1994 = fadd float %_0.i1998, %_0.i2418, !dbg !4393
  %_0.i2417 = fmul float %history.i136.i.sroa.35.06400, %_168.i.i.i317.i, !dbg !4395
  %_0.i1993 = fadd float %_0.i1997, %_0.i2417, !dbg !4397
  %_0.i2416 = fmul float %history.i136.i.sroa.35.06400, %_171.i.i.i320.i, !dbg !4399
  %_0.i1992 = fadd float %_0.i1996, %_0.i2416, !dbg !4401
  %_0.i2415 = fmul float %history.i136.i.sroa.35.06400, %_174.i.i.i323.i, !dbg !4403
  %_0.i1991 = fadd float %_0.i1995, %_0.i2415, !dbg !4405
  %170 = tail call noundef float @llvm.fabs.f32(float %_0.i1994), !dbg !4407
  %_3.i.i3549.inv = fcmp ogt float %169, %170, !dbg !4411
  %_4.i.i.v = select i1 %_3.i.i3549.inv, float %169, float %170, !dbg !4411
  %171 = tail call noundef float @llvm.fabs.f32(float %_0.i1993), !dbg !4407
  %_3.i.i3549.inv.1 = fcmp ogt float %_4.i.i.v, %171, !dbg !4411
  %_4.i.i.v.1 = select i1 %_3.i.i3549.inv.1, float %_4.i.i.v, float %171, !dbg !4411
  %172 = tail call noundef float @llvm.fabs.f32(float %_0.i1992), !dbg !4407
  %_3.i.i3549.inv.2 = fcmp ogt float %_4.i.i.v.1, %172, !dbg !4411
  %_4.i.i.v.2 = select i1 %_3.i.i3549.inv.2, float %_4.i.i.v.1, float %172, !dbg !4411
  %173 = tail call noundef float @llvm.fabs.f32(float %_0.i1991), !dbg !4407
  %_3.i.i3549.inv.3 = fcmp ogt float %_4.i.i.v.2, %173, !dbg !4411
  %_4.i.i.v.3 = select i1 %_3.i.i3549.inv.3, float %_4.i.i.v.2, float %173, !dbg !4411
  %_39.i334.i = getelementptr inbounds nuw float, ptr %peaks_left.i705, i32 %iter.sroa.0.0.i138.i6401, !dbg !4418
  store float %_4.i.i.v.3, ptr %_39.i334.i, align 4, !dbg !4432, !alias.scope !4434, !noalias !4175
  %exitcond.not = icmp eq i32 %168, %umax11909, !dbg !4137
  br i1 %exitcond.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i, label %bb5.i140.i, !dbg !4147

bb7.i339.i:                                       ; preds = %bb5.i140.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_11.i141.i, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #28, !dbg !4437, !noalias !4175
  unreachable, !dbg !4437

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit, %bb37.i731
  %history.i136.i.sroa.0.0.lcssa = phi float [ %history.i136.i.sroa.0.0.copyload, %bb37.i731 ], [ %_0.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !4438
  %history.i136.i.sroa.7.0.lcssa = phi float [ %history.i136.i.sroa.7.0.copyload, %bb37.i731 ], [ %history.i136.i.sroa.0.06390, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !4438
  %history.i136.i.sroa.10.0.lcssa = phi float [ %history.i136.i.sroa.10.0.copyload, %bb37.i731 ], [ %history.i136.i.sroa.7.06391, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !4438
  %history.i136.i.sroa.13.0.lcssa = phi float [ %history.i136.i.sroa.13.0.copyload, %bb37.i731 ], [ %history.i136.i.sroa.10.06392, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !4438
  %history.i136.i.sroa.16.0.lcssa = phi float [ %history.i136.i.sroa.16.0.copyload, %bb37.i731 ], [ %history.i136.i.sroa.13.06393, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !4438
  %history.i136.i.sroa.19.0.lcssa = phi float [ %history.i136.i.sroa.19.0.copyload, %bb37.i731 ], [ %history.i136.i.sroa.16.06394, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !4438
  %history.i136.i.sroa.22.0.lcssa = phi float [ %history.i136.i.sroa.22.0.copyload, %bb37.i731 ], [ %history.i136.i.sroa.19.06395, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !4438
  %history.i136.i.sroa.26.0.lcssa = phi float [ %history.i136.i.sroa.26.0.copyload, %bb37.i731 ], [ %history.i136.i.sroa.22.06396, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !4438
  %history.i136.i.sroa.29.0.lcssa = phi float [ %history.i136.i.sroa.29.0.copyload, %bb37.i731 ], [ %history.i136.i.sroa.26.06397, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !4438
  %history.i136.i.sroa.32.0.lcssa = phi float [ %history.i136.i.sroa.32.0.copyload, %bb37.i731 ], [ %history.i136.i.sroa.29.06398, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !4438
  %history.i136.i.sroa.35.0.lcssa = phi float [ %history.i136.i.sroa.35.0.copyload, %bb37.i731 ], [ %history.i136.i.sroa.32.06399, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !4438
  %history.i136.i.sroa.38.0.lcssa = phi float [ %history.i136.i.sroa.38.0.copyload, %bb37.i731 ], [ %history.i136.i.sroa.35.06400, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit ], !dbg !4438
  store float %history.i136.i.sroa.0.0.lcssa, ptr %hot_left.i708, align 4, !dbg !4439, !noalias !4132
  store float %history.i136.i.sroa.7.0.lcssa, ptr %history.i136.i.sroa.7.0.hot_left.i708.sroa_idx, align 4, !dbg !4439, !noalias !4132
  store float %history.i136.i.sroa.10.0.lcssa, ptr %history.i136.i.sroa.10.0.hot_left.i708.sroa_idx, align 4, !dbg !4439, !noalias !4132
  store float %history.i136.i.sroa.13.0.lcssa, ptr %history.i136.i.sroa.13.0.hot_left.i708.sroa_idx, align 4, !dbg !4439, !noalias !4132
  store float %history.i136.i.sroa.16.0.lcssa, ptr %history.i136.i.sroa.16.0.hot_left.i708.sroa_idx, align 4, !dbg !4439, !noalias !4132
  store float %history.i136.i.sroa.19.0.lcssa, ptr %history.i136.i.sroa.19.0.hot_left.i708.sroa_idx, align 4, !dbg !4439, !noalias !4132
  store float %history.i136.i.sroa.22.0.lcssa, ptr %history.i136.i.sroa.22.0.hot_left.i708.sroa_idx, align 4, !dbg !4439, !noalias !4132
  store float %history.i136.i.sroa.26.0.lcssa, ptr %history.i136.i.sroa.26.0.hot_left.i708.sroa_idx, align 4, !dbg !4439, !noalias !4132
  store float %history.i136.i.sroa.29.0.lcssa, ptr %history.i136.i.sroa.29.0.hot_left.i708.sroa_idx, align 4, !dbg !4439, !noalias !4132
  store float %history.i136.i.sroa.32.0.lcssa, ptr %history.i136.i.sroa.32.0.hot_left.i708.sroa_idx, align 4, !dbg !4439, !noalias !4132
  store float %history.i136.i.sroa.35.0.lcssa, ptr %history.i136.i.sroa.35.0.hot_left.i708.sroa_idx, align 4, !dbg !4439, !noalias !4132
  store float %history.i136.i.sroa.38.0.lcssa, ptr %history.i136.i.sroa.38.0.hot_left.i708.sroa_idx, align 4, !dbg !4439, !noalias !4132
  %history.i.i701.sroa.0.0.copyload = load float, ptr %hot_right.i707, align 4, !dbg !4440, !noalias !4442
  %history.i.i701.sroa.7.0.copyload = load float, ptr %history.i.i701.sroa.7.0.hot_right.i707.sroa_idx, align 4, !dbg !4440, !noalias !4442
  %history.i.i701.sroa.10.0.copyload = load float, ptr %history.i.i701.sroa.10.0.hot_right.i707.sroa_idx, align 4, !dbg !4440, !noalias !4442
  %history.i.i701.sroa.13.0.copyload = load float, ptr %history.i.i701.sroa.13.0.hot_right.i707.sroa_idx, align 4, !dbg !4440, !noalias !4442
  %history.i.i701.sroa.16.0.copyload = load float, ptr %history.i.i701.sroa.16.0.hot_right.i707.sroa_idx, align 4, !dbg !4440, !noalias !4442
  %history.i.i701.sroa.19.0.copyload = load float, ptr %history.i.i701.sroa.19.0.hot_right.i707.sroa_idx, align 4, !dbg !4440, !noalias !4442
  %history.i.i701.sroa.22.0.copyload = load float, ptr %history.i.i701.sroa.22.0.hot_right.i707.sroa_idx, align 4, !dbg !4440, !noalias !4442
  %history.i.i701.sroa.26.0.copyload = load float, ptr %history.i.i701.sroa.26.0.hot_right.i707.sroa_idx, align 4, !dbg !4440, !noalias !4442
  %history.i.i701.sroa.29.0.copyload = load float, ptr %history.i.i701.sroa.29.0.hot_right.i707.sroa_idx, align 4, !dbg !4440, !noalias !4442
  %history.i.i701.sroa.32.0.copyload = load float, ptr %history.i.i701.sroa.32.0.hot_right.i707.sroa_idx, align 4, !dbg !4440, !noalias !4442
  %history.i.i701.sroa.35.0.copyload = load float, ptr %history.i.i701.sroa.35.0.hot_right.i707.sroa_idx, align 4, !dbg !4440, !noalias !4442
  %history.i.i701.sroa.38.0.copyload = load float, ptr %history.i.i701.sroa.38.0.hot_right.i707.sroa_idx, align 4, !dbg !4440, !noalias !4442
  br i1 %_20.i139.i6389.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i736, label %bb5.i.i962.lr.ph, !dbg !4447

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
  br label %bb5.i.i962, !dbg !4447

bb5.i.i962:                                       ; preds = %bb5.i.i962.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917
  %iter.sroa.0.0.i.i7346427 = phi i32 [ 0, %bb5.i.i962.lr.ph ], [ %174, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ]
  %history.i.i701.sroa.35.06426 = phi float [ %history.i.i701.sroa.35.0.copyload, %bb5.i.i962.lr.ph ], [ %history.i.i701.sroa.32.06425, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ]
  %history.i.i701.sroa.32.06425 = phi float [ %history.i.i701.sroa.32.0.copyload, %bb5.i.i962.lr.ph ], [ %history.i.i701.sroa.29.06424, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ]
  %history.i.i701.sroa.29.06424 = phi float [ %history.i.i701.sroa.29.0.copyload, %bb5.i.i962.lr.ph ], [ %history.i.i701.sroa.26.06423, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ]
  %history.i.i701.sroa.26.06423 = phi float [ %history.i.i701.sroa.26.0.copyload, %bb5.i.i962.lr.ph ], [ %history.i.i701.sroa.22.06422, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ]
  %history.i.i701.sroa.22.06422 = phi float [ %history.i.i701.sroa.22.0.copyload, %bb5.i.i962.lr.ph ], [ %history.i.i701.sroa.19.06421, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ]
  %history.i.i701.sroa.19.06421 = phi float [ %history.i.i701.sroa.19.0.copyload, %bb5.i.i962.lr.ph ], [ %history.i.i701.sroa.16.06420, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ]
  %history.i.i701.sroa.16.06420 = phi float [ %history.i.i701.sroa.16.0.copyload, %bb5.i.i962.lr.ph ], [ %history.i.i701.sroa.13.06419, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ]
  %history.i.i701.sroa.13.06419 = phi float [ %history.i.i701.sroa.13.0.copyload, %bb5.i.i962.lr.ph ], [ %history.i.i701.sroa.10.06418, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ]
  %history.i.i701.sroa.10.06418 = phi float [ %history.i.i701.sroa.10.0.copyload, %bb5.i.i962.lr.ph ], [ %history.i.i701.sroa.7.06417, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ]
  %history.i.i701.sroa.7.06417 = phi float [ %history.i.i701.sroa.7.0.copyload, %bb5.i.i962.lr.ph ], [ %history.i.i701.sroa.0.06416, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ]
  %history.i.i701.sroa.0.06416 = phi float [ %history.i.i701.sroa.0.0.copyload, %bb5.i.i962.lr.ph ], [ %_0.i2915, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ]
  %174 = add nuw nsw i32 %iter.sroa.0.0.i.i7346427, 1, !dbg !4450
  %_11.i126.i = add nuw nsw i32 %iter.sroa.0.0.i.i7346427, %iter.sroa.0.0.i7267578, !dbg !4453
  %_24.i.i963 = icmp ugt i32 %_11.i126.i, %right_io.1, !dbg !4454
  br i1 %_24.i.i963, label %bb7.i.i1159, label %bb8.i.i964, !dbg !4454, !prof !755

bb8.i.i964:                                       ; preds = %bb5.i.i962
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4457), !dbg !4460
  %_3.not.i2913 = icmp eq i32 %right_io.1, %_11.i126.i, !dbg !4461
  br i1 %_3.not.i2913, label %panic.i2916, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917, !dbg !4461

panic.i2916:                                      ; preds = %bb8.i.i964
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #28, !dbg !4461, !noalias !4463
  unreachable, !dbg !4461

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917: ; preds = %bb8.i.i964
  %_31.i127.i = getelementptr inbounds nuw float, ptr %right_io.0, i32 %_11.i126.i, !dbg !4465
  %_0.i2915 = load float, ptr %_31.i127.i, align 4, !dbg !4461, !alias.scope !4457, !noalias !4467, !noundef !10
  %175 = tail call noundef float @llvm.fabs.f32(float %history.i.i701.sroa.19.06421), !dbg !4468
  %_0.i2510 = fmul float %_0.i2915, %_11.i.i.i.i980, !dbg !4471
  %_0.i2086 = fadd float %_0.i2510, 0.000000e+00, !dbg !4474
  %_0.i2509 = fmul float %_0.i2915, %_14.i.i.i.i983, !dbg !4476
  %_0.i2085 = fadd float %_0.i2509, 0.000000e+00, !dbg !4478
  %_0.i2508 = fmul float %_0.i2915, %_17.i.i.i.i986, !dbg !4480
  %_0.i2084 = fadd float %_0.i2508, 0.000000e+00, !dbg !4482
  %_0.i2507 = fmul float %_0.i2915, %_20.i.i.i.i989, !dbg !4484
  %_0.i2083 = fadd float %_0.i2507, 0.000000e+00, !dbg !4486
  %_0.i2506 = fmul float %history.i.i701.sroa.0.06416, %_25.i.i.i.i994, !dbg !4488
  %_0.i2082 = fadd float %_0.i2086, %_0.i2506, !dbg !4490
  %_0.i2505 = fmul float %history.i.i701.sroa.0.06416, %_28.i.i.i.i997, !dbg !4492
  %_0.i2081 = fadd float %_0.i2085, %_0.i2505, !dbg !4494
  %_0.i2504 = fmul float %history.i.i701.sroa.0.06416, %_31.i.i.i.i1000, !dbg !4496
  %_0.i2080 = fadd float %_0.i2084, %_0.i2504, !dbg !4498
  %_0.i2503 = fmul float %history.i.i701.sroa.0.06416, %_34.i.i.i.i1003, !dbg !4500
  %_0.i2079 = fadd float %_0.i2083, %_0.i2503, !dbg !4502
  %_0.i2502 = fmul float %history.i.i701.sroa.7.06417, %_39.i.i.i.i1008, !dbg !4504
  %_0.i2078 = fadd float %_0.i2082, %_0.i2502, !dbg !4506
  %_0.i2501 = fmul float %history.i.i701.sroa.7.06417, %_42.i.i.i.i1011, !dbg !4508
  %_0.i2077 = fadd float %_0.i2081, %_0.i2501, !dbg !4510
  %_0.i2500 = fmul float %history.i.i701.sroa.7.06417, %_45.i.i.i.i1014, !dbg !4512
  %_0.i2076 = fadd float %_0.i2080, %_0.i2500, !dbg !4514
  %_0.i2499 = fmul float %history.i.i701.sroa.7.06417, %_48.i.i.i.i1017, !dbg !4516
  %_0.i2075 = fadd float %_0.i2079, %_0.i2499, !dbg !4518
  %_0.i2498 = fmul float %history.i.i701.sroa.10.06418, %_53.i.i.i.i1022, !dbg !4520
  %_0.i2074 = fadd float %_0.i2078, %_0.i2498, !dbg !4522
  %_0.i2497 = fmul float %history.i.i701.sroa.10.06418, %_56.i.i.i.i1025, !dbg !4524
  %_0.i2073 = fadd float %_0.i2077, %_0.i2497, !dbg !4526
  %_0.i2496 = fmul float %history.i.i701.sroa.10.06418, %_59.i.i.i.i1028, !dbg !4528
  %_0.i2072 = fadd float %_0.i2076, %_0.i2496, !dbg !4530
  %_0.i2495 = fmul float %history.i.i701.sroa.10.06418, %_62.i.i.i.i1031, !dbg !4532
  %_0.i2071 = fadd float %_0.i2075, %_0.i2495, !dbg !4534
  %_0.i2494 = fmul float %history.i.i701.sroa.13.06419, %_67.i.i.i.i1036, !dbg !4536
  %_0.i2070 = fadd float %_0.i2074, %_0.i2494, !dbg !4538
  %_0.i2493 = fmul float %history.i.i701.sroa.13.06419, %_70.i.i.i.i1039, !dbg !4540
  %_0.i2069 = fadd float %_0.i2073, %_0.i2493, !dbg !4542
  %_0.i2492 = fmul float %history.i.i701.sroa.13.06419, %_73.i.i.i.i1042, !dbg !4544
  %_0.i2068 = fadd float %_0.i2072, %_0.i2492, !dbg !4546
  %_0.i2491 = fmul float %history.i.i701.sroa.13.06419, %_76.i.i.i.i1045, !dbg !4548
  %_0.i2067 = fadd float %_0.i2071, %_0.i2491, !dbg !4550
  %_0.i2490 = fmul float %history.i.i701.sroa.16.06420, %_81.i.i.i.i1050, !dbg !4552
  %_0.i2066 = fadd float %_0.i2070, %_0.i2490, !dbg !4554
  %_0.i2489 = fmul float %history.i.i701.sroa.16.06420, %_84.i.i.i.i1053, !dbg !4556
  %_0.i2065 = fadd float %_0.i2069, %_0.i2489, !dbg !4558
  %_0.i2488 = fmul float %history.i.i701.sroa.16.06420, %_87.i.i.i.i1056, !dbg !4560
  %_0.i2064 = fadd float %_0.i2068, %_0.i2488, !dbg !4562
  %_0.i2487 = fmul float %history.i.i701.sroa.16.06420, %_90.i.i.i.i1059, !dbg !4564
  %_0.i2063 = fadd float %_0.i2067, %_0.i2487, !dbg !4566
  %_0.i2486 = fmul float %history.i.i701.sroa.19.06421, %_95.i.i.i.i1064, !dbg !4568
  %_0.i2062 = fadd float %_0.i2066, %_0.i2486, !dbg !4570
  %_0.i2485 = fmul float %history.i.i701.sroa.19.06421, %_98.i.i.i.i1067, !dbg !4572
  %_0.i2061 = fadd float %_0.i2065, %_0.i2485, !dbg !4574
  %_0.i2484 = fmul float %history.i.i701.sroa.19.06421, %_101.i.i.i.i1070, !dbg !4576
  %_0.i2060 = fadd float %_0.i2064, %_0.i2484, !dbg !4578
  %_0.i2483 = fmul float %history.i.i701.sroa.19.06421, %_104.i.i.i.i1073, !dbg !4580
  %_0.i2059 = fadd float %_0.i2063, %_0.i2483, !dbg !4582
  %_0.i2482 = fmul float %history.i.i701.sroa.22.06422, %_109.i.i.i.i1078, !dbg !4584
  %_0.i2058 = fadd float %_0.i2062, %_0.i2482, !dbg !4586
  %_0.i2481 = fmul float %history.i.i701.sroa.22.06422, %_112.i.i.i.i1081, !dbg !4588
  %_0.i2057 = fadd float %_0.i2061, %_0.i2481, !dbg !4590
  %_0.i2480 = fmul float %history.i.i701.sroa.22.06422, %_115.i.i.i.i1084, !dbg !4592
  %_0.i2056 = fadd float %_0.i2060, %_0.i2480, !dbg !4594
  %_0.i2479 = fmul float %history.i.i701.sroa.22.06422, %_118.i.i.i.i1087, !dbg !4596
  %_0.i2055 = fadd float %_0.i2059, %_0.i2479, !dbg !4598
  %_0.i2478 = fmul float %history.i.i701.sroa.26.06423, %_123.i.i.i.i1092, !dbg !4600
  %_0.i2054 = fadd float %_0.i2058, %_0.i2478, !dbg !4602
  %_0.i2477 = fmul float %history.i.i701.sroa.26.06423, %_126.i.i.i.i1095, !dbg !4604
  %_0.i2053 = fadd float %_0.i2057, %_0.i2477, !dbg !4606
  %_0.i2476 = fmul float %history.i.i701.sroa.26.06423, %_129.i.i.i.i1098, !dbg !4608
  %_0.i2052 = fadd float %_0.i2056, %_0.i2476, !dbg !4610
  %_0.i2475 = fmul float %history.i.i701.sroa.26.06423, %_132.i.i.i.i1101, !dbg !4612
  %_0.i2051 = fadd float %_0.i2055, %_0.i2475, !dbg !4614
  %_0.i2474 = fmul float %history.i.i701.sroa.29.06424, %_137.i.i.i.i1106, !dbg !4616
  %_0.i2050 = fadd float %_0.i2054, %_0.i2474, !dbg !4618
  %_0.i2473 = fmul float %history.i.i701.sroa.29.06424, %_140.i.i.i.i1109, !dbg !4620
  %_0.i2049 = fadd float %_0.i2053, %_0.i2473, !dbg !4622
  %_0.i2472 = fmul float %history.i.i701.sroa.29.06424, %_143.i.i.i.i1112, !dbg !4624
  %_0.i2048 = fadd float %_0.i2052, %_0.i2472, !dbg !4626
  %_0.i2471 = fmul float %history.i.i701.sroa.29.06424, %_146.i.i.i.i1115, !dbg !4628
  %_0.i2047 = fadd float %_0.i2051, %_0.i2471, !dbg !4630
  %_0.i2470 = fmul float %history.i.i701.sroa.32.06425, %_151.i.i.i.i1120, !dbg !4632
  %_0.i2046 = fadd float %_0.i2050, %_0.i2470, !dbg !4634
  %_0.i2469 = fmul float %history.i.i701.sroa.32.06425, %_154.i.i.i.i1123, !dbg !4636
  %_0.i2045 = fadd float %_0.i2049, %_0.i2469, !dbg !4638
  %_0.i2468 = fmul float %history.i.i701.sroa.32.06425, %_157.i.i.i.i1126, !dbg !4640
  %_0.i2044 = fadd float %_0.i2048, %_0.i2468, !dbg !4642
  %_0.i2467 = fmul float %history.i.i701.sroa.32.06425, %_160.i.i.i.i1129, !dbg !4644
  %_0.i2043 = fadd float %_0.i2047, %_0.i2467, !dbg !4646
  %_0.i2466 = fmul float %history.i.i701.sroa.35.06426, %_165.i.i.i.i1134, !dbg !4648
  %_0.i2042 = fadd float %_0.i2046, %_0.i2466, !dbg !4650
  %_0.i2465 = fmul float %history.i.i701.sroa.35.06426, %_168.i.i.i.i1137, !dbg !4652
  %_0.i2041 = fadd float %_0.i2045, %_0.i2465, !dbg !4654
  %_0.i2464 = fmul float %history.i.i701.sroa.35.06426, %_171.i.i.i.i1140, !dbg !4656
  %_0.i2040 = fadd float %_0.i2044, %_0.i2464, !dbg !4658
  %_0.i2463 = fmul float %history.i.i701.sroa.35.06426, %_174.i.i.i.i1143, !dbg !4660
  %_0.i2039 = fadd float %_0.i2043, %_0.i2463, !dbg !4662
  %176 = tail call noundef float @llvm.fabs.f32(float %_0.i2042), !dbg !4664
  %_3.i.i3557.inv = fcmp ogt float %175, %176, !dbg !4666
  %_4.i.i3564.v = select i1 %_3.i.i3557.inv, float %175, float %176, !dbg !4666
  %177 = tail call noundef float @llvm.fabs.f32(float %_0.i2041), !dbg !4664
  %_3.i.i3557.inv.1 = fcmp ogt float %_4.i.i3564.v, %177, !dbg !4666
  %_4.i.i3564.v.1 = select i1 %_3.i.i3557.inv.1, float %_4.i.i3564.v, float %177, !dbg !4666
  %178 = tail call noundef float @llvm.fabs.f32(float %_0.i2040), !dbg !4664
  %_3.i.i3557.inv.2 = fcmp ogt float %_4.i.i3564.v.1, %178, !dbg !4666
  %_4.i.i3564.v.2 = select i1 %_3.i.i3557.inv.2, float %_4.i.i3564.v.1, float %178, !dbg !4666
  %179 = tail call noundef float @llvm.fabs.f32(float %_0.i2039), !dbg !4664
  %_3.i.i3557.inv.3 = fcmp ogt float %_4.i.i3564.v.2, %179, !dbg !4666
  %_4.i.i3564.v.3 = select i1 %_3.i.i3557.inv.3, float %_4.i.i3564.v.2, float %179, !dbg !4666
  %_39.i.i1154 = getelementptr inbounds nuw float, ptr %peaks_right.i704, i32 %iter.sroa.0.0.i.i7346427, !dbg !4669
  store float %_4.i.i3564.v.3, ptr %_39.i.i1154, align 4, !dbg !4674, !alias.scope !4676, !noalias !4467
  %exitcond11897.not = icmp eq i32 %174, %umax11909, !dbg !4679
  br i1 %exitcond11897.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i736, label %bb5.i.i962, !dbg !4447

bb7.i.i1159:                                      ; preds = %bb5.i.i962
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_11.i126.i, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #28, !dbg !4681, !noalias !4467
  unreachable, !dbg !4681

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i736: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i
  %history.i.i701.sroa.0.0.lcssa = phi float [ %history.i.i701.sroa.0.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %_0.i2915, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ], !dbg !4682
  %history.i.i701.sroa.7.0.lcssa = phi float [ %history.i.i701.sroa.7.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i701.sroa.0.06416, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ], !dbg !4682
  %history.i.i701.sroa.10.0.lcssa = phi float [ %history.i.i701.sroa.10.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i701.sroa.7.06417, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ], !dbg !4682
  %history.i.i701.sroa.13.0.lcssa = phi float [ %history.i.i701.sroa.13.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i701.sroa.10.06418, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ], !dbg !4682
  %history.i.i701.sroa.16.0.lcssa = phi float [ %history.i.i701.sroa.16.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i701.sroa.13.06419, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ], !dbg !4682
  %history.i.i701.sroa.19.0.lcssa = phi float [ %history.i.i701.sroa.19.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i701.sroa.16.06420, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ], !dbg !4682
  %history.i.i701.sroa.22.0.lcssa = phi float [ %history.i.i701.sroa.22.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i701.sroa.19.06421, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ], !dbg !4682
  %history.i.i701.sroa.26.0.lcssa = phi float [ %history.i.i701.sroa.26.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i701.sroa.22.06422, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ], !dbg !4682
  %history.i.i701.sroa.29.0.lcssa = phi float [ %history.i.i701.sroa.29.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i701.sroa.26.06423, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ], !dbg !4682
  %history.i.i701.sroa.32.0.lcssa = phi float [ %history.i.i701.sroa.32.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i701.sroa.29.06424, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ], !dbg !4682
  %history.i.i701.sroa.35.0.lcssa = phi float [ %history.i.i701.sroa.35.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i701.sroa.32.06425, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ], !dbg !4682
  %history.i.i701.sroa.38.0.lcssa = phi float [ %history.i.i701.sroa.38.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit340.i ], [ %history.i.i701.sroa.35.06426, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2917 ], !dbg !4682
  store float %history.i.i701.sroa.0.0.lcssa, ptr %hot_right.i707, align 4, !dbg !4683, !noalias !4442
  store float %history.i.i701.sroa.7.0.lcssa, ptr %history.i.i701.sroa.7.0.hot_right.i707.sroa_idx, align 4, !dbg !4683, !noalias !4442
  store float %history.i.i701.sroa.10.0.lcssa, ptr %history.i.i701.sroa.10.0.hot_right.i707.sroa_idx, align 4, !dbg !4683, !noalias !4442
  store float %history.i.i701.sroa.13.0.lcssa, ptr %history.i.i701.sroa.13.0.hot_right.i707.sroa_idx, align 4, !dbg !4683, !noalias !4442
  store float %history.i.i701.sroa.16.0.lcssa, ptr %history.i.i701.sroa.16.0.hot_right.i707.sroa_idx, align 4, !dbg !4683, !noalias !4442
  store float %history.i.i701.sroa.19.0.lcssa, ptr %history.i.i701.sroa.19.0.hot_right.i707.sroa_idx, align 4, !dbg !4683, !noalias !4442
  store float %history.i.i701.sroa.22.0.lcssa, ptr %history.i.i701.sroa.22.0.hot_right.i707.sroa_idx, align 4, !dbg !4683, !noalias !4442
  store float %history.i.i701.sroa.26.0.lcssa, ptr %history.i.i701.sroa.26.0.hot_right.i707.sroa_idx, align 4, !dbg !4683, !noalias !4442
  store float %history.i.i701.sroa.29.0.lcssa, ptr %history.i.i701.sroa.29.0.hot_right.i707.sroa_idx, align 4, !dbg !4683, !noalias !4442
  store float %history.i.i701.sroa.32.0.lcssa, ptr %history.i.i701.sroa.32.0.hot_right.i707.sroa_idx, align 4, !dbg !4683, !noalias !4442
  store float %history.i.i701.sroa.35.0.lcssa, ptr %history.i.i701.sroa.35.0.hot_right.i707.sroa_idx, align 4, !dbg !4683, !noalias !4442
  store float %history.i.i701.sroa.38.0.lcssa, ptr %history.i.i701.sroa.38.0.hot_right.i707.sroa_idx, align 4, !dbg !4683, !noalias !4442
  br i1 %_20.i139.i6389.not, label %bb13.i725.loopexit, label %bb42.i742.lr.ph, !dbg !4115

bb42.i742.lr.ph:                                  ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i736
  %_13.i188545004502 = load float, ptr %123, align 4, !alias.scope !4058, !noalias !4061, !noundef !10
  %_13.i187345034505 = load float, ptr %126, align 4, !alias.scope !4069, !noalias !4072, !noundef !10
  %_13.i186145064508 = load float, ptr %129, align 4, !alias.scope !4079, !noalias !4082, !noundef !10
  %_13.i185045094511 = load float, ptr %132, align 4, !alias.scope !4089, !noalias !4072, !noundef !10
  %_87.i763 = load i32, ptr %133, align 4
  %_62.i89.i820 = load float, ptr %144, align 4
  %_62.i.i913 = load float, ptr %160, align 4
  %_102.i945 = load i32, ptr %164, align 4
  %.promoted = load float, ptr %121, align 4, !alias.scope !4058, !noalias !4061
  %_64.i744.promoted = load float, ptr %_64.i744, align 4, !alias.scope !4058, !noalias !4061
  %.promoted6607 = load float, ptr %122, align 4, !alias.scope !4058, !noalias !4061
  %.promoted6677 = load float, ptr %124, align 4, !alias.scope !4069, !noalias !4072
  %_65.i745.promoted = load float, ptr %_65.i745, align 4, !alias.scope !4069, !noalias !4072
  %.promoted6815 = load float, ptr %125, align 4, !alias.scope !4069, !noalias !4072
  %.promoted6885 = load float, ptr %127, align 4, !alias.scope !4079, !noalias !4082
  %_69.i746.promoted = load float, ptr %_69.i746, align 4, !alias.scope !4079, !noalias !4082
  %.promoted7023 = load float, ptr %128, align 4, !alias.scope !4079, !noalias !4082
  %.promoted7093 = load float, ptr %130, align 4, !alias.scope !4089, !noalias !4072
  %_70.i747.promoted = load float, ptr %_70.i747, align 4, !alias.scope !4089, !noalias !4072
  %.promoted7231 = load float, ptr %131, align 4, !alias.scope !4089, !noalias !4072
  %.promoted7301 = load float, ptr %143, align 4
  %.promoted7370 = load float, ptr %145, align 4
  %.promoted7439 = load float, ptr %159, align 4
  %.promoted7508 = load float, ptr %161, align 4
  br label %bb42.i742, !dbg !4115

bb42.i742:                                        ; preds = %bb42.i742.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077
  %_0.i32087509 = phi float [ %.promoted7508, %bb42.i742.lr.ph ], [ %_0.i3208, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ]
  %_0.i28347440 = phi float [ %.promoted7439, %bb42.i742.lr.ph ], [ %_0.i2834, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ]
  %_0.i32127371 = phi float [ %.promoted7370, %bb42.i742.lr.ph ], [ %_0.i3212, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ]
  %_0.i28387302 = phi float [ %.promoted7301, %bb42.i742.lr.ph ], [ %_0.i2838, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ]
  %_12.i18487232 = phi float [ %.promoted7231, %bb42.i742.lr.ph ], [ %_0.i3331, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ], !dbg !4684
  %_0.i33387163 = phi float [ %_70.i747.promoted, %bb42.i742.lr.ph ], [ %_0.i3338, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ], !dbg !4684
  %_5.i18457094 = phi float [ %.promoted7093, %bb42.i742.lr.ph ], [ %_0.i.i3548, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ], !dbg !4684
  %_12.i18597024 = phi float [ %.promoted7023, %bb42.i742.lr.ph ], [ %_0.i3318, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ], !dbg !4684
  %_0.i33256955 = phi float [ %_69.i746.promoted, %bb42.i742.lr.ph ], [ %_0.i3325, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ], !dbg !4684
  %_5.i18536886 = phi float [ %.promoted6885, %bb42.i742.lr.ph ], [ %_0.i.i3541, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ], !dbg !4684
  %_12.i18716816 = phi float [ %.promoted6815, %bb42.i742.lr.ph ], [ %_0.i3305, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ], !dbg !4684
  %_0.i33126747 = phi float [ %_65.i745.promoted, %bb42.i742.lr.ph ], [ %_0.i3312, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ], !dbg !4684
  %_5.i18656678 = phi float [ %.promoted6677, %bb42.i742.lr.ph ], [ %_0.i.i3534, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ], !dbg !4684
  %_12.i18836608 = phi float [ %.promoted6607, %bb42.i742.lr.ph ], [ %_0.i3292, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ], !dbg !4684
  %_0.i32996539 = phi float [ %_64.i744.promoted, %bb42.i742.lr.ph ], [ %_0.i3299, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ], !dbg !4684
  %_5.i18776470 = phi float [ %.promoted, %bb42.i742.lr.ph ], [ %_0.i.i3527, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ], !dbg !4684
  %main_cursor.sroa.0.1.i7406467 = phi i32 [ %main_cursor.sroa.0.0.i7297581, %bb42.i742.lr.ph ], [ %spec.store.select11.i947, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ]
  %ring_cursor.sroa.0.1.i7396466 = phi i32 [ %ring_cursor.sroa.0.0.i7287580, %bb42.i742.lr.ph ], [ %spec.store.select12.i949, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ]
  %iter1.sroa.0.0.i7386465 = phi i32 [ 0, %bb42.i742.lr.ph ], [ %180, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077 ]
  %180 = add nuw nsw i32 %iter1.sroa.0.0.i7386465, 1, !dbg !4684
  %_60.i743 = add nuw nsw i32 %iter1.sroa.0.0.i7386465, %iter.sroa.0.0.i7267578, !dbg !4690
  %_0.i2827 = fadd float %_5.i18776470, -1.000000e+00, !dbg !4691
  %_3.i.i3521 = fcmp ogt float %_0.i2827, 0.000000e+00, !dbg !4694
  %_0.i.i3527 = select i1 %_3.i.i3521, float %_0.i2827, float 0.000000e+00, !dbg !4698
  %_0.i1987 = fadd float %_0.i32996539, %_12.i18836608, !dbg !4700
  %_0.i3299 = select i1 %_3.i.i3521, float %_0.i1987, float %_13.i188545004502, !dbg !4702
  %_0.i3292 = select i1 %_3.i.i3521, float %_12.i18836608, float 0.000000e+00, !dbg !4704
  %_0.i2828 = fadd float %_5.i18656678, -1.000000e+00, !dbg !4706
  %_3.i.i3528 = fcmp ogt float %_0.i2828, 0.000000e+00, !dbg !4708
  %_0.i.i3534 = select i1 %_3.i.i3528, float %_0.i2828, float 0.000000e+00, !dbg !4711
  %_0.i1988 = fadd float %_0.i33126747, %_12.i18716816, !dbg !4713
  %_0.i3312 = select i1 %_3.i.i3528, float %_0.i1988, float %_13.i187345034505, !dbg !4715
  %_0.i3305 = select i1 %_3.i.i3528, float %_12.i18716816, float 0.000000e+00, !dbg !4717
  %_0.i2829 = fadd float %_5.i18536886, -1.000000e+00, !dbg !4719
  %_3.i.i3535 = fcmp ogt float %_0.i2829, 0.000000e+00, !dbg !4721
  %_0.i.i3541 = select i1 %_3.i.i3535, float %_0.i2829, float 0.000000e+00, !dbg !4724
  %_0.i1989 = fadd float %_0.i33256955, %_12.i18597024, !dbg !4726
  %_0.i3325 = select i1 %_3.i.i3535, float %_0.i1989, float %_13.i186145064508, !dbg !4728
  %_0.i3318 = select i1 %_3.i.i3535, float %_12.i18597024, float 0.000000e+00, !dbg !4730
  %_0.i2830 = fadd float %_5.i18457094, -1.000000e+00, !dbg !4732
  %_3.i.i3542 = fcmp ogt float %_0.i2830, 0.000000e+00, !dbg !4734
  %_0.i.i3548 = select i1 %_3.i.i3542, float %_0.i2830, float 0.000000e+00, !dbg !4737
  %_0.i1990 = fadd float %_0.i33387163, %_12.i18487232, !dbg !4739
  %_0.i3338 = select i1 %_3.i.i3542, float %_0.i1990, float %_13.i185045094511, !dbg !4741
  %_0.i3331 = select i1 %_3.i.i3542, float %_12.i18487232, float 0.000000e+00, !dbg !4743
  %_126.i751 = getelementptr inbounds nuw float, ptr %peaks_left.i705, i32 %iter1.sroa.0.0.i7386465, !dbg !4745
  %_0.i2953 = load float, ptr %_126.i751, align 4, !dbg !4757, !alias.scope !4759, !noalias !4072, !noundef !10
  %_131.i753 = getelementptr inbounds nuw float, ptr %peaks_right.i704, i32 %iter1.sroa.0.0.i7386465, !dbg !4762
  %_0.i2948 = load float, ptr %_131.i753, align 4, !dbg !4772, !alias.scope !4774, !noalias !4072, !noundef !10
  %_3.i.i3584 = fcmp ule float %_0.i2948, %_0.i2953, !dbg !4777
  %_6.i.i3586 = bitcast float %_0.i2948 to i32, !dbg !4780
  %_8.i.i3588 = bitcast float %_0.i2953 to i32, !dbg !4784
  %_4.i.i3591 = select i1 %_3.i.i3584, i32 %_8.i.i3588, i32 %_6.i.i3586, !dbg !4786
  %_5.i3372 = and i32 %_4.i.i3591, %.none.i713, !dbg !4787
  %_7.i3375 = and i32 %_9.i3374, %_8.i.i3588, !dbg !4789
  %_4.i3376 = or disjoint i32 %_5.i3372, %_7.i3375, !dbg !4787
  %_0.i3377 = bitcast i32 %_4.i3376 to float, !dbg !4790
  %_7.i3368 = and i32 %_9.i3374, %_6.i.i3586, !dbg !4793
  %_4.i3369 = or disjoint i32 %_5.i3372, %_7.i3368, !dbg !4795
  %_0.i3370 = bitcast i32 %_4.i3369 to float, !dbg !4796
  %_132.i758 = icmp ugt i32 %_60.i743, %left_io.1, !dbg !4798
  br i1 %_132.i758, label %bb46.i961, label %bb47.i759, !dbg !4798, !prof !755

bb47.i759:                                        ; preds = %bb42.i742
  %_139.i761 = getelementptr inbounds nuw float, ptr %left_io.0, i32 %_60.i743, !dbg !4802
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4807), !dbg !4810
  %_3.not.i2941 = icmp eq i32 %left_io.1, %_60.i743, !dbg !4811
  br i1 %_3.not.i2941, label %panic.i2944, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2945, !dbg !4811

panic.i2944:                                      ; preds = %bb47.i759
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #28, !dbg !4811, !noalias !4813
  unreachable, !dbg !4811

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2945: ; preds = %bb47.i759
  %_0.i2943 = load float, ptr %_139.i761, align 4, !dbg !4811, !alias.scope !4807, !noalias !4072, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4814), !dbg !4817
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4818), !dbg !4817
  %width.i33.i764 = load i32, ptr %134, align 4, !dbg !4820, !alias.scope !4821, !noalias !4822, !noundef !10
  %_3.i1953 = fcmp uge float %_0.i3299, %_0.i3377, !dbg !4825
  %_0.i2386 = fdiv float %_0.i3299, %_0.i3377, !dbg !4827
  %_0.i3363 = select i1 %_3.i1953, float 1.000000e+00, float %_0.i2386, !dbg !4830
  %_158.1.i38.i769 = load i32, ptr %135, align 4, !dbg !4832, !alias.scope !4821, !noalias !4822, !noundef !10
  %_22.i39.i770 = mul i32 %width.i33.i764, %ring_cursor.sroa.0.1.i7396466, !dbg !4833
  %_90.i40.i771 = icmp ugt i32 %_22.i39.i770, %_158.1.i38.i769, !dbg !4834
  br i1 %_90.i40.i771, label %bb34.i124.i960, label %bb35.i41.i772, !dbg !4834, !prof !755

bb35.i41.i772:                                    ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2945
  %_158.0.i42.i773 = load ptr, ptr %136, align 4, !dbg !4832, !alias.scope !4821, !noalias !4822, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4839), !dbg !4842
  %_4.not.i3102 = icmp eq i32 %_158.1.i38.i769, %_22.i39.i770, !dbg !4843
  br i1 %_4.not.i3102, label %panic.i3104, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3105, !dbg !4843

panic.i3104:                                      ; preds = %bb35.i41.i772
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #28, !dbg !4843, !noalias !4845
  unreachable, !dbg !4843

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3105: ; preds = %bb35.i41.i772
  %_97.i44.i775 = getelementptr inbounds nuw float, ptr %_158.0.i42.i773, i32 %_22.i39.i770, !dbg !4846
  store float %_0.i3363, ptr %_97.i44.i775, align 4, !dbg !4843, !alias.scope !4839, !noalias !4851
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4852), !dbg !4855
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4856), !dbg !4855
  %width.i1477 = load i32, ptr %134, align 4, !dbg !4858, !alias.scope !4852, !noalias !4861, !noundef !10
  %181 = icmp eq i32 %width.i1477, 0, !dbg !4862
  br i1 %181, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1582, label %bb29.i1483.lr.ph, !dbg !4862

bb29.i1483.lr.ph:                                 ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3105
  %_126.1.i1488 = load i32, ptr %62, align 4, !alias.scope !4852, !noalias !4861, !noundef !10
  %_126.0.i1492 = load ptr, ptr %61, align 4, !nonnull !10
  %182 = add i32 %ring_cursor.sroa.0.1.i7396466, 1
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
  br label %bb29.i1483, !dbg !4862

bb29.i1483:                                       ; preds = %bb29.i1483.lr.ph, %bb28.i1545
  %iter.sroa.0.0.idx.i14816446 = phi i32 [ 0, %bb29.i1483.lr.ph ], [ %iter.sroa.0.0.add.i1486, %bb28.i1545 ]
  %iter.sroa.4.0.i14806445 = phi i32 [ 0, %bb29.i1483.lr.ph ], [ %_102.0.i1487, %bb28.i1545 ]
  %iter.sroa.7.0.i14796444 = phi i32 [ %width.i1477, %bb29.i1483.lr.ph ], [ %184, %bb28.i1545 ]
  %iter.sroa.0.0.ptr.i14826447 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 %iter.sroa.0.0.idx.i14816446, !dbg !4871
  %184 = add i32 %iter.sroa.7.0.i14796444, -1, !dbg !4871
  %_109.i1484 = icmp eq i32 %iter.sroa.0.0.idx.i14816446, 32, !dbg !4872
  br i1 %_109.i1484, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1582, label %bb33.i1485, !dbg !4881

bb33.i1485:                                       ; preds = %bb29.i1483
  %iter.sroa.0.0.add.i1486 = add nuw nsw i32 %iter.sroa.0.0.idx.i14816446, 4, !dbg !4882
  %_102.0.i1487 = add nuw nsw i32 %iter.sroa.4.0.i14806445, 1, !dbg !4885
  %exitcond11899.not = icmp eq i32 %iter.sroa.4.0.i14806445, %_126.1.i1488, !dbg !4888
  br i1 %exitcond11899.not, label %panic.i1490, label %bb2.i1491, !dbg !4888

bb2.i1491:                                        ; preds = %bb33.i1485
  %185 = getelementptr inbounds nuw %LaneShape, ptr %_126.0.i1492, i32 %iter.sroa.4.0.i14806445, !dbg !4888
  %shape.i1493 = load i32, ptr %185, align 4, !dbg !4888, !noalias !4890, !noundef !10
  %186 = getelementptr inbounds nuw i8, ptr %185, i32 4, !dbg !4888
  %shape3.i1494 = load i32, ptr %186, align 4, !dbg !4888, !noalias !4890, !noundef !10
  %187 = add i32 %shape3.i1494, %ring_cursor.sroa.0.1.i7396466, !dbg !4891
  %_18.not.i1495 = icmp ult i32 %187, %_87.i763, !dbg !4894
  %188 = select i1 %_18.not.i1495, i32 0, i32 %_87.i763, !dbg !4894
  %spec.select.i1496 = sub nuw i32 %187, %188, !dbg !4894
  %_25.i1499 = mul i32 %spec.select.i1496, %width.i1477, !dbg !4896
  %_24.i1500 = add i32 %_25.i1499, %iter.sroa.4.0.i14806445, !dbg !4896
  %_28.i1502 = icmp ult i32 %_24.i1500, %_128.1.i1501, !dbg !4898
  br i1 %_28.i1502, label %bb9.i1504, label %panic5.i1503, !dbg !4898

panic.i1490:                                      ; preds = %bb33.i1485
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_126.1.i1488, i32 noundef %_126.1.i1488, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_12856b5f9033764dbf23e04eb77c5c46) #28, !dbg !4888, !noalias !4890
  unreachable, !dbg !4888

bb9.i1504:                                        ; preds = %bb2.i1491
  %189 = getelementptr inbounds nuw float, ptr %_128.0.i1505, i32 %_24.i1500, !dbg !4898
  %190 = load float, ptr %189, align 4, !dbg !4898, !noalias !4890, !noundef !10
  %exitcond11900.not = icmp eq i32 %iter.sroa.4.0.i14806445, %_130.1.i1506, !dbg !4899
  br i1 %exitcond11900.not, label %panic6.i1508, label %bb10.i1509, !dbg !4899

panic5.i1503:                                     ; preds = %bb2.i1491
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_24.i1500, i32 noundef %_128.1.i1501, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70ebf0cf1c90c6161926611ccf249a32) #28, !dbg !4898, !noalias !4890
  unreachable, !dbg !4898

bb10.i1509:                                       ; preds = %bb9.i1504
  %191 = getelementptr inbounds nuw i32, ptr %_130.0.i1510, i32 %iter.sroa.4.0.i14806445, !dbg !4899
  %_30.i1511 = load i32, ptr %191, align 4, !dbg !4899, !noalias !4890, !noundef !10
  %192 = icmp eq i32 %_30.i1511, 0, !dbg !4901
  br i1 %192, label %bb14.i1520, label %bb12.i1512, !dbg !4901

panic6.i1508:                                     ; preds = %bb9.i1504
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_130.1.i1506, i32 noundef %_130.1.i1506, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_023d591aaad7f5cf482ac80a9401eca1) #28, !dbg !4899, !noalias !4890
  unreachable, !dbg !4899

bb12.i1512:                                       ; preds = %bb10.i1509
  %_35.i1514 = icmp ult i32 %iter.sroa.4.0.i14806445, %_132.1.i1513, !dbg !4903
  br i1 %_35.i1514, label %bb13.i1516, label %panic7.i1515, !dbg !4903

bb14.i1520:                                       ; preds = %bb34.i1581, %bb13.i1516, %bb10.i1509
  %newest.sroa.0.0.i1521 = phi float [ %190, %bb10.i1509 ], [ %_33.i1518, %bb34.i1581 ], [ %190, %bb13.i1516 ], !dbg !4904
  %exitcond11901.not = icmp eq i32 %iter.sroa.4.0.i14806445, %_132.1.i1513, !dbg !4905
  br i1 %exitcond11901.not, label %panic8.i1524, label %bb15.i1525, !dbg !4905

bb13.i1516:                                       ; preds = %bb12.i1512
  %193 = getelementptr inbounds nuw float, ptr %_132.0.i1517, i32 %iter.sroa.4.0.i14806445, !dbg !4903
  %_33.i1518 = load float, ptr %193, align 4, !dbg !4903, !noalias !4890, !noundef !10
  %_116.i1519 = fcmp olt float %_33.i1518, %190, !dbg !4907
  br i1 %_116.i1519, label %bb34.i1581, label %bb14.i1520, !dbg !4907

panic7.i1515:                                     ; preds = %bb12.i1512
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %iter.sroa.4.0.i14806445, i32 noundef %_132.1.i1513, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a228cac3279aae3a34886f59f51fbe9e) #28, !dbg !4903, !noalias !4890
  unreachable, !dbg !4903

bb34.i1581:                                       ; preds = %bb13.i1516
  br label %bb14.i1520, !dbg !4910

bb15.i1525:                                       ; preds = %bb14.i1520
  %194 = getelementptr inbounds nuw float, ptr %_132.0.i1517, i32 %iter.sroa.4.0.i14806445, !dbg !4905
  store float %newest.sroa.0.0.i1521, ptr %194, align 4, !dbg !4905, !noalias !4890
  %_40.i1527 = add i32 %_30.i1511, 1, !dbg !4911
  %complete.i1528 = icmp eq i32 %_40.i1527, %shape.i1493, !dbg !4911
  br i1 %complete.i1528, label %bb19.i1550, label %bb17.i1529, !dbg !4912

panic8.i1524:                                     ; preds = %bb14.i1520
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_132.1.i1513, i32 noundef %_132.1.i1513, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1fa5d88c1a2cc292a2767c48606a38fe) #28, !dbg !4905, !noalias !4890
  unreachable, !dbg !4905

bb17.i1529:                                       ; preds = %bb15.i1525
  %_42.i1531 = add i32 %iter.sroa.4.0.i14806445, %_43.i1530, !dbg !4914
  %_45.i1533 = icmp ult i32 %_42.i1531, %_128.1.i1501, !dbg !4915
  br i1 %_45.i1533, label %bb27.i1543, label %panic9.i1534, !dbg !4915

panic9.i1534:                                     ; preds = %bb17.i1529
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_42.i1531, i32 noundef %_128.1.i1501, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70d40ae2302f3ea19fa0c4a65fe82786) #28, !dbg !4915, !noalias !4890
  unreachable, !dbg !4915

bb27.i1543:                                       ; preds = %bb17.i1529
  %195 = getelementptr inbounds nuw float, ptr %_128.0.i1505, i32 %_42.i1531, !dbg !4915
  %_41.i1537 = load float, ptr %195, align 4, !dbg !4915, !noalias !4890, !noundef !10
  %_117.i1538 = fcmp olt float %_41.i1537, %newest.sroa.0.0.i1521, !dbg !4916
  %newest.sroa.0.1.i1539 = select i1 %_117.i1538, float %_41.i1537, float %newest.sroa.0.0.i1521, !dbg !4916
  store float %newest.sroa.0.1.i1539, ptr %iter.sroa.0.0.ptr.i14826447, align 4, !dbg !4918, !alias.scope !4856, !noalias !4919
  br label %bb28.i1545, !dbg !4920

bb28.i1545:                                       ; preds = %bb22.i1578, %bb19.i1550, %bb27.i1543
  %storemerge = phi i32 [ %_40.i1527, %bb27.i1543 ], [ 0, %bb19.i1550 ], [ 0, %bb22.i1578 ], !dbg !4921
  store i32 %storemerge, ptr %191, align 4, !dbg !4921, !noalias !4890
  %196 = icmp eq i32 %184, 0, !dbg !4862
  br i1 %196, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1582, label %bb29.i1483, !dbg !4862

bb19.i1550:                                       ; preds = %bb15.i1525
  store float %newest.sroa.0.0.i1521, ptr %iter.sroa.0.0.ptr.i14826447, align 4, !dbg !4918, !alias.scope !4856, !noalias !4919
  %_118.i15566440.not = icmp eq i32 %shape.i1493, 0, !dbg !4922
  br i1 %_118.i15566440.not, label %bb28.i1545, label %bb40.i1563.preheader, !dbg !4933

bb40.i1563.preheader:                             ; preds = %bb19.i1550
  %197 = load float, ptr %189, align 4, !dbg !4934, !noalias !4890, !noundef !10
  br label %bb40.i1563, !dbg !4935

bb40.i1563:                                       ; preds = %bb40.i1563.preheader, %bb22.i1578
  %iter2.sroa.0.0.i15556443 = phi i32 [ %_119.i1564, %bb22.i1578 ], [ 0, %bb40.i1563.preheader ]
  %suffix.sroa.0.0.i15546442 = phi float [ %suffix.sroa.0.1.i1574, %bb22.i1578 ], [ %197, %bb40.i1563.preheader ]
  %end.sroa.0.1.i15536441 = phi i32 [ %200, %bb22.i1578 ], [ %spec.select.i1496, %bb40.i1563.preheader ]
  %_54.i1565 = mul i32 %end.sroa.0.1.i15536441, %width.i1477, !dbg !4936
  %_53.i1566 = add i32 %_54.i1565, %iter.sroa.4.0.i14806445, !dbg !4936
  %_57.i1568 = icmp ult i32 %_53.i1566, %_128.1.i1501, !dbg !4935
  br i1 %_57.i1568, label %bb22.i1578, label %panic13.i1569, !dbg !4935

panic13.i1569:                                    ; preds = %bb40.i1563
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_53.i1566, i32 noundef %_128.1.i1501, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a271bbb7fd83c3605ea58a06b7065fa4) #28, !dbg !4935, !noalias !4890
  unreachable, !dbg !4935

bb22.i1578:                                       ; preds = %bb40.i1563
  %_119.i1564 = add nuw i32 %iter2.sroa.0.0.i15556443, 1, !dbg !4937
  %198 = getelementptr inbounds nuw float, ptr %_128.0.i1505, i32 %_53.i1566, !dbg !4935
  %_52.i1572 = load float, ptr %198, align 4, !dbg !4935, !noalias !4890, !noundef !10
  %_121.i1573 = fcmp olt float %suffix.sroa.0.0.i15546442, %_52.i1572, !dbg !4943
  %suffix.sroa.0.1.i1574 = select i1 %_121.i1573, float %suffix.sroa.0.0.i15546442, float %_52.i1572, !dbg !4943
  store float %suffix.sroa.0.1.i1574, ptr %198, align 4, !dbg !4945, !noalias !4890
  %199 = icmp eq i32 %end.sroa.0.1.i15536441, 0, !dbg !4946
  %spec.store.select.i1580 = select i1 %199, i32 %_87.i763, i32 %end.sroa.0.1.i15536441, !dbg !4946
  %200 = add i32 %spec.store.select.i1580, -1, !dbg !4947
  %exitcond11898.not = icmp eq i32 %_119.i1564, %shape.i1493, !dbg !4922
  br i1 %exitcond11898.not, label %bb28.i1545, label %bb40.i1563, !dbg !4933

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1582: ; preds = %bb29.i1483, %bb28.i1545, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3105
  %_0.i2940 = load float, ptr %scratch.i706, align 4, !dbg !4948, !alias.scope !4950, !noalias !4953, !noundef !10
  %_0.i2516 = fmul float %_0.i2940, 1.638400e+04, !dbg !4954
  %201 = tail call noundef float @llvm.floor.f32(float %_0.i2516), !dbg !4956
  %_0.i2515 = fmul float %201, 0x3F10000000000000, !dbg !4963
  %202 = icmp eq i32 %width.i33.i764, 0, !dbg !4965
  %_163.1.i82.i813.pre = load i32, ptr %141, align 4, !dbg !4970, !alias.scope !4821, !noalias !4822
  br i1 %202, label %bb53.i77.i808, label %bb36.i56.i787.lr.ph, !dbg !4965

bb36.i56.i787.lr.ph:                              ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1582
  %_159.1.i61.i792 = load i32, ptr %62, align 4, !alias.scope !4821, !noalias !4822, !noundef !10
  %_159.0.i65.i796 = load ptr, ptr %61, align 4, !nonnull !10
  %_161.0.i75.i806 = load ptr, ptr %142, align 4, !nonnull !10
  %exitcond11902.not = icmp eq i32 %_159.1.i61.i792, 0, !dbg !4971
  br i1 %exitcond11902.not, label %panic.i63.i794, label %bb14.i64.i795, !dbg !4971

bb34.i124.i960:                                   ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2945
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i39.i770, i32 noundef %_158.1.i38.i769, i32 noundef %_158.1.i38.i769, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_db7c42041d7aaaba4839906f37294547) #28, !dbg !4973, !noalias !4851
  unreachable, !dbg !4973

bb53.i77.i808:                                    ; preds = %bb18.i74.i805.7, %bb18.i74.i805, %bb18.i74.i805.1, %bb18.i74.i805.2, %bb18.i74.i805.3, %bb18.i74.i805.4, %bb18.i74.i805.5, %bb18.i74.i805.6, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1582
  %_0.i2938 = phi float [ %_0.i2940, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1582 ], [ %_47.i76.i807, %bb18.i74.i805 ], [ %_47.i76.i807, %bb18.i74.i805.7 ], [ %_47.i76.i807, %bb18.i74.i805.6 ], [ %_47.i76.i807, %bb18.i74.i805.5 ], [ %_47.i76.i807, %bb18.i74.i805.4 ], [ %_47.i76.i807, %bb18.i74.i805.3 ], [ %_47.i76.i807, %bb18.i74.i805.2 ], [ %_47.i76.i807, %bb18.i74.i805.1 ], !dbg !4974
  %_0.i2088 = fadd float %_0.i2515, %_0.i28387302, !dbg !4976
  %_0.i2838 = fsub float %_0.i2088, %_0.i2938, !dbg !4978
  %_123.i83.i814 = icmp ugt i32 %_22.i39.i770, %_163.1.i82.i813.pre, !dbg !4980
  br i1 %_123.i83.i814, label %bb41.i123.i959, label %bb42.i84.i815, !dbg !4980, !prof !755

bb14.i64.i795:                                    ; preds = %bb36.i56.i787.lr.ph
  %203 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i796, i32 8, !dbg !4971
  %_42.i66.i797 = load i32, ptr %203, align 4, !dbg !4971, !noalias !4953, !noundef !10
  %204 = add i32 %_42.i66.i797, %ring_cursor.sroa.0.1.i7396466, !dbg !4984
  %_45.not.i67.i798 = icmp ult i32 %204, %_87.i763, !dbg !4985
  %205 = select i1 %_45.not.i67.i798, i32 0, i32 %_87.i763, !dbg !4985
  %spec.select.i68.i799 = sub nuw i32 %204, %205, !dbg !4985
  %_49.i69.i800 = mul i32 %spec.select.i68.i799, %width.i33.i764, !dbg !4987
  %_51.i72.i803 = icmp ult i32 %_49.i69.i800, %_163.1.i82.i813.pre, !dbg !4988
  br i1 %_51.i72.i803, label %bb18.i74.i805, label %panic1.i73.i804, !dbg !4988

panic.i63.i794:                                   ; preds = %bb36.i56.i787.7, %bb36.i56.i787.6, %bb36.i56.i787.5, %bb36.i56.i787.4, %bb36.i56.i787.3, %bb36.i56.i787.2, %bb36.i56.i787.1, %bb36.i56.i787.lr.ph
  %_159.1.i61.i792.lcssa.ph = phi i32 [ 7, %bb36.i56.i787.7 ], [ 6, %bb36.i56.i787.6 ], [ 5, %bb36.i56.i787.5 ], [ 4, %bb36.i56.i787.4 ], [ 3, %bb36.i56.i787.3 ], [ 2, %bb36.i56.i787.2 ], [ 1, %bb36.i56.i787.1 ], [ 0, %bb36.i56.i787.lr.ph ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_159.1.i61.i792.lcssa.ph, i32 noundef %_159.1.i61.i792.lcssa.ph, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_33746731a929bad63709ddedbca82f5f) #28, !dbg !4971, !noalias !4953
  unreachable, !dbg !4971

bb18.i74.i805:                                    ; preds = %bb14.i64.i795
  %206 = getelementptr inbounds nuw float, ptr %_161.0.i75.i806, i32 %_49.i69.i800, !dbg !4988
  %_47.i76.i807 = load float, ptr %206, align 4, !dbg !4988, !noalias !4953, !noundef !10
  store float %_47.i76.i807, ptr %scratch.i706, align 4, !dbg !4989, !alias.scope !4818, !noalias !4990
  %207 = icmp eq i32 %width.i33.i764, 1, !dbg !4965
  br i1 %207, label %bb53.i77.i808, label %bb36.i56.i787.1, !dbg !4965

bb36.i56.i787.1:                                  ; preds = %bb18.i74.i805
  %exitcond11902.1.not = icmp eq i32 %_159.1.i61.i792, 1, !dbg !4971
  br i1 %exitcond11902.1.not, label %panic.i63.i794, label %bb14.i64.i795.1, !dbg !4971

bb14.i64.i795.1:                                  ; preds = %bb36.i56.i787.1
  %208 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i796, i32 20, !dbg !4971
  %_42.i66.i797.1 = load i32, ptr %208, align 4, !dbg !4971, !noalias !4953, !noundef !10
  %209 = add i32 %_42.i66.i797.1, %ring_cursor.sroa.0.1.i7396466, !dbg !4984
  %_45.not.i67.i798.1 = icmp ult i32 %209, %_87.i763, !dbg !4985
  %210 = select i1 %_45.not.i67.i798.1, i32 0, i32 %_87.i763, !dbg !4985
  %spec.select.i68.i799.1 = sub nuw i32 %209, %210, !dbg !4985
  %_49.i69.i800.1 = mul i32 %spec.select.i68.i799.1, %width.i33.i764, !dbg !4987
  %_48.i70.i801.1 = add i32 %_49.i69.i800.1, 1, !dbg !4987
  %_51.i72.i803.1 = icmp ult i32 %_48.i70.i801.1, %_163.1.i82.i813.pre, !dbg !4988
  br i1 %_51.i72.i803.1, label %bb18.i74.i805.1, label %panic1.i73.i804, !dbg !4988

bb18.i74.i805.1:                                  ; preds = %bb14.i64.i795.1
  %211 = getelementptr inbounds nuw float, ptr %_161.0.i75.i806, i32 %_48.i70.i801.1, !dbg !4988
  %_47.i76.i807.1 = load float, ptr %211, align 4, !dbg !4988, !noalias !4953, !noundef !10
  store float %_47.i76.i807.1, ptr %iter.sroa.0.0.ptr.i55.i7866451.1, align 4, !dbg !4989, !alias.scope !4818, !noalias !4990
  %212 = icmp eq i32 %width.i33.i764, 2, !dbg !4965
  br i1 %212, label %bb53.i77.i808, label %bb36.i56.i787.2, !dbg !4965

bb36.i56.i787.2:                                  ; preds = %bb18.i74.i805.1
  %exitcond11902.2.not = icmp eq i32 %_159.1.i61.i792, 2, !dbg !4971
  br i1 %exitcond11902.2.not, label %panic.i63.i794, label %bb14.i64.i795.2, !dbg !4971

bb14.i64.i795.2:                                  ; preds = %bb36.i56.i787.2
  %213 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i796, i32 32, !dbg !4971
  %_42.i66.i797.2 = load i32, ptr %213, align 4, !dbg !4971, !noalias !4953, !noundef !10
  %214 = add i32 %_42.i66.i797.2, %ring_cursor.sroa.0.1.i7396466, !dbg !4984
  %_45.not.i67.i798.2 = icmp ult i32 %214, %_87.i763, !dbg !4985
  %215 = select i1 %_45.not.i67.i798.2, i32 0, i32 %_87.i763, !dbg !4985
  %spec.select.i68.i799.2 = sub nuw i32 %214, %215, !dbg !4985
  %_49.i69.i800.2 = mul i32 %spec.select.i68.i799.2, %width.i33.i764, !dbg !4987
  %_48.i70.i801.2 = add i32 %_49.i69.i800.2, 2, !dbg !4987
  %_51.i72.i803.2 = icmp ult i32 %_48.i70.i801.2, %_163.1.i82.i813.pre, !dbg !4988
  br i1 %_51.i72.i803.2, label %bb18.i74.i805.2, label %panic1.i73.i804, !dbg !4988

bb18.i74.i805.2:                                  ; preds = %bb14.i64.i795.2
  %216 = getelementptr inbounds nuw float, ptr %_161.0.i75.i806, i32 %_48.i70.i801.2, !dbg !4988
  %_47.i76.i807.2 = load float, ptr %216, align 4, !dbg !4988, !noalias !4953, !noundef !10
  store float %_47.i76.i807.2, ptr %iter.sroa.0.0.ptr.i55.i7866451.2, align 4, !dbg !4989, !alias.scope !4818, !noalias !4990
  %217 = icmp eq i32 %width.i33.i764, 3, !dbg !4965
  br i1 %217, label %bb53.i77.i808, label %bb36.i56.i787.3, !dbg !4965

bb36.i56.i787.3:                                  ; preds = %bb18.i74.i805.2
  %exitcond11902.3.not = icmp eq i32 %_159.1.i61.i792, 3, !dbg !4971
  br i1 %exitcond11902.3.not, label %panic.i63.i794, label %bb14.i64.i795.3, !dbg !4971

bb14.i64.i795.3:                                  ; preds = %bb36.i56.i787.3
  %218 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i796, i32 44, !dbg !4971
  %_42.i66.i797.3 = load i32, ptr %218, align 4, !dbg !4971, !noalias !4953, !noundef !10
  %219 = add i32 %_42.i66.i797.3, %ring_cursor.sroa.0.1.i7396466, !dbg !4984
  %_45.not.i67.i798.3 = icmp ult i32 %219, %_87.i763, !dbg !4985
  %220 = select i1 %_45.not.i67.i798.3, i32 0, i32 %_87.i763, !dbg !4985
  %spec.select.i68.i799.3 = sub nuw i32 %219, %220, !dbg !4985
  %_49.i69.i800.3 = mul i32 %spec.select.i68.i799.3, %width.i33.i764, !dbg !4987
  %_48.i70.i801.3 = add i32 %_49.i69.i800.3, 3, !dbg !4987
  %_51.i72.i803.3 = icmp ult i32 %_48.i70.i801.3, %_163.1.i82.i813.pre, !dbg !4988
  br i1 %_51.i72.i803.3, label %bb18.i74.i805.3, label %panic1.i73.i804, !dbg !4988

bb18.i74.i805.3:                                  ; preds = %bb14.i64.i795.3
  %221 = getelementptr inbounds nuw float, ptr %_161.0.i75.i806, i32 %_48.i70.i801.3, !dbg !4988
  %_47.i76.i807.3 = load float, ptr %221, align 4, !dbg !4988, !noalias !4953, !noundef !10
  store float %_47.i76.i807.3, ptr %iter.sroa.0.0.ptr.i55.i7866451.3, align 4, !dbg !4989, !alias.scope !4818, !noalias !4990
  %222 = icmp eq i32 %width.i33.i764, 4, !dbg !4965
  br i1 %222, label %bb53.i77.i808, label %bb36.i56.i787.4, !dbg !4965

bb36.i56.i787.4:                                  ; preds = %bb18.i74.i805.3
  %exitcond11902.4.not = icmp eq i32 %_159.1.i61.i792, 4, !dbg !4971
  br i1 %exitcond11902.4.not, label %panic.i63.i794, label %bb14.i64.i795.4, !dbg !4971

bb14.i64.i795.4:                                  ; preds = %bb36.i56.i787.4
  %223 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i796, i32 56, !dbg !4971
  %_42.i66.i797.4 = load i32, ptr %223, align 4, !dbg !4971, !noalias !4953, !noundef !10
  %224 = add i32 %_42.i66.i797.4, %ring_cursor.sroa.0.1.i7396466, !dbg !4984
  %_45.not.i67.i798.4 = icmp ult i32 %224, %_87.i763, !dbg !4985
  %225 = select i1 %_45.not.i67.i798.4, i32 0, i32 %_87.i763, !dbg !4985
  %spec.select.i68.i799.4 = sub nuw i32 %224, %225, !dbg !4985
  %_49.i69.i800.4 = mul i32 %spec.select.i68.i799.4, %width.i33.i764, !dbg !4987
  %_48.i70.i801.4 = add i32 %_49.i69.i800.4, 4, !dbg !4987
  %_51.i72.i803.4 = icmp ult i32 %_48.i70.i801.4, %_163.1.i82.i813.pre, !dbg !4988
  br i1 %_51.i72.i803.4, label %bb18.i74.i805.4, label %panic1.i73.i804, !dbg !4988

bb18.i74.i805.4:                                  ; preds = %bb14.i64.i795.4
  %226 = getelementptr inbounds nuw float, ptr %_161.0.i75.i806, i32 %_48.i70.i801.4, !dbg !4988
  %_47.i76.i807.4 = load float, ptr %226, align 4, !dbg !4988, !noalias !4953, !noundef !10
  store float %_47.i76.i807.4, ptr %iter.sroa.0.0.ptr.i55.i7866451.4, align 4, !dbg !4989, !alias.scope !4818, !noalias !4990
  %227 = icmp eq i32 %width.i33.i764, 5, !dbg !4965
  br i1 %227, label %bb53.i77.i808, label %bb36.i56.i787.5, !dbg !4965

bb36.i56.i787.5:                                  ; preds = %bb18.i74.i805.4
  %exitcond11902.5.not = icmp eq i32 %_159.1.i61.i792, 5, !dbg !4971
  br i1 %exitcond11902.5.not, label %panic.i63.i794, label %bb14.i64.i795.5, !dbg !4971

bb14.i64.i795.5:                                  ; preds = %bb36.i56.i787.5
  %228 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i796, i32 68, !dbg !4971
  %_42.i66.i797.5 = load i32, ptr %228, align 4, !dbg !4971, !noalias !4953, !noundef !10
  %229 = add i32 %_42.i66.i797.5, %ring_cursor.sroa.0.1.i7396466, !dbg !4984
  %_45.not.i67.i798.5 = icmp ult i32 %229, %_87.i763, !dbg !4985
  %230 = select i1 %_45.not.i67.i798.5, i32 0, i32 %_87.i763, !dbg !4985
  %spec.select.i68.i799.5 = sub nuw i32 %229, %230, !dbg !4985
  %_49.i69.i800.5 = mul i32 %spec.select.i68.i799.5, %width.i33.i764, !dbg !4987
  %_48.i70.i801.5 = add i32 %_49.i69.i800.5, 5, !dbg !4987
  %_51.i72.i803.5 = icmp ult i32 %_48.i70.i801.5, %_163.1.i82.i813.pre, !dbg !4988
  br i1 %_51.i72.i803.5, label %bb18.i74.i805.5, label %panic1.i73.i804, !dbg !4988

bb18.i74.i805.5:                                  ; preds = %bb14.i64.i795.5
  %231 = getelementptr inbounds nuw float, ptr %_161.0.i75.i806, i32 %_48.i70.i801.5, !dbg !4988
  %_47.i76.i807.5 = load float, ptr %231, align 4, !dbg !4988, !noalias !4953, !noundef !10
  store float %_47.i76.i807.5, ptr %iter.sroa.0.0.ptr.i55.i7866451.5, align 4, !dbg !4989, !alias.scope !4818, !noalias !4990
  %232 = icmp eq i32 %width.i33.i764, 6, !dbg !4965
  br i1 %232, label %bb53.i77.i808, label %bb36.i56.i787.6, !dbg !4965

bb36.i56.i787.6:                                  ; preds = %bb18.i74.i805.5
  %exitcond11902.6.not = icmp eq i32 %_159.1.i61.i792, 6, !dbg !4971
  br i1 %exitcond11902.6.not, label %panic.i63.i794, label %bb14.i64.i795.6, !dbg !4971

bb14.i64.i795.6:                                  ; preds = %bb36.i56.i787.6
  %233 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i796, i32 80, !dbg !4971
  %_42.i66.i797.6 = load i32, ptr %233, align 4, !dbg !4971, !noalias !4953, !noundef !10
  %234 = add i32 %_42.i66.i797.6, %ring_cursor.sroa.0.1.i7396466, !dbg !4984
  %_45.not.i67.i798.6 = icmp ult i32 %234, %_87.i763, !dbg !4985
  %235 = select i1 %_45.not.i67.i798.6, i32 0, i32 %_87.i763, !dbg !4985
  %spec.select.i68.i799.6 = sub nuw i32 %234, %235, !dbg !4985
  %_49.i69.i800.6 = mul i32 %spec.select.i68.i799.6, %width.i33.i764, !dbg !4987
  %_48.i70.i801.6 = add i32 %_49.i69.i800.6, 6, !dbg !4987
  %_51.i72.i803.6 = icmp ult i32 %_48.i70.i801.6, %_163.1.i82.i813.pre, !dbg !4988
  br i1 %_51.i72.i803.6, label %bb18.i74.i805.6, label %panic1.i73.i804, !dbg !4988

bb18.i74.i805.6:                                  ; preds = %bb14.i64.i795.6
  %236 = getelementptr inbounds nuw float, ptr %_161.0.i75.i806, i32 %_48.i70.i801.6, !dbg !4988
  %_47.i76.i807.6 = load float, ptr %236, align 4, !dbg !4988, !noalias !4953, !noundef !10
  store float %_47.i76.i807.6, ptr %iter.sroa.0.0.ptr.i55.i7866451.6, align 4, !dbg !4989, !alias.scope !4818, !noalias !4990
  %237 = icmp eq i32 %width.i33.i764, 7, !dbg !4965
  br i1 %237, label %bb53.i77.i808, label %bb36.i56.i787.7, !dbg !4965

bb36.i56.i787.7:                                  ; preds = %bb18.i74.i805.6
  %exitcond11902.7.not = icmp eq i32 %_159.1.i61.i792, 7, !dbg !4971
  br i1 %exitcond11902.7.not, label %panic.i63.i794, label %bb14.i64.i795.7, !dbg !4971

bb14.i64.i795.7:                                  ; preds = %bb36.i56.i787.7
  %238 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i796, i32 92, !dbg !4971
  %_42.i66.i797.7 = load i32, ptr %238, align 4, !dbg !4971, !noalias !4953, !noundef !10
  %239 = add i32 %_42.i66.i797.7, %ring_cursor.sroa.0.1.i7396466, !dbg !4984
  %_45.not.i67.i798.7 = icmp ult i32 %239, %_87.i763, !dbg !4985
  %240 = select i1 %_45.not.i67.i798.7, i32 0, i32 %_87.i763, !dbg !4985
  %spec.select.i68.i799.7 = sub nuw i32 %239, %240, !dbg !4985
  %_49.i69.i800.7 = mul i32 %spec.select.i68.i799.7, %width.i33.i764, !dbg !4987
  %_48.i70.i801.7 = add i32 %_49.i69.i800.7, 7, !dbg !4987
  %_51.i72.i803.7 = icmp ult i32 %_48.i70.i801.7, %_163.1.i82.i813.pre, !dbg !4988
  br i1 %_51.i72.i803.7, label %bb18.i74.i805.7, label %panic1.i73.i804, !dbg !4988

bb18.i74.i805.7:                                  ; preds = %bb14.i64.i795.7
  %241 = getelementptr inbounds nuw float, ptr %_161.0.i75.i806, i32 %_48.i70.i801.7, !dbg !4988
  %_47.i76.i807.7 = load float, ptr %241, align 4, !dbg !4988, !noalias !4953, !noundef !10
  store float %_47.i76.i807.7, ptr %iter.sroa.0.0.ptr.i55.i7866451.7, align 4, !dbg !4989, !alias.scope !4818, !noalias !4990
  br label %bb53.i77.i808, !dbg !4965

panic1.i73.i804:                                  ; preds = %bb14.i64.i795.7, %bb14.i64.i795.6, %bb14.i64.i795.5, %bb14.i64.i795.4, %bb14.i64.i795.3, %bb14.i64.i795.2, %bb14.i64.i795.1, %bb14.i64.i795
  %_48.i70.i801.lcssa.ph = phi i32 [ %_48.i70.i801.7, %bb14.i64.i795.7 ], [ %_48.i70.i801.6, %bb14.i64.i795.6 ], [ %_48.i70.i801.5, %bb14.i64.i795.5 ], [ %_48.i70.i801.4, %bb14.i64.i795.4 ], [ %_48.i70.i801.3, %bb14.i64.i795.3 ], [ %_48.i70.i801.2, %bb14.i64.i795.2 ], [ %_48.i70.i801.1, %bb14.i64.i795.1 ], [ %_49.i69.i800, %bb14.i64.i795 ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_48.i70.i801.lcssa.ph, i32 noundef %_163.1.i82.i813.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_0e76a26fbb751dfb8cf8a069b5b0d39a) #28, !dbg !4988, !noalias !4953
  unreachable, !dbg !4988

bb42.i84.i815:                                    ; preds = %bb53.i77.i808
  %_163.0.i85.i816 = load ptr, ptr %142, align 4, !dbg !4970, !alias.scope !4821, !noalias !4822, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4991), !dbg !4994
  %_4.not.i3098 = icmp eq i32 %_163.1.i82.i813.pre, %_22.i39.i770, !dbg !4995
  br i1 %_4.not.i3098, label %panic.i3100, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3101, !dbg !4995

panic.i3100:                                      ; preds = %bb42.i84.i815
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #28, !dbg !4995, !noalias !4997
  unreachable, !dbg !4995

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3101: ; preds = %bb42.i84.i815
  %_130.i87.i818 = getelementptr inbounds nuw float, ptr %_163.0.i85.i816, i32 %_22.i39.i770, !dbg !4998
  store float %_0.i2515, ptr %_130.i87.i818, align 4, !dbg !4995, !alias.scope !4991, !noalias !4953
  %_0.i2385 = fdiv float %_0.i2838, %_62.i89.i820, !dbg !5003
  %_0.i2837 = fsub float 1.000000e+00, %_0.i2385, !dbg !5005
  %_0.i2836 = fsub float %_0.i2837, %_0.i32127371, !dbg !5007
  %_4.i2401 = fmul float %_0.i3312, %_0.i2836, !dbg !5009
  %_0.i2402 = fadd float %_0.i32127371, %_4.i2401, !dbg !5009
  %_3.i.i3575.inv = fcmp ogt float %_0.i2837, %_0.i2402, !dbg !5012
  %_4.i.i3582.v = select i1 %_3.i.i3575.inv, float %_0.i2837, float %_0.i2402, !dbg !5012
  %242 = tail call noundef float @llvm.fabs.f32(float %_4.i.i3582.v), !dbg !5016
  %243 = fcmp uge float %242, 0x3BC79CA100000000, !dbg !5020
  %_0.i3212 = select i1 %243, float %_4.i.i3582.v, float 0.000000e+00, !dbg !5023
  %_0.i2835 = fsub float 1.000000e+00, %_0.i3212, !dbg !5024
  %_164.1.i101.i832 = load i32, ptr %146, align 4, !dbg !5026, !alias.scope !4821, !noalias !4822, !noundef !10
  %_74.i102.i833 = mul i32 %width.i33.i764, %main_cursor.sroa.0.1.i7406467, !dbg !5028
  %_134.i103.i834 = icmp ugt i32 %_74.i102.i833, %_164.1.i101.i832, !dbg !5029
  br i1 %_134.i103.i834, label %bb47.i122.i958, label %bb48.i104.i835, !dbg !5029, !prof !755

bb41.i123.i959:                                   ; preds = %bb53.i77.i808
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i39.i770, i32 noundef %_163.1.i82.i813.pre, i32 noundef %_163.1.i82.i813.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c2286ff41a33b8c11aa270fe215441de) #28, !dbg !5034, !noalias !4953
  unreachable, !dbg !5034

bb48.i104.i835:                                   ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3101
  %_164.0.i105.i836 = load ptr, ptr %147, align 4, !dbg !5026, !alias.scope !4821, !noalias !4822, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5035), !dbg !5038
  %_3.not.i2932 = icmp eq i32 %_164.1.i101.i832, %_74.i102.i833, !dbg !5039
  br i1 %_3.not.i2932, label %panic.i2935, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3093, !dbg !5039

panic.i2935:                                      ; preds = %bb48.i104.i835
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #28, !dbg !5039, !noalias !5041
  unreachable, !dbg !5039

bb47.i122.i958:                                   ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3101
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_74.i102.i833, i32 noundef %_164.1.i101.i832, i32 noundef %_164.1.i101.i832, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a3a21efd18b403ec25db545d522c479) #28, !dbg !5042, !noalias !4953
  unreachable, !dbg !5042

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3093: ; preds = %bb48.i104.i835
  %_141.i107.i838 = getelementptr inbounds nuw float, ptr %_164.0.i105.i836, i32 %_74.i102.i833, !dbg !5043
  %_0.i2934 = load float, ptr %_141.i107.i838, align 4, !dbg !5039, !alias.scope !5035, !noalias !4953, !noundef !10
  store float %_0.i2943, ptr %_141.i107.i838, align 4, !dbg !5048, !alias.scope !5051, !noalias !4953
  %_0.i2514 = fmul float %_0.i2835, %_0.i2934, !dbg !5054
  %_6.i3351 = bitcast float %_0.i2934 to i32, !dbg !5056
  %_5.i3352 = and i32 %_6.i3351, %all.sroa.0.0.i715, !dbg !5059
  %_8.i3353 = bitcast float %_0.i2514 to i32, !dbg !5060
  %_7.i3355 = and i32 %_9.i3354, %_8.i3353, !dbg !5062
  %_4.i3356 = or disjoint i32 %_7.i3355, %_5.i3352, !dbg !5059
  store i32 %_4.i3356, ptr %_139.i761, align 4, !dbg !5063, !alias.scope !5065, !noalias !5068
  %_140.i852 = icmp ugt i32 %_60.i743, %right_io.1, !dbg !5069
  br i1 %_140.i852, label %bb48.i955, label %bb49.i853, !dbg !5069, !prof !755

bb46.i961:                                        ; preds = %bb42.i742
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_60.i743, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_f462f4e49ffb40af04504eaf795aa606) #28, !dbg !5073, !noalias !4072
  unreachable, !dbg !5073

bb49.i853:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3093
  %_147.i855 = getelementptr inbounds nuw float, ptr %right_io.0, i32 %_60.i743, !dbg !5074
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5079), !dbg !5082
  %_3.not.i2927 = icmp eq i32 %right_io.1, %_60.i743, !dbg !5083
  br i1 %_3.not.i2927, label %panic.i2930, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2931, !dbg !5083

panic.i2930:                                      ; preds = %bb49.i853
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #28, !dbg !5083, !noalias !5085
  unreachable, !dbg !5083

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2931: ; preds = %bb49.i853
  %_0.i2929 = load float, ptr %_147.i855, align 4, !dbg !5083, !alias.scope !5079, !noalias !4072, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5086), !dbg !5089
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5090), !dbg !5089
  %width.i.i857 = load i32, ptr %148, align 4, !dbg !5092, !alias.scope !5093, !noalias !5094, !noundef !10
  %_3.i1951 = fcmp uge float %_0.i3325, %_0.i3370, !dbg !5097
  %_0.i2384 = fdiv float %_0.i3325, %_0.i3370, !dbg !5099
  %_0.i3350 = select i1 %_3.i1951, float 1.000000e+00, float %_0.i2384, !dbg !5101
  %_158.1.i.i862 = load i32, ptr %149, align 4, !dbg !5103, !alias.scope !5093, !noalias !5094, !noundef !10
  %_22.i.i863 = mul i32 %width.i.i857, %ring_cursor.sroa.0.1.i7396466, !dbg !5104
  %_90.i.i864 = icmp ugt i32 %_22.i.i863, %_158.1.i.i862, !dbg !5105
  br i1 %_90.i.i864, label %bb34.i.i954, label %bb35.i.i865, !dbg !5105, !prof !755

bb35.i.i865:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2931
  %_158.0.i.i866 = load ptr, ptr %150, align 4, !dbg !5103, !alias.scope !5093, !noalias !5094, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5108), !dbg !5111
  %_4.not.i3086 = icmp eq i32 %_158.1.i.i862, %_22.i.i863, !dbg !5112
  br i1 %_4.not.i3086, label %panic.i3088, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3089, !dbg !5112

panic.i3088:                                      ; preds = %bb35.i.i865
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #28, !dbg !5112, !noalias !5114
  unreachable, !dbg !5112

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3089: ; preds = %bb35.i.i865
  %_97.i.i868 = getelementptr inbounds nuw float, ptr %_158.0.i.i866, i32 %_22.i.i863, !dbg !5115
  store float %_0.i3350, ptr %_97.i.i868, align 4, !dbg !5112, !alias.scope !5108, !noalias !5117
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5118), !dbg !5121
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5122), !dbg !5121
  %width.i = load i32, ptr %148, align 4, !dbg !5124, !alias.scope !5118, !noalias !5126, !noundef !10
  %244 = icmp eq i32 %width.i, 0, !dbg !5127
  br i1 %244, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit, label %bb29.i.lr.ph, !dbg !5127

bb29.i.lr.ph:                                     ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3089
  %_126.1.i = load i32, ptr %151, align 4, !alias.scope !5118, !noalias !5126, !noundef !10
  %_126.0.i = load ptr, ptr %152, align 4, !nonnull !10
  %245 = add i32 %ring_cursor.sroa.0.1.i7396466, 1
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
  br label %bb29.i, !dbg !5127

bb29.i:                                           ; preds = %bb29.i.lr.ph, %bb28.i
  %iter.sroa.0.0.idx.i6458 = phi i32 [ 0, %bb29.i.lr.ph ], [ %iter.sroa.0.0.add.i, %bb28.i ]
  %iter.sroa.4.0.i6457 = phi i32 [ 0, %bb29.i.lr.ph ], [ %_102.0.i, %bb28.i ]
  %iter.sroa.7.0.i6456 = phi i32 [ %width.i, %bb29.i.lr.ph ], [ %247, %bb28.i ]
  %iter.sroa.0.0.ptr.i6459 = getelementptr inbounds nuw i8, ptr %scratch.i706, i32 %iter.sroa.0.0.idx.i6458, !dbg !5129
  %247 = add i32 %iter.sroa.7.0.i6456, -1, !dbg !5129
  %_109.i = icmp eq i32 %iter.sroa.0.0.idx.i6458, 32, !dbg !5130
  br i1 %_109.i, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit, label %bb33.i, !dbg !5134

bb33.i:                                           ; preds = %bb29.i
  %iter.sroa.0.0.add.i = add nuw nsw i32 %iter.sroa.0.0.idx.i6458, 4, !dbg !5135
  %_102.0.i = add nuw nsw i32 %iter.sroa.4.0.i6457, 1, !dbg !5137
  %exitcond11904.not = icmp eq i32 %iter.sroa.4.0.i6457, %_126.1.i, !dbg !5138
  br i1 %exitcond11904.not, label %panic.i, label %bb2.i1457, !dbg !5138

bb2.i1457:                                        ; preds = %bb33.i
  %248 = getelementptr inbounds nuw %LaneShape, ptr %_126.0.i, i32 %iter.sroa.4.0.i6457, !dbg !5138
  %shape.i = load i32, ptr %248, align 4, !dbg !5138, !noalias !5139, !noundef !10
  %249 = getelementptr inbounds nuw i8, ptr %248, i32 4, !dbg !5138
  %shape3.i = load i32, ptr %249, align 4, !dbg !5138, !noalias !5139, !noundef !10
  %250 = add i32 %shape3.i, %ring_cursor.sroa.0.1.i7396466, !dbg !5140
  %_18.not.i = icmp ult i32 %250, %_87.i763, !dbg !5141
  %251 = select i1 %_18.not.i, i32 0, i32 %_87.i763, !dbg !5141
  %spec.select.i = sub nuw i32 %250, %251, !dbg !5141
  %_25.i = mul i32 %spec.select.i, %width.i, !dbg !5142
  %_24.i = add i32 %_25.i, %iter.sroa.4.0.i6457, !dbg !5142
  %_28.i1458 = icmp ult i32 %_24.i, %_128.1.i, !dbg !5143
  br i1 %_28.i1458, label %bb9.i, label %panic5.i, !dbg !5143

panic.i:                                          ; preds = %bb33.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_126.1.i, i32 noundef %_126.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_12856b5f9033764dbf23e04eb77c5c46) #28, !dbg !5138, !noalias !5139
  unreachable, !dbg !5138

bb9.i:                                            ; preds = %bb2.i1457
  %252 = getelementptr inbounds nuw float, ptr %_128.0.i, i32 %_24.i, !dbg !5143
  %253 = load float, ptr %252, align 4, !dbg !5143, !noalias !5139, !noundef !10
  %exitcond11905.not = icmp eq i32 %iter.sroa.4.0.i6457, %_130.1.i, !dbg !5144
  br i1 %exitcond11905.not, label %panic6.i, label %bb10.i1460, !dbg !5144

panic5.i:                                         ; preds = %bb2.i1457
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_24.i, i32 noundef %_128.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70ebf0cf1c90c6161926611ccf249a32) #28, !dbg !5143, !noalias !5139
  unreachable, !dbg !5143

bb10.i1460:                                       ; preds = %bb9.i
  %254 = getelementptr inbounds nuw i32, ptr %_130.0.i, i32 %iter.sroa.4.0.i6457, !dbg !5144
  %_30.i1461 = load i32, ptr %254, align 4, !dbg !5144, !noalias !5139, !noundef !10
  %255 = icmp eq i32 %_30.i1461, 0, !dbg !5145
  br i1 %255, label %bb14.i, label %bb12.i1462, !dbg !5145

panic6.i:                                         ; preds = %bb9.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_130.1.i, i32 noundef %_130.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_023d591aaad7f5cf482ac80a9401eca1) #28, !dbg !5144, !noalias !5139
  unreachable, !dbg !5144

bb12.i1462:                                       ; preds = %bb10.i1460
  %_35.i1463 = icmp ult i32 %iter.sroa.4.0.i6457, %_132.1.i, !dbg !5146
  br i1 %_35.i1463, label %bb13.i1464, label %panic7.i, !dbg !5146

bb14.i:                                           ; preds = %bb34.i, %bb13.i1464, %bb10.i1460
  %newest.sroa.0.0.i = phi float [ %253, %bb10.i1460 ], [ %_33.i1465, %bb34.i ], [ %253, %bb13.i1464 ], !dbg !5147
  %exitcond11906.not = icmp eq i32 %iter.sroa.4.0.i6457, %_132.1.i, !dbg !5148
  br i1 %exitcond11906.not, label %panic8.i, label %bb15.i1467, !dbg !5148

bb13.i1464:                                       ; preds = %bb12.i1462
  %256 = getelementptr inbounds nuw float, ptr %_132.0.i, i32 %iter.sroa.4.0.i6457, !dbg !5146
  %_33.i1465 = load float, ptr %256, align 4, !dbg !5146, !noalias !5139, !noundef !10
  %_116.i1466 = fcmp olt float %_33.i1465, %253, !dbg !5149
  br i1 %_116.i1466, label %bb34.i, label %bb14.i, !dbg !5149

panic7.i:                                         ; preds = %bb12.i1462
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %iter.sroa.4.0.i6457, i32 noundef %_132.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a228cac3279aae3a34886f59f51fbe9e) #28, !dbg !5146, !noalias !5139
  unreachable, !dbg !5146

bb34.i:                                           ; preds = %bb13.i1464
  br label %bb14.i, !dbg !5151

bb15.i1467:                                       ; preds = %bb14.i
  %257 = getelementptr inbounds nuw float, ptr %_132.0.i, i32 %iter.sroa.4.0.i6457, !dbg !5148
  store float %newest.sroa.0.0.i, ptr %257, align 4, !dbg !5148, !noalias !5139
  %_40.i = add i32 %_30.i1461, 1, !dbg !5152
  %complete.i1468 = icmp eq i32 %_40.i, %shape.i, !dbg !5152
  br i1 %complete.i1468, label %bb19.i1471, label %bb17.i, !dbg !5153

panic8.i:                                         ; preds = %bb14.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_132.1.i, i32 noundef %_132.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1fa5d88c1a2cc292a2767c48606a38fe) #28, !dbg !5148, !noalias !5139
  unreachable, !dbg !5148

bb17.i:                                           ; preds = %bb15.i1467
  %_42.i = add i32 %iter.sroa.4.0.i6457, %_43.i, !dbg !5154
  %_45.i = icmp ult i32 %_42.i, %_128.1.i, !dbg !5155
  br i1 %_45.i, label %bb27.i, label %panic9.i, !dbg !5155

panic9.i:                                         ; preds = %bb17.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_42.i, i32 noundef %_128.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70d40ae2302f3ea19fa0c4a65fe82786) #28, !dbg !5155, !noalias !5139
  unreachable, !dbg !5155

bb27.i:                                           ; preds = %bb17.i
  %258 = getelementptr inbounds nuw float, ptr %_128.0.i, i32 %_42.i, !dbg !5155
  %_41.i = load float, ptr %258, align 4, !dbg !5155, !noalias !5139, !noundef !10
  %_117.i = fcmp olt float %_41.i, %newest.sroa.0.0.i, !dbg !5156
  %newest.sroa.0.1.i = select i1 %_117.i, float %_41.i, float %newest.sroa.0.0.i, !dbg !5156
  store float %newest.sroa.0.1.i, ptr %iter.sroa.0.0.ptr.i6459, align 4, !dbg !5158, !alias.scope !5122, !noalias !5159
  br label %bb28.i, !dbg !5160

bb28.i:                                           ; preds = %bb22.i, %bb19.i1471, %bb27.i
  %storemerge4516 = phi i32 [ %_40.i, %bb27.i ], [ 0, %bb19.i1471 ], [ 0, %bb22.i ], !dbg !5161
  store i32 %storemerge4516, ptr %254, align 4, !dbg !5161, !noalias !5139
  %259 = icmp eq i32 %247, 0, !dbg !5127
  br i1 %259, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit, label %bb29.i, !dbg !5127

bb19.i1471:                                       ; preds = %bb15.i1467
  store float %newest.sroa.0.0.i, ptr %iter.sroa.0.0.ptr.i6459, align 4, !dbg !5158, !alias.scope !5122, !noalias !5159
  %_118.i6452.not = icmp eq i32 %shape.i, 0, !dbg !5162
  br i1 %_118.i6452.not, label %bb28.i, label %bb40.i.preheader, !dbg !5166

bb40.i.preheader:                                 ; preds = %bb19.i1471
  %260 = load float, ptr %252, align 4, !dbg !5167, !noalias !5139, !noundef !10
  br label %bb40.i, !dbg !5168

bb40.i:                                           ; preds = %bb40.i.preheader, %bb22.i
  %iter2.sroa.0.0.i14746455 = phi i32 [ %_119.i, %bb22.i ], [ 0, %bb40.i.preheader ]
  %suffix.sroa.0.0.i14736454 = phi float [ %suffix.sroa.0.1.i, %bb22.i ], [ %260, %bb40.i.preheader ]
  %end.sroa.0.1.i6453 = phi i32 [ %263, %bb22.i ], [ %spec.select.i, %bb40.i.preheader ]
  %_54.i = mul i32 %end.sroa.0.1.i6453, %width.i, !dbg !5169
  %_53.i = add i32 %_54.i, %iter.sroa.4.0.i6457, !dbg !5169
  %_57.i = icmp ult i32 %_53.i, %_128.1.i, !dbg !5168
  br i1 %_57.i, label %bb22.i, label %panic13.i, !dbg !5168

panic13.i:                                        ; preds = %bb40.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_53.i, i32 noundef %_128.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a271bbb7fd83c3605ea58a06b7065fa4) #28, !dbg !5168, !noalias !5139
  unreachable, !dbg !5168

bb22.i:                                           ; preds = %bb40.i
  %_119.i = add nuw i32 %iter2.sroa.0.0.i14746455, 1, !dbg !5170
  %261 = getelementptr inbounds nuw float, ptr %_128.0.i, i32 %_53.i, !dbg !5168
  %_52.i = load float, ptr %261, align 4, !dbg !5168, !noalias !5139, !noundef !10
  %_121.i = fcmp olt float %suffix.sroa.0.0.i14736454, %_52.i, !dbg !5173
  %suffix.sroa.0.1.i = select i1 %_121.i, float %suffix.sroa.0.0.i14736454, float %_52.i, !dbg !5173
  store float %suffix.sroa.0.1.i, ptr %261, align 4, !dbg !5175, !noalias !5139
  %262 = icmp eq i32 %end.sroa.0.1.i6453, 0, !dbg !5176
  %spec.store.select.i1476 = select i1 %262, i32 %_87.i763, i32 %end.sroa.0.1.i6453, !dbg !5176
  %263 = add i32 %spec.store.select.i1476, -1, !dbg !5177
  %exitcond11903.not = icmp eq i32 %_119.i, %shape.i, !dbg !5162
  br i1 %exitcond11903.not, label %bb28.i, label %bb40.i, !dbg !5166

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit: ; preds = %bb28.i, %bb29.i
  %_0.i2926.pre = load float, ptr %scratch.i706, align 4, !dbg !5178, !alias.scope !5180, !noalias !5183
  br label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit, !dbg !5178

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit: ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3089
  %_0.i2926 = phi float [ %_0.i2926.pre, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit.loopexit ], [ %_0.i2938, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3089 ], !dbg !5178
  %_0.i2513 = fmul float %_0.i2926, 1.638400e+04, !dbg !5184
  %264 = tail call noundef float @llvm.floor.f32(float %_0.i2513), !dbg !5186
  %_0.i2512 = fmul float %264, 0x3F10000000000000, !dbg !5190
  %265 = icmp eq i32 %width.i.i857, 0, !dbg !5192
  %_163.1.i.i906.pre = load i32, ptr %157, align 4, !dbg !5194, !alias.scope !5093, !noalias !5094
  br i1 %265, label %bb53.i.i901, label %bb36.i.i880.lr.ph, !dbg !5192

bb36.i.i880.lr.ph:                                ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit
  %_159.1.i.i885 = load i32, ptr %151, align 4, !alias.scope !5093, !noalias !5094, !noundef !10
  %_159.0.i.i889 = load ptr, ptr %152, align 4, !nonnull !10
  %_161.0.i.i899 = load ptr, ptr %158, align 4, !nonnull !10
  %exitcond11907.not = icmp eq i32 %_159.1.i.i885, 0, !dbg !5195
  br i1 %exitcond11907.not, label %panic.i.i887, label %bb14.i.i888, !dbg !5195

bb34.i.i954:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2931
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i863, i32 noundef %_158.1.i.i862, i32 noundef %_158.1.i.i862, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_db7c42041d7aaaba4839906f37294547) #28, !dbg !5196, !noalias !5117
  unreachable, !dbg !5196

bb53.i.i901:                                      ; preds = %bb18.i.i898.7, %bb18.i.i898, %bb18.i.i898.1, %bb18.i.i898.2, %bb18.i.i898.3, %bb18.i.i898.4, %bb18.i.i898.5, %bb18.i.i898.6, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit
  %_0.i2924 = phi float [ %_0.i2926, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit ], [ %_47.i.i900, %bb18.i.i898 ], [ %_47.i.i900, %bb18.i.i898.7 ], [ %_47.i.i900, %bb18.i.i898.6 ], [ %_47.i.i900, %bb18.i.i898.5 ], [ %_47.i.i900, %bb18.i.i898.4 ], [ %_47.i.i900, %bb18.i.i898.3 ], [ %_47.i.i900, %bb18.i.i898.2 ], [ %_47.i.i900, %bb18.i.i898.1 ], !dbg !5197
  %_0.i2087 = fadd float %_0.i2512, %_0.i28347440, !dbg !5199
  %_0.i2834 = fsub float %_0.i2087, %_0.i2924, !dbg !5201
  %_123.i.i907 = icmp ugt i32 %_22.i.i863, %_163.1.i.i906.pre, !dbg !5203
  br i1 %_123.i.i907, label %bb41.i.i953, label %bb42.i.i908, !dbg !5203, !prof !755

bb14.i.i888:                                      ; preds = %bb36.i.i880.lr.ph
  %266 = getelementptr inbounds nuw i8, ptr %_159.0.i.i889, i32 8, !dbg !5195
  %_42.i.i890 = load i32, ptr %266, align 4, !dbg !5195, !noalias !5183, !noundef !10
  %267 = add i32 %_42.i.i890, %ring_cursor.sroa.0.1.i7396466, !dbg !5206
  %_45.not.i.i891 = icmp ult i32 %267, %_87.i763, !dbg !5207
  %268 = select i1 %_45.not.i.i891, i32 0, i32 %_87.i763, !dbg !5207
  %spec.select.i.i892 = sub nuw i32 %267, %268, !dbg !5207
  %_49.i.i893 = mul i32 %spec.select.i.i892, %width.i.i857, !dbg !5208
  %_51.i.i896 = icmp ult i32 %_49.i.i893, %_163.1.i.i906.pre, !dbg !5209
  br i1 %_51.i.i896, label %bb18.i.i898, label %panic1.i.i897, !dbg !5209

panic.i.i887:                                     ; preds = %bb36.i.i880.7, %bb36.i.i880.6, %bb36.i.i880.5, %bb36.i.i880.4, %bb36.i.i880.3, %bb36.i.i880.2, %bb36.i.i880.1, %bb36.i.i880.lr.ph
  %_159.1.i.i885.lcssa.ph = phi i32 [ 7, %bb36.i.i880.7 ], [ 6, %bb36.i.i880.6 ], [ 5, %bb36.i.i880.5 ], [ 4, %bb36.i.i880.4 ], [ 3, %bb36.i.i880.3 ], [ 2, %bb36.i.i880.2 ], [ 1, %bb36.i.i880.1 ], [ 0, %bb36.i.i880.lr.ph ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_159.1.i.i885.lcssa.ph, i32 noundef %_159.1.i.i885.lcssa.ph, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_33746731a929bad63709ddedbca82f5f) #28, !dbg !5195, !noalias !5183
  unreachable, !dbg !5195

bb18.i.i898:                                      ; preds = %bb14.i.i888
  %269 = getelementptr inbounds nuw float, ptr %_161.0.i.i899, i32 %_49.i.i893, !dbg !5209
  %_47.i.i900 = load float, ptr %269, align 4, !dbg !5209, !noalias !5183, !noundef !10
  store float %_47.i.i900, ptr %scratch.i706, align 4, !dbg !5210, !alias.scope !5090, !noalias !5211
  %270 = icmp eq i32 %width.i.i857, 1, !dbg !5192
  br i1 %270, label %bb53.i.i901, label %bb36.i.i880.1, !dbg !5192

bb36.i.i880.1:                                    ; preds = %bb18.i.i898
  %exitcond11907.1.not = icmp eq i32 %_159.1.i.i885, 1, !dbg !5195
  br i1 %exitcond11907.1.not, label %panic.i.i887, label %bb14.i.i888.1, !dbg !5195

bb14.i.i888.1:                                    ; preds = %bb36.i.i880.1
  %271 = getelementptr inbounds nuw i8, ptr %_159.0.i.i889, i32 20, !dbg !5195
  %_42.i.i890.1 = load i32, ptr %271, align 4, !dbg !5195, !noalias !5183, !noundef !10
  %272 = add i32 %_42.i.i890.1, %ring_cursor.sroa.0.1.i7396466, !dbg !5206
  %_45.not.i.i891.1 = icmp ult i32 %272, %_87.i763, !dbg !5207
  %273 = select i1 %_45.not.i.i891.1, i32 0, i32 %_87.i763, !dbg !5207
  %spec.select.i.i892.1 = sub nuw i32 %272, %273, !dbg !5207
  %_49.i.i893.1 = mul i32 %spec.select.i.i892.1, %width.i.i857, !dbg !5208
  %_48.i.i894.1 = add i32 %_49.i.i893.1, 1, !dbg !5208
  %_51.i.i896.1 = icmp ult i32 %_48.i.i894.1, %_163.1.i.i906.pre, !dbg !5209
  br i1 %_51.i.i896.1, label %bb18.i.i898.1, label %panic1.i.i897, !dbg !5209

bb18.i.i898.1:                                    ; preds = %bb14.i.i888.1
  %274 = getelementptr inbounds nuw float, ptr %_161.0.i.i899, i32 %_48.i.i894.1, !dbg !5209
  %_47.i.i900.1 = load float, ptr %274, align 4, !dbg !5209, !noalias !5183, !noundef !10
  store float %_47.i.i900.1, ptr %iter.sroa.0.0.ptr.i.i8796463.1, align 4, !dbg !5210, !alias.scope !5090, !noalias !5211
  %275 = icmp eq i32 %width.i.i857, 2, !dbg !5192
  br i1 %275, label %bb53.i.i901, label %bb36.i.i880.2, !dbg !5192

bb36.i.i880.2:                                    ; preds = %bb18.i.i898.1
  %exitcond11907.2.not = icmp eq i32 %_159.1.i.i885, 2, !dbg !5195
  br i1 %exitcond11907.2.not, label %panic.i.i887, label %bb14.i.i888.2, !dbg !5195

bb14.i.i888.2:                                    ; preds = %bb36.i.i880.2
  %276 = getelementptr inbounds nuw i8, ptr %_159.0.i.i889, i32 32, !dbg !5195
  %_42.i.i890.2 = load i32, ptr %276, align 4, !dbg !5195, !noalias !5183, !noundef !10
  %277 = add i32 %_42.i.i890.2, %ring_cursor.sroa.0.1.i7396466, !dbg !5206
  %_45.not.i.i891.2 = icmp ult i32 %277, %_87.i763, !dbg !5207
  %278 = select i1 %_45.not.i.i891.2, i32 0, i32 %_87.i763, !dbg !5207
  %spec.select.i.i892.2 = sub nuw i32 %277, %278, !dbg !5207
  %_49.i.i893.2 = mul i32 %spec.select.i.i892.2, %width.i.i857, !dbg !5208
  %_48.i.i894.2 = add i32 %_49.i.i893.2, 2, !dbg !5208
  %_51.i.i896.2 = icmp ult i32 %_48.i.i894.2, %_163.1.i.i906.pre, !dbg !5209
  br i1 %_51.i.i896.2, label %bb18.i.i898.2, label %panic1.i.i897, !dbg !5209

bb18.i.i898.2:                                    ; preds = %bb14.i.i888.2
  %279 = getelementptr inbounds nuw float, ptr %_161.0.i.i899, i32 %_48.i.i894.2, !dbg !5209
  %_47.i.i900.2 = load float, ptr %279, align 4, !dbg !5209, !noalias !5183, !noundef !10
  store float %_47.i.i900.2, ptr %iter.sroa.0.0.ptr.i.i8796463.2, align 4, !dbg !5210, !alias.scope !5090, !noalias !5211
  %280 = icmp eq i32 %width.i.i857, 3, !dbg !5192
  br i1 %280, label %bb53.i.i901, label %bb36.i.i880.3, !dbg !5192

bb36.i.i880.3:                                    ; preds = %bb18.i.i898.2
  %exitcond11907.3.not = icmp eq i32 %_159.1.i.i885, 3, !dbg !5195
  br i1 %exitcond11907.3.not, label %panic.i.i887, label %bb14.i.i888.3, !dbg !5195

bb14.i.i888.3:                                    ; preds = %bb36.i.i880.3
  %281 = getelementptr inbounds nuw i8, ptr %_159.0.i.i889, i32 44, !dbg !5195
  %_42.i.i890.3 = load i32, ptr %281, align 4, !dbg !5195, !noalias !5183, !noundef !10
  %282 = add i32 %_42.i.i890.3, %ring_cursor.sroa.0.1.i7396466, !dbg !5206
  %_45.not.i.i891.3 = icmp ult i32 %282, %_87.i763, !dbg !5207
  %283 = select i1 %_45.not.i.i891.3, i32 0, i32 %_87.i763, !dbg !5207
  %spec.select.i.i892.3 = sub nuw i32 %282, %283, !dbg !5207
  %_49.i.i893.3 = mul i32 %spec.select.i.i892.3, %width.i.i857, !dbg !5208
  %_48.i.i894.3 = add i32 %_49.i.i893.3, 3, !dbg !5208
  %_51.i.i896.3 = icmp ult i32 %_48.i.i894.3, %_163.1.i.i906.pre, !dbg !5209
  br i1 %_51.i.i896.3, label %bb18.i.i898.3, label %panic1.i.i897, !dbg !5209

bb18.i.i898.3:                                    ; preds = %bb14.i.i888.3
  %284 = getelementptr inbounds nuw float, ptr %_161.0.i.i899, i32 %_48.i.i894.3, !dbg !5209
  %_47.i.i900.3 = load float, ptr %284, align 4, !dbg !5209, !noalias !5183, !noundef !10
  store float %_47.i.i900.3, ptr %iter.sroa.0.0.ptr.i.i8796463.3, align 4, !dbg !5210, !alias.scope !5090, !noalias !5211
  %285 = icmp eq i32 %width.i.i857, 4, !dbg !5192
  br i1 %285, label %bb53.i.i901, label %bb36.i.i880.4, !dbg !5192

bb36.i.i880.4:                                    ; preds = %bb18.i.i898.3
  %exitcond11907.4.not = icmp eq i32 %_159.1.i.i885, 4, !dbg !5195
  br i1 %exitcond11907.4.not, label %panic.i.i887, label %bb14.i.i888.4, !dbg !5195

bb14.i.i888.4:                                    ; preds = %bb36.i.i880.4
  %286 = getelementptr inbounds nuw i8, ptr %_159.0.i.i889, i32 56, !dbg !5195
  %_42.i.i890.4 = load i32, ptr %286, align 4, !dbg !5195, !noalias !5183, !noundef !10
  %287 = add i32 %_42.i.i890.4, %ring_cursor.sroa.0.1.i7396466, !dbg !5206
  %_45.not.i.i891.4 = icmp ult i32 %287, %_87.i763, !dbg !5207
  %288 = select i1 %_45.not.i.i891.4, i32 0, i32 %_87.i763, !dbg !5207
  %spec.select.i.i892.4 = sub nuw i32 %287, %288, !dbg !5207
  %_49.i.i893.4 = mul i32 %spec.select.i.i892.4, %width.i.i857, !dbg !5208
  %_48.i.i894.4 = add i32 %_49.i.i893.4, 4, !dbg !5208
  %_51.i.i896.4 = icmp ult i32 %_48.i.i894.4, %_163.1.i.i906.pre, !dbg !5209
  br i1 %_51.i.i896.4, label %bb18.i.i898.4, label %panic1.i.i897, !dbg !5209

bb18.i.i898.4:                                    ; preds = %bb14.i.i888.4
  %289 = getelementptr inbounds nuw float, ptr %_161.0.i.i899, i32 %_48.i.i894.4, !dbg !5209
  %_47.i.i900.4 = load float, ptr %289, align 4, !dbg !5209, !noalias !5183, !noundef !10
  store float %_47.i.i900.4, ptr %iter.sroa.0.0.ptr.i.i8796463.4, align 4, !dbg !5210, !alias.scope !5090, !noalias !5211
  %290 = icmp eq i32 %width.i.i857, 5, !dbg !5192
  br i1 %290, label %bb53.i.i901, label %bb36.i.i880.5, !dbg !5192

bb36.i.i880.5:                                    ; preds = %bb18.i.i898.4
  %exitcond11907.5.not = icmp eq i32 %_159.1.i.i885, 5, !dbg !5195
  br i1 %exitcond11907.5.not, label %panic.i.i887, label %bb14.i.i888.5, !dbg !5195

bb14.i.i888.5:                                    ; preds = %bb36.i.i880.5
  %291 = getelementptr inbounds nuw i8, ptr %_159.0.i.i889, i32 68, !dbg !5195
  %_42.i.i890.5 = load i32, ptr %291, align 4, !dbg !5195, !noalias !5183, !noundef !10
  %292 = add i32 %_42.i.i890.5, %ring_cursor.sroa.0.1.i7396466, !dbg !5206
  %_45.not.i.i891.5 = icmp ult i32 %292, %_87.i763, !dbg !5207
  %293 = select i1 %_45.not.i.i891.5, i32 0, i32 %_87.i763, !dbg !5207
  %spec.select.i.i892.5 = sub nuw i32 %292, %293, !dbg !5207
  %_49.i.i893.5 = mul i32 %spec.select.i.i892.5, %width.i.i857, !dbg !5208
  %_48.i.i894.5 = add i32 %_49.i.i893.5, 5, !dbg !5208
  %_51.i.i896.5 = icmp ult i32 %_48.i.i894.5, %_163.1.i.i906.pre, !dbg !5209
  br i1 %_51.i.i896.5, label %bb18.i.i898.5, label %panic1.i.i897, !dbg !5209

bb18.i.i898.5:                                    ; preds = %bb14.i.i888.5
  %294 = getelementptr inbounds nuw float, ptr %_161.0.i.i899, i32 %_48.i.i894.5, !dbg !5209
  %_47.i.i900.5 = load float, ptr %294, align 4, !dbg !5209, !noalias !5183, !noundef !10
  store float %_47.i.i900.5, ptr %iter.sroa.0.0.ptr.i.i8796463.5, align 4, !dbg !5210, !alias.scope !5090, !noalias !5211
  %295 = icmp eq i32 %width.i.i857, 6, !dbg !5192
  br i1 %295, label %bb53.i.i901, label %bb36.i.i880.6, !dbg !5192

bb36.i.i880.6:                                    ; preds = %bb18.i.i898.5
  %exitcond11907.6.not = icmp eq i32 %_159.1.i.i885, 6, !dbg !5195
  br i1 %exitcond11907.6.not, label %panic.i.i887, label %bb14.i.i888.6, !dbg !5195

bb14.i.i888.6:                                    ; preds = %bb36.i.i880.6
  %296 = getelementptr inbounds nuw i8, ptr %_159.0.i.i889, i32 80, !dbg !5195
  %_42.i.i890.6 = load i32, ptr %296, align 4, !dbg !5195, !noalias !5183, !noundef !10
  %297 = add i32 %_42.i.i890.6, %ring_cursor.sroa.0.1.i7396466, !dbg !5206
  %_45.not.i.i891.6 = icmp ult i32 %297, %_87.i763, !dbg !5207
  %298 = select i1 %_45.not.i.i891.6, i32 0, i32 %_87.i763, !dbg !5207
  %spec.select.i.i892.6 = sub nuw i32 %297, %298, !dbg !5207
  %_49.i.i893.6 = mul i32 %spec.select.i.i892.6, %width.i.i857, !dbg !5208
  %_48.i.i894.6 = add i32 %_49.i.i893.6, 6, !dbg !5208
  %_51.i.i896.6 = icmp ult i32 %_48.i.i894.6, %_163.1.i.i906.pre, !dbg !5209
  br i1 %_51.i.i896.6, label %bb18.i.i898.6, label %panic1.i.i897, !dbg !5209

bb18.i.i898.6:                                    ; preds = %bb14.i.i888.6
  %299 = getelementptr inbounds nuw float, ptr %_161.0.i.i899, i32 %_48.i.i894.6, !dbg !5209
  %_47.i.i900.6 = load float, ptr %299, align 4, !dbg !5209, !noalias !5183, !noundef !10
  store float %_47.i.i900.6, ptr %iter.sroa.0.0.ptr.i.i8796463.6, align 4, !dbg !5210, !alias.scope !5090, !noalias !5211
  %300 = icmp eq i32 %width.i.i857, 7, !dbg !5192
  br i1 %300, label %bb53.i.i901, label %bb36.i.i880.7, !dbg !5192

bb36.i.i880.7:                                    ; preds = %bb18.i.i898.6
  %exitcond11907.7.not = icmp eq i32 %_159.1.i.i885, 7, !dbg !5195
  br i1 %exitcond11907.7.not, label %panic.i.i887, label %bb14.i.i888.7, !dbg !5195

bb14.i.i888.7:                                    ; preds = %bb36.i.i880.7
  %301 = getelementptr inbounds nuw i8, ptr %_159.0.i.i889, i32 92, !dbg !5195
  %_42.i.i890.7 = load i32, ptr %301, align 4, !dbg !5195, !noalias !5183, !noundef !10
  %302 = add i32 %_42.i.i890.7, %ring_cursor.sroa.0.1.i7396466, !dbg !5206
  %_45.not.i.i891.7 = icmp ult i32 %302, %_87.i763, !dbg !5207
  %303 = select i1 %_45.not.i.i891.7, i32 0, i32 %_87.i763, !dbg !5207
  %spec.select.i.i892.7 = sub nuw i32 %302, %303, !dbg !5207
  %_49.i.i893.7 = mul i32 %spec.select.i.i892.7, %width.i.i857, !dbg !5208
  %_48.i.i894.7 = add i32 %_49.i.i893.7, 7, !dbg !5208
  %_51.i.i896.7 = icmp ult i32 %_48.i.i894.7, %_163.1.i.i906.pre, !dbg !5209
  br i1 %_51.i.i896.7, label %bb18.i.i898.7, label %panic1.i.i897, !dbg !5209

bb18.i.i898.7:                                    ; preds = %bb14.i.i888.7
  %304 = getelementptr inbounds nuw float, ptr %_161.0.i.i899, i32 %_48.i.i894.7, !dbg !5209
  %_47.i.i900.7 = load float, ptr %304, align 4, !dbg !5209, !noalias !5183, !noundef !10
  store float %_47.i.i900.7, ptr %iter.sroa.0.0.ptr.i.i8796463.7, align 4, !dbg !5210, !alias.scope !5090, !noalias !5211
  br label %bb53.i.i901, !dbg !5192

panic1.i.i897:                                    ; preds = %bb14.i.i888.7, %bb14.i.i888.6, %bb14.i.i888.5, %bb14.i.i888.4, %bb14.i.i888.3, %bb14.i.i888.2, %bb14.i.i888.1, %bb14.i.i888
  %_48.i.i894.lcssa.ph = phi i32 [ %_48.i.i894.7, %bb14.i.i888.7 ], [ %_48.i.i894.6, %bb14.i.i888.6 ], [ %_48.i.i894.5, %bb14.i.i888.5 ], [ %_48.i.i894.4, %bb14.i.i888.4 ], [ %_48.i.i894.3, %bb14.i.i888.3 ], [ %_48.i.i894.2, %bb14.i.i888.2 ], [ %_48.i.i894.1, %bb14.i.i888.1 ], [ %_49.i.i893, %bb14.i.i888 ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_48.i.i894.lcssa.ph, i32 noundef %_163.1.i.i906.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_0e76a26fbb751dfb8cf8a069b5b0d39a) #28, !dbg !5209, !noalias !5183
  unreachable, !dbg !5209

bb42.i.i908:                                      ; preds = %bb53.i.i901
  %_163.0.i.i909 = load ptr, ptr %158, align 4, !dbg !5194, !alias.scope !5093, !noalias !5094, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5212), !dbg !5215
  %_4.not.i3082 = icmp eq i32 %_163.1.i.i906.pre, %_22.i.i863, !dbg !5216
  br i1 %_4.not.i3082, label %panic.i3084, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3085, !dbg !5216

panic.i3084:                                      ; preds = %bb42.i.i908
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #28, !dbg !5216, !noalias !5218
  unreachable, !dbg !5216

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3085: ; preds = %bb42.i.i908
  %_130.i.i911 = getelementptr inbounds nuw float, ptr %_163.0.i.i909, i32 %_22.i.i863, !dbg !5219
  store float %_0.i2512, ptr %_130.i.i911, align 4, !dbg !5216, !alias.scope !5212, !noalias !5183
  %_0.i2383 = fdiv float %_0.i2834, %_62.i.i913, !dbg !5221
  %_0.i2833 = fsub float 1.000000e+00, %_0.i2383, !dbg !5223
  %_0.i2832 = fsub float %_0.i2833, %_0.i32087509, !dbg !5225
  %_4.i2399 = fmul float %_0.i3338, %_0.i2832, !dbg !5227
  %_0.i2400 = fadd float %_0.i32087509, %_4.i2399, !dbg !5227
  %_3.i.i3566.inv = fcmp ogt float %_0.i2833, %_0.i2400, !dbg !5229
  %_4.i.i3573.v = select i1 %_3.i.i3566.inv, float %_0.i2833, float %_0.i2400, !dbg !5229
  %305 = tail call noundef float @llvm.fabs.f32(float %_4.i.i3573.v), !dbg !5232
  %306 = fcmp uge float %305, 0x3BC79CA100000000, !dbg !5235
  %_0.i3208 = select i1 %306, float %_4.i.i3573.v, float 0.000000e+00, !dbg !5237
  %_0.i2831 = fsub float 1.000000e+00, %_0.i3208, !dbg !5238
  %_164.1.i.i925 = load i32, ptr %162, align 4, !dbg !5240, !alias.scope !5093, !noalias !5094, !noundef !10
  %_74.i.i926 = mul i32 %width.i.i857, %main_cursor.sroa.0.1.i7406467, !dbg !5241
  %_134.i.i927 = icmp ugt i32 %_74.i.i926, %_164.1.i.i925, !dbg !5242
  br i1 %_134.i.i927, label %bb47.i.i952, label %bb48.i.i928, !dbg !5242, !prof !755

bb41.i.i953:                                      ; preds = %bb53.i.i901
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i863, i32 noundef %_163.1.i.i906.pre, i32 noundef %_163.1.i.i906.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c2286ff41a33b8c11aa270fe215441de) #28, !dbg !5245, !noalias !5183
  unreachable, !dbg !5245

bb48.i.i928:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3085
  %_164.0.i.i929 = load ptr, ptr %163, align 4, !dbg !5240, !alias.scope !5093, !noalias !5094, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5246), !dbg !5249
  %_3.not.i2918 = icmp eq i32 %_164.1.i.i925, %_74.i.i926, !dbg !5250
  br i1 %_3.not.i2918, label %panic.i2921, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077, !dbg !5250

panic.i2921:                                      ; preds = %bb48.i.i928
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #28, !dbg !5250, !noalias !5252
  unreachable, !dbg !5250

bb47.i.i952:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3085
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_74.i.i926, i32 noundef %_164.1.i.i925, i32 noundef %_164.1.i.i925, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a3a21efd18b403ec25db545d522c479) #28, !dbg !5253, !noalias !5183
  unreachable, !dbg !5253

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3077: ; preds = %bb48.i.i928
  %_141.i.i931 = getelementptr inbounds nuw float, ptr %_164.0.i.i929, i32 %_74.i.i926, !dbg !5254
  %_0.i2920 = load float, ptr %_141.i.i931, align 4, !dbg !5250, !alias.scope !5246, !noalias !5183, !noundef !10
  store float %_0.i2929, ptr %_141.i.i931, align 4, !dbg !5256, !alias.scope !5258, !noalias !5183
  %_0.i2511 = fmul float %_0.i2831, %_0.i2920, !dbg !5261
  %_6.i3339 = bitcast float %_0.i2920 to i32, !dbg !5263
  %_5.i3340 = and i32 %_6.i3339, %all.sroa.0.0.i715, !dbg !5266
  %_8.i3341 = bitcast float %_0.i2511 to i32, !dbg !5267
  %_7.i3342 = and i32 %_9.i3354, %_8.i3341, !dbg !5269
  %_4.i3343 = or disjoint i32 %_7.i3342, %_5.i3340, !dbg !5266
  store i32 %_4.i3343, ptr %_147.i855, align 4, !dbg !5270, !alias.scope !5272, !noalias !5275
  %307 = add i32 %main_cursor.sroa.0.1.i7406467, 1, !dbg !5276
  %_100.i946 = icmp eq i32 %307, %_102.i945, !dbg !5277
  %spec.store.select11.i947 = select i1 %_100.i946, i32 0, i32 %307, !dbg !5277
  %308 = add i32 %ring_cursor.sroa.0.1.i7396466, 1, !dbg !5278
  %_103.i948 = icmp eq i32 %308, %_87.i763, !dbg !5279
  %spec.store.select12.i949 = select i1 %_103.i948, i32 0, i32 %308, !dbg !5279
  %exitcond11910.not = icmp eq i32 %180, %umax11909, !dbg !5280
  br i1 %exitcond11910.not, label %bb16.i737.bb13.i725.loopexit_crit_edge, label %bb42.i742, !dbg !4115

bb48.i955:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3093
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_60.i743, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_3ec0ee57975400dd5dbd052e150c5392) #28, !dbg !5283, !noalias !4072
  unreachable, !dbg !5283

_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit: ; preds = %bb13.i725.loopexit, %bb12.i
  %ring_cursor.sroa.0.0.i728.lcssa = phi i32 [ %_37.i717, %bb12.i ], [ %ring_cursor.sroa.0.1.i739.lcssa, %bb13.i725.loopexit ], !dbg !4008
  %main_cursor.sroa.0.0.i729.lcssa = phi i32 [ %_35.i716, %bb12.i ], [ %main_cursor.sroa.0.1.i740.lcssa, %bb13.i725.loopexit ], !dbg !4005
  call void @llvm.lifetime.start.p0(ptr nonnull %_106.i703), !dbg !5284, !noalias !3988
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(92) %_106.i703, ptr noundef nonnull align 4 dereferenceable(92) %hot_left.i708, i32 92, i1 false), !dbg !5284, !noalias !3988
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_106.i703, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33) #27, !dbg !5285, !noalias !4072
  call void @llvm.lifetime.end.p0(ptr nonnull %_106.i703), !dbg !5286, !noalias !3988
  call void @llvm.lifetime.start.p0(ptr nonnull %_108.i702), !dbg !5287, !noalias !3988
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(92) %_108.i702, ptr noundef nonnull align 4 dereferenceable(92) %hot_right.i707, i32 92, i1 false), !dbg !5287, !noalias !3988
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_108.i702, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34) #27, !dbg !5288, !noalias !4072
  call void @llvm.lifetime.end.p0(ptr nonnull %_108.i702), !dbg !5289, !noalias !3988
  store i32 %main_cursor.sroa.0.0.i729.lcssa, ptr %_35, align 4, !dbg !5290, !alias.scope !3982, !noalias !4007
  store i32 %ring_cursor.sroa.0.0.i728.lcssa, ptr %81, align 4, !dbg !5291, !alias.scope !3982, !noalias !4007
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i704), !dbg !5292, !noalias !3988
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i705), !dbg !5293, !noalias !3988
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i706), !dbg !5294, !noalias !3988
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_right.i707), !dbg !5295, !noalias !3988
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i708), !dbg !5296, !noalias !3988
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockfEB2_.exit, !dbg !3977

bb11.i:                                           ; preds = %bb10.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5297), !dbg !5300
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5301), !dbg !5300
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5303), !dbg !5300
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5305), !dbg !5300
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i437), !dbg !5307, !noalias !5311
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_left.i437, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #27, !dbg !5315, !noalias !5316
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_right.i436), !dbg !5317, !noalias !5311
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_right.i436, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #27, !dbg !5319, !noalias !5320
  %309 = load i8, ptr %79, align 4, !dbg !5321, !range !3570, !alias.scope !5297, !noalias !5325, !noundef !10
  %310 = load i8, ptr %80, align 1, !dbg !5326, !range !3570, !alias.scope !5297, !noalias !5325, !noundef !10
  %_35.i = load i32, ptr %_35, align 4, !dbg !5328, !alias.scope !5305, !noalias !5330, !noundef !10
  %_37.i445 = load i32, ptr %81, align 4, !dbg !5331, !alias.scope !5305, !noalias !5330, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i), !dbg !5333, !noalias !5311
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i, i8 0, i32 32, i1 false), !noalias !5311
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i435), !dbg !5335, !noalias !5311
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i435, i8 0, i32 1024, i1 false), !noalias !5311
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i434), !dbg !5337, !noalias !5311
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i434, i8 0, i32 1024, i1 false), !noalias !5311
  %_32.i441 = zext nneg i8 %309 to i32, !dbg !5321
  %.none.i442 = sub nsw i32 0, %_32.i441, !dbg !5339
  %_33.i443 = zext nneg i8 %310 to i32, !dbg !5326
  %all.sroa.0.0.i444 = sub nsw i32 0, %_33.i443, !dbg !5326
  %_111.not.i7944 = icmp eq i32 %frames, 0, !dbg !5340
  br i1 %_111.not.i7944, label %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit, label %bb37.i.lr.ph, !dbg !5340

bb37.i.lr.ph:                                     ; preds = %bb11.i
  %d9.i4013 = lshr i32 %frames, 5, !dbg !5350
  %r2.i4014 = and i32 %frames, 31, !dbg !5357
  %_19.not.i4015 = icmp ne i32 %r2.i4014, 0, !dbg !5358
  %311 = zext i1 %_19.not.i4015 to i32, !dbg !5358
  %yield_count.sroa.0.0.i4016 = add nuw nsw i32 %d9.i4013, %311, !dbg !5358
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
  %iter.sroa.0.0.ptr.i55.i7649.1 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 4
  %iter.sroa.0.0.ptr.i55.i7649.2 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 8
  %iter.sroa.0.0.ptr.i55.i7649.3 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 12
  %iter.sroa.0.0.ptr.i55.i7649.4 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 16
  %iter.sroa.0.0.ptr.i55.i7649.5 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 20
  %iter.sroa.0.0.ptr.i55.i7649.6 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 24
  %iter.sroa.0.0.ptr.i55.i7649.7 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 28
  %iter.sroa.0.0.ptr.i.i7661.1 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 4
  %iter.sroa.0.0.ptr.i.i7661.2 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 8
  %iter.sroa.0.0.ptr.i.i7661.3 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 12
  %iter.sroa.0.0.ptr.i.i7661.4 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 16
  %iter.sroa.0.0.ptr.i.i7661.5 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 20
  %iter.sroa.0.0.ptr.i.i7661.6 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 24
  %iter.sroa.0.0.ptr.i.i7661.7 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 28
  br label %bb37.i, !dbg !5340

bb16.i.bb13.i.loopexit_crit_edge:                 ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3117
  store float %_0.i2846, ptr %358, align 4, !dbg !5359
  store float %_0.i3220, ptr %360, align 4, !dbg !5373
  store float %_0.i2842, ptr %374, align 4, !dbg !5374
  store float %_0.i3216, ptr %376, align 4, !dbg !5376
  br label %bb13.i.loopexit, !dbg !5377

bb13.i.loopexit:                                  ; preds = %bb16.i.bb13.i.loopexit_crit_edge, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i457
  %ring_cursor.sroa.0.1.i459.lcssa = phi i32 [ %spec.store.select12.i, %bb16.i.bb13.i.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i4517947, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i457 ], !dbg !5383
  %main_cursor.sroa.0.1.i460.lcssa = phi i32 [ %spec.store.select11.i, %bb16.i.bb13.i.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i4527948, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i457 ], !dbg !5384
  %_111.not.i = icmp eq i32 %382, 0, !dbg !5340
  %indvars.iv.next11912 = add i32 %indvars.iv11911, -32, !dbg !5340
  br i1 %_111.not.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit, label %bb37.i, !dbg !5340

bb37.i:                                           ; preds = %bb37.i.lr.ph, %bb13.i.loopexit
  %indvars.iv11911 = phi i32 [ %frames, %bb37.i.lr.ph ], [ %indvars.iv.next11912, %bb13.i.loopexit ]
  %main_cursor.sroa.0.0.i4527948 = phi i32 [ %_35.i, %bb37.i.lr.ph ], [ %main_cursor.sroa.0.1.i460.lcssa, %bb13.i.loopexit ]
  %ring_cursor.sroa.0.0.i4517947 = phi i32 [ %_37.i445, %bb37.i.lr.ph ], [ %ring_cursor.sroa.0.1.i459.lcssa, %bb13.i.loopexit ]
  %iter2.sroa.0.0.i4507946 = phi i32 [ %yield_count.sroa.0.0.i4016, %bb37.i.lr.ph ], [ %382, %bb13.i.loopexit ]
  %iter.sroa.0.0.i7945 = phi i32 [ 0, %bb37.i.lr.ph ], [ %381, %bb13.i.loopexit ]
  %380 = call i32 @llvm.umax.i32(i32 %indvars.iv11911, i32 1), !dbg !5385
  %umax11930 = call i32 @llvm.umin.i32(i32 %380, i32 32), !dbg !5385
  %381 = add i32 %iter.sroa.0.0.i7945, 32, !dbg !5385
  %382 = add nsw i32 %iter2.sroa.0.0.i4507946, -1, !dbg !5389
  %history.i135.i.sroa.0.0.copyload = load float, ptr %hot_left.i437, align 4, !dbg !5390, !noalias !5392
  %history.i135.i.sroa.7.0.copyload = load float, ptr %history.i135.i.sroa.7.0.hot_left.i437.sroa_idx, align 4, !dbg !5390, !noalias !5392
  %history.i135.i.sroa.10.0.copyload = load float, ptr %history.i135.i.sroa.10.0.hot_left.i437.sroa_idx, align 4, !dbg !5390, !noalias !5392
  %history.i135.i.sroa.13.0.copyload = load float, ptr %history.i135.i.sroa.13.0.hot_left.i437.sroa_idx, align 4, !dbg !5390, !noalias !5392
  %history.i135.i.sroa.16.0.copyload = load float, ptr %history.i135.i.sroa.16.0.hot_left.i437.sroa_idx, align 4, !dbg !5390, !noalias !5392
  %history.i135.i.sroa.19.0.copyload = load float, ptr %history.i135.i.sroa.19.0.hot_left.i437.sroa_idx, align 4, !dbg !5390, !noalias !5392
  %history.i135.i.sroa.22.0.copyload = load float, ptr %history.i135.i.sroa.22.0.hot_left.i437.sroa_idx, align 4, !dbg !5390, !noalias !5392
  %history.i135.i.sroa.26.0.copyload = load float, ptr %history.i135.i.sroa.26.0.hot_left.i437.sroa_idx, align 4, !dbg !5390, !noalias !5392
  %history.i135.i.sroa.29.0.copyload = load float, ptr %history.i135.i.sroa.29.0.hot_left.i437.sroa_idx, align 4, !dbg !5390, !noalias !5392
  %history.i135.i.sroa.32.0.copyload = load float, ptr %history.i135.i.sroa.32.0.hot_left.i437.sroa_idx, align 4, !dbg !5390, !noalias !5392
  %history.i135.i.sroa.35.0.copyload = load float, ptr %history.i135.i.sroa.35.0.hot_left.i437.sroa_idx, align 4, !dbg !5390, !noalias !5392
  %history.i135.i.sroa.38.0.copyload = load float, ptr %history.i135.i.sroa.38.0.hot_left.i437.sroa_idx, align 4, !dbg !5390, !noalias !5392
  %_20.i138.i7586.not = icmp eq i32 %frames, %iter.sroa.0.0.i7945, !dbg !5397
  br i1 %_20.i138.i7586.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i, label %bb5.i139.i.lr.ph, !dbg !5401

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
  br label %bb5.i139.i, !dbg !5401

bb5.i139.i:                                       ; preds = %bb5.i139.i.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960
  %iter.sroa.0.0.i137.i7598 = phi i32 [ 0, %bb5.i139.i.lr.ph ], [ %383, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ]
  %history.i135.i.sroa.35.07597 = phi float [ %history.i135.i.sroa.35.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.32.07596, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ]
  %history.i135.i.sroa.32.07596 = phi float [ %history.i135.i.sroa.32.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.29.07595, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ]
  %history.i135.i.sroa.29.07595 = phi float [ %history.i135.i.sroa.29.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.26.07594, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ]
  %history.i135.i.sroa.26.07594 = phi float [ %history.i135.i.sroa.26.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.22.07593, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ]
  %history.i135.i.sroa.22.07593 = phi float [ %history.i135.i.sroa.22.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.19.07592, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ]
  %history.i135.i.sroa.19.07592 = phi float [ %history.i135.i.sroa.19.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.16.07591, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ]
  %history.i135.i.sroa.16.07591 = phi float [ %history.i135.i.sroa.16.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.13.07590, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ]
  %history.i135.i.sroa.13.07590 = phi float [ %history.i135.i.sroa.13.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.10.07589, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ]
  %history.i135.i.sroa.10.07589 = phi float [ %history.i135.i.sroa.10.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.7.07588, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ]
  %history.i135.i.sroa.7.07588 = phi float [ %history.i135.i.sroa.7.0.copyload, %bb5.i139.i.lr.ph ], [ %history.i135.i.sroa.0.07587, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ]
  %history.i135.i.sroa.0.07587 = phi float [ %history.i135.i.sroa.0.0.copyload, %bb5.i139.i.lr.ph ], [ %_0.i2958, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ]
  %383 = add nuw nsw i32 %iter.sroa.0.0.i137.i7598, 1, !dbg !5402
  %_11.i140.i = add nuw nsw i32 %iter.sroa.0.0.i137.i7598, %iter.sroa.0.0.i7945, !dbg !5405
  %_24.i141.i = icmp ugt i32 %_11.i140.i, %left_io.1, !dbg !5406
  br i1 %_24.i141.i, label %bb7.i338.i, label %bb8.i142.i, !dbg !5406, !prof !755

bb8.i142.i:                                       ; preds = %bb5.i139.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5409), !dbg !5412
  %_3.not.i2956 = icmp eq i32 %left_io.1, %_11.i140.i, !dbg !5413
  br i1 %_3.not.i2956, label %panic.i2959, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960, !dbg !5413

panic.i2959:                                      ; preds = %bb8.i142.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #28, !dbg !5413, !noalias !5415
  unreachable, !dbg !5413

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960: ; preds = %bb8.i142.i
  %_31.i144.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %_11.i140.i, !dbg !5417
  %_0.i2958 = load float, ptr %_31.i144.i, align 4, !dbg !5413, !alias.scope !5409, !noalias !5419, !noundef !10
  %384 = tail call noundef float @llvm.fabs.f32(float %history.i135.i.sroa.19.07592), !dbg !5420
  %_0.i2564 = fmul float %_0.i2958, %_11.i.i.i159.i, !dbg !5423
  %_0.i2136 = fadd float %_0.i2564, 0.000000e+00, !dbg !5426
  %_0.i2563 = fmul float %_0.i2958, %_14.i.i.i162.i, !dbg !5428
  %_0.i2135 = fadd float %_0.i2563, 0.000000e+00, !dbg !5430
  %_0.i2562 = fmul float %_0.i2958, %_17.i.i.i165.i, !dbg !5432
  %_0.i2134 = fadd float %_0.i2562, 0.000000e+00, !dbg !5434
  %_0.i2561 = fmul float %_0.i2958, %_20.i.i.i168.i, !dbg !5436
  %_0.i2133 = fadd float %_0.i2561, 0.000000e+00, !dbg !5438
  %_0.i2560 = fmul float %history.i135.i.sroa.0.07587, %_25.i.i.i173.i, !dbg !5440
  %_0.i2132 = fadd float %_0.i2136, %_0.i2560, !dbg !5442
  %_0.i2559 = fmul float %history.i135.i.sroa.0.07587, %_28.i.i.i176.i, !dbg !5444
  %_0.i2131 = fadd float %_0.i2135, %_0.i2559, !dbg !5446
  %_0.i2558 = fmul float %history.i135.i.sroa.0.07587, %_31.i.i.i179.i, !dbg !5448
  %_0.i2130 = fadd float %_0.i2134, %_0.i2558, !dbg !5450
  %_0.i2557 = fmul float %history.i135.i.sroa.0.07587, %_34.i.i.i182.i, !dbg !5452
  %_0.i2129 = fadd float %_0.i2133, %_0.i2557, !dbg !5454
  %_0.i2556 = fmul float %history.i135.i.sroa.7.07588, %_39.i.i.i187.i, !dbg !5456
  %_0.i2128 = fadd float %_0.i2132, %_0.i2556, !dbg !5458
  %_0.i2555 = fmul float %history.i135.i.sroa.7.07588, %_42.i.i.i190.i, !dbg !5460
  %_0.i2127 = fadd float %_0.i2131, %_0.i2555, !dbg !5462
  %_0.i2554 = fmul float %history.i135.i.sroa.7.07588, %_45.i.i.i193.i, !dbg !5464
  %_0.i2126 = fadd float %_0.i2130, %_0.i2554, !dbg !5466
  %_0.i2553 = fmul float %history.i135.i.sroa.7.07588, %_48.i.i.i196.i, !dbg !5468
  %_0.i2125 = fadd float %_0.i2129, %_0.i2553, !dbg !5470
  %_0.i2552 = fmul float %history.i135.i.sroa.10.07589, %_53.i.i.i201.i, !dbg !5472
  %_0.i2124 = fadd float %_0.i2128, %_0.i2552, !dbg !5474
  %_0.i2551 = fmul float %history.i135.i.sroa.10.07589, %_56.i.i.i204.i, !dbg !5476
  %_0.i2123 = fadd float %_0.i2127, %_0.i2551, !dbg !5478
  %_0.i2550 = fmul float %history.i135.i.sroa.10.07589, %_59.i.i.i207.i, !dbg !5480
  %_0.i2122 = fadd float %_0.i2126, %_0.i2550, !dbg !5482
  %_0.i2549 = fmul float %history.i135.i.sroa.10.07589, %_62.i.i.i210.i, !dbg !5484
  %_0.i2121 = fadd float %_0.i2125, %_0.i2549, !dbg !5486
  %_0.i2548 = fmul float %history.i135.i.sroa.13.07590, %_67.i.i.i215.i, !dbg !5488
  %_0.i2120 = fadd float %_0.i2124, %_0.i2548, !dbg !5490
  %_0.i2547 = fmul float %history.i135.i.sroa.13.07590, %_70.i.i.i218.i, !dbg !5492
  %_0.i2119 = fadd float %_0.i2123, %_0.i2547, !dbg !5494
  %_0.i2546 = fmul float %history.i135.i.sroa.13.07590, %_73.i.i.i221.i, !dbg !5496
  %_0.i2118 = fadd float %_0.i2122, %_0.i2546, !dbg !5498
  %_0.i2545 = fmul float %history.i135.i.sroa.13.07590, %_76.i.i.i224.i, !dbg !5500
  %_0.i2117 = fadd float %_0.i2121, %_0.i2545, !dbg !5502
  %_0.i2544 = fmul float %history.i135.i.sroa.16.07591, %_81.i.i.i229.i, !dbg !5504
  %_0.i2116 = fadd float %_0.i2120, %_0.i2544, !dbg !5506
  %_0.i2543 = fmul float %history.i135.i.sroa.16.07591, %_84.i.i.i232.i, !dbg !5508
  %_0.i2115 = fadd float %_0.i2119, %_0.i2543, !dbg !5510
  %_0.i2542 = fmul float %history.i135.i.sroa.16.07591, %_87.i.i.i235.i, !dbg !5512
  %_0.i2114 = fadd float %_0.i2118, %_0.i2542, !dbg !5514
  %_0.i2541 = fmul float %history.i135.i.sroa.16.07591, %_90.i.i.i238.i, !dbg !5516
  %_0.i2113 = fadd float %_0.i2117, %_0.i2541, !dbg !5518
  %_0.i2540 = fmul float %history.i135.i.sroa.19.07592, %_95.i.i.i243.i, !dbg !5520
  %_0.i2112 = fadd float %_0.i2116, %_0.i2540, !dbg !5522
  %_0.i2539 = fmul float %history.i135.i.sroa.19.07592, %_98.i.i.i246.i, !dbg !5524
  %_0.i2111 = fadd float %_0.i2115, %_0.i2539, !dbg !5526
  %_0.i2538 = fmul float %history.i135.i.sroa.19.07592, %_101.i.i.i249.i, !dbg !5528
  %_0.i2110 = fadd float %_0.i2114, %_0.i2538, !dbg !5530
  %_0.i2537 = fmul float %history.i135.i.sroa.19.07592, %_104.i.i.i252.i, !dbg !5532
  %_0.i2109 = fadd float %_0.i2113, %_0.i2537, !dbg !5534
  %_0.i2536 = fmul float %history.i135.i.sroa.22.07593, %_109.i.i.i257.i, !dbg !5536
  %_0.i2108 = fadd float %_0.i2112, %_0.i2536, !dbg !5538
  %_0.i2535 = fmul float %history.i135.i.sroa.22.07593, %_112.i.i.i260.i, !dbg !5540
  %_0.i2107 = fadd float %_0.i2111, %_0.i2535, !dbg !5542
  %_0.i2534 = fmul float %history.i135.i.sroa.22.07593, %_115.i.i.i263.i, !dbg !5544
  %_0.i2106 = fadd float %_0.i2110, %_0.i2534, !dbg !5546
  %_0.i2533 = fmul float %history.i135.i.sroa.22.07593, %_118.i.i.i266.i, !dbg !5548
  %_0.i2105 = fadd float %_0.i2109, %_0.i2533, !dbg !5550
  %_0.i2532 = fmul float %history.i135.i.sroa.26.07594, %_123.i.i.i271.i, !dbg !5552
  %_0.i2104 = fadd float %_0.i2108, %_0.i2532, !dbg !5554
  %_0.i2531 = fmul float %history.i135.i.sroa.26.07594, %_126.i.i.i274.i, !dbg !5556
  %_0.i2103 = fadd float %_0.i2107, %_0.i2531, !dbg !5558
  %_0.i2530 = fmul float %history.i135.i.sroa.26.07594, %_129.i.i.i277.i, !dbg !5560
  %_0.i2102 = fadd float %_0.i2106, %_0.i2530, !dbg !5562
  %_0.i2529 = fmul float %history.i135.i.sroa.26.07594, %_132.i.i.i280.i, !dbg !5564
  %_0.i2101 = fadd float %_0.i2105, %_0.i2529, !dbg !5566
  %_0.i2528 = fmul float %history.i135.i.sroa.29.07595, %_137.i.i.i285.i, !dbg !5568
  %_0.i2100 = fadd float %_0.i2104, %_0.i2528, !dbg !5570
  %_0.i2527 = fmul float %history.i135.i.sroa.29.07595, %_140.i.i.i288.i, !dbg !5572
  %_0.i2099 = fadd float %_0.i2103, %_0.i2527, !dbg !5574
  %_0.i2526 = fmul float %history.i135.i.sroa.29.07595, %_143.i.i.i291.i, !dbg !5576
  %_0.i2098 = fadd float %_0.i2102, %_0.i2526, !dbg !5578
  %_0.i2525 = fmul float %history.i135.i.sroa.29.07595, %_146.i.i.i294.i, !dbg !5580
  %_0.i2097 = fadd float %_0.i2101, %_0.i2525, !dbg !5582
  %_0.i2524 = fmul float %history.i135.i.sroa.32.07596, %_151.i.i.i299.i, !dbg !5584
  %_0.i2096 = fadd float %_0.i2100, %_0.i2524, !dbg !5586
  %_0.i2523 = fmul float %history.i135.i.sroa.32.07596, %_154.i.i.i302.i, !dbg !5588
  %_0.i2095 = fadd float %_0.i2099, %_0.i2523, !dbg !5590
  %_0.i2522 = fmul float %history.i135.i.sroa.32.07596, %_157.i.i.i305.i, !dbg !5592
  %_0.i2094 = fadd float %_0.i2098, %_0.i2522, !dbg !5594
  %_0.i2521 = fmul float %history.i135.i.sroa.32.07596, %_160.i.i.i308.i, !dbg !5596
  %_0.i2093 = fadd float %_0.i2097, %_0.i2521, !dbg !5598
  %_0.i2520 = fmul float %history.i135.i.sroa.35.07597, %_165.i.i.i313.i, !dbg !5600
  %_0.i2092 = fadd float %_0.i2096, %_0.i2520, !dbg !5602
  %_0.i2519 = fmul float %history.i135.i.sroa.35.07597, %_168.i.i.i316.i, !dbg !5604
  %_0.i2091 = fadd float %_0.i2095, %_0.i2519, !dbg !5606
  %_0.i2518 = fmul float %history.i135.i.sroa.35.07597, %_171.i.i.i319.i, !dbg !5608
  %_0.i2090 = fadd float %_0.i2094, %_0.i2518, !dbg !5610
  %_0.i2517 = fmul float %history.i135.i.sroa.35.07597, %_174.i.i.i322.i, !dbg !5612
  %_0.i2089 = fadd float %_0.i2093, %_0.i2517, !dbg !5614
  %385 = tail call noundef float @llvm.fabs.f32(float %_0.i2092), !dbg !5616
  %_3.i.i3593.inv = fcmp ogt float %384, %385, !dbg !5618
  %_4.i.i3600.v = select i1 %_3.i.i3593.inv, float %384, float %385, !dbg !5618
  %386 = tail call noundef float @llvm.fabs.f32(float %_0.i2091), !dbg !5616
  %_3.i.i3593.inv.1 = fcmp ogt float %_4.i.i3600.v, %386, !dbg !5618
  %_4.i.i3600.v.1 = select i1 %_3.i.i3593.inv.1, float %_4.i.i3600.v, float %386, !dbg !5618
  %387 = tail call noundef float @llvm.fabs.f32(float %_0.i2090), !dbg !5616
  %_3.i.i3593.inv.2 = fcmp ogt float %_4.i.i3600.v.1, %387, !dbg !5618
  %_4.i.i3600.v.2 = select i1 %_3.i.i3593.inv.2, float %_4.i.i3600.v.1, float %387, !dbg !5618
  %388 = tail call noundef float @llvm.fabs.f32(float %_0.i2089), !dbg !5616
  %_3.i.i3593.inv.3 = fcmp ogt float %_4.i.i3600.v.2, %388, !dbg !5618
  %_4.i.i3600.v.3 = select i1 %_3.i.i3593.inv.3, float %_4.i.i3600.v.2, float %388, !dbg !5618
  %_39.i333.i = getelementptr inbounds nuw float, ptr %peaks_left.i435, i32 %iter.sroa.0.0.i137.i7598, !dbg !5621
  store float %_4.i.i3600.v.3, ptr %_39.i333.i, align 4, !dbg !5626, !alias.scope !5628, !noalias !5419
  %exitcond11915.not = icmp eq i32 %383, %umax11930, !dbg !5397
  br i1 %exitcond11915.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i, label %bb5.i139.i, !dbg !5401

bb7.i338.i:                                       ; preds = %bb5.i139.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_11.i140.i, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #28, !dbg !5631, !noalias !5419
  unreachable, !dbg !5631

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960, %bb37.i
  %history.i135.i.sroa.0.0.lcssa = phi float [ %history.i135.i.sroa.0.0.copyload, %bb37.i ], [ %_0.i2958, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ], !dbg !5632
  %history.i135.i.sroa.7.0.lcssa = phi float [ %history.i135.i.sroa.7.0.copyload, %bb37.i ], [ %history.i135.i.sroa.0.07587, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ], !dbg !5632
  %history.i135.i.sroa.10.0.lcssa = phi float [ %history.i135.i.sroa.10.0.copyload, %bb37.i ], [ %history.i135.i.sroa.7.07588, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ], !dbg !5632
  %history.i135.i.sroa.13.0.lcssa = phi float [ %history.i135.i.sroa.13.0.copyload, %bb37.i ], [ %history.i135.i.sroa.10.07589, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ], !dbg !5632
  %history.i135.i.sroa.16.0.lcssa = phi float [ %history.i135.i.sroa.16.0.copyload, %bb37.i ], [ %history.i135.i.sroa.13.07590, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ], !dbg !5632
  %history.i135.i.sroa.19.0.lcssa = phi float [ %history.i135.i.sroa.19.0.copyload, %bb37.i ], [ %history.i135.i.sroa.16.07591, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ], !dbg !5632
  %history.i135.i.sroa.22.0.lcssa = phi float [ %history.i135.i.sroa.22.0.copyload, %bb37.i ], [ %history.i135.i.sroa.19.07592, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ], !dbg !5632
  %history.i135.i.sroa.26.0.lcssa = phi float [ %history.i135.i.sroa.26.0.copyload, %bb37.i ], [ %history.i135.i.sroa.22.07593, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ], !dbg !5632
  %history.i135.i.sroa.29.0.lcssa = phi float [ %history.i135.i.sroa.29.0.copyload, %bb37.i ], [ %history.i135.i.sroa.26.07594, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ], !dbg !5632
  %history.i135.i.sroa.32.0.lcssa = phi float [ %history.i135.i.sroa.32.0.copyload, %bb37.i ], [ %history.i135.i.sroa.29.07595, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ], !dbg !5632
  %history.i135.i.sroa.35.0.lcssa = phi float [ %history.i135.i.sroa.35.0.copyload, %bb37.i ], [ %history.i135.i.sroa.32.07596, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ], !dbg !5632
  %history.i135.i.sroa.38.0.lcssa = phi float [ %history.i135.i.sroa.38.0.copyload, %bb37.i ], [ %history.i135.i.sroa.35.07597, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2960 ], !dbg !5632
  store float %history.i135.i.sroa.0.0.lcssa, ptr %hot_left.i437, align 4, !dbg !5633, !noalias !5392
  store float %history.i135.i.sroa.7.0.lcssa, ptr %history.i135.i.sroa.7.0.hot_left.i437.sroa_idx, align 4, !dbg !5633, !noalias !5392
  store float %history.i135.i.sroa.10.0.lcssa, ptr %history.i135.i.sroa.10.0.hot_left.i437.sroa_idx, align 4, !dbg !5633, !noalias !5392
  store float %history.i135.i.sroa.13.0.lcssa, ptr %history.i135.i.sroa.13.0.hot_left.i437.sroa_idx, align 4, !dbg !5633, !noalias !5392
  store float %history.i135.i.sroa.16.0.lcssa, ptr %history.i135.i.sroa.16.0.hot_left.i437.sroa_idx, align 4, !dbg !5633, !noalias !5392
  store float %history.i135.i.sroa.19.0.lcssa, ptr %history.i135.i.sroa.19.0.hot_left.i437.sroa_idx, align 4, !dbg !5633, !noalias !5392
  store float %history.i135.i.sroa.22.0.lcssa, ptr %history.i135.i.sroa.22.0.hot_left.i437.sroa_idx, align 4, !dbg !5633, !noalias !5392
  store float %history.i135.i.sroa.26.0.lcssa, ptr %history.i135.i.sroa.26.0.hot_left.i437.sroa_idx, align 4, !dbg !5633, !noalias !5392
  store float %history.i135.i.sroa.29.0.lcssa, ptr %history.i135.i.sroa.29.0.hot_left.i437.sroa_idx, align 4, !dbg !5633, !noalias !5392
  store float %history.i135.i.sroa.32.0.lcssa, ptr %history.i135.i.sroa.32.0.hot_left.i437.sroa_idx, align 4, !dbg !5633, !noalias !5392
  store float %history.i135.i.sroa.35.0.lcssa, ptr %history.i135.i.sroa.35.0.hot_left.i437.sroa_idx, align 4, !dbg !5633, !noalias !5392
  store float %history.i135.i.sroa.38.0.lcssa, ptr %history.i135.i.sroa.38.0.hot_left.i437.sroa_idx, align 4, !dbg !5633, !noalias !5392
  %history.i.i433.sroa.0.0.copyload = load float, ptr %hot_right.i436, align 4, !dbg !5634, !noalias !5636
  %history.i.i433.sroa.7.0.copyload = load float, ptr %history.i.i433.sroa.7.0.hot_right.i436.sroa_idx, align 4, !dbg !5634, !noalias !5636
  %history.i.i433.sroa.10.0.copyload = load float, ptr %history.i.i433.sroa.10.0.hot_right.i436.sroa_idx, align 4, !dbg !5634, !noalias !5636
  %history.i.i433.sroa.13.0.copyload = load float, ptr %history.i.i433.sroa.13.0.hot_right.i436.sroa_idx, align 4, !dbg !5634, !noalias !5636
  %history.i.i433.sroa.16.0.copyload = load float, ptr %history.i.i433.sroa.16.0.hot_right.i436.sroa_idx, align 4, !dbg !5634, !noalias !5636
  %history.i.i433.sroa.19.0.copyload = load float, ptr %history.i.i433.sroa.19.0.hot_right.i436.sroa_idx, align 4, !dbg !5634, !noalias !5636
  %history.i.i433.sroa.22.0.copyload = load float, ptr %history.i.i433.sroa.22.0.hot_right.i436.sroa_idx, align 4, !dbg !5634, !noalias !5636
  %history.i.i433.sroa.26.0.copyload = load float, ptr %history.i.i433.sroa.26.0.hot_right.i436.sroa_idx, align 4, !dbg !5634, !noalias !5636
  %history.i.i433.sroa.29.0.copyload = load float, ptr %history.i.i433.sroa.29.0.hot_right.i436.sroa_idx, align 4, !dbg !5634, !noalias !5636
  %history.i.i433.sroa.32.0.copyload = load float, ptr %history.i.i433.sroa.32.0.hot_right.i436.sroa_idx, align 4, !dbg !5634, !noalias !5636
  %history.i.i433.sroa.35.0.copyload = load float, ptr %history.i.i433.sroa.35.0.hot_right.i436.sroa_idx, align 4, !dbg !5634, !noalias !5636
  %history.i.i433.sroa.38.0.copyload = load float, ptr %history.i.i433.sroa.38.0.hot_right.i436.sroa_idx, align 4, !dbg !5634, !noalias !5636
  br i1 %_20.i138.i7586.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i457, label %bb5.i.i492.lr.ph, !dbg !5641

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
  br label %bb5.i.i492, !dbg !5641

bb5.i.i492:                                       ; preds = %bb5.i.i492.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965
  %iter.sroa.0.0.i.i4557625 = phi i32 [ 0, %bb5.i.i492.lr.ph ], [ %389, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ]
  %history.i.i433.sroa.35.07624 = phi float [ %history.i.i433.sroa.35.0.copyload, %bb5.i.i492.lr.ph ], [ %history.i.i433.sroa.32.07623, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ]
  %history.i.i433.sroa.32.07623 = phi float [ %history.i.i433.sroa.32.0.copyload, %bb5.i.i492.lr.ph ], [ %history.i.i433.sroa.29.07622, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ]
  %history.i.i433.sroa.29.07622 = phi float [ %history.i.i433.sroa.29.0.copyload, %bb5.i.i492.lr.ph ], [ %history.i.i433.sroa.26.07621, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ]
  %history.i.i433.sroa.26.07621 = phi float [ %history.i.i433.sroa.26.0.copyload, %bb5.i.i492.lr.ph ], [ %history.i.i433.sroa.22.07620, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ]
  %history.i.i433.sroa.22.07620 = phi float [ %history.i.i433.sroa.22.0.copyload, %bb5.i.i492.lr.ph ], [ %history.i.i433.sroa.19.07619, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ]
  %history.i.i433.sroa.19.07619 = phi float [ %history.i.i433.sroa.19.0.copyload, %bb5.i.i492.lr.ph ], [ %history.i.i433.sroa.16.07618, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ]
  %history.i.i433.sroa.16.07618 = phi float [ %history.i.i433.sroa.16.0.copyload, %bb5.i.i492.lr.ph ], [ %history.i.i433.sroa.13.07617, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ]
  %history.i.i433.sroa.13.07617 = phi float [ %history.i.i433.sroa.13.0.copyload, %bb5.i.i492.lr.ph ], [ %history.i.i433.sroa.10.07616, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ]
  %history.i.i433.sroa.10.07616 = phi float [ %history.i.i433.sroa.10.0.copyload, %bb5.i.i492.lr.ph ], [ %history.i.i433.sroa.7.07615, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ]
  %history.i.i433.sroa.7.07615 = phi float [ %history.i.i433.sroa.7.0.copyload, %bb5.i.i492.lr.ph ], [ %history.i.i433.sroa.0.07614, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ]
  %history.i.i433.sroa.0.07614 = phi float [ %history.i.i433.sroa.0.0.copyload, %bb5.i.i492.lr.ph ], [ %_0.i2963, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ]
  %389 = add nuw nsw i32 %iter.sroa.0.0.i.i4557625, 1, !dbg !5644
  %_11.i.i493 = add nuw nsw i32 %iter.sroa.0.0.i.i4557625, %iter.sroa.0.0.i7945, !dbg !5647
  %_24.i.i494 = icmp ugt i32 %_11.i.i493, %right_io.1, !dbg !5648
  br i1 %_24.i.i494, label %bb7.i.i690, label %bb8.i.i495, !dbg !5648, !prof !755

bb8.i.i495:                                       ; preds = %bb5.i.i492
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5651), !dbg !5654
  %_3.not.i2961 = icmp eq i32 %right_io.1, %_11.i.i493, !dbg !5655
  br i1 %_3.not.i2961, label %panic.i2964, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965, !dbg !5655

panic.i2964:                                      ; preds = %bb8.i.i495
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #28, !dbg !5655, !noalias !5657
  unreachable, !dbg !5655

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965: ; preds = %bb8.i.i495
  %_31.i126.i = getelementptr inbounds nuw float, ptr %right_io.0, i32 %_11.i.i493, !dbg !5659
  %_0.i2963 = load float, ptr %_31.i126.i, align 4, !dbg !5655, !alias.scope !5651, !noalias !5661, !noundef !10
  %390 = tail call noundef float @llvm.fabs.f32(float %history.i.i433.sroa.19.07619), !dbg !5662
  %_0.i2612 = fmul float %_0.i2963, %_11.i.i.i.i511, !dbg !5665
  %_0.i2184 = fadd float %_0.i2612, 0.000000e+00, !dbg !5668
  %_0.i2611 = fmul float %_0.i2963, %_14.i.i.i.i514, !dbg !5670
  %_0.i2183 = fadd float %_0.i2611, 0.000000e+00, !dbg !5672
  %_0.i2610 = fmul float %_0.i2963, %_17.i.i.i.i517, !dbg !5674
  %_0.i2182 = fadd float %_0.i2610, 0.000000e+00, !dbg !5676
  %_0.i2609 = fmul float %_0.i2963, %_20.i.i.i.i520, !dbg !5678
  %_0.i2181 = fadd float %_0.i2609, 0.000000e+00, !dbg !5680
  %_0.i2608 = fmul float %history.i.i433.sroa.0.07614, %_25.i.i.i.i525, !dbg !5682
  %_0.i2180 = fadd float %_0.i2184, %_0.i2608, !dbg !5684
  %_0.i2607 = fmul float %history.i.i433.sroa.0.07614, %_28.i.i.i.i528, !dbg !5686
  %_0.i2179 = fadd float %_0.i2183, %_0.i2607, !dbg !5688
  %_0.i2606 = fmul float %history.i.i433.sroa.0.07614, %_31.i.i.i.i531, !dbg !5690
  %_0.i2178 = fadd float %_0.i2182, %_0.i2606, !dbg !5692
  %_0.i2605 = fmul float %history.i.i433.sroa.0.07614, %_34.i.i.i.i534, !dbg !5694
  %_0.i2177 = fadd float %_0.i2181, %_0.i2605, !dbg !5696
  %_0.i2604 = fmul float %history.i.i433.sroa.7.07615, %_39.i.i.i.i539, !dbg !5698
  %_0.i2176 = fadd float %_0.i2180, %_0.i2604, !dbg !5700
  %_0.i2603 = fmul float %history.i.i433.sroa.7.07615, %_42.i.i.i.i542, !dbg !5702
  %_0.i2175 = fadd float %_0.i2179, %_0.i2603, !dbg !5704
  %_0.i2602 = fmul float %history.i.i433.sroa.7.07615, %_45.i.i.i.i545, !dbg !5706
  %_0.i2174 = fadd float %_0.i2178, %_0.i2602, !dbg !5708
  %_0.i2601 = fmul float %history.i.i433.sroa.7.07615, %_48.i.i.i.i548, !dbg !5710
  %_0.i2173 = fadd float %_0.i2177, %_0.i2601, !dbg !5712
  %_0.i2600 = fmul float %history.i.i433.sroa.10.07616, %_53.i.i.i.i553, !dbg !5714
  %_0.i2172 = fadd float %_0.i2176, %_0.i2600, !dbg !5716
  %_0.i2599 = fmul float %history.i.i433.sroa.10.07616, %_56.i.i.i.i556, !dbg !5718
  %_0.i2171 = fadd float %_0.i2175, %_0.i2599, !dbg !5720
  %_0.i2598 = fmul float %history.i.i433.sroa.10.07616, %_59.i.i.i.i559, !dbg !5722
  %_0.i2170 = fadd float %_0.i2174, %_0.i2598, !dbg !5724
  %_0.i2597 = fmul float %history.i.i433.sroa.10.07616, %_62.i.i.i.i562, !dbg !5726
  %_0.i2169 = fadd float %_0.i2173, %_0.i2597, !dbg !5728
  %_0.i2596 = fmul float %history.i.i433.sroa.13.07617, %_67.i.i.i.i567, !dbg !5730
  %_0.i2168 = fadd float %_0.i2172, %_0.i2596, !dbg !5732
  %_0.i2595 = fmul float %history.i.i433.sroa.13.07617, %_70.i.i.i.i570, !dbg !5734
  %_0.i2167 = fadd float %_0.i2171, %_0.i2595, !dbg !5736
  %_0.i2594 = fmul float %history.i.i433.sroa.13.07617, %_73.i.i.i.i573, !dbg !5738
  %_0.i2166 = fadd float %_0.i2170, %_0.i2594, !dbg !5740
  %_0.i2593 = fmul float %history.i.i433.sroa.13.07617, %_76.i.i.i.i576, !dbg !5742
  %_0.i2165 = fadd float %_0.i2169, %_0.i2593, !dbg !5744
  %_0.i2592 = fmul float %history.i.i433.sroa.16.07618, %_81.i.i.i.i581, !dbg !5746
  %_0.i2164 = fadd float %_0.i2168, %_0.i2592, !dbg !5748
  %_0.i2591 = fmul float %history.i.i433.sroa.16.07618, %_84.i.i.i.i584, !dbg !5750
  %_0.i2163 = fadd float %_0.i2167, %_0.i2591, !dbg !5752
  %_0.i2590 = fmul float %history.i.i433.sroa.16.07618, %_87.i.i.i.i587, !dbg !5754
  %_0.i2162 = fadd float %_0.i2166, %_0.i2590, !dbg !5756
  %_0.i2589 = fmul float %history.i.i433.sroa.16.07618, %_90.i.i.i.i590, !dbg !5758
  %_0.i2161 = fadd float %_0.i2165, %_0.i2589, !dbg !5760
  %_0.i2588 = fmul float %history.i.i433.sroa.19.07619, %_95.i.i.i.i595, !dbg !5762
  %_0.i2160 = fadd float %_0.i2164, %_0.i2588, !dbg !5764
  %_0.i2587 = fmul float %history.i.i433.sroa.19.07619, %_98.i.i.i.i598, !dbg !5766
  %_0.i2159 = fadd float %_0.i2163, %_0.i2587, !dbg !5768
  %_0.i2586 = fmul float %history.i.i433.sroa.19.07619, %_101.i.i.i.i601, !dbg !5770
  %_0.i2158 = fadd float %_0.i2162, %_0.i2586, !dbg !5772
  %_0.i2585 = fmul float %history.i.i433.sroa.19.07619, %_104.i.i.i.i604, !dbg !5774
  %_0.i2157 = fadd float %_0.i2161, %_0.i2585, !dbg !5776
  %_0.i2584 = fmul float %history.i.i433.sroa.22.07620, %_109.i.i.i.i609, !dbg !5778
  %_0.i2156 = fadd float %_0.i2160, %_0.i2584, !dbg !5780
  %_0.i2583 = fmul float %history.i.i433.sroa.22.07620, %_112.i.i.i.i612, !dbg !5782
  %_0.i2155 = fadd float %_0.i2159, %_0.i2583, !dbg !5784
  %_0.i2582 = fmul float %history.i.i433.sroa.22.07620, %_115.i.i.i.i615, !dbg !5786
  %_0.i2154 = fadd float %_0.i2158, %_0.i2582, !dbg !5788
  %_0.i2581 = fmul float %history.i.i433.sroa.22.07620, %_118.i.i.i.i618, !dbg !5790
  %_0.i2153 = fadd float %_0.i2157, %_0.i2581, !dbg !5792
  %_0.i2580 = fmul float %history.i.i433.sroa.26.07621, %_123.i.i.i.i623, !dbg !5794
  %_0.i2152 = fadd float %_0.i2156, %_0.i2580, !dbg !5796
  %_0.i2579 = fmul float %history.i.i433.sroa.26.07621, %_126.i.i.i.i626, !dbg !5798
  %_0.i2151 = fadd float %_0.i2155, %_0.i2579, !dbg !5800
  %_0.i2578 = fmul float %history.i.i433.sroa.26.07621, %_129.i.i.i.i629, !dbg !5802
  %_0.i2150 = fadd float %_0.i2154, %_0.i2578, !dbg !5804
  %_0.i2577 = fmul float %history.i.i433.sroa.26.07621, %_132.i.i.i.i632, !dbg !5806
  %_0.i2149 = fadd float %_0.i2153, %_0.i2577, !dbg !5808
  %_0.i2576 = fmul float %history.i.i433.sroa.29.07622, %_137.i.i.i.i637, !dbg !5810
  %_0.i2148 = fadd float %_0.i2152, %_0.i2576, !dbg !5812
  %_0.i2575 = fmul float %history.i.i433.sroa.29.07622, %_140.i.i.i.i640, !dbg !5814
  %_0.i2147 = fadd float %_0.i2151, %_0.i2575, !dbg !5816
  %_0.i2574 = fmul float %history.i.i433.sroa.29.07622, %_143.i.i.i.i643, !dbg !5818
  %_0.i2146 = fadd float %_0.i2150, %_0.i2574, !dbg !5820
  %_0.i2573 = fmul float %history.i.i433.sroa.29.07622, %_146.i.i.i.i646, !dbg !5822
  %_0.i2145 = fadd float %_0.i2149, %_0.i2573, !dbg !5824
  %_0.i2572 = fmul float %history.i.i433.sroa.32.07623, %_151.i.i.i.i651, !dbg !5826
  %_0.i2144 = fadd float %_0.i2148, %_0.i2572, !dbg !5828
  %_0.i2571 = fmul float %history.i.i433.sroa.32.07623, %_154.i.i.i.i654, !dbg !5830
  %_0.i2143 = fadd float %_0.i2147, %_0.i2571, !dbg !5832
  %_0.i2570 = fmul float %history.i.i433.sroa.32.07623, %_157.i.i.i.i657, !dbg !5834
  %_0.i2142 = fadd float %_0.i2146, %_0.i2570, !dbg !5836
  %_0.i2569 = fmul float %history.i.i433.sroa.32.07623, %_160.i.i.i.i660, !dbg !5838
  %_0.i2141 = fadd float %_0.i2145, %_0.i2569, !dbg !5840
  %_0.i2568 = fmul float %history.i.i433.sroa.35.07624, %_165.i.i.i.i665, !dbg !5842
  %_0.i2140 = fadd float %_0.i2144, %_0.i2568, !dbg !5844
  %_0.i2567 = fmul float %history.i.i433.sroa.35.07624, %_168.i.i.i.i668, !dbg !5846
  %_0.i2139 = fadd float %_0.i2143, %_0.i2567, !dbg !5848
  %_0.i2566 = fmul float %history.i.i433.sroa.35.07624, %_171.i.i.i.i671, !dbg !5850
  %_0.i2138 = fadd float %_0.i2142, %_0.i2566, !dbg !5852
  %_0.i2565 = fmul float %history.i.i433.sroa.35.07624, %_174.i.i.i.i674, !dbg !5854
  %_0.i2137 = fadd float %_0.i2141, %_0.i2565, !dbg !5856
  %391 = tail call noundef float @llvm.fabs.f32(float %_0.i2140), !dbg !5858
  %_3.i.i3602.inv = fcmp ogt float %390, %391, !dbg !5860
  %_4.i.i3609.v = select i1 %_3.i.i3602.inv, float %390, float %391, !dbg !5860
  %392 = tail call noundef float @llvm.fabs.f32(float %_0.i2139), !dbg !5858
  %_3.i.i3602.inv.1 = fcmp ogt float %_4.i.i3609.v, %392, !dbg !5860
  %_4.i.i3609.v.1 = select i1 %_3.i.i3602.inv.1, float %_4.i.i3609.v, float %392, !dbg !5860
  %393 = tail call noundef float @llvm.fabs.f32(float %_0.i2138), !dbg !5858
  %_3.i.i3602.inv.2 = fcmp ogt float %_4.i.i3609.v.1, %393, !dbg !5860
  %_4.i.i3609.v.2 = select i1 %_3.i.i3602.inv.2, float %_4.i.i3609.v.1, float %393, !dbg !5860
  %394 = tail call noundef float @llvm.fabs.f32(float %_0.i2137), !dbg !5858
  %_3.i.i3602.inv.3 = fcmp ogt float %_4.i.i3609.v.2, %394, !dbg !5860
  %_4.i.i3609.v.3 = select i1 %_3.i.i3602.inv.3, float %_4.i.i3609.v.2, float %394, !dbg !5860
  %_39.i.i685 = getelementptr inbounds nuw float, ptr %peaks_right.i434, i32 %iter.sroa.0.0.i.i4557625, !dbg !5863
  store float %_4.i.i3609.v.3, ptr %_39.i.i685, align 4, !dbg !5868, !alias.scope !5870, !noalias !5661
  %exitcond11918.not = icmp eq i32 %389, %umax11930, !dbg !5873
  br i1 %exitcond11918.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i457, label %bb5.i.i492, !dbg !5641

bb7.i.i690:                                       ; preds = %bb5.i.i492
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_11.i.i493, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #28, !dbg !5875, !noalias !5661
  unreachable, !dbg !5875

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i457: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i
  %history.i.i433.sroa.0.0.lcssa = phi float [ %history.i.i433.sroa.0.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %_0.i2963, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ], !dbg !5876
  %history.i.i433.sroa.7.0.lcssa = phi float [ %history.i.i433.sroa.7.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i433.sroa.0.07614, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ], !dbg !5876
  %history.i.i433.sroa.10.0.lcssa = phi float [ %history.i.i433.sroa.10.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i433.sroa.7.07615, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ], !dbg !5876
  %history.i.i433.sroa.13.0.lcssa = phi float [ %history.i.i433.sroa.13.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i433.sroa.10.07616, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ], !dbg !5876
  %history.i.i433.sroa.16.0.lcssa = phi float [ %history.i.i433.sroa.16.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i433.sroa.13.07617, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ], !dbg !5876
  %history.i.i433.sroa.19.0.lcssa = phi float [ %history.i.i433.sroa.19.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i433.sroa.16.07618, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ], !dbg !5876
  %history.i.i433.sroa.22.0.lcssa = phi float [ %history.i.i433.sroa.22.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i433.sroa.19.07619, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ], !dbg !5876
  %history.i.i433.sroa.26.0.lcssa = phi float [ %history.i.i433.sroa.26.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i433.sroa.22.07620, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ], !dbg !5876
  %history.i.i433.sroa.29.0.lcssa = phi float [ %history.i.i433.sroa.29.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i433.sroa.26.07621, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ], !dbg !5876
  %history.i.i433.sroa.32.0.lcssa = phi float [ %history.i.i433.sroa.32.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i433.sroa.29.07622, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ], !dbg !5876
  %history.i.i433.sroa.35.0.lcssa = phi float [ %history.i.i433.sroa.35.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i433.sroa.32.07623, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ], !dbg !5876
  %history.i.i433.sroa.38.0.lcssa = phi float [ %history.i.i433.sroa.38.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit339.i ], [ %history.i.i433.sroa.35.07624, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2965 ], !dbg !5876
  store float %history.i.i433.sroa.0.0.lcssa, ptr %hot_right.i436, align 4, !dbg !5877, !noalias !5636
  store float %history.i.i433.sroa.7.0.lcssa, ptr %history.i.i433.sroa.7.0.hot_right.i436.sroa_idx, align 4, !dbg !5877, !noalias !5636
  store float %history.i.i433.sroa.10.0.lcssa, ptr %history.i.i433.sroa.10.0.hot_right.i436.sroa_idx, align 4, !dbg !5877, !noalias !5636
  store float %history.i.i433.sroa.13.0.lcssa, ptr %history.i.i433.sroa.13.0.hot_right.i436.sroa_idx, align 4, !dbg !5877, !noalias !5636
  store float %history.i.i433.sroa.16.0.lcssa, ptr %history.i.i433.sroa.16.0.hot_right.i436.sroa_idx, align 4, !dbg !5877, !noalias !5636
  store float %history.i.i433.sroa.19.0.lcssa, ptr %history.i.i433.sroa.19.0.hot_right.i436.sroa_idx, align 4, !dbg !5877, !noalias !5636
  store float %history.i.i433.sroa.22.0.lcssa, ptr %history.i.i433.sroa.22.0.hot_right.i436.sroa_idx, align 4, !dbg !5877, !noalias !5636
  store float %history.i.i433.sroa.26.0.lcssa, ptr %history.i.i433.sroa.26.0.hot_right.i436.sroa_idx, align 4, !dbg !5877, !noalias !5636
  store float %history.i.i433.sroa.29.0.lcssa, ptr %history.i.i433.sroa.29.0.hot_right.i436.sroa_idx, align 4, !dbg !5877, !noalias !5636
  store float %history.i.i433.sroa.32.0.lcssa, ptr %history.i.i433.sroa.32.0.hot_right.i436.sroa_idx, align 4, !dbg !5877, !noalias !5636
  store float %history.i.i433.sroa.35.0.lcssa, ptr %history.i.i433.sroa.35.0.hot_right.i436.sroa_idx, align 4, !dbg !5877, !noalias !5636
  store float %history.i.i433.sroa.38.0.lcssa, ptr %history.i.i433.sroa.38.0.hot_right.i436.sroa_idx, align 4, !dbg !5877, !noalias !5636
  br i1 %_20.i138.i7586.not, label %bb13.i.loopexit, label %bb42.i.lr.ph, !dbg !5377

bb42.i.lr.ph:                                     ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i457
  %_8.i31.i = load float, ptr %_64.i, align 4, !alias.scope !5878, !noalias !5881, !noundef !10
  %_9.i32.i = load float, ptr %_65.i462, align 4, !alias.scope !5883, !noalias !5884, !noundef !10
  %_8.i.i464 = load float, ptr %_69.i463, align 4, !alias.scope !5885, !noalias !5888, !noundef !10
  %_9.i.i465 = load float, ptr %_70.i, align 4, !alias.scope !5890, !noalias !5891, !noundef !10
  %_87.i = load i32, ptr %348, align 4
  %_62.i89.i = load float, ptr %359, align 4
  %_62.i.i486 = load float, ptr %375, align 4
  %_102.i = load i32, ptr %379, align 4
  %.promoted7668 = load float, ptr %358, align 4
  %.promoted7737 = load float, ptr %360, align 4
  %.promoted7806 = load float, ptr %374, align 4
  %.promoted7875 = load float, ptr %376, align 4
  br label %bb42.i, !dbg !5377

bb42.i:                                           ; preds = %bb42.i.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3117
  %_0.i32167876 = phi float [ %.promoted7875, %bb42.i.lr.ph ], [ %_0.i3216, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3117 ]
  %_0.i28427807 = phi float [ %.promoted7806, %bb42.i.lr.ph ], [ %_0.i2842, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3117 ]
  %_0.i32207738 = phi float [ %.promoted7737, %bb42.i.lr.ph ], [ %_0.i3220, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3117 ]
  %_0.i28467669 = phi float [ %.promoted7668, %bb42.i.lr.ph ], [ %_0.i2846, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3117 ]
  %main_cursor.sroa.0.1.i4607665 = phi i32 [ %main_cursor.sroa.0.0.i4527948, %bb42.i.lr.ph ], [ %spec.store.select11.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3117 ]
  %ring_cursor.sroa.0.1.i4597664 = phi i32 [ %ring_cursor.sroa.0.0.i4517947, %bb42.i.lr.ph ], [ %spec.store.select12.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3117 ]
  %iter1.sroa.0.0.i4587663 = phi i32 [ 0, %bb42.i.lr.ph ], [ %395, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3117 ]
  %395 = add nuw nsw i32 %iter1.sroa.0.0.i4587663, 1, !dbg !5892
  %_60.i = add nuw nsw i32 %iter1.sroa.0.0.i4587663, %iter.sroa.0.0.i7945, !dbg !5898
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5878), !dbg !5899
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5883), !dbg !5899
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5885), !dbg !5900
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5890), !dbg !5900
  %_126.i = getelementptr inbounds nuw float, ptr %peaks_left.i435, i32 %iter1.sroa.0.0.i4587663, !dbg !5901
  %_0.i3001 = load float, ptr %_126.i, align 4, !dbg !5912, !alias.scope !5914, !noalias !5917, !noundef !10
  %_131.i467 = getelementptr inbounds nuw float, ptr %peaks_right.i434, i32 %iter1.sroa.0.0.i4587663, !dbg !5918
  %_0.i2996 = load float, ptr %_131.i467, align 4, !dbg !5928, !alias.scope !5930, !noalias !5917, !noundef !10
  %_3.i.i3629 = fcmp ule float %_0.i2996, %_0.i3001, !dbg !5933
  %_6.i.i3631 = bitcast float %_0.i2996 to i32, !dbg !5936
  %_8.i.i3633 = bitcast float %_0.i3001 to i32, !dbg !5939
  %_4.i.i3636 = select i1 %_3.i.i3629, i32 %_8.i.i3633, i32 %_6.i.i3631, !dbg !5941
  %_5.i3412 = and i32 %_4.i.i3636, %.none.i442, !dbg !5942
  %_7.i3415 = and i32 %_9.i3414, %_8.i.i3633, !dbg !5944
  %_4.i3416 = or disjoint i32 %_5.i3412, %_7.i3415, !dbg !5942
  %_0.i3417 = bitcast i32 %_4.i3416 to float, !dbg !5945
  %_7.i3408 = and i32 %_9.i3414, %_6.i.i3631, !dbg !5947
  %_4.i3409 = or disjoint i32 %_5.i3412, %_7.i3408, !dbg !5949
  %_0.i3410 = bitcast i32 %_4.i3409 to float, !dbg !5950
  %_132.i = icmp ugt i32 %_60.i, %left_io.1, !dbg !5952
  br i1 %_132.i, label %bb46.i, label %bb47.i, !dbg !5952, !prof !755

bb47.i:                                           ; preds = %bb42.i
  %_139.i470 = getelementptr inbounds nuw float, ptr %left_io.0, i32 %_60.i, !dbg !5956
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5961), !dbg !5964
  %_3.not.i2989 = icmp eq i32 %left_io.1, %_60.i, !dbg !5965
  br i1 %_3.not.i2989, label %panic.i2992, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2993, !dbg !5965

panic.i2992:                                      ; preds = %bb47.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #28, !dbg !5965, !noalias !5967
  unreachable, !dbg !5965

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2993: ; preds = %bb47.i
  %_0.i2991 = load float, ptr %_139.i470, align 4, !dbg !5965, !alias.scope !5961, !noalias !5917, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5968), !dbg !5971
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5972), !dbg !5971
  %width.i33.i = load i32, ptr %349, align 4, !dbg !5974, !alias.scope !5975, !noalias !5976, !noundef !10
  %_3.i1957 = fcmp uge float %_8.i31.i, %_0.i3417, !dbg !5979
  %_0.i2390 = fdiv float %_8.i31.i, %_0.i3417, !dbg !5981
  %_0.i3403 = select i1 %_3.i1957, float 1.000000e+00, float %_0.i2390, !dbg !5983
  %_158.1.i38.i = load i32, ptr %350, align 4, !dbg !5985, !alias.scope !5975, !noalias !5976, !noundef !10
  %_22.i39.i = mul i32 %width.i33.i, %ring_cursor.sroa.0.1.i4597664, !dbg !5986
  %_90.i40.i = icmp ugt i32 %_22.i39.i, %_158.1.i38.i, !dbg !5987
  br i1 %_90.i40.i, label %bb34.i124.i, label %bb35.i41.i, !dbg !5987, !prof !755

bb35.i41.i:                                       ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2993
  %_158.0.i42.i = load ptr, ptr %351, align 4, !dbg !5985, !alias.scope !5975, !noalias !5976, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5990), !dbg !5993
  %_4.not.i3142 = icmp eq i32 %_158.1.i38.i, %_22.i39.i, !dbg !5994
  br i1 %_4.not.i3142, label %panic.i3144, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3145, !dbg !5994

panic.i3144:                                      ; preds = %bb35.i41.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #28, !dbg !5994, !noalias !5996
  unreachable, !dbg !5994

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3145: ; preds = %bb35.i41.i
  %_97.i44.i = getelementptr inbounds nuw float, ptr %_158.0.i42.i, i32 %_22.i39.i, !dbg !5997
  store float %_0.i3403, ptr %_97.i44.i, align 4, !dbg !5994, !alias.scope !5990, !noalias !5999
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6000), !dbg !6003
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6004), !dbg !6003
  %width.i1689 = load i32, ptr %349, align 4, !dbg !6006, !alias.scope !6000, !noalias !6008, !noundef !10
  %396 = icmp eq i32 %width.i1689, 0, !dbg !6009
  br i1 %396, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1794, label %bb29.i1695.lr.ph, !dbg !6009

bb29.i1695.lr.ph:                                 ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3145
  %_126.1.i1700 = load i32, ptr %62, align 4, !alias.scope !6000, !noalias !6008, !noundef !10
  %_126.0.i1704 = load ptr, ptr %61, align 4, !nonnull !10
  %397 = add i32 %ring_cursor.sroa.0.1.i4597664, 1
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
  br label %bb29.i1695, !dbg !6009

bb29.i1695:                                       ; preds = %bb29.i1695.lr.ph, %bb28.i1757
  %iter.sroa.0.0.idx.i16937644 = phi i32 [ 0, %bb29.i1695.lr.ph ], [ %iter.sroa.0.0.add.i1698, %bb28.i1757 ]
  %iter.sroa.4.0.i16927643 = phi i32 [ 0, %bb29.i1695.lr.ph ], [ %_102.0.i1699, %bb28.i1757 ]
  %iter.sroa.7.0.i16917642 = phi i32 [ %width.i1689, %bb29.i1695.lr.ph ], [ %399, %bb28.i1757 ]
  %iter.sroa.0.0.ptr.i16947645 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 %iter.sroa.0.0.idx.i16937644, !dbg !6011
  %399 = add i32 %iter.sroa.7.0.i16917642, -1, !dbg !6011
  %_109.i1696 = icmp eq i32 %iter.sroa.0.0.idx.i16937644, 32, !dbg !6012
  br i1 %_109.i1696, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1794, label %bb33.i1697, !dbg !6016

bb33.i1697:                                       ; preds = %bb29.i1695
  %iter.sroa.0.0.add.i1698 = add nuw nsw i32 %iter.sroa.0.0.idx.i16937644, 4, !dbg !6017
  %_102.0.i1699 = add nuw nsw i32 %iter.sroa.4.0.i16927643, 1, !dbg !6019
  %exitcond11920.not = icmp eq i32 %iter.sroa.4.0.i16927643, %_126.1.i1700, !dbg !6020
  br i1 %exitcond11920.not, label %panic.i1702, label %bb2.i1703, !dbg !6020

bb2.i1703:                                        ; preds = %bb33.i1697
  %400 = getelementptr inbounds nuw %LaneShape, ptr %_126.0.i1704, i32 %iter.sroa.4.0.i16927643, !dbg !6020
  %shape.i1705 = load i32, ptr %400, align 4, !dbg !6020, !noalias !6021, !noundef !10
  %401 = getelementptr inbounds nuw i8, ptr %400, i32 4, !dbg !6020
  %shape3.i1706 = load i32, ptr %401, align 4, !dbg !6020, !noalias !6021, !noundef !10
  %402 = add i32 %shape3.i1706, %ring_cursor.sroa.0.1.i4597664, !dbg !6022
  %_18.not.i1707 = icmp ult i32 %402, %_87.i, !dbg !6023
  %403 = select i1 %_18.not.i1707, i32 0, i32 %_87.i, !dbg !6023
  %spec.select.i1708 = sub nuw i32 %402, %403, !dbg !6023
  %_25.i1711 = mul i32 %spec.select.i1708, %width.i1689, !dbg !6024
  %_24.i1712 = add i32 %_25.i1711, %iter.sroa.4.0.i16927643, !dbg !6024
  %_28.i1714 = icmp ult i32 %_24.i1712, %_128.1.i1713, !dbg !6025
  br i1 %_28.i1714, label %bb9.i1716, label %panic5.i1715, !dbg !6025

panic.i1702:                                      ; preds = %bb33.i1697
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_126.1.i1700, i32 noundef %_126.1.i1700, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_12856b5f9033764dbf23e04eb77c5c46) #28, !dbg !6020, !noalias !6021
  unreachable, !dbg !6020

bb9.i1716:                                        ; preds = %bb2.i1703
  %404 = getelementptr inbounds nuw float, ptr %_128.0.i1717, i32 %_24.i1712, !dbg !6025
  %405 = load float, ptr %404, align 4, !dbg !6025, !noalias !6021, !noundef !10
  %exitcond11921.not = icmp eq i32 %iter.sroa.4.0.i16927643, %_130.1.i1718, !dbg !6026
  br i1 %exitcond11921.not, label %panic6.i1720, label %bb10.i1721, !dbg !6026

panic5.i1715:                                     ; preds = %bb2.i1703
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_24.i1712, i32 noundef %_128.1.i1713, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70ebf0cf1c90c6161926611ccf249a32) #28, !dbg !6025, !noalias !6021
  unreachable, !dbg !6025

bb10.i1721:                                       ; preds = %bb9.i1716
  %406 = getelementptr inbounds nuw i32, ptr %_130.0.i1722, i32 %iter.sroa.4.0.i16927643, !dbg !6026
  %_30.i1723 = load i32, ptr %406, align 4, !dbg !6026, !noalias !6021, !noundef !10
  %407 = icmp eq i32 %_30.i1723, 0, !dbg !6027
  br i1 %407, label %bb14.i1732, label %bb12.i1724, !dbg !6027

panic6.i1720:                                     ; preds = %bb9.i1716
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_130.1.i1718, i32 noundef %_130.1.i1718, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_023d591aaad7f5cf482ac80a9401eca1) #28, !dbg !6026, !noalias !6021
  unreachable, !dbg !6026

bb12.i1724:                                       ; preds = %bb10.i1721
  %_35.i1726 = icmp ult i32 %iter.sroa.4.0.i16927643, %_132.1.i1725, !dbg !6028
  br i1 %_35.i1726, label %bb13.i1728, label %panic7.i1727, !dbg !6028

bb14.i1732:                                       ; preds = %bb34.i1793, %bb13.i1728, %bb10.i1721
  %newest.sroa.0.0.i1733 = phi float [ %405, %bb10.i1721 ], [ %_33.i1730, %bb34.i1793 ], [ %405, %bb13.i1728 ], !dbg !6029
  %exitcond11922.not = icmp eq i32 %iter.sroa.4.0.i16927643, %_132.1.i1725, !dbg !6030
  br i1 %exitcond11922.not, label %panic8.i1736, label %bb15.i1737, !dbg !6030

bb13.i1728:                                       ; preds = %bb12.i1724
  %408 = getelementptr inbounds nuw float, ptr %_132.0.i1729, i32 %iter.sroa.4.0.i16927643, !dbg !6028
  %_33.i1730 = load float, ptr %408, align 4, !dbg !6028, !noalias !6021, !noundef !10
  %_116.i1731 = fcmp olt float %_33.i1730, %405, !dbg !6031
  br i1 %_116.i1731, label %bb34.i1793, label %bb14.i1732, !dbg !6031

panic7.i1727:                                     ; preds = %bb12.i1724
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %iter.sroa.4.0.i16927643, i32 noundef %_132.1.i1725, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a228cac3279aae3a34886f59f51fbe9e) #28, !dbg !6028, !noalias !6021
  unreachable, !dbg !6028

bb34.i1793:                                       ; preds = %bb13.i1728
  br label %bb14.i1732, !dbg !6033

bb15.i1737:                                       ; preds = %bb14.i1732
  %409 = getelementptr inbounds nuw float, ptr %_132.0.i1729, i32 %iter.sroa.4.0.i16927643, !dbg !6030
  store float %newest.sroa.0.0.i1733, ptr %409, align 4, !dbg !6030, !noalias !6021
  %_40.i1739 = add i32 %_30.i1723, 1, !dbg !6034
  %complete.i1740 = icmp eq i32 %_40.i1739, %shape.i1705, !dbg !6034
  br i1 %complete.i1740, label %bb19.i1762, label %bb17.i1741, !dbg !6035

panic8.i1736:                                     ; preds = %bb14.i1732
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_132.1.i1725, i32 noundef %_132.1.i1725, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1fa5d88c1a2cc292a2767c48606a38fe) #28, !dbg !6030, !noalias !6021
  unreachable, !dbg !6030

bb17.i1741:                                       ; preds = %bb15.i1737
  %_42.i1743 = add i32 %iter.sroa.4.0.i16927643, %_43.i1742, !dbg !6036
  %_45.i1745 = icmp ult i32 %_42.i1743, %_128.1.i1713, !dbg !6037
  br i1 %_45.i1745, label %bb27.i1755, label %panic9.i1746, !dbg !6037

panic9.i1746:                                     ; preds = %bb17.i1741
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_42.i1743, i32 noundef %_128.1.i1713, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70d40ae2302f3ea19fa0c4a65fe82786) #28, !dbg !6037, !noalias !6021
  unreachable, !dbg !6037

bb27.i1755:                                       ; preds = %bb17.i1741
  %410 = getelementptr inbounds nuw float, ptr %_128.0.i1717, i32 %_42.i1743, !dbg !6037
  %_41.i1749 = load float, ptr %410, align 4, !dbg !6037, !noalias !6021, !noundef !10
  %_117.i1750 = fcmp olt float %_41.i1749, %newest.sroa.0.0.i1733, !dbg !6038
  %newest.sroa.0.1.i1751 = select i1 %_117.i1750, float %_41.i1749, float %newest.sroa.0.0.i1733, !dbg !6038
  store float %newest.sroa.0.1.i1751, ptr %iter.sroa.0.0.ptr.i16947645, align 4, !dbg !6040, !alias.scope !6004, !noalias !6041
  br label %bb28.i1757, !dbg !6042

bb28.i1757:                                       ; preds = %bb22.i1790, %bb19.i1762, %bb27.i1755
  %storemerge4522 = phi i32 [ %_40.i1739, %bb27.i1755 ], [ 0, %bb19.i1762 ], [ 0, %bb22.i1790 ], !dbg !6043
  store i32 %storemerge4522, ptr %406, align 4, !dbg !6043, !noalias !6021
  %411 = icmp eq i32 %399, 0, !dbg !6009
  br i1 %411, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1794, label %bb29.i1695, !dbg !6009

bb19.i1762:                                       ; preds = %bb15.i1737
  store float %newest.sroa.0.0.i1733, ptr %iter.sroa.0.0.ptr.i16947645, align 4, !dbg !6040, !alias.scope !6004, !noalias !6041
  %_118.i17687638.not = icmp eq i32 %shape.i1705, 0, !dbg !6044
  br i1 %_118.i17687638.not, label %bb28.i1757, label %bb40.i1775.preheader, !dbg !6048

bb40.i1775.preheader:                             ; preds = %bb19.i1762
  %412 = load float, ptr %404, align 4, !dbg !6049, !noalias !6021, !noundef !10
  br label %bb40.i1775, !dbg !6050

bb40.i1775:                                       ; preds = %bb40.i1775.preheader, %bb22.i1790
  %iter2.sroa.0.0.i17677641 = phi i32 [ %_119.i1776, %bb22.i1790 ], [ 0, %bb40.i1775.preheader ]
  %suffix.sroa.0.0.i17667640 = phi float [ %suffix.sroa.0.1.i1786, %bb22.i1790 ], [ %412, %bb40.i1775.preheader ]
  %end.sroa.0.1.i17657639 = phi i32 [ %415, %bb22.i1790 ], [ %spec.select.i1708, %bb40.i1775.preheader ]
  %_54.i1777 = mul i32 %end.sroa.0.1.i17657639, %width.i1689, !dbg !6051
  %_53.i1778 = add i32 %_54.i1777, %iter.sroa.4.0.i16927643, !dbg !6051
  %_57.i1780 = icmp ult i32 %_53.i1778, %_128.1.i1713, !dbg !6050
  br i1 %_57.i1780, label %bb22.i1790, label %panic13.i1781, !dbg !6050

panic13.i1781:                                    ; preds = %bb40.i1775
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_53.i1778, i32 noundef %_128.1.i1713, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a271bbb7fd83c3605ea58a06b7065fa4) #28, !dbg !6050, !noalias !6021
  unreachable, !dbg !6050

bb22.i1790:                                       ; preds = %bb40.i1775
  %_119.i1776 = add nuw i32 %iter2.sroa.0.0.i17677641, 1, !dbg !6052
  %413 = getelementptr inbounds nuw float, ptr %_128.0.i1717, i32 %_53.i1778, !dbg !6050
  %_52.i1784 = load float, ptr %413, align 4, !dbg !6050, !noalias !6021, !noundef !10
  %_121.i1785 = fcmp olt float %suffix.sroa.0.0.i17667640, %_52.i1784, !dbg !6055
  %suffix.sroa.0.1.i1786 = select i1 %_121.i1785, float %suffix.sroa.0.0.i17667640, float %_52.i1784, !dbg !6055
  store float %suffix.sroa.0.1.i1786, ptr %413, align 4, !dbg !6057, !noalias !6021
  %414 = icmp eq i32 %end.sroa.0.1.i17657639, 0, !dbg !6058
  %spec.store.select.i1792 = select i1 %414, i32 %_87.i, i32 %end.sroa.0.1.i17657639, !dbg !6058
  %415 = add i32 %spec.store.select.i1792, -1, !dbg !6059
  %exitcond11919.not = icmp eq i32 %_119.i1776, %shape.i1705, !dbg !6044
  br i1 %exitcond11919.not, label %bb28.i1757, label %bb40.i1775, !dbg !6048

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1794: ; preds = %bb29.i1695, %bb28.i1757, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3145
  %_0.i2988 = load float, ptr %scratch.i, align 4, !dbg !6060, !alias.scope !6062, !noalias !6065, !noundef !10
  %_0.i2618 = fmul float %_0.i2988, 1.638400e+04, !dbg !6066
  %416 = tail call noundef float @llvm.floor.f32(float %_0.i2618), !dbg !6068
  %_0.i2617 = fmul float %416, 0x3F10000000000000, !dbg !6072
  %417 = icmp eq i32 %width.i33.i, 0, !dbg !6074
  %_163.1.i82.i.pre = load i32, ptr %356, align 4, !dbg !6076, !alias.scope !5975, !noalias !5976
  br i1 %417, label %bb53.i77.i, label %bb36.i56.i.lr.ph, !dbg !6074

bb36.i56.i.lr.ph:                                 ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1794
  %_159.1.i61.i = load i32, ptr %62, align 4, !alias.scope !5975, !noalias !5976, !noundef !10
  %_159.0.i65.i = load ptr, ptr %61, align 4, !nonnull !10
  %_161.0.i75.i = load ptr, ptr %357, align 4, !nonnull !10
  %exitcond11923.not = icmp eq i32 %_159.1.i61.i, 0, !dbg !6077
  br i1 %exitcond11923.not, label %panic.i63.i, label %bb14.i64.i, !dbg !6077

bb34.i124.i:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2993
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i39.i, i32 noundef %_158.1.i38.i, i32 noundef %_158.1.i38.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_db7c42041d7aaaba4839906f37294547) #28, !dbg !6078, !noalias !5999
  unreachable, !dbg !6078

bb53.i77.i:                                       ; preds = %bb18.i74.i.7, %bb18.i74.i, %bb18.i74.i.1, %bb18.i74.i.2, %bb18.i74.i.3, %bb18.i74.i.4, %bb18.i74.i.5, %bb18.i74.i.6, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1794
  %_0.i2986 = phi float [ %_0.i2988, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1794 ], [ %_47.i76.i, %bb18.i74.i ], [ %_47.i76.i, %bb18.i74.i.7 ], [ %_47.i76.i, %bb18.i74.i.6 ], [ %_47.i76.i, %bb18.i74.i.5 ], [ %_47.i76.i, %bb18.i74.i.4 ], [ %_47.i76.i, %bb18.i74.i.3 ], [ %_47.i76.i, %bb18.i74.i.2 ], [ %_47.i76.i, %bb18.i74.i.1 ], !dbg !6079
  %_0.i2186 = fadd float %_0.i2617, %_0.i28467669, !dbg !6081
  %_0.i2846 = fsub float %_0.i2186, %_0.i2986, !dbg !6083
  %_123.i83.i = icmp ugt i32 %_22.i39.i, %_163.1.i82.i.pre, !dbg !6085
  br i1 %_123.i83.i, label %bb41.i123.i, label %bb42.i84.i, !dbg !6085, !prof !755

bb14.i64.i:                                       ; preds = %bb36.i56.i.lr.ph
  %418 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i, i32 8, !dbg !6077
  %_42.i66.i = load i32, ptr %418, align 4, !dbg !6077, !noalias !6065, !noundef !10
  %419 = add i32 %_42.i66.i, %ring_cursor.sroa.0.1.i4597664, !dbg !6088
  %_45.not.i67.i = icmp ult i32 %419, %_87.i, !dbg !6089
  %420 = select i1 %_45.not.i67.i, i32 0, i32 %_87.i, !dbg !6089
  %spec.select.i68.i = sub nuw i32 %419, %420, !dbg !6089
  %_49.i69.i = mul i32 %spec.select.i68.i, %width.i33.i, !dbg !6090
  %_51.i72.i = icmp ult i32 %_49.i69.i, %_163.1.i82.i.pre, !dbg !6091
  br i1 %_51.i72.i, label %bb18.i74.i, label %panic1.i73.i, !dbg !6091

panic.i63.i:                                      ; preds = %bb36.i56.i.7, %bb36.i56.i.6, %bb36.i56.i.5, %bb36.i56.i.4, %bb36.i56.i.3, %bb36.i56.i.2, %bb36.i56.i.1, %bb36.i56.i.lr.ph
  %_159.1.i61.i.lcssa.ph = phi i32 [ 7, %bb36.i56.i.7 ], [ 6, %bb36.i56.i.6 ], [ 5, %bb36.i56.i.5 ], [ 4, %bb36.i56.i.4 ], [ 3, %bb36.i56.i.3 ], [ 2, %bb36.i56.i.2 ], [ 1, %bb36.i56.i.1 ], [ 0, %bb36.i56.i.lr.ph ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_159.1.i61.i.lcssa.ph, i32 noundef %_159.1.i61.i.lcssa.ph, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_33746731a929bad63709ddedbca82f5f) #28, !dbg !6077, !noalias !6065
  unreachable, !dbg !6077

bb18.i74.i:                                       ; preds = %bb14.i64.i
  %421 = getelementptr inbounds nuw float, ptr %_161.0.i75.i, i32 %_49.i69.i, !dbg !6091
  %_47.i76.i = load float, ptr %421, align 4, !dbg !6091, !noalias !6065, !noundef !10
  store float %_47.i76.i, ptr %scratch.i, align 4, !dbg !6092, !alias.scope !5972, !noalias !6093
  %422 = icmp eq i32 %width.i33.i, 1, !dbg !6074
  br i1 %422, label %bb53.i77.i, label %bb36.i56.i.1, !dbg !6074

bb36.i56.i.1:                                     ; preds = %bb18.i74.i
  %exitcond11923.1.not = icmp eq i32 %_159.1.i61.i, 1, !dbg !6077
  br i1 %exitcond11923.1.not, label %panic.i63.i, label %bb14.i64.i.1, !dbg !6077

bb14.i64.i.1:                                     ; preds = %bb36.i56.i.1
  %423 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i, i32 20, !dbg !6077
  %_42.i66.i.1 = load i32, ptr %423, align 4, !dbg !6077, !noalias !6065, !noundef !10
  %424 = add i32 %_42.i66.i.1, %ring_cursor.sroa.0.1.i4597664, !dbg !6088
  %_45.not.i67.i.1 = icmp ult i32 %424, %_87.i, !dbg !6089
  %425 = select i1 %_45.not.i67.i.1, i32 0, i32 %_87.i, !dbg !6089
  %spec.select.i68.i.1 = sub nuw i32 %424, %425, !dbg !6089
  %_49.i69.i.1 = mul i32 %spec.select.i68.i.1, %width.i33.i, !dbg !6090
  %_48.i70.i.1 = add i32 %_49.i69.i.1, 1, !dbg !6090
  %_51.i72.i.1 = icmp ult i32 %_48.i70.i.1, %_163.1.i82.i.pre, !dbg !6091
  br i1 %_51.i72.i.1, label %bb18.i74.i.1, label %panic1.i73.i, !dbg !6091

bb18.i74.i.1:                                     ; preds = %bb14.i64.i.1
  %426 = getelementptr inbounds nuw float, ptr %_161.0.i75.i, i32 %_48.i70.i.1, !dbg !6091
  %_47.i76.i.1 = load float, ptr %426, align 4, !dbg !6091, !noalias !6065, !noundef !10
  store float %_47.i76.i.1, ptr %iter.sroa.0.0.ptr.i55.i7649.1, align 4, !dbg !6092, !alias.scope !5972, !noalias !6093
  %427 = icmp eq i32 %width.i33.i, 2, !dbg !6074
  br i1 %427, label %bb53.i77.i, label %bb36.i56.i.2, !dbg !6074

bb36.i56.i.2:                                     ; preds = %bb18.i74.i.1
  %exitcond11923.2.not = icmp eq i32 %_159.1.i61.i, 2, !dbg !6077
  br i1 %exitcond11923.2.not, label %panic.i63.i, label %bb14.i64.i.2, !dbg !6077

bb14.i64.i.2:                                     ; preds = %bb36.i56.i.2
  %428 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i, i32 32, !dbg !6077
  %_42.i66.i.2 = load i32, ptr %428, align 4, !dbg !6077, !noalias !6065, !noundef !10
  %429 = add i32 %_42.i66.i.2, %ring_cursor.sroa.0.1.i4597664, !dbg !6088
  %_45.not.i67.i.2 = icmp ult i32 %429, %_87.i, !dbg !6089
  %430 = select i1 %_45.not.i67.i.2, i32 0, i32 %_87.i, !dbg !6089
  %spec.select.i68.i.2 = sub nuw i32 %429, %430, !dbg !6089
  %_49.i69.i.2 = mul i32 %spec.select.i68.i.2, %width.i33.i, !dbg !6090
  %_48.i70.i.2 = add i32 %_49.i69.i.2, 2, !dbg !6090
  %_51.i72.i.2 = icmp ult i32 %_48.i70.i.2, %_163.1.i82.i.pre, !dbg !6091
  br i1 %_51.i72.i.2, label %bb18.i74.i.2, label %panic1.i73.i, !dbg !6091

bb18.i74.i.2:                                     ; preds = %bb14.i64.i.2
  %431 = getelementptr inbounds nuw float, ptr %_161.0.i75.i, i32 %_48.i70.i.2, !dbg !6091
  %_47.i76.i.2 = load float, ptr %431, align 4, !dbg !6091, !noalias !6065, !noundef !10
  store float %_47.i76.i.2, ptr %iter.sroa.0.0.ptr.i55.i7649.2, align 4, !dbg !6092, !alias.scope !5972, !noalias !6093
  %432 = icmp eq i32 %width.i33.i, 3, !dbg !6074
  br i1 %432, label %bb53.i77.i, label %bb36.i56.i.3, !dbg !6074

bb36.i56.i.3:                                     ; preds = %bb18.i74.i.2
  %exitcond11923.3.not = icmp eq i32 %_159.1.i61.i, 3, !dbg !6077
  br i1 %exitcond11923.3.not, label %panic.i63.i, label %bb14.i64.i.3, !dbg !6077

bb14.i64.i.3:                                     ; preds = %bb36.i56.i.3
  %433 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i, i32 44, !dbg !6077
  %_42.i66.i.3 = load i32, ptr %433, align 4, !dbg !6077, !noalias !6065, !noundef !10
  %434 = add i32 %_42.i66.i.3, %ring_cursor.sroa.0.1.i4597664, !dbg !6088
  %_45.not.i67.i.3 = icmp ult i32 %434, %_87.i, !dbg !6089
  %435 = select i1 %_45.not.i67.i.3, i32 0, i32 %_87.i, !dbg !6089
  %spec.select.i68.i.3 = sub nuw i32 %434, %435, !dbg !6089
  %_49.i69.i.3 = mul i32 %spec.select.i68.i.3, %width.i33.i, !dbg !6090
  %_48.i70.i.3 = add i32 %_49.i69.i.3, 3, !dbg !6090
  %_51.i72.i.3 = icmp ult i32 %_48.i70.i.3, %_163.1.i82.i.pre, !dbg !6091
  br i1 %_51.i72.i.3, label %bb18.i74.i.3, label %panic1.i73.i, !dbg !6091

bb18.i74.i.3:                                     ; preds = %bb14.i64.i.3
  %436 = getelementptr inbounds nuw float, ptr %_161.0.i75.i, i32 %_48.i70.i.3, !dbg !6091
  %_47.i76.i.3 = load float, ptr %436, align 4, !dbg !6091, !noalias !6065, !noundef !10
  store float %_47.i76.i.3, ptr %iter.sroa.0.0.ptr.i55.i7649.3, align 4, !dbg !6092, !alias.scope !5972, !noalias !6093
  %437 = icmp eq i32 %width.i33.i, 4, !dbg !6074
  br i1 %437, label %bb53.i77.i, label %bb36.i56.i.4, !dbg !6074

bb36.i56.i.4:                                     ; preds = %bb18.i74.i.3
  %exitcond11923.4.not = icmp eq i32 %_159.1.i61.i, 4, !dbg !6077
  br i1 %exitcond11923.4.not, label %panic.i63.i, label %bb14.i64.i.4, !dbg !6077

bb14.i64.i.4:                                     ; preds = %bb36.i56.i.4
  %438 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i, i32 56, !dbg !6077
  %_42.i66.i.4 = load i32, ptr %438, align 4, !dbg !6077, !noalias !6065, !noundef !10
  %439 = add i32 %_42.i66.i.4, %ring_cursor.sroa.0.1.i4597664, !dbg !6088
  %_45.not.i67.i.4 = icmp ult i32 %439, %_87.i, !dbg !6089
  %440 = select i1 %_45.not.i67.i.4, i32 0, i32 %_87.i, !dbg !6089
  %spec.select.i68.i.4 = sub nuw i32 %439, %440, !dbg !6089
  %_49.i69.i.4 = mul i32 %spec.select.i68.i.4, %width.i33.i, !dbg !6090
  %_48.i70.i.4 = add i32 %_49.i69.i.4, 4, !dbg !6090
  %_51.i72.i.4 = icmp ult i32 %_48.i70.i.4, %_163.1.i82.i.pre, !dbg !6091
  br i1 %_51.i72.i.4, label %bb18.i74.i.4, label %panic1.i73.i, !dbg !6091

bb18.i74.i.4:                                     ; preds = %bb14.i64.i.4
  %441 = getelementptr inbounds nuw float, ptr %_161.0.i75.i, i32 %_48.i70.i.4, !dbg !6091
  %_47.i76.i.4 = load float, ptr %441, align 4, !dbg !6091, !noalias !6065, !noundef !10
  store float %_47.i76.i.4, ptr %iter.sroa.0.0.ptr.i55.i7649.4, align 4, !dbg !6092, !alias.scope !5972, !noalias !6093
  %442 = icmp eq i32 %width.i33.i, 5, !dbg !6074
  br i1 %442, label %bb53.i77.i, label %bb36.i56.i.5, !dbg !6074

bb36.i56.i.5:                                     ; preds = %bb18.i74.i.4
  %exitcond11923.5.not = icmp eq i32 %_159.1.i61.i, 5, !dbg !6077
  br i1 %exitcond11923.5.not, label %panic.i63.i, label %bb14.i64.i.5, !dbg !6077

bb14.i64.i.5:                                     ; preds = %bb36.i56.i.5
  %443 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i, i32 68, !dbg !6077
  %_42.i66.i.5 = load i32, ptr %443, align 4, !dbg !6077, !noalias !6065, !noundef !10
  %444 = add i32 %_42.i66.i.5, %ring_cursor.sroa.0.1.i4597664, !dbg !6088
  %_45.not.i67.i.5 = icmp ult i32 %444, %_87.i, !dbg !6089
  %445 = select i1 %_45.not.i67.i.5, i32 0, i32 %_87.i, !dbg !6089
  %spec.select.i68.i.5 = sub nuw i32 %444, %445, !dbg !6089
  %_49.i69.i.5 = mul i32 %spec.select.i68.i.5, %width.i33.i, !dbg !6090
  %_48.i70.i.5 = add i32 %_49.i69.i.5, 5, !dbg !6090
  %_51.i72.i.5 = icmp ult i32 %_48.i70.i.5, %_163.1.i82.i.pre, !dbg !6091
  br i1 %_51.i72.i.5, label %bb18.i74.i.5, label %panic1.i73.i, !dbg !6091

bb18.i74.i.5:                                     ; preds = %bb14.i64.i.5
  %446 = getelementptr inbounds nuw float, ptr %_161.0.i75.i, i32 %_48.i70.i.5, !dbg !6091
  %_47.i76.i.5 = load float, ptr %446, align 4, !dbg !6091, !noalias !6065, !noundef !10
  store float %_47.i76.i.5, ptr %iter.sroa.0.0.ptr.i55.i7649.5, align 4, !dbg !6092, !alias.scope !5972, !noalias !6093
  %447 = icmp eq i32 %width.i33.i, 6, !dbg !6074
  br i1 %447, label %bb53.i77.i, label %bb36.i56.i.6, !dbg !6074

bb36.i56.i.6:                                     ; preds = %bb18.i74.i.5
  %exitcond11923.6.not = icmp eq i32 %_159.1.i61.i, 6, !dbg !6077
  br i1 %exitcond11923.6.not, label %panic.i63.i, label %bb14.i64.i.6, !dbg !6077

bb14.i64.i.6:                                     ; preds = %bb36.i56.i.6
  %448 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i, i32 80, !dbg !6077
  %_42.i66.i.6 = load i32, ptr %448, align 4, !dbg !6077, !noalias !6065, !noundef !10
  %449 = add i32 %_42.i66.i.6, %ring_cursor.sroa.0.1.i4597664, !dbg !6088
  %_45.not.i67.i.6 = icmp ult i32 %449, %_87.i, !dbg !6089
  %450 = select i1 %_45.not.i67.i.6, i32 0, i32 %_87.i, !dbg !6089
  %spec.select.i68.i.6 = sub nuw i32 %449, %450, !dbg !6089
  %_49.i69.i.6 = mul i32 %spec.select.i68.i.6, %width.i33.i, !dbg !6090
  %_48.i70.i.6 = add i32 %_49.i69.i.6, 6, !dbg !6090
  %_51.i72.i.6 = icmp ult i32 %_48.i70.i.6, %_163.1.i82.i.pre, !dbg !6091
  br i1 %_51.i72.i.6, label %bb18.i74.i.6, label %panic1.i73.i, !dbg !6091

bb18.i74.i.6:                                     ; preds = %bb14.i64.i.6
  %451 = getelementptr inbounds nuw float, ptr %_161.0.i75.i, i32 %_48.i70.i.6, !dbg !6091
  %_47.i76.i.6 = load float, ptr %451, align 4, !dbg !6091, !noalias !6065, !noundef !10
  store float %_47.i76.i.6, ptr %iter.sroa.0.0.ptr.i55.i7649.6, align 4, !dbg !6092, !alias.scope !5972, !noalias !6093
  %452 = icmp eq i32 %width.i33.i, 7, !dbg !6074
  br i1 %452, label %bb53.i77.i, label %bb36.i56.i.7, !dbg !6074

bb36.i56.i.7:                                     ; preds = %bb18.i74.i.6
  %exitcond11923.7.not = icmp eq i32 %_159.1.i61.i, 7, !dbg !6077
  br i1 %exitcond11923.7.not, label %panic.i63.i, label %bb14.i64.i.7, !dbg !6077

bb14.i64.i.7:                                     ; preds = %bb36.i56.i.7
  %453 = getelementptr inbounds nuw i8, ptr %_159.0.i65.i, i32 92, !dbg !6077
  %_42.i66.i.7 = load i32, ptr %453, align 4, !dbg !6077, !noalias !6065, !noundef !10
  %454 = add i32 %_42.i66.i.7, %ring_cursor.sroa.0.1.i4597664, !dbg !6088
  %_45.not.i67.i.7 = icmp ult i32 %454, %_87.i, !dbg !6089
  %455 = select i1 %_45.not.i67.i.7, i32 0, i32 %_87.i, !dbg !6089
  %spec.select.i68.i.7 = sub nuw i32 %454, %455, !dbg !6089
  %_49.i69.i.7 = mul i32 %spec.select.i68.i.7, %width.i33.i, !dbg !6090
  %_48.i70.i.7 = add i32 %_49.i69.i.7, 7, !dbg !6090
  %_51.i72.i.7 = icmp ult i32 %_48.i70.i.7, %_163.1.i82.i.pre, !dbg !6091
  br i1 %_51.i72.i.7, label %bb18.i74.i.7, label %panic1.i73.i, !dbg !6091

bb18.i74.i.7:                                     ; preds = %bb14.i64.i.7
  %456 = getelementptr inbounds nuw float, ptr %_161.0.i75.i, i32 %_48.i70.i.7, !dbg !6091
  %_47.i76.i.7 = load float, ptr %456, align 4, !dbg !6091, !noalias !6065, !noundef !10
  store float %_47.i76.i.7, ptr %iter.sroa.0.0.ptr.i55.i7649.7, align 4, !dbg !6092, !alias.scope !5972, !noalias !6093
  br label %bb53.i77.i, !dbg !6074

panic1.i73.i:                                     ; preds = %bb14.i64.i.7, %bb14.i64.i.6, %bb14.i64.i.5, %bb14.i64.i.4, %bb14.i64.i.3, %bb14.i64.i.2, %bb14.i64.i.1, %bb14.i64.i
  %_48.i70.i.lcssa.ph = phi i32 [ %_48.i70.i.7, %bb14.i64.i.7 ], [ %_48.i70.i.6, %bb14.i64.i.6 ], [ %_48.i70.i.5, %bb14.i64.i.5 ], [ %_48.i70.i.4, %bb14.i64.i.4 ], [ %_48.i70.i.3, %bb14.i64.i.3 ], [ %_48.i70.i.2, %bb14.i64.i.2 ], [ %_48.i70.i.1, %bb14.i64.i.1 ], [ %_49.i69.i, %bb14.i64.i ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_48.i70.i.lcssa.ph, i32 noundef %_163.1.i82.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_0e76a26fbb751dfb8cf8a069b5b0d39a) #28, !dbg !6091, !noalias !6065
  unreachable, !dbg !6091

bb42.i84.i:                                       ; preds = %bb53.i77.i
  %_163.0.i85.i = load ptr, ptr %357, align 4, !dbg !6076, !alias.scope !5975, !noalias !5976, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6094), !dbg !6097
  %_4.not.i3138 = icmp eq i32 %_163.1.i82.i.pre, %_22.i39.i, !dbg !6098
  br i1 %_4.not.i3138, label %panic.i3140, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3141, !dbg !6098

panic.i3140:                                      ; preds = %bb42.i84.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #28, !dbg !6098, !noalias !6100
  unreachable, !dbg !6098

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3141: ; preds = %bb42.i84.i
  %_130.i87.i = getelementptr inbounds nuw float, ptr %_163.0.i85.i, i32 %_22.i39.i, !dbg !6101
  store float %_0.i2617, ptr %_130.i87.i, align 4, !dbg !6098, !alias.scope !6094, !noalias !6065
  %_0.i2389 = fdiv float %_0.i2846, %_62.i89.i, !dbg !6103
  %_0.i2845 = fsub float 1.000000e+00, %_0.i2389, !dbg !6105
  %_0.i2844 = fsub float %_0.i2845, %_0.i32207738, !dbg !6107
  %_4.i2405 = fmul float %_9.i32.i, %_0.i2844, !dbg !6109
  %_0.i2406 = fadd float %_0.i32207738, %_4.i2405, !dbg !6109
  %_3.i.i3620.inv = fcmp ogt float %_0.i2845, %_0.i2406, !dbg !6111
  %_4.i.i3627.v = select i1 %_3.i.i3620.inv, float %_0.i2845, float %_0.i2406, !dbg !6111
  %457 = tail call noundef float @llvm.fabs.f32(float %_4.i.i3627.v), !dbg !6114
  %458 = fcmp uge float %457, 0x3BC79CA100000000, !dbg !6117
  %_0.i3220 = select i1 %458, float %_4.i.i3627.v, float 0.000000e+00, !dbg !6119
  %_0.i2843 = fsub float 1.000000e+00, %_0.i3220, !dbg !6120
  %_164.1.i101.i = load i32, ptr %361, align 4, !dbg !6122, !alias.scope !5975, !noalias !5976, !noundef !10
  %_74.i102.i = mul i32 %width.i33.i, %main_cursor.sroa.0.1.i4607665, !dbg !6123
  %_134.i103.i = icmp ugt i32 %_74.i102.i, %_164.1.i101.i, !dbg !6124
  br i1 %_134.i103.i, label %bb47.i122.i, label %bb48.i104.i, !dbg !6124, !prof !755

bb41.i123.i:                                      ; preds = %bb53.i77.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i39.i, i32 noundef %_163.1.i82.i.pre, i32 noundef %_163.1.i82.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c2286ff41a33b8c11aa270fe215441de) #28, !dbg !6127, !noalias !6065
  unreachable, !dbg !6127

bb48.i104.i:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3141
  %_164.0.i105.i = load ptr, ptr %362, align 4, !dbg !6122, !alias.scope !5975, !noalias !5976, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6128), !dbg !6131
  %_3.not.i2980 = icmp eq i32 %_164.1.i101.i, %_74.i102.i, !dbg !6132
  br i1 %_3.not.i2980, label %panic.i2983, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3133, !dbg !6132

panic.i2983:                                      ; preds = %bb48.i104.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #28, !dbg !6132, !noalias !6134
  unreachable, !dbg !6132

bb47.i122.i:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3141
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_74.i102.i, i32 noundef %_164.1.i101.i, i32 noundef %_164.1.i101.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a3a21efd18b403ec25db545d522c479) #28, !dbg !6135, !noalias !6065
  unreachable, !dbg !6135

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3133: ; preds = %bb48.i104.i
  %_141.i107.i = getelementptr inbounds nuw float, ptr %_164.0.i105.i, i32 %_74.i102.i, !dbg !6136
  %_0.i2982 = load float, ptr %_141.i107.i, align 4, !dbg !6132, !alias.scope !6128, !noalias !6065, !noundef !10
  store float %_0.i2991, ptr %_141.i107.i, align 4, !dbg !6138, !alias.scope !6140, !noalias !6065
  %_0.i2616 = fmul float %_0.i2843, %_0.i2982, !dbg !6143
  %_6.i3391 = bitcast float %_0.i2982 to i32, !dbg !6145
  %_5.i3392 = and i32 %_6.i3391, %all.sroa.0.0.i444, !dbg !6148
  %_8.i3393 = bitcast float %_0.i2616 to i32, !dbg !6149
  %_7.i3395 = and i32 %_9.i3394, %_8.i3393, !dbg !6151
  %_4.i3396 = or disjoint i32 %_7.i3395, %_5.i3392, !dbg !6148
  store i32 %_4.i3396, ptr %_139.i470, align 4, !dbg !6152, !alias.scope !6154, !noalias !6157
  %_140.i = icmp ugt i32 %_60.i, %right_io.1, !dbg !6158
  br i1 %_140.i, label %bb48.i, label %bb49.i472, !dbg !6158, !prof !755

bb46.i:                                           ; preds = %bb42.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_60.i, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_f462f4e49ffb40af04504eaf795aa606) #28, !dbg !6162, !noalias !5917
  unreachable, !dbg !6162

bb49.i472:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3133
  %_147.i = getelementptr inbounds nuw float, ptr %right_io.0, i32 %_60.i, !dbg !6163
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6168), !dbg !6171
  %_3.not.i2975 = icmp eq i32 %right_io.1, %_60.i, !dbg !6172
  br i1 %_3.not.i2975, label %panic.i2978, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2979, !dbg !6172

panic.i2978:                                      ; preds = %bb49.i472
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #28, !dbg !6172, !noalias !6174
  unreachable, !dbg !6172

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2979: ; preds = %bb49.i472
  %_0.i2977 = load float, ptr %_147.i, align 4, !dbg !6172, !alias.scope !6168, !noalias !5917, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6175), !dbg !6178
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6179), !dbg !6178
  %width.i.i = load i32, ptr %363, align 4, !dbg !6181, !alias.scope !6182, !noalias !6183, !noundef !10
  %_3.i1955 = fcmp uge float %_8.i.i464, %_0.i3410, !dbg !6186
  %_0.i2388 = fdiv float %_8.i.i464, %_0.i3410, !dbg !6188
  %_0.i3390 = select i1 %_3.i1955, float 1.000000e+00, float %_0.i2388, !dbg !6190
  %_158.1.i.i = load i32, ptr %364, align 4, !dbg !6192, !alias.scope !6182, !noalias !6183, !noundef !10
  %_22.i.i477 = mul i32 %width.i.i, %ring_cursor.sroa.0.1.i4597664, !dbg !6193
  %_90.i.i = icmp ugt i32 %_22.i.i477, %_158.1.i.i, !dbg !6194
  br i1 %_90.i.i, label %bb34.i.i, label %bb35.i.i, !dbg !6194, !prof !755

bb35.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2979
  %_158.0.i.i = load ptr, ptr %365, align 4, !dbg !6192, !alias.scope !6182, !noalias !6183, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6197), !dbg !6200
  %_4.not.i3126 = icmp eq i32 %_158.1.i.i, %_22.i.i477, !dbg !6201
  br i1 %_4.not.i3126, label %panic.i3128, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3129, !dbg !6201

panic.i3128:                                      ; preds = %bb35.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #28, !dbg !6201, !noalias !6203
  unreachable, !dbg !6201

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3129: ; preds = %bb35.i.i
  %_97.i.i = getelementptr inbounds nuw float, ptr %_158.0.i.i, i32 %_22.i.i477, !dbg !6204
  store float %_0.i3390, ptr %_97.i.i, align 4, !dbg !6201, !alias.scope !6197, !noalias !6206
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6207), !dbg !6210
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6211), !dbg !6210
  %width.i1583 = load i32, ptr %363, align 4, !dbg !6213, !alias.scope !6207, !noalias !6215, !noundef !10
  %459 = icmp eq i32 %width.i1583, 0, !dbg !6216
  br i1 %459, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1688, label %bb29.i1589.lr.ph, !dbg !6216

bb29.i1589.lr.ph:                                 ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3129
  %_126.1.i1594 = load i32, ptr %366, align 4, !alias.scope !6207, !noalias !6215, !noundef !10
  %_126.0.i1598 = load ptr, ptr %367, align 4, !nonnull !10
  %460 = add i32 %ring_cursor.sroa.0.1.i4597664, 1
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
  br label %bb29.i1589, !dbg !6216

bb29.i1589:                                       ; preds = %bb29.i1589.lr.ph, %bb28.i1651
  %iter.sroa.0.0.idx.i15877656 = phi i32 [ 0, %bb29.i1589.lr.ph ], [ %iter.sroa.0.0.add.i1592, %bb28.i1651 ]
  %iter.sroa.4.0.i15867655 = phi i32 [ 0, %bb29.i1589.lr.ph ], [ %_102.0.i1593, %bb28.i1651 ]
  %iter.sroa.7.0.i15857654 = phi i32 [ %width.i1583, %bb29.i1589.lr.ph ], [ %462, %bb28.i1651 ]
  %iter.sroa.0.0.ptr.i15887657 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 %iter.sroa.0.0.idx.i15877656, !dbg !6218
  %462 = add i32 %iter.sroa.7.0.i15857654, -1, !dbg !6218
  %_109.i1590 = icmp eq i32 %iter.sroa.0.0.idx.i15877656, 32, !dbg !6219
  br i1 %_109.i1590, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1688.loopexit, label %bb33.i1591, !dbg !6223

bb33.i1591:                                       ; preds = %bb29.i1589
  %iter.sroa.0.0.add.i1592 = add nuw nsw i32 %iter.sroa.0.0.idx.i15877656, 4, !dbg !6224
  %_102.0.i1593 = add nuw nsw i32 %iter.sroa.4.0.i15867655, 1, !dbg !6226
  %exitcond11925.not = icmp eq i32 %iter.sroa.4.0.i15867655, %_126.1.i1594, !dbg !6227
  br i1 %exitcond11925.not, label %panic.i1596, label %bb2.i1597, !dbg !6227

bb2.i1597:                                        ; preds = %bb33.i1591
  %463 = getelementptr inbounds nuw %LaneShape, ptr %_126.0.i1598, i32 %iter.sroa.4.0.i15867655, !dbg !6227
  %shape.i1599 = load i32, ptr %463, align 4, !dbg !6227, !noalias !6228, !noundef !10
  %464 = getelementptr inbounds nuw i8, ptr %463, i32 4, !dbg !6227
  %shape3.i1600 = load i32, ptr %464, align 4, !dbg !6227, !noalias !6228, !noundef !10
  %465 = add i32 %shape3.i1600, %ring_cursor.sroa.0.1.i4597664, !dbg !6229
  %_18.not.i1601 = icmp ult i32 %465, %_87.i, !dbg !6230
  %466 = select i1 %_18.not.i1601, i32 0, i32 %_87.i, !dbg !6230
  %spec.select.i1602 = sub nuw i32 %465, %466, !dbg !6230
  %_25.i1605 = mul i32 %spec.select.i1602, %width.i1583, !dbg !6231
  %_24.i1606 = add i32 %_25.i1605, %iter.sroa.4.0.i15867655, !dbg !6231
  %_28.i1608 = icmp ult i32 %_24.i1606, %_128.1.i1607, !dbg !6232
  br i1 %_28.i1608, label %bb9.i1610, label %panic5.i1609, !dbg !6232

panic.i1596:                                      ; preds = %bb33.i1591
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_126.1.i1594, i32 noundef %_126.1.i1594, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_12856b5f9033764dbf23e04eb77c5c46) #28, !dbg !6227, !noalias !6228
  unreachable, !dbg !6227

bb9.i1610:                                        ; preds = %bb2.i1597
  %467 = getelementptr inbounds nuw float, ptr %_128.0.i1611, i32 %_24.i1606, !dbg !6232
  %468 = load float, ptr %467, align 4, !dbg !6232, !noalias !6228, !noundef !10
  %exitcond11926.not = icmp eq i32 %iter.sroa.4.0.i15867655, %_130.1.i1612, !dbg !6233
  br i1 %exitcond11926.not, label %panic6.i1614, label %bb10.i1615, !dbg !6233

panic5.i1609:                                     ; preds = %bb2.i1597
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_24.i1606, i32 noundef %_128.1.i1607, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70ebf0cf1c90c6161926611ccf249a32) #28, !dbg !6232, !noalias !6228
  unreachable, !dbg !6232

bb10.i1615:                                       ; preds = %bb9.i1610
  %469 = getelementptr inbounds nuw i32, ptr %_130.0.i1616, i32 %iter.sroa.4.0.i15867655, !dbg !6233
  %_30.i1617 = load i32, ptr %469, align 4, !dbg !6233, !noalias !6228, !noundef !10
  %470 = icmp eq i32 %_30.i1617, 0, !dbg !6234
  br i1 %470, label %bb14.i1626, label %bb12.i1618, !dbg !6234

panic6.i1614:                                     ; preds = %bb9.i1610
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_130.1.i1612, i32 noundef %_130.1.i1612, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_023d591aaad7f5cf482ac80a9401eca1) #28, !dbg !6233, !noalias !6228
  unreachable, !dbg !6233

bb12.i1618:                                       ; preds = %bb10.i1615
  %_35.i1620 = icmp ult i32 %iter.sroa.4.0.i15867655, %_132.1.i1619, !dbg !6235
  br i1 %_35.i1620, label %bb13.i1622, label %panic7.i1621, !dbg !6235

bb14.i1626:                                       ; preds = %bb34.i1687, %bb13.i1622, %bb10.i1615
  %newest.sroa.0.0.i1627 = phi float [ %468, %bb10.i1615 ], [ %_33.i1624, %bb34.i1687 ], [ %468, %bb13.i1622 ], !dbg !6236
  %exitcond11927.not = icmp eq i32 %iter.sroa.4.0.i15867655, %_132.1.i1619, !dbg !6237
  br i1 %exitcond11927.not, label %panic8.i1630, label %bb15.i1631, !dbg !6237

bb13.i1622:                                       ; preds = %bb12.i1618
  %471 = getelementptr inbounds nuw float, ptr %_132.0.i1623, i32 %iter.sroa.4.0.i15867655, !dbg !6235
  %_33.i1624 = load float, ptr %471, align 4, !dbg !6235, !noalias !6228, !noundef !10
  %_116.i1625 = fcmp olt float %_33.i1624, %468, !dbg !6238
  br i1 %_116.i1625, label %bb34.i1687, label %bb14.i1626, !dbg !6238

panic7.i1621:                                     ; preds = %bb12.i1618
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %iter.sroa.4.0.i15867655, i32 noundef %_132.1.i1619, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a228cac3279aae3a34886f59f51fbe9e) #28, !dbg !6235, !noalias !6228
  unreachable, !dbg !6235

bb34.i1687:                                       ; preds = %bb13.i1622
  br label %bb14.i1626, !dbg !6240

bb15.i1631:                                       ; preds = %bb14.i1626
  %472 = getelementptr inbounds nuw float, ptr %_132.0.i1623, i32 %iter.sroa.4.0.i15867655, !dbg !6237
  store float %newest.sroa.0.0.i1627, ptr %472, align 4, !dbg !6237, !noalias !6228
  %_40.i1633 = add i32 %_30.i1617, 1, !dbg !6241
  %complete.i1634 = icmp eq i32 %_40.i1633, %shape.i1599, !dbg !6241
  br i1 %complete.i1634, label %bb19.i1656, label %bb17.i1635, !dbg !6242

panic8.i1630:                                     ; preds = %bb14.i1626
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_132.1.i1619, i32 noundef %_132.1.i1619, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1fa5d88c1a2cc292a2767c48606a38fe) #28, !dbg !6237, !noalias !6228
  unreachable, !dbg !6237

bb17.i1635:                                       ; preds = %bb15.i1631
  %_42.i1637 = add i32 %iter.sroa.4.0.i15867655, %_43.i1636, !dbg !6243
  %_45.i1639 = icmp ult i32 %_42.i1637, %_128.1.i1607, !dbg !6244
  br i1 %_45.i1639, label %bb27.i1649, label %panic9.i1640, !dbg !6244

panic9.i1640:                                     ; preds = %bb17.i1635
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_42.i1637, i32 noundef %_128.1.i1607, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70d40ae2302f3ea19fa0c4a65fe82786) #28, !dbg !6244, !noalias !6228
  unreachable, !dbg !6244

bb27.i1649:                                       ; preds = %bb17.i1635
  %473 = getelementptr inbounds nuw float, ptr %_128.0.i1611, i32 %_42.i1637, !dbg !6244
  %_41.i1643 = load float, ptr %473, align 4, !dbg !6244, !noalias !6228, !noundef !10
  %_117.i1644 = fcmp olt float %_41.i1643, %newest.sroa.0.0.i1627, !dbg !6245
  %newest.sroa.0.1.i1645 = select i1 %_117.i1644, float %_41.i1643, float %newest.sroa.0.0.i1627, !dbg !6245
  store float %newest.sroa.0.1.i1645, ptr %iter.sroa.0.0.ptr.i15887657, align 4, !dbg !6247, !alias.scope !6211, !noalias !6248
  br label %bb28.i1651, !dbg !6249

bb28.i1651:                                       ; preds = %bb22.i1684, %bb19.i1656, %bb27.i1649
  %storemerge4525 = phi i32 [ %_40.i1633, %bb27.i1649 ], [ 0, %bb19.i1656 ], [ 0, %bb22.i1684 ], !dbg !6250
  store i32 %storemerge4525, ptr %469, align 4, !dbg !6250, !noalias !6228
  %474 = icmp eq i32 %462, 0, !dbg !6216
  br i1 %474, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1688.loopexit, label %bb29.i1589, !dbg !6216

bb19.i1656:                                       ; preds = %bb15.i1631
  store float %newest.sroa.0.0.i1627, ptr %iter.sroa.0.0.ptr.i15887657, align 4, !dbg !6247, !alias.scope !6211, !noalias !6248
  %_118.i16627650.not = icmp eq i32 %shape.i1599, 0, !dbg !6251
  br i1 %_118.i16627650.not, label %bb28.i1651, label %bb40.i1669.preheader, !dbg !6255

bb40.i1669.preheader:                             ; preds = %bb19.i1656
  %475 = load float, ptr %467, align 4, !dbg !6256, !noalias !6228, !noundef !10
  br label %bb40.i1669, !dbg !6257

bb40.i1669:                                       ; preds = %bb40.i1669.preheader, %bb22.i1684
  %iter2.sroa.0.0.i16617653 = phi i32 [ %_119.i1670, %bb22.i1684 ], [ 0, %bb40.i1669.preheader ]
  %suffix.sroa.0.0.i16607652 = phi float [ %suffix.sroa.0.1.i1680, %bb22.i1684 ], [ %475, %bb40.i1669.preheader ]
  %end.sroa.0.1.i16597651 = phi i32 [ %478, %bb22.i1684 ], [ %spec.select.i1602, %bb40.i1669.preheader ]
  %_54.i1671 = mul i32 %end.sroa.0.1.i16597651, %width.i1583, !dbg !6258
  %_53.i1672 = add i32 %_54.i1671, %iter.sroa.4.0.i15867655, !dbg !6258
  %_57.i1674 = icmp ult i32 %_53.i1672, %_128.1.i1607, !dbg !6257
  br i1 %_57.i1674, label %bb22.i1684, label %panic13.i1675, !dbg !6257

panic13.i1675:                                    ; preds = %bb40.i1669
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_53.i1672, i32 noundef %_128.1.i1607, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a271bbb7fd83c3605ea58a06b7065fa4) #28, !dbg !6257, !noalias !6228
  unreachable, !dbg !6257

bb22.i1684:                                       ; preds = %bb40.i1669
  %_119.i1670 = add nuw i32 %iter2.sroa.0.0.i16617653, 1, !dbg !6259
  %476 = getelementptr inbounds nuw float, ptr %_128.0.i1611, i32 %_53.i1672, !dbg !6257
  %_52.i1678 = load float, ptr %476, align 4, !dbg !6257, !noalias !6228, !noundef !10
  %_121.i1679 = fcmp olt float %suffix.sroa.0.0.i16607652, %_52.i1678, !dbg !6262
  %suffix.sroa.0.1.i1680 = select i1 %_121.i1679, float %suffix.sroa.0.0.i16607652, float %_52.i1678, !dbg !6262
  store float %suffix.sroa.0.1.i1680, ptr %476, align 4, !dbg !6264, !noalias !6228
  %477 = icmp eq i32 %end.sroa.0.1.i16597651, 0, !dbg !6265
  %spec.store.select.i1686 = select i1 %477, i32 %_87.i, i32 %end.sroa.0.1.i16597651, !dbg !6265
  %478 = add i32 %spec.store.select.i1686, -1, !dbg !6266
  %exitcond11924.not = icmp eq i32 %_119.i1670, %shape.i1599, !dbg !6251
  br i1 %exitcond11924.not, label %bb28.i1651, label %bb40.i1669, !dbg !6255

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1688.loopexit: ; preds = %bb28.i1651, %bb29.i1589
  %_0.i2974.pre = load float, ptr %scratch.i, align 4, !dbg !6267, !alias.scope !6269, !noalias !6272
  br label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1688, !dbg !6267

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1688: ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1688.loopexit, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3129
  %_0.i2974 = phi float [ %_0.i2974.pre, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1688.loopexit ], [ %_0.i2986, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3129 ], !dbg !6267
  %_0.i2615 = fmul float %_0.i2974, 1.638400e+04, !dbg !6273
  %479 = tail call noundef float @llvm.floor.f32(float %_0.i2615), !dbg !6275
  %_0.i2614 = fmul float %479, 0x3F10000000000000, !dbg !6279
  %480 = icmp eq i32 %width.i.i, 0, !dbg !6281
  %_163.1.i.i.pre = load i32, ptr %372, align 4, !dbg !6283, !alias.scope !6182, !noalias !6183
  br i1 %480, label %bb53.i.i, label %bb36.i.i.lr.ph, !dbg !6281

bb36.i.i.lr.ph:                                   ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1688
  %_159.1.i.i = load i32, ptr %366, align 4, !alias.scope !6182, !noalias !6183, !noundef !10
  %_159.0.i.i = load ptr, ptr %367, align 4, !nonnull !10
  %_161.0.i.i = load ptr, ptr %373, align 4, !nonnull !10
  %exitcond11928.not = icmp eq i32 %_159.1.i.i, 0, !dbg !6284
  br i1 %exitcond11928.not, label %panic.i.i, label %bb14.i.i, !dbg !6284

bb34.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit2979
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i477, i32 noundef %_158.1.i.i, i32 noundef %_158.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_db7c42041d7aaaba4839906f37294547) #28, !dbg !6285, !noalias !6206
  unreachable, !dbg !6285

bb53.i.i:                                         ; preds = %bb18.i.i.7, %bb18.i.i, %bb18.i.i.1, %bb18.i.i.2, %bb18.i.i.3, %bb18.i.i.4, %bb18.i.i.5, %bb18.i.i.6, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1688
  %_0.i2972 = phi float [ %_0.i2974, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1688 ], [ %_47.i.i, %bb18.i.i ], [ %_47.i.i, %bb18.i.i.7 ], [ %_47.i.i, %bb18.i.i.6 ], [ %_47.i.i, %bb18.i.i.5 ], [ %_47.i.i, %bb18.i.i.4 ], [ %_47.i.i, %bb18.i.i.3 ], [ %_47.i.i, %bb18.i.i.2 ], [ %_47.i.i, %bb18.i.i.1 ], !dbg !6286
  %_0.i2185 = fadd float %_0.i2614, %_0.i28427807, !dbg !6288
  %_0.i2842 = fsub float %_0.i2185, %_0.i2972, !dbg !6290
  %_123.i.i = icmp ugt i32 %_22.i.i477, %_163.1.i.i.pre, !dbg !6292
  br i1 %_123.i.i, label %bb41.i.i, label %bb42.i.i, !dbg !6292, !prof !755

bb14.i.i:                                         ; preds = %bb36.i.i.lr.ph
  %481 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 8, !dbg !6284
  %_42.i.i = load i32, ptr %481, align 4, !dbg !6284, !noalias !6272, !noundef !10
  %482 = add i32 %_42.i.i, %ring_cursor.sroa.0.1.i4597664, !dbg !6295
  %_45.not.i.i = icmp ult i32 %482, %_87.i, !dbg !6296
  %483 = select i1 %_45.not.i.i, i32 0, i32 %_87.i, !dbg !6296
  %spec.select.i.i = sub nuw i32 %482, %483, !dbg !6296
  %_49.i.i483 = mul i32 %spec.select.i.i, %width.i.i, !dbg !6297
  %_51.i.i = icmp ult i32 %_49.i.i483, %_163.1.i.i.pre, !dbg !6298
  br i1 %_51.i.i, label %bb18.i.i, label %panic1.i.i, !dbg !6298

panic.i.i:                                        ; preds = %bb36.i.i.7, %bb36.i.i.6, %bb36.i.i.5, %bb36.i.i.4, %bb36.i.i.3, %bb36.i.i.2, %bb36.i.i.1, %bb36.i.i.lr.ph
  %_159.1.i.i.lcssa.ph = phi i32 [ 7, %bb36.i.i.7 ], [ 6, %bb36.i.i.6 ], [ 5, %bb36.i.i.5 ], [ 4, %bb36.i.i.4 ], [ 3, %bb36.i.i.3 ], [ 2, %bb36.i.i.2 ], [ 1, %bb36.i.i.1 ], [ 0, %bb36.i.i.lr.ph ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_159.1.i.i.lcssa.ph, i32 noundef %_159.1.i.i.lcssa.ph, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_33746731a929bad63709ddedbca82f5f) #28, !dbg !6284, !noalias !6272
  unreachable, !dbg !6284

bb18.i.i:                                         ; preds = %bb14.i.i
  %484 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_49.i.i483, !dbg !6298
  %_47.i.i = load float, ptr %484, align 4, !dbg !6298, !noalias !6272, !noundef !10
  store float %_47.i.i, ptr %scratch.i, align 4, !dbg !6299, !alias.scope !6179, !noalias !6300
  %485 = icmp eq i32 %width.i.i, 1, !dbg !6281
  br i1 %485, label %bb53.i.i, label %bb36.i.i.1, !dbg !6281

bb36.i.i.1:                                       ; preds = %bb18.i.i
  %exitcond11928.1.not = icmp eq i32 %_159.1.i.i, 1, !dbg !6284
  br i1 %exitcond11928.1.not, label %panic.i.i, label %bb14.i.i.1, !dbg !6284

bb14.i.i.1:                                       ; preds = %bb36.i.i.1
  %486 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 20, !dbg !6284
  %_42.i.i.1 = load i32, ptr %486, align 4, !dbg !6284, !noalias !6272, !noundef !10
  %487 = add i32 %_42.i.i.1, %ring_cursor.sroa.0.1.i4597664, !dbg !6295
  %_45.not.i.i.1 = icmp ult i32 %487, %_87.i, !dbg !6296
  %488 = select i1 %_45.not.i.i.1, i32 0, i32 %_87.i, !dbg !6296
  %spec.select.i.i.1 = sub nuw i32 %487, %488, !dbg !6296
  %_49.i.i483.1 = mul i32 %spec.select.i.i.1, %width.i.i, !dbg !6297
  %_48.i.i.1 = add i32 %_49.i.i483.1, 1, !dbg !6297
  %_51.i.i.1 = icmp ult i32 %_48.i.i.1, %_163.1.i.i.pre, !dbg !6298
  br i1 %_51.i.i.1, label %bb18.i.i.1, label %panic1.i.i, !dbg !6298

bb18.i.i.1:                                       ; preds = %bb14.i.i.1
  %489 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.1, !dbg !6298
  %_47.i.i.1 = load float, ptr %489, align 4, !dbg !6298, !noalias !6272, !noundef !10
  store float %_47.i.i.1, ptr %iter.sroa.0.0.ptr.i.i7661.1, align 4, !dbg !6299, !alias.scope !6179, !noalias !6300
  %490 = icmp eq i32 %width.i.i, 2, !dbg !6281
  br i1 %490, label %bb53.i.i, label %bb36.i.i.2, !dbg !6281

bb36.i.i.2:                                       ; preds = %bb18.i.i.1
  %exitcond11928.2.not = icmp eq i32 %_159.1.i.i, 2, !dbg !6284
  br i1 %exitcond11928.2.not, label %panic.i.i, label %bb14.i.i.2, !dbg !6284

bb14.i.i.2:                                       ; preds = %bb36.i.i.2
  %491 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 32, !dbg !6284
  %_42.i.i.2 = load i32, ptr %491, align 4, !dbg !6284, !noalias !6272, !noundef !10
  %492 = add i32 %_42.i.i.2, %ring_cursor.sroa.0.1.i4597664, !dbg !6295
  %_45.not.i.i.2 = icmp ult i32 %492, %_87.i, !dbg !6296
  %493 = select i1 %_45.not.i.i.2, i32 0, i32 %_87.i, !dbg !6296
  %spec.select.i.i.2 = sub nuw i32 %492, %493, !dbg !6296
  %_49.i.i483.2 = mul i32 %spec.select.i.i.2, %width.i.i, !dbg !6297
  %_48.i.i.2 = add i32 %_49.i.i483.2, 2, !dbg !6297
  %_51.i.i.2 = icmp ult i32 %_48.i.i.2, %_163.1.i.i.pre, !dbg !6298
  br i1 %_51.i.i.2, label %bb18.i.i.2, label %panic1.i.i, !dbg !6298

bb18.i.i.2:                                       ; preds = %bb14.i.i.2
  %494 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.2, !dbg !6298
  %_47.i.i.2 = load float, ptr %494, align 4, !dbg !6298, !noalias !6272, !noundef !10
  store float %_47.i.i.2, ptr %iter.sroa.0.0.ptr.i.i7661.2, align 4, !dbg !6299, !alias.scope !6179, !noalias !6300
  %495 = icmp eq i32 %width.i.i, 3, !dbg !6281
  br i1 %495, label %bb53.i.i, label %bb36.i.i.3, !dbg !6281

bb36.i.i.3:                                       ; preds = %bb18.i.i.2
  %exitcond11928.3.not = icmp eq i32 %_159.1.i.i, 3, !dbg !6284
  br i1 %exitcond11928.3.not, label %panic.i.i, label %bb14.i.i.3, !dbg !6284

bb14.i.i.3:                                       ; preds = %bb36.i.i.3
  %496 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 44, !dbg !6284
  %_42.i.i.3 = load i32, ptr %496, align 4, !dbg !6284, !noalias !6272, !noundef !10
  %497 = add i32 %_42.i.i.3, %ring_cursor.sroa.0.1.i4597664, !dbg !6295
  %_45.not.i.i.3 = icmp ult i32 %497, %_87.i, !dbg !6296
  %498 = select i1 %_45.not.i.i.3, i32 0, i32 %_87.i, !dbg !6296
  %spec.select.i.i.3 = sub nuw i32 %497, %498, !dbg !6296
  %_49.i.i483.3 = mul i32 %spec.select.i.i.3, %width.i.i, !dbg !6297
  %_48.i.i.3 = add i32 %_49.i.i483.3, 3, !dbg !6297
  %_51.i.i.3 = icmp ult i32 %_48.i.i.3, %_163.1.i.i.pre, !dbg !6298
  br i1 %_51.i.i.3, label %bb18.i.i.3, label %panic1.i.i, !dbg !6298

bb18.i.i.3:                                       ; preds = %bb14.i.i.3
  %499 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.3, !dbg !6298
  %_47.i.i.3 = load float, ptr %499, align 4, !dbg !6298, !noalias !6272, !noundef !10
  store float %_47.i.i.3, ptr %iter.sroa.0.0.ptr.i.i7661.3, align 4, !dbg !6299, !alias.scope !6179, !noalias !6300
  %500 = icmp eq i32 %width.i.i, 4, !dbg !6281
  br i1 %500, label %bb53.i.i, label %bb36.i.i.4, !dbg !6281

bb36.i.i.4:                                       ; preds = %bb18.i.i.3
  %exitcond11928.4.not = icmp eq i32 %_159.1.i.i, 4, !dbg !6284
  br i1 %exitcond11928.4.not, label %panic.i.i, label %bb14.i.i.4, !dbg !6284

bb14.i.i.4:                                       ; preds = %bb36.i.i.4
  %501 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 56, !dbg !6284
  %_42.i.i.4 = load i32, ptr %501, align 4, !dbg !6284, !noalias !6272, !noundef !10
  %502 = add i32 %_42.i.i.4, %ring_cursor.sroa.0.1.i4597664, !dbg !6295
  %_45.not.i.i.4 = icmp ult i32 %502, %_87.i, !dbg !6296
  %503 = select i1 %_45.not.i.i.4, i32 0, i32 %_87.i, !dbg !6296
  %spec.select.i.i.4 = sub nuw i32 %502, %503, !dbg !6296
  %_49.i.i483.4 = mul i32 %spec.select.i.i.4, %width.i.i, !dbg !6297
  %_48.i.i.4 = add i32 %_49.i.i483.4, 4, !dbg !6297
  %_51.i.i.4 = icmp ult i32 %_48.i.i.4, %_163.1.i.i.pre, !dbg !6298
  br i1 %_51.i.i.4, label %bb18.i.i.4, label %panic1.i.i, !dbg !6298

bb18.i.i.4:                                       ; preds = %bb14.i.i.4
  %504 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.4, !dbg !6298
  %_47.i.i.4 = load float, ptr %504, align 4, !dbg !6298, !noalias !6272, !noundef !10
  store float %_47.i.i.4, ptr %iter.sroa.0.0.ptr.i.i7661.4, align 4, !dbg !6299, !alias.scope !6179, !noalias !6300
  %505 = icmp eq i32 %width.i.i, 5, !dbg !6281
  br i1 %505, label %bb53.i.i, label %bb36.i.i.5, !dbg !6281

bb36.i.i.5:                                       ; preds = %bb18.i.i.4
  %exitcond11928.5.not = icmp eq i32 %_159.1.i.i, 5, !dbg !6284
  br i1 %exitcond11928.5.not, label %panic.i.i, label %bb14.i.i.5, !dbg !6284

bb14.i.i.5:                                       ; preds = %bb36.i.i.5
  %506 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 68, !dbg !6284
  %_42.i.i.5 = load i32, ptr %506, align 4, !dbg !6284, !noalias !6272, !noundef !10
  %507 = add i32 %_42.i.i.5, %ring_cursor.sroa.0.1.i4597664, !dbg !6295
  %_45.not.i.i.5 = icmp ult i32 %507, %_87.i, !dbg !6296
  %508 = select i1 %_45.not.i.i.5, i32 0, i32 %_87.i, !dbg !6296
  %spec.select.i.i.5 = sub nuw i32 %507, %508, !dbg !6296
  %_49.i.i483.5 = mul i32 %spec.select.i.i.5, %width.i.i, !dbg !6297
  %_48.i.i.5 = add i32 %_49.i.i483.5, 5, !dbg !6297
  %_51.i.i.5 = icmp ult i32 %_48.i.i.5, %_163.1.i.i.pre, !dbg !6298
  br i1 %_51.i.i.5, label %bb18.i.i.5, label %panic1.i.i, !dbg !6298

bb18.i.i.5:                                       ; preds = %bb14.i.i.5
  %509 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.5, !dbg !6298
  %_47.i.i.5 = load float, ptr %509, align 4, !dbg !6298, !noalias !6272, !noundef !10
  store float %_47.i.i.5, ptr %iter.sroa.0.0.ptr.i.i7661.5, align 4, !dbg !6299, !alias.scope !6179, !noalias !6300
  %510 = icmp eq i32 %width.i.i, 6, !dbg !6281
  br i1 %510, label %bb53.i.i, label %bb36.i.i.6, !dbg !6281

bb36.i.i.6:                                       ; preds = %bb18.i.i.5
  %exitcond11928.6.not = icmp eq i32 %_159.1.i.i, 6, !dbg !6284
  br i1 %exitcond11928.6.not, label %panic.i.i, label %bb14.i.i.6, !dbg !6284

bb14.i.i.6:                                       ; preds = %bb36.i.i.6
  %511 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 80, !dbg !6284
  %_42.i.i.6 = load i32, ptr %511, align 4, !dbg !6284, !noalias !6272, !noundef !10
  %512 = add i32 %_42.i.i.6, %ring_cursor.sroa.0.1.i4597664, !dbg !6295
  %_45.not.i.i.6 = icmp ult i32 %512, %_87.i, !dbg !6296
  %513 = select i1 %_45.not.i.i.6, i32 0, i32 %_87.i, !dbg !6296
  %spec.select.i.i.6 = sub nuw i32 %512, %513, !dbg !6296
  %_49.i.i483.6 = mul i32 %spec.select.i.i.6, %width.i.i, !dbg !6297
  %_48.i.i.6 = add i32 %_49.i.i483.6, 6, !dbg !6297
  %_51.i.i.6 = icmp ult i32 %_48.i.i.6, %_163.1.i.i.pre, !dbg !6298
  br i1 %_51.i.i.6, label %bb18.i.i.6, label %panic1.i.i, !dbg !6298

bb18.i.i.6:                                       ; preds = %bb14.i.i.6
  %514 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.6, !dbg !6298
  %_47.i.i.6 = load float, ptr %514, align 4, !dbg !6298, !noalias !6272, !noundef !10
  store float %_47.i.i.6, ptr %iter.sroa.0.0.ptr.i.i7661.6, align 4, !dbg !6299, !alias.scope !6179, !noalias !6300
  %515 = icmp eq i32 %width.i.i, 7, !dbg !6281
  br i1 %515, label %bb53.i.i, label %bb36.i.i.7, !dbg !6281

bb36.i.i.7:                                       ; preds = %bb18.i.i.6
  %exitcond11928.7.not = icmp eq i32 %_159.1.i.i, 7, !dbg !6284
  br i1 %exitcond11928.7.not, label %panic.i.i, label %bb14.i.i.7, !dbg !6284

bb14.i.i.7:                                       ; preds = %bb36.i.i.7
  %516 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 92, !dbg !6284
  %_42.i.i.7 = load i32, ptr %516, align 4, !dbg !6284, !noalias !6272, !noundef !10
  %517 = add i32 %_42.i.i.7, %ring_cursor.sroa.0.1.i4597664, !dbg !6295
  %_45.not.i.i.7 = icmp ult i32 %517, %_87.i, !dbg !6296
  %518 = select i1 %_45.not.i.i.7, i32 0, i32 %_87.i, !dbg !6296
  %spec.select.i.i.7 = sub nuw i32 %517, %518, !dbg !6296
  %_49.i.i483.7 = mul i32 %spec.select.i.i.7, %width.i.i, !dbg !6297
  %_48.i.i.7 = add i32 %_49.i.i483.7, 7, !dbg !6297
  %_51.i.i.7 = icmp ult i32 %_48.i.i.7, %_163.1.i.i.pre, !dbg !6298
  br i1 %_51.i.i.7, label %bb18.i.i.7, label %panic1.i.i, !dbg !6298

bb18.i.i.7:                                       ; preds = %bb14.i.i.7
  %519 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.7, !dbg !6298
  %_47.i.i.7 = load float, ptr %519, align 4, !dbg !6298, !noalias !6272, !noundef !10
  store float %_47.i.i.7, ptr %iter.sroa.0.0.ptr.i.i7661.7, align 4, !dbg !6299, !alias.scope !6179, !noalias !6300
  br label %bb53.i.i, !dbg !6281

panic1.i.i:                                       ; preds = %bb14.i.i.7, %bb14.i.i.6, %bb14.i.i.5, %bb14.i.i.4, %bb14.i.i.3, %bb14.i.i.2, %bb14.i.i.1, %bb14.i.i
  %_48.i.i.lcssa.ph = phi i32 [ %_48.i.i.7, %bb14.i.i.7 ], [ %_48.i.i.6, %bb14.i.i.6 ], [ %_48.i.i.5, %bb14.i.i.5 ], [ %_48.i.i.4, %bb14.i.i.4 ], [ %_48.i.i.3, %bb14.i.i.3 ], [ %_48.i.i.2, %bb14.i.i.2 ], [ %_48.i.i.1, %bb14.i.i.1 ], [ %_49.i.i483, %bb14.i.i ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_48.i.i.lcssa.ph, i32 noundef %_163.1.i.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_0e76a26fbb751dfb8cf8a069b5b0d39a) #28, !dbg !6298, !noalias !6272
  unreachable, !dbg !6298

bb42.i.i:                                         ; preds = %bb53.i.i
  %_163.0.i.i = load ptr, ptr %373, align 4, !dbg !6283, !alias.scope !6182, !noalias !6183, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6301), !dbg !6304
  %_4.not.i3122 = icmp eq i32 %_163.1.i.i.pre, %_22.i.i477, !dbg !6305
  br i1 %_4.not.i3122, label %panic.i3124, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3125, !dbg !6305

panic.i3124:                                      ; preds = %bb42.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #28, !dbg !6305, !noalias !6307
  unreachable, !dbg !6305

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3125: ; preds = %bb42.i.i
  %_130.i.i = getelementptr inbounds nuw float, ptr %_163.0.i.i, i32 %_22.i.i477, !dbg !6308
  store float %_0.i2614, ptr %_130.i.i, align 4, !dbg !6305, !alias.scope !6301, !noalias !6272
  %_0.i2387 = fdiv float %_0.i2842, %_62.i.i486, !dbg !6310
  %_0.i2841 = fsub float 1.000000e+00, %_0.i2387, !dbg !6312
  %_0.i2840 = fsub float %_0.i2841, %_0.i32167876, !dbg !6314
  %_4.i2403 = fmul float %_9.i.i465, %_0.i2840, !dbg !6316
  %_0.i2404 = fadd float %_0.i32167876, %_4.i2403, !dbg !6316
  %_3.i.i3611.inv = fcmp ogt float %_0.i2841, %_0.i2404, !dbg !6318
  %_4.i.i3618.v = select i1 %_3.i.i3611.inv, float %_0.i2841, float %_0.i2404, !dbg !6318
  %520 = tail call noundef float @llvm.fabs.f32(float %_4.i.i3618.v), !dbg !6321
  %521 = fcmp uge float %520, 0x3BC79CA100000000, !dbg !6324
  %_0.i3216 = select i1 %521, float %_4.i.i3618.v, float 0.000000e+00, !dbg !6326
  %_0.i2839 = fsub float 1.000000e+00, %_0.i3216, !dbg !6327
  %_164.1.i.i = load i32, ptr %377, align 4, !dbg !6329, !alias.scope !6182, !noalias !6183, !noundef !10
  %_74.i.i = mul i32 %width.i.i, %main_cursor.sroa.0.1.i4607665, !dbg !6330
  %_134.i.i = icmp ugt i32 %_74.i.i, %_164.1.i.i, !dbg !6331
  br i1 %_134.i.i, label %bb47.i.i, label %bb48.i.i, !dbg !6331, !prof !755

bb41.i.i:                                         ; preds = %bb53.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i477, i32 noundef %_163.1.i.i.pre, i32 noundef %_163.1.i.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c2286ff41a33b8c11aa270fe215441de) #28, !dbg !6334, !noalias !6272
  unreachable, !dbg !6334

bb48.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3125
  %_164.0.i.i = load ptr, ptr %378, align 4, !dbg !6329, !alias.scope !6182, !noalias !6183, !nonnull !10, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6335), !dbg !6338
  %_3.not.i2966 = icmp eq i32 %_164.1.i.i, %_74.i.i, !dbg !6339
  br i1 %_3.not.i2966, label %panic.i2969, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3117, !dbg !6339

panic.i2969:                                      ; preds = %bb48.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #28, !dbg !6339, !noalias !6341
  unreachable, !dbg !6339

bb47.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3125
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_74.i.i, i32 noundef %_164.1.i.i, i32 noundef %_164.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a3a21efd18b403ec25db545d522c479) #28, !dbg !6342, !noalias !6272
  unreachable, !dbg !6342

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3117: ; preds = %bb48.i.i
  %_141.i.i = getelementptr inbounds nuw float, ptr %_164.0.i.i, i32 %_74.i.i, !dbg !6343
  %_0.i2968 = load float, ptr %_141.i.i, align 4, !dbg !6339, !alias.scope !6335, !noalias !6272, !noundef !10
  store float %_0.i2977, ptr %_141.i.i, align 4, !dbg !6345, !alias.scope !6347, !noalias !6272
  %_0.i2613 = fmul float %_0.i2839, %_0.i2968, !dbg !6350
  %_6.i3378 = bitcast float %_0.i2968 to i32, !dbg !6352
  %_5.i3379 = and i32 %_6.i3378, %all.sroa.0.0.i444, !dbg !6355
  %_8.i3380 = bitcast float %_0.i2613 to i32, !dbg !6356
  %_7.i3382 = and i32 %_9.i3394, %_8.i3380, !dbg !6358
  %_4.i3383 = or disjoint i32 %_7.i3382, %_5.i3379, !dbg !6355
  store i32 %_4.i3383, ptr %_147.i, align 4, !dbg !6359, !alias.scope !6361, !noalias !6364
  %522 = add i32 %main_cursor.sroa.0.1.i4607665, 1, !dbg !6365
  %_100.i = icmp eq i32 %522, %_102.i, !dbg !6366
  %spec.store.select11.i = select i1 %_100.i, i32 0, i32 %522, !dbg !6366
  %523 = add i32 %ring_cursor.sroa.0.1.i4597664, 1, !dbg !6367
  %_103.i = icmp eq i32 %523, %_87.i, !dbg !6368
  %spec.store.select12.i = select i1 %_103.i, i32 0, i32 %523, !dbg !6368
  %exitcond11931.not = icmp eq i32 %395, %umax11930, !dbg !6369
  br i1 %exitcond11931.not, label %bb16.i.bb13.i.loopexit_crit_edge, label %bb42.i, !dbg !5377

bb48.i:                                           ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3133
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_60.i, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_3ec0ee57975400dd5dbd052e150c5392) #28, !dbg !6372, !noalias !5917
  unreachable, !dbg !6372

_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit: ; preds = %bb13.i.loopexit, %bb11.i
  %ring_cursor.sroa.0.0.i451.lcssa = phi i32 [ %_37.i445, %bb11.i ], [ %ring_cursor.sroa.0.1.i459.lcssa, %bb13.i.loopexit ], !dbg !5331
  %main_cursor.sroa.0.0.i452.lcssa = phi i32 [ %_35.i, %bb11.i ], [ %main_cursor.sroa.0.1.i460.lcssa, %bb13.i.loopexit ], !dbg !5328
  call void @llvm.lifetime.start.p0(ptr nonnull %_106.i), !dbg !6373, !noalias !5311
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(92) %_106.i, ptr noundef nonnull align 4 dereferenceable(92) %hot_left.i437, i32 92, i1 false), !dbg !6373, !noalias !5311
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_106.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33) #27, !dbg !6374, !noalias !5917
  call void @llvm.lifetime.end.p0(ptr nonnull %_106.i), !dbg !6375, !noalias !5311
  call void @llvm.lifetime.start.p0(ptr nonnull %_108.i), !dbg !6376, !noalias !5311
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(92) %_108.i, ptr noundef nonnull align 4 dereferenceable(92) %hot_right.i436, i32 92, i1 false), !dbg !6376, !noalias !5311
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_108.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34) #27, !dbg !6377, !noalias !5917
  call void @llvm.lifetime.end.p0(ptr nonnull %_108.i), !dbg !6378, !noalias !5311
  store i32 %main_cursor.sroa.0.0.i452.lcssa, ptr %_35, align 4, !dbg !6379, !alias.scope !5305, !noalias !5330
  store i32 %ring_cursor.sroa.0.0.i451.lcssa, ptr %81, align 4, !dbg !6380, !alias.scope !5305, !noalias !5330
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i434), !dbg !6381, !noalias !5311
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i435), !dbg !6382, !noalias !5311
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i), !dbg !6383, !noalias !5311
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_right.i436), !dbg !6384, !noalias !5311
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i437), !dbg !6385, !noalias !5311
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockfEB2_.exit, !dbg !5300

bb7.i:                                            ; preds = %bb5.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6386), !dbg !6389
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6390), !dbg !6389
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6392), !dbg !6389
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6394), !dbg !6389
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6396), !dbg !6389
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i36), !dbg !6398, !noalias !6402
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_left.i36, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #27, !dbg !6405, !noalias !6406
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_right.i35), !dbg !6407, !noalias !6402
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_right.i35, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #27, !dbg !6409, !noalias !6410
  %524 = getelementptr inbounds nuw i8, ptr %self, i32 320, !dbg !6411
  %525 = load i8, ptr %524, align 4, !dbg !6411, !range !3570, !alias.scope !6386, !noalias !6415, !noundef !10
  %526 = getelementptr inbounds nuw i8, ptr %self, i32 321, !dbg !6416
  %527 = load i8, ptr %526, align 1, !dbg !6416, !range !3570, !alias.scope !6386, !noalias !6415, !noundef !10
  %528 = getelementptr inbounds nuw i8, ptr %self, i32 528, !dbg !6418
  %ring.i44 = load i32, ptr %528, align 4, !dbg !6418, !alias.scope !6390, !noalias !6420, !noundef !10
  %529 = getelementptr inbounds nuw i8, ptr %self, i32 532, !dbg !6421
  %main.i45 = load i32, ptr %529, align 4, !dbg !6421, !alias.scope !6390, !noalias !6420, !noundef !10
  %_36.i46 = load i32, ptr %_35, align 4, !dbg !6423, !alias.scope !6396, !noalias !6425, !noundef !10
  %530 = getelementptr inbounds nuw i8, ptr %self, i32 108, !dbg !6426
  %_37.i47 = load i32, ptr %530, align 4, !dbg !6426, !alias.scope !6396, !noalias !6425, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i34), !dbg !6428, !noalias !6402
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i34, i8 0, i32 1024, i1 false), !noalias !6402
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i33), !dbg !6430, !noalias !6402
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i33, i8 0, i32 1024, i1 false), !noalias !6402
  %_32.i40 = zext nneg i8 %525 to i32, !dbg !6411
  %.none.i41 = sub nsw i32 0, %_32.i40, !dbg !6432
  %_33.i42 = zext nneg i8 %527 to i32, !dbg !6416
  %all.sroa.0.0.i43 = sub nsw i32 0, %_33.i42, !dbg !6416
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i32), !dbg !6433, !noalias !6402
; call <true_peak_limiter::UniformHot<f32>>::new
  call fastcc void @_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotfE3newB5_(ptr noalias noundef align 4 captures(none) dereferenceable(44) %uniform_left.i32, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33, i32 %ring.i44, i32 %main.i45) #27, !dbg !6435
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_right.i31), !dbg !6436, !noalias !6402
  %_32.val = load i32, ptr %528, align 4, !dbg !6438, !noundef !10
  %_32.val3836 = load i32, ptr %529, align 4, !dbg !6438, !noundef !10
; call <true_peak_limiter::UniformHot<f32>>::new
  call fastcc void @_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotfE3newB5_(ptr noalias noundef align 4 captures(none) dereferenceable(44) %uniform_right.i31, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34, i32 %_32.val, i32 %_32.val3836) #27, !dbg !6438
  %_162.not.i588742 = icmp eq i32 %frames, 0, !dbg !6439
  br i1 %_162.not.i588742, label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit, label %bb44.i59.lr.ph, !dbg !6439

bb44.i59.lr.ph:                                   ; preds = %bb7.i
  %d9.i4041 = lshr i32 %frames, 5, !dbg !6449
  %r2.i4042 = and i32 %frames, 31, !dbg !6456
  %_19.not.i4043 = icmp ne i32 %r2.i4042, 0, !dbg !6457
  %531 = zext i1 %_19.not.i4043 to i32, !dbg !6457
  %yield_count.sroa.0.0.i4044 = add nuw nsw i32 %d9.i4041, %531, !dbg !6457
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
  br label %bb44.i59, !dbg !6439

bb19.i65.bb15.i53.loopexit_crit_edge:             ; preds = %bb67.i203
  store float %_0.i.i.lcssa1268314049, ptr %570, align 4
  store float %_0.i3247.lcssa1266914067, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265514085, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264114103, ptr %573, align 4
  store float %_0.i3260.lcssa1262714121, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261314139, ptr %574, align 4
  store float %_0.i.i3513.lcssa1259914157, ptr %576, align 4
  store float %_0.i3273.lcssa1258514175, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257114193, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255714211, ptr %579, align 4
  store float %_0.i3286.lcssa1254314229, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1252914247, ptr %580, align 4
  store float %_0.i2854.lcssa1271414265, ptr %585, align 4
  store float %_0.i3228.lcssa1273014283, ptr %587, align 4
  store float %_0.i2850.lcssa1275414301, ptr %593, align 4
  store float %_0.i3224.lcssa1275514319, ptr %595, align 4
  store i32 %storemerge.i1185.lcssa83908599, ptr %_22.i272.i, align 4
  store float %running.sroa.0.0.i1180.lcssa84178635, ptr %_21.i271.i, align 4
  store i32 %storemerge.i.lcssa85028671, ptr %_22.i.i160, align 4
  store float %running.sroa.0.0.i.lcssa85298707, ptr %_21.i.i159, align 4
  br label %bb15.i53.loopexit, !dbg !6458

bb15.i53.loopexit:                                ; preds = %bb19.i65.bb15.i53.loopexit_crit_edge, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i64
  %ring_cursor.sroa.0.1.i66.lcssa = phi i32 [ %ring_cursor.sroa.0.2.i206, %bb19.i65.bb15.i53.loopexit_crit_edge ], [ %ring_cursor.sroa.0.0.i548743, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i64 ], !dbg !6462
  %main_cursor.sroa.0.1.i67.lcssa = phi i32 [ %main_cursor.sroa.0.2.i209, %bb19.i65.bb15.i53.loopexit_crit_edge ], [ %main_cursor.sroa.0.0.i558744, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i64 ], !dbg !6463
  %_162.not.i58 = icmp eq i32 %599, 0, !dbg !6439
  %indvars.iv.next11933 = add i32 %indvars.iv11932, -32, !dbg !6439
  br i1 %_162.not.i58, label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit, label %bb44.i59, !dbg !6439

bb44.i59:                                         ; preds = %bb44.i59.lr.ph, %bb15.i53.loopexit
  %indvars.iv11932 = phi i32 [ %frames, %bb44.i59.lr.ph ], [ %indvars.iv.next11933, %bb15.i53.loopexit ]
  %iter2.sroa.0.0.i578746 = phi i32 [ %yield_count.sroa.0.0.i4044, %bb44.i59.lr.ph ], [ %599, %bb15.i53.loopexit ]
  %iter1.sroa.0.0.i568745 = phi i32 [ 0, %bb44.i59.lr.ph ], [ %598, %bb15.i53.loopexit ]
  %main_cursor.sroa.0.0.i558744 = phi i32 [ %_36.i46, %bb44.i59.lr.ph ], [ %main_cursor.sroa.0.1.i67.lcssa, %bb15.i53.loopexit ]
  %ring_cursor.sroa.0.0.i548743 = phi i32 [ %_37.i47, %bb44.i59.lr.ph ], [ %ring_cursor.sroa.0.1.i66.lcssa, %bb15.i53.loopexit ]
  %umin11952 = call i32 @llvm.umin.i32(i32 %indvars.iv11932, i32 32), !dbg !6464
  %umax11938 = call i32 @llvm.umax.i32(i32 %umin11952, i32 1), !dbg !6464
  %598 = add i32 %iter1.sroa.0.0.i568745, 32, !dbg !6464
  %599 = add nsw i32 %iter2.sroa.0.0.i578746, -1, !dbg !6468
  %600 = sub i32 %frames, %iter1.sroa.0.0.i568745, !dbg !6469
  %spec.store.select.i60 = tail call i32 @llvm.umin.i32(i32 %600, i32 32), !dbg !6470
  %history.i40.i.sroa.0.0.copyload = load float, ptr %hot_left.i36, align 4, !dbg !6475, !noalias !6477
  %history.i40.i.sroa.7.0.copyload = load float, ptr %history.i40.i.sroa.7.0.hot_left.i36.sroa_idx, align 4, !dbg !6475, !noalias !6477
  %history.i40.i.sroa.10.0.copyload = load float, ptr %history.i40.i.sroa.10.0.hot_left.i36.sroa_idx, align 4, !dbg !6475, !noalias !6477
  %history.i40.i.sroa.13.0.copyload = load float, ptr %history.i40.i.sroa.13.0.hot_left.i36.sroa_idx, align 4, !dbg !6475, !noalias !6477
  %history.i40.i.sroa.16.0.copyload = load float, ptr %history.i40.i.sroa.16.0.hot_left.i36.sroa_idx, align 4, !dbg !6475, !noalias !6477
  %history.i40.i.sroa.19.0.copyload = load float, ptr %history.i40.i.sroa.19.0.hot_left.i36.sroa_idx, align 4, !dbg !6475, !noalias !6477
  %history.i40.i.sroa.22.0.copyload = load float, ptr %history.i40.i.sroa.22.0.hot_left.i36.sroa_idx, align 4, !dbg !6475, !noalias !6477
  %history.i40.i.sroa.26.0.copyload = load float, ptr %history.i40.i.sroa.26.0.hot_left.i36.sroa_idx, align 4, !dbg !6475, !noalias !6477
  %history.i40.i.sroa.29.0.copyload = load float, ptr %history.i40.i.sroa.29.0.hot_left.i36.sroa_idx, align 4, !dbg !6475, !noalias !6477
  %history.i40.i.sroa.32.0.copyload = load float, ptr %history.i40.i.sroa.32.0.hot_left.i36.sroa_idx, align 4, !dbg !6475, !noalias !6477
  %history.i40.i.sroa.35.0.copyload = load float, ptr %history.i40.i.sroa.35.0.hot_left.i36.sroa_idx, align 4, !dbg !6475, !noalias !6477
  %history.i40.i.sroa.38.0.copyload = load float, ptr %history.i40.i.sroa.38.0.hot_left.i36.sroa_idx, align 4, !dbg !6475, !noalias !6477
  %_20.i43.i7953.not = icmp eq i32 %frames, %iter1.sroa.0.0.i568745, !dbg !6482
  br i1 %_20.i43.i7953.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i, label %bb5.i44.i.lr.ph, !dbg !6486

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
  br label %bb5.i44.i, !dbg !6486

bb5.i44.i:                                        ; preds = %bb5.i44.i.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008
  %iter.sroa.0.0.i42.i7965 = phi i32 [ 0, %bb5.i44.i.lr.ph ], [ %601, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ]
  %history.i40.i.sroa.35.07964 = phi float [ %history.i40.i.sroa.35.0.copyload, %bb5.i44.i.lr.ph ], [ %history.i40.i.sroa.32.07963, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ]
  %history.i40.i.sroa.32.07963 = phi float [ %history.i40.i.sroa.32.0.copyload, %bb5.i44.i.lr.ph ], [ %history.i40.i.sroa.29.07962, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ]
  %history.i40.i.sroa.29.07962 = phi float [ %history.i40.i.sroa.29.0.copyload, %bb5.i44.i.lr.ph ], [ %history.i40.i.sroa.26.07961, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ]
  %history.i40.i.sroa.26.07961 = phi float [ %history.i40.i.sroa.26.0.copyload, %bb5.i44.i.lr.ph ], [ %history.i40.i.sroa.22.07960, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ]
  %history.i40.i.sroa.22.07960 = phi float [ %history.i40.i.sroa.22.0.copyload, %bb5.i44.i.lr.ph ], [ %history.i40.i.sroa.19.07959, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ]
  %history.i40.i.sroa.19.07959 = phi float [ %history.i40.i.sroa.19.0.copyload, %bb5.i44.i.lr.ph ], [ %history.i40.i.sroa.16.07958, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ]
  %history.i40.i.sroa.16.07958 = phi float [ %history.i40.i.sroa.16.0.copyload, %bb5.i44.i.lr.ph ], [ %history.i40.i.sroa.13.07957, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ]
  %history.i40.i.sroa.13.07957 = phi float [ %history.i40.i.sroa.13.0.copyload, %bb5.i44.i.lr.ph ], [ %history.i40.i.sroa.10.07956, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ]
  %history.i40.i.sroa.10.07956 = phi float [ %history.i40.i.sroa.10.0.copyload, %bb5.i44.i.lr.ph ], [ %history.i40.i.sroa.7.07955, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ]
  %history.i40.i.sroa.7.07955 = phi float [ %history.i40.i.sroa.7.0.copyload, %bb5.i44.i.lr.ph ], [ %history.i40.i.sroa.0.07954, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ]
  %history.i40.i.sroa.0.07954 = phi float [ %history.i40.i.sroa.0.0.copyload, %bb5.i44.i.lr.ph ], [ %_0.i3006, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ]
  %601 = add nuw nsw i32 %iter.sroa.0.0.i42.i7965, 1, !dbg !6487
  %_11.i45.i = add nuw nsw i32 %iter.sroa.0.0.i42.i7965, %iter1.sroa.0.0.i568745, !dbg !6490
  %_24.i46.i = icmp ugt i32 %_11.i45.i, %left_io.1, !dbg !6491
  br i1 %_24.i46.i, label %bb7.i243.i, label %bb8.i47.i, !dbg !6491, !prof !755

bb8.i47.i:                                        ; preds = %bb5.i44.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6494), !dbg !6497
  %_3.not.i3004 = icmp eq i32 %left_io.1, %_11.i45.i, !dbg !6498
  br i1 %_3.not.i3004, label %panic.i3007, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008, !dbg !6498

panic.i3007:                                      ; preds = %bb8.i47.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #28, !dbg !6498, !noalias !6500
  unreachable, !dbg !6498

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008: ; preds = %bb8.i47.i
  %_31.i49.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %_11.i45.i, !dbg !6502
  %_0.i3006 = load float, ptr %_31.i49.i, align 4, !dbg !6498, !alias.scope !6494, !noalias !6504, !noundef !10
  %602 = tail call noundef float @llvm.fabs.f32(float %history.i40.i.sroa.19.07959), !dbg !6505
  %_0.i2666 = fmul float %_0.i3006, %_11.i.i.i64.i, !dbg !6508
  %_0.i2234 = fadd float %_0.i2666, 0.000000e+00, !dbg !6511
  %_0.i2665 = fmul float %_0.i3006, %_14.i.i.i67.i, !dbg !6513
  %_0.i2233 = fadd float %_0.i2665, 0.000000e+00, !dbg !6515
  %_0.i2664 = fmul float %_0.i3006, %_17.i.i.i70.i, !dbg !6517
  %_0.i2232 = fadd float %_0.i2664, 0.000000e+00, !dbg !6519
  %_0.i2663 = fmul float %_0.i3006, %_20.i.i.i73.i, !dbg !6521
  %_0.i2231 = fadd float %_0.i2663, 0.000000e+00, !dbg !6523
  %_0.i2662 = fmul float %history.i40.i.sroa.0.07954, %_25.i.i.i78.i, !dbg !6525
  %_0.i2230 = fadd float %_0.i2234, %_0.i2662, !dbg !6527
  %_0.i2661 = fmul float %history.i40.i.sroa.0.07954, %_28.i.i.i81.i, !dbg !6529
  %_0.i2229 = fadd float %_0.i2233, %_0.i2661, !dbg !6531
  %_0.i2660 = fmul float %history.i40.i.sroa.0.07954, %_31.i.i.i84.i, !dbg !6533
  %_0.i2228 = fadd float %_0.i2232, %_0.i2660, !dbg !6535
  %_0.i2659 = fmul float %history.i40.i.sroa.0.07954, %_34.i.i.i87.i, !dbg !6537
  %_0.i2227 = fadd float %_0.i2231, %_0.i2659, !dbg !6539
  %_0.i2658 = fmul float %history.i40.i.sroa.7.07955, %_39.i.i.i92.i, !dbg !6541
  %_0.i2226 = fadd float %_0.i2230, %_0.i2658, !dbg !6543
  %_0.i2657 = fmul float %history.i40.i.sroa.7.07955, %_42.i.i.i95.i, !dbg !6545
  %_0.i2225 = fadd float %_0.i2229, %_0.i2657, !dbg !6547
  %_0.i2656 = fmul float %history.i40.i.sroa.7.07955, %_45.i.i.i98.i, !dbg !6549
  %_0.i2224 = fadd float %_0.i2228, %_0.i2656, !dbg !6551
  %_0.i2655 = fmul float %history.i40.i.sroa.7.07955, %_48.i.i.i101.i, !dbg !6553
  %_0.i2223 = fadd float %_0.i2227, %_0.i2655, !dbg !6555
  %_0.i2654 = fmul float %history.i40.i.sroa.10.07956, %_53.i.i.i106.i, !dbg !6557
  %_0.i2222 = fadd float %_0.i2226, %_0.i2654, !dbg !6559
  %_0.i2653 = fmul float %history.i40.i.sroa.10.07956, %_56.i.i.i109.i, !dbg !6561
  %_0.i2221 = fadd float %_0.i2225, %_0.i2653, !dbg !6563
  %_0.i2652 = fmul float %history.i40.i.sroa.10.07956, %_59.i.i.i112.i, !dbg !6565
  %_0.i2220 = fadd float %_0.i2224, %_0.i2652, !dbg !6567
  %_0.i2651 = fmul float %history.i40.i.sroa.10.07956, %_62.i.i.i115.i, !dbg !6569
  %_0.i2219 = fadd float %_0.i2223, %_0.i2651, !dbg !6571
  %_0.i2650 = fmul float %history.i40.i.sroa.13.07957, %_67.i.i.i120.i, !dbg !6573
  %_0.i2218 = fadd float %_0.i2222, %_0.i2650, !dbg !6575
  %_0.i2649 = fmul float %history.i40.i.sroa.13.07957, %_70.i.i.i123.i, !dbg !6577
  %_0.i2217 = fadd float %_0.i2221, %_0.i2649, !dbg !6579
  %_0.i2648 = fmul float %history.i40.i.sroa.13.07957, %_73.i.i.i126.i, !dbg !6581
  %_0.i2216 = fadd float %_0.i2220, %_0.i2648, !dbg !6583
  %_0.i2647 = fmul float %history.i40.i.sroa.13.07957, %_76.i.i.i129.i, !dbg !6585
  %_0.i2215 = fadd float %_0.i2219, %_0.i2647, !dbg !6587
  %_0.i2646 = fmul float %history.i40.i.sroa.16.07958, %_81.i.i.i134.i, !dbg !6589
  %_0.i2214 = fadd float %_0.i2218, %_0.i2646, !dbg !6591
  %_0.i2645 = fmul float %history.i40.i.sroa.16.07958, %_84.i.i.i137.i, !dbg !6593
  %_0.i2213 = fadd float %_0.i2217, %_0.i2645, !dbg !6595
  %_0.i2644 = fmul float %history.i40.i.sroa.16.07958, %_87.i.i.i140.i, !dbg !6597
  %_0.i2212 = fadd float %_0.i2216, %_0.i2644, !dbg !6599
  %_0.i2643 = fmul float %history.i40.i.sroa.16.07958, %_90.i.i.i143.i, !dbg !6601
  %_0.i2211 = fadd float %_0.i2215, %_0.i2643, !dbg !6603
  %_0.i2642 = fmul float %history.i40.i.sroa.19.07959, %_95.i.i.i148.i, !dbg !6605
  %_0.i2210 = fadd float %_0.i2214, %_0.i2642, !dbg !6607
  %_0.i2641 = fmul float %history.i40.i.sroa.19.07959, %_98.i.i.i151.i, !dbg !6609
  %_0.i2209 = fadd float %_0.i2213, %_0.i2641, !dbg !6611
  %_0.i2640 = fmul float %history.i40.i.sroa.19.07959, %_101.i.i.i154.i, !dbg !6613
  %_0.i2208 = fadd float %_0.i2212, %_0.i2640, !dbg !6615
  %_0.i2639 = fmul float %history.i40.i.sroa.19.07959, %_104.i.i.i157.i, !dbg !6617
  %_0.i2207 = fadd float %_0.i2211, %_0.i2639, !dbg !6619
  %_0.i2638 = fmul float %history.i40.i.sroa.22.07960, %_109.i.i.i162.i, !dbg !6621
  %_0.i2206 = fadd float %_0.i2210, %_0.i2638, !dbg !6623
  %_0.i2637 = fmul float %history.i40.i.sroa.22.07960, %_112.i.i.i165.i, !dbg !6625
  %_0.i2205 = fadd float %_0.i2209, %_0.i2637, !dbg !6627
  %_0.i2636 = fmul float %history.i40.i.sroa.22.07960, %_115.i.i.i168.i, !dbg !6629
  %_0.i2204 = fadd float %_0.i2208, %_0.i2636, !dbg !6631
  %_0.i2635 = fmul float %history.i40.i.sroa.22.07960, %_118.i.i.i171.i, !dbg !6633
  %_0.i2203 = fadd float %_0.i2207, %_0.i2635, !dbg !6635
  %_0.i2634 = fmul float %history.i40.i.sroa.26.07961, %_123.i.i.i176.i, !dbg !6637
  %_0.i2202 = fadd float %_0.i2206, %_0.i2634, !dbg !6639
  %_0.i2633 = fmul float %history.i40.i.sroa.26.07961, %_126.i.i.i179.i, !dbg !6641
  %_0.i2201 = fadd float %_0.i2205, %_0.i2633, !dbg !6643
  %_0.i2632 = fmul float %history.i40.i.sroa.26.07961, %_129.i.i.i182.i, !dbg !6645
  %_0.i2200 = fadd float %_0.i2204, %_0.i2632, !dbg !6647
  %_0.i2631 = fmul float %history.i40.i.sroa.26.07961, %_132.i.i.i185.i, !dbg !6649
  %_0.i2199 = fadd float %_0.i2203, %_0.i2631, !dbg !6651
  %_0.i2630 = fmul float %history.i40.i.sroa.29.07962, %_137.i.i.i190.i, !dbg !6653
  %_0.i2198 = fadd float %_0.i2202, %_0.i2630, !dbg !6655
  %_0.i2629 = fmul float %history.i40.i.sroa.29.07962, %_140.i.i.i193.i, !dbg !6657
  %_0.i2197 = fadd float %_0.i2201, %_0.i2629, !dbg !6659
  %_0.i2628 = fmul float %history.i40.i.sroa.29.07962, %_143.i.i.i196.i, !dbg !6661
  %_0.i2196 = fadd float %_0.i2200, %_0.i2628, !dbg !6663
  %_0.i2627 = fmul float %history.i40.i.sroa.29.07962, %_146.i.i.i199.i, !dbg !6665
  %_0.i2195 = fadd float %_0.i2199, %_0.i2627, !dbg !6667
  %_0.i2626 = fmul float %history.i40.i.sroa.32.07963, %_151.i.i.i204.i, !dbg !6669
  %_0.i2194 = fadd float %_0.i2198, %_0.i2626, !dbg !6671
  %_0.i2625 = fmul float %history.i40.i.sroa.32.07963, %_154.i.i.i207.i, !dbg !6673
  %_0.i2193 = fadd float %_0.i2197, %_0.i2625, !dbg !6675
  %_0.i2624 = fmul float %history.i40.i.sroa.32.07963, %_157.i.i.i210.i, !dbg !6677
  %_0.i2192 = fadd float %_0.i2196, %_0.i2624, !dbg !6679
  %_0.i2623 = fmul float %history.i40.i.sroa.32.07963, %_160.i.i.i213.i, !dbg !6681
  %_0.i2191 = fadd float %_0.i2195, %_0.i2623, !dbg !6683
  %_0.i2622 = fmul float %history.i40.i.sroa.35.07964, %_165.i.i.i218.i, !dbg !6685
  %_0.i2190 = fadd float %_0.i2194, %_0.i2622, !dbg !6687
  %_0.i2621 = fmul float %history.i40.i.sroa.35.07964, %_168.i.i.i221.i, !dbg !6689
  %_0.i2189 = fadd float %_0.i2193, %_0.i2621, !dbg !6691
  %_0.i2620 = fmul float %history.i40.i.sroa.35.07964, %_171.i.i.i224.i, !dbg !6693
  %_0.i2188 = fadd float %_0.i2192, %_0.i2620, !dbg !6695
  %_0.i2619 = fmul float %history.i40.i.sroa.35.07964, %_174.i.i.i227.i, !dbg !6697
  %_0.i2187 = fadd float %_0.i2191, %_0.i2619, !dbg !6699
  %603 = tail call noundef float @llvm.fabs.f32(float %_0.i2190), !dbg !6701
  %_3.i.i3638.inv = fcmp ogt float %602, %603, !dbg !6703
  %_4.i.i3645.v = select i1 %_3.i.i3638.inv, float %602, float %603, !dbg !6703
  %604 = tail call noundef float @llvm.fabs.f32(float %_0.i2189), !dbg !6701
  %_3.i.i3638.inv.1 = fcmp ogt float %_4.i.i3645.v, %604, !dbg !6703
  %_4.i.i3645.v.1 = select i1 %_3.i.i3638.inv.1, float %_4.i.i3645.v, float %604, !dbg !6703
  %605 = tail call noundef float @llvm.fabs.f32(float %_0.i2188), !dbg !6701
  %_3.i.i3638.inv.2 = fcmp ogt float %_4.i.i3645.v.1, %605, !dbg !6703
  %_4.i.i3645.v.2 = select i1 %_3.i.i3638.inv.2, float %_4.i.i3645.v.1, float %605, !dbg !6703
  %606 = tail call noundef float @llvm.fabs.f32(float %_0.i2187), !dbg !6701
  %_3.i.i3638.inv.3 = fcmp ogt float %_4.i.i3645.v.2, %606, !dbg !6703
  %_4.i.i3645.v.3 = select i1 %_3.i.i3638.inv.3, float %_4.i.i3645.v.2, float %606, !dbg !6703
  %_39.i238.i = getelementptr inbounds nuw float, ptr %peaks_left.i34, i32 %iter.sroa.0.0.i42.i7965, !dbg !6706
  store float %_4.i.i3645.v.3, ptr %_39.i238.i, align 4, !dbg !6711, !alias.scope !6713, !noalias !6504
  %exitcond11936.not = icmp eq i32 %601, %umax11938, !dbg !6482
  br i1 %exitcond11936.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i, label %bb5.i44.i, !dbg !6486

bb7.i243.i:                                       ; preds = %bb5.i44.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_11.i45.i, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #28, !dbg !6716, !noalias !6504
  unreachable, !dbg !6716

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008, %bb44.i59
  %history.i40.i.sroa.0.0.lcssa = phi float [ %history.i40.i.sroa.0.0.copyload, %bb44.i59 ], [ %_0.i3006, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ], !dbg !6717
  %history.i40.i.sroa.7.0.lcssa = phi float [ %history.i40.i.sroa.7.0.copyload, %bb44.i59 ], [ %history.i40.i.sroa.0.07954, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ], !dbg !6717
  %history.i40.i.sroa.10.0.lcssa = phi float [ %history.i40.i.sroa.10.0.copyload, %bb44.i59 ], [ %history.i40.i.sroa.7.07955, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ], !dbg !6717
  %history.i40.i.sroa.13.0.lcssa = phi float [ %history.i40.i.sroa.13.0.copyload, %bb44.i59 ], [ %history.i40.i.sroa.10.07956, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ], !dbg !6717
  %history.i40.i.sroa.16.0.lcssa = phi float [ %history.i40.i.sroa.16.0.copyload, %bb44.i59 ], [ %history.i40.i.sroa.13.07957, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ], !dbg !6717
  %history.i40.i.sroa.19.0.lcssa = phi float [ %history.i40.i.sroa.19.0.copyload, %bb44.i59 ], [ %history.i40.i.sroa.16.07958, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ], !dbg !6717
  %history.i40.i.sroa.22.0.lcssa = phi float [ %history.i40.i.sroa.22.0.copyload, %bb44.i59 ], [ %history.i40.i.sroa.19.07959, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ], !dbg !6717
  %history.i40.i.sroa.26.0.lcssa = phi float [ %history.i40.i.sroa.26.0.copyload, %bb44.i59 ], [ %history.i40.i.sroa.22.07960, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ], !dbg !6717
  %history.i40.i.sroa.29.0.lcssa = phi float [ %history.i40.i.sroa.29.0.copyload, %bb44.i59 ], [ %history.i40.i.sroa.26.07961, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ], !dbg !6717
  %history.i40.i.sroa.32.0.lcssa = phi float [ %history.i40.i.sroa.32.0.copyload, %bb44.i59 ], [ %history.i40.i.sroa.29.07962, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ], !dbg !6717
  %history.i40.i.sroa.35.0.lcssa = phi float [ %history.i40.i.sroa.35.0.copyload, %bb44.i59 ], [ %history.i40.i.sroa.32.07963, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ], !dbg !6717
  %history.i40.i.sroa.38.0.lcssa = phi float [ %history.i40.i.sroa.38.0.copyload, %bb44.i59 ], [ %history.i40.i.sroa.35.07964, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3008 ], !dbg !6717
  store float %history.i40.i.sroa.0.0.lcssa, ptr %hot_left.i36, align 4, !dbg !6718, !noalias !6477
  store float %history.i40.i.sroa.7.0.lcssa, ptr %history.i40.i.sroa.7.0.hot_left.i36.sroa_idx, align 4, !dbg !6718, !noalias !6477
  store float %history.i40.i.sroa.10.0.lcssa, ptr %history.i40.i.sroa.10.0.hot_left.i36.sroa_idx, align 4, !dbg !6718, !noalias !6477
  store float %history.i40.i.sroa.13.0.lcssa, ptr %history.i40.i.sroa.13.0.hot_left.i36.sroa_idx, align 4, !dbg !6718, !noalias !6477
  store float %history.i40.i.sroa.16.0.lcssa, ptr %history.i40.i.sroa.16.0.hot_left.i36.sroa_idx, align 4, !dbg !6718, !noalias !6477
  store float %history.i40.i.sroa.19.0.lcssa, ptr %history.i40.i.sroa.19.0.hot_left.i36.sroa_idx, align 4, !dbg !6718, !noalias !6477
  store float %history.i40.i.sroa.22.0.lcssa, ptr %history.i40.i.sroa.22.0.hot_left.i36.sroa_idx, align 4, !dbg !6718, !noalias !6477
  store float %history.i40.i.sroa.26.0.lcssa, ptr %history.i40.i.sroa.26.0.hot_left.i36.sroa_idx, align 4, !dbg !6718, !noalias !6477
  store float %history.i40.i.sroa.29.0.lcssa, ptr %history.i40.i.sroa.29.0.hot_left.i36.sroa_idx, align 4, !dbg !6718, !noalias !6477
  store float %history.i40.i.sroa.32.0.lcssa, ptr %history.i40.i.sroa.32.0.hot_left.i36.sroa_idx, align 4, !dbg !6718, !noalias !6477
  store float %history.i40.i.sroa.35.0.lcssa, ptr %history.i40.i.sroa.35.0.hot_left.i36.sroa_idx, align 4, !dbg !6718, !noalias !6477
  store float %history.i40.i.sroa.38.0.lcssa, ptr %history.i40.i.sroa.38.0.hot_left.i36.sroa_idx, align 4, !dbg !6718, !noalias !6477
  %history.i.i13.sroa.0.0.copyload = load float, ptr %hot_right.i35, align 4, !dbg !6719, !noalias !6721
  %history.i.i13.sroa.7.0.copyload = load float, ptr %history.i.i13.sroa.7.0.hot_right.i35.sroa_idx, align 4, !dbg !6719, !noalias !6721
  %history.i.i13.sroa.10.0.copyload = load float, ptr %history.i.i13.sroa.10.0.hot_right.i35.sroa_idx, align 4, !dbg !6719, !noalias !6721
  %history.i.i13.sroa.13.0.copyload = load float, ptr %history.i.i13.sroa.13.0.hot_right.i35.sroa_idx, align 4, !dbg !6719, !noalias !6721
  %history.i.i13.sroa.16.0.copyload = load float, ptr %history.i.i13.sroa.16.0.hot_right.i35.sroa_idx, align 4, !dbg !6719, !noalias !6721
  %history.i.i13.sroa.19.0.copyload = load float, ptr %history.i.i13.sroa.19.0.hot_right.i35.sroa_idx, align 4, !dbg !6719, !noalias !6721
  %history.i.i13.sroa.22.0.copyload = load float, ptr %history.i.i13.sroa.22.0.hot_right.i35.sroa_idx, align 4, !dbg !6719, !noalias !6721
  %history.i.i13.sroa.26.0.copyload = load float, ptr %history.i.i13.sroa.26.0.hot_right.i35.sroa_idx, align 4, !dbg !6719, !noalias !6721
  %history.i.i13.sroa.29.0.copyload = load float, ptr %history.i.i13.sroa.29.0.hot_right.i35.sroa_idx, align 4, !dbg !6719, !noalias !6721
  %history.i.i13.sroa.32.0.copyload = load float, ptr %history.i.i13.sroa.32.0.hot_right.i35.sroa_idx, align 4, !dbg !6719, !noalias !6721
  %history.i.i13.sroa.35.0.copyload = load float, ptr %history.i.i13.sroa.35.0.hot_right.i35.sroa_idx, align 4, !dbg !6719, !noalias !6721
  %history.i.i13.sroa.38.0.copyload = load float, ptr %history.i.i13.sroa.38.0.hot_right.i35.sroa_idx, align 4, !dbg !6719, !noalias !6721
  br i1 %_20.i43.i7953.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i64, label %bb5.i.i212.lr.ph, !dbg !6726

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
  br label %bb5.i.i212, !dbg !6726

bb5.i.i212:                                       ; preds = %bb5.i.i212.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013
  %iter.sroa.0.0.i.i627992 = phi i32 [ 0, %bb5.i.i212.lr.ph ], [ %607, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ]
  %history.i.i13.sroa.35.07991 = phi float [ %history.i.i13.sroa.35.0.copyload, %bb5.i.i212.lr.ph ], [ %history.i.i13.sroa.32.07990, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ]
  %history.i.i13.sroa.32.07990 = phi float [ %history.i.i13.sroa.32.0.copyload, %bb5.i.i212.lr.ph ], [ %history.i.i13.sroa.29.07989, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ]
  %history.i.i13.sroa.29.07989 = phi float [ %history.i.i13.sroa.29.0.copyload, %bb5.i.i212.lr.ph ], [ %history.i.i13.sroa.26.07988, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ]
  %history.i.i13.sroa.26.07988 = phi float [ %history.i.i13.sroa.26.0.copyload, %bb5.i.i212.lr.ph ], [ %history.i.i13.sroa.22.07987, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ]
  %history.i.i13.sroa.22.07987 = phi float [ %history.i.i13.sroa.22.0.copyload, %bb5.i.i212.lr.ph ], [ %history.i.i13.sroa.19.07986, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ]
  %history.i.i13.sroa.19.07986 = phi float [ %history.i.i13.sroa.19.0.copyload, %bb5.i.i212.lr.ph ], [ %history.i.i13.sroa.16.07985, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ]
  %history.i.i13.sroa.16.07985 = phi float [ %history.i.i13.sroa.16.0.copyload, %bb5.i.i212.lr.ph ], [ %history.i.i13.sroa.13.07984, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ]
  %history.i.i13.sroa.13.07984 = phi float [ %history.i.i13.sroa.13.0.copyload, %bb5.i.i212.lr.ph ], [ %history.i.i13.sroa.10.07983, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ]
  %history.i.i13.sroa.10.07983 = phi float [ %history.i.i13.sroa.10.0.copyload, %bb5.i.i212.lr.ph ], [ %history.i.i13.sroa.7.07982, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ]
  %history.i.i13.sroa.7.07982 = phi float [ %history.i.i13.sroa.7.0.copyload, %bb5.i.i212.lr.ph ], [ %history.i.i13.sroa.0.07981, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ]
  %history.i.i13.sroa.0.07981 = phi float [ %history.i.i13.sroa.0.0.copyload, %bb5.i.i212.lr.ph ], [ %_0.i3011, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ]
  %607 = add nuw nsw i32 %iter.sroa.0.0.i.i627992, 1, !dbg !6729
  %_11.i31.i = add nuw nsw i32 %iter.sroa.0.0.i.i627992, %iter1.sroa.0.0.i568745, !dbg !6732
  %_24.i.i213 = icmp ugt i32 %_11.i31.i, %right_io.1, !dbg !6733
  br i1 %_24.i.i213, label %bb7.i.i410, label %bb8.i.i214, !dbg !6733, !prof !755

bb8.i.i214:                                       ; preds = %bb5.i.i212
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6736), !dbg !6739
  %_3.not.i3009 = icmp eq i32 %right_io.1, %_11.i31.i, !dbg !6740
  br i1 %_3.not.i3009, label %panic.i3012, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013, !dbg !6740

panic.i3012:                                      ; preds = %bb8.i.i214
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #28, !dbg !6740, !noalias !6742
  unreachable, !dbg !6740

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013: ; preds = %bb8.i.i214
  %_31.i.i216 = getelementptr inbounds nuw float, ptr %right_io.0, i32 %_11.i31.i, !dbg !6744
  %_0.i3011 = load float, ptr %_31.i.i216, align 4, !dbg !6740, !alias.scope !6736, !noalias !6746, !noundef !10
  %608 = tail call noundef float @llvm.fabs.f32(float %history.i.i13.sroa.19.07986), !dbg !6747
  %_0.i2714 = fmul float %_0.i3011, %_11.i.i.i.i231, !dbg !6750
  %_0.i2282 = fadd float %_0.i2714, 0.000000e+00, !dbg !6753
  %_0.i2713 = fmul float %_0.i3011, %_14.i.i.i.i234, !dbg !6755
  %_0.i2281 = fadd float %_0.i2713, 0.000000e+00, !dbg !6757
  %_0.i2712 = fmul float %_0.i3011, %_17.i.i.i.i237, !dbg !6759
  %_0.i2280 = fadd float %_0.i2712, 0.000000e+00, !dbg !6761
  %_0.i2711 = fmul float %_0.i3011, %_20.i.i.i.i240, !dbg !6763
  %_0.i2279 = fadd float %_0.i2711, 0.000000e+00, !dbg !6765
  %_0.i2710 = fmul float %history.i.i13.sroa.0.07981, %_25.i.i.i.i245, !dbg !6767
  %_0.i2278 = fadd float %_0.i2282, %_0.i2710, !dbg !6769
  %_0.i2709 = fmul float %history.i.i13.sroa.0.07981, %_28.i.i.i.i248, !dbg !6771
  %_0.i2277 = fadd float %_0.i2281, %_0.i2709, !dbg !6773
  %_0.i2708 = fmul float %history.i.i13.sroa.0.07981, %_31.i.i.i.i251, !dbg !6775
  %_0.i2276 = fadd float %_0.i2280, %_0.i2708, !dbg !6777
  %_0.i2707 = fmul float %history.i.i13.sroa.0.07981, %_34.i.i.i.i254, !dbg !6779
  %_0.i2275 = fadd float %_0.i2279, %_0.i2707, !dbg !6781
  %_0.i2706 = fmul float %history.i.i13.sroa.7.07982, %_39.i.i.i.i259, !dbg !6783
  %_0.i2274 = fadd float %_0.i2278, %_0.i2706, !dbg !6785
  %_0.i2705 = fmul float %history.i.i13.sroa.7.07982, %_42.i.i.i.i262, !dbg !6787
  %_0.i2273 = fadd float %_0.i2277, %_0.i2705, !dbg !6789
  %_0.i2704 = fmul float %history.i.i13.sroa.7.07982, %_45.i.i.i.i265, !dbg !6791
  %_0.i2272 = fadd float %_0.i2276, %_0.i2704, !dbg !6793
  %_0.i2703 = fmul float %history.i.i13.sroa.7.07982, %_48.i.i.i.i268, !dbg !6795
  %_0.i2271 = fadd float %_0.i2275, %_0.i2703, !dbg !6797
  %_0.i2702 = fmul float %history.i.i13.sroa.10.07983, %_53.i.i.i.i273, !dbg !6799
  %_0.i2270 = fadd float %_0.i2274, %_0.i2702, !dbg !6801
  %_0.i2701 = fmul float %history.i.i13.sroa.10.07983, %_56.i.i.i.i276, !dbg !6803
  %_0.i2269 = fadd float %_0.i2273, %_0.i2701, !dbg !6805
  %_0.i2700 = fmul float %history.i.i13.sroa.10.07983, %_59.i.i.i.i279, !dbg !6807
  %_0.i2268 = fadd float %_0.i2272, %_0.i2700, !dbg !6809
  %_0.i2699 = fmul float %history.i.i13.sroa.10.07983, %_62.i.i.i.i282, !dbg !6811
  %_0.i2267 = fadd float %_0.i2271, %_0.i2699, !dbg !6813
  %_0.i2698 = fmul float %history.i.i13.sroa.13.07984, %_67.i.i.i.i287, !dbg !6815
  %_0.i2266 = fadd float %_0.i2270, %_0.i2698, !dbg !6817
  %_0.i2697 = fmul float %history.i.i13.sroa.13.07984, %_70.i.i.i.i290, !dbg !6819
  %_0.i2265 = fadd float %_0.i2269, %_0.i2697, !dbg !6821
  %_0.i2696 = fmul float %history.i.i13.sroa.13.07984, %_73.i.i.i.i293, !dbg !6823
  %_0.i2264 = fadd float %_0.i2268, %_0.i2696, !dbg !6825
  %_0.i2695 = fmul float %history.i.i13.sroa.13.07984, %_76.i.i.i.i296, !dbg !6827
  %_0.i2263 = fadd float %_0.i2267, %_0.i2695, !dbg !6829
  %_0.i2694 = fmul float %history.i.i13.sroa.16.07985, %_81.i.i.i.i301, !dbg !6831
  %_0.i2262 = fadd float %_0.i2266, %_0.i2694, !dbg !6833
  %_0.i2693 = fmul float %history.i.i13.sroa.16.07985, %_84.i.i.i.i304, !dbg !6835
  %_0.i2261 = fadd float %_0.i2265, %_0.i2693, !dbg !6837
  %_0.i2692 = fmul float %history.i.i13.sroa.16.07985, %_87.i.i.i.i307, !dbg !6839
  %_0.i2260 = fadd float %_0.i2264, %_0.i2692, !dbg !6841
  %_0.i2691 = fmul float %history.i.i13.sroa.16.07985, %_90.i.i.i.i310, !dbg !6843
  %_0.i2259 = fadd float %_0.i2263, %_0.i2691, !dbg !6845
  %_0.i2690 = fmul float %history.i.i13.sroa.19.07986, %_95.i.i.i.i315, !dbg !6847
  %_0.i2258 = fadd float %_0.i2262, %_0.i2690, !dbg !6849
  %_0.i2689 = fmul float %history.i.i13.sroa.19.07986, %_98.i.i.i.i318, !dbg !6851
  %_0.i2257 = fadd float %_0.i2261, %_0.i2689, !dbg !6853
  %_0.i2688 = fmul float %history.i.i13.sroa.19.07986, %_101.i.i.i.i321, !dbg !6855
  %_0.i2256 = fadd float %_0.i2260, %_0.i2688, !dbg !6857
  %_0.i2687 = fmul float %history.i.i13.sroa.19.07986, %_104.i.i.i.i324, !dbg !6859
  %_0.i2255 = fadd float %_0.i2259, %_0.i2687, !dbg !6861
  %_0.i2686 = fmul float %history.i.i13.sroa.22.07987, %_109.i.i.i.i329, !dbg !6863
  %_0.i2254 = fadd float %_0.i2258, %_0.i2686, !dbg !6865
  %_0.i2685 = fmul float %history.i.i13.sroa.22.07987, %_112.i.i.i.i332, !dbg !6867
  %_0.i2253 = fadd float %_0.i2257, %_0.i2685, !dbg !6869
  %_0.i2684 = fmul float %history.i.i13.sroa.22.07987, %_115.i.i.i.i335, !dbg !6871
  %_0.i2252 = fadd float %_0.i2256, %_0.i2684, !dbg !6873
  %_0.i2683 = fmul float %history.i.i13.sroa.22.07987, %_118.i.i.i.i338, !dbg !6875
  %_0.i2251 = fadd float %_0.i2255, %_0.i2683, !dbg !6877
  %_0.i2682 = fmul float %history.i.i13.sroa.26.07988, %_123.i.i.i.i343, !dbg !6879
  %_0.i2250 = fadd float %_0.i2254, %_0.i2682, !dbg !6881
  %_0.i2681 = fmul float %history.i.i13.sroa.26.07988, %_126.i.i.i.i346, !dbg !6883
  %_0.i2249 = fadd float %_0.i2253, %_0.i2681, !dbg !6885
  %_0.i2680 = fmul float %history.i.i13.sroa.26.07988, %_129.i.i.i.i349, !dbg !6887
  %_0.i2248 = fadd float %_0.i2252, %_0.i2680, !dbg !6889
  %_0.i2679 = fmul float %history.i.i13.sroa.26.07988, %_132.i.i.i.i352, !dbg !6891
  %_0.i2247 = fadd float %_0.i2251, %_0.i2679, !dbg !6893
  %_0.i2678 = fmul float %history.i.i13.sroa.29.07989, %_137.i.i.i.i357, !dbg !6895
  %_0.i2246 = fadd float %_0.i2250, %_0.i2678, !dbg !6897
  %_0.i2677 = fmul float %history.i.i13.sroa.29.07989, %_140.i.i.i.i360, !dbg !6899
  %_0.i2245 = fadd float %_0.i2249, %_0.i2677, !dbg !6901
  %_0.i2676 = fmul float %history.i.i13.sroa.29.07989, %_143.i.i.i.i363, !dbg !6903
  %_0.i2244 = fadd float %_0.i2248, %_0.i2676, !dbg !6905
  %_0.i2675 = fmul float %history.i.i13.sroa.29.07989, %_146.i.i.i.i366, !dbg !6907
  %_0.i2243 = fadd float %_0.i2247, %_0.i2675, !dbg !6909
  %_0.i2674 = fmul float %history.i.i13.sroa.32.07990, %_151.i.i.i.i371, !dbg !6911
  %_0.i2242 = fadd float %_0.i2246, %_0.i2674, !dbg !6913
  %_0.i2673 = fmul float %history.i.i13.sroa.32.07990, %_154.i.i.i.i374, !dbg !6915
  %_0.i2241 = fadd float %_0.i2245, %_0.i2673, !dbg !6917
  %_0.i2672 = fmul float %history.i.i13.sroa.32.07990, %_157.i.i.i.i377, !dbg !6919
  %_0.i2240 = fadd float %_0.i2244, %_0.i2672, !dbg !6921
  %_0.i2671 = fmul float %history.i.i13.sroa.32.07990, %_160.i.i.i.i380, !dbg !6923
  %_0.i2239 = fadd float %_0.i2243, %_0.i2671, !dbg !6925
  %_0.i2670 = fmul float %history.i.i13.sroa.35.07991, %_165.i.i.i.i385, !dbg !6927
  %_0.i2238 = fadd float %_0.i2242, %_0.i2670, !dbg !6929
  %_0.i2669 = fmul float %history.i.i13.sroa.35.07991, %_168.i.i.i.i388, !dbg !6931
  %_0.i2237 = fadd float %_0.i2241, %_0.i2669, !dbg !6933
  %_0.i2668 = fmul float %history.i.i13.sroa.35.07991, %_171.i.i.i.i391, !dbg !6935
  %_0.i2236 = fadd float %_0.i2240, %_0.i2668, !dbg !6937
  %_0.i2667 = fmul float %history.i.i13.sroa.35.07991, %_174.i.i.i.i394, !dbg !6939
  %_0.i2235 = fadd float %_0.i2239, %_0.i2667, !dbg !6941
  %609 = tail call noundef float @llvm.fabs.f32(float %_0.i2238), !dbg !6943
  %_3.i.i3647.inv = fcmp ogt float %608, %609, !dbg !6945
  %_4.i.i3654.v = select i1 %_3.i.i3647.inv, float %608, float %609, !dbg !6945
  %610 = tail call noundef float @llvm.fabs.f32(float %_0.i2237), !dbg !6943
  %_3.i.i3647.inv.1 = fcmp ogt float %_4.i.i3654.v, %610, !dbg !6945
  %_4.i.i3654.v.1 = select i1 %_3.i.i3647.inv.1, float %_4.i.i3654.v, float %610, !dbg !6945
  %611 = tail call noundef float @llvm.fabs.f32(float %_0.i2236), !dbg !6943
  %_3.i.i3647.inv.2 = fcmp ogt float %_4.i.i3654.v.1, %611, !dbg !6945
  %_4.i.i3654.v.2 = select i1 %_3.i.i3647.inv.2, float %_4.i.i3654.v.1, float %611, !dbg !6945
  %612 = tail call noundef float @llvm.fabs.f32(float %_0.i2235), !dbg !6943
  %_3.i.i3647.inv.3 = fcmp ogt float %_4.i.i3654.v.2, %612, !dbg !6945
  %_4.i.i3654.v.3 = select i1 %_3.i.i3647.inv.3, float %_4.i.i3654.v.2, float %612, !dbg !6945
  %_39.i.i405 = getelementptr inbounds nuw float, ptr %peaks_right.i33, i32 %iter.sroa.0.0.i.i627992, !dbg !6948
  store float %_4.i.i3654.v.3, ptr %_39.i.i405, align 4, !dbg !6953, !alias.scope !6955, !noalias !6746
  %exitcond11939.not = icmp eq i32 %607, %umax11938, !dbg !6958
  br i1 %exitcond11939.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i64, label %bb5.i.i212, !dbg !6726

bb7.i.i410:                                       ; preds = %bb5.i.i212
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_11.i31.i, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #28, !dbg !6960, !noalias !6746
  unreachable, !dbg !6960

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i64: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i
  %history.i.i13.sroa.0.0.lcssa = phi float [ %history.i.i13.sroa.0.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i ], [ %_0.i3011, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ], !dbg !6961
  %history.i.i13.sroa.7.0.lcssa = phi float [ %history.i.i13.sroa.7.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i ], [ %history.i.i13.sroa.0.07981, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ], !dbg !6961
  %history.i.i13.sroa.10.0.lcssa = phi float [ %history.i.i13.sroa.10.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i ], [ %history.i.i13.sroa.7.07982, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ], !dbg !6961
  %history.i.i13.sroa.13.0.lcssa = phi float [ %history.i.i13.sroa.13.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i ], [ %history.i.i13.sroa.10.07983, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ], !dbg !6961
  %history.i.i13.sroa.16.0.lcssa = phi float [ %history.i.i13.sroa.16.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i ], [ %history.i.i13.sroa.13.07984, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ], !dbg !6961
  %history.i.i13.sroa.19.0.lcssa = phi float [ %history.i.i13.sroa.19.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i ], [ %history.i.i13.sroa.16.07985, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ], !dbg !6961
  %history.i.i13.sroa.22.0.lcssa = phi float [ %history.i.i13.sroa.22.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i ], [ %history.i.i13.sroa.19.07986, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ], !dbg !6961
  %history.i.i13.sroa.26.0.lcssa = phi float [ %history.i.i13.sroa.26.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i ], [ %history.i.i13.sroa.22.07987, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ], !dbg !6961
  %history.i.i13.sroa.29.0.lcssa = phi float [ %history.i.i13.sroa.29.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i ], [ %history.i.i13.sroa.26.07988, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ], !dbg !6961
  %history.i.i13.sroa.32.0.lcssa = phi float [ %history.i.i13.sroa.32.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i ], [ %history.i.i13.sroa.29.07989, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ], !dbg !6961
  %history.i.i13.sroa.35.0.lcssa = phi float [ %history.i.i13.sroa.35.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i ], [ %history.i.i13.sroa.32.07990, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ], !dbg !6961
  %history.i.i13.sroa.38.0.lcssa = phi float [ %history.i.i13.sroa.38.0.copyload, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit244.i ], [ %history.i.i13.sroa.35.07991, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3013 ], !dbg !6961
  store float %history.i.i13.sroa.0.0.lcssa, ptr %hot_right.i35, align 4, !dbg !6962, !noalias !6721
  store float %history.i.i13.sroa.7.0.lcssa, ptr %history.i.i13.sroa.7.0.hot_right.i35.sroa_idx, align 4, !dbg !6962, !noalias !6721
  store float %history.i.i13.sroa.10.0.lcssa, ptr %history.i.i13.sroa.10.0.hot_right.i35.sroa_idx, align 4, !dbg !6962, !noalias !6721
  store float %history.i.i13.sroa.13.0.lcssa, ptr %history.i.i13.sroa.13.0.hot_right.i35.sroa_idx, align 4, !dbg !6962, !noalias !6721
  store float %history.i.i13.sroa.16.0.lcssa, ptr %history.i.i13.sroa.16.0.hot_right.i35.sroa_idx, align 4, !dbg !6962, !noalias !6721
  store float %history.i.i13.sroa.19.0.lcssa, ptr %history.i.i13.sroa.19.0.hot_right.i35.sroa_idx, align 4, !dbg !6962, !noalias !6721
  store float %history.i.i13.sroa.22.0.lcssa, ptr %history.i.i13.sroa.22.0.hot_right.i35.sroa_idx, align 4, !dbg !6962, !noalias !6721
  store float %history.i.i13.sroa.26.0.lcssa, ptr %history.i.i13.sroa.26.0.hot_right.i35.sroa_idx, align 4, !dbg !6962, !noalias !6721
  store float %history.i.i13.sroa.29.0.lcssa, ptr %history.i.i13.sroa.29.0.hot_right.i35.sroa_idx, align 4, !dbg !6962, !noalias !6721
  store float %history.i.i13.sroa.32.0.lcssa, ptr %history.i.i13.sroa.32.0.hot_right.i35.sroa_idx, align 4, !dbg !6962, !noalias !6721
  store float %history.i.i13.sroa.35.0.lcssa, ptr %history.i.i13.sroa.35.0.hot_right.i35.sroa_idx, align 4, !dbg !6962, !noalias !6721
  store float %history.i.i13.sroa.38.0.lcssa, ptr %history.i.i13.sroa.38.0.hot_right.i35.sroa_idx, align 4, !dbg !6962, !noalias !6721
  br i1 %_20.i43.i7953.not, label %bb15.i53.loopexit, label %bb20.i70.lr.ph, !dbg !6458

bb20.i70.lr.ph:                                   ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i64
  %_68.i29.sroa.3.0.copyload = load i32, ptr %_68.i29.sroa.3.0..sroa_idx, align 4, !noalias !6402
  %_68.i29.sroa.4.0.copyload = load i32, ptr %_68.i29.sroa.4.0..sroa_idx, align 4, !noalias !6402
  %_69.i28.sroa.3.0.copyload = load i32, ptr %_69.i28.sroa.3.0..sroa_idx, align 4, !noalias !6402
  %_69.i28.sroa.4.0.copyload = load i32, ptr %_69.i28.sroa.4.0..sroa_idx, align 4, !noalias !6402
  %_54.0.i257.i = load ptr, ptr %uniform_left.i32, align 4, !nonnull !10, !align !6963
  %_54.1.i258.i = load i32, ptr %582, align 4
  %_18.i268.i = load i32, ptr %568, align 4
  %_29.i11928005.not = icmp eq i32 %_18.i268.i, 0
  %_56.0.i279.i = load ptr, ptr %583, align 4, !nonnull !10, !align !6963
  %_56.1.i280.i = load i32, ptr %584, align 4
  %_58.1.i305.i = load i32, ptr %588, align 4
  %_58.0.i304.i = load ptr, ptr %589, align 4, !nonnull !10, !align !6963
  %_54.0.i.i147 = load ptr, ptr %uniform_right.i31, align 4, !nonnull !10, !align !6963
  %_54.1.i.i148 = load i32, ptr %590, align 4
  %_18.i.i158 = load i32, ptr %569, align 4
  %_29.i11708009.not = icmp eq i32 %_18.i.i158, 0
  %_56.0.i.i165 = load ptr, ptr %591, align 4, !nonnull !10, !align !6963
  %_56.1.i.i166 = load i32, ptr %592, align 4
  %_58.1.i.i189 = load i32, ptr %596, align 4
  %_58.0.i.i188 = load ptr, ptr %597, align 4, !nonnull !10, !align !6963
  %_22.i272.i.promoted8598 = load i32, ptr %_22.i272.i, align 4
  %_21.i271.i.promoted8634 = load float, ptr %_21.i271.i, align 4
  %_22.i.i160.promoted8670 = load i32, ptr %_22.i.i160, align 4
  %_21.i.i159.promoted8706 = load float, ptr %_21.i.i159, align 4
  %_13.i193345294531 = load float, ptr %572, align 4
  %_13.i192145324534 = load float, ptr %575, align 4
  %_13.i190945354537 = load float, ptr %578, align 4
  %_13.i189745384540 = load float, ptr %581, align 4
  %_37.i292.i = load float, ptr %586, align 4
  %_37.i.i176 = load float, ptr %594, align 4
  %.promoted14048 = load float, ptr %570, align 4
  %_110.i111.promoted14066 = load float, ptr %_110.i111, align 4
  %.promoted14084 = load float, ptr %571, align 4
  %.promoted14102 = load float, ptr %573, align 4
  %_111.i112.promoted14120 = load float, ptr %_111.i112, align 4
  %.promoted14138 = load float, ptr %574, align 4
  %.promoted14156 = load float, ptr %576, align 4
  %_115.i113.promoted14174 = load float, ptr %_115.i113, align 4
  %.promoted14192 = load float, ptr %577, align 4
  %.promoted14210 = load float, ptr %579, align 4
  %_116.i114.promoted14228 = load float, ptr %_116.i114, align 4
  %.promoted14246 = load float, ptr %580, align 4
  %.promoted14264 = load float, ptr %585, align 4
  %.promoted14282 = load float, ptr %587, align 4
  %.promoted14300 = load float, ptr %593, align 4
  %.promoted14318 = load float, ptr %595, align 4
  br label %bb20.i70, !dbg !6458

bb20.i70:                                         ; preds = %bb20.i70.lr.ph, %bb67.i203
  %_0.i3224.lcssa1275514320 = phi float [ %.promoted14318, %bb20.i70.lr.ph ], [ %_0.i3224.lcssa1275514319, %bb67.i203 ]
  %_0.i2850.lcssa1275414302 = phi float [ %.promoted14300, %bb20.i70.lr.ph ], [ %_0.i2850.lcssa1275414301, %bb67.i203 ]
  %_0.i3228.lcssa1273014284 = phi float [ %.promoted14282, %bb20.i70.lr.ph ], [ %_0.i3228.lcssa1273014283, %bb67.i203 ]
  %_0.i2854.lcssa1271414266 = phi float [ %.promoted14264, %bb20.i70.lr.ph ], [ %_0.i2854.lcssa1271414265, %bb67.i203 ]
  %_0.i3279.lcssa1252914248 = phi float [ %.promoted14246, %bb20.i70.lr.ph ], [ %_0.i3279.lcssa1252914247, %bb67.i203 ]
  %_0.i3286.lcssa1254314230 = phi float [ %_116.i114.promoted14228, %bb20.i70.lr.ph ], [ %_0.i3286.lcssa1254314229, %bb67.i203 ]
  %_0.i.i3520.lcssa1255714212 = phi float [ %.promoted14210, %bb20.i70.lr.ph ], [ %_0.i.i3520.lcssa1255714211, %bb67.i203 ]
  %_0.i3266.lcssa1257114194 = phi float [ %.promoted14192, %bb20.i70.lr.ph ], [ %_0.i3266.lcssa1257114193, %bb67.i203 ]
  %_0.i3273.lcssa1258514176 = phi float [ %_115.i113.promoted14174, %bb20.i70.lr.ph ], [ %_0.i3273.lcssa1258514175, %bb67.i203 ]
  %_0.i.i3513.lcssa1259914158 = phi float [ %.promoted14156, %bb20.i70.lr.ph ], [ %_0.i.i3513.lcssa1259914157, %bb67.i203 ]
  %_0.i3253.lcssa1261314140 = phi float [ %.promoted14138, %bb20.i70.lr.ph ], [ %_0.i3253.lcssa1261314139, %bb67.i203 ]
  %_0.i3260.lcssa1262714122 = phi float [ %_111.i112.promoted14120, %bb20.i70.lr.ph ], [ %_0.i3260.lcssa1262714121, %bb67.i203 ]
  %_0.i.i3506.lcssa1264114104 = phi float [ %.promoted14102, %bb20.i70.lr.ph ], [ %_0.i.i3506.lcssa1264114103, %bb67.i203 ]
  %_0.i3241.lcssa1265514086 = phi float [ %.promoted14084, %bb20.i70.lr.ph ], [ %_0.i3241.lcssa1265514085, %bb67.i203 ]
  %_0.i3247.lcssa1266914068 = phi float [ %_110.i111.promoted14066, %bb20.i70.lr.ph ], [ %_0.i3247.lcssa1266914067, %bb67.i203 ]
  %_0.i.i.lcssa1268314050 = phi float [ %.promoted14048, %bb20.i70.lr.ph ], [ %_0.i.i.lcssa1268314049, %bb67.i203 ]
  %running.sroa.0.0.i.lcssa85298708 = phi float [ %_21.i.i159.promoted8706, %bb20.i70.lr.ph ], [ %running.sroa.0.0.i.lcssa85298707, %bb67.i203 ]
  %storemerge.i.lcssa85028672 = phi i32 [ %_22.i.i160.promoted8670, %bb20.i70.lr.ph ], [ %storemerge.i.lcssa85028671, %bb67.i203 ]
  %running.sroa.0.0.i1180.lcssa84178636 = phi float [ %_21.i271.i.promoted8634, %bb20.i70.lr.ph ], [ %running.sroa.0.0.i1180.lcssa84178635, %bb67.i203 ]
  %storemerge.i1185.lcssa83908600 = phi i32 [ %_22.i272.i.promoted8598, %bb20.i70.lr.ph ], [ %storemerge.i1185.lcssa83908599, %bb67.i203 ]
  %frame.sroa.0.0.i688595 = phi i32 [ 0, %bb20.i70.lr.ph ], [ %_83.i83, %bb67.i203 ]
  %main_cursor.sroa.0.1.i678594 = phi i32 [ %main_cursor.sroa.0.0.i558744, %bb20.i70.lr.ph ], [ %main_cursor.sroa.0.2.i209, %bb67.i203 ]
  %ring_cursor.sroa.0.1.i668593 = phi i32 [ %ring_cursor.sroa.0.0.i548743, %bb20.i70.lr.ph ], [ %ring_cursor.sroa.0.2.i206, %bb67.i203 ]
  %_65.i71 = sub nuw nsw i32 %spec.store.select.i60, %frame.sroa.0.0.i688595, !dbg !6964
  %ring.i1795 = load i32, ptr %528, align 4, !dbg !6965, !alias.scope !6968, !noalias !6971, !noundef !10
  %main.i1796 = load i32, ptr %529, align 4, !dbg !6975, !alias.scope !6968, !noalias !6971, !noundef !10
  %_10.i = add i32 %ring_cursor.sroa.0.1.i668593, 1, !dbg !6977
  %_38.not.i = icmp ult i32 %_10.i, %ring.i1795, !dbg !6979
  %613 = select i1 %_38.not.i, i32 0, i32 %ring.i1795, !dbg !6979
  %start1.sroa.0.0.i1797 = sub nuw i32 %_10.i, %613, !dbg !6979
  %_12.i1798 = add i32 %_68.i29.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i668593, !dbg !6982
  %_39.not.i = icmp ult i32 %_12.i1798, %ring.i1795, !dbg !6984
  %614 = select i1 %_39.not.i, i32 0, i32 %ring.i1795, !dbg !6984
  %left_end.sroa.0.0.i = sub nuw i32 %_12.i1798, %614, !dbg !6984
  %_15.i1799 = add i32 %_69.i28.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i668593, !dbg !6986
  %_40.not.i = icmp ult i32 %_15.i1799, %ring.i1795, !dbg !6988
  %615 = select i1 %_40.not.i, i32 0, i32 %ring.i1795, !dbg !6988
  %right_end.sroa.0.0.i = sub nuw i32 %_15.i1799, %615, !dbg !6988
  %_18.i = add i32 %_68.i29.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i668593, !dbg !6990
  %_41.not.i = icmp ult i32 %_18.i, %ring.i1795, !dbg !6992
  %616 = select i1 %_41.not.i, i32 0, i32 %ring.i1795, !dbg !6992
  %left_expiring.sroa.0.0.i = sub nuw i32 %_18.i, %616, !dbg !6992
  %_21.i = add i32 %_69.i28.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i668593, !dbg !6994
  %_42.not.i = icmp ult i32 %_21.i, %ring.i1795, !dbg !6996
  %617 = select i1 %_42.not.i, i32 0, i32 %ring.i1795, !dbg !6996
  %right_expiring.sroa.0.0.i = sub nuw i32 %_21.i, %617, !dbg !6996
  %618 = sub i32 %ring.i1795, %ring_cursor.sroa.0.1.i668593, !dbg !6998
  %spec.store.select.i1800 = tail call i32 @llvm.umin.i32(i32 %618, i32 %_65.i71), !dbg !7000
  %619 = sub i32 %main.i1796, %main_cursor.sroa.0.1.i678594, !dbg !7003
  %_24.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %619, i32 %spec.store.select.i1800), !dbg !7004
  %620 = sub i32 %ring.i1795, %start1.sroa.0.0.i1797, !dbg !7006
  %_25.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %620, i32 %_24.sroa.0.0.i), !dbg !7007
  %621 = sub i32 %ring.i1795, %left_end.sroa.0.0.i, !dbg !7009
  %_27.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %621, i32 %_25.sroa.0.0.i), !dbg !7010
  %622 = sub i32 %ring.i1795, %right_end.sroa.0.0.i, !dbg !7012
  %_29.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %622, i32 %_27.sroa.0.0.i), !dbg !7013
  %623 = sub i32 %ring.i1795, %left_expiring.sroa.0.0.i, !dbg !7015
  %_31.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %623, i32 %_29.sroa.0.0.i), !dbg !7016
  %624 = sub i32 %ring.i1795, %right_expiring.sroa.0.0.i, !dbg !7018
  %run.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %624, i32 %_31.sroa.0.0.i), !dbg !7019
  %_72.i73 = add i32 %frame.sroa.0.0.i688595, %iter1.sroa.0.0.i568745, !dbg !7021
  %_76.i74 = add i32 %run.sroa.0.0.i, %_72.i73, !dbg !7024
  %_172.i75 = icmp ult i32 %_76.i74, %_72.i73, !dbg !7027
  %_166.not.i76 = icmp ugt i32 %_76.i74, %left_io.1
  %or.cond.i77 = or i1 %_172.i75, %_166.not.i76, !dbg !7027
  br i1 %or.cond.i77, label %bb51.i211, label %bb49.i78, !dbg !7027, !prof !3544

bb51.i211:                                        ; preds = %bb20.i70
  store float %_0.i.i.lcssa1268314050, ptr %570, align 4
  store float %_0.i3247.lcssa1266914068, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265514086, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264114104, ptr %573, align 4
  store float %_0.i3260.lcssa1262714122, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261314140, ptr %574, align 4
  store float %_0.i.i3513.lcssa1259914158, ptr %576, align 4
  store float %_0.i3273.lcssa1258514176, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257114194, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255714212, ptr %579, align 4
  store float %_0.i3286.lcssa1254314230, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1252914248, ptr %580, align 4
  store float %_0.i2854.lcssa1271414266, ptr %585, align 4
  store float %_0.i3228.lcssa1273014284, ptr %587, align 4
  store float %_0.i2850.lcssa1275414302, ptr %593, align 4
  store float %_0.i3224.lcssa1275514320, ptr %595, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_72.i73, i32 noundef %_76.i74, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_bd3c2aadcc2cdd6816f02be0ceab2cc1) #28, !dbg !7034, !noalias !6396
  unreachable, !dbg !7034

bb49.i78:                                         ; preds = %bb20.i70
  %_175.i79 = getelementptr inbounds nuw float, ptr %left_io.0, i32 %_72.i73, !dbg !7035
  %_176.not.i80 = icmp ugt i32 %_76.i74, %right_io.1, !dbg !7039
  br i1 %_176.not.i80, label %bb54.i210, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit, !dbg !7039, !prof !755

bb54.i210:                                        ; preds = %bb49.i78
  store float %_0.i.i.lcssa1268314050, ptr %570, align 4
  store float %_0.i3247.lcssa1266914068, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265514086, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264114104, ptr %573, align 4
  store float %_0.i3260.lcssa1262714122, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261314140, ptr %574, align 4
  store float %_0.i.i3513.lcssa1259914158, ptr %576, align 4
  store float %_0.i3273.lcssa1258514176, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257114194, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255714212, ptr %579, align 4
  store float %_0.i3286.lcssa1254314230, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1252914248, ptr %580, align 4
  store float %_0.i2854.lcssa1271414266, ptr %585, align 4
  store float %_0.i3228.lcssa1273014284, ptr %587, align 4
  store float %_0.i2850.lcssa1275414302, ptr %593, align 4
  store float %_0.i3224.lcssa1275514320, ptr %595, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_72.i73, i32 noundef %_76.i74, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c1156ffa98f252967759477badafe965) #28, !dbg !7044, !noalias !6396
  unreachable, !dbg !7044

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit: ; preds = %bb49.i78
  %_183.i82 = getelementptr inbounds nuw float, ptr %right_io.0, i32 %_72.i73, !dbg !7045
  %_83.i83 = add nuw nsw i32 %run.sroa.0.0.i, %frame.sroa.0.0.i688595, !dbg !7049
  %_192.i89 = getelementptr inbounds nuw float, ptr %peaks_left.i34, i32 %frame.sroa.0.0.i688595, !dbg !7051
  %_201.i90 = getelementptr inbounds nuw float, ptr %peaks_right.i33, i32 %frame.sroa.0.0.i688595, !dbg !7061
  %_2.i8013.not = icmp eq i32 %run.sroa.0.0.i, 0, !dbg !7071
  br i1 %_2.i8013.not, label %bb67.i203, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph, !dbg !7071

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph: ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit
  %umax11942 = call i32 @llvm.umax.i32(i32 %ring_cursor.sroa.0.1.i668593, i32 %_54.1.i258.i), !dbg !7071
  %umax11943 = call i32 @llvm.umax.i32(i32 %main_cursor.sroa.0.1.i678594, i32 %_58.1.i305.i), !dbg !7071
  %625 = sub i32 %umax11942, %ring_cursor.sroa.0.1.i668593, !dbg !7071
  %626 = sub i32 %umax11943, %main_cursor.sroa.0.1.i678594, !dbg !7071
  %umin11946 = call i32 @llvm.umin.i32(i32 %621, i32 %622), !dbg !7071
  %umin11947 = call i32 @llvm.umin.i32(i32 %umin11946, i32 %623), !dbg !7071
  %umin11948 = call i32 @llvm.umin.i32(i32 %umin11947, i32 %624), !dbg !7071
  %umin11949 = call i32 @llvm.umin.i32(i32 %umin11948, i32 %620), !dbg !7071
  %umin11950 = call i32 @llvm.umin.i32(i32 %umin11949, i32 %618), !dbg !7071
  %umin11951 = call i32 @llvm.umin.i32(i32 %umin11950, i32 %619), !dbg !7071
  %627 = sub nsw i32 %umin11952, %frame.sroa.0.0.i688595, !dbg !7071
  %umin11953 = call i32 @llvm.umin.i32(i32 %umin11951, i32 %627), !dbg !7071
  br label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018, !dbg !7071

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165
  %_0.i32248564 = phi float [ %_0.i3224.lcssa1275514320, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i3224, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i28508535 = phi float [ %_0.i2850.lcssa1275414302, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i2850, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %running.sroa.0.0.i8507 = phi float [ %running.sroa.0.0.i.lcssa85298708, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %running.sroa.0.0.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %storemerge.i8480 = phi i32 [ %storemerge.i.lcssa85028672, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %storemerge.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i32288452 = phi float [ %_0.i3228.lcssa1273014284, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i3228, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i28548423 = phi float [ %_0.i2854.lcssa1271414266, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i2854, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %running.sroa.0.0.i11808395 = phi float [ %running.sroa.0.0.i1180.lcssa84178636, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %running.sroa.0.0.i1180, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %storemerge.i11858368 = phi i32 [ %storemerge.i1185.lcssa83908600, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %storemerge.i1185, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_12.i18958339 = phi float [ %_0.i3279.lcssa1252914248, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i3279, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ], !dbg !7081
  %_0.i32868310 = phi float [ %_0.i3286.lcssa1254314230, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i3286, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ], !dbg !7081
  %_5.i18898281 = phi float [ %_0.i.i3520.lcssa1255714212, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i.i3520, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ], !dbg !7081
  %_12.i19078251 = phi float [ %_0.i3266.lcssa1257114194, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i3266, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ], !dbg !7081
  %_0.i32738222 = phi float [ %_0.i3273.lcssa1258514176, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i3273, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ], !dbg !7081
  %_5.i19018193 = phi float [ %_0.i.i3513.lcssa1259914158, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i.i3513, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ], !dbg !7081
  %_12.i19198163 = phi float [ %_0.i3253.lcssa1261314140, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i3253, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ], !dbg !7081
  %_0.i32608134 = phi float [ %_0.i3260.lcssa1262714122, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i3260, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ], !dbg !7081
  %_5.i19138105 = phi float [ %_0.i.i3506.lcssa1264114104, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i.i3506, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ], !dbg !7081
  %_12.i19318075 = phi float [ %_0.i3241.lcssa1265514086, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i3241, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ], !dbg !7081
  %_0.i32478046 = phi float [ %_0.i3247.lcssa1266914068, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i3247, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ], !dbg !7081
  %_5.i19258017 = phi float [ %_0.i.i.lcssa1268314050, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_0.i.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ], !dbg !7081
  %iter.i19.sroa.41.08015 = phi i32 [ 0, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018.lr.ph ], [ %_206.0.i110, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_206.0.i110 = add nuw i32 %iter.i19.sroa.41.08015, 1, !dbg !7083
  %data.i.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_175.i79, i32 %iter.i19.sroa.41.08015, !dbg !7086
  %data.i5.i.i.i.i.i = getelementptr inbounds nuw float, ptr %_183.i82, i32 %iter.i19.sroa.41.08015, !dbg !7104
  %data.i.i.i.i = getelementptr inbounds nuw float, ptr %_192.i89, i32 %iter.i19.sroa.41.08015, !dbg !7107
  %data.i.i4094 = getelementptr inbounds nuw float, ptr %_201.i90, i32 %iter.i19.sroa.41.08015, !dbg !7114
  %_0.i2823 = fadd float %_5.i19258017, -1.000000e+00, !dbg !7117
  %_3.i.i = fcmp ogt float %_0.i2823, 0.000000e+00, !dbg !7122
  %_0.i.i = select i1 %_3.i.i, float %_0.i2823, float 0.000000e+00, !dbg !7125
  %_0.i1983 = fadd float %_0.i32478046, %_12.i19318075, !dbg !7127
  %_0.i3247 = select i1 %_3.i.i, float %_0.i1983, float %_13.i193345294531, !dbg !7129
  %_0.i3241 = select i1 %_3.i.i, float %_12.i19318075, float 0.000000e+00, !dbg !7131
  %_0.i2824 = fadd float %_5.i19138105, -1.000000e+00, !dbg !7133
  %_3.i.i3500 = fcmp ogt float %_0.i2824, 0.000000e+00, !dbg !7136
  %_0.i.i3506 = select i1 %_3.i.i3500, float %_0.i2824, float 0.000000e+00, !dbg !7139
  %_0.i1984 = fadd float %_0.i32608134, %_12.i19198163, !dbg !7141
  %_0.i3260 = select i1 %_3.i.i3500, float %_0.i1984, float %_13.i192145324534, !dbg !7143
  %_0.i3253 = select i1 %_3.i.i3500, float %_12.i19198163, float 0.000000e+00, !dbg !7145
  %_0.i2825 = fadd float %_5.i19018193, -1.000000e+00, !dbg !7147
  %_3.i.i3507 = fcmp ogt float %_0.i2825, 0.000000e+00, !dbg !7152
  %_0.i.i3513 = select i1 %_3.i.i3507, float %_0.i2825, float 0.000000e+00, !dbg !7155
  %_0.i1985 = fadd float %_0.i32738222, %_12.i19078251, !dbg !7157
  %_0.i3273 = select i1 %_3.i.i3507, float %_0.i1985, float %_13.i190945354537, !dbg !7159
  %_0.i3266 = select i1 %_3.i.i3507, float %_12.i19078251, float 0.000000e+00, !dbg !7161
  %_0.i2826 = fadd float %_5.i18898281, -1.000000e+00, !dbg !7163
  %_3.i.i3514 = fcmp ogt float %_0.i2826, 0.000000e+00, !dbg !7166
  %_0.i.i3520 = select i1 %_3.i.i3514, float %_0.i2826, float 0.000000e+00, !dbg !7169
  %_0.i1986 = fadd float %_0.i32868310, %_12.i18958339, !dbg !7171
  %_0.i3286 = select i1 %_3.i.i3514, float %_0.i1986, float %_13.i189745384540, !dbg !7173
  %_0.i3279 = select i1 %_3.i.i3514, float %_12.i18958339, float 0.000000e+00, !dbg !7175
  %_0.i3031 = load float, ptr %data.i.i.i.i, align 4, !dbg !7177, !alias.scope !7180, !noalias !6396, !noundef !10
  %_0.i3026 = load float, ptr %data.i.i4094, align 4, !dbg !7183, !alias.scope !7186, !noalias !6396, !noundef !10
  %_3.i.i3674 = fcmp ule float %_0.i3026, %_0.i3031, !dbg !7189
  %_6.i.i3676 = bitcast float %_0.i3026 to i32, !dbg !7193
  %_8.i.i3678 = bitcast float %_0.i3031 to i32, !dbg !7196
  %_4.i.i3681 = select i1 %_3.i.i3674, i32 %_8.i.i3678, i32 %_6.i.i3676, !dbg !7198
  %_5.i3452 = and i32 %_4.i.i3681, %.none.i41, !dbg !7199
  %_7.i3448 = and i32 %_9.i3454, %_6.i.i3676, !dbg !7202
  %_4.i3449 = or disjoint i32 %_5.i3452, %_7.i3448, !dbg !7205
  %_0.i3450 = bitcast i32 %_4.i3449 to float, !dbg !7206
  %_0.i3021 = load float, ptr %data.i.i.i.i.i.i, align 4, !dbg !7208, !alias.scope !7211, !noalias !6396, !noundef !10
  %_0.i3016 = load float, ptr %data.i5.i.i.i.i.i, align 4, !dbg !7214, !alias.scope !7217, !noalias !6396, !noundef !10
  %_213.i128 = add nuw i32 %iter.i19.sroa.41.08015, %ring_cursor.sroa.0.1.i668593, !dbg !7220
  %_214.i129 = add nuw i32 %iter.i19.sroa.41.08015, %main_cursor.sroa.0.1.i678594, !dbg !7225
  %_215.i130 = add nuw i32 %iter.i19.sroa.41.08015, %left_end.sroa.0.0.i, !dbg !7226
  %_216.i131 = add i32 %iter.i19.sroa.41.08015, %start1.sroa.0.0.i1797, !dbg !7227
  %_217.i132 = add nuw i32 %iter.i19.sroa.41.08015, %left_expiring.sroa.0.0.i, !dbg !7228
  %_7.i8.i260.i = add i32 %_213.i128, 1, !dbg !7229
  %exitcond11944.not = icmp eq i32 %iter.i19.sroa.41.08015, %625, !dbg !7237
  br i1 %exitcond11944.not, label %bb4.i13.i319.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i264.i, !dbg !7237, !prof !3544

bb4.i13.i319.i:                                   ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018
  store float %_0.i.i.lcssa1268314050, ptr %570, align 4
  store float %_0.i3247.lcssa1266914068, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265514086, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264114104, ptr %573, align 4
  store float %_0.i3260.lcssa1262714122, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261314140, ptr %574, align 4
  store float %_0.i.i3513.lcssa1259914158, ptr %576, align 4
  store float %_0.i3273.lcssa1258514176, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257114194, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255714212, ptr %579, align 4
  store float %_0.i3286.lcssa1254314230, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1252914248, ptr %580, align 4
  store float %_0.i2854.lcssa1271414266, ptr %585, align 4
  store float %_0.i3228.lcssa1273014284, ptr %587, align 4
  store float %_0.i2850.lcssa1275414302, ptr %593, align 4
  store float %_0.i3224.lcssa1275514320, ptr %595, align 4
  %628 = add i32 %umax11942, 1, !dbg !7071
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_213.i128, i32 noundef %628, i32 noundef range(i32 0, 536870912) %_54.1.i258.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #28, !dbg !7244, !noalias !7245
  unreachable, !dbg !7244

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i264.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018
  %_7.i3455 = and i32 %_9.i3454, %_8.i.i3678, !dbg !7253
  %_4.i3456 = or disjoint i32 %_5.i3452, %_7.i3455, !dbg !7199
  %_0.i3457 = bitcast i32 %_4.i3456 to float, !dbg !7254
  %_0.i2394 = fdiv float %_0.i3247, %_0.i3457, !dbg !7256
  %_3.i1961 = fcmp uge float %_0.i3247, %_0.i3457, !dbg !7258
  %_0.i3443 = select i1 %_3.i1961, float 1.000000e+00, float %_0.i2394, !dbg !7260
  %_17.i12.i265.i = getelementptr inbounds nuw float, ptr %_54.0.i257.i, i32 %_213.i128, !dbg !7262
  store float %_0.i3443, ptr %_17.i12.i265.i, align 4, !dbg !7266, !alias.scope !7268, !noalias !7271
  %or.cond.i1351.not = icmp ult i32 %_215.i130, %_54.1.i258.i, !dbg !7272
  br i1 %or.cond.i1351.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1355, label %bb4.i1354, !dbg !7272, !prof !7284

bb4.i1354:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i264.i
  store float %_0.i.i.lcssa1268314050, ptr %570, align 4
  store float %_0.i3247.lcssa1266914068, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265514086, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264114104, ptr %573, align 4
  store float %_0.i3260.lcssa1262714122, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261314140, ptr %574, align 4
  store float %_0.i.i3513.lcssa1259914158, ptr %576, align 4
  store float %_0.i3273.lcssa1258514176, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257114194, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255714212, ptr %579, align 4
  store float %_0.i3286.lcssa1254314230, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1252914248, ptr %580, align 4
  store float %_0.i2854.lcssa1271414266, ptr %585, align 4
  store float %_0.i3228.lcssa1273014284, ptr %587, align 4
  store float %_0.i2850.lcssa1275414302, ptr %593, align 4
  store float %_0.i3224.lcssa1275514320, ptr %595, align 4
  %_5.i1348 = add i32 %_215.i130, 1, !dbg !7285
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_215.i130, i32 noundef %_5.i1348, i32 noundef range(i32 0, 536870912) %_54.1.i258.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #28, !dbg !7286, !noalias !7287
  unreachable, !dbg !7286

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1355: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i264.i
  %_15.i1352 = getelementptr inbounds nuw float, ptr %_54.0.i257.i, i32 %_215.i130, !dbg !7293
  %_0.i2888 = load float, ptr %_15.i1352, align 4, !dbg !7297, !alias.scope !7299, !noalias !7302, !noundef !10
  %629 = icmp eq i32 %storemerge.i11858368, 0, !dbg !7303
  %_3.i.i3800.inv = fcmp olt float %running.sroa.0.0.i11808395, %_0.i2888, !dbg !7303
  %_4.i.i3807.v = select i1 %_3.i.i3800.inv, float %running.sroa.0.0.i11808395, float %_0.i2888, !dbg !7303
  %running.sroa.0.0.i1180 = select i1 %629, float %_0.i2888, float %_4.i.i3807.v, !dbg !7303
  %_15.i1181 = add i32 %storemerge.i11858368, 1, !dbg !7306
  %complete.i1182 = icmp eq i32 %_15.i1181, %_18.i268.i, !dbg !7306
  br i1 %complete.i1182, label %bb11.i1188.preheader, label %bb7.i1183, !dbg !7308

bb11.i1188.preheader:                             ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1355
  br i1 %_29.i11928005.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1204, label %bb19.i1193, !dbg !7310

bb7.i1183:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1355
  %or.cond.i1343.not = icmp ult i32 %_216.i131, %_54.1.i258.i, !dbg !7320
  br i1 %or.cond.i1343.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1347, label %bb4.i1346, !dbg !7320, !prof !7284

bb4.i1346:                                        ; preds = %bb7.i1183
  store float %_0.i.i.lcssa1268314050, ptr %570, align 4
  store float %_0.i3247.lcssa1266914068, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265514086, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264114104, ptr %573, align 4
  store float %_0.i3260.lcssa1262714122, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261314140, ptr %574, align 4
  store float %_0.i.i3513.lcssa1259914158, ptr %576, align 4
  store float %_0.i3273.lcssa1258514176, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257114194, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255714212, ptr %579, align 4
  store float %_0.i3286.lcssa1254314230, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1252914248, ptr %580, align 4
  store float %_0.i2854.lcssa1271414266, ptr %585, align 4
  store float %_0.i3228.lcssa1273014284, ptr %587, align 4
  store float %_0.i2850.lcssa1275414302, ptr %593, align 4
  store float %_0.i3224.lcssa1275514320, ptr %595, align 4
  %_5.i1340 = add i32 %_216.i131, 1, !dbg !7325
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_216.i131, i32 noundef %_5.i1340, i32 noundef range(i32 0, 536870912) %_54.1.i258.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #28, !dbg !7326, !noalias !7327
  unreachable, !dbg !7326

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1347: ; preds = %bb7.i1183
  %_15.i1344 = getelementptr inbounds nuw float, ptr %_54.0.i257.i, i32 %_216.i131, !dbg !7330
  %_0.i2890 = load float, ptr %_15.i1344, align 4, !dbg !7332, !alias.scope !7334, !noalias !7302, !noundef !10
  %_3.i.i3791.inv = fcmp olt float %_0.i2890, %running.sroa.0.0.i1180, !dbg !7337
  %_4.i.i3798.v = select i1 %_3.i.i3791.inv, float %_0.i2890, float %running.sroa.0.0.i1180, !dbg !7337
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1204, !dbg !7341

bb19.i1193:                                       ; preds = %bb11.i1188.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1200
  %end.sroa.0.0.i11918008 = phi i32 [ %631, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1200 ], [ %_215.i130, %bb11.i1188.preheader ]
  %suffix.sroa.0.0.i11908007 = phi float [ %_4.i.i3789.v, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1200 ], [ %_0.i2888, %bb11.i1188.preheader ]
  %iter.sroa.0.0.i11898006 = phi i32 [ %_30.i1194, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1200 ], [ 0, %bb11.i1188.preheader ]
  %or.cond.i1327.not = icmp ult i32 %end.sroa.0.0.i11918008, %_54.1.i258.i, !dbg !7342
  br i1 %or.cond.i1327.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1200, label %bb4.i1330, !dbg !7342, !prof !7284

bb4.i1330:                                        ; preds = %bb19.i1193
  store float %_0.i.i.lcssa1268314050, ptr %570, align 4
  store float %_0.i3247.lcssa1266914068, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265514086, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264114104, ptr %573, align 4
  store float %_0.i3260.lcssa1262714122, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261314140, ptr %574, align 4
  store float %_0.i.i3513.lcssa1259914158, ptr %576, align 4
  store float %_0.i3273.lcssa1258514176, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257114194, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255714212, ptr %579, align 4
  store float %_0.i3286.lcssa1254314230, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1252914248, ptr %580, align 4
  store float %_0.i2854.lcssa1271414266, ptr %585, align 4
  store float %_0.i3228.lcssa1273014284, ptr %587, align 4
  store float %_0.i2850.lcssa1275414302, ptr %593, align 4
  store float %_0.i3224.lcssa1275514320, ptr %595, align 4
  %_5.i1324 = add i32 %end.sroa.0.0.i11918008, 1, !dbg !7347
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %end.sroa.0.0.i11918008, i32 noundef %_5.i1324, i32 noundef range(i32 0, 536870912) %_54.1.i258.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #28, !dbg !7348, !noalias !7349
  unreachable, !dbg !7348

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1200: ; preds = %bb19.i1193
  %_30.i1194 = add nuw i32 %iter.sroa.0.0.i11898006, 1, !dbg !7352
  %_15.i1328 = getelementptr inbounds nuw float, ptr %_54.0.i257.i, i32 %end.sroa.0.0.i11918008, !dbg !7358
  %_0.i2894 = load float, ptr %_15.i1328, align 4, !dbg !7360, !alias.scope !7362, !noalias !7302, !noundef !10
  %_3.i.i3782.inv = fcmp olt float %suffix.sroa.0.0.i11908007, %_0.i2894, !dbg !7365
  %_4.i.i3789.v = select i1 %_3.i.i3782.inv, float %suffix.sroa.0.0.i11908007, float %_0.i2894, !dbg !7365
  store float %_4.i.i3789.v, ptr %_15.i1328, align 4, !dbg !7368, !alias.scope !7371, !noalias !7302
  %630 = icmp eq i32 %end.sroa.0.0.i11918008, 0, !dbg !7374
  %spec.store.select.i1202 = select i1 %630, i32 %ring.i44, i32 %end.sroa.0.0.i11918008, !dbg !7374
  %631 = add i32 %spec.store.select.i1202, -1, !dbg !7375
  %exitcond11940.not = icmp eq i32 %_30.i1194, %_18.i268.i, !dbg !7376
  br i1 %exitcond11940.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1204, label %bb19.i1193, !dbg !7310

_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1204: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1200, %bb11.i1188.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1347
  %storemerge.i1185 = phi i32 [ %_15.i1181, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1347 ], [ 0, %bb11.i1188.preheader ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1200 ], !dbg !7379
  %running.sroa.0.1.i1186 = phi float [ %_4.i.i3798.v, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1347 ], [ %running.sroa.0.0.i1180, %bb11.i1188.preheader ], [ %running.sroa.0.0.i1180, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1200 ], !dbg !7380
  %_0.i2720 = fmul float %running.sroa.0.1.i1186, 1.638400e+04, !dbg !7381
  %632 = tail call noundef float @llvm.floor.f32(float %_0.i2720), !dbg !7384
  %_0.i2719 = fmul float %632, 0x3F10000000000000, !dbg !7388
  %or.cond.i1415.not = icmp ult i32 %_217.i132, %_56.1.i280.i, !dbg !7390
  br i1 %or.cond.i1415.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1419, label %bb4.i1418, !dbg !7390, !prof !7284

bb4.i1418:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1204
  store float %_0.i.i.lcssa1268314050, ptr %570, align 4
  store float %_0.i3247.lcssa1266914068, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265514086, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264114104, ptr %573, align 4
  store float %_0.i3260.lcssa1262714122, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261314140, ptr %574, align 4
  store float %_0.i.i3513.lcssa1259914158, ptr %576, align 4
  store float %_0.i3273.lcssa1258514176, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257114194, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255714212, ptr %579, align 4
  store float %_0.i3286.lcssa1254314230, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1252914248, ptr %580, align 4
  store float %_0.i2854.lcssa1271414266, ptr %585, align 4
  store float %_0.i3228.lcssa1273014284, ptr %587, align 4
  store float %_0.i2850.lcssa1275414302, ptr %593, align 4
  store float %_0.i3224.lcssa1275514320, ptr %595, align 4
  %_5.i1412 = add i32 %_217.i132, 1, !dbg !7396
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_217.i132, i32 noundef %_5.i1412, i32 noundef range(i32 0, 536870912) %_56.1.i280.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #28, !dbg !7397, !noalias !7398
  unreachable, !dbg !7397

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1419: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1204
  %_15.i1416 = getelementptr inbounds nuw float, ptr %_56.0.i279.i, i32 %_217.i132, !dbg !7401
  %_0.i2872 = load float, ptr %_15.i1416, align 4, !dbg !7403, !alias.scope !7405, !noalias !7408, !noundef !10
  %_0.i2284 = fadd float %_0.i2719, %_0.i28548423, !dbg !7409
  %_0.i2854 = fsub float %_0.i2284, %_0.i2872, !dbg !7412
  %_8.not.i3.i289.i = icmp ugt i32 %_7.i8.i260.i, %_56.1.i280.i
  br i1 %_8.not.i3.i289.i, label %bb4.i6.i318.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i290.i, !dbg !7414, !prof !3544

bb4.i6.i318.i:                                    ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1419
  store float %_0.i.i.lcssa1268314050, ptr %570, align 4
  store float %_0.i3247.lcssa1266914068, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265514086, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264114104, ptr %573, align 4
  store float %_0.i3260.lcssa1262714122, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261314140, ptr %574, align 4
  store float %_0.i.i3513.lcssa1259914158, ptr %576, align 4
  store float %_0.i3273.lcssa1258514176, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257114194, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255714212, ptr %579, align 4
  store float %_0.i3286.lcssa1254314230, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1252914248, ptr %580, align 4
  store float %_0.i2854.lcssa1271414266, ptr %585, align 4
  store float %_0.i3228.lcssa1273014284, ptr %587, align 4
  store float %_0.i2850.lcssa1275414302, ptr %593, align 4
  store float %_0.i3224.lcssa1275514320, ptr %595, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_213.i128, i32 noundef %_7.i8.i260.i, i32 noundef range(i32 0, 536870912) %_56.1.i280.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #28, !dbg !7419, !noalias !7420
  unreachable, !dbg !7419

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i290.i: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1419
  %_17.i5.i291.i = getelementptr inbounds nuw float, ptr %_56.0.i279.i, i32 %_213.i128, !dbg !7423
  store float %_0.i2719, ptr %_17.i5.i291.i, align 4, !dbg !7425, !alias.scope !7427, !noalias !7408
  %_0.i2393 = fdiv float %_0.i2854, %_37.i292.i, !dbg !7430
  %_0.i2853 = fsub float 1.000000e+00, %_0.i2393, !dbg !7432
  %_0.i2852 = fsub float %_0.i2853, %_0.i32288452, !dbg !7435
  %_4.i2409 = fmul float %_0.i3260, %_0.i2852, !dbg !7438
  %_0.i2410 = fadd float %_0.i32288452, %_4.i2409, !dbg !7438
  %_3.i.i3665.inv = fcmp ogt float %_0.i2853, %_0.i2410, !dbg !7440
  %_4.i.i3672.v = select i1 %_3.i.i3665.inv, float %_0.i2853, float %_0.i2410, !dbg !7440
  %633 = tail call noundef float @llvm.fabs.f32(float %_4.i.i3672.v), !dbg !7444
  %634 = fcmp uge float %633, 0x3BC79CA100000000, !dbg !7448
  %_0.i3228 = select i1 %634, float %_4.i.i3672.v, float 0.000000e+00, !dbg !7450
  %_5.i1404 = add i32 %_214.i129, 1, !dbg !7451
  %exitcond11945.not = icmp eq i32 %iter.i19.sroa.41.08015, %626, !dbg !7454
  br i1 %exitcond11945.not, label %bb4.i1410, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3172, !dbg !7454, !prof !3544

bb4.i1410:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i290.i
  store float %_0.i.i.lcssa1268314050, ptr %570, align 4
  store float %_0.i3247.lcssa1266914068, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265514086, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264114104, ptr %573, align 4
  store float %_0.i3260.lcssa1262714122, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261314140, ptr %574, align 4
  store float %_0.i.i3513.lcssa1259914158, ptr %576, align 4
  store float %_0.i3273.lcssa1258514176, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257114194, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255714212, ptr %579, align 4
  store float %_0.i3286.lcssa1254314230, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1252914248, ptr %580, align 4
  store float %_0.i2854.lcssa1271414266, ptr %585, align 4
  store float %_0.i3228.lcssa1273014284, ptr %587, align 4
  store float %_0.i2850.lcssa1275414302, ptr %593, align 4
  store float %_0.i3224.lcssa1275514320, ptr %595, align 4
  %635 = add i32 %umax11943, 1, !dbg !7071
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_214.i129, i32 noundef %635, i32 noundef range(i32 0, 536870912) %_58.1.i305.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #28, !dbg !7458, !noalias !7459
  unreachable, !dbg !7458

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3172: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i290.i
  %_0.i2851 = fsub float 1.000000e+00, %_0.i3228, !dbg !7462
  %_15.i1408 = getelementptr inbounds nuw float, ptr %_58.0.i304.i, i32 %_214.i129, !dbg !7464
  %_0.i2874 = load float, ptr %_15.i1408, align 4, !dbg !7466, !alias.scope !7468, !noalias !7408, !noundef !10
  store float %_0.i3021, ptr %_15.i1408, align 4, !dbg !7471, !alias.scope !7475, !noalias !7408
  %_0.i2718 = fmul float %_0.i2851, %_0.i2874, !dbg !7478
  %_6.i3431 = bitcast float %_0.i2874 to i32, !dbg !7480
  %_5.i3432 = and i32 %_6.i3431, %all.sroa.0.0.i43, !dbg !7483
  %_8.i3433 = bitcast float %_0.i2718 to i32, !dbg !7484
  %_7.i3435 = and i32 %_9.i3434, %_8.i3433, !dbg !7486
  %_4.i3436 = or disjoint i32 %_7.i3435, %_5.i3432, !dbg !7483
  store i32 %_4.i3436, ptr %data.i.i.i.i.i.i, align 4, !dbg !7487, !alias.scope !7489, !noalias !7492
  %_220.i140 = add nuw i32 %iter.i19.sroa.41.08015, %right_end.sroa.0.0.i, !dbg !7493
  %_222.i142 = add nuw i32 %iter.i19.sroa.41.08015, %right_expiring.sroa.0.0.i, !dbg !7495
  %_8.not.i10.i.i152 = icmp ugt i32 %_7.i8.i260.i, %_54.1.i.i148
  br i1 %_8.not.i10.i.i152, label %bb4.i13.i.i202, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i154, !dbg !7496, !prof !3544

bb4.i13.i.i202:                                   ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3172
  store float %_0.i.i.lcssa1268314050, ptr %570, align 4
  store float %_0.i3247.lcssa1266914068, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265514086, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264114104, ptr %573, align 4
  store float %_0.i3260.lcssa1262714122, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261314140, ptr %574, align 4
  store float %_0.i.i3513.lcssa1259914158, ptr %576, align 4
  store float %_0.i3273.lcssa1258514176, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257114194, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255714212, ptr %579, align 4
  store float %_0.i3286.lcssa1254314230, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1252914248, ptr %580, align 4
  store float %_0.i2854.lcssa1271414266, ptr %585, align 4
  store float %_0.i3228.lcssa1273014284, ptr %587, align 4
  store float %_0.i2850.lcssa1275414302, ptr %593, align 4
  store float %_0.i3224.lcssa1275514320, ptr %595, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_213.i128, i32 noundef %_7.i8.i260.i, i32 noundef range(i32 0, 536870912) %_54.1.i.i148, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #28, !dbg !7502, !noalias !7503
  unreachable, !dbg !7502

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i154: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3172
  %_0.i2392 = fdiv float %_0.i3273, %_0.i3450, !dbg !7511
  %_3.i1959 = fcmp uge float %_0.i3273, %_0.i3450, !dbg !7513
  %_0.i3430 = select i1 %_3.i1959, float 1.000000e+00, float %_0.i2392, !dbg !7515
  %_17.i12.i.i155 = getelementptr inbounds nuw float, ptr %_54.0.i.i147, i32 %_213.i128, !dbg !7517
  store float %_0.i3430, ptr %_17.i12.i.i155, align 4, !dbg !7519, !alias.scope !7521, !noalias !7524
  %or.cond.i1383.not = icmp ult i32 %_220.i140, %_54.1.i.i148, !dbg !7525
  br i1 %or.cond.i1383.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1387, label %bb4.i1386, !dbg !7525, !prof !7284

bb4.i1386:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i154
  store float %_0.i.i.lcssa1268314050, ptr %570, align 4
  store float %_0.i3247.lcssa1266914068, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265514086, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264114104, ptr %573, align 4
  store float %_0.i3260.lcssa1262714122, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261314140, ptr %574, align 4
  store float %_0.i.i3513.lcssa1259914158, ptr %576, align 4
  store float %_0.i3273.lcssa1258514176, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257114194, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255714212, ptr %579, align 4
  store float %_0.i3286.lcssa1254314230, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1252914248, ptr %580, align 4
  store float %_0.i2854.lcssa1271414266, ptr %585, align 4
  store float %_0.i3228.lcssa1273014284, ptr %587, align 4
  store float %_0.i2850.lcssa1275414302, ptr %593, align 4
  store float %_0.i3224.lcssa1275514320, ptr %595, align 4
  %_5.i1380 = add i32 %_220.i140, 1, !dbg !7531
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_220.i140, i32 noundef %_5.i1380, i32 noundef range(i32 0, 536870912) %_54.1.i.i148, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #28, !dbg !7532, !noalias !7533
  unreachable, !dbg !7532

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1387: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i154
  %_15.i1384 = getelementptr inbounds nuw float, ptr %_54.0.i.i147, i32 %_220.i140, !dbg !7539
  %_0.i2880 = load float, ptr %_15.i1384, align 4, !dbg !7541, !alias.scope !7543, !noalias !7546, !noundef !10
  %636 = icmp eq i32 %storemerge.i8480, 0, !dbg !7547
  %_3.i.i3827.inv = fcmp olt float %running.sroa.0.0.i8507, %_0.i2880, !dbg !7547
  %_4.i.i3834.v = select i1 %_3.i.i3827.inv, float %running.sroa.0.0.i8507, float %_0.i2880, !dbg !7547
  %running.sroa.0.0.i = select i1 %636, float %_0.i2880, float %_4.i.i3834.v, !dbg !7547
  %_15.i = add i32 %storemerge.i8480, 1, !dbg !7548
  %complete.i = icmp eq i32 %_15.i, %_18.i.i158, !dbg !7548
  br i1 %complete.i, label %bb11.i1168.preheader, label %bb7.i1166, !dbg !7549

bb11.i1168.preheader:                             ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1387
  br i1 %_29.i11708009.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit, label %bb19.i1171, !dbg !7550

bb7.i1166:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1387
  %or.cond.i1375.not = icmp ult i32 %_216.i131, %_54.1.i.i148, !dbg !7553
  br i1 %or.cond.i1375.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1379, label %bb4.i1378, !dbg !7553, !prof !7284

bb4.i1378:                                        ; preds = %bb7.i1166
  store float %_0.i.i.lcssa1268314050, ptr %570, align 4
  store float %_0.i3247.lcssa1266914068, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265514086, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264114104, ptr %573, align 4
  store float %_0.i3260.lcssa1262714122, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261314140, ptr %574, align 4
  store float %_0.i.i3513.lcssa1259914158, ptr %576, align 4
  store float %_0.i3273.lcssa1258514176, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257114194, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255714212, ptr %579, align 4
  store float %_0.i3286.lcssa1254314230, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1252914248, ptr %580, align 4
  store float %_0.i2854.lcssa1271414266, ptr %585, align 4
  store float %_0.i3228.lcssa1273014284, ptr %587, align 4
  store float %_0.i2850.lcssa1275414302, ptr %593, align 4
  store float %_0.i3224.lcssa1275514320, ptr %595, align 4
  %_5.i1372 = add i32 %_216.i131, 1, !dbg !7558
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_216.i131, i32 noundef %_5.i1372, i32 noundef range(i32 0, 536870912) %_54.1.i.i148, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #28, !dbg !7559, !noalias !7560
  unreachable, !dbg !7559

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1379: ; preds = %bb7.i1166
  %_15.i1376 = getelementptr inbounds nuw float, ptr %_54.0.i.i147, i32 %_216.i131, !dbg !7563
  %_0.i2882 = load float, ptr %_15.i1376, align 4, !dbg !7565, !alias.scope !7567, !noalias !7546, !noundef !10
  %_3.i.i3818.inv = fcmp olt float %_0.i2882, %running.sroa.0.0.i, !dbg !7570
  %_4.i.i3825.v = select i1 %_3.i.i3818.inv, float %_0.i2882, float %running.sroa.0.0.i, !dbg !7570
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit, !dbg !7573

bb19.i1171:                                       ; preds = %bb11.i1168.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i
  %end.sroa.0.0.i8012 = phi i32 [ %638, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], [ %_220.i140, %bb11.i1168.preheader ]
  %suffix.sroa.0.0.i8011 = phi float [ %_4.i.i3816.v, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], [ %_0.i2880, %bb11.i1168.preheader ]
  %iter.sroa.0.0.i11698010 = phi i32 [ %_30.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], [ 0, %bb11.i1168.preheader ]
  %or.cond.i1359.not = icmp ult i32 %end.sroa.0.0.i8012, %_54.1.i.i148, !dbg !7574
  br i1 %or.cond.i1359.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i, label %bb4.i1362, !dbg !7574, !prof !7284

bb4.i1362:                                        ; preds = %bb19.i1171
  store float %_0.i.i.lcssa1268314050, ptr %570, align 4
  store float %_0.i3247.lcssa1266914068, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265514086, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264114104, ptr %573, align 4
  store float %_0.i3260.lcssa1262714122, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261314140, ptr %574, align 4
  store float %_0.i.i3513.lcssa1259914158, ptr %576, align 4
  store float %_0.i3273.lcssa1258514176, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257114194, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255714212, ptr %579, align 4
  store float %_0.i3286.lcssa1254314230, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1252914248, ptr %580, align 4
  store float %_0.i2854.lcssa1271414266, ptr %585, align 4
  store float %_0.i3228.lcssa1273014284, ptr %587, align 4
  store float %_0.i2850.lcssa1275414302, ptr %593, align 4
  store float %_0.i3224.lcssa1275514320, ptr %595, align 4
  %_5.i1356 = add i32 %end.sroa.0.0.i8012, 1, !dbg !7579
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %end.sroa.0.0.i8012, i32 noundef %_5.i1356, i32 noundef range(i32 0, 536870912) %_54.1.i.i148, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #28, !dbg !7580, !noalias !7581
  unreachable, !dbg !7580

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i: ; preds = %bb19.i1171
  %_30.i = add nuw i32 %iter.sroa.0.0.i11698010, 1, !dbg !7584
  %_15.i1360 = getelementptr inbounds nuw float, ptr %_54.0.i.i147, i32 %end.sroa.0.0.i8012, !dbg !7587
  %_0.i2886 = load float, ptr %_15.i1360, align 4, !dbg !7589, !alias.scope !7591, !noalias !7546, !noundef !10
  %_3.i.i3809.inv = fcmp olt float %suffix.sroa.0.0.i8011, %_0.i2886, !dbg !7594
  %_4.i.i3816.v = select i1 %_3.i.i3809.inv, float %suffix.sroa.0.0.i8011, float %_0.i2886, !dbg !7594
  store float %_4.i.i3816.v, ptr %_15.i1360, align 4, !dbg !7597, !alias.scope !7600, !noalias !7546
  %637 = icmp eq i32 %end.sroa.0.0.i8012, 0, !dbg !7603
  %spec.store.select.i1174 = select i1 %637, i32 %ring.i44, i32 %end.sroa.0.0.i8012, !dbg !7603
  %638 = add i32 %spec.store.select.i1174, -1, !dbg !7604
  %exitcond11941.not = icmp eq i32 %_30.i, %_18.i.i158, !dbg !7605
  br i1 %exitcond11941.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit, label %bb19.i1171, !dbg !7550

_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i, %bb11.i1168.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1379
  %storemerge.i = phi i32 [ %_15.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1379 ], [ 0, %bb11.i1168.preheader ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], !dbg !7607
  %running.sroa.0.1.i = phi float [ %_4.i.i3825.v, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1379 ], [ %running.sroa.0.0.i, %bb11.i1168.preheader ], [ %running.sroa.0.0.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i ], !dbg !7608
  %_0.i2717 = fmul float %running.sroa.0.1.i, 1.638400e+04, !dbg !7609
  %639 = tail call noundef float @llvm.floor.f32(float %_0.i2717), !dbg !7611
  %_0.i2716 = fmul float %639, 0x3F10000000000000, !dbg !7615
  %or.cond.i1399.not = icmp ult i32 %_222.i142, %_56.1.i.i166, !dbg !7617
  br i1 %or.cond.i1399.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1403, label %bb4.i1402, !dbg !7617, !prof !7284

bb4.i1402:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit
  store float %_0.i.i.lcssa1268314050, ptr %570, align 4
  store float %_0.i3247.lcssa1266914068, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265514086, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264114104, ptr %573, align 4
  store float %_0.i3260.lcssa1262714122, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261314140, ptr %574, align 4
  store float %_0.i.i3513.lcssa1259914158, ptr %576, align 4
  store float %_0.i3273.lcssa1258514176, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257114194, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255714212, ptr %579, align 4
  store float %_0.i3286.lcssa1254314230, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1252914248, ptr %580, align 4
  store float %_0.i2854.lcssa1271414266, ptr %585, align 4
  store float %_0.i3228.lcssa1273014284, ptr %587, align 4
  store float %_0.i2850.lcssa1275414302, ptr %593, align 4
  store float %_0.i3224.lcssa1275514320, ptr %595, align 4
  %_5.i1396 = add i32 %_222.i142, 1, !dbg !7622
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_222.i142, i32 noundef %_5.i1396, i32 noundef range(i32 0, 536870912) %_56.1.i.i166, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #28, !dbg !7623, !noalias !7624
  unreachable, !dbg !7623

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1403: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit
  %_15.i1400 = getelementptr inbounds nuw float, ptr %_56.0.i.i165, i32 %_222.i142, !dbg !7627
  %_0.i2876 = load float, ptr %_15.i1400, align 4, !dbg !7629, !alias.scope !7631, !noalias !7634, !noundef !10
  %_0.i2283 = fadd float %_0.i2716, %_0.i28508535, !dbg !7635
  %_0.i2850 = fsub float %_0.i2283, %_0.i2876, !dbg !7637
  %_8.not.i3.i.i173 = icmp ugt i32 %_7.i8.i260.i, %_56.1.i.i166
  br i1 %_8.not.i3.i.i173, label %bb4.i6.i.i201, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i174, !dbg !7639, !prof !3544

bb4.i6.i.i201:                                    ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1403
  store float %_0.i.i.lcssa1268314050, ptr %570, align 4
  store float %_0.i3247.lcssa1266914068, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265514086, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264114104, ptr %573, align 4
  store float %_0.i3260.lcssa1262714122, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261314140, ptr %574, align 4
  store float %_0.i.i3513.lcssa1259914158, ptr %576, align 4
  store float %_0.i3273.lcssa1258514176, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257114194, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255714212, ptr %579, align 4
  store float %_0.i3286.lcssa1254314230, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1252914248, ptr %580, align 4
  store float %_0.i2854.lcssa1271414266, ptr %585, align 4
  store float %_0.i3228.lcssa1273014284, ptr %587, align 4
  store float %_0.i2850.lcssa1275414302, ptr %593, align 4
  store float %_0.i3224.lcssa1275514320, ptr %595, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_213.i128, i32 noundef %_7.i8.i260.i, i32 noundef range(i32 0, 536870912) %_56.1.i.i166, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #28, !dbg !7644, !noalias !7645
  unreachable, !dbg !7644

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i174: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1403
  %_17.i5.i.i175 = getelementptr inbounds nuw float, ptr %_56.0.i.i165, i32 %_213.i128, !dbg !7648
  store float %_0.i2716, ptr %_17.i5.i.i175, align 4, !dbg !7650, !alias.scope !7652, !noalias !7634
  %_6.not.i1390 = icmp ugt i32 %_5.i1404, %_58.1.i.i189
  br i1 %_6.not.i1390, label %bb4.i1394, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165, !dbg !7655, !prof !3544

bb4.i1394:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i174
  store float %_0.i.i.lcssa1268314050, ptr %570, align 4
  store float %_0.i3247.lcssa1266914068, ptr %_110.i111, align 4
  store float %_0.i3241.lcssa1265514086, ptr %571, align 4
  store float %_0.i.i3506.lcssa1264114104, ptr %573, align 4
  store float %_0.i3260.lcssa1262714122, ptr %_111.i112, align 4
  store float %_0.i3253.lcssa1261314140, ptr %574, align 4
  store float %_0.i.i3513.lcssa1259914158, ptr %576, align 4
  store float %_0.i3273.lcssa1258514176, ptr %_115.i113, align 4
  store float %_0.i3266.lcssa1257114194, ptr %577, align 4
  store float %_0.i.i3520.lcssa1255714212, ptr %579, align 4
  store float %_0.i3286.lcssa1254314230, ptr %_116.i114, align 4
  store float %_0.i3279.lcssa1252914248, ptr %580, align 4
  store float %_0.i2854.lcssa1271414266, ptr %585, align 4
  store float %_0.i3228.lcssa1273014284, ptr %587, align 4
  store float %_0.i2850.lcssa1275414302, ptr %593, align 4
  store float %_0.i3224.lcssa1275514320, ptr %595, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_214.i129, i32 noundef %_5.i1404, i32 noundef range(i32 0, 536870912) %_58.1.i.i189, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #28, !dbg !7660, !noalias !7661
  unreachable, !dbg !7660

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i174
  %_0.i2391 = fdiv float %_0.i2850, %_37.i.i176, !dbg !7664
  %_0.i2849 = fsub float 1.000000e+00, %_0.i2391, !dbg !7666
  %_0.i2848 = fsub float %_0.i2849, %_0.i32248564, !dbg !7668
  %_4.i2407 = fmul float %_0.i3286, %_0.i2848, !dbg !7670
  %_0.i2408 = fadd float %_0.i32248564, %_4.i2407, !dbg !7670
  %_3.i.i3656.inv = fcmp ogt float %_0.i2849, %_0.i2408, !dbg !7672
  %_4.i.i3663.v = select i1 %_3.i.i3656.inv, float %_0.i2849, float %_0.i2408, !dbg !7672
  %640 = tail call noundef float @llvm.fabs.f32(float %_4.i.i3663.v), !dbg !7675
  %641 = fcmp uge float %640, 0x3BC79CA100000000, !dbg !7678
  %_0.i3224 = select i1 %641, float %_4.i.i3663.v, float 0.000000e+00, !dbg !7680
  %_0.i2847 = fsub float 1.000000e+00, %_0.i3224, !dbg !7681
  %_15.i1392 = getelementptr inbounds nuw float, ptr %_58.0.i.i188, i32 %_214.i129, !dbg !7683
  %_0.i2878 = load float, ptr %_15.i1392, align 4, !dbg !7685, !alias.scope !7687, !noalias !7634, !noundef !10
  store float %_0.i3016, ptr %_15.i1392, align 4, !dbg !7690, !alias.scope !7693, !noalias !7634
  %_0.i2715 = fmul float %_0.i2847, %_0.i2878, !dbg !7696
  %_6.i3418 = bitcast float %_0.i2878 to i32, !dbg !7698
  %_5.i3419 = and i32 %_6.i3418, %all.sroa.0.0.i43, !dbg !7701
  %_8.i3420 = bitcast float %_0.i2715 to i32, !dbg !7702
  %_7.i3422 = and i32 %_9.i3434, %_8.i3420, !dbg !7704
  %_4.i3423 = or disjoint i32 %_7.i3422, %_5.i3419, !dbg !7701
  store i32 %_4.i3423, ptr %data.i5.i.i.i.i.i, align 4, !dbg !7705, !alias.scope !7707, !noalias !7710
  %exitcond11954.not = icmp eq i32 %_206.0.i110, %umin11953, !dbg !7071
  br i1 %exitcond11954.not, label %bb67.i203, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3018, !dbg !7071

bb67.i203:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit
  %_0.i3224.lcssa1275514319 = phi float [ %_0.i3224.lcssa1275514320, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3224, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i2850.lcssa1275414301 = phi float [ %_0.i2850.lcssa1275414302, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i2850, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i3228.lcssa1273014283 = phi float [ %_0.i3228.lcssa1273014284, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3228, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i2854.lcssa1271414265 = phi float [ %_0.i2854.lcssa1271414266, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i2854, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i3279.lcssa1252914247 = phi float [ %_0.i3279.lcssa1252914248, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3279, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i3286.lcssa1254314229 = phi float [ %_0.i3286.lcssa1254314230, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3286, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i.i3520.lcssa1255714211 = phi float [ %_0.i.i3520.lcssa1255714212, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i.i3520, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i3266.lcssa1257114193 = phi float [ %_0.i3266.lcssa1257114194, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3266, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i3273.lcssa1258514175 = phi float [ %_0.i3273.lcssa1258514176, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3273, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i.i3513.lcssa1259914157 = phi float [ %_0.i.i3513.lcssa1259914158, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i.i3513, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i3253.lcssa1261314139 = phi float [ %_0.i3253.lcssa1261314140, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3253, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i3260.lcssa1262714121 = phi float [ %_0.i3260.lcssa1262714122, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3260, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i.i3506.lcssa1264114103 = phi float [ %_0.i.i3506.lcssa1264114104, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i.i3506, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i3241.lcssa1265514085 = phi float [ %_0.i3241.lcssa1265514086, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3241, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i3247.lcssa1266914067 = phi float [ %_0.i3247.lcssa1266914068, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i3247, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_0.i.i.lcssa1268314049 = phi float [ %_0.i.i.lcssa1268314050, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %_0.i.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %running.sroa.0.0.i.lcssa85298707 = phi float [ %running.sroa.0.0.i.lcssa85298708, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %running.sroa.0.0.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %storemerge.i.lcssa85028671 = phi i32 [ %storemerge.i.lcssa85028672, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %storemerge.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %running.sroa.0.0.i1180.lcssa84178635 = phi float [ %running.sroa.0.0.i1180.lcssa84178636, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %running.sroa.0.0.i1180, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %storemerge.i1185.lcssa83908599 = phi i32 [ %storemerge.i1185.lcssa83908600, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %storemerge.i1185, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3165 ]
  %_139.i204 = add i32 %run.sroa.0.0.i, %ring_cursor.sroa.0.1.i668593, !dbg !7711
  %_212.not.i205 = icmp ult i32 %_139.i204, %ring.i44, !dbg !7712
  %642 = select i1 %_212.not.i205, i32 0, i32 %ring.i44, !dbg !7712
  %ring_cursor.sroa.0.2.i206 = sub nuw i32 %_139.i204, %642, !dbg !7712
  %_141.i207 = add i32 %run.sroa.0.0.i, %main_cursor.sroa.0.1.i678594, !dbg !7715
  %_223.not.i208 = icmp ult i32 %_141.i207, %main.i45, !dbg !7716
  %643 = select i1 %_223.not.i208, i32 0, i32 %main.i45, !dbg !7716
  %main_cursor.sroa.0.2.i209 = sub nuw i32 %_141.i207, %643, !dbg !7716
  %_59.i69 = icmp ult i32 %_83.i83, %spec.store.select.i60, !dbg !6458
  br i1 %_59.i69, label %bb20.i70, label %bb19.i65.bb15.i53.loopexit_crit_edge, !dbg !6458

_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit: ; preds = %bb15.i53.loopexit, %bb7.i
  %ring_cursor.sroa.0.0.i54.lcssa = phi i32 [ %_37.i47, %bb7.i ], [ %ring_cursor.sroa.0.1.i66.lcssa, %bb15.i53.loopexit ], !dbg !6426
  %main_cursor.sroa.0.0.i55.lcssa = phi i32 [ %_36.i46, %bb7.i ], [ %main_cursor.sroa.0.1.i67.lcssa, %bb15.i53.loopexit ], !dbg !6423
  %644 = getelementptr inbounds nuw i8, ptr %uniform_left.i32, i32 24, !dbg !7718
  %left_prefix.i411 = load float, ptr %644, align 4, !dbg !7718, !noalias !6402, !noundef !10
  %645 = getelementptr inbounds nuw i8, ptr %uniform_left.i32, i32 28, !dbg !7719
  %left_phase.i412 = load i32, ptr %645, align 4, !dbg !7719, !noalias !6402, !noundef !10
  %646 = getelementptr inbounds nuw i8, ptr %uniform_right.i31, i32 24, !dbg !7720
  %right_prefix.i413 = load float, ptr %646, align 4, !dbg !7720, !noalias !6402, !noundef !10
  %647 = getelementptr inbounds nuw i8, ptr %uniform_right.i31, i32 28, !dbg !7721
  %right_phase.i414 = load i32, ptr %647, align 4, !dbg !7721, !noalias !6402, !noundef !10
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_right.i31), !dbg !7722, !noalias !6402
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i32), !dbg !7723, !noalias !6402
  %648 = getelementptr inbounds nuw i8, ptr %self, i32 364, !dbg !7724
  %_236.0.i415 = load ptr, ptr %648, align 4, !dbg !7724, !alias.scope !6392, !noalias !7726, !nonnull !10, !noundef !10
  %649 = getelementptr inbounds nuw i8, ptr %self, i32 368, !dbg !7724
  %_236.1.i416 = load i32, ptr %649, align 4, !dbg !7724, !alias.scope !6392, !noalias !7726, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7727), !dbg !7730
  %_4.not.i3150 = icmp eq i32 %_236.1.i416, 0, !dbg !7731
  br i1 %_4.not.i3150, label %panic.i3152, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3153, !dbg !7731

panic.i3152:                                      ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #28, !dbg !7731, !noalias !7733
  unreachable, !dbg !7731

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3153: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh2_fEB2_.exit
  store float %left_prefix.i411, ptr %_236.0.i415, align 4, !dbg !7731, !alias.scope !7727, !noalias !6396
  %_237.0.i417 = load ptr, ptr %68, align 4, !dbg !7734, !alias.scope !6392, !noalias !7726, !nonnull !10, !noundef !10
  %_237.1.i418 = load i32, ptr %69, align 4, !dbg !7734, !alias.scope !6392, !noalias !7726, !noundef !10
  %650 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i412), !dbg !7735
  br i1 %650, label %bb2.i4099, label %bb6.i4095, !dbg !7735

bb6.i4095:                                        ; preds = %bb2.i4099, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3153
  %end_or_len.idx.i = shl nuw nsw i32 %_237.1.i418, 2, !dbg !7739
  %end_or_len.i = getelementptr inbounds nuw i8, ptr %_237.0.i417, i32 %end_or_len.idx.i, !dbg !7739
  %_293.i = icmp eq i32 %_237.1.i418, 0, !dbg !7748
  br i1 %_293.i, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i4096, !dbg !7757

bb2.i4099:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3153
  %bytes1.sroa.0.0.zext.i = and i32 %left_phase.i412, 255, !dbg !7758
  %bytes1.sroa.0.0.isplat.i = mul nuw i32 %bytes1.sroa.0.0.zext.i, 16843009, !dbg !7758
  %_5.i4100 = icmp eq i32 %left_phase.i412, %bytes1.sroa.0.0.isplat.i, !dbg !7759
  br i1 %_5.i4100, label %bb3.i4101, label %bb6.i4095, !dbg !7759

bb3.i4101:                                        ; preds = %bb2.i4099
  %bytes.sroa.0.0.extract.trunc.i = trunc i32 %left_phase.i412 to i8, !dbg !7760
  %651 = shl nuw nsw i32 %_237.1.i418, 2, !dbg !7763
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %_237.0.i417, i8 %bytes.sroa.0.0.extract.trunc.i, i32 %651, i1 false), !dbg !7763, !alias.scope !7764, !noalias !6396
  br label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, !dbg !7767

bb10.i4096:                                       ; preds = %bb6.i4095, %bb10.i4096
  %iter.sroa.0.04.i = phi ptr [ %_38.i4097, %bb10.i4096 ], [ %_237.0.i417, %bb6.i4095 ]
  %_38.i4097 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i, i32 4, !dbg !7768
  store i32 %left_phase.i412, ptr %iter.sroa.0.04.i, align 4, !dbg !7771, !alias.scope !7764, !noalias !6396
  %_29.i4098 = icmp eq ptr %_38.i4097, %end_or_len.i, !dbg !7748
  br i1 %_29.i4098, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i4096, !dbg !7757

_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit: ; preds = %bb10.i4096, %bb6.i4095, %bb3.i4101
  %652 = getelementptr inbounds nuw i8, ptr %self, i32 464, !dbg !7773
  %_238.0.i419 = load ptr, ptr %652, align 4, !dbg !7773, !alias.scope !6394, !noalias !7774, !nonnull !10, !noundef !10
  %653 = getelementptr inbounds nuw i8, ptr %self, i32 468, !dbg !7773
  %_238.1.i420 = load i32, ptr %653, align 4, !dbg !7773, !alias.scope !6394, !noalias !7774, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7775), !dbg !7778
  %_4.not.i3146 = icmp eq i32 %_238.1.i420, 0, !dbg !7779
  br i1 %_4.not.i3146, label %panic.i3148, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3149, !dbg !7779

panic.i3148:                                      ; preds = %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #28, !dbg !7779, !noalias !7781
  unreachable, !dbg !7779

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3149: ; preds = %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit
  store float %right_prefix.i413, ptr %_238.0.i419, align 4, !dbg !7779, !alias.scope !7775, !noalias !6396
  %_239.0.i421 = load ptr, ptr %77, align 4, !dbg !7782, !alias.scope !6394, !noalias !7774, !nonnull !10, !noundef !10
  %_239.1.i422 = load i32, ptr %78, align 4, !dbg !7782, !alias.scope !6394, !noalias !7774, !noundef !10
  %654 = tail call i1 @llvm.is.constant.i32(i32 %right_phase.i414), !dbg !7783
  br i1 %654, label %bb2.i4110, label %bb6.i4102, !dbg !7783

bb6.i4102:                                        ; preds = %bb2.i4110, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3149
  %end_or_len.idx.i4103 = shl nuw nsw i32 %_239.1.i422, 2, !dbg !7786
  %end_or_len.i4104 = getelementptr inbounds nuw i8, ptr %_239.0.i421, i32 %end_or_len.idx.i4103, !dbg !7786
  %_293.i4105 = icmp eq i32 %_239.1.i422, 0, !dbg !7790
  br i1 %_293.i4105, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4116, label %bb10.i4106, !dbg !7793

bb2.i4110:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3149
  %bytes1.sroa.0.0.zext.i4111 = and i32 %right_phase.i414, 255, !dbg !7794
  %bytes1.sroa.0.0.isplat.i4112 = mul nuw i32 %bytes1.sroa.0.0.zext.i4111, 16843009, !dbg !7794
  %_5.i4113 = icmp eq i32 %right_phase.i414, %bytes1.sroa.0.0.isplat.i4112, !dbg !7795
  br i1 %_5.i4113, label %bb3.i4114, label %bb6.i4102, !dbg !7795

bb3.i4114:                                        ; preds = %bb2.i4110
  %bytes.sroa.0.0.extract.trunc.i4115 = trunc i32 %right_phase.i414 to i8, !dbg !7796
  %655 = shl nuw nsw i32 %_239.1.i422, 2, !dbg !7798
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %_239.0.i421, i8 %bytes.sroa.0.0.extract.trunc.i4115, i32 %655, i1 false), !dbg !7798, !alias.scope !7799, !noalias !6396
  br label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4116, !dbg !7802

bb10.i4106:                                       ; preds = %bb6.i4102, %bb10.i4106
  %iter.sroa.0.04.i4107 = phi ptr [ %_38.i4108, %bb10.i4106 ], [ %_239.0.i421, %bb6.i4102 ]
  %_38.i4108 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i4107, i32 4, !dbg !7803
  store i32 %right_phase.i414, ptr %iter.sroa.0.04.i4107, align 4, !dbg !7805, !alias.scope !7799, !noalias !6396
  %_29.i4109 = icmp eq ptr %_38.i4108, %end_or_len.i4104, !dbg !7790
  br i1 %_29.i4109, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4116, label %bb10.i4106, !dbg !7793

_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4116: ; preds = %bb10.i4106, %bb6.i4102, %bb3.i4114
  call void @llvm.lifetime.start.p0(ptr nonnull %_153.i16), !dbg !7806, !noalias !6402
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(92) %_153.i16, ptr noundef nonnull align 4 dereferenceable(92) %hot_left.i36, i32 92, i1 false), !dbg !7806, !noalias !6402
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_153.i16, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33) #27, !dbg !7807, !noalias !6396
  call void @llvm.lifetime.end.p0(ptr nonnull %_153.i16), !dbg !7808, !noalias !6402
  call void @llvm.lifetime.start.p0(ptr nonnull %_155.i15), !dbg !7809, !noalias !6402
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(92) %_155.i15, ptr noundef nonnull align 4 dereferenceable(92) %hot_right.i35, i32 92, i1 false), !dbg !7809, !noalias !6402
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %_155.i15, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34) #27, !dbg !7810, !noalias !6396
  call void @llvm.lifetime.end.p0(ptr nonnull %_155.i15), !dbg !7811, !noalias !6402
  store i32 %main_cursor.sroa.0.0.i55.lcssa, ptr %_35, align 4, !dbg !7812, !alias.scope !6396, !noalias !6425
  store i32 %ring_cursor.sroa.0.0.i54.lcssa, ptr %530, align 4, !dbg !7813, !alias.scope !6396, !noalias !6425
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i33), !dbg !7814, !noalias !6402
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i34), !dbg !7815, !noalias !6402
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_right.i35), !dbg !7816, !noalias !6402
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i36), !dbg !7817, !noalias !6402
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockfEB2_.exit, !dbg !6389

bb6.i:                                            ; preds = %bb5.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7818), !dbg !7821
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7822), !dbg !7821
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7824), !dbg !7821
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7826), !dbg !7821
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7828), !dbg !7821
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_left.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #27, !dbg !7830
; call <true_peak_limiter::HotChannel<f32>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr noalias noundef align 4 captures(none) dereferenceable(92) %hot_right.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #27, !dbg !7834
  %656 = getelementptr inbounds nuw i8, ptr %self, i32 320, !dbg !7836
  %657 = load i8, ptr %656, align 4, !dbg !7836, !range !3570, !alias.scope !7818, !noalias !7840, !noundef !10
  %658 = getelementptr inbounds nuw i8, ptr %self, i32 321, !dbg !7843
  %659 = load i8, ptr %658, align 1, !dbg !7843, !range !3570, !alias.scope !7818, !noalias !7840, !noundef !10
  %660 = getelementptr inbounds nuw i8, ptr %self, i32 528, !dbg !7845
  %ring.i = load i32, ptr %660, align 4, !dbg !7845, !alias.scope !7822, !noalias !7847, !noundef !10
  %661 = getelementptr inbounds nuw i8, ptr %self, i32 532, !dbg !7848
  %main.i = load i32, ptr %661, align 4, !dbg !7848, !alias.scope !7822, !noalias !7847, !noundef !10
  %_36.i = load i32, ptr %_35, align 4, !dbg !7850, !alias.scope !7828, !noalias !7852, !noundef !10
  %662 = getelementptr inbounds nuw i8, ptr %self, i32 108, !dbg !7853
  %_37.i = load i32, ptr %662, align 4, !dbg !7853, !alias.scope !7828, !noalias !7852, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i), !dbg !7855, !noalias !7857
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i, i8 0, i32 1024, i1 false), !noalias !7857
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_right.i), !dbg !7858, !noalias !7857
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_right.i, i8 0, i32 1024, i1 false), !noalias !7857
  %_32.i = zext nneg i8 %657 to i32, !dbg !7836
  %.none.i = sub nsw i32 0, %_32.i, !dbg !7860
  %_33.i = zext nneg i8 %659 to i32, !dbg !7843
  %all.sroa.0.0.i = sub nsw i32 0, %_33.i, !dbg !7843
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i), !dbg !7861, !noalias !7857
; call <true_peak_limiter::UniformHot<f32>>::new
  call fastcc void @_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotfE3newB5_(ptr noalias noundef align 4 captures(none) dereferenceable(44) %uniform_left.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33, i32 %ring.i, i32 %main.i) #27, !dbg !7863
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_right.i), !dbg !7864, !noalias !7857
  %_32.val3839 = load i32, ptr %660, align 4, !dbg !7866, !noundef !10
  %_32.val3840 = load i32, ptr %661, align 4, !dbg !7866, !noundef !10
; call <true_peak_limiter::UniformHot<f32>>::new
  call fastcc void @_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotfE3newB5_(ptr noalias noundef align 4 captures(none) dereferenceable(44) %uniform_right.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34, i32 %_32.val3839, i32 %_32.val3840) #27, !dbg !7866
  %_162.not.i9188 = icmp eq i32 %frames, 0, !dbg !7867
  br i1 %_162.not.i9188, label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit, label %bb44.i.lr.ph, !dbg !7867

bb44.i.lr.ph:                                     ; preds = %bb6.i
  %d9.i4117 = lshr i32 %frames, 5, !dbg !7877
  %r2.i4118 = and i32 %frames, 31, !dbg !7884
  %_19.not.i4119 = icmp ne i32 %r2.i4118, 0, !dbg !7885
  %663 = zext i1 %_19.not.i4119 to i32, !dbg !7885
  %yield_count.sroa.0.0.i4120 = add nuw nsw i32 %d9.i4117, %663, !dbg !7885
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
  %.promoted14972 = load float, ptr %705, align 4
  %.promoted14994 = load float, ptr %707, align 4
  %.promoted15016 = load float, ptr %713, align 4
  %.promoted15038 = load float, ptr %715, align 4
  br label %bb44.i, !dbg !7867

bb15.i.loopexit:                                  ; preds = %bb67.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i
  %_0.i3232.lcssa1223314391.lcssa15039 = phi float [ %_0.i3232.lcssa1223314391.lcssa15040, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %_0.i3232.lcssa1223314391, %bb67.i ]
  %_0.i2858.lcssa1223214373.lcssa15017 = phi float [ %_0.i2858.lcssa1223214373.lcssa15018, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %_0.i2858.lcssa1223214373, %bb67.i ]
  %_0.i3236.lcssa1220814355.lcssa14995 = phi float [ %_0.i3236.lcssa1220814355.lcssa14996, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %_0.i3236.lcssa1220814355, %bb67.i ]
  %_0.i2862.lcssa1219214337.lcssa14973 = phi float [ %_0.i2862.lcssa1219214337.lcssa14974, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %_0.i2862.lcssa1219214337, %bb67.i ]
  %running.sroa.0.0.i1210.lcssa89759153.lcssa14951 = phi float [ %running.sroa.0.0.i1210.lcssa89759153.lcssa14952, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %running.sroa.0.0.i1210.lcssa89759153, %bb67.i ]
  %storemerge.i1215.lcssa89489117.lcssa14930 = phi i32 [ %storemerge.i1215.lcssa89489117.lcssa14931, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %storemerge.i1215.lcssa89489117, %bb67.i ]
  %running.sroa.0.0.i1240.lcssa88639081.lcssa14909 = phi float [ %running.sroa.0.0.i1240.lcssa88639081.lcssa14910, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %running.sroa.0.0.i1240.lcssa88639081, %bb67.i ]
  %storemerge.i1245.lcssa88369045.lcssa14888 = phi i32 [ %storemerge.i1245.lcssa88369045.lcssa14889, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %storemerge.i1245.lcssa88369045, %bb67.i ]
  %ring_cursor.sroa.0.1.i.lcssa = phi i32 [ %ring_cursor.sroa.0.0.i9189, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %ring_cursor.sroa.0.2.i, %bb67.i ], !dbg !7886
  %main_cursor.sroa.0.1.i.lcssa = phi i32 [ %main_cursor.sroa.0.0.i9190, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i ], [ %main_cursor.sroa.0.2.i, %bb67.i ], !dbg !7887
  %_162.not.i = icmp eq i32 %719, 0, !dbg !7867
  %indvars.iv.next11956 = add i32 %indvars.iv11955, -32, !dbg !7867
  br i1 %_162.not.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit, label %bb44.i, !dbg !7867

bb44.i:                                           ; preds = %bb44.i.lr.ph, %bb15.i.loopexit
  %_0.i3232.lcssa1223314391.lcssa15040 = phi float [ %.promoted15038, %bb44.i.lr.ph ], [ %_0.i3232.lcssa1223314391.lcssa15039, %bb15.i.loopexit ]
  %_0.i2858.lcssa1223214373.lcssa15018 = phi float [ %.promoted15016, %bb44.i.lr.ph ], [ %_0.i2858.lcssa1223214373.lcssa15017, %bb15.i.loopexit ]
  %_0.i3236.lcssa1220814355.lcssa14996 = phi float [ %.promoted14994, %bb44.i.lr.ph ], [ %_0.i3236.lcssa1220814355.lcssa14995, %bb15.i.loopexit ]
  %_0.i2862.lcssa1219214337.lcssa14974 = phi float [ %.promoted14972, %bb44.i.lr.ph ], [ %_0.i2862.lcssa1219214337.lcssa14973, %bb15.i.loopexit ]
  %running.sroa.0.0.i1210.lcssa89759153.lcssa14952 = phi float [ %_21.i.i.promoted, %bb44.i.lr.ph ], [ %running.sroa.0.0.i1210.lcssa89759153.lcssa14951, %bb15.i.loopexit ]
  %storemerge.i1215.lcssa89489117.lcssa14931 = phi i32 [ %_22.i.i.promoted, %bb44.i.lr.ph ], [ %storemerge.i1215.lcssa89489117.lcssa14930, %bb15.i.loopexit ]
  %running.sroa.0.0.i1240.lcssa88639081.lcssa14910 = phi float [ %_21.i270.i.promoted, %bb44.i.lr.ph ], [ %running.sroa.0.0.i1240.lcssa88639081.lcssa14909, %bb15.i.loopexit ]
  %storemerge.i1245.lcssa88369045.lcssa14889 = phi i32 [ %_22.i271.i.promoted, %bb44.i.lr.ph ], [ %storemerge.i1245.lcssa88369045.lcssa14888, %bb15.i.loopexit ]
  %history.i.i.sroa.38.0.lcssa14868 = phi float [ %history.i.i.sroa.38.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.38.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.35.0.lcssa14848 = phi float [ %history.i.i.sroa.35.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.35.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.32.0.lcssa14828 = phi float [ %history.i.i.sroa.32.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.32.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.29.0.lcssa14808 = phi float [ %history.i.i.sroa.29.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.29.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.26.0.lcssa14788 = phi float [ %history.i.i.sroa.26.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.26.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.22.0.lcssa14768 = phi float [ %history.i.i.sroa.22.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.22.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.19.0.lcssa14748 = phi float [ %history.i.i.sroa.19.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.19.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.16.0.lcssa14728 = phi float [ %history.i.i.sroa.16.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.16.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.13.0.lcssa14708 = phi float [ %history.i.i.sroa.13.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.13.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.10.0.lcssa14688 = phi float [ %history.i.i.sroa.10.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.10.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.7.0.lcssa14668 = phi float [ %history.i.i.sroa.7.0.hot_right.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.7.0.lcssa, %bb15.i.loopexit ]
  %history.i.i.sroa.0.0.lcssa14648 = phi float [ %hot_right.i.promoted, %bb44.i.lr.ph ], [ %history.i.i.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %history.i39.i.sroa.38.0.lcssa14628 = phi float [ %history.i39.i.sroa.38.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i39.i.sroa.38.0.lcssa, %bb15.i.loopexit ]
  %history.i39.i.sroa.35.0.lcssa14608 = phi float [ %history.i39.i.sroa.35.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i39.i.sroa.35.0.lcssa, %bb15.i.loopexit ]
  %history.i39.i.sroa.32.0.lcssa14588 = phi float [ %history.i39.i.sroa.32.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i39.i.sroa.32.0.lcssa, %bb15.i.loopexit ]
  %history.i39.i.sroa.29.0.lcssa14568 = phi float [ %history.i39.i.sroa.29.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i39.i.sroa.29.0.lcssa, %bb15.i.loopexit ]
  %history.i39.i.sroa.26.0.lcssa14548 = phi float [ %history.i39.i.sroa.26.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i39.i.sroa.26.0.lcssa, %bb15.i.loopexit ]
  %history.i39.i.sroa.22.0.lcssa14528 = phi float [ %history.i39.i.sroa.22.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i39.i.sroa.22.0.lcssa, %bb15.i.loopexit ]
  %history.i39.i.sroa.19.0.lcssa14508 = phi float [ %history.i39.i.sroa.19.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i39.i.sroa.19.0.lcssa, %bb15.i.loopexit ]
  %history.i39.i.sroa.16.0.lcssa14488 = phi float [ %history.i39.i.sroa.16.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i39.i.sroa.16.0.lcssa, %bb15.i.loopexit ]
  %history.i39.i.sroa.13.0.lcssa14468 = phi float [ %history.i39.i.sroa.13.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i39.i.sroa.13.0.lcssa, %bb15.i.loopexit ]
  %history.i39.i.sroa.10.0.lcssa14448 = phi float [ %history.i39.i.sroa.10.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i39.i.sroa.10.0.lcssa, %bb15.i.loopexit ]
  %history.i39.i.sroa.7.0.lcssa14428 = phi float [ %history.i39.i.sroa.7.0.hot_left.i.sroa_idx.promoted, %bb44.i.lr.ph ], [ %history.i39.i.sroa.7.0.lcssa, %bb15.i.loopexit ]
  %history.i39.i.sroa.0.0.lcssa14408 = phi float [ %hot_left.i.promoted, %bb44.i.lr.ph ], [ %history.i39.i.sroa.0.0.lcssa, %bb15.i.loopexit ]
  %indvars.iv11955 = phi i32 [ %frames, %bb44.i.lr.ph ], [ %indvars.iv.next11956, %bb15.i.loopexit ]
  %iter2.sroa.0.0.i9192 = phi i32 [ %yield_count.sroa.0.0.i4120, %bb44.i.lr.ph ], [ %719, %bb15.i.loopexit ]
  %iter1.sroa.0.0.i9191 = phi i32 [ 0, %bb44.i.lr.ph ], [ %718, %bb15.i.loopexit ]
  %main_cursor.sroa.0.0.i9190 = phi i32 [ %_36.i, %bb44.i.lr.ph ], [ %main_cursor.sroa.0.1.i.lcssa, %bb15.i.loopexit ]
  %ring_cursor.sroa.0.0.i9189 = phi i32 [ %_37.i, %bb44.i.lr.ph ], [ %ring_cursor.sroa.0.1.i.lcssa, %bb15.i.loopexit ]
  %umin11975 = call i32 @llvm.umin.i32(i32 %indvars.iv11955, i32 32), !dbg !7888
  %umax11961 = call i32 @llvm.umax.i32(i32 %umin11975, i32 1), !dbg !7888
  %718 = add i32 %iter1.sroa.0.0.i9191, 32, !dbg !7888
  %719 = add nsw i32 %iter2.sroa.0.0.i9192, -1, !dbg !7892
  %720 = sub i32 %frames, %iter1.sroa.0.0.i9191, !dbg !7893
  %spec.store.select.i = tail call i32 @llvm.umin.i32(i32 %720, i32 32), !dbg !7895
  %_20.i42.i8751.not = icmp eq i32 %frames, %iter1.sroa.0.0.i9191, !dbg !7900
  br i1 %_20.i42.i8751.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i, label %bb5.i43.i.lr.ph, !dbg !7906

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
  br label %bb5.i43.i, !dbg !7906

bb5.i43.i:                                        ; preds = %bb5.i43.i.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038
  %iter.sroa.0.0.i41.i8763 = phi i32 [ 0, %bb5.i43.i.lr.ph ], [ %721, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ]
  %history.i39.i.sroa.35.08762 = phi float [ %history.i39.i.sroa.35.0.lcssa14608, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.32.08761, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ]
  %history.i39.i.sroa.32.08761 = phi float [ %history.i39.i.sroa.32.0.lcssa14588, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.29.08760, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ]
  %history.i39.i.sroa.29.08760 = phi float [ %history.i39.i.sroa.29.0.lcssa14568, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.26.08759, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ]
  %history.i39.i.sroa.26.08759 = phi float [ %history.i39.i.sroa.26.0.lcssa14548, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.22.08758, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ]
  %history.i39.i.sroa.22.08758 = phi float [ %history.i39.i.sroa.22.0.lcssa14528, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.19.08757, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ]
  %history.i39.i.sroa.19.08757 = phi float [ %history.i39.i.sroa.19.0.lcssa14508, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.16.08756, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ]
  %history.i39.i.sroa.16.08756 = phi float [ %history.i39.i.sroa.16.0.lcssa14488, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.13.08755, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ]
  %history.i39.i.sroa.13.08755 = phi float [ %history.i39.i.sroa.13.0.lcssa14468, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.10.08754, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ]
  %history.i39.i.sroa.10.08754 = phi float [ %history.i39.i.sroa.10.0.lcssa14448, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.7.08753, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ]
  %history.i39.i.sroa.7.08753 = phi float [ %history.i39.i.sroa.7.0.lcssa14428, %bb5.i43.i.lr.ph ], [ %history.i39.i.sroa.0.08752, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ]
  %history.i39.i.sroa.0.08752 = phi float [ %history.i39.i.sroa.0.0.lcssa14408, %bb5.i43.i.lr.ph ], [ %_0.i3036, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ]
  %721 = add nuw nsw i32 %iter.sroa.0.0.i41.i8763, 1, !dbg !7907
  %_11.i44.i = add nuw nsw i32 %iter.sroa.0.0.i41.i8763, %iter1.sroa.0.0.i9191, !dbg !7910
  %_24.i45.i = icmp ugt i32 %_11.i44.i, %left_io.1, !dbg !7911
  br i1 %_24.i45.i, label %bb7.i242.i, label %bb8.i46.i, !dbg !7911, !prof !755

bb8.i46.i:                                        ; preds = %bb5.i43.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7914), !dbg !7917
  %_3.not.i3034 = icmp eq i32 %left_io.1, %_11.i44.i, !dbg !7918
  br i1 %_3.not.i3034, label %panic.i3037, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038, !dbg !7918

panic.i3037:                                      ; preds = %bb8.i46.i
  store float %history.i39.i.sroa.0.0.lcssa14408, ptr %hot_left.i, align 4, !dbg !7920
  store float %history.i39.i.sroa.7.0.lcssa14428, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.10.0.lcssa14448, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.13.0.lcssa14468, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.16.0.lcssa14488, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.19.0.lcssa14508, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.22.0.lcssa14528, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.26.0.lcssa14548, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.29.0.lcssa14568, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.32.0.lcssa14588, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.35.0.lcssa14608, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.38.0.lcssa14628, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i.i.sroa.0.0.lcssa14648, ptr %hot_right.i, align 4, !dbg !7921
  store float %history.i.i.sroa.7.0.lcssa14668, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.10.0.lcssa14688, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.13.0.lcssa14708, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.16.0.lcssa14728, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.19.0.lcssa14748, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.22.0.lcssa14768, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.26.0.lcssa14788, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.29.0.lcssa14808, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.32.0.lcssa14828, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.35.0.lcssa14848, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.38.0.lcssa14868, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store i32 %storemerge.i1245.lcssa88369045.lcssa14889, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88639081.lcssa14910, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89489117.lcssa14931, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89759153.lcssa14952, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219214337.lcssa14974, ptr %705, align 4
  store float %_0.i3236.lcssa1220814355.lcssa14996, ptr %707, align 4
  store float %_0.i2858.lcssa1223214373.lcssa15018, ptr %713, align 4
  store float %_0.i3232.lcssa1223314391.lcssa15040, ptr %715, align 4
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #28, !dbg !7918, !noalias !7923
  unreachable, !dbg !7918

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038: ; preds = %bb8.i46.i
  %_31.i48.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %_11.i44.i, !dbg !7927
  %_0.i3036 = load float, ptr %_31.i48.i, align 4, !dbg !7918, !alias.scope !7914, !noalias !7929, !noundef !10
  %722 = tail call noundef float @llvm.fabs.f32(float %history.i39.i.sroa.19.08757), !dbg !7930
  %_0.i2768 = fmul float %_0.i3036, %_11.i.i.i63.i, !dbg !7933
  %_0.i2332 = fadd float %_0.i2768, 0.000000e+00, !dbg !7936
  %_0.i2767 = fmul float %_0.i3036, %_14.i.i.i66.i, !dbg !7938
  %_0.i2331 = fadd float %_0.i2767, 0.000000e+00, !dbg !7940
  %_0.i2766 = fmul float %_0.i3036, %_17.i.i.i69.i, !dbg !7942
  %_0.i2330 = fadd float %_0.i2766, 0.000000e+00, !dbg !7944
  %_0.i2765 = fmul float %_0.i3036, %_20.i.i.i72.i, !dbg !7946
  %_0.i2329 = fadd float %_0.i2765, 0.000000e+00, !dbg !7948
  %_0.i2764 = fmul float %history.i39.i.sroa.0.08752, %_25.i.i.i77.i, !dbg !7950
  %_0.i2328 = fadd float %_0.i2332, %_0.i2764, !dbg !7952
  %_0.i2763 = fmul float %history.i39.i.sroa.0.08752, %_28.i.i.i80.i, !dbg !7954
  %_0.i2327 = fadd float %_0.i2331, %_0.i2763, !dbg !7956
  %_0.i2762 = fmul float %history.i39.i.sroa.0.08752, %_31.i.i.i83.i, !dbg !7958
  %_0.i2326 = fadd float %_0.i2330, %_0.i2762, !dbg !7960
  %_0.i2761 = fmul float %history.i39.i.sroa.0.08752, %_34.i.i.i86.i, !dbg !7962
  %_0.i2325 = fadd float %_0.i2329, %_0.i2761, !dbg !7964
  %_0.i2760 = fmul float %history.i39.i.sroa.7.08753, %_39.i.i.i91.i, !dbg !7966
  %_0.i2324 = fadd float %_0.i2328, %_0.i2760, !dbg !7968
  %_0.i2759 = fmul float %history.i39.i.sroa.7.08753, %_42.i.i.i94.i, !dbg !7970
  %_0.i2323 = fadd float %_0.i2327, %_0.i2759, !dbg !7972
  %_0.i2758 = fmul float %history.i39.i.sroa.7.08753, %_45.i.i.i97.i, !dbg !7974
  %_0.i2322 = fadd float %_0.i2326, %_0.i2758, !dbg !7976
  %_0.i2757 = fmul float %history.i39.i.sroa.7.08753, %_48.i.i.i100.i, !dbg !7978
  %_0.i2321 = fadd float %_0.i2325, %_0.i2757, !dbg !7980
  %_0.i2756 = fmul float %history.i39.i.sroa.10.08754, %_53.i.i.i105.i, !dbg !7982
  %_0.i2320 = fadd float %_0.i2324, %_0.i2756, !dbg !7984
  %_0.i2755 = fmul float %history.i39.i.sroa.10.08754, %_56.i.i.i108.i, !dbg !7986
  %_0.i2319 = fadd float %_0.i2323, %_0.i2755, !dbg !7988
  %_0.i2754 = fmul float %history.i39.i.sroa.10.08754, %_59.i.i.i111.i, !dbg !7990
  %_0.i2318 = fadd float %_0.i2322, %_0.i2754, !dbg !7992
  %_0.i2753 = fmul float %history.i39.i.sroa.10.08754, %_62.i.i.i114.i, !dbg !7994
  %_0.i2317 = fadd float %_0.i2321, %_0.i2753, !dbg !7996
  %_0.i2752 = fmul float %history.i39.i.sroa.13.08755, %_67.i.i.i119.i, !dbg !7998
  %_0.i2316 = fadd float %_0.i2320, %_0.i2752, !dbg !8000
  %_0.i2751 = fmul float %history.i39.i.sroa.13.08755, %_70.i.i.i122.i, !dbg !8002
  %_0.i2315 = fadd float %_0.i2319, %_0.i2751, !dbg !8004
  %_0.i2750 = fmul float %history.i39.i.sroa.13.08755, %_73.i.i.i125.i, !dbg !8006
  %_0.i2314 = fadd float %_0.i2318, %_0.i2750, !dbg !8008
  %_0.i2749 = fmul float %history.i39.i.sroa.13.08755, %_76.i.i.i128.i, !dbg !8010
  %_0.i2313 = fadd float %_0.i2317, %_0.i2749, !dbg !8012
  %_0.i2748 = fmul float %history.i39.i.sroa.16.08756, %_81.i.i.i133.i, !dbg !8014
  %_0.i2312 = fadd float %_0.i2316, %_0.i2748, !dbg !8016
  %_0.i2747 = fmul float %history.i39.i.sroa.16.08756, %_84.i.i.i136.i, !dbg !8018
  %_0.i2311 = fadd float %_0.i2315, %_0.i2747, !dbg !8020
  %_0.i2746 = fmul float %history.i39.i.sroa.16.08756, %_87.i.i.i139.i, !dbg !8022
  %_0.i2310 = fadd float %_0.i2314, %_0.i2746, !dbg !8024
  %_0.i2745 = fmul float %history.i39.i.sroa.16.08756, %_90.i.i.i142.i, !dbg !8026
  %_0.i2309 = fadd float %_0.i2313, %_0.i2745, !dbg !8028
  %_0.i2744 = fmul float %history.i39.i.sroa.19.08757, %_95.i.i.i147.i, !dbg !8030
  %_0.i2308 = fadd float %_0.i2312, %_0.i2744, !dbg !8032
  %_0.i2743 = fmul float %history.i39.i.sroa.19.08757, %_98.i.i.i150.i, !dbg !8034
  %_0.i2307 = fadd float %_0.i2311, %_0.i2743, !dbg !8036
  %_0.i2742 = fmul float %history.i39.i.sroa.19.08757, %_101.i.i.i153.i, !dbg !8038
  %_0.i2306 = fadd float %_0.i2310, %_0.i2742, !dbg !8040
  %_0.i2741 = fmul float %history.i39.i.sroa.19.08757, %_104.i.i.i156.i, !dbg !8042
  %_0.i2305 = fadd float %_0.i2309, %_0.i2741, !dbg !8044
  %_0.i2740 = fmul float %history.i39.i.sroa.22.08758, %_109.i.i.i161.i, !dbg !8046
  %_0.i2304 = fadd float %_0.i2308, %_0.i2740, !dbg !8048
  %_0.i2739 = fmul float %history.i39.i.sroa.22.08758, %_112.i.i.i164.i, !dbg !8050
  %_0.i2303 = fadd float %_0.i2307, %_0.i2739, !dbg !8052
  %_0.i2738 = fmul float %history.i39.i.sroa.22.08758, %_115.i.i.i167.i, !dbg !8054
  %_0.i2302 = fadd float %_0.i2306, %_0.i2738, !dbg !8056
  %_0.i2737 = fmul float %history.i39.i.sroa.22.08758, %_118.i.i.i170.i, !dbg !8058
  %_0.i2301 = fadd float %_0.i2305, %_0.i2737, !dbg !8060
  %_0.i2736 = fmul float %history.i39.i.sroa.26.08759, %_123.i.i.i175.i, !dbg !8062
  %_0.i2300 = fadd float %_0.i2304, %_0.i2736, !dbg !8064
  %_0.i2735 = fmul float %history.i39.i.sroa.26.08759, %_126.i.i.i178.i, !dbg !8066
  %_0.i2299 = fadd float %_0.i2303, %_0.i2735, !dbg !8068
  %_0.i2734 = fmul float %history.i39.i.sroa.26.08759, %_129.i.i.i181.i, !dbg !8070
  %_0.i2298 = fadd float %_0.i2302, %_0.i2734, !dbg !8072
  %_0.i2733 = fmul float %history.i39.i.sroa.26.08759, %_132.i.i.i184.i, !dbg !8074
  %_0.i2297 = fadd float %_0.i2301, %_0.i2733, !dbg !8076
  %_0.i2732 = fmul float %history.i39.i.sroa.29.08760, %_137.i.i.i189.i, !dbg !8078
  %_0.i2296 = fadd float %_0.i2300, %_0.i2732, !dbg !8080
  %_0.i2731 = fmul float %history.i39.i.sroa.29.08760, %_140.i.i.i192.i, !dbg !8082
  %_0.i2295 = fadd float %_0.i2299, %_0.i2731, !dbg !8084
  %_0.i2730 = fmul float %history.i39.i.sroa.29.08760, %_143.i.i.i195.i, !dbg !8086
  %_0.i2294 = fadd float %_0.i2298, %_0.i2730, !dbg !8088
  %_0.i2729 = fmul float %history.i39.i.sroa.29.08760, %_146.i.i.i198.i, !dbg !8090
  %_0.i2293 = fadd float %_0.i2297, %_0.i2729, !dbg !8092
  %_0.i2728 = fmul float %history.i39.i.sroa.32.08761, %_151.i.i.i203.i, !dbg !8094
  %_0.i2292 = fadd float %_0.i2296, %_0.i2728, !dbg !8096
  %_0.i2727 = fmul float %history.i39.i.sroa.32.08761, %_154.i.i.i206.i, !dbg !8098
  %_0.i2291 = fadd float %_0.i2295, %_0.i2727, !dbg !8100
  %_0.i2726 = fmul float %history.i39.i.sroa.32.08761, %_157.i.i.i209.i, !dbg !8102
  %_0.i2290 = fadd float %_0.i2294, %_0.i2726, !dbg !8104
  %_0.i2725 = fmul float %history.i39.i.sroa.32.08761, %_160.i.i.i212.i, !dbg !8106
  %_0.i2289 = fadd float %_0.i2293, %_0.i2725, !dbg !8108
  %_0.i2724 = fmul float %history.i39.i.sroa.35.08762, %_165.i.i.i217.i, !dbg !8110
  %_0.i2288 = fadd float %_0.i2292, %_0.i2724, !dbg !8112
  %_0.i2723 = fmul float %history.i39.i.sroa.35.08762, %_168.i.i.i220.i, !dbg !8114
  %_0.i2287 = fadd float %_0.i2291, %_0.i2723, !dbg !8116
  %_0.i2722 = fmul float %history.i39.i.sroa.35.08762, %_171.i.i.i223.i, !dbg !8118
  %_0.i2286 = fadd float %_0.i2290, %_0.i2722, !dbg !8120
  %_0.i2721 = fmul float %history.i39.i.sroa.35.08762, %_174.i.i.i226.i, !dbg !8122
  %_0.i2285 = fadd float %_0.i2289, %_0.i2721, !dbg !8124
  %723 = tail call noundef float @llvm.fabs.f32(float %_0.i2288), !dbg !8126
  %_3.i.i3683.inv = fcmp ogt float %722, %723, !dbg !8128
  %_4.i.i3690.v = select i1 %_3.i.i3683.inv, float %722, float %723, !dbg !8128
  %724 = tail call noundef float @llvm.fabs.f32(float %_0.i2287), !dbg !8126
  %_3.i.i3683.inv.1 = fcmp ogt float %_4.i.i3690.v, %724, !dbg !8128
  %_4.i.i3690.v.1 = select i1 %_3.i.i3683.inv.1, float %_4.i.i3690.v, float %724, !dbg !8128
  %725 = tail call noundef float @llvm.fabs.f32(float %_0.i2286), !dbg !8126
  %_3.i.i3683.inv.2 = fcmp ogt float %_4.i.i3690.v.1, %725, !dbg !8128
  %_4.i.i3690.v.2 = select i1 %_3.i.i3683.inv.2, float %_4.i.i3690.v.1, float %725, !dbg !8128
  %726 = tail call noundef float @llvm.fabs.f32(float %_0.i2285), !dbg !8126
  %_3.i.i3683.inv.3 = fcmp ogt float %_4.i.i3690.v.2, %726, !dbg !8128
  %_4.i.i3690.v.3 = select i1 %_3.i.i3683.inv.3, float %_4.i.i3690.v.2, float %726, !dbg !8128
  %_39.i237.i = getelementptr inbounds nuw float, ptr %peaks_left.i, i32 %iter.sroa.0.0.i41.i8763, !dbg !8131
  store float %_4.i.i3690.v.3, ptr %_39.i237.i, align 4, !dbg !8136, !alias.scope !8138, !noalias !7929
  %exitcond11959.not = icmp eq i32 %721, %umax11961, !dbg !7900
  br i1 %exitcond11959.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i, label %bb5.i43.i, !dbg !7906

bb7.i242.i:                                       ; preds = %bb5.i43.i
  store float %history.i39.i.sroa.0.0.lcssa14408, ptr %hot_left.i, align 4, !dbg !7920
  store float %history.i39.i.sroa.7.0.lcssa14428, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.10.0.lcssa14448, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.13.0.lcssa14468, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.16.0.lcssa14488, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.19.0.lcssa14508, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.22.0.lcssa14528, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.26.0.lcssa14548, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.29.0.lcssa14568, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.32.0.lcssa14588, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.35.0.lcssa14608, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.38.0.lcssa14628, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i.i.sroa.0.0.lcssa14648, ptr %hot_right.i, align 4, !dbg !7921
  store float %history.i.i.sroa.7.0.lcssa14668, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.10.0.lcssa14688, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.13.0.lcssa14708, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.16.0.lcssa14728, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.19.0.lcssa14748, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.22.0.lcssa14768, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.26.0.lcssa14788, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.29.0.lcssa14808, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.32.0.lcssa14828, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.35.0.lcssa14848, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.38.0.lcssa14868, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store i32 %storemerge.i1245.lcssa88369045.lcssa14889, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88639081.lcssa14910, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89489117.lcssa14931, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89759153.lcssa14952, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219214337.lcssa14974, ptr %705, align 4
  store float %_0.i3236.lcssa1220814355.lcssa14996, ptr %707, align 4
  store float %_0.i2858.lcssa1223214373.lcssa15018, ptr %713, align 4
  store float %_0.i3232.lcssa1223314391.lcssa15040, ptr %715, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_11.i44.i, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #28, !dbg !8141, !noalias !7929
  unreachable, !dbg !8141

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038, %bb44.i
  %history.i39.i.sroa.0.0.lcssa = phi float [ %history.i39.i.sroa.0.0.lcssa14408, %bb44.i ], [ %_0.i3036, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ], !dbg !7920
  %history.i39.i.sroa.7.0.lcssa = phi float [ %history.i39.i.sroa.7.0.lcssa14428, %bb44.i ], [ %history.i39.i.sroa.0.08752, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ], !dbg !7920
  %history.i39.i.sroa.10.0.lcssa = phi float [ %history.i39.i.sroa.10.0.lcssa14448, %bb44.i ], [ %history.i39.i.sroa.7.08753, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ], !dbg !7920
  %history.i39.i.sroa.13.0.lcssa = phi float [ %history.i39.i.sroa.13.0.lcssa14468, %bb44.i ], [ %history.i39.i.sroa.10.08754, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ], !dbg !7920
  %history.i39.i.sroa.16.0.lcssa = phi float [ %history.i39.i.sroa.16.0.lcssa14488, %bb44.i ], [ %history.i39.i.sroa.13.08755, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ], !dbg !7920
  %history.i39.i.sroa.19.0.lcssa = phi float [ %history.i39.i.sroa.19.0.lcssa14508, %bb44.i ], [ %history.i39.i.sroa.16.08756, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ], !dbg !7920
  %history.i39.i.sroa.22.0.lcssa = phi float [ %history.i39.i.sroa.22.0.lcssa14528, %bb44.i ], [ %history.i39.i.sroa.19.08757, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ], !dbg !7920
  %history.i39.i.sroa.26.0.lcssa = phi float [ %history.i39.i.sroa.26.0.lcssa14548, %bb44.i ], [ %history.i39.i.sroa.22.08758, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ], !dbg !7920
  %history.i39.i.sroa.29.0.lcssa = phi float [ %history.i39.i.sroa.29.0.lcssa14568, %bb44.i ], [ %history.i39.i.sroa.26.08759, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ], !dbg !7920
  %history.i39.i.sroa.32.0.lcssa = phi float [ %history.i39.i.sroa.32.0.lcssa14588, %bb44.i ], [ %history.i39.i.sroa.29.08760, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ], !dbg !7920
  %history.i39.i.sroa.35.0.lcssa = phi float [ %history.i39.i.sroa.35.0.lcssa14608, %bb44.i ], [ %history.i39.i.sroa.32.08761, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ], !dbg !7920
  %history.i39.i.sroa.38.0.lcssa = phi float [ %history.i39.i.sroa.38.0.lcssa14628, %bb44.i ], [ %history.i39.i.sroa.35.08762, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3038 ], !dbg !7920
  br i1 %_20.i42.i8751.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i, label %bb5.i.i.lr.ph, !dbg !8142

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
  br label %bb5.i.i, !dbg !8142

bb5.i.i:                                          ; preds = %bb5.i.i.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043
  %iter.sroa.0.0.i.i8790 = phi i32 [ 0, %bb5.i.i.lr.ph ], [ %727, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ]
  %history.i.i.sroa.35.08789 = phi float [ %history.i.i.sroa.35.0.lcssa14848, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.32.08788, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ]
  %history.i.i.sroa.32.08788 = phi float [ %history.i.i.sroa.32.0.lcssa14828, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.29.08787, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ]
  %history.i.i.sroa.29.08787 = phi float [ %history.i.i.sroa.29.0.lcssa14808, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.26.08786, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ]
  %history.i.i.sroa.26.08786 = phi float [ %history.i.i.sroa.26.0.lcssa14788, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.22.08785, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ]
  %history.i.i.sroa.22.08785 = phi float [ %history.i.i.sroa.22.0.lcssa14768, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.19.08784, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ]
  %history.i.i.sroa.19.08784 = phi float [ %history.i.i.sroa.19.0.lcssa14748, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.16.08783, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ]
  %history.i.i.sroa.16.08783 = phi float [ %history.i.i.sroa.16.0.lcssa14728, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.13.08782, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ]
  %history.i.i.sroa.13.08782 = phi float [ %history.i.i.sroa.13.0.lcssa14708, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.10.08781, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ]
  %history.i.i.sroa.10.08781 = phi float [ %history.i.i.sroa.10.0.lcssa14688, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.7.08780, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ]
  %history.i.i.sroa.7.08780 = phi float [ %history.i.i.sroa.7.0.lcssa14668, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.0.08779, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ]
  %history.i.i.sroa.0.08779 = phi float [ %history.i.i.sroa.0.0.lcssa14648, %bb5.i.i.lr.ph ], [ %_0.i3041, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ]
  %727 = add nuw nsw i32 %iter.sroa.0.0.i.i8790, 1, !dbg !8145
  %_11.i.i = add nuw nsw i32 %iter.sroa.0.0.i.i8790, %iter1.sroa.0.0.i9191, !dbg !8148
  %_24.i.i = icmp ugt i32 %_11.i.i, %right_io.1, !dbg !8149
  br i1 %_24.i.i, label %bb7.i.i, label %bb8.i.i, !dbg !8149, !prof !755

bb8.i.i:                                          ; preds = %bb5.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8152), !dbg !8155
  %_3.not.i3039 = icmp eq i32 %right_io.1, %_11.i.i, !dbg !8156
  br i1 %_3.not.i3039, label %panic.i3042, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043, !dbg !8156

panic.i3042:                                      ; preds = %bb8.i.i
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !7920
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i.i.sroa.0.0.lcssa14648, ptr %hot_right.i, align 4, !dbg !7921
  store float %history.i.i.sroa.7.0.lcssa14668, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.10.0.lcssa14688, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.13.0.lcssa14708, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.16.0.lcssa14728, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.19.0.lcssa14748, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.22.0.lcssa14768, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.26.0.lcssa14788, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.29.0.lcssa14808, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.32.0.lcssa14828, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.35.0.lcssa14848, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.38.0.lcssa14868, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store i32 %storemerge.i1245.lcssa88369045.lcssa14889, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88639081.lcssa14910, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89489117.lcssa14931, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89759153.lcssa14952, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219214337.lcssa14974, ptr %705, align 4
  store float %_0.i3236.lcssa1220814355.lcssa14996, ptr %707, align 4
  store float %_0.i2858.lcssa1223214373.lcssa15018, ptr %713, align 4
  store float %_0.i3232.lcssa1223314391.lcssa15040, ptr %715, align 4
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d39684f46ea1235136a78696720e095f) #28, !dbg !8156, !noalias !8158
  unreachable, !dbg !8156

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043: ; preds = %bb8.i.i
  %_31.i.i = getelementptr inbounds nuw float, ptr %right_io.0, i32 %_11.i.i, !dbg !8162
  %_0.i3041 = load float, ptr %_31.i.i, align 4, !dbg !8156, !alias.scope !8152, !noalias !8164, !noundef !10
  %728 = tail call noundef float @llvm.fabs.f32(float %history.i.i.sroa.19.08784), !dbg !8165
  %_0.i2816 = fmul float %_0.i3041, %_11.i.i.i.i, !dbg !8168
  %_0.i2380 = fadd float %_0.i2816, 0.000000e+00, !dbg !8171
  %_0.i2815 = fmul float %_0.i3041, %_14.i.i.i.i, !dbg !8173
  %_0.i2379 = fadd float %_0.i2815, 0.000000e+00, !dbg !8175
  %_0.i2814 = fmul float %_0.i3041, %_17.i.i.i.i, !dbg !8177
  %_0.i2378 = fadd float %_0.i2814, 0.000000e+00, !dbg !8179
  %_0.i2813 = fmul float %_0.i3041, %_20.i.i.i.i, !dbg !8181
  %_0.i2377 = fadd float %_0.i2813, 0.000000e+00, !dbg !8183
  %_0.i2812 = fmul float %history.i.i.sroa.0.08779, %_25.i.i.i.i, !dbg !8185
  %_0.i2376 = fadd float %_0.i2380, %_0.i2812, !dbg !8187
  %_0.i2811 = fmul float %history.i.i.sroa.0.08779, %_28.i.i.i.i, !dbg !8189
  %_0.i2375 = fadd float %_0.i2379, %_0.i2811, !dbg !8191
  %_0.i2810 = fmul float %history.i.i.sroa.0.08779, %_31.i.i.i.i, !dbg !8193
  %_0.i2374 = fadd float %_0.i2378, %_0.i2810, !dbg !8195
  %_0.i2809 = fmul float %history.i.i.sroa.0.08779, %_34.i.i.i.i, !dbg !8197
  %_0.i2373 = fadd float %_0.i2377, %_0.i2809, !dbg !8199
  %_0.i2808 = fmul float %history.i.i.sroa.7.08780, %_39.i.i.i.i, !dbg !8201
  %_0.i2372 = fadd float %_0.i2376, %_0.i2808, !dbg !8203
  %_0.i2807 = fmul float %history.i.i.sroa.7.08780, %_42.i.i.i.i, !dbg !8205
  %_0.i2371 = fadd float %_0.i2375, %_0.i2807, !dbg !8207
  %_0.i2806 = fmul float %history.i.i.sroa.7.08780, %_45.i.i.i.i, !dbg !8209
  %_0.i2370 = fadd float %_0.i2374, %_0.i2806, !dbg !8211
  %_0.i2805 = fmul float %history.i.i.sroa.7.08780, %_48.i.i.i.i, !dbg !8213
  %_0.i2369 = fadd float %_0.i2373, %_0.i2805, !dbg !8215
  %_0.i2804 = fmul float %history.i.i.sroa.10.08781, %_53.i.i.i.i, !dbg !8217
  %_0.i2368 = fadd float %_0.i2372, %_0.i2804, !dbg !8219
  %_0.i2803 = fmul float %history.i.i.sroa.10.08781, %_56.i.i.i.i, !dbg !8221
  %_0.i2367 = fadd float %_0.i2371, %_0.i2803, !dbg !8223
  %_0.i2802 = fmul float %history.i.i.sroa.10.08781, %_59.i.i.i.i, !dbg !8225
  %_0.i2366 = fadd float %_0.i2370, %_0.i2802, !dbg !8227
  %_0.i2801 = fmul float %history.i.i.sroa.10.08781, %_62.i.i.i.i, !dbg !8229
  %_0.i2365 = fadd float %_0.i2369, %_0.i2801, !dbg !8231
  %_0.i2800 = fmul float %history.i.i.sroa.13.08782, %_67.i.i.i.i, !dbg !8233
  %_0.i2364 = fadd float %_0.i2368, %_0.i2800, !dbg !8235
  %_0.i2799 = fmul float %history.i.i.sroa.13.08782, %_70.i.i.i.i, !dbg !8237
  %_0.i2363 = fadd float %_0.i2367, %_0.i2799, !dbg !8239
  %_0.i2798 = fmul float %history.i.i.sroa.13.08782, %_73.i.i.i.i, !dbg !8241
  %_0.i2362 = fadd float %_0.i2366, %_0.i2798, !dbg !8243
  %_0.i2797 = fmul float %history.i.i.sroa.13.08782, %_76.i.i.i.i, !dbg !8245
  %_0.i2361 = fadd float %_0.i2365, %_0.i2797, !dbg !8247
  %_0.i2796 = fmul float %history.i.i.sroa.16.08783, %_81.i.i.i.i, !dbg !8249
  %_0.i2360 = fadd float %_0.i2364, %_0.i2796, !dbg !8251
  %_0.i2795 = fmul float %history.i.i.sroa.16.08783, %_84.i.i.i.i, !dbg !8253
  %_0.i2359 = fadd float %_0.i2363, %_0.i2795, !dbg !8255
  %_0.i2794 = fmul float %history.i.i.sroa.16.08783, %_87.i.i.i.i, !dbg !8257
  %_0.i2358 = fadd float %_0.i2362, %_0.i2794, !dbg !8259
  %_0.i2793 = fmul float %history.i.i.sroa.16.08783, %_90.i.i.i.i, !dbg !8261
  %_0.i2357 = fadd float %_0.i2361, %_0.i2793, !dbg !8263
  %_0.i2792 = fmul float %history.i.i.sroa.19.08784, %_95.i.i.i.i, !dbg !8265
  %_0.i2356 = fadd float %_0.i2360, %_0.i2792, !dbg !8267
  %_0.i2791 = fmul float %history.i.i.sroa.19.08784, %_98.i.i.i.i, !dbg !8269
  %_0.i2355 = fadd float %_0.i2359, %_0.i2791, !dbg !8271
  %_0.i2790 = fmul float %history.i.i.sroa.19.08784, %_101.i.i.i.i, !dbg !8273
  %_0.i2354 = fadd float %_0.i2358, %_0.i2790, !dbg !8275
  %_0.i2789 = fmul float %history.i.i.sroa.19.08784, %_104.i.i.i.i, !dbg !8277
  %_0.i2353 = fadd float %_0.i2357, %_0.i2789, !dbg !8279
  %_0.i2788 = fmul float %history.i.i.sroa.22.08785, %_109.i.i.i.i, !dbg !8281
  %_0.i2352 = fadd float %_0.i2356, %_0.i2788, !dbg !8283
  %_0.i2787 = fmul float %history.i.i.sroa.22.08785, %_112.i.i.i.i, !dbg !8285
  %_0.i2351 = fadd float %_0.i2355, %_0.i2787, !dbg !8287
  %_0.i2786 = fmul float %history.i.i.sroa.22.08785, %_115.i.i.i.i, !dbg !8289
  %_0.i2350 = fadd float %_0.i2354, %_0.i2786, !dbg !8291
  %_0.i2785 = fmul float %history.i.i.sroa.22.08785, %_118.i.i.i.i, !dbg !8293
  %_0.i2349 = fadd float %_0.i2353, %_0.i2785, !dbg !8295
  %_0.i2784 = fmul float %history.i.i.sroa.26.08786, %_123.i.i.i.i, !dbg !8297
  %_0.i2348 = fadd float %_0.i2352, %_0.i2784, !dbg !8299
  %_0.i2783 = fmul float %history.i.i.sroa.26.08786, %_126.i.i.i.i, !dbg !8301
  %_0.i2347 = fadd float %_0.i2351, %_0.i2783, !dbg !8303
  %_0.i2782 = fmul float %history.i.i.sroa.26.08786, %_129.i.i.i.i, !dbg !8305
  %_0.i2346 = fadd float %_0.i2350, %_0.i2782, !dbg !8307
  %_0.i2781 = fmul float %history.i.i.sroa.26.08786, %_132.i.i.i.i, !dbg !8309
  %_0.i2345 = fadd float %_0.i2349, %_0.i2781, !dbg !8311
  %_0.i2780 = fmul float %history.i.i.sroa.29.08787, %_137.i.i.i.i, !dbg !8313
  %_0.i2344 = fadd float %_0.i2348, %_0.i2780, !dbg !8315
  %_0.i2779 = fmul float %history.i.i.sroa.29.08787, %_140.i.i.i.i, !dbg !8317
  %_0.i2343 = fadd float %_0.i2347, %_0.i2779, !dbg !8319
  %_0.i2778 = fmul float %history.i.i.sroa.29.08787, %_143.i.i.i.i, !dbg !8321
  %_0.i2342 = fadd float %_0.i2346, %_0.i2778, !dbg !8323
  %_0.i2777 = fmul float %history.i.i.sroa.29.08787, %_146.i.i.i.i, !dbg !8325
  %_0.i2341 = fadd float %_0.i2345, %_0.i2777, !dbg !8327
  %_0.i2776 = fmul float %history.i.i.sroa.32.08788, %_151.i.i.i.i, !dbg !8329
  %_0.i2340 = fadd float %_0.i2344, %_0.i2776, !dbg !8331
  %_0.i2775 = fmul float %history.i.i.sroa.32.08788, %_154.i.i.i.i, !dbg !8333
  %_0.i2339 = fadd float %_0.i2343, %_0.i2775, !dbg !8335
  %_0.i2774 = fmul float %history.i.i.sroa.32.08788, %_157.i.i.i.i, !dbg !8337
  %_0.i2338 = fadd float %_0.i2342, %_0.i2774, !dbg !8339
  %_0.i2773 = fmul float %history.i.i.sroa.32.08788, %_160.i.i.i.i, !dbg !8341
  %_0.i2337 = fadd float %_0.i2341, %_0.i2773, !dbg !8343
  %_0.i2772 = fmul float %history.i.i.sroa.35.08789, %_165.i.i.i.i, !dbg !8345
  %_0.i2336 = fadd float %_0.i2340, %_0.i2772, !dbg !8347
  %_0.i2771 = fmul float %history.i.i.sroa.35.08789, %_168.i.i.i.i, !dbg !8349
  %_0.i2335 = fadd float %_0.i2339, %_0.i2771, !dbg !8351
  %_0.i2770 = fmul float %history.i.i.sroa.35.08789, %_171.i.i.i.i, !dbg !8353
  %_0.i2334 = fadd float %_0.i2338, %_0.i2770, !dbg !8355
  %_0.i2769 = fmul float %history.i.i.sroa.35.08789, %_174.i.i.i.i, !dbg !8357
  %_0.i2333 = fadd float %_0.i2337, %_0.i2769, !dbg !8359
  %729 = tail call noundef float @llvm.fabs.f32(float %_0.i2336), !dbg !8361
  %_3.i.i3692.inv = fcmp ogt float %728, %729, !dbg !8363
  %_4.i.i3699.v = select i1 %_3.i.i3692.inv, float %728, float %729, !dbg !8363
  %730 = tail call noundef float @llvm.fabs.f32(float %_0.i2335), !dbg !8361
  %_3.i.i3692.inv.1 = fcmp ogt float %_4.i.i3699.v, %730, !dbg !8363
  %_4.i.i3699.v.1 = select i1 %_3.i.i3692.inv.1, float %_4.i.i3699.v, float %730, !dbg !8363
  %731 = tail call noundef float @llvm.fabs.f32(float %_0.i2334), !dbg !8361
  %_3.i.i3692.inv.2 = fcmp ogt float %_4.i.i3699.v.1, %731, !dbg !8363
  %_4.i.i3699.v.2 = select i1 %_3.i.i3692.inv.2, float %_4.i.i3699.v.1, float %731, !dbg !8363
  %732 = tail call noundef float @llvm.fabs.f32(float %_0.i2333), !dbg !8361
  %_3.i.i3692.inv.3 = fcmp ogt float %_4.i.i3699.v.2, %732, !dbg !8363
  %_4.i.i3699.v.3 = select i1 %_3.i.i3692.inv.3, float %_4.i.i3699.v.2, float %732, !dbg !8363
  %_39.i.i = getelementptr inbounds nuw float, ptr %peaks_right.i, i32 %iter.sroa.0.0.i.i8790, !dbg !8366
  store float %_4.i.i3699.v.3, ptr %_39.i.i, align 4, !dbg !8371, !alias.scope !8373, !noalias !8164
  %exitcond11962.not = icmp eq i32 %727, %umax11961, !dbg !8376
  br i1 %exitcond11962.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i, label %bb5.i.i, !dbg !8142

bb7.i.i:                                          ; preds = %bb5.i.i
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !7920
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i.i.sroa.0.0.lcssa14648, ptr %hot_right.i, align 4, !dbg !7921
  store float %history.i.i.sroa.7.0.lcssa14668, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.10.0.lcssa14688, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.13.0.lcssa14708, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.16.0.lcssa14728, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.19.0.lcssa14748, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.22.0.lcssa14768, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.26.0.lcssa14788, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.29.0.lcssa14808, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.32.0.lcssa14828, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.35.0.lcssa14848, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.38.0.lcssa14868, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store i32 %storemerge.i1245.lcssa88369045.lcssa14889, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88639081.lcssa14910, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89489117.lcssa14931, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89759153.lcssa14952, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219214337.lcssa14974, ptr %705, align 4
  store float %_0.i3236.lcssa1220814355.lcssa14996, ptr %707, align 4
  store float %_0.i2858.lcssa1223214373.lcssa15018, ptr %713, align 4
  store float %_0.i3232.lcssa1223314391.lcssa15040, ptr %715, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_11.i.i, i32 noundef range(i32 0, 536870912) %right_io.1, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #28, !dbg !8378, !noalias !8164
  unreachable, !dbg !8378

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i
  %history.i.i.sroa.0.0.lcssa = phi float [ %history.i.i.sroa.0.0.lcssa14648, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %_0.i3041, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ], !dbg !7921
  %history.i.i.sroa.7.0.lcssa = phi float [ %history.i.i.sroa.7.0.lcssa14668, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i.sroa.0.08779, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ], !dbg !7921
  %history.i.i.sroa.10.0.lcssa = phi float [ %history.i.i.sroa.10.0.lcssa14688, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i.sroa.7.08780, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ], !dbg !7921
  %history.i.i.sroa.13.0.lcssa = phi float [ %history.i.i.sroa.13.0.lcssa14708, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i.sroa.10.08781, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ], !dbg !7921
  %history.i.i.sroa.16.0.lcssa = phi float [ %history.i.i.sroa.16.0.lcssa14728, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i.sroa.13.08782, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ], !dbg !7921
  %history.i.i.sroa.19.0.lcssa = phi float [ %history.i.i.sroa.19.0.lcssa14748, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i.sroa.16.08783, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ], !dbg !7921
  %history.i.i.sroa.22.0.lcssa = phi float [ %history.i.i.sroa.22.0.lcssa14768, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i.sroa.19.08784, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ], !dbg !7921
  %history.i.i.sroa.26.0.lcssa = phi float [ %history.i.i.sroa.26.0.lcssa14788, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i.sroa.22.08785, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ], !dbg !7921
  %history.i.i.sroa.29.0.lcssa = phi float [ %history.i.i.sroa.29.0.lcssa14808, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i.sroa.26.08786, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ], !dbg !7921
  %history.i.i.sroa.32.0.lcssa = phi float [ %history.i.i.sroa.32.0.lcssa14828, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i.sroa.29.08787, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ], !dbg !7921
  %history.i.i.sroa.35.0.lcssa = phi float [ %history.i.i.sroa.35.0.lcssa14848, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i.sroa.32.08788, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ], !dbg !7921
  %history.i.i.sroa.38.0.lcssa = phi float [ %history.i.i.sroa.38.0.lcssa14868, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit243.i ], [ %history.i.i.sroa.35.08789, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3043 ], !dbg !7921
  br i1 %_20.i42.i8751.not, label %bb15.i.loopexit, label %bb20.i.lr.ph, !dbg !8379

bb20.i.lr.ph:                                     ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkfEB2_.exit.i
  %_68.i.sroa.3.0.copyload = load i32, ptr %_68.i.sroa.3.0..sroa_idx, align 4, !noalias !7857
  %_68.i.sroa.4.0.copyload = load i32, ptr %_68.i.sroa.4.0..sroa_idx, align 4, !noalias !7857
  %_69.i.sroa.3.0.copyload = load i32, ptr %_69.i.sroa.3.0..sroa_idx, align 4, !noalias !7857
  %_69.i.sroa.4.0.copyload = load i32, ptr %_69.i.sroa.4.0..sroa_idx, align 4, !noalias !7857
  %_54.0.i256.i = load ptr, ptr %uniform_left.i, align 4, !nonnull !10, !align !6963
  %_54.1.i257.i = load i32, ptr %702, align 4
  %_18.i267.i = load i32, ptr %700, align 4
  %_29.i12528803.not = icmp eq i32 %_18.i267.i, 0
  %_56.0.i278.i = load ptr, ptr %703, align 4, !nonnull !10, !align !6963
  %_56.1.i279.i = load i32, ptr %704, align 4
  %_58.1.i304.i = load i32, ptr %708, align 4
  %_58.0.i303.i = load ptr, ptr %709, align 4, !nonnull !10, !align !6963
  %_54.0.i.i = load ptr, ptr %uniform_right.i, align 4, !nonnull !10, !align !6963
  %_54.1.i.i = load i32, ptr %710, align 4
  %_18.i.i = load i32, ptr %701, align 4
  %_29.i12228807.not = icmp eq i32 %_18.i.i, 0
  %_56.0.i.i = load ptr, ptr %711, align 4, !nonnull !10, !align !6963
  %_56.1.i.i = load i32, ptr %712, align 4
  %_58.1.i.i = load i32, ptr %716, align 4
  %_58.0.i.i = load ptr, ptr %717, align 4, !nonnull !10, !align !6963
  %_8.i29.i = load float, ptr %_110.i, align 4
  %_9.i30.i = load float, ptr %_111.i, align 4
  %_8.i.i = load float, ptr %_115.i, align 4
  %_9.i.i = load float, ptr %_116.i, align 4
  %_37.i291.i = load float, ptr %706, align 4
  %_37.i.i = load float, ptr %714, align 4
  br label %bb20.i, !dbg !8379

bb20.i:                                           ; preds = %bb20.i.lr.ph, %bb67.i
  %_0.i3232.lcssa1223314392 = phi float [ %_0.i3232.lcssa1223314391.lcssa15040, %bb20.i.lr.ph ], [ %_0.i3232.lcssa1223314391, %bb67.i ]
  %_0.i2858.lcssa1223214374 = phi float [ %_0.i2858.lcssa1223214373.lcssa15018, %bb20.i.lr.ph ], [ %_0.i2858.lcssa1223214373, %bb67.i ]
  %_0.i3236.lcssa1220814356 = phi float [ %_0.i3236.lcssa1220814355.lcssa14996, %bb20.i.lr.ph ], [ %_0.i3236.lcssa1220814355, %bb67.i ]
  %_0.i2862.lcssa1219214338 = phi float [ %_0.i2862.lcssa1219214337.lcssa14974, %bb20.i.lr.ph ], [ %_0.i2862.lcssa1219214337, %bb67.i ]
  %running.sroa.0.0.i1210.lcssa89759154 = phi float [ %running.sroa.0.0.i1210.lcssa89759153.lcssa14952, %bb20.i.lr.ph ], [ %running.sroa.0.0.i1210.lcssa89759153, %bb67.i ]
  %storemerge.i1215.lcssa89489118 = phi i32 [ %storemerge.i1215.lcssa89489117.lcssa14931, %bb20.i.lr.ph ], [ %storemerge.i1215.lcssa89489117, %bb67.i ]
  %running.sroa.0.0.i1240.lcssa88639082 = phi float [ %running.sroa.0.0.i1240.lcssa88639081.lcssa14910, %bb20.i.lr.ph ], [ %running.sroa.0.0.i1240.lcssa88639081, %bb67.i ]
  %storemerge.i1245.lcssa88369046 = phi i32 [ %storemerge.i1245.lcssa88369045.lcssa14889, %bb20.i.lr.ph ], [ %storemerge.i1245.lcssa88369045, %bb67.i ]
  %frame.sroa.0.0.i9041 = phi i32 [ 0, %bb20.i.lr.ph ], [ %_83.i, %bb67.i ]
  %main_cursor.sroa.0.1.i9040 = phi i32 [ %main_cursor.sroa.0.0.i9190, %bb20.i.lr.ph ], [ %main_cursor.sroa.0.2.i, %bb67.i ]
  %ring_cursor.sroa.0.1.i9039 = phi i32 [ %ring_cursor.sroa.0.0.i9189, %bb20.i.lr.ph ], [ %ring_cursor.sroa.0.2.i, %bb67.i ]
  %_65.i = sub nuw nsw i32 %spec.store.select.i, %frame.sroa.0.0.i9041, !dbg !8381
  %ring.i1803 = load i32, ptr %660, align 4, !dbg !8382, !alias.scope !8384, !noalias !8387, !noundef !10
  %main.i1804 = load i32, ptr %661, align 4, !dbg !8391, !alias.scope !8384, !noalias !8387, !noundef !10
  %_10.i1805 = add i32 %ring_cursor.sroa.0.1.i9039, 1, !dbg !8392
  %_38.not.i1806 = icmp ult i32 %_10.i1805, %ring.i1803, !dbg !8393
  %733 = select i1 %_38.not.i1806, i32 0, i32 %ring.i1803, !dbg !8393
  %start1.sroa.0.0.i1807 = sub nuw i32 %_10.i1805, %733, !dbg !8393
  %_12.i1809 = add i32 %_68.i.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i9039, !dbg !8395
  %_39.not.i1810 = icmp ult i32 %_12.i1809, %ring.i1803, !dbg !8396
  %734 = select i1 %_39.not.i1810, i32 0, i32 %ring.i1803, !dbg !8396
  %left_end.sroa.0.0.i1811 = sub nuw i32 %_12.i1809, %734, !dbg !8396
  %_15.i1813 = add i32 %_69.i.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i9039, !dbg !8398
  %_40.not.i1814 = icmp ult i32 %_15.i1813, %ring.i1803, !dbg !8399
  %735 = select i1 %_40.not.i1814, i32 0, i32 %ring.i1803, !dbg !8399
  %right_end.sroa.0.0.i1815 = sub nuw i32 %_15.i1813, %735, !dbg !8399
  %_18.i1817 = add i32 %_68.i.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i9039, !dbg !8401
  %_41.not.i1818 = icmp ult i32 %_18.i1817, %ring.i1803, !dbg !8402
  %736 = select i1 %_41.not.i1818, i32 0, i32 %ring.i1803, !dbg !8402
  %left_expiring.sroa.0.0.i1819 = sub nuw i32 %_18.i1817, %736, !dbg !8402
  %_21.i1821 = add i32 %_69.i.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i9039, !dbg !8404
  %_42.not.i1822 = icmp ult i32 %_21.i1821, %ring.i1803, !dbg !8405
  %737 = select i1 %_42.not.i1822, i32 0, i32 %ring.i1803, !dbg !8405
  %right_expiring.sroa.0.0.i1823 = sub nuw i32 %_21.i1821, %737, !dbg !8405
  %738 = sub i32 %ring.i1803, %ring_cursor.sroa.0.1.i9039, !dbg !8407
  %spec.store.select.i1824 = tail call i32 @llvm.umin.i32(i32 %738, i32 %_65.i), !dbg !8408
  %739 = sub i32 %main.i1804, %main_cursor.sroa.0.1.i9040, !dbg !8410
  %_24.sroa.0.0.i1826 = tail call i32 @llvm.umin.i32(i32 %739, i32 %spec.store.select.i1824), !dbg !8411
  %740 = sub i32 %ring.i1803, %start1.sroa.0.0.i1807, !dbg !8413
  %_25.sroa.0.0.i1828 = tail call i32 @llvm.umin.i32(i32 %740, i32 %_24.sroa.0.0.i1826), !dbg !8414
  %741 = sub i32 %ring.i1803, %left_end.sroa.0.0.i1811, !dbg !8416
  %_27.sroa.0.0.i1830 = tail call i32 @llvm.umin.i32(i32 %741, i32 %_25.sroa.0.0.i1828), !dbg !8417
  %742 = sub i32 %ring.i1803, %right_end.sroa.0.0.i1815, !dbg !8419
  %_29.sroa.0.0.i1832 = tail call i32 @llvm.umin.i32(i32 %742, i32 %_27.sroa.0.0.i1830), !dbg !8420
  %743 = sub i32 %ring.i1803, %left_expiring.sroa.0.0.i1819, !dbg !8422
  %_31.sroa.0.0.i1834 = tail call i32 @llvm.umin.i32(i32 %743, i32 %_29.sroa.0.0.i1832), !dbg !8423
  %744 = sub i32 %ring.i1803, %right_expiring.sroa.0.0.i1823, !dbg !8425
  %run.sroa.0.0.i1836 = tail call i32 @llvm.umin.i32(i32 %744, i32 %_31.sroa.0.0.i1834), !dbg !8426
  %_72.i = add i32 %frame.sroa.0.0.i9041, %iter1.sroa.0.0.i9191, !dbg !8428
  %_76.i = add i32 %run.sroa.0.0.i1836, %_72.i, !dbg !8431
  %_172.i = icmp ult i32 %_76.i, %_72.i, !dbg !8434
  %_166.not.i = icmp ugt i32 %_76.i, %left_io.1
  %or.cond.i = or i1 %_172.i, %_166.not.i, !dbg !8434
  br i1 %or.cond.i, label %bb51.i, label %bb49.i, !dbg !8434, !prof !3544

bb51.i:                                           ; preds = %bb20.i
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !7920
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !7921
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store i32 %storemerge.i1245.lcssa88369045.lcssa14889, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88639081.lcssa14910, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89489117.lcssa14931, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89759153.lcssa14952, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219214337.lcssa14974, ptr %705, align 4
  store float %_0.i3236.lcssa1220814355.lcssa14996, ptr %707, align 4
  store float %_0.i2858.lcssa1223214373.lcssa15018, ptr %713, align 4
  store float %_0.i3232.lcssa1223314391.lcssa15040, ptr %715, align 4
  store float %_0.i2862.lcssa1219214338, ptr %705, align 4
  store float %_0.i3236.lcssa1220814356, ptr %707, align 4
  store float %_0.i2858.lcssa1223214374, ptr %713, align 4
  store float %_0.i3232.lcssa1223314392, ptr %715, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_72.i, i32 noundef %_76.i, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_bd3c2aadcc2cdd6816f02be0ceab2cc1) #28, !dbg !8441, !noalias !7828
  unreachable, !dbg !8441

bb49.i:                                           ; preds = %bb20.i
  %_175.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %_72.i, !dbg !8442
  %_176.not.i = icmp ugt i32 %_76.i, %right_io.1, !dbg !8446
  br i1 %_176.not.i, label %bb54.i, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4181, !dbg !8446, !prof !755

bb54.i:                                           ; preds = %bb49.i
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !7920
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !7921
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store i32 %storemerge.i1245.lcssa88369045.lcssa14889, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88639081.lcssa14910, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89489117.lcssa14931, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89759153.lcssa14952, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219214337.lcssa14974, ptr %705, align 4
  store float %_0.i3236.lcssa1220814355.lcssa14996, ptr %707, align 4
  store float %_0.i2858.lcssa1223214373.lcssa15018, ptr %713, align 4
  store float %_0.i3232.lcssa1223314391.lcssa15040, ptr %715, align 4
  store float %_0.i2862.lcssa1219214338, ptr %705, align 4
  store float %_0.i3236.lcssa1220814356, ptr %707, align 4
  store float %_0.i2858.lcssa1223214374, ptr %713, align 4
  store float %_0.i3232.lcssa1223314392, ptr %715, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_72.i, i32 noundef %_76.i, i32 noundef range(i32 0, 536870912) %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c1156ffa98f252967759477badafe965) #28, !dbg !8451, !noalias !7828
  unreachable, !dbg !8451

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4181: ; preds = %bb49.i
  %_183.i = getelementptr inbounds nuw float, ptr %right_io.0, i32 %_72.i, !dbg !8452
  %_83.i = add nuw nsw i32 %run.sroa.0.0.i1836, %frame.sroa.0.0.i9041, !dbg !8456
  %_192.i = getelementptr inbounds nuw float, ptr %peaks_left.i, i32 %frame.sroa.0.0.i9041, !dbg !8458
  %_201.i = getelementptr inbounds nuw float, ptr %peaks_right.i, i32 %frame.sroa.0.0.i9041, !dbg !8468
  %_2.i41848811.not = icmp eq i32 %run.sroa.0.0.i1836, 0, !dbg !8478
  br i1 %_2.i41848811.not, label %bb67.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048.lr.ph, !dbg !8478

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048.lr.ph: ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4181
  %umax11965 = call i32 @llvm.umax.i32(i32 %ring_cursor.sroa.0.1.i9039, i32 %_54.1.i257.i), !dbg !8478
  %umax11966 = call i32 @llvm.umax.i32(i32 %main_cursor.sroa.0.1.i9040, i32 %_58.1.i304.i), !dbg !8478
  %745 = sub i32 %umax11965, %ring_cursor.sroa.0.1.i9039, !dbg !8478
  %746 = sub i32 %umax11966, %main_cursor.sroa.0.1.i9040, !dbg !8478
  %umin11969 = call i32 @llvm.umin.i32(i32 %741, i32 %742), !dbg !8478
  %umin11970 = call i32 @llvm.umin.i32(i32 %umin11969, i32 %743), !dbg !8478
  %umin11971 = call i32 @llvm.umin.i32(i32 %umin11970, i32 %744), !dbg !8478
  %umin11972 = call i32 @llvm.umin.i32(i32 %umin11971, i32 %740), !dbg !8478
  %umin11973 = call i32 @llvm.umin.i32(i32 %umin11972, i32 %738), !dbg !8478
  %umin11974 = call i32 @llvm.umin.i32(i32 %umin11973, i32 %739), !dbg !8478
  %747 = sub nsw i32 %umin11975, %frame.sroa.0.0.i9041, !dbg !8478
  %umin11976 = call i32 @llvm.umin.i32(i32 %umin11974, i32 %747), !dbg !8478
  br label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048, !dbg !8478

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195
  %_0.i32329010 = phi float [ %_0.i3232.lcssa1223314392, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048.lr.ph ], [ %_0.i3232, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %_0.i28588981 = phi float [ %_0.i2858.lcssa1223214374, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048.lr.ph ], [ %_0.i2858, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %running.sroa.0.0.i12108953 = phi float [ %running.sroa.0.0.i1210.lcssa89759154, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048.lr.ph ], [ %running.sroa.0.0.i1210, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %storemerge.i12158926 = phi i32 [ %storemerge.i1215.lcssa89489118, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048.lr.ph ], [ %storemerge.i1215, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %_0.i32368898 = phi float [ %_0.i3236.lcssa1220814356, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048.lr.ph ], [ %_0.i3236, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %_0.i28628869 = phi float [ %_0.i2862.lcssa1219214338, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048.lr.ph ], [ %_0.i2862, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %running.sroa.0.0.i12408841 = phi float [ %running.sroa.0.0.i1240.lcssa88639082, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048.lr.ph ], [ %running.sroa.0.0.i1240, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %storemerge.i12458814 = phi i32 [ %storemerge.i1245.lcssa88369046, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048.lr.ph ], [ %storemerge.i1245, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %iter.i.sroa.41.08813 = phi i32 [ 0, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048.lr.ph ], [ %_206.0.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %_206.0.i = add nuw i32 %iter.i.sroa.41.08813, 1, !dbg !8487
  %data.i.i.i.i.i.i4195 = getelementptr inbounds nuw float, ptr %_175.i, i32 %iter.i.sroa.41.08813, !dbg !8490
  %data.i5.i.i.i.i.i4199 = getelementptr inbounds nuw float, ptr %_183.i, i32 %iter.i.sroa.41.08813, !dbg !8497
  %data.i.i.i.i4204 = getelementptr inbounds nuw float, ptr %_192.i, i32 %iter.i.sroa.41.08813, !dbg !8500
  %data.i.i4209 = getelementptr inbounds nuw float, ptr %_201.i, i32 %iter.i.sroa.41.08813, !dbg !8503
  %_0.i3061 = load float, ptr %data.i.i.i.i4204, align 4, !dbg !8506, !alias.scope !8511, !noalias !7828, !noundef !10
  %_0.i3056 = load float, ptr %data.i.i4209, align 4, !dbg !8514, !alias.scope !8517, !noalias !7828, !noundef !10
  %_3.i.i3719 = fcmp ule float %_0.i3056, %_0.i3061, !dbg !8520
  %_6.i.i3721 = bitcast float %_0.i3056 to i32, !dbg !8524
  %_8.i.i3723 = bitcast float %_0.i3061 to i32, !dbg !8527
  %_4.i.i3726 = select i1 %_3.i.i3719, i32 %_8.i.i3723, i32 %_6.i.i3721, !dbg !8529
  %_5.i3492 = and i32 %_4.i.i3726, %.none.i, !dbg !8530
  %_7.i3488 = and i32 %_9.i3494, %_6.i.i3721, !dbg !8533
  %_4.i3489 = or disjoint i32 %_5.i3492, %_7.i3488, !dbg !8536
  %_0.i3490 = bitcast i32 %_4.i3489 to float, !dbg !8537
  %_0.i3051 = load float, ptr %data.i.i.i.i.i.i4195, align 4, !dbg !8539, !alias.scope !8542, !noalias !7828, !noundef !10
  %_0.i3046 = load float, ptr %data.i5.i.i.i.i.i4199, align 4, !dbg !8545, !alias.scope !8548, !noalias !7828, !noundef !10
  %_213.i = add nuw i32 %iter.i.sroa.41.08813, %ring_cursor.sroa.0.1.i9039, !dbg !8551
  %_214.i = add nuw i32 %iter.i.sroa.41.08813, %main_cursor.sroa.0.1.i9040, !dbg !8555
  %_215.i = add nuw i32 %iter.i.sroa.41.08813, %left_end.sroa.0.0.i1811, !dbg !8556
  %_216.i = add i32 %iter.i.sroa.41.08813, %start1.sroa.0.0.i1807, !dbg !8557
  %_217.i = add nuw i32 %iter.i.sroa.41.08813, %left_expiring.sroa.0.0.i1819, !dbg !8558
  %_7.i8.i259.i = add i32 %_213.i, 1, !dbg !8559
  %exitcond11967.not = icmp eq i32 %iter.i.sroa.41.08813, %745, !dbg !8562
  br i1 %exitcond11967.not, label %bb4.i13.i318.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i263.i, !dbg !8562, !prof !3544

bb4.i13.i318.i:                                   ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !7920
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !7921
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store i32 %storemerge.i1245.lcssa88369045.lcssa14889, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88639081.lcssa14910, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89489117.lcssa14931, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89759153.lcssa14952, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219214337.lcssa14974, ptr %705, align 4
  store float %_0.i3236.lcssa1220814355.lcssa14996, ptr %707, align 4
  store float %_0.i2858.lcssa1223214373.lcssa15018, ptr %713, align 4
  store float %_0.i3232.lcssa1223314391.lcssa15040, ptr %715, align 4
  store float %_0.i2862.lcssa1219214338, ptr %705, align 4
  store float %_0.i3236.lcssa1220814356, ptr %707, align 4
  store float %_0.i2858.lcssa1223214374, ptr %713, align 4
  store float %_0.i3232.lcssa1223314392, ptr %715, align 4
  %748 = add i32 %umax11965, 1, !dbg !8478
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_213.i, i32 noundef %748, i32 noundef range(i32 0, 536870912) %_54.1.i257.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #28, !dbg !8566, !noalias !8567
  unreachable, !dbg !8566

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i263.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048
  %_7.i3495 = and i32 %_9.i3494, %_8.i.i3723, !dbg !8575
  %_4.i3496 = or disjoint i32 %_5.i3492, %_7.i3495, !dbg !8530
  %_0.i3497 = bitcast i32 %_4.i3496 to float, !dbg !8576
  %_0.i2398 = fdiv float %_8.i29.i, %_0.i3497, !dbg !8578
  %_3.i1965 = fcmp uge float %_8.i29.i, %_0.i3497, !dbg !8580
  %_0.i3483 = select i1 %_3.i1965, float 1.000000e+00, float %_0.i2398, !dbg !8582
  %_17.i12.i264.i = getelementptr inbounds nuw float, ptr %_54.0.i256.i, i32 %_213.i, !dbg !8584
  store float %_0.i3483, ptr %_17.i12.i264.i, align 4, !dbg !8586, !alias.scope !8588, !noalias !8591
  %or.cond.i1287.not = icmp ult i32 %_215.i, %_54.1.i257.i, !dbg !8592
  br i1 %or.cond.i1287.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1291, label %bb4.i1290, !dbg !8592, !prof !7284

bb4.i1290:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i263.i
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !7920
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !7921
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store i32 %storemerge.i1245.lcssa88369045.lcssa14889, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88639081.lcssa14910, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89489117.lcssa14931, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89759153.lcssa14952, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219214337.lcssa14974, ptr %705, align 4
  store float %_0.i3236.lcssa1220814355.lcssa14996, ptr %707, align 4
  store float %_0.i2858.lcssa1223214373.lcssa15018, ptr %713, align 4
  store float %_0.i3232.lcssa1223314391.lcssa15040, ptr %715, align 4
  store float %_0.i2862.lcssa1219214338, ptr %705, align 4
  store float %_0.i3236.lcssa1220814356, ptr %707, align 4
  store float %_0.i2858.lcssa1223214374, ptr %713, align 4
  store float %_0.i3232.lcssa1223314392, ptr %715, align 4
  %_5.i1284 = add i32 %_215.i, 1, !dbg !8598
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_215.i, i32 noundef %_5.i1284, i32 noundef range(i32 0, 536870912) %_54.1.i257.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #28, !dbg !8599, !noalias !8600
  unreachable, !dbg !8599

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1291: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i263.i
  %_15.i1288 = getelementptr inbounds nuw float, ptr %_54.0.i256.i, i32 %_215.i, !dbg !8606
  %_0.i2904 = load float, ptr %_15.i1288, align 4, !dbg !8608, !alias.scope !8610, !noalias !8613, !noundef !10
  %749 = icmp eq i32 %storemerge.i12458814, 0, !dbg !8614
  %_3.i.i3746.inv = fcmp olt float %running.sroa.0.0.i12408841, %_0.i2904, !dbg !8614
  %_4.i.i3753.v = select i1 %_3.i.i3746.inv, float %running.sroa.0.0.i12408841, float %_0.i2904, !dbg !8614
  %running.sroa.0.0.i1240 = select i1 %749, float %_0.i2904, float %_4.i.i3753.v, !dbg !8614
  %_15.i1241 = add i32 %storemerge.i12458814, 1, !dbg !8615
  %complete.i1242 = icmp eq i32 %_15.i1241, %_18.i267.i, !dbg !8615
  br i1 %complete.i1242, label %bb11.i1248.preheader, label %bb7.i1243, !dbg !8616

bb11.i1248.preheader:                             ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1291
  br i1 %_29.i12528803.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1264, label %bb19.i1253, !dbg !8617

bb7.i1243:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1291
  %or.cond.i1279.not = icmp ult i32 %_216.i, %_54.1.i257.i, !dbg !8620
  br i1 %or.cond.i1279.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1283, label %bb4.i1282, !dbg !8620, !prof !7284

bb4.i1282:                                        ; preds = %bb7.i1243
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !7920
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !7921
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store i32 %storemerge.i1245.lcssa88369045.lcssa14889, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88639081.lcssa14910, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89489117.lcssa14931, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89759153.lcssa14952, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219214337.lcssa14974, ptr %705, align 4
  store float %_0.i3236.lcssa1220814355.lcssa14996, ptr %707, align 4
  store float %_0.i2858.lcssa1223214373.lcssa15018, ptr %713, align 4
  store float %_0.i3232.lcssa1223314391.lcssa15040, ptr %715, align 4
  store float %_0.i2862.lcssa1219214338, ptr %705, align 4
  store float %_0.i3236.lcssa1220814356, ptr %707, align 4
  store float %_0.i2858.lcssa1223214374, ptr %713, align 4
  store float %_0.i3232.lcssa1223314392, ptr %715, align 4
  %_5.i1276 = add i32 %_216.i, 1, !dbg !8625
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_216.i, i32 noundef %_5.i1276, i32 noundef range(i32 0, 536870912) %_54.1.i257.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #28, !dbg !8626, !noalias !8627
  unreachable, !dbg !8626

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1283: ; preds = %bb7.i1243
  %_15.i1280 = getelementptr inbounds nuw float, ptr %_54.0.i256.i, i32 %_216.i, !dbg !8630
  %_0.i2906 = load float, ptr %_15.i1280, align 4, !dbg !8632, !alias.scope !8634, !noalias !8613, !noundef !10
  %_3.i.i3737.inv = fcmp olt float %_0.i2906, %running.sroa.0.0.i1240, !dbg !8637
  %_4.i.i3744.v = select i1 %_3.i.i3737.inv, float %_0.i2906, float %running.sroa.0.0.i1240, !dbg !8637
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1264, !dbg !8640

bb19.i1253:                                       ; preds = %bb11.i1248.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1260
  %end.sroa.0.0.i12518806 = phi i32 [ %751, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1260 ], [ %_215.i, %bb11.i1248.preheader ]
  %suffix.sroa.0.0.i12508805 = phi float [ %_4.i.i3735.v, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1260 ], [ %_0.i2904, %bb11.i1248.preheader ]
  %iter.sroa.0.0.i12498804 = phi i32 [ %_30.i1254, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1260 ], [ 0, %bb11.i1248.preheader ]
  %or.cond.i1266.not = icmp ult i32 %end.sroa.0.0.i12518806, %_54.1.i257.i, !dbg !8641
  br i1 %or.cond.i1266.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1260, label %bb4.i, !dbg !8641, !prof !7284

bb4.i:                                            ; preds = %bb19.i1253
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !7920
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !7921
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store i32 %storemerge.i1245.lcssa88369045.lcssa14889, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88639081.lcssa14910, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89489117.lcssa14931, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89759153.lcssa14952, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219214337.lcssa14974, ptr %705, align 4
  store float %_0.i3236.lcssa1220814355.lcssa14996, ptr %707, align 4
  store float %_0.i2858.lcssa1223214373.lcssa15018, ptr %713, align 4
  store float %_0.i3232.lcssa1223314391.lcssa15040, ptr %715, align 4
  store float %_0.i2862.lcssa1219214338, ptr %705, align 4
  store float %_0.i3236.lcssa1220814356, ptr %707, align 4
  store float %_0.i2858.lcssa1223214374, ptr %713, align 4
  store float %_0.i3232.lcssa1223314392, ptr %715, align 4
  %_5.i = add i32 %end.sroa.0.0.i12518806, 1, !dbg !8646
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %end.sroa.0.0.i12518806, i32 noundef %_5.i, i32 noundef range(i32 0, 536870912) %_54.1.i257.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #28, !dbg !8647, !noalias !8648
  unreachable, !dbg !8647

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1260: ; preds = %bb19.i1253
  %_30.i1254 = add nuw i32 %iter.sroa.0.0.i12498804, 1, !dbg !8651
  %_15.i1267 = getelementptr inbounds nuw float, ptr %_54.0.i256.i, i32 %end.sroa.0.0.i12518806, !dbg !8654
  %_0.i2910 = load float, ptr %_15.i1267, align 4, !dbg !8656, !alias.scope !8658, !noalias !8613, !noundef !10
  %_3.i.i3728.inv = fcmp olt float %suffix.sroa.0.0.i12508805, %_0.i2910, !dbg !8661
  %_4.i.i3735.v = select i1 %_3.i.i3728.inv, float %suffix.sroa.0.0.i12508805, float %_0.i2910, !dbg !8661
  store float %_4.i.i3735.v, ptr %_15.i1267, align 4, !dbg !8664, !alias.scope !8667, !noalias !8613
  %750 = icmp eq i32 %end.sroa.0.0.i12518806, 0, !dbg !8670
  %spec.store.select.i1262 = select i1 %750, i32 %ring.i, i32 %end.sroa.0.0.i12518806, !dbg !8670
  %751 = add i32 %spec.store.select.i1262, -1, !dbg !8671
  %exitcond11963.not = icmp eq i32 %_30.i1254, %_18.i267.i, !dbg !8672
  br i1 %exitcond11963.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1264, label %bb19.i1253, !dbg !8617

_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1264: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1260, %bb11.i1248.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1283
  %storemerge.i1245 = phi i32 [ %_15.i1241, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1283 ], [ 0, %bb11.i1248.preheader ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1260 ], !dbg !8674
  %running.sroa.0.1.i1246 = phi float [ %_4.i.i3744.v, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1283 ], [ %running.sroa.0.0.i1240, %bb11.i1248.preheader ], [ %running.sroa.0.0.i1240, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1260 ], !dbg !8675
  %_0.i2822 = fmul float %running.sroa.0.1.i1246, 1.638400e+04, !dbg !8676
  %752 = tail call noundef float @llvm.floor.f32(float %_0.i2822), !dbg !8678
  %_0.i2821 = fmul float %752, 0x3F10000000000000, !dbg !8682
  %or.cond.i1447.not = icmp ult i32 %_217.i, %_56.1.i279.i, !dbg !8684
  br i1 %or.cond.i1447.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1451, label %bb4.i1450, !dbg !8684, !prof !7284

bb4.i1450:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1264
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !7920
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !7921
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store i32 %storemerge.i1245.lcssa88369045.lcssa14889, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88639081.lcssa14910, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89489117.lcssa14931, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89759153.lcssa14952, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219214337.lcssa14974, ptr %705, align 4
  store float %_0.i3236.lcssa1220814355.lcssa14996, ptr %707, align 4
  store float %_0.i2858.lcssa1223214373.lcssa15018, ptr %713, align 4
  store float %_0.i3232.lcssa1223314391.lcssa15040, ptr %715, align 4
  store float %_0.i2862.lcssa1219214338, ptr %705, align 4
  store float %_0.i3236.lcssa1220814356, ptr %707, align 4
  store float %_0.i2858.lcssa1223214374, ptr %713, align 4
  store float %_0.i3232.lcssa1223314392, ptr %715, align 4
  %_5.i1444 = add i32 %_217.i, 1, !dbg !8689
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_217.i, i32 noundef %_5.i1444, i32 noundef range(i32 0, 536870912) %_56.1.i279.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #28, !dbg !8690, !noalias !8691
  unreachable, !dbg !8690

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1451: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1264
  %_15.i1448 = getelementptr inbounds nuw float, ptr %_56.0.i278.i, i32 %_217.i, !dbg !8694
  %_0.i2864 = load float, ptr %_15.i1448, align 4, !dbg !8696, !alias.scope !8698, !noalias !8701, !noundef !10
  %_0.i2382 = fadd float %_0.i2821, %_0.i28628869, !dbg !8702
  %_0.i2862 = fsub float %_0.i2382, %_0.i2864, !dbg !8704
  %_8.not.i3.i288.i = icmp ugt i32 %_7.i8.i259.i, %_56.1.i279.i
  br i1 %_8.not.i3.i288.i, label %bb4.i6.i317.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i289.i, !dbg !8706, !prof !3544

bb4.i6.i317.i:                                    ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1451
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !7920
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !7921
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store i32 %storemerge.i1245.lcssa88369045.lcssa14889, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88639081.lcssa14910, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89489117.lcssa14931, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89759153.lcssa14952, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219214337.lcssa14974, ptr %705, align 4
  store float %_0.i3236.lcssa1220814355.lcssa14996, ptr %707, align 4
  store float %_0.i2858.lcssa1223214373.lcssa15018, ptr %713, align 4
  store float %_0.i3232.lcssa1223314391.lcssa15040, ptr %715, align 4
  store float %_0.i2862.lcssa1219214338, ptr %705, align 4
  store float %_0.i3236.lcssa1220814356, ptr %707, align 4
  store float %_0.i2858.lcssa1223214374, ptr %713, align 4
  store float %_0.i3232.lcssa1223314392, ptr %715, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_213.i, i32 noundef %_7.i8.i259.i, i32 noundef range(i32 0, 536870912) %_56.1.i279.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #28, !dbg !8711, !noalias !8712
  unreachable, !dbg !8711

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i289.i: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1451
  %_17.i5.i290.i = getelementptr inbounds nuw float, ptr %_56.0.i278.i, i32 %_213.i, !dbg !8715
  store float %_0.i2821, ptr %_17.i5.i290.i, align 4, !dbg !8717, !alias.scope !8719, !noalias !8701
  %_0.i2397 = fdiv float %_0.i2862, %_37.i291.i, !dbg !8722
  %_0.i2861 = fsub float 1.000000e+00, %_0.i2397, !dbg !8724
  %_0.i2860 = fsub float %_0.i2861, %_0.i32368898, !dbg !8726
  %_4.i2413 = fmul float %_9.i30.i, %_0.i2860, !dbg !8728
  %_0.i2414 = fadd float %_0.i32368898, %_4.i2413, !dbg !8728
  %_3.i.i3710.inv = fcmp ogt float %_0.i2861, %_0.i2414, !dbg !8730
  %_4.i.i3717.v = select i1 %_3.i.i3710.inv, float %_0.i2861, float %_0.i2414, !dbg !8730
  %753 = tail call noundef float @llvm.fabs.f32(float %_4.i.i3717.v), !dbg !8733
  %754 = fcmp uge float %753, 0x3BC79CA100000000, !dbg !8736
  %_0.i3236 = select i1 %754, float %_4.i.i3717.v, float 0.000000e+00, !dbg !8738
  %_5.i1436 = add i32 %_214.i, 1, !dbg !8739
  %exitcond11968.not = icmp eq i32 %iter.i.sroa.41.08813, %746, !dbg !8741
  br i1 %exitcond11968.not, label %bb4.i1442, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3202, !dbg !8741, !prof !3544

bb4.i1442:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i289.i
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !7920
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !7921
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store i32 %storemerge.i1245.lcssa88369045.lcssa14889, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88639081.lcssa14910, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89489117.lcssa14931, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89759153.lcssa14952, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219214337.lcssa14974, ptr %705, align 4
  store float %_0.i3236.lcssa1220814355.lcssa14996, ptr %707, align 4
  store float %_0.i2858.lcssa1223214373.lcssa15018, ptr %713, align 4
  store float %_0.i3232.lcssa1223314391.lcssa15040, ptr %715, align 4
  store float %_0.i2862.lcssa1219214338, ptr %705, align 4
  store float %_0.i3236.lcssa1220814356, ptr %707, align 4
  store float %_0.i2858.lcssa1223214374, ptr %713, align 4
  store float %_0.i3232.lcssa1223314392, ptr %715, align 4
  %755 = add i32 %umax11966, 1, !dbg !8478
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_214.i, i32 noundef %755, i32 noundef range(i32 0, 536870912) %_58.1.i304.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #28, !dbg !8745, !noalias !8746
  unreachable, !dbg !8745

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3202: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i289.i
  %_0.i2859 = fsub float 1.000000e+00, %_0.i3236, !dbg !8749
  %_15.i1440 = getelementptr inbounds nuw float, ptr %_58.0.i303.i, i32 %_214.i, !dbg !8751
  %_0.i2866 = load float, ptr %_15.i1440, align 4, !dbg !8753, !alias.scope !8755, !noalias !8701, !noundef !10
  store float %_0.i3051, ptr %_15.i1440, align 4, !dbg !8758, !alias.scope !8761, !noalias !8701
  %_0.i2820 = fmul float %_0.i2859, %_0.i2866, !dbg !8764
  %_6.i3471 = bitcast float %_0.i2866 to i32, !dbg !8766
  %_5.i3472 = and i32 %_6.i3471, %all.sroa.0.0.i, !dbg !8769
  %_8.i3473 = bitcast float %_0.i2820 to i32, !dbg !8770
  %_7.i3475 = and i32 %_9.i3474, %_8.i3473, !dbg !8772
  %_4.i3476 = or disjoint i32 %_7.i3475, %_5.i3472, !dbg !8769
  store i32 %_4.i3476, ptr %data.i.i.i.i.i.i4195, align 4, !dbg !8773, !alias.scope !8775, !noalias !8778
  %_220.i = add nuw i32 %iter.i.sroa.41.08813, %right_end.sroa.0.0.i1815, !dbg !8779
  %_222.i = add nuw i32 %iter.i.sroa.41.08813, %right_expiring.sroa.0.0.i1823, !dbg !8781
  %_8.not.i10.i.i = icmp ugt i32 %_7.i8.i259.i, %_54.1.i.i
  br i1 %_8.not.i10.i.i, label %bb4.i13.i.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i, !dbg !8782, !prof !3544

bb4.i13.i.i:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3202
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !7920
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !7921
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store i32 %storemerge.i1245.lcssa88369045.lcssa14889, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88639081.lcssa14910, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89489117.lcssa14931, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89759153.lcssa14952, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219214337.lcssa14974, ptr %705, align 4
  store float %_0.i3236.lcssa1220814355.lcssa14996, ptr %707, align 4
  store float %_0.i2858.lcssa1223214373.lcssa15018, ptr %713, align 4
  store float %_0.i3232.lcssa1223314391.lcssa15040, ptr %715, align 4
  store float %_0.i2862.lcssa1219214338, ptr %705, align 4
  store float %_0.i3236.lcssa1220814356, ptr %707, align 4
  store float %_0.i2858.lcssa1223214374, ptr %713, align 4
  store float %_0.i3232.lcssa1223314392, ptr %715, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_213.i, i32 noundef %_7.i8.i259.i, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #28, !dbg !8788, !noalias !8789
  unreachable, !dbg !8788

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3202
  %_0.i2396 = fdiv float %_8.i.i, %_0.i3490, !dbg !8797
  %_3.i1963 = fcmp uge float %_8.i.i, %_0.i3490, !dbg !8799
  %_0.i3470 = select i1 %_3.i1963, float 1.000000e+00, float %_0.i2396, !dbg !8801
  %_17.i12.i.i = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %_213.i, !dbg !8803
  store float %_0.i3470, ptr %_17.i12.i.i, align 4, !dbg !8805, !alias.scope !8807, !noalias !8810
  %or.cond.i1319.not = icmp ult i32 %_220.i, %_54.1.i.i, !dbg !8811
  br i1 %or.cond.i1319.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1323, label %bb4.i1322, !dbg !8811, !prof !7284

bb4.i1322:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !7920
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !7921
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store i32 %storemerge.i1245.lcssa88369045.lcssa14889, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88639081.lcssa14910, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89489117.lcssa14931, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89759153.lcssa14952, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219214337.lcssa14974, ptr %705, align 4
  store float %_0.i3236.lcssa1220814355.lcssa14996, ptr %707, align 4
  store float %_0.i2858.lcssa1223214373.lcssa15018, ptr %713, align 4
  store float %_0.i3232.lcssa1223314391.lcssa15040, ptr %715, align 4
  store float %_0.i2862.lcssa1219214338, ptr %705, align 4
  store float %_0.i3236.lcssa1220814356, ptr %707, align 4
  store float %_0.i2858.lcssa1223214374, ptr %713, align 4
  store float %_0.i3232.lcssa1223314392, ptr %715, align 4
  %_5.i1316 = add i32 %_220.i, 1, !dbg !8817
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_220.i, i32 noundef %_5.i1316, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #28, !dbg !8818, !noalias !8819
  unreachable, !dbg !8818

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1323: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit14.i.i
  %_15.i1320 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %_220.i, !dbg !8825
  %_0.i2896 = load float, ptr %_15.i1320, align 4, !dbg !8827, !alias.scope !8829, !noalias !8832, !noundef !10
  %756 = icmp eq i32 %storemerge.i12158926, 0, !dbg !8833
  %_3.i.i3773.inv = fcmp olt float %running.sroa.0.0.i12108953, %_0.i2896, !dbg !8833
  %_4.i.i3780.v = select i1 %_3.i.i3773.inv, float %running.sroa.0.0.i12108953, float %_0.i2896, !dbg !8833
  %running.sroa.0.0.i1210 = select i1 %756, float %_0.i2896, float %_4.i.i3780.v, !dbg !8833
  %_15.i1211 = add i32 %storemerge.i12158926, 1, !dbg !8834
  %complete.i1212 = icmp eq i32 %_15.i1211, %_18.i.i, !dbg !8834
  br i1 %complete.i1212, label %bb11.i1218.preheader, label %bb7.i1213, !dbg !8835

bb11.i1218.preheader:                             ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1323
  br i1 %_29.i12228807.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1234, label %bb19.i1223, !dbg !8836

bb7.i1213:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1323
  %or.cond.i1311.not = icmp ult i32 %_216.i, %_54.1.i.i, !dbg !8839
  br i1 %or.cond.i1311.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1315, label %bb4.i1314, !dbg !8839, !prof !7284

bb4.i1314:                                        ; preds = %bb7.i1213
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !7920
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !7921
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store i32 %storemerge.i1245.lcssa88369045.lcssa14889, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88639081.lcssa14910, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89489117.lcssa14931, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89759153.lcssa14952, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219214337.lcssa14974, ptr %705, align 4
  store float %_0.i3236.lcssa1220814355.lcssa14996, ptr %707, align 4
  store float %_0.i2858.lcssa1223214373.lcssa15018, ptr %713, align 4
  store float %_0.i3232.lcssa1223314391.lcssa15040, ptr %715, align 4
  store float %_0.i2862.lcssa1219214338, ptr %705, align 4
  store float %_0.i3236.lcssa1220814356, ptr %707, align 4
  store float %_0.i2858.lcssa1223214374, ptr %713, align 4
  store float %_0.i3232.lcssa1223314392, ptr %715, align 4
  %_5.i1308 = add i32 %_216.i, 1, !dbg !8844
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_216.i, i32 noundef %_5.i1308, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #28, !dbg !8845, !noalias !8846
  unreachable, !dbg !8845

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1315: ; preds = %bb7.i1213
  %_15.i1312 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %_216.i, !dbg !8849
  %_0.i2898 = load float, ptr %_15.i1312, align 4, !dbg !8851, !alias.scope !8853, !noalias !8832, !noundef !10
  %_3.i.i3764.inv = fcmp olt float %_0.i2898, %running.sroa.0.0.i1210, !dbg !8856
  %_4.i.i3771.v = select i1 %_3.i.i3764.inv, float %_0.i2898, float %running.sroa.0.0.i1210, !dbg !8856
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1234, !dbg !8859

bb19.i1223:                                       ; preds = %bb11.i1218.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230
  %end.sroa.0.0.i12218810 = phi i32 [ %758, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230 ], [ %_220.i, %bb11.i1218.preheader ]
  %suffix.sroa.0.0.i12208809 = phi float [ %_4.i.i3762.v, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230 ], [ %_0.i2896, %bb11.i1218.preheader ]
  %iter.sroa.0.0.i12198808 = phi i32 [ %_30.i1224, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230 ], [ 0, %bb11.i1218.preheader ]
  %or.cond.i1295.not = icmp ult i32 %end.sroa.0.0.i12218810, %_54.1.i.i, !dbg !8860
  br i1 %or.cond.i1295.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230, label %bb4.i1298, !dbg !8860, !prof !7284

bb4.i1298:                                        ; preds = %bb19.i1223
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !7920
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !7921
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store i32 %storemerge.i1245.lcssa88369045.lcssa14889, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88639081.lcssa14910, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89489117.lcssa14931, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89759153.lcssa14952, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219214337.lcssa14974, ptr %705, align 4
  store float %_0.i3236.lcssa1220814355.lcssa14996, ptr %707, align 4
  store float %_0.i2858.lcssa1223214373.lcssa15018, ptr %713, align 4
  store float %_0.i3232.lcssa1223314391.lcssa15040, ptr %715, align 4
  store float %_0.i2862.lcssa1219214338, ptr %705, align 4
  store float %_0.i3236.lcssa1220814356, ptr %707, align 4
  store float %_0.i2858.lcssa1223214374, ptr %713, align 4
  store float %_0.i3232.lcssa1223314392, ptr %715, align 4
  %_5.i1292 = add i32 %end.sroa.0.0.i12218810, 1, !dbg !8865
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %end.sroa.0.0.i12218810, i32 noundef %_5.i1292, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #28, !dbg !8866, !noalias !8867
  unreachable, !dbg !8866

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230: ; preds = %bb19.i1223
  %_30.i1224 = add nuw i32 %iter.sroa.0.0.i12198808, 1, !dbg !8870
  %_15.i1296 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %end.sroa.0.0.i12218810, !dbg !8873
  %_0.i2902 = load float, ptr %_15.i1296, align 4, !dbg !8875, !alias.scope !8877, !noalias !8832, !noundef !10
  %_3.i.i3755.inv = fcmp olt float %suffix.sroa.0.0.i12208809, %_0.i2902, !dbg !8880
  %_4.i.i3762.v = select i1 %_3.i.i3755.inv, float %suffix.sroa.0.0.i12208809, float %_0.i2902, !dbg !8880
  store float %_4.i.i3762.v, ptr %_15.i1296, align 4, !dbg !8883, !alias.scope !8886, !noalias !8832
  %757 = icmp eq i32 %end.sroa.0.0.i12218810, 0, !dbg !8889
  %spec.store.select.i1232 = select i1 %757, i32 %ring.i, i32 %end.sroa.0.0.i12218810, !dbg !8889
  %758 = add i32 %spec.store.select.i1232, -1, !dbg !8890
  %exitcond11964.not = icmp eq i32 %_30.i1224, %_18.i.i, !dbg !8891
  br i1 %exitcond11964.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1234, label %bb19.i1223, !dbg !8836

_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1234: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230, %bb11.i1218.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1315
  %storemerge.i1215 = phi i32 [ %_15.i1211, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1315 ], [ 0, %bb11.i1218.preheader ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230 ], !dbg !8893
  %running.sroa.0.1.i1216 = phi float [ %_4.i.i3771.v, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1315 ], [ %running.sroa.0.0.i1210, %bb11.i1218.preheader ], [ %running.sroa.0.0.i1210, %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit.i1230 ], !dbg !8894
  %_0.i2819 = fmul float %running.sroa.0.1.i1216, 1.638400e+04, !dbg !8895
  %759 = tail call noundef float @llvm.floor.f32(float %_0.i2819), !dbg !8897
  %_0.i2818 = fmul float %759, 0x3F10000000000000, !dbg !8901
  %or.cond.i1431.not = icmp ult i32 %_222.i, %_56.1.i.i, !dbg !8903
  br i1 %or.cond.i1431.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1435, label %bb4.i1434, !dbg !8903, !prof !7284

bb4.i1434:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1234
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !7920
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !7921
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store i32 %storemerge.i1245.lcssa88369045.lcssa14889, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88639081.lcssa14910, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89489117.lcssa14931, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89759153.lcssa14952, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219214337.lcssa14974, ptr %705, align 4
  store float %_0.i3236.lcssa1220814355.lcssa14996, ptr %707, align 4
  store float %_0.i2858.lcssa1223214373.lcssa15018, ptr %713, align 4
  store float %_0.i3232.lcssa1223314391.lcssa15040, ptr %715, align 4
  store float %_0.i2862.lcssa1219214338, ptr %705, align 4
  store float %_0.i3236.lcssa1220814356, ptr %707, align 4
  store float %_0.i2858.lcssa1223214374, ptr %713, align 4
  store float %_0.i3232.lcssa1223314392, ptr %715, align 4
  %_5.i1428 = add i32 %_222.i, 1, !dbg !8908
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_222.i, i32 noundef %_5.i1428, i32 noundef range(i32 0, 536870912) %_56.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #28, !dbg !8909, !noalias !8910
  unreachable, !dbg !8909

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1435: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformfEB2_.exit1234
  %_15.i1432 = getelementptr inbounds nuw float, ptr %_56.0.i.i, i32 %_222.i, !dbg !8913
  %_0.i2868 = load float, ptr %_15.i1432, align 4, !dbg !8915, !alias.scope !8917, !noalias !8920, !noundef !10
  %_0.i2381 = fadd float %_0.i2818, %_0.i28588981, !dbg !8921
  %_0.i2858 = fsub float %_0.i2381, %_0.i2868, !dbg !8923
  %_8.not.i3.i.i = icmp ugt i32 %_7.i8.i259.i, %_56.1.i.i
  br i1 %_8.not.i3.i.i, label %bb4.i6.i.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i, !dbg !8925, !prof !3544

bb4.i6.i.i:                                       ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1435
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !7920
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !7921
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store i32 %storemerge.i1245.lcssa88369045.lcssa14889, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88639081.lcssa14910, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89489117.lcssa14931, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89759153.lcssa14952, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219214337.lcssa14974, ptr %705, align 4
  store float %_0.i3236.lcssa1220814355.lcssa14996, ptr %707, align 4
  store float %_0.i2858.lcssa1223214373.lcssa15018, ptr %713, align 4
  store float %_0.i3232.lcssa1223314391.lcssa15040, ptr %715, align 4
  store float %_0.i2862.lcssa1219214338, ptr %705, align 4
  store float %_0.i3236.lcssa1220814356, ptr %707, align 4
  store float %_0.i2858.lcssa1223214374, ptr %713, align 4
  store float %_0.i3232.lcssa1223314392, ptr %715, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_213.i, i32 noundef %_7.i8.i259.i, i32 noundef range(i32 0, 536870912) %_56.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #28, !dbg !8930, !noalias !8931
  unreachable, !dbg !8930

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_lanefEB2_.exit1435
  %_17.i5.i.i = getelementptr inbounds nuw float, ptr %_56.0.i.i, i32 %_213.i, !dbg !8934
  store float %_0.i2818, ptr %_17.i5.i.i, align 4, !dbg !8936, !alias.scope !8938, !noalias !8920
  %_6.not.i1422 = icmp ugt i32 %_5.i1436, %_58.1.i.i
  br i1 %_6.not.i1422, label %bb4.i1426, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195, !dbg !8941, !prof !3544

bb4.i1426:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !7920
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !7921
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store i32 %storemerge.i1245.lcssa88369045.lcssa14889, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88639081.lcssa14910, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89489117.lcssa14931, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89759153.lcssa14952, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219214337.lcssa14974, ptr %705, align 4
  store float %_0.i3236.lcssa1220814355.lcssa14996, ptr %707, align 4
  store float %_0.i2858.lcssa1223214373.lcssa15018, ptr %713, align 4
  store float %_0.i3232.lcssa1223314391.lcssa15040, ptr %715, align 4
  store float %_0.i2862.lcssa1219214338, ptr %705, align 4
  store float %_0.i3236.lcssa1220814356, ptr %707, align 4
  store float %_0.i2858.lcssa1223214374, ptr %713, align 4
  store float %_0.i3232.lcssa1223314392, ptr %715, align 4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_214.i, i32 noundef %_5.i1436, i32 noundef range(i32 0, 536870912) %_58.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #28, !dbg !8946, !noalias !8947
  unreachable, !dbg !8946

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_lanefEB2_.exit7.i.i
  %_0.i2395 = fdiv float %_0.i2858, %_37.i.i, !dbg !8950
  %_0.i2857 = fsub float 1.000000e+00, %_0.i2395, !dbg !8952
  %_0.i2856 = fsub float %_0.i2857, %_0.i32329010, !dbg !8954
  %_4.i2411 = fmul float %_9.i.i, %_0.i2856, !dbg !8956
  %_0.i2412 = fadd float %_0.i32329010, %_4.i2411, !dbg !8956
  %_3.i.i3701.inv = fcmp ogt float %_0.i2857, %_0.i2412, !dbg !8958
  %_4.i.i3708.v = select i1 %_3.i.i3701.inv, float %_0.i2857, float %_0.i2412, !dbg !8958
  %760 = tail call noundef float @llvm.fabs.f32(float %_4.i.i3708.v), !dbg !8961
  %761 = fcmp uge float %760, 0x3BC79CA100000000, !dbg !8964
  %_0.i3232 = select i1 %761, float %_4.i.i3708.v, float 0.000000e+00, !dbg !8966
  %_0.i2855 = fsub float 1.000000e+00, %_0.i3232, !dbg !8967
  %_15.i1424 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i32 %_214.i, !dbg !8969
  %_0.i2870 = load float, ptr %_15.i1424, align 4, !dbg !8971, !alias.scope !8973, !noalias !8920, !noundef !10
  store float %_0.i3046, ptr %_15.i1424, align 4, !dbg !8976, !alias.scope !8979, !noalias !8920
  %_0.i2817 = fmul float %_0.i2855, %_0.i2870, !dbg !8982
  %_6.i3458 = bitcast float %_0.i2870 to i32, !dbg !8984
  %_5.i3459 = and i32 %_6.i3458, %all.sroa.0.0.i, !dbg !8987
  %_8.i3460 = bitcast float %_0.i2817 to i32, !dbg !8988
  %_7.i3462 = and i32 %_9.i3474, %_8.i3460, !dbg !8990
  %_4.i3463 = or disjoint i32 %_7.i3462, %_5.i3459, !dbg !8987
  store i32 %_4.i3463, ptr %data.i5.i.i.i.i.i4199, align 4, !dbg !8991, !alias.scope !8993, !noalias !8996
  %exitcond11977.not = icmp eq i32 %_206.0.i, %umin11976, !dbg !8478
  br i1 %exitcond11977.not, label %bb67.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit3048, !dbg !8478

bb67.i:                                           ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4181
  %_0.i3232.lcssa1223314391 = phi float [ %_0.i3232.lcssa1223314392, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4181 ], [ %_0.i3232, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %_0.i2858.lcssa1223214373 = phi float [ %_0.i2858.lcssa1223214374, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4181 ], [ %_0.i2858, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %_0.i3236.lcssa1220814355 = phi float [ %_0.i3236.lcssa1220814356, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4181 ], [ %_0.i3236, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %_0.i2862.lcssa1219214337 = phi float [ %_0.i2862.lcssa1219214338, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4181 ], [ %_0.i2862, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %running.sroa.0.0.i1210.lcssa89759153 = phi float [ %running.sroa.0.0.i1210.lcssa89759154, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4181 ], [ %running.sroa.0.0.i1210, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %storemerge.i1215.lcssa89489117 = phi i32 [ %storemerge.i1215.lcssa89489118, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4181 ], [ %storemerge.i1215, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %running.sroa.0.0.i1240.lcssa88639081 = phi float [ %running.sroa.0.0.i1240.lcssa88639082, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4181 ], [ %running.sroa.0.0.i1240, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %storemerge.i1245.lcssa88369045 = phi i32 [ %storemerge.i1245.lcssa88369046, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipIBN_IBN_INtNtNtBb_5slice4iter14ChunksExactMutfEB14_EINtB17_11ChunksExactfEEB1M_EINtB5_7ZipImplBW_B1M_E3newCsjLJhryqjeDL_17true_peak_limiter.exit4181 ], [ %storemerge.i1245, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3195 ]
  %_139.i = add i32 %run.sroa.0.0.i1836, %ring_cursor.sroa.0.1.i9039, !dbg !8997
  %_212.not.i = icmp ult i32 %_139.i, %ring.i, !dbg !8998
  %762 = select i1 %_212.not.i, i32 0, i32 %ring.i, !dbg !8998
  %ring_cursor.sroa.0.2.i = sub nuw i32 %_139.i, %762, !dbg !8998
  %_141.i = add i32 %run.sroa.0.0.i1836, %main_cursor.sroa.0.1.i9040, !dbg !9001
  %_223.not.i = icmp ult i32 %_141.i, %main.i, !dbg !9002
  %763 = select i1 %_223.not.i, i32 0, i32 %main.i, !dbg !9002
  %main_cursor.sroa.0.2.i = sub nuw i32 %_141.i, %763, !dbg !9002
  %_59.i = icmp ult i32 %_83.i, %spec.store.select.i, !dbg !8379
  br i1 %_59.i, label %bb20.i, label %bb15.i.loopexit, !dbg !8379

_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit: ; preds = %bb15.i.loopexit
  store float %history.i39.i.sroa.0.0.lcssa, ptr %hot_left.i, align 4, !dbg !7920
  store float %history.i39.i.sroa.7.0.lcssa, ptr %history.i39.i.sroa.7.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.10.0.lcssa, ptr %history.i39.i.sroa.10.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.13.0.lcssa, ptr %history.i39.i.sroa.13.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.16.0.lcssa, ptr %history.i39.i.sroa.16.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.19.0.lcssa, ptr %history.i39.i.sroa.19.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.22.0.lcssa, ptr %history.i39.i.sroa.22.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.26.0.lcssa, ptr %history.i39.i.sroa.26.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.29.0.lcssa, ptr %history.i39.i.sroa.29.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.32.0.lcssa, ptr %history.i39.i.sroa.32.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.35.0.lcssa, ptr %history.i39.i.sroa.35.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i39.i.sroa.38.0.lcssa, ptr %history.i39.i.sroa.38.0.hot_left.i.sroa_idx, align 4, !dbg !7920
  store float %history.i.i.sroa.0.0.lcssa, ptr %hot_right.i, align 4, !dbg !7921
  store float %history.i.i.sroa.7.0.lcssa, ptr %history.i.i.sroa.7.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.10.0.lcssa, ptr %history.i.i.sroa.10.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.13.0.lcssa, ptr %history.i.i.sroa.13.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.16.0.lcssa, ptr %history.i.i.sroa.16.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.19.0.lcssa, ptr %history.i.i.sroa.19.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.22.0.lcssa, ptr %history.i.i.sroa.22.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.26.0.lcssa, ptr %history.i.i.sroa.26.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.29.0.lcssa, ptr %history.i.i.sroa.29.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.32.0.lcssa, ptr %history.i.i.sroa.32.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.35.0.lcssa, ptr %history.i.i.sroa.35.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store float %history.i.i.sroa.38.0.lcssa, ptr %history.i.i.sroa.38.0.hot_right.i.sroa_idx, align 4, !dbg !7921
  store i32 %storemerge.i1245.lcssa88369045.lcssa14888, ptr %_22.i271.i, align 4
  store float %running.sroa.0.0.i1240.lcssa88639081.lcssa14909, ptr %_21.i270.i, align 4
  store i32 %storemerge.i1215.lcssa89489117.lcssa14930, ptr %_22.i.i, align 4
  store float %running.sroa.0.0.i1210.lcssa89759153.lcssa14951, ptr %_21.i.i, align 4
  store float %_0.i2862.lcssa1219214337.lcssa14973, ptr %705, align 4
  store float %_0.i3236.lcssa1220814355.lcssa14995, ptr %707, align 4
  store float %_0.i2858.lcssa1223214373.lcssa15017, ptr %713, align 4
  store float %_0.i3232.lcssa1223314391.lcssa15039, ptr %715, align 4
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit, !dbg !9004

_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit, %bb6.i
  %ring_cursor.sroa.0.0.i.lcssa = phi i32 [ %_37.i, %bb6.i ], [ %ring_cursor.sroa.0.1.i.lcssa, %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit ], !dbg !7853
  %main_cursor.sroa.0.0.i.lcssa = phi i32 [ %_36.i, %bb6.i ], [ %main_cursor.sroa.0.1.i.lcssa, %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit.loopexit ], !dbg !7850
  %764 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 24, !dbg !9004
  %left_prefix.i = load float, ptr %764, align 4, !dbg !9004, !noalias !7857, !noundef !10
  %765 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 28, !dbg !9005
  %left_phase.i = load i32, ptr %765, align 4, !dbg !9005, !noalias !7857, !noundef !10
  %766 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 24, !dbg !9006
  %right_prefix.i = load float, ptr %766, align 4, !dbg !9006, !noalias !7857, !noundef !10
  %767 = getelementptr inbounds nuw i8, ptr %uniform_right.i, i32 28, !dbg !9007
  %right_phase.i = load i32, ptr %767, align 4, !dbg !9007, !noalias !7857, !noundef !10
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_right.i), !dbg !9008, !noalias !7857
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i), !dbg !9009, !noalias !7857
  %768 = getelementptr inbounds nuw i8, ptr %self, i32 364, !dbg !9010
  %_236.0.i = load ptr, ptr %768, align 4, !dbg !9010, !alias.scope !7824, !noalias !9012, !nonnull !10, !noundef !10
  %769 = getelementptr inbounds nuw i8, ptr %self, i32 368, !dbg !9010
  %_236.1.i = load i32, ptr %769, align 4, !dbg !9010, !alias.scope !7824, !noalias !9012, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9013), !dbg !9016
  %_4.not.i3180 = icmp eq i32 %_236.1.i, 0, !dbg !9017
  br i1 %_4.not.i3180, label %panic.i3182, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3183, !dbg !9017

panic.i3182:                                      ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #28, !dbg !9017, !noalias !9019
  unreachable, !dbg !9017

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3183: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter21limiter_block_uniformKh1_fEB2_.exit
  store float %left_prefix.i, ptr %_236.0.i, align 4, !dbg !9017, !alias.scope !9013, !noalias !7828
  %_237.0.i = load ptr, ptr %68, align 4, !dbg !9020, !alias.scope !7824, !noalias !9012, !nonnull !10, !noundef !10
  %_237.1.i = load i32, ptr %69, align 4, !dbg !9020, !alias.scope !7824, !noalias !9012, !noundef !10
  %770 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i), !dbg !9021
  br i1 %770, label %bb2.i4226, label %bb6.i4218, !dbg !9021

bb6.i4218:                                        ; preds = %bb2.i4226, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3183
  %end_or_len.idx.i4219 = shl nuw nsw i32 %_237.1.i, 2, !dbg !9025
  %end_or_len.i4220 = getelementptr inbounds nuw i8, ptr %_237.0.i, i32 %end_or_len.idx.i4219, !dbg !9025
  %_293.i4221 = icmp eq i32 %_237.1.i, 0, !dbg !9029
  br i1 %_293.i4221, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4232, label %bb10.i4222, !dbg !9032

bb2.i4226:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3183
  %bytes1.sroa.0.0.zext.i4227 = and i32 %left_phase.i, 255, !dbg !9033
  %bytes1.sroa.0.0.isplat.i4228 = mul nuw i32 %bytes1.sroa.0.0.zext.i4227, 16843009, !dbg !9033
  %_5.i4229 = icmp eq i32 %left_phase.i, %bytes1.sroa.0.0.isplat.i4228, !dbg !9034
  br i1 %_5.i4229, label %bb3.i4230, label %bb6.i4218, !dbg !9034

bb3.i4230:                                        ; preds = %bb2.i4226
  %bytes.sroa.0.0.extract.trunc.i4231 = trunc i32 %left_phase.i to i8, !dbg !9035
  %771 = shl nuw nsw i32 %_237.1.i, 2, !dbg !9037
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %_237.0.i, i8 %bytes.sroa.0.0.extract.trunc.i4231, i32 %771, i1 false), !dbg !9037, !alias.scope !9038, !noalias !7828
  br label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4232, !dbg !9041

bb10.i4222:                                       ; preds = %bb6.i4218, %bb10.i4222
  %iter.sroa.0.04.i4223 = phi ptr [ %_38.i4224, %bb10.i4222 ], [ %_237.0.i, %bb6.i4218 ]
  %_38.i4224 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i4223, i32 4, !dbg !9042
  store i32 %left_phase.i, ptr %iter.sroa.0.04.i4223, align 4, !dbg !9044, !alias.scope !9038, !noalias !7828
  %_29.i4225 = icmp eq ptr %_38.i4224, %end_or_len.i4220, !dbg !9029
  br i1 %_29.i4225, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4232, label %bb10.i4222, !dbg !9032

_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4232: ; preds = %bb10.i4222, %bb6.i4218, %bb3.i4230
  %772 = getelementptr inbounds nuw i8, ptr %self, i32 464, !dbg !9045
  %_238.0.i = load ptr, ptr %772, align 4, !dbg !9045, !alias.scope !7826, !noalias !9046, !nonnull !10, !noundef !10
  %773 = getelementptr inbounds nuw i8, ptr %self, i32 468, !dbg !9045
  %_238.1.i = load i32, ptr %773, align 4, !dbg !9045, !alias.scope !7826, !noalias !9046, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9047), !dbg !9050
  %_4.not.i3176 = icmp eq i32 %_238.1.i, 0, !dbg !9051
  br i1 %_4.not.i3176, label %panic.i3178, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3179, !dbg !9051

panic.i3178:                                      ; preds = %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4232
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef 0, i32 noundef 0, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_42467a01aa002dcbd00fd8f2432248c2) #28, !dbg !9051, !noalias !9053
  unreachable, !dbg !9051

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3179: ; preds = %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4232
  store float %right_prefix.i, ptr %_238.0.i, align 4, !dbg !9051, !alias.scope !9047, !noalias !7828
  %_239.0.i = load ptr, ptr %77, align 4, !dbg !9054, !alias.scope !7826, !noalias !9046, !nonnull !10, !noundef !10
  %_239.1.i = load i32, ptr %78, align 4, !dbg !9054, !alias.scope !7826, !noalias !9046, !noundef !10
  %774 = tail call i1 @llvm.is.constant.i32(i32 %right_phase.i), !dbg !9055
  br i1 %774, label %bb2.i4241, label %bb6.i4233, !dbg !9055

bb6.i4233:                                        ; preds = %bb2.i4241, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3179
  %end_or_len.idx.i4234 = shl nuw nsw i32 %_239.1.i, 2, !dbg !9058
  %end_or_len.i4235 = getelementptr inbounds nuw i8, ptr %_239.0.i, i32 %end_or_len.idx.i4234, !dbg !9058
  %_293.i4236 = icmp eq i32 %_239.1.i, 0, !dbg !9062
  br i1 %_293.i4236, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4247, label %bb10.i4237, !dbg !9065

bb2.i4241:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane5store.exit3179
  %bytes1.sroa.0.0.zext.i4242 = and i32 %right_phase.i, 255, !dbg !9066
  %bytes1.sroa.0.0.isplat.i4243 = mul nuw i32 %bytes1.sroa.0.0.zext.i4242, 16843009, !dbg !9066
  %_5.i4244 = icmp eq i32 %right_phase.i, %bytes1.sroa.0.0.isplat.i4243, !dbg !9067
  br i1 %_5.i4244, label %bb3.i4245, label %bb6.i4233, !dbg !9067

bb3.i4245:                                        ; preds = %bb2.i4241
  %bytes.sroa.0.0.extract.trunc.i4246 = trunc i32 %right_phase.i to i8, !dbg !9068
  %775 = shl nuw nsw i32 %_239.1.i, 2, !dbg !9070
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %_239.0.i, i8 %bytes.sroa.0.0.extract.trunc.i4246, i32 %775, i1 false), !dbg !9070, !alias.scope !9071, !noalias !7828
  br label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4247, !dbg !9074

bb10.i4237:                                       ; preds = %bb6.i4233, %bb10.i4237
  %iter.sroa.0.04.i4238 = phi ptr [ %_38.i4239, %bb10.i4237 ], [ %_239.0.i, %bb6.i4233 ]
  %_38.i4239 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i4238, i32 4, !dbg !9075
  store i32 %right_phase.i, ptr %iter.sroa.0.04.i4238, align 4, !dbg !9077, !alias.scope !9071, !noalias !7828
  %_29.i4240 = icmp eq ptr %_38.i4239, %end_or_len.i4235, !dbg !9062
  br i1 %_29.i4240, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4247, label %bb10.i4237, !dbg !9065

_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4247: ; preds = %bb10.i4237, %bb6.i4233, %bb3.i4245
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %hot_left.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_33) #27, !dbg !9078
; call <true_peak_limiter::HotChannel<f32>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr noalias noundef readonly align 4 captures(none) dereferenceable(92) %hot_right.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_34) #27, !dbg !9079
  store i32 %main_cursor.sroa.0.0.i.lcssa, ptr %_35, align 4, !dbg !9080, !alias.scope !7828, !noalias !7852
  store i32 %ring_cursor.sroa.0.0.i.lcssa, ptr %662, align 4, !dbg !9081, !alias.scope !7828, !noalias !7852
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_right.i), !dbg !9082, !noalias !7857
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i), !dbg !9083, !noalias !7857
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockfEB2_.exit, !dbg !7821

_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockfEB2_.exit: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh2_fEB2_.exit, %_RINvCsjLJhryqjeDL_17true_peak_limiter22limiter_block_per_laneKh1_fEB2_.exit, %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4116, %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4247
  br i1 %quiet.sroa.0.0.off04436, label %bb28, label %bb40, !dbg !9084

bb22:                                             ; preds = %bb20
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9085), !dbg !9088
  %776 = getelementptr inbounds nuw i8, ptr %self, i32 380, !dbg !9089
  %_40.0.i = load ptr, ptr %776, align 4, !dbg !9089, !alias.scope !9085, !nonnull !10, !noundef !10
  %777 = getelementptr inbounds nuw i8, ptr %self, i32 384, !dbg !9089
  %_40.1.i = load i32, ptr %777, align 4, !dbg !9089, !alias.scope !9085, !noundef !10
  %778 = getelementptr inbounds nuw i8, ptr %self, i32 412, !dbg !9091
  %_41.0.i = load ptr, ptr %778, align 4, !dbg !9091, !alias.scope !9085, !nonnull !10, !noundef !10
  %779 = getelementptr inbounds nuw i8, ptr %self, i32 416, !dbg !9091
  %_41.1.i = load i32, ptr %779, align 4, !dbg !9091, !alias.scope !9085, !noundef !10
  %spec.store.select.i.i = tail call i32 @llvm.umin.i32(i32 %_41.1.i, i32 %_40.1.i), !dbg !9092
  %_2.i6.not.i = icmp eq i32 %spec.store.select.i.i, 0, !dbg !9098
  br i1 %_2.i6.not.i, label %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb3.i4248, !dbg !9098

bb3.i4248:                                        ; preds = %bb22, %bb5.i4250
  %iter.sroa.8.07.i = phi i32 [ %780, %bb5.i4250 ], [ 0, %bb22 ]
  %_3.i1.i.i = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i, i32 %iter.sroa.8.07.i, !dbg !9101
  %_14.i4249 = load i32, ptr %_3.i1.i.i, align 4, !dbg !9104, !noalias !9085, !noundef !10
  %_20.i = icmp eq i32 %_14.i4249, 0, !dbg !9105
  br i1 %_20.i, label %panic.i4257, label %bb5.i4250, !dbg !9105

bb5.i4250:                                        ; preds = %bb3.i4248
  %_3.i.i.i4251 = getelementptr inbounds nuw i32, ptr %_40.0.i, i32 %iter.sroa.8.07.i, !dbg !9106
  %780 = add nuw i32 %iter.sroa.8.07.i, 1, !dbg !9109
  %_18.i4252 = load i32, ptr %_3.i.i.i4251, align 4, !dbg !9110, !noalias !9085, !noundef !10
  %_19.i4253 = urem i32 %frames, %_14.i4249, !dbg !9105
  %_16.i4254 = add i32 %_19.i4253, %_18.i4252, !dbg !9111
  %_15.i4255 = urem i32 %_16.i4254, %_14.i4249, !dbg !9112
  store i32 %_15.i4255, ptr %_3.i.i.i4251, align 4, !dbg !9113, !noalias !9085
  %exitcond.not.i = icmp eq i32 %780, %spec.store.select.i.i, !dbg !9098
  br i1 %exitcond.not.i, label %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb3.i4248, !dbg !9098

panic.i4257:                                      ; preds = %bb3.i4248
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_f5b0427df9b659e554a697ca46ce8b5a) #28, !dbg !9105, !noalias !9085
  unreachable, !dbg !9105

_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit: ; preds = %bb5.i4250, %bb22
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9114), !dbg !9117
  %781 = getelementptr inbounds nuw i8, ptr %self, i32 480, !dbg !9118
  %_40.0.i4258 = load ptr, ptr %781, align 4, !dbg !9118, !alias.scope !9114, !nonnull !10, !noundef !10
  %782 = getelementptr inbounds nuw i8, ptr %self, i32 484, !dbg !9118
  %_40.1.i4259 = load i32, ptr %782, align 4, !dbg !9118, !alias.scope !9114, !noundef !10
  %783 = getelementptr inbounds nuw i8, ptr %self, i32 512, !dbg !9120
  %_41.0.i4260 = load ptr, ptr %783, align 4, !dbg !9120, !alias.scope !9114, !nonnull !10, !noundef !10
  %784 = getelementptr inbounds nuw i8, ptr %self, i32 516, !dbg !9120
  %_41.1.i4261 = load i32, ptr %784, align 4, !dbg !9120, !alias.scope !9114, !noundef !10
  %spec.store.select.i.i4262 = tail call i32 @llvm.umin.i32(i32 %_41.1.i4261, i32 %_40.1.i4259), !dbg !9121
  %_2.i6.not.i4263 = icmp eq i32 %spec.store.select.i.i4262, 0, !dbg !9127
  br i1 %_2.i6.not.i4263, label %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit4278, label %bb3.i4264, !dbg !9127

bb3.i4264:                                        ; preds = %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, %bb5.i4269
  %iter.sroa.8.07.i4265 = phi i32 [ %785, %bb5.i4269 ], [ 0, %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit ]
  %_3.i1.i.i4266 = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i4260, i32 %iter.sroa.8.07.i4265, !dbg !9130
  %_14.i4267 = load i32, ptr %_3.i1.i.i4266, align 4, !dbg !9133, !noalias !9114, !noundef !10
  %_20.i4268 = icmp eq i32 %_14.i4267, 0, !dbg !9134
  br i1 %_20.i4268, label %panic.i4277, label %bb5.i4269, !dbg !9134

bb5.i4269:                                        ; preds = %bb3.i4264
  %_3.i.i.i4270 = getelementptr inbounds nuw i32, ptr %_40.0.i4258, i32 %iter.sroa.8.07.i4265, !dbg !9135
  %785 = add nuw i32 %iter.sroa.8.07.i4265, 1, !dbg !9138
  %_18.i4271 = load i32, ptr %_3.i.i.i4270, align 4, !dbg !9139, !noalias !9114, !noundef !10
  %_19.i4272 = urem i32 %frames, %_14.i4267, !dbg !9134
  %_16.i4273 = add i32 %_19.i4272, %_18.i4271, !dbg !9140
  %_15.i4274 = urem i32 %_16.i4273, %_14.i4267, !dbg !9141
  store i32 %_15.i4274, ptr %_3.i.i.i4270, align 4, !dbg !9142, !noalias !9114
  %exitcond.not.i4275 = icmp eq i32 %785, %spec.store.select.i.i4262, !dbg !9127
  br i1 %exitcond.not.i4275, label %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit4278, label %bb3.i4264, !dbg !9127

panic.i4277:                                      ; preds = %bb3.i4264
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_f5b0427df9b659e554a697ca46ce8b5a) #28, !dbg !9134, !noalias !9114
  unreachable, !dbg !9134

_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit4278: ; preds = %bb5.i4269, %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit
  %786 = getelementptr inbounds nuw i8, ptr %self, i32 528, !dbg !9143
  %_29.val = load i32, ptr %786, align 4, !dbg !9143
  %787 = getelementptr inbounds nuw i8, ptr %self, i32 532, !dbg !9143
  %_29.val3843 = load i32, ptr %787, align 4, !dbg !9143, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9144), !dbg !9143
  %_10.i4279 = icmp eq i32 %_29.val3843, 0, !dbg !9147
  br i1 %_10.i4279, label %panic.i4289, label %bb1.i4280, !dbg !9147

bb1.i4280:                                        ; preds = %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit4278
  %_28 = getelementptr inbounds nuw i8, ptr %self, i32 104, !dbg !9151
  %_7.i = load i32, ptr %_28, align 4, !dbg !9152, !alias.scope !9144, !noundef !10
  %_8.i4281 = urem i32 %frames, %_29.val3843, !dbg !9147
  %_5.i4282 = add i32 %_8.i4281, %_7.i, !dbg !9153
  %_4.i4283 = urem i32 %_5.i4282, %_29.val3843, !dbg !9154
  store i32 %_4.i4283, ptr %_28, align 4, !dbg !9155, !alias.scope !9144
  %_17.i4284 = icmp eq i32 %_29.val, 0, !dbg !9156
  br i1 %_17.i4284, label %panic2.i, label %_RNvMs1_CsjLJhryqjeDL_17true_peak_limiterNtB5_7Cursors7advance.exit, !dbg !9156

panic.i4289:                                      ; preds = %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit4278
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c7e64e9e8489bc8e648659d0098d136c) #28, !dbg !9147, !noalias !9144
  unreachable, !dbg !9147

panic2.i:                                         ; preds = %bb1.i4280
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_34b23597fad9d85cb3bee6d64ebf77d5) #28, !dbg !9156, !noalias !9144
  unreachable, !dbg !9156

_RNvMs1_CsjLJhryqjeDL_17true_peak_limiterNtB5_7Cursors7advance.exit: ; preds = %bb1.i4280
  %788 = getelementptr inbounds nuw i8, ptr %self, i32 108, !dbg !9157
  %_14.i4286 = load i32, ptr %788, align 4, !dbg !9157, !alias.scope !9144, !noundef !10
  %_15.i4287 = urem i32 %frames, %_29.val, !dbg !9156
  %_12.i = add i32 %_15.i4287, %_14.i4286, !dbg !9158
  %_11.i4288 = urem i32 %_12.i, %_29.val, !dbg !9159
  store i32 %_11.i4288, ptr %788, align 4, !dbg !9160, !alias.scope !9144
  br label %bb42, !dbg !9161

bb28:                                             ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockfEB2_.exit
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_37 = tail call noundef zeroext i1 @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_33) #27, !dbg !9162
  br i1 %_37, label %bb30, label %bb40, !dbg !9163

bb30:                                             ; preds = %bb28
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_39 = tail call noundef zeroext i1 @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_34) #27, !dbg !9164
  br i1 %_39, label %bb32, label %bb40, !dbg !9165

bb32:                                             ; preds = %bb30
  %_80.not = icmp ugt i32 %frames, %left_io.1
  br i1 %_80.not, label %bb51, label %bb1.i4290, !dbg !9166, !prof !3544

bb51:                                             ; preds = %bb32
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef %frames, i32 noundef %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_8457596f73b1d71f2ddde42261d136f3) #28, !dbg !9174
  unreachable, !dbg !9174

bb1.i4290:                                        ; preds = %bb32, %bb12.i4304
  %io.sroa.5.0.i4291 = phi i32 [ %len.i.i.i4297, %bb12.i4304 ], [ %frames, %bb32 ]
  %io.sroa.0.0.i4292 = phi ptr [ %data.i.i.i4296, %bb12.i4304 ], [ %left_io.0, %bb32 ]
  %789 = icmp eq i32 %io.sroa.5.0.i4291, 0, !dbg !9175
  br i1 %789, label %bb34, label %bb13.preheader.i4293, !dbg !9175

bb13.preheader.i4293:                             ; preds = %bb1.i4290
  %spec.store.select.i4294 = tail call i32 @llvm.umin.i32(i32 %io.sroa.5.0.i4291, i32 32), !dbg !9178
  %data.i.i.idx.i4295 = shl nuw nsw i32 %spec.store.select.i4294, 2, !dbg !9181
  %data.i.i.i4296 = getelementptr inbounds nuw i8, ptr %io.sroa.0.0.i4292, i32 %data.i.i.idx.i4295, !dbg !9181
  br label %bb13.i4298, !dbg !9186

bb13.i4298:                                       ; preds = %bb13.i4298, %bb13.preheader.i4293
  %iter.sroa.0.08.i4299 = phi ptr [ %_35.i4301, %bb13.i4298 ], [ %io.sroa.0.0.i4292, %bb13.preheader.i4293 ]
  %bits.sroa.0.07.i4300 = phi i32 [ %790, %bb13.i4298 ], [ 0, %bb13.preheader.i4293 ]
  %_35.i4301 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i4299, i32 4, !dbg !9188
  %_95.i4302 = load i32, ptr %iter.sroa.0.08.i4299, align 4, !dbg !9190, !alias.scope !9191, !noundef !10
  %790 = or i32 %_95.i4302, %bits.sroa.0.07.i4300, !dbg !9194
  %_29.i4303 = icmp eq ptr %_35.i4301, %data.i.i.i4296, !dbg !9195
  br i1 %_29.i4303, label %bb12.i4304, label %bb13.i4298, !dbg !9186

bb12.i4304:                                       ; preds = %bb13.i4298
  %len.i.i.i4297 = sub nuw nsw i32 %io.sroa.5.0.i4291, %spec.store.select.i4294, !dbg !9197
  %791 = icmp eq i32 %790, 0, !dbg !9198
  br i1 %791, label %bb1.i4290, label %bb40, !dbg !9198

bb34:                                             ; preds = %bb1.i4290
  %_88.not = icmp ugt i32 %frames, %right_io.1, !dbg !9199
  br i1 %_88.not, label %bb54, label %bb1.i4320, !dbg !9199, !prof !755

bb40:                                             ; preds = %bb12.i4304, %bb12.i4334, %bb1.i4320, %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockfEB2_.exit, %bb28, %bb30
  %_36.sroa.0.0.off0 = phi i1 [ false, %bb12.i4334 ], [ false, %_RINvCsjLJhryqjeDL_17true_peak_limiter13limiter_blockfEB2_.exit ], [ false, %bb30 ], [ false, %bb28 ], [ true, %bb1.i4320 ], [ false, %bb12.i4304 ]
  %792 = zext i1 %_36.sroa.0.0.off0 to i8, !dbg !9205
  store i8 %792, ptr %38, align 8, !dbg !9205
  %793 = load i8, ptr %2, align 8, !dbg !9206, !range !3570, !noundef !10
  store i8 %793, ptr %0, align 1, !dbg !9207
  call void @llvm.lifetime.start.p0(ptr nonnull %shape), !dbg !9208
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(12) %shape, ptr noundef nonnull align 4 dereferenceable(12) %_32, i32 12, i1 false), !dbg !9209
  %794 = getelementptr inbounds nuw i8, ptr %self, i32 56, !dbg !9210
  %795 = load i32, ptr %794, align 8, !dbg !9210, !noundef !10
  %_53 = getelementptr inbounds nuw i8, ptr %self, i32 112, !dbg !9212
  %796 = getelementptr inbounds nuw i8, ptr %self, i32 88, !dbg !9219
  %_99.0 = load ptr, ptr %796, align 8, !dbg !9219, !nonnull !10, !noundef !10
  %797 = getelementptr inbounds nuw i8, ptr %self, i32 92, !dbg !9219
  %_99.1 = load i32, ptr %797, align 4, !dbg !9219, !noundef !10
  %798 = getelementptr inbounds nuw i8, ptr %self, i32 96, !dbg !9219
  %_100.0 = load ptr, ptr %798, align 8, !dbg !9219, !nonnull !10, !noundef !10
  %799 = getelementptr inbounds nuw i8, ptr %self, i32 100, !dbg !9219
  %_100.1 = load i32, ptr %799, align 4, !dbg !9219, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9220), !dbg !9223
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9224), !dbg !9223
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9226), !dbg !9223
  %_22.not.i1463.i = icmp eq i32 %left_io.1, 0, !dbg !9228
  br i1 %_22.not.i1463.i, label %bb6.i.preheader.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i, !dbg !9228

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i: ; preds = %bb40, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i
  %ok.sroa.0.0.i1366.i = phi i32 [ %_0.i33.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i ], [ -1, %bb40 ]
  %iter.sroa.0.0.i1265.i = phi ptr [ %_27.i16.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i ], [ %left_io.0, %bb40 ]
  %iter.sroa.5.0.i1164.i = phi i32 [ %_28.i17.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i ], [ %left_io.1, %bb40 ]
  %_27.i16.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i1265.i, i32 4, !dbg !9241
  %_28.i17.i = add nsw i32 %iter.sroa.5.0.i1164.i, -1, !dbg !9248
  %_0.i28.i = load float, ptr %iter.sroa.0.0.i1265.i, align 4, !dbg !9249, !alias.scope !9252, !noalias !9255, !noundef !10
  %800 = tail call noundef float @llvm.fabs.f32(float %_0.i28.i), !dbg !9257
  %_3.i.i4307 = fcmp olt float %800, 0x46293E5940000000, !dbg !9260
  %_0.i33.i = select i1 %_3.i.i4307, i32 %ok.sroa.0.0.i1366.i, i32 0, !dbg !9263
  %_22.not.i14.i = icmp eq i32 %_28.i17.i, 0, !dbg !9228
  br i1 %_22.not.i14.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockfECsjLJhryqjeDL_17true_peak_limiter.exit25.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i, !dbg !9228

_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockfECsjLJhryqjeDL_17true_peak_limiter.exit25.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i
  %801 = icmp eq i32 %_0.i33.i, -1, !dbg !9266
  br i1 %801, label %bb6.i.preheader.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i, !dbg !9269

bb6.i.preheader.i:                                ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockfECsjLJhryqjeDL_17true_peak_limiter.exit25.i, %bb40
  %_22.not.i67.i = icmp eq i32 %right_io.1, 0, !dbg !9270
  br i1 %_22.not.i67.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockfNCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit32.i, !dbg !9270

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit32.i: ; preds = %bb6.i.preheader.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit32.i
  %ok.sroa.0.0.i70.i = phi i32 [ %_0.i34.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit32.i ], [ -1, %bb6.i.preheader.i ]
  %iter.sroa.0.0.i69.i = phi ptr [ %_27.i.i4318, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit32.i ], [ %right_io.0, %bb6.i.preheader.i ]
  %iter.sroa.5.0.i68.i = phi i32 [ %_28.i.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit32.i ], [ %right_io.1, %bb6.i.preheader.i ]
  %_27.i.i4318 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i69.i, i32 4, !dbg !9274
  %_28.i.i = add nsw i32 %iter.sroa.5.0.i68.i, -1, !dbg !9277
  %_0.i30.i = load float, ptr %iter.sroa.0.0.i69.i, align 4, !dbg !9278, !alias.scope !9280, !noalias !9283, !noundef !10
  %802 = tail call noundef float @llvm.fabs.f32(float %_0.i30.i), !dbg !9284
  %_3.i26.i = fcmp olt float %802, 0x46293E5940000000, !dbg !9286
  %_0.i34.i = select i1 %_3.i26.i, i32 %ok.sroa.0.0.i70.i, i32 0, !dbg !9288
  %_22.not.i.i = icmp eq i32 %_28.i.i, 0, !dbg !9270
  br i1 %_22.not.i.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockfECsjLJhryqjeDL_17true_peak_limiter.exit.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit32.i, !dbg !9270

_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockfECsjLJhryqjeDL_17true_peak_limiter.exit.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit32.i
  %803 = icmp eq i32 %_0.i34.i, -1, !dbg !9290
  br i1 %803, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockfNCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit, label %bb7.i4319, !dbg !9292

bb7.i4319:                                        ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockfECsjLJhryqjeDL_17true_peak_limiter.exit.i
  br i1 %_22.not.i1463.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i, !dbg !9293

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i: ; preds = %bb7.i4319, %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockfECsjLJhryqjeDL_17true_peak_limiter.exit25.i
  br label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.i, !dbg !9293

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i
  %ok.sroa.0.012.i.i = phi i32 [ %_0.i8.i.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ -1, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i ]
  %iter.sroa.0.011.i.i = phi ptr [ %_42.i.i4308, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ %left_io.0, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i ]
  %iter.sroa.5.010.i.i = phi i32 [ %_43.i.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ %left_io.1, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.preheader.i ]
  %_42.i.i4308 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.011.i.i, i32 4, !dbg !9304
  %_43.i.i = add nsw i32 %iter.sroa.5.010.i.i, -1, !dbg !9311
  %_0.i.i.i4309 = load float, ptr %iter.sroa.0.011.i.i, align 4, !dbg !9312, !alias.scope !9315, !noalias !9255, !noundef !10
  %804 = tail call noundef float @llvm.fabs.f32(float %_0.i.i.i4309), !dbg !9320
  %_3.i.i.i4310 = fcmp olt float %804, 0x46293E5940000000, !dbg !9323
  %_0.i8.i.i = select i1 %_3.i.i.i4310, i32 %ok.sroa.0.012.i.i, i32 0, !dbg !9325
  %_37.not.i.i = icmp eq i32 %_43.i.i, 0, !dbg !9293
  br i1 %_37.not.i.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.i, !dbg !9293

_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i.i
  %805 = and i32 %_0.i8.i.i, 1065353216, !dbg !9327
  %806 = icmp ne i32 %805, 1065353216, !dbg !9330
  %807 = zext i1 %806 to i32, !dbg !9330
  %_37.not9.i40.i = icmp eq i32 %right_io.1, 0, !dbg !9334
  br i1 %_37.not9.i40.i, label %bb12.i.preheader.thread.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i, !dbg !9334

bb12.i.preheader.thread.i:                        ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit.i
  %808 = getelementptr inbounds nuw i8, ptr %self, i32 120, !dbg !9338
  store i32 %807, ptr %808, align 8, !dbg !9338, !alias.scope !9226, !noalias !9339
  %_1481.i = load i64, ptr %_53, align 8, !dbg !9340, !alias.scope !9226, !noalias !9339, !noundef !10
  %809 = tail call i64 @llvm.uadd.sat.i64(i64 %_1481.i, i64 1), !dbg !9341
  store i64 %809, ptr %_53, align 8, !dbg !9344, !alias.scope !9226, !noalias !9339
  br label %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit60.i, !dbg !9345

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i: ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit.i, %bb7.i4319
  %ok.sroa.0.0.lcssa.i76.i = phi i32 [ %807, %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit.i ], [ 0, %bb7.i4319 ]
  br label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.i, !dbg !9334

_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i
  %ok.sroa.0.012.i42.i = phi i32 [ %_0.i8.i49.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.i ], [ -1, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i ]
  %iter.sroa.0.011.i43.i = phi ptr [ %_42.i45.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.i ], [ %right_io.0, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i ]
  %iter.sroa.5.010.i44.i = phi i32 [ %_43.i46.i, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.i ], [ %right_io.1, %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.preheader.i ]
  %_42.i45.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.011.i43.i, i32 4, !dbg !9350
  %_43.i46.i = add nsw i32 %iter.sroa.5.010.i44.i, -1, !dbg !9353
  %_0.i.i47.i = load float, ptr %iter.sroa.0.011.i43.i, align 4, !dbg !9354, !alias.scope !9356, !noalias !9283, !noundef !10
  %810 = tail call noundef float @llvm.fabs.f32(float %_0.i.i47.i), !dbg !9361
  %_3.i.i48.i = fcmp olt float %810, 0x46293E5940000000, !dbg !9363
  %_0.i8.i49.i = select i1 %_3.i.i48.i, i32 %ok.sroa.0.012.i42.i, i32 0, !dbg !9365
  %_37.not.i50.i = icmp eq i32 %_43.i46.i, 0, !dbg !9334
  br i1 %_37.not.i50.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit53.i, label %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.i, !dbg !9334

_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit53.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane6scalarfNtB4_4Lane4load.exit.i41.i
  %811 = and i32 %_0.i8.i49.i, 1065353216, !dbg !9367
  %812 = icmp ne i32 %811, 1065353216, !dbg !9369
  %813 = zext i1 %812 to i32, !dbg !9369
  %814 = or i32 %ok.sroa.0.0.lcssa.i76.i, %813, !dbg !9338
  %815 = getelementptr inbounds nuw i8, ptr %self, i32 120, !dbg !9338
  store i32 %814, ptr %815, align 8, !dbg !9338, !alias.scope !9226, !noalias !9339
  %_14.i4311 = load i64, ptr %_53, align 8, !dbg !9340, !alias.scope !9226, !noalias !9339, !noundef !10
  %816 = tail call i64 @llvm.uadd.sat.i64(i64 %_14.i4311, i64 1), !dbg !9341
  store i64 %816, ptr %_53, align 8, !dbg !9344, !alias.scope !9226, !noalias !9339
  br i1 %_22.not.i1463.i, label %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit60.i, label %bb12.i.preheader.i, !dbg !9370

bb12.i.preheader.i:                               ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit53.i
  %.idx.i.i = shl nuw nsw i32 %left_io.1, 2, !dbg !9374
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %left_io.0, i8 0, i32 %.idx.i.i, i1 false), !dbg !9378, !alias.scope !9379, !noalias !9255
  br label %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit60.i, !dbg !9345

_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit60.i: ; preds = %bb12.i.preheader.i, %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit53.i, %bb12.i.preheader.thread.i
  %left.1.sink.i = phi i32 [ %left_io.1, %bb12.i.preheader.thread.i ], [ %right_io.1, %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit53.i ], [ %right_io.1, %bb12.i.preheader.i ]
  %left.0.sink.i = phi ptr [ %left_io.0, %bb12.i.preheader.thread.i ], [ %right_io.0, %_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskfECsjLJhryqjeDL_17true_peak_limiter.exit53.i ], [ %right_io.0, %bb12.i.preheader.i ]
  %.idx.i85.i = shl nuw nsw i32 %left.1.sink.i, 2, !dbg !9382
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %left.0.sink.i, i8 0, i32 %.idx.i85.i, i1 false), !dbg !9388, !alias.scope !9389, !noalias !9390
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 4 dereferenceable(100) %_33, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(12) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_99.0, i32 noundef %_99.1, i32 noundef %795) #27, !dbg !9391, !noalias !9396
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 4 dereferenceable(100) %_34, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(12) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_100.0, i32 noundef %_100.1, i32 noundef %795) #27, !dbg !9399, !noalias !9396
  store i32 0, ptr %_35, align 4, !dbg !9400, !noalias !9396
  %817 = getelementptr inbounds nuw i8, ptr %self, i32 108, !dbg !9400
  store i32 0, ptr %817, align 4, !dbg !9400, !noalias !9396
  br label %_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockfNCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit, !dbg !9401

_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockfNCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit: ; preds = %bb6.i.preheader.i, %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockfECsjLJhryqjeDL_17true_peak_limiter.exit.i, %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit60.i
  call void @llvm.lifetime.end.p0(ptr nonnull %shape), !dbg !9402
  br label %bb42, !dbg !9161

bb54:                                             ; preds = %bb34
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef %frames, i32 noundef %right_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_8b6b4fff29b57e2077f91d7df0eea7e7) #28, !dbg !9403
  unreachable, !dbg !9403

bb1.i4320:                                        ; preds = %bb34, %bb12.i4334
  %io.sroa.5.0.i4321 = phi i32 [ %len.i.i.i4327, %bb12.i4334 ], [ %frames, %bb34 ]
  %io.sroa.0.0.i4322 = phi ptr [ %data.i.i.i4326, %bb12.i4334 ], [ %right_io.0, %bb34 ]
  %818 = icmp eq i32 %io.sroa.5.0.i4321, 0, !dbg !9404
  br i1 %818, label %bb40, label %bb13.preheader.i4323, !dbg !9404

bb13.preheader.i4323:                             ; preds = %bb1.i4320
  %spec.store.select.i4324 = tail call i32 @llvm.umin.i32(i32 %io.sroa.5.0.i4321, i32 32), !dbg !9407
  %data.i.i.idx.i4325 = shl nuw nsw i32 %spec.store.select.i4324, 2, !dbg !9410
  %data.i.i.i4326 = getelementptr inbounds nuw i8, ptr %io.sroa.0.0.i4322, i32 %data.i.i.idx.i4325, !dbg !9410
  br label %bb13.i4328, !dbg !9415

bb13.i4328:                                       ; preds = %bb13.i4328, %bb13.preheader.i4323
  %iter.sroa.0.08.i4329 = phi ptr [ %_35.i4331, %bb13.i4328 ], [ %io.sroa.0.0.i4322, %bb13.preheader.i4323 ]
  %bits.sroa.0.07.i4330 = phi i32 [ %819, %bb13.i4328 ], [ 0, %bb13.preheader.i4323 ]
  %_35.i4331 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i4329, i32 4, !dbg !9417
  %_95.i4332 = load i32, ptr %iter.sroa.0.08.i4329, align 4, !dbg !9419, !alias.scope !9420, !noundef !10
  %819 = or i32 %_95.i4332, %bits.sroa.0.07.i4330, !dbg !9423
  %_29.i4333 = icmp eq ptr %_35.i4331, %data.i.i.i4326, !dbg !9424
  br i1 %_29.i4333, label %bb12.i4334, label %bb13.i4328, !dbg !9415

bb12.i4334:                                       ; preds = %bb13.i4328
  %len.i.i.i4327 = sub nuw nsw i32 %io.sroa.5.0.i4321, %spec.store.select.i4324, !dbg !9426
  %820 = icmp eq i32 %819, 0, !dbg !9427
  br i1 %820, label %bb1.i4320, label %bb40, !dbg !9427

bb42:                                             ; preds = %_RNvMs1_CsjLJhryqjeDL_17true_peak_limiterNtB5_7Cursors7advance.exit, %_RINvNtCscUXHzrJquta_14effect_runtime4bank12finish_blockfNCNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB10_11LimiterCorefE13process_block0EB10_.exit
  ret void, !dbg !9161
}
