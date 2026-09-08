_RNvXse_CsjLJhryqjeDL_17true_peak_limiterINtB5_27PreparedTruePeakLimiterBankNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ENtCs2mr8MC2dYJN_15effect_contract24PreparedNativeEffectBank12process_bankB5_:
.Lfunc_begin43:
	.loc	14 3083 0
	.functype	_RNvXse_CsjLJhryqjeDL_17true_peak_limiterINtB5_27PreparedTruePeakLimiterBankNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ENtCs2mr8MC2dYJN_15effect_contract24PreparedNativeEffectBank12process_bankB5_ (i32, i32, i32) -> ()
	.local  	i32, i32, i32, i32, i32, i32, i32, i32, i64, i32, i32, i32
	global.get	__stack_pointer
	i32.const	336
	i32.sub 
	local.tee	3
	global.set	__stack_pointer
.Ltmp8518:
	.loc	14 3126 13 prologue_end
	block   	
	local.get	2
	i32.load	20
	local.tee	4
	i32.eqz
	br_if   	0
	.loc	14 0 13 is_stmt 0
	local.get	1
	i32.const	0
	.loc	14 3127 13 is_stmt 1
	i32.store8	1124
.LBB43_2:
	.loc	14 0 13 is_stmt 0
	end_block
	local.get	2
	i32.load	16
	local.set	5
	.loc	14 3129 51 is_stmt 1
	local.get	1
	i32.load8_u	1232
	local.set	6
	local.get	3
	i32.const	8
	i32.add 
	i32.const	0
	i32.const	320
	memory.fill	0
.Ltmp8519:
	.loc	18 1050 9
	local.get	3
	local.get	6
	i32.store8	328
	block   	
	block   	
	block   	
	local.get	2
	i32.load	28
.Ltmp8520:
	.loc	14 3131 25
	local.tee	7
	i32.eqz
	br_if   	0
	.loc	14 0 25 is_stmt 0
	i32.const	1
	local.set	6
	local.get	7
	i32.const	1
.Ltmp8521:
	.loc	14 3132 23 is_stmt 1
	i32.eq  
	br_if   	2
	.loc	14 0 23 is_stmt 0
	local.get	2
	i32.load	24
	.loc	14 3132 23
	local.tee	8
	i32.load	4
.Ltmp8522:
	.loc	14 0 0
	local.tee	6
	local.get	8
	i32.load	0
.Ltmp8523:
	.loc	13 1050 16 is_stmt 1
	local.tee	9
	i32.lt_u
	br_if   	1
	local.get	6
	local.get	4
	i32.gt_u
	br_if   	1
.Ltmp8524:
	.loc	13 0 16 is_stmt 0
	local.get	5
	local.get	9
	i32.const	40
.Ltmp8525:
	.loc	38 89 24 is_stmt 1
	i32.mul 
	i32.add 
.Ltmp8526:
	.loc	13 1054 31
	local.get	6
	local.get	9
	i32.sub 
	local.get	1
	i32.const	824
	i32.add 
	local.tee	10
	local.get	2
	i64.load	48
	local.tee	11
	local.get	1
	i32.const	924
	i32.add 
	local.tee	12
	local.get	1
	i32.const	1024
	i32.add 
	local.tee	13
	i32.const	0
.Ltmp8527:
	.loc	14 3133 13
	local.get	3
	i32.const	8
	i32.add 
	call	_RNvCsjLJhryqjeDL_17true_peak_limiter16apply_automation
	block   	
	local.get	7
	local.get	7
	i32.const	0
	i32.ne  
	i32.sub 
	local.tee	14
	i32.const	1
.Ltmp8528:
	.loc	14 3132 23
	i32.ne  
	br_if   	0
	.loc	14 0 23 is_stmt 0
	i32.const	2
	local.set	6
	br      	3
.LBB43_8:
	end_block
	.loc	14 3132 23
	local.get	8
	i32.load	8
.Ltmp8529:
	.loc	14 0 0
	local.tee	6
	local.get	8
	i32.load	4
.Ltmp8530:
	.loc	13 1050 16 is_stmt 1
	local.tee	9
	i32.lt_u
	br_if   	1
	local.get	6
	local.get	4
	i32.gt_u
	br_if   	1
.Ltmp8531:
	.loc	13 0 16 is_stmt 0
	local.get	5
	local.get	9
	i32.const	40
.Ltmp8532:
	.loc	38 89 24 is_stmt 1
	i32.mul 
	i32.add 
.Ltmp8533:
	.loc	13 1054 31
	local.get	6
	local.get	9
	i32.sub 
	local.get	10
	local.get	11
	local.get	12
	local.get	13
	i32.const	1
.Ltmp8534:
	.loc	14 3140 17
	local.get	3
	i32.const	8
	i32.add 
	i32.const	40
	i32.add 
	.loc	14 3133 13
	call	_RNvCsjLJhryqjeDL_17true_peak_limiter16apply_automation
	local.get	7
	i32.const	2
.Ltmp8535:
	.loc	14 3131 25
	i32.eq  
	br_if   	0
	.loc	14 0 25 is_stmt 0
	block   	
	local.get	14
	i32.const	2
.Ltmp8536:
	.loc	14 3132 23 is_stmt 1
	i32.ne  
	br_if   	0
	.loc	14 0 23 is_stmt 0
	i32.const	3
	local.set	6
	.loc	14 3132 23
	br      	3
.LBB43_13:
	.loc	14 0 23
	end_block
	.loc	14 3132 23
	local.get	8
	i32.load	12
.Ltmp8537:
	.loc	14 0 0
	local.tee	6
	local.get	8
	i32.load	8
.Ltmp8538:
	.loc	13 1050 16 is_stmt 1
	local.tee	9
	i32.lt_u
	br_if   	1
	local.get	6
	local.get	4
	i32.gt_u
	br_if   	1
.Ltmp8539:
	.loc	13 0 16 is_stmt 0
	local.get	5
	local.get	9
	i32.const	40
.Ltmp8540:
	.loc	38 89 24 is_stmt 1
	i32.mul 
	i32.add 
.Ltmp8541:
	.loc	13 1054 31
	local.get	6
	local.get	9
	i32.sub 
	local.get	10
	local.get	11
	local.get	12
	local.get	13
	i32.const	2
	local.get	3
	i32.const	88
.Ltmp8542:
	.loc	14 3140 17
	i32.add 
	.loc	14 3133 13
	call	_RNvCsjLJhryqjeDL_17true_peak_limiter16apply_automation
	local.get	7
	i32.const	3
.Ltmp8543:
	.loc	14 3131 25
	i32.eq  
	br_if   	0
	.loc	14 0 25 is_stmt 0
	block   	
	local.get	14
	i32.const	3
.Ltmp8544:
	.loc	14 3132 23 is_stmt 1
	i32.ne  
	br_if   	0
	.loc	14 0 23 is_stmt 0
	i32.const	4
	local.set	6
	.loc	14 3132 23
	br      	3
.LBB43_18:
	.loc	14 0 23
	end_block
	.loc	14 3132 23
	local.get	8
	i32.load	16
.Ltmp8545:
	.loc	14 0 0
	local.tee	6
	local.get	8
	i32.load	12
.Ltmp8546:
	.loc	13 1050 16 is_stmt 1
	local.tee	9
	i32.lt_u
	br_if   	1
	local.get	6
	local.get	4
	i32.gt_u
	br_if   	1
.Ltmp8547:
	.loc	13 0 16 is_stmt 0
	local.get	5
	local.get	9
	i32.const	40
.Ltmp8548:
	.loc	38 89 24 is_stmt 1
	i32.mul 
	i32.add 
.Ltmp8549:
	.loc	13 1054 31
	local.get	6
	local.get	9
	i32.sub 
	local.get	10
	local.get	11
	local.get	12
	local.get	13
	i32.const	3
	local.get	3
	i32.const	128
.Ltmp8550:
	.loc	14 3140 17
	i32.add 
	.loc	14 3133 13
	call	_RNvCsjLJhryqjeDL_17true_peak_limiter16apply_automation
.Ltmp8551:
	.loc	14 3148 32
	local.get	1
	local.get	2
	i32.load	0
	local.get	2
	i32.load	4
	.loc	14 3148 44 is_stmt 0
	local.get	2
	i32.load	8
	local.get	2
	i32.load	12
	.loc	14 3148 57
	local.get	2
	i32.load	56
	.loc	14 3148 18
	call	_RNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB5_11LimiterCoreNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E13process_blockB5_
	.loc	14 3150 9 is_stmt 1
	local.get	0
	local.get	3
	i32.const	8
	i32.add 
	i32.const	328
	memory.copy	0, 0
.Ltmp8552:
	.loc	14 3085 6
	local.get	3
	i32.const	336
	i32.add 
	global.set	__stack_pointer
	return
.LBB43_21:
	.loc	14 0 6 is_stmt 0
	end_block
.Ltmp8553:
	.loc	14 3131 25 is_stmt 1
	local.get	7
	local.get	7
	i32.const	.Lalloc_297cd72dee4462ad27dbe24f13722ef7
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.LBB43_22:
	.loc	14 0 25 is_stmt 0
	end_block
.Ltmp8554:
	.loc	38 443 13 is_stmt 1
	local.get	9
	local.get	6
	local.get	4
	i32.const	.Lalloc_20d21e0032ad9c6108b3daa26f0ec9a2
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp8555:
.LBB43_23:
	.loc	38 0 13 is_stmt 0
	end_block
	.loc	14 3132 23 is_stmt 1
	local.get	6
	local.get	7
	i32.const	.Lalloc_f03666103170d1da5fd4470c4d946344
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
	end_function
