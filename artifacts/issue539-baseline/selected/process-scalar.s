_RNvXsd_CsdvPQf9CMsz3_17true_peak_limiterNtB5_23PreparedTruePeakLimiterNtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7process:
.Lfunc_begin47:
	.loc	15 2854 0
	.cfi_startproc
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%r13
	.cfi_def_cfa_offset 32
	pushq	%r12
	.cfi_def_cfa_offset 40
	pushq	%rbx
	.cfi_def_cfa_offset 48
	subq	$48, %rsp
	.cfi_def_cfa_offset 96
	.cfi_offset %rbx, -48
	.cfi_offset %r12, -40
	.cfi_offset %r13, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	movq	%rdx, %r15
	movq	%rsi, %r14
	movq	%rdi, %rbx
.Ltmp6813:
	.loc	15 2856 13 prologue_end
	movq	32(%rdx), %rdi
	movq	40(%rdx), %rsi
	testq	%rsi, %rsi
	je	.LBB47_2
	.loc	15 2857 13
	movb	$0, 780(%r14)
.LBB47_2:
.Ltmp6814:
	.loc	55 1017 30
	vxorps	%xmm0, %xmm0, %xmm0
	vmovups	%ymm0, (%rsp)
	movq	$0, 32(%rsp)
.Ltmp6815:
	.loc	55 1095 9
	movq	(%r15), %r12
	movq	8(%r15), %r13
.Ltmp6816:
	.loc	15 2864 13
	movq	80(%r15), %rcx
	.loc	15 2865 13
	leaq	136(%r14), %r8
	.loc	15 2866 13
	leaq	336(%r14), %r9
	movq	%rsp, %rax
	.loc	15 2861 9
	movq	%r14, %rdx
	pushq	%rax
	.cfi_adjust_cfa_offset 8
	pushq	$0
	.cfi_adjust_cfa_offset 8
	vzeroupper
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
	.cfi_adjust_cfa_offset -16
	.loc	15 2870 45
	movq	16(%r15), %rcx
	movq	24(%r15), %r8
	.loc	15 2870 19 is_stmt 0
	movq	%r14, %rdi
	movq	%r12, %rsi
	movq	%r13, %rdx
	movq	%r13, %r9
	callq	_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCorefE13process_blockB5_
	.loc	15 2871 9 is_stmt 1
	movq	32(%rsp), %rax
	movq	%rax, 32(%rbx)
	vmovups	(%rsp), %ymm0
	vmovups	%ymm0, (%rbx)
.Ltmp6817:
	.loc	15 2872 6
	movq	%rbx, %rax
	.loc	15 2872 6 epilogue_begin is_stmt 0
	addq	$48, %rsp
	.cfi_def_cfa_offset 48
	popq	%rbx
	.cfi_def_cfa_offset 40
	popq	%r12
	.cfi_def_cfa_offset 32
	popq	%r13
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	vzeroupper
	retq
.Ltmp6818:
.Lfunc_end47:
	.size	_RNvXsd_CsdvPQf9CMsz3_17true_peak_limiterNtB5_23PreparedTruePeakLimiterNtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7process, .Lfunc_end47-_RNvXsd_CsdvPQf9CMsz3_17true_peak_limiterNtB5_23PreparedTruePeakLimiterNtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7process
