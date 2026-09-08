_RNvXse_CsdvPQf9CMsz3_17true_peak_limiterINtB5_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank17process_bank_monoB5_:
.Lfunc_begin60:
	.loc	15 2930 0 is_stmt 1
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
	subq	$896, %rsp
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	movq	%rdi, 760(%rsp)
.Ltmp7836:
	.loc	15 2981 13 prologue_end
	movq	40(%rdx), %rax
	movq	%rax, 64(%rsp)
	testq	%rax, %rax
	je	.LBB60_2
	.loc	15 2982 13
	movb	$0, 2152(%rsi)
.LBB60_2:
	.loc	15 0 0 is_stmt 0
	movq	32(%rdx), %rax
	movq	%rax, 8(%rsp)
	movq	%rsi, 56(%rsp)
	.loc	15 2984 51 is_stmt 1
	movzbl	2288(%rsi), %eax
	vxorps	%xmm0, %xmm0, %xmm0
	vmovaps	%ymm0, 3840(%rsp)
	vmovaps	%ymm0, 3808(%rsp)
	vmovaps	%ymm0, 3776(%rsp)
	vmovaps	%ymm0, 3744(%rsp)
	vmovaps	%ymm0, 3712(%rsp)
	vmovaps	%ymm0, 3680(%rsp)
	vmovaps	%ymm0, 3648(%rsp)
	vmovaps	%ymm0, 3616(%rsp)
	vmovaps	%ymm0, 3584(%rsp)
	vmovaps	%ymm0, 3552(%rsp)
.Ltmp7837:
	.loc	55 1032 9
	movb	%al, 3872(%rsp)
	movq	%rdx, 160(%rsp)
	movq	56(%rdx), %rbx
.Ltmp7838:
	.loc	11 900 12
	cmpq	$1, %rbx
	movq	%rbx, %r12
	adcq	$-1, %r12
.Ltmp7839:
	.loc	15 2986 25
	testq	%rbx, %rbx
	je	.LBB60_266
.Ltmp7840:
	.loc	15 2987 23
	cmpq	$1, %rbx
	je	.LBB60_269
	.loc	15 0 23 is_stmt 0
	movq	160(%rsp), %rcx
	movq	48(%rcx), %r15
	movl	(%r15), %edi
	.loc	15 2987 23
	movl	4(%r15), %esi
.Ltmp7841:
	.loc	14 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	64(%rsp), %rdx
	jb	.LBB60_248
	cmpq	%rsi, %rdx
	jb	.LBB60_248
	.loc	14 0 16 is_stmt 0
	movq	56(%rsp), %rax
	leaq	1848(%rax), %r9
	leaq	1648(%rax), %r8
	movq	96(%rcx), %r13
	leaq	2048(%rax), %rdx
	.loc	14 1054 31 is_stmt 1
	subq	%rdi, %rsi
.Ltmp7842:
	.loc	37 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	8(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
	leaq	3552(%rsp), %rax
	movq	%rdx, %r14
.Ltmp7843:
	.loc	15 2988 13
	movq	%r13, %rcx
	movq	%r8, 16(%rsp)
	movq	%r9, 32(%rsp)
	pushq	%rax
	pushq	$0
	vzeroupper
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp7844:
	.loc	15 2987 23
	cmpq	$1, %r12
	je	.LBB60_270
	.loc	15 0 23 is_stmt 0
	movl	4(%r15), %edi
	.loc	15 2987 23
	movl	8(%r15), %esi
.Ltmp7845:
	.loc	14 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	64(%rsp), %rdx
	jb	.LBB60_248
	cmpq	%rsi, %rdx
	jb	.LBB60_248
	.loc	14 1054 31
	subq	%rdi, %rsi
.Ltmp7846:
	.loc	37 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	8(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp7847:
	.loc	15 2995 17
	leaq	3592(%rsp), %rax
	.loc	15 2988 13
	movq	%r14, %rdx
	movq	%r13, %rcx
	movq	16(%rsp), %r8
	movq	32(%rsp), %r9
	pushq	%rax
	pushq	$1
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp7848:
	.loc	15 2986 25
	cmpq	$2, %rbx
	je	.LBB60_266
.Ltmp7849:
	.loc	15 2987 23
	cmpq	$2, %r12
	je	.LBB60_271
	.loc	15 0 23 is_stmt 0
	movl	8(%r15), %edi
	.loc	15 2987 23
	movl	12(%r15), %esi
.Ltmp7850:
	.loc	14 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	64(%rsp), %rdx
	jb	.LBB60_248
	cmpq	%rsi, %rdx
	jb	.LBB60_248
	.loc	14 1054 31
	subq	%rdi, %rsi
.Ltmp7851:
	.loc	37 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	8(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp7852:
	.loc	15 2995 17
	leaq	3632(%rsp), %rax
	.loc	15 2988 13
	movq	%r14, %rdx
	movq	%r13, %rcx
	movq	16(%rsp), %r8
	movq	32(%rsp), %r9
	pushq	%rax
	pushq	$2
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp7853:
	.loc	15 2986 25
	cmpq	$3, %rbx
	je	.LBB60_266
.Ltmp7854:
	.loc	15 2987 23
	cmpq	$3, %r12
	je	.LBB60_272
	.loc	15 0 23 is_stmt 0
	movl	12(%r15), %edi
	.loc	15 2987 23
	movl	16(%r15), %esi
.Ltmp7855:
	.loc	14 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	64(%rsp), %rdx
	jb	.LBB60_248
	cmpq	%rsi, %rdx
	jb	.LBB60_248
	.loc	14 1054 31
	subq	%rdi, %rsi
.Ltmp7856:
	.loc	37 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	8(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp7857:
	.loc	15 2995 17
	leaq	3672(%rsp), %rax
	.loc	15 2988 13
	movq	%r14, %rdx
	movq	%r13, %rcx
	movq	16(%rsp), %r8
	movq	32(%rsp), %r9
	pushq	%rax
	pushq	$3
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp7858:
	.loc	15 2986 25
	cmpq	$4, %rbx
	je	.LBB60_266
.Ltmp7859:
	.loc	15 2987 23
	cmpq	$4, %r12
	je	.LBB60_273
	.loc	15 0 23 is_stmt 0
	movl	16(%r15), %edi
	.loc	15 2987 23
	movl	20(%r15), %esi
.Ltmp7860:
	.loc	14 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	64(%rsp), %rdx
	jb	.LBB60_248
	cmpq	%rsi, %rdx
	jb	.LBB60_248
	.loc	14 1054 31
	subq	%rdi, %rsi
.Ltmp7861:
	.loc	37 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	8(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp7862:
	.loc	15 2995 17
	leaq	3712(%rsp), %rax
	.loc	15 2988 13
	movq	%r14, %rdx
	movq	%r13, %rcx
	movq	16(%rsp), %r8
	movq	32(%rsp), %r9
	pushq	%rax
	pushq	$4
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp7863:
	.loc	15 2986 25
	cmpq	$5, %rbx
	je	.LBB60_266
.Ltmp7864:
	.loc	15 2987 23
	cmpq	$5, %r12
	je	.LBB60_274
	.loc	15 0 23 is_stmt 0
	movl	20(%r15), %edi
	.loc	15 2987 23
	movl	24(%r15), %esi
.Ltmp7865:
	.loc	14 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	64(%rsp), %rdx
	jb	.LBB60_248
	cmpq	%rsi, %rdx
	jb	.LBB60_248
	.loc	14 1054 31
	subq	%rdi, %rsi
.Ltmp7866:
	.loc	37 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	8(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp7867:
	.loc	15 2995 17
	leaq	3752(%rsp), %rax
	.loc	15 2988 13
	movq	%r14, %rdx
	movq	%r13, %rcx
	movq	16(%rsp), %r8
	movq	32(%rsp), %r9
	pushq	%rax
	pushq	$5
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp7868:
	.loc	15 2986 25
	cmpq	$6, %rbx
	je	.LBB60_266
.Ltmp7869:
	.loc	15 2987 23
	cmpq	$6, %r12
	je	.LBB60_275
	.loc	15 0 23 is_stmt 0
	movl	24(%r15), %edi
	.loc	15 2987 23
	movl	28(%r15), %esi
.Ltmp7870:
	.loc	14 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	64(%rsp), %rdx
	jb	.LBB60_248
	cmpq	%rsi, %rdx
	jb	.LBB60_248
	.loc	14 1054 31
	subq	%rdi, %rsi
.Ltmp7871:
	.loc	37 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	8(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp7872:
	.loc	15 2995 17
	leaq	3792(%rsp), %rax
	.loc	15 2988 13
	movq	%r14, %rdx
	movq	%r13, %rcx
	movq	16(%rsp), %r8
	movq	32(%rsp), %r9
	pushq	%rax
	pushq	$6
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
.Ltmp7873:
	.loc	15 2986 25
	cmpq	$7, %rbx
	je	.LBB60_266
.Ltmp7874:
	.loc	15 2987 23
	cmpq	$7, %r12
	je	.LBB60_277
	.loc	15 0 23 is_stmt 0
	movl	28(%r15), %edi
	.loc	15 2987 23
	movl	32(%r15), %esi
.Ltmp7875:
	.loc	14 1050 16 is_stmt 1
	cmpl	%edi, %esi
	movq	64(%rsp), %rdx
	jb	.LBB60_248
	cmpq	%rsi, %rdx
	jb	.LBB60_248
	.loc	14 1054 31
	subq	%rdi, %rsi
.Ltmp7876:
	.loc	37 89 24
	leaq	(%rdi,%rdi,4), %rax
	movq	8(%rsp), %rcx
	leaq	(%rcx,%rax,8), %rdi
.Ltmp7877:
	.loc	15 2995 17
	leaq	3832(%rsp), %rax
	.loc	15 2988 13
	movq	%r14, %rdx
	movq	%r13, %rcx
	movq	16(%rsp), %r8
	movq	32(%rsp), %r9
	pushq	%rax
	pushq	$7
	callq	*_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation@GOTPCREL(%rip)
	addq	$16, %rsp
	movq	160(%rsp), %rax
.Ltmp7878:
	.loc	15 3000 37
	movq	(%rax), %rbx
	movq	8(%rax), %r13
	.loc	15 3000 49 is_stmt 0
	movl	104(%rax), %eax
	movq	%rax, 24(%rsp)
.Ltmp7879:
	.loc	15 2171 21 is_stmt 1
	leaq	(,%rax,8), %rsi
	movq	56(%rsp), %r12
.Ltmp7880:
	.loc	15 2172 21
	movzbl	2153(%r12), %eax
	cmpb	2144(%r12), %al
	movq	%r13, 96(%rsp)
	movq	%rbx, 40(%rsp)
	movq	%rsi, 752(%rsp)
	jne	.LBB60_42
	.loc	15 2173 37
	movq	1776(%r12), %rcx
	movq	1784(%r12), %rax
.Ltmp7881:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp7882:
	.p2align	4
.LBB60_35:
.Ltmp7883:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp7884:
	.loc	17 180 28
	je	.LBB60_38
.Ltmp7885:
	.loc	15 687 21
	cmpl	$0, 12(%rcx)
.Ltmp7886:
	.loc	17 315 25
	jne	.LBB60_42
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB60_35
	jmp	.LBB60_42
.Ltmp7887:
.LBB60_38:
	.loc	15 2174 37 is_stmt 1
	movq	1792(%r12), %rcx
	movq	1800(%r12), %rax
.Ltmp7888:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp7889:
	.p2align	4
.LBB60_39:
.Ltmp7890:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp7891:
	.loc	17 180 28
	je	.LBB60_144
.Ltmp7892:
	.loc	15 687 21
	cmpl	$0, 12(%rcx)
.Ltmp7893:
	.loc	17 315 25
	jne	.LBB60_42
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB60_39
	jmp	.LBB60_42
.Ltmp7894:
.LBB60_144:
	.loc	17 0 25
	cmpq	%r13, %rsi
.Ltmp7895:
	.loc	14 1050 16 is_stmt 1
	ja	.LBB60_261
.Ltmp7896:
	.loc	14 0 16 is_stmt 0
	movq	%rsi, %rax
	movq	%rbx, %rcx
	.p2align	4
.LBB60_146:
.Ltmp7897:
	.loc	29 1504 12 is_stmt 1
	testq	%rax, %rax
	je	.LBB60_220
.Ltmp7898:
	.loc	10 1078 5
	cmpq	$32, %rax
	movl	$32, %edx
	cmovbq	%rax, %rdx
.Ltmp7899:
	.loc	12 961 18
	leal	(,%rdx,4), %edi
	xorl	%r9d, %r9d
	xorl	%r8d, %r8d
.Ltmp7900:
	.loc	12 0 18 is_stmt 0
.Ltmp7901:
	.p2align	4
.LBB60_148:
	.loc	34 134 13 is_stmt 1
	orl	(%rcx,%r8), %r9d
.Ltmp7902:
	.loc	16 1714 9
	addq	$4, %r8
	cmpq	%r8, %rdi
.Ltmp7903:
	.loc	17 180 28
	jne	.LBB60_148
.Ltmp7904:
	.loc	35 863 18
	leaq	(%rcx,%rdx,4), %rcx
.Ltmp7905:
	.loc	30 2054 74
	subq	%rdx, %rax
.Ltmp7906:
	.loc	34 136 12
	testl	%r9d, %r9d
	je	.LBB60_146
.Ltmp7907:
.LBB60_42:
	.loc	34 0 12 is_stmt 0
	xorl	%eax, %eax
.LBB60_43:
	movl	%eax, 124(%rsp)
.Ltmp7908:
	.loc	15 1011 5 is_stmt 1
	movq	1832(%r12), %rcx
	testq	%rcx, %rcx
	je	.LBB60_49
	.loc	15 0 5 is_stmt 0
	movq	1824(%r12), %rax
	shlq	$2, %rcx
	leaq	(%rcx,%rcx,2), %rcx
	movq	%rax, %rdx
	.p2align	4
.LBB60_45:
.Ltmp7909:
	.loc	16 1714 9 is_stmt 1
	testq	%rcx, %rcx
.Ltmp7910:
	.loc	17 180 28
	je	.LBB60_49
.Ltmp7911:
	.loc	15 387 34
	movl	(%rdx), %edi
	cmpl	(%rax), %edi
	jne	.LBB60_53
	movl	4(%rdx), %edi
	cmpl	4(%rax), %edi
	jne	.LBB60_53
	movl	8(%rdx), %edi
.Ltmp7912:
	.loc	17 0 0 is_stmt 0
	addq	$12, %rdx
	.loc	17 315 25 is_stmt 1
	addq	$-12, %rcx
.Ltmp7913:
	.loc	15 387 34
	cmpl	8(%rax), %edi
.Ltmp7914:
	.loc	17 315 25
	je	.LBB60_45
	jmp	.LBB60_53
.Ltmp7915:
.LBB60_49:
	.loc	15 1012 12
	movq	1768(%r12), %rax
	testq	%rax, %rax
	je	.LBB60_57
	.loc	15 0 12 is_stmt 0
	movq	1760(%r12), %rcx
	shlq	$2, %rax
	xorl	%edx, %edx
	.p2align	4
.LBB60_51:
.Ltmp7916:
	.loc	16 1714 9 is_stmt 1
	cmpq	%rdx, %rax
.Ltmp7917:
	.loc	17 180 28
	je	.LBB60_57
.Ltmp7918:
	.loc	17 315 25
	movl	(%rcx,%rdx), %edi
	addq	$4, %rdx
.Ltmp7919:
	.loc	15 1012 43
	cmpl	(%rcx), %edi
.Ltmp7920:
	.loc	17 315 25
	je	.LBB60_51
.Ltmp7921:
.LBB60_53:
	.loc	17 0 25 is_stmt 0
	leaq	960(%rsp), %rdi
	movq	16(%rsp), %rsi
.Ltmp7922:
	.loc	15 3062 24 is_stmt 1
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
.Ltmp7923:
	.loc	15 3063 43
	movq	1776(%r12), %rcx
	movq	1784(%r12), %rax
.Ltmp7924:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp7925:
	.p2align	4
.LBB60_54:
.Ltmp7926:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp7927:
	.loc	17 180 28
	je	.LBB60_61
.Ltmp7928:
	.loc	17 0 28 is_stmt 0
	movl	$0, 48(%rsp)
.Ltmp7929:
	.loc	15 687 21 is_stmt 1
	cmpl	$0, 12(%rcx)
	jne	.LBB60_70
.Ltmp7930:
	.loc	17 315 25
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB60_54
	jmp	.LBB60_70
.Ltmp7931:
.LBB60_57:
	.loc	17 0 25
	leaq	960(%rsp), %rdi
	movq	16(%rsp), %rsi
.Ltmp7932:
	.loc	15 3146 24 is_stmt 1
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_
.Ltmp7933:
	.loc	15 3147 43
	movq	1776(%r12), %rcx
	movq	1784(%r12), %rax
.Ltmp7934:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp7935:
	.p2align	4
.LBB60_58:
.Ltmp7936:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp7937:
	.loc	17 180 28
	je	.LBB60_65
.Ltmp7938:
	.loc	17 0 28 is_stmt 0
	movl	$0, 8(%rsp)
.Ltmp7939:
	.loc	15 687 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp7940:
	.loc	17 315 25
	jne	.LBB60_154
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB60_58
	jmp	.LBB60_154
.Ltmp7941:
.LBB60_61:
	.loc	15 3063 80 is_stmt 1
	movq	1792(%r12), %rcx
	movq	1800(%r12), %rax
.Ltmp7942:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp7943:
	.p2align	4
.LBB60_62:
.Ltmp7944:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp7945:
	.loc	17 180 28
	je	.LBB60_69
.Ltmp7946:
	.loc	17 0 28 is_stmt 0
	movl	$0, 48(%rsp)
.Ltmp7947:
	.loc	15 687 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp7948:
	.loc	17 315 25
	jne	.LBB60_70
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB60_62
	jmp	.LBB60_70
.Ltmp7949:
.LBB60_65:
	.loc	15 3147 80 is_stmt 1
	movq	1792(%r12), %rcx
	movq	1800(%r12), %rax
.Ltmp7950:
	.loc	17 314 17
	shlq	$4, %rax
	.loc	17 0 17 is_stmt 0
.Ltmp7951:
	.p2align	4
.LBB60_66:
.Ltmp7952:
	.loc	16 1714 9 is_stmt 1
	testq	%rax, %rax
.Ltmp7953:
	.loc	17 180 28
	je	.LBB60_153
.Ltmp7954:
	.loc	17 0 28 is_stmt 0
	movl	$0, 8(%rsp)
.Ltmp7955:
	.loc	15 687 21 is_stmt 1
	cmpl	$0, 12(%rcx)
.Ltmp7956:
	.loc	17 315 25
	jne	.LBB60_154
	addq	$-16, %rax
	movl	4(%rcx), %edx
	cmpl	%edx, (%rcx)
	.loc	17 0 0 is_stmt 0
	leaq	16(%rcx), %rcx
	.loc	17 315 25
	je	.LBB60_66
	jmp	.LBB60_154
.Ltmp7957:
.LBB60_69:
	.loc	17 0 25
	movb	$1, %al
	movl	%eax, 48(%rsp)
.LBB60_70:
.Ltmp7958:
	.loc	15 3066 19 is_stmt 1
	movzbl	1536(%r12), %r14d
.Ltmp7959:
	.loc	15 3067 21
	movzbl	1537(%r12), %ebx
.Ltmp7960:
	.loc	15 3068 27
	movl	1640(%r12), %r15d
.Ltmp7961:
	.loc	15 3069 27
	movl	1644(%r12), %eax
	movq	%rax, 8(%rsp)
	vxorps	%xmm0, %xmm0, %xmm0
	vmovaps	%ymm0, 832(%rsp)
	leaq	3904(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	vzeroupper
	callq	*memset@GOTPCREL(%rip)
	movq	24(%rsp), %rax
.Ltmp7962:
	.loc	14 3758 16
	leaq	31(%rax), %rcx
	shrq	$5, %rcx
.Ltmp7963:
	.loc	40 446 20
	je	.LBB60_142
.Ltmp7964:
	.loc	50 871 14
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm1
.Ltmp7965:
	.loc	50 713 19
	vpcmpeqd	%ymm0, %ymm0, %ymm0
	vxorps	%ymm0, %ymm1, %ymm0
	vmovaps	%ymm1, 1952(%rsp)
	vmovaps	%ymm1, 1984(%rsp)
	testb	%r14b, %r14b
	jne	.LBB60_73
.Ltmp7966:
	.loc	50 0 19 is_stmt 0
	vmovaps	%ymm0, 1984(%rsp)
.LBB60_73:
	testb	%bl, %bl
	movq	40(%rsp), %r14
	jne	.LBB60_75
	vmovaps	%ymm0, 1952(%rsp)
.LBB60_75:
.Ltmp7967:
	.loc	15 1612 23 is_stmt 1
	vmovaps	960(%rsp), %ymm4
	vmovaps	992(%rsp), %ymm8
	vmovaps	1024(%rsp), %ymm0
	vmovaps	%ymm0, 256(%rsp)
	vmovaps	1056(%rsp), %ymm3
	vmovaps	1088(%rsp), %ymm0
	vmovaps	%ymm0, 1728(%rsp)
	vmovaps	1120(%rsp), %ymm6
	vmovaps	1152(%rsp), %ymm0
	vmovaps	%ymm0, 1696(%rsp)
	vmovaps	1184(%rsp), %ymm2
	vmovaps	1216(%rsp), %ymm1
	vmovaps	1248(%rsp), %ymm7
	vmovaps	1280(%rsp), %ymm9
	vmovaps	1312(%rsp), %ymm0
	vmovaps	%ymm0, 1824(%rsp)
	movq	$0, 288(%rsp)
	movq	%r14, 2048(%rsp)
	movq	%r13, 1920(%rsp)
	movq	$0, 104(%rsp)
	movq	%rax, %r11
	xorl	%edx, %edx
	movq	%r15, %rbx
.Ltmp7968:
	.loc	40 446 20
	jmp	.LBB60_78
.Ltmp7969:
	.loc	40 0 20 is_stmt 0
.Ltmp7970:
	.p2align	4
.LBB60_76:
	.loc	15 749 9 is_stmt 1
	vmovaps	%ymm11, 1536(%rsp)
.Ltmp7971:
	.loc	15 1306 5
	vmovaps	%ymm10, 1632(%rsp)
.Ltmp7972:
	.loc	15 746 9
	vmovaps	%ymm3, 1568(%rsp)
.Ltmp7973:
	.loc	15 748 9
	vmovaps	%ymm9, 1472(%rsp)
.Ltmp7974:
	.loc	15 748 9 is_stmt 0
	vmovaps	%ymm0, 1344(%rsp)
	.loc	15 749 9 is_stmt 1
	vmovaps	%ymm12, 1408(%rsp)
.Ltmp7975:
	.loc	15 746 0
	vmovaps	%ymm15, 1440(%rsp)
	movq	24(%rsp), %rax
	vmovaps	512(%rsp), %ymm4
	vmovaps	544(%rsp), %ymm2
	vmovaps	576(%rsp), %ymm8
	vmovaps	384(%rsp), %ymm9
	vmovaps	416(%rsp), %ymm3
	vmovaps	480(%rsp), %ymm6
.Ltmp7976:
.LBB60_77:
	.loc	15 0 0 is_stmt 0
	addq	$32, %rdx
	movq	2144(%rsp), %rcx
	decq	%rcx
.Ltmp7977:
	.loc	40 446 20 is_stmt 1
	addq	$-32, %r11
	addq	$256, 104(%rsp)
	addq	$-256, 1920(%rsp)
	addq	$1024, 2048(%rsp)
	testq	%rcx, %rcx
	je	.LBB60_143
.LBB60_78:
	.loc	40 0 20 is_stmt 0
	vmovaps	%ymm6, %ymm5
	movq	%rcx, 2144(%rsp)
.Ltmp7978:
	.loc	14 2584 13 is_stmt 1
	cmpq	$1, %r11
	movq	%r11, %rsi
	adcq	$0, %rsi
	cmpq	$32, %rsi
	movl	$32, %ecx
	cmovaeq	%rcx, %rsi
	movq	%rdx, 2016(%rsp)
.Ltmp7979:
	.loc	10 1916 50
	cmpq	%rax, %rdx
	movq	%rsi, 2208(%rsp)
.Ltmp7980:
	.loc	11 900 12
	jne	.LBB60_80
	.loc	11 0 12 is_stmt 0
	vmovaps	256(%rsp), %ymm12
	vmovaps	1728(%rsp), %ymm11
	vmovaps	%ymm8, %ymm15
	vmovaps	%ymm9, %ymm13
	vmovaps	1696(%rsp), %ymm6
	.loc	11 900 12
	jmp	.LBB60_84
.Ltmp7981:
	.loc	11 0 12
.Ltmp7982:
	.p2align	4
.LBB60_80:
	leal	(,%rsi,8), %eax
	vmovaps	(%r12), %ymm0
	vmovaps	%ymm0, 448(%rsp)
	vmovaps	32(%r12), %ymm0
	vmovaps	%ymm0, 672(%rsp)
	vmovaps	64(%r12), %ymm0
	vmovaps	%ymm0, 352(%rsp)
	vmovaps	96(%r12), %ymm0
	vmovaps	%ymm0, 128(%rsp)
	vmovaps	128(%r12), %ymm0
	vmovaps	%ymm0, 640(%rsp)
	vmovaps	160(%r12), %ymm0
	vmovaps	%ymm0, 608(%rsp)
	vmovaps	192(%r12), %ymm0
	vmovaps	%ymm0, 1856(%rsp)
	vmovaps	224(%r12), %ymm0
	vmovaps	%ymm0, 1792(%rsp)
	vmovaps	256(%r12), %ymm0
	vmovaps	%ymm0, 800(%rsp)
	vmovaps	288(%r12), %ymm0
	vmovaps	%ymm0, 768(%rsp)
	vmovaps	320(%r12), %ymm0
	vmovaps	%ymm0, 576(%rsp)
	movq	1920(%rsp), %rdx
	xorl	%ecx, %ecx
	vmovaps	%ymm8, %ymm0
	vmovaps	%ymm7, %ymm10
	vmovaps	%ymm1, %ymm8
	vmovaps	%ymm2, %ymm14
	vmovaps	352(%r12), %ymm1
	vmovaps	%ymm1, 384(%rsp)
	vmovaps	384(%r12), %ymm1
	vmovaps	%ymm1, 544(%rsp)
	vmovaps	416(%r12), %ymm1
	vmovaps	%ymm1, 512(%rsp)
	vmovaps	448(%r12), %ymm1
	vmovaps	%ymm1, 480(%rsp)
	vmovaps	480(%r12), %ymm1
	vmovaps	%ymm1, 1760(%rsp)
	vmovaps	512(%r12), %ymm1
	vmovaps	%ymm1, 3232(%rsp)
	vmovaps	544(%r12), %ymm1
	vmovaps	%ymm1, 3200(%rsp)
	vmovaps	576(%r12), %ymm1
	vmovaps	%ymm1, 3168(%rsp)
	vmovaps	608(%r12), %ymm1
	vmovaps	%ymm1, 3136(%rsp)
	vmovaps	640(%r12), %ymm1
	vmovaps	%ymm1, 3104(%rsp)
	vmovaps	672(%r12), %ymm1
	vmovaps	%ymm1, 3072(%rsp)
	vmovaps	704(%r12), %ymm1
	vmovaps	%ymm1, 3040(%rsp)
	vmovaps	736(%r12), %ymm1
	vmovaps	%ymm1, 3008(%rsp)
	vmovaps	768(%r12), %ymm1
	vmovaps	%ymm1, 2976(%rsp)
	vmovaps	800(%r12), %ymm1
	vmovaps	%ymm1, 2944(%rsp)
	vmovaps	832(%r12), %ymm1
	vmovaps	%ymm1, 2912(%rsp)
	vmovaps	864(%r12), %ymm1
	vmovaps	%ymm1, 2880(%rsp)
	vmovaps	896(%r12), %ymm1
	vmovaps	%ymm1, 2848(%rsp)
	vmovaps	928(%r12), %ymm1
	vmovaps	%ymm1, 2816(%rsp)
	vmovaps	960(%r12), %ymm1
	vmovaps	%ymm1, 2784(%rsp)
	vmovaps	992(%r12), %ymm1
	vmovaps	%ymm1, 2752(%rsp)
	vmovaps	1024(%r12), %ymm1
	vmovaps	%ymm1, 2720(%rsp)
	vmovaps	1056(%r12), %ymm1
	vmovaps	%ymm1, 2688(%rsp)
	vmovaps	1088(%r12), %ymm1
	vmovaps	%ymm1, 2656(%rsp)
	vmovaps	1120(%r12), %ymm1
	vmovaps	%ymm1, 2624(%rsp)
	vmovaps	1152(%r12), %ymm1
	vmovaps	%ymm1, 2592(%rsp)
	vmovaps	1184(%r12), %ymm1
	vmovaps	%ymm1, 2560(%rsp)
	vmovaps	1216(%r12), %ymm1
	vmovaps	%ymm1, 2528(%rsp)
	vmovaps	1248(%r12), %ymm1
	vmovaps	%ymm1, 2496(%rsp)
	vmovaps	1280(%r12), %ymm1
	vmovaps	%ymm1, 2464(%rsp)
	vmovaps	1312(%r12), %ymm1
	vmovaps	%ymm1, 2432(%rsp)
	vmovaps	1344(%r12), %ymm1
	vmovaps	%ymm1, 2400(%rsp)
	vmovaps	1376(%r12), %ymm1
	vmovaps	%ymm1, 2368(%rsp)
	vmovaps	1408(%r12), %ymm1
	vmovaps	%ymm1, 2336(%rsp)
	vmovaps	1440(%r12), %ymm1
	vmovaps	%ymm1, 2304(%rsp)
	vmovaps	1472(%r12), %ymm1
	vmovaps	%ymm1, 2272(%rsp)
	vmovaps	1504(%r12), %ymm1
	vmovaps	%ymm1, 2240(%rsp)
	vmovaps	256(%rsp), %ymm12
	vmovaps	1728(%rsp), %ymm11
	vmovaps	1696(%rsp), %ymm6
	.p2align	4
.LBB60_81:
	vmovaps	%ymm14, 160(%rsp)
	vmovaps	%ymm8, 64(%rsp)
	movq	104(%rsp), %rsi
.Ltmp7983:
	.loc	37 568 12 is_stmt 1
	leaq	(%rsi,%rcx), %rdi
	cmpq	%r13, %rdi
	ja	.LBB60_252
.Ltmp7984:
	.loc	37 438 16
	cmpq	$7, %rdx
	jbe	.LBB60_152
.Ltmp7985:
	.loc	37 0 16 is_stmt 0
	vmovaps	%ymm4, %ymm15
	vmovaps	%ymm6, %ymm2
	vmovaps	%ymm5, %ymm6
	vmovaps	%ymm12, %ymm7
	vmovaps	%ymm0, %ymm1
	vmovaps	%ymm11, %ymm5
	vmovaps	%ymm3, %ymm11
	movq	2048(%rsp), %rsi
.Ltmp7986:
	.loc	1 551 14 is_stmt 1
	vmovups	(%rsi,%rcx,4), %ymm4
.Ltmp7987:
	.loc	50 82 19
	vbroadcastss	.LCPI60_0(%rip), %ymm12
.Ltmp7988:
	.loc	50 283 14
	vmulps	448(%rsp), %ymm4, %ymm8
	vxorps	%xmm13, %xmm13, %xmm13
.Ltmp7989:
	.loc	50 48 14
	vaddps	%ymm13, %ymm8, %ymm8
	vmovaps	%ymm9, %ymm0
.Ltmp7990:
	.loc	50 283 14
	vmulps	672(%rsp), %ymm4, %ymm9
.Ltmp7991:
	.loc	50 48 14
	vaddps	%ymm13, %ymm9, %ymm9
	vmovaps	%ymm10, %ymm3
.Ltmp7992:
	.loc	50 283 14
	vmulps	352(%rsp), %ymm4, %ymm10
.Ltmp7993:
	.loc	50 48 14
	vaddps	%ymm13, %ymm10, %ymm10
.Ltmp7994:
	.loc	50 283 14
	vmulps	128(%rsp), %ymm4, %ymm14
.Ltmp7995:
	.loc	50 48 14
	vaddps	%ymm13, %ymm14, %ymm14
.Ltmp7996:
	.loc	50 283 14
	vmulps	640(%rsp), %ymm15, %ymm13
.Ltmp7997:
	.loc	50 48 14
	vaddps	%ymm8, %ymm13, %ymm8
.Ltmp7998:
	.loc	50 283 14
	vmulps	608(%rsp), %ymm15, %ymm13
.Ltmp7999:
	.loc	50 48 14
	vaddps	%ymm9, %ymm13, %ymm9
.Ltmp8000:
	.loc	50 283 14
	vmulps	1856(%rsp), %ymm15, %ymm13
.Ltmp8001:
	.loc	50 48 14
	vaddps	%ymm10, %ymm13, %ymm10
.Ltmp8002:
	.loc	50 283 14
	vmulps	1792(%rsp), %ymm15, %ymm13
.Ltmp8003:
	.loc	50 48 14
	vaddps	%ymm14, %ymm13, %ymm13
	vmovaps	%ymm1, 256(%rsp)
.Ltmp8004:
	.loc	50 283 14
	vmulps	800(%rsp), %ymm1, %ymm14
.Ltmp8005:
	.loc	50 48 14
	vaddps	%ymm8, %ymm14, %ymm8
.Ltmp8006:
	.loc	50 283 14
	vmulps	768(%rsp), %ymm1, %ymm14
.Ltmp8007:
	.loc	50 48 14
	vaddps	%ymm9, %ymm14, %ymm9
.Ltmp8008:
	.loc	50 283 14
	vmulps	576(%rsp), %ymm1, %ymm14
.Ltmp8009:
	.loc	50 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp8010:
	.loc	50 283 14
	vmulps	384(%rsp), %ymm1, %ymm14
.Ltmp8011:
	.loc	50 48 14
	vaddps	%ymm13, %ymm14, %ymm13
	vmovaps	%ymm7, 416(%rsp)
.Ltmp8012:
	.loc	50 283 14
	vmulps	544(%rsp), %ymm7, %ymm14
.Ltmp8013:
	.loc	50 48 14
	vaddps	%ymm8, %ymm14, %ymm8
.Ltmp8014:
	.loc	50 283 14
	vmulps	512(%rsp), %ymm7, %ymm14
.Ltmp8015:
	.loc	50 48 14
	vaddps	%ymm9, %ymm14, %ymm9
.Ltmp8016:
	.loc	50 283 14
	vmulps	480(%rsp), %ymm7, %ymm14
.Ltmp8017:
	.loc	50 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp8018:
	.loc	50 283 14
	vmulps	1760(%rsp), %ymm7, %ymm14
.Ltmp8019:
	.loc	50 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp8020:
	.loc	50 283 14
	vmulps	3232(%rsp), %ymm11, %ymm14
.Ltmp8021:
	.loc	50 48 14
	vaddps	%ymm8, %ymm14, %ymm8
.Ltmp8022:
	.loc	50 283 14
	vmulps	3200(%rsp), %ymm11, %ymm14
.Ltmp8023:
	.loc	50 48 14
	vaddps	%ymm9, %ymm14, %ymm9
.Ltmp8024:
	.loc	50 283 14
	vmulps	3168(%rsp), %ymm11, %ymm14
.Ltmp8025:
	.loc	50 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp8026:
	.loc	50 283 14
	vmulps	3136(%rsp), %ymm11, %ymm14
.Ltmp8027:
	.loc	50 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp8028:
	.loc	50 283 14
	vmulps	3104(%rsp), %ymm5, %ymm14
.Ltmp8029:
	.loc	50 48 14
	vaddps	%ymm8, %ymm14, %ymm8
.Ltmp8030:
	.loc	50 283 14
	vmulps	3072(%rsp), %ymm5, %ymm14
.Ltmp8031:
	.loc	50 48 14
	vaddps	%ymm9, %ymm14, %ymm9
.Ltmp8032:
	.loc	50 283 14
	vmulps	3040(%rsp), %ymm5, %ymm14
.Ltmp8033:
	.loc	50 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp8034:
	.loc	50 283 14
	vmulps	3008(%rsp), %ymm5, %ymm14
.Ltmp8035:
	.loc	50 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp8036:
	.loc	50 283 14
	vmulps	2976(%rsp), %ymm6, %ymm14
.Ltmp8037:
	.loc	50 48 14
	vaddps	%ymm8, %ymm14, %ymm8
.Ltmp8038:
	.loc	50 283 14
	vmulps	2944(%rsp), %ymm6, %ymm14
.Ltmp8039:
	.loc	50 48 14
	vaddps	%ymm9, %ymm14, %ymm9
.Ltmp8040:
	.loc	50 283 14
	vmulps	2912(%rsp), %ymm6, %ymm14
.Ltmp8041:
	.loc	50 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp8042:
	.loc	50 283 14
	vmulps	2880(%rsp), %ymm6, %ymm14
.Ltmp8043:
	.loc	50 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp8044:
	.loc	50 283 14
	vmulps	2848(%rsp), %ymm2, %ymm14
.Ltmp8045:
	.loc	50 48 14
	vaddps	%ymm8, %ymm14, %ymm8
.Ltmp8046:
	.loc	50 283 14
	vmulps	2816(%rsp), %ymm2, %ymm14
.Ltmp8047:
	.loc	50 48 14
	vaddps	%ymm9, %ymm14, %ymm9
.Ltmp8048:
	.loc	50 283 14
	vmulps	2784(%rsp), %ymm2, %ymm14
.Ltmp8049:
	.loc	50 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp8050:
	.loc	50 283 14
	vmulps	2752(%rsp), %ymm2, %ymm14
.Ltmp8051:
	.loc	50 48 14
	vaddps	%ymm13, %ymm14, %ymm13
	vmovaps	160(%rsp), %ymm1
.Ltmp8052:
	.loc	50 283 14
	vmulps	2720(%rsp), %ymm1, %ymm14
.Ltmp8053:
	.loc	50 48 14
	vaddps	%ymm8, %ymm14, %ymm8
.Ltmp8054:
	.loc	50 283 14
	vmulps	2688(%rsp), %ymm1, %ymm14
.Ltmp8055:
	.loc	50 48 14
	vaddps	%ymm9, %ymm14, %ymm9
.Ltmp8056:
	.loc	50 283 14
	vmulps	2656(%rsp), %ymm1, %ymm14
.Ltmp8057:
	.loc	50 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp8058:
	.loc	50 283 14
	vmulps	2624(%rsp), %ymm1, %ymm14
.Ltmp8059:
	.loc	50 48 14
	vaddps	%ymm13, %ymm14, %ymm13
	vmovaps	64(%rsp), %ymm7
.Ltmp8060:
	.loc	50 283 14
	vmulps	2592(%rsp), %ymm7, %ymm14
.Ltmp8061:
	.loc	50 48 14
	vaddps	%ymm8, %ymm14, %ymm8
.Ltmp8062:
	.loc	50 283 14
	vmulps	2560(%rsp), %ymm7, %ymm14
.Ltmp8063:
	.loc	50 48 14
	vaddps	%ymm9, %ymm14, %ymm9
.Ltmp8064:
	.loc	50 283 14
	vmulps	2528(%rsp), %ymm7, %ymm14
.Ltmp8065:
	.loc	50 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp8066:
	.loc	50 283 14
	vmulps	2496(%rsp), %ymm7, %ymm14
.Ltmp8067:
	.loc	50 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp8068:
	.loc	50 283 14
	vmulps	2464(%rsp), %ymm3, %ymm14
.Ltmp8069:
	.loc	50 48 14
	vaddps	%ymm8, %ymm14, %ymm8
.Ltmp8070:
	.loc	50 283 14
	vmulps	2432(%rsp), %ymm3, %ymm14
.Ltmp8071:
	.loc	50 48 14
	vaddps	%ymm9, %ymm14, %ymm9
.Ltmp8072:
	.loc	50 283 14
	vmulps	2400(%rsp), %ymm3, %ymm14
.Ltmp8073:
	.loc	50 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp8074:
	.loc	50 283 14
	vmulps	2368(%rsp), %ymm3, %ymm14
.Ltmp8075:
	.loc	50 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp8076:
	.loc	50 283 14
	vmulps	2336(%rsp), %ymm0, %ymm14
.Ltmp8077:
	.loc	50 48 14
	vaddps	%ymm8, %ymm14, %ymm8
.Ltmp8078:
	.loc	50 283 14
	vmulps	2304(%rsp), %ymm0, %ymm14
.Ltmp8079:
	.loc	50 48 14
	vaddps	%ymm9, %ymm14, %ymm9
.Ltmp8080:
	.loc	50 283 14
	vmulps	2272(%rsp), %ymm0, %ymm14
.Ltmp8081:
	.loc	50 48 14
	vaddps	%ymm10, %ymm14, %ymm10
	vmovaps	%ymm0, 1824(%rsp)
.Ltmp8082:
	.loc	50 283 14
	vmulps	2240(%rsp), %ymm0, %ymm14
.Ltmp8083:
	.loc	50 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp8084:
	.loc	50 82 19
	vandps	%ymm6, %ymm12, %ymm14
.Ltmp8085:
	.loc	50 82 19 is_stmt 0
	vandps	%ymm12, %ymm8, %ymm8
.Ltmp8086:
	.loc	50 233 14 is_stmt 1
	vmaxps	%ymm8, %ymm14, %ymm8
.Ltmp8087:
	.loc	50 82 19
	vandps	%ymm12, %ymm9, %ymm9
.Ltmp8088:
	.loc	50 233 14
	vmaxps	%ymm9, %ymm8, %ymm8
.Ltmp8089:
	.loc	50 82 19
	vandps	%ymm12, %ymm10, %ymm9
.Ltmp8090:
	.loc	50 233 14
	vmaxps	%ymm9, %ymm8, %ymm8
.Ltmp8091:
	.loc	50 82 19
	vandps	%ymm12, %ymm13, %ymm0
	vmovaps	%ymm3, %ymm13
	vmovaps	256(%rsp), %ymm12
	vmovaps	416(%rsp), %ymm3
.Ltmp8092:
	.loc	50 233 14
	vmaxps	%ymm0, %ymm8, %ymm0
.Ltmp8093:
	.loc	1 551 14
	vmovups	%ymm0, 3904(%rsp,%rcx,4)
.Ltmp8094:
	.loc	10 1916 50
	addq	$8, %rcx
	addq	$-8, %rdx
	vmovaps	%ymm15, %ymm0
	vmovaps	%ymm13, %ymm9
	vmovaps	%ymm7, %ymm10
	vmovaps	%ymm1, %ymm8
	vmovaps	%ymm2, %ymm14
	cmpq	%rcx, %rax
.Ltmp8095:
	.loc	11 900 12
	jne	.LBB60_81
.Ltmp8096:
.LBB60_84:
	.loc	15 1618 5
	vmovaps	%ymm4, 960(%rsp)
	vmovaps	%ymm15, 992(%rsp)
	vmovaps	%ymm12, 256(%rsp)
	vmovaps	%ymm12, 1024(%rsp)
	vmovaps	%ymm3, 1056(%rsp)
	vmovaps	%ymm11, 1728(%rsp)
	vmovaps	%ymm11, 1088(%rsp)
	vmovaps	%ymm5, 1120(%rsp)
	vmovaps	%ymm6, 1696(%rsp)
	vmovaps	%ymm6, 1152(%rsp)
	vmovaps	%ymm2, 1184(%rsp)
	vmovaps	%ymm1, 1216(%rsp)
	vmovaps	%ymm7, 1248(%rsp)
	vmovaps	%ymm13, 1280(%rsp)
	vmovaps	1824(%rsp), %ymm0
	vmovaps	%ymm0, 1312(%rsp)
	movq	24(%rsp), %rax
	movq	2016(%rsp), %rdx
.Ltmp8097:
	.loc	10 1916 50
	cmpq	%rax, %rdx
	vmovaps	%ymm5, %ymm6
	vmovaps	%ymm15, %ymm8
	vmovaps	%ymm13, %ymm9
.Ltmp8098:
	.loc	11 900 12
	je	.LBB60_77
.Ltmp8099:
	.loc	11 0 12 is_stmt 0
	vmovaps	%ymm6, 480(%rsp)
	vmovaps	%ymm3, 416(%rsp)
	vmovaps	%ymm9, 384(%rsp)
	vmovaps	%ymm8, 576(%rsp)
	vmovaps	%ymm2, 544(%rsp)
	vmovaps	%ymm4, 512(%rsp)
	vmovaps	1344(%rsp), %ymm0
	vmovaps	1376(%rsp), %ymm2
	vmovaps	%ymm2, 800(%rsp)
	vmovaps	1408(%rsp), %ymm12
	vmovaps	1440(%rsp), %ymm15
	vmovaps	1504(%rsp), %ymm2
	vmovaps	%ymm2, 768(%rsp)
	movq	1624(%r12), %rcx
	movq	1632(%r12), %rax
	movq	%rax, 1792(%rsp)
	vmovaps	1664(%rsp), %ymm4
	vmovaps	1568(%rsp), %ymm3
	vmovaps	1472(%rsp), %ymm9
	vmovaps	1536(%rsp), %ymm11
	vmovaps	1632(%rsp), %ymm10
	xorl	%r8d, %r8d
	vbroadcastss	.LCPI60_2(%rip), %ymm5
	vbroadcastss	.LCPI60_3(%rip), %ymm6
	movq	%r11, 2176(%rsp)
	.p2align	4
.LBB60_86:
.Ltmp8100:
	.loc	15 3086 49 is_stmt 1
	cmpb	$0, 48(%rsp)
	jne	.LBB60_88
	.loc	15 0 49 is_stmt 0
	vbroadcastss	.LCPI60_1(%rip), %ymm13
.Ltmp8101:
	.loc	50 347 14 is_stmt 1
	vaddps	%ymm13, %ymm15, %ymm8
	vxorps	%xmm14, %xmm14, %xmm14
.Ltmp8102:
	.loc	50 233 14
	vmaxps	%ymm14, %ymm8, %ymm8
	vmovaps	%ymm8, %ymm15
.Ltmp8103:
	.loc	50 871 14
	vcmpgt_oqps	%ymm14, %ymm8, %ymm8
.Ltmp8104:
	.loc	50 48 14
	vaddps	%ymm0, %ymm12, %ymm0
	vmovaps	800(%rsp), %ymm2
.Ltmp8105:
	.loc	50 585 19
	vblendvps	%ymm8, %ymm0, %ymm2, %ymm0
.Ltmp8106:
	.loc	50 585 19 is_stmt 0
	vblendvps	%ymm8, %ymm12, %ymm14, %ymm12
.Ltmp8107:
	.loc	50 347 14 is_stmt 1
	vaddps	%ymm3, %ymm13, %ymm8
.Ltmp8108:
	.loc	50 233 14
	vmaxps	%ymm14, %ymm8, %ymm2
	vmovaps	%ymm2, %ymm3
.Ltmp8109:
	.loc	50 871 14
	vcmpgt_oqps	%ymm14, %ymm2, %ymm8
.Ltmp8110:
	.loc	50 48 14
	vaddps	%ymm9, %ymm11, %ymm9
	vmovaps	768(%rsp), %ymm2
.Ltmp8111:
	.loc	50 585 19
	vblendvps	%ymm8, %ymm9, %ymm2, %ymm9
.Ltmp8112:
	.loc	50 585 19 is_stmt 0
	vblendvps	%ymm8, %ymm11, %ymm14, %ymm11
.Ltmp8113:
.LBB60_88:
	.loc	15 0 0
	leaq	(%r8,%rdx), %rdi
	shlq	$3, %rdi
.Ltmp8114:
	.loc	37 568 12 is_stmt 1
	movq	%r13, %rdx
	subq	%rdi, %rdx
	jb	.LBB60_233
.Ltmp8115:
	.loc	37 438 16
	cmpq	$7, %rdx
	jbe	.LBB60_151
.Ltmp8116:
	.loc	15 1287 25
	movq	1688(%r12), %rsi
.Ltmp8117:
	.loc	15 1283 17
	movq	1840(%r12), %r9
	movq	%r9, 128(%rsp)
.Ltmp8118:
	.loc	15 1287 45
	imulq	8(%rsp), %r9
.Ltmp8119:
	.loc	37 580 12
	movq	%rsi, %rdx
	subq	%r9, %rdx
	jb	.LBB60_234
.Ltmp8120:
	.loc	37 451 16
	cmpq	$7, %rdx
	jbe	.LBB60_151
.Ltmp8121:
	.loc	37 0 16 is_stmt 0
	movq	%rdi, 1856(%rsp)
	movq	%rbx, 640(%rsp)
	movq	%r8, 608(%rsp)
	movq	%r8, %rax
	shlq	$5, %rax
	vmovups	3904(%rsp,%rax), %ymm8
.Ltmp8122:
	vmaxps	%ymm8, %ymm8, %ymm13
	vmovaps	1984(%rsp), %ymm14
.Ltmp8123:
	vblendvps	%ymm14, %ymm13, %ymm8, %ymm8
.Ltmp8124:
	vdivps	%ymm8, %ymm0, %ymm13
	vcmpgt_oqps	%ymm0, %ymm8, %ymm8
	vblendvps	%ymm8, %ymm13, %ymm5, %ymm8
.Ltmp8125:
	.loc	15 1287 25 is_stmt 1
	movq	1680(%r12), %rax
	movq	%r9, 352(%rsp)
.Ltmp8126:
	.loc	1 551 14
	vmovups	%ymm8, (%rax,%r9,4)
.Ltmp8127:
	.loc	15 1154 17
	movq	1840(%r12), %rdx
.Ltmp8128:
	.loc	43 37 12
	testq	%rdx, %rdx
	je	.LBB60_111
.Ltmp8129:
	.loc	43 0 12 is_stmt 0
	movq	56(%rsp), %rdi
	movq	1832(%rdi), %rax
	movq	%rax, 160(%rsp)
	movq	8(%rsp), %rax
	leaq	1(%rax), %r9
	cmpq	%rcx, %r9
	movq	%rcx, %rax
	movl	$0, %esi
	cmovbq	%rsi, %rax
	movq	1824(%rdi), %r14
	subq	%rax, %r9
	movq	1680(%rdi), %rax
	movq	1688(%rdi), %rsi
	movq	1768(%rdi), %r8
	movq	%r8, 448(%rsp)
	movq	1760(%rdi), %r12
	movq	1736(%rdi), %r8
	movq	%r8, 64(%rsp)
	movq	1728(%rdi), %rbx
	imulq	%rdx, %r9
	movq	%r9, 672(%rsp)
	movq	%rdx, %r13
	xorl	%r11d, %r11d
	xorl	%edi, %edi
	jmp	.LBB60_96
	.p2align	4
.LBB60_94:
	xorl	%r9d, %r9d
.LBB60_95:
	decq	%r13
	addq	$4, %rdi
.Ltmp8130:
	movl	%r9d, (%r12,%r11,4)
.Ltmp8131:
	incq	%r11
.Ltmp8132:
	.loc	43 37 12 is_stmt 1
	testq	%r13, %r13
	je	.LBB60_111
.LBB60_96:
.Ltmp8133:
	.loc	16 1714 9
	cmpq	$32, %rdi
.Ltmp8134:
	.loc	17 180 28
	je	.LBB60_111
.Ltmp8135:
	.loc	15 1158 21
	cmpq	160(%rsp), %r11
	je	.LBB60_255
	leaq	(%r11,%r11,2), %r9
	movl	4(%r14,%r9,4), %r8d
.Ltmp8136:
	.loc	15 1160 23
	addq	8(%rsp), %r8
.Ltmp8137:
	.loc	15 1161 12
	cmpq	%rcx, %r8
	movl	$0, %r10d
	cmovaeq	%rcx, %r10
	subq	%r10, %r8
.Ltmp8138:
	.loc	15 1168 42
	movq	%r8, %r15
	imulq	%rdx, %r15
	addq	%r11, %r15
	.loc	15 1168 22 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB60_257
.Ltmp8139:
	.loc	15 1169 24 is_stmt 1
	cmpq	448(%rsp), %r11
	je	.LBB60_258
.Ltmp8140:
	.loc	15 0 0 is_stmt 0
	movl	(%r14,%r9,4), %r10d
.Ltmp8141:
	vmovss	(%rax,%r15,4), %xmm8
.Ltmp8142:
	.loc	15 1169 24
	movl	(%r12,%r11,4), %r9d
	testq	%r9, %r9
.Ltmp8143:
	.loc	15 1170 26 is_stmt 1
	je	.LBB60_104
	.loc	15 1173 24
	cmpq	64(%rsp), %r11
	jae	.LBB60_259
	vmovss	(%rbx,%r11,4), %xmm13
.Ltmp8144:
	.loc	15 798 8
	vucomiss	%xmm13, %xmm8
	jbe	.LBB60_104
.Ltmp8145:
	.loc	15 0 8 is_stmt 0
	vmovaps	%xmm13, %xmm8
.LBB60_104:
.Ltmp8146:
	.loc	15 1175 9 is_stmt 1
	cmpq	64(%rsp), %r11
	je	.LBB60_256
	vmovss	%xmm8, (%rbx,%r11,4)
	.loc	15 1176 24
	incq	%r9
	cmpq	%r10, %r9
.Ltmp8147:
	.loc	15 1177 23
	jne	.LBB60_109
	.loc	15 1177 9 is_stmt 0
	vmovss	%xmm8, 832(%rsp,%rdi)
	.loc	15 1183 30 is_stmt 1
	vmovss	(%rax,%r15,4), %xmm8
	.loc	15 0 30 is_stmt 0
.Ltmp8148:
	.p2align	4
.LBB60_107:
.Ltmp8149:
	.loc	15 1186 65 is_stmt 1
	movq	%r8, %r15
	imulq	%rdx, %r15
	addq	%r11, %r15
	.loc	15 1186 45 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB60_250
.Ltmp8150:
	.loc	15 798 8 is_stmt 1
	vminss	(%rax,%r15,4), %xmm8, %xmm8
.Ltmp8151:
	.loc	15 1187 17
	vmovss	%xmm8, (%rax,%r15,4)
	.loc	15 1188 20
	testq	%r8, %r8
	cmoveq	%rcx, %r8
	.loc	15 1191 17
	decq	%r8
.Ltmp8152:
	.loc	10 1916 50
	decq	%r10
.Ltmp8153:
	.loc	11 900 12
	jne	.LBB60_107
	jmp	.LBB60_94
.Ltmp8154:
	.loc	11 0 12 is_stmt 0
.Ltmp8155:
	.p2align	4
.LBB60_109:
	movq	672(%rsp), %r8
	.loc	15 1180 44 is_stmt 1
	leaq	(%r11,%r8), %r15
	.loc	15 1180 24 is_stmt 0
	cmpq	%rsi, %r15
	jae	.LBB60_260
	vmovss	(%rax,%r15,4), %xmm13
.Ltmp8156:
	.loc	15 798 8 is_stmt 1
	vminss	%xmm8, %xmm13, %xmm8
.Ltmp8157:
	.loc	15 1177 9
	vmovss	%xmm8, 832(%rsp,%rdi)
	jmp	.LBB60_95
.Ltmp8158:
	.loc	15 0 9 is_stmt 0
.Ltmp8159:
	.p2align	4
.LBB60_111:
	.loc	1 551 14 is_stmt 1
	vmovaps	832(%rsp), %ymm13
	movq	56(%rsp), %r12
.Ltmp8160:
	.loc	15 1307 26
	movq	1704(%r12), %rsi
	vmovaps	%ymm13, %ymm14
	movq	128(%rsp), %rdi
.Ltmp8161:
	.loc	43 37 12
	testq	%rdi, %rdi
	movq	40(%rsp), %r14
	movq	640(%rsp), %rbx
	movq	2176(%rsp), %r11
	je	.LBB60_137
.Ltmp8162:
	.loc	43 0 12 is_stmt 0
	movq	1832(%r12), %r9
.Ltmp8163:
	.loc	15 1298 42 is_stmt 1
	testq	%r9, %r9
	je	.LBB60_282
	.loc	15 0 42 is_stmt 0
	movq	1824(%r12), %r10
	.loc	15 1298 42
	movl	8(%r10), %r8d
	.loc	15 1298 28
	addq	8(%rsp), %r8
.Ltmp8164:
	.loc	15 1299 16 is_stmt 1
	cmpq	%rcx, %r8
	movl	$0, %eax
	cmovaeq	%rcx, %rax
	subq	%rax, %r8
	.loc	15 1302 40
	imulq	%rdi, %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB60_263
	.loc	15 0 25
	movq	1696(%r12), %rdx
	.loc	15 1302 25
	vmovss	(%rdx,%r8,4), %xmm8
	.loc	15 1302 13
	vmovss	%xmm8, 832(%rsp)
.Ltmp8165:
	.loc	43 37 12 is_stmt 1
	cmpq	$1, %rdi
	je	.LBB60_136
.Ltmp8166:
	.loc	15 1298 42
	cmpq	$1, %r9
	je	.LBB60_264
	movl	20(%r10), %eax
	.loc	15 1298 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp8167:
	.loc	15 1299 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	128(%rsp), %rdi
	.loc	15 1302 40
	imulq	%rdi, %rax
	leaq	1(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB60_263
	vmovss	4(%rdx,%rax,4), %xmm8
	.loc	15 1302 13
	vmovss	%xmm8, 836(%rsp)
.Ltmp8168:
	.loc	43 37 12 is_stmt 1
	cmpq	$2, %rdi
	je	.LBB60_136
.Ltmp8169:
	.loc	15 1298 42
	cmpq	$2, %r9
	je	.LBB60_265
	movl	32(%r10), %eax
	.loc	15 1298 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp8170:
	.loc	15 1299 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	128(%rsp), %rdi
	.loc	15 1302 40
	imulq	%rdi, %rax
	leaq	2(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB60_263
	vmovss	8(%rdx,%rax,4), %xmm8
	.loc	15 1302 13
	vmovss	%xmm8, 840(%rsp)
.Ltmp8171:
	.loc	43 37 12 is_stmt 1
	cmpq	$3, %rdi
	je	.LBB60_136
.Ltmp8172:
	.loc	15 1298 42
	cmpq	$3, %r9
	je	.LBB60_267
	movl	44(%r10), %eax
	.loc	15 1298 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp8173:
	.loc	15 1299 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	128(%rsp), %rdi
	.loc	15 1302 40
	imulq	%rdi, %rax
	leaq	3(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB60_263
	vmovss	12(%rdx,%rax,4), %xmm8
	.loc	15 1302 13
	vmovss	%xmm8, 844(%rsp)
.Ltmp8174:
	.loc	43 37 12 is_stmt 1
	cmpq	$4, %rdi
	je	.LBB60_136
.Ltmp8175:
	.loc	15 1298 42
	cmpq	$4, %r9
	je	.LBB60_268
	movl	56(%r10), %eax
	.loc	15 1298 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp8176:
	.loc	15 1299 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	128(%rsp), %rdi
	.loc	15 1302 40
	imulq	%rdi, %rax
	leaq	4(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB60_263
	vmovss	16(%rdx,%rax,4), %xmm8
	.loc	15 1302 13
	vmovss	%xmm8, 848(%rsp)
.Ltmp8177:
	.loc	43 37 12 is_stmt 1
	cmpq	$5, %rdi
	je	.LBB60_136
.Ltmp8178:
	.loc	15 1298 42
	cmpq	$5, %r9
	je	.LBB60_276
	movl	68(%r10), %eax
	.loc	15 1298 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp8179:
	.loc	15 1299 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	128(%rsp), %rdi
	.loc	15 1302 40
	imulq	%rdi, %rax
	leaq	5(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB60_263
	vmovss	20(%rdx,%rax,4), %xmm8
	.loc	15 1302 13
	vmovss	%xmm8, 852(%rsp)
.Ltmp8180:
	.loc	43 37 12 is_stmt 1
	cmpq	$6, %rdi
	je	.LBB60_136
.Ltmp8181:
	.loc	15 1298 42
	cmpq	$6, %r9
	je	.LBB60_279
	movl	80(%r10), %eax
	.loc	15 1298 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp8182:
	.loc	15 1299 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	128(%rsp), %rdi
	.loc	15 1302 40
	imulq	%rdi, %rax
	leaq	6(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB60_263
	vmovss	24(%rdx,%rax,4), %xmm8
	.loc	15 1302 13
	vmovss	%xmm8, 856(%rsp)
.Ltmp8183:
	.loc	43 37 12 is_stmt 1
	cmpq	$7, %rdi
	je	.LBB60_136
.Ltmp8184:
	.loc	15 1298 42
	cmpq	$7, %r9
	je	.LBB60_280
	movl	92(%r10), %eax
	.loc	15 1298 28 is_stmt 0
	addq	8(%rsp), %rax
.Ltmp8185:
	.loc	15 1299 16 is_stmt 1
	cmpq	%rcx, %rax
	movl	$0, %edi
	cmovaeq	%rcx, %rdi
	subq	%rdi, %rax
	movq	128(%rsp), %rdi
	.loc	15 1302 40
	imulq	%rdi, %rax
	leaq	7(%rax), %r8
	.loc	15 1302 25 is_stmt 0
	cmpq	%rsi, %r8
	jae	.LBB60_263
	vmovss	28(%rdx,%rax,4), %xmm8
	.loc	15 1302 13
	vmovss	%xmm8, 860(%rsp)
.Ltmp8186:
	.loc	15 0 13
.Ltmp8187:
	.p2align	4
.LBB60_136:
	.loc	1 551 14 is_stmt 1
	vmovaps	832(%rsp), %ymm14
.Ltmp8188:
.LBB60_137:
	.loc	15 0 0 is_stmt 0
	vmulps	%ymm6, %ymm13, %ymm8
	vroundps	$9, %ymm8, %ymm8
	vbroadcastss	.LCPI60_4(%rip), %ymm13
	vmulps	%ymm13, %ymm8, %ymm8
.Ltmp8189:
	.loc	50 48 14 is_stmt 1
	vaddps	%ymm8, %ymm10, %ymm10
.Ltmp8190:
	.loc	50 347 14
	vsubps	%ymm14, %ymm10, %ymm10
.Ltmp8191:
	.loc	37 580 12
	movq	%rsi, %rdx
	subq	352(%rsp), %rdx
	jb	.LBB60_235
.Ltmp8192:
	.loc	37 451 16
	cmpq	$7, %rdx
	movq	96(%rsp), %r13
	jbe	.LBB60_151
.Ltmp8193:
	.loc	15 1307 26
	movq	1696(%r12), %rax
	movq	352(%rsp), %rdx
.Ltmp8194:
	.loc	1 551 14
	vmovups	%ymm8, (%rax,%rdx,4)
.Ltmp8195:
	.loc	50 360 14
	vdivps	%ymm4, %ymm10, %ymm8
.Ltmp8196:
	.loc	15 1311 43
	vmovaps	1600(%rsp), %ymm13
.Ltmp8197:
	.loc	50 347 14
	vsubps	%ymm8, %ymm5, %ymm8
.Ltmp8198:
	.loc	50 347 14 is_stmt 0
	vsubps	%ymm13, %ymm8, %ymm14
.Ltmp8199:
	.loc	50 283 14 is_stmt 1
	vmulps	%ymm14, %ymm9, %ymm14
.Ltmp8200:
	.loc	50 48 14
	vaddps	%ymm14, %ymm13, %ymm13
.Ltmp8201:
	.loc	50 233 14
	vmaxps	%ymm13, %ymm8, %ymm8
.Ltmp8202:
	.loc	50 82 19
	vbroadcastss	.LCPI60_0(%rip), %ymm13
	vandps	%ymm13, %ymm8, %ymm13
.Ltmp8203:
	.loc	50 871 14
	vbroadcastss	.LCPI60_5(%rip), %ymm14
	vcmplt_oqps	%ymm14, %ymm13, %ymm13
.Ltmp8204:
	.loc	50 82 19
	vandnps	%ymm8, %ymm13, %ymm8
.Ltmp8205:
	.loc	15 1312 5
	vmovaps	%ymm8, 1600(%rsp)
.Ltmp8206:
	.loc	15 1315 28
	movq	1672(%r12), %rsi
	.loc	15 1315 44 is_stmt 0
	imulq	%rbx, %rdi
.Ltmp8207:
	.loc	37 568 12 is_stmt 1
	movq	%rsi, %rdx
	subq	%rdi, %rdx
	jb	.LBB60_236
.Ltmp8208:
	.loc	37 438 16
	cmpq	$7, %rdx
	jbe	.LBB60_151
.Ltmp8209:
	.loc	37 0 16 is_stmt 0
	movq	608(%rsp), %r8
	incq	%r8
	movq	1856(%rsp), %rax
.Ltmp8210:
	leaq	(%r14,%rax,4), %rax
.Ltmp8211:
	vsubps	%ymm8, %ymm5, %ymm8
.Ltmp8212:
	.loc	15 1315 28 is_stmt 1
	movq	1664(%r12), %rdx
.Ltmp8213:
	.loc	1 551 14
	vmovups	(%rdx,%rdi,4), %ymm13
.Ltmp8214:
	.loc	1 551 14 is_stmt 0
	vmovups	(%rax), %ymm14
	vmovups	%ymm14, (%rdx,%rdi,4)
.Ltmp8215:
	.loc	50 283 14 is_stmt 1
	vmulps	%ymm13, %ymm8, %ymm8
	vmovaps	1952(%rsp), %ymm14
.Ltmp8216:
	.loc	50 585 19
	vblendvps	%ymm14, %ymm13, %ymm8, %ymm8
.Ltmp8217:
	.loc	1 551 14
	vmovups	%ymm8, (%rax)
.Ltmp8218:
	.loc	15 3115 13
	incq	%rbx
	.loc	15 3116 16
	cmpq	1792(%rsp), %rbx
	movl	$0, %esi
	cmoveq	%rsi, %rbx
	movq	8(%rsp), %rax
	.loc	15 3119 13
	incq	%rax
	.loc	15 3120 16
	cmpq	%rcx, %rax
	movl	$0, %edx
	movq	%rdx, 288(%rsp)
	cmoveq	%rsi, %rax
	movq	%rax, 8(%rsp)
.Ltmp8219:
	.loc	10 1916 50
	cmpq	2208(%rsp), %r8
	movq	2016(%rsp), %rdx
.Ltmp8220:
	.loc	11 900 12
	jne	.LBB60_86
	jmp	.LBB60_76
.Ltmp8221:
.LBB60_142:
	.loc	11 0 12 is_stmt 0
	movq	40(%rsp), %r14
	movq	%r15, %rbx
.LBB60_143:
	leaq	960(%rsp), %rdi
	movq	16(%rsp), %rsi
	.loc	15 3126 14 is_stmt 1
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	.loc	15 3127 5
	movl	%ebx, 1640(%r12)
	movq	8(%rsp), %rax
.Ltmp8222:
	.loc	15 0 0 is_stmt 0
	movl	%eax, 1644(%r12)
.Ltmp8223:
	.loc	15 2194 13 is_stmt 1
	cmpb	$0, 124(%rsp)
	jne	.LBB60_200
	jmp	.LBB60_208
.LBB60_151:
.Ltmp8224:
	.loc	15 749 9
	vmovaps	%ymm11, 1536(%rsp)
.Ltmp8225:
	.loc	15 1306 5
	vmovaps	%ymm10, 1632(%rsp)
.Ltmp8226:
.LBB60_152:
	.loc	15 0 0 is_stmt 0
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB60_153:
	movb	$1, %al
	movl	%eax, 8(%rsp)
.LBB60_154:
.Ltmp8227:
	.loc	15 3150 19 is_stmt 1
	movzbl	1536(%r12), %eax
	movb	%al, 160(%rsp)
.Ltmp8228:
	.loc	15 3151 21
	movzbl	1537(%r12), %eax
	movb	%al, 64(%rsp)
.Ltmp8229:
	.loc	15 3152 16
	movq	1624(%r12), %r15
.Ltmp8230:
	.loc	15 3153 16
	movq	1632(%r12), %r14
.Ltmp8231:
	.loc	15 3154 27
	movl	1640(%r12), %ebx
.Ltmp8232:
	.loc	15 3155 27
	movl	1644(%r12), %eax
	movq	%rax, 256(%rsp)
	leaq	3904(%rsp), %rdi
	movl	$1024, %edx
	xorl	%esi, %esi
	callq	*memset@GOTPCREL(%rip)
	leaq	832(%rsp), %rdi
	movq	16(%rsp), %rsi
.Ltmp8233:
	.loc	15 3159 32
	movq	%r15, %rdx
	movq	%r14, 720(%rsp)
	movq	%r14, %rcx
	callq	_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_
	movq	24(%rsp), %rcx
.Ltmp8234:
	.loc	14 3758 16
	leaq	31(%rcx), %r9
	shrq	$5, %r9
.Ltmp8235:
	.loc	40 446 20
	je	.LBB60_194
.Ltmp8236:
	.loc	50 871 14
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm4
.Ltmp8237:
	.loc	50 713 19
	vpcmpeqd	%ymm0, %ymm0, %ymm0
	vxorps	%ymm0, %ymm4, %ymm0
	vmovaps	%ymm4, %ymm14
	cmpb	$0, 160(%rsp)
	jne	.LBB60_157
.Ltmp8238:
	.loc	50 0 19 is_stmt 0
	vmovaps	%ymm0, %ymm14
.LBB60_157:
	movabsq	$2305843009213693944, %rax
	movq	%rax, 336(%rsp)
	cmpb	$0, 64(%rsp)
	movq	40(%rsp), %r14
	movq	%rbx, %rdi
	jne	.LBB60_159
	vmovaps	%ymm0, %ymm4
.LBB60_159:
.Ltmp8239:
	.loc	15 1612 23 is_stmt 1
	vmovaps	960(%rsp), %ymm9
	vmovaps	992(%rsp), %ymm13
	vmovaps	1024(%rsp), %ymm5
	vmovaps	1056(%rsp), %ymm12
	vmovaps	1088(%rsp), %ymm7
	vmovaps	1120(%rsp), %ymm10
	vmovaps	1152(%rsp), %ymm0
	vmovaps	%ymm0, 2112(%rsp)
	vmovaps	1184(%rsp), %ymm8
	vmovaps	1216(%rsp), %ymm11
	vmovaps	1248(%rsp), %ymm6
	vmovaps	1280(%rsp), %ymm1
	vmovaps	1312(%rsp), %ymm2
	movq	920(%rsp), %rax
	movq	%rax, 744(%rsp)
	movq	928(%rsp), %rax
	movq	%rax, 736(%rsp)
	movq	864(%rsp), %r10
	movq	872(%rsp), %rdx
	movq	912(%rsp), %rax
	movq	%rax, 1856(%rsp)
	movq	880(%rsp), %rax
	movq	%rax, 104(%rsp)
	movq	888(%rsp), %rax
	movq	%rax, 48(%rsp)
	movq	904(%rsp), %rax
	movq	%rax, 1824(%rsp)
	movq	896(%rsp), %rax
	movq	%rax, 1792(%rsp)
	vmovaps	832(%rsp), %ymm0
	vmovaps	%ymm0, 2080(%rsp)
	movl	936(%rsp), %r11d
	vmovaps	1376(%rsp), %ymm0
	vmovaps	%ymm0, 3488(%rsp)
	vmovaps	1504(%rsp), %ymm0
	vmovaps	%ymm0, 3456(%rsp)
	vmovaps	1664(%rsp), %ymm0
	vmovaps	%ymm0, 2048(%rsp)
	movl	$32, %esi
	addq	$7, 336(%rsp)
	movq	%r14, 768(%rsp)
	movq	%r13, %r8
	movq	$0, 800(%rsp)
	movq	%rcx, %rax
	xorl	%ebx, %ebx
	vmovaps	1440(%rsp), %ymm15
	vmovaps	1344(%rsp), %ymm0
	vmovaps	%ymm0, 192(%rsp)
	vmovaps	1408(%rsp), %ymm0
	vmovaps	%ymm0, 224(%rsp)
	vmovaps	1568(%rsp), %ymm0
	vmovaps	%ymm0, 640(%rsp)
	vmovaps	1472(%rsp), %ymm0
	vmovaps	%ymm0, 3328(%rsp)
	vmovaps	1536(%rsp), %ymm0
	vmovaps	%ymm0, 608(%rsp)
	vmovaps	1632(%rsp), %ymm0
	vmovaps	%ymm0, 64(%rsp)
	vmovaps	%ymm4, 3424(%rsp)
	vmovaps	%ymm14, 3392(%rsp)
.Ltmp8240:
.LBB60_160:
	.loc	15 0 23 is_stmt 0
	vmovaps	%ymm5, 1888(%rsp)
	movq	%rdi, 416(%rsp)
.Ltmp8241:
	.loc	14 2584 13 is_stmt 1
	cmpq	$32, %rax
	movl	$32, %edi
	movq	%rax, 704(%rsp)
	cmovbq	%rax, %rdi
	cmpq	$1, %rdi
	movq	%rdi, 728(%rsp)
	movq	%rdi, %rax
	adcq	$0, %rax
.Ltmp8242:
	.loc	10 1916 50
	movq	%rcx, %rdi
	subq	%rbx, %rdi
.Ltmp8243:
	.loc	10 1078 5
	cmpq	$32, %rdi
	cmovaeq	%rsi, %rdi
	movq	%rdi, 344(%rsp)
	movq	%rbx, 112(%rsp)
.Ltmp8244:
	.loc	10 1916 50
	cmpq	%rbx, %rcx
	movq	%r8, 712(%rsp)
	vmovaps	%ymm15, 3360(%rsp)
.Ltmp8245:
	.loc	11 900 12
	jne	.LBB60_162
	.loc	11 0 12 is_stmt 0
	vmovaps	%ymm13, %ymm5
	vmovaps	2112(%rsp), %ymm3
	vmovaps	%ymm2, %ymm15
	vmovaps	1888(%rsp), %ymm4
	.loc	11 900 12
	jmp	.LBB60_166
.Ltmp8246:
	.loc	11 0 12
.Ltmp8247:
	.p2align	4
.LBB60_162:
	shll	$3, %eax
	vmovaps	(%r12), %ymm0
	vmovaps	%ymm0, 576(%rsp)
	vmovaps	32(%r12), %ymm0
	vmovaps	%ymm0, 384(%rsp)
	vmovaps	64(%r12), %ymm0
	vmovaps	%ymm0, 544(%rsp)
	vmovaps	96(%r12), %ymm0
	vmovaps	%ymm0, 512(%rsp)
	vmovaps	128(%r12), %ymm0
	vmovaps	%ymm0, 480(%rsp)
	vmovaps	160(%r12), %ymm0
	vmovaps	%ymm0, 1760(%rsp)
	vmovaps	192(%r12), %ymm0
	vmovaps	%ymm0, 3232(%rsp)
	vmovaps	224(%r12), %ymm0
	vmovaps	%ymm0, 3200(%rsp)
	vmovaps	256(%r12), %ymm0
	vmovaps	%ymm0, 3168(%rsp)
	vmovaps	288(%r12), %ymm0
	vmovaps	%ymm0, 3136(%rsp)
	vmovaps	320(%r12), %ymm0
	vmovaps	%ymm0, 3104(%rsp)
	vmovaps	352(%r12), %ymm0
	vmovaps	%ymm0, 3072(%rsp)
	xorl	%ecx, %ecx
	vmovaps	%ymm1, 352(%rsp)
	vmovaps	%ymm6, %ymm14
	vmovaps	%ymm13, %ymm0
	vmovaps	%ymm11, %ymm13
	vmovaps	%ymm8, %ymm2
	vmovaps	384(%r12), %ymm1
	vmovaps	%ymm1, 3040(%rsp)
	vmovaps	416(%r12), %ymm1
	vmovaps	%ymm1, 3008(%rsp)
	vmovaps	448(%r12), %ymm1
	vmovaps	%ymm1, 2976(%rsp)
	vmovaps	480(%r12), %ymm1
	vmovaps	%ymm1, 2944(%rsp)
	vmovaps	512(%r12), %ymm1
	vmovaps	%ymm1, 2912(%rsp)
	vmovaps	544(%r12), %ymm1
	vmovaps	%ymm1, 2880(%rsp)
	vmovaps	576(%r12), %ymm1
	vmovaps	%ymm1, 2848(%rsp)
	vmovaps	608(%r12), %ymm1
	vmovaps	%ymm1, 2816(%rsp)
	vmovaps	640(%r12), %ymm1
	vmovaps	%ymm1, 2784(%rsp)
	vmovaps	672(%r12), %ymm1
	vmovaps	%ymm1, 2752(%rsp)
	vmovaps	704(%r12), %ymm1
	vmovaps	%ymm1, 2720(%rsp)
	vmovaps	736(%r12), %ymm1
	vmovaps	%ymm1, 2688(%rsp)
	vmovaps	768(%r12), %ymm1
	vmovaps	%ymm1, 2656(%rsp)
	vmovaps	800(%r12), %ymm1
	vmovaps	%ymm1, 2624(%rsp)
	vmovaps	832(%r12), %ymm1
	vmovaps	%ymm1, 2592(%rsp)
	vmovaps	864(%r12), %ymm1
	vmovaps	%ymm1, 2560(%rsp)
	vmovaps	896(%r12), %ymm1
	vmovaps	%ymm1, 2528(%rsp)
	vmovaps	928(%r12), %ymm1
	vmovaps	%ymm1, 2496(%rsp)
	vmovaps	960(%r12), %ymm1
	vmovaps	%ymm1, 2464(%rsp)
	vmovaps	992(%r12), %ymm1
	vmovaps	%ymm1, 2432(%rsp)
	vmovaps	1024(%r12), %ymm1
	vmovaps	%ymm1, 2400(%rsp)
	vmovaps	1056(%r12), %ymm1
	vmovaps	%ymm1, 2368(%rsp)
	vmovaps	1088(%r12), %ymm1
	vmovaps	%ymm1, 2336(%rsp)
	vmovaps	1120(%r12), %ymm1
	vmovaps	%ymm1, 2304(%rsp)
	vmovaps	1152(%r12), %ymm1
	vmovaps	%ymm1, 2272(%rsp)
	vmovaps	1184(%r12), %ymm1
	vmovaps	%ymm1, 2240(%rsp)
	vmovaps	1216(%r12), %ymm1
	vmovaps	%ymm1, 2016(%rsp)
	vmovaps	1248(%r12), %ymm1
	vmovaps	%ymm1, 2208(%rsp)
	vmovaps	1280(%r12), %ymm1
	vmovaps	%ymm1, 2176(%rsp)
	vmovaps	1312(%r12), %ymm1
	vmovaps	%ymm1, 1984(%rsp)
	vmovaps	1344(%r12), %ymm1
	vmovaps	%ymm1, 1952(%rsp)
	vmovaps	1376(%r12), %ymm1
	vmovaps	%ymm1, 288(%rsp)
	vmovaps	1408(%r12), %ymm1
	vmovaps	%ymm1, 1728(%rsp)
	vmovaps	1440(%r12), %ymm1
	vmovaps	%ymm1, 1696(%rsp)
	vmovaps	1472(%r12), %ymm1
	vmovaps	%ymm1, 2144(%rsp)
	vmovaps	1504(%r12), %ymm1
	vmovaps	%ymm1, 1920(%rsp)
	vmovaps	%ymm0, 3296(%rsp)
	vmovaps	%ymm9, 3264(%rsp)
	vmovaps	2112(%rsp), %ymm3
	vmovaps	1888(%rsp), %ymm4
	.p2align	4
.LBB60_163:
	vmovaps	%ymm14, 160(%rsp)
	vmovaps	352(%rsp), %ymm1
	vmovaps	%ymm1, 448(%rsp)
	movq	800(%rsp), %rdi
.Ltmp8248:
	.loc	37 568 12 is_stmt 1
	addq	%rcx, %rdi
	cmpq	%r13, %rdi
	ja	.LBB60_251
.Ltmp8249:
	.loc	37 438 16
	cmpq	$7, %r8
	jbe	.LBB60_253
.Ltmp8250:
	.loc	37 0 16 is_stmt 0
	vmovaps	%ymm3, %ymm8
	vmovaps	%ymm2, %ymm11
	vmovaps	%ymm9, %ymm15
	vmovaps	%ymm10, %ymm3
	vmovaps	%ymm12, %ymm6
	vmovaps	%ymm0, %ymm10
	movq	768(%rsp), %rdi
.Ltmp8251:
	.loc	1 551 14 is_stmt 1
	vmovups	(%rdi,%rcx,4), %ymm14
.Ltmp8252:
	.loc	50 82 19
	vbroadcastss	.LCPI60_0(%rip), %ymm2
.Ltmp8253:
	.loc	50 283 14
	vmulps	576(%rsp), %ymm14, %ymm0
	vxorps	%xmm9, %xmm9, %xmm9
.Ltmp8254:
	.loc	50 48 14
	vaddps	%ymm0, %ymm9, %ymm0
.Ltmp8255:
	.loc	50 283 14
	vmulps	384(%rsp), %ymm14, %ymm12
.Ltmp8256:
	.loc	50 48 14
	vaddps	%ymm9, %ymm12, %ymm12
	vmovaps	%ymm13, %ymm5
.Ltmp8257:
	.loc	50 283 14
	vmulps	544(%rsp), %ymm14, %ymm13
	vmovaps	%ymm14, 128(%rsp)
.Ltmp8258:
	.loc	50 48 14
	vaddps	%ymm9, %ymm13, %ymm13
.Ltmp8259:
	.loc	50 283 14
	vmulps	512(%rsp), %ymm14, %ymm14
.Ltmp8260:
	.loc	50 48 14
	vaddps	%ymm9, %ymm14, %ymm14
	vmovaps	%ymm7, %ymm9
	vmovaps	%ymm6, %ymm7
	vmovaps	%ymm4, %ymm6
	vmovaps	%ymm10, %ymm4
.Ltmp8261:
	.loc	50 283 14
	vmulps	480(%rsp), %ymm15, %ymm10
.Ltmp8262:
	.loc	50 48 14
	vaddps	%ymm0, %ymm10, %ymm0
.Ltmp8263:
	.loc	50 283 14
	vmulps	1760(%rsp), %ymm15, %ymm10
.Ltmp8264:
	.loc	50 48 14
	vaddps	%ymm12, %ymm10, %ymm10
.Ltmp8265:
	.loc	50 283 14
	vmulps	3232(%rsp), %ymm15, %ymm12
.Ltmp8266:
	.loc	50 48 14
	vaddps	%ymm13, %ymm12, %ymm12
.Ltmp8267:
	.loc	50 283 14
	vmulps	3200(%rsp), %ymm15, %ymm13
.Ltmp8268:
	.loc	50 48 14
	vaddps	%ymm14, %ymm13, %ymm13
.Ltmp8269:
	.loc	50 283 14
	vmulps	3168(%rsp), %ymm4, %ymm14
.Ltmp8270:
	.loc	50 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp8271:
	.loc	50 283 14
	vmulps	3136(%rsp), %ymm4, %ymm14
.Ltmp8272:
	.loc	50 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp8273:
	.loc	50 283 14
	vmulps	3104(%rsp), %ymm4, %ymm14
.Ltmp8274:
	.loc	50 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp8275:
	.loc	50 283 14
	vmulps	3072(%rsp), %ymm4, %ymm14
.Ltmp8276:
	.loc	50 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp8277:
	.loc	50 283 14
	vmulps	3040(%rsp), %ymm6, %ymm14
.Ltmp8278:
	.loc	50 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp8279:
	.loc	50 283 14
	vmulps	3008(%rsp), %ymm6, %ymm14
.Ltmp8280:
	.loc	50 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp8281:
	.loc	50 283 14
	vmulps	2976(%rsp), %ymm6, %ymm14
.Ltmp8282:
	.loc	50 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp8283:
	.loc	50 283 14
	vmulps	2944(%rsp), %ymm6, %ymm14
.Ltmp8284:
	.loc	50 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp8285:
	.loc	50 283 14
	vmulps	2912(%rsp), %ymm7, %ymm14
.Ltmp8286:
	.loc	50 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp8287:
	.loc	50 283 14
	vmulps	2880(%rsp), %ymm7, %ymm14
.Ltmp8288:
	.loc	50 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp8289:
	.loc	50 283 14
	vmulps	2848(%rsp), %ymm7, %ymm14
.Ltmp8290:
	.loc	50 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp8291:
	.loc	50 283 14
	vmulps	2816(%rsp), %ymm7, %ymm14
.Ltmp8292:
	.loc	50 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp8293:
	.loc	50 283 14
	vmulps	2784(%rsp), %ymm9, %ymm14
.Ltmp8294:
	.loc	50 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp8295:
	.loc	50 283 14
	vmulps	2752(%rsp), %ymm9, %ymm14
.Ltmp8296:
	.loc	50 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp8297:
	.loc	50 283 14
	vmulps	2720(%rsp), %ymm9, %ymm14
.Ltmp8298:
	.loc	50 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp8299:
	.loc	50 283 14
	vmulps	2688(%rsp), %ymm9, %ymm14
.Ltmp8300:
	.loc	50 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp8301:
	.loc	50 283 14
	vmulps	2656(%rsp), %ymm3, %ymm14
.Ltmp8302:
	.loc	50 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp8303:
	.loc	50 283 14
	vmulps	2624(%rsp), %ymm3, %ymm14
.Ltmp8304:
	.loc	50 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp8305:
	.loc	50 283 14
	vmulps	2592(%rsp), %ymm3, %ymm14
.Ltmp8306:
	.loc	50 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp8307:
	.loc	50 283 14
	vmulps	2560(%rsp), %ymm3, %ymm14
.Ltmp8308:
	.loc	50 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp8309:
	.loc	50 283 14
	vmulps	2528(%rsp), %ymm8, %ymm14
.Ltmp8310:
	.loc	50 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp8311:
	.loc	50 283 14
	vmulps	2496(%rsp), %ymm8, %ymm14
.Ltmp8312:
	.loc	50 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp8313:
	.loc	50 283 14
	vmulps	2464(%rsp), %ymm8, %ymm14
.Ltmp8314:
	.loc	50 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp8315:
	.loc	50 283 14
	vmulps	2432(%rsp), %ymm8, %ymm14
.Ltmp8316:
	.loc	50 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp8317:
	.loc	50 283 14
	vmulps	2400(%rsp), %ymm11, %ymm14
.Ltmp8318:
	.loc	50 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp8319:
	.loc	50 283 14
	vmulps	2368(%rsp), %ymm11, %ymm14
.Ltmp8320:
	.loc	50 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp8321:
	.loc	50 283 14
	vmulps	2336(%rsp), %ymm11, %ymm14
.Ltmp8322:
	.loc	50 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp8323:
	.loc	50 283 14
	vmulps	2304(%rsp), %ymm11, %ymm14
.Ltmp8324:
	.loc	50 48 14
	vaddps	%ymm13, %ymm14, %ymm13
	vmovaps	%ymm5, 672(%rsp)
.Ltmp8325:
	.loc	50 283 14
	vmulps	2272(%rsp), %ymm5, %ymm14
.Ltmp8326:
	.loc	50 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp8327:
	.loc	50 283 14
	vmulps	2240(%rsp), %ymm5, %ymm14
.Ltmp8328:
	.loc	50 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp8329:
	.loc	50 283 14
	vmulps	2016(%rsp), %ymm5, %ymm14
.Ltmp8330:
	.loc	50 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp8331:
	.loc	50 283 14
	vmulps	2208(%rsp), %ymm5, %ymm14
.Ltmp8332:
	.loc	50 48 14
	vaddps	%ymm13, %ymm14, %ymm13
	vmovaps	160(%rsp), %ymm1
.Ltmp8333:
	.loc	50 283 14
	vmulps	2176(%rsp), %ymm1, %ymm14
.Ltmp8334:
	.loc	50 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp8335:
	.loc	50 283 14
	vmulps	1984(%rsp), %ymm1, %ymm14
.Ltmp8336:
	.loc	50 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp8337:
	.loc	50 283 14
	vmulps	1952(%rsp), %ymm1, %ymm14
.Ltmp8338:
	.loc	50 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp8339:
	.loc	50 283 14
	vmulps	288(%rsp), %ymm1, %ymm14
.Ltmp8340:
	.loc	50 48 14
	vaddps	%ymm13, %ymm14, %ymm13
	vmovaps	%ymm15, %ymm5
	vmovaps	448(%rsp), %ymm15
.Ltmp8341:
	.loc	50 283 14
	vmulps	1728(%rsp), %ymm15, %ymm14
.Ltmp8342:
	.loc	50 48 14
	vaddps	%ymm0, %ymm14, %ymm0
.Ltmp8343:
	.loc	50 283 14
	vmulps	1696(%rsp), %ymm15, %ymm14
.Ltmp8344:
	.loc	50 48 14
	vaddps	%ymm10, %ymm14, %ymm10
.Ltmp8345:
	.loc	50 283 14
	vmulps	2144(%rsp), %ymm15, %ymm14
.Ltmp8346:
	.loc	50 48 14
	vaddps	%ymm12, %ymm14, %ymm12
.Ltmp8347:
	.loc	50 283 14
	vmulps	1920(%rsp), %ymm15, %ymm14
.Ltmp8348:
	.loc	50 48 14
	vaddps	%ymm13, %ymm14, %ymm13
.Ltmp8349:
	.loc	50 82 19
	vandps	%ymm2, %ymm3, %ymm14
.Ltmp8350:
	.loc	50 82 19 is_stmt 0
	vandps	%ymm2, %ymm0, %ymm0
.Ltmp8351:
	.loc	50 233 14 is_stmt 1
	vmaxps	%ymm0, %ymm14, %ymm0
.Ltmp8352:
	.loc	50 82 19
	vandps	%ymm2, %ymm10, %ymm10
.Ltmp8353:
	.loc	50 233 14
	vmaxps	%ymm10, %ymm0, %ymm0
.Ltmp8354:
	.loc	50 82 19
	vandps	%ymm2, %ymm12, %ymm10
.Ltmp8355:
	.loc	50 233 14
	vmaxps	%ymm10, %ymm0, %ymm0
	vmovaps	%ymm6, %ymm12
	vmovaps	%ymm9, %ymm10
	vmovaps	128(%rsp), %ymm9
.Ltmp8356:
	.loc	50 82 19
	vandps	%ymm2, %ymm13, %ymm2
.Ltmp8357:
	.loc	50 233 14
	vmaxps	%ymm2, %ymm0, %ymm0
.Ltmp8358:
	.loc	1 551 14
	vmovups	%ymm0, 3904(%rsp,%rcx,4)
.Ltmp8359:
	.loc	10 1916 50
	addq	$8, %rcx
	addq	$-8, %r8
	vmovaps	%ymm5, %ymm0
	vmovaps	%ymm1, 352(%rsp)
	vmovaps	672(%rsp), %ymm14
	vmovaps	%ymm14, %ymm6
	vmovaps	%ymm11, %ymm13
	vmovaps	%ymm8, %ymm2
	cmpq	%rcx, %rax
.Ltmp8360:
	.loc	11 900 12
	jne	.LBB60_163
.Ltmp8361:
.LBB60_166:
	.loc	15 0 0 is_stmt 0
	vmovaps	%ymm5, %ymm13
	vmovaps	%ymm4, %ymm5
.Ltmp8362:
	.loc	15 1618 5 is_stmt 1
	vmovaps	%ymm4, 1024(%rsp)
	vmovaps	%ymm12, 1056(%rsp)
	vmovaps	%ymm7, 1088(%rsp)
	vmovaps	%ymm10, 1120(%rsp)
	vmovaps	%ymm3, 2112(%rsp)
	vmovaps	%ymm3, 1152(%rsp)
	vmovaps	%ymm8, 1184(%rsp)
	vmovaps	%ymm11, 1216(%rsp)
	vmovaps	%ymm6, 1248(%rsp)
	vmovaps	%ymm1, 1280(%rsp)
	vmovaps	%ymm15, 1312(%rsp)
	movq	24(%rsp), %rcx
	movq	112(%rsp), %rbx
.Ltmp8363:
	.loc	10 1916 50
	cmpq	%rbx, %rcx
	vmovaps	3424(%rsp), %ymm4
	vmovaps	%ymm15, %ymm2
.Ltmp8364:
	.loc	15 3173 19
	jne	.LBB60_169
.Ltmp8365:
	.loc	15 0 19 is_stmt 0
	movq	416(%rsp), %rdi
	vmovaps	3360(%rsp), %ymm15
.LBB60_168:
	addq	$32, %rbx
	decq	%r9
	movq	704(%rsp), %rax
.Ltmp8366:
	.loc	40 446 20 is_stmt 1
	addq	$-32, %rax
	addq	$256, 800(%rsp)
	movq	712(%rsp), %r8
	addq	$-256, %r8
	addq	$1024, 768(%rsp)
	testq	%r9, %r9
	movl	$32, %esi
	jne	.LBB60_160
	jmp	.LBB60_249
.Ltmp8367:
	.loc	40 0 20 is_stmt 0
.Ltmp8368:
	.p2align	4
.LBB60_169:
	vmovaps	%ymm10, 1760(%rsp)
	vmovaps	%ymm2, 448(%rsp)
	vmovaps	%ymm1, 160(%rsp)
	vmovaps	%ymm6, 672(%rsp)
	vmovaps	%ymm11, 480(%rsp)
	vmovaps	%ymm7, 512(%rsp)
	vmovaps	%ymm9, %ymm6
	vmovaps	%ymm5, 1888(%rsp)
	vmovaps	%ymm13, %ymm11
	movq	%r9, 544(%rsp)
	vmovaps	224(%rsp), %ymm10
	vmovaps	192(%rsp), %ymm9
	xorl	%esi, %esi
	movq	416(%rsp), %rdi
	movq	256(%rsp), %rcx
	vmovaps	3392(%rsp), %ymm14
	xorl	%r13d, %r13d
	vmovaps	3360(%rsp), %ymm15
	vmovaps	3328(%rsp), %ymm3
	vbroadcastss	.LCPI60_2(%rip), %ymm13
	vmovaps	2080(%rsp), %ymm1
	movq	344(%rsp), %r14
	jmp	.LBB60_172
	.p2align	4
.LBB60_170:
	vmovaps	224(%rsp), %ymm5
	vmovaps	192(%rsp), %ymm0
.LBB60_171:
	movq	384(%rsp), %rsi
.Ltmp8369:
	addq	%r14, %rsi
	movq	256(%rsp), %rcx
.Ltmp8370:
	.loc	15 3227 39 is_stmt 1
	addq	%r14, %rcx
.Ltmp8371:
	.loc	15 1043 8
	cmpq	%r15, %rcx
	movq	%r15, %rax
	movl	$0, %r13d
	cmovbq	%r13, %rax
	subq	%rax, %rcx
	movq	416(%rsp), %rdi
.Ltmp8372:
	.loc	15 3228 39
	addq	%r14, %rdi
	movq	720(%rsp), %rax
.Ltmp8373:
	.loc	15 1043 8
	cmpq	%rax, %rdi
	cmovbq	%r13, %rax
	subq	%rax, %rdi
	vmovaps	%ymm5, 224(%rsp)
	vmovaps	%ymm0, 192(%rsp)
	movq	344(%rsp), %r14
.Ltmp8374:
	.loc	15 3173 19
	cmpq	%r14, %rsi
	movq	56(%rsp), %r12
	jae	.LBB60_192
.LBB60_172:
	.loc	15 0 19 is_stmt 0
	movq	%rsi, 384(%rsp)
	.loc	15 3181 21 is_stmt 1
	subq	%rsi, %r14
.Ltmp8375:
	.loc	15 1470 16
	movq	1624(%r12), %r8
.Ltmp8376:
	.loc	15 1471 16
	movq	1632(%r12), %rbx
.Ltmp8377:
	.loc	15 1472 25
	leaq	1(%rcx), %rax
.Ltmp8378:
	.loc	15 1043 8
	cmpq	%r8, %rax
	movq	%rcx, %r12
	movq	%r8, %rcx
	cmovbq	%r13, %rcx
	negq	%rcx
	movq	744(%rsp), %rax
.Ltmp8379:
	.loc	15 1473 28
	addq	%r12, %rax
.Ltmp8380:
	.loc	15 1043 8
	cmpq	%r8, %rax
	movq	%r8, %rsi
	cmovbq	%r13, %rsi
	subq	%rsi, %rax
	movq	736(%rsp), %rsi
.Ltmp8381:
	.loc	15 1475 33
	leaq	(%r12,%rsi), %r9
.Ltmp8382:
	.loc	15 1043 8
	cmpq	%r8, %r9
	movq	%r8, %rsi
	cmovbq	%r13, %rsi
	subq	%rsi, %r9
.Ltmp8383:
	.loc	15 1478 14
	movq	%r8, %r13
	subq	%r12, %r13
.Ltmp8384:
	.loc	10 1078 5
	cmpq	%r14, %r13
	cmovbq	%r13, %r14
	movq	%rdi, 416(%rsp)
.Ltmp8385:
	.loc	15 1479 14
	subq	%rdi, %rbx
.Ltmp8386:
	.loc	10 1078 5
	cmpq	%r14, %rbx
	cmovbq	%rbx, %r14
	movq	%r12, 256(%rsp)
.Ltmp8387:
	.loc	15 1043 8
	leaq	(%r12,%rcx), %rsi
	incq	%rsi
.Ltmp8388:
	.loc	15 1480 14
	movq	%r8, %r12
	movq	%rsi, 576(%rsp)
	subq	%rsi, %r12
.Ltmp8389:
	.loc	10 1078 5
	cmpq	%r14, %r12
	cmovbq	%r12, %r14
.Ltmp8390:
	.loc	15 1481 14
	movq	%r8, %rcx
	subq	%rax, %rcx
.Ltmp8391:
	.loc	10 1078 5
	cmpq	%r14, %rcx
	cmovbq	%rcx, %r14
.Ltmp8392:
	.loc	15 1483 14
	subq	%r9, %r8
.Ltmp8393:
	.loc	10 1078 5
	cmpq	%r14, %r8
	cmovbq	%r8, %r14
	movq	112(%rsp), %rsi
	movq	384(%rsp), %rdi
.Ltmp8394:
	.loc	15 3187 28
	addq	%rsi, %rdi
.Ltmp8395:
	.loc	15 3189 55
	leaq	(%r14,%rdi), %rsi
.Ltmp8396:
	.loc	15 3187 28
	shlq	$3, %rdi
.Ltmp8397:
	.loc	15 3189 55
	shlq	$3, %rsi
.Ltmp8398:
	.loc	14 1050 16
	cmpq	%rdi, %rsi
	jb	.LBB60_254
	cmpq	96(%rsp), %rsi
	ja	.LBB60_254
.Ltmp8399:
	.loc	32 304 12
	testq	%r14, %r14
	je	.LBB60_170
.Ltmp8400:
	.loc	32 0 12 is_stmt 0
	movq	40(%rsp), %rsi
	leaq	(%rsi,%rdi,4), %rsi
	movq	%rsi, 128(%rsp)
	movq	384(%rsp), %rdi
.Ltmp8401:
	movq	%rdi, %rsi
	shlq	$5, %rsi
.Ltmp8402:
	.loc	32 304 12
	cmpq	%r8, %rcx
	cmovbq	%rcx, %r8
.Ltmp8403:
	.loc	15 0 0
	leaq	(%rsp,%rsi), %rcx
	addq	$3904, %rcx
	movq	%rcx, 352(%rsp)
.Ltmp8404:
	.loc	32 304 12
	cmpq	%r12, %r8
	cmovaeq	%r12, %r8
	cmpq	%r13, %r8
	cmovaeq	%r13, %r8
	cmpq	%rbx, %r8
	cmovaeq	%rbx, %r8
	movq	728(%rsp), %rbx
	subq	%rdi, %rbx
	cmpq	%rbx, %r8
	cmovbq	%r8, %rbx
	andq	336(%rsp), %rbx
	vmovaps	%ymm10, %ymm5
	vmovaps	%ymm9, %ymm0
	xorl	%r13d, %r13d
	vmovaps	%ymm1, %ymm2
.Ltmp8405:
	.loc	32 0 12
.Ltmp8406:
	.p2align	4
.LBB60_176:
	.loc	15 3197 57 is_stmt 1
	cmpb	$0, 8(%rsp)
	movq	256(%rsp), %rcx
	jne	.LBB60_178
	.loc	15 0 57 is_stmt 0
	vbroadcastss	.LCPI60_1(%rip), %ymm13
.Ltmp8407:
	.loc	50 347 14 is_stmt 1
	vaddps	%ymm13, %ymm15, %ymm9
	vxorps	%xmm1, %xmm1, %xmm1
.Ltmp8408:
	.loc	50 233 14
	vmaxps	%ymm1, %ymm9, %ymm15
.Ltmp8409:
	.loc	50 871 14
	vcmpgt_oqps	%ymm1, %ymm15, %ymm9
.Ltmp8410:
	.loc	50 48 14
	vaddps	%ymm0, %ymm5, %ymm0
	vmovaps	%ymm3, %ymm10
	vmovaps	3488(%rsp), %ymm3
.Ltmp8411:
	.loc	50 585 19
	vblendvps	%ymm9, %ymm0, %ymm3, %ymm0
.Ltmp8412:
	.loc	50 585 19 is_stmt 0
	vblendvps	%ymm9, %ymm5, %ymm1, %ymm5
.Ltmp8413:
	.loc	50 347 14 is_stmt 1
	vaddps	640(%rsp), %ymm13, %ymm9
.Ltmp8414:
	.loc	50 233 14
	vmaxps	%ymm1, %ymm9, %ymm3
	vmovaps	%ymm3, 640(%rsp)
.Ltmp8415:
	.loc	50 871 14
	vcmpgt_oqps	%ymm1, %ymm3, %ymm9
	vmovaps	608(%rsp), %ymm13
.Ltmp8416:
	.loc	50 48 14
	vaddps	%ymm10, %ymm13, %ymm10
	vmovaps	3456(%rsp), %ymm3
.Ltmp8417:
	.loc	50 585 19
	vblendvps	%ymm9, %ymm10, %ymm3, %ymm3
.Ltmp8418:
	.loc	50 585 19 is_stmt 0
	vblendvps	%ymm9, %ymm13, %ymm1, %ymm13
	vmovaps	%ymm13, 608(%rsp)
	vbroadcastss	.LCPI60_2(%rip), %ymm13
.Ltmp8419:
.LBB60_178:
	.loc	15 1397 26 is_stmt 1
	addq	%r13, %rcx
.Ltmp8420:
	.loc	15 1032 16
	leaq	(,%rcx,8), %rdi
.Ltmp8421:
	.loc	15 1033 33
	leaq	8(,%rcx,8), %rsi
.Ltmp8422:
	.loc	14 1050 16
	leaq	7(,%rcx,8), %rcx
	cmpq	%rdx, %rcx
	jae	.LBB60_244
.Ltmp8423:
	.loc	15 0 0 is_stmt 0
	leaq	(,%r13,8), %rcx
	movq	352(%rsp), %r8
.Ltmp8424:
	.loc	1 551 14 is_stmt 1
	vmovups	(%r8,%rcx,4), %ymm9
.Ltmp8425:
	.loc	50 233 14
	vmaxps	%ymm9, %ymm9, %ymm10
.Ltmp8426:
	.loc	50 585 19
	vblendvps	%ymm14, %ymm10, %ymm9, %ymm9
.Ltmp8427:
	.loc	50 360 14
	vdivps	%ymm9, %ymm0, %ymm10
.Ltmp8428:
	.loc	15 0 0 is_stmt 0
	leaq	(%r13,%rax), %r12
.Ltmp8429:
	.loc	50 871 14 is_stmt 1
	vcmpgt_oqps	%ymm0, %ymm9, %ymm9
.Ltmp8430:
	.loc	50 585 19
	vblendvps	%ymm9, %ymm10, %ymm13, %ymm9
.Ltmp8431:
	.loc	1 551 14
	vmovups	%ymm9, (%r10,%rdi,4)
.Ltmp8432:
	.loc	15 1025 16
	leaq	(,%r12,8), %r8
.Ltmp8433:
	.loc	14 1050 16
	leaq	7(,%r12,8), %rcx
	cmpq	%rdx, %rcx
	jae	.LBB60_193
.Ltmp8434:
	.loc	1 551 14
	vmovups	(%r10,%r8,4), %ymm9
	vmovaps	%ymm9, %ymm1
.Ltmp8435:
	.loc	15 1101 22
	testl	%r11d, %r11d
	je	.LBB60_182
.Ltmp8436:
	.loc	50 257 14
	vminps	%ymm9, %ymm2, %ymm1
.Ltmp8437:
.LBB60_182:
	.loc	15 0 0 is_stmt 0
	movl	%r11d, %r11d
.Ltmp8438:
	.loc	15 1107 20 is_stmt 1
	incq	%r11
	movq	1856(%rsp), %r8
	movq	%r8, %rcx
	cmpq	%r8, %r11
.Ltmp8439:
	.loc	15 1108 22
	jne	.LBB60_186
	.loc	15 0 22 is_stmt 0
.Ltmp8440:
	.p2align	4
.LBB60_183:
.Ltmp8441:
	.loc	15 1025 16 is_stmt 1
	leaq	(,%r12,8), %r8
.Ltmp8442:
	.loc	14 1050 16
	leaq	7(,%r12,8), %r11
	cmpq	%rdx, %r11
	jae	.LBB60_193
.Ltmp8443:
	.loc	50 257 14
	vminps	(%r10,%r8,4), %ymm9, %ymm9
.Ltmp8444:
	.loc	1 551 14
	vmovups	%ymm9, (%r10,%r8,4)
.Ltmp8445:
	.loc	15 1119 16
	testq	%r12, %r12
	cmoveq	%r15, %r12
	.loc	15 1122 13
	decq	%r12
.Ltmp8446:
	.loc	10 1916 50
	decq	%rcx
.Ltmp8447:
	.loc	11 900 12
	jne	.LBB60_183
.Ltmp8448:
	.loc	11 0 12 is_stmt 0
	xorl	%r11d, %r11d
	vmovaps	%ymm1, %ymm2
	jmp	.LBB60_188
	.p2align	4
.LBB60_186:
	movq	576(%rsp), %rcx
	leaq	(%rcx,%r13), %r8
.Ltmp8449:
	.loc	14 1050 16 is_stmt 1
	leaq	7(,%r8,8), %rcx
.Ltmp8450:
	.loc	15 1025 16
	shlq	$3, %r8
.Ltmp8451:
	.loc	14 1050 16
	cmpq	%rdx, %rcx
	jae	.LBB60_193
.Ltmp8452:
	.loc	1 551 14
	vmovups	(%r10,%r8,4), %ymm2
.Ltmp8453:
	.loc	50 257 14
	vminps	%ymm1, %ymm2, %ymm2
.Ltmp8454:
.LBB60_188:
	.loc	15 0 0 is_stmt 0
	leaq	(%r9,%r13), %r8
.Ltmp8455:
	.loc	14 1050 16 is_stmt 1
	leaq	7(,%r8,8), %rcx
.Ltmp8456:
	.loc	15 1025 16
	shlq	$3, %r8
	movq	48(%rsp), %r12
.Ltmp8457:
	.loc	14 1050 16
	cmpq	%r12, %rcx
	movq	%r12, %rcx
	jae	.LBB60_245
.Ltmp8458:
	.loc	14 0 16 is_stmt 0
	cmpq	%rcx, %rsi
.Ltmp8459:
	.loc	14 1050 16
	ja	.LBB60_246
.Ltmp8460:
	.loc	15 0 0
	vbroadcastss	.LCPI60_3(%rip), %ymm9
	vmulps	%ymm2, %ymm9, %ymm2
	vroundps	$9, %ymm2, %ymm2
	vbroadcastss	.LCPI60_4(%rip), %ymm9
	vmulps	%ymm2, %ymm9, %ymm2
.Ltmp8461:
	vaddps	64(%rsp), %ymm2, %ymm9
	movq	104(%rsp), %rsi
	vsubps	(%rsi,%r8,4), %ymm9, %ymm9
	movq	416(%rsp), %rcx
.Ltmp8462:
	leaq	(%r13,%rcx), %r8
	vmovaps	%ymm9, 64(%rsp)
.Ltmp8463:
	.loc	50 360 14 is_stmt 1
	vdivps	2048(%rsp), %ymm9, %ymm9
.Ltmp8464:
	.loc	1 551 14
	vmovups	%ymm2, (%rsi,%rdi,4)
.Ltmp8465:
	.loc	15 1554 43
	vmovaps	1600(%rsp), %ymm2
.Ltmp8466:
	.loc	50 347 14
	vsubps	%ymm9, %ymm13, %ymm9
.Ltmp8467:
	.loc	50 347 14 is_stmt 0
	vsubps	%ymm2, %ymm9, %ymm10
.Ltmp8468:
	.loc	50 283 14 is_stmt 1
	vmulps	%ymm3, %ymm10, %ymm10
.Ltmp8469:
	.loc	50 48 14
	vaddps	%ymm2, %ymm10, %ymm2
.Ltmp8470:
	.loc	50 82 19
	vbroadcastss	.LCPI60_0(%rip), %ymm10
.Ltmp8471:
	.loc	50 233 14
	vmaxps	%ymm2, %ymm9, %ymm2
.Ltmp8472:
	.loc	50 82 19
	vandps	%ymm2, %ymm10, %ymm9
.Ltmp8473:
	.loc	50 871 14
	vbroadcastss	.LCPI60_5(%rip), %ymm10
	vcmplt_oqps	%ymm10, %ymm9, %ymm9
.Ltmp8474:
	.loc	50 82 19
	vandnps	%ymm2, %ymm9, %ymm2
.Ltmp8475:
	.loc	15 1555 5
	vmovaps	%ymm2, 1600(%rsp)
.Ltmp8476:
	.loc	14 1050 16
	leaq	7(,%r8,8), %rcx
.Ltmp8477:
	.loc	15 1025 16
	shlq	$3, %r8
.Ltmp8478:
	.loc	14 1050 16
	cmpq	1824(%rsp), %rcx
	jae	.LBB60_247
.Ltmp8479:
	.loc	15 0 0 is_stmt 0
	leaq	1(%r13), %rsi
	shlq	$5, %r13
	addq	128(%rsp), %r13
.Ltmp8480:
	.loc	50 347 14 is_stmt 1
	vsubps	%ymm2, %ymm13, %ymm2
	movq	1792(%rsp), %rcx
.Ltmp8481:
	.loc	1 551 14
	vmovups	(%rcx,%r8,4), %ymm9
.Ltmp8482:
	.loc	1 551 14 is_stmt 0
	vmovups	(%r13), %ymm10
	vmovups	%ymm10, (%rcx,%r8,4)
.Ltmp8483:
	.loc	50 283 14 is_stmt 1
	vmulps	%ymm2, %ymm9, %ymm2
.Ltmp8484:
	.loc	50 585 19
	vblendvps	%ymm4, %ymm9, %ymm2, %ymm2
.Ltmp8485:
	.loc	1 551 14
	vmovups	%ymm2, (%r13)
	movq	%rsi, %r13
	vmovaps	%ymm1, %ymm2
	vmovaps	%ymm5, %ymm10
	vmovaps	%ymm0, %ymm9
.Ltmp8486:
	.loc	32 304 12
	cmpq	%rbx, %rsi
	jne	.LBB60_176
	jmp	.LBB60_171
.Ltmp8487:
.LBB60_192:
	.loc	32 0 12 is_stmt 0
	vmovaps	%ymm1, 2080(%rsp)
	movq	%rcx, 256(%rsp)
.Ltmp8488:
	vmovaps	%ymm0, 1344(%rsp)
.Ltmp8489:
	vmovaps	%ymm5, 1408(%rsp)
	vmovaps	640(%rsp), %ymm2
.Ltmp8490:
	.loc	15 746 9 is_stmt 1
	vmovaps	%ymm2, 1568(%rsp)
	vmovaps	%ymm3, 3328(%rsp)
.Ltmp8491:
	.loc	15 748 9
	vmovaps	%ymm3, 1472(%rsp)
	vmovaps	608(%rsp), %ymm2
	.loc	15 749 9
	vmovaps	%ymm2, 1536(%rsp)
	vmovaps	64(%rsp), %ymm2
.Ltmp8492:
	.loc	15 1549 5
	vmovaps	%ymm2, 1632(%rsp)
	vmovaps	%ymm15, 1440(%rsp)
	vmovaps	%ymm5, 224(%rsp)
	vmovaps	%ymm0, 192(%rsp)
	movq	96(%rsp), %r13
	movq	40(%rsp), %r14
	movq	24(%rsp), %rcx
	movq	544(%rsp), %r9
	vmovaps	%ymm11, %ymm13
	vmovaps	1888(%rsp), %ymm5
	vmovaps	%ymm6, %ymm9
	vmovaps	512(%rsp), %ymm7
	vmovaps	480(%rsp), %ymm11
	vmovaps	672(%rsp), %ymm6
	vmovaps	160(%rsp), %ymm1
	movq	112(%rsp), %rbx
	vmovaps	448(%rsp), %ymm2
	vmovaps	1760(%rsp), %ymm10
	jmp	.LBB60_168
.Ltmp8493:
.LBB60_193:
	.loc	15 1618 5
	vmovaps	%ymm6, 960(%rsp)
	vmovaps	%ymm11, 992(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp8494:
	.loc	15 0 0 is_stmt 0
	vmovaps	%ymm0, 1344(%rsp)
	vmovaps	224(%rsp), %ymm0
.Ltmp8495:
	vmovaps	%ymm0, 1408(%rsp)
.Ltmp8496:
	.loc	15 1026 25 is_stmt 1
	leaq	8(%r8), %rsi
.Ltmp8497:
	.loc	15 0 0 is_stmt 0
	leaq	.Lalloc_296a0a227063f9d4f193ea1ae436593b(%rip), %rcx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8498:
.LBB60_194:
	.loc	15 3232 31 is_stmt 1
	movl	936(%rsp), %r11d
	movq	40(%rsp), %r14
	movq	%rbx, %rdi
.LBB60_195:
	.loc	15 3232 10
	vmovaps	832(%rsp), %ymm0
	vmovaps	%ymm0, 3520(%rsp)
.Ltmp8499:
	.loc	15 3235 23
	movq	1736(%r12), %rdx
.Ltmp8500:
	.loc	37 451 16
	cmpq	$7, %rdx
	jbe	.LBB60_152
.Ltmp8501:
	.loc	37 0 16 is_stmt 0
	movq	%rdi, %rbx
	.loc	15 3235 23 is_stmt 1
	movq	1728(%r12), %rax
.Ltmp8502:
	.loc	1 551 14
	vmovaps	3520(%rsp), %ymm0
	vmovups	%ymm0, (%rax)
.Ltmp8503:
	.loc	15 3236 5
	movq	1768(%r12), %rax
.Ltmp8504:
	.loc	16 1714 9
	testq	%rax, %rax
.Ltmp8505:
	.loc	17 180 28
	je	.LBB60_199
.Ltmp8506:
	.loc	17 0 28 is_stmt 0
	movq	1760(%r12), %rcx
	shlq	$2, %rax
	xorl	%edx, %edx
	.p2align	4
.LBB60_198:
.Ltmp8507:
	.loc	31 66 21 is_stmt 1
	movl	%r11d, (%rcx,%rdx)
.Ltmp8508:
	.loc	16 1714 9
	addq	$4, %rdx
	cmpq	%rdx, %rax
.Ltmp8509:
	.loc	17 180 28
	jne	.LBB60_198
.Ltmp8510:
.LBB60_199:
	.loc	17 0 28 is_stmt 0
	leaq	960(%rsp), %rdi
	movq	16(%rsp), %rsi
	.loc	15 3238 14 is_stmt 1
	vzeroupper
	callq	_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_
	.loc	15 3239 5
	movl	%ebx, 1640(%r12)
	movq	256(%rsp), %rax
.Ltmp8511:
	.loc	15 0 0 is_stmt 0
	movl	%eax, 1644(%r12)
.Ltmp8512:
	.loc	15 2194 13 is_stmt 1
	cmpb	$0, 124(%rsp)
	je	.LBB60_208
.LBB60_200:
	.loc	15 0 13 is_stmt 0
	movq	16(%rsp), %rdi
	.loc	15 2194 32
	callq	*_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest@GOTPCREL(%rip)
	.loc	15 2194 22
	testb	%al, %al
	je	.LBB60_208
	.loc	15 0 22
	movq	752(%rsp), %rsi
	cmpq	%r13, %rsi
.Ltmp8513:
	.loc	14 1050 16 is_stmt 1
	ja	.LBB60_262
.Ltmp8514:
	.loc	14 0 16 is_stmt 0
	movq	%r14, %rax
	.p2align	4
.LBB60_203:
.Ltmp8515:
	.loc	34 131 18 is_stmt 1
	movq	%rsi, %rcx
.Ltmp8516:
	.loc	29 1504 12
	testq	%rsi, %rsi
	je	.LBB60_207
.Ltmp8517:
	.loc	10 1078 5
	cmpq	$32, %rcx
	movl	$32, %edx
	cmovbq	%rcx, %rdx
.Ltmp8518:
	.loc	12 961 18
	leal	(,%rdx,4), %edi
	xorl	%r9d, %r9d
	xorl	%r8d, %r8d
.Ltmp8519:
	.loc	12 0 18 is_stmt 0
.Ltmp8520:
	.p2align	4
.LBB60_205:
	.loc	34 134 13 is_stmt 1
	orl	(%rax,%r8), %r9d
.Ltmp8521:
	.loc	16 1714 9
	addq	$4, %r8
	cmpq	%r8, %rdi
.Ltmp8522:
	.loc	17 180 28
	jne	.LBB60_205
.Ltmp8523:
	.loc	35 863 18
	leaq	(%rax,%rdx,4), %rax
.Ltmp8524:
	.loc	30 2054 74
	movq	%rcx, %rsi
	subq	%rdx, %rsi
.Ltmp8525:
	.loc	34 136 12
	testl	%r9d, %r9d
	je	.LBB60_203
.Ltmp8526:
.LBB60_207:
	.loc	29 1504 12
	testq	%rcx, %rcx
	sete	%al
	jmp	.LBB60_209
.Ltmp8527:
.LBB60_208:
	.loc	29 0 12 is_stmt 0
	xorl	%eax, %eax
.LBB60_209:
	.loc	15 2193 9 is_stmt 1
	movb	%al, 2152(%r12)
	.loc	15 2195 30
	movzbl	2144(%r12), %eax
	.loc	15 2195 9 is_stmt 0
	movb	%al, 2153(%r12)
.Ltmp8528:
	.loc	50 871 14 is_stmt 1
	vxorps	%xmm0, %xmm0, %xmm0
	vcmpeqps	%ymm0, %ymm0, %ymm0
	movabsq	$2305843009213693944, %rax
.Ltmp8529:
	.loc	30 2155 12
	andq	%r13, %rax
	je	.LBB60_215
.Ltmp8530:
	.loc	30 0 12 is_stmt 0
	vbroadcastss	.LCPI60_0(%rip), %ymm1
	xorl	%ecx, %ecx
	vbroadcastss	.LCPI60_6(%rip), %ymm2
	vmovaps	%ymm0, %ymm3
	.p2align	4
.LBB60_211:
.Ltmp8531:
	.loc	50 82 19 is_stmt 1
	vandps	(%r14,%rcx,4), %ymm1, %ymm4
.Ltmp8532:
	.loc	50 871 14
	vcmplt_oqps	%ymm2, %ymm4, %ymm4
.Ltmp8533:
	.loc	50 82 19
	vandps	%ymm4, %ymm3, %ymm3
.Ltmp8534:
	.loc	30 2155 12
	addq	$8, %rcx
	cmpq	%rcx, %rax
	jne	.LBB60_211
.Ltmp8535:
	.loc	51 176 9
	vpcmpeqd	%ymm4, %ymm4, %ymm4
	vtestps	%ymm4, %ymm3
.Ltmp8536:
	.loc	15 2196 12
	jb	.LBB60_219
	.loc	15 0 12 is_stmt 0
	xorl	%ecx, %ecx
	.p2align	4
.LBB60_214:
.Ltmp8537:
	.loc	50 82 19 is_stmt 1
	vandps	(%r14,%rcx,4), %ymm1, %ymm3
.Ltmp8538:
	.loc	50 871 14
	vcmplt_oqps	%ymm2, %ymm3, %ymm3
.Ltmp8539:
	.loc	50 82 19
	vandps	%ymm3, %ymm0, %ymm0
.Ltmp8540:
	.loc	30 2155 12
	addq	$8, %rcx
	cmpq	%rcx, %rax
	jne	.LBB60_214
	jmp	.LBB60_216
.Ltmp8541:
.LBB60_215:
	.loc	51 176 9
	vpcmpeqd	%ymm1, %ymm1, %ymm1
	vtestps	%ymm1, %ymm0
.Ltmp8542:
	.loc	15 2196 12
	jb	.LBB60_219
.LBB60_216:
	.loc	15 0 12 is_stmt 0
	leaq	1616(%r12), %rbx
.Ltmp8543:
	.loc	50 585 19 is_stmt 1
	vpbroadcastd	.LCPI60_2(%rip), %ymm1
	vpsrad	$31, %ymm0, %ymm0
	vpandn	%ymm1, %ymm0, %ymm0
.Ltmp8544:
	.loc	34 185 12
	vmovd	%xmm0, %eax
	xorl	%ecx, %ecx
	testl	%eax, %eax
	setne	%cl
	vpextrd	$1, %xmm0, %eax
	xorl	%edx, %edx
	testl	%eax, %eax
	setne	%dl
	leal	(%rcx,%rdx,2), %eax
	vpextrd	$2, %xmm0, %ecx
	xorl	%edx, %edx
	testl	%ecx, %ecx
	setne	%dl
	leal	(%rax,%rdx,4), %eax
	vpextrd	$3, %xmm0, %ecx
	xorl	%edx, %edx
	testl	%ecx, %ecx
	setne	%dl
	leal	(%rax,%rdx,8), %ecx
	vextracti128	$1, %ymm0, %xmm0
	vmovd	%xmm0, %edx
	xorl	%eax, %eax
	testl	%edx, %edx
	setne	%al
	shll	$4, %eax
	vpextrd	$1, %xmm0, %edx
	orl	%ecx, %eax
	xorl	%ecx, %ecx
	testl	%edx, %edx
	setne	%cl
	shll	$5, %ecx
	vpextrd	$2, %xmm0, %edx
	xorl	%esi, %esi
	testl	%edx, %edx
	setne	%sil
	shll	$6, %esi
	orl	%ecx, %esi
	vpextrd	$3, %xmm0, %ecx
	xorl	%edx, %edx
	testl	%ecx, %ecx
	setne	%dl
	shll	$7, %edx
	orl	%esi, %edx
	orl	%eax, %edx
.Ltmp8545:
	.loc	15 2199 9
	movl	%edx, 1576(%r12)
	.loc	15 2200 40
	movq	1568(%r12), %rax
.Ltmp8546:
	.loc	14 2428 13
	incq	%rax
	movq	$-1, %rcx
	cmovneq	%rax, %rcx
.Ltmp8547:
	.loc	15 2200 9
	movq	%rcx, 1568(%r12)
.Ltmp8548:
	.loc	16 1714 9
	testq	%r13, %r13
.Ltmp8549:
	.loc	17 180 28
	je	.LBB60_218
.Ltmp8550:
	.loc	12 961 18
	shlq	$2, %r13
.Ltmp8551:
	.loc	31 25 13
	movq	%r14, %rdi
	xorl	%esi, %esi
	movq	%r13, %rdx
	vzeroupper
	callq	*memset@GOTPCREL(%rip)
.Ltmp8552:
.LBB60_218:
	.loc	15 2202 21
	movq	16(%rbx), %rax
	movq	%rax, 3920(%rsp)
	vmovups	(%rbx), %xmm0
	vmovaps	%xmm0, 3904(%rsp)
.Ltmp8553:
	.loc	15 2203 20
	movl	2120(%r12), %ebx
.Ltmp8554:
	.loc	15 2205 40
	movq	1584(%r12), %rdx
	movq	1592(%r12), %rcx
	.loc	15 2205 14 is_stmt 0
	movq	_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults@GOTPCREL(%rip), %r14
	leaq	3904(%rsp), %r15
	movq	16(%rsp), %rdi
	movq	%r15, %rsi
	movl	%ebx, %r8d
	vzeroupper
	callq	*%r14
	.loc	15 2207 40 is_stmt 1
	movq	1600(%r12), %rdx
	movq	1608(%r12), %rcx
	movq	32(%rsp), %rdi
	.loc	15 2207 14 is_stmt 0
	movq	%r15, %rsi
	movl	%ebx, %r8d
	callq	*%r14
	.loc	15 2208 9 is_stmt 1
	movq	$0, 1640(%r12)
.Ltmp8555:
.LBB60_219:
	.loc	15 0 9 is_stmt 0
	leaq	3552(%rsp), %rsi
	.loc	15 3005 9 is_stmt 1
	movl	$328, %edx
	movq	760(%rsp), %rbx
	movq	%rbx, %rdi
	vzeroupper
	callq	*memcpy@GOTPCREL(%rip)
.Ltmp8556:
	.loc	15 2932 6
	movq	%rbx, %rax
	leaq	-40(%rbp), %rsp
	.loc	15 2932 6 epilogue_begin is_stmt 0
	popq	%rbx
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	.cfi_def_cfa %rsp, 8
	retq
.LBB60_220:
	.cfi_def_cfa %rbp, 16
	.loc	15 0 6
	movb	$1, %al
.Ltmp8557:
	.loc	15 2176 12 is_stmt 1
	cmpb	$0, 2152(%r12)
	je	.LBB60_43
.Ltmp8558:
	.loc	15 660 31
	movq	1768(%r12), %rcx
	.loc	15 660 57 is_stmt 0
	movq	1832(%r12), %rax
.Ltmp8559:
	.loc	10 1078 5 is_stmt 1
	cmpq	%rcx, %rax
	cmovbq	%rax, %rcx
.Ltmp8560:
	.loc	32 304 12
	testq	%rcx, %rcx
	je	.LBB60_228
.Ltmp8561:
	.loc	32 0 12 is_stmt 0
	movq	1760(%r12), %rsi
	movq	1824(%r12), %rdi
	xorl	%r8d, %r8d
	jmp	.LBB60_225
	.p2align	4
.LBB60_223:
.Ltmp8562:
	.loc	15 662 22 is_stmt 1
	xorl	%edx, %edx
	divq	%r9
.LBB60_224:
	.loc	15 662 13 is_stmt 0
	movl	%edx, (%rsi,%r8,4)
.Ltmp8563:
	.loc	15 0 0
	incq	%r8
.Ltmp8564:
	.loc	32 304 12 is_stmt 1
	addq	$12, %rdi
	cmpq	%r8, %rcx
	je	.LBB60_228
.Ltmp8565:
.LBB60_225:
	.loc	15 661 26
	movl	(%rdi), %r9d
	testq	%r9, %r9
.Ltmp8566:
	.loc	15 662 42
	je	.LBB60_278
	.loc	15 662 24 is_stmt 0
	movl	(%rsi,%r8,4), %r10d
	movq	24(%rsp), %rax
	.loc	15 662 42
	xorl	%edx, %edx
	divl	%r9d
	movl	%edx, %eax
	.loc	15 662 23
	addq	%r10, %rax
	.loc	15 662 22
	btq	$32, %rax
	jb	.LBB60_223
	xorl	%edx, %edx
	divl	%r9d
	jmp	.LBB60_224
.Ltmp8567:
.LBB60_228:
	.loc	15 2178 26 is_stmt 1
	movq	1632(%r12), %rsi
.Ltmp8568:
	.loc	15 452 44
	testq	%rsi, %rsi
	je	.LBB60_283
.Ltmp8569:
	.loc	15 2178 26
	movq	1624(%r12), %rcx
.Ltmp8570:
	.loc	15 452 23
	movl	1640(%r12), %edi
	movq	24(%rsp), %rdx
	movq	%rdx, %rax
	.loc	15 452 44 is_stmt 0
	cmpq	%rsi, %rdx
	jb	.LBB60_231
	.loc	15 0 44
	movq	24(%rsp), %rax
	.loc	15 452 44
	xorl	%edx, %edx
	divl	%esi
	movl	%edx, %eax
.LBB60_231:
	.loc	15 452 22
	addq	%rdi, %rax
	.loc	15 452 21
	movq	%rax, %rdx
	orq	%rsi, %rdx
	shrq	$32, %rdx
	je	.LBB60_237
	xorl	%edx, %edx
	divq	%rsi
	jmp	.LBB60_238
.Ltmp8571:
.LBB60_233:
	.loc	15 749 9 is_stmt 1
	vmovaps	%ymm11, 1536(%rsp)
.Ltmp8572:
	.loc	15 1306 5
	vmovaps	%ymm10, 1632(%rsp)
.Ltmp8573:
	.loc	37 569 13
	leaq	.Lalloc_903a4f70ab87a036c36d8512c06ab29a(%rip), %rcx
.Ltmp8574:
	.loc	15 0 0 is_stmt 0
	movq	%r13, %rsi
	movq	%r13, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB60_234:
.Ltmp8575:
	.loc	15 749 9 is_stmt 1
	vmovaps	%ymm11, 1536(%rsp)
.Ltmp8576:
	.loc	15 1306 5
	vmovaps	%ymm10, 1632(%rsp)
.Ltmp8577:
	.loc	37 581 13
	leaq	.Lalloc_9d2713d1692431af37d60082290049ed(%rip), %rcx
	movq	%r9, %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8578:
.LBB60_235:
	.loc	15 749 9
	vmovaps	%ymm11, 1536(%rsp)
.Ltmp8579:
	.loc	15 1306 5
	vmovaps	%ymm10, 1632(%rsp)
.Ltmp8580:
	.loc	37 581 13
	leaq	.Lalloc_8002ed69501742f3ea2ea25eb68cd581(%rip), %rcx
	movq	352(%rsp), %rdi
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8581:
.LBB60_236:
	.loc	15 749 9
	vmovaps	%ymm11, 1536(%rsp)
.Ltmp8582:
	.loc	15 1306 5
	vmovaps	%ymm10, 1632(%rsp)
.Ltmp8583:
	.loc	37 569 13
	leaq	.Lalloc_a07d19b424a92543209d516ece5bdf2a(%rip), %rcx
	movq	%rsi, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8584:
.LBB60_237:
	.loc	15 452 21
	xorl	%edx, %edx
	divl	%esi
.LBB60_238:
	.loc	15 452 9 is_stmt 0
	movl	%edx, 1640(%r12)
	.loc	15 453 44 is_stmt 1
	testq	%rcx, %rcx
	je	.LBB60_284
	.loc	15 453 23 is_stmt 0
	movl	1644(%r12), %esi
	movq	24(%rsp), %rdx
	.loc	15 453 44
	cmpq	%rcx, %rdx
	jb	.LBB60_241
	movl	%edx, %eax
	xorl	%edx, %edx
	divl	%ecx
.LBB60_241:
	.loc	15 453 22
	addq	%rsi, %rdx
	.loc	15 453 21
	movq	%rdx, %rax
	orq	%rcx, %rax
	shrq	$32, %rax
	je	.LBB60_243
	movq	%rdx, %rax
	xorl	%edx, %edx
	divq	%rcx
	.loc	15 453 9
	movl	%edx, 1644(%r12)
.Ltmp8585:
	.loc	15 0 0
	jmp	.LBB60_219
.LBB60_243:
.Ltmp8586:
	.loc	15 453 21
	movl	%edx, %eax
	xorl	%edx, %edx
	divl	%ecx
	.loc	15 453 9
	movl	%edx, 1644(%r12)
.Ltmp8587:
	.loc	15 0 0
	jmp	.LBB60_219
.LBB60_244:
.Ltmp8588:
	.loc	15 1618 5 is_stmt 1
	vmovaps	%ymm6, 960(%rsp)
	vmovaps	%ymm11, 992(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp8589:
	.loc	15 0 0 is_stmt 0
	vmovaps	%ymm0, 1344(%rsp)
	vmovaps	224(%rsp), %ymm0
.Ltmp8590:
	vmovaps	%ymm0, 1408(%rsp)
.Ltmp8591:
	.loc	37 456 13 is_stmt 1
	leaq	.Lalloc_cd0e502fea74c9eb9984d521d7f3533e(%rip), %rcx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8592:
.LBB60_245:
	.loc	15 1618 5
	vmovaps	%ymm6, 960(%rsp)
	vmovaps	%ymm11, 992(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp8593:
	.loc	15 0 0 is_stmt 0
	vmovaps	%ymm0, 1344(%rsp)
	vmovaps	224(%rsp), %ymm0
.Ltmp8594:
	vmovaps	%ymm0, 1408(%rsp)
.Ltmp8595:
	.loc	15 1026 25 is_stmt 1
	leaq	8(%r8), %rsi
	movq	%rcx, %rdx
.Ltmp8596:
	.loc	15 0 0 is_stmt 0
	leaq	.Lalloc_296a0a227063f9d4f193ea1ae436593b(%rip), %rcx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8597:
.LBB60_246:
	.loc	15 1618 5 is_stmt 1
	vmovaps	%ymm6, 960(%rsp)
	vmovaps	%ymm11, 992(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp8598:
	.loc	15 0 0 is_stmt 0
	vmovaps	%ymm0, 1344(%rsp)
	vmovaps	224(%rsp), %ymm0
.Ltmp8599:
	vmovaps	%ymm0, 1408(%rsp)
.Ltmp8600:
	.loc	37 456 13 is_stmt 1
	leaq	.Lalloc_cd0e502fea74c9eb9984d521d7f3533e(%rip), %rcx
	movq	48(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8601:
.LBB60_247:
	.loc	15 1618 5
	vmovaps	%ymm6, 960(%rsp)
	vmovaps	%ymm11, 992(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp8602:
	.loc	15 0 0 is_stmt 0
	vmovaps	%ymm0, 1344(%rsp)
	vmovaps	224(%rsp), %ymm0
.Ltmp8603:
	vmovaps	%ymm0, 1408(%rsp)
.Ltmp8604:
	.loc	15 1026 25 is_stmt 1
	leaq	8(%r8), %rsi
.Ltmp8605:
	.loc	37 443 13
	leaq	.Lalloc_296a0a227063f9d4f193ea1ae436593b(%rip), %rcx
	movq	%r8, %rdi
	movq	1824(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8606:
.LBB60_248:
	.loc	37 443 13
	leaq	.Lalloc_4ac05e629a5e1e4066705fcc17f0e09d(%rip), %rcx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8607:
.LBB60_249:
	.loc	15 1618 5
	vmovaps	%ymm9, 960(%rsp)
	vmovaps	%ymm13, 992(%rsp)
	vmovaps	2080(%rsp), %ymm0
	vmovaps	%ymm0, 832(%rsp)
	jmp	.LBB60_195
.Ltmp8608:
.LBB60_250:
	.loc	15 749 9
	vmovaps	%ymm11, 1536(%rsp)
.Ltmp8609:
	.loc	15 1306 5
	vmovaps	%ymm10, 1632(%rsp)
.Ltmp8610:
	.loc	15 1186 45
	leaq	.Lalloc_a19fa7d9077791c9eadbe74fafea8de7(%rip), %rdx
	movq	%r15, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8611:
.LBB60_251:
	.loc	15 0 45 is_stmt 0
	vmovaps	3264(%rsp), %ymm0
.Ltmp8612:
	.loc	15 1618 5 is_stmt 1
	vmovaps	%ymm0, 960(%rsp)
	vmovaps	3296(%rsp), %ymm0
	vmovaps	%ymm0, 992(%rsp)
.Ltmp8613:
.LBB60_252:
	.loc	15 0 0 is_stmt 0
	leaq	.Lalloc_e0592aef22128a0ac53753b9632a8183(%rip), %rcx
	movq	%r13, %rsi
	movq	%r13, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.LBB60_253:
	vmovaps	3264(%rsp), %ymm0
.Ltmp8614:
	.loc	15 1618 5 is_stmt 1
	vmovaps	%ymm0, 960(%rsp)
	vmovaps	3296(%rsp), %ymm0
	vmovaps	%ymm0, 992(%rsp)
.Ltmp8615:
	.loc	37 443 13
	leaq	.Lalloc_cae05af65618c8d83c932596374a1d80(%rip), %rcx
	movl	$8, %esi
	xorl	%edi, %edi
	movq	%r8, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8616:
.LBB60_254:
	.loc	15 1618 5
	vmovaps	%ymm6, 960(%rsp)
	vmovaps	%ymm11, 992(%rsp)
	vmovaps	192(%rsp), %ymm0
.Ltmp8617:
	.loc	15 0 0 is_stmt 0
	vmovaps	%ymm0, 1344(%rsp)
	vmovaps	224(%rsp), %ymm0
.Ltmp8618:
	vmovaps	%ymm0, 1408(%rsp)
.Ltmp8619:
	.loc	37 456 13 is_stmt 1
	leaq	.Lalloc_b0f5241d7ea6b85fff91278ccec58019(%rip), %rcx
	movq	96(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8620:
.LBB60_255:
	.loc	15 749 9
	vmovaps	%ymm11, 1536(%rsp)
.Ltmp8621:
	.loc	15 1306 5
	vmovaps	%ymm10, 1632(%rsp)
.Ltmp8622:
	.loc	15 1158 21
	leaq	.Lalloc_077370d5cece7380867993336836eb69(%rip), %rdx
	movq	160(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8623:
.LBB60_256:
	.loc	15 749 9
	vmovaps	%ymm11, 1536(%rsp)
.Ltmp8624:
	.loc	15 1306 5
	vmovaps	%ymm10, 1632(%rsp)
.Ltmp8625:
	.loc	15 1175 9
	leaq	.Lalloc_067dce244605df4a33956fbd4d726027(%rip), %rdx
	movq	64(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8626:
.LBB60_257:
	.loc	15 749 9
	vmovaps	%ymm11, 1536(%rsp)
.Ltmp8627:
	.loc	15 1306 5
	vmovaps	%ymm10, 1632(%rsp)
.Ltmp8628:
	.loc	15 1168 22
	leaq	.Lalloc_5cae9ea88ed6362f62dbad10291edbd4(%rip), %rdx
	movq	%r15, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8629:
.LBB60_258:
	.loc	15 749 9
	vmovaps	%ymm11, 1536(%rsp)
.Ltmp8630:
	.loc	15 1306 5
	vmovaps	%ymm10, 1632(%rsp)
.Ltmp8631:
	.loc	15 1169 24
	leaq	.Lalloc_9fdd00ed7abe4ccfd73f0d145a7b741b(%rip), %rdx
	movq	448(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8632:
.LBB60_259:
	.loc	15 749 9
	vmovaps	%ymm11, 1536(%rsp)
.Ltmp8633:
	.loc	15 1306 5
	vmovaps	%ymm10, 1632(%rsp)
.Ltmp8634:
	.loc	15 1173 24
	leaq	.Lalloc_697dc8945f5e5b040d7e9a0b92cb249f(%rip), %rdx
	movq	%r11, %rdi
	movq	64(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8635:
.LBB60_260:
	.loc	15 749 9
	vmovaps	%ymm11, 1536(%rsp)
.Ltmp8636:
	.loc	15 1306 5
	vmovaps	%ymm10, 1632(%rsp)
.Ltmp8637:
	.loc	15 1180 24
	leaq	.Lalloc_d5ea38731a1cb6311efef2f919234d06(%rip), %rdx
	movq	%r15, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8638:
.LBB60_261:
	.loc	37 443 13
	leaq	.Lalloc_5a4c0a22e74ec240d954f0d98bfc1654(%rip), %rcx
.Ltmp8639:
	.loc	37 443 13 is_stmt 0
	xorl	%edi, %edi
	movq	%r13, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8640:
.LBB60_262:
	.loc	37 443 13 is_stmt 1
	leaq	.Lalloc_6cbd2a842cd3fc5676a9a4fde31f14f8(%rip), %rcx
.Ltmp8641:
	.loc	37 443 13 is_stmt 0
	xorl	%edi, %edi
	movq	%r13, %rdx
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp8642:
.LBB60_263:
	.loc	15 749 9 is_stmt 1
	vmovaps	%ymm11, 1536(%rsp)
.Ltmp8643:
	.loc	15 1306 5
	vmovaps	%ymm10, 1632(%rsp)
.Ltmp8644:
	.loc	15 1302 25
	leaq	.Lalloc_aca25255cb99dc5ba482e692667f5878(%rip), %rdx
	movq	%r8, %rdi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8645:
.LBB60_264:
	.loc	15 0 25 is_stmt 0
	movl	$1, %eax
	jmp	.LBB60_281
.LBB60_265:
	movl	$2, %eax
	jmp	.LBB60_281
.LBB60_266:
.Ltmp8646:
	.loc	15 2986 25 is_stmt 1
	leaq	.Lalloc_2864b855a71e07cd82bfe5e37ca0325a(%rip), %rdx
	movq	%rbx, %rdi
	movq	%rbx, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB60_267:
	.loc	15 0 25 is_stmt 0
	movl	$3, %eax
	jmp	.LBB60_281
.LBB60_268:
	movl	$4, %eax
	jmp	.LBB60_281
.LBB60_269:
	movl	$1, %edi
.Ltmp8647:
	.loc	15 2987 23 is_stmt 1
	leaq	.Lalloc_8f78f3c6b9c9252b16ab7d7ccc563a9d(%rip), %rdx
	movq	%rbx, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB60_270:
	.loc	15 0 23 is_stmt 0
	movl	$2, %edi
	.loc	15 2987 23
	leaq	.Lalloc_8f78f3c6b9c9252b16ab7d7ccc563a9d(%rip), %rdx
	movq	%rbx, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB60_271:
	.loc	15 0 23
	movl	$3, %edi
	.loc	15 2987 23
	leaq	.Lalloc_8f78f3c6b9c9252b16ab7d7ccc563a9d(%rip), %rdx
	movq	%rbx, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB60_272:
	.loc	15 0 23
	movl	$4, %edi
	.loc	15 2987 23
	leaq	.Lalloc_8f78f3c6b9c9252b16ab7d7ccc563a9d(%rip), %rdx
	movq	%rbx, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB60_273:
	.loc	15 0 23
	movl	$5, %edi
	.loc	15 2987 23
	leaq	.Lalloc_8f78f3c6b9c9252b16ab7d7ccc563a9d(%rip), %rdx
	movq	%rbx, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB60_274:
	.loc	15 0 23
	movl	$6, %edi
	.loc	15 2987 23
	leaq	.Lalloc_8f78f3c6b9c9252b16ab7d7ccc563a9d(%rip), %rdx
	movq	%rbx, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB60_275:
	.loc	15 0 23
	movl	$7, %edi
	.loc	15 2987 23
	leaq	.Lalloc_8f78f3c6b9c9252b16ab7d7ccc563a9d(%rip), %rdx
	movq	%rbx, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB60_276:
	.loc	15 0 23
	movl	$5, %eax
	jmp	.LBB60_281
.LBB60_277:
	movl	$8, %edi
	.loc	15 2987 23
	leaq	.Lalloc_8f78f3c6b9c9252b16ab7d7ccc563a9d(%rip), %rdx
	movq	%rbx, %rsi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8648:
.LBB60_278:
	.loc	15 662 42 is_stmt 1
	leaq	.Lalloc_1781ea1b97b96e9885c590e9b4440a45(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.Ltmp8649:
.LBB60_279:
	.loc	15 0 42 is_stmt 0
	movl	$6, %eax
	jmp	.LBB60_281
.LBB60_280:
	movl	$7, %eax
.LBB60_281:
	movq	%rax, 288(%rsp)
.LBB60_282:
.Ltmp8650:
	.loc	15 749 9 is_stmt 1
	vmovaps	%ymm11, 1536(%rsp)
.Ltmp8651:
	.loc	15 1306 5
	vmovaps	%ymm10, 1632(%rsp)
.Ltmp8652:
	.loc	15 1298 42
	leaq	.Lalloc_891dd683d7367e0a1ec5a22f9c2a2aa8(%rip), %rdx
	movq	288(%rsp), %rdi
	movq	%rdi, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp8653:
.LBB60_283:
	.loc	15 452 44
	leaq	.Lalloc_6b076e9a9e313bc2481504f4da644a5c(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.LBB60_284:
	.loc	15 453 44
	leaq	.Lalloc_f370b9a38141751c9788be81259bacff(%rip), %rdi
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero@GOTPCREL(%rip)
.Ltmp8654:
.Lfunc_end60:
	.size	_RNvXse_CsdvPQf9CMsz3_17true_peak_limiterINtB5_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank17process_bank_monoB5_, .Lfunc_end60-_RNvXse_CsdvPQf9CMsz3_17true_peak_limiterINtB5_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank17process_bank_monoB5_
