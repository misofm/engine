_RNvXse_CsdvPQf9CMsz3_17true_peak_limiterINtB5_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bankB5_:
.Lfunc_begin59:
	.loc	15 2938 0
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
	subq	$376, %rsp
	.cfi_def_cfa_offset 432
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rdx, %r12
	movq	%rdi, 40(%rsp)
.Ltmp7788:
	.loc	15 2981 13 prologue_end
	movq	40(%rdx), %rax
	movq	%rax, 8(%rsp)
	testq	%rax, %rax
	je	.LBB59_1
	.loc	15 2982 13
	movb	$0, 2152(%rsi)
.LBB59_1:
	.loc	15 0 0 is_stmt 0
	movq	32(%r12), %rax
	movq	%rax, (%rsp)
	movq	%rsi, 32(%rsp)
	.loc	15 2984 51 is_stmt 1
	movzbl	2288(%rsi), %eax
	vxorps	%xmm0, %xmm0, %xmm0
	vmovups	%ymm0, 336(%rsp)
	vmovups	%ymm0, 304(%rsp)
	vmovups	%ymm0, 272(%rsp)
	vmovups	%ymm0, 240(%rsp)
	vmovups	%ymm0, 208(%rsp)
	vmovups	%ymm0, 176(%rsp)
	vmovups	%ymm0, 144(%rsp)
	vmovups	%ymm0, 112(%rsp)
	vmovups	%ymm0, 80(%rsp)
	vmovups	%ymm0, 48(%rsp)
.Ltmp7789:
	.loc	55 1032 9
	movb	%al, 368(%rsp)
	movq	56(%r12), %r15
.Ltmp7790:
	.loc	11 900 12
	cmpq	$1, %r15
	movq	%r15, %r13
	adcq	$-1, %r13
.Ltmp7791:
	.loc	15 2986 25
	testq	%r15, %r15
	je	.LBB59_5
.Ltmp7792:
	.loc	15 2987 23
	cmpq	$1, %r15
	je	.LBB59_3
	.loc	15 0 23 is_stmt 0
	movq	48(%r12), %rbx
	movl	(%rbx), %edi
	.loc	15 2987 23
	movl	4(%rbx), %esi
.Ltmp7793:
	.loc	14 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	8(%rsp), %rdx
	jb	.LBB59_44
	cmpq	%rsi, %rdx
	jb	.LBB59_44
	.loc	14 0 16 is_stmt 0
	movq	32(%rsp), %rax
	leaq	1848(%rax), %r9
	leaq	1648(%rax), %r8
	movq	96(%r12), %rbp
	leaq	2048(%rax), %rdx
	.loc	14 1054 31 is_stmt 1
	subq	%rdi, %rsi
.Ltmp7794:
	.loc	37 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
	leaq	48(%rsp), %rax
	movq	%rdx, 16(%rsp)
.Ltmp7795:
	.loc	15 2988 13
	movq	%rbp, %rcx
	movq	%r8, 24(%rsp)
	movq	%r9, %r14
	pushq	%rax
	.cfi_adjust_cfa_offset 8
	pushq	$0
	.cfi_adjust_cfa_offset 8
	vzeroupper
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp7796:
	.cfi_adjust_cfa_offset -16
	.loc	15 2987 23
	cmpq	$1, %r13
	je	.LBB59_9
	.loc	15 0 23 is_stmt 0
	movl	4(%rbx), %edi
	.loc	15 2987 23
	movl	8(%rbx), %esi
.Ltmp7797:
	.loc	14 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	8(%rsp), %rdx
	jb	.LBB59_44
	cmpq	%rsi, %rdx
	jb	.LBB59_44
	.loc	14 1054 31
	subq	%rdi, %rsi
.Ltmp7798:
	.loc	37 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp7799:
	.loc	15 2995 17
	leaq	88(%rsp), %rax
	movq	16(%rsp), %rdx
	.loc	15 2988 13
	movq	%rbp, %rcx
	movq	24(%rsp), %r8
	movq	%r14, %r9
	pushq	%rax
	.cfi_adjust_cfa_offset 8
	pushq	$1
	.cfi_adjust_cfa_offset 8
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp7800:
	.cfi_adjust_cfa_offset -16
	.loc	15 2986 25
	cmpq	$2, %r15
	je	.LBB59_5
.Ltmp7801:
	.loc	15 2987 23
	cmpq	$2, %r13
	je	.LBB59_14
	.loc	15 0 23 is_stmt 0
	movl	8(%rbx), %edi
	.loc	15 2987 23
	movl	12(%rbx), %esi
.Ltmp7802:
	.loc	14 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	8(%rsp), %rdx
	jb	.LBB59_44
	cmpq	%rsi, %rdx
	jb	.LBB59_44
	.loc	14 1054 31
	subq	%rdi, %rsi
.Ltmp7803:
	.loc	37 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp7804:
	.loc	15 2995 17
	leaq	128(%rsp), %rax
	movq	16(%rsp), %rdx
	.loc	15 2988 13
	movq	%rbp, %rcx
	movq	24(%rsp), %r8
	movq	%r14, %r9
	pushq	%rax
	.cfi_adjust_cfa_offset 8
	pushq	$2
	.cfi_adjust_cfa_offset 8
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp7805:
	.cfi_adjust_cfa_offset -16
	.loc	15 2986 25
	cmpq	$3, %r15
	je	.LBB59_5
.Ltmp7806:
	.loc	15 2987 23
	cmpq	$3, %r13
	je	.LBB59_19
	.loc	15 0 23 is_stmt 0
	movl	12(%rbx), %edi
	.loc	15 2987 23
	movl	16(%rbx), %esi
.Ltmp7807:
	.loc	14 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	8(%rsp), %rdx
	jb	.LBB59_44
	cmpq	%rsi, %rdx
	jb	.LBB59_44
	.loc	14 1054 31
	subq	%rdi, %rsi
.Ltmp7808:
	.loc	37 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp7809:
	.loc	15 2995 17
	leaq	168(%rsp), %rax
	movq	16(%rsp), %rdx
	.loc	15 2988 13
	movq	%rbp, %rcx
	movq	24(%rsp), %r8
	movq	%r14, %r9
	pushq	%rax
	.cfi_adjust_cfa_offset 8
	pushq	$3
	.cfi_adjust_cfa_offset 8
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp7810:
	.cfi_adjust_cfa_offset -16
	.loc	15 2986 25
	cmpq	$4, %r15
	je	.LBB59_5
.Ltmp7811:
	.loc	15 2987 23
	cmpq	$4, %r13
	je	.LBB59_24
	.loc	15 0 23 is_stmt 0
	movl	16(%rbx), %edi
	.loc	15 2987 23
	movl	20(%rbx), %esi
.Ltmp7812:
	.loc	14 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	8(%rsp), %rdx
	jb	.LBB59_44
	cmpq	%rsi, %rdx
	jb	.LBB59_44
	.loc	14 1054 31
	subq	%rdi, %rsi
.Ltmp7813:
	.loc	37 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp7814:
	.loc	15 2995 17
	leaq	208(%rsp), %rax
	movq	16(%rsp), %rdx
	.loc	15 2988 13
	movq	%rbp, %rcx
	movq	24(%rsp), %r8
	movq	%r14, %r9
	pushq	%rax
	.cfi_adjust_cfa_offset 8
	pushq	$4
	.cfi_adjust_cfa_offset 8
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp7815:
	.cfi_adjust_cfa_offset -16
	.loc	15 2986 25
	cmpq	$5, %r15
	je	.LBB59_5
.Ltmp7816:
	.loc	15 2987 23
	cmpq	$5, %r13
	je	.LBB59_29
	.loc	15 0 23 is_stmt 0
	movl	20(%rbx), %edi
	.loc	15 2987 23
	movl	24(%rbx), %esi
.Ltmp7817:
	.loc	14 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	8(%rsp), %rdx
	jb	.LBB59_44
	cmpq	%rsi, %rdx
	jb	.LBB59_44
	.loc	14 1054 31
	subq	%rdi, %rsi
.Ltmp7818:
	.loc	37 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp7819:
	.loc	15 2995 17
	leaq	248(%rsp), %rax
	movq	16(%rsp), %rdx
	.loc	15 2988 13
	movq	%rbp, %rcx
	movq	24(%rsp), %r8
	movq	%r14, %r9
	pushq	%rax
	.cfi_adjust_cfa_offset 8
	pushq	$5
	.cfi_adjust_cfa_offset 8
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp7820:
	.cfi_adjust_cfa_offset -16
	.loc	15 2986 25
	cmpq	$6, %r15
	je	.LBB59_5
.Ltmp7821:
	.loc	15 2987 23
	cmpq	$6, %r13
	je	.LBB59_34
	.loc	15 0 23 is_stmt 0
	movl	24(%rbx), %edi
	.loc	15 2987 23
	movl	28(%rbx), %esi
.Ltmp7822:
	.loc	14 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	8(%rsp), %rdx
	jb	.LBB59_44
	cmpq	%rsi, %rdx
	jb	.LBB59_44
	.loc	14 1054 31
	subq	%rdi, %rsi
.Ltmp7823:
	.loc	37 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp7824:
	.loc	15 2995 17
	leaq	288(%rsp), %rax
	movq	16(%rsp), %rdx
	.loc	15 2988 13
	movq	%rbp, %rcx
	movq	24(%rsp), %r8
	movq	%r14, %r9
	pushq	%rax
	.cfi_adjust_cfa_offset 8
	pushq	$6
	.cfi_adjust_cfa_offset 8
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp7825:
	.cfi_adjust_cfa_offset -16
	.loc	15 2986 25
	cmpq	$7, %r15
	je	.LBB59_5
.Ltmp7826:
	.loc	15 2987 23
	cmpq	$7, %r13
	je	.LBB59_39
	.loc	15 0 23 is_stmt 0
	movl	28(%rbx), %edi
	.loc	15 2987 23
	movl	32(%rbx), %esi
.Ltmp7827:
	.loc	14 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	8(%rsp), %rdx
	jb	.LBB59_44
	cmpq	%rsi, %rdx
	jb	.LBB59_44
	.loc	14 1054 31
	subq	%rdi, %rsi
.Ltmp7828:
	.loc	37 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp7829:
	.loc	15 2995 17
	leaq	328(%rsp), %rax
	movq	16(%rsp), %rdx
	.loc	15 2988 13
	movq	%rbp, %rcx
	movq	24(%rsp), %r8
	movq	%r14, %r9
	pushq	%rax
	.cfi_adjust_cfa_offset 8
	pushq	$7
	.cfi_adjust_cfa_offset 8
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp7830:
	.cfi_adjust_cfa_offset -16
	.loc	15 3003 32
	movq	(%r12), %rsi
	movq	8(%r12), %rdx
	.loc	15 3003 44 is_stmt 0
	movq	16(%r12), %rcx
	movq	24(%r12), %r8
	.loc	15 3003 57
	movl	104(%r12), %r9d
	movq	32(%rsp), %rdi
	.loc	15 3003 18
	callq	_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13process_blockB5_
	leaq	48(%rsp), %rsi
	.loc	15 3005 9 is_stmt 1
	movl	$328, %edx
	movq	40(%rsp), %rbx
	movq	%rbx, %rdi
	callq	*memcpy@GOTPCREL(%rip)
.Ltmp7831:
	.loc	15 2940 6
	movq	%rbx, %rax
	.loc	15 2940 6 epilogue_begin is_stmt 0
	addq	$376, %rsp
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
	retq
.LBB59_44:
	.cfi_def_cfa_offset 432
.Ltmp7832:
	.loc	37 443 13 is_stmt 1
	leaq	.Lalloc_4ac05e629a5e1e4066705fcc17f0e09d(%rip), %rcx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp7833:
.LBB59_5:
	.loc	15 2986 25
	leaq	.Lalloc_2864b855a71e07cd82bfe5e37ca0325a(%rip), %rdx
	movq	%r15, %rdi
	movq	%r15, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB59_3:
	.loc	15 0 25 is_stmt 0
	movl	$1, %edi
.Ltmp7834:
	.loc	15 2987 23 is_stmt 1
	leaq	.Lalloc_8f78f3c6b9c9252b16ab7d7ccc563a9d(%rip), %rdx
	movq	%r15, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB59_9:
	.loc	15 0 23 is_stmt 0
	movl	$2, %edi
	.loc	15 2987 23
	leaq	.Lalloc_8f78f3c6b9c9252b16ab7d7ccc563a9d(%rip), %rdx
	movq	%r15, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB59_14:
	.loc	15 0 23
	movl	$3, %edi
	.loc	15 2987 23
	leaq	.Lalloc_8f78f3c6b9c9252b16ab7d7ccc563a9d(%rip), %rdx
	movq	%r15, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB59_19:
	.loc	15 0 23
	movl	$4, %edi
	.loc	15 2987 23
	leaq	.Lalloc_8f78f3c6b9c9252b16ab7d7ccc563a9d(%rip), %rdx
	movq	%r15, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB59_24:
	.loc	15 0 23
	movl	$5, %edi
	.loc	15 2987 23
	leaq	.Lalloc_8f78f3c6b9c9252b16ab7d7ccc563a9d(%rip), %rdx
	movq	%r15, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB59_29:
	.loc	15 0 23
	movl	$6, %edi
	.loc	15 2987 23
	leaq	.Lalloc_8f78f3c6b9c9252b16ab7d7ccc563a9d(%rip), %rdx
	movq	%r15, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB59_34:
	.loc	15 0 23
	movl	$7, %edi
	.loc	15 2987 23
	leaq	.Lalloc_8f78f3c6b9c9252b16ab7d7ccc563a9d(%rip), %rdx
	movq	%r15, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB59_39:
	.loc	15 0 23
	movl	$8, %edi
	.loc	15 2987 23
	leaq	.Lalloc_8f78f3c6b9c9252b16ab7d7ccc563a9d(%rip), %rdx
	movq	%r15, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp7835:
.Lfunc_end59:
	.size	_RNvXse_CsdvPQf9CMsz3_17true_peak_limiterINtB5_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bankB5_, .Lfunc_end59-_RNvXse_CsdvPQf9CMsz3_17true_peak_limiterINtB5_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bankB5_
