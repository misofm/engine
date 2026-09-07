_RNvXsd_CsdOTRa1MFkeb_13gate_expanderINtB5_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bank:
.Lfunc_begin40:
	.loc	6 1069 0
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
	subq	$1432, %rsp
	.cfi_def_cfa_offset 1488
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	.loc	6 1070 29 prologue_end
	movzbl	1824(%rsi), %eax
.Ltmp6797:
	.loc	18 966 15
	cmpb	$2, %al
	.loc	18 966 9 is_stmt 0
	je	.LBB40_158
.Ltmp6798:
	.loc	20 1032 9 is_stmt 1
	vpxor	%xmm0, %xmm0, %xmm0
	vmovdqu	%ymm0, 1392(%rsp)
	vmovdqu	%ymm0, 1360(%rsp)
	vmovdqu	%ymm0, 1328(%rsp)
	vmovdqu	%ymm0, 1296(%rsp)
	vmovdqu	%ymm0, 1264(%rsp)
	vmovdqu	%ymm0, 1232(%rsp)
	vmovdqu	%ymm0, 1200(%rsp)
	vmovdqu	%ymm0, 1168(%rsp)
	vmovdqu	%ymm0, 1136(%rsp)
	vmovdqu	%ymm0, 1104(%rsp)
.Ltmp6799:
	.loc	20 186 45
	cmpb	%al, 108(%rdx)
.Ltmp6800:
	.loc	6 1075 20
	jne	.LBB40_60
	cmpq	$0, 64(%rdx)
	jne	.LBB40_60
	.loc	6 0 20 is_stmt 0
	movq	%rsi, %r9
	movq	%rdi, 408(%rsp)
	.loc	6 1080 35 is_stmt 1
	vmovdqu	%ymm0, 1072(%rsp)
	vmovdqu	%ymm0, 1040(%rsp)
	vmovdqu	%ymm0, 1008(%rsp)
	vmovdqu	%ymm0, 976(%rsp)
	vmovdqu	%ymm0, 944(%rsp)
	vmovdqu	%ymm0, 912(%rsp)
	vmovdqu	%ymm0, 880(%rsp)
	vmovdqu	%ymm0, 848(%rsp)
	vmovdqu	%ymm0, 816(%rsp)
	vmovdqu	%ymm0, 784(%rsp)
	movq	48(%rdx), %rcx
	movq	%rcx, 64(%rsp)
	movq	56(%rdx), %r10
	movq	32(%rdx), %rcx
	movq	%rcx, 56(%rsp)
	movq	40(%rdx), %rcx
	movq	%rcx, 96(%rsp)
	movq	%rdx, 72(%rsp)
	movq	96(%rdx), %rcx
	movq	%rcx, 48(%rsp)
	leaq	1088(%rsi), %rcx
	movq	%rcx, 456(%rsp)
.Ltmp6801:
	.loc	11 900 12
	cmpq	$1, %r10
	movq	%r10, %rcx
	adcq	$-1, %rcx
	movq	%rcx, 144(%rsp)
	leaq	1328(%rsi), %rcx
	movq	%rcx, 88(%rsp)
	xorl	%ebp, %ebp
	vmovss	.LCPI40_0(%rip), %xmm5
	vmovss	.LCPI40_1(%rip), %xmm6
	movq	%rsi, 40(%rsp)
	movb	%al, 39(%rsp)
	movq	%r10, 80(%rsp)
	jmp	.LBB40_5
	.loc	11 0 12 is_stmt 0
.Ltmp6802:
	.p2align	4
.LBB40_4:
	movq	%rbx, %rbp
.Ltmp6803:
	.loc	8 1916 50 is_stmt 1
	cmpq	$4, %rbx
.Ltmp6804:
	.loc	11 900 12
	je	.LBB40_61
.Ltmp6805:
.LBB40_5:
	.loc	6 1082 33
	cmpq	%r10, %rbp
	je	.LBB40_163
.Ltmp6806:
	.loc	6 0 0 is_stmt 0
	leaq	1(%rbp), %rsi
.Ltmp6807:
	.loc	6 1083 31 is_stmt 1
	cmpq	144(%rsp), %rbp
	je	.LBB40_164
	.loc	6 0 31 is_stmt 0
	movq	64(%rsp), %rcx
	movl	(%rcx,%rbp,4), %edi
	.loc	6 1083 31
	movl	(%rcx,%rsi,4), %esi
.Ltmp6808:
	.loc	15 1050 16 is_stmt 1
	cmpl	%edi, %esi
	jb	.LBB40_145
	cmpq	%rsi, 96(%rsp)
	jb	.LBB40_145
.Ltmp6809:
	.loc	15 0 16 is_stmt 0
	movl	$0, 288(%rsp)
	movl	$0, 296(%rsp)
	movl	$0, 304(%rsp)
	movl	$0, 312(%rsp)
	movl	$0, 320(%rsp)
	movl	$0, 328(%rsp)
	movl	$0, 336(%rsp)
	movl	$0, 344(%rsp)
.Ltmp6810:
	.loc	38 1714 9 is_stmt 1
	cmpl	%edi, %esi
.Ltmp6811:
	.loc	19 180 28
	jne	.LBB40_44
.Ltmp6812:
.LBB40_10:
	.loc	19 0 28 is_stmt 0
	movq	88(%rsp), %rax
	xorl	%ecx, %ecx
	leaq	1(%rbp), %rbx
.Ltmp6813:
	.loc	19 180 28
	jmp	.LBB40_13
.Ltmp6814:
	.loc	19 0 28
.Ltmp6815:
	.p2align	4
.LBB40_11:
	.loc	6 396 5 is_stmt 1
	vmovaps	-48(%rax), %xmm4
	vmovaps	%xmm4, (%rsp)
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%rbp,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm1
	vmovaps	%xmm1, -48(%rax)
.Ltmp6816:
	.loc	6 396 5
	vmovaps	-32(%rax), %xmm1
	vmovaps	%xmm1, (%rsp)
	.loc	6 397 5
	vmovss	%xmm0, (%rsp,%rbp,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -32(%rax)
.Ltmp6817:
	.loc	6 396 5
	vmovaps	-16(%rax), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 397 5
	vmovss	%xmm2, (%rsp,%rbp,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -16(%rax)
.Ltmp6818:
	.loc	6 396 5
	vmovaps	(%rax), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 397 5
	vmovss	%xmm3, (%rsp,%rbp,4)
	.loc	6 398 5
	vmovdqa	(%rsp), %xmm0
	vmovdqa	%xmm0, (%rax)
.Ltmp6819:
	.loc	6 661 17
	movl	$64, 1820(%r9)
.Ltmp6820:
.LBB40_12:
	.loc	6 0 0 is_stmt 0
	addq	$32, %rcx
.Ltmp6821:
	.loc	38 1714 9 is_stmt 1
	addq	$304, %rax
	cmpq	$64, %rcx
.Ltmp6822:
	.loc	19 180 28
	je	.LBB40_4
.Ltmp6823:
.LBB40_13:
	.loc	6 648 21
	cmpl	$1, 288(%rsp,%rcx)
	jne	.LBB40_14
	.loc	6 648 0 is_stmt 0
	leaq	-240(%rax), %rdx
	.loc	6 648 26
	vmovd	292(%rsp,%rcx), %xmm0
.Ltmp6824:
	.loc	1 551 14 is_stmt 1
	vmovups	(%rdx), %xmm1
	vmovaps	%xmm1, (%rsp)
.Ltmp6825:
	.loc	6 390 5
	vmovd	(%rsp,%rbp,4), %xmm1
	vmovd	%xmm1, %esi
.Ltmp6826:
	.loc	7 1244 18
	vmovd	%xmm0, %edi
.Ltmp6827:
	.loc	47 112 9
	cmpl	%edi, %esi
	setne	%dil
	movl	%esi, %r8d
	negl	%r8d
	seto	%r8b
	orb	%dil, %r8b
	andl	$2147483647, %esi
	cmpl	$2139095040, %esi
	setge	%sil
	orb	%r8b, %sil
	vsubss	%xmm1, %xmm0, %xmm2
	vmulss	%xmm5, %xmm2, %xmm2
	testb	%sil, %sil
	jne	.LBB40_18
	.loc	47 0 9 is_stmt 0
	vmovaps	%xmm0, %xmm1
	.loc	47 112 9
	je	.LBB40_36
.LBB40_19:
	.loc	47 0 9
	vmovaps	%xmm6, %xmm3
	.loc	47 112 9
	jne	.LBB40_21
.Ltmp6828:
.LBB40_20:
	.loc	47 0 9
	vxorps	%xmm3, %xmm3, %xmm3
.LBB40_21:
.Ltmp6829:
	.loc	6 396 5 is_stmt 1
	vmovaps	(%rdx), %xmm4
	vmovaps	%xmm4, (%rsp)
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%rbp,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm1
	vmovaps	%xmm1, (%rdx)
.Ltmp6830:
	.loc	6 396 5
	vmovaps	-224(%rax), %xmm1
	vmovaps	%xmm1, (%rsp)
	.loc	6 397 5
	vmovss	%xmm0, (%rsp,%rbp,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -224(%rax)
.Ltmp6831:
	.loc	6 396 5
	vmovaps	-208(%rax), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 397 5
	vmovss	%xmm2, (%rsp,%rbp,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -208(%rax)
.Ltmp6832:
	.loc	6 396 5
	vmovaps	-192(%rax), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 397 5
	vmovss	%xmm3, (%rsp,%rbp,4)
	.loc	6 398 5
	vmovdqa	(%rsp), %xmm0
	vmovdqa	%xmm0, -192(%rax)
.Ltmp6833:
	.loc	6 661 17
	movl	$64, 1820(%r9)
.Ltmp6834:
	.loc	6 648 21
	cmpl	$1, 296(%rsp,%rcx)
	je	.LBB40_22
.LBB40_15:
	cmpl	$1, 304(%rsp,%rcx)
	jne	.LBB40_16
.LBB40_27:
	.loc	6 648 26 is_stmt 0
	vmovd	308(%rsp,%rcx), %xmm0
.Ltmp6835:
	.loc	1 551 14 is_stmt 1
	vmovups	-112(%rax), %xmm1
	vmovaps	%xmm1, (%rsp)
.Ltmp6836:
	.loc	6 390 5
	vmovd	(%rsp,%rbp,4), %xmm1
	vmovd	%xmm1, %edx
.Ltmp6837:
	.loc	7 1244 18
	vmovd	%xmm0, %esi
.Ltmp6838:
	.loc	47 112 9
	cmpl	%esi, %edx
	setne	%sil
	movl	%edx, %edi
	negl	%edi
	seto	%dil
	orb	%sil, %dil
	andl	$2147483647, %edx
	cmpl	$2139095040, %edx
	setge	%dl
	orb	%dil, %dl
	vsubss	%xmm1, %xmm0, %xmm2
	vmulss	%xmm5, %xmm2, %xmm2
	testb	%dl, %dl
	jne	.LBB40_28
	.loc	47 0 9 is_stmt 0
	vmovaps	%xmm0, %xmm1
	.loc	47 112 9
	je	.LBB40_40
.LBB40_29:
	.loc	47 0 9
	vmovaps	%xmm6, %xmm3
	.loc	47 112 9
	jne	.LBB40_31
.Ltmp6839:
.LBB40_30:
	.loc	47 0 9
	vxorps	%xmm3, %xmm3, %xmm3
.LBB40_31:
.Ltmp6840:
	.loc	6 396 5 is_stmt 1
	vmovaps	-112(%rax), %xmm4
	vmovaps	%xmm4, (%rsp)
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%rbp,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm1
	vmovaps	%xmm1, -112(%rax)
.Ltmp6841:
	.loc	6 396 5
	vmovaps	-96(%rax), %xmm1
	vmovaps	%xmm1, (%rsp)
	.loc	6 397 5
	vmovss	%xmm0, (%rsp,%rbp,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -96(%rax)
.Ltmp6842:
	.loc	6 396 5
	vmovaps	-80(%rax), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 397 5
	vmovss	%xmm2, (%rsp,%rbp,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -80(%rax)
.Ltmp6843:
	.loc	6 396 5
	vmovaps	-64(%rax), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 397 5
	vmovss	%xmm3, (%rsp,%rbp,4)
	.loc	6 398 5
	vmovdqa	(%rsp), %xmm0
	vmovdqa	%xmm0, -64(%rax)
.Ltmp6844:
	.loc	6 661 17
	movl	$64, 1820(%r9)
.Ltmp6845:
	.loc	6 648 21
	cmpl	$1, 312(%rsp,%rcx)
	jne	.LBB40_12
	jmp	.LBB40_32
	.loc	6 0 21 is_stmt 0
.Ltmp6846:
	.p2align	4
.LBB40_14:
	.loc	6 648 21
	cmpl	$1, 296(%rsp,%rcx)
	jne	.LBB40_15
.LBB40_22:
	.loc	6 648 26
	vmovd	300(%rsp,%rcx), %xmm0
.Ltmp6847:
	.loc	1 551 14 is_stmt 1
	vmovups	-176(%rax), %xmm1
	vmovaps	%xmm1, (%rsp)
.Ltmp6848:
	.loc	6 390 5
	vmovd	(%rsp,%rbp,4), %xmm1
	vmovd	%xmm1, %edx
.Ltmp6849:
	.loc	7 1244 18
	vmovd	%xmm0, %esi
.Ltmp6850:
	.loc	47 112 9
	cmpl	%esi, %edx
	setne	%sil
	movl	%edx, %edi
	negl	%edi
	seto	%dil
	orb	%sil, %dil
	andl	$2147483647, %edx
	cmpl	$2139095040, %edx
	setge	%dl
	orb	%dil, %dl
	vsubss	%xmm1, %xmm0, %xmm2
	vmulss	%xmm5, %xmm2, %xmm2
	testb	%dl, %dl
	jne	.LBB40_23
	.loc	47 0 9 is_stmt 0
	vmovaps	%xmm0, %xmm1
	.loc	47 112 9
	je	.LBB40_38
.LBB40_24:
	.loc	47 0 9
	vmovaps	%xmm6, %xmm3
	.loc	47 112 9
	jne	.LBB40_26
.Ltmp6851:
.LBB40_25:
	.loc	47 0 9
	vxorps	%xmm3, %xmm3, %xmm3
.LBB40_26:
.Ltmp6852:
	.loc	6 396 5 is_stmt 1
	vmovaps	-176(%rax), %xmm4
	vmovaps	%xmm4, (%rsp)
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%rbp,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm1
	vmovaps	%xmm1, -176(%rax)
.Ltmp6853:
	.loc	6 396 5
	vmovaps	-160(%rax), %xmm1
	vmovaps	%xmm1, (%rsp)
	.loc	6 397 5
	vmovss	%xmm0, (%rsp,%rbp,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -160(%rax)
.Ltmp6854:
	.loc	6 396 5
	vmovaps	-144(%rax), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 397 5
	vmovss	%xmm2, (%rsp,%rbp,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -144(%rax)
.Ltmp6855:
	.loc	6 396 5
	vmovaps	-128(%rax), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 397 5
	vmovss	%xmm3, (%rsp,%rbp,4)
	.loc	6 398 5
	vmovdqa	(%rsp), %xmm0
	vmovdqa	%xmm0, -128(%rax)
.Ltmp6856:
	.loc	6 661 17
	movl	$64, 1820(%r9)
.Ltmp6857:
	.loc	6 648 21
	cmpl	$1, 304(%rsp,%rcx)
	je	.LBB40_27
.LBB40_16:
	cmpl	$1, 312(%rsp,%rcx)
	jne	.LBB40_12
.LBB40_32:
	.loc	6 648 26 is_stmt 0
	vmovd	316(%rsp,%rcx), %xmm0
.Ltmp6858:
	.loc	1 551 14 is_stmt 1
	vmovups	-48(%rax), %xmm1
	vmovaps	%xmm1, (%rsp)
.Ltmp6859:
	.loc	6 390 5
	vmovd	(%rsp,%rbp,4), %xmm1
	vmovd	%xmm1, %edx
.Ltmp6860:
	.loc	7 1244 18
	vmovd	%xmm0, %esi
.Ltmp6861:
	.loc	47 112 9
	cmpl	%esi, %edx
	setne	%sil
	movl	%edx, %edi
	negl	%edi
	seto	%dil
	orb	%sil, %dil
	andl	$2147483647, %edx
	cmpl	$2139095040, %edx
	setge	%dl
	orb	%dil, %dl
	vsubss	%xmm1, %xmm0, %xmm2
	vmulss	%xmm5, %xmm2, %xmm2
	testb	%dl, %dl
	jne	.LBB40_33
	.loc	47 0 9 is_stmt 0
	vmovaps	%xmm0, %xmm1
	.loc	47 112 9
	je	.LBB40_42
.LBB40_34:
	.loc	47 0 9
	vmovaps	%xmm6, %xmm3
	.loc	47 112 9
	jne	.LBB40_11
	jmp	.LBB40_43
	.loc	47 0 9
.Ltmp6862:
	.p2align	4
.LBB40_18:
	.loc	47 112 9
	jne	.LBB40_19
.LBB40_36:
	.loc	47 0 9
	vxorps	%xmm2, %xmm2, %xmm2
	vmovaps	%xmm6, %xmm3
	.loc	47 112 9
	je	.LBB40_20
	jmp	.LBB40_21
	.loc	47 0 9
.Ltmp6863:
	.p2align	4
.LBB40_28:
	.loc	47 112 9
	jne	.LBB40_29
.LBB40_40:
	.loc	47 0 9
	vxorps	%xmm2, %xmm2, %xmm2
	vmovaps	%xmm6, %xmm3
	.loc	47 112 9
	je	.LBB40_30
	jmp	.LBB40_31
	.loc	47 0 9
.Ltmp6864:
	.p2align	4
.LBB40_23:
	.loc	47 112 9
	jne	.LBB40_24
.LBB40_38:
	.loc	47 0 9
	vxorps	%xmm2, %xmm2, %xmm2
	vmovaps	%xmm6, %xmm3
	.loc	47 112 9
	je	.LBB40_25
	jmp	.LBB40_26
	.loc	47 0 9
.Ltmp6865:
	.p2align	4
.LBB40_33:
	.loc	47 112 9
	jne	.LBB40_34
.LBB40_42:
	.loc	47 0 9
	vxorps	%xmm2, %xmm2, %xmm2
	vmovaps	%xmm6, %xmm3
	.loc	47 112 9
	jne	.LBB40_11
.Ltmp6866:
.LBB40_43:
	.loc	47 0 9
	vxorps	%xmm3, %xmm3, %xmm3
	jmp	.LBB40_11
	.p2align	4
.LBB40_44:
	subq	%rdi, %rsi
	leaq	(%rdi,%rdi,4), %rax
	movq	56(%rsp), %rcx
	leaq	(%rcx,%rax,8), %r14
	leaq	(%rsi,%rsi,4), %rax
	leaq	(%r14,%rax,8), %rdx
	.loc	6 1088 25 is_stmt 1
	leaq	(%rbp,%rbp,4), %rax
	leaq	784(%rsp,%rax,8), %rsi
	movl	1796(%r9), %edi
	movq	800(%rsp,%rax,8), %r13
	movb	$1, %r8b
	xorl	%r12d, %r12d
	jmp	.LBB40_45
	.loc	6 0 25 is_stmt 0
.Ltmp6867:
	.p2align	4
.LBB40_58:
.Ltmp6868:
	addq	$40, %r14
.Ltmp6869:
	.loc	15 2428 13 is_stmt 1
	incq	%r13
	movq	$-1, %rax
	cmoveq	%rax, %r13
.Ltmp6870:
	.loc	6 0 0 is_stmt 0
	movq	%r13, 16(%rsi)
.Ltmp6871:
	.loc	34 82 9 is_stmt 1
	incq	%r12
.Ltmp6872:
	.loc	38 1714 9
	cmpq	%rdx, %r14
.Ltmp6873:
	.loc	19 180 28
	je	.LBB40_10
.Ltmp6874:
.LBB40_45:
	.loc	6 613 33
	movl	32(%r14), %eax
	.loc	6 613 27 is_stmt 0
	cmpl	$1, %eax
	je	.LBB40_48
	cmpl	$2, %eax
	jne	.LBB40_58
	.loc	6 0 27
	movl	$1, %eax
	leaq	320(%rsp), %rbx
.Ltmp6875:
	.loc	6 621 35 is_stmt 1
	movl	16(%r14), %r15d
.Ltmp6876:
	.loc	15 3178 26
	testl	%r15d, %r15d
.Ltmp6877:
	.loc	46 459 8
	jns	.LBB40_49
	jmp	.LBB40_58
.Ltmp6878:
	.loc	46 0 8 is_stmt 0
.Ltmp6879:
	.p2align	4
.LBB40_48:
	xorl	%eax, %eax
	leaq	288(%rsp), %rbx
	.loc	6 621 35 is_stmt 1
	movl	16(%r14), %r15d
.Ltmp6880:
	.loc	15 3178 26
	testl	%r15d, %r15d
.Ltmp6881:
	.loc	46 459 8
	js	.LBB40_58
.Ltmp6882:
.LBB40_49:
	.loc	6 630 25
	cmpq	%rdi, %r12
	jae	.LBB40_58
	cmpl	$3, %r15d
	ja	.LBB40_58
	.loc	6 632 20
	cmpl	$1, 28(%r14)
	jne	.LBB40_58
	.loc	6 0 20 is_stmt 0
	movq	48(%rsp), %rcx
	.loc	6 633 20 is_stmt 1
	cmpq	%rcx, (%r14)
	jne	.LBB40_58
	.loc	6 0 20 is_stmt 0
	movq	48(%rsp), %rcx
	.loc	6 634 20 is_stmt 1
	cmpq	%rcx, 8(%r14)
	jne	.LBB40_58
	.loc	6 635 20
	vmovd	20(%r14), %xmm0
.Ltmp6883:
	.loc	7 1244 18
	vmovd	%xmm0, %ecx
.Ltmp6884:
	.loc	6 635 20
	cmpl	%ecx, 24(%r14)
	jne	.LBB40_58
.Ltmp6885:
	.loc	6 0 0 is_stmt 0
	leal	(%rax,%r15,2), %eax
	movl	%eax, 128(%rsp)
.Ltmp6886:
	.loc	6 636 42 is_stmt 1
	leaq	(%r15,%r15,4), %rax
	leaq	.Lalloc_d0058eaee52f1ba8b2e5577cef0c3c7f(%rip), %rcx
	movq	%rdi, 208(%rsp)
	leaq	(%rcx,%rax,8), %rdi
	movq	%rdx, 240(%rsp)
	movq	%rsi, 224(%rsp)
	movl	%r8d, 176(%rsp)
	vmovdqa	%xmm0, 160(%rsp)
	.loc	6 636 20 is_stmt 0
	vzeroupper
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime6params21parameter_value_valid@GOTPCREL(%rip)
	vmovaps	160(%rsp), %xmm1
	movl	176(%rsp), %r8d
	movq	208(%rsp), %rdi
	movq	224(%rsp), %rsi
	movq	240(%rsp), %rdx
	vmovss	.LCPI40_1(%rip), %xmm6
	vmovss	.LCPI40_0(%rip), %xmm5
	movq	80(%rsp), %r10
	movq	40(%rsp), %r9
	movl	128(%rsp), %ecx
	movl	%ecx, %r11d
	cmpl	120(%rsp), %ecx
	seta	%cl
	testb	%al, %al
	je	.LBB40_58
	orb	%r8b, %cl
	testb	$1, %cl
	je	.LBB40_58
.Ltmp6887:
	.loc	6 639 17 is_stmt 1
	cmpb	$0, (%rbx,%r15,8)
	jne	.LBB40_58
.Ltmp6888:
	.loc	12 110 8
	vpxor	%xmm0, %xmm0, %xmm0
	vcmpeqss	%xmm0, %xmm1, %xmm0
	vandnps	%xmm1, %xmm0, %xmm0
.Ltmp6889:
	.loc	6 644 13
	movl	$1, (%rbx,%r15,8)
	vmovss	%xmm0, 4(%rbx,%r15,8)
.Ltmp6890:
	.loc	38 1714 9
	addq	$40, %r14
.Ltmp6891:
	.loc	19 180 28
	incq	%r12
	xorl	%r8d, %r8d
	movl	%r11d, 120(%rsp)
.Ltmp6892:
	.loc	38 1714 9
	cmpq	%rdx, %r14
.Ltmp6893:
	.loc	19 180 28
	jne	.LBB40_45
	jmp	.LBB40_10
.Ltmp6894:
.LBB40_60:
	.loc	6 1076 28
	vmovdqu	%ymm0, 288(%rdi)
	vmovdqu	%ymm0, 256(%rdi)
	vmovdqu	%ymm0, 224(%rdi)
	vmovdqu	%ymm0, 192(%rdi)
	vmovdqu	%ymm0, 160(%rdi)
	vmovdqu	%ymm0, 128(%rdi)
	vmovdqu	%ymm0, 96(%rdi)
	vmovdqu	%ymm0, 64(%rdi)
	vmovdqu	%ymm0, 32(%rdi)
	vmovdqu	%ymm0, (%rdi)
	movb	%al, 320(%rdi)
	jmp	.LBB40_144
.LBB40_61:
	.loc	6 0 28 is_stmt 0
	movq	72(%rsp), %rsi
.Ltmp6895:
	.loc	6 1091 30 is_stmt 1
	movl	104(%rsi), %eax
.Ltmp6896:
	.loc	6 1092 32
	movq	(%rsi), %rcx
	movq	%rcx, 56(%rsp)
	movq	8(%rsi), %rdx
	.loc	6 1092 44 is_stmt 0
	movq	16(%rsi), %rcx
	movq	%rcx, 80(%rsp)
	movq	24(%rsi), %r8
.Ltmp6897:
	.loc	6 682 25 is_stmt 1
	movl	1820(%r9), %ecx
.Ltmp6898:
	.loc	8 1078 5
	cmpl	%ecx, %eax
	movl	%ecx, %esi
	cmovbl	%eax, %esi
.Ltmp6899:
	.loc	6 683 12
	testl	%esi, %esi
	movq	%rax, 128(%rsp)
	movq	%rsi, 120(%rsp)
	movq	%rdx, 360(%rsp)
	movq	%r8, 352(%rsp)
	je	.LBB40_87
	.loc	6 684 25
	leaq	(,%rsi,4), %rsi
	cmpq	%rdx, %rsi
.Ltmp6900:
	.loc	15 1050 16
	ja	.LBB40_159
.Ltmp6901:
	.loc	25 451 16
	cmpq	%r8, %rsi
	ja	.LBB40_160
.Ltmp6902:
	.loc	25 0 16 is_stmt 0
	movl	%ecx, 136(%rsp)
.Ltmp6903:
	.loc	6 730 27 is_stmt 1
	movq	768(%r9), %r12
	movq	776(%r9), %rdx
	.loc	6 735 27
	movq	832(%r9), %r11
	movq	840(%r9), %rbp
	.loc	6 741 24
	movl	1812(%r9), %r8d
.Ltmp6904:
	.loc	21 238 16
	movl	1808(%r9), %r15d
.Ltmp6905:
	.loc	11 900 12
	movl	%r15d, %eax
	subl	1816(%r9), %eax
	movq	%rax, 72(%rsp)
	xorl	%r14d, %r14d
	vpxor	%xmm0, %xmm0, %xmm0
	xorl	%ecx, %ecx
	movq	%rbp, 48(%rsp)
	movq	%rdx, 96(%rsp)
	movq	%r15, 144(%rsp)
	vbroadcastss	.LCPI40_2(%rip), %xmm10
	vmovdqa	.LCPI40_4(%rip), %xmm12
	vbroadcastss	.LCPI40_5(%rip), %xmm9
.Ltmp6906:
	.loc	11 0 12 is_stmt 0
.Ltmp6907:
	.p2align	4
.LBB40_65:
	.loc	21 246 22 is_stmt 1
	leal	(%r15,%rcx), %edi
	andl	%r8d, %edi
.Ltmp6908:
	.loc	21 247 77
	leaq	4(,%rdi,4), %rsi
.Ltmp6909:
	.loc	21 246 21
	shlq	$2, %rdi
.Ltmp6910:
	.loc	25 451 16
	cmpq	%rdx, %rsi
	ja	.LBB40_147
.Ltmp6911:
	.loc	25 0 16 is_stmt 0
	movq	56(%rsp), %rax
	.loc	21 247 0 is_stmt 1
	leaq	(%rax,%r14), %r10
.Ltmp6912:
	.loc	1 551 14
	vmovups	(%r10), %xmm5
	vmovups	%xmm5, (%r12,%rdi,4)
.Ltmp6913:
	.loc	25 451 16
	cmpq	%rbp, %rsi
	ja	.LBB40_148
.Ltmp6914:
	.loc	25 0 16 is_stmt 0
	movq	80(%rsp), %rax
	.loc	21 248 0 is_stmt 1
	leaq	(%rax,%r14), %rbx
.Ltmp6915:
	.loc	1 551 14
	vmovups	(%rbx), %xmm5
	vmovups	%xmm5, (%r11,%rdi,4)
	movq	72(%rsp), %rax
.Ltmp6916:
	.loc	21 255 21
	leal	(%rax,%rcx), %r9d
	andl	%r8d, %r9d
.Ltmp6917:
	.loc	21 256 54
	leaq	4(,%r9,4), %rsi
.Ltmp6918:
	.loc	21 255 20
	shlq	$2, %r9
.Ltmp6919:
	.loc	25 438 16
	cmpq	%rdx, %rsi
	ja	.LBB40_149
.Ltmp6920:
	.loc	25 438 16 is_stmt 0
	cmpq	%rbp, %rsi
	ja	.LBB40_150
.Ltmp6921:
	.loc	21 271 24 is_stmt 1
	leal	(%r15,%rcx), %esi
	movl	%esi, %edi
	movq	40(%rsp), %rax
	subl	800(%rax), %edi
	andl	%r8d, %edi
	.loc	21 271 23 is_stmt 0
	shlq	$2, %rdi
.Ltmp6922:
	.loc	21 274 29 is_stmt 1
	cmpq	%rdx, %rdi
	jae	.LBB40_171
	.loc	21 275 29
	cmpq	%rbp, %rdi
	jae	.LBB40_180
.Ltmp6923:
	.loc	21 0 29 is_stmt 0
	movq	40(%rsp), %rax
	subl	864(%rax), %esi
	andl	%r8d, %esi
	shlq	$2, %rsi
.Ltmp6924:
	.loc	21 276 29 is_stmt 1
	cmpq	%rbp, %rsi
	jae	.LBB40_178
	.loc	21 0 29 is_stmt 0
	movq	%rbx, 88(%rsp)
	movq	%r10, 64(%rsp)
	.loc	21 277 29 is_stmt 1
	cmpq	%rdx, %rsi
	jae	.LBB40_175
.Ltmp6925:
	.loc	21 271 24
	leal	(%r15,%rcx), %eax
	movl	%eax, %ebx
	movq	40(%rsp), %r10
	subl	804(%r10), %ebx
	andl	%r8d, %ebx
	.loc	21 271 23 is_stmt 0
	leaq	1(,%rbx,4), %r13
.Ltmp6926:
	.loc	21 274 29 is_stmt 1
	cmpq	%rdx, %r13
	jae	.LBB40_168
	.loc	21 275 29
	cmpq	%rbp, %r13
	jae	.LBB40_166
.Ltmp6927:
	.loc	21 0 29 is_stmt 0
	movq	40(%rsp), %rbx
	subl	868(%rbx), %eax
	andl	%r8d, %eax
	leaq	1(,%rax,4), %rbx
.Ltmp6928:
	.loc	21 276 29 is_stmt 1
	cmpq	%rbp, %rbx
	jae	.LBB40_176
	.loc	21 0 29 is_stmt 0
	movq	%r14, 240(%rsp)
	.loc	21 277 29 is_stmt 1
	cmpq	%rdx, %rbx
	jae	.LBB40_173
.Ltmp6929:
	.loc	21 271 24
	leal	(%r15,%rcx), %eax
	movl	%eax, %r14d
	movq	40(%rsp), %r10
	subl	808(%r10), %r14d
	andl	%r8d, %r14d
	.loc	21 271 23 is_stmt 0
	leaq	2(,%r14,4), %r14
.Ltmp6930:
	.loc	21 274 29 is_stmt 1
	cmpq	%rdx, %r14
	jae	.LBB40_169
	.loc	21 275 29
	cmpq	%rbp, %r14
	jae	.LBB40_167
.Ltmp6931:
	.loc	21 0 29 is_stmt 0
	movq	40(%rsp), %r15
	subl	872(%r15), %eax
	andl	%r8d, %eax
	leaq	2(,%rax,4), %r15
.Ltmp6932:
	.loc	21 276 29 is_stmt 1
	cmpq	%rbp, %r15
	jae	.LBB40_177
	.loc	21 277 29
	cmpq	%rdx, %r15
	jae	.LBB40_174
.Ltmp6933:
	.loc	21 0 29 is_stmt 0
	movq	144(%rsp), %rax
	.loc	21 271 24 is_stmt 1
	leal	(%rax,%rcx), %ebp
	movl	%ebp, %eax
	movq	40(%rsp), %r10
	subl	812(%r10), %eax
	andl	%r8d, %eax
	.loc	21 271 23 is_stmt 0
	leaq	3(,%rax,4), %rax
.Ltmp6934:
	.loc	21 274 29 is_stmt 1
	cmpq	%rdx, %rax
	jae	.LBB40_170
	.loc	21 0 29 is_stmt 0
	movq	48(%rsp), %rdx
	.loc	21 275 29 is_stmt 1
	cmpq	%rdx, %rax
	jae	.LBB40_179
.Ltmp6935:
	.loc	21 0 29 is_stmt 0
	movq	40(%rsp), %r10
	subl	876(%r10), %ebp
	andl	%r8d, %ebp
	leaq	3(,%rbp,4), %rbp
.Ltmp6936:
	.loc	21 276 29 is_stmt 1
	cmpq	%rdx, %rbp
	jae	.LBB40_172
	.loc	21 277 29
	cmpq	96(%rsp), %rbp
	jae	.LBB40_165
.Ltmp6937:
	.loc	21 0 0 is_stmt 0
	vmovups	(%r12,%r9,4), %xmm1
	vmovaps	%xmm1, 208(%rsp)
.Ltmp6938:
	vmovups	(%r11,%r9,4), %xmm1
	vmovaps	%xmm1, 224(%rsp)
.Ltmp6939:
	.loc	1 551 14 is_stmt 1
	vmovd	(%r11,%rsi,4), %xmm5
.Ltmp6940:
	.loc	1 551 14 is_stmt 0
	vmovd	(%r12,%rsi,4), %xmm6
.Ltmp6941:
	.loc	1 551 14
	vpinsrd	$1, (%r11,%rbx,4), %xmm5, %xmm5
.Ltmp6942:
	.loc	1 551 14
	vpinsrd	$1, (%r12,%rbx,4), %xmm6, %xmm6
.Ltmp6943:
	.loc	1 551 14
	vpinsrd	$2, (%r11,%r15,4), %xmm5, %xmm5
.Ltmp6944:
	.loc	1 551 14
	vpinsrd	$2, (%r12,%r15,4), %xmm6, %xmm6
.Ltmp6945:
	.loc	21 0 0
	movl	(%r12,%rdi,4), %r9d
	movl	(%r11,%rdi,4), %esi
.Ltmp6946:
	.loc	1 551 14
	vpinsrd	$3, (%r11,%rbp,4), %xmm5, %xmm1
	vmovdqa	%xmm1, 160(%rsp)
.Ltmp6947:
	.loc	1 551 14
	vpinsrd	$3, (%r12,%rbp,4), %xmm6, %xmm1
	vmovdqa	%xmm1, 176(%rsp)
.Ltmp6948:
	.loc	21 0 0
	movl	(%r12,%r13,4), %ebx
	movl	(%r11,%r13,4), %edi
.Ltmp6949:
	.loc	1 551 14
	vmovd	%r9d, %xmm5
.Ltmp6950:
	.loc	21 0 0
	movl	(%r12,%r14,4), %ebp
	movl	(%r11,%r14,4), %r9d
.Ltmp6951:
	.loc	1 551 14
	vpinsrd	$1, %ebx, %xmm5, %xmm6
.Ltmp6952:
	.loc	21 0 0
	movl	(%r12,%rax,4), %ebx
	movl	(%r11,%rax,4), %eax
	movq	40(%rsp), %r14
.Ltmp6953:
	.loc	21 325 27 is_stmt 1
	vmovaps	1088(%r14), %xmm13
.Ltmp6954:
	.loc	21 323 26
	vmovaps	1136(%r14), %xmm7
.Ltmp6955:
	.loc	21 325 27
	vmovaps	1152(%r14), %xmm5
.Ltmp6956:
	.loc	21 323 26
	vmovaps	1200(%r14), %xmm14
.Ltmp6957:
	.file	48 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs"
	.loc	48 544 14
	vcmpltps	%xmm7, %xmm0, %xmm15
.Ltmp6958:
	.loc	48 36 14
	vaddps	1120(%r14), %xmm13, %xmm8
.Ltmp6959:
	.loc	48 504 14
	vcmpeqps	%xmm7, %xmm10, %xmm4
.Ltmp6960:
	.file	49 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/../../stdarch/crates/core_arch/src/x86/sse41.rs"
	.loc	49 131 19
	vblendvps	%xmm4, 1104(%r14), %xmm8, %xmm8
.Ltmp6961:
	.loc	21 326 13
	vmaskmovps	%xmm8, %xmm15, 1088(%r14)
	.loc	21 331 13
	vmaskmovps	%xmm0, %xmm4, 1120(%r14)
.Ltmp6962:
	.loc	1 551 14
	vpinsrd	$2, %ebp, %xmm6, %xmm4
	vbroadcastss	.LCPI40_3(%rip), %xmm2
.Ltmp6963:
	.loc	48 62 14
	vaddps	%xmm2, %xmm7, %xmm6
.Ltmp6964:
	.loc	21 332 13
	vmaskmovps	%xmm6, %xmm15, 1136(%r14)
.Ltmp6965:
	.loc	48 544 14
	vcmpltps	%xmm14, %xmm0, %xmm6
.Ltmp6966:
	.loc	48 36 14
	vaddps	1184(%r14), %xmm5, %xmm7
.Ltmp6967:
	.loc	48 504 14
	vcmpeqps	%xmm10, %xmm14, %xmm3
.Ltmp6968:
	.loc	49 131 19
	vblendvps	%xmm3, 1168(%r14), %xmm7, %xmm1
.Ltmp6969:
	.loc	21 326 13
	vmaskmovps	%xmm1, %xmm6, 1152(%r14)
	.loc	21 331 13
	vmaskmovps	%xmm0, %xmm3, 1184(%r14)
.Ltmp6970:
	.loc	1 551 14
	vpinsrd	$3, %ebx, %xmm4, %xmm7
.Ltmp6971:
	.loc	48 62 14
	vaddps	%xmm2, %xmm14, %xmm3
.Ltmp6972:
	.loc	21 332 13
	vmaskmovps	%xmm3, %xmm6, 1200(%r14)
.Ltmp6973:
	.loc	49 131 19
	vblendvps	%xmm15, %xmm8, %xmm13, %xmm13
.Ltmp6974:
	.loc	21 323 26
	vmovaps	1264(%r14), %xmm3
.Ltmp6975:
	.loc	48 544 14
	vcmpltps	%xmm3, %xmm0, %xmm4
.Ltmp6976:
	.loc	21 325 27
	vmovaps	1216(%r14), %xmm8
.Ltmp6977:
	.loc	48 36 14
	vaddps	1248(%r14), %xmm8, %xmm14
.Ltmp6978:
	.loc	48 504 14
	vcmpeqps	%xmm3, %xmm10, %xmm15
.Ltmp6979:
	.loc	49 131 19
	vblendvps	%xmm15, 1232(%r14), %xmm14, %xmm14
.Ltmp6980:
	.loc	21 326 13
	vmaskmovps	%xmm14, %xmm4, 1216(%r14)
	.loc	21 331 13
	vmaskmovps	%xmm0, %xmm15, 1248(%r14)
.Ltmp6981:
	.loc	1 551 14
	vmovd	%esi, %xmm15
	vpinsrd	$1, %edi, %xmm15, %xmm15
	vpinsrd	$2, %r9d, %xmm15, %xmm15
	vpinsrd	$3, %eax, %xmm15, %xmm15
.Ltmp6982:
	.loc	49 131 19
	vblendvps	%xmm6, %xmm1, %xmm5, %xmm5
.Ltmp6983:
	.loc	48 62 14
	vaddps	%xmm2, %xmm3, %xmm1
.Ltmp6984:
	.loc	21 332 13
	vmaskmovps	%xmm1, %xmm4, 1264(%r14)
.Ltmp6985:
	.loc	49 131 19
	vblendvps	%xmm4, %xmm14, %xmm8, %xmm14
.Ltmp6986:
	.loc	21 325 27
	vmovaps	1280(%r14), %xmm1
.Ltmp6987:
	.loc	21 323 26
	vmovaps	1328(%r14), %xmm3
.Ltmp6988:
	.loc	48 544 14
	vcmpltps	%xmm3, %xmm0, %xmm4
.Ltmp6989:
	.loc	48 36 14
	vaddps	1312(%r14), %xmm1, %xmm6
.Ltmp6990:
	.loc	48 504 14
	vcmpeqps	%xmm3, %xmm10, %xmm8
.Ltmp6991:
	.loc	49 131 19
	vblendvps	%xmm8, 1296(%r14), %xmm6, %xmm6
.Ltmp6992:
	.loc	21 326 13
	vmaskmovps	%xmm6, %xmm4, 1280(%r14)
	.loc	21 331 13
	vmaskmovps	%xmm0, %xmm8, 1312(%r14)
.Ltmp6993:
	.loc	49 131 19
	vblendvps	%xmm4, %xmm6, %xmm1, %xmm6
.Ltmp6994:
	.loc	48 62 14
	vaddps	%xmm2, %xmm3, %xmm1
.Ltmp6995:
	.loc	21 332 13
	vmaskmovps	%xmm1, %xmm4, 1328(%r14)
.Ltmp6996:
	.loc	48 544 14
	vcmpltps	976(%r14), %xmm0, %xmm1
.Ltmp6997:
	.loc	48 257 24
	vpand	%xmm7, %xmm12, %xmm3
.Ltmp6998:
	.loc	48 88 14
	vmulps	%xmm3, %xmm9, %xmm4
.Ltmp6999:
	.loc	48 257 24
	vpand	%xmm12, %xmm15, %xmm7
.Ltmp7000:
	.loc	48 88 14
	vmulps	%xmm7, %xmm9, %xmm8
.Ltmp7001:
	.loc	48 544 14
	vcmpltps	960(%r14), %xmm0, %xmm15
.Ltmp7002:
	.loc	48 36 14
	vaddps	%xmm4, %xmm8, %xmm4
.Ltmp7003:
	.loc	48 238 14
	vmaxps	%xmm7, %xmm3, %xmm7
.Ltmp7004:
	.loc	49 131 19
	vblendvps	%xmm15, %xmm7, %xmm3, %xmm3
.Ltmp7005:
	.loc	49 131 19 is_stmt 0
	vblendvps	%xmm1, %xmm4, %xmm3, %xmm1
	vbroadcastss	.LCPI40_6(%rip), %xmm3
.Ltmp7006:
	.loc	48 238 14 is_stmt 1
	vmaxps	%xmm3, %xmm1, %xmm1
	vbroadcastss	.LCPI40_7(%rip), %xmm3
.Ltmp7007:
	.loc	48 238 14 is_stmt 0
	vmaxps	%xmm3, %xmm1, %xmm1
	vbroadcastss	.LCPI40_36(%rip), %xmm3
.Ltmp7008:
	.file	50 "/rustc/8bab26f4f68e0e26f0bb7960be334d5b520ea452" "library/core/src/../../stdarch/crates/core_arch/src/x86/sse2.rs"
	.loc	50 863 14 is_stmt 1
	vandps	%xmm3, %xmm1, %xmm3
	vbroadcastss	.LCPI40_2(%rip), %xmm4
.Ltmp7009:
	.loc	50 889 14
	vorps	%xmm4, %xmm3, %xmm3
.Ltmp7010:
	.loc	48 62 14
	vaddps	%xmm2, %xmm3, %xmm3
	vbroadcastss	.LCPI40_10(%rip), %xmm4
.Ltmp7011:
	.loc	48 88 14
	vmulps	%xmm4, %xmm3, %xmm4
	vbroadcastss	.LCPI40_11(%rip), %xmm7
.Ltmp7012:
	.loc	48 36 14
	vsubps	%xmm4, %xmm7, %xmm4
.Ltmp7013:
	.loc	48 88 14
	vmulps	%xmm4, %xmm3, %xmm4
	vbroadcastss	.LCPI40_12(%rip), %xmm7
.Ltmp7014:
	.loc	48 36 14
	vaddps	%xmm7, %xmm4, %xmm4
.Ltmp7015:
	.loc	48 88 14
	vmulps	%xmm4, %xmm3, %xmm4
	vbroadcastss	.LCPI40_13(%rip), %xmm7
.Ltmp7016:
	.loc	48 36 14
	vaddps	%xmm7, %xmm4, %xmm4
.Ltmp7017:
	.loc	48 88 14
	vmulps	%xmm4, %xmm3, %xmm4
	vbroadcastss	.LCPI40_14(%rip), %xmm7
.Ltmp7018:
	.loc	48 36 14
	vaddps	%xmm7, %xmm4, %xmm4
.Ltmp7019:
	.loc	48 88 14
	vmulps	%xmm4, %xmm3, %xmm4
	vbroadcastss	.LCPI40_15(%rip), %xmm7
.Ltmp7020:
	.loc	48 36 14
	vaddps	%xmm7, %xmm4, %xmm4
.Ltmp7021:
	.loc	50 817 24
	vpsrld	$23, %xmm1, %xmm1
	vmovdqa	.LCPI40_16(%rip), %xmm7
.Ltmp7022:
	.loc	50 889 14
	vpor	%xmm7, %xmm1, %xmm1
	vbroadcastss	.LCPI40_17(%rip), %xmm7
.Ltmp7023:
	.loc	48 62 14
	vaddps	%xmm7, %xmm1, %xmm1
.Ltmp7024:
	.loc	48 88 14
	vmulps	%xmm4, %xmm3, %xmm3
.Ltmp7025:
	.loc	48 36 14
	vaddps	%xmm3, %xmm1, %xmm1
	vbroadcastss	.LCPI40_18(%rip), %xmm3
.Ltmp7026:
	.loc	48 88 14
	vmulps	%xmm3, %xmm1, %xmm1
	vbroadcastss	.LCPI40_19(%rip), %xmm3
.Ltmp7027:
	.loc	48 212 14
	vminps	%xmm3, %xmm1, %xmm1
	vbroadcastss	.LCPI40_20(%rip), %xmm3
.Ltmp7028:
	.loc	48 238 14
	vmaxps	%xmm3, %xmm1, %xmm1
.Ltmp7029:
	.loc	48 544 14
	vcmpltps	1344(%r14), %xmm0, %xmm3
.Ltmp7030:
	.loc	48 558 14
	vcmpleps	%xmm1, %xmm13, %xmm4
.Ltmp7031:
	.loc	48 62 14
	vsubps	%xmm6, %xmm13, %xmm6
.Ltmp7032:
	.loc	48 558 14
	vcmpleps	%xmm1, %xmm6, %xmm6
.Ltmp7033:
	.loc	48 257 24
	vandnps	%xmm4, %xmm3, %xmm4
.Ltmp7034:
	.loc	48 257 24 is_stmt 0
	vandps	%xmm3, %xmm6, %xmm7
.Ltmp7035:
	.loc	48 302 24 is_stmt 1
	vorps	%xmm4, %xmm7, %xmm4
.Ltmp7036:
	.loc	21 392 65
	vmovaps	912(%r14), %xmm8
.Ltmp7037:
	.loc	21 370 47
	vmovaps	1360(%r14), %xmm7
.Ltmp7038:
	.loc	48 544 14
	vcmpltps	%xmm7, %xmm0, %xmm15
.Ltmp7039:
	.loc	48 257 24
	vandnps	%xmm15, %xmm6, %xmm6
.Ltmp7040:
	.loc	21 392 36
	vmovaps	1376(%r14), %xmm15
.Ltmp7041:
	.loc	48 257 24
	vandps	%xmm3, %xmm6, %xmm3
.Ltmp7042:
	.loc	48 62 14
	vaddps	%xmm2, %xmm7, %xmm6
.Ltmp7043:
	.loc	49 131 19
	vblendvps	%xmm3, %xmm6, %xmm7, %xmm6
.Ltmp7044:
	.loc	21 325 27
	vmovaps	1392(%r14), %xmm7
.Ltmp7045:
	.loc	48 302 24
	vorps	%xmm4, %xmm3, %xmm3
.Ltmp7046:
	.loc	49 131 19
	vpcmpgtd	%xmm3, %xmm0, %xmm3
.Ltmp7047:
	.loc	49 131 19 is_stmt 0
	vblendvps	%xmm4, 928(%r14), %xmm6, %xmm4
.Ltmp7048:
	.loc	49 131 19
	vpand	%xmm3, %xmm10, %xmm3
.Ltmp7049:
	.loc	21 373 5 is_stmt 1
	vmovaps	%xmm4, 1360(%r14)
	.loc	21 382 5
	vmovdqa	%xmm3, 1344(%r14)
.Ltmp7050:
	.loc	48 62 14
	vaddps	%xmm2, %xmm5, %xmm4
.Ltmp7051:
	.loc	48 62 14 is_stmt 0
	vsubps	%xmm13, %xmm1, %xmm1
.Ltmp7052:
	.loc	48 88 14 is_stmt 1
	vmulps	%xmm1, %xmm4, %xmm1
	vbroadcastss	.LCPI40_37(%rip), %xmm4
.Ltmp7053:
	.loc	48 323 24
	vxorps	%xmm4, %xmm14, %xmm4
.Ltmp7054:
	.loc	48 238 14
	vmaxps	%xmm4, %xmm1, %xmm1
.Ltmp7055:
	.loc	48 212 14
	vminps	%xmm0, %xmm1, %xmm1
.Ltmp7056:
	.loc	48 544 14
	vcmpltps	%xmm3, %xmm0, %xmm3
.Ltmp7057:
	.loc	49 131 19
	vpcmpgtd	%xmm3, %xmm0, %xmm3
	vpandn	%xmm1, %xmm3, %xmm1
.Ltmp7058:
	.loc	48 544 14
	vcmpltps	%xmm1, %xmm15, %xmm3
.Ltmp7059:
	.loc	49 131 19
	vblendvps	%xmm3, 896(%r14), %xmm8, %xmm3
.Ltmp7060:
	.loc	48 62 14
	vsubps	%xmm15, %xmm1, %xmm1
.Ltmp7061:
	.loc	48 88 14
	vmulps	%xmm3, %xmm1, %xmm1
.Ltmp7062:
	.loc	48 36 14
	vaddps	%xmm1, %xmm15, %xmm1
.Ltmp7063:
	.loc	48 257 24
	vandps	%xmm1, %xmm12, %xmm3
	vbroadcastss	.LCPI40_22(%rip), %xmm4
.Ltmp7064:
	.loc	48 517 14
	vcmpltps	%xmm4, %xmm3, %xmm3
.Ltmp7065:
	.loc	48 257 24
	vandnps	%xmm1, %xmm3, %xmm5
.Ltmp7066:
	.loc	21 394 5
	vmovaps	%xmm5, 1376(%r14)
.Ltmp7067:
	.loc	21 323 26
	vmovaps	1440(%r14), %xmm1
.Ltmp7068:
	.loc	48 36 14
	vaddps	1424(%r14), %xmm7, %xmm3
.Ltmp7069:
	.loc	48 504 14
	vcmpeqps	%xmm1, %xmm10, %xmm4
.Ltmp7070:
	.loc	49 131 19
	vblendvps	%xmm4, 1408(%r14), %xmm3, %xmm3
.Ltmp7071:
	.loc	48 544 14
	vcmpltps	%xmm1, %xmm0, %xmm8
.Ltmp7072:
	.loc	48 544 14 is_stmt 0
	vcmpltps	944(%r14), %xmm0, %xmm6
.Ltmp7073:
	.loc	21 326 13 is_stmt 1
	vmaskmovps	%xmm3, %xmm8, 1392(%r14)
	.loc	21 331 13
	vmaskmovps	%xmm0, %xmm4, 1424(%r14)
.Ltmp7074:
	.loc	48 62 14
	vaddps	%xmm2, %xmm1, %xmm1
.Ltmp7075:
	.loc	21 332 13
	vmaskmovps	%xmm1, %xmm8, 1440(%r14)
.Ltmp7076:
	.loc	21 325 27
	vmovaps	1456(%r14), %xmm1
.Ltmp7077:
	.loc	21 323 26
	vmovaps	1504(%r14), %xmm4
.Ltmp7078:
	.loc	48 544 14
	vcmpltps	%xmm4, %xmm0, %xmm13
.Ltmp7079:
	.loc	48 36 14
	vaddps	1488(%r14), %xmm1, %xmm14
.Ltmp7080:
	.loc	48 504 14
	vcmpeqps	%xmm4, %xmm10, %xmm15
.Ltmp7081:
	.loc	49 131 19
	vblendvps	%xmm15, 1472(%r14), %xmm14, %xmm11
.Ltmp7082:
	.loc	21 326 13
	vmaskmovps	%xmm11, %xmm13, 1456(%r14)
	.loc	21 331 13
	vmaskmovps	%xmm0, %xmm15, 1488(%r14)
.Ltmp7083:
	.loc	49 131 19
	vblendvps	%xmm8, %xmm3, %xmm7, %xmm14
.Ltmp7084:
	.loc	48 62 14
	vaddps	%xmm2, %xmm4, %xmm3
.Ltmp7085:
	.loc	21 332 13
	vmaskmovps	%xmm3, %xmm13, 1504(%r14)
.Ltmp7086:
	.loc	49 131 19
	vblendvps	%xmm13, %xmm11, %xmm1, %xmm13
.Ltmp7087:
	.loc	21 325 27
	vmovaps	1520(%r14), %xmm1
.Ltmp7088:
	.loc	21 323 26
	vmovaps	1568(%r14), %xmm3
.Ltmp7089:
	.loc	48 544 14
	vcmpltps	%xmm3, %xmm0, %xmm4
.Ltmp7090:
	.loc	48 36 14
	vaddps	1552(%r14), %xmm1, %xmm7
.Ltmp7091:
	.loc	48 504 14
	vcmpeqps	%xmm3, %xmm10, %xmm8
.Ltmp7092:
	.loc	49 131 19
	vblendvps	%xmm8, 1536(%r14), %xmm7, %xmm7
.Ltmp7093:
	.loc	21 326 13
	vmaskmovps	%xmm7, %xmm4, 1520(%r14)
	.loc	21 331 13
	vmaskmovps	%xmm0, %xmm8, 1552(%r14)
.Ltmp7094:
	.loc	49 131 19
	vblendvps	%xmm4, %xmm7, %xmm1, %xmm15
.Ltmp7095:
	.loc	48 62 14
	vaddps	%xmm2, %xmm3, %xmm1
.Ltmp7096:
	.loc	21 332 13
	vmaskmovps	%xmm1, %xmm4, 1568(%r14)
.Ltmp7097:
	.loc	21 325 27
	vmovaps	1584(%r14), %xmm1
.Ltmp7098:
	.loc	21 323 26
	vmovaps	1632(%r14), %xmm3
.Ltmp7099:
	.loc	48 36 14
	vaddps	1616(%r14), %xmm1, %xmm4
.Ltmp7100:
	.loc	48 504 14
	vcmpeqps	%xmm3, %xmm10, %xmm8
.Ltmp7101:
	.loc	49 131 19
	vblendvps	%xmm8, 1600(%r14), %xmm4, %xmm4
.Ltmp7102:
	.loc	48 544 14
	vcmpltps	%xmm3, %xmm0, %xmm11
.Ltmp7103:
	.loc	49 131 19
	vblendvps	%xmm11, %xmm4, %xmm1, %xmm7
.Ltmp7104:
	.loc	21 326 13
	vmaskmovps	%xmm4, %xmm11, 1584(%r14)
	.loc	21 331 13
	vmaskmovps	%xmm0, %xmm8, 1616(%r14)
	vbroadcastss	.LCPI40_23(%rip), %xmm1
.Ltmp7105:
	.loc	48 88 14
	vmulps	%xmm1, %xmm5, %xmm1
	vbroadcastss	.LCPI40_24(%rip), %xmm4
.Ltmp7106:
	.loc	48 238 14
	vmaxps	%xmm4, %xmm1, %xmm1
	vbroadcastss	.LCPI40_25(%rip), %xmm4
.Ltmp7107:
	.loc	48 212 14
	vminps	%xmm4, %xmm1, %xmm1
.Ltmp7108:
	.loc	48 62 14
	vaddps	%xmm2, %xmm3, %xmm3
.Ltmp7109:
	.loc	21 332 13
	vmaskmovps	%xmm3, %xmm11, 1632(%r14)
.Ltmp7110:
	.loc	49 724 14
	vroundps	$9, %xmm1, %xmm3
.Ltmp7111:
	.loc	48 62 14
	vsubps	%xmm3, %xmm1, %xmm1
	vbroadcastss	.LCPI40_26(%rip), %xmm4
.Ltmp7112:
	.loc	48 88 14
	vmulps	%xmm4, %xmm1, %xmm4
	vbroadcastss	.LCPI40_27(%rip), %xmm8
.Ltmp7113:
	.loc	48 36 14
	vaddps	%xmm4, %xmm8, %xmm4
.Ltmp7114:
	.loc	48 88 14
	vmulps	%xmm4, %xmm1, %xmm4
	vbroadcastss	.LCPI40_28(%rip), %xmm8
.Ltmp7115:
	.loc	48 36 14
	vaddps	%xmm4, %xmm8, %xmm4
.Ltmp7116:
	.loc	48 88 14
	vmulps	%xmm4, %xmm1, %xmm4
	vbroadcastss	.LCPI40_29(%rip), %xmm8
.Ltmp7117:
	.loc	48 36 14
	vaddps	%xmm4, %xmm8, %xmm4
.Ltmp7118:
	.loc	48 88 14
	vmulps	%xmm4, %xmm1, %xmm4
	vbroadcastss	.LCPI40_30(%rip), %xmm8
.Ltmp7119:
	.loc	48 36 14
	vaddps	%xmm4, %xmm8, %xmm4
.Ltmp7120:
	.loc	48 88 14
	vmulps	%xmm4, %xmm1, %xmm1
.Ltmp7121:
	.loc	48 36 14
	vaddps	%xmm1, %xmm10, %xmm1
	vbroadcastss	.LCPI40_31(%rip), %xmm4
.Ltmp7122:
	.loc	48 36 14 is_stmt 0
	vaddps	%xmm4, %xmm3, %xmm3
.Ltmp7123:
	.loc	50 613 24 is_stmt 1
	vpslld	$23, %xmm3, %xmm3
.Ltmp7124:
	.loc	48 88 14
	vmulps	%xmm3, %xmm1, %xmm1
.Ltmp7125:
	.loc	48 504 14
	vcmpeqps	%xmm0, %xmm5, %xmm3
.Ltmp7126:
	.loc	48 302 24
	vorps	%xmm3, %xmm6, %xmm3
	vmovaps	208(%rsp), %xmm4
.Ltmp7127:
	.loc	48 88 14
	vmulps	%xmm1, %xmm4, %xmm1
.Ltmp7128:
	.loc	49 131 19
	vblendvps	%xmm3, %xmm4, %xmm1, %xmm11
.Ltmp7129:
	.loc	48 544 14
	vcmpltps	1072(%r14), %xmm0, %xmm1
.Ltmp7130:
	.loc	48 257 24
	vandps	160(%rsp), %xmm12, %xmm3
.Ltmp7131:
	.loc	48 88 14
	vmulps	%xmm3, %xmm9, %xmm4
.Ltmp7132:
	.loc	48 257 24
	vandps	176(%rsp), %xmm12, %xmm5
.Ltmp7133:
	.loc	48 88 14
	vmulps	%xmm5, %xmm9, %xmm6
.Ltmp7134:
	.loc	48 544 14
	vcmpltps	1056(%r14), %xmm0, %xmm8
.Ltmp7135:
	.loc	48 36 14
	vaddps	%xmm6, %xmm4, %xmm4
.Ltmp7136:
	.loc	48 238 14
	vmaxps	%xmm5, %xmm3, %xmm5
.Ltmp7137:
	.loc	49 131 19
	vblendvps	%xmm8, %xmm5, %xmm3, %xmm3
.Ltmp7138:
	.loc	49 131 19 is_stmt 0
	vblendvps	%xmm1, %xmm4, %xmm3, %xmm1
.Ltmp7139:
	.loc	48 238 14 is_stmt 1
	vbroadcastss	.LCPI40_6(%rip), %xmm3
	vmaxps	%xmm3, %xmm1, %xmm1
.Ltmp7140:
	.loc	48 238 14 is_stmt 0
	vbroadcastss	.LCPI40_7(%rip), %xmm3
	vmaxps	%xmm3, %xmm1, %xmm1
.Ltmp7141:
	.loc	50 863 14 is_stmt 1
	vandps	.LCPI40_8(%rip), %xmm1, %xmm3
.Ltmp7142:
	.loc	50 889 14
	vorps	.LCPI40_9(%rip), %xmm3, %xmm3
.Ltmp7143:
	.loc	48 62 14
	vaddps	%xmm2, %xmm3, %xmm3
.Ltmp7144:
	.loc	48 88 14
	vbroadcastss	.LCPI40_10(%rip), %xmm4
	vmulps	%xmm4, %xmm3, %xmm4
.Ltmp7145:
	.loc	48 36 14
	vbroadcastss	.LCPI40_11(%rip), %xmm5
	vsubps	%xmm4, %xmm5, %xmm4
.Ltmp7146:
	.loc	48 88 14
	vmulps	%xmm4, %xmm3, %xmm4
.Ltmp7147:
	.loc	48 36 14
	vbroadcastss	.LCPI40_12(%rip), %xmm5
	vaddps	%xmm5, %xmm4, %xmm4
.Ltmp7148:
	.loc	48 88 14
	vmulps	%xmm4, %xmm3, %xmm4
.Ltmp7149:
	.loc	48 36 14
	vbroadcastss	.LCPI40_13(%rip), %xmm5
	vaddps	%xmm5, %xmm4, %xmm4
.Ltmp7150:
	.loc	48 88 14
	vmulps	%xmm4, %xmm3, %xmm4
.Ltmp7151:
	.loc	48 36 14
	vbroadcastss	.LCPI40_14(%rip), %xmm5
	vaddps	%xmm5, %xmm4, %xmm4
.Ltmp7152:
	.loc	48 88 14
	vmulps	%xmm4, %xmm3, %xmm4
.Ltmp7153:
	.loc	48 36 14
	vbroadcastss	.LCPI40_15(%rip), %xmm5
	vaddps	%xmm5, %xmm4, %xmm4
.Ltmp7154:
	.loc	50 817 24
	vpsrld	$23, %xmm1, %xmm1
.Ltmp7155:
	.loc	50 889 14
	vpor	.LCPI40_16(%rip), %xmm1, %xmm1
.Ltmp7156:
	.loc	48 62 14
	vbroadcastss	.LCPI40_17(%rip), %xmm5
	vaddps	%xmm5, %xmm1, %xmm1
.Ltmp7157:
	.loc	48 88 14
	vmulps	%xmm4, %xmm3, %xmm3
.Ltmp7158:
	.loc	48 36 14
	vaddps	%xmm3, %xmm1, %xmm1
.Ltmp7159:
	.loc	48 88 14
	vbroadcastss	.LCPI40_18(%rip), %xmm3
	vmulps	%xmm3, %xmm1, %xmm1
.Ltmp7160:
	.loc	48 212 14
	vbroadcastss	.LCPI40_19(%rip), %xmm3
	vminps	%xmm3, %xmm1, %xmm1
.Ltmp7161:
	.loc	48 238 14
	vbroadcastss	.LCPI40_20(%rip), %xmm3
	vmaxps	%xmm3, %xmm1, %xmm1
.Ltmp7162:
	.loc	48 544 14
	vcmpltps	1648(%r14), %xmm0, %xmm3
.Ltmp7163:
	.loc	48 558 14
	vcmpleps	%xmm1, %xmm14, %xmm4
.Ltmp7164:
	.loc	48 62 14
	vsubps	%xmm7, %xmm14, %xmm5
.Ltmp7165:
	.loc	48 558 14
	vcmpleps	%xmm1, %xmm5, %xmm5
.Ltmp7166:
	.loc	48 257 24
	vandnps	%xmm4, %xmm3, %xmm4
.Ltmp7167:
	.loc	48 257 24 is_stmt 0
	vandps	%xmm3, %xmm5, %xmm6
.Ltmp7168:
	.loc	48 302 24 is_stmt 1
	vorps	%xmm4, %xmm6, %xmm4
.Ltmp7169:
	.loc	21 370 47
	vmovaps	1664(%r14), %xmm6
.Ltmp7170:
	.loc	48 544 14
	vcmpltps	%xmm6, %xmm0, %xmm7
.Ltmp7171:
	.loc	48 257 24
	vandnps	%xmm7, %xmm5, %xmm5
	vandps	%xmm3, %xmm5, %xmm3
.Ltmp7172:
	.loc	48 62 14
	vaddps	%xmm2, %xmm6, %xmm5
.Ltmp7173:
	.loc	49 131 19
	vblendvps	%xmm3, %xmm5, %xmm6, %xmm5
.Ltmp7174:
	.loc	48 302 24
	vorps	%xmm4, %xmm3, %xmm3
.Ltmp7175:
	.loc	49 131 19
	vblendvps	%xmm4, 1024(%r14), %xmm5, %xmm4
.Ltmp7176:
	.loc	21 373 5
	vmovaps	%xmm4, 1664(%r14)
.Ltmp7177:
	.loc	48 62 14
	vsubps	%xmm14, %xmm1, %xmm1
.Ltmp7178:
	.loc	49 131 19
	vpcmpgtd	%xmm3, %xmm0, %xmm3
	vpand	%xmm3, %xmm10, %xmm3
.Ltmp7179:
	.loc	21 382 5
	vmovdqa	%xmm3, 1648(%r14)
.Ltmp7180:
	.loc	48 62 14
	vaddps	%xmm2, %xmm13, %xmm4
.Ltmp7181:
	.loc	48 88 14
	vmulps	%xmm1, %xmm4, %xmm1
.Ltmp7182:
	.loc	21 392 36
	vmovaps	1680(%r14), %xmm4
.Ltmp7183:
	.loc	48 323 24
	vxorps	.LCPI40_21(%rip), %xmm15, %xmm5
.Ltmp7184:
	.loc	48 238 14
	vmaxps	%xmm5, %xmm1, %xmm1
.Ltmp7185:
	.loc	21 392 65
	vmovaps	1008(%r14), %xmm5
.Ltmp7186:
	.loc	48 212 14
	vminps	%xmm0, %xmm1, %xmm1
.Ltmp7187:
	.loc	48 544 14
	vcmpltps	%xmm3, %xmm0, %xmm3
.Ltmp7188:
	.loc	49 131 19
	vpcmpgtd	%xmm3, %xmm0, %xmm3
	vpandn	%xmm1, %xmm3, %xmm1
.Ltmp7189:
	.loc	48 544 14
	vcmpltps	%xmm1, %xmm4, %xmm3
.Ltmp7190:
	.loc	49 131 19
	vblendvps	%xmm3, 992(%r14), %xmm5, %xmm3
.Ltmp7191:
	.loc	48 62 14
	vsubps	%xmm4, %xmm1, %xmm1
.Ltmp7192:
	.loc	48 88 14
	vmulps	%xmm3, %xmm1, %xmm1
.Ltmp7193:
	.loc	48 36 14
	vaddps	%xmm1, %xmm4, %xmm1
.Ltmp7194:
	.loc	48 257 24
	vandps	%xmm1, %xmm12, %xmm3
.Ltmp7195:
	.loc	48 517 14
	vbroadcastss	.LCPI40_22(%rip), %xmm2
	vcmpltps	%xmm2, %xmm3, %xmm3
.Ltmp7196:
	.loc	48 257 24
	vandnps	%xmm1, %xmm3, %xmm1
.Ltmp7197:
	.loc	48 88 14
	vbroadcastss	.LCPI40_23(%rip), %xmm2
	vmulps	%xmm2, %xmm1, %xmm3
.Ltmp7198:
	.loc	48 238 14
	vbroadcastss	.LCPI40_24(%rip), %xmm2
	vmaxps	%xmm2, %xmm3, %xmm3
.Ltmp7199:
	.loc	48 212 14
	vbroadcastss	.LCPI40_25(%rip), %xmm2
	vminps	%xmm2, %xmm3, %xmm3
.Ltmp7200:
	.loc	49 724 14
	vroundps	$9, %xmm3, %xmm4
.Ltmp7201:
	.loc	48 62 14
	vsubps	%xmm4, %xmm3, %xmm3
.Ltmp7202:
	.loc	48 88 14
	vbroadcastss	.LCPI40_26(%rip), %xmm2
	vmulps	%xmm2, %xmm3, %xmm5
.Ltmp7203:
	.loc	48 36 14
	vbroadcastss	.LCPI40_27(%rip), %xmm2
	vaddps	%xmm2, %xmm5, %xmm5
.Ltmp7204:
	.loc	48 88 14
	vmulps	%xmm5, %xmm3, %xmm5
.Ltmp7205:
	.loc	48 36 14
	vbroadcastss	.LCPI40_28(%rip), %xmm2
	vaddps	%xmm2, %xmm5, %xmm5
.Ltmp7206:
	.loc	48 88 14
	vmulps	%xmm5, %xmm3, %xmm5
.Ltmp7207:
	.loc	48 36 14
	vbroadcastss	.LCPI40_29(%rip), %xmm2
	vaddps	%xmm2, %xmm5, %xmm5
.Ltmp7208:
	.loc	48 88 14
	vmulps	%xmm5, %xmm3, %xmm5
.Ltmp7209:
	.loc	48 36 14
	vbroadcastss	.LCPI40_30(%rip), %xmm2
	vaddps	%xmm2, %xmm5, %xmm5
.Ltmp7210:
	.loc	48 88 14
	vmulps	%xmm5, %xmm3, %xmm3
.Ltmp7211:
	.loc	21 394 5
	vmovaps	%xmm1, 1680(%r14)
.Ltmp7212:
	.loc	48 36 14
	vaddps	%xmm3, %xmm10, %xmm3
.Ltmp7213:
	.loc	48 36 14 is_stmt 0
	vbroadcastss	.LCPI40_31(%rip), %xmm2
	vaddps	%xmm2, %xmm4, %xmm4
.Ltmp7214:
	.loc	50 613 24 is_stmt 1
	vpslld	$23, %xmm4, %xmm4
.Ltmp7215:
	.loc	48 88 14
	vmulps	%xmm4, %xmm3, %xmm3
.Ltmp7216:
	.loc	48 504 14
	vcmpeqps	%xmm0, %xmm1, %xmm1
.Ltmp7217:
	.loc	48 544 14
	vcmpltps	1040(%r14), %xmm0, %xmm4
.Ltmp7218:
	.loc	48 302 24
	vorps	%xmm1, %xmm4, %xmm1
	vmovaps	224(%rsp), %xmm2
.Ltmp7219:
	.loc	48 88 14
	vmulps	%xmm3, %xmm2, %xmm3
.Ltmp7220:
	.loc	49 131 19
	vblendvps	%xmm1, %xmm2, %xmm3, %xmm1
	movq	64(%rsp), %rax
.Ltmp7221:
	.loc	1 551 14
	vmovups	%xmm11, (%rax)
	movq	88(%rsp), %rax
.Ltmp7222:
	.loc	1 551 14 is_stmt 0
	vmovups	%xmm1, (%rax)
.Ltmp7223:
	.loc	21 0 0
	incq	%rcx
	movq	240(%rsp), %r14
.Ltmp7224:
	.loc	8 1916 50 is_stmt 1
	addq	$16, %r14
	movq	120(%rsp), %rsi
	cmpq	%rcx, %rsi
	movq	96(%rsp), %rdx
	movq	48(%rsp), %rbp
	movq	144(%rsp), %r15
.Ltmp7225:
	.loc	11 900 12
	jne	.LBB40_65
.Ltmp7226:
	.loc	15 2584 13
	leal	(%rsi,%r15), %eax
	movq	40(%rsp), %r9
.Ltmp7227:
	.loc	21 297 5
	movl	%eax, 1808(%r9)
	movq	128(%rsp), %rax
	movq	360(%rsp), %rdx
	movq	352(%rsp), %r8
	movl	136(%rsp), %ecx
.Ltmp7228:
.LBB40_87:
	.loc	6 688 12
	cmpl	%ecx, %eax
	jbe	.LBB40_115
	.loc	6 689 25
	leaq	(,%rsi,4), %rdi
.Ltmp7229:
	.loc	25 580 12
	movq	%rdx, %r10
	subq	%rdi, %r10
	jb	.LBB40_161
.Ltmp7230:
	.loc	25 580 12 is_stmt 0
	movq	%r8, %rdx
	subq	%rdi, %rdx
	jb	.LBB40_162
.Ltmp7231:
	.loc	25 0 12
	movq	56(%rsp), %rcx
.Ltmp7232:
	.loc	25 101 24 is_stmt 1
	leaq	(%rcx,%rdi,4), %rcx
	movq	%rcx, 136(%rsp)
	movq	80(%rsp), %rcx
.Ltmp7233:
	.loc	25 101 24 is_stmt 0
	leaq	(%rcx,%rdi,4), %rcx
	movq	%rcx, 280(%rsp)
.Ltmp7234:
	.loc	6 695 17 is_stmt 1
	subq	%rsi, %rax
	movq	%rax, 160(%rsp)
.Ltmp7235:
	.loc	21 238 16
	movl	1808(%r9), %ecx
	movq	%rdx, 392(%rsp)
.Ltmp7236:
	.loc	11 900 12
	shrq	$2, %rdx
	movq	%rdx, 192(%rsp)
	vmovaps	1088(%r9), %xmm0
	vmovaps	%xmm0, 768(%rsp)
	vsubps	1280(%r9), %xmm0, %xmm0
	vmovaps	%xmm0, 752(%rsp)
	movq	%r10, 400(%rsp)
	vbroadcastss	.LCPI40_3(%rip), %xmm1
	vaddps	1152(%r9), %xmm1, %xmm0
	vmovaps	%xmm0, 736(%rsp)
	shrq	$2, %r10
	movq	%r10, 72(%rsp)
	vbroadcastss	.LCPI40_37(%rip), %xmm0
	vxorps	1216(%r9), %xmm0, %xmm2
	vmovaps	%xmm2, 720(%rsp)
	vmovaps	928(%r9), %xmm2
	vmovaps	%xmm2, 704(%rsp)
	vmovaps	1392(%r9), %xmm2
	vmovaps	%xmm2, 688(%rsp)
	vsubps	1584(%r9), %xmm2, %xmm2
	vmovaps	%xmm2, 672(%rsp)
	vaddps	1456(%r9), %xmm1, %xmm1
	vmovaps	%xmm1, 656(%rsp)
	vxorps	1520(%r9), %xmm0, %xmm0
	vmovaps	%xmm0, 640(%rsp)
	vmovaps	912(%r9), %xmm0
	vmovaps	%xmm0, 624(%rsp)
	movl	%ecx, %eax
	subl	876(%r9), %eax
	movq	%rax, 416(%rsp)
	vmovaps	896(%r9), %xmm0
	vmovaps	%xmm0, 608(%rsp)
	movl	%ecx, %eax
	subl	812(%r9), %eax
	movq	%rax, 424(%rsp)
	vmovaps	1024(%r9), %xmm0
	vmovaps	%xmm0, 592(%rsp)
	movl	%ecx, %eax
	subl	872(%r9), %eax
	movq	%rax, 432(%rsp)
	vmovaps	1008(%r9), %xmm0
	vmovaps	%xmm0, 576(%rsp)
	movl	%ecx, %eax
	subl	808(%r9), %eax
	movq	%rax, 440(%rsp)
	vmovaps	992(%r9), %xmm0
	vmovaps	%xmm0, 560(%rsp)
	movl	%ecx, %eax
	subl	868(%r9), %eax
	movq	%rax, 448(%rsp)
	movl	%ecx, %eax
	subl	804(%r9), %eax
	movq	%rax, 368(%rsp)
	movl	%ecx, %eax
	subl	864(%r9), %eax
	movq	%rax, 256(%rsp)
	movl	1816(%r9), %eax
	movl	%ecx, %edx
	subl	%eax, %edx
	movq	%rdx, 272(%rsp)
	movq	%rcx, 176(%rsp)
	subl	800(%r9), %ecx
	movq	%rcx, 264(%rsp)
	vmovaps	1344(%r9), %xmm0
	vmovaps	%xmm0, 96(%rsp)
.Ltmp7237:
	.loc	6 730 27
	movq	768(%r9), %r8
	movq	776(%r9), %rbx
	.loc	6 735 27
	movq	832(%r9), %rcx
	movq	840(%r9), %r10
	.loc	6 741 24
	movl	1812(%r9), %eax
	xorl	%edi, %edi
	xorl	%r13d, %r13d
	vmovaps	1360(%r9), %xmm0
	vmovaps	%xmm0, 144(%rsp)
	vmovaps	1648(%r9), %xmm11
	vmovaps	1664(%r9), %xmm3
	vxorps	%xmm2, %xmm2, %xmm2
	vcmpltps	976(%r9), %xmm2, %xmm0
	vmovaps	%xmm0, 544(%rsp)
	vcmpltps	960(%r9), %xmm2, %xmm0
	vmovaps	%xmm0, 528(%rsp)
	vcmpltps	944(%r9), %xmm2, %xmm0
	vmovaps	%xmm0, 512(%rsp)
	vcmpltps	1072(%r9), %xmm2, %xmm0
	vmovaps	%xmm0, 496(%rsp)
	vcmpltps	1056(%r9), %xmm2, %xmm0
	vmovaps	%xmm0, 480(%rsp)
	vcmpltps	1040(%r9), %xmm2, %xmm0
	vmovaps	%xmm0, 464(%rsp)
	movq	%rbx, 48(%rsp)
	vbroadcastss	.LCPI40_3(%rip), %xmm12
	vxorps	%xmm9, %xmm9, %xmm9
	.loc	6 0 24 is_stmt 0
.Ltmp7238:
	.p2align	4
.LBB40_91:
.Ltmp7239:
	.loc	15 1050 16 is_stmt 1
	cmpq	%r13, 72(%rsp)
	je	.LBB40_146
.Ltmp7240:
	.loc	15 0 16 is_stmt 0
	movq	176(%rsp), %rdx
	leal	(%rdx,%r13), %r11d
	andl	%eax, %r11d
	shlq	$2, %r11
.Ltmp7241:
	.loc	21 247 77 is_stmt 1
	leaq	4(%r11), %rsi
.Ltmp7242:
	.loc	25 451 16
	cmpq	%rbx, %rsi
	ja	.LBB40_153
.Ltmp7243:
	.loc	25 0 16 is_stmt 0
	movq	136(%rsp), %rdx
	.loc	21 247 0 is_stmt 1
	leaq	(%rdx,%rdi,4), %r9
.Ltmp7244:
	.loc	1 551 14
	vmovups	(%r9), %xmm0
	vmovups	%xmm0, (%r8,%r11,4)
.Ltmp7245:
	.loc	25 438 16
	cmpq	%r13, 192(%rsp)
	je	.LBB40_154
.Ltmp7246:
	.loc	25 451 16
	cmpq	%r10, %rsi
	ja	.LBB40_155
.Ltmp7247:
	.loc	25 0 16 is_stmt 0
	movq	280(%rsp), %rdx
	.loc	21 248 0 is_stmt 1
	leaq	(%rdx,%rdi,4), %rbp
.Ltmp7248:
	.loc	1 551 14
	vmovups	(%rbp), %xmm0
	vmovups	%xmm0, (%rcx,%r11,4)
	movq	272(%rsp), %rdx
.Ltmp7249:
	.loc	21 255 21
	leal	(%rdx,%r13), %r15d
	andl	%eax, %r15d
.Ltmp7250:
	.loc	21 256 54
	leaq	4(,%r15,4), %rsi
.Ltmp7251:
	.loc	21 255 20
	shlq	$2, %r15
.Ltmp7252:
	.loc	25 438 16
	cmpq	%rbx, %rsi
	ja	.LBB40_156
.Ltmp7253:
	.loc	25 438 16 is_stmt 0
	cmpq	%r10, %rsi
	ja	.LBB40_157
.Ltmp7254:
	.loc	25 0 16
	movq	264(%rsp), %rdx
.Ltmp7255:
	.loc	21 271 24 is_stmt 1
	leal	(%rdx,%r13), %r11d
	andl	%eax, %r11d
	.loc	21 271 23 is_stmt 0
	shlq	$2, %r11
.Ltmp7256:
	.loc	21 274 29 is_stmt 1
	cmpq	%rbx, %r11
	jae	.LBB40_196
	.loc	21 275 29
	cmpq	%r10, %r11
	jae	.LBB40_189
.Ltmp7257:
	.loc	21 0 29 is_stmt 0
	movq	256(%rsp), %rdx
	leal	(%rdx,%r13), %esi
	andl	%eax, %esi
	shlq	$2, %rsi
.Ltmp7258:
	.loc	21 276 29 is_stmt 1
	cmpq	%r10, %rsi
	jae	.LBB40_194
	.loc	21 277 29
	cmpq	%rbx, %rsi
	jae	.LBB40_192
.Ltmp7259:
	.loc	21 0 29 is_stmt 0
	movq	368(%rsp), %rdx
	.loc	21 271 24 is_stmt 1
	addl	%r13d, %edx
	andl	%eax, %edx
	.loc	21 271 23 is_stmt 0
	leaq	1(,%rdx,4), %r12
.Ltmp7260:
	.loc	21 274 29 is_stmt 1
	cmpq	%rbx, %r12
	jae	.LBB40_186
	.loc	21 275 29
	cmpq	%r10, %r12
	jae	.LBB40_181
.Ltmp7261:
	.loc	21 0 29 is_stmt 0
	movq	448(%rsp), %rdx
	addl	%r13d, %edx
	andl	%eax, %edx
	leaq	1(,%rdx,4), %r14
.Ltmp7262:
	.loc	21 276 29 is_stmt 1
	cmpq	%r10, %r14
	jae	.LBB40_190
	.loc	21 277 29
	cmpq	%rbx, %r14
	jae	.LBB40_184
.Ltmp7263:
	.loc	21 0 29 is_stmt 0
	movq	440(%rsp), %rdx
	.loc	21 271 24 is_stmt 1
	addl	%r13d, %edx
	andl	%eax, %edx
	.loc	21 271 23 is_stmt 0
	leaq	2(,%rdx,4), %rdx
.Ltmp7264:
	.loc	21 274 29 is_stmt 1
	cmpq	%rbx, %rdx
	jae	.LBB40_195
	.loc	21 0 29 is_stmt 0
	movq	%rbp, 88(%rsp)
	movq	%r9, 64(%rsp)
	.loc	21 275 29 is_stmt 1
	cmpq	%r10, %rdx
	jae	.LBB40_187
.Ltmp7265:
	.loc	21 0 29 is_stmt 0
	movq	432(%rsp), %r9
	addl	%r13d, %r9d
	andl	%eax, %r9d
	leaq	2(,%r9,4), %rbp
.Ltmp7266:
	.loc	21 276 29 is_stmt 1
	cmpq	%r10, %rbp
	jae	.LBB40_183
	.loc	21 277 29
	cmpq	%rbx, %rbp
	jae	.LBB40_191
.Ltmp7267:
	.loc	21 0 29 is_stmt 0
	movq	424(%rsp), %r9
	.loc	21 271 24 is_stmt 1
	addl	%r13d, %r9d
	andl	%eax, %r9d
	.loc	21 271 23 is_stmt 0
	leaq	3(,%r9,4), %r9
.Ltmp7268:
	.loc	21 274 29 is_stmt 1
	cmpq	%rbx, %r9
	jae	.LBB40_185
	.loc	21 275 29
	cmpq	%r10, %r9
	jae	.LBB40_188
.Ltmp7269:
	.loc	21 0 29 is_stmt 0
	movq	416(%rsp), %rbx
	addl	%r13d, %ebx
	andl	%eax, %ebx
	leaq	3(,%rbx,4), %rbx
.Ltmp7270:
	.loc	21 276 29 is_stmt 1
	cmpq	%r10, %rbx
	jae	.LBB40_193
	.loc	21 0 29 is_stmt 0
	vmovaps	%xmm3, 240(%rsp)
	.loc	21 277 29 is_stmt 1
	cmpq	48(%rsp), %rbx
	jae	.LBB40_182
.Ltmp7271:
	.loc	21 0 0 is_stmt 0
	vmovups	(%r8,%r15,4), %xmm0
	vmovaps	%xmm0, 208(%rsp)
.Ltmp7272:
	vmovups	(%rcx,%r15,4), %xmm0
	vmovaps	%xmm0, 224(%rsp)
.Ltmp7273:
	.loc	1 551 14 is_stmt 1
	vmovd	(%r8,%r11,4), %xmm0
.Ltmp7274:
	.loc	1 551 14 is_stmt 0
	vmovd	(%rcx,%r11,4), %xmm4
.Ltmp7275:
	.loc	1 551 14
	vpinsrd	$1, (%r8,%r12,4), %xmm0, %xmm0
.Ltmp7276:
	.loc	1 551 14
	vpinsrd	$1, (%rcx,%r12,4), %xmm4, %xmm4
.Ltmp7277:
	.loc	1 551 14
	vpinsrd	$2, (%r8,%rdx,4), %xmm0, %xmm0
.Ltmp7278:
	.loc	1 551 14
	vpinsrd	$2, (%rcx,%rdx,4), %xmm4, %xmm4
.Ltmp7279:
	.loc	1 551 14
	vpinsrd	$3, (%r8,%r9,4), %xmm0, %xmm8
.Ltmp7280:
	.loc	1 551 14
	vpinsrd	$3, (%rcx,%r9,4), %xmm4, %xmm0
.Ltmp7281:
	.loc	1 551 14
	vmovd	(%rcx,%rsi,4), %xmm4
.Ltmp7282:
	.loc	1 551 14
	vmovd	(%r8,%rsi,4), %xmm13
.Ltmp7283:
	.loc	1 551 14
	vpinsrd	$1, (%rcx,%r14,4), %xmm4, %xmm4
.Ltmp7284:
	.loc	1 551 14
	vpinsrd	$1, (%r8,%r14,4), %xmm13, %xmm13
.Ltmp7285:
	.loc	1 551 14
	vpinsrd	$2, (%rcx,%rbp,4), %xmm4, %xmm4
.Ltmp7286:
	.loc	1 551 14
	vpinsrd	$2, (%r8,%rbp,4), %xmm13, %xmm13
.Ltmp7287:
	.loc	1 551 14
	vpinsrd	$3, (%rcx,%rbx,4), %xmm4, %xmm4
.Ltmp7288:
	.loc	1 551 14
	vpinsrd	$3, (%r8,%rbx,4), %xmm13, %xmm13
	vmovdqa	.LCPI40_4(%rip), %xmm3
.Ltmp7289:
	.loc	48 257 24 is_stmt 1
	vpand	%xmm3, %xmm8, %xmm8
	vbroadcastss	.LCPI40_5(%rip), %xmm5
.Ltmp7290:
	.loc	48 88 14
	vmulps	%xmm5, %xmm8, %xmm2
.Ltmp7291:
	.loc	48 257 24
	vpand	%xmm3, %xmm0, %xmm0
.Ltmp7292:
	.loc	48 88 14
	vmulps	%xmm5, %xmm0, %xmm6
.Ltmp7293:
	.loc	48 36 14
	vaddps	%xmm6, %xmm2, %xmm2
.Ltmp7294:
	.loc	48 238 14
	vmaxps	%xmm0, %xmm8, %xmm0
	vmovaps	528(%rsp), %xmm6
.Ltmp7295:
	.loc	49 131 19
	vblendvps	%xmm6, %xmm0, %xmm8, %xmm0
	vmovaps	544(%rsp), %xmm6
.Ltmp7296:
	.loc	49 131 19 is_stmt 0
	vblendvps	%xmm6, %xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI40_6(%rip), %xmm10
.Ltmp7297:
	.loc	48 238 14 is_stmt 1
	vmaxps	%xmm10, %xmm0, %xmm0
	vbroadcastss	.LCPI40_7(%rip), %xmm2
.Ltmp7298:
	.loc	48 238 14 is_stmt 0
	vmaxps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI40_36(%rip), %xmm2
.Ltmp7299:
	.loc	50 863 14 is_stmt 1
	vandps	%xmm2, %xmm0, %xmm2
	vbroadcastss	.LCPI40_2(%rip), %xmm6
.Ltmp7300:
	.loc	50 889 14
	vorps	%xmm6, %xmm2, %xmm2
.Ltmp7301:
	.loc	48 62 14
	vaddps	%xmm2, %xmm12, %xmm2
	vbroadcastss	.LCPI40_10(%rip), %xmm6
.Ltmp7302:
	.loc	48 88 14
	vmulps	%xmm6, %xmm2, %xmm6
	vbroadcastss	.LCPI40_11(%rip), %xmm7
.Ltmp7303:
	.loc	48 36 14
	vsubps	%xmm6, %xmm7, %xmm6
.Ltmp7304:
	.loc	48 88 14
	vmulps	%xmm6, %xmm2, %xmm6
	vbroadcastss	.LCPI40_12(%rip), %xmm7
.Ltmp7305:
	.loc	48 36 14
	vaddps	%xmm7, %xmm6, %xmm6
.Ltmp7306:
	.loc	48 88 14
	vmulps	%xmm6, %xmm2, %xmm6
	vbroadcastss	.LCPI40_13(%rip), %xmm7
.Ltmp7307:
	.loc	48 36 14
	vaddps	%xmm7, %xmm6, %xmm6
.Ltmp7308:
	.loc	48 88 14
	vmulps	%xmm6, %xmm2, %xmm6
	vbroadcastss	.LCPI40_14(%rip), %xmm7
.Ltmp7309:
	.loc	48 36 14
	vaddps	%xmm7, %xmm6, %xmm6
.Ltmp7310:
	.loc	48 88 14
	vmulps	%xmm6, %xmm2, %xmm6
	vbroadcastss	.LCPI40_15(%rip), %xmm7
.Ltmp7311:
	.loc	48 36 14
	vaddps	%xmm7, %xmm6, %xmm6
.Ltmp7312:
	.loc	50 817 24
	vpsrld	$23, %xmm0, %xmm0
	vmovdqa	.LCPI40_16(%rip), %xmm7
.Ltmp7313:
	.loc	50 889 14
	vpor	%xmm7, %xmm0, %xmm0
	vbroadcastss	.LCPI40_17(%rip), %xmm7
.Ltmp7314:
	.loc	48 62 14
	vaddps	%xmm7, %xmm0, %xmm0
.Ltmp7315:
	.loc	48 88 14
	vmulps	%xmm6, %xmm2, %xmm2
.Ltmp7316:
	.loc	48 36 14
	vaddps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI40_18(%rip), %xmm2
.Ltmp7317:
	.loc	48 88 14
	vmulps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI40_19(%rip), %xmm2
.Ltmp7318:
	.loc	48 212 14
	vminps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI40_20(%rip), %xmm2
.Ltmp7319:
	.loc	48 238 14
	vmaxps	%xmm2, %xmm0, %xmm0
.Ltmp7320:
	.loc	48 544 14
	vcmpltps	96(%rsp), %xmm9, %xmm2
	vmovaps	768(%rsp), %xmm7
.Ltmp7321:
	.loc	48 558 14
	vcmpleps	%xmm0, %xmm7, %xmm6
	vmovaps	752(%rsp), %xmm8
.Ltmp7322:
	.loc	48 558 14 is_stmt 0
	vcmpleps	%xmm0, %xmm8, %xmm8
.Ltmp7323:
	.loc	48 257 24 is_stmt 1
	vandnps	%xmm6, %xmm2, %xmm6
.Ltmp7324:
	.loc	48 257 24 is_stmt 0
	vandps	%xmm2, %xmm8, %xmm15
.Ltmp7325:
	.loc	48 302 24 is_stmt 1
	vorps	%xmm6, %xmm15, %xmm6
	vmovaps	144(%rsp), %xmm1
.Ltmp7326:
	.loc	48 544 14
	vcmpltps	%xmm1, %xmm9, %xmm15
.Ltmp7327:
	.loc	48 257 24
	vandnps	%xmm15, %xmm8, %xmm8
	vandps	%xmm2, %xmm8, %xmm2
.Ltmp7328:
	.loc	48 302 24
	vorps	%xmm6, %xmm2, %xmm8
.Ltmp7329:
	.loc	49 131 19
	vpcmpgtd	%xmm8, %xmm9, %xmm8
	vpbroadcastd	.LCPI40_2(%rip), %xmm15
	vpand	%xmm15, %xmm8, %xmm14
.Ltmp7330:
	.loc	48 62 14
	vaddps	%xmm1, %xmm12, %xmm8
.Ltmp7331:
	.loc	49 131 19
	vblendvps	%xmm2, %xmm8, %xmm1, %xmm1
.Ltmp7332:
	.loc	49 131 19 is_stmt 0
	vblendvps	%xmm6, 704(%rsp), %xmm1, %xmm1
	movq	40(%rsp), %r9
	vmovaps	%xmm1, 144(%rsp)
.Ltmp7333:
	.loc	21 373 5 is_stmt 1
	vmovaps	%xmm1, 1360(%r9)
	.loc	21 382 5
	vmovdqa	%xmm14, 1344(%r9)
.Ltmp7334:
	.loc	48 62 14
	vsubps	%xmm7, %xmm0, %xmm0
.Ltmp7335:
	.loc	48 88 14
	vmulps	736(%rsp), %xmm0, %xmm0
.Ltmp7336:
	.loc	48 238 14
	vmaxps	720(%rsp), %xmm0, %xmm0
.Ltmp7337:
	.loc	48 212 14
	vminps	%xmm9, %xmm0, %xmm0
	vmovdqa	%xmm14, 96(%rsp)
.Ltmp7338:
	.loc	48 544 14
	vcmpltps	%xmm14, %xmm9, %xmm2
.Ltmp7339:
	.loc	49 131 19
	vpcmpgtd	%xmm2, %xmm9, %xmm2
	vpandn	%xmm0, %xmm2, %xmm0
.Ltmp7340:
	.loc	21 392 36
	vmovaps	1376(%r9), %xmm2
.Ltmp7341:
	.loc	48 544 14
	vcmpltps	%xmm0, %xmm2, %xmm6
	vmovaps	624(%rsp), %xmm7
.Ltmp7342:
	.loc	49 131 19
	vblendvps	%xmm6, 608(%rsp), %xmm7, %xmm6
.Ltmp7343:
	.loc	48 62 14
	vsubps	%xmm2, %xmm0, %xmm0
.Ltmp7344:
	.loc	48 88 14
	vmulps	%xmm6, %xmm0, %xmm0
.Ltmp7345:
	.loc	48 36 14
	vaddps	%xmm0, %xmm2, %xmm0
.Ltmp7346:
	.loc	48 257 24
	vandps	%xmm3, %xmm0, %xmm2
	vbroadcastss	.LCPI40_22(%rip), %xmm14
.Ltmp7347:
	.loc	48 517 14
	vcmpltps	%xmm14, %xmm2, %xmm2
.Ltmp7348:
	.loc	48 257 24
	vandnps	%xmm0, %xmm2, %xmm8
	vbroadcastss	.LCPI40_23(%rip), %xmm0
.Ltmp7349:
	.loc	48 88 14
	vmulps	%xmm0, %xmm8, %xmm0
	vbroadcastss	.LCPI40_24(%rip), %xmm1
.Ltmp7350:
	.loc	48 238 14
	vmaxps	%xmm1, %xmm0, %xmm0
	vbroadcastss	.LCPI40_25(%rip), %xmm1
.Ltmp7351:
	.loc	48 212 14
	vminps	%xmm1, %xmm0, %xmm0
.Ltmp7352:
	.loc	49 724 14
	vroundps	$9, %xmm0, %xmm2
.Ltmp7353:
	.loc	48 62 14
	vsubps	%xmm2, %xmm0, %xmm0
	vbroadcastss	.LCPI40_26(%rip), %xmm1
.Ltmp7354:
	.loc	48 88 14
	vmulps	%xmm1, %xmm0, %xmm6
	vbroadcastss	.LCPI40_27(%rip), %xmm1
.Ltmp7355:
	.loc	48 36 14
	vaddps	%xmm1, %xmm6, %xmm6
.Ltmp7356:
	.loc	48 88 14
	vmulps	%xmm6, %xmm0, %xmm6
	vbroadcastss	.LCPI40_28(%rip), %xmm1
.Ltmp7357:
	.loc	48 36 14
	vaddps	%xmm1, %xmm6, %xmm6
.Ltmp7358:
	.loc	48 88 14
	vmulps	%xmm6, %xmm0, %xmm6
	vbroadcastss	.LCPI40_29(%rip), %xmm1
.Ltmp7359:
	.loc	48 36 14
	vaddps	%xmm1, %xmm6, %xmm6
.Ltmp7360:
	.loc	48 88 14
	vmulps	%xmm6, %xmm0, %xmm6
	vbroadcastss	.LCPI40_30(%rip), %xmm1
.Ltmp7361:
	.loc	48 36 14
	vaddps	%xmm1, %xmm6, %xmm6
.Ltmp7362:
	.loc	48 88 14
	vmulps	%xmm6, %xmm0, %xmm0
.Ltmp7363:
	.loc	48 36 14
	vaddps	%xmm0, %xmm15, %xmm0
	vbroadcastss	.LCPI40_31(%rip), %xmm1
.Ltmp7364:
	.loc	48 36 14 is_stmt 0
	vaddps	%xmm1, %xmm2, %xmm2
.Ltmp7365:
	.loc	50 613 24 is_stmt 1
	vpslld	$23, %xmm2, %xmm2
.Ltmp7366:
	.loc	48 88 14
	vmulps	%xmm2, %xmm0, %xmm0
.Ltmp7367:
	.loc	48 504 14
	vcmpeqps	%xmm9, %xmm8, %xmm2
.Ltmp7368:
	.loc	48 302 24
	vorps	512(%rsp), %xmm2, %xmm2
	vmovaps	208(%rsp), %xmm1
.Ltmp7369:
	.loc	48 88 14
	vmulps	%xmm0, %xmm1, %xmm0
.Ltmp7370:
	.loc	49 131 19
	vblendvps	%xmm2, %xmm1, %xmm0, %xmm1
.Ltmp7371:
	.loc	21 392 36
	vmovaps	1680(%r9), %xmm0
.Ltmp7372:
	.loc	48 257 24
	vpand	%xmm3, %xmm4, %xmm2
.Ltmp7373:
	.loc	48 88 14
	vmulps	%xmm5, %xmm2, %xmm4
.Ltmp7374:
	.loc	48 257 24
	vpand	%xmm3, %xmm13, %xmm6
.Ltmp7375:
	.loc	48 88 14
	vmulps	%xmm5, %xmm6, %xmm13
.Ltmp7376:
	.loc	48 238 14
	vmaxps	%xmm6, %xmm2, %xmm6
	vmovaps	480(%rsp), %xmm5
.Ltmp7377:
	.loc	49 131 19
	vblendvps	%xmm5, %xmm6, %xmm2, %xmm2
.Ltmp7378:
	.loc	48 36 14
	vaddps	%xmm4, %xmm13, %xmm4
	vmovaps	496(%rsp), %xmm5
.Ltmp7379:
	.loc	49 131 19
	vblendvps	%xmm5, %xmm4, %xmm2, %xmm2
.Ltmp7380:
	.loc	48 238 14
	vmaxps	%xmm10, %xmm2, %xmm2
.Ltmp7381:
	.loc	48 238 14 is_stmt 0
	vbroadcastss	.LCPI40_7(%rip), %xmm4
	vmaxps	%xmm4, %xmm2, %xmm2
.Ltmp7382:
	.loc	50 863 14 is_stmt 1
	vandps	.LCPI40_8(%rip), %xmm2, %xmm4
.Ltmp7383:
	.loc	50 889 14
	vorps	.LCPI40_9(%rip), %xmm4, %xmm4
.Ltmp7384:
	.loc	48 62 14
	vaddps	%xmm4, %xmm12, %xmm4
.Ltmp7385:
	.loc	48 88 14
	vbroadcastss	.LCPI40_10(%rip), %xmm5
	vmulps	%xmm5, %xmm4, %xmm6
.Ltmp7386:
	.loc	48 36 14
	vbroadcastss	.LCPI40_11(%rip), %xmm5
	vsubps	%xmm6, %xmm5, %xmm6
.Ltmp7387:
	.loc	48 88 14
	vmulps	%xmm6, %xmm4, %xmm6
.Ltmp7388:
	.loc	48 36 14
	vbroadcastss	.LCPI40_12(%rip), %xmm5
	vaddps	%xmm5, %xmm6, %xmm6
.Ltmp7389:
	.loc	48 88 14
	vmulps	%xmm6, %xmm4, %xmm6
.Ltmp7390:
	.loc	48 36 14
	vbroadcastss	.LCPI40_13(%rip), %xmm5
	vaddps	%xmm5, %xmm6, %xmm6
.Ltmp7391:
	.loc	48 88 14
	vmulps	%xmm6, %xmm4, %xmm6
.Ltmp7392:
	.loc	48 36 14
	vbroadcastss	.LCPI40_14(%rip), %xmm5
	vaddps	%xmm5, %xmm6, %xmm6
.Ltmp7393:
	.loc	48 88 14
	vmulps	%xmm6, %xmm4, %xmm6
.Ltmp7394:
	.loc	48 36 14
	vbroadcastss	.LCPI40_15(%rip), %xmm5
	vaddps	%xmm5, %xmm6, %xmm6
.Ltmp7395:
	.loc	50 817 24
	vpsrld	$23, %xmm2, %xmm2
.Ltmp7396:
	.loc	50 889 14
	vpor	.LCPI40_16(%rip), %xmm2, %xmm2
.Ltmp7397:
	.loc	48 62 14
	vbroadcastss	.LCPI40_17(%rip), %xmm5
	vaddps	%xmm5, %xmm2, %xmm2
.Ltmp7398:
	.loc	48 88 14
	vmulps	%xmm6, %xmm4, %xmm4
.Ltmp7399:
	.loc	48 36 14
	vaddps	%xmm4, %xmm2, %xmm2
.Ltmp7400:
	.loc	48 88 14
	vbroadcastss	.LCPI40_18(%rip), %xmm4
	vmulps	%xmm4, %xmm2, %xmm2
.Ltmp7401:
	.loc	48 212 14
	vbroadcastss	.LCPI40_19(%rip), %xmm4
	vminps	%xmm4, %xmm2, %xmm2
.Ltmp7402:
	.loc	48 238 14
	vbroadcastss	.LCPI40_20(%rip), %xmm4
	vmaxps	%xmm4, %xmm2, %xmm2
.Ltmp7403:
	.loc	48 544 14
	vcmpltps	%xmm11, %xmm9, %xmm6
	vmovaps	688(%rsp), %xmm7
.Ltmp7404:
	.loc	48 558 14
	vcmpleps	%xmm2, %xmm7, %xmm4
	vmovaps	672(%rsp), %xmm10
.Ltmp7405:
	.loc	48 558 14 is_stmt 0
	vcmpleps	%xmm2, %xmm10, %xmm10
.Ltmp7406:
	.loc	48 257 24 is_stmt 1
	vandnps	%xmm4, %xmm6, %xmm4
.Ltmp7407:
	.loc	48 257 24 is_stmt 0
	vandps	%xmm6, %xmm10, %xmm13
.Ltmp7408:
	.loc	48 302 24 is_stmt 1
	vorps	%xmm4, %xmm13, %xmm4
	vmovaps	240(%rsp), %xmm5
.Ltmp7409:
	.loc	48 544 14
	vcmpltps	%xmm5, %xmm9, %xmm13
.Ltmp7410:
	.loc	48 257 24
	vandnps	%xmm13, %xmm10, %xmm10
	vandps	%xmm6, %xmm10, %xmm6
.Ltmp7411:
	.loc	48 302 24
	vorps	%xmm4, %xmm6, %xmm10
.Ltmp7412:
	.loc	49 131 19
	vpcmpgtd	%xmm10, %xmm9, %xmm10
.Ltmp7413:
	.loc	48 62 14
	vaddps	%xmm5, %xmm12, %xmm13
.Ltmp7414:
	.loc	49 131 19
	vblendvps	%xmm6, %xmm13, %xmm5, %xmm13
.Ltmp7415:
	.loc	49 131 19 is_stmt 0
	vpand	%xmm15, %xmm10, %xmm11
.Ltmp7416:
	.loc	48 62 14 is_stmt 1
	vsubps	%xmm7, %xmm2, %xmm2
.Ltmp7417:
	.loc	48 88 14
	vmulps	656(%rsp), %xmm2, %xmm2
.Ltmp7418:
	.loc	48 238 14
	vmaxps	640(%rsp), %xmm2, %xmm2
.Ltmp7419:
	.loc	48 212 14
	vminps	%xmm9, %xmm2, %xmm2
.Ltmp7420:
	.loc	48 544 14
	vcmpltps	%xmm11, %xmm9, %xmm6
.Ltmp7421:
	.loc	49 131 19
	vpcmpgtd	%xmm6, %xmm9, %xmm6
	vpandn	%xmm2, %xmm6, %xmm2
.Ltmp7422:
	.loc	48 544 14
	vcmpltps	%xmm2, %xmm0, %xmm6
	vmovaps	576(%rsp), %xmm7
.Ltmp7423:
	.loc	49 131 19
	vblendvps	%xmm6, 560(%rsp), %xmm7, %xmm6
.Ltmp7424:
	.loc	48 62 14
	vsubps	%xmm0, %xmm2, %xmm2
.Ltmp7425:
	.loc	48 88 14
	vmulps	%xmm6, %xmm2, %xmm2
.Ltmp7426:
	.loc	48 36 14
	vaddps	%xmm2, %xmm0, %xmm0
.Ltmp7427:
	.loc	48 257 24
	vandps	%xmm3, %xmm0, %xmm2
.Ltmp7428:
	.loc	48 517 14
	vcmpltps	%xmm14, %xmm2, %xmm2
.Ltmp7429:
	.loc	48 257 24
	vandnps	%xmm0, %xmm2, %xmm0
.Ltmp7430:
	.loc	48 88 14
	vbroadcastss	.LCPI40_23(%rip), %xmm2
	vmulps	%xmm2, %xmm0, %xmm2
.Ltmp7431:
	.loc	48 238 14
	vbroadcastss	.LCPI40_24(%rip), %xmm3
	vmaxps	%xmm3, %xmm2, %xmm2
.Ltmp7432:
	.loc	48 212 14
	vbroadcastss	.LCPI40_25(%rip), %xmm3
	vminps	%xmm3, %xmm2, %xmm2
.Ltmp7433:
	.loc	49 724 14
	vroundps	$9, %xmm2, %xmm6
.Ltmp7434:
	.loc	48 62 14
	vsubps	%xmm6, %xmm2, %xmm2
.Ltmp7435:
	.loc	48 88 14
	vbroadcastss	.LCPI40_26(%rip), %xmm3
	vmulps	%xmm3, %xmm2, %xmm14
.Ltmp7436:
	.loc	48 36 14
	vbroadcastss	.LCPI40_27(%rip), %xmm3
	vaddps	%xmm3, %xmm14, %xmm14
.Ltmp7437:
	.loc	48 88 14
	vmulps	%xmm2, %xmm14, %xmm14
.Ltmp7438:
	.loc	48 36 14
	vbroadcastss	.LCPI40_28(%rip), %xmm3
	vaddps	%xmm3, %xmm14, %xmm14
.Ltmp7439:
	.loc	48 88 14
	vmulps	%xmm2, %xmm14, %xmm14
.Ltmp7440:
	.loc	48 36 14
	vbroadcastss	.LCPI40_29(%rip), %xmm3
	vaddps	%xmm3, %xmm14, %xmm14
.Ltmp7441:
	.loc	48 88 14
	vmulps	%xmm2, %xmm14, %xmm14
.Ltmp7442:
	.loc	48 36 14
	vbroadcastss	.LCPI40_30(%rip), %xmm3
	vaddps	%xmm3, %xmm14, %xmm14
.Ltmp7443:
	.loc	48 88 14
	vmulps	%xmm2, %xmm14, %xmm2
.Ltmp7444:
	.loc	48 36 14
	vaddps	%xmm2, %xmm15, %xmm2
.Ltmp7445:
	.loc	48 36 14 is_stmt 0
	vbroadcastss	.LCPI40_31(%rip), %xmm3
	vaddps	%xmm3, %xmm6, %xmm6
.Ltmp7446:
	.loc	50 613 24 is_stmt 1
	vpslld	$23, %xmm6, %xmm6
.Ltmp7447:
	.loc	48 88 14
	vmulps	%xmm6, %xmm2, %xmm2
.Ltmp7448:
	.loc	48 504 14
	vcmpeqps	%xmm0, %xmm9, %xmm6
.Ltmp7449:
	.loc	48 302 24
	vorps	464(%rsp), %xmm6, %xmm6
	vmovaps	224(%rsp), %xmm3
.Ltmp7450:
	.loc	48 88 14
	vmulps	%xmm2, %xmm3, %xmm2
.Ltmp7451:
	.loc	49 131 19
	vblendvps	%xmm6, %xmm3, %xmm2, %xmm2
.Ltmp7452:
	.loc	21 394 5
	vmovaps	%xmm8, 1376(%r9)
.Ltmp7453:
	.loc	49 131 19
	vblendvps	%xmm4, 592(%rsp), %xmm13, %xmm3
.Ltmp7454:
	.loc	21 373 5
	vmovaps	%xmm3, 1664(%r9)
	.loc	21 382 5
	vmovdqa	%xmm11, 1648(%r9)
.Ltmp7455:
	.loc	21 394 5
	vmovaps	%xmm0, 1680(%r9)
	movq	64(%rsp), %rdx
.Ltmp7456:
	.loc	1 551 14
	vmovups	%xmm1, (%rdx)
	movq	88(%rsp), %rdx
.Ltmp7457:
	.loc	1 551 14 is_stmt 0
	vmovups	%xmm2, (%rdx)
.Ltmp7458:
	.loc	21 0 0
	incq	%r13
.Ltmp7459:
	.loc	8 1916 50 is_stmt 1
	addq	$4, %rdi
	cmpq	%r13, 160(%rsp)
	movq	48(%rsp), %rbx
.Ltmp7460:
	.loc	11 900 12
	jne	.LBB40_91
.Ltmp7461:
	.loc	11 0 12 is_stmt 0
	movq	160(%rsp), %rax
.Ltmp7462:
	.loc	15 2584 13 is_stmt 1
	addl	176(%rsp), %eax
.Ltmp7463:
	.loc	21 297 5
	movl	%eax, 1808(%r9)
	movq	128(%rsp), %rax
	movq	360(%rsp), %rdx
	movq	352(%rsp), %r8
	movq	120(%rsp), %rsi
.Ltmp7464:
.LBB40_115:
	.loc	6 698 9
	subl	%esi, 1820(%r9)
.Ltmp7465:
	.loc	6 765 33
	movq	$0, 288(%rsp)
	movq	$2, 296(%rsp)
	movq	56(%rsp), %rcx
	movq	%rcx, 304(%rsp)
	movq	%rdx, 312(%rsp)
	movq	80(%rsp), %rcx
	movq	%rcx, 320(%rsp)
	movq	%r8, 328(%rsp)
	movq	$0, 336(%rsp)
	leaq	768(%r9), %rcx
	movq	%rcx, 272(%rsp)
	leaq	512(%r9), %rcx
	movq	%rcx, 264(%rsp)
	leaq	896(%r9), %rcx
	movq	%rcx, 256(%rsp)
	vpxor	%xmm0, %xmm0, %xmm0
	vcmpeqps	%xmm0, %xmm0, %xmm0
.Ltmp7466:
	.loc	27 131 12
	leaq	(,%rax,4), %r10
	movl	$24, %eax
	vbroadcastss	.LCPI40_38(%rip), %xmm4
	vbroadcastss	.LCPI40_2(%rip), %xmm1
	vmovaps	%xmm1, 368(%rsp)
	vbroadcastss	.LCPI40_32(%rip), %xmm5
	vmovsd	.LCPI40_33(%rip), %xmm6
	vmovsd	.LCPI40_34(%rip), %xmm7
	vmovsd	.LCPI40_35(%rip), %xmm8
	xorl	%ecx, %ecx
	xorl	%esi, %esi
	vmovaps	%xmm0, 192(%rsp)
	movq	%r10, 56(%rsp)
	jmp	.LBB40_117
	.loc	27 0 12 is_stmt 0
.Ltmp7467:
	.p2align	4
.LBB40_116:
	movl	$1, %esi
	movl	$32, %eax
	.loc	27 131 12 is_stmt 1
	testb	$1, 280(%rsp)
	movb	$1, %cl
	vmovaps	192(%rsp), %xmm0
	jne	.LBB40_143
.Ltmp7468:
.LBB40_117:
	.loc	27 0 12 is_stmt 0
	movq	%rcx, 280(%rsp)
.Ltmp7469:
	.loc	25 253 13 is_stmt 1
	movq	%rsi, %rdx
	shlq	$4, %rdx
	leaq	304(%rsp), %rcx
.Ltmp7470:
	.loc	1 1733 9
	movq	(%rcx,%rdx), %rbp
	movq	%rdx, 96(%rsp)
	movq	8(%rcx,%rdx), %r15
.Ltmp7471:
	.loc	16 2155 12
	movq	%r15, %rcx
	andq	$-4, %rcx
	je	.LBB40_120
.Ltmp7472:
	.loc	16 0 12 is_stmt 0
	xorl	%edx, %edx
	vmovaps	192(%rsp), %xmm0
	.p2align	4
.LBB40_119:
.Ltmp7473:
	.loc	48 257 24 is_stmt 1
	vandps	(%rbp,%rdx,4), %xmm4, %xmm1
.Ltmp7474:
	.loc	48 517 14
	vcmpltps	%xmm5, %xmm1, %xmm1
.Ltmp7475:
	.loc	48 257 24
	vandps	%xmm1, %xmm0, %xmm0
.Ltmp7476:
	.loc	16 2155 12
	addq	$4, %rdx
	cmpq	%rdx, %rcx
	jne	.LBB40_119
.Ltmp7477:
.LBB40_120:
	.loc	16 0 12 is_stmt 0
	movq	%rsi, %r14
	imulq	$304, %rsi, %rcx
.Ltmp7478:
	.file	51 "/home/bl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wide-1.6.1" "src/f32x4_.rs"
	.loc	51 285 9 is_stmt 1
	vpcmpeqd	%xmm1, %xmm1, %xmm1
	vtestps	%xmm1, %xmm0
	vandps	1376(%r9,%rcx), %xmm4, %xmm0
.Ltmp7479:
	.loc	6 767 16
	jae	.LBB40_122
.Ltmp7480:
	.loc	48 517 14
	vcmpltps	%xmm5, %xmm0, %xmm1
.Ltmp7481:
	.loc	48 257 24
	vandps	192(%rsp), %xmm1, %xmm1
.Ltmp7482:
	.loc	51 285 9
	vpcmpeqd	%xmm2, %xmm2, %xmm2
	vtestps	%xmm2, %xmm1
.Ltmp7483:
	.loc	6 767 43
	jb	.LBB40_116
.LBB40_122:
.Ltmp7484:
	.loc	16 2155 12
	movq	%r15, %rdx
	vmovaps	192(%rsp), %xmm2
	vmovaps	%xmm2, %xmm1
	movabsq	$2305843009213693948, %rsi
	andq	%rsi, %rdx
	je	.LBB40_126
.Ltmp7485:
	.loc	16 0 12 is_stmt 0
	xorl	%esi, %esi
	vmovaps	%xmm2, %xmm1
	.p2align	4
.LBB40_124:
.Ltmp7486:
	.loc	48 257 24 is_stmt 1
	vandps	(%rbp,%rsi,4), %xmm4, %xmm2
.Ltmp7487:
	.loc	48 517 14
	vcmpltps	%xmm5, %xmm2, %xmm2
.Ltmp7488:
	.loc	48 257 24
	vandps	%xmm2, %xmm1, %xmm1
.Ltmp7489:
	.loc	16 2155 12
	addq	$4, %rsi
	cmpq	%rsi, %rdx
	jne	.LBB40_124
.Ltmp7490:
	.loc	16 0 12 is_stmt 0
	vmovaps	192(%rsp), %xmm2
.LBB40_126:
.Ltmp7491:
	.loc	48 517 14 is_stmt 1
	vcmpltps	%xmm5, %xmm0, %xmm0
.Ltmp7492:
	.loc	48 257 24
	vandps	%xmm2, %xmm0, %xmm0
.Ltmp7493:
	.loc	49 131 19
	vpsrad	$31, %xmm1, %xmm1
	vmovdqa	368(%rsp), %xmm2
	vpandn	%xmm2, %xmm1, %xmm1
.Ltmp7494:
	.loc	28 185 12
	vmovd	%xmm1, %esi
	xorl	%edx, %edx
	testl	%esi, %esi
	vpextrd	$1, %xmm1, %esi
	setne	%dl
	xorl	%r8d, %r8d
	testl	%esi, %esi
	setne	%r8b
	vpextrd	$2, %xmm1, %esi
	xorl	%edi, %edi
	testl	%esi, %esi
	setne	%dil
	vpextrd	$3, %xmm1, %r9d
	xorl	%esi, %esi
	testl	%r9d, %r9d
	setne	%sil
.Ltmp7495:
	.loc	49 131 19
	vpsrad	$31, %xmm0, %xmm0
	vpandn	%xmm2, %xmm0, %xmm0
.Ltmp7496:
	.loc	28 185 12
	vmovd	%xmm0, %r10d
	xorl	%r9d, %r9d
	testl	%r10d, %r10d
	setne	%r9b
	vpextrd	$1, %xmm0, %r10d
	xorl	%r12d, %r12d
	testl	%r10d, %r10d
	setne	%r12b
	addl	%r12d, %r12d
	vpextrd	$2, %xmm0, %r11d
	xorl	%r10d, %r10d
	testl	%r11d, %r11d
	setne	%r10b
	shll	$2, %r10d
	vpextrd	$3, %xmm0, %r11d
	xorl	%ebx, %ebx
	testl	%r11d, %r11d
	setne	%bl
	shll	$3, %ebx
	leal	(%rdx,%r8,2), %edx
	leal	(%rdx,%rdi,4), %edx
	leal	(%rdx,%rsi,8), %edx
.Ltmp7497:
	.loc	28 185 12 is_stmt 0
	orl	%r9d, %r12d
	orl	%r10d, %r12d
	orl	%edx, %r12d
.Ltmp7498:
	.loc	6 771 17 is_stmt 1
	orl	%ebx, %r12d
	movl	%r12d, 48(%rsp)
	movq	%r14, %rdi
.Ltmp7499:
	.loc	6 772 13
	movq	%r14, %rdx
	shlq	$5, %rdx
	movq	40(%rsp), %r9
	leaq	(%r9,%rdx), %r14
	shlq	$6, %rdi
	movq	272(%rsp), %rsi
	leaq	(%rsi,%rdi), %r13
	movq	96(%rsp), %rsi
.Ltmp7500:
	.loc	11 900 12
	addq	264(%rsp), %rsi
	movq	%rsi, 96(%rsp)
	addq	%r9, %rdi
	movq	%rdi, 80(%rsp)
	leaq	(%rdx,%rdx,2), %rdx
	movq	256(%rsp), %rsi
	leaq	(%rsi,%rdx), %rdi
	movq	%rdi, 120(%rsp)
	leaq	32(%rsi,%rdx), %rsi
	movq	%rsi, 64(%rsp)
	leaq	928(%r9,%rdx), %rdx
	movq	%rdx, 72(%rsp)
	movq	456(%rsp), %rdx
	leaq	(%rdx,%rcx), %rsi
	movq	%rsi, 136(%rsp)
	leaq	(%rdx,%rcx), %rbx
	addq	$288, %rbx
	addq	%rsp, %rax
	addq	$784, %rax
	movq	%rax, 144(%rsp)
	xorl	%r12d, %r12d
	movq	128(%rsp), %r8
	movq	56(%rsp), %r10
	movabsq	$9223372036854775807, %r11
	jmp	.LBB40_129
.Ltmp7501:
	.loc	11 0 12 is_stmt 0
.Ltmp7502:
	.p2align	4
.LBB40_127:
	.loc	6 781 34 is_stmt 1
	leaq	(%r12,%r12,4), %rax
	movq	144(%rsp), %rsi
	movq	(%rsi,%rax,8), %rcx
.Ltmp7503:
	.loc	15 2428 13
	addq	%r8, %rcx
	movq	$-1, %rdx
	cmovbq	%rdx, %rcx
.Ltmp7504:
	.loc	6 786 17
	movq	%rcx, (%rsi,%rax,8)
.Ltmp7505:
.LBB40_128:
	.loc	11 0 0 is_stmt 0
	incq	%r12
	.loc	11 900 12 is_stmt 1
	addq	$4, %rbp
.Ltmp7506:
	.loc	8 1916 50
	cmpq	$4, %r12
.Ltmp7507:
	.loc	11 900 12
	je	.LBB40_116
.Ltmp7508:
.LBB40_129:
	.loc	11 0 12 is_stmt 0
	movl	48(%rsp), %eax
.Ltmp7509:
	.loc	6 773 20 is_stmt 1
	btl	%r12d, %eax
	jae	.LBB40_128
	.loc	6 0 20 is_stmt 0
	testl	%r8d, %r8d
.Ltmp7510:
	.loc	11 900 12 is_stmt 1
	je	.LBB40_134
.Ltmp7511:
	.loc	11 0 12 is_stmt 0
	xorl	%eax, %eax
	.p2align	4
.LBB40_132:
.Ltmp7512:
	.loc	6 777 21 is_stmt 1
	leaq	(%r12,%rax), %rdi
	cmpq	%r15, %rdi
	jae	.LBB40_151
	movl	$0, (%rbp,%rax,4)
.Ltmp7513:
	.loc	8 1916 50
	addq	$4, %rax
	cmpq	%rax, %r10
.Ltmp7514:
	.loc	11 900 12
	jne	.LBB40_132
.Ltmp7515:
.LBB40_134:
	.loc	6 559 21
	movq	1696(%r9), %rax
.Ltmp7516:
	.loc	8 1916 50
	testq	%rax, %rax
.Ltmp7517:
	.loc	11 900 12
	je	.LBB40_138
.Ltmp7518:
	.loc	11 0 12 is_stmt 0
	movq	8(%r13), %rsi
	movq	%r12, %rdi
	.p2align	4
.LBB40_136:
.Ltmp7519:
	.loc	6 562 13 is_stmt 1
	cmpq	%rsi, %rdi
	jae	.LBB40_152
	movq	(%r13), %rcx
	movl	$0, (%rcx,%rdi,4)
.Ltmp7520:
	.loc	8 1916 50
	addq	$4, %rdi
	decq	%rax
.Ltmp7521:
	.loc	11 900 12
	jne	.LBB40_136
.Ltmp7522:
.LBB40_138:
	.loc	6 506 22
	movq	%r12, %rax
	shlq	$6, %rax
	vmovd	(%r14,%rax), %xmm12
	vmovss	4(%r14,%rax), %xmm11
	vmovss	8(%r14,%rax), %xmm10
	vmovss	12(%r14,%rax), %xmm9
	vmovss	16(%r14,%rax), %xmm0
	vmovss	20(%r14,%rax), %xmm2
	vmovss	24(%r14,%rax), %xmm13
	vmovss	28(%r14,%rax), %xmm1
.Ltmp7523:
	.loc	6 507 9
	movq	%r12, %rax
	shlq	$5, %rax
	movq	96(%rsp), %rcx
	vmovss	%xmm1, (%rcx,%rax)
	vmovss	%xmm0, 4(%rcx,%rax)
	vmovss	%xmm2, 8(%rcx,%rax)
	vmovss	%xmm13, 12(%rcx,%rax)
.Ltmp7524:
	.loc	6 530 27
	movl	1776(%r9), %eax
.Ltmp7525:
	.loc	9 82 17
	vcvtss2sd	%xmm1, %xmm1, %xmm1
.Ltmp7526:
	.loc	9 82 17 is_stmt 0
	vcvtsi2sd	%rax, %xmm15, %xmm3
.Ltmp7527:
	.loc	6 403 20 is_stmt 1
	vmulsd	%xmm3, %xmm1, %xmm1
	vdivsd	%xmm6, %xmm1, %xmm1
	.loc	6 403 19 is_stmt 0
	vaddsd	%xmm7, %xmm1, %xmm1
.Ltmp7528:
	.loc	10 1763 9 is_stmt 1
	vroundsd	$9, %xmm1, %xmm1, %xmm1
	vmovq	%xmm1, %rcx
.Ltmp7529:
	.loc	6 404 9
	testq	%rcx, %rcx
	sets	%dl
	andq	%r11, %rcx
	movabsq	$-4503599627370496, %rsi
	addq	%rcx, %rsi
	shrq	$53, %rsi
	cmpl	$1023, %esi
	setb	%sil
	andb	%dl, %sil
	movabsq	$9218868437227405311, %rdx
	cmpq	%rdx, %rcx
	setg	%cl
	orb	%sil, %cl
	jne	.LBB40_127
	vucomisd	%xmm8, %xmm1
	ja	.LBB40_127
.Ltmp7530:
	.loc	9 82 17
	vcvtss2sd	%xmm2, %xmm2, %xmm2
.Ltmp7531:
	.loc	6 403 20
	vmulsd	%xmm3, %xmm2, %xmm2
	vdivsd	%xmm6, %xmm2, %xmm2
	.loc	6 403 19 is_stmt 0
	vaddsd	%xmm7, %xmm2, %xmm2
.Ltmp7532:
	.loc	10 1763 9 is_stmt 1
	vroundsd	$9, %xmm2, %xmm2, %xmm2
	vmovq	%xmm2, %rcx
.Ltmp7533:
	.loc	6 404 9
	testq	%rcx, %rcx
	sets	%dl
	andq	%r11, %rcx
	movabsq	$-4503599627370496, %rsi
	addq	%rcx, %rsi
	shrq	$53, %rsi
	cmpl	$1023, %esi
	setb	%sil
	andb	%dl, %sil
	movabsq	$9218868437227405311, %rdx
	cmpq	%rdx, %rcx
	setg	%cl
	orb	%sil, %cl
	jne	.LBB40_127
	vucomisd	%xmm8, %xmm2
	ja	.LBB40_127
.Ltmp7534:
	.loc	6 531 80
	movl	1816(%r9), %ecx
	vxorpd	%xmm3, %xmm3, %xmm3
.Ltmp7535:
	.loc	6 407 10
	vmaxsd	%xmm1, %xmm3, %xmm1
	vminsd	%xmm1, %xmm8, %xmm1
	vcvttsd2si	%xmm1, %rdx
.Ltmp7536:
	.loc	6 407 10 is_stmt 0
	vmaxsd	%xmm2, %xmm3, %xmm1
	vminsd	%xmm1, %xmm8, %xmm1
	vcvttsd2si	%xmm1, %rsi
.Ltmp7537:
	.loc	6 533 9 is_stmt 1
	subl	%edx, %ecx
	movl	$0, %edx
	cmovbl	%edx, %ecx
	movq	80(%rsp), %rdx
	movl	%ecx, 800(%rdx,%r12,4)
	.loc	6 534 62
	movl	%esi, %ecx
	vcvtsi2ss	%rcx, %xmm15, %xmm1
	movq	64(%rsp), %rcx
.Ltmp7538:
	.loc	6 396 5
	vmovaps	(%rcx), %xmm2
	vmovaps	%xmm2, (%rsp)
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm1
	vmovaps	%xmm1, (%rcx)
	movq	%rax, %rdi
	movq	%rax, 176(%rsp)
	vmovss	%xmm9, 88(%rsp)
	vmovss	%xmm10, 240(%rsp)
	vmovss	%xmm11, 224(%rsp)
	vmovd	%xmm12, 208(%rsp)
	vmovss	%xmm13, 160(%rsp)
.Ltmp7539:
	.loc	6 538 13
	vzeroupper
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient@GOTPCREL(%rip)
	movq	120(%rsp), %rax
.Ltmp7540:
	.loc	6 396 5
	vmovaps	(%rax), %xmm1
	vmovaps	%xmm1, (%rsp)
	.loc	6 397 5
	vmovss	%xmm0, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, (%rax)
	vmovss	160(%rsp), %xmm0
	movq	176(%rsp), %rdi
.Ltmp7541:
	.loc	6 543 13
	callq	*_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient@GOTPCREL(%rip)
	vmovsd	.LCPI40_35(%rip), %xmm8
	movabsq	$9223372036854775807, %r11
	vmovsd	.LCPI40_34(%rip), %xmm7
	vmovsd	.LCPI40_33(%rip), %xmm6
	vbroadcastss	.LCPI40_32(%rip), %xmm5
	vbroadcastss	.LCPI40_38(%rip), %xmm4
	movq	56(%rsp), %r10
	movq	128(%rsp), %r8
	movq	40(%rsp), %r9
	movq	64(%rsp), %rax
.Ltmp7542:
	.loc	6 396 5
	vmovaps	-16(%rax), %xmm1
	vmovaps	%xmm1, (%rsp)
	.loc	6 397 5
	vmovss	%xmm0, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -16(%rax)
	movq	72(%rsp), %rax
.Ltmp7543:
	.loc	6 514 29
	vmovaps	(%rax), %xmm0
	vmovaps	%xmm0, (%rsp)
.Ltmp7544:
	.loc	6 390 5
	vmovss	(%rsp,%r12,4), %xmm0
.Ltmp7545:
	.loc	6 396 5
	vmovaps	(%rbx), %xmm1
	vmovaps	%xmm1, (%rsp)
	.loc	6 397 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm1
	vmovaps	%xmm1, (%rbx)
.Ltmp7546:
	.loc	6 396 5
	vmovaps	-32(%rbx), %xmm1
	vmovaps	%xmm1, (%rsp)
	.loc	6 397 5
	movl	$1065353216, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm1
	vmovaps	%xmm1, -32(%rbx)
.Ltmp7547:
	.loc	6 396 5
	vmovaps	-16(%rbx), %xmm1
	vmovaps	%xmm1, (%rsp)
	.loc	6 397 5
	vmovss	%xmm0, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -16(%rbx)
	movq	136(%rsp), %rax
.Ltmp7548:
	.loc	6 396 5
	vmovaps	(%rax), %xmm0
	vmovaps	%xmm0, (%rsp)
	vmovss	208(%rsp), %xmm1
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, (%rax)
.Ltmp7549:
	.loc	6 396 5
	vmovaps	-272(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -272(%rbx)
.Ltmp7550:
	.loc	6 396 5
	vmovaps	-256(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 397 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -256(%rbx)
.Ltmp7551:
	.loc	6 396 5
	vmovaps	-240(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 397 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -240(%rbx)
.Ltmp7552:
	.loc	6 396 5
	vmovaps	-224(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	vmovss	224(%rsp), %xmm1
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -224(%rbx)
.Ltmp7553:
	.loc	6 396 5
	vmovaps	-208(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -208(%rbx)
.Ltmp7554:
	.loc	6 396 5
	vmovaps	-192(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 397 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -192(%rbx)
.Ltmp7555:
	.loc	6 396 5
	vmovaps	-176(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 397 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -176(%rbx)
.Ltmp7556:
	.loc	6 396 5
	vmovaps	-160(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	vmovss	240(%rsp), %xmm1
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -160(%rbx)
.Ltmp7557:
	.loc	6 396 5
	vmovaps	-144(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -144(%rbx)
.Ltmp7558:
	.loc	6 396 5
	vmovaps	-128(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 397 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -128(%rbx)
.Ltmp7559:
	.loc	6 396 5
	vmovaps	-112(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 397 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -112(%rbx)
.Ltmp7560:
	.loc	6 396 5
	vmovaps	-96(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	vmovss	88(%rsp), %xmm1
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -96(%rbx)
.Ltmp7561:
	.loc	6 396 5
	vmovaps	-80(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 397 5
	vmovss	%xmm1, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -80(%rbx)
.Ltmp7562:
	.loc	6 396 5
	vmovaps	-64(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 397 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -64(%rbx)
.Ltmp7563:
	.loc	6 396 5
	vmovaps	-48(%rbx), %xmm0
	vmovaps	%xmm0, (%rsp)
	.loc	6 397 5
	movl	$0, (%rsp,%r12,4)
	.loc	6 398 5
	vmovaps	(%rsp), %xmm0
	vmovaps	%xmm0, -48(%rbx)
	jmp	.LBB40_127
.Ltmp7564:
.LBB40_143:
	.loc	1 551 14
	vmovups	784(%rsp), %ymm0
	vmovups	816(%rsp), %ymm1
	vmovups	848(%rsp), %ymm2
	vmovups	880(%rsp), %ymm3
	vmovups	%ymm0, 1104(%rsp)
	vmovups	%ymm1, 1136(%rsp)
	vmovups	%ymm2, 1168(%rsp)
	vmovups	%ymm3, 1200(%rsp)
	vmovdqu	912(%rsp), %ymm0
	vmovdqu	%ymm0, 1232(%rsp)
	leaq	1104(%rsp), %rsi
.Ltmp7565:
	.loc	6 1094 17
	movl	$320, %edx
	movq	408(%rsp), %rbx
	movq	%rbx, %rdi
	vzeroupper
	callq	*memcpy@GOTPCREL(%rip)
	movq	%rbx, %rdi
	movzbl	39(%rsp), %eax
	movb	%al, 320(%rbx)
.Ltmp7566:
.LBB40_144:
	.loc	6 1095 14
	movq	%rdi, %rax
	.loc	6 1095 14 epilogue_begin is_stmt 0
	addq	$1432, %rsp
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
	vzeroupper
	retq
.LBB40_145:
	.cfi_def_cfa_offset 1488
.Ltmp7567:
	.loc	25 443 13 is_stmt 1
	leaq	.Lalloc_d8ffa1b111d58613892f54938d1219d1(%rip), %rcx
	movq	96(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp7568:
.LBB40_146:
	.loc	25 443 13
	leaq	4(%rdi), %rsi
	leaq	.Lalloc_97ed0782c9e7425b1b215f66cb64a347(%rip), %rcx
	movq	400(%rsp), %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp7569:
.LBB40_147:
	.loc	25 456 13
	leaq	.Lalloc_cad5a7b24766ffcdda9cbfe066eb4078(%rip), %rcx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp7570:
.LBB40_148:
	.loc	25 456 13
	leaq	.Lalloc_72e24c5578221343e8a784545a3910d2(%rip), %rcx
	movq	%rbp, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp7571:
.LBB40_149:
	.loc	25 443 13
	leaq	.Lalloc_ba99eeeb3482270ebe1c674b4013dd6a(%rip), %rcx
	movq	%r9, %rdi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp7572:
.LBB40_150:
	.loc	25 443 13
	leaq	.Lalloc_0f4c27429e4885e1b619f1e4a3587acc(%rip), %rcx
	movq	%r9, %rdi
	movq	%rbp, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp7573:
.LBB40_151:
	.loc	6 777 21
	leaq	.Lalloc_30d570589940b68d7f77af9df77eb2ee(%rip), %rdx
	movq	%r15, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp7574:
.LBB40_152:
	.loc	6 562 13
	leaq	.Lalloc_f8f0512af3f0ba047152c3c522a44b59(%rip), %rdx
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp7575:
.LBB40_153:
	.loc	25 456 13
	leaq	.Lalloc_cad5a7b24766ffcdda9cbfe066eb4078(%rip), %rcx
	movq	%r11, %rdi
	movq	%rbx, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp7576:
.LBB40_154:
	.loc	25 0 13 is_stmt 0
	movabsq	$2305843009213693948, %rsi
	movq	392(%rsp), %rdx
.Ltmp7577:
	.loc	11 900 12 is_stmt 1
	andq	%rdx, %rsi
	addq	$4, %rsi
.Ltmp7578:
	.loc	25 443 13
	leaq	.Lalloc_aebeae245abdbd708a3d39b73262e641(%rip), %rcx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp7579:
.LBB40_155:
	.loc	25 456 13
	leaq	.Lalloc_72e24c5578221343e8a784545a3910d2(%rip), %rcx
	movq	%r11, %rdi
	movq	%r10, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp7580:
.LBB40_156:
	.loc	25 443 13
	leaq	.Lalloc_ba99eeeb3482270ebe1c674b4013dd6a(%rip), %rcx
	movq	%r15, %rdi
	movq	%rbx, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp7581:
.LBB40_157:
	.loc	25 443 13
	leaq	.Lalloc_0f4c27429e4885e1b619f1e4a3587acc(%rip), %rcx
	movq	%r15, %rdi
	movq	%r10, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp7582:
.LBB40_158:
	.loc	18 968 21
	leaq	.Lalloc_376120b9c5efdf3d59386c16952a74b7(%rip), %rdi
	leaq	.Lalloc_d8ffa1b111d58613892f54938d1219d1(%rip), %rdx
	movl	$31, %esi
	callq	*_RNvNtCs4NRVxsYgnAr_4core6option13expect_failed@GOTPCREL(%rip)
.Ltmp7583:
.LBB40_159:
	.loc	25 456 13
	leaq	.Lalloc_bc75afababbf20974edb8b966373fd0c(%rip), %rcx
	xorl	%edi, %edi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp7584:
.LBB40_160:
	.loc	25 456 13
	leaq	.Lalloc_a0e50f2d670fa7dbeabdb68abcb7ac10(%rip), %rcx
	xorl	%edi, %edi
	movq	%r8, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp7585:
.LBB40_161:
	.loc	25 581 13
	leaq	.Lalloc_e6adad6d678d00eb6b02b8162f35ebb4(%rip), %rcx
	movq	%rdx, %rsi
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp7586:
.LBB40_162:
	.loc	25 581 13
	leaq	.Lalloc_9c634077742a23e4340a846355ecc484(%rip), %rcx
	movq	%r8, %rsi
	movq	%r8, %rdx
	vzeroupper
	callq	*_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail@GOTPCREL(%rip)
.Ltmp7587:
.LBB40_163:
	.loc	6 1082 33
	leaq	.Lalloc_d8ffa1b111d58613892f54938d1219d1(%rip), %rdx
	movq	%r10, %rdi
	movq	%r10, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_164:
.Ltmp7588:
	.loc	6 1083 31
	leaq	.Lalloc_d8ffa1b111d58613892f54938d1219d1(%rip), %rdx
.Ltmp7589:
	.loc	6 0 0 is_stmt 0
	movq	%rsi, %rdi
	movq	%r10, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_165:
	movq	%rbp, %rsi
.Ltmp7590:
	.loc	21 277 29 is_stmt 1
	leaq	.Lalloc_c5ae5decb60097d7e1183cb43cae6b3f(%rip), %rdx
	movq	%rsi, %rdi
	movq	96(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_166:
	.loc	21 0 29 is_stmt 0
	movq	%r13, %rdi
	.loc	21 275 29 is_stmt 1
	leaq	.Lalloc_30bf8a301493a607abcc78dbf93977e0(%rip), %rdx
	movq	48(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_167:
	.loc	21 0 29 is_stmt 0
	movq	%r14, %rdi
	.loc	21 275 29
	leaq	.Lalloc_30bf8a301493a607abcc78dbf93977e0(%rip), %rdx
	movq	48(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_168:
	.loc	21 0 29
	movq	%r13, %rdi
	.loc	21 274 29 is_stmt 1
	leaq	.Lalloc_b53d553f9945f7702333f7af20204159(%rip), %rdx
	movq	96(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_169:
	.loc	21 0 29 is_stmt 0
	movq	%r14, %rdi
	.loc	21 274 29
	leaq	.Lalloc_b53d553f9945f7702333f7af20204159(%rip), %rdx
	movq	96(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_170:
	.loc	21 0 29
	movq	%rax, %rdi
.LBB40_171:
	.loc	21 274 29 is_stmt 1
	leaq	.Lalloc_b53d553f9945f7702333f7af20204159(%rip), %rdx
	movq	96(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_172:
	.loc	21 0 29 is_stmt 0
	movq	%rbp, %rsi
	.loc	21 276 29 is_stmt 1
	leaq	.Lalloc_5833dd3f10f38ea96ec24c6054ff083c(%rip), %rdx
.Ltmp7591:
	.loc	6 0 0 is_stmt 0
	movq	%rsi, %rdi
	movq	48(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_173:
	movq	%rbx, %rsi
.Ltmp7592:
	.loc	21 277 29 is_stmt 1
	leaq	.Lalloc_c5ae5decb60097d7e1183cb43cae6b3f(%rip), %rdx
	movq	%rsi, %rdi
	movq	96(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_174:
	.loc	21 0 29 is_stmt 0
	movq	%r15, %rsi
.LBB40_175:
	.loc	21 277 29 is_stmt 1
	leaq	.Lalloc_c5ae5decb60097d7e1183cb43cae6b3f(%rip), %rdx
	movq	%rsi, %rdi
	movq	96(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_176:
	.loc	21 0 29 is_stmt 0
	movq	%rbx, %rsi
	.loc	21 276 29 is_stmt 1
	leaq	.Lalloc_5833dd3f10f38ea96ec24c6054ff083c(%rip), %rdx
.Ltmp7593:
	.loc	6 0 0 is_stmt 0
	movq	%rsi, %rdi
	movq	48(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_177:
	movq	%r15, %rsi
.LBB40_178:
.Ltmp7594:
	.loc	21 276 29 is_stmt 1
	leaq	.Lalloc_5833dd3f10f38ea96ec24c6054ff083c(%rip), %rdx
.Ltmp7595:
	.loc	6 0 0 is_stmt 0
	movq	%rsi, %rdi
	movq	48(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_179:
	movq	%rax, %rdi
.LBB40_180:
.Ltmp7596:
	.loc	21 275 29 is_stmt 1
	leaq	.Lalloc_30bf8a301493a607abcc78dbf93977e0(%rip), %rdx
	movq	48(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp7597:
.LBB40_181:
	.loc	21 0 29 is_stmt 0
	movq	%r12, %r11
.Ltmp7598:
	.loc	21 275 29
	leaq	.Lalloc_30bf8a301493a607abcc78dbf93977e0(%rip), %rdx
	movq	%r11, %rdi
	movq	%r10, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_182:
	.loc	21 0 29
	movq	%rbx, %rsi
	.loc	21 277 29 is_stmt 1
	leaq	.Lalloc_c5ae5decb60097d7e1183cb43cae6b3f(%rip), %rdx
.Ltmp7599:
	.loc	6 0 0 is_stmt 0
	movq	%rsi, %rdi
	movq	48(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_183:
	movq	%rbp, %rsi
.Ltmp7600:
	.loc	21 276 29 is_stmt 1
	leaq	.Lalloc_5833dd3f10f38ea96ec24c6054ff083c(%rip), %rdx
.Ltmp7601:
	.loc	6 0 0 is_stmt 0
	movq	%rsi, %rdi
	movq	%r10, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_184:
	movq	%r14, %rsi
.Ltmp7602:
	.loc	21 277 29 is_stmt 1
	leaq	.Lalloc_c5ae5decb60097d7e1183cb43cae6b3f(%rip), %rdx
.Ltmp7603:
	.loc	6 0 0 is_stmt 0
	movq	%rsi, %rdi
	movq	48(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_185:
	movq	%r9, %r11
.Ltmp7604:
	.loc	21 274 29 is_stmt 1
	leaq	.Lalloc_b53d553f9945f7702333f7af20204159(%rip), %rdx
	movq	%r11, %rdi
	movq	48(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_186:
	.loc	21 0 29 is_stmt 0
	movq	%r12, %r11
	.loc	21 274 29
	leaq	.Lalloc_b53d553f9945f7702333f7af20204159(%rip), %rdx
	movq	%r11, %rdi
	movq	48(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_187:
	.loc	21 0 29
	movq	%rdx, %r11
	.loc	21 275 29 is_stmt 1
	leaq	.Lalloc_30bf8a301493a607abcc78dbf93977e0(%rip), %rdx
	movq	%r11, %rdi
	movq	%r10, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_188:
	.loc	21 0 29 is_stmt 0
	movq	%r9, %r11
.LBB40_189:
	.loc	21 275 29 is_stmt 1
	leaq	.Lalloc_30bf8a301493a607abcc78dbf93977e0(%rip), %rdx
	movq	%r11, %rdi
	movq	%r10, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_190:
	.loc	21 0 29 is_stmt 0
	movq	%r14, %rsi
	.loc	21 276 29 is_stmt 1
	leaq	.Lalloc_5833dd3f10f38ea96ec24c6054ff083c(%rip), %rdx
.Ltmp7605:
	.loc	6 0 0 is_stmt 0
	movq	%rsi, %rdi
	movq	%r10, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_191:
	movq	%rbp, %rsi
.LBB40_192:
.Ltmp7606:
	.loc	21 277 29 is_stmt 1
	leaq	.Lalloc_c5ae5decb60097d7e1183cb43cae6b3f(%rip), %rdx
.Ltmp7607:
	.loc	6 0 0 is_stmt 0
	movq	%rsi, %rdi
	movq	48(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_193:
	movq	%rbx, %rsi
.LBB40_194:
.Ltmp7608:
	.loc	21 276 29 is_stmt 1
	leaq	.Lalloc_5833dd3f10f38ea96ec24c6054ff083c(%rip), %rdx
.Ltmp7609:
	.loc	6 0 0 is_stmt 0
	movq	%rsi, %rdi
	movq	%r10, %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB40_195:
	movq	%rdx, %r11
.LBB40_196:
.Ltmp7610:
	.loc	21 274 29 is_stmt 1
	leaq	.Lalloc_b53d553f9945f7702333f7af20204159(%rip), %rdx
	movq	%r11, %rdi
	movq	48(%rsp), %rsi
	vzeroupper
	callq	*_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Ltmp7611:
.Lfunc_end40:
	.size	_RNvXsd_CsdOTRa1MFkeb_13gate_expanderINtB5_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bank, .Lfunc_end40-_RNvXsd_CsdOTRa1MFkeb_13gate_expanderINtB5_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bank
