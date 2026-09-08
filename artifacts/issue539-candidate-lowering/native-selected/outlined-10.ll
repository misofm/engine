define internal fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 32 captures(none) dereferenceable(128) %_0, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(200) %state, i64 %shape.8.val, i64 %shape.16.val) unnamed_addr #1 !dbg !16467 {
start:
  %ring_words = shl i64 %shape.8.val, 3, !dbg !16468
  %main_words = shl i64 %shape.16.val, 3, !dbg !16469
  %0 = getelementptr inbounds nuw i8, ptr %state, i64 184, !dbg !16471
  %_58.1 = load i64, ptr %0, align 8, !dbg !16471, !noundef !12
  %_8.not = icmp eq i64 %_58.1, 0, !dbg !16471
  br i1 %_8.not, label %panic, label %bb1, !dbg !16471

bb1:                                              ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %state, i64 176, !dbg !16471
  %_58.0 = load ptr, ptr %1, align 8, !dbg !16471, !nonnull !12, !noundef !12
  %lane = load i32, ptr %_58.0, align 4, !dbg !16471, !noundef !12
  %2 = getelementptr inbounds nuw i8, ptr %_58.0, i64 4, !dbg !16471
  %lane1 = load i32, ptr %2, align 4, !dbg !16471, !noundef !12
  %3 = getelementptr inbounds nuw i8, ptr %_58.0, i64 8, !dbg !16471
  %lane2 = load i32, ptr %3, align 4, !dbg !16471, !noundef !12
  %4 = getelementptr inbounds nuw i8, ptr %state, i64 80, !dbg !16473
  %_60.0 = load ptr, ptr %4, align 8, !dbg !16473, !nonnull !12, !noundef !12
  %5 = getelementptr inbounds nuw i8, ptr %state, i64 88, !dbg !16473
  %_60.1 = load i64, ptr %5, align 8, !dbg !16473, !noundef !12
  %_8.i = icmp samesign ugt i64 %_60.1, 7, !dbg !16475
  br i1 %_8.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit, label %bb2.i, !dbg !16475, !prof !651

bb2.i:                                            ; preds = %bb1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_60.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !16480, !noalias !16481
  unreachable, !dbg !16480

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit: ; preds = %bb1
  %6 = getelementptr inbounds nuw i8, ptr %state, i64 120, !dbg !16485
  %_61.1 = load i64, ptr %6, align 8, !dbg !16485, !noundef !12
  %_13.not = icmp eq i64 %_61.1, 0, !dbg !16485
  br i1 %_13.not, label %panic3, label %bb3, !dbg !16485

panic:                                            ; preds = %start
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4552947ca1e665baf4644d94967030f9) #30, !dbg !16471
  unreachable, !dbg !16471

bb3:                                              ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit
  %7 = getelementptr inbounds nuw i8, ptr %state, i64 112, !dbg !16485
  %_61.0 = load ptr, ptr %7, align 8, !dbg !16485, !nonnull !12, !noundef !12
  %_11 = load i32, ptr %_61.0, align 4, !dbg !16485, !noundef !12
  %8 = getelementptr inbounds nuw i8, ptr %state, i64 32, !dbg !16486
  %_63.0 = load ptr, ptr %8, align 8, !dbg !16486, !nonnull !12, !noundef !12
  %9 = getelementptr inbounds nuw i8, ptr %state, i64 40, !dbg !16486
  %_63.1 = load i64, ptr %9, align 8, !dbg !16486, !noundef !12
  %_32.not = icmp ugt i64 %ring_words, %_63.1
  br i1 %_32.not, label %bb6, label %bb4, !dbg !16487, !prof !165

panic3:                                           ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_823a10f588a6cbdf8b85bd154459b44e) #30, !dbg !16485
  unreachable, !dbg !16485

bb6:                                              ; preds = %bb3
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %ring_words, i64 noundef %_63.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c8124f7a6de0156cfd71e0a65b4b69a2) #30, !dbg !16496
  unreachable, !dbg !16496

bb4:                                              ; preds = %bb3
  %10 = getelementptr inbounds nuw i8, ptr %state, i64 48, !dbg !16497
  %_64.0 = load ptr, ptr %10, align 8, !dbg !16497, !nonnull !12, !noundef !12
  %11 = getelementptr inbounds nuw i8, ptr %state, i64 56, !dbg !16497
  %_64.1 = load i64, ptr %11, align 8, !dbg !16497, !noundef !12
  %_40.not = icmp ugt i64 %ring_words, %_64.1, !dbg !16498
  br i1 %_40.not, label %bb9, label %bb8, !dbg !16498, !prof !639

bb9:                                              ; preds = %bb4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %ring_words, i64 noundef %_64.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_624ccde1cab09599ea02385287f48f78) #30, !dbg !16504
  unreachable, !dbg !16504

bb8:                                              ; preds = %bb4
  %12 = getelementptr inbounds nuw i8, ptr %state, i64 24, !dbg !16505
  %_65.1 = load i64, ptr %12, align 8, !dbg !16505, !noundef !12
  %_47.not = icmp ugt i64 %main_words, %_65.1
  br i1 %_47.not, label %bb12, label %bb10, !dbg !16506, !prof !165

bb10:                                             ; preds = %bb8
  %13 = getelementptr inbounds nuw i8, ptr %state, i64 16, !dbg !16505
  %_65.0 = load ptr, ptr %13, align 8, !dbg !16505, !nonnull !12, !noundef !12
  %_31 = zext i32 %lane2 to i64, !dbg !16514
  %_30 = zext i32 %lane1 to i64, !dbg !16517
  %_29 = zext i32 %lane to i64, !dbg !16518
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_0, ptr noundef nonnull align 4 dereferenceable(32) %_60.0, i64 32, i1 false), !dbg !16519
  %14 = getelementptr inbounds nuw i8, ptr %_0, i64 104, !dbg !16519
  store i32 %_11, ptr %14, align 8, !dbg !16519
  %15 = getelementptr inbounds nuw i8, ptr %_0, i64 80, !dbg !16519
  store i64 %_29, ptr %15, align 16, !dbg !16519
  %_14.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 88, !dbg !16519
  store i64 %_30, ptr %_14.sroa.4.0..sroa_idx, align 8, !dbg !16519
  %_14.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 96, !dbg !16519
  store i64 %_31, ptr %_14.sroa.5.0..sroa_idx, align 16, !dbg !16519
  %16 = getelementptr inbounds nuw i8, ptr %_0, i64 32, !dbg !16519
  store ptr %_63.0, ptr %16, align 32, !dbg !16519
  %17 = getelementptr inbounds nuw i8, ptr %_0, i64 40, !dbg !16519
  store i64 %ring_words, ptr %17, align 8, !dbg !16519
  %18 = getelementptr inbounds nuw i8, ptr %_0, i64 48, !dbg !16519
  store ptr %_64.0, ptr %18, align 16, !dbg !16519
  %19 = getelementptr inbounds nuw i8, ptr %_0, i64 56, !dbg !16519
  store i64 %ring_words, ptr %19, align 8, !dbg !16519
  %20 = getelementptr inbounds nuw i8, ptr %_0, i64 64, !dbg !16519
  store ptr %_65.0, ptr %20, align 32, !dbg !16519
  %21 = getelementptr inbounds nuw i8, ptr %_0, i64 72, !dbg !16519
  store i64 %main_words, ptr %21, align 8, !dbg !16519
  ret void, !dbg !16520

bb12:                                             ; preds = %bb8
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %main_words, i64 noundef %_65.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6eb85161aaa18d5606c3983debc5991b) #30, !dbg !16521
  unreachable, !dbg !16521
}
