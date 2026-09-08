_RNvXse_CsjLJhryqjeDL_17true_peak_limiterINtB5_27PreparedTruePeakLimiterBankNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ENtCs2mr8MC2dYJN_15effect_contract24PreparedNativeEffectBank12process_bankB5_:
.Lfunc_begin44:
	.loc	22 3088 0
	.functype	_RNvXse_CsjLJhryqjeDL_17true_peak_limiterINtB5_27PreparedTruePeakLimiterBankNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ENtCs2mr8MC2dYJN_15effect_contract24PreparedNativeEffectBank12process_bankB5_ (i32, i32, i32) -> ()
	.local  	i32, i32, i32, i32, i32, i32, i32, i32, i64, i32, i32, i32
	global.get	__stack_pointer
	i32.const	336
	i32.sub 
	local.tee	3
	global.set	__stack_pointer
.Ltmp8584:
	.loc	22 3131 13 prologue_end
	block   	
	local.get	2
	i32.load	20
	local.tee	4
	i32.eqz
	br_if   	0
	.loc	22 0 13 is_stmt 0
	local.get	1
	i32.const	0
	.loc	22 3132 13 is_stmt 1
	i32.store8	1124
.LBB44_2:
	.loc	22 0 13 is_stmt 0
	end_block
	local.get	2
	i32.load	16
	local.set	5
	.loc	22 3134 51 is_stmt 1
	local.get	1
	i32.load8_u	1232
	local.set	6
	local.get	3
	i32.const	8
	i32.add 
	i32.const	0
	i32.const	320
	memory.fill	0
.Ltmp8585:
	.loc	26 1050 9
	local.get	3
	local.get	6
	i32.store8	328
	block   	
	block   	
	block   	
	local.get	2
	i32.load	28
.Ltmp8586:
	.loc	22 3136 25
	local.tee	7
	i32.eqz
	br_if   	0
	.loc	22 0 25 is_stmt 0
	i32.const	1
	local.set	6
	local.get	7
	i32.const	1
.Ltmp8587:
	.loc	22 3137 23 is_stmt 1
	i32.eq  
	br_if   	2
	.loc	22 0 23 is_stmt 0
	local.get	2
	i32.load	24
	.loc	22 3137 23
	local.tee	8
	i32.load	4
.Ltmp8588:
	.loc	22 0 0
	local.tee	6
	local.get	8
	i32.load	0
.Ltmp8589:
	.loc	21 1050 16 is_stmt 1
	local.tee	9
	i32.lt_u
	br_if   	1
	local.get	6
	local.get	4
	i32.gt_u
	br_if   	1
.Ltmp8590:
	.loc	21 0 16 is_stmt 0
	local.get	5
	local.get	9
	i32.const	40
.Ltmp8591:
	.loc	42 89 24 is_stmt 1
	i32.mul 
	i32.add 
.Ltmp8592:
	.loc	21 1054 31
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
.Ltmp8593:
	.loc	22 3138 13
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
.Ltmp8594:
	.loc	22 3137 23
	i32.ne  
	br_if   	0
	.loc	22 0 23 is_stmt 0
	i32.const	2
	local.set	6
	br      	3
.LBB44_8:
	end_block
	.loc	22 3137 23
	local.get	8
	i32.load	8
.Ltmp8595:
	.loc	22 0 0
	local.tee	6
	local.get	8
	i32.load	4
.Ltmp8596:
	.loc	21 1050 16 is_stmt 1
	local.tee	9
	i32.lt_u
	br_if   	1
	local.get	6
	local.get	4
	i32.gt_u
	br_if   	1
.Ltmp8597:
	.loc	21 0 16 is_stmt 0
	local.get	5
	local.get	9
	i32.const	40
.Ltmp8598:
	.loc	42 89 24 is_stmt 1
	i32.mul 
	i32.add 
.Ltmp8599:
	.loc	21 1054 31
	local.get	6
	local.get	9
	i32.sub 
	local.get	10
	local.get	11
	local.get	12
	local.get	13
	i32.const	1
.Ltmp8600:
	.loc	22 3145 17
	local.get	3
	i32.const	8
	i32.add 
	i32.const	40
	i32.add 
	.loc	22 3138 13
	call	_RNvCsjLJhryqjeDL_17true_peak_limiter16apply_automation
	local.get	7
	i32.const	2
.Ltmp8601:
	.loc	22 3136 25
	i32.eq  
	br_if   	0
	.loc	22 0 25 is_stmt 0
	block   	
	local.get	14
	i32.const	2
.Ltmp8602:
	.loc	22 3137 23 is_stmt 1
	i32.ne  
	br_if   	0
	.loc	22 0 23 is_stmt 0
	i32.const	3
	local.set	6
	.loc	22 3137 23
	br      	3
.LBB44_13:
	.loc	22 0 23
	end_block
	.loc	22 3137 23
	local.get	8
	i32.load	12
.Ltmp8603:
	.loc	22 0 0
	local.tee	6
	local.get	8
	i32.load	8
.Ltmp8604:
	.loc	21 1050 16 is_stmt 1
	local.tee	9
	i32.lt_u
	br_if   	1
	local.get	6
	local.get	4
	i32.gt_u
	br_if   	1
.Ltmp8605:
	.loc	21 0 16 is_stmt 0
	local.get	5
	local.get	9
	i32.const	40
.Ltmp8606:
	.loc	42 89 24 is_stmt 1
	i32.mul 
	i32.add 
.Ltmp8607:
	.loc	21 1054 31
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
.Ltmp8608:
	.loc	22 3145 17
	i32.add 
	.loc	22 3138 13
	call	_RNvCsjLJhryqjeDL_17true_peak_limiter16apply_automation
	local.get	7
	i32.const	3
.Ltmp8609:
	.loc	22 3136 25
	i32.eq  
	br_if   	0
	.loc	22 0 25 is_stmt 0
	block   	
	local.get	14
	i32.const	3
.Ltmp8610:
	.loc	22 3137 23 is_stmt 1
	i32.ne  
	br_if   	0
	.loc	22 0 23 is_stmt 0
	i32.const	4
	local.set	6
	.loc	22 3137 23
	br      	3
.LBB44_18:
	.loc	22 0 23
	end_block
	.loc	22 3137 23
	local.get	8
	i32.load	16
.Ltmp8611:
	.loc	22 0 0
	local.tee	6
	local.get	8
	i32.load	12
.Ltmp8612:
	.loc	21 1050 16 is_stmt 1
	local.tee	9
	i32.lt_u
	br_if   	1
	local.get	6
	local.get	4
	i32.gt_u
	br_if   	1
.Ltmp8613:
	.loc	21 0 16 is_stmt 0
	local.get	5
	local.get	9
	i32.const	40
.Ltmp8614:
	.loc	42 89 24 is_stmt 1
	i32.mul 
	i32.add 
.Ltmp8615:
	.loc	21 1054 31
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
.Ltmp8616:
	.loc	22 3145 17
	i32.add 
	.loc	22 3138 13
	call	_RNvCsjLJhryqjeDL_17true_peak_limiter16apply_automation
.Ltmp8617:
	.loc	22 3153 32
	local.get	1
	local.get	2
	i32.load	0
	local.get	2
	i32.load	4
	.loc	22 3153 44 is_stmt 0
	local.get	2
	i32.load	8
	local.get	2
	i32.load	12
	.loc	22 3153 57
	local.get	2
	i32.load	56
	.loc	22 3153 18
	call	_RNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB5_11LimiterCoreNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E13process_blockB5_
	.loc	22 3155 9 is_stmt 1
	local.get	0
	local.get	3
	i32.const	8
	i32.add 
	i32.const	328
	memory.copy	0, 0
.Ltmp8618:
	.loc	22 3090 6
	local.get	3
	i32.const	336
	i32.add 
	global.set	__stack_pointer
	return
.LBB44_21:
	.loc	22 0 6 is_stmt 0
	end_block
.Ltmp8619:
	.loc	22 3136 25 is_stmt 1
	local.get	7
	local.get	7
	i32.const	.Lalloc_4de42a380bf11124b0fb54945ff6764c
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.LBB44_22:
	.loc	22 0 25 is_stmt 0
	end_block
.Ltmp8620:
	.loc	42 443 13 is_stmt 1
	local.get	9
	local.get	6
	local.get	4
	i32.const	.Lalloc_1667d564aac521e9269794d81fe09235
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp8621:
.LBB44_23:
	.loc	42 0 13 is_stmt 0
	end_block
	.loc	22 3137 23 is_stmt 1
	local.get	6
	local.get	7
	i32.const	.Lalloc_73103fbd0321956551877cb949272a2e
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
	end_function
