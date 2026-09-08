_RNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB5_11LimiterCoreNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E18process_block_monoB5_:
.Lfunc_begin27:
	.loc	14 2303 0
	.functype	_RNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB5_11LimiterCoreNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E18process_block_monoB5_ (i32, i32, i32, i32) -> ()
	.local  	i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, v128, v128, v128, v128, f32, f32, v128, v128, i64
	global.get	__stack_pointer
	i32.const	2528
	i32.sub 
	local.tee	4
	global.set	__stack_pointer
	local.get	3
	i32.const	2
.Ltmp4391:
	.loc	14 2304 21 prologue_end
	i32.shl 
	local.set	5
.Ltmp4392:
	.loc	14 0 0 is_stmt 0
	local.get	0
	i32.load	992
	local.set	6
	local.get	0
	i32.load	988
	local.set	7
	.loc	14 2305 21 is_stmt 1
	block   	
	block   	
	block   	
	block   	
	block   	
	local.get	0
	i32.load8_u	1125
	.loc	14 2305 43 is_stmt 0
	local.get	0
	i32.load8_u	904
	.loc	14 2305 21
	i32.ne  
	br_if   	0
	.loc	14 0 21
	local.get	6
	i32.const	4
.Ltmp4393:
	.loc	15 314 17 is_stmt 1
	i32.shl 
	local.set	8
	local.get	7
	local.set	9
.LBB27_2:
.Ltmp4394:
	.loc	15 180 28
	block   	
	loop    	
	local.get	8
	i32.eqz
	br_if   	1
.Ltmp4395:
	.loc	14 690 21
	local.get	9
	i32.load	12
.Ltmp4396:
	.loc	15 315 25
	br_if   	2
	.loc	15 0 25 is_stmt 0
	local.get	8
	i32.const	-16
	i32.add 
	local.set	8
	.loc	15 315 25
	local.get	9
	i32.load	4
	local.set	10
	local.get	9
	i32.load	0
	local.set	11
	local.get	9
	i32.const	16
	.loc	15 0 0
	i32.add 
	local.set	9
	.loc	15 315 25
	local.get	11
	local.get	10
	i32.eq  
	br_if   	0
	br      	2
.LBB27_5:
.Ltmp4397:
	.loc	15 180 28 is_stmt 1
	end_loop
	end_block
.Ltmp4398:
	.loc	14 2307 37
	local.get	0
	i32.load	1000
	i32.const	4
.Ltmp4399:
	.loc	15 314 17
	i32.shl 
	local.set	8
.Ltmp4400:
	.loc	14 2307 37
	local.get	0
	i32.load	996
	local.set	9
.LBB27_6:
.Ltmp4401:
	.loc	15 180 28
	block   	
	loop    	
	local.get	8
	i32.eqz
	br_if   	1
.Ltmp4402:
	.loc	14 690 21
	local.get	9
	i32.load	12
.Ltmp4403:
	.loc	15 315 25
	br_if   	2
	.loc	15 0 25 is_stmt 0
	local.get	8
	i32.const	-16
	i32.add 
	local.set	8
	.loc	15 315 25
	local.get	9
	i32.load	4
	local.set	10
	local.get	9
	i32.load	0
	local.set	11
	local.get	9
	i32.const	16
	.loc	15 0 0
	i32.add 
	local.set	9
	.loc	15 315 25
	local.get	11
	local.get	10
	i32.ne  
	br_if   	2
	br      	0
.LBB27_9:
.Ltmp4404:
	.loc	15 180 28 is_stmt 1
	end_loop
	end_block
	block   	
	block   	
	local.get	5
	local.get	2
	i32.gt_u
.Ltmp4405:
	.loc	13 1050 16
	br_if   	0
.Ltmp4406:
	.loc	13 0 16 is_stmt 0
	local.get	5
	local.set	11
	local.get	1
	local.set	9
.LBB27_11:
.Ltmp4407:
	.loc	30 1504 12 is_stmt 1
	loop    	
	local.get	11
	i32.eqz
	br_if   	2
	.loc	30 0 12 is_stmt 0
	local.get	9
	local.get	11
	i32.const	32
	local.get	11
	i32.const	32
.Ltmp4408:
	.loc	9 1077 12 is_stmt 1
	i32.lt_u
	i32.select
	local.tee	12
	i32.const	2
.Ltmp4409:
	.loc	35 863 18
	i32.shl 
	local.tee	10
	i32.add 
	local.set	13
	i32.const	0
	local.set	8
.Ltmp4410:
.LBB27_13:
	.loc	36 134 21
	loop    	
	local.get	9
	i32.load	0
	.loc	36 134 13 is_stmt 0
	local.get	8
	i32.or  
	local.set	8
	local.get	9
	i32.const	4
.Ltmp4411:
	.loc	16 656 28 is_stmt 1
	i32.add 
	local.set	9
	local.get	10
	i32.const	-4
.Ltmp4412:
	.loc	16 1714 9
	i32.add 
.Ltmp4413:
	.loc	15 180 28
	local.tee	10
	br_if   	0
	end_loop
.Ltmp4414:
	.loc	31 2054 74
	local.get	11
	local.get	12
	i32.sub 
	local.set	11
	local.get	13
	local.set	9
.Ltmp4415:
	.loc	36 136 12
	local.get	8
	i32.eqz
	br_if   	0
	end_loop
	i32.const	0
	local.set	14
.Ltmp4416:
	.loc	14 2309 12
	br      	3
.Ltmp4417:
.LBB27_16:
	.loc	14 0 12 is_stmt 0
	end_block
	i32.const	0
.Ltmp4418:
	.loc	38 443 13 is_stmt 1
	local.get	5
	local.get	2
	i32.const	.Lalloc_2a8b9f3e4701c8e7f14c8a14ff519a24
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp4419:
.LBB27_17:
	.loc	38 0 13 is_stmt 0
	end_block
	i32.const	1
	local.set	14
	local.get	0
	i32.load8_u	1124
.Ltmp4420:
	.loc	14 2309 12 is_stmt 1
	i32.eqz
	br_if   	1
.Ltmp4421:
	.loc	14 663 57
	block   	
	block   	
	block   	
	block   	
	local.get	0
	i32.load	1016
	.loc	14 663 31 is_stmt 0
	local.tee	9
	local.get	0
	i32.load	984
.Ltmp4422:
	.loc	9 1077 12 is_stmt 1
	local.tee	8
	local.get	9
	local.get	8
	i32.lt_u
	i32.select
.Ltmp4423:
	.loc	33 304 12
	local.tee	7
	i32.eqz
	br_if   	0
.Ltmp4424:
	.loc	33 0 12 is_stmt 0
	local.get	0
	i32.load	1012
	local.set	8
	local.get	0
	i32.load	980
	local.set	9
.LBB27_20:
.Ltmp4425:
	.loc	14 664 26 is_stmt 1
	loop    	
	local.get	8
	i32.load	0
.Ltmp4426:
	.loc	14 665 42
	local.tee	10
	i32.eqz
	br_if   	2
	local.get	9
	local.get	3
	local.get	10
	i32.rem_u
	.loc	14 665 24 is_stmt 0
	local.get	9
	i32.load	0
	.loc	14 665 23
	i32.add 
	.loc	14 665 22
	local.get	10
	i32.rem_u
	.loc	14 665 13
	i32.store	0
	local.get	8
	i32.const	12
.Ltmp4427:
	.loc	33 304 12 is_stmt 1
	i32.add 
	local.set	8
	local.get	9
	i32.const	4
	i32.add 
	local.set	9
	local.get	7
	i32.const	-1
	i32.add 
	local.tee	7
	br_if   	0
.LBB27_22:
	end_loop
	end_block
.Ltmp4428:
	.loc	14 2311 26
	local.get	0
	i32.load	920
.Ltmp4429:
	.loc	14 455 44
	local.tee	8
	i32.eqz
	br_if   	1
.Ltmp4430:
	.loc	14 2311 26
	local.get	0
	i32.load	916
	local.set	9
.Ltmp4431:
	.loc	14 455 44
	local.get	0
	local.get	3
	local.get	8
	i32.rem_u
	.loc	14 455 23 is_stmt 0
	local.get	0
	i32.load	800
	.loc	14 455 22
	i32.add 
	.loc	14 455 21
	local.get	8
	i32.rem_u
	.loc	14 455 9
	i32.store	800
	.loc	14 456 44 is_stmt 1
	local.get	9
	i32.eqz
	br_if   	2
	local.get	0
	local.get	3
	local.get	9
	i32.rem_u
	.loc	14 456 23 is_stmt 0
	local.get	0
	i32.load	804
	.loc	14 456 22
	i32.add 
	.loc	14 456 21
	local.get	9
	i32.rem_u
	.loc	14 456 9
	i32.store	804
	br      	5
.Ltmp4432:
.LBB27_25:
	.loc	14 0 9
	end_block
.Ltmp4433:
	.loc	14 665 42 is_stmt 1
	i32.const	.Lalloc_f5b0427df9b659e554a697ca46ce8b5a
	call	_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero
	unreachable
.Ltmp4434:
.LBB27_26:
	.loc	14 0 42 is_stmt 0
	end_block
.Ltmp4435:
	.loc	14 455 44 is_stmt 1
	i32.const	.Lalloc_c7e64e9e8489bc8e648659d0098d136c
	call	_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero
	unreachable
.LBB27_27:
	.loc	14 0 44 is_stmt 0
	end_block
	.loc	14 456 44 is_stmt 1
	i32.const	.Lalloc_34b23597fad9d85cb3bee6d64ebf77d5
	call	_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero
	unreachable
.Ltmp4436:
.LBB27_28:
	.loc	14 0 44 is_stmt 0
	end_block
	i32.const	0
	local.set	14
.LBB27_29:
	end_block
	local.get	6
	i32.const	4
.Ltmp4437:
	.loc	15 314 17 is_stmt 1
	i32.shl 
	local.set	9
	local.get	0
	i32.const	924
.Ltmp4438:
	.loc	14 2323 13
	i32.add 
	local.set	15
	local.get	0
	i32.const	912
	.loc	14 2322 13
	i32.add 
	local.set	16
.LBB27_30:
.Ltmp4439:
	.loc	15 180 28
	block   	
	block   	
	block   	
	block   	
	block   	
	block   	
	block   	
	block   	
	block   	
	block   	
	loop    	
	local.get	9
	i32.eqz
	br_if   	1
.Ltmp4440:
	.loc	14 690 21
	block   	
	local.get	7
	i32.load	12
.Ltmp4441:
	.loc	15 315 25
	br_if   	0
	.loc	15 0 25 is_stmt 0
	local.get	9
	i32.const	-16
	i32.add 
	local.set	9
	.loc	15 315 25
	local.get	7
	i32.load	4
	local.set	8
	local.get	7
	i32.load	0
	local.set	10
	local.get	7
	i32.const	16
	.loc	15 0 0
	i32.add 
	local.set	7
	.loc	15 315 25
	local.get	10
	local.get	8
	i32.eq  
	br_if   	1
.LBB27_33:
	.loc	15 0 25
	end_block
	.loc	15 315 25
	end_loop
.Ltmp4442:
	.loc	14 1116 5 is_stmt 1
	block   	
	local.get	0
	i32.load	1016
	local.tee	9
	i32.eqz
	br_if   	0
	.loc	14 0 5 is_stmt 0
	local.get	9
	i32.const	12
	i32.mul 
	local.set	8
	local.get	0
	i32.load	1012
	local.tee	10
	local.set	9
.LBB27_35:
.Ltmp4443:
	.loc	15 180 28 is_stmt 1
	loop    	
	local.get	8
	i32.eqz
	br_if   	1
.Ltmp4444:
	.loc	14 390 34
	local.get	9
	i32.load	0
	local.get	10
	i32.load	0
	i32.ne  
	br_if   	7
	local.get	9
	i32.load	4
	local.get	10
	i32.load	4
	i32.ne  
	br_if   	7
.Ltmp4445:
	.loc	14 0 34 is_stmt 0
	local.get	8
	i32.const	-12
	.loc	15 315 25 is_stmt 1
	i32.add 
	local.set	8
.Ltmp4446:
	.loc	14 390 34
	local.get	9
	i32.load	8
	local.set	7
	local.get	9
	i32.const	12
.Ltmp4447:
	.loc	15 0 0 is_stmt 0
	i32.add 
	local.set	9
.Ltmp4448:
	.loc	14 390 34
	local.get	7
	local.get	10
	i32.load	8
	i32.eq  
.Ltmp4449:
	.loc	15 315 25 is_stmt 1
	br_if   	0
	br      	7
.LBB27_39:
.Ltmp4450:
	.loc	15 180 28
	end_loop
	end_block
.Ltmp4451:
	.loc	14 1117 12
	local.get	0
	i32.load	984
	local.tee	9
	i32.eqz
	br_if   	1
	.loc	14 0 12 is_stmt 0
	local.get	9
	i32.const	2
	i32.shl 
	local.set	7
	local.get	0
	i32.load	980
	local.set	8
	i32.const	0
	local.set	9
.LBB27_41:
.Ltmp4452:
	.loc	16 1714 9 is_stmt 1
	loop    	
	local.get	7
	local.get	9
	i32.eq  
.Ltmp4453:
	.loc	15 180 28
	br_if   	2
.Ltmp4454:
	.loc	15 0 0 is_stmt 0
	local.get	8
	local.get	9
	i32.add 
	local.set	10
	local.get	9
	i32.const	4
	.loc	15 315 25 is_stmt 1
	i32.add 
	local.set	9
	local.get	10
	i32.load	0
.Ltmp4455:
	.loc	14 1117 53
	local.get	8
	i32.load	0
	.loc	14 1117 43 is_stmt 0
	i32.ne  
.Ltmp4456:
	.loc	15 315 25 is_stmt 1
	br_if   	6
	br      	0
.Ltmp4457:
.LBB27_43:
	.loc	15 180 28
	end_loop
	end_block
.Ltmp4458:
	.loc	14 715 63
	local.get	0
	i32.load	1000
	i32.const	4
.Ltmp4459:
	.loc	15 314 17
	i32.shl 
	local.set	10
.Ltmp4460:
	.loc	14 715 63
	local.get	0
	i32.load	996
	local.set	9
.LBB27_44:
	.loc	14 0 63 is_stmt 0
	block   	
	loop    	
	local.get	10
.Ltmp4461:
	.loc	15 180 28 is_stmt 1
	local.tee	8
	i32.eqz
	br_if   	1
.Ltmp4462:
	.loc	14 690 21
	local.get	9
	i32.load	12
.Ltmp4463:
	.loc	15 315 25
	br_if   	1
	.loc	15 0 25 is_stmt 0
	local.get	8
	i32.const	-16
	i32.add 
	local.set	10
	.loc	15 315 25
	local.get	9
	i32.load	4
	local.set	7
	local.get	9
	i32.load	0
	local.set	11
	local.get	9
	i32.const	16
	.loc	15 0 0
	i32.add 
	local.set	9
	.loc	15 315 25
	local.get	11
	local.get	7
	i32.eq  
	br_if   	0
.LBB27_47:
	.loc	15 315 25 is_stmt 1
	end_loop
	end_block
.Ltmp4464:
	.loc	14 1116 5
	block   	
	local.get	0
	i32.load	1016
	local.tee	9
	i32.eqz
	br_if   	0
	.loc	14 0 5 is_stmt 0
	local.get	9
	i32.const	12
	i32.mul 
	local.set	10
	local.get	0
	i32.load	1012
	local.tee	7
	local.set	9
.LBB27_49:
.Ltmp4465:
	.loc	15 180 28 is_stmt 1
	loop    	
	local.get	10
	i32.eqz
	br_if   	1
.Ltmp4466:
	.loc	14 390 34
	local.get	9
	i32.load	0
	local.get	7
	i32.load	0
	i32.ne  
	br_if   	5
	local.get	9
	i32.load	4
	local.get	7
	i32.load	4
	i32.ne  
	br_if   	5
.Ltmp4467:
	.loc	14 0 34 is_stmt 0
	local.get	10
	i32.const	-12
	.loc	15 315 25 is_stmt 1
	i32.add 
	local.set	10
.Ltmp4468:
	.loc	14 390 34
	local.get	9
	i32.load	8
	local.set	11
	local.get	9
	i32.const	12
.Ltmp4469:
	.loc	15 0 0 is_stmt 0
	i32.add 
	local.set	9
.Ltmp4470:
	.loc	14 390 34
	local.get	11
	local.get	7
	i32.load	8
	i32.eq  
.Ltmp4471:
	.loc	15 315 25 is_stmt 1
	br_if   	0
	br      	5
.LBB27_53:
.Ltmp4472:
	.loc	15 180 28
	end_loop
	end_block
.Ltmp4473:
	.loc	14 1117 12
	block   	
	local.get	0
	i32.load	984
	local.tee	9
	i32.eqz
	br_if   	0
	.loc	14 0 12 is_stmt 0
	local.get	9
	i32.const	2
	i32.shl 
	local.set	11
	local.get	0
	i32.load	980
	local.set	10
	i32.const	0
	local.set	9
.LBB27_55:
.Ltmp4474:
	.loc	16 1714 9 is_stmt 1
	loop    	
	local.get	11
	local.get	9
	i32.eq  
.Ltmp4475:
	.loc	15 180 28
	br_if   	1
.Ltmp4476:
	.loc	15 0 0 is_stmt 0
	local.get	10
	local.get	9
	i32.add 
	local.set	7
	local.get	9
	i32.const	4
	.loc	15 315 25 is_stmt 1
	i32.add 
	local.set	9
	local.get	7
	i32.load	0
.Ltmp4477:
	.loc	14 1117 53
	local.get	10
	i32.load	0
	.loc	14 1117 43 is_stmt 0
	i32.ne  
.Ltmp4478:
	.loc	15 315 25 is_stmt 1
	br_if   	5
	br      	0
.LBB27_57:
.Ltmp4479:
	.loc	15 180 28
	end_loop
	end_block
.Ltmp4480:
	.loc	14 3187 12
	local.get	8
	i32.eqz
	br_if   	1
.LBB27_58:
	.loc	14 0 12 is_stmt 0
	end_block
.Ltmp4481:
	.loc	14 3327 24 is_stmt 1
	local.get	4
	local.get	15
	call	_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_
.Ltmp4482:
	.loc	14 3337 27
	local.get	0
	i32.load	804
	local.set	17
.Ltmp4483:
	.loc	14 3336 27
	local.get	0
	i32.load	800
	local.set	18
.Ltmp4484:
	.loc	14 3333 21
	local.get	0
	i32.load8_u	785
	local.set	9
.Ltmp4485:
	.loc	14 3332 19
	local.get	0
	i32.load8_u	784
	local.set	8
.Ltmp4486:
	.loc	14 3335 16
	local.get	0
	i32.load	920
	local.set	19
.Ltmp4487:
	.loc	14 3334 16
	local.get	0
	i32.load	916
	local.set	12
	local.get	4
	i32.const	1136
	i32.add 
	i32.const	0
	i32.const	1024
	memory.fill	0
.Ltmp4488:
	.loc	14 3341 32
	local.get	4
	i32.const	2160
	i32.add 
	local.get	15
	local.get	12
	local.get	19
	call	_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E3newB5_
.Ltmp4489:
	.loc	41 446 20
	block   	
	local.get	3
	br_if   	0
.Ltmp4490:
	.loc	14 3411 31
	local.get	4
	i32.load	2176
	local.set	6
	br      	2
.LBB27_60:
	.loc	14 0 31 is_stmt 0
	end_block
	local.get	3
	i32.const	5
.Ltmp4491:
	.loc	13 3756 21 is_stmt 1
	i32.shr_u
	local.get	3
	i32.const	31
.Ltmp4492:
	.loc	13 3757 21
	i32.and 
	i32.const	0
.Ltmp4493:
	.loc	13 3758 16
	i32.ne  
	i32.add 
	local.set	20
.Ltmp4494:
	.loc	14 3333 21
	v128.const	-1, -1, -1, -1
	local.tee	21
	v128.const	0, 0, 0, 0
	local.tee	22
	local.get	9
	i32.const	1
	i32.and 
	v128.select
	local.set	23
	local.get	21
	local.get	22
	local.get	8
	i32.const	1
.Ltmp4495:
	.loc	14 3332 19
	i32.and 
	v128.select
	local.set	24
	local.get	0
	v128.load	768
	local.set	25
	local.get	0
	v128.load	752
	local.set	26
	local.get	0
	v128.load	736
	local.set	27
	local.get	0
	v128.load	720
	local.set	28
	local.get	0
	v128.load	704
	local.set	29
	local.get	0
	v128.load	688
	local.set	30
	local.get	0
	v128.load	672
	local.set	31
	local.get	0
	v128.load	656
	local.set	32
	local.get	0
	v128.load	640
	local.set	33
	local.get	0
	v128.load	624
	local.set	34
	local.get	0
	v128.load	608
	local.set	35
	local.get	0
	v128.load	592
	local.set	36
	local.get	0
	v128.load	576
	local.set	37
	local.get	0
	v128.load	560
	local.set	38
	local.get	0
	v128.load	544
	local.set	39
	local.get	0
	v128.load	528
	local.set	40
	local.get	0
	v128.load	512
	local.set	41
	local.get	0
	v128.load	496
	local.set	42
	local.get	0
	v128.load	480
	local.set	43
	local.get	0
	v128.load	464
	local.set	44
	local.get	0
	v128.load	448
	local.set	45
	local.get	0
	v128.load	432
	local.set	46
	local.get	0
	v128.load	416
	local.set	47
	local.get	0
	v128.load	400
	local.set	48
	local.get	0
	v128.load	384
	local.set	49
	local.get	0
	v128.load	368
	local.set	50
	local.get	0
	v128.load	352
	local.set	51
	local.get	0
	v128.load	336
	local.set	52
	local.get	0
	v128.load	320
	local.set	53
	local.get	0
	v128.load	304
	local.set	54
	local.get	0
	v128.load	288
	local.set	55
	local.get	0
	v128.load	272
	local.set	56
	local.get	0
	v128.load	256
	local.set	57
	local.get	0
	v128.load	240
	local.set	58
	local.get	0
	v128.load	224
	local.set	59
	local.get	0
	v128.load	208
	local.set	60
	local.get	0
	v128.load	192
	local.set	61
	local.get	0
	v128.load	176
	local.set	62
	local.get	0
	v128.load	160
	local.set	63
	local.get	0
	v128.load	144
	local.set	64
	local.get	0
	v128.load	128
	local.set	65
	local.get	0
	v128.load	112
	local.set	66
	local.get	0
	v128.load	96
	local.set	67
	local.get	0
	v128.load	80
	local.set	68
	local.get	0
	v128.load	64
	local.set	69
	local.get	0
	v128.load	48
	local.set	70
	local.get	0
	v128.load	32
	local.set	71
	local.get	0
	v128.load	16
	local.set	72
	local.get	4
	i32.load	2208
	local.set	73
	local.get	4
	i32.load	2212
	local.set	74
	local.get	4
	i32.load	2204
	local.set	75
	local.get	4
	i32.load	2200
	local.set	76
	local.get	4
	i32.load	2180
	local.set	77
	local.get	4
	i32.load	2196
	local.set	7
	local.get	4
	i32.load	2192
	local.set	11
	local.get	4
	i32.load	2188
	local.set	78
	local.get	4
	i32.load	2184
	local.set	79
	local.get	4
	i32.load	2176
	local.set	6
	local.get	1
	local.set	80
	local.get	3
	local.set	81
	i32.const	0
	local.set	82
.LBB27_61:
.Ltmp4496:
	.loc	14 3344 55
	loop    	
	block   	
	block   	
	block   	
	block   	
	block   	
	block   	
	block   	
	block   	
	block   	
	block   	
	block   	
	local.get	3
	local.get	82
	i32.sub 
	local.tee	9
	i32.const	32
	local.get	9
	i32.const	32
.Ltmp4497:
	.loc	9 1077 12
	i32.lt_u
	i32.select
.Ltmp4498:
	.loc	14 3349 39
	local.tee	83
	local.get	82
	i32.add 
	i32.const	2
	i32.shl 
	local.tee	9
	local.get	82
	i32.const	2
.Ltmp4499:
	.loc	14 3345 31
	i32.shl 
.Ltmp4500:
	.loc	13 1050 16
	local.tee	8
	i32.lt_u
	br_if   	0
	local.get	9
	local.get	2
	i32.gt_u
	br_if   	0
.Ltmp4501:
	.loc	13 0 16 is_stmt 0
	local.get	81
	i32.const	32
	local.get	81
	i32.const	32
	i32.lt_u
	i32.select
	local.set	84
.Ltmp4502:
	.loc	14 1759 23 is_stmt 1
	local.get	4
	v128.load	160
	local.set	85
	local.get	4
	v128.load	144
	local.set	86
	local.get	4
	v128.load	128
	local.set	87
	local.get	4
	v128.load	112
	local.set	88
	local.get	4
	v128.load	96
	local.set	21
	local.get	4
	v128.load	80
	local.set	89
	local.get	4
	v128.load	64
	local.set	90
	local.get	4
	v128.load	48
	local.set	91
	local.get	4
	v128.load	32
	local.set	92
	local.get	4
	v128.load	16
	local.set	22
	local.get	4
	v128.load	0
	local.set	93
.Ltmp4503:
	.loc	33 304 12
	block   	
	block   	
	local.get	3
	local.get	82
	i32.eq  
	local.tee	13
	i32.eqz
	br_if   	0
.Ltmp4504:
	.loc	14 0 0 is_stmt 0
	local.get	4
	v128.load	176
	local.set	94
.Ltmp4505:
	.loc	33 304 12
	br      	1
.Ltmp4506:
.LBB27_65:
	.loc	33 0 12
	end_block
	local.get	84
	i32.const	1
	local.get	84
	i32.const	1
	i32.gt_u
	i32.select
	local.set	10
	local.get	4
	i32.const	1136
	i32.add 
	local.set	9
	local.get	80
	local.set	8
	local.get	85
	local.set	95
	local.get	86
	local.set	96
	local.get	87
	local.set	97
	local.get	88
	local.set	98
	local.get	21
	local.set	99
	local.get	89
	local.set	100
	local.get	90
	local.set	101
	local.get	91
	local.set	102
	local.get	92
	local.set	103
	local.get	22
	local.set	104
.LBB27_66:
	loop    	
	local.get	93
	local.set	22
.Ltmp4507:
	.loc	1 551 14 is_stmt 1
	local.get	9
	local.get	69
	local.get	8
	v128.load	0:p2align=2
.Ltmp4508:
	.loc	42 3867 14
	local.tee	93
	f32x4.mul
.Ltmp4509:
	.loc	42 3845 14
	v128.const	0x0p0, 0x0p0, 0x0p0, 0x0p0
.Ltmp4510:
	.loc	42 3845 14 is_stmt 0
	local.tee	105
	f32x4.add
.Ltmp4511:
	.loc	42 3867 14 is_stmt 1
	local.get	65
	local.get	22
	f32x4.mul
.Ltmp4512:
	.loc	42 3845 14
	f32x4.add
	local.get	61
	local.get	104
.Ltmp4513:
	.loc	42 3867 14
	local.tee	92
	f32x4.mul
.Ltmp4514:
	.loc	42 3845 14
	f32x4.add
	local.get	57
	local.get	103
.Ltmp4515:
	.loc	42 3867 14
	local.tee	91
	f32x4.mul
.Ltmp4516:
	.loc	42 3845 14
	f32x4.add
	local.get	53
	local.get	102
.Ltmp4517:
	.loc	42 3867 14
	local.tee	90
	f32x4.mul
.Ltmp4518:
	.loc	42 3845 14
	f32x4.add
	local.get	49
	local.get	101
.Ltmp4519:
	.loc	42 3867 14
	local.tee	89
	f32x4.mul
.Ltmp4520:
	.loc	42 3845 14
	f32x4.add
	local.get	45
	local.get	100
.Ltmp4521:
	.loc	42 3867 14
	local.tee	21
	f32x4.mul
.Ltmp4522:
	.loc	42 3845 14
	f32x4.add
	local.get	41
	local.get	99
.Ltmp4523:
	.loc	42 3867 14
	local.tee	88
	f32x4.mul
.Ltmp4524:
	.loc	42 3845 14
	f32x4.add
	local.get	37
	local.get	98
.Ltmp4525:
	.loc	42 3867 14
	local.tee	87
	f32x4.mul
.Ltmp4526:
	.loc	42 3845 14
	f32x4.add
	local.get	33
	local.get	97
.Ltmp4527:
	.loc	42 3867 14
	local.tee	86
	f32x4.mul
.Ltmp4528:
	.loc	42 3845 14
	f32x4.add
	local.get	29
	local.get	96
.Ltmp4529:
	.loc	42 3867 14
	local.tee	85
	f32x4.mul
.Ltmp4530:
	.loc	42 3845 14
	f32x4.add
	local.get	25
	local.get	95
.Ltmp4531:
	.loc	42 3867 14
	local.tee	94
	f32x4.mul
.Ltmp4532:
	.loc	42 3845 14
	f32x4.add
.Ltmp4533:
	.loc	42 3812 14
	f32x4.abs
.Ltmp4534:
	.loc	42 3867 14
	local.get	70
	local.get	93
	f32x4.mul
.Ltmp4535:
	.loc	42 3845 14
	local.get	105
	f32x4.add
.Ltmp4536:
	.loc	42 3867 14
	local.get	66
	local.get	22
	f32x4.mul
.Ltmp4537:
	.loc	42 3845 14
	f32x4.add
.Ltmp4538:
	.loc	42 3867 14
	local.get	62
	local.get	92
	f32x4.mul
.Ltmp4539:
	.loc	42 3845 14
	f32x4.add
.Ltmp4540:
	.loc	42 3867 14
	local.get	58
	local.get	91
	f32x4.mul
.Ltmp4541:
	.loc	42 3845 14
	f32x4.add
.Ltmp4542:
	.loc	42 3867 14
	local.get	54
	local.get	90
	f32x4.mul
.Ltmp4543:
	.loc	42 3845 14
	f32x4.add
.Ltmp4544:
	.loc	42 3867 14
	local.get	50
	local.get	89
	f32x4.mul
.Ltmp4545:
	.loc	42 3845 14
	f32x4.add
.Ltmp4546:
	.loc	42 3867 14
	local.get	46
	local.get	21
	f32x4.mul
.Ltmp4547:
	.loc	42 3845 14
	f32x4.add
.Ltmp4548:
	.loc	42 3867 14
	local.get	42
	local.get	88
	f32x4.mul
.Ltmp4549:
	.loc	42 3845 14
	f32x4.add
.Ltmp4550:
	.loc	42 3867 14
	local.get	38
	local.get	87
	f32x4.mul
.Ltmp4551:
	.loc	42 3845 14
	f32x4.add
.Ltmp4552:
	.loc	42 3867 14
	local.get	34
	local.get	86
	f32x4.mul
.Ltmp4553:
	.loc	42 3845 14
	f32x4.add
.Ltmp4554:
	.loc	42 3867 14
	local.get	30
	local.get	85
	f32x4.mul
.Ltmp4555:
	.loc	42 3845 14
	f32x4.add
.Ltmp4556:
	.loc	42 3867 14
	local.get	26
	local.get	94
	f32x4.mul
.Ltmp4557:
	.loc	42 3845 14
	f32x4.add
.Ltmp4558:
	.loc	42 3812 14
	f32x4.abs
.Ltmp4559:
	.loc	42 3867 14
	local.get	71
	local.get	93
	f32x4.mul
.Ltmp4560:
	.loc	42 3845 14
	local.get	105
	f32x4.add
.Ltmp4561:
	.loc	42 3867 14
	local.get	67
	local.get	22
	f32x4.mul
.Ltmp4562:
	.loc	42 3845 14
	f32x4.add
.Ltmp4563:
	.loc	42 3867 14
	local.get	63
	local.get	92
	f32x4.mul
.Ltmp4564:
	.loc	42 3845 14
	f32x4.add
.Ltmp4565:
	.loc	42 3867 14
	local.get	59
	local.get	91
	f32x4.mul
.Ltmp4566:
	.loc	42 3845 14
	f32x4.add
.Ltmp4567:
	.loc	42 3867 14
	local.get	55
	local.get	90
	f32x4.mul
.Ltmp4568:
	.loc	42 3845 14
	f32x4.add
.Ltmp4569:
	.loc	42 3867 14
	local.get	51
	local.get	89
	f32x4.mul
.Ltmp4570:
	.loc	42 3845 14
	f32x4.add
.Ltmp4571:
	.loc	42 3867 14
	local.get	47
	local.get	21
	f32x4.mul
.Ltmp4572:
	.loc	42 3845 14
	f32x4.add
.Ltmp4573:
	.loc	42 3867 14
	local.get	43
	local.get	88
	f32x4.mul
.Ltmp4574:
	.loc	42 3845 14
	f32x4.add
.Ltmp4575:
	.loc	42 3867 14
	local.get	39
	local.get	87
	f32x4.mul
.Ltmp4576:
	.loc	42 3845 14
	f32x4.add
.Ltmp4577:
	.loc	42 3867 14
	local.get	35
	local.get	86
	f32x4.mul
.Ltmp4578:
	.loc	42 3845 14
	f32x4.add
.Ltmp4579:
	.loc	42 3867 14
	local.get	31
	local.get	85
	f32x4.mul
.Ltmp4580:
	.loc	42 3845 14
	f32x4.add
.Ltmp4581:
	.loc	42 3867 14
	local.get	27
	local.get	94
	f32x4.mul
.Ltmp4582:
	.loc	42 3845 14
	f32x4.add
.Ltmp4583:
	.loc	42 3812 14
	f32x4.abs
.Ltmp4584:
	.loc	42 3867 14
	local.get	72
	local.get	93
	f32x4.mul
.Ltmp4585:
	.loc	42 3845 14
	local.get	105
	f32x4.add
.Ltmp4586:
	.loc	42 3867 14
	local.get	68
	local.get	22
	f32x4.mul
.Ltmp4587:
	.loc	42 3845 14
	f32x4.add
.Ltmp4588:
	.loc	42 3867 14
	local.get	64
	local.get	92
	f32x4.mul
.Ltmp4589:
	.loc	42 3845 14
	f32x4.add
.Ltmp4590:
	.loc	42 3867 14
	local.get	60
	local.get	91
	f32x4.mul
.Ltmp4591:
	.loc	42 3845 14
	f32x4.add
.Ltmp4592:
	.loc	42 3867 14
	local.get	56
	local.get	90
	f32x4.mul
.Ltmp4593:
	.loc	42 3845 14
	f32x4.add
.Ltmp4594:
	.loc	42 3867 14
	local.get	52
	local.get	89
	f32x4.mul
.Ltmp4595:
	.loc	42 3845 14
	f32x4.add
.Ltmp4596:
	.loc	42 3867 14
	local.get	48
	local.get	21
	f32x4.mul
.Ltmp4597:
	.loc	42 3845 14
	f32x4.add
.Ltmp4598:
	.loc	42 3867 14
	local.get	44
	local.get	88
	f32x4.mul
.Ltmp4599:
	.loc	42 3845 14
	f32x4.add
.Ltmp4600:
	.loc	42 3867 14
	local.get	40
	local.get	87
	f32x4.mul
.Ltmp4601:
	.loc	42 3845 14
	f32x4.add
.Ltmp4602:
	.loc	42 3867 14
	local.get	36
	local.get	86
	f32x4.mul
.Ltmp4603:
	.loc	42 3845 14
	f32x4.add
.Ltmp4604:
	.loc	42 3867 14
	local.get	32
	local.get	85
	f32x4.mul
.Ltmp4605:
	.loc	42 3845 14
	f32x4.add
.Ltmp4606:
	.loc	42 3867 14
	local.get	28
	local.get	94
	f32x4.mul
.Ltmp4607:
	.loc	42 3845 14
	f32x4.add
.Ltmp4608:
	.loc	42 3812 14
	f32x4.abs
.Ltmp4609:
	.loc	42 3812 14 is_stmt 0
	local.get	21
	f32x4.abs
.Ltmp4610:
	.loc	42 3928 9 is_stmt 1
	f32x4.pmax
	f32x4.pmax
	f32x4.pmax
	f32x4.pmax
.Ltmp4611:
	.loc	1 551 14
	v128.store	0:p2align=2
	local.get	9
	i32.const	16
.Ltmp4612:
	.loc	33 304 12
	i32.add 
	local.set	9
	local.get	8
	i32.const	16
	i32.add 
	local.set	8
	local.get	85
	local.set	95
	local.get	86
	local.set	96
	local.get	87
	local.set	97
	local.get	88
	local.set	98
	local.get	21
	local.set	99
	local.get	89
	local.set	100
	local.get	90
	local.set	101
	local.get	91
	local.set	102
	local.get	92
	local.set	103
	local.get	22
	local.set	104
	local.get	10
	i32.const	-1
	i32.add 
	local.tee	10
	br_if   	0
.LBB27_67:
	end_loop
	end_block
.Ltmp4613:
	.loc	14 1764 5
	local.get	4
	local.get	94
	v128.store	176
	local.get	4
	local.get	85
	v128.store	160
	local.get	4
	local.get	86
	v128.store	144
	local.get	4
	local.get	87
	v128.store	128
	local.get	4
	local.get	88
	v128.store	112
	local.get	4
	local.get	21
	v128.store	96
	local.get	4
	local.get	89
	v128.store	80
	local.get	4
	local.get	90
	v128.store	64
	local.get	4
	local.get	91
	v128.store	48
	local.get	4
	local.get	92
	v128.store	32
	local.get	4
	local.get	22
	v128.store	16
	local.get	4
	local.get	93
	v128.store	0
.Ltmp4614:
	.loc	14 3355 19
	local.get	13
	br_if   	10
	.loc	14 0 19 is_stmt 0
	i32.const	0
	local.set	106
	local.get	4
	v128.load	336
	local.set	99
	local.get	4
	v128.load	304
	local.set	100
	local.get	4
	v128.load	240
	local.set	101
	local.get	4
	v128.load	352
	local.set	98
	local.get	4
	v128.load	272
	local.set	96
	local.get	4
	v128.load	208
	local.set	97
.LBB27_69:
.Ltmp4615:
	.loc	14 1575 16 is_stmt 1
	loop    	
	local.get	0
	i32.load	916
.Ltmp4616:
	.loc	14 1580 33
	local.tee	9
	local.get	78
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp4617:
	.loc	14 1148 8
	local.get	9
	local.get	8
	local.get	9
	i32.lt_u
	i32.select
	i32.sub 
.Ltmp4618:
	.loc	14 1588 14
	local.tee	107
	i32.sub 
.Ltmp4619:
	.loc	14 1578 28
	local.tee	8
	local.get	9
	local.get	79
	local.get	17
	i32.add 
	local.tee	10
	i32.const	0
.Ltmp4620:
	.loc	14 1148 8
	local.get	9
	local.get	10
	local.get	9
	i32.lt_u
	i32.select
	i32.sub 
.Ltmp4621:
	.loc	14 1586 14
	local.tee	108
	i32.sub 
	local.tee	10
	local.get	9
	local.get	17
	i32.const	1
.Ltmp4622:
	.loc	14 1577 25
	i32.add 
	local.tee	13
	i32.const	0
.Ltmp4623:
	.loc	14 1148 8
	local.get	9
	local.get	13
	local.get	9
	i32.lt_u
	i32.select
	i32.sub 
.Ltmp4624:
	.loc	14 1585 14
	local.tee	109
	i32.sub 
.Ltmp4625:
	.loc	14 1576 16
	local.tee	13
	local.get	0
	i32.load	920
.Ltmp4626:
	.loc	14 1584 14
	local.get	18
	i32.sub 
	.loc	14 1583 14
	local.tee	110
	local.get	9
	local.get	17
	i32.sub 
.Ltmp4627:
	.loc	14 3363 21
	local.tee	9
	local.get	83
	local.get	106
	i32.sub 
.Ltmp4628:
	.loc	9 1077 12
	local.tee	111
	local.get	9
	local.get	111
	i32.lt_u
	i32.select
.Ltmp4629:
	.loc	9 1077 12 is_stmt 0
	local.tee	111
	local.get	110
	local.get	111
	i32.lt_u
	i32.select
.Ltmp4630:
	.loc	9 1077 12
	local.tee	111
	local.get	13
	local.get	111
	i32.lt_u
	i32.select
.Ltmp4631:
	.loc	9 1077 12
	local.tee	111
	local.get	10
	local.get	111
	i32.lt_u
	i32.select
.Ltmp4632:
	.loc	9 1077 12
	local.tee	111
	local.get	8
	local.get	111
	i32.lt_u
	i32.select
.Ltmp4633:
	.loc	14 3369 28 is_stmt 1
	local.tee	112
	local.get	106
	local.get	82
	i32.add 
.Ltmp4634:
	.loc	14 3371 55
	local.tee	113
	i32.add 
	i32.const	2
	i32.shl 
	local.tee	111
	local.get	113
	i32.const	2
.Ltmp4635:
	.loc	14 3369 28
	i32.shl 
.Ltmp4636:
	.loc	13 1050 16
	local.tee	113
	i32.lt_u
	br_if   	2
	local.get	111
	local.get	2
	i32.gt_u
	br_if   	2
.Ltmp4637:
	.loc	33 304 12
	block   	
	local.get	112
	i32.eqz
	br_if   	0
.Ltmp4638:
	.loc	33 0 12 is_stmt 0
	local.get	1
	local.get	113
	i32.const	2
	i32.shl 
	i32.add 
	local.set	114
.Ltmp4639:
	local.get	4
	i32.const	1136
	i32.add 
	local.get	106
	i32.const	4
	i32.shl 
	i32.add 
	local.set	115
.Ltmp4640:
	.loc	33 304 12
	local.get	10
	local.get	8
	local.get	10
	local.get	8
	i32.lt_u
	i32.select
	local.tee	8
	local.get	13
	local.get	8
	local.get	13
	i32.lt_u
	i32.select
	local.tee	8
	local.get	9
	local.get	8
	local.get	9
	i32.lt_u
	i32.select
	local.tee	9
	local.get	110
	local.get	9
	local.get	110
	i32.lt_u
	i32.select
	local.tee	9
	local.get	84
	local.get	106
	i32.sub 
	local.tee	8
	local.get	9
	local.get	8
	i32.lt_u
	i32.select
	i32.const	1073741823
	i32.and 
	local.set	116
	i32.const	0
	local.set	13
.Ltmp4641:
	.loc	14 853 61 is_stmt 1
	local.get	4
	v128.load	288
	local.set	85
	.loc	14 853 44 is_stmt 0
	local.get	4
	v128.load	256
	local.set	87
.Ltmp4642:
	.loc	14 853 61
	local.get	4
	v128.load	224
	local.set	86
	.loc	14 853 44
	local.get	4
	v128.load	192
	local.set	22
	local.get	99
	local.set	91
	local.get	100
	local.set	88
	local.get	101
	local.set	89
.Ltmp4643:
.LBB27_73:
	.loc	42 3845 14 is_stmt 1
	loop    	
	local.get	87
	local.get	85
	f32x4.add
.Ltmp4644:
	.loc	42 3856 14
	local.get	96
	local.get	88
	v128.const	-0x1p0, -0x1p0, -0x1p0, -0x1p0
.Ltmp4645:
	.loc	42 3856 14 is_stmt 0
	local.tee	21
	f32x4.add
.Ltmp4646:
	.loc	42 3929 13 is_stmt 1
	local.tee	92
	v128.const	0x0p0, 0x0p0, 0x0p0, 0x0p0
.Ltmp4647:
	.loc	42 3929 13 is_stmt 0
	local.tee	90
	f32x4.gt
.Ltmp4648:
	.loc	42 2188 14 is_stmt 1
	local.tee	88
	v128.bitselect
	local.set	87
.Ltmp4649:
	.loc	42 3845 14
	local.get	22
	local.get	86
	f32x4.add
.Ltmp4650:
	.loc	42 3856 14
	local.get	97
	local.get	89
	local.get	21
	f32x4.add
.Ltmp4651:
	.loc	42 3929 13
	local.tee	93
	local.get	90
	f32x4.gt
.Ltmp4652:
	.loc	42 2188 14
	local.tee	89
	v128.bitselect
	local.set	22
.Ltmp4653:
	.loc	42 2188 14 is_stmt 0
	local.get	85
	v128.const	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
.Ltmp4654:
	.loc	42 2188 14
	local.tee	21
	local.get	88
	v128.bitselect
	local.set	85
.Ltmp4655:
	.loc	42 2188 14
	local.get	86
	local.get	21
	local.get	89
	v128.bitselect
	local.set	86
.Ltmp4656:
	.loc	14 1502 26 is_stmt 1
	local.get	13
	local.get	17
	i32.add 
	i32.const	2
.Ltmp4657:
	.loc	14 1137 16
	i32.shl 
	local.tee	110
	i32.const	4
.Ltmp4658:
	.loc	14 1138 33
	i32.add 
	local.set	111
	local.get	110
	i32.const	3
.Ltmp4659:
	.loc	13 1050 16
	i32.or  
	local.get	7
	i32.ge_u
	br_if   	5
.Ltmp4660:
	.loc	13 0 16 is_stmt 0
	local.get	114
	local.get	13
	i32.const	4
	i32.shl 
	i32.add 
	local.tee	117
	v128.load	0:p2align=2
	local.set	94
	local.get	11
	local.get	110
	i32.const	2
.Ltmp4661:
	.loc	38 101 24 is_stmt 1
	i32.shl 
	local.tee	113
	i32.add 
	local.get	22
	local.get	115
	local.get	13
	i32.const	2
.Ltmp4662:
	.loc	14 0 0 is_stmt 0
	i32.shl 
	i32.const	2
.Ltmp4663:
	.loc	35 863 18 is_stmt 1
	i32.shl 
	i32.add 
.Ltmp4664:
	.loc	1 551 14
	v128.load	0:p2align=2
.Ltmp4665:
	.loc	42 2188 14
	local.tee	21
	local.get	21
	local.get	24
	v128.bitselect
.Ltmp4666:
	.loc	42 3878 14
	local.tee	21
	f32x4.div
.Ltmp4667:
	.loc	42 2188 14
	v128.const	0, 0, 128, 63, 0, 0, 128, 63, 0, 0, 128, 63, 0, 0, 128, 63
.Ltmp4668:
	.loc	42 2005 14
	local.get	21
	local.get	22
	f32x4.gt
.Ltmp4669:
	.loc	42 2188 14
	v128.bitselect
.Ltmp4670:
	.loc	1 551 14
	v128.store	0:p2align=2
.Ltmp4671:
	.loc	14 0 0 is_stmt 0
	local.get	13
	local.get	108
	i32.add 
	local.tee	9
	i32.const	2
.Ltmp4672:
	.loc	14 1130 16 is_stmt 1
	i32.shl 
	local.tee	8
	i32.const	3
.Ltmp4673:
	.loc	13 1050 16
	i32.or  
	local.get	7
	i32.ge_u
	br_if   	6
.Ltmp4674:
	.loc	13 0 16 is_stmt 0
	local.get	4
	local.get	11
	local.get	8
	i32.const	2
.Ltmp4675:
	.loc	38 89 24 is_stmt 1
	i32.shl 
	i32.add 
.Ltmp4676:
	.loc	1 551 14
	v128.load	0:p2align=2
.Ltmp4677:
	.loc	14 1206 22
	local.tee	21
	local.get	4
	v128.load	2160
	f32x4.pmin
	local.get	21
	local.get	6
	v128.select
.Ltmp4678:
	.loc	14 1211 5
	local.tee	90
	v128.store	2160
	block   	
	block   	
	local.get	6
	i32.const	1
	.loc	14 1212 20
	i32.add 
	local.tee	6
	local.get	77
	i32.eq  
.Ltmp4679:
	.loc	14 1213 22
	br_if   	0
.Ltmp4680:
	.loc	14 0 0 is_stmt 0
	local.get	13
	local.get	109
	i32.add 
	i32.const	2
.Ltmp4681:
	.loc	14 1130 16 is_stmt 1
	i32.shl 
	local.tee	9
	i32.const	3
.Ltmp4682:
	.loc	13 1050 16
	i32.or  
	local.get	7
	i32.ge_u
	br_if   	9
.Ltmp4683:
	.loc	13 0 16 is_stmt 0
	local.get	90
	local.get	11
	local.get	9
	i32.const	2
.Ltmp4684:
	.loc	38 89 24 is_stmt 1
	i32.shl 
	i32.add 
.Ltmp4685:
	.loc	1 551 14
	v128.load	0:p2align=2
.Ltmp4686:
	.loc	42 3911 9
	f32x4.pmin
	local.set	90
.Ltmp4687:
	.loc	14 1218 5
	br      	1
.LBB27_78:
	.loc	14 0 5 is_stmt 0
	end_block
	i32.const	0
	local.set	6
.Ltmp4688:
	.loc	10 900 12 is_stmt 1
	local.get	77
	i32.eqz
	br_if   	0
.Ltmp4689:
	.loc	10 0 12 is_stmt 0
	local.get	77
	local.set	8
.LBB27_80:
	loop    	
	local.get	9
	i32.const	2
.Ltmp4690:
	.loc	14 1130 16 is_stmt 1
	i32.shl 
	local.tee	10
	i32.const	3
.Ltmp4691:
	.loc	13 1050 16
	i32.or  
	local.get	7
	i32.ge_u
	br_if   	10
.Ltmp4692:
	.loc	13 0 16 is_stmt 0
	local.get	11
	local.get	10
	i32.const	2
.Ltmp4693:
	.loc	38 89 24 is_stmt 1
	i32.shl 
	i32.add 
.Ltmp4694:
	.loc	1 551 14
	local.tee	10
	local.get	10
	v128.load	0:p2align=2
.Ltmp4695:
	.loc	42 3911 9
	local.get	21
	f32x4.pmin
.Ltmp4696:
	.loc	1 551 14
	local.tee	21
	v128.store	0:p2align=2
.Ltmp4697:
	.loc	14 1224 16
	local.get	9
	local.get	12
	local.get	9
	i32.select
	i32.const	-1
	.loc	14 1227 13
	i32.add 
	local.set	9
	local.get	8
	i32.const	-1
.Ltmp4698:
	.loc	9 1916 50
	i32.add 
.Ltmp4699:
	.loc	10 900 12
	local.tee	8
	br_if   	0
.LBB27_82:
	.loc	10 900 12
	end_loop
	end_block
.Ltmp4700:
	.loc	14 0 0 is_stmt 0
	local.get	13
	local.get	107
	i32.add 
	i32.const	2
.Ltmp4701:
	.loc	14 1130 16 is_stmt 1
	i32.shl 
	local.tee	9
	i32.const	3
.Ltmp4702:
	.loc	13 1050 16
	i32.or  
	local.get	75
	i32.ge_u
	br_if   	9
.Ltmp4703:
	.loc	13 0 16 is_stmt 0
	local.get	111
	local.get	75
	i32.gt_u
.Ltmp4704:
	.loc	13 1050 16
	br_if   	10
.Ltmp4705:
	.loc	13 0 16
	local.get	76
	local.get	9
	i32.const	2
	i32.shl 
	i32.add 
	v128.load	0:p2align=2
	local.set	21
.Ltmp4706:
	.loc	38 101 24 is_stmt 1
	local.get	76
	local.get	113
	i32.add 
.Ltmp4707:
	.loc	14 0 0 is_stmt 0
	local.get	90
	v128.const	0x1p14, 0x1p14, 0x1p14, 0x1p14
	f32x4.mul
	f32x4.floor
	v128.const	0x1p-14, 0x1p-14, 0x1p-14, 0x1p-14
	f32x4.mul
.Ltmp4708:
	.loc	1 551 14 is_stmt 1
	local.tee	90
	v128.store	0:p2align=2
.Ltmp4709:
	.loc	14 1659 43
	local.get	4
	local.get	4
	v128.load	320
.Ltmp4710:
	.loc	42 3856 14
	local.tee	105
	v128.const	0x1p0, 0x1p0, 0x1p0, 0x1p0
.Ltmp4711:
	.loc	14 0 0 is_stmt 0
	local.tee	95
	local.get	90
	local.get	91
	f32x4.add
.Ltmp4712:
	local.get	21
	f32x4.sub
.Ltmp4713:
	.loc	42 3878 14 is_stmt 1
	local.tee	91
	local.get	98
	f32x4.div
.Ltmp4714:
	.loc	42 3856 14
	f32x4.sub
.Ltmp4715:
	.loc	42 3856 14 is_stmt 0
	local.tee	21
	local.get	105
	f32x4.sub
.Ltmp4716:
	.loc	42 3867 14 is_stmt 1
	local.get	87
	f32x4.mul
.Ltmp4717:
	.loc	42 3845 14
	f32x4.add
.Ltmp4718:
	.loc	42 3928 9
	local.get	21
	f32x4.pmax
.Ltmp4719:
	.loc	42 3812 14
	local.tee	21
	local.get	21
	f32x4.abs
.Ltmp4720:
	.loc	42 1991 14
	v128.const	0x1.79ca1p-67, 0x1.79ca1p-67, 0x1.79ca1p-67, 0x1.79ca1p-67
	f32x4.lt
.Ltmp4721:
	.loc	44 481 22
	v128.andnot
.Ltmp4722:
	.loc	14 1660 5
	local.tee	90
	v128.store	320
.Ltmp4723:
	.loc	14 0 0 is_stmt 0
	local.get	13
	local.get	18
	i32.add 
	i32.const	2
.Ltmp4724:
	.loc	14 1130 16 is_stmt 1
	i32.shl 
	local.tee	9
	i32.const	3
.Ltmp4725:
	.loc	13 1050 16
	i32.or  
	local.get	74
	i32.ge_u
	br_if   	11
.Ltmp4726:
	.loc	14 0 0 is_stmt 0
	local.get	88
	local.get	92
	v128.and
	local.set	88
	local.get	89
	local.get	93
	v128.and
	local.set	89
	local.get	73
	local.get	9
	i32.const	2
.Ltmp4727:
	.loc	38 89 24 is_stmt 1
	i32.shl 
	i32.add 
.Ltmp4728:
	.loc	1 551 14
	local.tee	9
	v128.load	0:p2align=2
	local.set	21
.Ltmp4729:
	.loc	1 551 14 is_stmt 0
	local.get	9
	local.get	94
	v128.store	0:p2align=2
.Ltmp4730:
	.loc	42 3856 14 is_stmt 1
	local.get	117
	local.get	21
	local.get	21
	local.get	95
	local.get	90
	f32x4.sub
.Ltmp4731:
	.loc	42 3867 14
	f32x4.mul
.Ltmp4732:
	.loc	42 2188 14
	local.get	23
	v128.bitselect
.Ltmp4733:
	.loc	1 551 14
	v128.store	0:p2align=2
	local.get	13
	i32.const	1
.Ltmp4734:
	.loc	14 0 0 is_stmt 0
	i32.add 
.Ltmp4735:
	.loc	33 304 12 is_stmt 1
	local.tee	13
	local.get	116
	i32.ne  
	br_if   	0
	end_loop
.Ltmp4736:
	.loc	14 854 9
	local.get	4
	local.get	85
	v128.store	288
	.loc	14 853 9
	local.get	4
	local.get	87
	v128.store	256
.Ltmp4737:
	.loc	14 854 9
	local.get	4
	local.get	86
	v128.store	224
	.loc	14 853 9
	local.get	4
	local.get	22
	v128.store	192
	local.get	91
	local.set	99
	local.get	88
	local.set	100
	local.get	89
	local.set	101
.Ltmp4738:
.LBB27_87:
	.loc	14 0 9 is_stmt 0
	end_block
	.loc	14 3407 39 is_stmt 1
	local.get	112
	local.get	18
	i32.add 
	local.tee	9
	i32.const	0
.Ltmp4739:
	.loc	14 1148 8
	local.get	19
	local.get	9
	local.get	19
	i32.lt_u
	i32.select
	i32.sub 
	local.set	18
.Ltmp4740:
	.loc	14 3406 39
	local.get	112
	local.get	17
	i32.add 
	local.tee	9
	i32.const	0
.Ltmp4741:
	.loc	14 1148 8
	local.get	12
	local.get	9
	local.get	12
	i32.lt_u
	i32.select
	i32.sub 
	local.set	17
.Ltmp4742:
	.loc	14 0 0 is_stmt 0
	local.get	112
	local.get	106
	i32.add 
.Ltmp4743:
	.loc	14 3355 19 is_stmt 1
	local.tee	106
	local.get	83
	i32.ge_u
	br_if   	10
	br      	0
.Ltmp4744:
.LBB27_88:
	.loc	13 1050 16
	end_loop
	end_block
.Ltmp4745:
	.loc	38 443 13
	local.get	8
	local.get	9
	local.get	2
	i32.const	.Lalloc_bac0677bfb785f04669fb3d80741c988
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp4746:
.LBB27_89:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	99
	v128.store	336
	local.get	4
	local.get	100
	v128.store	304
	local.get	4
	local.get	101
	v128.store	240
.Ltmp4747:
	.loc	38 456 13 is_stmt 1
	local.get	113
	local.get	111
	local.get	2
	i32.const	.Lalloc_1eb3474cf2d0fe655b193b0c53e74d06
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp4748:
.LBB27_90:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	99
	v128.store	336
	local.get	4
	local.get	100
	v128.store	304
	local.get	4
	local.get	101
	v128.store	240
.Ltmp4749:
	.loc	14 854 9 is_stmt 1
	local.get	4
	local.get	85
	v128.store	288
	.loc	14 853 9
	local.get	4
	local.get	87
	v128.store	256
.Ltmp4750:
	.loc	14 854 9
	local.get	4
	local.get	86
	v128.store	224
	.loc	14 853 9
	local.get	4
	local.get	22
	v128.store	192
.Ltmp4751:
	.loc	38 456 13
	local.get	110
	local.get	111
	local.get	7
	i32.const	.Lalloc_77f249f9c0b1ac1e2d67f390a6ea84d4
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp4752:
.LBB27_91:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	99
	v128.store	336
	local.get	4
	local.get	100
	v128.store	304
	local.get	4
	local.get	101
	v128.store	240
.Ltmp4753:
	.loc	14 854 9 is_stmt 1
	local.get	4
	local.get	85
	v128.store	288
	.loc	14 853 9
	local.get	4
	local.get	87
	v128.store	256
.Ltmp4754:
	.loc	14 854 9
	local.get	4
	local.get	86
	v128.store	224
	.loc	14 853 9
	local.get	4
	local.get	22
	v128.store	192
	local.get	8
	local.get	8
	i32.const	4
.Ltmp4755:
	.loc	14 1131 25
	i32.add 
.Ltmp4756:
	.loc	38 443 13
	local.get	7
	i32.const	.Lalloc_ecb3dab3c849990309e84d651d3f65d6
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp4757:
.LBB27_92:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	99
	v128.store	336
	local.get	4
	local.get	100
	v128.store	304
	local.get	4
	local.get	101
	v128.store	240
.Ltmp4758:
	.loc	14 854 9 is_stmt 1
	local.get	4
	local.get	85
	v128.store	288
	.loc	14 853 9
	local.get	4
	local.get	87
	v128.store	256
.Ltmp4759:
	.loc	14 854 9
	local.get	4
	local.get	86
	v128.store	224
	.loc	14 853 9
	local.get	4
	local.get	22
	v128.store	192
	local.get	9
	local.get	9
	i32.const	4
.Ltmp4760:
	.loc	14 1131 25
	i32.add 
.Ltmp4761:
	.loc	38 443 13
	local.get	7
	i32.const	.Lalloc_ecb3dab3c849990309e84d651d3f65d6
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp4762:
.LBB27_93:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	99
	v128.store	336
	local.get	4
	local.get	100
	v128.store	304
	local.get	4
	local.get	101
	v128.store	240
.Ltmp4763:
	.loc	14 854 9 is_stmt 1
	local.get	4
	local.get	85
	v128.store	288
	.loc	14 853 9
	local.get	4
	local.get	87
	v128.store	256
.Ltmp4764:
	.loc	14 854 9
	local.get	4
	local.get	86
	v128.store	224
	.loc	14 853 9
	local.get	4
	local.get	22
	v128.store	192
	local.get	10
	local.get	10
	i32.const	4
.Ltmp4765:
	.loc	14 1131 25
	i32.add 
.Ltmp4766:
	.loc	38 443 13
	local.get	7
	i32.const	.Lalloc_ecb3dab3c849990309e84d651d3f65d6
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp4767:
.LBB27_94:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	99
	v128.store	336
	local.get	4
	local.get	100
	v128.store	304
	local.get	4
	local.get	101
	v128.store	240
.Ltmp4768:
	.loc	14 854 9 is_stmt 1
	local.get	4
	local.get	85
	v128.store	288
	.loc	14 853 9
	local.get	4
	local.get	87
	v128.store	256
.Ltmp4769:
	.loc	14 854 9
	local.get	4
	local.get	86
	v128.store	224
	.loc	14 853 9
	local.get	4
	local.get	22
	v128.store	192
	local.get	9
	local.get	9
	i32.const	4
.Ltmp4770:
	.loc	14 1131 25
	i32.add 
.Ltmp4771:
	.loc	38 443 13
	local.get	75
	i32.const	.Lalloc_ecb3dab3c849990309e84d651d3f65d6
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp4772:
.LBB27_95:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	99
	v128.store	336
	local.get	4
	local.get	100
	v128.store	304
	local.get	4
	local.get	101
	v128.store	240
.Ltmp4773:
	.loc	14 854 9 is_stmt 1
	local.get	4
	local.get	85
	v128.store	288
	.loc	14 853 9
	local.get	4
	local.get	87
	v128.store	256
.Ltmp4774:
	.loc	14 854 9
	local.get	4
	local.get	86
	v128.store	224
	.loc	14 853 9
	local.get	4
	local.get	22
	v128.store	192
.Ltmp4775:
	.loc	38 456 13
	local.get	110
	local.get	111
	local.get	75
	i32.const	.Lalloc_77f249f9c0b1ac1e2d67f390a6ea84d4
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp4776:
.LBB27_96:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	99
	v128.store	336
	local.get	4
	local.get	100
	v128.store	304
	local.get	4
	local.get	101
	v128.store	240
.Ltmp4777:
	.loc	14 854 9 is_stmt 1
	local.get	4
	local.get	85
	v128.store	288
	.loc	14 853 9
	local.get	4
	local.get	87
	v128.store	256
.Ltmp4778:
	.loc	14 854 9
	local.get	4
	local.get	86
	v128.store	224
	.loc	14 853 9
	local.get	4
	local.get	22
	v128.store	192
	local.get	9
	local.get	9
	i32.const	4
.Ltmp4779:
	.loc	14 1131 25
	i32.add 
.Ltmp4780:
	.loc	38 443 13
	local.get	74
	i32.const	.Lalloc_ecb3dab3c849990309e84d651d3f65d6
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp4781:
.LBB27_97:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	99
	v128.store	336
	local.get	4
	local.get	100
	v128.store	304
	local.get	4
	local.get	101
	v128.store	240
.LBB27_98:
	end_block
	local.get	82
	i32.const	32
	i32.add 
	local.set	82
	local.get	80
	i32.const	512
.Ltmp4782:
	.loc	41 446 20 is_stmt 1
	i32.add 
	local.set	80
	local.get	81
	i32.const	-32
	i32.add 
	local.set	81
	local.get	20
	i32.const	-1
.Ltmp4783:
	.loc	14 0 0 is_stmt 0
	i32.add 
.Ltmp4784:
	.loc	41 446 20
	local.tee	20
	i32.eqz
	br_if   	2
	br      	0
.Ltmp4785:
.LBB27_99:
	.loc	14 3187 12 is_stmt 1
	end_loop
	end_block
.Ltmp4786:
	.loc	14 3327 24
	local.get	4
	i32.const	736
	i32.add 
	local.get	15
	call	_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_
.Ltmp4787:
	.loc	14 3337 27
	local.get	0
	i32.load	804
	local.set	17
.Ltmp4788:
	.loc	14 3336 27
	local.get	0
	i32.load	800
	local.set	18
.Ltmp4789:
	.loc	14 3333 21
	local.get	0
	i32.load8_u	785
	local.set	9
.Ltmp4790:
	.loc	14 3332 19
	local.get	0
	i32.load8_u	784
	local.set	8
.Ltmp4791:
	.loc	14 3335 16
	local.get	0
	i32.load	920
	local.set	19
.Ltmp4792:
	.loc	14 3334 16
	local.get	0
	i32.load	916
	local.set	12
	local.get	4
	i32.const	1136
	i32.add 
	i32.const	0
	i32.const	1024
	memory.fill	0
.Ltmp4793:
	.loc	14 3341 32
	local.get	4
	i32.const	2160
	i32.add 
	local.get	15
	local.get	12
	local.get	19
	call	_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E3newB5_
	local.get	4
	v128.load	736
	local.set	105
.Ltmp4794:
	.loc	41 446 20
	block   	
	block   	
	local.get	3
	br_if   	0
.Ltmp4795:
	.loc	14 3411 31
	local.get	4
	i32.load	2176
	local.set	6
.Ltmp4796:
	.loc	41 446 20
	br      	1
.Ltmp4797:
.LBB27_101:
	.loc	41 0 20 is_stmt 0
	end_block
	local.get	3
	i32.const	5
.Ltmp4798:
	.loc	13 3756 21 is_stmt 1
	i32.shr_u
	local.get	3
	i32.const	31
.Ltmp4799:
	.loc	13 3757 21
	i32.and 
	i32.const	0
.Ltmp4800:
	.loc	13 3758 16
	i32.ne  
	i32.add 
	local.set	20
.Ltmp4801:
	.loc	14 3333 21
	v128.const	-1, -1, -1, -1
	local.tee	21
	v128.const	0, 0, 0, 0
	local.tee	22
	local.get	9
	i32.const	1
	i32.and 
	v128.select
	local.set	104
	local.get	21
	local.get	22
	local.get	8
	i32.const	1
.Ltmp4802:
	.loc	14 3332 19
	i32.and 
	v128.select
	local.set	101
	local.get	0
	v128.load	768
	local.set	24
	local.get	0
	v128.load	752
	local.set	23
	local.get	0
	v128.load	736
	local.set	25
	local.get	0
	v128.load	720
	local.set	26
	local.get	0
	v128.load	704
	local.set	27
	local.get	0
	v128.load	688
	local.set	28
	local.get	0
	v128.load	672
	local.set	29
	local.get	0
	v128.load	656
	local.set	30
	local.get	0
	v128.load	640
	local.set	31
	local.get	0
	v128.load	624
	local.set	32
	local.get	0
	v128.load	608
	local.set	33
	local.get	0
	v128.load	592
	local.set	34
	local.get	0
	v128.load	576
	local.set	35
	local.get	0
	v128.load	560
	local.set	36
	local.get	0
	v128.load	544
	local.set	37
	local.get	0
	v128.load	528
	local.set	38
	local.get	0
	v128.load	512
	local.set	39
	local.get	0
	v128.load	496
	local.set	40
	local.get	0
	v128.load	480
	local.set	41
	local.get	0
	v128.load	464
	local.set	42
	local.get	0
	v128.load	448
	local.set	43
	local.get	0
	v128.load	432
	local.set	44
	local.get	0
	v128.load	416
	local.set	45
	local.get	0
	v128.load	400
	local.set	46
	local.get	0
	v128.load	384
	local.set	47
	local.get	0
	v128.load	368
	local.set	48
	local.get	0
	v128.load	352
	local.set	49
	local.get	0
	v128.load	336
	local.set	50
	local.get	0
	v128.load	320
	local.set	51
	local.get	0
	v128.load	304
	local.set	52
	local.get	0
	v128.load	288
	local.set	53
	local.get	0
	v128.load	272
	local.set	54
	local.get	0
	v128.load	256
	local.set	55
	local.get	0
	v128.load	240
	local.set	56
	local.get	0
	v128.load	224
	local.set	57
	local.get	0
	v128.load	208
	local.set	58
	local.get	0
	v128.load	192
	local.set	59
	local.get	0
	v128.load	176
	local.set	60
	local.get	0
	v128.load	160
	local.set	61
	local.get	0
	v128.load	144
	local.set	62
	local.get	0
	v128.load	128
	local.set	63
	local.get	0
	v128.load	112
	local.set	64
	local.get	0
	v128.load	96
	local.set	65
	local.get	0
	v128.load	80
	local.set	66
	local.get	0
	v128.load	64
	local.set	67
	local.get	0
	v128.load	48
	local.set	68
	local.get	0
	v128.load	32
	local.set	69
	local.get	0
	v128.load	16
	local.set	70
	local.get	4
	i32.load	2208
	local.set	73
	local.get	4
	i32.load	2212
	local.set	74
	local.get	4
	v128.load	1088
	local.set	102
	local.get	4
	i32.load	2204
	local.set	75
	local.get	4
	i32.load	2200
	local.set	76
	local.get	4
	i32.load	2180
	local.set	77
	local.get	4
	i32.load	2196
	local.set	7
	local.get	4
	i32.load	2192
	local.set	11
	local.get	4
	v128.load	992
	local.set	103
	local.get	4
	v128.load	928
	local.set	96
	local.get	4
	i32.load	2188
	local.set	81
	local.get	4
	i32.load	2184
	local.set	84
	local.get	4
	v128.load	1072
	local.set	97
	local.get	4
	i32.load	2176
	local.set	6
	local.get	4
	v128.load	912
	local.set	95
	local.get	4
	v128.load	896
	local.set	85
	local.get	4
	v128.load	880
	local.set	86
	local.get	4
	v128.load	864
	local.set	87
	local.get	4
	v128.load	848
	local.set	88
	local.get	4
	v128.load	832
	local.set	22
	local.get	4
	v128.load	816
	local.set	89
	local.get	4
	v128.load	800
	local.set	90
	local.get	4
	v128.load	784
	local.set	91
	local.get	4
	v128.load	768
	local.set	92
	local.get	4
	v128.load	752
	local.set	93
	local.get	1
	local.set	80
	local.get	3
	local.set	78
	i32.const	0
	local.set	82
.LBB27_102:
.Ltmp4803:
	.loc	14 3344 55
	loop    	
	block   	
	block   	
	block   	
	block   	
	block   	
	block   	
	block   	
	block   	
	block   	
	block   	
	local.get	3
	local.get	82
	i32.sub 
	local.tee	9
	i32.const	32
	local.get	9
	i32.const	32
.Ltmp4804:
	.loc	9 1077 12
	i32.lt_u
	i32.select
.Ltmp4805:
	.loc	14 3349 39
	local.tee	83
	local.get	82
	i32.add 
	i32.const	2
	i32.shl 
	local.tee	9
	local.get	82
	i32.const	2
.Ltmp4806:
	.loc	14 3345 31
	i32.shl 
.Ltmp4807:
	.loc	13 1050 16
	local.tee	8
	i32.lt_u
	br_if   	0
	local.get	9
	local.get	2
	i32.gt_u
	br_if   	0
.Ltmp4808:
	.loc	33 304 12
	local.get	3
	local.get	82
	i32.eq  
	br_if   	9
.Ltmp4809:
	.loc	33 0 12 is_stmt 0
	local.get	78
	i32.const	32
	local.get	78
	i32.const	32
	i32.lt_u
	i32.select
	local.tee	79
	i32.const	1
	local.get	79
	i32.const	1
	i32.gt_u
	i32.select
	local.set	10
	local.get	4
	i32.const	1136
	i32.add 
	local.set	9
	local.get	80
	local.set	8
.LBB27_106:
	loop    	
	local.get	85
	local.set	95
	local.get	86
	local.set	85
	local.get	87
	local.set	86
	local.get	88
	local.set	87
	local.get	22
	local.set	88
	local.get	89
	local.set	22
	local.get	90
	local.set	89
	local.get	91
	local.set	90
	local.get	92
	local.set	91
	local.get	93
	local.set	92
	local.get	105
	local.set	93
.Ltmp4810:
	.loc	1 551 14 is_stmt 1
	local.get	9
	local.get	67
	local.get	8
	v128.load	0:p2align=2
.Ltmp4811:
	.loc	42 3867 14
	local.tee	105
	f32x4.mul
.Ltmp4812:
	.loc	42 3845 14
	v128.const	0x0p0, 0x0p0, 0x0p0, 0x0p0
.Ltmp4813:
	.loc	42 3845 14 is_stmt 0
	local.tee	21
	f32x4.add
.Ltmp4814:
	.loc	42 3867 14 is_stmt 1
	local.get	63
	local.get	93
	f32x4.mul
.Ltmp4815:
	.loc	42 3845 14
	f32x4.add
.Ltmp4816:
	.loc	42 3867 14
	local.get	59
	local.get	92
	f32x4.mul
.Ltmp4817:
	.loc	42 3845 14
	f32x4.add
.Ltmp4818:
	.loc	42 3867 14
	local.get	55
	local.get	91
	f32x4.mul
.Ltmp4819:
	.loc	42 3845 14
	f32x4.add
.Ltmp4820:
	.loc	42 3867 14
	local.get	51
	local.get	90
	f32x4.mul
.Ltmp4821:
	.loc	42 3845 14
	f32x4.add
.Ltmp4822:
	.loc	42 3867 14
	local.get	47
	local.get	89
	f32x4.mul
.Ltmp4823:
	.loc	42 3845 14
	f32x4.add
.Ltmp4824:
	.loc	42 3867 14
	local.get	43
	local.get	22
	f32x4.mul
.Ltmp4825:
	.loc	42 3845 14
	f32x4.add
.Ltmp4826:
	.loc	42 3867 14
	local.get	39
	local.get	88
	f32x4.mul
.Ltmp4827:
	.loc	42 3845 14
	f32x4.add
.Ltmp4828:
	.loc	42 3867 14
	local.get	35
	local.get	87
	f32x4.mul
.Ltmp4829:
	.loc	42 3845 14
	f32x4.add
.Ltmp4830:
	.loc	42 3867 14
	local.get	31
	local.get	86
	f32x4.mul
.Ltmp4831:
	.loc	42 3845 14
	f32x4.add
.Ltmp4832:
	.loc	42 3867 14
	local.get	27
	local.get	85
	f32x4.mul
.Ltmp4833:
	.loc	42 3845 14
	f32x4.add
.Ltmp4834:
	.loc	42 3867 14
	local.get	24
	local.get	95
	f32x4.mul
.Ltmp4835:
	.loc	42 3845 14
	f32x4.add
.Ltmp4836:
	.loc	42 3812 14
	f32x4.abs
.Ltmp4837:
	.loc	42 3867 14
	local.get	68
	local.get	105
	f32x4.mul
.Ltmp4838:
	.loc	42 3845 14
	local.get	21
	f32x4.add
.Ltmp4839:
	.loc	42 3867 14
	local.get	64
	local.get	93
	f32x4.mul
.Ltmp4840:
	.loc	42 3845 14
	f32x4.add
.Ltmp4841:
	.loc	42 3867 14
	local.get	60
	local.get	92
	f32x4.mul
.Ltmp4842:
	.loc	42 3845 14
	f32x4.add
.Ltmp4843:
	.loc	42 3867 14
	local.get	56
	local.get	91
	f32x4.mul
.Ltmp4844:
	.loc	42 3845 14
	f32x4.add
.Ltmp4845:
	.loc	42 3867 14
	local.get	52
	local.get	90
	f32x4.mul
.Ltmp4846:
	.loc	42 3845 14
	f32x4.add
.Ltmp4847:
	.loc	42 3867 14
	local.get	48
	local.get	89
	f32x4.mul
.Ltmp4848:
	.loc	42 3845 14
	f32x4.add
.Ltmp4849:
	.loc	42 3867 14
	local.get	44
	local.get	22
	f32x4.mul
.Ltmp4850:
	.loc	42 3845 14
	f32x4.add
.Ltmp4851:
	.loc	42 3867 14
	local.get	40
	local.get	88
	f32x4.mul
.Ltmp4852:
	.loc	42 3845 14
	f32x4.add
.Ltmp4853:
	.loc	42 3867 14
	local.get	36
	local.get	87
	f32x4.mul
.Ltmp4854:
	.loc	42 3845 14
	f32x4.add
.Ltmp4855:
	.loc	42 3867 14
	local.get	32
	local.get	86
	f32x4.mul
.Ltmp4856:
	.loc	42 3845 14
	f32x4.add
.Ltmp4857:
	.loc	42 3867 14
	local.get	28
	local.get	85
	f32x4.mul
.Ltmp4858:
	.loc	42 3845 14
	f32x4.add
.Ltmp4859:
	.loc	42 3867 14
	local.get	23
	local.get	95
	f32x4.mul
.Ltmp4860:
	.loc	42 3845 14
	f32x4.add
.Ltmp4861:
	.loc	42 3812 14
	f32x4.abs
.Ltmp4862:
	.loc	42 3867 14
	local.get	69
	local.get	105
	f32x4.mul
.Ltmp4863:
	.loc	42 3845 14
	local.get	21
	f32x4.add
.Ltmp4864:
	.loc	42 3867 14
	local.get	65
	local.get	93
	f32x4.mul
.Ltmp4865:
	.loc	42 3845 14
	f32x4.add
.Ltmp4866:
	.loc	42 3867 14
	local.get	61
	local.get	92
	f32x4.mul
.Ltmp4867:
	.loc	42 3845 14
	f32x4.add
.Ltmp4868:
	.loc	42 3867 14
	local.get	57
	local.get	91
	f32x4.mul
.Ltmp4869:
	.loc	42 3845 14
	f32x4.add
.Ltmp4870:
	.loc	42 3867 14
	local.get	53
	local.get	90
	f32x4.mul
.Ltmp4871:
	.loc	42 3845 14
	f32x4.add
.Ltmp4872:
	.loc	42 3867 14
	local.get	49
	local.get	89
	f32x4.mul
.Ltmp4873:
	.loc	42 3845 14
	f32x4.add
.Ltmp4874:
	.loc	42 3867 14
	local.get	45
	local.get	22
	f32x4.mul
.Ltmp4875:
	.loc	42 3845 14
	f32x4.add
.Ltmp4876:
	.loc	42 3867 14
	local.get	41
	local.get	88
	f32x4.mul
.Ltmp4877:
	.loc	42 3845 14
	f32x4.add
.Ltmp4878:
	.loc	42 3867 14
	local.get	37
	local.get	87
	f32x4.mul
.Ltmp4879:
	.loc	42 3845 14
	f32x4.add
.Ltmp4880:
	.loc	42 3867 14
	local.get	33
	local.get	86
	f32x4.mul
.Ltmp4881:
	.loc	42 3845 14
	f32x4.add
.Ltmp4882:
	.loc	42 3867 14
	local.get	29
	local.get	85
	f32x4.mul
.Ltmp4883:
	.loc	42 3845 14
	f32x4.add
.Ltmp4884:
	.loc	42 3867 14
	local.get	25
	local.get	95
	f32x4.mul
.Ltmp4885:
	.loc	42 3845 14
	f32x4.add
.Ltmp4886:
	.loc	42 3812 14
	f32x4.abs
.Ltmp4887:
	.loc	42 3867 14
	local.get	70
	local.get	105
	f32x4.mul
.Ltmp4888:
	.loc	42 3845 14
	local.get	21
	f32x4.add
.Ltmp4889:
	.loc	42 3867 14
	local.get	66
	local.get	93
	f32x4.mul
.Ltmp4890:
	.loc	42 3845 14
	f32x4.add
.Ltmp4891:
	.loc	42 3867 14
	local.get	62
	local.get	92
	f32x4.mul
.Ltmp4892:
	.loc	42 3845 14
	f32x4.add
.Ltmp4893:
	.loc	42 3867 14
	local.get	58
	local.get	91
	f32x4.mul
.Ltmp4894:
	.loc	42 3845 14
	f32x4.add
.Ltmp4895:
	.loc	42 3867 14
	local.get	54
	local.get	90
	f32x4.mul
.Ltmp4896:
	.loc	42 3845 14
	f32x4.add
.Ltmp4897:
	.loc	42 3867 14
	local.get	50
	local.get	89
	f32x4.mul
.Ltmp4898:
	.loc	42 3845 14
	f32x4.add
.Ltmp4899:
	.loc	42 3867 14
	local.get	46
	local.get	22
	f32x4.mul
.Ltmp4900:
	.loc	42 3845 14
	f32x4.add
.Ltmp4901:
	.loc	42 3867 14
	local.get	42
	local.get	88
	f32x4.mul
.Ltmp4902:
	.loc	42 3845 14
	f32x4.add
.Ltmp4903:
	.loc	42 3867 14
	local.get	38
	local.get	87
	f32x4.mul
.Ltmp4904:
	.loc	42 3845 14
	f32x4.add
.Ltmp4905:
	.loc	42 3867 14
	local.get	34
	local.get	86
	f32x4.mul
.Ltmp4906:
	.loc	42 3845 14
	f32x4.add
.Ltmp4907:
	.loc	42 3867 14
	local.get	30
	local.get	85
	f32x4.mul
.Ltmp4908:
	.loc	42 3845 14
	f32x4.add
.Ltmp4909:
	.loc	42 3867 14
	local.get	26
	local.get	95
	f32x4.mul
.Ltmp4910:
	.loc	42 3845 14
	f32x4.add
.Ltmp4911:
	.loc	42 3812 14
	f32x4.abs
.Ltmp4912:
	.loc	42 3812 14 is_stmt 0
	local.get	22
	f32x4.abs
.Ltmp4913:
	.loc	42 3928 9 is_stmt 1
	f32x4.pmax
	f32x4.pmax
	f32x4.pmax
	f32x4.pmax
.Ltmp4914:
	.loc	1 551 14
	v128.store	0:p2align=2
	local.get	9
	i32.const	16
.Ltmp4915:
	.loc	33 304 12
	i32.add 
	local.set	9
	local.get	8
	i32.const	16
	i32.add 
	local.set	8
	local.get	10
	i32.const	-1
	i32.add 
	local.tee	10
	br_if   	0
	end_loop
	i32.const	0
	local.set	106
.Ltmp4916:
.LBB27_108:
	.loc	14 1575 16
	loop    	
	local.get	0
	i32.load	916
.Ltmp4917:
	.loc	14 1580 33
	local.tee	9
	local.get	81
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp4918:
	.loc	14 1148 8
	local.get	9
	local.get	8
	local.get	9
	i32.lt_u
	i32.select
	i32.sub 
.Ltmp4919:
	.loc	14 1588 14
	local.tee	107
	i32.sub 
.Ltmp4920:
	.loc	14 1578 28
	local.tee	8
	local.get	9
	local.get	84
	local.get	17
	i32.add 
	local.tee	10
	i32.const	0
.Ltmp4921:
	.loc	14 1148 8
	local.get	9
	local.get	10
	local.get	9
	i32.lt_u
	i32.select
	i32.sub 
.Ltmp4922:
	.loc	14 1586 14
	local.tee	108
	i32.sub 
	local.tee	10
	local.get	9
	local.get	17
	i32.const	1
.Ltmp4923:
	.loc	14 1577 25
	i32.add 
	local.tee	13
	i32.const	0
.Ltmp4924:
	.loc	14 1148 8
	local.get	9
	local.get	13
	local.get	9
	i32.lt_u
	i32.select
	i32.sub 
.Ltmp4925:
	.loc	14 1585 14
	local.tee	109
	i32.sub 
.Ltmp4926:
	.loc	14 1576 16
	local.tee	13
	local.get	0
	i32.load	920
.Ltmp4927:
	.loc	14 1584 14
	local.get	18
	i32.sub 
	.loc	14 1583 14
	local.tee	110
	local.get	9
	local.get	17
	i32.sub 
.Ltmp4928:
	.loc	14 3363 21
	local.tee	9
	local.get	83
	local.get	106
	i32.sub 
.Ltmp4929:
	.loc	9 1077 12
	local.tee	111
	local.get	9
	local.get	111
	i32.lt_u
	i32.select
.Ltmp4930:
	.loc	9 1077 12 is_stmt 0
	local.tee	111
	local.get	110
	local.get	111
	i32.lt_u
	i32.select
.Ltmp4931:
	.loc	9 1077 12
	local.tee	111
	local.get	13
	local.get	111
	i32.lt_u
	i32.select
.Ltmp4932:
	.loc	9 1077 12
	local.tee	111
	local.get	10
	local.get	111
	i32.lt_u
	i32.select
.Ltmp4933:
	.loc	9 1077 12
	local.tee	111
	local.get	8
	local.get	111
	i32.lt_u
	i32.select
.Ltmp4934:
	.loc	14 3369 28 is_stmt 1
	local.tee	112
	local.get	106
	local.get	82
	i32.add 
.Ltmp4935:
	.loc	14 3371 55
	local.tee	113
	i32.add 
	i32.const	2
	i32.shl 
	local.tee	111
	local.get	113
	i32.const	2
.Ltmp4936:
	.loc	14 3369 28
	i32.shl 
.Ltmp4937:
	.loc	13 1050 16
	local.tee	113
	i32.lt_u
	br_if   	2
	local.get	111
	local.get	2
	i32.gt_u
	br_if   	2
.Ltmp4938:
	.loc	33 304 12
	block   	
	local.get	112
	i32.eqz
	br_if   	0
.Ltmp4939:
	.loc	33 0 12 is_stmt 0
	local.get	1
	local.get	113
	i32.const	2
	i32.shl 
	i32.add 
	local.set	114
	local.get	4
	i32.const	1136
	i32.add 
	local.get	106
	i32.const	4
	i32.shl 
	i32.add 
	local.set	115
	local.get	10
	local.get	8
	local.get	10
	local.get	8
	i32.lt_u
	i32.select
	local.tee	8
	local.get	13
	local.get	8
	local.get	13
	i32.lt_u
	i32.select
	local.tee	8
	local.get	9
	local.get	8
	local.get	9
	i32.lt_u
	i32.select
	local.tee	9
	local.get	110
	local.get	9
	local.get	110
	i32.lt_u
	i32.select
	local.tee	9
	local.get	79
	local.get	106
	i32.sub 
	local.tee	8
	local.get	9
	local.get	8
	i32.lt_u
	i32.select
	i32.const	1073741823
	i32.and 
	local.set	116
	i32.const	0
	local.set	13
.LBB27_112:
.Ltmp4940:
	.loc	14 1502 26 is_stmt 1
	loop    	
	local.get	13
	local.get	17
	i32.add 
	i32.const	2
.Ltmp4941:
	.loc	14 1137 16
	i32.shl 
	local.tee	110
	i32.const	4
.Ltmp4942:
	.loc	14 1138 33
	i32.add 
	local.set	111
	local.get	110
	i32.const	3
.Ltmp4943:
	.loc	13 1050 16
	i32.or  
	local.get	7
	i32.ge_u
	br_if   	5
.Ltmp4944:
	.loc	13 0 16 is_stmt 0
	local.get	114
	local.get	13
	i32.const	4
	i32.shl 
	local.tee	9
	i32.add 
	local.tee	117
	v128.load	0:p2align=2
	local.set	98
	local.get	11
	local.get	110
	i32.const	2
.Ltmp4945:
	.loc	38 101 24 is_stmt 1
	i32.shl 
	local.tee	113
	i32.add 
.Ltmp4946:
	.loc	14 0 0 is_stmt 0
	local.get	96
	local.get	115
	local.get	9
	i32.add 
.Ltmp4947:
	v128.load	0:p2align=2
.Ltmp4948:
	.loc	42 2188 14 is_stmt 1
	local.tee	21
	local.get	21
	local.get	101
	v128.bitselect
.Ltmp4949:
	.loc	42 3878 14
	local.tee	21
	f32x4.div
.Ltmp4950:
	.loc	42 2188 14
	v128.const	0, 0, 128, 63, 0, 0, 128, 63, 0, 0, 128, 63, 0, 0, 128, 63
.Ltmp4951:
	.loc	42 2005 14
	local.get	96
	local.get	21
	f32x4.lt
.Ltmp4952:
	.loc	42 2188 14
	v128.bitselect
.Ltmp4953:
	.loc	1 551 14
	v128.store	0:p2align=2
.Ltmp4954:
	.loc	14 0 0 is_stmt 0
	local.get	13
	local.get	108
	i32.add 
	local.tee	9
	i32.const	2
.Ltmp4955:
	.loc	14 1130 16 is_stmt 1
	i32.shl 
	local.tee	8
	i32.const	3
.Ltmp4956:
	.loc	13 1050 16
	i32.or  
	local.get	7
	i32.ge_u
	br_if   	6
.Ltmp4957:
	.loc	13 0 16 is_stmt 0
	local.get	4
	local.get	11
	local.get	8
	i32.const	2
.Ltmp4958:
	.loc	38 89 24 is_stmt 1
	i32.shl 
	i32.add 
.Ltmp4959:
	.loc	1 551 14
	v128.load	0:p2align=2
.Ltmp4960:
	.loc	14 1206 22
	local.tee	21
	local.get	4
	v128.load	2160
	f32x4.pmin
	local.get	21
	local.get	6
	v128.select
.Ltmp4961:
	.loc	14 1211 5
	local.tee	94
	v128.store	2160
	block   	
	block   	
	local.get	6
	i32.const	1
	.loc	14 1212 20
	i32.add 
	local.tee	6
	local.get	77
	i32.eq  
.Ltmp4962:
	.loc	14 1213 22
	br_if   	0
.Ltmp4963:
	.loc	14 0 0 is_stmt 0
	local.get	13
	local.get	109
	i32.add 
	i32.const	2
.Ltmp4964:
	.loc	14 1130 16 is_stmt 1
	i32.shl 
	local.tee	9
	i32.const	3
.Ltmp4965:
	.loc	13 1050 16
	i32.or  
	local.get	7
	i32.ge_u
	br_if   	9
.Ltmp4966:
	.loc	13 0 16 is_stmt 0
	local.get	94
	local.get	11
	local.get	9
	i32.const	2
.Ltmp4967:
	.loc	38 89 24 is_stmt 1
	i32.shl 
	i32.add 
.Ltmp4968:
	.loc	1 551 14
	v128.load	0:p2align=2
.Ltmp4969:
	.loc	42 3911 9
	f32x4.pmin
	local.set	94
.Ltmp4970:
	.loc	14 1218 5
	br      	1
.LBB27_117:
	.loc	14 0 5 is_stmt 0
	end_block
	i32.const	0
	local.set	6
.Ltmp4971:
	.loc	10 900 12 is_stmt 1
	local.get	77
	i32.eqz
	br_if   	0
.Ltmp4972:
	.loc	10 0 12 is_stmt 0
	local.get	77
	local.set	8
.LBB27_119:
	loop    	
	local.get	9
	i32.const	2
.Ltmp4973:
	.loc	14 1130 16 is_stmt 1
	i32.shl 
	local.tee	10
	i32.const	3
.Ltmp4974:
	.loc	13 1050 16
	i32.or  
	local.get	7
	i32.ge_u
	br_if   	10
.Ltmp4975:
	.loc	13 0 16 is_stmt 0
	local.get	11
	local.get	10
	i32.const	2
.Ltmp4976:
	.loc	38 89 24 is_stmt 1
	i32.shl 
	i32.add 
.Ltmp4977:
	.loc	1 551 14
	local.tee	10
	local.get	10
	v128.load	0:p2align=2
.Ltmp4978:
	.loc	42 3911 9
	local.get	21
	f32x4.pmin
.Ltmp4979:
	.loc	1 551 14
	local.tee	21
	v128.store	0:p2align=2
.Ltmp4980:
	.loc	14 1224 16
	local.get	9
	local.get	12
	local.get	9
	i32.select
	i32.const	-1
	.loc	14 1227 13
	i32.add 
	local.set	9
	local.get	8
	i32.const	-1
.Ltmp4981:
	.loc	9 1916 50
	i32.add 
.Ltmp4982:
	.loc	10 900 12
	local.tee	8
	br_if   	0
.LBB27_121:
	.loc	10 900 12
	end_loop
	end_block
.Ltmp4983:
	.loc	14 0 0 is_stmt 0
	local.get	13
	local.get	107
	i32.add 
	i32.const	2
.Ltmp4984:
	.loc	14 1130 16 is_stmt 1
	i32.shl 
	local.tee	9
	i32.const	3
.Ltmp4985:
	.loc	13 1050 16
	i32.or  
	local.get	75
	i32.ge_u
	br_if   	9
.Ltmp4986:
	.loc	13 0 16 is_stmt 0
	local.get	111
	local.get	75
	i32.gt_u
.Ltmp4987:
	.loc	13 1050 16
	br_if   	10
.Ltmp4988:
	.loc	13 0 16
	local.get	76
	local.get	9
	i32.const	2
	i32.shl 
	i32.add 
	v128.load	0:p2align=2
	local.set	21
.Ltmp4989:
	.loc	38 101 24 is_stmt 1
	local.get	76
	local.get	113
	i32.add 
.Ltmp4990:
	.loc	14 0 0 is_stmt 0
	local.get	94
	v128.const	0x1p14, 0x1p14, 0x1p14, 0x1p14
	f32x4.mul
	f32x4.floor
	v128.const	0x1p-14, 0x1p-14, 0x1p-14, 0x1p-14
	f32x4.mul
.Ltmp4991:
	.loc	1 551 14 is_stmt 1
	local.tee	94
	v128.store	0:p2align=2
.Ltmp4992:
	.loc	14 1659 43
	local.get	4
	local.get	4
	v128.load	1056
.Ltmp4993:
	.loc	42 3856 14
	local.tee	99
	local.get	103
	v128.const	0x1p0, 0x1p0, 0x1p0, 0x1p0
.Ltmp4994:
	.loc	14 0 0 is_stmt 0
	local.tee	100
	local.get	94
	local.get	97
	f32x4.add
.Ltmp4995:
	local.get	21
	f32x4.sub
.Ltmp4996:
	.loc	42 3878 14 is_stmt 1
	local.tee	97
	local.get	102
	f32x4.div
.Ltmp4997:
	.loc	42 3856 14
	f32x4.sub
.Ltmp4998:
	.loc	42 3856 14 is_stmt 0
	local.tee	21
	local.get	99
	f32x4.sub
.Ltmp4999:
	.loc	42 3867 14 is_stmt 1
	f32x4.mul
.Ltmp5000:
	.loc	42 3845 14
	f32x4.add
.Ltmp5001:
	.loc	42 3928 9
	local.get	21
	f32x4.pmax
.Ltmp5002:
	.loc	42 3812 14
	local.tee	21
	local.get	21
	f32x4.abs
.Ltmp5003:
	.loc	42 1991 14
	v128.const	0x1.79ca1p-67, 0x1.79ca1p-67, 0x1.79ca1p-67, 0x1.79ca1p-67
	f32x4.lt
.Ltmp5004:
	.loc	44 481 22
	v128.andnot
.Ltmp5005:
	.loc	14 1660 5
	local.tee	94
	v128.store	1056
.Ltmp5006:
	.loc	14 0 0 is_stmt 0
	local.get	13
	local.get	18
	i32.add 
	i32.const	2
.Ltmp5007:
	.loc	14 1130 16 is_stmt 1
	i32.shl 
	local.tee	9
	i32.const	3
.Ltmp5008:
	.loc	13 1050 16
	i32.or  
	local.get	74
	i32.ge_u
	br_if   	11
.Ltmp5009:
	.loc	13 0 16 is_stmt 0
	local.get	73
	local.get	9
	i32.const	2
.Ltmp5010:
	.loc	38 89 24 is_stmt 1
	i32.shl 
	i32.add 
.Ltmp5011:
	.loc	1 551 14
	local.tee	9
	v128.load	0:p2align=2
	local.set	21
.Ltmp5012:
	.loc	1 551 14 is_stmt 0
	local.get	9
	local.get	98
	v128.store	0:p2align=2
.Ltmp5013:
	.loc	42 3856 14 is_stmt 1
	local.get	117
	local.get	21
	local.get	21
	local.get	100
	local.get	94
	f32x4.sub
.Ltmp5014:
	.loc	42 3867 14
	f32x4.mul
.Ltmp5015:
	.loc	42 2188 14
	local.get	104
	v128.bitselect
.Ltmp5016:
	.loc	1 551 14
	v128.store	0:p2align=2
	local.get	13
	i32.const	1
.Ltmp5017:
	.loc	14 0 0 is_stmt 0
	i32.add 
.Ltmp5018:
	.loc	33 304 12 is_stmt 1
	local.tee	13
	local.get	116
	i32.ne  
	br_if   	0
.LBB27_125:
	end_loop
	end_block
.Ltmp5019:
	.loc	14 3407 39
	local.get	112
	local.get	18
	i32.add 
	local.tee	9
	i32.const	0
.Ltmp5020:
	.loc	14 1148 8
	local.get	19
	local.get	9
	local.get	19
	i32.lt_u
	i32.select
	i32.sub 
	local.set	18
.Ltmp5021:
	.loc	14 3406 39
	local.get	112
	local.get	17
	i32.add 
	local.tee	9
	i32.const	0
.Ltmp5022:
	.loc	14 1148 8
	local.get	12
	local.get	9
	local.get	12
	i32.lt_u
	i32.select
	i32.sub 
	local.set	17
.Ltmp5023:
	.loc	14 0 0 is_stmt 0
	local.get	112
	local.get	106
	i32.add 
.Ltmp5024:
	.loc	14 3355 19 is_stmt 1
	local.tee	106
	local.get	83
	i32.ge_u
	br_if   	10
	br      	0
.Ltmp5025:
.LBB27_126:
	.loc	13 1050 16
	end_loop
	end_block
.Ltmp5026:
	.loc	38 443 13
	local.get	8
	local.get	9
	local.get	2
	i32.const	.Lalloc_bac0677bfb785f04669fb3d80741c988
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5027:
.LBB27_127:
	.loc	38 0 13 is_stmt 0
	end_block
.Ltmp5028:
	.loc	38 456 13 is_stmt 1
	local.get	113
	local.get	111
	local.get	2
	i32.const	.Lalloc_1eb3474cf2d0fe655b193b0c53e74d06
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5029:
.LBB27_128:
	.loc	38 0 13 is_stmt 0
	end_block
.Ltmp5030:
	.loc	38 456 13 is_stmt 1
	local.get	110
	local.get	111
	local.get	7
	i32.const	.Lalloc_77f249f9c0b1ac1e2d67f390a6ea84d4
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5031:
.LBB27_129:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	8
	local.get	8
	i32.const	4
.Ltmp5032:
	.loc	14 1131 25 is_stmt 1
	i32.add 
.Ltmp5033:
	.loc	38 443 13
	local.get	7
	i32.const	.Lalloc_ecb3dab3c849990309e84d651d3f65d6
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5034:
.LBB27_130:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	9
	local.get	9
	i32.const	4
.Ltmp5035:
	.loc	14 1131 25 is_stmt 1
	i32.add 
.Ltmp5036:
	.loc	38 443 13
	local.get	7
	i32.const	.Lalloc_ecb3dab3c849990309e84d651d3f65d6
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5037:
.LBB27_131:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	10
	local.get	10
	i32.const	4
.Ltmp5038:
	.loc	14 1131 25 is_stmt 1
	i32.add 
.Ltmp5039:
	.loc	38 443 13
	local.get	7
	i32.const	.Lalloc_ecb3dab3c849990309e84d651d3f65d6
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5040:
.LBB27_132:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	9
	local.get	9
	i32.const	4
.Ltmp5041:
	.loc	14 1131 25 is_stmt 1
	i32.add 
.Ltmp5042:
	.loc	38 443 13
	local.get	75
	i32.const	.Lalloc_ecb3dab3c849990309e84d651d3f65d6
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5043:
.LBB27_133:
	.loc	38 0 13 is_stmt 0
	end_block
.Ltmp5044:
	.loc	38 456 13 is_stmt 1
	local.get	110
	local.get	111
	local.get	75
	i32.const	.Lalloc_77f249f9c0b1ac1e2d67f390a6ea84d4
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5045:
.LBB27_134:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	9
	local.get	9
	i32.const	4
.Ltmp5046:
	.loc	14 1131 25 is_stmt 1
	i32.add 
.Ltmp5047:
	.loc	38 443 13
	local.get	74
	i32.const	.Lalloc_ecb3dab3c849990309e84d651d3f65d6
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5048:
.LBB27_135:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	82
	i32.const	32
	i32.add 
	local.set	82
	local.get	80
	i32.const	512
.Ltmp5049:
	.loc	41 446 20 is_stmt 1
	i32.add 
	local.set	80
	local.get	78
	i32.const	-32
	i32.add 
	local.set	78
	local.get	20
	i32.const	-1
.Ltmp5050:
	.loc	14 0 0 is_stmt 0
	i32.add 
.Ltmp5051:
	.loc	41 446 20
	local.tee	20
	br_if   	0
	end_loop
	local.get	4
	local.get	97
	v128.store	1072
.Ltmp5052:
	.loc	14 0 0
	local.get	4
	local.get	95
	v128.store	912
	local.get	4
	local.get	85
	v128.store	896
	local.get	4
	local.get	86
	v128.store	880
	local.get	4
	local.get	87
	v128.store	864
	local.get	4
	local.get	88
	v128.store	848
	local.get	4
	local.get	22
	v128.store	832
	local.get	4
	local.get	89
	v128.store	816
	local.get	4
	local.get	90
	v128.store	800
	local.get	4
	local.get	91
	v128.store	784
	local.get	4
	local.get	92
	v128.store	768
	local.get	4
	local.get	93
	v128.store	752
.LBB27_137:
	end_block
	local.get	4
	local.get	105
	v128.store	736
.Ltmp5053:
	.loc	14 3414 23 is_stmt 1
	block   	
	local.get	0
	i32.load	968
	local.tee	9
	i32.const	3
.Ltmp5054:
	.loc	38 451 16
	i32.le_u
	br_if   	0
.Ltmp5055:
	.loc	14 3414 23
	local.get	0
	i32.load	964
.Ltmp5056:
	.loc	14 0 0 is_stmt 0
	local.get	4
	v128.load	2160
.Ltmp5057:
	.loc	1 551 14 is_stmt 1
	v128.store	0:p2align=2
.Ltmp5058:
	.loc	14 3415 5
	block   	
	local.get	0
	i32.load	984
.Ltmp5059:
	.loc	15 180 28
	local.tee	9
	i32.eqz
	br_if   	0
.Ltmp5060:
	.loc	15 0 28 is_stmt 0
	local.get	9
	i32.const	2
	i32.shl 
	local.set	8
	local.get	0
	i32.load	980
	local.set	9
.LBB27_140:
.Ltmp5061:
	.loc	32 66 21 is_stmt 1
	loop    	
	local.get	9
	local.get	6
	i32.store	0
	local.get	9
	i32.const	4
.Ltmp5062:
	.loc	16 656 28
	i32.add 
	local.set	9
	local.get	8
	i32.const	-4
.Ltmp5063:
	.loc	16 1714 9
	i32.add 
.Ltmp5064:
	.loc	15 180 28
	local.tee	8
	br_if   	0
.LBB27_141:
	end_loop
	end_block
.Ltmp5065:
	.loc	14 3417 5
	local.get	4
	i32.const	2160
	i32.add 
	local.get	4
	i32.const	736
	i32.add 
	i32.const	368
	memory.copy	0, 0
	.loc	14 3417 14 is_stmt 0
	local.get	4
	i32.const	2160
	i32.add 
	local.get	15
	call	_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_
	.loc	14 3419 5 is_stmt 1
	local.get	0
	local.get	17
	i32.store	804
	.loc	14 3418 5
	local.get	0
	local.get	18
	i32.store	800
	br      	4
.LBB27_142:
	.loc	14 0 5 is_stmt 0
	end_block
	i32.const	0
	i32.const	4
.Ltmp5066:
	.loc	38 456 13 is_stmt 1
	local.get	9
	i32.const	.Lalloc_5f4e05826d69b5e832dc96c63f325058
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5067:
.LBB27_143:
	.loc	38 0 13 is_stmt 0
	end_block
.Ltmp5068:
	.loc	14 3414 23 is_stmt 1
	block   	
	local.get	0
	i32.load	968
	local.tee	9
	i32.const	3
.Ltmp5069:
	.loc	38 451 16
	i32.le_u
	br_if   	0
.Ltmp5070:
	.loc	14 3414 23
	local.get	0
	i32.load	964
.Ltmp5071:
	.loc	14 0 0 is_stmt 0
	local.get	4
	v128.load	2160
.Ltmp5072:
	.loc	1 551 14 is_stmt 1
	v128.store	0:p2align=2
.Ltmp5073:
	.loc	14 3415 5
	block   	
	local.get	0
	i32.load	984
	local.tee	9
	i32.eqz
	br_if   	0
	.loc	14 0 5 is_stmt 0
	local.get	9
	i32.const	2
	i32.shl 
	local.set	8
	local.get	0
	i32.load	980
	local.set	9
.LBB27_146:
.Ltmp5074:
	.loc	32 66 21 is_stmt 1
	loop    	
	local.get	9
	local.get	6
	i32.store	0
	local.get	9
	i32.const	4
.Ltmp5075:
	.loc	16 656 28
	i32.add 
	local.set	9
	local.get	8
	i32.const	-4
.Ltmp5076:
	.loc	16 1714 9
	i32.add 
	local.tee	8
	br_if   	0
.Ltmp5077:
.LBB27_147:
	.loc	16 0 9 is_stmt 0
	end_loop
	end_block
	.loc	14 3417 14 is_stmt 1
	local.get	4
	local.get	15
	call	_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_
	.loc	14 3419 5
	local.get	0
	local.get	17
	i32.store	804
	.loc	14 3418 5
	local.get	0
	local.get	18
	i32.store	800
.Ltmp5078:
	.loc	14 3192 13
	br      	3
.LBB27_148:
	.loc	14 0 13 is_stmt 0
	end_block
	i32.const	0
	i32.const	4
.Ltmp5079:
	.loc	38 456 13 is_stmt 1
	local.get	9
	i32.const	.Lalloc_5f4e05826d69b5e832dc96c63f325058
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5080:
.LBB27_149:
	.loc	38 0 13 is_stmt 0
	end_block
	.loc	14 3197 12 is_stmt 1
	local.get	8
	br_if   	0
.Ltmp5081:
	.loc	14 3247 24
	local.get	4
	i32.const	368
	i32.add 
	local.get	15
	call	_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_
.Ltmp5082:
	.loc	14 3255 27
	local.get	0
	i32.load	804
	local.set	17
.Ltmp5083:
	.loc	14 3254 27
	local.get	0
	i32.load	800
	local.set	106
.Ltmp5084:
	.loc	14 3253 21
	local.get	0
	i32.load8_u	785
	local.set	9
.Ltmp5085:
	.loc	14 3252 19
	local.get	0
	i32.load8_u	784
	local.set	8
	local.get	4
	v128.const	0, 0
	local.tee	21
	v128.store	2176:p2align=3
	local.get	4
	local.get	21
	v128.store	2160:p2align=3
	local.get	4
	i32.const	1136
	i32.add 
	i32.const	0
	i32.const	1024
	memory.fill	0
	local.get	4
	v128.load	368
	local.set	93
	block   	
	block   	
	block   	
	block   	
	block   	
	local.get	3
	i32.eqz
	br_if   	0
	.loc	14 0 19 is_stmt 0
	local.get	3
	i32.const	5
.Ltmp5086:
	.loc	13 3756 21 is_stmt 1
	i32.shr_u
	local.get	3
	i32.const	31
.Ltmp5087:
	.loc	13 3757 21
	i32.and 
	i32.const	0
.Ltmp5088:
	.loc	13 3758 16
	i32.ne  
	i32.add 
	local.set	79
.Ltmp5089:
	.loc	14 3253 21
	v128.const	-1, -1, -1, -1
	local.tee	21
	v128.const	0, 0, 0, 0
	local.tee	22
	local.get	9
	i32.const	1
	i32.and 
	v128.select
	local.set	118
	local.get	21
	local.get	22
	local.get	8
	i32.const	1
.Ltmp5090:
	.loc	14 3252 19
	i32.and 
	v128.select
	local.set	72
	local.get	0
	v128.load	768
	local.set	23
	local.get	0
	v128.load	752
	local.set	25
	local.get	0
	v128.load	736
	local.set	26
	local.get	0
	v128.load	720
	local.set	27
	local.get	0
	v128.load	704
	local.set	28
	local.get	0
	v128.load	688
	local.set	29
	local.get	0
	v128.load	672
	local.set	30
	local.get	0
	v128.load	656
	local.set	31
	local.get	0
	v128.load	640
	local.set	32
	local.get	0
	v128.load	624
	local.set	33
	local.get	0
	v128.load	608
	local.set	34
	local.get	0
	v128.load	592
	local.set	35
	local.get	0
	v128.load	576
	local.set	36
	local.get	0
	v128.load	560
	local.set	37
	local.get	0
	v128.load	544
	local.set	38
	local.get	0
	v128.load	528
	local.set	39
	local.get	0
	v128.load	512
	local.set	40
	local.get	0
	v128.load	496
	local.set	41
	local.get	0
	v128.load	480
	local.set	42
	local.get	0
	v128.load	464
	local.set	43
	local.get	0
	v128.load	448
	local.set	44
	local.get	0
	v128.load	432
	local.set	45
	local.get	0
	v128.load	416
	local.set	46
	local.get	0
	v128.load	400
	local.set	47
	local.get	0
	v128.load	384
	local.set	48
	local.get	0
	v128.load	368
	local.set	49
	local.get	0
	v128.load	352
	local.set	50
	local.get	0
	v128.load	336
	local.set	51
	local.get	0
	v128.load	320
	local.set	52
	local.get	0
	v128.load	304
	local.set	53
	local.get	0
	v128.load	288
	local.set	54
	local.get	0
	v128.load	272
	local.set	55
	local.get	0
	v128.load	256
	local.set	56
	local.get	0
	v128.load	240
	local.set	57
	local.get	0
	v128.load	224
	local.set	58
	local.get	0
	v128.load	208
	local.set	59
	local.get	0
	v128.load	192
	local.set	60
	local.get	0
	v128.load	176
	local.set	61
	local.get	0
	v128.load	160
	local.set	62
	local.get	0
	v128.load	144
	local.set	63
	local.get	0
	v128.load	128
	local.set	64
	local.get	0
	v128.load	112
	local.set	65
	local.get	0
	v128.load	96
	local.set	66
	local.get	0
	v128.load	80
	local.set	67
	local.get	0
	v128.load	64
	local.set	68
	local.get	0
	v128.load	48
	local.set	69
	local.get	0
	v128.load	32
	local.set	70
	local.get	0
	v128.load	16
	local.set	71
	local.get	4
	v128.load	704
	local.set	95
	local.get	4
	v128.load	544
	local.set	94
	local.get	4
	v128.load	528
	local.set	85
	local.get	4
	v128.load	512
	local.set	86
	local.get	4
	v128.load	496
	local.set	87
	local.get	4
	v128.load	480
	local.set	88
	local.get	4
	v128.load	464
	local.set	21
	local.get	4
	v128.load	448
	local.set	89
	local.get	4
	v128.load	432
	local.set	90
	local.get	4
	v128.load	416
	local.set	91
	local.get	4
	v128.load	400
	local.set	92
	local.get	4
	v128.load	384
	local.set	22
	local.get	4
	v128.load	720
	local.set	119
	local.get	4
	v128.load	624
	local.set	120
	local.get	4
	v128.load	560
	local.set	24
	local.get	1
	local.set	78
	local.get	3
	local.set	84
	i32.const	0
	local.set	19
.LBB27_152:
	.loc	14 0 19 is_stmt 0
	block   	
	block   	
	loop    	
	local.get	95
	local.set	121
.Ltmp5091:
	.loc	14 3260 51 is_stmt 1
	local.get	3
	local.get	19
	i32.sub 
	local.tee	9
	i32.const	32
	local.get	9
	i32.const	32
.Ltmp5092:
	.loc	9 1077 12
	i32.lt_u
	i32.select
.Ltmp5093:
	.loc	14 3265 35
	local.get	19
	i32.add 
	i32.const	2
	i32.shl 
	local.tee	9
	local.get	19
	i32.const	2
.Ltmp5094:
	.loc	14 3261 27
	i32.shl 
.Ltmp5095:
	.loc	13 1050 16
	local.tee	8
	i32.lt_u
	br_if   	5
	local.get	9
	local.get	2
	i32.gt_u
	br_if   	5
.Ltmp5096:
	.loc	13 0 16 is_stmt 0
	local.get	84
	i32.const	1
	local.get	84
	i32.const	1
	i32.gt_u
	i32.select
	local.tee	9
	i32.const	32
	local.get	9
	i32.const	32
	i32.lt_u
	i32.select
	local.set	83
.Ltmp5097:
	.loc	33 304 12 is_stmt 1
	block   	
	local.get	3
	local.get	19
	i32.eq  
	local.tee	7
	br_if   	0
.Ltmp5098:
	.loc	33 0 12 is_stmt 0
	local.get	4
	i32.const	1136
	i32.add 
	local.set	9
	local.get	78
	local.set	8
	local.get	83
	local.set	10
	local.get	85
	local.set	95
	local.get	86
	local.set	96
	local.get	87
	local.set	97
	local.get	88
	local.set	98
	local.get	21
	local.set	99
	local.get	89
	local.set	100
	local.get	90
	local.set	101
	local.get	91
	local.set	102
	local.get	92
	local.set	103
	local.get	22
	local.set	104
.LBB27_156:
	loop    	
	local.get	93
	local.set	22
.Ltmp5099:
	.loc	1 551 14 is_stmt 1
	local.get	9
	local.get	68
	local.get	8
	v128.load	0:p2align=2
.Ltmp5100:
	.loc	42 3867 14
	local.tee	93
	f32x4.mul
.Ltmp5101:
	.loc	42 3845 14
	v128.const	0x0p0, 0x0p0, 0x0p0, 0x0p0
.Ltmp5102:
	.loc	42 3845 14 is_stmt 0
	local.tee	105
	f32x4.add
.Ltmp5103:
	.loc	42 3867 14 is_stmt 1
	local.get	64
	local.get	22
	f32x4.mul
.Ltmp5104:
	.loc	42 3845 14
	f32x4.add
	local.get	60
	local.get	104
.Ltmp5105:
	.loc	42 3867 14
	local.tee	92
	f32x4.mul
.Ltmp5106:
	.loc	42 3845 14
	f32x4.add
	local.get	56
	local.get	103
.Ltmp5107:
	.loc	42 3867 14
	local.tee	91
	f32x4.mul
.Ltmp5108:
	.loc	42 3845 14
	f32x4.add
	local.get	52
	local.get	102
.Ltmp5109:
	.loc	42 3867 14
	local.tee	90
	f32x4.mul
.Ltmp5110:
	.loc	42 3845 14
	f32x4.add
	local.get	48
	local.get	101
.Ltmp5111:
	.loc	42 3867 14
	local.tee	89
	f32x4.mul
.Ltmp5112:
	.loc	42 3845 14
	f32x4.add
	local.get	44
	local.get	100
.Ltmp5113:
	.loc	42 3867 14
	local.tee	21
	f32x4.mul
.Ltmp5114:
	.loc	42 3845 14
	f32x4.add
	local.get	40
	local.get	99
.Ltmp5115:
	.loc	42 3867 14
	local.tee	88
	f32x4.mul
.Ltmp5116:
	.loc	42 3845 14
	f32x4.add
	local.get	36
	local.get	98
.Ltmp5117:
	.loc	42 3867 14
	local.tee	87
	f32x4.mul
.Ltmp5118:
	.loc	42 3845 14
	f32x4.add
	local.get	32
	local.get	97
.Ltmp5119:
	.loc	42 3867 14
	local.tee	86
	f32x4.mul
.Ltmp5120:
	.loc	42 3845 14
	f32x4.add
	local.get	28
	local.get	96
.Ltmp5121:
	.loc	42 3867 14
	local.tee	85
	f32x4.mul
.Ltmp5122:
	.loc	42 3845 14
	f32x4.add
	local.get	23
	local.get	95
.Ltmp5123:
	.loc	42 3867 14
	local.tee	94
	f32x4.mul
.Ltmp5124:
	.loc	42 3845 14
	f32x4.add
.Ltmp5125:
	.loc	42 3812 14
	f32x4.abs
.Ltmp5126:
	.loc	42 3867 14
	local.get	69
	local.get	93
	f32x4.mul
.Ltmp5127:
	.loc	42 3845 14
	local.get	105
	f32x4.add
.Ltmp5128:
	.loc	42 3867 14
	local.get	65
	local.get	22
	f32x4.mul
.Ltmp5129:
	.loc	42 3845 14
	f32x4.add
.Ltmp5130:
	.loc	42 3867 14
	local.get	61
	local.get	92
	f32x4.mul
.Ltmp5131:
	.loc	42 3845 14
	f32x4.add
.Ltmp5132:
	.loc	42 3867 14
	local.get	57
	local.get	91
	f32x4.mul
.Ltmp5133:
	.loc	42 3845 14
	f32x4.add
.Ltmp5134:
	.loc	42 3867 14
	local.get	53
	local.get	90
	f32x4.mul
.Ltmp5135:
	.loc	42 3845 14
	f32x4.add
.Ltmp5136:
	.loc	42 3867 14
	local.get	49
	local.get	89
	f32x4.mul
.Ltmp5137:
	.loc	42 3845 14
	f32x4.add
.Ltmp5138:
	.loc	42 3867 14
	local.get	45
	local.get	21
	f32x4.mul
.Ltmp5139:
	.loc	42 3845 14
	f32x4.add
.Ltmp5140:
	.loc	42 3867 14
	local.get	41
	local.get	88
	f32x4.mul
.Ltmp5141:
	.loc	42 3845 14
	f32x4.add
.Ltmp5142:
	.loc	42 3867 14
	local.get	37
	local.get	87
	f32x4.mul
.Ltmp5143:
	.loc	42 3845 14
	f32x4.add
.Ltmp5144:
	.loc	42 3867 14
	local.get	33
	local.get	86
	f32x4.mul
.Ltmp5145:
	.loc	42 3845 14
	f32x4.add
.Ltmp5146:
	.loc	42 3867 14
	local.get	29
	local.get	85
	f32x4.mul
.Ltmp5147:
	.loc	42 3845 14
	f32x4.add
.Ltmp5148:
	.loc	42 3867 14
	local.get	25
	local.get	94
	f32x4.mul
.Ltmp5149:
	.loc	42 3845 14
	f32x4.add
.Ltmp5150:
	.loc	42 3812 14
	f32x4.abs
.Ltmp5151:
	.loc	42 3867 14
	local.get	70
	local.get	93
	f32x4.mul
.Ltmp5152:
	.loc	42 3845 14
	local.get	105
	f32x4.add
.Ltmp5153:
	.loc	42 3867 14
	local.get	66
	local.get	22
	f32x4.mul
.Ltmp5154:
	.loc	42 3845 14
	f32x4.add
.Ltmp5155:
	.loc	42 3867 14
	local.get	62
	local.get	92
	f32x4.mul
.Ltmp5156:
	.loc	42 3845 14
	f32x4.add
.Ltmp5157:
	.loc	42 3867 14
	local.get	58
	local.get	91
	f32x4.mul
.Ltmp5158:
	.loc	42 3845 14
	f32x4.add
.Ltmp5159:
	.loc	42 3867 14
	local.get	54
	local.get	90
	f32x4.mul
.Ltmp5160:
	.loc	42 3845 14
	f32x4.add
.Ltmp5161:
	.loc	42 3867 14
	local.get	50
	local.get	89
	f32x4.mul
.Ltmp5162:
	.loc	42 3845 14
	f32x4.add
.Ltmp5163:
	.loc	42 3867 14
	local.get	46
	local.get	21
	f32x4.mul
.Ltmp5164:
	.loc	42 3845 14
	f32x4.add
.Ltmp5165:
	.loc	42 3867 14
	local.get	42
	local.get	88
	f32x4.mul
.Ltmp5166:
	.loc	42 3845 14
	f32x4.add
.Ltmp5167:
	.loc	42 3867 14
	local.get	38
	local.get	87
	f32x4.mul
.Ltmp5168:
	.loc	42 3845 14
	f32x4.add
.Ltmp5169:
	.loc	42 3867 14
	local.get	34
	local.get	86
	f32x4.mul
.Ltmp5170:
	.loc	42 3845 14
	f32x4.add
.Ltmp5171:
	.loc	42 3867 14
	local.get	30
	local.get	85
	f32x4.mul
.Ltmp5172:
	.loc	42 3845 14
	f32x4.add
.Ltmp5173:
	.loc	42 3867 14
	local.get	26
	local.get	94
	f32x4.mul
.Ltmp5174:
	.loc	42 3845 14
	f32x4.add
.Ltmp5175:
	.loc	42 3812 14
	f32x4.abs
.Ltmp5176:
	.loc	42 3867 14
	local.get	71
	local.get	93
	f32x4.mul
.Ltmp5177:
	.loc	42 3845 14
	local.get	105
	f32x4.add
.Ltmp5178:
	.loc	42 3867 14
	local.get	67
	local.get	22
	f32x4.mul
.Ltmp5179:
	.loc	42 3845 14
	f32x4.add
.Ltmp5180:
	.loc	42 3867 14
	local.get	63
	local.get	92
	f32x4.mul
.Ltmp5181:
	.loc	42 3845 14
	f32x4.add
.Ltmp5182:
	.loc	42 3867 14
	local.get	59
	local.get	91
	f32x4.mul
.Ltmp5183:
	.loc	42 3845 14
	f32x4.add
.Ltmp5184:
	.loc	42 3867 14
	local.get	55
	local.get	90
	f32x4.mul
.Ltmp5185:
	.loc	42 3845 14
	f32x4.add
.Ltmp5186:
	.loc	42 3867 14
	local.get	51
	local.get	89
	f32x4.mul
.Ltmp5187:
	.loc	42 3845 14
	f32x4.add
.Ltmp5188:
	.loc	42 3867 14
	local.get	47
	local.get	21
	f32x4.mul
.Ltmp5189:
	.loc	42 3845 14
	f32x4.add
.Ltmp5190:
	.loc	42 3867 14
	local.get	43
	local.get	88
	f32x4.mul
.Ltmp5191:
	.loc	42 3845 14
	f32x4.add
.Ltmp5192:
	.loc	42 3867 14
	local.get	39
	local.get	87
	f32x4.mul
.Ltmp5193:
	.loc	42 3845 14
	f32x4.add
.Ltmp5194:
	.loc	42 3867 14
	local.get	35
	local.get	86
	f32x4.mul
.Ltmp5195:
	.loc	42 3845 14
	f32x4.add
.Ltmp5196:
	.loc	42 3867 14
	local.get	31
	local.get	85
	f32x4.mul
.Ltmp5197:
	.loc	42 3845 14
	f32x4.add
.Ltmp5198:
	.loc	42 3867 14
	local.get	27
	local.get	94
	f32x4.mul
.Ltmp5199:
	.loc	42 3845 14
	f32x4.add
.Ltmp5200:
	.loc	42 3812 14
	f32x4.abs
.Ltmp5201:
	.loc	42 3812 14 is_stmt 0
	local.get	21
	f32x4.abs
.Ltmp5202:
	.loc	42 3928 9 is_stmt 1
	f32x4.pmax
	f32x4.pmax
	f32x4.pmax
	f32x4.pmax
.Ltmp5203:
	.loc	1 551 14
	v128.store	0:p2align=2
	local.get	9
	i32.const	16
.Ltmp5204:
	.loc	33 304 12
	i32.add 
	local.set	9
	local.get	8
	i32.const	16
	i32.add 
	local.set	8
	local.get	85
	local.set	95
	local.get	86
	local.set	96
	local.get	87
	local.set	97
	local.get	88
	local.set	98
	local.get	21
	local.set	99
	local.get	89
	local.set	100
	local.get	90
	local.set	101
	local.get	91
	local.set	102
	local.get	92
	local.set	103
	local.get	22
	local.set	104
	local.get	10
	i32.const	-1
	i32.add 
	local.tee	10
	br_if   	0
.LBB27_157:
	end_loop
	end_block
.Ltmp5205:
	.loc	10 900 12
	block   	
	block   	
	block   	
	local.get	7
	i32.eqz
	br_if   	0
	.loc	10 0 12 is_stmt 0
	local.get	121
	local.set	95
	.loc	10 900 12
	br      	1
.Ltmp5206:
.LBB27_159:
	.loc	10 0 12
	end_block
	local.get	0
	i32.load	920
	local.set	81
	local.get	0
	i32.load	916
	local.set	11
	i32.const	0
	local.set	109
	local.get	121
	local.set	95
.LBB27_160:
.Ltmp5207:
	.loc	14 3271 24 is_stmt 1
	loop    	
	block   	
	block   	
	block   	
	local.get	2
	local.get	109
	local.get	19
	i32.add 
	i32.const	2
	i32.shl 
.Ltmp5208:
	.loc	38 568 12
	local.tee	9
	i32.lt_u
	br_if   	0
	.loc	38 573 27
	block   	
	local.get	2
	local.get	9
	i32.sub 
	local.tee	8
	i32.const	3
.Ltmp5209:
	.loc	38 438 16
	i32.le_u
	br_if   	0
.Ltmp5210:
	.loc	14 1392 25
	local.get	0
	i32.load	944
.Ltmp5211:
	.loc	14 1388 17
	local.tee	8
	local.get	0
	i32.load	1020
.Ltmp5212:
	.loc	14 1392 45
	local.tee	74
	local.get	17
	i32.mul 
.Ltmp5213:
	.loc	38 580 12
	local.tee	73
	i32.lt_u
	br_if   	13
	.loc	38 585 27
	block   	
	local.get	8
	local.get	73
	i32.sub 
	local.tee	8
	i32.const	3
.Ltmp5214:
	.loc	38 451 16
	i32.le_u
	br_if   	0
.Ltmp5215:
	.loc	38 0 16 is_stmt 0
	local.get	1
	local.get	9
	i32.const	2
	i32.shl 
	i32.add 
	local.tee	82
	v128.load	0:p2align=2
	local.set	97
.Ltmp5216:
	.loc	14 1392 25 is_stmt 1
	local.get	0
	i32.load	940
	local.get	73
	i32.const	2
.Ltmp5217:
	.loc	38 101 24
	i32.shl 
	local.tee	112
	i32.add 
.Ltmp5218:
	.loc	14 0 0 is_stmt 0
	local.get	24
	local.get	4
	i32.const	1136
	i32.add 
	local.get	109
	i32.const	4
	i32.shl 
	i32.add 
	v128.load	0:p2align=2
.Ltmp5219:
	local.tee	105
	local.get	105
	local.get	72
	v128.bitselect
.Ltmp5220:
	local.tee	105
	f32x4.div
	v128.const	0, 0, 128, 63, 0, 0, 128, 63, 0, 0, 128, 63, 0, 0, 128, 63
	local.get	24
	local.get	105
	f32x4.lt
	v128.bitselect
.Ltmp5221:
	.loc	1 551 14 is_stmt 1
	v128.store	0:p2align=2
.Ltmp5222:
	.loc	14 1259 17
	block   	
	block   	
	block   	
	local.get	0
	i32.load	1020
.Ltmp5223:
	.loc	43 37 12
	local.tee	6
	i32.eqz
	br_if   	0
	.loc	43 0 12 is_stmt 0
	i32.const	0
	local.set	110
	local.get	6
	local.get	17
	i32.const	1
	i32.add 
	local.tee	9
	i32.const	0
	local.get	11
	local.get	9
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	i32.mul 
	local.set	116
	local.get	0
	i32.load	964
	local.set	77
	local.get	0
	i32.load	968
	local.set	117
	local.get	0
	i32.load	980
	local.set	115
	local.get	0
	i32.load	984
	local.set	107
	local.get	0
	i32.load	940
	local.set	13
	local.get	0
	i32.load	944
	local.set	12
	local.get	0
	i32.load	1012
	local.set	114
	local.get	0
	i32.load	1016
	local.set	108
	i32.const	0
	local.set	7
	local.get	6
	local.set	111
.LBB27_166:
	loop    	
	local.get	110
	i32.const	32
.Ltmp5224:
	.loc	16 1714 9 is_stmt 1
	i32.eq  
.Ltmp5225:
	.loc	15 180 28
	br_if   	1
.Ltmp5226:
	.loc	14 1263 21
	block   	
	block   	
	block   	
	block   	
	local.get	7
	local.get	108
	i32.eq  
	br_if   	0
	.loc	14 0 21 is_stmt 0
	local.get	114
	local.get	7
	i32.const	12
	.loc	14 1263 21
	i32.mul 
	i32.add 
	local.tee	8
	i32.load	4
.Ltmp5227:
	.loc	14 1265 23 is_stmt 1
	local.get	17
	i32.add 
	local.tee	9
	i32.const	0
.Ltmp5228:
	.loc	14 1266 12
	local.get	11
	local.get	9
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
.Ltmp5229:
	.loc	14 1273 42
	local.tee	9
	local.get	6
	i32.mul 
	local.get	7
	i32.add 
	.loc	14 1273 22 is_stmt 0
	local.tee	10
	local.get	12
	i32.ge_u
	br_if   	1
.Ltmp5230:
	.loc	14 1274 24 is_stmt 1
	local.get	7
	local.get	107
	i32.eq  
	br_if   	2
.Ltmp5231:
	.loc	14 0 0 is_stmt 0
	local.get	8
	i32.load	0
	local.set	8
	local.get	13
	local.get	10
	i32.const	2
.Ltmp5232:
	i32.shl 
	i32.add 
	local.tee	18
	f32.load	0
	local.set	122
	local.get	115
	local.get	7
	i32.const	2
.Ltmp5233:
	.loc	14 1274 24
	i32.shl 
	local.tee	10
	i32.add 
	local.tee	113
	i32.load	0
.Ltmp5234:
	.loc	14 1275 26 is_stmt 1
	local.tee	75
	i32.eqz
	br_if   	3
	.loc	14 1278 24
	block   	
	block   	
	local.get	7
	local.get	117
	i32.ge_u
	br_if   	0
	local.get	77
	local.get	10
	i32.add 
	f32.load	0
.Ltmp5235:
	.loc	14 903 8
	local.tee	123
	local.get	122
	f32.lt  
	br_if   	1
	br      	5
.Ltmp5236:
.LBB27_173:
	.loc	14 0 8 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	704
.Ltmp5237:
	local.get	4
	local.get	94
	v128.store	544
	local.get	4
	local.get	85
	v128.store	528
	local.get	4
	local.get	86
	v128.store	512
	local.get	4
	local.get	87
	v128.store	496
	local.get	4
	local.get	88
	v128.store	480
	local.get	4
	local.get	21
	v128.store	464
	local.get	4
	local.get	89
	v128.store	448
	local.get	4
	local.get	90
	v128.store	432
	local.get	4
	local.get	91
	v128.store	416
	local.get	4
	local.get	92
	v128.store	400
	local.get	4
	local.get	22
	v128.store	384
.Ltmp5238:
	.loc	14 1278 24 is_stmt 1
	local.get	7
	local.get	117
	i32.const	.Lalloc_a228cac3279aae3a34886f59f51fbe9e
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.LBB27_174:
	.loc	14 0 24 is_stmt 0
	end_block
	local.get	123
	local.set	122
.Ltmp5239:
	.loc	14 903 5 is_stmt 1
	br      	3
.Ltmp5240:
.LBB27_175:
	.loc	14 0 5 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	704
.Ltmp5241:
	local.get	4
	local.get	94
	v128.store	544
	local.get	4
	local.get	85
	v128.store	528
	local.get	4
	local.get	86
	v128.store	512
	local.get	4
	local.get	87
	v128.store	496
	local.get	4
	local.get	88
	v128.store	480
	local.get	4
	local.get	21
	v128.store	464
	local.get	4
	local.get	89
	v128.store	448
	local.get	4
	local.get	90
	v128.store	432
	local.get	4
	local.get	91
	v128.store	416
	local.get	4
	local.get	92
	v128.store	400
	local.get	4
	local.get	22
	v128.store	384
.Ltmp5242:
	.loc	14 1263 21 is_stmt 1
	local.get	108
	local.get	108
	i32.const	.Lalloc_12856b5f9033764dbf23e04eb77c5c46
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.Ltmp5243:
.LBB27_176:
	.loc	14 0 21 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	704
.Ltmp5244:
	local.get	4
	local.get	94
	v128.store	544
	local.get	4
	local.get	85
	v128.store	528
	local.get	4
	local.get	86
	v128.store	512
	local.get	4
	local.get	87
	v128.store	496
	local.get	4
	local.get	88
	v128.store	480
	local.get	4
	local.get	21
	v128.store	464
	local.get	4
	local.get	89
	v128.store	448
	local.get	4
	local.get	90
	v128.store	432
	local.get	4
	local.get	91
	v128.store	416
	local.get	4
	local.get	92
	v128.store	400
	local.get	4
	local.get	22
	v128.store	384
.Ltmp5245:
	.loc	14 1273 22 is_stmt 1
	local.get	10
	local.get	12
	i32.const	.Lalloc_70ebf0cf1c90c6161926611ccf249a32
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.Ltmp5246:
.LBB27_177:
	.loc	14 0 22 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	704
.Ltmp5247:
	local.get	4
	local.get	94
	v128.store	544
	local.get	4
	local.get	85
	v128.store	528
	local.get	4
	local.get	86
	v128.store	512
	local.get	4
	local.get	87
	v128.store	496
	local.get	4
	local.get	88
	v128.store	480
	local.get	4
	local.get	21
	v128.store	464
	local.get	4
	local.get	89
	v128.store	448
	local.get	4
	local.get	90
	v128.store	432
	local.get	4
	local.get	91
	v128.store	416
	local.get	4
	local.get	92
	v128.store	400
	local.get	4
	local.get	22
	v128.store	384
.Ltmp5248:
	.loc	14 1274 24 is_stmt 1
	local.get	107
	local.get	107
	i32.const	.Lalloc_023d591aaad7f5cf482ac80a9401eca1
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.LBB27_178:
	.loc	14 0 24 is_stmt 0
	end_block
.Ltmp5249:
	.loc	14 1280 9 is_stmt 1
	block   	
	block   	
	block   	
	local.get	7
	local.get	117
	i32.eq  
	br_if   	0
.Ltmp5250:
	.loc	14 0 0 is_stmt 0
	local.get	4
	i32.const	2160
	i32.add 
	local.get	110
	i32.add 
	local.set	76
.Ltmp5251:
	.loc	14 1280 9
	local.get	77
	local.get	10
	i32.add 
	local.get	122
	f32.store	0
	local.get	75
	i32.const	1
	.loc	14 1281 24 is_stmt 1
	i32.add 
	local.tee	75
	local.get	8
	i32.ne  
.Ltmp5252:
	.loc	14 1282 23
	br_if   	1
	.loc	14 1282 9 is_stmt 0
	local.get	76
	local.get	122
	f32.store	0
	i32.const	0
	local.set	75
	local.get	8
	i32.eqz
	br_if   	2
	.loc	14 1288 30 is_stmt 1
	local.get	18
	f32.load	0
	local.set	122
.LBB27_182:
.Ltmp5253:
	.loc	14 1291 65
	loop    	
	local.get	9
	local.get	6
	i32.mul 
	local.get	7
	i32.add 
	.loc	14 1291 45 is_stmt 0
	local.tee	10
	local.get	12
	i32.ge_u
	br_if   	7
	.loc	14 0 45
	local.get	13
	local.get	10
	i32.const	2
	.loc	14 1291 45
	i32.shl 
	i32.add 
	local.tee	10
	local.get	122
	local.get	10
	f32.load	0
.Ltmp5254:
	.loc	14 903 8 is_stmt 1
	local.tee	123
	local.get	122
	local.get	123
	f32.lt  
	f32.select
.Ltmp5255:
	.loc	14 1292 17
	local.tee	122
	f32.store	0
	.loc	14 1293 20
	local.get	9
	local.get	11
	local.get	9
	i32.select
	i32.const	-1
	.loc	14 1296 17
	i32.add 
	local.set	9
	local.get	8
	i32.const	-1
.Ltmp5256:
	.loc	9 1916 50
	i32.add 
.Ltmp5257:
	.loc	10 900 12
	local.tee	8
	br_if   	0
	br      	3
.Ltmp5258:
.LBB27_184:
	.loc	14 1280 9
	end_loop
	end_block
	local.get	4
	local.get	121
	v128.store	704
.Ltmp5259:
	.loc	14 0 0 is_stmt 0
	local.get	4
	local.get	94
	v128.store	544
	local.get	4
	local.get	85
	v128.store	528
	local.get	4
	local.get	86
	v128.store	512
	local.get	4
	local.get	87
	v128.store	496
	local.get	4
	local.get	88
	v128.store	480
	local.get	4
	local.get	21
	v128.store	464
	local.get	4
	local.get	89
	v128.store	448
	local.get	4
	local.get	90
	v128.store	432
	local.get	4
	local.get	91
	v128.store	416
	local.get	4
	local.get	92
	v128.store	400
	local.get	4
	local.get	22
	v128.store	384
.Ltmp5260:
	.loc	14 1280 9
	local.get	117
	local.get	117
	i32.const	.Lalloc_1fa5d88c1a2cc292a2767c48606a38fe
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.LBB27_185:
	.loc	14 0 9
	end_block
.Ltmp5261:
	.loc	14 1285 44 is_stmt 1
	local.get	7
	local.get	116
	i32.add 
	.loc	14 1285 24 is_stmt 0
	local.tee	9
	local.get	12
	i32.ge_u
	br_if   	3
	.loc	14 0 24
	local.get	76
	local.get	13
	local.get	9
	i32.const	2
	.loc	14 1285 24
	i32.shl 
	i32.add 
	f32.load	0
.Ltmp5262:
	.loc	14 903 8 is_stmt 1
	local.tee	123
	local.get	122
	local.get	123
	local.get	122
	f32.lt  
	f32.select
.Ltmp5263:
	.loc	14 1282 9
	f32.store	0
.Ltmp5264:
.LBB27_187:
	.loc	14 0 9 is_stmt 0
	end_block
	local.get	7
	i32.const	1
	i32.add 
	local.set	7
	local.get	110
	i32.const	4
	i32.add 
	local.set	110
.Ltmp5265:
	local.get	113
	local.get	75
	i32.store	0
	local.get	111
	i32.const	-1
.Ltmp5266:
	i32.add 
.Ltmp5267:
	.loc	43 37 12 is_stmt 1
	local.tee	111
	br_if   	0
.LBB27_188:
	.loc	43 37 12
	end_loop
	end_block
.Ltmp5268:
	.loc	14 1412 26
	local.get	0
	i32.load	952
	local.set	9
.Ltmp5269:
	.loc	1 551 14
	local.get	4
	v128.load	2160:p2align=3
	local.tee	96
	local.set	105
	local.get	74
	i32.eqz
	br_if   	6
.Ltmp5270:
	.loc	1 0 14 is_stmt 0
	block   	
	local.get	0
	i32.load	1016
	local.tee	10
	br_if   	0
	i32.const	0
	local.set	8
	br      	18
.LBB27_191:
	end_block
	block   	
	local.get	0
	i32.load	1012
.Ltmp5271:
	.loc	14 1403 42 is_stmt 1
	local.tee	7
	i32.load	8
	.loc	14 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5272:
	.loc	14 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	14 1407 40
	local.get	74
	i32.mul 
	.loc	14 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	14 0 25
	local.get	4
	local.get	0
	i32.load	948
	local.tee	12
	local.get	8
	i32.const	2
	.loc	14 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	14 1407 13
	f32.store	2160
	local.get	74
	i32.const	1
.Ltmp5273:
	.loc	43 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5274:
	.loc	43 0 12 is_stmt 0
	i32.const	1
	local.set	8
	local.get	10
	i32.const	1
.Ltmp5275:
	.loc	14 1403 42 is_stmt 1
	i32.eq  
	br_if   	18
	local.get	7
	i32.load	20
	.loc	14 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5276:
	.loc	14 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	14 1407 40
	local.get	74
	i32.mul 
	i32.const	1
	i32.add 
	.loc	14 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	14 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	14 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	14 1407 13
	f32.store	2164
	local.get	74
	i32.const	2
.Ltmp5277:
	.loc	43 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5278:
	.loc	43 0 12 is_stmt 0
	i32.const	2
	local.set	8
	local.get	10
	i32.const	2
.Ltmp5279:
	.loc	14 1403 42 is_stmt 1
	i32.eq  
	br_if   	18
	local.get	7
	i32.load	32
	.loc	14 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5280:
	.loc	14 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	14 1407 40
	local.get	74
	i32.mul 
	i32.const	2
	i32.add 
	.loc	14 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	14 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	14 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	14 1407 13
	f32.store	2168
	local.get	74
	i32.const	3
.Ltmp5281:
	.loc	43 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5282:
	.loc	43 0 12 is_stmt 0
	i32.const	3
	local.set	8
	local.get	10
	i32.const	3
.Ltmp5283:
	.loc	14 1403 42 is_stmt 1
	i32.eq  
	br_if   	18
	local.get	7
	i32.load	44
	.loc	14 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5284:
	.loc	14 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	14 1407 40
	local.get	74
	i32.mul 
	i32.const	3
	i32.add 
	.loc	14 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	14 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	14 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	14 1407 13
	f32.store	2172
	local.get	74
	i32.const	4
.Ltmp5285:
	.loc	43 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5286:
	.loc	43 0 12 is_stmt 0
	i32.const	4
	local.set	8
	local.get	10
	i32.const	4
.Ltmp5287:
	.loc	14 1403 42 is_stmt 1
	i32.eq  
	br_if   	18
	local.get	7
	i32.load	56
	.loc	14 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5288:
	.loc	14 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	14 1407 40
	local.get	74
	i32.mul 
	i32.const	4
	i32.add 
	.loc	14 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	14 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	14 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	14 1407 13
	f32.store	2176
	local.get	74
	i32.const	5
.Ltmp5289:
	.loc	43 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5290:
	.loc	43 0 12 is_stmt 0
	i32.const	5
	local.set	8
	local.get	10
	i32.const	5
.Ltmp5291:
	.loc	14 1403 42 is_stmt 1
	i32.eq  
	br_if   	18
	local.get	7
	i32.load	68
	.loc	14 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5292:
	.loc	14 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	14 1407 40
	local.get	74
	i32.mul 
	i32.const	5
	i32.add 
	.loc	14 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	14 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	14 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	14 1407 13
	f32.store	2180
	local.get	74
	i32.const	6
.Ltmp5293:
	.loc	43 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5294:
	.loc	43 0 12 is_stmt 0
	i32.const	6
	local.set	8
	local.get	10
	i32.const	6
.Ltmp5295:
	.loc	14 1403 42 is_stmt 1
	i32.eq  
	br_if   	18
	local.get	7
	i32.load	80
	.loc	14 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5296:
	.loc	14 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	14 1407 40
	local.get	74
	i32.mul 
	i32.const	6
	i32.add 
	.loc	14 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	14 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	14 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	14 1407 13
	f32.store	2184
	local.get	74
	i32.const	7
.Ltmp5297:
	.loc	43 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5298:
	.loc	43 0 12 is_stmt 0
	i32.const	7
	local.set	8
	local.get	10
	i32.const	7
.Ltmp5299:
	.loc	14 1403 42 is_stmt 1
	i32.eq  
	br_if   	18
	local.get	7
	i32.load	92
	.loc	14 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5300:
	.loc	14 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	14 1407 40
	local.get	74
	i32.mul 
	i32.const	7
	i32.add 
	.loc	14 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	14 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	14 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	14 1407 13
	f32.store	2188
.Ltmp5301:
	.loc	43 37 12 is_stmt 1
	br      	6
.Ltmp5302:
.LBB27_214:
	.loc	43 0 12 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	704
.Ltmp5303:
	local.get	4
	local.get	94
	v128.store	544
	local.get	4
	local.get	85
	v128.store	528
	local.get	4
	local.get	86
	v128.store	512
	local.get	4
	local.get	87
	v128.store	496
	local.get	4
	local.get	88
	v128.store	480
	local.get	4
	local.get	21
	v128.store	464
	local.get	4
	local.get	89
	v128.store	448
	local.get	4
	local.get	90
	v128.store	432
	local.get	4
	local.get	91
	v128.store	416
	local.get	4
	local.get	92
	v128.store	400
	local.get	4
	local.get	22
	v128.store	384
.Ltmp5304:
	.loc	14 1407 25 is_stmt 1
	local.get	8
	local.get	9
	i32.const	.Lalloc_0e76a26fbb751dfb8cf8a069b5b0d39a
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.Ltmp5305:
.LBB27_215:
	.loc	14 0 25 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	704
.Ltmp5306:
	local.get	4
	local.get	94
	v128.store	544
	local.get	4
	local.get	85
	v128.store	528
	local.get	4
	local.get	86
	v128.store	512
	local.get	4
	local.get	87
	v128.store	496
	local.get	4
	local.get	88
	v128.store	480
	local.get	4
	local.get	21
	v128.store	464
	local.get	4
	local.get	89
	v128.store	448
	local.get	4
	local.get	90
	v128.store	432
	local.get	4
	local.get	91
	v128.store	416
	local.get	4
	local.get	92
	v128.store	400
	local.get	4
	local.get	22
	v128.store	384
.Ltmp5307:
	.loc	14 1285 24 is_stmt 1
	local.get	9
	local.get	12
	i32.const	.Lalloc_70d40ae2302f3ea19fa0c4a65fe82786
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.Ltmp5308:
.LBB27_216:
	.loc	14 0 24 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	704
.Ltmp5309:
	local.get	4
	local.get	94
	v128.store	544
	local.get	4
	local.get	85
	v128.store	528
	local.get	4
	local.get	86
	v128.store	512
	local.get	4
	local.get	87
	v128.store	496
	local.get	4
	local.get	88
	v128.store	480
	local.get	4
	local.get	21
	v128.store	464
	local.get	4
	local.get	89
	v128.store	448
	local.get	4
	local.get	90
	v128.store	432
	local.get	4
	local.get	91
	v128.store	416
	local.get	4
	local.get	92
	v128.store	400
	local.get	4
	local.get	22
	v128.store	384
.Ltmp5310:
	.loc	14 1291 45 is_stmt 1
	local.get	10
	local.get	12
	i32.const	.Lalloc_a271bbb7fd83c3605ea58a06b7065fa4
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.Ltmp5311:
.LBB27_217:
	.loc	14 0 45 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	704
.Ltmp5312:
	local.get	4
	local.get	94
	v128.store	544
	local.get	4
	local.get	85
	v128.store	528
	local.get	4
	local.get	86
	v128.store	512
	local.get	4
	local.get	87
	v128.store	496
	local.get	4
	local.get	88
	v128.store	480
	local.get	4
	local.get	21
	v128.store	464
	local.get	4
	local.get	89
	v128.store	448
	local.get	4
	local.get	90
	v128.store	432
	local.get	4
	local.get	91
	v128.store	416
	local.get	4
	local.get	92
	v128.store	400
	local.get	4
	local.get	22
	v128.store	384
	i32.const	0
	i32.const	4
.Ltmp5313:
	.loc	38 456 13 is_stmt 1
	local.get	8
	i32.const	.Lalloc_5f4e05826d69b5e832dc96c63f325058
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5314:
.LBB27_218:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	704
.Ltmp5315:
	local.get	4
	local.get	94
	v128.store	544
	local.get	4
	local.get	85
	v128.store	528
	local.get	4
	local.get	86
	v128.store	512
	local.get	4
	local.get	87
	v128.store	496
	local.get	4
	local.get	88
	v128.store	480
	local.get	4
	local.get	21
	v128.store	464
	local.get	4
	local.get	89
	v128.store	448
	local.get	4
	local.get	90
	v128.store	432
	local.get	4
	local.get	91
	v128.store	416
	local.get	4
	local.get	92
	v128.store	400
	local.get	4
	local.get	22
	v128.store	384
	i32.const	0
	i32.const	4
.Ltmp5316:
	.loc	38 443 13 is_stmt 1
	local.get	8
	i32.const	.Lalloc_5f4e05826d69b5e832dc96c63f325058
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5317:
.LBB27_219:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	704
.Ltmp5318:
	local.get	4
	local.get	94
	v128.store	544
	local.get	4
	local.get	85
	v128.store	528
	local.get	4
	local.get	86
	v128.store	512
	local.get	4
	local.get	87
	v128.store	496
	local.get	4
	local.get	88
	v128.store	480
	local.get	4
	local.get	21
	v128.store	464
	local.get	4
	local.get	89
	v128.store	448
	local.get	4
	local.get	90
	v128.store	432
	local.get	4
	local.get	91
	v128.store	416
	local.get	4
	local.get	92
	v128.store	400
	local.get	4
	local.get	22
	v128.store	384
.Ltmp5319:
	.loc	38 569 13 is_stmt 1
	local.get	9
	local.get	2
	local.get	2
	i32.const	.Lalloc_16fe79fe907693415948d189acefd4a9
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5320:
.LBB27_220:
	.loc	38 0 13 is_stmt 0
	end_block
.Ltmp5321:
	.loc	1 551 14 is_stmt 1
	local.get	4
	v128.load	2160:p2align=3
	local.set	105
.Ltmp5322:
.LBB27_221:
	.loc	1 0 14 is_stmt 0
	end_block
.Ltmp5323:
	.loc	38 580 12 is_stmt 1
	block   	
	local.get	9
	local.get	73
	i32.ge_u
	br_if   	0
.Ltmp5324:
	.loc	38 0 12 is_stmt 0
	local.get	4
	local.get	121
	v128.store	704
.Ltmp5325:
	local.get	4
	local.get	94
	v128.store	544
	local.get	4
	local.get	85
	v128.store	528
	local.get	4
	local.get	86
	v128.store	512
	local.get	4
	local.get	87
	v128.store	496
	local.get	4
	local.get	88
	v128.store	480
	local.get	4
	local.get	21
	v128.store	464
	local.get	4
	local.get	89
	v128.store	448
	local.get	4
	local.get	90
	v128.store	432
	local.get	4
	local.get	91
	v128.store	416
	local.get	4
	local.get	92
	v128.store	400
	local.get	4
	local.get	22
	v128.store	384
.Ltmp5326:
	.loc	38 581 13 is_stmt 1
	local.get	73
	local.get	9
	local.get	9
	i32.const	.Lalloc_c2286ff41a33b8c11aa270fe215441de
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.LBB27_223:
	.loc	38 0 13 is_stmt 0
	end_block
	.loc	38 585 27 is_stmt 1
	local.get	9
	local.get	73
	i32.sub 
	local.tee	9
	i32.const	3
.Ltmp5327:
	.loc	38 451 16
	i32.le_u
	br_if   	7
.Ltmp5328:
	.loc	14 1412 26
	local.get	0
	i32.load	948
.Ltmp5329:
	.loc	38 101 24
	local.get	112
	i32.add 
.Ltmp5330:
	.loc	14 0 0 is_stmt 0
	local.get	96
	v128.const	0x1p14, 0x1p14, 0x1p14, 0x1p14
	f32x4.mul
	f32x4.floor
	v128.const	0x1p-14, 0x1p-14, 0x1p-14, 0x1p-14
	f32x4.mul
.Ltmp5331:
	.loc	1 551 14 is_stmt 1
	local.tee	96
	v128.store	0:p2align=2
.Ltmp5332:
	.loc	14 1416 43
	local.get	4
	local.get	4
	v128.load	688
.Ltmp5333:
	.loc	42 3856 14
	local.tee	98
	local.get	120
	v128.const	0x1p0, 0x1p0, 0x1p0, 0x1p0
.Ltmp5334:
	.loc	14 0 0 is_stmt 0
	local.tee	99
	local.get	96
	local.get	95
	f32x4.add
	local.get	105
	f32x4.sub
.Ltmp5335:
	.loc	42 3878 14 is_stmt 1
	local.tee	95
	local.get	119
	f32x4.div
.Ltmp5336:
	.loc	42 3856 14
	f32x4.sub
.Ltmp5337:
	.loc	42 3856 14 is_stmt 0
	local.tee	105
	local.get	98
	f32x4.sub
.Ltmp5338:
	.loc	42 3867 14 is_stmt 1
	f32x4.mul
.Ltmp5339:
	.loc	42 3845 14
	f32x4.add
.Ltmp5340:
	.loc	42 3928 9
	local.get	105
	f32x4.pmax
.Ltmp5341:
	.loc	42 3812 14
	local.tee	105
	local.get	105
	f32x4.abs
.Ltmp5342:
	.loc	42 1991 14
	v128.const	0x1.79ca1p-67, 0x1.79ca1p-67, 0x1.79ca1p-67, 0x1.79ca1p-67
	f32x4.lt
.Ltmp5343:
	.loc	44 481 22
	v128.andnot
.Ltmp5344:
	.loc	14 1417 5
	local.tee	96
	v128.store	688
.Ltmp5345:
	.loc	14 1420 28
	local.get	0
	i32.load	936
	.loc	14 1420 44 is_stmt 0
	local.tee	8
	local.get	74
	local.get	106
	i32.mul 
.Ltmp5346:
	.loc	38 568 12 is_stmt 1
	local.tee	9
	i32.lt_u
	br_if   	4
	.loc	38 573 27
	local.get	8
	local.get	9
	i32.sub 
	local.tee	8
	i32.const	3
.Ltmp5347:
	.loc	38 438 16
	i32.le_u
	br_if   	2
.Ltmp5348:
	.loc	14 1420 28
	local.get	0
	i32.load	932
	local.get	9
	i32.const	2
.Ltmp5349:
	.loc	38 89 24
	i32.shl 
	i32.add 
.Ltmp5350:
	.loc	1 551 14
	local.tee	9
	v128.load	0:p2align=2
	local.set	105
.Ltmp5351:
	.loc	1 551 14 is_stmt 0
	local.get	9
	local.get	97
	v128.store	0:p2align=2
.Ltmp5352:
	.loc	14 0 0
	local.get	82
	local.get	105
	local.get	105
	local.get	99
	local.get	96
	f32x4.sub
.Ltmp5353:
	.loc	42 3867 14 is_stmt 1
	f32x4.mul
.Ltmp5354:
	.loc	42 2188 14
	local.get	118
	v128.bitselect
.Ltmp5355:
	.loc	1 551 14
	v128.store	0:p2align=2
	i32.const	0
	local.get	17
	i32.const	1
.Ltmp5356:
	.loc	14 3299 13
	i32.add 
	.loc	14 3300 16
	local.tee	9
	local.get	9
	local.get	11
	i32.eq  
	i32.select
	local.set	17
	i32.const	0
	local.get	106
	i32.const	1
	.loc	14 3295 13
	i32.add 
	.loc	14 3296 16
	local.tee	9
	local.get	9
	local.get	81
	i32.eq  
	i32.select
	local.set	106
	local.get	109
	i32.const	1
.Ltmp5357:
	.loc	14 0 0 is_stmt 0
	i32.add 
.Ltmp5358:
	.loc	9 1916 50 is_stmt 1
	local.tee	109
	local.get	83
	i32.ne  
.Ltmp5359:
	.loc	10 900 12
	br_if   	0
.LBB27_227:
	end_loop
	end_block
	local.get	19
	i32.const	32
.Ltmp5360:
	.loc	14 0 0 is_stmt 0
	i32.add 
	local.set	19
	local.get	78
	i32.const	512
.Ltmp5361:
	.loc	41 446 20 is_stmt 1
	i32.add 
	local.set	78
	local.get	84
	i32.const	-32
	i32.add 
	local.set	84
	local.get	79
	i32.const	-1
.Ltmp5362:
	.loc	14 0 0 is_stmt 0
	i32.add 
.Ltmp5363:
	.loc	41 446 20
	local.tee	79
	i32.eqz
	br_if   	3
	br      	1
.Ltmp5364:
.LBB27_228:
	.loc	41 0 20
	end_block
.Ltmp5365:
	.loc	38 438 16 is_stmt 1
	end_loop
	local.get	4
	local.get	121
	v128.store	704
.Ltmp5366:
	.loc	14 0 0 is_stmt 0
	local.get	4
	local.get	94
	v128.store	544
	local.get	4
	local.get	85
	v128.store	528
	local.get	4
	local.get	86
	v128.store	512
	local.get	4
	local.get	87
	v128.store	496
	local.get	4
	local.get	88
	v128.store	480
	local.get	4
	local.get	21
	v128.store	464
	local.get	4
	local.get	89
	v128.store	448
	local.get	4
	local.get	90
	v128.store	432
	local.get	4
	local.get	91
	v128.store	416
	local.get	4
	local.get	92
	v128.store	400
	local.get	4
	local.get	22
	v128.store	384
	i32.const	0
	i32.const	4
.Ltmp5367:
	.loc	38 443 13 is_stmt 1
	local.get	8
	i32.const	.Lalloc_5f4e05826d69b5e832dc96c63f325058
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5368:
.LBB27_229:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	704
.Ltmp5369:
	local.get	4
	local.get	94
	v128.store	544
	local.get	4
	local.get	85
	v128.store	528
	local.get	4
	local.get	86
	v128.store	512
	local.get	4
	local.get	87
	v128.store	496
	local.get	4
	local.get	88
	v128.store	480
	local.get	4
	local.get	21
	v128.store	464
	local.get	4
	local.get	89
	v128.store	448
	local.get	4
	local.get	90
	v128.store	432
	local.get	4
	local.get	91
	v128.store	416
	local.get	4
	local.get	92
	v128.store	400
	local.get	4
	local.get	22
	v128.store	384
.Ltmp5370:
	.loc	38 569 13 is_stmt 1
	local.get	9
	local.get	8
	local.get	8
	i32.const	.Lalloc_2a3a21efd18b403ec25db545d522c479
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5371:
.LBB27_230:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	95
	v128.store	704
.Ltmp5372:
	local.get	4
	local.get	94
	v128.store	544
	local.get	4
	local.get	85
	v128.store	528
	local.get	4
	local.get	86
	v128.store	512
	local.get	4
	local.get	87
	v128.store	496
	local.get	4
	local.get	88
	v128.store	480
	local.get	4
	local.get	21
	v128.store	464
	local.get	4
	local.get	89
	v128.store	448
	local.get	4
	local.get	90
	v128.store	432
	local.get	4
	local.get	91
	v128.store	416
	local.get	4
	local.get	92
	v128.store	400
	local.get	4
	local.get	22
	v128.store	384
.LBB27_231:
	end_block
	local.get	4
	local.get	93
	v128.store	368
.Ltmp5373:
	.loc	14 3306 14 is_stmt 1
	local.get	4
	i32.const	368
	i32.add 
	local.get	15
	call	_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_
	.loc	14 3308 5
	local.get	0
	local.get	17
	i32.store	804
	.loc	14 3307 5
	local.get	0
	local.get	106
	i32.store	800
.Ltmp5374:
	.loc	14 3198 13
	br      	5
.LBB27_232:
	.loc	14 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	704
.Ltmp5375:
	local.get	4
	local.get	94
	v128.store	544
	local.get	4
	local.get	85
	v128.store	528
	local.get	4
	local.get	86
	v128.store	512
	local.get	4
	local.get	87
	v128.store	496
	local.get	4
	local.get	88
	v128.store	480
	local.get	4
	local.get	21
	v128.store	464
	local.get	4
	local.get	89
	v128.store	448
	local.get	4
	local.get	90
	v128.store	432
	local.get	4
	local.get	91
	v128.store	416
	local.get	4
	local.get	92
	v128.store	400
	local.get	4
	local.get	22
	v128.store	384
	i32.const	0
	i32.const	4
.Ltmp5376:
	.loc	38 456 13 is_stmt 1
	local.get	9
	i32.const	.Lalloc_5f4e05826d69b5e832dc96c63f325058
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5377:
.LBB27_233:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	704
.Ltmp5378:
	local.get	4
	local.get	94
	v128.store	544
	local.get	4
	local.get	85
	v128.store	528
	local.get	4
	local.get	86
	v128.store	512
	local.get	4
	local.get	87
	v128.store	496
	local.get	4
	local.get	88
	v128.store	480
	local.get	4
	local.get	21
	v128.store	464
	local.get	4
	local.get	89
	v128.store	448
	local.get	4
	local.get	90
	v128.store	432
	local.get	4
	local.get	91
	v128.store	416
	local.get	4
	local.get	92
	v128.store	400
	local.get	4
	local.get	22
	v128.store	384
.Ltmp5379:
	.loc	38 443 13 is_stmt 1
	local.get	8
	local.get	9
	local.get	2
	i32.const	.Lalloc_cab916602395946b4285251e481c2df8
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5380:
.LBB27_234:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	704
.Ltmp5381:
	local.get	4
	local.get	94
	v128.store	544
	local.get	4
	local.get	85
	v128.store	528
	local.get	4
	local.get	86
	v128.store	512
	local.get	4
	local.get	87
	v128.store	496
	local.get	4
	local.get	88
	v128.store	480
	local.get	4
	local.get	21
	v128.store	464
	local.get	4
	local.get	89
	v128.store	448
	local.get	4
	local.get	90
	v128.store	432
	local.get	4
	local.get	91
	v128.store	416
	local.get	4
	local.get	92
	v128.store	400
	local.get	4
	local.get	22
	v128.store	384
.Ltmp5382:
	.loc	38 581 13 is_stmt 1
	local.get	73
	local.get	8
	local.get	8
	i32.const	.Lalloc_db7c42041d7aaaba4839906f37294547
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5383:
.LBB27_235:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	704
.Ltmp5384:
	local.get	4
	local.get	94
	v128.store	544
	local.get	4
	local.get	85
	v128.store	528
	local.get	4
	local.get	86
	v128.store	512
	local.get	4
	local.get	87
	v128.store	496
	local.get	4
	local.get	88
	v128.store	480
	local.get	4
	local.get	21
	v128.store	464
	local.get	4
	local.get	89
	v128.store	448
	local.get	4
	local.get	90
	v128.store	432
	local.get	4
	local.get	91
	v128.store	416
	local.get	4
	local.get	92
	v128.store	400
	local.get	4
	local.get	22
	v128.store	384
.Ltmp5385:
	.loc	14 1403 42 is_stmt 1
	local.get	8
	local.get	8
	i32.const	.Lalloc_33746731a929bad63709ddedbca82f5f
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.Ltmp5386:
.LBB27_236:
	.loc	14 0 42 is_stmt 0
	end_block
.Ltmp5387:
	.loc	14 3247 24 is_stmt 1
	local.get	4
	i32.const	736
	i32.add 
	local.get	15
	call	_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_
.Ltmp5388:
	.loc	14 3255 27
	local.get	0
	i32.load	804
	local.set	17
.Ltmp5389:
	.loc	14 3254 27
	local.get	0
	i32.load	800
	local.set	106
.Ltmp5390:
	.loc	14 3253 21
	local.get	0
	i32.load8_u	785
	local.set	9
.Ltmp5391:
	.loc	14 3252 19
	local.get	0
	i32.load8_u	784
	local.set	8
	local.get	4
	v128.const	0, 0
	local.tee	21
	v128.store	1120:p2align=3
	local.get	4
	local.get	21
	v128.store	1104:p2align=3
	local.get	4
	i32.const	1136
	i32.add 
	i32.const	0
	i32.const	1024
	memory.fill	0
	block   	
	local.get	3
	i32.eqz
	br_if   	0
	.loc	14 0 19 is_stmt 0
	local.get	3
	i32.const	5
.Ltmp5392:
	.loc	13 3756 21 is_stmt 1
	i32.shr_u
	local.get	3
	i32.const	31
.Ltmp5393:
	.loc	13 3757 21
	i32.and 
	i32.const	0
.Ltmp5394:
	.loc	13 3758 16
	i32.ne  
	i32.add 
	local.set	79
.Ltmp5395:
	.loc	14 3253 21
	v128.const	-1, -1, -1, -1
	local.tee	21
	v128.const	0, 0, 0, 0
	local.tee	22
	local.get	9
	i32.const	1
	i32.and 
	v128.select
	local.set	118
	local.get	21
	local.get	22
	local.get	8
	i32.const	1
.Ltmp5396:
	.loc	14 3252 19
	i32.and 
	v128.select
	local.set	119
	local.get	0
	v128.load	768
	local.set	24
	local.get	0
	v128.load	752
	local.set	23
	local.get	0
	v128.load	736
	local.set	25
	local.get	0
	v128.load	720
	local.set	26
	local.get	0
	v128.load	704
	local.set	27
	local.get	0
	v128.load	688
	local.set	28
	local.get	0
	v128.load	672
	local.set	29
	local.get	0
	v128.load	656
	local.set	30
	local.get	0
	v128.load	640
	local.set	31
	local.get	0
	v128.load	624
	local.set	32
	local.get	0
	v128.load	608
	local.set	33
	local.get	0
	v128.load	592
	local.set	34
	local.get	0
	v128.load	576
	local.set	35
	local.get	0
	v128.load	560
	local.set	36
	local.get	0
	v128.load	544
	local.set	37
	local.get	0
	v128.load	528
	local.set	38
	local.get	0
	v128.load	512
	local.set	39
	local.get	0
	v128.load	496
	local.set	40
	local.get	0
	v128.load	480
	local.set	41
	local.get	0
	v128.load	464
	local.set	42
	local.get	0
	v128.load	448
	local.set	43
	local.get	0
	v128.load	432
	local.set	44
	local.get	0
	v128.load	416
	local.set	45
	local.get	0
	v128.load	400
	local.set	46
	local.get	0
	v128.load	384
	local.set	47
	local.get	0
	v128.load	368
	local.set	48
	local.get	0
	v128.load	352
	local.set	49
	local.get	0
	v128.load	336
	local.set	50
	local.get	0
	v128.load	320
	local.set	51
	local.get	0
	v128.load	304
	local.set	52
	local.get	0
	v128.load	288
	local.set	53
	local.get	0
	v128.load	272
	local.set	54
	local.get	0
	v128.load	256
	local.set	55
	local.get	0
	v128.load	240
	local.set	56
	local.get	0
	v128.load	224
	local.set	57
	local.get	0
	v128.load	208
	local.set	58
	local.get	0
	v128.load	192
	local.set	59
	local.get	0
	v128.load	176
	local.set	60
	local.get	0
	v128.load	160
	local.set	61
	local.get	0
	v128.load	144
	local.set	62
	local.get	0
	v128.load	128
	local.set	63
	local.get	0
	v128.load	112
	local.set	64
	local.get	0
	v128.load	96
	local.set	65
	local.get	0
	v128.load	80
	local.set	66
	local.get	0
	v128.load	64
	local.set	67
	local.get	0
	v128.load	48
	local.set	68
	local.get	0
	v128.load	32
	local.set	69
	local.get	0
	v128.load	16
	local.set	70
	local.get	4
	v128.load	1072
	local.set	97
	local.get	4
	v128.load	1040
	local.set	96
	local.get	4
	v128.load	976
	local.set	95
	local.get	4
	v128.load	912
	local.set	94
	local.get	4
	v128.load	896
	local.set	85
	local.get	4
	v128.load	880
	local.set	86
	local.get	4
	v128.load	864
	local.set	87
	local.get	4
	v128.load	848
	local.set	88
	local.get	4
	v128.load	832
	local.set	21
	local.get	4
	v128.load	816
	local.set	89
	local.get	4
	v128.load	800
	local.set	90
	local.get	4
	v128.load	784
	local.set	91
	local.get	4
	v128.load	768
	local.set	92
	local.get	4
	v128.load	752
	local.set	22
	local.get	4
	v128.load	736
	local.set	93
	local.get	4
	v128.load	1088
	local.set	120
	local.get	4
	v128.load	1008
	local.set	71
	local.get	4
	v128.load	944
	local.set	72
	local.get	1
	local.set	78
	local.get	3
	local.set	84
	i32.const	0
	local.set	19
.LBB27_238:
	.loc	14 0 19 is_stmt 0
	block   	
	block   	
	loop    	
	local.get	95
	local.set	124
	local.get	96
	local.set	125
	local.get	97
	local.set	121
.Ltmp5397:
	.loc	14 3260 51 is_stmt 1
	local.get	3
	local.get	19
	i32.sub 
	local.tee	9
	i32.const	32
	local.get	9
	i32.const	32
.Ltmp5398:
	.loc	9 1077 12
	i32.lt_u
	i32.select
.Ltmp5399:
	.loc	14 3265 35
	local.get	19
	i32.add 
	i32.const	2
	i32.shl 
	local.tee	9
	local.get	19
	i32.const	2
.Ltmp5400:
	.loc	14 3261 27
	i32.shl 
.Ltmp5401:
	.loc	13 1050 16
	local.tee	8
	i32.lt_u
	br_if   	6
	local.get	9
	local.get	2
	i32.gt_u
	br_if   	6
.Ltmp5402:
	.loc	13 0 16 is_stmt 0
	local.get	84
	i32.const	1
	local.get	84
	i32.const	1
	i32.gt_u
	i32.select
	local.tee	9
	i32.const	32
	local.get	9
	i32.const	32
	i32.lt_u
	i32.select
	local.set	83
.Ltmp5403:
	.loc	33 304 12 is_stmt 1
	block   	
	local.get	3
	local.get	19
	i32.eq  
	local.tee	7
	br_if   	0
.Ltmp5404:
	.loc	33 0 12 is_stmt 0
	local.get	4
	i32.const	1136
	i32.add 
	local.set	9
	local.get	78
	local.set	8
	local.get	83
	local.set	10
	local.get	85
	local.set	95
	local.get	86
	local.set	96
	local.get	87
	local.set	97
	local.get	88
	local.set	98
	local.get	21
	local.set	99
	local.get	89
	local.set	100
	local.get	90
	local.set	101
	local.get	91
	local.set	102
	local.get	92
	local.set	103
	local.get	22
	local.set	104
.LBB27_242:
	loop    	
	local.get	93
	local.set	22
.Ltmp5405:
	.loc	1 551 14 is_stmt 1
	local.get	9
	local.get	67
	local.get	8
	v128.load	0:p2align=2
.Ltmp5406:
	.loc	42 3867 14
	local.tee	93
	f32x4.mul
.Ltmp5407:
	.loc	42 3845 14
	v128.const	0x0p0, 0x0p0, 0x0p0, 0x0p0
.Ltmp5408:
	.loc	42 3845 14 is_stmt 0
	local.tee	105
	f32x4.add
.Ltmp5409:
	.loc	42 3867 14 is_stmt 1
	local.get	63
	local.get	22
	f32x4.mul
.Ltmp5410:
	.loc	42 3845 14
	f32x4.add
	local.get	59
	local.get	104
.Ltmp5411:
	.loc	42 3867 14
	local.tee	92
	f32x4.mul
.Ltmp5412:
	.loc	42 3845 14
	f32x4.add
	local.get	55
	local.get	103
.Ltmp5413:
	.loc	42 3867 14
	local.tee	91
	f32x4.mul
.Ltmp5414:
	.loc	42 3845 14
	f32x4.add
	local.get	51
	local.get	102
.Ltmp5415:
	.loc	42 3867 14
	local.tee	90
	f32x4.mul
.Ltmp5416:
	.loc	42 3845 14
	f32x4.add
	local.get	47
	local.get	101
.Ltmp5417:
	.loc	42 3867 14
	local.tee	89
	f32x4.mul
.Ltmp5418:
	.loc	42 3845 14
	f32x4.add
	local.get	43
	local.get	100
.Ltmp5419:
	.loc	42 3867 14
	local.tee	21
	f32x4.mul
.Ltmp5420:
	.loc	42 3845 14
	f32x4.add
	local.get	39
	local.get	99
.Ltmp5421:
	.loc	42 3867 14
	local.tee	88
	f32x4.mul
.Ltmp5422:
	.loc	42 3845 14
	f32x4.add
	local.get	35
	local.get	98
.Ltmp5423:
	.loc	42 3867 14
	local.tee	87
	f32x4.mul
.Ltmp5424:
	.loc	42 3845 14
	f32x4.add
	local.get	31
	local.get	97
.Ltmp5425:
	.loc	42 3867 14
	local.tee	86
	f32x4.mul
.Ltmp5426:
	.loc	42 3845 14
	f32x4.add
	local.get	27
	local.get	96
.Ltmp5427:
	.loc	42 3867 14
	local.tee	85
	f32x4.mul
.Ltmp5428:
	.loc	42 3845 14
	f32x4.add
	local.get	24
	local.get	95
.Ltmp5429:
	.loc	42 3867 14
	local.tee	94
	f32x4.mul
.Ltmp5430:
	.loc	42 3845 14
	f32x4.add
.Ltmp5431:
	.loc	42 3812 14
	f32x4.abs
.Ltmp5432:
	.loc	42 3867 14
	local.get	68
	local.get	93
	f32x4.mul
.Ltmp5433:
	.loc	42 3845 14
	local.get	105
	f32x4.add
.Ltmp5434:
	.loc	42 3867 14
	local.get	64
	local.get	22
	f32x4.mul
.Ltmp5435:
	.loc	42 3845 14
	f32x4.add
.Ltmp5436:
	.loc	42 3867 14
	local.get	60
	local.get	92
	f32x4.mul
.Ltmp5437:
	.loc	42 3845 14
	f32x4.add
.Ltmp5438:
	.loc	42 3867 14
	local.get	56
	local.get	91
	f32x4.mul
.Ltmp5439:
	.loc	42 3845 14
	f32x4.add
.Ltmp5440:
	.loc	42 3867 14
	local.get	52
	local.get	90
	f32x4.mul
.Ltmp5441:
	.loc	42 3845 14
	f32x4.add
.Ltmp5442:
	.loc	42 3867 14
	local.get	48
	local.get	89
	f32x4.mul
.Ltmp5443:
	.loc	42 3845 14
	f32x4.add
.Ltmp5444:
	.loc	42 3867 14
	local.get	44
	local.get	21
	f32x4.mul
.Ltmp5445:
	.loc	42 3845 14
	f32x4.add
.Ltmp5446:
	.loc	42 3867 14
	local.get	40
	local.get	88
	f32x4.mul
.Ltmp5447:
	.loc	42 3845 14
	f32x4.add
.Ltmp5448:
	.loc	42 3867 14
	local.get	36
	local.get	87
	f32x4.mul
.Ltmp5449:
	.loc	42 3845 14
	f32x4.add
.Ltmp5450:
	.loc	42 3867 14
	local.get	32
	local.get	86
	f32x4.mul
.Ltmp5451:
	.loc	42 3845 14
	f32x4.add
.Ltmp5452:
	.loc	42 3867 14
	local.get	28
	local.get	85
	f32x4.mul
.Ltmp5453:
	.loc	42 3845 14
	f32x4.add
.Ltmp5454:
	.loc	42 3867 14
	local.get	23
	local.get	94
	f32x4.mul
.Ltmp5455:
	.loc	42 3845 14
	f32x4.add
.Ltmp5456:
	.loc	42 3812 14
	f32x4.abs
.Ltmp5457:
	.loc	42 3867 14
	local.get	69
	local.get	93
	f32x4.mul
.Ltmp5458:
	.loc	42 3845 14
	local.get	105
	f32x4.add
.Ltmp5459:
	.loc	42 3867 14
	local.get	65
	local.get	22
	f32x4.mul
.Ltmp5460:
	.loc	42 3845 14
	f32x4.add
.Ltmp5461:
	.loc	42 3867 14
	local.get	61
	local.get	92
	f32x4.mul
.Ltmp5462:
	.loc	42 3845 14
	f32x4.add
.Ltmp5463:
	.loc	42 3867 14
	local.get	57
	local.get	91
	f32x4.mul
.Ltmp5464:
	.loc	42 3845 14
	f32x4.add
.Ltmp5465:
	.loc	42 3867 14
	local.get	53
	local.get	90
	f32x4.mul
.Ltmp5466:
	.loc	42 3845 14
	f32x4.add
.Ltmp5467:
	.loc	42 3867 14
	local.get	49
	local.get	89
	f32x4.mul
.Ltmp5468:
	.loc	42 3845 14
	f32x4.add
.Ltmp5469:
	.loc	42 3867 14
	local.get	45
	local.get	21
	f32x4.mul
.Ltmp5470:
	.loc	42 3845 14
	f32x4.add
.Ltmp5471:
	.loc	42 3867 14
	local.get	41
	local.get	88
	f32x4.mul
.Ltmp5472:
	.loc	42 3845 14
	f32x4.add
.Ltmp5473:
	.loc	42 3867 14
	local.get	37
	local.get	87
	f32x4.mul
.Ltmp5474:
	.loc	42 3845 14
	f32x4.add
.Ltmp5475:
	.loc	42 3867 14
	local.get	33
	local.get	86
	f32x4.mul
.Ltmp5476:
	.loc	42 3845 14
	f32x4.add
.Ltmp5477:
	.loc	42 3867 14
	local.get	29
	local.get	85
	f32x4.mul
.Ltmp5478:
	.loc	42 3845 14
	f32x4.add
.Ltmp5479:
	.loc	42 3867 14
	local.get	25
	local.get	94
	f32x4.mul
.Ltmp5480:
	.loc	42 3845 14
	f32x4.add
.Ltmp5481:
	.loc	42 3812 14
	f32x4.abs
.Ltmp5482:
	.loc	42 3867 14
	local.get	70
	local.get	93
	f32x4.mul
.Ltmp5483:
	.loc	42 3845 14
	local.get	105
	f32x4.add
.Ltmp5484:
	.loc	42 3867 14
	local.get	66
	local.get	22
	f32x4.mul
.Ltmp5485:
	.loc	42 3845 14
	f32x4.add
.Ltmp5486:
	.loc	42 3867 14
	local.get	62
	local.get	92
	f32x4.mul
.Ltmp5487:
	.loc	42 3845 14
	f32x4.add
.Ltmp5488:
	.loc	42 3867 14
	local.get	58
	local.get	91
	f32x4.mul
.Ltmp5489:
	.loc	42 3845 14
	f32x4.add
.Ltmp5490:
	.loc	42 3867 14
	local.get	54
	local.get	90
	f32x4.mul
.Ltmp5491:
	.loc	42 3845 14
	f32x4.add
.Ltmp5492:
	.loc	42 3867 14
	local.get	50
	local.get	89
	f32x4.mul
.Ltmp5493:
	.loc	42 3845 14
	f32x4.add
.Ltmp5494:
	.loc	42 3867 14
	local.get	46
	local.get	21
	f32x4.mul
.Ltmp5495:
	.loc	42 3845 14
	f32x4.add
.Ltmp5496:
	.loc	42 3867 14
	local.get	42
	local.get	88
	f32x4.mul
.Ltmp5497:
	.loc	42 3845 14
	f32x4.add
.Ltmp5498:
	.loc	42 3867 14
	local.get	38
	local.get	87
	f32x4.mul
.Ltmp5499:
	.loc	42 3845 14
	f32x4.add
.Ltmp5500:
	.loc	42 3867 14
	local.get	34
	local.get	86
	f32x4.mul
.Ltmp5501:
	.loc	42 3845 14
	f32x4.add
.Ltmp5502:
	.loc	42 3867 14
	local.get	30
	local.get	85
	f32x4.mul
.Ltmp5503:
	.loc	42 3845 14
	f32x4.add
.Ltmp5504:
	.loc	42 3867 14
	local.get	26
	local.get	94
	f32x4.mul
.Ltmp5505:
	.loc	42 3845 14
	f32x4.add
.Ltmp5506:
	.loc	42 3812 14
	f32x4.abs
.Ltmp5507:
	.loc	42 3812 14 is_stmt 0
	local.get	21
	f32x4.abs
.Ltmp5508:
	.loc	42 3928 9 is_stmt 1
	f32x4.pmax
	f32x4.pmax
	f32x4.pmax
	f32x4.pmax
.Ltmp5509:
	.loc	1 551 14
	v128.store	0:p2align=2
	local.get	9
	i32.const	16
.Ltmp5510:
	.loc	33 304 12
	i32.add 
	local.set	9
	local.get	8
	i32.const	16
	i32.add 
	local.set	8
	local.get	85
	local.set	95
	local.get	86
	local.set	96
	local.get	87
	local.set	97
	local.get	88
	local.set	98
	local.get	21
	local.set	99
	local.get	89
	local.set	100
	local.get	90
	local.set	101
	local.get	91
	local.set	102
	local.get	92
	local.set	103
	local.get	22
	local.set	104
	local.get	10
	i32.const	-1
	i32.add 
	local.tee	10
	br_if   	0
.LBB27_243:
	end_loop
	end_block
.Ltmp5511:
	.loc	10 900 12
	block   	
	block   	
	block   	
	local.get	7
	i32.eqz
	br_if   	0
	.loc	10 0 12 is_stmt 0
	local.get	121
	local.set	97
	local.get	125
	local.set	96
	local.get	124
	local.set	95
	.loc	10 900 12
	br      	1
.Ltmp5512:
.LBB27_245:
	.loc	10 0 12
	end_block
	local.get	0
	i32.load	920
	local.set	81
	local.get	0
	i32.load	916
	local.set	11
	i32.const	0
	local.set	109
	local.get	121
	local.set	97
	local.get	125
	local.set	96
	local.get	124
	local.set	95
.LBB27_246:
.Ltmp5513:
	.loc	14 853 61 is_stmt 1
	loop    	
	local.get	4
	local.get	4
	v128.load	960
.Ltmp5514:
	.loc	42 2188 14
	local.tee	105
	v128.const	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
.Ltmp5515:
	.loc	42 3856 14
	local.tee	98
	local.get	95
	v128.const	-0x1p0, -0x1p0, -0x1p0, -0x1p0
	local.tee	99
	f32x4.add
.Ltmp5516:
	.loc	42 3929 13
	local.tee	102
	v128.const	0x0p0, 0x0p0, 0x0p0, 0x0p0
	local.tee	100
	f32x4.gt
.Ltmp5517:
	.loc	42 2188 14
	local.tee	95
	v128.bitselect
.Ltmp5518:
	.loc	14 854 9
	v128.store	960
	.loc	14 853 44
	local.get	4
	local.get	105
	local.get	4
	v128.load	928
.Ltmp5519:
	.loc	42 3845 14
	f32x4.add
.Ltmp5520:
	.loc	42 2188 14
	local.get	72
	local.get	95
	v128.bitselect
.Ltmp5521:
	.loc	14 853 9
	local.tee	105
	v128.store	928
.Ltmp5522:
	.loc	14 853 44 is_stmt 0
	local.get	4
	local.get	4
	v128.load	992
	.loc	14 853 61
	local.get	4
	v128.load	1024
.Ltmp5523:
	.loc	42 3845 14 is_stmt 1
	local.tee	101
	f32x4.add
.Ltmp5524:
	.loc	42 3856 14
	local.get	71
	local.get	96
	local.get	99
	f32x4.add
.Ltmp5525:
	.loc	42 3929 13
	local.tee	103
	local.get	100
	f32x4.gt
.Ltmp5526:
	.loc	42 2188 14
	local.tee	96
	v128.bitselect
.Ltmp5527:
	.loc	14 853 9
	local.tee	99
	v128.store	992
.Ltmp5528:
	.loc	42 2188 14
	local.get	4
	local.get	101
	local.get	98
	local.get	96
	v128.bitselect
.Ltmp5529:
	.loc	14 854 9
	v128.store	1024
.Ltmp5530:
	.loc	14 3271 24
	block   	
	block   	
	block   	
	local.get	2
	local.get	109
	local.get	19
	i32.add 
	i32.const	2
	i32.shl 
.Ltmp5531:
	.loc	38 568 12
	local.tee	9
	i32.lt_u
	br_if   	0
	.loc	38 573 27
	block   	
	local.get	2
	local.get	9
	i32.sub 
	local.tee	8
	i32.const	3
.Ltmp5532:
	.loc	38 438 16
	i32.le_u
	br_if   	0
.Ltmp5533:
	.loc	14 1392 25
	local.get	0
	i32.load	944
.Ltmp5534:
	.loc	14 1388 17
	local.tee	8
	local.get	0
	i32.load	1020
.Ltmp5535:
	.loc	14 1392 45
	local.tee	74
	local.get	17
	i32.mul 
.Ltmp5536:
	.loc	38 580 12
	local.tee	73
	i32.lt_u
	br_if   	16
	.loc	38 585 27
	block   	
	local.get	8
	local.get	73
	i32.sub 
	local.tee	8
	i32.const	3
.Ltmp5537:
	.loc	38 451 16
	i32.le_u
	br_if   	0
.Ltmp5538:
	.loc	38 0 16 is_stmt 0
	local.get	1
	local.get	9
	i32.const	2
	i32.shl 
	i32.add 
	local.tee	82
	v128.load	0:p2align=2
	local.set	100
.Ltmp5539:
	.loc	14 1392 25 is_stmt 1
	local.get	0
	i32.load	940
	local.get	73
	i32.const	2
.Ltmp5540:
	.loc	38 101 24
	i32.shl 
	local.tee	112
	i32.add 
.Ltmp5541:
	.loc	14 0 0 is_stmt 0
	local.get	105
	local.get	4
	i32.const	1136
	i32.add 
	local.get	109
	i32.const	4
	i32.shl 
	i32.add 
	v128.load	0:p2align=2
.Ltmp5542:
	local.tee	98
	local.get	98
	local.get	119
	v128.bitselect
.Ltmp5543:
	local.tee	98
	f32x4.div
	v128.const	0, 0, 128, 63, 0, 0, 128, 63, 0, 0, 128, 63, 0, 0, 128, 63
	local.get	98
	local.get	105
	f32x4.gt
	v128.bitselect
.Ltmp5544:
	.loc	1 551 14 is_stmt 1
	v128.store	0:p2align=2
.Ltmp5545:
	.loc	14 1259 17
	block   	
	block   	
	block   	
	local.get	0
	i32.load	1020
.Ltmp5546:
	.loc	43 37 12
	local.tee	6
	i32.eqz
	br_if   	0
	.loc	43 0 12 is_stmt 0
	i32.const	0
	local.set	110
	local.get	6
	local.get	17
	i32.const	1
	i32.add 
	local.tee	9
	i32.const	0
	local.get	11
	local.get	9
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	i32.mul 
	local.set	116
	local.get	0
	i32.load	964
	local.set	77
	local.get	0
	i32.load	968
	local.set	117
	local.get	0
	i32.load	980
	local.set	115
	local.get	0
	i32.load	984
	local.set	107
	local.get	0
	i32.load	940
	local.set	13
	local.get	0
	i32.load	944
	local.set	12
	local.get	0
	i32.load	1012
	local.set	114
	local.get	0
	i32.load	1016
	local.set	108
	i32.const	0
	local.set	7
	local.get	6
	local.set	111
.LBB27_252:
	loop    	
	local.get	110
	i32.const	32
.Ltmp5547:
	.loc	16 1714 9 is_stmt 1
	i32.eq  
.Ltmp5548:
	.loc	15 180 28
	br_if   	1
.Ltmp5549:
	.loc	14 1263 21
	block   	
	block   	
	block   	
	block   	
	local.get	7
	local.get	108
	i32.eq  
	br_if   	0
	.loc	14 0 21 is_stmt 0
	local.get	114
	local.get	7
	i32.const	12
	.loc	14 1263 21
	i32.mul 
	i32.add 
	local.tee	8
	i32.load	4
.Ltmp5550:
	.loc	14 1265 23 is_stmt 1
	local.get	17
	i32.add 
	local.tee	9
	i32.const	0
.Ltmp5551:
	.loc	14 1266 12
	local.get	11
	local.get	9
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
.Ltmp5552:
	.loc	14 1273 42
	local.tee	9
	local.get	6
	i32.mul 
	local.get	7
	i32.add 
	.loc	14 1273 22 is_stmt 0
	local.tee	10
	local.get	12
	i32.ge_u
	br_if   	1
.Ltmp5553:
	.loc	14 1274 24 is_stmt 1
	local.get	7
	local.get	107
	i32.eq  
	br_if   	2
.Ltmp5554:
	.loc	14 0 0 is_stmt 0
	local.get	8
	i32.load	0
	local.set	8
	local.get	13
	local.get	10
	i32.const	2
.Ltmp5555:
	i32.shl 
	i32.add 
	local.tee	18
	f32.load	0
	local.set	122
	local.get	115
	local.get	7
	i32.const	2
.Ltmp5556:
	.loc	14 1274 24
	i32.shl 
	local.tee	10
	i32.add 
	local.tee	113
	i32.load	0
.Ltmp5557:
	.loc	14 1275 26 is_stmt 1
	local.tee	75
	i32.eqz
	br_if   	3
	.loc	14 1278 24
	block   	
	block   	
	local.get	7
	local.get	117
	i32.ge_u
	br_if   	0
	local.get	77
	local.get	10
	i32.add 
	f32.load	0
.Ltmp5558:
	.loc	14 903 8
	local.tee	123
	local.get	122
	f32.lt  
	br_if   	1
	br      	5
.Ltmp5559:
.LBB27_259:
	.loc	14 0 8 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	1072
	local.get	4
	local.get	125
	v128.store	1040
	local.get	4
	local.get	124
	v128.store	976
.Ltmp5560:
	local.get	4
	local.get	94
	v128.store	912
	local.get	4
	local.get	85
	v128.store	896
	local.get	4
	local.get	86
	v128.store	880
	local.get	4
	local.get	87
	v128.store	864
	local.get	4
	local.get	88
	v128.store	848
	local.get	4
	local.get	21
	v128.store	832
	local.get	4
	local.get	89
	v128.store	816
	local.get	4
	local.get	90
	v128.store	800
	local.get	4
	local.get	91
	v128.store	784
	local.get	4
	local.get	92
	v128.store	768
	local.get	4
	local.get	22
	v128.store	752
	local.get	4
	local.get	93
	v128.store	736
.Ltmp5561:
	.loc	14 1278 24 is_stmt 1
	local.get	7
	local.get	117
	i32.const	.Lalloc_a228cac3279aae3a34886f59f51fbe9e
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.LBB27_260:
	.loc	14 0 24 is_stmt 0
	end_block
	local.get	123
	local.set	122
.Ltmp5562:
	.loc	14 903 5 is_stmt 1
	br      	3
.Ltmp5563:
.LBB27_261:
	.loc	14 0 5 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	1072
	local.get	4
	local.get	125
	v128.store	1040
	local.get	4
	local.get	124
	v128.store	976
.Ltmp5564:
	local.get	4
	local.get	94
	v128.store	912
	local.get	4
	local.get	85
	v128.store	896
	local.get	4
	local.get	86
	v128.store	880
	local.get	4
	local.get	87
	v128.store	864
	local.get	4
	local.get	88
	v128.store	848
	local.get	4
	local.get	21
	v128.store	832
	local.get	4
	local.get	89
	v128.store	816
	local.get	4
	local.get	90
	v128.store	800
	local.get	4
	local.get	91
	v128.store	784
	local.get	4
	local.get	92
	v128.store	768
	local.get	4
	local.get	22
	v128.store	752
	local.get	4
	local.get	93
	v128.store	736
.Ltmp5565:
	.loc	14 1263 21 is_stmt 1
	local.get	108
	local.get	108
	i32.const	.Lalloc_12856b5f9033764dbf23e04eb77c5c46
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.Ltmp5566:
.LBB27_262:
	.loc	14 0 21 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	1072
	local.get	4
	local.get	125
	v128.store	1040
	local.get	4
	local.get	124
	v128.store	976
.Ltmp5567:
	local.get	4
	local.get	94
	v128.store	912
	local.get	4
	local.get	85
	v128.store	896
	local.get	4
	local.get	86
	v128.store	880
	local.get	4
	local.get	87
	v128.store	864
	local.get	4
	local.get	88
	v128.store	848
	local.get	4
	local.get	21
	v128.store	832
	local.get	4
	local.get	89
	v128.store	816
	local.get	4
	local.get	90
	v128.store	800
	local.get	4
	local.get	91
	v128.store	784
	local.get	4
	local.get	92
	v128.store	768
	local.get	4
	local.get	22
	v128.store	752
	local.get	4
	local.get	93
	v128.store	736
.Ltmp5568:
	.loc	14 1273 22 is_stmt 1
	local.get	10
	local.get	12
	i32.const	.Lalloc_70ebf0cf1c90c6161926611ccf249a32
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.Ltmp5569:
.LBB27_263:
	.loc	14 0 22 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	1072
	local.get	4
	local.get	125
	v128.store	1040
	local.get	4
	local.get	124
	v128.store	976
.Ltmp5570:
	local.get	4
	local.get	94
	v128.store	912
	local.get	4
	local.get	85
	v128.store	896
	local.get	4
	local.get	86
	v128.store	880
	local.get	4
	local.get	87
	v128.store	864
	local.get	4
	local.get	88
	v128.store	848
	local.get	4
	local.get	21
	v128.store	832
	local.get	4
	local.get	89
	v128.store	816
	local.get	4
	local.get	90
	v128.store	800
	local.get	4
	local.get	91
	v128.store	784
	local.get	4
	local.get	92
	v128.store	768
	local.get	4
	local.get	22
	v128.store	752
	local.get	4
	local.get	93
	v128.store	736
.Ltmp5571:
	.loc	14 1274 24 is_stmt 1
	local.get	107
	local.get	107
	i32.const	.Lalloc_023d591aaad7f5cf482ac80a9401eca1
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.LBB27_264:
	.loc	14 0 24 is_stmt 0
	end_block
.Ltmp5572:
	.loc	14 1280 9 is_stmt 1
	block   	
	block   	
	block   	
	local.get	7
	local.get	117
	i32.eq  
	br_if   	0
.Ltmp5573:
	.loc	14 0 0 is_stmt 0
	local.get	4
	i32.const	1104
	i32.add 
	local.get	110
	i32.add 
	local.set	76
.Ltmp5574:
	.loc	14 1280 9
	local.get	77
	local.get	10
	i32.add 
	local.get	122
	f32.store	0
	local.get	75
	i32.const	1
	.loc	14 1281 24 is_stmt 1
	i32.add 
	local.tee	75
	local.get	8
	i32.ne  
.Ltmp5575:
	.loc	14 1282 23
	br_if   	1
	.loc	14 1282 9 is_stmt 0
	local.get	76
	local.get	122
	f32.store	0
	i32.const	0
	local.set	75
	local.get	8
	i32.eqz
	br_if   	2
	.loc	14 1288 30 is_stmt 1
	local.get	18
	f32.load	0
	local.set	122
.LBB27_268:
.Ltmp5576:
	.loc	14 1291 65
	loop    	
	local.get	9
	local.get	6
	i32.mul 
	local.get	7
	i32.add 
	.loc	14 1291 45 is_stmt 0
	local.tee	10
	local.get	12
	i32.ge_u
	br_if   	7
	.loc	14 0 45
	local.get	13
	local.get	10
	i32.const	2
	.loc	14 1291 45
	i32.shl 
	i32.add 
	local.tee	10
	local.get	122
	local.get	10
	f32.load	0
.Ltmp5577:
	.loc	14 903 8 is_stmt 1
	local.tee	123
	local.get	122
	local.get	123
	f32.lt  
	f32.select
.Ltmp5578:
	.loc	14 1292 17
	local.tee	122
	f32.store	0
	.loc	14 1293 20
	local.get	9
	local.get	11
	local.get	9
	i32.select
	i32.const	-1
	.loc	14 1296 17
	i32.add 
	local.set	9
	local.get	8
	i32.const	-1
.Ltmp5579:
	.loc	9 1916 50
	i32.add 
.Ltmp5580:
	.loc	10 900 12
	local.tee	8
	br_if   	0
	br      	3
.Ltmp5581:
.LBB27_270:
	.loc	14 1280 9
	end_loop
	end_block
	local.get	4
	local.get	121
	v128.store	1072
	local.get	4
	local.get	125
	v128.store	1040
	local.get	4
	local.get	124
	v128.store	976
.Ltmp5582:
	.loc	14 0 0 is_stmt 0
	local.get	4
	local.get	94
	v128.store	912
	local.get	4
	local.get	85
	v128.store	896
	local.get	4
	local.get	86
	v128.store	880
	local.get	4
	local.get	87
	v128.store	864
	local.get	4
	local.get	88
	v128.store	848
	local.get	4
	local.get	21
	v128.store	832
	local.get	4
	local.get	89
	v128.store	816
	local.get	4
	local.get	90
	v128.store	800
	local.get	4
	local.get	91
	v128.store	784
	local.get	4
	local.get	92
	v128.store	768
	local.get	4
	local.get	22
	v128.store	752
	local.get	4
	local.get	93
	v128.store	736
.Ltmp5583:
	.loc	14 1280 9
	local.get	117
	local.get	117
	i32.const	.Lalloc_1fa5d88c1a2cc292a2767c48606a38fe
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.LBB27_271:
	.loc	14 0 9
	end_block
.Ltmp5584:
	.loc	14 1285 44 is_stmt 1
	local.get	7
	local.get	116
	i32.add 
	.loc	14 1285 24 is_stmt 0
	local.tee	9
	local.get	12
	i32.ge_u
	br_if   	3
	.loc	14 0 24
	local.get	76
	local.get	13
	local.get	9
	i32.const	2
	.loc	14 1285 24
	i32.shl 
	i32.add 
	f32.load	0
.Ltmp5585:
	.loc	14 903 8 is_stmt 1
	local.tee	123
	local.get	122
	local.get	123
	local.get	122
	f32.lt  
	f32.select
.Ltmp5586:
	.loc	14 1282 9
	f32.store	0
.Ltmp5587:
.LBB27_273:
	.loc	14 0 9 is_stmt 0
	end_block
	local.get	7
	i32.const	1
	i32.add 
	local.set	7
	local.get	110
	i32.const	4
	i32.add 
	local.set	110
.Ltmp5588:
	local.get	113
	local.get	75
	i32.store	0
	local.get	111
	i32.const	-1
.Ltmp5589:
	i32.add 
.Ltmp5590:
	.loc	43 37 12 is_stmt 1
	local.tee	111
	br_if   	0
.LBB27_274:
	.loc	43 37 12
	end_loop
	end_block
.Ltmp5591:
	.loc	14 1412 26
	local.get	0
	i32.load	952
	local.set	9
.Ltmp5592:
	.loc	1 551 14
	local.get	4
	v128.load	1104:p2align=3
	local.tee	98
	local.set	105
	local.get	74
	i32.eqz
	br_if   	6
.Ltmp5593:
	.loc	1 0 14 is_stmt 0
	block   	
	local.get	0
	i32.load	1016
	local.tee	10
	br_if   	0
	i32.const	0
	local.set	8
	br      	21
.LBB27_277:
	end_block
	block   	
	local.get	0
	i32.load	1012
.Ltmp5594:
	.loc	14 1403 42 is_stmt 1
	local.tee	7
	i32.load	8
	.loc	14 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5595:
	.loc	14 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	14 1407 40
	local.get	74
	i32.mul 
	.loc	14 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	14 0 25
	local.get	4
	local.get	0
	i32.load	948
	local.tee	12
	local.get	8
	i32.const	2
	.loc	14 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	14 1407 13
	f32.store	1104
	local.get	74
	i32.const	1
.Ltmp5596:
	.loc	43 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5597:
	.loc	43 0 12 is_stmt 0
	i32.const	1
	local.set	8
	local.get	10
	i32.const	1
.Ltmp5598:
	.loc	14 1403 42 is_stmt 1
	i32.eq  
	br_if   	21
	local.get	7
	i32.load	20
	.loc	14 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5599:
	.loc	14 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	14 1407 40
	local.get	74
	i32.mul 
	i32.const	1
	i32.add 
	.loc	14 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	14 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	14 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	14 1407 13
	f32.store	1108
	local.get	74
	i32.const	2
.Ltmp5600:
	.loc	43 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5601:
	.loc	43 0 12 is_stmt 0
	i32.const	2
	local.set	8
	local.get	10
	i32.const	2
.Ltmp5602:
	.loc	14 1403 42 is_stmt 1
	i32.eq  
	br_if   	21
	local.get	7
	i32.load	32
	.loc	14 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5603:
	.loc	14 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	14 1407 40
	local.get	74
	i32.mul 
	i32.const	2
	i32.add 
	.loc	14 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	14 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	14 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	14 1407 13
	f32.store	1112
	local.get	74
	i32.const	3
.Ltmp5604:
	.loc	43 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5605:
	.loc	43 0 12 is_stmt 0
	i32.const	3
	local.set	8
	local.get	10
	i32.const	3
.Ltmp5606:
	.loc	14 1403 42 is_stmt 1
	i32.eq  
	br_if   	21
	local.get	7
	i32.load	44
	.loc	14 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5607:
	.loc	14 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	14 1407 40
	local.get	74
	i32.mul 
	i32.const	3
	i32.add 
	.loc	14 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	14 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	14 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	14 1407 13
	f32.store	1116
	local.get	74
	i32.const	4
.Ltmp5608:
	.loc	43 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5609:
	.loc	43 0 12 is_stmt 0
	i32.const	4
	local.set	8
	local.get	10
	i32.const	4
.Ltmp5610:
	.loc	14 1403 42 is_stmt 1
	i32.eq  
	br_if   	21
	local.get	7
	i32.load	56
	.loc	14 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5611:
	.loc	14 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	14 1407 40
	local.get	74
	i32.mul 
	i32.const	4
	i32.add 
	.loc	14 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	14 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	14 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	14 1407 13
	f32.store	1120
	local.get	74
	i32.const	5
.Ltmp5612:
	.loc	43 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5613:
	.loc	43 0 12 is_stmt 0
	i32.const	5
	local.set	8
	local.get	10
	i32.const	5
.Ltmp5614:
	.loc	14 1403 42 is_stmt 1
	i32.eq  
	br_if   	21
	local.get	7
	i32.load	68
	.loc	14 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5615:
	.loc	14 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	14 1407 40
	local.get	74
	i32.mul 
	i32.const	5
	i32.add 
	.loc	14 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	14 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	14 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	14 1407 13
	f32.store	1124
	local.get	74
	i32.const	6
.Ltmp5616:
	.loc	43 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5617:
	.loc	43 0 12 is_stmt 0
	i32.const	6
	local.set	8
	local.get	10
	i32.const	6
.Ltmp5618:
	.loc	14 1403 42 is_stmt 1
	i32.eq  
	br_if   	21
	local.get	7
	i32.load	80
	.loc	14 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5619:
	.loc	14 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	14 1407 40
	local.get	74
	i32.mul 
	i32.const	6
	i32.add 
	.loc	14 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	14 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	14 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	14 1407 13
	f32.store	1128
	local.get	74
	i32.const	7
.Ltmp5620:
	.loc	43 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5621:
	.loc	43 0 12 is_stmt 0
	i32.const	7
	local.set	8
	local.get	10
	i32.const	7
.Ltmp5622:
	.loc	14 1403 42 is_stmt 1
	i32.eq  
	br_if   	21
	local.get	7
	i32.load	92
	.loc	14 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5623:
	.loc	14 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	14 1407 40
	local.get	74
	i32.mul 
	i32.const	7
	i32.add 
	.loc	14 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	14 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	14 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	14 1407 13
	f32.store	1132
.Ltmp5624:
	.loc	43 37 12 is_stmt 1
	br      	6
.Ltmp5625:
.LBB27_300:
	.loc	43 0 12 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	1072
	local.get	4
	local.get	125
	v128.store	1040
	local.get	4
	local.get	124
	v128.store	976
.Ltmp5626:
	local.get	4
	local.get	94
	v128.store	912
	local.get	4
	local.get	85
	v128.store	896
	local.get	4
	local.get	86
	v128.store	880
	local.get	4
	local.get	87
	v128.store	864
	local.get	4
	local.get	88
	v128.store	848
	local.get	4
	local.get	21
	v128.store	832
	local.get	4
	local.get	89
	v128.store	816
	local.get	4
	local.get	90
	v128.store	800
	local.get	4
	local.get	91
	v128.store	784
	local.get	4
	local.get	92
	v128.store	768
	local.get	4
	local.get	22
	v128.store	752
	local.get	4
	local.get	93
	v128.store	736
.Ltmp5627:
	.loc	14 1407 25 is_stmt 1
	local.get	8
	local.get	9
	i32.const	.Lalloc_0e76a26fbb751dfb8cf8a069b5b0d39a
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.Ltmp5628:
.LBB27_301:
	.loc	14 0 25 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	1072
	local.get	4
	local.get	125
	v128.store	1040
	local.get	4
	local.get	124
	v128.store	976
.Ltmp5629:
	local.get	4
	local.get	94
	v128.store	912
	local.get	4
	local.get	85
	v128.store	896
	local.get	4
	local.get	86
	v128.store	880
	local.get	4
	local.get	87
	v128.store	864
	local.get	4
	local.get	88
	v128.store	848
	local.get	4
	local.get	21
	v128.store	832
	local.get	4
	local.get	89
	v128.store	816
	local.get	4
	local.get	90
	v128.store	800
	local.get	4
	local.get	91
	v128.store	784
	local.get	4
	local.get	92
	v128.store	768
	local.get	4
	local.get	22
	v128.store	752
	local.get	4
	local.get	93
	v128.store	736
.Ltmp5630:
	.loc	14 1285 24 is_stmt 1
	local.get	9
	local.get	12
	i32.const	.Lalloc_70d40ae2302f3ea19fa0c4a65fe82786
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.Ltmp5631:
.LBB27_302:
	.loc	14 0 24 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	1072
	local.get	4
	local.get	125
	v128.store	1040
	local.get	4
	local.get	124
	v128.store	976
.Ltmp5632:
	local.get	4
	local.get	94
	v128.store	912
	local.get	4
	local.get	85
	v128.store	896
	local.get	4
	local.get	86
	v128.store	880
	local.get	4
	local.get	87
	v128.store	864
	local.get	4
	local.get	88
	v128.store	848
	local.get	4
	local.get	21
	v128.store	832
	local.get	4
	local.get	89
	v128.store	816
	local.get	4
	local.get	90
	v128.store	800
	local.get	4
	local.get	91
	v128.store	784
	local.get	4
	local.get	92
	v128.store	768
	local.get	4
	local.get	22
	v128.store	752
	local.get	4
	local.get	93
	v128.store	736
.Ltmp5633:
	.loc	14 1291 45 is_stmt 1
	local.get	10
	local.get	12
	i32.const	.Lalloc_a271bbb7fd83c3605ea58a06b7065fa4
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.Ltmp5634:
.LBB27_303:
	.loc	14 0 45 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	1072
	local.get	4
	local.get	125
	v128.store	1040
	local.get	4
	local.get	124
	v128.store	976
.Ltmp5635:
	local.get	4
	local.get	94
	v128.store	912
	local.get	4
	local.get	85
	v128.store	896
	local.get	4
	local.get	86
	v128.store	880
	local.get	4
	local.get	87
	v128.store	864
	local.get	4
	local.get	88
	v128.store	848
	local.get	4
	local.get	21
	v128.store	832
	local.get	4
	local.get	89
	v128.store	816
	local.get	4
	local.get	90
	v128.store	800
	local.get	4
	local.get	91
	v128.store	784
	local.get	4
	local.get	92
	v128.store	768
	local.get	4
	local.get	22
	v128.store	752
	local.get	4
	local.get	93
	v128.store	736
	i32.const	0
	i32.const	4
.Ltmp5636:
	.loc	38 456 13 is_stmt 1
	local.get	8
	i32.const	.Lalloc_5f4e05826d69b5e832dc96c63f325058
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5637:
.LBB27_304:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	1072
	local.get	4
	local.get	125
	v128.store	1040
	local.get	4
	local.get	124
	v128.store	976
.Ltmp5638:
	local.get	4
	local.get	94
	v128.store	912
	local.get	4
	local.get	85
	v128.store	896
	local.get	4
	local.get	86
	v128.store	880
	local.get	4
	local.get	87
	v128.store	864
	local.get	4
	local.get	88
	v128.store	848
	local.get	4
	local.get	21
	v128.store	832
	local.get	4
	local.get	89
	v128.store	816
	local.get	4
	local.get	90
	v128.store	800
	local.get	4
	local.get	91
	v128.store	784
	local.get	4
	local.get	92
	v128.store	768
	local.get	4
	local.get	22
	v128.store	752
	local.get	4
	local.get	93
	v128.store	736
	i32.const	0
	i32.const	4
.Ltmp5639:
	.loc	38 443 13 is_stmt 1
	local.get	8
	i32.const	.Lalloc_5f4e05826d69b5e832dc96c63f325058
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5640:
.LBB27_305:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	1072
	local.get	4
	local.get	125
	v128.store	1040
	local.get	4
	local.get	124
	v128.store	976
.Ltmp5641:
	local.get	4
	local.get	94
	v128.store	912
	local.get	4
	local.get	85
	v128.store	896
	local.get	4
	local.get	86
	v128.store	880
	local.get	4
	local.get	87
	v128.store	864
	local.get	4
	local.get	88
	v128.store	848
	local.get	4
	local.get	21
	v128.store	832
	local.get	4
	local.get	89
	v128.store	816
	local.get	4
	local.get	90
	v128.store	800
	local.get	4
	local.get	91
	v128.store	784
	local.get	4
	local.get	92
	v128.store	768
	local.get	4
	local.get	22
	v128.store	752
	local.get	4
	local.get	93
	v128.store	736
.Ltmp5642:
	.loc	38 569 13 is_stmt 1
	local.get	9
	local.get	2
	local.get	2
	i32.const	.Lalloc_16fe79fe907693415948d189acefd4a9
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5643:
.LBB27_306:
	.loc	38 0 13 is_stmt 0
	end_block
.Ltmp5644:
	.loc	1 551 14 is_stmt 1
	local.get	4
	v128.load	1104:p2align=3
	local.set	105
.Ltmp5645:
.LBB27_307:
	.loc	1 0 14 is_stmt 0
	end_block
.Ltmp5646:
	.loc	38 580 12 is_stmt 1
	block   	
	local.get	9
	local.get	73
	i32.ge_u
	br_if   	0
.Ltmp5647:
	.loc	38 0 12 is_stmt 0
	local.get	4
	local.get	121
	v128.store	1072
	local.get	4
	local.get	125
	v128.store	1040
	local.get	4
	local.get	124
	v128.store	976
.Ltmp5648:
	local.get	4
	local.get	94
	v128.store	912
	local.get	4
	local.get	85
	v128.store	896
	local.get	4
	local.get	86
	v128.store	880
	local.get	4
	local.get	87
	v128.store	864
	local.get	4
	local.get	88
	v128.store	848
	local.get	4
	local.get	21
	v128.store	832
	local.get	4
	local.get	89
	v128.store	816
	local.get	4
	local.get	90
	v128.store	800
	local.get	4
	local.get	91
	v128.store	784
	local.get	4
	local.get	92
	v128.store	768
	local.get	4
	local.get	22
	v128.store	752
	local.get	4
	local.get	93
	v128.store	736
.Ltmp5649:
	.loc	38 581 13 is_stmt 1
	local.get	73
	local.get	9
	local.get	9
	i32.const	.Lalloc_c2286ff41a33b8c11aa270fe215441de
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.LBB27_309:
	.loc	38 0 13 is_stmt 0
	end_block
	.loc	38 585 27 is_stmt 1
	local.get	9
	local.get	73
	i32.sub 
	local.tee	9
	i32.const	3
.Ltmp5650:
	.loc	38 451 16
	i32.le_u
	br_if   	8
.Ltmp5651:
	.loc	14 1412 26
	local.get	0
	i32.load	948
.Ltmp5652:
	.loc	38 101 24
	local.get	112
	i32.add 
.Ltmp5653:
	.loc	14 0 0 is_stmt 0
	local.get	98
	v128.const	0x1p14, 0x1p14, 0x1p14, 0x1p14
	f32x4.mul
	f32x4.floor
	v128.const	0x1p-14, 0x1p-14, 0x1p-14, 0x1p-14
	f32x4.mul
.Ltmp5654:
	.loc	1 551 14 is_stmt 1
	local.tee	98
	v128.store	0:p2align=2
.Ltmp5655:
	.loc	14 1416 43
	local.get	4
	local.get	4
	v128.load	1056
.Ltmp5656:
	.loc	42 3856 14
	local.tee	101
	v128.const	0x1p0, 0x1p0, 0x1p0, 0x1p0
.Ltmp5657:
	.loc	14 0 0 is_stmt 0
	local.tee	104
	local.get	98
	local.get	97
	f32x4.add
	local.get	105
	f32x4.sub
.Ltmp5658:
	.loc	42 3878 14 is_stmt 1
	local.tee	97
	local.get	120
	f32x4.div
.Ltmp5659:
	.loc	42 3856 14
	f32x4.sub
.Ltmp5660:
	.loc	42 3856 14 is_stmt 0
	local.tee	105
	local.get	101
	f32x4.sub
.Ltmp5661:
	.loc	42 3867 14 is_stmt 1
	local.get	99
	f32x4.mul
.Ltmp5662:
	.loc	42 3845 14
	f32x4.add
.Ltmp5663:
	.loc	42 3928 9
	local.get	105
	f32x4.pmax
.Ltmp5664:
	.loc	42 3812 14
	local.tee	105
	local.get	105
	f32x4.abs
.Ltmp5665:
	.loc	42 1991 14
	v128.const	0x1.79ca1p-67, 0x1.79ca1p-67, 0x1.79ca1p-67, 0x1.79ca1p-67
	f32x4.lt
.Ltmp5666:
	.loc	44 481 22
	v128.andnot
.Ltmp5667:
	.loc	14 1417 5
	local.tee	98
	v128.store	1056
.Ltmp5668:
	.loc	14 1420 28
	local.get	0
	i32.load	936
	.loc	14 1420 44 is_stmt 0
	local.tee	8
	local.get	74
	local.get	106
	i32.mul 
.Ltmp5669:
	.loc	38 568 12 is_stmt 1
	local.tee	9
	i32.lt_u
	br_if   	4
	.loc	38 573 27
	local.get	8
	local.get	9
	i32.sub 
	local.tee	8
	i32.const	3
.Ltmp5670:
	.loc	38 438 16
	i32.le_u
	br_if   	2
.Ltmp5671:
	.loc	14 0 0 is_stmt 0
	local.get	96
	local.get	103
	v128.and
	local.set	96
	local.get	95
	local.get	102
	v128.and
	local.set	95
.Ltmp5672:
	.loc	14 1420 28 is_stmt 1
	local.get	0
	i32.load	932
	local.get	9
	i32.const	2
.Ltmp5673:
	.loc	38 89 24
	i32.shl 
	i32.add 
.Ltmp5674:
	.loc	1 551 14
	local.tee	9
	v128.load	0:p2align=2
	local.set	105
.Ltmp5675:
	.loc	1 551 14 is_stmt 0
	local.get	9
	local.get	100
	v128.store	0:p2align=2
.Ltmp5676:
	.loc	14 0 0
	local.get	82
	local.get	105
	local.get	105
	local.get	104
	local.get	98
	f32x4.sub
.Ltmp5677:
	.loc	42 3867 14 is_stmt 1
	f32x4.mul
.Ltmp5678:
	.loc	42 2188 14
	local.get	118
	v128.bitselect
.Ltmp5679:
	.loc	1 551 14
	v128.store	0:p2align=2
	i32.const	0
	local.get	17
	i32.const	1
.Ltmp5680:
	.loc	14 3299 13
	i32.add 
	.loc	14 3300 16
	local.tee	9
	local.get	9
	local.get	11
	i32.eq  
	i32.select
	local.set	17
	i32.const	0
	local.get	106
	i32.const	1
	.loc	14 3295 13
	i32.add 
	.loc	14 3296 16
	local.tee	9
	local.get	9
	local.get	81
	i32.eq  
	i32.select
	local.set	106
	local.get	109
	i32.const	1
.Ltmp5681:
	.loc	14 0 0 is_stmt 0
	i32.add 
.Ltmp5682:
	.loc	9 1916 50 is_stmt 1
	local.tee	109
	local.get	83
	i32.ne  
.Ltmp5683:
	.loc	10 900 12
	br_if   	0
.LBB27_313:
	end_loop
	end_block
	local.get	19
	i32.const	32
.Ltmp5684:
	.loc	14 0 0 is_stmt 0
	i32.add 
	local.set	19
	local.get	78
	i32.const	512
.Ltmp5685:
	.loc	41 446 20 is_stmt 1
	i32.add 
	local.set	78
	local.get	84
	i32.const	-32
	i32.add 
	local.set	84
	local.get	79
	i32.const	-1
.Ltmp5686:
	.loc	14 0 0 is_stmt 0
	i32.add 
.Ltmp5687:
	.loc	41 446 20
	local.tee	79
	i32.eqz
	br_if   	3
	br      	1
.Ltmp5688:
.LBB27_314:
	.loc	41 0 20
	end_block
.Ltmp5689:
	.loc	38 438 16 is_stmt 1
	end_loop
	local.get	4
	local.get	121
	v128.store	1072
	local.get	4
	local.get	125
	v128.store	1040
	local.get	4
	local.get	124
	v128.store	976
.Ltmp5690:
	.loc	14 0 0 is_stmt 0
	local.get	4
	local.get	94
	v128.store	912
	local.get	4
	local.get	85
	v128.store	896
	local.get	4
	local.get	86
	v128.store	880
	local.get	4
	local.get	87
	v128.store	864
	local.get	4
	local.get	88
	v128.store	848
	local.get	4
	local.get	21
	v128.store	832
	local.get	4
	local.get	89
	v128.store	816
	local.get	4
	local.get	90
	v128.store	800
	local.get	4
	local.get	91
	v128.store	784
	local.get	4
	local.get	92
	v128.store	768
	local.get	4
	local.get	22
	v128.store	752
	local.get	4
	local.get	93
	v128.store	736
	i32.const	0
	i32.const	4
.Ltmp5691:
	.loc	38 443 13 is_stmt 1
	local.get	8
	i32.const	.Lalloc_5f4e05826d69b5e832dc96c63f325058
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5692:
.LBB27_315:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	1072
	local.get	4
	local.get	125
	v128.store	1040
	local.get	4
	local.get	124
	v128.store	976
.Ltmp5693:
	local.get	4
	local.get	94
	v128.store	912
	local.get	4
	local.get	85
	v128.store	896
	local.get	4
	local.get	86
	v128.store	880
	local.get	4
	local.get	87
	v128.store	864
	local.get	4
	local.get	88
	v128.store	848
	local.get	4
	local.get	21
	v128.store	832
	local.get	4
	local.get	89
	v128.store	816
	local.get	4
	local.get	90
	v128.store	800
	local.get	4
	local.get	91
	v128.store	784
	local.get	4
	local.get	92
	v128.store	768
	local.get	4
	local.get	22
	v128.store	752
	local.get	4
	local.get	93
	v128.store	736
.Ltmp5694:
	.loc	38 569 13 is_stmt 1
	local.get	9
	local.get	8
	local.get	8
	i32.const	.Lalloc_2a3a21efd18b403ec25db545d522c479
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5695:
.LBB27_316:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	97
	v128.store	1072
	local.get	4
	local.get	96
	v128.store	1040
	local.get	4
	local.get	95
	v128.store	976
.Ltmp5696:
	local.get	4
	local.get	94
	v128.store	912
	local.get	4
	local.get	85
	v128.store	896
	local.get	4
	local.get	86
	v128.store	880
	local.get	4
	local.get	87
	v128.store	864
	local.get	4
	local.get	88
	v128.store	848
	local.get	4
	local.get	21
	v128.store	832
	local.get	4
	local.get	89
	v128.store	816
	local.get	4
	local.get	90
	v128.store	800
	local.get	4
	local.get	91
	v128.store	784
	local.get	4
	local.get	92
	v128.store	768
	local.get	4
	local.get	22
	v128.store	752
	local.get	4
	local.get	93
	v128.store	736
.Ltmp5697:
.LBB27_317:
	end_block
	.loc	14 3306 5 is_stmt 1
	local.get	4
	i32.const	2160
	i32.add 
	local.get	4
	i32.const	736
	i32.add 
	i32.const	368
	memory.copy	0, 0
	.loc	14 3306 14 is_stmt 0
	local.get	4
	i32.const	2160
	i32.add 
	local.get	15
	call	_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_
	.loc	14 3308 5 is_stmt 1
	local.get	0
	local.get	17
	i32.store	804
	.loc	14 3307 5
	local.get	0
	local.get	106
	i32.store	800
.Ltmp5698:
.LBB27_318:
	.loc	14 0 5 is_stmt 0
	end_block
	i32.const	0
	local.set	9
	.loc	14 2327 13 is_stmt 1
	local.get	14
	i32.eqz
	br_if   	2
	.loc	14 2327 32 is_stmt 0
	local.get	15
	call	_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest
	i32.eqz
	br_if   	2
	.loc	14 0 32
	block   	
	local.get	5
	local.get	2
	i32.gt_u
	br_if   	0
	local.get	1
	local.set	9
.LBB27_322:
	loop    	
	block   	
	local.get	5
	br_if   	0
	i32.const	1
	local.set	9
	br      	5
.LBB27_324:
	end_block
	local.get	9
	local.get	5
	i32.const	32
	local.get	5
	i32.const	32
.Ltmp5699:
	.loc	9 1077 12 is_stmt 1
	i32.lt_u
	i32.select
	local.tee	7
	i32.const	2
.Ltmp5700:
	.loc	35 863 18
	i32.shl 
	local.tee	10
	i32.add 
	local.set	11
	i32.const	0
	local.set	8
.Ltmp5701:
.LBB27_325:
	.loc	36 134 21
	loop    	
	local.get	9
	i32.load	0
	.loc	36 134 13 is_stmt 0
	local.get	8
	i32.or  
	local.set	8
	local.get	9
	i32.const	4
.Ltmp5702:
	.loc	16 656 28 is_stmt 1
	i32.add 
	local.set	9
	local.get	10
	i32.const	-4
.Ltmp5703:
	.loc	16 1714 9
	i32.add 
.Ltmp5704:
	.loc	15 180 28
	local.tee	10
	br_if   	0
	end_loop
.Ltmp5705:
	.loc	31 2054 74
	local.get	5
	local.get	7
	i32.sub 
	local.set	5
	local.get	11
	local.set	9
.Ltmp5706:
	.loc	36 136 12
	local.get	8
	i32.eqz
	br_if   	0
	end_loop
	i32.const	0
	local.set	9
	br      	3
.Ltmp5707:
.LBB27_328:
	.loc	36 0 12 is_stmt 0
	end_block
	i32.const	0
.Ltmp5708:
	.loc	38 443 13 is_stmt 1
	local.get	5
	local.get	2
	i32.const	.Lalloc_83306e04d21648aa3da9cd74a1a9d08b
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5709:
.LBB27_329:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	1072
	local.get	4
	local.get	125
	v128.store	1040
	local.get	4
	local.get	124
	v128.store	976
.Ltmp5710:
	local.get	4
	local.get	94
	v128.store	912
	local.get	4
	local.get	85
	v128.store	896
	local.get	4
	local.get	86
	v128.store	880
	local.get	4
	local.get	87
	v128.store	864
	local.get	4
	local.get	88
	v128.store	848
	local.get	4
	local.get	21
	v128.store	832
	local.get	4
	local.get	89
	v128.store	816
	local.get	4
	local.get	90
	v128.store	800
	local.get	4
	local.get	91
	v128.store	784
	local.get	4
	local.get	92
	v128.store	768
	local.get	4
	local.get	22
	v128.store	752
	local.get	4
	local.get	93
	v128.store	736
	i32.const	0
	i32.const	4
.Ltmp5711:
	.loc	38 456 13 is_stmt 1
	local.get	9
	i32.const	.Lalloc_5f4e05826d69b5e832dc96c63f325058
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5712:
.LBB27_330:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	1072
	local.get	4
	local.get	125
	v128.store	1040
	local.get	4
	local.get	124
	v128.store	976
.Ltmp5713:
	local.get	4
	local.get	94
	v128.store	912
	local.get	4
	local.get	85
	v128.store	896
	local.get	4
	local.get	86
	v128.store	880
	local.get	4
	local.get	87
	v128.store	864
	local.get	4
	local.get	88
	v128.store	848
	local.get	4
	local.get	21
	v128.store	832
	local.get	4
	local.get	89
	v128.store	816
	local.get	4
	local.get	90
	v128.store	800
	local.get	4
	local.get	91
	v128.store	784
	local.get	4
	local.get	92
	v128.store	768
	local.get	4
	local.get	22
	v128.store	752
	local.get	4
	local.get	93
	v128.store	736
.Ltmp5714:
	.loc	38 443 13 is_stmt 1
	local.get	8
	local.get	9
	local.get	2
	i32.const	.Lalloc_cab916602395946b4285251e481c2df8
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5715:
.LBB27_331:
	.loc	38 0 13 is_stmt 0
	end_block
	.loc	14 2326 9 is_stmt 1
	local.get	0
	local.get	9
	i32.store8	1124
	.loc	14 2328 30
	local.get	0
	local.get	0
	i32.load8_u	904
	.loc	14 2328 9 is_stmt 0
	i32.store8	1125
	local.get	2
	i32.const	536870908
.Ltmp5716:
	.loc	30 1851 23 is_stmt 1
	i32.and 
	i32.eqz
	br_if   	0
.Ltmp5717:
	.loc	30 0 23 is_stmt 0
	i32.const	0
	local.get	2
	i32.const	-4
.Ltmp5718:
	.loc	31 2155 12 is_stmt 1
	i32.and 
	i32.sub 
	local.set	8
	v128.const	-1, -1, -1, -1
	local.set	21
	local.get	1
	local.set	9
.Ltmp5719:
.LBB27_333:
	.loc	1 551 14
	loop    	
	local.get	9
	v128.load	0:p2align=2
.Ltmp5720:
	.loc	42 3812 14
	f32x4.abs
.Ltmp5721:
	.loc	42 1991 14
	v128.const	0x1.93e594p99, 0x1.93e594p99, 0x1.93e594p99, 0x1.93e594p99
	local.tee	22
	f32x4.lt
.Ltmp5722:
	.loc	42 2138 14
	local.get	21
	v128.and
	local.set	21
	local.get	9
	i32.const	16
.Ltmp5723:
	.loc	35 863 18
	i32.add 
	local.set	9
	local.get	8
	i32.const	4
.Ltmp5724:
	.loc	31 2155 12
	i32.add 
	local.tee	8
	br_if   	0
.Ltmp5725:
	.loc	31 0 12 is_stmt 0
	end_loop
.Ltmp5726:
	.loc	42 2198 34 is_stmt 1
	local.get	21
	v128.not
	.loc	42 2198 14 is_stmt 0
	v128.any_true
	i32.eqz
	br_if   	0
.Ltmp5727:
	.loc	42 0 14
	i32.const	0
	local.get	2
	i32.const	-4
.Ltmp5728:
	.loc	31 2155 12 is_stmt 1
	i32.and 
	i32.sub 
	local.set	8
	v128.const	-1, -1, -1, -1
	local.set	21
	local.get	1
	local.set	9
.Ltmp5729:
.LBB27_336:
	.loc	1 551 14
	loop    	
	local.get	9
	v128.load	0:p2align=2
.Ltmp5730:
	.loc	42 3812 14
	f32x4.abs
.Ltmp5731:
	.loc	42 1991 14
	local.get	22
	f32x4.lt
.Ltmp5732:
	.loc	42 2138 14
	local.get	21
	v128.and
	local.set	21
	local.get	9
	i32.const	16
.Ltmp5733:
	.loc	35 863 18
	i32.add 
	local.set	9
	local.get	8
	i32.const	4
.Ltmp5734:
	.loc	31 2155 12
	i32.add 
	local.tee	8
	br_if   	0
.Ltmp5735:
	.loc	31 0 12 is_stmt 0
	end_loop
	local.get	0
	i64.const	-1
	.loc	14 2333 40 is_stmt 1
	local.get	0
	i64.load	0
	i64.const	1
.Ltmp5736:
	.loc	13 2428 13
	i64.add 
	local.tee	126
	local.get	126
	i64.eqz
	i64.select
.Ltmp5737:
	.loc	14 2333 9
	i64.store	0
	local.get	0
	i32.const	2
	i32.const	0
.Ltmp5738:
	.loc	42 2188 14
	v128.const	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
	v128.const	0, 0, 128, 63, 0, 0, 128, 63, 0, 0, 128, 63, 0, 0, 128, 63
	local.get	21
	v128.bitselect
.Ltmp5739:
	.loc	36 185 12
	local.tee	21
	i32x4.extract_lane	1
	i32.select
	local.get	21
	i32x4.extract_lane	0
	i32.const	0
	i32.ne  
	i32.or  
	i32.const	4
	i32.const	0
	local.get	21
	i32x4.extract_lane	2
	i32.select
	i32.or  
	i32.const	8
	i32.const	0
	local.get	21
	i32x4.extract_lane	3
	i32.select
	i32.or  
.Ltmp5740:
	.loc	14 2332 9
	i32.store	8
	block   	
	local.get	2
	i32.const	2
.Ltmp5741:
	.loc	11 961 18
	i32.shl 
.Ltmp5742:
	.loc	32 25 13
	local.tee	9
	i32.eqz
	br_if   	0
	.loc	32 0 13 is_stmt 0
	local.get	1
	i32.const	0
	.loc	32 25 13
	local.get	9
	memory.fill	0
.Ltmp5743:
.LBB27_339:
	.loc	32 0 13
	end_block
	.loc	14 2335 21 is_stmt 1
	local.get	4
	local.get	16
	i32.load	8
	i32.store	1144
	local.get	4
	local.get	16
	i64.load	0:p2align=2
	i64.store	1136
.Ltmp5744:
	.loc	14 2338 14
	local.get	15
	local.get	4
	i32.const	1136
	i32.add 
	.loc	14 2338 40 is_stmt 0
	local.get	0
	i32.load	808
	local.get	0
	i32.load	812
.Ltmp5745:
	.loc	14 2336 20 is_stmt 1
	local.get	0
	i32.load	880
.Ltmp5746:
	.loc	14 2338 14
	local.tee	9
	call	_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults
	local.get	0
	i32.const	1024
	.loc	14 2339 9
	i32.add 
	.loc	14 2340 14
	local.get	4
	i32.const	1136
	i32.add 
	.loc	14 2340 40 is_stmt 0
	local.get	0
	i32.load	816
	local.get	0
	i32.load	820
	.loc	14 2340 14
	local.get	9
	call	_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults
	local.get	0
	i64.const	0
	.loc	14 2341 9 is_stmt 1
	i64.store	800
.Ltmp5747:
.LBB27_340:
	.loc	14 0 9 is_stmt 0
	end_block
	.loc	14 2342 6 is_stmt 1
	local.get	4
	i32.const	2528
	i32.add 
	global.set	__stack_pointer
	return
.LBB27_341:
	.loc	14 0 6 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	1072
	local.get	4
	local.get	125
	v128.store	1040
	local.get	4
	local.get	124
	v128.store	976
.Ltmp5748:
	local.get	4
	local.get	94
	v128.store	912
	local.get	4
	local.get	85
	v128.store	896
	local.get	4
	local.get	86
	v128.store	880
	local.get	4
	local.get	87
	v128.store	864
	local.get	4
	local.get	88
	v128.store	848
	local.get	4
	local.get	21
	v128.store	832
	local.get	4
	local.get	89
	v128.store	816
	local.get	4
	local.get	90
	v128.store	800
	local.get	4
	local.get	91
	v128.store	784
	local.get	4
	local.get	92
	v128.store	768
	local.get	4
	local.get	22
	v128.store	752
	local.get	4
	local.get	93
	v128.store	736
.Ltmp5749:
	.loc	38 581 13 is_stmt 1
	local.get	73
	local.get	8
	local.get	8
	i32.const	.Lalloc_db7c42041d7aaaba4839906f37294547
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5750:
.LBB27_342:
	.loc	38 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	121
	v128.store	1072
	local.get	4
	local.get	125
	v128.store	1040
	local.get	4
	local.get	124
	v128.store	976
.Ltmp5751:
	local.get	4
	local.get	94
	v128.store	912
	local.get	4
	local.get	85
	v128.store	896
	local.get	4
	local.get	86
	v128.store	880
	local.get	4
	local.get	87
	v128.store	864
	local.get	4
	local.get	88
	v128.store	848
	local.get	4
	local.get	21
	v128.store	832
	local.get	4
	local.get	89
	v128.store	816
	local.get	4
	local.get	90
	v128.store	800
	local.get	4
	local.get	91
	v128.store	784
	local.get	4
	local.get	92
	v128.store	768
	local.get	4
	local.get	22
	v128.store	752
	local.get	4
	local.get	93
	v128.store	736
.Ltmp5752:
	.loc	14 1403 42 is_stmt 1
	local.get	8
	local.get	8
	i32.const	.Lalloc_33746731a929bad63709ddedbca82f5f
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
	end_function
