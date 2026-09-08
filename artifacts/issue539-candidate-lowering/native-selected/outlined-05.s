_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest:
.Lfunc_begin16:
	.cfi_startproc
	.loc	1 644 32 prologue_end
	movq	(%rdi), %rax
	movq	8(%rdi), %rcx
	.loc	1 0 32 is_stmt 0
.Ltmp3280:
	.p2align	4
.LBB16_1:
.Ltmp3281:
	.loc	15 1504 12 is_stmt 1
	testq	%rcx, %rcx
	je	.LBB16_6
.Ltmp3282:
	.loc	10 1078 5
	cmpq	$32, %rcx
	movl	$32, %edx
	cmovbq	%rcx, %rdx
.Ltmp3283:
	.loc	16 961 18
	leal	(,%rdx,4), %r8d
	xorl	%esi, %esi
	xorl	%r9d, %r9d
.Ltmp3284:
	.loc	16 0 18 is_stmt 0
.Ltmp3285:
	.p2align	4
.LBB16_3:
	.loc	17 134 13 is_stmt 1
	orl	(%rax,%r9), %esi
.Ltmp3286:
	.loc	7 1714 9
	addq	$4, %r9
	cmpq	%r9, %r8
.Ltmp3287:
	.loc	6 180 28
	jne	.LBB16_3
.Ltmp3288:
	.loc	18 863 18
	leaq	(%rax,%rdx,4), %rax
.Ltmp3289:
	.loc	19 2054 74
	subq	%rdx, %rcx
.Ltmp3290:
	.loc	17 136 12
	testl	%esi, %esi
	je	.LBB16_1
.Ltmp3291:
	.loc	17 0 12 is_stmt 0
	xorl	%eax, %eax
	.loc	1 655 6 is_stmt 1
	retq
.LBB16_6:
	.loc	1 645 39
	movq	16(%rdi), %rax
	movq	24(%rdi), %rcx
	.loc	1 0 39 is_stmt 0
.Ltmp3292:
	.p2align	4
.LBB16_7:
.Ltmp3293:
	.loc	15 1504 12 is_stmt 1
	testq	%rcx, %rcx
	je	.LBB16_12
.Ltmp3294:
	.loc	10 1078 5
	cmpq	$32, %rcx
	movl	$32, %edx
	cmovbq	%rcx, %rdx
.Ltmp3295:
	.loc	16 961 18
	leal	(,%rdx,4), %r8d
	xorl	%esi, %esi
	xorl	%r9d, %r9d
.Ltmp3296:
	.loc	16 0 18 is_stmt 0
.Ltmp3297:
	.p2align	4
.LBB16_9:
	.loc	17 134 13 is_stmt 1
	orl	(%rax,%r9), %esi
.Ltmp3298:
	.loc	7 1714 9
	addq	$4, %r9
	cmpq	%r9, %r8
.Ltmp3299:
	.loc	6 180 28
	jne	.LBB16_9
.Ltmp3300:
	.loc	18 863 18
	leaq	(%rax,%rdx,4), %rax
.Ltmp3301:
	.loc	19 2054 74
	subq	%rdx, %rcx
.Ltmp3302:
	.loc	17 136 12
	testl	%esi, %esi
	je	.LBB16_7
.Ltmp3303:
	.loc	17 0 12 is_stmt 0
	xorl	%eax, %eax
	.loc	1 655 6 is_stmt 1
	retq
.LBB16_12:
	.loc	1 646 39
	movq	64(%rdi), %rax
	movq	72(%rdi), %rcx
	.loc	1 0 39 is_stmt 0
.Ltmp3304:
	.p2align	4
.LBB16_13:
.Ltmp3305:
	.loc	15 1504 12 is_stmt 1
	testq	%rcx, %rcx
	je	.LBB16_18
.Ltmp3306:
	.loc	10 1078 5
	cmpq	$32, %rcx
	movl	$32, %edx
	cmovbq	%rcx, %rdx
.Ltmp3307:
	.loc	16 961 18
	leal	(,%rdx,4), %r8d
	xorl	%esi, %esi
	xorl	%r9d, %r9d
.Ltmp3308:
	.loc	16 0 18 is_stmt 0
.Ltmp3309:
	.p2align	4
.LBB16_15:
	.loc	17 134 13 is_stmt 1
	orl	(%rax,%r9), %esi
.Ltmp3310:
	.loc	7 1714 9
	addq	$4, %r9
	cmpq	%r9, %r8
.Ltmp3311:
	.loc	6 180 28
	jne	.LBB16_15
.Ltmp3312:
	.loc	18 863 18
	leaq	(%rax,%rdx,4), %rax
.Ltmp3313:
	.loc	19 2054 74
	subq	%rdx, %rcx
.Ltmp3314:
	.loc	17 136 12
	testl	%esi, %esi
	je	.LBB16_13
.Ltmp3315:
	.loc	17 0 12 is_stmt 0
	xorl	%eax, %eax
	.loc	1 655 6 is_stmt 1
	retq
.LBB16_18:
	.loc	1 647 32
	movq	32(%rdi), %rax
	movq	40(%rdi), %rcx
.Ltmp3316:
	.loc	6 314 17
	shlq	$2, %rcx
	xorl	%edx, %edx
	.loc	6 0 17 is_stmt 0
.Ltmp3317:
	.p2align	4
.LBB16_19:
.Ltmp3318:
	.loc	7 1714 9 is_stmt 1
	cmpq	%rdx, %rcx
.Ltmp3319:
	.loc	6 180 28
	je	.LBB16_22
.Ltmp3320:
	.loc	1 677 31
	cmpl	$1065353216, (%rax,%rdx)
.Ltmp3321:
	.loc	6 315 25
	leaq	4(%rdx), %rdx
	je	.LBB16_19
.Ltmp3322:
	.loc	6 0 25 is_stmt 0
	xorl	%eax, %eax
	.loc	1 655 6 is_stmt 1
	retq
.LBB16_22:
	.loc	1 648 32
	movq	48(%rdi), %rax
	movq	56(%rdi), %rcx
.Ltmp3323:
	.loc	6 314 17
	shlq	$2, %rcx
	xorl	%edx, %edx
	.loc	6 0 17 is_stmt 0
.Ltmp3324:
	.p2align	4
.LBB16_23:
.Ltmp3325:
	.loc	7 1714 9 is_stmt 1
	cmpq	%rdx, %rcx
.Ltmp3326:
	.loc	6 180 28
	je	.LBB16_26
.Ltmp3327:
	.loc	1 677 31
	cmpl	$1065353216, (%rax,%rdx)
.Ltmp3328:
	.loc	6 315 25
	leaq	4(%rdx), %rdx
	je	.LBB16_23
.Ltmp3329:
	.loc	6 0 25 is_stmt 0
	xorl	%eax, %eax
	.loc	1 655 6 is_stmt 1
	retq
.LBB16_26:
	.loc	1 649 32
	movq	80(%rdi), %rcx
	movq	88(%rdi), %rdx
.Ltmp3330:
	.loc	6 314 17
	shlq	$2, %rdx
	xorl	%esi, %esi
	.loc	6 0 17 is_stmt 0
.Ltmp3331:
	.p2align	4
.LBB16_27:
.Ltmp3332:
	.loc	7 1714 9 is_stmt 1
	cmpq	%rsi, %rdx
.Ltmp3333:
	.loc	6 180 28
	je	.LBB16_29
.Ltmp3334:
	.loc	6 0 28 is_stmt 0
	xorl	%eax, %eax
.Ltmp3335:
	.loc	1 677 31 is_stmt 1
	cmpl	$1065353216, (%rcx,%rsi)
.Ltmp3336:
	.loc	6 315 25
	leaq	4(%rsi), %rsi
	je	.LBB16_27
	jmp	.LBB16_33
.Ltmp3337:
.LBB16_29:
	.loc	1 650 16
	movq	96(%rdi), %rcx
	movq	104(%rdi), %rdx
	.loc	1 653 22
	movq	176(%rdi), %rsi
	movq	184(%rdi), %rax
.Ltmp3338:
	.loc	10 1078 5
	cmpq	%rdx, %rax
	cmovbq	%rax, %rdx
	xorl	%edi, %edi
.Ltmp3339:
	.loc	10 0 5 is_stmt 0
.Ltmp3340:
	.p2align	4
.LBB16_30:
	.loc	14 304 12 is_stmt 1
	cmpq	%rdi, %rdx
	je	.LBB16_31
.Ltmp3341:
	.loc	22 2494 21
	movl	(%rsi), %eax
.Ltmp3342:
	.loc	1 654 54
	vcvtsi2ss	%rax, %xmm15, %xmm0
.Ltmp3343:
	.loc	41 1244 18
	vmovd	%xmm0, %r8d
.Ltmp3344:
	.loc	22 2494 21
	addq	$12, %rsi
	xorl	%eax, %eax
.Ltmp3345:
	.loc	1 654 37
	cmpl	%r8d, (%rcx,%rdi,4)
.Ltmp3346:
	.loc	14 308 13
	leaq	1(%rdi), %rdi
.Ltmp3347:
	.loc	22 2494 21
	je	.LBB16_30
.Ltmp3348:
.LBB16_33:
	.loc	1 655 6
	retq
.LBB16_31:
	.loc	1 0 6 is_stmt 0
	movb	$1, %al
	.loc	1 655 6
	retq
.Ltmp3349:
.Lfunc_end16:
	.size	_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest, .Lfunc_end16-_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest
