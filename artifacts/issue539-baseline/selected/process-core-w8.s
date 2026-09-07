_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13process_blockB5_:
.Lfunc_begin32:
	.loc	15 2070 0
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	pushq	%r15
	pushq	%r14
	pushq	%r13
	pushq	%r12
	pushq	%rbx
	andq	$-32, %rsp
	subq	$4064, %rsp
	movq	$0, (%rsp)
	subq	$3360, %rsp
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%rcx, %r11
	movq	%rdx, %r13
	movq	%rsi, 192(%rsp)
.Ltmp3338:
	.loc	15 2071 21 prologue_end
	leaq	(,%r9,8), %rax
	movq	%rax, 2296(%rsp)
.Ltmp3339:
	.loc	15 2089 21
	movzbl	2153(%rdi), %eax
	cmpb	2144(%rdi), %al
	jne	.LBB32_31
	.loc	15 2090 37
	movq	1776(%rdi), %rcx
	movq	1784(%rdi), %rax
.Ltmp3340:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp3341:
	.p2align	4
.LBB32_2:
.Ltmp3342:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp3343:
	.loc	17 180 28
	je	.LBB32_5
.Ltmp3344:
	.loc	15 687 21
	cmpl	$0, 12(%rcx)
.Ltmp3345:
	.loc	17 315 25
	jne	.LBB32_31
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB32_2
	jmp	.LBB32_31
.Ltmp3346:
.LBB32_5:
	.loc	15 2091 37 is_stmt 1
	movq	1792(%rdi), %rcx
	movq	1800(%rdi), %rax
.Ltmp3347:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp3348:
	.p2align	4
.LBB32_6:
.Ltmp3349:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp3350:
	.loc	17 180 28
	je	.LBB32_9
.Ltmp3351:
	.loc	15 687 21
	cmpl	$0, 12(%rcx)
.Ltmp3352:
	.loc	17 315 25
	jne	.LBB32_31
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB32_6
	jmp	.LBB32_31
.Ltmp3353:
.LBB32_9:
	.loc	15 2092 37 is_stmt 1
	movq	1976(%rdi), %rcx
	movq	1984(%rdi), %rax
.Ltmp3354:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp3355:
	.p2align	4
.LBB32_10:
.Ltmp3356:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp3357:
	.loc	17 180 28
	je	.LBB32_13
.Ltmp3358:
	.loc	15 687 21
	cmpl	$0, 12(%rcx)
.Ltmp3359:
	.loc	17 315 25
	jne	.LBB32_31
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB32_10
	jmp	.LBB32_31
.Ltmp3360:
.LBB32_13:
	.loc	15 2093 37 is_stmt 1
	movq	1992(%rdi), %rcx
	movq	2000(%rdi), %rax
.Ltmp3361:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp3362:
	.p2align	4
.LBB32_14:
.Ltmp3363:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp3364:
	.loc	17 180 28
	je	.LBB32_17
.Ltmp3365:
	.loc	15 687 21
	cmpl	$0, 12(%rcx)
.Ltmp3366:
	.loc	17 315 25
	jne	.LBB32_31
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB32_14
	jmp	.LBB32_31
.Ltmp3367:
.LBB32_17:
	.loc	17 0 25
	leaq	(,%r9,8), %rsi
	cmpq	%r13, %rsi
.Ltmp3368:
	.loc	14 1050 16 is_stmt 1
	ja	.LBB32_23
.Ltmp3369:
	.loc	14 0 16 is_stmt 0
	movq	%rsi, %rax
	movq	192(%rsp), %rcx
	.p2align	4
.LBB32_19:
.Ltmp3370:
	.loc	29 1504 12 is_stmt 1
	testq	%rax, %rax
	je	.LBB32_25
.Ltmp3371:
	.loc	10 1078 5
	cmpq	$32, %rax
	movl	$32, %edx
	cmovbq	%rax, %rdx
.Ltmp3372:
	.loc	12 961 18
	leal	(,%rdx,4), %ebx
	xorl	%esi, %esi
	xorl	%r10d, %r10d
.Ltmp3373:
	.loc	12 0 18 is_stmt 0
.Ltmp3374:
	.p2align	4
.LBB32_21:
	.loc	34 134 13 is_stmt 1
	orl	(%rcx,%r10), %esi
.Ltmp3375:
	.loc	16 1714 9
	addq	$4, %r10
	cmpq	%r10, %rbx
.Ltmp3376:
	.loc	17 180 28
	jne	.LBB32_21
.Ltmp3377:
	.loc	35 863 18
	leaq	(%rcx,%rdx,4), %rcx
.Ltmp3378:
	.loc	30 2054 74
	subq	%rdx, %rax
.Ltmp3379:
	.loc	34 136 12
	testl	%esi, %esi
	je	.LBB32_19
	jmp	.LBB32_31
.Ltmp3380:
.LBB32_25:
	.loc	34 0 12 is_stmt 0
	leaq	(,%r9,8), %rsi
.Ltmp3381:
	.loc	37 438 16 is_stmt 1
	cmpq	%r8, %rsi
	ja	.LBB32_394
.Ltmp3382:
	.loc	37 0 16 is_stmt 0
	movq	%rsi, %rax
	movq	%r11, %rcx
	.p2align	4
.LBB32_27:
.Ltmp3383:
	.loc	29 1504 12 is_stmt 1
	testq	%rax, %rax
	je	.LBB32_348
.Ltmp3384:
	.loc	10 1078 5
	cmpq	$32, %rax
	movl	$32, %edx
	cmovbq	%rax, %rdx
.Ltmp3385:
	.loc	12 961 18
	leal	(,%rdx,4), %ebx
	xorl	%esi, %esi
	xorl	%r10d, %r10d
.Ltmp3386:
	.loc	12 0 18 is_stmt 0
.Ltmp3387:
	.p2align	4
.LBB32_29:
	.loc	34 134 13 is_stmt 1
	orl	(%rcx,%r10), %esi
.Ltmp3388:
	.loc	16 1714 9
	addq	$4, %r10
	cmpq	%r10, %rbx
.Ltmp3389:
	.loc	17 180 28
	jne	.LBB32_29
.Ltmp3390:
	.loc	35 863 18
	leaq	(%rcx,%rdx,4), %rcx
.Ltmp3391:
	.loc	30 2054 74
	subq	%rdx, %rax
.Ltmp3392:
	.loc	34 136 12
	testl	%esi, %esi
	je	.LBB32_27
.Ltmp3393:
.LBB32_31:
	.loc	34 0 12 is_stmt 0
	xorl	%r10d, %r10d
.LBB32_32:
.Ltmp3394:
	.loc	15 2126 13 is_stmt 1
	leaq	1648(%rdi), %r14
	.loc	15 2127 13
	leaq	1848(%rdi), %rbx
.Ltmp3395:
	.loc	15 1011 5
	movq	1832(%rdi), %rcx
	testq	%rcx, %rcx
	movq	%rdi, 24(%rsp)
	movq	%r8, 176(%rsp)
	movq	%r13, 208(%rsp)
	movq	%r11, 200(%rsp)
	movq	%r9, 184(%rsp)
	movq	%rbx, 392(%rsp)
	movq	%r14, 400(%rsp)
	movl	%r10d, 764(%rsp)
	je	.LBB32_38
	.loc	15 0 5 is_stmt 0
	movq	1824(%rdi), %rax
	shlq	$2, %rcx
	leaq	(%rcx,%rcx,2), %rcx
	movq	%rax, %rdx
	.p2align	4
.LBB32_34:
.Ltmp3396:
	.loc	16 1714 9 is_stmt 1
	testq	%rcx, %rcx
.Ltmp3397:
	.loc	17 180 28
	je	.LBB32_38
.Ltmp3398:
	.loc	15 387 34
	movl	(%rdx), %esi
	cmpl	(%rax), %esi
	jne	.LBB32_52
	movl	4(%rdx), %esi
	cmpl	4(%rax), %esi
	jne	.LBB32_52
	movl	8(%rdx), %esi
.Ltmp3399:
	.loc	17 0 0 is_stmt 0
	addq	$12, %rdx
	.loc	17 315 25 is_stmt 1
	addq	$-12, %rcx
.Ltmp3400:
	.loc	15 387 34
	cmpl	8(%rax), %esi
.Ltmp3401:
	.loc	17 315 25
	je	.LBB32_34
	jmp	.LBB32_52
.Ltmp3402:
.LBB32_38:
	.loc	15 1012 12
	movq	1768(%rdi), %rax
	testq	%rax, %rax
	je	.LBB32_42
	.loc	15 0 12 is_stmt 0
	movq	1760(%rdi), %rcx
	shlq	$2, %rax
	xorl	%edx, %edx
	.p2align	4
.LBB32_40:
.Ltmp3403:
	.loc	16 1714 9 is_stmt 1
	cmpq	%rdx, %rax
.Ltmp3404:
	.loc	17 180 28
	je	.LBB32_42
.Ltmp3405:
	.loc	17 315 25
	movl	(%rcx,%rdx), %esi
	addq	$4, %rdx
.Ltmp3406:
	.loc	15 1012 43
	cmpl	(%rcx), %esi
.Ltmp3407:
	.loc	17 315 25
	je	.LBB32_40
	jmp	.LBB32_52
.Ltmp3408:
.LBB32_42:
	.loc	15 1011 5
	movq	2032(%rdi), %rcx
	testq	%rcx, %rcx
	je	.LBB32_48
	.loc	15 0 5 is_stmt 0
	movq	2024(%rdi), %rax
	shlq	$2, %rcx
	leaq	(%rcx,%rcx,2), %rcx
	movq	%rax, %rdx
	.p2align	4
.LBB32_44:
.Ltmp3409:
	.loc	16 1714 9 is_stmt 1
	testq	%rcx, %rcx
.Ltmp3410:
	.loc	17 180 28
	je	.LBB32_48
.Ltmp3411:
	.loc	15 387 34
	movl	(%rdx), %esi
	cmpl	(%rax), %esi
	jne	.LBB32_52
	movl	4(%rdx), %esi
	cmpl	4(%rax), %esi
	jne	.LBB32_52
	movl	8(%rdx), %esi
.Ltmp3412:
	.loc	17 0 0 is_stmt 0
	addq	$12, %rdx
	.loc	17 315 25 is_stmt 1
	addq	$-12, %rcx
.Ltmp3413:
	.loc	15 387 34
	cmpl	8(%rax), %esi
.Ltmp3414:
	.loc	17 315 25
	je	.LBB32_44
	jmp	.LBB32_52
.Ltmp3415:
.LBB32_48:
	.loc	15 1012 12
	movq	1968(%rdi), %rax
	testq	%rax, %rax
	je	.LBB32_234
	.loc	15 0 12 is_stmt 0
	movq	1960(%rdi), %rcx
	shlq	$2, %rax
	xorl	%edx, %edx
	.p2align	4
.LBB32_50:
.Ltmp3416:
	.loc	16 1714 9 is_stmt 1
	cmpq	%rdx, %rax
.Ltmp3417:
	.loc	17 180 28
	je	.LBB32_234
.Ltmp3418:
	.loc	17 315 25
	movl	(%rcx,%rdx), %esi
	addq	$4, %rdx
.Ltmp3419:
	.loc	15 1012 43
	cmpl	(%rcx), %esi
.Ltmp3420:
	.loc	17 315 25
	je	.LBB32_50
.Ltmp3421:
.LBB32_52:
	.loc	17 0 25 is_stmt 0
	leaq	3808(%rsp), %rdi
.Ltmp3422:
	.loc	15 1647 24 is_stmt 1
	movq	%r14, %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
	leaq	4544(%rsp), %rdi
.Ltmp3423:
	.loc	15 1648 25
	movq	%rbx, %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
	movq	24(%rsp), %rsi
.Ltmp3424:
	.loc	15 1660 43
	movq	1776(%rsi), %rcx
	movq	1784(%rsi), %rax
.Ltmp3425:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp3426:
	.p2align	4
.LBB32_53:
.Ltmp3427:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp3428:
	.loc	17 180 28
	je	.LBB32_56
.Ltmp3429:
	.loc	17 0 28 is_stmt 0
	movl	$0, 256(%rsp)
.Ltmp3430:
	.loc	15 687 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp3431:
	.loc	17 315 25
	jne	.LBB32_69
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB32_53
	jmp	.LBB32_69
.Ltmp3432:
.LBB32_56:
	.loc	15 1661 33 is_stmt 1
	movq	1792(%rsi), %rcx
	movq	1800(%rsi), %rax
.Ltmp3433:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp3434:
	.p2align	4
.LBB32_57:
.Ltmp3435:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp3436:
	.loc	17 180 28
	je	.LBB32_60
.Ltmp3437:
	.loc	17 0 28 is_stmt 0
	movl	$0, 256(%rsp)
.Ltmp3438:
	.loc	15 687 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp3439:
	.loc	17 315 25
	jne	.LBB32_69
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB32_57
	jmp	.LBB32_69
.Ltmp3440:
.LBB32_60:
	.loc	15 1662 33 is_stmt 1
	movq	1976(%rsi), %rcx
	movq	1984(%rsi), %rax
.Ltmp3441:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp3442:
	.p2align	4
.LBB32_61:
.Ltmp3443:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp3444:
	.loc	17 180 28
	je	.LBB32_64
.Ltmp3445:
	.loc	17 0 28 is_stmt 0
	movl	$0, 256(%rsp)
.Ltmp3446:
	.loc	15 687 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp3447:
	.loc	17 315 25
	jne	.LBB32_69
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB32_61
	jmp	.LBB32_69
.Ltmp3448:
.LBB32_234:
	.loc	17 0 25
	leaq	2336(%rsp), %rdi
.Ltmp3449:
	.loc	15 1818 24 is_stmt 1
	movq	%r14, %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
	leaq	3072(%rsp), %rdi
.Ltmp3450:
	.loc	15 1819 25
	movq	%rbx, %rsi
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
	movq	24(%rsp), %r8
.Ltmp3451:
	.loc	15 1822 43
	movq	1776(%r8), %rcx
	movq	1784(%r8), %rax
.Ltmp3452:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp3453:
	.p2align	4
.LBB32_235:
.Ltmp3454:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp3455:
	.loc	17 180 28
	je	.LBB32_238
.Ltmp3456:
	.loc	17 0 28 is_stmt 0
	xorl	%esi, %esi
.Ltmp3457:
	.loc	15 687 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp3458:
	.loc	17 315 25
	jne	.LBB32_251
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB32_235
	jmp	.LBB32_251
.Ltmp3459:
.LBB32_64:
	.loc	15 1663 33 is_stmt 1
	movq	1992(%rsi), %rcx
	movq	2000(%rsi), %rax
.Ltmp3460:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp3461:
	.p2align	4
.LBB32_65:
.Ltmp3462:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp3463:
	.loc	17 180 28
	je	.LBB32_66
.Ltmp3464:
	.loc	17 0 28 is_stmt 0
	movl	$0, 256(%rsp)
.Ltmp3465:
	.loc	15 687 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp3466:
	.loc	17 315 25
	jne	.LBB32_69
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB32_65
	jmp	.LBB32_69
.Ltmp3467:
.LBB32_238:
	.loc	15 1823 33 is_stmt 1
	movq	1792(%r8), %rcx
	movq	1800(%r8), %rax
.Ltmp3468:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp3469:
	.p2align	4
.LBB32_239:
.Ltmp3470:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp3471:
	.loc	17 180 28
	je	.LBB32_242
.Ltmp3472:
	.loc	17 0 28 is_stmt 0
	xorl	%esi, %esi
.Ltmp3473:
	.loc	15 687 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp3474:
	.loc	17 315 25
	jne	.LBB32_251
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB32_239
	jmp	.LBB32_251
.Ltmp3475:
.LBB32_66:
	.loc	17 0 25
	movb	$1, %al
	movl	%eax, 256(%rsp)
.LBB32_69:
.Ltmp3476:
	.loc	15 1666 19 is_stmt 1
	movzbl	1536(%rsi), %r14d
.Ltmp3477:
	.loc	15 1667 21
	movzbl	1537(%rsi), %ebx
.Ltmp3478:
	.loc	15 1668 27
	movl	1640(%rsi), %r12d
.Ltmp3479:
	.loc	15 1669 27
	movl	1644(%rsi), %eax
	movq	%rax, 32(%rsp)
	vxorps	%xmm0, %xmm0, %xmm0
	vmovaps	%ymm0, 544(%rsp)
	leaq	6376(%rsp), %rdi
	movq	memset@GOTPCREL(%rip), %r15
	movl	$1024, %edx
	xorl	%esi, %esi
	vzeroupper
	callq	*%r15
	leaq	5344(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*%r15
	movq	184(%rsp), %rax
.Ltmp3480:
	.loc	14 3758 16
	addq	$31, %rax
	shrq	$5, %rax
.Ltmp3481:
	.loc	40 446 20
	je	.LBB32_70
.Ltmp3482:
	.loc	40 0 20 is_stmt 0
	movq	%rax, 408(%rsp)
.Ltmp3483:
	.file	50 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/../../stdarch/crates/core_arch/src/x86/avx.rs"
	.loc	50 871 14 is_stmt 1
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm1
.Ltmp3484:
	.loc	50 713 19
	vpcmpeqd	%ymm0, %ymm0, %ymm0
	vxorps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm1, 224(%rsp)
	vmovaps	%ymm1, 1344(%rsp)
	testb	%r14b, %r14b
	movq	176(%rsp), %r14
	movq	200(%rsp), %r15
	jne	.LBB32_74
.Ltmp3485:
	.loc	50 0 19 is_stmt 0
	vmovaps	%ymm0, 1344(%rsp)
.LBB32_74:
	testb	%bl, %bl
	movq	24(%rsp), %r11
	jne	.LBB32_76
	vmovaps	%ymm0, 224(%rsp)
.LBB32_76:
	movq	$0, 16(%rsp)
	movq	%r15, 2112(%rsp)
	movq	%r14, 528(%rsp)
	movq	192(%rsp), %rax
	movq	%rax, 320(%rsp)
	movq	%r13, 536(%rsp)
	movq	$0, 1376(%rsp)
	movq	184(%rsp), %rax
	xorl	%edx, %edx
	movq	%r12, %rbx
.Ltmp3486:
	.loc	40 446 20 is_stmt 1
	jmp	.LBB32_79
.Ltmp3487:
	.loc	40 0 20 is_stmt 0
.Ltmp3488:
	.p2align	4
.LBB32_77:
	vmovaps	96(%rsp), %ymm0
.Ltmp3489:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 4288(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp3490:
	.loc	15 746 0 is_stmt 0
	vmovaps	%ymm0, 5024(%rsp)
.Ltmp3491:
.LBB32_78:
	.loc	15 0 0
	addq	$32, %rdx
	movq	408(%rsp), %rax
	decq	%rax
	movq	744(%rsp), %rcx
.Ltmp3492:
	.loc	40 446 20 is_stmt 1
	addq	$-32, %rcx
	addq	$256, 1376(%rsp)
	addq	$-256, 536(%rsp)
	addq	$1024, 320(%rsp)
	addq	$-256, 528(%rsp)
	addq	$1024, 2112(%rsp)
	movq	%rax, 408(%rsp)
	testq	%rax, %rax
	movq	%rcx, %rax
	je	.LBB32_71
.LBB32_79:
.Ltmp3493:
	.loc	14 2584 13
	cmpq	$1, %rax
	movq	%rax, 744(%rsp)
	movq	%rax, %rcx
	adcq	$0, %rcx
	cmpq	$32, %rcx
	movl	$32, %eax
	cmovaeq	%rax, %rcx
	movq	%rcx, 752(%rsp)
	leal	(,%rcx,8), %eax
.Ltmp3494:
	.loc	15 1612 23
	vmovaps	3808(%rsp), %ymm4
	vmovaps	3840(%rsp), %ymm5
	vmovaps	3872(%rsp), %ymm0
	vmovaps	3904(%rsp), %ymm1
	vmovaps	3936(%rsp), %ymm2
	vmovaps	3968(%rsp), %ymm8
	vmovaps	4000(%rsp), %ymm3
	vmovaps	4032(%rsp), %ymm7
	vmovaps	4064(%rsp), %ymm14
	vmovaps	4096(%rsp), %ymm11
	vmovaps	4128(%rsp), %ymm13
	movq	%rdx, 216(%rsp)
.Ltmp3495:
	.loc	10 1916 50
	cmpq	%rdx, 184(%rsp)
.Ltmp3496:
	.loc	11 900 12
	jne	.LBB32_81
.Ltmp3497:
	.loc	15 0 0 is_stmt 0
	vmovaps	4160(%rsp), %ymm6
	vmovaps	%ymm6, 672(%rsp)
.Ltmp3498:
	.loc	11 900 12
	jmp	.LBB32_88
.Ltmp3499:
	.loc	11 0 12
.Ltmp3500:
	.p2align	4
.LBB32_81:
	vmovaps	(%r11), %ymm6
	vmovaps	%ymm6, 768(%rsp)
	vmovaps	32(%r11), %ymm6
	vmovaps	%ymm6, 448(%rsp)
	vmovaps	64(%r11), %ymm6
	vmovaps	%ymm6, 480(%rsp)
	vmovaps	%ymm5, %ymm10
	vmovaps	96(%r11), %ymm5
	vmovaps	%ymm5, 704(%rsp)
	vmovaps	128(%r11), %ymm5
	vmovaps	%ymm5, 416(%rsp)
	vmovaps	160(%r11), %ymm5
	vmovaps	%ymm5, 288(%rsp)
	vmovaps	192(%r11), %ymm5
	vmovaps	%ymm5, 960(%rsp)
	vmovaps	224(%r11), %ymm5
	vmovaps	%ymm5, 128(%rsp)
	vmovaps	256(%r11), %ymm5
	vmovaps	%ymm5, 96(%rsp)
	vmovaps	288(%r11), %ymm5
	vmovaps	%ymm5, 928(%rsp)
	vmovaps	320(%r11), %ymm5
	vmovaps	%ymm5, 896(%rsp)
	movq	536(%rsp), %rdx
	xorl	%ecx, %ecx
	vmovaps	%ymm13, %ymm15
	vmovaps	%ymm11, %ymm12
	vmovaps	%ymm14, %ymm9
	vmovaps	%ymm7, %ymm6
	vmovaps	352(%r11), %ymm5
	vmovaps	%ymm5, 864(%rsp)
	vmovaps	384(%r11), %ymm5
	vmovaps	%ymm5, 832(%rsp)
	vmovaps	416(%r11), %ymm5
	vmovaps	%ymm5, 800(%rsp)
	vmovaps	448(%r11), %ymm5
	vmovaps	%ymm5, 2048(%rsp)
	vmovaps	480(%r11), %ymm5
	vmovaps	%ymm5, 2016(%rsp)
	vmovaps	512(%r11), %ymm5
	vmovaps	%ymm5, 1984(%rsp)
	vmovaps	544(%r11), %ymm5
	vmovaps	%ymm5, 1952(%rsp)
	vmovaps	576(%r11), %ymm5
	vmovaps	%ymm5, 1920(%rsp)
	vmovaps	608(%r11), %ymm5
	vmovaps	%ymm5, 1312(%rsp)
	vmovaps	640(%r11), %ymm5
	vmovaps	%ymm5, 1888(%rsp)
	vmovaps	672(%r11), %ymm5
	vmovaps	%ymm5, 1856(%rsp)
	vmovaps	704(%r11), %ymm5
	vmovaps	%ymm5, 1824(%rsp)
	vmovaps	736(%r11), %ymm5
	vmovaps	%ymm5, 1792(%rsp)
	vmovaps	768(%r11), %ymm5
	vmovaps	%ymm5, 1760(%rsp)
	vmovaps	800(%r11), %ymm5
	vmovaps	%ymm5, 1280(%rsp)
	vmovaps	832(%r11), %ymm5
	vmovaps	%ymm5, 1728(%rsp)
	vmovaps	864(%r11), %ymm5
	vmovaps	%ymm5, 1696(%rsp)
	vmovaps	896(%r11), %ymm5
	vmovaps	%ymm5, 1664(%rsp)
	vmovaps	928(%r11), %ymm5
	vmovaps	%ymm5, 1632(%rsp)
	vmovaps	960(%r11), %ymm5
	vmovaps	%ymm5, 1600(%rsp)
	vmovaps	992(%r11), %ymm5
	vmovaps	%ymm5, 1152(%rsp)
	vmovaps	1024(%r11), %ymm5
	vmovaps	%ymm5, 1248(%rsp)
	vmovaps	1056(%r11), %ymm5
	vmovaps	%ymm5, 1568(%rsp)
	vmovaps	1088(%r11), %ymm5
	vmovaps	%ymm5, 1536(%rsp)
	vmovaps	1120(%r11), %ymm5
	vmovaps	%ymm5, 1504(%rsp)
	vmovaps	1152(%r11), %ymm5
	vmovaps	%ymm5, 1216(%rsp)
	vmovaps	1184(%r11), %ymm5
	vmovaps	%ymm5, 1472(%rsp)
	vmovaps	1216(%r11), %ymm5
	vmovaps	%ymm5, 1440(%rsp)
	vmovaps	1248(%r11), %ymm5
	vmovaps	%ymm5, 1408(%rsp)
	vmovaps	1280(%r11), %ymm5
	vmovaps	%ymm5, 2240(%rsp)
	vmovaps	1312(%r11), %ymm5
	vmovaps	%ymm5, 2208(%rsp)
	vmovaps	1344(%r11), %ymm5
	vmovaps	%ymm5, 2176(%rsp)
	vmovaps	1376(%r11), %ymm5
	vmovaps	%ymm5, 2144(%rsp)
	vmovaps	1408(%r11), %ymm5
	vmovaps	%ymm5, 1120(%rsp)
	vmovaps	1440(%r11), %ymm5
	vmovaps	%ymm5, 1184(%rsp)
	vmovaps	1472(%r11), %ymm5
	vmovaps	%ymm5, 2304(%rsp)
	vmovaps	1504(%r11), %ymm5
	vmovaps	%ymm5, 2080(%rsp)
	.p2align	4
.LBB32_82:
	movq	1376(%rsp), %rsi
.Ltmp3501:
	.loc	37 568 12 is_stmt 1
	leaq	(%rsi,%rcx), %rdi
	cmpq	%r13, %rdi
	ja	.LBB32_85
.Ltmp3502:
	.loc	37 438 16
	cmpq	$7, %rdx
	jbe	.LBB32_84
.Ltmp3503:
	.loc	37 0 16 is_stmt 0
	vmovaps	%ymm3, %ymm14
	vmovaps	%ymm8, %ymm3
	vmovaps	%ymm2, %ymm8
	vmovaps	%ymm1, %ymm2
	vmovaps	%ymm0, %ymm1
	vmovaps	%ymm10, %ymm0
	movq	320(%rsp), %rsi
	vmovaps	%ymm12, %ymm13
	vmovaps	%ymm4, 64(%rsp)
	vmovaps	%ymm9, 352(%rsp)
	vmovaps	%ymm6, %ymm11
.Ltmp3504:
	.loc	1 551 14 is_stmt 1
	vmovups	(%rsi,%rcx,4), %ymm7
.Ltmp3505:
	.loc	50 82 19
	vbroadcastss	.LCPI32_0(%rip), %ymm10
.Ltmp3506:
	.loc	50 283 14
	vmulps	768(%rsp), %ymm7, %ymm6
	vmovaps	%ymm8, %ymm5
	vmovaps	%ymm2, %ymm8
	vmovaps	%ymm1, %ymm2
	vmovaps	%ymm0, %ymm4
	vxorps	%xmm0, %xmm0, %xmm0
.Ltmp3507:
	.loc	50 48 14
	vaddps	%ymm0, %ymm6, %ymm6
.Ltmp3508:
	.loc	50 283 14
	vmulps	448(%rsp), %ymm7, %ymm9
.Ltmp3509:
	.loc	50 48 14
	vaddps	%ymm0, %ymm9, %ymm9
.Ltmp3510:
	.loc	50 283 14
	vmulps	480(%rsp), %ymm7, %ymm12
.Ltmp3511:
	.loc	50 48 14
	vaddps	%ymm0, %ymm12, %ymm12
	vmovaps	%ymm15, %ymm1
.Ltmp3512:
	.loc	50 283 14
	vmulps	704(%rsp), %ymm7, %ymm15
.Ltmp3513:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm15
	vmovaps	64(%rsp), %ymm0
.Ltmp3514:
	.loc	50 283 14
	vmulps	416(%rsp), %ymm0, %ymm0
.Ltmp3515:
	.loc	50 48 14
	vaddps	%ymm0, %ymm6, %ymm0
	vmovaps	64(%rsp), %ymm6
.Ltmp3516:
	.loc	50 283 14
	vmulps	288(%rsp), %ymm6, %ymm6
.Ltmp3517:
	.loc	50 48 14
	vaddps	%ymm6, %ymm9, %ymm6
	vmovaps	960(%rsp), %ymm9
.Ltmp3518:
	.loc	50 283 14
	vmulps	64(%rsp), %ymm9, %ymm9
.Ltmp3519:
	.loc	50 48 14
	vaddps	%ymm9, %ymm12, %ymm9
	vmovaps	128(%rsp), %ymm12
.Ltmp3520:
	.loc	50 283 14
	vmulps	64(%rsp), %ymm12, %ymm12
.Ltmp3521:
	.loc	50 48 14
	vaddps	%ymm12, %ymm15, %ymm12
.Ltmp3522:
	.loc	50 283 14
	vmulps	96(%rsp), %ymm4, %ymm15
.Ltmp3523:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp3524:
	.loc	50 283 14
	vmulps	928(%rsp), %ymm4, %ymm15
.Ltmp3525:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp3526:
	.loc	50 283 14
	vmulps	896(%rsp), %ymm4, %ymm15
.Ltmp3527:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp3528:
	.loc	50 283 14
	vmulps	864(%rsp), %ymm4, %ymm15
.Ltmp3529:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp3530:
	.loc	50 283 14
	vmulps	832(%rsp), %ymm2, %ymm15
.Ltmp3531:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp3532:
	.loc	50 283 14
	vmulps	800(%rsp), %ymm2, %ymm15
.Ltmp3533:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp3534:
	.loc	50 283 14
	vmulps	2048(%rsp), %ymm2, %ymm15
.Ltmp3535:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp3536:
	.loc	50 283 14
	vmulps	2016(%rsp), %ymm2, %ymm15
.Ltmp3537:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp3538:
	.loc	50 283 14
	vmulps	1984(%rsp), %ymm8, %ymm15
.Ltmp3539:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp3540:
	.loc	50 283 14
	vmulps	1952(%rsp), %ymm8, %ymm15
.Ltmp3541:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp3542:
	.loc	50 283 14
	vmulps	1920(%rsp), %ymm8, %ymm15
.Ltmp3543:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp3544:
	.loc	50 283 14
	vmulps	1312(%rsp), %ymm8, %ymm15
.Ltmp3545:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp3546:
	.loc	50 283 14
	vmulps	1888(%rsp), %ymm5, %ymm15
.Ltmp3547:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp3548:
	.loc	50 283 14
	vmulps	1856(%rsp), %ymm5, %ymm15
.Ltmp3549:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp3550:
	.loc	50 283 14
	vmulps	1824(%rsp), %ymm5, %ymm15
.Ltmp3551:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp3552:
	.loc	50 283 14
	vmulps	1792(%rsp), %ymm5, %ymm15
.Ltmp3553:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp3554:
	.loc	50 283 14
	vmulps	1760(%rsp), %ymm3, %ymm15
.Ltmp3555:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp3556:
	.loc	50 283 14
	vmulps	1280(%rsp), %ymm3, %ymm15
.Ltmp3557:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp3558:
	.loc	50 283 14
	vmulps	1728(%rsp), %ymm3, %ymm15
.Ltmp3559:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp3560:
	.loc	50 283 14
	vmulps	1696(%rsp), %ymm3, %ymm15
.Ltmp3561:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp3562:
	.loc	50 283 14
	vmulps	1664(%rsp), %ymm14, %ymm15
.Ltmp3563:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp3564:
	.loc	50 283 14
	vmulps	1632(%rsp), %ymm14, %ymm15
.Ltmp3565:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp3566:
	.loc	50 283 14
	vmulps	1600(%rsp), %ymm14, %ymm15
.Ltmp3567:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp3568:
	.loc	50 283 14
	vmulps	1152(%rsp), %ymm14, %ymm15
.Ltmp3569:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp3570:
	.loc	50 283 14
	vmulps	1248(%rsp), %ymm11, %ymm15
.Ltmp3571:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp3572:
	.loc	50 283 14
	vmulps	1568(%rsp), %ymm11, %ymm15
.Ltmp3573:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp3574:
	.loc	50 283 14
	vmulps	1536(%rsp), %ymm11, %ymm15
.Ltmp3575:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp3576:
	.loc	50 283 14
	vmulps	1504(%rsp), %ymm11, %ymm15
.Ltmp3577:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
	vmovaps	352(%rsp), %ymm15
.Ltmp3578:
	.loc	50 283 14
	vmulps	1216(%rsp), %ymm15, %ymm15
.Ltmp3579:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
	vmovaps	352(%rsp), %ymm15
.Ltmp3580:
	.loc	50 283 14
	vmulps	1472(%rsp), %ymm15, %ymm15
.Ltmp3581:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
	vmovaps	352(%rsp), %ymm15
.Ltmp3582:
	.loc	50 283 14
	vmulps	1440(%rsp), %ymm15, %ymm15
.Ltmp3583:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
	vmovaps	352(%rsp), %ymm15
.Ltmp3584:
	.loc	50 283 14
	vmulps	1408(%rsp), %ymm15, %ymm15
.Ltmp3585:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp3586:
	.loc	50 283 14
	vmulps	2240(%rsp), %ymm13, %ymm15
.Ltmp3587:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp3588:
	.loc	50 283 14
	vmulps	2208(%rsp), %ymm13, %ymm15
.Ltmp3589:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp3590:
	.loc	50 283 14
	vmulps	2176(%rsp), %ymm13, %ymm15
.Ltmp3591:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp3592:
	.loc	50 283 14
	vmulps	2144(%rsp), %ymm13, %ymm15
.Ltmp3593:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp3594:
	.loc	50 283 14
	vmulps	1120(%rsp), %ymm1, %ymm15
.Ltmp3595:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp3596:
	.loc	50 283 14
	vmulps	1184(%rsp), %ymm1, %ymm15
.Ltmp3597:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp3598:
	.loc	50 283 14
	vmulps	2304(%rsp), %ymm1, %ymm15
.Ltmp3599:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
	vmovaps	%ymm1, 672(%rsp)
.Ltmp3600:
	.loc	50 283 14
	vmulps	2080(%rsp), %ymm1, %ymm15
.Ltmp3601:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp3602:
	.loc	50 82 19
	vandps	%ymm3, %ymm10, %ymm15
.Ltmp3603:
	.loc	50 82 19 is_stmt 0
	vandps	%ymm0, %ymm10, %ymm0
.Ltmp3604:
	.loc	50 233 14 is_stmt 1
	vmaxps	%ymm0, %ymm15, %ymm0
.Ltmp3605:
	.loc	50 82 19
	vandps	%ymm6, %ymm10, %ymm6
.Ltmp3606:
	.loc	50 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp3607:
	.loc	50 82 19
	vandps	%ymm10, %ymm9, %ymm6
.Ltmp3608:
	.loc	50 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp3609:
	.loc	50 82 19
	vandps	%ymm10, %ymm12, %ymm6
.Ltmp3610:
	.loc	50 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp3611:
	.loc	1 551 14
	vmovups	%ymm0, 6376(%rsp,%rcx,4)
	vmovaps	%ymm4, %ymm0
	vmovaps	%ymm2, %ymm1
	vmovaps	%ymm8, %ymm2
	vmovaps	%ymm5, %ymm8
	vmovaps	%ymm7, %ymm4
	vmovaps	%ymm14, %ymm7
	vmovaps	%ymm11, %ymm14
	vmovaps	352(%rsp), %ymm11
	vmovaps	64(%rsp), %ymm5
.Ltmp3612:
	.loc	10 1916 50
	addq	$8, %rcx
	addq	$-8, %rdx
	vmovaps	%ymm5, %ymm10
	vmovaps	%ymm13, %ymm15
	vmovaps	%ymm11, %ymm12
	vmovaps	%ymm14, %ymm9
	vmovaps	%ymm7, %ymm6
	cmpq	%rcx, %rax
.Ltmp3613:
	.loc	11 900 12
	jne	.LBB32_82
.Ltmp3614:
.LBB32_88:
	.loc	15 1618 5
	vmovaps	%ymm4, 3808(%rsp)
	vmovaps	%ymm5, 3840(%rsp)
	vmovaps	%ymm0, 3872(%rsp)
	vmovaps	%ymm1, 3904(%rsp)
	vmovaps	%ymm2, 3936(%rsp)
	vmovaps	%ymm8, 3968(%rsp)
	vmovaps	%ymm3, 4000(%rsp)
	vmovaps	%ymm7, 4032(%rsp)
	vmovaps	%ymm14, 4064(%rsp)
	vmovaps	%ymm11, 4096(%rsp)
	vmovaps	%ymm13, 4128(%rsp)
	vmovaps	672(%rsp), %ymm0
	vmovaps	%ymm0, 4160(%rsp)
.Ltmp3615:
	.loc	15 1612 23
	vmovaps	4544(%rsp), %ymm4
	vmovaps	4576(%rsp), %ymm5
	vmovaps	4608(%rsp), %ymm0
	vmovaps	4640(%rsp), %ymm1
	vmovaps	4672(%rsp), %ymm2
	vmovaps	4704(%rsp), %ymm8
	vmovaps	4736(%rsp), %ymm3
	vmovaps	4768(%rsp), %ymm7
	vmovaps	4800(%rsp), %ymm14
	vmovaps	4832(%rsp), %ymm11
	vmovaps	4864(%rsp), %ymm13
	movq	216(%rsp), %rcx
.Ltmp3616:
	.loc	10 1916 50
	cmpq	%rcx, 184(%rsp)
.Ltmp3617:
	.loc	11 900 12
	jne	.LBB32_90
.Ltmp3618:
	.loc	15 0 0 is_stmt 0
	vmovaps	4896(%rsp), %ymm6
	vmovaps	%ymm6, 672(%rsp)
.Ltmp3619:
	.loc	11 900 12
	jmp	.LBB32_94
.Ltmp3620:
	.loc	11 0 12
.Ltmp3621:
	.p2align	4
.LBB32_90:
	vmovaps	(%r11), %ymm6
	vmovaps	%ymm6, 768(%rsp)
	vmovaps	32(%r11), %ymm6
	vmovaps	%ymm6, 448(%rsp)
	vmovaps	64(%r11), %ymm6
	vmovaps	%ymm6, 480(%rsp)
	vmovaps	%ymm5, %ymm10
	vmovaps	96(%r11), %ymm5
	vmovaps	%ymm5, 704(%rsp)
	vmovaps	128(%r11), %ymm5
	vmovaps	%ymm5, 416(%rsp)
	vmovaps	160(%r11), %ymm5
	vmovaps	%ymm5, 288(%rsp)
	vmovaps	192(%r11), %ymm5
	vmovaps	%ymm5, 960(%rsp)
	vmovaps	224(%r11), %ymm5
	vmovaps	%ymm5, 128(%rsp)
	vmovaps	256(%r11), %ymm5
	vmovaps	%ymm5, 96(%rsp)
	vmovaps	288(%r11), %ymm5
	vmovaps	%ymm5, 928(%rsp)
	vmovaps	320(%r11), %ymm5
	vmovaps	%ymm5, 896(%rsp)
	movq	528(%rsp), %rdx
	xorl	%ecx, %ecx
	vmovaps	%ymm13, %ymm15
	vmovaps	%ymm11, %ymm12
	vmovaps	%ymm14, %ymm9
	vmovaps	%ymm7, %ymm6
	vmovaps	352(%r11), %ymm5
	vmovaps	%ymm5, 864(%rsp)
	vmovaps	384(%r11), %ymm5
	vmovaps	%ymm5, 832(%rsp)
	vmovaps	416(%r11), %ymm5
	vmovaps	%ymm5, 800(%rsp)
	vmovaps	448(%r11), %ymm5
	vmovaps	%ymm5, 2048(%rsp)
	vmovaps	480(%r11), %ymm5
	vmovaps	%ymm5, 2016(%rsp)
	vmovaps	512(%r11), %ymm5
	vmovaps	%ymm5, 1984(%rsp)
	vmovaps	544(%r11), %ymm5
	vmovaps	%ymm5, 1952(%rsp)
	vmovaps	576(%r11), %ymm5
	vmovaps	%ymm5, 1920(%rsp)
	vmovaps	608(%r11), %ymm5
	vmovaps	%ymm5, 1312(%rsp)
	vmovaps	640(%r11), %ymm5
	vmovaps	%ymm5, 1888(%rsp)
	vmovaps	672(%r11), %ymm5
	vmovaps	%ymm5, 1856(%rsp)
	vmovaps	704(%r11), %ymm5
	vmovaps	%ymm5, 1824(%rsp)
	vmovaps	736(%r11), %ymm5
	vmovaps	%ymm5, 1792(%rsp)
	vmovaps	768(%r11), %ymm5
	vmovaps	%ymm5, 1760(%rsp)
	vmovaps	800(%r11), %ymm5
	vmovaps	%ymm5, 1280(%rsp)
	vmovaps	832(%r11), %ymm5
	vmovaps	%ymm5, 1728(%rsp)
	vmovaps	864(%r11), %ymm5
	vmovaps	%ymm5, 1696(%rsp)
	vmovaps	896(%r11), %ymm5
	vmovaps	%ymm5, 1664(%rsp)
	vmovaps	928(%r11), %ymm5
	vmovaps	%ymm5, 1632(%rsp)
	vmovaps	960(%r11), %ymm5
	vmovaps	%ymm5, 1600(%rsp)
	vmovaps	992(%r11), %ymm5
	vmovaps	%ymm5, 1152(%rsp)
	vmovaps	1024(%r11), %ymm5
	vmovaps	%ymm5, 1248(%rsp)
	vmovaps	1056(%r11), %ymm5
	vmovaps	%ymm5, 1568(%rsp)
	vmovaps	1088(%r11), %ymm5
	vmovaps	%ymm5, 1536(%rsp)
	vmovaps	1120(%r11), %ymm5
	vmovaps	%ymm5, 1504(%rsp)
	vmovaps	1152(%r11), %ymm5
	vmovaps	%ymm5, 1216(%rsp)
	vmovaps	1184(%r11), %ymm5
	vmovaps	%ymm5, 1472(%rsp)
	vmovaps	1216(%r11), %ymm5
	vmovaps	%ymm5, 1440(%rsp)
	vmovaps	1248(%r11), %ymm5
	vmovaps	%ymm5, 1408(%rsp)
	vmovaps	1280(%r11), %ymm5
	vmovaps	%ymm5, 2240(%rsp)
	vmovaps	1312(%r11), %ymm5
	vmovaps	%ymm5, 2208(%rsp)
	vmovaps	1344(%r11), %ymm5
	vmovaps	%ymm5, 2176(%rsp)
	vmovaps	1376(%r11), %ymm5
	vmovaps	%ymm5, 2144(%rsp)
	vmovaps	1408(%r11), %ymm5
	vmovaps	%ymm5, 1120(%rsp)
	vmovaps	1440(%r11), %ymm5
	vmovaps	%ymm5, 1184(%rsp)
	vmovaps	1472(%r11), %ymm5
	vmovaps	%ymm5, 2304(%rsp)
	vmovaps	1504(%r11), %ymm5
	vmovaps	%ymm5, 2080(%rsp)
	.p2align	4
.LBB32_91:
	movq	1376(%rsp), %rsi
.Ltmp3622:
	.loc	37 568 12 is_stmt 1
	leaq	(%rsi,%rcx), %rdi
	cmpq	%r14, %rdi
	ja	.LBB32_101
.Ltmp3623:
	.loc	37 438 16
	cmpq	$7, %rdx
	jbe	.LBB32_84
.Ltmp3624:
	.loc	37 0 16 is_stmt 0
	vmovaps	%ymm3, %ymm14
	vmovaps	%ymm8, %ymm3
	vmovaps	%ymm2, %ymm8
	vmovaps	%ymm1, %ymm2
	vmovaps	%ymm0, %ymm1
	vmovaps	%ymm10, %ymm0
	movq	2112(%rsp), %rsi
	vmovaps	%ymm12, %ymm13
	vmovaps	%ymm4, 64(%rsp)
	vmovaps	%ymm9, 352(%rsp)
	vmovaps	%ymm6, %ymm11
.Ltmp3625:
	.loc	1 551 14 is_stmt 1
	vmovups	(%rsi,%rcx,4), %ymm7
.Ltmp3626:
	.loc	50 82 19
	vbroadcastss	.LCPI32_0(%rip), %ymm10
.Ltmp3627:
	.loc	50 283 14
	vmulps	768(%rsp), %ymm7, %ymm6
	vmovaps	%ymm8, %ymm5
	vmovaps	%ymm2, %ymm8
	vmovaps	%ymm1, %ymm2
	vmovaps	%ymm0, %ymm4
	vxorps	%xmm0, %xmm0, %xmm0
.Ltmp3628:
	.loc	50 48 14
	vaddps	%ymm0, %ymm6, %ymm6
.Ltmp3629:
	.loc	50 283 14
	vmulps	448(%rsp), %ymm7, %ymm9
.Ltmp3630:
	.loc	50 48 14
	vaddps	%ymm0, %ymm9, %ymm9
.Ltmp3631:
	.loc	50 283 14
	vmulps	480(%rsp), %ymm7, %ymm12
.Ltmp3632:
	.loc	50 48 14
	vaddps	%ymm0, %ymm12, %ymm12
	vmovaps	%ymm15, %ymm1
.Ltmp3633:
	.loc	50 283 14
	vmulps	704(%rsp), %ymm7, %ymm15
.Ltmp3634:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm15
	vmovaps	64(%rsp), %ymm0
.Ltmp3635:
	.loc	50 283 14
	vmulps	416(%rsp), %ymm0, %ymm0
.Ltmp3636:
	.loc	50 48 14
	vaddps	%ymm0, %ymm6, %ymm0
	vmovaps	64(%rsp), %ymm6
.Ltmp3637:
	.loc	50 283 14
	vmulps	288(%rsp), %ymm6, %ymm6
.Ltmp3638:
	.loc	50 48 14
	vaddps	%ymm6, %ymm9, %ymm6
	vmovaps	960(%rsp), %ymm9
.Ltmp3639:
	.loc	50 283 14
	vmulps	64(%rsp), %ymm9, %ymm9
.Ltmp3640:
	.loc	50 48 14
	vaddps	%ymm9, %ymm12, %ymm9
	vmovaps	128(%rsp), %ymm12
.Ltmp3641:
	.loc	50 283 14
	vmulps	64(%rsp), %ymm12, %ymm12
.Ltmp3642:
	.loc	50 48 14
	vaddps	%ymm12, %ymm15, %ymm12
.Ltmp3643:
	.loc	50 283 14
	vmulps	96(%rsp), %ymm4, %ymm15
.Ltmp3644:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp3645:
	.loc	50 283 14
	vmulps	928(%rsp), %ymm4, %ymm15
.Ltmp3646:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp3647:
	.loc	50 283 14
	vmulps	896(%rsp), %ymm4, %ymm15
.Ltmp3648:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp3649:
	.loc	50 283 14
	vmulps	864(%rsp), %ymm4, %ymm15
.Ltmp3650:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp3651:
	.loc	50 283 14
	vmulps	832(%rsp), %ymm2, %ymm15
.Ltmp3652:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp3653:
	.loc	50 283 14
	vmulps	800(%rsp), %ymm2, %ymm15
.Ltmp3654:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp3655:
	.loc	50 283 14
	vmulps	2048(%rsp), %ymm2, %ymm15
.Ltmp3656:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp3657:
	.loc	50 283 14
	vmulps	2016(%rsp), %ymm2, %ymm15
.Ltmp3658:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp3659:
	.loc	50 283 14
	vmulps	1984(%rsp), %ymm8, %ymm15
.Ltmp3660:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp3661:
	.loc	50 283 14
	vmulps	1952(%rsp), %ymm8, %ymm15
.Ltmp3662:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp3663:
	.loc	50 283 14
	vmulps	1920(%rsp), %ymm8, %ymm15
.Ltmp3664:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp3665:
	.loc	50 283 14
	vmulps	1312(%rsp), %ymm8, %ymm15
.Ltmp3666:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp3667:
	.loc	50 283 14
	vmulps	1888(%rsp), %ymm5, %ymm15
.Ltmp3668:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp3669:
	.loc	50 283 14
	vmulps	1856(%rsp), %ymm5, %ymm15
.Ltmp3670:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp3671:
	.loc	50 283 14
	vmulps	1824(%rsp), %ymm5, %ymm15
.Ltmp3672:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp3673:
	.loc	50 283 14
	vmulps	1792(%rsp), %ymm5, %ymm15
.Ltmp3674:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp3675:
	.loc	50 283 14
	vmulps	1760(%rsp), %ymm3, %ymm15
.Ltmp3676:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp3677:
	.loc	50 283 14
	vmulps	1280(%rsp), %ymm3, %ymm15
.Ltmp3678:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp3679:
	.loc	50 283 14
	vmulps	1728(%rsp), %ymm3, %ymm15
.Ltmp3680:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp3681:
	.loc	50 283 14
	vmulps	1696(%rsp), %ymm3, %ymm15
.Ltmp3682:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp3683:
	.loc	50 283 14
	vmulps	1664(%rsp), %ymm14, %ymm15
.Ltmp3684:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp3685:
	.loc	50 283 14
	vmulps	1632(%rsp), %ymm14, %ymm15
.Ltmp3686:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp3687:
	.loc	50 283 14
	vmulps	1600(%rsp), %ymm14, %ymm15
.Ltmp3688:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp3689:
	.loc	50 283 14
	vmulps	1152(%rsp), %ymm14, %ymm15
.Ltmp3690:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp3691:
	.loc	50 283 14
	vmulps	1248(%rsp), %ymm11, %ymm15
.Ltmp3692:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp3693:
	.loc	50 283 14
	vmulps	1568(%rsp), %ymm11, %ymm15
.Ltmp3694:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp3695:
	.loc	50 283 14
	vmulps	1536(%rsp), %ymm11, %ymm15
.Ltmp3696:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp3697:
	.loc	50 283 14
	vmulps	1504(%rsp), %ymm11, %ymm15
.Ltmp3698:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
	vmovaps	352(%rsp), %ymm15
.Ltmp3699:
	.loc	50 283 14
	vmulps	1216(%rsp), %ymm15, %ymm15
.Ltmp3700:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
	vmovaps	352(%rsp), %ymm15
.Ltmp3701:
	.loc	50 283 14
	vmulps	1472(%rsp), %ymm15, %ymm15
.Ltmp3702:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
	vmovaps	352(%rsp), %ymm15
.Ltmp3703:
	.loc	50 283 14
	vmulps	1440(%rsp), %ymm15, %ymm15
.Ltmp3704:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
	vmovaps	352(%rsp), %ymm15
.Ltmp3705:
	.loc	50 283 14
	vmulps	1408(%rsp), %ymm15, %ymm15
.Ltmp3706:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp3707:
	.loc	50 283 14
	vmulps	2240(%rsp), %ymm13, %ymm15
.Ltmp3708:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp3709:
	.loc	50 283 14
	vmulps	2208(%rsp), %ymm13, %ymm15
.Ltmp3710:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp3711:
	.loc	50 283 14
	vmulps	2176(%rsp), %ymm13, %ymm15
.Ltmp3712:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp3713:
	.loc	50 283 14
	vmulps	2144(%rsp), %ymm13, %ymm15
.Ltmp3714:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp3715:
	.loc	50 283 14
	vmulps	1120(%rsp), %ymm1, %ymm15
.Ltmp3716:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp3717:
	.loc	50 283 14
	vmulps	1184(%rsp), %ymm1, %ymm15
.Ltmp3718:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp3719:
	.loc	50 283 14
	vmulps	2304(%rsp), %ymm1, %ymm15
.Ltmp3720:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
	vmovaps	%ymm1, 672(%rsp)
.Ltmp3721:
	.loc	50 283 14
	vmulps	2080(%rsp), %ymm1, %ymm15
.Ltmp3722:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp3723:
	.loc	50 82 19
	vandps	%ymm3, %ymm10, %ymm15
.Ltmp3724:
	.loc	50 82 19 is_stmt 0
	vandps	%ymm0, %ymm10, %ymm0
.Ltmp3725:
	.loc	50 233 14 is_stmt 1
	vmaxps	%ymm0, %ymm15, %ymm0
.Ltmp3726:
	.loc	50 82 19
	vandps	%ymm6, %ymm10, %ymm6
.Ltmp3727:
	.loc	50 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp3728:
	.loc	50 82 19
	vandps	%ymm10, %ymm9, %ymm6
.Ltmp3729:
	.loc	50 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp3730:
	.loc	50 82 19
	vandps	%ymm10, %ymm12, %ymm6
.Ltmp3731:
	.loc	50 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp3732:
	.loc	1 551 14
	vmovups	%ymm0, 5344(%rsp,%rcx,4)
	vmovaps	%ymm4, %ymm0
	vmovaps	%ymm2, %ymm1
	vmovaps	%ymm8, %ymm2
	vmovaps	%ymm5, %ymm8
	vmovaps	%ymm7, %ymm4
	vmovaps	%ymm14, %ymm7
	vmovaps	%ymm11, %ymm14
	vmovaps	352(%rsp), %ymm11
	vmovaps	64(%rsp), %ymm5
.Ltmp3733:
	.loc	10 1916 50
	addq	$8, %rcx
	addq	$-8, %rdx
	vmovaps	%ymm5, %ymm10
	vmovaps	%ymm13, %ymm15
	vmovaps	%ymm11, %ymm12
	vmovaps	%ymm14, %ymm9
	vmovaps	%ymm7, %ymm6
	cmpq	%rcx, %rax
.Ltmp3734:
	.loc	11 900 12
	jne	.LBB32_91
.Ltmp3735:
.LBB32_94:
	.loc	15 1618 5
	vmovaps	%ymm4, 4544(%rsp)
	vmovaps	%ymm5, 4576(%rsp)
	vmovaps	%ymm0, 4608(%rsp)
	vmovaps	%ymm1, 4640(%rsp)
	vmovaps	%ymm2, 4672(%rsp)
	vmovaps	%ymm8, 4704(%rsp)
	vmovaps	%ymm3, 4736(%rsp)
	vmovaps	%ymm7, 4768(%rsp)
	vmovaps	%ymm14, 4800(%rsp)
	vmovaps	%ymm11, 4832(%rsp)
	vmovaps	%ymm13, 4864(%rsp)
	vmovaps	672(%rsp), %ymm0
	vmovaps	%ymm0, 4896(%rsp)
	movq	216(%rsp), %rdx
.Ltmp3736:
	.loc	10 1916 50
	cmpq	%rdx, 184(%rsp)
	je	.LBB32_78
.Ltmp3737:
	.loc	10 0 50 is_stmt 0
	vmovaps	4192(%rsp), %ymm11
	vmovaps	4256(%rsp), %ymm0
	vmovaps	%ymm0, 928(%rsp)
	vmovaps	4288(%rsp), %ymm0
	vmovaps	%ymm0, 96(%rsp)
	vmovaps	4320(%rsp), %ymm12
	vmovaps	4416(%rsp), %ymm0
	vmovaps	%ymm0, 864(%rsp)
	vmovaps	4384(%rsp), %ymm0
	vmovaps	%ymm0, 896(%rsp)
	vmovaps	4928(%rsp), %ymm3
	vmovaps	4992(%rsp), %ymm0
	vmovaps	%ymm0, 832(%rsp)
	vmovaps	5024(%rsp), %ymm0
	vmovaps	%ymm0, 128(%rsp)
	vmovaps	5056(%rsp), %ymm5
	vmovaps	5152(%rsp), %ymm6
	vmovaps	5120(%rsp), %ymm0
	vmovaps	%ymm0, 800(%rsp)
	vmovaps	4480(%rsp), %ymm1
	vmovaps	5216(%rsp), %ymm0
	xorl	%eax, %eax
	vbroadcastss	.LCPI32_2(%rip), %ymm14
	.p2align	4
.LBB32_96:
.Ltmp3738:
	.loc	15 1701 77 is_stmt 1
	cmpb	$0, 256(%rsp)
	jne	.LBB32_98
	.loc	15 0 77 is_stmt 0
	vbroadcastss	.LCPI32_1(%rip), %ymm9
.Ltmp3739:
	.loc	50 347 14 is_stmt 1
	vaddps	96(%rsp), %ymm9, %ymm2
	vxorps	%xmm10, %xmm10, %xmm10
.Ltmp3740:
	.loc	50 233 14
	vmaxps	%ymm10, %ymm2, %ymm2
	vmovaps	%ymm2, 96(%rsp)
.Ltmp3741:
	.loc	50 871 14
	vcmpgt_oqps	%ymm10, %ymm2, %ymm2
	vmovaps	928(%rsp), %ymm13
.Ltmp3742:
	.loc	50 48 14
	vaddps	%ymm13, %ymm11, %ymm7
.Ltmp3743:
	.loc	15 748 73
	vmovaps	4224(%rsp), %ymm8
.Ltmp3744:
	.loc	50 585 19
	vblendvps	%ymm2, %ymm7, %ymm8, %ymm11
.Ltmp3745:
	.loc	15 748 73
	vmovaps	4352(%rsp), %ymm7
.Ltmp3746:
	.loc	50 585 19
	vblendvps	%ymm2, %ymm13, %ymm10, %ymm13
.Ltmp3747:
	.loc	50 347 14
	vaddps	864(%rsp), %ymm9, %ymm2
.Ltmp3748:
	.loc	50 233 14
	vmaxps	%ymm10, %ymm2, %ymm4
.Ltmp3749:
	.loc	50 871 14
	vcmpgt_oqps	%ymm10, %ymm4, %ymm2
	vmovaps	896(%rsp), %ymm15
.Ltmp3750:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm8
.Ltmp3751:
	.loc	50 585 19
	vblendvps	%ymm2, %ymm8, %ymm7, %ymm12
.Ltmp3752:
	.loc	50 585 19 is_stmt 0
	vblendvps	%ymm2, %ymm15, %ymm10, %ymm15
.Ltmp3753:
	.loc	50 347 14 is_stmt 1
	vaddps	128(%rsp), %ymm9, %ymm2
.Ltmp3754:
	.loc	50 233 14
	vmaxps	%ymm10, %ymm2, %ymm2
	vmovaps	%ymm2, 128(%rsp)
.Ltmp3755:
	.loc	50 871 14
	vcmpgt_oqps	%ymm10, %ymm2, %ymm2
	vmovaps	832(%rsp), %ymm8
.Ltmp3756:
	.loc	50 48 14
	vaddps	%ymm3, %ymm8, %ymm3
.Ltmp3757:
	.loc	15 748 73
	vmovaps	4960(%rsp), %ymm7
.Ltmp3758:
	.loc	50 585 19
	vblendvps	%ymm2, %ymm3, %ymm7, %ymm3
.Ltmp3759:
	.loc	15 748 73
	vmovaps	5088(%rsp), %ymm7
.Ltmp3760:
	.loc	50 585 19
	vblendvps	%ymm2, %ymm8, %ymm10, %ymm8
.Ltmp3761:
	.loc	50 347 14
	vaddps	%ymm6, %ymm9, %ymm2
.Ltmp3762:
	.loc	50 233 14
	vmaxps	%ymm10, %ymm2, %ymm6
.Ltmp3763:
	.loc	50 871 14
	vcmpgt_oqps	%ymm10, %ymm6, %ymm2
	vmovaps	800(%rsp), %ymm9
.Ltmp3764:
	.loc	50 48 14
	vaddps	%ymm5, %ymm9, %ymm5
.Ltmp3765:
	.loc	50 585 19
	vblendvps	%ymm2, %ymm5, %ymm7, %ymm5
.Ltmp3766:
	.loc	50 585 19 is_stmt 0
	vblendvps	%ymm2, %ymm9, %ymm10, %ymm9
.Ltmp3767:
	.loc	15 748 9 is_stmt 1
	vmovaps	%ymm11, 4192(%rsp)
	vmovaps	%ymm13, 928(%rsp)
	.loc	15 749 9
	vmovaps	%ymm13, 4256(%rsp)
	vmovaps	%ymm4, 864(%rsp)
.Ltmp3768:
	.loc	15 746 9
	vmovaps	%ymm4, 4416(%rsp)
.Ltmp3769:
	.loc	15 748 9
	vmovaps	%ymm12, 4320(%rsp)
	vmovaps	%ymm15, 896(%rsp)
	.loc	15 749 9
	vmovaps	%ymm15, 4384(%rsp)
.Ltmp3770:
	.loc	15 748 9
	vmovaps	%ymm3, 4928(%rsp)
	vmovaps	%ymm8, 832(%rsp)
	.loc	15 749 9
	vmovaps	%ymm8, 4992(%rsp)
.Ltmp3771:
	.loc	15 746 9
	vmovaps	%ymm6, 5152(%rsp)
.Ltmp3772:
	.loc	15 748 9
	vmovaps	%ymm5, 5056(%rsp)
	vmovaps	%ymm9, 800(%rsp)
	.loc	15 749 9
	vmovaps	%ymm9, 5120(%rsp)
.Ltmp3773:
.LBB32_98:
	.loc	15 0 0 is_stmt 0
	leaq	(%rax,%rdx), %rsi
	shlq	$3, %rsi
.Ltmp3774:
	.loc	37 568 12 is_stmt 1
	movq	%r13, %rdx
	movq	%rsi, 480(%rsp)
	subq	%rsi, %rdx
	jb	.LBB32_185
.Ltmp3775:
	.loc	37 438 16
	cmpq	$7, %rdx
	jbe	.LBB32_100
.Ltmp3776:
	.loc	15 1287 25
	movq	1688(%r11), %rsi
.Ltmp3777:
	.loc	15 1283 17
	movq	1840(%r11), %rcx
.Ltmp3778:
	.loc	15 1287 45
	movq	%rcx, %rdi
	imulq	32(%rsp), %rdi
.Ltmp3779:
	.loc	37 580 12
	movq	%rsi, %rdx
	subq	%rdi, %rdx
	jb	.LBB32_135
.Ltmp3780:
	.loc	37 451 16
	cmpq	$7, %rdx
	jbe	.LBB32_100
.Ltmp3781:
	.loc	37 0 16 is_stmt 0
	movq	%rcx, 448(%rsp)
	movq	%rbx, 704(%rsp)
	movq	%rax, 288(%rsp)
	shlq	$5, %rax
	vmovups	6376(%rsp,%rax), %ymm7
.Ltmp3782:
	vmovups	5344(%rsp,%rax), %ymm8
.Ltmp3783:
	vmaxps	%ymm7, %ymm8, %ymm2
	vmovaps	1344(%rsp), %ymm9
.Ltmp3784:
	vblendvps	%ymm9, %ymm2, %ymm7, %ymm7
.Ltmp3785:
	vdivps	%ymm7, %ymm11, %ymm9
.Ltmp3786:
	movq	1624(%r11), %r13
.Ltmp3787:
	vcmpgt_oqps	%ymm11, %ymm7, %ymm7
	vblendvps	%ymm7, %ymm9, %ymm14, %ymm7
.Ltmp3788:
	.loc	15 1287 25 is_stmt 1
	movq	1680(%r11), %rax
	movq	%rdi, 416(%rsp)
.Ltmp3789:
	.loc	1 551 14
	vmovups	%ymm7, (%rax,%rdi,4)
.Ltmp3790:
	.loc	15 1154 17
	movq	1840(%r11), %rdx
.Ltmp3791:
	.loc	43 37 12
	testq	%rdx, %rdx
	je	.LBB32_123
.Ltmp3792:
	.loc	43 0 12 is_stmt 0
	movq	24(%rsp), %rdi
	movq	1832(%rdi), %rax
	movq	%rax, 352(%rsp)
	movq	32(%rsp), %rax
	leaq	1(%rax), %r9
	cmpq	%r13, %r9
	movq	%r13, %rax
	movl	$0, %ecx
	cmovbq	%rcx, %rax
	movq	1824(%rdi), %rcx
	subq	%rax, %r9
	movq	1680(%rdi), %rax
	movq	1688(%rdi), %rsi
	movq	1768(%rdi), %r8
	movq	%r8, 672(%rsp)
	movq	1760(%rdi), %rbx
	movq	1736(%rdi), %r8
	movq	%r8, 64(%rsp)
	movq	1728(%rdi), %r14
	imulq	%rdx, %r9
	movq	%r9, 768(%rsp)
	movq	%rdx, %r15
	xorl	%r11d, %r11d
	xorl	%edi, %edi
	jmp	.LBB32_107
	.p2align	4
.LBB32_134:
	xorl	%r9d, %r9d
.LBB32_122:
	decq	%r15
	addq	$4, %rdi
.Ltmp3793:
	movl	%r9d, (%rbx,%r11,4)
.Ltmp3794:
	incq	%r11
.Ltmp3795:
	.loc	43 37 12 is_stmt 1
	testq	%r15, %r15
	je	.LBB32_123
.LBB32_107:
.Ltmp3796:
	.loc	16 1714 9
	cmpq	$32, %rdi
.Ltmp3797:
	.loc	17 180 28
	je	.LBB32_123
.Ltmp3798:
	.loc	15 1158 21
	cmpq	352(%rsp), %r11
	je	.LBB32_395
	leaq	(%r11,%r11,2), %r9
	movl	4(%rcx,%r9,4), %r8d
.Ltmp3799:
	.loc	15 1160 23
	addq	32(%rsp), %r8
.Ltmp3800:
	.loc	15 1161 12
	cmpq	%r13, %r8
	movl	$0, %r10d
	cmovaeq	%r13, %r10
	subq	%r10, %r8
.Ltmp3801:
	.loc	15 1168 42
	movq	%r8, %r12
	imulq	%rdx, %r12
	addq	%r11, %r12
	.loc	15 1168 22 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB32_396
.Ltmp3802:
	.loc	15 1169 24 is_stmt 1
	cmpq	672(%rsp), %r11
	je	.LBB32_118
.Ltmp3803:
	.loc	15 0 0 is_stmt 0
	movl	(%rcx,%r9,4), %r10d
.Ltmp3804:
	vmovss	(%rax,%r12,4), %xmm7
.Ltmp3805:
	.loc	15 1169 24
	movl	(%rbx,%r11,4), %r9d
	testq	%r9, %r9
.Ltmp3806:
	.loc	15 1170 26 is_stmt 1
	je	.LBB32_114
	.loc	15 1173 24
	cmpq	64(%rsp), %r11
	jae	.LBB32_189
	vmovss	(%r14,%r11,4), %xmm9
.Ltmp3807:
	.loc	15 798 8
	vucomiss	%xmm9, %xmm7
	jbe	.LBB32_114
.Ltmp3808:
	.loc	15 0 8 is_stmt 0
	vmovaps	%xmm9, %xmm7
.LBB32_114:
.Ltmp3809:
	.loc	15 1175 9 is_stmt 1
	cmpq	64(%rsp), %r11
	je	.LBB32_191
	vmovss	%xmm7, (%r14,%r11,4)
	.loc	15 1176 24
	incq	%r9
	cmpq	%r10, %r9
.Ltmp3810:
	.loc	15 1177 23
	jne	.LBB32_116
	.loc	15 1177 9 is_stmt 0
	vmovss	%xmm7, 544(%rsp,%rdi)
	.loc	15 1183 30 is_stmt 1
	vmovss	(%rax,%r12,4), %xmm7
	.loc	15 0 30 is_stmt 0
.Ltmp3811:
	.p2align	4
.LBB32_132:
.Ltmp3812:
	.loc	15 1186 65 is_stmt 1
	movq	%r8, %r12
	imulq	%rdx, %r12
	addq	%r11, %r12
	.loc	15 1186 45 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB32_397
.Ltmp3813:
	.loc	15 798 8 is_stmt 1
	vminss	(%rax,%r12,4), %xmm7, %xmm7
.Ltmp3814:
	.loc	15 1187 17
	vmovss	%xmm7, (%rax,%r12,4)
	.loc	15 1188 20
	testq	%r8, %r8
	cmoveq	%r13, %r8
	.loc	15 1191 17
	decq	%r8
.Ltmp3815:
	.loc	10 1916 50
	decq	%r10
.Ltmp3816:
	.loc	11 900 12
	jne	.LBB32_132
	jmp	.LBB32_134
.Ltmp3817:
	.loc	11 0 12 is_stmt 0
.Ltmp3818:
	.p2align	4
.LBB32_116:
	movq	768(%rsp), %r8
	.loc	15 1180 44 is_stmt 1
	leaq	(%r11,%r8), %r12
	.loc	15 1180 24 is_stmt 0
	cmpq	%rsi, %r12
	jae	.LBB32_117
	vmovss	(%rax,%r12,4), %xmm9
.Ltmp3819:
	.loc	15 798 8 is_stmt 1
	vminss	%xmm7, %xmm9, %xmm7
.Ltmp3820:
	.loc	15 1177 9
	vmovss	%xmm7, 544(%rsp,%rdi)
	jmp	.LBB32_122
.Ltmp3821:
	.loc	15 0 9 is_stmt 0
.Ltmp3822:
	.p2align	4
.LBB32_123:
	.loc	1 551 14 is_stmt 1
	vmovaps	544(%rsp), %ymm7
	movq	24(%rsp), %r11
.Ltmp3823:
	.loc	15 1307 26
	movq	1704(%r11), %rsi
	vmovaps	%ymm7, %ymm10
	movq	448(%rsp), %rdi
.Ltmp3824:
	.loc	43 37 12
	testq	%rdi, %rdi
	movq	176(%rsp), %r14
	movq	704(%rsp), %rbx
	movq	416(%rsp), %r15
	je	.LBB32_138
.Ltmp3825:
	.loc	43 0 12 is_stmt 0
	movq	1832(%r11), %r9
.Ltmp3826:
	.loc	15 1298 42 is_stmt 1
	testq	%r9, %r9
	je	.LBB32_130
	.loc	15 0 42 is_stmt 0
	movq	1824(%r11), %r10
	.loc	15 1298 42
	movl	8(%r10), %r8d
	.loc	15 1298 28
	addq	32(%rsp), %r8
.Ltmp3827:
	.loc	15 1299 16 is_stmt 1
	cmpq	%r13, %r8
	movl	$0, %eax
	cmovaeq	%r13, %rax
	subq	%rax, %r8
	.loc	15 1302 40
	imulq	%rdi, %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB32_232
	.loc	15 0 25
	movq	1696(%r11), %rdx
	.loc	15 1302 25
	vmovss	(%rdx,%r8,4), %xmm9
	.loc	15 1302 13
	vmovss	%xmm9, 544(%rsp)
.Ltmp3828:
	.loc	43 37 12 is_stmt 1
	cmpq	$1, %rdi
	je	.LBB32_137
.Ltmp3829:
	.loc	15 1298 42
	cmpq	$1, %r9
	je	.LBB32_128
	movl	20(%r10), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp3830:
	.loc	15 1299 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%rdi, %rax
	leaq	1(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB32_232
	vmovss	4(%rdx,%rax,4), %xmm9
	.loc	15 1302 13
	vmovss	%xmm9, 548(%rsp)
.Ltmp3831:
	.loc	43 37 12 is_stmt 1
	cmpq	$2, %rdi
	je	.LBB32_137
.Ltmp3832:
	.loc	15 1298 42
	cmpq	$2, %r9
	je	.LBB32_162
	movl	32(%r10), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp3833:
	.loc	15 1299 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%rdi, %rax
	leaq	2(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB32_232
	vmovss	8(%rdx,%rax,4), %xmm9
	.loc	15 1302 13
	vmovss	%xmm9, 552(%rsp)
.Ltmp3834:
	.loc	43 37 12 is_stmt 1
	cmpq	$3, %rdi
	je	.LBB32_137
.Ltmp3835:
	.loc	15 1298 42
	cmpq	$3, %r9
	je	.LBB32_166
	movl	44(%r10), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp3836:
	.loc	15 1299 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%rdi, %rax
	leaq	3(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB32_232
	vmovss	12(%rdx,%rax,4), %xmm9
	.loc	15 1302 13
	vmovss	%xmm9, 556(%rsp)
.Ltmp3837:
	.loc	43 37 12 is_stmt 1
	cmpq	$4, %rdi
	je	.LBB32_137
.Ltmp3838:
	.loc	15 1298 42
	cmpq	$4, %r9
	je	.LBB32_170
	movl	56(%r10), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp3839:
	.loc	15 1299 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%rdi, %rax
	leaq	4(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB32_232
	vmovss	16(%rdx,%rax,4), %xmm9
	.loc	15 1302 13
	vmovss	%xmm9, 560(%rsp)
.Ltmp3840:
	.loc	43 37 12 is_stmt 1
	cmpq	$5, %rdi
	je	.LBB32_137
.Ltmp3841:
	.loc	15 1298 42
	cmpq	$5, %r9
	je	.LBB32_174
	movl	68(%r10), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp3842:
	.loc	15 1299 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%rdi, %rax
	leaq	5(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB32_232
	vmovss	20(%rdx,%rax,4), %xmm9
	.loc	15 1302 13
	vmovss	%xmm9, 564(%rsp)
.Ltmp3843:
	.loc	43 37 12 is_stmt 1
	cmpq	$6, %rdi
	je	.LBB32_137
.Ltmp3844:
	.loc	15 1298 42
	cmpq	$6, %r9
	je	.LBB32_178
	movl	80(%r10), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp3845:
	.loc	15 1299 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%rdi, %rax
	leaq	6(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB32_232
	vmovss	24(%rdx,%rax,4), %xmm9
	.loc	15 1302 13
	vmovss	%xmm9, 568(%rsp)
.Ltmp3846:
	.loc	43 37 12 is_stmt 1
	cmpq	$7, %rdi
	je	.LBB32_137
.Ltmp3847:
	.loc	15 1298 42
	cmpq	$7, %r9
	je	.LBB32_182
	movl	92(%r10), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp3848:
	.loc	15 1299 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%rdi, %rax
	leaq	7(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB32_232
	vmovss	28(%rdx,%rax,4), %xmm9
	.loc	15 1302 13
	vmovss	%xmm9, 572(%rsp)
.Ltmp3849:
	.loc	15 0 13
.Ltmp3850:
	.p2align	4
.LBB32_137:
	.loc	1 551 14 is_stmt 1
	vmovaps	544(%rsp), %ymm10
.Ltmp3851:
.LBB32_138:
	.loc	15 0 0 is_stmt 0
	vbroadcastss	.LCPI32_3(%rip), %ymm9
	vmulps	%ymm7, %ymm9, %ymm7
	vroundps	$9, %ymm7, %ymm7
	vbroadcastss	.LCPI32_4(%rip), %ymm9
	vmulps	%ymm7, %ymm9, %ymm7
.Ltmp3852:
	.loc	50 48 14 is_stmt 1
	vaddps	%ymm1, %ymm7, %ymm1
.Ltmp3853:
	.loc	50 347 14
	vsubps	%ymm10, %ymm1, %ymm1
.Ltmp3854:
	.loc	15 1306 5
	vmovaps	%ymm1, 4480(%rsp)
.Ltmp3855:
	.loc	37 580 12
	movq	%rsi, %rdx
	subq	%r15, %rdx
	jb	.LBB32_398
.Ltmp3856:
	.loc	37 451 16
	cmpq	$7, %rdx
	jbe	.LBB32_100
.Ltmp3857:
	.loc	15 1307 26
	movq	1696(%r11), %rax
.Ltmp3858:
	.loc	1 551 14
	vmovups	%ymm7, (%rax,%r15,4)
.Ltmp3859:
	.loc	50 360 14
	vdivps	4512(%rsp), %ymm1, %ymm7
.Ltmp3860:
	.loc	15 1311 43
	vmovaps	4448(%rsp), %ymm9
.Ltmp3861:
	.loc	50 347 14
	vsubps	%ymm7, %ymm14, %ymm7
.Ltmp3862:
	.loc	50 347 14 is_stmt 0
	vsubps	%ymm9, %ymm7, %ymm13
.Ltmp3863:
	.loc	50 283 14 is_stmt 1
	vmulps	%ymm13, %ymm12, %ymm13
.Ltmp3864:
	.loc	50 48 14
	vaddps	%ymm13, %ymm9, %ymm9
.Ltmp3865:
	.loc	50 233 14
	vmaxps	%ymm9, %ymm7, %ymm7
.Ltmp3866:
	.loc	50 82 19
	vbroadcastss	.LCPI32_0(%rip), %ymm9
	vandps	%ymm7, %ymm9, %ymm13
.Ltmp3867:
	.loc	50 871 14
	vbroadcastss	.LCPI32_5(%rip), %ymm15
	vcmplt_oqps	%ymm15, %ymm13, %ymm13
.Ltmp3868:
	.loc	50 82 19
	vandnps	%ymm7, %ymm13, %ymm7
.Ltmp3869:
	.loc	15 1312 5
	vmovaps	%ymm7, 4448(%rsp)
.Ltmp3870:
	.loc	15 1315 28
	movq	1672(%r11), %rsi
	.loc	15 1315 44 is_stmt 0
	imulq	%rbx, %rdi
.Ltmp3871:
	.loc	37 568 12 is_stmt 1
	movq	%rsi, %rdx
	subq	%rdi, %rdx
	jb	.LBB32_231
.Ltmp3872:
	.loc	37 438 16
	cmpq	$7, %rdx
	jbe	.LBB32_100
.Ltmp3873:
	.loc	37 0 16 is_stmt 0
	movq	192(%rsp), %rax
	movq	480(%rsp), %rsi
	leaq	(%rax,%rsi,4), %rax
.Ltmp3874:
	vsubps	%ymm7, %ymm14, %ymm7
.Ltmp3875:
	.loc	15 1315 28 is_stmt 1
	movq	1664(%r11), %rcx
.Ltmp3876:
	.loc	1 551 14
	vmovups	(%rcx,%rdi,4), %ymm13
.Ltmp3877:
	.loc	1 551 14 is_stmt 0
	vmovups	(%rax), %ymm14
.Ltmp3878:
	.loc	50 283 14 is_stmt 1
	vmulps	%ymm7, %ymm13, %ymm7
	vmovaps	224(%rsp), %ymm15
.Ltmp3879:
	.loc	50 585 19
	vblendvps	%ymm15, %ymm13, %ymm7, %ymm7
.Ltmp3880:
	.loc	1 551 14
	vmovups	%ymm14, (%rcx,%rdi,4)
.Ltmp3881:
	.loc	1 551 14 is_stmt 0
	vmovups	%ymm7, (%rax)
.Ltmp3882:
	.loc	37 568 12 is_stmt 1
	movq	%r14, %rdx
	subq	%rsi, %rdx
	jb	.LBB32_233
.Ltmp3883:
	.loc	37 438 16
	cmpq	$7, %rdx
	jbe	.LBB32_100
.Ltmp3884:
	.loc	15 1287 25
	movq	1888(%r11), %rsi
.Ltmp3885:
	.loc	15 1283 17
	movq	2040(%r11), %rdi
.Ltmp3886:
	.loc	15 1287 45
	movq	%rdi, %rcx
	imulq	32(%rsp), %rcx
.Ltmp3887:
	.loc	37 580 12
	movq	%rsi, %rdx
	subq	%rcx, %rdx
	jb	.LBB32_230
.Ltmp3888:
	.loc	37 451 16
	cmpq	$7, %rdx
	jbe	.LBB32_100
.Ltmp3889:
	.loc	37 0 16 is_stmt 0
	vmovaps	1344(%rsp), %ymm7
	vblendvps	%ymm7, %ymm2, %ymm8, %ymm2
.Ltmp3890:
	vdivps	%ymm2, %ymm3, %ymm7
	vcmpgt_oqps	%ymm3, %ymm2, %ymm2
	vbroadcastss	.LCPI32_2(%rip), %ymm14
	vblendvps	%ymm2, %ymm7, %ymm14, %ymm2
.Ltmp3891:
	.loc	15 1287 25 is_stmt 1
	movq	1880(%r11), %rax
.Ltmp3892:
	.loc	1 551 14
	vmovups	%ymm2, (%rax,%rcx,4)
.Ltmp3893:
	.loc	15 1154 17
	movq	2040(%r11), %rdx
.Ltmp3894:
	.loc	43 37 12
	testq	%rdx, %rdx
	movq	%rcx, 416(%rsp)
	je	.LBB32_195
.Ltmp3895:
	.loc	43 0 12 is_stmt 0
	movq	%rdi, 960(%rsp)
	movq	2032(%r11), %rax
	movq	%rax, 672(%rsp)
	movq	32(%rsp), %rax
	leaq	1(%rax), %rdi
	cmpq	%r13, %rdi
	movq	%r13, %rcx
	movl	$0, %eax
	cmovbq	%rax, %rcx
	movq	2024(%r11), %rax
	subq	%rcx, %rdi
	movq	1880(%r11), %rbx
	movq	1888(%r11), %rsi
	movq	1968(%r11), %rcx
	movq	%rcx, 768(%rsp)
	movq	1960(%r11), %r15
	movq	1936(%r11), %rcx
	movq	%rcx, 64(%rsp)
	movq	1928(%r11), %rcx
	movq	%rcx, 352(%rsp)
	imulq	%rdx, %rdi
	movq	%rdi, 448(%rsp)
	movq	%rdx, %r10
	xorl	%r11d, %r11d
	xorl	%r8d, %r8d
	jmp	.LBB32_148
	.p2align	4
.LBB32_229:
	xorl	%ecx, %ecx
.LBB32_193:
	decq	%r10
	addq	$4, %r8
.Ltmp3896:
	movl	%ecx, (%r15,%r11,4)
.Ltmp3897:
	incq	%r11
.Ltmp3898:
	.loc	43 37 12 is_stmt 1
	testq	%r10, %r10
	je	.LBB32_194
.LBB32_148:
.Ltmp3899:
	.loc	16 1714 9
	cmpq	$32, %r8
.Ltmp3900:
	.loc	17 180 28
	je	.LBB32_194
.Ltmp3901:
	.loc	15 1158 21
	cmpq	672(%rsp), %r11
	je	.LBB32_186
	leaq	(%r11,%r11,2), %rcx
	movl	4(%rax,%rcx,4), %r12d
.Ltmp3902:
	.loc	15 1160 23
	addq	32(%rsp), %r12
.Ltmp3903:
	.loc	15 1161 12
	cmpq	%r13, %r12
	movl	$0, %edi
	cmovaeq	%r13, %rdi
	subq	%rdi, %r12
.Ltmp3904:
	.loc	15 1168 42
	movq	%r12, %rdi
	imulq	%rdx, %rdi
	addq	%r11, %rdi
	.loc	15 1168 22 is_stmt 0
	cmpq	%rsi, %rdi
	jae	.LBB32_187
.Ltmp3905:
	.loc	15 1169 24 is_stmt 1
	cmpq	768(%rsp), %r11
	je	.LBB32_188
.Ltmp3906:
	.loc	15 0 0 is_stmt 0
	movl	(%rax,%rcx,4), %r9d
.Ltmp3907:
	vmovss	(%rbx,%rdi,4), %xmm2
.Ltmp3908:
	.loc	15 1169 24
	movl	(%r15,%r11,4), %ecx
	testq	%rcx, %rcx
	je	.LBB32_155
.Ltmp3909:
	.loc	15 1173 24 is_stmt 1
	cmpq	64(%rsp), %r11
	jae	.LBB32_189
	.loc	15 0 24 is_stmt 0
	movq	352(%rsp), %r14
	.loc	15 1173 24
	vmovss	(%r14,%r11,4), %xmm7
.Ltmp3910:
	.loc	15 798 8 is_stmt 1
	vucomiss	%xmm7, %xmm2
	jbe	.LBB32_155
.Ltmp3911:
	.loc	15 0 8 is_stmt 0
	vmovaps	%xmm7, %xmm2
.LBB32_155:
.Ltmp3912:
	.loc	15 1175 9 is_stmt 1
	cmpq	64(%rsp), %r11
	je	.LBB32_191
	.loc	15 0 9 is_stmt 0
	movq	352(%rsp), %r14
	.loc	15 1175 9
	vmovss	%xmm2, (%r14,%r11,4)
	.loc	15 1176 24 is_stmt 1
	incq	%rcx
	cmpq	%r9, %rcx
.Ltmp3913:
	.loc	15 1177 23
	jne	.LBB32_157
	.loc	15 1177 9 is_stmt 0
	vmovss	%xmm2, 544(%rsp,%r8)
	.loc	15 1183 30 is_stmt 1
	vmovss	(%rbx,%rdi,4), %xmm2
	.loc	15 0 30 is_stmt 0
.Ltmp3914:
	.p2align	4
.LBB32_227:
.Ltmp3915:
	.loc	15 1186 65 is_stmt 1
	movq	%r12, %r14
	imulq	%rdx, %r14
	addq	%r11, %r14
	.loc	15 1186 45 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB32_399
.Ltmp3916:
	.loc	15 798 8 is_stmt 1
	vminss	(%rbx,%r14,4), %xmm2, %xmm2
.Ltmp3917:
	.loc	15 1187 17
	vmovss	%xmm2, (%rbx,%r14,4)
	.loc	15 1188 20
	testq	%r12, %r12
	cmoveq	%r13, %r12
	.loc	15 1191 17
	decq	%r12
.Ltmp3918:
	.loc	10 1916 50
	decq	%r9
.Ltmp3919:
	.loc	11 900 12
	jne	.LBB32_227
	jmp	.LBB32_229
.Ltmp3920:
	.loc	11 0 12 is_stmt 0
.Ltmp3921:
	.p2align	4
.LBB32_157:
	movq	448(%rsp), %rdi
	.loc	15 1180 44 is_stmt 1
	leaq	(%r11,%rdi), %r14
	.loc	15 1180 24 is_stmt 0
	cmpq	%rsi, %r14
	jae	.LBB32_158
	vmovss	(%rbx,%r14,4), %xmm7
.Ltmp3922:
	.loc	15 798 8 is_stmt 1
	vminss	%xmm2, %xmm7, %xmm2
.Ltmp3923:
	.loc	15 1177 9
	vmovss	%xmm2, 544(%rsp,%r8)
	jmp	.LBB32_193
.Ltmp3924:
	.loc	15 0 9 is_stmt 0
.Ltmp3925:
	.p2align	4
.LBB32_194:
	.loc	1 551 14 is_stmt 1
	vmovaps	544(%rsp), %ymm10
	movq	24(%rsp), %r11
	movq	176(%rsp), %r14
	movq	704(%rsp), %rbx
	movq	960(%rsp), %rdi
	movq	416(%rsp), %rcx
.Ltmp3926:
.LBB32_195:
	.loc	15 1307 26
	movq	1904(%r11), %rsi
	vmovaps	%ymm10, %ymm7
.Ltmp3927:
	.loc	43 37 12
	testq	%rdi, %rdi
	movq	200(%rsp), %r15
	je	.LBB32_221
.Ltmp3928:
	.loc	43 0 12 is_stmt 0
	movq	2032(%r11), %r9
.Ltmp3929:
	.loc	15 1298 42 is_stmt 1
	testq	%r9, %r9
	je	.LBB32_130
	.loc	15 0 42 is_stmt 0
	movq	2024(%r11), %r10
	.loc	15 1298 42
	movl	8(%r10), %r8d
	.loc	15 1298 28
	addq	32(%rsp), %r8
.Ltmp3930:
	.loc	15 1299 16 is_stmt 1
	cmpq	%r13, %r8
	movl	$0, %eax
	cmovaeq	%r13, %rax
	subq	%rax, %r8
	.loc	15 1302 40
	imulq	%rdi, %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB32_232
	.loc	15 0 25
	movq	1896(%r11), %rdx
	.loc	15 1302 25
	vmovss	(%rdx,%r8,4), %xmm2
	.loc	15 1302 13
	vmovss	%xmm2, 544(%rsp)
.Ltmp3931:
	.loc	43 37 12 is_stmt 1
	cmpq	$1, %rdi
	je	.LBB32_220
.Ltmp3932:
	.loc	15 1298 42
	cmpq	$1, %r9
	je	.LBB32_128
	.loc	15 0 42 is_stmt 0
	movq	%rdi, %r12
	.loc	15 1298 42
	movl	20(%r10), %eax
	.loc	15 1298 28
	addq	32(%rsp), %rax
.Ltmp3933:
	.loc	15 1299 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%r12, %rax
	leaq	1(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB32_232
	.loc	15 0 25
	movq	%r12, %rdi
	.loc	15 1302 25
	vmovss	4(%rdx,%rax,4), %xmm2
	.loc	15 1302 13
	vmovss	%xmm2, 548(%rsp)
.Ltmp3934:
	.loc	43 37 12 is_stmt 1
	cmpq	$2, %r12
	movq	416(%rsp), %rcx
	je	.LBB32_220
.Ltmp3935:
	.loc	15 1298 42
	cmpq	$2, %r9
	je	.LBB32_162
	movl	32(%r10), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp3936:
	.loc	15 1299 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%r12, %rax
	leaq	2(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB32_232
	.loc	15 0 25
	movq	%r12, %rdi
	.loc	15 1302 25
	vmovss	8(%rdx,%rax,4), %xmm2
	.loc	15 1302 13
	vmovss	%xmm2, 552(%rsp)
.Ltmp3937:
	.loc	43 37 12 is_stmt 1
	cmpq	$3, %r12
	movq	416(%rsp), %rcx
	je	.LBB32_220
.Ltmp3938:
	.loc	15 1298 42
	cmpq	$3, %r9
	je	.LBB32_166
	movl	44(%r10), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp3939:
	.loc	15 1299 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%r12, %rax
	leaq	3(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB32_232
	.loc	15 0 25
	movq	%r12, %rdi
	.loc	15 1302 25
	vmovss	12(%rdx,%rax,4), %xmm2
	.loc	15 1302 13
	vmovss	%xmm2, 556(%rsp)
.Ltmp3940:
	.loc	43 37 12 is_stmt 1
	cmpq	$4, %r12
	movq	416(%rsp), %rcx
	je	.LBB32_220
.Ltmp3941:
	.loc	15 1298 42
	cmpq	$4, %r9
	je	.LBB32_170
	movl	56(%r10), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp3942:
	.loc	15 1299 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%r12, %rax
	leaq	4(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB32_232
	.loc	15 0 25
	movq	%r12, %rdi
	.loc	15 1302 25
	vmovss	16(%rdx,%rax,4), %xmm2
	.loc	15 1302 13
	vmovss	%xmm2, 560(%rsp)
.Ltmp3943:
	.loc	43 37 12 is_stmt 1
	cmpq	$5, %r12
	movq	416(%rsp), %rcx
	je	.LBB32_220
.Ltmp3944:
	.loc	15 1298 42
	cmpq	$5, %r9
	je	.LBB32_174
	movl	68(%r10), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp3945:
	.loc	15 1299 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%r12, %rax
	leaq	5(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB32_232
	.loc	15 0 25
	movq	%r12, %rdi
	.loc	15 1302 25
	vmovss	20(%rdx,%rax,4), %xmm2
	.loc	15 1302 13
	vmovss	%xmm2, 564(%rsp)
.Ltmp3946:
	.loc	43 37 12 is_stmt 1
	cmpq	$6, %r12
	movq	416(%rsp), %rcx
	je	.LBB32_220
.Ltmp3947:
	.loc	15 1298 42
	cmpq	$6, %r9
	je	.LBB32_178
	movl	80(%r10), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp3948:
	.loc	15 1299 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%r12, %rax
	leaq	6(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB32_232
	.loc	15 0 25
	movq	%r12, %rdi
	.loc	15 1302 25
	vmovss	24(%rdx,%rax,4), %xmm2
	.loc	15 1302 13
	vmovss	%xmm2, 568(%rsp)
.Ltmp3949:
	.loc	43 37 12 is_stmt 1
	cmpq	$7, %r12
	movq	416(%rsp), %rcx
	je	.LBB32_220
.Ltmp3950:
	.loc	15 1298 42
	cmpq	$7, %r9
	je	.LBB32_182
	movl	92(%r10), %eax
	.loc	15 1298 28 is_stmt 0
	addq	32(%rsp), %rax
.Ltmp3951:
	.loc	15 1299 16 is_stmt 1
	cmpq	%r13, %rax
	movl	$0, %ecx
	cmovaeq	%r13, %rcx
	subq	%rcx, %rax
	.loc	15 1302 40
	imulq	%r12, %rax
	leaq	7(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB32_232
	.loc	15 0 25
	movq	%r12, %rdi
	.loc	15 1302 25
	vmovss	28(%rdx,%rax,4), %xmm2
	.loc	15 1302 13
	vmovss	%xmm2, 572(%rsp)
	movq	416(%rsp), %rcx
.Ltmp3952:
	.loc	15 0 13
.Ltmp3953:
	.p2align	4
.LBB32_220:
	.loc	1 551 14 is_stmt 1
	vmovaps	544(%rsp), %ymm7
.Ltmp3954:
.LBB32_221:
	.loc	15 0 0 is_stmt 0
	vbroadcastss	.LCPI32_3(%rip), %ymm2
	vmulps	%ymm2, %ymm10, %ymm2
	vroundps	$9, %ymm2, %ymm2
	vbroadcastss	.LCPI32_4(%rip), %ymm8
	vmulps	%ymm2, %ymm8, %ymm2
.Ltmp3955:
	.loc	50 48 14 is_stmt 1
	vaddps	%ymm0, %ymm2, %ymm0
.Ltmp3956:
	.loc	50 347 14
	vsubps	%ymm7, %ymm0, %ymm0
.Ltmp3957:
	.loc	15 1306 5
	vmovaps	%ymm0, 5216(%rsp)
.Ltmp3958:
	.loc	37 580 12
	movq	%rsi, %rdx
	subq	%rcx, %rdx
	jb	.LBB32_400
.Ltmp3959:
	.loc	37 451 16
	cmpq	$7, %rdx
	jbe	.LBB32_100
.Ltmp3960:
	.loc	15 1307 26
	movq	1896(%r11), %rax
.Ltmp3961:
	.loc	1 551 14
	vmovups	%ymm2, (%rax,%rcx,4)
.Ltmp3962:
	.loc	50 360 14
	vdivps	5248(%rsp), %ymm0, %ymm2
.Ltmp3963:
	.loc	15 1311 43
	vmovaps	5184(%rsp), %ymm7
.Ltmp3964:
	.loc	50 347 14
	vsubps	%ymm2, %ymm14, %ymm2
.Ltmp3965:
	.loc	50 347 14 is_stmt 0
	vsubps	%ymm7, %ymm2, %ymm8
.Ltmp3966:
	.loc	50 283 14 is_stmt 1
	vmulps	%ymm5, %ymm8, %ymm8
.Ltmp3967:
	.loc	50 48 14
	vaddps	%ymm7, %ymm8, %ymm7
.Ltmp3968:
	.loc	50 233 14
	vmaxps	%ymm7, %ymm2, %ymm2
.Ltmp3969:
	.loc	50 82 19
	vandps	%ymm2, %ymm9, %ymm7
.Ltmp3970:
	.loc	50 871 14
	vbroadcastss	.LCPI32_5(%rip), %ymm8
	vcmplt_oqps	%ymm8, %ymm7, %ymm7
.Ltmp3971:
	.loc	50 82 19
	vandnps	%ymm2, %ymm7, %ymm2
.Ltmp3972:
	.loc	15 1312 5
	vmovaps	%ymm2, 5184(%rsp)
.Ltmp3973:
	.loc	15 1315 28
	movq	1872(%r11), %rsi
	.loc	15 1315 44 is_stmt 0
	imulq	%rbx, %rdi
.Ltmp3974:
	.loc	37 568 12 is_stmt 1
	movq	%rsi, %rdx
	subq	%rdi, %rdx
	jb	.LBB32_231
.Ltmp3975:
	.loc	37 438 16
	cmpq	$7, %rdx
	jbe	.LBB32_100
.Ltmp3976:
	.loc	37 0 16 is_stmt 0
	movq	288(%rsp), %rsi
	incq	%rsi
	movq	480(%rsp), %rax
.Ltmp3977:
	leaq	(%r15,%rax,4), %rax
.Ltmp3978:
	vsubps	%ymm2, %ymm14, %ymm2
.Ltmp3979:
	.loc	15 1315 28 is_stmt 1
	movq	1864(%r11), %rcx
.Ltmp3980:
	.loc	1 551 14
	vmovups	(%rcx,%rdi,4), %ymm7
.Ltmp3981:
	.loc	1 551 14 is_stmt 0
	vmovups	(%rax), %ymm8
.Ltmp3982:
	.loc	50 283 14 is_stmt 1
	vmulps	%ymm7, %ymm2, %ymm2
	vmovaps	224(%rsp), %ymm9
.Ltmp3983:
	.loc	50 585 19
	vblendvps	%ymm9, %ymm7, %ymm2, %ymm2
.Ltmp3984:
	.loc	1 551 14
	vmovups	%ymm8, (%rcx,%rdi,4)
.Ltmp3985:
	.loc	15 1754 13
	incq	%rbx
	.loc	15 1755 16
	cmpq	1632(%r11), %rbx
.Ltmp3986:
	.loc	1 551 14
	vmovups	%ymm2, (%rax)
	movl	$0, %edx
.Ltmp3987:
	.loc	15 1755 16
	cmoveq	%rdx, %rbx
	movq	32(%rsp), %rax
	.loc	15 1758 13
	incq	%rax
	.loc	15 1759 16
	cmpq	%r13, %rax
	movl	$0, %ecx
	movq	%rcx, 16(%rsp)
	cmoveq	%rdx, %rax
	movq	%rax, 32(%rsp)
	movq	%rsi, %rax
.Ltmp3988:
	.loc	10 1916 50
	cmpq	752(%rsp), %rsi
	movq	208(%rsp), %r13
	movq	216(%rsp), %rdx
.Ltmp3989:
	.loc	11 900 12
	jne	.LBB32_96
	jmp	.LBB32_77
.Ltmp3990:
.LBB32_70:
	.loc	11 0 12 is_stmt 0
	movq	176(%rsp), %r14
	movq	200(%rsp), %r15
	movq	%r12, %rbx
.LBB32_71:
	leaq	3808(%rsp), %rdi
	movq	400(%rsp), %rsi
	.loc	15 1765 14 is_stmt 1
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	leaq	4544(%rsp), %rdi
	movq	392(%rsp), %rsi
	.loc	15 1766 15
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	movq	24(%rsp), %r9
	.loc	15 1767 5
	movl	%ebx, 1640(%r9)
	movq	32(%rsp), %rax
.Ltmp3991:
	.loc	15 0 0 is_stmt 0
	movl	%eax, 1644(%r9)
.Ltmp3992:
	.loc	15 2141 35 is_stmt 1
	cmpb	$0, 764(%rsp)
	je	.LBB32_263
.LBB32_377:
	.loc	15 0 35 is_stmt 0
	movq	400(%rsp), %rdi
	.loc	15 2142 26 is_stmt 1
	callq	*_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest@GOTPCREL(%rip)
	.loc	15 2142 16 is_stmt 0
	testb	%al, %al
	je	.LBB32_378
	.loc	15 0 16
	movq	392(%rsp), %rdi
	.loc	15 2143 27 is_stmt 1
	callq	*_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest@GOTPCREL(%rip)
	.loc	15 2143 16 is_stmt 0
	testb	%al, %al
	je	.LBB32_378
	.loc	15 0 16
	movq	2296(%rsp), %rsi
	cmpq	%r13, %rsi
	movq	192(%rsp), %r10
.Ltmp3993:
	.loc	14 1050 16 is_stmt 1
	ja	.LBB32_417
.Ltmp3994:
	.loc	14 0 16 is_stmt 0
	movq	%rsi, %rax
	movq	%r10, %rcx
	movq	24(%rsp), %r9
	.p2align	4
.LBB32_382:
.Ltmp3995:
	.loc	29 1504 12 is_stmt 1
	testq	%rax, %rax
	je	.LBB32_387
.Ltmp3996:
	.loc	10 1078 5
	cmpq	$32, %rax
	movl	$32, %edx
	cmovbq	%rax, %rdx
.Ltmp3997:
	.loc	12 961 18
	leal	(,%rdx,4), %edi
	xorl	%r11d, %r11d
	xorl	%r8d, %r8d
.Ltmp3998:
	.loc	12 0 18 is_stmt 0
.Ltmp3999:
	.p2align	4
.LBB32_384:
	.loc	34 134 13 is_stmt 1
	orl	(%rcx,%r8), %r11d
.Ltmp4000:
	.loc	16 1714 9
	addq	$4, %r8
	cmpq	%r8, %rdi
.Ltmp4001:
	.loc	17 180 28
	jne	.LBB32_384
.Ltmp4002:
	.loc	35 863 18
	leaq	(%rcx,%rdx,4), %rcx
.Ltmp4003:
	.loc	30 2054 74
	subq	%rdx, %rax
.Ltmp4004:
	.loc	34 136 12
	testl	%r11d, %r11d
	je	.LBB32_382
	.loc	34 0 12 is_stmt 0
	xorl	%ecx, %ecx
	.loc	34 136 12
	jmp	.LBB32_265
.Ltmp4005:
.LBB32_378:
	.loc	34 0 12
	xorl	%ecx, %ecx
	movq	24(%rsp), %r9
	jmp	.LBB32_264
.LBB32_100:
	vmovaps	96(%rsp), %ymm0
.Ltmp4006:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 4288(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp4007:
	.loc	15 746 0 is_stmt 0
	vmovaps	%ymm0, 5024(%rsp)
.Ltmp4008:
.LBB32_84:
	.loc	15 0 0
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_231:
	vmovaps	96(%rsp), %ymm0
.Ltmp4009:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 4288(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp4010:
	.loc	15 746 0 is_stmt 0
	vmovaps	%ymm0, 5024(%rsp)
.Ltmp4011:
	.loc	37 569 13 is_stmt 1
	leaq	.Lalloc_a07d19b424a92543209d516ece5bdf2a(%rip), %rcx
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4012:
.LBB32_242:
	.loc	15 1824 33
	movq	1976(%r8), %rcx
	movq	1984(%r8), %rax
.Ltmp4013:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp4014:
	.p2align	4
.LBB32_243:
.Ltmp4015:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp4016:
	.loc	17 180 28
	je	.LBB32_246
.Ltmp4017:
	.loc	17 0 28 is_stmt 0
	xorl	%esi, %esi
.Ltmp4018:
	.loc	15 687 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp4019:
	.loc	17 315 25
	jne	.LBB32_251
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB32_243
	jmp	.LBB32_251
.Ltmp4020:
.LBB32_387:
	.loc	37 438 16 is_stmt 1
	cmpq	%r14, %rsi
	ja	.LBB32_418
.Ltmp4021:
	.loc	37 0 16 is_stmt 0
	movq	%r15, %rax
	.p2align	4
.LBB32_389:
.Ltmp4022:
	.loc	34 131 18 is_stmt 1
	movq	%rsi, %rcx
.Ltmp4023:
	.loc	29 1504 12
	testq	%rsi, %rsi
	je	.LBB32_393
.Ltmp4024:
	.loc	10 1078 5
	cmpq	$32, %rcx
	movl	$32, %edx
	cmovbq	%rcx, %rdx
.Ltmp4025:
	.loc	12 961 18
	leal	(,%rdx,4), %edi
	xorl	%r11d, %r11d
	xorl	%r8d, %r8d
.Ltmp4026:
	.loc	12 0 18 is_stmt 0
.Ltmp4027:
	.p2align	4
.LBB32_391:
	.loc	34 134 13 is_stmt 1
	orl	(%rax,%r8), %r11d
.Ltmp4028:
	.loc	16 1714 9
	addq	$4, %r8
	cmpq	%r8, %rdi
.Ltmp4029:
	.loc	17 180 28
	jne	.LBB32_391
.Ltmp4030:
	.loc	35 863 18
	leaq	(%rax,%rdx,4), %rax
.Ltmp4031:
	.loc	30 2054 74
	movq	%rcx, %rsi
	subq	%rdx, %rsi
.Ltmp4032:
	.loc	34 136 12
	testl	%r11d, %r11d
	je	.LBB32_389
.Ltmp4033:
.LBB32_393:
	.loc	29 1504 12
	testq	%rcx, %rcx
	sete	%cl
.Ltmp4034:
	.loc	15 2141 35
	jmp	.LBB32_265
.LBB32_185:
	.loc	15 0 35 is_stmt 0
	vmovaps	96(%rsp), %ymm0
.Ltmp4035:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 4288(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp4036:
	.loc	15 746 0 is_stmt 0
	vmovaps	%ymm0, 5024(%rsp)
.Ltmp4037:
	.loc	37 569 13 is_stmt 1
	leaq	.Lalloc_f473a90cd9861be1576d1010d795a4d4(%rip), %rcx
	movq	480(%rsp), %rdi
.Ltmp4038:
	.loc	15 0 0 is_stmt 0
	movq	%r13, %rsi
	movq	%r13, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_135:
	vmovaps	96(%rsp), %ymm0
.Ltmp4039:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 4288(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp4040:
	.loc	15 746 0 is_stmt 0
	vmovaps	%ymm0, 5024(%rsp)
.Ltmp4041:
	.loc	37 581 13 is_stmt 1
	leaq	.Lalloc_9d2713d1692431af37d60082290049ed(%rip), %rcx
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4042:
.LBB32_398:
	.loc	37 0 13 is_stmt 0
	vmovaps	96(%rsp), %ymm0
.Ltmp4043:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 4288(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp4044:
	.loc	15 746 0 is_stmt 0
	vmovaps	%ymm0, 5024(%rsp)
.Ltmp4045:
	.loc	37 581 13 is_stmt 1
	leaq	.Lalloc_8002ed69501742f3ea2ea25eb68cd581(%rip), %rcx
	movq	%r15, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4046:
.LBB32_233:
	.loc	37 0 13 is_stmt 0
	vmovaps	96(%rsp), %ymm0
.Ltmp4047:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 4288(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp4048:
	.loc	15 746 0 is_stmt 0
	vmovaps	%ymm0, 5024(%rsp)
.Ltmp4049:
	.loc	37 569 13 is_stmt 1
	leaq	.Lalloc_59d870179e725b921b323beec09137a0(%rip), %rcx
	movq	480(%rsp), %rdi
.Ltmp4050:
	.loc	15 0 0 is_stmt 0
	movq	%r14, %rsi
	movq	%r14, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_230:
	vmovaps	96(%rsp), %ymm0
.Ltmp4051:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 4288(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp4052:
	.loc	15 746 0 is_stmt 0
	vmovaps	%ymm0, 5024(%rsp)
	movq	%rcx, %rdi
.Ltmp4053:
	.loc	37 581 13 is_stmt 1
	leaq	.Lalloc_9d2713d1692431af37d60082290049ed(%rip), %rcx
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4054:
.LBB32_400:
	.loc	37 0 13 is_stmt 0
	vmovaps	96(%rsp), %ymm0
.Ltmp4055:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 4288(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp4056:
	.loc	15 746 0 is_stmt 0
	vmovaps	%ymm0, 5024(%rsp)
	movq	%rcx, %rdi
.Ltmp4057:
	.loc	37 581 13 is_stmt 1
	leaq	.Lalloc_8002ed69501742f3ea2ea25eb68cd581(%rip), %rcx
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4058:
.LBB32_246:
	.loc	15 1825 33
	movq	1992(%r8), %rcx
	movq	2000(%r8), %rax
.Ltmp4059:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp4060:
	.p2align	4
.LBB32_247:
.Ltmp4061:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp4062:
	.loc	17 180 28
	je	.LBB32_248
.Ltmp4063:
	.loc	17 0 28 is_stmt 0
	xorl	%esi, %esi
.Ltmp4064:
	.loc	15 687 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp4065:
	.loc	17 315 25
	jne	.LBB32_251
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB32_247
	jmp	.LBB32_251
.Ltmp4066:
.LBB32_248:
	.loc	17 0 25
	movb	$1, %sil
.LBB32_251:
	movl	%esi, 2304(%rsp)
.Ltmp4067:
	.loc	15 1828 19 is_stmt 1
	movzbl	1536(%r8), %r14d
.Ltmp4068:
	.loc	15 1829 21
	movzbl	1537(%r8), %eax
	movb	%al, 32(%rsp)
.Ltmp4069:
	.loc	15 1830 16
	movq	1624(%r8), %r12
.Ltmp4070:
	.loc	15 1831 16
	movq	1632(%r8), %rbx
.Ltmp4071:
	.loc	15 1832 27
	movl	1640(%r8), %eax
	movq	%rax, 1184(%rsp)
.Ltmp4072:
	.loc	15 1833 27
	movl	1644(%r8), %eax
	movq	%rax, 1120(%rsp)
	leaq	6376(%rsp), %rdi
	movq	memset@GOTPCREL(%rip), %r15
	movl	$1024, %edx
	xorl	%esi, %esi
	movq	%r8, %r13
	callq	*%r15
	leaq	5344(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*%r15
	leaq	992(%rsp), %rdi
	movq	400(%rsp), %rsi
.Ltmp4073:
	.loc	15 1840 32
	movq	%r12, %rdx
	movq	%rbx, 16(%rsp)
	movq	%rbx, %rcx
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_
.Ltmp4074:
	.loc	15 1841 33
	movq	1624(%r13), %rdx
	movq	1632(%r13), %rcx
	leaq	544(%rsp), %rdi
	movq	392(%rsp), %rsi
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_
	movq	184(%rsp), %rax
.Ltmp4075:
	.loc	14 3758 16
	leaq	31(%rax), %rdi
	shrq	$5, %rdi
.Ltmp4076:
	.loc	40 446 20
	je	.LBB32_252
.Ltmp4077:
	.loc	50 871 14
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm1
.Ltmp4078:
	.loc	50 713 19
	vpcmpeqd	%ymm0, %ymm0, %ymm0
	vxorps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm1, 768(%rsp)
	vmovaps	%ymm1, 672(%rsp)
	testb	%r14b, %r14b
	movq	176(%rsp), %r14
	movq	208(%rsp), %r13
	movq	200(%rsp), %r15
	jne	.LBB32_286
.Ltmp4079:
	.loc	50 0 19 is_stmt 0
	vmovaps	%ymm0, 672(%rsp)
.LBB32_286:
	cmpb	$0, 32(%rsp)
	movq	24(%rsp), %r11
	jne	.LBB32_288
	vmovaps	%ymm0, 768(%rsp)
.LBB32_288:
	movl	$32, %r10d
	movabsq	$2305843009213693944, %rax
	addq	$7, %rax
	movq	%rax, 528(%rsp)
	movq	%r15, %r9
	movq	%r14, %rsi
	movq	192(%rsp), %rbx
	movq	%r13, %rdx
	movq	$0, 352(%rsp)
	movq	184(%rsp), %rcx
	movq	%rcx, %rax
	xorl	%r8d, %r8d
.LBB32_291:
	movq	%rdi, 408(%rsp)
.Ltmp4080:
	.loc	14 2584 13 is_stmt 1
	cmpq	$32, %rax
	movl	$32, %edi
	movq	%rax, 536(%rsp)
	cmovbq	%rax, %rdi
	cmpq	$1, %rdi
	movq	%rdi, 752(%rsp)
	movq	%rdi, %rax
	adcq	$0, %rax
	shll	$3, %eax
.Ltmp4081:
	.loc	10 1916 50
	movq	%rcx, %rdi
	subq	%r8, %rdi
.Ltmp4082:
	.loc	10 1078 5
	cmpq	$32, %rdi
	cmovaeq	%r10, %rdi
	movq	%rdi, 216(%rsp)
.Ltmp4083:
	.loc	15 1612 23
	vmovaps	2336(%rsp), %ymm4
	vmovaps	2368(%rsp), %ymm5
	vmovaps	2400(%rsp), %ymm0
	vmovaps	2432(%rsp), %ymm1
	vmovaps	2464(%rsp), %ymm2
	vmovaps	2496(%rsp), %ymm10
	vmovaps	2528(%rsp), %ymm3
	vmovaps	2560(%rsp), %ymm7
	vmovaps	2592(%rsp), %ymm11
	vmovaps	2624(%rsp), %ymm14
	vmovaps	2656(%rsp), %ymm8
	movq	%r8, 2080(%rsp)
.Ltmp4084:
	.loc	10 1916 50
	cmpq	%r8, %rcx
	movq	%rdx, 744(%rsp)
.Ltmp4085:
	.loc	11 900 12
	jne	.LBB32_293
.Ltmp4086:
	.loc	15 0 0 is_stmt 0
	vmovaps	2688(%rsp), %ymm6
	vmovaps	%ymm6, 448(%rsp)
.Ltmp4087:
	.loc	11 900 12
	jmp	.LBB32_297
.Ltmp4088:
.LBB32_293:
	.loc	11 0 12
	vmovaps	(%r11), %ymm6
	vmovaps	%ymm6, 480(%rsp)
	vmovaps	32(%r11), %ymm6
	vmovaps	%ymm6, 704(%rsp)
	vmovaps	64(%r11), %ymm6
	vmovaps	%ymm6, 1376(%rsp)
	vmovaps	%ymm8, %ymm15
	vmovaps	%ymm5, %ymm13
	vmovaps	96(%r11), %ymm5
	vmovaps	%ymm5, 416(%rsp)
	vmovaps	128(%r11), %ymm5
	vmovaps	%ymm5, 1344(%rsp)
	vmovaps	160(%r11), %ymm5
	vmovaps	%ymm5, 224(%rsp)
	vmovaps	192(%r11), %ymm5
	vmovaps	%ymm5, 288(%rsp)
	vmovaps	224(%r11), %ymm5
	vmovaps	%ymm5, 256(%rsp)
	vmovaps	256(%r11), %ymm5
	vmovaps	%ymm5, 320(%rsp)
	vmovaps	288(%r11), %ymm5
	vmovaps	%ymm5, 2112(%rsp)
	vmovaps	320(%r11), %ymm5
	vmovaps	%ymm5, 960(%rsp)
	xorl	%ecx, %ecx
	vmovaps	%ymm14, %ymm12
	vmovaps	%ymm11, %ymm9
	vmovaps	%ymm7, %ymm6
	vmovaps	352(%r11), %ymm5
	vmovaps	%ymm5, 128(%rsp)
	vmovaps	384(%r11), %ymm5
	vmovaps	%ymm5, 96(%rsp)
	vmovaps	416(%r11), %ymm5
	vmovaps	%ymm5, 928(%rsp)
	vmovaps	448(%r11), %ymm5
	vmovaps	%ymm5, 896(%rsp)
	vmovaps	480(%r11), %ymm5
	vmovaps	%ymm5, 864(%rsp)
	vmovaps	512(%r11), %ymm5
	vmovaps	%ymm5, 832(%rsp)
	vmovaps	544(%r11), %ymm5
	vmovaps	%ymm5, 800(%rsp)
	vmovaps	576(%r11), %ymm5
	vmovaps	%ymm5, 2048(%rsp)
	vmovaps	608(%r11), %ymm5
	vmovaps	%ymm5, 2016(%rsp)
	vmovaps	640(%r11), %ymm5
	vmovaps	%ymm5, 1984(%rsp)
	vmovaps	672(%r11), %ymm5
	vmovaps	%ymm5, 1952(%rsp)
	vmovaps	704(%r11), %ymm5
	vmovaps	%ymm5, 1920(%rsp)
	vmovaps	736(%r11), %ymm5
	vmovaps	%ymm5, 1312(%rsp)
	vmovaps	768(%r11), %ymm5
	vmovaps	%ymm5, 1888(%rsp)
	vmovaps	800(%r11), %ymm5
	vmovaps	%ymm5, 1856(%rsp)
	vmovaps	832(%r11), %ymm5
	vmovaps	%ymm5, 1824(%rsp)
	vmovaps	864(%r11), %ymm5
	vmovaps	%ymm5, 1792(%rsp)
	vmovaps	896(%r11), %ymm5
	vmovaps	%ymm5, 1760(%rsp)
	vmovaps	928(%r11), %ymm5
	vmovaps	%ymm5, 1280(%rsp)
	vmovaps	960(%r11), %ymm5
	vmovaps	%ymm5, 1728(%rsp)
	vmovaps	992(%r11), %ymm5
	vmovaps	%ymm5, 1696(%rsp)
	vmovaps	1024(%r11), %ymm5
	vmovaps	%ymm5, 1664(%rsp)
	vmovaps	1056(%r11), %ymm5
	vmovaps	%ymm5, 1632(%rsp)
	vmovaps	1088(%r11), %ymm5
	vmovaps	%ymm5, 1600(%rsp)
	vmovaps	1120(%r11), %ymm5
	vmovaps	%ymm5, 1152(%rsp)
	vmovaps	1152(%r11), %ymm5
	vmovaps	%ymm5, 1248(%rsp)
	vmovaps	1184(%r11), %ymm5
	vmovaps	%ymm5, 1568(%rsp)
	vmovaps	1216(%r11), %ymm5
	vmovaps	%ymm5, 1536(%rsp)
	vmovaps	1248(%r11), %ymm5
	vmovaps	%ymm5, 1504(%rsp)
	vmovaps	1280(%r11), %ymm5
	vmovaps	%ymm5, 1216(%rsp)
	vmovaps	1312(%r11), %ymm5
	vmovaps	%ymm5, 1472(%rsp)
	vmovaps	1344(%r11), %ymm5
	vmovaps	%ymm5, 1440(%rsp)
	vmovaps	1376(%r11), %ymm5
	vmovaps	%ymm5, 1408(%rsp)
	vmovaps	1408(%r11), %ymm5
	vmovaps	%ymm5, 2240(%rsp)
	vmovaps	1440(%r11), %ymm5
	vmovaps	%ymm5, 2208(%rsp)
	vmovaps	1472(%r11), %ymm5
	vmovaps	%ymm5, 2176(%rsp)
	vmovaps	1504(%r11), %ymm5
	vmovaps	%ymm5, 2144(%rsp)
	.p2align	4
.LBB32_294:
	movq	352(%rsp), %rdi
.Ltmp4089:
	.loc	37 568 12 is_stmt 1
	addq	%rcx, %rdi
	cmpq	%r13, %rdi
	ja	.LBB32_85
.Ltmp4090:
	.loc	37 438 16
	cmpq	$7, %rdx
	jbe	.LBB32_84
.Ltmp4091:
	.loc	37 0 16 is_stmt 0
	vmovaps	%ymm9, %ymm8
	vmovaps	%ymm2, %ymm9
	vmovaps	%ymm1, %ymm2
	vmovaps	%ymm0, %ymm1
	vmovaps	%ymm13, %ymm0
	vmovaps	%ymm12, 32(%rsp)
	vmovaps	%ymm4, 64(%rsp)
	vmovaps	%ymm6, %ymm14
	vmovaps	%ymm3, %ymm11
.Ltmp4092:
	.loc	1 551 14 is_stmt 1
	vmovups	(%rbx,%rcx,4), %ymm7
.Ltmp4093:
	.loc	50 82 19
	vbroadcastss	.LCPI32_0(%rip), %ymm13
.Ltmp4094:
	.loc	50 283 14
	vmulps	480(%rsp), %ymm7, %ymm6
	vmovaps	%ymm10, %ymm3
	vmovaps	%ymm9, %ymm5
	vmovaps	%ymm2, %ymm10
	vmovaps	%ymm1, %ymm2
	vmovaps	%ymm0, %ymm4
	vxorps	%xmm0, %xmm0, %xmm0
.Ltmp4095:
	.loc	50 48 14
	vaddps	%ymm0, %ymm6, %ymm6
.Ltmp4096:
	.loc	50 283 14
	vmulps	704(%rsp), %ymm7, %ymm9
.Ltmp4097:
	.loc	50 48 14
	vaddps	%ymm0, %ymm9, %ymm9
.Ltmp4098:
	.loc	50 283 14
	vmulps	1376(%rsp), %ymm7, %ymm12
.Ltmp4099:
	.loc	50 48 14
	vaddps	%ymm0, %ymm12, %ymm12
	vmovaps	%ymm15, %ymm1
.Ltmp4100:
	.loc	50 283 14
	vmulps	416(%rsp), %ymm7, %ymm15
.Ltmp4101:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm15
	vmovaps	64(%rsp), %ymm0
.Ltmp4102:
	.loc	50 283 14
	vmulps	1344(%rsp), %ymm0, %ymm0
.Ltmp4103:
	.loc	50 48 14
	vaddps	%ymm0, %ymm6, %ymm0
	vmovaps	64(%rsp), %ymm6
.Ltmp4104:
	.loc	50 283 14
	vmulps	224(%rsp), %ymm6, %ymm6
.Ltmp4105:
	.loc	50 48 14
	vaddps	%ymm6, %ymm9, %ymm6
	vmovaps	64(%rsp), %ymm9
.Ltmp4106:
	.loc	50 283 14
	vmulps	288(%rsp), %ymm9, %ymm9
.Ltmp4107:
	.loc	50 48 14
	vaddps	%ymm9, %ymm12, %ymm9
	vmovaps	64(%rsp), %ymm12
.Ltmp4108:
	.loc	50 283 14
	vmulps	256(%rsp), %ymm12, %ymm12
.Ltmp4109:
	.loc	50 48 14
	vaddps	%ymm12, %ymm15, %ymm12
.Ltmp4110:
	.loc	50 283 14
	vmulps	320(%rsp), %ymm4, %ymm15
.Ltmp4111:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp4112:
	.loc	50 283 14
	vmulps	2112(%rsp), %ymm4, %ymm15
.Ltmp4113:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp4114:
	.loc	50 283 14
	vmulps	960(%rsp), %ymm4, %ymm15
.Ltmp4115:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp4116:
	.loc	50 283 14
	vmulps	128(%rsp), %ymm4, %ymm15
.Ltmp4117:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp4118:
	.loc	50 283 14
	vmulps	96(%rsp), %ymm2, %ymm15
.Ltmp4119:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp4120:
	.loc	50 283 14
	vmulps	928(%rsp), %ymm2, %ymm15
.Ltmp4121:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp4122:
	.loc	50 283 14
	vmulps	896(%rsp), %ymm2, %ymm15
.Ltmp4123:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp4124:
	.loc	50 283 14
	vmulps	864(%rsp), %ymm2, %ymm15
.Ltmp4125:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp4126:
	.loc	50 283 14
	vmulps	832(%rsp), %ymm10, %ymm15
.Ltmp4127:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp4128:
	.loc	50 283 14
	vmulps	800(%rsp), %ymm10, %ymm15
.Ltmp4129:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp4130:
	.loc	50 283 14
	vmulps	2048(%rsp), %ymm10, %ymm15
.Ltmp4131:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp4132:
	.loc	50 283 14
	vmulps	2016(%rsp), %ymm10, %ymm15
.Ltmp4133:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp4134:
	.loc	50 283 14
	vmulps	1984(%rsp), %ymm5, %ymm15
.Ltmp4135:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp4136:
	.loc	50 283 14
	vmulps	1952(%rsp), %ymm5, %ymm15
.Ltmp4137:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp4138:
	.loc	50 283 14
	vmulps	1920(%rsp), %ymm5, %ymm15
.Ltmp4139:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp4140:
	.loc	50 283 14
	vmulps	1312(%rsp), %ymm5, %ymm15
.Ltmp4141:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp4142:
	.loc	50 283 14
	vmulps	1888(%rsp), %ymm3, %ymm15
.Ltmp4143:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp4144:
	.loc	50 283 14
	vmulps	1856(%rsp), %ymm3, %ymm15
.Ltmp4145:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp4146:
	.loc	50 283 14
	vmulps	1824(%rsp), %ymm3, %ymm15
.Ltmp4147:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp4148:
	.loc	50 283 14
	vmulps	1792(%rsp), %ymm3, %ymm15
.Ltmp4149:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp4150:
	.loc	50 283 14
	vmulps	1760(%rsp), %ymm11, %ymm15
.Ltmp4151:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp4152:
	.loc	50 283 14
	vmulps	1280(%rsp), %ymm11, %ymm15
.Ltmp4153:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp4154:
	.loc	50 283 14
	vmulps	1728(%rsp), %ymm11, %ymm15
.Ltmp4155:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp4156:
	.loc	50 283 14
	vmulps	1696(%rsp), %ymm11, %ymm15
.Ltmp4157:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp4158:
	.loc	50 283 14
	vmulps	1664(%rsp), %ymm14, %ymm15
.Ltmp4159:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp4160:
	.loc	50 283 14
	vmulps	1632(%rsp), %ymm14, %ymm15
.Ltmp4161:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp4162:
	.loc	50 283 14
	vmulps	1600(%rsp), %ymm14, %ymm15
.Ltmp4163:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp4164:
	.loc	50 283 14
	vmulps	1152(%rsp), %ymm14, %ymm15
.Ltmp4165:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp4166:
	.loc	50 283 14
	vmulps	1248(%rsp), %ymm8, %ymm15
.Ltmp4167:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp4168:
	.loc	50 283 14
	vmulps	1568(%rsp), %ymm8, %ymm15
.Ltmp4169:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp4170:
	.loc	50 283 14
	vmulps	1536(%rsp), %ymm8, %ymm15
.Ltmp4171:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp4172:
	.loc	50 283 14
	vmulps	1504(%rsp), %ymm8, %ymm15
.Ltmp4173:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
	vmovaps	1216(%rsp), %ymm15
.Ltmp4174:
	.loc	50 283 14
	vmulps	32(%rsp), %ymm15, %ymm15
.Ltmp4175:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
	vmovaps	1472(%rsp), %ymm15
.Ltmp4176:
	.loc	50 283 14
	vmulps	32(%rsp), %ymm15, %ymm15
.Ltmp4177:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
	vmovaps	1440(%rsp), %ymm15
.Ltmp4178:
	.loc	50 283 14
	vmulps	32(%rsp), %ymm15, %ymm15
.Ltmp4179:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
	vmovaps	1408(%rsp), %ymm15
.Ltmp4180:
	.loc	50 283 14
	vmulps	32(%rsp), %ymm15, %ymm15
.Ltmp4181:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp4182:
	.loc	50 283 14
	vmulps	2240(%rsp), %ymm1, %ymm15
.Ltmp4183:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp4184:
	.loc	50 283 14
	vmulps	2208(%rsp), %ymm1, %ymm15
.Ltmp4185:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp4186:
	.loc	50 283 14
	vmulps	2176(%rsp), %ymm1, %ymm15
.Ltmp4187:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
	vmovaps	%ymm1, 448(%rsp)
.Ltmp4188:
	.loc	50 283 14
	vmulps	2144(%rsp), %ymm1, %ymm15
.Ltmp4189:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp4190:
	.loc	50 82 19
	vandps	%ymm3, %ymm13, %ymm15
.Ltmp4191:
	.loc	50 82 19 is_stmt 0
	vandps	%ymm0, %ymm13, %ymm0
.Ltmp4192:
	.loc	50 233 14 is_stmt 1
	vmaxps	%ymm0, %ymm15, %ymm0
.Ltmp4193:
	.loc	50 82 19
	vandps	%ymm6, %ymm13, %ymm6
.Ltmp4194:
	.loc	50 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp4195:
	.loc	50 82 19
	vandps	%ymm13, %ymm9, %ymm6
.Ltmp4196:
	.loc	50 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp4197:
	.loc	50 82 19
	vandps	%ymm13, %ymm12, %ymm6
.Ltmp4198:
	.loc	50 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp4199:
	.loc	1 551 14
	vmovups	%ymm0, 6376(%rsp,%rcx,4)
	vmovaps	%ymm4, %ymm0
	vmovaps	%ymm2, %ymm1
	vmovaps	%ymm10, %ymm2
	vmovaps	%ymm5, %ymm10
	vmovaps	%ymm7, %ymm4
	vmovaps	%ymm11, %ymm7
	vmovaps	%ymm14, %ymm11
	vmovaps	%ymm8, %ymm14
	vmovaps	64(%rsp), %ymm5
	vmovaps	32(%rsp), %ymm8
.Ltmp4200:
	.loc	10 1916 50
	addq	$8, %rcx
	addq	$-8, %rdx
	vmovaps	%ymm5, %ymm13
	vmovaps	%ymm8, %ymm15
	vmovaps	%ymm14, %ymm12
	vmovaps	%ymm11, %ymm9
	vmovaps	%ymm7, %ymm6
	cmpq	%rcx, %rax
.Ltmp4201:
	.loc	11 900 12
	jne	.LBB32_294
.Ltmp4202:
.LBB32_297:
	.loc	15 1618 5
	vmovaps	%ymm4, 2336(%rsp)
	vmovaps	%ymm5, 2368(%rsp)
	vmovaps	%ymm0, 2400(%rsp)
	vmovaps	%ymm1, 2432(%rsp)
	vmovaps	%ymm2, 2464(%rsp)
	vmovaps	%ymm10, 2496(%rsp)
	vmovaps	%ymm3, 2528(%rsp)
	vmovaps	%ymm7, 2560(%rsp)
	vmovaps	%ymm11, 2592(%rsp)
	vmovaps	%ymm14, 2624(%rsp)
	vmovaps	%ymm8, 2656(%rsp)
	vmovaps	448(%rsp), %ymm0
	vmovaps	%ymm0, 2688(%rsp)
.Ltmp4203:
	.loc	15 1612 23
	vmovaps	3072(%rsp), %ymm4
	vmovaps	3104(%rsp), %ymm5
	vmovaps	3136(%rsp), %ymm0
	vmovaps	3168(%rsp), %ymm1
	vmovaps	3200(%rsp), %ymm2
	vmovaps	3232(%rsp), %ymm10
	vmovaps	3264(%rsp), %ymm3
	vmovaps	3296(%rsp), %ymm7
	vmovaps	3328(%rsp), %ymm11
	vmovaps	3360(%rsp), %ymm14
	vmovaps	3392(%rsp), %ymm8
	movq	2080(%rsp), %rcx
.Ltmp4204:
	.loc	10 1916 50
	cmpq	%rcx, 184(%rsp)
.Ltmp4205:
	.loc	11 900 12
	jne	.LBB32_299
.Ltmp4206:
	.loc	15 0 0 is_stmt 0
	vmovaps	3424(%rsp), %ymm6
	vmovaps	%ymm6, 448(%rsp)
.Ltmp4207:
	.loc	11 900 12
	jmp	.LBB32_303
.Ltmp4208:
.LBB32_299:
	.loc	11 0 12
	vmovaps	(%r11), %ymm6
	vmovaps	%ymm6, 480(%rsp)
	vmovaps	32(%r11), %ymm6
	vmovaps	%ymm6, 704(%rsp)
	vmovaps	64(%r11), %ymm6
	vmovaps	%ymm6, 1376(%rsp)
	vmovaps	%ymm8, %ymm15
	vmovaps	%ymm5, %ymm13
	vmovaps	96(%r11), %ymm5
	vmovaps	%ymm5, 416(%rsp)
	vmovaps	128(%r11), %ymm5
	vmovaps	%ymm5, 1344(%rsp)
	vmovaps	160(%r11), %ymm5
	vmovaps	%ymm5, 224(%rsp)
	vmovaps	192(%r11), %ymm5
	vmovaps	%ymm5, 288(%rsp)
	vmovaps	224(%r11), %ymm5
	vmovaps	%ymm5, 256(%rsp)
	vmovaps	256(%r11), %ymm5
	vmovaps	%ymm5, 320(%rsp)
	vmovaps	288(%r11), %ymm5
	vmovaps	%ymm5, 2112(%rsp)
	vmovaps	320(%r11), %ymm5
	vmovaps	%ymm5, 960(%rsp)
	movq	%rsi, %rdx
	xorl	%ecx, %ecx
	vmovaps	%ymm14, %ymm12
	vmovaps	%ymm11, %ymm9
	vmovaps	%ymm7, %ymm6
	vmovaps	352(%r11), %ymm5
	vmovaps	%ymm5, 128(%rsp)
	vmovaps	384(%r11), %ymm5
	vmovaps	%ymm5, 96(%rsp)
	vmovaps	416(%r11), %ymm5
	vmovaps	%ymm5, 928(%rsp)
	vmovaps	448(%r11), %ymm5
	vmovaps	%ymm5, 896(%rsp)
	vmovaps	480(%r11), %ymm5
	vmovaps	%ymm5, 864(%rsp)
	vmovaps	512(%r11), %ymm5
	vmovaps	%ymm5, 832(%rsp)
	vmovaps	544(%r11), %ymm5
	vmovaps	%ymm5, 800(%rsp)
	vmovaps	576(%r11), %ymm5
	vmovaps	%ymm5, 2048(%rsp)
	vmovaps	608(%r11), %ymm5
	vmovaps	%ymm5, 2016(%rsp)
	vmovaps	640(%r11), %ymm5
	vmovaps	%ymm5, 1984(%rsp)
	vmovaps	672(%r11), %ymm5
	vmovaps	%ymm5, 1952(%rsp)
	vmovaps	704(%r11), %ymm5
	vmovaps	%ymm5, 1920(%rsp)
	vmovaps	736(%r11), %ymm5
	vmovaps	%ymm5, 1312(%rsp)
	vmovaps	768(%r11), %ymm5
	vmovaps	%ymm5, 1888(%rsp)
	vmovaps	800(%r11), %ymm5
	vmovaps	%ymm5, 1856(%rsp)
	vmovaps	832(%r11), %ymm5
	vmovaps	%ymm5, 1824(%rsp)
	vmovaps	864(%r11), %ymm5
	vmovaps	%ymm5, 1792(%rsp)
	vmovaps	896(%r11), %ymm5
	vmovaps	%ymm5, 1760(%rsp)
	vmovaps	928(%r11), %ymm5
	vmovaps	%ymm5, 1280(%rsp)
	vmovaps	960(%r11), %ymm5
	vmovaps	%ymm5, 1728(%rsp)
	vmovaps	992(%r11), %ymm5
	vmovaps	%ymm5, 1696(%rsp)
	vmovaps	1024(%r11), %ymm5
	vmovaps	%ymm5, 1664(%rsp)
	vmovaps	1056(%r11), %ymm5
	vmovaps	%ymm5, 1632(%rsp)
	vmovaps	1088(%r11), %ymm5
	vmovaps	%ymm5, 1600(%rsp)
	vmovaps	1120(%r11), %ymm5
	vmovaps	%ymm5, 1152(%rsp)
	vmovaps	1152(%r11), %ymm5
	vmovaps	%ymm5, 1248(%rsp)
	vmovaps	1184(%r11), %ymm5
	vmovaps	%ymm5, 1568(%rsp)
	vmovaps	1216(%r11), %ymm5
	vmovaps	%ymm5, 1536(%rsp)
	vmovaps	1248(%r11), %ymm5
	vmovaps	%ymm5, 1504(%rsp)
	vmovaps	1280(%r11), %ymm5
	vmovaps	%ymm5, 1216(%rsp)
	vmovaps	1312(%r11), %ymm5
	vmovaps	%ymm5, 1472(%rsp)
	vmovaps	1344(%r11), %ymm5
	vmovaps	%ymm5, 1440(%rsp)
	vmovaps	1376(%r11), %ymm5
	vmovaps	%ymm5, 1408(%rsp)
	vmovaps	1408(%r11), %ymm5
	vmovaps	%ymm5, 2240(%rsp)
	vmovaps	1440(%r11), %ymm5
	vmovaps	%ymm5, 2208(%rsp)
	vmovaps	1472(%r11), %ymm5
	vmovaps	%ymm5, 2176(%rsp)
	vmovaps	1504(%r11), %ymm5
	vmovaps	%ymm5, 2144(%rsp)
	.p2align	4
.LBB32_300:
	movq	352(%rsp), %rdi
.Ltmp4209:
	.loc	37 568 12 is_stmt 1
	addq	%rcx, %rdi
	cmpq	%r14, %rdi
	ja	.LBB32_101
.Ltmp4210:
	.loc	37 438 16
	cmpq	$7, %rdx
	jbe	.LBB32_84
.Ltmp4211:
	.loc	37 0 16 is_stmt 0
	vmovaps	%ymm9, %ymm8
	vmovaps	%ymm2, %ymm9
	vmovaps	%ymm1, %ymm2
	vmovaps	%ymm0, %ymm1
	vmovaps	%ymm13, %ymm0
	vmovaps	%ymm12, 32(%rsp)
	vmovaps	%ymm4, 64(%rsp)
	vmovaps	%ymm6, %ymm14
	vmovaps	%ymm3, %ymm11
.Ltmp4212:
	.loc	1 551 14 is_stmt 1
	vmovups	(%r9,%rcx,4), %ymm7
.Ltmp4213:
	.loc	50 82 19
	vbroadcastss	.LCPI32_0(%rip), %ymm13
.Ltmp4214:
	.loc	50 283 14
	vmulps	480(%rsp), %ymm7, %ymm6
	vmovaps	%ymm10, %ymm3
	vmovaps	%ymm9, %ymm5
	vmovaps	%ymm2, %ymm10
	vmovaps	%ymm1, %ymm2
	vmovaps	%ymm0, %ymm4
	vxorps	%xmm0, %xmm0, %xmm0
.Ltmp4215:
	.loc	50 48 14
	vaddps	%ymm0, %ymm6, %ymm6
.Ltmp4216:
	.loc	50 283 14
	vmulps	704(%rsp), %ymm7, %ymm9
.Ltmp4217:
	.loc	50 48 14
	vaddps	%ymm0, %ymm9, %ymm9
.Ltmp4218:
	.loc	50 283 14
	vmulps	1376(%rsp), %ymm7, %ymm12
.Ltmp4219:
	.loc	50 48 14
	vaddps	%ymm0, %ymm12, %ymm12
	vmovaps	%ymm15, %ymm1
.Ltmp4220:
	.loc	50 283 14
	vmulps	416(%rsp), %ymm7, %ymm15
.Ltmp4221:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm15
	vmovaps	64(%rsp), %ymm0
.Ltmp4222:
	.loc	50 283 14
	vmulps	1344(%rsp), %ymm0, %ymm0
.Ltmp4223:
	.loc	50 48 14
	vaddps	%ymm0, %ymm6, %ymm0
	vmovaps	64(%rsp), %ymm6
.Ltmp4224:
	.loc	50 283 14
	vmulps	224(%rsp), %ymm6, %ymm6
.Ltmp4225:
	.loc	50 48 14
	vaddps	%ymm6, %ymm9, %ymm6
	vmovaps	64(%rsp), %ymm9
.Ltmp4226:
	.loc	50 283 14
	vmulps	288(%rsp), %ymm9, %ymm9
.Ltmp4227:
	.loc	50 48 14
	vaddps	%ymm9, %ymm12, %ymm9
	vmovaps	64(%rsp), %ymm12
.Ltmp4228:
	.loc	50 283 14
	vmulps	256(%rsp), %ymm12, %ymm12
.Ltmp4229:
	.loc	50 48 14
	vaddps	%ymm12, %ymm15, %ymm12
.Ltmp4230:
	.loc	50 283 14
	vmulps	320(%rsp), %ymm4, %ymm15
.Ltmp4231:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp4232:
	.loc	50 283 14
	vmulps	2112(%rsp), %ymm4, %ymm15
.Ltmp4233:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp4234:
	.loc	50 283 14
	vmulps	960(%rsp), %ymm4, %ymm15
.Ltmp4235:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp4236:
	.loc	50 283 14
	vmulps	128(%rsp), %ymm4, %ymm15
.Ltmp4237:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp4238:
	.loc	50 283 14
	vmulps	96(%rsp), %ymm2, %ymm15
.Ltmp4239:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp4240:
	.loc	50 283 14
	vmulps	928(%rsp), %ymm2, %ymm15
.Ltmp4241:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp4242:
	.loc	50 283 14
	vmulps	896(%rsp), %ymm2, %ymm15
.Ltmp4243:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp4244:
	.loc	50 283 14
	vmulps	864(%rsp), %ymm2, %ymm15
.Ltmp4245:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp4246:
	.loc	50 283 14
	vmulps	832(%rsp), %ymm10, %ymm15
.Ltmp4247:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp4248:
	.loc	50 283 14
	vmulps	800(%rsp), %ymm10, %ymm15
.Ltmp4249:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp4250:
	.loc	50 283 14
	vmulps	2048(%rsp), %ymm10, %ymm15
.Ltmp4251:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp4252:
	.loc	50 283 14
	vmulps	2016(%rsp), %ymm10, %ymm15
.Ltmp4253:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp4254:
	.loc	50 283 14
	vmulps	1984(%rsp), %ymm5, %ymm15
.Ltmp4255:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp4256:
	.loc	50 283 14
	vmulps	1952(%rsp), %ymm5, %ymm15
.Ltmp4257:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp4258:
	.loc	50 283 14
	vmulps	1920(%rsp), %ymm5, %ymm15
.Ltmp4259:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp4260:
	.loc	50 283 14
	vmulps	1312(%rsp), %ymm5, %ymm15
.Ltmp4261:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp4262:
	.loc	50 283 14
	vmulps	1888(%rsp), %ymm3, %ymm15
.Ltmp4263:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp4264:
	.loc	50 283 14
	vmulps	1856(%rsp), %ymm3, %ymm15
.Ltmp4265:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp4266:
	.loc	50 283 14
	vmulps	1824(%rsp), %ymm3, %ymm15
.Ltmp4267:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp4268:
	.loc	50 283 14
	vmulps	1792(%rsp), %ymm3, %ymm15
.Ltmp4269:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp4270:
	.loc	50 283 14
	vmulps	1760(%rsp), %ymm11, %ymm15
.Ltmp4271:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp4272:
	.loc	50 283 14
	vmulps	1280(%rsp), %ymm11, %ymm15
.Ltmp4273:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp4274:
	.loc	50 283 14
	vmulps	1728(%rsp), %ymm11, %ymm15
.Ltmp4275:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp4276:
	.loc	50 283 14
	vmulps	1696(%rsp), %ymm11, %ymm15
.Ltmp4277:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp4278:
	.loc	50 283 14
	vmulps	1664(%rsp), %ymm14, %ymm15
.Ltmp4279:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp4280:
	.loc	50 283 14
	vmulps	1632(%rsp), %ymm14, %ymm15
.Ltmp4281:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp4282:
	.loc	50 283 14
	vmulps	1600(%rsp), %ymm14, %ymm15
.Ltmp4283:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp4284:
	.loc	50 283 14
	vmulps	1152(%rsp), %ymm14, %ymm15
.Ltmp4285:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp4286:
	.loc	50 283 14
	vmulps	1248(%rsp), %ymm8, %ymm15
.Ltmp4287:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp4288:
	.loc	50 283 14
	vmulps	1568(%rsp), %ymm8, %ymm15
.Ltmp4289:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp4290:
	.loc	50 283 14
	vmulps	1536(%rsp), %ymm8, %ymm15
.Ltmp4291:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
.Ltmp4292:
	.loc	50 283 14
	vmulps	1504(%rsp), %ymm8, %ymm15
.Ltmp4293:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
	vmovaps	1216(%rsp), %ymm15
.Ltmp4294:
	.loc	50 283 14
	vmulps	32(%rsp), %ymm15, %ymm15
.Ltmp4295:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
	vmovaps	1472(%rsp), %ymm15
.Ltmp4296:
	.loc	50 283 14
	vmulps	32(%rsp), %ymm15, %ymm15
.Ltmp4297:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
	vmovaps	1440(%rsp), %ymm15
.Ltmp4298:
	.loc	50 283 14
	vmulps	32(%rsp), %ymm15, %ymm15
.Ltmp4299:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
	vmovaps	1408(%rsp), %ymm15
.Ltmp4300:
	.loc	50 283 14
	vmulps	32(%rsp), %ymm15, %ymm15
.Ltmp4301:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp4302:
	.loc	50 283 14
	vmulps	2240(%rsp), %ymm1, %ymm15
.Ltmp4303:
	.loc	50 48 14
	vaddps	%ymm0, %ymm15, %ymm0
.Ltmp4304:
	.loc	50 283 14
	vmulps	2208(%rsp), %ymm1, %ymm15
.Ltmp4305:
	.loc	50 48 14
	vaddps	%ymm6, %ymm15, %ymm6
.Ltmp4306:
	.loc	50 283 14
	vmulps	2176(%rsp), %ymm1, %ymm15
.Ltmp4307:
	.loc	50 48 14
	vaddps	%ymm15, %ymm9, %ymm9
	vmovaps	%ymm1, 448(%rsp)
.Ltmp4308:
	.loc	50 283 14
	vmulps	2144(%rsp), %ymm1, %ymm15
.Ltmp4309:
	.loc	50 48 14
	vaddps	%ymm15, %ymm12, %ymm12
.Ltmp4310:
	.loc	50 82 19
	vandps	%ymm3, %ymm13, %ymm15
.Ltmp4311:
	.loc	50 82 19 is_stmt 0
	vandps	%ymm0, %ymm13, %ymm0
.Ltmp4312:
	.loc	50 233 14 is_stmt 1
	vmaxps	%ymm0, %ymm15, %ymm0
.Ltmp4313:
	.loc	50 82 19
	vandps	%ymm6, %ymm13, %ymm6
.Ltmp4314:
	.loc	50 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp4315:
	.loc	50 82 19
	vandps	%ymm13, %ymm9, %ymm6
.Ltmp4316:
	.loc	50 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp4317:
	.loc	50 82 19
	vandps	%ymm13, %ymm12, %ymm6
.Ltmp4318:
	.loc	50 233 14
	vmaxps	%ymm6, %ymm0, %ymm0
.Ltmp4319:
	.loc	1 551 14
	vmovups	%ymm0, 5344(%rsp,%rcx,4)
	vmovaps	%ymm4, %ymm0
	vmovaps	%ymm2, %ymm1
	vmovaps	%ymm10, %ymm2
	vmovaps	%ymm5, %ymm10
	vmovaps	%ymm7, %ymm4
	vmovaps	%ymm11, %ymm7
	vmovaps	%ymm14, %ymm11
	vmovaps	%ymm8, %ymm14
	vmovaps	64(%rsp), %ymm5
	vmovaps	32(%rsp), %ymm8
.Ltmp4320:
	.loc	10 1916 50
	addq	$8, %rcx
	addq	$-8, %rdx
	vmovaps	%ymm5, %ymm13
	vmovaps	%ymm8, %ymm15
	vmovaps	%ymm14, %ymm12
	vmovaps	%ymm11, %ymm9
	vmovaps	%ymm7, %ymm6
	cmpq	%rcx, %rax
.Ltmp4321:
	.loc	11 900 12
	jne	.LBB32_300
.Ltmp4322:
.LBB32_303:
	.loc	15 1618 5
	vmovaps	%ymm4, 3072(%rsp)
	vmovaps	%ymm5, 3104(%rsp)
	vmovaps	%ymm0, 3136(%rsp)
	vmovaps	%ymm1, 3168(%rsp)
	vmovaps	%ymm2, 3200(%rsp)
	vmovaps	%ymm10, 3232(%rsp)
	vmovaps	%ymm3, 3264(%rsp)
	vmovaps	%ymm7, 3296(%rsp)
	vmovaps	%ymm11, 3328(%rsp)
	vmovaps	%ymm14, 3360(%rsp)
	vmovaps	%ymm8, 3392(%rsp)
	vmovaps	448(%rsp), %ymm0
	vmovaps	%ymm0, 3424(%rsp)
	movq	2080(%rsp), %r8
.Ltmp4323:
	.loc	10 1916 50
	cmpq	%r8, 184(%rsp)
.Ltmp4324:
	.loc	15 1865 19
	jne	.LBB32_305
.Ltmp4325:
	.loc	15 0 19 is_stmt 0
	movq	408(%rsp), %rdi
.LBB32_290:
	addq	$32, %r8
	decq	%rdi
	movq	536(%rsp), %rax
.Ltmp4326:
	.loc	40 446 20 is_stmt 1
	addq	$-32, %rax
	addq	$256, 352(%rsp)
	movq	744(%rsp), %rdx
	addq	$-256, %rdx
	addq	$1024, %rbx
	addq	$-256, %rsi
	addq	$1024, %r9
	testq	%rdi, %rdi
	movq	184(%rsp), %rcx
	movl	$32, %r10d
	jne	.LBB32_291
	jmp	.LBB32_253
.Ltmp4327:
.LBB32_305:
	.loc	40 0 20 is_stmt 0
	movq	%rbx, 1408(%rsp)
	movq	%rsi, 1440(%rsp)
	movq	%r9, 1472(%rsp)
.Ltmp4328:
	.loc	15 1871 21 is_stmt 1
	movq	1080(%rsp), %r13
	movq	1088(%rsp), %rax
	movq	%rax, 1568(%rsp)
	.loc	15 1872 21
	movq	632(%rsp), %rax
	movq	%rax, 1536(%rsp)
	movq	640(%rsp), %rax
	movq	%rax, 1504(%rsp)
	movq	1024(%rsp), %rcx
	movq	1032(%rsp), %rdx
	movq	1072(%rsp), %rax
	movq	%rax, 800(%rsp)
	movq	1040(%rsp), %rax
	movq	%rax, 1920(%rsp)
	movq	1048(%rsp), %rax
	movq	%rax, 1952(%rsp)
	movq	1064(%rsp), %rax
	movq	%rax, 1312(%rsp)
	movq	1056(%rsp), %rax
	movq	%rax, 1888(%rsp)
	movq	576(%rsp), %r8
	movq	584(%rsp), %rax
	vmovaps	992(%rsp), %ymm11
	vmovaps	544(%rsp), %ymm8
	movq	624(%rsp), %rsi
	movq	%rsi, 1856(%rsp)
	movq	592(%rsp), %rsi
	movq	%rsi, 1824(%rsp)
	movq	600(%rsp), %rsi
	movq	%rsi, 1792(%rsp)
	movq	616(%rsp), %rsi
	movq	%rsi, 1280(%rsp)
	movq	608(%rsp), %rsi
	movq	%rsi, 1728(%rsp)
	movl	1096(%rsp), %esi
	movl	%esi, 64(%rsp)
	movl	648(%rsp), %esi
	movl	%esi, 32(%rsp)
	xorl	%r15d, %r15d
	movq	1184(%rsp), %rdi
	movq	1120(%rsp), %r9
	xorl	%ebx, %ebx
	movq	216(%rsp), %r10
	movq	%r13, 1216(%rsp)
	movq	%rdx, 1760(%rsp)
.LBB32_306:
	.loc	15 0 21 is_stmt 0
	movq	%r15, 448(%rsp)
	.loc	15 1870 21 is_stmt 1
	subq	%r15, %r10
	movq	%r11, %rsi
.Ltmp4329:
	.loc	15 1470 16
	movq	1624(%r11), %r11
	movq	%r13, %r14
.Ltmp4330:
	.loc	15 1471 16
	movq	1632(%rsi), %r13
.Ltmp4331:
	.loc	15 1472 25
	leaq	1(%r9), %rsi
.Ltmp4332:
	.loc	15 1043 8
	cmpq	%r11, %rsi
	movq	%r11, %rsi
	cmovbq	%rbx, %rsi
	negq	%rsi
	addq	%r9, %rsi
	incq	%rsi
	movq	%rsi, 704(%rsp)
.Ltmp4333:
	.loc	15 1473 28
	leaq	(%r14,%r9), %r15
.Ltmp4334:
	.loc	15 1043 8
	cmpq	%r11, %r15
	movq	%r11, %rsi
	cmovbq	%rbx, %rsi
	subq	%rsi, %r15
	movq	1536(%rsp), %rsi
.Ltmp4335:
	.loc	15 1474 29
	leaq	(%rsi,%r9), %r14
.Ltmp4336:
	.loc	15 1043 8
	cmpq	%r11, %r14
	movq	%r11, %rsi
	cmovbq	%rbx, %rsi
	subq	%rsi, %r14
	movq	%r14, 416(%rsp)
	movq	1568(%rsp), %rsi
.Ltmp4337:
	.loc	15 1475 33
	leaq	(%rsi,%r9), %r14
.Ltmp4338:
	.loc	15 1043 8
	cmpq	%r11, %r14
	movq	%r11, %rsi
	cmovbq	%rbx, %rsi
	subq	%rsi, %r14
	movq	%r14, 1376(%rsp)
	movq	1504(%rsp), %rsi
.Ltmp4339:
	.loc	15 1476 34
	leaq	(%rsi,%r9), %r14
.Ltmp4340:
	.loc	15 1043 8
	cmpq	%r11, %r14
	movq	%r11, %rsi
	cmovbq	%rbx, %rsi
	subq	%rsi, %r14
	movq	%r14, 1344(%rsp)
.Ltmp4341:
	.loc	15 1478 14
	movq	%r11, %r14
	movq	%r9, 1120(%rsp)
	subq	%r9, %r14
.Ltmp4342:
	.loc	10 1078 5
	cmpq	%r10, %r14
	cmovbq	%r14, %r10
	movq	%rdi, 1184(%rsp)
.Ltmp4343:
	.loc	15 1479 14
	subq	%rdi, %r13
.Ltmp4344:
	.loc	10 1078 5
	cmpq	%r10, %r13
	movq	%r13, 480(%rsp)
	cmovbq	%r13, %r10
.Ltmp4345:
	.loc	15 1480 14
	movq	%r11, %r13
	movq	704(%rsp), %rsi
	subq	%rsi, %r13
.Ltmp4346:
	.loc	10 1078 5
	cmpq	%r10, %r13
	cmovbq	%r13, %r10
.Ltmp4347:
	.loc	15 1481 14
	movq	%r11, %rbx
	movq	%r15, 2112(%rsp)
	subq	%r15, %rbx
.Ltmp4348:
	.loc	10 1078 5
	cmpq	%r10, %rbx
	cmovbq	%rbx, %r10
.Ltmp4349:
	.loc	15 1482 14
	movq	%r11, %r15
	movq	416(%rsp), %rsi
	subq	%rsi, %r15
.Ltmp4350:
	.loc	10 1078 5
	cmpq	%r10, %r15
	cmovbq	%r15, %r10
.Ltmp4351:
	.loc	15 1483 14
	movq	%r11, %r9
	movq	1376(%rsp), %rsi
	subq	%rsi, %r9
.Ltmp4352:
	.loc	10 1078 5
	cmpq	%r10, %r9
	cmovbq	%r9, %r10
	movq	1344(%rsp), %rsi
.Ltmp4353:
	.loc	15 1484 14
	subq	%rsi, %r11
.Ltmp4354:
	.loc	10 1078 5
	cmpq	%r10, %r11
	cmovbq	%r11, %r10
	movq	2080(%rsp), %rsi
	movq	448(%rsp), %rdi
.Ltmp4355:
	.loc	15 1876 28
	addq	%rsi, %rdi
	movq	%r10, 1152(%rsp)
.Ltmp4356:
	.loc	15 1878 55
	leaq	(%r10,%rdi), %rsi
.Ltmp4357:
	.loc	15 1876 28
	shlq	$3, %rdi
.Ltmp4358:
	.loc	15 1878 55
	shlq	$3, %rsi
.Ltmp4359:
	.loc	14 1050 16
	cmpq	%rdi, %rsi
	jb	.LBB32_401
	cmpq	208(%rsp), %rsi
	ja	.LBB32_401
.Ltmp4360:
	.loc	37 451 16
	cmpq	176(%rsp), %rsi
	ja	.LBB32_402
.Ltmp4361:
	.loc	37 0 16 is_stmt 0
	movq	448(%rsp), %rsi
.Ltmp4362:
	.loc	15 1880 46 is_stmt 1
	leaq	(,%rsi,8), %r10
	movq	%r10, 224(%rsp)
	movq	1152(%rsp), %r10
	.loc	15 1880 61 is_stmt 0
	addq	%r10, %rsi
	movq	%rsi, 1248(%rsp)
	cmpq	$33, %rsi
.Ltmp4363:
	.loc	14 1050 16 is_stmt 1
	jae	.LBB32_403
.Ltmp4364:
	.loc	32 304 12
	testq	%r10, %r10
	je	.LBB32_311
.Ltmp4365:
	.loc	15 1890 85
	cmpq	%r15, %rbx
	cmovbq	%rbx, %r15
	cmpq	%r9, %r15
	cmovaeq	%r9, %r15
	cmpq	%r11, %r15
	cmovaeq	%r11, %r15
	cmpq	%r13, %r15
	cmovaeq	%r13, %r15
	cmpq	%r14, %r15
	cmovaeq	%r14, %r15
	movq	192(%rsp), %r9
.Ltmp4366:
	.loc	15 0 0 is_stmt 0
	leaq	(%r9,%rdi,4), %rsi
	movq	%rsi, 832(%rsp)
	movq	200(%rsp), %r9
.Ltmp4367:
	leaq	(%r9,%rdi,4), %rsi
	movq	%rsi, 2016(%rsp)
	movq	480(%rsp), %rsi
.Ltmp4368:
	.loc	15 1890 85
	cmpq	%rsi, %r15
	cmovaeq	%rsi, %r15
	movq	752(%rsp), %rsi
	subq	448(%rsp), %rsi
	movq	224(%rsp), %r9
.Ltmp4369:
	.loc	15 0 0
	leaq	(%rsp,%r9,4), %rdi
	addq	$6376, %rdi
	movq	%rdi, 928(%rsp)
.Ltmp4370:
	leaq	(%rsp,%r9,4), %rdi
	addq	$5344, %rdi
	movq	%rdi, 896(%rsp)
.Ltmp4371:
	.loc	15 1890 85
	cmpq	%rsi, %r15
	cmovbq	%r15, %rsi
	andq	528(%rsp), %rsi
	movq	%rsi, 1984(%rsp)
	vmovaps	2720(%rsp), %ymm13
	vmovaps	2752(%rsp), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
	vmovaps	2784(%rsp), %ymm0
	vmovaps	%ymm0, 256(%rsp)
	vmovaps	2816(%rsp), %ymm0
	vmovaps	%ymm0, 320(%rsp)
	vmovaps	2880(%rsp), %ymm0
	vmovaps	%ymm0, 1664(%rsp)
	vmovaps	3456(%rsp), %ymm10
	vmovaps	3488(%rsp), %ymm0
	vmovaps	%ymm0, 1632(%rsp)
	vmovaps	3520(%rsp), %ymm0
	vmovaps	%ymm0, 224(%rsp)
	vmovaps	3552(%rsp), %ymm0
	vmovaps	%ymm0, 288(%rsp)
	vmovaps	3616(%rsp), %ymm0
	vmovaps	%ymm0, 1600(%rsp)
	vmovaps	3040(%rsp), %ymm0
	vmovaps	%ymm0, 864(%rsp)
	vmovaps	3776(%rsp), %ymm0
	vmovaps	%ymm0, 2048(%rsp)
	vmovaps	2944(%rsp), %ymm0
	vmovaps	%ymm0, 960(%rsp)
	vmovaps	2848(%rsp), %ymm4
	vmovaps	2912(%rsp), %ymm0
	vmovaps	%ymm0, 128(%rsp)
	vmovaps	3680(%rsp), %ymm0
	vmovaps	%ymm0, 96(%rsp)
	xorl	%r13d, %r13d
	vmovaps	3584(%rsp), %ymm15
	vmovaps	3648(%rsp), %ymm2
	vmovaps	3008(%rsp), %ymm6
	vmovaps	3744(%rsp), %ymm9
	.loc	15 0 85
.Ltmp4372:
	.p2align	4
.LBB32_313:
	.loc	15 1890 85 is_stmt 1
	cmpb	$0, 2304(%rsp)
	movq	1120(%rsp), %rsi
	jne	.LBB32_315
	.loc	15 0 85 is_stmt 0
	vbroadcastss	.LCPI32_1(%rip), %ymm1
.Ltmp4373:
	.loc	50 347 14 is_stmt 1
	vaddps	320(%rsp), %ymm1, %ymm0
	vxorps	%xmm3, %xmm3, %xmm3
.Ltmp4374:
	.loc	50 233 14
	vmaxps	%ymm3, %ymm0, %ymm0
	vmovaps	%ymm0, 320(%rsp)
.Ltmp4375:
	.loc	50 871 14
	vcmpgt_oqps	%ymm3, %ymm0, %ymm0
	vmovaps	256(%rsp), %ymm5
.Ltmp4376:
	.loc	50 48 14
	vaddps	%ymm5, %ymm13, %ymm7
	vmovaps	1696(%rsp), %ymm12
.Ltmp4377:
	.loc	50 585 19
	vblendvps	%ymm0, %ymm7, %ymm12, %ymm13
.Ltmp4378:
	.loc	50 585 19 is_stmt 0
	vblendvps	%ymm0, %ymm5, %ymm3, %ymm5
	vmovaps	%ymm5, 256(%rsp)
.Ltmp4379:
	.loc	50 347 14 is_stmt 1
	vaddps	960(%rsp), %ymm1, %ymm0
.Ltmp4380:
	.loc	50 233 14
	vmaxps	%ymm3, %ymm0, %ymm12
.Ltmp4381:
	.loc	50 871 14
	vcmpgt_oqps	%ymm3, %ymm12, %ymm0
	vmovaps	128(%rsp), %ymm14
.Ltmp4382:
	.loc	50 48 14
	vaddps	%ymm4, %ymm14, %ymm4
	vmovaps	1664(%rsp), %ymm5
.Ltmp4383:
	.loc	50 585 19
	vblendvps	%ymm0, %ymm4, %ymm5, %ymm4
.Ltmp4384:
	.loc	50 585 19 is_stmt 0
	vblendvps	%ymm0, %ymm14, %ymm3, %ymm14
.Ltmp4385:
	.loc	50 347 14 is_stmt 1
	vaddps	288(%rsp), %ymm1, %ymm0
.Ltmp4386:
	.loc	50 233 14
	vmaxps	%ymm3, %ymm0, %ymm0
	vmovaps	%ymm0, 288(%rsp)
.Ltmp4387:
	.loc	50 871 14
	vcmpgt_oqps	%ymm3, %ymm0, %ymm0
	vmovaps	224(%rsp), %ymm5
.Ltmp4388:
	.loc	50 48 14
	vaddps	%ymm5, %ymm10, %ymm7
	vmovaps	1632(%rsp), %ymm10
.Ltmp4389:
	.loc	50 585 19
	vblendvps	%ymm0, %ymm7, %ymm10, %ymm10
.Ltmp4390:
	.loc	50 585 19 is_stmt 0
	vblendvps	%ymm0, %ymm5, %ymm3, %ymm5
	vmovaps	%ymm5, 224(%rsp)
.Ltmp4391:
	.loc	50 347 14 is_stmt 1
	vaddps	96(%rsp), %ymm1, %ymm0
.Ltmp4392:
	.loc	50 233 14
	vmaxps	%ymm3, %ymm0, %ymm5
.Ltmp4393:
	.loc	50 871 14
	vcmpgt_oqps	%ymm3, %ymm5, %ymm0
.Ltmp4394:
	.loc	50 48 14
	vaddps	%ymm2, %ymm15, %ymm7
	vmovaps	1600(%rsp), %ymm1
.Ltmp4395:
	.loc	50 585 19
	vblendvps	%ymm0, %ymm7, %ymm1, %ymm15
.Ltmp4396:
	.loc	50 585 19 is_stmt 0
	vblendvps	%ymm0, %ymm2, %ymm3, %ymm2
	vmovaps	%ymm12, 960(%rsp)
.Ltmp4397:
	.loc	15 746 9 is_stmt 1
	vmovaps	%ymm12, 2944(%rsp)
.Ltmp4398:
	.loc	15 748 9
	vmovaps	%ymm4, 2848(%rsp)
	vmovaps	%ymm14, 128(%rsp)
	.loc	15 749 9
	vmovaps	%ymm14, 2912(%rsp)
	vmovaps	%ymm5, 96(%rsp)
.Ltmp4399:
	.loc	15 746 9
	vmovaps	%ymm5, 3680(%rsp)
.Ltmp4400:
	.loc	15 748 9
	vmovaps	%ymm15, 3584(%rsp)
	.loc	15 749 9
	vmovaps	%ymm2, 3648(%rsp)
.Ltmp4401:
.LBB32_315:
	.loc	15 0 9 is_stmt 0
	vbroadcastss	.LCPI32_2(%rip), %ymm5
	movl	64(%rsp), %r14d
.Ltmp4402:
	.loc	15 1397 26 is_stmt 1
	addq	%r13, %rsi
.Ltmp4403:
	.loc	15 1032 16
	leaq	(,%rsi,8), %r9
.Ltmp4404:
	.loc	15 1033 33
	leaq	8(,%rsi,8), %r15
.Ltmp4405:
	.loc	14 1050 16
	leaq	7(,%rsi,8), %rsi
	cmpq	%rdx, %rsi
	jae	.LBB32_316
.Ltmp4406:
	.loc	15 0 0 is_stmt 0
	leaq	(,%r13,8), %rsi
	movq	928(%rsp), %rdi
.Ltmp4407:
	vmovups	(%rdi,%rsi,4), %ymm0
	movq	896(%rsp), %rdi
.Ltmp4408:
	vmovups	(%rdi,%rsi,4), %ymm7
.Ltmp4409:
	vmaxps	%ymm0, %ymm7, %ymm12
	vmovaps	672(%rsp), %ymm3
.Ltmp4410:
	.loc	50 585 19 is_stmt 1
	vblendvps	%ymm3, %ymm12, %ymm0, %ymm0
.Ltmp4411:
	.loc	50 360 14
	vdivps	%ymm0, %ymm13, %ymm14
	movq	2112(%rsp), %rsi
.Ltmp4412:
	.loc	15 0 0 is_stmt 0
	addq	%r13, %rsi
.Ltmp4413:
	.loc	50 871 14 is_stmt 1
	vcmpgt_oqps	%ymm13, %ymm0, %ymm0
.Ltmp4414:
	.loc	50 585 19
	vblendvps	%ymm0, %ymm14, %ymm5, %ymm0
.Ltmp4415:
	.loc	1 551 14
	vmovups	%ymm0, (%rcx,%r9,4)
.Ltmp4416:
	.loc	15 1025 16
	leaq	(,%rsi,8), %rdi
.Ltmp4417:
	.loc	14 1050 16
	leaq	7(,%rsi,8), %r10
	cmpq	%rdx, %r10
	jae	.LBB32_320
.Ltmp4418:
	.loc	1 551 14
	vmovups	(%rcx,%rdi,4), %ymm0
	vmovaps	%ymm0, %ymm14
.Ltmp4419:
	.loc	15 1101 22
	testl	%r14d, %r14d
	je	.LBB32_324
.Ltmp4420:
	.loc	50 257 14
	vminps	%ymm0, %ymm11, %ymm14
.Ltmp4421:
.LBB32_324:
	.loc	50 0 14 is_stmt 0
	movq	704(%rsp), %rdi
	leaq	(%rdi,%r13), %r10
.Ltmp4422:
	movl	%r14d, %r11d
.Ltmp4423:
	.loc	15 1107 20 is_stmt 1
	incq	%r11
	movq	800(%rsp), %rdi
	movq	%rdi, %rbx
	cmpq	%rdi, %r11
.Ltmp4424:
	.loc	15 1108 22
	jne	.LBB32_325
	.loc	15 0 22 is_stmt 0
.Ltmp4425:
	.p2align	4
.LBB32_328:
.Ltmp4426:
	.loc	15 1025 16 is_stmt 1
	leaq	(,%rsi,8), %rdi
.Ltmp4427:
	.loc	14 1050 16
	leaq	7(,%rsi,8), %r11
	cmpq	%rdx, %r11
	jae	.LBB32_326
.Ltmp4428:
	.loc	50 257 14
	vminps	(%rcx,%rdi,4), %ymm0, %ymm0
.Ltmp4429:
	.loc	1 551 14
	vmovups	%ymm0, (%rcx,%rdi,4)
.Ltmp4430:
	.loc	15 1119 16
	testq	%rsi, %rsi
	cmoveq	%r12, %rsi
	.loc	15 1122 13
	decq	%rsi
.Ltmp4431:
	.loc	10 1916 50
	decq	%rbx
.Ltmp4432:
	.loc	11 900 12
	jne	.LBB32_328
.Ltmp4433:
	.loc	11 0 12 is_stmt 0
	xorl	%r11d, %r11d
	vmovaps	%ymm14, %ymm0
	jmp	.LBB32_331
	.p2align	4
.LBB32_325:
.Ltmp4434:
	.loc	15 1025 16 is_stmt 1
	leaq	(,%r10,8), %rdi
.Ltmp4435:
	.loc	14 1050 16
	leaq	7(,%r10,8), %rsi
	cmpq	%rdx, %rsi
	jae	.LBB32_326
.Ltmp4436:
	.loc	1 551 14
	vmovups	(%rcx,%rdi,4), %ymm0
.Ltmp4437:
	.loc	50 257 14
	vminps	%ymm14, %ymm0, %ymm0
.Ltmp4438:
.LBB32_331:
	.loc	50 0 14 is_stmt 0
	movq	1376(%rsp), %rsi
	leaq	(%rsi,%r13), %rdi
.Ltmp4439:
	.loc	14 1050 16 is_stmt 1
	leaq	7(,%rdi,8), %rsi
.Ltmp4440:
	.loc	15 1025 16
	shlq	$3, %rdi
	movq	1952(%rsp), %rbx
.Ltmp4441:
	.loc	14 1050 16
	cmpq	%rbx, %rsi
	movq	1920(%rsp), %r14
	jae	.LBB32_404
.Ltmp4442:
	.loc	15 0 0 is_stmt 0
	vbroadcastss	.LCPI32_3(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
	vroundps	$9, %ymm0, %ymm0
	vbroadcastss	.LCPI32_4(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
.Ltmp4443:
	.loc	50 48 14 is_stmt 1
	vaddps	%ymm6, %ymm0, %ymm6
.Ltmp4444:
	.loc	50 347 14
	vsubps	(%r14,%rdi,4), %ymm6, %ymm6
.Ltmp4445:
	.loc	15 1549 5
	vmovaps	%ymm6, 3008(%rsp)
	cmpq	%rbx, %r15
.Ltmp4446:
	.loc	14 1050 16
	ja	.LBB32_405
.Ltmp4447:
	.loc	14 0 16 is_stmt 0
	movq	1184(%rsp), %rsi
	leaq	(%rsi,%r13), %rdi
.Ltmp4448:
	.loc	1 551 14 is_stmt 1
	vmovups	%ymm0, (%r14,%r9,4)
.Ltmp4449:
	.loc	50 360 14
	vdivps	864(%rsp), %ymm6, %ymm0
.Ltmp4450:
	.loc	15 1554 43
	vmovaps	2976(%rsp), %ymm11
.Ltmp4451:
	.loc	50 347 14
	vsubps	%ymm0, %ymm5, %ymm0
.Ltmp4452:
	.loc	50 347 14 is_stmt 0
	vsubps	%ymm11, %ymm0, %ymm3
.Ltmp4453:
	.loc	50 283 14 is_stmt 1
	vmulps	%ymm3, %ymm4, %ymm3
.Ltmp4454:
	.loc	50 48 14
	vaddps	%ymm3, %ymm11, %ymm3
.Ltmp4455:
	.loc	50 233 14
	vmaxps	%ymm3, %ymm0, %ymm0
.Ltmp4456:
	.loc	50 82 19
	vbroadcastss	.LCPI32_0(%rip), %ymm11
	vandps	%ymm0, %ymm11, %ymm3
.Ltmp4457:
	.loc	50 871 14
	vbroadcastss	.LCPI32_5(%rip), %ymm1
	vcmplt_oqps	%ymm1, %ymm3, %ymm3
.Ltmp4458:
	.loc	50 82 19
	vandnps	%ymm0, %ymm3, %ymm0
.Ltmp4459:
	.loc	15 1555 5
	vmovaps	%ymm0, 2976(%rsp)
.Ltmp4460:
	.loc	15 1025 16
	leaq	(,%rdi,8), %r14
.Ltmp4461:
	.loc	15 1026 25
	leaq	8(,%rdi,8), %rsi
.Ltmp4462:
	.loc	14 1050 16
	leaq	7(,%rdi,8), %rdi
	cmpq	1312(%rsp), %rdi
	jae	.LBB32_406
.Ltmp4463:
	.loc	14 0 16 is_stmt 0
	movq	%rsi, 448(%rsp)
	movl	%r11d, 64(%rsp)
	movq	%r15, %rbx
	movq	%r13, %r15
	shlq	$5, %r15
	movq	832(%rsp), %rdi
	addq	%r15, %rdi
.Ltmp4464:
	.loc	50 347 14 is_stmt 1
	vsubps	%ymm0, %ymm5, %ymm0
	movq	1888(%rsp), %r11
.Ltmp4465:
	.loc	1 551 14
	vmovups	(%r11,%r14,4), %ymm3
	vmovaps	%ymm5, %ymm1
.Ltmp4466:
	.loc	1 551 14 is_stmt 0
	vmovups	(%rdi), %ymm5
	vmovups	%ymm5, (%r11,%r14,4)
.Ltmp4467:
	.loc	50 283 14 is_stmt 1
	vmulps	%ymm3, %ymm0, %ymm0
	vmovaps	768(%rsp), %ymm5
.Ltmp4468:
	.loc	50 585 19
	vblendvps	%ymm5, %ymm3, %ymm0, %ymm0
.Ltmp4469:
	.loc	1 551 14
	vmovups	%ymm0, (%rdi)
	movq	%rbx, %rsi
	cmpq	%rax, %rbx
.Ltmp4470:
	.loc	14 1050 16
	ja	.LBB32_407
.Ltmp4471:
	.loc	14 0 16 is_stmt 0
	vmovaps	672(%rsp), %ymm0
	vblendvps	%ymm0, %ymm12, %ymm7, %ymm0
.Ltmp4472:
	.loc	50 360 14 is_stmt 1
	vdivps	%ymm0, %ymm10, %ymm3
	movq	416(%rsp), %rdi
.Ltmp4473:
	.loc	15 0 0 is_stmt 0
	leaq	(%rdi,%r13), %rbx
.Ltmp4474:
	.loc	50 871 14 is_stmt 1
	vcmpgt_oqps	%ymm10, %ymm0, %ymm0
.Ltmp4475:
	.loc	50 585 19
	vblendvps	%ymm0, %ymm3, %ymm1, %ymm0
.Ltmp4476:
	.loc	1 551 14
	vmovups	%ymm0, (%r8,%r9,4)
.Ltmp4477:
	.loc	15 1025 16
	leaq	(,%rbx,8), %rdi
.Ltmp4478:
	.loc	14 1050 16
	leaq	7(,%rbx,8), %r11
	cmpq	%rax, %r11
	jae	.LBB32_408
.Ltmp4479:
	.loc	14 0 16 is_stmt 0
	vmovaps	%ymm1, %ymm5
	movq	%r9, 480(%rsp)
	movq	%r12, %r9
.Ltmp4480:
	.loc	1 551 14 is_stmt 1
	vmovups	(%r8,%rdi,4), %ymm0
	vmovaps	%ymm0, %ymm7
.Ltmp4481:
	.loc	15 1101 22
	cmpl	$0, 32(%rsp)
	je	.LBB32_338
.Ltmp4482:
	.loc	50 257 14
	vminps	%ymm0, %ymm8, %ymm7
.Ltmp4483:
.LBB32_338:
	.loc	15 0 0 is_stmt 0
	movl	32(%rsp), %r11d
.Ltmp4484:
	.loc	15 1107 20 is_stmt 1
	incq	%r11
	movq	1856(%rsp), %r12
	movq	%r12, %rdi
	cmpq	%r12, %r11
	movq	%r9, %r12
	movq	1824(%rsp), %r9
.Ltmp4485:
	.loc	15 1108 22
	jne	.LBB32_339
	.loc	15 0 22 is_stmt 0
.Ltmp4486:
	.p2align	4
.LBB32_341:
.Ltmp4487:
	.loc	15 1025 16 is_stmt 1
	leaq	(,%rbx,8), %r10
.Ltmp4488:
	.loc	14 1050 16
	leaq	7(,%rbx,8), %r11
	cmpq	%rax, %r11
	jae	.LBB32_410
.Ltmp4489:
	.loc	50 257 14
	vminps	(%r8,%r10,4), %ymm0, %ymm0
.Ltmp4490:
	.loc	1 551 14
	vmovups	%ymm0, (%r8,%r10,4)
.Ltmp4491:
	.loc	15 1119 16
	testq	%rbx, %rbx
	cmoveq	%r12, %rbx
	.loc	15 1122 13
	decq	%rbx
.Ltmp4492:
	.loc	10 1916 50
	decq	%rdi
.Ltmp4493:
	.loc	11 900 12
	jne	.LBB32_341
.Ltmp4494:
	.loc	11 0 12 is_stmt 0
	movl	$0, 32(%rsp)
	vmovaps	%ymm7, %ymm0
	jmp	.LBB32_344
	.p2align	4
.LBB32_339:
.Ltmp4495:
	.loc	14 1050 16 is_stmt 1
	leaq	7(,%r10,8), %rdi
.Ltmp4496:
	.loc	15 1025 16
	shlq	$3, %r10
.Ltmp4497:
	.loc	14 1050 16
	cmpq	%rax, %rdi
	jae	.LBB32_409
.Ltmp4498:
	.loc	1 551 14
	vmovups	(%r8,%r10,4), %ymm0
.Ltmp4499:
	.loc	50 257 14
	vminps	%ymm7, %ymm0, %ymm0
	movl	%r11d, 32(%rsp)
.Ltmp4500:
.LBB32_344:
	.loc	50 0 14 is_stmt 0
	movq	1344(%rsp), %rdi
	leaq	(%rdi,%r13), %r10
.Ltmp4501:
	.loc	14 1050 16 is_stmt 1
	leaq	7(,%r10,8), %rdi
.Ltmp4502:
	.loc	15 1025 16
	shlq	$3, %r10
	movq	1792(%rsp), %rbx
.Ltmp4503:
	.loc	14 1050 16
	cmpq	%rbx, %rdi
	movq	24(%rsp), %r11
	movq	1760(%rsp), %rdx
	jae	.LBB32_411
.Ltmp4504:
	.loc	15 0 0 is_stmt 0
	vbroadcastss	.LCPI32_3(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
	vroundps	$9, %ymm0, %ymm0
	vbroadcastss	.LCPI32_4(%rip), %ymm1
	vmulps	%ymm1, %ymm0, %ymm0
.Ltmp4505:
	.loc	50 48 14 is_stmt 1
	vaddps	%ymm0, %ymm9, %ymm3
.Ltmp4506:
	.loc	50 347 14
	vsubps	(%r9,%r10,4), %ymm3, %ymm9
.Ltmp4507:
	.loc	15 1549 5
	vmovaps	%ymm9, 3744(%rsp)
	cmpq	%rbx, %rsi
.Ltmp4508:
	.loc	14 1050 16
	ja	.LBB32_412
.Ltmp4509:
	.loc	50 360 14
	vdivps	2048(%rsp), %ymm9, %ymm3
	movq	480(%rsp), %rdi
.Ltmp4510:
	.loc	1 551 14
	vmovups	%ymm0, (%r9,%rdi,4)
.Ltmp4511:
	.loc	15 1554 43
	vmovaps	3712(%rsp), %ymm0
.Ltmp4512:
	.loc	50 347 14
	vsubps	%ymm3, %ymm5, %ymm3
	vmovaps	%ymm5, %ymm1
.Ltmp4513:
	.loc	50 347 14 is_stmt 0
	vsubps	%ymm0, %ymm3, %ymm5
.Ltmp4514:
	.loc	50 283 14 is_stmt 1
	vmulps	%ymm5, %ymm15, %ymm5
.Ltmp4515:
	.loc	50 48 14
	vaddps	%ymm5, %ymm0, %ymm0
.Ltmp4516:
	.loc	50 233 14
	vmaxps	%ymm0, %ymm3, %ymm0
.Ltmp4517:
	.loc	50 82 19
	vandps	%ymm0, %ymm11, %ymm3
.Ltmp4518:
	.loc	50 871 14
	vbroadcastss	.LCPI32_5(%rip), %ymm5
	vcmplt_oqps	%ymm5, %ymm3, %ymm3
.Ltmp4519:
	.loc	50 82 19
	vandnps	%ymm0, %ymm3, %ymm0
.Ltmp4520:
	.loc	15 1555 5
	vmovaps	%ymm0, 3712(%rsp)
	movq	448(%rsp), %rsi
	cmpq	1280(%rsp), %rsi
.Ltmp4521:
	.loc	14 1050 16
	ja	.LBB32_413
.Ltmp4522:
	.loc	15 0 0 is_stmt 0
	incq	%r13
	addq	2016(%rsp), %r15
.Ltmp4523:
	.loc	50 347 14 is_stmt 1
	vsubps	%ymm0, %ymm1, %ymm0
	movq	1728(%rsp), %rsi
.Ltmp4524:
	.loc	1 551 14
	vmovups	(%rsi,%r14,4), %ymm3
.Ltmp4525:
	.loc	1 551 14 is_stmt 0
	vmovups	(%r15), %ymm5
	vmovups	%ymm5, (%rsi,%r14,4)
.Ltmp4526:
	.loc	50 283 14 is_stmt 1
	vmulps	%ymm3, %ymm0, %ymm0
	vmovaps	768(%rsp), %ymm5
.Ltmp4527:
	.loc	50 585 19
	vblendvps	%ymm5, %ymm3, %ymm0, %ymm0
.Ltmp4528:
	.loc	1 551 14
	vmovups	%ymm0, (%r15)
	vmovaps	%ymm7, %ymm8
	vmovaps	%ymm14, %ymm11
.Ltmp4529:
	.loc	32 304 12
	cmpq	1984(%rsp), %r13
	jne	.LBB32_313
.Ltmp4530:
	.loc	32 0 12 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp4531:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 2816(%rsp)
.Ltmp4532:
	.loc	15 0 0 is_stmt 0
	vmovaps	%ymm13, 2720(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp4533:
	vmovaps	%ymm0, 2784(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp4534:
	.loc	15 746 0
	vmovaps	%ymm0, 3552(%rsp)
.Ltmp4535:
	.loc	15 0 0
	vmovaps	%ymm10, 3456(%rsp)
	vmovaps	224(%rsp), %ymm0
.Ltmp4536:
	vmovaps	%ymm0, 3520(%rsp)
	vmovaps	%ymm7, %ymm8
	vmovaps	%ymm14, %ymm11
	jmp	.LBB32_318
.Ltmp4537:
.LBB32_311:
	movq	24(%rsp), %r11
.LBB32_318:
	movq	176(%rsp), %r14
	movq	1120(%rsp), %r9
	xorl	%ebx, %ebx
	movq	216(%rsp), %r10
	movq	1216(%rsp), %r13
	movq	1152(%rsp), %r15
	.loc	15 1942 39 is_stmt 1
	addq	%r15, %r9
.Ltmp4538:
	.loc	15 1043 8
	cmpq	%r12, %r9
	movq	%r12, %rsi
	cmovbq	%rbx, %rsi
	subq	%rsi, %r9
	movq	1184(%rsp), %rdi
.Ltmp4539:
	.loc	15 1943 39
	addq	%r15, %rdi
	movq	16(%rsp), %rsi
.Ltmp4540:
	.loc	15 1043 8
	cmpq	%rsi, %rdi
	cmovbq	%rbx, %rsi
	subq	%rsi, %rdi
	movq	1248(%rsp), %rsi
	movq	%rsi, %r15
.Ltmp4541:
	.loc	15 1865 19
	cmpq	%r10, %rsi
	jb	.LBB32_306
.Ltmp4542:
	.loc	15 0 19 is_stmt 0
	movq	%r9, 1120(%rsp)
	movq	%rdi, 1184(%rsp)
	movl	64(%rsp), %eax
	movl	%eax, 1096(%rsp)
	vmovaps	%ymm11, 992(%rsp)
	movl	32(%rsp), %eax
	movl	%eax, 648(%rsp)
	vmovaps	%ymm8, 544(%rsp)
	movq	208(%rsp), %r13
	movq	200(%rsp), %r15
	movq	408(%rsp), %rdi
	movq	1472(%rsp), %r9
	movq	1440(%rsp), %rsi
	movq	1408(%rsp), %rbx
	movq	2080(%rsp), %r8
	jmp	.LBB32_290
.LBB32_252:
	movq	24(%rsp), %r11
	movq	176(%rsp), %r14
	movq	208(%rsp), %r13
	movq	200(%rsp), %r15
.LBB32_253:
	.loc	15 1948 13 is_stmt 1
	vmovaps	992(%rsp), %ymm0
	vmovaps	%ymm0, 5280(%rsp)
	.loc	15 1949 13
	movl	1096(%rsp), %ecx
	.loc	15 1950 13
	vmovaps	544(%rsp), %ymm0
	vmovaps	%ymm0, 5312(%rsp)
.Ltmp4543:
	.loc	15 1958 23
	movq	1736(%r11), %rdx
.Ltmp4544:
	.loc	37 451 16
	cmpq	$7, %rdx
	jbe	.LBB32_84
.Ltmp4545:
	.loc	15 0 0 is_stmt 0
	movl	648(%rsp), %eax
.Ltmp4546:
	.loc	15 1958 23 is_stmt 1
	movq	1728(%r11), %rdx
.Ltmp4547:
	.loc	1 551 14
	vmovaps	5280(%rsp), %ymm0
	vmovups	%ymm0, (%rdx)
.Ltmp4548:
	.loc	15 1959 5
	movq	1768(%r11), %rdx
.Ltmp4549:
	.loc	16 1714 9
	testq	%rdx, %rdx
.Ltmp4550:
	.loc	17 180 28
	je	.LBB32_257
.Ltmp4551:
	.loc	17 0 28 is_stmt 0
	movq	1760(%r11), %rsi
	shlq	$2, %rdx
	xorl	%edi, %edi
	.p2align	4
.LBB32_256:
.Ltmp4552:
	.loc	31 66 21 is_stmt 1
	movl	%ecx, (%rsi,%rdi)
.Ltmp4553:
	.loc	16 1714 9
	addq	$4, %rdi
	cmpq	%rdi, %rdx
.Ltmp4554:
	.loc	17 180 28
	jne	.LBB32_256
.Ltmp4555:
.LBB32_257:
	.loc	15 1960 24
	movq	1936(%r11), %rdx
.Ltmp4556:
	.loc	37 451 16
	cmpq	$7, %rdx
	jbe	.LBB32_84
.Ltmp4557:
	.loc	15 1960 24
	movq	1928(%r11), %rcx
.Ltmp4558:
	.loc	1 551 14
	vmovaps	5312(%rsp), %ymm0
	vmovups	%ymm0, (%rcx)
.Ltmp4559:
	.loc	15 1961 5
	movq	1968(%r11), %rcx
.Ltmp4560:
	.loc	16 1714 9
	testq	%rcx, %rcx
.Ltmp4561:
	.loc	17 180 28
	je	.LBB32_261
.Ltmp4562:
	.loc	17 0 28 is_stmt 0
	movq	1960(%r11), %rdx
	shlq	$2, %rcx
	xorl	%esi, %esi
	.p2align	4
.LBB32_260:
.Ltmp4563:
	.loc	31 66 21 is_stmt 1
	movl	%eax, (%rdx,%rsi)
.Ltmp4564:
	.loc	16 1714 9
	addq	$4, %rsi
	cmpq	%rsi, %rcx
.Ltmp4565:
	.loc	17 180 28
	jne	.LBB32_260
.Ltmp4566:
.LBB32_261:
	.loc	17 0 28 is_stmt 0
	leaq	2336(%rsp), %rdi
	movq	400(%rsp), %rsi
	.loc	15 1963 14 is_stmt 1
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	leaq	3072(%rsp), %rdi
	movq	392(%rsp), %rsi
	.loc	15 1964 15
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	movq	24(%rsp), %r9
	movq	1184(%rsp), %rax
	.loc	15 1965 5
	movl	%eax, 1640(%r9)
	movq	1120(%rsp), %rax
.Ltmp4567:
	.loc	15 0 0 is_stmt 0
	movl	%eax, 1644(%r9)
.Ltmp4568:
	.loc	15 2141 35 is_stmt 1
	cmpb	$0, 764(%rsp)
	jne	.LBB32_377
.LBB32_263:
	.loc	15 0 35 is_stmt 0
	xorl	%ecx, %ecx
.LBB32_264:
	movq	192(%rsp), %r10
.LBB32_265:
	leaq	1616(%r9), %rdx
	movabsq	$2305843009213693944, %rax
	.loc	15 2141 9 is_stmt 1
	movb	%cl, 2152(%r9)
	.loc	15 2146 30
	movzbl	2144(%r9), %ecx
	.loc	15 2146 9 is_stmt 0
	movb	%cl, 2153(%r9)
	.loc	15 2147 21 is_stmt 1
	movq	16(%rdx), %rcx
	movq	%rcx, 5360(%rsp)
	vmovups	(%rdx), %xmm0
	vmovaps	%xmm0, 5344(%rsp)
.Ltmp4569:
	.loc	50 871 14
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm0
.Ltmp4570:
	.loc	30 2155 12
	movq	%r13, %rcx
	vmovaps	%ymm0, %ymm1
	andq	%rax, %rcx
	je	.LBB32_268
.Ltmp4571:
	.loc	30 0 12 is_stmt 0
	vbroadcastss	.LCPI32_0(%rip), %ymm2
	xorl	%edx, %edx
	vbroadcastss	.LCPI32_6(%rip), %ymm3
	vmovaps	%ymm0, %ymm1
	.p2align	4
.LBB32_267:
.Ltmp4572:
	.loc	50 82 19 is_stmt 1
	vandps	(%r10,%rdx,4), %ymm2, %ymm4
.Ltmp4573:
	.loc	50 871 14
	vcmplt_oqps	%ymm3, %ymm4, %ymm4
.Ltmp4574:
	.loc	50 82 19
	vandps	%ymm4, %ymm1, %ymm1
.Ltmp4575:
	.loc	30 2155 12
	addq	$8, %rdx
	cmpq	%rdx, %rcx
	jne	.LBB32_267
.Ltmp4576:
.LBB32_268:
	.loc	15 0 0 is_stmt 0
	movl	2120(%r9), %edi
.Ltmp4577:
	.loc	15 2154 0 is_stmt 1
	movq	1584(%r9), %r12
	movq	1592(%r9), %r13
	movq	1600(%r9), %r8
	movq	1608(%r9), %rbx
.Ltmp4578:
	.file	51 "/home/bl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wide-1.6.1" "src/f32x8_.rs"
	.loc	51 176 9
	vpcmpeqd	%ymm2, %ymm2, %ymm2
	vtestps	%ymm2, %ymm1
.Ltmp4579:
	.loc	34 208 8
	jae	.LBB32_273
.Ltmp4580:
	.loc	30 2155 12
	movq	%r14, %rdx
	vmovaps	%ymm0, %ymm1
	andq	%rax, %rdx
	je	.LBB32_272
.Ltmp4581:
	.loc	30 0 12 is_stmt 0
	vbroadcastss	.LCPI32_0(%rip), %ymm2
	xorl	%esi, %esi
	vbroadcastss	.LCPI32_6(%rip), %ymm3
	vmovaps	%ymm0, %ymm1
	.p2align	4
.LBB32_271:
.Ltmp4582:
	.loc	50 82 19 is_stmt 1
	vandps	(%r15,%rsi,4), %ymm2, %ymm4
.Ltmp4583:
	.loc	50 871 14
	vcmplt_oqps	%ymm3, %ymm4, %ymm4
.Ltmp4584:
	.loc	50 82 19
	vandps	%ymm4, %ymm1, %ymm1
.Ltmp4585:
	.loc	30 2155 12
	addq	$8, %rsi
	cmpq	%rsi, %rdx
	jne	.LBB32_271
.Ltmp4586:
.LBB32_272:
	.loc	51 176 9
	vpcmpeqd	%ymm2, %ymm2, %ymm2
	vtestps	%ymm2, %ymm1
.Ltmp4587:
	.loc	34 208 34
	jb	.LBB32_376
.LBB32_273:
	.loc	34 0 34 is_stmt 0
	vmovaps	%ymm0, %ymm1
.Ltmp4588:
	.loc	30 2155 12 is_stmt 1
	testq	%rcx, %rcx
.Ltmp4589:
	.loc	30 2155 12 is_stmt 0
	je	.LBB32_276
.Ltmp4590:
	.loc	30 0 12
	vbroadcastss	.LCPI32_0(%rip), %ymm2
	xorl	%edx, %edx
	vbroadcastss	.LCPI32_6(%rip), %ymm3
	vmovaps	%ymm0, %ymm1
	.p2align	4
.LBB32_275:
.Ltmp4591:
	.loc	50 82 19 is_stmt 1
	vandps	(%r10,%rdx,4), %ymm2, %ymm4
.Ltmp4592:
	.loc	50 871 14
	vcmplt_oqps	%ymm3, %ymm4, %ymm4
.Ltmp4593:
	.loc	50 82 19
	vandps	%ymm4, %ymm1, %ymm1
.Ltmp4594:
	.loc	30 2155 12
	addq	$8, %rdx
	cmpq	%rdx, %rcx
	jne	.LBB32_275
.Ltmp4595:
.LBB32_276:
	.loc	50 585 19
	vpsrad	$31, %ymm1, %ymm2
	vpbroadcastd	.LCPI32_2(%rip), %ymm1
	vpandn	%ymm1, %ymm2, %ymm2
.Ltmp4596:
	.loc	34 185 12
	vmovd	%xmm2, %ecx
	xorl	%edx, %edx
	testl	%ecx, %ecx
	setne	%dl
	movl	%edx, 448(%rsp)
	vpextrd	$1, %xmm2, %edx
	xorl	%ecx, %ecx
	testl	%edx, %edx
	setne	%cl
	vpextrd	$2, %xmm2, %edx
	addl	%ecx, %ecx
	xorl	%esi, %esi
	testl	%edx, %edx
	setne	%sil
	shll	$2, %esi
	movl	%esi, 480(%rsp)
	vpextrd	$3, %xmm2, %edx
	xorl	%esi, %esi
	testl	%edx, %edx
	setne	%sil
	shll	$3, %esi
	vextracti128	$1, %ymm2, %xmm2
	vmovd	%xmm2, %edx
	xorl	%r9d, %r9d
	testl	%edx, %edx
	setne	%r9b
	shll	$4, %r9d
	movl	%r9d, 704(%rsp)
	vpextrd	$1, %xmm2, %edx
	xorl	%r10d, %r10d
	testl	%edx, %edx
	setne	%r10b
	vpextrd	$2, %xmm2, %edx
	shll	$5, %r10d
	xorl	%r9d, %r9d
	testl	%edx, %edx
	setne	%r9b
	shll	$6, %r9d
	vpextrd	$3, %xmm2, %edx
	xorl	%r11d, %r11d
	testl	%edx, %edx
	setne	%r11b
	shll	$7, %r11d
.Ltmp4597:
	.loc	30 2155 12
	andq	%r14, %rax
	movl	%edi, 32(%rsp)
	movq	%r8, 64(%rsp)
	movq	%rbx, 352(%rsp)
	movq	%r12, 672(%rsp)
	movq	%r13, 768(%rsp)
	je	.LBB32_279
.Ltmp4598:
	.loc	30 0 12 is_stmt 0
	xorl	%edx, %edx
	vbroadcastss	.LCPI32_0(%rip), %ymm2
	vbroadcastss	.LCPI32_6(%rip), %ymm3
	.p2align	4
.LBB32_278:
.Ltmp4599:
	.loc	50 82 19 is_stmt 1
	vandps	(%r15,%rdx,4), %ymm2, %ymm4
.Ltmp4600:
	.loc	50 871 14
	vcmplt_oqps	%ymm3, %ymm4, %ymm4
.Ltmp4601:
	.loc	50 82 19
	vandps	%ymm4, %ymm0, %ymm0
.Ltmp4602:
	.loc	30 2155 12
	addq	$8, %rdx
	cmpq	%rdx, %rax
	jne	.LBB32_278
.Ltmp4603:
.LBB32_279:
	.loc	50 585 19
	vpsrad	$31, %ymm0, %ymm0
	vpandn	%ymm1, %ymm0, %ymm0
.Ltmp4604:
	.loc	34 185 12
	vmovd	%xmm0, %edx
	xorl	%eax, %eax
	testl	%edx, %edx
	vpextrd	$1, %xmm0, %edx
	setne	%al
	xorl	%ebx, %ebx
	testl	%edx, %edx
	setne	%bl
	addl	%ebx, %ebx
	vpextrd	$2, %xmm0, %edx
	xorl	%r14d, %r14d
	testl	%edx, %edx
	setne	%r14b
	vpextrd	$3, %xmm0, %edx
	shll	$2, %r14d
	xorl	%r15d, %r15d
	testl	%edx, %edx
	setne	%r15b
	shll	$3, %r15d
	vextracti128	$1, %ymm0, %xmm0
	vmovd	%xmm0, %edx
	xorl	%r13d, %r13d
	testl	%edx, %edx
	setne	%r13b
	vpextrd	$1, %xmm0, %edx
	shll	$4, %r13d
	xorl	%r12d, %r12d
	testl	%edx, %edx
	setne	%r12b
	shll	$5, %r12d
	vpextrd	$2, %xmm0, %edx
	xorl	%edi, %edi
	testl	%edx, %edx
	setne	%dil
	vpextrd	$3, %xmm0, %r8d
	shll	$6, %edi
	xorl	%edx, %edx
	testl	%r8d, %r8d
	setne	%dl
	shll	$7, %edx
.Ltmp4605:
	.loc	34 185 12 is_stmt 0
	orl	%edi, %edx
.Ltmp4606:
	.loc	34 185 12
	orl	448(%rsp), %ecx
	orl	480(%rsp), %ecx
	orl	704(%rsp), %esi
	orl	%r10d, %esi
	orl	%ecx, %esi
	orl	%r11d, %r9d
	orl	%esi, %r9d
.Ltmp4607:
	.loc	34 185 12
	orl	%eax, %r9d
	orl	%ebx, %r9d
	orl	%r14d, %r15d
	orl	%r9d, %r15d
	orl	%r13d, %r12d
	orl	%r15d, %r12d
.Ltmp4608:
	.loc	34 211 5 is_stmt 1
	orl	%edx, %r12d
	movq	24(%rsp), %rdx
	movl	%r12d, 1576(%rdx)
	.loc	34 212 31
	movq	1568(%rdx), %rax
.Ltmp4609:
	.loc	14 2428 13
	incq	%rax
	movq	$-1, %rcx
	cmovneq	%rax, %rcx
.Ltmp4610:
	.loc	34 212 5
	movq	%rcx, 1568(%rdx)
	movq	208(%rsp), %rdx
.Ltmp4611:
	.loc	16 1714 9
	testq	%rdx, %rdx
.Ltmp4612:
	.loc	17 180 28
	je	.LBB32_281
.Ltmp4613:
	.loc	12 961 18
	shlq	$2, %rdx
	movq	192(%rsp), %rdi
.Ltmp4614:
	.loc	31 25 13
	xorl	%esi, %esi
	vzeroupper
	callq	*memset@GOTPCREL(%rip)
.Ltmp4615:
.LBB32_281:
	.loc	31 0 13 is_stmt 0
	movq	176(%rsp), %rdx
.Ltmp4616:
	.loc	16 1714 9 is_stmt 1
	testq	%rdx, %rdx
	movq	200(%rsp), %rdi
	movq	672(%rsp), %r15
	movq	768(%rsp), %r12
.Ltmp4617:
	.loc	17 180 28
	je	.LBB32_283
.Ltmp4618:
	.loc	12 961 18
	shlq	$2, %rdx
.Ltmp4619:
	.loc	31 25 13
	xorl	%esi, %esi
	vzeroupper
	callq	*memset@GOTPCREL(%rip)
.Ltmp4620:
.LBB32_283:
	.loc	15 2155 18
	movq	_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults@GOTPCREL(%rip), %rbx
	leaq	5344(%rsp), %r14
	movq	400(%rsp), %rdi
	movq	%r14, %rsi
	movq	%r15, %rdx
	movq	%r12, %rcx
	movl	32(%rsp), %r15d
	movl	%r15d, %r8d
	vzeroupper
	callq	*%rbx
	movq	392(%rsp), %rdi
	.loc	15 2156 19
	movq	%r14, %rsi
	movq	64(%rsp), %rdx
	movq	352(%rsp), %rcx
	movl	%r15d, %r8d
	callq	*%rbx
	movq	24(%rsp), %rax
	.loc	15 2157 13
	movq	$0, 1640(%rax)
.Ltmp4621:
.LBB32_376:
	.loc	15 2159 6
	leaq	-40(%rbp), %rsp
	.loc	15 2159 6 epilogue_begin is_stmt 0
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	vzeroupper
	retq
.LBB32_326:
	.cfi_def_cfa %rbp, 16
	.loc	15 0 6
	vmovaps	320(%rsp), %ymm0
.Ltmp4622:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 2816(%rsp)
.Ltmp4623:
	.loc	15 0 0 is_stmt 0
	vmovaps	%ymm13, 2720(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp4624:
	vmovaps	%ymm0, 2784(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp4625:
	.loc	15 746 0
	vmovaps	%ymm0, 3552(%rsp)
.Ltmp4626:
	.loc	15 0 0
	vmovaps	%ymm10, 3456(%rsp)
	vmovaps	224(%rsp), %ymm0
.Ltmp4627:
	vmovaps	%ymm0, 3520(%rsp)
.Ltmp4628:
	movl	%r14d, 1096(%rsp)
.Ltmp4629:
	vmovaps	%ymm14, 992(%rsp)
.Ltmp4630:
.LBB32_321:
	movl	32(%rsp), %eax
.Ltmp4631:
	movl	%eax, 648(%rsp)
.Ltmp4632:
	vmovaps	%ymm8, 544(%rsp)
.Ltmp4633:
	.loc	15 1026 25 is_stmt 1
	leaq	8(%rdi), %rsi
.Ltmp4634:
	.loc	37 443 13
	leaq	.Lalloc_296a0a227063f9d4f193ea1ae436593b(%rip), %rcx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4635:
.LBB32_410:
	.loc	37 0 13 is_stmt 0
	movq	%rax, %rdx
	vmovaps	320(%rsp), %ymm0
.Ltmp4636:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 2816(%rsp)
.Ltmp4637:
	.loc	15 0 0 is_stmt 0
	vmovaps	%ymm13, 2720(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp4638:
	vmovaps	%ymm0, 2784(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp4639:
	.loc	15 746 0
	vmovaps	%ymm0, 3552(%rsp)
.Ltmp4640:
	.loc	15 0 0
	vmovaps	%ymm10, 3456(%rsp)
	vmovaps	224(%rsp), %ymm0
.Ltmp4641:
	vmovaps	%ymm0, 3520(%rsp)
	movl	64(%rsp), %ecx
.Ltmp4642:
	movl	%ecx, 1096(%rsp)
.Ltmp4643:
	vmovaps	%ymm14, 992(%rsp)
	movl	32(%rsp), %ecx
.Ltmp4644:
	movl	%ecx, 648(%rsp)
.Ltmp4645:
	vmovaps	%ymm7, 544(%rsp)
.Ltmp4646:
	.loc	15 1026 25 is_stmt 1
	leaq	8(%r10), %rsi
.Ltmp4647:
	.loc	37 443 13
	leaq	.Lalloc_296a0a227063f9d4f193ea1ae436593b(%rip), %rcx
	movq	%r10, %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4648:
.LBB32_85:
	.loc	15 0 0 is_stmt 0
	leaq	.Lalloc_e0592aef22128a0ac53753b9632a8183(%rip), %rcx
	movq	%r13, %rsi
	movq	%r13, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB32_101:
	leaq	.Lalloc_e0592aef22128a0ac53753b9632a8183(%rip), %rcx
	movq	%r14, %rsi
	movq	%r14, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4649:
.LBB32_348:
	movb	$1, %r10b
	.loc	15 2096 12 is_stmt 1
	cmpb	$0, 2152(%rdi)
	je	.LBB32_32
.Ltmp4650:
	.loc	15 660 31
	movq	1768(%rdi), %rcx
	.loc	15 660 57 is_stmt 0
	movq	1832(%rdi), %rax
.Ltmp4651:
	.loc	10 1078 5 is_stmt 1
	cmpq	%rcx, %rax
	cmovbq	%rax, %rcx
.Ltmp4652:
	.loc	32 304 12
	testq	%rcx, %rcx
	je	.LBB32_356
.Ltmp4653:
	.loc	32 0 12 is_stmt 0
	movq	1760(%rdi), %rsi
	movq	1824(%rdi), %r11
	xorl	%r8d, %r8d
	jmp	.LBB32_351
.LBB32_354:
.Ltmp4654:
	.loc	15 662 22 is_stmt 1
	xorl	%edx, %edx
	divq	%rbx
.LBB32_355:
	.loc	15 662 13 is_stmt 0
	movl	%edx, (%rsi,%r8,4)
.Ltmp4655:
	.loc	15 0 0
	incq	%r8
.Ltmp4656:
	.loc	32 304 12 is_stmt 1
	addq	$12, %r11
	cmpq	%r8, %rcx
	je	.LBB32_356
.Ltmp4657:
.LBB32_351:
	.loc	15 661 26
	movl	(%r11), %ebx
	testq	%rbx, %rbx
.Ltmp4658:
	.loc	15 662 42
	je	.LBB32_414
	.loc	15 662 24 is_stmt 0
	movl	(%rsi,%r8,4), %r10d
	.loc	15 662 42
	movl	%r9d, %eax
	xorl	%edx, %edx
	divl	%ebx
	movl	%edx, %eax
	.loc	15 662 23
	addq	%r10, %rax
	.loc	15 662 22
	btq	$32, %rax
	jb	.LBB32_354
	xorl	%edx, %edx
	divl	%ebx
	jmp	.LBB32_355
.Ltmp4659:
.LBB32_397:
	.loc	15 0 22
	vmovaps	96(%rsp), %ymm0
.Ltmp4660:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 4288(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp4661:
	.loc	15 746 0 is_stmt 0
	vmovaps	%ymm0, 5024(%rsp)
.Ltmp4662:
	.loc	15 1186 45 is_stmt 1
	leaq	.Lalloc_a19fa7d9077791c9eadbe74fafea8de7(%rip), %rdx
	movq	%r12, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4663:
.LBB32_399:
	.loc	15 0 45 is_stmt 0
	vmovaps	96(%rsp), %ymm0
.Ltmp4664:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 4288(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp4665:
	.loc	15 746 0 is_stmt 0
	vmovaps	%ymm0, 5024(%rsp)
.Ltmp4666:
	.loc	15 1186 45 is_stmt 1
	leaq	.Lalloc_a19fa7d9077791c9eadbe74fafea8de7(%rip), %rdx
	movq	%r14, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4667:
.LBB32_356:
	.loc	15 660 31
	movq	1968(%rdi), %rcx
	.loc	15 660 57 is_stmt 0
	movq	2032(%rdi), %rax
.Ltmp4668:
	.loc	10 1078 5 is_stmt 1
	cmpq	%rcx, %rax
	cmovbq	%rax, %rcx
.Ltmp4669:
	.loc	32 304 12
	testq	%rcx, %rcx
	je	.LBB32_363
.Ltmp4670:
	.loc	32 0 12 is_stmt 0
	movq	1960(%rdi), %rsi
	movq	2024(%rdi), %r11
	xorl	%r8d, %r8d
	jmp	.LBB32_358
.LBB32_361:
.Ltmp4671:
	.loc	15 662 22 is_stmt 1
	movq	%rdx, %rax
	xorl	%edx, %edx
	divq	%rbx
.LBB32_362:
	.loc	15 662 13 is_stmt 0
	movl	%edx, (%rsi,%r8,4)
.Ltmp4672:
	.loc	15 0 0
	incq	%r8
.Ltmp4673:
	.loc	32 304 12 is_stmt 1
	addq	$12, %r11
	cmpq	%r8, %rcx
	je	.LBB32_363
.Ltmp4674:
.LBB32_358:
	.loc	15 661 26
	movl	(%r11), %ebx
	testq	%rbx, %rbx
.Ltmp4675:
	.loc	15 662 42
	je	.LBB32_414
	.loc	15 662 24 is_stmt 0
	movl	(%rsi,%r8,4), %r10d
	.loc	15 662 42
	movl	%r9d, %eax
	xorl	%edx, %edx
	divl	%ebx
	.loc	15 662 23
	addq	%r10, %rdx
	.loc	15 662 22
	btq	$32, %rdx
	jb	.LBB32_361
	movl	%edx, %eax
	xorl	%edx, %edx
	divl	%ebx
	jmp	.LBB32_362
.Ltmp4676:
.LBB32_363:
	.loc	15 2113 26 is_stmt 1
	movq	1632(%rdi), %rsi
.Ltmp4677:
	.loc	15 452 44
	testq	%rsi, %rsi
	je	.LBB32_415
.Ltmp4678:
	.loc	15 2113 26
	movq	1624(%rdi), %rcx
.Ltmp4679:
	.loc	15 452 23
	movl	1640(%rdi), %r8d
	movq	%r9, %rax
	.loc	15 452 44 is_stmt 0
	cmpq	%rsi, %r9
	jb	.LBB32_366
	movl	%r9d, %eax
	xorl	%edx, %edx
	divl	%esi
	movl	%edx, %eax
.LBB32_366:
	.loc	15 452 22
	addq	%r8, %rax
	.loc	15 452 21
	movq	%rax, %rdx
	orq	%rsi, %rdx
	shrq	$32, %rdx
	je	.LBB32_367
	xorl	%edx, %edx
	divq	%rsi
	jmp	.LBB32_369
.Ltmp4680:
.LBB32_316:
	.loc	15 0 21
	vmovaps	320(%rsp), %ymm0
.Ltmp4681:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 2816(%rsp)
.Ltmp4682:
	.loc	15 0 0 is_stmt 0
	vmovaps	%ymm13, 2720(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp4683:
	vmovaps	%ymm0, 2784(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp4684:
	.loc	15 746 0
	vmovaps	%ymm0, 3552(%rsp)
.Ltmp4685:
	.loc	15 0 0
	vmovaps	%ymm10, 3456(%rsp)
	vmovaps	224(%rsp), %ymm0
.Ltmp4686:
	vmovaps	%ymm0, 3520(%rsp)
.Ltmp4687:
	movl	%r14d, 1096(%rsp)
.Ltmp4688:
	vmovaps	%ymm11, 992(%rsp)
	movl	32(%rsp), %eax
.Ltmp4689:
	movl	%eax, 648(%rsp)
.Ltmp4690:
	vmovaps	%ymm8, 544(%rsp)
.Ltmp4691:
	.loc	37 456 13 is_stmt 1
	leaq	.Lalloc_cd0e502fea74c9eb9984d521d7f3533e(%rip), %rcx
	movq	%r9, %rdi
	movq	%r15, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4692:
.LBB32_320:
	.loc	37 0 13 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp4693:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 2816(%rsp)
.Ltmp4694:
	.loc	15 0 0 is_stmt 0
	vmovaps	%ymm13, 2720(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp4695:
	vmovaps	%ymm0, 2784(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp4696:
	.loc	15 746 0
	vmovaps	%ymm0, 3552(%rsp)
.Ltmp4697:
	.loc	15 0 0
	vmovaps	%ymm10, 3456(%rsp)
	vmovaps	224(%rsp), %ymm0
.Ltmp4698:
	vmovaps	%ymm0, 3520(%rsp)
.Ltmp4699:
	movl	%r14d, 1096(%rsp)
.Ltmp4700:
	vmovaps	%ymm11, 992(%rsp)
	jmp	.LBB32_321
.Ltmp4701:
.LBB32_404:
	vmovaps	320(%rsp), %ymm0
.Ltmp4702:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 2816(%rsp)
.Ltmp4703:
	.loc	15 0 0 is_stmt 0
	vmovaps	%ymm13, 2720(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp4704:
	vmovaps	%ymm0, 2784(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp4705:
	.loc	15 746 0
	vmovaps	%ymm0, 3552(%rsp)
.Ltmp4706:
	.loc	15 0 0
	vmovaps	%ymm10, 3456(%rsp)
	vmovaps	224(%rsp), %ymm0
.Ltmp4707:
	vmovaps	%ymm0, 3520(%rsp)
.Ltmp4708:
	movl	%r11d, 1096(%rsp)
.Ltmp4709:
	vmovaps	%ymm14, 992(%rsp)
	movl	32(%rsp), %eax
.Ltmp4710:
	movl	%eax, 648(%rsp)
.Ltmp4711:
	vmovaps	%ymm8, 544(%rsp)
.Ltmp4712:
	.loc	15 1026 25 is_stmt 1
	leaq	8(%rdi), %rsi
.Ltmp4713:
	.loc	37 443 13
	leaq	.Lalloc_296a0a227063f9d4f193ea1ae436593b(%rip), %rcx
	movq	%rbx, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4714:
.LBB32_405:
	.loc	37 0 13 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp4715:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 2816(%rsp)
.Ltmp4716:
	.loc	15 0 0 is_stmt 0
	vmovaps	%ymm13, 2720(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp4717:
	vmovaps	%ymm0, 2784(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp4718:
	.loc	15 746 0
	vmovaps	%ymm0, 3552(%rsp)
.Ltmp4719:
	.loc	15 0 0
	vmovaps	%ymm10, 3456(%rsp)
	vmovaps	224(%rsp), %ymm0
.Ltmp4720:
	vmovaps	%ymm0, 3520(%rsp)
.Ltmp4721:
	movl	%r11d, 1096(%rsp)
.Ltmp4722:
	vmovaps	%ymm14, 992(%rsp)
	movl	32(%rsp), %eax
.Ltmp4723:
	movl	%eax, 648(%rsp)
.Ltmp4724:
	vmovaps	%ymm8, 544(%rsp)
.Ltmp4725:
	.loc	37 456 13 is_stmt 1
	leaq	.Lalloc_cd0e502fea74c9eb9984d521d7f3533e(%rip), %rcx
	movq	%r9, %rdi
	movq	%r15, %rsi
	movq	%rbx, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4726:
.LBB32_406:
	.loc	37 0 13 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp4727:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 2816(%rsp)
.Ltmp4728:
	.loc	15 0 0 is_stmt 0
	vmovaps	%ymm13, 2720(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp4729:
	vmovaps	%ymm0, 2784(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp4730:
	.loc	15 746 0
	vmovaps	%ymm0, 3552(%rsp)
.Ltmp4731:
	.loc	15 0 0
	vmovaps	%ymm10, 3456(%rsp)
	vmovaps	224(%rsp), %ymm0
.Ltmp4732:
	vmovaps	%ymm0, 3520(%rsp)
.Ltmp4733:
	movl	%r11d, 1096(%rsp)
.Ltmp4734:
	vmovaps	%ymm14, 992(%rsp)
	movl	32(%rsp), %eax
.Ltmp4735:
	movl	%eax, 648(%rsp)
.Ltmp4736:
	vmovaps	%ymm8, 544(%rsp)
.Ltmp4737:
	.loc	37 443 13 is_stmt 1
	leaq	.Lalloc_296a0a227063f9d4f193ea1ae436593b(%rip), %rcx
	movq	%r14, %rdi
	movq	1312(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4738:
.LBB32_407:
	.loc	37 0 13 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp4739:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 2816(%rsp)
.Ltmp4740:
	.loc	15 0 0 is_stmt 0
	vmovaps	%ymm13, 2720(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp4741:
	vmovaps	%ymm0, 2784(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp4742:
	.loc	15 746 0
	vmovaps	%ymm0, 3552(%rsp)
.Ltmp4743:
	.loc	15 0 0
	vmovaps	%ymm10, 3456(%rsp)
	vmovaps	224(%rsp), %ymm0
.Ltmp4744:
	vmovaps	%ymm0, 3520(%rsp)
	movl	64(%rsp), %ecx
.Ltmp4745:
	movl	%ecx, 1096(%rsp)
.Ltmp4746:
	vmovaps	%ymm14, 992(%rsp)
	movl	32(%rsp), %ecx
.Ltmp4747:
	movl	%ecx, 648(%rsp)
.Ltmp4748:
	vmovaps	%ymm8, 544(%rsp)
.Ltmp4749:
	.loc	37 456 13 is_stmt 1
	leaq	.Lalloc_cd0e502fea74c9eb9984d521d7f3533e(%rip), %rcx
	movq	%r9, %rdi
	movq	%rax, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4750:
.LBB32_408:
	.loc	37 0 13 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp4751:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 2816(%rsp)
.Ltmp4752:
	.loc	15 0 0 is_stmt 0
	vmovaps	%ymm13, 2720(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp4753:
	vmovaps	%ymm0, 2784(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp4754:
	.loc	15 746 0
	vmovaps	%ymm0, 3552(%rsp)
.Ltmp4755:
	.loc	15 0 0
	vmovaps	%ymm10, 3456(%rsp)
	vmovaps	224(%rsp), %ymm0
.Ltmp4756:
	vmovaps	%ymm0, 3520(%rsp)
	movl	64(%rsp), %ecx
.Ltmp4757:
	movl	%ecx, 1096(%rsp)
.Ltmp4758:
	vmovaps	%ymm14, 992(%rsp)
	movl	32(%rsp), %ecx
.Ltmp4759:
	movl	%ecx, 648(%rsp)
.Ltmp4760:
	vmovaps	%ymm8, 544(%rsp)
.Ltmp4761:
	.loc	15 1026 25 is_stmt 1
	leaq	8(%rdi), %rsi
.Ltmp4762:
	.loc	37 443 13
	leaq	.Lalloc_296a0a227063f9d4f193ea1ae436593b(%rip), %rcx
	movq	%rax, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4763:
.LBB32_411:
	.loc	37 0 13 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp4764:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 2816(%rsp)
.Ltmp4765:
	.loc	15 0 0 is_stmt 0
	vmovaps	%ymm13, 2720(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp4766:
	vmovaps	%ymm0, 2784(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp4767:
	.loc	15 746 0
	vmovaps	%ymm0, 3552(%rsp)
.Ltmp4768:
	.loc	15 0 0
	vmovaps	%ymm10, 3456(%rsp)
	vmovaps	224(%rsp), %ymm0
.Ltmp4769:
	vmovaps	%ymm0, 3520(%rsp)
	movl	64(%rsp), %eax
.Ltmp4770:
	movl	%eax, 1096(%rsp)
.Ltmp4771:
	vmovaps	%ymm14, 992(%rsp)
	movl	32(%rsp), %eax
.Ltmp4772:
	movl	%eax, 648(%rsp)
.Ltmp4773:
	vmovaps	%ymm7, 544(%rsp)
.Ltmp4774:
	.loc	15 1026 25 is_stmt 1
	leaq	8(%r10), %rsi
.Ltmp4775:
	.loc	37 443 13
	leaq	.Lalloc_296a0a227063f9d4f193ea1ae436593b(%rip), %rcx
	movq	%r10, %rdi
	movq	%rbx, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4776:
.LBB32_412:
	.loc	37 0 13 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp4777:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 2816(%rsp)
.Ltmp4778:
	.loc	15 0 0 is_stmt 0
	vmovaps	%ymm13, 2720(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp4779:
	vmovaps	%ymm0, 2784(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp4780:
	.loc	15 746 0
	vmovaps	%ymm0, 3552(%rsp)
.Ltmp4781:
	.loc	15 0 0
	vmovaps	%ymm10, 3456(%rsp)
	vmovaps	224(%rsp), %ymm0
.Ltmp4782:
	vmovaps	%ymm0, 3520(%rsp)
	movl	64(%rsp), %eax
.Ltmp4783:
	movl	%eax, 1096(%rsp)
.Ltmp4784:
	vmovaps	%ymm14, 992(%rsp)
	movl	32(%rsp), %eax
.Ltmp4785:
	movl	%eax, 648(%rsp)
.Ltmp4786:
	vmovaps	%ymm7, 544(%rsp)
.Ltmp4787:
	.loc	37 456 13 is_stmt 1
	leaq	.Lalloc_cd0e502fea74c9eb9984d521d7f3533e(%rip), %rcx
	movq	480(%rsp), %rdi
	movq	%rbx, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4788:
.LBB32_413:
	.loc	37 0 13 is_stmt 0
	vmovaps	320(%rsp), %ymm0
.Ltmp4789:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 2816(%rsp)
.Ltmp4790:
	.loc	15 0 0 is_stmt 0
	vmovaps	%ymm13, 2720(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp4791:
	vmovaps	%ymm0, 2784(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp4792:
	.loc	15 746 0
	vmovaps	%ymm0, 3552(%rsp)
.Ltmp4793:
	.loc	15 0 0
	vmovaps	%ymm10, 3456(%rsp)
	vmovaps	224(%rsp), %ymm0
.Ltmp4794:
	vmovaps	%ymm0, 3520(%rsp)
	movl	64(%rsp), %eax
.Ltmp4795:
	movl	%eax, 1096(%rsp)
.Ltmp4796:
	vmovaps	%ymm14, 992(%rsp)
	movl	32(%rsp), %eax
.Ltmp4797:
	movl	%eax, 648(%rsp)
.Ltmp4798:
	vmovaps	%ymm7, 544(%rsp)
.Ltmp4799:
	.loc	37 443 13 is_stmt 1
	leaq	.Lalloc_296a0a227063f9d4f193ea1ae436593b(%rip), %rcx
	movq	%r14, %rdi
	movq	1280(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4800:
.LBB32_367:
	.loc	15 452 21
	xorl	%edx, %edx
	divl	%esi
.LBB32_369:
	.loc	15 452 9 is_stmt 0
	movl	%edx, 1640(%rdi)
	.loc	15 453 44 is_stmt 1
	testq	%rcx, %rcx
	je	.LBB32_416
	.loc	15 453 23 is_stmt 0
	movl	1644(%rdi), %esi
	.loc	15 453 44
	cmpq	%rcx, %r9
	jb	.LBB32_372
	movl	%r9d, %eax
	xorl	%edx, %edx
	divl	%ecx
	movl	%edx, %r9d
.LBB32_372:
	.loc	15 453 22
	addq	%rsi, %r9
	.loc	15 453 21
	movq	%r9, %rax
	orq	%rcx, %rax
	shrq	$32, %rax
	je	.LBB32_373
	movq	%r9, %rax
	xorl	%edx, %edx
	divq	%rcx
	jmp	.LBB32_375
.LBB32_373:
	movl	%r9d, %eax
	xorl	%edx, %edx
	divl	%ecx
.LBB32_375:
	.loc	15 453 9
	movl	%edx, 1644(%rdi)
	jmp	.LBB32_376
.Ltmp4801:
.LBB32_409:
	.loc	15 0 9
	vmovaps	320(%rsp), %ymm0
.Ltmp4802:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 2816(%rsp)
.Ltmp4803:
	.loc	15 0 0 is_stmt 0
	vmovaps	%ymm13, 2720(%rsp)
	vmovaps	256(%rsp), %ymm0
.Ltmp4804:
	vmovaps	%ymm0, 2784(%rsp)
	vmovaps	288(%rsp), %ymm0
.Ltmp4805:
	.loc	15 746 0
	vmovaps	%ymm0, 3552(%rsp)
.Ltmp4806:
	.loc	15 0 0
	vmovaps	%ymm10, 3456(%rsp)
	vmovaps	224(%rsp), %ymm0
.Ltmp4807:
	vmovaps	%ymm0, 3520(%rsp)
	movl	64(%rsp), %ecx
.Ltmp4808:
	movl	%ecx, 1096(%rsp)
.Ltmp4809:
	vmovaps	%ymm14, 992(%rsp)
	movl	32(%rsp), %ecx
.Ltmp4810:
	movl	%ecx, 648(%rsp)
.Ltmp4811:
	vmovaps	%ymm7, 544(%rsp)
.Ltmp4812:
	.loc	15 1026 25 is_stmt 1
	leaq	8(%r10), %rsi
.Ltmp4813:
	.loc	37 443 13
	leaq	.Lalloc_296a0a227063f9d4f193ea1ae436593b(%rip), %rcx
	movq	%r10, %rdi
	movq	%rax, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4814:
.LBB32_191:
	.loc	37 0 13 is_stmt 0
	vmovaps	96(%rsp), %ymm0
.Ltmp4815:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 4288(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp4816:
	.loc	15 746 0 is_stmt 0
	vmovaps	%ymm0, 5024(%rsp)
.Ltmp4817:
	.loc	15 1175 9 is_stmt 1
	leaq	.Lalloc_067dce244605df4a33956fbd4d726027(%rip), %rdx
	movq	64(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4818:
.LBB32_189:
	.loc	15 0 9 is_stmt 0
	vmovaps	96(%rsp), %ymm0
.Ltmp4819:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 4288(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp4820:
	.loc	15 746 0 is_stmt 0
	vmovaps	%ymm0, 5024(%rsp)
.Ltmp4821:
	.loc	15 1173 24 is_stmt 1
	leaq	.Lalloc_697dc8945f5e5b040d7e9a0b92cb249f(%rip), %rdx
	movq	%r11, %rdi
	movq	64(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4822:
.LBB32_395:
	.loc	15 0 24 is_stmt 0
	vmovaps	96(%rsp), %ymm0
.Ltmp4823:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 4288(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp4824:
	.loc	15 746 0 is_stmt 0
	vmovaps	%ymm0, 5024(%rsp)
.Ltmp4825:
	.loc	15 1158 21 is_stmt 1
	leaq	.Lalloc_077370d5cece7380867993336836eb69(%rip), %rdx
	movq	352(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4826:
.LBB32_396:
	.loc	15 0 21 is_stmt 0
	vmovaps	96(%rsp), %ymm0
.Ltmp4827:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 4288(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp4828:
	.loc	15 746 0 is_stmt 0
	vmovaps	%ymm0, 5024(%rsp)
.Ltmp4829:
	.loc	15 1168 22 is_stmt 1
	leaq	.Lalloc_5cae9ea88ed6362f62dbad10291edbd4(%rip), %rdx
	movq	%r12, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4830:
.LBB32_118:
	.loc	15 0 22 is_stmt 0
	vmovaps	96(%rsp), %ymm0
.Ltmp4831:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 4288(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp4832:
	.loc	15 746 0 is_stmt 0
	vmovaps	%ymm0, 5024(%rsp)
.Ltmp4833:
	.loc	15 1169 24 is_stmt 1
	leaq	.Lalloc_9fdd00ed7abe4ccfd73f0d145a7b741b(%rip), %rdx
	movq	672(%rsp), %rdi
.Ltmp4834:
	.loc	15 0 0 is_stmt 0
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4835:
.LBB32_186:
	vmovaps	96(%rsp), %ymm0
.Ltmp4836:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 4288(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp4837:
	.loc	15 746 0 is_stmt 0
	vmovaps	%ymm0, 5024(%rsp)
.Ltmp4838:
	.loc	15 1158 21 is_stmt 1
	leaq	.Lalloc_077370d5cece7380867993336836eb69(%rip), %rdx
	movq	672(%rsp), %rdi
.Ltmp4839:
	.loc	15 0 0 is_stmt 0
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4840:
.LBB32_187:
	vmovaps	96(%rsp), %ymm0
.Ltmp4841:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 4288(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp4842:
	.loc	15 746 0 is_stmt 0
	vmovaps	%ymm0, 5024(%rsp)
.Ltmp4843:
	.loc	15 1168 22 is_stmt 1
	leaq	.Lalloc_5cae9ea88ed6362f62dbad10291edbd4(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4844:
.LBB32_188:
	.loc	15 0 22 is_stmt 0
	vmovaps	96(%rsp), %ymm0
.Ltmp4845:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 4288(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp4846:
	.loc	15 746 0 is_stmt 0
	vmovaps	%ymm0, 5024(%rsp)
.Ltmp4847:
	.loc	15 1169 24 is_stmt 1
	leaq	.Lalloc_9fdd00ed7abe4ccfd73f0d145a7b741b(%rip), %rdx
	movq	768(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4848:
.LBB32_401:
	.loc	15 0 24 is_stmt 0
	movl	64(%rsp), %eax
	movl	%eax, 1096(%rsp)
	vmovaps	%ymm11, 992(%rsp)
	movl	32(%rsp), %eax
	movl	%eax, 648(%rsp)
	vmovaps	%ymm8, 544(%rsp)
.Ltmp4849:
	.loc	37 456 13 is_stmt 1
	leaq	.Lalloc_14e3d3ba493695bccf4dcde9be821bfc(%rip), %rcx
	movq	208(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4850:
.LBB32_403:
	.loc	37 0 13 is_stmt 0
	movl	64(%rsp), %eax
	movl	%eax, 1096(%rsp)
	vmovaps	%ymm11, 992(%rsp)
	movl	32(%rsp), %eax
	movl	%eax, 648(%rsp)
	vmovaps	%ymm8, 544(%rsp)
	movq	1248(%rsp), %rsi
.Ltmp4851:
	.loc	15 1880 61 is_stmt 1
	shlq	$3, %rsi
.Ltmp4852:
	.loc	37 443 13
	leaq	.Lalloc_f81d29d0a485451b3eceb8bcdce402aa(%rip), %rcx
	movl	$256, %edx
	movq	224(%rsp), %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4853:
.LBB32_117:
	.loc	37 0 13 is_stmt 0
	vmovaps	96(%rsp), %ymm0
.Ltmp4854:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 4288(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp4855:
	.loc	15 746 0 is_stmt 0
	vmovaps	%ymm0, 5024(%rsp)
.Ltmp4856:
	.loc	15 1180 24 is_stmt 1
	leaq	.Lalloc_d5ea38731a1cb6311efef2f919234d06(%rip), %rdx
	movq	%r12, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4857:
.LBB32_158:
	.loc	15 0 24 is_stmt 0
	vmovaps	96(%rsp), %ymm0
.Ltmp4858:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 4288(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp4859:
	.loc	15 746 0 is_stmt 0
	vmovaps	%ymm0, 5024(%rsp)
.Ltmp4860:
	.loc	15 1180 24 is_stmt 1
	leaq	.Lalloc_d5ea38731a1cb6311efef2f919234d06(%rip), %rdx
	movq	%r14, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4861:
.LBB32_402:
	.loc	15 0 24 is_stmt 0
	movl	64(%rsp), %eax
	movl	%eax, 1096(%rsp)
	vmovaps	%ymm11, 992(%rsp)
	movl	32(%rsp), %eax
	movl	%eax, 648(%rsp)
	vmovaps	%ymm8, 544(%rsp)
.Ltmp4862:
	.loc	37 456 13 is_stmt 1
	leaq	.Lalloc_8706d23ef0096dbeb31c5991bb003668(%rip), %rcx
	movq	176(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4863:
.LBB32_232:
	.loc	37 0 13 is_stmt 0
	vmovaps	96(%rsp), %ymm0
.Ltmp4864:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 4288(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp4865:
	.loc	15 746 0 is_stmt 0
	vmovaps	%ymm0, 5024(%rsp)
.Ltmp4866:
	.loc	15 1302 25 is_stmt 1
	leaq	.Lalloc_aca25255cb99dc5ba482e692667f5878(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4867:
.LBB32_417:
	.loc	37 443 13
	leaq	.Lalloc_df34220a19b474a957e74caf6cc3e38d(%rip), %rcx
.Ltmp4868:
	.loc	37 443 13 is_stmt 0
	xorl	%edi, %edi
	movq	%r13, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4869:
.LBB32_23:
	.loc	37 443 13 is_stmt 1
	leaq	.Lalloc_449c9992b9e28dda1c528a8de6026d01(%rip), %rcx
.Ltmp4870:
	.loc	37 443 13 is_stmt 0
	xorl	%edi, %edi
	movq	%r13, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4871:
.LBB32_128:
	.loc	37 0 13
	movl	$1, %eax
	jmp	.LBB32_129
.LBB32_418:
.Ltmp4872:
	.loc	37 443 13 is_stmt 1
	leaq	.Lalloc_983c0eafc7ebd40f6695894c941b193b(%rip), %rcx
	xorl	%edi, %edi
	movq	%r14, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4873:
.LBB32_162:
	.loc	37 0 13 is_stmt 0
	movl	$2, %eax
	jmp	.LBB32_129
.LBB32_394:
.Ltmp4874:
	.loc	37 443 13 is_stmt 1
	leaq	.Lalloc_69177cbbe364d953a90a8930cefca3fe(%rip), %rcx
	xorl	%edi, %edi
	movq	%r8, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp4875:
.LBB32_166:
	.loc	37 0 13 is_stmt 0
	movl	$3, %eax
	jmp	.LBB32_129
.LBB32_170:
	movl	$4, %eax
	jmp	.LBB32_129
.LBB32_174:
	movl	$5, %eax
	jmp	.LBB32_129
.LBB32_178:
	movl	$6, %eax
	jmp	.LBB32_129
.LBB32_182:
	movl	$7, %eax
.LBB32_129:
	movq	%rax, 16(%rsp)
.LBB32_130:
	vmovaps	96(%rsp), %ymm0
.Ltmp4876:
	.loc	15 746 0 is_stmt 1
	vmovaps	%ymm0, 4288(%rsp)
	vmovaps	128(%rsp), %ymm0
.Ltmp4877:
	.loc	15 746 0 is_stmt 0
	vmovaps	%ymm0, 5024(%rsp)
.Ltmp4878:
	.loc	15 1298 42 is_stmt 1
	leaq	.Lalloc_891dd683d7367e0a1ec5a22f9c2a2aa8(%rip), %rdx
	movq	16(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp4879:
.LBB32_414:
	.loc	15 662 42
	leaq	.Lalloc_1781ea1b97b96e9885c590e9b4440a45(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.Ltmp4880:
.LBB32_415:
	.loc	15 452 44
	leaq	.Lalloc_6b076e9a9e313bc2481504f4da644a5c(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.LBB32_416:
	.loc	15 453 44
	leaq	.Lalloc_f370b9a38141751c9788be81259bacff(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.Ltmp4881:
.Lfunc_end32:
	.size	_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13process_blockB5_, .Lfunc_end32-_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13process_blockB5_
