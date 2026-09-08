_RNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB5_11LimiterCoreNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E18process_block_monoB5_:
.Lfunc_begin28:
	.loc	22 2308 0
	.functype	_RNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB5_11LimiterCoreNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E18process_block_monoB5_ (i32, i32, i32, i32) -> ()
	.local  	i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, v128, v128, v128, v128, i32, i32, i32, i32, i32, i32, i32, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, i32, i32, i32, i32, i32, i32, i32, i32, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, v128, f32, f32, v128, v128, v128, i64
	global.get	__stack_pointer
	i32.const	2528
	i32.sub 
	local.tee	4
	global.set	__stack_pointer
	local.get	3
	i32.const	2
.Ltmp4430:
	.loc	22 2309 21 prologue_end
	i32.shl 
	local.set	5
.Ltmp4431:
	.loc	22 0 0 is_stmt 0
	local.get	0
	i32.load	992
	local.set	6
	local.get	0
	i32.load	988
	local.set	7
	.loc	22 2310 21 is_stmt 1
	block   	
	block   	
	block   	
	block   	
	block   	
	local.get	0
	i32.load8_u	1125
	.loc	22 2310 43 is_stmt 0
	local.get	0
	i32.load8_u	904
	.loc	22 2310 21
	i32.ne  
	br_if   	0
	.loc	22 0 21
	local.get	6
	i32.const	4
.Ltmp4432:
	.loc	23 314 17 is_stmt 1
	i32.shl 
	local.set	8
	local.get	7
	local.set	9
.LBB28_2:
.Ltmp4433:
	.loc	23 180 28
	block   	
	loop    	
	local.get	8
	i32.eqz
	br_if   	1
.Ltmp4434:
	.loc	22 690 21
	local.get	9
	i32.load	12
.Ltmp4435:
	.loc	23 315 25
	br_if   	2
	.loc	23 0 25 is_stmt 0
	local.get	8
	i32.const	-16
	i32.add 
	local.set	8
	.loc	23 315 25
	local.get	9
	i32.load	4
	local.set	10
	local.get	9
	i32.load	0
	local.set	11
	local.get	9
	i32.const	16
	.loc	23 0 0
	i32.add 
	local.set	9
	.loc	23 315 25
	local.get	11
	local.get	10
	i32.eq  
	br_if   	0
	br      	2
.LBB28_5:
.Ltmp4436:
	.loc	23 180 28 is_stmt 1
	end_loop
	end_block
.Ltmp4437:
	.loc	22 2312 37
	local.get	0
	i32.load	1000
	i32.const	4
.Ltmp4438:
	.loc	23 314 17
	i32.shl 
	local.set	8
.Ltmp4439:
	.loc	22 2312 37
	local.get	0
	i32.load	996
	local.set	9
.LBB28_6:
.Ltmp4440:
	.loc	23 180 28
	block   	
	loop    	
	local.get	8
	i32.eqz
	br_if   	1
.Ltmp4441:
	.loc	22 690 21
	local.get	9
	i32.load	12
.Ltmp4442:
	.loc	23 315 25
	br_if   	2
	.loc	23 0 25 is_stmt 0
	local.get	8
	i32.const	-16
	i32.add 
	local.set	8
	.loc	23 315 25
	local.get	9
	i32.load	4
	local.set	10
	local.get	9
	i32.load	0
	local.set	11
	local.get	9
	i32.const	16
	.loc	23 0 0
	i32.add 
	local.set	9
	.loc	23 315 25
	local.get	11
	local.get	10
	i32.ne  
	br_if   	2
	br      	0
.LBB28_9:
.Ltmp4443:
	.loc	23 180 28 is_stmt 1
	end_loop
	end_block
	block   	
	block   	
	local.get	5
	local.get	2
	i32.gt_u
.Ltmp4444:
	.loc	21 1050 16
	br_if   	0
.Ltmp4445:
	.loc	21 0 16 is_stmt 0
	local.get	5
	local.set	11
	local.get	1
	local.set	9
.LBB28_11:
.Ltmp4446:
	.loc	2 1504 12 is_stmt 1
	loop    	
	local.get	11
	i32.eqz
	br_if   	2
	.loc	2 0 12 is_stmt 0
	local.get	9
	local.get	11
	i32.const	32
	local.get	11
	i32.const	32
.Ltmp4447:
	.loc	17 1077 12 is_stmt 1
	i32.lt_u
	i32.select
	local.tee	12
	i32.const	2
.Ltmp4448:
	.loc	6 863 18
	i32.shl 
	local.tee	10
	i32.add 
	local.set	13
	i32.const	0
	local.set	8
.Ltmp4449:
.LBB28_13:
	.loc	1 134 21
	loop    	
	local.get	9
	i32.load	0
	.loc	1 134 13 is_stmt 0
	local.get	8
	i32.or  
	local.set	8
	local.get	9
	i32.const	4
.Ltmp4450:
	.loc	24 656 28 is_stmt 1
	i32.add 
	local.set	9
	local.get	10
	i32.const	-4
.Ltmp4451:
	.loc	24 1714 9
	i32.add 
.Ltmp4452:
	.loc	23 180 28
	local.tee	10
	br_if   	0
	end_loop
.Ltmp4453:
	.loc	3 2054 74
	local.get	11
	local.get	12
	i32.sub 
	local.set	11
	local.get	13
	local.set	9
.Ltmp4454:
	.loc	1 136 12
	local.get	8
	i32.eqz
	br_if   	0
	end_loop
	i32.const	0
	local.set	14
.Ltmp4455:
	.loc	22 2314 12
	br      	3
.Ltmp4456:
.LBB28_16:
	.loc	22 0 12 is_stmt 0
	end_block
	i32.const	0
.Ltmp4457:
	.loc	42 443 13 is_stmt 1
	local.get	5
	local.get	2
	i32.const	.Lalloc_81e9b689ac7837328a8c5e199528fa13
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp4458:
.LBB28_17:
	.loc	42 0 13 is_stmt 0
	end_block
	i32.const	1
	local.set	14
	local.get	0
	i32.load8_u	1124
.Ltmp4459:
	.loc	22 2314 12 is_stmt 1
	i32.eqz
	br_if   	1
.Ltmp4460:
	.loc	22 663 57
	block   	
	block   	
	block   	
	block   	
	local.get	0
	i32.load	1016
	.loc	22 663 31 is_stmt 0
	local.tee	9
	local.get	0
	i32.load	984
.Ltmp4461:
	.loc	17 1077 12 is_stmt 1
	local.tee	8
	local.get	9
	local.get	8
	i32.lt_u
	i32.select
.Ltmp4462:
	.loc	39 304 12
	local.tee	7
	i32.eqz
	br_if   	0
.Ltmp4463:
	.loc	39 0 12 is_stmt 0
	local.get	0
	i32.load	1012
	local.set	8
	local.get	0
	i32.load	980
	local.set	9
.LBB28_20:
.Ltmp4464:
	.loc	22 664 26 is_stmt 1
	loop    	
	local.get	8
	i32.load	0
.Ltmp4465:
	.loc	22 665 42
	local.tee	10
	i32.eqz
	br_if   	2
	local.get	9
	local.get	3
	local.get	10
	i32.rem_u
	.loc	22 665 24 is_stmt 0
	local.get	9
	i32.load	0
	.loc	22 665 23
	i32.add 
	.loc	22 665 22
	local.get	10
	i32.rem_u
	.loc	22 665 13
	i32.store	0
	local.get	8
	i32.const	12
.Ltmp4466:
	.loc	39 304 12 is_stmt 1
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
.LBB28_22:
	end_loop
	end_block
.Ltmp4467:
	.loc	22 2316 26
	local.get	0
	i32.load	920
.Ltmp4468:
	.loc	22 455 44
	local.tee	8
	i32.eqz
	br_if   	1
.Ltmp4469:
	.loc	22 2316 26
	local.get	0
	i32.load	916
	local.set	9
.Ltmp4470:
	.loc	22 455 44
	local.get	0
	local.get	3
	local.get	8
	i32.rem_u
	.loc	22 455 23 is_stmt 0
	local.get	0
	i32.load	800
	.loc	22 455 22
	i32.add 
	.loc	22 455 21
	local.get	8
	i32.rem_u
	.loc	22 455 9
	i32.store	800
	.loc	22 456 44 is_stmt 1
	local.get	9
	i32.eqz
	br_if   	2
	local.get	0
	local.get	3
	local.get	9
	i32.rem_u
	.loc	22 456 23 is_stmt 0
	local.get	0
	i32.load	804
	.loc	22 456 22
	i32.add 
	.loc	22 456 21
	local.get	9
	i32.rem_u
	.loc	22 456 9
	i32.store	804
	br      	5
.Ltmp4471:
.LBB28_25:
	.loc	22 0 9
	end_block
.Ltmp4472:
	.loc	22 665 42 is_stmt 1
	i32.const	.Lalloc_f5b0427df9b659e554a697ca46ce8b5a
	call	_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero
	unreachable
.Ltmp4473:
.LBB28_26:
	.loc	22 0 42 is_stmt 0
	end_block
.Ltmp4474:
	.loc	22 455 44 is_stmt 1
	i32.const	.Lalloc_c7e64e9e8489bc8e648659d0098d136c
	call	_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero
	unreachable
.LBB28_27:
	.loc	22 0 44 is_stmt 0
	end_block
	.loc	22 456 44 is_stmt 1
	i32.const	.Lalloc_34b23597fad9d85cb3bee6d64ebf77d5
	call	_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero
	unreachable
.Ltmp4475:
.LBB28_28:
	.loc	22 0 44 is_stmt 0
	end_block
	i32.const	0
	local.set	14
.LBB28_29:
	end_block
	local.get	6
	i32.const	4
.Ltmp4476:
	.loc	23 314 17 is_stmt 1
	i32.shl 
	local.set	9
	local.get	0
	i32.const	924
.Ltmp4477:
	.loc	22 2328 13
	i32.add 
	local.set	15
	local.get	0
	i32.const	912
	.loc	22 2327 13
	i32.add 
	local.set	16
.LBB28_30:
.Ltmp4478:
	.loc	23 180 28
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
.Ltmp4479:
	.loc	22 690 21
	block   	
	local.get	7
	i32.load	12
.Ltmp4480:
	.loc	23 315 25
	br_if   	0
	.loc	23 0 25 is_stmt 0
	local.get	9
	i32.const	-16
	i32.add 
	local.set	9
	.loc	23 315 25
	local.get	7
	i32.load	4
	local.set	8
	local.get	7
	i32.load	0
	local.set	10
	local.get	7
	i32.const	16
	.loc	23 0 0
	i32.add 
	local.set	7
	.loc	23 315 25
	local.get	10
	local.get	8
	i32.eq  
	br_if   	1
.LBB28_33:
	.loc	23 0 25
	end_block
	.loc	23 315 25
	end_loop
.Ltmp4481:
	.loc	22 1116 5 is_stmt 1
	block   	
	local.get	0
	i32.load	1016
	local.tee	9
	i32.eqz
	br_if   	0
	.loc	22 0 5 is_stmt 0
	local.get	9
	i32.const	12
	i32.mul 
	local.set	8
	local.get	0
	i32.load	1012
	local.tee	10
	local.set	9
.LBB28_35:
.Ltmp4482:
	.loc	23 180 28 is_stmt 1
	loop    	
	local.get	8
	i32.eqz
	br_if   	1
.Ltmp4483:
	.loc	22 390 34
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
.Ltmp4484:
	.loc	22 0 34 is_stmt 0
	local.get	8
	i32.const	-12
	.loc	23 315 25 is_stmt 1
	i32.add 
	local.set	8
.Ltmp4485:
	.loc	22 390 34
	local.get	9
	i32.load	8
	local.set	7
	local.get	9
	i32.const	12
.Ltmp4486:
	.loc	23 0 0 is_stmt 0
	i32.add 
	local.set	9
.Ltmp4487:
	.loc	22 390 34
	local.get	7
	local.get	10
	i32.load	8
	i32.eq  
.Ltmp4488:
	.loc	23 315 25 is_stmt 1
	br_if   	0
	br      	7
.LBB28_39:
.Ltmp4489:
	.loc	23 180 28
	end_loop
	end_block
.Ltmp4490:
	.loc	22 1117 12
	local.get	0
	i32.load	984
	local.tee	9
	i32.eqz
	br_if   	1
	.loc	22 0 12 is_stmt 0
	local.get	9
	i32.const	2
	i32.shl 
	local.set	7
	local.get	0
	i32.load	980
	local.set	8
	i32.const	0
	local.set	9
.LBB28_41:
.Ltmp4491:
	.loc	24 1714 9 is_stmt 1
	loop    	
	local.get	7
	local.get	9
	i32.eq  
.Ltmp4492:
	.loc	23 180 28
	br_if   	2
.Ltmp4493:
	.loc	23 0 0 is_stmt 0
	local.get	8
	local.get	9
	i32.add 
	local.set	10
	local.get	9
	i32.const	4
	.loc	23 315 25 is_stmt 1
	i32.add 
	local.set	9
	local.get	10
	i32.load	0
.Ltmp4494:
	.loc	22 1117 53
	local.get	8
	i32.load	0
	.loc	22 1117 43 is_stmt 0
	i32.ne  
.Ltmp4495:
	.loc	23 315 25 is_stmt 1
	br_if   	6
	br      	0
.Ltmp4496:
.LBB28_43:
	.loc	23 180 28
	end_loop
	end_block
.Ltmp4497:
	.loc	22 715 63
	local.get	0
	i32.load	1000
	i32.const	4
.Ltmp4498:
	.loc	23 314 17
	i32.shl 
	local.set	10
.Ltmp4499:
	.loc	22 715 63
	local.get	0
	i32.load	996
	local.set	9
.LBB28_44:
	.loc	22 0 63 is_stmt 0
	block   	
	loop    	
	local.get	10
.Ltmp4500:
	.loc	23 180 28 is_stmt 1
	local.tee	8
	i32.eqz
	br_if   	1
.Ltmp4501:
	.loc	22 690 21
	local.get	9
	i32.load	12
.Ltmp4502:
	.loc	23 315 25
	br_if   	1
	.loc	23 0 25 is_stmt 0
	local.get	8
	i32.const	-16
	i32.add 
	local.set	10
	.loc	23 315 25
	local.get	9
	i32.load	4
	local.set	7
	local.get	9
	i32.load	0
	local.set	11
	local.get	9
	i32.const	16
	.loc	23 0 0
	i32.add 
	local.set	9
	.loc	23 315 25
	local.get	11
	local.get	7
	i32.eq  
	br_if   	0
.LBB28_47:
	.loc	23 315 25 is_stmt 1
	end_loop
	end_block
.Ltmp4503:
	.loc	22 1116 5
	block   	
	local.get	0
	i32.load	1016
	local.tee	9
	i32.eqz
	br_if   	0
	.loc	22 0 5 is_stmt 0
	local.get	9
	i32.const	12
	i32.mul 
	local.set	10
	local.get	0
	i32.load	1012
	local.tee	7
	local.set	9
.LBB28_49:
.Ltmp4504:
	.loc	23 180 28 is_stmt 1
	loop    	
	local.get	10
	i32.eqz
	br_if   	1
.Ltmp4505:
	.loc	22 390 34
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
.Ltmp4506:
	.loc	22 0 34 is_stmt 0
	local.get	10
	i32.const	-12
	.loc	23 315 25 is_stmt 1
	i32.add 
	local.set	10
.Ltmp4507:
	.loc	22 390 34
	local.get	9
	i32.load	8
	local.set	11
	local.get	9
	i32.const	12
.Ltmp4508:
	.loc	23 0 0 is_stmt 0
	i32.add 
	local.set	9
.Ltmp4509:
	.loc	22 390 34
	local.get	11
	local.get	7
	i32.load	8
	i32.eq  
.Ltmp4510:
	.loc	23 315 25 is_stmt 1
	br_if   	0
	br      	5
.LBB28_53:
.Ltmp4511:
	.loc	23 180 28
	end_loop
	end_block
.Ltmp4512:
	.loc	22 1117 12
	block   	
	local.get	0
	i32.load	984
	local.tee	9
	i32.eqz
	br_if   	0
	.loc	22 0 12 is_stmt 0
	local.get	9
	i32.const	2
	i32.shl 
	local.set	11
	local.get	0
	i32.load	980
	local.set	10
	i32.const	0
	local.set	9
.LBB28_55:
.Ltmp4513:
	.loc	24 1714 9 is_stmt 1
	loop    	
	local.get	11
	local.get	9
	i32.eq  
.Ltmp4514:
	.loc	23 180 28
	br_if   	1
.Ltmp4515:
	.loc	23 0 0 is_stmt 0
	local.get	10
	local.get	9
	i32.add 
	local.set	7
	local.get	9
	i32.const	4
	.loc	23 315 25 is_stmt 1
	i32.add 
	local.set	9
	local.get	7
	i32.load	0
.Ltmp4516:
	.loc	22 1117 53
	local.get	10
	i32.load	0
	.loc	22 1117 43 is_stmt 0
	i32.ne  
.Ltmp4517:
	.loc	23 315 25 is_stmt 1
	br_if   	5
	br      	0
.LBB28_57:
.Ltmp4518:
	.loc	23 180 28
	end_loop
	end_block
.Ltmp4519:
	.loc	22 3192 12
	local.get	8
	i32.eqz
	br_if   	1
.LBB28_58:
	.loc	22 0 12 is_stmt 0
	end_block
.Ltmp4520:
	.loc	22 3332 24 is_stmt 1
	local.get	4
	local.get	15
	call	_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_
.Ltmp4521:
	.loc	22 3342 27
	local.get	0
	i32.load	804
	local.set	17
.Ltmp4522:
	.loc	22 3341 27
	local.get	0
	i32.load	800
	local.set	18
.Ltmp4523:
	.loc	22 3338 21
	local.get	0
	i32.load8_u	785
	local.set	9
.Ltmp4524:
	.loc	22 3337 19
	local.get	0
	i32.load8_u	784
	local.set	8
.Ltmp4525:
	.loc	22 3340 16
	local.get	0
	i32.load	920
	local.set	19
.Ltmp4526:
	.loc	22 3339 16
	local.get	0
	i32.load	916
	local.set	12
	local.get	4
	i32.const	1136
	i32.add 
	i32.const	0
	i32.const	1024
	memory.fill	0
.Ltmp4527:
	.loc	22 3346 32
	local.get	4
	i32.const	2160
	i32.add 
	local.get	15
	local.get	12
	local.get	19
	call	_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E3newB5_
.Ltmp4528:
	.loc	44 446 20
	block   	
	local.get	3
	br_if   	0
.Ltmp4529:
	.loc	22 3416 31
	local.get	4
	i32.load	2176
	local.set	6
	br      	2
.LBB28_60:
	.loc	22 0 31 is_stmt 0
	end_block
	local.get	3
	i32.const	5
.Ltmp4530:
	.loc	21 3756 21 is_stmt 1
	i32.shr_u
	local.get	3
	i32.const	31
.Ltmp4531:
	.loc	21 3757 21
	i32.and 
	i32.const	0
.Ltmp4532:
	.loc	21 3758 16
	i32.ne  
	i32.add 
	local.set	20
.Ltmp4533:
	.loc	22 3338 21
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
.Ltmp4534:
	.loc	22 3337 19
	i32.and 
	v128.select
	local.set	24
	local.get	4
	i32.load	2208
	local.set	25
	local.get	4
	i32.load	2212
	local.set	26
	local.get	4
	i32.load	2204
	local.set	27
	local.get	4
	i32.load	2200
	local.set	28
	local.get	4
	i32.load	2180
	local.set	29
	local.get	4
	i32.load	2196
	local.set	7
	local.get	4
	i32.load	2192
	local.set	11
	local.get	4
	i32.load	2188
	local.set	30
	local.get	4
	i32.load	2184
	local.set	31
.Ltmp4535:
	.loc	22 1759 23
	local.get	4
	v128.load	176
	local.set	32
	local.get	4
	v128.load	160
	local.set	33
	local.get	4
	v128.load	144
	local.set	34
	local.get	4
	v128.load	128
	local.set	35
	local.get	4
	v128.load	112
	local.set	36
	local.get	4
	v128.load	96
	local.set	37
	local.get	4
	v128.load	80
	local.set	38
	local.get	4
	v128.load	64
	local.set	39
	local.get	4
	v128.load	48
	local.set	40
	local.get	4
	v128.load	32
	local.set	41
	local.get	4
	v128.load	16
	local.set	42
	local.get	4
	v128.load	0
	local.set	43
	local.get	4
	i32.load	2176
	local.set	6
	local.get	1
	local.set	44
	local.get	2
	local.set	45
	i32.const	0
	local.set	46
	local.get	3
	local.set	47
	i32.const	0
	local.set	48
.Ltmp4536:
.LBB28_61:
	.loc	22 0 23 is_stmt 0
	loop    	
	local.get	47
	i32.const	32
	local.get	47
	i32.const	32
.Ltmp4537:
	.loc	21 2584 13 is_stmt 1
	i32.lt_u
	i32.select
	local.set	49
.Ltmp4538:
	.loc	17 1916 50
	block   	
	local.get	3
	local.get	48
	i32.eq  
.Ltmp4539:
	.loc	18 900 12
	local.tee	50
	br_if   	0
.Ltmp4540:
	.loc	18 0 12 is_stmt 0
	local.get	49
	i32.const	1
	local.get	49
	i32.const	1
	i32.gt_u
	i32.select
	local.set	51
	local.get	0
	v128.load	768
	local.set	52
	local.get	0
	v128.load	752
	local.set	53
	local.get	0
	v128.load	736
	local.set	54
	local.get	0
	v128.load	720
	local.set	55
	local.get	0
	v128.load	704
	local.set	56
	local.get	0
	v128.load	688
	local.set	57
	local.get	0
	v128.load	672
	local.set	58
	local.get	0
	v128.load	656
	local.set	59
	local.get	0
	v128.load	640
	local.set	60
	local.get	0
	v128.load	624
	local.set	61
	local.get	0
	v128.load	608
	local.set	62
	local.get	0
	v128.load	592
	local.set	63
	local.get	0
	v128.load	576
	local.set	64
	local.get	0
	v128.load	560
	local.set	65
	local.get	0
	v128.load	544
	local.set	66
	local.get	0
	v128.load	528
	local.set	67
	local.get	0
	v128.load	512
	local.set	68
	local.get	0
	v128.load	496
	local.set	69
	local.get	0
	v128.load	480
	local.set	70
	local.get	0
	v128.load	464
	local.set	71
	local.get	0
	v128.load	448
	local.set	72
	local.get	0
	v128.load	432
	local.set	73
	local.get	0
	v128.load	416
	local.set	74
	local.get	0
	v128.load	400
	local.set	75
	local.get	0
	v128.load	384
	local.set	76
	local.get	0
	v128.load	368
	local.set	77
	local.get	0
	v128.load	352
	local.set	78
	local.get	0
	v128.load	336
	local.set	79
	local.get	0
	v128.load	320
	local.set	80
	local.get	0
	v128.load	304
	local.set	81
	local.get	0
	v128.load	288
	local.set	82
	local.get	0
	v128.load	272
	local.set	83
	local.get	0
	v128.load	256
	local.set	84
	local.get	0
	v128.load	240
	local.set	85
	local.get	0
	v128.load	224
	local.set	86
	local.get	0
	v128.load	208
	local.set	87
	local.get	0
	v128.load	192
	local.set	88
	local.get	0
	v128.load	176
	local.set	89
	local.get	0
	v128.load	160
	local.set	90
	local.get	0
	v128.load	144
	local.set	91
	local.get	0
	v128.load	128
	local.set	92
	local.get	0
	v128.load	112
	local.set	93
	local.get	0
	v128.load	96
	local.set	94
	local.get	0
	v128.load	80
	local.set	95
	local.get	0
	v128.load	64
	local.set	96
	local.get	0
	v128.load	48
	local.set	97
	local.get	0
	v128.load	32
	local.set	98
	local.get	0
	v128.load	16
	local.set	99
	local.get	4
	i32.const	1136
	i32.add 
	local.set	10
	local.get	44
	local.set	13
	local.get	45
	local.set	8
	local.get	46
	local.set	9
	local.get	33
	local.set	21
	local.get	34
	local.set	22
	local.get	35
	local.set	100
	local.get	36
	local.set	101
	local.get	37
	local.set	102
	local.get	38
	local.set	103
	local.get	39
	local.set	104
	local.get	40
	local.set	105
	local.get	41
	local.set	106
	local.get	42
	local.set	107
.LBB28_63:
	loop    	
	local.get	43
	local.set	42
	local.get	107
	local.set	41
	local.get	106
	local.set	40
	local.get	105
	local.set	39
	local.get	104
	local.set	38
	local.get	103
	local.set	37
	local.get	102
	local.set	36
	local.get	101
	local.set	35
	local.get	100
	local.set	34
	local.get	22
	local.set	33
	local.get	21
	local.set	32
.Ltmp4541:
	.loc	42 568 12 is_stmt 1
	block   	
	block   	
	local.get	9
	local.get	2
	i32.gt_u
	br_if   	0
.Ltmp4542:
	.loc	42 0 12 is_stmt 0
	local.get	8
	i32.const	3
.Ltmp4543:
	.loc	42 438 16 is_stmt 1
	i32.gt_u
	br_if   	1
	.loc	42 0 16 is_stmt 0
	i32.const	0
	i32.const	4
	.loc	42 443 13 is_stmt 1
	local.get	8
	i32.const	.Lalloc_5f4e05826d69b5e832dc96c63f325058
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp4544:
.LBB28_66:
	.loc	42 0 13 is_stmt 0
	end_block
.Ltmp4545:
	.loc	42 569 13 is_stmt 1
	local.get	9
	local.get	2
	local.get	2
	i32.const	.Lalloc_d37239ff881951c49620cb5a9c000a8e
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp4546:
.LBB28_67:
	.loc	42 0 13 is_stmt 0
	end_block
.Ltmp4547:
	.loc	4 551 14 is_stmt 1
	local.get	10
	local.get	96
	local.get	13
	v128.load	0:p2align=2
.Ltmp4548:
	.loc	5 3867 14
	local.tee	43
	f32x4.mul
.Ltmp4549:
	.loc	5 3845 14
	v128.const	0x0p0, 0x0p0, 0x0p0, 0x0p0
.Ltmp4550:
	.loc	5 3845 14 is_stmt 0
	local.tee	21
	f32x4.add
.Ltmp4551:
	.loc	5 3867 14 is_stmt 1
	local.get	92
	local.get	42
	f32x4.mul
.Ltmp4552:
	.loc	5 3845 14
	f32x4.add
.Ltmp4553:
	.loc	5 3867 14
	local.get	88
	local.get	41
	f32x4.mul
.Ltmp4554:
	.loc	5 3845 14
	f32x4.add
.Ltmp4555:
	.loc	5 3867 14
	local.get	84
	local.get	40
	f32x4.mul
.Ltmp4556:
	.loc	5 3845 14
	f32x4.add
.Ltmp4557:
	.loc	5 3867 14
	local.get	80
	local.get	39
	f32x4.mul
.Ltmp4558:
	.loc	5 3845 14
	f32x4.add
.Ltmp4559:
	.loc	5 3867 14
	local.get	76
	local.get	38
	f32x4.mul
.Ltmp4560:
	.loc	5 3845 14
	f32x4.add
.Ltmp4561:
	.loc	5 3867 14
	local.get	72
	local.get	37
	f32x4.mul
.Ltmp4562:
	.loc	5 3845 14
	f32x4.add
.Ltmp4563:
	.loc	5 3867 14
	local.get	68
	local.get	36
	f32x4.mul
.Ltmp4564:
	.loc	5 3845 14
	f32x4.add
.Ltmp4565:
	.loc	5 3867 14
	local.get	64
	local.get	35
	f32x4.mul
.Ltmp4566:
	.loc	5 3845 14
	f32x4.add
.Ltmp4567:
	.loc	5 3867 14
	local.get	60
	local.get	34
	f32x4.mul
.Ltmp4568:
	.loc	5 3845 14
	f32x4.add
.Ltmp4569:
	.loc	5 3867 14
	local.get	56
	local.get	33
	f32x4.mul
.Ltmp4570:
	.loc	5 3845 14
	f32x4.add
.Ltmp4571:
	.loc	5 3867 14
	local.get	52
	local.get	32
	f32x4.mul
.Ltmp4572:
	.loc	5 3845 14
	f32x4.add
.Ltmp4573:
	.loc	5 3812 14
	f32x4.abs
.Ltmp4574:
	.loc	5 3867 14
	local.get	97
	local.get	43
	f32x4.mul
.Ltmp4575:
	.loc	5 3845 14
	local.get	21
	f32x4.add
.Ltmp4576:
	.loc	5 3867 14
	local.get	93
	local.get	42
	f32x4.mul
.Ltmp4577:
	.loc	5 3845 14
	f32x4.add
.Ltmp4578:
	.loc	5 3867 14
	local.get	89
	local.get	41
	f32x4.mul
.Ltmp4579:
	.loc	5 3845 14
	f32x4.add
.Ltmp4580:
	.loc	5 3867 14
	local.get	85
	local.get	40
	f32x4.mul
.Ltmp4581:
	.loc	5 3845 14
	f32x4.add
.Ltmp4582:
	.loc	5 3867 14
	local.get	81
	local.get	39
	f32x4.mul
.Ltmp4583:
	.loc	5 3845 14
	f32x4.add
.Ltmp4584:
	.loc	5 3867 14
	local.get	77
	local.get	38
	f32x4.mul
.Ltmp4585:
	.loc	5 3845 14
	f32x4.add
.Ltmp4586:
	.loc	5 3867 14
	local.get	73
	local.get	37
	f32x4.mul
.Ltmp4587:
	.loc	5 3845 14
	f32x4.add
.Ltmp4588:
	.loc	5 3867 14
	local.get	69
	local.get	36
	f32x4.mul
.Ltmp4589:
	.loc	5 3845 14
	f32x4.add
.Ltmp4590:
	.loc	5 3867 14
	local.get	65
	local.get	35
	f32x4.mul
.Ltmp4591:
	.loc	5 3845 14
	f32x4.add
.Ltmp4592:
	.loc	5 3867 14
	local.get	61
	local.get	34
	f32x4.mul
.Ltmp4593:
	.loc	5 3845 14
	f32x4.add
.Ltmp4594:
	.loc	5 3867 14
	local.get	57
	local.get	33
	f32x4.mul
.Ltmp4595:
	.loc	5 3845 14
	f32x4.add
.Ltmp4596:
	.loc	5 3867 14
	local.get	53
	local.get	32
	f32x4.mul
.Ltmp4597:
	.loc	5 3845 14
	f32x4.add
.Ltmp4598:
	.loc	5 3812 14
	f32x4.abs
.Ltmp4599:
	.loc	5 3867 14
	local.get	98
	local.get	43
	f32x4.mul
.Ltmp4600:
	.loc	5 3845 14
	local.get	21
	f32x4.add
.Ltmp4601:
	.loc	5 3867 14
	local.get	94
	local.get	42
	f32x4.mul
.Ltmp4602:
	.loc	5 3845 14
	f32x4.add
.Ltmp4603:
	.loc	5 3867 14
	local.get	90
	local.get	41
	f32x4.mul
.Ltmp4604:
	.loc	5 3845 14
	f32x4.add
.Ltmp4605:
	.loc	5 3867 14
	local.get	86
	local.get	40
	f32x4.mul
.Ltmp4606:
	.loc	5 3845 14
	f32x4.add
.Ltmp4607:
	.loc	5 3867 14
	local.get	82
	local.get	39
	f32x4.mul
.Ltmp4608:
	.loc	5 3845 14
	f32x4.add
.Ltmp4609:
	.loc	5 3867 14
	local.get	78
	local.get	38
	f32x4.mul
.Ltmp4610:
	.loc	5 3845 14
	f32x4.add
.Ltmp4611:
	.loc	5 3867 14
	local.get	74
	local.get	37
	f32x4.mul
.Ltmp4612:
	.loc	5 3845 14
	f32x4.add
.Ltmp4613:
	.loc	5 3867 14
	local.get	70
	local.get	36
	f32x4.mul
.Ltmp4614:
	.loc	5 3845 14
	f32x4.add
.Ltmp4615:
	.loc	5 3867 14
	local.get	66
	local.get	35
	f32x4.mul
.Ltmp4616:
	.loc	5 3845 14
	f32x4.add
.Ltmp4617:
	.loc	5 3867 14
	local.get	62
	local.get	34
	f32x4.mul
.Ltmp4618:
	.loc	5 3845 14
	f32x4.add
.Ltmp4619:
	.loc	5 3867 14
	local.get	58
	local.get	33
	f32x4.mul
.Ltmp4620:
	.loc	5 3845 14
	f32x4.add
.Ltmp4621:
	.loc	5 3867 14
	local.get	54
	local.get	32
	f32x4.mul
.Ltmp4622:
	.loc	5 3845 14
	f32x4.add
.Ltmp4623:
	.loc	5 3812 14
	f32x4.abs
.Ltmp4624:
	.loc	5 3867 14
	local.get	99
	local.get	43
	f32x4.mul
.Ltmp4625:
	.loc	5 3845 14
	local.get	21
	f32x4.add
.Ltmp4626:
	.loc	5 3867 14
	local.get	95
	local.get	42
	f32x4.mul
.Ltmp4627:
	.loc	5 3845 14
	f32x4.add
.Ltmp4628:
	.loc	5 3867 14
	local.get	91
	local.get	41
	f32x4.mul
.Ltmp4629:
	.loc	5 3845 14
	f32x4.add
.Ltmp4630:
	.loc	5 3867 14
	local.get	87
	local.get	40
	f32x4.mul
.Ltmp4631:
	.loc	5 3845 14
	f32x4.add
.Ltmp4632:
	.loc	5 3867 14
	local.get	83
	local.get	39
	f32x4.mul
.Ltmp4633:
	.loc	5 3845 14
	f32x4.add
.Ltmp4634:
	.loc	5 3867 14
	local.get	79
	local.get	38
	f32x4.mul
.Ltmp4635:
	.loc	5 3845 14
	f32x4.add
.Ltmp4636:
	.loc	5 3867 14
	local.get	75
	local.get	37
	f32x4.mul
.Ltmp4637:
	.loc	5 3845 14
	f32x4.add
.Ltmp4638:
	.loc	5 3867 14
	local.get	71
	local.get	36
	f32x4.mul
.Ltmp4639:
	.loc	5 3845 14
	f32x4.add
.Ltmp4640:
	.loc	5 3867 14
	local.get	67
	local.get	35
	f32x4.mul
.Ltmp4641:
	.loc	5 3845 14
	f32x4.add
.Ltmp4642:
	.loc	5 3867 14
	local.get	63
	local.get	34
	f32x4.mul
.Ltmp4643:
	.loc	5 3845 14
	f32x4.add
.Ltmp4644:
	.loc	5 3867 14
	local.get	59
	local.get	33
	f32x4.mul
.Ltmp4645:
	.loc	5 3845 14
	f32x4.add
.Ltmp4646:
	.loc	5 3867 14
	local.get	55
	local.get	32
	f32x4.mul
.Ltmp4647:
	.loc	5 3845 14
	f32x4.add
.Ltmp4648:
	.loc	5 3812 14
	f32x4.abs
.Ltmp4649:
	.loc	5 3812 14 is_stmt 0
	local.get	37
	f32x4.abs
.Ltmp4650:
	.loc	5 3928 9 is_stmt 1
	f32x4.pmax
	f32x4.pmax
	f32x4.pmax
	f32x4.pmax
.Ltmp4651:
	.loc	4 551 14
	v128.store	0:p2align=2
	local.get	13
	i32.const	16
.Ltmp4652:
	.loc	17 1916 50
	i32.add 
	local.set	13
	local.get	8
	i32.const	-4
	i32.add 
	local.set	8
	local.get	9
	i32.const	4
	i32.add 
	local.set	9
	local.get	10
	i32.const	16
	i32.add 
	local.set	10
	local.get	33
	local.set	21
	local.get	34
	local.set	22
	local.get	35
	local.set	100
	local.get	36
	local.set	101
	local.get	37
	local.set	102
	local.get	38
	local.set	103
	local.get	39
	local.set	104
	local.get	40
	local.set	105
	local.get	41
	local.set	106
	local.get	42
	local.set	107
	local.get	51
	i32.const	-1
	i32.add 
.Ltmp4653:
	.loc	18 900 12
	local.tee	51
	br_if   	0
.LBB28_68:
	end_loop
	end_block
.Ltmp4654:
	.loc	22 1765 5
	local.get	4
	local.get	32
	v128.store	176
	local.get	4
	local.get	33
	v128.store	160
	local.get	4
	local.get	34
	v128.store	144
	local.get	4
	local.get	35
	v128.store	128
	local.get	4
	local.get	36
	v128.store	112
	local.get	4
	local.get	37
	v128.store	96
	local.get	4
	local.get	38
	v128.store	80
	local.get	4
	local.get	39
	v128.store	64
	local.get	4
	local.get	40
	v128.store	48
	local.get	4
	local.get	41
	v128.store	32
	local.get	4
	local.get	42
	v128.store	16
	local.get	4
	local.get	43
	v128.store	0
.Ltmp4655:
	.loc	22 3360 19
	block   	
	local.get	50
	br_if   	0
	.loc	22 0 19 is_stmt 0
	local.get	3
	local.get	48
	i32.sub 
	local.tee	9
	i32.const	32
	local.get	9
	i32.const	32
	i32.lt_u
	i32.select
	local.set	108
	i32.const	0
	local.set	109
	local.get	4
	v128.load	336
	local.set	59
	local.get	4
	v128.load	304
	local.set	60
	local.get	4
	v128.load	240
	local.set	61
	local.get	4
	v128.load	352
	local.set	58
	local.get	4
	v128.load	272
	local.set	56
	local.get	4
	v128.load	208
	local.set	57
.LBB28_70:
.Ltmp4656:
	.loc	22 1575 16 is_stmt 1
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
	local.get	0
	i32.load	916
.Ltmp4657:
	.loc	22 1580 33
	local.tee	9
	local.get	30
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp4658:
	.loc	22 1148 8
	local.get	9
	local.get	8
	local.get	9
	i32.lt_u
	i32.select
	i32.sub 
.Ltmp4659:
	.loc	22 1588 14
	local.tee	110
	i32.sub 
.Ltmp4660:
	.loc	22 1578 28
	local.tee	8
	local.get	9
	local.get	31
	local.get	17
	i32.add 
	local.tee	10
	i32.const	0
.Ltmp4661:
	.loc	22 1148 8
	local.get	9
	local.get	10
	local.get	9
	i32.lt_u
	i32.select
	i32.sub 
.Ltmp4662:
	.loc	22 1586 14
	local.tee	111
	i32.sub 
	local.tee	10
	local.get	9
	local.get	17
	i32.const	1
.Ltmp4663:
	.loc	22 1577 25
	i32.add 
	local.tee	13
	i32.const	0
.Ltmp4664:
	.loc	22 1148 8
	local.get	9
	local.get	13
	local.get	9
	i32.lt_u
	i32.select
	i32.sub 
.Ltmp4665:
	.loc	22 1585 14
	local.tee	112
	i32.sub 
.Ltmp4666:
	.loc	22 1576 16
	local.tee	13
	local.get	0
	i32.load	920
.Ltmp4667:
	.loc	22 1584 14
	local.get	18
	i32.sub 
	.loc	22 1583 14
	local.tee	51
	local.get	9
	local.get	17
	i32.sub 
.Ltmp4668:
	.loc	22 3368 21
	local.tee	9
	local.get	108
	local.get	109
	i32.sub 
.Ltmp4669:
	.loc	17 1077 12
	local.tee	50
	local.get	9
	local.get	50
	i32.lt_u
	i32.select
.Ltmp4670:
	.loc	17 1077 12 is_stmt 0
	local.tee	50
	local.get	51
	local.get	50
	i32.lt_u
	i32.select
.Ltmp4671:
	.loc	17 1077 12
	local.tee	50
	local.get	13
	local.get	50
	i32.lt_u
	i32.select
.Ltmp4672:
	.loc	17 1077 12
	local.tee	50
	local.get	10
	local.get	50
	i32.lt_u
	i32.select
.Ltmp4673:
	.loc	17 1077 12
	local.tee	50
	local.get	8
	local.get	50
	i32.lt_u
	i32.select
.Ltmp4674:
	.loc	22 3374 28 is_stmt 1
	local.tee	113
	local.get	109
	local.get	48
	i32.add 
.Ltmp4675:
	.loc	22 3376 55
	local.tee	114
	i32.add 
	i32.const	2
	i32.shl 
	local.tee	50
	local.get	114
	i32.const	2
.Ltmp4676:
	.loc	22 3374 28
	i32.shl 
.Ltmp4677:
	.loc	21 1050 16
	local.tee	114
	i32.lt_u
	br_if   	1
	local.get	50
	local.get	2
	i32.gt_u
	br_if   	1
.Ltmp4678:
	.loc	39 304 12
	block   	
	local.get	113
	i32.eqz
	br_if   	0
.Ltmp4679:
	.loc	39 0 12 is_stmt 0
	local.get	1
	local.get	114
	i32.const	2
	i32.shl 
	i32.add 
	local.set	115
.Ltmp4680:
	local.get	4
	i32.const	1136
	i32.add 
	local.get	109
	i32.const	4
	i32.shl 
	i32.add 
	local.set	116
.Ltmp4681:
	.loc	39 304 12
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
	local.get	51
	local.get	9
	local.get	51
	i32.lt_u
	i32.select
	local.tee	9
	local.get	49
	local.get	109
	i32.sub 
	local.tee	8
	local.get	9
	local.get	8
	i32.lt_u
	i32.select
	i32.const	1073741823
	i32.and 
	local.set	117
	i32.const	0
	local.set	13
.Ltmp4682:
	.loc	22 853 61 is_stmt 1
	local.get	4
	v128.load	288
	local.set	100
	.loc	22 853 44 is_stmt 0
	local.get	4
	v128.load	256
	local.set	102
.Ltmp4683:
	.loc	22 853 61
	local.get	4
	v128.load	224
	local.set	101
	.loc	22 853 44
	local.get	4
	v128.load	192
	local.set	22
	local.get	59
	local.set	106
	local.get	60
	local.set	103
	local.get	61
	local.set	104
.Ltmp4684:
.LBB28_74:
	.loc	5 3845 14 is_stmt 1
	loop    	
	local.get	102
	local.get	100
	f32x4.add
.Ltmp4685:
	.loc	5 3856 14
	local.get	56
	local.get	103
	v128.const	-0x1p0, -0x1p0, -0x1p0, -0x1p0
.Ltmp4686:
	.loc	5 3856 14 is_stmt 0
	local.tee	21
	f32x4.add
.Ltmp4687:
	.loc	5 3929 13 is_stmt 1
	local.tee	107
	v128.const	0x0p0, 0x0p0, 0x0p0, 0x0p0
.Ltmp4688:
	.loc	5 3929 13 is_stmt 0
	local.tee	105
	f32x4.gt
.Ltmp4689:
	.loc	5 2188 14 is_stmt 1
	local.tee	103
	v128.bitselect
	local.set	102
.Ltmp4690:
	.loc	5 3845 14
	local.get	22
	local.get	101
	f32x4.add
.Ltmp4691:
	.loc	5 3856 14
	local.get	57
	local.get	104
	local.get	21
	f32x4.add
.Ltmp4692:
	.loc	5 3929 13
	local.tee	52
	local.get	105
	f32x4.gt
.Ltmp4693:
	.loc	5 2188 14
	local.tee	104
	v128.bitselect
	local.set	22
.Ltmp4694:
	.loc	5 2188 14 is_stmt 0
	local.get	100
	v128.const	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
.Ltmp4695:
	.loc	5 2188 14
	local.tee	21
	local.get	103
	v128.bitselect
	local.set	100
.Ltmp4696:
	.loc	5 2188 14
	local.get	101
	local.get	21
	local.get	104
	v128.bitselect
	local.set	101
.Ltmp4697:
	.loc	22 1502 26 is_stmt 1
	local.get	13
	local.get	17
	i32.add 
	i32.const	2
.Ltmp4698:
	.loc	22 1137 16
	i32.shl 
	local.tee	51
	i32.const	4
.Ltmp4699:
	.loc	22 1138 33
	i32.add 
	local.set	50
	local.get	51
	i32.const	3
.Ltmp4700:
	.loc	21 1050 16
	i32.or  
	local.get	7
	i32.ge_u
	br_if   	4
.Ltmp4701:
	.loc	21 0 16 is_stmt 0
	local.get	115
	local.get	13
	i32.const	4
	i32.shl 
	i32.add 
	local.tee	118
	v128.load	0:p2align=2
	local.set	53
	local.get	11
	local.get	51
	i32.const	2
.Ltmp4702:
	.loc	42 101 24 is_stmt 1
	i32.shl 
	local.tee	114
	i32.add 
	local.get	22
	local.get	116
	local.get	13
	i32.const	2
.Ltmp4703:
	.loc	22 0 0 is_stmt 0
	i32.shl 
	i32.const	2
.Ltmp4704:
	.loc	6 863 18 is_stmt 1
	i32.shl 
	i32.add 
.Ltmp4705:
	.loc	4 551 14
	v128.load	0:p2align=2
.Ltmp4706:
	.loc	5 2188 14
	local.tee	21
	local.get	21
	local.get	24
	v128.bitselect
.Ltmp4707:
	.loc	5 3878 14
	local.tee	21
	f32x4.div
.Ltmp4708:
	.loc	5 2188 14
	v128.const	0, 0, 128, 63, 0, 0, 128, 63, 0, 0, 128, 63, 0, 0, 128, 63
.Ltmp4709:
	.loc	5 2005 14
	local.get	21
	local.get	22
	f32x4.gt
.Ltmp4710:
	.loc	5 2188 14
	v128.bitselect
.Ltmp4711:
	.loc	4 551 14
	v128.store	0:p2align=2
.Ltmp4712:
	.loc	22 0 0 is_stmt 0
	local.get	13
	local.get	111
	i32.add 
	local.tee	9
	i32.const	2
.Ltmp4713:
	.loc	22 1130 16 is_stmt 1
	i32.shl 
	local.tee	8
	i32.const	3
.Ltmp4714:
	.loc	21 1050 16
	i32.or  
	local.get	7
	i32.ge_u
	br_if   	5
.Ltmp4715:
	.loc	21 0 16 is_stmt 0
	local.get	4
	local.get	11
	local.get	8
	i32.const	2
.Ltmp4716:
	.loc	42 89 24 is_stmt 1
	i32.shl 
	i32.add 
.Ltmp4717:
	.loc	4 551 14
	v128.load	0:p2align=2
.Ltmp4718:
	.loc	22 1206 22
	local.tee	21
	local.get	4
	v128.load	2160
	f32x4.pmin
	local.get	21
	local.get	6
	v128.select
.Ltmp4719:
	.loc	22 1211 5
	local.tee	105
	v128.store	2160
	block   	
	block   	
	local.get	6
	i32.const	1
	.loc	22 1212 20
	i32.add 
	local.tee	6
	local.get	29
	i32.eq  
.Ltmp4720:
	.loc	22 1213 22
	br_if   	0
.Ltmp4721:
	.loc	22 0 0 is_stmt 0
	local.get	13
	local.get	112
	i32.add 
	i32.const	2
.Ltmp4722:
	.loc	22 1130 16 is_stmt 1
	i32.shl 
	local.tee	9
	i32.const	3
.Ltmp4723:
	.loc	21 1050 16
	i32.or  
	local.get	7
	i32.ge_u
	br_if   	8
.Ltmp4724:
	.loc	21 0 16 is_stmt 0
	local.get	105
	local.get	11
	local.get	9
	i32.const	2
.Ltmp4725:
	.loc	42 89 24 is_stmt 1
	i32.shl 
	i32.add 
.Ltmp4726:
	.loc	4 551 14
	v128.load	0:p2align=2
.Ltmp4727:
	.loc	5 3911 9
	f32x4.pmin
	local.set	105
.Ltmp4728:
	.loc	22 1218 5
	br      	1
.LBB28_79:
	.loc	22 0 5 is_stmt 0
	end_block
	i32.const	0
	local.set	6
.Ltmp4729:
	.loc	18 900 12 is_stmt 1
	local.get	29
	i32.eqz
	br_if   	0
.Ltmp4730:
	.loc	18 0 12 is_stmt 0
	local.get	29
	local.set	8
.LBB28_81:
	loop    	
	local.get	9
	i32.const	2
.Ltmp4731:
	.loc	22 1130 16 is_stmt 1
	i32.shl 
	local.tee	10
	i32.const	3
.Ltmp4732:
	.loc	21 1050 16
	i32.or  
	local.get	7
	i32.ge_u
	br_if   	9
.Ltmp4733:
	.loc	21 0 16 is_stmt 0
	local.get	11
	local.get	10
	i32.const	2
.Ltmp4734:
	.loc	42 89 24 is_stmt 1
	i32.shl 
	i32.add 
.Ltmp4735:
	.loc	4 551 14
	local.tee	10
	local.get	10
	v128.load	0:p2align=2
.Ltmp4736:
	.loc	5 3911 9
	local.get	21
	f32x4.pmin
.Ltmp4737:
	.loc	4 551 14
	local.tee	21
	v128.store	0:p2align=2
.Ltmp4738:
	.loc	22 1224 16
	local.get	9
	local.get	12
	local.get	9
	i32.select
	i32.const	-1
	.loc	22 1227 13
	i32.add 
	local.set	9
	local.get	8
	i32.const	-1
.Ltmp4739:
	.loc	17 1916 50
	i32.add 
.Ltmp4740:
	.loc	18 900 12
	local.tee	8
	br_if   	0
.LBB28_83:
	.loc	18 900 12
	end_loop
	end_block
.Ltmp4741:
	.loc	22 0 0 is_stmt 0
	local.get	13
	local.get	110
	i32.add 
	i32.const	2
.Ltmp4742:
	.loc	22 1130 16 is_stmt 1
	i32.shl 
	local.tee	9
	i32.const	3
.Ltmp4743:
	.loc	21 1050 16
	i32.or  
	local.get	27
	i32.ge_u
	br_if   	8
.Ltmp4744:
	.loc	21 0 16 is_stmt 0
	local.get	50
	local.get	27
	i32.gt_u
.Ltmp4745:
	.loc	21 1050 16
	br_if   	9
.Ltmp4746:
	.loc	21 0 16
	local.get	28
	local.get	9
	i32.const	2
	i32.shl 
	i32.add 
	v128.load	0:p2align=2
	local.set	21
.Ltmp4747:
	.loc	42 101 24 is_stmt 1
	local.get	28
	local.get	114
	i32.add 
.Ltmp4748:
	.loc	22 0 0 is_stmt 0
	local.get	105
	v128.const	0x1p14, 0x1p14, 0x1p14, 0x1p14
	f32x4.mul
	f32x4.floor
	v128.const	0x1p-14, 0x1p-14, 0x1p-14, 0x1p-14
	f32x4.mul
.Ltmp4749:
	.loc	4 551 14 is_stmt 1
	local.tee	105
	v128.store	0:p2align=2
.Ltmp4750:
	.loc	22 1659 43
	local.get	4
	local.get	4
	v128.load	320
.Ltmp4751:
	.loc	5 3856 14
	local.tee	54
	v128.const	0x1p0, 0x1p0, 0x1p0, 0x1p0
.Ltmp4752:
	.loc	22 0 0 is_stmt 0
	local.tee	55
	local.get	105
	local.get	106
	f32x4.add
.Ltmp4753:
	local.get	21
	f32x4.sub
.Ltmp4754:
	.loc	5 3878 14 is_stmt 1
	local.tee	106
	local.get	58
	f32x4.div
.Ltmp4755:
	.loc	5 3856 14
	f32x4.sub
.Ltmp4756:
	.loc	5 3856 14 is_stmt 0
	local.tee	21
	local.get	54
	f32x4.sub
.Ltmp4757:
	.loc	5 3867 14 is_stmt 1
	local.get	102
	f32x4.mul
.Ltmp4758:
	.loc	5 3845 14
	f32x4.add
.Ltmp4759:
	.loc	5 3928 9
	local.get	21
	f32x4.pmax
.Ltmp4760:
	.loc	5 3812 14
	local.tee	21
	local.get	21
	f32x4.abs
.Ltmp4761:
	.loc	5 1991 14
	v128.const	0x1.79ca1p-67, 0x1.79ca1p-67, 0x1.79ca1p-67, 0x1.79ca1p-67
	f32x4.lt
.Ltmp4762:
	.loc	8 481 22
	v128.andnot
.Ltmp4763:
	.loc	22 1660 5
	local.tee	105
	v128.store	320
.Ltmp4764:
	.loc	22 0 0 is_stmt 0
	local.get	13
	local.get	18
	i32.add 
	i32.const	2
.Ltmp4765:
	.loc	22 1130 16 is_stmt 1
	i32.shl 
	local.tee	9
	i32.const	3
.Ltmp4766:
	.loc	21 1050 16
	i32.or  
	local.get	26
	i32.ge_u
	br_if   	10
.Ltmp4767:
	.loc	22 0 0 is_stmt 0
	local.get	103
	local.get	107
	v128.and
	local.set	103
	local.get	104
	local.get	52
	v128.and
	local.set	104
	local.get	25
	local.get	9
	i32.const	2
.Ltmp4768:
	.loc	42 89 24 is_stmt 1
	i32.shl 
	i32.add 
.Ltmp4769:
	.loc	4 551 14
	local.tee	9
	v128.load	0:p2align=2
	local.set	21
.Ltmp4770:
	.loc	4 551 14 is_stmt 0
	local.get	9
	local.get	53
	v128.store	0:p2align=2
.Ltmp4771:
	.loc	5 3856 14 is_stmt 1
	local.get	118
	local.get	21
	local.get	21
	local.get	55
	local.get	105
	f32x4.sub
.Ltmp4772:
	.loc	5 3867 14
	f32x4.mul
.Ltmp4773:
	.loc	5 2188 14
	local.get	23
	v128.bitselect
.Ltmp4774:
	.loc	4 551 14
	v128.store	0:p2align=2
	local.get	13
	i32.const	1
.Ltmp4775:
	.loc	22 0 0 is_stmt 0
	i32.add 
.Ltmp4776:
	.loc	39 304 12 is_stmt 1
	local.tee	13
	local.get	117
	i32.ne  
	br_if   	0
	end_loop
.Ltmp4777:
	.loc	22 854 9
	local.get	4
	local.get	100
	v128.store	288
	.loc	22 853 9
	local.get	4
	local.get	102
	v128.store	256
.Ltmp4778:
	.loc	22 854 9
	local.get	4
	local.get	101
	v128.store	224
	.loc	22 853 9
	local.get	4
	local.get	22
	v128.store	192
	local.get	106
	local.set	59
	local.get	103
	local.set	60
	local.get	104
	local.set	61
.Ltmp4779:
.LBB28_88:
	.loc	22 0 9 is_stmt 0
	end_block
	.loc	22 3412 39 is_stmt 1
	local.get	113
	local.get	18
	i32.add 
	local.tee	9
	i32.const	0
.Ltmp4780:
	.loc	22 1148 8
	local.get	19
	local.get	9
	local.get	19
	i32.lt_u
	i32.select
	i32.sub 
	local.set	18
.Ltmp4781:
	.loc	22 3411 39
	local.get	113
	local.get	17
	i32.add 
	local.tee	9
	i32.const	0
.Ltmp4782:
	.loc	22 1148 8
	local.get	12
	local.get	9
	local.get	12
	i32.lt_u
	i32.select
	i32.sub 
	local.set	17
.Ltmp4783:
	.loc	22 0 0 is_stmt 0
	local.get	113
	local.get	109
	i32.add 
.Ltmp4784:
	.loc	22 3360 19 is_stmt 1
	local.tee	109
	local.get	108
	i32.ge_u
	br_if   	9
	br      	0
.LBB28_89:
.Ltmp4785:
	.loc	21 1050 16
	end_loop
	end_block
	local.get	4
	local.get	59
	v128.store	336
	local.get	4
	local.get	60
	v128.store	304
	local.get	4
	local.get	61
	v128.store	240
.Ltmp4786:
	.loc	42 456 13
	local.get	114
	local.get	50
	local.get	2
	i32.const	.Lalloc_192e368852d87f0f082761c8ef71c730
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp4787:
.LBB28_90:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	59
	v128.store	336
	local.get	4
	local.get	60
	v128.store	304
	local.get	4
	local.get	61
	v128.store	240
.Ltmp4788:
	.loc	22 854 9 is_stmt 1
	local.get	4
	local.get	100
	v128.store	288
	.loc	22 853 9
	local.get	4
	local.get	102
	v128.store	256
.Ltmp4789:
	.loc	22 854 9
	local.get	4
	local.get	101
	v128.store	224
	.loc	22 853 9
	local.get	4
	local.get	22
	v128.store	192
.Ltmp4790:
	.loc	42 456 13
	local.get	51
	local.get	50
	local.get	7
	i32.const	.Lalloc_77f249f9c0b1ac1e2d67f390a6ea84d4
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp4791:
.LBB28_91:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	59
	v128.store	336
	local.get	4
	local.get	60
	v128.store	304
	local.get	4
	local.get	61
	v128.store	240
.Ltmp4792:
	.loc	22 854 9 is_stmt 1
	local.get	4
	local.get	100
	v128.store	288
	.loc	22 853 9
	local.get	4
	local.get	102
	v128.store	256
.Ltmp4793:
	.loc	22 854 9
	local.get	4
	local.get	101
	v128.store	224
	.loc	22 853 9
	local.get	4
	local.get	22
	v128.store	192
	local.get	8
	local.get	8
	i32.const	4
.Ltmp4794:
	.loc	22 1131 25
	i32.add 
.Ltmp4795:
	.loc	42 443 13
	local.get	7
	i32.const	.Lalloc_ecb3dab3c849990309e84d651d3f65d6
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp4796:
.LBB28_92:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	59
	v128.store	336
	local.get	4
	local.get	60
	v128.store	304
	local.get	4
	local.get	61
	v128.store	240
.Ltmp4797:
	.loc	22 854 9 is_stmt 1
	local.get	4
	local.get	100
	v128.store	288
	.loc	22 853 9
	local.get	4
	local.get	102
	v128.store	256
.Ltmp4798:
	.loc	22 854 9
	local.get	4
	local.get	101
	v128.store	224
	.loc	22 853 9
	local.get	4
	local.get	22
	v128.store	192
	local.get	9
	local.get	9
	i32.const	4
.Ltmp4799:
	.loc	22 1131 25
	i32.add 
.Ltmp4800:
	.loc	42 443 13
	local.get	7
	i32.const	.Lalloc_ecb3dab3c849990309e84d651d3f65d6
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp4801:
.LBB28_93:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	59
	v128.store	336
	local.get	4
	local.get	60
	v128.store	304
	local.get	4
	local.get	61
	v128.store	240
.Ltmp4802:
	.loc	22 854 9 is_stmt 1
	local.get	4
	local.get	100
	v128.store	288
	.loc	22 853 9
	local.get	4
	local.get	102
	v128.store	256
.Ltmp4803:
	.loc	22 854 9
	local.get	4
	local.get	101
	v128.store	224
	.loc	22 853 9
	local.get	4
	local.get	22
	v128.store	192
	local.get	10
	local.get	10
	i32.const	4
.Ltmp4804:
	.loc	22 1131 25
	i32.add 
.Ltmp4805:
	.loc	42 443 13
	local.get	7
	i32.const	.Lalloc_ecb3dab3c849990309e84d651d3f65d6
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp4806:
.LBB28_94:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	59
	v128.store	336
	local.get	4
	local.get	60
	v128.store	304
	local.get	4
	local.get	61
	v128.store	240
.Ltmp4807:
	.loc	22 854 9 is_stmt 1
	local.get	4
	local.get	100
	v128.store	288
	.loc	22 853 9
	local.get	4
	local.get	102
	v128.store	256
.Ltmp4808:
	.loc	22 854 9
	local.get	4
	local.get	101
	v128.store	224
	.loc	22 853 9
	local.get	4
	local.get	22
	v128.store	192
	local.get	9
	local.get	9
	i32.const	4
.Ltmp4809:
	.loc	22 1131 25
	i32.add 
.Ltmp4810:
	.loc	42 443 13
	local.get	27
	i32.const	.Lalloc_ecb3dab3c849990309e84d651d3f65d6
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp4811:
.LBB28_95:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	59
	v128.store	336
	local.get	4
	local.get	60
	v128.store	304
	local.get	4
	local.get	61
	v128.store	240
.Ltmp4812:
	.loc	22 854 9 is_stmt 1
	local.get	4
	local.get	100
	v128.store	288
	.loc	22 853 9
	local.get	4
	local.get	102
	v128.store	256
.Ltmp4813:
	.loc	22 854 9
	local.get	4
	local.get	101
	v128.store	224
	.loc	22 853 9
	local.get	4
	local.get	22
	v128.store	192
.Ltmp4814:
	.loc	42 456 13
	local.get	51
	local.get	50
	local.get	27
	i32.const	.Lalloc_77f249f9c0b1ac1e2d67f390a6ea84d4
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp4815:
.LBB28_96:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	59
	v128.store	336
	local.get	4
	local.get	60
	v128.store	304
	local.get	4
	local.get	61
	v128.store	240
.Ltmp4816:
	.loc	22 854 9 is_stmt 1
	local.get	4
	local.get	100
	v128.store	288
	.loc	22 853 9
	local.get	4
	local.get	102
	v128.store	256
.Ltmp4817:
	.loc	22 854 9
	local.get	4
	local.get	101
	v128.store	224
	.loc	22 853 9
	local.get	4
	local.get	22
	v128.store	192
	local.get	9
	local.get	9
	i32.const	4
.Ltmp4818:
	.loc	22 1131 25
	i32.add 
.Ltmp4819:
	.loc	42 443 13
	local.get	26
	i32.const	.Lalloc_ecb3dab3c849990309e84d651d3f65d6
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp4820:
.LBB28_97:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	59
	v128.store	336
	local.get	4
	local.get	60
	v128.store	304
	local.get	4
	local.get	61
	v128.store	240
.LBB28_98:
	end_block
	local.get	48
	i32.const	32
	i32.add 
	local.set	48
	local.get	44
	i32.const	512
.Ltmp4821:
	.loc	44 446 20 is_stmt 1
	i32.add 
	local.set	44
	local.get	45
	i32.const	-128
	i32.add 
	local.set	45
	local.get	46
	i32.const	128
	i32.add 
	local.set	46
	local.get	47
	i32.const	-32
	i32.add 
	local.set	47
	local.get	20
	i32.const	-1
.Ltmp4822:
	.loc	22 0 0 is_stmt 0
	i32.add 
.Ltmp4823:
	.loc	44 446 20
	local.tee	20
	i32.eqz
	br_if   	2
	br      	0
.Ltmp4824:
.LBB28_99:
	.loc	22 3192 12 is_stmt 1
	end_loop
	end_block
.Ltmp4825:
	.loc	22 3332 24
	local.get	4
	i32.const	736
	i32.add 
	local.get	15
	call	_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_
.Ltmp4826:
	.loc	22 3342 27
	local.get	0
	i32.load	804
	local.set	17
.Ltmp4827:
	.loc	22 3341 27
	local.get	0
	i32.load	800
	local.set	18
.Ltmp4828:
	.loc	22 3338 21
	local.get	0
	i32.load8_u	785
	local.set	9
.Ltmp4829:
	.loc	22 3337 19
	local.get	0
	i32.load8_u	784
	local.set	8
.Ltmp4830:
	.loc	22 3340 16
	local.get	0
	i32.load	920
	local.set	19
.Ltmp4831:
	.loc	22 3339 16
	local.get	0
	i32.load	916
	local.set	12
	local.get	4
	i32.const	1136
	i32.add 
	i32.const	0
	i32.const	1024
	memory.fill	0
.Ltmp4832:
	.loc	22 3346 32
	local.get	4
	i32.const	2160
	i32.add 
	local.get	15
	local.get	12
	local.get	19
	call	_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E3newB5_
	local.get	4
	v128.load	736
	local.set	100
.Ltmp4833:
	.loc	44 446 20
	block   	
	block   	
	local.get	3
	br_if   	0
.Ltmp4834:
	.loc	22 3416 31
	local.get	4
	i32.load	2176
	local.set	6
.Ltmp4835:
	.loc	44 446 20
	br      	1
.Ltmp4836:
.LBB28_101:
	.loc	44 0 20 is_stmt 0
	end_block
	local.get	3
	i32.const	5
.Ltmp4837:
	.loc	21 3756 21 is_stmt 1
	i32.shr_u
	local.get	3
	i32.const	31
.Ltmp4838:
	.loc	21 3757 21
	i32.and 
	i32.const	0
.Ltmp4839:
	.loc	21 3758 16
	i32.ne  
	i32.add 
	local.set	20
.Ltmp4840:
	.loc	22 3338 21
	v128.const	-1, -1, -1, -1
	local.tee	21
	v128.const	0, 0, 0, 0
	local.tee	22
	local.get	9
	i32.const	1
	i32.and 
	v128.select
	local.set	52
	local.get	21
	local.get	22
	local.get	8
	i32.const	1
.Ltmp4841:
	.loc	22 3337 19
	i32.and 
	v128.select
	local.set	43
	local.get	4
	i32.load	2208
	local.set	25
	local.get	4
	i32.load	2212
	local.set	26
	local.get	4
	v128.load	1088
	local.set	106
	local.get	4
	i32.load	2204
	local.set	27
	local.get	4
	i32.load	2200
	local.set	28
	local.get	4
	i32.load	2180
	local.set	29
	local.get	4
	i32.load	2196
	local.set	7
	local.get	4
	i32.load	2192
	local.set	11
	local.get	4
	v128.load	992
	local.set	107
	local.get	4
	v128.load	928
	local.set	102
	local.get	4
	i32.load	2188
	local.set	49
	local.get	4
	i32.load	2184
	local.set	30
	local.get	4
	v128.load	1072
	local.set	103
	local.get	4
	i32.load	2176
	local.set	6
	local.get	4
	v128.load	912
	local.set	101
	local.get	4
	v128.load	896
	local.set	37
	local.get	4
	v128.load	880
	local.set	33
	local.get	4
	v128.load	864
	local.set	34
	local.get	4
	v128.load	848
	local.set	35
	local.get	4
	v128.load	832
	local.set	22
	local.get	4
	v128.load	816
	local.set	36
	local.get	4
	v128.load	800
	local.set	38
	local.get	4
	v128.load	784
	local.set	39
	local.get	4
	v128.load	768
	local.set	40
	local.get	4
	v128.load	752
	local.set	41
	local.get	1
	local.set	44
	local.get	2
	local.set	45
	i32.const	0
	local.set	46
	local.get	3
	local.set	47
	i32.const	0
	local.set	48
.LBB28_102:
.Ltmp4842:
	.loc	17 1916 50
	loop    	
	block   	
	local.get	3
	local.get	48
	i32.eq  
.Ltmp4843:
	.loc	18 900 12
	br_if   	0
.Ltmp4844:
	.loc	18 0 12 is_stmt 0
	local.get	3
	local.get	48
	i32.sub 
	local.tee	9
	i32.const	32
	local.get	9
	i32.const	32
	i32.lt_u
	i32.select
	local.set	108
	local.get	47
	i32.const	32
	local.get	47
	i32.const	32
	i32.lt_u
	i32.select
	local.tee	31
	i32.const	1
	local.get	31
	i32.const	1
	i32.gt_u
	i32.select
	local.set	51
	local.get	0
	v128.load	768
	local.set	42
	local.get	0
	v128.load	752
	local.set	104
	local.get	0
	v128.load	736
	local.set	105
	local.get	0
	v128.load	720
	local.set	32
	local.get	0
	v128.load	704
	local.set	53
	local.get	0
	v128.load	688
	local.set	54
	local.get	0
	v128.load	672
	local.set	55
	local.get	0
	v128.load	656
	local.set	56
	local.get	0
	v128.load	640
	local.set	57
	local.get	0
	v128.load	624
	local.set	24
	local.get	0
	v128.load	608
	local.set	58
	local.get	0
	v128.load	592
	local.set	23
	local.get	0
	v128.load	576
	local.set	59
	local.get	0
	v128.load	560
	local.set	60
	local.get	0
	v128.load	544
	local.set	61
	local.get	0
	v128.load	528
	local.set	62
	local.get	0
	v128.load	512
	local.set	63
	local.get	0
	v128.load	496
	local.set	64
	local.get	0
	v128.load	480
	local.set	65
	local.get	0
	v128.load	464
	local.set	66
	local.get	0
	v128.load	448
	local.set	67
	local.get	0
	v128.load	432
	local.set	68
	local.get	0
	v128.load	416
	local.set	69
	local.get	0
	v128.load	400
	local.set	70
	local.get	0
	v128.load	384
	local.set	71
	local.get	0
	v128.load	368
	local.set	72
	local.get	0
	v128.load	352
	local.set	73
	local.get	0
	v128.load	336
	local.set	74
	local.get	0
	v128.load	320
	local.set	75
	local.get	0
	v128.load	304
	local.set	76
	local.get	0
	v128.load	288
	local.set	77
	local.get	0
	v128.load	272
	local.set	78
	local.get	0
	v128.load	256
	local.set	79
	local.get	0
	v128.load	240
	local.set	80
	local.get	0
	v128.load	224
	local.set	81
	local.get	0
	v128.load	208
	local.set	82
	local.get	0
	v128.load	192
	local.set	83
	local.get	0
	v128.load	176
	local.set	84
	local.get	0
	v128.load	160
	local.set	85
	local.get	0
	v128.load	144
	local.set	86
	local.get	0
	v128.load	128
	local.set	87
	local.get	0
	v128.load	112
	local.set	88
	local.get	0
	v128.load	96
	local.set	89
	local.get	0
	v128.load	80
	local.set	90
	local.get	0
	v128.load	64
	local.set	91
	local.get	0
	v128.load	48
	local.set	92
	local.get	0
	v128.load	32
	local.set	93
	local.get	0
	v128.load	16
	local.set	94
	local.get	4
	i32.const	1136
	i32.add 
	local.set	10
	local.get	44
	local.set	13
	local.get	45
	local.set	8
	local.get	46
	local.set	9
.LBB28_104:
	loop    	
	local.get	37
	local.set	101
	local.get	33
	local.set	37
	local.get	34
	local.set	33
	local.get	35
	local.set	34
	local.get	22
	local.set	35
	local.get	36
	local.set	22
	local.get	38
	local.set	36
	local.get	39
	local.set	38
	local.get	40
	local.set	39
	local.get	41
	local.set	40
	local.get	100
	local.set	41
.Ltmp4845:
	.loc	42 568 12 is_stmt 1
	block   	
	block   	
	local.get	9
	local.get	2
	i32.gt_u
	br_if   	0
.Ltmp4846:
	.loc	42 0 12 is_stmt 0
	local.get	8
	i32.const	3
.Ltmp4847:
	.loc	42 438 16 is_stmt 1
	i32.gt_u
	br_if   	1
	.loc	42 0 16 is_stmt 0
	i32.const	0
	i32.const	4
	.loc	42 443 13 is_stmt 1
	local.get	8
	i32.const	.Lalloc_5f4e05826d69b5e832dc96c63f325058
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp4848:
.LBB28_107:
	.loc	42 0 13 is_stmt 0
	end_block
.Ltmp4849:
	.loc	42 569 13 is_stmt 1
	local.get	9
	local.get	2
	local.get	2
	i32.const	.Lalloc_d37239ff881951c49620cb5a9c000a8e
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp4850:
.LBB28_108:
	.loc	42 0 13 is_stmt 0
	end_block
.Ltmp4851:
	.loc	4 551 14 is_stmt 1
	local.get	10
	local.get	91
	local.get	13
	v128.load	0:p2align=2
.Ltmp4852:
	.loc	5 3867 14
	local.tee	100
	f32x4.mul
.Ltmp4853:
	.loc	5 3845 14
	v128.const	0x0p0, 0x0p0, 0x0p0, 0x0p0
.Ltmp4854:
	.loc	5 3845 14 is_stmt 0
	local.tee	21
	f32x4.add
.Ltmp4855:
	.loc	5 3867 14 is_stmt 1
	local.get	87
	local.get	41
	f32x4.mul
.Ltmp4856:
	.loc	5 3845 14
	f32x4.add
.Ltmp4857:
	.loc	5 3867 14
	local.get	83
	local.get	40
	f32x4.mul
.Ltmp4858:
	.loc	5 3845 14
	f32x4.add
.Ltmp4859:
	.loc	5 3867 14
	local.get	79
	local.get	39
	f32x4.mul
.Ltmp4860:
	.loc	5 3845 14
	f32x4.add
.Ltmp4861:
	.loc	5 3867 14
	local.get	75
	local.get	38
	f32x4.mul
.Ltmp4862:
	.loc	5 3845 14
	f32x4.add
.Ltmp4863:
	.loc	5 3867 14
	local.get	71
	local.get	36
	f32x4.mul
.Ltmp4864:
	.loc	5 3845 14
	f32x4.add
.Ltmp4865:
	.loc	5 3867 14
	local.get	67
	local.get	22
	f32x4.mul
.Ltmp4866:
	.loc	5 3845 14
	f32x4.add
.Ltmp4867:
	.loc	5 3867 14
	local.get	63
	local.get	35
	f32x4.mul
.Ltmp4868:
	.loc	5 3845 14
	f32x4.add
.Ltmp4869:
	.loc	5 3867 14
	local.get	59
	local.get	34
	f32x4.mul
.Ltmp4870:
	.loc	5 3845 14
	f32x4.add
.Ltmp4871:
	.loc	5 3867 14
	local.get	57
	local.get	33
	f32x4.mul
.Ltmp4872:
	.loc	5 3845 14
	f32x4.add
.Ltmp4873:
	.loc	5 3867 14
	local.get	53
	local.get	37
	f32x4.mul
.Ltmp4874:
	.loc	5 3845 14
	f32x4.add
.Ltmp4875:
	.loc	5 3867 14
	local.get	42
	local.get	101
	f32x4.mul
.Ltmp4876:
	.loc	5 3845 14
	f32x4.add
.Ltmp4877:
	.loc	5 3812 14
	f32x4.abs
.Ltmp4878:
	.loc	5 3867 14
	local.get	92
	local.get	100
	f32x4.mul
.Ltmp4879:
	.loc	5 3845 14
	local.get	21
	f32x4.add
.Ltmp4880:
	.loc	5 3867 14
	local.get	88
	local.get	41
	f32x4.mul
.Ltmp4881:
	.loc	5 3845 14
	f32x4.add
.Ltmp4882:
	.loc	5 3867 14
	local.get	84
	local.get	40
	f32x4.mul
.Ltmp4883:
	.loc	5 3845 14
	f32x4.add
.Ltmp4884:
	.loc	5 3867 14
	local.get	80
	local.get	39
	f32x4.mul
.Ltmp4885:
	.loc	5 3845 14
	f32x4.add
.Ltmp4886:
	.loc	5 3867 14
	local.get	76
	local.get	38
	f32x4.mul
.Ltmp4887:
	.loc	5 3845 14
	f32x4.add
.Ltmp4888:
	.loc	5 3867 14
	local.get	72
	local.get	36
	f32x4.mul
.Ltmp4889:
	.loc	5 3845 14
	f32x4.add
.Ltmp4890:
	.loc	5 3867 14
	local.get	68
	local.get	22
	f32x4.mul
.Ltmp4891:
	.loc	5 3845 14
	f32x4.add
.Ltmp4892:
	.loc	5 3867 14
	local.get	64
	local.get	35
	f32x4.mul
.Ltmp4893:
	.loc	5 3845 14
	f32x4.add
.Ltmp4894:
	.loc	5 3867 14
	local.get	60
	local.get	34
	f32x4.mul
.Ltmp4895:
	.loc	5 3845 14
	f32x4.add
.Ltmp4896:
	.loc	5 3867 14
	local.get	24
	local.get	33
	f32x4.mul
.Ltmp4897:
	.loc	5 3845 14
	f32x4.add
.Ltmp4898:
	.loc	5 3867 14
	local.get	54
	local.get	37
	f32x4.mul
.Ltmp4899:
	.loc	5 3845 14
	f32x4.add
.Ltmp4900:
	.loc	5 3867 14
	local.get	104
	local.get	101
	f32x4.mul
.Ltmp4901:
	.loc	5 3845 14
	f32x4.add
.Ltmp4902:
	.loc	5 3812 14
	f32x4.abs
.Ltmp4903:
	.loc	5 3867 14
	local.get	93
	local.get	100
	f32x4.mul
.Ltmp4904:
	.loc	5 3845 14
	local.get	21
	f32x4.add
.Ltmp4905:
	.loc	5 3867 14
	local.get	89
	local.get	41
	f32x4.mul
.Ltmp4906:
	.loc	5 3845 14
	f32x4.add
.Ltmp4907:
	.loc	5 3867 14
	local.get	85
	local.get	40
	f32x4.mul
.Ltmp4908:
	.loc	5 3845 14
	f32x4.add
.Ltmp4909:
	.loc	5 3867 14
	local.get	81
	local.get	39
	f32x4.mul
.Ltmp4910:
	.loc	5 3845 14
	f32x4.add
.Ltmp4911:
	.loc	5 3867 14
	local.get	77
	local.get	38
	f32x4.mul
.Ltmp4912:
	.loc	5 3845 14
	f32x4.add
.Ltmp4913:
	.loc	5 3867 14
	local.get	73
	local.get	36
	f32x4.mul
.Ltmp4914:
	.loc	5 3845 14
	f32x4.add
.Ltmp4915:
	.loc	5 3867 14
	local.get	69
	local.get	22
	f32x4.mul
.Ltmp4916:
	.loc	5 3845 14
	f32x4.add
.Ltmp4917:
	.loc	5 3867 14
	local.get	65
	local.get	35
	f32x4.mul
.Ltmp4918:
	.loc	5 3845 14
	f32x4.add
.Ltmp4919:
	.loc	5 3867 14
	local.get	61
	local.get	34
	f32x4.mul
.Ltmp4920:
	.loc	5 3845 14
	f32x4.add
.Ltmp4921:
	.loc	5 3867 14
	local.get	58
	local.get	33
	f32x4.mul
.Ltmp4922:
	.loc	5 3845 14
	f32x4.add
.Ltmp4923:
	.loc	5 3867 14
	local.get	55
	local.get	37
	f32x4.mul
.Ltmp4924:
	.loc	5 3845 14
	f32x4.add
.Ltmp4925:
	.loc	5 3867 14
	local.get	105
	local.get	101
	f32x4.mul
.Ltmp4926:
	.loc	5 3845 14
	f32x4.add
.Ltmp4927:
	.loc	5 3812 14
	f32x4.abs
.Ltmp4928:
	.loc	5 3867 14
	local.get	94
	local.get	100
	f32x4.mul
.Ltmp4929:
	.loc	5 3845 14
	local.get	21
	f32x4.add
.Ltmp4930:
	.loc	5 3867 14
	local.get	90
	local.get	41
	f32x4.mul
.Ltmp4931:
	.loc	5 3845 14
	f32x4.add
.Ltmp4932:
	.loc	5 3867 14
	local.get	86
	local.get	40
	f32x4.mul
.Ltmp4933:
	.loc	5 3845 14
	f32x4.add
.Ltmp4934:
	.loc	5 3867 14
	local.get	82
	local.get	39
	f32x4.mul
.Ltmp4935:
	.loc	5 3845 14
	f32x4.add
.Ltmp4936:
	.loc	5 3867 14
	local.get	78
	local.get	38
	f32x4.mul
.Ltmp4937:
	.loc	5 3845 14
	f32x4.add
.Ltmp4938:
	.loc	5 3867 14
	local.get	74
	local.get	36
	f32x4.mul
.Ltmp4939:
	.loc	5 3845 14
	f32x4.add
.Ltmp4940:
	.loc	5 3867 14
	local.get	70
	local.get	22
	f32x4.mul
.Ltmp4941:
	.loc	5 3845 14
	f32x4.add
.Ltmp4942:
	.loc	5 3867 14
	local.get	66
	local.get	35
	f32x4.mul
.Ltmp4943:
	.loc	5 3845 14
	f32x4.add
.Ltmp4944:
	.loc	5 3867 14
	local.get	62
	local.get	34
	f32x4.mul
.Ltmp4945:
	.loc	5 3845 14
	f32x4.add
.Ltmp4946:
	.loc	5 3867 14
	local.get	23
	local.get	33
	f32x4.mul
.Ltmp4947:
	.loc	5 3845 14
	f32x4.add
.Ltmp4948:
	.loc	5 3867 14
	local.get	56
	local.get	37
	f32x4.mul
.Ltmp4949:
	.loc	5 3845 14
	f32x4.add
.Ltmp4950:
	.loc	5 3867 14
	local.get	32
	local.get	101
	f32x4.mul
.Ltmp4951:
	.loc	5 3845 14
	f32x4.add
.Ltmp4952:
	.loc	5 3812 14
	f32x4.abs
.Ltmp4953:
	.loc	5 3812 14 is_stmt 0
	local.get	22
	f32x4.abs
.Ltmp4954:
	.loc	5 3928 9 is_stmt 1
	f32x4.pmax
	f32x4.pmax
	f32x4.pmax
	f32x4.pmax
.Ltmp4955:
	.loc	4 551 14
	v128.store	0:p2align=2
	local.get	13
	i32.const	16
.Ltmp4956:
	.loc	17 1916 50
	i32.add 
	local.set	13
	local.get	8
	i32.const	-4
	i32.add 
	local.set	8
	local.get	9
	i32.const	4
	i32.add 
	local.set	9
	local.get	10
	i32.const	16
	i32.add 
	local.set	10
	local.get	51
	i32.const	-1
	i32.add 
.Ltmp4957:
	.loc	18 900 12
	local.tee	51
	br_if   	0
	end_loop
	i32.const	0
	local.set	109
.Ltmp4958:
.LBB28_110:
	.loc	22 1575 16
	block   	
	block   	
	block   	
	block   	
	block   	
	block   	
	block   	
	block   	
	loop    	
	local.get	0
	i32.load	916
.Ltmp4959:
	.loc	22 1580 33
	local.tee	9
	local.get	49
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp4960:
	.loc	22 1148 8
	local.get	9
	local.get	8
	local.get	9
	i32.lt_u
	i32.select
	i32.sub 
.Ltmp4961:
	.loc	22 1588 14
	local.tee	110
	i32.sub 
.Ltmp4962:
	.loc	22 1578 28
	local.tee	8
	local.get	9
	local.get	30
	local.get	17
	i32.add 
	local.tee	10
	i32.const	0
.Ltmp4963:
	.loc	22 1148 8
	local.get	9
	local.get	10
	local.get	9
	i32.lt_u
	i32.select
	i32.sub 
.Ltmp4964:
	.loc	22 1586 14
	local.tee	111
	i32.sub 
	local.tee	10
	local.get	9
	local.get	17
	i32.const	1
.Ltmp4965:
	.loc	22 1577 25
	i32.add 
	local.tee	13
	i32.const	0
.Ltmp4966:
	.loc	22 1148 8
	local.get	9
	local.get	13
	local.get	9
	i32.lt_u
	i32.select
	i32.sub 
.Ltmp4967:
	.loc	22 1585 14
	local.tee	112
	i32.sub 
.Ltmp4968:
	.loc	22 1576 16
	local.tee	13
	local.get	0
	i32.load	920
.Ltmp4969:
	.loc	22 1584 14
	local.get	18
	i32.sub 
	.loc	22 1583 14
	local.tee	51
	local.get	9
	local.get	17
	i32.sub 
.Ltmp4970:
	.loc	22 3368 21
	local.tee	9
	local.get	108
	local.get	109
	i32.sub 
.Ltmp4971:
	.loc	17 1077 12
	local.tee	50
	local.get	9
	local.get	50
	i32.lt_u
	i32.select
.Ltmp4972:
	.loc	17 1077 12 is_stmt 0
	local.tee	50
	local.get	51
	local.get	50
	i32.lt_u
	i32.select
.Ltmp4973:
	.loc	17 1077 12
	local.tee	50
	local.get	13
	local.get	50
	i32.lt_u
	i32.select
.Ltmp4974:
	.loc	17 1077 12
	local.tee	50
	local.get	10
	local.get	50
	i32.lt_u
	i32.select
.Ltmp4975:
	.loc	17 1077 12
	local.tee	50
	local.get	8
	local.get	50
	i32.lt_u
	i32.select
.Ltmp4976:
	.loc	22 3374 28 is_stmt 1
	local.tee	113
	local.get	109
	local.get	48
	i32.add 
.Ltmp4977:
	.loc	22 3376 55
	local.tee	114
	i32.add 
	i32.const	2
	i32.shl 
	local.tee	50
	local.get	114
	i32.const	2
.Ltmp4978:
	.loc	22 3374 28
	i32.shl 
.Ltmp4979:
	.loc	21 1050 16
	local.tee	114
	i32.lt_u
	br_if   	1
	local.get	50
	local.get	2
	i32.gt_u
	br_if   	1
.Ltmp4980:
	.loc	39 304 12
	block   	
	local.get	113
	i32.eqz
	br_if   	0
.Ltmp4981:
	.loc	39 0 12 is_stmt 0
	local.get	1
	local.get	114
	i32.const	2
	i32.shl 
	i32.add 
	local.set	115
	local.get	4
	i32.const	1136
	i32.add 
	local.get	109
	i32.const	4
	i32.shl 
	i32.add 
	local.set	116
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
	local.get	51
	local.get	9
	local.get	51
	i32.lt_u
	i32.select
	local.tee	9
	local.get	31
	local.get	109
	i32.sub 
	local.tee	8
	local.get	9
	local.get	8
	i32.lt_u
	i32.select
	i32.const	1073741823
	i32.and 
	local.set	117
	i32.const	0
	local.set	13
.LBB28_114:
.Ltmp4982:
	.loc	22 1502 26 is_stmt 1
	loop    	
	local.get	13
	local.get	17
	i32.add 
	i32.const	2
.Ltmp4983:
	.loc	22 1137 16
	i32.shl 
	local.tee	51
	i32.const	4
.Ltmp4984:
	.loc	22 1138 33
	i32.add 
	local.set	50
	local.get	51
	i32.const	3
.Ltmp4985:
	.loc	21 1050 16
	i32.or  
	local.get	7
	i32.ge_u
	br_if   	4
.Ltmp4986:
	.loc	21 0 16 is_stmt 0
	local.get	115
	local.get	13
	i32.const	4
	i32.shl 
	local.tee	9
	i32.add 
	local.tee	118
	v128.load	0:p2align=2
	local.set	104
	local.get	11
	local.get	51
	i32.const	2
.Ltmp4987:
	.loc	42 101 24 is_stmt 1
	i32.shl 
	local.tee	114
	i32.add 
.Ltmp4988:
	.loc	22 0 0 is_stmt 0
	local.get	102
	local.get	116
	local.get	9
	i32.add 
.Ltmp4989:
	v128.load	0:p2align=2
.Ltmp4990:
	.loc	5 2188 14 is_stmt 1
	local.tee	21
	local.get	21
	local.get	43
	v128.bitselect
.Ltmp4991:
	.loc	5 3878 14
	local.tee	21
	f32x4.div
.Ltmp4992:
	.loc	5 2188 14
	v128.const	0, 0, 128, 63, 0, 0, 128, 63, 0, 0, 128, 63, 0, 0, 128, 63
.Ltmp4993:
	.loc	5 2005 14
	local.get	102
	local.get	21
	f32x4.lt
.Ltmp4994:
	.loc	5 2188 14
	v128.bitselect
.Ltmp4995:
	.loc	4 551 14
	v128.store	0:p2align=2
.Ltmp4996:
	.loc	22 0 0 is_stmt 0
	local.get	13
	local.get	111
	i32.add 
	local.tee	9
	i32.const	2
.Ltmp4997:
	.loc	22 1130 16 is_stmt 1
	i32.shl 
	local.tee	8
	i32.const	3
.Ltmp4998:
	.loc	21 1050 16
	i32.or  
	local.get	7
	i32.ge_u
	br_if   	5
.Ltmp4999:
	.loc	21 0 16 is_stmt 0
	local.get	4
	local.get	11
	local.get	8
	i32.const	2
.Ltmp5000:
	.loc	42 89 24 is_stmt 1
	i32.shl 
	i32.add 
.Ltmp5001:
	.loc	4 551 14
	v128.load	0:p2align=2
.Ltmp5002:
	.loc	22 1206 22
	local.tee	21
	local.get	4
	v128.load	2160
	f32x4.pmin
	local.get	21
	local.get	6
	v128.select
.Ltmp5003:
	.loc	22 1211 5
	local.tee	42
	v128.store	2160
	block   	
	block   	
	local.get	6
	i32.const	1
	.loc	22 1212 20
	i32.add 
	local.tee	6
	local.get	29
	i32.eq  
.Ltmp5004:
	.loc	22 1213 22
	br_if   	0
.Ltmp5005:
	.loc	22 0 0 is_stmt 0
	local.get	13
	local.get	112
	i32.add 
	i32.const	2
.Ltmp5006:
	.loc	22 1130 16 is_stmt 1
	i32.shl 
	local.tee	9
	i32.const	3
.Ltmp5007:
	.loc	21 1050 16
	i32.or  
	local.get	7
	i32.ge_u
	br_if   	8
.Ltmp5008:
	.loc	21 0 16 is_stmt 0
	local.get	42
	local.get	11
	local.get	9
	i32.const	2
.Ltmp5009:
	.loc	42 89 24 is_stmt 1
	i32.shl 
	i32.add 
.Ltmp5010:
	.loc	4 551 14
	v128.load	0:p2align=2
.Ltmp5011:
	.loc	5 3911 9
	f32x4.pmin
	local.set	42
.Ltmp5012:
	.loc	22 1218 5
	br      	1
.LBB28_119:
	.loc	22 0 5 is_stmt 0
	end_block
	i32.const	0
	local.set	6
.Ltmp5013:
	.loc	18 900 12 is_stmt 1
	local.get	29
	i32.eqz
	br_if   	0
.Ltmp5014:
	.loc	18 0 12 is_stmt 0
	local.get	29
	local.set	8
.LBB28_121:
	loop    	
	local.get	9
	i32.const	2
.Ltmp5015:
	.loc	22 1130 16 is_stmt 1
	i32.shl 
	local.tee	10
	i32.const	3
.Ltmp5016:
	.loc	21 1050 16
	i32.or  
	local.get	7
	i32.ge_u
	br_if   	9
.Ltmp5017:
	.loc	21 0 16 is_stmt 0
	local.get	11
	local.get	10
	i32.const	2
.Ltmp5018:
	.loc	42 89 24 is_stmt 1
	i32.shl 
	i32.add 
.Ltmp5019:
	.loc	4 551 14
	local.tee	10
	local.get	10
	v128.load	0:p2align=2
.Ltmp5020:
	.loc	5 3911 9
	local.get	21
	f32x4.pmin
.Ltmp5021:
	.loc	4 551 14
	local.tee	21
	v128.store	0:p2align=2
.Ltmp5022:
	.loc	22 1224 16
	local.get	9
	local.get	12
	local.get	9
	i32.select
	i32.const	-1
	.loc	22 1227 13
	i32.add 
	local.set	9
	local.get	8
	i32.const	-1
.Ltmp5023:
	.loc	17 1916 50
	i32.add 
.Ltmp5024:
	.loc	18 900 12
	local.tee	8
	br_if   	0
.LBB28_123:
	.loc	18 900 12
	end_loop
	end_block
.Ltmp5025:
	.loc	22 0 0 is_stmt 0
	local.get	13
	local.get	110
	i32.add 
	i32.const	2
.Ltmp5026:
	.loc	22 1130 16 is_stmt 1
	i32.shl 
	local.tee	9
	i32.const	3
.Ltmp5027:
	.loc	21 1050 16
	i32.or  
	local.get	27
	i32.ge_u
	br_if   	8
.Ltmp5028:
	.loc	21 0 16 is_stmt 0
	local.get	50
	local.get	27
	i32.gt_u
.Ltmp5029:
	.loc	21 1050 16
	br_if   	9
.Ltmp5030:
	.loc	21 0 16
	local.get	28
	local.get	9
	i32.const	2
	i32.shl 
	i32.add 
	v128.load	0:p2align=2
	local.set	21
.Ltmp5031:
	.loc	42 101 24 is_stmt 1
	local.get	28
	local.get	114
	i32.add 
.Ltmp5032:
	.loc	22 0 0 is_stmt 0
	local.get	42
	v128.const	0x1p14, 0x1p14, 0x1p14, 0x1p14
	f32x4.mul
	f32x4.floor
	v128.const	0x1p-14, 0x1p-14, 0x1p-14, 0x1p-14
	f32x4.mul
.Ltmp5033:
	.loc	4 551 14 is_stmt 1
	local.tee	42
	v128.store	0:p2align=2
.Ltmp5034:
	.loc	22 1659 43
	local.get	4
	local.get	4
	v128.load	1056
.Ltmp5035:
	.loc	5 3856 14
	local.tee	105
	local.get	107
	v128.const	0x1p0, 0x1p0, 0x1p0, 0x1p0
.Ltmp5036:
	.loc	22 0 0 is_stmt 0
	local.tee	32
	local.get	42
	local.get	103
	f32x4.add
.Ltmp5037:
	local.get	21
	f32x4.sub
.Ltmp5038:
	.loc	5 3878 14 is_stmt 1
	local.tee	103
	local.get	106
	f32x4.div
.Ltmp5039:
	.loc	5 3856 14
	f32x4.sub
.Ltmp5040:
	.loc	5 3856 14 is_stmt 0
	local.tee	21
	local.get	105
	f32x4.sub
.Ltmp5041:
	.loc	5 3867 14 is_stmt 1
	f32x4.mul
.Ltmp5042:
	.loc	5 3845 14
	f32x4.add
.Ltmp5043:
	.loc	5 3928 9
	local.get	21
	f32x4.pmax
.Ltmp5044:
	.loc	5 3812 14
	local.tee	21
	local.get	21
	f32x4.abs
.Ltmp5045:
	.loc	5 1991 14
	v128.const	0x1.79ca1p-67, 0x1.79ca1p-67, 0x1.79ca1p-67, 0x1.79ca1p-67
	f32x4.lt
.Ltmp5046:
	.loc	8 481 22
	v128.andnot
.Ltmp5047:
	.loc	22 1660 5
	local.tee	42
	v128.store	1056
.Ltmp5048:
	.loc	22 0 0 is_stmt 0
	local.get	13
	local.get	18
	i32.add 
	i32.const	2
.Ltmp5049:
	.loc	22 1130 16 is_stmt 1
	i32.shl 
	local.tee	9
	i32.const	3
.Ltmp5050:
	.loc	21 1050 16
	i32.or  
	local.get	26
	i32.ge_u
	br_if   	10
.Ltmp5051:
	.loc	21 0 16 is_stmt 0
	local.get	25
	local.get	9
	i32.const	2
.Ltmp5052:
	.loc	42 89 24 is_stmt 1
	i32.shl 
	i32.add 
.Ltmp5053:
	.loc	4 551 14
	local.tee	9
	v128.load	0:p2align=2
	local.set	21
.Ltmp5054:
	.loc	4 551 14 is_stmt 0
	local.get	9
	local.get	104
	v128.store	0:p2align=2
.Ltmp5055:
	.loc	5 3856 14 is_stmt 1
	local.get	118
	local.get	21
	local.get	21
	local.get	32
	local.get	42
	f32x4.sub
.Ltmp5056:
	.loc	5 3867 14
	f32x4.mul
.Ltmp5057:
	.loc	5 2188 14
	local.get	52
	v128.bitselect
.Ltmp5058:
	.loc	4 551 14
	v128.store	0:p2align=2
	local.get	13
	i32.const	1
.Ltmp5059:
	.loc	22 0 0 is_stmt 0
	i32.add 
.Ltmp5060:
	.loc	39 304 12 is_stmt 1
	local.tee	13
	local.get	117
	i32.ne  
	br_if   	0
.LBB28_127:
	end_loop
	end_block
.Ltmp5061:
	.loc	22 3412 39
	local.get	113
	local.get	18
	i32.add 
	local.tee	9
	i32.const	0
.Ltmp5062:
	.loc	22 1148 8
	local.get	19
	local.get	9
	local.get	19
	i32.lt_u
	i32.select
	i32.sub 
	local.set	18
.Ltmp5063:
	.loc	22 3411 39
	local.get	113
	local.get	17
	i32.add 
	local.tee	9
	i32.const	0
.Ltmp5064:
	.loc	22 1148 8
	local.get	12
	local.get	9
	local.get	12
	i32.lt_u
	i32.select
	i32.sub 
	local.set	17
.Ltmp5065:
	.loc	22 0 0 is_stmt 0
	local.get	113
	local.get	109
	i32.add 
.Ltmp5066:
	.loc	22 3360 19 is_stmt 1
	local.tee	109
	local.get	108
	i32.ge_u
	br_if   	9
	br      	0
.LBB28_128:
.Ltmp5067:
	.loc	21 1050 16
	end_loop
	end_block
.Ltmp5068:
	.loc	42 456 13
	local.get	114
	local.get	50
	local.get	2
	i32.const	.Lalloc_192e368852d87f0f082761c8ef71c730
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5069:
.LBB28_129:
	.loc	42 0 13 is_stmt 0
	end_block
.Ltmp5070:
	.loc	42 456 13 is_stmt 1
	local.get	51
	local.get	50
	local.get	7
	i32.const	.Lalloc_77f249f9c0b1ac1e2d67f390a6ea84d4
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5071:
.LBB28_130:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	8
	local.get	8
	i32.const	4
.Ltmp5072:
	.loc	22 1131 25 is_stmt 1
	i32.add 
.Ltmp5073:
	.loc	42 443 13
	local.get	7
	i32.const	.Lalloc_ecb3dab3c849990309e84d651d3f65d6
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5074:
.LBB28_131:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	9
	local.get	9
	i32.const	4
.Ltmp5075:
	.loc	22 1131 25 is_stmt 1
	i32.add 
.Ltmp5076:
	.loc	42 443 13
	local.get	7
	i32.const	.Lalloc_ecb3dab3c849990309e84d651d3f65d6
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5077:
.LBB28_132:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	10
	local.get	10
	i32.const	4
.Ltmp5078:
	.loc	22 1131 25 is_stmt 1
	i32.add 
.Ltmp5079:
	.loc	42 443 13
	local.get	7
	i32.const	.Lalloc_ecb3dab3c849990309e84d651d3f65d6
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5080:
.LBB28_133:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	9
	local.get	9
	i32.const	4
.Ltmp5081:
	.loc	22 1131 25 is_stmt 1
	i32.add 
.Ltmp5082:
	.loc	42 443 13
	local.get	27
	i32.const	.Lalloc_ecb3dab3c849990309e84d651d3f65d6
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5083:
.LBB28_134:
	.loc	42 0 13 is_stmt 0
	end_block
.Ltmp5084:
	.loc	42 456 13 is_stmt 1
	local.get	51
	local.get	50
	local.get	27
	i32.const	.Lalloc_77f249f9c0b1ac1e2d67f390a6ea84d4
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5085:
.LBB28_135:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	9
	local.get	9
	i32.const	4
.Ltmp5086:
	.loc	22 1131 25 is_stmt 1
	i32.add 
.Ltmp5087:
	.loc	42 443 13
	local.get	26
	i32.const	.Lalloc_ecb3dab3c849990309e84d651d3f65d6
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5088:
.LBB28_136:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	48
	i32.const	32
.Ltmp5089:
	i32.add 
	local.set	48
	local.get	44
	i32.const	512
	.loc	44 446 20 is_stmt 1
	i32.add 
	local.set	44
	local.get	45
	i32.const	-128
	i32.add 
	local.set	45
	local.get	46
	i32.const	128
	i32.add 
	local.set	46
	local.get	47
	i32.const	-32
	i32.add 
	local.set	47
	local.get	20
	i32.const	-1
	.loc	44 0 0 is_stmt 0
	i32.add 
	.loc	44 446 20
	local.tee	20
	br_if   	0
	end_loop
	local.get	4
	local.get	103
	v128.store	1072
.Ltmp5090:
	.loc	22 0 0
	local.get	4
	local.get	101
	v128.store	912
	local.get	4
	local.get	37
	v128.store	896
	local.get	4
	local.get	33
	v128.store	880
	local.get	4
	local.get	34
	v128.store	864
	local.get	4
	local.get	35
	v128.store	848
	local.get	4
	local.get	22
	v128.store	832
	local.get	4
	local.get	36
	v128.store	816
	local.get	4
	local.get	38
	v128.store	800
	local.get	4
	local.get	39
	v128.store	784
	local.get	4
	local.get	40
	v128.store	768
	local.get	4
	local.get	41
	v128.store	752
.LBB28_138:
	end_block
	local.get	4
	local.get	100
	v128.store	736
.Ltmp5091:
	.loc	22 3419 23 is_stmt 1
	block   	
	local.get	0
	i32.load	968
	local.tee	9
	i32.const	3
.Ltmp5092:
	.loc	42 451 16
	i32.le_u
	br_if   	0
.Ltmp5093:
	.loc	22 3419 23
	local.get	0
	i32.load	964
.Ltmp5094:
	.loc	22 0 0 is_stmt 0
	local.get	4
	v128.load	2160
.Ltmp5095:
	.loc	4 551 14 is_stmt 1
	v128.store	0:p2align=2
.Ltmp5096:
	.loc	22 3420 5
	block   	
	local.get	0
	i32.load	984
.Ltmp5097:
	.loc	23 180 28
	local.tee	9
	i32.eqz
	br_if   	0
.Ltmp5098:
	.loc	23 0 28 is_stmt 0
	local.get	9
	i32.const	2
	i32.shl 
	local.set	8
	local.get	0
	i32.load	980
	local.set	9
.LBB28_141:
.Ltmp5099:
	.loc	38 66 21 is_stmt 1
	loop    	
	local.get	9
	local.get	6
	i32.store	0
	local.get	9
	i32.const	4
.Ltmp5100:
	.loc	24 656 28
	i32.add 
	local.set	9
	local.get	8
	i32.const	-4
.Ltmp5101:
	.loc	24 1714 9
	i32.add 
.Ltmp5102:
	.loc	23 180 28
	local.tee	8
	br_if   	0
.LBB28_142:
	end_loop
	end_block
.Ltmp5103:
	.loc	22 3422 5
	local.get	4
	i32.const	2160
	i32.add 
	local.get	4
	i32.const	736
	i32.add 
	i32.const	368
	memory.copy	0, 0
	.loc	22 3422 14 is_stmt 0
	local.get	4
	i32.const	2160
	i32.add 
	local.get	15
	call	_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_
	.loc	22 3424 5 is_stmt 1
	local.get	0
	local.get	17
	i32.store	804
	.loc	22 3423 5
	local.get	0
	local.get	18
	i32.store	800
	br      	4
.LBB28_143:
	.loc	22 0 5 is_stmt 0
	end_block
	i32.const	0
	i32.const	4
.Ltmp5104:
	.loc	42 456 13 is_stmt 1
	local.get	9
	i32.const	.Lalloc_5f4e05826d69b5e832dc96c63f325058
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5105:
.LBB28_144:
	.loc	42 0 13 is_stmt 0
	end_block
.Ltmp5106:
	.loc	22 3419 23 is_stmt 1
	block   	
	local.get	0
	i32.load	968
	local.tee	9
	i32.const	3
.Ltmp5107:
	.loc	42 451 16
	i32.le_u
	br_if   	0
.Ltmp5108:
	.loc	22 3419 23
	local.get	0
	i32.load	964
.Ltmp5109:
	.loc	22 0 0 is_stmt 0
	local.get	4
	v128.load	2160
.Ltmp5110:
	.loc	4 551 14 is_stmt 1
	v128.store	0:p2align=2
.Ltmp5111:
	.loc	22 3420 5
	block   	
	local.get	0
	i32.load	984
	local.tee	9
	i32.eqz
	br_if   	0
	.loc	22 0 5 is_stmt 0
	local.get	9
	i32.const	2
	i32.shl 
	local.set	8
	local.get	0
	i32.load	980
	local.set	9
.LBB28_147:
.Ltmp5112:
	.loc	38 66 21 is_stmt 1
	loop    	
	local.get	9
	local.get	6
	i32.store	0
	local.get	9
	i32.const	4
.Ltmp5113:
	.loc	24 656 28
	i32.add 
	local.set	9
	local.get	8
	i32.const	-4
.Ltmp5114:
	.loc	24 1714 9
	i32.add 
	local.tee	8
	br_if   	0
.Ltmp5115:
.LBB28_148:
	.loc	24 0 9 is_stmt 0
	end_loop
	end_block
	.loc	22 3422 14 is_stmt 1
	local.get	4
	local.get	15
	call	_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_
	.loc	22 3424 5
	local.get	0
	local.get	17
	i32.store	804
	.loc	22 3423 5
	local.get	0
	local.get	18
	i32.store	800
.Ltmp5116:
	.loc	22 3197 13
	br      	3
.LBB28_149:
	.loc	22 0 13 is_stmt 0
	end_block
	i32.const	0
	i32.const	4
.Ltmp5117:
	.loc	42 456 13 is_stmt 1
	local.get	9
	i32.const	.Lalloc_5f4e05826d69b5e832dc96c63f325058
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5118:
.LBB28_150:
	.loc	42 0 13 is_stmt 0
	end_block
	.loc	22 3202 12 is_stmt 1
	local.get	8
	br_if   	0
.Ltmp5119:
	.loc	22 3252 24
	local.get	4
	i32.const	368
	i32.add 
	local.get	15
	call	_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_
.Ltmp5120:
	.loc	22 3260 27
	local.get	0
	i32.load	804
	local.set	17
.Ltmp5121:
	.loc	22 3259 27
	local.get	0
	i32.load	800
	local.set	109
.Ltmp5122:
	.loc	22 3258 21
	local.get	0
	i32.load8_u	785
	local.set	9
.Ltmp5123:
	.loc	22 3257 19
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
	local.set	42
	block   	
	block   	
	block   	
	block   	
	local.get	3
	i32.eqz
	br_if   	0
	.loc	22 0 19 is_stmt 0
	local.get	3
	i32.const	5
.Ltmp5124:
	.loc	21 3756 21 is_stmt 1
	i32.shr_u
	local.get	3
	i32.const	31
.Ltmp5125:
	.loc	21 3757 21
	i32.and 
	i32.const	0
.Ltmp5126:
	.loc	21 3758 16
	i32.ne  
	i32.add 
	local.set	45
.Ltmp5127:
	.loc	22 3258 21
	v128.const	-1, -1, -1, -1
	local.tee	21
	v128.const	0, 0, 0, 0
	local.tee	22
	local.get	9
	i32.const	1
	i32.and 
	v128.select
	local.set	119
	local.get	21
	local.get	22
	local.get	8
	i32.const	1
.Ltmp5128:
	.loc	22 3257 19
	i32.and 
	v128.select
	local.set	99
	local.get	4
	v128.load	704
	local.set	22
.Ltmp5129:
	.loc	22 1759 23
	local.get	4
	v128.load	544
	local.set	120
	local.get	4
	v128.load	528
	local.set	121
	local.get	4
	v128.load	512
	local.set	122
	local.get	4
	v128.load	496
	local.set	123
	local.get	4
	v128.load	480
	local.set	124
	local.get	4
	v128.load	464
	local.set	125
	local.get	4
	v128.load	448
	local.set	126
	local.get	4
	v128.load	432
	local.set	127
	local.get	4
	v128.load	416
	local.set	128
	local.get	4
	v128.load	400
	local.set	129
	local.get	4
	v128.load	384
	local.set	130
	local.get	4
	v128.load	720
	local.set	131
	local.get	4
	v128.load	624
	local.set	132
	local.get	4
	v128.load	560
	local.set	52
	local.get	1
	local.set	30
	local.get	2
	local.set	31
	i32.const	0
	local.set	44
	local.get	3
	local.set	49
	i32.const	0
	local.set	19
.Ltmp5130:
.LBB28_153:
	.loc	22 0 23 is_stmt 0
	block   	
	block   	
	loop    	
	local.get	22
	local.set	133
	local.get	49
	i32.const	1
	local.get	49
	i32.const	1
.Ltmp5131:
	.loc	21 2584 13 is_stmt 1
	i32.gt_u
	i32.select
	local.tee	9
	i32.const	32
	local.get	9
	i32.const	32
	i32.lt_u
	i32.select
	local.set	108
.Ltmp5132:
	.loc	17 1916 50
	block   	
	block   	
	local.get	3
	local.get	19
	i32.eq  
.Ltmp5133:
	.loc	18 900 12
	local.tee	12
	i32.eqz
	br_if   	0
	.loc	18 0 12 is_stmt 0
	local.get	130
	local.set	22
	local.get	129
	local.set	37
	local.get	128
	local.set	33
	local.get	127
	local.set	34
	local.get	126
	local.set	35
	local.get	125
	local.set	21
	local.get	124
	local.set	36
	local.get	123
	local.set	38
	local.get	122
	local.set	39
	local.get	121
	local.set	40
	local.get	120
	local.set	41
	.loc	18 900 12
	br      	1
.Ltmp5134:
.LBB28_155:
	.loc	18 0 12
	end_block
	local.get	0
	v128.load	768
	local.set	53
	local.get	0
	v128.load	752
	local.set	54
	local.get	0
	v128.load	736
	local.set	55
	local.get	0
	v128.load	720
	local.set	56
	local.get	0
	v128.load	704
	local.set	57
	local.get	0
	v128.load	688
	local.set	24
	local.get	0
	v128.load	672
	local.set	58
	local.get	0
	v128.load	656
	local.set	23
	local.get	0
	v128.load	640
	local.set	59
	local.get	0
	v128.load	624
	local.set	60
	local.get	0
	v128.load	608
	local.set	61
	local.get	0
	v128.load	592
	local.set	62
	local.get	0
	v128.load	576
	local.set	63
	local.get	0
	v128.load	560
	local.set	64
	local.get	0
	v128.load	544
	local.set	65
	local.get	0
	v128.load	528
	local.set	66
	local.get	0
	v128.load	512
	local.set	67
	local.get	0
	v128.load	496
	local.set	68
	local.get	0
	v128.load	480
	local.set	69
	local.get	0
	v128.load	464
	local.set	70
	local.get	0
	v128.load	448
	local.set	71
	local.get	0
	v128.load	432
	local.set	72
	local.get	0
	v128.load	416
	local.set	73
	local.get	0
	v128.load	400
	local.set	74
	local.get	0
	v128.load	384
	local.set	75
	local.get	0
	v128.load	368
	local.set	76
	local.get	0
	v128.load	352
	local.set	77
	local.get	0
	v128.load	336
	local.set	78
	local.get	0
	v128.load	320
	local.set	79
	local.get	0
	v128.load	304
	local.set	80
	local.get	0
	v128.load	288
	local.set	81
	local.get	0
	v128.load	272
	local.set	82
	local.get	0
	v128.load	256
	local.set	83
	local.get	0
	v128.load	240
	local.set	84
	local.get	0
	v128.load	224
	local.set	85
	local.get	0
	v128.load	208
	local.set	86
	local.get	0
	v128.load	192
	local.set	87
	local.get	0
	v128.load	176
	local.set	88
	local.get	0
	v128.load	160
	local.set	89
	local.get	0
	v128.load	144
	local.set	90
	local.get	0
	v128.load	128
	local.set	91
	local.get	0
	v128.load	112
	local.set	92
	local.get	0
	v128.load	96
	local.set	93
	local.get	0
	v128.load	80
	local.set	94
	local.get	0
	v128.load	64
	local.set	95
	local.get	0
	v128.load	48
	local.set	96
	local.get	0
	v128.load	32
	local.set	97
	local.get	0
	v128.load	16
	local.set	98
	local.get	4
	i32.const	1136
	i32.add 
	local.set	10
	local.get	30
	local.set	7
	local.get	31
	local.set	8
	local.get	44
	local.set	9
	local.get	108
	local.set	11
	local.get	121
	local.set	100
	local.get	122
	local.set	101
	local.get	123
	local.set	102
	local.get	124
	local.set	103
	local.get	125
	local.set	104
	local.get	126
	local.set	105
	local.get	127
	local.set	32
	local.get	128
	local.set	43
	local.get	129
	local.set	106
	local.get	130
	local.set	107
.LBB28_156:
	loop    	
	local.get	42
	local.set	22
	local.get	107
	local.set	37
	local.get	106
	local.set	33
	local.get	43
	local.set	34
	local.get	32
	local.set	35
	local.get	105
	local.set	21
	local.get	104
	local.set	36
	local.get	103
	local.set	38
	local.get	102
	local.set	39
	local.get	101
	local.set	40
	local.get	100
	local.set	41
.Ltmp5135:
	.loc	42 568 12 is_stmt 1
	block   	
	block   	
	local.get	9
	local.get	2
	i32.gt_u
	br_if   	0
.Ltmp5136:
	.loc	42 0 12 is_stmt 0
	local.get	8
	i32.const	3
.Ltmp5137:
	.loc	42 438 16 is_stmt 1
	i32.gt_u
	br_if   	1
.Ltmp5138:
	.loc	42 0 16 is_stmt 0
	local.get	4
	local.get	133
	v128.store	704
	.loc	22 1765 5 is_stmt 1
	local.get	4
	local.get	120
	v128.store	544
	local.get	4
	local.get	121
	v128.store	528
	local.get	4
	local.get	122
	v128.store	512
	local.get	4
	local.get	123
	v128.store	496
	local.get	4
	local.get	124
	v128.store	480
	local.get	4
	local.get	125
	v128.store	464
	local.get	4
	local.get	126
	v128.store	448
	local.get	4
	local.get	127
	v128.store	432
	local.get	4
	local.get	128
	v128.store	416
	local.get	4
	local.get	129
	v128.store	400
	local.get	4
	local.get	130
	v128.store	384
	i32.const	0
	i32.const	4
.Ltmp5139:
	.loc	42 443 13
	local.get	8
	i32.const	.Lalloc_5f4e05826d69b5e832dc96c63f325058
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5140:
.LBB28_159:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	133
	v128.store	704
	.loc	22 1765 5 is_stmt 1
	local.get	4
	local.get	120
	v128.store	544
	local.get	4
	local.get	121
	v128.store	528
	local.get	4
	local.get	122
	v128.store	512
	local.get	4
	local.get	123
	v128.store	496
	local.get	4
	local.get	124
	v128.store	480
	local.get	4
	local.get	125
	v128.store	464
	local.get	4
	local.get	126
	v128.store	448
	local.get	4
	local.get	127
	v128.store	432
	local.get	4
	local.get	128
	v128.store	416
	local.get	4
	local.get	129
	v128.store	400
	local.get	4
	local.get	130
	v128.store	384
.Ltmp5141:
	.loc	42 569 13
	local.get	9
	local.get	2
	local.get	2
	i32.const	.Lalloc_d37239ff881951c49620cb5a9c000a8e
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5142:
.LBB28_160:
	.loc	42 0 13 is_stmt 0
	end_block
.Ltmp5143:
	.loc	4 551 14 is_stmt 1
	local.get	10
	local.get	95
	local.get	7
	v128.load	0:p2align=2
.Ltmp5144:
	.loc	5 3867 14
	local.tee	42
	f32x4.mul
.Ltmp5145:
	.loc	5 3845 14
	v128.const	0x0p0, 0x0p0, 0x0p0, 0x0p0
.Ltmp5146:
	.loc	5 3845 14 is_stmt 0
	local.tee	100
	f32x4.add
.Ltmp5147:
	.loc	5 3867 14 is_stmt 1
	local.get	91
	local.get	22
	f32x4.mul
.Ltmp5148:
	.loc	5 3845 14
	f32x4.add
.Ltmp5149:
	.loc	5 3867 14
	local.get	87
	local.get	37
	f32x4.mul
.Ltmp5150:
	.loc	5 3845 14
	f32x4.add
.Ltmp5151:
	.loc	5 3867 14
	local.get	83
	local.get	33
	f32x4.mul
.Ltmp5152:
	.loc	5 3845 14
	f32x4.add
.Ltmp5153:
	.loc	5 3867 14
	local.get	79
	local.get	34
	f32x4.mul
.Ltmp5154:
	.loc	5 3845 14
	f32x4.add
.Ltmp5155:
	.loc	5 3867 14
	local.get	75
	local.get	35
	f32x4.mul
.Ltmp5156:
	.loc	5 3845 14
	f32x4.add
.Ltmp5157:
	.loc	5 3867 14
	local.get	71
	local.get	21
	f32x4.mul
.Ltmp5158:
	.loc	5 3845 14
	f32x4.add
.Ltmp5159:
	.loc	5 3867 14
	local.get	67
	local.get	36
	f32x4.mul
.Ltmp5160:
	.loc	5 3845 14
	f32x4.add
.Ltmp5161:
	.loc	5 3867 14
	local.get	63
	local.get	38
	f32x4.mul
.Ltmp5162:
	.loc	5 3845 14
	f32x4.add
.Ltmp5163:
	.loc	5 3867 14
	local.get	59
	local.get	39
	f32x4.mul
.Ltmp5164:
	.loc	5 3845 14
	f32x4.add
.Ltmp5165:
	.loc	5 3867 14
	local.get	57
	local.get	40
	f32x4.mul
.Ltmp5166:
	.loc	5 3845 14
	f32x4.add
.Ltmp5167:
	.loc	5 3867 14
	local.get	53
	local.get	41
	f32x4.mul
.Ltmp5168:
	.loc	5 3845 14
	f32x4.add
.Ltmp5169:
	.loc	5 3812 14
	f32x4.abs
.Ltmp5170:
	.loc	5 3867 14
	local.get	96
	local.get	42
	f32x4.mul
.Ltmp5171:
	.loc	5 3845 14
	local.get	100
	f32x4.add
.Ltmp5172:
	.loc	5 3867 14
	local.get	92
	local.get	22
	f32x4.mul
.Ltmp5173:
	.loc	5 3845 14
	f32x4.add
.Ltmp5174:
	.loc	5 3867 14
	local.get	88
	local.get	37
	f32x4.mul
.Ltmp5175:
	.loc	5 3845 14
	f32x4.add
.Ltmp5176:
	.loc	5 3867 14
	local.get	84
	local.get	33
	f32x4.mul
.Ltmp5177:
	.loc	5 3845 14
	f32x4.add
.Ltmp5178:
	.loc	5 3867 14
	local.get	80
	local.get	34
	f32x4.mul
.Ltmp5179:
	.loc	5 3845 14
	f32x4.add
.Ltmp5180:
	.loc	5 3867 14
	local.get	76
	local.get	35
	f32x4.mul
.Ltmp5181:
	.loc	5 3845 14
	f32x4.add
.Ltmp5182:
	.loc	5 3867 14
	local.get	72
	local.get	21
	f32x4.mul
.Ltmp5183:
	.loc	5 3845 14
	f32x4.add
.Ltmp5184:
	.loc	5 3867 14
	local.get	68
	local.get	36
	f32x4.mul
.Ltmp5185:
	.loc	5 3845 14
	f32x4.add
.Ltmp5186:
	.loc	5 3867 14
	local.get	64
	local.get	38
	f32x4.mul
.Ltmp5187:
	.loc	5 3845 14
	f32x4.add
.Ltmp5188:
	.loc	5 3867 14
	local.get	60
	local.get	39
	f32x4.mul
.Ltmp5189:
	.loc	5 3845 14
	f32x4.add
.Ltmp5190:
	.loc	5 3867 14
	local.get	24
	local.get	40
	f32x4.mul
.Ltmp5191:
	.loc	5 3845 14
	f32x4.add
.Ltmp5192:
	.loc	5 3867 14
	local.get	54
	local.get	41
	f32x4.mul
.Ltmp5193:
	.loc	5 3845 14
	f32x4.add
.Ltmp5194:
	.loc	5 3812 14
	f32x4.abs
.Ltmp5195:
	.loc	5 3867 14
	local.get	97
	local.get	42
	f32x4.mul
.Ltmp5196:
	.loc	5 3845 14
	local.get	100
	f32x4.add
.Ltmp5197:
	.loc	5 3867 14
	local.get	93
	local.get	22
	f32x4.mul
.Ltmp5198:
	.loc	5 3845 14
	f32x4.add
.Ltmp5199:
	.loc	5 3867 14
	local.get	89
	local.get	37
	f32x4.mul
.Ltmp5200:
	.loc	5 3845 14
	f32x4.add
.Ltmp5201:
	.loc	5 3867 14
	local.get	85
	local.get	33
	f32x4.mul
.Ltmp5202:
	.loc	5 3845 14
	f32x4.add
.Ltmp5203:
	.loc	5 3867 14
	local.get	81
	local.get	34
	f32x4.mul
.Ltmp5204:
	.loc	5 3845 14
	f32x4.add
.Ltmp5205:
	.loc	5 3867 14
	local.get	77
	local.get	35
	f32x4.mul
.Ltmp5206:
	.loc	5 3845 14
	f32x4.add
.Ltmp5207:
	.loc	5 3867 14
	local.get	73
	local.get	21
	f32x4.mul
.Ltmp5208:
	.loc	5 3845 14
	f32x4.add
.Ltmp5209:
	.loc	5 3867 14
	local.get	69
	local.get	36
	f32x4.mul
.Ltmp5210:
	.loc	5 3845 14
	f32x4.add
.Ltmp5211:
	.loc	5 3867 14
	local.get	65
	local.get	38
	f32x4.mul
.Ltmp5212:
	.loc	5 3845 14
	f32x4.add
.Ltmp5213:
	.loc	5 3867 14
	local.get	61
	local.get	39
	f32x4.mul
.Ltmp5214:
	.loc	5 3845 14
	f32x4.add
.Ltmp5215:
	.loc	5 3867 14
	local.get	58
	local.get	40
	f32x4.mul
.Ltmp5216:
	.loc	5 3845 14
	f32x4.add
.Ltmp5217:
	.loc	5 3867 14
	local.get	55
	local.get	41
	f32x4.mul
.Ltmp5218:
	.loc	5 3845 14
	f32x4.add
.Ltmp5219:
	.loc	5 3812 14
	f32x4.abs
.Ltmp5220:
	.loc	5 3867 14
	local.get	98
	local.get	42
	f32x4.mul
.Ltmp5221:
	.loc	5 3845 14
	local.get	100
	f32x4.add
.Ltmp5222:
	.loc	5 3867 14
	local.get	94
	local.get	22
	f32x4.mul
.Ltmp5223:
	.loc	5 3845 14
	f32x4.add
.Ltmp5224:
	.loc	5 3867 14
	local.get	90
	local.get	37
	f32x4.mul
.Ltmp5225:
	.loc	5 3845 14
	f32x4.add
.Ltmp5226:
	.loc	5 3867 14
	local.get	86
	local.get	33
	f32x4.mul
.Ltmp5227:
	.loc	5 3845 14
	f32x4.add
.Ltmp5228:
	.loc	5 3867 14
	local.get	82
	local.get	34
	f32x4.mul
.Ltmp5229:
	.loc	5 3845 14
	f32x4.add
.Ltmp5230:
	.loc	5 3867 14
	local.get	78
	local.get	35
	f32x4.mul
.Ltmp5231:
	.loc	5 3845 14
	f32x4.add
.Ltmp5232:
	.loc	5 3867 14
	local.get	74
	local.get	21
	f32x4.mul
.Ltmp5233:
	.loc	5 3845 14
	f32x4.add
.Ltmp5234:
	.loc	5 3867 14
	local.get	70
	local.get	36
	f32x4.mul
.Ltmp5235:
	.loc	5 3845 14
	f32x4.add
.Ltmp5236:
	.loc	5 3867 14
	local.get	66
	local.get	38
	f32x4.mul
.Ltmp5237:
	.loc	5 3845 14
	f32x4.add
.Ltmp5238:
	.loc	5 3867 14
	local.get	62
	local.get	39
	f32x4.mul
.Ltmp5239:
	.loc	5 3845 14
	f32x4.add
.Ltmp5240:
	.loc	5 3867 14
	local.get	23
	local.get	40
	f32x4.mul
.Ltmp5241:
	.loc	5 3845 14
	f32x4.add
.Ltmp5242:
	.loc	5 3867 14
	local.get	56
	local.get	41
	f32x4.mul
.Ltmp5243:
	.loc	5 3845 14
	f32x4.add
.Ltmp5244:
	.loc	5 3812 14
	f32x4.abs
.Ltmp5245:
	.loc	5 3812 14 is_stmt 0
	local.get	21
	f32x4.abs
.Ltmp5246:
	.loc	5 3928 9 is_stmt 1
	f32x4.pmax
	f32x4.pmax
	f32x4.pmax
	f32x4.pmax
.Ltmp5247:
	.loc	4 551 14
	v128.store	0:p2align=2
	local.get	7
	i32.const	16
.Ltmp5248:
	.loc	17 1916 50
	i32.add 
	local.set	7
	local.get	8
	i32.const	-4
	i32.add 
	local.set	8
	local.get	9
	i32.const	4
	i32.add 
	local.set	9
	local.get	10
	i32.const	16
	i32.add 
	local.set	10
	local.get	40
	local.set	100
	local.get	39
	local.set	101
	local.get	38
	local.set	102
	local.get	36
	local.set	103
	local.get	21
	local.set	104
	local.get	35
	local.set	105
	local.get	34
	local.set	32
	local.get	33
	local.set	43
	local.get	37
	local.set	106
	local.get	22
	local.set	107
	local.get	11
	i32.const	-1
	i32.add 
.Ltmp5249:
	.loc	18 900 12
	local.tee	11
	br_if   	0
.LBB28_161:
	end_loop
	end_block
.Ltmp5250:
	.loc	22 0 0 is_stmt 0
	local.get	41
	local.set	120
	local.get	40
	local.set	121
	local.get	39
	local.set	122
	local.get	38
	local.set	123
	local.get	36
	local.set	124
	local.get	21
	local.set	125
	local.get	35
	local.set	126
	local.get	34
	local.set	127
	local.get	33
	local.set	128
	local.get	37
	local.set	129
	local.get	22
	local.set	130
.Ltmp5251:
	.loc	18 900 12
	block   	
	block   	
	block   	
	local.get	12
	i32.eqz
	br_if   	0
	.loc	18 0 12
	local.get	133
	local.set	22
	.loc	18 900 12
	br      	1
.Ltmp5252:
.LBB28_163:
	.loc	18 0 12
	end_block
	local.get	0
	i32.load	920
	local.set	47
	local.get	0
	i32.load	916
	local.set	11
	i32.const	0
	local.set	112
	local.get	133
	local.set	22
.LBB28_164:
.Ltmp5253:
	.loc	22 3276 24 is_stmt 1
	loop    	
	block   	
	block   	
	block   	
	local.get	2
	local.get	112
	local.get	19
	i32.add 
	i32.const	2
	i32.shl 
.Ltmp5254:
	.loc	42 568 12
	local.tee	9
	i32.lt_u
	br_if   	0
	.loc	42 573 27
	block   	
	local.get	2
	local.get	9
	i32.sub 
	local.tee	8
	i32.const	3
.Ltmp5255:
	.loc	42 438 16
	i32.le_u
	br_if   	0
.Ltmp5256:
	.loc	22 1392 25
	local.get	0
	i32.load	944
.Ltmp5257:
	.loc	22 1388 17
	local.tee	8
	local.get	0
	i32.load	1020
.Ltmp5258:
	.loc	22 1392 45
	local.tee	26
	local.get	17
	i32.mul 
.Ltmp5259:
	.loc	42 580 12
	local.tee	25
	i32.lt_u
	br_if   	12
	.loc	42 585 27
	block   	
	local.get	8
	local.get	25
	i32.sub 
	local.tee	8
	i32.const	3
.Ltmp5260:
	.loc	42 451 16
	i32.le_u
	br_if   	0
.Ltmp5261:
	.loc	42 0 16 is_stmt 0
	local.get	1
	local.get	9
	i32.const	2
	i32.shl 
	i32.add 
	local.tee	48
	v128.load	0:p2align=2
	local.set	33
.Ltmp5262:
	.loc	22 1392 25 is_stmt 1
	local.get	0
	i32.load	940
	local.get	25
	i32.const	2
.Ltmp5263:
	.loc	42 101 24
	i32.shl 
	local.tee	113
	i32.add 
.Ltmp5264:
	.loc	22 0 0 is_stmt 0
	local.get	52
	local.get	4
	i32.const	1136
	i32.add 
	local.get	112
	i32.const	4
	i32.shl 
	i32.add 
	v128.load	0:p2align=2
.Ltmp5265:
	local.tee	21
	local.get	21
	local.get	99
	v128.bitselect
.Ltmp5266:
	local.tee	21
	f32x4.div
	v128.const	0, 0, 128, 63, 0, 0, 128, 63, 0, 0, 128, 63, 0, 0, 128, 63
	local.get	52
	local.get	21
	f32x4.lt
	v128.bitselect
.Ltmp5267:
	.loc	4 551 14 is_stmt 1
	v128.store	0:p2align=2
.Ltmp5268:
	.loc	22 1259 17
	block   	
	block   	
	block   	
	local.get	0
	i32.load	1020
.Ltmp5269:
	.loc	45 37 12
	local.tee	6
	i32.eqz
	br_if   	0
	.loc	45 0 12 is_stmt 0
	i32.const	0
	local.set	51
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
	local.set	117
	local.get	0
	i32.load	964
	local.set	29
	local.get	0
	i32.load	968
	local.set	118
	local.get	0
	i32.load	980
	local.set	116
	local.get	0
	i32.load	984
	local.set	110
	local.get	0
	i32.load	940
	local.set	13
	local.get	0
	i32.load	944
	local.set	12
	local.get	0
	i32.load	1012
	local.set	115
	local.get	0
	i32.load	1016
	local.set	111
	i32.const	0
	local.set	7
	local.get	6
	local.set	50
.LBB28_170:
	loop    	
	local.get	51
	i32.const	32
.Ltmp5270:
	.loc	24 1714 9 is_stmt 1
	i32.eq  
.Ltmp5271:
	.loc	23 180 28
	br_if   	1
.Ltmp5272:
	.loc	22 1263 21
	block   	
	block   	
	block   	
	block   	
	local.get	7
	local.get	111
	i32.eq  
	br_if   	0
	.loc	22 0 21 is_stmt 0
	local.get	115
	local.get	7
	i32.const	12
	.loc	22 1263 21
	i32.mul 
	i32.add 
	local.tee	8
	i32.load	4
.Ltmp5273:
	.loc	22 1265 23 is_stmt 1
	local.get	17
	i32.add 
	local.tee	9
	i32.const	0
.Ltmp5274:
	.loc	22 1266 12
	local.get	11
	local.get	9
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
.Ltmp5275:
	.loc	22 1273 42
	local.tee	9
	local.get	6
	i32.mul 
	local.get	7
	i32.add 
	.loc	22 1273 22 is_stmt 0
	local.tee	10
	local.get	12
	i32.ge_u
	br_if   	1
.Ltmp5276:
	.loc	22 1274 24 is_stmt 1
	local.get	7
	local.get	110
	i32.eq  
	br_if   	2
.Ltmp5277:
	.loc	22 0 0 is_stmt 0
	local.get	8
	i32.load	0
	local.set	8
	local.get	13
	local.get	10
	i32.const	2
.Ltmp5278:
	i32.shl 
	i32.add 
	local.tee	18
	f32.load	0
	local.set	134
	local.get	116
	local.get	7
	i32.const	2
.Ltmp5279:
	.loc	22 1274 24
	i32.shl 
	local.tee	10
	i32.add 
	local.tee	114
	i32.load	0
.Ltmp5280:
	.loc	22 1275 26 is_stmt 1
	local.tee	27
	i32.eqz
	br_if   	3
	.loc	22 1278 24
	block   	
	block   	
	local.get	7
	local.get	118
	i32.ge_u
	br_if   	0
	local.get	29
	local.get	10
	i32.add 
	f32.load	0
.Ltmp5281:
	.loc	22 903 8
	local.tee	135
	local.get	134
	f32.lt  
	br_if   	1
	br      	5
.Ltmp5282:
.LBB28_177:
	.loc	22 0 8 is_stmt 0
	end_block
	local.get	4
	local.get	133
	v128.store	704
.Ltmp5283:
	.loc	22 1765 5 is_stmt 1
	local.get	4
	local.get	120
	v128.store	544
	local.get	4
	local.get	121
	v128.store	528
	local.get	4
	local.get	122
	v128.store	512
	local.get	4
	local.get	123
	v128.store	496
	local.get	4
	local.get	124
	v128.store	480
	local.get	4
	local.get	125
	v128.store	464
	local.get	4
	local.get	126
	v128.store	448
	local.get	4
	local.get	127
	v128.store	432
	local.get	4
	local.get	128
	v128.store	416
	local.get	4
	local.get	129
	v128.store	400
	local.get	4
	local.get	130
	v128.store	384
.Ltmp5284:
	.loc	22 1278 24
	local.get	7
	local.get	118
	i32.const	.Lalloc_a228cac3279aae3a34886f59f51fbe9e
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.LBB28_178:
	.loc	22 0 24 is_stmt 0
	end_block
	local.get	135
	local.set	134
.Ltmp5285:
	.loc	22 903 5 is_stmt 1
	br      	3
.Ltmp5286:
.LBB28_179:
	.loc	22 0 5 is_stmt 0
	end_block
	local.get	4
	local.get	133
	v128.store	704
.Ltmp5287:
	.loc	22 1765 5 is_stmt 1
	local.get	4
	local.get	120
	v128.store	544
	local.get	4
	local.get	121
	v128.store	528
	local.get	4
	local.get	122
	v128.store	512
	local.get	4
	local.get	123
	v128.store	496
	local.get	4
	local.get	124
	v128.store	480
	local.get	4
	local.get	125
	v128.store	464
	local.get	4
	local.get	126
	v128.store	448
	local.get	4
	local.get	127
	v128.store	432
	local.get	4
	local.get	128
	v128.store	416
	local.get	4
	local.get	129
	v128.store	400
	local.get	4
	local.get	130
	v128.store	384
.Ltmp5288:
	.loc	22 1263 21
	local.get	111
	local.get	111
	i32.const	.Lalloc_12856b5f9033764dbf23e04eb77c5c46
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.Ltmp5289:
.LBB28_180:
	.loc	22 0 21 is_stmt 0
	end_block
	local.get	4
	local.get	133
	v128.store	704
.Ltmp5290:
	.loc	22 1765 5 is_stmt 1
	local.get	4
	local.get	120
	v128.store	544
	local.get	4
	local.get	121
	v128.store	528
	local.get	4
	local.get	122
	v128.store	512
	local.get	4
	local.get	123
	v128.store	496
	local.get	4
	local.get	124
	v128.store	480
	local.get	4
	local.get	125
	v128.store	464
	local.get	4
	local.get	126
	v128.store	448
	local.get	4
	local.get	127
	v128.store	432
	local.get	4
	local.get	128
	v128.store	416
	local.get	4
	local.get	129
	v128.store	400
	local.get	4
	local.get	130
	v128.store	384
.Ltmp5291:
	.loc	22 1273 22
	local.get	10
	local.get	12
	i32.const	.Lalloc_70ebf0cf1c90c6161926611ccf249a32
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.Ltmp5292:
.LBB28_181:
	.loc	22 0 22 is_stmt 0
	end_block
	local.get	4
	local.get	133
	v128.store	704
.Ltmp5293:
	.loc	22 1765 5 is_stmt 1
	local.get	4
	local.get	120
	v128.store	544
	local.get	4
	local.get	121
	v128.store	528
	local.get	4
	local.get	122
	v128.store	512
	local.get	4
	local.get	123
	v128.store	496
	local.get	4
	local.get	124
	v128.store	480
	local.get	4
	local.get	125
	v128.store	464
	local.get	4
	local.get	126
	v128.store	448
	local.get	4
	local.get	127
	v128.store	432
	local.get	4
	local.get	128
	v128.store	416
	local.get	4
	local.get	129
	v128.store	400
	local.get	4
	local.get	130
	v128.store	384
.Ltmp5294:
	.loc	22 1274 24
	local.get	110
	local.get	110
	i32.const	.Lalloc_023d591aaad7f5cf482ac80a9401eca1
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.LBB28_182:
	.loc	22 0 24 is_stmt 0
	end_block
.Ltmp5295:
	.loc	22 1280 9 is_stmt 1
	block   	
	block   	
	block   	
	local.get	7
	local.get	118
	i32.eq  
	br_if   	0
.Ltmp5296:
	.loc	22 0 0 is_stmt 0
	local.get	4
	i32.const	2160
	i32.add 
	local.get	51
	i32.add 
	local.set	28
.Ltmp5297:
	.loc	22 1280 9
	local.get	29
	local.get	10
	i32.add 
	local.get	134
	f32.store	0
	local.get	27
	i32.const	1
	.loc	22 1281 24 is_stmt 1
	i32.add 
	local.tee	27
	local.get	8
	i32.ne  
.Ltmp5298:
	.loc	22 1282 23
	br_if   	1
	.loc	22 1282 9 is_stmt 0
	local.get	28
	local.get	134
	f32.store	0
	i32.const	0
	local.set	27
	local.get	8
	i32.eqz
	br_if   	2
	.loc	22 1288 30 is_stmt 1
	local.get	18
	f32.load	0
	local.set	134
.LBB28_186:
.Ltmp5299:
	.loc	22 1291 65
	loop    	
	local.get	9
	local.get	6
	i32.mul 
	local.get	7
	i32.add 
	.loc	22 1291 45 is_stmt 0
	local.tee	10
	local.get	12
	i32.ge_u
	br_if   	7
	.loc	22 0 45
	local.get	13
	local.get	10
	i32.const	2
	.loc	22 1291 45
	i32.shl 
	i32.add 
	local.tee	10
	local.get	134
	local.get	10
	f32.load	0
.Ltmp5300:
	.loc	22 903 8 is_stmt 1
	local.tee	135
	local.get	134
	local.get	135
	f32.lt  
	f32.select
.Ltmp5301:
	.loc	22 1292 17
	local.tee	134
	f32.store	0
	.loc	22 1293 20
	local.get	9
	local.get	11
	local.get	9
	i32.select
	i32.const	-1
	.loc	22 1296 17
	i32.add 
	local.set	9
	local.get	8
	i32.const	-1
.Ltmp5302:
	.loc	17 1916 50
	i32.add 
.Ltmp5303:
	.loc	18 900 12
	local.tee	8
	br_if   	0
	br      	3
.Ltmp5304:
.LBB28_188:
	.loc	22 1280 9
	end_loop
	end_block
	local.get	4
	local.get	133
	v128.store	704
.Ltmp5305:
	.loc	22 1765 5
	local.get	4
	local.get	120
	v128.store	544
	local.get	4
	local.get	121
	v128.store	528
	local.get	4
	local.get	122
	v128.store	512
	local.get	4
	local.get	123
	v128.store	496
	local.get	4
	local.get	124
	v128.store	480
	local.get	4
	local.get	125
	v128.store	464
	local.get	4
	local.get	126
	v128.store	448
	local.get	4
	local.get	127
	v128.store	432
	local.get	4
	local.get	128
	v128.store	416
	local.get	4
	local.get	129
	v128.store	400
	local.get	4
	local.get	130
	v128.store	384
.Ltmp5306:
	.loc	22 1280 9
	local.get	118
	local.get	118
	i32.const	.Lalloc_1fa5d88c1a2cc292a2767c48606a38fe
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.LBB28_189:
	.loc	22 0 9 is_stmt 0
	end_block
.Ltmp5307:
	.loc	22 1285 44 is_stmt 1
	local.get	7
	local.get	117
	i32.add 
	.loc	22 1285 24 is_stmt 0
	local.tee	9
	local.get	12
	i32.ge_u
	br_if   	3
	.loc	22 0 24
	local.get	28
	local.get	13
	local.get	9
	i32.const	2
	.loc	22 1285 24
	i32.shl 
	i32.add 
	f32.load	0
.Ltmp5308:
	.loc	22 903 8 is_stmt 1
	local.tee	135
	local.get	134
	local.get	135
	local.get	134
	f32.lt  
	f32.select
.Ltmp5309:
	.loc	22 1282 9
	f32.store	0
.Ltmp5310:
.LBB28_191:
	.loc	22 0 9 is_stmt 0
	end_block
	local.get	7
	i32.const	1
	i32.add 
	local.set	7
	local.get	51
	i32.const	4
	i32.add 
	local.set	51
.Ltmp5311:
	local.get	114
	local.get	27
	i32.store	0
	local.get	50
	i32.const	-1
.Ltmp5312:
	i32.add 
.Ltmp5313:
	.loc	45 37 12 is_stmt 1
	local.tee	50
	br_if   	0
.LBB28_192:
	.loc	45 37 12
	end_loop
	end_block
.Ltmp5314:
	.loc	22 1412 26
	local.get	0
	i32.load	952
	local.set	9
.Ltmp5315:
	.loc	4 551 14
	local.get	4
	v128.load	2160:p2align=3
	local.tee	37
	local.set	21
	local.get	26
	i32.eqz
	br_if   	6
.Ltmp5316:
	.loc	4 0 14 is_stmt 0
	block   	
	local.get	0
	i32.load	1016
	local.tee	10
	br_if   	0
	i32.const	0
	local.set	8
	br      	17
.LBB28_195:
	end_block
	block   	
	local.get	0
	i32.load	1012
.Ltmp5317:
	.loc	22 1403 42 is_stmt 1
	local.tee	7
	i32.load	8
	.loc	22 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5318:
	.loc	22 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	22 1407 40
	local.get	26
	i32.mul 
	.loc	22 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	22 0 25
	local.get	4
	local.get	0
	i32.load	948
	local.tee	12
	local.get	8
	i32.const	2
	.loc	22 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	22 1407 13
	f32.store	2160
	local.get	26
	i32.const	1
.Ltmp5319:
	.loc	45 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5320:
	.loc	45 0 12 is_stmt 0
	i32.const	1
	local.set	8
	local.get	10
	i32.const	1
.Ltmp5321:
	.loc	22 1403 42 is_stmt 1
	i32.eq  
	br_if   	17
	local.get	7
	i32.load	20
	.loc	22 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5322:
	.loc	22 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	22 1407 40
	local.get	26
	i32.mul 
	i32.const	1
	i32.add 
	.loc	22 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	22 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	22 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	22 1407 13
	f32.store	2164
	local.get	26
	i32.const	2
.Ltmp5323:
	.loc	45 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5324:
	.loc	45 0 12 is_stmt 0
	i32.const	2
	local.set	8
	local.get	10
	i32.const	2
.Ltmp5325:
	.loc	22 1403 42 is_stmt 1
	i32.eq  
	br_if   	17
	local.get	7
	i32.load	32
	.loc	22 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5326:
	.loc	22 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	22 1407 40
	local.get	26
	i32.mul 
	i32.const	2
	i32.add 
	.loc	22 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	22 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	22 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	22 1407 13
	f32.store	2168
	local.get	26
	i32.const	3
.Ltmp5327:
	.loc	45 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5328:
	.loc	45 0 12 is_stmt 0
	i32.const	3
	local.set	8
	local.get	10
	i32.const	3
.Ltmp5329:
	.loc	22 1403 42 is_stmt 1
	i32.eq  
	br_if   	17
	local.get	7
	i32.load	44
	.loc	22 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5330:
	.loc	22 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	22 1407 40
	local.get	26
	i32.mul 
	i32.const	3
	i32.add 
	.loc	22 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	22 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	22 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	22 1407 13
	f32.store	2172
	local.get	26
	i32.const	4
.Ltmp5331:
	.loc	45 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5332:
	.loc	45 0 12 is_stmt 0
	i32.const	4
	local.set	8
	local.get	10
	i32.const	4
.Ltmp5333:
	.loc	22 1403 42 is_stmt 1
	i32.eq  
	br_if   	17
	local.get	7
	i32.load	56
	.loc	22 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5334:
	.loc	22 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	22 1407 40
	local.get	26
	i32.mul 
	i32.const	4
	i32.add 
	.loc	22 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	22 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	22 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	22 1407 13
	f32.store	2176
	local.get	26
	i32.const	5
.Ltmp5335:
	.loc	45 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5336:
	.loc	45 0 12 is_stmt 0
	i32.const	5
	local.set	8
	local.get	10
	i32.const	5
.Ltmp5337:
	.loc	22 1403 42 is_stmt 1
	i32.eq  
	br_if   	17
	local.get	7
	i32.load	68
	.loc	22 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5338:
	.loc	22 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	22 1407 40
	local.get	26
	i32.mul 
	i32.const	5
	i32.add 
	.loc	22 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	22 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	22 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	22 1407 13
	f32.store	2180
	local.get	26
	i32.const	6
.Ltmp5339:
	.loc	45 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5340:
	.loc	45 0 12 is_stmt 0
	i32.const	6
	local.set	8
	local.get	10
	i32.const	6
.Ltmp5341:
	.loc	22 1403 42 is_stmt 1
	i32.eq  
	br_if   	17
	local.get	7
	i32.load	80
	.loc	22 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5342:
	.loc	22 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	22 1407 40
	local.get	26
	i32.mul 
	i32.const	6
	i32.add 
	.loc	22 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	22 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	22 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	22 1407 13
	f32.store	2184
	local.get	26
	i32.const	7
.Ltmp5343:
	.loc	45 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5344:
	.loc	45 0 12 is_stmt 0
	i32.const	7
	local.set	8
	local.get	10
	i32.const	7
.Ltmp5345:
	.loc	22 1403 42 is_stmt 1
	i32.eq  
	br_if   	17
	local.get	7
	i32.load	92
	.loc	22 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5346:
	.loc	22 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	22 1407 40
	local.get	26
	i32.mul 
	i32.const	7
	i32.add 
	.loc	22 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	22 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	22 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	22 1407 13
	f32.store	2188
.Ltmp5347:
	.loc	45 37 12 is_stmt 1
	br      	6
.Ltmp5348:
.LBB28_218:
	.loc	45 0 12 is_stmt 0
	end_block
	local.get	4
	local.get	133
	v128.store	704
.Ltmp5349:
	.loc	22 1765 5 is_stmt 1
	local.get	4
	local.get	120
	v128.store	544
	local.get	4
	local.get	121
	v128.store	528
	local.get	4
	local.get	122
	v128.store	512
	local.get	4
	local.get	123
	v128.store	496
	local.get	4
	local.get	124
	v128.store	480
	local.get	4
	local.get	125
	v128.store	464
	local.get	4
	local.get	126
	v128.store	448
	local.get	4
	local.get	127
	v128.store	432
	local.get	4
	local.get	128
	v128.store	416
	local.get	4
	local.get	129
	v128.store	400
	local.get	4
	local.get	130
	v128.store	384
.Ltmp5350:
	.loc	22 1407 25
	local.get	8
	local.get	9
	i32.const	.Lalloc_0e76a26fbb751dfb8cf8a069b5b0d39a
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.Ltmp5351:
.LBB28_219:
	.loc	22 0 25 is_stmt 0
	end_block
	local.get	4
	local.get	133
	v128.store	704
.Ltmp5352:
	.loc	22 1765 5 is_stmt 1
	local.get	4
	local.get	120
	v128.store	544
	local.get	4
	local.get	121
	v128.store	528
	local.get	4
	local.get	122
	v128.store	512
	local.get	4
	local.get	123
	v128.store	496
	local.get	4
	local.get	124
	v128.store	480
	local.get	4
	local.get	125
	v128.store	464
	local.get	4
	local.get	126
	v128.store	448
	local.get	4
	local.get	127
	v128.store	432
	local.get	4
	local.get	128
	v128.store	416
	local.get	4
	local.get	129
	v128.store	400
	local.get	4
	local.get	130
	v128.store	384
.Ltmp5353:
	.loc	22 1285 24
	local.get	9
	local.get	12
	i32.const	.Lalloc_70d40ae2302f3ea19fa0c4a65fe82786
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.Ltmp5354:
.LBB28_220:
	.loc	22 0 24 is_stmt 0
	end_block
	local.get	4
	local.get	133
	v128.store	704
.Ltmp5355:
	.loc	22 1765 5 is_stmt 1
	local.get	4
	local.get	120
	v128.store	544
	local.get	4
	local.get	121
	v128.store	528
	local.get	4
	local.get	122
	v128.store	512
	local.get	4
	local.get	123
	v128.store	496
	local.get	4
	local.get	124
	v128.store	480
	local.get	4
	local.get	125
	v128.store	464
	local.get	4
	local.get	126
	v128.store	448
	local.get	4
	local.get	127
	v128.store	432
	local.get	4
	local.get	128
	v128.store	416
	local.get	4
	local.get	129
	v128.store	400
	local.get	4
	local.get	130
	v128.store	384
.Ltmp5356:
	.loc	22 1291 45
	local.get	10
	local.get	12
	i32.const	.Lalloc_a271bbb7fd83c3605ea58a06b7065fa4
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.Ltmp5357:
.LBB28_221:
	.loc	22 0 45 is_stmt 0
	end_block
	local.get	4
	local.get	133
	v128.store	704
.Ltmp5358:
	.loc	22 1765 5 is_stmt 1
	local.get	4
	local.get	120
	v128.store	544
	local.get	4
	local.get	121
	v128.store	528
	local.get	4
	local.get	122
	v128.store	512
	local.get	4
	local.get	123
	v128.store	496
	local.get	4
	local.get	124
	v128.store	480
	local.get	4
	local.get	125
	v128.store	464
	local.get	4
	local.get	126
	v128.store	448
	local.get	4
	local.get	127
	v128.store	432
	local.get	4
	local.get	128
	v128.store	416
	local.get	4
	local.get	129
	v128.store	400
	local.get	4
	local.get	130
	v128.store	384
	i32.const	0
	i32.const	4
.Ltmp5359:
	.loc	42 456 13
	local.get	8
	i32.const	.Lalloc_5f4e05826d69b5e832dc96c63f325058
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5360:
.LBB28_222:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	133
	v128.store	704
.Ltmp5361:
	.loc	22 1765 5 is_stmt 1
	local.get	4
	local.get	120
	v128.store	544
	local.get	4
	local.get	121
	v128.store	528
	local.get	4
	local.get	122
	v128.store	512
	local.get	4
	local.get	123
	v128.store	496
	local.get	4
	local.get	124
	v128.store	480
	local.get	4
	local.get	125
	v128.store	464
	local.get	4
	local.get	126
	v128.store	448
	local.get	4
	local.get	127
	v128.store	432
	local.get	4
	local.get	128
	v128.store	416
	local.get	4
	local.get	129
	v128.store	400
	local.get	4
	local.get	130
	v128.store	384
	i32.const	0
	i32.const	4
.Ltmp5362:
	.loc	42 443 13
	local.get	8
	i32.const	.Lalloc_5f4e05826d69b5e832dc96c63f325058
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5363:
.LBB28_223:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	133
	v128.store	704
.Ltmp5364:
	.loc	22 1765 5 is_stmt 1
	local.get	4
	local.get	120
	v128.store	544
	local.get	4
	local.get	121
	v128.store	528
	local.get	4
	local.get	122
	v128.store	512
	local.get	4
	local.get	123
	v128.store	496
	local.get	4
	local.get	124
	v128.store	480
	local.get	4
	local.get	125
	v128.store	464
	local.get	4
	local.get	126
	v128.store	448
	local.get	4
	local.get	127
	v128.store	432
	local.get	4
	local.get	128
	v128.store	416
	local.get	4
	local.get	129
	v128.store	400
	local.get	4
	local.get	130
	v128.store	384
.Ltmp5365:
	.loc	42 569 13
	local.get	9
	local.get	2
	local.get	2
	i32.const	.Lalloc_8b3aff86bf6965bbce4da27800054d27
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5366:
.LBB28_224:
	.loc	42 0 13 is_stmt 0
	end_block
.Ltmp5367:
	.loc	4 551 14 is_stmt 1
	local.get	4
	v128.load	2160:p2align=3
	local.set	21
.Ltmp5368:
.LBB28_225:
	.loc	4 0 14 is_stmt 0
	end_block
.Ltmp5369:
	.loc	42 580 12 is_stmt 1
	block   	
	local.get	9
	local.get	25
	i32.ge_u
	br_if   	0
.Ltmp5370:
	.loc	42 0 12 is_stmt 0
	local.get	4
	local.get	133
	v128.store	704
.Ltmp5371:
	.loc	22 1765 5 is_stmt 1
	local.get	4
	local.get	120
	v128.store	544
	local.get	4
	local.get	121
	v128.store	528
	local.get	4
	local.get	122
	v128.store	512
	local.get	4
	local.get	123
	v128.store	496
	local.get	4
	local.get	124
	v128.store	480
	local.get	4
	local.get	125
	v128.store	464
	local.get	4
	local.get	126
	v128.store	448
	local.get	4
	local.get	127
	v128.store	432
	local.get	4
	local.get	128
	v128.store	416
	local.get	4
	local.get	129
	v128.store	400
	local.get	4
	local.get	130
	v128.store	384
.Ltmp5372:
	.loc	42 581 13
	local.get	25
	local.get	9
	local.get	9
	i32.const	.Lalloc_c2286ff41a33b8c11aa270fe215441de
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.LBB28_227:
	.loc	42 0 13 is_stmt 0
	end_block
	.loc	42 585 27 is_stmt 1
	local.get	9
	local.get	25
	i32.sub 
	local.tee	9
	i32.const	3
.Ltmp5373:
	.loc	42 451 16
	i32.le_u
	br_if   	7
.Ltmp5374:
	.loc	22 1412 26
	local.get	0
	i32.load	948
.Ltmp5375:
	.loc	42 101 24
	local.get	113
	i32.add 
.Ltmp5376:
	.loc	22 0 0 is_stmt 0
	local.get	37
	v128.const	0x1p14, 0x1p14, 0x1p14, 0x1p14
	f32x4.mul
	f32x4.floor
	v128.const	0x1p-14, 0x1p-14, 0x1p-14, 0x1p-14
	f32x4.mul
.Ltmp5377:
	.loc	4 551 14 is_stmt 1
	local.tee	37
	v128.store	0:p2align=2
.Ltmp5378:
	.loc	22 1416 43
	local.get	4
	local.get	4
	v128.load	688
.Ltmp5379:
	.loc	5 3856 14
	local.tee	34
	local.get	132
	v128.const	0x1p0, 0x1p0, 0x1p0, 0x1p0
.Ltmp5380:
	.loc	22 0 0 is_stmt 0
	local.tee	35
	local.get	37
	local.get	22
	f32x4.add
	local.get	21
	f32x4.sub
.Ltmp5381:
	.loc	5 3878 14 is_stmt 1
	local.tee	22
	local.get	131
	f32x4.div
.Ltmp5382:
	.loc	5 3856 14
	f32x4.sub
.Ltmp5383:
	.loc	5 3856 14 is_stmt 0
	local.tee	21
	local.get	34
	f32x4.sub
.Ltmp5384:
	.loc	5 3867 14 is_stmt 1
	f32x4.mul
.Ltmp5385:
	.loc	5 3845 14
	f32x4.add
.Ltmp5386:
	.loc	5 3928 9
	local.get	21
	f32x4.pmax
.Ltmp5387:
	.loc	5 3812 14
	local.tee	21
	local.get	21
	f32x4.abs
.Ltmp5388:
	.loc	5 1991 14
	v128.const	0x1.79ca1p-67, 0x1.79ca1p-67, 0x1.79ca1p-67, 0x1.79ca1p-67
	f32x4.lt
.Ltmp5389:
	.loc	8 481 22
	v128.andnot
.Ltmp5390:
	.loc	22 1417 5
	local.tee	37
	v128.store	688
.Ltmp5391:
	.loc	22 1420 28
	local.get	0
	i32.load	936
	.loc	22 1420 44 is_stmt 0
	local.tee	8
	local.get	26
	local.get	109
	i32.mul 
.Ltmp5392:
	.loc	42 568 12 is_stmt 1
	local.tee	9
	i32.lt_u
	br_if   	4
	.loc	42 573 27
	local.get	8
	local.get	9
	i32.sub 
	local.tee	8
	i32.const	3
.Ltmp5393:
	.loc	42 438 16
	i32.le_u
	br_if   	2
.Ltmp5394:
	.loc	22 1420 28
	local.get	0
	i32.load	932
	local.get	9
	i32.const	2
.Ltmp5395:
	.loc	42 89 24
	i32.shl 
	i32.add 
.Ltmp5396:
	.loc	4 551 14
	local.tee	9
	v128.load	0:p2align=2
	local.set	21
.Ltmp5397:
	.loc	4 551 14 is_stmt 0
	local.get	9
	local.get	33
	v128.store	0:p2align=2
.Ltmp5398:
	.loc	22 0 0
	local.get	48
	local.get	21
	local.get	21
	local.get	35
	local.get	37
	f32x4.sub
.Ltmp5399:
	.loc	5 3867 14 is_stmt 1
	f32x4.mul
.Ltmp5400:
	.loc	5 2188 14
	local.get	119
	v128.bitselect
.Ltmp5401:
	.loc	4 551 14
	v128.store	0:p2align=2
	i32.const	0
	local.get	17
	i32.const	1
.Ltmp5402:
	.loc	22 3304 13
	i32.add 
	.loc	22 3305 16
	local.tee	9
	local.get	9
	local.get	11
	i32.eq  
	i32.select
	local.set	17
	i32.const	0
	local.get	109
	i32.const	1
	.loc	22 3300 13
	i32.add 
	.loc	22 3301 16
	local.tee	9
	local.get	9
	local.get	47
	i32.eq  
	i32.select
	local.set	109
	local.get	112
	i32.const	1
.Ltmp5403:
	.loc	22 0 0 is_stmt 0
	i32.add 
.Ltmp5404:
	.loc	17 1916 50 is_stmt 1
	local.tee	112
	local.get	108
	i32.ne  
.Ltmp5405:
	.loc	18 900 12
	br_if   	0
.LBB28_231:
	end_loop
	end_block
	local.get	19
	i32.const	32
.Ltmp5406:
	.loc	22 0 0 is_stmt 0
	i32.add 
	local.set	19
	local.get	30
	i32.const	512
.Ltmp5407:
	.loc	44 446 20 is_stmt 1
	i32.add 
	local.set	30
	local.get	31
	i32.const	-128
	i32.add 
	local.set	31
	local.get	44
	i32.const	128
	i32.add 
	local.set	44
	local.get	49
	i32.const	-32
	i32.add 
	local.set	49
	local.get	45
	i32.const	-1
.Ltmp5408:
	.loc	22 0 0 is_stmt 0
	i32.add 
.Ltmp5409:
	.loc	44 446 20
	local.tee	45
	i32.eqz
	br_if   	3
	br      	1
.Ltmp5410:
.LBB28_232:
	.loc	44 0 20
	end_block
.Ltmp5411:
	.loc	42 438 16 is_stmt 1
	end_loop
	local.get	4
	local.get	133
	v128.store	704
.Ltmp5412:
	.loc	22 1765 5
	local.get	4
	local.get	120
	v128.store	544
	local.get	4
	local.get	121
	v128.store	528
	local.get	4
	local.get	122
	v128.store	512
	local.get	4
	local.get	123
	v128.store	496
	local.get	4
	local.get	124
	v128.store	480
	local.get	4
	local.get	125
	v128.store	464
	local.get	4
	local.get	126
	v128.store	448
	local.get	4
	local.get	127
	v128.store	432
	local.get	4
	local.get	128
	v128.store	416
	local.get	4
	local.get	129
	v128.store	400
	local.get	4
	local.get	130
	v128.store	384
	i32.const	0
	i32.const	4
.Ltmp5413:
	.loc	42 443 13
	local.get	8
	i32.const	.Lalloc_5f4e05826d69b5e832dc96c63f325058
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5414:
.LBB28_233:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	133
	v128.store	704
.Ltmp5415:
	.loc	22 1765 5 is_stmt 1
	local.get	4
	local.get	120
	v128.store	544
	local.get	4
	local.get	121
	v128.store	528
	local.get	4
	local.get	122
	v128.store	512
	local.get	4
	local.get	123
	v128.store	496
	local.get	4
	local.get	124
	v128.store	480
	local.get	4
	local.get	125
	v128.store	464
	local.get	4
	local.get	126
	v128.store	448
	local.get	4
	local.get	127
	v128.store	432
	local.get	4
	local.get	128
	v128.store	416
	local.get	4
	local.get	129
	v128.store	400
	local.get	4
	local.get	130
	v128.store	384
.Ltmp5416:
	.loc	42 569 13
	local.get	9
	local.get	8
	local.get	8
	i32.const	.Lalloc_2a3a21efd18b403ec25db545d522c479
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5417:
.LBB28_234:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	22
	v128.store	704
.Ltmp5418:
	.loc	22 1765 5 is_stmt 1
	local.get	4
	local.get	120
	v128.store	544
	local.get	4
	local.get	121
	v128.store	528
	local.get	4
	local.get	122
	v128.store	512
	local.get	4
	local.get	123
	v128.store	496
	local.get	4
	local.get	124
	v128.store	480
	local.get	4
	local.get	125
	v128.store	464
	local.get	4
	local.get	126
	v128.store	448
	local.get	4
	local.get	127
	v128.store	432
	local.get	4
	local.get	128
	v128.store	416
	local.get	4
	local.get	129
	v128.store	400
	local.get	4
	local.get	130
	v128.store	384
.Ltmp5419:
.LBB28_235:
	.loc	22 0 5 is_stmt 0
	end_block
	local.get	4
	local.get	42
	v128.store	368
.Ltmp5420:
	.loc	22 3311 14 is_stmt 1
	local.get	4
	i32.const	368
	i32.add 
	local.get	15
	call	_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_
	.loc	22 3313 5
	local.get	0
	local.get	17
	i32.store	804
	.loc	22 3312 5
	local.get	0
	local.get	109
	i32.store	800
.Ltmp5421:
	.loc	22 3203 13
	br      	4
.LBB28_236:
	.loc	22 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	133
	v128.store	704
.Ltmp5422:
	.loc	22 1765 5 is_stmt 1
	local.get	4
	local.get	120
	v128.store	544
	local.get	4
	local.get	121
	v128.store	528
	local.get	4
	local.get	122
	v128.store	512
	local.get	4
	local.get	123
	v128.store	496
	local.get	4
	local.get	124
	v128.store	480
	local.get	4
	local.get	125
	v128.store	464
	local.get	4
	local.get	126
	v128.store	448
	local.get	4
	local.get	127
	v128.store	432
	local.get	4
	local.get	128
	v128.store	416
	local.get	4
	local.get	129
	v128.store	400
	local.get	4
	local.get	130
	v128.store	384
	i32.const	0
	i32.const	4
.Ltmp5423:
	.loc	42 456 13
	local.get	9
	i32.const	.Lalloc_5f4e05826d69b5e832dc96c63f325058
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5424:
.LBB28_237:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	133
	v128.store	704
.Ltmp5425:
	.loc	22 1765 5 is_stmt 1
	local.get	4
	local.get	120
	v128.store	544
	local.get	4
	local.get	121
	v128.store	528
	local.get	4
	local.get	122
	v128.store	512
	local.get	4
	local.get	123
	v128.store	496
	local.get	4
	local.get	124
	v128.store	480
	local.get	4
	local.get	125
	v128.store	464
	local.get	4
	local.get	126
	v128.store	448
	local.get	4
	local.get	127
	v128.store	432
	local.get	4
	local.get	128
	v128.store	416
	local.get	4
	local.get	129
	v128.store	400
	local.get	4
	local.get	130
	v128.store	384
.Ltmp5426:
	.loc	42 581 13
	local.get	25
	local.get	8
	local.get	8
	i32.const	.Lalloc_db7c42041d7aaaba4839906f37294547
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5427:
.LBB28_238:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	133
	v128.store	704
.Ltmp5428:
	.loc	22 1765 5 is_stmt 1
	local.get	4
	local.get	120
	v128.store	544
	local.get	4
	local.get	121
	v128.store	528
	local.get	4
	local.get	122
	v128.store	512
	local.get	4
	local.get	123
	v128.store	496
	local.get	4
	local.get	124
	v128.store	480
	local.get	4
	local.get	125
	v128.store	464
	local.get	4
	local.get	126
	v128.store	448
	local.get	4
	local.get	127
	v128.store	432
	local.get	4
	local.get	128
	v128.store	416
	local.get	4
	local.get	129
	v128.store	400
	local.get	4
	local.get	130
	v128.store	384
.Ltmp5429:
	.loc	22 1403 42
	local.get	8
	local.get	8
	i32.const	.Lalloc_33746731a929bad63709ddedbca82f5f
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.Ltmp5430:
.LBB28_239:
	.loc	22 0 42 is_stmt 0
	end_block
.Ltmp5431:
	.loc	22 3252 24 is_stmt 1
	local.get	4
	i32.const	736
	i32.add 
	local.get	15
	call	_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_
.Ltmp5432:
	.loc	22 3260 27
	local.get	0
	i32.load	804
	local.set	17
.Ltmp5433:
	.loc	22 3259 27
	local.get	0
	i32.load	800
	local.set	109
.Ltmp5434:
	.loc	22 3258 21
	local.get	0
	i32.load8_u	785
	local.set	9
.Ltmp5435:
	.loc	22 3257 19
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
	.loc	22 0 19 is_stmt 0
	local.get	3
	i32.const	5
.Ltmp5436:
	.loc	21 3756 21 is_stmt 1
	i32.shr_u
	local.get	3
	i32.const	31
.Ltmp5437:
	.loc	21 3757 21
	i32.and 
	i32.const	0
.Ltmp5438:
	.loc	21 3758 16
	i32.ne  
	i32.add 
	local.set	45
.Ltmp5439:
	.loc	22 3258 21
	v128.const	-1, -1, -1, -1
	local.tee	21
	v128.const	0, 0, 0, 0
	local.tee	22
	local.get	9
	i32.const	1
	i32.and 
	v128.select
	local.set	119
	local.get	21
	local.get	22
	local.get	8
	i32.const	1
.Ltmp5440:
	.loc	22 3257 19
	i32.and 
	v128.select
	local.set	131
	local.get	4
	v128.load	1072
	local.set	33
	local.get	4
	v128.load	1040
	local.set	37
	local.get	4
	v128.load	976
	local.set	22
	local.get	4
	v128.load	912
	local.set	136
	local.get	4
	v128.load	896
	local.set	121
	local.get	4
	v128.load	880
	local.set	122
	local.get	4
	v128.load	864
	local.set	123
	local.get	4
	v128.load	848
	local.set	124
	local.get	4
	v128.load	832
	local.set	125
	local.get	4
	v128.load	816
	local.set	126
	local.get	4
	v128.load	800
	local.set	127
	local.get	4
	v128.load	784
	local.set	128
	local.get	4
	v128.load	768
	local.set	129
	local.get	4
	v128.load	752
	local.set	130
	local.get	4
	v128.load	736
	local.set	133
	local.get	4
	v128.load	1088
	local.set	132
	local.get	4
	v128.load	1008
	local.set	98
	local.get	4
	v128.load	944
	local.set	99
	local.get	1
	local.set	30
	local.get	2
	local.set	31
	i32.const	0
	local.set	44
	local.get	3
	local.set	49
	i32.const	0
	local.set	19
.LBB28_241:
	.loc	22 0 19 is_stmt 0
	block   	
	block   	
	loop    	
	local.get	22
	local.set	137
	local.get	37
	local.set	138
	local.get	33
	local.set	120
	local.get	49
	i32.const	1
	local.get	49
	i32.const	1
.Ltmp5441:
	.loc	21 2584 13 is_stmt 1
	i32.gt_u
	i32.select
	local.tee	9
	i32.const	32
	local.get	9
	i32.const	32
	i32.lt_u
	i32.select
	local.set	108
.Ltmp5442:
	.loc	17 1916 50
	block   	
	block   	
	local.get	3
	local.get	19
	i32.eq  
.Ltmp5443:
	.loc	18 900 12
	local.tee	12
	i32.eqz
	br_if   	0
	.loc	18 0 12 is_stmt 0
	local.get	133
	local.set	41
	local.get	130
	local.set	22
	local.get	129
	local.set	37
	local.get	128
	local.set	33
	local.get	127
	local.set	34
	local.get	126
	local.set	35
	local.get	125
	local.set	21
	local.get	124
	local.set	36
	local.get	123
	local.set	38
	local.get	122
	local.set	39
	local.get	121
	local.set	40
	local.get	136
	local.set	42
	.loc	18 900 12
	br      	1
.Ltmp5444:
.LBB28_243:
	.loc	18 0 12
	end_block
	local.get	0
	v128.load	768
	local.set	52
	local.get	0
	v128.load	752
	local.set	53
	local.get	0
	v128.load	736
	local.set	54
	local.get	0
	v128.load	720
	local.set	55
	local.get	0
	v128.load	704
	local.set	56
	local.get	0
	v128.load	688
	local.set	57
	local.get	0
	v128.load	672
	local.set	24
	local.get	0
	v128.load	656
	local.set	58
	local.get	0
	v128.load	640
	local.set	23
	local.get	0
	v128.load	624
	local.set	59
	local.get	0
	v128.load	608
	local.set	60
	local.get	0
	v128.load	592
	local.set	61
	local.get	0
	v128.load	576
	local.set	62
	local.get	0
	v128.load	560
	local.set	63
	local.get	0
	v128.load	544
	local.set	64
	local.get	0
	v128.load	528
	local.set	65
	local.get	0
	v128.load	512
	local.set	66
	local.get	0
	v128.load	496
	local.set	67
	local.get	0
	v128.load	480
	local.set	68
	local.get	0
	v128.load	464
	local.set	69
	local.get	0
	v128.load	448
	local.set	70
	local.get	0
	v128.load	432
	local.set	71
	local.get	0
	v128.load	416
	local.set	72
	local.get	0
	v128.load	400
	local.set	73
	local.get	0
	v128.load	384
	local.set	74
	local.get	0
	v128.load	368
	local.set	75
	local.get	0
	v128.load	352
	local.set	76
	local.get	0
	v128.load	336
	local.set	77
	local.get	0
	v128.load	320
	local.set	78
	local.get	0
	v128.load	304
	local.set	79
	local.get	0
	v128.load	288
	local.set	80
	local.get	0
	v128.load	272
	local.set	81
	local.get	0
	v128.load	256
	local.set	82
	local.get	0
	v128.load	240
	local.set	83
	local.get	0
	v128.load	224
	local.set	84
	local.get	0
	v128.load	208
	local.set	85
	local.get	0
	v128.load	192
	local.set	86
	local.get	0
	v128.load	176
	local.set	87
	local.get	0
	v128.load	160
	local.set	88
	local.get	0
	v128.load	144
	local.set	89
	local.get	0
	v128.load	128
	local.set	90
	local.get	0
	v128.load	112
	local.set	91
	local.get	0
	v128.load	96
	local.set	92
	local.get	0
	v128.load	80
	local.set	93
	local.get	0
	v128.load	64
	local.set	94
	local.get	0
	v128.load	48
	local.set	95
	local.get	0
	v128.load	32
	local.set	96
	local.get	0
	v128.load	16
	local.set	97
	local.get	4
	i32.const	1136
	i32.add 
	local.set	10
	local.get	30
	local.set	7
	local.get	31
	local.set	8
	local.get	44
	local.set	9
	local.get	108
	local.set	11
	local.get	121
	local.set	100
	local.get	122
	local.set	101
	local.get	123
	local.set	102
	local.get	124
	local.set	103
	local.get	125
	local.set	104
	local.get	126
	local.set	105
	local.get	127
	local.set	32
	local.get	128
	local.set	43
	local.get	129
	local.set	106
	local.get	130
	local.set	107
	local.get	133
	local.set	41
.LBB28_244:
	loop    	
	local.get	41
	local.set	22
	local.get	107
	local.set	37
	local.get	106
	local.set	33
	local.get	43
	local.set	34
	local.get	32
	local.set	35
	local.get	105
	local.set	21
	local.get	104
	local.set	36
	local.get	103
	local.set	38
	local.get	102
	local.set	39
	local.get	101
	local.set	40
	local.get	100
	local.set	42
.Ltmp5445:
	.loc	42 568 12 is_stmt 1
	block   	
	block   	
	local.get	9
	local.get	2
	i32.gt_u
	br_if   	0
.Ltmp5446:
	.loc	42 0 12 is_stmt 0
	local.get	8
	i32.const	3
.Ltmp5447:
	.loc	42 438 16 is_stmt 1
	i32.gt_u
	br_if   	1
.Ltmp5448:
	.loc	42 0 16 is_stmt 0
	local.get	4
	local.get	120
	v128.store	1072
	local.get	4
	local.get	138
	v128.store	1040
	local.get	4
	local.get	137
	v128.store	976
	local.get	4
	local.get	136
	v128.store	912
	local.get	4
	local.get	121
	v128.store	896
	local.get	4
	local.get	122
	v128.store	880
	local.get	4
	local.get	123
	v128.store	864
	local.get	4
	local.get	124
	v128.store	848
	local.get	4
	local.get	125
	v128.store	832
	local.get	4
	local.get	126
	v128.store	816
	local.get	4
	local.get	127
	v128.store	800
	local.get	4
	local.get	128
	v128.store	784
	local.get	4
	local.get	129
	v128.store	768
	local.get	4
	local.get	130
	v128.store	752
	local.get	4
	local.get	133
	v128.store	736
	i32.const	0
	i32.const	4
.Ltmp5449:
	.loc	42 443 13 is_stmt 1
	local.get	8
	i32.const	.Lalloc_5f4e05826d69b5e832dc96c63f325058
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5450:
.LBB28_247:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	120
	v128.store	1072
	local.get	4
	local.get	138
	v128.store	1040
	local.get	4
	local.get	137
	v128.store	976
	local.get	4
	local.get	136
	v128.store	912
	local.get	4
	local.get	121
	v128.store	896
	local.get	4
	local.get	122
	v128.store	880
	local.get	4
	local.get	123
	v128.store	864
	local.get	4
	local.get	124
	v128.store	848
	local.get	4
	local.get	125
	v128.store	832
	local.get	4
	local.get	126
	v128.store	816
	local.get	4
	local.get	127
	v128.store	800
	local.get	4
	local.get	128
	v128.store	784
	local.get	4
	local.get	129
	v128.store	768
	local.get	4
	local.get	130
	v128.store	752
	local.get	4
	local.get	133
	v128.store	736
.Ltmp5451:
	.loc	42 569 13 is_stmt 1
	local.get	9
	local.get	2
	local.get	2
	i32.const	.Lalloc_d37239ff881951c49620cb5a9c000a8e
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5452:
.LBB28_248:
	.loc	42 0 13 is_stmt 0
	end_block
.Ltmp5453:
	.loc	4 551 14 is_stmt 1
	local.get	10
	local.get	94
	local.get	7
	v128.load	0:p2align=2
.Ltmp5454:
	.loc	5 3867 14
	local.tee	41
	f32x4.mul
.Ltmp5455:
	.loc	5 3845 14
	v128.const	0x0p0, 0x0p0, 0x0p0, 0x0p0
.Ltmp5456:
	.loc	5 3845 14 is_stmt 0
	local.tee	100
	f32x4.add
.Ltmp5457:
	.loc	5 3867 14 is_stmt 1
	local.get	90
	local.get	22
	f32x4.mul
.Ltmp5458:
	.loc	5 3845 14
	f32x4.add
.Ltmp5459:
	.loc	5 3867 14
	local.get	86
	local.get	37
	f32x4.mul
.Ltmp5460:
	.loc	5 3845 14
	f32x4.add
.Ltmp5461:
	.loc	5 3867 14
	local.get	82
	local.get	33
	f32x4.mul
.Ltmp5462:
	.loc	5 3845 14
	f32x4.add
.Ltmp5463:
	.loc	5 3867 14
	local.get	78
	local.get	34
	f32x4.mul
.Ltmp5464:
	.loc	5 3845 14
	f32x4.add
.Ltmp5465:
	.loc	5 3867 14
	local.get	74
	local.get	35
	f32x4.mul
.Ltmp5466:
	.loc	5 3845 14
	f32x4.add
.Ltmp5467:
	.loc	5 3867 14
	local.get	70
	local.get	21
	f32x4.mul
.Ltmp5468:
	.loc	5 3845 14
	f32x4.add
.Ltmp5469:
	.loc	5 3867 14
	local.get	66
	local.get	36
	f32x4.mul
.Ltmp5470:
	.loc	5 3845 14
	f32x4.add
.Ltmp5471:
	.loc	5 3867 14
	local.get	62
	local.get	38
	f32x4.mul
.Ltmp5472:
	.loc	5 3845 14
	f32x4.add
.Ltmp5473:
	.loc	5 3867 14
	local.get	23
	local.get	39
	f32x4.mul
.Ltmp5474:
	.loc	5 3845 14
	f32x4.add
.Ltmp5475:
	.loc	5 3867 14
	local.get	56
	local.get	40
	f32x4.mul
.Ltmp5476:
	.loc	5 3845 14
	f32x4.add
.Ltmp5477:
	.loc	5 3867 14
	local.get	52
	local.get	42
	f32x4.mul
.Ltmp5478:
	.loc	5 3845 14
	f32x4.add
.Ltmp5479:
	.loc	5 3812 14
	f32x4.abs
.Ltmp5480:
	.loc	5 3867 14
	local.get	95
	local.get	41
	f32x4.mul
.Ltmp5481:
	.loc	5 3845 14
	local.get	100
	f32x4.add
.Ltmp5482:
	.loc	5 3867 14
	local.get	91
	local.get	22
	f32x4.mul
.Ltmp5483:
	.loc	5 3845 14
	f32x4.add
.Ltmp5484:
	.loc	5 3867 14
	local.get	87
	local.get	37
	f32x4.mul
.Ltmp5485:
	.loc	5 3845 14
	f32x4.add
.Ltmp5486:
	.loc	5 3867 14
	local.get	83
	local.get	33
	f32x4.mul
.Ltmp5487:
	.loc	5 3845 14
	f32x4.add
.Ltmp5488:
	.loc	5 3867 14
	local.get	79
	local.get	34
	f32x4.mul
.Ltmp5489:
	.loc	5 3845 14
	f32x4.add
.Ltmp5490:
	.loc	5 3867 14
	local.get	75
	local.get	35
	f32x4.mul
.Ltmp5491:
	.loc	5 3845 14
	f32x4.add
.Ltmp5492:
	.loc	5 3867 14
	local.get	71
	local.get	21
	f32x4.mul
.Ltmp5493:
	.loc	5 3845 14
	f32x4.add
.Ltmp5494:
	.loc	5 3867 14
	local.get	67
	local.get	36
	f32x4.mul
.Ltmp5495:
	.loc	5 3845 14
	f32x4.add
.Ltmp5496:
	.loc	5 3867 14
	local.get	63
	local.get	38
	f32x4.mul
.Ltmp5497:
	.loc	5 3845 14
	f32x4.add
.Ltmp5498:
	.loc	5 3867 14
	local.get	59
	local.get	39
	f32x4.mul
.Ltmp5499:
	.loc	5 3845 14
	f32x4.add
.Ltmp5500:
	.loc	5 3867 14
	local.get	57
	local.get	40
	f32x4.mul
.Ltmp5501:
	.loc	5 3845 14
	f32x4.add
.Ltmp5502:
	.loc	5 3867 14
	local.get	53
	local.get	42
	f32x4.mul
.Ltmp5503:
	.loc	5 3845 14
	f32x4.add
.Ltmp5504:
	.loc	5 3812 14
	f32x4.abs
.Ltmp5505:
	.loc	5 3867 14
	local.get	96
	local.get	41
	f32x4.mul
.Ltmp5506:
	.loc	5 3845 14
	local.get	100
	f32x4.add
.Ltmp5507:
	.loc	5 3867 14
	local.get	92
	local.get	22
	f32x4.mul
.Ltmp5508:
	.loc	5 3845 14
	f32x4.add
.Ltmp5509:
	.loc	5 3867 14
	local.get	88
	local.get	37
	f32x4.mul
.Ltmp5510:
	.loc	5 3845 14
	f32x4.add
.Ltmp5511:
	.loc	5 3867 14
	local.get	84
	local.get	33
	f32x4.mul
.Ltmp5512:
	.loc	5 3845 14
	f32x4.add
.Ltmp5513:
	.loc	5 3867 14
	local.get	80
	local.get	34
	f32x4.mul
.Ltmp5514:
	.loc	5 3845 14
	f32x4.add
.Ltmp5515:
	.loc	5 3867 14
	local.get	76
	local.get	35
	f32x4.mul
.Ltmp5516:
	.loc	5 3845 14
	f32x4.add
.Ltmp5517:
	.loc	5 3867 14
	local.get	72
	local.get	21
	f32x4.mul
.Ltmp5518:
	.loc	5 3845 14
	f32x4.add
.Ltmp5519:
	.loc	5 3867 14
	local.get	68
	local.get	36
	f32x4.mul
.Ltmp5520:
	.loc	5 3845 14
	f32x4.add
.Ltmp5521:
	.loc	5 3867 14
	local.get	64
	local.get	38
	f32x4.mul
.Ltmp5522:
	.loc	5 3845 14
	f32x4.add
.Ltmp5523:
	.loc	5 3867 14
	local.get	60
	local.get	39
	f32x4.mul
.Ltmp5524:
	.loc	5 3845 14
	f32x4.add
.Ltmp5525:
	.loc	5 3867 14
	local.get	24
	local.get	40
	f32x4.mul
.Ltmp5526:
	.loc	5 3845 14
	f32x4.add
.Ltmp5527:
	.loc	5 3867 14
	local.get	54
	local.get	42
	f32x4.mul
.Ltmp5528:
	.loc	5 3845 14
	f32x4.add
.Ltmp5529:
	.loc	5 3812 14
	f32x4.abs
.Ltmp5530:
	.loc	5 3867 14
	local.get	97
	local.get	41
	f32x4.mul
.Ltmp5531:
	.loc	5 3845 14
	local.get	100
	f32x4.add
.Ltmp5532:
	.loc	5 3867 14
	local.get	93
	local.get	22
	f32x4.mul
.Ltmp5533:
	.loc	5 3845 14
	f32x4.add
.Ltmp5534:
	.loc	5 3867 14
	local.get	89
	local.get	37
	f32x4.mul
.Ltmp5535:
	.loc	5 3845 14
	f32x4.add
.Ltmp5536:
	.loc	5 3867 14
	local.get	85
	local.get	33
	f32x4.mul
.Ltmp5537:
	.loc	5 3845 14
	f32x4.add
.Ltmp5538:
	.loc	5 3867 14
	local.get	81
	local.get	34
	f32x4.mul
.Ltmp5539:
	.loc	5 3845 14
	f32x4.add
.Ltmp5540:
	.loc	5 3867 14
	local.get	77
	local.get	35
	f32x4.mul
.Ltmp5541:
	.loc	5 3845 14
	f32x4.add
.Ltmp5542:
	.loc	5 3867 14
	local.get	73
	local.get	21
	f32x4.mul
.Ltmp5543:
	.loc	5 3845 14
	f32x4.add
.Ltmp5544:
	.loc	5 3867 14
	local.get	69
	local.get	36
	f32x4.mul
.Ltmp5545:
	.loc	5 3845 14
	f32x4.add
.Ltmp5546:
	.loc	5 3867 14
	local.get	65
	local.get	38
	f32x4.mul
.Ltmp5547:
	.loc	5 3845 14
	f32x4.add
.Ltmp5548:
	.loc	5 3867 14
	local.get	61
	local.get	39
	f32x4.mul
.Ltmp5549:
	.loc	5 3845 14
	f32x4.add
.Ltmp5550:
	.loc	5 3867 14
	local.get	58
	local.get	40
	f32x4.mul
.Ltmp5551:
	.loc	5 3845 14
	f32x4.add
.Ltmp5552:
	.loc	5 3867 14
	local.get	55
	local.get	42
	f32x4.mul
.Ltmp5553:
	.loc	5 3845 14
	f32x4.add
.Ltmp5554:
	.loc	5 3812 14
	f32x4.abs
.Ltmp5555:
	.loc	5 3812 14 is_stmt 0
	local.get	21
	f32x4.abs
.Ltmp5556:
	.loc	5 3928 9 is_stmt 1
	f32x4.pmax
	f32x4.pmax
	f32x4.pmax
	f32x4.pmax
.Ltmp5557:
	.loc	4 551 14
	v128.store	0:p2align=2
	local.get	7
	i32.const	16
.Ltmp5558:
	.loc	17 1916 50
	i32.add 
	local.set	7
	local.get	8
	i32.const	-4
	i32.add 
	local.set	8
	local.get	9
	i32.const	4
	i32.add 
	local.set	9
	local.get	10
	i32.const	16
	i32.add 
	local.set	10
	local.get	40
	local.set	100
	local.get	39
	local.set	101
	local.get	38
	local.set	102
	local.get	36
	local.set	103
	local.get	21
	local.set	104
	local.get	35
	local.set	105
	local.get	34
	local.set	32
	local.get	33
	local.set	43
	local.get	37
	local.set	106
	local.get	22
	local.set	107
	local.get	11
	i32.const	-1
	i32.add 
.Ltmp5559:
	.loc	18 900 12
	local.tee	11
	br_if   	0
.LBB28_249:
	end_loop
	end_block
.Ltmp5560:
	.loc	22 0 0 is_stmt 0
	local.get	42
	local.set	136
	local.get	40
	local.set	121
	local.get	39
	local.set	122
	local.get	38
	local.set	123
	local.get	36
	local.set	124
	local.get	21
	local.set	125
	local.get	35
	local.set	126
	local.get	34
	local.set	127
	local.get	33
	local.set	128
	local.get	37
	local.set	129
	local.get	22
	local.set	130
	local.get	41
	local.set	133
.Ltmp5561:
	.loc	18 900 12
	block   	
	block   	
	block   	
	local.get	12
	i32.eqz
	br_if   	0
	.loc	18 0 12
	local.get	120
	local.set	33
	local.get	138
	local.set	37
	local.get	137
	local.set	22
	.loc	18 900 12
	br      	1
.Ltmp5562:
.LBB28_251:
	.loc	18 0 12
	end_block
	local.get	0
	i32.load	920
	local.set	47
	local.get	0
	i32.load	916
	local.set	11
	i32.const	0
	local.set	112
	local.get	120
	local.set	33
	local.get	138
	local.set	37
	local.get	137
	local.set	22
.LBB28_252:
.Ltmp5563:
	.loc	22 853 61 is_stmt 1
	loop    	
	local.get	4
	local.get	4
	v128.load	960
.Ltmp5564:
	.loc	5 2188 14
	local.tee	21
	v128.const	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
.Ltmp5565:
	.loc	5 3856 14
	local.tee	34
	local.get	22
	v128.const	-0x1p0, -0x1p0, -0x1p0, -0x1p0
	local.tee	35
	f32x4.add
.Ltmp5566:
	.loc	5 3929 13
	local.tee	39
	v128.const	0x0p0, 0x0p0, 0x0p0, 0x0p0
	local.tee	36
	f32x4.gt
.Ltmp5567:
	.loc	5 2188 14
	local.tee	22
	v128.bitselect
.Ltmp5568:
	.loc	22 854 9
	v128.store	960
	.loc	22 853 44
	local.get	4
	local.get	21
	local.get	4
	v128.load	928
.Ltmp5569:
	.loc	5 3845 14
	f32x4.add
.Ltmp5570:
	.loc	5 2188 14
	local.get	99
	local.get	22
	v128.bitselect
.Ltmp5571:
	.loc	22 853 9
	local.tee	21
	v128.store	928
.Ltmp5572:
	.loc	22 853 44 is_stmt 0
	local.get	4
	local.get	4
	v128.load	992
	.loc	22 853 61
	local.get	4
	v128.load	1024
.Ltmp5573:
	.loc	5 3845 14 is_stmt 1
	local.tee	38
	f32x4.add
.Ltmp5574:
	.loc	5 3856 14
	local.get	98
	local.get	37
	local.get	35
	f32x4.add
.Ltmp5575:
	.loc	5 3929 13
	local.tee	40
	local.get	36
	f32x4.gt
.Ltmp5576:
	.loc	5 2188 14
	local.tee	37
	v128.bitselect
.Ltmp5577:
	.loc	22 853 9
	local.tee	35
	v128.store	992
.Ltmp5578:
	.loc	5 2188 14
	local.get	4
	local.get	38
	local.get	34
	local.get	37
	v128.bitselect
.Ltmp5579:
	.loc	22 854 9
	v128.store	1024
.Ltmp5580:
	.loc	22 3276 24
	block   	
	block   	
	block   	
	local.get	2
	local.get	112
	local.get	19
	i32.add 
	i32.const	2
	i32.shl 
.Ltmp5581:
	.loc	42 568 12
	local.tee	9
	i32.lt_u
	br_if   	0
	.loc	42 573 27
	block   	
	local.get	2
	local.get	9
	i32.sub 
	local.tee	8
	i32.const	3
.Ltmp5582:
	.loc	42 438 16
	i32.le_u
	br_if   	0
.Ltmp5583:
	.loc	22 1392 25
	local.get	0
	i32.load	944
.Ltmp5584:
	.loc	22 1388 17
	local.tee	8
	local.get	0
	i32.load	1020
.Ltmp5585:
	.loc	22 1392 45
	local.tee	26
	local.get	17
	i32.mul 
.Ltmp5586:
	.loc	42 580 12
	local.tee	25
	i32.lt_u
	br_if   	15
	.loc	42 585 27
	block   	
	local.get	8
	local.get	25
	i32.sub 
	local.tee	8
	i32.const	3
.Ltmp5587:
	.loc	42 451 16
	i32.le_u
	br_if   	0
.Ltmp5588:
	.loc	42 0 16 is_stmt 0
	local.get	1
	local.get	9
	i32.const	2
	i32.shl 
	i32.add 
	local.tee	48
	v128.load	0:p2align=2
	local.set	36
.Ltmp5589:
	.loc	22 1392 25 is_stmt 1
	local.get	0
	i32.load	940
	local.get	25
	i32.const	2
.Ltmp5590:
	.loc	42 101 24
	i32.shl 
	local.tee	113
	i32.add 
.Ltmp5591:
	.loc	22 0 0 is_stmt 0
	local.get	21
	local.get	4
	i32.const	1136
	i32.add 
	local.get	112
	i32.const	4
	i32.shl 
	i32.add 
	v128.load	0:p2align=2
.Ltmp5592:
	local.tee	34
	local.get	34
	local.get	131
	v128.bitselect
.Ltmp5593:
	local.tee	34
	f32x4.div
	v128.const	0, 0, 128, 63, 0, 0, 128, 63, 0, 0, 128, 63, 0, 0, 128, 63
	local.get	34
	local.get	21
	f32x4.gt
	v128.bitselect
.Ltmp5594:
	.loc	4 551 14 is_stmt 1
	v128.store	0:p2align=2
.Ltmp5595:
	.loc	22 1259 17
	block   	
	block   	
	block   	
	local.get	0
	i32.load	1020
.Ltmp5596:
	.loc	45 37 12
	local.tee	6
	i32.eqz
	br_if   	0
	.loc	45 0 12 is_stmt 0
	i32.const	0
	local.set	51
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
	local.set	117
	local.get	0
	i32.load	964
	local.set	29
	local.get	0
	i32.load	968
	local.set	118
	local.get	0
	i32.load	980
	local.set	116
	local.get	0
	i32.load	984
	local.set	110
	local.get	0
	i32.load	940
	local.set	13
	local.get	0
	i32.load	944
	local.set	12
	local.get	0
	i32.load	1012
	local.set	115
	local.get	0
	i32.load	1016
	local.set	111
	i32.const	0
	local.set	7
	local.get	6
	local.set	50
.LBB28_258:
	loop    	
	local.get	51
	i32.const	32
.Ltmp5597:
	.loc	24 1714 9 is_stmt 1
	i32.eq  
.Ltmp5598:
	.loc	23 180 28
	br_if   	1
.Ltmp5599:
	.loc	22 1263 21
	block   	
	block   	
	block   	
	block   	
	local.get	7
	local.get	111
	i32.eq  
	br_if   	0
	.loc	22 0 21 is_stmt 0
	local.get	115
	local.get	7
	i32.const	12
	.loc	22 1263 21
	i32.mul 
	i32.add 
	local.tee	8
	i32.load	4
.Ltmp5600:
	.loc	22 1265 23 is_stmt 1
	local.get	17
	i32.add 
	local.tee	9
	i32.const	0
.Ltmp5601:
	.loc	22 1266 12
	local.get	11
	local.get	9
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
.Ltmp5602:
	.loc	22 1273 42
	local.tee	9
	local.get	6
	i32.mul 
	local.get	7
	i32.add 
	.loc	22 1273 22 is_stmt 0
	local.tee	10
	local.get	12
	i32.ge_u
	br_if   	1
.Ltmp5603:
	.loc	22 1274 24 is_stmt 1
	local.get	7
	local.get	110
	i32.eq  
	br_if   	2
.Ltmp5604:
	.loc	22 0 0 is_stmt 0
	local.get	8
	i32.load	0
	local.set	8
	local.get	13
	local.get	10
	i32.const	2
.Ltmp5605:
	i32.shl 
	i32.add 
	local.tee	18
	f32.load	0
	local.set	134
	local.get	116
	local.get	7
	i32.const	2
.Ltmp5606:
	.loc	22 1274 24
	i32.shl 
	local.tee	10
	i32.add 
	local.tee	114
	i32.load	0
.Ltmp5607:
	.loc	22 1275 26 is_stmt 1
	local.tee	27
	i32.eqz
	br_if   	3
	.loc	22 1278 24
	block   	
	block   	
	local.get	7
	local.get	118
	i32.ge_u
	br_if   	0
	local.get	29
	local.get	10
	i32.add 
	f32.load	0
.Ltmp5608:
	.loc	22 903 8
	local.tee	135
	local.get	134
	f32.lt  
	br_if   	1
	br      	5
.Ltmp5609:
.LBB28_265:
	.loc	22 0 8 is_stmt 0
	end_block
	local.get	4
	local.get	120
	v128.store	1072
	local.get	4
	local.get	138
	v128.store	1040
	local.get	4
	local.get	137
	v128.store	976
.Ltmp5610:
	local.get	4
	local.get	136
	v128.store	912
	local.get	4
	local.get	121
	v128.store	896
	local.get	4
	local.get	122
	v128.store	880
	local.get	4
	local.get	123
	v128.store	864
	local.get	4
	local.get	124
	v128.store	848
	local.get	4
	local.get	125
	v128.store	832
	local.get	4
	local.get	126
	v128.store	816
	local.get	4
	local.get	127
	v128.store	800
	local.get	4
	local.get	128
	v128.store	784
	local.get	4
	local.get	129
	v128.store	768
	local.get	4
	local.get	130
	v128.store	752
	local.get	4
	local.get	133
	v128.store	736
.Ltmp5611:
	.loc	22 1278 24 is_stmt 1
	local.get	7
	local.get	118
	i32.const	.Lalloc_a228cac3279aae3a34886f59f51fbe9e
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.LBB28_266:
	.loc	22 0 24 is_stmt 0
	end_block
	local.get	135
	local.set	134
.Ltmp5612:
	.loc	22 903 5 is_stmt 1
	br      	3
.Ltmp5613:
.LBB28_267:
	.loc	22 0 5 is_stmt 0
	end_block
	local.get	4
	local.get	120
	v128.store	1072
	local.get	4
	local.get	138
	v128.store	1040
	local.get	4
	local.get	137
	v128.store	976
.Ltmp5614:
	local.get	4
	local.get	136
	v128.store	912
	local.get	4
	local.get	121
	v128.store	896
	local.get	4
	local.get	122
	v128.store	880
	local.get	4
	local.get	123
	v128.store	864
	local.get	4
	local.get	124
	v128.store	848
	local.get	4
	local.get	125
	v128.store	832
	local.get	4
	local.get	126
	v128.store	816
	local.get	4
	local.get	127
	v128.store	800
	local.get	4
	local.get	128
	v128.store	784
	local.get	4
	local.get	129
	v128.store	768
	local.get	4
	local.get	130
	v128.store	752
	local.get	4
	local.get	133
	v128.store	736
.Ltmp5615:
	.loc	22 1263 21 is_stmt 1
	local.get	111
	local.get	111
	i32.const	.Lalloc_12856b5f9033764dbf23e04eb77c5c46
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.Ltmp5616:
.LBB28_268:
	.loc	22 0 21 is_stmt 0
	end_block
	local.get	4
	local.get	120
	v128.store	1072
	local.get	4
	local.get	138
	v128.store	1040
	local.get	4
	local.get	137
	v128.store	976
.Ltmp5617:
	local.get	4
	local.get	136
	v128.store	912
	local.get	4
	local.get	121
	v128.store	896
	local.get	4
	local.get	122
	v128.store	880
	local.get	4
	local.get	123
	v128.store	864
	local.get	4
	local.get	124
	v128.store	848
	local.get	4
	local.get	125
	v128.store	832
	local.get	4
	local.get	126
	v128.store	816
	local.get	4
	local.get	127
	v128.store	800
	local.get	4
	local.get	128
	v128.store	784
	local.get	4
	local.get	129
	v128.store	768
	local.get	4
	local.get	130
	v128.store	752
	local.get	4
	local.get	133
	v128.store	736
.Ltmp5618:
	.loc	22 1273 22 is_stmt 1
	local.get	10
	local.get	12
	i32.const	.Lalloc_70ebf0cf1c90c6161926611ccf249a32
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.Ltmp5619:
.LBB28_269:
	.loc	22 0 22 is_stmt 0
	end_block
	local.get	4
	local.get	120
	v128.store	1072
	local.get	4
	local.get	138
	v128.store	1040
	local.get	4
	local.get	137
	v128.store	976
.Ltmp5620:
	local.get	4
	local.get	136
	v128.store	912
	local.get	4
	local.get	121
	v128.store	896
	local.get	4
	local.get	122
	v128.store	880
	local.get	4
	local.get	123
	v128.store	864
	local.get	4
	local.get	124
	v128.store	848
	local.get	4
	local.get	125
	v128.store	832
	local.get	4
	local.get	126
	v128.store	816
	local.get	4
	local.get	127
	v128.store	800
	local.get	4
	local.get	128
	v128.store	784
	local.get	4
	local.get	129
	v128.store	768
	local.get	4
	local.get	130
	v128.store	752
	local.get	4
	local.get	133
	v128.store	736
.Ltmp5621:
	.loc	22 1274 24 is_stmt 1
	local.get	110
	local.get	110
	i32.const	.Lalloc_023d591aaad7f5cf482ac80a9401eca1
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.LBB28_270:
	.loc	22 0 24 is_stmt 0
	end_block
.Ltmp5622:
	.loc	22 1280 9 is_stmt 1
	block   	
	block   	
	block   	
	local.get	7
	local.get	118
	i32.eq  
	br_if   	0
.Ltmp5623:
	.loc	22 0 0 is_stmt 0
	local.get	4
	i32.const	1104
	i32.add 
	local.get	51
	i32.add 
	local.set	28
.Ltmp5624:
	.loc	22 1280 9
	local.get	29
	local.get	10
	i32.add 
	local.get	134
	f32.store	0
	local.get	27
	i32.const	1
	.loc	22 1281 24 is_stmt 1
	i32.add 
	local.tee	27
	local.get	8
	i32.ne  
.Ltmp5625:
	.loc	22 1282 23
	br_if   	1
	.loc	22 1282 9 is_stmt 0
	local.get	28
	local.get	134
	f32.store	0
	i32.const	0
	local.set	27
	local.get	8
	i32.eqz
	br_if   	2
	.loc	22 1288 30 is_stmt 1
	local.get	18
	f32.load	0
	local.set	134
.LBB28_274:
.Ltmp5626:
	.loc	22 1291 65
	loop    	
	local.get	9
	local.get	6
	i32.mul 
	local.get	7
	i32.add 
	.loc	22 1291 45 is_stmt 0
	local.tee	10
	local.get	12
	i32.ge_u
	br_if   	7
	.loc	22 0 45
	local.get	13
	local.get	10
	i32.const	2
	.loc	22 1291 45
	i32.shl 
	i32.add 
	local.tee	10
	local.get	134
	local.get	10
	f32.load	0
.Ltmp5627:
	.loc	22 903 8 is_stmt 1
	local.tee	135
	local.get	134
	local.get	135
	f32.lt  
	f32.select
.Ltmp5628:
	.loc	22 1292 17
	local.tee	134
	f32.store	0
	.loc	22 1293 20
	local.get	9
	local.get	11
	local.get	9
	i32.select
	i32.const	-1
	.loc	22 1296 17
	i32.add 
	local.set	9
	local.get	8
	i32.const	-1
.Ltmp5629:
	.loc	17 1916 50
	i32.add 
.Ltmp5630:
	.loc	18 900 12
	local.tee	8
	br_if   	0
	br      	3
.Ltmp5631:
.LBB28_276:
	.loc	22 1280 9
	end_loop
	end_block
	local.get	4
	local.get	120
	v128.store	1072
	local.get	4
	local.get	138
	v128.store	1040
	local.get	4
	local.get	137
	v128.store	976
.Ltmp5632:
	.loc	22 0 0 is_stmt 0
	local.get	4
	local.get	136
	v128.store	912
	local.get	4
	local.get	121
	v128.store	896
	local.get	4
	local.get	122
	v128.store	880
	local.get	4
	local.get	123
	v128.store	864
	local.get	4
	local.get	124
	v128.store	848
	local.get	4
	local.get	125
	v128.store	832
	local.get	4
	local.get	126
	v128.store	816
	local.get	4
	local.get	127
	v128.store	800
	local.get	4
	local.get	128
	v128.store	784
	local.get	4
	local.get	129
	v128.store	768
	local.get	4
	local.get	130
	v128.store	752
	local.get	4
	local.get	133
	v128.store	736
.Ltmp5633:
	.loc	22 1280 9
	local.get	118
	local.get	118
	i32.const	.Lalloc_1fa5d88c1a2cc292a2767c48606a38fe
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.LBB28_277:
	.loc	22 0 9
	end_block
.Ltmp5634:
	.loc	22 1285 44 is_stmt 1
	local.get	7
	local.get	117
	i32.add 
	.loc	22 1285 24 is_stmt 0
	local.tee	9
	local.get	12
	i32.ge_u
	br_if   	3
	.loc	22 0 24
	local.get	28
	local.get	13
	local.get	9
	i32.const	2
	.loc	22 1285 24
	i32.shl 
	i32.add 
	f32.load	0
.Ltmp5635:
	.loc	22 903 8 is_stmt 1
	local.tee	135
	local.get	134
	local.get	135
	local.get	134
	f32.lt  
	f32.select
.Ltmp5636:
	.loc	22 1282 9
	f32.store	0
.Ltmp5637:
.LBB28_279:
	.loc	22 0 9 is_stmt 0
	end_block
	local.get	7
	i32.const	1
	i32.add 
	local.set	7
	local.get	51
	i32.const	4
	i32.add 
	local.set	51
.Ltmp5638:
	local.get	114
	local.get	27
	i32.store	0
	local.get	50
	i32.const	-1
.Ltmp5639:
	i32.add 
.Ltmp5640:
	.loc	45 37 12 is_stmt 1
	local.tee	50
	br_if   	0
.LBB28_280:
	.loc	45 37 12
	end_loop
	end_block
.Ltmp5641:
	.loc	22 1412 26
	local.get	0
	i32.load	952
	local.set	9
.Ltmp5642:
	.loc	4 551 14
	local.get	4
	v128.load	1104:p2align=3
	local.tee	34
	local.set	21
	local.get	26
	i32.eqz
	br_if   	6
.Ltmp5643:
	.loc	4 0 14 is_stmt 0
	block   	
	local.get	0
	i32.load	1016
	local.tee	10
	br_if   	0
	i32.const	0
	local.set	8
	br      	20
.LBB28_283:
	end_block
	block   	
	local.get	0
	i32.load	1012
.Ltmp5644:
	.loc	22 1403 42 is_stmt 1
	local.tee	7
	i32.load	8
	.loc	22 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5645:
	.loc	22 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	22 1407 40
	local.get	26
	i32.mul 
	.loc	22 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	22 0 25
	local.get	4
	local.get	0
	i32.load	948
	local.tee	12
	local.get	8
	i32.const	2
	.loc	22 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	22 1407 13
	f32.store	1104
	local.get	26
	i32.const	1
.Ltmp5646:
	.loc	45 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5647:
	.loc	45 0 12 is_stmt 0
	i32.const	1
	local.set	8
	local.get	10
	i32.const	1
.Ltmp5648:
	.loc	22 1403 42 is_stmt 1
	i32.eq  
	br_if   	20
	local.get	7
	i32.load	20
	.loc	22 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5649:
	.loc	22 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	22 1407 40
	local.get	26
	i32.mul 
	i32.const	1
	i32.add 
	.loc	22 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	22 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	22 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	22 1407 13
	f32.store	1108
	local.get	26
	i32.const	2
.Ltmp5650:
	.loc	45 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5651:
	.loc	45 0 12 is_stmt 0
	i32.const	2
	local.set	8
	local.get	10
	i32.const	2
.Ltmp5652:
	.loc	22 1403 42 is_stmt 1
	i32.eq  
	br_if   	20
	local.get	7
	i32.load	32
	.loc	22 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5653:
	.loc	22 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	22 1407 40
	local.get	26
	i32.mul 
	i32.const	2
	i32.add 
	.loc	22 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	22 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	22 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	22 1407 13
	f32.store	1112
	local.get	26
	i32.const	3
.Ltmp5654:
	.loc	45 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5655:
	.loc	45 0 12 is_stmt 0
	i32.const	3
	local.set	8
	local.get	10
	i32.const	3
.Ltmp5656:
	.loc	22 1403 42 is_stmt 1
	i32.eq  
	br_if   	20
	local.get	7
	i32.load	44
	.loc	22 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5657:
	.loc	22 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	22 1407 40
	local.get	26
	i32.mul 
	i32.const	3
	i32.add 
	.loc	22 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	22 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	22 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	22 1407 13
	f32.store	1116
	local.get	26
	i32.const	4
.Ltmp5658:
	.loc	45 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5659:
	.loc	45 0 12 is_stmt 0
	i32.const	4
	local.set	8
	local.get	10
	i32.const	4
.Ltmp5660:
	.loc	22 1403 42 is_stmt 1
	i32.eq  
	br_if   	20
	local.get	7
	i32.load	56
	.loc	22 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5661:
	.loc	22 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	22 1407 40
	local.get	26
	i32.mul 
	i32.const	4
	i32.add 
	.loc	22 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	22 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	22 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	22 1407 13
	f32.store	1120
	local.get	26
	i32.const	5
.Ltmp5662:
	.loc	45 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5663:
	.loc	45 0 12 is_stmt 0
	i32.const	5
	local.set	8
	local.get	10
	i32.const	5
.Ltmp5664:
	.loc	22 1403 42 is_stmt 1
	i32.eq  
	br_if   	20
	local.get	7
	i32.load	68
	.loc	22 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5665:
	.loc	22 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	22 1407 40
	local.get	26
	i32.mul 
	i32.const	5
	i32.add 
	.loc	22 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	22 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	22 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	22 1407 13
	f32.store	1124
	local.get	26
	i32.const	6
.Ltmp5666:
	.loc	45 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5667:
	.loc	45 0 12 is_stmt 0
	i32.const	6
	local.set	8
	local.get	10
	i32.const	6
.Ltmp5668:
	.loc	22 1403 42 is_stmt 1
	i32.eq  
	br_if   	20
	local.get	7
	i32.load	80
	.loc	22 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5669:
	.loc	22 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	22 1407 40
	local.get	26
	i32.mul 
	i32.const	6
	i32.add 
	.loc	22 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	22 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	22 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	22 1407 13
	f32.store	1128
	local.get	26
	i32.const	7
.Ltmp5670:
	.loc	45 37 12 is_stmt 1
	i32.eq  
	br_if   	6
.Ltmp5671:
	.loc	45 0 12 is_stmt 0
	i32.const	7
	local.set	8
	local.get	10
	i32.const	7
.Ltmp5672:
	.loc	22 1403 42 is_stmt 1
	i32.eq  
	br_if   	20
	local.get	7
	i32.load	92
	.loc	22 1403 28 is_stmt 0
	local.get	17
	i32.add 
	local.tee	8
	i32.const	0
.Ltmp5673:
	.loc	22 1404 16 is_stmt 1
	local.get	11
	local.get	8
	local.get	11
	i32.lt_u
	i32.select
	i32.sub 
	.loc	22 1407 40
	local.get	26
	i32.mul 
	i32.const	7
	i32.add 
	.loc	22 1407 25 is_stmt 0
	local.tee	8
	local.get	9
	i32.ge_u
	br_if   	0
	.loc	22 0 25
	local.get	4
	local.get	12
	local.get	8
	i32.const	2
	.loc	22 1407 25
	i32.shl 
	i32.add 
	f32.load	0
	.loc	22 1407 13
	f32.store	1132
.Ltmp5674:
	.loc	45 37 12 is_stmt 1
	br      	6
.Ltmp5675:
.LBB28_306:
	.loc	45 0 12 is_stmt 0
	end_block
	local.get	4
	local.get	120
	v128.store	1072
	local.get	4
	local.get	138
	v128.store	1040
	local.get	4
	local.get	137
	v128.store	976
.Ltmp5676:
	local.get	4
	local.get	136
	v128.store	912
	local.get	4
	local.get	121
	v128.store	896
	local.get	4
	local.get	122
	v128.store	880
	local.get	4
	local.get	123
	v128.store	864
	local.get	4
	local.get	124
	v128.store	848
	local.get	4
	local.get	125
	v128.store	832
	local.get	4
	local.get	126
	v128.store	816
	local.get	4
	local.get	127
	v128.store	800
	local.get	4
	local.get	128
	v128.store	784
	local.get	4
	local.get	129
	v128.store	768
	local.get	4
	local.get	130
	v128.store	752
	local.get	4
	local.get	133
	v128.store	736
.Ltmp5677:
	.loc	22 1407 25 is_stmt 1
	local.get	8
	local.get	9
	i32.const	.Lalloc_0e76a26fbb751dfb8cf8a069b5b0d39a
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.Ltmp5678:
.LBB28_307:
	.loc	22 0 25 is_stmt 0
	end_block
	local.get	4
	local.get	120
	v128.store	1072
	local.get	4
	local.get	138
	v128.store	1040
	local.get	4
	local.get	137
	v128.store	976
.Ltmp5679:
	local.get	4
	local.get	136
	v128.store	912
	local.get	4
	local.get	121
	v128.store	896
	local.get	4
	local.get	122
	v128.store	880
	local.get	4
	local.get	123
	v128.store	864
	local.get	4
	local.get	124
	v128.store	848
	local.get	4
	local.get	125
	v128.store	832
	local.get	4
	local.get	126
	v128.store	816
	local.get	4
	local.get	127
	v128.store	800
	local.get	4
	local.get	128
	v128.store	784
	local.get	4
	local.get	129
	v128.store	768
	local.get	4
	local.get	130
	v128.store	752
	local.get	4
	local.get	133
	v128.store	736
.Ltmp5680:
	.loc	22 1285 24 is_stmt 1
	local.get	9
	local.get	12
	i32.const	.Lalloc_70d40ae2302f3ea19fa0c4a65fe82786
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.Ltmp5681:
.LBB28_308:
	.loc	22 0 24 is_stmt 0
	end_block
	local.get	4
	local.get	120
	v128.store	1072
	local.get	4
	local.get	138
	v128.store	1040
	local.get	4
	local.get	137
	v128.store	976
.Ltmp5682:
	local.get	4
	local.get	136
	v128.store	912
	local.get	4
	local.get	121
	v128.store	896
	local.get	4
	local.get	122
	v128.store	880
	local.get	4
	local.get	123
	v128.store	864
	local.get	4
	local.get	124
	v128.store	848
	local.get	4
	local.get	125
	v128.store	832
	local.get	4
	local.get	126
	v128.store	816
	local.get	4
	local.get	127
	v128.store	800
	local.get	4
	local.get	128
	v128.store	784
	local.get	4
	local.get	129
	v128.store	768
	local.get	4
	local.get	130
	v128.store	752
	local.get	4
	local.get	133
	v128.store	736
.Ltmp5683:
	.loc	22 1291 45 is_stmt 1
	local.get	10
	local.get	12
	i32.const	.Lalloc_a271bbb7fd83c3605ea58a06b7065fa4
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
.Ltmp5684:
.LBB28_309:
	.loc	22 0 45 is_stmt 0
	end_block
	local.get	4
	local.get	120
	v128.store	1072
	local.get	4
	local.get	138
	v128.store	1040
	local.get	4
	local.get	137
	v128.store	976
.Ltmp5685:
	local.get	4
	local.get	136
	v128.store	912
	local.get	4
	local.get	121
	v128.store	896
	local.get	4
	local.get	122
	v128.store	880
	local.get	4
	local.get	123
	v128.store	864
	local.get	4
	local.get	124
	v128.store	848
	local.get	4
	local.get	125
	v128.store	832
	local.get	4
	local.get	126
	v128.store	816
	local.get	4
	local.get	127
	v128.store	800
	local.get	4
	local.get	128
	v128.store	784
	local.get	4
	local.get	129
	v128.store	768
	local.get	4
	local.get	130
	v128.store	752
	local.get	4
	local.get	133
	v128.store	736
	i32.const	0
	i32.const	4
.Ltmp5686:
	.loc	42 456 13 is_stmt 1
	local.get	8
	i32.const	.Lalloc_5f4e05826d69b5e832dc96c63f325058
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5687:
.LBB28_310:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	120
	v128.store	1072
	local.get	4
	local.get	138
	v128.store	1040
	local.get	4
	local.get	137
	v128.store	976
.Ltmp5688:
	local.get	4
	local.get	136
	v128.store	912
	local.get	4
	local.get	121
	v128.store	896
	local.get	4
	local.get	122
	v128.store	880
	local.get	4
	local.get	123
	v128.store	864
	local.get	4
	local.get	124
	v128.store	848
	local.get	4
	local.get	125
	v128.store	832
	local.get	4
	local.get	126
	v128.store	816
	local.get	4
	local.get	127
	v128.store	800
	local.get	4
	local.get	128
	v128.store	784
	local.get	4
	local.get	129
	v128.store	768
	local.get	4
	local.get	130
	v128.store	752
	local.get	4
	local.get	133
	v128.store	736
	i32.const	0
	i32.const	4
.Ltmp5689:
	.loc	42 443 13 is_stmt 1
	local.get	8
	i32.const	.Lalloc_5f4e05826d69b5e832dc96c63f325058
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5690:
.LBB28_311:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	120
	v128.store	1072
	local.get	4
	local.get	138
	v128.store	1040
	local.get	4
	local.get	137
	v128.store	976
.Ltmp5691:
	local.get	4
	local.get	136
	v128.store	912
	local.get	4
	local.get	121
	v128.store	896
	local.get	4
	local.get	122
	v128.store	880
	local.get	4
	local.get	123
	v128.store	864
	local.get	4
	local.get	124
	v128.store	848
	local.get	4
	local.get	125
	v128.store	832
	local.get	4
	local.get	126
	v128.store	816
	local.get	4
	local.get	127
	v128.store	800
	local.get	4
	local.get	128
	v128.store	784
	local.get	4
	local.get	129
	v128.store	768
	local.get	4
	local.get	130
	v128.store	752
	local.get	4
	local.get	133
	v128.store	736
.Ltmp5692:
	.loc	42 569 13 is_stmt 1
	local.get	9
	local.get	2
	local.get	2
	i32.const	.Lalloc_8b3aff86bf6965bbce4da27800054d27
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5693:
.LBB28_312:
	.loc	42 0 13 is_stmt 0
	end_block
.Ltmp5694:
	.loc	4 551 14 is_stmt 1
	local.get	4
	v128.load	1104:p2align=3
	local.set	21
.Ltmp5695:
.LBB28_313:
	.loc	4 0 14 is_stmt 0
	end_block
.Ltmp5696:
	.loc	42 580 12 is_stmt 1
	block   	
	local.get	9
	local.get	25
	i32.ge_u
	br_if   	0
.Ltmp5697:
	.loc	42 0 12 is_stmt 0
	local.get	4
	local.get	120
	v128.store	1072
	local.get	4
	local.get	138
	v128.store	1040
	local.get	4
	local.get	137
	v128.store	976
.Ltmp5698:
	local.get	4
	local.get	136
	v128.store	912
	local.get	4
	local.get	121
	v128.store	896
	local.get	4
	local.get	122
	v128.store	880
	local.get	4
	local.get	123
	v128.store	864
	local.get	4
	local.get	124
	v128.store	848
	local.get	4
	local.get	125
	v128.store	832
	local.get	4
	local.get	126
	v128.store	816
	local.get	4
	local.get	127
	v128.store	800
	local.get	4
	local.get	128
	v128.store	784
	local.get	4
	local.get	129
	v128.store	768
	local.get	4
	local.get	130
	v128.store	752
	local.get	4
	local.get	133
	v128.store	736
.Ltmp5699:
	.loc	42 581 13 is_stmt 1
	local.get	25
	local.get	9
	local.get	9
	i32.const	.Lalloc_c2286ff41a33b8c11aa270fe215441de
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.LBB28_315:
	.loc	42 0 13 is_stmt 0
	end_block
	.loc	42 585 27 is_stmt 1
	local.get	9
	local.get	25
	i32.sub 
	local.tee	9
	i32.const	3
.Ltmp5700:
	.loc	42 451 16
	i32.le_u
	br_if   	8
.Ltmp5701:
	.loc	22 1412 26
	local.get	0
	i32.load	948
.Ltmp5702:
	.loc	42 101 24
	local.get	113
	i32.add 
.Ltmp5703:
	.loc	22 0 0 is_stmt 0
	local.get	34
	v128.const	0x1p14, 0x1p14, 0x1p14, 0x1p14
	f32x4.mul
	f32x4.floor
	v128.const	0x1p-14, 0x1p-14, 0x1p-14, 0x1p-14
	f32x4.mul
.Ltmp5704:
	.loc	4 551 14 is_stmt 1
	local.tee	34
	v128.store	0:p2align=2
.Ltmp5705:
	.loc	22 1416 43
	local.get	4
	local.get	4
	v128.load	1056
.Ltmp5706:
	.loc	5 3856 14
	local.tee	38
	v128.const	0x1p0, 0x1p0, 0x1p0, 0x1p0
.Ltmp5707:
	.loc	22 0 0 is_stmt 0
	local.tee	41
	local.get	34
	local.get	33
	f32x4.add
	local.get	21
	f32x4.sub
.Ltmp5708:
	.loc	5 3878 14 is_stmt 1
	local.tee	33
	local.get	132
	f32x4.div
.Ltmp5709:
	.loc	5 3856 14
	f32x4.sub
.Ltmp5710:
	.loc	5 3856 14 is_stmt 0
	local.tee	21
	local.get	38
	f32x4.sub
.Ltmp5711:
	.loc	5 3867 14 is_stmt 1
	local.get	35
	f32x4.mul
.Ltmp5712:
	.loc	5 3845 14
	f32x4.add
.Ltmp5713:
	.loc	5 3928 9
	local.get	21
	f32x4.pmax
.Ltmp5714:
	.loc	5 3812 14
	local.tee	21
	local.get	21
	f32x4.abs
.Ltmp5715:
	.loc	5 1991 14
	v128.const	0x1.79ca1p-67, 0x1.79ca1p-67, 0x1.79ca1p-67, 0x1.79ca1p-67
	f32x4.lt
.Ltmp5716:
	.loc	8 481 22
	v128.andnot
.Ltmp5717:
	.loc	22 1417 5
	local.tee	34
	v128.store	1056
.Ltmp5718:
	.loc	22 1420 28
	local.get	0
	i32.load	936
	.loc	22 1420 44 is_stmt 0
	local.tee	8
	local.get	26
	local.get	109
	i32.mul 
.Ltmp5719:
	.loc	42 568 12 is_stmt 1
	local.tee	9
	i32.lt_u
	br_if   	4
	.loc	42 573 27
	local.get	8
	local.get	9
	i32.sub 
	local.tee	8
	i32.const	3
.Ltmp5720:
	.loc	42 438 16
	i32.le_u
	br_if   	2
.Ltmp5721:
	.loc	22 0 0 is_stmt 0
	local.get	37
	local.get	40
	v128.and
	local.set	37
	local.get	22
	local.get	39
	v128.and
	local.set	22
.Ltmp5722:
	.loc	22 1420 28 is_stmt 1
	local.get	0
	i32.load	932
	local.get	9
	i32.const	2
.Ltmp5723:
	.loc	42 89 24
	i32.shl 
	i32.add 
.Ltmp5724:
	.loc	4 551 14
	local.tee	9
	v128.load	0:p2align=2
	local.set	21
.Ltmp5725:
	.loc	4 551 14 is_stmt 0
	local.get	9
	local.get	36
	v128.store	0:p2align=2
.Ltmp5726:
	.loc	22 0 0
	local.get	48
	local.get	21
	local.get	21
	local.get	41
	local.get	34
	f32x4.sub
.Ltmp5727:
	.loc	5 3867 14 is_stmt 1
	f32x4.mul
.Ltmp5728:
	.loc	5 2188 14
	local.get	119
	v128.bitselect
.Ltmp5729:
	.loc	4 551 14
	v128.store	0:p2align=2
	i32.const	0
	local.get	17
	i32.const	1
.Ltmp5730:
	.loc	22 3304 13
	i32.add 
	.loc	22 3305 16
	local.tee	9
	local.get	9
	local.get	11
	i32.eq  
	i32.select
	local.set	17
	i32.const	0
	local.get	109
	i32.const	1
	.loc	22 3300 13
	i32.add 
	.loc	22 3301 16
	local.tee	9
	local.get	9
	local.get	47
	i32.eq  
	i32.select
	local.set	109
	local.get	112
	i32.const	1
.Ltmp5731:
	.loc	22 0 0 is_stmt 0
	i32.add 
.Ltmp5732:
	.loc	17 1916 50 is_stmt 1
	local.tee	112
	local.get	108
	i32.ne  
.Ltmp5733:
	.loc	18 900 12
	br_if   	0
.LBB28_319:
	end_loop
	end_block
	local.get	19
	i32.const	32
.Ltmp5734:
	.loc	22 0 0 is_stmt 0
	i32.add 
	local.set	19
	local.get	30
	i32.const	512
.Ltmp5735:
	.loc	44 446 20 is_stmt 1
	i32.add 
	local.set	30
	local.get	31
	i32.const	-128
	i32.add 
	local.set	31
	local.get	44
	i32.const	128
	i32.add 
	local.set	44
	local.get	49
	i32.const	-32
	i32.add 
	local.set	49
	local.get	45
	i32.const	-1
.Ltmp5736:
	.loc	22 0 0 is_stmt 0
	i32.add 
.Ltmp5737:
	.loc	44 446 20
	local.tee	45
	i32.eqz
	br_if   	3
	br      	1
.Ltmp5738:
.LBB28_320:
	.loc	44 0 20
	end_block
.Ltmp5739:
	.loc	42 438 16 is_stmt 1
	end_loop
	local.get	4
	local.get	120
	v128.store	1072
	local.get	4
	local.get	138
	v128.store	1040
	local.get	4
	local.get	137
	v128.store	976
.Ltmp5740:
	.loc	22 0 0 is_stmt 0
	local.get	4
	local.get	136
	v128.store	912
	local.get	4
	local.get	121
	v128.store	896
	local.get	4
	local.get	122
	v128.store	880
	local.get	4
	local.get	123
	v128.store	864
	local.get	4
	local.get	124
	v128.store	848
	local.get	4
	local.get	125
	v128.store	832
	local.get	4
	local.get	126
	v128.store	816
	local.get	4
	local.get	127
	v128.store	800
	local.get	4
	local.get	128
	v128.store	784
	local.get	4
	local.get	129
	v128.store	768
	local.get	4
	local.get	130
	v128.store	752
	local.get	4
	local.get	133
	v128.store	736
	i32.const	0
	i32.const	4
.Ltmp5741:
	.loc	42 443 13 is_stmt 1
	local.get	8
	i32.const	.Lalloc_5f4e05826d69b5e832dc96c63f325058
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5742:
.LBB28_321:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	120
	v128.store	1072
	local.get	4
	local.get	138
	v128.store	1040
	local.get	4
	local.get	137
	v128.store	976
.Ltmp5743:
	local.get	4
	local.get	136
	v128.store	912
	local.get	4
	local.get	121
	v128.store	896
	local.get	4
	local.get	122
	v128.store	880
	local.get	4
	local.get	123
	v128.store	864
	local.get	4
	local.get	124
	v128.store	848
	local.get	4
	local.get	125
	v128.store	832
	local.get	4
	local.get	126
	v128.store	816
	local.get	4
	local.get	127
	v128.store	800
	local.get	4
	local.get	128
	v128.store	784
	local.get	4
	local.get	129
	v128.store	768
	local.get	4
	local.get	130
	v128.store	752
	local.get	4
	local.get	133
	v128.store	736
.Ltmp5744:
	.loc	42 569 13 is_stmt 1
	local.get	9
	local.get	8
	local.get	8
	i32.const	.Lalloc_2a3a21efd18b403ec25db545d522c479
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5745:
.LBB28_322:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	33
	v128.store	1072
	local.get	4
	local.get	37
	v128.store	1040
	local.get	4
	local.get	22
	v128.store	976
.Ltmp5746:
	local.get	4
	local.get	136
	v128.store	912
	local.get	4
	local.get	121
	v128.store	896
	local.get	4
	local.get	122
	v128.store	880
	local.get	4
	local.get	123
	v128.store	864
	local.get	4
	local.get	124
	v128.store	848
	local.get	4
	local.get	125
	v128.store	832
	local.get	4
	local.get	126
	v128.store	816
	local.get	4
	local.get	127
	v128.store	800
	local.get	4
	local.get	128
	v128.store	784
	local.get	4
	local.get	129
	v128.store	768
	local.get	4
	local.get	130
	v128.store	752
	local.get	4
	local.get	133
	v128.store	736
.Ltmp5747:
.LBB28_323:
	end_block
	.loc	22 3311 5 is_stmt 1
	local.get	4
	i32.const	2160
	i32.add 
	local.get	4
	i32.const	736
	i32.add 
	i32.const	368
	memory.copy	0, 0
	.loc	22 3311 14 is_stmt 0
	local.get	4
	i32.const	2160
	i32.add 
	local.get	15
	call	_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_
	.loc	22 3313 5 is_stmt 1
	local.get	0
	local.get	17
	i32.store	804
	.loc	22 3312 5
	local.get	0
	local.get	109
	i32.store	800
.Ltmp5748:
.LBB28_324:
	.loc	22 0 5 is_stmt 0
	end_block
	i32.const	0
	local.set	9
	.loc	22 2332 13 is_stmt 1
	local.get	14
	i32.eqz
	br_if   	1
	.loc	22 2332 32 is_stmt 0
	local.get	15
	call	_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest
	i32.eqz
	br_if   	1
	.loc	22 0 32
	block   	
	local.get	5
	local.get	2
	i32.gt_u
	br_if   	0
	local.get	1
	local.set	9
.LBB28_328:
	loop    	
	block   	
	local.get	5
	br_if   	0
	i32.const	1
	local.set	9
	br      	4
.LBB28_330:
	end_block
	local.get	9
	local.get	5
	i32.const	32
	local.get	5
	i32.const	32
.Ltmp5749:
	.loc	17 1077 12 is_stmt 1
	i32.lt_u
	i32.select
	local.tee	7
	i32.const	2
.Ltmp5750:
	.loc	6 863 18
	i32.shl 
	local.tee	10
	i32.add 
	local.set	11
	i32.const	0
	local.set	8
.Ltmp5751:
.LBB28_331:
	.loc	1 134 21
	loop    	
	local.get	9
	i32.load	0
	.loc	1 134 13 is_stmt 0
	local.get	8
	i32.or  
	local.set	8
	local.get	9
	i32.const	4
.Ltmp5752:
	.loc	24 656 28 is_stmt 1
	i32.add 
	local.set	9
	local.get	10
	i32.const	-4
.Ltmp5753:
	.loc	24 1714 9
	i32.add 
.Ltmp5754:
	.loc	23 180 28
	local.tee	10
	br_if   	0
	end_loop
.Ltmp5755:
	.loc	3 2054 74
	local.get	5
	local.get	7
	i32.sub 
	local.set	5
	local.get	11
	local.set	9
.Ltmp5756:
	.loc	1 136 12
	local.get	8
	i32.eqz
	br_if   	0
	end_loop
	i32.const	0
	local.set	9
	br      	2
.Ltmp5757:
.LBB28_334:
	.loc	1 0 12 is_stmt 0
	end_block
	i32.const	0
.Ltmp5758:
	.loc	42 443 13 is_stmt 1
	local.get	5
	local.get	2
	i32.const	.Lalloc_ec68fe2eaf9f84061aee17781c0add97
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5759:
.LBB28_335:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	120
	v128.store	1072
	local.get	4
	local.get	138
	v128.store	1040
	local.get	4
	local.get	137
	v128.store	976
.Ltmp5760:
	local.get	4
	local.get	136
	v128.store	912
	local.get	4
	local.get	121
	v128.store	896
	local.get	4
	local.get	122
	v128.store	880
	local.get	4
	local.get	123
	v128.store	864
	local.get	4
	local.get	124
	v128.store	848
	local.get	4
	local.get	125
	v128.store	832
	local.get	4
	local.get	126
	v128.store	816
	local.get	4
	local.get	127
	v128.store	800
	local.get	4
	local.get	128
	v128.store	784
	local.get	4
	local.get	129
	v128.store	768
	local.get	4
	local.get	130
	v128.store	752
	local.get	4
	local.get	133
	v128.store	736
	i32.const	0
	i32.const	4
.Ltmp5761:
	.loc	42 456 13 is_stmt 1
	local.get	9
	i32.const	.Lalloc_5f4e05826d69b5e832dc96c63f325058
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5762:
.LBB28_336:
	.loc	42 0 13 is_stmt 0
	end_block
	.loc	22 2331 9 is_stmt 1
	local.get	0
	local.get	9
	i32.store8	1124
	.loc	22 2333 30
	local.get	0
	local.get	0
	i32.load8_u	904
	.loc	22 2333 9 is_stmt 0
	i32.store8	1125
	local.get	2
	i32.const	536870908
.Ltmp5763:
	.loc	2 1851 23 is_stmt 1
	i32.and 
	i32.eqz
	br_if   	0
.Ltmp5764:
	.loc	2 0 23 is_stmt 0
	i32.const	0
	local.get	2
	i32.const	-4
.Ltmp5765:
	.loc	3 2155 12 is_stmt 1
	i32.and 
	i32.sub 
	local.set	8
	v128.const	-1, -1, -1, -1
	local.set	21
	local.get	1
	local.set	9
.Ltmp5766:
.LBB28_338:
	.loc	4 551 14
	loop    	
	local.get	9
	v128.load	0:p2align=2
.Ltmp5767:
	.loc	5 3812 14
	f32x4.abs
.Ltmp5768:
	.loc	5 1991 14
	v128.const	0x1.93e594p99, 0x1.93e594p99, 0x1.93e594p99, 0x1.93e594p99
	f32x4.lt
.Ltmp5769:
	.loc	5 2138 14
	local.get	21
	v128.and
	local.set	21
	local.get	9
	i32.const	16
.Ltmp5770:
	.loc	6 863 18
	i32.add 
	local.set	9
	local.get	8
	i32.const	4
.Ltmp5771:
	.loc	3 2155 12
	i32.add 
	local.tee	8
	br_if   	0
.Ltmp5772:
	.loc	3 0 12 is_stmt 0
	end_loop
.Ltmp5773:
	.loc	5 2198 34 is_stmt 1
	local.get	21
	v128.not
	.loc	5 2198 14 is_stmt 0
	v128.any_true
	i32.eqz
	br_if   	0
.Ltmp5774:
	.loc	22 2337 39 is_stmt 1
	local.get	0
	local.get	1
	local.get	2
	call	_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter
	.loc	22 2337 9 is_stmt 0
	i32.store	8
	local.get	0
	i64.const	-1
	.loc	22 2338 40 is_stmt 1
	local.get	0
	i64.load	0
	i64.const	1
.Ltmp5775:
	.loc	21 2428 13
	i64.add 
	local.tee	139
	local.get	139
	i64.eqz
	i64.select
.Ltmp5776:
	.loc	22 2338 9
	i64.store	0
	block   	
	local.get	2
	i32.const	2
.Ltmp5777:
	.loc	19 961 18
	i32.shl 
.Ltmp5778:
	.loc	38 25 13
	local.tee	9
	i32.eqz
	br_if   	0
	.loc	38 0 13 is_stmt 0
	local.get	1
	i32.const	0
	.loc	38 25 13
	local.get	9
	memory.fill	0
.Ltmp5779:
.LBB28_342:
	.loc	38 0 13
	end_block
	.loc	22 2340 21 is_stmt 1
	local.get	4
	local.get	16
	i32.load	8
	i32.store	1144
	local.get	4
	local.get	16
	i64.load	0:p2align=2
	i64.store	1136
.Ltmp5780:
	.loc	22 2343 14
	local.get	15
	local.get	4
	i32.const	1136
	i32.add 
	.loc	22 2343 40 is_stmt 0
	local.get	0
	i32.load	808
	local.get	0
	i32.load	812
.Ltmp5781:
	.loc	22 2341 20 is_stmt 1
	local.get	0
	i32.load	880
.Ltmp5782:
	.loc	22 2343 14
	local.tee	9
	call	_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults
	local.get	0
	i32.const	1024
	.loc	22 2344 9
	i32.add 
	.loc	22 2345 14
	local.get	4
	i32.const	1136
	i32.add 
	.loc	22 2345 40 is_stmt 0
	local.get	0
	i32.load	816
	local.get	0
	i32.load	820
	.loc	22 2345 14
	local.get	9
	call	_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults
	local.get	0
	i64.const	0
	.loc	22 2346 9 is_stmt 1
	i64.store	800
.Ltmp5783:
.LBB28_343:
	.loc	22 0 9 is_stmt 0
	end_block
	.loc	22 2347 6 is_stmt 1
	local.get	4
	i32.const	2528
	i32.add 
	global.set	__stack_pointer
	return
.LBB28_344:
	.loc	22 0 6 is_stmt 0
	end_block
	local.get	4
	local.get	120
	v128.store	1072
	local.get	4
	local.get	138
	v128.store	1040
	local.get	4
	local.get	137
	v128.store	976
.Ltmp5784:
	local.get	4
	local.get	136
	v128.store	912
	local.get	4
	local.get	121
	v128.store	896
	local.get	4
	local.get	122
	v128.store	880
	local.get	4
	local.get	123
	v128.store	864
	local.get	4
	local.get	124
	v128.store	848
	local.get	4
	local.get	125
	v128.store	832
	local.get	4
	local.get	126
	v128.store	816
	local.get	4
	local.get	127
	v128.store	800
	local.get	4
	local.get	128
	v128.store	784
	local.get	4
	local.get	129
	v128.store	768
	local.get	4
	local.get	130
	v128.store	752
	local.get	4
	local.get	133
	v128.store	736
.Ltmp5785:
	.loc	42 581 13 is_stmt 1
	local.get	25
	local.get	8
	local.get	8
	i32.const	.Lalloc_db7c42041d7aaaba4839906f37294547
	call	_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail
	unreachable
.Ltmp5786:
.LBB28_345:
	.loc	42 0 13 is_stmt 0
	end_block
	local.get	4
	local.get	120
	v128.store	1072
	local.get	4
	local.get	138
	v128.store	1040
	local.get	4
	local.get	137
	v128.store	976
.Ltmp5787:
	local.get	4
	local.get	136
	v128.store	912
	local.get	4
	local.get	121
	v128.store	896
	local.get	4
	local.get	122
	v128.store	880
	local.get	4
	local.get	123
	v128.store	864
	local.get	4
	local.get	124
	v128.store	848
	local.get	4
	local.get	125
	v128.store	832
	local.get	4
	local.get	126
	v128.store	816
	local.get	4
	local.get	127
	v128.store	800
	local.get	4
	local.get	128
	v128.store	784
	local.get	4
	local.get	129
	v128.store	768
	local.get	4
	local.get	130
	v128.store	752
	local.get	4
	local.get	133
	v128.store	736
.Ltmp5788:
	.loc	22 1403 42 is_stmt 1
	local.get	8
	local.get	8
	i32.const	.Lalloc_33746731a929bad63709ddedbca82f5f
	call	_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check
	unreachable
	end_function
