_RNvXse_CsdvPQf9CMsz3_17true_peak_limiterINtB5_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bankB5_:
.Lfunc_begin61:
	.loc	1 3090 0
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
.Ltmp12978:
	.loc	1 3133 13 prologue_end
	movq	40(%rdx), %rax
	movq	%rax, 8(%rsp)
	testq	%rax, %rax
	je	.LBB61_1
	.loc	1 3134 13
	movb	$0, 2152(%rsi)
.LBB61_1:
	.loc	1 0 0 is_stmt 0
	movq	32(%r12), %rax
	movq	%rax, (%rsp)
	movq	%rsi, 32(%rsp)
	.loc	1 3136 51 is_stmt 1
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
.Ltmp12979:
	.loc	2 1032 9
	movb	%al, 368(%rsp)
	movq	56(%r12), %r15
.Ltmp12980:
	.loc	3 900 12
	cmpq	$1, %r15
	movq	%r15, %r13
	adcq	$-1, %r13
.Ltmp12981:
	.loc	1 3138 25
	testq	%r15, %r15
	je	.LBB61_5
.Ltmp12982:
	.loc	1 3139 23
	cmpq	$1, %r15
	je	.LBB61_3
	.loc	1 0 23 is_stmt 0
	movq	48(%r12), %rbx
	movl	(%rbx), %edi
	.loc	1 3139 23
	movl	4(%rbx), %esi
.Ltmp12983:
	.loc	4 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	8(%rsp), %rdx
	jb	.LBB61_44
	cmpq	%rsi, %rdx
	jb	.LBB61_44
	.loc	4 0 16 is_stmt 0
	movq	32(%rsp), %rax
	leaq	1848(%rax), %r9
	leaq	1648(%rax), %r8
	movq	96(%r12), %rbp
	leaq	2048(%rax), %rdx
	.loc	4 1054 31 is_stmt 1
	subq	%rdi, %rsi
.Ltmp12984:
	.loc	5 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
	leaq	48(%rsp), %rax
	movq	%rdx, 16(%rsp)
.Ltmp12985:
	.loc	1 3140 13
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
.Ltmp12986:
	.cfi_adjust_cfa_offset -16
	.loc	1 3139 23
	cmpq	$1, %r13
	je	.LBB61_9
	.loc	1 0 23 is_stmt 0
	movl	4(%rbx), %edi
	.loc	1 3139 23
	movl	8(%rbx), %esi
.Ltmp12987:
	.loc	4 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	8(%rsp), %rdx
	jb	.LBB61_44
	cmpq	%rsi, %rdx
	jb	.LBB61_44
	.loc	4 1054 31
	subq	%rdi, %rsi
.Ltmp12988:
	.loc	5 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp12989:
	.loc	1 3147 17
	leaq	88(%rsp), %rax
	movq	16(%rsp), %rdx
	.loc	1 3140 13
	movq	%rbp, %rcx
	movq	24(%rsp), %r8
	movq	%r14, %r9
	pushq	%rax
	.cfi_adjust_cfa_offset 8
	pushq	$1
	.cfi_adjust_cfa_offset 8
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp12990:
	.cfi_adjust_cfa_offset -16
	.loc	1 3138 25
	cmpq	$2, %r15
	je	.LBB61_5
.Ltmp12991:
	.loc	1 3139 23
	cmpq	$2, %r13
	je	.LBB61_14
	.loc	1 0 23 is_stmt 0
	movl	8(%rbx), %edi
	.loc	1 3139 23
	movl	12(%rbx), %esi
.Ltmp12992:
	.loc	4 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	8(%rsp), %rdx
	jb	.LBB61_44
	cmpq	%rsi, %rdx
	jb	.LBB61_44
	.loc	4 1054 31
	subq	%rdi, %rsi
.Ltmp12993:
	.loc	5 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp12994:
	.loc	1 3147 17
	leaq	128(%rsp), %rax
	movq	16(%rsp), %rdx
	.loc	1 3140 13
	movq	%rbp, %rcx
	movq	24(%rsp), %r8
	movq	%r14, %r9
	pushq	%rax
	.cfi_adjust_cfa_offset 8
	pushq	$2
	.cfi_adjust_cfa_offset 8
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp12995:
	.cfi_adjust_cfa_offset -16
	.loc	1 3138 25
	cmpq	$3, %r15
	je	.LBB61_5
.Ltmp12996:
	.loc	1 3139 23
	cmpq	$3, %r13
	je	.LBB61_19
	.loc	1 0 23 is_stmt 0
	movl	12(%rbx), %edi
	.loc	1 3139 23
	movl	16(%rbx), %esi
.Ltmp12997:
	.loc	4 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	8(%rsp), %rdx
	jb	.LBB61_44
	cmpq	%rsi, %rdx
	jb	.LBB61_44
	.loc	4 1054 31
	subq	%rdi, %rsi
.Ltmp12998:
	.loc	5 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp12999:
	.loc	1 3147 17
	leaq	168(%rsp), %rax
	movq	16(%rsp), %rdx
	.loc	1 3140 13
	movq	%rbp, %rcx
	movq	24(%rsp), %r8
	movq	%r14, %r9
	pushq	%rax
	.cfi_adjust_cfa_offset 8
	pushq	$3
	.cfi_adjust_cfa_offset 8
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp13000:
	.cfi_adjust_cfa_offset -16
	.loc	1 3138 25
	cmpq	$4, %r15
	je	.LBB61_5
.Ltmp13001:
	.loc	1 3139 23
	cmpq	$4, %r13
	je	.LBB61_24
	.loc	1 0 23 is_stmt 0
	movl	16(%rbx), %edi
	.loc	1 3139 23
	movl	20(%rbx), %esi
.Ltmp13002:
	.loc	4 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	8(%rsp), %rdx
	jb	.LBB61_44
	cmpq	%rsi, %rdx
	jb	.LBB61_44
	.loc	4 1054 31
	subq	%rdi, %rsi
.Ltmp13003:
	.loc	5 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp13004:
	.loc	1 3147 17
	leaq	208(%rsp), %rax
	movq	16(%rsp), %rdx
	.loc	1 3140 13
	movq	%rbp, %rcx
	movq	24(%rsp), %r8
	movq	%r14, %r9
	pushq	%rax
	.cfi_adjust_cfa_offset 8
	pushq	$4
	.cfi_adjust_cfa_offset 8
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp13005:
	.cfi_adjust_cfa_offset -16
	.loc	1 3138 25
	cmpq	$5, %r15
	je	.LBB61_5
.Ltmp13006:
	.loc	1 3139 23
	cmpq	$5, %r13
	je	.LBB61_29
	.loc	1 0 23 is_stmt 0
	movl	20(%rbx), %edi
	.loc	1 3139 23
	movl	24(%rbx), %esi
.Ltmp13007:
	.loc	4 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	8(%rsp), %rdx
	jb	.LBB61_44
	cmpq	%rsi, %rdx
	jb	.LBB61_44
	.loc	4 1054 31
	subq	%rdi, %rsi
.Ltmp13008:
	.loc	5 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp13009:
	.loc	1 3147 17
	leaq	248(%rsp), %rax
	movq	16(%rsp), %rdx
	.loc	1 3140 13
	movq	%rbp, %rcx
	movq	24(%rsp), %r8
	movq	%r14, %r9
	pushq	%rax
	.cfi_adjust_cfa_offset 8
	pushq	$5
	.cfi_adjust_cfa_offset 8
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp13010:
	.cfi_adjust_cfa_offset -16
	.loc	1 3138 25
	cmpq	$6, %r15
	je	.LBB61_5
.Ltmp13011:
	.loc	1 3139 23
	cmpq	$6, %r13
	je	.LBB61_34
	.loc	1 0 23 is_stmt 0
	movl	24(%rbx), %edi
	.loc	1 3139 23
	movl	28(%rbx), %esi
.Ltmp13012:
	.loc	4 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	8(%rsp), %rdx
	jb	.LBB61_44
	cmpq	%rsi, %rdx
	jb	.LBB61_44
	.loc	4 1054 31
	subq	%rdi, %rsi
.Ltmp13013:
	.loc	5 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp13014:
	.loc	1 3147 17
	leaq	288(%rsp), %rax
	movq	16(%rsp), %rdx
	.loc	1 3140 13
	movq	%rbp, %rcx
	movq	24(%rsp), %r8
	movq	%r14, %r9
	pushq	%rax
	.cfi_adjust_cfa_offset 8
	pushq	$6
	.cfi_adjust_cfa_offset 8
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp13015:
	.cfi_adjust_cfa_offset -16
	.loc	1 3138 25
	cmpq	$7, %r15
	je	.LBB61_5
.Ltmp13016:
	.loc	1 3139 23
	cmpq	$7, %r13
	je	.LBB61_39
	.loc	1 0 23 is_stmt 0
	movl	28(%rbx), %edi
	.loc	1 3139 23
	movl	32(%rbx), %esi
.Ltmp13017:
	.loc	4 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	8(%rsp), %rdx
	jb	.LBB61_44
	cmpq	%rsi, %rdx
	jb	.LBB61_44
	.loc	4 1054 31
	subq	%rdi, %rsi
.Ltmp13018:
	.loc	5 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp13019:
	.loc	1 3147 17
	leaq	328(%rsp), %rax
	movq	16(%rsp), %rdx
	.loc	1 3140 13
	movq	%rbp, %rcx
	movq	24(%rsp), %r8
	movq	%r14, %r9
	pushq	%rax
	.cfi_adjust_cfa_offset 8
	pushq	$7
	.cfi_adjust_cfa_offset 8
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp13020:
	.cfi_adjust_cfa_offset -16
	.loc	1 3155 32
	movq	(%r12), %rsi
	movq	8(%r12), %rdx
	.loc	1 3155 44 is_stmt 0
	movq	16(%r12), %rcx
	movq	24(%r12), %r8
	.loc	1 3155 57
	movl	104(%r12), %r9d
	movq	32(%rsp), %rdi
	.loc	1 3155 18
	callq	_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13process_blockB5_
	leaq	48(%rsp), %rsi
	.loc	1 3157 9 is_stmt 1
	movl	$328, %edx
	movq	40(%rsp), %rbx
	movq	%rbx, %rdi
	callq	*memcpy@GOTPCREL(%rip)
.Ltmp13021:
	.loc	1 3092 6
	movq	%rbx, %rax
	.loc	1 3092 6 epilogue_begin is_stmt 0
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
.LBB61_44:
	.cfi_def_cfa_offset 432
.Ltmp13022:
	.loc	5 443 13 is_stmt 1
	leaq	.Lalloc_cdadbf7cf6c6abfb3682542d463ff330(%rip), %rcx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp13023:
.LBB61_5:
	.loc	1 3138 25
	leaq	.Lalloc_8790d798703aa1c903d88be7cb1df9ea(%rip), %rdx
	movq	%r15, %rdi
	movq	%r15, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB61_3:
	.loc	1 0 25 is_stmt 0
	movl	$1, %edi
.Ltmp13024:
	.loc	1 3139 23 is_stmt 1
	leaq	.Lalloc_55e533af89cecaf7e999abe53b851675(%rip), %rdx
	movq	%r15, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB61_9:
	.loc	1 0 23 is_stmt 0
	movl	$2, %edi
	.loc	1 3139 23
	leaq	.Lalloc_55e533af89cecaf7e999abe53b851675(%rip), %rdx
	movq	%r15, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB61_14:
	.loc	1 0 23
	movl	$3, %edi
	.loc	1 3139 23
	leaq	.Lalloc_55e533af89cecaf7e999abe53b851675(%rip), %rdx
	movq	%r15, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB61_19:
	.loc	1 0 23
	movl	$4, %edi
	.loc	1 3139 23
	leaq	.Lalloc_55e533af89cecaf7e999abe53b851675(%rip), %rdx
	movq	%r15, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB61_24:
	.loc	1 0 23
	movl	$5, %edi
	.loc	1 3139 23
	leaq	.Lalloc_55e533af89cecaf7e999abe53b851675(%rip), %rdx
	movq	%r15, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB61_29:
	.loc	1 0 23
	movl	$6, %edi
	.loc	1 3139 23
	leaq	.Lalloc_55e533af89cecaf7e999abe53b851675(%rip), %rdx
	movq	%r15, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB61_34:
	.loc	1 0 23
	movl	$7, %edi
	.loc	1 3139 23
	leaq	.Lalloc_55e533af89cecaf7e999abe53b851675(%rip), %rdx
	movq	%r15, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB61_39:
	.loc	1 0 23
	movl	$8, %edi
	.loc	1 3139 23
	leaq	.Lalloc_55e533af89cecaf7e999abe53b851675(%rip), %rdx
	movq	%r15, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp13025:
.Lfunc_end61:
	.size	_RNvXse_CsdvPQf9CMsz3_17true_peak_limiterINtB5_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bankB5_, .Lfunc_end61-_RNvXse_CsdvPQf9CMsz3_17true_peak_limiterINtB5_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bankB5_
