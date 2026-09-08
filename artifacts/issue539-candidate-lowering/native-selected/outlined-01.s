_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation:
.Lfunc_begin10:
	.loc	1 2391 0 is_stmt 1
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
	subq	$120, %rsp
	.cfi_def_cfa_offset 176
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%r9, 56(%rsp)
	movl	$0, 24(%rsp)
	movl	$0, 32(%rsp)
	movl	$0, 40(%rsp)
	movl	$0, 48(%rsp)
.Ltmp2897:
	.loc	7 1714 9 prologue_end
	testq	%rsi, %rsi
.Ltmp2898:
	.loc	6 180 28
	je	.LBB10_4
.Ltmp2899:
	.loc	6 0 28 is_stmt 0
	movq	%rdi, %r13
	movq	184(%rsp), %rdi
	leaq	40(%rsp), %r9
	leaq	(%rsi,%rsi,4), %rax
	leaq	(,%rax,8), %r14
	addq	%r13, %r14
	movl	92(%rdx), %r10d
	movq	16(%rdi), %rbx
	movb	$1, %al
	movl	%eax, (%rsp)
	xorl	%r15d, %r15d
	leaq	24(%rsp), %r11
	.p2align	4
.LBB10_2:
.Ltmp2900:
	.loc	1 2403 29 is_stmt 1
	movl	32(%r13), %eax
	.loc	1 2403 23 is_stmt 0
	cmpl	$1, %eax
	je	.LBB10_3
	cmpl	$2, %eax
	jne	.LBB10_52
	.loc	1 0 23
	movl	$1, %eax
	movq	%r9, %r12
	jmp	.LBB10_39
	.p2align	4
.LBB10_3:
	xorl	%eax, %eax
	movq	%r11, %r12
.LBB10_39:
.Ltmp2901:
	.loc	1 2411 25 is_stmt 1
	movl	16(%r13), %ebp
.Ltmp2902:
	.loc	4 3178 26
	testl	%ebp, %ebp
.Ltmp2903:
	.loc	38 459 8
	js	.LBB10_52
.Ltmp2904:
	.loc	38 0 8 is_stmt 0
	cmpl	$1, %ebp
.Ltmp2905:
	.loc	1 2420 21 is_stmt 1
	ja	.LBB10_52
	cmpq	%r10, %r15
	jae	.LBB10_52
	.loc	1 2422 16
	cmpl	$1, 28(%r13)
	jne	.LBB10_52
	.loc	1 2423 16
	cmpq	%rcx, (%r13)
	jne	.LBB10_52
	.loc	1 2424 16
	cmpq	%rcx, 8(%r13)
	jne	.LBB10_52
	.loc	1 2425 16
	vmovd	20(%r13), %xmm0
.Ltmp2906:
	.file	41 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/num/f32.rs"
	.loc	41 1244 18
	vmovd	%xmm0, %esi
.Ltmp2907:
	.loc	1 2425 16
	cmpl	%esi, 24(%r13)
	jne	.LBB10_52
.Ltmp2908:
	.loc	1 0 0 is_stmt 0
	leal	(%rax,%rbp,2), %eax
	movl	%eax, 12(%rsp)
.Ltmp2909:
	.loc	1 2426 38 is_stmt 1
	leaq	(,%rbp,4), %rax
	addq	%rbp, %rax
	movl	%esi, 20(%rsp)
	leaq	.Lalloc_ce93677d93a92959931541f74fa374eb(%rip), %rsi
	leaq	(%rsi,%rax,8), %rdi
	movq	%r8, 88(%rsp)
	movq	%rdx, 80(%rsp)
	movq	%rcx, 72(%rsp)
	movq	%r10, 64(%rsp)
	vmovdqa	%xmm0, 96(%rsp)
	.loc	1 2426 16 is_stmt 0
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime6params21parameter_value_valid@GOTPCREL(%rip)
	vmovaps	96(%rsp), %xmm1
	leaq	24(%rsp), %r11
	movq	64(%rsp), %r10
	leaq	40(%rsp), %r9
	movq	184(%rsp), %rdi
	movq	72(%rsp), %rcx
	movq	80(%rsp), %rdx
	movq	88(%rsp), %r8
	movl	12(%rsp), %esi
	cmpl	16(%rsp), %esi
	seta	%sil
	negl	20(%rsp)
	jo	.LBB10_52
	testb	%al, %al
	je	.LBB10_52
	orb	(%rsp), %sil
	testb	$1, %sil
	je	.LBB10_52
.Ltmp2910:
	.loc	1 2430 13 is_stmt 1
	cmpb	$0, (%r12,%rbp,8)
	je	.LBB10_50
.Ltmp2911:
	.loc	1 0 13 is_stmt 0
.Ltmp2912:
	.p2align	4
.LBB10_52:
	addq	$40, %r13
.Ltmp2913:
	.loc	4 2428 13 is_stmt 1
	incq	%rbx
	movq	$-1, %rax
	cmoveq	%rax, %rbx
.Ltmp2914:
	.loc	1 0 0 is_stmt 0
	movq	%rbx, 16(%rdi)
.Ltmp2915:
	.loc	26 82 9 is_stmt 1
	incq	%r15
.Ltmp2916:
	.loc	7 1714 9
	cmpq	%r14, %r13
.Ltmp2917:
	.loc	6 180 28
	jne	.LBB10_2
	jmp	.LBB10_4
.Ltmp2918:
.LBB10_50:
	.file	42 "/home/bl/misofm/engine-limiter-stationary-dispatch" "crates/effect-runtime/src/params.rs"
	.loc	42 110 8
	vpxor	%xmm0, %xmm0, %xmm0
	vcmpeqss	%xmm0, %xmm1, %xmm0
	vandnps	%xmm1, %xmm0, %xmm0
.Ltmp2919:
	.loc	1 2435 9
	movl	$1, (%r12,%rbp,8)
	vmovss	%xmm0, 4(%r12,%rbp,8)
.Ltmp2920:
	.loc	7 1714 9
	addq	$40, %r13
.Ltmp2921:
	.loc	6 180 28
	incq	%r15
	movl	$0, (%rsp)
	movl	12(%rsp), %eax
	movl	%eax, 16(%rsp)
.Ltmp2922:
	.loc	7 1714 9
	cmpq	%r14, %r13
.Ltmp2923:
	.loc	6 180 28
	jne	.LBB10_2
.Ltmp2924:
.LBB10_4:
	.loc	6 0 28 is_stmt 0
	movq	176(%rsp), %r15
	.loc	1 2437 16 is_stmt 1
	movl	72(%rdx), %ebx
.Ltmp2925:
	.loc	1 2439 16
	cmpl	$1, 24(%rsp)
	movq	56(%rsp), %r13
	jne	.LBB10_12
	.loc	1 2440 13
	movq	136(%r8), %rsi
	cmpq	%rsi, %r15
	jae	.LBB10_54
	.loc	1 0 13 is_stmt 0
	movq	%r8, %rbp
	.loc	1 2440 13
	movq	128(%r8), %r12
	.loc	1 2439 21 is_stmt 1
	vmovss	28(%rsp), %xmm0
	.loc	1 2440 13
	movq	%r15, %r13
	shlq	$4, %r13
	leaq	(%r12,%r13), %r14
.Ltmp2926:
	.file	43 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/convert/num.rs"
	.loc	43 82 17
	vcvtss2sd	%xmm0, %xmm0, %xmm0
.Ltmp2927:
	.loc	1 883 22
	vaddsd	.LCPI10_0(%rip), %xmm0, %xmm0
.Ltmp2928:
	.file	44 "/home/bl/misofm/engine-limiter-stationary-dispatch" "crates/math/src/lib.rs"
	.loc	44 232 10
	vmulsd	.LCPI10_1(%rip), %xmm0, %xmm0
.Ltmp2929:
	.loc	44 58 5
	callq	*_RNvNtNtCshmZ46FhrXRY_4math8vendored4exp24exp2@GOTPCREL(%rip)
.Ltmp2930:
	.loc	1 883 5
	vcvtsd2ss	%xmm0, %xmm0, %xmm0
.Ltmp2931:
	.file	45 "/home/bl/misofm/engine-limiter-stationary-dispatch" "crates/effect-runtime/src/ramp.rs"
	.loc	45 80 9
	vmovss	%xmm0, 4(%r12,%r13)
	.loc	45 81 48
	vmovd	(%r12,%r13), %xmm1
	vmovd	%xmm1, %eax
	movl	%eax, %ecx
	andl	$2147483647, %ecx
	cmpl	$2139095039, %ecx
.Ltmp2932:
	.loc	45 112 9
	jg	.LBB10_9
	.loc	45 112 0 is_stmt 0
	vmovd	%xmm0, %ecx
	.loc	45 112 9
	cmpl	%ecx, %eax
	jne	.LBB10_9
	negl	%eax
	jo	.LBB10_9
.Ltmp2933:
	.loc	45 82 13 is_stmt 1
	vmovss	%xmm0, (%r14)
	xorl	%eax, %eax
	vxorps	%xmm0, %xmm0, %xmm0
	jmp	.LBB10_11
.LBB10_9:
	.loc	45 87 21
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	45 87 9 is_stmt 0
	vmulss	.LCPI10_2(%rip), %xmm0, %xmm0
	movl	$64, %eax
.LBB10_11:
	.loc	45 0 0
	vmovss	%xmm0, 8(%r14)
	movl	%eax, 12(%r14)
	movq	%rbp, %r8
	movq	56(%rsp), %r13
.Ltmp2934:
.LBB10_12:
	vcvtsi2sd	%rbx, %xmm15, %xmm1
.Ltmp2935:
	.loc	1 2442 16 is_stmt 1
	cmpl	$1, 32(%rsp)
	vmovsd	%xmm1, (%rsp)
	jne	.LBB10_19
	.loc	1 2443 13
	movq	152(%r8), %rsi
	cmpq	%rsi, %r15
	jae	.LBB10_36
	movq	144(%r8), %r14
	.loc	1 2442 21
	vmovss	36(%rsp), %xmm0
	.loc	1 2443 13
	movq	%r15, %r12
	shlq	$4, %r12
.Ltmp2936:
	.loc	43 82 17
	vcvtss2sd	%xmm0, %xmm0, %xmm0
.Ltmp2937:
	.loc	1 890 30
	vmulsd	.LCPI10_3(%rip), %xmm0, %xmm0
.Ltmp2938:
	.loc	1 2443 13
	leaq	(%r14,%r12), %rbx
.Ltmp2939:
	.loc	1 890 29
	vmulsd	%xmm1, %xmm0, %xmm0
	vmovsd	.LCPI10_0(%rip), %xmm1
	.loc	1 890 22 is_stmt 0
	vdivsd	%xmm0, %xmm1, %xmm0
.Ltmp2940:
	.loc	44 52 5 is_stmt 1
	callq	*_RNvNtNtCshmZ46FhrXRY_4math8vendored3exp3exp@GOTPCREL(%rip)
	vmovsd	.LCPI10_4(%rip), %xmm1
.Ltmp2941:
	.loc	1 890 5
	vsubsd	%xmm0, %xmm1, %xmm0
	vcvtsd2ss	%xmm0, %xmm0, %xmm0
.Ltmp2942:
	.loc	45 80 9
	vmovss	%xmm0, 4(%r14,%r12)
	.loc	45 81 48
	vmovd	(%r14,%r12), %xmm1
	vmovd	%xmm1, %eax
	movl	%eax, %ecx
	andl	$2147483647, %ecx
	cmpl	$2139095039, %ecx
.Ltmp2943:
	.loc	45 112 9
	jg	.LBB10_17
	.loc	45 112 0 is_stmt 0
	vmovd	%xmm0, %ecx
	.loc	45 112 9
	cmpl	%ecx, %eax
	jne	.LBB10_17
	negl	%eax
	jo	.LBB10_17
.Ltmp2944:
	.loc	45 82 13 is_stmt 1
	vmovss	%xmm0, (%rbx)
	xorl	%eax, %eax
	vxorps	%xmm0, %xmm0, %xmm0
	.loc	45 89 6
	jmp	.LBB10_18
.LBB10_17:
	.loc	45 87 21
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	45 87 9 is_stmt 0
	vmulss	.LCPI10_2(%rip), %xmm0, %xmm0
	movl	$64, %eax
.LBB10_18:
	.loc	45 0 0
	vmovss	%xmm0, 8(%rbx)
	movl	%eax, 12(%rbx)
	vmovsd	(%rsp), %xmm1
.Ltmp2945:
.LBB10_19:
	.loc	1 2439 16 is_stmt 1
	cmpl	$1, 40(%rsp)
	jne	.LBB10_27
	.loc	1 2440 13
	movq	136(%r13), %rsi
	cmpq	%rsi, %r15
	jae	.LBB10_54
	movq	128(%r13), %r14
	.loc	1 2439 21
	vmovss	44(%rsp), %xmm0
	.loc	1 2440 13
	movq	%r15, %r12
	shlq	$4, %r12
	leaq	(%r14,%r12), %rbx
.Ltmp2946:
	.loc	43 82 17
	vcvtss2sd	%xmm0, %xmm0, %xmm0
.Ltmp2947:
	.loc	1 883 22
	vaddsd	.LCPI10_0(%rip), %xmm0, %xmm0
.Ltmp2948:
	.loc	44 232 10
	vmulsd	.LCPI10_1(%rip), %xmm0, %xmm0
.Ltmp2949:
	.loc	44 58 5
	callq	*_RNvNtNtCshmZ46FhrXRY_4math8vendored4exp24exp2@GOTPCREL(%rip)
.Ltmp2950:
	.loc	1 883 5
	vcvtsd2ss	%xmm0, %xmm0, %xmm0
.Ltmp2951:
	.loc	45 80 9
	vmovss	%xmm0, 4(%r14,%r12)
	.loc	45 81 48
	vmovd	(%r14,%r12), %xmm1
	vmovd	%xmm1, %eax
	movl	%eax, %ecx
	andl	$2147483647, %ecx
	cmpl	$2139095039, %ecx
.Ltmp2952:
	.loc	45 112 9
	jg	.LBB10_25
	.loc	45 112 0 is_stmt 0
	vmovd	%xmm0, %ecx
	.loc	45 112 9
	cmpl	%ecx, %eax
	jne	.LBB10_25
	negl	%eax
	jo	.LBB10_25
.Ltmp2953:
	.loc	45 82 13 is_stmt 1
	vmovss	%xmm0, (%rbx)
	xorl	%eax, %eax
	vxorps	%xmm0, %xmm0, %xmm0
	jmp	.LBB10_26
.LBB10_25:
	.loc	45 87 21
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	45 87 9 is_stmt 0
	vmulss	.LCPI10_2(%rip), %xmm0, %xmm0
	movl	$64, %eax
.LBB10_26:
	.loc	45 0 9
	vmovsd	(%rsp), %xmm1
	vmovss	%xmm0, 8(%rbx)
	movl	%eax, 12(%rbx)
.Ltmp2954:
.LBB10_27:
	.loc	1 2442 16 is_stmt 1
	cmpl	$1, 48(%rsp)
	jne	.LBB10_35
	.loc	1 2443 13
	movq	152(%r13), %rsi
	cmpq	%rsi, %r15
	jae	.LBB10_36
	movq	144(%r13), %r14
	.loc	1 2442 21
	vmovss	52(%rsp), %xmm0
	.loc	1 2443 13
	shlq	$4, %r15
.Ltmp2955:
	.loc	43 82 17
	vcvtss2sd	%xmm0, %xmm0, %xmm0
.Ltmp2956:
	.loc	1 890 30
	vmulsd	.LCPI10_3(%rip), %xmm0, %xmm0
.Ltmp2957:
	.loc	1 2443 13
	leaq	(%r14,%r15), %rbx
.Ltmp2958:
	.loc	1 890 29
	vmulsd	%xmm1, %xmm0, %xmm0
	vmovsd	.LCPI10_0(%rip), %xmm1
	.loc	1 890 22 is_stmt 0
	vdivsd	%xmm0, %xmm1, %xmm0
.Ltmp2959:
	.loc	44 52 5 is_stmt 1
	callq	*_RNvNtNtCshmZ46FhrXRY_4math8vendored3exp3exp@GOTPCREL(%rip)
	vmovsd	.LCPI10_4(%rip), %xmm1
.Ltmp2960:
	.loc	1 890 5
	vsubsd	%xmm0, %xmm1, %xmm0
	vcvtsd2ss	%xmm0, %xmm0, %xmm0
.Ltmp2961:
	.loc	45 80 9
	vmovss	%xmm0, 4(%r14,%r15)
	.loc	45 81 48
	vmovd	(%r14,%r15), %xmm1
	vmovd	%xmm1, %eax
	movl	%eax, %ecx
	andl	$2147483647, %ecx
	cmpl	$2139095039, %ecx
.Ltmp2962:
	.loc	45 112 9
	jg	.LBB10_33
	.loc	45 112 0 is_stmt 0
	vmovd	%xmm0, %ecx
	.loc	45 112 9
	cmpl	%ecx, %eax
	jne	.LBB10_33
	negl	%eax
	jo	.LBB10_33
.Ltmp2963:
	.loc	45 82 13 is_stmt 1
	vmovss	%xmm0, (%rbx)
	xorl	%eax, %eax
	vxorps	%xmm0, %xmm0, %xmm0
	.loc	45 89 6
	jmp	.LBB10_34
.LBB10_33:
	.loc	45 87 21
	vsubss	%xmm1, %xmm0, %xmm0
	.loc	45 87 9 is_stmt 0
	vmulss	.LCPI10_2(%rip), %xmm0, %xmm0
	movl	$64, %eax
.LBB10_34:
	.loc	45 0 0
	vmovss	%xmm0, 8(%rbx)
	movl	%eax, 12(%rbx)
.Ltmp2964:
.LBB10_35:
	.loc	1 2446 2 epilogue_begin is_stmt 1
	addq	$120, %rsp
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
.LBB10_54:
	.cfi_def_cfa_offset 176
.Ltmp2965:
	.loc	1 2440 13
	leaq	.Lalloc_5cf57561855c71df127711389f77b379(%rip), %rdx
	movq	%r15, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp2966:
.LBB10_36:
	.loc	1 2443 13
	leaq	.Lalloc_93e6eee1cc4097d01ff84e10e6b70f81(%rip), %rdx
	movq	%r15, %rdi
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp2967:
.Lfunc_end10:
	.size	_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation, .Lfunc_end10-_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation
