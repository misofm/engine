_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults:
.Lfunc_begin17:
	.loc	1 580 0 is_stmt 1
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%r13
	.cfi_def_cfa_offset 40
	pushq	%r12
	.cfi_def_cfa_offset 48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	subq	$168, %rsp
	.cfi_def_cfa_offset 224
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rdi, (%rsp)
.Ltmp3350:
	.loc	7 1714 9 prologue_end
	testq	%rcx, %rcx
.Ltmp3351:
	.loc	6 180 28
	je	.LBB17_18
.Ltmp3352:
	.loc	6 0 28 is_stmt 0
	movq	%rdx, %r12
	movq	(%rsp), %r10
	movq	168(%r10), %rdi
	movq	160(%r10), %rax
	movq	%rax, 16(%rsp)
	movq	8(%rsi), %r9
	cmpq	$32, %r9
	movq	%rdi, 48(%rsp)
	jb	.LBB17_20
	shlq	$2, %rcx
	leaq	(%rcx,%rcx,2), %rax
	movq	%rax, 72(%rsp)
	movl	%r8d, %eax
	vcvtsi2sd	%rax, %xmm15, %xmm0
	vmovsd	%xmm0, 24(%rsp)
	movq	(%rsi), %rax
	movq	%rax, 56(%rsp)
	movq	184(%r10), %rax
	movq	176(%r10), %rcx
	movq	%rcx, 96(%rsp)
	movq	128(%r10), %rcx
	movq	%rcx, 80(%rsp)
	movq	136(%r10), %rcx
	movq	152(%r10), %rsi
	movq	%rsi, 8(%rsp)
	movq	144(%r10), %rsi
	movq	%rsi, 64(%rsp)
	movq	%rcx, 32(%rsp)
.Ltmp3353:
	.loc	1 582 13 is_stmt 1
	negq	%rcx
	movq	%rcx, 88(%rsp)
	movq	%rax, 40(%rsp)
	negq	%rax
	movq	%rax, 104(%rsp)
	negq	%rdi
	movq	%rdi, 112(%rsp)
	movl	$3, %ebx
	movq	$-1, %rbp
	xorl	%r14d, %r14d
	vmovsd	.LCPI17_4(%rip), %xmm2
.Ltmp3354:
	.loc	1 0 13 is_stmt 0
.Ltmp3355:
	.p2align	4
.LBB17_3:
	movq	112(%rsp), %rax
.Ltmp3356:
	.loc	7 656 28 is_stmt 1
	addq	%rbp, %rax
.Ltmp3357:
	.loc	1 582 13
	cmpq	$-1, %rax
	je	.LBB17_21
	.loc	1 582 39 is_stmt 0
	vmovss	8(%r12,%r14), %xmm0
	movq	16(%rsp), %rax
	.loc	1 582 13
	vmovss	%xmm0, 4(%rax,%rbp,4)
.Ltmp3358:
	.loc	43 82 17 is_stmt 1
	vcvtss2sd	%xmm0, %xmm0, %xmm0
.Ltmp3359:
	.loc	1 895 20
	vmulsd	24(%rsp), %xmm0, %xmm0
	vdivsd	.LCPI17_0(%rip), %xmm0, %xmm0
	.loc	1 895 19 is_stmt 0
	vaddsd	.LCPI17_1(%rip), %xmm0, %xmm0
.Ltmp3360:
	.loc	46 1763 9 is_stmt 1
	vroundsd	$9, %xmm0, %xmm0, %xmm0
	vmovq	%xmm0, %rax
.Ltmp3361:
	.loc	1 896 9
	testq	%rax, %rax
	sets	%cl
	movabsq	$9223372036854775807, %rsi
	andq	%rsi, %rax
	movabsq	$-4503599627370496, %rsi
	addq	%rax, %rsi
	shrq	$53, %rsi
	cmpl	$1023, %esi
	setb	%sil
	andb	%cl, %sil
	movabsq	$9218868437227405311, %rcx
	cmpq	%rcx, %rax
	setg	%al
	orb	%sil, %al
	movl	$1, %eax
	jne	.LBB17_10
	.loc	1 899 5
	vcvttsd2si	%xmm0, %rax
	movq	%rax, %rcx
	sarq	$63, %rcx
	vsubsd	.LCPI17_2(%rip), %xmm0, %xmm1
	vcvttsd2si	%xmm1, %rsi
	andq	%rcx, %rsi
	orq	%rax, %rsi
	vxorpd	%xmm1, %xmm1, %xmm1
	vucomisd	%xmm1, %xmm0
	movl	$0, %eax
	cmovaeq	%rsi, %rax
	vucomisd	.LCPI17_3(%rip), %xmm0
	movq	$-1, %rcx
	ja	.LBB17_7
	.loc	1 0 5 is_stmt 0
	movq	%rax, %rcx
.LBB17_7:
	movq	56(%rsp), %rsi
	movq	%rsi, %rax
.Ltmp3362:
	.loc	10 1078 5 is_stmt 1
	cmpq	%rcx, %rsi
	jb	.LBB17_9
.Ltmp3363:
	.loc	10 0 5 is_stmt 0
	movq	%rcx, %rax
.LBB17_9:
.Ltmp3364:
	.loc	1 400 22 is_stmt 1
	incq	%rax
.Ltmp3365:
.LBB17_10:
	.loc	1 0 22 is_stmt 0
	movq	104(%rsp), %rcx
	addq	%rbp, %rcx
.Ltmp3366:
	.loc	1 583 13 is_stmt 1
	cmpq	$-1, %rcx
	je	.LBB17_22
	.loc	1 0 13 is_stmt 0
	movq	%rax, %rcx
.Ltmp3367:
	.loc	10 2025 24 is_stmt 1
	cmpq	%r9, %rax
	jb	.LBB17_13
	.loc	10 0 24 is_stmt 0
	movq	%r9, %rcx
.LBB17_13:
	movl	$32, %esi
	.loc	10 2025 24 is_stmt 1
	cmpq	$32, %rax
.Ltmp3368:
	.loc	1 404 25
	jb	.LBB17_15
.Ltmp3369:
	.loc	1 0 25 is_stmt 0
	movl	%ecx, %esi
.LBB17_15:
	movq	88(%rsp), %rax
	leaq	(%rax,%rbp), %r13
	movq	%r9, %r15
.Ltmp3370:
	.loc	1 404 25 is_stmt 1
	movl	%r9d, %eax
	subl	%esi, %eax
	movq	96(%rsp), %rcx
.Ltmp3371:
	.loc	1 583 13
	movl	%esi, (%rcx,%r14)
	movl	%esi, 4(%rcx,%r14)
	movl	%eax, 8(%rcx,%r14)
	.loc	1 584 68
	vmovss	(%r12,%r14), %xmm0
.Ltmp3372:
	.loc	43 82 17
	vcvtss2sd	%xmm0, %xmm0, %xmm0
.Ltmp3373:
	.loc	1 883 22
	vaddsd	%xmm2, %xmm0, %xmm0
.Ltmp3374:
	.loc	44 232 10
	vmulsd	.LCPI17_5(%rip), %xmm0, %xmm0
.Ltmp3375:
	.loc	44 58 5
	callq	*_RNvNtNtCshmZ46FhrXRY_4math8vendored4exp24exp2@GOTPCREL(%rip)
.Ltmp3376:
	.loc	1 584 13
	cmpq	$-1, %r13
	je	.LBB17_23
.Ltmp3377:
	.loc	1 883 5
	vcvtsd2ss	%xmm0, %xmm0, %xmm0
	movq	80(%rsp), %rax
.Ltmp3378:
	.loc	1 584 13
	vmovss	%xmm0, -12(%rax,%rbx,4)
	vmovss	%xmm0, -8(%rax,%rbx,4)
	movq	$0, -4(%rax,%rbx,4)
	movq	%r12, %r13
	.loc	1 585 72
	vmovss	4(%r12,%r14), %xmm0
.Ltmp3379:
	.loc	43 82 17
	vcvtss2sd	%xmm0, %xmm0, %xmm0
.Ltmp3380:
	.loc	1 890 30
	vmulsd	.LCPI17_6(%rip), %xmm0, %xmm0
	.loc	1 890 29 is_stmt 0
	vmulsd	24(%rsp), %xmm0, %xmm0
	.loc	1 890 22
	vmovsd	.LCPI17_4(%rip), %xmm1
	vdivsd	%xmm0, %xmm1, %xmm0
.Ltmp3381:
	.loc	44 52 5 is_stmt 1
	callq	*_RNvNtNtCshmZ46FhrXRY_4math8vendored3exp3exp@GOTPCREL(%rip)
.Ltmp3382:
	.loc	1 585 13
	incq	%rbp
	cmpq	%rbp, 8(%rsp)
	je	.LBB17_24
	.loc	1 585 0 is_stmt 0
	addq	$12, %r14
.Ltmp3383:
	.loc	1 890 5 is_stmt 1
	vmovsd	.LCPI17_7(%rip), %xmm1
	vsubsd	%xmm0, %xmm1, %xmm0
	vcvtsd2ss	%xmm0, %xmm0, %xmm0
	movq	64(%rsp), %rax
.Ltmp3384:
	.loc	1 585 13
	vmovss	%xmm0, -12(%rax,%rbx,4)
	vmovss	%xmm0, -8(%rax,%rbx,4)
	movq	$0, -4(%rax,%rbx,4)
	.loc	1 585 0 is_stmt 0
	addq	$4, %rbx
.Ltmp3385:
	.loc	7 1714 9 is_stmt 1
	cmpq	%r14, 72(%rsp)
	movq	%r13, %r12
	movq	%r15, %r9
	vmovsd	.LCPI17_4(%rip), %xmm2
.Ltmp3386:
	.loc	6 180 28
	jne	.LBB17_3
.Ltmp3387:
.LBB17_18:
	.loc	6 0 28 is_stmt 0
	movq	(%rsp), %rdi
	.loc	1 587 14 epilogue_begin is_stmt 1
	addq	$168, %rsp
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	jmp	_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState13clear_runtime
.LBB17_20:
	.cfi_def_cfa_offset 224
.Ltmp3388:
	.loc	1 582 13
	testq	%rdi, %rdi
	jne	.LBB17_19
.LBB17_21:
	leaq	.Lalloc_a20102380cf44fc1efe2c5344cffc5c6(%rip), %rdx
	movq	48(%rsp), %rdi
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB17_19:
	.loc	1 582 39 is_stmt 0
	vmovss	8(%r12), %xmm0
	movq	16(%rsp), %rax
	.loc	1 582 13
	vmovss	%xmm0, (%rax)
.Ltmp3389:
	.loc	48 185 27 is_stmt 1
	movq	$32, 120(%rsp)
	movq	%r9, 128(%rsp)
	leaq	120(%rsp), %rax
.Ltmp3390:
	.loc	48 172 21
	movq	%rax, 136(%rsp)
	leaq	_RNvXsZ_NtNtCs4NRVxsYgnAr_4core3fmt3numjNtB7_5Debug3fmt(%rip), %rax
	movq	%rax, 144(%rsp)
	leaq	128(%rsp), %rcx
	movq	%rcx, 152(%rsp)
	movq	%rax, 160(%rsp)
.Ltmp3391:
	.loc	48 172 21 is_stmt 0
	leaq	.Lalloc_c04ec3b757b5ea96b9c02edd3d57a08e(%rip), %rdi
	leaq	.Lalloc_7a177f1354713a4ff75927eff53b346c(%rip), %rdx
	leaq	136(%rsp), %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking9panic_fmt@GOTPCREL(%rip)
.Ltmp3392:
.LBB17_22:
	.loc	1 583 13 is_stmt 1
	leaq	.Lalloc_1134635281c242588a9fd414ca7c27d6(%rip), %rdx
	movq	40(%rsp), %rdi
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB17_23:
	.loc	1 584 13
	leaq	.Lalloc_954640f53e67cee7e83cb536615a8349(%rip), %rdx
	movq	32(%rsp), %rdi
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB17_24:
	.loc	1 585 13
	leaq	.Lalloc_1f687d87a0376ba32176b9f54c6cf767(%rip), %rdx
	movq	8(%rsp), %rdi
	movq	%rdi, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp3393:
.Lfunc_end17:
	.size	_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults, .Lfunc_end17-_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults
