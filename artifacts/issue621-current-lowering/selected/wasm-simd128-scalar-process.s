_RNvXsd_CsjLJhryqjeDL_17true_peak_limiterNtB5_23PreparedTruePeakLimiterNtCs2mr8MC2dYJN_15effect_contract20PreparedNativeEffect7process:
.Lfunc_begin42:
	.loc	22 3004 0
	.functype	_RNvXsd_CsjLJhryqjeDL_17true_peak_limiterNtB5_23PreparedTruePeakLimiterNtCs2mr8MC2dYJN_15effect_contract20PreparedNativeEffect7process (i32, i32, i32) -> ()
	.local  	i32, i32, i32, v128, i32, i32
	global.get	__stack_pointer
	i32.const	48
	i32.sub 
	local.tee	3
	global.set	__stack_pointer
.Ltmp8576:
	.loc	22 3006 13 prologue_end
	local.get	2
	i32.load	16
	local.set	4
	block   	
	local.get	2
	i32.load	20
	local.tee	5
	i32.eqz
	br_if   	0
	.loc	22 0 13 is_stmt 0
	local.get	1
	i32.const	0
	.loc	22 3007 13 is_stmt 1
	i32.store8	536
.LBB42_2:
	.loc	22 0 13 is_stmt 0
	end_block
	local.get	3
	i64.const	0
.Ltmp8577:
	.loc	26 1035 30 is_stmt 1
	i64.store	40
	local.get	3
	v128.const	0, 0
	local.tee	6
	v128.store	24:p2align=3
	local.get	3
	local.get	6
	v128.store	8:p2align=3
.Ltmp8578:
	.loc	26 1113 9
	local.get	2
	i32.load	0
	local.set	7
	local.get	2
	i32.load	4
	local.set	8
.Ltmp8579:
	.loc	22 3014 13
	local.get	4
	local.get	5
	local.get	1
	local.get	2
	i64.load	40
	local.get	1
	i32.const	324
	.loc	22 3015 13
	i32.add 
	local.get	1
	i32.const	424
	.loc	22 3016 13
	i32.add 
	i32.const	0
	.loc	22 3011 9
	local.get	3
	i32.const	8
	i32.add 
	call	_RNvCsjLJhryqjeDL_17true_peak_limiter16apply_automation
	.loc	22 3020 45
	local.get	1
	local.get	7
	local.get	8
	local.get	2
	i32.load	8
	local.get	2
	i32.load	12
	.loc	22 3020 19 is_stmt 0
	local.get	8
	call	_RNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB5_11LimiterCorefE13process_blockB5_
	.loc	22 3021 9 is_stmt 1
	local.get	0
	local.get	3
	i64.load	40
	i64.store	32
	local.get	0
	local.get	3
	v128.load	24:p2align=3
	v128.store	16:p2align=3
	local.get	0
	local.get	3
	v128.load	8:p2align=3
	v128.store	0:p2align=3
.Ltmp8580:
	.loc	22 3022 6
	local.get	3
	i32.const	48
	i32.add 
	global.set	__stack_pointer
	end_function
